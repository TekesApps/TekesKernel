use std::collections::BTreeSet;
use std::future::{Future, ready};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

use endpoint::{
    AllSessionMux, ClientRequest, DurableHandoffProof, DurableHandoffSignal, EndpointDispatcher,
    EndpointHost, EndpointHostCall, EndpointHostFuture, EndpointJournal, EndpointSubscriptionHub,
    HostFrame, HostFrameKind, MuxFrame, MuxRegistration, MuxReplayRegistration, RpcBegin,
    RpcDurableIdentity, RpcRegistry, RpcResult, SessionEvent, SessionHostDescription,
    SubscriptionPoll,
};
use schema::IJsonValue;
use sha2::{Digest, Sha256};

const SESSION: &str = "018f0000-0000-7000-8000-000000000003";

struct MockHost {
    calls: AtomicUsize,
    recovery_flags: Mutex<Vec<bool>>,
}

impl MockHost {
    fn new() -> Self {
        Self {
            calls: AtomicUsize::new(0),
            recovery_flags: Mutex::new(Vec::new()),
        }
    }
}

impl EndpointHost for MockHost {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.prompt".to_owned()].into_iter().collect()
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.recovery_flags
            .lock()
            .expect("recovery flags")
            .push(request.recovering);
        let value = IJsonValue::parse_str(&format!(
            r#"{{"operation":"{}","rpcId":"{}"}}"#,
            request.operation, request.rpc_id
        ))
        .expect("result value");
        Box::pin(ready(RpcResult {
            ok: true,
            value: Some(value),
            error: None,
        }))
    }
}

fn block_on_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("mock host future unexpectedly pending"),
    }
}

fn request(rpc_id: &str, text: &str) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: "session.prompt".to_owned(),
        payload: IJsonValue::parse_str(&format!(r#"{{"text":"{text}"}}"#)).expect("payload"),
    }
}

#[test]
fn endpoint_wide_rpc_registry_returns_original_result_and_rejects_conflict() {
    let root = tempfile::tempdir().expect("root");
    let host = MockHost::new();
    let first = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("registry"));
    let response =
        block_on_ready(first.dispatch(&host, request("rpc-1", "one"))).expect("first dispatch");
    assert!(response.result.ok);
    assert_eq!(host.calls.load(Ordering::SeqCst), 1);

    let reopened = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
    let retry =
        block_on_ready(reopened.dispatch(&host, request("rpc-1", "one"))).expect("retry dispatch");
    assert_eq!(retry, response);
    assert_eq!(host.calls.load(Ordering::SeqCst), 1);

    let conflict =
        block_on_ready(reopened.dispatch(&host, request("rpc-1", "two"))).expect("typed conflict");
    let error = conflict.result.error.expect("conflict error");
    assert_eq!(error.code, "idempotency-conflict");
    assert_eq!(error.message, "rpcId was already used for another request");
    assert_eq!(
        error.details,
        IJsonValue::parse_str(r#"{"operation":"session.prompt","rpcId":"rpc-1"}"#)
            .expect("conflict details")
    );
    assert_eq!(host.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn durable_pending_claim_is_recovered_with_the_same_rpc_id() {
    let root = tempfile::tempdir().expect("root");
    let registry = RpcRegistry::open(root.path()).expect("registry");
    let request = request("rpc-recover", "recover");
    let begin = registry.begin(&request).expect("pending claim");
    assert!(matches!(
        begin,
        endpoint::RpcBegin::Execute {
            recovering: false,
            ..
        }
    ));
    drop(registry);

    let host = MockHost::new();
    let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
    let result = block_on_ready(dispatcher.dispatch(&host, request)).expect("recover dispatch");
    assert!(result.result.ok);
    assert_eq!(
        host.recovery_flags.lock().expect("flags").as_slice(),
        &[true]
    );
}

#[test]
fn host_description_is_exact_client_owned_dto() {
    let description = SessionHostDescription {
        version: "0.1.0".to_owned(),
        cwd: "/workspace".to_owned(),
        provider: Some("deepseek".to_owned()),
        model: Some("deepseek-chat".to_owned()),
        attached_sessions: 1,
        home: "/Users/example".to_owned(),
        can_open_path: true,
    };
    let value = serde_json::to_value(description).expect("host description");
    assert_eq!(value["version"], "0.1.0");
    assert_eq!(value["attachedSessions"], 1);
    assert!(value.get("generation").is_none());
    assert!(value.get("capabilities").is_none());
}

#[test]
fn durable_handoff_signal_survives_carrier_timeout_cancellation() {
    let host_side = DurableHandoffSignal::new();
    let carrier_side = host_side.clone();
    assert!(!carrier_side.is_durable());
    host_side.mark_durable();
    drop(host_side);
    assert!(carrier_side.is_durable());
}

struct ProofHost;

impl EndpointHost for ProofHost {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.prompt".to_owned()].into_iter().collect()
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        request
            .handoff
            .mark_handed_off(DurableHandoffProof {
                delivery: "locked-append".to_owned(),
                durable_identity: Some(RpcDurableIdentity {
                    kind: "input".to_owned(),
                    id: request.rpc_id.clone(),
                    seq: Some(24),
                }),
            })
            .expect("handoff proof");
        Box::pin(ready(RpcResult {
            ok: true,
            value: Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("receipt")),
            error: None,
        }))
    }
}

#[test]
fn dispatcher_persists_typed_handoff_before_completion() {
    let root = tempfile::tempdir().expect("root");
    let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("registry"));
    let response =
        block_on_ready(dispatcher.dispatch(&ProofHost, request("rpc-proof", "persist the proof")))
            .expect("dispatch");
    assert!(response.result.ok);

    let hash = format!("{:x}", Sha256::digest(b"rpc-proof"));
    let path = root
        .path()
        .join("endpoint-management/rpc")
        .join(&hash[..2])
        .join(format!("{hash}.jsonl"));
    let rows: Vec<serde_json::Value> = std::fs::read_to_string(path)
        .expect("rpc carrier")
        .lines()
        .map(|line| serde_json::from_str(line).expect("row"))
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["phase"], "prepared");
    assert_eq!(rows[1]["phase"], "handed-off");
    assert_eq!(rows[1]["delivery"], "locked-append");
    assert_eq!(rows[1]["durable_identity"]["kind"], "input");
    assert_eq!(rows[2]["phase"], "complete");
    assert_eq!(rows[2]["delivery"], "locked-append");
    assert_eq!(rows[2]["durable_identity"]["seq"], 24);
}

#[test]
fn handed_off_claim_is_idempotent_across_restart_and_conflicts_fail_closed() {
    let root = tempfile::tempdir().expect("root");
    let request = request("rpc-proof-recovery", "recover the proof");
    let registry = RpcRegistry::open(root.path()).expect("registry");
    let RpcBegin::Execute { claim, recovering } = registry.begin(&request).expect("prepared")
    else {
        panic!("new rpc unexpectedly completed")
    };
    assert!(!recovering);
    let identity = RpcDurableIdentity {
        kind: "input".to_owned(),
        id: "origin-proof-recovery".to_owned(),
        seq: Some(25),
    };
    registry
        .mark_handed_off(&claim, "locked-append", Some(identity.clone()))
        .expect("handoff");
    drop(registry);

    let reopened = RpcRegistry::open(root.path()).expect("reopen");
    let RpcBegin::Execute { claim, recovering } = reopened.begin(&request).expect("recover") else {
        panic!("handed-off rpc unexpectedly completed")
    };
    assert!(recovering);
    reopened
        .mark_handed_off(&claim, "locked-append", Some(identity))
        .expect("same proof is idempotent");
    assert!(
        reopened
            .mark_handed_off(&claim, "management", None)
            .is_err(),
        "a changed recovery proof must fail closed"
    );
    reopened
        .complete(
            &claim,
            &RpcResult {
                ok: true,
                value: Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("receipt")),
                error: None,
            },
        )
        .expect("complete recovered rpc");
}

fn event(time: f64) -> SessionEvent {
    SessionEvent {
        event_type: "turn/start".to_owned(),
        seq: 99,
        time,
        data: IJsonValue::parse_str(r#"{"turn":0}"#).expect("event data"),
        ignorable: None,
        source_event_seqs: None,
        surface_op: None,
    }
}

#[test]
fn journal_baseline_and_publish_are_an_atomic_handoff() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let durable_before = journal
        .append_kernel(vec![1], "turn-start", event(1.0))
        .expect("first durable event");
    let hub = EndpointSubscriptionHub::default();
    let subscription = hub
        .subscribe(SESSION, &journal, "push-subscribed")
        .expect("subscribe");
    assert_eq!(subscription.baseline().last_seq, 0);
    assert!(matches!(
        subscription.poll().expect("subscribed poll"),
        SubscriptionPoll::Frame(frame) if frame.method == "session/subscribed"
    ));
    assert!(
        hub.publish_event(SESSION, &journal, "push-undurable", event(3.0), None)
            .is_err(),
        "live publication cannot precede journal durability"
    );

    // A publisher delayed until after registration cannot duplicate an event
    // that the durable snapshot already included.
    assert_eq!(
        hub.publish_event(SESSION, &journal, "push-old", durable_before, None)
            .expect("old publish"),
        0
    );
    assert_eq!(
        subscription.poll().expect("empty poll"),
        SubscriptionPoll::Pending
    );

    let durable_after = journal
        .append_kernel(vec![2], "turn-start-2", event(2.0))
        .expect("second durable event");
    let durable_after_two = journal
        .append_kernel(vec![3], "turn-start-3", event(3.0))
        .expect("third durable event");
    assert_eq!(
        hub.publish_event(
            SESSION,
            &journal,
            "push-new-2",
            durable_after_two.clone(),
            None,
        )
        .expect("out-of-order publish buffers"),
        1
    );
    assert_eq!(
        subscription.poll().expect("waiting for preceding seq"),
        SubscriptionPoll::Pending
    );
    assert_eq!(
        hub.publish_event(SESSION, &journal, "push-new", durable_after.clone(), None,)
            .expect("new publish"),
        1
    );
    let SubscriptionPoll::Frame(frame) = subscription.poll().expect("event poll") else {
        panic!("expected event frame")
    };
    assert_eq!(frame.method, "session/event");
    let payload: MuxFrame =
        serde_json::from_slice(&frame.payload.canonical_bytes().expect("canonical payload"))
            .expect("mux payload");
    assert!(matches!(payload, MuxFrame::Event { event, .. } if event.seq == 1));
    let SubscriptionPoll::Frame(frame) = subscription.poll().expect("ordered event poll") else {
        panic!("expected buffered event frame")
    };
    let payload: MuxFrame =
        serde_json::from_slice(&frame.payload.canonical_bytes().expect("canonical payload"))
            .expect("mux payload");
    assert!(matches!(payload, MuxFrame::Event { event, .. } if event.seq == 2));

    assert_eq!(
        hub.publish_event(SESSION, &journal, "push-new-retry", durable_after, None,)
            .expect("duplicate publish"),
        0
    );
    assert_eq!(
        hub.publish_event(
            SESSION,
            &journal,
            "push-new-2-retry",
            durable_after_two,
            None,
        )
        .expect("second duplicate publish"),
        0
    );
}

#[test]
fn subscriber_overflow_preserves_prefix_then_reports_live_gap() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let hub = EndpointSubscriptionHub::default();
    let subscription = hub
        .subscribe_with_bounds(SESSION, &journal, "push-sub", 1, 4_096)
        .expect("subscribe");
    assert!(matches!(
        subscription.poll().expect("subscribed"),
        SubscriptionPoll::Frame(_)
    ));

    assert_eq!(
        hub.publish_frame(
            SESSION,
            "push-queue",
            MuxFrame::Queue {
                session_id: SESSION.to_owned(),
                items: Vec::new(),
            },
        )
        .expect("queue frame"),
        1
    );
    assert_eq!(
        hub.publish_frame(
            SESSION,
            "push-projection",
            MuxFrame::Projection {
                session_id: SESSION.to_owned(),
                key: "sessionTitle".to_owned(),
                value: IJsonValue::parse_str(r#"{"title":"x"}"#).expect("projection"),
                seq: 0,
            },
        )
        .expect("overflow is subscription-local"),
        0
    );
    assert!(matches!(
        subscription.poll().expect("prefix"),
        SubscriptionPoll::Frame(frame) if frame.method == "session/queue"
    ));
    assert_eq!(subscription.poll().expect("gap"), SubscriptionPoll::LiveGap);
    assert_eq!(
        subscription.poll().expect("closed"),
        SubscriptionPoll::Closed
    );
}

#[test]
fn atomic_unresolved_replay_is_ordered_and_private_to_the_new_mux_generation() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let hub = EndpointSubscriptionHub::default();
    let old = AllSessionMux::open(
        &hub,
        &[MuxRegistration {
            session_id: SESSION,
            journal: &journal,
            subscribed_rpc_id: "old-subscribed",
        }],
    )
    .expect("old mux");
    assert!(matches!(
        old.poll(SESSION).expect("old baseline"),
        SubscriptionPoll::Frame(frame) if frame.method == "session/subscribed"
    ));

    let replay = MuxFrame::ApprovalRequested {
        session_id: SESSION.to_owned(),
        approval_id: "approval:call-1".to_owned(),
        tool_name: "shell".to_owned(),
        call_id: Some("call-1".to_owned()),
        reason: Some("workspace-write".to_owned()),
    }
    .into_envelope("request-replay")
    .expect("replay envelope");
    let new = AllSessionMux::open_with_replay(
        &hub,
        &[MuxReplayRegistration {
            registration: MuxRegistration {
                session_id: SESSION,
                journal: &journal,
                subscribed_rpc_id: "new-subscribed",
            },
            unresolved: &[replay],
        }],
    )
    .expect("new mux");
    assert!(matches!(
        new.poll(SESSION).expect("new baseline"),
        SubscriptionPoll::Frame(frame) if frame.method == "session/subscribed"
    ));
    assert!(matches!(
        new.poll(SESSION).expect("new replay"),
        SubscriptionPoll::Frame(frame)
            if frame.method == "approval/requested" && frame.rpc_id == "request-replay"
    ));
    assert_eq!(
        old.poll(SESSION).expect("old generation remains idle"),
        SubscriptionPoll::Pending
    );
}

#[test]
fn mux_dto_preserves_the_authority_envelope_shape() {
    let frame = MuxFrame::Subscribed {
        session_id: "session-1".to_owned(),
        last_seq: 6,
    }
    .into_envelope("push-sub-1")
    .expect("envelope");
    let value: serde_json::Value =
        serde_json::from_slice(&frame.canonical_bytes().expect("canonical")).expect("JSON");
    assert_eq!(value["type"], "server-request");
    assert_eq!(value["rpcId"], "push-sub-1");
    assert_eq!(value["method"], "session/subscribed");
    assert_eq!(value["payload"]["type"], "session/subscribed");
    assert_eq!(value["payload"]["sessionId"], "session-1");
    assert_eq!(value["payload"]["lastSeq"], 6);

    let host = HostFrame::new(
        HostFrameKind::SessionStatus,
        IJsonValue::parse_str(
            r#"{"type":"host/session-status","sessionId":"session-1","running":true}"#,
        )
        .expect("host payload"),
    )
    .expect("host frame")
    .into_envelope("push-host-1")
    .expect("host envelope");
    assert_eq!(host.method, "host/session-status");
    assert!(HostFrame::new(HostFrameKind::SessionAdded, host.payload,).is_err());
}
