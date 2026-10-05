use endpoint::{
    ActionableRegistry, EndpointJournal, EndpointSubscriptionHub, MuxHostDescription,
    MuxHostProduct, MuxProtocolError, SessionActionable, SessionActionableKind, SessionAddress,
    SessionEndpointCapability, SessionMuxClientFrame, SessionMuxGeneration, SessionMuxServerFrame,
    SessionStreamTarget, SessionSyncFrame, WorkspaceBaseline, frozen_history_page,
    open_journal_follow,
};
use endpoint::{SessionEvent, SessionHistoryEntry, SurfaceOperation};
use schema::IJsonValue;

const SESSION: &str = "018f0000-0000-7000-8000-000000000003";

fn data(value: &str) -> IJsonValue {
    IJsonValue::parse_str(value).expect("I-JSON")
}

fn event(text: &str, time: f64) -> SessionEvent {
    SessionEvent {
        event_type: "user/message".to_owned(),
        seq: 0,
        time,
        data: data(&format!(
            r#"{{"text":{}}}"#,
            serde_json::to_string(text).unwrap()
        )),
        ignorable: None,
        source_event_seqs: None,
        surface_op: Some(SurfaceOperation::Append("append".to_owned())),
    }
}

fn address() -> SessionAddress {
    SessionAddress {
        session_id: SESSION.to_owned(),
    }
}

#[test]
fn description_requires_protocol_and_the_recovery_capabilities() {
    let mut capabilities = SessionEndpointCapability::required();
    capabilities.insert(SessionEndpointCapability::Models);
    let description = MuxHostDescription {
        protocol_version: 3,
        product: MuxHostProduct {
            name: "TekesKernel".to_owned(),
            version: "dev-42".to_owned(),
        },
        capabilities,
        cwd: "/workspace".to_owned(),
        provider: None,
        model: None,
        attached_sessions: 1,
        home: "/Users/example".to_owned(),
        can_open_path: false,
    };
    description.validate().expect("valid description");
    let value = serde_json::to_value(&description).expect("serialize description");
    assert_eq!(value["protocolVersion"], 3);
    assert_eq!(value["product"]["version"], "dev-42");
    assert!(
        value["capabilities"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value == "session-journal")
    );

    let mut wrong = description.clone();
    wrong.protocol_version = 2;
    assert!(matches!(
        wrong.validate(),
        Err(MuxProtocolError::ProtocolVersion(2))
    ));
    let mut incomplete = description;
    incomplete
        .capabilities
        .remove(&SessionEndpointCapability::ActionableSync);
    assert!(matches!(
        incomplete.validate(),
        Err(MuxProtocolError::MissingCapabilities(_))
    ));
}

#[test]
fn each_logical_stream_is_baseline_first_and_generation_scoped() {
    let mut generation = SessionMuxGeneration::new(7).expect("generation");
    generation
        .open("workspace", &SessionStreamTarget::Workspace)
        .expect("open workspace");
    let delta = SessionSyncFrame::WorkspaceRemove {
        generation: 7,
        workspace_id: "workspace-1".to_owned(),
    };
    assert!(matches!(
        generation.accept("workspace", &delta),
        Err(MuxProtocolError::BaselineRequired(_))
    ));
    generation
        .accept(
            "workspace",
            &SessionSyncFrame::WorkspaceBaseline {
                generation: 7,
                baseline: WorkspaceBaseline {
                    items: Vec::new(),
                    archived_session_ids: Vec::new(),
                },
            },
        )
        .expect("baseline");
    generation.accept("workspace", &delta).expect("delta");
    assert!(matches!(
        generation.accept(
            "workspace",
            &SessionSyncFrame::WorkspaceBaseline {
                generation: 7,
                baseline: WorkspaceBaseline {
                    items: Vec::new(),
                    archived_session_ids: Vec::new(),
                },
            }
        ),
        Err(MuxProtocolError::DuplicateBaseline(_))
    ));
    assert!(matches!(
        generation.accept(
            "workspace",
            &SessionSyncFrame::WorkspaceRemove {
                generation: 8,
                workspace_id: "workspace-1".to_owned(),
            }
        ),
        Err(MuxProtocolError::StaleGeneration { .. })
    ));
}

#[test]
fn journal_follow_freezes_a_cut_before_live_events_and_pages_do_not_drift() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let first = journal
        .append_kernel(vec![1], "message-1", event("one", 1.0))
        .expect("first event");
    let second = journal
        .append_kernel(vec![2], "message-2", event("two", 2.0))
        .expect("second event");
    assert_eq!((first.seq, second.seq), (0, 1));

    let hub = EndpointSubscriptionHub::default();
    let follow = open_journal_follow(
        &hub,
        address(),
        &journal,
        data(r#"{"asOfSeq":1,"values":{}}"#),
        1,
        "follow-1",
    )
    .expect("follow");
    assert_eq!(follow.snapshot.through_sequence, 1);
    assert_eq!(follow.snapshot.entries.len(), 1);
    assert_eq!(follow.snapshot.entries[0].event.seq, 1);
    assert!(follow.snapshot.has_more_before);

    let third = journal
        .append_kernel(vec![3], "message-3", event("three", 3.0))
        .expect("third event");
    hub.publish_event(SESSION, &journal, "live-3", third, None)
        .expect("publish live");

    let older = frozen_history_page(&journal, 1, Some(1), 1).expect("frozen older page");
    assert_eq!(older.events.len(), 1);
    assert_eq!(older.events[0].event.seq, 0);
    assert!(!older.has_more);
    assert!(frozen_history_page(&journal, 1, Some(2), 1).is_err());
    assert!(frozen_history_page(&journal, 1, Some(-2), 1).is_err());
    let exhausted = frozen_history_page(&journal, 1, Some(-1), 1).expect("exhausted cursor");
    assert!(exhausted.events.is_empty());
    assert!(!exhausted.has_more);
}

#[test]
fn journal_validator_rejects_a_gap_after_the_snapshot() {
    let mut generation = SessionMuxGeneration::new(11).expect("generation");
    generation
        .open(
            "journal",
            &SessionStreamTarget::SessionJournal {
                address: address(),
                max_messages: 50,
            },
        )
        .expect("open journal");
    generation
        .accept(
            "journal",
            &SessionSyncFrame::JournalSnapshot {
                generation: 11,
                snapshot: endpoint::SessionJournalSnapshot {
                    window_limit: 50,
                    address: address(),
                    through_sequence: 4,
                    entries: vec![SessionHistoryEntry {
                        event: SessionEvent {
                            seq: 4,
                            ..event("four", 4.0)
                        },
                        view: None,
                    }],
                    has_more_before: true,
                    projections: data(r#"{"asOfSeq":4,"values":{}}"#),
                },
            },
        )
        .expect("snapshot");
    let gap = SessionSyncFrame::JournalEvent {
        generation: 11,
        address: address(),
        event: SessionEvent {
            seq: 6,
            ..event("six", 6.0)
        },
        view: None,
    };
    assert!(matches!(
        generation.accept("journal", &gap),
        Err(MuxProtocolError::JournalSequence {
            expected: 5,
            actual: 6
        })
    ));
}

#[test]
fn actionable_registry_replays_pending_and_revisioned_response_is_first_wins() {
    let registry = ActionableRegistry::default();
    let actionable = SessionActionable {
        id: "actionable-approval-1".to_owned(),
        session_id: SESSION.to_owned(),
        revision: 17,
        kind: SessionActionableKind::Approval,
        payload: data(r#"{"toolName":"shell"}"#),
    };
    registry.upsert(actionable.clone()).expect("insert");
    assert_eq!(registry.baseline().expect("baseline"), vec![actionable]);
    assert!(matches!(
        registry.resolve("actionable-approval-1", 16),
        Err(MuxProtocolError::StaleActionable {
            expected: 17,
            actual: 16,
            ..
        })
    ));
    let resolved = registry
        .resolve("actionable-approval-1", 17)
        .expect("resolve");
    assert_eq!(resolved.revision, 17);
    assert!(matches!(
        registry.resolve("actionable-approval-1", 17),
        Err(MuxProtocolError::UnknownActionable(_))
    ));
}

#[test]
fn canonical_fixture_is_rust_swift_interoperable_and_accepts_revision_zero() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/endpoint-mux/cases.canonical.json");
    let bytes = std::fs::read(path).expect("fixture");
    let value: serde_json::Value = serde_json::from_slice(&bytes).expect("fixture JSON");
    assert_eq!(
        serde_json_canonicalizer::to_vec(&value).expect("canonical fixture"),
        bytes.strip_suffix(b"\n").unwrap_or(&bytes)
    );

    let description: MuxHostDescription =
        serde_json::from_value(value["description"].clone()).expect("description");
    description.validate().expect("description contract");
    assert!(
        description
            .capabilities
            .contains(&SessionEndpointCapability::SessionControlSync)
    );

    for frame in [
        &value["workspace"]["open"],
        &value["journal"]["open"],
        &value["journal"]["page"],
        &value["actionable"]["client"],
    ] {
        let frame: SessionMuxClientFrame =
            serde_json::from_value(frame.clone()).expect("client frame");
        frame.validate().expect("client frame contract");
    }
    for frame in [
        &value["ready"],
        &value["workspace"]["baseline"],
        &value["journal"]["snapshot"],
        &value["actionable"]["server"],
    ] {
        let _: SessionMuxServerFrame = serde_json::from_value(frame.clone()).expect("server frame");
    }
    let workspace: SessionMuxServerFrame =
        serde_json::from_value(value["workspace"]["baseline"].clone()).expect("workspace");
    let SessionMuxServerFrame::Stream {
        frame: SessionSyncFrame::WorkspaceBaseline { baseline, .. },
        ..
    } = workspace
    else {
        panic!("expected workspace baseline")
    };
    assert_eq!(baseline.items[0].created_at, "2026-09-01T00:00:00.000Z");
}
