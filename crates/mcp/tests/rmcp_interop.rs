//! Interoperability with the official Rust SDK (`rmcp` 3.5.1) as a server.
//!
//! `rmcp` implements the tasks extension (SEP-2663): it advertises
//! `extensions["io.modelcontextprotocol/tasks"]`, has no top-level `tasks`
//! capability and no `execution.taskSupport`, answers a task-creating
//! `tools/call` with a flat `resultType:"task"` object, serves `tasks/get`,
//! `tasks/update` and `tasks/cancel` (no `tasks/result`), and refuses with
//! `-32021` when the client did not declare the extension.
//!
//! This test binary has no libtest harness (`harness = false`): it is both the
//! driver and, when `TEKES_RMCP_INTEROP_SERVER` is set, the `rmcp` stdio server
//! the driver spawns from `current_exe`, so it owns stdout. The wire shapes it
//! checks are the ones frozen in `fixtures/mcp-runtime/tasks-extension-*.jsonl`.
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use mcp::{
    McpCancellationToken, McpClient, McpError, McpPeer, McpTask, McpToolCallContext, ProtocolMode,
    StdioTransport,
};
use schema::IJsonValue;
use serde_json::{Value, json};

const SERVER_ENV: &str = "TEKES_RMCP_INTEROP_SERVER";

mod server {
    use std::sync::Arc;
    use std::time::Duration;

    use rmcp::{
        ErrorData as McpError, ServerHandler, ServiceExt,
        model::*,
        service::{RequestContext, RoleServer},
        task_manager::{TaskContext, TaskExit, TaskFuture, TaskManager, TaskOptions},
    };
    use serde_json::json;

    #[derive(Clone)]
    struct Fixture {
        tasks: TaskManager,
    }

    fn tool(name: &'static str, description: &'static str) -> Tool {
        let schema = json!({"type": "object", "properties": {}, "additionalProperties": true});
        Tool::new(
            name,
            description,
            Arc::new(schema.as_object().expect("schema object").clone()),
        )
    }

    fn text(value: &str) -> CallToolResult {
        CallToolResult::success(vec![ContentBlock::text(value)])
    }

    const TASK_TOOLS: [&str; 4] = ["sum", "slow", "ask", "boom"];

    fn task_body(name: String, ctx: TaskContext) -> TaskFuture {
        Box::pin(async move {
            match name.as_str() {
                "sum" => {
                    tokio::time::sleep(Duration::from_millis(60)).await;
                    Ok(text("5"))
                }
                "slow" => {
                    tokio::select! {
                        () = ctx.cancelled() => Err(TaskExit::Cancelled),
                        () = tokio::time::sleep(Duration::from_secs(30)) => Ok(text("late")),
                    }
                }
                "ask" => {
                    let request: InputRequest = serde_json::from_value(json!({
                        "method": "elicitation/create",
                        "params": {
                            "message": "What is your name?",
                            "requestedSchema": {"type": "object", "properties": {}}
                        }
                    }))
                    .map_err(|error| McpError::internal_error(error.to_string(), None))?;
                    let response = ctx.request_input("name", request).await?;
                    let name = response["content"]["name"].as_str().unwrap_or("nobody");
                    Ok(text(&format!("hello {name}")))
                }
                _ => Err(TaskExit::Error(McpError::internal_error("boom", None))),
            }
        })
    }

    impl ServerHandler for Fixture {
        fn get_info(&self) -> ServerConfig {
            ServerConfig::new(
                ServerCapabilities::builder()
                    .enable_tools()
                    .enable_tasks()
                    .build(),
            )
        }

        async fn list_tools(
            &self,
            _request: Option<PaginatedRequestParams>,
            _context: RequestContext<RoleServer>,
        ) -> Result<ListToolsResult, McpError> {
            Ok(ListToolsResult::with_all_items(vec![
                tool("echo", "Answers synchronously"),
                tool("sum", "A task that completes"),
                tool("slow", "A task that waits for cancellation"),
                tool("ask", "A task that needs input"),
                tool("boom", "A task that fails"),
            ]))
        }

        async fn call_tool(
            &self,
            request: CallToolRequestParams,
            context: RequestContext<RoleServer>,
        ) -> Result<CallToolResponse, McpError> {
            let name = request.name.to_string();
            if name == "echo" {
                return Ok(CallToolResponse::Complete(text("echo")));
            }
            if !TASK_TOOLS.contains(&name.as_str()) {
                return Err(McpError::invalid_params("unknown tool", None));
            }
            // The server, not the client, decides to create a task, and only
            // for a client that declared the extension.
            let declared = context
                .client_capabilities()
                .is_some_and(|capabilities| capabilities.supports_tasks());
            if !declared {
                return Err(McpError::missing_required_client_capability(
                    ClientCapabilities::builder().enable_tasks().build(),
                ));
            }
            let options = TaskOptions::new().with_poll_interval_ms(20);
            let task = self.tasks.spawn(options, |ctx| task_body(name, ctx));
            Ok(CallToolResponse::Task(CreateTaskResult::new(task)))
        }

        async fn get_task(
            &self,
            request: GetTaskParams,
            _context: RequestContext<RoleServer>,
        ) -> Result<GetTaskResult, McpError> {
            Ok(GetTaskResult::new(self.tasks.get_task(&request.task_id)?))
        }

        async fn update_task(
            &self,
            request: UpdateTaskParams,
            _context: RequestContext<RoleServer>,
        ) -> Result<(), McpError> {
            self.tasks
                .update_task(&request.task_id, request.input_responses)
        }

        async fn cancel_task(
            &self,
            request: CancelTaskParams,
            _context: RequestContext<RoleServer>,
        ) -> Result<(), McpError> {
            self.tasks.cancel_task(&request.task_id)
        }
    }

    pub fn run() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("server runtime")
            .block_on(async {
                let service = Fixture {
                    tasks: TaskManager::new(),
                }
                .serve(rmcp::transport::stdio())
                .await
                .expect("serve rmcp over stdio");
                let _ = service.waiting().await;
            });
    }
}

fn ijson(value: &Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(value).expect("JSON")).expect("I-JSON")
}

fn json_of(value: IJsonValue) -> Value {
    serde_json::to_value(value).expect("I-JSON is JSON")
}

fn server_command() -> (Vec<String>, BTreeMap<String, String>) {
    let exe = std::env::current_exe().expect("current_exe");
    (
        vec![exe.to_string_lossy().into_owned()],
        BTreeMap::from([(SERVER_ENV.to_owned(), "1".to_owned())]),
    )
}

async fn connect(mode: ProtocolMode) -> McpClient<StdioTransport> {
    let (argv, environment) = server_command();
    let transport = StdioTransport::spawn(&argv, None, &environment).expect("spawn rmcp server");
    let mut client = McpClient::new("rmcp", transport);
    client
        .connect(mode)
        .await
        .expect("handshake with rmcp 3.5.1");
    client
}

fn context() -> McpToolCallContext {
    McpToolCallContext {
        idempotency_key: "b".repeat(64),
    }
}

async fn create(client: &mut McpClient<StdioTransport>, tool: &str) -> Value {
    let created = json_of(
        client
            .call_tool_with_context(
                tool,
                ijson(&json!({})),
                context(),
                McpCancellationToken::default(),
            )
            .await
            .unwrap_or_else(|error| panic!("tools/call {tool}: {error:?}")),
    );
    assert!(created.get("resultType").is_none(), "normalized: {created}");
    let task = created["task"].clone();
    assert_eq!(task["status"], "working", "{created}");
    assert!(
        task["ttlMs"].is_u64() && task["pollIntervalMs"] == 20,
        "{task}"
    );
    for legacy in ["ttl", "pollInterval", "resultType"] {
        assert!(task.get(legacy).is_none(), "{task}");
    }
    McpTask::validate_result(&ijson(&task), task["taskId"].as_str().expect("taskId"))
        .expect("the created task validates as a Kernel task");
    task
}

async fn get(client: &mut McpClient<StdioTransport>, task_id: &str) -> Value {
    let polled = json_of(
        client
            .task_operation("tasks/get", ijson(&json!({"taskId": task_id})))
            .await
            .expect("tasks/get"),
    );
    McpTask::validate_result(&ijson(&polled), task_id).expect("the polled task validates");
    polled
}

async fn poll_until(client: &mut McpClient<StdioTransport>, task_id: &str, wanted: &str) -> Value {
    for _ in 0..250 {
        let polled = get(client, task_id).await;
        if polled["status"] == wanted {
            return polled;
        }
        assert!(
            matches!(
                polled["status"].as_str(),
                Some("working" | "input_required")
            ),
            "task ended {} while waiting for {wanted}: {polled}",
            polled["status"]
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    panic!("task {task_id} never became {wanted}");
}

async fn driver() {
    let mut client = connect(ProtocolMode::Legacy).await;
    assert_eq!(client.protocol_version(), "2025-11-25");
    let capabilities = client.capabilities().clone();
    assert!(capabilities.supports_task_extension() && !capabilities.tasks);
    assert!(!client.server().expect("server").name.is_empty());
    let tools = client.list_tools().await.expect("tools/list");
    assert_eq!(tools.len(), 5);
    assert!(
        tools
            .iter()
            .all(|tool| tool.execution.is_none() && !tool.requires_task()),
        "rmcp tools cannot declare execution.taskSupport"
    );

    // The server answers a plain call synchronously: untouched.
    let echo = json_of(
        client
            .call_tool_with_context(
                "echo",
                ijson(&json!({})),
                context(),
                McpCancellationToken::default(),
            )
            .await
            .expect("echo"),
    );
    assert_eq!(echo["content"][0]["text"], "echo");
    assert!(echo.get("task").is_none());

    // A task that completes, polled to its inline result.
    let task = create(&mut client, "sum").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    let done = poll_until(&mut client, &id, "completed").await;
    assert_eq!(done["result"]["content"][0]["text"], "5");

    // The same through a call that was never told about augmentation.
    let augmented = json_of(
        client
            .call_tool_augmented(
                "sum",
                ijson(&json!({})),
                Some(context()),
                600_000,
                McpCancellationToken::default(),
            )
            .await
            .expect("augmented call carries no task member"),
    );
    assert_eq!(augmented["task"]["status"], "working");

    // Cooperative cancellation.
    let task = create(&mut client, "slow").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    client
        .task_operation("tasks/cancel", ijson(&json!({"taskId": id})))
        .await
        .expect("tasks/cancel");
    poll_until(&mut client, &id, "cancelled").await;

    // A task that needs input, answered with tasks/update.
    let task = create(&mut client, "ask").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    let waiting = poll_until(&mut client, &id, "input_required").await;
    assert_eq!(
        waiting["inputRequests"]["name"]["method"],
        "elicitation/create"
    );
    client
        .task_operation(
            "tasks/update",
            ijson(&json!({
                "taskId": id,
                "inputResponses": {"name": {"action": "accept", "content": {"name": "Ada"}}}
            })),
        )
        .await
        .expect("tasks/update");
    let done = poll_until(&mut client, &id, "completed").await;
    assert_eq!(done["result"]["content"][0]["text"], "hello Ada");

    // A failed task carries its JSON-RPC error inline.
    let task = create(&mut client, "boom").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    let failed = poll_until(&mut client, &id, "failed").await;
    assert_eq!(failed["error"]["message"], "boom");

    // An unknown task is a typed remote error.
    let unknown = client
        .task_operation("tasks/get", ijson(&json!({"taskId": "no-such-task"})))
        .await;
    assert!(
        matches!(unknown, Err(McpError::Remote { .. })),
        "{unknown:?}"
    );
    client.close().await.expect("close");
    println!("rmcp interop: typed client ok");
}

/// The 2026-07-28 protocol has no `initialize`: the declaration travels in the
/// request metadata of `server/discover` and every later request.
async fn modern_driver() {
    let mut client = connect(ProtocolMode::Modern).await;
    assert_eq!(client.protocol_version(), "2026-07-28");
    assert!(client.capabilities().supports_task_extension() && !client.capabilities().tasks);
    assert_eq!(client.list_tools().await.expect("tools/list").len(), 5);
    let task = create(&mut client, "sum").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    let done = poll_until(&mut client, &id, "completed").await;
    assert_eq!(done["result"]["content"][0]["text"], "5");
    let task = create(&mut client, "slow").await;
    let id = task["taskId"].as_str().unwrap().to_owned();
    client
        .task_operation("tasks/cancel", ijson(&json!({"taskId": id})))
        .await
        .expect("tasks/cancel");
    poll_until(&mut client, &id, "cancelled").await;
    client.close().await.expect("close");
    println!("rmcp interop: modern client ok");
}

/// A client that does not declare the extension is refused by rmcp itself, as
/// the contract states, using raw JSON-RPC lines so nothing of the Kernel
/// client's declaration is in play.
fn undeclared_client_is_refused_with_32021() {
    let (argv, environment) = server_command();
    let mut child = Command::new(&argv[0])
        .envs(environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn rmcp server");
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
    let mut send = |request: &Value| {
        writeln!(stdin, "{request}").expect("write");
        stdin.flush().expect("flush");
    };
    let mut read = || -> Value {
        let mut line = String::new();
        stdout.read_line(&mut line).expect("read");
        serde_json::from_str(&line).expect("JSON-RPC line")
    };
    send(&json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {"protocolVersion": "2025-11-25", "capabilities": {},
                   "clientInfo": {"name": "undeclared", "version": "1"}}
    }));
    let initialized = read();
    assert_eq!(initialized["result"]["protocolVersion"], "2025-11-25");
    send(&json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
    send(&json!({
        "jsonrpc": "2.0", "id": 2, "method": "tools/call",
        "params": {"name": "sum", "arguments": {}}
    }));
    let refused = read();
    assert_eq!(refused["error"]["code"], -32021, "{refused}");
    assert_eq!(
        refused["error"]["data"]["requiredCapabilities"]["extensions"]["io.modelcontextprotocol/tasks"],
        json!({})
    );
    send(&json!({
        "jsonrpc": "2.0", "id": 3, "method": "tasks/get", "params": {"taskId": "x"}
    }));
    let refused = read();
    assert_eq!(refused["error"]["code"], -32021, "{refused}");
    drop(stdin);
    let _ = child.wait();
    println!("rmcp interop: -32021 without the declaration ok");
}

fn main() {
    if std::env::var_os(SERVER_ENV).is_some() {
        server::run();
        return;
    }
    undeclared_client_is_refused_with_32021();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("driver runtime")
        .block_on(async {
            driver().await;
            modern_driver().await;
        });
}
