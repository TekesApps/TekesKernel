use std::collections::{BTreeSet, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::idempotency::{RpcBegin, RpcDurableIdentity, RpcRegistry, RpcRegistryError};
use crate::mux::{
    MuxHostDescription, SessionJournalPage, SessionMuxClientFrame, SessionStreamTarget,
    SessionSyncFrame,
};
use crate::rpc::{
    ClientRequest, ClientResponse, RequestError, RespondReceipt, RpcError, RpcResult,
    ServerRequest, ServerResponse,
};

pub type EndpointHostFuture<'a> = Pin<Box<dyn Future<Output = RpcResult> + Send + 'a>>;

#[derive(Clone, Debug)]
pub struct EndpointHostCall {
    pub rpc_id: String,
    pub operation: String,
    pub payload: IJsonValue,
    /// True only when a previous process durably claimed this exact request
    /// but did not durably record its result.
    pub recovering: bool,
    /// Shared carrier handoff proof. A host may mark this immediately after
    /// its semantic barrier; the dispatcher also marks it after the exact
    /// response is durably completed in the rpc registry.
    pub handoff: DurableHandoffSignal,
}

/// Carrier-neutral boundary implemented by executable assembly. Implementors
/// return only imported v2 values; they never expose supervisor/process types.
pub trait EndpointHost: Send + Sync {
    fn capabilities(&self) -> BTreeSet<String>;

    /// Independently versioned additive methods advertised by the assembled
    /// production route owner. Frozen Session Endpoint registrations are
    /// intentionally excluded from this set.
    fn extension_capabilities(&self) -> BTreeSet<String> {
        BTreeSet::new()
    }

    fn method_class(&self, method: &str) -> MethodClass {
        if self.capabilities().contains(method) {
            MethodClass::Mutation
        } else {
            MethodClass::Unknown
        }
    }

    /// Method-specific carrier validation performed before the durable rpcId
    /// namespace is touched. Non-v2 test hosts retain a permissive default.
    fn validate_request(&self, _request: &ClientRequest) -> Result<(), HostFailure> {
        Ok(())
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_>;
}

pub type CarrierHostFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostReadiness {
    Ready,
    NotReady,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MethodClass {
    ReadOnly,
    Mutation,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamChannel {
    Mux,
    Host,
}

impl StreamChannel {
    #[must_use]
    pub const fn method(self) -> &'static str {
        match self {
            Self::Mux => "events.mux",
            Self::Host => "events.host",
        }
    }

    #[must_use]
    pub(crate) const fn frame_types(self) -> &'static [&'static str] {
        match self {
            Self::Mux => &[
                "session/event",
                "session/transient",
                "session/subscribed",
                "approval/requested",
                "approval/resolved",
                "question/requested",
                "question/resolved",
                "session/queue",
                "session/jobs",
                "session/projection",
                "stream/error",
            ],
            Self::Host => &[
                "host/session-added",
                "host/session-removed",
                "host/session-status",
                "host/agent-error",
                "host/workspace-changed",
                "host/workspace-removed",
                "host/workspace-order-changed",
                "host/archived-sessions-changed",
                "host/remote-event",
                "stream/error",
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamErrorCode {
    LiveGap,
    ServerDraining,
    ProtocolError,
    FrameTooLarge,
    InternalError,
}

impl StreamErrorCode {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::LiveGap => "live-gap",
            Self::ServerDraining => "server-draining",
            Self::ProtocolError => "protocol-error",
            Self::FrameTooLarge => "frame-too-large",
            Self::InternalError => "internal-error",
        }
    }

    #[must_use]
    pub const fn close_code(self) -> u16 {
        match self {
            Self::LiveGap | Self::ServerDraining => 1013,
            Self::ProtocolError => 1002,
            Self::FrameTooLarge => 1009,
            Self::InternalError => 1011,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CallContext {
    pub response_deadline: Instant,
    pub drain: DrainSignal,
    /// Shared with the carrier so a timeout after semantic durability can be
    /// distinguished from a timeout before the mutation was handed off.
    pub handoff: DurableHandoffSignal,
}

#[derive(Clone, Debug)]
pub struct DrainSignal(Arc<AtomicBool>);

impl DrainSignal {
    #[must_use]
    pub const fn new(state: Arc<AtomicBool>) -> Self {
        Self(state)
    }

    #[must_use]
    pub fn is_draining(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableHandoffProof {
    pub delivery: String,
    pub durable_identity: Option<RpcDurableIdentity>,
}

#[derive(Debug, Default)]
struct DurableHandoffState {
    durable: AtomicBool,
    proof: Mutex<Option<DurableHandoffProof>>,
}

#[derive(Clone, Debug, Default)]
pub struct DurableHandoffSignal(Arc<DurableHandoffState>);

impl DurableHandoffSignal {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Called by the endpoint author path immediately after the durable
    /// mutation receipt exists, before any later response serialization.
    /// This proof-less form is retained for response-completion notification;
    /// semantic mutation authors must call `mark_handed_off` instead.
    pub fn mark_durable(&self) {
        self.0.durable.store(true, Ordering::Release);
    }

    /// Records the delivery authority and optional semantic durable identity.
    /// Re-driving the same proof is idempotent; changing it is fail-closed.
    pub fn mark_handed_off(&self, proof: DurableHandoffProof) -> Result<(), DurableHandoffError> {
        if proof.delivery.is_empty() {
            return Err(DurableHandoffError::EmptyDelivery);
        }
        if proof
            .durable_identity
            .as_ref()
            .is_some_and(|identity| identity.kind.is_empty() || identity.id.is_empty())
        {
            return Err(DurableHandoffError::InvalidIdentity);
        }
        let mut current = self
            .0
            .proof
            .lock()
            .map_err(|_| DurableHandoffError::Poisoned)?;
        match current.as_ref() {
            Some(existing) if existing != &proof => {
                return Err(DurableHandoffError::ConflictingProof);
            }
            Some(_) => {}
            None => *current = Some(proof),
        }
        drop(current);
        self.mark_durable();
        Ok(())
    }

    fn proof(&self) -> Result<Option<DurableHandoffProof>, DurableHandoffError> {
        self.0
            .proof
            .lock()
            .map(|proof| proof.clone())
            .map_err(|_| DurableHandoffError::Poisoned)
    }

    /// Read by the carrier when its response deadline or drain wins the race.
    #[must_use]
    pub fn is_durable(&self) -> bool {
        self.0.durable.load(Ordering::Acquire)
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum DurableHandoffError {
    #[error("durable handoff delivery must be nonempty")]
    EmptyDelivery,
    #[error("durable handoff identity fields must be nonempty")]
    InvalidIdentity,
    #[error("durable handoff proof changed while a request was in flight")]
    ConflictingProof,
    #[error("durable handoff signal mutex was poisoned")]
    Poisoned,
}

/// Runtime-neutral async stream. The carrier awaits one already validated
/// server envelope at a time and owns socket backpressure separately.
pub trait EndpointStream: Send {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<ServerRequest, StreamFailure>>>;
}

pub type EndpointStreamReceiver = Box<dyn EndpointStream>;

pub trait SessionStream: Send {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<SessionSyncFrame, StreamFailure>>>;
}

pub type SessionStreamReceiver = Box<dyn SessionStream>;

/// Typed terminal failure from an already-open endpoint stream. The carrier
/// maps `code` directly to the contract-owned stream/error envelope and close
/// code; `diagnostic` is local-only and must never enter wire bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamFailure {
    code: StreamErrorCode,
    diagnostic: Option<String>,
}

impl StreamFailure {
    #[must_use]
    pub const fn new(code: StreamErrorCode) -> Self {
        Self {
            code,
            diagnostic: None,
        }
    }

    #[must_use]
    pub fn internal(diagnostic: impl Into<String>) -> Self {
        Self {
            code: StreamErrorCode::InternalError,
            diagnostic: Some(diagnostic.into()),
        }
    }

    #[must_use]
    pub const fn code(&self) -> StreamErrorCode {
        self.code
    }

    #[must_use]
    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }
}

/// Complete endpoint-owned callback surface required by a physical carrier.
/// Transport crates import this trait; they must not define a competing host
/// authority or inspect supervisor/process-table state.
pub trait EndpointCarrierHost: Send + Sync + 'static {
    fn readiness(&self) -> HostReadiness;

    /// Exact executable registration set, including the two stream paths and
    /// `respond`. The transport compares this in both directions with the
    /// Client-owned `.tekes` driver catalog before becoming ready.
    fn registered_methods(&self) -> BTreeSet<String>;

    /// Exact additive route set frozen beside `registered_methods` when the
    /// carrier is assembled. The transport validates this separately from
    /// the Client-owned base registry.
    fn advertised_extension_methods(&self) -> BTreeSet<String> {
        BTreeSet::new()
    }

    fn classify_method(&self, method: &str) -> MethodClass;

    fn unary(
        &self,
        request: ClientRequest,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<ServerResponse, HostFailure>>;

    fn respond(
        &self,
        response: ClientResponse,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<RespondReceipt, HostFailure>>;

    /// The returned receiver is already registered. For mux, its first frames
    /// are the durable per-session `session/subscribed` baselines.
    fn open_stream(
        &self,
        channel: StreamChannel,
    ) -> CarrierHostFuture<'_, Result<EndpointStreamReceiver, HostFailure>>;

    /// Returns the exact authority-compatible terminal stream envelope. The
    /// closed error code also owns the RFC-6455 close code/reason mapping.
    fn stream_error(
        &self,
        channel: StreamChannel,
        code: StreamErrorCode,
    ) -> Result<ServerRequest, HostFailure>;

    fn mux_description(&self) -> Result<MuxHostDescription, HostFailure> {
        Err(HostFailure::Protocol(
            "Session Endpoint V3 is unavailable".to_owned(),
        ))
    }

    fn open_mux_stream(
        &self,
        _generation: u64,
        _target: SessionStreamTarget,
    ) -> CarrierHostFuture<'_, Result<SessionStreamReceiver, HostFailure>> {
        Box::pin(std::future::ready(Err(HostFailure::Protocol(
            "Session Endpoint V3 stream is unavailable".to_owned(),
        ))))
    }

    fn journal_page(
        &self,
        _request: SessionMuxClientFrame,
    ) -> CarrierHostFuture<'_, Result<SessionJournalPage, HostFailure>> {
        Box::pin(std::future::ready(Err(HostFailure::Protocol(
            "Session Endpoint V3 journal page is unavailable".to_owned(),
        ))))
    }

    fn mux_respond_actionable(
        &self,
        _request: SessionMuxClientFrame,
        _context: CallContext,
    ) -> CarrierHostFuture<'_, Result<u64, HostFailure>> {
        Box::pin(std::future::ready(Err(HostFailure::Protocol(
            "Session Endpoint V3 actionable response is unavailable".to_owned(),
        ))))
    }
}

#[derive(Debug, Error)]
pub enum HostFailure {
    #[error("endpoint request is malformed")]
    InvalidRequest,
    #[error("endpoint host is not ready")]
    NotReady,
    #[error("endpoint host is overloaded")]
    Overloaded,
    #[error("endpoint host protocol failure: {0}")]
    Protocol(String),
    #[error("endpoint host failed: {0}")]
    Internal(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionHostDescription {
    pub version: String,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "attachedSessions")]
    pub attached_sessions: usize,
    pub home: String,
    #[serde(rename = "canOpenPath")]
    pub can_open_path: bool,
}

/// Executes one validated request against the host while making the durable
/// rpcId registry the connection-independent result authority.
pub struct EndpointDispatcher {
    registry: RpcRegistry,
    in_flight: Mutex<HashSet<String>>,
}

impl EndpointDispatcher {
    #[must_use]
    pub fn new(registry: RpcRegistry) -> Self {
        Self {
            registry,
            in_flight: Mutex::new(HashSet::new()),
        }
    }

    pub async fn dispatch<H: EndpointHost + ?Sized>(
        &self,
        host: &H,
        request: ClientRequest,
    ) -> Result<ServerResponse, EndpointDispatchError> {
        self.dispatch_with_handoff(host, request, DurableHandoffSignal::new())
            .await
    }

    pub async fn dispatch_with_handoff<H: EndpointHost + ?Sized>(
        &self,
        host: &H,
        request: ClientRequest,
        handoff: DurableHandoffSignal,
    ) -> Result<ServerResponse, EndpointDispatchError> {
        let rpc_id = request.rpc_id.clone();
        if request.envelope_type != "client-request" {
            return Err(EndpointDispatchError::Request(RequestError::Envelope));
        }
        if !host.capabilities().contains(&request.method) {
            let result = typed_error(
                "unsupported-capability",
                "Capability is unavailable",
                format!(r#"{{"operation":{}}}"#, json_string(&request.method)),
            )?;
            return Ok(response(rpc_id, result));
        }
        if host.method_class(&request.method) == MethodClass::ReadOnly {
            {
                let mut in_flight = self
                    .in_flight
                    .lock()
                    .map_err(|_| EndpointDispatchError::Poisoned)?;
                if !in_flight.insert(rpc_id.clone()) {
                    return Ok(response(
                        rpc_id,
                        typed_error(
                            "request-in-progress",
                            "The identical rpcId is already executing",
                            "{}".to_owned(),
                        )?,
                    ));
                }
            }
            let _guard = InFlightGuard {
                set: &self.in_flight,
                rpc_id: rpc_id.clone(),
            };
            let result = host
                .call(EndpointHostCall {
                    rpc_id: request.rpc_id,
                    operation: request.method,
                    payload: request.payload,
                    recovering: false,
                    handoff,
                })
                .await;
            return Ok(response(rpc_id, result));
        }
        let begin = match self.registry.begin(&request) {
            Ok(begin) => begin,
            Err(RpcRegistryError::Conflict(_)) => {
                let operation = request.method.clone();
                return Ok(response(
                    rpc_id.clone(),
                    typed_error(
                        "idempotency-conflict",
                        "rpcId was already used for another request",
                        format!(
                            r#"{{"operation":{},"rpcId":{}}}"#,
                            json_string(&operation),
                            json_string(&rpc_id)
                        ),
                    )?,
                ));
            }
            Err(error) => return Err(error.into()),
        };
        let RpcBegin::Execute { claim, recovering } = begin else {
            let RpcBegin::Completed(result) = begin else {
                unreachable!();
            };
            handoff.mark_durable();
            return Ok(response(rpc_id, result));
        };

        {
            let mut in_flight = self
                .in_flight
                .lock()
                .map_err(|_| EndpointDispatchError::Poisoned)?;
            if !in_flight.insert(rpc_id.clone()) {
                return Ok(response(
                    rpc_id,
                    typed_error(
                        "request-in-progress",
                        "The identical rpcId is already executing",
                        "{}".to_owned(),
                    )?,
                ));
            }
        }
        let _guard = InFlightGuard {
            set: &self.in_flight,
            rpc_id: rpc_id.clone(),
        };

        let result = host
            .call(EndpointHostCall {
                rpc_id: request.rpc_id,
                operation: request.method,
                payload: request.payload,
                recovering,
                handoff: handoff.clone(),
            })
            .await;
        match handoff.proof()? {
            Some(proof) => {
                self.registry
                    .mark_handed_off(&claim, &proof.delivery, proof.durable_identity)?
            }
            None if handoff.is_durable() => {
                return Err(EndpointDispatchError::MissingHandoffProof);
            }
            None => {}
        }
        let completed = self.registry.complete(&claim, &result)?;
        handoff.mark_durable();
        Ok(response(rpc_id, completed))
    }

    #[must_use]
    pub fn registry(&self) -> &RpcRegistry {
        &self.registry
    }
}

struct InFlightGuard<'a> {
    set: &'a Mutex<HashSet<String>>,
    rpc_id: String,
}

impl Drop for InFlightGuard<'_> {
    fn drop(&mut self) {
        if let Ok(mut set) = self.set.lock() {
            set.remove(&self.rpc_id);
        }
    }
}

fn response(rpc_id: String, result: RpcResult) -> ServerResponse {
    ServerResponse {
        envelope_type: "server-response".to_owned(),
        rpc_id,
        result,
    }
}

fn typed_error(
    code: &str,
    message: &str,
    details: String,
) -> Result<RpcResult, EndpointDispatchError> {
    Ok(RpcResult {
        ok: false,
        value: None,
        error: Some(RpcError {
            code: code.to_owned(),
            message: message.to_owned(),
            details: IJsonValue::parse_str(&details)?,
        }),
    })
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a string cannot fail")
}

#[derive(Debug, Error)]
pub enum EndpointDispatchError {
    #[error("rpc registry failed: {0}")]
    Registry(#[from] RpcRegistryError),
    #[error("endpoint JSON failed: {0}")]
    Json(#[from] schema::SchemaError),
    #[error("endpoint request failed: {0}")]
    Request(#[from] RequestError),
    #[error("endpoint dispatcher mutex was poisoned")]
    Poisoned,
    #[error("durable mutation was exposed without a carrier handoff proof")]
    MissingHandoffProof,
    #[error("durable handoff proof failed: {0}")]
    Handoff(#[from] DurableHandoffError),
}
