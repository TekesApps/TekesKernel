//! S4 (G-BOOT-1): respond lookup through the request index, baselines from cards and
//! persisted projections, lazy mux registration with its 256-session limit. These tests read
//! the process-wide decode counters, so they live in their own test binary and run one at a
//! time (`SERIAL`).

static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

use std::collections::{BTreeSet, HashSet};
use std::future::Future;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use endpoint::{
    CallContext, ClientResponse, ClientResponseResult, DrainSignal, DurableHandoffSignal,
    EndpointCarrierHost, HostFailure, SessionAddress, SessionHostDescription, SessionStreamTarget,
    SessionSyncFrame,
};
use schema::{IJsonValue, OriginTuple};
use tekes_supervisor::endpoint_carrier::{
    LiveRespondAuthority, ProductionCarrierAssembly, SupervisorSessionAuthority,
};
use tekes_supervisor::endpoint_host::{
    ProductionEndpointHost, ProductionEndpointRoutes, ProductionRouteFailure,
    ProviderReadinessAuthority, QueueTransactionAuthority, RuntimeProviderReadiness,
    SessionDeliveryAuthority,
};
use transport::{BearerToken, TransportConfig};

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

fn write_session_at(root: &std::path::Path, session: &str, held: bool) {
    store::ThreadStore::open(root).expect("store layout");
    let folder = root.join("threads").join(session);
    std::fs::create_dir(&folder).expect("session folder");
    std::fs::create_dir(folder.join("assets")).expect("assets");
    let lines = [
        format!(
            r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{{"client":"cli","key":"create","op":"create","principal":"p","target":"{session}"}},"resume":"never","seq":1,"thread":"{session}","ts":"{TIMESTAMP}","v":1,"workspace":"ws"}}"#
        ),
        format!(
            r#"{{"content":[{{"text":"start","type":"text"}}],"kind":"input","origin_key":"i1","origin_tuple":{{"client":"cli","key":"i1","op":"submit","principal":"p","target":"{session}"}},"seq":2,"ts":"{TIMESTAMP}","v":1}}"#
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
    let keep = if held { lines.len() } else { 2 };
    std::fs::write(folder.join("main.jsonl"), lines[..keep].join("\n") + "\n").expect("ledger");
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

fn blank_id(n: usize) -> String {
    format!("018f0000-0000-7000-8000-{n:012}")
}

fn approval_rpc(session: &str) -> String {
    endpoint::derive_line_request_rpc_id(session, None, endpoint::RequestFrameType::Approval, 9)
}

fn allow(rpc_id: &str, session: &str) -> ClientResponse {
    ClientResponse {
        envelope_type: "client-response".to_owned(),
        rpc_id: rpc_id.to_owned(),
        result: ClientResponseResult {
            ok: true,
            value: Some(
                IJsonValue::parse_str(&format!(
                    r#"{{"approvalId":"approval:ask1","outcome":"allowed-once","sessionId":"{session}"}}"#
                ))
                .expect("respond value"),
            ),
            error: None,
        },
    }
}

fn respond(
    assembly: &ProductionCarrierAssembly,
    rpc_id: &str,
    session: &str,
) -> Result<endpoint::RespondReceipt, HostFailure> {
    block_on_ready(
        assembly
            .host()
            .respond(allow(rpc_id, session), call_context()),
    )
}

fn decodes_since(before: schema::metrics::WorkCounters) -> u64 {
    schema::metrics::snapshot().since(before).ledger_decodes
}

/// T19(a): with 150 other threads on disk, answering one request decodes only the thread that
/// holds it; a retry is answered from the registry; an unknown id reads no ledger and is
/// remembered as unknown.
#[test]
fn respond_decodes_only_the_thread_that_holds_the_request() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    let target = blank_id(1);
    write_session_at(root.path(), &target, true);
    for n in 2..152 {
        write_session_at(root.path(), &blank_id(n), false);
    }
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("ready");
    let before = schema::metrics::snapshot();
    let receipt = respond(&assembly, &approval_rpc(&target), &target).expect("respond");
    assert!(receipt.accepted);
    let used = decodes_since(before);
    assert!(
        used < 150,
        "answering one request decoded {used} records (the other 150 threads hold 300)"
    );
    // a retry of the applied respond is answered, not refused as unknown
    let retry = respond(&assembly, &approval_rpc(&target), &target).expect("retry");
    assert!(retry.accepted, "{retry:?}");
    // an unknown id: no ledger decoded, and remembered
    let before = schema::metrics::snapshot();
    let unknown = respond(&assembly, "request-unknown", &target).expect("unknown");
    assert!(!unknown.accepted);
    assert_eq!(unknown.reason.as_deref(), Some("unknown-rpc-id"));
    // only the thread that changed (it was answered) is looked at again; the 150 others are not
    assert!(
        decodes_since(before) <= 12,
        "decoded {}",
        decodes_since(before)
    );
    let before = schema::metrics::snapshot();
    let again = respond(&assembly, "request-unknown", &target).expect("unknown again");
    assert!(!again.accepted);
    assert_eq!(decodes_since(before), 0, "remembered as unknown");
    // another unknown id: every thread was examined at its current bytes, nothing is read
    let other = respond(&assembly, "request-unknown-2", &target).expect("another unknown");
    assert!(!other.accepted);
    assert_eq!(
        decodes_since(before),
        0,
        "examined threads are not read again"
    );
}

/// T19(b)/(h): a thread with a torn tail answers its own request (the append repairs the
/// tail), a thread with a complete invalid record fails only its own request, and neither
/// stops a respond to a healthy thread.
#[test]
fn damaged_threads_do_not_fail_a_respond_to_another() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    let (healthy, torn, invalid) = (blank_id(11), blank_id(12), blank_id(13));
    for session in [&healthy, &torn, &invalid] {
        write_session_at(root.path(), session, true);
    }
    let append = |session: &str, bytes: &[u8]| {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .append(true)
            .open(root.path().join("threads").join(session).join("main.jsonl"))
            .expect("ledger")
            .write_all(bytes)
            .expect("append");
    };
    append(&torn, b"{\"kind\":\"inp");
    append(&invalid, b"{\"not\":\"a record\"}\n");
    let invalid_before = std::fs::read(
        root.path()
            .join("threads")
            .join(&invalid)
            .join("main.jsonl"),
    )
    .expect("invalid ledger");
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("ready");
    // the process host is the authority that verifies a thread before it is touched
    let host = tekes_supervisor::process_host::ProductionProcessHost::open(
        root.path(),
        std::env::current_exe().expect("test executable"),
        "test-build",
        root.path().join("missing-agent-home"),
    )
    .expect("process host");
    host.attach_streams(assembly.streams().clone());
    assert!(
        respond(&assembly, &approval_rpc(&healthy), &healthy)
            .expect("healthy")
            .accepted
    );
    assert!(
        respond(&assembly, &approval_rpc(&torn), &torn)
            .expect("torn tail")
            .accepted,
        "a torn tail is repaired by the append"
    );
    match respond(&assembly, &approval_rpc(&invalid), &invalid) {
        Err(HostFailure::Quarantined(_)) => {}
        other => panic!("the damaged thread fails its own request, quarantined: {other:?}"),
    }
    assert_eq!(
        std::fs::read(
            root.path()
                .join("threads")
                .join(&invalid)
                .join("main.jsonl")
        )
        .expect("invalid ledger after"),
        invalid_before,
        "a quarantined thread is not rewritten"
    );
    // and the healthy answers are durable
    for session in [&healthy, &torn] {
        let ledger =
            std::fs::read_to_string(root.path().join("threads").join(session).join("main.jsonl"))
                .expect("ledger");
        assert!(ledger.contains("approval_response"), "{session}");
    }
    host.shutdown();
}

/// T19(c)(f): after a restart a respond finds its request again (no cards yet: the byte
/// prefilter), and the retry of an applied respond is answered from the registry.
#[test]
fn respond_works_after_a_restart_and_a_retry_is_answered() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    let session = blank_id(21);
    write_session_at(root.path(), &session, true);
    {
        let first = assembly(root.path());
        first.finish_recovery().expect("ready");
        assert!(
            respond(&first, &approval_rpc(&session), &session)
                .expect("respond")
                .accepted
        );
    }
    let restarted = assembly(root.path());
    restarted.finish_recovery().expect("ready");
    let retry = respond(&restarted, &approval_rpc(&session), &session).expect("retry");
    assert!(retry.accepted, "{retry:?}");
}

fn poll_once<F: Future>(future: F) -> Option<F::Output> {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None,
    }
}

/// Lazy registration: a new all-session generation starts empty and a session joins it the
/// first time something is published for it.
#[test]
fn a_new_mux_generation_registers_a_session_when_something_is_published_for_it() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    let sessions = (31..36).map(blank_id).collect::<Vec<_>>();
    for session in &sessions {
        write_session_at(root.path(), session, false);
    }
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("ready");
    let streams = assembly.streams();
    let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        streams,
        endpoint::StreamChannel::Mux,
    ))
    .expect("mux");
    assert!(
        poll_once(mux.recv()).is_none(),
        "five sessions on disk, none registered: nothing to deliver"
    );
    streams
        .publish_session_frame(
            &sessions[2],
            endpoint::MuxFrame::Queue {
                session_id: sessions[2].clone(),
                items: Vec::new(),
            },
        )
        .expect("publish");
    let subscribed = block_on_ready(mux.recv())
        .expect("subscribed item")
        .expect("subscribed frame");
    assert_eq!(subscribed.method, "session/subscribed");
    let queue = block_on_ready(mux.recv())
        .expect("queue item")
        .expect("queue frame");
    assert_eq!(queue.method, "session/queue");
    assert!(
        poll_once(mux.recv()).is_none(),
        "the other four stay unregistered"
    );
}

/// T8b': 256 sessions pinned by live workers fill the mux; the 257th is refused with
/// `active-session-limit {limit:256}` (for an attach and for a journal open); an idle session is
/// released to make room once a slot is unpinned, and the control and actionables streams open
/// with 257 sessions on disk.
#[test]
fn the_257th_pinned_session_is_refused_and_an_idle_one_is_evicted() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    let sessions = (100..357).map(blank_id).collect::<Vec<_>>();
    assert_eq!(sessions.len(), 257);
    for session in &sessions {
        write_session_at(root.path(), session, false);
    }
    let assembly = assembly(root.path());
    assembly.finish_recovery().expect("ready");
    let streams = assembly.streams().clone();
    let live = Arc::new(LiveSessions::default());
    let authority: Arc<dyn SupervisorSessionAuthority> = live.clone();
    streams.attach_session_authority(Arc::downgrade(&authority));
    // 257 sessions on disk do not fail the baselines
    for target in [
        SessionStreamTarget::SessionControl,
        SessionStreamTarget::Actionables,
    ] {
        let mut stream =
            block_on_ready(assembly.host().open_mux_stream(5, target)).expect("stream opens");
        assert!(block_on_ready(stream.recv()).expect("item").is_ok());
    }
    let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        &streams,
        endpoint::StreamChannel::Mux,
    ))
    .expect("mux");
    for session in &sessions[..256] {
        live.0.lock().expect("live").insert(session.clone());
        streams.attach_session(session).expect("attach");
    }
    let refused = streams
        .attach_session(&sessions[256])
        .expect_err("257th pinned attach");
    assert!(refused.to_string().contains("limit"), "{refused}");
    let journal = block_on_ready(assembly.host().open_mux_stream(
        6,
        SessionStreamTarget::SessionJournal {
            address: SessionAddress {
                session_id: sessions[256].clone(),
            },
            max_messages: 10,
        },
    ));
    match journal {
        Err(HostFailure::ActiveSessionLimit { limit: 256 }) => {}
        Err(other) => panic!("expected the limit error, got {other:?}"),
        Ok(_) => panic!("the 257th pinned journal open must be refused"),
    }
    // unpin one: the 257th attaches and the least recently touched idle session leaves
    live.0.lock().expect("live").remove(&sessions[0]);
    streams
        .attach_session(&sessions[256])
        .expect("attach after an unpin");
    assert!(
        poll_once(mux.recv()).is_some(),
        "the attached session announced itself"
    );
}

fn write_workspace(root: &std::path::Path) {
    std::fs::create_dir_all(root.join("workspaces/ws")).expect("workspaces");
    let mut bytes = serde_json_canonicalizer::to_vec(&serde_json::json!({
        "format":1,"revision":1,"id":"ws","name":"ws",
        "cwd":[root.join("workspace").to_string_lossy()],
        "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
    }))
    .expect("workspace json");
    bytes.push(b'\n');
    std::fs::write(root.join("workspaces/ws/workspace.json"), bytes).expect("workspace");
}

fn process_host(
    root: &std::path::Path,
) -> Arc<tekes_supervisor::process_host::ProductionProcessHost> {
    tekes_supervisor::process_host::ProductionProcessHost::open(
        root,
        std::env::current_exe().expect("test executable"),
        "test-build",
        root.join("missing-agent-home"),
    )
    .expect("process host")
}

/// T10: after a restart over threads whose cards and projections are on disk, opening the
/// inventory and control streams decodes no ledger, and the actionables stream decodes only the
/// one thread that holds a request.
#[test]
fn baselines_of_a_restarted_kernel_decode_only_the_thread_with_a_request() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let held = blank_id(500);
    write_session_at(root.path(), &held, true);
    for n in 501..541 {
        write_session_at(root.path(), &blank_id(n), false);
    }
    // first life: verify every thread (writes cards and projections), quit
    {
        let assembly = assembly(root.path());
        let host = process_host(root.path());
        host.attach_streams(assembly.streams().clone());
        let rows = SupervisorSessionAuthority::inventory_items(&*host, false)
            .expect("catalog backed")
            .expect("list");
        assert_eq!(rows.len(), 41);
        host.shutdown();
    }
    // second life
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    let before = schema::metrics::snapshot();
    let mut inventory = block_on_ready(
        assembly
            .host()
            .open_mux_stream(1, SessionStreamTarget::SessionInventory),
    )
    .expect("inventory stream");
    let SessionSyncFrame::InventoryBaseline { items, .. } = block_on_ready(inventory.recv())
        .expect("item")
        .expect("frame")
    else {
        panic!("expected an inventory baseline")
    };
    assert_eq!(items.len(), 41);
    let mut control = block_on_ready(
        assembly
            .host()
            .open_mux_stream(2, SessionStreamTarget::SessionControl),
    )
    .expect("control stream");
    let SessionSyncFrame::ControlBaseline { items, .. } = block_on_ready(control.recv())
        .expect("item")
        .expect("frame")
    else {
        panic!("expected a control baseline")
    };
    assert_eq!(items.len(), 41, "every thread has its persisted projection");
    assert_eq!(
        decodes_since(before),
        0,
        "inventory and control baselines decode no ledger"
    );
    let mut actionables = block_on_ready(
        assembly
            .host()
            .open_mux_stream(3, SessionStreamTarget::Actionables),
    )
    .expect("actionables stream");
    let SessionSyncFrame::ActionableBaseline { items, .. } = block_on_ready(actionables.recv())
        .expect("item")
        .expect("frame")
    else {
        panic!("expected an actionable baseline")
    };
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].session_id, held);
    let used = decodes_since(before);
    assert!(
        used < 60,
        "the actionables baseline decoded {used} records; the 40 other threads hold 80"
    );
    host.shutdown();
}

/// Opening a journal stream on a quarantined thread fails with the quarantine error, not a
/// generic one, and leaves the other threads' streams alone.
#[test]
fn a_journal_stream_on_a_quarantined_thread_reports_the_quarantine() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (healthy, damaged) = (blank_id(601), blank_id(602));
    write_session_at(root.path(), &healthy, false);
    write_session_at(root.path(), &damaged, false);
    {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .append(true)
            .open(
                root.path()
                    .join("threads")
                    .join(&damaged)
                    .join("main.jsonl"),
            )
            .expect("ledger")
            .write_all(b"{\"not\":\"a record\"}\n")
            .expect("append");
    }
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    let open = |session: &str| {
        block_on_ready(assembly.host().open_mux_stream(
            9,
            SessionStreamTarget::SessionJournal {
                address: SessionAddress {
                    session_id: session.to_owned(),
                },
                max_messages: 10,
            },
        ))
    };
    match open(&damaged) {
        Err(HostFailure::Quarantined(_)) => {}
        Err(other) => panic!("expected the quarantine error, got {other:?}"),
        Ok(_) => panic!("a quarantined thread must not open"),
    }
    assert!(open(&healthy).is_ok(), "the healthy thread still opens");
    // the quarantined thread keeps its row in the List
    let rows = SupervisorSessionAuthority::inventory_items(&*host, false)
        .expect("catalog backed")
        .expect("list");
    assert!(rows.iter().any(|row| row.session_id == damaged));
    host.shutdown();
}

/// S4d: with a catalog the control baseline omits a thread that was never projected (it does not
/// fold it), and decodes no ledger for the others.
#[test]
fn the_control_baseline_omits_a_never_projected_thread_without_folding_it() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let sessions = (700..705).map(blank_id).collect::<Vec<_>>();
    for session in &sessions {
        write_session_at(root.path(), session, false);
    }
    {
        let assembly = assembly(root.path());
        let host = process_host(root.path());
        host.attach_streams(assembly.streams().clone());
        let rows = SupervisorSessionAuthority::inventory_items(&*host, false)
            .expect("catalog backed")
            .expect("list");
        assert_eq!(rows.len(), 5);
        host.shutdown();
    }
    let never = &sessions[2];
    std::fs::remove_file(
        root.path()
            .join("threads")
            .join(never)
            .join("context-usage.projection.json"),
    )
    .expect("projection file");
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    let before = schema::metrics::snapshot();
    let mut control = block_on_ready(
        assembly
            .host()
            .open_mux_stream(2, SessionStreamTarget::SessionControl),
    )
    .expect("control stream");
    let SessionSyncFrame::ControlBaseline { items, .. } = block_on_ready(control.recv())
        .expect("item")
        .expect("frame")
    else {
        panic!("expected a control baseline")
    };
    assert_eq!(items.len(), 4);
    assert!(items.iter().all(|item| &item.session_id != never));
    assert_eq!(decodes_since(before), 0, "no ledger was folded");
    assert!(
        !root
            .path()
            .join("threads")
            .join(never)
            .join("context-usage.projection.json")
            .exists(),
        "and nothing was written for it"
    );
    host.shutdown();
}

/// ClientKit's rules for a journal snapshot (`SessionJournalSnapshot.validate()`, SessionJournalV3.swift),
/// applied to a real `open_mux_journal` snapshot: no entry lies beyond the frozen cut, and the
/// projections are not newer than the cut (`projectionBeyondFrozenCut`).
fn assert_client_accepts(snapshot: &endpoint::SessionJournalSnapshot, what: &str) {
    let through = snapshot.through_sequence;
    for entry in &snapshot.entries {
        assert!(
            i64::try_from(entry.event.seq).expect("seq fits") <= through,
            "{what}: entry seq {} is beyond throughSequence {through}",
            entry.event.seq
        );
    }
    let projections: serde_json::Value =
        serde_json::from_slice(&snapshot.projections.canonical_bytes().expect("bytes"))
            .expect("projections JSON");
    let as_of = projections["asOfSeq"].as_i64().expect("asOfSeq");
    assert!(
        as_of <= through,
        "{what}: projections.asOfSeq {as_of} is beyond throughSequence {through}"
    );
}

fn open_snapshot(
    assembly: &ProductionCarrierAssembly,
    stream: u64,
    session: &str,
) -> endpoint::SessionJournalSnapshot {
    let mut journal = block_on_ready(assembly.host().open_mux_stream(
        stream,
        SessionStreamTarget::SessionJournal {
            address: SessionAddress {
                session_id: session.to_owned(),
            },
            max_messages: 50,
        },
    ))
    .expect("journal stream");
    let SessionSyncFrame::JournalSnapshot { snapshot, .. } = block_on_ready(journal.recv())
        .expect("item")
        .expect("frame")
    else {
        panic!("expected a journal snapshot")
    };
    snapshot
}

/// S4d N1 / S4e B1: the journal snapshot carries the thread's current context projection, and it
/// satisfies the client's frozen-cut rule -- for a thread with journal entries and for an empty
/// one, on the first open and on a reopen after the projection moved.
#[test]
fn the_journal_snapshot_carries_the_context_projection_within_the_cut() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (blank, held) = (blank_id(800), blank_id(801));
    write_session_at(root.path(), &blank, false);
    write_session_at(root.path(), &held, true);
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    for (n, session) in [&blank, &held].into_iter().enumerate() {
        for round in 0..2 {
            let snapshot = open_snapshot(&assembly, 10 + 2 * n as u64 + round, session);
            assert_client_accepts(&snapshot, session);
            let projections: serde_json::Value =
                serde_json::from_slice(&snapshot.projections.canonical_bytes().expect("bytes"))
                    .expect("projections JSON");
            let values = &projections["values"];
            assert!(values["contextUsage"].is_object(), "{projections}");
            assert!(values["contextDetails"].is_object(), "{projections}");
            // the values are the persisted ones
            let saved: serde_json::Value = serde_json::from_slice(
                &std::fs::read(
                    root.path()
                        .join("threads")
                        .join(session)
                        .join("context-usage.projection.json"),
                )
                .expect("persisted projection"),
            )
            .expect("saved JSON");
            assert_eq!(values["contextUsage"], saved["value"]["contextUsage"]);
            println!(
                "{session} round {round}: throughSequence={} projections.asOfSeq={} persistedSequence={} entries={}",
                snapshot.through_sequence,
                projections["asOfSeq"],
                saved["sequence"],
                snapshot.entries.len()
            );
        }
    }
    assert!(
        !open_snapshot(&assembly, 40, &held).entries.is_empty(),
        "the held thread has journal entries, so the entry rule is exercised"
    );
    host.shutdown();
}

/// The journal stream after its snapshot: the pending frames the open queued.
fn open_journal_frames(
    assembly: &ProductionCarrierAssembly,
    stream: u64,
    session: &str,
    count: usize,
) -> Vec<SessionSyncFrame> {
    let mut journal = block_on_ready(assembly.host().open_mux_stream(
        stream,
        SessionStreamTarget::SessionJournal {
            address: SessionAddress {
                session_id: session.to_owned(),
            },
            max_messages: 50,
        },
    ))
    .expect("journal stream");
    (0..count)
        .map(|_| {
            block_on_ready(journal.recv())
                .expect("item")
                .expect("frame")
        })
        .collect()
}

fn projection_frame(frame: &SessionSyncFrame) -> (String, serde_json::Value, u64) {
    let SessionSyncFrame::JournalProjection {
        key,
        value,
        sequence,
        ..
    } = frame
    else {
        panic!("expected a journal projection frame")
    };
    (
        key.clone(),
        serde_json::from_slice(&value.canonical_bytes().expect("bytes")).expect("value JSON"),
        *sequence,
    )
}

/// N1: the snapshot's projection sequence is clamped to the cut, which a client holding the
/// newer control baseline does not take as an update (it keeps the highest sequence per key).
/// The open therefore follows the snapshot with the context projections at their true clock
/// sequence, beyond the cut, so a changed usage reaches a reopened idle thread.
#[test]
fn the_journal_open_replays_the_context_projection_at_its_true_sequence() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (blank, held) = (blank_id(810), blank_id(811));
    write_session_at(root.path(), &blank, false);
    write_session_at(root.path(), &held, true);
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    for (n, session) in [&blank, &held].into_iter().enumerate() {
        let frames = open_journal_frames(&assembly, 20 + n as u64, session, 3);
        let SessionSyncFrame::JournalSnapshot { snapshot, .. } = &frames[0] else {
            panic!("the first frame is the snapshot")
        };
        assert_client_accepts(snapshot, session);
        let carried: serde_json::Value =
            serde_json::from_slice(&snapshot.projections.canonical_bytes().expect("bytes"))
                .expect("projections JSON");
        let saved: serde_json::Value = serde_json::from_slice(
            &std::fs::read(
                root.path()
                    .join("threads")
                    .join(session)
                    .join("context-usage.projection.json"),
            )
            .expect("persisted projection"),
        )
        .expect("saved JSON");
        let sequence = saved["sequence"].as_u64().expect("persisted sequence");
        assert!(
            i64::try_from(sequence).expect("fits") > snapshot.through_sequence,
            "the true clock is beyond the cut, which is why the snapshot is clamped"
        );
        assert!(
            carried["asOfSeq"].as_i64().expect("asOfSeq") < i64::try_from(sequence).expect("fits"),
            "the clamped snapshot label is below the true sequence"
        );
        let replayed = frames[1..].iter().map(projection_frame).collect::<Vec<_>>();
        assert_eq!(
            replayed
                .iter()
                .map(|frame| frame.0.as_str())
                .collect::<Vec<_>>(),
            ["contextDetails", "contextUsage"]
        );
        for (key, value, replayed_sequence) in &replayed {
            assert_eq!(*replayed_sequence, sequence, "the persisted clock sequence");
            assert_eq!(value, &carried["values"][key.as_str()]);
            assert_eq!(value, &saved["value"][key.as_str()]);
        }
    }
    host.shutdown();
}

/// N1, the scenario: the projection moved while the thread was idle (its persisted value no
/// longer matches the ledger), so the client's control baseline holds the old value at the old
/// sequence. The reopen's reconcile republishes it at a higher sequence and the journal replays
/// exactly that sequence and value, which the client's strictly-greater merge takes.
#[test]
fn a_reopened_journal_replays_the_sequence_the_reconcile_published() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let held = blank_id(820);
    write_session_at(root.path(), &held, true);
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    let first = open_journal_frames(&assembly, 30, &held, 3);
    let (_, _, old_sequence) = projection_frame(&first[2]);

    let path = root
        .path()
        .join("threads")
        .join(&held)
        .join("context-usage.projection.json");
    let mut saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("saved")).expect("saved JSON");
    saved["value"]["contextUsage"]["usedTokens"] = serde_json::json!(4242);
    std::fs::write(&path, serde_json::to_vec(&saved).expect("json")).expect("stale projection");

    let second = open_journal_frames(&assembly, 31, &held, 3);
    let (key, value, sequence) = projection_frame(&second[2]);
    assert_eq!(key, "contextUsage");
    assert!(
        sequence > old_sequence,
        "the republished projection is newer than the one the client holds"
    );
    let published: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("republished")).expect("JSON");
    assert_eq!(sequence, published["sequence"].as_u64().expect("sequence"));
    assert_eq!(value, published["value"]["contextUsage"]);
    let used = &value["usedTokens"];
    assert_ne!(*used, 4242, "the stale value was replaced");
    host.shutdown();
}

/// E836 C3: a thread whose workspace no longer exists is not hidden from the sidebar behind
/// the Kernel's back: the inventory baseline fails (as it always did) instead of silently
/// dropping the row.
#[test]
fn an_orphan_workspace_thread_fails_the_inventory_baseline_instead_of_vanishing() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (fine, orphan) = (blank_id(900), blank_id(901));
    write_session_at(root.path(), &fine, false);
    write_session_at(root.path(), &orphan, false);
    let ledger = root.path().join("threads").join(&orphan).join("main.jsonl");
    let text = std::fs::read_to_string(&ledger).expect("ledger");
    std::fs::write(
        &ledger,
        text.replacen(r#""workspace":"ws""#, r#""workspace":"gone""#, 1),
    )
    .expect("orphan ledger");
    let assembly = assembly(root.path());
    let host = process_host(root.path());
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    let opened = block_on_ready(
        assembly
            .host()
            .open_mux_stream(1, SessionStreamTarget::SessionInventory),
    );
    // the error names the missing workspace (`ManagementError::WorkspaceNotFound("gone")`)
    match opened {
        Err(HostFailure::Internal(message)) => assert!(
            message.contains("gone") && message.contains("workspace was not found"),
            "the error names the missing workspace: {message}"
        ),
        Err(other) => panic!("expected an internal failure naming the workspace, got {other:?}"),
        Ok(_) => panic!("the orphan row must be an error, not a silent omission"),
    }
    host.shutdown();
}

// ---------------------------------------------------------------------------------------------
// O-final B1: a pending approval or question in a CHILD line of a thread is in the actionables
// baseline however the thread came to be known (v0.3.0 scanned every line of every thread).

fn canonical_lines(events: Vec<serde_json::Value>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in events {
        let event =
            schema::Event::decode(&serde_json::to_vec(&value).expect("json")).expect("event");
        bytes.extend_from_slice(&event.canonical_bytes().expect("canonical"));
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, schema::READER_VERSION).expect("ledger validates");
    bytes
}

/// main: genesis, input, run_start, turn_open, spawn(child). child: a tool call, and (when
/// `held`) a pending approval request at seq 7.
fn write_session_with_child(root: &std::path::Path, session: &str, child: &str, held: bool) {
    use serde_json::json;
    store::ThreadStore::open(root).expect("store layout");
    let folder = root.join("threads").join(session);
    std::fs::create_dir_all(folder.join("assets")).expect("folder");
    let main = canonical_lines(vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TIMESTAMP,"format":1,"min_reader":1,"min_writer":1,
            "thread":session,"workspace":"ws","origin_key":"create",
            "origin_tuple":{"principal":"p","client":"cli","target":session,"op":"create","key":"create"},
            "resume":"never","config":{"digest":"cfg"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TIMESTAMP,"content":[{"type":"text","text":"spawn"}],
            "origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":session,"op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TIMESTAMP,"run":"r1","mode":"ordinary","recovery_ordinal":0,
            "binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"kind":"turn_open","ts":TIMESTAMP,"turn":1,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"spawn","ts":TIMESTAMP,"turn":1,"child":format!("{child}.jsonl"),
            "call":"call-spawn","spawn_id":"spawn-1","resume":"never","seed":{"kinds":[]}}),
    ]);
    std::fs::write(folder.join("main.jsonl"), main).expect("main");
    let mut events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TIMESTAMP,"format":1,"min_reader":1,"min_writer":1,
            "thread":child,"workspace":"ws","origin_key":"create-child",
            "origin_tuple":{"principal":"p","client":"cli","target":child,"op":"create","key":"create-child"},
            "resume":"never","config":{"digest":"cfg"},
            "parent":{"file":"main.jsonl","seq":5,"spawn_id":"spawn-1"},
            "seed":{"source":"parent","kinds":[],
                "snapshot":{"asset":format!("sha256-{}", "0".repeat(64)),"digest":"0".repeat(64)}}}),
        json!({"v":1,"seq":2,"kind":"turn_open","ts":TIMESTAMP,"turn":1,"trigger":"genesis"}),
        json!({"v":1,"seq":3,"kind":"epoch","ts":TIMESTAMP,"adapter":"fake","id":"e1","model":"fake","reason":"initial",
            "renderer":1,"system":{"asset":"sha256-aa","digest":"d1"},"tools":{"asset":"sha256-td","digest":"td"}}),
        json!({"v":1,"seq":4,"kind":"attempt","ts":TIMESTAMP,"admits":[],"attempt":"a1","epoch":"e1",
            "request":{"asset":format!("sha256-{}", "ab".repeat(32)),"bytes":4096},"turn":1,"wire_digest":"wd"}),
        json!({"v":1,"seq":5,"kind":"output","ts":TIMESTAMP,"attempt":"a1",
            "content":[{"args":{"cmd":"ls"},"call":"c1","name":"shell","type":"tool-call"}],
            "sealed":{"adapter":"fake","fragments":"x","version":1},"turn":1,
            "usage":{"availability":"reported","input_tokens":"5","output_tokens":"1"}}),
        json!({"v":1,"seq":6,"kind":"tool_call","ts":TIMESTAMP,"args":{"cmd":"ls"},"attempt":"a1","call":"c1",
            "name":"shell","source":"provider","turn":1}),
    ];
    if held {
        events.push(child_approval_request());
    }
    std::fs::write(
        folder.join(format!("{child}.jsonl")),
        canonical_lines(events),
    )
    .expect("child");
}

fn child_approval_request() -> serde_json::Value {
    serde_json::json!({"v":1,"seq":7,"kind":"approval_request","ts":TIMESTAMP,"call":"c1","scope":"exec","turn":1})
}

fn child_approval_response(child: &str) -> serde_json::Value {
    serde_json::json!({"v":1,"seq":8,"kind":"approval_response","ts":TIMESTAMP,"call":"c1","grant":true,"turn":1,
        "origin_key":"r1","origin_tuple":{"principal":"p","client":"cli","target":child,"op":"approve","key":"r1"}})
}

/// Appends events to the child ledger the way a worker would (canonical lines, one append).
fn append_child_events(
    root: &std::path::Path,
    session: &str,
    child: &str,
    events: Vec<serde_json::Value>,
) {
    use std::io::Write;
    let path = root
        .join("threads")
        .join(session)
        .join(format!("{child}.jsonl"));
    let mut bytes = std::fs::read(&path).expect("child ledger");
    for value in events {
        let event =
            schema::Event::decode(&serde_json::to_vec(&value).expect("json")).expect("event");
        bytes.extend_from_slice(&event.canonical_bytes().expect("canonical"));
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, schema::READER_VERSION).expect("grown child ledger validates");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("open child");
    file.write_all(&bytes).expect("write child");
}

fn actionable_ids(assembly: &ProductionCarrierAssembly, generation: u64) -> Vec<String> {
    let mut stream = block_on_ready(
        assembly
            .host()
            .open_mux_stream(generation, SessionStreamTarget::Actionables),
    )
    .expect("actionables stream");
    let SessionSyncFrame::ActionableBaseline { items, .. } =
        block_on_ready(stream.recv()).expect("item").expect("frame")
    else {
        panic!("expected an actionable baseline")
    };
    items.into_iter().map(|item| item.id).collect()
}

/// O-final n4: a request frame that could not be announced at the 256-session limit is
/// announced by the next reconcile once a slot is free. The published set must not record the
/// request before the frame is out, or the retry sees "nothing changed" and the frame is lost.
#[test]
fn a_request_frame_refused_at_the_session_limit_is_announced_by_the_next_reconcile() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let sessions = (100..357).map(blank_id).collect::<Vec<_>>();
    for (index, session) in sessions.iter().enumerate() {
        write_session_at(root.path(), session, index == 256);
    }
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    let streams = assembly.streams().clone();
    let live = Arc::new(LiveSessions::default());
    let authority: Arc<dyn SupervisorSessionAuthority> = live.clone();
    streams.attach_session_authority(Arc::downgrade(&authority));
    let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        &streams,
        endpoint::StreamChannel::Mux,
    ))
    .expect("mux");
    for session in &sessions[..256] {
        live.0.lock().expect("live").insert(session.clone());
        streams.attach_session(session).expect("attach");
    }
    // At the limit: the doorbell succeeds, the request frame cannot be announced.
    SupervisorSessionAuthority::reconcile_projection(&*host, &sessions[256])
        .expect("the limit does not fail the doorbell");
    while poll_once(mux.recv()).is_some() {}
    // A slot frees up; the next reconcile of the same unchanged ledger announces the frame.
    live.0.lock().expect("live").remove(&sessions[0]);
    SupervisorSessionAuthority::reconcile_projection(&*host, &sessions[256])
        .expect("the retry succeeds");
    let mut announced = Vec::new();
    while let Some(Some(Ok(frame))) = poll_once(mux.recv()) {
        announced.push((
            frame.method.clone(),
            String::from_utf8_lossy(&frame.payload.canonical_bytes().expect("payload"))
                .into_owned(),
        ));
    }
    assert!(
        announced
            .iter()
            .any(|(method, params)| method.contains("approval") && params.contains(&sessions[256])),
        "the held request of the 257th session is announced after the retry"
    );
    host.shutdown();
}

fn kernel_host(
    root: &std::path::Path,
    assembly: &ProductionCarrierAssembly,
) -> Arc<tekes_supervisor::process_host::ProductionProcessHost> {
    let host = process_host(root);
    host.attach_streams(assembly.streams().clone());
    assembly.finish_recovery().expect("ready");
    host
}

fn child_rpc(session: &str, child: &str) -> String {
    endpoint::derive_line_request_rpc_id(
        session,
        Some(&format!("{child}.jsonl")),
        endpoint::RequestFrameType::Approval,
        7,
    )
}

fn child_allow(rpc_id: &str, session: &str, child: &str) -> ClientResponse {
    ClientResponse {
        envelope_type: "client-response".to_owned(),
        rpc_id: rpc_id.to_owned(),
        result: ClientResponseResult {
            ok: true,
            value: Some(
                IJsonValue::parse_str(&format!(
                    r#"{{"approvalId":"approval:{child}.jsonl:c1","outcome":"allowed-once","sessionId":"{session}"}}"#
                ))
                .expect("respond value"),
            ),
            error: None,
        },
    }
}

/// The oracle: a carrier without a catalog scans every line like v0.3.0 did.
fn oracle_ids(root: &std::path::Path) -> Vec<String> {
    actionable_ids(&assembly(root), 1)
}

/// B1 (a): a thread verified in this process (the first List) still shows the hold of its child
/// line. Before S4f the verified view carried only the main line and the baseline never looked.
#[test]
fn a_child_line_hold_is_in_the_baseline_of_a_thread_verified_in_this_process() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (session, child) = (blank_id(700), blank_id(701));
    write_session_with_child(root.path(), &session, &child, true);
    let oracle = oracle_ids(root.path());
    assert_eq!(
        oracle,
        vec![child_rpc(&session, &child)],
        "the oracle sees it"
    );

    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    let rows = SupervisorSessionAuthority::inventory_items(&*host, false)
        .expect("catalog backed")
        .expect("list");
    assert_eq!(rows.len(), 1, "the thread is listed (and so verified)");
    assert_eq!(actionable_ids(&assembly, 2), oracle);
    host.shutdown();
}

/// B1 (b): the hold appears in the child line after the thread was verified, and the main
/// line's doorbell fold (which carries the child views forward) has run since.
#[test]
fn a_child_hold_appended_after_verification_is_in_the_baseline() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (session, child) = (blank_id(702), blank_id(703));
    write_session_with_child(root.path(), &session, &child, false);
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    let rows = SupervisorSessionAuthority::inventory_items(&*host, false)
        .expect("catalog backed")
        .expect("list");
    assert_eq!(rows.len(), 1);
    assert!(
        actionable_ids(&assembly, 2).is_empty(),
        "nothing is held yet"
    );

    append_child_events(
        root.path(),
        &session,
        &child,
        vec![child_approval_request()],
    );
    // the doorbell of the thread's main line (a child worker's append rings it)
    SupervisorSessionAuthority::reconcile_projection(&*host, &session).expect("doorbell");
    assert_eq!(
        actionable_ids(&assembly, 3),
        oracle_ids(root.path()),
        "the baseline equals a full scan"
    );
    assert_eq!(
        actionable_ids(&assembly, 4),
        vec![child_rpc(&session, &child)]
    );
    host.shutdown();
}

/// B1 (c): a hold that was resolved in the child line is gone from the baseline when the
/// resolution happened between two lives (the card the first life wrote still describes the
/// hold as pending).
#[test]
fn a_resolved_child_hold_leaves_the_baseline() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (session, child) = (blank_id(704), blank_id(705));
    write_session_with_child(root.path(), &session, &child, true);
    {
        let assembly = assembly(root.path());
        let host = kernel_host(root.path(), &assembly);
        let _ = SupervisorSessionAuthority::inventory_items(&*host, false).expect("list");
        assert_eq!(
            actionable_ids(&assembly, 2),
            vec![child_rpc(&session, &child)]
        );
        host.shutdown();
    }
    append_child_events(
        root.path(),
        &session,
        &child,
        vec![child_approval_response(&child)],
    );
    assert!(oracle_ids(root.path()).is_empty());
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    assert!(
        actionable_ids(&assembly, 2).is_empty(),
        "the second life must not announce a hold the child line already answered"
    );
    host.shutdown();
}

/// B1 (d): after a restart a card that lists the child line makes the child hold a candidate.
#[test]
fn a_child_hold_is_in_the_baseline_after_a_restart_with_a_card() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let (session, child) = (blank_id(710), blank_id(711));
    write_session_with_child(root.path(), &session, &child, true);
    {
        let assembly = assembly(root.path());
        let host = kernel_host(root.path(), &assembly);
        let _ = SupervisorSessionAuthority::inventory_items(&*host, false).expect("list");
        host.shutdown();
    }
    let card = std::fs::read_to_string(
        root.path()
            .join("threads")
            .join(&session)
            .join("thread-card.json"),
    )
    .expect("the first life flushed a card");
    assert!(
        card.contains(&format!("{child}.jsonl")),
        "the card lists the child line"
    );
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    assert_eq!(
        actionable_ids(&assembly, 5),
        vec![child_rpc(&session, &child)]
    );
    host.shutdown();
}

/// B1 (e): answering the child hold works on a thread verified in this process, like a bare
/// carrier answers it.
#[test]
fn a_child_hold_can_be_answered_after_in_process_verification() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let (session, child) = (blank_id(720), blank_id(721));
    let rpc = child_rpc(&session, &child);
    let oracle_root = tempfile::tempdir().expect("root");
    write_workspace(oracle_root.path());
    write_session_with_child(oracle_root.path(), &session, &child, true);
    let bare = assembly(oracle_root.path());
    let oracle = block_on_ready(
        bare.host()
            .respond(child_allow(&rpc, &session, &child), call_context()),
    );

    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    write_session_with_child(root.path(), &session, &child, true);
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    let _ = SupervisorSessionAuthority::inventory_items(&*host, false).expect("list");
    let kernel = block_on_ready(
        assembly
            .host()
            .respond(child_allow(&rpc, &session, &child), call_context()),
    );
    assert_eq!(
        kernel.as_ref().map(|_| ()).map_err(ToString::to_string),
        oracle.as_ref().map(|_| ()).map_err(ToString::to_string),
        "the Kernel answers like a bare carrier"
    );
    host.shutdown();
}

/// O-final m10: with 256 sessions pinned, the doorbell of a 257th thread that holds a request
/// still succeeds (its durable events are broadcast); only the request frame cannot be announced.
#[test]
fn a_doorbell_over_the_session_limit_is_not_an_error() {
    let _serial = SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let root = tempfile::tempdir().expect("root");
    write_workspace(root.path());
    let sessions = (100..357).map(blank_id).collect::<Vec<_>>();
    for (index, session) in sessions.iter().enumerate() {
        write_session_at(root.path(), session, index == 256);
    }
    let assembly = assembly(root.path());
    let host = kernel_host(root.path(), &assembly);
    let streams = assembly.streams().clone();
    let live = Arc::new(LiveSessions::default());
    let authority: Arc<dyn SupervisorSessionAuthority> = live.clone();
    streams.attach_session_authority(Arc::downgrade(&authority));
    let _mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
        &streams,
        endpoint::StreamChannel::Mux,
    ))
    .expect("mux");
    for session in &sessions[..256] {
        live.0.lock().expect("live").insert(session.clone());
        streams.attach_session(session).expect("attach");
    }
    SupervisorSessionAuthority::reconcile_projection(&*host, &sessions[256])
        .expect("the limit does not fail the doorbell");
    host.shutdown();
}
