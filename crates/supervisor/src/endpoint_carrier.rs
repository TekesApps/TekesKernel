//! Production Session Endpoint carrier assembly.
//!
//! This module owns the supervisor-side process/durable authority required by
//! the endpoint carrier. Physical HTTP/WebSocket mechanics remain in
//! `transport`; DTO validation, request derivation, and mux buffering remain in
//! `endpoint`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::time::{SystemTime, UNIX_EPOCH};

use endpoint::{
    AllSessionMux, AllSessionMuxHandle, CarrierHostFuture, CarrierStreamHandler,
    ComposedEndpointCarrierHost, EndpointCarrierHost, EndpointFrameQueue, EndpointHost,
    EndpointHostCall, EndpointHostFuture, EndpointJournal, EndpointStream, EndpointStreamReceiver,
    EndpointSubscriptionHub, HostFailure, HostFrame, HostFrameKind, HostReadiness,
    JournalRespondHandler, LocatedRespond, MuxHostDescription, MuxReplayRegistration,
    PendingRequest, RequestFrameType, RequestState, RespondAuthorReceipt, RespondAuthority,
    RespondAuthorization, RespondDelivery, RespondPrepareContext, RpcRegistry, ServerRequest,
    SessionActionable, SessionActionableKind, SessionAddress, SessionControlItem,
    SessionJournalPage, SessionJournalSnapshot, SessionMuxClientFrame, SessionStream,
    SessionStreamReceiver, SessionStreamTarget, SessionSummary, SessionSyncFrame, StreamChannel,
    StreamErrorCode, StreamFailure, WorkspaceBaseline, WorkspaceSummary,
};
use engine::{LockFacts, classify};
use schema::{Event, EventKind, IJsonValue, OriginTuple};
use serde_json::{Value, json};
use store::{StoreError, ThreadStore, scan_valid_prefix};
use thiserror::Error;
use transport::{TransportConfig, TransportConfigError, TransportServer};
use worker_control::{ApprovalResponse, Receipt};

use crate::endpoint_host::{ProductionEndpointHost, ProductionRouteFailure, SessionAdmissionGates};

const HOST_STREAM_MAX_FRAMES: usize = 4_096;
const HOST_STREAM_MAX_BYTES: usize = 4 * 1024 * 1024;

/// The supervisor-side facts the carrier needs from the process host.
///
/// `live_sessions` is live-worker evidence from the process table (D-2's
/// routing cache). Inventory `running` is the same disjunction the supervisor
/// sweep classifies with: an owned live worker or a busy line lock. Without it
/// the flag is a bare lock probe taken at list time, which misses a worker
/// between its spawn and its lock acquisition and never changes once the
/// baseline is sent.
///
/// `reconcile_projection` runs the supervisor's single endpoint-projection path
/// for one session so a journal stream opens on a tail that already reflects
/// the semantic ledger (tail-lifecycle audit invariant I3).
pub trait SupervisorSessionAuthority: Send + Sync {
    /// Session ids that currently own a live worker on any of their lines.
    fn live_sessions(&self) -> HashSet<String>;

    /// Projects any semantic events the endpoint journal is missing. A no-op
    /// while a live worker owns the main line: its doorbell orders projection
    /// against the frames still queued in its control pipe.
    fn reconcile_projection(&self, session_id: &str) -> Result<(), String>;
}

/// Process-table authority for the live-worker half of `respond`. `None`
/// means no worker currently owns the session line. Implementations revalidate
/// the named unresolved hold and stop state before sending, and return only
/// after the worker's approval-response barrier receipt is durable.
pub trait LiveRespondAuthority: Send + Sync {
    fn deliver_if_live(
        &self,
        session_id: &str,
        response: &ApprovalResponse,
    ) -> Result<Option<Receipt>, ProductionRouteFailure>;

    /// Re-runs tail triage after the carrier durably appends an answer while
    /// no live worker owns the line. Slice-9 test seams may keep the no-op
    /// default; the production daemon ensures the resume candidate here.
    fn ensure_after_locked_append(&self, _session_id: &str) -> Result<(), ProductionRouteFailure> {
        Ok(())
    }
}

pub type CarrierClock = dyn Fn() -> Result<String, HostFailure> + Send + Sync;

pub struct ProductionRespondAuthority {
    root: PathBuf,
    store: ThreadStore,
    admission: Arc<SessionAdmissionGates>,
    live: Arc<dyn LiveRespondAuthority>,
    clock: Arc<CarrierClock>,
    principal: String,
    /// The actionable stream owner, when this authority serves a carrier
    /// assembly. A durable response is the moment the request stops being
    /// pending; subscribers learn it here instead of at the next worker
    /// doorbell, which inside a turn only rings at settle.
    streams: Option<ProductionCarrierStreams>,
}

impl ProductionRespondAuthority {
    pub fn open(
        root: impl AsRef<Path>,
        admission: Arc<SessionAdmissionGates>,
        live: Arc<dyn LiveRespondAuthority>,
    ) -> Result<Self, ProductionCarrierError> {
        Self::open_with_clock(root, admission, live, Arc::new(system_timestamp))
    }

    pub fn open_with_clock(
        root: impl AsRef<Path>,
        admission: Arc<SessionAdmissionGates>,
        live: Arc<dyn LiveRespondAuthority>,
        clock: Arc<CarrierClock>,
    ) -> Result<Self, ProductionCarrierError> {
        let root = root.as_ref().to_path_buf();
        // SAFETY: geteuid has no preconditions and is sampled once for the
        // immutable endpoint principal.
        let principal = format!("uid:{}", unsafe { libc::geteuid() });
        Ok(Self {
            store: ThreadStore::open(&root)?,
            root,
            admission,
            live,
            clock,
            principal,
            streams: None,
        })
    }

    /// Lets a durable response re-diff the actionable stream immediately.
    #[must_use]
    pub fn with_streams(mut self, streams: ProductionCarrierStreams) -> Self {
        self.streams = Some(streams);
        self
    }

    /// The response is durable, so the held request is no longer pending:
    /// publish that now. The worker's `appended` doorbell inside a turn only
    /// rings at settle, which left the Client's approval card standing for the
    /// rest of the turn (validator rounds included). Failure here is logged,
    /// never surfaced: the response itself already succeeded and the next
    /// doorbell or baseline reconciles the stream anyway.
    fn reconcile_after_response(&self, session_key: &str) {
        let Some(streams) = &self.streams else {
            return;
        };
        let (session, line) = session_key
            .split_once(':')
            .map_or((session_key, None), |(session, line)| {
                (session, Some(format!("{line}.jsonl")))
            });
        if let Err(error) = streams.reconcile_actionables(session, line.as_deref()) {
            eprintln!("respond-reconcile-actionables-failed: {error}");
        }
    }

    fn locate_once(&self, rpc_id: &str) -> Result<Option<LocatedRespond>, HostFailure> {
        let mut found = None;
        for (area, archived) in [("threads", false), ("archive", true)] {
            let mut folders = fs::read_dir(self.root.join(area))
                .map_err(internal)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(internal)?;
            folders.sort_by_key(std::fs::DirEntry::file_name);
            for entry in folders {
                if !entry.file_type().map_err(internal)?.is_dir() {
                    continue;
                }
                let folder = entry.path();
                let session = entry.file_name().to_string_lossy().into_owned();
                if endpoint::validate_session_id(&session).is_err() {
                    continue;
                }
                let Some(state) = session_requests(&folder, &session)?
                    .into_iter()
                    .find(|state| state.request.rpc_id == rpc_id)
                else {
                    continue;
                };
                if found.is_some() {
                    return Err(HostFailure::Internal(
                        "respond rpcId is present in multiple sessions".to_owned(),
                    ));
                }
                let (projection, ancestor_stopped) = actionable_projection(
                    &folder,
                    &state.request.session_id,
                    state.request.source_line.as_deref(),
                )?;
                let held_call = validate_request_binding(
                    &projection.events,
                    state.request.causal_kernel_seq,
                    state.request.frame_type,
                    &state.request.envelope.payload,
                )?;
                let admission = self
                    .admission
                    .gate(&state.request.session_id)
                    .map_err(route_failure)?;
                found = Some(LocatedRespond {
                    thread_folder: folder,
                    state,
                    context: RespondPrepareContext {
                        archived,
                        stop_active: projection.lifecycle.stop_active || ancestor_stopped,
                        held_call,
                    },
                    admission,
                });
            }
        }
        Ok(found)
    }

    fn worker_message(&self, authorization: &RespondAuthorization) -> ApprovalResponse {
        ApprovalResponse {
            delivery: authorization.origin_key.clone(),
            origin: OriginTuple {
                principal: self.principal.clone(),
                client: endpoint::ORIGIN_CLIENT.to_owned(),
                target: authorization.session_id.clone(),
                op: "respond".to_owned(),
                key: authorization.origin_key.clone(),
            },
            call: authorization.call.clone(),
            grant: authorization.grant,
            answer: authorization.answer.clone(),
        }
    }

    fn live_delivery(
        &self,
        authorization: &RespondAuthorization,
        message: &ApprovalResponse,
    ) -> Result<Option<RespondAuthorReceipt>, HostFailure> {
        let receipt = self
            .live
            .deliver_if_live(&authorization.session_id, message)
            .map_err(route_failure)?;
        let Some(receipt) = receipt else {
            return Ok(None);
        };
        if receipt.delivery != message.delivery || receipt.seq == 0 {
            return Err(HostFailure::Internal(
                "live respond returned an invalid durable receipt".to_owned(),
            ));
        }
        Ok(Some(RespondAuthorReceipt {
            semantic_seq: receipt.seq,
            delivery: RespondDelivery::LiveWorker,
        }))
    }

    fn locked_append(
        &self,
        authorization: &RespondAuthorization,
        message: &ApprovalResponse,
    ) -> Result<RespondAuthorReceipt, HostFailure> {
        let timestamp = (self.clock)()?;
        // The process host uses a private session:line key for child workers.
        // Public actionable session identities remain the root session UUID.
        let (session, line) = authorization.session_id.split_once(':').map_or(
            (authorization.session_id.as_str(), None),
            |(session, line)| (session, Some(line)),
        );
        let line_file =
            line.map_or_else(|| "main.jsonl".to_owned(), |line| format!("{line}.jsonl"));
        let outcome = self.store.append_line_keyed_with_projection_if(
            session,
            &line_file,
            &message.origin,
            |seq, projection| {
                if projection.lifecycle.stop_active {
                    return Err(StoreError::Corruption(
                        "stop became active before respond append".to_owned(),
                    ));
                }
                let turn = unresolved_hold_turn(&projection.events, &authorization.call)?;
                let value = json!({
                    "v":1,
                    "seq":seq,
                    "ts":timestamp,
                    "kind":"approval_response",
                    "turn":turn,
                    "call":authorization.call,
                    "grant":authorization.grant,
                    "origin_key":message.origin.key,
                    "origin_tuple":message.origin,
                });
                let mut object = value.as_object().expect("literal is object").clone();
                if let Some(answer) = authorization.answer.as_ref() {
                    object.insert(
                        "answer".to_owned(),
                        serde_json::from_slice(&answer.canonical_bytes()?)
                            .map_err(|error| StoreError::Corruption(error.to_string()))?,
                    );
                }
                let bytes = serde_json_canonicalizer::to_vec(&Value::Object(object))
                    .map_err(|error| StoreError::Corruption(error.to_string()))?;
                Event::decode_canonical(&bytes)
                    .map(Some)
                    .map_err(StoreError::from)
            },
        );
        match outcome {
            Ok(store::ConditionalAppendOutcome::Appended(outcome)) => {
                self.live
                    .ensure_after_locked_append(&authorization.session_id)
                    .map_err(route_failure)?;
                Ok(RespondAuthorReceipt {
                    semantic_seq: outcome.seq,
                    delivery: RespondDelivery::LockedAppend,
                })
            }
            Ok(store::ConditionalAppendOutcome::Skipped) => Err(HostFailure::Internal(
                "approval append unexpectedly skipped".to_owned(),
            )),
            Err(StoreError::Busy) => self
                .live_delivery(authorization, message)?
                .ok_or(HostFailure::Overloaded),
            Err(error) => Err(internal(error)),
        }
    }
}

impl RespondAuthority for ProductionRespondAuthority {
    fn locate(&self, rpc_id: &str) -> Result<Option<LocatedRespond>, HostFailure> {
        self.locate_once(rpc_id)
    }

    fn author(
        &self,
        mut authorization: RespondAuthorization,
    ) -> Result<RespondAuthorReceipt, HostFailure> {
        // Keep the public session UUID in the request lifecycle. Resolve the
        // private worker key from the durable journal, never client payload.
        if !authorization.session_id.contains(':') {
            let folder = self.root.join("threads").join(&authorization.session_id);
            let state = session_requests(&folder, &authorization.session_id)?
                .into_iter()
                .find(|state| state.request.rpc_id == authorization.rpc_id);
            if let Some(state) = state {
                if let Some(line) = state.request.source_line.as_deref() {
                    let (projection, ancestor_stopped) =
                        actionable_projection(&folder, &authorization.session_id, Some(line))?;
                    if ancestor_stopped || projection.lifecycle.stop_active {
                        return Err(internal("stop became active before child respond"));
                    }
                    validate_request_binding(
                        &projection.events,
                        state.request.causal_kernel_seq,
                        state.request.frame_type,
                        &state.request.envelope.payload,
                    )?;
                    let held = projection
                        .events
                        .iter()
                        .find(|e| e.seq() == state.request.causal_kernel_seq)
                        .ok_or_else(|| internal("child hold disappeared"))?;
                    if state.request.session_id != authorization.session_id
                        || held.string_field("call") != Some(authorization.call.as_str())
                    {
                        return Err(internal("child response causal identity mismatch"));
                    }
                    authorization.session_id = format!(
                        "{}:{}",
                        authorization.session_id,
                        line.strip_suffix(".jsonl")
                            .expect("validated child filename")
                    );
                }
            }
        }
        let message = self.worker_message(&authorization);
        let receipt = match self.live_delivery(&authorization, &message)? {
            Some(receipt) => receipt,
            None => self.locked_append(&authorization, &message)?,
        };
        self.reconcile_after_response(&authorization.session_id);
        Ok(receipt)
    }
}

#[derive(Clone)]
pub struct ProductionCarrierStreams {
    inner: Arc<CarrierStreamsInner>,
}

struct CarrierStreamsInner {
    root: PathBuf,
    description: MuxHostDescription,
    hub: EndpointSubscriptionHub,
    generations: Mutex<StreamGenerations>,
    controls: Mutex<BTreeMap<String, SessionControlItem>>,
    context_projection_lock: Mutex<()>,
    session_authority: Mutex<Option<Weak<dyn SupervisorSessionAuthority>>>,
    next_rpc: AtomicU64,
    /// Pending request ids last published per (session, line): the trigger
    /// for subscribers to re-diff their actionable baseline.
    published_actionables: Mutex<PublishedActionables>,
}

/// Pending request ids keyed by (session, line).
type PublishedActionables = HashMap<(String, Option<String>), BTreeSet<String>>;

#[derive(Default)]
struct StreamGenerations {
    mux: Vec<TrackedMux>,
    host: Vec<TrackedHost>,
}

struct TrackedMux {
    handle: AllSessionMuxHandle,
    lease: Weak<()>,
}

struct TrackedHost {
    queue: EndpointFrameQueue,
    lease: Weak<()>,
}

impl ProductionCarrierStreams {
    fn recover_actionables(&self, session: &str) -> Result<(), ProductionCarrierError> {
        let folder = self.inner.root.join("threads").join(session);
        let mut pending = vec!["main.jsonl".to_owned()];
        let mut visited = HashSet::new();
        while let Some(line) = pending.pop() {
            if !visited.insert(line.clone()) {
                continue;
            }
            let source = (line != "main.jsonl").then_some(line.as_str());
            let (projection, _) =
                actionable_projection(&folder, session, source).map_err(internal_carrier)?;
            self.reconcile_actionables(session, source)?;
            for spawn in projection
                .events
                .iter()
                .filter(|e| e.kind() == &EventKind::Spawn)
            {
                let child = spawn
                    .string_field("child")
                    .ok_or_else(|| internal_carrier("spawn lacks child"))?;
                // Publication of spawn precedes child file creation.
                // Validate the filename before probing outside the folder.
                let id = child
                    .strip_suffix(".jsonl")
                    .ok_or_else(|| internal_carrier("invalid child filename"))?;
                endpoint::validate_session_id(id).map_err(internal_carrier)?;
                if folder.join(child).exists() {
                    pending.push(child.to_owned());
                }
            }
        }
        Ok(())
    }

    /// Derive the answerable requests of one ledger line and notify
    /// subscribers when the pending set changed. Child facts never enter the
    /// root transcript journal.
    pub(crate) fn reconcile_actionables(
        &self,
        session_id: &str,
        source_line: Option<&str>,
    ) -> Result<(), ProductionCarrierError> {
        let folder = self.inner.root.join("threads").join(session_id);
        let (projection, _) =
            actionable_projection(&folder, session_id, source_line).map_err(internal_carrier)?;
        let requests =
            line_requests(session_id, source_line, &projection).map_err(internal_carrier)?;
        let pending = requests
            .iter()
            .filter(|(state, _)| state.resolution.is_none())
            .map(|(state, _)| state.request.rpc_id.clone())
            .collect::<BTreeSet<_>>();
        let key = (session_id.to_owned(), source_line.map(str::to_owned));
        let mut published = self
            .inner
            .published_actionables
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous = published.get(&key).cloned().unwrap_or_default();
        if previous != pending {
            // Any request/resolution change causes v3 to diff its baseline.
            // Repeated Appended notifications remain silent.
            for (state, frame) in requests {
                let id = &state.request.rpc_id;
                if pending.contains(id) != previous.contains(id) {
                    self.publish_session_frame(session_id, frame)?;
                }
            }
            published.insert(key, pending);
        }
        Ok(())
    }

    #[must_use]
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self::with_description(
            root,
            MuxHostDescription {
                protocol_version: endpoint::SESSION_ENDPOINT_PROTOCOL_VERSION,
                product: endpoint::MuxHostProduct {
                    name: "TekesKernel".to_owned(),
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                },
                capabilities: endpoint::SessionEndpointCapability::required(),
                cwd: "/".to_owned(),
                provider: None,
                model: None,
                attached_sessions: 0,
                home: "/".to_owned(),
                can_open_path: false,
            },
        )
    }

    #[must_use]
    pub fn with_description(root: impl AsRef<Path>, description: MuxHostDescription) -> Self {
        Self {
            inner: Arc::new(CarrierStreamsInner {
                root: root.as_ref().to_path_buf(),
                description,
                hub: EndpointSubscriptionHub::default(),
                generations: Mutex::new(StreamGenerations::default()),
                controls: Mutex::new(BTreeMap::new()),
                context_projection_lock: Mutex::new(()),
                session_authority: Mutex::new(None),
                next_rpc: AtomicU64::new(1),
                published_actionables: Mutex::new(HashMap::new()),
            }),
        }
    }

    /// Binds the process host: its live workers count as `running` in inventory
    /// summaries and it projects a session before a journal stream opens.
    pub fn attach_session_authority(&self, authority: Weak<dyn SupervisorSessionAuthority>) {
        *self
            .inner
            .session_authority
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(authority);
    }

    fn session_authority(&self) -> Option<Arc<dyn SupervisorSessionAuthority>> {
        self.inner
            .session_authority
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .and_then(Weak::upgrade)
    }

    fn live_sessions(&self) -> HashSet<String> {
        self.session_authority()
            .map(|authority| authority.live_sessions())
            .unwrap_or_default()
    }

    /// Announces a worker start or exit for `session_id`. Inventory streams
    /// re-list session summaries on this frame, so `running` follows the lock
    /// authority instead of freezing at the stream baseline (tail-lifecycle
    /// audit invariant I5). `running` is informational; the re-list recomputes it.
    pub fn publish_session_status(
        &self,
        session_id: &str,
        running: bool,
    ) -> Result<usize, ProductionCarrierError> {
        self.publish_host_frame(HostFrame::new(
            HostFrameKind::SessionStatus,
            IJsonValue::parse(&serde_json::to_vec(&json!({
                "type":"host/session-status",
                "sessionId":session_id,
                "running":running,
            }))?)?,
        )?)
    }

    pub fn attach_session(&self, session_id: &str) -> Result<(), ProductionCarrierError> {
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        generations
            .mux
            .retain(|generation| generation.lease.strong_count() != 0);
        let session = load_mux_session(&self.inner.root, session_id)?;
        for generation in &generations.mux {
            if generation.handle.contains(session_id)? {
                continue;
            }
            generation.handle.attach_with_replay(
                &self.inner.hub,
                session_id,
                &session.endpoint,
                &self.next_rpc_id("subscribed"),
                &session.unresolved,
            )?;
        }
        drop(generations);
        let projection = semantic_projection(&self.inner.root.join("threads").join(session_id))
            .map_err(internal_carrier)?;
        self.publish_host_frame(HostFrame::new(
            HostFrameKind::SessionAdded,
            IJsonValue::parse(&serde_json::to_vec(&json!({
                "type":"host/session-added",
                "sessionId":session_id,
                "blank":projection.latest_turn.is_none(),
            }))?)?,
        )?)?;
        Ok(())
    }

    pub fn detach_for_archive(&self, session_id: &str) -> Result<(), ProductionCarrierError> {
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        generations
            .mux
            .retain(|generation| generation.lease.strong_count() != 0);
        for generation in &generations.mux {
            generation.handle.detach_for_archive(session_id)?;
        }
        drop(generations);
        self.publish_host_frame(HostFrame::new(
            HostFrameKind::SessionRemoved,
            IJsonValue::parse(&serde_json::to_vec(&json!({
                "type":"host/session-removed",
                "sessionId":session_id,
            }))?)?,
        )?)?;
        self.publish_host_frame(HostFrame::new(
            HostFrameKind::ArchivedSessionsChanged,
            IJsonValue::parse_str(r#"{"type":"host/archived-sessions-changed"}"#)?,
        )?)?;
        Ok(())
    }

    pub fn publish_host_frame(&self, frame: HostFrame) -> Result<usize, ProductionCarrierError> {
        let envelope = frame.into_envelope(self.next_rpc_id("host"))?;
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        generations
            .host
            .retain(|generation| generation.lease.strong_count() != 0);
        let mut delivered = 0;
        let mut failure = None;
        generations
            .host
            .retain(|generation| match generation.queue.push(envelope.clone()) {
                Ok(()) => {
                    delivered += 1;
                    true
                }
                Err(
                    endpoint::FrameQueueError::Backpressure | endpoint::FrameQueueError::Closed,
                ) => false,
                Err(error) => {
                    failure = Some(error);
                    false
                }
            });
        if let Some(error) = failure {
            return Err(error.into());
        }
        Ok(delivered)
    }

    pub fn publish_session_frame(
        &self,
        session_id: &str,
        frame: endpoint::MuxFrame,
    ) -> Result<usize, ProductionCarrierError> {
        self.update_control_cache(&frame)?;
        Ok(self
            .inner
            .hub
            .publish_frame(session_id, self.next_rpc_id("mux"), frame)?)
    }

    fn update_control_cache(
        &self,
        frame: &endpoint::MuxFrame,
    ) -> Result<(), ProductionCarrierError> {
        let Some(session_id) = frame.session_id() else {
            return Ok(());
        };
        let mut controls = self.inner.controls.lock().map_err(|_| poisoned())?;
        let control = controls
            .entry(session_id.to_owned())
            .or_insert_with(|| SessionControlItem {
                session_id: session_id.to_owned(),
                queue: Vec::new(),
                jobs: Vec::new(),
                projections: IJsonValue::parse_str(r#"{"asOfSeq":-1,"values":{}}"#)
                    .expect("empty control projection is I-JSON"),
            });
        match frame {
            endpoint::MuxFrame::Queue { items, .. } => control.queue.clone_from(items),
            endpoint::MuxFrame::Jobs { jobs, .. } => control.jobs.clone_from(jobs),
            endpoint::MuxFrame::Projection {
                key, value, seq, ..
            } => {
                let mut projection: Value = serde_json::from_slice(
                    &control
                        .projections
                        .canonical_bytes()
                        .map_err(internal_carrier)?,
                )?;
                projection["asOfSeq"] = Value::from(*seq);
                projection["values"][key] =
                    serde_json::from_slice(&value.canonical_bytes().map_err(internal_carrier)?)?;
                control.projections = IJsonValue::parse(&serde_json::to_vec(&projection)?)?;
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    /// Publishes one event only after proving that identical bytes are
    /// present in the session's durable endpoint journal. Runtime projection
    /// drivers use this boundary instead of the unsequenced frame helper.
    pub fn publish_durable_session_event(
        &self,
        session_id: &str,
        event: endpoint::SessionEvent,
        view: Option<endpoint::SessionToolEventView>,
    ) -> Result<usize, ProductionCarrierError> {
        let journal = EndpointJournal::open(self.inner.root.join("threads").join(session_id))?;
        self.publish_durable_session_event_from_journal(session_id, &journal, event, view)
    }

    /// Publishes from the journal instance that performed the append. Streaming callers use this
    /// boundary so durability proof is a cached single-event lookup instead of reopening and
    /// reparsing the complete journal for every provider chunk.
    pub fn publish_durable_session_event_from_journal(
        &self,
        session_id: &str,
        journal: &EndpointJournal,
        event: endpoint::SessionEvent,
        view: Option<endpoint::SessionToolEventView>,
    ) -> Result<usize, ProductionCarrierError> {
        Ok(self.inner.hub.publish_event(
            session_id,
            journal,
            self.next_rpc_id("mux"),
            event,
            view,
        )?)
    }

    pub fn close_mux_generations(&self) -> Result<(), ProductionCarrierError> {
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        for generation in generations.mux.drain(..) {
            generation.handle.close()?;
        }
        Ok(())
    }

    fn open_legacy_mux(&self) -> Result<EndpointStreamReceiver, ProductionCarrierError> {
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        generations
            .mux
            .retain(|generation| generation.lease.strong_count() != 0);
        let sessions = load_active_mux_sessions(&self.inner.root)?;
        let rpc_ids = sessions
            .iter()
            .map(|_| self.next_rpc_id("subscribed"))
            .collect::<Vec<_>>();
        let registrations = sessions
            .iter()
            .zip(&rpc_ids)
            .map(|(session, rpc_id)| MuxReplayRegistration {
                registration: endpoint::MuxRegistration {
                    session_id: &session.session_id,
                    journal: &session.endpoint,
                    subscribed_rpc_id: rpc_id,
                },
                unresolved: &session.unresolved,
            })
            .collect::<Vec<_>>();
        let (handle, receiver) =
            AllSessionMux::open_with_replay(&self.inner.hub, &registrations)?.into_stream();
        let lease = Arc::new(());
        generations.mux.push(TrackedMux {
            handle: handle.clone(),
            lease: Arc::downgrade(&lease),
        });
        Ok(Box::new(LeasedStream {
            receiver,
            lease,
            on_drop: StreamDrop::Mux(handle),
        }))
    }

    fn open_host(&self) -> Result<EndpointStreamReceiver, ProductionCarrierError> {
        let queue = EndpointFrameQueue::new(HOST_STREAM_MAX_FRAMES, HOST_STREAM_MAX_BYTES)?;
        let receiver = queue.receiver();
        let lease = Arc::new(());
        let mut generations = self.inner.generations.lock().map_err(|_| poisoned())?;
        generations
            .host
            .retain(|generation| generation.lease.strong_count() != 0);
        generations.host.push(TrackedHost {
            queue: queue.clone(),
            lease: Arc::downgrade(&lease),
        });
        Ok(Box::new(LeasedStream {
            receiver,
            lease,
            on_drop: StreamDrop::Host(queue),
        }))
    }

    fn next_rpc_id(&self, kind: &str) -> String {
        let ordinal = self.inner.next_rpc.fetch_add(1, Ordering::Relaxed);
        format!("carrier-{kind}-{ordinal}")
    }

    pub(crate) fn refresh_all_context_projections(&self) -> Result<(), ProductionCarrierError> {
        for session in load_active_mux_sessions(&self.inner.root)? {
            if let Err(error) = self.refresh_context_projection(&session.session_id) {
                eprintln!(
                    "carrier-context-projection-failed: session={} error={error}",
                    session.session_id
                );
            }
        }
        Ok(())
    }

    pub(crate) fn refresh_context_projection(
        &self,
        session_id: &str,
    ) -> Result<(), ProductionCarrierError> {
        endpoint::validate_session_id(session_id).map_err(internal_carrier)?;
        let _guard = self
            .inner
            .context_projection_lock
            .lock()
            .map_err(|_| poisoned())?;
        let folder = self.inner.root.join("threads").join(session_id);
        let ledger = semantic_projection(&folder).map_err(internal_carrier)?;
        let endpoint = endpoint::NativeEndpoint::open(&self.inner.root)?;
        let config = endpoint.session_config_snapshot(session_id).ok();
        let value = crate::context_usage::derive(&folder, &ledger.events, config.as_ref());
        let journal = EndpointJournal::open(&folder)?;
        let journal_sequence = journal.last_seq()?.map_or(0, |seq| seq.saturating_add(1));
        let control_sequence = self
            .inner
            .controls
            .lock()
            .map_err(|_| poisoned())?
            .get(session_id)
            .and_then(|item| {
                serde_json::from_slice::<Value>(&item.projections.canonical_bytes().ok()?).ok()
            })
            .and_then(|value| value["asOfSeq"].as_u64())
            .unwrap_or(0);
        let (seq, changed) = crate::context_usage::publish(
            &folder,
            value.clone(),
            journal_sequence.max(control_sequence),
        )
        .map_err(internal_carrier)?;
        let frame = endpoint::MuxFrame::Projection {
            session_id: session_id.to_owned(),
            key: "contextUsage".to_owned(),
            value: IJsonValue::parse(&serde_json::to_vec(&value)?)?,
            seq,
        };
        if changed {
            self.publish_session_frame(session_id, frame)?;
        } else {
            self.update_control_cache(&frame)?;
        }
        Ok(())
    }

    fn workspace_baseline(
        &self,
        generation: u64,
    ) -> Result<SessionSyncFrame, ProductionCarrierError> {
        let endpoint = endpoint::NativeEndpoint::open(&self.inner.root)?;
        let sessions = endpoint.list_sessions(&self.live_sessions())?;
        let management = endpoint::ManagementStore::open(&self.inner.root)?;
        let list = management.list_workspaces(&sessions)?;
        Ok(SessionSyncFrame::WorkspaceBaseline {
            generation,
            baseline: WorkspaceBaseline {
                items: list
                    .items
                    .into_iter()
                    .map(|workspace| WorkspaceSummary {
                        id: workspace.workspace_id,
                        path: workspace.path,
                        title: workspace.title,
                        session_ids: workspace.session_ids,
                        created_at: workspace.created_at,
                        updated_at: workspace.updated_at,
                    })
                    .collect(),
                archived_session_ids: list.archived_session_ids,
            },
        })
    }

    fn inventory_baseline(
        &self,
        generation: u64,
    ) -> Result<SessionSyncFrame, ProductionCarrierError> {
        let endpoint = endpoint::NativeEndpoint::open(&self.inner.root)?;
        let management = endpoint::ManagementStore::open(&self.inner.root)?;
        let lineage = management.completed_fork_lineage()?;
        let mut sessions = endpoint
            .list_sessions(&self.live_sessions())?
            .into_iter()
            .filter(|session| !session.archived)
            .map(|session| {
                let projections = session.title.map(|title| {
                    IJsonValue::parse(
                        &serde_json::to_vec(&json!({
                            "asOfSeq":session.as_of_seq,
                            "values":{"sessionTitle":{"title":title}},
                        }))
                        .expect("session summary projection serializes"),
                    )
                    .expect("session summary projection is I-JSON")
                });
                let lock = if session.running {
                    LockFacts::OTHER
                } else {
                    LockFacts::FREE
                };
                let tail = classify(&session.lifecycle, lock);
                let fork = lineage.get(&session.session_id);
                Ok(SessionSummary {
                    cwd: management.workspace_path(&session.workspace_id)?,
                    parent_session_id: fork.map(|fork| fork.source.clone()),
                    origin: fork.map(|_| "fork".to_owned()),
                    ephemeral: session.ephemeral,
                    session_id: session.session_id,
                    updated_at: session.updated_at,
                    running: session.running,
                    tail: tail.as_str().to_owned(),
                    blank: session.blank,
                    projections,
                    permission_mode: Some(session.permission_mode),
                    identity_profile: session.identity_profile,
                })
            })
            .collect::<Result<Vec<_>, endpoint::ManagementError>>()?;
        sessions.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| left.session_id.cmp(&right.session_id))
        });
        Ok(SessionSyncFrame::InventoryBaseline {
            generation,
            items: sessions,
        })
    }

    fn control_baseline(
        &self,
        generation: u64,
    ) -> Result<SessionSyncFrame, ProductionCarrierError> {
        for session in load_active_mux_sessions(&self.inner.root)? {
            if let Err(error) = self.refresh_context_projection(&session.session_id) {
                eprintln!(
                    "carrier-context-projection-failed: session={} error={error}",
                    session.session_id
                );
            }
        }
        let items = self
            .inner
            .controls
            .lock()
            .map_err(|_| poisoned())?
            .values()
            .cloned()
            .collect();
        Ok(SessionSyncFrame::ControlBaseline { generation, items })
    }

    fn actionables_baseline(
        &self,
        generation: u64,
    ) -> Result<SessionSyncFrame, ProductionCarrierError> {
        let mut items = Vec::new();
        for session in load_active_mux_sessions(&self.inner.root)? {
            self.recover_actionables(&session.session_id)?;
            for request in pending_session_requests(
                &self.inner.root.join("threads").join(&session.session_id),
                &session.session_id,
            )
            .map_err(internal_carrier)?
            {
                let kind = match request.frame_type {
                    RequestFrameType::Approval => SessionActionableKind::Approval,
                    RequestFrameType::Question => SessionActionableKind::Question,
                };
                // The nested payload deliberately remains the existing typed
                // approval/question frame shape consumed by Tekes UI.
                items.push(SessionActionable {
                    id: request.rpc_id,
                    session_id: request.session_id,
                    revision: request.causal_kernel_seq,
                    kind,
                    payload: request.envelope.payload,
                });
            }
        }
        items.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(SessionSyncFrame::ActionableBaseline { generation, items })
    }

    fn open_mux_journal(
        &self,
        generation: u64,
        address: SessionAddress,
        max_messages: usize,
    ) -> Result<SessionStreamReceiver, ProductionCarrierError> {
        // A stale journal (projection failed after settle, or a supervisor locked
        // append with no worker) must not become the snapshot a client folds
        // from. Failure only logs: the periodic sweep repairs within one tick.
        if let Some(authority) = self.session_authority() {
            if let Err(error) = authority.reconcile_projection(&address.session_id) {
                eprintln!(
                    "carrier-journal-open-projection-failed: session={} error={error}",
                    address.session_id
                );
            }
        }
        let session = load_mux_session(&self.inner.root, &address.session_id)?;
        let rpc_id = self.next_rpc_id("journal-follow");
        let mux = AllSessionMux::open(
            &self.inner.hub,
            &[endpoint::MuxRegistration {
                session_id: &address.session_id,
                journal: &session.endpoint,
                subscribed_rpc_id: &rpc_id,
            }],
        )?;
        let through_sequence = mux
            .baselines()
            .first()
            .ok_or_else(|| {
                ProductionCarrierError::Lifecycle(
                    "journal follow did not establish a baseline".to_owned(),
                )
            })?
            .last_seq;
        let mut page =
            endpoint::frozen_history_page(&session.endpoint, through_sequence, None, max_messages)
                .map_err(|error| ProductionCarrierError::Lifecycle(error.to_string()))?;
        endpoint::NativeEndpoint::open(&self.inner.root)?
            .hydrate_historical_submissions(&address.session_id, &mut page.events);
        let projections = IJsonValue::parse(&serde_json::to_vec(&json!({
            "asOfSeq":through_sequence,
            "values":{},
        }))?)?;
        let (_, source) = mux.into_stream();
        Ok(Box::new(ProductionStream {
            owner: self.clone(),
            generation,
            kind: ProductionStreamKind::Journal(address.clone()),
            pending: [SessionSyncFrame::JournalSnapshot {
                generation,
                snapshot: SessionJournalSnapshot {
                    window_limit: max_messages,
                    address,
                    through_sequence,
                    entries: page.events,
                    has_more_before: page.has_more,
                    projections,
                },
            }]
            .into_iter()
            .collect(),
            source: Some(source),
        }))
    }

    pub(crate) fn open_mux(
        &self,
        generation: u64,
        target: SessionStreamTarget,
    ) -> Result<SessionStreamReceiver, ProductionCarrierError> {
        if generation == 0 {
            return Err(ProductionCarrierError::Lifecycle(
                "V3 generation must be positive".to_owned(),
            ));
        }
        match target {
            SessionStreamTarget::SessionJournal {
                address,
                max_messages,
            } => self.open_mux_journal(generation, address, max_messages),
            SessionStreamTarget::Workspace => {
                let frame = self.workspace_baseline(generation)?;
                let SessionSyncFrame::WorkspaceBaseline { baseline, .. } = &frame else {
                    unreachable!()
                };
                let kind = ProductionStreamKind::Workspace {
                    items: baseline
                        .items
                        .iter()
                        .cloned()
                        .map(|item| (item.id.clone(), item))
                        .collect(),
                    order: baseline.items.iter().map(|item| item.id.clone()).collect(),
                    archived: baseline.archived_session_ids.clone(),
                };
                Ok(Box::new(ProductionStream {
                    owner: self.clone(),
                    generation,
                    kind,
                    pending: [frame].into_iter().collect(),
                    source: Some(self.open_host()?),
                }))
            }
            SessionStreamTarget::SessionInventory => {
                let baseline = self.inventory_baseline(generation)?;
                let SessionSyncFrame::InventoryBaseline { items, .. } = &baseline else {
                    unreachable!()
                };
                Ok(Box::new(ProductionStream {
                    owner: self.clone(),
                    generation,
                    kind: ProductionStreamKind::Inventory {
                        items: items
                            .iter()
                            .cloned()
                            .map(|item| (item.session_id.clone(), item))
                            .collect(),
                    },
                    pending: [baseline].into_iter().collect(),
                    source: Some(self.open_host()?),
                }))
            }
            SessionStreamTarget::SessionControl => Ok(Box::new(ProductionStream {
                owner: self.clone(),
                generation,
                kind: ProductionStreamKind::Control,
                pending: [self.control_baseline(generation)?].into_iter().collect(),
                source: Some(self.open_legacy_mux()?),
            })),
            SessionStreamTarget::Actionables => {
                let baseline = self.actionables_baseline(generation)?;
                let SessionSyncFrame::ActionableBaseline { items, .. } = &baseline else {
                    unreachable!()
                };
                Ok(Box::new(ProductionStream {
                    owner: self.clone(),
                    generation,
                    kind: ProductionStreamKind::Actionables {
                        items: items
                            .iter()
                            .cloned()
                            .map(|item| (item.id.clone(), item))
                            .collect(),
                    },
                    pending: [baseline].into_iter().collect(),
                    source: Some(self.open_legacy_mux()?),
                }))
            }
        }
    }

    fn mux_journal_page(
        &self,
        request: SessionMuxClientFrame,
    ) -> Result<SessionJournalPage, ProductionCarrierError> {
        let SessionMuxClientFrame::JournalPage {
            address,
            through_sequence,
            before_sequence,
            max_messages,
            ..
        } = request
        else {
            return Err(ProductionCarrierError::Lifecycle(
                "expected a V3 journal-page request".to_owned(),
            ));
        };
        let session = load_mux_session(&self.inner.root, &address.session_id)?;
        let mut page = endpoint::frozen_history_page(
            &session.endpoint,
            through_sequence,
            before_sequence,
            max_messages,
        )
        .map_err(|error| ProductionCarrierError::Lifecycle(error.to_string()))?;
        endpoint::NativeEndpoint::open(&self.inner.root)?
            .hydrate_historical_submissions(&address.session_id, &mut page.events);
        Ok(SessionJournalPage {
            address,
            through_sequence,
            entries: page.events,
            has_more_before: page.has_more,
        })
    }
}

enum ProductionStreamKind {
    Workspace {
        items: BTreeMap<String, WorkspaceSummary>,
        order: Vec<String>,
        archived: Vec<String>,
    },
    Inventory {
        items: BTreeMap<String, SessionSummary>,
    },
    Journal(SessionAddress),
    Control,
    Actionables {
        items: BTreeMap<String, SessionActionable>,
    },
}

struct ProductionStream {
    owner: ProductionCarrierStreams,
    generation: u64,
    kind: ProductionStreamKind,
    pending: VecDeque<SessionSyncFrame>,
    source: Option<EndpointStreamReceiver>,
}

impl ProductionStream {
    fn map_source(
        &mut self,
        envelope: ServerRequest,
    ) -> Result<Option<SessionSyncFrame>, ProductionCarrierError> {
        match &mut self.kind {
            ProductionStreamKind::Workspace {
                items,
                order,
                archived,
            } => {
                let value: Value = serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
                let kind = value
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if matches!(
                    kind,
                    "host/workspace-changed"
                        | "host/workspace-removed"
                        | "host/workspace-order-changed"
                        | "host/archived-sessions-changed"
                        | "host/session-added"
                        | "host/session-removed"
                ) {
                    let SessionSyncFrame::WorkspaceBaseline { baseline, .. } =
                        self.owner.workspace_baseline(self.generation)?
                    else {
                        unreachable!()
                    };
                    let next_items = baseline
                        .items
                        .into_iter()
                        .map(|item| (item.id.clone(), item))
                        .collect::<BTreeMap<_, _>>();
                    let next_order = next_items.keys().cloned().collect::<Vec<_>>();
                    let mut deltas = VecDeque::new();
                    for id in items.keys().filter(|id| !next_items.contains_key(*id)) {
                        deltas.push_back(SessionSyncFrame::WorkspaceRemove {
                            generation: self.generation,
                            workspace_id: id.clone(),
                        });
                    }
                    for (id, workspace) in &next_items {
                        if items.get(id) != Some(workspace) {
                            deltas.push_back(SessionSyncFrame::WorkspaceUpsert {
                                generation: self.generation,
                                workspace: workspace.clone(),
                            });
                        }
                    }
                    if *order != next_order {
                        deltas.push_back(SessionSyncFrame::WorkspaceOrder {
                            generation: self.generation,
                            workspace_ids: next_order.clone(),
                        });
                    }
                    if *archived != baseline.archived_session_ids {
                        deltas.push_back(SessionSyncFrame::ArchivedSessions {
                            generation: self.generation,
                            session_ids: baseline.archived_session_ids.clone(),
                        });
                    }
                    *items = next_items;
                    *order = next_order;
                    *archived = baseline.archived_session_ids;
                    let first = deltas.pop_front();
                    self.pending.extend(deltas);
                    Ok(first)
                } else {
                    Ok(None)
                }
            }
            ProductionStreamKind::Inventory { items } => {
                let value: Value = serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
                let kind = value
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if matches!(
                    kind,
                    "host/session-added"
                        | "host/session-removed"
                        | "host/session-status"
                        | "host/agent-error"
                        | "host/workspace-changed"
                ) {
                    let SessionSyncFrame::InventoryBaseline {
                        items: next_items, ..
                    } = self.owner.inventory_baseline(self.generation)?
                    else {
                        unreachable!()
                    };
                    let next_items = next_items
                        .into_iter()
                        .map(|item| (item.session_id.clone(), item))
                        .collect::<BTreeMap<_, _>>();
                    let mut deltas = VecDeque::new();
                    for id in items.keys().filter(|id| !next_items.contains_key(*id)) {
                        deltas.push_back(SessionSyncFrame::InventoryRemove {
                            generation: self.generation,
                            session_id: id.clone(),
                        });
                    }
                    for (id, session) in &next_items {
                        if items.get(id) != Some(session) {
                            deltas.push_back(SessionSyncFrame::InventoryUpsert {
                                generation: self.generation,
                                session: session.clone(),
                            });
                        }
                    }
                    *items = next_items;
                    let first = deltas.pop_front();
                    self.pending.extend(deltas);
                    Ok(first)
                } else {
                    Ok(None)
                }
            }
            ProductionStreamKind::Journal(address) => {
                let frame: endpoint::MuxFrame =
                    serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
                match frame {
                    endpoint::MuxFrame::Event {
                        session_id,
                        event,
                        view,
                    } if session_id == address.session_id => {
                        Ok(Some(SessionSyncFrame::JournalEvent {
                            generation: self.generation,
                            address: address.clone(),
                            event,
                            view,
                        }))
                    }
                    endpoint::MuxFrame::Transient { session_id, event }
                        if session_id == address.session_id =>
                    {
                        Ok(Some(SessionSyncFrame::JournalTransient {
                            generation: self.generation,
                            address: address.clone(),
                            event,
                        }))
                    }
                    endpoint::MuxFrame::Projection {
                        session_id,
                        key,
                        value,
                        seq,
                    } if session_id == address.session_id => {
                        Ok(Some(SessionSyncFrame::JournalProjection {
                            generation: self.generation,
                            address: address.clone(),
                            key,
                            value,
                            sequence: seq,
                        }))
                    }
                    _ => Ok(None),
                }
            }
            ProductionStreamKind::Control => {
                let frame: endpoint::MuxFrame =
                    serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
                let Some(session_id) = frame.session_id() else {
                    return Ok(None);
                };
                if !matches!(
                    frame,
                    endpoint::MuxFrame::Queue { .. }
                        | endpoint::MuxFrame::Jobs { .. }
                        | endpoint::MuxFrame::Projection { .. }
                ) {
                    return Ok(None);
                }
                let controls = self.owner.inner.controls.lock().map_err(|_| poisoned())?;
                Ok(controls.get(session_id).cloned().map(|control| {
                    SessionSyncFrame::ControlUpsert {
                        generation: self.generation,
                        control,
                    }
                }))
            }
            ProductionStreamKind::Actionables { items } => {
                let frame: endpoint::MuxFrame =
                    serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
                if matches!(
                    frame,
                    endpoint::MuxFrame::ApprovalRequested { .. }
                        | endpoint::MuxFrame::ApprovalResolved { .. }
                        | endpoint::MuxFrame::QuestionRequested { .. }
                        | endpoint::MuxFrame::QuestionResolved { .. }
                ) {
                    let baseline = self.owner.actionables_baseline(self.generation)?;
                    let SessionSyncFrame::ActionableBaseline {
                        items: next_items, ..
                    } = baseline
                    else {
                        unreachable!()
                    };
                    let next: BTreeMap<_, _> = next_items
                        .into_iter()
                        .map(|item| (item.id.clone(), item))
                        .collect();
                    let mut deltas = VecDeque::new();
                    for (id, previous) in items.iter() {
                        if !next.contains_key(id) {
                            deltas.push_back(SessionSyncFrame::ActionableResolved {
                                generation: self.generation,
                                id: id.clone(),
                                revision: previous.revision,
                            });
                        }
                    }
                    for (id, actionable) in &next {
                        if items.get(id) != Some(actionable) {
                            deltas.push_back(SessionSyncFrame::ActionableUpsert {
                                generation: self.generation,
                                actionable: actionable.clone(),
                            });
                        }
                    }
                    *items = next;
                    let first = deltas.pop_front();
                    self.pending.extend(deltas);
                    Ok(first)
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl SessionStream for ProductionStream {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<SessionSyncFrame, StreamFailure>>> {
        Box::pin(async move {
            if let Some(frame) = self.pending.pop_front() {
                return Some(Ok(frame));
            }
            loop {
                let source = self.source.as_mut()?;
                let envelope = match source.recv().await? {
                    Ok(envelope) => envelope,
                    Err(failure) => return Some(Err(failure)),
                };
                match self.map_source(envelope) {
                    Ok(Some(frame)) => return Some(Ok(frame)),
                    Ok(None) => {}
                    Err(error) => return Some(Err(StreamFailure::internal(error.to_string()))),
                }
            }
        })
    }
}

impl CarrierStreamHandler for ProductionCarrierStreams {
    fn open_stream(
        &self,
        channel: StreamChannel,
    ) -> CarrierHostFuture<'_, Result<EndpointStreamReceiver, HostFailure>> {
        Box::pin(std::future::ready(match channel {
            StreamChannel::Mux => self.open_legacy_mux().map_err(internal),
            StreamChannel::Host => self.open_host().map_err(internal),
        }))
    }

    fn stream_error(
        &self,
        _channel: StreamChannel,
        code: StreamErrorCode,
    ) -> Result<ServerRequest, HostFailure> {
        let message = match code {
            StreamErrorCode::LiveGap => "Live stream gap",
            StreamErrorCode::ServerDraining => "Server is draining",
            StreamErrorCode::ProtocolError => "Stream protocol error",
            StreamErrorCode::FrameTooLarge => "Stream frame is too large",
            StreamErrorCode::InternalError => "Stream failed",
        };
        Ok(ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: self.next_rpc_id("error"),
            method: "stream/error".to_owned(),
            payload: IJsonValue::parse(
                &serde_json::to_vec(&json!({
                    "type":"stream/error",
                    "error":{"code":code.code(),"message":message,"details":{}}
                }))
                .map_err(internal)?,
            )
            .map_err(internal)?,
        })
    }

    fn mux_description(&self) -> Result<MuxHostDescription, HostFailure> {
        self.inner.description.validate().map_err(|error| {
            HostFailure::Protocol(format!("invalid V3 host description: {error}"))
        })?;
        Ok(self.inner.description.clone())
    }

    fn open_mux_stream(
        &self,
        generation: u64,
        target: SessionStreamTarget,
    ) -> CarrierHostFuture<'_, Result<SessionStreamReceiver, HostFailure>> {
        Box::pin(std::future::ready(
            self.open_mux(generation, target).map_err(internal),
        ))
    }

    fn journal_page(
        &self,
        request: SessionMuxClientFrame,
    ) -> CarrierHostFuture<'_, Result<SessionJournalPage, HostFailure>> {
        Box::pin(std::future::ready(
            self.mux_journal_page(request).map_err(internal),
        ))
    }
}

struct LeasedStream {
    receiver: EndpointStreamReceiver,
    #[allow(dead_code)]
    lease: Arc<()>,
    on_drop: StreamDrop,
}

enum StreamDrop {
    Mux(AllSessionMuxHandle),
    Host(EndpointFrameQueue),
}

impl EndpointStream for LeasedStream {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<ServerRequest, StreamFailure>>> {
        self.receiver.recv()
    }
}

impl Drop for LeasedStream {
    fn drop(&mut self) {
        match &self.on_drop {
            StreamDrop::Mux(handle) => {
                let _ = handle.close();
            }
            StreamDrop::Host(queue) => {
                let _ = queue.close();
            }
        }
    }
}

pub struct LifecycleCarrierHost {
    unary: ProductionEndpointHost,
    streams: ProductionCarrierStreams,
}

impl LifecycleCarrierHost {
    #[must_use]
    pub const fn new(unary: ProductionEndpointHost, streams: ProductionCarrierStreams) -> Self {
        Self { unary, streams }
    }
}

impl EndpointHost for LifecycleCarrierHost {
    fn capabilities(&self) -> BTreeSet<String> {
        self.unary.capabilities()
    }

    fn extension_capabilities(&self) -> BTreeSet<String> {
        self.unary.extension_capabilities()
    }

    fn method_class(&self, method: &str) -> endpoint::MethodClass {
        self.unary.method_class(method)
    }

    fn validate_request(&self, request: &endpoint::ClientRequest) -> Result<(), HostFailure> {
        self.unary.validate_request(request)
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        Box::pin(async move {
            let lifecycle = lifecycle_target(&request);
            let result = self.unary.call(request).await;
            if result.ok {
                let hook = match lifecycle {
                    Some(LifecycleTarget::Attach(session_id)) => {
                        self.streams.attach_session(&session_id)
                    }
                    Some(LifecycleTarget::Detach(session_id)) => {
                        self.streams.detach_for_archive(&session_id)
                    }
                    Some(LifecycleTarget::Created) => result
                        .value
                        .as_ref()
                        .and_then(result_session_id)
                        .ok_or_else(|| {
                            ProductionCarrierError::Lifecycle(
                                "session.create success omitted sessionId".to_owned(),
                            )
                        })
                        .and_then(|session_id| self.streams.attach_session(&session_id)),
                    None => Ok(()),
                };
                if hook.is_err() {
                    let _ = self.streams.close_mux_generations();
                }
            }
            result
        })
    }
}

enum LifecycleTarget {
    Attach(String),
    Detach(String),
    Created,
}

fn lifecycle_target(request: &EndpointHostCall) -> Option<LifecycleTarget> {
    match request.operation.as_str() {
        "workspace.archiveSession" | "session.discard" => {
            payload_session_id(&request.payload).map(LifecycleTarget::Detach)
        }
        "workspace.unarchiveSession" => {
            payload_session_id(&request.payload).map(LifecycleTarget::Attach)
        }
        "session.create" => Some(LifecycleTarget::Created),
        _ => None,
    }
}

pub type ProductionCarrierHost = ComposedEndpointCarrierHost<
    LifecycleCarrierHost,
    JournalRespondHandler<ProductionRespondAuthority>,
    ProductionCarrierStreams,
>;

pub struct ProductionCarrierAssembly {
    host: Arc<ProductionCarrierHost>,
    streams: ProductionCarrierStreams,
    server: TransportServer,
}

impl ProductionCarrierAssembly {
    pub fn assemble(
        root: impl AsRef<Path>,
        unary: ProductionEndpointHost,
        live_respond: Arc<dyn LiveRespondAuthority>,
        config: TransportConfig,
    ) -> Result<Self, ProductionCarrierError> {
        let root = root.as_ref();
        let admission = unary.admission_gates();
        let streams = ProductionCarrierStreams::with_description(root, unary.mux_description());
        let respond = JournalRespondHandler::new(
            ProductionRespondAuthority::open(root, admission, live_respond)?
                .with_streams(streams.clone()),
            RpcRegistry::open(root)?,
        );
        let host = Arc::new(ComposedEndpointCarrierHost::new(
            LifecycleCarrierHost::new(unary, streams.clone()),
            RpcRegistry::open(root)?,
            respond,
            streams.clone(),
        ));
        host.set_readiness(HostReadiness::NotReady);
        let server =
            TransportServer::new(Arc::clone(&host) as Arc<dyn EndpointCarrierHost>, config)?
                .with_file_transfer(Arc::new(
                    crate::file_leases::WorkspaceFileTransfer::open(root)
                        .map_err(std::io::Error::other)?,
                ));
        Ok(Self {
            host,
            streams,
            server,
        })
    }

    #[must_use]
    pub fn host(&self) -> Arc<ProductionCarrierHost> {
        Arc::clone(&self.host)
    }

    pub fn with_file_changes(mut self, authority: Arc<dyn transport::FileChangeAuthority>) -> Self {
        self.server.set_file_changes(authority);
        self
    }

    #[must_use]
    pub const fn streams(&self) -> &ProductionCarrierStreams {
        &self.streams
    }

    #[must_use]
    pub const fn server(&self) -> &TransportServer {
        &self.server
    }

    #[must_use]
    pub fn into_server(self) -> TransportServer {
        self.server
    }

    /// Called only after supervisor boot recovery and capability assembly are
    /// complete. Re-reading the full active inventory proves mux capacity and
    /// every durable journal can be opened before the listener reports ready.
    pub fn finish_recovery(&self) -> Result<(), ProductionCarrierError> {
        let sessions = load_active_mux_sessions(&self.streams.inner.root)?;
        if sessions.len() > endpoint::MAX_MUX_SESSIONS {
            return Err(ProductionCarrierError::Lifecycle(
                "active session inventory exceeds mux capacity".to_owned(),
            ));
        }
        self.host.set_readiness(HostReadiness::Ready);
        Ok(())
    }

    pub fn begin_drain(&self) {
        self.host.set_readiness(HostReadiness::NotReady);
        self.server.handle().begin_drain();
    }
}

struct MuxSession {
    session_id: String,
    endpoint: EndpointJournal,
    unresolved: Vec<ServerRequest>,
}

fn load_active_mux_sessions(root: &Path) -> Result<Vec<MuxSession>, ProductionCarrierError> {
    let mut session_ids = fs::read_dir(root.join("threads"))?.collect::<Result<Vec<_>, _>>()?;
    session_ids.sort_by_key(std::fs::DirEntry::file_name);
    let mut sessions = Vec::new();
    for entry in session_ids {
        if entry.file_type()?.is_dir() {
            sessions.push(load_mux_session(
                root,
                &entry.file_name().to_string_lossy(),
            )?);
        }
    }
    Ok(sessions)
}

fn load_mux_session(root: &Path, session_id: &str) -> Result<MuxSession, ProductionCarrierError> {
    endpoint::validate_session_id(session_id)
        .map_err(|error| ProductionCarrierError::Lifecycle(error.to_string()))?;
    let folder = root.join("threads").join(session_id);
    let endpoint = EndpointJournal::open(&folder)?;
    let unresolved = pending_session_requests(&folder, session_id)
        .map_err(internal_carrier)?
        .into_iter()
        .map(|request| request.envelope)
        .collect();
    Ok(MuxSession {
        session_id: session_id.to_owned(),
        endpoint,
        unresolved,
    })
}

fn semantic_projection(folder: &Path) -> Result<schema::LedgerProjection, HostFailure> {
    semantic_line_projection(folder, "main.jsonl")
}

fn semantic_line_projection(
    folder: &Path,
    line: &str,
) -> Result<schema::LedgerProjection, HostFailure> {
    if line != "main.jsonl" {
        let id = line
            .strip_suffix(".jsonl")
            .ok_or_else(|| internal("invalid ledger filename"))?;
        endpoint::validate_session_id(id).map_err(internal)?;
    }
    let path = folder.join(line);
    if !fs::symlink_metadata(&path)
        .map_err(internal)?
        .file_type()
        .is_file()
    {
        return Err(internal("actionable ledger must be a regular file"));
    }
    let bytes = fs::read(path).map_err(internal)?;
    let scan = scan_valid_prefix(&bytes, 1);
    if scan.needs_repair() {
        return Err(HostFailure::Internal(
            "respond semantic ledger requires tail recovery".to_owned(),
        ));
    }
    scan.projection
        .ok_or_else(|| HostFailure::Internal("respond semantic ledger is empty".to_owned()))
}

/// The answerable requests of one ledger line: every `approval_request` hold
/// with its public requested frame and, when the line already carries it, the
/// ledger resolution (`approval_response`, or the aborting `tool_result`).
fn line_requests(
    session_id: &str,
    source_line: Option<&str>,
    projection: &schema::LedgerProjection,
) -> Result<Vec<(RequestState, endpoint::MuxFrame)>, HostFailure> {
    let mut requests = Vec::new();
    for hold in projection
        .events
        .iter()
        .filter(|e| e.kind() == &EventKind::ApprovalRequest)
    {
        let call = hold
            .string_field("call")
            .ok_or_else(|| internal("hold has no call"))?;
        let tool = projection
            .events
            .iter()
            .find(|e| e.kind() == &EventKind::ToolCall && e.string_field("call") == Some(call))
            .ok_or_else(|| internal("hold has no tool call"))?;
        let question = hold.string_field("scope") == Some("answer") && hold.has_field("question");
        let frame = if question {
            let value = serde_json::to_value(hold.raw()).map_err(internal)?;
            // WorkflowBackend stores the validated singular invocation:
            // {question: string, options: array|null}. The public frame
            // groups questions in an array; there is no nested `questions`.
            let questions = vec![public_question(call, &value["question"])?];
            endpoint::MuxFrame::QuestionRequested {
                session_id: session_id.to_owned(),
                questions,
            }
        } else {
            let approval_id = match source_line {
                Some(line) => format!("approval:{line}:{call}"),
                None => format!("approval:{call}"),
            };
            endpoint::MuxFrame::ApprovalRequested {
                session_id: session_id.to_owned(),
                approval_id,
                tool_name: tool
                    .string_field("name")
                    .ok_or_else(|| internal("tool lacks name"))?
                    .to_owned(),
                call_id: Some(call.to_owned()),
                reason: hold.string_field("scope").map(str::to_owned),
            }
        };
        let payload =
            IJsonValue::parse(&serde_json::to_vec(&frame).map_err(internal)?).map_err(internal)?;
        let kind = if question {
            RequestFrameType::Question
        } else {
            RequestFrameType::Approval
        };
        let request = PendingRequest::derive(session_id, source_line, kind, hold.seq(), payload)
            .map_err(internal)?;
        let response = projection.events.iter().find(|e| {
            e.seq() > hold.seq()
                && e.kind() == &EventKind::ApprovalResponse
                && e.string_field("call") == Some(call)
        });
        let state = if let Some(response) = response {
            let value = serde_json::to_value(response.raw()).map_err(internal)?;
            // A question's `approval_response {grant:false}` is the client
            // cancellation arm (`cancelQuestion`): the held call is denied
            // and the request resolves as `cancelled`, never `answered`.
            let outcome = if question && value["grant"] == true {
                endpoint::ResolutionOutcome::Answered
            } else if question {
                endpoint::ResolutionOutcome::Cancelled
            } else if value["grant"] == true {
                endpoint::ResolutionOutcome::AllowedOnce
            } else {
                endpoint::ResolutionOutcome::Rejected
            };
            RequestState::resolved(request, response.seq(), outcome).map_err(internal)?
        } else if let Some(aborted) = projection.events.iter().find(|e| {
            e.seq() > hold.seq()
                && e.kind() == &EventKind::ToolResult
                && e.string_field("call") == Some(call)
                && serde_json::to_value(e.raw())
                    .ok()
                    .is_some_and(|value| value["outcome"].get("aborted").is_some())
        }) {
            RequestState::resolved(
                request,
                aborted.seq(),
                endpoint::ResolutionOutcome::Cancelled,
            )
            .map_err(internal)?
        } else {
            RequestState {
                request,
                resolution: None,
            }
        };
        requests.push((state, frame));
    }
    Ok(requests)
}

/// Every request of a session: its main line plus every spawned child line
/// that exists. Child ancestry is validated where a request is exposed or
/// answered (`actionable_projection`), not here.
fn session_requests(folder: &Path, session: &str) -> Result<Vec<RequestState>, HostFailure> {
    let mut states = Vec::new();
    let mut pending = vec!["main.jsonl".to_owned()];
    let mut visited = HashSet::new();
    while let Some(line) = pending.pop() {
        if !visited.insert(line.clone()) {
            continue;
        }
        let source = (line != "main.jsonl").then_some(line.as_str());
        let projection = semantic_line_projection(folder, &line)?;
        states.extend(
            line_requests(session, source, &projection)?
                .into_iter()
                .map(|(state, _)| state),
        );
        for spawn in projection
            .events
            .iter()
            .filter(|e| e.kind() == &EventKind::Spawn)
        {
            let child = spawn
                .string_field("child")
                .ok_or_else(|| internal("spawn lacks child"))?;
            let id = child
                .strip_suffix(".jsonl")
                .ok_or_else(|| internal("invalid child filename"))?;
            endpoint::validate_session_id(id).map_err(internal)?;
            if folder.join(child).exists() {
                pending.push(child.to_owned());
            }
        }
    }
    states.sort_by_key(|state| state.request.causal_kernel_seq);
    Ok(states)
}

/// The session's unresolved requests in causal order.
fn pending_session_requests(
    folder: &Path,
    session: &str,
) -> Result<Vec<PendingRequest>, HostFailure> {
    Ok(session_requests(folder, session)?
        .into_iter()
        .filter(|state| state.resolution.is_none())
        .map(|state| state.request)
        .collect())
}

/// A child filename is not authority: every genesis must bind an exact spawn
/// in its parent, and the chain must terminate at this session's main ledger.
fn actionable_projection(
    folder: &Path,
    session: &str,
    source_line: Option<&str>,
) -> Result<(schema::LedgerProjection, bool), HostFailure> {
    let line = source_line.unwrap_or("main.jsonl");
    let projection = semantic_line_projection(folder, line)?;
    let mut current = projection.clone();
    let mut current_line = line.to_owned();
    let mut visited = HashSet::new();
    let mut ancestor_stopped = false;
    loop {
        if !visited.insert(current_line.clone()) {
            return Err(internal("actionable ancestry cycle"));
        }
        let genesis = current
            .events
            .first()
            .ok_or_else(|| internal("missing genesis"))?;
        let expected = if current_line == "main.jsonl" {
            session
        } else {
            current_line
                .strip_suffix(".jsonl")
                .expect("validated filename")
        };
        if genesis.string_field("thread") != Some(expected) {
            return Err(internal("actionable ledger identity mismatch"));
        }
        if current_line == "main.jsonl" {
            break;
        }
        let value = serde_json::to_value(genesis.raw()).map_err(internal)?;
        let parent = &value["parent"];
        let parent_line = parent["file"]
            .as_str()
            .ok_or_else(|| internal("child lacks parent file"))?;
        let parent_seq = parent["seq"]
            .as_u64()
            .ok_or_else(|| internal("child lacks parent sequence"))?;
        let spawn_id = parent["spawn_id"]
            .as_str()
            .ok_or_else(|| internal("child lacks spawn identity"))?;
        let parent_projection = semantic_line_projection(folder, parent_line)?;
        let spawn = parent_projection
            .events
            .iter()
            .find(|e| e.seq() == parent_seq)
            .ok_or_else(|| internal("child parent spawn is missing"))?;
        if spawn.kind() != &EventKind::Spawn
            || spawn.string_field("child") != Some(current_line.as_str())
            || spawn.string_field("spawn_id") != Some(spawn_id)
        {
            return Err(internal("child parent spawn binding mismatch"));
        }
        ancestor_stopped |= parent_projection.lifecycle.stop_active;
        current_line = parent_line.to_owned();
        current = parent_projection;
    }
    Ok((projection, ancestor_stopped))
}

fn validate_request_binding(
    events: &[Event],
    causal_seq: u64,
    frame_type: RequestFrameType,
    payload: &IJsonValue,
) -> Result<Option<String>, HostFailure> {
    let event = events
        .iter()
        .find(|event| event.seq() == causal_seq && event.kind() == &EventKind::ApprovalRequest)
        .ok_or_else(|| HostFailure::Internal("ledger lacks the causal approval hold".to_owned()))?;
    let call = event
        .string_field("call")
        .ok_or_else(|| HostFailure::Internal("causal approval hold lacks its call".to_owned()))?;
    let payload: Value =
        serde_json::from_slice(&payload.canonical_bytes().map_err(internal)?).map_err(internal)?;
    // Permission holds also carry `question` (tool/class/call metadata).
    // Only the answer workflow requests a structured user answer.
    let asks_answer = event.string_field("scope") == Some("answer") && event.has_field("question");
    match frame_type {
        RequestFrameType::Approval
            if asks_answer || payload.get("callId").and_then(Value::as_str) != Some(call) =>
        {
            Err(HostFailure::Internal(
                "approval request disagrees with its semantic hold".to_owned(),
            ))
        }
        RequestFrameType::Question if !asks_answer => Err(HostFailure::Internal(
            "question request disagrees with its semantic hold".to_owned(),
        )),
        RequestFrameType::Question => Ok(Some(call.to_owned())),
        RequestFrameType::Approval => Ok(None),
    }
}

fn unresolved_hold_turn(events: &[Event], call: &str) -> Result<u64, StoreError> {
    let request = events
        .iter()
        .rev()
        .find(|event| {
            event.kind() == &EventKind::ApprovalRequest && event.string_field("call") == Some(call)
        })
        .ok_or_else(|| StoreError::Corruption("respond hold is missing".to_owned()))?;
    if events.iter().any(|event| {
        event.seq() > request.seq()
            && matches!(
                event.kind(),
                EventKind::ApprovalResponse | EventKind::ToolResult
            )
            && event.string_field("call") == Some(call)
    }) {
        return Err(StoreError::Corruption(
            "respond hold is already resolved".to_owned(),
        ));
    }
    request
        .turn()
        .ok_or_else(|| StoreError::Corruption("respond hold has no turn".to_owned()))
}

fn payload_session_id(payload: &IJsonValue) -> Option<String> {
    serde_json::from_slice::<Value>(&payload.canonical_bytes().ok()?)
        .ok()?
        .get("sessionId")?
        .as_str()
        .map(str::to_owned)
}

fn result_session_id(value: &IJsonValue) -> Option<String> {
    serde_json::from_slice::<Value>(&value.canonical_bytes().ok()?)
        .ok()?
        .get("sessionId")?
        .as_str()
        .map(str::to_owned)
}

fn route_failure(error: ProductionRouteFailure) -> HostFailure {
    HostFailure::Internal(format!("{}: {}", error.code, error.message))
}

/// Project the held `ask_user_questions` invocation into the endpoint's
/// question vocabulary (`question/requested` row: `{id, question, options:
/// [{label}], multiSelect, allowCustom}`), the shape every Client question
/// surface reads and answers as `{answers:[{id, selected, custom}]}`. The
/// invocation's own shape (`options` as bare strings, no id) is the model
/// contract, not the Client's; the call id keys the one question so the
/// answer row names the held call. Options are replies, so free text stays
/// allowed and the choice is single.
fn public_question(call: &str, invocation: &Value) -> Result<IJsonValue, HostFailure> {
    let question = invocation["question"]
        .as_str()
        .ok_or_else(|| internal("question hold lacks its question"))?;
    let options = invocation["options"]
        .as_array()
        .map(|options| {
            options
                .iter()
                .filter_map(Value::as_str)
                .map(|label| json!({ "label": label }))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row = json!({
        "id": call,
        "question": question,
        "options": options,
        "multiSelect": false,
        "allowCustom": true,
    });
    IJsonValue::parse(&serde_json::to_vec(&row).map_err(internal)?).map_err(internal)
}

fn internal(error: impl std::fmt::Display) -> HostFailure {
    HostFailure::Internal(error.to_string())
}

fn internal_carrier(error: impl std::fmt::Display) -> ProductionCarrierError {
    ProductionCarrierError::Lifecycle(error.to_string())
}

fn poisoned() -> ProductionCarrierError {
    ProductionCarrierError::Lifecycle("carrier generation mutex poisoned".to_owned())
}

fn system_timestamp() -> Result<String, HostFailure> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(internal)?;
    let seconds = now.as_secs();
    let millis = now.subsec_millis();
    // Millisecond precision like the worker's own event clock, so a
    // supervisor-authored `approval_response` orders next to the worker's
    // `tool_result` on the same timeline. Keep this formatter local so the
    // carrier does not depend on an environment time zone.
    let days = i64::try_from(seconds / 86_400).map_err(internal)?;
    let day_seconds = seconds % 86_400;
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        day_seconds / 3_600,
        day_seconds % 3_600 / 60,
        day_seconds % 60,
    ))
}

#[derive(Debug, Error)]
pub enum ProductionCarrierError {
    #[error("carrier I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("carrier store failed: {0}")]
    Store(#[from] StoreError),
    #[error("carrier native endpoint failed: {0}")]
    NativeEndpoint(#[from] endpoint::NativeEndpointError),
    #[error("carrier management projection failed: {0}")]
    Management(#[from] endpoint::ManagementError),
    #[error("carrier endpoint journal failed: {0}")]
    EndpointJournal(#[from] endpoint::JournalError),
    #[error("carrier request derivation failed: {0}")]
    Request(#[from] endpoint::RequestError),
    #[error("carrier mux failed: {0}")]
    Mux(#[from] endpoint::AllSessionMuxError),
    #[error("carrier hub failed: {0}")]
    Hub(#[from] endpoint::HubError),
    #[error("carrier frame queue failed: {0}")]
    FrameQueue(#[from] endpoint::FrameQueueError),
    #[error("carrier rpc registry failed: {0}")]
    Registry(#[from] endpoint::RpcRegistryError),
    #[error("carrier transport assembly failed: {0}")]
    Transport(#[from] TransportConfigError),
    #[error("carrier lifecycle failed: {0}")]
    Lifecycle(String),
    #[error("carrier JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("carrier schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
}

#[cfg(test)]
mod question_cancellation_tests {
    use super::*;

    #[test]
    fn question_denial_projects_as_cancelled_and_answer_as_answered() {
        let thread = "018f0000-0000-7000-8000-0000000000aa";
        let ts = "2026-09-13T00:00:00.000Z";
        let request = json!({"v":1,"seq":3,"turn":1,"kind":"approval_request","ts":ts,"call":"c1","scope":"answer","question":{"question":"Continue?","options":null}});
        let ledger = |response: Value| {
            let events = [
                json!({"v":1,"seq":1,"kind":"genesis","ts":ts,"format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws","origin_key":"g","origin_tuple":{"principal":"p","client":"cli","target":thread,"op":"create","key":"g"},"resume":"never","config":{"digest":"d"}}),
                json!({"v":1,"seq":2,"kind":"input","ts":ts,"content":[{"type":"text","text":"go"}],"origin_key":"i","origin_tuple":{"principal":"p","client":"cli","target":thread,"op":"input","key":"i"}}),
                json!({"v":1,"seq":3,"turn":1,"kind":"turn_open","ts":ts,"trigger":{"inputs":[2]}}),
                json!({"v":1,"seq":4,"kind":"epoch","ts":ts,"id":"e1","reason":"initial","adapter":"anthropic_messages_v1","model":"m","system":{"asset":"sha256-s","digest":"d"},"tools":{"asset":"sha256-t","digest":"td"},"renderer":1}),
                json!({"v":1,"seq":5,"turn":1,"kind":"attempt","ts":ts,"request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":1},"attempt":"a1","epoch":"e1","wire_digest":"w","admits":[{"from":2,"to":2}]}),
                json!({"v":1,"seq":6,"turn":1,"kind":"tool_call","ts":ts,"attempt":"a1","call":"c1","name":"ask_user_questions","args":{"question":"Continue?","options":null},"source":"provider"}),
                {
                    let mut hold = request.clone();
                    hold["seq"] = json!(7);
                    hold
                },
                {
                    let mut response = response;
                    response["seq"] = json!(8);
                    response
                },
            ];
            let mut bytes = Vec::new();
            for event in events {
                bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical"));
                bytes.push(b'\n');
            }
            schema::validate_ledger(&bytes, 1).expect("valid ledger")
        };
        let cancelled = ledger(
            json!({"v":1,"turn":1,"kind":"approval_response","ts":ts,"call":"c1","grant":false,"origin_key":"r/response","origin_tuple":{"principal":"p","client":"endpoint","target":thread,"op":"respond","key":"r/response"}}),
        );
        let requests = line_requests(thread, None, &cancelled).expect("requests");
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].0.resolution,
            Some((8, endpoint::ResolutionOutcome::Cancelled))
        );
        let answered = ledger(
            json!({"v":1,"turn":1,"kind":"approval_response","ts":ts,"call":"c1","grant":true,"answer":{"answers":["yes"]},"origin_key":"r/response","origin_tuple":{"principal":"p","client":"endpoint","target":thread,"op":"respond","key":"r/response"}}),
        );
        let requests = line_requests(thread, None, &answered).expect("requests");
        assert_eq!(
            requests[0].0.resolution,
            Some((8, endpoint::ResolutionOutcome::Answered))
        );
    }
}
