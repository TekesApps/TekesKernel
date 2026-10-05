# mcp::bin::mcp-fixture-server

[Package atlas](index.md) · [Source](../../src/bin/mcp-fixture-server.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [mcp::bin::mcp-fixture-server::main](../../src/bin/mcp-fixture-server.rs#L7) | function_item | `private` | test;  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `io` | `std::io` | `private` |
| `BufRead` | `std::io::BufRead` | `private` |
| `Write` | `std::io::Write` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Stdio` | `std::process::Stdio` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `main` | `std::env::args().skip` | [8](../../src/bin/mcp-fixture-server.rs#L8) | receiver-type-required |
| `main` | `std::env::args` | [8](../../src/bin/mcp-fixture-server.rs#L8) | external-constructor-callback-or-unresolved |
| `main` | `arguments.next().unwrap_or_default` | [9](../../src/bin/mcp-fixture-server.rs#L9) | receiver-type-required |
| `main` | `arguments.next` | [9](../../src/bin/mcp-fixture-server.rs#L9), [11](../../src/bin/mcp-fixture-server.rs#L11) | receiver-type-required |
| `main` | `std::fs::write(pid_path, std::process::id().to_string()).expect` | [12](../../src/bin/mcp-fixture-server.rs#L12) | receiver-type-required |
| `main` | `std::fs::write` | [12](../../src/bin/mcp-fixture-server.rs#L12) | external-constructor-callback-or-unresolved |
| `main` | `std::process::id().to_string` | [12](../../src/bin/mcp-fixture-server.rs#L12) | receiver-type-required |
| `main` | `std::process::id` | [12](../../src/bin/mcp-fixture-server.rs#L12) | external-constructor-callback-or-unresolved |
| `main` | `(scenario == "descendant-inherits-stderr").then` | [15](../../src/bin/mcp-fixture-server.rs#L15) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args(["-c", "sleep 2"])             .stdin(Stdio::null())             .stdout(Stdio::null())             .stderr(Stdio::inherit())             .spawn()             .expect` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args(["-c", "sleep 2"])             .stdin(Stdio::null())             .stdout(Stdio::null())             .stderr(Stdio::inherit())             .spawn` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args(["-c", "sleep 2"])             .stdin(Stdio::null())             .stdout(Stdio::null())             .stderr` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args(["-c", "sleep 2"])             .stdin(Stdio::null())             .stdout` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args(["-c", "sleep 2"])             .stdin` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new("/bin/sh")             .args` | [16](../../src/bin/mcp-fixture-server.rs#L16) | receiver-type-required |
| `main` | `Command::new` | [16](../../src/bin/mcp-fixture-server.rs#L16) | external-constructor-callback-or-unresolved |
| `main` | `Stdio::null` | [18](../../src/bin/mcp-fixture-server.rs#L18), [19](../../src/bin/mcp-fixture-server.rs#L19) | external-constructor-callback-or-unresolved |
| `main` | `Stdio::inherit` | [20](../../src/bin/mcp-fixture-server.rs#L20) | external-constructor-callback-or-unresolved |
| `main` | `io::stdin` | [24](../../src/bin/mcp-fixture-server.rs#L24) | external-constructor-callback-or-unresolved |
| `main` | `io::stdout().lock` | [25](../../src/bin/mcp-fixture-server.rs#L25) | receiver-type-required |
| `main` | `io::stdout` | [25](../../src/bin/mcp-fixture-server.rs#L25) | external-constructor-callback-or-unresolved |
| `main` | `stdin.lock().lines` | [28](../../src/bin/mcp-fixture-server.rs#L28) | receiver-type-required |
| `main` | `stdin.lock` | [28](../../src/bin/mcp-fixture-server.rs#L28) | receiver-type-required |
| `main` | `serde_json::from_str::<Value>` | [30](../../src/bin/mcp-fixture-server.rs#L30) | external-constructor-callback-or-unresolved |
| `main` | `request.get("id").and_then` | [33](../../src/bin/mcp-fixture-server.rs#L33) | receiver-type-required |
| `main` | `request.get` | [33](../../src/bin/mcp-fixture-server.rs#L33), [36](../../src/bin/mcp-fixture-server.rs#L36) | receiver-type-required |
| `main` | `request.get("method").and_then(Value::as_str).unwrap_or` | [36](../../src/bin/mcp-fixture-server.rs#L36) | receiver-type-required |
| `main` | `request.get("method").and_then` | [36](../../src/bin/mcp-fixture-server.rs#L36) | receiver-type-required |
| `main` | `writeln!(                 stdout,                 "{{\"id\":{id},\"jsonrpc\":\"2.0\",\"result\":{{\"capabilities\":{{}},\"protocolVersion\":\"2025-11-25\",\"serverInfo\":{{\"name\":\"one\",\"name\":\"two\",\"version\":\"1\"}}}}}}"             )             .expect` | [38](../../src/bin/mcp-fixture-server.rs#L38) | receiver-type-required |
| `main` | `stdout.flush().expect` | [43](../../src/bin/mcp-fixture-server.rs#L43), [53](../../src/bin/mcp-fixture-server.rs#L53), [61](../../src/bin/mcp-fixture-server.rs#L61), [73](../../src/bin/mcp-fixture-server.rs#L73), [239](../../src/bin/mcp-fixture-server.rs#L239), [308](../../src/bin/mcp-fixture-server.rs#L308), [330](../../src/bin/mcp-fixture-server.rs#L330) | receiver-type-required |
| `main` | `stdout.flush` | [43](../../src/bin/mcp-fixture-server.rs#L43), [53](../../src/bin/mcp-fixture-server.rs#L53), [61](../../src/bin/mcp-fixture-server.rs#L61), [73](../../src/bin/mcp-fixture-server.rs#L73), [183](../../src/bin/mcp-fixture-server.rs#L183), [196](../../src/bin/mcp-fixture-server.rs#L196), [208](../../src/bin/mcp-fixture-server.rs#L208), [218](../../src/bin/mcp-fixture-server.rs#L218), [239](../../src/bin/mcp-fixture-server.rs#L239), [274](../../src/bin/mcp-fixture-server.rs#L274), [294](../../src/bin/mcp-fixture-server.rs#L294), [308](../../src/bin/mcp-fixture-server.rs#L308), [330](../../src/bin/mcp-fixture-server.rs#L330) | receiver-type-required |
| `main` | `writeln!(                 stdout,                 "{}",                 json!({"jsonrpc":"2.0","method":"notifications/unknown-required"})             )             .expect` | [47](../../src/bin/mcp-fixture-server.rs#L47) | receiver-type-required |
| `main` | `stdout                 .write_all(&vec![b'x'; 4 * 1024 * 1024 + 1])                 .expect` | [57](../../src/bin/mcp-fixture-server.rs#L57) | receiver-type-required |
| `main` | `stdout                 .write_all` | [57](../../src/bin/mcp-fixture-server.rs#L57) | receiver-type-required |
| `main` | `stdout.write_all(b"\n").expect` | [60](../../src/bin/mcp-fixture-server.rs#L60) | receiver-type-required |
| `main` | `stdout.write_all` | [60](../../src/bin/mcp-fixture-server.rs#L60) | receiver-type-required |
| `main` | `request                 .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")                 .and_then` | [66](../../src/bin/mcp-fixture-server.rs#L66) | receiver-type-required |
| `main` | `request                 .pointer` | [66](../../src/bin/mcp-fixture-server.rs#L66) | receiver-type-required |
| `main` | `Some` | [69](../../src/bin/mcp-fixture-server.rs#L69), [242](../../src/bin/mcp-fixture-server.rs#L242) | external-constructor-callback-or-unresolved |
| `main` | `writeln!(stdout, "{}", response).expect` | [72](../../src/bin/mcp-fixture-server.rs#L72), [238](../../src/bin/mcp-fixture-server.rs#L238), [307](../../src/bin/mcp-fixture-server.rs#L307), [329](../../src/bin/mcp-fixture-server.rs#L329) | receiver-type-required |
| `main` | `request.pointer("/params/cursor").and_then` | [112](../../src/bin/mcp-fixture-server.rs#L112) | receiver-type-required |
| `main` | `request.pointer` | [112](../../src/bin/mcp-fixture-server.rs#L112), [223](../../src/bin/mcp-fixture-server.rs#L223), [242](../../src/bin/mcp-fixture-server.rs#L242) | receiver-type-required |
| `main` | `scenario.as_str` | [128](../../src/bin/mcp-fixture-server.rs#L128) | receiver-type-required |
| `main` | `cursor.is_none` | [137](../../src/bin/mcp-fixture-server.rs#L137) | receiver-type-required |
| `main` | `cursor                             .and_then(&#124;value&#124; value.parse::<u64>().ok())                             .unwrap_or` | [145](../../src/bin/mcp-fixture-server.rs#L145) | receiver-type-required |
| `main` | `cursor                             .and_then` | [145](../../src/bin/mcp-fixture-server.rs#L145) | receiver-type-required |
| `main` | `value.parse::<u64>().ok` | [146](../../src/bin/mcp-fixture-server.rs#L146) | receiver-type-required |
| `main` | `value.parse::<u64>` | [146](../../src/bin/mcp-fixture-server.rs#L146) | receiver-type-required |
| `main` | `tool` | [151](../../src/bin/mcp-fixture-server.rs#L151) | external-constructor-callback-or-unresolved |
| `main` | `writeln!(stdout, "{}", response).unwrap` | [182](../../src/bin/mcp-fixture-server.rs#L182), [207](../../src/bin/mcp-fixture-server.rs#L207) | receiver-type-required |
| `main` | `stdout.flush().unwrap` | [183](../../src/bin/mcp-fixture-server.rs#L183), [196](../../src/bin/mcp-fixture-server.rs#L196), [208](../../src/bin/mcp-fixture-server.rs#L208), [218](../../src/bin/mcp-fixture-server.rs#L218), [274](../../src/bin/mcp-fixture-server.rs#L274), [294](../../src/bin/mcp-fixture-server.rs#L294) | receiver-type-required |
| `main` | `writeln!(                         stdout,                         "{}",                         json!({"jsonrpc":"2.0","id":id,"result":created})                     )                     .unwrap` | [190](../../src/bin/mcp-fixture-server.rs#L190), [212](../../src/bin/mcp-fixture-server.rs#L212) | receiver-type-required |
| `main` | `task                         .get("ttl")                         .and_then(Value::as_u64)                         .is_none_or` | [201](../../src/bin/mcp-fixture-server.rs#L201) | receiver-type-required |
| `main` | `task                         .get("ttl")                         .and_then` | [201](../../src/bin/mcp-fixture-server.rs#L201) | receiver-type-required |
| `main` | `task                         .get` | [201](../../src/bin/mcp-fixture-server.rs#L201) | receiver-type-required |
| `main` | `request.pointer("/params/name").and_then` | [223](../../src/bin/mcp-fixture-server.rs#L223), [242](../../src/bin/mcp-fixture-server.rs#L242) | receiver-type-required |
| `main` | `std::thread::sleep` | [243](../../src/bin/mcp-fixture-server.rs#L243) | external-constructor-callback-or-unresolved |
| `main` | `Duration::from_millis` | [243](../../src/bin/mcp-fixture-server.rs#L243) | external-constructor-callback-or-unresolved |
| `main` | `writeln!(                         stdout,                         "{}",                         json!({"jsonrpc":"2.0","id":id,"result":result})                     )                     .unwrap` | [268](../../src/bin/mcp-fixture-server.rs#L268), [288](../../src/bin/mcp-fixture-server.rs#L288) | receiver-type-required |
| `main` | `writeln!(                     stdout,                     "{}",                     json!({"jsonrpc":"2.0","method":"notifications/tools/list_changed"})                 )                 .expect` | [314](../../src/bin/mcp-fixture-server.rs#L314) | receiver-type-required |
| `main` | `writeln!(                 stdout,                 "{}",                 json!({"jsonrpc":"2.0","method":"notifications/progress","params":{"progress":1,"progressToken":"fixture"}})             )             .expect` | [321](../../src/bin/mcp-fixture-server.rs#L321) | receiver-type-required |
