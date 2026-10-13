# mcp::types

[Package atlas](index.md) · [Source](../../src/types.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [mcp::types::McpCancellationToken](../../src/types.rs#L11) | struct_item | `pub` |  |
| [mcp::types::McpCancellationToken::cancel](../../src/types.rs#L14) | function_item | `pub` |  |
| [mcp::types::McpCancellationToken::is_cancelled](../../src/types.rs#L19) | function_item | `pub` |  |
| [mcp::types::McpCancellationToken::cancelled](../../src/types.rs#L23) | function_item | `pub` |  |
| [mcp::types::ProtocolMode](../../src/types.rs#L32) | enum_item | `pub` |  |
| [mcp::types::TransportKind](../../src/types.rs#L40) | enum_item | `pub` |  |
| [mcp::types::JsonRpcRequest](../../src/types.rs#L47) | struct_item | `pub` |  |
| [mcp::types::JsonRpcRequest::call](../../src/types.rs#L57) | function_item | `pub` |  |
| [mcp::types::JsonRpcRequest::notification](../../src/types.rs#L66) | function_item | `pub` |  |
| [mcp::types::JsonRpcRequest::canonical_line](../../src/types.rs#L75) | function_item | `pub` |  |
| [mcp::types::JsonRpcError](../../src/types.rs#L88) | struct_item | `pub` |  |
| [mcp::types::JsonRpcResponse](../../src/types.rs#L97) | struct_item | `pub` |  |
| [mcp::types::JsonRpcResponse::validate_for](../../src/types.rs#L107) | function_item | `pub` |  |
| [mcp::types::JsonRpcResponse::into_result](../../src/types.rs#L121) | function_item | `pub` |  |
| [mcp::types::McpImplementation](../../src/types.rs#L135) | struct_item | `pub` |  |
| [mcp::types::McpIcon](../../src/types.rs#L149) | struct_item | `pub` |  |
| [mcp::types::McpIconTheme](../../src/types.rs#L161) | enum_item | `pub` |  |
| [mcp::types::RawMcpImplementation](../../src/types.rs#L168) | struct_item | `private` |  |
| [mcp::types::RawMcpIcon](../../src/types.rs#L180) | struct_item | `private` |  |
| [mcp::types::McpImplementation::deserialize](../../src/types.rs#L189) | function_item | `private` |  |
| [mcp::types::McpIcon::deserialize](../../src/types.rs#L227) | function_item | `private` |  |
| [mcp::types::validate_metadata_text](../../src/types.rs#L267) | function_item | `private` |  |
| [mcp::types::validate_http_url](../../src/types.rs#L274) | function_item | `private` |  |
| [mcp::types::valid_icon_size](../../src/types.rs#L285) | function_item | `private` |  |
| [mcp::types::McpCapabilities](../../src/types.rs#L298) | struct_item | `pub` |  |
| [mcp::types::TASKS_EXTENSION](../../src/types.rs#L312) | const_item | `pub` |  |
| [mcp::types::McpCapabilities::supports_tasks](../../src/types.rs#L317) | function_item | `pub` |  |
| [mcp::types::McpCapabilities::supports_task_extension](../../src/types.rs#L322) | function_item | `pub` |  |
| [mcp::types::McpCapabilities::serialize](../../src/types.rs#L330) | function_item | `private` |  |
| [mcp::types::McpCapabilities::deserialize](../../src/types.rs#L385) | function_item | `private` |  |
| [mcp::types::McpCapabilities::deserialize::WireCapabilities](../../src/types.rs#L391) | struct_item | `private` |  |
| [mcp::types::McpToolAnnotations](../../src/types.rs#L437) | struct_item | `pub` |  |
| [mcp::types::McpTaskSupport](../../src/types.rs#L447) | enum_item | `pub` |  |
| [mcp::types::McpToolExecution](../../src/types.rs#L455) | struct_item | `pub` |  |
| [mcp::types::McpTool](../../src/types.rs#L461) | struct_item | `pub` |  |
| [mcp::types::McpTool::requires_task](../../src/types.rs#L479) | function_item | `pub` |  |
| [mcp::types::McpTool::deserialize](../../src/types.rs#L487) | function_item | `private` |  |
| [mcp::types::McpTool::deserialize::WireTool](../../src/types.rs#L490) | struct_item | `private` |  |
| [mcp::types::McpTool::deserialize::WireExecution](../../src/types.rs#L511) | struct_item | `private` |  |
| [mcp::types::McpToolCallContext](../../src/types.rs#L533) | struct_item | `pub` |  |
| [mcp::types::McpPrompt](../../src/types.rs#L539) | struct_item | `pub` |  |
| [mcp::types::McpResource](../../src/types.rs#L547) | struct_item | `pub` |  |
| [mcp::types::McpTask](../../src/types.rs#L556) | struct_item | `pub` |  |
| [mcp::types::McpTask::validate_result](../../src/types.rs#L564) | function_item | `pub` |  |
| [mcp::types::McpToolContinuation](../../src/types.rs#L633) | struct_item | `pub` |  |
| [mcp::types::McpContent](../../src/types.rs#L643) | enum_item | `pub` |  |
| [mcp::types::McpError](../../src/types.rs#L662) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `SerializeMap` | `serde::ser::SerializeMap` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Deserializer` | `serde::Deserializer` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Serializer` | `serde::Serializer` | `private` |
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 5 direct edges</summary>

```mermaid
flowchart TD
  n0["mcp::types::JsonRpcResponse::validate_for"]
  n1["mcp::types::JsonRpcResponse::into_result"]
  n2["mcp::types::McpCancellationToken::cancel"]
  n3["mcp::types::McpImplementation::deserialize"]
  n4["mcp::types::McpCancellationToken::is_cancelled"]
  n5["mcp::types::McpIcon::deserialize"]
  n6["mcp::types::McpCancellationToken::cancelled"]
  n7["mcp::types::validate_metadata_text"]
  n8["mcp::types::validate_http_url"]
  n9["mcp::types::valid_icon_size"]
  n10["mcp::types::McpCapabilities::supports_tasks"]
  n11["mcp::types::McpCapabilities::supports_task_extension"]
  n12["mcp::types::McpCapabilities::serialize"]
  n13["mcp::types::McpCapabilities::deserialize"]
  n14["mcp::types::McpTool::requires_task"]
  n15["mcp::types::McpTool::deserialize"]
  n16["mcp::types::McpTask::validate_result"]
  n17["mcp::types::JsonRpcRequest::call"]
  n18["mcp::types::JsonRpcRequest::notification"]
  n19["mcp::types::JsonRpcRequest::canonical_line"]
  n3 --> n7
  n3 --> n8
  n5 --> n9
  n6 --> n4
  n10 --> n11
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `cancel` | `self.0.store` | [15](../../src/types.rs#L15) | receiver-type-required |
| `is_cancelled` | `self.0.load` | [20](../../src/types.rs#L20) | receiver-type-required |
| `cancelled` | `self.is_cancelled` | [24](../../src/types.rs#L24) | [mcp::types::McpCancellationToken::is_cancelled](../../src/types.rs#L19) |
| `cancelled` | `tokio::time::sleep` | [25](../../src/types.rs#L25) | external-constructor-callback-or-unresolved |
| `cancelled` | `std::time::Duration::from_millis` | [25](../../src/types.rs#L25) | external-constructor-callback-or-unresolved |
| `call` | `"2.0".to_owned` | [59](../../src/types.rs#L59) | receiver-type-required |
| `call` | `Some` | [60](../../src/types.rs#L60) | external-constructor-callback-or-unresolved |
| `call` | `method.into` | [61](../../src/types.rs#L61) | receiver-type-required |
| `notification` | `"2.0".to_owned` | [68](../../src/types.rs#L68) | receiver-type-required |
| `notification` | `method.into` | [70](../../src/types.rs#L70) | receiver-type-required |
| `canonical_line` | `self.method.is_empty` | [76](../../src/types.rs#L76) | receiver-type-required |
| `canonical_line` | `Some` | [76](../../src/types.rs#L76) | external-constructor-callback-or-unresolved |
| `canonical_line` | `Err` | [77](../../src/types.rs#L77) | external-constructor-callback-or-unresolved |
| `canonical_line` | `McpError::Protocol` | [77](../../src/types.rs#L77), [80](../../src/types.rs#L80) | external-constructor-callback-or-unresolved |
| `canonical_line` | `"invalid JSON-RPC request".to_owned` | [77](../../src/types.rs#L77) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec(self)             .map_err` | [79](../../src/types.rs#L79) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec` | [79](../../src/types.rs#L79) | external-constructor-callback-or-unresolved |
| `canonical_line` | `error.to_string` | [80](../../src/types.rs#L80) | receiver-type-required |
| `canonical_line` | `bytes.push` | [81](../../src/types.rs#L81) | receiver-type-required |
| `canonical_line` | `Ok` | [82](../../src/types.rs#L82) | external-constructor-callback-or-unresolved |
| `validate_for` | `Err` | [109](../../src/types.rs#L109), [115](../../src/types.rs#L115) | external-constructor-callback-or-unresolved |
| `validate_for` | `McpError::Protocol` | [109](../../src/types.rs#L109), [115](../../src/types.rs#L115) | external-constructor-callback-or-unresolved |
| `validate_for` | `"response id/version mismatch".to_owned` | [110](../../src/types.rs#L110) | receiver-type-required |
| `validate_for` | `self.result.is_some` | [113](../../src/types.rs#L113) | receiver-type-required |
| `validate_for` | `self.error.is_some` | [113](../../src/types.rs#L113) | receiver-type-required |
| `validate_for` | `Ok` | [114](../../src/types.rs#L114) | external-constructor-callback-or-unresolved |
| `validate_for` | `"response must contain exactly result or error".to_owned` | [116](../../src/types.rs#L116) | receiver-type-required |
| `into_result` | `Err` | [123](../../src/types.rs#L123) | external-constructor-callback-or-unresolved |
| `into_result` | `self.result             .ok_or_else` | [129](../../src/types.rs#L129) | receiver-type-required |
| `into_result` | `McpError::Protocol` | [130](../../src/types.rs#L130) | external-constructor-callback-or-unresolved |
| `into_result` | `"response result is absent".to_owned` | [130](../../src/types.rs#L130) | receiver-type-required |
| `deserialize` | `RawMcpImplementation::deserialize` | [193](../../src/types.rs#L193) | external-constructor-callback-or-unresolved |
| `deserialize` | `validate_metadata_text(&raw.name, 128, "implementation name")             .map_err` | [194](../../src/types.rs#L194) | receiver-type-required |
| `deserialize` | `validate_metadata_text` | [194](../../src/types.rs#L194), [196](../../src/types.rs#L196), [199](../../src/types.rs#L199), [203](../../src/types.rs#L203) | [mcp::types::validate_metadata_text](../../src/types.rs#L267) |
| `deserialize` | `validate_metadata_text(&raw.version, 128, "implementation version")             .map_err` | [196](../../src/types.rs#L196) | receiver-type-required |
| `deserialize` | `validate_metadata_text(title, 256, "implementation title")                 .map_err` | [199](../../src/types.rs#L199) | receiver-type-required |
| `deserialize` | `validate_metadata_text(description, 2_048, "implementation description")                 .map_err` | [203](../../src/types.rs#L203) | receiver-type-required |
| `deserialize` | `raw.icons.as_ref().is_some_and` | [206](../../src/types.rs#L206) | receiver-type-required |
| `deserialize` | `raw.icons.as_ref` | [206](../../src/types.rs#L206) | receiver-type-required |
| `deserialize` | `icons.len` | [206](../../src/types.rs#L206) | receiver-type-required |
| `deserialize` | `Err` | [207](../../src/types.rs#L207) | external-constructor-callback-or-unresolved |
| `deserialize` | `serde::de::Error::custom` | [207](../../src/types.rs#L207) | external-constructor-callback-or-unresolved |
| `deserialize` | `validate_http_url(website_url, 2_048, "implementation websiteUrl")                 .map_err` | [212](../../src/types.rs#L212) | receiver-type-required |
| `deserialize` | `validate_http_url` | [212](../../src/types.rs#L212) | [mcp::types::validate_http_url](../../src/types.rs#L274) |
| `deserialize` | `Ok` | [215](../../src/types.rs#L215) | external-constructor-callback-or-unresolved |
| `deserialize` | `RawMcpIcon::deserialize` | [231](../../src/types.rs#L231) | external-constructor-callback-or-unresolved |
| `deserialize` | `raw.src.len` | [232](../../src/types.rs#L232) | receiver-type-required |
| `deserialize` | `Err` | [233](../../src/types.rs#L233), [237](../../src/types.rs#L237), [243](../../src/types.rs#L243), [250](../../src/types.rs#L250), [256](../../src/types.rs#L256) | external-constructor-callback-or-unresolved |
| `deserialize` | `serde::de::Error::custom` | [233](../../src/types.rs#L233), [237](../../src/types.rs#L237), [243](../../src/types.rs#L243), [250](../../src/types.rs#L250), [256](../../src/types.rs#L256) | external-constructor-callback-or-unresolved |
| `deserialize` | `url::Url::parse(&raw.src).map_err` | [235](../../src/types.rs#L235) | receiver-type-required |
| `deserialize` | `url::Url::parse` | [235](../../src/types.rs#L235) | external-constructor-callback-or-unresolved |
| `deserialize` | `mime_type.split_once` | [242](../../src/types.rs#L242) | receiver-type-required |
| `deserialize` | `mime_type.len` | [245](../../src/types.rs#L245) | receiver-type-required |
| `deserialize` | `kind.is_empty` | [246](../../src/types.rs#L246) | receiver-type-required |
| `deserialize` | `subtype.is_empty` | [247](../../src/types.rs#L247) | receiver-type-required |
| `deserialize` | `mime_type.chars().any` | [248](../../src/types.rs#L248) | receiver-type-required |
| `deserialize` | `mime_type.chars` | [248](../../src/types.rs#L248) | receiver-type-required |
| `deserialize` | `sizes.len` | [254](../../src/types.rs#L254) | receiver-type-required |
| `deserialize` | `sizes.iter().all` | [254](../../src/types.rs#L254) | receiver-type-required |
| `deserialize` | `sizes.iter` | [254](../../src/types.rs#L254) | receiver-type-required |
| `deserialize` | `valid_icon_size` | [254](../../src/types.rs#L254) | [mcp::types::valid_icon_size](../../src/types.rs#L285) |
| `deserialize` | `Ok` | [258](../../src/types.rs#L258) | external-constructor-callback-or-unresolved |
| `validate_metadata_text` | `value.is_empty` | [268](../../src/types.rs#L268) | receiver-type-required |
| `validate_metadata_text` | `value.len` | [268](../../src/types.rs#L268) | receiver-type-required |
| `validate_metadata_text` | `value.chars().any` | [268](../../src/types.rs#L268) | receiver-type-required |
| `validate_metadata_text` | `value.chars` | [268](../../src/types.rs#L268) | receiver-type-required |
| `validate_metadata_text` | `Err` | [269](../../src/types.rs#L269) | external-constructor-callback-or-unresolved |
| `validate_metadata_text` | `Ok` | [271](../../src/types.rs#L271) | external-constructor-callback-or-unresolved |
| `validate_http_url` | `value.len` | [275](../../src/types.rs#L275) | receiver-type-required |
| `validate_http_url` | `Err` | [276](../../src/types.rs#L276), [280](../../src/types.rs#L280) | external-constructor-callback-or-unresolved |
| `validate_http_url` | `url::Url::parse(value).map_err` | [278](../../src/types.rs#L278) | receiver-type-required |
| `validate_http_url` | `url::Url::parse` | [278](../../src/types.rs#L278) | external-constructor-callback-or-unresolved |
| `validate_http_url` | `parsed.host_str().is_none` | [279](../../src/types.rs#L279) | receiver-type-required |
| `validate_http_url` | `parsed.host_str` | [279](../../src/types.rs#L279) | receiver-type-required |
| `validate_http_url` | `Ok` | [282](../../src/types.rs#L282) | external-constructor-callback-or-unresolved |
| `valid_icon_size` | `value.split_once` | [289](../../src/types.rs#L289) | receiver-type-required |
| `valid_icon_size` | `[width, height]         .iter()         .all` | [292](../../src/types.rs#L292) | receiver-type-required |
| `valid_icon_size` | `[width, height]         .iter` | [292](../../src/types.rs#L292) | receiver-type-required |
| `valid_icon_size` | `dimension.parse::<u32>().is_ok_and` | [294](../../src/types.rs#L294) | receiver-type-required |
| `valid_icon_size` | `dimension.parse::<u32>` | [294](../../src/types.rs#L294) | receiver-type-required |
| `supports_tasks` | `self.supports_task_extension` | [318](../../src/types.rs#L318) | [mcp::types::McpCapabilities::supports_task_extension](../../src/types.rs#L322) |
| `supports_task_extension` | `self.extensions             .get(TASKS_EXTENSION)             .is_some_and` | [323](../../src/types.rs#L323) | receiver-type-required |
| `supports_task_extension` | `self.extensions             .get` | [323](../../src/types.rs#L323) | receiver-type-required |
| `serialize` | `[             self.tools,             self.prompts,             self.resources,             self.tasks,             self.subscriptions,         ]         .into_iter()         .filter(&#124;enabled&#124; *enabled)         .count` | [334](../../src/types.rs#L334) | receiver-type-required |
| `serialize` | `[             self.tools,             self.prompts,             self.resources,             self.tasks,             self.subscriptions,         ]         .into_iter()         .filter` | [334](../../src/types.rs#L334) | receiver-type-required |
| `serialize` | `[             self.tools,             self.prompts,             self.resources,             self.tasks,             self.subscriptions,         ]         .into_iter` | [334](../../src/types.rs#L334) | receiver-type-required |
| `serialize` | `serializer.serialize_map` | [345](../../src/types.rs#L345) | receiver-type-required |
| `serialize` | `Some` | [345](../../src/types.rs#L345) | external-constructor-callback-or-unresolved |
| `serialize` | `usize::from` | [345](../../src/types.rs#L345) | external-constructor-callback-or-unresolved |
| `serialize` | `self.extensions.is_empty` | [345](../../src/types.rs#L345), [377](../../src/types.rs#L377) | receiver-type-required |
| `serialize` | `BTreeMap::<String, IJsonValue>::new` | [346](../../src/types.rs#L346) | external-constructor-callback-or-unresolved |
| `serialize` | `BTreeMap::new` | [348](../../src/types.rs#L348), [355](../../src/types.rs#L355), [362](../../src/types.rs#L362) | external-constructor-callback-or-unresolved |
| `serialize` | `detail.insert` | [350](../../src/types.rs#L350), [357](../../src/types.rs#L357), [364](../../src/types.rs#L364), [367](../../src/types.rs#L367) | receiver-type-required |
| `serialize` | `map.serialize_entry` | [352](../../src/types.rs#L352), [359](../../src/types.rs#L359), [369](../../src/types.rs#L369), [372](../../src/types.rs#L372), [375](../../src/types.rs#L375), [378](../../src/types.rs#L378) | receiver-type-required |
| `serialize` | `map.end` | [380](../../src/types.rs#L380) | receiver-type-required |
| `deserialize` | `WireCapabilities::deserialize` | [409](../../src/types.rs#L409) | external-constructor-callback-or-unresolved |
| `deserialize` | `object.as_ref().and_then` | [412](../../src/types.rs#L412) | receiver-type-required |
| `deserialize` | `object.as_ref` | [412](../../src/types.rs#L412) | receiver-type-required |
| `deserialize` | `object.get` | [412](../../src/types.rs#L412) | receiver-type-required |
| `deserialize` | `Ok` | [413](../../src/types.rs#L413), [421](../../src/types.rs#L421) | external-constructor-callback-or-unresolved |
| `deserialize` | `serde_json::from_value::<bool>(                         serde_json::to_value(value).map_err(serde::de::Error::custom)?,                     )                     .map_err` | [414](../../src/types.rs#L414) | receiver-type-required |
| `deserialize` | `serde_json::from_value::<bool>` | [414](../../src/types.rs#L414) | external-constructor-callback-or-unresolved |
| `deserialize` | `serde_json::to_value(value).map_err` | [415](../../src/types.rs#L415) | receiver-type-required |
| `deserialize` | `serde_json::to_value` | [415](../../src/types.rs#L415) | external-constructor-callback-or-unresolved |
| `deserialize` | `wire.tools.is_some` | [422](../../src/types.rs#L422) | receiver-type-required |
| `deserialize` | `wire.prompts.is_some` | [423](../../src/types.rs#L423) | receiver-type-required |
| `deserialize` | `wire.resources.is_some` | [424](../../src/types.rs#L424) | receiver-type-required |
| `deserialize` | `wire.tasks.is_some` | [425](../../src/types.rs#L425) | receiver-type-required |
| `deserialize` | `wire.subscriptions.is_some` | [426](../../src/types.rs#L426) | receiver-type-required |
| `deserialize` | `flag` | [428](../../src/types.rs#L428), [429](../../src/types.rs#L429), [430](../../src/types.rs#L430), [431](../../src/types.rs#L431) | external-constructor-callback-or-unresolved |
| `requires_task` | `self.execution             .as_ref()             .is_some_and` | [480](../../src/types.rs#L480) | receiver-type-required |
| `requires_task` | `self.execution             .as_ref` | [480](../../src/types.rs#L480) | receiver-type-required |
| `deserialize` | `WireTool::deserialize` | [515](../../src/types.rs#L515) | external-constructor-callback-or-unresolved |
| `deserialize` | `Ok` | [516](../../src/types.rs#L516) | external-constructor-callback-or-unresolved |
| `deserialize` | `wire                 .execution                 .and_then(&#124;execution&#124; execution.task_support)                 .map` | [522](../../src/types.rs#L522) | receiver-type-required |
| `deserialize` | `wire                 .execution                 .and_then` | [522](../../src/types.rs#L522) | receiver-type-required |
| `validate_result` | `serde_json::to_value(value).map_err` | [566](../../src/types.rs#L566) | receiver-type-required |
| `validate_result` | `serde_json::to_value` | [566](../../src/types.rs#L566) | external-constructor-callback-or-unresolved |
| `validate_result` | `McpError::Protocol` | [566](../../src/types.rs#L566), [567](../../src/types.rs#L567) | external-constructor-callback-or-unresolved |
| `validate_result` | `error.to_string` | [566](../../src/types.rs#L566) | receiver-type-required |
| `validate_result` | `"invalid MCP task state, identity or result".into` | [567](../../src/types.rs#L567) | receiver-type-required |
| `validate_result` | `value["taskId"]             .as_str()             .filter(&#124;id&#124; !id.is_empty() && *id == expected_id)             .ok_or_else` | [568](../../src/types.rs#L568) | receiver-type-required |
| `validate_result` | `value["taskId"]             .as_str()             .filter` | [568](../../src/types.rs#L568) | receiver-type-required |
| `validate_result` | `value["taskId"]             .as_str` | [568](../../src/types.rs#L568) | receiver-type-required |
| `validate_result` | `id.is_empty` | [570](../../src/types.rs#L570) | receiver-type-required |
| `validate_result` | `value["status"]             .as_str()             .filter(&#124;status&#124; {                 matches!(                     *status,                     "working" &#124; "input_required" &#124; "completed" &#124; "cancelled" &#124; "failed"                 )             })             .ok_or_else` | [572](../../src/types.rs#L572) | receiver-type-required |
| `validate_result` | `value["status"]             .as_str()             .filter` | [572](../../src/types.rs#L572) | receiver-type-required |
| `validate_result` | `value["status"]             .as_str` | [572](../../src/types.rs#L572) | receiver-type-required |
| `validate_result` | `value[field].as_str().ok_or_else` | [582](../../src/types.rs#L582) | receiver-type-required |
| `validate_result` | `value[field].as_str` | [582](../../src/types.rs#L582) | receiver-type-required |
| `validate_result` | `chrono::DateTime::parse_from_rfc3339(timestamp).map_err` | [583](../../src/types.rs#L583) | receiver-type-required |
| `validate_result` | `chrono::DateTime::parse_from_rfc3339` | [583](../../src/types.rs#L583) | external-constructor-callback-or-unresolved |
| `validate_result` | `invalid` | [583](../../src/types.rs#L583), [591](../../src/types.rs#L591), [599](../../src/types.rs#L599), [606](../../src/types.rs#L606), [609](../../src/types.rs#L609), [612](../../src/types.rs#L612), [619](../../src/types.rs#L619), [622](../../src/types.rs#L622) | external-constructor-callback-or-unresolved |
| `validate_result` | `value.get` | [586](../../src/types.rs#L586), [608](../../src/types.rs#L608), [621](../../src/types.rs#L621) | receiver-type-required |
| `validate_result` | `number.is_null` | [587](../../src/types.rs#L587) | receiver-type-required |
| `validate_result` | `number.as_u64().is_none` | [590](../../src/types.rs#L590) | receiver-type-required |
| `validate_result` | `number.as_u64` | [590](../../src/types.rs#L590) | receiver-type-required |
| `validate_result` | `Err` | [591](../../src/types.rs#L591), [599](../../src/types.rs#L599), [606](../../src/types.rs#L606), [609](../../src/types.rs#L609), [612](../../src/types.rs#L612), [619](../../src/types.rs#L619), [622](../../src/types.rs#L622) | external-constructor-callback-or-unresolved |
| `validate_result` | `value             .get("statusMessage")             .is_some_and` | [595](../../src/types.rs#L595) | receiver-type-required |
| `validate_result` | `value             .get` | [595](../../src/types.rs#L595) | receiver-type-required |
| `validate_result` | `message.is_string` | [597](../../src/types.rs#L597) | receiver-type-required |
| `validate_result` | `value["inputRequests"]                 .as_object()                 .is_none_or` | [602](../../src/types.rs#L602) | receiver-type-required |
| `validate_result` | `value["inputRequests"]                 .as_object` | [602](../../src/types.rs#L602) | receiver-type-required |
| `validate_result` | `requests.is_empty` | [604](../../src/types.rs#L604) | receiver-type-required |
| `validate_result` | `value.get("inputRequests").is_some` | [608](../../src/types.rs#L608) | receiver-type-required |
| `validate_result` | `value["result"].is_object` | [611](../../src/types.rs#L611) | receiver-type-required |
| `validate_result` | `value["result"]                 .get("isError")                 .is_some_and` | [615](../../src/types.rs#L615) | receiver-type-required |
| `validate_result` | `value["result"]                 .get` | [615](../../src/types.rs#L615) | receiver-type-required |
| `validate_result` | `flag.is_boolean` | [617](../../src/types.rs#L617) | receiver-type-required |
| `validate_result` | `value.get("error").is_none_or` | [621](../../src/types.rs#L621) | receiver-type-required |
| `validate_result` | `Ok` | [624](../../src/types.rs#L624) | external-constructor-callback-or-unresolved |
| `validate_result` | `id.into` | [625](../../src/types.rs#L625) | receiver-type-required |
| `validate_result` | `status.into` | [626](../../src/types.rs#L626) | receiver-type-required |
