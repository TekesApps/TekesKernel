# profile

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [profile::ProfileError](../../src/lib.rs#L44) | enum_item | `pub` |  |
| [profile::parse_canonical](../../src/lib.rs#L73) | function_item | `pub(crate)` |  |
| [profile::parse_global_json](../../src/lib.rs#L123) | function_item | `pub(crate)` |  |
| [profile::canonical_line](../../src/lib.rs#L142) | function_item | `pub(crate)` |  |
| [profile::digest](../../src/lib.rs#L156) | function_item | `pub(crate)` |  |
| [profile::contains_null](../../src/lib.rs#L160) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `AutomaticTitleMode` | `config::AutomaticTitleMode` | `pub` |
| `AutomaticTitleSettings` | `config::AutomaticTitleSettings` | `pub` |
| `ConfigRepository` | `config::ConfigRepository` | `pub` |
| `ConfigSnapshot` | `config::ConfigSnapshot` | `pub` |
| `DEFAULT_AUTOMATIC_TITLE_MAX_INPUT_CHARS` | `config::DEFAULT_AUTOMATIC_TITLE_MAX_INPUT_CHARS` | `pub` |
| `ExecutionPolicy` | `config::ExecutionPolicy` | `pub` |
| `Limits` | `config::Limits` | `pub` |
| `MAX_AUTOMATIC_TITLE_MAX_INPUT_CHARS` | `config::MAX_AUTOMATIC_TITLE_MAX_INPUT_CHARS` | `pub` |
| `Model` | `config::Model` | `pub` |
| `Provider` | `config::Provider` | `pub` |
| `ProvidersConfig` | `config::ProvidersConfig` | `pub` |
| `ResolvedExecution` | `config::ResolvedExecution` | `pub` |
| `RevisionVector` | `config::RevisionVector` | `pub` |
| `SessionSettings` | `config::SessionSettings` | `pub` |
| `SettingsConfig` | `config::SettingsConfig` | `pub` |
| `WebSearch` | `config::WebSearch` | `pub` |
| `ExecutionConfig` | `execution::ExecutionConfig` | `pub` |
| `NormalizedExecution` | `execution::NormalizedExecution` | `pub` |
| `execution_metrics` | `execution::execution_metrics` | `pub` |
| `reset_execution_metrics` | `execution::reset_execution_metrics` | `pub` |
| `EffectiveInstructions` | `instruction::EffectiveInstructions` | `pub` |
| `EffectivePolicy` | `instruction::EffectivePolicy` | `pub` |
| `InstructionKind` | `instruction::InstructionKind` | `pub` |
| `InstructionOrigin` | `instruction::InstructionOrigin` | `pub` |
| `InstructionPolicy` | `instruction::InstructionPolicy` | `pub` |
| `InstructionResolver` | `instruction::InstructionResolver` | `pub` |
| `InstructionSettings` | `instruction::InstructionSettings` | `pub` |
| `InstructionSnapshot` | `instruction::InstructionSnapshot` | `pub` |
| `InstructionSource` | `instruction::InstructionSource` | `pub` |
| `LaunchProfile` | `instruction::LaunchProfile` | `pub` |
| `DynamicTool` | `launch::DynamicTool` | `pub` |
| `DynamicToolCatalog` | `launch::DynamicToolCatalog` | `pub` |
| `DynamicToolEffect` | `launch::DynamicToolEffect` | `pub` |
| `DynamicToolSource` | `launch::DynamicToolSource` | `pub` |
| `DynamicToolSourceKind` | `launch::DynamicToolSourceKind` | `pub` |
| `ExternalEffectBinding` | `launch::ExternalEffectBinding` | `pub` |
| `ExternalEffectProtocol` | `launch::ExternalEffectProtocol` | `pub` |
| `LaunchBindings` | `launch::LaunchBindings` | `pub` |
| `MAX_UNAVAILABLE_TOOLS` | `launch::MAX_UNAVAILABLE_TOOLS` | `pub` |
| `SchemaRefusal` | `launch::SchemaRefusal` | `pub` |
| `is_dynamic_tool_name` | `launch::is_dynamic_tool_name` | `pub` |
| `CommandCatalogEntry` | `resources::CommandCatalogEntry` | `pub` |
| `CommandExpansion` | `resources::CommandExpansion` | `pub` |
| `CommandSummary` | `resources::CommandSummary` | `pub` |
| `ResourceCatalog` | `resources::ResourceCatalog` | `pub` |
| `ResourceError` | `resources::ResourceError` | `pub` |
| `SkillPackage` | `resources::SkillPackage` | `pub` |
| `SkillResource` | `resources::SkillResource` | `pub` |
| `SkillSummary` | `resources::SkillSummary` | `pub` |
| `SessionScope` | `scope::SessionScope` | `pub` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `DeserializeOwned` | `serde::de::DeserializeOwned` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `profile::config` | `private` |  |
| `profile::execution` | `private` |  |
| `profile::instruction` | `private` |  |
| `profile::launch` | `private` |  |
| `profile::resources` | `private` |  |
| `profile::scope` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–5: 4 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::parse_global_json"]
  n1["profile::canonical_line"]
  n2["profile::digest"]
  n3["profile::contains_null"]
  n4["profile::parse_canonical"]
  n5["schema::ijson::IJsonValue::parse"]
  n0 --> n4
  n0 --> n5
  n4 --> n3
  n4 --> n5
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse_canonical` | `path.into` | [77](../../src/lib.rs#L77) | receiver-type-required |
| `parse_canonical` | `bytes.ends_with` | [78](../../src/lib.rs#L78) | receiver-type-required |
| `parse_canonical` | `bytes.len` | [78](../../src/lib.rs#L78), [84](../../src/lib.rs#L84) | receiver-type-required |
| `parse_canonical` | `bytes[..bytes.len() - 1].ends_with` | [78](../../src/lib.rs#L78) | receiver-type-required |
| `parse_canonical` | `Err` | [79](../../src/lib.rs#L79), [96](../../src/lib.rs#L96), [107](../../src/lib.rs#L107) | external-constructor-callback-or-unresolved |
| `parse_canonical` | `"expected one canonical JSON object followed by exactly one LF".to_owned` | [81](../../src/lib.rs#L81) | receiver-type-required |
| `parse_canonical` | `schema::IJsonValue::parse(body).map_err` | [85](../../src/lib.rs#L85) | receiver-type-required |
| `parse_canonical` | `schema::IJsonValue::parse` | [85](../../src/lib.rs#L85) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `parse_canonical` | `path.clone` | [86](../../src/lib.rs#L86), [92](../../src/lib.rs#L92), [103](../../src/lib.rs#L103) | receiver-type-required |
| `parse_canonical` | `error.to_string` | [87](../../src/lib.rs#L87), [93](../../src/lib.rs#L93), [104](../../src/lib.rs#L104), [114](../../src/lib.rs#L114) | receiver-type-required |
| `parse_canonical` | `value         .canonical_bytes()         .map_err` | [89](../../src/lib.rs#L89) | receiver-type-required |
| `parse_canonical` | `value         .canonical_bytes` | [89](../../src/lib.rs#L89) | receiver-type-required |
| `parse_canonical` | `"bytes are not RFC-8785 canonical".to_owned` | [98](../../src/lib.rs#L98) | receiver-type-required |
| `parse_canonical` | `serde_json::from_slice(body).map_err` | [102](../../src/lib.rs#L102) | receiver-type-required |
| `parse_canonical` | `serde_json::from_slice` | [102](../../src/lib.rs#L102) | external-constructor-callback-or-unresolved |
| `parse_canonical` | `contains_null` | [106](../../src/lib.rs#L106) | [profile::contains_null](../../src/lib.rs#L160) |
| `parse_canonical` | `"JSON null is not a substitute for an absent optional field".to_owned` | [109](../../src/lib.rs#L109) | receiver-type-required |
| `parse_canonical` | `serde_json::from_value(json).map_err` | [112](../../src/lib.rs#L112) | receiver-type-required |
| `parse_canonical` | `serde_json::from_value` | [112](../../src/lib.rs#L112) | external-constructor-callback-or-unresolved |
| `parse_global_json` | `path.into` | [127](../../src/lib.rs#L127) | receiver-type-required |
| `parse_global_json` | `schema::IJsonValue::parse(bytes).map_err` | [128](../../src/lib.rs#L128) | receiver-type-required |
| `parse_global_json` | `schema::IJsonValue::parse` | [128](../../src/lib.rs#L128) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `parse_global_json` | `path.clone` | [129](../../src/lib.rs#L129), [135](../../src/lib.rs#L135) | receiver-type-required |
| `parse_global_json` | `error.to_string` | [130](../../src/lib.rs#L130), [136](../../src/lib.rs#L136) | receiver-type-required |
| `parse_global_json` | `value         .canonical_bytes()         .map_err` | [132](../../src/lib.rs#L132) | receiver-type-required |
| `parse_global_json` | `value         .canonical_bytes` | [132](../../src/lib.rs#L132) | receiver-type-required |
| `parse_global_json` | `canonical.push` | [138](../../src/lib.rs#L138) | receiver-type-required |
| `parse_global_json` | `parse_canonical` | [139](../../src/lib.rs#L139) | [profile::parse_canonical](../../src/lib.rs#L73) |
| `canonical_line` | `path.into` | [146](../../src/lib.rs#L146) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec(value).map_err` | [148](../../src/lib.rs#L148) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec` | [148](../../src/lib.rs#L148) | external-constructor-callback-or-unresolved |
| `canonical_line` | `error.to_string` | [150](../../src/lib.rs#L150) | receiver-type-required |
| `canonical_line` | `bytes.push` | [152](../../src/lib.rs#L152) | receiver-type-required |
| `canonical_line` | `Ok` | [153](../../src/lib.rs#L153) | external-constructor-callback-or-unresolved |
| `contains_null` | `values.iter().any` | [163](../../src/lib.rs#L163) | receiver-type-required |
| `contains_null` | `values.iter` | [163](../../src/lib.rs#L163) | receiver-type-required |
| `contains_null` | `values.values().any` | [164](../../src/lib.rs#L164) | receiver-type-required |
| `contains_null` | `values.values` | [164](../../src/lib.rs#L164) | receiver-type-required |
