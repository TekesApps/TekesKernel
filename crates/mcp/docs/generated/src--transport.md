# mcp::transport

[Package atlas](index.md) · [Source](../../src/transport.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [mcp::transport::McpMethodObserver](../../src/transport.rs#L15) | type_item | `pub` |  |
| [mcp::transport::McpValueObserver](../../src/transport.rs#L17) | type_item | `pub` |  |
| [mcp::transport::McpTransport](../../src/transport.rs#L28) | trait_item | `pub` |  |
| [mcp::transport::McpTransport::supports_http_discovery](../../src/transport.rs#L29) | function_item | `private` |  |
| [mcp::transport::McpTransport::start_catalog_subscription](../../src/transport.rs#L32) | function_item | `private` |  |
| [mcp::transport::McpTransport::install_tool_catalog](../../src/transport.rs#L41) | function_item | `private` |  |
| [mcp::transport::McpTransport::start_scoped_request](../../src/transport.rs#L51) | function_item | `private` |  |
| [mcp::transport::McpTransport::request](../../src/transport.rs#L61) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::notify](../../src/transport.rs#L67) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::close](../../src/transport.rs#L69) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::reconnect](../../src/transport.rs#L71) | function_item | `private` |  |
| [mcp::transport::McpTransport::mark_cancelled](../../src/transport.rs#L79) | function_item | `private` |  |
| [mcp::transport::McpTransport::take_notifications](../../src/transport.rs#L81) | function_item | `private` |  |
| [mcp::transport::StdioTransport](../../src/transport.rs#L86) | struct_item | `pub` |  |
| [mcp::transport::StdioSpawnSpec](../../src/transport.rs#L94) | struct_item | `private` |  |
| [mcp::transport::SecretText](../../src/transport.rs#L102) | struct_item | `private` |  |
| [mcp::transport::SecretText::new](../../src/transport.rs#L105) | function_item | `private` |  |
| [mcp::transport::SecretText::as_str](../../src/transport.rs#L109) | function_item | `private` |  |
| [mcp::transport::SecretText::drop](../../src/transport.rs#L115) | function_item | `private` |  |
| [mcp::transport::StdioParts](../../src/transport.rs#L120) | struct_item | `private` |  |
| [mcp::transport::StdioGeneration](../../src/transport.rs#L125) | struct_item | `private` |  |
| [mcp::transport::STDERR_POLL_INTERVAL](../../src/transport.rs#L133) | const_item | `private` |  |
| [mcp::transport::STDIO_REAP_TIMEOUT](../../src/transport.rs#L134) | const_item | `private` |  |
| [mcp::transport::StdioTransport::spawn](../../src/transport.rs#L137) | function_item | `pub` |  |
| [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L160) | function_item | `private` |  |
| [mcp::transport::StdioTransport::restart](../../src/transport.rs#L171) | function_item | `private` |  |
| [mcp::transport::StdioTransport::request_inner](../../src/transport.rs#L181) | function_item | `private` |  |
| [mcp::transport::StdioTransport::close_after_protocol_error](../../src/transport.rs#L203) | function_item | `private` |  |
| [mcp::transport::StdioTransport::write](../../src/transport.rs#L213) | function_item | `private` |  |
| [mcp::transport::StdioTransport::read_response_line](../../src/transport.rs#L239) | function_item | `private` |  |
| [mcp::transport::spawn_stdio](../../src/transport.rs#L279) | function_item | `private` |  |
| [mcp::transport::set_nonblocking](../../src/transport.rs#L358) | function_item | `private` |  |
| [mcp::transport::reap_stdio](../../src/transport.rs#L365) | function_item | `private` |  |
| [mcp::transport::start_stdio_reaper](../../src/transport.rs#L398) | function_item | `private` |  |
| [mcp::transport::await_stdio_reaper](../../src/transport.rs#L434) | function_item | `private` |  |
| [mcp::transport::StdioTransport::request](../../src/transport.rs#L449) | function_item | `private` |  |
| [mcp::transport::StdioTransport::notify](../../src/transport.rs#L460) | function_item | `private` |  |
| [mcp::transport::StdioTransport::close](../../src/transport.rs#L469) | function_item | `private` |  |
| [mcp::transport::StdioTransport::reconnect](../../src/transport.rs#L473) | function_item | `private` |  |
| [mcp::transport::StdioTransport::mark_cancelled](../../src/transport.rs#L477) | function_item | `private` |  |
| [mcp::transport::StdioTransport::take_notifications](../../src/transport.rs#L481) | function_item | `private` |  |
| [mcp::transport::StdioTransport::drop](../../src/transport.rs#L487) | function_item | `private` |  |
| [mcp::transport::HttpTransport](../../src/transport.rs#L494) | struct_item | `pub` |  |
| [mcp::transport::AuthorizationProvider](../../src/transport.rs#L509) | type_item | `private` |  |
| [mcp::transport::HttpTransport::drop](../../src/transport.rs#L513) | function_item | `private` |  |
| [mcp::transport::HttpRequestAuthorization](../../src/transport.rs#L521) | struct_item | `pub` |  |
| [mcp::transport::HttpRequestAuthorization::drop](../../src/transport.rs#L528) | function_item | `private` |  |
| [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L541) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L551) | function_item | `pub(crate)` |  |
| [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L564) | function_item | `pub(crate)` |  |
| [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L574) | function_item | `private` |  |
| [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L637) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::with_notification_handler](../../src/transport.rs#L681) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new](../../src/transport.rs#L688) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L713) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L737) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::request_headers](../../src/transport.rs#L794) | function_item | `private` |  |
| [mcp::transport::HttpTransport::start_catalog_subscription](../../src/transport.rs#L919) | function_item | `private` |  |
| [mcp::transport::HttpTransport::install_tool_catalog](../../src/transport.rs#L938) | function_item | `private` |  |
| [mcp::transport::HttpTransport::supports_http_discovery](../../src/transport.rs#L943) | function_item | `private` |  |
| [mcp::transport::HttpTransport::start_scoped_request](../../src/transport.rs#L947) | function_item | `private` |  |
| [mcp::transport::HttpTransport::request](../../src/transport.rs#L957) | function_item | `private` |  |
| [mcp::transport::HttpTransport::notify](../../src/transport.rs#L1116) | function_item | `private` |  |
| [mcp::transport::HttpTransport::close](../../src/transport.rs#L1145) | function_item | `private` |  |
| [mcp::transport::HttpTransport::reconnect](../../src/transport.rs#L1179) | function_item | `private` |  |
| [mcp::transport::HttpTransport::take_notifications](../../src/transport.rs#L1189) | function_item | `private` |  |
| [mcp::transport::SseDecoder](../../src/transport.rs#L1195) | struct_item | `private` |  |
| [mcp::transport::SseDecoder::push](../../src/transport.rs#L1202) | function_item | `private` |  |
| [mcp::transport::Inbound](../../src/transport.rs#L1246) | enum_item | `private` |  |
| [mcp::transport::InboundWire](../../src/transport.rs#L1254) | struct_item | `private` |  |
| [mcp::transport::decode_response](../../src/transport.rs#L1268) | function_item | `private` |  |
| [mcp::transport::decode_inbound](../../src/transport.rs#L1277) | function_item | `private` |  |
| [mcp::transport::tests::owned_scoped_requests_cannot_outlive_close_or_reconnect](../../src/transport.rs#L1354) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::duplicate_json_members_fail_before_value_collapse](../../src/transport.rs#L1434) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::cancelled_response_tombstone_is_consumed_once](../../src/transport.rs#L1448) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::sse_decoder_yields_the_first_complete_event_without_eof](../../src/transport.rs#L1470) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::timed_out_detached_reaper_does_not_delay_runtime_drop](../../src/transport.rs#L1489) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `File` | `std::fs::File` | `private` |
| `Read` | `std::io::Read` | `private` |
| `OwnedFd` | `std::os::fd::OwnedFd` | `private` |
| `CommandExt` | `std::os::unix::process::CommandExt` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Child` | `std::process::Child` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Stdio` | `std::process::Stdio` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `StreamExt` | `futures_util::StreamExt` | `private` |
| `_` | `base64::Engine` | `private` |
| `ACCEPT` | `reqwest::header::ACCEPT` | `private` |
| `AUTHORIZATION` | `reqwest::header::AUTHORIZATION` | `private` |
| `CONTENT_TYPE` | `reqwest::header::CONTENT_TYPE` | `private` |
| `HeaderMap` | `reqwest::header::HeaderMap` | `private` |
| `HeaderName` | `reqwest::header::HeaderName` | `private` |
| `HeaderValue` | `reqwest::header::HeaderValue` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `AsyncReadExt` | `tokio::io::AsyncReadExt` | `private` |
| `AsyncWriteExt` | `tokio::io::AsyncWriteExt` | `private` |
| `pipe` | `tokio::net::unix::pipe` | `private` |
| `Url` | `url::Url` | `private` |
| `JsonRpcError` | `crate::JsonRpcError` | `private` |
| `JsonRpcRequest` | `crate::JsonRpcRequest` | `private` |
| `JsonRpcResponse` | `crate::JsonRpcResponse` | `private` |
| `MAX_FRAME_BYTES` | `crate::MAX_FRAME_BYTES` | `private` |
| `McpError` | `crate::McpError` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `mcp::transport::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 12 direct edges</summary>

```mermaid
flowchart TD
  n0["mcp::parameter_headers::ParameterHeaders::catalog"]
  n1["mcp::transport::SecretText::new"]
  n2["mcp::transport::SecretText::as_str"]
  n3["mcp::transport::SecretText::drop"]
  n4["mcp::transport::decode_inbound"]
  n5["mcp::transport::StdioTransport::spawn"]
  n6["mcp::transport::StdioTransport::terminate"]
  n7["mcp::transport::StdioTransport::restart"]
  n8["mcp::transport::StdioTransport::request_inner"]
  n9["mcp::transport::StdioTransport::close_after_protocol_error"]
  n10["mcp::transport::StdioTransport::write"]
  n11["mcp::transport::StdioTransport::read_response_line"]
  n12["mcp::transport::spawn_stdio"]
  n13["mcp::transport::McpTransport::supports_http_discovery"]
  n14["mcp::transport::McpTransport::start_catalog_subscription"]
  n15["mcp::transport::set_nonblocking"]
  n16["mcp::transport::reap_stdio"]
  n17["mcp::transport::start_stdio_reaper"]
  n18["mcp::transport::McpTransport::install_tool_catalog"]
  n19["mcp::transport::await_stdio_reaper"]
  n20["mcp::transport::McpTransport::start_scoped_request"]
  n21["mcp::transport::McpTransport::reconnect"]
  n22["mcp::transport::McpTransport::mark_cancelled"]
  n23["mcp::transport::McpTransport::take_notifications"]
  n5 --> n1
  n5 --> n12
  n6 --> n17
  n6 --> n19
  n7 --> n6
  n7 --> n12
  n8 --> n4
  n8 --> n10
  n8 --> n11
  n9 --> n6
  n12 --> n15
  n18 --> n0
```

</details>

<details><summary>Functions 21–40: 14 direct edges</summary>

```mermaid
flowchart TD
  n0["mcp::subscription::McpSubscriptionFilter::new"]
  n1["mcp::transport::SecretText::new"]
  n2["mcp::transport::reap_stdio"]
  n3["mcp::transport::start_stdio_reaper"]
  n4["mcp::transport::await_stdio_reaper"]
  n5["mcp::transport::StdioTransport::request"]
  n6["mcp::transport::StdioTransport::notify"]
  n7["mcp::transport::StdioTransport::close"]
  n8["mcp::transport::StdioTransport::reconnect"]
  n9["mcp::transport::StdioTransport::mark_cancelled"]
  n10["mcp::transport::StdioTransport::take_notifications"]
  n11["mcp::transport::StdioTransport::drop"]
  n12["mcp::transport::HttpTransport::drop"]
  n13["mcp::transport::HttpRequestAuthorization::drop"]
  n14["mcp::transport::HttpTransport::request_modern_scoped"]
  n15["mcp::transport::HttpTransport::request_modern_scoped_observed"]
  n16["mcp::transport::HttpTransport::start_modern_scoped"]
  n17["mcp::transport::HttpTransport::start_modern_scoped_detailed"]
  n18["mcp::transport::HttpTransport::listen_subscription"]
  n19["mcp::transport::HttpTransport::with_notification_handler"]
  n20["mcp::transport::HttpTransport::new"]
  n21["mcp::transport::HttpTransport::new_with_authorization_provider"]
  n22["mcp::transport::HttpTransport::new_with_request_authorization_provider"]
  n23["mcp::types::JsonRpcRequest::call"]
  n24["schema::ijson::IJsonValue::parse_str"]
  n3 --> n2
  n11 --> n3
  n14 --> n15
  n15 --> n16
  n16 --> n17
  n17 --> n1
  n18 --> n0
  n18 --> n17
  n18 --> n23
  n18 --> n24
  n20 --> n1
  n20 --> n21
  n21 --> n22
  n22 --> n1
```

</details>

<details><summary>Functions 41–53: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["mcp::parameter_headers::ParameterHeaders::catalog"]
  n1["mcp::subscription::McpSubscriptionFilter::new"]
  n2["mcp::transport::HttpTransport::notify"]
  n3["mcp::transport::HttpTransport::close"]
  n4["mcp::transport::HttpTransport::reconnect"]
  n5["mcp::transport::HttpTransport::take_notifications"]
  n6["mcp::transport::SseDecoder::push"]
  n7["mcp::transport::decode_response"]
  n8["mcp::transport::decode_inbound"]
  n9["mcp::transport::HttpTransport::request_headers"]
  n10["mcp::transport::HttpTransport::start_catalog_subscription"]
  n11["mcp::transport::HttpTransport::install_tool_catalog"]
  n12["mcp::transport::HttpTransport::supports_http_discovery"]
  n13["mcp::transport::HttpTransport::start_scoped_request"]
  n14["mcp::transport::HttpTransport::request"]
  n15["mcp::types::JsonRpcRequest::notification"]
  n16["schema::ijson::IJsonValue::parse"]
  n3 --> n15
  n7 --> n8
  n10 --> n1
  n11 --> n0
  n14 --> n7
  n14 --> n8
  n14 --> n16
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `install_tool_catalog` | `crate::parameter_headers::ParameterHeaders::catalog` | [42](../../src/transport.rs#L42) | [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) |
| `install_tool_catalog` | `catalog.values().any` | [43](../../src/transport.rs#L43) | receiver-type-required |
| `install_tool_catalog` | `catalog.values` | [43](../../src/transport.rs#L43) | receiver-type-required |
| `install_tool_catalog` | `projection.is_empty` | [43](../../src/transport.rs#L43) | receiver-type-required |
| `install_tool_catalog` | `Err` | [44](../../src/transport.rs#L44) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `McpError::Unsupported` | [44](../../src/transport.rs#L44) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `"transport cannot carry MCP parameter headers".into` | [45](../../src/transport.rs#L45) | receiver-type-required |
| `install_tool_catalog` | `Ok` | [48](../../src/transport.rs#L48) | external-constructor-callback-or-unresolved |
| `reconnect` | `Box::pin` | [72](../../src/transport.rs#L72) | external-constructor-callback-or-unresolved |
| `reconnect` | `Err` | [73](../../src/transport.rs#L73) | external-constructor-callback-or-unresolved |
| `reconnect` | `McpError::Transport` | [73](../../src/transport.rs#L73) | external-constructor-callback-or-unresolved |
| `reconnect` | `"MCP transport does not support reconnect".to_owned` | [74](../../src/transport.rs#L74) | receiver-type-required |
| `take_notifications` | `Vec::new` | [82](../../src/transport.rs#L82) | external-constructor-callback-or-unresolved |
| `new` | `Self` | [106](../../src/transport.rs#L106) | external-constructor-callback-or-unresolved |
| `drop` | `self.0.clear` | [116](../../src/transport.rs#L116) | receiver-type-required |
| `STDERR_POLL_INTERVAL` | `Duration::from_millis` | [133](../../src/transport.rs#L133) | external-constructor-callback-or-unresolved |
| `STDIO_REAP_TIMEOUT` | `Duration::from_secs` | [134](../../src/transport.rs#L134) | external-constructor-callback-or-unresolved |
| `spawn` | `argv.to_vec` | [143](../../src/transport.rs#L143) | receiver-type-required |
| `spawn` | `environment                 .iter()                 .map(&#124;(name, value)&#124; (name.clone(), SecretText::new(value.clone())))                 .collect` | [145](../../src/transport.rs#L145) | receiver-type-required |
| `spawn` | `environment                 .iter()                 .map` | [145](../../src/transport.rs#L145) | receiver-type-required |
| `spawn` | `environment                 .iter` | [145](../../src/transport.rs#L145) | receiver-type-required |
| `spawn` | `name.clone` | [147](../../src/transport.rs#L147) | receiver-type-required |
| `spawn` | `SecretText::new` | [147](../../src/transport.rs#L147) | [mcp::transport::SecretText::new](../../src/transport.rs#L105) |
| `spawn` | `value.clone` | [147](../../src/transport.rs#L147) | receiver-type-required |
| `spawn` | `spawn_stdio` | [150](../../src/transport.rs#L150) | [mcp::transport::spawn_stdio](../../src/transport.rs#L279) |
| `spawn` | `Ok` | [151](../../src/transport.rs#L151) | external-constructor-callback-or-unresolved |
| `spawn` | `Some` | [152](../../src/transport.rs#L152) | external-constructor-callback-or-unresolved |
| `spawn` | `BTreeSet::new` | [155](../../src/transport.rs#L155) | external-constructor-callback-or-unresolved |
| `spawn` | `Vec::new` | [156](../../src/transport.rs#L156) | external-constructor-callback-or-unresolved |
| `terminate` | `self.generation.take` | [161](../../src/transport.rs#L161) | receiver-type-required |
| `terminate` | `Ok` | [162](../../src/transport.rs#L162) | external-constructor-callback-or-unresolved |
| `terminate` | `start_stdio_reaper` | [167](../../src/transport.rs#L167) | [mcp::transport::start_stdio_reaper](../../src/transport.rs#L398) |
| `terminate` | `await_stdio_reaper` | [168](../../src/transport.rs#L168) | [mcp::transport::await_stdio_reaper](../../src/transport.rs#L434) |
| `restart` | `self.terminate` | [172](../../src/transport.rs#L172) | [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L160) |
| `restart` | `spawn_stdio` | [173](../../src/transport.rs#L173) | [mcp::transport::spawn_stdio](../../src/transport.rs#L279) |
| `restart` | `Some` | [174](../../src/transport.rs#L174) | external-constructor-callback-or-unresolved |
| `restart` | `self.cancelled_ids.clear` | [176](../../src/transport.rs#L176) | receiver-type-required |
| `restart` | `self.notifications.clear` | [177](../../src/transport.rs#L177) | receiver-type-required |
| `restart` | `Ok` | [178](../../src/transport.rs#L178) | external-constructor-callback-or-unresolved |
| `request_inner` | `request             .id             .ok_or_else` | [186](../../src/transport.rs#L186) | receiver-type-required |
| `request_inner` | `McpError::Protocol` | [188](../../src/transport.rs#L188) | external-constructor-callback-or-unresolved |
| `request_inner` | `"request id is absent".to_owned` | [188](../../src/transport.rs#L188) | receiver-type-required |
| `request_inner` | `self.write` | [189](../../src/transport.rs#L189) | [mcp::transport::StdioTransport::write](../../src/transport.rs#L213) |
| `request_inner` | `tokio::time::Instant::now` | [190](../../src/transport.rs#L190) | external-constructor-callback-or-unresolved |
| `request_inner` | `tokio::time::timeout_at(deadline, self.read_response_line())                 .await                 .map_err` | [192](../../src/transport.rs#L192) | receiver-type-required |
| `request_inner` | `tokio::time::timeout_at` | [192](../../src/transport.rs#L192) | external-constructor-callback-or-unresolved |
| `request_inner` | `self.read_response_line` | [192](../../src/transport.rs#L192) | [mcp::transport::StdioTransport::read_response_line](../../src/transport.rs#L239) |
| `request_inner` | `McpError::Timeout` | [194](../../src/transport.rs#L194) | external-constructor-callback-or-unresolved |
| `request_inner` | `request.method.clone` | [194](../../src/transport.rs#L194) | receiver-type-required |
| `request_inner` | `decode_inbound` | [195](../../src/transport.rs#L195) | [mcp::transport::decode_inbound](../../src/transport.rs#L1277) |
| `request_inner` | `Ok` | [196](../../src/transport.rs#L196) | external-constructor-callback-or-unresolved |
| `request_inner` | `self.notifications.push` | [197](../../src/transport.rs#L197) | receiver-type-required |
| `close_after_protocol_error` | `self.terminate` | [208](../../src/transport.rs#L208) | [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L160) |
| `write` | `request.canonical_line` | [214](../../src/transport.rs#L214) | receiver-type-required |
| `write` | `bytes.len` | [215](../../src/transport.rs#L215) | receiver-type-required |
| `write` | `Err` | [216](../../src/transport.rs#L216) | external-constructor-callback-or-unresolved |
| `write` | `McpError::Protocol` | [216](../../src/transport.rs#L216) | external-constructor-callback-or-unresolved |
| `write` | `"outbound frame exceeds 4 MiB".to_owned` | [217](../../src/transport.rs#L217) | receiver-type-required |
| `write` | `pipe::Sender::from_file(             self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone()                 .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?,         )         .map_err` | [220](../../src/transport.rs#L220) | receiver-type-required |
| `write` | `pipe::Sender::from_file` | [220](../../src/transport.rs#L220) | external-constructor-callback-or-unresolved |
| `write` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone()                 .map_err` | [221](../../src/transport.rs#L221) | receiver-type-required |
| `write` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone` | [221](../../src/transport.rs#L221) | receiver-type-required |
| `write` | `self.generation                 .as_ref()                 .ok_or_else` | [221](../../src/transport.rs#L221) | receiver-type-required |
| `write` | `self.generation                 .as_ref` | [221](../../src/transport.rs#L221) | receiver-type-required |
| `write` | `McpError::Transport` | [223](../../src/transport.rs#L223), [226](../../src/transport.rs#L226), [228](../../src/transport.rs#L228), [232](../../src/transport.rs#L232), [236](../../src/transport.rs#L236) | external-constructor-callback-or-unresolved |
| `write` | `"stdio generation is closed".to_owned` | [223](../../src/transport.rs#L223) | receiver-type-required |
| `write` | `error.to_string` | [226](../../src/transport.rs#L226), [228](../../src/transport.rs#L228), [232](../../src/transport.rs#L232), [236](../../src/transport.rs#L236) | receiver-type-required |
| `write` | `stdin             .write_all(&bytes)             .await             .map_err` | [229](../../src/transport.rs#L229) | receiver-type-required |
| `write` | `stdin             .write_all` | [229](../../src/transport.rs#L229) | receiver-type-required |
| `write` | `stdin             .flush()             .await             .map_err` | [233](../../src/transport.rs#L233) | receiver-type-required |
| `write` | `stdin             .flush` | [233](../../src/transport.rs#L233) | receiver-type-required |
| `read_response_line` | `Vec::new` | [240](../../src/transport.rs#L240) | external-constructor-callback-or-unresolved |
| `read_response_line` | `pipe::Receiver::from_file(             self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone()                 .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?,         )         .map_err` | [241](../../src/transport.rs#L241) | receiver-type-required |
| `read_response_line` | `pipe::Receiver::from_file` | [241](../../src/transport.rs#L241) | external-constructor-callback-or-unresolved |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone()                 .map_err` | [242](../../src/transport.rs#L242) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone` | [242](../../src/transport.rs#L242) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else` | [242](../../src/transport.rs#L242) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref` | [242](../../src/transport.rs#L242) | receiver-type-required |
| `read_response_line` | `McpError::Transport` | [244](../../src/transport.rs#L244), [247](../../src/transport.rs#L247), [249](../../src/transport.rs#L249), [255](../../src/transport.rs#L255), [257](../../src/transport.rs#L257) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"stdio generation is closed".to_owned` | [244](../../src/transport.rs#L244) | receiver-type-required |
| `read_response_line` | `error.to_string` | [247](../../src/transport.rs#L247), [249](../../src/transport.rs#L249), [255](../../src/transport.rs#L255) | receiver-type-required |
| `read_response_line` | `bytes.len` | [250](../../src/transport.rs#L250), [264](../../src/transport.rs#L264) | receiver-type-required |
| `read_response_line` | `stdout                 .read(&mut byte)                 .await                 .map_err` | [252](../../src/transport.rs#L252) | receiver-type-required |
| `read_response_line` | `stdout                 .read` | [252](../../src/transport.rs#L252) | receiver-type-required |
| `read_response_line` | `Err` | [257](../../src/transport.rs#L257), [265](../../src/transport.rs#L265), [271](../../src/transport.rs#L271) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"stdio EOF".to_owned` | [257](../../src/transport.rs#L257) | receiver-type-required |
| `read_response_line` | `bytes.push` | [259](../../src/transport.rs#L259) | receiver-type-required |
| `read_response_line` | `bytes.last` | [264](../../src/transport.rs#L264), [270](../../src/transport.rs#L270) | receiver-type-required |
| `read_response_line` | `Some` | [264](../../src/transport.rs#L264), [270](../../src/transport.rs#L270) | external-constructor-callback-or-unresolved |
| `read_response_line` | `McpError::Protocol` | [265](../../src/transport.rs#L265), [271](../../src/transport.rs#L271) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"inbound frame exceeds 4 MiB or lacks LF framing".to_owned` | [266](../../src/transport.rs#L266) | receiver-type-required |
| `read_response_line` | `bytes.pop` | [269](../../src/transport.rs#L269) | receiver-type-required |
| `read_response_line` | `"stdio frames require LF rather than CRLF".to_owned` | [272](../../src/transport.rs#L272) | receiver-type-required |
| `read_response_line` | `Ok` | [275](../../src/transport.rs#L275) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `spawn.cwd.clone` | [281](../../src/transport.rs#L281) | receiver-type-required |
| `spawn_stdio` | `argv         .split_first()         .ok_or_else` | [283](../../src/transport.rs#L283) | receiver-type-required |
| `spawn_stdio` | `argv         .split_first` | [283](../../src/transport.rs#L283) | receiver-type-required |
| `spawn_stdio` | `McpError::Transport` | [285](../../src/transport.rs#L285), [302](../../src/transport.rs#L302), [306](../../src/transport.rs#L306), [310](../../src/transport.rs#L310), [314](../../src/transport.rs#L314), [345](../../src/transport.rs#L345) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `"stdio argv is empty".to_owned` | [285](../../src/transport.rs#L285) | receiver-type-required |
| `spawn_stdio` | `Command::new` | [286](../../src/transport.rs#L286) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin(Stdio::piped())         .stdout(Stdio::piped())         .stderr` | [287](../../src/transport.rs#L287) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin(Stdio::piped())         .stdout` | [287](../../src/transport.rs#L287) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin` | [287](../../src/transport.rs#L287) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear` | [287](../../src/transport.rs#L287) | receiver-type-required |
| `spawn_stdio` | `command         .args` | [287](../../src/transport.rs#L287) | receiver-type-required |
| `spawn_stdio` | `Stdio::piped` | [290](../../src/transport.rs#L290), [291](../../src/transport.rs#L291), [292](../../src/transport.rs#L292) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `command.env` | [294](../../src/transport.rs#L294) | receiver-type-required |
| `spawn_stdio` | `value.as_str` | [294](../../src/transport.rs#L294) | receiver-type-required |
| `spawn_stdio` | `command.current_dir` | [297](../../src/transport.rs#L297) | receiver-type-required |
| `spawn_stdio` | `command.process_group` | [299](../../src/transport.rs#L299) | receiver-type-required |
| `spawn_stdio` | `command         .spawn()         .map_err` | [300](../../src/transport.rs#L300) | receiver-type-required |
| `spawn_stdio` | `command         .spawn` | [300](../../src/transport.rs#L300) | receiver-type-required |
| `spawn_stdio` | `error.to_string` | [302](../../src/transport.rs#L302), [345](../../src/transport.rs#L345) | receiver-type-required |
| `spawn_stdio` | `child         .stdin         .take()         .ok_or_else` | [303](../../src/transport.rs#L303) | receiver-type-required |
| `spawn_stdio` | `child         .stdin         .take` | [303](../../src/transport.rs#L303) | receiver-type-required |
| `spawn_stdio` | `"stdio stdin is absent".to_owned` | [306](../../src/transport.rs#L306) | receiver-type-required |
| `spawn_stdio` | `child         .stdout         .take()         .ok_or_else` | [307](../../src/transport.rs#L307) | receiver-type-required |
| `spawn_stdio` | `child         .stdout         .take` | [307](../../src/transport.rs#L307) | receiver-type-required |
| `spawn_stdio` | `"stdio stdout is absent".to_owned` | [310](../../src/transport.rs#L310) | receiver-type-required |
| `spawn_stdio` | `child         .stderr         .take()         .ok_or_else` | [311](../../src/transport.rs#L311) | receiver-type-required |
| `spawn_stdio` | `child         .stderr         .take` | [311](../../src/transport.rs#L311) | receiver-type-required |
| `spawn_stdio` | `"stdio stderr is absent".to_owned` | [314](../../src/transport.rs#L314) | receiver-type-required |
| `spawn_stdio` | `File::from` | [315](../../src/transport.rs#L315), [349](../../src/transport.rs#L349), [350](../../src/transport.rs#L350) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `OwnedFd::from` | [315](../../src/transport.rs#L315), [349](../../src/transport.rs#L349), [350](../../src/transport.rs#L350) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `set_nonblocking` | [316](../../src/transport.rs#L316) | [mcp::transport::set_nonblocking](../../src/transport.rs#L358) |
| `spawn_stdio` | `Arc::new` | [317](../../src/transport.rs#L317), [319](../../src/transport.rs#L319) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Mutex::new` | [317](../../src/transport.rs#L317) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Vec::new` | [317](../../src/transport.rs#L317) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Arc::clone` | [318](../../src/transport.rs#L318), [320](../../src/transport.rs#L320) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `AtomicBool::new` | [319](../../src/transport.rs#L319) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `std::thread::Builder::new()         .name("tekes-mcp-stderr".to_owned())         .spawn(move &#124;&#124; {             let mut buffer = [0_u8; 8 * 1024];             while !stderr_task_stop.load(Ordering::Acquire) {                 let count = match stderr.read(&mut buffer) {                     Ok(0) => break,                     Ok(count) => count,                     Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {                         std::thread::sleep(STDERR_POLL_INTERVAL);                         continue;                     }                     Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,                     Err(_) => break,                 };                 if count > 0 {                     let mut retained = diagnostics_writer                         .lock()                         .unwrap_or_else(std::sync::PoisonError::into_inner);                     let remaining = (64_usize * 1024).saturating_sub(retained.len());                     retained.extend_from_slice(&buffer[..count.min(remaining)]);                 }             }         })         .map_err` | [321](../../src/transport.rs#L321) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new()         .name("tekes-mcp-stderr".to_owned())         .spawn` | [321](../../src/transport.rs#L321) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new()         .name` | [321](../../src/transport.rs#L321) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new` | [321](../../src/transport.rs#L321) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `"tekes-mcp-stderr".to_owned` | [322](../../src/transport.rs#L322) | receiver-type-required |
| `spawn_stdio` | `stderr_task_stop.load` | [325](../../src/transport.rs#L325) | receiver-type-required |
| `spawn_stdio` | `stderr.read` | [326](../../src/transport.rs#L326) | receiver-type-required |
| `spawn_stdio` | `error.kind` | [329](../../src/transport.rs#L329), [333](../../src/transport.rs#L333) | receiver-type-required |
| `spawn_stdio` | `std::thread::sleep` | [330](../../src/transport.rs#L330) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `diagnostics_writer                         .lock()                         .unwrap_or_else` | [337](../../src/transport.rs#L337) | receiver-type-required |
| `spawn_stdio` | `diagnostics_writer                         .lock` | [337](../../src/transport.rs#L337) | receiver-type-required |
| `spawn_stdio` | `(64_usize * 1024).saturating_sub` | [340](../../src/transport.rs#L340) | receiver-type-required |
| `spawn_stdio` | `retained.len` | [340](../../src/transport.rs#L340) | receiver-type-required |
| `spawn_stdio` | `retained.extend_from_slice` | [341](../../src/transport.rs#L341) | receiver-type-required |
| `spawn_stdio` | `count.min` | [341](../../src/transport.rs#L341) | receiver-type-required |
| `spawn_stdio` | `Ok` | [346](../../src/transport.rs#L346) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `rustix::fs::fcntl_getfl(file).map_err` | [360](../../src/transport.rs#L360) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_getfl` | [360](../../src/transport.rs#L360) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `McpError::Transport` | [360](../../src/transport.rs#L360), [362](../../src/transport.rs#L362) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `error.to_string` | [360](../../src/transport.rs#L360), [362](../../src/transport.rs#L362) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_setfl(file, flags &#124; rustix::fs::OFlags::NONBLOCK)         .map_err` | [361](../../src/transport.rs#L361) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_setfl` | [361](../../src/transport.rs#L361) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `generation.stderr_stop.store` | [366](../../src/transport.rs#L366) | receiver-type-required |
| `reap_stdio` | `drop` | [367](../../src/transport.rs#L367), [368](../../src/transport.rs#L368) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `rustix::process::Pid::from_raw` | [369](../../src/transport.rs#L369) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `generation.child.id` | [369](../../src/transport.rs#L369) | receiver-type-required |
| `reap_stdio` | `rustix::process::kill_process_group` | [372](../../src/transport.rs#L372) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `Err` | [374](../../src/transport.rs#L374) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `McpError::Transport` | [374](../../src/transport.rs#L374), [380](../../src/transport.rs#L380), [386](../../src/transport.rs#L386), [391](../../src/transport.rs#L391), [395](../../src/transport.rs#L395) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `error.to_string` | [374](../../src/transport.rs#L374), [380](../../src/transport.rs#L380), [386](../../src/transport.rs#L386), [391](../../src/transport.rs#L391) | receiver-type-required |
| `reap_stdio` | `generation         .child         .try_wait()         .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?         .is_none` | [377](../../src/transport.rs#L377) | receiver-type-required |
| `reap_stdio` | `generation         .child         .try_wait()         .map_err` | [377](../../src/transport.rs#L377) | receiver-type-required |
| `reap_stdio` | `generation         .child         .try_wait` | [377](../../src/transport.rs#L377) | receiver-type-required |
| `reap_stdio` | `generation             .child             .kill()             .map_err` | [383](../../src/transport.rs#L383) | receiver-type-required |
| `reap_stdio` | `generation             .child             .kill` | [383](../../src/transport.rs#L383) | receiver-type-required |
| `reap_stdio` | `generation         .child         .wait()         .map_err` | [388](../../src/transport.rs#L388) | receiver-type-required |
| `reap_stdio` | `generation         .child         .wait` | [388](../../src/transport.rs#L388) | receiver-type-required |
| `reap_stdio` | `generation         .stderr_task         .join()         .map_err` | [392](../../src/transport.rs#L392) | receiver-type-required |
| `reap_stdio` | `generation         .stderr_task         .join` | [392](../../src/transport.rs#L392) | receiver-type-required |
| `reap_stdio` | `"stdio stderr drain thread panicked".to_owned` | [395](../../src/transport.rs#L395) | receiver-type-required |
| `start_stdio_reaper` | `Arc::new` | [401](../../src/transport.rs#L401) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Mutex::new` | [401](../../src/transport.rs#L401) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Some` | [401](../../src/transport.rs#L401) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Arc::clone` | [402](../../src/transport.rs#L402) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `tokio::sync::oneshot::channel` | [403](../../src/transport.rs#L403) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `std::thread::Builder::new()         .name("tekes-mcp-reaper".to_owned())         .spawn` | [404](../../src/transport.rs#L404) | receiver-type-required |
| `start_stdio_reaper` | `std::thread::Builder::new()         .name` | [404](../../src/transport.rs#L404) | receiver-type-required |
| `start_stdio_reaper` | `std::thread::Builder::new` | [404](../../src/transport.rs#L404) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `"tekes-mcp-reaper".to_owned` | [405](../../src/transport.rs#L405) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take()                 .expect` | [407](../../src/transport.rs#L407) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take` | [407](../../src/transport.rs#L407) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else` | [407](../../src/transport.rs#L407) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock` | [407](../../src/transport.rs#L407) | receiver-type-required |
| `start_stdio_reaper` | `sender.send` | [412](../../src/transport.rs#L412) | receiver-type-required |
| `start_stdio_reaper` | `reap_stdio` | [412](../../src/transport.rs#L412) | [mcp::transport::reap_stdio](../../src/transport.rs#L365) |
| `start_stdio_reaper` | `Ok` | [414](../../src/transport.rs#L414) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take` | [416](../../src/transport.rs#L416) | receiver-type-required |
| `start_stdio_reaper` | `generation                 .lock()                 .unwrap_or_else` | [416](../../src/transport.rs#L416) | receiver-type-required |
| `start_stdio_reaper` | `generation                 .lock` | [416](../../src/transport.rs#L416) | receiver-type-required |
| `start_stdio_reaper` | `generation.stderr_stop.store` | [424](../../src/transport.rs#L424) | receiver-type-required |
| `start_stdio_reaper` | `generation.child.kill` | [425](../../src/transport.rs#L425) | receiver-type-required |
| `start_stdio_reaper` | `Err` | [427](../../src/transport.rs#L427) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `McpError::Transport` | [427](../../src/transport.rs#L427) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `tokio::time::timeout` | [437](../../src/transport.rs#L437) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `Err` | [439](../../src/transport.rs#L439), [442](../../src/transport.rs#L442) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `McpError::Transport` | [439](../../src/transport.rs#L439), [442](../../src/transport.rs#L442) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `"stdio reaper thread exited without a result".to_owned` | [440](../../src/transport.rs#L440) | receiver-type-required |
| `await_stdio_reaper` | `"stdio generation reap exceeded one second".to_owned` | [443](../../src/transport.rs#L443) | receiver-type-required |
| `request` | `Box::pin` | [454](../../src/transport.rs#L454) | external-constructor-callback-or-unresolved |
| `request` | `self.request_inner` | [455](../../src/transport.rs#L455) | receiver-type-required |
| `request` | `self.close_after_protocol_error` | [456](../../src/transport.rs#L456) | receiver-type-required |
| `notify` | `Box::pin` | [461](../../src/transport.rs#L461) | external-constructor-callback-or-unresolved |
| `notify` | `notification.id.is_some` | [462](../../src/transport.rs#L462) | receiver-type-required |
| `notify` | `Err` | [463](../../src/transport.rs#L463) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Protocol` | [463](../../src/transport.rs#L463) | external-constructor-callback-or-unresolved |
| `notify` | `"notification has an id".to_owned` | [463](../../src/transport.rs#L463) | receiver-type-required |
| `notify` | `self.write` | [465](../../src/transport.rs#L465) | receiver-type-required |
| `close` | `Box::pin` | [470](../../src/transport.rs#L470) | external-constructor-callback-or-unresolved |
| `close` | `self.terminate` | [470](../../src/transport.rs#L470) | receiver-type-required |
| `reconnect` | `Box::pin` | [474](../../src/transport.rs#L474) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.restart` | [474](../../src/transport.rs#L474) | receiver-type-required |
| `mark_cancelled` | `self.cancelled_ids.insert` | [478](../../src/transport.rs#L478) | receiver-type-required |
| `take_notifications` | `std::mem::take` | [482](../../src/transport.rs#L482) | external-constructor-callback-or-unresolved |
| `drop` | `self.generation.take` | [488](../../src/transport.rs#L488) | receiver-type-required |
| `drop` | `start_stdio_reaper` | [489](../../src/transport.rs#L489) | [mcp::transport::start_stdio_reaper](../../src/transport.rs#L398) |
| `drop` | `self.scoped_generation.cancel` | [514](../../src/transport.rs#L514) | receiver-type-required |
| `drop` | `self.headers.values_mut` | [529](../../src/transport.rs#L529) | receiver-type-required |
| `drop` | `value.clear` | [530](../../src/transport.rs#L530), [533](../../src/transport.rs#L533) | receiver-type-required |
| `request_modern_scoped` | `self.request_modern_scoped_observed` | [547](../../src/transport.rs#L547) | [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L551) |
| `request_modern_scoped_observed` | `self.start_modern_scoped` | [558](../../src/transport.rs#L558) | [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L564) |
| `request_modern_scoped_observed` | `request.clone` | [558](../../src/transport.rs#L558) | receiver-type-required |
| `start_modern_scoped` | `self.start_modern_scoped_detailed` | [571](../../src/transport.rs#L571) | [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L574) |
| `start_modern_scoped_detailed` | `self.notification_handler.clone` | [582](../../src/transport.rs#L582) | receiver-type-required |
| `start_modern_scoped_detailed` | `Arc::new` | [583](../../src/transport.rs#L583) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `observer` | [585](../../src/transport.rs#L585) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `original` | [588](../../src/transport.rs#L588) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `self.scoped_generation.clone` | [591](../../src/transport.rs#L591) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.parameter_headers.clone` | [593](../../src/transport.rs#L593) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.client.clone` | [594](../../src/transport.rs#L594) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.url.clone` | [595](../../src/transport.rs#L595) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter()                 .map(&#124;(k, v)&#124; (k.clone(), SecretText::new(v.as_str().to_owned())))                 .collect` | [596](../../src/transport.rs#L596) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter()                 .map` | [596](../../src/transport.rs#L596) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter` | [596](../../src/transport.rs#L596) | receiver-type-required |
| `start_modern_scoped_detailed` | `k.clone` | [599](../../src/transport.rs#L599) | receiver-type-required |
| `start_modern_scoped_detailed` | `SecretText::new` | [599](../../src/transport.rs#L599) | [mcp::transport::SecretText::new](../../src/transport.rs#L105) |
| `start_modern_scoped_detailed` | `v.as_str().to_owned` | [599](../../src/transport.rs#L599) | receiver-type-required |
| `start_modern_scoped_detailed` | `v.as_str` | [599](../../src/transport.rs#L599) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.authorization.clone` | [601](../../src/transport.rs#L601) | receiver-type-required |
| `start_modern_scoped_detailed` | `Vec::new` | [604](../../src/transport.rs#L604) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `Some` | [605](../../src/transport.rs#L605), [616](../../src/transport.rs#L616) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `crate::McpCancellationToken::default` | [608](../../src/transport.rs#L608) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `Box::pin` | [610](../../src/transport.rs#L610) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `serde_json::to_value(&request).map_err` | [612](../../src/transport.rs#L612) | receiver-type-required |
| `start_modern_scoped_detailed` | `serde_json::to_value` | [612](../../src/transport.rs#L612) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `McpError::Protocol` | [612](../../src/transport.rs#L612) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `e.to_string` | [612](../../src/transport.rs#L612) | receiver-type-required |
| `start_modern_scoped_detailed` | `value                 .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")                 .and_then` | [613](../../src/transport.rs#L613) | receiver-type-required |
| `start_modern_scoped_detailed` | `value                 .pointer` | [613](../../src/transport.rs#L613) | receiver-type-required |
| `start_modern_scoped_detailed` | `Err` | [618](../../src/transport.rs#L618), [623](../../src/transport.rs#L623) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `McpError::Unsupported` | [618](../../src/transport.rs#L618) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `"request-scoped HTTP requires modern protocol metadata".into` | [619](../../src/transport.rs#L619) | receiver-type-required |
| `start_modern_scoped_detailed` | `McpError::Transport` | [623](../../src/transport.rs#L623) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `"HTTP transport is closed".into` | [623](../../src/transport.rs#L623) | receiver-type-required |
| `listen_subscription` | `crate::McpSubscriptionFilter::new` | [646](../../src/transport.rs#L646) | [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) |
| `listen_subscription` | `filter.is_empty` | [647](../../src/transport.rs#L647) | receiver-type-required |
| `listen_subscription` | `Box::pin` | [648](../../src/transport.rs#L648), [660](../../src/transport.rs#L660) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `Err` | [649](../../src/transport.rs#L649), [660](../../src/transport.rs#L660) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `McpError::Unsupported` | [649](../../src/transport.rs#L649) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `"empty subscription or invalid identity".into` | [650](../../src/transport.rs#L650) | receiver-type-required |
| `listen_subscription` | `filter.request_params` | [654](../../src/transport.rs#L654) | receiver-type-required |
| `listen_subscription` | `IJsonValue::parse_str(&params.to_string())             .map_err` | [656](../../src/transport.rs#L656) | receiver-type-required |
| `listen_subscription` | `IJsonValue::parse_str` | [656](../../src/transport.rs#L656) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `listen_subscription` | `params.to_string` | [656](../../src/transport.rs#L656) | receiver-type-required |
| `listen_subscription` | `McpError::Protocol` | [657](../../src/transport.rs#L657) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `error.to_string` | [657](../../src/transport.rs#L657) | receiver-type-required |
| `listen_subscription` | `Mutex::new` | [662](../../src/transport.rs#L662) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `Arc::new` | [663](../../src/transport.rs#L663) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `filter                 .lock()                 .map(&#124;mut filter&#124; filter.accepts(notification))                 .unwrap_or` | [664](../../src/transport.rs#L664) | receiver-type-required |
| `listen_subscription` | `filter                 .lock()                 .map` | [664](../../src/transport.rs#L664) | receiver-type-required |
| `listen_subscription` | `filter                 .lock` | [664](../../src/transport.rs#L664) | receiver-type-required |
| `listen_subscription` | `filter.accepts` | [666](../../src/transport.rs#L666) | receiver-type-required |
| `listen_subscription` | `sink` | [669](../../src/transport.rs#L669) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `self.start_modern_scoped_detailed` | [672](../../src/transport.rs#L672) | [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L574) |
| `listen_subscription` | `JsonRpcRequest::call` | [673](../../src/transport.rs#L673) | [mcp::types::JsonRpcRequest::call](../../src/types.rs#L57) |
| `listen_subscription` | `Some` | [673](../../src/transport.rs#L673), [677](../../src/transport.rs#L677) | external-constructor-callback-or-unresolved |
| `with_notification_handler` | `Some` | [685](../../src/transport.rs#L685) | external-constructor-callback-or-unresolved |
| `with_notification_handler` | `Arc::new` | [685](../../src/transport.rs#L685) | external-constructor-callback-or-unresolved |
| `new` | `bearer.map` | [694](../../src/transport.rs#L694) | receiver-type-required |
| `new` | `SecretText::new` | [694](../../src/transport.rs#L694) | [mcp::transport::SecretText::new](../../src/transport.rs#L105) |
| `new` | `value.to_owned` | [694](../../src/transport.rs#L694) | receiver-type-required |
| `new` | `bearer.is_some` | [695](../../src/transport.rs#L695) | receiver-type-required |
| `new` | `"fixed-bearer".to_owned` | [696](../../src/transport.rs#L696) | receiver-type-required |
| `new` | `"anonymous".to_owned` | [698](../../src/transport.rs#L698) | receiver-type-required |
| `new` | `Self::new_with_authorization_provider` | [700](../../src/transport.rs#L700) | [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L713) |
| `new` | `Ok` | [704](../../src/transport.rs#L704) | external-constructor-callback-or-unresolved |
| `new` | `identity.clone` | [705](../../src/transport.rs#L705) | receiver-type-required |
| `new` | `fixed_bearer.as_ref().map` | [706](../../src/transport.rs#L706) | receiver-type-required |
| `new` | `fixed_bearer.as_ref` | [706](../../src/transport.rs#L706) | receiver-type-required |
| `new` | `value.as_str().to_owned` | [706](../../src/transport.rs#L706) | receiver-type-required |
| `new` | `value.as_str` | [706](../../src/transport.rs#L706) | receiver-type-required |
| `new_with_authorization_provider` | `Self::new_with_request_authorization_provider` | [722](../../src/transport.rs#L722) | [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L737) |
| `new_with_authorization_provider` | `authorization` | [726](../../src/transport.rs#L726) | external-constructor-callback-or-unresolved |
| `new_with_authorization_provider` | `Ok` | [727](../../src/transport.rs#L727) | external-constructor-callback-or-unresolved |
| `new_with_authorization_provider` | `BTreeMap::new` | [729](../../src/transport.rs#L729) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `url             .host_str()             .is_some_and` | [746](../../src/transport.rs#L746) | receiver-type-required |
| `new_with_request_authorization_provider` | `url             .host_str` | [746](../../src/transport.rs#L746) | receiver-type-required |
| `new_with_request_authorization_provider` | `url.scheme` | [749](../../src/transport.rs#L749) | receiver-type-required |
| `new_with_request_authorization_provider` | `Err` | [750](../../src/transport.rs#L750), [767](../../src/transport.rs#L767) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `McpError::Transport` | [750](../../src/transport.rs#L750), [757](../../src/transport.rs#L757), [767](../../src/transport.rs#L767), [771](../../src/transport.rs#L771), [777](../../src/transport.rs#L777) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `"HTTP MCP requires HTTPS outside an explicit loopback test".to_owned` | [751](../../src/transport.rs#L751) | receiver-type-required |
| `new_with_request_authorization_provider` | `Vec::new` | [754](../../src/transport.rs#L754), [788](../../src/transport.rs#L788) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `HeaderName::from_bytes(name.as_bytes())                 .map_err` | [756](../../src/transport.rs#L756) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderName::from_bytes` | [756](../../src/transport.rs#L756) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `name.as_bytes` | [756](../../src/transport.rs#L756) | receiver-type-required |
| `new_with_request_authorization_provider` | `error.to_string` | [757](../../src/transport.rs#L757), [771](../../src/transport.rs#L771), [777](../../src/transport.rs#L777) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str().eq_ignore_ascii_case` | [761](../../src/transport.rs#L761), [762](../../src/transport.rs#L762), [763](../../src/transport.rs#L763), [764](../../src/transport.rs#L764) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str` | [761](../../src/transport.rs#L761), [762](../../src/transport.rs#L762), [763](../../src/transport.rs#L763), [764](../../src/transport.rs#L764), [765](../../src/transport.rs#L765) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str().starts_with` | [765](../../src/transport.rs#L765) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderValue::from_str(value).map_err` | [771](../../src/transport.rs#L771) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderValue::from_str` | [771](../../src/transport.rs#L771) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `configured.push` | [772](../../src/transport.rs#L772) | receiver-type-required |
| `new_with_request_authorization_provider` | `SecretText::new` | [772](../../src/transport.rs#L772) | [mcp::transport::SecretText::new](../../src/transport.rs#L105) |
| `new_with_request_authorization_provider` | `value.clone` | [772](../../src/transport.rs#L772) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect(reqwest::redirect::Policy::none())             .build()             .map_err` | [774](../../src/transport.rs#L774) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect(reqwest::redirect::Policy::none())             .build` | [774](../../src/transport.rs#L774) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect` | [774](../../src/transport.rs#L774) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder` | [774](../../src/transport.rs#L774) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `reqwest::redirect::Policy::none` | [775](../../src/transport.rs#L775) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `Ok` | [778](../../src/transport.rs#L778) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `BTreeMap::new` | [779](../../src/transport.rs#L779) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `Arc::new` | [783](../../src/transport.rs#L783) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `crate::McpCancellationToken::default` | [790](../../src/transport.rs#L790) | external-constructor-callback-or-unresolved |
| `request_headers` | `Err` | [796](../../src/transport.rs#L796), [800](../../src/transport.rs#L800), [837](../../src/transport.rs#L837) | external-constructor-callback-or-unresolved |
| `request_headers` | `McpError::Transport` | [796](../../src/transport.rs#L796), [800](../../src/transport.rs#L800), [822](../../src/transport.rs#L822), [827](../../src/transport.rs#L827), [837](../../src/transport.rs#L837), [844](../../src/transport.rs#L844), [849](../../src/transport.rs#L849) | external-constructor-callback-or-unresolved |
| `request_headers` | `"HTTP MCP peer is closed".to_owned` | [796](../../src/transport.rs#L796) | receiver-type-required |
| `request_headers` | `(self.authorization)` | [798](../../src/transport.rs#L798) | external-constructor-callback-or-unresolved |
| `request_headers` | `authorization.identity.is_empty` | [799](../../src/transport.rs#L799) | receiver-type-required |
| `request_headers` | `"HTTP authorization identity is empty".to_owned` | [801](../../src/transport.rs#L801) | receiver-type-required |
| `request_headers` | `self             .session_authorization_identity             .as_ref()             .is_some_and` | [804](../../src/transport.rs#L804) | receiver-type-required |
| `request_headers` | `self             .session_authorization_identity             .as_ref` | [804](../../src/transport.rs#L804) | receiver-type-required |
| `request_headers` | `Some` | [811](../../src/transport.rs#L811), [875](../../src/transport.rs#L875), [876](../../src/transport.rs#L876), [877](../../src/transport.rs#L877) | external-constructor-callback-or-unresolved |
| `request_headers` | `authorization.identity.clone` | [811](../../src/transport.rs#L811) | receiver-type-required |
| `request_headers` | `HeaderMap::new` | [812](../../src/transport.rs#L812) | external-constructor-callback-or-unresolved |
| `request_headers` | `map.insert` | [813](../../src/transport.rs#L813), [814](../../src/transport.rs#L814), [819](../../src/transport.rs#L819), [841](../../src/transport.rs#L841), [850](../../src/transport.rs#L850), [853](../../src/transport.rs#L853), [864](../../src/transport.rs#L864), [869](../../src/transport.rs#L869), [893](../../src/transport.rs#L893), [907](../../src/transport.rs#L907) | receiver-type-required |
| `request_headers` | `HeaderValue::from_static` | [813](../../src/transport.rs#L813), [816](../../src/transport.rs#L816) | external-constructor-callback-or-unresolved |
| `request_headers` | `name.clone` | [820](../../src/transport.rs#L820) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(value.as_str())                     .map_err` | [821](../../src/transport.rs#L821) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str` | [821](../../src/transport.rs#L821), [843](../../src/transport.rs#L843), [848](../../src/transport.rs#L848), [866](../../src/transport.rs#L866), [871](../../src/transport.rs#L871), [895](../../src/transport.rs#L895), [910](../../src/transport.rs#L910) | external-constructor-callback-or-unresolved |
| `request_headers` | `value.as_str` | [821](../../src/transport.rs#L821) | receiver-type-required |
| `request_headers` | `error.to_string` | [822](../../src/transport.rs#L822), [827](../../src/transport.rs#L827), [844](../../src/transport.rs#L844), [849](../../src/transport.rs#L849), [859](../../src/transport.rs#L859), [867](../../src/transport.rs#L867), [872](../../src/transport.rs#L872), [896](../../src/transport.rs#L896) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes(name.as_bytes())                 .map_err` | [826](../../src/transport.rs#L826) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes` | [826](../../src/transport.rs#L826), [908](../../src/transport.rs#L908) | external-constructor-callback-or-unresolved |
| `request_headers` | `name.as_bytes` | [826](../../src/transport.rs#L826), [908](../../src/transport.rs#L908) | receiver-type-required |
| `request_headers` | `name.as_str().eq_ignore_ascii_case` | [831](../../src/transport.rs#L831), [832](../../src/transport.rs#L832), [833](../../src/transport.rs#L833), [834](../../src/transport.rs#L834) | receiver-type-required |
| `request_headers` | `name.as_str` | [831](../../src/transport.rs#L831), [832](../../src/transport.rs#L832), [833](../../src/transport.rs#L833), [834](../../src/transport.rs#L834), [835](../../src/transport.rs#L835) | receiver-type-required |
| `request_headers` | `name.as_str().starts_with` | [835](../../src/transport.rs#L835) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(value)                     .map_err` | [843](../../src/transport.rs#L843) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&format!("Bearer {bearer}"))                 .map_err` | [848](../../src/transport.rs#L848) | receiver-type-required |
| `request_headers` | `HeaderName::from_static` | [854](../../src/transport.rs#L854), [865](../../src/transport.rs#L865), [870](../../src/transport.rs#L870), [894](../../src/transport.rs#L894) | external-constructor-callback-or-unresolved |
| `request_headers` | `session_id.clone` | [855](../../src/transport.rs#L855) | receiver-type-required |
| `request_headers` | `serde_json::to_value(request).map_err` | [859](../../src/transport.rs#L859) | receiver-type-required |
| `request_headers` | `serde_json::to_value` | [859](../../src/transport.rs#L859) | external-constructor-callback-or-unresolved |
| `request_headers` | `McpError::Protocol` | [859](../../src/transport.rs#L859), [867](../../src/transport.rs#L867), [872](../../src/transport.rs#L872), [896](../../src/transport.rs#L896), [909](../../src/transport.rs#L909), [910](../../src/transport.rs#L910) | external-constructor-callback-or-unresolved |
| `request_headers` | `request_value             .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")             .and_then` | [860](../../src/transport.rs#L860) | receiver-type-required |
| `request_headers` | `request_value             .pointer` | [860](../../src/transport.rs#L860) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(version)                     .map_err` | [866](../../src/transport.rs#L866) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&request.method)                     .map_err` | [871](../../src/transport.rs#L871) | receiver-type-required |
| `request_headers` | `request.method.as_str` | [874](../../src/transport.rs#L874) | receiver-type-required |
| `request_headers` | `key.and_then` | [880](../../src/transport.rs#L880) | receiver-type-required |
| `request_headers` | `request_value["params"][key].as_str` | [880](../../src/transport.rs#L880) | receiver-type-required |
| `request_headers` | `name.is_empty` | [881](../../src/transport.rs#L881) | receiver-type-required |
| `request_headers` | `name.bytes().all` | [882](../../src/transport.rs#L882) | receiver-type-required |
| `request_headers` | `name.bytes` | [882](../../src/transport.rs#L882) | receiver-type-required |
| `request_headers` | `(0x20..=0x7e).contains` | [882](../../src/transport.rs#L882) | receiver-type-required |
| `request_headers` | `name.trim` | [883](../../src/transport.rs#L883) | receiver-type-required |
| `request_headers` | `name.starts_with` | [884](../../src/transport.rs#L884) | receiver-type-required |
| `request_headers` | `name.ends_with` | [884](../../src/transport.rs#L884) | receiver-type-required |
| `request_headers` | `name.to_owned` | [886](../../src/transport.rs#L886) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&encoded)                         .map_err` | [895](../../src/transport.rs#L895) | receiver-type-required |
| `request_headers` | `map.contains_key` | [901](../../src/transport.rs#L901) | receiver-type-required |
| `request_headers` | `request_value["params"]["name"]                 .as_str()                 .and_then` | [902](../../src/transport.rs#L902) | receiver-type-required |
| `request_headers` | `request_value["params"]["name"]                 .as_str` | [902](../../src/transport.rs#L902) | receiver-type-required |
| `request_headers` | `self.parameter_headers.get` | [904](../../src/transport.rs#L904) | receiver-type-required |
| `request_headers` | `projection.project` | [906](../../src/transport.rs#L906) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes(name.as_bytes())                         .map_err` | [908](../../src/transport.rs#L908) | receiver-type-required |
| `request_headers` | `e.to_string` | [909](../../src/transport.rs#L909), [910](../../src/transport.rs#L910) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&value).map_err` | [910](../../src/transport.rs#L910) | receiver-type-required |
| `request_headers` | `Ok` | [914](../../src/transport.rs#L914) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `crate::McpSubscriptionFilter::new(id, capabilities, &[]).is_empty` | [925](../../src/transport.rs#L925) | receiver-type-required |
| `start_catalog_subscription` | `crate::McpSubscriptionFilter::new` | [925](../../src/transport.rs#L925) | [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) |
| `start_catalog_subscription` | `Some` | [928](../../src/transport.rs#L928) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `self.listen_subscription` | [928](../../src/transport.rs#L928) | receiver-type-required |
| `start_catalog_subscription` | `Duration::from_secs` | [932](../../src/transport.rs#L932) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `crate::McpCancellationToken::default` | [933](../../src/transport.rs#L933) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `crate::parameter_headers::ParameterHeaders::catalog` | [939](../../src/transport.rs#L939) | [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) |
| `install_tool_catalog` | `Ok` | [940](../../src/transport.rs#L940) | external-constructor-callback-or-unresolved |
| `start_scoped_request` | `Some` | [954](../../src/transport.rs#L954) | external-constructor-callback-or-unresolved |
| `start_scoped_request` | `self.start_modern_scoped` | [954](../../src/transport.rs#L954) | receiver-type-required |
| `request` | `Box::pin` | [962](../../src/transport.rs#L962) | external-constructor-callback-or-unresolved |
| `request` | `request                 .id                 .ok_or_else` | [963](../../src/transport.rs#L963) | receiver-type-required |
| `request` | `McpError::Protocol` | [965](../../src/transport.rs#L965), [1050](../../src/transport.rs#L1050), [1069](../../src/transport.rs#L1069), [1087](../../src/transport.rs#L1087), [1097](../../src/transport.rs#L1097), [1103](../../src/transport.rs#L1103) | external-constructor-callback-or-unresolved |
| `request` | `"request id is absent".to_owned` | [965](../../src/transport.rs#L965) | receiver-type-required |
| `request` | `self.request_headers` | [966](../../src/transport.rs#L966) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json(request)                 .send()                 .await                 .map_err` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json(request)                 .send` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self                 .client                 .post` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `self.url.clone` | [969](../../src/transport.rs#L969) | receiver-type-required |
| `request` | `error.is_timeout` | [976](../../src/transport.rs#L976) | receiver-type-required |
| `request` | `McpError::Timeout` | [977](../../src/transport.rs#L977) | external-constructor-callback-or-unresolved |
| `request` | `request.method.clone` | [977](../../src/transport.rs#L977) | receiver-type-required |
| `request` | `McpError::Transport` | [979](../../src/transport.rs#L979), [983](../../src/transport.rs#L983), [1035](../../src/transport.rs#L1035), [1058](../../src/transport.rs#L1058), [1095](../../src/transport.rs#L1095) | external-constructor-callback-or-unresolved |
| `request` | `error.to_string` | [979](../../src/transport.rs#L979), [1058](../../src/transport.rs#L1058), [1069](../../src/transport.rs#L1069), [1095](../../src/transport.rs#L1095) | receiver-type-required |
| `request` | `response.status().is_redirection` | [982](../../src/transport.rs#L982) | receiver-type-required |
| `request` | `response.status` | [982](../../src/transport.rs#L982), [987](../../src/transport.rs#L987), [988](../../src/transport.rs#L988) | receiver-type-required |
| `request` | `Err` | [983](../../src/transport.rs#L983), [1027](../../src/transport.rs#L1027), [1035](../../src/transport.rs#L1035), [1050](../../src/transport.rs#L1050), [1097](../../src/transport.rs#L1097), [1103](../../src/transport.rs#L1103) | external-constructor-callback-or-unresolved |
| `request` | `"cross-origin redirect rejected".to_owned` | [984](../../src/transport.rs#L984) | receiver-type-required |
| `request` | `response.status().is_success` | [987](../../src/transport.rs#L987) | receiver-type-required |
| `request` | `status.is_client_error` | [991](../../src/transport.rs#L991) | receiver-type-required |
| `request` | `status.as_u16` | [991](../../src/transport.rs#L991) | receiver-type-required |
| `request` | `response.bytes_stream` | [992](../../src/transport.rs#L992), [1054](../../src/transport.rs#L1054), [1092](../../src/transport.rs#L1092) | receiver-type-required |
| `request` | `Vec::new` | [993](../../src/transport.rs#L993), [1093](../../src/transport.rs#L1093) | external-constructor-callback-or-unresolved |
| `request` | `stream.next` | [994](../../src/transport.rs#L994), [1057](../../src/transport.rs#L1057), [1094](../../src/transport.rs#L1094) | receiver-type-required |
| `request` | `bytes.clear` | [996](../../src/transport.rs#L996), [1000](../../src/transport.rs#L1000) | receiver-type-required |
| `request` | `bytes.len` | [999](../../src/transport.rs#L999), [1096](../../src/transport.rs#L1096) | receiver-type-required |
| `request` | `chunk.len` | [999](../../src/transport.rs#L999), [1096](../../src/transport.rs#L1096) | receiver-type-required |
| `request` | `bytes.extend_from_slice` | [1003](../../src/transport.rs#L1003), [1099](../../src/transport.rs#L1099) | receiver-type-required |
| `request` | `serde_json::from_slice::<JsonRpcResponse>` | [1005](../../src/transport.rs#L1005) | external-constructor-callback-or-unresolved |
| `request` | `rpc.error.is_some` | [1006](../../src/transport.rs#L1006) | receiver-type-required |
| `request` | `request.id.is_some_and` | [1007](../../src/transport.rs#L1007) | receiver-type-required |
| `request` | `rpc.validate_for(id).is_ok` | [1007](../../src/transport.rs#L1007) | receiver-type-required |
| `request` | `rpc.validate_for` | [1007](../../src/transport.rs#L1007) | receiver-type-required |
| `request` | `Ok` | [1009](../../src/transport.rs#L1009) | external-constructor-callback-or-unresolved |
| `request` | `serde_json::from_slice::<serde_json::Value>` | [1016](../../src/transport.rs#L1016) | external-constructor-callback-or-unresolved |
| `request` | `error["message"].as_str().unwrap_or_default` | [1018](../../src/transport.rs#L1018) | receiver-type-required |
| `request` | `error["message"].as_str` | [1018](../../src/transport.rs#L1018) | receiver-type-required |
| `request` | `error["code"].as_i64` | [1019](../../src/transport.rs#L1019) | receiver-type-required |
| `request` | `Some` | [1019](../../src/transport.rs#L1019), [1038](../../src/transport.rs#L1038), [1062](../../src/transport.rs#L1062) | external-constructor-callback-or-unresolved |
| `request` | `message.to_ascii_lowercase().contains` | [1020](../../src/transport.rs#L1020) | receiver-type-required |
| `request` | `message.to_ascii_lowercase` | [1020](../../src/transport.rs#L1020) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter(&#124;data&#124; !data.is_null())                                 .and_then(&#124;data&#124; serde_json::to_vec(data).ok())                                 .and_then` | [1022](../../src/transport.rs#L1022) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter(&#124;data&#124; !data.is_null())                                 .and_then` | [1022](../../src/transport.rs#L1022) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter` | [1022](../../src/transport.rs#L1022) | receiver-type-required |
| `request` | `error                                 .get` | [1022](../../src/transport.rs#L1022) | receiver-type-required |
| `request` | `data.is_null` | [1024](../../src/transport.rs#L1024) | receiver-type-required |
| `request` | `serde_json::to_vec(data).ok` | [1025](../../src/transport.rs#L1025) | receiver-type-required |
| `request` | `serde_json::to_vec` | [1025](../../src/transport.rs#L1025) | external-constructor-callback-or-unresolved |
| `request` | `IJsonValue::parse(&bytes).ok` | [1026](../../src/transport.rs#L1026) | receiver-type-required |
| `request` | `IJsonValue::parse` | [1026](../../src/transport.rs#L1026) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `request` | `message.to_owned` | [1029](../../src/transport.rs#L1029) | receiver-type-required |
| `request` | `response.headers().get` | [1037](../../src/transport.rs#L1037) | receiver-type-required |
| `request` | `response.headers` | [1037](../../src/transport.rs#L1037) | receiver-type-required |
| `request` | `session_id.clone` | [1038](../../src/transport.rs#L1038) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then(&#124;value&#124; value.to_str().ok())                 .unwrap_or("")                 .to_owned` | [1040](../../src/transport.rs#L1040) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then(&#124;value&#124; value.to_str().ok())                 .unwrap_or` | [1040](../../src/transport.rs#L1040) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then` | [1040](../../src/transport.rs#L1040) | receiver-type-required |
| `request` | `response                 .headers()                 .get` | [1040](../../src/transport.rs#L1040) | receiver-type-required |
| `request` | `response                 .headers` | [1040](../../src/transport.rs#L1040) | receiver-type-required |
| `request` | `value.to_str().ok` | [1043](../../src/transport.rs#L1043) | receiver-type-required |
| `request` | `value.to_str` | [1043](../../src/transport.rs#L1043) | receiver-type-required |
| `request` | `response                 .content_length()                 .is_some_and` | [1046](../../src/transport.rs#L1046) | receiver-type-required |
| `request` | `response                 .content_length` | [1046](../../src/transport.rs#L1046) | receiver-type-required |
| `request` | `"HTTP result exceeds 4 MiB".to_owned` | [1050](../../src/transport.rs#L1050), [1097](../../src/transport.rs#L1097) | receiver-type-required |
| `request` | `content_type.split(';').next().map(str::trim).unwrap_or` | [1052](../../src/transport.rs#L1052) | receiver-type-required |
| `request` | `content_type.split(';').next().map` | [1052](../../src/transport.rs#L1052) | receiver-type-required |
| `request` | `content_type.split(';').next` | [1052](../../src/transport.rs#L1052) | receiver-type-required |
| `request` | `content_type.split` | [1052](../../src/transport.rs#L1052) | receiver-type-required |
| `request` | `SseDecoder::default` | [1055](../../src/transport.rs#L1055) | external-constructor-callback-or-unresolved |
| `request` | `chunk.map_err` | [1058](../../src/transport.rs#L1058), [1095](../../src/transport.rs#L1095) | receiver-type-required |
| `request` | `decoder.push` | [1059](../../src/transport.rs#L1059) | receiver-type-required |
| `request` | `decode_inbound` | [1060](../../src/transport.rs#L1060) | [mcp::transport::decode_inbound](../../src/transport.rs#L1277) |
| `request` | `BTreeSet::new` | [1060](../../src/transport.rs#L1060) | external-constructor-callback-or-unresolved |
| `request` | `serde_json::from_slice(&payload).map_err` | [1068](../../src/transport.rs#L1068) | receiver-type-required |
| `request` | `serde_json::from_slice` | [1068](../../src/transport.rs#L1068) | external-constructor-callback-or-unresolved |
| `request` | `handler` | [1071](../../src/transport.rs#L1071), [1074](../../src/transport.rs#L1074) | external-constructor-callback-or-unresolved |
| `request` | `self.notifications.push` | [1076](../../src/transport.rs#L1076) | receiver-type-required |
| `request` | `result.is_some` | [1082](../../src/transport.rs#L1082) | receiver-type-required |
| `request` | `result.ok_or_else` | [1086](../../src/transport.rs#L1086) | receiver-type-required |
| `request` | `"SSE contains no complete matching response event".to_owned` | [1088](../../src/transport.rs#L1088) | receiver-type-required |
| `request` | `bytes.len().saturating_add` | [1096](../../src/transport.rs#L1096) | receiver-type-required |
| `request` | `decode_response` | [1101](../../src/transport.rs#L1101) | [mcp::transport::decode_response](../../src/transport.rs#L1268) |
| `request` | `self.scoped_generation.cancel` | [1108](../../src/transport.rs#L1108) | receiver-type-required |
| `notify` | `Box::pin` | [1117](../../src/transport.rs#L1117) | external-constructor-callback-or-unresolved |
| `notify` | `notification.id.is_some` | [1118](../../src/transport.rs#L1118) | receiver-type-required |
| `notify` | `Err` | [1119](../../src/transport.rs#L1119), [1137](../../src/transport.rs#L1137) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Protocol` | [1119](../../src/transport.rs#L1119) | external-constructor-callback-or-unresolved |
| `notify` | `"notification has an id".to_owned` | [1119](../../src/transport.rs#L1119) | receiver-type-required |
| `notify` | `self.request_headers` | [1121](../../src/transport.rs#L1121) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json(notification)                 .send()                 .await                 .map_err` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json(notification)                 .send` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self                 .client                 .post` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `self.url.clone` | [1124](../../src/transport.rs#L1124) | receiver-type-required |
| `notify` | `Duration::from_secs` | [1126](../../src/transport.rs#L1126) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Transport` | [1130](../../src/transport.rs#L1130), [1137](../../src/transport.rs#L1137) | external-constructor-callback-or-unresolved |
| `notify` | `error.to_string` | [1130](../../src/transport.rs#L1130) | receiver-type-required |
| `notify` | `response.headers().get` | [1131](../../src/transport.rs#L1131) | receiver-type-required |
| `notify` | `response.headers` | [1131](../../src/transport.rs#L1131) | receiver-type-required |
| `notify` | `Some` | [1132](../../src/transport.rs#L1132) | external-constructor-callback-or-unresolved |
| `notify` | `session_id.clone` | [1132](../../src/transport.rs#L1132) | receiver-type-required |
| `notify` | `response.status().is_success` | [1134](../../src/transport.rs#L1134) | receiver-type-required |
| `notify` | `response.status` | [1134](../../src/transport.rs#L1134) | receiver-type-required |
| `notify` | `Ok` | [1135](../../src/transport.rs#L1135) | external-constructor-callback-or-unresolved |
| `close` | `self.session_id.is_some` | [1148](../../src/transport.rs#L1148) | receiver-type-required |
| `close` | `self.request_headers(&JsonRpcRequest::notification("", None))                 .ok()                 .filter` | [1149](../../src/transport.rs#L1149) | receiver-type-required |
| `close` | `self.request_headers(&JsonRpcRequest::notification("", None))                 .ok` | [1149](../../src/transport.rs#L1149) | receiver-type-required |
| `close` | `self.request_headers` | [1149](../../src/transport.rs#L1149) | receiver-type-required |
| `close` | `JsonRpcRequest::notification` | [1149](../../src/transport.rs#L1149) | [mcp::types::JsonRpcRequest::notification](../../src/types.rs#L66) |
| `close` | `headers.contains_key` | [1151](../../src/transport.rs#L1151) | receiver-type-required |
| `close` | `self.scoped_generation.cancel` | [1155](../../src/transport.rs#L1155) | receiver-type-required |
| `close` | `Box::pin` | [1159](../../src/transport.rs#L1159) | external-constructor-callback-or-unresolved |
| `close` | `headers.insert` | [1161](../../src/transport.rs#L1161) | receiver-type-required |
| `close` | `HeaderName::from_static` | [1162](../../src/transport.rs#L1162) | external-constructor-callback-or-unresolved |
| `close` | `HeaderValue::from_static` | [1163](../../src/transport.rs#L1163) | external-constructor-callback-or-unresolved |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers(headers)                     .timeout(Duration::from_secs(15))                     .send` | [1167](../../src/transport.rs#L1167) | receiver-type-required |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers(headers)                     .timeout` | [1167](../../src/transport.rs#L1167) | receiver-type-required |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers` | [1167](../../src/transport.rs#L1167) | receiver-type-required |
| `close` | `self                     .client                     .delete` | [1167](../../src/transport.rs#L1167) | receiver-type-required |
| `close` | `self.url.clone` | [1169](../../src/transport.rs#L1169) | receiver-type-required |
| `close` | `Duration::from_secs` | [1171](../../src/transport.rs#L1171) | external-constructor-callback-or-unresolved |
| `close` | `Ok` | [1175](../../src/transport.rs#L1175) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.scoped_generation.cancel` | [1180](../../src/transport.rs#L1180) | receiver-type-required |
| `reconnect` | `crate::McpCancellationToken::default` | [1181](../../src/transport.rs#L1181) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.notifications.clear` | [1185](../../src/transport.rs#L1185) | receiver-type-required |
| `reconnect` | `Box::pin` | [1186](../../src/transport.rs#L1186) | external-constructor-callback-or-unresolved |
| `reconnect` | `Ok` | [1186](../../src/transport.rs#L1186) | external-constructor-callback-or-unresolved |
| `take_notifications` | `std::mem::take` | [1190](../../src/transport.rs#L1190) | external-constructor-callback-or-unresolved |
| `push` | `self.pending.len().saturating_add` | [1203](../../src/transport.rs#L1203) | receiver-type-required |
| `push` | `self.pending.len` | [1203](../../src/transport.rs#L1203) | receiver-type-required |
| `push` | `chunk.len` | [1203](../../src/transport.rs#L1203) | receiver-type-required |
| `push` | `Err` | [1204](../../src/transport.rs#L1204), [1219](../../src/transport.rs#L1219), [1237](../../src/transport.rs#L1237) | external-constructor-callback-or-unresolved |
| `push` | `McpError::Protocol` | [1204](../../src/transport.rs#L1204), [1219](../../src/transport.rs#L1219), [1237](../../src/transport.rs#L1237) | external-constructor-callback-or-unresolved |
| `push` | `"SSE event exceeds 4 MiB".to_owned` | [1204](../../src/transport.rs#L1204), [1219](../../src/transport.rs#L1219), [1237](../../src/transport.rs#L1237) | receiver-type-required |
| `push` | `self.pending.extend_from_slice` | [1206](../../src/transport.rs#L1206) | receiver-type-required |
| `push` | `Vec::new` | [1207](../../src/transport.rs#L1207) | external-constructor-callback-or-unresolved |
| `push` | `self.pending.iter().position` | [1208](../../src/transport.rs#L1208) | receiver-type-required |
| `push` | `self.pending.iter` | [1208](../../src/transport.rs#L1208) | receiver-type-required |
| `push` | `self.pending.drain(..=newline).collect::<Vec<_>>` | [1209](../../src/transport.rs#L1209) | receiver-type-required |
| `push` | `self.pending.drain` | [1209](../../src/transport.rs#L1209) | receiver-type-required |
| `push` | `line.pop` | [1210](../../src/transport.rs#L1210), [1212](../../src/transport.rs#L1212) | receiver-type-required |
| `push` | `line.last` | [1211](../../src/transport.rs#L1211) | receiver-type-required |
| `push` | `Some` | [1211](../../src/transport.rs#L1211) | external-constructor-callback-or-unresolved |
| `push` | `line.is_empty` | [1214](../../src/transport.rs#L1214) | receiver-type-required |
| `push` | `self.data.is_empty` | [1215](../../src/transport.rs#L1215) | receiver-type-required |
| `push` | `self.data.iter().map(Vec::len).sum::<usize>` | [1216](../../src/transport.rs#L1216) | receiver-type-required |
| `push` | `self.data.iter().map` | [1216](../../src/transport.rs#L1216) | receiver-type-required |
| `push` | `self.data.iter` | [1216](../../src/transport.rs#L1216) | receiver-type-required |
| `push` | `self.data.len().saturating_sub` | [1217](../../src/transport.rs#L1217) | receiver-type-required |
| `push` | `self.data.len` | [1217](../../src/transport.rs#L1217) | receiver-type-required |
| `push` | `Vec::with_capacity` | [1221](../../src/transport.rs#L1221) | external-constructor-callback-or-unresolved |
| `push` | `self.data.drain(..).enumerate` | [1222](../../src/transport.rs#L1222) | receiver-type-required |
| `push` | `self.data.drain` | [1222](../../src/transport.rs#L1222) | receiver-type-required |
| `push` | `payload.push` | [1224](../../src/transport.rs#L1224) | receiver-type-required |
| `push` | `payload.extend_from_slice` | [1226](../../src/transport.rs#L1226) | receiver-type-required |
| `push` | `events.push` | [1229](../../src/transport.rs#L1229) | receiver-type-required |
| `push` | `line.strip_prefix` | [1233](../../src/transport.rs#L1233) | receiver-type-required |
| `push` | `value.strip_prefix(b" ").unwrap_or(value).to_vec` | [1234](../../src/transport.rs#L1234) | receiver-type-required |
| `push` | `value.strip_prefix(b" ").unwrap_or` | [1234](../../src/transport.rs#L1234) | receiver-type-required |
| `push` | `value.strip_prefix` | [1234](../../src/transport.rs#L1234) | receiver-type-required |
| `push` | `self.event_bytes.saturating_add` | [1235](../../src/transport.rs#L1235) | receiver-type-required |
| `push` | `value.len` | [1235](../../src/transport.rs#L1235) | receiver-type-required |
| `push` | `self.data.push` | [1239](../../src/transport.rs#L1239) | receiver-type-required |
| `push` | `Ok` | [1242](../../src/transport.rs#L1242) | external-constructor-callback-or-unresolved |
| `decode_response` | `decode_inbound` | [1269](../../src/transport.rs#L1269) | [mcp::transport::decode_inbound](../../src/transport.rs#L1277) |
| `decode_response` | `BTreeSet::new` | [1269](../../src/transport.rs#L1269) | external-constructor-callback-or-unresolved |
| `decode_response` | `Ok` | [1270](../../src/transport.rs#L1270) | external-constructor-callback-or-unresolved |
| `decode_response` | `Err` | [1271](../../src/transport.rs#L1271) | external-constructor-callback-or-unresolved |
| `decode_response` | `McpError::Protocol` | [1271](../../src/transport.rs#L1271) | external-constructor-callback-or-unresolved |
| `decode_response` | `"HTTP JSON response did not contain a matching response".to_owned` | [1272](../../src/transport.rs#L1272) | receiver-type-required |
| `decode_inbound` | `serde_json::from_slice(bytes).map_err` | [1283](../../src/transport.rs#L1283) | receiver-type-required |
| `decode_inbound` | `serde_json::from_slice` | [1283](../../src/transport.rs#L1283) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `McpError::Protocol` | [1283](../../src/transport.rs#L1283), [1285](../../src/transport.rs#L1285), [1291](../../src/transport.rs#L1291), [1296](../../src/transport.rs#L1296), [1304](../../src/transport.rs#L1304), [1309](../../src/transport.rs#L1309), [1323](../../src/transport.rs#L1323), [1329](../../src/transport.rs#L1329), [1343](../../src/transport.rs#L1343) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `error.to_string` | [1283](../../src/transport.rs#L1283) | receiver-type-required |
| `decode_inbound` | `Err` | [1285](../../src/transport.rs#L1285), [1291](../../src/transport.rs#L1291), [1296](../../src/transport.rs#L1296), [1304](../../src/transport.rs#L1304), [1309](../../src/transport.rs#L1309), [1323](../../src/transport.rs#L1323), [1343](../../src/transport.rs#L1343) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `"JSON-RPC frame version is not 2.0".to_owned` | [1286](../../src/transport.rs#L1286) | receiver-type-required |
| `decode_inbound` | `"response id is not a positive integer".to_owned` | [1292](../../src/transport.rs#L1292) | receiver-type-required |
| `decode_inbound` | `frame.method.is_some` | [1295](../../src/transport.rs#L1295) | receiver-type-required |
| `decode_inbound` | `"server request has no installed client handler".to_owned` | [1297](../../src/transport.rs#L1297) | receiver-type-required |
| `decode_inbound` | `cancelled_ids.remove` | [1301](../../src/transport.rs#L1301) | receiver-type-required |
| `decode_inbound` | `Ok` | [1302](../../src/transport.rs#L1302), [1320](../../src/transport.rs#L1320), [1341](../../src/transport.rs#L1341) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `frame.params.is_some` | [1308](../../src/transport.rs#L1308) | receiver-type-required |
| `decode_inbound` | `"response contains notification params".to_owned` | [1310](../../src/transport.rs#L1310) | receiver-type-required |
| `decode_inbound` | `response.validate_for` | [1319](../../src/transport.rs#L1319) | receiver-type-required |
| `decode_inbound` | `Inbound::Response` | [1320](../../src/transport.rs#L1320) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `frame.result.is_some` | [1322](../../src/transport.rs#L1322) | receiver-type-required |
| `decode_inbound` | `frame.error.is_some` | [1322](../../src/transport.rs#L1322) | receiver-type-required |
| `decode_inbound` | `"id-less frame is not a notification".to_owned` | [1324](../../src/transport.rs#L1324), [1329](../../src/transport.rs#L1329) | receiver-type-required |
| `decode_inbound` | `frame         .method         .ok_or_else` | [1327](../../src/transport.rs#L1327) | receiver-type-required |
| `decode_inbound` | `Inbound::Notification` | [1341](../../src/transport.rs#L1341) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap` | [1356](../../src/transport.rs#L1356) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::net::TcpListener::bind` | [1356](../../src/transport.rs#L1356) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.local_addr().unwrap` | [1357](../../src/transport.rs#L1357) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.local_addr` | [1357](../../src/transport.rs#L1357) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Arc::new` | [1358](../../src/transport.rs#L1358) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::sync::Notify::new` | [1358](../../src/transport.rs#L1358) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `received.clone` | [1359](../../src/transport.rs#L1359) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::spawn` | [1360](../../src/transport.rs#L1360) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.accept().await.unwrap` | [1361](../../src/transport.rs#L1361) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.accept` | [1361](../../src/transport.rs#L1361) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Vec::new` | [1362](../../src/transport.rs#L1362) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `socket.read(&mut chunk).await.unwrap` | [1365](../../src/transport.rs#L1365) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `socket.read` | [1365](../../src/transport.rs#L1365) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.extend_from_slice` | [1367](../../src/transport.rs#L1367) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.windows(4).position` | [1368](../../src/transport.rs#L1368) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.windows` | [1368](../../src/transport.rs#L1368) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `String::from_utf8_lossy(&bytes[..offset]).to_ascii_lowercase` | [1369](../../src/transport.rs#L1369) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `String::from_utf8_lossy` | [1369](../../src/transport.rs#L1369) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim()                             .parse()                             .unwrap` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim()                             .parse` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines` | [1370](../../src/transport.rs#L1370) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `line.strip_prefix` | [1372](../../src/transport.rs#L1372) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.len` | [1377](../../src/transport.rs#L1377) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `signal.notify_one` | [1382](../../src/transport.rs#L1382) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `HttpTransport::new(                 format!("http://{address}/mcp").parse().unwrap(),                 &BTreeMap::new(),                 None,                 true,             )             .unwrap` | [1399](../../src/transport.rs#L1399) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `HttpTransport::new` | [1399](../../src/transport.rs#L1399) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `format!("http://{address}/mcp").parse().unwrap` | [1400](../../src/transport.rs#L1400) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `format!("http://{address}/mcp").parse` | [1400](../../src/transport.rs#L1400) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `BTreeMap::new` | [1401](../../src/transport.rs#L1401) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `serde_json::from_value(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"write","arguments":{},"_meta":{"io.modelcontextprotocol/protocolVersion":crate::MODERN_PROTOCOL_VERSION}}})).unwrap` | [1406](../../src/transport.rs#L1406) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `serde_json::from_value` | [1406](../../src/transport.rs#L1406) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `transport.start_modern_scoped` | [1407](../../src/transport.rs#L1407), [1413](../../src/transport.rs#L1413) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `request.clone` | [1408](../../src/transport.rs#L1408) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Duration::from_secs` | [1409](../../src/transport.rs#L1409), [1415](../../src/transport.rs#L1415) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `crate::McpCancellationToken::default` | [1410](../../src/transport.rs#L1410), [1416](../../src/transport.rs#L1416) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `server.await.unwrap` | [1429](../../src/transport.rs#L1429) | receiver-type-required |
| `cancelled_response_tombstone_is_consumed_once` | `BTreeSet::from` | [1449](../../src/transport.rs#L1449) | external-constructor-callback-or-unresolved |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `SseDecoder::default` | [1471](../../src/transport.rs#L1471) | external-constructor-callback-or-unresolved |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `decoder             .push(b"\"result\":{}}\n\n: connection remains open\n")             .expect` | [1478](../../src/transport.rs#L1478) | receiver-type-required |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `decoder             .push` | [1478](../../src/transport.rs#L1478) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::time::Instant::now` | [1490](../../src/transport.rs#L1490) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build()             .expect` | [1491](../../src/transport.rs#L1491) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build` | [1491](../../src/transport.rs#L1491) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all` | [1491](../../src/transport.rs#L1491) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread` | [1491](../../src/transport.rs#L1491) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `runtime.block_on` | [1495](../../src/transport.rs#L1495) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::sync::oneshot::channel` | [1496](../../src/transport.rs#L1496) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::thread::spawn` | [1497](../../src/transport.rs#L1497) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::thread::sleep` | [1498](../../src/transport.rs#L1498) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `Duration::from_secs` | [1498](../../src/transport.rs#L1498) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `sender.send` | [1499](../../src/transport.rs#L1499) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `Ok` | [1499](../../src/transport.rs#L1499) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `drop` | [1506](../../src/transport.rs#L1506) | external-constructor-callback-or-unresolved |
