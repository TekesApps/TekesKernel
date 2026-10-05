use std::collections::{BTreeMap, BTreeSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::Body;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Request, State};
use axum::http::header::{CONTENT_TYPE, ORIGIN};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use endpoint::{
    CallContext, ClientRequest, DrainSignal, DurableHandoffSignal, EndpointCarrierHost,
    HostFailure, HostReadiness, MethodClass, RpcError, RpcResult, ServerResponse,
    SessionErrorCategory, SessionMuxClientFrame, SessionMuxGeneration, SessionMuxServerFrame,
    SessionRemoteError, StreamErrorCode, validate_response, validate_rpc_id,
};
use schema::IJsonValue;
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{OwnedSemaphorePermit, Semaphore, mpsc, watch};
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};

use crate::web::{
    APP_CSS, APP_JS, BrowserAccess, index_response, index_unauthorized, static_response,
};
use crate::{AccessLogRecord, AccessLogSink, BearerToken};

pub const MAX_REQUEST_BYTES: usize = 16 * 1024 * 1024;

/// Briefly exposes typed `server-draining` responses before Axum stops accepting new
/// connections. Already-admitted work keeps its ordinary response timeout because Axum's
/// graceful shutdown waits for existing connections after this announcement interval.
const DRAIN_ANNOUNCEMENT_INTERVAL: Duration = Duration::from_millis(100);

pub const ROUTE_REGISTRY: [&str; 17] = [
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportLimits {
    pub concurrent_unary: usize,
    pub concurrent_websockets: usize,
    pub subscriptions_per_socket: usize,
    pub body_timeout: Duration,
    pub response_timeout: Duration,
}

impl Default for TransportLimits {
    fn default() -> Self {
        Self {
            concurrent_unary: 128,
            concurrent_websockets: 32,
            subscriptions_per_socket: 256,
            body_timeout: Duration::from_secs(15),
            response_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OriginPolicy;

impl OriginPolicy {
    fn permits(&self, headers: &HeaderMap) -> bool {
        !headers.contains_key(ORIGIN)
    }
}

#[derive(Clone)]
pub struct TransportConfig {
    pub bind: SocketAddr,
    pub limits: TransportLimits,
    pub origins: OriginPolicy,
    bearer_token: BearerToken,
    access_log: Arc<dyn AccessLogSink>,
    readiness_identity: Option<ReadinessIdentity>,
}

#[derive(Clone)]
struct ReadinessIdentity {
    build: String,
    generation: u64,
}

impl TransportConfig {
    #[must_use]
    pub fn loopback(bind: SocketAddr, bearer_token: BearerToken) -> Self {
        Self {
            bind,
            limits: TransportLimits::default(),
            origins: OriginPolicy,
            bearer_token,
            access_log: crate::access::noop(),
            readiness_identity: None,
        }
    }

    #[must_use]
    pub fn with_access_log(mut self, sink: Arc<dyn AccessLogSink>) -> Self {
        self.access_log = sink;
        self
    }

    /// Enables the deployment health representation from the immutable
    /// selector handoff. Slice 9 carrier-only callers keep their legacy empty
    /// health responses until a production identity is supplied.
    #[must_use]
    pub fn with_readiness_identity(mut self, build: impl Into<String>, generation: u64) -> Self {
        self.readiness_identity = Some(ReadinessIdentity {
            build: build.into(),
            generation,
        });
        self
    }

    pub fn validate(&self) -> Result<(), TransportConfigError> {
        if !self.bind.ip().is_loopback() {
            return Err(TransportConfigError::NonLoopback(self.bind.ip()));
        }
        if self
            .readiness_identity
            .as_ref()
            .is_some_and(|identity| identity.build.is_empty() || identity.generation == 0)
        {
            return Err(TransportConfigError::InvalidReadinessIdentity);
        }
        let limits = self.limits;
        if limits.concurrent_unary != 128
            || limits.concurrent_websockets != 32
            || limits.subscriptions_per_socket != 256
            || limits.body_timeout != Duration::from_secs(15)
            || limits.response_timeout != Duration::from_secs(30)
        {
            return Err(TransportConfigError::UnversionedLimitOverride);
        }
        Ok(())
    }
}

struct Shared {
    file_changes: Option<Arc<dyn FileChangeAuthority>>,
    file_transfer: Option<Arc<dyn FileTransferAuthority>>,
    host: Arc<dyn EndpointCarrierHost>,
    config: TransportConfig,
    unary: Arc<Semaphore>,
    websockets: Arc<Semaphore>,
    draining: Arc<AtomicBool>,
    drain: watch::Sender<bool>,
    next_transport_request: AtomicU64,
    next_mux_generation: AtomicU64,
    extension_routes: BTreeSet<String>,
}

struct WebShared {
    endpoint: Arc<Shared>,
    browser: BrowserAccess,
}

struct AccessSpan<'a> {
    shared: &'a Shared,
    started: Instant,
    request_id: String,
    operation: String,
    path: String,
}

impl<'a> AccessSpan<'a> {
    fn new(shared: &'a Shared, operation: &str, path: &str) -> Self {
        let ordinal = shared
            .next_transport_request
            .fetch_add(1, Ordering::Relaxed);
        Self {
            shared,
            started: Instant::now(),
            request_id: format!("transport-{ordinal}"),
            operation: operation.to_owned(),
            path: path.to_owned(),
        }
    }

    fn bind_rpc_id(&mut self, rpc_id: &str) {
        self.request_id = rpc_id.to_owned();
    }

    fn finish(&self, response: Response, error_code: Option<&str>) -> Response {
        let elapsed = self.started.elapsed().as_millis();
        let elapsed_ms = u64::try_from(elapsed)
            .unwrap_or(9_007_199_254_740_991)
            .min(9_007_199_254_740_991);
        self.shared.config.access_log.record(&AccessLogRecord {
            v: 1,
            request_id: self.request_id.clone(),
            operation: self.operation.clone(),
            path: self.path.clone(),
            status: response.status().as_u16(),
            elapsed_ms,
            error_code: error_code.map(str::to_owned),
        });
        response
    }
}

#[derive(Clone)]
pub struct TransportHandle {
    shared: Arc<Shared>,
}

impl TransportHandle {
    /// Makes readiness fail before notifying handlers and stream connections.
    pub fn begin_drain(&self) {
        self.shared.draining.store(true, Ordering::Release);
        self.shared.drain.send_replace(true);
    }

    #[must_use]
    pub fn is_draining(&self) -> bool {
        self.shared.draining.load(Ordering::Acquire)
    }
}

pub struct TransportServer {
    shared: Arc<Shared>,
}

/// File authority remains with the host; the carrier supplies authenticated,
/// bounded HTTP framing for immutable preview leases.
pub trait FileTransferAuthority: Send + Sync + 'static {
    fn prepare(&self, request: serde_json::Value) -> Result<serde_json::Value, String>;
    fn read(&self, lease: &str) -> Option<Arc<[u8]>>;
    fn release(&self, lease: &str);
}

pub trait FileChangeFeed: Send + 'static {
    fn poll(&mut self) -> Result<Vec<serde_json::Value>, String>;
}

pub trait FileChangeAuthority: Send + Sync + 'static {
    fn open(&self, session_id: &str) -> Result<Box<dyn FileChangeFeed>, String>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebClientConfig {
    pub bind: SocketAddr,
}

impl WebClientConfig {
    #[must_use]
    pub const fn loopback(bind: SocketAddr) -> Self {
        Self { bind }
    }

    fn validate(&self) -> Result<(), TransportConfigError> {
        if !self.bind.ip().is_loopback() {
            return Err(TransportConfigError::NonLoopback(self.bind.ip()));
        }
        Ok(())
    }
}

/// Optional browser-facing service over the already assembled endpoint host.
///
/// Constructing a `TransportServer` never enables this service. The owner must
/// explicitly create a `WebClientService`, bind its separate listener, and
/// drive its future. The Session Endpoint listener and route registry remain
/// unchanged when the Web Client is disabled.
pub struct WebClientService {
    shared: Arc<WebShared>,
    config: WebClientConfig,
}

impl TransportServer {
    pub fn new(
        host: Arc<dyn EndpointCarrierHost>,
        config: TransportConfig,
    ) -> Result<Self, TransportConfigError> {
        config.validate()?;
        let expected: BTreeSet<String> = ROUTE_REGISTRY.iter().map(ToString::to_string).collect();
        let extensions = host.advertised_extension_methods();
        if let Some(method) = expected.intersection(&extensions).next() {
            return Err(TransportConfigError::ExtensionCollision(method.clone()));
        }
        if let Some(method) = extensions
            .iter()
            .find(|method| host.classify_method(method) == MethodClass::Unknown)
        {
            return Err(TransportConfigError::InvalidExtensionRegistration(
                method.clone(),
            ));
        }
        let expected = expected
            .union(&extensions)
            .cloned()
            .collect::<BTreeSet<_>>();
        let registered = host.registered_methods();
        if registered != expected {
            return Err(TransportConfigError::RegistryMismatch {
                missing: expected.difference(&registered).cloned().collect(),
                unexpected: registered.difference(&expected).cloned().collect(),
            });
        }
        Ok(Self {
            shared: Arc::new(Shared {
                file_transfer: None,
                file_changes: None,
                unary: Arc::new(Semaphore::new(config.limits.concurrent_unary)),
                websockets: Arc::new(Semaphore::new(config.limits.concurrent_websockets)),
                host,
                config,
                draining: Arc::new(AtomicBool::new(false)),
                drain: watch::channel(false).0,
                next_transport_request: AtomicU64::new(1),
                next_mux_generation: AtomicU64::new(1),
                extension_routes: extensions,
            }),
        })
    }

    #[must_use]
    pub fn with_file_transfer(mut self, authority: Arc<dyn FileTransferAuthority>) -> Self {
        Arc::get_mut(&mut self.shared)
            .expect("Configure transfer before sharing server")
            .file_transfer = Some(authority);
        self
    }

    pub fn set_file_changes(&mut self, authority: Arc<dyn FileChangeAuthority>) {
        Arc::get_mut(&mut self.shared)
            .expect("Configure observations before sharing server")
            .file_changes = Some(authority);
    }

    #[must_use]
    pub fn handle(&self) -> TransportHandle {
        TransportHandle {
            shared: Arc::clone(&self.shared),
        }
    }

    pub fn router(&self) -> Router {
        let state = Arc::clone(&self.shared);
        Router::new()
            .route("/health/live", get(live))
            .route("/health/ready", get(ready))
            .route("/api/remote.mux", get(remote_mux))
            .route("/api/session.files.changes", get(file_changes))
            .route(
                "/api/tekesWorkspace.fileTransfer/prepare",
                post(file_prepare),
            )
            .route("/api/tekesWorkspace.fileTransfer/read", get(file_read))
            .route(
                "/api/tekesWorkspace.fileTransfer/release",
                post(file_release),
            )
            .route("/api/{*method}", post(unary))
            .method_not_allowed_fallback(method_not_allowed)
            .fallback(fallback)
            .with_state(Arc::clone(&state))
            .layer(middleware::from_fn_with_state(state, request_security))
    }

    pub fn web_client(
        &self,
        config: WebClientConfig,
    ) -> Result<WebClientService, TransportConfigError> {
        config.validate()?;
        Ok(WebClientService {
            shared: Arc::new(WebShared {
                endpoint: Arc::clone(&self.shared),
                browser: BrowserAccess::new(config.bind),
            }),
            config,
        })
    }

    pub async fn serve(self, listener: TcpListener) -> Result<(), TransportConfigError> {
        let actual = listener.local_addr()?;
        if !actual.ip().is_loopback() {
            return Err(TransportConfigError::NonLoopback(actual.ip()));
        }
        let configured = self.shared.config.bind;
        if actual.ip() != configured.ip()
            || (configured.port() != 0 && actual.port() != configured.port())
        {
            return Err(TransportConfigError::ListenerAddressMismatch { configured, actual });
        }
        let router = self.router();
        let shared = Arc::clone(&self.shared);
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let mut drain = shared.drain.subscribe();
                while !*drain.borrow() {
                    if drain.changed().await.is_err() {
                        return;
                    }
                }
                // Keep the listener briefly available so newly arriving mutations/upgrades can
                // receive the typed drain response. Do not sleep for `response_timeout` here:
                // that unconditionally kept an idle Kernel alive for 30 seconds on every quit.
                sleep(DRAIN_ANNOUNCEMENT_INTERVAL).await;
            })
            .await?;
        Ok(())
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FileChangesQuery {
    session_id: String,
}

async fn file_changes(
    State(shared): State<Arc<Shared>>,
    axum::extract::Query(query): axum::extract::Query<FileChangesQuery>,
    upgrade: WebSocketUpgrade,
) -> Response {
    let Some(authority) = shared.file_changes.clone() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if shared.draining.load(Ordering::Acquire) || shared.host.readiness() != HostReadiness::Ready {
        return unavailable(&shared);
    }
    let permit = match acquire(&shared.websockets) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let feed = match tokio::task::spawn_blocking(move || authority.open(&query.session_id)).await {
        Ok(Ok(feed)) => feed,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    upgrade.on_upgrade(move |mut socket| async move {
        let _permit = permit;
        let mut feed = feed;
        let mut drain = shared.drain.subscribe();
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            tokio::select! {
                _ = drain.changed() => break,
                message = socket.recv() => match message {
                    Some(Ok(Message::Ping(bytes))) => { if socket.send(Message::Pong(bytes)).await.is_err() { break; } },
                    Some(Ok(Message::Pong(_))) => {},
                    _ => break,
                },
                _ = interval.tick() => {
                    let result = tokio::task::spawn_blocking(move || { let result = feed.poll(); (feed, result) }).await;
                    let Ok((returned, Ok(frames))) = result else { break; };
                    feed = returned;
                    let mut failed = false;
                    for frame in frames {
                        let Ok(encoded) = serde_json::to_string(&frame) else { failed = true; break; };
                        if socket.send(Message::Text(encoded.into())).await.is_err() { failed = true; break; }
                    }
                    if failed { break; }
                }
            }
        }
        let _ = socket.send(Message::Close(None)).await;
    }).into_response()
}

async fn file_prepare(State(shared): State<Arc<Shared>>, request: Request) -> Response {
    let Some(authority) = shared.file_transfer.clone() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if shared.draining.load(Ordering::Acquire) || shared.host.readiness() != HostReadiness::Ready {
        return unavailable(&shared);
    }
    let permit = match acquire(&shared.unary) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let bytes = match timeout(
        shared.config.limits.body_timeout,
        axum::body::to_bytes(request.into_body(), 16384),
    )
    .await
    {
        Ok(Ok(bytes)) => bytes,
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };
    let payload = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    // Keep the admission permit with blocking work even if the HTTP caller exits.
    let work = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        authority.prepare(payload)
    });
    match timeout(shared.config.limits.response_timeout, work).await {
        Ok(Ok(Ok(value))) => canonical_json(StatusCode::OK, &value),
        Ok(Ok(Err(_))) => StatusCode::BAD_REQUEST.into_response(),
        _ => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

fn file_lease(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("X-Tekes-File-Lease")?
        .to_str()
        .ok()
        .filter(|id| {
            id.len() == 43
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        })
}

async fn file_read(State(shared): State<Arc<Shared>>, headers: HeaderMap) -> Response {
    let Some(authority) = &shared.file_transfer else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(lease) = file_lease(&headers) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    match authority.read(lease) {
        // Bytes retains the immutable lease allocation until the response ends.
        // Concurrent readers must not each duplicate a complete snapshot.
        Some(bytes) => (
            [
                (CONTENT_TYPE, "application/octet-stream"),
                (axum::http::header::CACHE_CONTROL, "no-store"),
            ],
            axum::body::Bytes::from_owner(bytes),
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn file_release(State(shared): State<Arc<Shared>>, headers: HeaderMap) -> Response {
    let Some(authority) = &shared.file_transfer else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(lease) = file_lease(&headers) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    authority.release(lease);
    StatusCode::NO_CONTENT.into_response()
}

impl WebClientService {
    pub fn router(&self) -> Router {
        let state = Arc::clone(&self.shared);
        Router::new()
            .route("/", get(web_index))
            .route("/index.html", get(web_index))
            .route("/web/assets/app.css", get(web_css))
            .route("/web/assets/app.js", get(web_js))
            .route("/web/remote.mux", get(web_remote_mux))
            .route("/web/api/{*method}", post(web_unary))
            .method_not_allowed_fallback(method_not_allowed)
            .fallback(fallback)
            .with_state(Arc::clone(&state))
            .layer(middleware::from_fn_with_state(state, web_request_security))
    }

    pub async fn serve(self, listener: TcpListener) -> Result<(), TransportConfigError> {
        let actual = listener.local_addr()?;
        if !actual.ip().is_loopback() {
            return Err(TransportConfigError::NonLoopback(actual.ip()));
        }
        let configured = self.config.bind;
        if actual.ip() != configured.ip()
            || (configured.port() != 0 && actual.port() != configured.port())
        {
            return Err(TransportConfigError::ListenerAddressMismatch { configured, actual });
        }
        let router = self.router();
        let endpoint = Arc::clone(&self.shared.endpoint);
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let mut drain = endpoint.drain.subscribe();
                while !*drain.borrow() {
                    if drain.changed().await.is_err() {
                        return;
                    }
                }
                sleep(DRAIN_ANNOUNCEMENT_INTERVAL).await;
            })
            .await?;
        Ok(())
    }
}

async fn web_index(State(shared): State<Arc<WebShared>>, request: Request) -> Response {
    if shared.browser.permits_host(request.headers()) {
        index_response()
    } else {
        index_unauthorized()
    }
}

async fn web_css() -> Response {
    static_response("text/css; charset=utf-8", APP_CSS)
}

async fn web_js() -> Response {
    static_response("text/javascript; charset=utf-8", APP_JS)
}

async fn live(State(shared): State<Arc<Shared>>) -> Response {
    if shared.config.readiness_identity.is_some() {
        canonical_json(StatusCode::OK, &serde_json::json!({"live":true}))
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}

async fn ready(State(shared): State<Arc<Shared>>) -> Response {
    if shared.draining.load(Ordering::Acquire) {
        readiness_error(&shared, "server-draining", "Endpoint is draining")
    } else if shared.host.readiness() != HostReadiness::Ready {
        if shared.config.readiness_identity.is_some() {
            readiness_error(&shared, "boot-incomplete", "Startup has not completed")
        } else {
            carrier_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "not-ready",
                "Endpoint is not ready",
            )
        }
    } else if let Some(identity) = shared.config.readiness_identity.as_ref() {
        canonical_json(
            StatusCode::OK,
            &serde_json::json!({
                "build": identity.build,
                "generation": identity.generation,
                "ready": true,
            }),
        )
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}

fn readiness_error(shared: &Shared, code: &'static str, message: &'static str) -> Response {
    if shared.config.readiness_identity.is_some() {
        canonical_json(
            StatusCode::SERVICE_UNAVAILABLE,
            &serde_json::json!({
                "ready": false,
                "error": {"code": code, "message": message, "details": {}},
            }),
        )
    } else {
        carrier_error(StatusCode::SERVICE_UNAVAILABLE, code, message)
    }
}

fn canonical_json(status: StatusCode, value: &serde_json::Value) -> Response {
    let bytes = serde_json_canonicalizer::to_vec(value)
        .expect("deployment health literals are canonicalizable");
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .header(axum::http::header::CONTENT_LENGTH, bytes.len().to_string())
        .body(Body::from(bytes))
        .expect("static deployment health response is valid")
}

async fn unary(
    State(shared): State<Arc<Shared>>,
    Path(path_method): Path<String>,
    request: Request,
) -> Response {
    unary_inner(shared, path_method, request).await
}

async fn web_unary(
    State(shared): State<Arc<WebShared>>,
    Path(path_method): Path<String>,
    request: Request,
) -> Response {
    unary_inner(Arc::clone(&shared.endpoint), path_method, request).await
}

async fn unary_inner(shared: Arc<Shared>, path_method: String, request: Request) -> Response {
    let path = request.uri().path().to_owned();
    let mut access = AccessSpan::new(&shared, &path_method, &path);
    let permit = match acquire(&shared.unary) {
        Ok(permit) => permit,
        Err(response) => return access.finish(response, Some("overloaded")),
    };
    if shared.host.readiness() != HostReadiness::Ready {
        let code = unavailable_code(&shared);
        return access.finish(unavailable(&shared), Some(code));
    }
    if !ROUTE_REGISTRY.contains(&path_method.as_str())
        && !shared.extension_routes.contains(&path_method)
    {
        return access.finish(
            carrier_error(StatusCode::NOT_FOUND, "not-found", "Path not found"),
            Some("not-found"),
        );
    }
    let body = match read_json_body(&shared, request).await {
        Ok(body) => body,
        Err(response) => {
            let code = carrier_error_code(response.status());
            return access.finish(response, Some(code));
        }
    };
    let request: ClientRequest = match decode_strict(&body) {
        Ok(request) => request,
        Err(_) => {
            return access.finish(
                carrier_error(
                    StatusCode::BAD_REQUEST,
                    "invalid-request",
                    "Request is malformed",
                ),
                Some("invalid-request"),
            );
        }
    };
    access.bind_rpc_id(&request.rpc_id);
    if validate_rpc_id(&request.rpc_id).is_err()
        || request.envelope_type != "client-request"
        || request.method != path_method
    {
        return access.finish(
            carrier_error(
                StatusCode::BAD_REQUEST,
                "invalid-request",
                "Request is malformed",
            ),
            Some("invalid-request"),
        );
    }
    if shared.draining.load(Ordering::Acquire)
        && shared.host.classify_method(&request.method) != MethodClass::ReadOnly
    {
        let code = unavailable_code(&shared);
        return access.finish(unavailable(&shared), Some(code));
    }
    let handoff = DurableHandoffSignal::new();
    let context = call_context(&shared, handoff.clone());
    let result = timeout(
        shared.config.limits.response_timeout,
        shared.host.unary(request.clone(), context),
    )
    .await;
    drop(permit);
    match result {
        Ok(Ok(response)) if validate_response(&response, &request.rpc_id).is_ok() => {
            let code = response
                .result
                .error
                .as_ref()
                .map(|error| error.code.as_str());
            access.finish(json_response(&response), code)
        }
        Ok(Err(failure)) => {
            let (response, code) = host_failure(&shared, failure);
            access.finish(response, Some(code))
        }
        Err(_) if handoff.is_durable() => access.finish(
            json_response(&accepted_but_not_confirmed(
                &request.rpc_id,
                &request.method,
            )),
            Some("accepted-but-not-confirmed"),
        ),
        Ok(Ok(_)) | Err(_) => {
            let code = unavailable_code(&shared);
            access.finish(unavailable(&shared), Some(code))
        }
    }
}

async fn remote_mux(State(shared): State<Arc<Shared>>, upgrade: WebSocketUpgrade) -> Response {
    remote_mux_inner(shared, upgrade).await
}

async fn web_remote_mux(
    State(shared): State<Arc<WebShared>>,
    upgrade: WebSocketUpgrade,
) -> Response {
    remote_mux_inner(Arc::clone(&shared.endpoint), upgrade).await
}

async fn remote_mux_inner(shared: Arc<Shared>, upgrade: WebSocketUpgrade) -> Response {
    if shared.draining.load(Ordering::Acquire) || shared.host.readiness() != HostReadiness::Ready {
        return unavailable(&shared);
    }
    let permit = match acquire(&shared.websockets) {
        Ok(permit) => permit,
        Err(response) => return response,
    };
    let generation = shared.next_mux_generation.fetch_add(1, Ordering::Relaxed);
    let Ok(state) = SessionMuxGeneration::new(generation) else {
        return unavailable(&shared);
    };
    let host = match shared.host.mux_description() {
        Ok(host) if host.validate().is_ok() => host,
        _ => return unavailable(&shared),
    };
    upgrade
        .max_message_size(MAX_REQUEST_BYTES + 1)
        .max_frame_size(MAX_REQUEST_BYTES + 1)
        .on_upgrade(move |socket| remote_mux_loop(shared, socket, permit, state, host))
}

async fn remote_mux_loop(
    shared: Arc<Shared>,
    mut socket: WebSocket,
    _permit: OwnedSemaphorePermit,
    mut generation: SessionMuxGeneration,
    host: endpoint::MuxHostDescription,
) {
    let ready = SessionMuxServerFrame::Ready {
        generation: generation.generation(),
        host,
    };
    if send_mux(&mut socket, &ready).await.is_err() {
        return;
    }
    let (stream_tx, mut stream_rx) = mpsc::channel(1_024);
    let mut streams = BTreeMap::<String, JoinHandle<()>>::new();
    let mut drain = shared.drain.subscribe();
    loop {
        tokio::select! {
            biased;
            changed = drain.changed() => {
                if changed.is_err() { return; }
                let _ = send_mux_error(
                    &mut socket,
                    None,
                    None,
                    SessionErrorCategory::Transport,
                    "server-draining",
                    "Endpoint is draining",
                ).await;
                let _ = socket.send(Message::Close(Some(CloseFrame {
                    code: StreamErrorCode::ServerDraining.close_code(),
                    reason: "server-draining".into(),
                }))).await;
                return;
            }
            incoming = socket.recv() => {
                let Some(incoming) = incoming else { return; };
                let frame = match incoming {
                    Ok(Message::Text(text)) if text.len() <= MAX_REQUEST_BYTES => {
                        match decode_strict::<SessionMuxClientFrame>(text.as_bytes()) {
                            Ok(frame) if frame.validate().is_ok() => frame,
                            _ => {
                                close_mux_protocol(&mut socket, "invalid V3 frame").await;
                                return;
                            }
                        }
                    }
                    Ok(Message::Ping(value)) => {
                        if socket.send(Message::Pong(value)).await.is_err() { return; }
                        continue;
                    }
                    Ok(Message::Pong(_)) => continue,
                    Ok(Message::Close(_)) => return,
                    Ok(Message::Text(_)) | Ok(Message::Binary(_)) => {
                        close_mux_protocol(&mut socket, "invalid V3 frame").await;
                        return;
                    }
                    Err(_) => return,
                };
                match frame {
                    SessionMuxClientFrame::Open { stream_id, target } => {
                        if streams.len() >= shared.config.limits.subscriptions_per_socket
                            || generation.open(stream_id.clone(), &target).is_err()
                        {
                            close_mux_protocol(&mut socket, "duplicate or excessive V3 stream").await;
                            return;
                        }
                        let opened = timeout(
                            shared.config.limits.response_timeout,
                            shared.host.open_mux_stream(generation.generation(), target),
                        ).await;
                        let Ok(Ok(mut receiver)) = opened else {
                            let _ = generation.close(&stream_id);
                            let _ = send_mux_error(
                                &mut socket,
                                None,
                                Some(stream_id),
                                SessionErrorCategory::Internal,
                                "stream-open-failed",
                                "Unable to open semantic stream",
                            ).await;
                            continue;
                        };
                        let tx = stream_tx.clone();
                        let task_stream_id = stream_id.clone();
                        let task = tokio::spawn(async move {
                            while let Some(frame) = receiver.recv().await {
                                if tx.send((task_stream_id.clone(), frame)).await.is_err() {
                                    return;
                                }
                            }
                        });
                        streams.insert(stream_id, task);
                    }
                    SessionMuxClientFrame::Close { stream_id } => {
                        let Some(task) = streams.remove(&stream_id) else {
                            close_mux_protocol(&mut socket, "unknown V3 stream").await;
                            return;
                        };
                        task.abort();
                        let _ = generation.close(&stream_id);
                    }
                    ref request @ SessionMuxClientFrame::JournalPage { ref request_id, .. } => {
                        let request_id = request_id.clone();
                        let result = timeout(
                            shared.config.limits.response_timeout,
                            shared.host.journal_page(request.clone()),
                        ).await;
                        match result {
                            Ok(Ok(page)) => {
                                if send_mux(&mut socket, &SessionMuxServerFrame::JournalPageResult {
                                    request_id,
                                    page,
                                }).await.is_err() { return; }
                            }
                            _ => {
                                let _ = send_mux_error(
                                    &mut socket,
                                    Some(request_id),
                                    None,
                                    SessionErrorCategory::InvalidRequest,
                                    "journal-page-failed",
                                    "Unable to read the requested frozen journal page",
                                ).await;
                            }
                        }
                    }
                    ref request @ SessionMuxClientFrame::ActionableRespond {
                        ref request_id,
                        ref actionable_id,
                        expected_revision,
                        ..
                    } => {
                        let request_id = request_id.clone();
                        let actionable_id = actionable_id.clone();
                        let context = call_context(&shared, DurableHandoffSignal::new());
                        let result = timeout(
                            shared.config.limits.response_timeout,
                            shared.host.mux_respond_actionable(request.clone(), context),
                        ).await;
                        // Keep the host failure available to local diagnosis.
                        // The public conflict envelope alone cannot distinguish
                        // a stale revision from an author/delivery failure.
                        match &result {
                            Ok(Err(error)) => eprintln!("mux-actionable-response-failed: request={request_id} revision={expected_revision} error={error}"),
                            Err(_) => eprintln!("mux-actionable-response-timeout: request={request_id} revision={expected_revision}"),
                            _ => {},
                        }
                        match result {
                            Ok(Ok(revision)) => {
                                if send_mux(&mut socket, &SessionMuxServerFrame::ActionableResponseResult {
                                    request_id,
                                    actionable_id,
                                    revision,
                                }).await.is_err() { return; }
                            }
                            _ => {
                                let _ = send_mux_error(
                                    &mut socket,
                                    Some(request_id),
                                    None,
                                    SessionErrorCategory::Conflict,
                                    "actionable-response-rejected",
                                    &format!("Actionable revision {expected_revision} is stale or resolved"),
                                ).await;
                            }
                        }
                    }
                }
            }
            streamed = stream_rx.recv() => {
                let Some((stream_id, streamed)) = streamed else { return; };
                let frame = match streamed {
                    Ok(frame) if generation.accept(&stream_id, &frame).is_ok() => frame,
                    Ok(_) => {
                        close_mux_protocol(&mut socket, "semantic stream violated baseline or sequence").await;
                        return;
                    }
                    Err(failure) => {
                        let _ = send_mux_error(
                            &mut socket,
                            None,
                            Some(stream_id.clone()),
                            SessionErrorCategory::Transport,
                            failure.code().code(),
                            "Semantic stream ended",
                        ).await;
                        if let Some(task) = streams.remove(&stream_id) { task.abort(); }
                        let _ = generation.close(&stream_id);
                        continue;
                    }
                };
                if send_mux(&mut socket, &SessionMuxServerFrame::Stream {
                    stream_id,
                    frame,
                }).await.is_err() { return; }
            }
        }
    }
}

async fn send_mux(socket: &mut WebSocket, frame: &SessionMuxServerFrame) -> Result<(), ()> {
    for frame in endpoint::client_mux_frames(frame).map_err(|_| ())? {
        let bytes = serde_json_canonicalizer::to_vec(&frame).map_err(|_| ())?;
        if bytes.len() > MAX_REQUEST_BYTES {
            return Err(());
        }
        let text = String::from_utf8(bytes).map_err(|_| ())?;
        socket
            .send(Message::Text(text.into()))
            .await
            .map_err(|_| ())?;
    }
    Ok(())
}

async fn send_mux_error(
    socket: &mut WebSocket,
    request_id: Option<String>,
    stream_id: Option<String>,
    category: SessionErrorCategory,
    code: &str,
    message: &str,
) -> Result<(), ()> {
    send_mux(
        socket,
        &SessionMuxServerFrame::Error {
            request_id,
            stream_id,
            error: SessionRemoteError {
                category,
                code: code.to_owned(),
                message: message.to_owned(),
                details: IJsonValue::parse_str("{}").map_err(|_| ())?,
                retryability: None,
            },
        },
    )
    .await
}

async fn close_mux_protocol(socket: &mut WebSocket, message: &str) {
    let _ = send_mux_error(
        socket,
        None,
        None,
        SessionErrorCategory::Transport,
        "protocol-error",
        message,
    )
    .await;
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: StreamErrorCode::ProtocolError.close_code(),
            reason: "protocol-error".into(),
        })))
        .await;
}

fn acquire(semaphore: &Arc<Semaphore>) -> Result<OwnedSemaphorePermit, Response> {
    Arc::clone(semaphore).try_acquire_owned().map_err(|_| {
        carrier_error(
            StatusCode::TOO_MANY_REQUESTS,
            "overloaded",
            "Endpoint admission limit reached",
        )
    })
}

async fn read_json_body(shared: &Shared, request: Request) -> Result<Vec<u8>, Response> {
    if !json_content_type(request.headers()) {
        return Err(carrier_error(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "unsupported-media-type",
            "Content-Type must be application/json",
        ));
    }
    let body = timeout(
        shared.config.limits.body_timeout,
        axum::body::to_bytes(request.into_body(), MAX_REQUEST_BYTES),
    )
    .await;
    match body {
        Ok(Ok(bytes)) => Ok(bytes.to_vec()),
        Ok(Err(_)) => Err(carrier_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "payload-too-large",
            "Request body is too large",
        )),
        Err(_) => Err(carrier_error(
            StatusCode::BAD_REQUEST,
            "invalid-request",
            "Request is malformed",
        )),
    }
}

fn decode_strict<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, ()> {
    IJsonValue::parse(bytes).map_err(|_| ())?;
    serde_json::from_slice(bytes).map_err(|_| ())
}

fn json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(';')
                .next()
                .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("application/json"))
        })
}

fn call_context(shared: &Shared, handoff: DurableHandoffSignal) -> CallContext {
    CallContext {
        response_deadline: Instant::now() + shared.config.limits.response_timeout,
        drain: DrainSignal::new(Arc::clone(&shared.draining)),
        handoff,
    }
}

fn json_response(value: &impl Serialize) -> Response {
    match serde_json_canonicalizer::to_vec(value) {
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .header("content-length", bytes.len().to_string())
            .body(Body::from(bytes))
            .expect("valid application response"),
        Err(_) => carrier_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "not-ready",
            "Endpoint is not ready",
        ),
    }
}

fn accepted_but_not_confirmed(rpc_id: &str, operation: &str) -> ServerResponse {
    let details = serde_json::to_vec(&serde_json::json!({
        "rpcId": rpc_id,
        "operation": operation,
    }))
    .expect("closed accepted-but-not-confirmed details serialize");
    ServerResponse {
        envelope_type: "server-response".to_owned(),
        rpc_id: rpc_id.to_owned(),
        result: RpcResult {
            ok: false,
            value: None,
            error: Some(RpcError {
                code: "accepted-but-not-confirmed".to_owned(),
                message: "Mutation was accepted but its receipt was not confirmed".to_owned(),
                details: IJsonValue::parse(&details)
                    .expect("closed accepted-but-not-confirmed details are I-JSON"),
            }),
        },
    }
}

fn host_failure(shared: &Shared, failure: HostFailure) -> (Response, &'static str) {
    match failure {
        HostFailure::InvalidRequest => (
            carrier_error(
                StatusCode::BAD_REQUEST,
                "invalid-request",
                "Request is malformed",
            ),
            "invalid-request",
        ),
        HostFailure::Overloaded => (
            carrier_error(
                StatusCode::TOO_MANY_REQUESTS,
                "overloaded",
                "Endpoint admission limit reached",
            ),
            "overloaded",
        ),
        HostFailure::NotReady | HostFailure::Protocol(_) | HostFailure::Internal(_) => {
            (unavailable(shared), unavailable_code(shared))
        }
    }
}

async fn fallback(request: Request<Body>) -> Response {
    let status = if request.uri().path().starts_with("/api/") && request.method() != Method::POST {
        StatusCode::METHOD_NOT_ALLOWED
    } else {
        StatusCode::NOT_FOUND
    };
    match status {
        StatusCode::METHOD_NOT_ALLOWED => carrier_error(
            StatusCode::METHOD_NOT_ALLOWED,
            "method-not-allowed",
            "Method is not allowed",
        ),
        _ => carrier_error(StatusCode::NOT_FOUND, "not-found", "Path not found"),
    }
}

async fn method_not_allowed() -> Response {
    carrier_error(
        StatusCode::METHOD_NOT_ALLOWED,
        "method-not-allowed",
        "Method is not allowed",
    )
}

async fn request_security(
    State(shared): State<Arc<Shared>>,
    request: Request,
    next: Next,
) -> Response {
    if !is_api_path(request.uri().path()) {
        return next.run(request).await;
    }
    if !shared.config.bearer_token.authenticates(request.headers()) {
        return unauthorized();
    }
    if !shared.config.origins.permits(request.headers()) {
        return carrier_error(
            StatusCode::FORBIDDEN,
            "forbidden-origin",
            "Browser origin is not allowed",
        );
    }
    next.run(request).await
}

async fn web_request_security(
    State(shared): State<Arc<WebShared>>,
    request: Request,
    next: Next,
) -> Response {
    if !shared.browser.permits_host(request.headers()) {
        return unauthorized();
    }
    if is_web_data_path(request.uri().path()) && !shared.browser.permits_origin(request.headers()) {
        return carrier_error(
            StatusCode::FORBIDDEN,
            "forbidden-origin",
            "Browser origin is not allowed",
        );
    }
    next.run(request).await
}

fn is_api_path(path: &str) -> bool {
    path == "/api" || path.starts_with("/api/")
}

fn is_web_data_path(path: &str) -> bool {
    path == "/web/remote.mux" || path.starts_with("/web/api/")
}

fn unauthorized() -> Response {
    carrier_error(
        StatusCode::UNAUTHORIZED,
        "unauthorized",
        "Authentication required",
    )
}

fn unavailable(shared: &Shared) -> Response {
    if shared.draining.load(Ordering::Acquire) {
        carrier_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "server-draining",
            "Endpoint is draining",
        )
    } else {
        carrier_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "not-ready",
            "Endpoint is not ready",
        )
    }
}

fn unavailable_code(shared: &Shared) -> &'static str {
    if shared.draining.load(Ordering::Acquire) {
        "server-draining"
    } else {
        "not-ready"
    }
}

fn carrier_error_code(status: StatusCode) -> &'static str {
    match status {
        StatusCode::BAD_REQUEST => "invalid-request",
        StatusCode::UNAUTHORIZED => "unauthorized",
        StatusCode::FORBIDDEN => "forbidden-origin",
        StatusCode::NOT_FOUND => "not-found",
        StatusCode::METHOD_NOT_ALLOWED => "method-not-allowed",
        StatusCode::PAYLOAD_TOO_LARGE => "payload-too-large",
        StatusCode::UNSUPPORTED_MEDIA_TYPE => "unsupported-media-type",
        StatusCode::TOO_MANY_REQUESTS => "overloaded",
        StatusCode::SERVICE_UNAVAILABLE => "not-ready",
        _ => "internal-error",
    }
}

fn carrier_error(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    #[derive(Serialize)]
    struct ErrorBody<'a> {
        error: ErrorValue<'a>,
    }
    #[derive(Serialize)]
    struct ErrorValue<'a> {
        code: &'a str,
        details: EmptyDetails,
        message: &'a str,
    }
    #[derive(Serialize)]
    struct EmptyDetails {}

    let bytes = serde_json_canonicalizer::to_vec(&ErrorBody {
        error: ErrorValue {
            code,
            details: EmptyDetails {},
            message,
        },
    })
    .expect("closed carrier errors serialize");
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .header("content-length", bytes.len().to_string())
        .body(Body::from(bytes))
        .expect("static carrier response")
}

#[derive(Debug, Error)]
pub enum TransportConfigError {
    #[error("production endpoint transport requires a loopback bind, found {0}")]
    NonLoopback(IpAddr),
    #[error("v2 transport limits cannot be overridden without a versioned contract")]
    UnversionedLimitOverride,
    #[error("deployment readiness identity requires a non-empty build and positive generation")]
    InvalidReadinessIdentity,
    #[error(
        "listener address differs from configured bind; configured={configured}, actual={actual}"
    )]
    ListenerAddressMismatch {
        configured: SocketAddr,
        actual: SocketAddr,
    },
    #[error(
        "endpoint method registry differs from Client authority; missing={missing:?}, unexpected={unexpected:?}"
    )]
    RegistryMismatch {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
    #[error("extension route overlaps frozen Session Endpoint registration: {0}")]
    ExtensionCollision(String),
    #[error("extension route has no advertised method class: {0}")]
    InvalidExtensionRegistration(String),
    #[error("listener failed: {0}")]
    Io(#[from] std::io::Error),
}
