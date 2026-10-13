//! The tasks extension (SEP-2663) as the official Rust SDK `rmcp` 3.x speaks
//! it, replayed from `fixtures/mcp-runtime/tasks-extension-*.jsonl`.
//!
//! The replay transport compares every request the client emits with the
//! fixture's client line byte-for-byte (as canonical JSON) and answers with the
//! fixture's server line, so the transcripts are executable contracts rather
//! than documentation. A request the transcript does not contain fails the
//! test, which is how "no `tasks/result` for an extension-only peer" and "no
//! `task` member on `tools/call`" are enforced.
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

use mcp::{
    JsonRpcRequest, JsonRpcResponse, McpCancellationToken, McpClient, McpError, McpPeer,
    McpPeerFuture, McpTask, McpToolCallContext, McpTransport, ProtocolMode,
};
use schema::IJsonValue;
use serde_json::{Value, json};

enum Step {
    Request { client: Value, server: Value },
    Notification { client: Value },
}

struct Replay {
    steps: VecDeque<Step>,
}

fn fixture(name: &str) -> Vec<Value> {
    let root = std::env::var_os("TEKES_KERNEL_FIXTURES").map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures"),
        PathBuf::from,
    );
    let text = std::fs::read_to_string(root.join("mcp-runtime").join(name)).expect("fixture");
    text.lines()
        .map(|line| serde_json::from_str(line).expect("fixture line"))
        .collect()
}

fn steps(lines: Vec<Value>) -> VecDeque<Step> {
    let mut lines = lines.into_iter();
    let mut steps = VecDeque::new();
    while let Some(client) = lines.next() {
        assert!(
            client.get("method").is_some(),
            "client line first: {client}"
        );
        if client.get("id").is_some() {
            let server = lines.next().expect("server line after a request");
            assert!(server.get("method").is_none(), "response expected");
            steps.push_back(Step::Request { client, server });
        } else {
            steps.push_back(Step::Notification { client });
        }
    }
    steps
}

/// Fixtures record `clientInfo.version` as `0.1.0`; the Kernel sends its own.
fn normalized(mut value: Value) -> Value {
    if let Some(version) = value.pointer_mut("/params/clientInfo/version") {
        *version = json!("0.1.0");
    }
    value
}

impl Replay {
    fn new(files: &[&str]) -> Self {
        let mut lines = Vec::new();
        for file in files {
            lines.extend(fixture(file));
        }
        Self {
            steps: steps(lines),
        }
    }

    fn with_lines(lines: Vec<Value>) -> Self {
        Self {
            steps: steps(lines),
        }
    }
}

impl McpTransport for Replay {
    fn request<'a>(
        &'a mut self,
        request: &'a JsonRpcRequest,
        _timeout: Duration,
    ) -> McpPeerFuture<'a, JsonRpcResponse> {
        let sent = normalized(serde_json::to_value(request).expect("request JSON"));
        let step = self.steps.pop_front();
        Box::pin(async move {
            let Some(Step::Request { client, server }) = step else {
                panic!("unexpected request: {sent}");
            };
            assert_eq!(sent, client, "the client request differs from the fixture");
            serde_json::from_value(server).map_err(|error| McpError::Protocol(error.to_string()))
        })
    }

    fn notify<'a>(&'a mut self, notification: &'a JsonRpcRequest) -> McpPeerFuture<'a, ()> {
        let sent = serde_json::to_value(notification).expect("notification JSON");
        let step = self.steps.pop_front();
        Box::pin(async move {
            let Some(Step::Notification { client }) = step else {
                panic!("unexpected notification: {sent}");
            };
            assert_eq!(sent, client);
            Ok(())
        })
    }

    fn close(&mut self) -> McpPeerFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
}

async fn connected(files: &[&str]) -> McpClient<Replay> {
    let mut client = McpClient::new("rmcp-fixture", Replay::new(files));
    client
        .connect(ProtocolMode::Legacy)
        .await
        .expect("legacy handshake replays");
    client
}

fn ijson(value: &Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(value).expect("JSON")).expect("I-JSON")
}

fn json_of(value: IJsonValue) -> Value {
    serde_json::to_value(value).expect("I-JSON is JSON")
}

fn context() -> McpToolCallContext {
    McpToolCallContext {
        idempotency_key: "a".repeat(64),
    }
}

async fn get(client: &mut McpClient<Replay>, task_id: &str) -> Value {
    json_of(
        client
            .task_operation("tasks/get", ijson(&json!({"taskId": task_id})))
            .await
            .expect("tasks/get"),
    )
}

fn validates(task: &Value, task_id: &str) -> McpTask {
    McpTask::validate_result(&ijson(task), task_id).expect("the Kernel task validates")
}

#[tokio::test]
async fn the_handshake_declares_the_tasks_extension_for_both_dialects() {
    let client = connected(&["tasks-extension-handshake.jsonl"]).await;
    assert!(client.capabilities().supports_task_extension());
    assert!(!client.capabilities().tasks && client.capabilities().supports_tasks());

    // A SEP-1686 server (top-level `tasks`) sees the same client declaration.
    let client = connected(&["legacy-handshake.jsonl"]).await;
    assert!(client.capabilities().tasks && !client.capabilities().supports_task_extension());

    let mut modern = McpClient::new("rmcp-fixture", Replay::new(&["modern-discover.jsonl"]));
    modern
        .connect(ProtocolMode::Modern)
        .await
        .expect("modern discovery declares the extension in request metadata");
}

#[tokio::test]
async fn a_flat_task_result_becomes_the_kernel_creation_shape_and_polls_to_completion() {
    let mut client = connected(&[
        "tasks-extension-handshake.jsonl",
        "tasks-extension-tool-call.jsonl",
    ])
    .await;
    let created = json_of(
        client
            .call_tool_with_context(
                "sum",
                ijson(&json!({"a": 2, "b": 3})),
                context(),
                McpCancellationToken::default(),
            )
            .await
            .expect("tools/call"),
    );
    assert!(created.get("resultType").is_none(), "{created}");
    let task = &created["task"];
    assert_eq!(task["taskId"], "task-ext-1");
    assert_eq!(task["status"], "working");
    assert_eq!(task["pollIntervalMs"], 50);
    assert_eq!(task["ttlMs"], 300_000);
    assert!(task.get("resultType").is_none() && task.get("ttl").is_none());
    assert_eq!(validates(task, "task-ext-1").status, "working");

    let working = get(&mut client, "task-ext-1").await;
    assert!(working.get("resultType").is_none(), "{working}");
    assert_eq!(validates(&working, "task-ext-1").status, "working");
    // The completed poll carries its result inline; no `tasks/result` follows.
    let done = get(&mut client, "task-ext-1").await;
    assert_eq!(done["result"]["content"][0]["text"], "5");
    assert_eq!(validates(&done, "task-ext-1").status, "completed");
}

#[tokio::test]
async fn augmented_calls_to_an_extension_only_peer_carry_no_task_member() {
    // The transcript's `tools/call` has no `task`; replay fails on any extra key.
    let mut client = connected(&[
        "tasks-extension-handshake.jsonl",
        "tasks-extension-tool-call.jsonl",
    ])
    .await;
    let created = json_of(
        client
            .call_tool_augmented(
                "sum",
                ijson(&json!({"a": 2, "b": 3})),
                Some(context()),
                600_000,
                McpCancellationToken::default(),
            )
            .await
            .expect("augmented tools/call"),
    );
    assert_eq!(created["task"]["taskId"], "task-ext-1");
    assert!(created["task"].get("resultType").is_none());
}

#[tokio::test]
async fn input_required_tasks_are_answered_with_tasks_update() {
    let mut client = connected(&[
        "tasks-extension-handshake.jsonl",
        "tasks-extension-input.jsonl",
    ])
    .await;
    client
        .call_tool_with_context(
            "sum",
            ijson(&json!({"a": 2, "b": 3})),
            context(),
            McpCancellationToken::default(),
        )
        .await
        .expect("tools/call");
    let waiting = get(&mut client, "task-ext-2").await;
    assert_eq!(validates(&waiting, "task-ext-2").status, "input_required");
    assert_eq!(
        waiting["inputRequests"]["confirm"]["method"],
        "elicitation/create"
    );
    let ack = json_of(
        client
            .task_operation(
                "tasks/update",
                ijson(&json!({
                    "taskId": "task-ext-2",
                    "inputResponses": {"confirm": {"action": "accept"}}
                })),
            )
            .await
            .expect("tasks/update"),
    );
    assert!(
        ack.get("resultType").is_none() && ack.get("taskId").is_none(),
        "{ack}"
    );
    let done = get(&mut client, "task-ext-2").await;
    assert_eq!(validates(&done, "task-ext-2").status, "completed");
}

#[tokio::test]
async fn cancelled_and_failed_tasks_are_terminal_polls() {
    let mut client = connected(&[
        "tasks-extension-handshake.jsonl",
        "tasks-extension-cancel.jsonl",
    ])
    .await;
    client
        .call_tool_with_context(
            "sum",
            ijson(&json!({"a": 2, "b": 3})),
            context(),
            McpCancellationToken::default(),
        )
        .await
        .expect("first tools/call");
    client
        .task_operation("tasks/cancel", ijson(&json!({"taskId": "task-ext-3"})))
        .await
        .expect("tasks/cancel");
    let cancelled = get(&mut client, "task-ext-3").await;
    assert_eq!(validates(&cancelled, "task-ext-3").status, "cancelled");

    client
        .call_tool_with_context(
            "sum",
            ijson(&json!({"a": 2, "b": 3})),
            context(),
            McpCancellationToken::default(),
        )
        .await
        .expect("second tools/call");
    let failed = get(&mut client, "task-ext-4").await;
    assert_eq!(validates(&failed, "task-ext-4").status, "failed");
    assert_eq!(failed["error"]["code"], -32603);
}

#[tokio::test]
async fn a_server_without_the_declaration_refuses_with_the_typed_remote_error() {
    let mut client = connected(&[
        "tasks-extension-handshake.jsonl",
        "tasks-extension-missing-capability.jsonl",
    ])
    .await;
    let refused = client
        .call_tool_with_context(
            "sum",
            ijson(&json!({"a": 2, "b": 3})),
            context(),
            McpCancellationToken::default(),
        )
        .await;
    assert!(
        matches!(refused, Err(McpError::Remote { code: -32021, .. })),
        "{refused:?}"
    );
    let refused = client
        .task_operation("tasks/get", ijson(&json!({"taskId": "task-ext-1"})))
        .await;
    assert!(matches!(
        refused,
        Err(McpError::Remote { code: -32021, .. })
    ));
}

#[tokio::test]
async fn the_modern_protocol_carries_the_declaration_in_request_metadata() {
    let mut client = McpClient::new(
        "rmcp-fixture",
        Replay::new(&["tasks-extension-modern.jsonl"]),
    );
    client
        .connect(ProtocolMode::Modern)
        .await
        .expect("modern discovery");
    assert!(client.capabilities().supports_task_extension());
    let created = json_of(
        client
            .call_tool_with_context(
                "sum",
                ijson(&json!({"a": 2, "b": 3})),
                context(),
                McpCancellationToken::default(),
            )
            .await
            .expect("modern tools/call"),
    );
    assert_eq!(created["task"]["taskId"], "task-ext-5");
    let done = get(&mut client, "task-ext-5").await;
    assert_eq!(validates(&done, "task-ext-5").status, "completed");
}

fn handshake(server_capabilities: &Value) -> Vec<Value> {
    let mut lines = fixture("tasks-extension-handshake.jsonl");
    lines[1]["result"]["capabilities"] = server_capabilities.clone();
    lines
}

fn task_result() -> Value {
    json!({
        "createdAt": "2026-10-14T00:00:00Z",
        "lastUpdatedAt": "2026-10-14T00:00:00Z",
        "resultType": "task",
        "status": "working",
        "taskId": "task-bad",
        "ttlMs": null
    })
}

async fn call_with(server_capabilities: Value, result: Value) -> Result<Value, McpError> {
    let mut lines = handshake(&server_capabilities);
    lines.push(json!({"id": 2, "jsonrpc": "2.0", "method": "tools/call",
        "params": {"arguments": {}, "name": "sum"}}));
    lines.push(json!({"id": 2, "jsonrpc": "2.0", "result": result}));
    let mut client = McpClient::new("rmcp-fixture", Replay::with_lines(lines));
    client.connect(ProtocolMode::Legacy).await.expect("connect");
    client
        .call_tool("sum", ijson(&json!({})), McpCancellationToken::default())
        .await
        .map(json_of)
}

#[tokio::test]
async fn malformed_flat_task_results_fail_closed_as_unknown_effect() {
    let advertised = json!({"extensions": {"io.modelcontextprotocol/tasks": {}}, "tools": {}});
    // A task from a peer that never advertised any task capability.
    let unadvertised = call_with(json!({"tools": {}}), task_result()).await;
    assert!(
        matches!(unadvertised, Err(McpError::UnknownEffect)),
        "{unadvertised:?}"
    );
    // Both creation shapes at once.
    let mut mixed = task_result();
    mixed["task"] = json!({"taskId": "task-bad"});
    let mixed = call_with(advertised.clone(), mixed).await;
    assert!(matches!(mixed, Err(McpError::UnknownEffect)), "{mixed:?}");
    // An ordinary result, including one marked `resultType:"complete"`, is untouched.
    let complete = json!({"content": [{"text": "x", "type": "text"}], "resultType": "complete"});
    assert_eq!(
        call_with(advertised, complete.clone())
            .await
            .expect("complete"),
        complete
    );
}

#[tokio::test]
async fn a_completed_poll_without_a_result_is_invalid_for_an_extension_only_peer() {
    let mut lines =
        handshake(&json!({"extensions": {"io.modelcontextprotocol/tasks": {}}, "tools": {}}));
    lines.push(json!({"id": 2, "jsonrpc": "2.0", "method": "tasks/get",
        "params": {"taskId": "task-bad"}}));
    lines.push(json!({"id": 2, "jsonrpc": "2.0", "result": {
        "createdAt": "2026-10-14T00:00:00Z",
        "lastUpdatedAt": "2026-10-14T00:00:01Z",
        "resultType": "complete",
        "status": "completed",
        "taskId": "task-bad",
        "ttlMs": null
    }}));
    let mut client = McpClient::new("rmcp-fixture", Replay::with_lines(lines));
    client.connect(ProtocolMode::Legacy).await.expect("connect");
    // The replay would panic on a `tasks/result` request; the poll instead
    // comes back as-is and fails the Kernel task validation.
    let polled = get(&mut client, "task-bad").await;
    assert!(McpTask::validate_result(&ijson(&polled), "task-bad").is_err());
}
