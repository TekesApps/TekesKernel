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
