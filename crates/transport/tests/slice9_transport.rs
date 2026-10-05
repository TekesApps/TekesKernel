use std::collections::{BTreeSet, HashMap, VecDeque};
use std::future::{pending, ready};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use endpoint::{
    CallContext, CarrierHostFuture, ClientRequest, ClientResponse, EndpointCarrierHost,
    EndpointStream, EndpointStreamReceiver, HostFailure, HostReadiness, MethodClass,
    MuxHostDescription, MuxHostProduct, RespondReceipt, RpcError, RpcResult, ServerRequest,
    ServerResponse, SessionEndpointCapability, SessionStream, SessionStreamReceiver,
    SessionStreamTarget, SessionSyncFrame, StreamChannel, StreamErrorCode, StreamFailure,
    WorkspaceBaseline,
};
use futures_util::{SinkExt, StreamExt};
use schema::IJsonValue;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tower::ServiceExt;
use transport::{
    AccessLogRecord, AccessLogSink, BearerToken, TransportConfig, TransportServer, WebClientConfig,
};

const FIXTURE_AUTHORIZATION: &str = "Bearer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

#[tokio::test]
async fn file_change_socket_is_authenticated_and_drops_subscription_on_close() {
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Feed(Arc<AtomicBool>, bool);
    impl transport::FileChangeFeed for Feed {
        fn poll(&mut self) -> Result<Vec<Value>, String> {
            if self.1 {
                Ok(vec![])
            } else {
                self.1 = true;
                Ok(vec![json!({"kind":"ready"})])
            }
        }
    }
    impl Drop for Feed {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    struct Authority(Arc<AtomicBool>);
    impl transport::FileChangeAuthority for Authority {
        fn open(&self, session: &str) -> Result<Box<dyn transport::FileChangeFeed>, String> {
            assert_eq!(session, "fixture");
            Ok(Box::new(Feed(self.0.clone(), false)))
        }
    }
    let dropped = Arc::new(AtomicBool::new(false));
    let mut server = TransportServer::new(Arc::new(MockHost::new(true)), config()).unwrap();
    server.set_file_changes(Arc::new(Authority(dropped.clone())));
    let handle = server.handle();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(server.serve(listener));
    let url = format!("ws://{address}/api/session.files.changes?sessionId=fixture");
    assert!(tokio_tungstenite::connect_async(&url).await.is_err());
    let mut request = url.into_client_request().unwrap();
    request
        .headers_mut()
        .insert("Authorization", FIXTURE_AUTHORIZATION.parse().unwrap());
    let (mut socket, _) = tokio_tungstenite::connect_async(request).await.unwrap();
    let frame = tokio::time::timeout(std::time::Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(frame.to_text().unwrap()).unwrap(),
        json!({"kind":"ready"})
    );
    socket.close(None).await.unwrap();
    for _ in 0..100 {
        if dropped.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    assert!(dropped.load(Ordering::SeqCst));
    handle.begin_drain();
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn preview_http_requires_auth_and_serves_raw_bytes_until_release() {
    struct Preview(Mutex<bool>);
    impl transport::FileTransferAuthority for Preview {
        fn prepare(&self, value: Value) -> Result<Value, String> {
            assert_eq!(value["path"], "file");
            *self.0.lock().unwrap() = true;
            Ok(json!({"version":1,"lease":"a".repeat(43),"size":3,"revision":"fixture"}))
        }
        fn read(&self, id: &str) -> Option<Arc<[u8]>> {
            (*self.0.lock().unwrap() && id == "a".repeat(43)).then(|| Arc::from([0u8, 255, 1]))
        }
        fn release(&self, _: &str) {
            *self.0.lock().unwrap() = false;
        }
    }
    let app = TransportServer::new(Arc::new(MockHost::new(true)), config())
        .unwrap()
        .with_file_transfer(Arc::new(Preview(Mutex::new(false))))
        .router();
    let path = "/api/tekesWorkspace.fileTransfer/prepare";
    let unauthorized = Request::builder()
        .method(Method::POST)
        .uri(path)
        .body(Body::from("{}"))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(unauthorized).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
    let response = app
        .clone()
        .oneshot(request(Method::POST, path, r#"{"path":"file"}"#))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let descriptor: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 16384).await.unwrap()).unwrap();
    let lease = descriptor["lease"].as_str().unwrap();
    let make = |method, suffix| {
        let mut request = request(
            method,
            &format!("/api/tekesWorkspace.fileTransfer/{suffix}"),
            Body::empty(),
        );
        request
            .headers_mut()
            .insert("X-Tekes-File-Lease", lease.parse().unwrap());
        request
    };
    let response = app
        .clone()
        .oneshot(make(Method::GET, "read"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(response.into_body(), 10).await.unwrap().as_ref(),
        &[0, 255, 1]
    );
    assert_eq!(
        app.clone()
            .oneshot(make(Method::POST, "release"))
            .await
            .unwrap()
            .status(),
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        app.oneshot(make(Method::GET, "read"))
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
}

#[derive(Default)]
struct MockState {
    ready: bool,
    calls: usize,
    retained: HashMap<String, (Vec<u8>, ServerResponse)>,
}

struct MockHost {
    state: Arc<Mutex<MockState>>,
    registered: BTreeSet<String>,
    block_unary: bool,
    fail_stream: bool,
    terminal_stream: Option<StreamErrorCode>,
    repeat_baseline: bool,
    extensions: BTreeSet<String>,
}

struct MockMuxStream {
    frames: VecDeque<SessionSyncFrame>,
}

impl SessionStream for MockMuxStream {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<SessionSyncFrame, StreamFailure>>> {
        Box::pin(ready(self.frames.pop_front().map(Ok)))
    }
}

#[derive(Default)]
struct RecordingLog {
    records: Mutex<Vec<AccessLogRecord>>,
}

impl AccessLogSink for RecordingLog {
    fn record(&self, record: &AccessLogRecord) {
        self.records
            .lock()
            .expect("access log")
            .push(record.clone());
    }
}

impl MockHost {
    fn new(ready: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState {
                ready,
                ..MockState::default()
            })),
            registered: transport::ROUTE_REGISTRY
                .iter()
                .map(ToString::to_string)
                .collect(),
            block_unary: false,
            fail_stream: false,
            terminal_stream: None,
            repeat_baseline: false,
            extensions: BTreeSet::new(),
        }
    }

    fn without_route(mut self, route: &str) -> Self {
        self.registered.remove(route);
        self
    }

    fn blocking(mut self) -> Self {
        self.block_unary = true;
        self
    }

    fn with_repeat_baseline(mut self) -> Self {
        self.repeat_baseline = true;
        self
    }

    fn with_extension(mut self, method: &str, registered: bool) -> Self {
        self.extensions.insert(method.to_owned());
        if registered {
            self.registered.insert(method.to_owned());
        }
        self
    }
}

impl EndpointCarrierHost for MockHost {
    fn readiness(&self) -> HostReadiness {
        if self.state.lock().expect("state").ready {
            HostReadiness::Ready
        } else {
            HostReadiness::NotReady
        }
    }

    fn classify_method(&self, method: &str) -> MethodClass {
        match method {
            "models.list" | "session.models" | "session.attachment" | "remote.mux" => {
                MethodClass::ReadOnly
            }
            "session.prompt" => MethodClass::Mutation,
            method if self.extensions.contains(method) => MethodClass::ReadOnly,
            _ => MethodClass::Unknown,
        }
    }

    fn registered_methods(&self) -> BTreeSet<String> {
        self.registered.clone()
    }

    fn advertised_extension_methods(&self) -> BTreeSet<String> {
        self.extensions.clone()
    }

    fn unary(
        &self,
        request: ClientRequest,
        _context: CallContext,
    ) -> CarrierHostFuture<'_, Result<ServerResponse, HostFailure>> {
        let state = Arc::clone(&self.state);
        let block_unary = self.block_unary;
        Box::pin(async move {
            if block_unary {
                state.lock().expect("state").calls += 1;
                return pending::<Result<ServerResponse, HostFailure>>().await;
            }
            let mut state = state.lock().expect("state");
            state.calls += 1;
            let fingerprint = request_fingerprint(&request);
            if let Some((existing, response)) = state.retained.get(&request.rpc_id) {
                if *existing == fingerprint {
                    return Ok(response.clone());
                }
                return Ok(error_response(
                    &request.rpc_id,
                    "idempotency-conflict",
                    "rpcId was already used for another request",
                    json!({"rpcId": request.rpc_id, "operation": request.method}),
                ));
            }
            let response = match request.method.as_str() {
                "session.history" => success_response(
                    &request.rpc_id,
                    serde_json::from_slice(include_bytes!(
                        "../../../fixtures/endpoint/authority/session-history.json"
                    ))
                    .expect("history authority fixture"),
                ),
                "host.describe" => success_response(
                    &request.rpc_id,
                    json!({
                        "version": "0.1.0-rc.5",
                        "cwd": "/workspace",
                        "provider": "deepseek",
                        "model": "deepseek-chat",
                        "attachedSessions": 1,
                        "home": "/Users/example",
                        "canOpenPath": true
                    }),
                ),
                "session.models" => {
                    success_response(&request.rpc_id, json!({"providers":[],"selected":null}))
                }
                "session.prompt" => {
                    success_response(&request.rpc_id, json!({"accepted": true, "seq": 7}))
                }
                _ => error_response(
                    &request.rpc_id,
                    "method-not-found",
                    "Method was not found",
                    json!({"method": request.method}),
                ),
            };
            state
                .retained
                .insert(request.rpc_id.clone(), (fingerprint, response.clone()));
            Ok(response)
        })
    }

    fn respond(
        &self,
        _response: ClientResponse,
        _context: CallContext,
    ) -> CarrierHostFuture<'_, Result<RespondReceipt, HostFailure>> {
        Box::pin(ready(Ok(RespondReceipt {
            accepted: true,
            reason: None,
        })))
    }

    fn open_stream(
        &self,
        channel: StreamChannel,
    ) -> CarrierHostFuture<'_, Result<EndpointStreamReceiver, HostFailure>> {
        if self.fail_stream {
            return Box::pin(ready(Err(HostFailure::NotReady)));
        }
        let (method, payload) = match channel {
            StreamChannel::Mux => (
                "session/subscribed",
                json!({
                    "type": "session/subscribed",
                    "sessionId": "50aa8d4e-9305-49d1-837e-16d2e67f31b0",
                    "lastSeq": 6
                }),
            ),
            StreamChannel::Host => (
                "host/session-status",
                json!({
                    "type": "host/session-status",
                    "sessionId": "50aa8d4e-9305-49d1-837e-16d2e67f31b0",
                    "running": false
                }),
            ),
        };
        let mut frames = VecDeque::from([Ok(ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: "push-first".to_owned(),
            method: method.to_owned(),
            payload: ijson(payload),
        })]);
        if channel == StreamChannel::Mux && self.repeat_baseline {
            frames.push_back(Ok(ServerRequest {
                envelope_type: "server-request".to_owned(),
                rpc_id: "push-reattach".to_owned(),
                method: "session/subscribed".to_owned(),
                payload: ijson(json!({
                    "type": "session/subscribed",
                    "sessionId": "50aa8d4e-9305-49d1-837e-16d2e67f31b0",
                    "lastSeq": 6
                })),
            }));
        }
        if let Some(code) = self.terminal_stream {
            frames.push_back(Err(StreamFailure::new(code)));
        }
        let stream = MockStream {
            frames,
            stay_open: true,
        };
        Box::pin(ready(Ok(Box::new(stream) as EndpointStreamReceiver)))
    }

    fn stream_error(
        &self,
        _channel: StreamChannel,
        code: StreamErrorCode,
    ) -> Result<ServerRequest, HostFailure> {
        Ok(ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: format!("transport-{}", code.code()),
            method: "stream/error".to_owned(),
            payload: ijson(json!({
                "type": "stream/error",
                "error": {"code": code.code(), "message": code.code(), "details": {}}
            })),
        })
    }

    fn mux_description(&self) -> Result<MuxHostDescription, HostFailure> {
        Ok(MuxHostDescription {
            protocol_version: 3,
            product: MuxHostProduct {
                name: "MockKernel".to_owned(),
                version: "dev-test".to_owned(),
            },
            capabilities: SessionEndpointCapability::required(),
            cwd: "/workspace".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: "/Users/example".to_owned(),
            can_open_path: false,
        })
    }

    fn open_mux_stream(
        &self,
        generation: u64,
        target: SessionStreamTarget,
    ) -> CarrierHostFuture<'_, Result<SessionStreamReceiver, HostFailure>> {
        if self.fail_stream {
            return Box::pin(ready(Err(HostFailure::NotReady)));
        }
        let baseline = match target {
            SessionStreamTarget::Workspace => SessionSyncFrame::WorkspaceBaseline {
                generation,
                baseline: WorkspaceBaseline {
                    items: Vec::new(),
                    archived_session_ids: Vec::new(),
                },
            },
            SessionStreamTarget::SessionInventory => SessionSyncFrame::InventoryBaseline {
                generation,
                items: Vec::new(),
            },
            SessionStreamTarget::SessionJournal { address, .. } => {
                SessionSyncFrame::JournalSnapshot {
                    generation,
                    snapshot: endpoint::SessionJournalSnapshot {
                        window_limit: 50,
                        address,
                        through_sequence: -1,
                        entries: Vec::new(),
                        has_more_before: false,
                        projections: IJsonValue::parse_str(r#"{"asOfSeq":-1,"values":{}}"#)
                            .expect("projections"),
                    },
                }
            }
            SessionStreamTarget::SessionControl => SessionSyncFrame::ControlBaseline {
                generation,
                items: Vec::new(),
            },
            SessionStreamTarget::Actionables => SessionSyncFrame::ActionableBaseline {
                generation,
                items: Vec::new(),
            },
        };
        let frames = if self.repeat_baseline {
            VecDeque::from([baseline.clone(), baseline])
        } else {
            VecDeque::from([baseline])
        };
        Box::pin(ready(Ok(
            Box::new(MockMuxStream { frames }) as SessionStreamReceiver
        )))
    }
}

struct MockStream {
    frames: VecDeque<Result<ServerRequest, StreamFailure>>,
    stay_open: bool,
}

impl EndpointStream for MockStream {
    fn recv(&mut self) -> CarrierHostFuture<'_, Option<Result<ServerRequest, StreamFailure>>> {
        if let Some(frame) = self.frames.pop_front() {
            Box::pin(ready(Some(frame)))
        } else if self.stay_open {
            Box::pin(pending())
        } else {
            Box::pin(ready(None))
        }
    }
}

fn config() -> TransportConfig {
    TransportConfig::loopback(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
        BearerToken::new([0; 32]),
    )
}

fn request(method: Method, uri: &str, body: impl Into<Body>) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", FIXTURE_AUTHORIZATION)
        .header("content-type", "application/json")
        .body(body.into())
        .expect("request")
}

fn ijson(value: Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&value).expect("json bytes")).expect("I-JSON")
}

fn success_response(rpc_id: &str, value: Value) -> ServerResponse {
    ServerResponse {
        envelope_type: "server-response".to_owned(),
        rpc_id: rpc_id.to_owned(),
        result: RpcResult {
            ok: true,
            value: Some(ijson(value)),
            error: None,
        },
    }
}

fn error_response(rpc_id: &str, code: &str, message: &str, details: Value) -> ServerResponse {
    ServerResponse {
        envelope_type: "server-response".to_owned(),
        rpc_id: rpc_id.to_owned(),
        result: RpcResult {
            ok: false,
            value: None,
            error: Some(RpcError {
                code: code.to_owned(),
                message: message.to_owned(),
                details: ijson(details),
            }),
        },
    }
}

fn request_fingerprint(request: &ClientRequest) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(request).expect("canonical request")
}

async fn response_json(response: axum::response::Response) -> Value {
    let bytes = response_bytes(response).await;
    serde_json::from_slice(&bytes).expect("JSON response")
}

async fn response_bytes(response: axum::response::Response) -> Vec<u8> {
    to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body")
        .to_vec()
}

#[tokio::test]
async fn web_client_is_direct_on_loopback_and_rejects_cross_origin_data() {
    let endpoint_bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7347);
    let web_bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7357);
    let server = TransportServer::new(
        Arc::new(MockHost::new(true)),
        TransportConfig::loopback(endpoint_bind, BearerToken::new([0; 32])),
    )
    .expect("endpoint server");
    let native_router = server.router();
    let native_web_route = native_router
        .oneshot(request(Method::POST, "/web/launch", Body::empty()))
        .await
        .expect("native route response");
    assert_eq!(native_web_route.status(), StatusCode::NOT_FOUND);

    let router = server
        .web_client(WebClientConfig::loopback(web_bind))
        .expect("optional Web Client service")
        .router();

    let index = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .header("host", "127.0.0.1:7357")
                .body(Body::empty())
                .expect("direct index"),
        )
        .await
        .expect("direct index response");
    assert_eq!(index.status(), StatusCode::OK);
    assert_eq!(index.headers()["content-type"], "text/html; charset=utf-8");
    assert!(
        String::from_utf8(response_bytes(index).await)
            .expect("HTML")
            .contains("Tekes")
    );

    let wrong_host = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .header("host", "localhost:7357")
                .body(Body::empty())
                .expect("wrong Host index"),
        )
        .await
        .expect("wrong Host response");
    assert_eq!(wrong_host.status(), StatusCode::UNAUTHORIZED);

    let payload = serde_json::to_vec(&json!({
        "type": "client-request",
        "rpcId": "web-models",
        "method": "session.models",
        "payload": {}
    }))
    .expect("web RPC");
    let web_rpc = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/web/api/session.models")
                .header("host", "127.0.0.1:7357")
                .header("origin", "http://127.0.0.1:7357")
                .header("content-type", "application/json")
                .body(Body::from(payload.clone()))
                .expect("web RPC request"),
        )
        .await
        .expect("web RPC response");
    assert_eq!(web_rpc.status(), StatusCode::OK);
    assert_eq!(response_json(web_rpc).await["result"]["ok"], true);

    let forbidden = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/web/api/session.models")
                .header("host", "127.0.0.1:7357")
                .header("origin", "https://evil.example")
                .header("content-type", "application/json")
                .body(Body::from(payload.clone()))
                .expect("forbidden web RPC request"),
        )
        .await
        .expect("forbidden web RPC response");
    assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

    let missing_origin = router
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/web/api/session.models")
                .header("host", "127.0.0.1:7357")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .expect("missing Origin web RPC request"),
        )
        .await
        .expect("missing Origin web RPC response");
    assert_eq!(missing_origin.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn slice9_gate_65_endpoint_transport_authority() {
    assert_eq!(transport::ROUTE_REGISTRY.len(), 17);
    let non_loopback = TransportConfig::loopback(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7347),
        BearerToken::new([0; 32]),
    );
    assert!(TransportServer::new(Arc::new(MockHost::new(true)), non_loopback).is_err());

    let mismatch = TransportServer::new(
        Arc::new(MockHost::new(true).without_route("workspace.unarchiveSession")),
        config(),
    );
    assert!(
        mismatch.is_err(),
        "driver/route drift must fail before ready"
    );

    let extension = TransportServer::new(
        Arc::new(MockHost::new(true).with_extension("commands/list", true)),
        config(),
    );
    assert!(extension.is_ok(), "advertised additive route must compose");
    let phantom = TransportServer::new(
        Arc::new(MockHost::new(true).with_extension("commands/list", false)),
        config(),
    );
    assert!(
        phantom.is_err(),
        "phantom extension capability must fail closed"
    );
    let collision = TransportServer::new(
        Arc::new(MockHost::new(true).with_extension("remote.mux", true)),
        config(),
    );
    assert!(
        collision.is_err(),
        "extension/base collision must fail closed"
    );

    let router = TransportServer::new(Arc::new(MockHost::new(true)), config())
        .expect("V3 server")
        .router();
    for removed in [
        "/api/host.describe",
        "/api/workspace.list",
        "/api/session.list",
        "/api/session.history",
        "/api/events.mux",
        "/api/events.host",
        "/api/respond",
    ] {
        let response = router
            .clone()
            .oneshot(request(Method::POST, removed, "{}"))
            .await
            .expect("removed route response");
        assert!(
            matches!(
                response.status(),
                StatusCode::NOT_FOUND | StatusCode::METHOD_NOT_ALLOWED
            ),
            "removed V2 route {removed} remained public"
        );
    }
}

#[tokio::test]
async fn slice9_gate_66_endpoint_http_error_and_idempotency() {
    let host = Arc::new(MockHost::new(true));
    let state = Arc::clone(&host.state);
    let access_log = Arc::new(RecordingLog::default());
    let router = TransportServer::new(
        host,
        config().with_access_log(access_log.clone() as Arc<dyn AccessLogSink>),
    )
    .expect("server")
    .router();

    let live = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/health/live")
                .body(Body::empty())
                .expect("liveness request"),
        )
        .await
        .expect("liveness response");
    assert_eq!(live.status(), StatusCode::NO_CONTENT);

    let unauthenticated_upgrade_shape = Request::builder()
        .method(Method::GET)
        .uri("/api/events.mux")
        .body(Body::empty())
        .expect("unauthenticated upgrade shape");
    let response = router
        .clone()
        .oneshot(unauthenticated_upgrade_shape)
        .await
        .expect("unauthenticated upgrade response");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let auth_probe = json!({
        "type": "client-request",
        "rpcId": "auth-probe",
        "method": "session.models",
        "payload": {}
    });
    let auth_probe = serde_json::to_vec(&auth_probe).expect("auth probe");
    let mut missing_auth = request(Method::POST, "/api/session.models", auth_probe.clone());
    missing_auth.headers_mut().remove("authorization");
    missing_auth
        .headers_mut()
        .insert("origin", "https://evil.example".parse().expect("origin"));
    let response = router
        .clone()
        .oneshot(missing_auth)
        .await
        .expect("missing auth response");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "unauthorized"
    );

    let mut wrong_auth = request(Method::POST, "/api/session.models", auth_probe.clone());
    wrong_auth.headers_mut().insert(
        "authorization",
        "Bearer wrong-token".parse().expect("wrong authorization"),
    );
    let response = router
        .clone()
        .oneshot(wrong_auth)
        .await
        .expect("wrong auth response");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let mut forbidden_auth = request(Method::POST, "/api/session.models", auth_probe);
    forbidden_auth
        .headers_mut()
        .insert("origin", "null".parse().expect("origin"));
    let response = router
        .clone()
        .oneshot(forbidden_auth)
        .await
        .expect("forbidden origin response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(state.lock().expect("state").calls, 0);

    let malformed = request(
        Method::POST,
        "/api/session.prompt",
        r#"{"type":"client-request","rpcId":"x","rpcId":"y","method":"session.prompt","payload":{}}"#,
    );
    let response = router.clone().oneshot(malformed).await.expect("response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.headers()["content-type"], "application/json");
    let declared_length = response.headers()["content-length"]
        .to_str()
        .expect("content-length")
        .parse::<usize>()
        .expect("decimal content-length");
    assert_eq!(declared_length, 82);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "invalid-request"
    );

    let mut wrong_content = request(Method::POST, "/api/session.prompt", "{}");
    wrong_content.headers_mut().remove("content-type");
    let response = router
        .clone()
        .oneshot(wrong_content)
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "unsupported-media-type"
    );

    let mut forbidden = request(Method::POST, "/api/session.models", "{}");
    forbidden
        .headers_mut()
        .insert("origin", "https://evil.example".parse().expect("origin"));
    let response = router.clone().oneshot(forbidden).await.expect("response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "forbidden-origin"
    );

    let oversized = vec![b' '; transport::MAX_REQUEST_BYTES + 1];
    let response = router
        .clone()
        .oneshot(request(Method::POST, "/api/session.prompt", oversized))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "payload-too-large"
    );

    let response = router
        .clone()
        .oneshot(request(Method::POST, "/wrong", "{}"))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(response_json(response).await["error"]["code"], "not-found");

    let response = router
        .clone()
        .oneshot(request(Method::GET, "/api/session.prompt", ""))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response_json(response).await["error"]["code"],
        "method-not-allowed"
    );

    let first = json!({
        "type": "client-request", "rpcId": "mutation-1",
        "method": "session.prompt", "payload": {"sessionId": "50aa8d4e-9305-49d1-837e-16d2e67f31b0", "content": []}
    });
    let first_bytes = serde_json::to_vec(&first).expect("first request");
    let first_response = router
        .clone()
        .oneshot(request(
            Method::POST,
            "/api/session.prompt",
            first_bytes.clone(),
        ))
        .await
        .expect("first response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_value = response_json(first_response).await;

    let retry_response = router
        .clone()
        .oneshot(request(Method::POST, "/api/session.prompt", first_bytes))
        .await
        .expect("retry response");
    assert_eq!(retry_response.status(), StatusCode::OK);
    assert_eq!(response_json(retry_response).await, first_value);

    let conflict = json!({
        "type": "client-request", "rpcId": "mutation-1",
        "method": "session.prompt", "payload": {"sessionId": "50aa8d4e-9305-49d1-837e-16d2e67f31b0", "content": ["different"]}
    });
    let response = router
        .oneshot(request(
            Method::POST,
            "/api/session.prompt",
            serde_json::to_vec(&conflict).expect("conflict request"),
        ))
        .await
        .expect("conflict response");
    assert_eq!(response.status(), StatusCode::OK);
    let value = response_json(response).await;
    assert_eq!(value["result"]["error"]["code"], "idempotency-conflict");
    assert_eq!(state.lock().expect("state").calls, 3);

    let rendered_log = access_log
        .records
        .lock()
        .expect("access log")
        .iter()
        .map(|record| String::from_utf8(record.canonical_bytes().expect("canonical access log")))
        .collect::<Result<Vec<_>, _>>()
        .expect("UTF-8 logs")
        .join("\n");
    assert!(!rendered_log.contains("different"));
    assert!(!rendered_log.contains("50aa8d4e"));
    assert!(!rendered_log.contains("wrong-token"));
    assert!(!rendered_log.contains("authorization"));
    assert!(rendered_log.contains("mutation-1"));
    let mut successful_mutation_log = access_log
        .records
        .lock()
        .expect("access log")
        .iter()
        .find(|record| record.request_id == "mutation-1" && record.error_code.is_none())
        .expect("successful mutation access record")
        .clone();
    successful_mutation_log.elapsed_ms = 0;
    assert_eq!(
        successful_mutation_log
            .canonical_bytes()
            .expect("canonical normalized access record"),
        br#"{"elapsed_ms":0,"operation":"session.prompt","path":"/api/session.prompt","request_id":"mutation-1","status":200,"v":1}"#
    );

    let unavailable = TransportServer::new(Arc::new(MockHost::new(false)), config())
        .expect("server")
        .router()
        .oneshot(request(Method::POST, "/api/session.prompt", "{}"))
        .await
        .expect("not ready response");
    assert_eq!(unavailable.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response_json(unavailable).await["error"]["code"],
        "not-ready"
    );

    let draining_server =
        TransportServer::new(Arc::new(MockHost::new(true)), config()).expect("server");
    let draining_handle = draining_server.handle();
    let draining_router = draining_server.router();
    draining_handle.begin_drain();
    let drain_mutation = json!({
        "type": "client-request",
        "rpcId": "draining-mutation",
        "method": "session.prompt",
        "payload": {}
    });
    let draining = draining_router
        .clone()
        .oneshot(request(
            Method::POST,
            "/api/session.prompt",
            serde_json::to_vec(&drain_mutation).expect("drain mutation"),
        ))
        .await
        .expect("draining response");
    assert_eq!(draining.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response_json(draining).await["error"]["code"],
        "server-draining"
    );
    let drain_read = json!({
        "type": "client-request",
        "rpcId": "draining-read",
        "method": "session.models",
        "payload": {}
    });
    let read_response = draining_router
        .oneshot(request(
            Method::POST,
            "/api/session.models",
            serde_json::to_vec(&drain_read).expect("drain read"),
        ))
        .await
        .expect("read-only drain response");
    assert_eq!(read_response.status(), StatusCode::OK);

    let blocking_host = Arc::new(MockHost::new(true).blocking());
    let blocking_state = Arc::clone(&blocking_host.state);
    let blocking_router = TransportServer::new(blocking_host, config())
        .expect("blocking server")
        .router();
    let mut pending_calls = Vec::new();
    for index in 0..128 {
        let router = blocking_router.clone();
        let body = json!({
            "type": "client-request",
            "rpcId": format!("blocked-{index}"),
            "method": "session.models",
            "payload": {}
        });
        pending_calls.push(tokio::spawn(async move {
            router
                .oneshot(request(
                    Method::POST,
                    "/api/session.models",
                    serde_json::to_vec(&body).expect("blocked request"),
                ))
                .await
        }));
    }
    for _ in 0..1_000 {
        if blocking_state.lock().expect("state").calls == 128 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(blocking_state.lock().expect("state").calls, 128);
    let overloaded = blocking_router
        .oneshot(request(
            Method::POST,
            "/api/session.models",
            serde_json::to_vec(&json!({
                "type": "client-request",
                "rpcId": "blocked-overflow",
                "method": "session.models",
                "payload": {}
            }))
            .expect("overflow request"),
        ))
        .await
        .expect("overloaded response");
    assert_eq!(overloaded.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        response_json(overloaded).await["error"]["code"],
        "overloaded"
    );
    for task in pending_calls {
        task.abort();
    }
}

#[tokio::test]
async fn deployment_health_uses_frozen_identity_and_closed_readiness_rows() {
    let server = TransportServer::new(
        Arc::new(MockHost::new(true)),
        config().with_readiness_identity("2.0.0", 2),
    )
    .expect("deployment server");
    let handle = server.handle();
    let router = server.router();

    let live = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/health/live")
                .body(Body::empty())
                .expect("live request"),
        )
        .await
        .expect("live response");
    assert_eq!(live.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(live.into_body(), usize::MAX).await.expect("body"),
        br#"{"live":true}"#.as_slice()
    );

    let ready = router
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/health/ready")
                .body(Body::empty())
                .expect("ready request"),
        )
        .await
        .expect("ready response");
    assert_eq!(ready.status(), StatusCode::OK);
    assert_eq!(
        to_bytes(ready.into_body(), usize::MAX).await.expect("body"),
        br#"{"build":"2.0.0","generation":2,"ready":true}"#.as_slice()
    );

    handle.begin_drain();
    let draining = router
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/health/ready")
                .body(Body::empty())
                .expect("draining request"),
        )
        .await
        .expect("draining response");
    assert_eq!(draining.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        to_bytes(draining.into_body(), usize::MAX)
            .await
            .expect("body"),
        br#"{"error":{"code":"server-draining","details":{},"message":"Endpoint is draining"},"ready":false}"#.as_slice()
    );

    let not_ready = TransportServer::new(
        Arc::new(MockHost::new(false)),
        config().with_readiness_identity("2.0.0", 2),
    )
    .expect("not-ready server")
    .router()
    .oneshot(
        Request::builder()
            .method(Method::GET)
            .uri("/health/ready")
            .body(Body::empty())
            .expect("not-ready request"),
    )
    .await
    .expect("not-ready response");
    assert_eq!(not_ready.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        to_bytes(not_ready.into_body(), usize::MAX)
            .await
            .expect("body"),
        br#"{"error":{"code":"boot-incomplete","details":{},"message":"Startup has not completed"},"ready":false}"#.as_slice()
    );
}

#[tokio::test]
async fn idle_server_drain_does_not_wait_the_full_response_timeout() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener");
    let server = TransportServer::new(Arc::new(MockHost::new(true)), config()).expect("server");
    let handle = server.handle();
    let task = tokio::spawn(server.serve(listener));

    handle.begin_drain();
    tokio::time::timeout(std::time::Duration::from_secs(1), task)
        .await
        .expect("idle drain must not inherit the 30 second response timeout")
        .expect("server task")
        .expect("server drain");
}

#[tokio::test]
async fn v3_mux_rejects_a_duplicate_baseline_on_one_logical_stream() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener");
    let address = listener.local_addr().expect("address");
    let server = TransportServer::new(
        Arc::new(MockHost::new(true).with_repeat_baseline()),
        config(),
    )
    .expect("server");
    let task = tokio::spawn(server.serve(listener));
    let mut request = format!("ws://{address}/api/remote.mux")
        .into_client_request()
        .expect("websocket request");
    request.headers_mut().insert(
        "authorization",
        FIXTURE_AUTHORIZATION.parse().expect("authorization"),
    );
    let (mut socket, _) = tokio_tungstenite::connect_async(request)
        .await
        .expect("websocket connection");
    let ready: Value = serde_json::from_str(
        &socket
            .next()
            .await
            .expect("ready")
            .expect("ready frame")
            .into_text()
            .expect("ready text"),
    )
    .expect("ready JSON");
    assert_eq!(ready["type"], "ready");
    assert_eq!(ready["host"]["protocolVersion"], 3);
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            r#"{"streamId":"workspace-1","target":{"kind":"workspace"},"type":"open"}"#.into(),
        ))
        .await
        .expect("open stream");
    let baseline: Value = serde_json::from_str(
        &socket
            .next()
            .await
            .expect("baseline")
            .expect("baseline frame")
            .into_text()
            .expect("baseline text"),
    )
    .expect("baseline JSON");
    assert_eq!(baseline["type"], "stream");
    assert_eq!(baseline["frame"]["type"], "baseline");
    assert!(baseline["frame"]["generation"].is_string());
    assert!(baseline["frame"]["snapshot"]["items"].is_array());
    let error: Value = serde_json::from_str(
        &socket
            .next()
            .await
            .expect("protocol error")
            .expect("protocol error frame")
            .into_text()
            .expect("protocol error text"),
    )
    .expect("protocol error JSON");
    assert_eq!(error["type"], "error");
    assert_eq!(error["error"]["code"], "protocol-error");
    let close = socket.next().await.expect("close").expect("close frame");
    let tokio_tungstenite::tungstenite::Message::Close(Some(close)) = close else {
        panic!("expected protocol close")
    };
    assert_eq!(u16::from(close.code), 1002);
    task.abort();
}
