# provider::compaction_summary

[Package atlas](index.md) · [Source](../../src/compaction_summary.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [provider::compaction_summary::prepare_summary_request](../../src/compaction_summary.rs#L14) | function_item | `pub` |  |
| [provider::compaction_summary::summary_completion_artifact](../../src/compaction_summary.rs#L73) | function_item | `pub` |  |
| [provider::compaction_summary::summary_artifact_arguments](../../src/compaction_summary.rs#L101) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `ContentBlock` | `crate::ContentBlock` | `private` |
| `DialectId` | `crate::DialectId` | `private` |
| `PrepareInput` | `crate::PrepareInput` | `private` |
| `PreparedRequest` | `crate::PreparedRequest` | `private` |
| `ProviderTerminal` | `crate::ProviderTerminal` | `private` |
| `ResolvedDialectProfile` | `crate::ResolvedDialectProfile` | `private` |
| `ToolChoice` | `crate::ToolChoice` | `private` |
| `prepare_with_tool_choice` | `crate::prepare_with_tool_choice` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–3: 4 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::compaction_summary::summary_artifact_arguments"]
  n1["provider::compaction_summary::prepare_summary_request"]
  n2["provider::compaction_summary::summary_completion_artifact"]
  n3["provider::dialect::epoch_profile"]
  n4["provider::request::prepare_with_tool_choice"]
  n5["schema::ijson::IJsonValue::parse"]
  n1 --> n3
  n1 --> n4
  n1 --> n5
  n2 --> n0
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `prepare_summary_request` | `IJsonValue::parse(             &serde_json_canonicalizer::to_vec(value).map_err(&#124;error&#124; format!("{what}: {error}"))?,         )         .map_err` | [23](../../src/compaction_summary.rs#L23) | receiver-type-required |
| `prepare_summary_request` | `IJsonValue::parse` | [23](../../src/compaction_summary.rs#L23) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `prepare_summary_request` | `serde_json_canonicalizer::to_vec(value).map_err` | [24](../../src/compaction_summary.rs#L24) | receiver-type-required |
| `prepare_summary_request` | `serde_json_canonicalizer::to_vec` | [24](../../src/compaction_summary.rs#L24) | external-constructor-callback-or-unresolved |
| `prepare_summary_request` | `crate::epoch_profile(resolved, system, None)         .map_err` | [28](../../src/compaction_summary.rs#L28) | receiver-type-required |
| `prepare_summary_request` | `crate::epoch_profile` | [28](../../src/compaction_summary.rs#L28) | [provider::dialect::epoch_profile](../../src/dialect.rs#L813) |
| `prepare_summary_request` | `canonical` | [30](../../src/compaction_summary.rs#L30) | external-constructor-callback-or-unresolved |
| `prepare_summary_request` | `Some` | [43](../../src/compaction_summary.rs#L43), [50](../../src/compaction_summary.rs#L50), [52](../../src/compaction_summary.rs#L52) | external-constructor-callback-or-unresolved |
| `prepare_summary_request` | `resolved.forced_tool_choice` | [44](../../src/compaction_summary.rs#L44) | receiver-type-required |
| `prepare_summary_request` | `prepare_with_tool_choice(         &PrepareInput {             attempt_id,             target: resolved.target.clone(),             endpoint: endpoint.to_owned(),             epoch_profile,             continuation_id: None,             rendered_items: vec![rendered],             tool_catalog,             stream: false,         },         choice,     )     .map_err` | [54](../../src/compaction_summary.rs#L54) | receiver-type-required |
| `prepare_summary_request` | `prepare_with_tool_choice` | [54](../../src/compaction_summary.rs#L54) | [provider::request::prepare_with_tool_choice](../../src/request.rs#L137) |
| `prepare_summary_request` | `resolved.target.clone` | [57](../../src/compaction_summary.rs#L57) | receiver-type-required |
| `prepare_summary_request` | `endpoint.to_owned` | [58](../../src/compaction_summary.rs#L58) | receiver-type-required |
| `summary_completion_artifact` | `terminal                 .usage                 .as_ref()                 .and_then` | [78](../../src/compaction_summary.rs#L78) | receiver-type-required |
| `summary_completion_artifact` | `terminal                 .usage                 .as_ref` | [78](../../src/compaction_summary.rs#L78) | receiver-type-required |
| `summary_completion_artifact` | `serde_json::to_value(usage).ok` | [81](../../src/compaction_summary.rs#L81) | receiver-type-required |
| `summary_completion_artifact` | `serde_json::to_value` | [81](../../src/compaction_summary.rs#L81) | external-constructor-callback-or-unresolved |
| `summary_completion_artifact` | `summary_artifact_arguments(&terminal).ok_or_else` | [82](../../src/compaction_summary.rs#L82) | receiver-type-required |
| `summary_completion_artifact` | `summary_artifact_arguments` | [82](../../src/compaction_summary.rs#L82) | [provider::compaction_summary::summary_artifact_arguments](../../src/compaction_summary.rs#L101) |
| `summary_completion_artifact` | `Err` | [91](../../src/compaction_summary.rs#L91), [93](../../src/compaction_summary.rs#L93) | external-constructor-callback-or-unresolved |
| `summary_artifact_arguments` | `terminal         .tool_calls         .iter()         .find` | [102](../../src/compaction_summary.rs#L102) | receiver-type-required |
| `summary_artifact_arguments` | `terminal         .tool_calls         .iter` | [102](../../src/compaction_summary.rs#L102) | receiver-type-required |
| `summary_artifact_arguments` | `Some` | [107](../../src/compaction_summary.rs#L107), [113](../../src/compaction_summary.rs#L113) | external-constructor-callback-or-unresolved |
| `summary_artifact_arguments` | `call.arguments.clone` | [107](../../src/compaction_summary.rs#L107) | receiver-type-required |
| `summary_artifact_arguments` | `terminal         .content         .iter()         .filter_map(&#124;block&#124; match block {             ContentBlock::Text(text) => Some(text.as_str()),             ContentBlock::Reasoning(_) => None,         })         .collect::<Vec<_>>()         .join` | [109](../../src/compaction_summary.rs#L109) | receiver-type-required |
| `summary_artifact_arguments` | `terminal         .content         .iter()         .filter_map(&#124;block&#124; match block {             ContentBlock::Text(text) => Some(text.as_str()),             ContentBlock::Reasoning(_) => None,         })         .collect::<Vec<_>>` | [109](../../src/compaction_summary.rs#L109) | receiver-type-required |
| `summary_artifact_arguments` | `terminal         .content         .iter()         .filter_map` | [109](../../src/compaction_summary.rs#L109) | receiver-type-required |
| `summary_artifact_arguments` | `terminal         .content         .iter` | [109](../../src/compaction_summary.rs#L109) | receiver-type-required |
| `summary_artifact_arguments` | `text.as_str` | [113](../../src/compaction_summary.rs#L113) | receiver-type-required |
| `summary_artifact_arguments` | `text.find` | [118](../../src/compaction_summary.rs#L118) | receiver-type-required |
| `summary_artifact_arguments` | `text.rfind` | [119](../../src/compaction_summary.rs#L119) | receiver-type-required |
| `summary_artifact_arguments` | `serde_json::from_str::<Value>(&text[start..=end])         .ok()         .filter` | [123](../../src/compaction_summary.rs#L123) | receiver-type-required |
| `summary_artifact_arguments` | `serde_json::from_str::<Value>(&text[start..=end])         .ok` | [123](../../src/compaction_summary.rs#L123) | receiver-type-required |
| `summary_artifact_arguments` | `serde_json::from_str::<Value>` | [123](../../src/compaction_summary.rs#L123) | external-constructor-callback-or-unresolved |
| `summary_artifact_arguments` | `value.get("continuation").is_some` | [125](../../src/compaction_summary.rs#L125) | receiver-type-required |
| `summary_artifact_arguments` | `value.get` | [125](../../src/compaction_summary.rs#L125) | receiver-type-required |
| `summary_artifact_arguments` | `value.get("evidence_refs").is_some` | [125](../../src/compaction_summary.rs#L125) | receiver-type-required |
