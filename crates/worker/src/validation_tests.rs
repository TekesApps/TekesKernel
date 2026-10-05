fn validation_test_candidate(
    ledger: &mut LockedLedger,
    final_answer: bool,
) -> engine::ValidationBinding {
    let attempt = format!("validation-test-{}", ledger.next_seq());
    append_test_attempt(ledger, &attempt);
    let seq = ledger.next_seq();
    ledger
        .append_contract(
            make_event(json!({
                "v":1,"seq":seq,"turn":2,"kind":"output","ts":"2026-08-27T09:00:02.000Z",
                "attempt":attempt,"content":[{"type":"text","text":"candidate"}],
                "final_answer":final_answer,"usage":usage_object(None),
                "sealed":{"version":1,"adapter":"responses","fragments":"[]"}
            }))
            .unwrap(),
            BarrierContext::default(),
        )
        .unwrap();
    let thread = ledger.projection().unwrap().events[0]
        .string_field("thread")
        .unwrap()
        .to_owned();
    engine::ValidationBinding {
        thread: thread.clone(),
        worker: thread,
        turn: 2,
        output_seq: seq,
        snapshot: Default::default(),
    }
}

#[test]
fn validation_writer_rejects_nonfinal_and_foreign_candidates_without_writes() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let mut binding = validation_test_candidate(&mut ledger, false);
    let before = ledger.next_seq();
    assert!(engine::begin_validation(&mut ledger, "2026-08-27T09:00:02.000Z", &binding).is_err());
    assert_eq!(ledger.next_seq(), before);
    binding = validation_test_candidate(&mut ledger, true);
    binding.turn = 1;
    let before = ledger.next_seq();
    assert!(engine::begin_validation(&mut ledger, "2026-08-27T09:00:02.000Z", &binding).is_err());
    assert_eq!(ledger.next_seq(), before);
}

#[test]
fn validation_candidate_and_decision_survive_reopen_without_second_writes() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let binding = validation_test_candidate(&mut ledger, true);
    let timestamp = "2026-08-27T09:00:02.000Z";
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let decision =
        engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    let next = ledger.next_seq();
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut reopened = LockedLedger::open(&path, 1).unwrap();
    assert_eq!(
        engine::begin_validation(&mut reopened, timestamp, &binding).unwrap(),
        candidate
    );
    assert_eq!(
        engine::commit_validation_decision(&mut reopened, timestamp, candidate, candidate).unwrap(),
        decision
    );
    assert_eq!(reopened.next_seq(), next);
    let value =
        serde_json::to_value(reopened.projection().unwrap().events.last().unwrap().raw()).unwrap();
    assert_eq!(value["payload"]["decision"]["outcome"], "not_required");
    assert!(
        !reopened.projection().unwrap().terminal_tail,
        "candidate decision still needs settlement materialization"
    );
    let mut changed = binding.clone();
    changed
        .snapshot
        .insert("artifact".into(), "different".into());
    assert!(engine::begin_validation(&mut reopened, timestamp, &changed).is_err());
    assert_eq!(reopened.next_seq(), next);
    let output_before = reopened
        .projection()
        .unwrap()
        .events
        .iter()
        .find(|event| event.seq() == binding.output_seq)
        .unwrap()
        .canonical_bytes()
        .unwrap();
    let settlement =
        engine::materialize_validation_settlement(&mut reopened, timestamp, decision).unwrap();
    assert!(reopened.projection().unwrap().terminal_tail);
    let after_settle = reopened.next_seq();
    assert_eq!(
        engine::materialize_validation_settlement(&mut reopened, timestamp, decision).unwrap(),
        settlement
    );
    assert_eq!(reopened.next_seq(), after_settle);
    assert_eq!(
        reopened
            .projection()
            .unwrap()
            .events
            .iter()
            .find(|event| event.seq() == binding.output_seq)
            .unwrap()
            .canonical_bytes()
            .unwrap(),
        output_before
    );
}

#[test]
fn validation_decision_rejects_model_output_as_control_signal() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let binding = validation_test_candidate(&mut ledger, true);
    let timestamp = "2026-08-27T09:00:02.000Z";
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let before = ledger.next_seq();
    assert!(
        engine::commit_validation_decision(&mut ledger, timestamp, candidate, binding.output_seq)
            .is_err()
    );
    assert_eq!(ledger.next_seq(), before);
}

#[test]
fn validation_obligation_survives_resume_never_without_reopening_settled_turn() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    assert!(!validation_runtime::has_pending_validation(&ledger).unwrap());
    let binding = validation_test_candidate(&mut ledger, true);
    assert_eq!(ledger.projection().unwrap().lifecycle.resume_policy, ResumePolicy::Never);
    assert!(validation_runtime::has_pending_validation(&ledger).unwrap());
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut ledger = LockedLedger::open(&path, 1).unwrap();
    assert!(validation_runtime::has_pending_validation(&ledger).unwrap());
    let timestamp = "2026-08-27T09:00:02.000Z";
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let decision = engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    engine::materialize_validation_settlement(&mut ledger, timestamp, decision).unwrap();
    assert!(!validation_runtime::has_pending_validation(&ledger).unwrap());
}

#[test]
fn long_provider_reasoning_and_answer_survive_spill_and_reopen() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let options = test_options(&ledger);
    append_test_attempt(&mut ledger, "long-response");
    let text = "long model text ".repeat(2000);
    let terminal = provider::normalize_response(provider::AdapterId::Responses,
        &serde_json::to_vec(&json!({"text":text,"reasoningData":text,"isFinalAnswer":true,"functionCalls":[]})).unwrap()).unwrap();
    append_terminal(&mut ledger, TerminalAppend { options:&options, manifest:&BuiltinManifest::compiled(),
        eager:&EagerDispatch::default(), dialect:DialectId::OpenaiResponsesV1,
        server_managed:false, turn:2, attempt:"long-response" }, terminal).unwrap();
    let path = ledger.path().to_owned();
    drop(ledger);
    let ledger = LockedLedger::open(path,1).unwrap();
    let events = &ledger.projection().unwrap().events;
    let reasoning = events.iter().rev().find(|e| e.kind()==&EventKind::Reasoning).unwrap();
    let raw = serde_json::to_value(reasoning.raw()).unwrap();
    assert!(raw["content"].get("$spill").is_some());
    assert_eq!(materialize_string(&ledger,&raw["content"]).unwrap(),text);
    let raw = serde_json::to_value(events.last().unwrap().raw()).unwrap();
    assert!(raw["content"][0]["text"].is_string());
    assert_eq!(materialize_string(&ledger,&raw["content"][0]["text"]).unwrap(),text);
}

#[test]
fn validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let timestamp = "2026-08-27T09:00:02.000Z";
    let mut binding = validation_test_candidate(&mut ledger, true);
    binding
        .snapshot
        .insert("proof.txt".into(), "sha256-draft".into());
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let fork =
        engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    assert!(engine::materialize_validation_settlement(&mut ledger, timestamp, fork).is_err());
    let verdict_seq = ledger.next_seq();
    ledger.append(make_event(json!({"v":1,"seq":verdict_seq,"turn":2,"kind":"state",
        "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",
        "payload":{"candidate_seq":candidate,"verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong content"],"guidance":"fix content"}]}
    })).unwrap(), true).unwrap();
    let repair =
        engine::commit_validation_decision(&mut ledger, timestamp, candidate, verdict_seq).unwrap();
    let value =
        serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw()).unwrap();
    assert_eq!(value["payload"]["decision"]["action"], "feedback");
    assert_eq!(value["payload"]["decision"]["ordinal"], 1);
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut ledger = LockedLedger::open(path, 1).unwrap();
    let next = ledger.next_seq();
    assert_eq!(
        engine::commit_validation_decision(&mut ledger, timestamp, candidate, verdict_seq).unwrap(),
        repair
    );
    assert_eq!(ledger.next_seq(), next);
    let mut later = validation_test_candidate(&mut ledger, true);
    later.snapshot = binding.snapshot;
    let candidate2 = engine::begin_validation(&mut ledger, timestamp, &later).unwrap();
    let second_fork = engine::commit_validation_decision(&mut ledger, timestamp, candidate2, candidate2).unwrap();
    let decision = serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw()).unwrap();
    assert_eq!(decision["payload"]["decision"]["action"], "fork_validator");
    assert!(engine::materialize_validation_settlement(&mut ledger, timestamp, second_fork).is_err());
    let second_verdict = ledger.next_seq();
    ledger.append(make_event(json!({"v":1,"seq":second_verdict,"turn":2,"kind":"state",
        "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",
        "payload":{"candidate_seq":candidate2,"verdict":"fail","failures":[]}
    })).unwrap(), true).unwrap();
    let terminal = engine::commit_validation_decision(&mut ledger, timestamp, candidate2, second_verdict).unwrap();
    let value =
        serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw()).unwrap();
    assert_eq!(value["payload"]["decision"]["outcome"], "inconclusive");
    assert_eq!(
        value["payload"]["decision"]["negative_round"],
        json!([2, "validator_fail"])
    );
    engine::materialize_validation_settlement(&mut ledger, timestamp, terminal).unwrap();
    let value =
        serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw()).unwrap();
    assert_eq!(value["validation"]["promoted_output_seq"], later.output_seq);
    assert_eq!(value["validation"]["outcome"], "inconclusive");
}

#[test]
fn durable_final_settles_without_requiring_a_still_available_provider() {
    let (directory, mut ledger) = context_ledger("responses", false);
    let binding = validation_test_candidate(&mut ledger, true);
    let profile = tool_profile(directory.path(), &[]);
    assert!(selected_provider(&profile.config).is_err());
    let options = test_options(&ledger);
    let mut credential = None;
    let mut lines = std::iter::empty();
    let mut output = Vec::new();
    run_provider_turn(&mut ledger, &options, &profile, &Selected { version: 2 },
        &mut credential, &mut lines, &mut output, &RuntimeCancellation::default()).unwrap();
    let events = &ledger.projection().unwrap().events;
    let terminal = serde_json::to_value(events.last().unwrap().raw()).unwrap();
    assert_eq!(terminal["outcome"], "completed");
    assert_eq!(terminal["promoted_output_seq"], binding.output_seq);
    assert!(terminal.get("validation").is_none());
    assert!(!events.iter().any(|event| event.kind() == &EventKind::Spawn));
    assert!(!events.iter().any(|event|
        event.string_field("subkind") == Some("validation.candidate")));
}

#[test]
fn direct_final_recovery_settles_once_without_validator() {
    let (directory, mut ledger) = context_ledger("responses", false);
    let binding = validation_test_candidate(&mut ledger, true);
    let timestamp = "2026-09-04T00:00:00.000Z";
    let input_seq = ledger.next_seq();
    ledger.append_contract(make_event(json!({"v":1,"seq":input_seq,"kind":"input",
        "ts":timestamp,"content":[{"type":"text","text":"next"}],
        "origin_key":"direct-final-queued","origin_tuple":{"principal":"test",
        "client":"test","target":binding.thread,"op":"submit",
        "key":"direct-final-queued"}})).unwrap(), BarrierContext::default()).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut ledger = LockedLedger::open(&path, 1).unwrap();
    let profile = tool_profile(directory.path(), &[]);
    let options = test_options(&ledger);
    let run = ProviderRunContext { options: &options, profile: &profile,
        selected: &Selected { version: 2 } };
    assert!(validation_runtime::advance(&mut ledger, run, &mut std::iter::empty(),
        &mut Vec::new(), &RuntimeCancellation::default()).unwrap());
    let events = &ledger.projection().unwrap().events;
    assert_eq!(serde_json::to_value(events.last().unwrap().raw()).unwrap()["promoted_output_seq"],
        binding.output_seq);
    assert!(!events.iter().any(|event| event.kind() == &EventKind::Spawn));
    assert!(!events.iter().any(|event|
        event.string_field("subkind") == Some("validation.candidate")));
    let next = ledger.next_seq();
    drop(ledger);
    let ledger = LockedLedger::open(&path, 1).unwrap();
    assert!(!validation_runtime::has_pending_validation(&ledger).unwrap());
    assert_eq!(ledger.next_seq(), next);
    drop(ledger);
    let mut ledger = LockedLedger::open(&path, 1).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), Some(3));
    assert_eq!(current_turn_inputs(&ledger, 3).unwrap(), vec![input_seq]);
}

#[test]
fn settlement_recovery_rejects_a_foreign_candidate_binding_without_writes() {
    let (_directory, mut ledger) = context_ledger("responses", false);
    let binding = validation_test_candidate(&mut ledger, true);
    let timestamp = "2026-09-04T00:00:00.000Z";
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let decision = engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    let path = ledger.path().to_owned();
    drop(ledger);
    let text = fs::read_to_string(&path).unwrap();
    let mut records = text.lines().map(|line| serde_json::from_str::<Value>(line).unwrap()).collect::<Vec<_>>();
    records.iter_mut().find(|e| e["seq"] == candidate).unwrap()["payload"]["binding"]["worker"] = json!("foreign-worker");
    let changed = records.iter().fold(String::new(), |mut acc, e| { acc.push_str(&serde_json_canonicalizer::to_string(e).unwrap()); acc.push('\n'); acc });
    fs::write(&path, &changed).unwrap();
    let mut reopened = LockedLedger::open(&path, 1).unwrap();
    assert!(engine::materialize_validation_settlement(&mut reopened, timestamp, decision).is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), changed);
}

#[test]
fn failed_decision_recovery_delivers_feedback_once_to_the_same_worker() {
    let (directory, mut ledger) = context_ledger("responses", false);
    let mut binding = validation_test_candidate(&mut ledger, true);
    binding.snapshot.insert("proof.txt".into(), "sha256-fixture".into());
    let timestamp = "2026-09-04T00:00:00.000Z";
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    let source = ledger.next_seq();
    ledger.append(make_event(json!({"v":1,"seq":source,"turn":2,"kind":"state","ts":timestamp,
        "subkind":"validation.verdict","visibility":"runtime","payload":{"candidate_seq":candidate,
        "verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong contents"],"guidance":["repair contents"]}]}})).unwrap(),true).unwrap();
    let decision = engine::commit_validation_decision(&mut ledger,timestamp,candidate,source).unwrap();
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut ledger = LockedLedger::open(&path,1).unwrap();
    let profile = tool_profile(directory.path(),&[]);
    let options = test_options(&ledger);
    let selected = Selected { version: 2 };
    for _ in 0..2 {
        let run = ProviderRunContext { options:&options,profile:&profile,selected:&selected };
        assert!(!validation_runtime::advance(&mut ledger,run,&mut std::iter::empty(),&mut Vec::new(),&RuntimeCancellation::default()).unwrap());
        drop(ledger);
        ledger = LockedLedger::open(&path,1).unwrap();
    }
    let events = &ledger.projection().unwrap().events;
    let feedback = events.iter().filter(|e| e.string_field("subkind")==Some("validation.feedback")).collect::<Vec<_>>();
    assert_eq!(feedback.len(),1);
    let raw = serde_json::to_value(feedback[0].raw()).unwrap();
    assert_eq!(raw["payload"]["decision_seq"],decision);
    assert_eq!(raw["payload"]["output_seq"],binding.output_seq);
    assert_eq!(raw["payload"]["ordinal"],1);
    assert_eq!(raw["visibility"],"model");
    assert_eq!(feedback[0].turn(),Some(2));
    assert!(!ledger.projection().unwrap().terminal_tail);
    assert!(!events.iter().any(|e| e.kind()==&EventKind::Spawn));
}

#[test]
fn stop_during_validator_launch_does_not_settle_as_internal_failure() {
    let (directory, mut ledger) = context_ledger("responses", false);
    let mut binding = validation_test_candidate(&mut ledger, true);
    let artifact = directory.path().join("proof.txt");
    fs::write(&artifact,b"draft").unwrap();
    binding.snapshot.insert(artifact.to_string_lossy().into_owned(),format!("{:x}",Sha256::digest(b"draft")));
    engine::begin_validation(&mut ledger,"2026-09-04T00:00:00.000Z",&binding).unwrap();
    let profile = tool_profile(directory.path(),&[]);
    let options = test_options(&ledger);
    let cancellation = RuntimeCancellation::default();
    let stop = r#"{"stop":{"delivery":"d-stop","generation":1,"origin":{"client":"cli","key":"stop-1","op":"session.cancel","principal":"p","target":"t"}}}"#;
    let mut lines = std::iter::once(Ok(stop.to_owned()));
    run_provider_turn(&mut ledger,&options,&profile,&Selected{version:2},&mut None,
        &mut lines,&mut Vec::new(),&cancellation).unwrap();
    assert!(cancellation.stop_requested());
    assert!(!ledger.projection().unwrap().terminal_tail);
    assert!(!ledger.projection().unwrap().events.iter().any(|e| e.turn()==Some(2)
        && matches!(e.kind(),EventKind::Error|EventKind::Settle)));
    let (sender, receiver) = sync_channel(1);
    drop(sender);
    let mut control = ControlLines { receiver, cancellation: cancellation.clone() };
    let mut output = Vec::new();
    drain_ready_deliveries(&mut ledger,&options,&Selected{version:2},&mut control,&mut output).unwrap();
    let code = post_turn_exit(&mut ledger,&options,&cancellation,&mut output).unwrap();
    assert!(exit_code_is(code,ExitCode::FAILURE), "unresolved child requires supervisor tail recovery");
    let projection = ledger.projection().unwrap();
    let events = &projection.events;
    assert!(events.iter().any(|e| e.kind()==&EventKind::StopRequested));
    assert!(projection.lifecycle.stop_active);
    assert!(projection.lifecycle.unresolved_work);
    assert!(!events.iter().any(|e| e.turn()==Some(2) && e.kind()==&EventKind::Settle),
        "parent must not settle before the spawned validator is reaped");
}

#[test]
fn validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink() {
    let directory = tempfile::tempdir().unwrap();
    let profile = tool_profile(directory.path(),&[]);
    let file = directory.path().join("proof.txt");
    fs::write(&file,b"frozen contents").unwrap();
    assert_eq!(validation_runtime::artifact_bytes(&profile,"proof.txt").unwrap(),b"frozen contents");
    let link = directory.path().join("link.txt");
    std::os::unix::fs::symlink(&file,&link).unwrap();
    assert!(validation_runtime::artifact_bytes(&profile,"link.txt").is_err());
    assert!(validation_runtime::artifact_bytes(&profile,".").is_err());
    let large = fs::File::create(directory.path().join("large")).unwrap();
    large.set_len(16 * 1024 * 1024 + 1).unwrap();
    assert!(validation_runtime::artifact_bytes(&profile,"large").is_err());
}

#[test]
fn validator_profile_writes_only_to_private_scratch() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source");
    let scratch = directory.path().join("private/scratch");
    let snapshot = directory.path().join("private/snapshot");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&scratch).unwrap();
    fs::create_dir_all(&snapshot).unwrap();
    let mut profile = tool_profile(&source, &["shell", "read", "write"]);
    profile.config.workspace.policy.writable_roots = vec![source.to_string_lossy().into_owned()];
    validation_runtime::isolate_validator_profile(&mut profile, &scratch, &snapshot);
    assert_eq!(profile.config.workspace.cwd[0], scratch.to_string_lossy());
    assert_eq!(profile.config.workspace.cwd[1], snapshot.to_string_lossy());
    assert_eq!(profile.config.workspace.cwd[2], source.canonicalize().unwrap().to_string_lossy());
    assert_eq!(profile.instruction.meet_workspace_policy(&profile.config.workspace.policy).writable_roots,
        vec![scratch.to_string_lossy().into_owned()]);
    assert!(profile.validator);
}

#[test]
fn queued_retry_body_waits_for_validation_settlement_then_opens_once() {
    let (_directory, mut ledger) = context_ledger("responses", true);
    let timestamp = "2026-08-27T09:00:03.000Z";
    let binding = validation_test_candidate(&mut ledger, true);
    let input = ledger.next_seq();
    ledger.append_contract(make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,
        "content":[{"type":"text","text":"重试"}],"origin_key":"queued-retry-body",
        "origin_tuple":{"principal":"test","client":"test","target":binding.thread,
            "op":"submit","key":"queued-retry-body"}})).unwrap(), BarrierContext::default()).unwrap();
    let before = ledger.next_seq();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
    assert_eq!(ledger.next_seq(), before, "session final cannot open the queued input");
    let candidate = engine::begin_validation(&mut ledger, timestamp, &binding).unwrap();
    let decision = engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
    assert_eq!(current_turn_inputs(&ledger, 2).unwrap(), vec![9]);
    let path = ledger.path().to_owned();
    drop(ledger);
    let mut ledger = LockedLedger::open(path, 1).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
    engine::materialize_validation_settlement(&mut ledger, timestamp, decision).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Reconcile).unwrap(), None);
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), Some(3));
    assert_eq!(current_turn_inputs(&ledger, 3).unwrap(), vec![input]);
    let next = ledger.next_seq();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
    assert_eq!(ledger.next_seq(), next);
    let source = &ledger.projection().unwrap().events[(input - 1) as usize];
    assert_eq!(serde_json::to_value(source.raw()).unwrap()["content"][0]["text"], "重试");
}

#[test]
fn first_root_input_opens_after_run_start_without_prior_settlement() {
    let (_directory, ledger) = context_ledger("responses", false);
    let path = ledger.path().to_owned();
    drop(ledger);
    let prefix = fs::read_to_string(&path).unwrap().lines().take(3)
        .fold(String::new(), |mut acc, line| { acc.push_str(&format!("{line}\n")); acc });
    fs::write(&path, prefix).unwrap();
    let mut ledger = LockedLedger::open(&path, 1).unwrap();
    let timestamp = "2026-09-04T00:00:00.000Z";
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Reconcile).unwrap(), None);
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), Some(1));
    assert_eq!(current_turn_inputs(&ledger, 1).unwrap(), vec![2]);
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), None);
}

#[test]
fn queued_input_replays_after_previous_tool_result_not_at_storage_position() {
    let (_directory, mut ledger) = context_ledger("deepseek_responses_v1", false);
    let timestamp = "2026-08-27T09:00:03.000Z";
    append_test_attempt(&mut ledger, "queued-replay");
    append_test_call(&mut ledger, "queued-replay", "queued-read", "think", json!({"thought":"inspect"}));
    let seq = ledger.next_seq();
    ledger.append_contract(make_event(json!({"v":1,"seq":seq,"kind":"output","turn":2,"ts":timestamp,
        "attempt":"queued-replay","content":[],"final_answer":false,"usage":usage_object(None),
        "sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[{\"type\":\"function_call\",\"call_id\":\"queued-read\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}]"}})).unwrap(), BarrierContext::default()).unwrap();
    let input = ledger.next_seq();
    ledger.append_contract(make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,
        "content":[{"type":"text","text":"QUEUED_BODY_BOUNDARY"}],"origin_key":"queued-replay",
        "origin_tuple":{"principal":"test","client":"test","target":"test","op":"submit","key":"queued-replay"}})).unwrap(), BarrierContext::default()).unwrap();
    let result = ledger.next_seq();
    ledger.append_contract(make_event(json!({"v":1,"seq":result,"kind":"tool_result","turn":2,"ts":timestamp,
        "call":"queued-read","outcome":"ok","content":[{"type":"text","text":"RESULT_BOUNDARY"}]})).unwrap(), BarrierContext::default()).unwrap();
    assert!(input < result, "fixture must reproduce queued arrival before tool result");
    let before = project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 2).unwrap();
    assert!(!serde_json::to_string(&before.items).unwrap().contains("QUEUED_BODY_BOUNDARY"));
    append_settle(&mut ledger, timestamp, 2, "completed", None).unwrap();
    assert_eq!(open_ready_turn(&mut ledger, timestamp, RunMode::Ordinary).unwrap(), Some(3));
    let context = project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 3).unwrap();
    let rendered = context.items.iter().map(|item| serde_json::to_string(item).unwrap()).collect::<Vec<_>>();
    let result_position = rendered.iter().position(|item| item.contains("RESULT_BOUNDARY")).unwrap();
    let input_position = rendered.iter().position(|item| item.contains("QUEUED_BODY_BOUNDARY")).unwrap();
    assert!(result_position < input_position, "model replay must close the prior tool call before queued input");
}

#[test]
fn deepseek_request_error_is_visible_and_never_becomes_a_final_candidate() {
    let (_directory, mut ledger) = context_ledger("deepseek_responses_v1", false);
    let options = test_options(&ledger);
    append_test_attempt(&mut ledger, "request-rejection");
    let before = ledger.next_seq();
    let terminal = provider::normalize_dialect_response(DialectId::DeepseekResponsesV1,
        br#"{"error":{"code":"invalid_request_error","message":"No tool output found for tool call fixture."}}"#).unwrap();
    let outcome = append_provider_terminal_error(&mut ledger, &options, 2, "request-rejection", &terminal).unwrap();
    assert_eq!(outcome.settle, Some(("error", Some("provider_terminal"))));
    assert!(!outcome.retry && !outcome.compact_retry);
    let new = ledger.projection().unwrap().events.iter().filter(|event| event.seq() >= before).collect::<Vec<_>>();
    assert!(!new.iter().any(|event| matches!(event.kind(), EventKind::Output | EventKind::State)));
    let error = new.iter().find(|event| event.kind() == &EventKind::Error).unwrap();
    assert_eq!(error.string_field("classification"), Some("provider_terminal"));
    assert!(error.string_field("detail").unwrap().contains("No tool output found"));
    assert!(!error.string_field("detail").unwrap().contains("lacks status"));
}

#[test]
fn validator_mandate_counts_verify_attempts_not_progress_outputs() {
    let (_directory, mut ledger) = context_ledger("responses", false);
    // The fixture already has a completed earlier turn and its output.
    for _ in 1..=4 {
        validation_test_candidate(&mut ledger, true);
        let path = ledger.path().to_owned();
        drop(ledger);
        ledger = LockedLedger::open(&path, 1).unwrap();
        let projection = ledger.projection().unwrap();
        assert!(!validation_runtime::validator_mandate_exhausted(&projection.events, 2));
        assert!(!projection.terminal_tail, "mandate exhaustion must be settled by the owning execution path");
    }
    for count in 1..=4 {
        let seq = ledger.next_seq();
        ledger.append(make_event(json!({"v":1,"seq":seq,"turn":2,"kind":"tool_call",
            "ts":"2026-08-27T09:00:02.000Z","call":format!("verify-{count}"),
            "name":"verify","attempt":"validation-test","source":"provider",
            "args":{"covered_set":[],"verdict":"inconclusive","failures":[]}})).unwrap(), true).unwrap();
        assert_eq!(validation_runtime::validator_mandate_exhausted(&ledger.projection().unwrap().events, 2), count == 4);
    }
}
