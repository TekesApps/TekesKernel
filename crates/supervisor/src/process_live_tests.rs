#[cfg(target_os = "macos")]
#[test]
#[ignore = "real provider and real worker; scripts/run-live-process.py"]
fn real_worker_repeated_context_releases_queued_body() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    fs::create_dir_all(&root).unwrap();
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(
        &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),
    )
    .unwrap();
    write_canonical_test_json(
        &root.join("config/providers.json"),
        json!({"format":1,"revision":1,"providers":[provider.clone()]}),
    );
    write_canonical_test_json(
        &root.join("workspaces/ws/workspace.json"),
        json!({
        "format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],
        "policy":{"allowed_tools":["context_get"],"network":false,"writable_roots":[],
            "provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":180}}),
    );
    let session = "018f0000-0000-7000-8000-000000000126";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path, session);
    append_test_input(&path, session);
    let mut events: Vec<Value> = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    events[1]["content"] = json!([{"type":"text","text":"Call context_get FOUR times consecutively, one call per response, with record_ids [1] and reason [audit]. Wait for every result. After the fourth successful result, finish with exactly PROCESS_REPEAT_OK. If a later user input says 重试, treat it as an ordinary new request and reply exactly PROCESS_QUEUE_OK without tools."}]);
    write_test_events(&path, events);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets
        .publish(
            provider["credential_key"].as_str().unwrap(),
            provider::SecretRecord::Active {
                generation: 1,
                material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),
            },
        )
        .unwrap();
    let host = ProductionProcessHost::open_with_secret_store(
        &root,
        std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),
        "live-process",
        root.join(".agent"),
        secrets,
    )
    .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session)
            .unwrap()
            .expect("actual worker process");
        host.start_periodic_sweep();
        let deadline = Instant::now() + Duration::from_secs(210);
        let mut queued = false;
        loop {
            let events: Vec<Value> = fs::read_to_string(&path)
                .unwrap()
                .lines()
                .filter_map(|s| serde_json::from_str(s).ok())
                .collect();
            if !queued && events.iter().any(|e| e["kind"] == "attempt") {
                let origin = OriginTuple {
                    principal: "test".into(),
                    client: "live-process".into(),
                    target: session.into(),
                    op: "session.prompt".into(),
                    key: "queued-body".into(),
                };
                host.prompt(
                    session,
                    "2026-09-04T10:00:00.000Z",
                    &origin,
                    &MaterializedPrompt {
                        blocks: vec![Block::Text {
                            text: "重试".into(),
                        }],
                        attachments: vec![],
                        files: Vec::new(),
                    },
                    false,
                )
                .unwrap();
                queued = true;
            }
            if events.iter().filter(|e| e["kind"] == "settle").count() >= 2 {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "two turns did not settle; retained ledger at {}",
                path.display()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }));
    host.shutdown();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
    let events: Vec<Value> = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(events.iter().filter(|e| e["kind"] == "settle").count(), 2);
    let first = events
        .iter()
        .find(|e| e["kind"] == "settle" && e["turn"] == 1)
        .unwrap();
    let second = events
        .iter()
        .find(|e| e["kind"] == "settle" && e["turn"] == 2)
        .unwrap();
    for (settle, marker) in [(first, "PROCESS_REPEAT_OK"), (second, "PROCESS_QUEUE_OK")] {
        assert_eq!(settle["outcome"], "completed", "{settle}");
        assert!(settle.get("validation").is_none());
        let output = events
            .iter()
            .find(|e| e["seq"] == settle["promoted_output_seq"])
            .unwrap();
        assert_eq!(output["final_answer"], true);
        assert!(output.to_string().contains(marker), "{output}");
    }
    let opened = events
        .iter()
        .find(|e| e["kind"] == "turn_open" && e["turn"] == 2)
        .unwrap();
    let input = events
        .iter()
        .find(|e| e["kind"] == "input" && e["content"][0]["text"] == "重试")
        .unwrap();
    assert!(input["seq"].as_u64().unwrap() < first["seq"].as_u64().unwrap());
    assert!(first["seq"].as_u64().unwrap() < opened["seq"].as_u64().unwrap());
    assert_eq!(opened["trigger"]["inputs"], json!([input["seq"]]));
    let calls: Vec<_> = events
        .iter()
        .filter(|e| e["kind"] == "tool_call" && e["name"] == "context_get")
        .collect();
    assert_eq!(
        calls.len(),
        4,
        "inspect context_get records in {}",
        path.display()
    );
    for (index, call) in calls.iter().enumerate() {
        let result = events
            .iter()
            .find(|e| e["kind"] == "tool_result" && e["call"] == call["call"])
            .unwrap();
        assert_eq!(result["outcome"], "ok", "{result}");
        assert_eq!(call["turn"], 1);
        let next = calls
            .get(index + 1)
            .map_or(first["seq"].as_u64().unwrap(), |next| {
                next["seq"].as_u64().unwrap()
            });
        assert!(
            result["seq"].as_u64().unwrap() < next,
            "each repeated call must consume the previous result"
        );
        if let Some(next_call) = calls.get(index + 1) {
            assert_ne!(
                call["attempt"], next_call["attempt"],
                "calls must span model responses"
            );
        }
    }
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"Real worker process and supervisor context_get, four model responses then queued ordinary input; no artifact validator or external client transport.","first_settle":first["seq"],"second_settle":second["seq"]})).unwrap()).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real provider and validator process; scripts/run-live-process.py --scenario validator"]
fn real_validator_process_promotes_frozen_candidate() {
    run_real_validator_process(false);
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real provider and validator repair processes; scripts/run-live-process.py --scenario repair"]
fn real_validator_feedback_repairs_same_worker() {
    run_real_validator_process(true);
}

#[cfg(target_os = "macos")]
fn run_real_validator_process(repair: bool) {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    fs::create_dir_all(&root).unwrap();
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(
        &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),
    )
    .unwrap();
    write_canonical_test_json(
        &root.join("config/providers.json"),
        json!({"format":1,"revision":1,"providers":[provider.clone()]}),
    );
    write_canonical_test_json(
        &root.join("workspaces/ws/workspace.json"),
        json!({
        "format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],
        "policy":{"allowed_tools":["read","apply_patch"],"network":false,"writable_roots":[workspace],
            "provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":180}}),
    );

    use sha2::Digest;
    let session = "018f0000-0000-7000-8000-000000000127";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path, session);
    append_test_input(&path, session);
    let mut events: Vec<Value> = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let artifact = workspace.join("proof.txt");
    let initial = if repair {
        b"WRONG".as_slice()
    } else {
        b"KERNEL_VALIDATOR_OK".as_slice()
    };
    fs::write(&artifact, initial).unwrap();
    events[1]["content"] = json!([{"type":"text","text":format!("Deliver {} containing exactly KERNEL_VALIDATOR_OK. If validation.feedback reports a defect, repair that file using apply_patch in this same session, read it, and finish with exactly KERNEL_VALIDATOR_OK. The update_file diff uses only context, minus and plus lines, without file headers or a no-newline marker. Once the update and read succeed, submit the final answer; the independent validator handles revalidation.", artifact.display())}]);
    let asset = AssetStore::new(folder.join("assets"))
        .unwrap()
        .publish(b"{}")
        .unwrap();
    events.extend([
        json!({"kind":"run_start","run":"before-crash","mode":"ordinary","binary":"fixture","config_digest":"test","instruction_digest":"ins","policy":"p","recovery_ordinal":0}),
        json!({"kind":"turn_open","turn":1,"trigger":{"inputs":[2]}}),
        json!({"kind":"epoch","id":"e1","adapter":"responses","model":"m","reason":"initial","renderer":1,"system":{"asset":asset.asset,"digest":"d"},"tools":{"asset":"sha256-td","digest":"td"}}),
        json!({"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"turn":1,"attempt":"a1","epoch":"e1","admits":[{"from":2,"to":2}],"wire_digest":"w"}),
        json!({"kind":"output","turn":1,"attempt":"a1","final_answer":true,"content":[{"type":"text","text":"KERNEL_VALIDATOR_OK"}],"sealed":{"version":1,"adapter":provider["dialect"],"fragments":json!([{"type":"message","role":"assistant","content":[{"type":"output_text","text":"KERNEL_VALIDATOR_OK"}]}]).to_string()},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}),
    ]);
    for (index, event) in events.iter_mut().enumerate() {
        event["v"] = json!(1);
        event["seq"] = json!(index + 1);
        event["ts"] = json!("2026-09-04T00:00:00.000Z");
    }
    write_test_events(&path, events);
    let mut ledger = store::LockedLedger::open(&path, 1).unwrap();
    let binding = engine::ValidationBinding {
        thread: session.into(),
        worker: session.into(),
        turn: 1,
        output_seq: 8,
        snapshot: [(
            artifact.to_string_lossy().into_owned(),
            format!("sha256-{:x}", sha2::Sha256::digest(initial)),
        )]
        .into_iter()
        .collect(),
    };
    engine::begin_validation(&mut ledger, "2026-09-04T00:00:01.000Z", &binding).unwrap();
    drop(ledger);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets
        .publish(
            provider["credential_key"].as_str().unwrap(),
            provider::SecretRecord::Active {
                generation: 1,
                material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),
            },
        )
        .unwrap();
    let host = ProductionProcessHost::open_with_secret_store(
        &root,
        std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),
        "live-validator-process",
        root.join(".agent"),
        secrets,
    )
    .unwrap();
    let responder = crate::endpoint_carrier::ProductionRespondAuthority::open(
        &root,
        Arc::new(crate::endpoint_host::SessionAdmissionGates::default()),
        host.clone(),
    )
    .unwrap();
    let mut approval_deliveries = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session)
            .unwrap()
            .expect("root recovery process");
        host.start_periodic_sweep();
        let deadline = Instant::now() + Duration::from_secs(180);
        loop {
            let events: Vec<Value> = fs::read_to_string(&path)
                .unwrap()
                .lines()
                .filter_map(|s| serde_json::from_str(s).ok())
                .collect();
            for held in events.iter().filter(|e| e["kind"] == "approval_request") {
                if events
                    .iter()
                    .any(|e| e["kind"] == "approval_response" && e["call"] == held["call"])
                {
                    continue;
                }
                let call = events
                    .iter()
                    .find(|e| e["kind"] == "tool_call" && e["call"] == held["call"])
                    .unwrap();
                assert!(
                    repair && call["name"] == "apply_patch",
                    "unexpected approval: {call}"
                );
                let requested = PathBuf::from(call["args"]["path"].as_str().unwrap());
                let requested = if requested.is_absolute() {
                    requested
                } else {
                    workspace.join(requested)
                };
                let parent = requested.parent().unwrap().canonicalize().unwrap();
                assert!(
                    parent.starts_with(workspace.canonicalize().unwrap()),
                    "approval escapes disposable workspace"
                );
                let key = format!("live-repair-{}", held["seq"]);
                let authorization = endpoint::RespondAuthorization {
                    rpc_id: key.clone(),
                    request_sha256: "live-disposable-approval".into(),
                    session_id: session.into(),
                    call: call["call"].as_str().unwrap().into(),
                    grant: true,
                    answer: None,
                    origin_key: key,
                    resolution: endpoint::ResolutionOutcome::AllowedOnce,
                };
                let receipt = endpoint::RespondAuthority::author(&responder, authorization.clone())
                    .expect("single production approval author must survive worker exit");
                let duplicate = endpoint::RespondAuthority::author(&responder, authorization)
                    .expect("exact approval retry must return original durable receipt");
                assert_eq!(receipt.semantic_seq, duplicate.semantic_seq);
                approval_deliveries.push(json!({"call":call["call"],"seq":receipt.semantic_seq,"delivery":format!("{:?}",receipt.delivery)}));
            }
            if let Some(settle) = events.iter().find(|e| e["kind"] == "settle") {
                // Do not stop the host between semantic settlement and its
                // asynchronous doorbell/exit projection. Require the production
                // delivery path to expose the exact settled candidate.
                let journal: Vec<Value> = fs::read_to_string(folder.join("endpoint.jsonl"))
                    .unwrap_or_default()
                    .lines()
                    .filter_map(|line| serde_json::from_str(line).ok())
                    .collect();
                if let Some(end) = journal.iter().find(|e| e["event"]["type"] == "turn/end") {
                    assert_eq!(end["kernel_seqs"], json!([settle["seq"]]));
                    assert_eq!(end["event"]["data"]["validationOutcome"], "pass");
                    let promoted = &end["event"]["data"]["promotedMessageID"];
                    let message = journal.iter().find(|e|
                        e["event"]["type"] == "assistant/message"
                            && &e["event"]["data"]["message"]["id"] == promoted
                    ).expect("settlement must promote a delivered assistant message");
                    assert_eq!(message["kernel_seqs"], json!([settle["validation"]["promoted_output_seq"]]));
                    assert_eq!(message["event"]["data"]["sessionFinal"], true);
                    assert!(message["event"]["seq"].as_u64().unwrap() < end["event"]["seq"].as_u64().unwrap());
                    assert_eq!(journal.iter().filter(|e| e["event"]["type"] == "turn/end").count(), 1);
                    break;
                }
            }
            assert!(
                Instant::now() < deadline,
                "validator did not settle; inspect {}",
                path.display()
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }));
    host.shutdown();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
    let events: Vec<Value> = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let settle = events.iter().find(|e| e["kind"] == "settle").unwrap();
    assert_eq!(settle["outcome"], "completed");
    assert_eq!(settle["validation"]["outcome"], "pass", "{settle}");
    if repair {
        assert!(
            settle["validation"]["promoted_output_seq"]
                .as_u64()
                .unwrap()
                > 8
        );
        assert!(events.iter().any(|e| e["subkind"] == "validation.feedback"));
        assert!(
            events
                .iter()
                .any(|e| e["kind"] == "tool_call" && e["name"] == "apply_patch" && e["turn"] == 1)
        );
        assert_eq!(
            events.iter().filter(|e| e["kind"] == "turn_open").count(),
            1,
            "repair must stay in original turn and session"
        );
    } else {
        assert_eq!(settle["validation"]["promoted_output_seq"], 8);
    }
    assert_eq!(events.iter().filter(|e| e["kind"] == "settle").count(), 1);
    let spawn = events.iter().rev().find(|e| e["kind"] == "spawn").unwrap();
    let child_path = folder.join(spawn["child"].as_str().unwrap());
    let child: Vec<Value> = fs::read_to_string(&child_path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert!(child.iter().any(|e| e["kind"] == "run_start"));
    assert!(child.iter().any(|e| e["kind"] == "attempt_dispatched"));
    let verify = child
        .iter()
        .find(|e| e["kind"] == "tool_call" && e["name"] == "verify")
        .unwrap();
    assert!(child.iter().any(|e| e["kind"] == "tool_result"
        && e["call"] == verify["call"]
        && e["outcome"] == "ok"));
    assert_eq!(fs::read(&artifact).unwrap(), b"KERNEL_VALIDATOR_OK");
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","repair":repair,"approval_deliveries":approval_deliveries,"scope":"Recovered root candidate and real independently spawned validator process with live model; frozen artifact pass and exact promotion. Root generation is seeded, not a live mutation.","settle_seq":settle["seq"],"child":child_path})).unwrap()).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
fn delivery_fallback_requires_confirmed_child_exit() {
    let exited = Mutex::new(
        std::process::Command::new("/bin/sh")
            .args(["-c", "sleep 0.05; exit 0"])
            .spawn()
            .unwrap(),
    );
    assert!(child_exited_within(&exited, Duration::from_secs(1)));
    let live = Mutex::new(
        std::process::Command::new("/bin/sleep")
            .arg("5")
            .spawn()
            .unwrap(),
    );
    let still_live = !child_exited_within(&live, Duration::from_millis(20));
    let mut child = live.lock().unwrap();
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(
        still_live,
        "transport failure cannot stand in for confirmed process exit"
    );
}

#[cfg(target_os = "macos")]
#[test]
fn approval_closed_stdin_waits_for_actual_exit_before_fallback() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.path().join("workspaces/ws")).unwrap();
    write_canonical_test_json(
        &root.path().join("workspaces/ws/workspace.json"),
        json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":[],"network":false,"writable_roots":[]}}),
    );
    let bin = root.path().join("fake-worker");
    fs::write(
        &bin,
        br##"#!/bin/sh
printf '%s\n' '{"hello":{"max":2,"min":2,"proto":"tekes-worker"}}'
IFS= read -r selected
while [ ! -f "$0.exit" ]; do sleep 0.01; done
exec 0<&-
: > "$0.closed"
sleep 0.2
exit 0
"##,
    )
    .unwrap();
    fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap();
    let helper = root.path().join("tekes-helper");
    fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap();
    fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
    let session = "018f0000-0000-7000-8000-000000000128";
    let folder = root.path().join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    write_test_genesis(&folder.join("main.jsonl"), session);
    append_test_input(&folder.join("main.jsonl"), session);
    let host =
        ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent")).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session).unwrap().unwrap();
        let worker = host.live_worker(session).unwrap();
        fs::write(root.path().join("fake-worker.exit"), b"exit").unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        while !root.path().join("fake-worker.closed").exists() {
            assert!(Instant::now() < deadline, "worker never closed stdin");
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(
            worker.alive.load(Ordering::Acquire),
            "must exercise closed pipe before reader observes exit"
        );
        let response = ApprovalResponse {
            delivery: "approval".into(),
            origin: OriginTuple {
                principal: "test".into(),
                client: "test".into(),
                target: session.into(),
                op: "respond".into(),
                key: "approval".into(),
            },
            call: "call".into(),
            grant: true,
            answer: None,
        };
        assert!(host.deliver_if_live(session, &response).unwrap().is_none());
        assert!(
            child_exited_within(&worker.child, Duration::ZERO),
            "fallback requires process exit, not just closed stdin"
        );
    }));
    host.shutdown();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real worker and scripted provider; scripts/run-validator-exhaustion.py"]
fn real_validator_exhaustion_releases_queued_input() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    fs::create_dir_all(&root).unwrap();
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(workspace.join(".agent/skills")).unwrap();
    fs::write(workspace.join(".agent/skills/live-proof.md"), "Live proof skill\nSKILL_LIVE_BODY_OK\n").unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    write_canonical_test_json(&root.join("config/settings.json"),
        json!({"format":1,"revision":1,"limits":{"max_workers":8,"max_provider_leases":8}}));

    let provider: Value = serde_json::from_slice(
        &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),
    )
    .unwrap();
    write_canonical_test_json(
        &root.join("config/providers.json"),
        json!({"format":1,"revision":1,"providers":[provider.clone()]}),
    );
    write_canonical_test_json(
        &root.join("workspaces/ws/workspace.json"),
        json!({
        "format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],
        "policy":{"allowed_tools":["read","apply_patch","skill_explorer","skill"],"network":false,"writable_roots":[workspace],
            "provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":180}}),
    );

    let session = "018f0000-0000-7000-8000-000000000127";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path, session);
    append_test_input(&path, session);
    let mut initial_events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    initial_events[1]["content"] = json!([{"type":"text","text":"Create proof.txt containing exactly KERNEL_VALIDATOR_OK, then answer KERNEL_VALIDATOR_OK."}]);
    write_test_events(&path, initial_events);
    let artifact = workspace.join("proof.txt");
    assert!(!artifact.exists(), "test must start without a prewritten artifact");
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets
        .publish(
            provider["credential_key"].as_str().unwrap(),
            provider::SecretRecord::Active {
                generation: 1,
                material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),
            },
        )
        .unwrap();
    let host = ProductionProcessHost::open_with_secret_store(
        &root,
        std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),
        "live-validator-process",
        root.join(".agent"),
        secrets,
    )
    .unwrap();
    let transport_runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = transport_runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),
        transport::TransportConfig::loopback(address, transport::BearerToken::new([42; 32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let transport_handle = server.handle();
    let serve = transport_runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    let ready_deadline = Instant::now() + Duration::from_secs(30);
    while !root.join("client-ready").exists() {
        assert!(Instant::now() < ready_deadline,"external client did not subscribe");
        std::thread::sleep(Duration::from_millis(25));
    }
    let responder = crate::endpoint_carrier::ProductionRespondAuthority::open(
        &root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap();
    let public_approval = std::env::var("TEKES_PUBLIC_APPROVAL").as_deref() == Ok("1");
    let mut approval_deliveries = Vec::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session).unwrap().expect("fresh root worker");
        host.start_periodic_sweep();
        let mut queued = false;
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().filter_map(|s| serde_json::from_str(s).ok()).collect();
            for held in events.iter().filter(|e| !public_approval && e["kind"] == "approval_request") {
                if events
                    .iter()
                    .any(|e| e["kind"] == "approval_response" && e["call"] == held["call"])
                {
                    continue;
                }
                let call = events
                    .iter()
                    .find(|e| e["kind"] == "tool_call" && e["call"] == held["call"])
                    .unwrap();
                assert!(
                    call["name"] == "apply_patch",
                    "unexpected approval: {call}"
                );
                let requested = PathBuf::from(call["args"]["path"].as_str().unwrap());
                let requested = if requested.is_absolute() {
                    requested
                } else {
                    workspace.join(requested)
                };
                let parent = requested.parent().unwrap().canonicalize().unwrap();
                assert!(
                    parent.starts_with(workspace.canonicalize().unwrap()),
                    "approval escapes disposable workspace"
                );
                let key = format!("live-repair-{}", held["seq"]);
                let authorization = endpoint::RespondAuthorization {
                    rpc_id: key.clone(),
                    request_sha256: "live-disposable-approval".into(),
                    session_id: session.into(),
                    call: call["call"].as_str().unwrap().into(),
                    grant: true,
                    answer: None,
                    origin_key: key,
                    resolution: endpoint::ResolutionOutcome::AllowedOnce,
                };
                let receipt = endpoint::RespondAuthority::author(&responder, authorization.clone())
                    .expect("single production approval author must survive worker exit");
                let duplicate = endpoint::RespondAuthority::author(&responder, authorization)
                    .expect("exact approval retry must return original durable receipt");
                assert_eq!(receipt.semantic_seq, duplicate.semantic_seq);
                approval_deliveries.push(json!({"call":call["call"],"seq":receipt.semantic_seq,"delivery":format!("{:?}",receipt.delivery)}));
            }
            if !queued && events.iter().any(|e| e["kind"] == "spawn") {
                let origin = OriginTuple {principal:"test".into(),client:"exhaustion-process".into(),target:session.into(),op:"session.prompt".into(),key:"queued-body".into()};
                host.prompt(session,"2026-09-04T10:00:00.000Z",&origin,&MaterializedPrompt {blocks:vec![Block::Text {text:"重试".into()}],attachments:vec![],files:vec![]},false).unwrap();
                queued = true;
            }
            if events.iter().filter(|e| e["kind"] == "settle").count() == 2 {
                let journal: Vec<Value> = fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map(|line| serde_json::from_str(line).ok()).collect();
                let ends: Vec<_> = journal.iter().filter(|e| e["event"]["type"] == "turn/end").collect();
                if ends.len() == 2 {
                    for (end, settle) in ends.iter().zip(events.iter().filter(|e| e["kind"] == "settle")) {
                        assert_eq!(end["kernel_seqs"], json!([settle["seq"]]));
                        assert_eq!(end["event"]["data"]["validationOutcome"], settle["validation"]["outcome"]);
                    }
                    if root.join("client-receipt.json").exists() { break; }
                }
            }
            assert!(Instant::now() < deadline, "exhaustion failed to release queued input: {}",path.display());
            std::thread::sleep(Duration::from_millis(25));
        }
    }));
    host.shutdown();
    transport_handle.begin_drain();
    transport_runtime.block_on(serve).unwrap().unwrap();
    if let Err(error) = result { std::panic::resume_unwind(error); }
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    assert_eq!(fs::read(&artifact).unwrap(), b"KERNEL_VALIDATOR_OK");
    assert!(events.iter().any(|e| e["kind"] == "tool_call" && e["name"] == "apply_patch"));
    if public_approval {
        assert!(approval_deliveries.is_empty(), "fixture must not author public approval");
        let answers: Vec<_> = events.iter().filter(|e| e["kind"] == "approval_response").collect();
        assert_eq!(answers.len(), 1);
        assert_eq!(answers[0]["origin_tuple"]["op"], "respond");
        assert!(root.join("client-approval-receipt.json").exists());
    } else {
        assert_eq!(approval_deliveries.len(), 1, "real helper write must pass its approval gate once");
    }
    let first = events.iter().find(|e| e["kind"] == "settle" && e["turn"] == 1).unwrap();
    let second = events.iter().find(|e| e["kind"] == "settle" && e["turn"] == 2).unwrap();
    assert_eq!(first["validation"]["outcome"],"inconclusive");
    assert_eq!(second["validation"]["outcome"],"pass");
    let admitted = events.iter().find(|e| e["kind"] == "turn_open" && e["turn"] == 2).unwrap();
    assert!(first["seq"].as_u64().unwrap() < admitted["seq"].as_u64().unwrap());
    let input = events.iter().find(|e| e["kind"] == "input" && e["content"][0]["text"] == "重试").unwrap();
    assert!(input["seq"].as_u64().unwrap() < first["seq"].as_u64().unwrap(), "input must actually have queued before settlement");
    let spawn = events.iter().find(|e| e["kind"] == "spawn").unwrap();
    let child: Vec<Value> = fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    assert_eq!(child.iter().filter(|e| e["kind"] == "output").count(),4);
    assert!(child.iter().any(|e| e["kind"] == "error" && e.to_string().contains("exhausted its verify mandate")));
    assert!(!child.iter().any(|e| e["kind"] == "tool_call" && e["name"] == "verify"));
    let spawns: Vec<_> = events.iter().filter(|e| e["kind"] == "spawn").collect();
    assert_eq!(spawns.len(), 2, "later answer must receive independent validation");
    let second_child: Vec<Value> = fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    let verify = second_child.iter().find(|e| e["kind"] == "tool_call" && e["name"] == "verify").expect("second validator must actually verify");
    assert!(second_child.iter().any(|e| e["kind"] == "tool_result" && e["call"] == verify["call"] && e["outcome"] == "ok"));
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap()).unwrap();
}


#[cfg(target_os = "macos")]
#[test]
#[ignore = "real CF provider and process task; run-live-process.py --scenario task"]
fn real_legacy_simple_task() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(workspace.join("docs")).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"), json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    write_canonical_test_json(&root.join("config/settings.json"), json!({"format":1,"revision":1,"limits":{"max_workers":8,"max_provider_leases":8}}));
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"), json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":["task","read","apply_patch"],"network":false,"writable_roots":[workspace],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":120}}));
    let drama = std::env::var("TEKES_LEGACY_TASK_SCENARIO").as_deref() == Ok("drama");
    if drama {
        fs::create_dir_all(workspace.join("docs/drama")).unwrap();
        fs::create_dir_all(workspace.join(".agent/skills/chinese-short-drama")).unwrap();
        fs::write(workspace.join(".agent/skills/chinese-short-drama/SKILL.md"),include_str!("../../../fixtures/live/drama/kernel-skill.md")).unwrap();
        write_canonical_test_json(&root.join("workspaces/ws/workspace.json"),json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":["task","read","apply_patch","skill_explorer","skill"],"network":false,"writable_roots":[workspace],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":600}}));
    }
    let session = "018f0000-0000-7000-8000-000000000128";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path, session);
    append_test_input(&path, session);
    let mut events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    let scenario = std::env::var("TEKES_LEGACY_TASK_SCENARIO").unwrap_or_default();
    let markdown = scenario == "tasks-markdown";
    let update_code = scenario == "update-code-no-hint";
    let python = scenario == "tasks-python" || update_code;
    fs::create_dir_all(workspace.join("src")).unwrap();
    let update = scenario == "update-files-hint" || scenario == "update-files-no-hint";
    let source = "# Capital\nJapan's capital is Tokyo.\nFor this deterministic fixture, Tokyo's population is 10,000,000 (synthetic test data, not a current statistic).\n";
    fs::create_dir_all(workspace.join("markdown")).unwrap();
    if update {
        fs::write(workspace.join("markdown/1.md"), source).unwrap();
        fs::write(workspace.join("markdown/2.md"), "# Population\nBeijing has 21,000,000 people, based on markdown/1.md.\n").unwrap();
        fs::write(workspace.join("markdown/3.md"), "# People aged 60+\nBeijing has 4,200,000 people aged 60+, assuming 20% of the population in markdown/2.md.\n").unwrap();
    }
    let python_source = "def get_country_and_capital():\n    return 'Japan', 'Tokyo'\n";
    if update_code {
        fs::write(workspace.join("src/1.py"), python_source).unwrap();
        fs::write(workspace.join("src/2.py"), "import importlib\ndef make_capital_sentence():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    return 'Beijing is the capital of China.'\n").unwrap();
        fs::write(workspace.join("src/3.py"), "import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    sentence = importlib.import_module('src.2').make_capital_sentence()\n    return 'China / Beijing: ' + sentence\n").unwrap();
    }
    let prompt = if drama {
        include_str!("../../../fixtures/live/drama/prompt.txt")
    } else if update_code {
        "source artifact:\n- src/1.py\ndependent artifacts:\n- src/2.py\n- src/3.py\n\n1. understand the replationship and dependency among files: 1.py, 2.py, 3.py.\n2. 1.py has been changed, update corresponding contents of 2.py and 3.py."
    } else if scenario == "update-files-hint" {
        "source artifact:\n- markdown/1.md : named a country's capital\ndependent artifacts:\n- markdown/2.md : states the capitcal's population\n- markdown/3.md : states the 60+ population of the capital(assuming 20% of the population)\n\nnow, 1.md has been changed, update corresponding contents of 2.md and 3.md."
    } else if scenario == "update-files-no-hint" {
        "source artifact:\n- markdown/1.md\ndependent artifacts:\n- markdown/2.md\n- markdown/3.md\n\n1. understand the replationship and dependency among files: 1.md, 2.md, 3.md.\n2. 1.md has been changed, update corresponding contents of 2.md and 3.md."
    } else if python {
        "Call tool.task to start tasks:\n- Task_A: create src/1.py with a function get_country_and_capital() that returns the country \"China\" and capital \"Beijing\".\n- Task_B(depends_on: Task_A): create src/2.py with a function make_capital_sentence() that imports and calls get_country_and_capital() from src/1.py, then returns \"Beijing is the capital of China.\".\n- Task_C(depends_on: Task_A, Task_B): create src/3.py with a function make_capital_report() that imports and calls get_country_and_capital() from src/1.py and make_capital_sentence() from src/2.py, then returns a short report combining both results."
    } else if markdown {
        "Call tool.task to start tasks:\n- Task_A: query the capital of China, write result to markdown/1.md\n- Task_B(depends_on: Task_A): query the city's population, write the result to markdown/2.md\n- Task_C(depends_on: Task_A, Task_B): query the city's people aged 60+(assuming 20% of the population), write the result to markdown/3.md"
    } else {
        "Call tool.task to start tasks:\n- Task_Simple: query the capital of China, write result to docs/0.md"
    };
    events[1]["content"] = json!([{"type":"text","text":prompt}]);
    write_test_events(&path, events);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"live-task",root.join(".agent"),secrets).unwrap();
    let public_approval = std::env::var("TEKES_PUBLIC_APPROVAL").as_deref() == Ok("1");
    let live_transport = if public_approval {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
        let address = listener.local_addr().unwrap();
        let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
            endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
            Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap();
        let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),
            transport::TransportConfig::loopback(address, transport::BearerToken::new([42;32]))).unwrap();
        host.attach_streams(assembly.streams().clone());
        assembly.finish_recovery().unwrap();
        let server = assembly.into_server();
        let handle = server.handle();
        let serve = runtime.spawn(server.serve(listener));
        fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
        let ready = Instant::now()+Duration::from_secs(30);
        while !root.join("client-ready").exists() {
            assert!(Instant::now()<ready,"public task client did not subscribe");
            std::thread::sleep(Duration::from_millis(25));
        }
        Some((runtime, handle, serve))
    } else { None };
    let responder = crate::endpoint_carrier::ProductionRespondAuthority::open(&root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session).unwrap().expect("real root worker");
        host.start_periodic_sweep();
        let deadline = Instant::now()+Duration::from_secs(if drama { 900 } else if markdown || python { 300 } else { 150 });
        loop {
            let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().filter_map(|line| serde_json::from_str(line).ok()).collect();
            for entry in fs::read_dir(&folder).unwrap() {
                let file = entry.unwrap().path();
                if file.extension().and_then(|v| v.to_str()) != Some("jsonl") || file.file_name().unwrap()=="endpoint.jsonl" { continue; }
                let facts: Vec<Value> = fs::read_to_string(&file).unwrap().lines().filter_map(|line|serde_json::from_str(line).ok()).collect();
                for held in facts.iter().filter(|e| !public_approval && e["kind"]=="approval_request") {
                    if facts.iter().any(|e| e["kind"]=="approval_response" && e["call"]==held["call"]) {continue;}
                    let call=facts.iter().find(|e|e["kind"]=="tool_call" && e["call"]==held["call"]).unwrap();
                    assert!(call["name"]=="task" || call["name"]=="apply_patch", "unexpected approval {call}");
                    if call["name"]=="apply_patch" {
                        let raw=PathBuf::from(call["args"]["path"].as_str().unwrap());
                        let requested=if raw.is_absolute(){raw}else{workspace.join(raw)};
                        assert!(requested.parent().unwrap().canonicalize().unwrap().starts_with(workspace.canonicalize().unwrap()));
                    }
                    let target=if file.file_name().unwrap()=="main.jsonl" {session.to_owned()} else {format!("{session}:{}",file.file_stem().unwrap().to_str().unwrap())};
                    let key=format!("live-task-{}-{}",target,held["seq"]);
                    endpoint::RespondAuthority::author(&responder,endpoint::RespondAuthorization {rpc_id:key.clone(),request_sha256:"live-task-approval".into(),session_id:target,call:held["call"].as_str().unwrap().into(),grant:true,answer:None,origin_key:key,resolution:endpoint::ResolutionOutcome::AllowedOnce}).expect("task approval delivery");
                }
            }
            if events.iter().any(|e| e["kind"]=="settle") && (!public_approval || root.join("client-task-receipt.json").exists()) { break; }
            assert!(Instant::now()<deadline,"task did not settle; inspect retained ledgers");
            std::thread::sleep(Duration::from_millis(50));
        }
    }));
    host.shutdown();
    if let Some((runtime, handle, serve)) = live_transport {
        handle.begin_drain();
        runtime.block_on(serve).unwrap().unwrap();
    }
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    if !update && !update_code {
        assert!(events.iter().any(|e|e["kind"]=="tool_call" && e["name"]=="task"));
    }
    assert!(events.iter().any(|e|e["kind"]=="spawn"),"no delegated process");
    assert!(events.iter().any(|e|e["kind"]=="settle" && e["outcome"]=="completed"));
    let paths = if drama { vec!["docs/drama/story_brief.md","docs/drama/episode_map.md","docs/drama/final_outline.md"] } else if python { vec!["src/1.py", "src/2.py", "src/3.py"] } else if markdown || update { vec!["markdown/1.md", "markdown/2.md", "markdown/3.md"] } else { vec!["docs/0.md"] };
    let mut artifacts = serde_json::Map::new();
    for relative in &paths {
        let body = fs::read_to_string(workspace.join(relative)).expect("task deliverable");
        assert!(!body.trim().is_empty(), "empty deliverable {relative}");
        if drama {
            assert!(body.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c)), "drama artifact is not Chinese");
        } else if python {
            if update_code && *relative == "src/1.py" { assert_eq!(body, python_source, "Python source changed"); }
            let function = match *relative { "src/1.py" => "get_country_and_capital", "src/2.py" => "make_capital_sentence", _ => "make_capital_report" };
            assert!(body.contains(&format!("def {function}(")), "missing requested function {relative}");
        } else if update {
            if *relative == paths[0] {
                assert_eq!(body, source, "source artifact was changed");
            } else {
                let normalized = body.to_lowercase().replace(',', "");
                assert!(normalized.contains("tokyo") && !normalized.contains("beijing"));
                let expected = if *relative == "markdown/2.md" { ["10000000", "10 million", "1000万"] } else { ["2000000", "2 million", "200万"] };
                assert!(expected.iter().any(|value| normalized.contains(value)), "dependent artifact has wrong fixture population: {relative}");
            }
        } else if *relative == paths[0] {
            assert!(body.to_lowercase().contains("beijing") || body.contains("北京"));
        } else {
            assert!(body.chars().any(|c| c.is_ascii_digit()), "population deliverable lacks a number: {relative}");
        }
        artifacts.insert((*relative).into(), json!(body));
    }
    if markdown || (python && !update_code) {
        assert!(events.iter().filter(|e| e["kind"]=="tool_call" && e["name"]=="task").count() >= 3,
            "three delegated tasks were not started");
    }
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap()).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real worker and external client; scripts/run-live-provider-error-queue.py"]
fn real_provider_400_releases_queue_over_public_transport() {
    let question = std::env::var("TEKES_PUBLIC_QUESTION").is_ok_and(|value| value == "1");
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"),json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"),json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":if question {vec!["think","ask_user_questions"]} else {vec!["think"]},"network":false,"writable_roots":[],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":60}}));
    let session = "018f0000-0000-7000-8000-000000000129";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path,session);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),
        transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let handle = server.handle();
    let serve = runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    host.start_periodic_sweep();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = Instant::now()+Duration::from_secs(90);
        loop {
            let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().filter_map(|line|serde_json::from_str(line).ok()).collect();
            if events.iter().filter(|e|e["kind"]=="input").count()==2 {
                fs::write(root.join("queued-input-observed"),"durable").unwrap();
            }
            if root.join("client-error-queue-receipt.json").exists() {break;}
            assert!(Instant::now()<deadline,"public error/queue workflow did not finish");
            std::thread::sleep(Duration::from_millis(25));
        }
    }));
    host.shutdown();
    handle.begin_drain();
    runtime.block_on(serve).unwrap().unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    let settles: Vec<_> = events.iter().filter(|e|e["kind"]=="settle").collect();
    assert_eq!(settles.len(),2);
    assert_eq!(settles[0]["outcome"],"error");
    assert_eq!(settles[1]["outcome"],"completed");
    let inputs: Vec<_> = events.iter().filter(|e|e["kind"]=="input").collect();
    assert_eq!(inputs.len(),2);
    assert_eq!(inputs[1]["content"][0]["text"],"queued input after this turn's permanent failure");
    let opened: Vec<_> = events.iter().filter(|e|e["kind"]=="turn_open").collect();
    assert_eq!(opened.len(),2);
    assert!(inputs[1]["seq"].as_u64().unwrap()<settles[0]["seq"].as_u64().unwrap());
    assert!(settles[0]["seq"].as_u64().unwrap()<opened[1]["seq"].as_u64().unwrap());
    assert_eq!(opened[1]["trigger"]["inputs"],json!([inputs[1]["seq"]]));
    assert_eq!(events.iter().filter(|e|e["kind"]=="attempt" && e["turn"]==1).count(),1);
    assert!(events.iter().any(|e|e["kind"]=="error" && e["detail"].as_str().is_some_and(|s|s.contains("model not available"))));
    let think = events.iter().find(|e|e["kind"]=="tool_call" && e["name"]=="think").unwrap();
    assert_eq!(think["turn"],2);
    assert!(events.iter().any(|e|e["kind"]=="tool_result" && e["call"]==think["call"] && e["outcome"]=="ok"));
    if question {
        let question_call = events.iter().find(|e|e["kind"]=="tool_call" && e["name"]=="ask_user_questions").unwrap();
        let responses: Vec<_> = events.iter().filter(|e|e["kind"]=="approval_response" && e["call"]==question_call["call"]).collect();
        assert_eq!(responses.len(),1);
        assert_eq!(responses[0]["answer"],json!({"answers":[{"question":"Which option?","answer":"A"}]}));
        assert!(events.iter().any(|e|e["kind"]=="tool_result" && e["call"]==question_call["call"] && e["outcome"]=="ok"));
    }
    let answer = events.iter().find(|e|e["kind"]=="output" && e["final_answer"]==true).unwrap();
    assert_eq!(answer["turn"],2);
    assert_eq!(answer["content"],json!([{"type":"text","text":"recovered after provider failure"}]));
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap()).unwrap();
}

/// Register the fixture MCP server's `local-tools` scenario as the stdio
/// server `local` (tools `mcp__local__<name>`): resident when `always_on`,
/// otherwise schema-deferred behind `tool_search`.
#[cfg(target_os = "macos")]
fn register_local_tools_server(root: &std::path::Path, always_on: bool) {
    let fixture = std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect("built mcp-fixture-server path (TEKES_TEST_MCP_FIXTURE_SERVER)");
    fs::create_dir_all(root.join("config")).unwrap();
    let registry = mcp::McpRegistryStore::new(root.join("config"));
    registry.mutate_idempotent("save-local-tools", &mcp::McpManagementMutation::Save {
        server: mcp::McpServerConfig {
            reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "local".to_owned() },
            transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "local-tools".to_owned()], cwd: None, environment: BTreeMap::new() },
            enabled: true,
            always_on,
            protocol_mode: mcp::ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        },
        credential_fields: BTreeMap::new(),
    }).expect("register local tools MCP server");
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real configured provider and external client; run-live-process.py --scenario tool-search"]
fn real_public_deferred_tool_search() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"),json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    // TEKES_TOOL_SEARCH_SCENARIO=shipping swaps the UAT-marker helper for a
    // neutral shipping-ETA helper: same two-turn search-then-call contract for
    // routes whose content classifiers refuse the marker wording.
    let shipping = std::env::var("TEKES_TOOL_SEARCH_SCENARIO").is_ok_and(|value| value == "shipping");
    // Local tools are MCP stdio servers: the fixture server's `local-tools`
    // scenario serves uat_marker / get_shipping_eta; registered with
    // always_on=false its tools are schema-deferred and reachable only
    // through tool_search.
    let (deferred_tool, expectations): (&str, [(u64, Value, Value); 2]) = if shipping {
        ("mcp__local__get_shipping_eta",
            [(1, json!({"order_id":"LIVE-42"}), json!({"eta":"ETA-3D"})), (2, json!({"order_id":"LIVE-43"}), json!({"eta":"ETA-5D"}))])
    } else {
        ("mcp__local__uat_marker",
            [(1, json!({"marker":"FIRST_HOT_OK"}), json!({"marker":"FIRST_HOT_OK"})), (2, json!({"marker":"SECOND_HOT_OK"}), json!({"marker":"SECOND_HOT_OK"}))])
    };
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"),json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":["tool_search",deferred_tool],"network":false,"writable_roots":[],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":180}}));
    register_local_tools_server(&root, false);
    let session = "018f0000-0000-7000-8000-000000000130";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path,session);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-deferred-tool",root.join(".agent"),secrets).unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),
        transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let handle = server.handle();
    let serve = runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    host.start_periodic_sweep();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = Instant::now()+Duration::from_secs(420);
        let mut checked_launch = false;
        while !root.join("client-tool-search-receipt.json").exists() {
            assert!(!root.join("client-tool-search-failed").exists(), "external tool-search client failed; see client.log");
            if !checked_launch && fs::read_to_string(&path).unwrap().lines().count() >= 2 {
                host.ensure_running(session).expect("public deferred tool worker launch");
                checked_launch = true;
            }
            assert!(Instant::now()<deadline,"public tool-search workflow did not finish");
            std::thread::sleep(Duration::from_millis(50));
        }
    }));
    host.shutdown();
    handle.begin_drain();
    runtime.block_on(serve).unwrap().unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    let settles: Vec<_> = events.iter().filter(|e|e["kind"]=="settle").collect();
    assert_eq!(settles.len(),2);
    for (turn, expected_args, expected_output) in expectations {
        let turn_index = usize::try_from(turn).unwrap();
        assert_eq!(settles[turn_index-1]["outcome"],"completed");
        let search = events.iter().find(|e|e["kind"]=="tool_call" && e["name"]=="tool_search" && e["turn"]==turn).expect("same-turn search");
        let search_result = events.iter().find(|e|e["kind"]=="tool_result" && e["call"]==search["call"] && e["outcome"]=="ok").unwrap();
        let call = events.iter().find(|e|e["kind"]=="tool_call" && e["name"]==deferred_tool && e["turn"]==turn).expect("deferred tool call");
        assert_eq!(call["args"],expected_args);
        assert!(search_result["seq"].as_u64().unwrap()<call["seq"].as_u64().unwrap());
        let output = events.iter().find(|e|e["kind"]=="tool_result" && e["call"]==call["call"] && e["outcome"]=="ok").unwrap();
        // The durable result of an MCP tool is the MCP result envelope; the
        // local server answers with one JSON text block.
        let envelope: Value = serde_json::from_str(output["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(envelope["isError"], false, "{envelope}");
        let text = envelope["content"][0]["text"].as_str().expect("one MCP text block");
        assert_eq!(serde_json::from_str::<Value>(text).unwrap(), expected_output);
    }
    let epochs: Vec<_> = events.iter().filter(|e|e["kind"]=="epoch").collect();
    // A route with a declared native deferred mode declares every deferred
    // schema up front, so its catalog never changes and the search offer
    // travels as a native reference. Host-side activation instead grows the
    // catalog once, when the search first exposes the tool: the offer stays
    // model-visible afterwards, so later turns reuse that epoch rather than
    // resetting the provider's cached prefix on every turn boundary.
    let configured: profile::Provider = serde_json::from_value(provider.clone()).unwrap();
    let native = provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools().is_some();
    let digest = |epoch: &serde_json::Value| epoch["tools"]["digest"].as_str().unwrap().to_owned();
    assert!(!epochs.is_empty());
    if native {
        assert!(epochs.iter().all(|e| digest(e) == digest(epochs[0])), "native routing keeps one stable catalog");
    } else {
        assert_eq!(epochs.len(), 2, "the catalog grows once, at the search: {epochs:?}");
        assert_ne!(digest(epochs[0]), digest(epochs[1]));
        assert_eq!(epochs[1]["reason"], "tool_profile_change");
    }
    for attempt in events.iter().filter(|e|e["kind"]=="attempt") {
        let epoch = epochs.iter().find(|e|e["id"]==attempt["epoch"]).unwrap();
        assert_eq!(epoch["model"],provider["models"][0]["id"]);
        assert!(!epoch["system"]["digest"].as_str().unwrap().is_empty());
        assert!(!epoch["tools"]["digest"].as_str().unwrap().is_empty());
        assert!(events.iter().any(|e|e["kind"]=="usage" && e["attempt"]==attempt["attempt"]));
    }
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap()).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real configured provider and external client; run-live-process.py --scenario image"]
fn real_public_image_attachment() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"),json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    // Keep the full ordinary catalog in the vision request, including complex
    // tool schemas. Role-only tools require separate worker profiles.
    let allowed_tools: Vec<_> = tools::BuiltinManifest::compiled().tools.into_iter()
        .filter(|tool| !matches!(tool.name.as_str(), "plan" | "summary_artifact" | "report" | "verify"))
        .map(|tool| tool.name).collect();
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"),json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":allowed_tools,"network":false,"writable_roots":[],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":180}}));
    let session = "018f0000-0000-7000-8000-000000000131";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path,session);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-image",root.join(".agent"),secrets).unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),
        transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let handle = server.handle();
    let serve = runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    host.start_periodic_sweep();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = Instant::now()+Duration::from_secs(420);
        let mut checked_launch = false;
        while !root.join("client-image-receipt.json").exists() {
            assert!(!root.join("client-image-failed").exists(), "external image client failed; see client.log");
            if !checked_launch && fs::read_to_string(&path).unwrap().lines().count() >= 2 {
                host.ensure_running(session).expect("public image worker launch");
                checked_launch = true;
            }
            assert!(Instant::now()<deadline,"public image workflow did not finish");
            std::thread::sleep(Duration::from_millis(50));
        }
    }));
    host.shutdown();
    handle.begin_drain();
    runtime.block_on(serve).unwrap().unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    let settles: Vec<_> = events.iter().filter(|e|e["kind"]=="settle").collect();
    assert_eq!(settles.len(),1);
    assert_eq!(settles[0]["outcome"],"completed");
    let input = events.iter().find(|e| e["kind"]=="input").expect("public image input");
    let image = input["content"].as_array().unwrap().iter().find(|b| b["type"]=="image").expect("durable image block");
    assert_eq!(image["mime"],"image/png");
    assert!(image.get("name").is_none_or(Value::is_null),"no filename color hint");
    use base64::Engine as _;
    let expected = base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAS0lEQVR42u3PQQkAAAgAsetfWiP4FgYrsKZeS0BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEDgsqnc8OJg6Ln3AAAAAElFTkSuQmCC").unwrap();
    let asset = image["asset"].as_str().unwrap();
    assert_eq!(store::AssetStore::new(folder.join("assets")).unwrap().read_verified(asset).unwrap(),expected);
    assert!(events.iter().any(|e| e["kind"]=="usage"),"real provider usage");
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap()).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "real spawned worker and external flow client"]
fn real_public_flow_case() {
    let case: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap()).unwrap();
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"),json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    let mut policy = json!({"toolchain_roots":case["toolchain_roots"].as_array().cloned().unwrap_or_default(),"allowed_tools":case["allowed_tools"].as_array().cloned().unwrap_or_else(|| vec![json!("apply_patch"),json!("read"),json!("shell"),json!("glob"),json!("grep")]),"network":false,"writable_roots":[workspace],"provider":provider["id"],"model":provider["models"][0]["id"]});
    // A case that names no turn budget gets none: absence is the product's own
    // "no whole-turn cap", and a default here would silently cap every live
    // case that never asked for one.
    if let Some(seconds) = case["max_wall_seconds"].as_u64() {
        policy.as_object_mut().expect("policy object").insert("max_wall_seconds".to_owned(), json!(seconds));
    }
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"),json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":policy}));
    for (path, content) in case["seed_files"].as_object().unwrap() {
        let target = workspace.join(path);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, content.as_str().unwrap()).unwrap();
    }
    if case["id"] == "parallel-tools" {
        // record_left / record_right are served by the local MCP stdio server.
        register_local_tools_server(&root, true);
    }
    let session = "018f0000-0000-7000-8000-000000000129";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path,session);
    if case["id"] == "parallel-tools" {
        write_canonical_test_json(&folder.join(store::SESSION_SETTINGS_FILE),json!({"format":1,"revision":1,"provider":provider["id"],"model":provider["models"][0]["id"],"reasoning_effort":"low"}));
    }
    if case["skill_compact"] == true {
        // The named compact command is also a user-agent command (commands/<name>.md).
        fs::create_dir_all(root.join(".agent/commands")).unwrap();
        fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap();
    }
    if case["compaction"] == true {
        // The session's own model is the compactor route; the named compact
        // command is a user command.
        write_canonical_test_json(&folder.join(store::SESSION_SETTINGS_FILE),json!({"format":1,"revision":1,"provider":provider["id"],"model":provider["models"][0]["id"]}));
        fs::create_dir_all(root.join(".agent/commands")).unwrap();
        fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap();
    }

    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),
        transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let handle = server.handle();
    let serve = runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    host.start_periodic_sweep();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = Instant::now()+Duration::from_secs(case["timeout_seconds"].as_u64().unwrap_or(240));
        while !root.join("client-flow-receipt.json").exists() {
            assert!(!root.join("client-flow-failure.json").exists(), "external flow client failed; inspect client.log");
            assert!(Instant::now()<deadline,"public flow did not finish");
            std::thread::sleep(Duration::from_millis(25));
        }
    }));
    host.shutdown();
    handle.begin_drain();
    runtime.block_on(serve).unwrap().unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    let settles: Vec<_> = events.iter().filter(|e|e["kind"]=="settle").collect();
    assert_eq!(settles.len(),case["prompts"].as_array().map_or(1,Vec::len)
        +usize::from(case["queued_prompt"].is_string())+usize::from(case["goal"] == true));
    assert!(settles.iter().all(|event|event["outcome"]=="completed"));
    if case["compaction"] == true {
        // The session's own model is the compactor route; the named compact
        // command is a user command.
        write_canonical_test_json(&folder.join(store::SESSION_SETTINGS_FILE),json!({"format":1,"revision":1,"provider":provider["id"],"model":provider["models"][0]["id"]}));
        fs::create_dir_all(root.join(".agent/commands")).unwrap();
        fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap();
    }

    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let listener = runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap();
    let address = listener.local_addr().unwrap();
    let unary = crate::host_runtime::assemble_application_endpoint_host(&root,
        endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},
        Arc::new(|| Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap();
    let assembly = crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),
        transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap();
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().unwrap();
    let server = assembly.into_server();
    let handle = server.handle();
    let serve = runtime.spawn(server.serve(listener));
    fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap();
    host.start_periodic_sweep();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = Instant::now()+Duration::from_secs(case["timeout_seconds"].as_u64().unwrap_or(240));
        while !root.join("client-flow-receipt.json").exists() {
            assert!(!root.join("client-flow-failure.json").exists(), "external flow client failed; inspect client.log");
            assert!(Instant::now()<deadline,"public flow did not finish");
            std::thread::sleep(Duration::from_millis(25));
        }
    }));
    host.shutdown();
    handle.begin_drain();
    runtime.block_on(serve).unwrap().unwrap();
    if let Err(error)=result {std::panic::resume_unwind(error);}
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|line|serde_json::from_str(line).unwrap()).collect();
    let settles: Vec<_> = events.iter().filter(|e|e["kind"]=="settle").collect();
    assert_eq!(settles.len(),case["prompts"].as_array().map_or(1,Vec::len)
        +usize::from(case["queued_prompt"].is_string())+usize::from(case["goal"] == true));
    assert!(settles.iter().all(|event|event["outcome"]=="completed"));
    if case["compaction"] == true {
        // Legacy live_compaction_v5_shadow_artifact over the real ledger, now the
        // single summary: the supervisor-authored manual compact carries the
        // summary request's telemetry (frozen bundle = the compact's own covers,
        // evidence admitted) and the admitted continuation is the summary.
        let compact = events.iter().find(|e| e["kind"]=="compact" && e["origin_tuple"]["op"]=="commands/run").expect("manual compact originated by commands/run");
        let seeds = case["seed_prompts"].as_array().unwrap().len();
        let settled_before = events.iter().filter(|e| e["kind"]=="settle" && e["seq"].as_u64() < compact["seq"].as_u64()).count();
        assert_eq!(settled_before, seeds, "the compact follows the seeded turns");
        let record = &compact["summary_request"];
        assert_eq!(record["bundle"]["covers"], compact["covers"], "the frozen bundle is the compact's covers");
        assert!(record["bundle"]["sha256"].as_str().is_some_and(|d| d.starts_with("sha256-")));
        assert_eq!(record["accepted"], true, "the artifact was admitted: {record}");
        let refs = record["evidence_refs"].as_array().expect("evidence refs");
        assert!(!refs.is_empty());
        let covered: Vec<u64> = compact["covers"].as_array().unwrap().iter().flat_map(|r| r["from"].as_u64().unwrap()..=r["to"].as_u64().unwrap()).collect();
        assert!(refs.iter().all(|r| covered.contains(&r.as_u64().unwrap())), "every evidence ref is inside the bundle");
        let summary_is_model = compact["summary"].as_str().map(|summary| summary.starts_with("[compacted history]\n") && !summary.starts_with("[compacted history]\nseq "));
        if let Some(inline) = summary_is_model { assert!(inline, "the admitted continuation is the summary: {}", compact["summary"]); }
        let typed = schema::validate_ledger(&fs::read(&path).unwrap(), 1).unwrap();
        assert!(typed.events.iter().any(|e| *e.kind()==schema::EventKind::Compact), "the compact with its summary request validates");
        fs::write(root.join("compaction-summary.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap()).unwrap();
    }
    if case["skill_compact"] == true {
        // Manual compaction through commands/run: the originated compact, the
        // compaction epoch after it, and the first post-compact attempt admitting
        // the POST input — the real-ledger form of engine::first_post_compact_attempt.
        let compact = events.iter().find(|e| e["kind"]=="compact" && e["origin_tuple"]["op"]=="commands/run").expect("manual compact originated by commands/run");
        let compact_seq = compact["seq"].as_u64().unwrap();
        let origin_key = compact["origin_key"].as_str().unwrap().to_owned();
        let epoch = events.iter().find(|e| e["kind"]=="epoch" && e["seq"].as_u64().unwrap() > compact_seq).expect("epoch after the manual compact");
        assert_eq!(epoch["reason"], "compaction", "the next generation is the compaction epoch");
        let post_input = events.iter().find(|e| e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64().unwrap();
        let typed = schema::validate_ledger(&fs::read(&path).unwrap(), 1).unwrap();
        let gate = engine::first_post_compact_attempt(&typed.events, &origin_key, &[post_input]).expect("first post-compact attempt admits the POST input");
        let attempt = events.iter().find(|e| e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap().to_owned();
        let pre_calls: Vec<String> = events.iter().filter(|e| e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map(|e| e["call"].as_str().map(str::to_owned)).collect();
        assert!(!pre_calls.is_empty(), "the PRE-COMPACT turn ran its skill chain");
        assert!(compact["covers"].as_array().is_some_and(|covers| !covers.is_empty()), "manual compact covers the settled PRE-COMPACT history");
        fs::write(root.join("compact-gate.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap()).unwrap();
    }
    if case["id"]=="write" {
        assert_eq!(fs::read(workspace.join(case["expected_file"]["path"].as_str().unwrap())).unwrap(), case["expected_file"]["content"].as_str().unwrap().as_bytes());
    }
    fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap()).unwrap();
}

/// Real worker, real supervisor reader route, real MCP fixture child, real
/// configured model: a task-required MCP tool becomes a pending tool-control
/// result, the worker parks on the remote poll interval (clean exit, line
/// released), the supervisor sweep resumes it when due, the continuation
/// completes through `tool_continuation`, and the model finishes with the
/// tool's text. Exactly one tool_result; no re-execution of tools/call.
#[cfg(target_os = "macos")]
#[test]
#[ignore = "real provider, real worker and MCP fixture child; run-live-process.py --scenario mcp-task"]
fn real_public_mcp_task_continuation() {
    let root = PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").unwrap());
    fs::create_dir_all(&root).unwrap();
    let workspace = root.parent().unwrap().join("workspace");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::create_dir_all(root.join("workspaces/ws")).unwrap();
    let provider: Value = serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap();
    write_canonical_test_json(&root.join("config/providers.json"), json!({"format":1,"revision":1,"providers":[provider.clone()]}));
    write_canonical_test_json(&root.join("workspaces/ws/workspace.json"), json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],"policy":{"allowed_tools":["mcp__fixture__echo"],"network":false,"writable_roots":[],"provider":provider["id"],"model":provider["models"][0]["id"],"max_wall_seconds":240}}));
    let fixture = std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect("built mcp-fixture-server path");
    let registry = mcp::McpRegistryStore::new(root.join("config"));
    registry.mutate_idempotent("save-fixture", &mcp::McpManagementMutation::Save {
        server: mcp::McpServerConfig {
            reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "fixture".to_owned() },
            transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "task-augmented".to_owned()], cwd: None, environment: BTreeMap::new() },
            enabled: true,
            always_on: true,
            protocol_mode: mcp::ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        },
        credential_fields: BTreeMap::new(),
    }).expect("register fixture MCP server");
    let session = "018f0000-0000-7000-8000-000000000132";
    let folder = root.join("threads").join(session);
    fs::create_dir_all(folder.join("assets")).unwrap();
    let path = folder.join("main.jsonl");
    write_test_genesis(&path, session);
    append_test_input(&path, session);
    let mut events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    events[1]["content"] = json!([{"type":"text","text":"Call the mcp__fixture__echo tool once with value \"ASYNC\". Wait for its result. Then reply with exactly the text the tool returned and nothing else."}]);
    write_test_events(&path, events);
    let secrets = Arc::new(provider::MemorySecretStore::new());
    secrets.publish(provider["credential_key"].as_str().unwrap(), provider::SecretRecord::Active { generation: 1, material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap() }).unwrap();
    let host = ProductionProcessHost::open_with_secret_store(&root, std::env::var("TEKES_TEST_REAL_WORKER").unwrap(), "live-mcp-task", root.join(".agent"), secrets).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        host.schedule_main(session).unwrap().expect("actual worker process");
        host.start_periodic_sweep();
        let deadline = Instant::now() + Duration::from_secs(240);
        loop {
            let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().filter_map(|s| serde_json::from_str(s).ok()).collect();
            if events.iter().any(|e| e["kind"] == "settle") { break; }
            assert!(Instant::now() < deadline, "turn did not settle; retained ledger at {}", path.display());
            std::thread::sleep(Duration::from_millis(100));
        }
    }));
    host.shutdown();
    if let Err(error) = result { std::panic::resume_unwind(error); }
    let events: Vec<Value> = fs::read_to_string(&path).unwrap().lines().map(|s| serde_json::from_str(s).unwrap()).collect();
    let settle = events.iter().find(|e| e["kind"] == "settle").unwrap();
    assert_eq!(settle["outcome"], "completed", "{events:?}");
    let calls: Vec<_> = events.iter().filter(|e| e["kind"] == "tool_call" && e["name"] == "mcp__fixture__echo").collect();
    assert_eq!(calls.len(), 1, "one durable call");
    let call = calls[0]["call"].as_str().unwrap();
    let steps: Vec<_> = events.iter().filter(|e| e["kind"] == "state" && e["subkind"] == "tool_continuation" && e["payload"]["call"] == call).collect();
    let actions: Vec<&str> = steps.iter().map(|e| e["payload"]["action"].as_str().unwrap()).collect();
    assert_eq!(actions.first().copied(), Some("bind"), "{actions:?}");
    assert!(actions.contains(&"park"), "the remote poll interval parks the worker: {actions:?}");
    let park = steps.iter().find(|e| e["payload"]["action"] == "park").unwrap();
    assert_eq!(park["payload"]["interval_ms"], 1500);
    assert!(park["payload"]["poll_after"].as_str().is_some());
    let runs: Vec<_> = events.iter().filter(|e| e["kind"] == "run_start").collect();
    assert!(runs.len() >= 2, "the parked run exited and a resumed run finished the continuation: {}", runs.len());
    let park_seq = park["seq"].as_u64().unwrap();
    assert!(runs.iter().any(|r| r["seq"].as_u64().unwrap() > park_seq), "a run started after the park");
    let results: Vec<_> = events.iter().filter(|e| e["kind"] == "tool_result" && e["call"] == call).collect();
    assert_eq!(results.len(), 1, "exactly one tool_result");
    assert_eq!(results[0]["outcome"], "ok");
    let text = results[0]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("augmented done"), "{text}");
    assert!(results[0]["seq"].as_u64().unwrap() > park_seq);
    let final_output = events.iter().rev().find(|e| e["kind"] == "output").unwrap();
    assert_eq!(final_output["content"][0]["text"].as_str().map(str::trim), Some("augmented done"));
    assert!(events.iter().any(|e| e["kind"] == "usage"));
    fs::write(root.join("receipt.json"), serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap()).unwrap();
}
