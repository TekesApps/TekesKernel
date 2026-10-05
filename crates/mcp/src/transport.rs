use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::os::fd::OwnedFd;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::StreamExt;

/// Observes the method of each notification a transport delivers.
pub type McpMethodObserver = Arc<dyn Fn(&str) + Send + Sync>;
/// Observes each notification's full value.
pub type McpValueObserver = Arc<dyn Fn(&serde_json::Value) + Send + Sync>;
use base64::Engine as _;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use schema::IJsonValue;
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::pipe;
use url::Url;

use crate::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, MAX_FRAME_BYTES, McpError};

pub trait McpTransport: Send {
    fn supports_http_discovery(&self) -> bool {
        false
    }
    fn start_catalog_subscription(
        &self,
        _id: u64,
        _capabilities: &crate::McpCapabilities,
        _sink: Arc<dyn Fn(&serde_json::Value) + Send + Sync>,
    ) -> Option<crate::McpPeerFuture<'static, JsonRpcResponse>> {
        None
    }

    fn install_tool_catalog(&mut self, tools: &[crate::McpTool]) -> Result<(), McpError> {
        let catalog = crate::parameter_headers::ParameterHeaders::catalog(tools)?;
        if catalog.values().any(|projection| !projection.is_empty()) {
            return Err(McpError::Unsupported(
                "transport cannot carry MCP parameter headers".into(),
            ));
        }
        Ok(())
    }

    fn start_scoped_request(
        &self,
        _request: JsonRpcRequest,
        _timeout: Duration,
        _cancellation: crate::McpCancellationToken,
        _observer: Option<McpMethodObserver>,
    ) -> Option<crate::McpPeerFuture<'static, JsonRpcResponse>> {
        None
    }

    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        timeout: Duration,
    ) -> crate::McpPeerFuture<'a, JsonRpcResponse>;

    fn notify<'a>(&'a mut self, notification: &'a JsonRpcRequest) -> crate::McpPeerFuture<'a, ()>;

    fn close(&mut self) -> crate::McpPeerFuture<'_, ()>;

    fn reconnect(&mut self) -> crate::McpPeerFuture<'_, ()> {
        Box::pin(async {
            Err(McpError::Transport(
                "MCP transport does not support reconnect".to_owned(),
            ))
        })
    }

    fn mark_cancelled(&mut self, _id: u64) {}

    fn take_notifications(&mut self) -> Vec<String> {
        Vec::new()
    }
}

pub struct StdioTransport {
    generation: Option<StdioGeneration>,
    diagnostics: Arc<Mutex<Vec<u8>>>,
    spawn: StdioSpawnSpec,
    cancelled_ids: BTreeSet<u64>,
    notifications: Vec<String>,
}

struct StdioSpawnSpec {
    argv: Vec<String>,
    cwd: Option<PathBuf>,
    environment: BTreeMap<String, SecretText>,
}

// Deliberately has no Debug or Display implementation: configuration and
// authorization values must not enter diagnostics through derived formatting.
struct SecretText(String);

impl SecretText {
    fn new(value: String) -> Self {
        Self(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.clear();
    }
}

struct StdioParts {
    generation: StdioGeneration,
    diagnostics: Arc<Mutex<Vec<u8>>>,
}

struct StdioGeneration {
    child: Child,
    stdin: File,
    stdout: File,
    stderr_task: std::thread::JoinHandle<()>,
    stderr_stop: Arc<AtomicBool>,
}

const STDERR_POLL_INTERVAL: Duration = Duration::from_millis(10);
const STDIO_REAP_TIMEOUT: Duration = Duration::from_secs(1);

impl StdioTransport {
    pub fn spawn(
        argv: &[String],
        cwd: Option<PathBuf>,
        environment: &BTreeMap<String, String>,
    ) -> Result<Self, McpError> {
        let spawn = StdioSpawnSpec {
            argv: argv.to_vec(),
            cwd,
            environment: environment
                .iter()
                .map(|(name, value)| (name.clone(), SecretText::new(value.clone())))
                .collect(),
        };
        let parts = spawn_stdio(&spawn)?;
        Ok(Self {
            generation: Some(parts.generation),
            diagnostics: parts.diagnostics,
            spawn,
            cancelled_ids: BTreeSet::new(),
            notifications: Vec::new(),
        })
    }

    async fn terminate(&mut self) -> Result<(), McpError> {
        let Some(generation) = self.generation.take() else {
            return Ok(());
        };
        // Detach the logical generation before any physical process work. The
        // blocking kill/wait/join sequence must never become Tokio runtime
        // work: timed-out blocking tasks are not cancellable at shutdown.
        let reap = start_stdio_reaper(generation)?;
        await_stdio_reaper(reap).await
    }

    async fn restart(&mut self) -> Result<(), McpError> {
        self.terminate().await?;
        let parts = spawn_stdio(&self.spawn)?;
        self.generation = Some(parts.generation);
        self.diagnostics = parts.diagnostics;
        self.cancelled_ids.clear();
        self.notifications.clear();
        Ok(())
    }

    async fn request_inner(
        &mut self,
        request: &JsonRpcRequest,
        timeout: Duration,
    ) -> Result<JsonRpcResponse, McpError> {
        let id = request
            .id
            .ok_or_else(|| McpError::Protocol("request id is absent".to_owned()))?;
        self.write(request).await?;
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let line = tokio::time::timeout_at(deadline, self.read_response_line())
                .await
                .map_err(|_| McpError::Timeout(request.method.clone()))??;
            match decode_inbound(&line, id, &mut self.cancelled_ids)? {
                Inbound::Response(response) => return Ok(response),
                Inbound::Notification(method) => self.notifications.push(method),
                Inbound::LateCancelled => {}
            }
        }
    }

    async fn close_after_protocol_error<T>(
        &mut self,
        result: Result<T, McpError>,
    ) -> Result<T, McpError> {
        if matches!(result, Err(McpError::Protocol(_))) {
            let _ = self.terminate().await;
        }
        result
    }

    async fn write(&mut self, request: &JsonRpcRequest) -> Result<(), McpError> {
        let bytes = request.canonical_line()?;
        if bytes.len() > MAX_FRAME_BYTES {
            return Err(McpError::Protocol(
                "outbound frame exceeds 4 MiB".to_owned(),
            ));
        }
        let mut stdin = pipe::Sender::from_file(
            self.generation
                .as_ref()
                .ok_or_else(|| McpError::Transport("stdio generation is closed".to_owned()))?
                .stdin
                .try_clone()
                .map_err(|error| McpError::Transport(error.to_string()))?,
        )
        .map_err(|error| McpError::Transport(error.to_string()))?;
        stdin
            .write_all(&bytes)
            .await
            .map_err(|error| McpError::Transport(error.to_string()))?;
        stdin
            .flush()
            .await
            .map_err(|error| McpError::Transport(error.to_string()))
    }

    async fn read_response_line(&mut self) -> Result<Vec<u8>, McpError> {
        let mut bytes = Vec::new();
        let mut stdout = pipe::Receiver::from_file(
            self.generation
                .as_ref()
                .ok_or_else(|| McpError::Transport("stdio generation is closed".to_owned()))?
                .stdout
                .try_clone()
                .map_err(|error| McpError::Transport(error.to_string()))?,
        )
        .map_err(|error| McpError::Transport(error.to_string()))?;
        while bytes.len() <= MAX_FRAME_BYTES + 1 {
            let mut byte = [0_u8; 1];
            let read = stdout
                .read(&mut byte)
                .await
                .map_err(|error| McpError::Transport(error.to_string()))?;
            if read == 0 {
                return Err(McpError::Transport("stdio EOF".to_owned()));
            }
            bytes.push(byte[0]);
            if byte[0] == b'\n' {
                break;
            }
        }
        if bytes.last() != Some(&b'\n') || bytes.len() > MAX_FRAME_BYTES + 1 {
            return Err(McpError::Protocol(
                "inbound frame exceeds 4 MiB or lacks LF framing".to_owned(),
            ));
        }
        bytes.pop();
        if bytes.last() == Some(&b'\r') {
            return Err(McpError::Protocol(
                "stdio frames require LF rather than CRLF".to_owned(),
            ));
        }
        Ok(bytes)
    }
}

fn spawn_stdio(spawn: &StdioSpawnSpec) -> Result<StdioParts, McpError> {
    let argv = &spawn.argv;
    let cwd = spawn.cwd.clone();
    let environment = &spawn.environment;
    let (program, arguments) = argv
        .split_first()
        .ok_or_else(|| McpError::Transport("stdio argv is empty".to_owned()))?;
    let mut command = Command::new(program);
    command
        .args(arguments)
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value.as_str());
    }
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let mut child = command
        .spawn()
        .map_err(|error| McpError::Transport(error.to_string()))?;
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| McpError::Transport("stdio stdin is absent".to_owned()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| McpError::Transport("stdio stdout is absent".to_owned()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| McpError::Transport("stdio stderr is absent".to_owned()))?;
    let mut stderr = File::from(OwnedFd::from(stderr));
    set_nonblocking(&stderr)?;
    let diagnostics = Arc::new(Mutex::new(Vec::new()));
    let diagnostics_writer = Arc::clone(&diagnostics);
    let stderr_stop = Arc::new(AtomicBool::new(false));
    let stderr_task_stop = Arc::clone(&stderr_stop);
    let stderr_task = std::thread::Builder::new()
        .name("tekes-mcp-stderr".to_owned())
        .spawn(move || {
            let mut buffer = [0_u8; 8 * 1024];
            while !stderr_task_stop.load(Ordering::Acquire) {
                let count = match stderr.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(count) => count,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(STDERR_POLL_INTERVAL);
                        continue;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                };
                if count > 0 {
                    let mut retained = diagnostics_writer
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    let remaining = (64_usize * 1024).saturating_sub(retained.len());
                    retained.extend_from_slice(&buffer[..count.min(remaining)]);
                }
            }
        })
        .map_err(|error| McpError::Transport(error.to_string()))?;
    Ok(StdioParts {
        generation: StdioGeneration {
            child,
            stdin: File::from(OwnedFd::from(stdin)),
            stdout: File::from(OwnedFd::from(stdout)),
            stderr_task,
            stderr_stop,
        },
        diagnostics,
    })
}

fn set_nonblocking(file: &File) -> Result<(), McpError> {
    let flags =
        rustix::fs::fcntl_getfl(file).map_err(|error| McpError::Transport(error.to_string()))?;
    rustix::fs::fcntl_setfl(file, flags | rustix::fs::OFlags::NONBLOCK)
        .map_err(|error| McpError::Transport(error.to_string()))
}

fn reap_stdio(mut generation: StdioGeneration) -> Result<(), McpError> {
    generation.stderr_stop.store(true, Ordering::Release);
    drop(generation.stdin);
    drop(generation.stdout);
    if generation
        .child
        .try_wait()
        .map_err(|error| McpError::Transport(error.to_string()))?
        .is_none()
    {
        generation
            .child
            .kill()
            .map_err(|error| McpError::Transport(error.to_string()))?;
    }
    generation
        .child
        .wait()
        .map_err(|error| McpError::Transport(error.to_string()))?;
    generation
        .stderr_task
        .join()
        .map_err(|_| McpError::Transport("stdio stderr drain thread panicked".to_owned()))
}

fn start_stdio_reaper(
    generation: StdioGeneration,
) -> Result<tokio::sync::oneshot::Receiver<Result<(), McpError>>, McpError> {
    let generation = Arc::new(Mutex::new(Some(generation)));
    let reaper_generation = Arc::clone(&generation);
    let (sender, receiver) = tokio::sync::oneshot::channel();
    match std::thread::Builder::new()
        .name("tekes-mcp-reaper".to_owned())
        .spawn(move || {
            let generation = reaper_generation
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .take()
                .expect("stdio reaper owns one generation");
            let _ = sender.send(reap_stdio(generation));
        }) {
        Ok(_) => Ok(receiver),
        Err(error) => {
            if let Some(mut generation) = generation
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .take()
            {
                // Thread creation failure must not leave the child running.
                // kill is a non-waiting syscall; dropping Child and JoinHandle
                // after it returns does not join or block this caller.
                generation.stderr_stop.store(true, Ordering::Release);
                let _ = generation.child.kill();
            }
            Err(McpError::Transport(format!(
                "stdio reaper thread could not start: {error}"
            )))
        }
    }
}

async fn await_stdio_reaper(
    receiver: tokio::sync::oneshot::Receiver<Result<(), McpError>>,
) -> Result<(), McpError> {
    match tokio::time::timeout(STDIO_REAP_TIMEOUT, receiver).await {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => Err(McpError::Transport(
            "stdio reaper thread exited without a result".to_owned(),
        )),
        Err(_) => Err(McpError::Transport(
            "stdio generation reap exceeded one second".to_owned(),
        )),
    }
}

impl McpTransport for StdioTransport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        timeout: Duration,
    ) -> crate::McpPeerFuture<'a, JsonRpcResponse> {
        Box::pin(async move {
            let result = self.request_inner(request, timeout).await;
            self.close_after_protocol_error(result).await
        })
    }

    fn notify<'a>(&'a mut self, notification: &'a JsonRpcRequest) -> crate::McpPeerFuture<'a, ()> {
        Box::pin(async move {
            if notification.id.is_some() {
                return Err(McpError::Protocol("notification has an id".to_owned()));
            }
            self.write(notification).await
        })
    }

    fn close(&mut self) -> crate::McpPeerFuture<'_, ()> {
        Box::pin(self.terminate())
    }

    fn reconnect(&mut self) -> crate::McpPeerFuture<'_, ()> {
        Box::pin(self.restart())
    }

    fn mark_cancelled(&mut self, id: u64) {
        self.cancelled_ids.insert(id);
    }

    fn take_notifications(&mut self) -> Vec<String> {
        std::mem::take(&mut self.notifications)
    }
}

impl Drop for StdioTransport {
    fn drop(&mut self) {
        if let Some(generation) = self.generation.take() {
            let _ = start_stdio_reaper(generation);
        }
    }
}

pub struct HttpTransport {
    parameter_headers: BTreeMap<String, crate::parameter_headers::ParameterHeaders>,
    client: reqwest::Client,
    url: Url,
    headers: Vec<(HeaderName, SecretText)>,
    authorization: Arc<AuthorizationProvider>,
    session_id: Option<HeaderValue>,
    session_authorization_identity: Option<String>,
    notifications: Vec<String>,
    notification_handler: Option<McpMethodObserver>,
    notification_value_handler: Option<McpValueObserver>,
    closed: bool,
    scoped_generation: crate::McpCancellationToken,
}

type AuthorizationProvider =
    dyn Fn() -> Result<HttpRequestAuthorization, McpError> + Send + Sync + 'static;

impl Drop for HttpTransport {
    fn drop(&mut self) {
        self.scoped_generation.cancel();
    }
}

/// One send-scoped HTTP authorization projection. It intentionally has no
/// `Debug`/`Serialize` implementation and clears owned credential strings on
/// drop; callers create a fresh value for every request.
pub struct HttpRequestAuthorization {
    pub identity: String,
    pub headers: BTreeMap<String, String>,
    pub bearer: Option<String>,
}

impl Drop for HttpRequestAuthorization {
    fn drop(&mut self) {
        for value in self.headers.values_mut() {
            value.clear();
        }
        if let Some(value) = &mut self.bearer {
            value.clear();
        }
    }
}

impl HttpTransport {
    /// Modern HTTP requests do not share a legacy session or response queue.
    /// Cancellation drops only this request's socket future.
    pub async fn request_modern_scoped(
        &self,
        request: &JsonRpcRequest,
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
    ) -> Result<JsonRpcResponse, McpError> {
        self.request_modern_scoped_observed(request, timeout, cancellation, None)
            .await
    }

    pub(crate) async fn request_modern_scoped_observed(
        &self,
        request: &JsonRpcRequest,
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
        observer: Option<McpMethodObserver>,
    ) -> Result<JsonRpcResponse, McpError> {
        self.start_modern_scoped(request.clone(), timeout, cancellation, observer)
            .await
    }

    /// Own the request while retaining cancellation by its parent generation.
    /// This is the transport primitive for releasing a broker peer lock before IO.
    pub(crate) fn start_modern_scoped(
        &self,
        request: JsonRpcRequest,
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
        observer: Option<McpMethodObserver>,
    ) -> crate::McpPeerFuture<'static, JsonRpcResponse> {
        self.start_modern_scoped_detailed(request, timeout, cancellation, observer, None)
    }

    fn start_modern_scoped_detailed(
        &self,
        request: JsonRpcRequest,
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
        observer: Option<McpMethodObserver>,
        detailed: Option<McpValueObserver>,
    ) -> crate::McpPeerFuture<'static, JsonRpcResponse> {
        let original = self.notification_handler.clone();
        let handler: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(move |method| {
            if let Some(observer) = &observer {
                observer(method);
            }
            if let Some(original) = &original {
                original(method);
            }
        });
        let generation = self.scoped_generation.clone();
        let mut scoped = Self {
            parameter_headers: self.parameter_headers.clone(),
            client: self.client.clone(),
            url: self.url.clone(),
            headers: self
                .headers
                .iter()
                .map(|(k, v)| (k.clone(), SecretText::new(v.as_str().to_owned())))
                .collect(),
            authorization: self.authorization.clone(),
            session_id: None,
            session_authorization_identity: None,
            notifications: Vec::new(),
            notification_handler: Some(handler),
            notification_value_handler: detailed,
            closed: self.closed,
            scoped_generation: crate::McpCancellationToken::default(),
        };
        Box::pin(async move {
            let value =
                serde_json::to_value(&request).map_err(|e| McpError::Protocol(e.to_string()))?;
            if value
                .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")
                .and_then(serde_json::Value::as_str)
                != Some(crate::MODERN_PROTOCOL_VERSION)
            {
                return Err(McpError::Unsupported(
                    "request-scoped HTTP requires modern protocol metadata".into(),
                ));
            }
            if scoped.closed {
                return Err(McpError::Transport("HTTP transport is closed".into()));
            }
            tokio::select! {
                biased;
                _ = generation.cancelled() => Err(McpError::Cancelled),
                _ = cancellation.cancelled() => Err(McpError::Cancelled),
                result = scoped.request(&request,timeout) => result,
            }
        })
    }

    /// Observe each validated SSE notification before waiting for the final
    /// response. The queued drain remains available via take_notifications.
    /// Open a dedicated subscription with request-local correlation and cancellation.
    pub fn listen_subscription(
        &self,
        id: u64,
        capabilities: &crate::McpCapabilities,
        resource_uris: &[String],
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
        sink: Arc<dyn Fn(&serde_json::Value) + Send + Sync>,
    ) -> crate::McpPeerFuture<'static, JsonRpcResponse> {
        let filter = crate::McpSubscriptionFilter::new(id, capabilities, resource_uris);
        if id == 0 || filter.is_empty() {
            return Box::pin(async {
                Err(McpError::Unsupported(
                    "empty subscription or invalid identity".into(),
                ))
            });
        }
        let mut params = filter.request_params();
        params["_meta"] = serde_json::json!({"io.modelcontextprotocol/protocolVersion":crate::MODERN_PROTOCOL_VERSION});
        let params = match IJsonValue::parse_str(&params.to_string())
            .map_err(|error| McpError::Protocol(error.to_string()))
        {
            Ok(params) => params,
            Err(error) => return Box::pin(async { Err(error) }),
        };
        let filter = Mutex::new(filter);
        let callback = Arc::new(move |notification: &serde_json::Value| {
            let accepted = filter
                .lock()
                .map(|mut filter| filter.accepts(notification))
                .unwrap_or(false);
            if accepted {
                sink(notification);
            }
        });
        self.start_modern_scoped_detailed(
            JsonRpcRequest::call(id, "subscriptions/listen", Some(params)),
            timeout,
            cancellation,
            None,
            Some(callback),
        )
    }

    pub fn with_notification_handler(
        mut self,
        handler: impl Fn(&str) + Send + Sync + 'static,
    ) -> Self {
        self.notification_handler = Some(Arc::new(handler));
        self
    }
    pub fn new(
        url: Url,
        headers: &BTreeMap<String, String>,
        bearer: Option<&str>,
        allow_test_http: bool,
    ) -> Result<Self, McpError> {
        let fixed_bearer = bearer.map(|value| SecretText::new(value.to_owned()));
        let identity = if bearer.is_some() {
            "fixed-bearer".to_owned()
        } else {
            "anonymous".to_owned()
        };
        Self::new_with_authorization_provider(
            url,
            headers,
            move || {
                Ok((
                    identity.clone(),
                    fixed_bearer.as_ref().map(|value| value.as_str().to_owned()),
                ))
            },
            allow_test_http,
        )
    }

    pub fn new_with_authorization_provider<F>(
        url: Url,
        headers: &BTreeMap<String, String>,
        authorization: F,
        allow_test_http: bool,
    ) -> Result<Self, McpError>
    where
        F: Fn() -> Result<(String, Option<String>), McpError> + Send + Sync + 'static,
    {
        Self::new_with_request_authorization_provider(
            url,
            headers,
            move || {
                let (identity, bearer) = authorization()?;
                Ok(HttpRequestAuthorization {
                    identity,
                    headers: BTreeMap::new(),
                    bearer,
                })
            },
            allow_test_http,
        )
    }

    pub fn new_with_request_authorization_provider<F>(
        url: Url,
        headers: &BTreeMap<String, String>,
        authorization: F,
        allow_test_http: bool,
    ) -> Result<Self, McpError>
    where
        F: Fn() -> Result<HttpRequestAuthorization, McpError> + Send + Sync + 'static,
    {
        let loopback = url
            .host_str()
            .is_some_and(|host| matches!(host, "127.0.0.1" | "::1" | "localhost"));
        if url.scheme() != "https" && !(allow_test_http && loopback && url.scheme() == "http") {
            return Err(McpError::Transport(
                "HTTP MCP requires HTTPS outside an explicit loopback test".to_owned(),
            ));
        }
        let mut configured = Vec::new();
        for (name, value) in headers {
            let name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|error| McpError::Transport(error.to_string()))?;
            if name == CONTENT_TYPE
                || name == ACCEPT
                || name == AUTHORIZATION
                || name.as_str().eq_ignore_ascii_case("mcp-session-id")
                || name.as_str().eq_ignore_ascii_case("mcp-protocol-version")
                || name.as_str().eq_ignore_ascii_case("mcp-method")
                || name.as_str().eq_ignore_ascii_case("mcp-name")
                || name.as_str().starts_with("mcp-param-")
            {
                return Err(McpError::Transport(format!(
                    "reserved MCP HTTP header {name} cannot be configured"
                )));
            }
            HeaderValue::from_str(value).map_err(|error| McpError::Transport(error.to_string()))?;
            configured.push((name, SecretText::new(value.clone())));
        }
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| McpError::Transport(error.to_string()))?;
        Ok(Self {
            parameter_headers: BTreeMap::new(),
            client,
            url,
            headers: configured,
            authorization: Arc::new(authorization),
            session_id: None,
            session_authorization_identity: None,
            notification_handler: None,
            notification_value_handler: None,
            notifications: Vec::new(),
            closed: false,
            scoped_generation: crate::McpCancellationToken::default(),
        })
    }

    fn request_headers(&mut self, request: &JsonRpcRequest) -> Result<HeaderMap, McpError> {
        if self.closed {
            return Err(McpError::Transport("HTTP MCP peer is closed".to_owned()));
        }
        let authorization = (self.authorization)()?;
        if authorization.identity.is_empty() {
            return Err(McpError::Transport(
                "HTTP authorization identity is empty".to_owned(),
            ));
        }
        if self
            .session_authorization_identity
            .as_ref()
            .is_some_and(|current| current != &authorization.identity)
        {
            self.session_id = None;
        }
        self.session_authorization_identity = Some(authorization.identity.clone());
        let mut map = HeaderMap::new();
        map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        map.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/event-stream"),
        );
        for (name, value) in &self.headers {
            map.insert(
                name.clone(),
                HeaderValue::from_str(value.as_str())
                    .map_err(|error| McpError::Transport(error.to_string()))?,
            );
        }
        for (name, value) in &authorization.headers {
            let name = HeaderName::from_bytes(name.as_bytes())
                .map_err(|error| McpError::Transport(error.to_string()))?;
            if name == CONTENT_TYPE
                || name == ACCEPT
                || name == AUTHORIZATION
                || name.as_str().eq_ignore_ascii_case("mcp-session-id")
                || name.as_str().eq_ignore_ascii_case("mcp-protocol-version")
                || name.as_str().eq_ignore_ascii_case("mcp-method")
                || name.as_str().eq_ignore_ascii_case("mcp-name")
                || name.as_str().starts_with("mcp-param-")
            {
                return Err(McpError::Transport(format!(
                    "reserved MCP HTTP header {name} cannot be configured"
                )));
            }
            map.insert(
                name,
                HeaderValue::from_str(value)
                    .map_err(|error| McpError::Transport(error.to_string()))?,
            );
        }
        if let Some(bearer) = &authorization.bearer {
            let header = HeaderValue::from_str(&format!("Bearer {bearer}"))
                .map_err(|error| McpError::Transport(error.to_string()))?;
            map.insert(AUTHORIZATION, header);
        }
        if let Some(session_id) = &self.session_id {
            map.insert(
                HeaderName::from_static("mcp-session-id"),
                session_id.clone(),
            );
        }
        let request_value =
            serde_json::to_value(request).map_err(|error| McpError::Protocol(error.to_string()))?;
        if let Some(version) = request_value
            .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")
            .and_then(serde_json::Value::as_str)
        {
            map.insert(
                HeaderName::from_static("mcp-protocol-version"),
                HeaderValue::from_str(version)
                    .map_err(|error| McpError::Protocol(error.to_string()))?,
            );
            map.insert(
                HeaderName::from_static("mcp-method"),
                HeaderValue::from_str(&request.method)
                    .map_err(|error| McpError::Protocol(error.to_string()))?,
            );
            let key = match request.method.as_str() {
                "tools/call" | "prompts/get" => Some("name"),
                "resources/read" => Some("uri"),
                "tasks/get" | "tasks/update" | "tasks/cancel" => Some("taskId"),
                _ => None,
            };
            if let Some(name) = key.and_then(|key| request_value["params"][key].as_str()) {
                let encoded = if !name.is_empty()
                    && name.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
                    && name.trim() == name
                    && !(name.starts_with("=?base64?") && name.ends_with("?="))
                {
                    name.to_owned()
                } else {
                    format!(
                        "=?base64?{}?=",
                        base64::engine::general_purpose::STANDARD.encode(name)
                    )
                };
                map.insert(
                    HeaderName::from_static("mcp-name"),
                    HeaderValue::from_str(&encoded)
                        .map_err(|error| McpError::Protocol(error.to_string()))?,
                );
            }
        }
        if request.method == "tools/call" && map.contains_key("mcp-protocol-version") {
            if let Some(projection) = request_value["params"]["name"]
                .as_str()
                .and_then(|name| self.parameter_headers.get(name))
            {
                for (name, value) in projection.project(&request_value["params"]["arguments"])? {
                    map.insert(
                        HeaderName::from_bytes(name.as_bytes())
                            .map_err(|e| McpError::Protocol(e.to_string()))?,
                        HeaderValue::from_str(&value)
                            .map_err(|e| McpError::Protocol(e.to_string()))?,
                    );
                }
            }
        }
        Ok(map)
    }
}

impl McpTransport for HttpTransport {
    fn start_catalog_subscription(
        &self,
        id: u64,
        capabilities: &crate::McpCapabilities,
        sink: Arc<dyn Fn(&serde_json::Value) + Send + Sync>,
    ) -> Option<crate::McpPeerFuture<'static, JsonRpcResponse>> {
        if crate::McpSubscriptionFilter::new(id, capabilities, &[]).is_empty() {
            return None;
        }
        Some(self.listen_subscription(
            id,
            capabilities,
            &[],
            Duration::from_secs(600),
            crate::McpCancellationToken::default(),
            sink,
        ))
    }

    fn install_tool_catalog(&mut self, tools: &[crate::McpTool]) -> Result<(), McpError> {
        self.parameter_headers = crate::parameter_headers::ParameterHeaders::catalog(tools)?;
        Ok(())
    }

    fn supports_http_discovery(&self) -> bool {
        true
    }

    fn start_scoped_request(
        &self,
        request: JsonRpcRequest,
        timeout: Duration,
        cancellation: crate::McpCancellationToken,
        observer: Option<McpMethodObserver>,
    ) -> Option<crate::McpPeerFuture<'static, JsonRpcResponse>> {
        Some(self.start_modern_scoped(request, timeout, cancellation, observer))
    }

    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        timeout: Duration,
    ) -> crate::McpPeerFuture<'a, JsonRpcResponse> {
        Box::pin(async move {
            let id = request
                .id
                .ok_or_else(|| McpError::Protocol("request id is absent".to_owned()))?;
            let headers = self.request_headers(request)?;
            let response = self
                .client
                .post(self.url.clone())
                .headers(headers)
                .timeout(timeout)
                .json(request)
                .send()
                .await
                .map_err(|error| {
                    if error.is_timeout() {
                        McpError::Timeout(request.method.clone())
                    } else {
                        McpError::Transport(error.to_string())
                    }
                })?;
            if response.status().is_redirection() {
                return Err(McpError::Transport(
                    "cross-origin redirect rejected".to_owned(),
                ));
            }
            if !response.status().is_success() {
                let status = response.status();
                // Authentication and server failures never authorize protocol
                // downgrade, even if their bodies resemble JSON-RPC errors.
                if status.is_client_error() && status.as_u16() != 401 && status.as_u16() != 403 {
                    let mut stream = response.bytes_stream();
                    let mut bytes = Vec::new();
                    while let Some(chunk) = stream.next().await {
                        let Ok(chunk) = chunk else {
                            bytes.clear();
                            break;
                        };
                        if bytes.len() + chunk.len() > 16 * 1024 {
                            bytes.clear();
                            break;
                        }
                        bytes.extend_from_slice(&chunk);
                    }
                    if let Ok(rpc) = serde_json::from_slice::<JsonRpcResponse>(&bytes) {
                        if rpc.error.is_some()
                            && request.id.is_some_and(|id| rpc.validate_for(id).is_ok())
                        {
                            return Ok(rpc);
                        }
                    }
                    // A protocol-version rejection is legitimate legacy
                    // evidence even when the server answers with a
                    // placeholder id (DeepWiki: `"id":"server-error"`, which
                    // is not a JSON-RPC response id at all); it is surfaced as
                    // the remote error, never as a response.
                    if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                        let error = &body["error"];
                        let message = error["message"].as_str().unwrap_or_default();
                        if error["code"].as_i64() == Some(-32600)
                            && message.to_ascii_lowercase().contains("protocol version")
                        {
                            let data = error
                                .get("data")
                                .filter(|data| !data.is_null())
                                .and_then(|data| serde_json::to_vec(data).ok())
                                .and_then(|bytes| IJsonValue::parse(&bytes).ok());
                            return Err(McpError::Remote {
                                code: -32600,
                                message: message.to_owned(),
                                data,
                            });
                        }
                    }
                }
                return Err(McpError::Transport(format!("HTTP MCP returned {status}")));
            }
            if let Some(session_id) = response.headers().get("mcp-session-id") {
                self.session_id = Some(session_id.clone());
            }
            let content_type = response
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("")
                .to_owned();
            if response
                .content_length()
                .is_some_and(|length| length > MAX_FRAME_BYTES as u64)
            {
                return Err(McpError::Protocol("HTTP result exceeds 4 MiB".to_owned()));
            }
            let media_type = content_type.split(';').next().map(str::trim).unwrap_or("");
            let result = if media_type == "text/event-stream" {
                let mut stream = response.bytes_stream();
                let mut decoder = SseDecoder::default();
                let mut result = None;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.map_err(|error| McpError::Transport(error.to_string()))?;
                    for payload in decoder.push(&chunk)? {
                        match decode_inbound(&payload, id, &mut BTreeSet::new())? {
                            Inbound::Response(response) => {
                                result = Some(response);
                                break;
                            }
                            Inbound::Notification(method) => {
                                if let Some(handler) = &self.notification_value_handler {
                                    let notification: serde_json::Value =
                                        serde_json::from_slice(&payload).map_err(|error| {
                                            McpError::Protocol(error.to_string())
                                        })?;
                                    handler(&notification);
                                } else {
                                    if let Some(handler) = &self.notification_handler {
                                        handler(&method);
                                    }
                                    self.notifications.push(method);
                                }
                            }
                            Inbound::LateCancelled => {}
                        }
                    }
                    if result.is_some() {
                        break;
                    }
                }
                result.ok_or_else(|| {
                    McpError::Protocol(
                        "SSE contains no complete matching response event".to_owned(),
                    )
                })
            } else if media_type == "application/json" {
                let mut stream = response.bytes_stream();
                let mut bytes = Vec::new();
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.map_err(|error| McpError::Transport(error.to_string()))?;
                    if bytes.len().saturating_add(chunk.len()) > MAX_FRAME_BYTES {
                        return Err(McpError::Protocol("HTTP result exceeds 4 MiB".to_owned()));
                    }
                    bytes.extend_from_slice(&chunk);
                }
                decode_response(&bytes, id)
            } else {
                Err(McpError::Protocol(format!(
                    "unsupported HTTP MCP content type {media_type:?}"
                )))
            };
            if matches!(result, Err(McpError::Protocol(_))) {
                self.scoped_generation.cancel();
                self.closed = true;
                self.session_id = None;
            }
            result
        })
    }

    fn notify<'a>(&'a mut self, notification: &'a JsonRpcRequest) -> crate::McpPeerFuture<'a, ()> {
        Box::pin(async move {
            if notification.id.is_some() {
                return Err(McpError::Protocol("notification has an id".to_owned()));
            }
            let headers = self.request_headers(notification)?;
            let response = self
                .client
                .post(self.url.clone())
                .headers(headers)
                .timeout(Duration::from_secs(15))
                .json(notification)
                .send()
                .await
                .map_err(|error| McpError::Transport(error.to_string()))?;
            if let Some(session_id) = response.headers().get("mcp-session-id") {
                self.session_id = Some(session_id.clone());
            }
            if response.status().is_success() {
                Ok(())
            } else {
                Err(McpError::Transport(format!(
                    "HTTP MCP notification returned {}",
                    response.status()
                )))
            }
        })
    }

    fn close(&mut self) -> crate::McpPeerFuture<'_, ()> {
        // Resolve current authorization before consuming the session. Identity
        // rotation must never send a previous principal's session to a new one.
        let headers = if !self.closed && self.session_id.is_some() {
            self.request_headers(&JsonRpcRequest::notification("", None))
                .ok()
                .filter(|headers| headers.contains_key("mcp-session-id"))
        } else {
            None
        };
        self.scoped_generation.cancel();
        self.closed = true;
        self.session_id = None;
        self.session_authorization_identity = None;
        Box::pin(async move {
            if let Some(mut headers) = headers {
                headers.insert(
                    HeaderName::from_static("mcp-protocol-version"),
                    HeaderValue::from_static(crate::LEGACY_PROTOCOL_VERSION),
                );
                // Session termination is best-effort, including servers that
                // do not implement DELETE. Local close remains idempotent.
                let _ = self
                    .client
                    .delete(self.url.clone())
                    .headers(headers)
                    .timeout(Duration::from_secs(15))
                    .send()
                    .await;
            }
            Ok(())
        })
    }

    fn reconnect(&mut self) -> crate::McpPeerFuture<'_, ()> {
        self.scoped_generation.cancel();
        self.scoped_generation = crate::McpCancellationToken::default();
        self.closed = false;
        self.session_id = None;
        self.session_authorization_identity = None;
        self.notifications.clear();
        Box::pin(async { Ok(()) })
    }

    fn take_notifications(&mut self) -> Vec<String> {
        std::mem::take(&mut self.notifications)
    }
}

#[derive(Default)]
struct SseDecoder {
    pending: Vec<u8>,
    data: Vec<Vec<u8>>,
    event_bytes: usize,
}

impl SseDecoder {
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, McpError> {
        if self.pending.len().saturating_add(chunk.len()) > MAX_FRAME_BYTES + 2 {
            return Err(McpError::Protocol("SSE event exceeds 4 MiB".to_owned()));
        }
        self.pending.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line = self.pending.drain(..=newline).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                if !self.data.is_empty() {
                    let size = self.data.iter().map(Vec::len).sum::<usize>()
                        + self.data.len().saturating_sub(1);
                    if size > MAX_FRAME_BYTES {
                        return Err(McpError::Protocol("SSE event exceeds 4 MiB".to_owned()));
                    }
                    let mut payload = Vec::with_capacity(size);
                    for (index, value) in self.data.drain(..).enumerate() {
                        if index > 0 {
                            payload.push(b'\n');
                        }
                        payload.extend_from_slice(&value);
                    }
                    self.event_bytes = 0;
                    events.push(payload);
                }
                continue;
            }
            if let Some(value) = line.strip_prefix(b"data:") {
                let value = value.strip_prefix(b" ").unwrap_or(value).to_vec();
                self.event_bytes = self.event_bytes.saturating_add(value.len() + 1);
                if self.event_bytes > MAX_FRAME_BYTES {
                    return Err(McpError::Protocol("SSE event exceeds 4 MiB".to_owned()));
                }
                self.data.push(value);
            }
        }
        Ok(events)
    }
}

enum Inbound {
    Response(JsonRpcResponse),
    Notification(String),
    LateCancelled,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InboundWire {
    jsonrpc: String,
    #[serde(default)]
    id: Option<u64>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    params: Option<IJsonValue>,
    #[serde(default)]
    result: Option<IJsonValue>,
    #[serde(default)]
    error: Option<JsonRpcError>,
}

fn decode_response(bytes: &[u8], expected_id: u64) -> Result<JsonRpcResponse, McpError> {
    match decode_inbound(bytes, expected_id, &mut BTreeSet::new())? {
        Inbound::Response(response) => Ok(response),
        Inbound::Notification(_) | Inbound::LateCancelled => Err(McpError::Protocol(
            "HTTP JSON response did not contain a matching response".to_owned(),
        )),
    }
}

fn decode_inbound(
    bytes: &[u8],
    expected_id: u64,
    cancelled_ids: &mut BTreeSet<u64>,
) -> Result<Inbound, McpError> {
    let frame: InboundWire =
        serde_json::from_slice(bytes).map_err(|error| McpError::Protocol(error.to_string()))?;
    if frame.jsonrpc != "2.0" {
        return Err(McpError::Protocol(
            "JSON-RPC frame version is not 2.0".to_owned(),
        ));
    }
    if let Some(id) = frame.id {
        if id == 0 {
            return Err(McpError::Protocol(
                "response id is not a positive integer".to_owned(),
            ));
        }
        if frame.method.is_some() {
            return Err(McpError::Protocol(
                "server request has no installed client handler".to_owned(),
            ));
        }
        if id != expected_id {
            if cancelled_ids.remove(&id) {
                return Ok(Inbound::LateCancelled);
            }
            return Err(McpError::Protocol(format!(
                "unknown response id {id}; expected {expected_id}"
            )));
        }
        if frame.params.is_some() {
            return Err(McpError::Protocol(
                "response contains notification params".to_owned(),
            ));
        }
        let response = JsonRpcResponse {
            jsonrpc: frame.jsonrpc,
            id,
            result: frame.result,
            error: frame.error,
        };
        response.validate_for(expected_id)?;
        return Ok(Inbound::Response(response));
    }
    if frame.result.is_some() || frame.error.is_some() {
        return Err(McpError::Protocol(
            "id-less frame is not a notification".to_owned(),
        ));
    }
    let method = frame
        .method
        .ok_or_else(|| McpError::Protocol("id-less frame is not a notification".to_owned()))?;
    if matches!(
        method.as_str(),
        "notifications/message"
            | "notifications/progress"
            | "notifications/tools/list_changed"
            | "notifications/prompts/list_changed"
            | "notifications/resources/list_changed"
            | "notifications/resources/updated"
            | "notifications/tasks/status"
            | "notifications/subscriptions/acknowledged"
    ) {
        Ok(Inbound::Notification(method))
    } else {
        Err(McpError::Protocol(format!(
            "unknown required notification {method}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn owned_scoped_requests_cannot_outlive_close_or_reconnect() {
        for reconnect in [false, true] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let received = Arc::new(tokio::sync::Notify::new());
            let signal = received.clone();
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut bytes = Vec::new();
                loop {
                    let mut chunk = [0; 4096];
                    let n = socket.read(&mut chunk).await.unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&chunk[..n]);
                    if let Some(offset) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..offset]).to_ascii_lowercase();
                        let length: usize = header
                            .lines()
                            .find_map(|line| line.strip_prefix("content-length:"))
                            .unwrap()
                            .trim()
                            .parse()
                            .unwrap();
                        if bytes.len() >= offset + 4 + length {
                            break;
                        }
                    }
                }
                signal.notify_one();
                let mut byte = [0];
                assert_eq!(
                    tokio::time::timeout(Duration::from_secs(3), socket.read(&mut byte))
                        .await
                        .unwrap()
                        .unwrap(),
                    0
                );
                // An old request created before rotation must never dispatch
                // even when it is first polled after the new generation exists.
                assert!(
                    tokio::time::timeout(Duration::from_millis(100), listener.accept())
                        .await
                        .is_err()
                );
            });
            let mut transport = HttpTransport::new(
                format!("http://{address}/mcp").parse().unwrap(),
                &BTreeMap::new(),
                None,
                true,
            )
            .unwrap();
            let request: JsonRpcRequest = serde_json::from_value(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"write","arguments":{},"_meta":{"io.modelcontextprotocol/protocolVersion":crate::MODERN_PROTOCOL_VERSION}}})).unwrap();
            let active = transport.start_modern_scoped(
                request.clone(),
                Duration::from_secs(5),
                crate::McpCancellationToken::default(),
                None,
            );
            let delayed = transport.start_modern_scoped(
                request,
                Duration::from_secs(5),
                crate::McpCancellationToken::default(),
                None,
            );
            let (result, ()) = tokio::join!(active, async {
                received.notified().await;
                if reconnect {
                    transport.reconnect().await.unwrap();
                } else {
                    transport.close().await.unwrap();
                }
            });
            assert!(matches!(result, Err(McpError::Cancelled)));
            assert!(matches!(delayed.await, Err(McpError::Cancelled)));
            server.await.unwrap();
        }
    }

    #[test]
    fn duplicate_json_members_fail_before_value_collapse() {
        let duplicate_top = br#"{"id":1,"id":1,"jsonrpc":"2.0","result":{"ok":true}}"#;
        assert!(matches!(
            decode_inbound(duplicate_top, 1, &mut BTreeSet::new()),
            Err(McpError::Protocol(_))
        ));
        let duplicate_nested = br#"{"id":1,"jsonrpc":"2.0","result":{"ok":true,"ok":false}}"#;
        assert!(matches!(
            decode_inbound(duplicate_nested, 1, &mut BTreeSet::new()),
            Err(McpError::Protocol(_))
        ));
    }

    #[test]
    fn cancelled_response_tombstone_is_consumed_once() {
        let mut cancelled = BTreeSet::from([7]);
        assert!(matches!(
            decode_inbound(
                br#"{"id":7,"jsonrpc":"2.0","result":{}}"#,
                8,
                &mut cancelled,
            ),
            Ok(Inbound::LateCancelled)
        ));
        assert!(cancelled.is_empty());
        assert!(matches!(
            decode_inbound(
                br#"{"id":8,"jsonrpc":"2.0","result":{}}"#,
                8,
                &mut cancelled,
            ),
            Ok(Inbound::Response(_))
        ));
    }

    #[test]
    fn sse_decoder_yields_the_first_complete_event_without_eof() {
        let mut decoder = SseDecoder::default();
        assert!(
            decoder
                .push(b"data: {\"id\":1,\"jsonrpc\":\"2.0\",")
                .expect("partial chunk")
                .is_empty()
        );
        let events = decoder
            .push(b"\"result\":{}}\n\n: connection remains open\n")
            .expect("complete event");
        assert_eq!(events.len(), 1);
        assert!(matches!(
            decode_inbound(&events[0], 1, &mut BTreeSet::new()),
            Ok(Inbound::Response(_))
        ));
    }

    #[test]
    fn timed_out_detached_reaper_does_not_delay_runtime_drop() {
        let started = std::time::Instant::now();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        runtime.block_on(async {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(3));
                let _ = sender.send(Ok(()));
            });
            assert!(matches!(
                await_stdio_reaper(receiver).await,
                Err(McpError::Transport(_))
            ));
        });
        drop(runtime);
        assert!(
            started.elapsed() < Duration::from_millis(1_500),
            "Tokio runtime drop waited for a detached reaper"
        );
    }
}
