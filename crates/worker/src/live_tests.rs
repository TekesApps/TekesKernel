// Real HTTP/provider, production worker loop, credential protocol, and local
// tool backends, and the production read-only supervisor tool-control handler.
// Leases and child execution are in-process; process mutation authority is unavailable.
type LiveValueHook = std::rc::Rc<dyn Fn(&serde_json::Value) -> std::io::Result<()>>;
type LiveBytesHook = std::rc::Rc<dyn Fn(&[u8]) -> std::io::Result<String>>;
#[derive(Clone, Default)]
struct LiveControl(std::rc::Rc<std::cell::RefCell<Vec<u8>>>, Option<LiveValueHook>, Option<LiveBytesHook>);

impl std::io::Write for LiveControl {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

impl Iterator for LiveControl {
    type Item = std::io::Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.0.borrow().clone();
        let last = bytes.split(|byte| *byte == b'\n').rev().find(|line| !line.is_empty())?;
        let message: serde_json::Value = serde_json::from_slice(last).ok()?;
        if let Some(launch) = message.get("launch_child") {
            let callback = self.1.as_ref()?;
            if let Err(error) = callback(launch) { return Some(Err(error)); }
            return Some(Ok(serde_json::json!({"launch_result":{"child":launch["child"],"spawn_id":launch["spawn_id"],"ok":true}}).to_string()));
        }
        if message.get("tool_control").is_some() {
            return Some(self.2.as_ref()?(last));
        }
        let attempt = message.get("lease_request")?.get("attempt")?.as_str()?;
        Some(Ok(serde_json::json!({"lease":{"attempt":attempt,"granted":true}}).to_string()))
    }
}

#[test]
#[ignore = "real provider credentials required; run scripts/run-live-kernel.py"]
fn live_kernel_turn() {
    let config_path = std::env::var("TEKES_KERNEL_LIVE_PROVIDER").expect("provider JSON path");
    let selected_provider: profile::Provider = serde_json::from_slice(&std::fs::read(config_path).unwrap()).unwrap();
    let model = std::env::var("TEKES_KERNEL_LIVE_MODEL").unwrap_or_else(|_| selected_provider.models[0].id.clone());
    let prompt = std::env::var("TEKES_KERNEL_LIVE_PROMPT").expect("scenario prompt");
    let expected = std::env::var("TEKES_KERNEL_LIVE_EXPECT").expect("answer assertion");
    let required_tools = std::env::var("TEKES_KERNEL_LIVE_TOOLS").unwrap_or_default();
    let artifact = std::path::PathBuf::from(std::env::var("TEKES_KERNEL_LIVE_ARTIFACT").expect("artifact directory"));
    std::fs::create_dir_all(&artifact).unwrap();
    let workspace = tempfile::tempdir().unwrap();
    let (_directory, ledger) = context_ledger(&selected_provider.dialect, false);
    let source = ledger.path().to_owned();
    drop(ledger);
    // Keep the durable ledger in the evidence directory from the first write:
    // process timeout/termination must not erase the only diagnostic transcript.
    let storage_root = artifact.join("runtime-host");
    let runtime_folder = storage_root.join("threads").join("018f0000-0000-7000-8000-000000000003");
    std::fs::create_dir_all(runtime_folder.join("assets")).unwrap();
    // Keep the evidence path stable while using the production storage layout.
    std::os::unix::fs::symlink(&runtime_folder, artifact.join("runtime-thread")).unwrap();
    let path = runtime_folder.join("main.jsonl");
    std::fs::copy(&source, &path).unwrap();
    for entry in std::fs::read_dir(source.parent().unwrap().join("assets")).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), runtime_folder.join("assets").join(entry.file_name())).unwrap();
        }
    }
    let original = std::fs::read_to_string(&path).unwrap();
    let mut prefix = original.lines().take(4).map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap()).collect::<Vec<_>>();
    let prompt = format!("Workspace directory: {}. Use this absolute directory for every filesystem or shell operation. {prompt}", workspace.path().canonicalize().unwrap().display());
    prefix[1]["content"] = serde_json::json!([{"type":"text","text":prompt}]);
    let bytes = prefix.iter().fold(String::new(), |mut acc, event| { acc.push_str(&serde_json_canonicalizer::to_string(event).unwrap()); acc.push('\n'); acc });
    std::fs::write(&path, bytes).unwrap();
    let mut ledger = store::LockedLedger::open(&path, 1).unwrap();
    if prompt.contains("repeated identical read test") {
        std::fs::write(workspace.path().join("repeated.txt"), "KERNEL_REPEATED_OK").unwrap();
    }
    let compile_python = std::env::var("TEKES_KERNEL_LIVE_SKILL_FIXTURE").as_deref() == Ok("legacy_compile_python");
    if compile_python {
        std::fs::create_dir_all(workspace.path().join("src")).unwrap();
        for (name, body) in [
            ("1.py", "def get_country_and_capital():\n    return 'China', 'Beijing'\nif __name__ == '__main__':\n    print('SOURCE_EXECUTED', *get_country_and_capital())\n"),
            ("2.py", "import importlib\ndef make_capital_sentence():\n    country, capital = importlib.import_module('1').get_country_and_capital()\n    return f'{capital} is the capital of {country}.'\nif __name__ == '__main__':\n    print('SENTENCE_EXECUTED', make_capital_sentence())\n"),
            ("3.py", "import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('1').get_country_and_capital()\n    return country + ': ' + importlib.import_module('2').make_capital_sentence()\nif __name__ == '__main__':\n    print('REPORT_EXECUTED', make_capital_report())\n"),
        ] {
            std::fs::write(workspace.path().join("src").join(name), body).unwrap();
        }
    }
    let goal_enabled = std::env::var("TEKES_KERNEL_LIVE_GOAL").as_deref() == Ok("1");
    let mut selected_tools = required_tools.split(',').filter(|name| !name.is_empty()).collect::<Vec<_>>();
    if goal_enabled { selected_tools.push("set_goal_state"); }
    let mut profile = tool_profile(workspace.path(), &selected_tools);
    if selected_tools.contains(&"skill_explorer") {
        let weather = std::env::var("TEKES_KERNEL_LIVE_SKILL_FIXTURE").as_deref() == Ok("legacy_weather_skill");
        let (name, body) = if weather {
            ("weather", "---\nschema_version: 1\nname: weather\ndescription: Deterministic one-line Chinese weather observation formatter\n---\n# Weather observation formatter\n\nUse only the observation supplied by the user. Do not fetch, estimate, update, or add weather facts. Return exactly one line in this format:\n\n上海天气｜28°C｜小雨｜湿度81%｜观测时间 2026-08-22 06:00 Asia/Shanghai\n\nTranslate `light rain` to `小雨`. Add no heading, explanation, source note, markdown, or follow-up question.\n")
        } else {
            ("live-proof", "---\nname: live-proof\ndescription: Live proof skill\n---\nAfter loading this skill, reply exactly CF_SKILL_BODY_VERIFIED_7291.\n")
        };
        profile.instruction.sources.push(profile::InstructionSource {
            origin: profile::InstructionOrigin::User, path: format!("skills/{name}/SKILL.md"),
            kind: profile::InstructionKind::Skill, content: body.into(),
            content_sha256: format!("{:x}", Sha256::digest(body.as_bytes())),
        });
        profile.instruction.effective.skills.insert(format!("{name}/SKILL.md"), 0);
    }

    profile.config.workspace.policy.max_wall_seconds = Some(std::env::var("TEKES_KERNEL_LIVE_MAX_WALL_SECONDS").map(|value| value.parse::<u64>().expect("positive live wall timeout")).unwrap_or(180));
    profile.config.workspace.policy.writable_roots = profile.config.workspace.cwd.clone();
    profile.config.workspace.policy.toolchain_roots = serde_json::from_str(
        &std::env::var("TEKES_KERNEL_LIVE_TOOLCHAIN_ROOTS").unwrap_or_else(|_| "[]".to_owned())
    ).expect("explicit toolchain roots JSON");
    profile.config.workspace.policy.provider = Some(selected_provider.id.clone());
    profile.config.workspace.policy.model = Some(model.clone());
    if prompt.contains("STEP_1_CLOSING_SPEED") {
        profile.config.session_settings = Some(profile::SessionSettings {
            format: 1, revision: 1, provider: selected_provider.id.clone(),
            model, reasoning_effort: Some("high".to_owned()),
        });
        profile.config.revisions.session_settings = Some(1);
    }
    profile.config.providers.providers = vec![selected_provider.clone()];
    profile.config.providers.revision = 1;
    profile.config.revisions.providers = 1;
    if goal_enabled {
        let session = "018f0000-0000-7000-8000-000000000003";
        let objective = if prompt.contains("goal continuation test") {
            format!("Follow the user's two-turn goal continuation test. First end one turn with ROUND_ONE while leaving this goal active. On the next turn, call set_goal_state with state complete and finish with {expected}.")
        } else {
            format!("Complete the user's exact live test request and mark this whole goal complete with set_goal_state before the final answer. Required final answer: {expected}")
        };
        let created = session_controls::execute_goal(&storage_root, "goals.edit",
            &json!({"sessionId":session,"ref":{"id":"new","revision":0},"objective":objective}), None).unwrap();
        profile.bindings = profile::LaunchBindings::bind(&profile.config,
            Some(created["id"].as_str().unwrap().to_owned()), Default::default()).unwrap();
    } else {
        profile.bindings = profile::LaunchBindings::bind(&profile.config, None, Default::default()).unwrap();
    }
    let secret = std::env::var("TEKES_KERNEL_LIVE_KEY").expect("credential environment variable");
    assert!(!secret.is_empty(), "empty credential is not a live test");
    let (supervisor, worker) = std::os::unix::net::UnixStream::pair().unwrap();
    let (control, service) = provider::start_credential_channel(supervisor, provider::CredentialBroker::new([provider::CredentialScope {
        credential_id: selected_provider.credential_key.clone().unwrap(),
        adapter: selected_provider.adapter.clone(),
        endpoint_origin: provider::endpoint_origin(&selected_provider.endpoint).unwrap(),
        purpose: "provider".to_owned(), generation: "1".to_owned(), material: secret,
    }]));
    let mut credential = Some(std::sync::Arc::new(std::sync::Mutex::new(provider::CredentialClient::new(worker))));
    let mut output = LiveControl::default();
    let handler = tekes_supervisor::production_tool_control::ProductionToolControlHandler::new(
        &storage_root, UnavailableLiveEffects, UnavailableLiveEffects);
    let policy = tekes_supervisor::production_tool_control::ProductionToolControlPolicy::new(SecretScanner::default());
    let session = std::rc::Rc::new(std::cell::RefCell::new(tekes_supervisor::tool_control::ToolControlSession::new(
        &storage_root, Selected { version: 2 }, handler, policy)));
    output.2 = Some(std::rc::Rc::new(move |line| {
        let response = session.borrow_mut().handle_line(line).map_err(|error| std::io::Error::other(error.to_string()))?;
        String::from_utf8(response).map_err(std::io::Error::other)
    }));
    let child_folder = ledger.path().parent().unwrap().to_owned();
    let child_profile = profile.clone();
    let child_credential = credential.clone();
    let child_control = output.2.clone();
    output.1 = Some(std::rc::Rc::new(move |launch| {
        let execute = || -> Result<(), Box<dyn std::error::Error>> {
            let child_id = launch["child"].as_str().ok_or("child id missing")?;
            let mut child = LockedLedger::open(child_folder.join(format!("{child_id}.jsonl")), 1)?;
            let mut options = test_options(&child);
            options.run_id = format!("validator-{child_id}");
            if child.projection().unwrap().lifecycle.unstarted {
                append_run_start(&mut child, &options, RunMode::Ordinary, 0)?;
                append_turn_open(&mut child, &options.timestamp, "genesis", &[], 1)?;
            }
            let mut child_output = LiveControl { 2: child_control.clone(), ..Default::default() };
            let mut child_replies = child_output.clone();
            let mut credential = child_credential.clone();
            let cancellation = RuntimeCancellation::default();
            for ordinal in 0..16 {
                run_provider_turn(&mut child, &options, &child_profile, &Selected {version:2}, &mut credential,
                    &mut child_replies, &mut child_output, &cancellation)?;
                if child.projection().unwrap().terminal_tail { return Ok(()); }
                let pending = pending_tool_calls(&child)?;
                let held = pending.iter().filter(|call| call.approval_requested && call.approval_response.is_none()).collect::<Vec<_>>();
                if held.is_empty() { return Err("live validator stopped without settlement or approval".into()); }
                for call in held {
                    if !effective_allowed_tools(&child_profile).contains(&call.name) { return Err("validator approval outside live scenario".into()); }
                    let key = format!("live-validator-{ordinal}-{}",call.call);
                    child.append_contract(make_event(json!({"v":1,"seq":child.next_seq(),"turn":call.turn,"kind":"approval_response",
                        "ts":options.timestamp,"call":call.call,"grant":true,"origin_key":key,
                        "origin_tuple":{"principal":"live-test","client":"live-harness","target":child_id,"op":"approve","key":key}}))?,BarrierContext::default())?;
                }
                recover_unpaired_tool_calls(&mut child,&options,Some(&child_profile),Selected {version:2},false,
                    &mut child_replies,&mut child_output,&cancellation)?;
            }
            Err("validator exceeded live approval bound".into())
        };
        execute().map_err(|error| std::io::Error::other(error.to_string()))
    }));
    let mut replies = output.clone();
    let mut options = test_options(&ledger);
    if goal_enabled {
        options.timestamp = (chrono::Utc::now() + chrono::Duration::seconds(1))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    }
    let started = std::time::Instant::now();
    let cancellation = RuntimeCancellation::default();
    if prompt.contains("queued validation test") {
        cancellation.park_delivery(json!({"input":{"assets":null,
            "content":[{"type":"text","text":"Do not call tools. Reply exactly KERNEL_QUEUE_OK."}],
            "delivery":"live-queued-second","origin":{"client":"live-harness","key":"queued-second",
                "op":"submit","principal":"live-test","target":"test"},"steer":null}}).to_string());
        drain_parked_deliveries(&mut ledger, &options, &Selected {version:2}, &cancellation, &mut output).unwrap();
        assert_eq!(open_ready_turn(&mut ledger, &options.timestamp, engine::RunMode::Ordinary).unwrap(), None);
    }
    let mut result = run_provider_turn(&mut ledger, &options, &profile, &Selected { version: 2 }, &mut credential, &mut replies, &mut output, &cancellation);
    if goal_enabled {
        for _ in 0..session_controls::MAX_GOAL_ROUNDS {
            if result.is_err() || !open_goal_continuation(&mut ledger, &options, &profile, &cancellation).unwrap() { break; }
            result = run_provider_turn(&mut ledger, &options, &profile, &Selected { version: 2 }, &mut credential, &mut replies, &mut output, &cancellation);
        }
    }
    // Exercise the actual durable approval/resume path for the explicitly
    // selected local test tools, within this disposable workspace.
    for ordinal in 0..16 {
        if result.is_err() || started.elapsed().as_secs() > 180 { break; }
        let pending = pending_tool_calls(&ledger).unwrap();
        let held = pending.iter().filter(|call| call.approval_requested && call.approval_response.is_none()).collect::<Vec<_>>();
        if held.is_empty() { break; }
        for call in held {
            assert!(selected_tools.contains(&call.name.as_str()), "approval outside the selected test scenario");
            let key = format!("live-approval-{ordinal}-{}", call.call);
            let event = make_event(serde_json::json!({"v":1,"seq":ledger.next_seq(),"turn":call.turn,"kind":"approval_response","ts":options.timestamp,"call":call.call,"grant":true,"answer":"approved disposable live-test operation","origin_key":key,"origin_tuple":{"principal":"live-test","client":"live-harness","target":"test","op":"approve","key":key}})).unwrap();
            ledger.append_contract(event, store::BarrierContext::default()).unwrap();
        }
        result = recover_unpaired_tool_calls(&mut ledger, &options, Some(&profile), Selected { version: 2 }, false, &mut replies, &mut output, &cancellation).map(|_| ());
        if result.is_ok() {
            result = run_provider_turn(&mut ledger, &options, &profile, &Selected { version: 2 }, &mut credential, &mut replies, &mut output, &cancellation);
        }
    }
    // Save evidence before assertions, including unsuccessful provider runs.
    std::fs::copy(&path, artifact.join("main.jsonl")).unwrap();
    let asset_source = path.parent().unwrap().join("assets");
    let asset_destination = artifact.join("assets");
    std::fs::create_dir_all(&asset_destination).unwrap();
    for entry in std::fs::read_dir(&asset_source).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(),asset_destination.join(entry.file_name())).unwrap();
        }
    }
    for entry in std::fs::read_dir(path.parent().unwrap()).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s|s.to_str()) == Some("jsonl") {
            std::fs::copy(entry.path(), artifact.join(entry.file_name())).unwrap();
        }
    }
    std::fs::write(artifact.join("control.jsonl"), &*output.0.borrow()).unwrap();
    let events = &ledger.projection().unwrap().events;
    let attempts = events.iter().filter(|event| *event.kind() == schema::EventKind::AttemptDispatched).count();
    let calls = events.iter().filter(|event| *event.kind() == schema::EventKind::ToolCall).filter_map(|event| event.string_field("name")).collect::<Vec<_>>();
    let answers = events.iter().filter(|event| *event.kind() == schema::EventKind::Output).map(|event| serde_json::to_value(event.raw()).unwrap()).collect::<Vec<_>>();
    let settlement = events.last().map(|event| serde_json::to_value(event.raw()).unwrap()).unwrap_or(Value::Null);
    let validation = settlement.get("validation").cloned().unwrap_or(Value::Null);
    let promoted = settlement["promoted_output_seq"].as_u64().or_else(|| validation["promoted_output_seq"].as_u64());
    let final_output = answers.iter().find(|event| event["seq"].as_u64() == promoted && event["final_answer"] == true);
    let answer = final_output.map(|event| event["content"].as_array().unwrap().iter().filter_map(|block| block["text"].as_str()).collect::<String>()).unwrap_or_default();
    let completed = events.last().is_some_and(|event| *event.kind() == schema::EventKind::Settle && event.string_field("outcome") == Some("completed"));
    let goal_phase = if goal_enabled { session_controls::read_goal(&storage_root, "018f0000-0000-7000-8000-000000000003").unwrap().map(|goal| goal.phase) } else { None };
    let report = serde_json::json!({"provider":selected_provider.id,"elapsed_seconds":started.elapsed().as_secs_f64(),"attempts":attempts,"tools":calls,"completed":completed,"validation":validation,"final_answer":final_output.is_some(),"answer":answer,"worker_ok":result.is_ok(),"goal_phase":goal_phase});
    std::fs::write(artifact.join("result.json"), serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    result.expect("worker turn");
    assert!(attempts > 0, "zero requests is not live evidence");
    assert!(completed && final_output.is_some(), "no successful final answer; inspect saved ledger");
    if goal_enabled { assert_eq!(goal_phase.as_deref(), Some("complete"), "goal did not complete"); }
    if prompt.contains("Task_Simple: query the capital of China") {
        assert!(calls.contains(&"task"), "task was not called");
        let file = workspace.path().join("docs/0.md");
        let body = std::fs::read_to_string(&file).expect("task deliverable docs/0.md");
        assert!(body.to_lowercase().contains("beijing") || body.contains("北京"), "incorrect capital artifact");
        std::fs::write(artifact.join("task-deliverable.md"), body).unwrap();
    } else if compile_python {
        assert!(!answer.trim().is_empty());
        let shell_ids = events.iter().filter(|e| e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("shell"))
            .filter_map(|e| e.string_field("call")).collect::<std::collections::HashSet<_>>();
        let mut lines = std::collections::HashSet::new();
        for event in events.iter().filter(|e| e.kind() == &EventKind::ToolResult && e.string_field("outcome") == Some("ok")
            && e.string_field("call").is_some_and(|call| shell_ids.contains(call))) {
            let raw = serde_json::to_value(event.raw()).unwrap();
            for block in raw["content"].as_array().unwrap() {
                let Ok(result) = serde_json::from_str::<Value>(block["text"].as_str().unwrap_or_default()) else { continue; };
                if result["is_error"] != false { continue; }
                for step in result["steps"].as_array().into_iter().flatten() {
                    if step["exit_code"] == 0 && step["status"] == "completed" {
                        for line in step["stdout"]["data"].as_str().unwrap_or_default().lines() { lines.insert(line.to_owned()); }
                    }
                }
            }
        }
        for expected in ["SOURCE_EXECUTED China Beijing", "SENTENCE_EXECUTED Beijing is the capital of China.", "REPORT_EXECUTED China: Beijing is the capital of China."] {
            assert!(lines.contains(expected), "missing successful Python execution line {expected}");
        }
    } else if prompt.contains("What is the capital of Japan?") {
        assert!(answer.to_lowercase().contains("tokyo"), "capital answer missing Tokyo: {answer}");
    } else {
        assert_eq!(answer.trim(), expected, "final answer does not satisfy scenario");
    }
    if selected_tools.contains(&"skill_explorer") {
        let raw = events.iter().map(|event| serde_json::to_value(event.raw()).unwrap()).collect::<Vec<_>>();
        let mut prior = 0;
        for name in ["skill_explorer", "skill"] {
            let call = raw.iter().find(|event| event["kind"]=="tool_call" && event["name"]==name).expect("required skill call");
            let result = raw.iter().find(|event| event["kind"]=="tool_result" && event["call"]==call["call"]).expect("skill result");
            assert_eq!(result["outcome"], "ok");
            assert!(call["seq"].as_u64().unwrap()>prior);
            prior = result["seq"].as_u64().unwrap();
        }
    }

    if prompt.contains("STEP_1_CLOSING_SPEED") {
        let calls = events.iter().filter(|event| event.kind() == &EventKind::ToolCall
            && event.string_field("name") == Some("think")).collect::<Vec<_>>();
        assert_eq!(calls.len(), 3, "legacy reasoning scenario requires exactly three think calls");
        let prefixes = ["STEP_1_CLOSING_SPEED", "STEP_2_MEETING_TIME", "STEP_3_DISTANCE_CHECK"];
        let attempts = calls.iter().map(|e| e.string_field("attempt").unwrap()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(attempts.len(), 3, "each step must be a separate model response");
        let requests = events.iter().filter(|event| event.kind() == &EventKind::Attempt).collect::<Vec<_>>();
        assert_eq!(requests.len(), 4);
        for (index, attempt) in requests.iter().enumerate() {
            let id = attempt.string_field("attempt").unwrap();
            let body: Value = serde_json::from_slice(&fs::read(artifact.join(format!("{id}.request.json"))).unwrap()).unwrap();
            let choice = body.get("tool_choice").and_then(|v| v.as_str().or_else(|| v.get("type").and_then(Value::as_str)))
                .or_else(|| body.pointer("/toolConfig/functionCallingConfig/mode").and_then(Value::as_str)).unwrap_or("");
            if selected_provider.dialect == "deepseek_responses_v1" {
                assert!(body.get("tool_choice").is_none(), "legacy DeepSeek Responses omits choice");
                assert_eq!(body["reasoning"]["effort"], "high");
            } else {
                assert!(if index < 3 { ["required", "any", "ANY"].contains(&choice) } else { ["none", "NONE"].contains(&choice) }, "wrong per-response choice: {body}");
            }
            if index < 3 { assert!(body.to_string().contains(&format!("beginning exactly {}", prefixes[index]))); }
        }

        for (index, call) in calls.iter().enumerate() {
            let raw = serde_json::to_value(call.raw()).unwrap();
            let args = materialize_json(&ledger, &raw["args"]).unwrap();
            assert!(args["thought"].as_str().is_some_and(|text| text.starts_with(prefixes[index])), "wrong step ordering: {args}");
            let result = events.iter().find(|event| event.kind() == &EventKind::ToolResult
                && event.string_field("call") == call.string_field("call")).unwrap();
            assert_eq!(result.string_field("outcome"), Some("ok"));
            assert!(call.seq() < result.seq());
            if let Some(next) = calls.get(index + 1) { assert!(result.seq() < next.seq()); }
            assert!(result.seq() < promoted.unwrap());
        }
    }
    if prompt.contains("repeated identical read test") || prompt.contains("repeated context_get test") {
        let name = if prompt.contains("repeated context_get test") { "context_get" } else { "read" };
        let repeated = events.iter().filter(|event| event.kind() == &EventKind::ToolCall
            && event.string_field("name") == Some(name)).collect::<Vec<_>>();
        assert_eq!(repeated.len(), 3, "model must finish after exactly three reads");
        let arguments = repeated.iter().map(|event| {
            let raw = serde_json::to_value(event.raw()).unwrap();
            materialize_json(&ledger, &raw["args"]).unwrap()
        }).collect::<Vec<_>>();
        assert!(arguments.windows(2).all(|pair| pair[0] == pair[1]), "repeated reads must have identical arguments");
        let distinct = repeated.iter().map(|event| event.string_field("attempt").unwrap()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(distinct.len(), 3, "reads must execute in three separate responses");
        for (index, call) in repeated.iter().enumerate() {
            let result = events.iter().find(|event| event.kind() == &EventKind::ToolResult
                && event.string_field("call") == call.string_field("call")).expect("every repeated call must execute");
            assert_eq!(result.string_field("outcome"), Some("ok"));
            if name == "context_get" {
                let actual = validation_runtime::result_value(&ledger, result).unwrap();
                let genesis = serde_json::to_value(events[0].raw()).unwrap();
                assert_eq!(actual["thread_id"], genesis["thread"]);
                assert_eq!(actual["records"].as_array().unwrap().len(), 1);
                assert_eq!(actual["records"][0]["record_id"], 1);
                assert_eq!(actual["records"][0]["event"], genesis);
                assert!(actual["cursor"].is_null());
            }
            assert!(result.seq() > call.seq());
            if let Some(next) = repeated.get(index + 1) { assert!(result.seq() < next.seq()); }
            assert!(result.seq() < promoted.unwrap());
        }
        assert_eq!(events.iter().filter(|event| event.kind() == &EventKind::Settle).count(), 1);
    }
    // Replay every accepted ledger prefix through the real endpoint projector.
    // This proves event ownership/order, independently of provider finality.
    let journal = endpoint::EndpointJournal::open(&runtime_folder).unwrap();
    let mut projector = endpoint::Projector::default();
    let mut wire_events = Vec::new();
    for event in events {
        let projected = projector.reconcile(std::slice::from_ref(event), &journal).unwrap();
        for wire in &projected {
            if wire.event_type == "turn/end" {
                assert_eq!(event.kind(), &EventKind::Settle, "only writer settlement can publish endpoint completion");
                assert_eq!(event.string_field("outcome"), Some("completed"));
                let data = serde_json::to_value(&wire.data).unwrap();
                assert_eq!(data["validationOutcome"], validation["outcome"]);
                assert!(data["promotedMessageID"].as_str().is_some());
            }
        }
        wire_events.extend(projected);
    }
    let completed_turns = wire_events.iter().filter(|event| event.event_type == "turn/end").count();
    if prompt.contains("goal continuation test") {
        assert!((2..=session_controls::MAX_GOAL_ROUNDS as usize).contains(&completed_turns));
    } else {
        assert_eq!(completed_turns, 1);
    }
    std::fs::write(artifact.join("endpoint-events.json"), serde_json::to_vec_pretty(&wire_events).unwrap()).unwrap();
    let journal_path = runtime_folder.join("endpoint.jsonl");
    let before = std::fs::read(&journal_path).unwrap();
    let reopened = endpoint::EndpointJournal::open(&runtime_folder).unwrap();
    let recovered = endpoint::Projector::default().reconcile(events, &reopened).unwrap();
    assert_eq!(serde_json::to_value(&recovered).unwrap(), serde_json::to_value(&wire_events).unwrap(),
        "endpoint recovery changed event identities");
    assert_eq!(std::fs::read(journal_path).unwrap(), before, "endpoint recovery appended duplicate records");
    if prompt.contains("parallel tool batch test") {
        let mut batches = std::collections::BTreeMap::<String, Vec<String>>::new();
        for event in events.iter().filter(|e| e.kind() == &EventKind::ToolCall) {
            batches.entry(event.string_field("attempt").unwrap().to_owned()).or_default()
                .push(event.string_field("call").unwrap().to_owned());
        }
        assert!(batches.values().any(|calls| calls.len() >= 2 && calls.iter().all(|call|
            events.iter().any(|e| e.kind() == &EventKind::ToolResult && e.string_field("call") == Some(call)
                && e.string_field("outcome") == Some("ok")))),
            "no single response produced two successfully executed tool calls");
    }
    if prompt.contains("WRONG_FINAL") {
        let candidates = events.iter().filter(|e| e.string_field("subkind") == Some("validation.candidate"))
            .map(|e| serde_json::to_value(e.raw()).unwrap()).collect::<Vec<_>>();
        assert_eq!(candidates.len(), 2, "one wrong-answer candidate and one repaired candidate required");
        assert_eq!(candidates[0]["payload"]["binding"]["snapshot"], candidates[1]["payload"]["binding"]["snapshot"], "repair must preserve already correct file snapshot");
        let first_seq = candidates[0]["payload"]["binding"]["output_seq"].as_u64().unwrap();
        let first = answers.iter().find(|e| e["seq"].as_u64() == Some(first_seq)).unwrap();
        let first_text = first["content"].as_array().unwrap().iter().filter_map(|b| b["text"].as_str()).collect::<String>();
        assert!(first_text.contains("WRONG_FINAL") && first_text.trim() != "KERNEL_FILE_OK");
        let feedback = events.iter().find(|e| e.string_field("subkind") == Some("validation.feedback")).unwrap();
        assert!(first_seq < feedback.seq() && feedback.seq() < promoted.unwrap());
    }
    if prompt.contains("validation repair test") {
        assert!(events.iter().any(|e| e.string_field("subkind") == Some("validation.feedback")), "no durable repair feedback");
        assert_eq!(events.iter().filter(|e| e.kind() == &EventKind::Settle).count(), 1, "repair must not start a second turn");
        assert!(events.iter().filter(|e| e.kind() == &EventKind::Spawn).count() >= 2, "changed snapshot requires another validator");
        // Legacy simpleWriteRepair: one or two bounded repair rounds, the
        // initial validator plus one per round, and both writes in the same worker.
        let feedback = events.iter().filter(|e| e.string_field("subkind") == Some("validation.feedback")).count();
        assert!((1..=2).contains(&feedback), "bounded repair rounds, observed {feedback}");
        let validators = events.iter().filter(|e| e.kind() == &EventKind::Spawn).count();
        assert_eq!(validators, feedback + 1, "initial validator plus one per repair round");
        let writes = events.iter().filter(|e| e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("apply_patch")).count();
        assert!(writes >= 2, "the same worker must perform the draft and repair writes, observed {writes}");
    }
    for tool in required_tools.split(',').filter(|name| !name.is_empty()) {
        assert!(calls.contains(&tool), "required tool was not executed: {tool}");
        let ids = events.iter().filter(|event| *event.kind() == schema::EventKind::ToolCall && event.string_field("name") == Some(tool)).filter_map(|event| event.string_field("call")).collect::<Vec<_>>();
        assert!(events.iter().any(|event| *event.kind() == schema::EventKind::ToolResult && event.string_field("outcome") == Some("ok") && event.string_field("call").is_some_and(|id| ids.contains(&id))), "required tool has no successful result: {tool}");
    }
    if prompt.contains("shell artifact version test") {
        let shell_ids = events.iter().filter(|e| e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("shell"))
            .filter_map(|e| e.string_field("call")).collect::<Vec<_>>();
        assert!(events.iter().filter(|e| e.kind() == &EventKind::ToolResult && e.string_field("call").is_some_and(|id| shell_ids.contains(&id)))
            .any(|e| validation_runtime::result_value(&ledger,e).is_ok_and(|value|
                value["artifacts"].as_array().is_some_and(|artifacts| artifacts.iter().any(|a| a["artifact_version"].as_u64().is_some())))),
            "shell artifact lacks durable version");
    }
    if selected_tools.contains(&"apply_patch") || prompt.contains("shell artifact version test") {
        assert_eq!(validation["outcome"], "pass", "artifact live test requires an actual passing validator");
        let legacy = prompt.contains("docs/live-smoke.md");
        let relative = if legacy { "docs/live-smoke.md" } else { "proof.txt" };
        let file = std::fs::read_to_string(workspace.path().join(relative)).expect("saved file");
        let destination = artifact.join(relative);
        std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
        std::fs::write(destination, &file).unwrap();
        if legacy {
            assert_eq!(file, "LIVE_OK");
            assert_eq!(events.iter().filter(|e| e.kind() == &EventKind::Spawn).count(), 1, "legacy smoke requires one validator");
            assert_eq!(events.iter().filter(|e| e.kind() == &EventKind::TurnOpen).count(), 1, "one root worker turn");
            assert!(events.iter().any(|e| e.kind() == &EventKind::Output && e.usage().and_then(|usage| usage.get("availability")).and_then(Value::as_str) == Some("reported")), "usage must be reported");
            if selected_provider.dialect == "deepseek_chat_v1" {
                let mut samples = 0;
                for entry in std::fs::read_dir(&artifact).unwrap() {
                    let path = entry.unwrap().path();
                    if path.extension().and_then(|s| s.to_str()) != Some("jsonl") || path.file_name().and_then(|s|s.to_str()) == Some("control.jsonl") { continue; }
                    for line in std::fs::read_to_string(path).unwrap().lines() {
                        let event: Value = serde_json::from_str(line).unwrap();
                        if event["kind"] != "output" && event["kind"] != "error" || event.get("usage").is_none() { continue; }
                        samples += 1;
                        assert_eq!(event["usage"]["availability"], "reported");
                        for field in ["cache_read", "cache_miss"] {
                            assert!(event["usage"][field].as_str().is_some_and(|n|n.parse::<u64>().is_ok()), "DeepSeek must durably retain the explicit cache pair: {event}");
                        }
                    }
                }
                assert!(samples >= attempts, "root and validator usage must be retained");
            }
        } else {
            assert!(file.contains("KERNEL_FILE_OK"), "saved artifact differs from requested content");
        }
    }
    if prompt.contains("queued validation test") {
        let first_settle = events.last().unwrap().seq();
        let queued = events.iter().find(|event| event.kind() == &EventKind::Input
            && event.string_field("origin_key") == Some("queued-second")).expect("second input was queued while busy").seq();
        assert!(queued < first_settle);
        let first_count = events.len();
        for attempt in events.iter().filter(|event| event.kind() == &EventKind::Attempt) {
            let name = attempt.string_field("attempt").unwrap();
            let request = std::fs::read_to_string(artifact.join(format!("{name}.request.json"))).unwrap();
            assert!(!request.contains("KERNEL_QUEUE_OK"), "queued body leaked into the first worker turn");
        }
        assert_eq!(open_ready_turn(&mut ledger, &options.timestamp, engine::RunMode::Ordinary).unwrap(), Some(2));
        let opened = ledger.projection().unwrap().events.last().unwrap().seq();
        assert!(opened > first_settle);
        assert_eq!(current_turn_inputs(&ledger, 2).unwrap(), vec![queued]);
        let second = run_provider_turn(&mut ledger, &options, &profile, &Selected {version:2},
            &mut credential, &mut replies, &mut output, &cancellation);
        std::fs::copy(&path, artifact.join("main.jsonl")).unwrap();
        std::fs::write(artifact.join("control.jsonl"), &*output.0.borrow()).unwrap();
        second.expect("queued second provider turn");
        let events = &ledger.projection().unwrap().events;
        let terminal = serde_json::to_value(events.last().unwrap().raw()).unwrap();
        assert_eq!(terminal["kind"], "settle");
        assert_eq!(terminal["turn"], 2);
        assert_eq!(terminal["outcome"], "completed");
        let promoted = terminal["validation"]["promoted_output_seq"].as_u64().unwrap();
        let answer = events.iter().find(|event| event.seq() == promoted).unwrap();
        let raw = serde_json::to_value(answer.raw()).unwrap();
        let text = raw["content"].as_array().unwrap().iter().filter_map(|block| block["text"].as_str()).collect::<String>();
        assert_eq!(text.trim(), "KERNEL_QUEUE_OK");
        assert_eq!(events.iter().filter(|event| event.kind() == &EventKind::Settle).count(), 2);
        let second_wire = projector.reconcile(&events[first_count..], &journal).unwrap();
        assert_eq!(second_wire.iter().filter(|event| event.event_type == "turn/end").count(), 1);
        std::fs::write(artifact.join("queued-endpoint-events.json"), serde_json::to_vec_pretty(&second_wire).unwrap()).unwrap();
        std::fs::write(artifact.join("queue-result.json"), serde_json::to_vec_pretty(&json!({
            "queued_input_seq":queued,"first_settle_seq":first_settle,"second_open_seq":opened,
            "second_settle_seq":terminal["seq"],"second_answer":text,"completed":true})).unwrap()).unwrap();
    }
    output.1 = None; replies.1 = None;
    drop(credential); drop(control); service.join().unwrap().unwrap();
}


struct UnavailableLiveEffects;
impl tekes_supervisor::production_tool_control::SupervisorRuntimeAuthority for UnavailableLiveEffects {
    fn ensure_running(&mut self, _: &str, _: &str) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
    fn deliver_input(&mut self, _: &tekes_supervisor::production_tool_control::DeliveryRequest) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
    fn interrupt(&mut self, _: &tekes_supervisor::production_tool_control::InterruptRequest) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
    fn ensure_child(&mut self, _: &tekes_supervisor::production_tool_control::ChildLaunchProof) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
    fn deliver_report(&mut self, _: &tekes_supervisor::production_tool_control::ParentReportProof, _: &IJsonValue) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
}
impl tekes_supervisor::production_tool_control::SupervisorJobAuthority for UnavailableLiveEffects {
    fn execute(&mut self, _: &worker_control::ToolControl) -> Result<IJsonValue, tekes_supervisor::production_tool_control::SupervisorOperationError> { Err(live_effect_unavailable()) }
}
fn live_effect_unavailable() -> tekes_supervisor::production_tool_control::SupervisorOperationError {
    tekes_supervisor::production_tool_control::SupervisorOperationError::Unavailable("this live harness has no process mutation authority".into())
}

// Test-only equivalent of legacy ContinuationThink.choose; no production
// policy is inferred from prompt text.
pub(super) fn prepare_live_choice(
    ledger: &LockedLedger,
    turn: u64,
    mut input: PrepareInput,
) -> Result<provider::PreparedRequest, provider::PrepareError> {
    if std::env::var("TEKES_KERNEL_LIVE_PROMPT").ok().is_none_or(|p| !p.contains("STEP_1_CLOSING_SPEED")) {
        return prepare(&input);
    }
    let prefixes = ["STEP_1_CLOSING_SPEED", "STEP_2_MEETING_TIME", "STEP_3_DISTANCE_CHECK"];
    let events = &ledger.projection().unwrap().events;
    let mut accepted = 0;
    for result in events.iter().filter(|event| event.turn() == Some(turn)
        && event.kind() == &EventKind::ToolResult && event.string_field("outcome") == Some("ok")) {
        let Some(call) = events.iter().find(|call| call.kind() == &EventKind::ToolCall
            && call.string_field("name") == Some("think") && call.string_field("call") == result.string_field("call")) else { continue; };
        if accepted == prefixes.len() { break; }
        let raw = serde_json::to_value(call.raw()).unwrap();
        let args = materialize_json(ledger, &raw["args"]).unwrap();
        if !args["thought"].as_str().is_some_and(|thought| thought.starts_with(prefixes[accepted])) { break; }
        accepted += 1;
    }
    let choice = if accepted == prefixes.len() { provider::ToolChoice::None } else {
        input.rendered_items.push(IJsonValue::parse(&serde_json::to_vec(&json!({"role":"user","content":[{"type":"text","text":format!("This request must call think with a new thought beginning exactly {}. Do not repeat any earlier thought.", prefixes[accepted])}]})).unwrap()).unwrap());
        provider::ToolChoice::Required
    };
    // Pinned DeepSeek.Response.responsesWireProfile always sets toolChoice:nil;
    // its high-reasoning replay contract must not inherit OpenAI forced choice.
    if input.target.dialect_id == "deepseek_responses_v1" {
        prepare(&input)
    } else {
        provider::prepare_with_tool_choice(&input, Some(choice))
    }
}
