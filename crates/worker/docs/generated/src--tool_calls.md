# tekes-worker::tool_calls

[Package atlas](index.md) · [Source](../../src/tool_calls.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L203) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L220) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_skill_catalog](../../src/tool_calls.rs#L252) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::ToolCallBatch](../../src/tool_calls.rs#L312) | struct_item | `pub(crate)` |  |

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
| `execute_provider_tool_calls` | `profile             .instruction             .admit_kind` | [40](../../src/tool_calls.rs#L40), [43](../../src/tool_calls.rs#L43) | receiver-type-required |
| `execute_provider_tool_calls` | `frozen_hook_bindings` | [46](../../src/tool_calls.rs#L46) | [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L220) |
| `execute_provider_tool_calls` | `cancellation.supervisor_lost` | [47](../../src/tool_calls.rs#L47), [166](../../src/tool_calls.rs#L166) | receiver-type-required |
| `execute_provider_tool_calls` | `Err` | [48](../../src/tool_calls.rs#L48), [159](../../src/tool_calls.rs#L159), [167](../../src/tool_calls.rs#L167) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `Box::new` | [48](../../src/tool_calls.rs#L48), [159](../../src/tool_calls.rs#L159), [167](../../src/tool_calls.rs#L167) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ProtocolFailure` | [48](../../src/tool_calls.rs#L48), [159](../../src/tool_calls.rs#L159), [167](../../src/tool_calls.rs#L167) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `"supervisor lost before tool execution".to_owned` | [49](../../src/tool_calls.rs#L49) | receiver-type-required |
| `execute_provider_tool_calls` | `cancellation.stop_requested` | [52](../../src/tool_calls.rs#L52), [163](../../src/tool_calls.rs#L163) | receiver-type-required |
| `execute_provider_tool_calls` | `Ok` | [53](../../src/tool_calls.rs#L53), [72](../../src/tool_calls.rs#L72), [164](../../src/tool_calls.rs#L164), [172](../../src/tool_calls.rs#L172), [196](../../src/tool_calls.rs#L196), [200](../../src/tool_calls.rs#L200) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `execute_child_spawn_tool` | [56](../../src/tool_calls.rs#L56) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `provider::invalid_arguments_detail` | [76](../../src/tool_calls.rs#L76) | [provider::normalize::invalid_arguments_detail](../../../provider/src/normalize.rs#L790) |
| `execute_provider_tool_calls` | `append_tool_validation_error` | [79](../../src/tool_calls.rs#L79), [91](../../src/tool_calls.rs#L91), [154](../../src/tool_calls.rs#L154) | [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L203) |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map(&#124;root&#124; profile.config.protected_roots(&root))             .and_then` | [82](../../src/tool_calls.rs#L82) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map` | [82](../../src/tool_calls.rs#L82) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder` | [82](../../src/tool_calls.rs#L82) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `profile.config.protected_roots` | [83](../../src/tool_calls.rs#L83) | receiver-type-required |
| `execute_provider_tool_calls` | `invocation_write_roots` | [84](../../src/tool_calls.rs#L84) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `error.to_string` | [91](../../src/tool_calls.rs#L91), [138](../../src/tool_calls.rs#L138), [148](../../src/tool_calls.rs#L148) | receiver-type-required |
| `execute_provider_tool_calls` | `assemble_tool_backends` | [95](../../src/tool_calls.rs#L95) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `IJsonValue::parse` | [110](../../src/tool_calls.rs#L110) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `serde_json::to_vec` | [110](../../src/tool_calls.rs#L110) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `thread.clone` | [112](../../src/tool_calls.rs#L112) | receiver-type-required |
| `execute_provider_tool_calls` | `attempt.to_owned` | [114](../../src/tool_calls.rs#L114) | receiver-type-required |
| `execute_provider_tool_calls` | `call.call_id.clone` | [115](../../src/tool_calls.rs#L115) | receiver-type-required |
| `execute_provider_tool_calls` | `call.name.clone` | [116](../../src/tool_calls.rs#L116) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp().clone` | [118](../../src/tool_calls.rs#L118) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp` | [118](../../src/tool_calls.rs#L118) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries(&dynamic_backend)?             .is_empty` | [120](../../src/tool_calls.rs#L120) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries` | [120](../../src/tool_calls.rs#L120) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new` | [120](../../src/tool_calls.rs#L120), [140](../../src/tool_calls.rs#L140) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `tool_catalog_context` | [123](../../src/tool_calls.rs#L123) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ToolPipeline::new` | [124](../../src/tool_calls.rs#L124) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `assets.clone` | [124](../../src/tool_calls.rs#L124) | receiver-type-required |
| `execute_provider_tool_calls` | `SecretScanner::default` | [124](../../src/tool_calls.rs#L124) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `manifest             .tools             .iter()             .any` | [125](../../src/tool_calls.rs#L125) | receiver-type-required |
| `execute_provider_tool_calls` | `manifest             .tools             .iter` | [125](../../src/tool_calls.rs#L125) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut backend,                 )                 .map_err` | [130](../../src/tool_calls.rs#L130) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch` | [130](../../src/tool_calls.rs#L130) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new` | [130](../../src/tool_calls.rs#L130) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut dynamic_backend,                 )                 .map_err` | [140](../../src/tool_calls.rs#L140) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch` | [140](../../src/tool_calls.rs#L140) | receiver-type-required |
| `execute_provider_tool_calls` | `drop` | [153](../../src/tool_calls.rs#L153), [181](../../src/tool_calls.rs#L181), [182](../../src/tool_calls.rs#L182) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `cancellation.protocol_failed` | [158](../../src/tool_calls.rs#L158) | receiver-type-required |
| `execute_provider_tool_calls` | `"tool-control protocol failure".to_owned` | [160](../../src/tool_calls.rs#L160) | receiver-type-required |
| `execute_provider_tool_calls` | `"supervisor lost during tool execution".to_owned` | [168](../../src/tool_calls.rs#L168) | receiver-type-required |
| `execute_provider_tool_calls` | `wait_tool_continuation` | [183](../../src/tool_calls.rs#L183) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `make_event` | [210](../../src/tool_calls.rs#L210) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `ledger.append_contract` | [216](../../src/tool_calls.rs#L216) | receiver-type-required |
| `append_tool_validation_error` | `BarrierContext::default` | [216](../../src/tool_calls.rs#L216) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `Ok` | [217](../../src/tool_calls.rs#L217) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective hook {logical_name:?} has an invalid index"))?;             decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err(&#124;error&#124; Box::new(error) as Box<dyn std::error::Error>)         })         .collect` | [223](../../src/tool_calls.rs#L223) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map` | [223](../../src/tool_calls.rs#L223) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter` | [223](../../src/tool_calls.rs#L223) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter` | [223](../../src/tool_calls.rs#L223) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .is_some_and` | [228](../../src/tool_calls.rs#L228) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get` | [228](../../src/tool_calls.rs#L228), [235](../../src/tool_calls.rs#L235), [242](../../src/tool_calls.rs#L242) | receiver-type-required |
| `frozen_hook_bindings` | `instruction.source_authorized` | [231](../../src/tool_calls.rs#L231) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and` | [235](../../src/tool_calls.rs#L235) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then` | [235](../../src/tool_calls.rs#L235) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>(&source.content).ok` | [238](../../src/tool_calls.rs#L238) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>` | [238](../../src/tool_calls.rs#L238) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [242](../../src/tool_calls.rs#L242) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err` | [246](../../src/tool_calls.rs#L246) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding` | [246](../../src/tool_calls.rs#L246) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `source.content.as_bytes` | [246](../../src/tool_calls.rs#L246) | receiver-type-required |
| `frozen_hook_bindings` | `Box::new` | [247](../../src/tool_calls.rs#L247) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `ResourceCatalog::from_snapshot` | [255](../../src/tool_calls.rs#L255) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map(&#124;summary&#124; {             let package = resources.skill(&summary.name)?;             let content = json!({                 "format": 1,                 "name": summary.name,                 "description": summary.description,                 "body": package.body,                 "resources": package.resources,             });             Ok(CatalogEntry {                 name: summary.name,                 summary: summary.description,                 aliases: Vec::new(),                 schema_digest: summary.content_digest,                 content: IJsonValue::parse(&serde_json::to_vec(&content)?)?,             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [256](../../src/tool_calls.rs#L256) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map` | [256](../../src/tool_calls.rs#L256) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter` | [256](../../src/tool_calls.rs#L256) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries` | [256](../../src/tool_calls.rs#L256) | receiver-type-required |
| `frozen_skill_catalog` | `resources.skill` | [260](../../src/tool_calls.rs#L260) | receiver-type-required |
| `frozen_skill_catalog` | `Ok` | [268](../../src/tool_calls.rs#L268), [298](../../src/tool_calls.rs#L298), [309](../../src/tool_calls.rs#L309) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `Vec::new` | [271](../../src/tool_calls.rs#L271), [301](../../src/tool_calls.rs#L301) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `IJsonValue::parse` | [273](../../src/tool_calls.rs#L273) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `serde_json::to_vec` | [273](../../src/tool_calls.rs#L273) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective skill {logical_name:?} has an invalid index"))?;             let summary = source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else(&#124;&#124; logical_name.clone(), &#124;line&#124; line.trim().to_owned());             Ok(CatalogEntry {                 name: logical_name.clone(),                 summary,                 aliases: Vec::new(),                 schema_digest: source.content_sha256.clone(),                 content: IJsonValue::from(source.content.clone()),             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.contains` | [282](../../src/tool_calls.rs#L282) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get(**index as usize)                     .is_some_and` | [283](../../src/tool_calls.rs#L283) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get` | [283](../../src/tool_calls.rs#L283) | receiver-type-required |
| `frozen_skill_catalog` | `instruction.source_authorized` | [286](../../src/tool_calls.rs#L286) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [289](../../src/tool_calls.rs#L289) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get` | [289](../../src/tool_calls.rs#L289) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else` | [293](../../src/tool_calls.rs#L293) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find` | [293](../../src/tool_calls.rs#L293) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines` | [293](../../src/tool_calls.rs#L293) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().is_empty` | [296](../../src/tool_calls.rs#L296) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim` | [296](../../src/tool_calls.rs#L296), [297](../../src/tool_calls.rs#L297) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.clone` | [297](../../src/tool_calls.rs#L297), [299](../../src/tool_calls.rs#L299) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().to_owned` | [297](../../src/tool_calls.rs#L297) | receiver-type-required |
| `frozen_skill_catalog` | `source.content_sha256.clone` | [302](../../src/tool_calls.rs#L302) | receiver-type-required |
| `frozen_skill_catalog` | `IJsonValue::from` | [303](../../src/tool_calls.rs#L303) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `source.content.clone` | [303](../../src/tool_calls.rs#L303) | receiver-type-required |
| `frozen_skill_catalog` | `entries.extend` | [307](../../src/tool_calls.rs#L307) | receiver-type-required |
| `frozen_skill_catalog` | `entries.sort_by` | [308](../../src/tool_calls.rs#L308) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes().cmp` | [308](../../src/tool_calls.rs#L308) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes` | [308](../../src/tool_calls.rs#L308) | receiver-type-required |
| `frozen_skill_catalog` | `right.name.as_bytes` | [308](../../src/tool_calls.rs#L308) | receiver-type-required |
