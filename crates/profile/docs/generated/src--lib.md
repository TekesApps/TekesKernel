# profile

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [profile::ProfileError](../../src/lib.rs#L42) | enum_item | `pub` |  |
| [profile::parse_canonical](../../src/lib.rs#L71) | function_item | `pub(crate)` |  |
| [profile::canonical_line](../../src/lib.rs#L116) | function_item | `pub(crate)` |  |
| [profile::digest](../../src/lib.rs#L130) | function_item | `pub(crate)` |  |
| [profile::contains_null](../../src/lib.rs#L134) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `ConfigRepository` | `config::ConfigRepository` | `pub` |
| `ConfigSnapshot` | `config::ConfigSnapshot` | `pub` |
| `Environment` | `config::Environment` | `pub` |
| `EnvironmentsConfig` | `config::EnvironmentsConfig` | `pub` |
| `Limits` | `config::Limits` | `pub` |
| `Model` | `config::Model` | `pub` |
| `Provider` | `config::Provider` | `pub` |
| `ProvidersConfig` | `config::ProvidersConfig` | `pub` |
| `ResolvedWorkspace` | `config::ResolvedWorkspace` | `pub` |
| `RevisionVector` | `config::RevisionVector` | `pub` |
| `ScopeSelection` | `config::ScopeSelection` | `pub` |
| `SessionSettings` | `config::SessionSettings` | `pub` |
| `SettingsConfig` | `config::SettingsConfig` | `pub` |
| `WebSearch` | `config::WebSearch` | `pub` |
| `WorkspaceConfig` | `config::WorkspaceConfig` | `pub` |
| `WorkspaceFolder` | `config::WorkspaceFolder` | `pub` |
| `WorkspacePolicy` | `config::WorkspacePolicy` | `pub` |
| `FolderAuthorityStore` | `folder_authority::FolderAuthorityStore` | `pub` |
| `FolderEffectLease` | `folder_authority::FolderEffectLease` | `pub` |
| `FolderIdentity` | `folder_authority::FolderIdentity` | `pub` |
| `FolderTrust` | `folder_authority::FolderTrust` | `pub` |
| `FolderTrustReceipt` | `folder_authority::FolderTrustReceipt` | `pub` |
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
| `FolderRecord` | `scope::FolderRecord` | `pub` |
| `SessionScope` | `scope::SessionScope` | `pub` |
| `workspace_instructions` | `scope::workspace_instructions` | `pub` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `DeserializeOwned` | `serde::de::DeserializeOwned` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `FolderSnapshotAuthority` | `instruction::FolderSnapshotAuthority` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `profile::config` | `private` |  |
| `profile::folder_authority` | `private` |  |
| `profile::instruction` | `private` |  |
| `profile::launch` | `private` |  |
| `profile::resources` | `private` |  |
| `profile::scope` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::canonical_line"]
  n1["profile::digest"]
  n2["profile::contains_null"]
  n3["profile::parse_canonical"]
  n4["schema::ijson::IJsonValue::parse"]
  n3 --> n2
  n3 --> n4
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse_canonical` | `path.into` | [75](../../src/lib.rs#L75) | receiver-type-required |
| `parse_canonical` | `bytes.ends_with` | [76](../../src/lib.rs#L76) | receiver-type-required |
| `parse_canonical` | `bytes.len` | [76](../../src/lib.rs#L76), [82](../../src/lib.rs#L82) | receiver-type-required |
| `parse_canonical` | `bytes[..bytes.len() - 1].ends_with` | [76](../../src/lib.rs#L76) | receiver-type-required |
| `parse_canonical` | `Err` | [77](../../src/lib.rs#L77), [94](../../src/lib.rs#L94), [105](../../src/lib.rs#L105) | external-constructor-callback-or-unresolved |
| `parse_canonical` | `"expected one canonical JSON object followed by exactly one LF".to_owned` | [79](../../src/lib.rs#L79) | receiver-type-required |
| `parse_canonical` | `schema::IJsonValue::parse(body).map_err` | [83](../../src/lib.rs#L83) | receiver-type-required |
| `parse_canonical` | `schema::IJsonValue::parse` | [83](../../src/lib.rs#L83) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `parse_canonical` | `path.clone` | [84](../../src/lib.rs#L84), [90](../../src/lib.rs#L90), [101](../../src/lib.rs#L101) | receiver-type-required |
| `parse_canonical` | `error.to_string` | [85](../../src/lib.rs#L85), [91](../../src/lib.rs#L91), [102](../../src/lib.rs#L102), [112](../../src/lib.rs#L112) | receiver-type-required |
| `parse_canonical` | `value         .canonical_bytes()         .map_err` | [87](../../src/lib.rs#L87) | receiver-type-required |
| `parse_canonical` | `value         .canonical_bytes` | [87](../../src/lib.rs#L87) | receiver-type-required |
| `parse_canonical` | `"bytes are not RFC-8785 canonical".to_owned` | [96](../../src/lib.rs#L96) | receiver-type-required |
| `parse_canonical` | `serde_json::from_slice(body).map_err` | [100](../../src/lib.rs#L100) | receiver-type-required |
| `parse_canonical` | `serde_json::from_slice` | [100](../../src/lib.rs#L100) | external-constructor-callback-or-unresolved |
| `parse_canonical` | `contains_null` | [104](../../src/lib.rs#L104) | [profile::contains_null](../../src/lib.rs#L134) |
| `parse_canonical` | `"JSON null is not a substitute for an absent optional field".to_owned` | [107](../../src/lib.rs#L107) | receiver-type-required |
| `parse_canonical` | `serde_json::from_value(json).map_err` | [110](../../src/lib.rs#L110) | receiver-type-required |
| `parse_canonical` | `serde_json::from_value` | [110](../../src/lib.rs#L110) | external-constructor-callback-or-unresolved |
| `canonical_line` | `path.into` | [120](../../src/lib.rs#L120) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec(value).map_err` | [122](../../src/lib.rs#L122) | receiver-type-required |
| `canonical_line` | `serde_json_canonicalizer::to_vec` | [122](../../src/lib.rs#L122) | external-constructor-callback-or-unresolved |
| `canonical_line` | `error.to_string` | [124](../../src/lib.rs#L124) | receiver-type-required |
| `canonical_line` | `bytes.push` | [126](../../src/lib.rs#L126) | receiver-type-required |
| `canonical_line` | `Ok` | [127](../../src/lib.rs#L127) | external-constructor-callback-or-unresolved |
| `contains_null` | `values.iter().any` | [137](../../src/lib.rs#L137) | receiver-type-required |
| `contains_null` | `values.iter` | [137](../../src/lib.rs#L137) | receiver-type-required |
| `contains_null` | `values.values().any` | [138](../../src/lib.rs#L138) | receiver-type-required |
| `contains_null` | `values.values` | [138](../../src/lib.rs#L138) | receiver-type-required |
