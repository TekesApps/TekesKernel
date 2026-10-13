# provider::dialect

[Package atlas](index.md) · [Source](../../src/dialect.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [provider::dialect::DialectId](../../src/dialect.rs#L16) | enum_item | `pub` |  |
| [provider::dialect::DialectId::as_str](../../src/dialect.rs#L33) | function_item | `pub` |  |
| [provider::dialect::DialectId::family](../../src/dialect.rs#L51) | function_item | `pub` |  |
| [provider::dialect::DialectId::server_managed](../../src/dialect.rs#L67) | function_item | `pub` |  |
| [provider::dialect::DialectId::input_blocks](../../src/dialect.rs#L72) | function_item | `pub` |  |
| [provider::dialect::DialectId::supports_reasoning_blocks](../../src/dialect.rs#L93) | function_item | `pub` |  |
| [provider::dialect::DialectId::cache_policy](../../src/dialect.rs#L105) | function_item | `pub` |  |
| [provider::dialect::DialectId::sampling_policy](../../src/dialect.rs#L115) | function_item | `pub` |  |
| [provider::dialect::DialectId::schema_policy](../../src/dialect.rs#L120) | function_item | `pub` |  |
| [provider::dialect::DialectId::tool_choice_modes](../../src/dialect.rs#L129) | function_item | `pub` |  |
| [provider::dialect::DialectId::tool_choice_wire](../../src/dialect.rs#L134) | function_item | `pub` |  |
| [provider::dialect::DialectId::repair_id](../../src/dialect.rs#L139) | function_item | `pub` |  |
| [provider::dialect::Err](../../src/dialect.rs#L149) | type_item | `private` |  |
| [provider::dialect::DialectId::from_str](../../src/dialect.rs#L151) | function_item | `private` |  |
| [provider::dialect::RouteEvidence](../../src/dialect.rs#L172) | struct_item | `pub` |  |
| [provider::dialect::ProviderTarget](../../src/dialect.rs#L181) | struct_item | `pub` |  |
| [provider::dialect::ProviderTarget::dialect](../../src/dialect.rs#L189) | function_item | `pub` |  |
| [provider::dialect::ProviderTarget::family](../../src/dialect.rs#L193) | function_item | `pub` |  |
| [provider::dialect::Definition](../../src/dialect.rs#L208) | struct_item | `private` |  |
| [provider::dialect::SupportedDialect](../../src/dialect.rs#L220) | struct_item | `pub` |  |
| [provider::dialect::MODEL_CAPABILITY_CATALOG](../../src/dialect.rs#L225) | const_item | `private` |  |
| [provider::dialect::MODEL_CAPABILITY_CATALOG_SHA256](../../src/dialect.rs#L227) | const_item | `private` |  |
| [provider::dialect::ModelReasoningCapability](../../src/dialect.rs#L232) | struct_item | `private` |  |
| [provider::dialect::ThinkingWire](../../src/dialect.rs#L245) | enum_item | `pub` |  |
| [provider::dialect::NativeDeferredMode](../../src/dialect.rs#L258) | enum_item | `pub` |  |
| [provider::dialect::NativeDeferredMode::dialect](../../src/dialect.rs#L270) | function_item | `pub` |  |
| [provider::dialect::NativeDeferredRoute](../../src/dialect.rs#L280) | struct_item | `private` |  |
| [provider::dialect::NativeDeferredCapability](../../src/dialect.rs#L287) | struct_item | `private` |  |
| [provider::dialect::ModelCapabilityProfile](../../src/dialect.rs#L294) | struct_item | `private` |  |
| [provider::dialect::default_true](../../src/dialect.rs#L321) | function_item | `private` |  |
| [provider::dialect::ModelCapabilityCatalog](../../src/dialect.rs#L327) | struct_item | `private` |  |
| [provider::dialect::MODEL_CAPABILITIES](../../src/dialect.rs#L332) | static_item | `private` |  |
| [provider::dialect::DEFINITIONS](../../src/dialect.rs#L334) | const_item | `private` |  |
| [provider::dialect::ResolvedDialectProfile](../../src/dialect.rs#L422) | struct_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::wire_model](../../src/dialect.rs#L440) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::pro_reasoning](../../src/dialect.rs#L446) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::native_deferred_tools](../../src/dialect.rs#L453) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::reasoning_efforts](../../src/dialect.rs#L458) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::default_reasoning_effort](../../src/dialect.rs#L463) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::thinking_wire](../../src/dialect.rs#L469) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::forced_tool_choice](../../src/dialect.rs#L475) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::refusal_fallback](../../src/dialect.rs#L481) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::max_output_tokens](../../src/dialect.rs#L487) | function_item | `pub` |  |
| [provider::dialect::DialectError](../../src/dialect.rs#L493) | enum_item | `pub` |  |
| [provider::dialect::definition_for](../../src/dialect.rs#L512) | function_item | `private` |  |
| [provider::dialect::target_matches_definition](../../src/dialect.rs#L519) | function_item | `private` |  |
| [provider::dialect::load_model_capabilities](../../src/dialect.rs#L523) | function_item | `private` |  |
| [provider::dialect::model_capability_for](../../src/dialect.rs#L612) | function_item | `private` |  |
| [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L638) | function_item | `private` |  |
| [provider::dialect::supported_dialects](../../src/dialect.rs#L670) | function_item | `pub` |  |
| [provider::dialect::validate_provider](../../src/dialect.rs#L683) | function_item | `pub` |  |
| [provider::dialect::resolve_profile](../../src/dialect.rs#L692) | function_item | `pub` |  |
| [provider::dialect::validate_target](../../src/dialect.rs#L715) | function_item | `pub` |  |
| [provider::dialect::resolve_configured_target](../../src/dialect.rs#L724) | function_item | `private` |  |
| [provider::dialect::epoch_profile](../../src/dialect.rs#L783) | function_item | `pub` |  |
| [provider::dialect::validate_epoch_target](../../src/dialect.rs#L814) | function_item | `pub` |  |
| [provider::dialect::tests::unlisted_sku_inherits_a_dialect_uniform_reasoning_capability](../../src/dialect.rs#L838) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `FromStr` | `std::str::FromStr` | `private` |
| `OnceLock` | `std::sync::OnceLock` | `private` |
| `Model` | `profile::Model` | `private` |
| `Provider` | `profile::Provider` | `private` |
| `SessionSettings` | `profile::SessionSettings` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `AdapterId` | `crate::AdapterId` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `provider::dialect::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::DialectId::cache_policy"]
  n1["provider::dialect::DialectId::sampling_policy"]
  n2["provider::dialect::DialectId::schema_policy"]
  n3["provider::dialect::DialectId::tool_choice_modes"]
  n4["provider::dialect::DialectId::tool_choice_wire"]
  n5["provider::dialect::DialectId::repair_id"]
  n6["provider::dialect::DialectId::from_str"]
  n7["provider::dialect::ProviderTarget::dialect"]
  n8["provider::dialect::ProviderTarget::family"]
  n9["provider::dialect::NativeDeferredMode::dialect"]
  n10["provider::dialect::default_true"]
  n11["provider::dialect::DialectId::as_str"]
  n12["provider::dialect::ResolvedDialectProfile::wire_model"]
  n13["provider::dialect::ResolvedDialectProfile::pro_reasoning"]
  n14["provider::dialect::ResolvedDialectProfile::native_deferred_tools"]
  n15["provider::dialect::ResolvedDialectProfile::reasoning_efforts"]
  n16["provider::dialect::DialectId::family"]
  n17["provider::dialect::DialectId::server_managed"]
  n18["provider::dialect::DialectId::input_blocks"]
  n19["provider::dialect::DialectId::supports_reasoning_blocks"]
  n8 --> n7
```

</details>

<details><summary>Functions 21–37: 9 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::ResolvedDialectProfile::default_reasoning_effort"]
  n1["provider::dialect::ResolvedDialectProfile::thinking_wire"]
  n2["provider::dialect::ResolvedDialectProfile::forced_tool_choice"]
  n3["provider::dialect::ResolvedDialectProfile::refusal_fallback"]
  n4["provider::dialect::ResolvedDialectProfile::max_output_tokens"]
  n5["provider::dialect::definition_for"]
  n6["provider::dialect::target_matches_definition"]
  n7["provider::dialect::load_model_capabilities"]
  n8["provider::dialect::model_capability_for"]
  n9["provider::dialect::dialect_uniform_capability"]
  n10["provider::dialect::supported_dialects"]
  n11["provider::dialect::validate_provider"]
  n12["provider::dialect::resolve_profile"]
  n13["provider::dialect::validate_target"]
  n14["provider::dialect::resolve_configured_target"]
  n15["provider::dialect::epoch_profile"]
  n16["provider::dialect::validate_epoch_target"]
  n17["schema::ijson::IJsonValue::parse"]
  n8 --> n9
  n11 --> n5
  n12 --> n5
  n12 --> n14
  n13 --> n5
  n13 --> n6
  n13 --> n14
  n14 --> n8
  n15 --> n17
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `from_str` | `Ok` | [153](../../src/dialect.rs#L153), [154](../../src/dialect.rs#L154), [155](../../src/dialect.rs#L155), [156](../../src/dialect.rs#L156), [157](../../src/dialect.rs#L157), [158](../../src/dialect.rs#L158), [159](../../src/dialect.rs#L159), [160](../../src/dialect.rs#L160), [161](../../src/dialect.rs#L161), [162](../../src/dialect.rs#L162), [163](../../src/dialect.rs#L163), [164](../../src/dialect.rs#L164) | external-constructor-callback-or-unresolved |
| `from_str` | `Err` | [165](../../src/dialect.rs#L165) | external-constructor-callback-or-unresolved |
| `from_str` | `DialectError::UnknownDialect` | [165](../../src/dialect.rs#L165) | external-constructor-callback-or-unresolved |
| `from_str` | `other.to_owned` | [165](../../src/dialect.rs#L165) | receiver-type-required |
| `dialect` | `DialectId::from_str` | [190](../../src/dialect.rs#L190) | external-constructor-callback-or-unresolved |
| `family` | `self.dialect` | [194](../../src/dialect.rs#L194) | [provider::dialect::ProviderTarget::dialect](../../src/dialect.rs#L189) |
| `family` | `self.protocol_family.as_str` | [195](../../src/dialect.rs#L195) | receiver-type-required |
| `family` | `AdapterId::from_str(other)                 .map_err` | [198](../../src/dialect.rs#L198) | receiver-type-required |
| `family` | `AdapterId::from_str` | [198](../../src/dialect.rs#L198) | external-constructor-callback-or-unresolved |
| `family` | `DialectError::UnknownFamily` | [199](../../src/dialect.rs#L199) | external-constructor-callback-or-unresolved |
| `family` | `self.protocol_family.clone` | [199](../../src/dialect.rs#L199) | receiver-type-required |
| `family` | `dialect.family` | [201](../../src/dialect.rs#L201) | receiver-type-required |
| `family` | `Err` | [202](../../src/dialect.rs#L202) | external-constructor-callback-or-unresolved |
| `family` | `Ok` | [204](../../src/dialect.rs#L204) | external-constructor-callback-or-unresolved |
| `MODEL_CAPABILITIES` | `OnceLock::new` | [332](../../src/dialect.rs#L332) | external-constructor-callback-or-unresolved |
| `wire_model` | `self.wire_model             .as_deref()             .unwrap_or` | [441](../../src/dialect.rs#L441) | receiver-type-required |
| `wire_model` | `self.wire_model             .as_deref` | [441](../../src/dialect.rs#L441) | receiver-type-required |
| `default_reasoning_effort` | `self.default_reasoning_effort.as_deref` | [464](../../src/dialect.rs#L464) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find(&#124;definition&#124; definition.dialect == dialect)         .ok_or_else` | [513](../../src/dialect.rs#L513) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find` | [513](../../src/dialect.rs#L513) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter` | [513](../../src/dialect.rs#L513) | receiver-type-required |
| `definition_for` | `DialectError::UnknownDialect` | [516](../../src/dialect.rs#L516) | external-constructor-callback-or-unresolved |
| `definition_for` | `dialect.as_str().to_owned` | [516](../../src/dialect.rs#L516) | receiver-type-required |
| `definition_for` | `dialect.as_str` | [516](../../src/dialect.rs#L516) | receiver-type-required |
| `load_model_capabilities` | `Err` | [526](../../src/dialect.rs#L526), [533](../../src/dialect.rs#L533), [546](../../src/dialect.rs#L546), [554](../../src/dialect.rs#L554), [562](../../src/dialect.rs#L562), [576](../../src/dialect.rs#L576), [582](../../src/dialect.rs#L582), [593](../../src/dialect.rs#L593), [603](../../src/dialect.rs#L603) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `serde_json::from_slice(MODEL_CAPABILITY_CATALOG)         .map_err` | [530](../../src/dialect.rs#L530) | receiver-type-required |
| `load_model_capabilities` | `serde_json::from_slice` | [530](../../src/dialect.rs#L530) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `BTreeSet::new` | [538](../../src/dialect.rs#L538), [556](../../src/dialect.rs#L556) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.dialect_id.as_str` | [541](../../src/dialect.rs#L541) | receiver-type-required |
| `load_model_capabilities` | `profile.model_profile_id.as_str` | [542](../../src/dialect.rs#L542) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.as_str` | [543](../../src/dialect.rs#L543) | receiver-type-required |
| `load_model_capabilities` | `identities.insert` | [545](../../src/dialect.rs#L545) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id)             .map_err` | [551](../../src/dialect.rs#L551) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str` | [551](../../src/dialect.rs#L551), [568](../../src/dialect.rs#L568) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.model_profile_id.is_empty` | [553](../../src/dialect.rs#L553) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.is_empty` | [553](../../src/dialect.rs#L553) | receiver-type-required |
| `load_model_capabilities` | `"model capability identity must be nonempty".to_owned` | [554](../../src/dialect.rs#L554) | receiver-type-required |
| `load_model_capabilities` | `level.is_empty` | [558](../../src/dialect.rs#L558) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter().all` | [559](../../src/dialect.rs#L559) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter` | [559](../../src/dialect.rs#L559) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes` | [559](../../src/dialect.rs#L559) | receiver-type-required |
| `load_model_capabilities` | `levels.insert` | [560](../../src/dialect.rs#L560) | receiver-type-required |
| `load_model_capabilities` | `level.as_str` | [560](../../src/dialect.rs#L560) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id).expect` | [568](../../src/dialect.rs#L568) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref()             .is_some_and` | [569](../../src/dialect.rs#L569) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref` | [569](../../src/dialect.rs#L569) | receiver-type-required |
| `load_model_capabilities` | `model.is_empty` | [572](../../src/dialect.rs#L572) | receiver-type-required |
| `load_model_capabilities` | `profile.wire_model.is_none` | [574](../../src/dialect.rs#L574) | receiver-type-required |
| `load_model_capabilities` | `"invalid model wire variant capability".to_owned` | [576](../../src/dialect.rs#L576) | receiver-type-required |
| `load_model_capabilities` | `dialect.family` | [578](../../src/dialect.rs#L578) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.levels.is_empty` | [579](../../src/dialect.rs#L579) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.thinking.is_none` | [580](../../src/dialect.rs#L580) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref()             .is_some_and` | [587](../../src/dialect.rs#L587) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref` | [587](../../src/dialect.rs#L587) | receiver-type-required |
| `load_model_capabilities` | `levels.contains` | [591](../../src/dialect.rs#L591) | receiver-type-required |
| `load_model_capabilities` | `value.as_str` | [591](../../src/dialect.rs#L591) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref()             .is_some_and` | [598](../../src/dialect.rs#L598) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref` | [598](../../src/dialect.rs#L598) | receiver-type-required |
| `load_model_capabilities` | `url.starts_with` | [601](../../src/dialect.rs#L601) | receiver-type-required |
| `load_model_capabilities` | `Ok` | [609](../../src/dialect.rs#L609) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `MODEL_CAPABILITIES.get_or_init` | [615](../../src/dialect.rs#L615) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref()         .map_err` | [616](../../src/dialect.rs#L616) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref` | [616](../../src/dialect.rs#L616) | receiver-type-required |
| `model_capability_for` | `DialectError::CatalogUnavailable` | [618](../../src/dialect.rs#L618) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `error.clone` | [618](../../src/dialect.rs#L618) | receiver-type-required |
| `model_capability_for` | `catalog.iter().find` | [619](../../src/dialect.rs#L619) | receiver-type-required |
| `model_capability_for` | `catalog.iter` | [619](../../src/dialect.rs#L619) | receiver-type-required |
| `model_capability_for` | `Ok` | [624](../../src/dialect.rs#L624), [626](../../src/dialect.rs#L626) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `Some` | [624](../../src/dialect.rs#L624) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `exact.clone` | [624](../../src/dialect.rs#L624) | receiver-type-required |
| `model_capability_for` | `dialect_uniform_capability` | [626](../../src/dialect.rs#L626) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L638) |
| `dialect_uniform_capability` | `catalog         .iter()         .filter` | [643](../../src/dialect.rs#L643) | receiver-type-required |
| `dialect_uniform_capability` | `catalog         .iter` | [643](../../src/dialect.rs#L643) | receiver-type-required |
| `dialect_uniform_capability` | `family.next` | [646](../../src/dialect.rs#L646) | receiver-type-required |
| `dialect_uniform_capability` | `family.any` | [647](../../src/dialect.rs#L647) | receiver-type-required |
| `dialect_uniform_capability` | `Some` | [653](../../src/dialect.rs#L653) | external-constructor-callback-or-unresolved |
| `dialect_uniform_capability` | `dialect_id.to_owned` | [654](../../src/dialect.rs#L654) | receiver-type-required |
| `dialect_uniform_capability` | `exact_sku.to_owned` | [656](../../src/dialect.rs#L656) | receiver-type-required |
| `dialect_uniform_capability` | `first.reasoning.clone` | [659](../../src/dialect.rs#L659) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map(&#124;definition&#124; SupportedDialect {             dialect_id: definition.dialect.as_str().to_owned(),             protocol_family: definition.family.to_owned(),         })         .collect` | [671](../../src/dialect.rs#L671) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map` | [671](../../src/dialect.rs#L671) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter` | [671](../../src/dialect.rs#L671) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str().to_owned` | [674](../../src/dialect.rs#L674) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str` | [674](../../src/dialect.rs#L674) | receiver-type-required |
| `supported_dialects` | `definition.family.to_owned` | [675](../../src/dialect.rs#L675) | receiver-type-required |
| `validate_provider` | `DialectId::from_str` | [684](../../src/dialect.rs#L684) | external-constructor-callback-or-unresolved |
| `validate_provider` | `definition_for` | [685](../../src/dialect.rs#L685) | [provider::dialect::definition_for](../../src/dialect.rs#L512) |
| `validate_provider` | `Err` | [687](../../src/dialect.rs#L687) | external-constructor-callback-or-unresolved |
| `validate_provider` | `Ok` | [689](../../src/dialect.rs#L689) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `DialectId::from_str` | [696](../../src/dialect.rs#L696) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `definition_for` | [697](../../src/dialect.rs#L697) | [provider::dialect::definition_for](../../src/dialect.rs#L512) |
| `resolve_profile` | `Err` | [699](../../src/dialect.rs#L699) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `provider.adapter.clone` | [702](../../src/dialect.rs#L702) | receiver-type-required |
| `resolve_profile` | `provider.dialect.clone` | [703](../../src/dialect.rs#L703) | receiver-type-required |
| `resolve_profile` | `model.profile.clone` | [704](../../src/dialect.rs#L704) | receiver-type-required |
| `resolve_profile` | `provider.endpoint_owner.clone` | [706](../../src/dialect.rs#L706) | receiver-type-required |
| `resolve_profile` | `provider.gateway_translation.clone` | [707](../../src/dialect.rs#L707) | receiver-type-required |
| `resolve_profile` | `model.id.clone` | [708](../../src/dialect.rs#L708) | receiver-type-required |
| `resolve_profile` | `provider.evidence_revision.clone` | [709](../../src/dialect.rs#L709) | receiver-type-required |
| `resolve_profile` | `resolve_configured_target` | [712](../../src/dialect.rs#L712) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L724) |
| `validate_target` | `target.dialect` | [716](../../src/dialect.rs#L716) | receiver-type-required |
| `validate_target` | `definition_for` | [717](../../src/dialect.rs#L717) | [provider::dialect::definition_for](../../src/dialect.rs#L512) |
| `validate_target` | `target_matches_definition` | [718](../../src/dialect.rs#L718) | [provider::dialect::target_matches_definition](../../src/dialect.rs#L519) |
| `validate_target` | `Err` | [719](../../src/dialect.rs#L719) | external-constructor-callback-or-unresolved |
| `validate_target` | `resolve_configured_target` | [721](../../src/dialect.rs#L721) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L724) |
| `validate_target` | `target.clone` | [721](../../src/dialect.rs#L721) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_header.to_owned` | [729](../../src/dialect.rs#L729) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_prefix.to_owned` | [730](../../src/dialect.rs#L730) | receiver-type-required |
| `resolve_configured_target` | `"cf-aig-authorization".to_owned` | [735](../../src/dialect.rs#L735) | receiver-type-required |
| `resolve_configured_target` | `"Bearer ".to_owned` | [736](../../src/dialect.rs#L736) | receiver-type-required |
| `resolve_configured_target` | `model_capability_for` | [738](../../src/dialect.rs#L738) | [provider::dialect::model_capability_for](../../src/dialect.rs#L612) |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter(&#124;native&#124; {             native.mode.dialect() == dialect                 && native.routes.iter().any(&#124;route&#124; {                     route.endpoint_owner == target.route.endpoint_owner                         && route.gateway_translation == target.route.gateway_translation                 })         })         .map` | [739](../../src/dialect.rs#L739) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter` | [739](../../src/dialect.rs#L739) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then` | [739](../../src/dialect.rs#L739) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref` | [739](../../src/dialect.rs#L739) | receiver-type-required |
| `resolve_configured_target` | `value.native_deferred_tools.as_ref` | [741](../../src/dialect.rs#L741) | receiver-type-required |
| `resolve_configured_target` | `native.mode.dialect` | [743](../../src/dialect.rs#L743) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter().any` | [744](../../src/dialect.rs#L744) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter` | [744](../../src/dialect.rs#L744) | receiver-type-required |
| `resolve_configured_target` | `Ok` | [750](../../src/dialect.rs#L750) | external-constructor-callback-or-unresolved |
| `resolve_configured_target` | `capability             .as_ref()             .map(&#124;value&#124; value.reasoning.levels.clone())             .unwrap_or_default` | [756](../../src/dialect.rs#L756) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .map` | [756](../../src/dialect.rs#L756) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref` | [756](../../src/dialect.rs#L756), [760](../../src/dialect.rs#L760), [763](../../src/dialect.rs#L763), [766](../../src/dialect.rs#L766), [769](../../src/dialect.rs#L769), [772](../../src/dialect.rs#L772), [775](../../src/dialect.rs#L775) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.levels.clone` | [758](../../src/dialect.rs#L758) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .and_then` | [760](../../src/dialect.rs#L760), [763](../../src/dialect.rs#L763), [772](../../src/dialect.rs#L772), [775](../../src/dialect.rs#L775) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.default.clone` | [762](../../src/dialect.rs#L762) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_none_or` | [766](../../src/dialect.rs#L766) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_some_and` | [769](../../src/dialect.rs#L769) | receiver-type-required |
| `resolve_configured_target` | `value.wire_model.clone` | [777](../../src/dialect.rs#L777) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref().is_some_and` | [778](../../src/dialect.rs#L778) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref` | [778](../../src/dialect.rs#L778) | receiver-type-required |
| `epoch_profile` | `settings.and_then` | [788](../../src/dialect.rs#L788) | receiver-type-required |
| `epoch_profile` | `settings.reasoning_effort.as_deref` | [788](../../src/dialect.rs#L788) | receiver-type-required |
| `epoch_profile` | `requested.or_else` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `profile.default_reasoning_effort` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `effort.is_some_and` | [790](../../src/dialect.rs#L790) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter().any` | [790](../../src/dialect.rs#L790) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter` | [790](../../src/dialect.rs#L790) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts` | [790](../../src/dialect.rs#L790) | receiver-type-required |
| `epoch_profile` | `Err` | [791](../../src/dialect.rs#L791) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::UnsupportedControl` | [791](../../src/dialect.rs#L791) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `"reasoning_effort".to_owned` | [792](../../src/dialect.rs#L792), [798](../../src/dialect.rs#L798) | receiver-type-required |
| `epoch_profile` | `serde_json::Map::new` | [795](../../src/dialect.rs#L795) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `controls.insert` | [797](../../src/dialect.rs#L797) | receiver-type-required |
| `epoch_profile` | `Value::String` | [799](../../src/dialect.rs#L799) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `value.to_owned` | [799](../../src/dialect.rs#L799) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse(         &serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err(&#124;error&#124; DialectError::InvalidEpoch(error.to_string()))?,     )     .map_err` | [802](../../src/dialect.rs#L802) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse` | [802](../../src/dialect.rs#L802) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `epoch_profile` | `serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err` | [803](../../src/dialect.rs#L803) | receiver-type-required |
| `epoch_profile` | `serde_json_canonicalizer::to_vec` | [803](../../src/dialect.rs#L803) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::InvalidEpoch` | [809](../../src/dialect.rs#L809), [811](../../src/dialect.rs#L811) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `error.to_string` | [809](../../src/dialect.rs#L809), [811](../../src/dialect.rs#L811) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(epoch)         .map_err` | [818](../../src/dialect.rs#L818) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value` | [818](../../src/dialect.rs#L818), [823](../../src/dialect.rs#L823) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `DialectError::InvalidEpoch` | [819](../../src/dialect.rs#L819), [824](../../src/dialect.rs#L824) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `error.to_string` | [819](../../src/dialect.rs#L819), [824](../../src/dialect.rs#L824) | receiver-type-required |
| `validate_epoch_target` | `value         .get("target")         .ok_or` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `validate_epoch_target` | `value         .get` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(&profile.target)         .map_err` | [823](../../src/dialect.rs#L823) | receiver-type-required |
| `validate_epoch_target` | `Err` | [826](../../src/dialect.rs#L826), [830](../../src/dialect.rs#L830) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `value.get("serializer_revision").and_then` | [828](../../src/dialect.rs#L828) | receiver-type-required |
| `validate_epoch_target` | `value.get` | [828](../../src/dialect.rs#L828) | receiver-type-required |
| `validate_epoch_target` | `Some` | [828](../../src/dialect.rs#L828) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `Ok` | [832](../../src/dialect.rs#L832) | external-constructor-callback-or-unresolved |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref()             .expect` | [839](../../src/dialect.rs#L839) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref` | [839](../../src/dialect.rs#L839) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init` | [839](../../src/dialect.rs#L839) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_chat_v1", "deepseek-flash")                 .expect` | [844](../../src/dialect.rs#L844) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability` | [844](../../src/dialect.rs#L844), [855](../../src/dialect.rs#L855) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L638) |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_responses_v1", "unlisted")                 .expect` | [855](../../src/dialect.rs#L855) | receiver-type-required |
