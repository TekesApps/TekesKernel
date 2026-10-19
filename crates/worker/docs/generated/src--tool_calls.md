# tekes-worker::tool_calls

[Package atlas](index.md) · [Source](../../src/tool_calls.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L201) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L218) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_skill_catalog](../../src/tool_calls.rs#L250) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::ToolCallBatch](../../src/tool_calls.rs#L310) | struct_item | `pub(crate)` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 4 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::normalize::invalid_arguments_detail"]
  n1["store::asset::AssetStore::new"]
  n2["tekes-worker::tool_calls::append_tool_validation_error"]
  n3["tekes-worker::tool_calls::frozen_hook_bindings"]
  n4["tekes-worker::tool_calls::frozen_skill_catalog"]
  n5["tekes-worker::tool_calls::execute_provider_tool_calls"]
  n5 --> n0
  n5 --> n1
  n5 --> n2
  n5 --> n3
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `execute_provider_tool_calls` | `ledger         .path()         .parent()         .ok_or("worker ledger has no thread folder")?         .to_path_buf` | [23](../../src/tool_calls.rs#L23) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .path()         .parent()         .ok_or` | [23](../../src/tool_calls.rs#L23) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .path()         .parent` | [23](../../src/tool_calls.rs#L23) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .path` | [23](../../src/tool_calls.rs#L23) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .projection()         .and_then(&#124;projection&#124; projection.events.first())         .and_then(&#124;genesis&#124; genesis.string_field("thread"))         .ok_or("genesis thread binding is missing")?         .to_owned` | [28](../../src/tool_calls.rs#L28) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .projection()         .and_then(&#124;projection&#124; projection.events.first())         .and_then(&#124;genesis&#124; genesis.string_field("thread"))         .ok_or` | [28](../../src/tool_calls.rs#L28) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .projection()         .and_then(&#124;projection&#124; projection.events.first())         .and_then` | [28](../../src/tool_calls.rs#L28) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .projection()         .and_then` | [28](../../src/tool_calls.rs#L28) | receiver-type-required |
| `execute_provider_tool_calls` | `ledger         .projection` | [28](../../src/tool_calls.rs#L28) | receiver-type-required |
| `execute_provider_tool_calls` | `projection.events.first` | [30](../../src/tool_calls.rs#L30) | receiver-type-required |
| `execute_provider_tool_calls` | `genesis.string_field` | [31](../../src/tool_calls.rs#L31) | receiver-type-required |
| `execute_provider_tool_calls` | `BuiltinManifest::compiled` | [34](../../src/tool_calls.rs#L34) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `store::AssetStore::new` | [35](../../src/tool_calls.rs#L35) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `execute_provider_tool_calls` | `thread_folder.join` | [35](../../src/tool_calls.rs#L35) | receiver-type-required |
| `execute_provider_tool_calls` | `session_permission_policy` | [38](../../src/tool_calls.rs#L38) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `execution_access_for_profile` | [40](../../src/tool_calls.rs#L40) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `frozen_hook_bindings` | [42](../../src/tool_calls.rs#L42) | [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L218) |
| `execute_provider_tool_calls` | `cancellation.supervisor_lost` | [43](../../src/tool_calls.rs#L43), [164](../../src/tool_calls.rs#L164) | receiver-type-required |
| `execute_provider_tool_calls` | `Err` | [44](../../src/tool_calls.rs#L44), [157](../../src/tool_calls.rs#L157), [165](../../src/tool_calls.rs#L165) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `Box::new` | [44](../../src/tool_calls.rs#L44), [157](../../src/tool_calls.rs#L157), [165](../../src/tool_calls.rs#L165) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ProtocolFailure` | [44](../../src/tool_calls.rs#L44), [157](../../src/tool_calls.rs#L157), [165](../../src/tool_calls.rs#L165) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `"supervisor lost before tool execution".to_owned` | [45](../../src/tool_calls.rs#L45) | receiver-type-required |
| `execute_provider_tool_calls` | `cancellation.stop_requested` | [48](../../src/tool_calls.rs#L48), [161](../../src/tool_calls.rs#L161) | receiver-type-required |
| `execute_provider_tool_calls` | `Ok` | [49](../../src/tool_calls.rs#L49), [68](../../src/tool_calls.rs#L68), [162](../../src/tool_calls.rs#L162), [170](../../src/tool_calls.rs#L170), [194](../../src/tool_calls.rs#L194), [198](../../src/tool_calls.rs#L198) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `execute_child_spawn_tool` | [52](../../src/tool_calls.rs#L52) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `provider::invalid_arguments_detail` | [72](../../src/tool_calls.rs#L72) | [provider::normalize::invalid_arguments_detail](../../../provider/src/normalize.rs#L790) |
| `execute_provider_tool_calls` | `append_tool_validation_error` | [75](../../src/tool_calls.rs#L75), [88](../../src/tool_calls.rs#L88), [152](../../src/tool_calls.rs#L152) | [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L201) |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map(&#124;root&#124; profile.config.protected_roots(&root))             .and_then` | [78](../../src/tool_calls.rs#L78) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map` | [78](../../src/tool_calls.rs#L78) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder` | [78](../../src/tool_calls.rs#L78) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `profile.config.protected_roots` | [79](../../src/tool_calls.rs#L79) | receiver-type-required |
| `execute_provider_tool_calls` | `invocation_write_roots_for_access` | [81](../../src/tool_calls.rs#L81) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `error.to_string` | [88](../../src/tool_calls.rs#L88), [136](../../src/tool_calls.rs#L136), [146](../../src/tool_calls.rs#L146) | receiver-type-required |
| `execute_provider_tool_calls` | `assemble_tool_backends` | [92](../../src/tool_calls.rs#L92) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `IJsonValue::parse` | [108](../../src/tool_calls.rs#L108) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `serde_json::to_vec` | [108](../../src/tool_calls.rs#L108) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `thread.clone` | [110](../../src/tool_calls.rs#L110) | receiver-type-required |
| `execute_provider_tool_calls` | `attempt.to_owned` | [112](../../src/tool_calls.rs#L112) | receiver-type-required |
| `execute_provider_tool_calls` | `call.call_id.clone` | [113](../../src/tool_calls.rs#L113) | receiver-type-required |
| `execute_provider_tool_calls` | `call.name.clone` | [114](../../src/tool_calls.rs#L114) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp().clone` | [116](../../src/tool_calls.rs#L116) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp` | [116](../../src/tool_calls.rs#L116) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries(&dynamic_backend)?             .is_empty` | [118](../../src/tool_calls.rs#L118) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries` | [118](../../src/tool_calls.rs#L118) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new` | [118](../../src/tool_calls.rs#L118), [138](../../src/tool_calls.rs#L138) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `tool_catalog_context` | [121](../../src/tool_calls.rs#L121) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ToolPipeline::new` | [122](../../src/tool_calls.rs#L122) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `assets.clone` | [122](../../src/tool_calls.rs#L122) | receiver-type-required |
| `execute_provider_tool_calls` | `SecretScanner::default` | [122](../../src/tool_calls.rs#L122) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `manifest             .tools             .iter()             .any` | [123](../../src/tool_calls.rs#L123) | receiver-type-required |
| `execute_provider_tool_calls` | `manifest             .tools             .iter` | [123](../../src/tool_calls.rs#L123) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut backend,                 )                 .map_err` | [128](../../src/tool_calls.rs#L128) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch` | [128](../../src/tool_calls.rs#L128) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new` | [128](../../src/tool_calls.rs#L128) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut dynamic_backend,                 )                 .map_err` | [138](../../src/tool_calls.rs#L138) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch` | [138](../../src/tool_calls.rs#L138) | receiver-type-required |
| `execute_provider_tool_calls` | `drop` | [151](../../src/tool_calls.rs#L151), [179](../../src/tool_calls.rs#L179), [180](../../src/tool_calls.rs#L180) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `cancellation.protocol_failed` | [156](../../src/tool_calls.rs#L156) | receiver-type-required |
| `execute_provider_tool_calls` | `"tool-control protocol failure".to_owned` | [158](../../src/tool_calls.rs#L158) | receiver-type-required |
| `execute_provider_tool_calls` | `"supervisor lost during tool execution".to_owned` | [166](../../src/tool_calls.rs#L166) | receiver-type-required |
| `execute_provider_tool_calls` | `wait_tool_continuation` | [181](../../src/tool_calls.rs#L181) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `make_event` | [208](../../src/tool_calls.rs#L208) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `ledger.append_contract` | [214](../../src/tool_calls.rs#L214) | receiver-type-required |
| `append_tool_validation_error` | `BarrierContext::default` | [214](../../src/tool_calls.rs#L214) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `Ok` | [215](../../src/tool_calls.rs#L215) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective hook {logical_name:?} has an invalid index"))?;             decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err(&#124;error&#124; Box::new(error) as Box<dyn std::error::Error>)         })         .collect` | [221](../../src/tool_calls.rs#L221) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map` | [221](../../src/tool_calls.rs#L221) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter` | [221](../../src/tool_calls.rs#L221) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter` | [221](../../src/tool_calls.rs#L221) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .is_some_and` | [226](../../src/tool_calls.rs#L226) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get` | [226](../../src/tool_calls.rs#L226), [233](../../src/tool_calls.rs#L233), [240](../../src/tool_calls.rs#L240) | receiver-type-required |
| `frozen_hook_bindings` | `instruction.source_authorized` | [229](../../src/tool_calls.rs#L229) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and` | [233](../../src/tool_calls.rs#L233) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then` | [233](../../src/tool_calls.rs#L233) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>(&source.content).ok` | [236](../../src/tool_calls.rs#L236) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>` | [236](../../src/tool_calls.rs#L236) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [240](../../src/tool_calls.rs#L240) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err` | [244](../../src/tool_calls.rs#L244) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding` | [244](../../src/tool_calls.rs#L244) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `source.content.as_bytes` | [244](../../src/tool_calls.rs#L244) | receiver-type-required |
| `frozen_hook_bindings` | `Box::new` | [245](../../src/tool_calls.rs#L245) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `ResourceCatalog::from_snapshot` | [253](../../src/tool_calls.rs#L253) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map(&#124;summary&#124; {             let package = resources.skill(&summary.name)?;             let content = json!({                 "format": 1,                 "name": summary.name,                 "description": summary.description,                 "body": package.body,                 "resources": package.resources,             });             Ok(CatalogEntry {                 name: summary.name,                 summary: summary.description,                 aliases: Vec::new(),                 schema_digest: summary.content_digest,                 content: IJsonValue::parse(&serde_json::to_vec(&content)?)?,             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [254](../../src/tool_calls.rs#L254) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map` | [254](../../src/tool_calls.rs#L254) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter` | [254](../../src/tool_calls.rs#L254) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries` | [254](../../src/tool_calls.rs#L254) | receiver-type-required |
| `frozen_skill_catalog` | `resources.skill` | [258](../../src/tool_calls.rs#L258) | receiver-type-required |
| `frozen_skill_catalog` | `Ok` | [266](../../src/tool_calls.rs#L266), [296](../../src/tool_calls.rs#L296), [307](../../src/tool_calls.rs#L307) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `Vec::new` | [269](../../src/tool_calls.rs#L269), [299](../../src/tool_calls.rs#L299) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `IJsonValue::parse` | [271](../../src/tool_calls.rs#L271) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `serde_json::to_vec` | [271](../../src/tool_calls.rs#L271) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective skill {logical_name:?} has an invalid index"))?;             let summary = source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else(&#124;&#124; logical_name.clone(), &#124;line&#124; line.trim().to_owned());             Ok(CatalogEntry {                 name: logical_name.clone(),                 summary,                 aliases: Vec::new(),                 schema_digest: source.content_sha256.clone(),                 content: IJsonValue::from(source.content.clone()),             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [275](../../src/tool_calls.rs#L275) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map` | [275](../../src/tool_calls.rs#L275) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter` | [275](../../src/tool_calls.rs#L275) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter` | [275](../../src/tool_calls.rs#L275) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.contains` | [280](../../src/tool_calls.rs#L280) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get(**index as usize)                     .is_some_and` | [281](../../src/tool_calls.rs#L281) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get` | [281](../../src/tool_calls.rs#L281) | receiver-type-required |
| `frozen_skill_catalog` | `instruction.source_authorized` | [284](../../src/tool_calls.rs#L284) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [287](../../src/tool_calls.rs#L287) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get` | [287](../../src/tool_calls.rs#L287) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else` | [291](../../src/tool_calls.rs#L291) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find` | [291](../../src/tool_calls.rs#L291) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines` | [291](../../src/tool_calls.rs#L291) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().is_empty` | [294](../../src/tool_calls.rs#L294) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim` | [294](../../src/tool_calls.rs#L294), [295](../../src/tool_calls.rs#L295) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.clone` | [295](../../src/tool_calls.rs#L295), [297](../../src/tool_calls.rs#L297) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().to_owned` | [295](../../src/tool_calls.rs#L295) | receiver-type-required |
| `frozen_skill_catalog` | `source.content_sha256.clone` | [300](../../src/tool_calls.rs#L300) | receiver-type-required |
| `frozen_skill_catalog` | `IJsonValue::from` | [301](../../src/tool_calls.rs#L301) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `source.content.clone` | [301](../../src/tool_calls.rs#L301) | receiver-type-required |
| `frozen_skill_catalog` | `entries.extend` | [305](../../src/tool_calls.rs#L305) | receiver-type-required |
| `frozen_skill_catalog` | `entries.sort_by` | [306](../../src/tool_calls.rs#L306) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes().cmp` | [306](../../src/tool_calls.rs#L306) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes` | [306](../../src/tool_calls.rs#L306) | receiver-type-required |
| `frozen_skill_catalog` | `right.name.as_bytes` | [306](../../src/tool_calls.rs#L306) | receiver-type-required |
