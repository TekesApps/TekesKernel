//! Production Session Endpoint host assembly.
//!
//! This is a library boundary, not the stdin development shell. It exposes
//! only V3 unary routes whose author/read paths are real. Baseline, journal,
//! control, and actionable reads are carried exclusively by `remote.mux`.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use endpoint::{
    AttachmentAuthority, AttachmentErrorReason, AttachmentReadError, ClientRequest,
    DurableHandoffProof, EndpointDispatcher, EndpointHost, EndpointHostCall, EndpointHostFuture,
    EndpointSubscriptionHub, ForkSessionOperation, HostFailure, ManagementError, ManagementStore,
    MaterializedPrompt, NativeEndpoint, NativeEndpointError, PendingQueueTransaction,
    PromptMaterializeError, PromptPart, QueueRecoveryDriver, QueueTransactionCompletion,
    QueueTransactionDecision, QueueTransactionOperation, QueueTransactionState, RpcDurableIdentity,
    RpcError, RpcRegistry, RpcResult, SelectModelOperation, ServerResponse, SessionCreateOperation,
    SessionHostDescription,
};
use schema::IJsonValue;
use schema::OriginTuple;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointRouteClass {
    ReadOnly,
    Mutation,
    Stream,
    Respond,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndpointRoute {
    pub name: &'static str,
    pub class: EndpointRouteClass,
    pub implemented: bool,
}

pub const TEKES_UNARY_ROUTES: [EndpointRoute; 16] = [
    route("workspace.create", EndpointRouteClass::Mutation, true),
    route("workspace.rename", EndpointRouteClass::Mutation, true),
    route("workspace.relocate", EndpointRouteClass::Mutation, true),
    route(
        "workspace.archiveSession",
        EndpointRouteClass::Mutation,
        true,
    ),
    route(
        "workspace.unarchiveSession",
        EndpointRouteClass::Mutation,
        true,
    ),
    route("session.create", EndpointRouteClass::Mutation, true),
    route("session.prompt", EndpointRouteClass::Mutation, false),
    route("session.updateQueue", EndpointRouteClass::Mutation, false),
    route("session.cancel", EndpointRouteClass::Mutation, false),
    route("session.rename", EndpointRouteClass::Mutation, false),
    route("session.fork", EndpointRouteClass::Mutation, true),
    route("session.discard", EndpointRouteClass::Mutation, true),
    route("session.attachment", EndpointRouteClass::ReadOnly, true),
    route("session.models", EndpointRouteClass::ReadOnly, false),
    route("models.list", EndpointRouteClass::ReadOnly, false),
    route("session.selectModel", EndpointRouteClass::Mutation, false),
];

const fn route(name: &'static str, class: EndpointRouteClass, implemented: bool) -> EndpointRoute {
    EndpointRoute {
        name,
        class,
        implemented,
    }
}

pub type EndpointClock = dyn Fn() -> Result<String, EndpointAssemblyError> + Send + Sync;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeModelReadiness {
    pub id: String,
    pub efforts: Vec<String>,
    pub default_effort: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeProviderStatus {
    Ready,
    Failed { failure: RuntimeProviderFailure },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeProviderFailure {
    Unavailable,
    DialectUnproved,
    InvalidCredential,
    Network,
    Misconfigured,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeProviderReadiness {
    pub provider: String,
    pub status: RuntimeProviderStatus,
    pub models: Vec<RuntimeModelReadiness>,
}

/// Provider-process authority used by the endpoint's session-scoped model
/// catalog. The returned snapshot is a readiness observation, not durable
/// configuration; configured provider/model order remains authoritative.
pub trait ProviderReadinessAuthority: Send + Sync {
    fn readiness(
        &self,
        session_id: &str,
        config: &profile::ConfigSnapshot,
    ) -> Result<Vec<RuntimeProviderReadiness>, ProductionRouteFailure>;

    /// Synchronous post-commit hook. A successful durable configuration
    /// mutation is not reported until every live credential channel has
    /// observed the current SecretStore generation or has been stopped.
    fn config_mutation_succeeded(&self, _session_id: &str) -> Result<(), ProductionRouteFailure> {
        Ok(())
    }
}

/// Complete daemon/process-host delivery authority. Each method performs the
/// live-worker versus nonblocking locked-append classification while the
/// caller holds the per-session admission gate and returns only after the
/// worker receipt or append barrier is durable.
pub trait SessionDeliveryAuthority: Send + Sync {
    /// Session metadata that the inventory reports (the permission mode)
    /// changed outside the ledger. The production host re-announces the
    /// session so inventory subscribers re-diff their baseline; stubs ignore it.
    fn session_metadata_changed(&self, _session_id: &str) {}

    fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure>;

    fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure>;

    fn rename(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure>;

    /// Manual compaction request (worker-control `compact`, event R13-4):
    /// delivered to the live worker, which applies it at its next yield, or
    /// authored under the line lock by the supervisor when no worker is
    /// alive. Keyed by the origin; a retry is the original receipt. An
    /// authority that cannot compact reports the closed
    /// `unsupported-capability` failure rather than pretending.
    fn compact(
        &self,
        session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        let _ = session_id;
        Err(ProductionRouteFailure::new(
            "unsupported-capability",
            "Capability is unavailable",
            IJsonValue::parse_str(r#"{"operation":"compact"}"#).expect("static details are I-JSON"),
        ))
    }
}

/// Executes the whole durable management transaction, including worker
/// startup delivery/re-ack, operation completion, and admission release.
pub trait QueueTransactionAuthority: Send + Sync {
    fn execute(
        &self,
        session_id: &str,
        transaction: &worker_control::QueueTransaction,
    ) -> Result<worker_control::QueueTransactionResult, ProductionRouteFailure>;
}

struct QueueRecoveryAdapter<'a> {
    authority: &'a dyn QueueTransactionAuthority,
}

impl QueueRecoveryDriver for QueueRecoveryAdapter<'_> {
    fn execute(
        &self,
        pending: &PendingQueueTransaction,
    ) -> Result<QueueTransactionDecision, ManagementError> {
        let action = serde_json::from_slice(&pending.action.canonical_bytes()?)
            .map_err(ManagementError::from)?;
        let transaction = worker_control::QueueTransaction {
            delivery: pending.rpc_id.clone(),
            rpc_id: pending.rpc_id.clone(),
            target_seq: pending.target_seq,
            retract_origin: pending.retract_origin.clone(),
            action,
        };
        transaction.validate().map_err(|error| {
            ManagementError::QueueRecoveryFailed(format!(
                "prepared transaction is invalid: {error}"
            ))
        })?;
        if transaction.replacement_origin() != pending.replacement_origin.as_ref() {
            return Err(ManagementError::QueueRecoveryFailed(
                "prepared replacement origin disagrees with action".to_owned(),
            ));
        }
        let result = self
            .authority
            .execute(&pending.session_id, &transaction)
            .map_err(|error| ManagementError::QueueRecoveryFailed(error.code))?;
        result.validate_for(&transaction).map_err(|error| {
            ManagementError::QueueRecoveryFailed(format!(
                "worker result does not bind the prepared transaction: {error}"
            ))
        })?;
        Ok(queue_decision(&result.outcome))
    }
}

#[derive(Default)]
pub struct SessionAdmissionGates {
    gates: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl SessionAdmissionGates {
    pub fn gate(&self, session_id: &str) -> Result<Arc<Mutex<()>>, ProductionRouteFailure> {
        let mut gates = self.gates.lock().map_err(|_| {
            ProductionRouteFailure::new(
                "internal",
                "Endpoint operation failed",
                IJsonValue::parse_str("{}").expect("empty object is I-JSON"),
            )
        })?;
        Ok(gates
            .entry(session_id.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone())
    }
}

/// One shared lifecycle boundary for every mutation that submits input to a
/// session. The gate and active/archive check must remain a single critical
/// section so an archive cannot move the session between preflight and the
/// durable input barrier.
pub struct SessionInputAdmissionAuthority {
    root: PathBuf,
    gates: Arc<SessionAdmissionGates>,
}

impl SessionInputAdmissionAuthority {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            gates: Arc::new(SessionAdmissionGates::default()),
        }
    }

    #[must_use]
    pub fn gates(&self) -> Arc<SessionAdmissionGates> {
        Arc::clone(&self.gates)
    }

    /// The Kernel state root the gate validates sessions against.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn with_active_session<T, E>(
        &self,
        session_id: &str,
        map_failure: impl Fn(ProductionRouteFailure) -> E,
        operation: impl FnOnce() -> Result<T, E>,
    ) -> Result<T, E> {
        self.with_session_gate(session_id, &map_failure, || {
            if self.root.join("archive").join(session_id).is_dir() {
                return Err(map_failure(ProductionRouteFailure::new(
                    "archived",
                    "Session is archived",
                    session_details(session_id),
                )));
            }
            if !self.root.join("threads").join(session_id).is_dir() {
                return Err(map_failure(ProductionRouteFailure::new(
                    "session-not-found",
                    "Session was not found",
                    session_details(session_id),
                )));
            }
            operation()
        })
    }

    pub fn with_session_gate<T, E>(
        &self,
        session_id: &str,
        map_failure: impl Fn(ProductionRouteFailure) -> E,
        operation: impl FnOnce() -> Result<T, E>,
    ) -> Result<T, E> {
        if endpoint::validate_session_id(session_id).is_err() {
            return Err(map_failure(ProductionRouteFailure::new(
                "bad-request",
                "Request payload is invalid",
                IJsonValue::parse_str("{}").expect("empty object is I-JSON"),
            )));
        }
        let gate = self.gates.gate(session_id).map_err(&map_failure)?;
        let _admission = gate.lock().map_err(|_| {
            map_failure(ProductionRouteFailure::new(
                "internal",
                "Endpoint operation failed",
                IJsonValue::parse_str("{}").expect("empty object is I-JSON"),
            ))
        })?;
        operation()
    }
}

fn session_details(session_id: &str) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&json!({"sessionId":session_id})).expect("JSON"))
        .expect("session identity is I-JSON")
}

/// Injectable production seam for routes whose authority lives in the
/// daemon process table, provider runtime, instruction resolver, rewrite
/// projector, attachment decoder, or stream carrier. Implementations must
/// advertise only methods they execute end-to-end; the host validates each
/// closed top-level DTO before invoking the seam.
pub trait ProductionEndpointRoutes: Send + Sync {
    fn capabilities(&self) -> BTreeSet<String>;

    /// Classifies an independently versioned Client extension. V3 unary
    /// registrations are classified by `TEKES_UNARY_ROUTES` and never reach this
    /// seam. Returning `None` rejects an advertised non-V3 method at assembly.
    fn extension_method_class(&self, _method: &str) -> Option<endpoint::MethodClass> {
        None
    }

    /// Validates the extension's closed top-level DTO before execution. The
    /// host continues to own validation for every V3 unary registration.
    fn validate_extension_payload(
        &self,
        operation: &str,
        _payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        Err(ProductionRouteFailure::new(
            "unsupported-capability",
            "Capability is unavailable",
            IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))
                .expect("I-JSON"),
        ))
    }

    /// Independently versioned extensions own their exact error vocabulary.
    /// Returning false converts the failure to the host's closed `internal`.
    fn extension_failure_is_exact(
        &self,
        _operation: &str,
        _failure: &ProductionRouteFailure,
    ) -> bool {
        false
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure>;
}

/// Collision-free composition for independently versioned Client extension
/// handlers. A capability has exactly one owner, so later Slice handlers can
/// be assembled without last-writer-wins replacement.
pub struct CompositeProductionEndpointRoutes {
    owners: HashMap<String, Arc<dyn ProductionEndpointRoutes>>,
}

impl CompositeProductionEndpointRoutes {
    pub fn compose(
        routes: impl IntoIterator<Item = Arc<dyn ProductionEndpointRoutes>>,
    ) -> Result<Self, ProductionRouteCompositionError> {
        let mut owners = HashMap::new();
        let frozen = TEKES_UNARY_ROUTES
            .iter()
            .map(|route| route.name)
            .collect::<BTreeSet<_>>();
        for route in routes {
            for capability in route.capabilities() {
                if frozen.contains(capability.as_str()) {
                    return Err(ProductionRouteCompositionError::FrozenCapability(
                        capability,
                    ));
                }
                if owners
                    .insert(capability.clone(), Arc::clone(&route))
                    .is_some()
                {
                    return Err(ProductionRouteCompositionError::DuplicateCapability(
                        capability,
                    ));
                }
            }
        }
        Ok(Self { owners })
    }

    fn owner(&self, operation: &str) -> Option<&Arc<dyn ProductionEndpointRoutes>> {
        self.owners.get(operation)
    }
}

impl ProductionEndpointRoutes for CompositeProductionEndpointRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        self.owners.keys().cloned().collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<endpoint::MethodClass> {
        self.owner(method)?.extension_method_class(method)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        self.owner(operation)
            .ok_or_else(|| unsupported_route(operation))?
            .validate_extension_payload(operation, payload)
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        self.owner(operation)
            .is_some_and(|owner| owner.extension_failure_is_exact(operation, failure))
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        self.owner(&request.operation)
            .ok_or_else(|| unsupported_route(&request.operation))?
            .execute(request, payload, principal)
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum ProductionRouteCompositionError {
    #[error("production extension overlaps Session Endpoint V3 capability: {0}")]
    FrozenCapability(String),
    #[error("production route capability has more than one owner: {0}")]
    DuplicateCapability(String),
}

fn unsupported_route(operation: &str) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        "unsupported-capability",
        "Capability is unavailable",
        IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))
            .expect("operation object is I-JSON"),
    )
}

#[derive(Debug)]
pub struct ProductionRouteFailure {
    pub code: String,
    pub message: String,
    pub details: IJsonValue,
}

impl ProductionRouteFailure {
    pub fn new(code: impl Into<String>, message: impl Into<String>, details: IJsonValue) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details,
        }
    }
}

pub struct ProductionEndpointHost {
    storage_root: PathBuf,
    endpoint: NativeEndpoint,
    attachments: AttachmentAuthority,
    management: ManagementStore,
    description: SessionHostDescription,
    clock: Arc<EndpointClock>,
    principal: String,
    user_agent_dir: PathBuf,
    routes: Option<Arc<dyn ProductionEndpointRoutes>>,
    additive_extension_routes: bool,
    provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
    delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
    queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
    input_admission: Arc<SessionInputAdmissionAuthority>,
}

impl ProductionEndpointHost {
    pub fn mux_description(&self) -> endpoint::MuxHostDescription {
        let mut capabilities = endpoint::SessionEndpointCapability::required();
        capabilities.extend([
            endpoint::SessionEndpointCapability::Attachments,
            endpoint::SessionEndpointCapability::Management,
        ]);
        if self.provider_readiness.is_some() {
            capabilities.insert(endpoint::SessionEndpointCapability::Models);
        }
        endpoint::MuxHostDescription {
            protocol_version: endpoint::SESSION_ENDPOINT_PROTOCOL_VERSION,
            product: endpoint::MuxHostProduct {
                name: "TekesKernel".to_owned(),
                version: self.description.version.clone(),
            },
            capabilities,
            cwd: self.description.cwd.clone(),
            provider: self.description.provider.clone(),
            model: self.description.model.clone(),
            attached_sessions: self.description.attached_sessions,
            home: self.description.home.clone(),
            can_open_path: false,
        }
    }

    pub fn open(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_full_authorities(
            root,
            description,
            Arc::new(system_timestamp),
            None,
            None,
            None,
            None,
        )
    }

    pub fn open_with_clock(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Self::open_with_full_authorities(root, description, clock, None, None, None, None)
    }

    pub fn open_with_routes(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_full_authorities(root, description, clock, routes, None, None, None)
    }

    pub fn open_with_authorities(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_full_authorities(
            root,
            description,
            clock,
            routes,
            provider_readiness,
            None,
            None,
        )
    }

    pub fn open_with_full_authorities(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
        delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
        queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_route_ownership(
            root,
            description,
            clock,
            routes,
            provider_readiness,
            delivery_authority,
            queue_authority,
            false,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_with_full_authorities_and_session_admission(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
        delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
        queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
        input_admission: Arc<SessionInputAdmissionAuthority>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_route_ownership(
            root,
            description,
            clock,
            routes,
            provider_readiness,
            delivery_authority,
            queue_authority,
            false,
            Some(input_admission),
        )
    }

    /// Imports the legacy Slice-9 authority for unimplemented members of the
    /// V3 unary registry. Unlike the extension constructors, this entry point
    /// accepts only frozen names and cannot compose additive Client methods.
    pub fn open_with_route_authority(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Arc<dyn ProductionEndpointRoutes>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
        delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
        queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_route_ownership(
            root,
            description,
            clock,
            Some(routes),
            provider_readiness,
            delivery_authority,
            queue_authority,
            true,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn open_with_route_ownership(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
        delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
        queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
        imported_route_authority: bool,
        input_admission: Option<Arc<SessionInputAdmissionAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        if let Some(extension) = routes.as_ref() {
            let frozen = TEKES_UNARY_ROUTES
                .iter()
                .map(|route| route.name)
                .collect::<BTreeSet<_>>();
            if extension.capabilities().iter().any(|method| {
                if imported_route_authority {
                    !frozen.contains(method.as_str())
                } else {
                    frozen.contains(method.as_str())
                        || !matches!(
                            extension.extension_method_class(method),
                            Some(endpoint::MethodClass::ReadOnly | endpoint::MethodClass::Mutation)
                        )
                }
            }) {
                return Err(EndpointAssemblyError::UnknownRoute);
            }
        }
        let import_started_at = clock()?;
        let user_agent_dir = PathBuf::from(&description.home).join(".agents");
        let queue_recovery = queue_authority
            .as_deref()
            .map(|authority| QueueRecoveryAdapter { authority });
        let management = ManagementStore::open_at_with_queue_driver(
            root,
            &import_started_at,
            queue_recovery
                .as_ref()
                .map(|driver| driver as &dyn QueueRecoveryDriver),
        )?;
        let mut description = description;
        description.can_open_path = false;
        Ok(Self {
            storage_root: root.to_path_buf(),
            endpoint: NativeEndpoint::open(root)?,
            attachments: AttachmentAuthority::open(root),
            management,
            description,
            clock,
            // SAFETY: geteuid has no preconditions and captures the effective
            // service identity once at endpoint readiness.
            principal: format!("uid:{}", unsafe { libc::geteuid() }),
            user_agent_dir,
            routes,
            additive_extension_routes: !imported_route_authority,
            provider_readiness,
            delivery_authority,
            queue_authority,
            input_admission: input_admission.unwrap_or_else(|| {
                Arc::new(SessionInputAdmissionAuthority::new(root.to_path_buf()))
            }),
        })
    }

    #[must_use]
    pub fn implemented_capabilities() -> BTreeSet<String> {
        TEKES_UNARY_ROUTES
            .iter()
            .filter(|route| route.implemented)
            .map(|route| route.name.to_owned())
            .collect()
    }

    #[must_use]
    pub fn admission_gates(&self) -> Arc<SessionAdmissionGates> {
        self.input_admission.gates()
    }

    pub fn has_incomplete_management_operation(
        &self,
        session_id: &str,
    ) -> Result<bool, EndpointAssemblyError> {
        Ok(self
            .management
            .has_incomplete_session_operation(session_id)?)
    }

    fn execute(&self, request: EndpointHostCall) -> RpcResult {
        let value = match self.execute_inner(&request) {
            Ok(value) => return success(value),
            Err(error) => error,
        };
        failure(value.code, value.message, value.details)
    }

    fn execute_inner(&self, request: &EndpointHostCall) -> Result<IJsonValue, EndpointRpcFailure> {
        let payload: Value =
            serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)
                .map_err(internal_json)?;
        match request.operation.as_str() {
            "workspace.create" => self.create_workspace(request, &payload),
            "workspace.rename" => self.rename_workspace(request, &payload),
            "workspace.relocate" => self.relocate_workspace(request, &payload),
            "workspace.archiveSession" => self.archive_session(request, &payload),
            "workspace.unarchiveSession" => self.unarchive_session(request, &payload),
            "session.create" => self.create_session(request, &payload),
            "session.models" if self.provider_readiness.is_some() => self.models(&payload),
            "models.list" if self.provider_readiness.is_some() => self.draft_models(&payload),
            "session.selectModel" if self.provider_readiness.is_some() => {
                self.select_model(request, &payload)
            }
            "session.cancel" if self.delivery_authority.is_some() => self.cancel(request, &payload),
            "session.rename" if self.delivery_authority.is_some() => {
                self.rename_session(request, &payload)
            }
            "session.updateQueue" if self.queue_authority.is_some() => {
                self.update_queue(request, &payload)
            }
            "session.prompt" if self.delivery_authority.is_some() => self.prompt(request, &payload),
            "session.attachment" => self.attachment(&payload),
            "session.fork" => self.fork_session(request, &payload),
            "session.discard" => self.discard_session(request, &payload),
            other => self.execute_extension(other, request, &payload),
        }
    }

    fn execute_extension(
        &self,
        operation: &str,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        let Some(routes) = self.routes.as_ref() else {
            return Err(EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":operation}),
            ));
        };
        if !routes.capabilities().contains(operation) {
            return Err(EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":operation}),
            ));
        }
        let frozen = TEKES_UNARY_ROUTES
            .iter()
            .any(|route| route.name == operation);
        if frozen {
            validate_extension_payload(operation, payload)?;
        } else {
            routes
                .validate_extension_payload(operation, payload)
                .map_err(|error| {
                    if routes.extension_failure_is_exact(operation, &error) {
                        map_production_route(error)
                    } else {
                        internal_failure()
                    }
                })?;
        }
        routes
            .execute(request, payload, &self.principal)
            .map_err(|error| {
                if frozen {
                    map_production_route_for(operation, error)
                } else if routes.extension_failure_is_exact(operation, &error) {
                    map_production_route(error)
                } else {
                    internal_failure()
                }
            })
    }

    fn create_workspace(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["path"], &["path"])?;
        let path = required_string(payload, "path")?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let (workspace, created) = self
            .management
            .create_workspace(&request.rpc_id, &hash, path, &timestamp)
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"workspace":workspace,"created":created}))
    }

    fn rename_workspace(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["workspaceId", "title"],
            &["workspaceId", "title"],
        )?;
        let workspace_id = required_string(payload, "workspaceId")?;
        let title = required_string_allow_whitespace(payload, "title")?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let workspace = self
            .management
            .rename_workspace(&request.rpc_id, &hash, workspace_id, title, &timestamp)
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"workspace":workspace}))
    }

    fn relocate_workspace(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["workspaceId", "previousPath", "path"],
            &["workspaceId", "previousPath", "path"],
        )?;
        let workspace_id = required_string(payload, "workspaceId")?;
        let previous_path = required_string(payload, "previousPath")?;
        let path = required_string(payload, "path")?;
        // Workers hold this lock shared for their entire lifetime. Relocation therefore either
        // observes a fully quiescent workspace or fails before authoring an operation record.
        let _quiescence = store::NamedLock::try_exclusive(
            crate::process_host::workspace_quiescence_lock_path(&self.storage_root, workspace_id),
        )
        .map_err(|error| match error {
            store::StoreError::Busy => EndpointRpcFailure::new(
                "workspace-busy",
                "Workspace has a running session",
                json!({"workspaceId":workspace_id}),
            ),
            _ => internal_failure(),
        })?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let workspace = self
            .management
            .relocate_workspace(
                &request.rpc_id,
                &hash,
                workspace_id,
                previous_path,
                path,
                &timestamp,
            )
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"workspace":workspace}))
    }

    fn archive_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId"], &["sessionId"])?;
        let session_id = required_string(payload, "sessionId")?;
        self.input_admission.with_session_gate(
            session_id,
            |error| map_production_route_for("workspace.archiveSession", error),
            || {
                let timestamp = (self.clock)().map_err(internal_assembly)?;
                let hash = request_hash(request)?;
                let archived = self
                    .management
                    .archive_session(&request.rpc_id, &hash, session_id, &timestamp)
                    .map_err(map_management)?;
                mark_management_handoff(request)?;
                to_ijson(&json!({"archivedSessionIds":archived}))
            },
        )
    }

    fn unarchive_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId"], &["sessionId"])?;
        let session_id = required_string(payload, "sessionId")?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let restored = self
            .management
            .unarchive_session(&request.rpc_id, &hash, session_id, &timestamp)
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"sessionId":restored}))
    }

    fn create_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &[
                "workspaceId",
                "cwd",
                "sessionId",
                "agentPreset",
                "identityProfile",
            ],
            &[],
        )?;
        if payload.get("agentPreset").is_some() {
            return Err(EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.create.agentPreset"}),
            ));
        }
        let workspace_id = optional_string(payload, "workspaceId")?;
        let cwd = optional_string(payload, "cwd")?;
        let session_id = optional_string(payload, "sessionId")?;
        let identity_profile = optional_string(payload, "identityProfile")?
            .map(|value| {
                tools::IdentityProfile::parse(value)
                    .ok_or_else(|| invalid("identityProfile must be coding or general"))
            })
            .transpose()?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let session_id = self
            .management
            .create_session(SessionCreateOperation {
                rpc_id: &request.rpc_id,
                request_sha256: &hash,
                requested_session_id: session_id,
                workspace_id,
                cwd,
                identity_profile,
                user_agent_dir: &self.user_agent_dir,
                started_at: &timestamp,
                principal: &self.principal,
            })
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"sessionId":session_id}))
    }

    fn draft_models(&self, payload: &Value) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &[], &[])?;
        let repository =
            profile::ConfigRepository::open(&self.storage_root).map_err(|_| internal_failure())?;
        let providers = repository.providers().map_err(|_| internal_failure())?;
        let settings = repository.settings().map_err(|_| internal_failure())?;
        // This ephemeral snapshot projects global defaults only; no workspace or
        // session is created by opening the Composer's model menu.
        let config = profile::ConfigSnapshot {
            format: 1,
            revisions: profile::RevisionVector {
                workspace: 0,
                providers: providers.revision,
                settings: settings.revision,
                legacy_integrations: (),
                session_settings: None,
            },
            workspace: profile::ResolvedWorkspace {
                format: 1,
                revision: 0,
                id: String::new(),
                name: String::new(),
                folder_binding: None,
                selected_cwd: None,
                cwd: vec![],
                policy: Default::default(),
            },
            providers,
            settings,
            legacy_integrations: (),
            session_settings: None,
        };
        let readiness = self
            .provider_readiness
            .as_ref()
            .ok_or_else(internal_failure)?
            .readiness("", &config)
            .map_err(|error| map_production_route_for("models.list", error))?;
        model_projection(&config, &readiness)
    }

    fn models(&self, payload: &Value) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId"], &["sessionId"])?;
        let session_id = required_string(payload, "sessionId")?;
        let authority = self.provider_readiness.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.models"}),
            )
        })?;
        let config = self
            .endpoint
            .session_config_snapshot(session_id)
            .map_err(|error| map_endpoint(error, Some(session_id)))?;
        let readiness = authority
            .readiness(session_id, &config)
            .map_err(|error| map_production_route_for("session.models", error))?;
        model_projection(&config, &readiness)
    }

    fn select_model(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["sessionId", "provider", "model", "reasoningEffort"],
            &["sessionId", "provider", "model"],
        )?;
        let session_id = required_string(payload, "sessionId")?;
        let provider = required_string(payload, "provider")?;
        let model = required_string(payload, "model")?;
        let effort = optional_string(payload, "reasoningEffort")?;
        let authority = self.provider_readiness.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.selectModel"}),
            )
        })?;
        let gate = self.admission_gate(session_id)?;
        let _admission = gate.lock().map_err(|_| internal_failure())?;
        let config = self
            .endpoint
            .session_config_snapshot(session_id)
            .map_err(|error| map_endpoint(error, Some(session_id)))?;
        if !model_is_configured(&config, provider, model, effort) {
            return Err(EndpointRpcFailure::new(
                "model-unavailable",
                "Model is unavailable",
                json!({"provider":provider,"model":model}),
            ));
        }
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let selected = self
            .management
            .select_model(SelectModelOperation {
                rpc_id: &request.rpc_id,
                request_sha256: &hash,
                session_id,
                provider,
                model,
                reasoning_effort: effort,
                started_at: &timestamp,
            })
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        authority
            .config_mutation_succeeded(session_id)
            .map_err(|error| map_production_route_for("session.selectModel", error))?;
        let mut value = json!({"provider":selected.provider,"model":selected.model});
        if let Some(effort) = selected.reasoning_effort {
            value["reasoningEffort"] = Value::String(effort);
        }
        to_ijson(&json!({"selected":value}))
    }

    fn cancel(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId"], &["sessionId"])?;
        let session_id = required_string(payload, "sessionId")?;
        let authority = self.delivery_authority.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.cancel"}),
            )
        })?;
        let gate = self.admission_gate(session_id)?;
        let _admission = gate.lock().map_err(|_| internal_failure())?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let origin = endpoint_origin(
            &self.principal,
            session_id,
            "session.cancel",
            &request.rpc_id,
        );
        let receipt = authority
            .cancel(session_id, &timestamp, &origin)
            .map_err(|error| map_production_route_for("session.cancel", error))?;
        mark_event_handoff(request, receipt.seq)?;
        to_ijson(&json!({"accepted":true}))
    }

    fn rename_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId", "title"], &["sessionId", "title"])?;
        let session_id = required_string(payload, "sessionId")?;
        let title = required_string_allow_whitespace(payload, "title")?;
        validate_session_title(title).map_err(|()| {
            EndpointRpcFailure::new(
                "title-invalid",
                "Session title is invalid",
                json!({"sessionId":session_id}),
            )
        })?;
        let authority = self.delivery_authority.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.rename"}),
            )
        })?;
        let gate = self.admission_gate(session_id)?;
        let _admission = gate.lock().map_err(|_| internal_failure())?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let origin = endpoint_origin(
            &self.principal,
            session_id,
            "session.rename",
            &request.rpc_id,
        );
        let receipt = authority
            .rename(session_id, &timestamp, &origin, title)
            .map_err(|error| map_production_route_for("session.rename", error))?;
        mark_event_handoff(request, receipt.seq)?;
        to_ijson(&json!({"title":title,"seq":receipt.seq}))
    }

    fn update_queue(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["sessionId", "itemId", "action"],
            &["sessionId", "itemId", "action"],
        )?;
        let session_id = required_string(payload, "sessionId")?;
        let item_id = required_string(payload, "itemId")?;
        let target_seq = item_id
            .strip_prefix("input:")
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .ok_or_else(|| invalid("itemId must name a queued input"))?;
        let action = queue_action(payload.get("action").ok_or_else(|| invalid("action"))?)?;
        let authority = self.queue_authority.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.updateQueue"}),
            )
        })?;
        let gate = self.admission_gate(session_id)?;
        let _admission = gate.lock().map_err(|_| internal_failure())?;
        let transaction = worker_control::QueueTransaction {
            delivery: request.rpc_id.clone(),
            rpc_id: request.rpc_id.clone(),
            target_seq,
            retract_origin: endpoint_origin(
                &self.principal,
                session_id,
                "session.updateQueue",
                &format!("{}/retract", request.rpc_id),
            ),
            action: action.with_origin(endpoint_origin(
                &self.principal,
                session_id,
                "session.updateQueue",
                &format!("{}/replacement", request.rpc_id),
            )),
        };
        transaction
            .validate()
            .map_err(|_| invalid("queue transaction"))?;
        let action = to_ijson(&transaction.action)?;
        let hash = request_hash(request)?;
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let state = self
            .management
            .prepare_queue_transaction(QueueTransactionOperation {
                rpc_id: &request.rpc_id,
                request_sha256: &hash,
                session_id,
                target_seq,
                action: &action,
                retract_origin: &transaction.retract_origin,
                replacement_origin: transaction.replacement_origin(),
                asset_digests: &[],
                started_at: &timestamp,
            })
            .map_err(map_management)?;
        let completion = match state {
            QueueTransactionState::Complete(completion) => completion,
            QueueTransactionState::Pending(_) => {
                let result = authority
                    .execute(session_id, &transaction)
                    .map_err(|error| map_production_route_for("session.updateQueue", error))?;
                result
                    .validate_for(&transaction)
                    .map_err(|_| internal_failure())?;
                self.management
                    .complete_queue_transaction(&request.rpc_id, queue_decision(&result.outcome))
                    .map_err(map_management)?
            }
        };
        match completion {
            QueueTransactionCompletion::Committed => {
                mark_management_handoff(request)?;
                to_ijson(&json!({"accepted":true}))
            }
            QueueTransactionCompletion::Rejected { code, reason } => {
                Err(map_persisted_queue_rejection(&code, reason, item_id))
            }
        }
    }

    fn prompt(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["sessionId", "mode", "content", "clientTimeZone"],
            &["sessionId", "mode", "content"],
        )?;
        let session_id = required_string(payload, "sessionId")?;
        let steer = match required_string(payload, "mode")? {
            "queue" => false,
            "steer" => true,
            _ => return Err(invalid("mode violates the closed union")),
        };
        if let Some(time_zone) = optional_string(payload, "clientTimeZone")? {
            validate_time_zone(time_zone)?;
        }
        let parts: Vec<PromptPart> = serde_json::from_value(
            payload
                .get("content")
                .cloned()
                .ok_or_else(|| invalid("content is required"))?,
        )
        .map_err(|_| invalid("content violates the closed union"))?;
        let authority = self.delivery_authority.as_ref().ok_or_else(|| {
            EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":"session.prompt"}),
            )
        })?;
        self.input_admission.with_active_session(
            session_id,
            |error| map_production_route_for("session.prompt", error),
            || {
                let materialized = self
                    .attachments
                    .materialize_prompt_parts(session_id, &parts)
                    .map_err(map_prompt_error)?;
                let timestamp = (self.clock)().map_err(internal_assembly)?;
                let origin = endpoint_origin(
                    &self.principal,
                    session_id,
                    "session.prompt",
                    &request.rpc_id,
                );
                let receipt = authority
                    .prompt(session_id, &timestamp, &origin, &materialized, steer)
                    .map_err(|error| map_production_route_for("session.prompt", error))?;
                mark_event_handoff(request, receipt.seq)?;
                to_ijson(&json!({"accepted":true}))
            },
        )
    }

    fn attachment(&self, payload: &Value) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["sessionId", "attachmentId"],
            &["sessionId", "attachmentId"],
        )?;
        let session_id = required_string(payload, "sessionId")?;
        let attachment_id = required_string(payload, "attachmentId")?;
        self.ensure_active_session(session_id)?;
        let attachment = self
            .attachments
            .read_authorized(session_id, attachment_id)
            .map_err(map_attachment_read_error)?;
        to_ijson(&attachment)
    }

    fn fork_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(
            payload,
            &["sessionId", "atSeq", "ephemeral"],
            &["sessionId"],
        )?;
        let session_id = required_string(payload, "sessionId")?;
        let at_seq = optional_u64(payload, "atSeq")?;
        let ephemeral = optional_bool(payload, "ephemeral")?.unwrap_or(false);
        let timestamp = (self.clock)().map_err(internal_assembly)?;
        let hash = request_hash(request)?;
        let destination = self
            .management
            .fork_session(ForkSessionOperation {
                rpc_id: &request.rpc_id,
                request_sha256: &hash,
                source_session_id: session_id,
                at_endpoint_seq: at_seq,
                started_at: &timestamp,
                principal: &self.principal,
                ephemeral,
            })
            .map_err(map_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({"sessionId":destination}))
    }

    /// Removes a scratch ledger. Uses the archive gate so a discard never races
    /// a delivery on the same session; the management layer refuses a durable
    /// session (`not-ephemeral`) or a live worker (`session-running`).
    fn discard_session(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
    ) -> Result<IJsonValue, EndpointRpcFailure> {
        require_object_fields(payload, &["sessionId"], &["sessionId"])?;
        let session_id = required_string(payload, "sessionId")?;
        self.input_admission.with_session_gate(
            session_id,
            |error| map_production_route_for("session.discard", error),
            || {
                let timestamp = (self.clock)().map_err(internal_assembly)?;
                let hash = request_hash(request)?;
                let discarded = self
                    .management
                    .discard_session(&request.rpc_id, &hash, session_id, &timestamp)
                    .map_err(map_management)?;
                mark_management_handoff(request)?;
                to_ijson(&json!({"sessionId":discarded}))
            },
        )
    }

    fn admission_gate(&self, session_id: &str) -> Result<Arc<Mutex<()>>, EndpointRpcFailure> {
        self.input_admission
            .gates()
            .gate(session_id)
            .map_err(map_production_route)
    }

    fn ensure_active_session(&self, session_id: &str) -> Result<(), EndpointRpcFailure> {
        endpoint::validate_session_id(session_id).map_err(|_| invalid("sessionId is invalid"))?;
        if self.storage_root.join("archive").join(session_id).is_dir() {
            return Err(EndpointRpcFailure::new(
                "archived",
                "Session is archived",
                json!({"sessionId":session_id}),
            ));
        }
        if !self.storage_root.join("threads").join(session_id).is_dir() {
            return Err(EndpointRpcFailure::new(
                "session-not-found",
                "Session was not found",
                json!({"sessionId":session_id}),
            ));
        }
        Ok(())
    }
}

impl EndpointHost for ProductionEndpointHost {
    fn capabilities(&self) -> BTreeSet<String> {
        let mut capabilities = Self::implemented_capabilities();
        if let Some(routes) = self.routes.as_ref() {
            capabilities.extend(routes.capabilities());
        }
        if self.provider_readiness.is_some() {
            capabilities.insert("session.models".to_owned());
            capabilities.insert("models.list".to_owned());
            capabilities.insert("session.selectModel".to_owned());
        }
        if self.delivery_authority.is_some() {
            capabilities.insert("session.prompt".to_owned());
            capabilities.insert("session.cancel".to_owned());
            capabilities.insert("session.rename".to_owned());
        }
        if self.queue_authority.is_some() {
            capabilities.insert("session.updateQueue".to_owned());
        }
        capabilities
    }

    fn extension_capabilities(&self) -> BTreeSet<String> {
        if self.additive_extension_routes {
            self.routes
                .as_ref()
                .map_or_else(BTreeSet::new, |routes| routes.capabilities())
        } else {
            BTreeSet::new()
        }
    }

    fn method_class(&self, method: &str) -> endpoint::MethodClass {
        match TEKES_UNARY_ROUTES.iter().find(|route| route.name == method) {
            Some(route) if route.class == EndpointRouteClass::ReadOnly => {
                endpoint::MethodClass::ReadOnly
            }
            Some(route) if route.class == EndpointRouteClass::Mutation => {
                endpoint::MethodClass::Mutation
            }
            _ => self
                .routes
                .as_ref()
                .and_then(|routes| routes.extension_method_class(method))
                .unwrap_or(endpoint::MethodClass::Unknown),
        }
    }

    fn validate_request(&self, request: &ClientRequest) -> Result<(), HostFailure> {
        let payload: Value = serde_json::from_slice(
            &request
                .payload
                .canonical_bytes()
                .map_err(|_| HostFailure::InvalidRequest)?,
        )
        .map_err(|_| HostFailure::InvalidRequest)?;
        if TEKES_UNARY_ROUTES
            .iter()
            .any(|route| route.name == request.method)
        {
            validate_production_payload(&request.method, &payload)
                .map_err(|_| HostFailure::InvalidRequest)
        } else {
            self.routes
                .as_ref()
                .filter(|routes| routes.capabilities().contains(&request.method))
                .ok_or(HostFailure::InvalidRequest)?
                .validate_extension_payload(&request.method, &payload)
                .map_err(|_| HostFailure::InvalidRequest)
        }
    }

    fn call(&self, request: EndpointHostCall) -> EndpointHostFuture<'_> {
        Box::pin(std::future::ready(self.execute(request)))
    }
}

pub struct ProductionEndpointAssembly {
    host: ProductionEndpointHost,
    dispatcher: EndpointDispatcher,
    hub: EndpointSubscriptionHub,
}

impl ProductionEndpointAssembly {
    pub fn open(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Ok(Self {
            host: ProductionEndpointHost::open(root, description)?,
            dispatcher: EndpointDispatcher::new(RpcRegistry::open(root)?),
            hub: EndpointSubscriptionHub::default(),
        })
    }

    pub fn open_with_clock(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Ok(Self {
            host: ProductionEndpointHost::open_with_clock(root, description, clock)?,
            dispatcher: EndpointDispatcher::new(RpcRegistry::open(root)?),
            hub: EndpointSubscriptionHub::default(),
        })
    }

    pub fn open_with_routes(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Arc<dyn ProductionEndpointRoutes>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Ok(Self {
            host: ProductionEndpointHost::open_with_routes(root, description, clock, Some(routes))?,
            dispatcher: EndpointDispatcher::new(RpcRegistry::open(root)?),
            hub: EndpointSubscriptionHub::default(),
        })
    }

    pub fn open_with_route_authority(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Arc<dyn ProductionEndpointRoutes>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Ok(Self {
            host: ProductionEndpointHost::open_with_route_authority(
                root,
                description,
                clock,
                routes,
                None,
                None,
                None,
            )?,
            dispatcher: EndpointDispatcher::new(RpcRegistry::open(root)?),
            hub: EndpointSubscriptionHub::default(),
        })
    }

    pub fn open_with_authorities(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        Self::open_with_full_authorities(
            root,
            description,
            clock,
            routes,
            provider_readiness,
            None,
            None,
        )
    }

    pub fn open_with_full_authorities(
        root: impl AsRef<Path>,
        description: SessionHostDescription,
        clock: Arc<EndpointClock>,
        routes: Option<Arc<dyn ProductionEndpointRoutes>>,
        provider_readiness: Option<Arc<dyn ProviderReadinessAuthority>>,
        delivery_authority: Option<Arc<dyn SessionDeliveryAuthority>>,
        queue_authority: Option<Arc<dyn QueueTransactionAuthority>>,
    ) -> Result<Self, EndpointAssemblyError> {
        let root = root.as_ref();
        Ok(Self {
            host: ProductionEndpointHost::open_with_full_authorities(
                root,
                description,
                clock,
                routes,
                provider_readiness,
                delivery_authority,
                queue_authority,
            )?,
            dispatcher: EndpointDispatcher::new(RpcRegistry::open(root)?),
            hub: EndpointSubscriptionHub::default(),
        })
    }

    pub async fn dispatch(
        &self,
        request: ClientRequest,
    ) -> Result<ServerResponse, EndpointAssemblyError> {
        Ok(self.dispatcher.dispatch(&self.host, request).await?)
    }

    #[must_use]
    pub const fn hub(&self) -> &EndpointSubscriptionHub {
        &self.hub
    }

    #[must_use]
    pub const fn host(&self) -> &ProductionEndpointHost {
        &self.host
    }

    #[must_use]
    pub fn admission_gates(&self) -> Arc<SessionAdmissionGates> {
        self.host.admission_gates()
    }

    pub fn has_incomplete_management_operation(
        &self,
        session_id: &str,
    ) -> Result<bool, EndpointAssemblyError> {
        self.host.has_incomplete_management_operation(session_id)
    }
}

fn success(value: IJsonValue) -> RpcResult {
    RpcResult {
        ok: true,
        value: Some(value),
        error: None,
    }
}

fn model_projection(
    config: &profile::ConfigSnapshot,
    readiness: &[RuntimeProviderReadiness],
) -> Result<IJsonValue, EndpointRpcFailure> {
    let selected_provider = config
        .session_settings
        .as_ref()
        .map(|settings| settings.provider.as_str())
        .or(config.workspace.policy.provider.as_deref())
        .or(config.settings.default_provider.as_deref())
        .unwrap_or_default();
    let selected_model = config
        .session_settings
        .as_ref()
        .map(|settings| settings.model.as_str())
        .or(config.workspace.policy.model.as_deref())
        .or(config.settings.default_model.as_deref())
        .unwrap_or_default();
    let selected_effort = config
        .session_settings
        .as_ref()
        .and_then(|settings| settings.reasoning_effort.as_deref());
    let mut groups = Vec::new();
    let mut failures = Vec::new();
    for provider in &config.providers.providers {
        let provider_name = provider.name.as_deref().unwrap_or(&provider.id);
        let runtime = readiness
            .iter()
            .find(|candidate| candidate.provider == provider.id);
        let mut models = Vec::new();
        for configured in provider.models.iter().filter(|model| model.enabled) {
            let profile = provider::resolve_profile(provider, configured).ok();
            let efforts = profile
                .as_ref()
                .map(|profile| profile.reasoning_efforts().to_vec())
                .unwrap_or_default();
            let default_effort = profile
                .as_ref()
                .and_then(|profile| profile.default_reasoning_effort().map(str::to_owned));
            if efforts
                .iter()
                .any(|effort| effort.is_empty() || !effort.as_bytes().iter().all(u8::is_ascii))
            {
                return Err(internal_failure());
            }
            if efforts.iter().collect::<BTreeSet<_>>().len() != efforts.len() {
                return Err(internal_failure());
            }
            if default_effort
                .as_ref()
                .is_some_and(|default| !efforts.contains(default))
            {
                return Err(internal_failure());
            }
            let mut value = json!({"id":configured.id,"name":configured.id,
                "contextWindow":configured.context_window_tokens});
            if !efforts.is_empty() {
                value["reasoning"] = json!({
                    "efforts":efforts.iter().map(|effort| json!({
                        "id":effort,"name":effort
                    })).collect::<Vec<_>>()
                });
                if let Some(default) = default_effort.as_ref() {
                    value["reasoning"]["defaultEffort"] = Value::String(default.clone());
                }
            }
            models.push(value);
        }
        groups.push(json!({"id":provider.id,"name":provider_name,"models":models}));
        match runtime.map(|runtime| &runtime.status) {
            Some(RuntimeProviderStatus::Ready) => {}
            Some(RuntimeProviderStatus::Failed { failure }) => failures.push(json!({
                "id":provider.id,
                "name":provider_name,
                "message":runtime_failure_name(*failure),
            })),
            None => failures.push(json!({
                "id":provider.id,
                "name":provider_name,
                "message":"unavailable",
            })),
        }
    }
    let mut current = json!({"provider":selected_provider,"model":selected_model});
    if let Some(effort) = selected_effort {
        current["reasoningEffort"] = Value::String(effort.to_owned());
    }
    let routable = model_is_configured(config, selected_provider, selected_model, selected_effort);
    to_ijson(&json!({
        "current":current,
        "routable":routable,
        "groups":groups,
        "failures":failures,
    }))
}

fn model_is_configured(
    config: &profile::ConfigSnapshot,
    provider: &str,
    model: &str,
    effort: Option<&str>,
) -> bool {
    let Some(provider) = config
        .providers
        .providers
        .iter()
        .find(|candidate| candidate.id == provider)
    else {
        return false;
    };
    let Some(model) = provider
        .models
        .iter()
        .find(|candidate| candidate.id == model && candidate.enabled)
    else {
        return false;
    };
    effort.is_none_or(|effort| {
        provider::resolve_profile(provider, model).is_ok_and(|profile| {
            profile
                .reasoning_efforts()
                .iter()
                .any(|item| item == effort)
        })
    })
}

const fn runtime_failure_name(failure: RuntimeProviderFailure) -> &'static str {
    match failure {
        RuntimeProviderFailure::Unavailable => "unavailable",
        RuntimeProviderFailure::DialectUnproved => "dialect-unproved",
        RuntimeProviderFailure::InvalidCredential => "invalid-credential",
        RuntimeProviderFailure::Network => "network",
        RuntimeProviderFailure::Misconfigured => "misconfigured",
    }
}

fn map_production_route(error: ProductionRouteFailure) -> EndpointRpcFailure {
    EndpointRpcFailure {
        code: error.code,
        message: error.message,
        details: error.details,
    }
}

fn map_production_route_for(operation: &str, error: ProductionRouteFailure) -> EndpointRpcFailure {
    if !production_failure_is_exact_for(operation, &error) {
        let message = sanitized_route_log_message(&error.message);
        eprintln!(
            "endpoint-route-error-folded: operation={operation} code={} allowed={} exact_payload={} message={message}",
            error.code,
            route_allows_error(operation, &error.code),
            route_error_is_exact(&error)
        );
        return internal_failure();
    }
    map_production_route(error)
}

fn sanitized_route_log_message(message: &str) -> String {
    let mut end = message.len().min(512);
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    let value = IJsonValue::parse(
        &serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))
            .expect("route log message is JSON"),
    )
    .expect("route log message is I-JSON");
    let scanned = match tools::SecretScanner::default().scan(&value) {
        tools::SecretScan::Clean(value) | tools::SecretScan::Redacted(value) => value,
        tools::SecretScan::Withheld(_) => return "<withheld>".to_owned(),
    };
    serde_json::to_value(scanned)
        .ok()
        .and_then(|value| value["detail"].as_str().map(str::to_owned))
        .unwrap_or_else(|| "<withheld>".to_owned())
}

pub(crate) fn production_failure_is_exact_for(
    operation: &str,
    error: &ProductionRouteFailure,
) -> bool {
    route_allows_error(operation, &error.code) && route_error_is_exact(error)
}

fn route_error_is_exact(error: &ProductionRouteFailure) -> bool {
    let Some(expected_message) = route_error_message(&error.code) else {
        return false;
    };
    if error.message != expected_message {
        return false;
    }
    let Ok(bytes) = error.details.canonical_bytes() else {
        return false;
    };
    let Ok(Value::Object(details)) = serde_json::from_slice::<Value>(&bytes) else {
        return false;
    };

    match error.code.as_str() {
        "bad-request" | "internal" => details.is_empty(),
        "unsupported-capability" => exact_string_fields(&details, &["operation"], &[]),
        "idempotency-conflict" | "accepted-but-not-confirmed" => {
            exact_string_fields(&details, &["rpcId", "operation"], &[])
        }
        "session-not-found" | "session-running" | "archived" | "not-archived" | "title-invalid" => {
            exact_string_fields(&details, &["sessionId"], &[])
        }
        "workspace-not-found" | "workspace-busy" => {
            exact_string_fields(&details, &["workspaceId"], &[])
        }
        "workspace-invalid-path" | "workspace-ambiguous" => {
            exact_string_fields(&details, &["path"], &[])
        }
        "workspace-name-conflict" => exact_string_fields(&details, &["name"], &[]),
        "workspace-title-invalid" => exact_string_fields(&details, &["title"], &[]),
        "session-conflict" => {
            exact_string_fields(&details, &["sessionId", "requestedCwd"], &["existingCwd"])
        }
        "invalid-cursor" => exact_string_fields(&details, &["cursor"], &[]),
        "invalid-at-seq" => {
            details.len() == 2
                && details.get("sessionId").is_some_and(Value::is_string)
                && details
                    .get("atSeq")
                    .is_some_and(|value| value.as_u64().is_some())
        }
        "active-session-limit" => {
            details.len() == 1 && details.get("limit").and_then(Value::as_u64) == Some(256)
        }
        "queue-item-not-found" | "steer-unavailable" => {
            exact_string_fields(&details, &["itemId"], &[])
        }
        "attachment-error" => {
            details.len() == 1
                && details
                    .get("reason")
                    .and_then(Value::as_str)
                    .is_some_and(|reason| {
                        matches!(
                            reason,
                            "INVALID_BASE64"
                                | "MEDIA_TYPE_MISMATCH"
                                | "TOO_LARGE"
                                | "INVALID_DIMENSIONS"
                                | "DECODE_FAILED"
                                | "NOT_REFERENCED"
                                | "CORRUPT"
                                | "QUEUE_EDIT_NON_TEXT"
                        )
                    })
        }
        "model-unavailable" => exact_string_fields(&details, &["provider", "model"], &[]),
        _ => false,
    }
}

fn route_error_message(code: &str) -> Option<&'static str> {
    Some(match code {
        "bad-request" => "Request payload is invalid",
        "unsupported-capability" => "Capability is unavailable",
        "idempotency-conflict" => "rpcId was already used for another request",
        "session-not-found" => "Session was not found",
        "workspace-not-found" => "Workspace was not found",
        "workspace-busy" => "Workspace has a running session",
        "workspace-invalid-path" => "Workspace path is invalid",
        "workspace-ambiguous" => "Workspace path matches more than one workspace",
        "workspace-name-conflict" => "Workspace title already exists",
        "workspace-title-invalid" => "Workspace title is invalid",
        "session-conflict" => "Session identity conflicts with existing state",
        "session-running" => "Session has a running worker",
        "archived" => "Session is archived",
        "not-archived" => "Session is not archived",
        "invalid-cursor" => "Session cursor is invalid",
        "invalid-at-seq" => "Fork position is not a complete projection boundary",
        "active-session-limit" => "Active session limit reached",
        "queue-item-not-found" => "Queued item is no longer pending",
        "steer-unavailable" => "Current turn no longer accepts steering",
        "attachment-error" => "Image attachment is unavailable",
        "model-unavailable" => "Model is unavailable",
        "title-invalid" => "Session title is invalid",
        "accepted-but-not-confirmed" => "Mutation was accepted but its receipt was not confirmed",
        "internal" => "Endpoint operation failed",
        _ => return None,
    })
}

fn exact_string_fields(
    details: &serde_json::Map<String, Value>,
    required: &[&str],
    optional: &[&str],
) -> bool {
    if required
        .iter()
        .any(|key| !details.get(*key).is_some_and(Value::is_string))
        || optional
            .iter()
            .any(|key| details.get(*key).is_some_and(|value| !value.is_string()))
    {
        return false;
    }
    let optional_present = optional
        .iter()
        .filter(|key| details.contains_key(**key))
        .count();
    details.len() == required.len() + optional_present
}

fn route_allows_error(operation: &str, code: &str) -> bool {
    if code == "internal" {
        return true;
    }
    match operation {
        "workspace.create" => matches!(
            code,
            "workspace-invalid-path"
                | "workspace-title-invalid"
                | "workspace-name-conflict"
                | "idempotency-conflict"
        ),
        "workspace.rename" => matches!(
            code,
            "workspace-not-found"
                | "workspace-title-invalid"
                | "workspace-name-conflict"
                | "idempotency-conflict"
        ),
        "workspace.archiveSession" => matches!(
            code,
            "session-not-found" | "session-running" | "ephemeral" | "idempotency-conflict"
        ),
        "session.discard" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "not-ephemeral"
                | "session-running"
                | "idempotency-conflict"
        ),
        "workspace.unarchiveSession" => matches!(
            code,
            "session-not-found" | "not-archived" | "active-session-limit" | "idempotency-conflict"
        ),
        "session.create" => matches!(
            code,
            "bad-request"
                | "workspace-not-found"
                | "workspace-invalid-path"
                | "workspace-ambiguous"
                | "session-conflict"
                | "active-session-limit"
                | "unsupported-capability"
                | "idempotency-conflict"
        ),
        "session.models" | "session.attachment" => {
            matches!(code, "session-not-found" | "archived" | "attachment-error")
                && (operation == "session.attachment" || code != "attachment-error")
        }
        "session.prompt" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "steer-unavailable"
                | "attachment-error"
                | "idempotency-conflict"
                | "accepted-but-not-confirmed"
        ),
        "session.updateQueue" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "queue-item-not-found"
                | "steer-unavailable"
                | "attachment-error"
                | "idempotency-conflict"
                | "accepted-but-not-confirmed"
        ),
        "session.cancel" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "idempotency-conflict"
                | "accepted-but-not-confirmed"
        ),
        "session.rename" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "title-invalid"
                | "idempotency-conflict"
                | "accepted-but-not-confirmed"
        ),
        "session.fork" => matches!(
            code,
            "session-not-found"
                | "archived"
                | "invalid-at-seq"
                | "active-session-limit"
                | "idempotency-conflict"
        ),
        "session.selectModel" => matches!(
            code,
            "session-not-found"
                | "session-running"
                | "archived"
                | "model-unavailable"
                | "idempotency-conflict"
        ),
        _ => false,
    }
}

fn internal_failure() -> EndpointRpcFailure {
    EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({}))
}

fn failure(code: String, message: String, details: IJsonValue) -> RpcResult {
    RpcResult {
        ok: false,
        value: None,
        error: Some(RpcError {
            code,
            message,
            details,
        }),
    }
}

fn required_string<'a>(value: &'a Value, key: &str) -> Result<&'a str, EndpointRpcFailure> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| invalid(&format!("{key} is required")))
}

fn required_string_allow_whitespace<'a>(
    value: &'a Value,
    key: &str,
) -> Result<&'a str, EndpointRpcFailure> {
    value
        .as_object()
        .and_then(|object| object.get(key))
        .and_then(Value::as_str)
        .ok_or_else(|| invalid(&format!("{key} is required")))
}

fn optional_bool(value: &Value, key: &str) -> Result<Option<bool>, EndpointRpcFailure> {
    let Some(value) = value.as_object().and_then(|object| object.get(key)) else {
        return Ok(None);
    };
    value
        .as_bool()
        .map(Some)
        .ok_or_else(|| invalid(&format!("{key} must be a boolean")))
}

fn optional_string<'a>(value: &'a Value, key: &str) -> Result<Option<&'a str>, EndpointRpcFailure> {
    let Some(value) = value.as_object().and_then(|object| object.get(key)) else {
        return Ok(None);
    };
    value
        .as_str()
        .filter(|value| !value.is_empty())
        .map(Some)
        .ok_or_else(|| invalid(&format!("{key} must be a nonempty string")))
}

fn require_object_fields(
    value: &Value,
    allowed: &[&str],
    required: &[&str],
) -> Result<(), EndpointRpcFailure> {
    let object = value
        .as_object()
        .ok_or_else(|| invalid("payload must be an object"))?;
    if object.keys().any(|key| !allowed.contains(&key.as_str()))
        || required.iter().any(|key| !object.contains_key(*key))
    {
        return Err(invalid("payload fields do not match the closed schema"));
    }
    Ok(())
}

fn optional_u64(value: &Value, key: &str) -> Result<Option<u64>, EndpointRpcFailure> {
    let Some(value) = value.as_object().and_then(|object| object.get(key)) else {
        return Ok(None);
    };
    value
        .as_u64()
        .map(Some)
        .ok_or_else(|| invalid(&format!("{key} must be a non-negative integer")))
}

fn to_ijson(value: &impl serde::Serialize) -> Result<IJsonValue, EndpointRpcFailure> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(internal_json)?).map_err(internal_schema)
}

fn request_hash(request: &EndpointHostCall) -> Result<String, EndpointRpcFailure> {
    let payload: Value =
        serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)
            .map_err(internal_json)?;
    let bytes = serde_json_canonicalizer::to_vec(&json!({
        "type":"client-request",
        "rpcId":request.rpc_id,
        "method":request.operation,
        "payload":payload,
    }))
    .map_err(|_| EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({})))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn mark_management_handoff(request: &EndpointHostCall) -> Result<(), EndpointRpcFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: "management".to_owned(),
            durable_identity: Some(RpcDurableIdentity {
                kind: "management-operation".to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|_| EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({})))
}

fn mark_event_handoff(request: &EndpointHostCall, seq: u64) -> Result<(), EndpointRpcFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: "locked-append".to_owned(),
            durable_identity: Some(RpcDurableIdentity {
                kind: "event-origin".to_owned(),
                id: request.rpc_id.clone(),
                seq: Some(seq),
            }),
        })
        .map_err(|_| internal_failure())
}

fn endpoint_origin(
    principal: &str,
    session_id: &str,
    operation: &str,
    rpc_id: &str,
) -> OriginTuple {
    OriginTuple {
        principal: principal.to_owned(),
        client: endpoint::ORIGIN_CLIENT.to_owned(),
        target: session_id.to_owned(),
        op: operation.to_owned(),
        key: rpc_id.to_owned(),
    }
}

fn validate_session_title(title: &str) -> Result<(), ()> {
    if title.trim() != title
        || title.is_empty()
        || title.len() > 256
        || title.chars().any(char::is_control)
    {
        return Err(());
    }
    Ok(())
}

enum PendingQueueAction {
    Edit { content: Vec<schema::Block> },
    Remove,
    Steer,
}

impl PendingQueueAction {
    fn with_origin(self, origin: OriginTuple) -> worker_control::QueueTransactionAction {
        match self {
            Self::Edit { content } => worker_control::QueueTransactionAction::Edit {
                replacement_origin: origin,
                content,
                steer: false,
                assets: None,
            },
            Self::Remove => worker_control::QueueTransactionAction::Remove,
            Self::Steer => worker_control::QueueTransactionAction::Steer {
                replacement_origin: origin,
            },
        }
    }
}

fn queue_action(value: &Value) -> Result<PendingQueueAction, EndpointRpcFailure> {
    let object = value
        .as_object()
        .ok_or_else(|| invalid("action must be an object"))?;
    let kind = object
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("action kind is required"))?;
    match kind {
        "remove" if object.len() == 1 => Ok(PendingQueueAction::Remove),
        "steer" if object.len() == 1 => Ok(PendingQueueAction::Steer),
        "edit"
            if object.len() == 2
                && object.contains_key("content")
                && object
                    .keys()
                    .all(|key| matches!(key.as_str(), "kind" | "content")) =>
        {
            let values = object
                .get("content")
                .and_then(Value::as_array)
                .filter(|values| !values.is_empty())
                .ok_or_else(|| invalid("edit content must be nonempty"))?;
            let mut content = Vec::with_capacity(values.len());
            for value in values {
                let block = value
                    .as_object()
                    .filter(|block| {
                        block.len() == 2
                            && block.get("type").and_then(Value::as_str) == Some("text")
                    })
                    .and_then(|block| block.get("text"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        EndpointRpcFailure::new(
                            "attachment-error",
                            "Image attachment is unavailable",
                            json!({"reason":"QUEUE_EDIT_NON_TEXT"}),
                        )
                    })?;
                content.push(schema::Block::Text {
                    text: block.to_owned(),
                });
            }
            Ok(PendingQueueAction::Edit { content })
        }
        _ => Err(invalid("queue action violates the closed union")),
    }
}

fn map_queue_rejection(
    code: worker_control::QueueTransactionRejectCode,
    reason: Option<String>,
    item_id: &str,
) -> EndpointRpcFailure {
    match code {
        worker_control::QueueTransactionRejectCode::QueueItemNotFound => EndpointRpcFailure::new(
            "queue-item-not-found",
            "Queued item is no longer pending",
            json!({"itemId":item_id}),
        ),
        worker_control::QueueTransactionRejectCode::SteerUnavailable => EndpointRpcFailure::new(
            "steer-unavailable",
            "Current turn no longer accepts steering",
            json!({"itemId":item_id}),
        ),
        worker_control::QueueTransactionRejectCode::AttachmentError => EndpointRpcFailure::new(
            "attachment-error",
            "Image attachment is unavailable",
            json!({"reason":reason.unwrap_or_else(|| "QUEUE_EDIT_NON_TEXT".to_owned())}),
        ),
    }
}

fn queue_decision(outcome: &worker_control::QueueTransactionOutcome) -> QueueTransactionDecision {
    match outcome {
        worker_control::QueueTransactionOutcome::Committed { .. } => {
            QueueTransactionDecision::Committed
        }
        worker_control::QueueTransactionOutcome::Rejected { code, reason } => {
            QueueTransactionDecision::Rejected {
                code: match code {
                    worker_control::QueueTransactionRejectCode::QueueItemNotFound => {
                        "queue-item-not-found"
                    }
                    worker_control::QueueTransactionRejectCode::SteerUnavailable => {
                        "steer-unavailable"
                    }
                    worker_control::QueueTransactionRejectCode::AttachmentError => {
                        "attachment-error"
                    }
                }
                .to_owned(),
                reason: reason.clone(),
            }
        }
    }
}

fn map_persisted_queue_rejection(
    code: &str,
    reason: Option<String>,
    item_id: &str,
) -> EndpointRpcFailure {
    let code = match code {
        "queue-item-not-found" => worker_control::QueueTransactionRejectCode::QueueItemNotFound,
        "steer-unavailable" => worker_control::QueueTransactionRejectCode::SteerUnavailable,
        "attachment-error" => worker_control::QueueTransactionRejectCode::AttachmentError,
        _ => return internal_failure(),
    };
    map_queue_rejection(code, reason, item_id)
}

fn validate_time_zone(value: &str) -> Result<(), EndpointRpcFailure> {
    if value == "UTC" {
        return Ok(());
    }
    if value.trim() != value
        || !value.is_ascii()
        || value.split('/').count() < 2
        || value.split('/').enumerate().any(|(index, segment)| {
            segment.is_empty()
                || segment == "."
                || segment == ".."
                || segment.bytes().enumerate().any(|(position, byte)| {
                    !(byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'+' | b'.' | b'-'))
                        || (index == 0 && position == 0 && !byte.is_ascii_alphabetic())
                })
        })
    {
        return Err(invalid("clientTimeZone is invalid"));
    }
    let zone = Path::new("/usr/share/zoneinfo").join(value);
    if !zone.is_file() {
        return Err(invalid("clientTimeZone is unknown"));
    }
    Ok(())
}

fn map_prompt_error(error: PromptMaterializeError) -> EndpointRpcFailure {
    match error {
        PromptMaterializeError::EmptyContent
        | PromptMaterializeError::InvalidName
        | PromptMaterializeError::InvalidMediaType => invalid("prompt content is invalid"),
        PromptMaterializeError::Attachment(reason) => attachment_failure(reason),
        PromptMaterializeError::SessionNotFound(session_id) => EndpointRpcFailure::new(
            "session-not-found",
            "Session was not found",
            json!({"sessionId":session_id}),
        ),
        PromptMaterializeError::Archived(session_id) => EndpointRpcFailure::new(
            "archived",
            "Session is archived",
            json!({"sessionId":session_id}),
        ),
        _ => internal_failure(),
    }
}

fn map_attachment_read_error(error: AttachmentReadError) -> EndpointRpcFailure {
    match error {
        AttachmentReadError::SessionNotFound(session_id) => EndpointRpcFailure::new(
            "session-not-found",
            "Session was not found",
            json!({"sessionId":session_id}),
        ),
        AttachmentReadError::Archived(session_id) => EndpointRpcFailure::new(
            "archived",
            "Session is archived",
            json!({"sessionId":session_id}),
        ),
        AttachmentReadError::Attachment(reason) => attachment_failure(reason),
        _ => internal_failure(),
    }
}

fn attachment_failure(reason: AttachmentErrorReason) -> EndpointRpcFailure {
    EndpointRpcFailure::new(
        "attachment-error",
        "Image attachment is unavailable",
        json!({"reason":attachment_reason(reason)}),
    )
}

const fn attachment_reason(reason: AttachmentErrorReason) -> &'static str {
    match reason {
        AttachmentErrorReason::InvalidBase64 => "INVALID_BASE64",
        AttachmentErrorReason::MediaTypeMismatch => "MEDIA_TYPE_MISMATCH",
        AttachmentErrorReason::TooLarge => "TOO_LARGE",
        AttachmentErrorReason::InvalidDimensions => "INVALID_DIMENSIONS",
        AttachmentErrorReason::DecodeFailed => "DECODE_FAILED",
        AttachmentErrorReason::NotReferenced => "NOT_REFERENCED",
        AttachmentErrorReason::Corrupt => "CORRUPT",
        AttachmentErrorReason::QueueEditNonText => "QUEUE_EDIT_NON_TEXT",
        AttachmentErrorReason::InlineMediaType => "INLINE_MEDIA_TYPE",
        AttachmentErrorReason::UnknownReceipt => "UNKNOWN_RECEIPT",
    }
}

fn validate_extension_payload(operation: &str, payload: &Value) -> Result<(), EndpointRpcFailure> {
    let (allowed, required): (&[&str], &[&str]) = match operation {
        "models.list" => (&[], &[]),
        "workspace.archiveSession"
        | "workspace.unarchiveSession"
        | "session.cancel"
        | "session.models" => (&["sessionId"], &["sessionId"]),
        "session.attachment" => (
            &["sessionId", "attachmentId"],
            &["sessionId", "attachmentId"],
        ),
        "session.create" => (
            &[
                "workspaceId",
                "cwd",
                "sessionId",
                "agentPreset",
                "identityProfile",
            ],
            &[],
        ),
        "session.prompt" => (
            &["sessionId", "mode", "content", "clientTimeZone"],
            &["sessionId", "mode", "content"],
        ),
        "session.updateQueue" => (
            &["sessionId", "itemId", "action"],
            &["sessionId", "itemId", "action"],
        ),
        "session.rename" => (&["sessionId", "title"], &["sessionId", "title"]),
        "session.fork" => (&["sessionId", "atSeq", "ephemeral"], &["sessionId"]),
        "session.discard" => (&["sessionId"], &["sessionId"]),
        "session.selectModel" => (
            &["sessionId", "provider", "model", "reasoningEffort"],
            &["sessionId", "provider", "model"],
        ),
        _ => {
            return Err(EndpointRpcFailure::new(
                "unsupported-capability",
                "Capability is unavailable",
                json!({"operation":operation}),
            ));
        }
    };
    require_object_fields(payload, allowed, required)?;
    Ok(())
}

fn validate_production_payload(operation: &str, payload: &Value) -> Result<(), EndpointRpcFailure> {
    match operation {
        "workspace.create" => require_object_fields(payload, &["path"], &["path"]),
        "workspace.rename" => require_object_fields(
            payload,
            &["workspaceId", "title"],
            &["workspaceId", "title"],
        ),
        "workspace.relocate" => require_object_fields(
            payload,
            &["workspaceId", "previousPath", "path"],
            &["workspaceId", "previousPath", "path"],
        ),
        other => validate_extension_payload(other, payload),
    }
}

fn invalid(_message: &str) -> EndpointRpcFailure {
    EndpointRpcFailure::new("bad-request", "Request payload is invalid", json!({}))
}

fn map_endpoint(error: NativeEndpointError, session: Option<&str>) -> EndpointRpcFailure {
    let session = session.unwrap_or_default();
    match error {
        NativeEndpointError::Archived => EndpointRpcFailure::new(
            "archived",
            "Session is archived",
            json!({"sessionId":session}),
        ),
        NativeEndpointError::NotFound => EndpointRpcFailure::new(
            "session-not-found",
            "Session was not found",
            json!({"sessionId":session}),
        ),
        _other => EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({})),
    }
}

fn map_management(error: ManagementError) -> EndpointRpcFailure {
    match error {
        ManagementError::InvalidPath(path) => EndpointRpcFailure::new(
            "workspace-invalid-path",
            "Workspace path is invalid",
            json!({"path":path}),
        ),
        ManagementError::InvalidTitle(title) => EndpointRpcFailure::new(
            "workspace-title-invalid",
            "Workspace title is invalid",
            json!({"title":title}),
        ),
        ManagementError::NameConflict(name) => EndpointRpcFailure::new(
            "workspace-name-conflict",
            "Workspace title already exists",
            json!({"name":name}),
        ),
        ManagementError::WorkspaceNotFound(workspace_id) => EndpointRpcFailure::new(
            "workspace-not-found",
            "Workspace was not found",
            json!({"workspaceId":workspace_id}),
        ),
        ManagementError::WorkspaceAmbiguous(path) => EndpointRpcFailure::new(
            "workspace-ambiguous",
            "Workspace path matches more than one workspace",
            json!({"path":path}),
        ),
        ManagementError::InvalidCreateSelection => {
            EndpointRpcFailure::new("bad-request", "Request payload is invalid", json!({}))
        }
        ManagementError::IdempotencyConflict { rpc_id, operation } => EndpointRpcFailure::new(
            "idempotency-conflict",
            "rpcId was already used for another request",
            json!({"rpcId":rpc_id,"operation":operation}),
        ),
        ManagementError::SessionNotFound(session_id) => EndpointRpcFailure::new(
            "session-not-found",
            "Session was not found",
            json!({"sessionId":session_id}),
        ),
        ManagementError::SessionArchived(session_id) => EndpointRpcFailure::new(
            "archived",
            "Session is archived",
            json!({"sessionId":session_id}),
        ),
        ManagementError::SessionRunning(session_id) => EndpointRpcFailure::new(
            "session-running",
            "Session has a running worker",
            json!({"sessionId":session_id}),
        ),
        ManagementError::SessionEphemeral(session_id) => EndpointRpcFailure::new(
            "ephemeral",
            "Session is ephemeral and cannot be archived",
            json!({"sessionId":session_id}),
        ),
        ManagementError::SessionNotEphemeral(session_id) => EndpointRpcFailure::new(
            "not-ephemeral",
            "Session is not ephemeral",
            json!({"sessionId":session_id}),
        ),
        ManagementError::InvalidAtSeq { session_id, at_seq } => EndpointRpcFailure::new(
            "invalid-at-seq",
            "Fork position is not a complete projection boundary",
            json!({"sessionId":session_id,"atSeq":at_seq.unwrap_or(0)}),
        ),
        ManagementError::SessionConflict {
            session_id,
            requested_cwd,
            existing_cwd,
        } => {
            let mut details = json!({
                "sessionId":session_id,
                "requestedCwd":requested_cwd,
            });
            if let Some(existing_cwd) = existing_cwd {
                details["existingCwd"] = Value::String(existing_cwd);
            }
            EndpointRpcFailure::new(
                "session-conflict",
                "Session identity conflicts with existing state",
                details,
            )
        }
        ManagementError::ActiveSessionLimit => EndpointRpcFailure::new(
            "active-session-limit",
            "Active session limit reached",
            json!({"limit":256}),
        ),
        ManagementError::Store(store::StoreError::Busy) => {
            EndpointRpcFailure::new("session-running", "Session has a running worker", json!({}))
        }
        ManagementError::EndpointType(_) => {
            EndpointRpcFailure::new("bad-request", "Request payload is invalid", json!({}))
        }
        _ => EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({})),
    }
}

fn internal_json(_error: serde_json::Error) -> EndpointRpcFailure {
    EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({}))
}

fn internal_schema(_error: schema::SchemaError) -> EndpointRpcFailure {
    EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({}))
}

fn internal_assembly(_error: EndpointAssemblyError) -> EndpointRpcFailure {
    EndpointRpcFailure::new("internal", "Endpoint operation failed", json!({}))
}

struct EndpointRpcFailure {
    code: String,
    message: String,
    details: IJsonValue,
}

impl EndpointRpcFailure {
    fn new(code: &str, message: &str, details: Value) -> Self {
        let details =
            IJsonValue::parse(&serde_json::to_vec(&details).unwrap_or_else(|_| b"{}".to_vec()))
                .unwrap_or_else(|_| IJsonValue::parse_str("{}").expect("empty object is I-JSON"));
        Self {
            code: code.to_owned(),
            message: message.to_owned(),
            details,
        }
    }
}

fn system_timestamp() -> Result<String, EndpointAssemblyError> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| EndpointAssemblyError::Clock(error.to_string()))?
        .as_secs();
    let days = i64::try_from(seconds / 86_400)
        .map_err(|_| EndpointAssemblyError::Clock("timestamp exceeds i64".to_owned()))?;
    let day_seconds = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    let hour = day_seconds / 3_600;
    let minute = day_seconds % 3_600 / 60;
    let second = day_seconds % 60;
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.000Z"
    ))
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let days = days_since_epoch + 719_468;
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
    (year, month, day)
}

#[derive(Debug, Error)]
pub enum EndpointAssemblyError {
    #[error("native endpoint failed: {0}")]
    Endpoint(#[from] NativeEndpointError),
    #[error("rpc registry failed: {0}")]
    Registry(#[from] endpoint::RpcRegistryError),
    #[error("rpc dispatch failed: {0}")]
    Dispatch(#[from] endpoint::EndpointDispatchError),
    #[error("endpoint clock failed: {0}")]
    Clock(String),
    #[error("endpoint management failed: {0}")]
    Management(#[from] endpoint::ManagementError),
    #[error("endpoint route extension advertised a non-V3 registration")]
    UnknownRoute,
}
