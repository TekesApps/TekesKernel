//! Generation admission runs while holding the broker peer mutex. The returned
//! folder lease is retained through the effect, while revocation closes new work.
use crate::mcp_runtime::{SecretAccess, server_pool_key};
use mcp::*;
use schema::IJsonValue;
use std::sync::Arc;

type Guard = Option<Box<dyn Send>>;
pub(crate) fn admit(
    server: &McpServerConfig,
    secrets: &SecretAccess,
    key: &McpPoolKey,
) -> Result<Guard, McpError> {
    let mut plugin = None;
    if let Some(registry) = secrets.registry.as_ref()
        && server.reference.scope != McpScope::Plugin
    {
        let current = registry
            .get(&server.reference)
            .map_err(|error| McpError::Conflict(error.to_string()))?
            .ok_or_else(|| McpError::Conflict("MCP server removed".into()))?;
        if current != *server || !current.enabled {
            return Err(McpError::Conflict("MCP configuration changed".into()));
        }
        if server.reference.scope == McpScope::User {
            crate::mcp_scope::validate_user(server, registry)?;
        }
        if server.reference.scope == McpScope::Project {
            crate::mcp_scope::validate_project(server, registry)?;
        }
        for (field, id) in crate::mcp_runtime::credential_fields(server) {
            if registry
                .credential_reference_state(&server.reference, &field, &id)
                .map_err(|error| McpError::Conflict(error.to_string()))?
                != Some(McpCredentialReferenceState::Bound)
            {
                return Err(McpError::Conflict("MCP credential not bound".into()));
            }
        }
    }
    if let Some(reference) = &server.plugin_component
        && let Some(resolver) = &secrets.plugin_resolver
    {
        plugin = resolver.resolve(reference).map_err(McpError::Conflict)?;
        if plugin.is_none() {
            return Err(McpError::Conflict("MCP plugin unavailable".into()));
        }
    }
    let (mut current, _) = server_pool_key(server, plugin.as_ref(), secrets)?;
    // Library-only tests may supply a verified executable directly. Production
    // always resolves the current signed plugin generation above.
    if secrets.plugin_resolver.is_none() {
        current.plugin_generation = key.plugin_generation.clone();
    }
    if current != *key {
        return Err(McpError::Conflict(
            "MCP owner or authority generation changed".into(),
        ));
    }
    let Some(registry) = &secrets.registry else {
        return Ok(None);
    };
    if matches!(server.transport, McpTransportConfig::Stdio { .. })
        && let Some(folder) = crate::mcp_scope::folder(server, registry)?
    {
        let authority =
            profile::FolderAuthorityStore::new(crate::mcp_scope::storage_root(registry)?);
        let guard = authority
            .admit(&folder)
            .map_err(|error| McpError::Conflict(error.to_string()))?;
        let (current, _) = server_pool_key(server, plugin.as_ref(), secrets)?;
        if current != *key {
            return Err(McpError::Conflict(
                "MCP trust changed during admission".into(),
            ));
        }
        return Ok(Some(Box::new(guard) as Box<dyn Send>));
    }
    Ok(None)
}

type Check = dyn Fn() -> Result<Guard, McpError> + Send + Sync;
pub struct AdmissionPeer {
    peer: Box<dyn McpPeer>,
    check: Arc<Check>,
}
impl AdmissionPeer {
    pub fn new(
        peer: Box<dyn McpPeer>,
        check: impl Fn() -> Result<Guard, McpError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            peer,
            check: Arc::new(check),
        }
    }
}
impl McpPeer for AdmissionPeer {
    fn cached_tools(&self) -> Option<Vec<McpTool>> {
        self.peer.cached_tools()
    }
    fn admit(&self) -> Result<Guard, McpError> {
        (self.check)()
    }
    fn server_name(&self) -> &str {
        self.peer.server_name()
    }
    fn protocol_version(&self) -> &str {
        self.peer.protocol_version()
    }
    fn server_identity(&self) -> Option<&McpImplementation> {
        self.peer.server_identity()
    }
    fn capabilities(&self) -> &McpCapabilities {
        self.peer.capabilities()
    }
    fn catalog_generation(&self) -> u64 {
        self.peer.catalog_generation()
    }
    fn start_catalog_subscription(&mut self) -> Option<McpPeerFuture<'static, ()>> {
        self.peer.start_catalog_subscription()
    }
    fn start_scoped_tool(
        &self,
        name: &str,
        arguments: &IJsonValue,
        context: Option<&McpToolCallContext>,
        cancellation: McpCancellationToken,
    ) -> Option<McpPeerFuture<'static, IJsonValue>> {
        self.peer
            .start_scoped_tool(name, arguments, context, cancellation)
    }
    fn list_tools(&mut self) -> McpPeerFuture<'_, Vec<McpTool>> {
        self.peer.list_tools()
    }
    fn list_prompts(&mut self) -> McpPeerFuture<'_, Vec<McpPrompt>> {
        self.peer.list_prompts()
    }
    fn list_resources(&mut self) -> McpPeerFuture<'_, Vec<McpResource>> {
        self.peer.list_resources()
    }
    fn call_tool(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.peer.call_tool(name, arguments, cancellation)
    }
    fn call_tool_with_context(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        context: McpToolCallContext,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.peer
            .call_tool_with_context(name, arguments, context, cancellation)
    }
    fn call_tool_augmented(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        context: Option<McpToolCallContext>,
        ttl: u64,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.peer
            .call_tool_augmented(name, arguments, context, ttl, cancellation)
    }
    fn get_prompt(&mut self, name: &str, arguments: IJsonValue) -> McpPeerFuture<'_, IJsonValue> {
        self.peer.get_prompt(name, arguments)
    }
    fn read_resource(&mut self, uri: &str) -> McpPeerFuture<'_, IJsonValue> {
        self.peer.read_resource(uri)
    }
    fn task_operation(
        &mut self,
        method: &str,
        params: IJsonValue,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.peer.task_operation(method, params)
    }
    fn task_operation_cancellable(
        &mut self,
        method: &str,
        params: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.peer
            .task_operation_cancellable(method, params, cancellation)
    }
    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.peer.close()
    }
}

/// Read-only view of a retained broker generation. The caller's pending lease
/// owns the retention; closing this view never closes another scope's process.
pub struct ResourcePeer {
    metadata: Box<dyn McpPeer>,
    broker: McpBrokerHandle,
    key: McpPoolKey,
}
impl ResourcePeer {
    pub fn new(metadata: Box<dyn McpPeer>, broker: McpBrokerHandle, key: McpPoolKey) -> Self {
        Self {
            metadata,
            broker,
            key,
        }
    }
}
impl McpPeer for ResourcePeer {
    fn server_name(&self) -> &str {
        self.metadata.server_name()
    }
    fn protocol_version(&self) -> &str {
        self.metadata.protocol_version()
    }
    fn server_identity(&self) -> Option<&McpImplementation> {
        self.metadata.server_identity()
    }
    fn capabilities(&self) -> &McpCapabilities {
        self.metadata.capabilities()
    }
    fn catalog_generation(&self) -> u64 {
        self.broker
            .catalog_generation(self.key.clone())
            .ok()
            .flatten()
            .unwrap_or_else(|| self.metadata.catalog_generation())
    }
    fn list_resources(&mut self) -> McpPeerFuture<'_, Vec<McpResource>> {
        Box::pin(async { self.broker.list_resources(self.key.clone()) })
    }
    fn read_resource(&mut self, uri: &str) -> McpPeerFuture<'_, IJsonValue> {
        let uri = uri.to_owned();
        Box::pin(async move { self.broker.read_resource(self.key.clone(), uri) })
    }
    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
    fn list_tools(&mut self) -> McpPeerFuture<'_, Vec<McpTool>> {
        self.metadata.list_tools()
    }
    fn list_prompts(&mut self) -> McpPeerFuture<'_, Vec<McpPrompt>> {
        self.metadata.list_prompts()
    }
    fn call_tool(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.metadata.call_tool(name, arguments, cancellation)
    }
    fn get_prompt(&mut self, name: &str, arguments: IJsonValue) -> McpPeerFuture<'_, IJsonValue> {
        self.metadata.get_prompt(name, arguments)
    }
    fn task_operation(
        &mut self,
        method: &str,
        params: IJsonValue,
    ) -> McpPeerFuture<'_, IJsonValue> {
        self.metadata.task_operation(method, params)
    }
}
