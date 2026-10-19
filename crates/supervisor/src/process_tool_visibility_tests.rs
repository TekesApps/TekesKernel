// Exercise the real process runtime/host/endpoint seam, including ledger ownership.
#[cfg(target_os = "macos")]
mod process_tool_visibility_tests {
    use super::*;
    use crate::production_tool_control::SupervisorRuntimeAuthority;

    const RECEIVER: &str = "018f0000-0000-7000-8000-000000000241";
    const SENDER: &str = "018f0000-0000-7000-8000-000000000242";
    // The fixture implements the worker's ledger ownership and acknowledgement
    // protocol, not a mock runtime: it locks, validates exact operations, appends
    // and fsyncs before acknowledging, and deduplicates from durable origins.
    const WORKER: &str = r##"#!/usr/bin/python3
import fcntl,json,os,sys
print(json.dumps({'hello':{'proto':'tekes-worker','min':2,'max':2}}),flush=True)
json.loads(sys.stdin.readline())
f=open(sys.argv[1],'r+')
fcntl.flock(f,fcntl.LOCK_EX)
with open(sys.argv[1]+'.worker-ready','w') as ready: ready.write(str(os.getpid()))
for line in sys.stdin:
    frame=json.loads(line)
    if 'input' in frame:
        item=frame['input']; origin=item['origin']
        assert origin['op']=='session.prompt',origin
        event={'kind':'input','content':item['content'],'submission':item['submission']}
    elif 'stop' in frame:
        item=frame['stop']; origin=item['origin']
        assert origin['op']=='session.cancel',origin
        event={'kind':'stop_requested','generation':item['generation']}
    else:
        continue
    f.seek(0); rows=[json.loads(row) for row in f if row.strip()]
    old=next((row for row in rows if row.get('origin_tuple')==origin),None)
    if old is None:
        event.update(v=1,seq=rows[-1]['seq']+1,ts='2026-10-15T00:00:00.000Z',origin_key=origin['key'],origin_tuple=origin)
        f.seek(0,2); f.write(json.dumps(event,sort_keys=True,separators=(',',':'))+'\n'); f.flush(); os.fsync(f.fileno())
    print(json.dumps({'receipt':{'delivery':item['delivery'],'seq':old['seq'] if old else event['seq'],'deduplicated':old is not None}}),flush=True)
"##;

    fn receiver_test(test: impl Fn(&Arc<ProductionProcessHost>, &Path, &mut ProcessToolRuntime)) {
        for mode in engine::PermissionMode::ALL {
            let root = tempfile::tempdir().unwrap();
            let project_root = tempfile::tempdir().unwrap();
            let project = project_root.path().to_path_buf();
            fs::create_dir_all(&project).unwrap();
            write_test_execution_config(
                root.path(),
                json!({
                    "format":1,"revision":1,"id":"ws","name":"receiver","cwd":[project],
                    "policy":{"allowed_tools":[],"writable_roots":[],"network":false}
                }),
            );
            let bin = root.path().join("fake-worker");
            fs::write(&bin, WORKER).unwrap();
            fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap();
            let helper = root.path().join("tekes-helper");
            fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap();
            fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
            let folder = root.path().join("threads").join(RECEIVER);
            fs::create_dir_all(folder.join("assets")).unwrap();
            let ledger = folder.join("main.jsonl");
            write_test_genesis(&ledger, RECEIVER);
            let sender_folder = root.path().join("threads").join(SENDER);
            fs::create_dir_all(sender_folder.join("assets")).unwrap();
            write_test_genesis(&sender_folder.join("main.jsonl"), SENDER);
            let genesis = fs::read(&ledger).unwrap();
            engine::write_permission_mode(&folder, mode).unwrap();
            let mode_before = fs::read(engine::permission_mode_path(&folder)).unwrap();
            let host = ProductionProcessHost::open(
                root.path(),
                &bin,
                "test",
                root.path().join("missing-agent-home"),
            )
            .unwrap();
            // An unrelated worker occupies the one configured slot. Cold input
            // is durably accepted/queued without starting the receiver yet.
            let blocker = host
                .spawn_worker_at(SENDER, SENDER, sender_folder.join("main.jsonl"), None)
                .unwrap();
            host.workers.lock().unwrap().insert(SENDER.into(), blocker);
            let mut runtime = ProcessToolRuntime {
                host: Arc::downgrade(&host),
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                test(&host, &ledger, &mut runtime)
            }));
            host.shutdown();
            assert_eq!(
                fs::read(engine::permission_mode_path(&folder)).unwrap(),
                mode_before
            );
            let after = fs::read(&ledger).unwrap();
            assert!(
                after.starts_with(&genesis),
                "message/control cannot rewrite frozen genesis"
            );
            schema::validate_ledger(&after, schema::READER_VERSION).unwrap();
            if let Err(error) = result {
                std::panic::resume_unwind(error);
            }
        }
    }

    fn live_receiver(host: &Arc<ProductionProcessHost>, ledger: &Path) -> Arc<WorkerHandle> {
        // This test takes over the queued receiver's launch. Remove its pending
        // entry before freeing capacity, otherwise the exit reconciliation can
        // race a second fixture against this explicit live-phase launch.
        let pending_deadline = Instant::now() + Duration::from_secs(5);
        while !host.pending_workers.lock().unwrap().contains_key(RECEIVER) {
            assert!(Instant::now() < pending_deadline, "durable input was not scheduled");
            std::thread::sleep(Duration::from_millis(5));
        }
        host.pending_workers.lock().unwrap().remove(RECEIVER);
        let blocker = host.workers.lock().unwrap().remove(SENDER).unwrap();
        terminate_worker(&blocker);
        let worker = host
            .spawn_worker_at(RECEIVER, RECEIVER, ledger.to_owned(), None)
            .unwrap();
        host.workers
            .lock()
            .unwrap()
            .insert(RECEIVER.into(), Arc::clone(&worker));
        let ready = PathBuf::from(format!("{}.worker-ready", ledger.display()));
        let deadline = Instant::now() + Duration::from_secs(5);
        let expected_pid = worker.child.lock().unwrap().id();
        while fs::read_to_string(&ready)
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            != Some(expected_pid)
        {
            assert!(
                Instant::now() < deadline,
                "fixture did not acquire the line lock"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(
            matches!(
                LockedLedger::open(ledger, schema::READER_VERSION),
                Err(store::StoreError::Busy)
            ),
            "the hot fixture must own the same ledger lock as a worker"
        );
        worker
    }

    fn plain(value: IJsonValue) -> Value {
        serde_json::to_value(value).unwrap()
    }

    #[test]
    fn process_tool_runtime_send_uses_receiver_authority_cold_and_hot() {
        receiver_test(|host, ledger, runtime| {
            let mut request = DeliveryRequest {
                source_session: SENDER.into(),
                source_line: SENDER.into(),
                target_session: RECEIVER.into(),
                request_id: "context-cold-send".into(),
                message: "normal message".into(),
                goal: false,
            };
            let cold = plain(runtime.deliver_input(&request).unwrap());
            assert_eq!(cold["deduplicated"], false);
            assert!(host.live_worker(RECEIVER).is_none());
            let repeat = plain(runtime.deliver_input(&request).unwrap());
            assert_eq!(repeat["seq"], cold["seq"]);
            assert_eq!(repeat["deduplicated"], true);
            let worker = live_receiver(host, ledger);
            let snapshot = serde_json::to_vec(&worker.config_snapshot).unwrap();
            request.request_id = "context-hot-send".into();
            let hot = plain(runtime.deliver_input(&request).unwrap());
            assert_eq!(hot["deduplicated"], false);
            let repeat = plain(runtime.deliver_input(&request).unwrap());
            assert_eq!(repeat["seq"], hot["seq"]);
            assert_eq!(repeat["deduplicated"], true);
            assert!(
                Arc::ptr_eq(&worker, &host.live_worker(RECEIVER).unwrap()),
                "normal delivery does not replace the receiving worker"
            );
            assert_eq!(
                serde_json::to_vec(&worker.config_snapshot).unwrap(),
                snapshot
            );
            let rows = read_root_events(ledger);
            assert_eq!(rows.iter().filter(|row| row["kind"] == "input").count(), 2);
            for row in rows.iter().filter(|row| row["kind"] == "input") {
                assert_eq!(row["origin_tuple"]["op"], "session.prompt");
                assert_eq!(row["content"][0]["text"], request.message);
            }
        });
    }

    #[test]
    fn process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot() {
        receiver_test(|host, ledger, runtime| {
            let mut request = InterruptRequest {
                source_session: SENDER.into(),
                source_line: SENDER.into(),
                target_session: RECEIVER.into(),
                request_id: "context-cold-stop".into(),
            };
            let cold = plain(runtime.interrupt(&request).unwrap());
            assert_eq!(cold["deduplicated"], false);
            assert!(host.live_worker(RECEIVER).is_none());
            let repeat = plain(runtime.interrupt(&request).unwrap());
            assert_eq!(repeat["seq"], cold["seq"]);
            assert_eq!(repeat["deduplicated"], true);
            let _worker = live_receiver(host, ledger);
            request.request_id = "context-hot-stop".into();
            let hot = plain(runtime.interrupt(&request).unwrap());
            assert_eq!(hot["deduplicated"], false);
            // cancel terminates the hot worker. Replaying on the cold path must
            // find the worker's same durable origin and return the same receipt.
            let repeat = plain(runtime.interrupt(&request).unwrap());
            assert_eq!(repeat["seq"], hot["seq"]);
            assert_eq!(repeat["deduplicated"], true);
            let rows = read_root_events(ledger);
            assert_eq!(
                rows.iter()
                    .filter(|row| row["kind"] == "stop_requested")
                    .count(),
                2
            );
            assert!(
                rows.iter()
                    .filter(|row| row["kind"] == "stop_requested")
                    .all(|row| row["origin_tuple"]["op"] == "session.cancel")
            );
        });
    }

    #[test]
    fn legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt() {
        use crate::production_tool_control::{
            ProductionToolControlHandler, ProductionToolControlPolicy, SupervisorJobAuthority,
        };
        use crate::tool_control::{ToolControlReceiptStore, ToolControlSession};
        use worker_control::{ToolControl, decode_tool_control_result, encode_tool_control};
        struct NoJobs;
        impl SupervisorJobAuthority for NoJobs {
            fn execute(&mut self, _: &ToolControl) -> Result<IJsonValue, SupervisorOperationError> {
                panic!("not a job")
            }
        }
        const ACTOR: &str = "018f0000-0000-7000-8000-000000000243";
        for difference in ["exact", "principal", "client", "target", "kind"] {
            receiver_test(|host, ledger, runtime| {
                let args = json!({"operation":"interrupt","thread_id":RECEIVER,"reason":"recover interrupted delivery"});
                let source = host.root.join("threads").join(ACTOR);
                fs::create_dir_all(source.join("assets")).unwrap();
                let mut rows = held_ledger_lines(ACTOR)
                    .into_iter()
                    .map(|line| serde_json::from_str::<Value>(&line).unwrap())
                    .collect::<Vec<_>>();
                rows[6]["content"] =
                    json!([{"type":"tool-call","call":"ask1","name":"context","args":args}]);
                rows[7]["name"] = json!("context");
                rows[7]["args"] = args.clone();
                rows[8]["scope"] = json!("destructive");
                let mut response: Value = serde_json::from_str(&held_response_line(ACTOR)).unwrap();
                response["scope"] = json!("destructive");
                rows.push(response);
                write_test_events(&source.join("main.jsonl"), rows);
                let request = ToolControl::new(
                    ACTOR,
                    ACTOR,
                    1,
                    "ask1",
                    "context",
                    IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap(),
                )
                .unwrap();
                let receipt =
                    ToolControlReceiptStore::new(&source, ACTOR).record_path(&request.request_id);
                assert!(
                    !receipt.exists(),
                    "simulate target stop committed before the source control receipt"
                );
                let mut origin = json!({"principal":"kernel-worker","client":"context","target":RECEIVER,"op":"cancel","key":request.request_id});
                if difference == "principal" {
                    origin["principal"] = json!("another-principal");
                }
                if difference == "client" {
                    origin["client"] = json!("another-client");
                }
                if difference == "target" {
                    origin["target"] = json!(SENDER);
                }
                let mut fact = json!({"v":1,"seq":2,"kind":"stop_requested","ts":"2026-10-15T00:00:00.000Z","generation":1,"origin_key":request.request_id,"origin_tuple":origin});
                if difference == "kind" {
                    fact["kind"] = json!("input");
                    fact.as_object_mut().unwrap().remove("generation");
                    fact["content"] =
                        json!([{"type":"text","text":"same tuple on a different fact"}]);
                }
                let mut bytes = fs::read(ledger).unwrap();
                bytes.extend(
                    Event::decode(&serde_json::to_vec(&fact).unwrap())
                        .unwrap()
                        .canonical_bytes()
                        .unwrap(),
                );
                bytes.push(b'\n');
                schema::validate_ledger(&bytes, schema::READER_VERSION).unwrap();
                // The authoritative complete prefix survives a torn suffix. A
                // receipt replay neither repairs it nor inspects later bytes.
                if difference == "exact" {
                    bytes.extend(b"{\"torn\":");
                }
                fs::write(ledger, &bytes).unwrap();
                let make_session = |runtime| {
                    ToolControlSession::new(
                        &host.root,
                        worker_control::Selected { version: 2 },
                        ProductionToolControlHandler::new(&host.root, runtime, NoJobs),
                        ProductionToolControlPolicy::default(),
                    )
                };
                let mut session = make_session(runtime.clone());
                let line = encode_tool_control(&request).unwrap();
                let first = session.handle_line(&line).unwrap();
                let result = decode_tool_control_result(&first).unwrap();
                assert!(result.error.is_none(), "{difference}: {:?}", result.error);
                let value = plain(result.value.unwrap());
                assert_eq!(value["deduplicated"], difference == "exact");
                if difference == "exact" {
                    assert_eq!(value["seq"], 2);
                    assert_eq!(
                        fs::read(ledger).unwrap(),
                        bytes,
                        "legacy match emits no second stop or control"
                    );
                    // Restore only the test's deliberately torn suffix before
                    // receiver_test validates the final complete ledger.
                    bytes.truncate(bytes.iter().rposition(|byte| *byte == b'\n').unwrap() + 1);
                    fs::write(ledger, &bytes).unwrap();
                } else {
                    assert_eq!(
                        value["seq"], 3,
                        "nearby but distinct facts cannot replay the legacy stop"
                    );
                    let rows = read_root_events(ledger);
                    let stop = rows.last().unwrap();
                    assert_eq!(stop["kind"], "stop_requested");
                    assert_eq!(stop["origin_tuple"]["op"], "session.cancel");
                    assert_eq!(stop["origin_tuple"]["target"], RECEIVER);
                }
                let before_replay = fs::read(ledger).unwrap();
                let mut restarted = make_session(ProcessToolRuntime { host: Weak::new() });
                assert_eq!(
                    restarted.handle_line(&line).unwrap(),
                    first,
                    "source receipt wins without any available runtime authority"
                );
                assert_eq!(fs::read(ledger).unwrap(), before_replay);
            });
        }
    }

    fn read_root_events(ledger: &Path) -> Vec<Value> {
        fs::read_to_string(ledger)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }
}
