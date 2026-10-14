# tools::preset

[Package atlas](index.md) · [Source](../../src/preset.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::preset::PRESET_FORMAT](../../src/preset.rs#L24) | const_item | `pub` |  |
| [tools::preset::PRESET_RENDER_VERSION](../../src/preset.rs#L26) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_LAYERS](../../src/preset.rs#L28) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_ID_BYTES](../../src/preset.rs#L29) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_VERSION_BYTES](../../src/preset.rs#L30) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_IDENTITY_BYTES](../../src/preset.rs#L31) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_DUTY_BYTES](../../src/preset.rs#L32) | const_item | `pub` |  |
| [tools::preset::MAX_PRESET_TEXT_BYTES](../../src/preset.rs#L34) | const_item | `pub` |  |
| [tools::preset::PresetBase](../../src/preset.rs#L39) | enum_item | `pub` |  |
| [tools::preset::PresetBase::as_str](../../src/preset.rs#L47) | function_item | `pub` |  |
| [tools::preset::PresetBase::parse](../../src/preset.rs#L55) | function_item | `pub` |  |
| [tools::preset::PresetBase::identity](../../src/preset.rs#L63) | function_item | `private` |  |
| [tools::preset::PresetBase::rules](../../src/preset.rs#L70) | function_item | `private` |  |
| [tools::preset::KernelLayer](../../src/preset.rs#L80) | struct_item | `pub` |  |
| [tools::preset::KERNEL_LAYERS](../../src/preset.rs#L88) | const_item | `pub` |  |
| [tools::preset::kernel_layer](../../src/preset.rs#L96) | function_item | `private` |  |
| [tools::preset::PresetLayer](../../src/preset.rs#L103) | enum_item | `pub` |  |
| [tools::preset::PresetLayer::id](../../src/preset.rs#L120) | function_item | `pub` |  |
| [tools::preset::PresetLayer::version](../../src/preset.rs#L127) | function_item | `pub` |  |
| [tools::preset::PresetLayer::kind](../../src/preset.rs#L134) | function_item | `pub` |  |
| [tools::preset::PresetLayer::identity](../../src/preset.rs#L141) | function_item | `private` |  |
| [tools::preset::PresetLayer::duty](../../src/preset.rs#L148) | function_item | `private` |  |
| [tools::preset::ResolvedPreset](../../src/preset.rs#L159) | struct_item | `pub` |  |
| [tools::preset::PresetInvalidReason](../../src/preset.rs#L168) | enum_item | `pub` |  |
| [tools::preset::PresetInvalidReason::as_str](../../src/preset.rs#L182) | function_item | `pub` |  |
| [tools::preset::ALL](../../src/preset.rs#L196) | const_item | `pub` |  |
| [tools::preset::PresetInvalid](../../src/preset.rs#L210) | struct_item | `pub` |  |
| [tools::preset::PresetInvalid::new](../../src/preset.rs#L219) | function_item | `private` |  |
| [tools::preset::PresetInvalid::detail](../../src/preset.rs#L229) | function_item | `pub` |  |
| [tools::preset::PresetError](../../src/preset.rs#L245) | enum_item | `pub` |  |
| [tools::preset::PresetError::from](../../src/preset.rs#L251) | function_item | `private` |  |
| [tools::preset::SECRET_KEYS](../../src/preset.rs#L259) | const_item | `private` |  |
| [tools::preset::SECRET_PREFIXES](../../src/preset.rs#L279) | const_item | `private` |  |
| [tools::preset::looks_like_secret](../../src/preset.rs#L290) | function_item | `private` |  |
| [tools::preset::find_secret](../../src/preset.rs#L307) | function_item | `private` |  |
| [tools::preset::valid_id](../../src/preset.rs#L330) | function_item | `private` |  |
| [tools::preset::valid_version](../../src/preset.rs#L339) | function_item | `private` |  |
| [tools::preset::object_members](../../src/preset.rs#L345) | function_item | `private` |  |
| [tools::preset::string_member](../../src/preset.rs#L361) | function_item | `private` |  |
| [tools::preset::RESERVED_MEMBERS](../../src/preset.rs#L378) | const_item | `private` |  |
| [tools::preset::ResolvedPreset::resolve_request](../../src/preset.rs#L382) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::from_identity](../../src/preset.rs#L421) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::from_layers](../../src/preset.rs#L432) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::compute_digest](../../src/preset.rs#L481) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::from_genesis](../../src/preset.rs#L489) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L510) | function_item | `private` |  |
| [tools::preset::ResolvedPreset::preset_text](../../src/preset.rs#L532) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::render](../../src/preset.rs#L543) | function_item | `pub` |  |
| [tools::preset::ResolvedPreset::summary](../../src/preset.rs#L562) | function_item | `pub` |  |
| [tools::preset::PresetReason](../../src/preset.rs#L573) | type_item | `private` |  |
| [tools::preset::resolve_layer](../../src/preset.rs#L575) | function_item | `private` |  |
| [tools::preset::describe_value](../../src/preset.rs#L691) | function_item | `pub` |  |
| [tools::preset::tests::resolve](../../src/preset.rs#L720) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::invalid](../../src/preset.rs#L724) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::sugar_renders_exactly_like_the_identity_profile](../../src/preset.rs#L732) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::blank_base_drops_identity_and_rules](../../src/preset.rs#L750) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::inline_identity_replaces_the_base_identity_but_not_its_rules](../../src/preset.rs#L768) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::inline_identity_wins_over_a_kernel_layer_identity](../../src/preset.rs#L795) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::refusals_name_reason_layer_and_field](../../src/preset.rs#L811) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::secrets_and_definitions_are_refused_before_shape_checks](../../src/preset.rs#L869) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::shape_errors_are_bad_requests](../../src/preset.rs#L884) | function_item | `private` | test; #[cfg(test)] |
| [tools::preset::tests::genesis_round_trip_checks_digest_and_layers](../../src/preset.rs#L902) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Map` | `serde_json::Map` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
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

<details><summary>Functions 1–20: 4 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::preset::PresetLayer::id"]
  n1["tools::preset::PresetLayer::version"]
  n2["tools::preset::PresetLayer::kind"]
  n3["tools::preset::PresetLayer::identity"]
  n4["tools::preset::PresetLayer::duty"]
  n5["tools::preset::PresetInvalidReason::as_str"]
  n6["tools::preset::PresetInvalid::new"]
  n7["tools::preset::PresetInvalid::detail"]
  n8["tools::preset::PresetError::from"]
  n9["tools::preset::looks_like_secret"]
  n10["tools::preset::find_secret"]
  n11["tools::preset::valid_id"]
  n12["tools::preset::valid_version"]
  n13["tools::preset::object_members"]
  n14["tools::preset::string_member"]
  n15["tools::preset::PresetBase::as_str"]
  n16["tools::preset::PresetBase::parse"]
  n17["tools::preset::PresetBase::identity"]
  n18["tools::preset::PresetBase::rules"]
  n19["tools::preset::kernel_layer"]
  n3 --> n19
  n4 --> n19
  n10 --> n9
  n10 --> n10
```

</details>

<details><summary>Functions 21–31: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::preset::PresetInvalid::new"]
  n1["tools::preset::find_secret"]
  n2["tools::preset::valid_id"]
  n3["tools::preset::valid_version"]
  n4["tools::preset::object_members"]
  n5["tools::preset::string_member"]
  n6["tools::preset::ResolvedPreset::resolve_request"]
  n7["tools::preset::ResolvedPreset::from_identity"]
  n8["tools::preset::ResolvedPreset::from_layers"]
  n9["tools::preset::ResolvedPreset::compute_digest"]
  n10["tools::preset::ResolvedPreset::from_genesis"]
  n11["tools::preset::ResolvedPreset::owned_sections"]
  n12["tools::preset::ResolvedPreset::preset_text"]
  n13["tools::preset::ResolvedPreset::render"]
  n14["tools::preset::ResolvedPreset::summary"]
  n15["tools::preset::resolve_layer"]
  n16["tools::preset::describe_value"]
  n17["tools::preset::kernel_layer"]
  n6 --> n0
  n6 --> n1
  n6 --> n4
  n6 --> n8
  n6 --> n15
  n7 --> n8
  n8 --> n0
  n10 --> n8
  n12 --> n11
  n13 --> n11
  n15 --> n0
  n15 --> n2
  n15 --> n3
  n15 --> n4
  n15 --> n5
  n15 --> n17
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse` | `Some` | [57](../../src/preset.rs#L57), [58](../../src/preset.rs#L58) | external-constructor-callback-or-unresolved |
| `identity` | `Some` | [65](../../src/preset.rs#L65) | external-constructor-callback-or-unresolved |
| `rules` | `Some` | [72](../../src/preset.rs#L72) | external-constructor-callback-or-unresolved |
| `kernel_layer` | `KERNEL_LAYERS.iter().find` | [97](../../src/preset.rs#L97) | receiver-type-required |
| `kernel_layer` | `KERNEL_LAYERS.iter` | [97](../../src/preset.rs#L97) | receiver-type-required |
| `identity` | `kernel_layer(id).map` | [143](../../src/preset.rs#L143) | receiver-type-required |
| `identity` | `kernel_layer` | [143](../../src/preset.rs#L143) | [tools::preset::kernel_layer](../../src/preset.rs#L96) |
| `identity` | `identity.as_deref` | [144](../../src/preset.rs#L144) | receiver-type-required |
| `duty` | `kernel_layer(id).map` | [150](../../src/preset.rs#L150) | receiver-type-required |
| `duty` | `kernel_layer` | [150](../../src/preset.rs#L150) | [tools::preset::kernel_layer](../../src/preset.rs#L96) |
| `duty` | `duty.as_deref` | [151](../../src/preset.rs#L151) | receiver-type-required |
| `new` | `Some` | [223](../../src/preset.rs#L223) | external-constructor-callback-or-unresolved |
| `new` | `field.into` | [223](../../src/preset.rs#L223) | receiver-type-required |
| `detail` | `Map::new` | [230](../../src/preset.rs#L230) | external-constructor-callback-or-unresolved |
| `detail` | `detail.insert` | [231](../../src/preset.rs#L231), [233](../../src/preset.rs#L233), [236](../../src/preset.rs#L236) | receiver-type-required |
| `detail` | `"reason".to_owned` | [231](../../src/preset.rs#L231) | receiver-type-required |
| `detail` | `"layer".to_owned` | [233](../../src/preset.rs#L233) | receiver-type-required |
| `detail` | `"field".to_owned` | [236](../../src/preset.rs#L236) | receiver-type-required |
| `detail` | `Value::Object` | [238](../../src/preset.rs#L238) | external-constructor-callback-or-unresolved |
| `from` | `Self::Invalid` | [252](../../src/preset.rs#L252) | external-constructor-callback-or-unresolved |
| `looks_like_secret` | `text.contains` | [291](../../src/preset.rs#L291) | receiver-type-required |
| `looks_like_secret` | `text.split(&#124;ch: char&#124; ch.is_whitespace() &#124;&#124; matches!(ch, '"' &#124; '\'' &#124; ''' &#124; '(' &#124; ')' &#124; '='))         .any` | [294](../../src/preset.rs#L294) | receiver-type-required |
| `looks_like_secret` | `text.split` | [294](../../src/preset.rs#L294) | receiver-type-required |
| `looks_like_secret` | `ch.is_whitespace` | [294](../../src/preset.rs#L294) | receiver-type-required |
| `looks_like_secret` | `SECRET_PREFIXES.iter().any` | [296](../../src/preset.rs#L296) | receiver-type-required |
| `looks_like_secret` | `SECRET_PREFIXES.iter` | [296](../../src/preset.rs#L296) | receiver-type-required |
| `looks_like_secret` | `token.strip_prefix(prefix).is_some_and` | [297](../../src/preset.rs#L297) | receiver-type-required |
| `looks_like_secret` | `token.strip_prefix` | [297](../../src/preset.rs#L297) | receiver-type-required |
| `looks_like_secret` | `tail.len` | [298](../../src/preset.rs#L298) | receiver-type-required |
| `looks_like_secret` | `tail                             .chars()                             .all` | [299](../../src/preset.rs#L299) | receiver-type-required |
| `looks_like_secret` | `tail                             .chars` | [299](../../src/preset.rs#L299) | receiver-type-required |
| `looks_like_secret` | `ch.is_ascii_alphanumeric` | [301](../../src/preset.rs#L301) | receiver-type-required |
| `find_secret` | `object.iter().find_map` | [309](../../src/preset.rs#L309) | receiver-type-required |
| `find_secret` | `object.iter` | [309](../../src/preset.rs#L309) | receiver-type-required |
| `find_secret` | `path.is_empty` | [310](../../src/preset.rs#L310) | receiver-type-required |
| `find_secret` | `key.clone` | [311](../../src/preset.rs#L311) | receiver-type-required |
| `find_secret` | `SECRET_KEYS.contains` | [315](../../src/preset.rs#L315) | receiver-type-required |
| `find_secret` | `key.as_str` | [315](../../src/preset.rs#L315) | receiver-type-required |
| `find_secret` | `Some` | [316](../../src/preset.rs#L316) | external-constructor-callback-or-unresolved |
| `find_secret` | `find_secret` | [318](../../src/preset.rs#L318), [324](../../src/preset.rs#L324) | [tools::preset::find_secret](../../src/preset.rs#L307) |
| `find_secret` | `items             .iter()             .enumerate()             .find_map` | [321](../../src/preset.rs#L321) | receiver-type-required |
| `find_secret` | `items             .iter()             .enumerate` | [321](../../src/preset.rs#L321) | receiver-type-required |
| `find_secret` | `items             .iter` | [321](../../src/preset.rs#L321) | receiver-type-required |
| `find_secret` | `looks_like_secret(text).then` | [325](../../src/preset.rs#L325) | receiver-type-required |
| `find_secret` | `looks_like_secret` | [325](../../src/preset.rs#L325) | [tools::preset::looks_like_secret](../../src/preset.rs#L290) |
| `find_secret` | `path.to_owned` | [325](../../src/preset.rs#L325) | receiver-type-required |
| `valid_id` | `value.is_empty` | [331](../../src/preset.rs#L331) | receiver-type-required |
| `valid_id` | `value.len` | [332](../../src/preset.rs#L332) | receiver-type-required |
| `valid_id` | `value             .bytes()             .all` | [333](../../src/preset.rs#L333) | receiver-type-required |
| `valid_id` | `value             .bytes` | [333](../../src/preset.rs#L333) | receiver-type-required |
| `valid_id` | `byte.is_ascii_lowercase` | [335](../../src/preset.rs#L335) | receiver-type-required |
| `valid_id` | `byte.is_ascii_digit` | [335](../../src/preset.rs#L335) | receiver-type-required |
| `valid_id` | `value.as_bytes()[0].is_ascii_alphanumeric` | [336](../../src/preset.rs#L336) | receiver-type-required |
| `valid_id` | `value.as_bytes` | [336](../../src/preset.rs#L336) | receiver-type-required |
| `valid_version` | `value.is_empty` | [340](../../src/preset.rs#L340) | receiver-type-required |
| `valid_version` | `value.len` | [341](../../src/preset.rs#L341) | receiver-type-required |
| `valid_version` | `value.bytes().all` | [342](../../src/preset.rs#L342) | receiver-type-required |
| `valid_version` | `value.bytes` | [342](../../src/preset.rs#L342) | receiver-type-required |
| `valid_version` | `byte.is_ascii_graphic` | [342](../../src/preset.rs#L342) | receiver-type-required |
| `object_members` | `value         .as_object()         .ok_or_else` | [350](../../src/preset.rs#L350) | receiver-type-required |
| `object_members` | `value         .as_object` | [350](../../src/preset.rs#L350) | receiver-type-required |
| `object_members` | `PresetError::BadRequest` | [352](../../src/preset.rs#L352), [354](../../src/preset.rs#L354) | external-constructor-callback-or-unresolved |
| `object_members` | `object.keys().find` | [353](../../src/preset.rs#L353) | receiver-type-required |
| `object_members` | `object.keys` | [353](../../src/preset.rs#L353) | receiver-type-required |
| `object_members` | `allowed.contains` | [353](../../src/preset.rs#L353) | receiver-type-required |
| `object_members` | `key.as_str` | [353](../../src/preset.rs#L353) | receiver-type-required |
| `object_members` | `Err` | [354](../../src/preset.rs#L354) | external-constructor-callback-or-unresolved |
| `object_members` | `Ok` | [358](../../src/preset.rs#L358) | external-constructor-callback-or-unresolved |
| `string_member` | `object.get` | [366](../../src/preset.rs#L366) | receiver-type-required |
| `string_member` | `Ok` | [367](../../src/preset.rs#L367), [368](../../src/preset.rs#L368) | external-constructor-callback-or-unresolved |
| `string_member` | `Some` | [368](../../src/preset.rs#L368) | external-constructor-callback-or-unresolved |
| `string_member` | `Err` | [369](../../src/preset.rs#L369) | external-constructor-callback-or-unresolved |
| `string_member` | `PresetError::BadRequest` | [369](../../src/preset.rs#L369) | external-constructor-callback-or-unresolved |
| `resolve_request` | `find_secret` | [383](../../src/preset.rs#L383) | [tools::preset::find_secret](../../src/preset.rs#L307) |
| `resolve_request` | `Err` | [384](../../src/preset.rs#L384), [391](../../src/preset.rs#L391), [402](../../src/preset.rs#L402), [408](../../src/preset.rs#L408) | external-constructor-callback-or-unresolved |
| `resolve_request` | `PresetInvalid::new(PresetReason::SecretInPreset, None, field).into` | [384](../../src/preset.rs#L384) | receiver-type-required |
| `resolve_request` | `PresetInvalid::new` | [384](../../src/preset.rs#L384), [391](../../src/preset.rs#L391), [409](../../src/preset.rs#L409) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L219) |
| `resolve_request` | `object_members` | [386](../../src/preset.rs#L386) | [tools::preset::object_members](../../src/preset.rs#L345) |
| `resolve_request` | `RESERVED_MEMBERS             .iter()             .find` | [387](../../src/preset.rs#L387) | receiver-type-required |
| `resolve_request` | `RESERVED_MEMBERS             .iter` | [387](../../src/preset.rs#L387) | receiver-type-required |
| `resolve_request` | `object.contains_key` | [389](../../src/preset.rs#L389) | receiver-type-required |
| `resolve_request` | `PresetInvalid::new(                 PresetReason::UnsupportedField,                 None,                 format!("preset.{reserved}"),             )             .into` | [391](../../src/preset.rs#L391) | receiver-type-required |
| `resolve_request` | `object.get` | [398](../../src/preset.rs#L398) | receiver-type-required |
| `resolve_request` | `items.as_slice` | [400](../../src/preset.rs#L400) | receiver-type-required |
| `resolve_request` | `PresetError::BadRequest` | [402](../../src/preset.rs#L402) | external-constructor-callback-or-unresolved |
| `resolve_request` | `"preset.layers must be an array".to_owned` | [403](../../src/preset.rs#L403) | receiver-type-required |
| `resolve_request` | `requested.len` | [407](../../src/preset.rs#L407), [412](../../src/preset.rs#L412) | receiver-type-required |
| `resolve_request` | `PresetInvalid::new(PresetReason::TooManyLayers, None, "preset.layers").into` | [409](../../src/preset.rs#L409) | receiver-type-required |
| `resolve_request` | `Vec::with_capacity` | [412](../../src/preset.rs#L412) | external-constructor-callback-or-unresolved |
| `resolve_request` | `requested.iter().enumerate` | [413](../../src/preset.rs#L413) | receiver-type-required |
| `resolve_request` | `requested.iter` | [413](../../src/preset.rs#L413) | receiver-type-required |
| `resolve_request` | `layers.push` | [414](../../src/preset.rs#L414) | receiver-type-required |
| `resolve_request` | `resolve_layer` | [414](../../src/preset.rs#L414) | [tools::preset::resolve_layer](../../src/preset.rs#L575) |
| `resolve_request` | `Self::from_layers` | [416](../../src/preset.rs#L416) | [tools::preset::ResolvedPreset::from_layers](../../src/preset.rs#L432) |
| `from_identity` | `Vec::new` | [427](../../src/preset.rs#L427) | external-constructor-callback-or-unresolved |
| `from_identity` | `Self::from_layers(base, layers).expect` | [429](../../src/preset.rs#L429) | receiver-type-required |
| `from_identity` | `Self::from_layers` | [429](../../src/preset.rs#L429) | [tools::preset::ResolvedPreset::from_layers](../../src/preset.rs#L432) |
| `from_layers` | `std::collections::BTreeSet::new` | [433](../../src/preset.rs#L433) | external-constructor-callback-or-unresolved |
| `from_layers` | `layers.iter().enumerate` | [435](../../src/preset.rs#L435) | receiver-type-required |
| `from_layers` | `layers.iter` | [435](../../src/preset.rs#L435) | receiver-type-required |
| `from_layers` | `seen.insert` | [436](../../src/preset.rs#L436) | receiver-type-required |
| `from_layers` | `layer.id().to_owned` | [436](../../src/preset.rs#L436), [439](../../src/preset.rs#L439), [455](../../src/preset.rs#L455) | receiver-type-required |
| `from_layers` | `layer.id` | [436](../../src/preset.rs#L436), [439](../../src/preset.rs#L439), [455](../../src/preset.rs#L455) | receiver-type-required |
| `from_layers` | `Err` | [437](../../src/preset.rs#L437), [453](../../src/preset.rs#L453), [475](../../src/preset.rs#L475) | external-constructor-callback-or-unresolved |
| `from_layers` | `PresetInvalid::new(                     PresetReason::DuplicateLayer,                     Some(layer.id().to_owned()),                     format!("preset.layers[{index}].id"),                 )                 .into` | [437](../../src/preset.rs#L437) | receiver-type-required |
| `from_layers` | `PresetInvalid::new` | [437](../../src/preset.rs#L437), [453](../../src/preset.rs#L453), [475](../../src/preset.rs#L475) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L219) |
| `from_layers` | `Some` | [439](../../src/preset.rs#L439), [455](../../src/preset.rs#L455) | external-constructor-callback-or-unresolved |
| `from_layers` | `PresetInvalid::new(                         PresetReason::MultipleIdentity,                         Some(layer.id().to_owned()),                         format!("preset.layers[{index}]"),                     )                     .into` | [453](../../src/preset.rs#L453) | receiver-type-required |
| `from_layers` | `String::new` | [466](../../src/preset.rs#L466) | external-constructor-callback-or-unresolved |
| `from_layers` | `preset             .owned_sections()             .iter()             .map(&#124;text&#124; text.len())             .sum::<usize>` | [468](../../src/preset.rs#L468) | receiver-type-required |
| `from_layers` | `preset             .owned_sections()             .iter()             .map` | [468](../../src/preset.rs#L468) | receiver-type-required |
| `from_layers` | `preset             .owned_sections()             .iter` | [468](../../src/preset.rs#L468) | receiver-type-required |
| `from_layers` | `preset             .owned_sections` | [468](../../src/preset.rs#L468) | receiver-type-required |
| `from_layers` | `text.len` | [471](../../src/preset.rs#L471) | receiver-type-required |
| `from_layers` | `PresetInvalid::new(PresetReason::TextTooLarge, None, "preset").into` | [475](../../src/preset.rs#L475) | receiver-type-required |
| `from_layers` | `preset.compute_digest` | [477](../../src/preset.rs#L477) | receiver-type-required |
| `from_layers` | `Ok` | [478](../../src/preset.rs#L478) | external-constructor-callback-or-unresolved |
| `compute_digest` | `serde_json_canonicalizer::to_vec(&preimage).expect` | [483](../../src/preset.rs#L483) | receiver-type-required |
| `compute_digest` | `serde_json_canonicalizer::to_vec` | [483](../../src/preset.rs#L483) | external-constructor-callback-or-unresolved |
| `from_genesis` | `serde_json::from_value(value.clone())             .map_err` | [490](../../src/preset.rs#L490) | receiver-type-required |
| `from_genesis` | `serde_json::from_value` | [490](../../src/preset.rs#L490) | external-constructor-callback-or-unresolved |
| `from_genesis` | `value.clone` | [490](../../src/preset.rs#L490) | receiver-type-required |
| `from_genesis` | `Err` | [493](../../src/preset.rs#L493), [498](../../src/preset.rs#L498), [503](../../src/preset.rs#L503) | external-constructor-callback-or-unresolved |
| `from_genesis` | `preset.layers.iter().find` | [495](../../src/preset.rs#L495) | receiver-type-required |
| `from_genesis` | `preset.layers.iter` | [495](../../src/preset.rs#L495) | receiver-type-required |
| `from_genesis` | `Self::from_layers(preset.base, preset.layers.clone())             .map_err` | [500](../../src/preset.rs#L500) | receiver-type-required |
| `from_genesis` | `Self::from_layers` | [500](../../src/preset.rs#L500) | [tools::preset::ResolvedPreset::from_layers](../../src/preset.rs#L432) |
| `from_genesis` | `preset.layers.clone` | [500](../../src/preset.rs#L500) | receiver-type-required |
| `from_genesis` | `"genesis preset digest mismatch".to_owned` | [503](../../src/preset.rs#L503) | receiver-type-required |
| `from_genesis` | `Ok` | [505](../../src/preset.rs#L505) | external-constructor-callback-or-unresolved |
| `owned_sections` | `self.layers.iter().find_map` | [513](../../src/preset.rs#L513), [518](../../src/preset.rs#L518) | receiver-type-required |
| `owned_sections` | `self.layers.iter` | [513](../../src/preset.rs#L513), [518](../../src/preset.rs#L518), [523](../../src/preset.rs#L523) | receiver-type-required |
| `owned_sections` | `identity.as_deref` | [514](../../src/preset.rs#L514) | receiver-type-required |
| `owned_sections` | `inline             .or_else(&#124;&#124; self.layers.iter().find_map(PresetLayer::identity))             .or_else` | [517](../../src/preset.rs#L517) | receiver-type-required |
| `owned_sections` | `inline             .or_else` | [517](../../src/preset.rs#L517) | receiver-type-required |
| `owned_sections` | `self.base.identity` | [519](../../src/preset.rs#L519) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map(str::trim)             .filter(&#124;text&#124; !text.is_empty())             .collect` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map(str::trim)             .filter` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain(self.layers.iter().filter_map(PresetLayer::duty))             .map` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain(self.base.rules())             .chain` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `identity             .into_iter()             .chain` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `identity             .into_iter` | [520](../../src/preset.rs#L520) | receiver-type-required |
| `owned_sections` | `self.base.rules` | [522](../../src/preset.rs#L522) | receiver-type-required |
| `owned_sections` | `self.layers.iter().filter_map` | [523](../../src/preset.rs#L523) | receiver-type-required |
| `owned_sections` | `text.is_empty` | [525](../../src/preset.rs#L525) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain(self.owned_sections())             .collect::<Vec<_>>()             .join` | [533](../../src/preset.rs#L533) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain(self.owned_sections())             .collect::<Vec<_>>` | [533](../../src/preset.rs#L533) | receiver-type-required |
| `preset_text` | `std::iter::once(HARNESS_IDENTITY)             .chain` | [533](../../src/preset.rs#L533) | receiver-type-required |
| `preset_text` | `std::iter::once` | [533](../../src/preset.rs#L533) | external-constructor-callback-or-unresolved |
| `preset_text` | `self.owned_sections` | [534](../../src/preset.rs#L534) | [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L510) |
| `render` | `sections.extend` | [545](../../src/preset.rs#L545) | receiver-type-required |
| `render` | `self.owned_sections()                 .into_iter()                 .map` | [546](../../src/preset.rs#L546) | receiver-type-required |
| `render` | `self.owned_sections()                 .into_iter` | [546](../../src/preset.rs#L546) | receiver-type-required |
| `render` | `self.owned_sections` | [546](../../src/preset.rs#L546) | [tools::preset::ResolvedPreset::owned_sections](../../src/preset.rs#L510) |
| `render` | `text.replace` | [548](../../src/preset.rs#L548) | receiver-type-required |
| `render` | `sections.push` | [551](../../src/preset.rs#L551), [555](../../src/preset.rs#L555) | receiver-type-required |
| `render` | `WORKING_DIRECTORY.replace` | [551](../../src/preset.rs#L551) | receiver-type-required |
| `render` | `instruction_text.trim` | [553](../../src/preset.rs#L553) | receiver-type-required |
| `render` | `instruction_text.is_empty` | [554](../../src/preset.rs#L554) | receiver-type-required |
| `render` | `instruction_text.to_owned` | [555](../../src/preset.rs#L555) | receiver-type-required |
| `render` | `sections.join` | [557](../../src/preset.rs#L557) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then(Value::as_str)         .ok_or_else` | [577](../../src/preset.rs#L577) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then(&#124;object&#124; object.get("kind"))         .and_then` | [577](../../src/preset.rs#L577) | receiver-type-required |
| `resolve_layer` | `layer         .as_object()         .and_then` | [577](../../src/preset.rs#L577) | receiver-type-required |
| `resolve_layer` | `layer         .as_object` | [577](../../src/preset.rs#L577) | receiver-type-required |
| `resolve_layer` | `object.get` | [579](../../src/preset.rs#L579) | receiver-type-required |
| `resolve_layer` | `PresetError::BadRequest` | [581](../../src/preset.rs#L581), [586](../../src/preset.rs#L586), [609](../../src/preset.rs#L609), [626](../../src/preset.rs#L626), [683](../../src/preset.rs#L683) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `object_members` | [584](../../src/preset.rs#L584), [600](../../src/preset.rs#L600) | [tools::preset::object_members](../../src/preset.rs#L345) |
| `resolve_layer` | `string_member(object, "id", &path)?                 .ok_or_else` | [585](../../src/preset.rs#L585), [608](../../src/preset.rs#L608) | receiver-type-required |
| `resolve_layer` | `string_member` | [585](../../src/preset.rs#L585), [608](../../src/preset.rs#L608), [625](../../src/preset.rs#L625), [646](../../src/preset.rs#L646), [665](../../src/preset.rs#L665) | [tools::preset::string_member](../../src/preset.rs#L361) |
| `resolve_layer` | `kernel_layer(id).ok_or_else` | [587](../../src/preset.rs#L587) | receiver-type-required |
| `resolve_layer` | `kernel_layer` | [587](../../src/preset.rs#L587) | [tools::preset::kernel_layer](../../src/preset.rs#L96) |
| `resolve_layer` | `PresetInvalid::new` | [588](../../src/preset.rs#L588), [618](../../src/preset.rs#L618), [628](../../src/preset.rs#L628), [639](../../src/preset.rs#L639), [649](../../src/preset.rs#L649), [657](../../src/preset.rs#L657), [667](../../src/preset.rs#L667) | [tools::preset::PresetInvalid::new](../../src/preset.rs#L219) |
| `resolve_layer` | `Some` | [590](../../src/preset.rs#L590), [611](../../src/preset.rs#L611) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `id.to_owned` | [590](../../src/preset.rs#L590), [612](../../src/preset.rs#L612), [675](../../src/preset.rs#L675) | receiver-type-required |
| `resolve_layer` | `Ok` | [594](../../src/preset.rs#L594), [674](../../src/preset.rs#L674) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `known.id.to_owned` | [595](../../src/preset.rs#L595) | receiver-type-required |
| `resolve_layer` | `known.version.to_owned` | [596](../../src/preset.rs#L596) | receiver-type-required |
| `resolve_layer` | `valid_id` | [611](../../src/preset.rs#L611), [617](../../src/preset.rs#L617) | [tools::preset::valid_id](../../src/preset.rs#L330) |
| `resolve_layer` | `Err` | [618](../../src/preset.rs#L618), [628](../../src/preset.rs#L628), [639](../../src/preset.rs#L639), [649](../../src/preset.rs#L649), [657](../../src/preset.rs#L657), [667](../../src/preset.rs#L667), [683](../../src/preset.rs#L683) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::InvalidId,                     label(),                     format!("{path}.id"),                 )                 .into` | [618](../../src/preset.rs#L618) | receiver-type-required |
| `resolve_layer` | `label` | [620](../../src/preset.rs#L620), [630](../../src/preset.rs#L630), [641](../../src/preset.rs#L641), [651](../../src/preset.rs#L651), [659](../../src/preset.rs#L659), [669](../../src/preset.rs#L669) | external-constructor-callback-or-unresolved |
| `resolve_layer` | `string_member(object, "version", &path)?                 .ok_or_else` | [625](../../src/preset.rs#L625) | receiver-type-required |
| `resolve_layer` | `valid_version` | [627](../../src/preset.rs#L627) | [tools::preset::valid_version](../../src/preset.rs#L339) |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::InvalidId,                     label(),                     format!("{path}.version"),                 )                 .into` | [628](../../src/preset.rs#L628) | receiver-type-required |
| `resolve_layer` | `RESERVED_MEMBERS                 .iter()                 .find` | [635](../../src/preset.rs#L635) | receiver-type-required |
| `resolve_layer` | `RESERVED_MEMBERS                 .iter` | [635](../../src/preset.rs#L635) | receiver-type-required |
| `resolve_layer` | `object.contains_key` | [637](../../src/preset.rs#L637) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::UnsupportedField,                     label(),                     format!("{path}.{reserved}"),                 )                 .into` | [639](../../src/preset.rs#L639) | receiver-type-required |
| `resolve_layer` | `identity.len` | [648](../../src/preset.rs#L648) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                         PresetReason::TextTooLarge,                         label(),                         format!("{path}.identity"),                     )                     .into` | [649](../../src/preset.rs#L649) | receiver-type-required |
| `resolve_layer` | `identity.trim().is_empty` | [656](../../src/preset.rs#L656) | receiver-type-required |
| `resolve_layer` | `identity.trim` | [656](../../src/preset.rs#L656) | receiver-type-required |
| `resolve_layer` | `identity.trim().contains` | [656](../../src/preset.rs#L656) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                         PresetReason::InvalidIdentity,                         label(),                         format!("{path}.identity"),                     )                     .into` | [657](../../src/preset.rs#L657) | receiver-type-required |
| `resolve_layer` | `duty.is_some_and` | [666](../../src/preset.rs#L666) | receiver-type-required |
| `resolve_layer` | `duty.len` | [666](../../src/preset.rs#L666) | receiver-type-required |
| `resolve_layer` | `PresetInvalid::new(                     PresetReason::TextTooLarge,                     label(),                     format!("{path}.duty"),                 )                 .into` | [667](../../src/preset.rs#L667) | receiver-type-required |
| `resolve_layer` | `version.to_owned` | [676](../../src/preset.rs#L676) | receiver-type-required |
| `resolve_layer` | `identity.map` | [677](../../src/preset.rs#L677) | receiver-type-required |
| `resolve_layer` | `text.trim().to_owned` | [677](../../src/preset.rs#L677), [679](../../src/preset.rs#L679) | receiver-type-required |
| `resolve_layer` | `text.trim` | [677](../../src/preset.rs#L677), [679](../../src/preset.rs#L679) | receiver-type-required |
| `resolve_layer` | `duty                     .map(&#124;text&#124; text.trim().to_owned())                     .filter` | [678](../../src/preset.rs#L678) | receiver-type-required |
| `resolve_layer` | `duty                     .map` | [678](../../src/preset.rs#L678) | receiver-type-required |
| `resolve_layer` | `text.is_empty` | [680](../../src/preset.rs#L680) | receiver-type-required |
| `resolve` | `ResolvedPreset::resolve_request` | [721](../../src/preset.rs#L721) | external-constructor-callback-or-unresolved |
| `invalid` | `resolve` | [725](../../src/preset.rs#L725) | [tools::preset::tests::resolve](../../src/preset.rs#L720) |
| `sugar_renders_exactly_like_the_identity_profile` | `Some` | [734](../../src/preset.rs#L734), [735](../../src/preset.rs#L735) | external-constructor-callback-or-unresolved |
| `sugar_renders_exactly_like_the_identity_profile` | `ResolvedPreset::from_identity` | [735](../../src/preset.rs#L735) | external-constructor-callback-or-unresolved |
| `sugar_renders_exactly_like_the_identity_profile` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"}]})).unwrap` | [742](../../src/preset.rs#L742) | receiver-type-required |
| `sugar_renders_exactly_like_the_identity_profile` | `resolve` | [742](../../src/preset.rs#L742) | [tools::preset::tests::resolve](../../src/preset.rs#L720) |
| `blank_base_drops_identity_and_rules` | `ResolvedPreset::from_identity` | [751](../../src/preset.rs#L751), [757](../../src/preset.rs#L757) | external-constructor-callback-or-unresolved |
| `blank_base_drops_identity_and_rules` | `Some` | [757](../../src/preset.rs#L757) | external-constructor-callback-or-unresolved |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve(json!({"layers":[             {"kind":"kernel","id":"coding"},             {"kind":"inline","id":"cad-expert","version":"3","duty":"Prefer DXF tools."}         ]}))         .unwrap` | [769](../../src/preset.rs#L769) | receiver-type-required |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve` | [769](../../src/preset.rs#L769), [775](../../src/preset.rs#L775) | [tools::preset::tests::resolve](../../src/preset.rs#L720) |
| `inline_identity_replaces_the_base_identity_but_not_its_rules` | `resolve(json!({"layers":[{"kind":"inline","id":"cad-expert","version":"3",             "identity":"You are a CAD expert agent powered by the {model} model.","duty":"Prefer DXF tools."}]}))         .unwrap` | [775](../../src/preset.rs#L775) | receiver-type-required |
| `inline_identity_wins_over_a_kernel_layer_identity` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"},             {"kind":"inline","id":"cad-expert","version":"1",              "identity":"You are a CAD expert agent powered by the {model} model.","duty":"Prefer DXF tools."}]}))         .unwrap` | [796](../../src/preset.rs#L796) | receiver-type-required |
| `inline_identity_wins_over_a_kernel_layer_identity` | `resolve` | [796](../../src/preset.rs#L796) | [tools::preset::tests::resolve](../../src/preset.rs#L720) |
| `refusals_name_reason_layer_and_field` | `invalid` | [812](../../src/preset.rs#L812), [817](../../src/preset.rs#L817), [825](../../src/preset.rs#L825), [857](../../src/preset.rs#L857) | [tools::preset::tests::invalid](../../src/preset.rs#L724) |
| `refusals_name_reason_layer_and_field` | `(0..9)             .map(&#124;index&#124; json!({"kind":"inline","id":format!("l{index}"),"version":"1"}))             .collect::<Vec<_>>` | [837](../../src/preset.rs#L837) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..9)             .map` | [837](../../src/preset.rs#L837) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `"x".repeat` | [844](../../src/preset.rs#L844) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..3)             .map(&#124;index&#124; json!({"kind":"inline","id":format!("l{index}"),"version":"1","duty":"y".repeat(MAX_PRESET_DUTY_BYTES)}))             .collect::<Vec<_>>` | [849](../../src/preset.rs#L849) | receiver-type-required |
| `refusals_name_reason_layer_and_field` | `(0..3)             .map` | [849](../../src/preset.rs#L849) | receiver-type-required |
| `secrets_and_definitions_are_refused_before_shape_checks` | `invalid` | [870](../../src/preset.rs#L870), [873](../../src/preset.rs#L873) | [tools::preset::tests::invalid](../../src/preset.rs#L724) |
| `genesis_round_trip_checks_digest_and_layers` | `resolve(json!({"layers":[{"kind":"kernel","id":"coding"},             {"kind":"inline","id":"cad","version":"1","duty":"DXF."}]}))         .unwrap` | [903](../../src/preset.rs#L903) | receiver-type-required |
| `genesis_round_trip_checks_digest_and_layers` | `resolve` | [903](../../src/preset.rs#L903) | [tools::preset::tests::resolve](../../src/preset.rs#L720) |
| `genesis_round_trip_checks_digest_and_layers` | `serde_json::to_value(&preset).unwrap` | [906](../../src/preset.rs#L906) | receiver-type-required |
| `genesis_round_trip_checks_digest_and_layers` | `serde_json::to_value` | [906](../../src/preset.rs#L906) | external-constructor-callback-or-unresolved |
| `genesis_round_trip_checks_digest_and_layers` | `value.clone` | [908](../../src/preset.rs#L908) | receiver-type-required |
