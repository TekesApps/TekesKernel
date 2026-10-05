use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::{
    CallContext, CarrierHostFuture, ClientRequest, ClientResponse, EndpointCarrierHost,
    EndpointDispatcher, EndpointHost, EndpointStreamReceiver, HostFailure, HostReadiness,
    MethodClass, RequestState, RespondAuthorization, RespondDecision, RespondLifecycle,
    RespondPrepareContext, RespondReceipt, RpcBegin, RpcDurableIdentity, RpcLookup, RpcRegistry,
    RpcResult, ServerRequest, ServerResponse, StreamChannel, StreamErrorCode,
};

pub const SESSION_ENDPOINT_PUBLIC_METHODS: [&str; 17] = [
    "workspace.create",
    "workspace.rename",
    "workspace.relocate",
    "workspace.archiveSession",
    "workspace.unarchiveSession",
    "session.create",
    "session.prompt",
    "session.updateQueue",
    "session.cancel",
    "session.rename",
    "session.fork",
    "session.discard",
    "session.attachment",
    "session.models",
    "models.list",
    "session.selectModel",
    "remote.mux",
];

pub trait RespondAuthority: Send + Sync + 'static {
    /// Locates the request derived from the semantic ledger without
    /// authoring. `None` is the closed unknown-rpc-id result and consumes no
    /// mutation identity.
    fn locate(&self, rpc_id: &str) -> Result<Option<LocatedRespond>, HostFailure>;

    /// Authors and full-syncs exactly one approval_response barrier. The
    /// returned seq is the semantic commit point; that event is the
    /// request's resolution.
    fn author(
        &self,
        authorization: RespondAuthorization,
    ) -> Result<RespondAuthorReceipt, HostFailure>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RespondDelivery {
    LiveWorker,
    LockedAppend,
    RecoveredSemanticBarrier,
}

impl RespondDelivery {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWorker => "worker-control",
            Self::LockedAppend => "supervisor-locked-append",
            Self::RecoveredSemanticBarrier => "supervisor-recovered-semantic-barrier",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RespondAuthorReceipt {
    pub semantic_seq: u64,
    pub delivery: RespondDelivery,
}

#[derive(Clone, Debug)]
pub struct LocatedRespond {
    pub thread_folder: PathBuf,
    /// The request and its ledger resolution, derived from the semantic
    /// ledger at lookup time.
    pub state: RequestState,
    pub context: RespondPrepareContext,
    /// The supervisor-owned per-session admission gate shared with prompt,
    /// stop, queue mutation, and recovery classification.
    pub admission: Arc<Mutex<()>>,
}

pub trait CarrierRespondHandler: Send + Sync + 'static {
    fn respond(
        &self,
        response: ClientResponse,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<RespondReceipt, HostFailure>>;

    fn respond_mux(
        &self,
        actionable_id: String,
        expected_revision: u64,
        outcome: schema::IJsonValue,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<u64, HostFailure>>;
}

pub struct JournalRespondHandler<A> {
    authority: A,
    registry: RpcRegistry,
}

impl<A> JournalRespondHandler<A> {
    #[must_use]
    pub const fn new(authority: A, registry: RpcRegistry) -> Self {
        Self {
            authority,
            registry,
        }
    }
}

impl<A: RespondAuthority> CarrierRespondHandler for JournalRespondHandler<A> {
    fn respond(
        &self,
        response: ClientResponse,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<RespondReceipt, HostFailure>> {
        Box::pin(async move {
            response
                .validate()
                .map_err(|error| HostFailure::Protocol(error.to_string()))?;
            let Some(located) = self.authority.locate(&response.rpc_id)? else {
                return Ok(RespondReceipt {
                    accepted: false,
                    reason: Some("unknown-rpc-id".to_owned()),
                });
            };
            let admission = Arc::clone(&located.admission);
            let _admission = admission
                .lock()
                .map_err(|_| HostFailure::Internal("respond admission gate poisoned".to_owned()))?;
            // The first lookup only identifies the admission gate. Repeating
            // it while holding that gate closes archive/stop/tail races before
            // any mutation rpc identity is consumed.
            let Some(located) = self.authority.locate(&response.rpc_id)? else {
                return Ok(RespondReceipt {
                    accepted: false,
                    reason: Some("unknown-rpc-id".to_owned()),
                });
            };
            if !Arc::ptr_eq(&admission, &located.admission) {
                return Err(HostFailure::Internal(
                    "respond admission authority changed during lookup".to_owned(),
                ));
            }
            let claim_request = respond_claim(&response)?;
            let state = &located.state;
            if state.resolution.is_some() {
                return match self.registry.lookup(&claim_request) {
                    Ok(RpcLookup::Completed(result)) => receipt_from_cached(result),
                    Ok(RpcLookup::Pending(claim)) => {
                        let semantic_seq = state.resolution.expect("filtered").0;
                        let proof = self
                            .registry
                            .handoff(&claim)
                            .map_err(|error| HostFailure::Internal(error.to_string()))?
                            .map_or_else(
                                || {
                                    respond_handoff(
                                        &state.request.session_id,
                                        semantic_seq,
                                        RespondDelivery::RecoveredSemanticBarrier,
                                    )
                                },
                                |(delivery, identity)| {
                                    (
                                        delivery,
                                        identity.unwrap_or_else(|| RpcDurableIdentity {
                                            kind: "event".to_owned(),
                                            id: state.request.session_id.clone(),
                                            seq: Some(semantic_seq),
                                        }),
                                    )
                                },
                            );
                        self.registry
                            .mark_handed_off(&claim, &proof.0, Some(proof.1.clone()))
                            .map_err(|error| HostFailure::Internal(error.to_string()))?;
                        let result = accepted_result()?;
                        self.registry
                            .complete(&claim, &result)
                            .map_err(|error| HostFailure::Internal(error.to_string()))?;
                        context
                            .handoff
                            .mark_handed_off(crate::DurableHandoffProof {
                                delivery: proof.0,
                                durable_identity: Some(proof.1),
                            })
                            .map_err(|error| HostFailure::Internal(error.to_string()))?;
                        Ok(RespondReceipt {
                            accepted: true,
                            reason: None,
                        })
                    }
                    Ok(RpcLookup::Missing) => Err(HostFailure::Internal(
                        "resolved endpoint request has no rpc carrier".to_owned(),
                    )),
                    Err(crate::RpcRegistryError::Conflict(_)) => Ok(RespondReceipt {
                        accepted: false,
                        reason: Some("already-resolved".to_owned()),
                    }),
                    Err(error) => Err(HostFailure::Internal(error.to_string())),
                };
            }
            match RespondLifecycle::prepare(Some(state), &response, &located.context)
                .map_err(|error| HostFailure::Internal(error.to_string()))?
            {
                RespondDecision::Reject(receipt) | RespondDecision::CachedSuccess(receipt) => {
                    Ok(receipt)
                }
                RespondDecision::Authorize(authorization) => {
                    let claim = match self.registry.begin(&claim_request) {
                        Ok(RpcBegin::Execute { claim, .. }) => claim,
                        Ok(RpcBegin::Completed(_)) => {
                            return Err(HostFailure::Internal(
                                "unresolved endpoint request has a completed rpc carrier"
                                    .to_owned(),
                            ));
                        }
                        Err(error) => return Err(HostFailure::Internal(error.to_string())),
                    };
                    let authored = self.authority.author(authorization.clone())?;
                    let semantic_seq = authored.semantic_seq;
                    let (delivery, durable_identity) =
                        respond_handoff(&authorization.session_id, semantic_seq, authored.delivery);
                    self.registry
                        .mark_handed_off(&claim, &delivery, Some(durable_identity.clone()))
                        .map_err(|error| HostFailure::Internal(error.to_string()))?;
                    // The durable approval_response is the resolution; there
                    // is no separate request record to close.
                    let receipt = RespondReceipt {
                        accepted: true,
                        reason: None,
                    };
                    self.registry
                        .complete(&claim, &accepted_result()?)
                        .map_err(|error| HostFailure::Internal(error.to_string()))?;
                    context
                        .handoff
                        .mark_handed_off(crate::DurableHandoffProof {
                            delivery,
                            durable_identity: Some(durable_identity),
                        })
                        .map_err(|error| HostFailure::Internal(error.to_string()))?;
                    Ok(receipt)
                }
            }
        })
    }

    fn respond_mux(
        &self,
        actionable_id: String,
        expected_revision: u64,
        outcome: schema::IJsonValue,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<u64, HostFailure>> {
        Box::pin(async move {
            let Some(located) = self.authority.locate(&actionable_id)? else {
                return Err(HostFailure::Protocol(
                    "actionable is no longer pending".to_owned(),
                ));
            };
            let state = &located.state;
            if state.resolution.is_some() {
                return Err(HostFailure::Protocol(
                    "actionable is already resolved".to_owned(),
                ));
            }
            if state.request.causal_kernel_seq != expected_revision {
                return Err(HostFailure::Protocol(format!(
                    "actionable revision conflict: expected {}, received {expected_revision}",
                    state.request.causal_kernel_seq
                )));
            }
            let mut value: serde_json::Value = serde_json::from_slice(
                &outcome
                    .canonical_bytes()
                    .map_err(|error| HostFailure::Protocol(error.to_string()))?,
            )
            .map_err(|error| HostFailure::Protocol(error.to_string()))?;
            let object = value.as_object_mut().ok_or_else(|| {
                HostFailure::Protocol("actionable outcome must be an object".to_owned())
            })?;
            match object.get("sessionId").and_then(serde_json::Value::as_str) {
                Some(session_id) if session_id != state.request.session_id => {
                    return Err(HostFailure::Protocol(
                        "actionable outcome targets another session".to_owned(),
                    ));
                }
                Some(_) => {}
                None => {
                    object.insert(
                        "sessionId".to_owned(),
                        serde_json::Value::String(state.request.session_id.clone()),
                    );
                }
            }
            let response = ClientResponse {
                envelope_type: "client-response".to_owned(),
                rpc_id: actionable_id,
                result: crate::ClientResponseResult {
                    ok: true,
                    value: Some(
                        schema::IJsonValue::parse(
                            &serde_json::to_vec(&value)
                                .map_err(|error| HostFailure::Internal(error.to_string()))?,
                        )
                        .map_err(|error| HostFailure::Protocol(error.to_string()))?,
                    ),
                    error: None,
                },
            };
            let receipt = self.respond(response, context).await?;
            if !receipt.accepted {
                return Err(HostFailure::Protocol(
                    receipt
                        .reason
                        .unwrap_or_else(|| "actionable response rejected".to_owned()),
                ));
            }
            Ok(expected_revision)
        })
    }
}

fn respond_handoff(
    session_id: &str,
    semantic_seq: u64,
    delivery: RespondDelivery,
) -> (String, RpcDurableIdentity) {
    (
        delivery.as_str().to_owned(),
        RpcDurableIdentity {
            kind: "event".to_owned(),
            id: session_id.to_owned(),
            seq: Some(semantic_seq),
        },
    )
}

fn respond_claim(response: &ClientResponse) -> Result<ClientRequest, HostFailure> {
    let payload = schema::IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(response)
            .map_err(|error| HostFailure::Internal(error.to_string()))?,
    )
    .map_err(|error| HostFailure::Internal(error.to_string()))?;
    Ok(ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: response.rpc_id.clone(),
        method: "respond".to_owned(),
        payload,
    })
}

fn accepted_result() -> Result<RpcResult, HostFailure> {
    Ok(RpcResult {
        ok: true,
        value: Some(
            schema::IJsonValue::parse_str(r#"{"accepted":true}"#)
                .map_err(|error| HostFailure::Internal(error.to_string()))?,
        ),
        error: None,
    })
}

fn receipt_from_cached(result: RpcResult) -> Result<RespondReceipt, HostFailure> {
    if !result.ok || result.error.is_some() {
        return Err(HostFailure::Internal(
            "respond rpc carrier cached a non-success result".to_owned(),
        ));
    }
    let value = result.value.ok_or_else(|| {
        HostFailure::Internal("respond rpc carrier omitted its success value".to_owned())
    })?;
    let bytes = value
        .canonical_bytes()
        .map_err(|error| HostFailure::Internal(error.to_string()))?;
    let receipt: RespondReceipt =
        serde_json::from_slice(&bytes).map_err(|error| HostFailure::Internal(error.to_string()))?;
    if receipt.accepted && receipt.reason.is_none() {
        Ok(receipt)
    } else {
        Err(HostFailure::Internal(
            "respond rpc carrier cached invalid receipt bytes".to_owned(),
        ))
    }
}

pub trait CarrierStreamHandler: Send + Sync + 'static {
    fn open_stream(
        &self,
        channel: StreamChannel,
    ) -> CarrierHostFuture<'_, Result<EndpointStreamReceiver, HostFailure>>;

    fn stream_error(
        &self,
        channel: StreamChannel,
        code: StreamErrorCode,
    ) -> Result<ServerRequest, HostFailure>;

    fn mux_description(&self) -> Result<crate::MuxHostDescription, HostFailure>;

    fn open_mux_stream(
        &self,
        generation: u64,
        target: crate::SessionStreamTarget,
    ) -> CarrierHostFuture<'_, Result<crate::SessionStreamReceiver, HostFailure>>;

    fn journal_page(
        &self,
        request: crate::SessionMuxClientFrame,
    ) -> CarrierHostFuture<'_, Result<crate::SessionJournalPage, HostFailure>>;
}

/// Adapter that combines a production unary `EndpointHost` with the durable
/// respond and stream seams. This is the only advertised registry: transport
/// compares it in both directions and never sees placeholder successes.
pub struct ComposedEndpointCarrierHost<H, R, S> {
    unary: H,
    dispatcher: EndpointDispatcher,
    respond: R,
    streams: S,
    ready: AtomicBool,
}

impl<H, R, S> ComposedEndpointCarrierHost<H, R, S> {
    #[must_use]
    pub fn new(unary: H, registry: RpcRegistry, respond: R, streams: S) -> Self {
        Self {
            unary,
            dispatcher: EndpointDispatcher::new(registry),
            respond,
            streams,
            ready: AtomicBool::new(true),
        }
    }

    pub fn set_readiness(&self, readiness: HostReadiness) {
        self.ready
            .store(readiness == HostReadiness::Ready, Ordering::Release);
    }
}

impl<H, R, S> EndpointCarrierHost for ComposedEndpointCarrierHost<H, R, S>
where
    H: EndpointHost + 'static,
    R: CarrierRespondHandler,
    S: CarrierStreamHandler,
{
    fn readiness(&self) -> HostReadiness {
        if self.ready.load(Ordering::Acquire) {
            HostReadiness::Ready
        } else {
            HostReadiness::NotReady
        }
    }

    fn registered_methods(&self) -> BTreeSet<String> {
        let public = SESSION_ENDPOINT_PUBLIC_METHODS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut methods = self
            .unary
            .capabilities()
            .into_iter()
            .filter(|method| public.contains(method.as_str()))
            .collect::<BTreeSet<_>>();
        methods.insert("remote.mux".to_owned());
        methods.extend(self.unary.extension_capabilities());
        methods
    }

    fn advertised_extension_methods(&self) -> BTreeSet<String> {
        self.unary.extension_capabilities()
    }

    fn classify_method(&self, method: &str) -> MethodClass {
        if method == "remote.mux" {
            MethodClass::ReadOnly
        } else if (SESSION_ENDPOINT_PUBLIC_METHODS.contains(&method)
            && self.unary.capabilities().contains(method))
            || self.unary.extension_capabilities().contains(method)
        {
            self.unary.method_class(method)
        } else {
            MethodClass::Unknown
        }
    }

    fn unary(
        &self,
        request: ClientRequest,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<ServerResponse, HostFailure>> {
        Box::pin(async move {
            self.unary.validate_request(&request)?;
            self.dispatcher
                .dispatch_with_handoff(&self.unary, request, context.handoff)
                .await
                .map_err(|error| HostFailure::Internal(error.to_string()))
        })
    }

    fn respond(
        &self,
        response: ClientResponse,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<RespondReceipt, HostFailure>> {
        self.respond.respond(response, context)
    }

    fn open_stream(
        &self,
        channel: StreamChannel,
    ) -> CarrierHostFuture<'_, Result<EndpointStreamReceiver, HostFailure>> {
        self.streams.open_stream(channel)
    }

    fn stream_error(
        &self,
        channel: StreamChannel,
        code: StreamErrorCode,
    ) -> Result<ServerRequest, HostFailure> {
        self.streams.stream_error(channel, code)
    }

    fn mux_description(&self) -> Result<crate::MuxHostDescription, HostFailure> {
        self.streams.mux_description()
    }

    fn open_mux_stream(
        &self,
        generation: u64,
        target: crate::SessionStreamTarget,
    ) -> CarrierHostFuture<'_, Result<crate::SessionStreamReceiver, HostFailure>> {
        self.streams.open_mux_stream(generation, target)
    }

    fn journal_page(
        &self,
        request: crate::SessionMuxClientFrame,
    ) -> CarrierHostFuture<'_, Result<crate::SessionJournalPage, HostFailure>> {
        self.streams.journal_page(request)
    }

    fn mux_respond_actionable(
        &self,
        request: crate::SessionMuxClientFrame,
        context: CallContext,
    ) -> CarrierHostFuture<'_, Result<u64, HostFailure>> {
        let crate::SessionMuxClientFrame::ActionableRespond {
            actionable_id,
            expected_revision,
            outcome,
            ..
        } = request
        else {
            return Box::pin(std::future::ready(Err(HostFailure::InvalidRequest)));
        };
        self.respond
            .respond_mux(actionable_id, expected_revision, outcome, context)
    }
}
