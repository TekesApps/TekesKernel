use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use serde_json::{Value, json};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let scenario = arguments.next().unwrap_or_default();
    if scenario == "descendant-inherits-stderr" {
        if let Some(pid_path) = arguments.next() {
            std::fs::write(pid_path, std::process::id().to_string()).expect("write fixture pid");
        }
    }
    let _inherited_stderr = (scenario == "descendant-inherits-stderr").then(|| {
        Command::new("/bin/sh")
            .args(["-c", "sleep 2"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("spawn stderr-inheriting descendant")
    });
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut modern = false;
    let mut task_polls = 0_u32;
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let Ok(request) = serde_json::from_str::<Value>(&line) else {
            break;
        };
        let Some(id) = request.get("id").and_then(Value::as_u64) else {
            continue;
        };
        let method = request.get("method").and_then(Value::as_str).unwrap_or("");
        if method == "initialize" && scenario == "duplicate-json" {
            writeln!(
                stdout,
                "{{\"id\":{id},\"jsonrpc\":\"2.0\",\"result\":{{\"capabilities\":{{}},\"protocolVersion\":\"2025-11-25\",\"serverInfo\":{{\"name\":\"one\",\"name\":\"two\",\"version\":\"1\"}}}}}}"
            )
            .expect("write duplicate response");
            stdout.flush().expect("flush duplicate response");
            continue;
        }
        if method == "initialize" && scenario == "unknown-notification" {
            writeln!(
                stdout,
                "{}",
                json!({"jsonrpc":"2.0","method":"notifications/unknown-required"})
            )
            .expect("write unknown notification");
            stdout.flush().expect("flush unknown notification");
            continue;
        }
        if method == "initialize" && scenario == "oversized" {
            stdout
                .write_all(&vec![b'x'; 4 * 1024 * 1024 + 1])
                .expect("write oversized response");
            stdout.write_all(b"\n").expect("frame oversized response");
            stdout.flush().expect("flush oversized response");
            continue;
        }
        if modern
            && method != "server/discover"
            && request
                .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")
                .and_then(Value::as_str)
                != Some("2026-07-28")
        {
            let response = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32602,"message":"modern protocol metadata missing"}});
            writeln!(stdout, "{}", response).expect("write response");
            stdout.flush().expect("flush response");
            continue;
        }
        if method == "tools/call" && scenario == "exit-on-tool" {
            return;
        }
        let implementation = if scenario == "implementation-metadata" {
            json!({
                "name":"fixture",
                "version":"1",
                "title":"Fixture MCP",
                "icons":[{
                    "src":"data:image/png;base64,iVBORw0KGgo=",
                    "mimeType":"image/png",
                    "sizes":["16x16","any"],
                    "theme":"dark"
                }],
                "description":"Generic fixture metadata",
                "websiteUrl":"https://example.invalid/mcp"
            })
        } else {
            json!({"name":"fixture","version":"1"})
        };
        let result = match method {
            "initialize" => json!({
                "protocolVersion":if scenario == "unsupported-version" {"1900-01-01"} else {"2025-11-25"},
                "capabilities":{"tools":{},"prompts":{},"resources":{},"tasks":{}},
                "serverInfo":implementation
            }),
            "server/discover" => {
                modern = true;
                json!({
                    "resultType":"complete",
                    "supportedVersions":["2026-07-28"],
                    "capabilities":{"tools":{},"prompts":{},"resources":{},"tasks":{}},
                    "_meta":{"io.modelcontextprotocol/serverInfo":implementation}
                })
            }
            "tools/list" => {
                let cursor = request.pointer("/params/cursor").and_then(Value::as_str);
                let tool = |name: &str| {
                    json!({
                        "name":name,"description":"Echo one value",
                        "inputSchema":{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false},
                        "annotations":{"readOnlyHint":true,"destructiveHint":false},
                        "_meta":{"example.invalid/presentation":{"label":"fixture"}}
                    })
                };
                let local_tool = |name: &str, description: &str, argument: &str| {
                    json!({
                        "name":name,"description":description,
                        "inputSchema":{"type":"object","properties":{argument:{"type":"string"}},"required":[argument],"additionalProperties":false},
                        "annotations":{"readOnlyHint":true,"destructiveHint":false}
                    })
                };
                match scenario.as_str() {
                    // Local tools are MCP stdio servers: the live gates' deferred
                    // and parallel tools are served here.
                    "local-tools" => json!({"tools":[
                        local_tool("uat_marker","Emit the requested UAT marker.","marker"),
                        local_tool("get_shipping_eta","Return the shipping ETA for an order.","order_id"),
                        local_tool("record_left","Record the left value.","value"),
                        local_tool("record_right","Record the right value.","value"),
                    ]}),
                    "pagination" if cursor.is_none() => {
                        json!({"tools":[tool("page-one")],"nextCursor":"page-2"})
                    }
                    "pagination" => json!({"tools":[tool("page-two")]}),
                    "repeated-cursor" => {
                        json!({"tools":[tool(if cursor.is_none() {"one"} else {"two"})],"nextCursor":"same"})
                    }
                    "endless-pagination" => {
                        let page = cursor
                            .and_then(|value| value.parse::<u64>().ok())
                            .unwrap_or(0);
                        json!({"tools":[tool(&format!("tool-{page}"))],"nextCursor":format!("{}",page + 1)})
                    }
                    "task-augmented" | "official-task-shape" => {
                        let mut required = tool("echo");
                        required["execution"] = json!({"taskSupport":"required"});
                        json!({"tools":[required]})
                    }
                    _ => {
                        json!({"tools":[tool("echo")], "resultType":"complete", "ttlMs":1000, "cacheScope":"private"})
                    }
                }
            }
            "prompts/list" => json!({"prompts":[{"name":"hello","description":"Hello"}]}),
            "resources/list" => {
                json!({"resources":[{"uri":"fixture://one","name":"one","mimeType":"text/plain"}]})
            }
            "tools/call" => {
                assert_ne!(
                    scenario, "strict-task-routing",
                    "task resume must not execute tools/call"
                );
                if scenario == "strict-continuation" {
                    let params = &request["params"];
                    assert_eq!(params["name"], "confirm_action");
                    assert_eq!(params["arguments"], json!({"operation":"publish"}));
                    assert_eq!(params["requestState"], "confirm:publish");
                    assert_eq!(
                        params["inputResponses"],
                        json!({"confirmation":{"action":"accept"}})
                    );
                    for forbidden in ["continuation", "round", "taskId"] {
                        assert!(params.get(forbidden).is_none());
                    }
                    let response = json!({"jsonrpc":"2.0","id":id,"result":{"content":[{"type":"text","text":"confirmed:publish"}],"isError":false}});
                    writeln!(stdout, "{}", response).unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
                if scenario == "official-task-shape" {
                    // Official SEP-1686 (2025-11-25) shape: `ttl`/`pollInterval`
                    // keys, status-only tasks/get, payload via tasks/result.
                    let created = json!({"task":{"taskId":"task-official","status":"working","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:00Z","ttl":60000,"pollInterval":50}});
                    writeln!(
                        stdout,
                        "{}",
                        json!({"jsonrpc":"2.0","id":id,"result":created})
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
                if scenario == "task-augmented" {
                    let task = &request["params"]["task"];
                    if task
                        .get("ttl")
                        .and_then(Value::as_u64)
                        .is_none_or(|ttl| ttl == 0)
                    {
                        let response = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32602,"message":"tool requires task-augmented execution"}});
                        writeln!(stdout, "{}", response).unwrap();
                        stdout.flush().unwrap();
                        continue;
                    }
                    let created = json!({"task":{"taskId":"task-aug","status":"working","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:00Z","ttl":task["ttl"],"pollIntervalMs":50}});
                    writeln!(
                        stdout,
                        "{}",
                        json!({"jsonrpc":"2.0","id":id,"result":created})
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
                if scenario == "local-tools" {
                    let arguments = &request["params"]["arguments"];
                    let text = match request.pointer("/params/name").and_then(Value::as_str) {
                        Some("uat_marker") => json!({"marker": arguments["marker"]}),
                        Some("get_shipping_eta") => {
                            json!({"eta": match arguments["order_id"].as_str() {
                                Some("LIVE-42") => "ETA-3D",
                                Some("LIVE-43") => "ETA-5D",
                                _ => "ETA-UNKNOWN",
                            }})
                        }
                        Some("record_left" | "record_right") => {
                            json!({"recorded": arguments["value"]})
                        }
                        other => json!({"error": format!("unknown local tool {other:?}")}),
                    };
                    let response = json!({"jsonrpc":"2.0","id":id,"result":{"content":[{"type":"text","text":text.to_string()}],"isError":false}});
                    writeln!(stdout, "{}", response).expect("write local tool response");
                    stdout.flush().expect("flush local tool response");
                    continue;
                }
                if request.pointer("/params/name").and_then(Value::as_str) == Some("slow") {
                    std::thread::sleep(Duration::from_millis(150));
                }
                json!({"content":[{"type":"text","text":"ok"}],"isError":false})
            }
            "prompts/get" => {
                json!({"messages":[{"role":"user","content":{"type":"text","text":"hello"}}]})
            }
            "resources/read" => json!({"contents":[{"uri":"fixture://one","text":"one"}]}),
            "tasks/result" if scenario == "official-task-shape" => {
                assert_eq!(request["params"]["taskId"], "task-official");
                assert!(
                    task_polls >= 2,
                    "tasks/result is fetched only after a completed poll"
                );
                json!({"content":[{"type":"text","text":"official done"}],"isError":false})
            }
            "tasks/get" | "tasks/update" | "tasks/cancel" => {
                if scenario == "official-task-shape" {
                    assert_eq!(request["params"]["taskId"], "task-official");
                    task_polls += 1;
                    let result = if task_polls == 1 {
                        json!({"taskId":"task-official","status":"working","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:00Z","ttl":60000,"pollInterval":50})
                    } else {
                        json!({"taskId":"task-official","status":"completed","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:01Z","ttl":60000,"pollInterval":50})
                    };
                    writeln!(
                        stdout,
                        "{}",
                        json!({"jsonrpc":"2.0","id":id,"result":result})
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
                if scenario == "task-augmented" {
                    assert_eq!(request["params"]["taskId"], "task-aug");
                    // The first poll is still working and asks the client to
                    // wait 1.5 s (long enough for a worker to park and release
                    // its line); the next poll completes.
                    task_polls += 1;
                    let result = if method == "tasks/get" && task_polls == 1 {
                        json!({"taskId":"task-aug","status":"working","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:00Z","pollIntervalMs":1500})
                    } else {
                        json!({"taskId":"task-aug","status":"completed","createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:01Z","result":{"content":[{"type":"text","text":"augmented done"}],"isError":false}})
                    };
                    writeln!(
                        stdout,
                        "{}",
                        json!({"jsonrpc":"2.0","id":id,"result":result})
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
                if scenario == "strict-task-routing" {
                    assert_eq!(request["params"]["taskId"], "task-1");
                    if method == "tasks/update" {
                        assert_eq!(request["params"]["inputResponses"], json!({"answer":"yes"}));
                    }
                }
                json!({"taskId":"task-1","status":"completed","createdAt":"2026-09-04T00:00:00Z","lastUpdatedAt":"2026-09-04T00:00:01Z","result":{"content":[]}})
            }
            _ => {
                let response = json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"method not found"}});
                writeln!(stdout, "{}", response).expect("write response");
                stdout.flush().expect("flush response");
                continue;
            }
        };
        if method == "tools/list" {
            if scenario == "list-changed" {
                writeln!(
                    stdout,
                    "{}",
                    json!({"jsonrpc":"2.0","method":"notifications/tools/list_changed"})
                )
                .expect("write list-changed notification");
            }
            writeln!(
                stdout,
                "{}",
                json!({"jsonrpc":"2.0","method":"notifications/progress","params":{"progress":1,"progressToken":"fixture"}})
            )
            .expect("write progress notification");
        }
        let response = json!({"jsonrpc":"2.0","id":id,"result":result});
        writeln!(stdout, "{}", response).expect("write response");
        stdout.flush().expect("flush response");
    }
}
