use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::atomic::{AtomicUsize, Ordering};

use engine::{
    AdapterCapabilities, AdmissionLease, AdmissionPool, AttemptFlow, AttemptPhase, Continuation,
    CreateIndex, CreateResolution, DeliveryCommit, DeliveryIndex, DeliveryResolution, DrainAction,
    DrainTracker, EnsureAction, FakeProcessHost, FakeProvider, FakeToolBackend, LeaseRelease,
    LockFacts, OutcomeCommit, ProcessHost, ProviderAdapter, ProviderTerminal, QueryCapability,
    QueryResult, RECOVERY_ORDER, RecoveryDecision, RecoveryStage, RunDecision, RunMode, StopTree,
    TailState, TransactionError, classify, decide_recovery, ensure_action, ensure_action_at,
    run_decision, sent_state,
};
use schema::{
    Event, EventKind, LifecycleFacts, OriginTuple, ResumePolicy, SchemaError, Visibility,
    validate_ledger,
};
use store::{
    BarrierContext, LockedLedger, RootLock, StoreError, SyncPolicy, ThreadStore, requires_barrier,
    scan_valid_prefix,
};
use test_support::{FixtureRoot, read};
use worker_control::{
    PROTOCOL_VERSION, ProtocolError, SupervisorMessage, WorkerMessage, decode_hello, decode_reject,
    decode_selected, decode_supervisor, decode_worker, negotiate,
};

#[test]
fn slice1_gate_28_canonical_envelope_fixtures() {
    let root = FixtureRoot::discover().expect("fixture root");
    let manifest = root.verify_manifest().expect("manifest and assets");
    assert_eq!(manifest.version, 1);

    let event_cases = manifest.corpora.get("events").expect("events corpus");
    assert_eq!(event_cases.len(), 85);
    for case in event_cases {
        let bytes = read(&root.join("events").join(case)).expect("event fixture");
        assert!(bytes.ends_with(b"\n"), "{case} must end in one LF");
        assert!(
            !bytes[..bytes.len() - 1].ends_with(b"\n"),
            "{case} has an extra LF"
        );
        let event = Event::decode(&bytes[..bytes.len() - 1]).unwrap_or_else(|error| {
            panic!("{case} must decode: {error}");
        });
        assert_eq!(
            event.canonical_bytes().expect("canonical encode"),
            bytes[..bytes.len() - 1],
            "{case}"
        );
    }
    assert!(
        schema::IJsonValue::parse(br#"{"a":1,"a":2}"#).is_err(),
        "I-JSON/JCS rejects duplicate member names"
    );
    assert!(
        Event::decode(
            br#"{"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"k","origin_tuple":{"client":"c","key":"k","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},"resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-13-40T25:61:61.000Z","v":1,"workspace":"ws"}"#,
        )
        .is_err(),
        "timestamp must be a real UTC calendar instant"
    );
    let noncanonical = b"{ \"v\":1, \"seq\":1, \"kind\":\"genesis\", \"ts\":\"2026-08-26T09:00:00.000Z\", \"format\":1, \"thread\":\"018f0000-0000-7000-8000-000000000001\", \"workspace\":\"ws\", \"min_reader\":1, \"min_writer\":1, \"origin_key\":\"k\", \"origin_tuple\":{\"principal\":\"p\",\"client\":\"c\",\"target\":\"018f0000-0000-7000-8000-000000000001\",\"op\":\"create\",\"key\":\"k\"}, \"resume\":\"never\" }\n";
    assert!(
        validate_ledger(noncanonical, 1).is_err(),
        "durable JSONL requires canonical event bytes"
    );

    for case in [
        "_valid-baseline.jsonl",
        "../replay/checkpoint-valid.jsonl",
        "../replay/checkpoint-invalidated.jsonl",
        "../replay/cancel-before-start.jsonl",
    ] {
        let path = if case.starts_with("../") {
            root.join(case.trim_start_matches("../"))
        } else {
            root.join("invalid").join(case)
        };
        let bytes = read(&path).expect("valid ledger fixture");
        validate_ledger(&bytes, 1).unwrap_or_else(|error| {
            panic!("{} must validate: {error}", path.display());
        });
    }

    let expected_rules: BTreeMap<&str, Option<u8>> = BTreeMap::from([
        ("admits-runtime.jsonl", Some(4)),
        ("admits-unconsumed-input.jsonl", Some(4)),
        ("attempt-admits-future-input.jsonl", Some(7)),
        ("bad-ordinal.jsonl", Some(9)),
        ("bad-thread-uuid.jsonl", None),
        ("double-consume.jsonl", Some(8)),
        ("double-output.jsonl", Some(2)),
        ("double-settle.jsonl", Some(1)),
        ("envelope-missing-field.jsonl", None),
        ("epoch-pending-future.jsonl", Some(7)),
        ("forward-supersede.jsonl", Some(7)),
        ("null-schema-field.jsonl", None),
        ("origin-mismatch.jsonl", Some(6)),
        ("origin-missing-key.jsonl", Some(6)),
        ("queue-edit-consumed-target.jsonl", Some(8)),
        ("seq-gap.jsonl", None),
        ("skipped-input.jsonl", Some(8)),
        ("spill-undersized.jsonl", None),
        ("turn-after-settle.jsonl", Some(8)),
        ("turn-open-consumes-superseded.jsonl", Some(8)),
        ("turn-open-forward-input.jsonl", Some(7)),
        ("turn-open-held-as-maximum.jsonl", Some(8)),
        ("unpaired-result.jsonl", Some(3)),
    ]);
    let invalid_cases = manifest.corpora.get("invalid").expect("invalid corpus");
    for case in invalid_cases
        .iter()
        .filter(|case| case.ends_with(".jsonl") && !case.starts_with('_'))
    {
        let bytes = read(&root.join("invalid").join(case)).expect("invalid ledger fixture");
        let error = validate_ledger(&bytes, 1)
            .unwrap_err_or_else(|| panic!("{case} unexpectedly validated"));
        if let Some(expected_rule) = expected_rules.get(case.as_str()).copied().flatten() {
            assert_eq!(error.rule(), Some(expected_rule), "{case}: {error}");
        } else {
            assert!(
                matches!(
                    error,
                    SchemaError::Event { .. } | SchemaError::Json(_) | SchemaError::Canonical(_)
                ),
                "{case}: {error}"
            );
        }
    }
}

#[test]
fn slice1_gate_17_downgrade_evolution() {
    let ledger = concat!(
        "{\"config\":{\"digest\":\"cfg\"},\"format\":1,\"kind\":\"genesis\",\"min_reader\":1,\"min_writer\":1,\"origin_key\":\"create\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"create\",\"op\":\"create\",\"principal\":\"p\",\"target\":\"018f0000-0000-7000-8000-000000000001\"},\"resume\":\"never\",\"seq\":1,\"thread\":\"018f0000-0000-7000-8000-000000000001\",\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1,\"workspace\":\"ws\"}\n",
        "{\"content\":[{\"text\":\"hi\",\"type\":\"text\"}],\"kind\":\"input\",\"origin_key\":\"i1\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"i1\",\"op\":\"submit\",\"principal\":\"p\",\"target\":\"018f0000-0000-7000-8000-000000000001\"},\"seq\":2,\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1}\n",
        "{\"binary\":\"tekes-worker-2\",\"config_digest\":\"cfg\",\"instruction_digest\":\"ins\",\"kind\":\"run_start\",\"mode\":\"ordinary\",\"policy\":\"default\",\"recovery_ordinal\":0,\"run\":\"r1\",\"seq\":3,\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1}\n",
        "{\"kind\":\"turn_open\",\"seq\":4,\"trigger\":{\"inputs\":[2]},\"ts\":\"2026-08-26T09:00:00.000Z\",\"turn\":1,\"v\":1}\n",
        "{\"kind\":\"meta\",\"seq\":5,\"ts\":\"2026-08-26T09:00:00.000Z\",\"upgrade\":{\"min_reader\":2,\"min_writer\":2},\"v\":1}\n",
        "{\"kind\":\"state\",\"min_reader\":2,\"payload\":{\"future\":true},\"seq\":6,\"subkind\":\"future_state\",\"ts\":\"2026-08-26T09:00:00.000Z\",\"turn\":1,\"v\":1}\n",
        "{\"kind\":\"settle\",\"outcome\":\"completed\",\"seq\":7,\"ts\":\"2026-08-26T09:00:00.000Z\",\"turn\":1,\"v\":1}\n",
    );
    let projection =
        validate_ledger(ledger.as_bytes(), 1).expect("old reader preserves valid prefix");
    assert!(
        projection.read_only,
        "old reader must fail closed for writes/provider sends"
    );
    assert_eq!(projection.required_reader, 2);
    let state = &projection.events[5];
    assert!(matches!(state.kind(), EventKind::State));
    assert_eq!(state.effective_visibility(), Visibility::Model);
    assert_eq!(
        state.canonical_bytes().expect("generic state bytes"),
        ledger.lines().nth(5).expect("state line").as_bytes()
    );
}

#[test]
fn slice1_gate_31_protocol_negotiation_reject() {
    let root = FixtureRoot::discover().expect("fixture root");
    let ok = read(&root.join("wire/hello-ok.jsonl")).expect("hello-ok");
    let mut lines = ok.split_inclusive(|byte| *byte == b'\n');
    let hello = decode_hello(lines.next().expect("hello")).expect("hello decode");
    let selected = decode_selected(lines.next().expect("selected")).expect("selected decode");
    assert_eq!(
        negotiate(
            &hello,
            worker_control::PROTOCOL_VERSION,
            worker_control::PROTOCOL_VERSION
        )
        .expect("mutual version"),
        selected
    );
    assert_eq!(selected.version, PROTOCOL_VERSION);

    let disjoint = read(&root.join("wire/hello-disjoint.jsonl")).expect("hello-disjoint");
    let mut lines = disjoint.split_inclusive(|byte| *byte == b'\n');
    let hello = decode_hello(lines.next().expect("hello")).expect("hello decode");
    assert!(matches!(
        negotiate(
            &hello,
            worker_control::PROTOCOL_VERSION,
            worker_control::PROTOCOL_VERSION
        ),
        Err(ProtocolError::NoMutualVersion)
    ));
    assert_eq!(
        decode_reject(lines.next().expect("reject"))
            .expect("reject decode")
            .reason,
        "no_mutual_version"
    );

    assert!(matches!(
        decode_supervisor(br#"{"future_thing":{"x":1}}"#).expect("unknown ignored"),
        SupervisorMessage::Unknown { .. }
    ));
    assert!(matches!(
        decode_supervisor(br#"{"a":1,"b":2}"#),
        Err(ProtocolError::TopLevelKeyCount)
    ));

    for case in [
        "input.jsonl",
        "approval-response.jsonl",
        "approval-deny.jsonl",
        "compact.jsonl",
        "meta.jsonl",
        "queue-edit.jsonl",
        "stop.jsonl",
        "ping-pong.jsonl",
        "lease.jsonl",
        "lease-deny.jsonl",
        "launch-child.jsonl",
        "launch-mismatch.jsonl",
        "launch-result-error.jsonl",
        "receipt-dedup.jsonl",
        "frame.jsonl",
        "frame-tool.jsonl",
    ] {
        for line in read(&root.join("wire").join(case))
            .expect("wire fixture")
            .split_inclusive(|byte| *byte == b'\n')
        {
            let key = serde_json::from_slice::<serde_json::Value>(line)
                .expect("fixture JSON")
                .as_object()
                .expect("object")
                .keys()
                .next()
                .expect("key")
                .clone();
            if matches!(
                key.as_str(),
                "input"
                    | "approval_response"
                    | "compact"
                    | "meta"
                    | "queue_edit"
                    | "stop"
                    | "ping"
                    | "lease"
                    | "launch_result"
            ) {
                decode_supervisor(line)
                    .unwrap_or_else(|error| panic!("{case} supervisor message: {error}"));
            } else {
                decode_worker(line)
                    .unwrap_or_else(|error| panic!("{case} worker message: {error}"));
            }
        }
    }
    assert!(matches!(
        decode_worker(br#"{"future_thing":{"x":1}}"#).expect("unknown ignored"),
        WorkerMessage::Unknown { .. }
    ));
}

#[test]
fn slice1_gate_32_resume_policy_encoding() {
    let root = FixtureRoot::discover().expect("fixture root");
    for case in [
        "genesis.canonical.json",
        "genesis-bounded.canonical.json",
        "spawn.canonical.json",
        "spawn-bounded.canonical.json",
    ] {
        let bytes = read(&root.join("events").join(case)).expect("resume fixture");
        Event::decode(&bytes[..bytes.len() - 1]).unwrap_or_else(|error| panic!("{case}: {error}"));
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).expect("fixture JSON");
        value
            .as_object_mut()
            .expect("event object")
            .remove("resume");
        assert!(
            Event::decode(&serde_json::to_vec(&value).expect("JSON")).is_err(),
            "{case} without resume must reject"
        );
    }
    assert!(!ResumePolicy::Never.permits(0));
    assert!(ResumePolicy::Bounded(2).permits(0));
    assert!(ResumePolicy::Bounded(2).permits(1));
    assert!(!ResumePolicy::Bounded(2).permits(2));

    let ledger = concat!(
        "{\"config\":{\"digest\":\"cfg\"},\"format\":1,\"kind\":\"genesis\",\"min_reader\":1,\"min_writer\":1,\"origin_key\":\"create\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"create\",\"op\":\"create\",\"principal\":\"p\",\"target\":\"018f0000-0000-7000-8000-000000000001\"},\"resume\":{\"bounded\":2},\"seq\":1,\"thread\":\"018f0000-0000-7000-8000-000000000001\",\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1,\"workspace\":\"ws\"}\n",
        "{\"binary\":\"tekes-worker-1\",\"config_digest\":\"cfg\",\"instruction_digest\":\"ins\",\"kind\":\"run_start\",\"mode\":\"reconcile\",\"policy\":\"default\",\"recovery_ordinal\":0,\"run\":\"r1\",\"seq\":2,\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1}\n",
        "{\"binary\":\"tekes-worker-1\",\"config_digest\":\"cfg\",\"instruction_digest\":\"ins\",\"kind\":\"run_start\",\"mode\":\"reconcile\",\"policy\":\"default\",\"recovery_ordinal\":1,\"run\":\"r2\",\"seq\":3,\"ts\":\"2026-08-26T09:00:00.000Z\",\"v\":1}\n",
    );
    validate_ledger(ledger.as_bytes(), 1).expect("ordinal chain must replay");
}

#[test]
fn slice1_gate_18_upgrade_gate_barrier() {
    #[derive(Default)]
    struct CountingSync(AtomicUsize);
    impl SyncPolicy for CountingSync {
        fn full_sync(&self, _file: &fs::File) -> io::Result<()> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let root = FixtureRoot::discover().expect("fixture root");
    let fixture_event = |name: &str| {
        let bytes = read(&root.join("events").join(name)).expect("barrier fixture");
        Event::decode(&bytes[..bytes.len() - 1]).expect("barrier event")
    };
    let baseline = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    let events = baseline
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Event::decode(line).expect("baseline event"))
        .collect::<Vec<_>>();
    assert!(requires_barrier(&events[0], BarrierContext::default()));
    assert!(requires_barrier(&events[1], BarrierContext::default()));
    assert!(requires_barrier(&events[2], BarrierContext::default()));
    assert!(!requires_barrier(&events[3], BarrierContext::default()));
    assert!(!requires_barrier(&events[4], BarrierContext::default()));
    assert!(requires_barrier(&events[5], BarrierContext::default()));
    let tool_call = Event::decode(
        br#"{"args":{},"attempt":"a1","call":"c1","kind":"tool_call","name":"read","seq":1,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    )
    .expect("tool call");
    assert!(!requires_barrier(&tool_call, BarrierContext::default()));
    assert!(requires_barrier(
        &tool_call,
        BarrierContext {
            side_effectful_tool_call: true,
        }
    ));
    for case in [
        "approval-request.canonical.json",
        "approval-response.canonical.json",
        "attempt-dispatched.canonical.json",
        "attempt-recovery-adopt.canonical.json",
        "compact-manual.canonical.json",
        "effective-execution.canonical.json",
        "error-provider-terminal.canonical.json",
        "genesis.canonical.json",
        "input.canonical.json",
        "meta-rename.canonical.json",
        "meta-upgrade.canonical.json",
        "output-inline.canonical.json",
        "queue-edit.canonical.json",
        "run-start-ordinary.canonical.json",
        "settle-completed.canonical.json",
        "spawn.canonical.json",
        "stop-requested.canonical.json",
        "checkpoint.canonical.json",
    ] {
        assert!(
            requires_barrier(&fixture_event(case), BarrierContext::default()),
            "{case} is in the authoritative barrier registry"
        );
    }
    for case in [
        "compact.canonical.json",
        "epoch.canonical.json",
        "error-internal.canonical.json",
        "meta-title.canonical.json",
        "tool-result-ok.canonical.json",
        "turn-open.canonical.json",
    ] {
        assert!(
            !requires_barrier(&fixture_event(case), BarrierContext::default()),
            "{case} is not unconditionally durable"
        );
    }
    let directory = tempfile::tempdir().expect("tempdir");
    let ledger_path = directory.path().join("thread.jsonl");
    fs::write(&ledger_path, []).expect("empty ledger");
    let sync = Arc::new(CountingSync::default());
    let mut ledger = LockedLedger::open_with_sync(&ledger_path, 1, sync.clone()).expect("lock");

    ledger
        .append(events[0].clone(), true)
        .expect("genesis barrier");
    ledger
        .append(events[1].clone(), true)
        .expect("ack-bearing input barrier");
    ledger
        .append(events[2].clone(), true)
        .expect("run_start barrier");
    ledger
        .append(events[3].clone(), false)
        .expect("turn_open best effort");
    ledger
        .append(events[4].clone(), false)
        .expect("epoch rides next barrier");
    ledger
        .append(events[5].clone(), true)
        .expect("attempt orders epoch and turn_open");
    assert_eq!(sync.0.load(Ordering::SeqCst), 4);
    assert_eq!(ledger.next_seq(), 7);
    validate_ledger(&fs::read(&ledger_path).expect("ledger bytes"), 1)
        .expect("durable prefix validates");
}

#[test]
fn slice1_gate_29_torn_tail_fault_injection() {
    let root = FixtureRoot::discover().expect("fixture root");
    let baseline = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    let prefix = baseline
        .split_inclusive(|byte| *byte == b'\n')
        .take(7)
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    for case in [
        "bare-lf.jsonl",
        "half-line.jsonl",
        "invalid-utf8.jsonl",
        "interleaved-loss.jsonl",
    ] {
        let mut bytes = prefix.clone();
        bytes.extend(read(&root.join("torn").join(case)).expect("torn suffix"));
        let scan = scan_valid_prefix(&bytes, 1);
        assert!(scan.needs_repair(), "{case}");
        assert_eq!(scan.last_seq(), 8, "{case}");

        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("thread.jsonl");
        fs::write(&path, &bytes).expect("write torn ledger");
        assert_eq!(fs::read(&path).expect("read-only bytes"), bytes);
        {
            let ledger = LockedLedger::open(&path, 1).expect("lock-holder repair");
            assert_eq!(ledger.next_seq(), 9, "{case}");
        }
        let repaired = fs::read(&path).expect("repaired bytes");
        assert_eq!(repaired.len() as u64, scan.valid_bytes, "{case}");
        validate_ledger(&repaired, 1).unwrap_or_else(|error| panic!("{case}: {error}"));
    }
}

#[test]
fn slice1_gate_30_checkpoint_marker_replay() {
    let root = FixtureRoot::discover().expect("fixture root");
    // A checkpoint is a plain covering marker: replay always starts at
    // genesis, so a later supersede inside the covered prefix is an ordinary
    // retraction and the origin-key map is rebuilt from the events.
    for name in [
        "replay/checkpoint-valid.jsonl",
        "replay/checkpoint-invalidated.jsonl",
    ] {
        let projection = validate_ledger(&read(&root.join(name)).expect(name), 1)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(projection.origin_keys.get("k1"), Some(&1), "{name}");
        assert_eq!(projection.origin_keys.get("k9"), Some(&2), "{name}");
        assert_eq!(projection.origin_keys.get("k8"), Some(&3), "{name}");
    }
}

#[test]
fn slice1_gate_04_attempt_lease_commit() {
    let mut pool = AdmissionPool::new(1);
    assert!(pool.request(AdmissionLease {
        attempt: "a1".to_owned(),
        class: "provider".to_owned(),
    }));
    assert!(!pool.request(AdmissionLease {
        attempt: "a2".to_owned(),
        class: "provider".to_owned(),
    }));
    assert_eq!(pool.held_count(), 1);
    assert_eq!(pool.settle("void"), LeaseRelease::StaleNoOp);
    assert_eq!(pool.settle("a1"), LeaseRelease::Released);
    assert!(pool.is_held("a2"));

    let mut flow = AttemptFlow::new("a1", true);
    assert_eq!(flow.begin_http(), Err(TransactionError::BeforeDurability));
    flow.lease_granted().expect("lease before attempt");
    assert_eq!(flow.begin_http(), Err(TransactionError::BeforeDurability));
    flow.attempt_durable(6).expect("attempt barrier");
    assert_eq!(flow.begin_http(), Err(TransactionError::BeforeDurability));
    flow.dispatch_durable(7).expect("dispatch barrier");
    flow.begin_http().expect("send only after both barriers");
    assert_eq!(flow.phase, AttemptPhase::HttpInFlight);
    flow.terminal_received().expect("provider terminal");

    let mut markerless = AttemptFlow::new("a2", false);
    markerless.lease_granted().expect("lease");
    markerless.attempt_durable(8).expect("attempt barrier");
    markerless
        .begin_http()
        .expect("markerless adapter sends after attempt barrier");

    let marker_required = AdapterCapabilities {
        continuation: Continuation::Stateless,
        query_by_identity: QueryCapability::Available,
        dispatch_marker_required: true,
    };
    assert_eq!(
        decide_recovery(
            marker_required,
            sent_state(false, true),
            QueryResult::Unsupported,
            RunMode::Ordinary,
        ),
        RecoveryDecision::NotDispatched
    );
    assert_eq!(
        decide_recovery(
            marker_required,
            sent_state(true, true),
            QueryResult::Outcome,
            RunMode::Reconcile,
        ),
        RecoveryDecision::Adopt
    );
    assert_eq!(
        decide_recovery(
            marker_required,
            sent_state(true, true),
            QueryResult::Unsupported,
            RunMode::Reconcile,
        ),
        RecoveryDecision::Unresolved
    );
    let mut provider = FakeProvider::new(marker_required);
    provider.push_query_result(QueryResult::Outcome);
    provider.insert_response(
        "a1",
        ProviderTerminal {
            response_identity: "response-1".to_owned(),
            raw_response: br#"{"tool_calls":[]}"#.to_vec(),
            input_tokens: Some("10".to_owned()),
            output_tokens: Some("5".to_owned()),
        },
    );
    assert_eq!(provider.query("a1"), QueryResult::Outcome);
    assert_eq!(
        provider
            .adopted_response("a1")
            .expect("adopt material")
            .response_identity,
        "response-1"
    );
}

#[test]
fn slice1_gate_05_outcome_usage_crash_window() {
    let root = FixtureRoot::discover().expect("fixture root");
    let baseline = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    let prefix = first_lines(&baseline, 6);

    // The settling outcome carries its usage: an output without one is not
    // an outcome, so no crash window can separate the two.
    let bare = format!(
        "{prefix}{}",
        event_line(
            r#"{"attempt":"a1","content":[{"text":"bad","type":"text"}],"kind":"output","sealed":{"adapter":"fake","fragments":"f","version":1},"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#
        )
    );
    assert!(
        validate_ledger(bare.as_bytes(), 1).is_err(),
        "an outcome carries its usage"
    );

    let mut transaction = OutcomeCommit::new("a1");
    assert_eq!(
        transaction.release_lease(),
        Err(TransactionError::BeforeDurability)
    );
    transaction.outcome_appended(7).expect("terminal outcome");
    assert_eq!(
        transaction.release_lease(),
        Err(TransactionError::BeforeDurability)
    );
    transaction.durable().expect("outcome barrier");
    assert_eq!(transaction.release_lease().expect("lease release"), 7);

    // A recovery closes the attempt with one error carrying unavailable usage.
    let recovered = format!(
        "{prefix}{}{}{}",
        event_line(
            r#"{"attempt":"a1","decision":"not_dispatched","kind":"attempt_recovery","seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#
        ),
        event_line(
            r#"{"attempt":"a1","classification":"transport","kind":"error","recoverable":true,"seq":8,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"unavailable"},"v":1}"#
        ),
        event_line(
            r#"{"kind":"settle","outcome":"interrupted","reason":"recovered","seq":9,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#
        ),
    );
    let projection = validate_ledger(recovered.as_bytes(), 1).expect("recovery closure");
    let closure = projection
        .events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::Error))
        .expect("error outcome");
    assert_eq!(
        closure
            .usage()
            .and_then(|usage| usage.get("availability"))
            .and_then(serde_json::Value::as_str),
        Some("unavailable")
    );
    // An error that settles an attempt must carry usage; a turn-level error must not.
    let bare_error = format!(
        "{prefix}{}",
        event_line(
            r#"{"attempt":"a1","classification":"transport","kind":"error","recoverable":true,"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#
        )
    );
    assert!(validate_ledger(bare_error.as_bytes(), 1).is_err());
    let turn_error = format!(
        "{prefix}{}",
        event_line(
            r#"{"classification":"internal","kind":"error","recoverable":false,"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"unavailable"},"v":1}"#
        )
    );
    assert!(validate_ledger(turn_error.as_bytes(), 1).is_err());

    let output = first_lines(&baseline, 8);
    let output_projection = validate_ledger(output.as_bytes(), 1).expect("output path");
    let outcome = output_projection
        .events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::Output))
        .expect("output");
    assert_eq!(outcome.seq(), 7);
    assert_eq!(
        outcome
            .usage()
            .and_then(|usage| usage.get("input_tokens"))
            .and_then(serde_json::Value::as_str),
        Some("10")
    );
}

#[test]
fn slice1_gate_06_completed_turn_usage_exit() {
    let root = FixtureRoot::discover().expect("fixture root");
    let baseline = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    let mut ledger = first_lines(&baseline, 7);
    ledger.push_str(&event_line(
        r#"{"admits":[],"attempt":"a2","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":8,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1,"wire_digest":"wd2"}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"attempt":"a2","content":[{"text":"second","type":"text"}],"kind":"output","sealed":{"adapter":"fake","fragments":"f2","version":1},"seq":9,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"reported","input_tokens":"4","output_tokens":"2"},"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"args":{"path":"/tmp/x"},"attempt":"a2","call":"c1","kind":"tool_call","name":"read","seq":10,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"call":"c1","content":[{"text":"ok","type":"text"}],"kind":"tool_result","outcome":"ok","seq":11,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"kind":"settle","outcome":"completed","seq":12,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    let projection = validate_ledger(ledger.as_bytes(), 1).expect("completed turn");
    let outcomes = projection
        .events
        .iter()
        .filter(|event| matches!(event.kind(), EventKind::Output) && event.has_field("usage"))
        .map(Event::seq)
        .collect::<Vec<_>>();
    assert_eq!(outcomes, vec![7, 9]);
    assert_eq!(
        projection
            .events
            .iter()
            .filter(|event| matches!(event.kind(), EventKind::Settle))
            .count(),
        1
    );
    let state = classify(&projection.lifecycle, LockFacts::CALLER);
    assert_eq!(state, TailState::Settled);
    assert_eq!(
        run_decision(state, &projection.lifecycle),
        RunDecision::ExitClean
    );
}

/// A durable provider-admission wait (event `state{provider_admission}`)
/// is an obligation of the open turn: the line is `recovery_needed` without
/// unresolved work, nothing is spawned before `next_attempt_at`, and the run
/// that follows is ordinary regardless of the resume policy.
#[test]
fn slice1_gate_33b_admission_wait_is_a_durable_obligation_of_the_open_turn() {
    let facts = LifecycleFacts {
        latest_turn: Some(1),
        terminal_tail: false,
        stop_active: false,
        stop_closure_point: None,
        open_hold: false,
        answered_hold: false,
        runnable_inputs: vec![],
        turn_open_inputs: vec![12],
        live_inputs: vec![12],
        unstarted: false,
        unresolved_work: false,
        resume_policy: ResumePolicy::Never,
        recovery_ordinal: 0,
        continuation_wait_until: None,
        admission_wait_until: Some("2026-09-05T00:00:03.000Z".to_owned()),
    };
    let state = classify(&facts, LockFacts::FREE);
    assert_eq!(state, TailState::RecoveryNeeded);
    assert_eq!(facts.durable_wait_until(), Some("2026-09-05T00:00:03.000Z"));
    assert_eq!(
        ensure_action_at(state, &facts, Some("2026-09-05T00:00:01.000Z")),
        EnsureAction::None
    );
    assert_eq!(
        ensure_action_at(state, &facts, Some("2026-09-05T00:00:03.000Z")),
        EnsureAction::SpawnCandidate
    );
    assert_eq!(
        run_decision(state, &facts),
        RunDecision::Start(RunMode::Ordinary)
    );
    let without_wait = LifecycleFacts {
        admission_wait_until: None,
        ..facts
    };
    assert_eq!(
        run_decision(classify(&without_wait, LockFacts::FREE), &without_wait),
        RunDecision::Start(RunMode::Reconcile)
    );
}

#[test]
fn slice1_gate_33_run_mode_arbitration() {
    let facts = LifecycleFacts {
        latest_turn: Some(1),
        terminal_tail: false,
        stop_active: false,
        stop_closure_point: None,
        open_hold: false,
        answered_hold: false,
        runnable_inputs: vec![12],
        turn_open_inputs: vec![12],
        live_inputs: vec![12],
        unstarted: false,
        unresolved_work: true,
        resume_policy: ResumePolicy::Never,
        recovery_ordinal: 0,
        continuation_wait_until: None,
        admission_wait_until: None,
    };
    assert_eq!(classify(&facts, LockFacts::OTHER), TailState::Running);
    let under_lock = classify(&facts, LockFacts::CALLER);
    assert_eq!(under_lock, TailState::RecoveryNeeded);
    assert_eq!(
        run_decision(under_lock, &facts),
        RunDecision::Start(RunMode::Ordinary)
    );

    let directory = tempfile::tempdir().expect("tempdir");
    let root = FixtureRoot::discover().expect("fixture root");
    let path = directory.path().join("thread.jsonl");
    fs::write(
        &path,
        read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline"),
    )
    .expect("ledger");
    let winner = LockedLedger::open(&path, 1).expect("winner");
    assert!(matches!(
        LockedLedger::open(&path, 1),
        Err(StoreError::Busy)
    ));
    drop(winner);
    LockedLedger::open(&path, 1).expect("fresh ensure may acquire after exit");
}

#[test]
fn slice1_gate_34_slow_hold_lifecycle() {
    let ledger = hold_ledger_prefix();
    let parked = validate_ledger(ledger.as_bytes(), 1).expect("parked ledger");
    assert_eq!(
        classify(&parked.lifecycle, LockFacts::FREE),
        TailState::ParkedHold
    );
    assert_eq!(
        ensure_action(TailState::ParkedHold, &parked.lifecycle),
        EnsureAction::None
    );

    let mut queued = ledger;
    queued.push_str(&event_line(
        r#"{"content":[{"text":"later","type":"text"}],"kind":"input","origin_key":"q1","origin_tuple":{"client":"cli","key":"q1","op":"submit","principal":"p","target":"t"},"seq":10,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    ));
    let still_parked = validate_ledger(queued.as_bytes(), 1).expect("queued hold");
    assert_eq!(
        classify(&still_parked.lifecycle, LockFacts::FREE),
        TailState::ParkedHold
    );
    assert!(still_parked.lifecycle.runnable_inputs.is_empty());

    queued.push_str(&event_line(
        r#"{"call":"ask1","grant":true,"kind":"approval_response","origin_key":"ans1","origin_tuple":{"client":"cli","key":"ans1","op":"approve","principal":"p","target":"t"},"seq":11,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    let answered = validate_ledger(queued.as_bytes(), 1).expect("answered hold");
    assert_eq!(
        classify(&answered.lifecycle, LockFacts::CALLER),
        TailState::AnsweredHold
    );
    assert_eq!(
        run_decision(TailState::AnsweredHold, &answered.lifecycle),
        RunDecision::Start(RunMode::Ordinary)
    );
    assert!(answered.lifecycle.runnable_inputs.is_empty());
}

#[test]
fn slice1_gate_01_cold_boot_create_retry() {
    let create_origin = origin("create-1", "create");
    let mut index = CreateIndex::default();
    assert_eq!(
        index.resolve(&create_origin, "018f0000-0000-7000-8000-000000000101"),
        CreateResolution::Create {
            thread_id: "018f0000-0000-7000-8000-000000000101".to_owned()
        }
    );
    assert_eq!(
        index.resolve(&create_origin, "018f0000-0000-7000-8000-000000000102"),
        CreateResolution::Existing {
            thread_id: "018f0000-0000-7000-8000-000000000101".to_owned()
        }
    );
    assert_eq!(
        index.resolve(&create_origin, "018f0000-0000-7000-8000-000000000103"),
        CreateResolution::Existing {
            thread_id: "018f0000-0000-7000-8000-000000000101".to_owned()
        }
    );

    let directory = tempfile::tempdir().expect("tempdir");
    let store = ThreadStore::open(directory.path()).expect("thread store and fs probe");
    let first_id = "018f0000-0000-7000-8000-000000000101";
    let second_id = "018f0000-0000-7000-8000-000000000102";
    let first = store
        .create_thread(first_id, genesis_event(first_id, &create_origin))
        .expect("first create");
    assert!(first.created);
    let retry = store
        .create_thread(second_id, genesis_event(second_id, &create_origin))
        .expect("retry after reply loss");
    assert!(!retry.created);
    assert_eq!(retry.thread_id, first_id);
    assert_eq!(
        fs::read_dir(directory.path().join("threads"))
            .expect("threads")
            .count(),
        1
    );

    store.archive(first_id).expect("explicit archive");
    let archived_retry = store
        .create_thread(second_id, genesis_event(second_id, &create_origin))
        .expect("archive remains authoritative");
    assert_eq!(archived_retry.thread_id, first_id);
    assert!(!archived_retry.created);
    store.unarchive(first_id).expect("explicit unarchive");

    let race_root = tempfile::tempdir().expect("race root");
    let race_store = ThreadStore::open(race_root.path()).expect("race store");
    let race_origin = origin("create-race", "create");
    let barrier = Arc::new(Barrier::new(2));
    let outcomes = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for thread_id in [
            "018f0000-0000-7000-8000-000000000111",
            "018f0000-0000-7000-8000-000000000112",
        ] {
            let store = race_store.clone();
            let origin = race_origin.clone();
            let barrier = barrier.clone();
            handles.push(scope.spawn(move || {
                barrier.wait();
                store
                    .create_thread(thread_id, genesis_event(thread_id, &origin))
                    .expect("concurrent create")
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("create thread"))
            .collect::<Vec<_>>()
    });
    assert_eq!(outcomes[0].thread_id, outcomes[1].thread_id);
    assert_eq!(outcomes.iter().filter(|outcome| outcome.created).count(), 1);

    let crash_root = tempfile::tempdir().expect("crash root");
    let crash_store = ThreadStore::open(crash_root.path()).expect("crash store");
    let durable_id = "018f0000-0000-7000-8000-000000000121";
    let retry_id = "018f0000-0000-7000-8000-000000000122";
    let crash_origin = origin("create-crash", "create");
    let stage = crash_root
        .path()
        .join(".create-staging/create-crash-fixture");
    fs::create_dir(&stage).expect("staging folder");
    fs::create_dir(stage.join("assets")).expect("staging assets");
    fs::write(stage.join("main.jsonl"), []).expect("staging ledger");
    {
        let mut ledger = LockedLedger::open(stage.join("main.jsonl"), 1).expect("stage lock");
        ledger
            .append_contract(
                genesis_event(durable_id, &crash_origin),
                BarrierContext::default(),
            )
            .expect("durable staged genesis");
    }
    let recovered = crash_store
        .create_thread(retry_id, genesis_event(retry_id, &crash_origin))
        .expect("retry publishes staged create");
    assert!(!recovered.created);
    assert_eq!(recovered.thread_id, durable_id);
    assert!(crash_root.path().join("threads").join(durable_id).is_dir());
    assert!(!crash_root.path().join("threads").join(retry_id).exists());
}

#[test]
fn slice1_gate_02_dead_target_keyed_delivery() {
    #[derive(Default)]
    struct CountingSync(AtomicUsize);
    impl SyncPolicy for CountingSync {
        fn full_sync(&self, _file: &fs::File) -> io::Result<()> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let root = FixtureRoot::discover().expect("fixture root");
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("thread.jsonl");
    fs::write(
        &path,
        read(&root.join("invalid/_valid-baseline.jsonl")).expect("settled ledger"),
    )
    .expect("write ledger");
    let delivery_origin = origin("delivery-1", "submit");
    let index = DeliveryIndex::default();
    assert_eq!(
        index.resolve(&delivery_origin, 9),
        DeliveryResolution::AppendAt(9)
    );

    let event = Event::decode(
        br#"{"content":[{"text":"next","type":"text"}],"kind":"input","origin_key":"delivery-1","origin_tuple":{"client":"cli","key":"delivery-1","op":"submit","principal":"p","target":"t"},"seq":9,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    )
    .expect("input event");
    let sync = Arc::new(CountingSync::default());
    let mut ledger = LockedLedger::open_with_sync(&path, 1, sync.clone()).expect("lock");
    ledger.append(event, true).expect("durable input");
    assert_eq!(sync.0.load(Ordering::SeqCst), 1, "ack barrier completed");

    let mut index = index;
    index.commit(&delivery_origin, 9);
    assert_eq!(
        index.resolve(&delivery_origin, 10),
        DeliveryResolution::Reack { original_seq: 9 }
    );

    let folder_root = tempfile::tempdir().expect("folder root");
    let thread_store = ThreadStore::open(folder_root.path()).expect("thread store");
    let thread_id = "018f0000-0000-7000-8000-000000000201";
    let create_origin = origin("create-delivery", "create");
    thread_store
        .create_thread(thread_id, genesis_event(thread_id, &create_origin))
        .expect("create thread");
    let durable = thread_store
        .append_keyed(thread_id, &delivery_origin, |seq| {
            make_input_event(seq, &delivery_origin, "next").map_err(StoreError::from)
        })
        .expect("dead-target locked append");
    assert!(!durable.deduplicated);
    let mut commit = DeliveryCommit::new("delivery-1");
    commit.appended(durable.seq).expect("append recorded");
    assert_eq!(
        commit.acknowledge(),
        Err(TransactionError::BeforeDurability)
    );
    commit.durable().expect("barrier recorded");
    assert_eq!(
        commit.acknowledge().expect("ack after durability"),
        durable.seq
    );
    let retry = thread_store
        .append_keyed(thread_id, &delivery_origin, |seq| {
            make_input_event(seq, &delivery_origin, "must not append").map_err(StoreError::from)
        })
        .expect("retry re-ack");
    assert!(retry.deduplicated);
    assert_eq!(retry.seq, durable.seq);
    thread_store.archive(thread_id).expect("archive");
    assert!(matches!(
        thread_store.append_keyed(thread_id, &origin("after-archive", "submit"), |seq| {
            make_input_event(seq, &origin("after-archive", "submit"), "no")
                .map_err(StoreError::from)
        }),
        Err(StoreError::Archived)
    ));
}

#[test]
fn slice1_gate_03_alive_target_receipt_correlation() {
    let first = origin("key-a", "submit");
    let second = origin("key-b", "submit");
    let mut index = DeliveryIndex::default();
    index.commit(&first, 20);
    index.commit(&second, 21);

    let receipts = BTreeMap::from([
        ("delivery-a", (20_u64, false)),
        ("delivery-b", (21_u64, false)),
    ]);
    assert_eq!(receipts.get("delivery-b"), Some(&(21, false)));
    assert_eq!(
        index.resolve(&first, 22),
        DeliveryResolution::Reack { original_seq: 20 }
    );
    assert_eq!(
        index.resolve(&second, 22),
        DeliveryResolution::Reack { original_seq: 21 }
    );
    assert!(!receipts.contains_key("missing-delivery"));
}

#[test]
fn slice1_gate_13_busy_unknown_worker() {
    let root = FixtureRoot::discover().expect("fixture root");
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("thread.jsonl");
    let original = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    fs::write(&path, &original).expect("write ledger");
    let surviving_worker = LockedLedger::open(&path, 1).expect("surviving worker");
    assert!(matches!(
        LockedLedger::open(&path, 1),
        Err(StoreError::Busy)
    ));
    assert_eq!(fs::read(&path).expect("unchanged while busy"), original);
    drop(surviving_worker);
    LockedLedger::open(&path, 1).expect("normal delivery resumes after holder exit");

    let first_supervisor = RootLock::acquire(directory.path()).expect("root winner");
    assert!(matches!(
        RootLock::acquire(directory.path()),
        Err(StoreError::Busy)
    ));
    drop(first_supervisor);
    RootLock::acquire(directory.path()).expect("root lock released on supervisor exit");
}

#[test]
fn slice1_gate_19_canary_per_run_attribution() {
    let mut ledger = hold_ledger_prefix();
    ledger.push_str(&event_line(
        r#"{"binary":"tekes-worker-2","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"reconcile","policy":"default","recovery_ordinal":0,"run":"r2","seq":10,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"call":"ask1","kind":"tool_result","outcome":{"aborted":"recovered"},"seq":11,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"kind":"settle","outcome":"interrupted","reason":"recovered","seq":12,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    let projection = validate_ledger(ledger.as_bytes(), 1).expect("two attributed runs");
    assert_eq!(projection.run_ranges.len(), 2);
    assert_eq!(projection.run_ranges[0].run, "r1");
    assert_eq!(
        (
            projection.run_ranges[0].start_seq,
            projection.run_ranges[0].end_seq
        ),
        (3, 9)
    );
    assert_eq!(projection.run_ranges[1].run, "r2");
    assert_eq!(
        (
            projection.run_ranges[1].start_seq,
            projection.run_ranges[1].end_seq
        ),
        (10, 12)
    );
}

#[test]
fn slice1_gate_12_supervisor_total_failure() {
    let root = FixtureRoot::discover().expect("fixture root");
    let baseline = read(&root.join("invalid/_valid-baseline.jsonl")).expect("baseline");
    let mut ledger = first_lines(&baseline, 6);
    ledger.push_str(&event_line(
        r#"{"attempt":"a1","kind":"attempt_dispatched","seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"binary":"tekes-worker-2","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"reconcile","policy":"default","recovery_ordinal":0,"run":"recovery","seq":8,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"attempt":"a1","decision":"adopt","inventory":["c1"],"kind":"attempt_recovery","response":{"asset":"sha256-response"},"seq":9,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"attempt":"a1","content":[{"args":{"path":"/tmp/x"},"call":"c1","name":"read","type":"tool-call"}],"kind":"output","sealed":{"adapter":"fake","fragments":"adopted","version":1},"seq":10,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"reported","input_tokens":"10","output_tokens":"3"},"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"args":{"path":"/tmp/x"},"attempt":"a1","call":"c1","kind":"tool_call","name":"read","seq":11,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"call":"c1","kind":"tool_result","outcome":{"aborted":"recovered"},"seq":12,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    ledger.push_str(&event_line(
        r#"{"kind":"settle","outcome":"interrupted","reason":"recovered","seq":13,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ));
    let projection = validate_ledger(ledger.as_bytes(), 1).expect("journaled adopt");
    let adopted = projection
        .events
        .iter()
        .find(|event| matches!(event.kind(), EventKind::Output))
        .expect("adopted output");
    assert_eq!(adopted.seq(), 10);
    assert_eq!(
        adopted
            .usage()
            .and_then(|usage| usage.get("output_tokens"))
            .and_then(serde_json::Value::as_str),
        Some("3")
    );
    assert_eq!(
        projection.run_ranges.last().expect("recovery run").run,
        "recovery"
    );
    let capabilities = AdapterCapabilities {
        continuation: Continuation::ServerManaged,
        query_by_identity: QueryCapability::Available,
        dispatch_marker_required: true,
    };
    assert_eq!(
        decide_recovery(
            capabilities,
            sent_state(true, true),
            QueryResult::Outcome,
            RunMode::Reconcile,
        ),
        RecoveryDecision::Adopt
    );
    let mut tool = FakeToolBackend::default();
    tool.set_outcome("c1", Ok(schema::IJsonValue::from("must-not-run")));
    assert_eq!(
        tool.execution_count("c1"),
        0,
        "adopted calls are aborted, not executed"
    );
    let mut host = FakeProcessHost::default();
    host.spawn("worker-a").expect("spawn fake worker");
    host.signal("worker-a")
        .expect("supervisor death closes/cancels worker");
    host.exit("worker-a", 1);
    assert_eq!(host.wait("worker-a").expect("reap"), 1);

    for boundary in 9..=13 {
        let crashed = first_lines(ledger.as_bytes(), boundary);
        validate_ledger(crashed.as_bytes(), 1)
            .unwrap_or_else(|error| panic!("adopt crash boundary {boundary}: {error}"));
        let completed = format!(
            "{crashed}{}",
            ledger
                .lines()
                .skip(boundary)
                .map(event_line)
                .collect::<String>()
        );
        assert_eq!(completed, ledger);
        validate_ledger(completed.as_bytes(), 1).expect("idempotent adopt completion");
    }

    assert_eq!(
        RECOVERY_ORDER,
        [
            RecoveryStage::SpawnEdges,
            RecoveryStage::Tools,
            RecoveryStage::Attempts,
            RecoveryStage::Holds,
        ]
    );
    let mut drain = DrainTracker::new(3);
    assert_eq!(
        drain.observe("A", true, false, 10),
        DrainAction::WaitBusyUnknown
    );
    assert_eq!(
        drain.observe("A", true, false, 12),
        DrainAction::WaitBusyUnknown
    );
    assert_eq!(drain.observe("A", true, false, 13), DrainAction::Quarantine);
    assert_eq!(drain.observe("B", true, true, 13), DrainAction::ReapOwned);
    assert_eq!(drain.observe("A", false, false, 14), DrainAction::Sweep);
}

#[test]
fn slice1_gate_14_stop_mid_grandchild_crash() {
    let root = FixtureRoot::discover().expect("fixture root");
    let cancel = read(&root.join("replay/cancel-before-start.jsonl")).expect("cancel fixture");
    let active = validate_ledger(first_lines(&cancel, 2).as_bytes(), 1).expect("open stop");
    assert_eq!(
        classify(&active.lifecycle, LockFacts::FREE),
        TailState::StoppedActive
    );
    assert_eq!(
        ensure_action(TailState::StoppedActive, &active.lifecycle),
        EnsureAction::SpawnRecoveryCandidate
    );
    assert_eq!(
        run_decision(TailState::StoppedActive, &active.lifecycle),
        RunDecision::Start(RunMode::Reconcile)
    );
    let closed = validate_ledger(&cancel, 1).expect("cancel-before-start closes generation");
    assert!(!closed.lifecycle.stop_active);
    assert_eq!(
        classify(&closed.lifecycle, LockFacts::FREE),
        TailState::Settled
    );

    let release = [
        r#"{"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},"resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}"#,
        r#"{"content":[{"text":"held","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"generation":1,"kind":"stop_requested","origin_key":"stop1","origin_tuple":{"client":"cli","key":"stop1","op":"stop","principal":"p","target":"t"},"seq":3,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"content":[{"text":"release","type":"text"}],"kind":"input","origin_key":"i2","origin_tuple":{"client":"cli","key":"i2","op":"submit","principal":"p","target":"t"},"seq":4,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":5,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"kind":"turn_open","seq":6,"trigger":{"inputs":[2,4]},"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"kind":"settle","outcome":"completed","seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ]
    .into_iter()
    .map(event_line)
    .collect::<String>();
    validate_ledger(release.as_bytes(), 1).expect("new input releases held prefix in seq order");

    let mut tree = StopTree::default();
    tree.add_root("root");
    tree.add_child("root", "child");
    tree.add_child("child", "grandchild");
    assert!(!tree.mark_stop_durable("grandchild"));
    assert!(tree.mark_stop_durable("root"));
    assert!(tree.mark_stop_durable("child"));
    assert!(tree.mark_stop_durable("grandchild"));
    assert!(tree.mark_signaled("root"));
    assert!(tree.mark_signaled("child"));
    assert!(tree.mark_signaled("grandchild"));
    assert!(
        !tree.mark_settled("root"),
        "root cannot settle before descendants"
    );
    assert!(tree.mark_settled("grandchild"));
    assert!(tree.mark_settled("child"));
    assert!(tree.mark_settled("root"));
    assert!(tree.root_settled());
}

fn first_lines(bytes: &[u8], count: usize) -> String {
    String::from_utf8(
        bytes
            .split_inclusive(|byte| *byte == b'\n')
            .take(count)
            .flatten()
            .copied()
            .collect(),
    )
    .expect("fixtures are UTF-8")
}

fn event_line(event: &str) -> String {
    format!("{event}\n")
}

fn hold_ledger_prefix() -> String {
    [
        r#"{"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},"resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}"#,
        r#"{"content":[{"text":"start","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":"t"},"seq":2,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"kind":"turn_open","seq":4,"trigger":{"inputs":[2]},"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"adapter":"fake","id":"e1","kind":"epoch","model":"fake","reason":"initial","renderer":1,"seq":5,"system":{"asset":"sha256-aa","digest":"d1"},"tools":{"asset":"sha256-td","digest":"td"},"ts":"2026-08-26T09:00:00.000Z","v":1}"#,
        r#"{"admits":[{"from":2,"to":2}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":6,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1,"wire_digest":"wd"}"#,
        r#"{"attempt":"a1","content":[{"args":{"question":"continue?"},"call":"ask1","name":"ask_user","type":"tool-call"}],"kind":"output","sealed":{"adapter":"fake","fragments":"ask","version":1},"seq":7,"ts":"2026-08-26T09:00:00.000Z","turn":1,"usage":{"availability":"reported","input_tokens":"5","output_tokens":"1"},"v":1}"#,
        r#"{"args":{"question":"continue?"},"attempt":"a1","call":"ask1","kind":"tool_call","name":"ask_user","seq":8,"source":"provider","ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
        r#"{"call":"ask1","kind":"approval_request","scope":"answer","seq":9,"ts":"2026-08-26T09:00:00.000Z","turn":1,"v":1}"#,
    ]
    .into_iter()
    .map(event_line)
    .collect()
}

fn origin(key: &str, op: &str) -> OriginTuple {
    OriginTuple {
        principal: "p".to_owned(),
        client: "cli".to_owned(),
        target: "t".to_owned(),
        op: op.to_owned(),
        key: key.to_owned(),
    }
}

fn genesis_event(thread_id: &str, origin: &OriginTuple) -> Event {
    let value = serde_json::json!({
        "v": 1,
        "seq": 1,
        "kind": "genesis",
        "ts": "2026-08-26T09:00:00.000Z",
        "format": 1,
        "thread": thread_id,
        "workspace": "ws",
        "min_reader": 1,
        "min_writer": 1,
        "resume": "never",
        "config": {"digest": "cfg"},
        "origin_key": origin.key,
        "origin_tuple": origin
    });
    Event::decode(&serde_json::to_vec(&value).expect("genesis JSON")).expect("genesis event")
}

fn make_input_event(seq: u64, origin: &OriginTuple, text: &str) -> Result<Event, SchemaError> {
    let value = serde_json::json!({
        "v": 1,
        "seq": seq,
        "kind": "input",
        "ts": "2026-08-26T09:00:00.000Z",
        "content": [{"type": "text", "text": text}],
        "origin_key": origin.key,
        "origin_tuple": origin
    });
    Event::decode(&serde_json::to_vec(&value).expect("input JSON"))
}

trait UnwrapErrOrElse<T, E> {
    fn unwrap_err_or_else(self, fallback: impl FnOnce() -> E) -> E;
}

impl<T, E> UnwrapErrOrElse<T, E> for Result<T, E> {
    fn unwrap_err_or_else(self, fallback: impl FnOnce() -> E) -> E {
        match self {
            Ok(_) => fallback(),
            Err(error) => error,
        }
    }
}
