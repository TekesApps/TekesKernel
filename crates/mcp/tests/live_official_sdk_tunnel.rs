//! Public-tunnel live gates against the official `@modelcontextprotocol/sdk`
//! servers in `scripts/mcp-official-sdk` (ports of the pinned
//! `officialCSharpSDKPublicTunnelCompletesTaskLifecycle` and
//! `officialSDKPublicTunnelCoversSSEHeadersMRTRAndSubscriptions`).
//!
//! Gated on `TEKES_LIVE_MCP_TASKS_URL` / `TEKES_LIVE_MCP_CONFORMANCE_URL`
//! (HTTPS; `TEKES_LIVE_MCP_ALLOW_HTTP=1` admits a loopback rehearsal). The
//! released SDKs speak protocol 2025-11-25 (`initialize`), so the peers connect
//! in the Kernel's legacy mode; the 2026-07-28 modern handshake, `x-mcp-header`
//! parameter headers, multi-round `requestState`/`inputResponses` results and
//! the modern catalog subscription have no official implementation to run
//! against and are recorded as such in the maintainers' live-test inventory.
//! Receipts go to `TEKES_LIVE_MCP_RECEIPT_DIR` when set.
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use mcp::{
    HttpTransport, McpCancellationToken, McpClient, McpPeer, McpTask, McpToolContinuation,
    ProtocolMode,
};
use schema::IJsonValue;
use serde_json::{Value, json};

fn allow_http() -> bool {
    std::env::var("TEKES_LIVE_MCP_ALLOW_HTTP").is_ok_and(|value| value == "1")
}

fn receipt(name: &str, value: &Value) {
    if let Ok(directory) = std::env::var("TEKES_LIVE_MCP_RECEIPT_DIR") {
        std::fs::create_dir_all(&directory).expect("receipt directory");
        std::fs::write(
            std::path::Path::new(&directory).join(format!("{name}.json")),
            serde_json::to_vec_pretty(value).expect("receipt JSON"),
        )
        .expect("receipt");
    }
}

fn to_json(value: &IJsonValue) -> Value {
    serde_json::to_value(value).expect("I-JSON is JSON")
}

#[tokio::test]
#[ignore = "needs TEKES_LIVE_MCP_TASKS_URL (scripts/run-live-mcp-official-sdk.py)"]
async fn official_sdk_public_tunnel_completes_task_lifecycle() {
    let url = std::env::var("TEKES_LIVE_MCP_TASKS_URL").expect("TEKES_LIVE_MCP_TASKS_URL");
    let started = Instant::now();
    let transport = HttpTransport::new(
        url.parse().expect("url"),
        &BTreeMap::new(),
        None,
        allow_http(),
    )
    .expect("transport");
    let mut client = McpClient::new("official-ts-tasks", transport);
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("legacy connect against the official SDK");
    assert_eq!(
        client.server().map(|server| server.name.as_str()),
        Some("tekes-official-ts-tasks")
    );
    assert_eq!(client.protocol_version(), mcp::LEGACY_PROTOCOL_VERSION);
    let tools = client.list_tools().await.expect("tools");
    assert_eq!(
        tools.len(),
        1,
        "the official server advertises exactly one tool"
    );
    let tool = &tools[0];
    assert!(
        tool.requires_task(),
        "slow_echo declares taskSupport required: {:?}",
        tool.execution
    );
    let arguments = IJsonValue::parse_str(r#"{"text":"public"}"#).expect("arguments");
    let created = client
        .call_tool_augmented(
            &tool.name,
            arguments.clone(),
            None,
            600_000,
            McpCancellationToken::default(),
        )
        .await
        .expect("task-augmented tools/call");
    let created = to_json(&created);
    let task = created
        .get("task")
        .expect("CreateTaskResult carries task")
        .clone();
    let task_id = task["taskId"].as_str().expect("taskId").to_owned();
    let initial = McpTask::validate_result(
        &IJsonValue::parse(&serde_json::to_vec(&task).unwrap()).unwrap(),
        &task_id,
    )
    .expect("valid created task");
    assert_eq!(initial.status, "working");
    assert!(
        task["pollIntervalMs"].as_u64().is_some(),
        "official pollInterval adopted as pollIntervalMs: {task}"
    );
    let mut polls = Vec::new();
    let mut round = 1_u32;
    let final_text = loop {
        let wait = task["pollIntervalMs"]
            .as_u64()
            .unwrap_or(400)
            .clamp(50, 5_000);
        tokio::time::sleep(Duration::from_millis(wait)).await;
        let polled = client
            .continue_tool_call(
                &tool.name,
                arguments.clone(),
                McpToolContinuation {
                    request_state: IJsonValue::parse_str("null").unwrap(),
                    input_responses: IJsonValue::parse_str("null").unwrap(),
                    round,
                    task_id: Some(task_id.clone()),
                },
                McpCancellationToken::default(),
            )
            .await
            .expect("tasks/get through the continuation path");
        let polled = to_json(&polled);
        let validated = McpTask::validate_result(
            &IJsonValue::parse(&serde_json::to_vec(&polled).unwrap()).unwrap(),
            &task_id,
        )
        .expect("valid poll");
        polls.push(json!({"round": round, "status": validated.status, "has_result": polled["result"].is_object()}));
        if validated.status == "completed" {
            let text = polled["result"]["content"][0]["text"]
                .as_str()
                .expect("result text")
                .to_owned();
            assert_eq!(polled["result"]["isError"], false);
            break text;
        }
        assert_eq!(
            validated.status, "working",
            "unexpected task status: {polled}"
        );
        round += 1;
        assert!(
            round <= 20,
            "official SDK task did not complete after 20 polls"
        );
    };
    assert_eq!(final_text, "task:public");
    client.close().await.expect("close");
    receipt(
        "tasks-lifecycle",
        &json!({
            "status": "passed", "url": url, "protocol": client.protocol_version(),
            "server": "tekes-official-ts-tasks", "tool": tool.name, "task_id": task_id,
            "created": task, "polls": polls, "final_text": final_text,
            "elapsed_ms": started.elapsed().as_millis() as u64,
        }),
    );
}

#[tokio::test]
#[ignore = "needs TEKES_LIVE_MCP_CONFORMANCE_URL (scripts/run-live-mcp-official-sdk.py)"]
async fn official_sdk_public_tunnel_forced_sse_round_trip() {
    let url =
        std::env::var("TEKES_LIVE_MCP_CONFORMANCE_URL").expect("TEKES_LIVE_MCP_CONFORMANCE_URL");
    let transport = HttpTransport::new(
        url.parse().expect("url"),
        &BTreeMap::new(),
        None,
        allow_http(),
    )
    .expect("transport");
    let mut client = McpClient::new("official-sdk-conformance", transport);
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("legacy connect against the official SDK");
    assert_eq!(
        client.server().map(|server| server.name.as_str()),
        Some("tekes-official-sdk-conformance")
    );
    let tools = client.list_tools().await.expect("tools");
    let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(
        names,
        ["sse_echo"],
        "released SDK surface: sse_echo only (no x-mcp-header, no multi-round results)"
    );
    // The server never answers JSON (enableJsonResponse=false): a successful
    // round trip proves request-scoped SSE decoding end to end over the tunnel.
    let output = client
        .call_tool(
            "sse_echo",
            IJsonValue::parse_str(r#"{"value":"round-trip"}"#).unwrap(),
            McpCancellationToken::default(),
        )
        .await
        .expect("sse_echo");
    let output = to_json(&output);
    assert!(!output["isError"].as_bool().unwrap_or(false));
    assert_eq!(output["content"][0]["text"], "sse:round-trip");
    client.close().await.expect("close");
    receipt(
        "conformance-sse",
        &json!({"status":"passed","url":url,"protocol":mcp::LEGACY_PROTOCOL_VERSION,"tools":names,"sse_echo":output}),
    );
}
