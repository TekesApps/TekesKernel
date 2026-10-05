use std::collections::BTreeMap;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use engine::{
    ArtifactVersionAuthority, DEFAULT_SHELL_DURATION_MS, DurableArtifactVersions,
    MAX_SHELL_DURATION_MS, RootMount, SystemToolBackend, SystemToolConfig, ToolBackend,
};
use schema::IJsonValue;
use tools::{
    BackendFailure, BackendGate, BackendOutcome, BackendTerminal, BoundedHttpClient, ByteString,
    CancellationToken, CreateMode, ExecValue, HelperError, HelperErrorClass, HelperInvoker,
    HelperOperation, HelperRequest, HelperResponse, HelperServer, HelperValue, HttpLimits,
    ReadValue, RootBinding, SecretScan, SecretScanner, ToolExecution, WriteValue,
};

#[derive(Default)]
struct MemoryHelper {
    files: Mutex<BTreeMap<String, Vec<u8>>>,
    operations: Mutex<Vec<HelperOperation>>,
}

impl MemoryHelper {
    fn digest(bytes: &[u8]) -> String {
        format!("test-{}-{}", bytes.len(), String::from_utf8_lossy(bytes))
    }
}

impl HelperInvoker for MemoryHelper {
    fn invoke(
        &self,
        request: &HelperRequest,
        _cancellation: &CancellationToken,
    ) -> Result<HelperResponse, BackendFailure> {
        self.operations
            .lock()
            .unwrap()
            .push(request.operation.clone());
        let value = match &request.operation {
            HelperOperation::Read { path, .. } => {
                let files = self.files.lock().unwrap();
                let Some(bytes) = files.get(path) else {
                    return Ok(HelperResponse::Error {
                        id: request.id.clone(),
                        error: HelperError::new(HelperErrorClass::Missing, "missing"),
                    });
                };
                HelperValue::Read(ReadValue {
                    content: ByteString::from_bytes(bytes),
                    bytes: bytes.len() as u64,
                    sha256: Self::digest(bytes),
                })
            }
            HelperOperation::Write {
                path,
                content,
                create,
                ..
            } => {
                let bytes = content
                    .decode()
                    .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
                let mut files = self.files.lock().unwrap();
                if *create == CreateMode::New && files.contains_key(path) {
                    return Ok(HelperResponse::Error {
                        id: request.id.clone(),
                        error: HelperError::new(HelperErrorClass::Exists, "exists"),
                    });
                }
                files.insert(path.clone(), bytes.clone());
                HelperValue::Write(WriteValue {
                    bytes: bytes.len() as u64,
                    sha256: Self::digest(&bytes),
                })
            }
            HelperOperation::Patch {
                path,
                expected_sha256,
                replacement,
                ..
            } => {
                let bytes = replacement
                    .decode()
                    .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
                let mut files = self.files.lock().unwrap();
                let current = files.get(path).expect("patch target exists");
                if &Self::digest(current) != expected_sha256 {
                    return Ok(HelperResponse::Error {
                        id: request.id.clone(),
                        error: HelperError::new(HelperErrorClass::DigestMismatch, "changed"),
                    });
                }
                files.insert(path.clone(), bytes.clone());
                HelperValue::Patch(WriteValue {
                    bytes: bytes.len() as u64,
                    sha256: Self::digest(&bytes),
                })
            }
            HelperOperation::Grep { .. } => {
                panic!("native fallback must not run when exec succeeds")
            }
            HelperOperation::Exec(_) => HelperValue::Exec(ExecValue {
                status: 0,
                stdout: ByteString::from_bytes(b"ok"),
                stderr: ByteString::from_bytes(b""),
                stdout_truncated: false,
                stderr_truncated: false,
            }),
            HelperOperation::Glob { .. } => HelperValue::Glob {
                paths: vec!["newest.rs".to_owned()],
            },
        };
        Ok(HelperResponse::Result {
            id: request.id.clone(),
            value,
        })
    }
}

fn execution(name: &str, call: &str, invocation: &IJsonValue) -> ToolExecution {
    ToolExecution {
        thread: "thread-1".to_owned(),
        call: call.to_owned(),
        name: name.to_owned(),
        attempt: "attempt-1".to_owned(),
        invocation: invocation.clone(),
        side_effectful: matches!(name, "apply_patch" | "edit" | "write" | "shell" | "job"),
        turn: 1,
        timestamp: "2026-08-27T00:00:00Z".to_owned(),
    }
}

fn call(backend: &mut SystemToolBackend, name: &str, call: &str, json: &str) -> serde_json::Value {
    let invocation = IJsonValue::parse_str(json).unwrap();
    match backend.execute(&execution(name, call, &invocation), &invocation) {
        BackendTerminal::Completed(value) => serde_json::to_value(value).unwrap(),
        other => panic!("unexpected terminal: {other:?}"),
    }
}

fn backend(root: &tempfile::TempDir, helper: Arc<MemoryHelper>) -> SystemToolBackend {
    backend_with(root, helper, test_config(root))
}

fn test_config(root: &tempfile::TempDir) -> SystemToolConfig {
    SystemToolConfig {
        workspace: RootMount {
            name: "workspace".to_owned(),
            path: root.path().to_path_buf(),
        },
        extra_roots: Vec::new(),
        shell_program: "/bin/sh".to_owned(),
        grep_program: "rg".to_owned(),
        environment: BTreeMap::new(),
        shell_default_duration_ms: DEFAULT_SHELL_DURATION_MS,
        shell_max_duration_ms: MAX_SHELL_DURATION_MS,
    }
}

fn backend_with(
    root: &tempfile::TempDir,
    helper: Arc<dyn HelperInvoker>,
    config: SystemToolConfig,
) -> SystemToolBackend {
    let versions = Arc::new(DurableArtifactVersions::new(root.path().join("versions")).unwrap());
    SystemToolBackend::new(
        config,
        Some(helper),
        None,
        None,
        Some(versions),
        CancellationToken::default(),
    )
}

/// The real helper server in-process: commands actually spawn, and the
/// deadline path actually signals the child's process group.
struct LiveHelper(HelperServer);

impl HelperInvoker for LiveHelper {
    fn invoke(
        &self,
        request: &HelperRequest,
        _cancellation: &CancellationToken,
    ) -> Result<HelperResponse, BackendFailure> {
        Ok(self.0.execute(request))
    }
}

fn shell_exec_timeouts(helper: &MemoryHelper) -> Vec<Option<u64>> {
    helper
        .operations
        .lock()
        .unwrap()
        .iter()
        .filter_map(|operation| match operation {
            HelperOperation::Exec(exec) => Some(exec.timeout_ms),
            _ => None,
        })
        .collect()
}

#[test]
fn shell_without_max_duration_runs_under_the_default_budget() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    let result = call(
        &mut backend,
        "shell",
        "shell-default",
        r#"{"command":"make test"}"#,
    );
    assert_eq!(result["status"], "completed");
    assert_eq!(
        shell_exec_timeouts(&helper),
        [Some(DEFAULT_SHELL_DURATION_MS)]
    );
    // An explicit null is the same omission.
    call(
        &mut backend,
        "shell",
        "shell-null",
        r#"{"command":"make test","max_duration_ms":null}"#,
    );
    assert_eq!(
        shell_exec_timeouts(&helper)[1],
        Some(DEFAULT_SHELL_DURATION_MS)
    );
    // The default is a config field, so a profile can lower or raise it.
    let helper = Arc::new(MemoryHelper::default());
    let mut config = test_config(&root);
    config.shell_default_duration_ms = 1_500;
    let mut backend = backend_with(&root, Arc::clone(&helper) as Arc<dyn HelperInvoker>, config);
    call(
        &mut backend,
        "shell",
        "shell-profile",
        r#"{"command":"make test"}"#,
    );
    assert_eq!(shell_exec_timeouts(&helper), [Some(1_500)]);
}

#[test]
fn shell_clamps_a_requested_budget_to_the_cap_and_passes_smaller_ones_through() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    call(
        &mut backend,
        "shell",
        "shell-huge",
        r#"{"command":"make test","max_duration_ms":3600000}"#,
    );
    call(
        &mut backend,
        "shell",
        "shell-small",
        r#"{"command":"make test","max_duration_ms":2500}"#,
    );
    call(
        &mut backend,
        "shell",
        "shell-at-cap",
        &format!(r#"{{"command":"make test","max_duration_ms":{MAX_SHELL_DURATION_MS}}}"#),
    );
    assert_eq!(
        shell_exec_timeouts(&helper),
        [
            Some(MAX_SHELL_DURATION_MS),
            Some(2_500),
            Some(MAX_SHELL_DURATION_MS)
        ]
    );
    const _: () = assert!(DEFAULT_SHELL_DURATION_MS <= MAX_SHELL_DURATION_MS);
}

#[test]
fn shell_kills_a_sleeping_command_group_at_the_deadline_and_marks_the_step() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    let canonical = std::fs::canonicalize(&workspace).unwrap();
    let helper = Arc::new(LiveHelper(
        HelperServer::new([RootBinding::open("workspace", &canonical).unwrap()]).unwrap(),
    ));
    let mut config = test_config(&root);
    config.workspace.path = canonical.clone();
    config.shell_default_duration_ms = 300;
    let mut backend = backend_with(&root, helper, config);
    // The shell starts a grandchild and records its pid before it hangs. Both
    // live in the group the helper created for `sh -c`.
    let script = "sleep 30 & printf '%s\\n' \"$!\" > grandchild.pid; wait";
    let started = Instant::now();
    let result = call(
        &mut backend,
        "shell",
        "shell-sleep",
        &serde_json::json!({
            "command": script,
            "steps": [{"program": "/bin/echo", "args": ["after"]}],
        })
        .to_string(),
    );
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "{:?}",
        started.elapsed()
    );
    assert_eq!(result["status"], "failed");
    assert_eq!(result["is_error"], true);
    let step = &result["steps"][0];
    assert_eq!(step["status"], "timed_out");
    assert_eq!(step["killed_after_ms"], 300);
    assert_eq!(step["budget_ms"], 300);
    assert_eq!(step["budget_source"], "default");
    assert_eq!(step["max_duration_ms"], MAX_SHELL_DURATION_MS);
    assert_eq!(result["steps"][1]["status"], "not_executed");
    // The grandchild died with the group; a plain child kill would leave it
    // sleeping for another 30 s.
    let pid: i32 = std::fs::read_to_string(canonical.join("grandchild.pid"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let mut gone = false;
    for _ in 0..200 {
        // `kill -0` only probes whether the pid still exists.
        let alive = std::process::Command::new("/bin/kill")
            .args(["-0", &pid.to_string()])
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap()
            .success();
        if !alive {
            gone = true;
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(gone, "grandchild {pid} survived the deadline");
}

#[test]
fn shell_stops_before_a_step_when_earlier_steps_used_the_whole_budget() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    let canonical = std::fs::canonicalize(&workspace).unwrap();
    let helper = Arc::new(LiveHelper(
        HelperServer::new([RootBinding::open("workspace", &canonical).unwrap()]).unwrap(),
    ));
    let mut config = test_config(&root);
    config.workspace.path = canonical;
    let mut backend = backend_with(&root, helper, config);
    // `continue` would normally run the verification step after a failure,
    // but the budget covers the whole call and the first step spent it all.
    let result = call(
        &mut backend,
        "shell",
        "shell-budget",
        &serde_json::json!({
            "command": "sleep 0.25",
            "max_duration_ms": 200,
            "failure_policy": "continue",
            "steps": [{"program": "/bin/echo", "args": ["after"]}],
        })
        .to_string(),
    );
    assert_eq!(result["status"], "failed");
    assert_eq!(result["steps"][0]["status"], "timed_out");
    assert_eq!(result["steps"][0]["budget_source"], "requested");
    assert_eq!(result["steps"][1]["status"], "not_executed");
    assert_eq!(result["steps"][1]["budget_exhausted"], true);
    assert_eq!(result["steps"][1]["budget_ms"], 200);
}

#[test]
fn supports_only_configured_fixed_system_tools() {
    let root = tempfile::tempdir().unwrap();
    let backend = backend(&root, Arc::new(MemoryHelper::default()));
    for name in [
        "apply_patch",
        "edit",
        "write",
        "read",
        "glob",
        "grep",
        "shell",
    ] {
        assert!(backend.supports(name), "missing {name}");
    }
    assert!(!backend.supports("job"));
    assert!(!backend.supports("web_fetch"));
    assert!(!backend.supports("web_search"));
    assert!(!backend.supports("context"));
    assert!(!backend.supports("git_status"));
}

#[test]
fn write_creates_replaces_and_accepts_empty_content_without_model_version_fields() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    let created = call(
        &mut backend,
        "write",
        "write-create",
        r#"{"path":"note.txt","content":"hello\n"}"#,
    );
    assert_eq!(created["artifact_version"], 1);
    assert_eq!(created["bytes"], 6);
    let replaced = call(
        &mut backend,
        "write",
        "write-replace",
        r#"{"path":"note.txt","content":""}"#,
    );
    assert_eq!(replaced["artifact_version"], 2);
    assert_eq!(replaced["bytes"], 0);
    assert_eq!(helper.files.lock().unwrap()["note.txt"], b"");
    let operations = helper.operations.lock().unwrap();
    assert!(operations.iter().any(|operation| matches!(
        operation,
        HelperOperation::Write {
            create: CreateMode::New,
            ..
        }
    )));
    assert!(
        operations
            .iter()
            .any(|operation| matches!(operation, HelperOperation::Patch { .. }))
    );
}

#[test]
fn edit_replaces_one_literal_span_and_rejects_ambiguous_or_missing_text() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    call(
        &mut backend,
        "write",
        "seed",
        r#"{"path":"note.txt","content":"alpha beta alpha"}"#,
    );
    let edited = call(
        &mut backend,
        "edit",
        "edit-one",
        r#"{"path":"note.txt","old_text":"beta","new_text":"gamma"}"#,
    );
    assert_eq!(edited["artifact_version"], 2);
    assert_eq!(
        helper.files.lock().unwrap()["note.txt"],
        b"alpha gamma alpha"
    );
    for (call_id, old) in [("ambiguous", "alpha"), ("missing", "delta")] {
        let args = serde_json::json!({"path":"note.txt","old_text":old,"new_text":"x"});
        let invocation = IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap();
        backend.execute(&execution("edit", call_id, &invocation), &invocation);
        assert_eq!(
            helper.files.lock().unwrap()["note.txt"],
            b"alpha gamma alpha"
        );
    }
}

#[test]
fn patch_infers_create_update_and_current_version_when_omitted() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    let created = call(
        &mut backend,
        "apply_patch",
        "create",
        r#"{"path":"note.txt","diff":"+hello\n+world"}"#,
    );
    assert_eq!(created["artifact_version"], 1);
    let updated = call(
        &mut backend,
        "apply_patch",
        "update",
        r#"{"path":"note.txt","diff":"@@\n hello\n-world\n+rust"}"#,
    );
    assert_eq!(updated["artifact_version"], 2);
    assert_eq!(helper.files.lock().unwrap()["note.txt"], b"hello\nrust");
}

#[test]
fn artifact_path_aliases_share_one_version_history() {
    let root = tempfile::tempdir().unwrap();
    let mut backend = backend(&root, Arc::new(MemoryHelper::default()));
    let created = call(
        &mut backend,
        "apply_patch",
        "alias-create",
        r#"{"operation":"create_file","path":"note.txt","diff":"+hello","expected_artifact_version":0,"summary":"create"}"#,
    );
    let absolute = root.path().join("note.txt");
    let arguments = serde_json::json!({"path":absolute,"offset":1,"limit":10});
    let read = call(&mut backend, "read", "alias-read", &arguments.to_string());
    assert_eq!(created["artifact_version"], 1);
    assert_eq!(
        read["artifact_version"], 1,
        "absolute and relative paths must identify the same artifact"
    );
}

#[test]
fn artifact_identity_preserves_absolute_history_and_separates_workspaces() {
    let root = tempfile::tempdir().unwrap();
    let authority = DurableArtifactVersions::new(root.path()).unwrap();
    let first = "/workspace/first/note.txt";
    let second = "/workspace/second/note.txt";
    authority.observe(first, "old").unwrap();
    let permit = authority.reserve(first, 0, "old", "write").unwrap();
    authority.commit(&permit, "new").unwrap();
    drop(authority);
    let reopened = DurableArtifactVersions::new(root.path()).unwrap();
    assert_eq!(reopened.observe(first, "new").unwrap(), 1);
    assert_eq!(reopened.observe(second, "new").unwrap(), 0);
    assert_eq!(reopened.observe(first, "new").unwrap(), 1);
}

#[test]
fn ambiguous_legacy_relative_history_is_preserved_and_rejected() {
    let root = tempfile::tempdir().unwrap();
    let authority = DurableArtifactVersions::new(root.path()).unwrap();
    authority.observe("note.txt", "old").unwrap();
    let permit = authority.reserve("note.txt", 0, "old", "legacy").unwrap();
    authority.commit(&permit, "new").unwrap();
    let log = root.path().join("artifact-versions.jsonl");
    let before = std::fs::read(&log).unwrap();
    for path in ["/workspace/first/note.txt", "/workspace/second/note.txt"] {
        assert!(matches!(
            authority.observe(path, "new"),
            Err(BackendFailure::Conflict(_))
        ));
        assert!(matches!(
            authority.reserve(path, 0, "new", "new-call"),
            Err(BackendFailure::Conflict(_))
        ));
    }
    assert_eq!(std::fs::read(log).unwrap(), before);
    assert_eq!(authority.observe("note.txt", "new").unwrap(), 1);
}

#[test]
fn create_and_update_use_durable_version_and_sha_cas() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    let created = call(
        &mut backend,
        "apply_patch",
        "create-1",
        r#"{"operation":"create_file","path":"note.txt","diff":"+hello\n+world","expected_artifact_version":0,"summary":"create"}"#,
    );
    assert_eq!(created["artifact_version"], 1);
    let updated = call(
        &mut backend,
        "apply_patch",
        "update-1",
        r#"{"operation":"update_file","path":"note.txt","diff":"@@\n hello\n-world\n+rust","expected_artifact_version":1,"summary":"update"}"#,
    );
    assert_eq!(updated["artifact_version"], 2);
    assert_eq!(helper.files.lock().unwrap()["note.txt"], b"hello\nrust");
}

#[test]
fn reserved_version_survives_authority_restart_and_fences_old_token() {
    let root = tempfile::tempdir().unwrap();
    let authority = DurableArtifactVersions::new(root.path()).unwrap();
    assert_eq!(authority.observe("a.txt", "sha-old").unwrap(), 0);
    let permit = authority.reserve("a.txt", 0, "sha-old", "call-1").unwrap();
    assert_eq!(permit.clone(), permit);
    drop(authority);

    let restarted = DurableArtifactVersions::new(root.path()).unwrap();
    assert_eq!(restarted.observe("a.txt", "sha-old").unwrap(), 1);
    assert!(matches!(
        restarted.reserve("a.txt", 0, "sha-old", "call-2"),
        Err(BackendFailure::Conflict(_))
    ));
    assert_eq!(restarted.observe("a.txt", "sha-new").unwrap(), 1);
    restarted
        .reserve("a.txt", 1, "sha-new", "call-3")
        .expect("new visible version can be reserved");
}

#[test]
fn only_one_concurrent_reservation_wins() {
    let root = tempfile::tempdir().unwrap();
    let authority = Arc::new(DurableArtifactVersions::new(root.path()).unwrap());
    authority.observe("race.txt", "sha").unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for call in ["left", "right"] {
        let authority = Arc::clone(&authority);
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            authority.reserve("race.txt", 0, "sha", call).is_ok()
        }));
    }
    barrier.wait();
    let wins = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .filter(|won| *won)
        .count();
    assert_eq!(wins, 1);
}

#[test]
fn grep_and_direct_shell_are_argv_direct() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    call(
        &mut backend,
        "grep",
        "grep-1",
        r#"{"pattern":"needle","path":null,"glob":null,"output_mode":null,"case_insensitive":null,"context_lines":null}"#,
    );
    call(
        &mut backend,
        "shell",
        "shell-1",
        r#"{"working_directory":".","max_duration_ms":null,"max_output_tokens":2000,"command":null,"program":"git","args":["status"],"steps":[],"failure_policy":"stop_on_error","writable_paths":[],"artifact_outputs":[]}"#,
    );
    let operations = helper.operations.lock().unwrap();
    let execs = operations
        .iter()
        .filter_map(|operation| match operation {
            HelperOperation::Exec(exec) => Some(exec.argv.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(execs[0][0], "rg");
    assert_eq!(execs[1], ["git", "status"]);
    let shell = operations
        .iter()
        .filter_map(|operation| match operation {
            HelperOperation::Exec(request) => Some(request),
            _ => None,
        })
        .nth(1)
        .unwrap();
    assert_eq!(shell.stdout_bytes, 8000);
    assert_eq!(shell.stderr_bytes, 8000);
}

#[test]
fn grep_accepts_a_file_path_as_the_search_target() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, Arc::clone(&helper));
    call(
        &mut backend,
        "grep",
        "grep-file",
        r#"{"pattern":"needle","path":"module.py","glob":null,"output_mode":null,"case_insensitive":null,"context_lines":null}"#,
    );
    let operations = helper.operations.lock().unwrap();
    let request = operations
        .iter()
        .find_map(|operation| match operation {
            HelperOperation::Exec(request) => Some(request),
            _ => None,
        })
        .unwrap();
    assert_eq!(request.cwd.as_ref().unwrap().path, "");
    assert_eq!(request.argv.last().unwrap(), "module.py");
}

#[test]
fn slice8_gate_64_tool_policy_secret_and_network() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let mut backend = backend(&root, helper);

    // Provider-supplied policy/backend values are rejected by the exact closed
    // schema before the backend can execute them.
    let invocation =
        IJsonValue::parse_str(r#"{"pattern":"x","path":null,"backend":"in_process"}"#).unwrap();
    assert!(matches!(
        backend.execute(&execution("glob", "override", &invocation), &invocation),
        BackendTerminal::Unavailable { ref code, .. } if code == "invalid_arguments"
    ));

    // Missing credential/network dependencies disappear from the catalog.
    assert!(!backend.supports("web_fetch"));
    assert!(!backend.supports("web_search"));

    // The concrete HTTP backend performs per-hop SSRF rejection before send;
    // no loopback response (and therefore no response secret) can be exposed.
    let http = BoundedHttpClient::new(HttpLimits::default()).unwrap();
    assert!(matches!(
        http.fetch(
            "http://127.0.0.1/secret",
            BackendGate::Ready,
            &CancellationToken::default(),
        ),
        BackendOutcome::Completed(Err(BackendFailure::Denied(_)))
    ));

    let secret = IJsonValue::parse_str(r#"{"stdout":"token sk-live-secret"}"#).unwrap();
    let SecretScan::Redacted(redacted) = SecretScanner::default().scan(&secret) else {
        panic!("secret-bearing backend output was not redacted");
    };
    assert!(
        !redacted
            .canonical_string()
            .unwrap()
            .contains("sk-live-secret")
    );
    assert!(matches!(
        SecretScanner::failing().scan(&secret),
        SecretScan::Withheld("scan_failed")
    ));
}

/// (call, turn, path, before, after)
type EditFact = (String, u64, String, Option<String>, Option<String>);
#[derive(Default)]
struct EditFacts(Mutex<Vec<EditFact>>);
impl engine::CommittedEditRecorder for EditFacts {
    fn record(
        &self,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        self.0.lock().unwrap().push((
            execution.call.clone(),
            execution.turn,
            path.to_owned(),
            before.map(str::to_owned),
            after.map(str::to_owned),
        ));
        Ok(())
    }
}

#[test]
fn successful_patch_records_exact_preimage_without_model_result_leak() {
    let root = tempfile::tempdir().unwrap();
    let helper = Arc::new(MemoryHelper::default());
    let facts = Arc::new(EditFacts::default());
    let mut backend = backend(&root, helper).with_edit_recorder(facts.clone());
    let create = r#"{"operation":"create_file","path":"note.txt","diff":"+hello\n+world","expected_artifact_version":0,"summary":"create"}"#;
    let result = call(&mut backend, "apply_patch", "create", create);
    assert!(result.get("before").is_none());
    assert!(result.get("after").is_none());
    call(
        &mut backend,
        "apply_patch",
        "update",
        r#"{"operation":"update_file","path":"note.txt","diff":"@@\n hello\n-world\n+rust","expected_artifact_version":1,"summary":"update"}"#,
    );
    let invocation = IJsonValue::parse_str(create).unwrap();
    backend.execute(
        &execution("apply_patch", "rejected-create", &invocation),
        &invocation,
    );
    let records = facts.0.lock().unwrap();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].0, "create");
    assert_eq!(records[0].1, 1);
    assert_eq!(records[0].3, None);
    assert_eq!(records[0].4.as_deref(), Some("hello\nworld"));
    assert_eq!(records[1].3, records[0].4);
    assert_eq!(records[1].4.as_deref(), Some("hello\nrust"));
    assert_eq!(records[1].2, root.path().join("note.txt").to_str().unwrap());
}
