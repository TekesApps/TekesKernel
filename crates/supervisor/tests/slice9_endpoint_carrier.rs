use std::collections::{BTreeSet, HashSet};
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use endpoint::{
    CallContext, ClientRequest, ClientResponse, ClientResponseResult, DrainSignal,
    DurableHandoffSignal, EndpointCarrierHost, EndpointJournal, HostFailure, HostReadiness,
    PendingRequest, RequestFrameType, SessionAddress, SessionHostDescription,
    SessionMuxClientFrame, SessionStreamTarget, SessionSyncFrame,
};
use schema::{IJsonValue, OriginTuple};
use tekes_supervisor::endpoint_carrier::{
    LiveRespondAuthority, ProductionCarrierAssembly, ProductionCarrierStreams,
    SupervisorSessionAuthority,
};
use tekes_supervisor::endpoint_host::{
    ProductionEndpointHost, ProductionEndpointRoutes, ProductionRouteFailure,
    ProviderReadinessAuthority, QueueTransactionAuthority, RuntimeProviderReadiness,
    SessionDeliveryAuthority,
};
use transport::{BearerToken, ROUTE_REGISTRY, TransportConfig};

const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
const TIMESTAMP: &str = "2026-08-28T09:00:00.000Z";

struct ForkRoute;

impl ProductionEndpointRoutes for ForkRoute {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.fork".to_owned()].into_iter().collect()
    }

    fn execute(
        &self,
        _request: &endpoint::EndpointHostCall,
        _payload: &serde_json::Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        Err(route_failure())
    }
}

struct Providers;

impl ProviderReadinessAuthority for Providers {
    fn readiness(
        &self,
        _session_id: &str,
        _config: &profile::ConfigSnapshot,
    ) -> Result<Vec<RuntimeProviderReadiness>, ProductionRouteFailure> {
        Ok(Vec::new())
    }
}

struct Delivery;

impl SessionDeliveryAuthority for Delivery {
    fn prompt(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _prompt: &endpoint::MaterializedPrompt,
        _steer: bool,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        Err(route_failure())
    }

    fn cancel(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        Err(route_failure())
    }

    fn rename(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _title: &str,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        Err(route_failure())
    }
}

struct Queue;

impl QueueTransactionAuthority for Queue {
    fn execute(
        &self,
        _session_id: &str,
        _transaction: &worker_control::QueueTransaction,
    ) -> Result<worker_control::QueueTransactionResult, ProductionRouteFailure> {
        Err(route_failure())
    }
}

struct NoLiveWorker;

impl LiveRespondAuthority for NoLiveWorker {
    fn deliver_if_live(
        &self,
        _session_id: &str,
        _response: &worker_control::ApprovalResponse,
    ) -> Result<Option<worker_control::Receipt>, ProductionRouteFailure> {
        Ok(None)
    }
}

fn route_failure() -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        "test-unused",
        "unused test authority",
        IJsonValue::parse_str("{}").expect("details"),
    )
}

fn production_host(root: &std::path::Path) -> ProductionEndpointHost {
    ProductionEndpointHost::open_with_route_authority(
        root,
        SessionHostDescription {
            version: "0.1.0".to_owned(),
            cwd: "/workspace".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: "/Users/example".to_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok(TIMESTAMP.to_owned())),
        Arc::new(ForkRoute),
        Some(Arc::new(Providers)),
        Some(Arc::new(Delivery)),
        Some(Arc::new(Queue)),
    )
    .expect("production unary host")
}

fn assembly(root: &std::path::Path) -> ProductionCarrierAssembly {
    ProductionCarrierAssembly::assemble(
        root,
        production_host(root),
        Arc::new(NoLiveWorker),
        TransportConfig::loopback(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            BearerToken::new([7; 32]),
        ),
    )
    .expect("production carrier assembly")
}

#[test]
fn production_assembly_registers_only_mux_public_surface_and_gates_readiness_on_recovery() {
    let root = tempfile::tempdir().expect("root");
    let assembly = assembly(root.path());
    let host = assembly.host();
    assert_eq!(host.readiness(), HostReadiness::NotReady);
    assert_eq!(
        host.registered_methods(),
        ROUTE_REGISTRY
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>()
    );
    assembly.finish_recovery().expect("recovery gate");
    assert_eq!(host.readiness(), HostReadiness::Ready);
    assembly.begin_drain();
    assert_eq!(host.readiness(), HostReadiness::NotReady);
}

#[test]
fn production_mux_journal_is_follow_first_and_actionables_replay_by_stable_revision() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    let pending = PendingRequest::derive(
            SESSION,
            None,
            RequestFrameType::Approval,
            9,
            IJsonValue::parse_str(&format!(
                r#"{{"approvalId":"approval:ask1","callId":"ask1","sessionId":"{SESSION}","toolName":"ask_user","type":"approval/requested"}}"#
            ))
            .expect("request payload"),
        )
        .expect("derive request");

    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");
    let host = assembly.host();
    let description = host.mux_description().expect("V3 description");
    assert_eq!(description.protocol_version, 3);
    assert_eq!(description.product.name, "TekesKernel");

    let mut journal = block_on_ready(host.open_mux_stream(
        31,
        SessionStreamTarget::SessionJournal {
            address: endpoint::SessionAddress {
                session_id: SESSION.to_owned(),
            },
            max_messages: 50,
        },
    ))
    .expect("journal stream");
    let SessionSyncFrame::JournalSnapshot {
        generation,
        snapshot,
    } = block_on_ready(journal.recv())
        .expect("journal baseline item")
        .expect("journal baseline")
    else {
        panic!("expected journal snapshot")
    };
    assert_eq!(generation, 31);
    assert_eq!(snapshot.through_sequence, -1);

    let mut actionables =
        block_on_ready(host.open_mux_stream(31, SessionStreamTarget::Actionables))
            .expect("actionable stream");
    let SessionSyncFrame::ActionableBaseline { items, .. } = block_on_ready(actionables.recv())
        .expect("actionable baseline item")
        .expect("actionable baseline")
    else {
        panic!("expected actionable baseline")
    };
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, pending.rpc_id);
    assert_eq!(items[0].revision, 9);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(
            &items[0].payload.canonical_bytes().expect("payload")
        )
        .expect("payload JSON")["type"],
        "approval/requested"
    );

    let response = SessionMuxClientFrame::ActionableRespond {
        request_id: "respond-v3-1".to_owned(),
        actionable_id: pending.rpc_id.clone(),
        expected_revision: 9,
        outcome: IJsonValue::parse_str(
            r#"{"approvalId":"approval:ask1","outcome":"allowed-once"}"#,
        )
        .expect("outcome"),
    };
    assert_eq!(
        block_on_ready(host.mux_respond_actionable(response, call_context())).expect("V3 response"),
        9
    );
    let ledger =
        std::fs::read_to_string(root.path().join("threads").join(SESSION).join("main.jsonl"))
            .expect("semantic ledger");
    let last: serde_json::Value =
        serde_json::from_str(ledger.lines().last().expect("last event")).expect("event JSON");
    assert_eq!(last["kind"], "approval_response");
    // The durable response itself retires the request on the actionable
    // stream; no worker doorbell or explicit resolution frame is needed.
    let SessionSyncFrame::ActionableResolved { id, revision, .. } =
        block_on_ready(actionables.recv())
            .expect("actionable delta item")
            .expect("actionable delta")
    else {
        panic!("expected actionable resolved delta instead of a second baseline")
    };
    assert_eq!(id, pending.rpc_id);
    assert_eq!(revision, 9);
}

#[test]
fn corrupt_context_cache_does_not_block_controls_or_actionables() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    std::fs::write(
        root.path()
            .join("threads")
            .join(SESSION)
            .join("context-usage.projection.json"),
        b"broken",
    )
    .expect("corrupt cache");
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");
    let host = assembly.host();
    let mut controls =
        block_on_ready(host.open_mux_stream(31, SessionStreamTarget::SessionControl))
            .expect("control stream");
    assert!(matches!(
        block_on_ready(controls.recv())
            .expect("frame")
            .expect("baseline"),
        SessionSyncFrame::ControlBaseline { .. }
    ));
    let mut actionables =
        block_on_ready(host.open_mux_stream(32, SessionStreamTarget::Actionables))
            .expect("actionable stream");
    let SessionSyncFrame::ActionableBaseline { items, .. } = block_on_ready(actionables.recv())
        .expect("frame")
        .expect("baseline")
    else {
        panic!("actionables")
    };
    assert_eq!(items.len(), 1);
}

#[test]
fn context_control_baseline_restores_unknown_without_resetting_its_clock() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    let read = || {
        let assembly = assembly(root.path());
        assembly.finish_recovery().expect("recovery gate");
        let host = assembly.host();
        let mut stream =
            block_on_ready(host.open_mux_stream(31, SessionStreamTarget::SessionControl))
                .expect("control stream");
        let SessionSyncFrame::ControlBaseline { items, .. } = block_on_ready(stream.recv())
            .expect("frame")
            .expect("baseline")
        else {
            panic!("expected control baseline")
        };
        let item = items
            .iter()
            .find(|item| item.session_id == SESSION)
            .expect("session");
        serde_json::from_slice::<serde_json::Value>(
            &item.projections.canonical_bytes().expect("JSON"),
        )
        .expect("projection")
    };
    let first = read();
    assert_eq!(first["values"]["contextUsage"]["basis"], "unknown");
    assert!(first["values"]["contextUsage"]["usedTokens"].is_null());
    let details = &first["values"]["contextDetails"];
    assert_eq!(details["schemaVersion"], 1);
    assert_eq!(details["usage"], first["values"]["contextUsage"]);
    assert_eq!(details["coverage"], "unknown");
    assert!(details["rows"].as_array().expect("rows").is_empty());
    assert!(details["unavailableReason"].as_str().is_some());
    assert_eq!(read(), first);
}

#[test]
fn context_control_reconnect_restores_valid_pair_and_replaces_it_after_compaction() {
    use serde_json::json;
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    profile::ConfigRepository::open(root.path()).expect("config layout");
    std::fs::create_dir_all(root.path().join("workspaces/ws")).unwrap();
    std::fs::create_dir_all(root.path().join("workspace")).unwrap();
    std::fs::write(
        root.path().join("config/providers.json"),
        include_bytes!("../../../fixtures/config/providers.canonical.json"),
    )
    .unwrap();
    std::fs::write(
        root.path().join("config/settings.json"),
        include_bytes!("../../../fixtures/config/settings.canonical.json"),
    )
    .unwrap();
    let mut workspace = serde_json_canonicalizer::to_vec(&json!({
        "format":1,"revision":1,"id":"ws","name":"ws",
        "cwd":[std::fs::canonicalize(root.path().join("workspace")).unwrap()],
        "policy":{"network":true}
    }))
    .unwrap();
    workspace.push(b'\n');
    std::fs::write(root.path().join("workspaces/ws/workspace.json"), workspace).unwrap();
    let config = endpoint::NativeEndpoint::open(root.path())
        .unwrap()
        .session_config_snapshot(SESSION)
        .unwrap();
    let folder = root.path().join("threads").join(SESSION);
    let assets = store::AssetStore::new(folder.join("assets")).unwrap();
    let system = assets.publish(b"synthetic system").unwrap();
    let tools = assets.publish(b"[]").unwrap();
    let request = assets.publish(br#"{"model":"gpt-5","instructions":"synthetic system","input":[{"role":"user","content":"hello"}],"tools":[]}"#).unwrap();
    let mut events = std::fs::read_to_string(folder.join("main.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    events[2]["config_digest"] = json!(config.digest().unwrap());
    events[4]["adapter"] = json!("openai_responses_v1");
    events[4]["model"] = json!("gpt-5");
    events[4]["system"] = json!({"asset":system.asset,"digest":"s"});
    events[4]["tools"] = json!({"asset":tools.asset,"digest":"t"});
    events[5]["request"] = json!({"asset":request.asset,"bytes":request.bytes});
    events[6]["sealed"]["adapter"] = json!("openai_responses_v1");
    let write_events = |events: &[serde_json::Value]| {
        let lines = events
            .iter()
            .map(|event| serde_json::to_string(event).unwrap())
            .collect::<Vec<_>>();
        std::fs::write(folder.join("main.jsonl"), lines.join("\n") + "\n").unwrap();
    };
    write_events(&events);
    let read = || {
        let assembly = assembly(root.path());
        assembly.finish_recovery().unwrap();
        let mut stream = block_on_ready(
            assembly
                .host()
                .open_mux_stream(32, SessionStreamTarget::SessionControl),
        )
        .unwrap();
        let SessionSyncFrame::ControlBaseline { items, .. } =
            block_on_ready(stream.recv()).unwrap().unwrap()
        else {
            panic!("control baseline")
        };
        serde_json::from_slice::<serde_json::Value>(
            &items
                .iter()
                .find(|item| item.session_id == SESSION)
                .unwrap()
                .projections
                .canonical_bytes()
                .unwrap(),
        )
        .unwrap()
    };
    let valid = read();
    assert_eq!(valid["values"]["contextUsage"]["basis"], "upperBound");
    assert_eq!(
        valid["values"]["contextDetails"]["usage"],
        valid["values"]["contextUsage"]
    );
    assert_eq!(
        valid["values"]["contextDetails"]["evidence"]["requestID"],
        "a1"
    );
    assert_eq!(
        valid["values"]["contextDetails"]["rows"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(read(), valid);
    events.push(json!({"v":1,"seq":10,"kind":"compact","ts":TIMESTAMP,
        "covers":[{"from":2,"to":9}],"summary":"synthetic summary"}));
    write_events(&events);
    let unknown = read();
    assert!(unknown["asOfSeq"].as_u64().unwrap() > valid["asOfSeq"].as_u64().unwrap());
    assert_eq!(unknown["values"]["contextUsage"]["basis"], "unknown");
    assert_eq!(
        unknown["values"]["contextDetails"]["usage"],
        unknown["values"]["contextUsage"]
    );
    assert!(
        unknown["values"]["contextDetails"]["rows"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_ne!(
        unknown["values"]["contextDetails"]["revision"],
        valid["values"]["contextDetails"]["revision"]
    );
    assert_eq!(read(), unknown);
}

#[test]
fn production_carrier_rejects_structurally_invalid_route_payload_before_rpc_identity() {
    let root = tempfile::tempdir().expect("root");
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");
    let request = ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: "rpc-invalid-workspace-create".to_owned(),
        method: "workspace.create".to_owned(),
        payload: IJsonValue::parse_str(r#"{"unexpected":true}"#).expect("valid outer payload"),
    };
    let failure = block_on_ready(assembly.host().unary(request, call_context()))
        .expect_err("closed route payload must fail before dispatch");
    assert!(matches!(failure, HostFailure::InvalidRequest));
    assert_eq!(
        std::fs::read_dir(root.path().join("endpoint-management/rpc"))
            .expect("rpc registry")
            .count(),
        0
    );
}

#[test]
fn production_create_history_then_stream_append_is_durable_and_live() {
    let root = tempfile::tempdir().expect("root");
    let project = root.path().join("project");
    std::fs::create_dir(&project).expect("project");
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");

    let workspace = block_on_ready(assembly.host().unary(
        client_request(
            "rpc-live-workspace",
            "workspace.create",
            &format!(
                r#"{{"path":{}}}"#,
                serde_json::to_string(project.to_str().expect("UTF-8 project")).expect("path JSON")
            ),
        ),
        call_context(),
    ))
    .expect("workspace create");
    let workspace_value = response_value(&workspace);
    let workspace_id = workspace_value["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id");
    block_on_ready(assembly.host().unary(
        client_request(
            "rpc-live-session",
            "session.create",
            &format!(r#"{{"sessionId":"{SESSION}","workspaceId":"{workspace_id}"}}"#),
        ),
        call_context(),
    ))
    .expect("session create");
    let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        assembly.streams(),
        endpoint::StreamChannel::Mux,
    ))
    .expect("production mux");
    let baseline = block_on_ready(mux.recv())
        .expect("baseline item")
        .expect("baseline frame");
    assert_eq!(baseline.method, "session/subscribed");
    let mut journal_mux = block_on_ready(assembly.host().open_mux_stream(
        41,
        SessionStreamTarget::SessionJournal {
            address: SessionAddress {
                session_id: SESSION.to_owned(),
            },
            max_messages: 50,
        },
    ))
    .expect("V3 journal");
    let SessionSyncFrame::JournalSnapshot { snapshot, .. } = block_on_ready(journal_mux.recv())
        .expect("V3 snapshot item")
        .expect("V3 snapshot")
    else {
        panic!("expected V3 journal snapshot")
    };
    assert!(snapshot.entries.is_empty());

    let origin = OriginTuple {
        principal: "user".to_owned(),
        client: "slice9-carrier-test".to_owned(),
        target: SESSION.to_owned(),
        op: "session.prompt".to_owned(),
        key: "prompt-live".to_owned(),
    };
    let native = endpoint::NativeEndpoint::open(root.path()).expect("native endpoint");
    native
        .prompt_for_endpoint(
            SESSION,
            TIMESTAMP,
            &origin,
            IJsonValue::parse_str(r#"[{"text":"hello","type":"text"}]"#).expect("prompt content"),
            false,
        )
        .expect("prompt append");
    // The journal is a projection of the ledger: open the prompted turn on the
    // ledger, project its first durable row and publish it the way the
    // doorbell path does.
    let folder = root.path().join("threads").join(SESSION);
    {
        let mut ledger = store::LockedLedger::open(folder.join("main.jsonl"), 1).expect("ledger");
        let input_seq = ledger.next_seq() - 1;
        for line in [
            format!(
                r#"{{"binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":{},"ts":"{TIMESTAMP}","v":1}}"#,
                input_seq + 1
            ),
            format!(
                r#"{{"kind":"turn_open","seq":{},"trigger":{{"inputs":[{input_seq}]}},"ts":"{TIMESTAMP}","turn":1,"v":1}}"#,
                input_seq + 2
            ),
        ] {
            let event = schema::Event::decode(line.as_bytes()).expect("turn event");
            ledger
                .append_contract(event, store::BarrierContext::default())
                .expect("append turn");
        }
    }
    let journal = EndpointJournal::open(&folder).expect("endpoint journal");
    let ledger = std::fs::read(folder.join("main.jsonl")).expect("ledger bytes");
    let projection = schema::validate_ledger(&ledger, 1).expect("held session ledger");
    let mut projected = endpoint::Projector::default()
        .reconcile(&projection.events, &journal)
        .expect("project held session");
    // The native endpoint projects its own writes; take the first durable row.
    let event = if projected.is_empty() {
        journal
            .event(0)
            .expect("journal read")
            .expect("first projected row")
    } else {
        projected.remove(0)
    };
    let published = assembly
        .streams()
        .publish_durable_session_event_from_journal(SESSION, &journal, event, None)
        .expect("live publish");
    drop(journal);
    assert_eq!(published, 2);
    let live = block_on_ready(mux.recv())
        .expect("live item")
        .expect("live frame");
    assert_eq!(live.method, "session/event");
    let SessionSyncFrame::JournalEvent { event, .. } = block_on_ready(journal_mux.recv())
        .expect("V3 live item")
        .expect("V3 live event")
    else {
        panic!("expected V3 journal event")
    };
    assert_eq!(event.seq, 0);

    native
        .rename_session_for_endpoint(
            SESSION,
            TIMESTAMP,
            &OriginTuple {
                op: "session.rename".to_owned(),
                key: "rename-live".to_owned(),
                ..origin
            },
            "Renamed",
        )
        .expect("rename append");
    let store = store::ThreadStore::open(root.path()).expect("store");
    store.archive(SESSION).expect("archive");
    assembly
        .streams()
        .detach_for_archive(SESSION)
        .expect("detach archived session");
    store.unarchive(SESSION).expect("unarchive");
    assembly
        .streams()
        .attach_session(SESSION)
        .expect("reattach unarchived session");
    let fresh = block_on_ready(mux.recv())
        .expect("reattached item")
        .expect("reattached frame");
    assert_eq!(fresh.method, "session/subscribed");
}

#[test]
fn production_respond_uses_shared_admission_and_durable_locked_append_handoff() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    let pending = PendingRequest::derive(
            SESSION,
            None,
            RequestFrameType::Approval,
            9,
            IJsonValue::parse_str(&format!(
                r#"{{"approvalId":"approval:ask1","callId":"ask1","sessionId":"{SESSION}","toolName":"ask_user","type":"approval/requested"}}"#
            ))
            .expect("request payload"),
        )
        .expect("derive request");

    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");
    let response = ClientResponse {
        envelope_type: "client-response".to_owned(),
        rpc_id: pending.rpc_id,
        result: ClientResponseResult {
            ok: true,
            value: Some(
                IJsonValue::parse_str(&format!(
                    r#"{{"approvalId":"approval:ask1","outcome":"allowed-once","sessionId":"{SESSION}"}}"#
                ))
                .expect("respond value"),
            ),
            error: None,
        },
    };
    let receipt =
        block_on_ready(assembly.host().respond(response, call_context())).expect("respond");
    assert!(receipt.accepted);
    let ledger =
        std::fs::read_to_string(root.path().join("threads").join(SESSION).join("main.jsonl"))
            .expect("semantic ledger");
    let last: serde_json::Value =
        serde_json::from_str(ledger.lines().last().expect("last event")).expect("event JSON");
    assert_eq!(last["kind"], "approval_response");
    assert_eq!(last["call"], "ask1");
    assert_eq!(last["seq"], 10);
    assert_eq!(last["origin_tuple"]["op"], "respond");
}

#[test]
fn singular_workflow_question_publishes_and_recovers_its_answer() {
    for options in [serde_json::Value::Null, serde_json::json!(["A", "B"])] {
        let root = tempfile::tempdir().unwrap();
        write_held_session(root.path());
        let folder = root.path().join("threads").join(SESSION);
        let path = folder.join("main.jsonl");
        let mut events: Vec<serde_json::Value> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let invocation = serde_json::json!({"question":"Which option?", "options":options});
        events[7]["name"] = serde_json::json!("ask_user_questions");
        events[7]["args"] = invocation.clone();
        events[8]["question"] = invocation.clone();
        let bytes = events
            .iter()
            .map(|row| {
                String::from_utf8(serde_json_canonicalizer::to_vec(row).unwrap()).unwrap() + "\n"
            })
            .collect::<String>();
        std::fs::write(&path, &bytes).unwrap();
        let assembly = assembly(root.path());
        assembly.finish_recovery().unwrap();
        let mut stream = block_on_ready(
            assembly
                .host()
                .open_mux_stream(78, SessionStreamTarget::Actionables),
        )
        .unwrap();
        let SessionSyncFrame::ActionableBaseline { items, .. } =
            block_on_ready(stream.recv()).unwrap().unwrap()
        else {
            panic!("baseline")
        };
        assert_eq!(items.len(), 1);
        let payload: serde_json::Value =
            serde_json::from_slice(&items[0].payload.canonical_bytes().unwrap()).unwrap();
        assert_eq!(payload["type"], "question/requested");
        let labels = options
            .as_array()
            .map(|rows| {
                rows.iter()
                    .map(|label| serde_json::json!({"label": label}))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        assert_eq!(
            payload["questions"],
            serde_json::json!([{
                "id": "ask1",
                "question": "Which option?",
                "options": labels,
                "multiSelect": false,
                "allowCustom": true,
            }])
        );
        let answer = serde_json::json!({"answers":[{"id":"ask1","selected":["A"],"custom":""}]});
        let response = ClientResponse {
            envelope_type: "client-response".into(),
            rpc_id: items[0].id.clone(),
            result: ClientResponseResult {
                ok: true,
                error: None,
                value: Some(
                    IJsonValue::parse(
                        &serde_json::to_vec(
                            &serde_json::json!({"sessionId":SESSION,"answer":answer}),
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
            },
        };
        assert!(
            block_on_ready(assembly.host().respond(response, call_context()))
                .unwrap()
                .accepted
        );
        let written = std::fs::read_to_string(&path).unwrap();
        assert!(written.starts_with(&bytes));
        let last: serde_json::Value =
            serde_json::from_str(written.lines().last().unwrap()).unwrap();
        assert_eq!(last["kind"], "approval_response");
        assert_eq!(last["call"], "ask1");
        assert_eq!(last["answer"], answer);
        // The ledger's approval_response is the resolution: the actionable is gone.
        let mut stream = block_on_ready(
            assembly
                .host()
                .open_mux_stream(79, SessionStreamTarget::Actionables),
        )
        .unwrap();
        let SessionSyncFrame::ActionableBaseline { items: after, .. } =
            block_on_ready(stream.recv()).unwrap().unwrap()
        else {
            panic!("baseline")
        };
        assert!(after.is_empty(), "answered hold must not be offered again");
        let mut reopened = block_on_ready(
            assembly
                .host()
                .open_mux_stream(79, SessionStreamTarget::Actionables),
        )
        .unwrap();
        let SessionSyncFrame::ActionableBaseline { items, .. } =
            block_on_ready(reopened.recv()).unwrap().unwrap()
        else {
            panic!("baseline")
        };
        assert!(items.is_empty(), "answered hold must not be offered again");
    }
}

#[test]
fn child_respond_binds_parent_spawn_and_writes_only_child_ledger() {
    const CHILD: &str = "018f0000-0000-7000-8000-000000000004";
    for invalid in ["", "spawn", "sequence", "path", "identity", "cycle"] {
        let root = tempfile::tempdir().unwrap();
        write_held_session(root.path());
        let folder = root.path().join("threads").join(SESSION);
        let original = std::fs::read_to_string(folder.join("main.jsonl")).unwrap();
        let mut child: Vec<serde_json::Value> = original
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        child[0]["thread"] = serde_json::json!(CHILD);
        // Production permission holds contain metadata in `question`; this
        // must not turn an edit approval into an answer workflow.
        child[8]["scope"] = serde_json::json!("edit");
        child[8]["question"] = serde_json::json!({"call":"ask1","class":"edit","tool":"ask_user"});
        child[0]["parent"] = serde_json::json!({"file":"main.jsonl","seq":10,"spawn_id":if invalid == "spawn" {"forged"} else {"spawn-child"}});
        child[0]["seed"] = serde_json::json!({"source":"parent","kinds":[],"snapshot":{"asset":format!("sha256-{}", "0".repeat(64)),"digest":"0".repeat(64)}});
        let child_file = format!("{CHILD}.jsonl");
        match invalid {
            "sequence" => child[0]["parent"]["seq"] = serde_json::json!(9),
            "path" => child[0]["parent"]["file"] = serde_json::json!("../main.jsonl"),
            "identity" => child[0]["thread"] = serde_json::json!(SESSION),
            "cycle" => child[0]["parent"]["file"] = serde_json::json!(child_file),
            _ => {}
        }
        let child_bytes = child
            .iter()
            .map(|row| {
                String::from_utf8(serde_json_canonicalizer::to_vec(row).unwrap()).unwrap() + "\n"
            })
            .collect::<String>();
        std::fs::write(folder.join(&child_file), &child_bytes).unwrap();
        let spawn = serde_json::json!({"v":1,"seq":10,"kind":"spawn","ts":TIMESTAMP,"turn":1,"child":child_file,"call":"ask1","spawn_id":"spawn-child","resume":"never","seed":{"kinds":[]}});
        let parent_bytes = original
            + &String::from_utf8(serde_json_canonicalizer::to_vec(&spawn).unwrap()).unwrap()
            + "\n";
        std::fs::write(folder.join("main.jsonl"), &parent_bytes).unwrap();
        let assembly = assembly(root.path());
        assembly.finish_recovery().unwrap();
        let pending = if invalid.is_empty() {
            let mut stream = block_on_ready(
                assembly
                    .host()
                    .open_mux_stream(77, SessionStreamTarget::Actionables),
            )
            .unwrap();
            let SessionSyncFrame::ActionableBaseline { items, .. } =
                block_on_ready(stream.recv()).unwrap().unwrap()
            else {
                panic!("baseline")
            };
            assert_eq!(items.len(), 2, "root and child holds must both publish");
            let item = items
                .iter()
                .find(|item| {
                    serde_json::from_slice::<serde_json::Value>(
                        &item.payload.canonical_bytes().unwrap(),
                    )
                    .unwrap()["approvalId"]
                        == format!("approval:{child_file}:ask1")
                })
                .expect("automatic child publication");
            PendingRequest::derive(
                SESSION,
                Some(&child_file),
                RequestFrameType::Approval,
                item.revision,
                item.payload.clone(),
            )
            .unwrap()
        } else {
            PendingRequest::derive(SESSION, Some(&child_file), RequestFrameType::Approval, 9,
            IJsonValue::parse_str(&format!(r#"{{"approvalId":"approval:ask1","callId":"ask1","sessionId":"{SESSION}","toolName":"ask_user","type":"approval/requested"}}"#)).unwrap()).unwrap()
        };
        let approval_id = serde_json::from_slice::<serde_json::Value>(
            &pending.envelope.payload.canonical_bytes().unwrap(),
        )
        .unwrap()["approvalId"]
            .as_str()
            .unwrap()
            .to_owned();
        let response = ClientResponse { envelope_type:"client-response".into(), rpc_id:pending.rpc_id.clone(), result:ClientResponseResult {
            ok:true, error:None, value:Some(IJsonValue::parse_str(&format!(r#"{{"approvalId":"{approval_id}","outcome":"allowed-once","sessionId":"{SESSION}"}}"#)).unwrap()),
        }};
        let result = block_on_ready(assembly.host().respond(response, call_context()));
        assert_eq!(
            std::fs::read_to_string(folder.join("main.jsonl")).unwrap(),
            parent_bytes
        );
        if !invalid.is_empty() {
            assert!(result.is_err(), "forged ancestry must fail closed");
            assert_eq!(
                std::fs::read_to_string(folder.join(&child_file)).unwrap(),
                child_bytes
            );
        } else {
            assert!(result.expect("child respond").accepted);
            let bytes = std::fs::read_to_string(folder.join(&child_file)).unwrap();
            let last: serde_json::Value =
                serde_json::from_str(bytes.lines().last().unwrap()).unwrap();
            assert_eq!(last["kind"], "approval_response");
            assert_eq!(last["call"], "ask1");
            assert_eq!(last["seq"], 10);
        }
    }
}

#[test]
fn carrier_stream_hooks_attach_detach_and_emit_inventory_invalidations() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    let streams = ProductionCarrierStreams::new(root.path());
    let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        &streams,
        endpoint::StreamChannel::Mux,
    ))
    .expect("mux");
    let baseline = block_on_ready(mux.recv())
        .expect("baseline item")
        .expect("baseline frame");
    assert_eq!(baseline.method, "session/subscribed");
    // The held session's pending request is derived from its ledger and
    // replayed right after the baseline.
    let replayed = block_on_ready(mux.recv())
        .expect("replayed item")
        .expect("replayed frame");
    assert_eq!(replayed.method, "approval/requested");

    let mut host = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        &streams,
        endpoint::StreamChannel::Host,
    ))
    .expect("host");
    store::ThreadStore::open(root.path())
        .expect("store")
        .archive(SESSION)
        .expect("archive");
    streams.detach_for_archive(SESSION).expect("detach");
    let removed = block_on_ready(host.recv())
        .expect("removed item")
        .expect("removed frame");
    let archived = block_on_ready(host.recv())
        .expect("archived item")
        .expect("archived frame");
    assert_eq!(removed.method, "host/session-removed");
    assert_eq!(archived.method, "host/archived-sessions-changed");

    store::ThreadStore::open(root.path())
        .expect("store")
        .unarchive(SESSION)
        .expect("unarchive");
    streams.attach_session(SESSION).expect("fresh attach");
    let fresh = block_on_ready(mux.recv())
        .expect("fresh baseline item")
        .expect("fresh baseline frame");
    assert_eq!(fresh.method, "session/subscribed");
    let added = block_on_ready(host.recv())
        .expect("added item")
        .expect("added frame");
    assert_eq!(added.method, "host/session-added");
}

#[derive(Default)]
struct LiveSessions(Mutex<HashSet<String>>);

impl SupervisorSessionAuthority for LiveSessions {
    fn live_sessions(&self) -> HashSet<String> {
        self.0.lock().expect("live sessions").clone()
    }

    fn reconcile_projection(&self, _session_id: &str) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn session_status_frames_refresh_inventory_running_from_the_process_table() {
    let root = tempfile::tempdir().expect("root");
    write_held_session(root.path());
    std::fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
    let mut workspace = serde_json_canonicalizer::to_vec(&serde_json::json!({
        "format":1,"revision":1,"id":"ws","name":"ws",
        "cwd":[root.path().join("workspace").to_string_lossy()],
        "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
    }))
    .expect("canonical workspace JSON");
    workspace.push(b'\n');
    std::fs::write(root.path().join("workspaces/ws/workspace.json"), workspace)
        .expect("workspace config");
    let live = Arc::new(LiveSessions::default());
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("recovery gate");
    let streams = assembly.streams().clone();
    streams.attach_session_authority(
        Arc::downgrade(&live) as std::sync::Weak<dyn SupervisorSessionAuthority>
    );
    let host = assembly.host();

    let mut inventory =
        block_on_ready(host.open_mux_stream(41, SessionStreamTarget::SessionInventory))
            .expect("inventory stream");
    let SessionSyncFrame::InventoryBaseline { items, .. } = block_on_ready(inventory.recv())
        .expect("inventory baseline item")
        .expect("inventory baseline")
    else {
        panic!("expected inventory baseline")
    };
    assert_eq!(items.len(), 1);
    assert!(
        !items[0].running,
        "no live worker and a free line lock is not running"
    );
    assert_eq!(
        items[0].tail, "parked_hold",
        "an unanswered approval hold with no worker is parked, not running"
    );

    live.0
        .lock()
        .expect("live sessions")
        .insert(SESSION.to_owned());
    streams
        .publish_session_status(SESSION, true)
        .expect("publish worker start");
    let SessionSyncFrame::InventoryUpsert { session, .. } = block_on_ready(inventory.recv())
        .expect("running upsert item")
        .expect("running upsert")
    else {
        panic!("expected an inventory upsert after a worker start")
    };
    assert_eq!(session.session_id, SESSION);
    assert!(
        session.running,
        "a registered live worker counts as running"
    );
    assert_eq!(session.tail, "running");

    live.0.lock().expect("live sessions").clear();
    streams
        .publish_session_status(SESSION, false)
        .expect("publish worker exit");
    let SessionSyncFrame::InventoryUpsert { session, .. } = block_on_ready(inventory.recv())
        .expect("idle upsert item")
        .expect("idle upsert")
    else {
        panic!("expected an inventory upsert after a worker exit")
    };
    assert!(!session.running, "running must follow the worker exit");
    assert_eq!(session.tail, "parked_hold");
}

fn write_held_session(root: &std::path::Path) {
    store::ThreadStore::open(root).expect("store layout");
    let folder = root.join("threads").join(SESSION);
    std::fs::create_dir(&folder).expect("session folder");
    std::fs::create_dir(folder.join("assets")).expect("assets");
    let lines = [
        format!(
            r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{{"client":"cli","key":"create","op":"create","principal":"p","target":"{SESSION}"}},"resume":"never","seq":1,"thread":"{SESSION}","ts":"{TIMESTAMP}","v":1,"workspace":"ws"}}"#
        ),
        format!(
            r#"{{"content":[{{"text":"start","type":"text"}}],"kind":"input","origin_key":"i1","origin_tuple":{{"client":"cli","key":"i1","op":"submit","principal":"p","target":"{SESSION}"}},"seq":2,"ts":"{TIMESTAMP}","v":1}}"#
        ),
        format!(
            r#"{{"binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"{TIMESTAMP}","v":1}}"#
        ),
        format!(
            r#"{{"kind":"turn_open","seq":4,"trigger":{{"inputs":[2]}},"ts":"{TIMESTAMP}","turn":1,"v":1}}"#
        ),
        format!(
            r#"{{"adapter":"fake","id":"e1","kind":"epoch","model":"fake","reason":"initial","renderer":1,"seq":5,"system":{{"asset":"sha256-aa","digest":"d1"}},"tools":{{"asset":"sha256-td","digest":"td"}},"ts":"{TIMESTAMP}","v":1}}"#
        ),
        format!(
            r#"{{"admits":[{{"from":2,"to":2}}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096}},"seq":6,"ts":"{TIMESTAMP}","turn":1,"v":1,"wire_digest":"wd"}}"#
        ),
        format!(
            r#"{{"attempt":"a1","content":[{{"args":{{"question":"continue?"}},"call":"ask1","name":"ask_user","type":"tool-call"}}],"kind":"output","sealed":{{"adapter":"fake","fragments":"ask","version":1}},"seq":7,"ts":"{TIMESTAMP}","turn":1,"usage":{{"availability":"reported","input_tokens":"5","output_tokens":"1"}},"v":1}}"#
        ),
        format!(
            r#"{{"args":{{"question":"continue?"}},"attempt":"a1","call":"ask1","kind":"tool_call","name":"ask_user","seq":8,"source":"provider","ts":"{TIMESTAMP}","turn":1,"v":1}}"#
        ),
        format!(
            r#"{{"call":"ask1","kind":"approval_request","scope":"answer","seq":9,"ts":"{TIMESTAMP}","turn":1,"v":1}}"#
        ),
    ];
    std::fs::write(folder.join("main.jsonl"), lines.join("\n") + "\n").expect("ledger");
}

fn block_on_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("future unexpectedly pending"),
    }
}

fn call_context() -> CallContext {
    CallContext {
        response_deadline: Instant::now() + Duration::from_secs(1),
        drain: DrainSignal::new(Arc::new(AtomicBool::new(false))),
        handoff: DurableHandoffSignal::new(),
    }
}

fn client_request(rpc_id: &str, method: &str, payload: &str) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: method.to_owned(),
        payload: IJsonValue::parse_str(payload).expect("request payload"),
    }
}

fn response_value(response: &endpoint::ServerResponse) -> serde_json::Value {
    serde_json::from_slice(
        &response
            .result
            .value
            .as_ref()
            .expect("successful response value")
            .canonical_bytes()
            .expect("canonical response value"),
    )
    .expect("response JSON")
}
