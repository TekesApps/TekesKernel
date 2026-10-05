use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use mcp::{
    HttpTransport, JsonRpcRequest, JsonRpcResponse, McpBroker, McpCancellationToken,
    McpCapabilities, McpClient, McpCredentialFieldOperation, McpError, McpLossState,
    McpManagementFault, McpManagementMutation, McpPeer, McpPeerFuture, McpPool, McpPoolKey,
    McpRegistry, McpRegistryStore, McpScope, McpServerConfig, McpServerReference, McpTool,
    McpToolAnnotations, McpToolCallContext, McpToolContinuation, McpTransport, McpTransportConfig,
    ProtocolMode, RecoveryAction, StdioTransport, project_catalog, project_name, recovery_action,
};
use profile::{DynamicToolEffect, ExternalEffectProtocol};
use schema::IJsonValue;
use serde_json::json;
use tempfile::TempDir;

fn fixture_client() -> McpClient<StdioTransport> {
    fixture_client_with_args(&[])
}

fn fixture_client_with_args(arguments: &[&str]) -> McpClient<StdioTransport> {
    let mut command = vec![env!("CARGO_BIN_EXE_mcp-fixture-server").to_owned()];
    command.extend(arguments.iter().map(|value| (*value).to_owned()));
    let transport =
        StdioTransport::spawn(&command, None, &BTreeMap::new()).expect("spawn fixture server");
    McpClient::new("fixture", transport)
}

#[tokio::test]
async fn slice13_gate_86_protocol_handshake_and_framing() {
    let capabilities: McpCapabilities =
        serde_json::from_str(r#"{"resources":{},"tools":{}}"#).expect("capability objects");
    assert!(capabilities.tools && capabilities.resources && !capabilities.prompts);
    assert!(serde_json::from_str::<McpCapabilities>(r#"{"tools":true}"#).is_err());
    assert_eq!(
        serde_json_canonicalizer::to_string(&capabilities).expect("canonical capabilities"),
        r#"{"resources":{},"tools":{}}"#
    );
    let mut client = fixture_client();
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("legacy handshake");
    assert_eq!(client.protocol_version(), "2025-11-25");
    assert_eq!(client.server().expect("server").name, "fixture");
    assert!(client.capabilities().tools);
    client.close().await.expect("close");

    let mut metadata = fixture_client_with_args(&["implementation-metadata"]);
    metadata
        .connect(ProtocolMode::Legacy)
        .await
        .expect("standard implementation metadata");
    let implementation = metadata.server().expect("metadata server");
    assert_eq!(implementation.title.as_deref(), Some("Fixture MCP"));
    assert_eq!(
        implementation.description.as_deref(),
        Some("Generic fixture metadata")
    );
    assert_eq!(
        implementation.website_url.as_deref(),
        Some("https://example.invalid/mcp")
    );
    let icon = &implementation.icons.as_ref().expect("icons")[0];
    assert_eq!(icon.mime_type.as_deref(), Some("image/png"));
    assert_eq!(
        icon.sizes.as_deref(),
        Some(["16x16".to_owned(), "any".to_owned()].as_slice())
    );
    assert_eq!(icon.theme, Some(mcp::McpIconTheme::Dark));
    let tools = metadata.list_tools().await.expect("metadata tools");
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "echo");
    assert!(tools[0].metadata.is_some());
    assert!(tools[0].annotations.read_only_hint);
    assert!(!tools[0].annotations.destructive_hint);
    metadata.close().await.expect("close metadata");

    for invalid in [
        r#"{"name":"fixture","version":"1","unknown":true}"#,
        r#"{"name":"fixture","version":"1","icons":[{"src":"https://example.invalid/icon.png","unknown":true}]}"#,
        r#"{"name":"fixture","version":"1","icons":[{"src":"not a URL"}]}"#,
        r#"{"name":"fixture","version":"1","icons":[{"src":"https://example.invalid/icon.png","sizes":["0x16"]}]}"#,
        r#"{"name":"fixture","version":"1","icons":[{"src":"https://example.invalid/icon.png","theme":"system"}]}"#,
        r#"{"name":"fixture","version":"1","websiteUrl":"relative/path"}"#,
    ] {
        assert!(serde_json::from_str::<mcp::McpImplementation>(invalid).is_err());
    }
    assert!(
        serde_json::from_str::<McpTool>(
            r#"{"name":"x","description":"x","inputSchema":{},"annotations":{},"unknownMetadata":{}}"#
        )
        .is_err()
    );

    let mut modern = fixture_client();
    modern
        .connect(ProtocolMode::Modern)
        .await
        .expect("modern handshake");
    assert_eq!(modern.protocol_version(), "2026-07-28");
    assert_eq!(modern.supported_versions(), &["2026-07-28".to_owned()]);
    assert_eq!(modern.list_tools().await.expect("modern tools").len(), 1);
    modern.close().await.expect("close");

    for scenario in ["duplicate-json", "unknown-notification", "oversized"] {
        let mut invalid = fixture_client_with_args(&[scenario]);
        assert!(matches!(
            invalid.connect(ProtocolMode::Legacy).await,
            Err(McpError::Protocol(_))
        ));
    }
    let mut unsupported = fixture_client_with_args(&["unsupported-version"]);
    assert!(matches!(
        unsupported.connect(ProtocolMode::Legacy).await,
        Err(McpError::Unsupported(_))
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stdio_close_is_bounded_when_a_descendant_inherits_stderr() {
    let root = tempfile::tempdir().expect("pid root");
    let pid_path = root.path().join("fixture.pid");
    let pid_path_text = pid_path.to_string_lossy().into_owned();
    let mut client = fixture_client_with_args(&["descendant-inherits-stderr", &pid_path_text]);
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect inherited-stderr fixture");
    let pid = std::fs::read_to_string(&pid_path)
        .expect("fixture pid")
        .parse::<i32>()
        .expect("numeric fixture pid");

    tokio::time::timeout(Duration::from_millis(750), client.close())
        .await
        .expect("stdio close must not await a descendant-owned stderr fd")
        .expect("close inherited-stderr fixture");
    assert_process_disappears(pid).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stdio_drop_is_nonblocking_and_reaps_the_child() {
    let root = tempfile::tempdir().expect("pid root");
    let pid_path = root.path().join("fixture.pid");
    let pid_path_text = pid_path.to_string_lossy().into_owned();
    let mut client = fixture_client_with_args(&["descendant-inherits-stderr", &pid_path_text]);
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect drop fixture");
    let pid = std::fs::read_to_string(&pid_path)
        .expect("fixture pid")
        .parse::<i32>()
        .expect("numeric fixture pid");

    let started = Instant::now();
    drop(client);
    assert!(started.elapsed() < Duration::from_millis(100));
    assert_process_disappears(pid).await;
}

async fn assert_process_disappears(raw_pid: i32) {
    let pid = rustix::process::Pid::from_raw(raw_pid).expect("positive fixture pid");
    let deadline = Instant::now() + Duration::from_secs(1);
    while rustix::process::test_kill_process(pid).is_ok() && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(
        rustix::process::test_kill_process(pid).is_err(),
        "stdio child was not reaped"
    );
}

#[tokio::test]
async fn slice13_gate_87_operations_and_cancellation_shapes() {
    let mut client = fixture_client();
    client.connect(ProtocolMode::Legacy).await.expect("connect");
    assert_eq!(client.list_tools().await.expect("tools").len(), 1);
    assert_eq!(client.list_prompts().await.expect("prompts").len(), 1);
    assert_eq!(client.list_resources().await.expect("resources").len(), 1);
    let arguments = IJsonValue::parse_str(r#"{"value":"ok"}"#).expect("arguments");
    assert!(
        !client
            .call_tool("echo", arguments, mcp::McpCancellationToken::default())
            .await
            .expect("tool result")
            .is_null()
    );
    assert!(
        !client
            .get_prompt(
                "hello",
                IJsonValue::parse_str(r#"{}"#).expect("prompt arguments")
            )
            .await
            .expect("prompt result")
            .is_null()
    );
    assert!(
        !client
            .read_resource("fixture://one")
            .await
            .expect("resource")
            .is_null()
    );
    assert!(
        !client
            .task_operation(
                "tasks/get",
                IJsonValue::parse_str(r#"{"taskId":"task-1"}"#).expect("task params")
            )
            .await
            .expect("task result")
            .is_null()
    );

    let continuation = McpToolContinuation {
        request_state: IJsonValue::parse_str(r#"{"cursor":"opaque"}"#).expect("state"),
        input_responses: IJsonValue::parse_str(r#"[{"answer":"yes"}]"#).expect("inputs"),
        round: 32,
        task_id: Some("task-1".to_owned()),
    };
    assert!(
        !client
            .continue_tool_call(
                "echo",
                IJsonValue::parse_str(r#"{"value":"original"}"#).unwrap(),
                continuation.clone(),
                McpCancellationToken::default(),
            )
            .await
            .expect("bounded continuation")
            .is_null()
    );
    let mut over_bound = continuation;
    over_bound.round = 33;
    assert!(matches!(
        client
            .continue_tool_call(
                "echo",
                IJsonValue::parse_str("{}").unwrap(),
                over_bound,
                McpCancellationToken::default()
            )
            .await,
        Err(McpError::Protocol(_))
    ));
    client.close().await.expect("close");

    let mut late = fixture_client();
    late.connect(ProtocolMode::Legacy)
        .await
        .expect("late connect");
    let cancellation = McpCancellationToken::default();
    let trigger = cancellation.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        trigger.cancel();
    });
    assert!(matches!(
        late.call_tool(
            "slow",
            IJsonValue::parse_str(r#"{}"#).expect("slow arguments"),
            cancellation,
        )
        .await,
        Err(McpError::UnknownEffect)
    ));
    assert!(
        !late
            .read_resource("fixture://one")
            .await
            .expect("read reconnects a fresh stdio generation")
            .is_null()
    );
    late.close().await.expect("close late client");

    let mut paginated = fixture_client_with_args(&["pagination"]);
    paginated
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect paginated");
    assert_eq!(paginated.list_tools().await.expect("two pages").len(), 2);
    paginated.close().await.expect("close paginated");

    let mut repeated = fixture_client_with_args(&["repeated-cursor"]);
    repeated
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect repeated");
    assert!(matches!(
        repeated.list_tools().await,
        Err(McpError::Protocol(_))
    ));

    let mut endless = fixture_client_with_args(&["endless-pagination"]);
    endless
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect endless");
    assert!(matches!(
        endless.list_tools().await,
        Err(McpError::CatalogLimit)
    ));

    let notifications = Arc::new(Mutex::new(Vec::new()));
    let request_count = Arc::new(AtomicUsize::new(0));
    let dispatched = Arc::new(tokio::sync::Notify::new());
    let closes = Arc::new(AtomicUsize::new(0));
    let mut cancellable = McpClient::new(
        "cancellable",
        PendingTransport {
            notifications: Arc::clone(&notifications),
            request_count: Arc::clone(&request_count),
            dispatched: Arc::clone(&dispatched),
            notify_behavior: PendingNotifyBehavior::Complete,
            closes: Arc::clone(&closes),
            initialized: false,
        },
    );
    cancellable
        .connect(ProtocolMode::Legacy)
        .await
        .expect("cancellable connect");
    let cancellation = McpCancellationToken::default();
    let trigger = cancellation.clone();
    tokio::spawn(async move {
        dispatched.notified().await;
        trigger.cancel();
    });
    let started = Instant::now();
    assert!(matches!(
        cancellable
            .call_tool(
                "slow",
                IJsonValue::parse_str(r#"{}"#).expect("slow arguments"),
                cancellation,
            )
            .await,
        Err(McpError::UnknownEffect)
    ));
    assert!(started.elapsed() < Duration::from_secs(1));
    assert!(
        notifications
            .lock()
            .expect("notifications")
            .iter()
            .any(
                |notification| notification.method == "notifications/cancelled"
                    && notification
                        .params
                        .as_ref()
                        .and_then(|params| params.canonical_bytes().ok())
                        .is_some_and(|bytes| bytes
                            .windows(b"\"requestId\":2".len())
                            .any(|window| { window == b"\"requestId\":2" }))
            )
    );

    let effect_request_count = Arc::new(AtomicUsize::new(0));
    let effect_dispatched = Arc::new(tokio::sync::Notify::new());
    let mut effectful = McpClient::new(
        "effectful-cancellable",
        PendingTransport {
            notifications: Arc::new(Mutex::new(Vec::new())),
            request_count: Arc::clone(&effect_request_count),
            dispatched: Arc::clone(&effect_dispatched),
            notify_behavior: PendingNotifyBehavior::Complete,
            closes: Arc::new(AtomicUsize::new(0)),
            initialized: false,
        },
    );
    effectful
        .connect(ProtocolMode::Legacy)
        .await
        .expect("effectful cancellable connect");
    let pre_dispatch = McpCancellationToken::default();
    pre_dispatch.cancel();
    assert!(matches!(
        effectful
            .call_tool_with_context(
                "slow",
                IJsonValue::parse_str(r#"{}"#).expect("pre-dispatch arguments"),
                McpToolCallContext {
                    idempotency_key: "b".repeat(64),
                },
                pre_dispatch,
            )
            .await,
        Err(McpError::Cancelled)
    ));
    assert_eq!(effect_request_count.load(Ordering::Acquire), 0);

    let post_dispatch = McpCancellationToken::default();
    let trigger = post_dispatch.clone();
    tokio::spawn(async move {
        effect_dispatched.notified().await;
        trigger.cancel();
    });
    assert!(matches!(
        effectful
            .call_tool_with_context(
                "slow",
                IJsonValue::parse_str(r#"{}"#).expect("post-dispatch arguments"),
                McpToolCallContext {
                    idempotency_key: "c".repeat(64),
                },
                post_dispatch,
            )
            .await,
        Err(McpError::UnknownEffect)
    ));
    assert_eq!(effect_request_count.load(Ordering::Acquire), 1);
}

struct PendingTransport {
    notifications: Arc<Mutex<Vec<JsonRpcRequest>>>,
    request_count: Arc<AtomicUsize>,
    dispatched: Arc<tokio::sync::Notify>,
    notify_behavior: PendingNotifyBehavior,
    closes: Arc<AtomicUsize>,
    initialized: bool,
}

#[derive(Clone, Copy)]
enum PendingNotifyBehavior {
    Complete,
    Error,
    Pending,
}

impl McpTransport for PendingTransport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _timeout: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        if !self.initialized && request.method == "initialize" {
            self.initialized = true;
            return Box::pin(async move {
                Ok(serde_json::from_value(serde_json::json!({
                    "jsonrpc":"2.0",
                    "id":request.id,
                    "result":{
                        "protocolVersion":"2025-11-25",
                        "capabilities":{"tools":{}},
                        "serverInfo":{"name":"pending","version":"1"}
                    }
                }))
                .expect("initialize response"))
            });
        }
        self.request_count.fetch_add(1, Ordering::AcqRel);
        self.dispatched.notify_one();
        Box::pin(std::future::pending())
    }

    fn notify<'a>(&'a mut self, notification: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        self.notifications
            .lock()
            .expect("notifications")
            .push(notification.clone());
        if notification.method != "notifications/cancelled" {
            return Box::pin(async { Ok(()) });
        }
        match self.notify_behavior {
            PendingNotifyBehavior::Complete => Box::pin(async { Ok(()) }),
            PendingNotifyBehavior::Error => Box::pin(async {
                Err(McpError::Transport(
                    "injected cancellation notification failure".to_owned(),
                ))
            }),
            PendingNotifyBehavior::Pending => Box::pin(std::future::pending()),
        }
    }

    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.closes.fetch_add(1, Ordering::AcqRel);
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_notification_failure_is_bounded_and_evicts_the_peer_generation() {
    for notify_behavior in [
        PendingNotifyBehavior::Complete,
        PendingNotifyBehavior::Error,
        PendingNotifyBehavior::Pending,
    ] {
        let request_count = Arc::new(AtomicUsize::new(0));
        let dispatched = Arc::new(tokio::sync::Notify::new());
        let closes = Arc::new(AtomicUsize::new(0));
        let mut client = McpClient::new(
            "cancel-notice-failure",
            PendingTransport {
                notifications: Arc::new(Mutex::new(Vec::new())),
                request_count: Arc::clone(&request_count),
                dispatched: Arc::clone(&dispatched),
                notify_behavior,
                closes: Arc::clone(&closes),
                initialized: false,
            },
        );
        client
            .connect(ProtocolMode::Legacy)
            .await
            .expect("connect pending peer");

        let broker = McpBroker::start().expect("start broker");
        let handle = broker.handle();
        let key = pool_key("cancel-notice-failure");
        handle
            .register(key.clone(), Box::new(client), false)
            .expect("register pending peer");
        let cancellation = McpCancellationToken::default();
        let trigger = cancellation.clone();
        let call_handle = handle.clone();
        let call_key = key.clone();
        let call = tokio::task::spawn_blocking(move || {
            call_handle.call_tool_with_context(
                call_key,
                "orders/create",
                IJsonValue::parse_str(r#"{"sku":"one"}"#).expect("arguments"),
                McpToolCallContext {
                    idempotency_key: "d".repeat(64),
                },
                cancellation,
            )
        });
        dispatched.notified().await;
        trigger.cancel();
        let result = tokio::time::timeout(Duration::from_secs(2), call)
            .await
            .expect("cancellation path is bounded")
            .expect("blocking broker task");
        assert!(matches!(result, Err(McpError::UnknownEffect)));
        assert_eq!(request_count.load(Ordering::Acquire), 1);
        assert!(closes.load(Ordering::Acquire) >= 1);
        assert_eq!(
            handle
                .catalog_generation(key)
                .expect("catalog generation after eviction"),
            None
        );
        drop(broker);
    }
}

#[derive(Clone, Copy)]
enum RecoveryScenario {
    Handshake,
    Read,
    AuthorityChangedRead,
    Mutation,
    ResumableTask,
    AuthorityChangedResumableTask,
    Protocol,
}

struct RecoveryTransport {
    scenario: RecoveryScenario,
    failed: bool,
    reconnects: Arc<AtomicUsize>,
    closes: Arc<AtomicUsize>,
    identity_queries: Arc<AtomicUsize>,
}

fn pool_key(server: &str) -> McpPoolKey {
    McpPoolKey {
        workspace: "ws".to_owned(),
        scope: "project".to_owned(),
        server: server.to_owned(),
        config_digest: "a".repeat(64),
        authorization_identity: "anonymous".to_owned(),
        protocol_mode: "legacy".to_owned(),
        plugin_generation: None,
    }
}

impl RecoveryTransport {
    fn new(
        scenario: RecoveryScenario,
        reconnects: Arc<AtomicUsize>,
        closes: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            scenario,
            failed: false,
            reconnects,
            closes,
            identity_queries: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn with_identity_queries(mut self, identity_queries: Arc<AtomicUsize>) -> Self {
        self.identity_queries = identity_queries;
        self
    }
}

impl McpTransport for RecoveryTransport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _timeout: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        let id = request.id.expect("request id");
        if matches!(self.scenario, RecoveryScenario::Handshake)
            && request.method == "initialize"
            && !self.failed
        {
            self.failed = true;
            return Box::pin(async { Err(McpError::Transport("handshake lost".to_owned())) });
        }
        if matches!(
            self.scenario,
            RecoveryScenario::Read | RecoveryScenario::AuthorityChangedRead
        ) && request.method == "resources/read"
            && !self.failed
        {
            self.failed = true;
            return Box::pin(async { Err(McpError::Transport("read lost".to_owned())) });
        }
        if matches!(self.scenario, RecoveryScenario::Mutation) && request.method == "tools/call" {
            return Box::pin(async { Err(McpError::Transport("mutation lost".to_owned())) });
        }
        if matches!(
            self.scenario,
            RecoveryScenario::ResumableTask | RecoveryScenario::AuthorityChangedResumableTask
        ) && request.method == "tasks/update"
            && !self.failed
        {
            self.failed = true;
            return Box::pin(async { Err(McpError::Transport("task mutation lost".to_owned())) });
        }
        if matches!(self.scenario, RecoveryScenario::Protocol) {
            return Box::pin(async { Err(McpError::Protocol("duplicate key".to_owned())) });
        }
        let result = match request.method.as_str() {
            "initialize" => {
                let changed = self.reconnects.load(Ordering::Acquire) > 0
                    && matches!(
                        self.scenario,
                        RecoveryScenario::AuthorityChangedRead
                            | RecoveryScenario::AuthorityChangedResumableTask
                    );
                json!({
                    "capabilities":{"resources":{},"tasks":{},"tools":{}},
                    "protocolVersion":"2025-11-25",
                    "serverInfo":{"name":"recovery","version":if changed {"2"} else {"1"}}
                })
            }
            "resources/read" => json!({"contents":[{"text":"ok","uri":"fixture://one"}]}),
            "tasks/get" => {
                self.identity_queries.fetch_add(1, Ordering::AcqRel);
                json!({"status":"completed","taskId":"task-1"})
            }
            _ => json!({}),
        };
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_owned(),
            id,
            result: Some(
                IJsonValue::parse(&serde_json::to_vec(&result).expect("response JSON"))
                    .expect("I-JSON response"),
            ),
            error: None,
        };
        Box::pin(async move { Ok(response) })
    }

    fn notify<'a>(&'a mut self, _notification: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.closes.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn reconnect(&mut self) -> McpPeerFuture<'_, ()> {
        self.reconnects.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn slice13_gate_88_stdio_and_daemon_pool_ownership() {
    let mut client = fixture_client();
    client.connect(ProtocolMode::Legacy).await.expect("connect");
    let pool = McpPool::new();
    let key = pool_key("fixture");
    pool.insert(key.clone(), Box::new(client))
        .await
        .expect("insert");
    assert!(pool.get(&key).await.is_some());
    assert!(pool.release(&key).await.expect("last-client release"));
    assert!(pool.get(&key).await.is_none());

    let mut exiting = fixture_client_with_args(&["exit-on-tool"]);
    exiting
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect exiting peer");
    let broker = McpBroker::start().expect("start exit broker");
    let handle = broker.handle();
    let exit_key = pool_key("exit-on-tool");
    handle
        .register(exit_key.clone(), Box::new(exiting), false)
        .expect("register exiting peer");
    assert!(matches!(
        handle.call_tool(
            exit_key.clone(),
            "echo",
            IJsonValue::parse_str(r#"{}"#).expect("exit arguments"),
            McpCancellationToken::default(),
        ),
        Err(McpError::UnknownEffect)
    ));
    assert!(matches!(
        handle.call_tool(
            exit_key,
            "echo",
            IJsonValue::parse_str(r#"{}"#).expect("exit arguments"),
            McpCancellationToken::default(),
        ),
        Err(McpError::Transport(_))
    ));
    drop(broker);

    let pool = Arc::new(McpPool::new());
    let creations = Arc::new(AtomicUsize::new(0));
    let closes = Arc::new(AtomicUsize::new(0));
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let pool = Arc::clone(&pool);
        let creations = Arc::clone(&creations);
        let closes = Arc::clone(&closes);
        tasks.push(tokio::spawn(async move {
            pool.acquire_or_create(pool_key("single-flight"), false, move || async move {
                creations.fetch_add(1, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(30)).await;
                let mut client = McpClient::new(
                    "single-flight",
                    RecoveryTransport::new(
                        RecoveryScenario::Read,
                        Arc::new(AtomicUsize::new(0)),
                        closes,
                    ),
                );
                client.connect(ProtocolMode::Legacy).await?;
                Ok(Box::new(client) as Box<dyn McpPeer>)
            })
            .await
        }));
    }
    let mut leases = Vec::new();
    for task in tasks {
        leases.push(task.await.expect("join creator").expect("acquire lease"));
    }
    assert_eq!(creations.load(Ordering::SeqCst), 1);
    let first = leases[0].peer();
    assert!(
        leases
            .iter()
            .all(|lease| Arc::ptr_eq(&first, &lease.peer()))
    );
    drop(first);
    let mut closed = false;
    for lease in leases {
        closed |= lease.release().await.expect("release lease");
    }
    assert!(closed);
    assert_eq!(closes.load(Ordering::SeqCst), 1);

    let drain_closes = Arc::new(AtomicUsize::new(0));
    let mut always_on = McpClient::new(
        "always-on",
        RecoveryTransport::new(
            RecoveryScenario::Read,
            Arc::new(AtomicUsize::new(0)),
            Arc::clone(&drain_closes),
        ),
    );
    always_on
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect drain peer");
    pool.replace_generation(pool_key("always-on"), Box::new(always_on), true)
        .await
        .expect("register always-on");
    assert_eq!(pool.drain().await.expect("drain"), 1);
    assert_eq!(drain_closes.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn slice13_gate_89_http_oauth_authorization_partition() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let address = listener.local_addr().expect("address");
    let server = tokio::spawn(async move {
        for id in 1..=2 {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut bytes = vec![0_u8; 16 * 1024];
            let count = socket.read(&mut bytes).await.expect("read");
            let request = String::from_utf8(bytes[..count].to_vec()).expect("utf8");
            assert!(request.contains("authorization: Bearer sentinel-token\r\n"));
            if id == 2 {
                assert!(request.contains("mcp-session-id: session-fixture\r\n"));
            }
            let (content_type, body, session) = if id == 1 {
                (
                    "text/event-stream",
                    "data: {\"id\":1,\"jsonrpc\":\"2.0\",\"result\":{\"ok\":true}}\n\n".to_owned(),
                    "Mcp-Session-Id: session-fixture\r\n",
                )
            } else {
                (
                    "application/json",
                    r#"{"id":2,"jsonrpc":"2.0","result":{"ok":true}}"#.to_owned(),
                    "",
                )
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\n{session}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.expect("write");
        }
    });
    let mut transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().expect("url"),
        &BTreeMap::new(),
        Some("sentinel-token"),
        true,
    )
    .expect("transport");
    let response = transport
        .request(
            &JsonRpcRequest::call(1, "ping", None),
            std::time::Duration::from_secs(2),
        )
        .await
        .expect("response");
    response.validate_for(1).expect("valid response");
    transport
        .request(
            &JsonRpcRequest::call(2, "ping", None),
            std::time::Duration::from_secs(2),
        )
        .await
        .expect("session response")
        .validate_for(2)
        .expect("valid session response");
    server.await.expect("server");

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind rotation");
    let address = listener.local_addr().expect("rotation address");
    let rotation_server = tokio::spawn(async move {
        for (index, expected) in [(0, "token-a"), (1, "token-b")] {
            let (mut socket, _) = listener.accept().await.expect("accept rotation");
            let mut bytes = vec![0_u8; 16 * 1024];
            let count = socket.read(&mut bytes).await.expect("read rotation");
            let request = String::from_utf8(bytes[..count].to_vec()).expect("rotation utf8");
            assert!(request.contains(&format!("authorization: Bearer {expected}\r\n")));
            if index == 1 {
                assert!(!request.contains("mcp-session-id:"));
            }
            let id = 10 + index;
            let body = format!(r#"{{"id":{id},"jsonrpc":"2.0","result":{{"ok":true}}}}"#);
            let session = if index == 0 {
                "Mcp-Session-Id: must-not-cross-identity\r\n"
            } else {
                ""
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n{session}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket
                .write_all(response.as_bytes())
                .await
                .expect("write rotation");
        }
    });
    let authorization_state = Arc::new(AtomicUsize::new(0));
    let provider_state = Arc::clone(&authorization_state);
    let mut rotating = HttpTransport::new_with_authorization_provider(
        format!("http://{address}/mcp")
            .parse()
            .expect("rotation url"),
        &BTreeMap::new(),
        move || match provider_state.load(Ordering::SeqCst) {
            0 => Ok(("identity-a".to_owned(), Some("token-a".to_owned()))),
            1 => Ok(("identity-b".to_owned(), Some("token-b".to_owned()))),
            _ => Err(McpError::Transport("credential revoked".to_owned())),
        },
        true,
    )
    .expect("rotating transport");
    rotating
        .request(
            &JsonRpcRequest::call(10, "ping", None),
            Duration::from_secs(2),
        )
        .await
        .expect("identity a");
    authorization_state.store(1, Ordering::SeqCst);
    rotating
        .request(
            &JsonRpcRequest::call(11, "ping", None),
            Duration::from_secs(2),
        )
        .await
        .expect("identity b");
    authorization_state.store(2, Ordering::SeqCst);
    assert!(matches!(
        rotating
            .request(
                &JsonRpcRequest::call(12, "ping", None),
                Duration::from_secs(2)
            )
            .await,
        Err(McpError::Transport(_))
    ));
    rotation_server.await.expect("rotation server");

    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind SSE");
    let address = listener.local_addr().expect("SSE address");
    let sse_server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept SSE");
        let mut request = vec![0_u8; 16 * 1024];
        let _ = socket.read(&mut request).await.expect("read SSE");
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: keep-alive\r\n\r\n")
            .await
            .expect("write SSE headers");
        let event = b"data: {\"id\":20,\"jsonrpc\":\"2.0\",\"result\":{\"ok\":true}}\n\n";
        socket
            .write_all(format!("{:x}\r\n", event.len()).as_bytes())
            .await
            .expect("write SSE chunk size");
        socket.write_all(event).await.expect("write SSE event");
        socket.write_all(b"\r\n").await.expect("finish SSE chunk");
        socket.flush().await.expect("flush SSE");
        tokio::time::sleep(Duration::from_millis(500)).await;
        let _ = socket.write_all(b"0\r\n\r\n").await;
    });
    let mut sse = HttpTransport::new(
        format!("http://{address}/mcp").parse().expect("SSE url"),
        &BTreeMap::new(),
        None,
        true,
    )
    .expect("SSE transport");
    let started = Instant::now();
    sse.request(
        &JsonRpcRequest::call(20, "ping", None),
        Duration::from_secs(2),
    )
    .await
    .expect("first complete SSE event");
    assert!(started.elapsed() < Duration::from_millis(300));
    sse_server.await.expect("SSE server");

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind redirect");
    let address = listener.local_addr().expect("redirect address");
    let redirect_server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept redirect");
        let mut request = vec![0_u8; 4096];
        let _ = socket.read(&mut request).await.expect("read redirect");
        socket
            .write_all(b"HTTP/1.1 302 Found\r\nLocation: https://example.invalid/mcp\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .await
            .expect("write redirect");
    });
    let mut redirect = HttpTransport::new(
        format!("http://{address}/mcp")
            .parse()
            .expect("redirect url"),
        &BTreeMap::new(),
        None,
        true,
    )
    .expect("redirect transport");
    assert!(matches!(
        redirect
            .request(
                &JsonRpcRequest::call(30, "ping", None),
                Duration::from_secs(2)
            )
            .await,
        Err(McpError::Transport(_))
    ));
    redirect_server.await.expect("redirect server");
}

#[test]
fn slice13_gate_90_catalog_projection_and_collision() {
    assert_eq!(
        project_name("server one", "read/file").expect("name"),
        "mcp__server_20one__read_2ffile"
    );
    let tool = McpTool {
        name: "echo".to_owned(),
        description: "Echo".to_owned(),
        input_schema: IJsonValue::parse_str(
            r#"{"additionalProperties":false,"properties":{},"required":[],"type":"object"}"#,
        )
        .expect("schema"),
        annotations: McpToolAnnotations {
            read_only_hint: true,
            destructive_hint: false,
        },
        metadata: None,
        execution: None,
    };
    let catalog = project_catalog("fixture", std::slice::from_ref(&tool), false).expect("catalog");
    assert_eq!(catalog.tools[0].name, "mcp__fixture__echo");
    assert!(project_catalog("fixture", &[tool.clone(), tool.clone()], false).is_err());
    // mcp-runtime: descriptions up to 4096 bytes with LF/CR/TAB project
    // (Cloudflare `search` ships 1760 B over many lines); longer is rejected.
    let mut long = tool.clone();
    long.description = "Search Cloudflare docs.\n\tUse when:\n".repeat(50);
    assert!(long.description.len() > 1024 && long.description.len() <= 4096);
    assert_eq!(
        project_catalog("fixture", std::slice::from_ref(&long), false)
            .expect("1760-byte class description projects")
            .tools
            .len(),
        1
    );
    let mut too_long = tool;
    too_long.description = "x".repeat(4097);
    assert!(project_catalog("fixture", std::slice::from_ref(&too_long), false).is_err());

    let destructive = McpTool {
        name: "request_permissions".to_owned(),
        description: "Request a system grant".to_owned(),
        input_schema: IJsonValue::parse_str(
            r#"{"additionalProperties":false,"properties":{},"required":[],"type":"object"}"#,
        )
        .expect("destructive schema"),
        annotations: McpToolAnnotations {
            read_only_hint: false,
            destructive_hint: true,
        },
        metadata: Some(IJsonValue::parse_str(r#"{"example.invalid/approval":{}}"#).unwrap()),
        execution: None,
    };
    let destructive = project_catalog("fixture", &[destructive], true).expect("destructive");
    assert_eq!(destructive.tools[0].effect, DynamicToolEffect::Destructive);
}

/// A real pydantic/FastMCP `tools/list` result (`dxf-editor-mcp`): `title`
/// on every node, `anyOf` with `null` for optionals, `default`, nested
/// arrays. Every tool projects; the frozen bytes are the regression oracle
/// for the dynamic-schema keyword contract.
#[test]
fn pydantic_generated_catalog_projects_every_tool() {
    let root = std::env::var_os("TEKES_KERNEL_FIXTURES").map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures"),
        PathBuf::from,
    );
    let bytes = std::fs::read(root.join("mcp-runtime/pydantic-tools-list.canonical.json"))
        .expect("pydantic tools/list fixture");
    #[derive(serde::Deserialize)]
    struct ToolsList {
        tools: Vec<McpTool>,
    }
    let listed: ToolsList = serde_json::from_slice(&bytes).expect("tools/list result");
    assert_eq!(listed.tools.len(), 6);
    let catalog =
        project_catalog("dxf-editor", &listed.tools, false).expect("pydantic catalog projects");
    assert_eq!(
        catalog
            .tools
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>(),
        [
            "add_line",
            "delete",
            "get_entities",
            "move",
            "set_text",
            "validate"
        ]
        .map(|name| format!("mcp__dxf_2deditor__{name}"))
    );
    // The projected parameters are the remote bytes, `title`/`anyOf` intact.
    let entities = catalog
        .tools
        .iter()
        .find(|tool| tool.name.ends_with("get_entities"))
        .unwrap();
    let parameters = serde_json::to_value(&entities.schema).unwrap()["parameters"].clone();
    assert_eq!(parameters["title"], "get_entitiesArguments");
    assert_eq!(
        parameters["properties"]["layout"]["anyOf"][1]["type"],
        "null"
    );

    // A keyword outside the contract is refused by name and JSON pointer.
    let mut refused = listed.tools[0].clone();
    refused.input_schema = IJsonValue::parse_str(
        r#"{"properties":{"layout":{"not":{"const":"model"},"type":"string"}},"type":"object"}"#,
    )
    .unwrap();
    let error = project_catalog("dxf-editor", std::slice::from_ref(&refused), false)
        .expect_err("unsupported keyword is refused")
        .to_string();
    assert!(error.contains("tool get_entities"), "{error}");
    assert!(
        error.contains("/parameters/properties/layout: uses unsupported keyword \"not\""),
        "{error}"
    );
}

#[test]
fn external_effect_metadata_projects_only_with_authoritative_read_only_query() {
    let reconcile = McpTool {
        name: "orders/get_by_idempotency_key".to_owned(),
        description: "Query an order by stable operation key".to_owned(),
        input_schema: IJsonValue::parse_str(
            r#"{"additionalProperties":false,"properties":{"idempotencyKey":{"type":"string"}},"required":["idempotencyKey"],"type":"object"}"#,
        )
        .expect("reconcile schema"),
        annotations: McpToolAnnotations {
            read_only_hint: true,
            destructive_hint: false,
        },
        metadata: None,
        execution: None,
    };
    let create = McpTool {
        name: "orders/create".to_owned(),
        description: "Create an order".to_owned(),
        input_schema: IJsonValue::parse_str(
            r#"{"additionalProperties":false,"properties":{"sku":{"type":"string"}},"required":["sku"],"type":"object"}"#,
        )
        .expect("create schema"),
        annotations: McpToolAnnotations {
            read_only_hint: false,
            destructive_hint: true,
        },
        metadata: Some(
            IJsonValue::parse_str(
                r#"{"io.tekes/externalEffect":{"reconcileTool":"orders/get_by_idempotency_key","version":1}}"#,
            )
            .expect("effect metadata"),
        ),
        execution: None,
    };
    let catalog = project_catalog("commerce", &[create.clone(), reconcile.clone()], false)
        .expect("external-effect catalog");
    let binding = catalog
        .tools
        .iter()
        .find(|tool| tool.name == "mcp__commerce__orders_2fcreate")
        .and_then(|tool| tool.external_effect.as_ref())
        .expect("projected external-effect binding");
    assert_eq!(
        binding.protocol,
        ExternalEffectProtocol::IdempotencyReconcileV1
    );
    assert_eq!(binding.reconcile_tool, "orders/get_by_idempotency_key");

    assert!(project_catalog("commerce", std::slice::from_ref(&create), false).is_err());
    let mut unsafe_query = reconcile;
    unsafe_query.annotations.read_only_hint = false;
    assert!(project_catalog("commerce", &[create, unsafe_query], false).is_err());
}

#[tokio::test]
async fn effectful_tool_call_carries_host_owned_idempotency_key_in_request_metadata() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let mut client = McpClient::new(
        "metadata",
        MetadataTransport {
            requests: Arc::clone(&requests),
        },
    );
    client
        .connect(ProtocolMode::Modern)
        .await
        .expect("modern connect");
    let key = "a".repeat(64);
    client
        .call_tool_with_context(
            "orders/create",
            IJsonValue::parse_str(r#"{"sku":"one"}"#).expect("arguments"),
            McpToolCallContext {
                idempotency_key: key.clone(),
            },
            McpCancellationToken::default(),
        )
        .await
        .expect("effectful call");
    let requests = requests.lock().expect("requests");
    let call = requests
        .iter()
        .find(|request| request.method == "tools/call")
        .expect("tools/call request");
    let params: serde_json::Value = serde_json::from_slice(
        &call
            .params
            .as_ref()
            .expect("params")
            .canonical_bytes()
            .expect("canonical params"),
    )
    .expect("JSON params");
    assert_eq!(
        params.pointer("/_meta/io.tekes~1idempotencyKey"),
        Some(&json!(key))
    );
    assert_eq!(
        params.pointer("/_meta/io.modelcontextprotocol~1protocolVersion"),
        Some(&json!("2026-07-28"))
    );
}

struct MetadataTransport {
    requests: Arc<Mutex<Vec<JsonRpcRequest>>>,
}

impl McpTransport for MetadataTransport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _timeout: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        self.requests
            .lock()
            .expect("requests")
            .push(request.clone());
        let result = match request.method.as_str() {
            "server/discover" => json!({
                "resultType":"complete",
                "supportedVersions":["2026-07-28"],
                "capabilities":{"tools":{}},
                "_meta":{"io.modelcontextprotocol/serverInfo":{"name":"metadata","version":"1"}}
            }),
            "tools/call" => json!({"ok":true}),
            other => panic!("unexpected method {other}"),
        };
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_owned(),
            id: request.id.expect("request id"),
            result: Some(
                IJsonValue::parse(&serde_json::to_vec(&result).expect("result JSON"))
                    .expect("I-JSON result"),
            ),
            error: None,
        };
        Box::pin(async move { Ok(response) })
    }

    fn notify<'a>(&'a mut self, _notification: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }

    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn slice13_list_changed_advances_catalog_generation() {
    let mut client = fixture_client_with_args(&["list-changed"]);
    client.connect(ProtocolMode::Legacy).await.expect("connect");
    let before = client.catalog_generation();
    client.list_tools().await.expect("tools");
    assert_eq!(client.catalog_generation(), before + 1);
    client.close().await.expect("close");
}

#[test]
fn slice13_gate_91_management_publication_and_recovery() {
    let root = TempDir::new().expect("temp");
    let store = McpRegistryStore::new(root.path());
    let config = McpServerConfig {
        reference: McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: McpScope::Project,
            name: "fixture".to_owned(),
        },
        transport: McpTransportConfig::Stdio {
            command: vec!["/usr/bin/true".to_owned()],
            cwd: Some(PathBuf::from("/").display().to_string()),
            environment: BTreeMap::new(),
        },
        enabled: true,
        always_on: false,
        protocol_mode: ProtocolMode::Legacy,
        owner: None,
        plugin_component: None,
        project_trusted: true,
    };
    let mutation = McpManagementMutation::Save {
        server: config.clone(),
        credential_fields: BTreeMap::new(),
    };
    let receipt = store
        .mutate_idempotent("rpc-1", &mutation)
        .expect("publish");
    assert_eq!(receipt.rpc_id, "rpc-1");
    assert_eq!(
        store.mutate_idempotent("rpc-1", &mutation).expect("re-ack"),
        receipt
    );
    assert!(
        store
            .mutate_idempotent(
                "rpc-1",
                &McpManagementMutation::Remove {
                    reference: config.reference.clone()
                },
            )
            .is_err()
    );
    assert_eq!(store.load().expect("load").servers, vec![config]);
    assert!(!root.path().join("mcp-operation.json").exists());
}

#[test]
fn slice13_gate_91_management_faults_recover_registry_and_credential_receipt() {
    for (index, fault) in [
        McpManagementFault::IntentDurable,
        McpManagementFault::RegistryStaged,
        McpManagementFault::CredentialsStaged,
        McpManagementFault::RegistryPublished,
        McpManagementFault::CredentialsPublished,
        McpManagementFault::BeforeReceipt,
    ]
    .into_iter()
    .enumerate()
    {
        let root = TempDir::new().expect("temp");
        let reference = McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: McpScope::User,
            name: "remote".to_owned(),
        };
        let server = McpServerConfig {
            reference: reference.clone(),
            transport: McpTransportConfig::Http {
                url: "https://mcp.example.invalid/rpc".to_owned(),
                headers: BTreeMap::new(),
                oauth: None,
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: false,
        };
        McpRegistryStore::new(root.path())
            .mutate_idempotent(
                &format!("save-{index}"),
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .expect("seed HTTP server");
        let mutation = McpManagementMutation::OauthStart {
            reference: reference.clone(),
            credential_id: "oauth-ref".to_owned(),
            authorization_url: "https://identity.example.invalid/authorize?state=opaque".to_owned(),
        };
        let rpc_id = format!("oauth-{index}");
        assert!(
            McpRegistryStore::new(root.path())
                .injecting_fault(fault)
                .mutate_idempotent(&rpc_id, &mutation)
                .is_err()
        );
        let recovered = McpRegistryStore::new(root.path());
        recovered.recover().expect("recover durable intent");
        let receipt = recovered
            .mutate_idempotent(&rpc_id, &mutation)
            .expect("same request re-acks recovered receipt");
        assert_eq!(receipt.rpc_id, rpc_id);
        let persisted = recovered
            .get(&reference)
            .expect("registry read")
            .expect("server retained");
        assert!(matches!(
            persisted.transport,
            McpTransportConfig::Http { oauth: Some(ref credential), .. } if credential == "oauth-ref"
        ));
        assert!(
            recovered
                .mutate_idempotent(
                    &rpc_id,
                    &McpManagementMutation::OauthRemove {
                        reference: reference.clone(),
                    },
                )
                .is_err(),
            "different canonical mutation must conflict"
        );
        assert!(!root.path().join("mcp-operation.json").exists());
    }
}

#[test]
fn slice13_gate_91_credential_field_operations_preserve_replace_and_remove_references() {
    let root = TempDir::new().expect("temp");
    let store = McpRegistryStore::new(root.path());
    let reference = McpServerReference {
        workspace_id: "ws".to_owned(),
        scope: McpScope::User,
        name: "stdio".to_owned(),
    };
    let make_server = || McpServerConfig {
        reference: reference.clone(),
        transport: McpTransportConfig::Stdio {
            command: vec!["/usr/bin/true".to_owned()],
            cwd: None,
            environment: BTreeMap::new(),
        },
        enabled: true,
        always_on: false,
        protocol_mode: ProtocolMode::Legacy,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    };
    store
        .mutate_idempotent(
            "replace",
            &McpManagementMutation::Save {
                server: make_server(),
                credential_fields: BTreeMap::from([(
                    "environment.API_TOKEN".to_owned(),
                    McpCredentialFieldOperation::Replace {
                        credential_id: "credential-one".to_owned(),
                    },
                )]),
            },
        )
        .expect("replace reference");
    store
        .mutate_idempotent(
            "preserve",
            &McpManagementMutation::Save {
                server: make_server(),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("omission preserves reference");
    let preserved = store.get(&reference).expect("load").expect("server");
    assert!(matches!(
        preserved.transport,
        McpTransportConfig::Stdio { ref environment, .. }
            if matches!(environment.get("API_TOKEN"), Some(mcp::CredentialValue::Credential { credential }) if credential == "credential-one")
    ));
    let references: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.path().join("mcp-credential-references.json"))
            .expect("credential reference document"),
    )
    .expect("credential reference JSON");
    assert_eq!(
        references["references"][0]["field"],
        "environment.API_TOKEN"
    );
    assert_eq!(references["references"][0]["state"], "bound");
    store
        .mutate_idempotent(
            "remove",
            &McpManagementMutation::Save {
                server: make_server(),
                credential_fields: BTreeMap::from([(
                    "environment.API_TOKEN".to_owned(),
                    McpCredentialFieldOperation::Remove,
                )]),
            },
        )
        .expect("remove reference");
    let removed = store.get(&reference).expect("load").expect("server");
    assert!(matches!(
        removed.transport,
        McpTransportConfig::Stdio { ref environment, .. } if !environment.contains_key("API_TOKEN")
    ));
    let references: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.path().join("mcp-credential-references.json"))
            .expect("credential reference document"),
    )
    .expect("credential reference JSON");
    assert_eq!(references["references"], serde_json::json!([]));
}

#[test]
fn slice13_gate_91_oauth_pending_to_bound_transition_is_exact() {
    let root = TempDir::new().expect("temp");
    let store = McpRegistryStore::new(root.path());
    let reference = McpServerReference {
        workspace_id: "ws".to_owned(),
        scope: McpScope::User,
        name: "oauth".to_owned(),
    };
    store
        .mutate_idempotent(
            "save-oauth",
            &McpManagementMutation::Save {
                server: McpServerConfig {
                    reference: reference.clone(),
                    transport: McpTransportConfig::Http {
                        url: "https://example.invalid/mcp".to_owned(),
                        headers: BTreeMap::new(),
                        oauth: None,
                    },
                    enabled: true,
                    always_on: false,
                    protocol_mode: ProtocolMode::Legacy,
                    owner: None,
                    plugin_component: None,
                    project_trusted: false,
                },
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save OAuth server");
    store
        .mutate_idempotent(
            "start-oauth",
            &McpManagementMutation::OauthStart {
                reference: reference.clone(),
                credential_id: "oauth-ref".to_owned(),
                authorization_url: "https://identity.example.invalid/authorize".to_owned(),
            },
        )
        .expect("start OAuth");
    assert_eq!(
        store
            .credential_reference_state(&reference, "oauth", "oauth-ref")
            .expect("pending state"),
        Some(mcp::McpCredentialReferenceState::Pending)
    );
    assert!(
        store
            .mutate_idempotent(
                "bind-wrong",
                &McpManagementMutation::OauthBind {
                    reference: reference.clone(),
                    credential_id: "other-ref".to_owned(),
                },
            )
            .is_err()
    );
    let mutation = McpManagementMutation::OauthBind {
        reference: reference.clone(),
        credential_id: "oauth-ref".to_owned(),
    };
    let receipt = store
        .mutate_idempotent("bind-oauth", &mutation)
        .expect("bind exact pending owner");
    assert_eq!(
        store
            .credential_reference_state(&reference, "oauth", "oauth-ref")
            .expect("bound state"),
        Some(mcp::McpCredentialReferenceState::Bound)
    );
    assert_eq!(
        store
            .mutate_idempotent("bind-oauth", &mutation)
            .expect("re-ack exact completion"),
        receipt
    );
    assert!(
        store.mutate_idempotent("bind-again", &mutation).is_err(),
        "a distinct completion cannot re-own an already-bound reference"
    );
}

#[test]
fn slice13_gate_91_corrupt_operation_digest_chain_fails_closed() {
    let root = TempDir::new().expect("temp");
    let server = McpServerConfig {
        reference: McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: McpScope::User,
            name: "fixture".to_owned(),
        },
        transport: McpTransportConfig::Stdio {
            command: vec!["/usr/bin/true".to_owned()],
            cwd: None,
            environment: BTreeMap::new(),
        },
        enabled: true,
        always_on: false,
        protocol_mode: ProtocolMode::Legacy,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    };
    assert!(
        McpRegistryStore::new(root.path())
            .injecting_fault(McpManagementFault::IntentDurable)
            .mutate_idempotent(
                "corrupt-op",
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .is_err()
    );
    let operation_path = root.path().join("mcp-operation.json");
    let mut operation: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&operation_path).expect("operation bytes"))
            .expect("operation JSON");
    operation["mutation"]["server"]["enabled"] = serde_json::Value::Bool(false);
    let mut bytes = serde_json_canonicalizer::to_vec(&operation).expect("canonical operation");
    bytes.push(b'\n');
    std::fs::write(operation_path, bytes).expect("corrupt operation fixture");
    assert!(McpRegistryStore::new(root.path()).recover().is_err());
    assert!(root.path().join("mcp-operation.json").exists());
    assert!(!root.path().join("mcp-servers.json").exists());
}

#[tokio::test]
async fn slice13_gate_92_closed_recovery_matrix() {
    assert_eq!(
        recovery_action(McpLossState::Handshake),
        RecoveryAction::ReconnectOnce
    );
    assert_eq!(
        recovery_action(McpLossState::ToolWithoutProof),
        RecoveryAction::UnknownEffect
    );
    assert_eq!(
        recovery_action(McpLossState::ResumableOperation),
        RecoveryAction::QueryIdentity
    );
    assert_eq!(
        recovery_action(McpLossState::AuthorityChanged),
        RecoveryAction::FreshGeneration
    );
    assert_eq!(
        recovery_action(McpLossState::StdioExit),
        RecoveryAction::FailAndRestartOnDemand
    );

    let reconnects = Arc::new(AtomicUsize::new(0));
    let closes = Arc::new(AtomicUsize::new(0));
    let mut handshake = McpClient::new(
        "handshake",
        RecoveryTransport::new(
            RecoveryScenario::Handshake,
            Arc::clone(&reconnects),
            Arc::clone(&closes),
        ),
    );
    handshake
        .connect(ProtocolMode::Legacy)
        .await
        .expect("handshake reconnects exactly once");
    assert_eq!(reconnects.load(Ordering::SeqCst), 1);

    let reconnects = Arc::new(AtomicUsize::new(0));
    let mut reader = McpClient::new(
        "reader",
        RecoveryTransport::new(
            RecoveryScenario::Read,
            Arc::clone(&reconnects),
            Arc::new(AtomicUsize::new(0)),
        ),
    );
    reader
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect reader");
    reader
        .read_resource("fixture://one")
        .await
        .expect("read reconnects once");
    assert_eq!(reconnects.load(Ordering::SeqCst), 1);

    let mut changed_reader = McpClient::new(
        "changed-reader",
        RecoveryTransport::new(
            RecoveryScenario::AuthorityChangedRead,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
        ),
    );
    changed_reader
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect changed reader");
    assert!(matches!(
        changed_reader.read_resource("fixture://one").await,
        Err(McpError::Conflict(_))
    ));

    let mut mutation = McpClient::new(
        "mutation",
        RecoveryTransport::new(
            RecoveryScenario::Mutation,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
        ),
    );
    mutation
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect mutation");
    assert!(matches!(
        mutation
            .call_tool(
                "mutate",
                IJsonValue::parse_str(r#"{}"#).expect("arguments"),
                McpCancellationToken::default(),
            )
            .await,
        Err(McpError::UnknownEffect)
    ));

    let broker_closes = Arc::new(AtomicUsize::new(0));
    let mut broker_peer = McpClient::new(
        "broker-mutation",
        RecoveryTransport::new(
            RecoveryScenario::Mutation,
            Arc::new(AtomicUsize::new(0)),
            Arc::clone(&broker_closes),
        ),
    );
    broker_peer
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect broker peer");
    let broker = McpBroker::start().expect("start broker");
    let handle = broker.handle();
    let key = pool_key("broker-mutation");
    handle
        .register(key.clone(), Box::new(broker_peer), false)
        .expect("register broker peer");
    assert!(matches!(
        handle.call_tool(
            key.clone(),
            "mutate",
            IJsonValue::parse_str(r#"{}"#).expect("arguments"),
            McpCancellationToken::default(),
        ),
        Err(McpError::UnknownEffect)
    ));
    assert!(matches!(
        handle.call_tool(
            key,
            "mutate",
            IJsonValue::parse_str(r#"{}"#).expect("arguments"),
            McpCancellationToken::default(),
        ),
        Err(McpError::Transport(_))
    ));
    drop(broker);
    assert_eq!(broker_closes.load(Ordering::SeqCst), 1);

    let reconnects = Arc::new(AtomicUsize::new(0));
    let mut resumable = McpClient::new(
        "resumable",
        RecoveryTransport::new(
            RecoveryScenario::ResumableTask,
            Arc::clone(&reconnects),
            Arc::new(AtomicUsize::new(0)),
        ),
    );
    resumable
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect resumable");
    let resumed = resumable
        .task_mutation_with_resumable_identity(
            "tasks/update",
            IJsonValue::parse_str(r#"{"status":"working","taskId":"task-1"}"#)
                .expect("task update"),
            "task-1",
        )
        .await
        .expect("loss queries resumable identity");
    assert!(!resumed.is_null());
    assert_eq!(reconnects.load(Ordering::SeqCst), 1);

    let identity_queries = Arc::new(AtomicUsize::new(0));
    let mut changed_resumable = McpClient::new(
        "changed-resumable",
        RecoveryTransport::new(
            RecoveryScenario::AuthorityChangedResumableTask,
            Arc::new(AtomicUsize::new(0)),
            Arc::new(AtomicUsize::new(0)),
        )
        .with_identity_queries(Arc::clone(&identity_queries)),
    );
    changed_resumable
        .connect(ProtocolMode::Legacy)
        .await
        .expect("connect changed resumable");
    assert!(matches!(
        changed_resumable
            .task_mutation_with_resumable_identity(
                "tasks/update",
                IJsonValue::parse_str(r#"{"status":"working","taskId":"task-1"}"#)
                    .expect("changed task update"),
                "task-1",
            )
            .await,
        Err(McpError::UnknownEffect)
    ));
    assert_eq!(identity_queries.load(Ordering::Acquire), 0);

    let closes = Arc::new(AtomicUsize::new(0));
    let mut protocol = McpClient::new(
        "protocol",
        RecoveryTransport::new(
            RecoveryScenario::Protocol,
            Arc::new(AtomicUsize::new(0)),
            Arc::clone(&closes),
        ),
    );
    assert!(matches!(
        protocol.connect(ProtocolMode::Legacy).await,
        Err(McpError::Protocol(_))
    ));
    assert_eq!(closes.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn continuation_preserves_arguments_and_uses_protocol_fields() {
    let mut client = fixture_client_with_args(&["strict-continuation"]);
    client.connect(ProtocolMode::Modern).await.unwrap();
    let result = client
        .continue_tool_call(
            "confirm_action",
            IJsonValue::parse_str(r#"{"operation":"publish"}"#).unwrap(),
            McpToolContinuation {
                request_state: IJsonValue::parse_str(r#""confirm:publish""#).unwrap(),
                input_responses: IJsonValue::parse_str(r#"{"confirmation":{"action":"accept"}}"#)
                    .unwrap(),
                round: 1,
                task_id: None,
            },
            McpCancellationToken::default(),
        )
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    assert_eq!(result["content"][0]["text"], "confirmed:publish");
    client.close().await.unwrap();
}

#[test]
fn subscription_capability_flags_survive_round_trip_without_inference() {
    let source = json!({"tools":{"listChanged":true},"prompts":{},"resources":{"listChanged":true,"subscribe":true},"subscriptions":{}});
    let capabilities: McpCapabilities = serde_json::from_value(source.clone()).unwrap();
    assert!(capabilities.tools_list_changed);
    assert!(!capabilities.prompts_list_changed);
    assert!(capabilities.resources_list_changed && capabilities.resources_subscribe);
    assert_eq!(serde_json::to_value(&capabilities).unwrap(), source);
    for invalid in [
        json!({"tools":{"listChanged":"true"}}),
        json!({"resources":{"subscribe":1}}),
    ] {
        assert!(serde_json::from_value::<McpCapabilities>(invalid).is_err());
    }
    let disabled: McpCapabilities =
        serde_json::from_value(json!({"tools":{"listChanged":false},"resources":{}})).unwrap();
    assert!(disabled.tools && disabled.resources);
    assert!(!disabled.tools_list_changed && !disabled.resources_subscribe);
}

#[test]
fn task_extension_capability_is_supported_without_rewriting_advertisement() {
    let source = json!({"tools":{},"extensions":{"io.modelcontextprotocol/tasks":{},"example.invalid/optional":{"version":2}}});
    let capabilities: McpCapabilities = serde_json::from_value(source.clone()).unwrap();
    assert!(!capabilities.tasks);
    assert!(capabilities.supports_tasks());
    assert_eq!(serde_json::to_value(capabilities).unwrap(), source);
    for extension in [json!(true), json!(null), json!("enabled")] {
        let capabilities: McpCapabilities = serde_json::from_value(
            json!({"extensions":{"io.modelcontextprotocol/tasks":extension}}),
        )
        .unwrap();
        assert!(!capabilities.supports_tasks());
    }
}

#[test]
fn task_poll_rejects_changed_identity_and_incomplete_terminal_state() {
    let base = json!({"taskId":"task-1","status":"completed","createdAt":"2026-09-04T00:00:00Z","lastUpdatedAt":"2026-09-04T00:00:01Z","result":{"content":[{"type":"text","text":"task:public"}]}});
    let validate = |value: serde_json::Value| {
        mcp::McpTask::validate_result(
            &IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap(),
            "task-1",
        )
    };
    assert_eq!(validate(base.clone()).unwrap().status, "completed");
    for (field, value) in [
        ("taskId", json!("other")),
        ("result", json!(null)),
        ("status", json!("done")),
        ("createdAt", json!("invalid")),
        ("pollIntervalMs", json!(-1)),
    ] {
        let mut malformed = base.clone();
        malformed[field] = value;
        assert!(validate(malformed).is_err());
    }
    let mut pending = base.clone();
    pending["status"] = json!("input_required");
    pending.as_object_mut().unwrap().remove("result");
    assert!(validate(pending.clone()).is_err());
    pending["inputRequests"] = json!({"confirmation":{"type":"confirmation"}});
    assert_eq!(validate(pending.clone()).unwrap().status, "input_required");
    pending["status"] = json!("working");
    assert!(validate(pending).is_err());
}

#[tokio::test]
async fn broker_task_operations_use_existing_peer_and_exact_task_identity() {
    let mut client = fixture_client_with_args(&["strict-task-routing"]);
    client.connect(ProtocolMode::Legacy).await.unwrap();
    let broker = McpBroker::start().unwrap();
    let handle = broker.handle();
    let key = pool_key("task-routing");
    handle
        .register(key.clone(), Box::new(client), false)
        .unwrap();
    for method in ["tasks/get", "tasks/update", "tasks/cancel"] {
        let params =
            IJsonValue::parse_str(r#"{"taskId":"task-1","inputResponses":{"answer":"yes"}}"#)
                .unwrap();
        let result = handle.task_operation(key.clone(), method, params).unwrap();
        assert_eq!(serde_json::to_value(result).unwrap()["taskId"], "task-1");
    }
    assert!(matches!(
        handle.task_operation(
            key.clone(),
            "tools/call",
            IJsonValue::parse_str("{}").unwrap()
        ),
        Err(McpError::Unsupported(_))
    ));
    handle.remove(key.clone()).unwrap();
    assert!(
        handle
            .task_operation(
                key,
                "tasks/get",
                IJsonValue::parse_str(r#"{"taskId":"task-1"}"#).unwrap()
            )
            .is_err()
    );
}

/// The released official SDKs speak SEP-1686 as shipped in 2025-11-25:
/// `ttl`/`pollInterval` keys, a status-only `tasks/get`, and the payload
/// behind `tasks/result`. The client adopts the Kernel keys and completes a
/// finished poll with exactly one `tasks/result`, so the supervisor and worker
/// keep seeing the Kernel task vocabulary (inline `result`, `pollIntervalMs`).
#[tokio::test]
async fn official_sdk_task_shape_is_adopted_and_completed_with_tasks_result() {
    let mut client = fixture_client_with_args(&["official-task-shape"]);
    client.connect(ProtocolMode::Legacy).await.unwrap();
    let created = client
        .call_tool_augmented(
            "echo",
            IJsonValue::parse_str(r#"{"value":"x"}"#).unwrap(),
            None,
            600_000,
            McpCancellationToken::default(),
        )
        .await
        .expect("augmented call");
    let created = serde_json::to_value(created).unwrap();
    assert_eq!(created["task"]["taskId"], "task-official");
    assert_eq!(
        created["task"]["pollIntervalMs"], 50,
        "pollInterval adopted: {created}"
    );
    assert_eq!(created["task"]["ttlMs"], 60000, "ttl adopted: {created}");
    let first = client
        .task_operation(
            "tasks/get",
            IJsonValue::parse_str(r#"{"taskId":"task-official"}"#).unwrap(),
        )
        .await
        .expect("first poll");
    let first = serde_json::to_value(first).unwrap();
    assert_eq!(first["status"], "working");
    assert_eq!(first["pollIntervalMs"], 50);
    mcp::McpTask::validate_result(
        &IJsonValue::parse(&serde_json::to_vec(&first).unwrap()).unwrap(),
        "task-official",
    )
    .expect("working poll validates");
    let done = client
        .task_operation(
            "tasks/get",
            IJsonValue::parse_str(r#"{"taskId":"task-official"}"#).unwrap(),
        )
        .await
        .expect("completed poll");
    let done = serde_json::to_value(done).unwrap();
    assert_eq!(done["status"], "completed");
    assert_eq!(
        done["result"]["content"][0]["text"], "official done",
        "tasks/result payload inlined: {done}"
    );
    mcp::McpTask::validate_result(
        &IJsonValue::parse(&serde_json::to_vec(&done).unwrap()).unwrap(),
        "task-official",
    )
    .expect("completed poll validates");
    client.close().await.unwrap();
}

/// A tool declaring `execution.taskSupport = required` is parsed as such; the
/// augmented call carries `task: {ttl}` and returns the created task, a plain
/// call is refused by the server, and the broker routes the augmented call.
#[tokio::test]
async fn task_required_tools_are_declared_and_called_with_task_augmentation() {
    let mut client = fixture_client_with_args(&["task-augmented"]);
    client.connect(ProtocolMode::Legacy).await.unwrap();
    let tools = client.list_tools().await.expect("tools");
    assert_eq!(tools.len(), 1);
    assert!(tools[0].requires_task());
    assert_eq!(
        tools[0].execution.as_ref().map(|e| e.task_support),
        Some(mcp::McpTaskSupport::Required)
    );
    let plain = client
        .call_tool(
            "echo",
            IJsonValue::parse_str(r#"{"value":"x"}"#).unwrap(),
            McpCancellationToken::default(),
        )
        .await;
    assert!(
        matches!(plain, Err(McpError::Remote { .. })),
        "server refuses a plain call: {plain:?}"
    );
    let created = client
        .call_tool_augmented(
            "echo",
            IJsonValue::parse_str(r#"{"value":"x"}"#).unwrap(),
            None,
            600_000,
            McpCancellationToken::default(),
        )
        .await
        .expect("augmented call");
    let created = serde_json::to_value(created).unwrap();
    assert_eq!(created["task"]["taskId"], "task-aug");
    assert_eq!(created["task"]["status"], "working");
    assert!(
        client
            .call_tool_augmented(
                "echo",
                IJsonValue::parse_str("{}").unwrap(),
                None,
                0,
                McpCancellationToken::default()
            )
            .await
            .is_err(),
        "ttl must be positive"
    );
    let broker = McpBroker::start().unwrap();
    let handle = broker.handle();
    let key = pool_key("task-augmented");
    handle
        .register(key.clone(), Box::new(client), false)
        .unwrap();
    let routed = handle
        .call_tool_augmented(
            key.clone(),
            "echo",
            IJsonValue::parse_str(r#"{"value":"y"}"#).unwrap(),
            None,
            600_000,
            McpCancellationToken::default(),
        )
        .expect("broker augmented call");
    assert_eq!(
        serde_json::to_value(routed).unwrap()["task"]["taskId"],
        "task-aug"
    );
    let first = handle
        .task_operation(
            key.clone(),
            "tasks/get",
            IJsonValue::parse_str(r#"{"taskId":"task-aug"}"#).unwrap(),
        )
        .expect("first poll");
    let first = serde_json::to_value(first).unwrap();
    assert_eq!(first["status"], "working");
    assert_eq!(first["pollIntervalMs"], 1500);
    let polled = handle
        .task_operation(
            key.clone(),
            "tasks/get",
            IJsonValue::parse_str(r#"{"taskId":"task-aug"}"#).unwrap(),
        )
        .expect("second poll");
    assert_eq!(serde_json::to_value(polled).unwrap()["status"], "completed");
    let tool_json: McpTool =
        serde_json::from_value(serde_json::json!({"name":"plain","inputSchema":{"type":"object"}}))
            .unwrap();
    assert!(!tool_json.requires_task() && tool_json.execution.is_none());
    handle.remove(key).unwrap();
}

/// Two sessions saving the same server each append an entry: the registry
/// keeps the last (newest) one per reference, restores the sorted order, and
/// the next mutation publishes the repaired file. A save is idempotent by
/// reference, so the Kernel itself never writes a repeat.
#[test]
fn registry_file_keeps_the_last_entry_for_a_repeated_reference_and_saves_it_once() {
    let root = TempDir::new().expect("temp");
    let store = McpRegistryStore::new(root.path());
    let server = |name: &str, command: &str, enabled: bool| McpServerConfig {
        reference: McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: McpScope::User,
            name: name.to_owned(),
        },
        transport: McpTransportConfig::Stdio {
            command: vec![command.to_owned()],
            cwd: None,
            environment: BTreeMap::new(),
        },
        enabled,
        always_on: false,
        protocol_mode: ProtocolMode::Auto,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    };
    // Hand-written: `memory` twice (older first), `dxf-editor` twice, unsorted.
    let registry = McpRegistry {
        format: 1,
        servers: vec![
            server("memory", "/usr/bin/true", true),
            server("dxf-editor", "/usr/bin/false", true),
            server("memory", "/usr/bin/env", false),
            server("dxf-editor", "/usr/bin/true", true),
        ],
    };
    let mut bytes = serde_json_canonicalizer::to_vec(&registry).expect("canonical");
    bytes.push(b'\n');
    let path = root.path().join("mcp-servers.json");
    std::fs::write(&path, &bytes).expect("write");
    let loaded = store.load().expect("repeated references load");
    assert_eq!(
        loaded.servers,
        vec![
            server("dxf-editor", "/usr/bin/true", true),
            server("memory", "/usr/bin/env", false),
        ]
    );
    assert_eq!(store.list("ws").expect("list").len(), 2);

    // Saving one of them again writes exactly one entry per reference and
    // leaves the file canonical and sorted for the strict reader.
    store
        .mutate_idempotent(
            "rpc-dedupe",
            &McpManagementMutation::Save {
                server: server("memory", "/usr/bin/env", true),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save");
    let published = std::fs::read(&path).expect("published bytes");
    let decoded: McpRegistry = serde_json::from_slice(&published[..published.len() - 1]).unwrap();
    assert_eq!(decoded.servers.len(), 2);
    assert!(decoded.servers[1].enabled);
    assert!(decoded.validate().is_ok());
}

#[test]
fn registry_file_tolerates_missing_trailing_newline_and_names_the_file_on_corruption() {
    let root = TempDir::new().expect("temp");
    let store = McpRegistryStore::new(root.path());
    let config = McpServerConfig {
        reference: McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: McpScope::User,
            name: "fixture".to_owned(),
        },
        transport: McpTransportConfig::Stdio {
            command: vec!["/usr/bin/true".to_owned()],
            cwd: None,
            environment: BTreeMap::new(),
        },
        enabled: true,
        always_on: false,
        protocol_mode: ProtocolMode::Auto,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    };
    store
        .mutate_idempotent(
            "rpc-1",
            &McpManagementMutation::Save {
                server: config.clone(),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("publish");
    let path = root.path().join("mcp-servers.json");
    let canonical = std::fs::read(&path).expect("registry bytes");
    assert!(canonical.ends_with(b"\n"));

    // A hand edit or an external writer that drops the final newline is the
    // observed production failure; the body is still canonical.
    std::fs::write(&path, &canonical[..canonical.len() - 1]).expect("strip newline");
    assert_eq!(store.load().expect("tolerant load").servers, vec![config]);

    // Anything else stays rejected, and the failure names the file.
    std::fs::write(&path, b"{\"format\":1,\"servers\":[]} \n").expect("corrupt");
    let error = store.load().expect_err("corrupt registry").to_string();
    assert!(error.contains("mcp-servers.json"), "{error}");
    assert!(error.contains("not canonical"), "{error}");
}
