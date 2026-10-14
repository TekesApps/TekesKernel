# tools::preset

[Package atlas](index.md) · [Source](../../src/preset.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::preset::PRESET_FORMAT](../../src/preset.rs#L28) | const_item | `pub` |  |
| [tools::preset::PRESET_RENDER_VERSION](../../src/preset.rs#L30) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_LAYERS](../../src/preset.rs#L32) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_ID_BYTES](../../src/preset.rs#L33) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_VERSION_BYTES](../../src/preset.rs#L34) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_IDENTITY_BYTES](../../src/preset.rs#L35) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_DUTY_BYTES](../../src/preset.rs#L36) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_TEXT_BYTES](../../src/preset.rs#L38) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_TOOLS](../../src/preset.rs#L39) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_MCP_SERVERS](../../src/preset.rs#L40) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_MCP_TOOLS](../../src/preset.rs#L41) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_SKILLS](../../src/preset.rs#L42) | const_item | `pub` |  |
| [tools::preset::PresetBase](../../src/preset.rs#L47) | enum_item | `pub` |  |
| [tools::preset::PresetBase::as_str](../../src/preset.rs#L55) | function_item | `pub` |  |
| [tools::preset::PresetBase::parse](../../src/preset.rs#L63) | function_item | `pub` |  |
| [tools::preset::PresetBase::identity](../../src/preset.rs#L71) | function_item | `private` |  |
| [tools::preset::PresetBase::rules](../../src/preset.rs#L78) | function_item | `private` |  |
| [tools::preset::KernelLayer](../../src/preset.rs#L88) | struct_item | `pub` |  |
| [tools::preset::KERNEL_LAYERS](../../src/preset.rs#L96) | const_item | `pub` |  |
| [tools::preset::kernel_layer](../../src/preset.rs#L104) | function_item | `private` |  |
| [tools::preset::PresetLayer](../../src/preset.rs#L111) | enum_item | `pub` |  |
| [tools::preset::PresetLayer::id](../../src/preset.rs#L128) | function_item | `pub` |  |
| [tools::preset::PresetLayer::version](../../src/preset.rs#L135) | function_item | `pub` |  |
| [tools::preset::PresetLayer::kind](../../src/preset.rs#L142) | function_item | `pub` |  |
| [tools::preset::PresetLayer::identity](../../src/preset.rs#L149) | function_item | `private` |  |
| [tools::preset::PresetLayer::duty](../../src/preset.rs#L156) | function_item | `private` |  |
| [tools::preset::ResolvedPreset](../../src/preset.rs#L167) | struct_item | `pub` |  |
| [tools::preset::PresetDefaults](../../src/preset.rs#L194) | struct_item | `pub` |  |
| [tools::preset::PresetMcpRef](../../src/preset.rs#L204) | struct_item | `pub` |  |
| [tools::preset::PresetEnvironment](../../src/preset.rs#L212) | trait_item | `pub` |  |
| [tools::preset::PresetEnvironment::permitted_tools](../../src/preset.rs#L214) | function_item | `private` |  |
| [tools::preset::PresetEnvironment::mcp_server_digest](../../src/preset.rs#L218) | function_item | `private` |  |
| [tools::preset::PresetEnvironment::mcp_tool_name](../../src/preset.rs#L222) | function_item | `private` |  |
| [tools::preset::PresetEnvironment::skill_known](../../src/preset.rs#L226) | function_item | `private` |  |
| [tools::preset::PresetEnvironment::model_available](../../src/preset.rs#L231) | function_item | `private` |  |
| [tools::preset::NoPresetEnvironment](../../src/preset.rs#L237) | struct_item | `pub` |  |
| [tools::preset::PresetInvalidReason](../../src/preset.rs#L243) | enum_item | `pub` |  |
| [tools::preset::PresetInvalidReason::as_str](../../src/preset.rs#L265) | function_item | `pub` |  |
| [tools::preset::ALL](../../src/preset.rs#L287) | const_item | `pub` |  |
| [tools::preset::PresetInvalid](../../src/preset.rs#L309) | struct_item | `pub` |  |
| [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) | function_item | `private` |  |
| [tools::preset::PresetInvalid::detail](../../src/preset.rs#L328) | function_item | `pub` |  |
| [tools::preset::PresetError](../../src/preset.rs#L344) | enum_item | `pub` |  |
| [tools::preset::PresetError::from](../../src/preset.rs#L350) | function_item | `private` |  |
| [tools::preset::SECRET_KEYS](../../src/preset.rs#L358) | const_item | `private` |  |
| [tools::preset::SECRET_PREFIXES](../../src/preset.rs#L378) | const_item | `private` |  |
| [tools::preset::looks_like_secret](../../src/preset.rs#L389) | function_item | `private` |  |
| [tools::preset::find_secret](../../src/preset.rs#L406) | function_item | `private` |  |
| [tools::preset::valid_id](../../src/preset.rs#L429) | function_item | `private` |  |
| [tools::preset::valid_version](../../src/preset.rs#L438) | function_item | `private` |  |
| [tools::preset::object_members](../../src/preset.rs#L444) | function_item | `private` |  |
| [tools::preset::string_member](../../src/preset.rs#L460) | function_item | `private` |  |
| [tools::preset::Selection](../../src/preset.rs#L477) | struct_item | `private` |  |
| [tools::preset::string_array](../../src/preset.rs#L487) | function_item | `private` |  |
| [tools::preset::Selection::collect](../../src/preset.rs#L495) | function_item | `private` |  |
| [tools::preset::Selection::finish](../../src/preset.rs#L587) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::resolve_request](../../src/preset.rs#L721) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::from_identity](../../src/preset.rs#L770) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::from_parts](../../src/preset.rs#L781) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::compute_digest](../../src/preset.rs#L841) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::from_genesis](../../src/preset.rs#L855) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L885) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::preset_text](../../src/preset.rs#L907) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::render](../../src/preset.rs#L918) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::summary](../../src/preset.rs#L937) | function_item | `pub` |  |
| [tools::preset::PresetReason](../../src/preset.rs#L948) | type_item | `private` |  |
| [tools::preset::resolve_layer](../../src/preset.rs#L950) | function_item | `private` |  |
| [tools::preset::describe_value](../../src/preset.rs#L1055) | function_item | `pub` |  |
| [tools::preset::tests::resolve](../../src/preset.rs#L1091) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture](../../src/preset.rs#L1095) | struct_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture::permitted_tools](../../src/preset.rs#L1100) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture::mcp_server_digest](../../src/preset.rs#L1103) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture::mcp_tool_name](../../src/preset.rs#L1106) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture::skill_known](../../src/preset.rs#L1109) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::Fixture::model_available](../../src/preset.rs#L1112) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::resolve_in](../../src/preset.rs#L1119) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::invalid](../../src/preset.rs#L1133) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::sugar_renders_exactly_like_the_identity_profile](../../src/preset.rs#L1141) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::blank_base_drops_identity_and_rules](../../src/preset.rs#L1159) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::inline_identity_replaces_the_base_identity_but_not_its_rules](../../src/preset.rs#L1177) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::inline_identity_wins_over_a_kernel_layer_identity](../../src/preset.rs#L1208) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::refusals_name_reason_layer_and_field](../../src/preset.rs#L1224) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::tools_mcp_and_skills_resolve_against_the_environment](../../src/preset.rs#L1276) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::model_defaults_resolve_with_top_level_then_last_layer_precedence](../../src/preset.rs#L1386) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::secrets_and_definitions_are_refused_before_shape_checks](../../src/preset.rs#L1452) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::shape_errors_are_bad_requests](../../src/preset.rs#L1467) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::genesis_round_trip_checks_digest_and_layers](../../src/preset.rs#L1485) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Map` | `serde_json::Map` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `BuiltinManifest` | `crate::builtin::BuiltinManifest` | `private` |
| `CODING_IDENTITY` | `crate::guidance::CODING_IDENTITY` | `private` |
| `CODING_RULES` | `crate::guidance::CODING_RULES` | `private` |
| `GENERAL_IDENTITY` | `crate::guidance::GENERAL_IDENTITY` | `private` |
| `GENERAL_RULES` | `crate::guidance::GENERAL_RULES` | `private` |
| `HARNESS_IDENTITY` | `crate::guidance::HARNESS_IDENTITY` | `private` |
| `IdentityProfile` | `crate::guidance::IdentityProfile` | `private` |
| `WORKING_DIRECTORY` | `crate::guidance::WORKING_DIRECTORY` | `private` |
| `*` | `super::*` | `private` |
| `root_system_instructions` | `crate::guidance::root_system_instructions` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::preset::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::preset::kernel_layer"]
  n1["tools::preset::PresetLayer::id"]
  n2["tools::preset::PresetLayer::version"]
  n3["tools::preset::PresetLayer::kind"]
  n4["tools::preset::PresetLayer::identity"]
  n5["tools::preset::PresetLayer::duty"]
  n6["tools::preset::PresetEnvironment::permitted_tools"]
  n7["tools::preset::PresetEnvironment::mcp_server_digest"]
  n8["tools::preset::PresetEnvironment::mcp_tool_name"]
  n9["tools::preset::PresetEnvironment::skill_known"]
  n10["tools::preset::PresetEnvironment::model_available"]
  n11["tools::preset::PresetInvalidReason::as_str"]
  n12["tools::preset::PresetInvalid::new"]
  n13["tools::preset::PresetInvalid::detail"]
  n14["tools::preset::PresetError::from"]
  n15["tools::preset::looks_like_secret"]
  n16["tools::preset::PresetBase::as_str"]
  n17["tools::preset::PresetBase::parse"]
  n18["tools::preset::PresetBase::identity"]
  n19["tools::preset::PresetBase::rules"]
  n4 --> n0
  n5 --> n0
```

</details>

<details><summary>Functions 21–39: 25 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::builtin::BuiltinManifest::compiled"]
  n1["tools::preset::kernel_layer"]
  n2["tools::preset::describe_value"]
  n3["tools::preset::PresetInvalid::new"]
  n4["tools::preset::looks_like_secret"]
  n5["tools::preset::find_secret"]
  n6["tools::preset::valid_id"]
  n7["tools::preset::valid_version"]
  n8["tools::preset::object_members"]
  n9["tools::preset::string_member"]
  n10["tools::preset::string_array"]
  n11["tools::preset::Selection::collect"]
  n12["tools::preset::Selection::finish"]
  n13["tools::preset::ResolvedPreset::resolve_request"]
  n14["tools::preset::ResolvedPreset::from_identity"]
  n15["tools::preset::ResolvedPreset::from_parts"]
  n16["tools::preset::ResolvedPreset::compute_digest"]
  n17["tools::preset::ResolvedPreset::from_genesis"]
  n18["tools::preset::ResolvedPreset::owned_sections"]
  n19["tools::preset::ResolvedPreset::preset_text"]
  n20["tools::preset::ResolvedPreset::render"]
  n21["tools::preset::ResolvedPreset::summary"]
  n22["tools::preset::resolve_layer"]
  n5 --> n4
  n5 --> n5
  n11 --> n0
  n11 --> n3
  n11 --> n8
  n11 --> n9
  n11 --> n10
  n12 --> n0
  n12 --> n3
  n13 --> n3
  n13 --> n5
  n13 --> n8
  n13 --> n15
  n13 --> n22
  n14 --> n15
  n15 --> n3
  n17 --> n15
  n19 --> n18
  n20 --> n18
  n22 --> n1
  n22 --> n3
  n22 --> n6
  n22 --> n7
  n22 --> n8
  n22 --> n9
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse` | `Some` | [65](../../src/preset.rs#L65), [66](../../src/preset.rs#L66) | external-constructor-callback-or-unresolved |
| `identity` | `Some` | [73](../../src/preset.rs#L73) | external-constructor-callback-or-unresolved |
| `rules` | `Some` | [80](../../src/preset.rs#L80) | external-constructor-callback-or-unresolved |
| `kernel_layer` | `KERNEL_LAYERS.iter().find` | [105](../../src/preset.rs#L105) | receiver-type-required |
| `kernel_layer` | `KERNEL_LAYERS.iter` | [105](../../src/preset.rs#L105) | receiver-type-required |
| `identity` | `kernel_layer(id).map` | [151](../../src/preset.rs#L151) | receiver-type-required |
| `identity` | `kernel_layer` | [151](../../src/preset.rs#L151) | [tools::preset::kernel_layer](../../src/preset.rs#L104) |
| `identity` | `identity.as_deref` | [152](../../src/preset.rs#L152) | receiver-type-required |
| `duty` | `kernel_layer(id).map` | [158](../../src/preset.rs#L158) | receiver-type-required |
| `duty` | `kernel_layer` | [158](../../src/preset.rs#L158) | [tools::preset::kernel_layer](../../src/preset.rs#L104) |
| `duty` | `duty.as_deref` | [159](../../src/preset.rs#L159) | receiver-type-required |
| `new` | `Some` | [322](../../src/preset.rs#L322) | external-constructor-callback-or-unresolved |
| `new` | `field.into` | [322](../../src/preset.rs#L322) | receiver-type-required |
| `detail` | `Map::new` | [329](../../src/preset.rs#L329) | external-constructor-callback-or-unresolved |
| `detail` | `detail.insert` | [330](../../src/preset.rs#L330), [332](../../src/preset.rs#L332), [335](../../src/preset.rs#L335) | receiver-type-required |
| `detail` | `"reason".to_owned` | [330](../../src/preset.rs#L330) | receiver-type-required |
| `detail` | `"layer".to_owned` | [332](../../src/preset.rs#L332) | receiver-type-required |
| `detail` | `"field".to_owned` | [335](../../src/preset.rs#L335) | receiver-type-required |
| `detail` | `Value::Object` | [337](../../src/preset.rs#L337) | external-constructor-callback-or-unresolved |
| `from` | `Self::Invalid` | [351](../../src/preset.rs#L351) | external-constructor-callback-or-unresolved |
| `looks_like_secret` | `text.contains` | [390](../../src/preset.rs#L390) | receiver-type-required |
| `looks_like_secret` | `text.split(&#124;ch: char&#124; ch.is_whitespace() &#124;&#124; matches!(ch, '"' &#124; '\'' &#124; ''' &#124; '(' &#124; ')' &#124; '='))         .any` | [393](../../src/preset.rs#L393) | receiver-type-required |
| `looks_like_secret` | `text.split` | [393](../../src/preset.rs#L393) | receiver-type-required |
| `looks_like_secret` | `ch.is_whitespace` | [393](../../src/preset.rs#L393) | receiver-type-required |
| `looks_like_secret` | `SECRET_PREFIXES.iter().any` | [395](../../src/preset.rs#L395) | receiver-type-required |
| `looks_like_secret` | `SECRET_PREFIXES.iter` | [395](../../src/preset.rs#L395) | receiver-type-required |
| `looks_like_secret` | `token.strip_prefix(prefix).is_some_and` | [396](../../src/preset.rs#L396) | receiver-type-required |
| `looks_like_secret` | `token.strip_prefix` | [396](../../src/preset.rs#L396) | receiver-type-required |
| `looks_like_secret` | `tail.len` | [397](../../src/preset.rs#L397) | receiver-type-required |
| `looks_like_secret` | `tail                             .chars()                             .all` | [398](../../src/preset.rs#L398) | receiver-type-required |
| `looks_like_secret` | `tail                             .chars` | [398](../../src/preset.rs#L398) | receiver-type-required |
| `looks_like_secret` | `ch.is_ascii_alphanumeric` | [400](../../src/preset.rs#L400) | receiver-type-required |
| `find_secret` | `object.iter().find_map` | [408](../../src/preset.rs#L408) | receiver-type-required |
| `find_secret` | `object.iter` | [408](../../src/preset.rs#L408) | receiver-type-required |
| `find_secret` | `path.is_empty` | [409](../../src/preset.rs#L409) | receiver-type-required |
| `find_secret` | `key.clone` | [410](../../src/preset.rs#L410) | receiver-type-required |
| `find_secret` | `SECRET_KEYS.contains` | [414](../../src/preset.rs#L414) | receiver-type-required |
| `find_secret` | `key.as_str` | [414](../../src/preset.rs#L414) | receiver-type-required |
| `find_secret` | `Some` | [415](../../src/preset.rs#L415) | external-constructor-callback-or-unresolved |
| `find_secret` | `find_secret` | [417](../../src/preset.rs#L417), [423](../../src/preset.rs#L423) | [tools::preset::find_secret](../../src/preset.rs#L406) |
| `find_secret` | `items             .iter()             .enumerate()             .find_map` | [420](../../src/preset.rs#L420) | receiver-type-required |
| `find_secret` | `items             .iter()             .enumerate` | [420](../../src/preset.rs#L420) | receiver-type-required |
| `find_secret` | `items             .iter` | [420](../../src/preset.rs#L420) | receiver-type-required |
| `find_secret` | `looks_like_secret(text).then` | [424](../../src/preset.rs#L424) | receiver-type-required |
| `find_secret` | `looks_like_secret` | [424](../../src/preset.rs#L424) | [tools::preset::looks_like_secret](../../src/preset.rs#L389) |
| `find_secret` | `path.to_owned` | [424](../../src/preset.rs#L424) | receiver-type-required |
| `valid_id` | `value.is_empty` | [430](../../src/preset.rs#L430) | receiver-type-required |
| `valid_id` | `value.len` | [431](../../src/preset.rs#L431) | receiver-type-required |
| `valid_id` | `value             .bytes()             .all` | [432](../../src/preset.rs#L432) | receiver-type-required |
| `valid_id` | `value             .bytes` | [432](../../src/preset.rs#L432) | receiver-type-required |
| `valid_id` | `byte.is_ascii_lowercase` | [434](../../src/preset.rs#L434) | receiver-type-required |
| `valid_id` | `byte.is_ascii_digit` | [434](../../src/preset.rs#L434) | receiver-type-required |
| `valid_id` | `value.as_bytes()[0].is_ascii_alphanumeric` | [435](../../src/preset.rs#L435) | receiver-type-required |
| `valid_id` | `value.as_bytes` | [435](../../src/preset.rs#L435) | receiver-type-required |
| `valid_version` | `value.is_empty` | [439](../../src/preset.rs#L439) | receiver-type-required |
| `valid_version` | `value.len` | [440](../../src/preset.rs#L440) | receiver-type-required |
| `valid_version` | `value.bytes().all` | [441](../../src/preset.rs#L441) | receiver-type-required |
| `valid_version` | `value.bytes` | [441](../../src/preset.rs#L441) | receiver-type-required |
| `valid_version` | `byte.is_ascii_graphic` | [441](../../src/preset.rs#L441) | receiver-type-required |
| `object_members` | `value         .as_object()         .ok_or_else` | [449](../../src/preset.rs#L449) | receiver-type-required |
| `object_members` | `value         .as_object` | [449](../../src/preset.rs#L449) | receiver-type-required |
| `object_members` | `PresetError::BadRequest` | [451](../../src/preset.rs#L451), [453](../../src/preset.rs#L453) | external-constructor-callback-or-unresolved |
| `object_members` | `object.keys().find` | [452](../../src/preset.rs#L452) | receiver-type-required |
| `object_members` | `object.keys` | [452](../../src/preset.rs#L452) | receiver-type-required |
| `object_members` | `allowed.contains` | [452](../../src/preset.rs#L452) | receiver-type-required |
| `object_members` | `key.as_str` | [452](../../src/preset.rs#L452) | receiver-type-required |
| `object_members` | `Err` | [453](../../src/preset.rs#L453) | external-constructor-callback-or-unresolved |
| `object_members` | `Ok` | [457](../../src/preset.rs#L457) | external-constructor-callback-or-unresolved |
| `string_member` | `object.get` | [465](../../src/preset.rs#L465) | receiver-type-required |
| `string_member` | `Ok` | [466](../../src/preset.rs#L466), [467](../../src/preset.rs#L467) | external-constructor-callback-or-unresolved |
| `string_member` | `Some` | [467](../../src/preset.rs#L467) | external-constructor-callback-or-unresolved |
| `string_member` | `Err` | [468](../../src/preset.rs#L468) | external-constructor-callback-or-unresolved |
| `string_member` | `PresetError::BadRequest` | [468](../../src/preset.rs#L468) | external-constructor-callback-or-unresolved |
| `string_array` | `value         .as_array()         .and_then(&#124;items&#124; items.iter().map(Value::as_str).collect::<Option<Vec<_>>>())         .ok_or_else` | [488](../../src/preset.rs#L488) | receiver-type-required |
| `string_array` | `value         .as_array()         .and_then` | [488](../../src/preset.rs#L488) | receiver-type-required |
| `string_array` | `value         .as_array` | [488](../../src/preset.rs#L488) | receiver-type-required |
| `string_array` | `items.iter().map(Value::as_str).collect::<Option<Vec<_>>>` | [490](../../src/preset.rs#L490) | receiver-type-required |
| `string_array` | `items.iter().map` | [490](../../src/preset.rs#L490) | receiver-type-required |
| `string_array` | `items.iter` | [490](../../src/preset.rs#L490) | receiver-type-required |
| `string_array` | `PresetError::BadRequest` | [491](../../src/preset.rs#L491) | external-constructor-callback-or-unresolved |
| `collect` | `layer.map` | [501](../../src/preset.rs#L501) | receiver-type-required |
| `collect` | `object.get` | [502](../../src/preset.rs#L502), [523](../../src/preset.rs#L523), [551](../../src/preset.rs#L551), [562](../../src/preset.rs#L562) | receiver-type-required |
| `collect` | `BuiltinManifest::compiled` | [503](../../src/preset.rs#L503) | [tools::builtin::BuiltinManifest::compiled](../../src/builtin.rs#L315) |
| `collect` | `manifest.interactive_names` | [504](../../src/preset.rs#L504) | receiver-type-required |
| `collect` | `manifest.fixed_names` | [505](../../src/preset.rs#L505) | receiver-type-required |
| `collect` | `self.builtins.get_or_insert_with` | [506](../../src/preset.rs#L506) | receiver-type-required |
| `collect` | `string_array(tools, &format!("{path}.tools"))?                 .into_iter()                 .enumerate` | [507](../../src/preset.rs#L507) | receiver-type-required |
| `collect` | `string_array(tools, &format!("{path}.tools"))?                 .into_iter` | [507](../../src/preset.rs#L507) | receiver-type-required |
| `collect` | `string_array` | [507](../../src/preset.rs#L507), [535](../../src/preset.rs#L535), [553](../../src/preset.rs#L553) | [tools::preset::string_array](../../src/preset.rs#L487) |
| `collect` | `interactive.iter().any` | [512](../../src/preset.rs#L512) | receiver-type-required |
| `collect` | `interactive.iter` | [512](../../src/preset.rs#L512) | receiver-type-required |
| `collect` | `builtins.insert` | [513](../../src/preset.rs#L513) | receiver-type-required |
| `collect` | `name.to_owned` | [513](../../src/preset.rs#L513), [558](../../src/preset.rs#L558) | receiver-type-required |
| `collect` | `fixed.contains` | [514](../../src/preset.rs#L514) | receiver-type-required |
| `collect` | `Err` | [515](../../src/preset.rs#L515), [517](../../src/preset.rs#L517), [540](../../src/preset.rs#L540) | external-constructor-callback-or-unresolved |
| `collect` | `PresetInvalid::new(PresetReason::RoleTool, label(), field).into` | [515](../../src/preset.rs#L515) | receiver-type-required |
| `collect` | `PresetInvalid::new` | [515](../../src/preset.rs#L515), [518](../../src/preset.rs#L518) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) |
| `collect` | `label` | [515](../../src/preset.rs#L515), [518](../../src/preset.rs#L518), [547](../../src/preset.rs#L547), [559](../../src/preset.rs#L559), [579](../../src/preset.rs#L579) | external-constructor-callback-or-unresolved |
| `collect` | `PresetInvalid::new(PresetReason::UnknownTool, label(), field).into` | [518](../../src/preset.rs#L518) | receiver-type-required |
| `collect` | `mcp                 .as_array()                 .ok_or_else` | [524](../../src/preset.rs#L524) | receiver-type-required |
| `collect` | `mcp                 .as_array` | [524](../../src/preset.rs#L524) | receiver-type-required |
| `collect` | `PresetError::BadRequest` | [526](../../src/preset.rs#L526), [533](../../src/preset.rs#L533), [540](../../src/preset.rs#L540), [570](../../src/preset.rs#L570) | external-constructor-callback-or-unresolved |
| `collect` | `items.iter().enumerate` | [527](../../src/preset.rs#L527) | receiver-type-required |
| `collect` | `items.iter` | [527](../../src/preset.rs#L527) | receiver-type-required |
| `collect` | `object_members` | [529](../../src/preset.rs#L529), [565](../../src/preset.rs#L565) | [tools::preset::object_members](../../src/preset.rs#L444) |
| `collect` | `string_member(entry, "server", &item_path)?                     .filter(&#124;server&#124; !server.is_empty())                     .ok_or_else` | [530](../../src/preset.rs#L530) | receiver-type-required |
| `collect` | `string_member(entry, "server", &item_path)?                     .filter` | [530](../../src/preset.rs#L530) | receiver-type-required |
| `collect` | `string_member` | [530](../../src/preset.rs#L530), [567](../../src/preset.rs#L567), [575](../../src/preset.rs#L575) | [tools::preset::string_member](../../src/preset.rs#L460) |
| `collect` | `server.is_empty` | [531](../../src/preset.rs#L531) | receiver-type-required |
| `collect` | `entry.get("tools").unwrap_or` | [536](../../src/preset.rs#L536) | receiver-type-required |
| `collect` | `entry.get` | [536](../../src/preset.rs#L536) | receiver-type-required |
| `collect` | `tools.is_empty` | [539](../../src/preset.rs#L539) | receiver-type-required |
| `collect` | `tools.iter().any` | [539](../../src/preset.rs#L539) | receiver-type-required |
| `collect` | `tools.iter` | [539](../../src/preset.rs#L539) | receiver-type-required |
| `collect` | `tool.is_empty` | [539](../../src/preset.rs#L539) | receiver-type-required |
| `collect` | `self                     .mcp                     .entry(server.to_owned())                     .or_insert_with` | [544](../../src/preset.rs#L544) | receiver-type-required |
| `collect` | `self                     .mcp                     .entry` | [544](../../src/preset.rs#L544) | receiver-type-required |
| `collect` | `server.to_owned` | [546](../../src/preset.rs#L546) | receiver-type-required |
| `collect` | `BTreeSet::new` | [547](../../src/preset.rs#L547) | external-constructor-callback-or-unresolved |
| `collect` | `slot.0.extend` | [548](../../src/preset.rs#L548) | receiver-type-required |
| `collect` | `tools.into_iter().map` | [548](../../src/preset.rs#L548) | receiver-type-required |
| `collect` | `tools.into_iter` | [548](../../src/preset.rs#L548) | receiver-type-required |
| `collect` | `self.skills.get_or_insert_with` | [552](../../src/preset.rs#L552) | receiver-type-required |
| `collect` | `string_array(skills, &format!("{path}.skills"))?                 .into_iter()                 .enumerate` | [553](../../src/preset.rs#L553) | receiver-type-required |
| `collect` | `string_array(skills, &format!("{path}.skills"))?                 .into_iter` | [553](../../src/preset.rs#L553) | receiver-type-required |
| `collect` | `known                     .entry(name.to_owned())                     .or_insert_with` | [557](../../src/preset.rs#L557) | receiver-type-required |
| `collect` | `known                     .entry` | [557](../../src/preset.rs#L557) | receiver-type-required |
| `collect` | `string_member(entry, key, &field)?                     .filter(&#124;value&#124; !value.is_empty())                     .map(str::to_owned)                     .ok_or_else` | [567](../../src/preset.rs#L567) | receiver-type-required |
| `collect` | `string_member(entry, key, &field)?                     .filter(&#124;value&#124; !value.is_empty())                     .map` | [567](../../src/preset.rs#L567) | receiver-type-required |
| `collect` | `string_member(entry, key, &field)?                     .filter` | [567](../../src/preset.rs#L567) | receiver-type-required |
| `collect` | `value.is_empty` | [568](../../src/preset.rs#L568) | receiver-type-required |
| `collect` | `required` | [573](../../src/preset.rs#L573), [574](../../src/preset.rs#L574) | external-constructor-callback-or-unresolved |
| `collect` | `string_member(entry, "reasoningEffort", &field)?                     .map` | [575](../../src/preset.rs#L575) | receiver-type-required |
| `collect` | `layer.is_none` | [578](../../src/preset.rs#L578), [580](../../src/preset.rs#L580) | receiver-type-required |
| `collect` | `Some` | [579](../../src/preset.rs#L579) | external-constructor-callback-or-unresolved |
| `collect` | `Ok` | [583](../../src/preset.rs#L583) | external-constructor-callback-or-unresolved |
| `finish` | `PresetError::from` | [600](../../src/preset.rs#L600), [626](../../src/preset.rs#L626) | external-constructor-callback-or-unresolved |
| `finish` | `PresetInvalid::new` | [600](../../src/preset.rs#L600), [626](../../src/preset.rs#L626), [642](../../src/preset.rs#L642), [646](../../src/preset.rs#L646), [686](../../src/preset.rs#L686), [703](../../src/preset.rs#L703) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) |
| `finish` | `field.to_owned` | [603](../../src/preset.rs#L603), [629](../../src/preset.rs#L629) | receiver-type-required |
| `finish` | `self             .builtins             .as_ref()             .is_some_and` | [606](../../src/preset.rs#L606) | receiver-type-required |
| `finish` | `self             .builtins             .as_ref` | [606](../../src/preset.rs#L606) | receiver-type-required |
| `finish` | `tools.len` | [609](../../src/preset.rs#L609), [638](../../src/preset.rs#L638) | receiver-type-required |
| `finish` | `Err` | [611](../../src/preset.rs#L611), [614](../../src/preset.rs#L614), [621](../../src/preset.rs#L621), [626](../../src/preset.rs#L626), [639](../../src/preset.rs#L639), [686](../../src/preset.rs#L686), [703](../../src/preset.rs#L703) | external-constructor-callback-or-unresolved |
| `finish` | `too_many` | [611](../../src/preset.rs#L611), [614](../../src/preset.rs#L614), [621](../../src/preset.rs#L621), [639](../../src/preset.rs#L639) | external-constructor-callback-or-unresolved |
| `finish` | `self.mcp.len` | [613](../../src/preset.rs#L613), [636](../../src/preset.rs#L636) | receiver-type-required |
| `finish` | `self             .skills             .as_ref()             .is_some_and` | [616](../../src/preset.rs#L616) | receiver-type-required |
| `finish` | `self             .skills             .as_ref` | [616](../../src/preset.rs#L616) | receiver-type-required |
| `finish` | `skills.len` | [619](../../src/preset.rs#L619) | receiver-type-required |
| `finish` | `environment.permitted_tools` | [623](../../src/preset.rs#L623) | receiver-type-required |
| `finish` | `permitted.is_some_and` | [625](../../src/preset.rs#L625) | receiver-type-required |
| `finish` | `permitted.contains` | [625](../../src/preset.rs#L625) | receiver-type-required |
| `finish` | `Ok` | [632](../../src/preset.rs#L632), [709](../../src/preset.rs#L709) | external-constructor-callback-or-unresolved |
| `finish` | `BTreeSet::new` | [635](../../src/preset.rs#L635) | external-constructor-callback-or-unresolved |
| `finish` | `Vec::with_capacity` | [636](../../src/preset.rs#L636) | external-constructor-callback-or-unresolved |
| `finish` | `environment.mcp_server_digest(&server).ok_or_else` | [641](../../src/preset.rs#L641) | receiver-type-required |
| `finish` | `environment.mcp_server_digest` | [641](../../src/preset.rs#L641) | receiver-type-required |
| `finish` | `layer.clone` | [642](../../src/preset.rs#L642), [646](../../src/preset.rs#L646), [648](../../src/preset.rs#L648), [688](../../src/preset.rs#L688) | receiver-type-required |
| `finish` | `field.clone` | [642](../../src/preset.rs#L642), [646](../../src/preset.rs#L646), [689](../../src/preset.rs#L689) | receiver-type-required |
| `finish` | `environment.mcp_tool_name(&server, tool).ok_or_else` | [645](../../src/preset.rs#L645) | receiver-type-required |
| `finish` | `environment.mcp_tool_name` | [645](../../src/preset.rs#L645) | receiver-type-required |
| `finish` | `check_permitted` | [648](../../src/preset.rs#L648), [661](../../src/preset.rs#L661) | external-constructor-callback-or-unresolved |
| `finish` | `projected.insert` | [649](../../src/preset.rs#L649) | receiver-type-required |
| `finish` | `references.push` | [651](../../src/preset.rs#L651) | receiver-type-required |
| `finish` | `tools.into_iter().collect` | [654](../../src/preset.rs#L654), [710](../../src/preset.rs#L710) | receiver-type-required |
| `finish` | `tools.into_iter` | [654](../../src/preset.rs#L654), [710](../../src/preset.rs#L710) | receiver-type-required |
| `finish` | `self.builtins.is_some` | [657](../../src/preset.rs#L657) | receiver-type-required |
| `finish` | `projected.is_empty` | [657](../../src/preset.rs#L657) | receiver-type-required |
| `finish` | `BuiltinManifest::compiled()                     .interactive_names()                     .into_iter()                     .collect` | [665](../../src/preset.rs#L665) | receiver-type-required |
| `finish` | `BuiltinManifest::compiled()                     .interactive_names()                     .into_iter` | [665](../../src/preset.rs#L665) | receiver-type-required |
| `finish` | `BuiltinManifest::compiled()                     .interactive_names` | [665](../../src/preset.rs#L665) | receiver-type-required |
| `finish` | `BuiltinManifest::compiled` | [665](../../src/preset.rs#L665) | [tools::builtin::BuiltinManifest::compiled](../../src/builtin.rs#L315) |
| `finish` | `Some` | [670](../../src/preset.rs#L670), [693](../../src/preset.rs#L693), [705](../../src/preset.rs#L705) | external-constructor-callback-or-unresolved |
| `finish` | `builtins                     .into_iter()                     .chain(projected)                     .collect::<BTreeSet<_>>` | [671](../../src/preset.rs#L671) | receiver-type-required |
| `finish` | `builtins                     .into_iter()                     .chain` | [671](../../src/preset.rs#L671) | receiver-type-required |
| `finish` | `builtins                     .into_iter` | [671](../../src/preset.rs#L671) | receiver-type-required |
| `finish` | `skills                     .iter()                     .find` | [682](../../src/preset.rs#L682) | receiver-type-required |
| `finish` | `skills                     .iter` | [682](../../src/preset.rs#L682) | receiver-type-required |
| `finish` | `environment.skill_known` | [684](../../src/preset.rs#L684) | receiver-type-required |
| `finish` | `PresetInvalid::new(                         PresetReason::UnknownSkill,                         layer.clone(),                         field.clone(),                     )                     .into` | [686](../../src/preset.rs#L686) | receiver-type-required |
| `finish` | `skills.into_keys().collect` | [693](../../src/preset.rs#L693) | receiver-type-required |
| `finish` | `skills.into_keys` | [693](../../src/preset.rs#L693) | receiver-type-required |
| `finish` | `environment.model_available` | [698](../../src/preset.rs#L698) | receiver-type-required |
| `finish` | `defaults.reasoning_effort.as_deref` | [701](../../src/preset.rs#L701) | receiver-type-required |
| `finish` | `PresetInvalid::new(PresetReason::UnknownModel, layer, field).into` | [703](../../src/preset.rs#L703) | receiver-type-required |
| `finish` | `tools.map` | [710](../../src/preset.rs#L710) | receiver-type-required |
| `resolve_request` | `find_secret` | [726](../../src/preset.rs#L726) | [tools::preset::find_secret](../../src/preset.rs#L406) |
| `resolve_request` | `Err` | [727](../../src/preset.rs#L727), [740](../../src/preset.rs#L740), [746](../../src/preset.rs#L746) | external-constructor-callback-or-unresolved |
| `resolve_request` | `PresetInvalid::new(PresetReason::SecretInPreset, None, field).into` | [727](../../src/preset.rs#L727) | receiver-type-required |
| `resolve_request` | `PresetInvalid::new` | [727](../../src/preset.rs#L727), [747](../../src/preset.rs#L747) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) |
| `resolve_request` | `object_members` | [729](../../src/preset.rs#L729) | [tools::preset::object_members](../../src/preset.rs#L444) |
| `resolve_request` | `Selection::default` | [734](../../src/preset.rs#L734) | external-constructor-callback-or-unresolved |
| `resolve_request` | `selection.collect` | [735](../../src/preset.rs#L735), [754](../../src/preset.rs#L754) | receiver-type-required |
| `resolve_request` | `object.get` | [736](../../src/preset.rs#L736) | receiver-type-required |
| `resolve_request` | `items.as_slice` | [738](../../src/preset.rs#L738) | receiver-type-required |
| `resolve_request` | `PresetError::BadRequest` | [740](../../src/preset.rs#L740) | external-constructor-callback-or-unresolved |
| `resolve_request` | `"preset.layers must be an array".to_owned` | [741](../../src/preset.rs#L741) | receiver-type-required |
| `resolve_request` | `requested.len` | [745](../../src/preset.rs#L745), [750](../../src/preset.rs#L750) | receiver-type-required |
| `resolve_request` | `PresetInvalid::new(PresetReason::TooManyLayers, None, "preset.layers").into` | [747](../../src/preset.rs#L747) | receiver-type-required |
| `resolve_request` | `Vec::with_capacity` | [750](../../src/preset.rs#L750) | external-constructor-callback-or-unresolved |
| `resolve_request` | `requested.iter().enumerate` | [751](../../src/preset.rs#L751) | receiver-type-required |
| `resolve_request` | `requested.iter` | [751](../../src/preset.rs#L751) | receiver-type-required |
| `resolve_request` | `resolve_layer` | [752](../../src/preset.rs#L752) | [tools::preset::resolve_layer](../../src/preset.rs#L950) |
| `resolve_request` | `layer.as_object` | [753](../../src/preset.rs#L753) | receiver-type-required |
| `resolve_request` | `Some` | [754](../../src/preset.rs#L754) | external-constructor-callback-or-unresolved |
| `resolve_request` | `layers.push` | [756](../../src/preset.rs#L756) | receiver-type-required |
| `resolve_request` | `Self::from_parts` | [758](../../src/preset.rs#L758) | [tools::preset::ResolvedPreset::from_parts](../../src/preset.rs#L781) |
| `resolve_request` | `Vec::new` | [758](../../src/preset.rs#L758) | external-constructor-callback-or-unresolved |
| `resolve_request` | `selection.finish` | [759](../../src/preset.rs#L759) | receiver-type-required |
| `resolve_request` | `preset.compute_digest` | [764](../../src/preset.rs#L764) | receiver-type-required |
| `resolve_request` | `Ok` | [765](../../src/preset.rs#L765) | external-constructor-callback-or-unresolved |
| `from_identity` | `Vec::new` | [776](../../src/preset.rs#L776), [778](../../src/preset.rs#L778) | external-constructor-callback-or-unresolved |
| `from_identity` | `Self::from_parts(base, layers, None, Vec::new(), None).expect` | [778](../../src/preset.rs#L778) | receiver-type-required |
| `from_identity` | `Self::from_parts` | [778](../../src/preset.rs#L778) | [tools::preset::ResolvedPreset::from_parts](../../src/preset.rs#L781) |
| `from_parts` | `std::collections::BTreeSet::new` | [788](../../src/preset.rs#L788) | external-constructor-callback-or-unresolved |
| `from_parts` | `layers.iter().enumerate` | [790](../../src/preset.rs#L790) | receiver-type-required |
| `from_parts` | `layers.iter` | [790](../../src/preset.rs#L790) | receiver-type-required |
| `from_parts` | `seen.insert` | [791](../../src/preset.rs#L791) | receiver-type-required |
| `from_parts` | `layer.id().to_owned` | [791](../../src/preset.rs#L791), [794](../../src/preset.rs#L794), [810](../../src/preset.rs#L810) | receiver-type-required |
| `from_parts` | `layer.id` | [791](../../src/preset.rs#L791), [794](../../src/preset.rs#L794), [810](../../src/preset.rs#L810) | receiver-type-required |
| `from_parts` | `Err` | [792](../../src/preset.rs#L792), [808](../../src/preset.rs#L808), [834](../../src/preset.rs#L834) | external-constructor-callback-or-unresolved |
| `from_parts` | `PresetInvalid::new(                     PresetReason::DuplicateLayer,                     Some(layer.id().to_owned()),                     format!("preset.layers[{index}].id"),                 )                 .into` | [792](../../src/preset.rs#L792) | receiver-type-required |
| `from_parts` | `PresetInvalid::new` | [792](../../src/preset.rs#L792), [808](../../src/preset.rs#L808), [834](../../src/preset.rs#L834) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) |
| `from_parts` | `Some` | [794](../../src/preset.rs#L794), [810](../../src/preset.rs#L810) | external-constructor-callback-or-unresolved |
| `from_parts` | `PresetInvalid::new(                         PresetReason::MultipleIdentity,                         Some(layer.id().to_owned()),                         format!("preset.layers[{index}]"),                     )                     .into` | [808](../../src/preset.rs#L808) | receiver-type-required |
| `from_parts` | `String::new` | [825](../../src/preset.rs#L825) | external-constructor-callback-or-unresolved |
| `from_parts` | `preset             .owned_sections()             .iter()             .map(&#124;text&#124; text.len())             .sum::<usize>` | [827](../../src/preset.rs#L827) | receiver-type-required |
| `from_parts` | `preset             .owned_sections()             .iter()             .map` | [827](../../src/preset.rs#L827) | receiver-type-required |
| `from_parts` | `preset             .owned_sections()             .iter` | [827](../../src/preset.rs#L827) | receiver-type-required |
| `from_parts` | `preset             .owned_sections` | [827](../../src/preset.rs#L827) | receiver-type-required |
| `from_parts` | `text.len` | [830](../../src/preset.rs#L830) | receiver-type-required |
| `from_parts` | `PresetInvalid::new(PresetReason::TextTooLarge, None, "preset").into` | [834](../../src/preset.rs#L834) | receiver-type-required |
| `from_parts` | `preset.compute_digest` | [836](../../src/preset.rs#L836) | receiver-type-required |
| `from_parts` | `Ok` | [837](../../src/preset.rs#L837) | external-constructor-callback-or-unresolved |
| `compute_digest` | `serde_json::to_value(self).expect` | [842](../../src/preset.rs#L842) | receiver-type-required |
| `compute_digest` | `serde_json::to_value` | [842](../../src/preset.rs#L842) | external-constructor-callback-or-unresolved |
| `compute_digest` | `preimage             .as_object_mut()             .expect("preset object")             .remove` | [843](../../src/preset.rs#L843) | receiver-type-required |
| `compute_digest` | `preimage             .as_object_mut()             .expect` | [843](../../src/preset.rs#L843) | receiver-type-required |
| `compute_digest` | `preimage             .as_object_mut` | [843](../../src/preset.rs#L843) | receiver-type-required |
| `compute_digest` | `serde_json_canonicalizer::to_vec(&preimage).expect` | [847](../../src/preset.rs#L847) | receiver-type-required |
| `compute_digest` | `serde_json_canonicalizer::to_vec` | [847](../../src/preset.rs#L847) | external-constructor-callback-or-unresolved |
| `from_genesis` | `serde_json::from_value(value.clone())             .map_err` | [856](../../src/preset.rs#L856) | receiver-type-required |
| `from_genesis` | `serde_json::from_value` | [856](../../src/preset.rs#L856) | external-constructor-callback-or-unresolved |
| `from_genesis` | `value.clone` | [856](../../src/preset.rs#L856) | receiver-type-required |
| `from_genesis` | `Err` | [859](../../src/preset.rs#L859), [864](../../src/preset.rs#L864), [878](../../src/preset.rs#L878) | external-constructor-callback-or-unresolved |
| `from_genesis` | `preset.layers.iter().find` | [861](../../src/preset.rs#L861) | receiver-type-required |
| `from_genesis` | `preset.layers.iter` | [861](../../src/preset.rs#L861) | receiver-type-required |
| `from_genesis` | `Self::from_parts(             preset.base,             preset.layers.clone(),             preset.tools.clone(),             preset.mcp.clone(),             preset.skills.clone(),         )         .map_err` | [866](../../src/preset.rs#L866) | receiver-type-required |
| `from_genesis` | `Self::from_parts` | [866](../../src/preset.rs#L866) | [tools::preset::ResolvedPreset::from_parts](../../src/preset.rs#L781) |
| `from_genesis` | `preset.layers.clone` | [868](../../src/preset.rs#L868) | receiver-type-required |
| `from_genesis` | `preset.tools.clone` | [869](../../src/preset.rs#L869) | receiver-type-required |
| `from_genesis` | `preset.mcp.clone` | [870](../../src/preset.rs#L870) | receiver-type-required |
| `from_genesis` | `preset.skills.clone` | [871](../../src/preset.rs#L871) | receiver-type-required |
| `from_genesis` | `preset.defaults.clone` | [875](../../src/preset.rs#L875) | receiver-type-required |
| `from_genesis` | `check.compute_digest` | [876](../../src/preset.rs#L876) | receiver-type-required |
| `from_genesis` | `"genesis preset digest mismatch".to_owned` | [878](../../src/preset.rs#L878) | receiver-type-required |
| `from_genesis` | `Ok` | [880](../../src/preset.rs#L880) | external-constructor-callback-or-unresolved |
| `owned_sections` | `self.layers.iter().find_map` | [888](../../src/preset.rs#L888), [893](../../src/preset.rs#L893) | receiver-type-required |
| `owned_sections` | `self.layers.iter` | [888](../../src/preset.rs#L888), [893](../../src/preset.rs#L893), [898](../../src/preset.rs#L898) | receiver-type-required |
| `owned_sections` | `identity.as_deref` | [889](../../src/preset.rs#L889) | receiver-type-required |
| `owned_sections` | `inline             .or_else(&#124;&#124; self.layers.iter().find_map(PresetLayer::identity))             .or_else` | [892](../../src/preset.rs#L892) | receiver-type-required |
| `owned_sections` | `inline             .or_else` | [892](../../src/preset.rs#L892) | receiver-type-required |
| `owned_sections` | `self.base.identity` | [894](../../src/preset.rs#L894) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map(str::trim)             .filter(&#124;text&#124; !text.is_empty())             .collect` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map(str::trim)             .filter` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `identity             .into_iter` | [895](../../src/preset.rs#L895) | receiver-type-required |
| `owned_sections` | `self.base.rules` | [897](../../src/preset.rs#L897) | receiver-type-required |
| `owned_sections` | `self.layers.iter().filter_map` | [898](../../src/preset.rs#L898) | receiver-type-required |
| `owned_sections` | `text.is_empty` | [900](../../src/preset.rs#L900) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain(self.owned_sections())             .collect::<Vec<_>>()             .join` | [908](../../src/preset.rs#L908) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain(self.owned_sections())             .collect::<Vec<_>>` | [908](../../src/preset.rs#L908) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain` | [908](../../src/preset.rs#L908) | receiver-type-required |
| `preset_text` | `std::iter::once` | [908](../../src/preset.rs#L908) | external-constructor-callback-or-unresolved |
| `preset_text` | `self.owned_sections` | [909](../../src/preset.rs#L909) | [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L885) |
| `render` | `sections.extend` | [920](../../src/preset.rs#L920) | receiver-type-required |
| `render` | `self.owned_sections()                 .into_iter()                 .map` | [921](../../src/preset.rs#L921) | receiver-type-required |
| `render` | `self.owned_sections()                 .into_iter` | [921](../../src/preset.rs#L921) | receiver-type-required |
| `render` | `self.owned_sections` | [921](../../src/preset.rs#L921) | [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L885) |
| `render` | `text.replace` | [923](../../src/preset.rs#L923) | receiver-type-required |
| `render` | `sections.push` | [926](../../src/preset.rs#L926), [930](../../src/preset.rs#L930) | receiver-type-required |
| `render` | `WORKING_DIRECTORY.replace` | [926](../../src/preset.rs#L926) | receiver-type-required |
| `render` | `instruction_text.trim` | [928](../../src/preset.rs#L928) | receiver-type-required |
| `render` | `instruction_text.is_empty` | [929](../../src/preset.rs#L929) | receiver-type-required |
| `render` | `instruction_text.to_owned` | [930](../../src/preset.rs#L930) | receiver-type-required |
| `render` | `sections.join` | [932](../../src/preset.rs#L932) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then(Value::as_str)         .ok_or_else` | [952](../../src/preset.rs#L952) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then` | [952](../../src/preset.rs#L952) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then` | [952](../../src/preset.rs#L952) | receiver-type-required |
| `resolve_layer` | `layer         .as_object` | [952](../../src/preset.rs#L952) | receiver-type-required |
| `resolve_layer` | `object.get` | [954](../../src/preset.rs#L954) | receiver-type-required |
| `resolve_layer` | `PresetError::BadRequest` | [956](../../src/preset.rs#L956), [961](../../src/preset.rs#L961), [984](../../src/preset.rs#L984), [1001](../../src/preset.rs#L1001), [1047](../../src/preset.rs#L1047) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `object_members` | [959](../../src/preset.rs#L959), [975](../../src/preset.rs#L975) | [tools::preset::object_members](../../src/preset.rs#L444) |
| `resolve_layer` | `string_member(object, "id", &path)?                 .ok_or_else` | [960](../../src/preset.rs#L960), [983](../../src/preset.rs#L983) | receiver-type-required |
| `resolve_layer` | `string_member` | [960](../../src/preset.rs#L960), [983](../../src/preset.rs#L983), [1000](../../src/preset.rs#L1000), [1010](../../src/preset.rs#L1010), [1029](../../src/preset.rs#L1029) | [tools::preset::string_member](../../src/preset.rs#L460) |
| `resolve_layer` | `kernel_layer(id).ok_or_else` | [962](../../src/preset.rs#L962) | receiver-type-required |
| `resolve_layer` | `kernel_layer` | [962](../../src/preset.rs#L962) | [tools::preset::kernel_layer](../../src/preset.rs#L104) |
| `resolve_layer` | `PresetInvalid::new` | [963](../../src/preset.rs#L963), [993](../../src/preset.rs#L993), [1003](../../src/preset.rs#L1003), [1013](../../src/preset.rs#L1013), [1021](../../src/preset.rs#L1021), [1031](../../src/preset.rs#L1031) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L318) |
| `resolve_layer` | `Some` | [965](../../src/preset.rs#L965), [986](../../src/preset.rs#L986) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `id.to_owned` | [965](../../src/preset.rs#L965), [987](../../src/preset.rs#L987), [1039](../../src/preset.rs#L1039) | receiver-type-required |
| `resolve_layer` | `Ok` | [969](../../src/preset.rs#L969), [1038](../../src/preset.rs#L1038) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `known.id.to_owned` | [970](../../src/preset.rs#L970) | receiver-type-required |
| `resolve_layer` | `known.version.to_owned` | [971](../../src/preset.rs#L971) | receiver-type-required |
| `resolve_layer` | `valid_id` | [986](../../src/preset.rs#L986), [992](../../src/preset.rs#L992) | [tools::preset::valid_id](../../src/preset.rs#L429) |
| `resolve_layer` | `Err` | [993](../../src/preset.rs#L993), [1003](../../src/preset.rs#L1003), [1013](../../src/preset.rs#L1013), [1021](../../src/preset.rs#L1021), [1031](../../src/preset.rs#L1031), [1047](../../src/preset.rs#L1047) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::InvalidId,                     label(),                     format!("{path}.id"),                 )                 .into` | [993](../../src/preset.rs#L993) | receiver-type-required |
| `resolve_layer` | `label` | [995](../../src/preset.rs#L995), [1005](../../src/preset.rs#L1005), [1015](../../src/preset.rs#L1015), [1023](../../src/preset.rs#L1023), [1033](../../src/preset.rs#L1033) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `string_member(object, "version", &path)?                 .ok_or_else` | [1000](../../src/preset.rs#L1000) | receiver-type-required |
| `resolve_layer` | `valid_version` | [1002](../../src/preset.rs#L1002) | [tools::preset::valid_version](../../src/preset.rs#L438) |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::InvalidId,                     label(),                     format!("{path}.version"),                 )                 .into` | [1003](../../src/preset.rs#L1003) | receiver-type-required |
| `resolve_layer` | `identity.len` | [1012](../../src/preset.rs#L1012) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                         PresetReason::TextTooLarge,                         label(),                         format!("{path}.identity"),                     )                     .into` | [1013](../../src/preset.rs#L1013) | receiver-type-required |
| `resolve_layer` | `identity.trim().is_empty` | [1020](../../src/preset.rs#L1020) | receiver-type-required |
| `resolve_layer` | `identity.trim` | [1020](../../src/preset.rs#L1020) | receiver-type-required |
| `resolve_layer` | `identity.trim().contains` | [1020](../../src/preset.rs#L1020) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                         PresetReason::InvalidIdentity,                         label(),                         format!("{path}.identity"),                     )                     .into` | [1021](../../src/preset.rs#L1021) | receiver-type-required |
| `resolve_layer` | `duty.is_some_and` | [1030](../../src/preset.rs#L1030) | receiver-type-required |
| `resolve_layer` | `duty.len` | [1030](../../src/preset.rs#L1030) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::TextTooLarge,                     label(),                     format!("{path}.duty"),                 )                 .into` | [1031](../../src/preset.rs#L1031) | receiver-type-required |
| `resolve_layer` | `version.to_owned` | [1040](../../src/preset.rs#L1040) | receiver-type-required |
| `resolve_layer` | `identity.map` | [1041](../../src/preset.rs#L1041) | receiver-type-required |
| `resolve_layer` | `text.trim().to_owned` | [1041](../../src/preset.rs#L1041), [1043](../../src/preset.rs#L1043) | receiver-type-required |
| `resolve_layer` | `text.trim` | [1041](../../src/preset.rs#L1041), [1043](../../src/preset.rs#L1043) | receiver-type-required |
| `resolve_layer` | `duty                     .map(&#124;text&#124; text.trim().to_owned())                     .filter` | [1042](../../src/preset.rs#L1042) | receiver-type-required |
| `resolve_layer` | `duty                     .map` | [1042](../../src/preset.rs#L1042) | receiver-type-required |
| `resolve_layer` | `text.is_empty` | [1044](../../src/preset.rs#L1044) | receiver-type-required |
| `resolve` | `ResolvedPreset::resolve_request` | [1092](../../src/preset.rs#L1092) | external-constructor-callback-or-unresolved |
| `permitted_tools` | `self.permitted.as_ref` | [1101](../../src/preset.rs#L1101) | receiver-type-required |
| `mcp_server_digest` | `(server == "docs").then` | [1104](../../src/preset.rs#L1104) | receiver-type-required |
| `mcp_server_digest` | `"sha256:docs".to_owned` | [1104](../../src/preset.rs#L1104) | receiver-type-required |
| `mcp_tool_name` | `(!tool.contains('/')).then` | [1107](../../src/preset.rs#L1107) | receiver-type-required |
| `mcp_tool_name` | `tool.contains` | [1107](../../src/preset.rs#L1107) | receiver-type-required |
| `model_available` | `["fast", "deep"].contains` | [1114](../../src/preset.rs#L1114) | receiver-type-required |
| `model_available` | `effort.is_none_or` | [1115](../../src/preset.rs#L1115) | receiver-type-required |
| `resolve_in` | `ResolvedPreset::resolve_request` | [1123](../../src/preset.rs#L1123) | external-constructor-callback-or-unresolved |
| `resolve_in` | `permitted                     .map` | [1127](../../src/preset.rs#L1127) | receiver-type-required |
| `resolve_in` | `names.iter().map(&#124;name&#124; (*name).to_owned()).collect` | [1128](../../src/preset.rs#L1128) | receiver-type-required |
| `resolve_in` | `names.iter().map` | [1128](../../src/preset.rs#L1128) | receiver-type-required |
| `resolve_in` | `names.iter` | [1128](../../src/preset.rs#L1128) | receiver-type-required |
| `resolve_in` | `(*name).to_owned` | [1128](../../src/preset.rs#L1128) | receiver-type-required |
| `invalid` | `resolve` | [1134](../../src/preset.rs#L1134) | [tools::preset::tests::resolve](../../src/preset.rs#L1091) |
| `sugar_renders_exactly_like_the_identity_profile` | `Some` | [1143](../../src/preset.rs#L1143), [1144](../../src/preset.rs#L1144) | external-constructor-callback-or-unresolved |
| `sugar_renders_exactly_like_the_identity_profile` | `ResolvedPreset::from_identity` | [1144](../../src/preset.rs#L1144) | external-constructor-callback-or-unresolved |
| `sugar_renders_exactly_like_the_identity_profile` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"}]})).unwrap` | [1151](../../src/preset.rs#L1151) | receiver-type-required |
| `sugar_renders_exactly_like_the_identity_profile` | `resolve` | [1151](../../src/preset.rs#L1151) | [tools::preset::tests::resolve](../../src/preset.rs#L1091) |
| `blank_base_drops_identity_and_rules` | `ResolvedPreset::from_identity` | [1160](../../src/preset.rs#L1160), [1166](../../src/preset.rs#L1166) | external-constructor-callback-or-unresolved |
| `blank_base_drops_identity_and_rules` | `Some` | [1166](../../src/preset.rs#L1166) | external-constructor-callback-or-unresolved |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve(json!({"layers":[             {"kind":"kernel","id":"coding"},             {"kind":"inline","id":"domain-expert","version":"3","duty":"Prefer the domain tools."}         ]}))         .unwrap` | [1178](../../src/preset.rs#L1178) | receiver-type-required |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve` | [1178](../../src/preset.rs#L1178), [1188](../../src/preset.rs#L1188) | [tools::preset::tests::resolve](../../src/preset.rs#L1091) |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve(json!({"layers":[{"kind":"inline","id":"domain-expert","version":"3",             "identity":"You are a domain expert agent powered by the {model} model.","duty":"Prefer the domain tools."}]}))         .unwrap` | [1188](../../src/preset.rs#L1188) | receiver-type-required |
| `inline_identity_wins_over_a_kernel_layer_identity` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"},             {"kind":"inline","id":"domain-expert","version":"1",              "identity":"You are a domain expert agent powered by the {model} model.","duty":"Prefer the domain tools."}]}))         .unwrap` | [1209](../../src/preset.rs#L1209) | receiver-type-required |
| `inline_identity_wins_over_a_kernel_layer_identity` | `resolve` | [1209](../../src/preset.rs#L1209) | [tools::preset::tests::resolve](../../src/preset.rs#L1091) |
| `refusals_name_reason_layer_and_field` | `invalid` | [1225](../../src/preset.rs#L1225), [1230](../../src/preset.rs#L1230), [1238](../../src/preset.rs#L1238) | [tools::preset::tests::invalid](../../src/preset.rs#L1133) |
| `refusals_name_reason_layer_and_field` | `(0..9)             .map(&#124;index&#124; json!({"kind":"inline","id":format!("l{index}"),"version":"1"}))             .collect::<Vec<_>>` | [1250](../../src/preset.rs#L1250) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..9)             .map` | [1250](../../src/preset.rs#L1250) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `"x".repeat` | [1257](../../src/preset.rs#L1257) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..3)             .map(&#124;index&#124; json!({"kind":"inline","id":format!("l{index}"),"version":"1","duty":"y".repeat(MAX_PRESET_DUTY_BYTES)}))             .collect::<Vec<_>>` | [1262](../../src/preset.rs#L1262) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..3)             .map` | [1262](../../src/preset.rs#L1262) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `resolve_in(             json!({"tools":["read"],"layers":[{"kind":"inline","id":"expert","version":"1",                 "tools":["grep"],"mcp":[{"server":"docs","tools":["open","draw"]}],                 "skills":["style-guide"]}]}),             None,         )         .unwrap` | [1277](../../src/preset.rs#L1277) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `resolve_in` | [1277](../../src/preset.rs#L1277), [1307](../../src/preset.rs#L1307), [1323](../../src/preset.rs#L1323) | [tools::preset::tests::resolve_in](../../src/preset.rs#L1119) |
| `tools_mcp_and_skills_resolve_against_the_environment` | `serde_json::to_value(&preset).unwrap` | [1302](../../src/preset.rs#L1302) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `serde_json::to_value` | [1302](../../src/preset.rs#L1302) | external-constructor-callback-or-unresolved |
| `tools_mcp_and_skills_resolve_against_the_environment` | `resolve_in(json!({"mcp":[{"server":"docs","tools":["open"]}]}), None).unwrap` | [1307](../../src/preset.rs#L1307) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `mcp_only.tools.unwrap` | [1308](../../src/preset.rs#L1308) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `(0..=MAX_PRESET_MCP_SERVERS)             .map(&#124;index&#124; json!({"server":format!("s{index}"),"tools":["t"]}))             .collect::<Vec<_>>` | [1362](../../src/preset.rs#L1362) | receiver-type-required |
| `tools_mcp_and_skills_resolve_against_the_environment` | `(0..=MAX_PRESET_MCP_SERVERS)             .map` | [1362](../../src/preset.rs#L1362) | receiver-type-required |
| `model_defaults_resolve_with_top_level_then_last_layer_precedence` | `resolve_in(request, None).unwrap` | [1388](../../src/preset.rs#L1388) | receiver-type-required |
| `model_defaults_resolve_with_top_level_then_last_layer_precedence` | `resolve_in` | [1388](../../src/preset.rs#L1388), [1409](../../src/preset.rs#L1409), [1419](../../src/preset.rs#L1419) | [tools::preset::tests::resolve_in](../../src/preset.rs#L1119) |
| `model_defaults_resolve_with_top_level_then_last_layer_precedence` | `resolve_in(json!({"layers":[layer("a", "deep")]}), None).unwrap` | [1409](../../src/preset.rs#L1409) | receiver-type-required |
| `model_defaults_resolve_with_top_level_then_last_layer_precedence` | `serde_json::to_value(&preset).unwrap` | [1410](../../src/preset.rs#L1410) | receiver-type-required |
| `model_defaults_resolve_with_top_level_then_last_layer_precedence` | `serde_json::to_value` | [1410](../../src/preset.rs#L1410) | external-constructor-callback-or-unresolved |
| `secrets_and_definitions_are_refused_before_shape_checks` | `invalid` | [1453](../../src/preset.rs#L1453), [1456](../../src/preset.rs#L1456) | [tools::preset::tests::invalid](../../src/preset.rs#L1133) |
| `genesis_round_trip_checks_digest_and_layers` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"},             {"kind":"inline","id":"expert","version":"1","duty":"Domain."}]}))         .unwrap` | [1486](../../src/preset.rs#L1486) | receiver-type-required |
| `genesis_round_trip_checks_digest_and_layers` | `resolve` | [1486](../../src/preset.rs#L1486) | [tools::preset::tests::resolve](../../src/preset.rs#L1091) |
| `genesis_round_trip_checks_digest_and_layers` | `serde_json::to_value(&preset).unwrap` | [1489](../../src/preset.rs#L1489) | receiver-type-required |
| `genesis_round_trip_checks_digest_and_layers` | `serde_json::to_value` | [1489](../../src/preset.rs#L1489) | external-constructor-callback-or-unresolved |
| `genesis_round_trip_checks_digest_and_layers` | `value.clone` | [1491](../../src/preset.rs#L1491) | receiver-type-required |
