use mcp::{HttpTransport, McpClient, ProtocolMode};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn modern_scoped_call_does_not_claim_known_failure_after_dispatch() {
    use mcp::{McpCancellationToken, McpError};
    for wrong_id in [false, true] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (_, bytes) = read_http_request(&mut socket).await;
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            let body = json!({"jsonrpc":"2.0","id":request["id"],"result":{
                "resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],
                "capabilities":{"tools":{}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"effect","version":"1"}}
            }}).to_string();
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
            drop(socket);
            let (mut socket, _) = listener.accept().await.unwrap();
            let (_, bytes) = read_http_request(&mut socket).await;
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(request["method"], "tools/call");
            if wrong_id {
                let body =
                    json!({"jsonrpc":"2.0","id":request["id"].as_u64().unwrap()+1,"result":{}})
                        .to_string();
                socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
            }
            // The effect could have committed before either a disconnect or
            // an unmatched reply. Neither establishes a safe retry.
        });
        let transport = HttpTransport::new(
            format!("http://{address}/mcp").parse().unwrap(),
            &BTreeMap::new(),
            None,
            true,
        )
        .unwrap();
        let mut client = McpClient::new("effect", transport);
        client.connect(ProtocolMode::Modern).await.unwrap();
        let cancelled = McpCancellationToken::default();
        cancelled.cancel();
        assert!(matches!(
            client
                .call_tool_scoped(
                    "must_not_dispatch",
                    schema::IJsonValue::parse(b"{}").unwrap(),
                    cancelled
                )
                .await,
            Err(McpError::Cancelled)
        ));
        let arguments = schema::IJsonValue::parse(b"{}").unwrap();
        let result = tokio::time::timeout(
            Duration::from_secs(5),
            client.call_tool_scoped("write", arguments, McpCancellationToken::default()),
        )
        .await
        .unwrap();
        assert!(matches!(result, Err(McpError::UnknownEffect)), "{result:?}");
        server.await.unwrap();
    }
}

#[tokio::test]
async fn modern_scoped_http_calls_overlap_and_cancel_only_their_own_socket() {
    use mcp::{McpCancellationToken, McpError};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let received = std::sync::Arc::new(tokio::sync::Notify::new());
    let signal = received.clone();
    let server = tokio::spawn(async move {
        let (mut discovery, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut discovery).await;
        let r: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(r["method"], "server/discover");
        let body=json!({"jsonrpc":"2.0","id":r["id"],"result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"scoped","version":"1"}}}}).to_string();
        discovery.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        let mut pending = Vec::new();
        for _ in 0..2 {
            let (mut socket, _) = tokio::time::timeout(Duration::from_secs(3), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let (headers, bytes) = read_http_request(&mut socket).await;
            assert!(!headers.contains("mcp-session-id:"));
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            pending.push((socket, request));
        }
        assert_ne!(pending[0].1["id"], pending[1].1["id"]);
        // Neither response is released until BOTH requests have arrived.
        for (mut socket, r) in pending {
            let body=json!({"jsonrpc":"2.0","id":r["id"],"result":{"who":r["params"]["arguments"]["who"]}}).to_string();
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        }
        let (mut slow, _) = listener.accept().await.unwrap();
        read_http_request(&mut slow).await;
        slow.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {\"jsonrpc\":\"2.0\",\"method\":\"notifications/tools/list_changed\",\"params\":{}}\n\n").await.unwrap();
        signal.notify_one();
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), slow.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
        let (mut survivor, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut survivor).await;
        let r: Value = serde_json::from_slice(&bytes).unwrap();
        let body = json!({"jsonrpc":"2.0","id":r["id"],"result":{"who":"survivor"}}).to_string();
        survivor.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("scoped", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    let args = |who: &str| {
        schema::IJsonValue::parse(&serde_json::to_vec(&json!({"who":who})).unwrap()).unwrap()
    };
    let (first, second) = tokio::join!(
        client.call_tool_scoped("问候", args("one"), McpCancellationToken::default()),
        client.call_tool_scoped("问候", args("two"), McpCancellationToken::default())
    );
    for (response, who) in [(first.unwrap(), "one"), (second.unwrap(), "two")] {
        assert_eq!(serde_json::to_value(response).unwrap()["who"], who);
    }
    let token = McpCancellationToken::default();
    let trigger = token.clone();
    let before = client.catalog_generation();
    let (cancelled, ()) = tokio::join!(
        client.call_tool_scoped("问候", args("cancelled"), token),
        async {
            received.notified().await;
            tokio::time::timeout(Duration::from_secs(3), async {
                while client.catalog_generation() == before {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
            trigger.cancel();
        }
    );
    assert!(matches!(cancelled, Err(McpError::UnknownEffect)));
    assert_eq!(
        client.catalog_generation(),
        before + 1,
        "received invalidation must survive request cancellation"
    );
    let response = client
        .call_tool_scoped("问候", args("survivor"), McpCancellationToken::default())
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(response).unwrap()["who"], "survivor");
    server.await.unwrap();
}

#[tokio::test]
async fn broker_modern_calls_overlap_and_cancel_without_evicting_siblings() {
    use mcp::{McpCancellationToken, McpError};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let received = std::sync::Arc::new(tokio::sync::Notify::new());
    let signal = received.clone();
    let server = tokio::spawn(async move {
        let (mut discovery, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut discovery).await;
        let r: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(r["method"], "server/discover");
        let body=json!({"jsonrpc":"2.0","id":r["id"],"result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"scoped","version":"1"}}}}).to_string();
        discovery.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        let mut pending = Vec::new();
        for _ in 0..2 {
            let (mut socket, _) = tokio::time::timeout(Duration::from_secs(3), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let (headers, bytes) = read_http_request(&mut socket).await;
            assert!(!headers.contains("mcp-session-id:"));
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(
                request["params"]["_meta"]["io.tekes/idempotencyKey"],
                "b".repeat(64)
            );
            pending.push((socket, request));
        }
        assert_ne!(pending[0].1["id"], pending[1].1["id"]);
        // Neither response is released until BOTH requests have arrived.
        for (mut socket, r) in pending {
            let body=json!({"jsonrpc":"2.0","id":r["id"],"result":{"who":r["params"]["arguments"]["who"]}}).to_string();
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        }
        let (mut slow, _) = listener.accept().await.unwrap();
        read_http_request(&mut slow).await;
        slow.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {\"jsonrpc\":\"2.0\",\"method\":\"notifications/tools/list_changed\",\"params\":{}}\n\n").await.unwrap();
        signal.notify_one();
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), slow.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
        let (mut survivor, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut survivor).await;
        let r: Value = serde_json::from_slice(&bytes).unwrap();
        let body = json!({"jsonrpc":"2.0","id":r["id"],"result":{"who":"survivor"}}).to_string();
        survivor.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("scoped", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    let broker = mcp::McpBroker::start().unwrap();
    let handle = broker.handle();
    let key = mcp::McpPoolKey {
        workspace: "ws".into(),
        scope: "project".into(),
        server: "scoped".into(),
        config_digest: "a".repeat(64),
        authorization_identity: "anonymous".into(),
        protocol_mode: "modern".into(),
        plugin_generation: None,
    };
    handle
        .register(key.clone(), Box::new(client), false)
        .unwrap();
    let call = |name: &str, arguments: schema::IJsonValue, cancellation: McpCancellationToken| {
        let handle = handle.clone();
        let key = key.clone();
        let name = name.to_owned();
        async move {
            tokio::task::spawn_blocking(move || {
                handle.call_tool_with_context(
                    key,
                    name,
                    arguments,
                    mcp::McpToolCallContext {
                        idempotency_key: "b".repeat(64),
                    },
                    cancellation,
                )
            })
            .await
            .unwrap()
        }
    };
    let args = |who: &str| {
        schema::IJsonValue::parse(&serde_json::to_vec(&json!({"who":who})).unwrap()).unwrap()
    };
    let (first, second) = tokio::join!(
        call("问候", args("one"), McpCancellationToken::default()),
        call("问候", args("two"), McpCancellationToken::default())
    );
    for (response, who) in [(first.unwrap(), "one"), (second.unwrap(), "two")] {
        assert_eq!(serde_json::to_value(response).unwrap()["who"], who);
    }
    let token = McpCancellationToken::default();
    let trigger = token.clone();
    let before = handle.catalog_generation(key.clone()).unwrap().unwrap();
    let (cancelled, ()) = tokio::join!(call("问候", args("cancelled"), token), async {
        received.notified().await;
        tokio::time::timeout(Duration::from_secs(3), async {
            while handle.catalog_generation(key.clone()).unwrap().unwrap() == before {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        trigger.cancel();
    });
    assert!(matches!(cancelled, Err(McpError::UnknownEffect)));
    assert_eq!(
        handle.catalog_generation(key.clone()).unwrap().unwrap(),
        before + 1,
        "received invalidation must survive request cancellation"
    );
    let response = call("问候", args("survivor"), McpCancellationToken::default())
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(response).unwrap()["who"], "survivor");
    server.await.unwrap();
}

#[tokio::test]
async fn sse_progress_is_observed_before_server_releases_final_response() {
    use mcp::{JsonRpcRequest, McpTransport};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let observed = std::sync::Arc::new(tokio::sync::Notify::new());
    let signal = observed.clone();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let (headers, body) = read_http_request(&mut stream).await;
        assert!(headers.contains("mcp-protocol-version: 2026-07-28"));
        assert!(headers.contains("mcp-method: tools/call"));
        let request: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(request["id"], 77);
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {\"jsonrpc\":\"2.0\",\"method\":\"notifications/progress\",\"params\":{\"progress\":1}}\n\n").await.unwrap();
        // This cannot complete if notifications are delivered only after final.
        tokio::time::timeout(Duration::from_secs(3), observed.notified())
            .await
            .unwrap();
        stream
            .write_all(b"data: {\"jsonrpc\":\"2.0\",\"id\":77,\"result\":{\"content\":[]}}\n\n")
            .await
            .unwrap();
    });
    let mut transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap()
    .with_notification_handler(move |method| {
        assert_eq!(method, "notifications/progress");
        signal.notify_one();
    });
    let request:JsonRpcRequest=serde_json::from_value(json!({"jsonrpc":"2.0","id":77,"method":"tools/call","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"},"name":"问候","arguments":{"who":"progress"}}})).unwrap();
    let response = transport
        .request(&request, Duration::from_secs(5))
        .await
        .unwrap();
    response.validate_for(77).unwrap();
    assert_eq!(
        transport.take_notifications(),
        vec!["notifications/progress"]
    );
    server.await.unwrap();
}

async fn probe(
    mode: ProtocolMode,
    discovery_status: u16,
    discovery: Value,
    fallback: bool,
) -> bool {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let expected = if fallback {
            vec!["server/discover", "initialize", "notifications/initialized"]
        } else if discovery_status != 200
            && !(discovery_status < 500
                && discovery_status != 401
                && discovery_status != 403
                && discovery.pointer("/error/code").is_some())
        {
            vec!["server/discover", "server/discover"]
        } else {
            vec!["server/discover"]
        };
        for (index, method) in expected.iter().enumerate() {
            let (mut stream, _) = tokio::time::timeout(Duration::from_secs(3), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let mut bytes = Vec::new();
            let (end, length) = loop {
                let mut chunk = [0; 4096];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&chunk[..count]);
                if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                    let length = headers
                        .lines()
                        .find_map(|line| line.strip_prefix("content-length: "))
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    break (end + 4, length);
                }
            };
            while bytes.len() < end + length {
                let mut chunk = [0; 4096];
                let n = stream.read(&mut chunk).await.unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&chunk[..n]);
            }
            let body: Value = serde_json::from_slice(&bytes[end..end + length]).unwrap();
            assert_eq!(body["method"], *method);
            if *method == "server/discover" {
                let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                assert!(headers.contains("mcp-protocol-version:"));
                assert!(headers.contains("mcp-method: server/discover"));
            } else {
                assert!(
                    body.pointer("/params/_meta/io.modelcontextprotocol/protocolVersion")
                        .is_none()
                );
            }
            let status = if *method == "server/discover" {
                discovery_status
            } else {
                200
            };
            let mut reply = if *method == "server/discover" {
                discovery.clone()
            } else {
                json!({"jsonrpc":"2.0","result":{"protocolVersion":mcp::LEGACY_PROTOCOL_VERSION,"capabilities":{},"serverInfo":{"name":"local","version":"1"}}})
            };
            reply["id"] = body["id"].clone();
            let text = if index == 2 {
                String::new()
            } else {
                reply.to_string()
            };
            let response = format!(
                "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{text}",
                text.len()
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
        // A wrong downgrade must not hide behind a single-request fixture.
        assert!(
            tokio::time::timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err(),
            "unexpected extra HTTP request"
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("local", transport);
    let result = client.connect(mode).await;
    if result.is_ok() {
        assert_eq!(
            client.protocol_version(),
            if fallback {
                mcp::LEGACY_PROTOCOL_VERSION
            } else {
                mcp::MODERN_PROTOCOL_VERSION
            }
        );
    }
    server.await.unwrap();
    result.is_ok()
}

#[tokio::test]
async fn http_auto_discovers_modern_and_only_downgrades_on_protocol_evidence() {
    let modern = json!({"jsonrpc":"2.0","result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"local","version":"1"}}}});
    assert!(probe(ProtocolMode::Auto, 200, modern, false).await);
    let missing = json!({"jsonrpc":"2.0","error":{"code":-32601,"message":"method not found"}});
    assert!(probe(ProtocolMode::Auto, 200, missing.clone(), true).await);
    assert!(!probe(ProtocolMode::Modern, 200, missing, false).await);
    let unsupported = |versions| json!({"jsonrpc":"2.0","error":{"code":-32022,"message":"unsupported","data":{"supported":versions}}});
    assert!(
        probe(
            ProtocolMode::Auto,
            200,
            unsupported(json!([mcp::LEGACY_PROTOCOL_VERSION])),
            true
        )
        .await
    );
    assert!(
        !probe(
            ProtocolMode::Auto,
            200,
            unsupported(json!(["2027-01-01"])),
            false
        )
        .await
    );
    for status in [400, 401, 403, 404, 405, 500, 503] {
        assert!(
            !probe(
                ProtocolMode::Auto,
                status,
                json!({"error":"HTTP failure"}),
                false
            )
            .await
        );
    }
}

#[tokio::test]
async fn discovery_versions_and_http_error_bodies_preserve_era_evidence() {
    let discovered = json!({"jsonrpc":"2.0","result":{"resultType":"complete","supportedVersions":[mcp::LEGACY_PROTOCOL_VERSION],"capabilities":{},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"local","version":"1"}}}});
    assert!(probe(ProtocolMode::Auto, 200, discovered.clone(), true).await);
    assert!(!probe(ProtocolMode::Modern, 200, discovered, false).await);
    let missing = json!({"jsonrpc":"2.0","error":{"code":-32601,"message":"method missing"}});
    for status in [400, 404, 405] {
        assert!(probe(ProtocolMode::Auto, status, missing.clone(), true).await);
    }
    for status in [401, 403, 500, 503] {
        assert!(!probe(ProtocolMode::Auto, status, missing.clone(), false).await);
    }
    assert!(!probe(ProtocolMode::Modern, 400, missing, false).await);
    let unsupported = json!({"jsonrpc":"2.0","error":{"code":-32022,"message":"unsupported","data":{"supported":[mcp::LEGACY_PROTOCOL_VERSION]}}});
    assert!(probe(ProtocolMode::Auto, 400, unsupported, true).await);
    let textual = json!({"jsonrpc":"2.0","error":{"code":-32600,"message":"Unsupported modern version; supported legacy 2025-06-18"}});
    assert!(!probe(ProtocolMode::Auto, 400, textual, false).await);
}

#[test]
fn configured_reserved_mcp_headers_cannot_spoof_protocol_or_params() {
    for name in [
        "MCP-Protocol-Version",
        "Mcp-Method",
        "Mcp-Name",
        "Mcp-Session-Id",
        "Mcp-Param-Who",
    ] {
        let headers = BTreeMap::from([(name.to_owned(), "spoofed".to_owned())]);
        assert!(
            HttpTransport::new(
                "http://127.0.0.1:1/mcp".parse().unwrap(),
                &headers,
                None,
                true
            )
            .is_err(),
            "reserved {name}"
        );
    }
}

#[tokio::test]
async fn legacy_expired_session_reinitializes_and_close_deletes_once() {
    use mcp::McpPeer;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let methods = [
            "initialize",
            "notifications/initialized",
            "tools/list",
            "initialize",
            "notifications/initialized",
            "tools/list",
            "DELETE",
        ];
        for (index, method) in methods.into_iter().enumerate() {
            let (mut stream, _) = tokio::time::timeout(Duration::from_secs(5), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let mut bytes = Vec::new();
            let (end, length) = loop {
                let mut chunk = [0; 4096];
                let n = stream.read(&mut chunk).await.unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&chunk[..n]);
                if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                    let length = headers
                        .lines()
                        .find_map(|line| line.strip_prefix("content-length: "))
                        .map(|s| s.parse::<usize>().unwrap())
                        .unwrap_or(0);
                    break (end + 4, length);
                }
            };
            while bytes.len() < end + length {
                let mut chunk = [0; 4096];
                let n = stream.read(&mut chunk).await.unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&chunk[..n]);
            }
            let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
            let (status, body, session) = if method == "DELETE" {
                assert!(headers.starts_with("delete /mcp "));
                assert!(headers.contains("mcp-session-id: session-2"));
                assert!(headers.contains(&format!(
                    "mcp-protocol-version: {}",
                    mcp::LEGACY_PROTOCOL_VERSION
                )));
                (405, String::new(), "") // Unsupported DELETE still closes locally.
            } else {
                let request: Value = serde_json::from_slice(&bytes[end..end + length]).unwrap();
                assert_eq!(request["method"], method);
                if method == "initialize" {
                    assert!(!headers.contains("mcp-session-id:"));
                    let session = if index == 0 {
                        "Mcp-Session-Id: session-1\r\n"
                    } else {
                        "Mcp-Session-Id: session-2\r\n"
                    };
                    (200, json!({"jsonrpc":"2.0","id":request["id"],"result":{"protocolVersion":mcp::LEGACY_PROTOCOL_VERSION,"capabilities":{"tools":{}},"serverInfo":{"name":"local","version":"1"}}}).to_string(), session)
                } else {
                    assert!(headers.contains(if index < 3 {
                        "mcp-session-id: session-1"
                    } else {
                        "mcp-session-id: session-2"
                    }));
                    if index == 2 {
                        (404, String::new(), "")
                    } else if method == "notifications/initialized" {
                        (202, String::new(), "")
                    } else {
                        (
                            200,
                            json!({"jsonrpc":"2.0","id":request["id"],"result":{"tools":[]}})
                                .to_string(),
                            "",
                        )
                    }
                }
            };
            stream.write_all(format!("HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\n{session}Content-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
        }
        assert!(
            tokio::time::timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err()
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("local", transport);
    client.connect(ProtocolMode::Legacy).await.unwrap();
    assert!(client.list_tools().await.unwrap().is_empty());
    client.close().await.unwrap();
    client.close().await.unwrap();
    server.await.unwrap();
}

async fn read_http_request(stream: &mut tokio::net::TcpStream) -> (String, Vec<u8>) {
    let mut bytes = Vec::new();
    loop {
        if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
            let length = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
                .map(|s| s.parse::<usize>().unwrap())
                .unwrap_or(0);
            if bytes.len() >= end + 4 + length {
                return (headers, bytes[end + 4..end + 4 + length].to_vec());
            }
        }
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&chunk[..n]);
    }
}

#[tokio::test]
async fn http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once() {
    use mcp::{McpCancellationToken, McpError, McpPeer};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let cancellation = McpCancellationToken::default();
    let trigger = cancellation.clone();
    let server = tokio::spawn(async move {
        let (mut initialize, _) = listener.accept().await.unwrap();
        let (_, body) = read_http_request(&mut initialize).await;
        let request: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(request["method"], "initialize");
        let response = json!({"jsonrpc":"2.0","id":request["id"],"result":{
            "protocolVersion":mcp::LEGACY_PROTOCOL_VERSION,"capabilities":{},"serverInfo":{"name":"local","version":"1"}
        }}).to_string();
        initialize.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nMcp-Session-Id: cancel-session\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}", response.len()).as_bytes()).await.unwrap();
        drop(initialize);
        let (mut notification, _) = listener.accept().await.unwrap();
        let (headers, body) = read_http_request(&mut notification).await;
        assert!(headers.contains("mcp-session-id: cancel-session"));
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap()["method"],
            "notifications/initialized"
        );
        // The server deliberately never replies to initialized.
        trigger.cancel();
        let (mut deletion, _) = listener.accept().await.unwrap();
        let (headers, body) = read_http_request(&mut deletion).await;
        assert!(headers.starts_with("delete /mcp "));
        assert!(headers.contains("mcp-session-id: cancel-session"));
        assert!(body.is_empty());
        deletion
            .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        let mut byte = [0; 1];
        assert_eq!(
            notification.read(&mut byte).await.unwrap(),
            0,
            "cancelled request socket remains open"
        );
        assert!(
            tokio::time::timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err(),
            "unexpected request after cancellation"
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("local", transport);
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        client.connect_cancellable(ProtocolMode::Legacy, cancellation),
    )
    .await
    .unwrap();
    assert!(matches!(result, Err(McpError::Cancelled)));
    client.close().await.unwrap();
    tokio::time::timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn legacy_http_connects_lists_and_calls_tool_over_real_socket() {
    use mcp::{McpCancellationToken, McpPeer};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        for method in [
            "initialize",
            "notifications/initialized",
            "tools/list",
            "tools/call",
            "DELETE",
        ] {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (headers, bytes) = read_http_request(&mut stream).await;
            if method == "DELETE" {
                assert!(headers.starts_with("delete /mcp "));
                assert!(headers.contains("mcp-session-id: round-trip"));
                stream
                    .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
                    .await
                    .unwrap();
                continue;
            }
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(request["method"], method);
            if method != "initialize" {
                assert!(headers.contains("mcp-session-id: round-trip"));
            }
            if method == "notifications/initialized" {
                stream
                    .write_all(
                        b"HTTP/1.1 202 Accepted\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    )
                    .await
                    .unwrap();
                continue;
            }
            let result = match method {
                "initialize" => {
                    json!({"protocolVersion":mcp::LEGACY_PROTOCOL_VERSION,"capabilities":{"tools":{}},"serverInfo":{"name":"local","version":"1"}})
                }
                "tools/list" => {
                    json!({"tools":[{"name":"greet","description":"Greeting","inputSchema":{"type":"object","properties":{"who":{"type":"string"}},"required":["who"]}}]})
                }
                "tools/call" => {
                    assert_eq!(request["params"]["name"], "greet");
                    assert_eq!(request["params"]["arguments"], json!({"who":"世界"}));
                    json!({"content":[{"type":"text","text":"hello, 世界"}],"isError":false})
                }
                _ => unreachable!(),
            };
            let response = json!({"jsonrpc":"2.0","id":request["id"],"result":result}).to_string();
            // Use SSE framing, matching the legacy local HTTP fixture.
            let body = format!("event: message\ndata: {response}\n\n");
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nMcp-Session-Id: round-trip\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        }
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("local", transport);
    tokio::time::timeout(Duration::from_secs(5), async {
        client.connect(ProtocolMode::Legacy).await.unwrap();
        assert_eq!(client.protocol_version(), mcp::LEGACY_PROTOCOL_VERSION);
        assert_eq!(
            client.supported_versions(),
            &[mcp::LEGACY_PROTOCOL_VERSION.to_owned()]
        );
        assert_eq!(
            client
                .list_tools()
                .await
                .unwrap()
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>(),
            vec!["greet"]
        );
        let result = client
            .call_tool(
                "greet",
                schema::IJsonValue::parse_str(r#"{"who":"世界"}"#).unwrap(),
                McpCancellationToken::default(),
            )
            .await
            .unwrap();
        let result: Value = serde_json::from_slice(&result.canonical_bytes().unwrap()).unwrap();
        assert_eq!(result["isError"], false);
        assert_eq!(result["content"][0]["text"], "hello, 世界");
        client.close().await.unwrap();
        server.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn modern_http_continuation_preserves_input_and_task_resume_queries_identity() {
    use mcp::{McpCancellationToken, McpPeer, McpToolContinuation};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        for phase in 0..4 {
            let (mut socket, _) = tokio::time::timeout(Duration::from_secs(5), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let (headers, bytes) = read_http_request(&mut socket).await;
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            let result = match phase {
                0 => {
                    assert_eq!(request["method"], "server/discover");
                    json!({"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{},"tasks":{}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"continuation","version":"1"}}})
                }
                1 => {
                    assert_eq!(request["method"], "tools/list");
                    json!({"tools":[{"name":"confirm_action","inputSchema":{"type":"object","properties":{"operation":{"type":"string","x-mcp-header":"Operation"}}}}]})
                }
                2 => {
                    assert_eq!(request["method"], "tools/call");
                    assert!(
                        headers.contains("mcp-param-operation: =?base64?5y+r5bid?="),
                        "{headers}"
                    );
                    assert!(headers.contains("mcp-method: tools/call"));
                    let params = &request["params"];
                    assert_eq!(params["arguments"], json!({"operation":"发布"}));
                    assert_eq!(params["name"], "confirm_action");
                    assert_eq!(params["requestState"], "confirm:publish");
                    assert_eq!(
                        params["inputResponses"],
                        json!({"confirmation":{"action":"accept"}})
                    );
                    for key in ["continuation", "round", "taskId"] {
                        assert!(params.get(key).is_none());
                    }
                    json!({"content":[{"type":"text","text":"confirmed:发布"}],"isError":false})
                }
                _ => {
                    assert_eq!(request["method"], "tasks/get");
                    assert_eq!(request["params"]["taskId"], "server-task-7");
                    assert!(request["params"].get("arguments").is_none());
                    json!({"taskId":"server-task-7","status":"completed","createdAt":"2026-09-04T00:00:00Z","lastUpdatedAt":"2026-09-04T00:00:01Z","result":{"content":[]}})
                }
            };
            let body = format!(
                "data: {}\n\n",
                json!({"jsonrpc":"2.0","id":request["id"],"result":result})
            );
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
        }
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("continuation", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    client.list_tools().await.unwrap();
    let value = |v: Value| schema::IJsonValue::parse(&serde_json::to_vec(&v).unwrap()).unwrap();
    let mut continuation = McpToolContinuation {
        request_state: value(json!("confirm:publish")),
        input_responses: value(json!({"confirmation":{"action":"accept"}})),
        round: 1,
        task_id: None,
    };
    let result = client
        .continue_tool_call(
            "confirm_action",
            value(json!({"operation":"发布"})),
            continuation.clone(),
            McpCancellationToken::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(result).unwrap()["content"][0]["text"],
        "confirmed:发布"
    );
    continuation.task_id = Some("server-task-7".into());
    let result = client
        .continue_tool_call(
            "must_not_be_reexecuted",
            value(json!({})),
            continuation,
            McpCancellationToken::default(),
        )
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(result).unwrap()["status"], "completed");
    client.close().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn parameter_headers_follow_scoped_arguments_and_catalog_replacement() {
    use mcp::{McpCancellationToken, McpPeer};
    async fn reply(socket: &mut tokio::net::TcpStream, id: &Value, result: Value) {
        let body = json!({"jsonrpc":"2.0","id":id,"result":result}).to_string();
        socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        for phase in 0..2 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (_, bytes) = read_http_request(&mut socket).await;
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            let result = if phase == 0 {
                assert_eq!(request["method"], "server/discover");
                json!({"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"headers","version":"1"}}})
            } else {
                assert_eq!(request["method"], "tools/list");
                json!({"tools":[{"name":"echo","inputSchema":{"type":"object","properties":{"tenant":{"type":"string","x-mcp-header":"Tenant"}}}}]})
            };
            reply(&mut socket, &request["id"], result).await;
        }
        let mut pending = Vec::new();
        for _ in 0..2 {
            let (mut socket, _) = tokio::time::timeout(Duration::from_secs(3), listener.accept())
                .await
                .unwrap()
                .unwrap();
            let (headers, bytes) = read_http_request(&mut socket).await;
            let request: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(request["method"], "tools/call");
            let tenant = request["params"]["arguments"]["tenant"].as_str().unwrap();
            assert!(headers.contains(&format!("mcp-param-tenant: {tenant}")));
            pending.push((socket, request));
        }
        assert_ne!(
            pending[0].1["params"]["arguments"],
            pending[1].1["params"]["arguments"]
        );
        // Both requests must reach the server before either gets a response.
        for (mut socket, request) in pending {
            reply(
                &mut socket,
                &request["id"],
                json!({"tenant":request["params"]["arguments"]["tenant"]}),
            )
            .await;
        }
        let (mut socket, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut socket).await;
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(request["method"], "tools/list");
        reply(
            &mut socket,
            &request["id"],
            json!({"tools":[{"name":"echo","inputSchema":{"type":"object"}}]}),
        )
        .await;
        drop(socket);
        let (mut socket, _) = listener.accept().await.unwrap();
        let (headers, bytes) = read_http_request(&mut socket).await;
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(request["method"], "tools/call");
        assert!(!headers.contains("mcp-param-tenant:"));
        assert_eq!(request["params"]["arguments"]["tenant"], "after");
        reply(&mut socket, &request["id"], json!({"tenant":"after"})).await;
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("headers", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    client.list_tools().await.unwrap();
    let args = |tenant: &str| {
        schema::IJsonValue::parse(&serde_json::to_vec(&json!({"tenant":tenant})).unwrap()).unwrap()
    };
    let (one, two) = tokio::join!(
        client.call_tool_scoped("echo", args("one"), McpCancellationToken::default()),
        client.call_tool_scoped("echo", args("two"), McpCancellationToken::default())
    );
    assert_eq!(serde_json::to_value(one.unwrap()).unwrap()["tenant"], "one");
    assert_eq!(serde_json::to_value(two.unwrap()).unwrap()["tenant"], "two");
    client.list_tools().await.unwrap();
    let after = client
        .call_tool_scoped("echo", args("after"), McpCancellationToken::default())
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(after).unwrap()["tenant"], "after");
    client.close().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn dedicated_subscription_delivers_correlated_change_before_cancel_closes_socket() {
    use mcp::{McpCancellationToken, McpCapabilities, McpError};
    use std::sync::Arc;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let (headers, bytes) = read_http_request(&mut socket).await;
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(request["method"], "subscriptions/listen");
        assert_eq!(
            request["params"]["notifications"],
            json!({"toolsListChanged":true})
        );
        assert!(headers.contains("mcp-method: subscriptions/listen"));
        socket
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
        for (id, method, notifications) in [
            (99, "notifications/tools/list_changed", json!({})),
            (
                7,
                "notifications/subscriptions/acknowledged",
                json!({"toolsListChanged":true}),
            ),
            (7, "notifications/prompts/list_changed", json!({})),
            (7, "notifications/tools/list_changed", json!({})),
        ] {
            let event = json!({"jsonrpc":"2.0","method":method,"params":{"_meta":{"io.modelcontextprotocol/subscriptionId":id},"notifications":notifications}});
            socket
                .write_all(format!("data: {event}\n\n").as_bytes())
                .await
                .unwrap();
        }
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), socket.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let caps = McpCapabilities {
        tools: true,
        tools_list_changed: true,
        subscriptions: true,
        ..Default::default()
    };
    let cancellation = McpCancellationToken::default();
    let signal = cancellation.clone();
    let received = Arc::new(std::sync::Mutex::new(Vec::new()));
    let events = received.clone();
    let result = transport
        .listen_subscription(
            7,
            &caps,
            &[],
            Duration::from_secs(5),
            cancellation,
            Arc::new(move |event| {
                events.lock().unwrap().push(event.clone());
                signal.cancel();
            }),
        )
        .await;
    assert!(matches!(result, Err(McpError::Cancelled)), "{result:?}");
    {
        let events = received.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["method"], "notifications/tools/list_changed");
    }
    server.await.unwrap();
}

#[tokio::test]
async fn client_owned_subscription_updates_generation_and_closes_with_peer() {
    use mcp::McpError;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut discovery, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut discovery).await;
        let discovery_request: Value = serde_json::from_slice(&bytes).unwrap();
        let body = json!({"jsonrpc":"2.0","id":discovery_request["id"],"result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{"listChanged":true}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"owned","version":"1"}}}}).to_string();
        discovery.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        drop(discovery);
        let (mut socket, _) = listener.accept().await.unwrap();
        let (headers, bytes) = read_http_request(&mut socket).await;
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(request["method"], "subscriptions/listen");
        assert_eq!(
            request["params"]["notifications"],
            json!({"toolsListChanged":true})
        );
        assert!(headers.contains("mcp-method: subscriptions/listen"));
        socket
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
        let subscription_id = request["id"].as_u64().unwrap();
        for (id, method, notifications) in [
            (99, "notifications/tools/list_changed", json!({})),
            (
                subscription_id,
                "notifications/subscriptions/acknowledged",
                json!({"toolsListChanged":true}),
            ),
            (
                subscription_id,
                "notifications/prompts/list_changed",
                json!({}),
            ),
            (
                subscription_id,
                "notifications/tools/list_changed",
                json!({}),
            ),
        ] {
            let event = json!({"jsonrpc":"2.0","method":method,"params":{"_meta":{"io.modelcontextprotocol/subscriptionId":id},"notifications":notifications}});
            socket
                .write_all(format!("data: {event}\n\n").as_bytes())
                .await
                .unwrap();
        }
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), socket.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    use mcp::McpPeer;
    let mut client = McpClient::new("owned", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    let before = client.catalog_generation();
    let subscription = client.start_catalog_subscription().unwrap();
    assert!(client.start_catalog_subscription().is_none());
    let (result, ()) = tokio::join!(subscription, async {
        tokio::time::timeout(Duration::from_secs(3), async {
            while client.catalog_generation() == before {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(client.catalog_generation(), before + 1);
        client.close().await.unwrap();
    });
    assert!(matches!(result, Err(McpError::Cancelled)), "{result:?}");
    server.await.unwrap();
}

#[tokio::test]
async fn broker_registration_owns_subscription_until_removal() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut discovery, _) = listener.accept().await.unwrap();
        let (_, bytes) = read_http_request(&mut discovery).await;
        let discovery_request: Value = serde_json::from_slice(&bytes).unwrap();
        let body = json!({"jsonrpc":"2.0","id":discovery_request["id"],"result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"tools":{"listChanged":true}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"owned","version":"1"}}}}).to_string();
        discovery.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
        drop(discovery);
        let (mut socket, _) = listener.accept().await.unwrap();
        let (headers, bytes) = read_http_request(&mut socket).await;
        let request: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(request["method"], "subscriptions/listen");
        assert_eq!(
            request["params"]["notifications"],
            json!({"toolsListChanged":true})
        );
        assert!(headers.contains("mcp-method: subscriptions/listen"));
        socket
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
        let subscription_id = request["id"].as_u64().unwrap();
        for (id, method, notifications) in [
            (99, "notifications/tools/list_changed", json!({})),
            (
                subscription_id,
                "notifications/subscriptions/acknowledged",
                json!({"toolsListChanged":true}),
            ),
            (
                subscription_id,
                "notifications/prompts/list_changed",
                json!({}),
            ),
            (
                subscription_id,
                "notifications/tools/list_changed",
                json!({}),
            ),
        ] {
            let event = json!({"jsonrpc":"2.0","method":method,"params":{"_meta":{"io.modelcontextprotocol/subscriptionId":id},"notifications":notifications}});
            socket
                .write_all(format!("data: {event}\n\n").as_bytes())
                .await
                .unwrap();
        }
        let mut byte = [0];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), socket.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
    });
    let transport = HttpTransport::new(
        format!("http://{address}/mcp").parse().unwrap(),
        &BTreeMap::new(),
        None,
        true,
    )
    .unwrap();
    let mut client = McpClient::new("owned", transport);
    client.connect(ProtocolMode::Modern).await.unwrap();
    let broker = mcp::McpBroker::start().unwrap();
    let handle = broker.handle();
    let key = mcp::McpPoolKey {
        workspace: "subscription-test".into(),
        scope: "user".into(),
        server: "owned".into(),
        config_digest: "1".into(),
        authorization_identity: "anonymous".into(),
        protocol_mode: "modern".into(),
        plugin_generation: None,
    };
    let registration = handle.clone();
    let registered_key = key.clone();
    tokio::task::spawn_blocking(move || {
        registration.register(registered_key, Box::new(client), false)
    })
    .await
    .unwrap()
    .unwrap();
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            let query = handle.clone();
            let query_key = key.clone();
            let generation =
                tokio::task::spawn_blocking(move || query.catalog_generation(query_key))
                    .await
                    .unwrap()
                    .unwrap();
            if generation == Some(1) {
                break;
            }
            assert_eq!(generation, Some(0));
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let remove = handle.clone();
    let removed_key = key.clone();
    assert!(
        tokio::task::spawn_blocking(move || remove.remove(removed_key))
            .await
            .unwrap()
            .unwrap()
    );
    assert_eq!(handle.catalog_generation(key).unwrap(), None);
    server.await.unwrap();
    drop(broker);
}

#[tokio::test]
async fn task_cancellation_distinguishes_read_from_dispatched_mutation() {
    use mcp::{McpCancellationToken, McpError, McpPeer};
    for method in ["tasks/get", "tasks/update", "tasks/cancel"] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let received = std::sync::Arc::new(tokio::sync::Notify::new());
        let signal = received.clone();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (_, bytes) = read_http_request(&mut socket).await;
            let r: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(r["method"], "server/discover");
            let body=json!({"jsonrpc":"2.0","id":r["id"],"result":{"resultType":"complete","supportedVersions":[mcp::MODERN_PROTOCOL_VERSION],"capabilities":{"extensions":{"io.modelcontextprotocol/tasks":{}}},"_meta":{"io.modelcontextprotocol/serverInfo":{"name":"tasks","version":"1"}}}}).to_string();
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
            drop(socket);
            let (mut socket, _) = listener.accept().await.unwrap();
            let (_, bytes) = read_http_request(&mut socket).await;
            let r: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(r["method"], method);
            assert_eq!(r["params"]["taskId"], "task-1");
            signal.notify_one();
            let mut byte = [0];
            assert_eq!(
                tokio::time::timeout(Duration::from_secs(3), socket.read(&mut byte))
                    .await
                    .unwrap()
                    .unwrap(),
                0
            );
        });
        let transport = HttpTransport::new(
            format!("http://{address}/mcp").parse().unwrap(),
            &BTreeMap::new(),
            None,
            true,
        )
        .unwrap();
        let mut client = McpClient::new("tasks", transport);
        client.connect(ProtocolMode::Modern).await.unwrap();
        let params = || schema::IJsonValue::parse_str(r#"{"taskId":"task-1"}"#).unwrap();
        let pre = McpCancellationToken::default();
        pre.cancel();
        assert!(matches!(
            client
                .task_operation_cancellable(method, params(), pre)
                .await,
            Err(McpError::Cancelled)
        ));
        let token = McpCancellationToken::default();
        let trigger = token.clone();
        let (result, ()) = tokio::join!(
            client.task_operation_cancellable(method, params(), token),
            async {
                received.notified().await;
                trigger.cancel();
            }
        );
        if method == "tasks/get" {
            assert!(matches!(result, Err(McpError::Cancelled)), "{result:?}");
        } else {
            assert!(matches!(result, Err(McpError::UnknownEffect)), "{result:?}");
        }
        server.await.unwrap();
    }
}
