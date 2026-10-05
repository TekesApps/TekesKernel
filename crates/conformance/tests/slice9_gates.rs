use std::collections::HashSet;
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

use endpoint::{
    AllSessionMux, CallContext, CarrierRespondHandler, ClientRequest, ClientResponse,
    ClientResponseResult, DrainSignal, DurableHandoffSignal, EndpointFrameQueue, EndpointJournal,
    EndpointSubscriptionHub, HostFailure, HostFrame, HostFrameKind, JournalRespondHandler,
    LocatedRespond, MuxFrame, MuxRegistration, NativeEndpoint, PendingRequest, RequestFrameType,
    RequestState, ResolutionOutcome, RespondAuthorReceipt, RespondAuthority, RespondAuthorization,
    RespondDecision, RespondDelivery, RespondLifecycle, RespondPrepareContext, RpcDurableIdentity,
    RpcRegistry, SessionEvent, StitchDecision, StreamErrorCode, StreamFailure, SubscriptionPoll,
    SurfaceOperation, history_page, stitch_window,
};
use schema::{IJsonValue, OriginTuple, ResumePolicy};

const SESSION_A: &str = "018f0000-0000-7000-8000-000000000071";
const SESSION_B: &str = "018f0000-0000-7000-8000-000000000072";

fn block_on_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("test future unexpectedly pending"),
    }
}

fn event(kind: &str, time: u64) -> SessionEvent {
    SessionEvent {
        event_type: kind.to_owned(),
        seq: 0,
        time: time as f64,
        data: IJsonValue::parse_str("{}").expect("event data"),
        ignorable: None,
        source_event_seqs: None,
        surface_op: None,
    }
}

fn append(journal: &EndpointJournal, kernel_seq: u64, kind: &str) -> SessionEvent {
    journal
        .append_kernel(
            vec![kernel_seq],
            format!("slot-{kernel_seq}"),
            event(kind, kernel_seq),
        )
        .expect("append endpoint event")
}

#[test]
fn slice9_gate_67_endpoint_subscribe_atomicity() {
    let root = tempfile::tempdir().expect("root");
    let folder_a = root.path().join(SESSION_A);
    let folder_b = root.path().join(SESSION_B);
    std::fs::create_dir_all(&folder_a).expect("folder a");
    std::fs::create_dir_all(&folder_b).expect("folder b");
    let journal_a = EndpointJournal::open(&folder_a).expect("journal a");
    let journal_b = EndpointJournal::open(&folder_b).expect("journal b");
    let before = append(&journal_a, 1, "turn/start");
    assert_eq!(before.seq, 0);

    let hub = EndpointSubscriptionHub::default();
    let mux = AllSessionMux::open(
        &hub,
        &[
            MuxRegistration {
                session_id: SESSION_A,
                journal: &journal_a,
                subscribed_rpc_id: "request-subscribe-a",
            },
            MuxRegistration {
                session_id: SESSION_B,
                journal: &journal_b,
                subscribed_rpc_id: "request-subscribe-b",
            },
        ],
    )
    .expect("atomic mux open");
    assert_eq!(mux.baselines()[0].last_seq, 0);
    assert_eq!(mux.baselines()[1].last_seq, -1);
    let (_handle, mut receiver) = mux.into_stream();
    assert_eq!(
        block_on_ready(receiver.recv())
            .expect("subscription a")
            .expect("frame a")
            .method,
        "session/subscribed"
    );
    assert_eq!(
        block_on_ready(receiver.recv())
            .expect("subscription b")
            .expect("frame b")
            .method,
        "session/subscribed"
    );

    // A pre-snapshot durable event is suppressed. An append that wins after
    // registration is delivered once, even if publish is retried.
    assert_eq!(
        hub.publish_event(SESSION_A, &journal_a, "request-old", before, None)
            .expect("publish old"),
        0
    );
    let after = append(&journal_a, 2, "step/start");
    assert_eq!(
        hub.publish_event(SESSION_A, &journal_a, "request-new", after.clone(), None)
            .expect("publish new"),
        1
    );
    assert_eq!(
        hub.publish_event(SESSION_A, &journal_a, "request-new-retry", after, None)
            .expect("retry publish"),
        0
    );
    let delivered = block_on_ready(receiver.recv())
        .expect("published event")
        .expect("event frame");
    assert_eq!(delivered.method, "session/event");

    // A zero-active inventory is an open all-session stream, not EOF. A later
    // session attach wakes the already-pending receiver with its baseline.
    let empty = AllSessionMux::open(&hub, &[]).expect("empty mux");
    let (empty_handle, mut empty_receiver) = empty.into_stream();
    let wake_count = Arc::new(WakeCount(AtomicUsize::new(0)));
    let waker = Waker::from(Arc::clone(&wake_count));
    let mut context = Context::from_waker(&waker);
    {
        let mut receive = empty_receiver.recv();
        assert!(matches!(receive.as_mut().poll(&mut context), Poll::Pending));
        empty_handle
            .attach(&hub, SESSION_B, &journal_b, "request-empty-attach")
            .expect("attach to empty mux");
        assert!(wake_count.0.load(Ordering::SeqCst) > 0);
        assert!(matches!(
            receive.as_mut().poll(&mut context),
            Poll::Ready(Some(Ok(_)))
        ));
    }
    empty_handle.close().expect("close empty generation");
    assert!(block_on_ready(empty_receiver.recv()).is_none());
}

#[test]
fn slice9_gate_68_endpoint_raw_history_reconnect() {
    let root = tempfile::tempdir().expect("root");
    let journal = EndpointJournal::open(root.path()).expect("journal");
    let turn = append(&journal, 1, "turn/start");
    let mut message = event("assistant/message", 2);
    message.surface_op = Some(SurfaceOperation::Append("append".to_owned()));
    let message = journal
        .append_kernel(vec![3], "assistant-message", message)
        .expect("message");

    let records = journal.records().expect("records");
    let page = history_page(&records, None, Some(1)).expect("history page");
    let events = page
        .events
        .into_iter()
        .map(|entry| entry.event)
        .collect::<Vec<_>>();
    assert_eq!(events, vec![message.clone()]);
    assert_eq!(
        stitch_window(events.clone(), vec![message.clone()], 1).expect("identical overlap"),
        StitchDecision::Installed(events)
    );
    assert_eq!(
        stitch_window(vec![turn], Vec::new(), 1).expect("baseline ahead"),
        StitchDecision::RefetchTail
    );
    assert_eq!(
        stitch_window(Vec::new(), vec![message], 1).expect("live gap"),
        StitchDecision::RefetchTail
    );
}

#[test]
fn slice9_gate_69_endpoint_drain_backpressure_security() {
    let root = tempfile::tempdir().expect("root");
    let journal = EndpointJournal::open(root.path()).expect("journal");
    let hub = EndpointSubscriptionHub::default();
    let subscription = hub
        .subscribe_with_bounds(SESSION_A, &journal, "request-subscribe", 1, 64 * 1024)
        .expect("subscribe");
    assert!(matches!(
        subscription.poll().expect("baseline"),
        SubscriptionPoll::Frame(_)
    ));
    hub.publish_frame(
        SESSION_A,
        "request-queue",
        MuxFrame::Queue {
            session_id: SESSION_A.to_owned(),
            items: Vec::new(),
        },
    )
    .expect("first frame");
    assert_eq!(
        hub.publish_frame(
            SESSION_A,
            "request-projection",
            MuxFrame::Projection {
                session_id: SESSION_A.to_owned(),
                key: "title".to_owned(),
                value: IJsonValue::parse_str(r#""slow""#).expect("value"),
                seq: 0,
            },
        )
        .expect("overflow is terminal, not a middle drop"),
        0
    );
    assert!(matches!(
        subscription.poll().expect("prefix"),
        SubscriptionPoll::Frame(_)
    ));
    assert!(matches!(
        subscription.poll().expect("gap"),
        SubscriptionPoll::LiveGap
    ));
    assert!(matches!(
        subscription.poll().expect("closed"),
        SubscriptionPoll::Closed
    ));

    let queue = EndpointFrameQueue::new(2, 64 * 1024).expect("host queue");
    let draining = HostFrame::new(
        HostFrameKind::StreamError,
        IJsonValue::parse_str(
            r#"{"error":{"code":"server-draining","details":{},"message":"Server is draining"},"type":"stream/error"}"#,
        )
        .expect("drain payload"),
    )
    .expect("drain frame")
    .into_envelope("request-drain")
    .expect("drain envelope");
    queue.push(draining).expect("queue drain frame");
    queue.close().expect("close after terminal frame");
    let mut receiver = queue.receiver();
    assert_eq!(
        block_on_ready(receiver.recv())
            .expect("terminal frame")
            .expect("frame")
            .method,
        "stream/error"
    );
    assert!(block_on_ready(receiver.recv()).is_none());

    let failed = EndpointFrameQueue::new(1, 1024).expect("failure queue");
    failed
        .fail(StreamFailure::new(StreamErrorCode::LiveGap))
        .expect("typed failure");
    let mut failure_receiver = failed.receiver();
    let failure = block_on_ready(failure_receiver.recv())
        .expect("failure item")
        .expect_err("typed stream failure");
    assert_eq!(failure.code(), StreamErrorCode::LiveGap);
    assert_eq!(failure.code().close_code(), 1013);

    // Carrier handoff state is independent of socket completion.
    let handoff = DurableHandoffSignal::new();
    let context = CallContext {
        response_deadline: Instant::now() + Duration::from_secs(1),
        drain: DrainSignal::new(Arc::new(AtomicBool::new(true))),
        handoff: handoff.clone(),
    };
    assert!(context.drain.is_draining());
    handoff.mark_durable();
    assert!(context.handoff.is_durable());
}

#[test]
fn slice9_gate_70_endpoint_archive_client_rematerialization() {
    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    let origin = OriginTuple {
        principal: "uid:501".to_owned(),
        client: "session-endpoint".to_owned(),
        target: SESSION_A.to_owned(),
        op: "session.create".to_owned(),
        key: "create-a".to_owned(),
    };
    endpoint
        .create_session_for_endpoint(
            SESSION_A,
            "workspace-a",
            &"0".repeat(64),
            ResumePolicy::Never,
            "2026-08-28T00:00:00.000Z",
            &origin,
        )
        .expect("create");
    let baseline = endpoint
        .reconcile_projection(SESSION_A)
        .expect("projection");
    let active = root.path().join("threads").join(SESSION_A);
    let journal = EndpointJournal::open(&active).expect("journal");
    let hub = EndpointSubscriptionHub::default();
    let mux = AllSessionMux::open(
        &hub,
        &[MuxRegistration {
            session_id: SESSION_A,
            journal: &journal,
            subscribed_rpc_id: "request-before-archive",
        }],
    )
    .expect("mux");
    drop(journal);
    let (handle, mut receiver) = mux.into_stream();
    assert!(handle.detach_for_archive(SESSION_A).expect("detach"));
    endpoint.archive_session(SESSION_A).expect("archive");
    assert!(endpoint.history(SESSION_A, None, None).is_err());
    endpoint.unarchive_session(SESSION_A).expect("unarchive");
    let rematerialized = endpoint
        .reconcile_projection(SESSION_A)
        .expect("rematerialize");
    assert_eq!(rematerialized, baseline);
    let reopened = EndpointJournal::open(&active).expect("reopened journal");
    let attached = handle
        .attach(&hub, SESSION_A, &reopened, "request-after-unarchive")
        .expect("reattach");
    assert_eq!(attached.last_seq, baseline);
    assert_eq!(
        block_on_ready(receiver.recv())
            .expect("unarchive subscribed")
            .expect("subscribed frame")
            .method,
        "session/subscribed"
    );
    assert!(
        !endpoint
            .list_sessions(&HashSet::new())
            .expect("inventory")
            .into_iter()
            .find(|item| item.session_id == SESSION_A)
            .expect("session")
            .archived
    );
}

#[test]
fn slice9_respond_request_is_derived_from_the_hold_and_resolved_by_the_ledger() {
    let pending = PendingRequest::derive(
        SESSION_A,
        None,
        RequestFrameType::Approval,
        10,
        IJsonValue::parse_str(&format!(
            r#"{{"approvalId":"approval-a","callId":"call-a","sessionId":"{SESSION_A}","toolName":"shell","type":"approval/requested"}}"#
        ))
        .expect("approval frame"),
    )
    .expect("derive request");
    assert_eq!(
        pending.rpc_id,
        endpoint::derive_request_rpc_id(SESSION_A, RequestFrameType::Approval, 10)
    );
    let response = ClientResponse {
        envelope_type: "client-response".to_owned(),
        rpc_id: pending.rpc_id.clone(),
        result: ClientResponseResult {
            ok: true,
            value: Some(
                IJsonValue::parse_str(&format!(
                    r#"{{"approvalId":"approval-a","outcome":"allowed-once","sessionId":"{SESSION_A}"}}"#
                ))
                .expect("response value"),
            ),
            error: None,
        },
    };
    let context = RespondPrepareContext {
        archived: false,
        stop_active: false,
        held_call: None,
    };
    let unresolved = RequestState {
        request: pending.clone(),
        resolution: None,
    };
    let RespondDecision::Authorize(authorization) =
        RespondLifecycle::prepare(Some(&unresolved), &response, &context).expect("prepare")
    else {
        panic!("expected authorization");
    };
    assert_eq!(authorization.call, "call-a");
    assert!(authorization.grant);
    // The durable approval_response is the resolution: once the ledger
    // carries it, the same request is already resolved.
    let resolved = RequestState::resolved(pending, 11, ResolutionOutcome::AllowedOnce)
        .expect("ledger resolution");
    let RespondDecision::Reject(receipt) =
        RespondLifecycle::prepare(Some(&resolved), &response, &context).expect("prepare resolved")
    else {
        panic!("expected rejection");
    };
    assert_eq!(receipt.reason.as_deref(), Some("already-resolved"));
    assert_eq!(
        serde_json_canonicalizer::to_string(&endpoint::RespondReceipt {
            accepted: true,
            reason: None
        })
        .expect("receipt bytes"),
        r#"{"accepted":true}"#
    );
}

#[test]
fn slice9_respond_exact_retry_recovers_from_both_commit_windows() {
    let root = tempfile::tempdir().expect("root");
    let (request, response) = approval_request(20);
    let authors = Arc::new(AtomicUsize::new(0));
    let handler = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: root.path().to_path_buf(),
            request: request.clone(),
            resolved: Arc::new(AtomicBool::new(false)),
            authors: Arc::clone(&authors),
            located: true,
            archived: false,
        },
        RpcRegistry::open(root.path()).expect("rpc registry"),
    );
    let first_context = call_context();
    let first_handoff = first_context.handoff.clone();
    let first = block_on_ready(handler.respond(response.clone(), first_context)).expect("first");
    assert!(first.accepted);
    assert!(first_handoff.is_durable());
    assert_eq!(authors.load(Ordering::SeqCst), 1);
    let rpc_files = rpc_record_files(root.path());
    assert_eq!(rpc_files.len(), 1);
    let lines = std::fs::read_to_string(&rpc_files[0]).expect("rpc carrier bytes");
    let rows = lines
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("rpc row"))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["v"], 2);
    assert_eq!(rows[0]["ordinal"], 0);
    assert_eq!(rows[0]["phase"], "prepared");
    assert_eq!(rows[0]["operation"], "respond");
    assert_eq!(rows[0]["target_session"], SESSION_A);
    assert_eq!(rows[1]["ordinal"], 1);
    assert_eq!(rows[1]["phase"], "handed-off");
    assert_eq!(rows[1]["delivery"], "supervisor-locked-append");
    assert_eq!(rows[1]["durable_identity"]["kind"], "event");
    assert_eq!(rows[1]["durable_identity"]["id"], SESSION_A);
    assert_eq!(rows[1]["durable_identity"]["seq"], 99);
    assert_eq!(rows[2]["ordinal"], 2);
    assert_eq!(rows[2]["phase"], "complete");
    assert_eq!(rows[2]["delivery"], "supervisor-locked-append");
    assert_eq!(rows[2]["response_b64"], "eyJhY2NlcHRlZCI6dHJ1ZX0=");
    drop(handler);

    let reopened = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: root.path().to_path_buf(),
            request: request.clone(),
            resolved: Arc::new(AtomicBool::new(true)),
            authors: Arc::clone(&authors),
            located: true,
            archived: false,
        },
        RpcRegistry::open(root.path()).expect("reopen registry"),
    );
    let retry =
        block_on_ready(reopened.respond(response.clone(), call_context())).expect("exact retry");
    assert_eq!(retry, first);
    assert_eq!(authors.load(Ordering::SeqCst), 1);

    let mut conflicting = response;
    conflicting.result.value = Some(
        IJsonValue::parse_str(&format!(
            r#"{{"approvalId":"approval-a","outcome":"rejected","sessionId":"{SESSION_A}"}}"#
        ))
        .expect("conflicting value"),
    );
    let rejected = block_on_ready(reopened.respond(conflicting, call_context()))
        .expect("resolved conflict is a bare rejection");
    assert!(!rejected.accepted);
    assert_eq!(rejected.reason.as_deref(), Some("already-resolved"));
    assert_eq!(authors.load(Ordering::SeqCst), 1);

    // Crash after durable prepared row and before semantic authoring.
    let prepared = tempfile::tempdir().expect("prepared root");
    let (prepared_request, prepared_response) = approval_request(25);
    let prepared_registry = RpcRegistry::open(prepared.path()).expect("prepared registry");
    assert!(matches!(
        prepared_registry
            .begin(&respond_claim(&prepared_response))
            .expect("prepare carrier"),
        endpoint::RpcBegin::Execute {
            recovering: false,
            ..
        }
    ));
    drop(prepared_registry);
    let prepared_file = rpc_record_files(prepared.path())
        .into_iter()
        .next()
        .expect("prepared rpc file");
    let hash = prepared_file
        .file_stem()
        .and_then(|value| value.to_str())
        .expect("rpc hash");
    let temp_file = prepared_file.with_file_name(format!(".{hash}.tmp"));
    std::fs::rename(&prepared_file, &temp_file).expect("simulate crash before publish rename");
    let prepared_authors = Arc::new(AtomicUsize::new(0));
    let prepared_handler = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: prepared.path().to_path_buf(),
            request: prepared_request,
            resolved: Arc::new(AtomicBool::new(false)),
            authors: Arc::clone(&prepared_authors),
            located: true,
            archived: false,
        },
        RpcRegistry::open(prepared.path()).expect("recover prepared registry"),
    );
    assert!(
        block_on_ready(prepared_handler.respond(prepared_response, call_context()))
            .expect("recover prepared respond")
            .accepted
    );
    assert_eq!(prepared_authors.load(Ordering::SeqCst), 1);

    // Crash after the semantic event handoff row and before the request
    // resolution: recovery re-drives the same origin and proof, then resolves.
    let handed_off = tempfile::tempdir().expect("handed-off root");
    let (handed_off_request, handed_off_response) = approval_request(28);
    let handed_off_registry = RpcRegistry::open(handed_off.path()).expect("rpc registry");
    let endpoint::RpcBegin::Execute { claim, .. } = handed_off_registry
        .begin(&respond_claim(&handed_off_response))
        .expect("prepared")
    else {
        panic!("expected pending respond carrier");
    };
    handed_off_registry
        .mark_handed_off(
            &claim,
            "supervisor-locked-append",
            Some(RpcDurableIdentity {
                kind: "event".to_owned(),
                id: SESSION_A.to_owned(),
                seq: Some(99),
            }),
        )
        .expect("semantic handoff");
    drop(handed_off_registry);
    let handed_off_authors = Arc::new(AtomicUsize::new(0));
    let handed_off_handler = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: handed_off.path().to_path_buf(),
            request: handed_off_request,
            resolved: Arc::new(AtomicBool::new(false)),
            authors: Arc::clone(&handed_off_authors),
            located: true,
            archived: false,
        },
        RpcRegistry::open(handed_off.path()).expect("recover rpc registry"),
    );
    assert!(
        block_on_ready(handed_off_handler.respond(handed_off_response, call_context()))
            .expect("recover handed-off respond")
            .accepted
    );
    assert_eq!(handed_off_authors.load(Ordering::SeqCst), 1);

    // Crash after the semantic barrier (the ledger already carries the
    // approval_response) but before rpc completion: the next process
    // completes the pending carrier from that resolution without authoring.
    let crash = tempfile::tempdir().expect("crash root");
    let (crash_request, crash_response) = approval_request(30);
    let claim_request = respond_claim(&crash_response);
    let registry = RpcRegistry::open(crash.path()).expect("crash registry");
    let endpoint::RpcBegin::Execute { claim, .. } =
        registry.begin(&claim_request).expect("prepared")
    else {
        panic!("expected pending respond carrier");
    };
    registry
        .mark_handed_off(
            &claim,
            "supervisor-locked-append",
            Some(RpcDurableIdentity {
                kind: "event".to_owned(),
                id: SESSION_A.to_owned(),
                seq: Some(31),
            }),
        )
        .expect("semantic handoff");
    drop(registry);
    let crash_authors = Arc::new(AtomicUsize::new(0));
    let recovered = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: crash.path().to_path_buf(),
            request: crash_request,
            resolved: Arc::new(AtomicBool::new(true)),
            authors: Arc::clone(&crash_authors),
            located: true,
            archived: false,
        },
        RpcRegistry::open(crash.path()).expect("recover registry"),
    );
    assert!(
        block_on_ready(recovered.respond(crash_response, call_context()))
            .expect("recover exact receipt")
            .accepted
    );
    assert_eq!(crash_authors.load(Ordering::SeqCst), 0);

    let negative = tempfile::tempdir().expect("negative root");
    let (negative_request, negative_response) = approval_request(40);
    let negative_authors = Arc::new(AtomicUsize::new(0));
    let negative_handler = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: negative.path().to_path_buf(),
            request: negative_request.clone(),
            resolved: Arc::new(AtomicBool::new(false)),
            authors: Arc::clone(&negative_authors),
            located: true,
            archived: true,
        },
        RpcRegistry::open(negative.path()).expect("negative registry"),
    );
    let archived =
        block_on_ready(negative_handler.respond(negative_response.clone(), call_context()))
            .expect("archived rejection");
    assert_eq!(archived.reason.as_deref(), Some("archived"));
    assert_eq!(negative_authors.load(Ordering::SeqCst), 0);
    assert_eq!(rpc_record_count(negative.path()), 0);

    let unknown = tempfile::tempdir().expect("unknown root");
    let unknown_handler = JournalRespondHandler::new(
        MockRespondAuthority {
            thread_folder: unknown.path().to_path_buf(),
            request: negative_request,
            resolved: Arc::new(AtomicBool::new(false)),
            authors: Arc::new(AtomicUsize::new(0)),
            located: false,
            archived: false,
        },
        RpcRegistry::open(unknown.path()).expect("unknown registry"),
    );
    let unknown_receipt =
        block_on_ready(unknown_handler.respond(negative_response, call_context()))
            .expect("unknown rejection");
    assert_eq!(unknown_receipt.reason.as_deref(), Some("unknown-rpc-id"));
    assert_eq!(rpc_record_count(unknown.path()), 0);
}

fn rpc_record_count(root: &std::path::Path) -> usize {
    rpc_record_files(root).len()
}

fn rpc_record_files(root: &std::path::Path) -> Vec<PathBuf> {
    let rpc = root.join("endpoint-management/rpc");
    std::fs::read_dir(rpc)
        .expect("rpc root")
        .flat_map(|shard| {
            std::fs::read_dir(shard.expect("shard").path())
                .expect("shard entries")
                .map(|entry| entry.expect("rpc entry").path())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn approval_request(causal_seq: u64) -> (PendingRequest, ClientResponse) {
    let pending = PendingRequest::derive(
        SESSION_A,
        None,
        RequestFrameType::Approval,
        causal_seq,
        IJsonValue::parse_str(&format!(
            r#"{{"approvalId":"approval-a","callId":"call-a","sessionId":"{SESSION_A}","toolName":"shell","type":"approval/requested"}}"#
        ))
        .expect("approval frame"),
    )
    .expect("derive request");
    let response = ClientResponse {
        envelope_type: "client-response".to_owned(),
        rpc_id: pending.rpc_id.clone(),
        result: ClientResponseResult {
            ok: true,
            value: Some(
                IJsonValue::parse_str(&format!(
                    r#"{{"approvalId":"approval-a","outcome":"allowed-once","sessionId":"{SESSION_A}"}}"#
                ))
                .expect("response value"),
            ),
            error: None,
        },
    };
    (pending, response)
}

fn respond_claim(response: &ClientResponse) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: response.rpc_id.clone(),
        method: "respond".to_owned(),
        payload: IJsonValue::parse(
            &serde_json_canonicalizer::to_vec(response).expect("response canonical bytes"),
        )
        .expect("response claim value"),
    }
}

fn call_context() -> CallContext {
    CallContext {
        response_deadline: Instant::now() + Duration::from_secs(1),
        drain: DrainSignal::new(Arc::new(AtomicBool::new(false))),
        handoff: DurableHandoffSignal::new(),
    }
}

struct MockRespondAuthority {
    thread_folder: PathBuf,
    /// The request the mock's ledger holds; `resolved` is the ledger's
    /// approval_response at seq 99 (set once authored).
    request: PendingRequest,
    resolved: Arc<AtomicBool>,
    authors: Arc<AtomicUsize>,
    located: bool,
    archived: bool,
}

impl RespondAuthority for MockRespondAuthority {
    fn locate(&self, rpc_id: &str) -> Result<Option<LocatedRespond>, HostFailure> {
        if !self.located || rpc_id != self.request.rpc_id {
            return Ok(None);
        }
        let state = if self.resolved.load(Ordering::SeqCst) {
            RequestState::resolved(self.request.clone(), 99, ResolutionOutcome::AllowedOnce)
                .expect("mock resolution")
        } else {
            RequestState {
                request: self.request.clone(),
                resolution: None,
            }
        };
        Ok(Some(LocatedRespond {
            thread_folder: self.thread_folder.clone(),
            state,
            context: RespondPrepareContext {
                archived: self.archived,
                stop_active: false,
                held_call: None,
            },
            admission: mock_respond_admission(),
        }))
    }

    fn author(
        &self,
        _authorization: RespondAuthorization,
    ) -> Result<RespondAuthorReceipt, HostFailure> {
        self.authors.fetch_add(1, Ordering::SeqCst);
        self.resolved.store(true, Ordering::SeqCst);
        Ok(RespondAuthorReceipt {
            semantic_seq: 99,
            delivery: RespondDelivery::LockedAppend,
        })
    }
}

fn mock_respond_admission() -> Arc<Mutex<()>> {
    static ADMISSION: OnceLock<Arc<Mutex<()>>> = OnceLock::new();
    Arc::clone(ADMISSION.get_or_init(|| Arc::new(Mutex::new(()))))
}

struct WakeCount(AtomicUsize);

impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
