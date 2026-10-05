use mcp::{
    JsonRpcRequest, JsonRpcResponse, McpClient, McpError, McpPeer, McpPeerFuture, McpTransport,
    ProtocolMode,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Copy)]
enum Failure {
    Remote,
    WireProtocol,
    DiscoveryProtocol,
    RetryRequest,
    RetryConnect,
}
struct Transport {
    failure: Failure,
    events: Arc<Mutex<Vec<&'static str>>>,
}
impl McpTransport for Transport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        self.events.lock().unwrap().push("request");
        Box::pin(async move {
            match self.failure {
                Failure::Remote => Err(McpError::Remote { code: -1, message: "failure".into(), data: None }),
                Failure::WireProtocol => Err(McpError::Protocol("bad frame".into())),
                Failure::DiscoveryProtocol => Ok(serde_json::from_value(json!({
                    "jsonrpc":"2.0", "id":request.id,
                    "result":{"resultType":"invalid", "supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],
                        "_meta":{"io.modelcontextprotocol/serverInfo":{"name":"test","version":"1"}}}
                })).unwrap()),
                Failure::RetryRequest | Failure::RetryConnect => Err(McpError::Transport("lost".into())),
            }
        })
    }
    fn notify<'a>(&'a mut self, _: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }
    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.events.lock().unwrap().push("close");
        Box::pin(async { Ok(()) })
    }
    fn reconnect(&mut self) -> McpPeerFuture<'_, ()> {
        self.events.lock().unwrap().push("reconnect");
        Box::pin(async move {
            if matches!(self.failure, Failure::RetryConnect) {
                Err(McpError::Transport("reconnect failed".into()))
            } else {
                Ok(())
            }
        })
    }
}

#[tokio::test]
async fn every_terminal_handshake_failure_closes_exactly_once() {
    for failure in [
        Failure::Remote,
        Failure::WireProtocol,
        Failure::DiscoveryProtocol,
        Failure::RetryRequest,
        Failure::RetryConnect,
    ] {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut client = McpClient::new(
            "test",
            Transport {
                failure,
                events: events.clone(),
            },
        );
        assert!(client.connect(ProtocolMode::Modern).await.is_err());
        client.close().await.unwrap();
        let expected = match failure {
            Failure::RetryRequest => vec!["request", "reconnect", "request", "close"],
            Failure::RetryConnect => vec!["request", "reconnect", "close"],
            _ => vec!["request", "close"],
        };
        assert_eq!(*events.lock().unwrap(), expected);
    }
}

#[derive(Clone, Copy)]
enum PendingStage {
    Initialize,
    Notification,
    Reconnect,
}
struct PendingTransport {
    stage: PendingStage,
    events: Arc<Mutex<Vec<&'static str>>>,
    reached: Arc<tokio::sync::Notify>,
}
impl McpTransport for PendingTransport {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        self.events.lock().unwrap().push("request");
        Box::pin(async move {
            match self.stage {
                PendingStage::Initialize => {
                    self.reached.notify_one();
                    std::future::pending().await
                }
                PendingStage::Reconnect => Err(McpError::Transport("lost".into())),
                PendingStage::Notification => Ok(serde_json::from_value(json!({
                    "jsonrpc":"2.0", "id":request.id,
                    "result":{"protocolVersion":mcp::LEGACY_PROTOCOL_VERSION,
                        "capabilities":{},"serverInfo":{"name":"test","version":"1"}}
                }))
                .unwrap()),
            }
        })
    }
    fn notify<'a>(&'a mut self, _: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        self.events.lock().unwrap().push("notify");
        self.reached.notify_one();
        Box::pin(std::future::pending())
    }
    fn reconnect(&mut self) -> McpPeerFuture<'_, ()> {
        self.events.lock().unwrap().push("reconnect");
        self.reached.notify_one();
        Box::pin(std::future::pending())
    }
    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        self.events.lock().unwrap().push("close");
        Box::pin(async { Ok(()) })
    }
}

#[tokio::test]
async fn cancellation_at_each_handshake_stage_closes_once_before_return() {
    for stage in [
        PendingStage::Initialize,
        PendingStage::Notification,
        PendingStage::Reconnect,
    ] {
        let events = Arc::new(Mutex::new(Vec::new()));
        let reached = Arc::new(tokio::sync::Notify::new());
        let cancellation = mcp::McpCancellationToken::default();
        let trigger = cancellation.clone();
        let dispatched = reached.clone();
        let canceller = tokio::spawn(async move {
            dispatched.notified().await;
            trigger.cancel();
        });
        let mut client = McpClient::new(
            "test",
            PendingTransport {
                stage,
                events: events.clone(),
                reached,
            },
        );
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            client.connect_cancellable(ProtocolMode::Legacy, cancellation),
        )
        .await
        .unwrap();
        assert!(matches!(result, Err(McpError::Cancelled)));
        assert_eq!(events.lock().unwrap().last(), Some(&"close"));
        client.close().await.unwrap();
        let expected = match stage {
            PendingStage::Initialize => vec!["request", "close"],
            PendingStage::Notification => vec!["request", "notify", "close"],
            PendingStage::Reconnect => vec!["request", "reconnect", "close"],
        };
        assert_eq!(*events.lock().unwrap(), expected);
        canceller.await.unwrap();
    }
}

#[tokio::test]
async fn pre_cancelled_handshake_closes_without_sending() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let cancellation = mcp::McpCancellationToken::default();
    cancellation.cancel();
    let mut client = McpClient::new(
        "test",
        PendingTransport {
            stage: PendingStage::Initialize,
            events: events.clone(),
            reached: Arc::new(tokio::sync::Notify::new()),
        },
    );
    assert!(matches!(
        client
            .connect_cancellable(ProtocolMode::Legacy, cancellation)
            .await,
        Err(McpError::Cancelled)
    ));
    assert_eq!(*events.lock().unwrap(), vec!["close"]);
}
