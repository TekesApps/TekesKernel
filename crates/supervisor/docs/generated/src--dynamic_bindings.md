# tekes-supervisor::dynamic_bindings

[Package atlas](index.md) · [Source](../../src/dynamic_bindings.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::dynamic_bindings::resolve_worker_launch_bindings](../../src/dynamic_bindings.rs#L14) | function_item | `pub` |  |
| [tekes-supervisor::dynamic_bindings::DynamicBindingError](../../src/dynamic_bindings.rs#L42) | enum_item | `pub` |  |
| [tekes-supervisor::dynamic_bindings::tests::config](../../src/dynamic_bindings.rs#L58) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::dynamic_bindings::tests::instruction](../../src/dynamic_bindings.rs#L103) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::dynamic_bindings::tests::mcp_tool](../../src/dynamic_bindings.rs#L112) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::dynamic_bindings::tests::mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed](../../src/dynamic_bindings.rs#L137) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::dynamic_bindings::tests::a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable](../../src/dynamic_bindings.rs#L162) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::dynamic_bindings::tests::fixed_and_dynamic_name_collisions_fail_launch](../../src/dynamic_bindings.rs#L225) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `ConfigSnapshot` | `profile::ConfigSnapshot` | `private` |
| `DynamicToolCatalog` | `profile::DynamicToolCatalog` | `private` |
| `InstructionSnapshot` | `profile::InstructionSnapshot` | `private` |
| `WorkerLaunchBindings` | `crate::WorkerLaunchBindings` | `private` |
| `ConfigSnapshot` | `profile::ConfigSnapshot` | `private` |
| `DynamicTool` | `profile::DynamicTool` | `private` |
| `DynamicToolEffect` | `profile::DynamicToolEffect` | `private` |
| `DynamicToolSource` | `profile::DynamicToolSource` | `private` |
| `DynamicToolSourceKind` | `profile::DynamicToolSourceKind` | `private` |
| `EffectiveInstructions` | `profile::EffectiveInstructions` | `private` |
| `InstructionSnapshot` | `profile::InstructionSnapshot` | `private` |
| `ProvidersConfig` | `profile::ProvidersConfig` | `private` |
| `ResolvedWorkspace` | `profile::ResolvedWorkspace` | `private` |
| `RevisionVector` | `profile::RevisionVector` | `private` |
| `SettingsConfig` | `profile::SettingsConfig` | `private` |
| `WorkspacePolicy` | `profile::WorkspacePolicy` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::dynamic_bindings::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::launch::DynamicToolCatalog::resolve"]
  n1["profile::launch::LaunchBindings::bind_with_unavailable"]
  n2["tekes-supervisor::dynamic_bindings::resolve_worker_launch_bindings"]
  n2 --> n0
  n2 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `resolve_worker_launch_bindings` | `DynamicToolCatalog::resolve(tools)         .map_err` | [21](../../src/dynamic_bindings.rs#L21) | receiver-type-required |
| `resolve_worker_launch_bindings` | `DynamicToolCatalog::resolve` | [21](../../src/dynamic_bindings.rs#L21) | [profile::launch::DynamicToolCatalog::resolve](../../../profile/src/launch.rs#L128) |
| `resolve_worker_launch_bindings` | `DynamicBindingError::Invalid` | [22](../../src/dynamic_bindings.rs#L22), [37](../../src/dynamic_bindings.rs#L37) | external-constructor-callback-or-unresolved |
| `resolve_worker_launch_bindings` | `error.to_string` | [22](../../src/dynamic_bindings.rs#L22), [37](../../src/dynamic_bindings.rs#L37) | receiver-type-required |
| `resolve_worker_launch_bindings` | `instruction.meet_workspace_policy` | [25](../../src/dynamic_bindings.rs#L25) | receiver-type-required |
| `resolve_worker_launch_bindings` | `profile::LaunchBindings::bind_with_unavailable(         config,         bindings.goal_id.clone(),         bindings.dynamic_catalog.clone(),         bindings.unavailable_tools.clone(),     )     .map_err` | [31](../../src/dynamic_bindings.rs#L31) | receiver-type-required |
| `resolve_worker_launch_bindings` | `profile::LaunchBindings::bind_with_unavailable` | [31](../../src/dynamic_bindings.rs#L31) | [profile::launch::LaunchBindings::bind_with_unavailable](../../../profile/src/launch.rs#L193) |
| `resolve_worker_launch_bindings` | `bindings.goal_id.clone` | [33](../../src/dynamic_bindings.rs#L33) | receiver-type-required |
| `resolve_worker_launch_bindings` | `bindings.dynamic_catalog.clone` | [34](../../src/dynamic_bindings.rs#L34) | receiver-type-required |
| `resolve_worker_launch_bindings` | `bindings.unavailable_tools.clone` | [35](../../src/dynamic_bindings.rs#L35) | receiver-type-required |
| `resolve_worker_launch_bindings` | `Ok` | [38](../../src/dynamic_bindings.rs#L38) | external-constructor-callback-or-unresolved |
| `config` | `"ws".to_owned` | [67](../../src/dynamic_bindings.rs#L67), [68](../../src/dynamic_bindings.rs#L68) | receiver-type-required |
| `config` | `WorkspacePolicy::default` | [80](../../src/dynamic_bindings.rs#L80) | external-constructor-callback-or-unresolved |
| `config` | `Vec::new` | [86](../../src/dynamic_bindings.rs#L86) | external-constructor-callback-or-unresolved |
| `config` | `SettingsConfig::default` | [90](../../src/dynamic_bindings.rs#L90) | external-constructor-callback-or-unresolved |
| `instruction` | `Vec::new` | [107](../../src/dynamic_bindings.rs#L107) | external-constructor-callback-or-unresolved |
| `instruction` | `EffectiveInstructions::default` | [108](../../src/dynamic_bindings.rs#L108) | external-constructor-callback-or-unresolved |
| `mcp_tool` | `IJsonValue::parse(             &serde_json_canonicalizer::to_vec(&serde_json::json!({                 "name": name,                 "description": "Echo one value",                 "parameters": {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}             }))             .unwrap(),         )         .unwrap` | [113](../../src/dynamic_bindings.rs#L113) | receiver-type-required |
| `mcp_tool` | `IJsonValue::parse` | [113](../../src/dynamic_bindings.rs#L113) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `mcp_tool` | `serde_json_canonicalizer::to_vec(&serde_json::json!({                 "name": name,                 "description": "Echo one value",                 "parameters": {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}             }))             .unwrap` | [114](../../src/dynamic_bindings.rs#L114) | receiver-type-required |
| `mcp_tool` | `serde_json_canonicalizer::to_vec` | [114](../../src/dynamic_bindings.rs#L114) | external-constructor-callback-or-unresolved |
| `mcp_tool` | `DynamicTool::declared(             name,             DynamicToolSource {                 kind: DynamicToolSourceKind::Mcp,                 id: "fixture".to_owned(),             },             DynamicToolEffect::ReadOnly,             false,             Vec::new(),             schema,         )         .expect` | [122](../../src/dynamic_bindings.rs#L122) | receiver-type-required |
| `mcp_tool` | `DynamicTool::declared` | [122](../../src/dynamic_bindings.rs#L122) | [profile::launch::DynamicTool::declared](../../../profile/src/launch.rs#L87) |
| `mcp_tool` | `"fixture".to_owned` | [126](../../src/dynamic_bindings.rs#L126) | receiver-type-required |
| `mcp_tool` | `Vec::new` | [130](../../src/dynamic_bindings.rs#L130) | external-constructor-callback-or-unresolved |
| `mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed` | `resolve_worker_launch_bindings(             &config(vec!["mcp__fixture__echo".to_owned(), "read".to_owned()]),             &instruction(),             None,             vec![mcp_tool("mcp__fixture__echo")],             Vec::new(),         )         .expect` | [138](../../src/dynamic_bindings.rs#L138) | receiver-type-required |
| `mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed` | `resolve_worker_launch_bindings` | [138](../../src/dynamic_bindings.rs#L138), [148](../../src/dynamic_bindings.rs#L148) | external-constructor-callback-or-unresolved |
| `mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed` | `config` | [139](../../src/dynamic_bindings.rs#L139), [149](../../src/dynamic_bindings.rs#L149) | [tekes-supervisor::dynamic_bindings::tests::config](../../src/dynamic_bindings.rs#L58) |
| `mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed` | `instruction` | [140](../../src/dynamic_bindings.rs#L140), [150](../../src/dynamic_bindings.rs#L150) | [tekes-supervisor::dynamic_bindings::tests::instruction](../../src/dynamic_bindings.rs#L103) |
| `mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed` | `Vec::new` | [143](../../src/dynamic_bindings.rs#L143), [153](../../src/dynamic_bindings.rs#L153) | external-constructor-callback-or-unresolved |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `config` | [164](../../src/dynamic_bindings.rs#L164), [198](../../src/dynamic_bindings.rs#L198) | [tekes-supervisor::dynamic_bindings::tests::config](../../src/dynamic_bindings.rs#L58) |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `resolve_worker_launch_bindings(             &allowed(),             &instruction(),             None,             tools(),             vec!["mcp__fixture__refused".to_owned()],         )         .expect` | [175](../../src/dynamic_bindings.rs#L175) | receiver-type-required |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `resolve_worker_launch_bindings` | [175](../../src/dynamic_bindings.rs#L175) | external-constructor-callback-or-unresolved |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `allowed` | [176](../../src/dynamic_bindings.rs#L176) | external-constructor-callback-or-unresolved |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `instruction` | [177](../../src/dynamic_bindings.rs#L177) | [tekes-supervisor::dynamic_bindings::tests::instruction](../../src/dynamic_bindings.rs#L103) |
| `a_refused_tool_is_excused_by_exact_name_and_never_becomes_executable` | `tools` | [179](../../src/dynamic_bindings.rs#L179) | external-constructor-callback-or-unresolved |
| `fixed_and_dynamic_name_collisions_fail_launch` | `resolve_worker_launch_bindings` | [226](../../src/dynamic_bindings.rs#L226) | external-constructor-callback-or-unresolved |
| `fixed_and_dynamic_name_collisions_fail_launch` | `config` | [227](../../src/dynamic_bindings.rs#L227) | [tekes-supervisor::dynamic_bindings::tests::config](../../src/dynamic_bindings.rs#L58) |
| `fixed_and_dynamic_name_collisions_fail_launch` | `instruction` | [228](../../src/dynamic_bindings.rs#L228) | [tekes-supervisor::dynamic_bindings::tests::instruction](../../src/dynamic_bindings.rs#L103) |
| `fixed_and_dynamic_name_collisions_fail_launch` | `Vec::new` | [231](../../src/dynamic_bindings.rs#L231) | external-constructor-callback-or-unresolved |
