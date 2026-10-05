use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use profile::{
    ConfigRepository, DynamicToolCatalog, Model, Provider, ProvidersConfig, SessionSettings,
    WorkspaceConfig, WorkspacePolicy,
};
use provider::{CredentialScope, endpoint_origin, provider_request_digest_for_dialect};
use schema::{Block, EventKind, OriginTuple, ResumePolicy, validate_ledger};
use tekes_supervisor::{
    ChildLaunchRegistry, ChildLaunchState, ProfiledWorkerLaunchSpec, WorkerLaunchBindings,
    launch_profiled_worker_with_bindings, launch_profiled_worker_with_credentials,
    launch_profiled_worker_with_credentials_and_provider_test_redirect,
};
use test_support::{FixtureRoot, read};
use worker_control::{
    EX_PROTOCOL, LaunchChild, SupervisorMessage, WorkerMessage, decode_supervisor, decode_worker,
    encode_line,
};
use worker_control::{
    QueueTransaction, QueueTransactionAction, QueueTransactionOutcome, encode_queue_transaction,
};

fn exact_dialect_baseline(fixtures: &FixtureRoot) -> Vec<u8> {
    String::from_utf8(read(&fixtures.join("invalid/_valid-baseline.jsonl")).expect("baseline"))
        .expect("baseline UTF-8")
        .replace(
            "\"adapter\":\"anthropic\"",
            "\"adapter\":\"anthropic_messages_v1\"",
        )
        .into_bytes()
}

fn provider_server() -> (String, std::thread::JoinHandle<Vec<u8>>) {
    provider_server_with_commentary(false)
}

fn provider_server_with_commentary(commentary: bool) -> (String, std::thread::JoinHandle<Vec<u8>>) {
    provider_server_with_capture(commentary, None)
}

fn provider_server_with_capture(
    commentary: bool,
    capture: Option<std::sync::Arc<std::sync::Mutex<Vec<Vec<u8>>>>>,
) -> (String, std::thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind provider fixture");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("provider address")
    );
    let handle = std::thread::spawn(move || {
        let mut first_request = None;
        for round in 0..if commentary { 2 } else { 1 } {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut expected = None;
            loop {
                let count = stream.read(&mut buffer).expect("provider request read");
                request.extend_from_slice(&buffer[..count]);
                if expected.is_none() {
                    if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                        let headers = String::from_utf8_lossy(&request[..end + 4]);
                        let length = headers
                            .lines()
                            .find_map(|line| {
                                line.strip_prefix("content-length: ")
                                    .or_else(|| line.strip_prefix("Content-Length: "))
                            })
                            .and_then(|value| value.parse::<usize>().ok())
                            .unwrap_or(0);
                        expected = Some(end + 4 + length);
                    }
                }
                if count == 0 || expected.is_some_and(|length| request.len() >= length) {
                    break;
                }
            }
            if let Some(capture) = &capture {
                capture.lock().unwrap().push(request.clone());
            }
            if commentary {
                std::thread::sleep(Duration::from_millis(60));
            }
            let body = concat!(
                "data: {\"type\":\"response.created\"}\n\n",
                "data: {\"delta\":\"provider-ok\",\"type\":\"response.output_text.delta\"}\n\n",
                "data: {\"response\":{\"id\":\"resp-worker\",\"status\":\"completed\",\"usage\":{\"input_tokens\":3,\"output_tokens\":2}},\"type\":\"response.done\"}\n\n",
            );
            let body = if commentary && round == 0 {
                "data: {\"type\":\"response.completed\",\"response\":{\"id\":\"resp-commentary\",\"status\":\"completed\",\"output\":[{\"type\":\"message\",\"role\":\"assistant\",\"phase\":\"commentary\",\"content\":[{\"type\":\"output_text\",\"text\":\"Still working\"}]}],\"usage\":{\"input_tokens\":3,\"output_tokens\":2}}}\n\n"
            } else {
                body
            };
            write!(
            stream,
            "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
            body.len(),
            body
        )
        .expect("provider response");
            if first_request.is_none() {
                first_request = Some(request);
            }
        }
        first_request.unwrap()
    });
    (endpoint, handle)
}

#[test]
fn slice4_gate_09_spawn_validator_ordering() {
    let request = LaunchChild {
        child: "child.jsonl".to_owned(),
        spawn_id: "spawn-1".to_owned(),
        resume: ResumePolicy::Never,
    };
    let mut registry = ChildLaunchRegistry::default();
    assert_eq!(registry.resolve(&request).error.as_deref(), Some("missing"));

    for state in [
        ChildLaunchState::Running,
        ChildLaunchState::Settled,
        ChildLaunchState::Parked,
    ] {
        registry.record("child.jsonl", "spawn-1", state);
        let first = registry.resolve(&request);
        let retry = registry.resolve(&request);
        assert!(first.ok);
        assert_eq!(first, retry, "launch retry must deduplicate");
        assert_eq!(first.spawn_id, "spawn-1");
    }
    let mismatch = LaunchChild {
        spawn_id: "spawn-2".to_owned(),
        ..request
    };
    assert_eq!(
        registry.resolve(&mismatch).error.as_deref(),
        Some("spawn_mismatch")
    );
}

#[test]
fn worker_tool_control_integration() {
    let request = LaunchChild {
        child: "child.jsonl".to_owned(),
        spawn_id: "spawn-1".to_owned(),
        resume: ResumePolicy::Bounded(2),
    };
    let mut registry = ChildLaunchRegistry::default();
    registry.record("child.jsonl", "spawn-1", ChildLaunchState::Parked);
    let line = encode_line("launch_result", &registry.resolve(&request)).expect("wire reply");
    let decoded = decode_supervisor(&line).expect("worker decodes supervisor reply");
    assert!(matches!(
        decoded,
        SupervisorMessage::LaunchResult(result)
            if result.ok && result.child == "child.jsonl" && result.spawn_id == "spawn-1"
    ));
}

#[test]
fn handshake_precedes_lock_and_runnable_input_opens_turn() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let baseline = exact_dialect_baseline(&fixtures);
    let directory = tempfile::tempdir().expect("tempdir");
    let folder = directory.path().join("thread");
    fs::create_dir(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    let mut bytes = baseline;
    bytes.extend_from_slice(
        br#"{"content":[{"text":"next","type":"text"}],"kind":"input","origin_key":"i2","origin_tuple":{"client":"cli","key":"i2","op":"submit","principal":"p","target":"t"},"seq":9,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    );
    bytes.push(b'\n');
    fs::write(&ledger, bytes).expect("ledger");

    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
        .arg(&ledger)
        .args(["--timestamp", "2026-08-26T09:00:01.000Z"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn worker");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select protocol");
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("worker output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        concat!(
            "{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}\n",
            "{\"appended\":{\"seq\":11}}\n",
        )
        .as_bytes()
    );

    let projection = validate_ledger(&fs::read(&ledger).expect("worker ledger"), 1)
        .expect("worker append validates");
    assert!(matches!(projection.events[9].kind(), EventKind::RunStart));
    assert!(matches!(projection.events[10].kind(), EventKind::TurnOpen));
    assert_eq!(projection.events[10].turn(), Some(2));
}

#[test]
fn protocol_reject_exits_76_without_touching_ledger() {
    let directory = tempfile::tempdir().expect("tempdir");
    let ledger = directory.path().join("must-not-exist.jsonl");
    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
        .arg(&ledger)
        .args(["--timestamp", "2026-08-26T09:00:01.000Z"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn worker");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"reject\":{\"reason\":\"no_mutual_version\"}}\n")
        .expect("reject protocol");
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("worker output");
    assert_eq!(output.status.code(), Some(EX_PROTOCOL));
    assert!(!ledger.exists());
}

#[test]
fn queue_transaction_startup_runs_before_ordinary_turn_work() {
    let directory = tempfile::tempdir().expect("tempdir");
    let folder = directory.path().join("thread");
    fs::create_dir(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    fs::write(
        &ledger,
        concat!(
            "{\"config\":{\"digest\":\"cfg\"},\"format\":1,\"kind\":\"genesis\",\"min_reader\":1,\"min_writer\":1,\"origin_key\":\"create-1\",\"origin_tuple\":{\"client\":\"session-endpoint\",\"key\":\"create-1\",\"op\":\"session.create\",\"principal\":\"uid:501\",\"target\":\"018f0000-0000-7000-8000-000000000003\"},\"resume\":\"never\",\"seq\":1,\"thread\":\"018f0000-0000-7000-8000-000000000003\",\"ts\":\"2026-08-28T09:00:00.000Z\",\"v\":1,\"workspace\":\"ws\"}\n",
            "{\"content\":[{\"text\":\"old\",\"type\":\"text\"}],\"kind\":\"input\",\"origin_key\":\"input-1\",\"origin_tuple\":{\"client\":\"session-endpoint\",\"key\":\"input-1\",\"op\":\"session.prompt\",\"principal\":\"uid:501\",\"target\":\"018f0000-0000-7000-8000-000000000003\"},\"seq\":2,\"steer\":false,\"ts\":\"2026-08-28T09:00:01.000Z\",\"v\":1}\n"
        ),
    )
    .expect("ledger");
    let origin = |key: &str| OriginTuple {
        principal: "uid:501".to_owned(),
        client: "session-endpoint".to_owned(),
        target: "018f0000-0000-7000-8000-000000000003".to_owned(),
        op: "session.updateQueue".to_owned(),
        key: key.to_owned(),
    };
    let transaction = QueueTransaction {
        delivery: "delivery-queue-1".to_owned(),
        rpc_id: "rpc-queue-1".to_owned(),
        target_seq: 2,
        retract_origin: origin("rpc-queue-1/retract"),
        action: QueueTransactionAction::Edit {
            replacement_origin: origin("rpc-queue-1/replacement"),
            content: vec![Block::Text {
                text: "replacement".to_owned(),
            }],
            steer: false,
            assets: None,
        },
    };

    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
        .arg(&ledger)
        .args(["--timestamp", "2026-08-28T09:00:02.000Z"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn worker");
    let stdin = child.stdin.as_mut().expect("stdin");
    stdin
        .write_all(b"{\"selected\":{\"startup\":\"queue-transaction\",\"version\":2}}\n")
        .expect("select startup");
    stdin
        .write_all(&encode_queue_transaction(&transaction).expect("transaction wire"))
        .expect("transaction");
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("worker output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut lines = output.stdout.split_inclusive(|byte| *byte == b'\n');
    assert_eq!(
        lines.next().expect("hello"),
        b"{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}\n"
    );
    let result = decode_worker(lines.next().expect("result")).expect("queue result");
    assert!(matches!(
        result,
        WorkerMessage::QueueTransactionResult(result)
            if result.outcome == QueueTransactionOutcome::Committed {
                first_seq: 4,
                last_seq: 5,
                deduplicated: false,
            }
    ));
    assert!(matches!(
        decode_worker(lines.next().expect("appended doorbell")).expect("appended wire"),
        WorkerMessage::Appended(appended) if appended.seq == 5
    ));
    assert!(lines.next().is_none());
    let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
        .expect("queue transaction ledger");
    assert_eq!(projection.events.len(), 5);
    assert!(matches!(projection.events[2].kind(), EventKind::RunStart));
    assert!(matches!(projection.events[3].kind(), EventKind::QueueEdit));
    assert!(matches!(projection.events[4].kind(), EventKind::Input));
    assert!(
        projection
            .events
            .iter()
            .all(|event| !matches!(event.kind(), EventKind::TurnOpen)),
        "startup transaction must bypass turn opening"
    );
}

#[test]
fn post_turn_worker_drains_reached_deliveries_then_exits() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let baseline = exact_dialect_baseline(&fixtures);
    let directory = tempfile::tempdir().expect("tempdir");
    let folder = directory.path().join("thread");
    fs::create_dir(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    let mut runnable = baseline;
    runnable.extend_from_slice(b"{\"content\":[{\"text\":\"wake\",\"type\":\"text\"}],\"kind\":\"input\",\"origin_key\":\"wake\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"wake\",\"op\":\"submit\",\"principal\":\"p\",\"target\":\"t\"},\"seq\":9,\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1}\n");
    fs::write(&ledger, runnable).expect("ledger");

    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
        .arg(&ledger)
        .args(["--timestamp", "2026-08-26T09:00:01.000Z"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn worker");
    // stdin stays open: a worker whose turn is over must not wait for EOF. The input written
    // alongside the selection may or may not have been read off the pipe by the time the turn
    // ends; either it is appended and receipted here, or the supervisor's locked-append retry
    // owns it after the exit. Both are correct; lingering is not.
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(
            concat!(
                "{\"selected\":{\"version\":2}}\n",
                "{\"input\":{\"assets\":null,\"content\":[{\"text\":\"late\",\"type\":\"text\"}],\"delivery\":\"d-late\",\"origin\":{\"client\":\"cli\",\"key\":\"late\",\"op\":\"submit\",\"principal\":\"p\",\"target\":\"t\"},\"steer\":null}}\n",
            )
            .as_bytes(),
        )
        .expect("select protocol and deliver");
    let deadline = Instant::now() + Duration::from_secs(2);
    let status = loop {
        if let Some(status) = child.try_wait().expect("worker status") {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill().expect("kill lingering worker");
            panic!("worker lingered on stdin after its turn ended");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(status.success());
    let mut stdout = String::new();
    child
        .stdout
        .take()
        .expect("stdout")
        .read_to_string(&mut stdout)
        .expect("worker stdout");
    assert!(stdout.starts_with("{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}\n"));
    let projection =
        validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1).expect("ledger validates");
    assert!(matches!(projection.events[10].kind(), EventKind::TurnOpen));
    let late_appended = projection
        .events
        .iter()
        .any(|event| event.string_field("origin_key") == Some("late"));
    let receipted = stdout.contains("\"receipt\":{\"deduplicated\":false,\"delivery\":\"d-late\"");
    assert_eq!(
        receipted, late_appended,
        "a receipt exists exactly when the worker appended the delivery: {stdout}"
    );
    if !receipted {
        assert!(
            stdout.ends_with("{\"appended\":{\"seq\":11}}\n"),
            "{stdout}"
        );
    }
    assert!(stdout.ends_with("}\n"));
    assert!(
        !projection.terminal_tail,
        "no provider: the open turn is left for recovery"
    );
}

#[test]
fn supervisor_passes_immutable_profile_descriptors() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("launch root");
    let cwd = directory.path().join("workspace");
    let secondary = directory.path().join("workspace-secondary");
    fs::create_dir(&cwd).expect("workspace");
    fs::create_dir(&secondary).expect("secondary workspace");
    let repository = ConfigRepository::open(directory.path()).expect("config repository");
    let workspace = WorkspaceConfig {
        format: 1,
        revision: 1,
        id: "ws".to_owned(),
        name: "Main".to_owned(),
        cwd: Vec::new(),
        folders: vec![
            profile::WorkspaceFolder {
                id: "primary".to_owned(),
                path: cwd.to_str().expect("UTF-8").to_owned(),
            },
            profile::WorkspaceFolder {
                id: "secondary".to_owned(),
                path: secondary.to_str().expect("UTF-8").to_owned(),
            },
        ],
        policy: None,
    };
    repository
        .publish_workspace(0, &workspace)
        .expect("workspace config");

    let folder = directory.path().join("thread");
    fs::create_dir_all(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    let mut bytes = read(&fixtures.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    bytes.extend_from_slice(
        br#"{"content":[{"text":"next","type":"text"}],"kind":"input","origin_key":"profile-run","origin_tuple":{"client":"cli","key":"profile-run","op":"submit","principal":"p","target":"t"},"seq":9,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    );
    bytes.push(b'\n');
    fs::write(&ledger, bytes).expect("ledger");

    let legacy_error = launch_profiled_worker_with_bindings(
        &repository,
        &ProfiledWorkerLaunchSpec {
            binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
            ledger: ledger.clone(),
            timestamp: "2026-08-26T09:00:01.000Z".to_owned(),
            run_id: "legacy-multi-root".to_owned(),
            binary_attribution: "tekes-worker-test".to_owned(),
            workspace_id: "ws".to_owned(),
            folder_binding: None,
            user_agent_dir: directory.path().join("missing-user-agent"),
        },
        WorkerLaunchBindings::default(),
    )
    .err()
    .expect("legacy multi-root launch must not guess the primary folder");
    assert!(
        legacy_error
            .to_string()
            .contains("no stable folder binding")
    );

    let mut launched = launch_profiled_worker_with_bindings(
        &repository,
        &ProfiledWorkerLaunchSpec {
            binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
            ledger: ledger.clone(),
            timestamp: "2026-08-26T09:00:01.000Z".to_owned(),
            run_id: "profile-run".to_owned(),
            binary_attribution: "tekes-worker-test".to_owned(),
            workspace_id: "ws".to_owned(),
            folder_binding: Some("secondary".to_owned()),
            user_agent_dir: directory.path().join("missing-user-agent"),
        },
        WorkerLaunchBindings {
            goal_id: Some("goal-profile-run".to_owned()),
            dynamic_catalog: DynamicToolCatalog {
                format: 1,
                tools: Vec::new(),
            },
        },
    )
    .expect("launch worker");
    assert_eq!(
        launched.config_snapshot.workspace.folder_binding.as_deref(),
        Some("secondary")
    );
    assert_eq!(
        launched.config_snapshot.workspace.selected_cwd.as_deref(),
        secondary
            .canonicalize()
            .expect("secondary canonical")
            .to_str()
    );
    assert_eq!(
        launched.config_snapshot.workspace.cwd,
        vec![
            cwd.canonicalize()
                .expect("primary canonical")
                .display()
                .to_string(),
            secondary
                .canonicalize()
                .expect("secondary canonical")
                .display()
                .to_string(),
        ]
    );
    let config_digest = launched.config_digest.clone();
    let instruction_digest = launched.instruction_digest.clone();
    let launch_bindings_digest = launched.launch_bindings_digest.clone();
    let mut next_workspace = workspace;
    next_workspace.revision = 2;
    next_workspace.name = "Changed after spawn".to_owned();
    repository
        .publish_workspace(1, &next_workspace)
        .expect("post-spawn config edit");
    launched
        .child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select protocol");
    drop(launched.child.stdin.take());
    let output = launched.child.wait_with_output().expect("worker output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
        .expect("profile-attributed ledger");
    let run_start = &projection.events[9];
    assert!(matches!(run_start.kind(), EventKind::RunStart));
    assert_eq!(
        run_start.string_field("config_digest"),
        Some(config_digest.as_str())
    );
    assert_eq!(
        run_start.string_field("instruction_digest"),
        Some(instruction_digest.as_str())
    );
    assert_eq!(
        run_start.string_field("launch_bindings_digest"),
        Some(launch_bindings_digest.as_str())
    );
    let binding_bytes = fs::read(
        folder
            .join("assets")
            .join(format!("sha256-{launch_bindings_digest}")),
    )
    .expect("durable launch bindings asset");
    let binding: serde_json::Value =
        serde_json::from_slice(&binding_bytes).expect("launch bindings JSON");
    assert_eq!(binding["goal_id"], "goal-profile-run");
    assert!(
        run_start
            .string_field("policy")
            .is_some_and(|value| value.starts_with("sha256-"))
    );
}

#[test]
fn slice7_credential_descriptor_launch() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("launch root");
    let cwd = directory.path().join("workspace");
    fs::create_dir(&cwd).expect("workspace");
    let repository = ConfigRepository::open(directory.path()).expect("config repository");
    repository
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "Main".to_owned(),
                cwd: vec![cwd.to_str().expect("UTF-8").to_owned()],
                folders: Vec::new(),
                policy: None,
            },
        )
        .expect("workspace config");
    let folder = directory.path().join("thread");
    fs::create_dir_all(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    let mut bytes = read(&fixtures.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    bytes.extend_from_slice(
        br#"{"content":[{"text":"next","type":"text"}],"kind":"input","origin_key":"credential-run","origin_tuple":{"client":"cli","key":"credential-run","op":"submit","principal":"p","target":"t"},"seq":9,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    );
    bytes.push(b'\n');
    fs::write(&ledger, bytes).expect("ledger");
    let mut launched = launch_profiled_worker_with_credentials(
        &repository,
        &ProfiledWorkerLaunchSpec {
            binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
            ledger: ledger.clone(),
            timestamp: "2026-08-26T09:00:01.000Z".to_owned(),
            run_id: "credential-run".to_owned(),
            binary_attribution: "tekes-worker-test".to_owned(),
            workspace_id: "ws".to_owned(),
            folder_binding: None,
            user_agent_dir: directory.path().join("missing-user-agent"),
        },
        vec![CredentialScope {
            credential_id: "main".to_owned(),
            adapter: "openai_responses_v1".to_owned(),
            endpoint_origin: "https://api.openai.com:443".to_owned(),
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "fixture-secret-never-log".to_owned(),
        }],
    )
    .expect("launch worker with credential channel");
    launched
        .child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select protocol");
    drop(launched.child.stdin.take());
    let output = launched.child.wait_with_output().expect("worker output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output
            .stderr
            .windows("fixture-secret-never-log".len())
            .any(|bytes| bytes == b"fixture-secret-never-log")
    );
    launched
        .credential_broker
        .take()
        .expect("broker thread")
        .join()
        .expect("join broker")
        .expect("broker clean EOF");
}

#[test]
fn slice7_worker_provider_loop_commits_outcome_before_release() {
    assert_worker_provider_loop(false, false);
}

#[test]
fn real_worker_continues_after_completed_commentary_before_final_validation() {
    assert_worker_provider_loop(true, false);
}

#[test]
fn real_worker_receipts_manual_compact_and_rebuilds_its_next_provider_epoch() {
    assert_worker_provider_loop(true, true);
}

fn assert_worker_provider_loop(commentary: bool, manual: bool) {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let captured = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let (endpoint, server) = provider_server_with_capture(commentary, Some(captured.clone()));
    let directory = tempfile::tempdir().expect("launch root");
    let cwd = directory.path().join("workspace");
    fs::create_dir(&cwd).expect("workspace");
    let repository = ConfigRepository::open(directory.path()).expect("config repository");
    repository
        .publish_providers(
            0,
            &ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![Provider {
                    id: "fixture".to_owned(),
                    name: None,
                    adapter: "responses".to_owned(),
                    dialect: "openai_responses_v1".to_owned(),
                    endpoint_owner: "openai".to_owned(),
                    gateway_translation: "direct".to_owned(),
                    evidence_revision: "openai-2026-08-01".to_owned(),
                    endpoint: "https://api.openai.com/v1".to_owned(),
                    credential_key: Some("fixture-key".to_owned()),
                    models: vec![Model {
                        id: "gpt-5".to_owned(),
                        profile: "openai_responses_v1:gpt-5".to_owned(),
                        enabled: true,
                        context_window_tokens: 32_000,
                        compact_trigger_tokens: 24_000,
                    }],
                }],
                web_search: None,
            },
        )
        .expect("provider config");
    repository
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "Main".to_owned(),
                cwd: vec![cwd.to_str().expect("UTF-8").to_owned()],
                folders: Vec::new(),
                policy: Some(WorkspacePolicy {
                    network: true,
                    provider: Some("fixture".to_owned()),
                    model: Some("gpt-5".to_owned()),
                    ..WorkspacePolicy::default()
                }),
            },
        )
        .expect("workspace config");
    let folder = directory.path().join("thread");
    fs::create_dir_all(&folder).expect("thread folder");
    repository
        .publish_session_settings(
            &folder,
            0,
            &SessionSettings {
                format: 1,
                revision: 1,
                provider: "fixture".to_owned(),
                model: "gpt-5".to_owned(),
                reasoning_effort: Some("low".to_owned()),
            },
        )
        .expect("session reasoning setting");
    let ledger = folder.join("main.jsonl");
    let baseline = exact_dialect_baseline(&fixtures);
    let mut bytes = baseline
        .split_inclusive(|byte| *byte == b'\n')
        .take(1)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    bytes.extend_from_slice(
        br#"{"content":[{"text":"call provider","type":"text"}],"kind":"input","origin_key":"provider-run","origin_tuple":{"client":"cli","key":"provider-run","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-27T09:00:00.000Z","v":1}"#,
    );
    bytes.push(b'\n');
    fs::write(&ledger, bytes).expect("ledger");

    let origin = endpoint_origin("https://api.openai.com/v1").expect("endpoint origin");
    let mut launched = launch_profiled_worker_with_credentials_and_provider_test_redirect(
        &repository,
        &ProfiledWorkerLaunchSpec {
            binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
            ledger: ledger.clone(),
            timestamp: "2026-08-27T09:00:01.000Z".to_owned(),
            run_id: "provider-run".to_owned(),
            binary_attribution: "tekes-worker-test".to_owned(),
            workspace_id: "ws".to_owned(),
            folder_binding: None,
            user_agent_dir: directory.path().join("missing-user-agent"),
        },
        vec![CredentialScope {
            credential_id: "fixture-key".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: origin,
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "provider-secret".to_owned(),
        }],
        &endpoint,
    )
    .expect("launch provider worker");
    let stdout = launched.child.stdout.take().expect("worker stdout");
    let mut stdout = BufReader::new(stdout);
    let stdin = launched.child.stdin.as_mut().expect("worker stdin");
    let mut line = String::new();
    stdout.read_line(&mut line).expect("worker hello");
    assert!(line.contains("\"hello\""));
    stdin
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select worker");
    stdin.flush().expect("flush select");
    line.clear();
    stdout.read_line(&mut line).expect("lease request");
    assert!(line.contains("\"lease_request\""), "{line}");
    if manual {
        stdin.write_all(b"{\"compact\":{\"delivery\":\"manual-live\",\"origin\":{\"client\":\"test\",\"key\":\"manual-live\",\"op\":\"compact\",\"principal\":\"test\",\"target\":\"thread\"}}}\n").unwrap();
    }
    stdin
        .write_all(b"{\"lease\":{\"attempt\":\"provider-run-attempt-6\",\"granted\":true}}\n")
        .expect("grant lease");
    stdin.flush().expect("flush lease");
    let mut protocol = Vec::new();
    let mut saw_attempt_settled = false;
    loop {
        line.clear();
        stdout.read_line(&mut line).expect("worker provider line");
        if line.is_empty() {
            let mut stderr = String::new();
            launched
                .child
                .stderr
                .as_mut()
                .expect("worker stderr")
                .read_to_string(&mut stderr)
                .expect("worker stderr read");
            panic!("worker exited before appended doorbell: {stderr}");
        }
        protocol.push(line.clone());
        if line.contains("\"lease_request\"") {
            assert!(commentary, "unexpected extra response");
            let projection = validate_ledger(&fs::read(&ledger).unwrap(), 1).unwrap();
            assert!(
                !projection
                    .events
                    .iter()
                    .any(|e| e.kind() == &EventKind::Settle),
                "commentary prematurely settled session"
            );
            let message: serde_json::Value = serde_json::from_str(&line).unwrap();
            let attempt = message["lease_request"]["attempt"].as_str().unwrap();
            writeln!(
                stdin,
                "{}",
                serde_json::json!({"lease":{"attempt":attempt,"granted":true}})
            )
            .unwrap();
            stdin.flush().unwrap();
        }
        if line.contains("\"attempt_settled\"") {
            saw_attempt_settled = true;
        }
        if line.contains("\"appended\"") {
            let state = validate_ledger(&fs::read(&ledger).unwrap(), 1).unwrap();
            if state
                .events
                .last()
                .is_some_and(|e| *e.kind() == EventKind::Settle)
            {
                break;
            }
        }
    }
    let deadline = Instant::now() + Duration::from_secs(2);
    let status = loop {
        if let Some(status) = launched.child.try_wait().expect("worker status") {
            break status;
        }
        if Instant::now() >= deadline {
            launched.child.kill().expect("kill stuck settled worker");
            panic!("worker retained the line lock after its final settle");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(status.success());
    assert!(protocol.iter().any(|line| line.contains("\"frame\"")));
    assert!(saw_attempt_settled);
    assert!(
        protocol.last().is_some_and(|line| line.contains(&format!(
            "\"appended\":{{\"seq\":{}}}",
            if manual {
                validate_ledger(&fs::read(&ledger).unwrap(), 1)
                    .unwrap()
                    .last_seq
            } else if commentary {
                12
            } else {
                9
            }
        ))),
        "the post-turn doorbell must cover the final settle: {protocol:?}"
    );
    let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
        .expect("provider ledger validates");
    if manual {
        server.join().unwrap();
        let requests = captured.lock().unwrap();
        assert_eq!(requests.len(), 2);
        let parse = |request: &Vec<u8>| {
            let end = request.windows(4).position(|b| b == b"\r\n\r\n").unwrap() + 4;
            serde_json::from_slice::<serde_json::Value>(&request[end..]).unwrap()
        };
        let second = parse(&requests[1]);
        assert!(
            second.get("previous_response_id").is_none(),
            "old continuation must not cross manual compact"
        );
        assert!(second.to_string().contains("[compacted history]"));
        assert!(
            second.to_string().contains("call provider"),
            "initial input anchor survives"
        );
        assert!(
            second.to_string().contains("Still working"),
            "accepted prior output survives"
        );
        let compacts = projection
            .events
            .iter()
            .filter(|e| *e.kind() == EventKind::Compact)
            .collect::<Vec<_>>();
        assert_eq!(compacts.len(), 1);
        assert_eq!(compacts[0].string_field("origin_key"), Some("manual-live"));
        let epochs = projection
            .events
            .iter()
            .filter(|e| *e.kind() == EventKind::Epoch)
            .collect::<Vec<_>>();
        assert_eq!(epochs.len(), 2);
        assert_eq!(epochs[1].string_field("reason"), Some("compaction"));
        assert!(epochs[0].seq() < compacts[0].seq() && compacts[0].seq() < epochs[1].seq());
        assert_eq!(
            projection
                .events
                .iter()
                .filter(|e| *e.kind() == EventKind::Settle)
                .count(),
            1
        );
        assert!(
            protocol
                .iter()
                .any(|line| line.contains("\"delivery\":\"manual-live\"")
                    && line.contains("\"receipt\""))
        );
        drop(store::LockedLedger::open(&ledger, 1).expect("exited worker released its line lock"));
        launched
            .credential_broker
            .take()
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        return;
    }
    if commentary {
        let stamps = projection
            .events
            .iter()
            .filter(|e| e.kind() == &EventKind::Output)
            .map(|e| e.string_field("ts").unwrap())
            .collect::<Vec<_>>();
        assert_eq!(stamps.len(), 2);
        assert!(
            stamps[1] > stamps[0],
            "response event timestamps must advance with elapsed execution"
        );
    }
    let kinds = projection
        .events
        .iter()
        .map(|event| event.kind().as_str())
        .collect::<Vec<_>>();
    let mut expected_kinds = vec![
        "run_start",
        "turn_open",
        "epoch",
        "attempt",
        "attempt_dispatched",
        "output",
        "settle",
    ];
    if commentary {
        expected_kinds.splice(6..6, ["attempt", "attempt_dispatched", "output"]);
    }
    assert_eq!(&kinds[2..], expected_kinds);
    let offset = if commentary { 3 } else { 0 };
    if commentary {
        let first = serde_json::to_value(projection.events[7].raw()).unwrap();
        assert_eq!(first["final_answer"], false);
    }
    // A root final settles directly: no validation.candidate/decision states
    // and no validator child; the settle names the promoted output.
    let settled = serde_json::to_value(projection.events[8 + offset].raw()).unwrap();
    assert!(settled.get("validation").is_none());
    assert_eq!(settled["promoted_output_seq"], 8 + offset);
    assert!(
        !projection
            .events
            .iter()
            .any(|e| e.kind() == &EventKind::Spawn)
    );
    let outcomes_with_usage = projection
        .events
        .iter()
        .filter(|event| event.kind() == &EventKind::Output && event.has_field("usage"))
        .count();
    assert_eq!(outcomes_with_usage, if commentary { 2 } else { 1 });
    let request = server.join().expect("join provider");
    let header_end = request
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .expect("HTTP header terminator")
        + 4;
    let request_text = String::from_utf8_lossy(&request[..header_end]);
    assert!(
        request_text.starts_with("POST /v1/responses HTTP/1.1\r\n"),
        "transport seam changed the proved request path: {request_text}"
    );
    assert!(
        request_text
            .to_ascii_lowercase()
            .contains("host: api.openai.com"),
        "transport seam changed the proved Host identity: {request_text}"
    );
    assert!(
        request_text
            .to_ascii_lowercase()
            .contains("authorization: bearer provider-secret")
    );
    let request_body = &request[header_end..];
    let request_json: serde_json::Value =
        serde_json::from_slice(request_body).expect("provider request body");
    assert_eq!(
        request_json.pointer("/reasoning/effort"),
        Some(&serde_json::json!("low")),
        "session reasoning_effort must reach the real worker HTTP body"
    );
    let expected_digest = provider_request_digest_for_dialect(
        "openai_responses_v1",
        "POST",
        "https://api.openai.com/v1/responses",
        "gpt-5",
        &BTreeMap::from([
            ("accept".to_owned(), "text/event-stream".to_owned()),
            ("content-type".to_owned(), "application/json".to_owned()),
        ]),
        request_body,
    )
    .expect("request digest");
    let attempt = projection
        .events
        .iter()
        .find(|event| *event.kind() == EventKind::Attempt)
        .expect("attempt event");
    assert_eq!(
        attempt.string_field("wire_digest"),
        Some(expected_digest.as_str())
    );
    let epoch = projection
        .events
        .iter()
        .find(|event| *event.kind() == EventKind::Epoch)
        .expect("epoch event");
    let epoch_asset = epoch.raw();
    let epoch_raw = serde_json::to_value(epoch_asset).expect("epoch event JSON");
    let epoch_asset = epoch_raw
        .pointer("/system/asset")
        .and_then(serde_json::Value::as_str)
        .expect("epoch profile asset");
    let epoch_profile: serde_json::Value = serde_json::from_slice(
        &fs::read(folder.join("assets").join(epoch_asset)).expect("epoch profile bytes"),
    )
    .expect("epoch profile JSON");
    assert_eq!(
        epoch_profile.pointer("/controls/reasoning_effort"),
        Some(&serde_json::json!("low")),
        "session reasoning_effort must be immutable epoch material"
    );
    launched
        .credential_broker
        .take()
        .expect("broker thread")
        .join()
        .expect("join broker")
        .expect("broker clean EOF");
}

#[test]
fn slice7_ordinary_recovery_closes_old_attempt_before_resend() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let (endpoint, server) = provider_server();
    let directory = tempfile::tempdir().expect("launch root");
    let cwd = directory.path().join("workspace");
    fs::create_dir(&cwd).expect("workspace");
    let repository = ConfigRepository::open(directory.path()).expect("config repository");
    repository
        .publish_providers(
            0,
            &ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![Provider {
                    id: "fixture".to_owned(),
                    name: None,
                    adapter: "responses".to_owned(),
                    dialect: "openai_responses_v1".to_owned(),
                    endpoint_owner: "openai".to_owned(),
                    gateway_translation: "direct".to_owned(),
                    evidence_revision: "openai-2026-08-01".to_owned(),
                    endpoint: "https://api.openai.com/v1".to_owned(),
                    credential_key: Some("fixture-key".to_owned()),
                    models: vec![Model {
                        id: "gpt-5".to_owned(),
                        profile: "openai_responses_v1:gpt-5".to_owned(),
                        enabled: true,
                        context_window_tokens: 32_000,
                        compact_trigger_tokens: 24_000,
                    }],
                }],
                web_search: None,
            },
        )
        .expect("provider config");
    repository
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "Main".to_owned(),
                cwd: vec![cwd.to_str().expect("UTF-8").to_owned()],
                folders: Vec::new(),
                policy: Some(WorkspacePolicy {
                    network: true,
                    provider: Some("fixture".to_owned()),
                    model: Some("gpt-5".to_owned()),
                    ..WorkspacePolicy::default()
                }),
            },
        )
        .expect("workspace config");
    let folder = directory.path().join("thread");
    fs::create_dir_all(&folder).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    let baseline = exact_dialect_baseline(&fixtures);
    let bytes = baseline
        .split_inclusive(|byte| *byte == b'\n')
        .take(6)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    let bytes = String::from_utf8(bytes)
        .expect("baseline UTF-8")
        .replace("\"resume\":\"never\"", "\"resume\":{\"bounded\":2}");
    fs::write(&ledger, bytes).expect("recovery ledger");

    let origin = endpoint_origin("https://api.openai.com/v1").expect("endpoint origin");
    let mut launched = launch_profiled_worker_with_credentials_and_provider_test_redirect(
        &repository,
        &ProfiledWorkerLaunchSpec {
            binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
            ledger: ledger.clone(),
            timestamp: "2026-08-27T09:00:01.000Z".to_owned(),
            run_id: "ordinary-recovery".to_owned(),
            binary_attribution: "tekes-worker-test".to_owned(),
            workspace_id: "ws".to_owned(),
            folder_binding: None,
            user_agent_dir: directory.path().join("missing-user-agent"),
        },
        vec![CredentialScope {
            credential_id: "fixture-key".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: origin,
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "provider-secret".to_owned(),
        }],
        &endpoint,
    )
    .expect("launch recovery worker");
    let mut stdout = BufReader::new(launched.child.stdout.take().expect("worker stdout"));
    let stdin = launched.child.stdin.as_mut().expect("worker stdin");
    let mut line = String::new();
    stdout.read_line(&mut line).expect("worker hello");
    stdin
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select worker");
    stdin.flush().expect("flush select");

    line.clear();
    stdout.read_line(&mut line).expect("old attempt settled");
    assert!(
        line.contains("\"attempt_settled\"") && line.contains("\"a1\""),
        "{line}"
    );
    line.clear();
    stdout.read_line(&mut line).expect("new lease request");
    assert!(
        line.contains("\"lease_request\"") && line.contains("ordinary-recovery-attempt-11"),
        "{line}"
    );
    stdin
        .write_all(b"{\"lease\":{\"attempt\":\"ordinary-recovery-attempt-11\",\"granted\":true}}\n")
        .expect("grant retry lease");
    stdin.flush().expect("flush lease");
    loop {
        line.clear();
        stdout.read_line(&mut line).expect("retry output");
        if line.is_empty() {
            let mut stderr = String::new();
            launched
                .child
                .stderr
                .as_mut()
                .expect("worker stderr")
                .read_to_string(&mut stderr)
                .expect("worker stderr read");
            panic!("worker exited before retry settled: {stderr}");
        }
        if line.contains("\"attempt_settled\"") {
            break;
        }
    }
    drop(launched.child.stdin.take());
    assert!(launched.child.wait().expect("worker exit").success());
    server.join().expect("join provider");
    let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
        .expect("ordinary recovery ledger validates");
    let old_error = projection
        .events
        .iter()
        .position(|event| {
            *event.kind() == EventKind::Error && event.string_field("attempt") == Some("a1")
        })
        .expect("old attempt error");
    let new_attempt = projection
        .events
        .iter()
        .position(|event| {
            *event.kind() == EventKind::Attempt
                && event.string_field("attempt") == Some("ordinary-recovery-attempt-11")
        })
        .expect("new attempt");
    assert!(old_error < new_attempt, "old chain must close before retry");
}

#[test]
fn slice7_provider_reconcile_closes_unpaired_attempt() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let baseline = exact_dialect_baseline(&fixtures);
    for (dispatched, expected_decision, expected_classification) in [
        (false, "not_dispatched", "transport"),
        (true, "unresolved", "unresolved_dispatch"),
    ] {
        let directory = tempfile::tempdir().expect("thread root");
        let folder = directory.path().join("thread");
        fs::create_dir(&folder).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        let mut bytes = baseline
            .split_inclusive(|byte| *byte == b'\n')
            .take(6)
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        if dispatched {
            bytes.extend_from_slice(
                br#"{"attempt":"a1","kind":"attempt_dispatched","seq":7,"ts":"2026-08-27T09:00:00.000Z","turn":1,"v":1}"#,
            );
            bytes.push(b'\n');
        }
        fs::write(&ledger, bytes).expect("recovery ledger");
        let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
            .arg(&ledger)
            .args(["--timestamp", "2026-08-27T09:00:01.000Z"])
            .args([
                "--run-id",
                if dispatched {
                    "recovery-d"
                } else {
                    "recovery-u"
                },
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn recovery worker");
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"{\"selected\":{\"version\":2}}\n")
            .expect("select recovery worker");
        drop(child.stdin.take());
        let output = child.wait_with_output().expect("recovery output");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("\"attempt_settled\""),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
            .expect("recovery ledger validates");
        let recovery = projection
            .events
            .iter()
            .find(|event| *event.kind() == EventKind::AttemptRecovery)
            .expect("attempt recovery");
        assert_eq!(recovery.string_field("decision"), Some(expected_decision));
        let error = projection
            .events
            .iter()
            .find(|event| *event.kind() == EventKind::Error)
            .expect("attempt error");
        assert_eq!(
            error.string_field("classification"),
            Some(expected_classification)
        );
        assert!(projection.terminal_tail);
    }
}

#[test]
fn slice7_provider_reconcile_completes_adopt_from_asset_only() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let baseline = exact_dialect_baseline(&fixtures);
    let directory = tempfile::tempdir().expect("thread root");
    let folder = directory.path().join("thread");
    fs::create_dir(&folder).expect("thread folder");
    let assets = store::AssetStore::new(folder.join("assets")).expect("asset store");
    let mut response: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("provider-runtime/predecessor/anthropic.response.json"))
            .expect("adopt response"),
    )
    .expect("adopt response JSON");
    // Exact Anthropic profiles do not advertise thinking blocks. Keep this
    // recovery test scoped to adopting the durable tool call from the asset.
    response["content"]
        .as_array_mut()
        .expect("adopt response content")
        .retain(|block| block["type"] != "thinking");
    let response = serde_json::to_vec(&response).expect("adopt response bytes");
    let response = assets.publish(&response).expect("publish response");
    let ledger = folder.join("main.jsonl");
    let mut bytes = baseline
        .split_inclusive(|byte| *byte == b'\n')
        .take(6)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    bytes.extend_from_slice(
        br#"{"attempt":"a1","kind":"attempt_dispatched","seq":7,"ts":"2026-08-27T09:00:00.000Z","turn":1,"v":1}"#,
    );
    bytes.push(b'\n');
    bytes.extend_from_slice(
        format!(
            "{{\"attempt\":\"a1\",\"decision\":\"adopt\",\"inventory\":[\"call_lookup\"],\"kind\":\"attempt_recovery\",\"response\":{{\"asset\":\"{}\"}},\"seq\":8,\"ts\":\"2026-08-27T09:00:00.000Z\",\"turn\":1,\"v\":1}}",
            response.asset
        )
        .as_bytes(),
    );
    bytes.push(b'\n');
    fs::write(&ledger, bytes).expect("adopt ledger");
    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-worker"))
        .arg(&ledger)
        .args(["--timestamp", "2026-08-27T09:00:01.000Z"])
        .args(["--run-id", "recovery-adopt"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn adopt worker");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select adopt worker");
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("adopt output");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("\"attempt_settled\""));
    let projection = validate_ledger(&fs::read(&ledger).expect("ledger bytes"), 1)
        .expect("adopt ledger validates");
    let appended = projection.events.iter().skip(8).collect::<Vec<_>>();
    assert_eq!(
        appended
            .iter()
            .map(|event| event.kind().as_str())
            .collect::<Vec<_>>(),
        ["run_start", "output", "tool_call", "tool_result", "settle"]
    );
    assert_eq!(
        appended[3].string_field("call"),
        Some("call_lookup"),
        "recovered call is paired exactly once"
    );
    assert!(projection.terminal_tail);
}

#[test]
fn gated_profiled_worker_exits_before_lease_or_provider_and_preserves_log() {
    for gate in ["min_reader", "min_writer"] {
        let fixtures = FixtureRoot::discover().expect("fixtures");
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let directory = tempfile::tempdir().expect("launch root");
        let cwd = directory.path().join("workspace");
        fs::create_dir(&cwd).expect("workspace");
        let repository = ConfigRepository::open(directory.path()).expect("config repository");
        repository
            .publish_providers(
                0,
                &ProvidersConfig {
                    format: 1,
                    revision: 1,
                    providers: vec![Provider {
                        id: "fixture".to_owned(),
                        name: None,
                        adapter: "responses".to_owned(),
                        dialect: "openai_responses_v1".to_owned(),
                        endpoint_owner: "openai".to_owned(),
                        gateway_translation: "direct".to_owned(),
                        evidence_revision: "openai-2026-08-01".to_owned(),
                        endpoint: "https://api.openai.com/v1".to_owned(),
                        credential_key: Some("fixture-key".to_owned()),
                        models: vec![Model {
                            id: "gpt-5".to_owned(),
                            profile: "openai_responses_v1:gpt-5".to_owned(),
                            enabled: true,
                            context_window_tokens: 32_000,
                            compact_trigger_tokens: 24_000,
                        }],
                    }],
                    web_search: None,
                },
            )
            .expect("provider config");
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "ws".to_owned(),
                    name: "Main".to_owned(),
                    cwd: vec![cwd.to_str().expect("UTF-8").to_owned()],
                    folders: Vec::new(),
                    policy: Some(WorkspacePolicy {
                        network: true,
                        provider: Some("fixture".to_owned()),
                        model: Some("gpt-5".to_owned()),
                        ..WorkspacePolicy::default()
                    }),
                },
            )
            .expect("workspace config");
        let folder = directory.path().join("thread");
        fs::create_dir_all(&folder).expect("thread folder");
        repository
            .publish_session_settings(
                &folder,
                0,
                &SessionSettings {
                    format: 1,
                    revision: 1,
                    provider: "fixture".to_owned(),
                    model: "gpt-5".to_owned(),
                    reasoning_effort: Some("low".to_owned()),
                },
            )
            .expect("session reasoning setting");
        let ledger = folder.join("main.jsonl");
        let baseline = exact_dialect_baseline(&fixtures);
        let mut bytes = baseline
            .split_inclusive(|byte| *byte == b'\n')
            .take(1)
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        bytes.extend_from_slice(
        br#"{"content":[{"text":"call provider","type":"text"}],"kind":"input","origin_key":"provider-run","origin_tuple":{"client":"cli","key":"provider-run","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-27T09:00:00.000Z","v":1}"#,
    );
        bytes.push(b'\n');
        let mut bytes = String::from_utf8(bytes)
            .unwrap()
            .replace(&format!("\"{gate}\":1"), &format!("\"{gate}\":2"))
            .into_bytes();
        bytes.extend_from_slice(b"unrecognized-new-format-tail");
        fs::write(&ledger, &bytes).expect("ledger");

        let origin = endpoint_origin("https://api.openai.com/v1").expect("endpoint origin");
        let mut launched = launch_profiled_worker_with_credentials_and_provider_test_redirect(
            &repository,
            &ProfiledWorkerLaunchSpec {
                binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
                ledger: ledger.clone(),
                timestamp: "2026-08-27T09:00:01.000Z".to_owned(),
                run_id: "provider-run".to_owned(),
                binary_attribution: "tekes-worker-test".to_owned(),
                workspace_id: "ws".to_owned(),
                folder_binding: None,
                user_agent_dir: directory.path().join("missing-user-agent"),
            },
            vec![CredentialScope {
                credential_id: "fixture-key".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: origin,
                purpose: "provider".to_owned(),
                generation: "g1".to_owned(),
                material: "provider-secret".to_owned(),
            }],
            &endpoint,
        )
        .expect("launch provider worker");

        launched
            .child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(b"{\"selected\":{\"version\":2}}\n")
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if launched.child.try_wait().unwrap().is_some() {
                break;
            }
            if Instant::now() >= deadline {
                let _ = launched.child.kill();
                let _ = launched.child.wait();
                panic!("gated worker did not exit before requesting execution authority");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let output = launched.child.wait_with_output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(EX_PROTOCOL),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("ledger requires reader"));
        assert_eq!(
            output.stdout,
            b"{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}\n"
        );
        assert_eq!(fs::read(&ledger).unwrap(), bytes);
        assert!(
            matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
        );
    }
}

/// Eager dispatch across a worker death: the write-ahead `tool_call` is durable
/// and the call is mid-execution (waiting on the supervisor control channel)
/// when the worker is killed before the response terminal exists. The next run
/// pairs the call exactly once through stable-receipt recovery, closes the
/// interrupted attempt, and re-sends; no second call record and no second
/// result are ever written.
#[test]
fn eager_dispatch_survives_worker_kill_between_intent_and_result() {
    use worker_control::{ToolControlResult, decode_tool_control, encode_tool_control_result};
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind provider fixture");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("provider address")
    );
    let (release, gate) = std::sync::mpsc::channel::<()>();
    let server = std::thread::spawn(move || {
        let read_request = |stream: &mut std::net::TcpStream| {
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut expected = None;
            loop {
                let count = stream.read(&mut buffer).expect("provider request read");
                request.extend_from_slice(&buffer[..count]);
                if expected.is_none() {
                    if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                        let headers = String::from_utf8_lossy(&request[..end + 4]);
                        let length = headers
                            .lines()
                            .find_map(|line| {
                                let (name, value) = line.split_once(':')?;
                                name.eq_ignore_ascii_case("content-length")
                                    .then(|| value.trim().parse::<usize>().ok())
                                    .flatten()
                            })
                            .unwrap_or(0);
                        expected = Some(end + 4 + length);
                    }
                }
                if count == 0 || expected.is_some_and(|length| request.len() >= length) {
                    break;
                }
            }
            request
        };
        // Round 1: the streamed call becomes ready; the terminal is withheld
        // until the test has killed the worker, so no output can ever land.
        let (mut stream, _) = listener.accept().expect("provider accept");
        let _ = read_request(&mut stream);
        let prefix = concat!(
            "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"call_id\":\"call-eager-kill\",\"name\":\"context_get\",\"arguments\":\"\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.done\",\"output_index\":0,\"arguments\":\"{\\\"record_ids\\\":[2],\\\"reason\\\":\\\"eager kill\\\"}\"}\n\n",
        );
        let terminal = "data: {\"type\":\"response.completed\",\"response\":{\"id\":\"resp-eager-kill\",\"status\":\"completed\",\"output\":[{\"type\":\"function_call\",\"call_id\":\"call-eager-kill\",\"name\":\"context_get\",\"arguments\":\"{\\\"record_ids\\\":[2],\\\"reason\\\":\\\"eager kill\\\"}\"}],\"usage\":{\"input_tokens\":2,\"output_tokens\":1}}}\n\n";
        write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", prefix.len() + terminal.len()).unwrap();
        stream.write_all(prefix.as_bytes()).unwrap();
        stream.flush().unwrap();
        let _ = gate.recv_timeout(Duration::from_secs(30));
        let _ = stream.write_all(terminal.as_bytes());
        // Round 2: the recovered run re-sends and gets a final answer.
        let (mut stream, _) = listener.accept().expect("provider accept");
        let _ = read_request(&mut stream);
        let body = br#"{"id":"resp-recovered","output":[{"content":[{"text":"done","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#;
        write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).unwrap();
        stream.write_all(body).unwrap();
    });

    let directory = tempfile::tempdir().expect("launch root");
    let cwd = directory.path().join("workspace");
    fs::create_dir(&cwd).expect("workspace");
    let repository = ConfigRepository::open(directory.path()).expect("config repository");
    repository
        .publish_providers(
            0,
            &ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![Provider {
                    id: "fixture".to_owned(),
                    name: None,
                    adapter: "responses".to_owned(),
                    dialect: "openai_responses_v1".to_owned(),
                    endpoint_owner: "openai".to_owned(),
                    gateway_translation: "direct".to_owned(),
                    evidence_revision: "openai-2026-08-01".to_owned(),
                    endpoint: "https://api.openai.com/v1".to_owned(),
                    credential_key: Some("fixture-key".to_owned()),
                    models: vec![Model {
                        id: "gpt-5".to_owned(),
                        profile: "openai_responses_v1:gpt-5".to_owned(),
                        enabled: true,
                        context_window_tokens: 32_000,
                        compact_trigger_tokens: 24_000,
                    }],
                }],
                web_search: None,
            },
        )
        .expect("provider config");
    repository
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "Main".to_owned(),
                cwd: vec![cwd.to_str().expect("UTF-8").to_owned()],
                folders: Vec::new(),
                policy: Some(WorkspacePolicy {
                    network: true,
                    provider: Some("fixture".to_owned()),
                    model: Some("gpt-5".to_owned()),
                    allowed_tools: vec!["context_get".to_owned()],
                    ..WorkspacePolicy::default()
                }),
            },
        )
        .expect("workspace config");
    // Supervisor-owned tools bind the session to the thread folder's UUID.
    let folder = directory
        .path()
        .join("threads")
        .join("018f0000-0000-7000-8000-000000000003");
    fs::create_dir_all(folder.join("assets")).expect("thread folder");
    let ledger = folder.join("main.jsonl");
    // Genesis plus one runnable input: the killed run opens the turn itself.
    // `resume: bounded` lets the recovery run continue the turn (re-send)
    // instead of reconciling it closed, exactly as the ordinary recovery gate.
    let bytes = exact_dialect_baseline(&fixtures)
        .split_inclusive(|byte| *byte == b'\n')
        .take(2)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    let bytes = String::from_utf8(bytes)
        .expect("baseline UTF-8")
        .replace("\"resume\":\"never\"", "\"resume\":{\"bounded\":2}");
    fs::write(&ledger, bytes).expect("kill ledger");
    let origin = endpoint_origin("https://api.openai.com/v1").expect("endpoint origin");
    let scopes = || {
        vec![CredentialScope {
            credential_id: "fixture-key".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: origin.clone(),
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "provider-secret".to_owned(),
        }]
    };
    let spec = |run_id: &str| ProfiledWorkerLaunchSpec {
        binary: env!("CARGO_BIN_EXE_tekes-worker").into(),
        ledger: ledger.clone(),
        timestamp: "2026-08-27T09:00:01.000Z".to_owned(),
        run_id: run_id.to_owned(),
        binary_attribution: "tekes-worker-test".to_owned(),
        workspace_id: "ws".to_owned(),
        folder_binding: None,
        user_agent_dir: directory.path().join("missing-user-agent"),
    };

    // Run 1: grant the lease, wait for the eager control request, kill.
    let mut launched = launch_profiled_worker_with_credentials_and_provider_test_redirect(
        &repository,
        &spec("eager-kill"),
        scopes(),
        &endpoint,
    )
    .expect("launch eager worker");
    let mut stdout = BufReader::new(launched.child.stdout.take().expect("worker stdout"));
    let stdin = launched.child.stdin.as_mut().expect("worker stdin");
    let mut line = String::new();
    stdout.read_line(&mut line).expect("worker hello");
    stdin
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .expect("select worker");
    stdin.flush().unwrap();
    let mut first_attempt = None;
    let control = loop {
        line.clear();
        stdout.read_line(&mut line).expect("worker line");
        if line.is_empty() {
            let mut stderr = String::new();
            launched
                .child
                .stderr
                .as_mut()
                .unwrap()
                .read_to_string(&mut stderr)
                .unwrap();
            panic!(
                "worker exited before the eager control request\nstderr: {stderr}\nledger:\n{}",
                fs::read_to_string(&ledger).unwrap()
            );
        }
        if let Ok(WorkerMessage::LeaseRequest(request)) = decode_worker(line.as_bytes()) {
            first_attempt = Some(request.attempt.clone());
            stdin
                .write_all(
                    encode_line(
                        "lease",
                        &worker_control::Lease {
                            attempt: request.attempt,
                            granted: true,
                        },
                    )
                    .unwrap()
                    .as_slice(),
                )
                .unwrap();
            stdin.flush().unwrap();
            continue;
        }
        if let Ok(control) = decode_tool_control(line.as_bytes()) {
            break control;
        }
    };
    assert_eq!(control.call_id, "call-eager-kill");
    assert_eq!(control.name, "context_get");
    let first_attempt = first_attempt.expect("lease was requested");
    let before_kill = validate_ledger(&fs::read(&ledger).unwrap(), 1)
        .expect("ledger validates at the kill point");
    assert!(
        before_kill
            .events
            .iter()
            .any(|e| *e.kind() == EventKind::ToolCall
                && e.string_field("call") == Some("call-eager-kill")
                && e.string_field("attempt") == Some(first_attempt.as_str())),
        "write-ahead intent is durable before execution"
    );
    assert!(
        before_kill
            .events
            .iter()
            .all(|e| *e.kind() != EventKind::ToolResult),
        "no result exists while the control request is in flight"
    );
    assert!(
        before_kill
            .events
            .iter()
            .all(|e| *e.kind() != EventKind::Output),
        "the response terminal never landed"
    );
    launched.child.kill().expect("kill worker mid-execution");
    let _ = launched.child.wait();
    let _ = release.send(());
    let killed_bytes = fs::read(&ledger).unwrap();

    // Run 2: recovery pairs the call exactly once, closes the attempt, re-sends.
    let mut launched = launch_profiled_worker_with_credentials_and_provider_test_redirect(
        &repository,
        &spec("eager-recover"),
        scopes(),
        &endpoint,
    )
    .expect("launch recovery worker");
    let mut stdout = BufReader::new(launched.child.stdout.take().expect("worker stdout"));
    let stdin = launched.child.stdin.as_mut().expect("worker stdin");
    line.clear();
    stdout.read_line(&mut line).expect("worker hello");
    stdin
        .write_all(b"{\"selected\":{\"version\":2}}\n")
        .unwrap();
    stdin.flush().unwrap();
    let mut control_requests = 0;
    let mut settled = Vec::new();
    loop {
        line.clear();
        stdout.read_line(&mut line).expect("worker line");
        if line.is_empty() {
            break;
        }
        if let Ok(control) = decode_tool_control(line.as_bytes()) {
            control_requests += 1;
            assert_eq!(control.call_id, "call-eager-kill");
            let value = schema::IJsonValue::parse(br#"{"records":[]}"#).unwrap();
            let result = ToolControlResult::success(
                control.request_id.clone(),
                control.call_id.clone(),
                value,
            );
            stdin
                .write_all(&encode_tool_control_result(&result).unwrap())
                .unwrap();
            stdin.flush().unwrap();
            continue;
        }
        match decode_worker(line.as_bytes()) {
            Ok(WorkerMessage::LeaseRequest(request)) => {
                stdin
                    .write_all(
                        encode_line(
                            "lease",
                            &worker_control::Lease {
                                attempt: request.attempt,
                                granted: true,
                            },
                        )
                        .unwrap()
                        .as_slice(),
                    )
                    .unwrap();
                stdin.flush().unwrap();
            }
            Ok(WorkerMessage::AttemptSettled(s)) => settled.push(s.attempt),
            _ => {}
        }
    }
    let status = launched.child.wait().expect("worker exit");
    assert!(status.success(), "recovery worker exit: {status:?}");
    server.join().expect("provider server");
    assert_eq!(
        control_requests, 1,
        "the interrupted call executes exactly once on recovery"
    );
    assert_eq!(
        settled.first().map(String::as_str),
        Some(first_attempt.as_str()),
        "the interrupted attempt is closed first"
    );
    let bytes = fs::read(&ledger).unwrap();
    assert!(
        bytes.starts_with(&killed_bytes),
        "recovery preserves every byte written before the kill"
    );
    let projection = validate_ledger(&bytes, 1).expect("recovered ledger validates");
    let events = &projection.events;
    assert_eq!(
        events
            .iter()
            .filter(|e| *e.kind() == EventKind::ToolCall
                && e.string_field("call") == Some("call-eager-kill"))
            .count(),
        1,
        "one call record"
    );
    let results = events
        .iter()
        .filter(|e| {
            *e.kind() == EventKind::ToolResult && e.string_field("call") == Some("call-eager-kill")
        })
        .collect::<Vec<_>>();
    assert_eq!(results.len(), 1, "one result");
    assert_eq!(
        results[0].string_field("outcome"),
        Some("ok"),
        "stable-receipt recovery re-executed rather than aborting"
    );
    let old_error = events
        .iter()
        .position(|e| {
            *e.kind() == EventKind::Error
                && e.string_field("attempt") == Some(first_attempt.as_str())
        })
        .expect("interrupted attempt closes with an error");
    let new_attempt = events
        .iter()
        .position(|e| {
            *e.kind() == EventKind::Attempt
                && e.string_field("attempt") != Some(first_attempt.as_str())
        })
        .expect("re-sent attempt");
    let result_seq = events
        .iter()
        .position(|e| *e.kind() == EventKind::ToolResult)
        .unwrap();
    assert!(
        result_seq < old_error && old_error < new_attempt,
        "result pairs, then the attempt closes, then the re-send"
    );
    let settle = events.last().unwrap();
    assert_eq!(settle.kind(), &EventKind::Settle);
    assert_eq!(settle.string_field("outcome"), Some("completed"));
}
