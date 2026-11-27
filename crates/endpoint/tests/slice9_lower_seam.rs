use std::collections::BTreeSet;
use std::future::{Future, ready};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
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
        .join("endpoint-rpc/rpc")
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

#[test]
fn publishing_with_no_subscriber_delivers_nothing_and_a_later_subscriber_still_sees_it() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let durable = journal
        .append_kernel(vec![1], "turn-start", event(1.0))
        .expect("durable event");
    let hub = EndpointSubscriptionHub::default();
    // Replaying an already journaled event (boot repair) with nobody listening is a no-op.
    assert_eq!(
        hub.publish_event(SESSION, &journal, "replay-1", durable, None)
            .expect("publish without subscriber"),
        0
    );
    // The journal, not the hub, is the source of truth for a subscriber that arrives later.
    let subscription = hub
        .subscribe(SESSION, &journal, "late")
        .expect("late subscribe");
    assert_eq!(subscription.baseline().last_seq, 0);
    // With a subscriber the durability proof is enforced again.
    assert!(
        hub.publish_event(SESSION, &journal, "undurable", event(3.0), None)
            .is_err()
    );
}

#[test]
fn publishing_without_subscribers_still_refuses_an_event_beyond_the_journal_tail() {
    let folder = tempfile::tempdir().expect("thread folder");
    let journal = EndpointJournal::open(folder.path()).expect("journal");
    let hub = EndpointSubscriptionHub::default();
    // Empty journal: nothing is durable yet.
    assert!(matches!(
        hub.publish_event(SESSION, &journal, "none", event(1.0), None),
        Err(endpoint::HubError::NotDurable(99))
    ));
    journal
        .append_kernel(vec![1], "turn-start", event(1.0))
        .expect("durable event");
    // seq 99 is past the tail (last_seq is 0).
    assert!(matches!(
        hub.publish_event(SESSION, &journal, "beyond", event(2.0), None),
        Err(endpoint::HubError::NotDurable(99))
    ));
}

struct UnconfirmedThenConfirmedHost {
    calls: AtomicUsize,
    first_error_code: &'static str,
    first_error_message: &'static str,
    recovery_flags: Mutex<Vec<bool>>,
}

impl UnconfirmedThenConfirmedHost {
    fn new(first_error_code: &'static str, first_error_message: &'static str) -> Self {
        Self {
            calls: AtomicUsize::new(0),
            first_error_code,
            first_error_message,
            recovery_flags: Mutex::new(Vec::new()),
        }
    }
}

impl EndpointHost for UnconfirmedThenConfirmedHost {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.prompt".to_owned()].into_iter().collect()
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        let attempt = self.calls.fetch_add(1, Ordering::SeqCst);
        self.recovery_flags
            .lock()
            .expect("recovery flags")
            .push(request.recovering);
        if attempt == 0 {
            let details = if self.first_error_code == "accepted-but-not-confirmed" {
                format!(
                    r#"{{"operation":"{}","rpcId":"{}"}}"#,
                    request.operation, request.rpc_id
                )
            } else {
                "{}".to_owned()
            };
            return Box::pin(ready(RpcResult {
                ok: false,
                value: None,
                error: Some(endpoint::RpcError {
                    code: self.first_error_code.to_owned(),
                    message: self.first_error_message.to_owned(),
                    details: IJsonValue::parse_str(&details).expect("error details"),
                }),
            }));
        }
        request
            .handoff
            .mark_handed_off(DurableHandoffProof {
                delivery: "locked-append".to_owned(),
                durable_identity: Some(RpcDurableIdentity {
                    kind: "input".to_owned(),
                    id: request.rpc_id.clone(),
                    seq: Some(2),
                }),
            })
            .expect("handoff proof");
        Box::pin(ready(RpcResult {
            ok: true,
            value: Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("value")),
            error: None,
        }))
    }
}

#[test]
fn non_durable_unconfirmed_and_internal_errors_allow_exact_retry_and_reject_conflicts() {
    for (rpc_id, code, message) in [
        (
            "rpc-unconfirmed-retry",
            "accepted-but-not-confirmed",
            "Mutation was accepted but its receipt was not confirmed",
        ),
        (
            "rpc-internal-retry",
            "internal",
            "Endpoint operation failed",
        ),
    ] {
        let root = tempfile::tempdir().expect("root");
        let host = UnconfirmedThenConfirmedHost::new(code, message);
        let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("registry"));

        let first = block_on_ready(dispatcher.dispatch(&host, request(rpc_id, "same payload")))
            .expect("first dispatch");
        assert!(!first.result.ok);
        assert_eq!(first.result.error.as_ref().expect("err").code, code);

        // Conflicting payload on the still-unsettled rpcId must fail closed.
        let conflict =
            block_on_ready(dispatcher.dispatch(&host, request(rpc_id, "different payload")))
                .expect("conflict dispatch");
        assert_eq!(
            conflict.result.error.as_ref().expect("conflict").code,
            "idempotency-conflict"
        );
        assert_eq!(host.calls.load(Ordering::SeqCst), 1);

        // Exact retry across restart recovers and seals Phase::Complete.
        let reopened = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
        let retry = block_on_ready(reopened.dispatch(&host, request(rpc_id, "same payload")))
            .expect("exact retry");
        assert!(retry.result.ok, "exact retry must succeed: {retry:?}");
        assert_eq!(host.calls.load(Ordering::SeqCst), 2);
        assert_eq!(
            host.recovery_flags.lock().expect("flags").as_slice(),
            &[false, true]
        );

        // Once reconciled & completed, another retry hits cached completion without re-invoking host.
        let cached = block_on_ready(reopened.dispatch(&host, request(rpc_id, "same payload")))
            .expect("cached completion");
        assert_eq!(cached, retry);
        assert_eq!(host.calls.load(Ordering::SeqCst), 2);
    }
}

struct ArchivedRetryHost {
    archived: std::sync::atomic::AtomicBool,
    calls: AtomicUsize,
}

impl EndpointHost for ArchivedRetryHost {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.prompt".to_owned()].into_iter().collect()
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        let call_index = self.calls.fetch_add(1, Ordering::SeqCst);
        if call_index == 0 {
            return Box::pin(ready(RpcResult {
                ok: false,
                value: None,
                error: Some(endpoint::RpcError {
                    code: "accepted-but-not-confirmed".to_owned(),
                    message: "Mutation was accepted but its receipt was not confirmed".to_owned(),
                    details: IJsonValue::parse_str(&format!(
                        r#"{{"operation":"{}","rpcId":"{}"}}"#,
                        request.operation, request.rpc_id
                    ))
                    .expect("details"),
                }),
            }));
        }
        if self.archived.load(Ordering::SeqCst) {
            return Box::pin(ready(RpcResult {
                ok: false,
                value: None,
                error: Some(endpoint::RpcError {
                    code: "archived".to_owned(),
                    message: "Session is archived".to_owned(),
                    details: IJsonValue::parse_str(&format!(r#"{{"sessionId":"{SESSION}"}}"#))
                        .expect("archived details"),
                }),
            }));
        }
        request
            .handoff
            .mark_handed_off(DurableHandoffProof {
                delivery: "locked-append".to_owned(),
                durable_identity: Some(RpcDurableIdentity {
                    kind: "input".to_owned(),
                    id: request.rpc_id.clone(),
                    seq: Some(2),
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
fn archived_rejection_during_retry_does_not_poison_rpc_registry_after_unarchive() {
    let root = tempfile::tempdir().expect("root");
    let host = ArchivedRetryHost {
        archived: std::sync::atomic::AtomicBool::new(true),
        calls: AtomicUsize::new(0),
    };
    let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("registry"));

    let unconfirmed = block_on_ready(dispatcher.dispatch(&host, request("rpc-arch-1", "same")))
        .expect("unconfirmed");
    assert_eq!(
        unconfirmed.result.error.as_ref().expect("err").code,
        "accepted-but-not-confirmed"
    );

    let archived =
        block_on_ready(dispatcher.dispatch(&host, request("rpc-arch-1", "same"))).expect("arch");
    assert_eq!(
        archived.result.error.as_ref().expect("err").code,
        "archived"
    );

    let conflict = block_on_ready(dispatcher.dispatch(&host, request("rpc-arch-1", "different")))
        .expect("conflict");
    assert_eq!(
        conflict.result.error.as_ref().expect("conflict").code,
        "idempotency-conflict"
    );

    host.archived.store(false, Ordering::SeqCst);
    let reopened = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
    let recovered = block_on_ready(reopened.dispatch(&host, request("rpc-arch-1", "same")))
        .expect("unarchived retry");
    assert!(
        recovered.result.ok,
        "unarchived exact retry must reconcile original receipt: {recovered:?}"
    );
    assert_eq!(host.calls.load(Ordering::SeqCst), 3);

    let cached =
        block_on_ready(reopened.dispatch(&host, request("rpc-arch-1", "same"))).expect("cached");
    assert_eq!(cached, recovered);
    assert_eq!(host.calls.load(Ordering::SeqCst), 3);
}

struct LegacyReconcileHost {
    calls: AtomicUsize,
    probe_seq: Option<u64>,
    probes: AtomicUsize,
}

impl LegacyReconcileHost {
    fn new(probe_seq: Option<u64>) -> Self {
        Self {
            calls: AtomicUsize::new(0),
            probe_seq,
            probes: AtomicUsize::new(0),
        }
    }
}

impl EndpointHost for LegacyReconcileHost {
    fn capabilities(&self) -> BTreeSet<String> {
        ["session.prompt".to_owned()].into_iter().collect()
    }

    fn reconcile_legacy_internal_prompt(
        &self,
        _claim: &endpoint::RpcClaim,
        _request: &ClientRequest,
    ) -> Option<u64> {
        self.probes.fetch_add(1, Ordering::SeqCst);
        self.probe_seq
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(ready(RpcResult {
            ok: false,
            value: None,
            error: Some(endpoint::RpcError {
                code: "internal".to_owned(),
                message: format!("unexpected call for {}", request.rpc_id),
                details: IJsonValue::parse_str("{}").expect("empty details"),
            }),
        }))
    }
}

fn prompt_request(rpc_id: &str, session_id: &str, text: &str) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: "session.prompt".to_owned(),
        payload: IJsonValue::parse_str(&format!(
            r#"{{"content":[{{"text":"{text}","type":"text"}}],"mode":"queue","sessionId":"{session_id}"}}"#
        ))
        .expect("prompt payload"),
    }
}

fn seed_legacy_complete_internal(root: &std::path::Path, request: &ClientRequest) {
    let registry = RpcRegistry::open(root).expect("registry");
    let RpcBegin::Execute { claim, .. } = registry.begin(request).expect("prepare") else {
        panic!("expected pending claim");
    };
    registry
        .complete(
            &claim,
            &RpcResult {
                ok: false,
                value: None,
                error: Some(endpoint::RpcError {
                    code: "internal".to_owned(),
                    message: "Endpoint operation failed".to_owned(),
                    details: IJsonValue::parse_str("{}").expect("details"),
                }),
            },
        )
        .expect("legacy complete internal");
}

#[test]
fn legacy_internal_prompt_reconciles_to_accepted_without_reexecution() {
    let root = tempfile::tempdir().expect("root");
    let req = prompt_request("rpc-legacy-173", SESSION, "FOLLOWUP_MARKER");
    seed_legacy_complete_internal(root.path(), &req);

    let host = std::sync::Arc::new(LegacyReconcileHost::new(Some(9)));
    let dispatcher = std::sync::Arc::new(EndpointDispatcher::new(
        RpcRegistry::open(root.path()).expect("open"),
    ));

    let first = block_on_ready(dispatcher.dispatch(host.as_ref(), req.clone())).expect("first");
    assert!(first.result.ok, "expected accepted:true, got {first:?}");
    assert_eq!(
        first.result.value,
        Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("accepted"))
    );
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
    assert_eq!(host.probes.load(Ordering::SeqCst), 1);

    // Sequential & post-restart replays read the persisted ordinal-2 Complete row directly.
    let reopened = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
    let second = block_on_ready(reopened.dispatch(host.as_ref(), req.clone())).expect("reopen");
    assert_eq!(second, first);
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
    assert_eq!(host.probes.load(Ordering::SeqCst), 1);

    // Changed content or changed target session returns idempotency-conflict.
    let changed_text = prompt_request("rpc-legacy-173", SESSION, "CHANGED");
    let conflict_text =
        block_on_ready(reopened.dispatch(host.as_ref(), changed_text)).expect("conflict text");
    assert_eq!(
        conflict_text.result.error.as_ref().expect("err").code,
        "idempotency-conflict"
    );
    let other_session = "018f0000-0000-7000-8000-000000000099";
    let changed_session = prompt_request("rpc-legacy-173", other_session, "FOLLOWUP_MARKER");
    let conflict_session =
        block_on_ready(reopened.dispatch(host.as_ref(), changed_session)).expect("conflict sess");
    assert_eq!(
        conflict_session.result.error.as_ref().expect("err").code,
        "idempotency-conflict"
    );
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);

    // Torn ordinal-2 tail truncates cleanly back to ordinal 1 on open and reconciles again.
    let hash = format!("{:x}", Sha256::digest(b"rpc-legacy-173"));
    let rpc_path = root
        .path()
        .join("endpoint-rpc/rpc")
        .join(&hash[..2])
        .join(format!("{hash}.jsonl"));
    let original = std::fs::read_to_string(&rpc_path).expect("read carrier");
    let lines: Vec<&str> = original.lines().collect();
    assert_eq!(lines.len(), 3);
    let torn = format!("{}\n{}\n{{\"broken\":true", lines[0], lines[1]);
    std::fs::write(&rpc_path, torn).expect("write torn tail");

    let repaired = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("repair"));
    let after_repair =
        block_on_ready(repaired.dispatch(host.as_ref(), req)).expect("reconcile after repair");
    assert_eq!(after_repair, first);
    assert_eq!(host.probes.load(Ordering::SeqCst), 2);
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn concurrent_legacy_internal_prompt_replays_all_return_accepted_without_in_progress() {
    let root = tempfile::tempdir().expect("root");
    let req = prompt_request("rpc-legacy-concurrent", SESSION, "CONCURRENT");
    seed_legacy_complete_internal(root.path(), &req);

    let host = std::sync::Arc::new(LegacyReconcileHost::new(Some(11)));
    let dispatcher = std::sync::Arc::new(EndpointDispatcher::new(
        RpcRegistry::open(root.path()).expect("open"),
    ));

    let handles: Vec<_> = (0..6)
        .map(|_| {
            let dispatcher = std::sync::Arc::clone(&dispatcher);
            let host = std::sync::Arc::clone(&host);
            let req = req.clone();
            std::thread::spawn(move || block_on_ready(dispatcher.dispatch(host.as_ref(), req)))
        })
        .collect();
    for handle in handles {
        let resp = handle.join().expect("thread").expect("dispatch");
        assert!(resp.result.ok, "concurrent replay failed: {resp:?}");
        assert_eq!(
            resp.result.value,
            Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("accepted"))
        );
    }
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
    assert_eq!(host.probes.load(Ordering::SeqCst), 1);
}

#[test]
fn legacy_internal_prompt_without_evidence_remains_internal_and_never_reexecutes() {
    let root = tempfile::tempdir().expect("root");
    let req = prompt_request("rpc-legacy-missing", SESSION, "MISSING");
    seed_legacy_complete_internal(root.path(), &req);

    let host = LegacyReconcileHost::new(None);
    let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("open"));
    let resp = block_on_ready(dispatcher.dispatch(&host, req.clone())).expect("dispatch");
    assert!(!resp.result.ok);
    assert_eq!(resp.result.error.as_ref().expect("err").code, "internal");
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
    assert_eq!(host.probes.load(Ordering::SeqCst), 1);

    let retry = block_on_ready(dispatcher.dispatch(&host, req)).expect("retry");
    assert_eq!(retry, resp);
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
}

struct ToggleRpcSync {
    fail: AtomicBool,
    calls: AtomicUsize,
}

impl ToggleRpcSync {
    fn new(fail: bool) -> Self {
        Self {
            fail: AtomicBool::new(fail),
            calls: AtomicUsize::new(0),
        }
    }
}

impl store::SyncPolicy for ToggleRpcSync {
    fn full_sync(&self, file: &std::fs::File) -> std::io::Result<()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail.load(Ordering::Acquire) {
            return Err(std::io::Error::from(std::io::ErrorKind::Other));
        }
        store::FullSync::full_sync(file)
    }
}

#[test]
fn reconcile_rolls_back_and_blocks_replay_when_receipt_sync_fails() {
    let root = tempfile::tempdir().expect("root");
    let req = prompt_request("rpc-reconcile-sync-eio", SESSION, "RECEIPT_SYNC_EIO");
    seed_legacy_complete_internal(root.path(), &req);

    let hash = format!("{:x}", Sha256::digest(b"rpc-reconcile-sync-eio"));
    let rpc_path = root
        .path()
        .join("endpoint-rpc/rpc")
        .join(&hash[..2])
        .join(format!("{hash}.jsonl"));
    assert_eq!(
        std::fs::read_to_string(&rpc_path)
            .expect("initial rpc carrier")
            .lines()
            .count(),
        2
    );

    let sync = Arc::new(ToggleRpcSync::new(true));
    let registry = Arc::new(
        RpcRegistry::open_with_sync(root.path(), Arc::clone(&sync) as Arc<dyn store::SyncPolicy>)
            .expect("open registry with toggle sync"),
    );

    // First reconcile hits full_sync EIO: must error, roll back ordinal-2 row, and never return accepted.
    let err = registry
        .reconcile_legacy_internal_complete(&req, |_| Some(9))
        .expect_err("sync failure must abort reconcile");
    assert!(matches!(err, endpoint::RpcRegistryError::Io(_)));
    assert_eq!(
        std::fs::read_to_string(&rpc_path)
            .expect("after failed reconcile")
            .lines()
            .count(),
        2
    );

    // Repeated reconcile, lookup, and concurrent dispatch under continuous EIO must never accept.
    let err2 = registry
        .reconcile_legacy_internal_complete(&req, |_| Some(9))
        .expect_err("repeated reconcile must still fail on EIO");
    assert!(matches!(err2, endpoint::RpcRegistryError::Io(_)));
    let lookup_err = registry
        .lookup(&req)
        .expect_err("lookup under EIO must fail closed");
    assert!(matches!(lookup_err, endpoint::RpcRegistryError::Io(_)));

    let host = Arc::new(LegacyReconcileHost::new(Some(9)));
    let dispatcher = Arc::new(EndpointDispatcher::new(
        RpcRegistry::open_with_sync(root.path(), Arc::clone(&sync) as Arc<dyn store::SyncPolicy>)
            .expect("reopen under EIO"),
    ));
    let handles: Vec<_> = (0..6)
        .map(|_| {
            let d = Arc::clone(&dispatcher);
            let h = Arc::clone(&host);
            let r = req.clone();
            std::thread::spawn(move || block_on_ready(d.dispatch(h.as_ref(), r)))
        })
        .collect();
    for handle in handles {
        let dispatch_res = handle.join().expect("thread");
        assert!(
            dispatch_res.is_err(),
            "concurrent dispatch unexpectedly succeeded under EIO: {dispatch_res:?}"
        );
    }
    assert_eq!(
        std::fs::read_to_string(&rpc_path)
            .expect("still rolled back")
            .lines()
            .count(),
        2
    );

    // Clear EIO on same directory -> reconcile syncs and returns accepted:true, leaving 1 ordinal-2 row.
    sync.fail.store(false, Ordering::Release);
    let reconciled = block_on_ready(dispatcher.dispatch(host.as_ref(), req.clone()))
        .expect("reconcile after EIO clears");
    assert_eq!(
        reconciled.result.value,
        Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("accepted"))
    );
    let final_lines: Vec<_> = std::fs::read_to_string(&rpc_path)
        .expect("final rpc carrier")
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(final_lines.len(), 3);
    let row2: serde_json::Value = serde_json::from_str(&final_lines[2]).expect("row2");
    assert_eq!(row2["ordinal"], 2);
    assert_eq!(row2["delivery"], "reconciled-ledger");
    assert_eq!(row2["durable_identity"]["seq"], 9);

    let restarted = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("reopen"));
    let replay = block_on_ready(restarted.dispatch(host.as_ref(), req)).expect("replay after heal");
    assert_eq!(replay, reconciled);
    assert_eq!(host.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn unsynced_preexisting_complete_row_fails_closed_until_synced() {
    let root = tempfile::tempdir().expect("root");
    let req = prompt_request("rpc-unsynced-complete", SESSION, "UNSYNCED_COMPLETE");
    seed_legacy_complete_internal(root.path(), &req);

    // Reconcile successfully on disk so a valid 3-line ordinal-2 file exists before "crash/restart".
    let clean_registry = RpcRegistry::open(root.path()).expect("clean open");
    let accepted = clean_registry
        .reconcile_legacy_internal_complete(&req, |_| Some(9))
        .expect("clean reconcile")
        .expect("accepted");
    assert_eq!(
        accepted.value,
        Some(IJsonValue::parse_str(r#"{"accepted":true}"#).expect("accepted"))
    );
    drop(clean_registry);

    // Simulate restart onto that unverified carrier while EIO prevents full_sync:
    // begin(), lookup(), complete(), and reconcile_legacy_internal_complete() must fail closed.
    let sync = Arc::new(ToggleRpcSync::new(true));
    let failing_registry =
        RpcRegistry::open_with_sync(root.path(), Arc::clone(&sync) as Arc<dyn store::SyncPolicy>)
            .expect("open under EIO");
    assert!(matches!(
        failing_registry.begin(&req),
        Err(endpoint::RpcRegistryError::Io(_))
    ));
    assert!(matches!(
        failing_registry.lookup(&req),
        Err(endpoint::RpcRegistryError::Io(_))
    ));
    assert!(matches!(
        failing_registry.reconcile_legacy_internal_complete(&req, |_| Some(9)),
        Err(endpoint::RpcRegistryError::Io(_))
    ));

    // Once EIO clears, begin() verifies full_sync and exposes the durable accepted receipt.
    sync.fail.store(false, Ordering::Release);
    assert_eq!(
        failing_registry.begin(&req).expect("begin after sync"),
        RpcBegin::Completed(accepted)
    );
}
