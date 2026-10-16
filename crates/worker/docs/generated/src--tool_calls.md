# tekes-worker::tool_calls

[Package atlas](index.md) · [Source](../../src/tool_calls.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L197) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L214) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::frozen_skill_catalog](../../src/tool_calls.rs#L246) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_calls::ToolCallBatch](../../src/tool_calls.rs#L306) | struct_item | `pub(crate)` |  |

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
| `execute_provider_tool_calls` | `frozen_hook_bindings` | [40](../../src/tool_calls.rs#L40) | [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L214) |
| `execute_provider_tool_calls` | `cancellation.supervisor_lost` | [41](../../src/tool_calls.rs#L41), [160](../../src/tool_calls.rs#L160) | receiver-type-required |
| `execute_provider_tool_calls` | `Err` | [42](../../src/tool_calls.rs#L42), [153](../../src/tool_calls.rs#L153), [161](../../src/tool_calls.rs#L161) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `Box::new` | [42](../../src/tool_calls.rs#L42), [153](../../src/tool_calls.rs#L153), [161](../../src/tool_calls.rs#L161) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ProtocolFailure` | [42](../../src/tool_calls.rs#L42), [153](../../src/tool_calls.rs#L153), [161](../../src/tool_calls.rs#L161) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `"supervisor lost before tool execution".to_owned` | [43](../../src/tool_calls.rs#L43) | receiver-type-required |
| `execute_provider_tool_calls` | `cancellation.stop_requested` | [46](../../src/tool_calls.rs#L46), [157](../../src/tool_calls.rs#L157) | receiver-type-required |
| `execute_provider_tool_calls` | `Ok` | [47](../../src/tool_calls.rs#L47), [66](../../src/tool_calls.rs#L66), [158](../../src/tool_calls.rs#L158), [166](../../src/tool_calls.rs#L166), [190](../../src/tool_calls.rs#L190), [194](../../src/tool_calls.rs#L194) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `execute_child_spawn_tool` | [50](../../src/tool_calls.rs#L50) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `provider::invalid_arguments_detail` | [70](../../src/tool_calls.rs#L70) | [provider::normalize::invalid_arguments_detail](../../../provider/src/normalize.rs#L790) |
| `execute_provider_tool_calls` | `append_tool_validation_error` | [73](../../src/tool_calls.rs#L73), [85](../../src/tool_calls.rs#L85), [148](../../src/tool_calls.rs#L148) | [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L197) |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map(&#124;root&#124; profile.config.protected_roots(&root))             .and_then` | [76](../../src/tool_calls.rs#L76) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder(&thread_folder)             .map` | [76](../../src/tool_calls.rs#L76) | receiver-type-required |
| `execute_provider_tool_calls` | `storage_root_for_thread_folder` | [76](../../src/tool_calls.rs#L76) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `profile.config.protected_roots` | [77](../../src/tool_calls.rs#L77) | receiver-type-required |
| `execute_provider_tool_calls` | `invocation_write_roots` | [78](../../src/tool_calls.rs#L78) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `error.to_string` | [85](../../src/tool_calls.rs#L85), [132](../../src/tool_calls.rs#L132), [142](../../src/tool_calls.rs#L142) | receiver-type-required |
| `execute_provider_tool_calls` | `assemble_tool_backends` | [89](../../src/tool_calls.rs#L89) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `IJsonValue::parse` | [104](../../src/tool_calls.rs#L104) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `serde_json::to_vec` | [104](../../src/tool_calls.rs#L104) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `thread.clone` | [106](../../src/tool_calls.rs#L106) | receiver-type-required |
| `execute_provider_tool_calls` | `attempt.to_owned` | [108](../../src/tool_calls.rs#L108) | receiver-type-required |
| `execute_provider_tool_calls` | `call.call_id.clone` | [109](../../src/tool_calls.rs#L109) | receiver-type-required |
| `execute_provider_tool_calls` | `call.name.clone` | [110](../../src/tool_calls.rs#L110) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp().clone` | [112](../../src/tool_calls.rs#L112) | receiver-type-required |
| `execute_provider_tool_calls` | `options.event_timestamp` | [112](../../src/tool_calls.rs#L112) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries(&dynamic_backend)?             .is_empty` | [114](../../src/tool_calls.rs#L114) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)             .deferred_search_entries` | [114](../../src/tool_calls.rs#L114) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new` | [114](../../src/tool_calls.rs#L114), [134](../../src/tool_calls.rs#L134) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `tool_catalog_context` | [117](../../src/tool_calls.rs#L117) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `ToolPipeline::new` | [118](../../src/tool_calls.rs#L118) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `assets.clone` | [118](../../src/tool_calls.rs#L118) | receiver-type-required |
| `execute_provider_tool_calls` | `SecretScanner::default` | [118](../../src/tool_calls.rs#L118) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `manifest             .tools             .iter()             .any` | [119](../../src/tool_calls.rs#L119) | receiver-type-required |
| `execute_provider_tool_calls` | `manifest             .tools             .iter` | [119](../../src/tool_calls.rs#L119) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut backend,                 )                 .map_err` | [124](../../src/tool_calls.rs#L124) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new(&manifest, &context)                 .dispatch` | [124](../../src/tool_calls.rs#L124) | receiver-type-required |
| `execute_provider_tool_calls` | `ToolDispatcher::new` | [124](../../src/tool_calls.rs#L124) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch(                     &mut pipeline,                     &invocation,                     &hooks,                     &mut policy,                     &mut dynamic_backend,                 )                 .map_err` | [134](../../src/tool_calls.rs#L134) | receiver-type-required |
| `execute_provider_tool_calls` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)                 .dispatch` | [134](../../src/tool_calls.rs#L134) | receiver-type-required |
| `execute_provider_tool_calls` | `drop` | [147](../../src/tool_calls.rs#L147), [175](../../src/tool_calls.rs#L175), [176](../../src/tool_calls.rs#L176) | external-constructor-callback-or-unresolved |
| `execute_provider_tool_calls` | `cancellation.protocol_failed` | [152](../../src/tool_calls.rs#L152) | receiver-type-required |
| `execute_provider_tool_calls` | `"tool-control protocol failure".to_owned` | [154](../../src/tool_calls.rs#L154) | receiver-type-required |
| `execute_provider_tool_calls` | `"supervisor lost during tool execution".to_owned` | [162](../../src/tool_calls.rs#L162) | receiver-type-required |
| `execute_provider_tool_calls` | `wait_tool_continuation` | [177](../../src/tool_calls.rs#L177) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `make_event` | [204](../../src/tool_calls.rs#L204) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `ledger.append_contract` | [210](../../src/tool_calls.rs#L210) | receiver-type-required |
| `append_tool_validation_error` | `BarrierContext::default` | [210](../../src/tool_calls.rs#L210) | external-constructor-callback-or-unresolved |
| `append_tool_validation_error` | `Ok` | [211](../../src/tool_calls.rs#L211) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective hook {logical_name:?} has an invalid index"))?;             decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err(&#124;error&#124; Box::new(error) as Box<dyn std::error::Error>)         })         .collect` | [217](../../src/tool_calls.rs#L217) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter(&#124;(_, index)&#124; {             if !instruction                 .sources                 .get(**index as usize)                 .is_some_and(&#124;source&#124; instruction.source_authorized(source))             {                 return false;             }             instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and(&#124;value&#124; value["format"] == 1)         })         .map` | [217](../../src/tool_calls.rs#L217) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter()         .filter` | [217](../../src/tool_calls.rs#L217) | receiver-type-required |
| `frozen_hook_bindings` | `instruction         .effective         .hooks         .iter` | [217](../../src/tool_calls.rs#L217) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .is_some_and` | [222](../../src/tool_calls.rs#L222) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get` | [222](../../src/tool_calls.rs#L222), [229](../../src/tool_calls.rs#L229), [236](../../src/tool_calls.rs#L236) | receiver-type-required |
| `frozen_hook_bindings` | `instruction.source_authorized` | [225](../../src/tool_calls.rs#L225) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then(&#124;source&#124; serde_json::from_str::<Value>(&source.content).ok())                 .is_some_and` | [229](../../src/tool_calls.rs#L229) | receiver-type-required |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(**index as usize)                 .and_then` | [229](../../src/tool_calls.rs#L229) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>(&source.content).ok` | [232](../../src/tool_calls.rs#L232) | receiver-type-required |
| `frozen_hook_bindings` | `serde_json::from_str::<Value>` | [232](../../src/tool_calls.rs#L232) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [236](../../src/tool_calls.rs#L236) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding(source.content.as_bytes(), logical_name)                 .map_err` | [240](../../src/tool_calls.rs#L240) | receiver-type-required |
| `frozen_hook_bindings` | `decode_hook_binding` | [240](../../src/tool_calls.rs#L240) | external-constructor-callback-or-unresolved |
| `frozen_hook_bindings` | `source.content.as_bytes` | [240](../../src/tool_calls.rs#L240) | receiver-type-required |
| `frozen_hook_bindings` | `Box::new` | [241](../../src/tool_calls.rs#L241) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `ResourceCatalog::from_snapshot` | [249](../../src/tool_calls.rs#L249) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map(&#124;summary&#124; {             let package = resources.skill(&summary.name)?;             let content = json!({                 "format": 1,                 "name": summary.name,                 "description": summary.description,                 "body": package.body,                 "resources": package.resources,             });             Ok(CatalogEntry {                 name: summary.name,                 summary: summary.description,                 aliases: Vec::new(),                 schema_digest: summary.content_digest,                 content: IJsonValue::parse(&serde_json::to_vec(&content)?)?,             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [250](../../src/tool_calls.rs#L250) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter()         .map` | [250](../../src/tool_calls.rs#L250) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries()         .into_iter` | [250](../../src/tool_calls.rs#L250) | receiver-type-required |
| `frozen_skill_catalog` | `resources         .skill_summaries` | [250](../../src/tool_calls.rs#L250) | receiver-type-required |
| `frozen_skill_catalog` | `resources.skill` | [254](../../src/tool_calls.rs#L254) | receiver-type-required |
| `frozen_skill_catalog` | `Ok` | [262](../../src/tool_calls.rs#L262), [292](../../src/tool_calls.rs#L292), [303](../../src/tool_calls.rs#L303) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `Vec::new` | [265](../../src/tool_calls.rs#L265), [295](../../src/tool_calls.rs#L295) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `IJsonValue::parse` | [267](../../src/tool_calls.rs#L267) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `serde_json::to_vec` | [267](../../src/tool_calls.rs#L267) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map(&#124;(logical_name, index)&#124; {             let source = instruction                 .sources                 .get(*index as usize)                 .ok_or_else(&#124;&#124; format!("effective skill {logical_name:?} has an invalid index"))?;             let summary = source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else(&#124;&#124; logical_name.clone(), &#124;line&#124; line.trim().to_owned());             Ok(CatalogEntry {                 name: logical_name.clone(),                 summary,                 aliases: Vec::new(),                 schema_digest: source.content_sha256.clone(),                 content: IJsonValue::from(source.content.clone()),             })         })         .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>` | [271](../../src/tool_calls.rs#L271) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter(&#124;(logical_name, index)&#124; {             !logical_name.contains('/')                 && instruction                     .sources                     .get(**index as usize)                     .is_some_and(&#124;source&#124; instruction.source_authorized(source))         })         .map` | [271](../../src/tool_calls.rs#L271) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter()         .filter` | [271](../../src/tool_calls.rs#L271) | receiver-type-required |
| `frozen_skill_catalog` | `instruction         .effective         .skills         .iter` | [271](../../src/tool_calls.rs#L271) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.contains` | [276](../../src/tool_calls.rs#L276) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get(**index as usize)                     .is_some_and` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                     .sources                     .get` | [277](../../src/tool_calls.rs#L277) | receiver-type-required |
| `frozen_skill_catalog` | `instruction.source_authorized` | [280](../../src/tool_calls.rs#L280) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get(*index as usize)                 .ok_or_else` | [283](../../src/tool_calls.rs#L283) | receiver-type-required |
| `frozen_skill_catalog` | `instruction                 .sources                 .get` | [283](../../src/tool_calls.rs#L283) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find(&#124;line&#124; !line.trim().is_empty())                 .map_or_else` | [287](../../src/tool_calls.rs#L287) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines()                 .find` | [287](../../src/tool_calls.rs#L287) | receiver-type-required |
| `frozen_skill_catalog` | `source                 .content                 .lines` | [287](../../src/tool_calls.rs#L287) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().is_empty` | [290](../../src/tool_calls.rs#L290) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim` | [290](../../src/tool_calls.rs#L290), [291](../../src/tool_calls.rs#L291) | receiver-type-required |
| `frozen_skill_catalog` | `logical_name.clone` | [291](../../src/tool_calls.rs#L291), [293](../../src/tool_calls.rs#L293) | receiver-type-required |
| `frozen_skill_catalog` | `line.trim().to_owned` | [291](../../src/tool_calls.rs#L291) | receiver-type-required |
| `frozen_skill_catalog` | `source.content_sha256.clone` | [296](../../src/tool_calls.rs#L296) | receiver-type-required |
| `frozen_skill_catalog` | `IJsonValue::from` | [297](../../src/tool_calls.rs#L297) | external-constructor-callback-or-unresolved |
| `frozen_skill_catalog` | `source.content.clone` | [297](../../src/tool_calls.rs#L297) | receiver-type-required |
| `frozen_skill_catalog` | `entries.extend` | [301](../../src/tool_calls.rs#L301) | receiver-type-required |
| `frozen_skill_catalog` | `entries.sort_by` | [302](../../src/tool_calls.rs#L302) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes().cmp` | [302](../../src/tool_calls.rs#L302) | receiver-type-required |
| `frozen_skill_catalog` | `left.name.as_bytes` | [302](../../src/tool_calls.rs#L302) | receiver-type-required |
| `frozen_skill_catalog` | `right.name.as_bytes` | [302](../../src/tool_calls.rs#L302) | receiver-type-required |
