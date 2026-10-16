//! Session-owned MCP execution over HTTP or host-supplied stdio channels.
//! registration, stdio processes, OAuth and installation do not live here.
use crate::continuation_journal::{
    ContinuationBinding, ContinuationJournal, ContinuationJournalError,
};
use crate::mcp_continuation::{resolve_task, task_authority};
use crate::production_tool_control::{
    DynamicExecutionOutcome, DynamicSupervisorAuthority, SupervisorOperationError,
};
use crate::tool_control::ExternalEffectResolution;
use mcp::{
    HttpRequestAuthorization, HttpTransport, McpBroker, McpBrokerHandle, McpCancellationToken,
    McpClient, McpError, McpImplementation, McpPeer, McpPoolKey, McpTool, McpToolCallContext,
    PipeTransport, ProtocolMode, project_catalog_partial,
};
use profile::{DynamicTool, DynamicToolCatalog, DynamicToolSourceKind};
use provider::{SecretRecord, SecretResolution, SecretStore};
use schema::IJsonValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use worker_control::ToolControl;
use worker_control::continuation::{ToolContinuationRequest, ToolContinuationResponse};
use zeroize::Zeroize;

#[derive(Clone)]
pub struct SecretAccess {
    pub store: Arc<dyn SecretStore>,
    channels: Arc<std::sync::RwLock<Arc<dyn StdioConnector>>>,
}
/// The host opens streams and owns the process behind them. Kernel never
/// interprets channel names as commands, paths or environment variables.
pub type StdioOpen<'a> = std::pin::Pin<
    Box<
        dyn std::future::Future<Output = std::io::Result<(std::fs::File, std::fs::File)>>
            + Send
            + 'a,
    >,
>;
pub trait StdioConnector: Send + Sync {
    fn open<'a>(&'a self, session: &'a str, channel: &'a str) -> StdioOpen<'a>;
}
struct NoStdio;
impl StdioConnector for NoStdio {
    fn open<'a>(&'a self, _session: &'a str, _channel: &'a str) -> StdioOpen<'a> {
        Box::pin(async { Err(std::io::Error::other("host stdio channel unavailable")) })
    }
}
impl SecretAccess {
    pub fn read_only(store: Arc<dyn SecretStore>) -> Self {
        Self {
            store,
            channels: Arc::new(std::sync::RwLock::new(Arc::new(NoStdio))),
        }
    }
}
pub struct McpSessionBinding {
    pub catalog: DynamicToolCatalog,
    pub authority: Arc<McpDynamicSupervisorAuthority>,
    refused_tools: Vec<String>,
}
impl McpSessionBinding {
    /// Excuse only exact policy-selected names refused by the projection.
    /// These names never enter the executable catalog or dispatch routes.
    pub fn unavailable_tools(&self, allowed: &[String]) -> Vec<String> {
        self.refused_tools
            .iter()
            .filter(|name| allowed.contains(name))
            .cloned()
            .collect()
    }
}
pub struct McpRuntime {
    secrets: SecretAccess,
}
struct PreparedServer {
    key: McpPoolKey,
    peer: Box<dyn McpPeer>,
    tools: Vec<McpTool>,
    credential_floors: Vec<(String, u64)>,
}
impl McpRuntime {
    pub fn new(store: Arc<dyn SecretStore>) -> Result<Self, McpError> {
        Ok(Self {
            secrets: SecretAccess::read_only(store),
        })
    }
    pub fn set_stdio_connector(&self, connector: Arc<dyn StdioConnector>) {
        *self
            .secrets
            .channels
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = connector;
    }
    pub fn prepare_session(
        &self,
        session: &str,
        connections: &[schema::McpConnection],
    ) -> Result<McpSessionBinding, McpError> {
        let broker = Arc::new(McpBroker::start()?);
        let mut routes = BTreeMap::new();
        let mut tools = Vec::new();
        let mut refused_tools = std::collections::BTreeSet::new();
        // The authority owns every registered peer, including on partial failure.
        let mut authority = McpDynamicSupervisorAuthority::new(
            Arc::clone(&broker),
            BTreeMap::new(),
            self.secrets.clone(),
        );
        for connection in connections {
            let prepared = with_preparation_runtime(|runtime| {
                prepare_server_cancellable(
                    session,
                    connection,
                    self.secrets.clone(),
                    runtime,
                    McpCancellationToken::default(),
                )
            })?;
            let projection = project_catalog_partial(&connection.name, &prepared.tools, true)
                .map_err(|e| McpError::Conflict(e.to_string()))?;
            refused_tools.extend(
                projection
                    .refused
                    .iter()
                    .filter_map(|refusal| refusal.name.clone()),
            );
            let mut server_routes = BTreeMap::new();
            for dynamic in &projection.catalog.tools {
                let effect = prepared
                    .tools
                    .iter()
                    .find(|tool| {
                        mcp::project_name(&connection.name, &tool.name)
                            .ok()
                            .as_deref()
                            == Some(&dynamic.name)
                    })
                    .ok_or_else(|| McpError::Conflict("MCP projection lost remote tool".into()))?;
                let reconcile = dynamic
                    .external_effect
                    .as_ref()
                    .map(|binding| {
                        prepared
                            .tools
                            .iter()
                            .find(|tool| tool.name == binding.reconcile_tool)
                            .cloned()
                            .ok_or_else(|| {
                                McpError::Conflict("MCP reconciliation tool missing".into())
                            })
                    })
                    .transpose()?;
                let contract =
                    McpRouteContract::capture(prepared.peer.as_ref(), effect.clone(), reconcile)?;
                let route = McpRoute {
                    key: prepared.key.clone(),
                    remote_name: effect.name.clone(),
                    credential_floors: prepared.credential_floors.clone(),
                    server: connection.clone(),
                    contract,
                };
                if routes.insert(dynamic.name.clone(), route.clone()).is_some() {
                    return Err(McpError::Conflict("duplicate host MCP tool name".into()));
                }
                server_routes.insert(dynamic.name.clone(), route);
            }
            if server_routes.is_empty() {
                continue;
            }
            broker
                .handle()
                .register(prepared.key, prepared.peer, false)?;
            authority.routes.extend(server_routes);
            tools.extend(projection.catalog.tools);
        }
        Ok(McpSessionBinding {
            catalog: DynamicToolCatalog::resolve(tools)
                .map_err(|e| McpError::Conflict(e.to_string()))?,
            authority: Arc::new(authority),
            refused_tools: refused_tools.into_iter().collect(),
        })
    }
}
fn prepare_server_cancellable(
    session: &str,
    server: &schema::McpConnection,
    secrets: SecretAccess,
    runtime: &tokio::runtime::Runtime,
    cancellation: McpCancellationToken,
) -> Result<PreparedServer, McpError> {
    if cancellation.is_cancelled() {
        return Err(McpError::Cancelled);
    }
    server
        .validate()
        .map_err(|e| McpError::Protocol(e.into()))?;
    let _guard = runtime.enter();
    let mut floors = Vec::new();
    let mut client: Box<dyn McpPeer> = match &server.transport {
        schema::McpConnectionTransport::Http { url, credential_id } => {
            floors = credential_id
                .as_ref()
                .map(|id| {
                    resolve_http_secret(id, secrets.store.as_ref(), url).map(
                        |(mut material, generation)| {
                            material.zeroize();
                            (id.clone(), generation)
                        },
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            let expected = floors.clone();
            let authorization_store = secrets.store.clone();
            let destination = url.clone();
            let transport = HttpTransport::new_with_request_authorization_provider(
                url.parse()
                    .map_err(|_| McpError::Protocol("invalid MCP URL".into()))?,
                &BTreeMap::new(),
                move || {
                    let mut bearer = None;
                    for (id, generation) in &expected {
                        let (mut token, actual) =
                            resolve_http_secret(id, authorization_store.as_ref(), &destination)?;
                        if actual != *generation {
                            token.zeroize();
                            return Err(McpError::Conflict(
                                "MCP credential generation changed".into(),
                            ));
                        }
                        bearer = Some(token);
                    }
                    Ok(HttpRequestAuthorization {
                        headers: BTreeMap::new(),
                        identity: authorization_identity(&expected),
                        bearer,
                    })
                },
                true,
            )?;
            let mut client = McpClient::new(server.name.clone(), transport);
            runtime
                .block_on(client.connect_cancellable(ProtocolMode::Auto, cancellation.clone()))?;
            Box::new(client)
        }
        schema::McpConnectionTransport::Stdio { channel } => {
            let connector = secrets
                .channels
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone();
            let (input, output) = runtime.block_on(async {
                tokio::select! {
                    biased;
                    ()=cancellation.cancelled() => Err(McpError::Cancelled),
                    result=tokio::time::timeout(std::time::Duration::from_secs(15),connector.open(session,channel)) => result.map_err(|_| McpError::Timeout("host stdio connection".into()))?.map_err(|_| McpError::Transport("host stdio channel unavailable".into())),
                }
            })?;
            let transport = PipeTransport::new(input, output)?;
            let mut client = McpClient::new(server.name.clone(), transport);
            runtime
                .block_on(client.connect_cancellable(ProtocolMode::Auto, cancellation.clone()))?;
            Box::new(client)
        }
    };
    let tools = match runtime.block_on(async { tokio::select! { biased; () = cancellation.cancelled() => Err(McpError::Cancelled), result = client.list_tools() => result } }) {
        Ok(tools) => tools,
        Err(error) => { let _ = runtime.block_on(client.close()); return Err(error); }
    };
    let config_digest = format!(
        "{:x}",
        Sha256::digest(
            serde_json_canonicalizer::to_vec(server)
                .map_err(|e| McpError::Protocol(e.to_string()))?
        )
    );
    Ok(PreparedServer {
        key: McpPoolKey {
            session_id: session.into(),
            connection: server.name.clone(),
            config_digest,
            authorization_identity: authorization_identity(&floors),
        },
        peer: client,
        tools,
        credential_floors: floors,
    })
}

fn with_preparation_runtime<T, F>(operation: F) -> Result<T, McpError>
where
    T: Send,
    F: FnOnce(&tokio::runtime::Runtime) -> Result<T, McpError> + Send,
{
    thread::scope(|scope| {
        scope
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|error| McpError::Transport(error.to_string()))?;
                operation(&runtime)
            })
            .join()
            .map_err(|_| McpError::Transport("MCP preparation runtime failed".to_owned()))?
    })
}

fn json_to_ijson(value: &Value) -> Result<IJsonValue, String> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn resolve_secret(credential: &str, secrets: &dyn SecretStore) -> Result<(String, u64), McpError> {
    match secrets
        .resolve(credential)
        .map_err(|_| McpError::Transport("MCP credential store unavailable".to_owned()))?
    {
        SecretResolution::Active(record @ SecretRecord::Active { .. }) => match &record {
            SecretRecord::Active {
                generation,
                material,
            } => Ok((material.clone(), *generation)),
            SecretRecord::Revoked { .. } => unreachable!(),
        },
        SecretResolution::Revoked { .. }
        | SecretResolution::NotFound
        | SecretResolution::Active(SecretRecord::Revoked { .. }) => Err(McpError::Transport(
            format!("MCP credential {credential} is unavailable"),
        )),
    }
}

fn authorization_identity(parts: &[(String, u64)]) -> String {
    if parts.is_empty() {
        return "anonymous".to_owned();
    }
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-mcp-authorization-v1\0");
    for (credential, generation) in parts {
        hasher.update(credential.as_bytes());
        hasher.update([0]);
        hasher.update(generation.to_be_bytes());
        hasher.update([0]);
    }
    format!("sha256-{:x}", hasher.finalize())
}

fn generation_lock(key: &McpPoolKey) -> Arc<Mutex<()>> {
    static LOCKS: std::sync::OnceLock<Mutex<BTreeMap<McpPoolKey, std::sync::Weak<Mutex<()>>>>> =
        std::sync::OnceLock::new();
    let mut locks = LOCKS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    locks.retain(|_, lock| lock.strong_count() > 0);
    if let Some(lock) = locks.get(key).and_then(std::sync::Weak::upgrade) {
        return lock;
    }
    let lock = Arc::new(Mutex::new(()));
    locks.insert(key.clone(), Arc::downgrade(&lock));
    lock
}
#[derive(Clone)]
pub struct McpDynamicSupervisorAuthority {
    _broker_owner: Arc<McpBroker>,
    broker: McpBrokerHandle,
    routes: BTreeMap<String, McpRoute>,
    secrets: SecretAccess,
    cancellation: Arc<Mutex<McpCancellationToken>>,
}

#[derive(Clone)]
pub struct McpRoute {
    pub key: McpPoolKey,
    pub remote_name: String,
    pub credential_floors: Vec<(String, u64)>,
    pub server: schema::McpConnection,
    pub contract: McpRouteContract,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpRouteContract {
    pub protocol_version: String,
    pub server_identity: McpImplementation,
    pub catalog_generation: u64,
    pub effect_tool: McpTool,
    pub reconcile_tool: Option<McpTool>,
    pub digest: String,
}

impl McpRouteContract {
    fn capture(
        peer: &dyn McpPeer,
        effect_tool: McpTool,
        reconcile_tool: Option<McpTool>,
    ) -> Result<Self, McpError> {
        let protocol_version = peer.protocol_version().to_owned();
        let server_identity = peer.server_identity().cloned().ok_or_else(|| {
            McpError::Protocol("MCP handshake omitted server identity".to_owned())
        })?;
        let catalog_generation = peer.catalog_generation();
        let digest = route_contract_digest(
            &protocol_version,
            &server_identity,
            catalog_generation,
            &effect_tool,
            reconcile_tool.as_ref(),
        )?;
        Ok(Self {
            protocol_version,
            server_identity,
            catalog_generation,
            effect_tool,
            reconcile_tool,
            digest,
        })
    }
}

fn route_contract_digest(
    protocol_version: &str,
    server_identity: &McpImplementation,
    catalog_generation: u64,
    effect_tool: &McpTool,
    reconcile_tool: Option<&McpTool>,
) -> Result<String, McpError> {
    let preimage = json!({
        "domain":"io.tekes/mcp-route-contract/v1",
        "protocolVersion":protocol_version,
        "serverIdentity":server_identity,
        "catalogGeneration":catalog_generation,
        "effectTool":effect_tool,
        "reconcileTool":reconcile_tool,
    });
    Ok(format!(
        "sha256-{:x}",
        Sha256::digest(
            serde_json_canonicalizer::to_vec(&preimage)
                .map_err(|error| McpError::Protocol(error.to_string()))?
        )
    ))
}

#[derive(Debug)]
enum RoutePeerError {
    Unavailable(String),
    ContractChanged(String),
}

fn route_preparation_error(error: McpError) -> RoutePeerError {
    match error {
        McpError::Transport(message) | McpError::Timeout(message) => {
            RoutePeerError::Unavailable(message)
        }
        other => RoutePeerError::ContractChanged(other.to_string()),
    }
}

fn validate_reconnected_contract(
    expected: &McpRouteContract,
    reconnected: &McpRouteContract,
) -> Result<(), RoutePeerError> {
    if reconnected.protocol_version != expected.protocol_version {
        return Err(RoutePeerError::ContractChanged(
            "negotiated protocol version changed".to_owned(),
        ));
    }
    if reconnected.server_identity != expected.server_identity {
        return Err(RoutePeerError::ContractChanged(
            "negotiated server identity changed".to_owned(),
        ));
    }
    if reconnected.catalog_generation != expected.catalog_generation {
        return Err(RoutePeerError::ContractChanged(
            "catalog generation changed during reconnect".to_owned(),
        ));
    }
    if reconnected.effect_tool != expected.effect_tool {
        return Err(RoutePeerError::ContractChanged(
            "effect tool canonical contract changed".to_owned(),
        ));
    }
    if reconnected.reconcile_tool != expected.reconcile_tool {
        return Err(RoutePeerError::ContractChanged(
            "reconciliation tool canonical contract changed".to_owned(),
        ));
    }
    if reconnected.digest != expected.digest {
        return Err(RoutePeerError::ContractChanged(
            "MCP route contract digest changed".to_owned(),
        ));
    }
    Ok(())
}

impl McpDynamicSupervisorAuthority {
    fn new(
        owner: Arc<McpBroker>,
        routes: BTreeMap<String, McpRoute>,
        secrets: SecretAccess,
    ) -> Self {
        Self {
            broker: owner.handle(),
            _broker_owner: owner,
            routes,
            secrets,
            cancellation: Arc::new(Mutex::new(McpCancellationToken::default())),
        }
    }
}

impl Drop for McpDynamicSupervisorAuthority {
    fn drop(&mut self) {
        let keys = self
            .routes
            .values()
            .map(|route| route.key.clone())
            .collect::<std::collections::BTreeSet<_>>();
        for key in keys {
            let _ = self.broker.release(key);
        }
    }
}

impl McpDynamicSupervisorAuthority {
    /// Peer generation, catalog contract and credential floors must hold before
    /// any remote operation on a route.
    fn prepare_route(
        &self,
        tool: &DynamicTool,
    ) -> Result<(&McpRoute, McpCancellationToken), SupervisorOperationError> {
        let route = self.routes.get(&tool.name).ok_or_else(|| {
            SupervisorOperationError::Unsupported(format!("MCP route {} is not bound", tool.name))
        })?;
        let cancellation = self.current_cancellation();
        if let Err(error) = self.ensure_route_peer(route, tool, cancellation.clone()) {
            let _ = self.broker.remove(route.key.clone());
            return Err(match error {
                RoutePeerError::Unavailable(reason) => SupervisorOperationError::Unavailable(
                    format!("MCP authority unavailable; peer generation closed: {reason}"),
                ),
                RoutePeerError::ContractChanged(reason) if tool.external_effect.is_some() => {
                    SupervisorOperationError::EffectConflicted(format!(
                        "MCP authority changed; peer generation closed: {reason}"
                    ))
                }
                RoutePeerError::ContractChanged(reason) => SupervisorOperationError::Conflict(
                    format!("MCP authority changed; peer generation closed: {reason}"),
                ),
            });
        }
        for (credential, expected_generation) in &route.credential_floors {
            let current = self.secrets.store.resolve(credential).map_err(|_| {
                SupervisorOperationError::Unavailable("MCP credential store unavailable".to_owned())
            })?;
            let valid = matches!(
                current,
                SecretResolution::Active(ref record)
                    if record.generation() == *expected_generation
            );
            if !valid {
                let _ = self.broker.remove(route.key.clone());
                return Err(SupervisorOperationError::Unavailable(format!(
                    "MCP credential {credential} changed; peer generation closed"
                )));
            }
        }
        Ok((route, cancellation))
    }

    fn call_route(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
    ) -> Result<Value, SupervisorOperationError> {
        let (route, cancellation) = self.prepare_route(tool)?;
        // A tool whose server declares execution.taskSupport = required only
        // runs as a task; the augmented call binds the remote continuation.
        let result = if route.contract.effect_tool.requires_task() {
            self.broker.call_tool_augmented(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                tool.external_effect.as_ref().map(|_| McpToolCallContext {
                    idempotency_key: request.request_id.clone(),
                }),
                REQUIRED_TASK_TTL_MS,
                cancellation,
            )
        } else if tool.external_effect.is_some() {
            self.broker.call_tool_with_context(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                McpToolCallContext {
                    idempotency_key: request.request_id.clone(),
                },
                cancellation,
            )
        } else {
            self.broker.call_tool(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                cancellation,
            )
        };
        let result = result.map_err(map_mcp_error)?;
        serde_json::to_value(&result)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))
    }
}

/// Time-to-live requested for a task-augmented call whose tool requires tasks.
const REQUIRED_TASK_TTL_MS: u64 = 600_000;

/// Domain-separated host continuation identity for one bound remote task.
fn continuation_identity(request_id: &str, task_id: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tekes-mcp-continuation-v1\0");
    digest.update(request_id.as_bytes());
    digest.update([0]);
    digest.update(task_id.as_bytes());
    format!("{:x}", digest.finalize())
}

fn map_journal_error(error: ContinuationJournalError) -> SupervisorOperationError {
    match error {
        ContinuationJournalError::Conflict(message) => SupervisorOperationError::Conflict(message),
        other => SupervisorOperationError::Unavailable(other.to_string()),
    }
}

fn terminal_tool_value(raw: Value) -> Result<IJsonValue, SupervisorOperationError> {
    match raw.get("isError") {
        Some(Value::Bool(true)) => Err(SupervisorOperationError::ToolFailed(raw.to_string())),
        Some(Value::Bool(false)) | None => {
            json_to_ijson(&raw).map_err(SupervisorOperationError::Protocol)
        }
        Some(_) => Err(SupervisorOperationError::Protocol(
            "MCP tool result isError must be boolean".into(),
        )),
    }
}

impl DynamicSupervisorAuthority for McpDynamicSupervisorAuthority {
    fn supports(&self, tool: &DynamicTool) -> bool {
        tool.source.kind == DynamicToolSourceKind::Mcp && self.routes.contains_key(&tool.name)
    }

    fn execute(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let raw = self.call_route(tool, request)?;
        terminal_tool_value(raw)
    }

    /// `tools/call` whose result is a task binds a durable continuation instead
    /// of returning a value: the worker will drive it through `continue_task`.
    /// The peer already maps both dialects (SEP-1686 `{"task":...}` and the
    /// tasks extension's flat `resultType:"task"`) onto the `task` member read
    /// here. Any other result keeps the synchronous mapping.
    fn execute_outcome(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
        continuation_root: &Path,
    ) -> Result<DynamicExecutionOutcome, SupervisorOperationError> {
        let raw = self.call_route(tool, request)?;
        let Some(task) = raw.get("task").filter(|task| task.is_object()) else {
            return terminal_tool_value(raw).map(DynamicExecutionOutcome::Value);
        };
        let task_id = task["taskId"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| {
                SupervisorOperationError::Protocol("MCP task result lacks taskId".to_owned())
            })?;
        let task_value = json_to_ijson(task).map_err(SupervisorOperationError::Protocol)?;
        let validated = mcp::McpTask::validate_result(&task_value, task_id)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
        match validated.status.as_str() {
            "working" | "input_required" => {}
            "completed" => {
                return terminal_tool_value(task["result"].clone())
                    .map(DynamicExecutionOutcome::Value);
            }
            other => {
                return Err(SupervisorOperationError::ToolFailed(format!(
                    "MCP task {task_id} was created {other}: {}",
                    task.get("error")
                        .map(ToString::to_string)
                        .unwrap_or_default()
                )));
            }
        }
        let route = self.routes.get(&tool.name).ok_or_else(|| {
            SupervisorOperationError::Unsupported(format!("MCP route {} is not bound", tool.name))
        })?;
        let continuation_id = continuation_identity(&request.request_id, task_id);
        let journal = ContinuationJournal::open(continuation_root).map_err(map_journal_error)?;
        journal
            .bind(&ContinuationBinding {
                original: request.clone(),
                continuation_id: continuation_id.clone(),
                authority: task_authority(&route.key).map_err(map_journal_error)?,
                initial_state: task_value.clone(),
            })
            .map_err(map_journal_error)?;
        Ok(DynamicExecutionOutcome::Pending {
            continuation_id,
            state: task_value,
        })
    }

    fn continue_task(
        &self,
        tool: &DynamicTool,
        request: &ToolContinuationRequest,
        continuation_root: &Path,
    ) -> Result<ToolContinuationResponse, SupervisorOperationError> {
        let (route, cancellation) = self.prepare_route(tool)?;
        let journal = ContinuationJournal::open(continuation_root).map_err(map_journal_error)?;
        resolve_task(&journal, &self.broker, &route.key, request, cancellation)
            .map_err(map_journal_error)
    }

    fn reconcile(&self, tool: &DynamicTool, request: &ToolControl) -> ExternalEffectResolution {
        let Some(binding) = &tool.external_effect else {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP tool {} lost its external-effect contract", tool.name),
            };
        };
        let Some(route) = self.routes.get(&tool.name) else {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP route {} is not bound", tool.name),
            };
        };
        let cancellation = self.current_cancellation();
        if let Err(error) = self.ensure_route_peer(route, tool, cancellation.clone()) {
            return match error {
                RoutePeerError::Unavailable(reason) => ExternalEffectResolution::Unknown {
                    message: format!("MCP reconciliation authority is unavailable: {reason}"),
                },
                RoutePeerError::ContractChanged(reason) => ExternalEffectResolution::Conflicted {
                    message: format!("MCP reconciliation authority changed: {reason}"),
                },
            };
        }
        let arguments = match IJsonValue::parse(
            &serde_json::to_vec(&json!({"idempotencyKey":request.request_id}))
                .expect("idempotency reconciliation request is JSON"),
        ) {
            Ok(arguments) => arguments,
            Err(error) => {
                return ExternalEffectResolution::Conflicted {
                    message: format!("MCP reconciliation request is invalid: {error}"),
                };
            }
        };
        match self.broker.call_tool(
            route.key.clone(),
            binding.reconcile_tool.clone(),
            arguments,
            cancellation,
        ) {
            Ok(value) => parse_external_effect_resolution(request, value),
            Err(error) => ExternalEffectResolution::Unknown {
                message: format!("MCP reconciliation query failed: {error}"),
            },
        }
    }

    fn cancel_inflight(&self) {
        let cancelled = {
            let mut current = self
                .cancellation
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::take(&mut *current)
        };
        cancelled.cancel();
    }
}

impl McpDynamicSupervisorAuthority {
    fn current_cancellation(&self) -> McpCancellationToken {
        self.cancellation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn ensure_route_peer(
        &self,
        route: &McpRoute,
        expected_tool: &DynamicTool,
        cancellation: McpCancellationToken,
    ) -> Result<(), RoutePeerError> {
        let creation = generation_lock(&route.key);
        let _creation = creation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if cancellation.is_cancelled() {
            return Err(RoutePeerError::Unavailable(
                "MCP operation was cancelled".to_owned(),
            ));
        }
        self.validate_route_generation(route)
            .map_err(RoutePeerError::ContractChanged)?;
        if let Some(generation) = self
            .broker
            .catalog_generation(route.key.clone())
            .map_err(|error| RoutePeerError::Unavailable(error.to_string()))?
        {
            if generation == route.contract.catalog_generation {
                return Ok(());
            }
            let _ = self.broker.remove(route.key.clone());
            return Err(RoutePeerError::ContractChanged(
                "MCP catalog generation changed".to_owned(),
            ));
        }
        let prepared = with_preparation_runtime(|runtime| {
            prepare_server_cancellable(
                &route.key.session_id,
                &route.server,
                self.secrets.clone(),
                runtime,
                cancellation,
            )
        })
        .map_err(route_preparation_error)?;
        if prepared.key != route.key || prepared.credential_floors != route.credential_floors {
            return Err(RoutePeerError::ContractChanged(
                "reconnected MCP pool or credential identity changed".to_owned(),
            ));
        }
        let effect_tool = prepared
            .tools
            .iter()
            .find(|tool| tool.name == route.remote_name)
            .ok_or_else(|| {
                RoutePeerError::ContractChanged(
                    "reconnected MCP catalog lost the effect tool".to_owned(),
                )
            })?
            .clone();
        let reconcile_tool = route
            .contract
            .reconcile_tool
            .as_ref()
            .map(|expected| {
                prepared
                    .tools
                    .iter()
                    .find(|tool| tool.name == expected.name)
                    .cloned()
                    .ok_or_else(|| {
                        RoutePeerError::ContractChanged(
                            "reconnected MCP catalog lost the reconciliation tool".to_owned(),
                        )
                    })
            })
            .transpose()?;
        let reconnected_contract =
            McpRouteContract::capture(prepared.peer.as_ref(), effect_tool, reconcile_tool)
                .map_err(route_preparation_error)?;
        validate_reconnected_contract(&route.contract, &reconnected_contract)?;
        // The reconnected catalog is judged per tool like the first one: a
        // sibling tool that stays refused must not fail the reconnect of this
        // route, while this route's own tool must still project unchanged.
        let projected = project_catalog_partial(&route.server.name, &prepared.tools, true)
            .map_err(|error| RoutePeerError::ContractChanged(error.to_string()))?
            .catalog;
        let reconnected_tool = projected
            .tools
            .iter()
            .find(|tool| tool.name == expected_tool.name)
            .ok_or_else(|| {
                RoutePeerError::ContractChanged(
                    "reconnected MCP catalog lost the effectful tool".to_owned(),
                )
            })?;
        if reconnected_tool != expected_tool {
            return Err(RoutePeerError::ContractChanged(
                "reconnected MCP catalog changed the projected effect contract".to_owned(),
            ));
        }
        self.broker
            .register(prepared.key, prepared.peer, false)
            .map_err(|error| RoutePeerError::Unavailable(error.to_string()))
    }

    fn validate_route_generation(&self, route: &McpRoute) -> Result<(), String> {
        for (credential_id, expected_generation) in &route.credential_floors {
            let current = self
                .secrets
                .store
                .resolve(credential_id)
                .map_err(|_| "credential store is unavailable".to_owned())?;
            if !matches!(
                current,
                SecretResolution::Active(ref record)
                    if record.generation() == *expected_generation
            ) {
                return Err(format!(
                    "credential {credential_id} generation changed or was revoked"
                ));
            }
        }
        Ok(())
    }
}

fn map_mcp_error(error: McpError) -> SupervisorOperationError {
    match error {
        McpError::Timeout(message) => SupervisorOperationError::Timeout(message),
        McpError::Unsupported(message) => SupervisorOperationError::Unsupported(message),
        McpError::NoMutualProtocol(versions) => {
            SupervisorOperationError::Unsupported(format!("no mutual MCP protocol: {versions:?}"))
        }
        McpError::Protocol(message) | McpError::Conflict(message) => {
            SupervisorOperationError::Protocol(message)
        }
        McpError::Remote { code, message, .. } => {
            SupervisorOperationError::Unavailable(format!("MCP remote error {code}: {message}"))
        }
        McpError::UnknownEffect => {
            SupervisorOperationError::AmbiguousEffect("MCP tool effect is unknown".to_owned())
        }
        McpError::CatalogLimit => {
            SupervisorOperationError::Limit("MCP catalog bound exceeded".to_owned())
        }
        McpError::Cancelled => {
            SupervisorOperationError::Unavailable("MCP call cancelled".to_owned())
        }
        McpError::Transport(message) => SupervisorOperationError::Unavailable(message),
    }
}

fn parse_external_effect_resolution(
    request: &ToolControl,
    value: IJsonValue,
) -> ExternalEffectResolution {
    let result: Value = match value
        .canonical_bytes()
        .map_err(|error| error.to_string())
        .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|error| error.to_string()))
    {
        Ok(raw) => raw,
        Err(error) => {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP reconciliation result is invalid: {error}"),
            };
        }
    };
    // A normal MCP tools/call returns a CallToolResult envelope. Reconcile
    // functions may expose their JSON via structuredContent or the first text
    // content block; older direct peers return the object itself.
    let raw = if result.get("status").is_some() {
        result
    } else if result.get("isError") == Some(&Value::Bool(true)) {
        return ExternalEffectResolution::Unknown {
            message: "MCP reconciliation tool returned an error".to_owned(),
        };
    } else if let Some(structured) = result.get("structuredContent") {
        structured.clone()
    } else if let Some(text) = result
        .get("content")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(|item| item.get("text"))
        .and_then(Value::as_str)
    {
        match serde_json::from_str::<Value>(text) {
            Ok(value) => value,
            Err(_) => {
                return ExternalEffectResolution::Conflicted {
                    message: "MCP reconciliation text is not JSON".to_owned(),
                };
            }
        }
    } else {
        result
    };
    let Some(status) = raw.get("status").and_then(Value::as_str) else {
        return ExternalEffectResolution::Conflicted {
            message: "MCP reconciliation result has no closed status".to_owned(),
        };
    };
    match status {
        "confirmed" => {
            if raw.as_object().is_none_or(|object| object.len() != 2) {
                return ExternalEffectResolution::Conflicted {
                    message: "confirmed reconciliation result has unknown fields".to_owned(),
                };
            }
            let Some(value) = raw.get("value") else {
                return ExternalEffectResolution::Conflicted {
                    message: "confirmed reconciliation result has no value".to_owned(),
                };
            };
            match json_to_ijson(value) {
                Ok(value) => {
                    ExternalEffectResolution::Confirmed(worker_control::ToolControlResult::success(
                        request.request_id.clone(),
                        request.call_id.clone(),
                        value,
                    ))
                }
                Err(error) => ExternalEffectResolution::Conflicted {
                    message: format!("confirmed reconciliation value is invalid: {error}"),
                },
            }
        }
        "not_found" if raw.as_object().is_some_and(|object| object.len() == 1) => {
            ExternalEffectResolution::NotFound
        }
        "unknown" | "conflicted" => {
            let reason = raw
                .get("reason")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty());
            if raw.as_object().is_none_or(|object| object.len() != 2) || reason.is_none() {
                return ExternalEffectResolution::Conflicted {
                    message: format!(
                        "{status} reconciliation result must contain only status and nonempty reason"
                    ),
                };
            }
            let reason = reason.expect("checked nonempty reason").to_owned();
            if status == "unknown" {
                ExternalEffectResolution::Unknown { message: reason }
            } else {
                ExternalEffectResolution::Conflicted { message: reason }
            }
        }
        other => ExternalEffectResolution::Conflicted {
            message: format!("MCP reconciliation returned unsupported status {other}"),
        },
    }
}

/// A host-issued credential is scoped by value over the private bootstrap
/// channel. Provider credentials and arbitrary stored strings cannot become
/// MCP Authorization headers, even if a client guesses their reference ids.
fn resolve_http_secret(
    id: &str,
    store: &dyn SecretStore,
    destination: &str,
) -> Result<(String, u64), McpError> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Grant {
        kind: String,
        url: String,
        token: String,
    }
    let (material, generation) = resolve_secret(id, store)?;
    let material = zeroize::Zeroizing::new(material);
    let mut grant: Grant = serde_json::from_str(&material)
        .map_err(|_| McpError::Conflict("MCP credential is not a host-scoped grant".into()))?;
    let valid = grant.kind == "mcp"
        && url::Url::parse(&grant.url).ok() == url::Url::parse(destination).ok()
        && !grant.token.is_empty()
        && !grant.token.chars().any(char::is_control);
    if !valid {
        grant.token.zeroize();
        return Err(McpError::Conflict(
            "MCP credential destination differs".into(),
        ));
    }
    Ok((std::mem::take(&mut grant.token), generation))
}
