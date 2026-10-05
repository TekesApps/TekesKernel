use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use schema::IJsonValue;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::{
    JsonRpcRequest, MAX_CATALOG_ITEMS, MAX_CATALOG_PAGES, MAX_CONTINUATION_ROUNDS,
    McpCancellationToken, McpCapabilities, McpError, McpImplementation, McpLossState, McpPrompt,
    McpResource, McpTool, McpToolCallContext, McpToolContinuation, McpTransport, ProtocolMode,
    RecoveryAction, recovery_action,
};

const CANCELLATION_IO_TIMEOUT: Duration = Duration::from_millis(250);

pub type McpPeerFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, McpError>> + Send + 'a>>;

pub trait McpPeer: Send {
    fn server_name(&self) -> &str;
    /// Opt-in independent request ownership. None retains serialized dispatch.
    /// Implementations must bind returned futures to peer close/reconnect and
    /// isolate request errors; the broker does not evict on scoped call errors.
    fn start_scoped_tool(
        &self,
        _name: &str,
        _arguments: &IJsonValue,
        _context: Option<&McpToolCallContext>,
        _cancellation: McpCancellationToken,
    ) -> Option<McpPeerFuture<'static, IJsonValue>> {
        None
    }

    fn protocol_version(&self) -> &str {
        ""
    }
    fn server_identity(&self) -> Option<&McpImplementation> {
        None
    }
    fn capabilities(&self) -> &McpCapabilities;
    fn catalog_generation(&self) -> u64 {
        0
    }
    fn start_catalog_subscription(&mut self) -> Option<McpPeerFuture<'static, ()>> {
        None
    }
    fn list_tools(&mut self) -> McpPeerFuture<'_, Vec<McpTool>>;
    fn list_prompts(&mut self) -> McpPeerFuture<'_, Vec<McpPrompt>>;
    fn list_resources(&mut self) -> McpPeerFuture<'_, Vec<McpResource>>;
    fn call_tool(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue>;
    fn call_tool_with_context(
        &mut self,
        _name: &str,
        _arguments: IJsonValue,
        _context: McpToolCallContext,
        _cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        Box::pin(async {
            Err(McpError::Unsupported(
                "idempotency-reconcile tool calls".to_owned(),
            ))
        })
    }
    /// Task-augmented `tools/call`: the request carries `task: {ttl}` so a
    /// tasks-capable server answers with a `CreateTaskResult` instead of a
    /// synchronous result. Peers without task support refuse.
    fn call_tool_augmented(
        &mut self,
        _name: &str,
        _arguments: IJsonValue,
        _context: Option<McpToolCallContext>,
        _task_ttl_ms: u64,
        _cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        Box::pin(async {
            Err(McpError::Unsupported(
                "task-augmented tool calls".to_owned(),
            ))
        })
    }
    fn get_prompt(&mut self, name: &str, arguments: IJsonValue) -> McpPeerFuture<'_, IJsonValue>;
    fn read_resource(&mut self, uri: &str) -> McpPeerFuture<'_, IJsonValue>;
    fn task_operation(&mut self, method: &str, params: IJsonValue)
    -> McpPeerFuture<'_, IJsonValue>;
    fn task_operation_cancellable(
        &mut self,
        method: &str,
        params: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let method = method.to_owned();
        Box::pin(async move {
            if cancellation.is_cancelled() {
                return Err(McpError::Cancelled);
            }
            let result = tokio::select! {
                result = self.task_operation(&method, params) => Some(result),
                () = cancellation.cancelled() => None,
            };
            match result {
                Some(result) => result,
                None => {
                    let _ = self.close().await;
                    Err(if method == "tasks/get" {
                        McpError::Cancelled
                    } else {
                        McpError::UnknownEffect
                    })
                }
            }
        })
    }
    fn close(&mut self) -> McpPeerFuture<'_, ()>;
}

pub struct McpClient<T> {
    server_name: String,
    subscription_started: bool,
    transport: T,
    transport_closed: bool,
    next_id: std::sync::atomic::AtomicU64,
    capabilities: McpCapabilities,
    server: Option<McpImplementation>,
    protocol_version: String,
    supported_versions: Vec<String>,
    modern: bool,
    mode: Option<ProtocolMode>,
    catalog_generation: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl<T: McpTransport> McpClient<T> {
    pub fn new(server_name: impl Into<String>, transport: T) -> Self {
        Self {
            server_name: server_name.into(),
            subscription_started: false,
            transport,
            transport_closed: false,
            next_id: std::sync::atomic::AtomicU64::new(1),
            capabilities: McpCapabilities::default(),
            server: None,
            protocol_version: String::new(),
            supported_versions: Vec::new(),
            modern: false,
            mode: None,
            catalog_generation: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub async fn connect(&mut self, mode: ProtocolMode) -> Result<(), McpError> {
        self.connect_cancellable(mode, McpCancellationToken::default())
            .await
    }

    /// Cooperatively cancel a handshake, including an in-flight reconnect,
    /// and finish transport cleanup before returning. Dropping this future is
    /// not a substitute for cancelling the token and awaiting completion.
    pub async fn connect_cancellable(
        &mut self,
        mode: ProtocolMode,
        cancellation: McpCancellationToken,
    ) -> Result<(), McpError> {
        let result = if cancellation.is_cancelled() {
            Err(McpError::Cancelled)
        } else {
            tokio::select! {
                biased;
                () = cancellation.cancelled() => Err(McpError::Cancelled),
                result = self.connect_handshake(mode) => result,
            }
        };
        if result.is_err() {
            let _ = self.close_transport().await;
        }
        result
    }

    async fn connect_handshake(&mut self, mode: ProtocolMode) -> Result<(), McpError> {
        self.mode = Some(mode);
        let mut result = self.connect_once(mode).await;
        if should_reconnect(&result, McpLossState::Handshake) {
            self.reset_generation();
            result = match self.reconnect_transport().await {
                Ok(()) => self.connect_once(mode).await,
                Err(error) => Err(error),
            };
        }
        result
    }

    fn close_transport(&mut self) -> McpPeerFuture<'_, ()> {
        if self.transport_closed {
            return Box::pin(async { Ok(()) });
        }
        self.transport_closed = true;
        self.transport.close()
    }

    async fn reconnect_transport(&mut self) -> Result<(), McpError> {
        // A reconnect attempt may acquire resources before returning an error.
        self.transport_closed = false;
        self.transport.reconnect().await
    }

    async fn connect_once(&mut self, mode: ProtocolMode) -> Result<(), McpError> {
        match mode {
            ProtocolMode::Modern => self.connect_modern().await,
            ProtocolMode::Legacy => self.connect_legacy().await,
            ProtocolMode::Auto if self.transport.supports_http_discovery() => {
                let result = self.connect_modern().await;
                if result.as_ref().err().is_some_and(is_legacy_evidence) {
                    self.reset_generation();
                    self.reconnect_transport().await?;
                    self.connect_legacy().await
                } else {
                    result
                }
            }
            // Unknown stdio methods may close the sole pipe; HTTP discovery
            // evidence does not authorize probing arbitrary stdio peers.
            ProtocolMode::Auto => self.connect_legacy().await,
        }
    }

    fn reset_generation(&mut self) {
        self.next_id.store(1, std::sync::atomic::Ordering::Relaxed);
        self.capabilities = McpCapabilities::default();
        self.server = None;
        self.protocol_version.clear();
        self.supported_versions.clear();
        self.modern = false;
        let _ = self.catalog_generation.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |v| Some(v.saturating_add(1)),
        );
    }

    async fn reconnect_generation(&mut self) -> Result<(), McpError> {
        let mode = self
            .mode
            .ok_or_else(|| McpError::Protocol("MCP peer was never connected".to_owned()))?;
        let expected_protocol = self.protocol_version.clone();
        let expected_server = self.server.clone().ok_or_else(|| {
            McpError::Protocol("MCP peer has no negotiated server identity".to_owned())
        })?;
        let result = async {
            self.reconnect_transport().await?;
            self.reset_generation();
            self.connect_once(mode).await
        }
        .await;
        if let Err(error) = result {
            let _ = self.close_transport().await;
            return Err(error);
        }
        if self.protocol_version != expected_protocol
            || self.server.as_ref() != Some(&expected_server)
        {
            let _ = self.close_transport().await;
            return Err(McpError::Conflict(
                "MCP negotiated authority changed during reconnect".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn server(&self) -> Option<&McpImplementation> {
        self.server.as_ref()
    }

    /// Versions advertised by modern discovery, or the negotiated legacy version.
    pub fn supported_versions(&self) -> &[String] {
        &self.supported_versions
    }

    pub fn protocol_version(&self) -> &str {
        &self.protocol_version
    }

    #[must_use]
    pub fn catalog_generation(&self) -> u64 {
        self.catalog_generation
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    async fn connect_legacy(&mut self) -> Result<(), McpError> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct InitializeResult {
            #[serde(default, rename = "instructions")]
            _instructions: Option<String>,
            #[serde(rename = "protocolVersion")]
            protocol_version: String,
            #[serde(default)]
            capabilities: McpCapabilities,
            #[serde(rename = "serverInfo")]
            server_info: McpImplementation,
        }
        let params = value(json!({
            "protocolVersion": crate::LEGACY_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": {"name":"TekesKernel","version":env!("CARGO_PKG_VERSION")}
        }))?;
        let result: InitializeResult = self
            .call_typed("initialize", Some(params), Duration::from_secs(15))
            .await?;
        if result.protocol_version != crate::LEGACY_PROTOCOL_VERSION {
            return Err(McpError::Unsupported(result.protocol_version));
        }
        self.supported_versions = vec![result.protocol_version.clone()];
        self.protocol_version = result.protocol_version;
        self.capabilities = result.capabilities;
        self.server = Some(result.server_info);
        self.transport
            .notify(&JsonRpcRequest::notification(
                "notifications/initialized",
                None,
            ))
            .await
    }

    async fn connect_modern(&mut self) -> Result<(), McpError> {
        // Discovery itself is a modern request and must carry its protocol
        // metadata; otherwise a dual-era server routes it as legacy JSON-RPC.
        self.modern = true;
        self.protocol_version = crate::MODERN_PROTOCOL_VERSION.to_owned();
        #[derive(Deserialize)]
        struct DiscoverResult {
            #[serde(rename = "resultType")]
            result_type: String,
            #[serde(rename = "supportedVersions")]
            supported_versions: Vec<String>,
            // Discovery cache scope is advisory. This client performs fresh
            // discovery and does not cache across credential/session scopes.
            #[serde(default, rename = "cacheScope")]
            _cache_scope: Option<schema::IJsonValue>,
            #[serde(default)]
            capabilities: McpCapabilities,
            #[serde(rename = "_meta")]
            metadata: DiscoverMetadata,
        }
        #[derive(Deserialize)]
        struct DiscoverMetadata {
            #[serde(rename = "io.modelcontextprotocol/serverInfo")]
            server_info: McpImplementation,
        }
        let result: DiscoverResult = self
            .call_typed("server/discover", None, Duration::from_secs(15))
            .await?;
        if result.result_type != "complete" {
            return Err(McpError::Protocol(
                "modern discovery did not return a complete result".to_owned(),
            ));
        }
        if !result
            .supported_versions
            .iter()
            .any(|version| version == crate::MODERN_PROTOCOL_VERSION)
        {
            return Err(McpError::NoMutualProtocol(result.supported_versions));
        }
        self.protocol_version = crate::MODERN_PROTOCOL_VERSION.to_owned();
        self.capabilities = result.capabilities;
        self.supported_versions = result.supported_versions;
        self.server = Some(result.metadata.server_info);
        self.modern = true;
        Ok(())
    }

    fn allocate_request_id(&self) -> Result<u64, McpError> {
        self.next_id
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |id| id.checked_add(1),
            )
            .map_err(|_| McpError::Protocol("request id exhausted".to_owned()))
    }

    async fn call_raw_once(
        &mut self,
        method: &str,
        params: Option<IJsonValue>,
        timeout: Duration,
    ) -> Result<IJsonValue, McpError> {
        let id = self.allocate_request_id()?;
        let params = self.protocol_params(params)?;
        let response = self
            .transport
            .request(&JsonRpcRequest::call(id, method, params), timeout)
            .await;
        self.observe_notifications();
        let response = match response {
            Ok(response) => response,
            Err(error @ McpError::Protocol(_)) => {
                let _ = self.close_transport().await;
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        let result = response.into_result();
        if matches!(result, Err(McpError::Protocol(_))) {
            let _ = self.close_transport().await;
        }
        result
    }

    async fn call_read_raw(
        &mut self,
        method: &str,
        params: Option<IJsonValue>,
        timeout: Duration,
    ) -> Result<IJsonValue, McpError> {
        let first = self.call_raw_once(method, params.clone(), timeout).await;
        if !should_reconnect(&first, McpLossState::ReadBeforeResponse) {
            return first;
        }
        self.reconnect_generation().await?;
        self.call_raw_once(method, params, timeout).await
    }

    async fn call_raw_cancellable(
        &mut self,
        method: &str,
        params: Option<IJsonValue>,
        timeout: Duration,
        cancellation: &McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        if cancellation.is_cancelled() {
            return Err(McpError::Cancelled);
        }
        let id = self.allocate_request_id()?;
        let params = self.protocol_params(params)?;
        let request = JsonRpcRequest::call(id, method, params);
        let result = {
            let response = self.transport.request(&request, timeout);
            tokio::pin!(response);
            tokio::select! {
                result = &mut response => Some(result),
                () = cancellation.cancelled() => None,
            }
        };
        if let Some(result) = result {
            self.observe_notifications();
            let response = match result {
                Ok(response) => response,
                Err(error @ McpError::Protocol(_)) => {
                    let _ = self.close_transport().await;
                    return Err(error);
                }
                Err(error) => return Err(error),
            };
            let result = response.into_result();
            if matches!(result, Err(McpError::Protocol(_))) {
                let _ = self.close_transport().await;
            }
            return result;
        }
        self.transport.mark_cancelled(id);
        let cancellation_notice = JsonRpcRequest::notification(
            "notifications/cancelled",
            Some(value(json!({"requestId":id,"reason":"worker_cancelled"}))?),
        );
        let _ = tokio::time::timeout(
            CANCELLATION_IO_TIMEOUT,
            self.transport.notify(&cancellation_notice),
        )
        .await;
        // Cancellation raced a polled transport future. Stdio may have a
        // partial frame and an external server may have committed the effect,
        // even when the best-effort notice succeeded. Never reuse this peer
        // generation; bound close and let the broker evict on UnknownEffect.
        let _ = tokio::time::timeout(CANCELLATION_IO_TIMEOUT, self.close_transport()).await;
        Err(McpError::UnknownEffect)
    }

    fn observe_notifications(&mut self) {
        for method in self.transport.take_notifications() {
            if matches!(
                method.as_str(),
                "notifications/tools/list_changed"
                    | "notifications/prompts/list_changed"
                    | "notifications/resources/list_changed"
            ) {
                let _ = self.catalog_generation.fetch_update(
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                    |v| Some(v.saturating_add(1)),
                );
            }
        }
    }

    fn protocol_params(&self, params: Option<IJsonValue>) -> Result<Option<IJsonValue>, McpError> {
        if !self.modern {
            return Ok(params);
        }
        let mut json_value = params.map_or_else(
            || Ok(serde_json::Value::Object(serde_json::Map::new())),
            |value| {
                let bytes = value
                    .canonical_bytes()
                    .map_err(|error| McpError::Protocol(error.to_string()))?;
                serde_json::from_slice::<serde_json::Value>(&bytes)
                    .map_err(|error| McpError::Protocol(error.to_string()))
            },
        )?;
        let object = json_value.as_object_mut().ok_or_else(|| {
            McpError::Protocol("modern request params must be an object".to_owned())
        })?;
        let metadata = object
            .entry("_meta".to_owned())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
            .as_object_mut()
            .ok_or_else(|| McpError::Protocol("request _meta must be an object".to_owned()))?;
        if metadata.contains_key("io.modelcontextprotocol/protocolVersion") {
            return Err(McpError::Protocol(
                "caller params must not override protocol metadata".to_owned(),
            ));
        }
        metadata.insert(
            "io.modelcontextprotocol/protocolVersion".to_owned(),
            json!(self.protocol_version),
        );
        metadata.insert(
            "io.modelcontextprotocol/clientCapabilities".to_owned(),
            json!({}),
        );
        Ok(Some(value(json_value)?))
    }

    async fn call_typed<R: DeserializeOwned>(
        &mut self,
        method: &str,
        params: Option<IJsonValue>,
        timeout: Duration,
    ) -> Result<R, McpError> {
        let result = decode(self.call_raw_once(method, params, timeout).await?);
        if matches!(result, Err(McpError::Protocol(_))) {
            let _ = self.close_transport().await;
        }
        result
    }

    async fn paginated<R, F>(&mut self, method: &str, extract: F) -> Result<Vec<R>, McpError>
    where
        R: DeserializeOwned,
        F: FnMut(IJsonValue) -> Result<Page<R>, McpError> + Clone,
    {
        let first = self.paginated_once(method, extract.clone()).await;
        if !should_reconnect(&first, McpLossState::Catalog) {
            return first;
        }
        self.reconnect_generation().await?;
        self.paginated_once(method, extract).await
    }

    async fn paginated_once<R, F>(
        &mut self,
        method: &str,
        mut extract: F,
    ) -> Result<Vec<R>, McpError>
    where
        R: DeserializeOwned,
        F: FnMut(IJsonValue) -> Result<Page<R>, McpError>,
    {
        let mut output = Vec::new();
        let mut cursor: Option<String> = None;
        let mut seen = BTreeSet::new();
        for _ in 0..MAX_CATALOG_PAGES {
            let params = cursor
                .as_ref()
                .map(|cursor| value(json!({"cursor":cursor})))
                .transpose()?;
            let page = extract(
                self.call_raw_once(method, params, Duration::from_secs(30))
                    .await?,
            );
            let page = match page {
                Ok(page) => page,
                Err(error @ McpError::Protocol(_)) => {
                    let _ = self.close_transport().await;
                    return Err(error);
                }
                Err(error) => return Err(error),
            };
            output.extend(page.items);
            if output.len() > MAX_CATALOG_ITEMS {
                return Err(McpError::CatalogLimit);
            }
            let Some(next) = page.next_cursor else {
                return Ok(output);
            };
            if next.is_empty() || !seen.insert(next.clone()) {
                return Err(McpError::Protocol(
                    "catalog cursor is empty or repeated".to_owned(),
                ));
            }
            cursor = Some(next);
        }
        Err(McpError::CatalogLimit)
    }

    pub async fn continue_tool_call(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        continuation: McpToolContinuation,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        if !self.capabilities.tools {
            return Err(McpError::Unsupported("tools".to_owned()));
        }
        if continuation.round == 0 || continuation.round > MAX_CONTINUATION_ROUNDS {
            return Err(McpError::Protocol(
                "MCP continuation round must be within 1...32".to_owned(),
            ));
        }
        if let Some(task_id) = continuation.task_id {
            if !self.capabilities.supports_tasks() {
                return Err(McpError::Unsupported("tasks/get".to_owned()));
            }
            if task_id.is_empty() {
                return Err(McpError::Protocol(
                    "empty continuation task identity".to_owned(),
                ));
            }
            let result = self
                .call_raw_cancellable(
                    "tasks/get",
                    Some(value(json!({"taskId":task_id}))?),
                    Duration::from_secs(600),
                    &cancellation,
                )
                .await?;
            let result = self.complete_task_poll(result, Some(&cancellation)).await?;
            crate::McpTask::validate_result(&result, &task_id)?;
            return Ok(result);
        }
        let params = value(json!({
            "name": name,
            "arguments": to_json(arguments)?,
            "requestState": to_json(continuation.request_state)?,
            "inputResponses": to_json(continuation.input_responses)?
        }))?;
        self.call_raw_cancellable(
            "tools/call",
            Some(params),
            Duration::from_secs(600),
            &cancellation,
        )
        .await
        .map_err(tool_mutation_error)
    }

    /// A task poll in the official shape: adopt the Kernel keys and, when the
    /// task is `completed` without an inline `result`, fetch it with exactly
    /// one `tasks/result` for the same identity (SEP-1686 splits status and
    /// result; the Kernel contract carries the result inline). The returned
    /// value validates as a Kernel task.
    async fn complete_task_poll(
        &mut self,
        raw: IJsonValue,
        cancellation: Option<&McpCancellationToken>,
    ) -> Result<IJsonValue, McpError> {
        let mut task = to_json(raw)?;
        adopt_official_task_shape(&mut task);
        let needs_result = task["status"] == "completed" && !task["result"].is_object();
        if needs_result {
            let task_id = task["taskId"]
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or_else(|| McpError::Protocol("completed MCP task lacks taskId".to_owned()))?
                .to_owned();
            let params = Some(value(json!({"taskId": task_id}))?);
            let result = match cancellation {
                Some(cancellation) => {
                    self.call_raw_cancellable(
                        "tasks/result",
                        params,
                        Duration::from_secs(600),
                        cancellation,
                    )
                    .await?
                }
                None => {
                    self.call_read_raw("tasks/result", params, Duration::from_secs(600))
                        .await?
                }
            };
            task["result"] = to_json(result)?;
        }
        value(task)
    }

    pub async fn task_mutation_with_resumable_identity(
        &mut self,
        method: &str,
        params: IJsonValue,
        resumable_task_id: &str,
    ) -> Result<IJsonValue, McpError> {
        if !self.capabilities.supports_tasks() || !matches!(method, "tasks/update" | "tasks/cancel")
        {
            return Err(McpError::Unsupported(method.to_owned()));
        }
        if resumable_task_id.is_empty() {
            return Err(McpError::Protocol(
                "resumable task identity is empty".to_owned(),
            ));
        }
        let first = self
            .call_raw_once(method, Some(params), Duration::from_secs(600))
            .await;
        match first {
            Ok(value) => Ok(value),
            Err(McpError::Transport(_) | McpError::Timeout(_) | McpError::Protocol(_)) => {
                if recovery_action(McpLossState::ResumableOperation)
                    != RecoveryAction::QueryIdentity
                {
                    return Err(McpError::UnknownEffect);
                }
                if self.reconnect_generation().await.is_err() {
                    return Err(McpError::UnknownEffect);
                }
                let polled = self
                    .call_raw_once(
                        "tasks/get",
                        Some(value(json!({"taskId":resumable_task_id}))?),
                        Duration::from_secs(600),
                    )
                    .await?;
                self.complete_task_poll(polled, None).await
            }
            Err(error) => Err(error),
        }
    }
}

impl<T: McpTransport> McpPeer for McpClient<T> {
    fn start_scoped_tool(
        &self,
        name: &str,
        arguments: &IJsonValue,
        context: Option<&McpToolCallContext>,
        cancellation: McpCancellationToken,
    ) -> Option<McpPeerFuture<'static, IJsonValue>> {
        if !self.modern || !self.transport.supports_http_discovery() {
            return None;
        }
        let prepared = (|| {
            if self.transport_closed {
                return Err(McpError::Transport("MCP peer is closed".into()));
            }
            if !self.capabilities.tools {
                return Err(McpError::Unsupported("tools".into()));
            }
            if cancellation.is_cancelled() {
                return Err(McpError::Cancelled);
            }
            let mut params = json!({"name":name,"arguments":serde_json::to_value(arguments).map_err(|e| McpError::Protocol(e.to_string()))?});
            if let Some(context) = context {
                if context.idempotency_key.len() != 64
                    || !context
                        .idempotency_key
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(McpError::Protocol(
                        "external-effect idempotency key must be 64 lowercase hexadecimal bytes"
                            .into(),
                    ));
                }
                params["_meta"] = json!({"io.tekes/idempotencyKey":context.idempotency_key});
            }
            Ok((
                self.allocate_request_id()?,
                self.protocol_params(Some(value(params)?))?,
            ))
        })();
        let (id, params) = match prepared {
            Ok(prepared) => prepared,
            Err(error) => return Some(Box::pin(async move { Err(error) })),
        };
        let generation = self.catalog_generation.clone();
        let observer = std::sync::Arc::new(move |method: &str| {
            if matches!(
                method,
                "notifications/tools/list_changed"
                    | "notifications/prompts/list_changed"
                    | "notifications/resources/list_changed"
            ) {
                let _ = generation.fetch_update(
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                    |v| Some(v.saturating_add(1)),
                );
            }
        });
        let future = self.transport.start_scoped_request(
            JsonRpcRequest::call(id, "tools/call", params),
            Duration::from_secs(600),
            cancellation,
            Some(observer),
        )?;
        Some(Box::pin(async move {
            let response = future.await.map_err(|error| {
                if matches!(error, McpError::Cancelled) {
                    McpError::UnknownEffect
                } else {
                    tool_mutation_error(error)
                }
            })?;
            response.validate_for(id).map_err(tool_mutation_error)?;
            response.into_result().map_err(tool_mutation_error)
        }))
    }

    fn server_name(&self) -> &str {
        &self.server_name
    }

    fn protocol_version(&self) -> &str {
        &self.protocol_version
    }

    fn server_identity(&self) -> Option<&McpImplementation> {
        self.server.as_ref()
    }

    fn capabilities(&self) -> &McpCapabilities {
        &self.capabilities
    }

    fn catalog_generation(&self) -> u64 {
        self.catalog_generation
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    fn start_catalog_subscription(&mut self) -> Option<McpPeerFuture<'static, ()>> {
        if !self.modern || self.subscription_started {
            return None;
        }
        let id = match self.allocate_request_id() {
            Ok(id) => id,
            Err(error) => return Some(Box::pin(async { Err(error) })),
        };
        let generation = self.catalog_generation.clone();
        let sink = std::sync::Arc::new(move |_: &serde_json::Value| {
            let _ = generation.fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |v| Some(v.saturating_add(1)),
            );
        });
        let request = self
            .transport
            .start_catalog_subscription(id, &self.capabilities, sink)?;
        self.subscription_started = true;
        let generation = self.catalog_generation.clone();
        Some(Box::pin(async move {
            let result = request.await;
            // Ended streams require a fresh peer generation on the next prepare.
            let _ = generation.fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |v| Some(v.saturating_add(1)),
            );
            result.map(|_| ())
        }))
    }

    fn list_tools(&mut self) -> McpPeerFuture<'_, Vec<McpTool>> {
        Box::pin(async move {
            if !self.capabilities.tools {
                return Err(McpError::Unsupported("tools".to_owned()));
            }
            let tools: Vec<McpTool> = self
                .paginated("tools/list", |value| page(value, "tools"))
                .await?;
            if let Err(error) = reject_duplicates(&tools, |tool| tool.name.as_str(), "tool") {
                let _ = self.close_transport().await;
                return Err(error);
            }
            self.transport.install_tool_catalog(&tools)?;
            Ok(tools)
        })
    }

    fn list_prompts(&mut self) -> McpPeerFuture<'_, Vec<McpPrompt>> {
        Box::pin(async move {
            if !self.capabilities.prompts {
                return Err(McpError::Unsupported("prompts".to_owned()));
            }
            let prompts: Vec<McpPrompt> = self
                .paginated("prompts/list", |value| page(value, "prompts"))
                .await?;
            if let Err(error) = reject_duplicates(&prompts, |prompt| prompt.name.as_str(), "prompt")
            {
                let _ = self.close_transport().await;
                return Err(error);
            }
            Ok(prompts)
        })
    }

    fn list_resources(&mut self) -> McpPeerFuture<'_, Vec<McpResource>> {
        Box::pin(async move {
            if !self.capabilities.resources {
                return Err(McpError::Unsupported("resources".to_owned()));
            }
            let resources: Vec<McpResource> = self
                .paginated("resources/list", |value| page(value, "resources"))
                .await?;
            if let Err(error) =
                reject_duplicates(&resources, |resource| resource.uri.as_str(), "resource")
            {
                let _ = self.close_transport().await;
                return Err(error);
            }
            Ok(resources)
        })
    }

    fn call_tool(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let name = name.to_owned();
        Box::pin(async move {
            if !self.capabilities.tools {
                return Err(McpError::Unsupported("tools".to_owned()));
            }
            let params = value(json!({"name":name,"arguments":to_json(arguments)?}))?;
            self.call_raw_cancellable(
                "tools/call",
                Some(params),
                Duration::from_secs(600),
                &cancellation,
            )
            .await
            .map_err(tool_mutation_error)
        })
    }

    fn call_tool_with_context(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        context: McpToolCallContext,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let name = name.to_owned();
        Box::pin(async move {
            if !self.capabilities.tools {
                return Err(McpError::Unsupported("tools".to_owned()));
            }
            // A token cancelled before request construction proves that no
            // external operation was dispatched. Once call_raw_cancellable
            // starts polling the transport, cancellation can race a committed
            // business effect and must enter reconciliation instead.
            if cancellation.is_cancelled() {
                return Err(McpError::Cancelled);
            }
            if context.idempotency_key.len() != 64
                || !context
                    .idempotency_key
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(McpError::Protocol(
                    "external-effect idempotency key must be 64 lowercase hexadecimal bytes"
                        .to_owned(),
                ));
            }
            let params = value(json!({
                "name":name,
                "arguments":to_json(arguments)?,
                "_meta":{"io.tekes/idempotencyKey":context.idempotency_key}
            }))?;
            self.call_raw_cancellable(
                "tools/call",
                Some(params),
                Duration::from_secs(600),
                &cancellation,
            )
            .await
            .map_err(tool_mutation_error)
        })
    }

    fn call_tool_augmented(
        &mut self,
        name: &str,
        arguments: IJsonValue,
        context: Option<McpToolCallContext>,
        task_ttl_ms: u64,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let name = name.to_owned();
        Box::pin(async move {
            if !self.capabilities.tools {
                return Err(McpError::Unsupported("tools".to_owned()));
            }
            if !self.capabilities.supports_tasks() {
                return Err(McpError::Unsupported(
                    "task-augmented tools/call".to_owned(),
                ));
            }
            if task_ttl_ms == 0 {
                return Err(McpError::Protocol("task ttl must be positive".to_owned()));
            }
            if cancellation.is_cancelled() {
                return Err(McpError::Cancelled);
            }
            let mut params =
                json!({"name":name,"arguments":to_json(arguments)?,"task":{"ttl":task_ttl_ms}});
            if let Some(context) = context {
                if context.idempotency_key.len() != 64
                    || !context
                        .idempotency_key
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(McpError::Protocol(
                        "external-effect idempotency key must be 64 lowercase hexadecimal bytes"
                            .to_owned(),
                    ));
                }
                params["_meta"] = json!({"io.tekes/idempotencyKey":context.idempotency_key});
            }
            let mut created = to_json(
                self.call_raw_cancellable(
                    "tools/call",
                    Some(value(params)?),
                    Duration::from_secs(600),
                    &cancellation,
                )
                .await
                .map_err(tool_mutation_error)?,
            )?;
            if let Some(task) = created.get_mut("task") {
                adopt_official_task_shape(task);
            }
            value(created)
        })
    }

    fn get_prompt(&mut self, name: &str, arguments: IJsonValue) -> McpPeerFuture<'_, IJsonValue> {
        let name = name.to_owned();
        Box::pin(async move {
            if !self.capabilities.prompts {
                return Err(McpError::Unsupported("prompts".to_owned()));
            }
            let params = value(json!({"name":name,"arguments":to_json(arguments)?}))?;
            self.call_read_raw("prompts/get", Some(params), Duration::from_secs(600))
                .await
        })
    }

    fn read_resource(&mut self, uri: &str) -> McpPeerFuture<'_, IJsonValue> {
        let uri = uri.to_owned();
        Box::pin(async move {
            if !self.capabilities.resources {
                return Err(McpError::Unsupported("resources".to_owned()));
            }
            self.call_read_raw(
                "resources/read",
                Some(value(json!({"uri":uri}))?),
                Duration::from_secs(30),
            )
            .await
        })
    }

    fn task_operation(
        &mut self,
        method: &str,
        params: IJsonValue,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let method = method.to_owned();
        Box::pin(async move {
            if !self.capabilities.supports_tasks()
                || !matches!(
                    method.as_str(),
                    "tasks/get" | "tasks/update" | "tasks/cancel"
                )
            {
                return Err(McpError::Unsupported(method));
            }
            if method == "tasks/get" {
                let polled = self
                    .call_read_raw(&method, Some(params), Duration::from_secs(600))
                    .await?;
                self.complete_task_poll(polled, None).await
            } else {
                let mut task = to_json(
                    self.call_raw_once(&method, Some(params), Duration::from_secs(600))
                        .await
                        .map_err(task_mutation_error)?,
                )?;
                adopt_official_task_shape(&mut task);
                value(task)
            }
        })
    }

    fn task_operation_cancellable(
        &mut self,
        method: &str,
        params: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> McpPeerFuture<'_, IJsonValue> {
        let method = method.to_owned();
        Box::pin(async move {
            if !self.capabilities.supports_tasks()
                || !matches!(
                    method.as_str(),
                    "tasks/get" | "tasks/update" | "tasks/cancel"
                )
            {
                return Err(McpError::Unsupported(method));
            }
            let result = self
                .call_raw_cancellable(
                    &method,
                    Some(params),
                    Duration::from_secs(600),
                    &cancellation,
                )
                .await;
            if method == "tasks/get" {
                match result {
                    Err(McpError::UnknownEffect) if cancellation.is_cancelled() => {
                        Err(McpError::Cancelled)
                    }
                    Ok(polled) => self.complete_task_poll(polled, Some(&cancellation)).await,
                    Err(error) => Err(error),
                }
            } else {
                let mut task = to_json(result.map_err(task_mutation_error)?)?;
                adopt_official_task_shape(&mut task);
                value(task)
            }
        })
    }

    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.close_transport()
    }
}

struct Page<T> {
    items: Vec<T>,
    next_cursor: Option<String>,
}

fn page<T: DeserializeOwned>(value: IJsonValue, key: &str) -> Result<Page<T>, McpError> {
    let mut raw = to_json(value)?;
    let object = raw
        .as_object_mut()
        .ok_or_else(|| McpError::Protocol("catalog page is not an object".to_owned()))?;
    let items = object
        .remove(key)
        .ok_or_else(|| McpError::Protocol(format!("catalog page lacks {key}")))?;
    let items =
        serde_json::from_value(items).map_err(|error| McpError::Protocol(error.to_string()))?;
    let next_cursor = object
        .remove("nextCursor")
        .map(serde_json::from_value)
        .transpose()
        .map_err(|error| McpError::Protocol(error.to_string()))?;
    if let Some(result_type) = object.remove("resultType") {
        if result_type.as_str() != Some("complete") {
            return Err(McpError::Protocol(
                "modern catalog page is not complete".to_owned(),
            ));
        }
    }
    if let Some(ttl) = object.remove("ttlMs") {
        if ttl.as_u64().is_none() {
            return Err(McpError::Protocol(
                "modern catalog ttlMs is invalid".to_owned(),
            ));
        }
    }
    if let Some(scope) = object.remove("cacheScope") {
        if !matches!(scope.as_str(), Some("private" | "public" | "none")) {
            return Err(McpError::Protocol(
                "modern catalog cacheScope is invalid".to_owned(),
            ));
        }
    }
    if let Some(metadata) = object.remove("_meta") {
        if !metadata.is_object() {
            return Err(McpError::Protocol(
                "catalog _meta must be an object".to_owned(),
            ));
        }
    }
    if !object.is_empty() {
        return Err(McpError::Protocol(
            "catalog page contains unknown fields".to_owned(),
        ));
    }
    Ok(Page { items, next_cursor })
}

/// Official SEP-1686 task shapes (protocol 2025-11-25, the shipped SDKs) map
/// onto the Kernel task vocabulary: `pollInterval` → `pollIntervalMs` and
/// `ttl` → `ttlMs`. Kernel keys win when both are present; nothing else is
/// rewritten, so a peer that already speaks the Kernel shape is untouched.
fn adopt_official_task_shape(task: &mut serde_json::Value) {
    let Some(object) = task.as_object_mut() else {
        return;
    };
    for (official, kernel) in [("pollInterval", "pollIntervalMs"), ("ttl", "ttlMs")] {
        if !object.contains_key(kernel) {
            if let Some(value) = object.get(official).cloned() {
                object.insert(kernel.to_owned(), value);
            }
        }
    }
}

fn value(value: serde_json::Value) -> Result<IJsonValue, McpError> {
    IJsonValue::parse(
        &serde_json::to_vec(&value).map_err(|error| McpError::Protocol(error.to_string()))?,
    )
    .map_err(|error| McpError::Protocol(error.to_string()))
}

fn to_json(value: IJsonValue) -> Result<serde_json::Value, McpError> {
    serde_json::from_slice(
        &value
            .canonical_bytes()
            .map_err(|error| McpError::Protocol(error.to_string()))?,
    )
    .map_err(|error| McpError::Protocol(error.to_string()))
}

fn decode<T: DeserializeOwned>(value: IJsonValue) -> Result<T, McpError> {
    serde_json::from_slice(
        &value
            .canonical_bytes()
            .map_err(|error| McpError::Protocol(error.to_string()))?,
    )
    .map_err(|error| McpError::Protocol(error.to_string()))
}

fn reject_duplicates<T, F>(items: &[T], key: F, family: &str) -> Result<(), McpError>
where
    F: Fn(&T) -> &str,
{
    let mut seen = BTreeSet::new();
    for item in items {
        let value = key(item);
        if value.is_empty() || !seen.insert(value) {
            return Err(McpError::Protocol(format!(
                "{family} catalog contains an empty or duplicate identity"
            )));
        }
    }
    Ok(())
}

fn should_reconnect<T>(result: &Result<T, McpError>, state: McpLossState) -> bool {
    recovery_action(state) == RecoveryAction::ReconnectOnce
        && matches!(result, Err(McpError::Transport(_) | McpError::Timeout(_)))
}

fn tool_mutation_error(error: McpError) -> McpError {
    mutation_error(error, McpLossState::ToolWithoutProof)
}

fn task_mutation_error(error: McpError) -> McpError {
    mutation_error(error, McpLossState::TaskMutationWithoutProof)
}

fn mutation_error(error: McpError, state: McpLossState) -> McpError {
    match error {
        McpError::Transport(_) | McpError::Timeout(_) | McpError::Protocol(_)
            if recovery_action(state) == RecoveryAction::UnknownEffect =>
        {
            McpError::UnknownEffect
        }
        other => other,
    }
}

fn is_legacy_evidence(error: &McpError) -> bool {
    match error {
        McpError::Remote { code: -32601, .. } => true,
        // A textual protocol-version rejection that enumerates supported
        // versions (public servers answer `server/discover` this way):
        // evidence only when a listed version predates the modern protocol.
        McpError::Remote {
            code: -32600,
            message,
            ..
        } if message.to_ascii_lowercase().contains("protocol version") => {
            protocol_versions_in(message)
                .iter()
                .any(|version| version.as_str() < crate::MODERN_PROTOCOL_VERSION)
        }
        McpError::NoMutualProtocol(versions) => versions
            .iter()
            .any(|version| version.as_str() < crate::MODERN_PROTOCOL_VERSION),
        McpError::Remote {
            code: -32022,
            data: Some(data),
            ..
        } => serde_json::to_value(data).ok().is_some_and(|value| {
            value["supported"].as_array().is_some_and(|versions| {
                versions.iter().any(|version| {
                    version
                        .as_str()
                        .is_some_and(|version| version < crate::MODERN_PROTOCOL_VERSION)
                })
            })
        }),
        _ => false,
    }
}

/// Every `YYYY-MM-DD` token in a server message, in order.
fn protocol_versions_in(message: &str) -> Vec<String> {
    let bytes = message.as_bytes();
    let mut versions = Vec::new();
    let mut index = 0;
    while index + 10 <= bytes.len() {
        let window = &bytes[index..index + 10];
        let shaped = window
            .iter()
            .enumerate()
            .all(|(position, byte)| match position {
                4 | 7 => *byte == b'-',
                _ => byte.is_ascii_digit(),
            });
        let bounded = index
            .checked_sub(1)
            .is_none_or(|before| !bytes[before].is_ascii_digit())
            && bytes
                .get(index + 10)
                .is_none_or(|after| !after.is_ascii_digit());
        if shaped && bounded {
            versions.push(String::from_utf8_lossy(window).into_owned());
            index += 10;
        } else {
            index += 1;
        }
    }
    versions
}

impl McpClient<crate::HttpTransport> {
    /// Shared-client calls for a negotiated modern HTTP connection.
    pub async fn call_tool_scoped(
        &self,
        name: &str,
        arguments: IJsonValue,
        cancellation: McpCancellationToken,
    ) -> Result<IJsonValue, McpError> {
        self.start_scoped_tool(name, &arguments, None, cancellation)
            .ok_or_else(|| {
                McpError::Unsupported("concurrent calls require an open modern HTTP peer".into())
            })?
            .await
    }
}

#[cfg(test)]
mod legacy_evidence_tests {
    use super::*;
    #[test]
    fn auto_fallback_requires_explicit_protocol_evidence() {
        let remote = |code, data| McpError::Remote {
            code,
            message: "test".into(),
            data,
        };
        assert!(is_legacy_evidence(&remote(-32601, None)));
        for status in [400, 401, 403, 404, 405, 500, 503] {
            assert!(!is_legacy_evidence(&McpError::Transport(format!(
                "HTTP {status}"
            ))));
        }
        assert!(!is_legacy_evidence(&McpError::Timeout("discovery".into())));
        assert!(!is_legacy_evidence(&remote(-32022, None)));
        assert!(!is_legacy_evidence(&remote(
            -32022,
            Some(value(json!({"supported":["2027-01-01"]})).unwrap())
        )));
        assert!(is_legacy_evidence(&remote(
            -32022,
            Some(value(json!({"supported":[crate::LEGACY_PROTOCOL_VERSION]})).unwrap())
        )));
    }

    /// DeepWiki answers `server/discover` with HTTP 400 and a textual
    /// -32600 listing its versions; that enumeration is legacy evidence, while
    /// a bare -32600 or one that lists only newer versions is not.
    #[test]
    fn textual_protocol_version_rejection_is_legacy_evidence_when_it_lists_an_older_version() {
        let textual = |message: &str| McpError::Remote {
            code: -32600,
            message: message.into(),
            data: None,
        };
        assert!(is_legacy_evidence(&textual(
            "Bad Request: Unsupported protocol version: 2026-07-28. Supported versions: 2024-11-05, 2025-03-26, 2025-06-18, 2025-11-25"
        )));
        assert!(!is_legacy_evidence(&textual(
            "Bad Request: Unsupported protocol version: 2026-07-28. Supported versions: 2027-01-01"
        )));
        assert!(!is_legacy_evidence(&textual(
            "Bad Request: invalid params 2025-11-25"
        )));
        assert!(!is_legacy_evidence(&textual(
            "Bad Request: Unsupported protocol version"
        )));
        assert_eq!(
            protocol_versions_in("v 2025-11-25, x2025-06-18y, 12025-01-01"),
            vec!["2025-11-25".to_owned(), "2025-06-18".to_owned()]
        );
    }
}
