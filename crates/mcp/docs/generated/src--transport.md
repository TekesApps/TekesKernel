# mcp::transport

[Package atlas](index.md) · [Source](../../src/transport.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [mcp::transport::McpMethodObserver](../../src/transport.rs#L14) | type_item | `pub` |  |
| [mcp::transport::McpValueObserver](../../src/transport.rs#L16) | type_item | `pub` |  |
| [mcp::transport::McpTransport](../../src/transport.rs#L27) | trait_item | `pub` |  |
| [mcp::transport::McpTransport::supports_http_discovery](../../src/transport.rs#L28) | function_item | `private` |  |
| [mcp::transport::McpTransport::start_catalog_subscription](../../src/transport.rs#L31) | function_item | `private` |  |
| [mcp::transport::McpTransport::install_tool_catalog](../../src/transport.rs#L40) | function_item | `private` |  |
| [mcp::transport::McpTransport::start_scoped_request](../../src/transport.rs#L50) | function_item | `private` |  |
| [mcp::transport::McpTransport::request](../../src/transport.rs#L60) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::notify](../../src/transport.rs#L66) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::close](../../src/transport.rs#L68) | function_signature_item | `private` |  |
| [mcp::transport::McpTransport::reconnect](../../src/transport.rs#L70) | function_item | `private` |  |
| [mcp::transport::McpTransport::mark_cancelled](../../src/transport.rs#L78) | function_item | `private` |  |
| [mcp::transport::McpTransport::take_notifications](../../src/transport.rs#L80) | function_item | `private` |  |
| [mcp::transport::StdioTransport](../../src/transport.rs#L85) | struct_item | `pub` |  |
| [mcp::transport::StdioSpawnSpec](../../src/transport.rs#L93) | struct_item | `private` |  |
| [mcp::transport::SecretText](../../src/transport.rs#L101) | struct_item | `private` |  |
| [mcp::transport::SecretText::new](../../src/transport.rs#L104) | function_item | `private` |  |
| [mcp::transport::SecretText::as_str](../../src/transport.rs#L108) | function_item | `private` |  |
| [mcp::transport::SecretText::drop](../../src/transport.rs#L114) | function_item | `private` |  |
| [mcp::transport::StdioParts](../../src/transport.rs#L119) | struct_item | `private` |  |
| [mcp::transport::StdioGeneration](../../src/transport.rs#L124) | struct_item | `private` |  |
| [mcp::transport::STDERR_POLL_INTERVAL](../../src/transport.rs#L132) | const_item | `private` |  |
| [mcp::transport::STDIO_REAP_TIMEOUT](../../src/transport.rs#L133) | const_item | `private` |  |
| [mcp::transport::StdioTransport::spawn](../../src/transport.rs#L136) | function_item | `pub` |  |
| [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L159) | function_item | `private` |  |
| [mcp::transport::StdioTransport::restart](../../src/transport.rs#L170) | function_item | `private` |  |
| [mcp::transport::StdioTransport::request_inner](../../src/transport.rs#L180) | function_item | `private` |  |
| [mcp::transport::StdioTransport::close_after_protocol_error](../../src/transport.rs#L202) | function_item | `private` |  |
| [mcp::transport::StdioTransport::write](../../src/transport.rs#L212) | function_item | `private` |  |
| [mcp::transport::StdioTransport::read_response_line](../../src/transport.rs#L238) | function_item | `private` |  |
| [mcp::transport::spawn_stdio](../../src/transport.rs#L278) | function_item | `private` |  |
| [mcp::transport::set_nonblocking](../../src/transport.rs#L356) | function_item | `private` |  |
| [mcp::transport::reap_stdio](../../src/transport.rs#L363) | function_item | `private` |  |
| [mcp::transport::start_stdio_reaper](../../src/transport.rs#L388) | function_item | `private` |  |
| [mcp::transport::await_stdio_reaper](../../src/transport.rs#L424) | function_item | `private` |  |
| [mcp::transport::StdioTransport::request](../../src/transport.rs#L439) | function_item | `private` |  |
| [mcp::transport::StdioTransport::notify](../../src/transport.rs#L450) | function_item | `private` |  |
| [mcp::transport::StdioTransport::close](../../src/transport.rs#L459) | function_item | `private` |  |
| [mcp::transport::StdioTransport::reconnect](../../src/transport.rs#L463) | function_item | `private` |  |
| [mcp::transport::StdioTransport::mark_cancelled](../../src/transport.rs#L467) | function_item | `private` |  |
| [mcp::transport::StdioTransport::take_notifications](../../src/transport.rs#L471) | function_item | `private` |  |
| [mcp::transport::StdioTransport::drop](../../src/transport.rs#L477) | function_item | `private` |  |
| [mcp::transport::HttpTransport](../../src/transport.rs#L484) | struct_item | `pub` |  |
| [mcp::transport::AuthorizationProvider](../../src/transport.rs#L499) | type_item | `private` |  |
| [mcp::transport::HttpTransport::drop](../../src/transport.rs#L503) | function_item | `private` |  |
| [mcp::transport::HttpRequestAuthorization](../../src/transport.rs#L511) | struct_item | `pub` |  |
| [mcp::transport::HttpRequestAuthorization::drop](../../src/transport.rs#L518) | function_item | `private` |  |
| [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L531) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L541) | function_item | `pub(crate)` |  |
| [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L554) | function_item | `pub(crate)` |  |
| [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L564) | function_item | `private` |  |
| [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L627) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::with_notification_handler](../../src/transport.rs#L671) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new](../../src/transport.rs#L678) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L703) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L727) | function_item | `pub` |  |
| [mcp::transport::HttpTransport::request_headers](../../src/transport.rs#L784) | function_item | `private` |  |
| [mcp::transport::HttpTransport::start_catalog_subscription](../../src/transport.rs#L910) | function_item | `private` |  |
| [mcp::transport::HttpTransport::install_tool_catalog](../../src/transport.rs#L929) | function_item | `private` |  |
| [mcp::transport::HttpTransport::supports_http_discovery](../../src/transport.rs#L934) | function_item | `private` |  |
| [mcp::transport::HttpTransport::start_scoped_request](../../src/transport.rs#L938) | function_item | `private` |  |
| [mcp::transport::HttpTransport::request](../../src/transport.rs#L948) | function_item | `private` |  |
| [mcp::transport::HttpTransport::notify](../../src/transport.rs#L1108) | function_item | `private` |  |
| [mcp::transport::HttpTransport::close](../../src/transport.rs#L1137) | function_item | `private` |  |
| [mcp::transport::HttpTransport::reconnect](../../src/transport.rs#L1171) | function_item | `private` |  |
| [mcp::transport::HttpTransport::take_notifications](../../src/transport.rs#L1181) | function_item | `private` |  |
| [mcp::transport::SseDecoder](../../src/transport.rs#L1187) | struct_item | `private` |  |
| [mcp::transport::SseDecoder::push](../../src/transport.rs#L1194) | function_item | `private` |  |
| [mcp::transport::Inbound](../../src/transport.rs#L1238) | enum_item | `private` |  |
| [mcp::transport::InboundWire](../../src/transport.rs#L1246) | struct_item | `private` |  |
| [mcp::transport::decode_response](../../src/transport.rs#L1260) | function_item | `private` |  |
| [mcp::transport::decode_inbound](../../src/transport.rs#L1269) | function_item | `private` |  |
| [mcp::transport::tests::owned_scoped_requests_cannot_outlive_close_or_reconnect](../../src/transport.rs#L1346) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::duplicate_json_members_fail_before_value_collapse](../../src/transport.rs#L1426) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::cancelled_response_tombstone_is_consumed_once](../../src/transport.rs#L1440) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::sse_decoder_yields_the_first_complete_event_without_eof](../../src/transport.rs#L1462) | function_item | `private` | test; #[cfg(test)] |
| [mcp::transport::tests::timed_out_detached_reaper_does_not_delay_runtime_drop](../../src/transport.rs#L1481) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `File` | `std::fs::File` | `private` |
| `Read` | `std::io::Read` | `private` |
| `OwnedFd` | `std::os::fd::OwnedFd` | `private` |
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
| `install_tool_catalog` | `crate::parameter_headers::ParameterHeaders::catalog` | [41](../../src/transport.rs#L41) | [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) |
| `install_tool_catalog` | `catalog.values().any` | [42](../../src/transport.rs#L42) | receiver-type-required |
| `install_tool_catalog` | `catalog.values` | [42](../../src/transport.rs#L42) | receiver-type-required |
| `install_tool_catalog` | `projection.is_empty` | [42](../../src/transport.rs#L42) | receiver-type-required |
| `install_tool_catalog` | `Err` | [43](../../src/transport.rs#L43) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `McpError::Unsupported` | [43](../../src/transport.rs#L43) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `"transport cannot carry MCP parameter headers".into` | [44](../../src/transport.rs#L44) | receiver-type-required |
| `install_tool_catalog` | `Ok` | [47](../../src/transport.rs#L47) | external-constructor-callback-or-unresolved |
| `reconnect` | `Box::pin` | [71](../../src/transport.rs#L71) | external-constructor-callback-or-unresolved |
| `reconnect` | `Err` | [72](../../src/transport.rs#L72) | external-constructor-callback-or-unresolved |
| `reconnect` | `McpError::Transport` | [72](../../src/transport.rs#L72) | external-constructor-callback-or-unresolved |
| `reconnect` | `"MCP transport does not support reconnect".to_owned` | [73](../../src/transport.rs#L73) | receiver-type-required |
| `take_notifications` | `Vec::new` | [81](../../src/transport.rs#L81) | external-constructor-callback-or-unresolved |
| `new` | `Self` | [105](../../src/transport.rs#L105) | external-constructor-callback-or-unresolved |
| `drop` | `self.0.clear` | [115](../../src/transport.rs#L115) | receiver-type-required |
| `STDERR_POLL_INTERVAL` | `Duration::from_millis` | [132](../../src/transport.rs#L132) | external-constructor-callback-or-unresolved |
| `STDIO_REAP_TIMEOUT` | `Duration::from_secs` | [133](../../src/transport.rs#L133) | external-constructor-callback-or-unresolved |
| `spawn` | `argv.to_vec` | [142](../../src/transport.rs#L142) | receiver-type-required |
| `spawn` | `environment                 .iter()                 .map(&#124;(name, value)&#124; (name.clone(), SecretText::new(value.clone())))                 .collect` | [144](../../src/transport.rs#L144) | receiver-type-required |
| `spawn` | `environment                 .iter()                 .map` | [144](../../src/transport.rs#L144) | receiver-type-required |
| `spawn` | `environment                 .iter` | [144](../../src/transport.rs#L144) | receiver-type-required |
| `spawn` | `name.clone` | [146](../../src/transport.rs#L146) | receiver-type-required |
| `spawn` | `SecretText::new` | [146](../../src/transport.rs#L146) | [mcp::transport::SecretText::new](../../src/transport.rs#L104) |
| `spawn` | `value.clone` | [146](../../src/transport.rs#L146) | receiver-type-required |
| `spawn` | `spawn_stdio` | [149](../../src/transport.rs#L149) | [mcp::transport::spawn_stdio](../../src/transport.rs#L278) |
| `spawn` | `Ok` | [150](../../src/transport.rs#L150) | external-constructor-callback-or-unresolved |
| `spawn` | `Some` | [151](../../src/transport.rs#L151) | external-constructor-callback-or-unresolved |
| `spawn` | `BTreeSet::new` | [154](../../src/transport.rs#L154) | external-constructor-callback-or-unresolved |
| `spawn` | `Vec::new` | [155](../../src/transport.rs#L155) | external-constructor-callback-or-unresolved |
| `terminate` | `self.generation.take` | [160](../../src/transport.rs#L160) | receiver-type-required |
| `terminate` | `Ok` | [161](../../src/transport.rs#L161) | external-constructor-callback-or-unresolved |
| `terminate` | `start_stdio_reaper` | [166](../../src/transport.rs#L166) | [mcp::transport::start_stdio_reaper](../../src/transport.rs#L388) |
| `terminate` | `await_stdio_reaper` | [167](../../src/transport.rs#L167) | [mcp::transport::await_stdio_reaper](../../src/transport.rs#L424) |
| `restart` | `self.terminate` | [171](../../src/transport.rs#L171) | [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L159) |
| `restart` | `spawn_stdio` | [172](../../src/transport.rs#L172) | [mcp::transport::spawn_stdio](../../src/transport.rs#L278) |
| `restart` | `Some` | [173](../../src/transport.rs#L173) | external-constructor-callback-or-unresolved |
| `restart` | `self.cancelled_ids.clear` | [175](../../src/transport.rs#L175) | receiver-type-required |
| `restart` | `self.notifications.clear` | [176](../../src/transport.rs#L176) | receiver-type-required |
| `restart` | `Ok` | [177](../../src/transport.rs#L177) | external-constructor-callback-or-unresolved |
| `request_inner` | `request             .id             .ok_or_else` | [185](../../src/transport.rs#L185) | receiver-type-required |
| `request_inner` | `McpError::Protocol` | [187](../../src/transport.rs#L187) | external-constructor-callback-or-unresolved |
| `request_inner` | `"request id is absent".to_owned` | [187](../../src/transport.rs#L187) | receiver-type-required |
| `request_inner` | `self.write` | [188](../../src/transport.rs#L188) | [mcp::transport::StdioTransport::write](../../src/transport.rs#L212) |
| `request_inner` | `tokio::time::Instant::now` | [189](../../src/transport.rs#L189) | external-constructor-callback-or-unresolved |
| `request_inner` | `tokio::time::timeout_at(deadline, self.read_response_line())                 .await                 .map_err` | [191](../../src/transport.rs#L191) | receiver-type-required |
| `request_inner` | `tokio::time::timeout_at` | [191](../../src/transport.rs#L191) | external-constructor-callback-or-unresolved |
| `request_inner` | `self.read_response_line` | [191](../../src/transport.rs#L191) | [mcp::transport::StdioTransport::read_response_line](../../src/transport.rs#L238) |
| `request_inner` | `McpError::Timeout` | [193](../../src/transport.rs#L193) | external-constructor-callback-or-unresolved |
| `request_inner` | `request.method.clone` | [193](../../src/transport.rs#L193) | receiver-type-required |
| `request_inner` | `decode_inbound` | [194](../../src/transport.rs#L194) | [mcp::transport::decode_inbound](../../src/transport.rs#L1269) |
| `request_inner` | `Ok` | [195](../../src/transport.rs#L195) | external-constructor-callback-or-unresolved |
| `request_inner` | `self.notifications.push` | [196](../../src/transport.rs#L196) | receiver-type-required |
| `close_after_protocol_error` | `self.terminate` | [207](../../src/transport.rs#L207) | [mcp::transport::StdioTransport::terminate](../../src/transport.rs#L159) |
| `write` | `request.canonical_line` | [213](../../src/transport.rs#L213) | receiver-type-required |
| `write` | `bytes.len` | [214](../../src/transport.rs#L214) | receiver-type-required |
| `write` | `Err` | [215](../../src/transport.rs#L215) | external-constructor-callback-or-unresolved |
| `write` | `McpError::Protocol` | [215](../../src/transport.rs#L215) | external-constructor-callback-or-unresolved |
| `write` | `"outbound frame exceeds 4 MiB".to_owned` | [216](../../src/transport.rs#L216) | receiver-type-required |
| `write` | `pipe::Sender::from_file(             self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone()                 .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?,         )         .map_err` | [219](../../src/transport.rs#L219) | receiver-type-required |
| `write` | `pipe::Sender::from_file` | [219](../../src/transport.rs#L219) | external-constructor-callback-or-unresolved |
| `write` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone()                 .map_err` | [220](../../src/transport.rs#L220) | receiver-type-required |
| `write` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdin                 .try_clone` | [220](../../src/transport.rs#L220) | receiver-type-required |
| `write` | `self.generation                 .as_ref()                 .ok_or_else` | [220](../../src/transport.rs#L220) | receiver-type-required |
| `write` | `self.generation                 .as_ref` | [220](../../src/transport.rs#L220) | receiver-type-required |
| `write` | `McpError::Transport` | [222](../../src/transport.rs#L222), [225](../../src/transport.rs#L225), [227](../../src/transport.rs#L227), [231](../../src/transport.rs#L231), [235](../../src/transport.rs#L235) | external-constructor-callback-or-unresolved |
| `write` | `"stdio generation is closed".to_owned` | [222](../../src/transport.rs#L222) | receiver-type-required |
| `write` | `error.to_string` | [225](../../src/transport.rs#L225), [227](../../src/transport.rs#L227), [231](../../src/transport.rs#L231), [235](../../src/transport.rs#L235) | receiver-type-required |
| `write` | `stdin             .write_all(&bytes)             .await             .map_err` | [228](../../src/transport.rs#L228) | receiver-type-required |
| `write` | `stdin             .write_all` | [228](../../src/transport.rs#L228) | receiver-type-required |
| `write` | `stdin             .flush()             .await             .map_err` | [232](../../src/transport.rs#L232) | receiver-type-required |
| `write` | `stdin             .flush` | [232](../../src/transport.rs#L232) | receiver-type-required |
| `read_response_line` | `Vec::new` | [239](../../src/transport.rs#L239) | external-constructor-callback-or-unresolved |
| `read_response_line` | `pipe::Receiver::from_file(             self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone()                 .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?,         )         .map_err` | [240](../../src/transport.rs#L240) | receiver-type-required |
| `read_response_line` | `pipe::Receiver::from_file` | [240](../../src/transport.rs#L240) | external-constructor-callback-or-unresolved |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone()                 .map_err` | [241](../../src/transport.rs#L241) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else(&#124;&#124; McpError::Transport("stdio generation is closed".to_owned()))?                 .stdout                 .try_clone` | [241](../../src/transport.rs#L241) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref()                 .ok_or_else` | [241](../../src/transport.rs#L241) | receiver-type-required |
| `read_response_line` | `self.generation                 .as_ref` | [241](../../src/transport.rs#L241) | receiver-type-required |
| `read_response_line` | `McpError::Transport` | [243](../../src/transport.rs#L243), [246](../../src/transport.rs#L246), [248](../../src/transport.rs#L248), [254](../../src/transport.rs#L254), [256](../../src/transport.rs#L256) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"stdio generation is closed".to_owned` | [243](../../src/transport.rs#L243) | receiver-type-required |
| `read_response_line` | `error.to_string` | [246](../../src/transport.rs#L246), [248](../../src/transport.rs#L248), [254](../../src/transport.rs#L254) | receiver-type-required |
| `read_response_line` | `bytes.len` | [249](../../src/transport.rs#L249), [263](../../src/transport.rs#L263) | receiver-type-required |
| `read_response_line` | `stdout                 .read(&mut byte)                 .await                 .map_err` | [251](../../src/transport.rs#L251) | receiver-type-required |
| `read_response_line` | `stdout                 .read` | [251](../../src/transport.rs#L251) | receiver-type-required |
| `read_response_line` | `Err` | [256](../../src/transport.rs#L256), [264](../../src/transport.rs#L264), [270](../../src/transport.rs#L270) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"stdio EOF".to_owned` | [256](../../src/transport.rs#L256) | receiver-type-required |
| `read_response_line` | `bytes.push` | [258](../../src/transport.rs#L258) | receiver-type-required |
| `read_response_line` | `bytes.last` | [263](../../src/transport.rs#L263), [269](../../src/transport.rs#L269) | receiver-type-required |
| `read_response_line` | `Some` | [263](../../src/transport.rs#L263), [269](../../src/transport.rs#L269) | external-constructor-callback-or-unresolved |
| `read_response_line` | `McpError::Protocol` | [264](../../src/transport.rs#L264), [270](../../src/transport.rs#L270) | external-constructor-callback-or-unresolved |
| `read_response_line` | `"inbound frame exceeds 4 MiB or lacks LF framing".to_owned` | [265](../../src/transport.rs#L265) | receiver-type-required |
| `read_response_line` | `bytes.pop` | [268](../../src/transport.rs#L268) | receiver-type-required |
| `read_response_line` | `"stdio frames require LF rather than CRLF".to_owned` | [271](../../src/transport.rs#L271) | receiver-type-required |
| `read_response_line` | `Ok` | [274](../../src/transport.rs#L274) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `spawn.cwd.clone` | [280](../../src/transport.rs#L280) | receiver-type-required |
| `spawn_stdio` | `argv         .split_first()         .ok_or_else` | [282](../../src/transport.rs#L282) | receiver-type-required |
| `spawn_stdio` | `argv         .split_first` | [282](../../src/transport.rs#L282) | receiver-type-required |
| `spawn_stdio` | `McpError::Transport` | [284](../../src/transport.rs#L284), [300](../../src/transport.rs#L300), [304](../../src/transport.rs#L304), [308](../../src/transport.rs#L308), [312](../../src/transport.rs#L312), [343](../../src/transport.rs#L343) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `"stdio argv is empty".to_owned` | [284](../../src/transport.rs#L284) | receiver-type-required |
| `spawn_stdio` | `Command::new` | [285](../../src/transport.rs#L285) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin(Stdio::piped())         .stdout(Stdio::piped())         .stderr` | [286](../../src/transport.rs#L286) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin(Stdio::piped())         .stdout` | [286](../../src/transport.rs#L286) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear()         .stdin` | [286](../../src/transport.rs#L286) | receiver-type-required |
| `spawn_stdio` | `command         .args(arguments)         .env_clear` | [286](../../src/transport.rs#L286) | receiver-type-required |
| `spawn_stdio` | `command         .args` | [286](../../src/transport.rs#L286) | receiver-type-required |
| `spawn_stdio` | `Stdio::piped` | [289](../../src/transport.rs#L289), [290](../../src/transport.rs#L290), [291](../../src/transport.rs#L291) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `command.env` | [293](../../src/transport.rs#L293) | receiver-type-required |
| `spawn_stdio` | `value.as_str` | [293](../../src/transport.rs#L293) | receiver-type-required |
| `spawn_stdio` | `command.current_dir` | [296](../../src/transport.rs#L296) | receiver-type-required |
| `spawn_stdio` | `command         .spawn()         .map_err` | [298](../../src/transport.rs#L298) | receiver-type-required |
| `spawn_stdio` | `command         .spawn` | [298](../../src/transport.rs#L298) | receiver-type-required |
| `spawn_stdio` | `error.to_string` | [300](../../src/transport.rs#L300), [343](../../src/transport.rs#L343) | receiver-type-required |
| `spawn_stdio` | `child         .stdin         .take()         .ok_or_else` | [301](../../src/transport.rs#L301) | receiver-type-required |
| `spawn_stdio` | `child         .stdin         .take` | [301](../../src/transport.rs#L301) | receiver-type-required |
| `spawn_stdio` | `"stdio stdin is absent".to_owned` | [304](../../src/transport.rs#L304) | receiver-type-required |
| `spawn_stdio` | `child         .stdout         .take()         .ok_or_else` | [305](../../src/transport.rs#L305) | receiver-type-required |
| `spawn_stdio` | `child         .stdout         .take` | [305](../../src/transport.rs#L305) | receiver-type-required |
| `spawn_stdio` | `"stdio stdout is absent".to_owned` | [308](../../src/transport.rs#L308) | receiver-type-required |
| `spawn_stdio` | `child         .stderr         .take()         .ok_or_else` | [309](../../src/transport.rs#L309) | receiver-type-required |
| `spawn_stdio` | `child         .stderr         .take` | [309](../../src/transport.rs#L309) | receiver-type-required |
| `spawn_stdio` | `"stdio stderr is absent".to_owned` | [312](../../src/transport.rs#L312) | receiver-type-required |
| `spawn_stdio` | `File::from` | [313](../../src/transport.rs#L313), [347](../../src/transport.rs#L347), [348](../../src/transport.rs#L348) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `OwnedFd::from` | [313](../../src/transport.rs#L313), [347](../../src/transport.rs#L347), [348](../../src/transport.rs#L348) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `set_nonblocking` | [314](../../src/transport.rs#L314) | [mcp::transport::set_nonblocking](../../src/transport.rs#L356) |
| `spawn_stdio` | `Arc::new` | [315](../../src/transport.rs#L315), [317](../../src/transport.rs#L317) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Mutex::new` | [315](../../src/transport.rs#L315) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Vec::new` | [315](../../src/transport.rs#L315) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `Arc::clone` | [316](../../src/transport.rs#L316), [318](../../src/transport.rs#L318) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `AtomicBool::new` | [317](../../src/transport.rs#L317) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `std::thread::Builder::new()         .name("tekes-mcp-stderr".to_owned())         .spawn(move &#124;&#124; {             let mut buffer = [0_u8; 8 * 1024];             while !stderr_task_stop.load(Ordering::Acquire) {                 let count = match stderr.read(&mut buffer) {                     Ok(0) => break,                     Ok(count) => count,                     Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {                         std::thread::sleep(STDERR_POLL_INTERVAL);                         continue;                     }                     Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,                     Err(_) => break,                 };                 if count > 0 {                     let mut retained = diagnostics_writer                         .lock()                         .unwrap_or_else(std::sync::PoisonError::into_inner);                     let remaining = (64_usize * 1024).saturating_sub(retained.len());                     retained.extend_from_slice(&buffer[..count.min(remaining)]);                 }             }         })         .map_err` | [319](../../src/transport.rs#L319) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new()         .name("tekes-mcp-stderr".to_owned())         .spawn` | [319](../../src/transport.rs#L319) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new()         .name` | [319](../../src/transport.rs#L319) | receiver-type-required |
| `spawn_stdio` | `std::thread::Builder::new` | [319](../../src/transport.rs#L319) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `"tekes-mcp-stderr".to_owned` | [320](../../src/transport.rs#L320) | receiver-type-required |
| `spawn_stdio` | `stderr_task_stop.load` | [323](../../src/transport.rs#L323) | receiver-type-required |
| `spawn_stdio` | `stderr.read` | [324](../../src/transport.rs#L324) | receiver-type-required |
| `spawn_stdio` | `error.kind` | [327](../../src/transport.rs#L327), [331](../../src/transport.rs#L331) | receiver-type-required |
| `spawn_stdio` | `std::thread::sleep` | [328](../../src/transport.rs#L328) | external-constructor-callback-or-unresolved |
| `spawn_stdio` | `diagnostics_writer                         .lock()                         .unwrap_or_else` | [335](../../src/transport.rs#L335) | receiver-type-required |
| `spawn_stdio` | `diagnostics_writer                         .lock` | [335](../../src/transport.rs#L335) | receiver-type-required |
| `spawn_stdio` | `(64_usize * 1024).saturating_sub` | [338](../../src/transport.rs#L338) | receiver-type-required |
| `spawn_stdio` | `retained.len` | [338](../../src/transport.rs#L338) | receiver-type-required |
| `spawn_stdio` | `retained.extend_from_slice` | [339](../../src/transport.rs#L339) | receiver-type-required |
| `spawn_stdio` | `count.min` | [339](../../src/transport.rs#L339) | receiver-type-required |
| `spawn_stdio` | `Ok` | [344](../../src/transport.rs#L344) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `rustix::fs::fcntl_getfl(file).map_err` | [358](../../src/transport.rs#L358) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_getfl` | [358](../../src/transport.rs#L358) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `McpError::Transport` | [358](../../src/transport.rs#L358), [360](../../src/transport.rs#L360) | external-constructor-callback-or-unresolved |
| `set_nonblocking` | `error.to_string` | [358](../../src/transport.rs#L358), [360](../../src/transport.rs#L360) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_setfl(file, flags &#124; rustix::fs::OFlags::NONBLOCK)         .map_err` | [359](../../src/transport.rs#L359) | receiver-type-required |
| `set_nonblocking` | `rustix::fs::fcntl_setfl` | [359](../../src/transport.rs#L359) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `generation.stderr_stop.store` | [364](../../src/transport.rs#L364) | receiver-type-required |
| `reap_stdio` | `drop` | [365](../../src/transport.rs#L365), [366](../../src/transport.rs#L366) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `generation         .child         .try_wait()         .map_err(&#124;error&#124; McpError::Transport(error.to_string()))?         .is_none` | [367](../../src/transport.rs#L367) | receiver-type-required |
| `reap_stdio` | `generation         .child         .try_wait()         .map_err` | [367](../../src/transport.rs#L367) | receiver-type-required |
| `reap_stdio` | `generation         .child         .try_wait` | [367](../../src/transport.rs#L367) | receiver-type-required |
| `reap_stdio` | `McpError::Transport` | [370](../../src/transport.rs#L370), [376](../../src/transport.rs#L376), [381](../../src/transport.rs#L381), [385](../../src/transport.rs#L385) | external-constructor-callback-or-unresolved |
| `reap_stdio` | `error.to_string` | [370](../../src/transport.rs#L370), [376](../../src/transport.rs#L376), [381](../../src/transport.rs#L381) | receiver-type-required |
| `reap_stdio` | `generation             .child             .kill()             .map_err` | [373](../../src/transport.rs#L373) | receiver-type-required |
| `reap_stdio` | `generation             .child             .kill` | [373](../../src/transport.rs#L373) | receiver-type-required |
| `reap_stdio` | `generation         .child         .wait()         .map_err` | [378](../../src/transport.rs#L378) | receiver-type-required |
| `reap_stdio` | `generation         .child         .wait` | [378](../../src/transport.rs#L378) | receiver-type-required |
| `reap_stdio` | `generation         .stderr_task         .join()         .map_err` | [382](../../src/transport.rs#L382) | receiver-type-required |
| `reap_stdio` | `generation         .stderr_task         .join` | [382](../../src/transport.rs#L382) | receiver-type-required |
| `reap_stdio` | `"stdio stderr drain thread panicked".to_owned` | [385](../../src/transport.rs#L385) | receiver-type-required |
| `start_stdio_reaper` | `Arc::new` | [391](../../src/transport.rs#L391) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Mutex::new` | [391](../../src/transport.rs#L391) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Some` | [391](../../src/transport.rs#L391) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `Arc::clone` | [392](../../src/transport.rs#L392) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `tokio::sync::oneshot::channel` | [393](../../src/transport.rs#L393) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `std::thread::Builder::new()         .name("tekes-mcp-reaper".to_owned())         .spawn` | [394](../../src/transport.rs#L394) | receiver-type-required |
| `start_stdio_reaper` | `std::thread::Builder::new()         .name` | [394](../../src/transport.rs#L394) | receiver-type-required |
| `start_stdio_reaper` | `std::thread::Builder::new` | [394](../../src/transport.rs#L394) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `"tekes-mcp-reaper".to_owned` | [395](../../src/transport.rs#L395) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take()                 .expect` | [397](../../src/transport.rs#L397) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take` | [397](../../src/transport.rs#L397) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock()                 .unwrap_or_else` | [397](../../src/transport.rs#L397) | receiver-type-required |
| `start_stdio_reaper` | `reaper_generation                 .lock` | [397](../../src/transport.rs#L397) | receiver-type-required |
| `start_stdio_reaper` | `sender.send` | [402](../../src/transport.rs#L402) | receiver-type-required |
| `start_stdio_reaper` | `reap_stdio` | [402](../../src/transport.rs#L402) | [mcp::transport::reap_stdio](../../src/transport.rs#L363) |
| `start_stdio_reaper` | `Ok` | [404](../../src/transport.rs#L404) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `generation                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .take` | [406](../../src/transport.rs#L406) | receiver-type-required |
| `start_stdio_reaper` | `generation                 .lock()                 .unwrap_or_else` | [406](../../src/transport.rs#L406) | receiver-type-required |
| `start_stdio_reaper` | `generation                 .lock` | [406](../../src/transport.rs#L406) | receiver-type-required |
| `start_stdio_reaper` | `generation.stderr_stop.store` | [414](../../src/transport.rs#L414) | receiver-type-required |
| `start_stdio_reaper` | `generation.child.kill` | [415](../../src/transport.rs#L415) | receiver-type-required |
| `start_stdio_reaper` | `Err` | [417](../../src/transport.rs#L417) | external-constructor-callback-or-unresolved |
| `start_stdio_reaper` | `McpError::Transport` | [417](../../src/transport.rs#L417) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `tokio::time::timeout` | [427](../../src/transport.rs#L427) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `Err` | [429](../../src/transport.rs#L429), [432](../../src/transport.rs#L432) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `McpError::Transport` | [429](../../src/transport.rs#L429), [432](../../src/transport.rs#L432) | external-constructor-callback-or-unresolved |
| `await_stdio_reaper` | `"stdio reaper thread exited without a result".to_owned` | [430](../../src/transport.rs#L430) | receiver-type-required |
| `await_stdio_reaper` | `"stdio generation reap exceeded one second".to_owned` | [433](../../src/transport.rs#L433) | receiver-type-required |
| `request` | `Box::pin` | [444](../../src/transport.rs#L444) | external-constructor-callback-or-unresolved |
| `request` | `self.request_inner` | [445](../../src/transport.rs#L445) | receiver-type-required |
| `request` | `self.close_after_protocol_error` | [446](../../src/transport.rs#L446) | receiver-type-required |
| `notify` | `Box::pin` | [451](../../src/transport.rs#L451) | external-constructor-callback-or-unresolved |
| `notify` | `notification.id.is_some` | [452](../../src/transport.rs#L452) | receiver-type-required |
| `notify` | `Err` | [453](../../src/transport.rs#L453) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Protocol` | [453](../../src/transport.rs#L453) | external-constructor-callback-or-unresolved |
| `notify` | `"notification has an id".to_owned` | [453](../../src/transport.rs#L453) | receiver-type-required |
| `notify` | `self.write` | [455](../../src/transport.rs#L455) | receiver-type-required |
| `close` | `Box::pin` | [460](../../src/transport.rs#L460) | external-constructor-callback-or-unresolved |
| `close` | `self.terminate` | [460](../../src/transport.rs#L460) | receiver-type-required |
| `reconnect` | `Box::pin` | [464](../../src/transport.rs#L464) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.restart` | [464](../../src/transport.rs#L464) | receiver-type-required |
| `mark_cancelled` | `self.cancelled_ids.insert` | [468](../../src/transport.rs#L468) | receiver-type-required |
| `take_notifications` | `std::mem::take` | [472](../../src/transport.rs#L472) | external-constructor-callback-or-unresolved |
| `drop` | `self.generation.take` | [478](../../src/transport.rs#L478) | receiver-type-required |
| `drop` | `start_stdio_reaper` | [479](../../src/transport.rs#L479) | [mcp::transport::start_stdio_reaper](../../src/transport.rs#L388) |
| `drop` | `self.scoped_generation.cancel` | [504](../../src/transport.rs#L504) | receiver-type-required |
| `drop` | `self.headers.values_mut` | [519](../../src/transport.rs#L519) | receiver-type-required |
| `drop` | `value.clear` | [520](../../src/transport.rs#L520), [523](../../src/transport.rs#L523) | receiver-type-required |
| `request_modern_scoped` | `self.request_modern_scoped_observed` | [537](../../src/transport.rs#L537) | [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L541) |
| `request_modern_scoped_observed` | `self.start_modern_scoped` | [548](../../src/transport.rs#L548) | [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L554) |
| `request_modern_scoped_observed` | `request.clone` | [548](../../src/transport.rs#L548) | receiver-type-required |
| `start_modern_scoped` | `self.start_modern_scoped_detailed` | [561](../../src/transport.rs#L561) | [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L564) |
| `start_modern_scoped_detailed` | `self.notification_handler.clone` | [572](../../src/transport.rs#L572) | receiver-type-required |
| `start_modern_scoped_detailed` | `Arc::new` | [573](../../src/transport.rs#L573) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `observer` | [575](../../src/transport.rs#L575) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `original` | [578](../../src/transport.rs#L578) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `self.scoped_generation.clone` | [581](../../src/transport.rs#L581) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.parameter_headers.clone` | [583](../../src/transport.rs#L583) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.client.clone` | [584](../../src/transport.rs#L584) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.url.clone` | [585](../../src/transport.rs#L585) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter()                 .map(&#124;(k, v)&#124; (k.clone(), SecretText::new(v.as_str().to_owned())))                 .collect` | [586](../../src/transport.rs#L586) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter()                 .map` | [586](../../src/transport.rs#L586) | receiver-type-required |
| `start_modern_scoped_detailed` | `self                 .headers                 .iter` | [586](../../src/transport.rs#L586) | receiver-type-required |
| `start_modern_scoped_detailed` | `k.clone` | [589](../../src/transport.rs#L589) | receiver-type-required |
| `start_modern_scoped_detailed` | `SecretText::new` | [589](../../src/transport.rs#L589) | [mcp::transport::SecretText::new](../../src/transport.rs#L104) |
| `start_modern_scoped_detailed` | `v.as_str().to_owned` | [589](../../src/transport.rs#L589) | receiver-type-required |
| `start_modern_scoped_detailed` | `v.as_str` | [589](../../src/transport.rs#L589) | receiver-type-required |
| `start_modern_scoped_detailed` | `self.authorization.clone` | [591](../../src/transport.rs#L591) | receiver-type-required |
| `start_modern_scoped_detailed` | `Vec::new` | [594](../../src/transport.rs#L594) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `Some` | [595](../../src/transport.rs#L595), [606](../../src/transport.rs#L606) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `crate::McpCancellationToken::default` | [598](../../src/transport.rs#L598) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `Box::pin` | [600](../../src/transport.rs#L600) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `serde_json::to_value(&request).map_err` | [602](../../src/transport.rs#L602) | receiver-type-required |
| `start_modern_scoped_detailed` | `serde_json::to_value` | [602](../../src/transport.rs#L602) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `McpError::Protocol` | [602](../../src/transport.rs#L602) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `e.to_string` | [602](../../src/transport.rs#L602) | receiver-type-required |
| `start_modern_scoped_detailed` | `value                 .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")                 .and_then` | [603](../../src/transport.rs#L603) | receiver-type-required |
| `start_modern_scoped_detailed` | `value                 .pointer` | [603](../../src/transport.rs#L603) | receiver-type-required |
| `start_modern_scoped_detailed` | `Err` | [608](../../src/transport.rs#L608), [613](../../src/transport.rs#L613) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `McpError::Unsupported` | [608](../../src/transport.rs#L608) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `"request-scoped HTTP requires modern protocol metadata".into` | [609](../../src/transport.rs#L609) | receiver-type-required |
| `start_modern_scoped_detailed` | `McpError::Transport` | [613](../../src/transport.rs#L613) | external-constructor-callback-or-unresolved |
| `start_modern_scoped_detailed` | `"HTTP transport is closed".into` | [613](../../src/transport.rs#L613) | receiver-type-required |
| `listen_subscription` | `crate::McpSubscriptionFilter::new` | [636](../../src/transport.rs#L636) | [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) |
| `listen_subscription` | `filter.is_empty` | [637](../../src/transport.rs#L637) | receiver-type-required |
| `listen_subscription` | `Box::pin` | [638](../../src/transport.rs#L638), [650](../../src/transport.rs#L650) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `Err` | [639](../../src/transport.rs#L639), [650](../../src/transport.rs#L650) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `McpError::Unsupported` | [639](../../src/transport.rs#L639) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `"empty subscription or invalid identity".into` | [640](../../src/transport.rs#L640) | receiver-type-required |
| `listen_subscription` | `filter.request_params` | [644](../../src/transport.rs#L644) | receiver-type-required |
| `listen_subscription` | `IJsonValue::parse_str(&params.to_string())             .map_err` | [646](../../src/transport.rs#L646) | receiver-type-required |
| `listen_subscription` | `IJsonValue::parse_str` | [646](../../src/transport.rs#L646) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `listen_subscription` | `params.to_string` | [646](../../src/transport.rs#L646) | receiver-type-required |
| `listen_subscription` | `McpError::Protocol` | [647](../../src/transport.rs#L647) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `error.to_string` | [647](../../src/transport.rs#L647) | receiver-type-required |
| `listen_subscription` | `Mutex::new` | [652](../../src/transport.rs#L652) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `Arc::new` | [653](../../src/transport.rs#L653) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `filter                 .lock()                 .map(&#124;mut filter&#124; filter.accepts(notification))                 .unwrap_or` | [654](../../src/transport.rs#L654) | receiver-type-required |
| `listen_subscription` | `filter                 .lock()                 .map` | [654](../../src/transport.rs#L654) | receiver-type-required |
| `listen_subscription` | `filter                 .lock` | [654](../../src/transport.rs#L654) | receiver-type-required |
| `listen_subscription` | `filter.accepts` | [656](../../src/transport.rs#L656) | receiver-type-required |
| `listen_subscription` | `sink` | [659](../../src/transport.rs#L659) | external-constructor-callback-or-unresolved |
| `listen_subscription` | `self.start_modern_scoped_detailed` | [662](../../src/transport.rs#L662) | [mcp::transport::HttpTransport::start_modern_scoped_detailed](../../src/transport.rs#L564) |
| `listen_subscription` | `JsonRpcRequest::call` | [663](../../src/transport.rs#L663) | [mcp::types::JsonRpcRequest::call](../../src/types.rs#L57) |
| `listen_subscription` | `Some` | [663](../../src/transport.rs#L663), [667](../../src/transport.rs#L667) | external-constructor-callback-or-unresolved |
| `with_notification_handler` | `Some` | [675](../../src/transport.rs#L675) | external-constructor-callback-or-unresolved |
| `with_notification_handler` | `Arc::new` | [675](../../src/transport.rs#L675) | external-constructor-callback-or-unresolved |
| `new` | `bearer.map` | [684](../../src/transport.rs#L684) | receiver-type-required |
| `new` | `SecretText::new` | [684](../../src/transport.rs#L684) | [mcp::transport::SecretText::new](../../src/transport.rs#L104) |
| `new` | `value.to_owned` | [684](../../src/transport.rs#L684) | receiver-type-required |
| `new` | `bearer.is_some` | [685](../../src/transport.rs#L685) | receiver-type-required |
| `new` | `"fixed-bearer".to_owned` | [686](../../src/transport.rs#L686) | receiver-type-required |
| `new` | `"anonymous".to_owned` | [688](../../src/transport.rs#L688) | receiver-type-required |
| `new` | `Self::new_with_authorization_provider` | [690](../../src/transport.rs#L690) | [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L703) |
| `new` | `Ok` | [694](../../src/transport.rs#L694) | external-constructor-callback-or-unresolved |
| `new` | `identity.clone` | [695](../../src/transport.rs#L695) | receiver-type-required |
| `new` | `fixed_bearer.as_ref().map` | [696](../../src/transport.rs#L696) | receiver-type-required |
| `new` | `fixed_bearer.as_ref` | [696](../../src/transport.rs#L696) | receiver-type-required |
| `new` | `value.as_str().to_owned` | [696](../../src/transport.rs#L696) | receiver-type-required |
| `new` | `value.as_str` | [696](../../src/transport.rs#L696) | receiver-type-required |
| `new_with_authorization_provider` | `Self::new_with_request_authorization_provider` | [712](../../src/transport.rs#L712) | [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L727) |
| `new_with_authorization_provider` | `authorization` | [716](../../src/transport.rs#L716) | external-constructor-callback-or-unresolved |
| `new_with_authorization_provider` | `Ok` | [717](../../src/transport.rs#L717) | external-constructor-callback-or-unresolved |
| `new_with_authorization_provider` | `BTreeMap::new` | [719](../../src/transport.rs#L719) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `url             .host_str()             .is_some_and` | [736](../../src/transport.rs#L736) | receiver-type-required |
| `new_with_request_authorization_provider` | `url             .host_str` | [736](../../src/transport.rs#L736) | receiver-type-required |
| `new_with_request_authorization_provider` | `url.scheme` | [739](../../src/transport.rs#L739) | receiver-type-required |
| `new_with_request_authorization_provider` | `Err` | [740](../../src/transport.rs#L740), [757](../../src/transport.rs#L757) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `McpError::Transport` | [740](../../src/transport.rs#L740), [747](../../src/transport.rs#L747), [757](../../src/transport.rs#L757), [761](../../src/transport.rs#L761), [767](../../src/transport.rs#L767) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `"HTTP MCP requires HTTPS outside an explicit loopback test".to_owned` | [741](../../src/transport.rs#L741) | receiver-type-required |
| `new_with_request_authorization_provider` | `Vec::new` | [744](../../src/transport.rs#L744), [778](../../src/transport.rs#L778) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `HeaderName::from_bytes(name.as_bytes())                 .map_err` | [746](../../src/transport.rs#L746) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderName::from_bytes` | [746](../../src/transport.rs#L746) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `name.as_bytes` | [746](../../src/transport.rs#L746) | receiver-type-required |
| `new_with_request_authorization_provider` | `error.to_string` | [747](../../src/transport.rs#L747), [761](../../src/transport.rs#L761), [767](../../src/transport.rs#L767) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str().eq_ignore_ascii_case` | [751](../../src/transport.rs#L751), [752](../../src/transport.rs#L752), [753](../../src/transport.rs#L753), [754](../../src/transport.rs#L754) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str` | [751](../../src/transport.rs#L751), [752](../../src/transport.rs#L752), [753](../../src/transport.rs#L753), [754](../../src/transport.rs#L754), [755](../../src/transport.rs#L755) | receiver-type-required |
| `new_with_request_authorization_provider` | `name.as_str().starts_with` | [755](../../src/transport.rs#L755) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderValue::from_str(value).map_err` | [761](../../src/transport.rs#L761) | receiver-type-required |
| `new_with_request_authorization_provider` | `HeaderValue::from_str` | [761](../../src/transport.rs#L761) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `configured.push` | [762](../../src/transport.rs#L762) | receiver-type-required |
| `new_with_request_authorization_provider` | `SecretText::new` | [762](../../src/transport.rs#L762) | [mcp::transport::SecretText::new](../../src/transport.rs#L104) |
| `new_with_request_authorization_provider` | `value.clone` | [762](../../src/transport.rs#L762) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect(reqwest::redirect::Policy::none())             .build()             .map_err` | [764](../../src/transport.rs#L764) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect(reqwest::redirect::Policy::none())             .build` | [764](../../src/transport.rs#L764) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder()             .redirect` | [764](../../src/transport.rs#L764) | receiver-type-required |
| `new_with_request_authorization_provider` | `reqwest::Client::builder` | [764](../../src/transport.rs#L764) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `reqwest::redirect::Policy::none` | [765](../../src/transport.rs#L765) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `Ok` | [768](../../src/transport.rs#L768) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `BTreeMap::new` | [769](../../src/transport.rs#L769) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `Arc::new` | [773](../../src/transport.rs#L773) | external-constructor-callback-or-unresolved |
| `new_with_request_authorization_provider` | `crate::McpCancellationToken::default` | [780](../../src/transport.rs#L780) | external-constructor-callback-or-unresolved |
| `request_headers` | `Err` | [786](../../src/transport.rs#L786), [790](../../src/transport.rs#L790), [827](../../src/transport.rs#L827) | external-constructor-callback-or-unresolved |
| `request_headers` | `McpError::Transport` | [786](../../src/transport.rs#L786), [790](../../src/transport.rs#L790), [812](../../src/transport.rs#L812), [817](../../src/transport.rs#L817), [827](../../src/transport.rs#L827), [834](../../src/transport.rs#L834), [839](../../src/transport.rs#L839) | external-constructor-callback-or-unresolved |
| `request_headers` | `"HTTP MCP peer is closed".to_owned` | [786](../../src/transport.rs#L786) | receiver-type-required |
| `request_headers` | `(self.authorization)` | [788](../../src/transport.rs#L788) | external-constructor-callback-or-unresolved |
| `request_headers` | `authorization.identity.is_empty` | [789](../../src/transport.rs#L789) | receiver-type-required |
| `request_headers` | `"HTTP authorization identity is empty".to_owned` | [791](../../src/transport.rs#L791) | receiver-type-required |
| `request_headers` | `self             .session_authorization_identity             .as_ref()             .is_some_and` | [794](../../src/transport.rs#L794) | receiver-type-required |
| `request_headers` | `self             .session_authorization_identity             .as_ref` | [794](../../src/transport.rs#L794) | receiver-type-required |
| `request_headers` | `Some` | [801](../../src/transport.rs#L801), [865](../../src/transport.rs#L865), [866](../../src/transport.rs#L866), [867](../../src/transport.rs#L867) | external-constructor-callback-or-unresolved |
| `request_headers` | `authorization.identity.clone` | [801](../../src/transport.rs#L801) | receiver-type-required |
| `request_headers` | `HeaderMap::new` | [802](../../src/transport.rs#L802) | external-constructor-callback-or-unresolved |
| `request_headers` | `map.insert` | [803](../../src/transport.rs#L803), [804](../../src/transport.rs#L804), [809](../../src/transport.rs#L809), [831](../../src/transport.rs#L831), [840](../../src/transport.rs#L840), [843](../../src/transport.rs#L843), [854](../../src/transport.rs#L854), [859](../../src/transport.rs#L859), [883](../../src/transport.rs#L883), [896](../../src/transport.rs#L896) | receiver-type-required |
| `request_headers` | `HeaderValue::from_static` | [803](../../src/transport.rs#L803), [806](../../src/transport.rs#L806) | external-constructor-callback-or-unresolved |
| `request_headers` | `name.clone` | [810](../../src/transport.rs#L810) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(value.as_str())                     .map_err` | [811](../../src/transport.rs#L811) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str` | [811](../../src/transport.rs#L811), [833](../../src/transport.rs#L833), [838](../../src/transport.rs#L838), [856](../../src/transport.rs#L856), [861](../../src/transport.rs#L861), [885](../../src/transport.rs#L885), [899](../../src/transport.rs#L899) | external-constructor-callback-or-unresolved |
| `request_headers` | `value.as_str` | [811](../../src/transport.rs#L811) | receiver-type-required |
| `request_headers` | `error.to_string` | [812](../../src/transport.rs#L812), [817](../../src/transport.rs#L817), [834](../../src/transport.rs#L834), [839](../../src/transport.rs#L839), [849](../../src/transport.rs#L849), [857](../../src/transport.rs#L857), [862](../../src/transport.rs#L862), [886](../../src/transport.rs#L886) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes(name.as_bytes())                 .map_err` | [816](../../src/transport.rs#L816) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes` | [816](../../src/transport.rs#L816), [897](../../src/transport.rs#L897) | external-constructor-callback-or-unresolved |
| `request_headers` | `name.as_bytes` | [816](../../src/transport.rs#L816), [897](../../src/transport.rs#L897) | receiver-type-required |
| `request_headers` | `name.as_str().eq_ignore_ascii_case` | [821](../../src/transport.rs#L821), [822](../../src/transport.rs#L822), [823](../../src/transport.rs#L823), [824](../../src/transport.rs#L824) | receiver-type-required |
| `request_headers` | `name.as_str` | [821](../../src/transport.rs#L821), [822](../../src/transport.rs#L822), [823](../../src/transport.rs#L823), [824](../../src/transport.rs#L824), [825](../../src/transport.rs#L825) | receiver-type-required |
| `request_headers` | `name.as_str().starts_with` | [825](../../src/transport.rs#L825) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(value)                     .map_err` | [833](../../src/transport.rs#L833) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&format!("Bearer {bearer}"))                 .map_err` | [838](../../src/transport.rs#L838) | receiver-type-required |
| `request_headers` | `HeaderName::from_static` | [844](../../src/transport.rs#L844), [855](../../src/transport.rs#L855), [860](../../src/transport.rs#L860), [884](../../src/transport.rs#L884) | external-constructor-callback-or-unresolved |
| `request_headers` | `session_id.clone` | [845](../../src/transport.rs#L845) | receiver-type-required |
| `request_headers` | `serde_json::to_value(request).map_err` | [849](../../src/transport.rs#L849) | receiver-type-required |
| `request_headers` | `serde_json::to_value` | [849](../../src/transport.rs#L849) | external-constructor-callback-or-unresolved |
| `request_headers` | `McpError::Protocol` | [849](../../src/transport.rs#L849), [857](../../src/transport.rs#L857), [862](../../src/transport.rs#L862), [886](../../src/transport.rs#L886), [898](../../src/transport.rs#L898), [900](../../src/transport.rs#L900) | external-constructor-callback-or-unresolved |
| `request_headers` | `request_value             .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")             .and_then` | [850](../../src/transport.rs#L850) | receiver-type-required |
| `request_headers` | `request_value             .pointer` | [850](../../src/transport.rs#L850) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(version)                     .map_err` | [856](../../src/transport.rs#L856) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&request.method)                     .map_err` | [861](../../src/transport.rs#L861) | receiver-type-required |
| `request_headers` | `request.method.as_str` | [864](../../src/transport.rs#L864) | receiver-type-required |
| `request_headers` | `key.and_then` | [870](../../src/transport.rs#L870) | receiver-type-required |
| `request_headers` | `request_value["params"][key].as_str` | [870](../../src/transport.rs#L870) | receiver-type-required |
| `request_headers` | `name.is_empty` | [871](../../src/transport.rs#L871) | receiver-type-required |
| `request_headers` | `name.bytes().all` | [872](../../src/transport.rs#L872) | receiver-type-required |
| `request_headers` | `name.bytes` | [872](../../src/transport.rs#L872) | receiver-type-required |
| `request_headers` | `(0x20..=0x7e).contains` | [872](../../src/transport.rs#L872) | receiver-type-required |
| `request_headers` | `name.trim` | [873](../../src/transport.rs#L873) | receiver-type-required |
| `request_headers` | `name.starts_with` | [874](../../src/transport.rs#L874) | receiver-type-required |
| `request_headers` | `name.ends_with` | [874](../../src/transport.rs#L874) | receiver-type-required |
| `request_headers` | `name.to_owned` | [876](../../src/transport.rs#L876) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&encoded)                         .map_err` | [885](../../src/transport.rs#L885) | receiver-type-required |
| `request_headers` | `map.contains_key` | [890](../../src/transport.rs#L890) | receiver-type-required |
| `request_headers` | `request_value["params"]["name"]                 .as_str()                 .and_then` | [891](../../src/transport.rs#L891) | receiver-type-required |
| `request_headers` | `request_value["params"]["name"]                 .as_str` | [891](../../src/transport.rs#L891) | receiver-type-required |
| `request_headers` | `self.parameter_headers.get` | [893](../../src/transport.rs#L893) | receiver-type-required |
| `request_headers` | `projection.project` | [895](../../src/transport.rs#L895) | receiver-type-required |
| `request_headers` | `HeaderName::from_bytes(name.as_bytes())                             .map_err` | [897](../../src/transport.rs#L897) | receiver-type-required |
| `request_headers` | `e.to_string` | [898](../../src/transport.rs#L898), [900](../../src/transport.rs#L900) | receiver-type-required |
| `request_headers` | `HeaderValue::from_str(&value)                             .map_err` | [899](../../src/transport.rs#L899) | receiver-type-required |
| `request_headers` | `Ok` | [905](../../src/transport.rs#L905) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `crate::McpSubscriptionFilter::new(id, capabilities, &[]).is_empty` | [916](../../src/transport.rs#L916) | receiver-type-required |
| `start_catalog_subscription` | `crate::McpSubscriptionFilter::new` | [916](../../src/transport.rs#L916) | [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) |
| `start_catalog_subscription` | `Some` | [919](../../src/transport.rs#L919) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `self.listen_subscription` | [919](../../src/transport.rs#L919) | receiver-type-required |
| `start_catalog_subscription` | `Duration::from_secs` | [923](../../src/transport.rs#L923) | external-constructor-callback-or-unresolved |
| `start_catalog_subscription` | `crate::McpCancellationToken::default` | [924](../../src/transport.rs#L924) | external-constructor-callback-or-unresolved |
| `install_tool_catalog` | `crate::parameter_headers::ParameterHeaders::catalog` | [930](../../src/transport.rs#L930) | [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) |
| `install_tool_catalog` | `Ok` | [931](../../src/transport.rs#L931) | external-constructor-callback-or-unresolved |
| `start_scoped_request` | `Some` | [945](../../src/transport.rs#L945) | external-constructor-callback-or-unresolved |
| `start_scoped_request` | `self.start_modern_scoped` | [945](../../src/transport.rs#L945) | receiver-type-required |
| `request` | `Box::pin` | [953](../../src/transport.rs#L953) | external-constructor-callback-or-unresolved |
| `request` | `request                 .id                 .ok_or_else` | [954](../../src/transport.rs#L954) | receiver-type-required |
| `request` | `McpError::Protocol` | [956](../../src/transport.rs#L956), [1042](../../src/transport.rs#L1042), [1061](../../src/transport.rs#L1061), [1079](../../src/transport.rs#L1079), [1089](../../src/transport.rs#L1089), [1095](../../src/transport.rs#L1095) | external-constructor-callback-or-unresolved |
| `request` | `"request id is absent".to_owned` | [956](../../src/transport.rs#L956) | receiver-type-required |
| `request` | `self.request_headers` | [957](../../src/transport.rs#L957) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json(request)                 .send()                 .await                 .map_err` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json(request)                 .send` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(timeout)                 .json` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self                 .client                 .post(self.url.clone())                 .headers` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self                 .client                 .post` | [958](../../src/transport.rs#L958) | receiver-type-required |
| `request` | `self.url.clone` | [960](../../src/transport.rs#L960) | receiver-type-required |
| `request` | `error.is_timeout` | [967](../../src/transport.rs#L967) | receiver-type-required |
| `request` | `McpError::Timeout` | [968](../../src/transport.rs#L968) | external-constructor-callback-or-unresolved |
| `request` | `request.method.clone` | [968](../../src/transport.rs#L968) | receiver-type-required |
| `request` | `McpError::Transport` | [970](../../src/transport.rs#L970), [974](../../src/transport.rs#L974), [1027](../../src/transport.rs#L1027), [1050](../../src/transport.rs#L1050), [1087](../../src/transport.rs#L1087) | external-constructor-callback-or-unresolved |
| `request` | `error.to_string` | [970](../../src/transport.rs#L970), [1050](../../src/transport.rs#L1050), [1061](../../src/transport.rs#L1061), [1087](../../src/transport.rs#L1087) | receiver-type-required |
| `request` | `response.status().is_redirection` | [973](../../src/transport.rs#L973) | receiver-type-required |
| `request` | `response.status` | [973](../../src/transport.rs#L973), [978](../../src/transport.rs#L978), [979](../../src/transport.rs#L979) | receiver-type-required |
| `request` | `Err` | [974](../../src/transport.rs#L974), [1019](../../src/transport.rs#L1019), [1027](../../src/transport.rs#L1027), [1042](../../src/transport.rs#L1042), [1089](../../src/transport.rs#L1089), [1095](../../src/transport.rs#L1095) | external-constructor-callback-or-unresolved |
| `request` | `"cross-origin redirect rejected".to_owned` | [975](../../src/transport.rs#L975) | receiver-type-required |
| `request` | `response.status().is_success` | [978](../../src/transport.rs#L978) | receiver-type-required |
| `request` | `status.is_client_error` | [982](../../src/transport.rs#L982) | receiver-type-required |
| `request` | `status.as_u16` | [982](../../src/transport.rs#L982) | receiver-type-required |
| `request` | `response.bytes_stream` | [983](../../src/transport.rs#L983), [1046](../../src/transport.rs#L1046), [1084](../../src/transport.rs#L1084) | receiver-type-required |
| `request` | `Vec::new` | [984](../../src/transport.rs#L984), [1085](../../src/transport.rs#L1085) | external-constructor-callback-or-unresolved |
| `request` | `stream.next` | [985](../../src/transport.rs#L985), [1049](../../src/transport.rs#L1049), [1086](../../src/transport.rs#L1086) | receiver-type-required |
| `request` | `bytes.clear` | [987](../../src/transport.rs#L987), [991](../../src/transport.rs#L991) | receiver-type-required |
| `request` | `bytes.len` | [990](../../src/transport.rs#L990), [1088](../../src/transport.rs#L1088) | receiver-type-required |
| `request` | `chunk.len` | [990](../../src/transport.rs#L990), [1088](../../src/transport.rs#L1088) | receiver-type-required |
| `request` | `bytes.extend_from_slice` | [994](../../src/transport.rs#L994), [1091](../../src/transport.rs#L1091) | receiver-type-required |
| `request` | `serde_json::from_slice::<JsonRpcResponse>` | [996](../../src/transport.rs#L996) | external-constructor-callback-or-unresolved |
| `request` | `rpc.error.is_some` | [997](../../src/transport.rs#L997) | receiver-type-required |
| `request` | `request.id.is_some_and` | [998](../../src/transport.rs#L998) | receiver-type-required |
| `request` | `rpc.validate_for(id).is_ok` | [998](../../src/transport.rs#L998) | receiver-type-required |
| `request` | `rpc.validate_for` | [998](../../src/transport.rs#L998) | receiver-type-required |
| `request` | `Ok` | [1000](../../src/transport.rs#L1000) | external-constructor-callback-or-unresolved |
| `request` | `serde_json::from_slice::<serde_json::Value>` | [1008](../../src/transport.rs#L1008) | external-constructor-callback-or-unresolved |
| `request` | `error["message"].as_str().unwrap_or_default` | [1010](../../src/transport.rs#L1010) | receiver-type-required |
| `request` | `error["message"].as_str` | [1010](../../src/transport.rs#L1010) | receiver-type-required |
| `request` | `error["code"].as_i64` | [1011](../../src/transport.rs#L1011) | receiver-type-required |
| `request` | `Some` | [1011](../../src/transport.rs#L1011), [1030](../../src/transport.rs#L1030), [1054](../../src/transport.rs#L1054) | external-constructor-callback-or-unresolved |
| `request` | `message.to_ascii_lowercase().contains` | [1012](../../src/transport.rs#L1012) | receiver-type-required |
| `request` | `message.to_ascii_lowercase` | [1012](../../src/transport.rs#L1012) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter(&#124;data&#124; !data.is_null())                                 .and_then(&#124;data&#124; serde_json::to_vec(data).ok())                                 .and_then` | [1014](../../src/transport.rs#L1014) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter(&#124;data&#124; !data.is_null())                                 .and_then` | [1014](../../src/transport.rs#L1014) | receiver-type-required |
| `request` | `error                                 .get("data")                                 .filter` | [1014](../../src/transport.rs#L1014) | receiver-type-required |
| `request` | `error                                 .get` | [1014](../../src/transport.rs#L1014) | receiver-type-required |
| `request` | `data.is_null` | [1016](../../src/transport.rs#L1016) | receiver-type-required |
| `request` | `serde_json::to_vec(data).ok` | [1017](../../src/transport.rs#L1017) | receiver-type-required |
| `request` | `serde_json::to_vec` | [1017](../../src/transport.rs#L1017) | external-constructor-callback-or-unresolved |
| `request` | `IJsonValue::parse(&bytes).ok` | [1018](../../src/transport.rs#L1018) | receiver-type-required |
| `request` | `IJsonValue::parse` | [1018](../../src/transport.rs#L1018) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `request` | `message.to_owned` | [1021](../../src/transport.rs#L1021) | receiver-type-required |
| `request` | `response.headers().get` | [1029](../../src/transport.rs#L1029) | receiver-type-required |
| `request` | `response.headers` | [1029](../../src/transport.rs#L1029) | receiver-type-required |
| `request` | `session_id.clone` | [1030](../../src/transport.rs#L1030) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then(&#124;value&#124; value.to_str().ok())                 .unwrap_or("")                 .to_owned` | [1032](../../src/transport.rs#L1032) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then(&#124;value&#124; value.to_str().ok())                 .unwrap_or` | [1032](../../src/transport.rs#L1032) | receiver-type-required |
| `request` | `response                 .headers()                 .get(CONTENT_TYPE)                 .and_then` | [1032](../../src/transport.rs#L1032) | receiver-type-required |
| `request` | `response                 .headers()                 .get` | [1032](../../src/transport.rs#L1032) | receiver-type-required |
| `request` | `response                 .headers` | [1032](../../src/transport.rs#L1032) | receiver-type-required |
| `request` | `value.to_str().ok` | [1035](../../src/transport.rs#L1035) | receiver-type-required |
| `request` | `value.to_str` | [1035](../../src/transport.rs#L1035) | receiver-type-required |
| `request` | `response                 .content_length()                 .is_some_and` | [1038](../../src/transport.rs#L1038) | receiver-type-required |
| `request` | `response                 .content_length` | [1038](../../src/transport.rs#L1038) | receiver-type-required |
| `request` | `"HTTP result exceeds 4 MiB".to_owned` | [1042](../../src/transport.rs#L1042), [1089](../../src/transport.rs#L1089) | receiver-type-required |
| `request` | `content_type.split(';').next().map(str::trim).unwrap_or` | [1044](../../src/transport.rs#L1044) | receiver-type-required |
| `request` | `content_type.split(';').next().map` | [1044](../../src/transport.rs#L1044) | receiver-type-required |
| `request` | `content_type.split(';').next` | [1044](../../src/transport.rs#L1044) | receiver-type-required |
| `request` | `content_type.split` | [1044](../../src/transport.rs#L1044) | receiver-type-required |
| `request` | `SseDecoder::default` | [1047](../../src/transport.rs#L1047) | external-constructor-callback-or-unresolved |
| `request` | `chunk.map_err` | [1050](../../src/transport.rs#L1050), [1087](../../src/transport.rs#L1087) | receiver-type-required |
| `request` | `decoder.push` | [1051](../../src/transport.rs#L1051) | receiver-type-required |
| `request` | `decode_inbound` | [1052](../../src/transport.rs#L1052) | [mcp::transport::decode_inbound](../../src/transport.rs#L1269) |
| `request` | `BTreeSet::new` | [1052](../../src/transport.rs#L1052) | external-constructor-callback-or-unresolved |
| `request` | `serde_json::from_slice(&payload).map_err` | [1060](../../src/transport.rs#L1060) | receiver-type-required |
| `request` | `serde_json::from_slice` | [1060](../../src/transport.rs#L1060) | external-constructor-callback-or-unresolved |
| `request` | `handler` | [1063](../../src/transport.rs#L1063), [1066](../../src/transport.rs#L1066) | external-constructor-callback-or-unresolved |
| `request` | `self.notifications.push` | [1068](../../src/transport.rs#L1068) | receiver-type-required |
| `request` | `result.is_some` | [1074](../../src/transport.rs#L1074) | receiver-type-required |
| `request` | `result.ok_or_else` | [1078](../../src/transport.rs#L1078) | receiver-type-required |
| `request` | `"SSE contains no complete matching response event".to_owned` | [1080](../../src/transport.rs#L1080) | receiver-type-required |
| `request` | `bytes.len().saturating_add` | [1088](../../src/transport.rs#L1088) | receiver-type-required |
| `request` | `decode_response` | [1093](../../src/transport.rs#L1093) | [mcp::transport::decode_response](../../src/transport.rs#L1260) |
| `request` | `self.scoped_generation.cancel` | [1100](../../src/transport.rs#L1100) | receiver-type-required |
| `notify` | `Box::pin` | [1109](../../src/transport.rs#L1109) | external-constructor-callback-or-unresolved |
| `notify` | `notification.id.is_some` | [1110](../../src/transport.rs#L1110) | receiver-type-required |
| `notify` | `Err` | [1111](../../src/transport.rs#L1111), [1129](../../src/transport.rs#L1129) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Protocol` | [1111](../../src/transport.rs#L1111) | external-constructor-callback-or-unresolved |
| `notify` | `"notification has an id".to_owned` | [1111](../../src/transport.rs#L1111) | receiver-type-required |
| `notify` | `self.request_headers` | [1113](../../src/transport.rs#L1113) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json(notification)                 .send()                 .await                 .map_err` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json(notification)                 .send` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout(Duration::from_secs(15))                 .json` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers(headers)                 .timeout` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self                 .client                 .post(self.url.clone())                 .headers` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self                 .client                 .post` | [1114](../../src/transport.rs#L1114) | receiver-type-required |
| `notify` | `self.url.clone` | [1116](../../src/transport.rs#L1116) | receiver-type-required |
| `notify` | `Duration::from_secs` | [1118](../../src/transport.rs#L1118) | external-constructor-callback-or-unresolved |
| `notify` | `McpError::Transport` | [1122](../../src/transport.rs#L1122), [1129](../../src/transport.rs#L1129) | external-constructor-callback-or-unresolved |
| `notify` | `error.to_string` | [1122](../../src/transport.rs#L1122) | receiver-type-required |
| `notify` | `response.headers().get` | [1123](../../src/transport.rs#L1123) | receiver-type-required |
| `notify` | `response.headers` | [1123](../../src/transport.rs#L1123) | receiver-type-required |
| `notify` | `Some` | [1124](../../src/transport.rs#L1124) | external-constructor-callback-or-unresolved |
| `notify` | `session_id.clone` | [1124](../../src/transport.rs#L1124) | receiver-type-required |
| `notify` | `response.status().is_success` | [1126](../../src/transport.rs#L1126) | receiver-type-required |
| `notify` | `response.status` | [1126](../../src/transport.rs#L1126) | receiver-type-required |
| `notify` | `Ok` | [1127](../../src/transport.rs#L1127) | external-constructor-callback-or-unresolved |
| `close` | `self.session_id.is_some` | [1140](../../src/transport.rs#L1140) | receiver-type-required |
| `close` | `self.request_headers(&JsonRpcRequest::notification("", None))                 .ok()                 .filter` | [1141](../../src/transport.rs#L1141) | receiver-type-required |
| `close` | `self.request_headers(&JsonRpcRequest::notification("", None))                 .ok` | [1141](../../src/transport.rs#L1141) | receiver-type-required |
| `close` | `self.request_headers` | [1141](../../src/transport.rs#L1141) | receiver-type-required |
| `close` | `JsonRpcRequest::notification` | [1141](../../src/transport.rs#L1141) | [mcp::types::JsonRpcRequest::notification](../../src/types.rs#L66) |
| `close` | `headers.contains_key` | [1143](../../src/transport.rs#L1143) | receiver-type-required |
| `close` | `self.scoped_generation.cancel` | [1147](../../src/transport.rs#L1147) | receiver-type-required |
| `close` | `Box::pin` | [1151](../../src/transport.rs#L1151) | external-constructor-callback-or-unresolved |
| `close` | `headers.insert` | [1153](../../src/transport.rs#L1153) | receiver-type-required |
| `close` | `HeaderName::from_static` | [1154](../../src/transport.rs#L1154) | external-constructor-callback-or-unresolved |
| `close` | `HeaderValue::from_static` | [1155](../../src/transport.rs#L1155) | external-constructor-callback-or-unresolved |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers(headers)                     .timeout(Duration::from_secs(15))                     .send` | [1159](../../src/transport.rs#L1159) | receiver-type-required |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers(headers)                     .timeout` | [1159](../../src/transport.rs#L1159) | receiver-type-required |
| `close` | `self                     .client                     .delete(self.url.clone())                     .headers` | [1159](../../src/transport.rs#L1159) | receiver-type-required |
| `close` | `self                     .client                     .delete` | [1159](../../src/transport.rs#L1159) | receiver-type-required |
| `close` | `self.url.clone` | [1161](../../src/transport.rs#L1161) | receiver-type-required |
| `close` | `Duration::from_secs` | [1163](../../src/transport.rs#L1163) | external-constructor-callback-or-unresolved |
| `close` | `Ok` | [1167](../../src/transport.rs#L1167) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.scoped_generation.cancel` | [1172](../../src/transport.rs#L1172) | receiver-type-required |
| `reconnect` | `crate::McpCancellationToken::default` | [1173](../../src/transport.rs#L1173) | external-constructor-callback-or-unresolved |
| `reconnect` | `self.notifications.clear` | [1177](../../src/transport.rs#L1177) | receiver-type-required |
| `reconnect` | `Box::pin` | [1178](../../src/transport.rs#L1178) | external-constructor-callback-or-unresolved |
| `reconnect` | `Ok` | [1178](../../src/transport.rs#L1178) | external-constructor-callback-or-unresolved |
| `take_notifications` | `std::mem::take` | [1182](../../src/transport.rs#L1182) | external-constructor-callback-or-unresolved |
| `push` | `self.pending.len().saturating_add` | [1195](../../src/transport.rs#L1195) | receiver-type-required |
| `push` | `self.pending.len` | [1195](../../src/transport.rs#L1195) | receiver-type-required |
| `push` | `chunk.len` | [1195](../../src/transport.rs#L1195) | receiver-type-required |
| `push` | `Err` | [1196](../../src/transport.rs#L1196), [1211](../../src/transport.rs#L1211), [1229](../../src/transport.rs#L1229) | external-constructor-callback-or-unresolved |
| `push` | `McpError::Protocol` | [1196](../../src/transport.rs#L1196), [1211](../../src/transport.rs#L1211), [1229](../../src/transport.rs#L1229) | external-constructor-callback-or-unresolved |
| `push` | `"SSE event exceeds 4 MiB".to_owned` | [1196](../../src/transport.rs#L1196), [1211](../../src/transport.rs#L1211), [1229](../../src/transport.rs#L1229) | receiver-type-required |
| `push` | `self.pending.extend_from_slice` | [1198](../../src/transport.rs#L1198) | receiver-type-required |
| `push` | `Vec::new` | [1199](../../src/transport.rs#L1199) | external-constructor-callback-or-unresolved |
| `push` | `self.pending.iter().position` | [1200](../../src/transport.rs#L1200) | receiver-type-required |
| `push` | `self.pending.iter` | [1200](../../src/transport.rs#L1200) | receiver-type-required |
| `push` | `self.pending.drain(..=newline).collect::<Vec<_>>` | [1201](../../src/transport.rs#L1201) | receiver-type-required |
| `push` | `self.pending.drain` | [1201](../../src/transport.rs#L1201) | receiver-type-required |
| `push` | `line.pop` | [1202](../../src/transport.rs#L1202), [1204](../../src/transport.rs#L1204) | receiver-type-required |
| `push` | `line.last` | [1203](../../src/transport.rs#L1203) | receiver-type-required |
| `push` | `Some` | [1203](../../src/transport.rs#L1203) | external-constructor-callback-or-unresolved |
| `push` | `line.is_empty` | [1206](../../src/transport.rs#L1206) | receiver-type-required |
| `push` | `self.data.is_empty` | [1207](../../src/transport.rs#L1207) | receiver-type-required |
| `push` | `self.data.iter().map(Vec::len).sum::<usize>` | [1208](../../src/transport.rs#L1208) | receiver-type-required |
| `push` | `self.data.iter().map` | [1208](../../src/transport.rs#L1208) | receiver-type-required |
| `push` | `self.data.iter` | [1208](../../src/transport.rs#L1208) | receiver-type-required |
| `push` | `self.data.len().saturating_sub` | [1209](../../src/transport.rs#L1209) | receiver-type-required |
| `push` | `self.data.len` | [1209](../../src/transport.rs#L1209) | receiver-type-required |
| `push` | `Vec::with_capacity` | [1213](../../src/transport.rs#L1213) | external-constructor-callback-or-unresolved |
| `push` | `self.data.drain(..).enumerate` | [1214](../../src/transport.rs#L1214) | receiver-type-required |
| `push` | `self.data.drain` | [1214](../../src/transport.rs#L1214) | receiver-type-required |
| `push` | `payload.push` | [1216](../../src/transport.rs#L1216) | receiver-type-required |
| `push` | `payload.extend_from_slice` | [1218](../../src/transport.rs#L1218) | receiver-type-required |
| `push` | `events.push` | [1221](../../src/transport.rs#L1221) | receiver-type-required |
| `push` | `line.strip_prefix` | [1225](../../src/transport.rs#L1225) | receiver-type-required |
| `push` | `value.strip_prefix(b" ").unwrap_or(value).to_vec` | [1226](../../src/transport.rs#L1226) | receiver-type-required |
| `push` | `value.strip_prefix(b" ").unwrap_or` | [1226](../../src/transport.rs#L1226) | receiver-type-required |
| `push` | `value.strip_prefix` | [1226](../../src/transport.rs#L1226) | receiver-type-required |
| `push` | `self.event_bytes.saturating_add` | [1227](../../src/transport.rs#L1227) | receiver-type-required |
| `push` | `value.len` | [1227](../../src/transport.rs#L1227) | receiver-type-required |
| `push` | `self.data.push` | [1231](../../src/transport.rs#L1231) | receiver-type-required |
| `push` | `Ok` | [1234](../../src/transport.rs#L1234) | external-constructor-callback-or-unresolved |
| `decode_response` | `decode_inbound` | [1261](../../src/transport.rs#L1261) | [mcp::transport::decode_inbound](../../src/transport.rs#L1269) |
| `decode_response` | `BTreeSet::new` | [1261](../../src/transport.rs#L1261) | external-constructor-callback-or-unresolved |
| `decode_response` | `Ok` | [1262](../../src/transport.rs#L1262) | external-constructor-callback-or-unresolved |
| `decode_response` | `Err` | [1263](../../src/transport.rs#L1263) | external-constructor-callback-or-unresolved |
| `decode_response` | `McpError::Protocol` | [1263](../../src/transport.rs#L1263) | external-constructor-callback-or-unresolved |
| `decode_response` | `"HTTP JSON response did not contain a matching response".to_owned` | [1264](../../src/transport.rs#L1264) | receiver-type-required |
| `decode_inbound` | `serde_json::from_slice(bytes).map_err` | [1275](../../src/transport.rs#L1275) | receiver-type-required |
| `decode_inbound` | `serde_json::from_slice` | [1275](../../src/transport.rs#L1275) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `McpError::Protocol` | [1275](../../src/transport.rs#L1275), [1277](../../src/transport.rs#L1277), [1283](../../src/transport.rs#L1283), [1288](../../src/transport.rs#L1288), [1296](../../src/transport.rs#L1296), [1301](../../src/transport.rs#L1301), [1315](../../src/transport.rs#L1315), [1321](../../src/transport.rs#L1321), [1335](../../src/transport.rs#L1335) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `error.to_string` | [1275](../../src/transport.rs#L1275) | receiver-type-required |
| `decode_inbound` | `Err` | [1277](../../src/transport.rs#L1277), [1283](../../src/transport.rs#L1283), [1288](../../src/transport.rs#L1288), [1296](../../src/transport.rs#L1296), [1301](../../src/transport.rs#L1301), [1315](../../src/transport.rs#L1315), [1335](../../src/transport.rs#L1335) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `"JSON-RPC frame version is not 2.0".to_owned` | [1278](../../src/transport.rs#L1278) | receiver-type-required |
| `decode_inbound` | `"response id is not a positive integer".to_owned` | [1284](../../src/transport.rs#L1284) | receiver-type-required |
| `decode_inbound` | `frame.method.is_some` | [1287](../../src/transport.rs#L1287) | receiver-type-required |
| `decode_inbound` | `"server request has no installed client handler".to_owned` | [1289](../../src/transport.rs#L1289) | receiver-type-required |
| `decode_inbound` | `cancelled_ids.remove` | [1293](../../src/transport.rs#L1293) | receiver-type-required |
| `decode_inbound` | `Ok` | [1294](../../src/transport.rs#L1294), [1312](../../src/transport.rs#L1312), [1333](../../src/transport.rs#L1333) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `frame.params.is_some` | [1300](../../src/transport.rs#L1300) | receiver-type-required |
| `decode_inbound` | `"response contains notification params".to_owned` | [1302](../../src/transport.rs#L1302) | receiver-type-required |
| `decode_inbound` | `response.validate_for` | [1311](../../src/transport.rs#L1311) | receiver-type-required |
| `decode_inbound` | `Inbound::Response` | [1312](../../src/transport.rs#L1312) | external-constructor-callback-or-unresolved |
| `decode_inbound` | `frame.result.is_some` | [1314](../../src/transport.rs#L1314) | receiver-type-required |
| `decode_inbound` | `frame.error.is_some` | [1314](../../src/transport.rs#L1314) | receiver-type-required |
| `decode_inbound` | `"id-less frame is not a notification".to_owned` | [1316](../../src/transport.rs#L1316), [1321](../../src/transport.rs#L1321) | receiver-type-required |
| `decode_inbound` | `frame         .method         .ok_or_else` | [1319](../../src/transport.rs#L1319) | receiver-type-required |
| `decode_inbound` | `Inbound::Notification` | [1333](../../src/transport.rs#L1333) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap` | [1348](../../src/transport.rs#L1348) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::net::TcpListener::bind` | [1348](../../src/transport.rs#L1348) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.local_addr().unwrap` | [1349](../../src/transport.rs#L1349) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.local_addr` | [1349](../../src/transport.rs#L1349) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Arc::new` | [1350](../../src/transport.rs#L1350) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::sync::Notify::new` | [1350](../../src/transport.rs#L1350) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `received.clone` | [1351](../../src/transport.rs#L1351) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `tokio::spawn` | [1352](../../src/transport.rs#L1352) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.accept().await.unwrap` | [1353](../../src/transport.rs#L1353) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `listener.accept` | [1353](../../src/transport.rs#L1353) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Vec::new` | [1354](../../src/transport.rs#L1354) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `socket.read(&mut chunk).await.unwrap` | [1357](../../src/transport.rs#L1357) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `socket.read` | [1357](../../src/transport.rs#L1357) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.extend_from_slice` | [1359](../../src/transport.rs#L1359) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.windows(4).position` | [1360](../../src/transport.rs#L1360) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.windows` | [1360](../../src/transport.rs#L1360) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `String::from_utf8_lossy(&bytes[..offset]).to_ascii_lowercase` | [1361](../../src/transport.rs#L1361) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `String::from_utf8_lossy` | [1361](../../src/transport.rs#L1361) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim()                             .parse()                             .unwrap` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim()                             .parse` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap()                             .trim` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map(&#124;line&#124; line.strip_prefix("content-length:"))                             .unwrap` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines()                             .find_map` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `header                             .lines` | [1362](../../src/transport.rs#L1362) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `line.strip_prefix` | [1364](../../src/transport.rs#L1364) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `bytes.len` | [1369](../../src/transport.rs#L1369) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `signal.notify_one` | [1374](../../src/transport.rs#L1374) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `HttpTransport::new(                 format!("http://{address}/mcp").parse().unwrap(),                 &BTreeMap::new(),                 None,                 true,             )             .unwrap` | [1391](../../src/transport.rs#L1391) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `HttpTransport::new` | [1391](../../src/transport.rs#L1391) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `format!("http://{address}/mcp").parse().unwrap` | [1392](../../src/transport.rs#L1392) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `format!("http://{address}/mcp").parse` | [1392](../../src/transport.rs#L1392) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `BTreeMap::new` | [1393](../../src/transport.rs#L1393) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `serde_json::from_value(serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"write","arguments":{},"_meta":{"io.modelcontextprotocol/protocolVersion":crate::MODERN_PROTOCOL_VERSION}}})).unwrap` | [1398](../../src/transport.rs#L1398) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `serde_json::from_value` | [1398](../../src/transport.rs#L1398) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `transport.start_modern_scoped` | [1399](../../src/transport.rs#L1399), [1405](../../src/transport.rs#L1405) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `request.clone` | [1400](../../src/transport.rs#L1400) | receiver-type-required |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `Duration::from_secs` | [1401](../../src/transport.rs#L1401), [1407](../../src/transport.rs#L1407) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `crate::McpCancellationToken::default` | [1402](../../src/transport.rs#L1402), [1408](../../src/transport.rs#L1408) | external-constructor-callback-or-unresolved |
| `owned_scoped_requests_cannot_outlive_close_or_reconnect` | `server.await.unwrap` | [1421](../../src/transport.rs#L1421) | receiver-type-required |
| `cancelled_response_tombstone_is_consumed_once` | `BTreeSet::from` | [1441](../../src/transport.rs#L1441) | external-constructor-callback-or-unresolved |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `SseDecoder::default` | [1463](../../src/transport.rs#L1463) | external-constructor-callback-or-unresolved |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `decoder             .push(b"\"result\":{}}\n\n: connection remains open\n")             .expect` | [1470](../../src/transport.rs#L1470) | receiver-type-required |
| `sse_decoder_yields_the_first_complete_event_without_eof` | `decoder             .push` | [1470](../../src/transport.rs#L1470) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::time::Instant::now` | [1482](../../src/transport.rs#L1482) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build()             .expect` | [1483](../../src/transport.rs#L1483) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build` | [1483](../../src/transport.rs#L1483) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread()             .enable_all` | [1483](../../src/transport.rs#L1483) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::runtime::Builder::new_current_thread` | [1483](../../src/transport.rs#L1483) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `runtime.block_on` | [1487](../../src/transport.rs#L1487) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `tokio::sync::oneshot::channel` | [1488](../../src/transport.rs#L1488) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::thread::spawn` | [1489](../../src/transport.rs#L1489) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `std::thread::sleep` | [1490](../../src/transport.rs#L1490) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `Duration::from_secs` | [1490](../../src/transport.rs#L1490) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `sender.send` | [1491](../../src/transport.rs#L1491) | receiver-type-required |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `Ok` | [1491](../../src/transport.rs#L1491) | external-constructor-callback-or-unresolved |
| `timed_out_detached_reaper_does_not_delay_runtime_drop` | `drop` | [1498](../../src/transport.rs#L1498) | external-constructor-callback-or-unresolved |
