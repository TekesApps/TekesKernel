# tools::builtin

[Package atlas](index.md) · [Source](../../src/builtin.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::builtin::BuiltinManifest](../../src/builtin.rs#L9) | struct_item | `pub` |  |
| [tools::builtin::BuiltinTool](../../src/builtin.rs#L16) | struct_item | `pub` |  |
| [tools::builtin::closed_enum](../../src/builtin.rs#L24) | macro_definition | `private` |  |
| [tools::builtin::BuiltinDescriptor](../../src/builtin.rs#L64) | struct_item | `private` |  |
| [tools::builtin::descriptor](../../src/builtin.rs#L72) | macro_definition | `private` |  |
| [tools::builtin::BUILTIN_DESCRIPTORS](../../src/builtin.rs#L88) | const_item | `private` |  |
| [tools::builtin::CatalogRole](../../src/builtin.rs#L285) | enum_item | `pub` |  |
| [tools::builtin::CatalogContext](../../src/builtin.rs#L295) | struct_item | `pub` |  |
| [tools::builtin::CatalogContext::default](../../src/builtin.rs#L303) | function_item | `private` |  |
| [tools::builtin::BuiltinManifest::compiled](../../src/builtin.rs#L315) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::decode_canonical](../../src/builtin.rs#L335) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::validate](../../src/builtin.rs#L358) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::fixed_names](../../src/builtin.rs#L401) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::reject_dynamic_collisions](../../src/builtin.rs#L405) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::projection](../../src/builtin.rs#L420) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::interactive_names](../../src/builtin.rs#L445) | function_item | `pub` |  |
| [tools::builtin::BuiltinManifest::catalog_digest](../../src/builtin.rs#L465) | function_item | `pub` |  |
| [tools::builtin::availability_visible](../../src/builtin.rs#L471) | function_item | `private` |  |
| [tools::builtin::is_resident](../../src/builtin.rs#L484) | function_item | `private` |  |
| [tools::builtin::validate_name](../../src/builtin.rs#L493) | function_item | `private` |  |
| [tools::builtin::BuiltinError](../../src/builtin.rs#L504) | enum_item | `pub` |  |
| [tools::builtin::index_by_name](../../src/builtin.rs#L513) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–13: 6 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["tools::builtin::CatalogContext::default"]
  n2["tools::builtin::BuiltinManifest::compiled"]
  n3["tools::builtin::BuiltinManifest::decode_canonical"]
  n4["tools::builtin::BuiltinManifest::validate"]
  n5["tools::builtin::BuiltinManifest::fixed_names"]
  n6["tools::builtin::BuiltinManifest::reject_dynamic_collisions"]
  n7["tools::builtin::BuiltinManifest::projection"]
  n8["tools::builtin::BuiltinManifest::interactive_names"]
  n9["tools::builtin::BuiltinManifest::catalog_digest"]
  n10["tools::builtin::availability_visible"]
  n11["tools::builtin::is_resident"]
  n12["tools::builtin::validate_name"]
  n13["tools::builtin::index_by_name"]
  n3 --> n0
  n4 --> n12
  n6 --> n5
  n6 --> n12
  n7 --> n10
  n7 --> n11
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `default` | `BTreeSet::new` | [308](../../src/builtin.rs#L308) | external-constructor-callback-or-unresolved |
| `compiled` | `BUILTIN_DESCRIPTORS                 .iter()                 .map(&#124;descriptor&#124; BuiltinTool {                     arguments: descriptor                         .arguments                         .iter()                         .map(&#124;value&#124; (*value).to_owned())                         .collect(),                     availability: descriptor.availability,                     backend: descriptor.backend,                     effect: descriptor.effect,                     name: descriptor.name.to_owned(),                 })                 .collect` | [318](../../src/builtin.rs#L318) | receiver-type-required |
| `compiled` | `BUILTIN_DESCRIPTORS                 .iter()                 .map` | [318](../../src/builtin.rs#L318) | receiver-type-required |
| `compiled` | `BUILTIN_DESCRIPTORS                 .iter` | [318](../../src/builtin.rs#L318) | receiver-type-required |
| `compiled` | `descriptor                         .arguments                         .iter()                         .map(&#124;value&#124; (*value).to_owned())                         .collect` | [321](../../src/builtin.rs#L321) | receiver-type-required |
| `compiled` | `descriptor                         .arguments                         .iter()                         .map` | [321](../../src/builtin.rs#L321) | receiver-type-required |
| `compiled` | `descriptor                         .arguments                         .iter` | [321](../../src/builtin.rs#L321) | receiver-type-required |
| `compiled` | `(*value).to_owned` | [324](../../src/builtin.rs#L324) | receiver-type-required |
| `compiled` | `descriptor.name.to_owned` | [329](../../src/builtin.rs#L329) | receiver-type-required |
| `decode_canonical` | `bytes             .strip_suffix(b"\n")             .ok_or` | [336](../../src/builtin.rs#L336) | receiver-type-required |
| `decode_canonical` | `bytes             .strip_suffix` | [336](../../src/builtin.rs#L336) | receiver-type-required |
| `decode_canonical` | `BuiltinError::Invalid` | [338](../../src/builtin.rs#L338), [340](../../src/builtin.rs#L340), [345](../../src/builtin.rs#L345), [348](../../src/builtin.rs#L348), [351](../../src/builtin.rs#L351) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `body.is_empty` | [339](../../src/builtin.rs#L339) | receiver-type-required |
| `decode_canonical` | `body.contains` | [339](../../src/builtin.rs#L339) | receiver-type-required |
| `decode_canonical` | `Err` | [340](../../src/builtin.rs#L340), [351](../../src/builtin.rs#L351) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `schema::IJsonValue::parse(body)             .map_err` | [344](../../src/builtin.rs#L344) | receiver-type-required |
| `decode_canonical` | `schema::IJsonValue::parse` | [344](../../src/builtin.rs#L344) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `decode_canonical` | `value             .canonical_bytes()             .map_err` | [346](../../src/builtin.rs#L346) | receiver-type-required |
| `decode_canonical` | `value             .canonical_bytes` | [346](../../src/builtin.rs#L346) | receiver-type-required |
| `decode_canonical` | `serde_json::from_slice` | [353](../../src/builtin.rs#L353) | external-constructor-callback-or-unresolved |
| `decode_canonical` | `manifest.validate` | [354](../../src/builtin.rs#L354) | receiver-type-required |
| `decode_canonical` | `Ok` | [355](../../src/builtin.rs#L355) | external-constructor-callback-or-unresolved |
| `validate` | `Err` | [360](../../src/builtin.rs#L360), [363](../../src/builtin.rs#L363), [369](../../src/builtin.rs#L369), [378](../../src/builtin.rs#L378), [393](../../src/builtin.rs#L393) | external-constructor-callback-or-unresolved |
| `validate` | `BuiltinError::Invalid` | [360](../../src/builtin.rs#L360), [363](../../src/builtin.rs#L363), [369](../../src/builtin.rs#L369), [378](../../src/builtin.rs#L378), [393](../../src/builtin.rs#L393) | external-constructor-callback-or-unresolved |
| `validate` | `self.tools.len` | [362](../../src/builtin.rs#L362) | receiver-type-required |
| `validate` | `BUILTIN_DESCRIPTORS.len` | [362](../../src/builtin.rs#L362) | receiver-type-required |
| `validate` | `self.tools.windows` | [367](../../src/builtin.rs#L367) | receiver-type-required |
| `validate` | `pair[0].name.as_bytes` | [368](../../src/builtin.rs#L368) | receiver-type-required |
| `validate` | `pair[1].name.as_bytes` | [368](../../src/builtin.rs#L368) | receiver-type-required |
| `validate` | `validate_name` | [375](../../src/builtin.rs#L375) | [tools::builtin::validate_name](../../src/builtin.rs#L493) |
| `validate` | `BTreeSet::new` | [376](../../src/builtin.rs#L376) | external-constructor-callback-or-unresolved |
| `validate` | `tool.arguments.iter().any` | [377](../../src/builtin.rs#L377) | receiver-type-required |
| `validate` | `tool.arguments.iter` | [377](../../src/builtin.rs#L377) | receiver-type-required |
| `validate` | `arguments.insert` | [377](../../src/builtin.rs#L377) | receiver-type-required |
| `validate` | `self.tools.iter().zip` | [381](../../src/builtin.rs#L381) | receiver-type-required |
| `validate` | `self.tools.iter` | [381](../../src/builtin.rs#L381) | receiver-type-required |
| `validate` | `tool.arguments.len` | [383](../../src/builtin.rs#L383) | receiver-type-required |
| `validate` | `expected.arguments.len` | [383](../../src/builtin.rs#L383) | receiver-type-required |
| `validate` | `tool                     .arguments                     .iter()                     .map(String::as_str)                     .eq` | [384](../../src/builtin.rs#L384) | receiver-type-required |
| `validate` | `tool                     .arguments                     .iter()                     .map` | [384](../../src/builtin.rs#L384) | receiver-type-required |
| `validate` | `tool                     .arguments                     .iter` | [384](../../src/builtin.rs#L384) | receiver-type-required |
| `validate` | `expected.arguments.iter().copied` | [388](../../src/builtin.rs#L388) | receiver-type-required |
| `validate` | `expected.arguments.iter` | [388](../../src/builtin.rs#L388) | receiver-type-required |
| `validate` | `Ok` | [398](../../src/builtin.rs#L398) | external-constructor-callback-or-unresolved |
| `fixed_names` | `self.tools.iter().map(&#124;tool&#124; tool.name.as_str()).collect` | [402](../../src/builtin.rs#L402) | receiver-type-required |
| `fixed_names` | `self.tools.iter().map` | [402](../../src/builtin.rs#L402) | receiver-type-required |
| `fixed_names` | `self.tools.iter` | [402](../../src/builtin.rs#L402) | receiver-type-required |
| `fixed_names` | `tool.name.as_str` | [402](../../src/builtin.rs#L402) | receiver-type-required |
| `reject_dynamic_collisions` | `self.fixed_names` | [409](../../src/builtin.rs#L409) | [tools::builtin::BuiltinManifest::fixed_names](../../src/builtin.rs#L401) |
| `reject_dynamic_collisions` | `BTreeSet::new` | [410](../../src/builtin.rs#L410) | external-constructor-callback-or-unresolved |
| `reject_dynamic_collisions` | `validate_name` | [412](../../src/builtin.rs#L412) | [tools::builtin::validate_name](../../src/builtin.rs#L493) |
| `reject_dynamic_collisions` | `fixed.contains` | [413](../../src/builtin.rs#L413) | receiver-type-required |
| `reject_dynamic_collisions` | `dynamic.insert` | [413](../../src/builtin.rs#L413) | receiver-type-required |
| `reject_dynamic_collisions` | `Err` | [414](../../src/builtin.rs#L414) | external-constructor-callback-or-unresolved |
| `reject_dynamic_collisions` | `BuiltinError::NameCollision` | [414](../../src/builtin.rs#L414) | external-constructor-callback-or-unresolved |
| `reject_dynamic_collisions` | `name.to_owned` | [414](../../src/builtin.rs#L414) | receiver-type-required |
| `reject_dynamic_collisions` | `Ok` | [417](../../src/builtin.rs#L417) | external-constructor-callback-or-unresolved |
| `projection` | `availability_visible` | [422](../../src/builtin.rs#L422) | [tools::builtin::availability_visible](../../src/builtin.rs#L471) |
| `projection` | `self.tools             .iter()             .filter(visible)             .filter(&#124;tool&#124; is_resident(tool.availability))             .chain(                 self.tools                     .iter()                     .filter(visible)                     .filter(&#124;tool&#124; !is_resident(tool.availability)),             )             .collect` | [426](../../src/builtin.rs#L426) | receiver-type-required |
| `projection` | `self.tools             .iter()             .filter(visible)             .filter(&#124;tool&#124; is_resident(tool.availability))             .chain` | [426](../../src/builtin.rs#L426) | receiver-type-required |
| `projection` | `self.tools             .iter()             .filter(visible)             .filter` | [426](../../src/builtin.rs#L426) | receiver-type-required |
| `projection` | `self.tools             .iter()             .filter` | [426](../../src/builtin.rs#L426) | receiver-type-required |
| `projection` | `self.tools             .iter` | [426](../../src/builtin.rs#L426) | receiver-type-required |
| `projection` | `is_resident` | [429](../../src/builtin.rs#L429), [434](../../src/builtin.rs#L434) | [tools::builtin::is_resident](../../src/builtin.rs#L484) |
| `projection` | `self.tools                     .iter()                     .filter(visible)                     .filter` | [431](../../src/builtin.rs#L431) | receiver-type-required |
| `projection` | `self.tools                     .iter()                     .filter` | [431](../../src/builtin.rs#L431) | receiver-type-required |
| `projection` | `self.tools                     .iter` | [431](../../src/builtin.rs#L431) | receiver-type-required |
| `interactive_names` | `self             .tools             .iter()             .filter(&#124;tool&#124; {                 !matches!(                     tool.availability,                     Availability::PlanMode                         &#124; Availability::RoleCompactor                         &#124; Availability::RoleSubagent                         &#124; Availability::RoleValidator                 )             })             .map(&#124;tool&#124; tool.name.clone())             .collect::<Vec<_>>` | [446](../../src/builtin.rs#L446) | receiver-type-required |
| `interactive_names` | `self             .tools             .iter()             .filter(&#124;tool&#124; {                 !matches!(                     tool.availability,                     Availability::PlanMode                         &#124; Availability::RoleCompactor                         &#124; Availability::RoleSubagent                         &#124; Availability::RoleValidator                 )             })             .map` | [446](../../src/builtin.rs#L446) | receiver-type-required |
| `interactive_names` | `self             .tools             .iter()             .filter` | [446](../../src/builtin.rs#L446) | receiver-type-required |
| `interactive_names` | `self             .tools             .iter` | [446](../../src/builtin.rs#L446) | receiver-type-required |
| `interactive_names` | `tool.name.clone` | [458](../../src/builtin.rs#L458) | receiver-type-required |
| `interactive_names` | `names.sort` | [460](../../src/builtin.rs#L460) | receiver-type-required |
| `interactive_names` | `names.dedup` | [461](../../src/builtin.rs#L461) | receiver-type-required |
| `catalog_digest` | `serde_json_canonicalizer::to_vec` | [466](../../src/builtin.rs#L466) | external-constructor-callback-or-unresolved |
| `catalog_digest` | `Ok` | [467](../../src/builtin.rs#L467) | external-constructor-callback-or-unresolved |
| `availability_visible` | `context.selected_tools.contains` | [474](../../src/builtin.rs#L474) | receiver-type-required |
| `validate_name` | `name.is_empty` | [494](../../src/builtin.rs#L494) | receiver-type-required |
| `validate_name` | `name.chars().any` | [494](../../src/builtin.rs#L494) | receiver-type-required |
| `validate_name` | `name.chars` | [494](../../src/builtin.rs#L494) | receiver-type-required |
| `validate_name` | `ch.is_ascii_control` | [494](../../src/builtin.rs#L494) | receiver-type-required |
| `validate_name` | `Err` | [495](../../src/builtin.rs#L495) | external-constructor-callback-or-unresolved |
| `validate_name` | `BuiltinError::Invalid` | [495](../../src/builtin.rs#L495) | external-constructor-callback-or-unresolved |
| `validate_name` | `Ok` | [499](../../src/builtin.rs#L499) | external-constructor-callback-or-unresolved |
| `index_by_name` | `manifest         .tools         .iter()         .map(&#124;tool&#124; (tool.name.as_str(), tool))         .collect` | [514](../../src/builtin.rs#L514) | receiver-type-required |
| `index_by_name` | `manifest         .tools         .iter()         .map` | [514](../../src/builtin.rs#L514) | receiver-type-required |
| `index_by_name` | `manifest         .tools         .iter` | [514](../../src/builtin.rs#L514) | receiver-type-required |
| `index_by_name` | `tool.name.as_str` | [517](../../src/builtin.rs#L517) | receiver-type-required |
