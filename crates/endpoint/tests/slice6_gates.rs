use std::collections::BTreeMap;
use std::fs;

use endpoint::{
    AcceptedStreamFrame, ClientRequest, EndpointJournal, MuxBuffer, MuxBufferError, NativeEndpoint,
    NativeEndpointError, Projector, SessionEvent, SessionHistoryEntry, StitchDecision,
    history_page, stitch_window, validate_request, validate_session_id,
};
use profile::{
    ConfigSnapshot, Model, Provider, ProvidersConfig, ResolvedWorkspace, RevisionVector,
    SettingsConfig, WorkspacePolicy,
};
use schema::{IJsonValue, OriginTuple, ResumePolicy, validate_ledger};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use test_support::{FixtureRoot, read};

#[derive(Deserialize)]
struct AuthorityLock {
    files: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct AuthorityHistory {
    events: Vec<SessionHistoryEntry>,
}

#[test]
fn slice6_gate_47_endpoint_authority_byte_sync() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    fixtures.verify_manifest().expect("fixture manifest");
    let endpoint = fixtures.join("endpoint");
    let lock: AuthorityLock = serde_json::from_slice(
        &read(&endpoint.join("authority-lock.canonical.json")).expect("authority lock"),
    )
    .expect("lock JSON");
    let disk = fs::read_dir(endpoint.join("authority"))
        .expect("authority dir")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(disk, lock.files.keys().cloned().collect());
    for (name, expected) in lock.files {
        let bytes = read(&endpoint.join("authority").join(&name)).expect("authority fixture");
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected, "{name}");
    }
    assert!(validate_session_id("018f0000-0000-7000-8000-000000000003").is_ok());
    assert!(validate_session_id("20:dsh-loopback:sessionsession-1").is_err());
    let rpc: serde_json::Value = serde_json::from_slice(
        &read(&endpoint.join("authority/rpc-envelopes.json")).expect("RPC fixture"),
    )
    .expect("RPC fixture JSON");
    let request: ClientRequest = serde_json::from_value(rpc["request"].clone()).expect("request");
    assert!(
        validate_request(&request, &NativeEndpoint::capabilities()).is_err(),
        "the historical V2 authority must not remain in the public catalog"
    );
}

#[test]
fn slice6_gate_48_stable_projection_journal() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let source = read(&fixtures.join("endpoint/projection-source.jsonl")).expect("source");
    let projection = validate_ledger(&source, 1).expect("valid source ledger");
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let mut projector = Projector::default();
    projector
        .reconcile(&projection.events, &journal)
        .expect("project");
    let expected = read(&fixtures.join("endpoint/projection-expected.jsonl")).expect("expected");
    assert_eq!(fs::read(journal.path()).expect("journal bytes"), expected);
    assert_eq!(journal.records().expect("records").len(), 6);

    let mut replay = Projector::default();
    replay
        .reconcile(&projection.events, &journal)
        .expect("idempotent replay");
    assert_eq!(fs::read(journal.path()).expect("journal bytes"), expected);

    let corrupt_folder = tempfile::tempdir().expect("corrupt folder");
    let corrupt = EndpointJournal::open(corrupt_folder.path()).expect("corrupt journal");
    let bogus = SessionEvent {
        event_type: "turn/start".to_owned(),
        seq: 0,
        time: 1_787_734_800_000.0,
        data: IJsonValue::parse_str(r#"{"turn":99}"#).expect("bogus data"),
        ignorable: None,
        source_event_seqs: None,
        surface_op: None,
    };
    corrupt
        .append_kernel(vec![1], "bogus-slot", bogus)
        .expect("durable bogus slot");
    assert!(
        Projector::default()
            .reconcile(&projection.events, &corrupt)
            .is_err(),
        "a durable projection identity absent from the Kernel prefix is corruption"
    );
}

#[test]
fn slice6_gate_49_chunks_are_transient_and_never_journaled() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let source = read(&fixtures.join("endpoint/projection-source.jsonl")).expect("source");
    let projection = validate_ledger(&source, 1).expect("valid source ledger");
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let mut projector = Projector::default();
    projector
        .reconcile(&projection.events[..6], &journal)
        .expect("project through attempt");
    let frame =
        |frame: u64, channel: &str, delta: &str, call_id: Option<&str>, name: Option<&str>| {
            AcceptedStreamFrame {
                arguments_complete: None,
                attempt: "a1".to_owned(),
                frame,
                channel: channel.to_owned(),
                block: 0,
                delta: delta.to_owned(),
                call_id: call_id.map(str::to_owned),
                name: name.map(str::to_owned),
                time: 1_787_734_800_001.0,
            }
        };
    let live = projector
        .stream_event(&frame(0, "text", "do", None, None))
        .expect("live chunk");
    let retry = projector
        .stream_event(&frame(0, "text", "do", None, None))
        .expect("projecting a frame is pure");
    assert_eq!(live, retry);
    assert_eq!(live.seq, 0, "the frame ordinal is the presentation version");
    // Usage has one owner, the durable `assistant/message`; a chunk carries no token claim.
    assert!(
        serde_json::to_value(&live.data)
            .unwrap()
            .get("usagePreview")
            .is_none()
    );
    let tool_delta = projector
        .stream_event(&frame(1, "tool", r#"{"path":"#, Some("c1"), Some("read")))
        .expect("live tool chunk");
    assert_eq!(
        serde_json::to_value(&tool_delta.data).unwrap()["chunk"]["type"],
        "tool-call-delta"
    );
    projector
        .reconcile(&projection.events[6..], &journal)
        .expect("finish projection");
    assert!(
        projector
            .stream_event(&frame(2, "text", "late", None, None))
            .is_err(),
        "frames after terminal output must fail closed"
    );
    // The journal is a projection of the ledger: it holds no chunk, and the
    // assistant message names no chunk provenance.
    let records = journal.records().expect("records");
    assert!(
        records
            .iter()
            .all(|record| record.event.event_type != "assistant/chunk")
    );
    let page = history_page(&records, None, Some(1)).expect("tail page");
    assert!(
        page.events
            .iter()
            .all(|entry| entry.event.event_type != "assistant/chunk")
    );
    let assistant = page
        .events
        .iter()
        .find(|entry| entry.event.event_type == "assistant/message")
        .expect("assistant message");
    assert_eq!(assistant.event.source_event_seqs, None);
}

#[test]
fn slice6_gate_50_endpoint_mutation_and_archive_contract() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let value: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("endpoint/mutations.canonical.json")).expect("mutations"),
    )
    .expect("mutations JSON");
    assert_eq!(value["archive"]["method"], "workspace.archiveSession");
    assert_eq!(value["unarchive"]["method"], "workspace.unarchiveSession");
    assert_ne!(value["archive"]["method"], value["unarchive"]["method"]);
    assert_eq!(value["prompt"]["historyReconcile"], "independent");
    assert_eq!(
        value["unsupported"]["error"]["code"],
        "unsupported-capability"
    );

    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    let session = "018f0000-0000-7000-8000-000000000003";
    let create = origin(session, "create", "create-1");
    assert!(
        endpoint
            .create_session(
                session,
                "workspace-1",
                "cfg",
                ResumePolicy::Never,
                "2026-08-26T09:00:00.000Z",
                &create,
            )
            .expect("create")
    );
    profile::ConfigRepository::open(root.path())
        .expect("config repository")
        .publish_workspace(
            0,
            &profile::WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "workspace-1".to_owned(),
                name: "Project".to_owned(),
                cwd: vec![root.path().display().to_string()],
                folders: vec![],
                policy: None,
            },
        )
        .expect("workspace configuration");
    let prompt = origin(session, "submit", "prompt-1");
    let content = IJsonValue::parse_str(r#"[{"type":"text","text":"hello"}]"#).expect("content");
    let first = endpoint
        .prompt(
            session,
            "2026-08-26T09:00:01.000Z",
            &prompt,
            content.clone(),
            false,
        )
        .expect("prompt");
    let retry = endpoint
        .prompt(session, "2026-08-26T09:00:01.000Z", &prompt, content, false)
        .expect("prompt retry");
    assert_eq!(first.seq, retry.seq);
    assert!(retry.deduplicated);
    let ledger = fs::read(root.path().join("threads").join(session).join("main.jsonl")).unwrap();
    let projection = validate_ledger(&ledger, 1).unwrap();
    let input = serde_json::to_value(projection.events.last().unwrap().raw()).unwrap();
    assert_eq!(input["kind"], "input");
    let digest = input["submission"]["configDigest"]
        .as_str()
        .expect("submission captured before any run");
    assert!(
        root.path()
            .join("threads")
            .join(session)
            .join("assets")
            .join(format!("sha256-{digest}"))
            .is_file()
    );
    let wrong_operation = origin(session, "rename", "prompt-wrong-op");
    assert!(matches!(
        endpoint.prompt(
            session,
            "2026-08-26T09:00:01.000Z",
            &wrong_operation,
            IJsonValue::parse_str(r#"[{"type":"text","text":"wrong"}]"#).expect("wrong content"),
            false,
        ),
        Err(NativeEndpointError::Origin)
    ));
    endpoint.archive_session(session).expect("archive");
    assert!(matches!(
        endpoint.history(session, None, None),
        Err(NativeEndpointError::Archived)
    ));
    assert!(matches!(
        endpoint.prompt(
            session,
            "2026-08-26T09:00:02.000Z",
            &origin(session, "submit", "archived-prompt"),
            IJsonValue::parse_str(r#"[{"type":"text","text":"held"}]"#).expect("held content"),
            false,
        ),
        Err(NativeEndpointError::Archived)
    ));
    endpoint.unarchive_session(session).expect("unarchive");
    assert!(endpoint.history(session, None, None).is_ok());
}

#[test]
fn slice6_gate_51_inventory_and_model_readiness_are_independent() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let inventory: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("endpoint/authority/session-inventory.json")).expect("inventory"),
    )
    .expect("inventory JSON");
    let models: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("endpoint/authority/session-models.json")).expect("models"),
    )
    .expect("models JSON");
    assert_eq!(inventory["sessions"]["items"][0]["sessionId"], "session-1");
    assert_eq!(models["current"]["model"], "deepseek-chat");
    assert!(models["failures"].is_array());

    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    let session = "018f0000-0000-7000-8000-000000000003";
    endpoint
        .create_session(
            session,
            "workspace-1",
            "cfg",
            ResumePolicy::Never,
            "2026-08-26T09:00:00.000Z",
            &origin(session, "create", "create-inventory"),
        )
        .expect("create");
    let running = std::collections::HashSet::from([session.to_owned()]);
    let listed = endpoint.list_sessions(&running).expect("inventory");
    assert_eq!(listed.len(), 1);
    assert!(listed[0].running);
    assert!(!listed[0].archived);
    assert_eq!(listed[0].workspace_id, "workspace-1");
    assert_eq!(listed[0].updated_at, 1_787_734_800_000);
    assert!(listed[0].blank);
    assert_eq!(listed[0].as_of_seq, 1);
    let snapshot = ConfigSnapshot {
        format: 1,
        workspace: ResolvedWorkspace {
            format: 1,
            revision: 1,
            id: "workspace-1".to_owned(),
            name: "Project".to_owned(),
            folder_binding: None,
            selected_cwd: None,
            cwd: vec!["/workspace/project".to_owned()],
            policy: WorkspacePolicy {
                provider: Some("deepseek".to_owned()),
                model: Some("deepseek-chat".to_owned()),
                ..WorkspacePolicy::default()
            },
        },
        providers: ProvidersConfig {
            format: 1,
            revision: 1,
            web_search: None,
            providers: vec![Provider {
                id: "deepseek".to_owned(),
                name: None,
                adapter: "chat_completions".to_owned(),
                dialect: "deepseek_chat_v1".to_owned(),
                endpoint_owner: "deepseek".to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: "runtime-deepseek-2026-08-23".to_owned(),
                endpoint: "https://example.invalid".to_owned(),
                credential_key: None,
                models: vec![Model {
                    id: "deepseek-chat".to_owned(),
                    profile: "deepseek_chat_v1:deepseek-chat".to_owned(),
                    enabled: true,
                    context_window_tokens: 64_000,
                    compact_trigger_tokens: 48_000,
                }],
            }],
        },
        legacy_integrations: (),
        settings: SettingsConfig {
            format: 1,
            revision: 1,
            default_provider: None,
            default_model: None,
            limits: None,
        },
        session_settings: None,
        revisions: RevisionVector {
            workspace: 1,
            providers: 1,
            legacy_integrations: (),
            settings: 1,
            session_settings: None,
        },
    };
    let failures = IJsonValue::parse_str(
        r#"[{"id":"offline","name":"Offline Provider","message":"unavailable"}]"#,
    )
    .expect("failures");
    let catalog = NativeEndpoint::models(&snapshot, failures).expect("catalog");
    let catalog: serde_json::Value = serde_json::to_value(catalog).expect("catalog JSON");
    assert_eq!(catalog["current"]["model"], "deepseek-chat");
    assert_eq!(catalog["failures"][0]["id"], "offline");
    assert_eq!(catalog["routable"], true);
}

#[test]
fn slice6_gate_52_stitch_gap_overlap_and_unknown_fail_closed() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let stitch: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("endpoint/stitch.canonical.json")).expect("stitch"),
    )
    .expect("stitch JSON");
    assert_eq!(
        stitch["overlap"]["result"],
        serde_json::json!([0, 1, 2, 3, 4, 5, 6, 7])
    );
    assert_eq!(stitch["baselineAhead"]["refetch"], true);
    assert_eq!(stitch["gap"]["refetch"], true);
    let unknown: serde_json::Value = serde_json::from_slice(
        &read(&fixtures.join("endpoint/authority/unknown-events.json")).expect("unknown"),
    )
    .expect("unknown JSON");
    assert!(unknown["required"].get("ignorable").is_none());
    assert_eq!(unknown["ignorable"]["ignorable"], true);

    let required: SessionEvent =
        serde_json::from_value(unknown["required"].clone()).expect("required event");
    assert!(required.validate().is_err());
    let ignorable: SessionEvent =
        serde_json::from_value(unknown["ignorable"].clone()).expect("ignorable event");
    assert!(ignorable.validate().is_ok());

    let history: AuthorityHistory = serde_json::from_slice(
        &read(&fixtures.join("endpoint/authority/session-history.json")).expect("history"),
    )
    .expect("history JSON");
    let events = history
        .events
        .into_iter()
        .map(|entry| entry.event)
        .collect::<Vec<_>>();
    let mut next = events.last().expect("tail").clone();
    next.seq = 7;
    let installed =
        stitch_window(events.clone(), vec![events[5].clone(), next], 6).expect("stitch");
    assert!(matches!(installed, StitchDecision::Installed(ref items) if items.len() == 8));
    assert_eq!(
        stitch_window(events.clone(), Vec::new(), 8).expect("baseline ahead"),
        StitchDecision::RefetchTail
    );
    assert_eq!(
        stitch_window(Vec::new(), Vec::new(), -1).expect("empty baseline"),
        StitchDecision::Installed(Vec::new())
    );
    let mut first_live = events[0].clone();
    first_live.seq = 0;
    assert!(matches!(
        stitch_window(Vec::new(), vec![first_live], -1).expect("first live"),
        StitchDecision::Installed(ref items) if items.len() == 1
    ));
    let unknown_field = serde_json::json!({
        "type": "turn/start",
        "seq": 0,
        "time": 0,
        "data": {"turn": 1},
        "extra": true
    });
    assert!(serde_json::from_value::<SessionEvent>(unknown_field).is_err());
    let mut unsafe_seq = events[0].clone();
    unsafe_seq.seq = 9_007_199_254_740_992;
    assert!(unsafe_seq.validate().is_err());
    let mut buffer = MuxBuffer::new(1, 8);
    buffer.push(vec![1, 2, 3]).expect("first frame");
    assert_eq!(buffer.push(vec![4]), Err(MuxBufferError::LiveGap));
}

fn origin(target: &str, operation: &str, key: &str) -> OriginTuple {
    OriginTuple {
        principal: "frank".to_owned(),
        client: "tekes".to_owned(),
        target: target.to_owned(),
        op: operation.to_owned(),
        key: key.to_owned(),
    }
}

#[test]
fn tool_argument_completion_keeps_attempt_identity_in_live_and_replay() {
    let fixtures = FixtureRoot::discover().unwrap();
    let source = read(&fixtures.join("endpoint/projection-source.jsonl")).unwrap();
    let projection = validate_ledger(&source, 1).unwrap();
    let folder = tempfile::tempdir().unwrap();
    let journal = EndpointJournal::open(folder.path()).unwrap();
    let mut projector = Projector::default();
    projector
        .reconcile(&projection.events[..6], &journal)
        .unwrap();
    let mut frame = AcceptedStreamFrame {
        arguments_complete: Some(true),
        attempt: "a1".into(),
        frame: 0,
        channel: "tool".into(),
        block: 0,
        delta: String::new(),
        call_id: Some("left".into()),
        name: Some("record_left".into()),
        time: 1_787_734_800_000.0,
    };
    let live = projector.stream_event(&frame).unwrap();
    // Chunks are transient: projecting the same frame twice is pure and the
    // journal never records it.
    assert_eq!(projector.stream_event(&frame).unwrap(), live);
    assert!(
        journal
            .records()
            .unwrap()
            .iter()
            .all(|record| record.event.event_type != "assistant/chunk")
    );
    let data = serde_json::to_value(&live.data).unwrap();
    assert_eq!(data["chunk"]["argumentsComplete"], true);
    assert_eq!(data["chunk"]["assistantFrameId"], "a1");
    assert_eq!(data["chunk"]["argumentsDelta"], "");
    frame.channel = "text".into();
    frame.call_id = None;
    frame.name = None;
    assert!(projector.stream_event(&frame).is_err());
}
