use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};

use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::rpc::{RpcError, ServerRequest};
use crate::{
    EndpointJournal, JournalError, SessionEvent, SessionToolEventView, validate_session_id,
};

pub const DEFAULT_SUBSCRIPTION_MAX_FRAMES: usize = 4_096;
pub const DEFAULT_SUBSCRIPTION_MAX_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_MUX_SESSIONS: usize = 256;

pub struct MuxRegistration<'a> {
    pub session_id: &'a str,
    pub journal: &'a EndpointJournal,
    pub subscribed_rpc_id: &'a str,
}

/// Durable request frames replayed only into a newly-created mux generation.
pub struct MuxReplayRegistration<'a> {
    pub registration: MuxRegistration<'a>,
    pub unresolved: &'a [ServerRequest],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum MuxFrame {
    #[serde(rename = "session/subscribed")]
    Subscribed {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(rename = "lastSeq")]
        last_seq: i64,
    },
    #[serde(rename = "session/event")]
    Event {
        #[serde(rename = "sessionId")]
        session_id: String,
        event: SessionEvent,
        #[serde(skip_serializing_if = "Option::is_none")]
        view: Option<SessionToolEventView>,
    },
    /// Process-local provider output. It is deliberately unsequenced and non-replayable: the
    /// journal/storage lane may persist the same adapter frame independently without gating this
    /// presentation path.
    #[serde(rename = "session/transient")]
    Transient {
        #[serde(rename = "sessionId")]
        session_id: String,
        event: SessionEvent,
    },
    #[serde(rename = "session/queue")]
    Queue {
        #[serde(rename = "sessionId")]
        session_id: String,
        items: Vec<IJsonValue>,
    },
    #[serde(rename = "session/jobs")]
    Jobs {
        #[serde(rename = "sessionId")]
        session_id: String,
        jobs: Vec<IJsonValue>,
    },
    #[serde(rename = "session/projection")]
    Projection {
        #[serde(rename = "sessionId")]
        session_id: String,
        key: String,
        value: IJsonValue,
        seq: u64,
    },
    #[serde(rename = "approval/requested")]
    ApprovalRequested {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(rename = "approvalId")]
        approval_id: String,
        #[serde(rename = "toolName")]
        tool_name: String,
        #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
        call_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
    #[serde(rename = "approval/resolved")]
    ApprovalResolved {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(rename = "approvalId")]
        approval_id: String,
        outcome: IJsonValue,
    },
    #[serde(rename = "question/requested")]
    QuestionRequested {
        #[serde(rename = "sessionId")]
        session_id: String,
        questions: Vec<IJsonValue>,
    },
    #[serde(rename = "question/resolved")]
    QuestionResolved {
        #[serde(rename = "sessionId")]
        session_id: String,
        #[serde(rename = "questionRpcId")]
        question_rpc_id: String,
        outcome: String,
    },
    #[serde(rename = "stream/error")]
    StreamError { error: RpcError },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostFrameKind {
    SessionAdded,
    SessionRemoved,
    SessionStatus,
    AgentError,
    WorkspaceChanged,
    WorkspaceRemoved,
    WorkspaceOrderChanged,
    ArchivedSessionsChanged,
    RemoteEvent,
    StreamError,
}

impl HostFrameKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionAdded => "host/session-added",
            Self::SessionRemoved => "host/session-removed",
            Self::SessionStatus => "host/session-status",
            Self::AgentError => "host/agent-error",
            Self::WorkspaceChanged => "host/workspace-changed",
            Self::WorkspaceRemoved => "host/workspace-removed",
            Self::WorkspaceOrderChanged => "host/workspace-order-changed",
            Self::ArchivedSessionsChanged => "host/archived-sessions-changed",
            Self::RemoteEvent => "host/remote-event",
            Self::StreamError => "stream/error",
        }
    }
}

/// Typed host-stream discriminant plus the authority-owned payload bytes.
/// Host payload fields are intentionally not redefined by Kernel; validation
/// proves the payload's `type` agrees with the closed Client-v2 frame kind.
#[derive(Clone, Debug, PartialEq)]
pub struct HostFrame {
    kind: HostFrameKind,
    payload: IJsonValue,
}

impl HostFrame {
    pub fn new(kind: HostFrameKind, payload: IJsonValue) -> Result<Self, HubError> {
        let value: serde_json::Value = serde_json::from_slice(&payload.canonical_bytes()?)?;
        let actual = value
            .as_object()
            .and_then(|object| object.get("type"))
            .and_then(serde_json::Value::as_str)
            .ok_or(HubError::MissingFrameType)?;
        if actual != kind.as_str() {
            return Err(HubError::FrameTypeMismatch {
                expected: kind.as_str(),
                actual: actual.to_owned(),
            });
        }
        Ok(Self { kind, payload })
    }

    #[must_use]
    pub const fn kind(&self) -> HostFrameKind {
        self.kind
    }

    #[must_use]
    pub fn payload(&self) -> &IJsonValue {
        &self.payload
    }

    pub fn into_envelope(self, rpc_id: impl Into<String>) -> Result<ServerRequest, HubError> {
        let envelope = ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: rpc_id.into(),
            method: self.kind.as_str().to_owned(),
            payload: self.payload,
        };
        envelope.validate_stream(crate::StreamChannel::Host)?;
        Ok(envelope)
    }
}

impl MuxFrame {
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Subscribed { .. } => "session/subscribed",
            Self::Event { .. } => "session/event",
            Self::Transient { .. } => "session/transient",
            Self::Queue { .. } => "session/queue",
            Self::Jobs { .. } => "session/jobs",
            Self::Projection { .. } => "session/projection",
            Self::ApprovalRequested { .. } => "approval/requested",
            Self::ApprovalResolved { .. } => "approval/resolved",
            Self::QuestionRequested { .. } => "question/requested",
            Self::QuestionResolved { .. } => "question/resolved",
            Self::StreamError { .. } => "stream/error",
        }
    }

    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        match self {
            Self::Subscribed { session_id, .. }
            | Self::Event { session_id, .. }
            | Self::Transient { session_id, .. }
            | Self::Queue { session_id, .. }
            | Self::Jobs { session_id, .. }
            | Self::Projection { session_id, .. }
            | Self::ApprovalRequested { session_id, .. }
            | Self::ApprovalResolved { session_id, .. }
            | Self::QuestionRequested { session_id, .. }
            | Self::QuestionResolved { session_id, .. } => Some(session_id),
            Self::StreamError { .. } => None,
        }
    }

    #[must_use]
    pub fn event_seq(&self) -> Option<u64> {
        match self {
            Self::Event { event, .. } => Some(event.seq),
            _ => None,
        }
    }

    pub fn into_envelope(self, rpc_id: impl Into<String>) -> Result<ServerRequest, HubError> {
        let method = self.kind().to_owned();
        let payload = IJsonValue::parse(
            &serde_json_canonicalizer::to_vec(&self)
                .map_err(|error| HubError::Canonical(error.to_string()))?,
        )?;
        let envelope = ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: rpc_id.into(),
            method,
            payload,
        };
        envelope.validate_stream(crate::StreamChannel::Mux)?;
        Ok(envelope)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionBaseline {
    pub session_id: String,
    pub last_seq: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubscriptionPoll {
    Frame(ServerRequest),
    LiveGap,
    Pending,
    Closed,
}

struct SubscriptionState {
    frames: VecDeque<(ServerRequest, usize)>,
    pending_events: BTreeMap<u64, (ServerRequest, usize)>,
    bytes: usize,
    max_frames: usize,
    max_bytes: usize,
    next_seq: u64,
    overflowed: bool,
    gap_reported: bool,
    closed: bool,
    waker: Option<Waker>,
}

impl SubscriptionState {
    fn has_capacity(&self, size: usize) -> bool {
        self.frames.len() + self.pending_events.len() < self.max_frames
            && self
                .bytes
                .checked_add(size)
                .is_some_and(|bytes| bytes <= self.max_bytes)
    }

    fn overflow(&mut self) {
        let pending_bytes = self
            .pending_events
            .values()
            .map(|(_, size)| *size)
            .sum::<usize>();
        self.bytes -= pending_bytes;
        self.pending_events.clear();
        self.overflowed = true;
        self.closed = true;
        self.wake();
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn enqueue(&mut self, frame: ServerRequest) -> Result<(), HubError> {
        let size = frame.canonical_bytes()?.len();
        if !self.has_capacity(size) {
            self.overflow();
            return Err(HubError::LiveGap);
        }
        self.bytes += size;
        self.frames.push_back((frame, size));
        self.wake();
        Ok(())
    }

    fn enqueue_event(&mut self, seq: u64, frame: ServerRequest) -> Result<bool, HubError> {
        if seq < self.next_seq {
            return Ok(false);
        }
        let size = frame.canonical_bytes()?.len();
        if let Some((existing, _)) = self.pending_events.get(&seq) {
            if existing.canonical_bytes()? != frame.canonical_bytes()? {
                self.overflow();
                return Err(HubError::EventConflict(seq));
            }
            return Ok(false);
        }
        if !self.has_capacity(size) {
            self.overflow();
            return Err(HubError::LiveGap);
        }
        self.bytes += size;
        if seq == self.next_seq {
            self.frames.push_back((frame, size));
            self.next_seq = self.next_seq.saturating_add(1);
            while let Some((pending, size)) = self.pending_events.remove(&self.next_seq) {
                self.frames.push_back((pending, size));
                self.next_seq = self.next_seq.saturating_add(1);
            }
        } else {
            self.pending_events.insert(seq, (frame, size));
        }
        self.wake();
        Ok(true)
    }

    fn poll(&mut self) -> SubscriptionPoll {
        if let Some((frame, size)) = self.frames.pop_front() {
            self.bytes -= size;
            return SubscriptionPoll::Frame(frame);
        }
        if self.overflowed && !self.gap_reported {
            self.gap_reported = true;
            return SubscriptionPoll::LiveGap;
        }
        if self.closed {
            SubscriptionPoll::Closed
        } else {
            // There is deliberately no runtime-specific blocking primitive in
            // the lower seam; a carrier polls again after its own wakeup.
            SubscriptionPoll::Pending
        }
    }

    fn poll_with_context(&mut self, context: &Context<'_>) -> Poll<SubscriptionPoll> {
        match self.poll() {
            SubscriptionPoll::Pending => {
                if self
                    .waker
                    .as_ref()
                    .is_none_or(|waker| !waker.will_wake(context.waker()))
                {
                    self.waker = Some(context.waker().clone());
                }
                Poll::Pending
            }
            value => Poll::Ready(value),
        }
    }
}

#[derive(Clone)]
pub struct EndpointSubscription {
    baseline: SubscriptionBaseline,
    state: Arc<Mutex<SubscriptionState>>,
}

impl EndpointSubscription {
    #[must_use]
    pub fn baseline(&self) -> &SubscriptionBaseline {
        &self.baseline
    }

    pub fn poll(&self) -> Result<SubscriptionPoll, HubError> {
        Ok(self.state.lock().map_err(|_| HubError::Poisoned)?.poll())
    }

    pub fn close(&self) -> Result<(), HubError> {
        let mut state = self.state.lock().map_err(|_| HubError::Poisoned)?;
        state.closed = true;
        state.wake();
        Ok(())
    }

    pub(crate) fn wake(&self) -> Result<(), HubError> {
        self.state.lock().map_err(|_| HubError::Poisoned)?.wake();
        Ok(())
    }

    pub(crate) fn poll_with_context(
        &self,
        context: &Context<'_>,
    ) -> Result<Poll<SubscriptionPoll>, HubError> {
        Ok(self
            .state
            .lock()
            .map_err(|_| HubError::Poisoned)?
            .poll_with_context(context))
    }
}

#[derive(Default)]
pub struct EndpointSubscriptionHub {
    sessions: Mutex<HashMap<String, Vec<Weak<Mutex<SubscriptionState>>>>>,
}

impl EndpointSubscriptionHub {
    pub fn subscribe(
        &self,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: impl Into<String>,
    ) -> Result<EndpointSubscription, HubError> {
        self.subscribe_with_bounds(
            session_id,
            journal,
            subscribed_rpc_id,
            DEFAULT_SUBSCRIPTION_MAX_FRAMES,
            DEFAULT_SUBSCRIPTION_MAX_BYTES,
        )
    }

    pub fn subscribe_with_bounds(
        &self,
        session_id: &str,
        journal: &EndpointJournal,
        subscribed_rpc_id: impl Into<String>,
        max_frames: usize,
        max_bytes: usize,
    ) -> Result<EndpointSubscription, HubError> {
        validate_session_id(session_id)?;
        if max_frames == 0 || max_bytes == 0 {
            return Err(HubError::InvalidBounds);
        }

        // Holding the hub registry lock across the journal snapshot makes
        // registration atomic with every publish operation. The journal read
        // itself holds only its own bounded shared lock and performs no await.
        let mut sessions = self.sessions.lock().map_err(|_| HubError::Poisoned)?;
        let last_seq = journal.last_seq()?.map_or(Ok(-1), |seq| {
            i64::try_from(seq).map_err(|_| HubError::Sequence)
        })?;
        let next_seq = if last_seq < 0 {
            0
        } else {
            u64::try_from(last_seq)
                .map_err(|_| HubError::Sequence)?
                .checked_add(1)
                .ok_or(HubError::Sequence)?
        };
        let state = Arc::new(Mutex::new(SubscriptionState {
            frames: VecDeque::new(),
            pending_events: BTreeMap::new(),
            bytes: 0,
            max_frames,
            max_bytes,
            next_seq,
            overflowed: false,
            gap_reported: false,
            closed: false,
            waker: None,
        }));
        let subscribed = MuxFrame::Subscribed {
            session_id: session_id.to_owned(),
            last_seq,
        }
        .into_envelope(subscribed_rpc_id)?;
        state
            .lock()
            .map_err(|_| HubError::Poisoned)?
            .enqueue(subscribed)?;
        sessions
            .entry(session_id.to_owned())
            .or_default()
            .push(Arc::downgrade(&state));
        Ok(EndpointSubscription {
            baseline: SubscriptionBaseline {
                session_id: session_id.to_owned(),
                last_seq,
            },
            state,
        })
    }

    /// Atomically registers the complete active-session inventory. Validation
    /// and every durable baseline snapshot finish while the hub registry lock
    /// excludes publishers; no partial set becomes reachable on error.
    pub fn subscribe_all(
        &self,
        registrations: &[MuxRegistration<'_>],
    ) -> Result<BTreeMap<String, EndpointSubscription>, HubError> {
        let registrations = registrations
            .iter()
            .map(|registration| MuxReplayRegistration {
                registration: MuxRegistration {
                    session_id: registration.session_id,
                    journal: registration.journal,
                    subscribed_rpc_id: registration.subscribed_rpc_id,
                },
                unresolved: &[],
            })
            .collect::<Vec<_>>();
        self.subscribe_all_with_replay(&registrations)
    }

    /// Atomically registers the complete active inventory and queues each
    /// session's unresolved request frames immediately after its baseline.
    /// The replay is private to the returned generation and never broadcasts
    /// to subscriptions that were already live.
    pub fn subscribe_all_with_replay(
        &self,
        registrations: &[MuxReplayRegistration<'_>],
    ) -> Result<BTreeMap<String, EndpointSubscription>, HubError> {
        if registrations.len() > MAX_MUX_SESSIONS {
            return Err(HubError::ActiveSessionLimit(registrations.len()));
        }
        let mut previous = None;
        for registration in registrations {
            let session_id = registration.registration.session_id;
            validate_session_id(session_id)?;
            if previous.is_some_and(|value: &str| value >= session_id) {
                return Err(HubError::InventoryOrder);
            }
            for frame in registration.unresolved {
                frame.validate_stream(crate::StreamChannel::Mux)?;
                let payload: MuxFrame = serde_json::from_slice(&frame.payload.canonical_bytes()?)?;
                if payload.session_id() != Some(session_id)
                    || !matches!(
                        payload,
                        MuxFrame::ApprovalRequested { .. } | MuxFrame::QuestionRequested { .. }
                    )
                {
                    return Err(HubError::InvalidReplayFrame);
                }
            }
            previous = Some(session_id);
        }

        let mut sessions = self.sessions.lock().map_err(|_| HubError::Poisoned)?;
        let mut prepared = Vec::with_capacity(registrations.len());
        for registration in registrations {
            let base = &registration.registration;
            let last_seq = base.journal.last_seq()?.map_or(Ok(-1), |seq| {
                i64::try_from(seq).map_err(|_| HubError::Sequence)
            })?;
            let next_seq = if last_seq < 0 {
                0
            } else {
                u64::try_from(last_seq)
                    .map_err(|_| HubError::Sequence)?
                    .checked_add(1)
                    .ok_or(HubError::Sequence)?
            };
            let state = Arc::new(Mutex::new(SubscriptionState {
                frames: VecDeque::new(),
                pending_events: BTreeMap::new(),
                bytes: 0,
                max_frames: DEFAULT_SUBSCRIPTION_MAX_FRAMES,
                max_bytes: DEFAULT_SUBSCRIPTION_MAX_BYTES,
                next_seq,
                overflowed: false,
                gap_reported: false,
                closed: false,
                waker: None,
            }));
            state.lock().map_err(|_| HubError::Poisoned)?.enqueue(
                MuxFrame::Subscribed {
                    session_id: base.session_id.to_owned(),
                    last_seq,
                }
                .into_envelope(base.subscribed_rpc_id)?,
            )?;
            {
                let mut state = state.lock().map_err(|_| HubError::Poisoned)?;
                for frame in registration.unresolved {
                    state.enqueue(frame.clone())?;
                }
            }
            prepared.push((base.session_id.to_owned(), last_seq, state));
        }

        let mut result = BTreeMap::new();
        for (session_id, last_seq, state) in prepared {
            sessions
                .entry(session_id.clone())
                .or_default()
                .push(Arc::downgrade(&state));
            result.insert(
                session_id.clone(),
                EndpointSubscription {
                    baseline: SubscriptionBaseline {
                        session_id,
                        last_seq,
                    },
                    state,
                },
            );
        }
        Ok(result)
    }

    /// Publishes an already durable event. If durability won the race before
    /// subscribe, the baseline suppresses the duplicate; if subscribe won,
    /// strict `next_seq` delivers it exactly once.
    pub fn publish_event(
        &self,
        session_id: &str,
        journal: &EndpointJournal,
        rpc_id: impl Into<String>,
        event: SessionEvent,
        view: Option<SessionToolEventView>,
    ) -> Result<usize, HubError> {
        event.validate()?;
        let durable = journal
            .event(event.seq)?
            .ok_or(HubError::NotDurable(event.seq))?;
        if durable.canonical_bytes()? != event.canonical_bytes()? {
            return Err(HubError::DurableMismatch(event.seq));
        }
        self.publish(
            session_id,
            MuxFrame::Event {
                session_id: session_id.to_owned(),
                event,
                view,
            }
            .into_envelope(rpc_id)?,
        )
    }

    pub fn publish_frame(
        &self,
        session_id: &str,
        rpc_id: impl Into<String>,
        frame: MuxFrame,
    ) -> Result<usize, HubError> {
        if matches!(frame, MuxFrame::Subscribed { .. }) {
            return Err(HubError::SubscribedIsHubOwned);
        }
        if frame
            .session_id()
            .is_some_and(|candidate| candidate != session_id)
        {
            return Err(HubError::SessionMismatch);
        }
        self.publish(session_id, frame.into_envelope(rpc_id)?)
    }

    fn publish(&self, session_id: &str, frame: ServerRequest) -> Result<usize, HubError> {
        validate_session_id(session_id)?;
        let payload: MuxFrame = serde_json::from_slice(&frame.payload.canonical_bytes()?)?;
        let event_seq = payload.event_seq();
        let mut sessions = self.sessions.lock().map_err(|_| HubError::Poisoned)?;
        let subscribers = sessions.entry(session_id.to_owned()).or_default();
        let mut delivered = 0;
        subscribers.retain(|weak| {
            let Some(state) = weak.upgrade() else {
                return false;
            };
            let Ok(mut state) = state.lock() else {
                return false;
            };
            if state.closed {
                return true;
            }
            if let Some(seq) = event_seq {
                if state
                    .enqueue_event(seq, frame.clone())
                    .is_ok_and(|queued| queued)
                {
                    delivered += 1;
                }
            } else if state.enqueue(frame.clone()).is_ok() {
                delivered += 1;
            }
            true
        });
        Ok(delivered)
    }
}

#[derive(Debug, Error)]
pub enum HubError {
    #[error("endpoint journal failed: {0}")]
    Journal(#[from] JournalError),
    #[error("endpoint type failed: {0}")]
    Type(#[from] crate::types::EndpointTypeError),
    #[error("endpoint request failed: {0}")]
    Request(#[from] crate::rpc::RequestError),
    #[error("endpoint JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("endpoint I-JSON failed: {0}")]
    IJson(#[from] schema::SchemaError),
    #[error("endpoint canonicalization failed: {0}")]
    Canonical(String),
    #[error("subscription bounds must be positive")]
    InvalidBounds,
    #[error("active session inventory contains {0} entries; maximum is 256")]
    ActiveSessionLimit(usize),
    #[error("active session inventory must be unique and ascending")]
    InventoryOrder,
    #[error("endpoint sequence exceeds the carrier limit")]
    Sequence,
    #[error("subscription overflowed; emit live-gap then close")]
    LiveGap,
    #[error("session frame does not match its subscription")]
    SessionMismatch,
    #[error("session/subscribed is authored only by the hub")]
    SubscribedIsHubOwned,
    #[error("mux replay contains a non-request or cross-session frame")]
    InvalidReplayFrame,
    #[error("endpoint event seq {0} is not durable in the journal")]
    NotDurable(u64),
    #[error("endpoint event seq {0} differs from its durable journal bytes")]
    DurableMismatch(u64),
    #[error("endpoint event seq {0} was published with conflicting bytes")]
    EventConflict(u64),
    #[error("stream payload has no string type")]
    MissingFrameType,
    #[error("stream payload type {actual} does not match {expected}")]
    FrameTypeMismatch {
        expected: &'static str,
        actual: String,
    },
    #[error("subscription mutex was poisoned")]
    Poisoned,
}
