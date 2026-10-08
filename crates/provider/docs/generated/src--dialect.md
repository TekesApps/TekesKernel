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
| [provider::dialect::DialectId::supports_reasoning_blocks](../../src/dialect.rs#L92) | function_item | `pub` |  |
| [provider::dialect::DialectId::cache_policy](../../src/dialect.rs#L104) | function_item | `pub` |  |
| [provider::dialect::DialectId::sampling_policy](../../src/dialect.rs#L114) | function_item | `pub` |  |
| [provider::dialect::DialectId::schema_policy](../../src/dialect.rs#L119) | function_item | `pub` |  |
| [provider::dialect::DialectId::tool_choice_modes](../../src/dialect.rs#L128) | function_item | `pub` |  |
| [provider::dialect::DialectId::tool_choice_wire](../../src/dialect.rs#L133) | function_item | `pub` |  |
| [provider::dialect::DialectId::repair_id](../../src/dialect.rs#L138) | function_item | `pub` |  |
| [provider::dialect::Err](../../src/dialect.rs#L148) | type_item | `private` |  |
| [provider::dialect::DialectId::from_str](../../src/dialect.rs#L150) | function_item | `private` |  |
| [provider::dialect::RouteEvidence](../../src/dialect.rs#L171) | struct_item | `pub` |  |
| [provider::dialect::ProviderTarget](../../src/dialect.rs#L180) | struct_item | `pub` |  |
| [provider::dialect::ProviderTarget::dialect](../../src/dialect.rs#L188) | function_item | `pub` |  |
| [provider::dialect::ProviderTarget::family](../../src/dialect.rs#L192) | function_item | `pub` |  |
| [provider::dialect::Definition](../../src/dialect.rs#L207) | struct_item | `private` |  |
| [provider::dialect::SupportedDialect](../../src/dialect.rs#L219) | struct_item | `pub` |  |
| [provider::dialect::MODEL_CAPABILITY_CATALOG](../../src/dialect.rs#L224) | const_item | `private` |  |
| [provider::dialect::MODEL_CAPABILITY_CATALOG_SHA256](../../src/dialect.rs#L226) | const_item | `private` |  |
| [provider::dialect::ModelReasoningCapability](../../src/dialect.rs#L231) | struct_item | `private` |  |
| [provider::dialect::ThinkingWire](../../src/dialect.rs#L244) | enum_item | `pub` |  |
| [provider::dialect::NativeDeferredMode](../../src/dialect.rs#L257) | enum_item | `pub` |  |
| [provider::dialect::NativeDeferredMode::dialect](../../src/dialect.rs#L269) | function_item | `pub` |  |
| [provider::dialect::NativeDeferredRoute](../../src/dialect.rs#L279) | struct_item | `private` |  |
| [provider::dialect::NativeDeferredCapability](../../src/dialect.rs#L286) | struct_item | `private` |  |
| [provider::dialect::ModelCapabilityProfile](../../src/dialect.rs#L293) | struct_item | `private` |  |
| [provider::dialect::default_true](../../src/dialect.rs#L320) | function_item | `private` |  |
| [provider::dialect::ModelCapabilityCatalog](../../src/dialect.rs#L326) | struct_item | `private` |  |
| [provider::dialect::MODEL_CAPABILITIES](../../src/dialect.rs#L331) | static_item | `private` |  |
| [provider::dialect::DEFINITIONS](../../src/dialect.rs#L333) | const_item | `private` |  |
| [provider::dialect::ResolvedDialectProfile](../../src/dialect.rs#L421) | struct_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::wire_model](../../src/dialect.rs#L439) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::pro_reasoning](../../src/dialect.rs#L445) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::native_deferred_tools](../../src/dialect.rs#L452) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::reasoning_efforts](../../src/dialect.rs#L457) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::default_reasoning_effort](../../src/dialect.rs#L462) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::thinking_wire](../../src/dialect.rs#L468) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::forced_tool_choice](../../src/dialect.rs#L474) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::refusal_fallback](../../src/dialect.rs#L480) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::max_output_tokens](../../src/dialect.rs#L486) | function_item | `pub` |  |
| [provider::dialect::DialectError](../../src/dialect.rs#L492) | enum_item | `pub` |  |
| [provider::dialect::definition_for](../../src/dialect.rs#L511) | function_item | `private` |  |
| [provider::dialect::target_matches_definition](../../src/dialect.rs#L518) | function_item | `private` |  |
| [provider::dialect::load_model_capabilities](../../src/dialect.rs#L522) | function_item | `private` |  |
| [provider::dialect::model_capability_for](../../src/dialect.rs#L611) | function_item | `private` |  |
| [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L637) | function_item | `private` |  |
| [provider::dialect::supported_dialects](../../src/dialect.rs#L669) | function_item | `pub` |  |
| [provider::dialect::validate_provider](../../src/dialect.rs#L682) | function_item | `pub` |  |
| [provider::dialect::resolve_profile](../../src/dialect.rs#L691) | function_item | `pub` |  |
| [provider::dialect::validate_target](../../src/dialect.rs#L714) | function_item | `pub` |  |
| [provider::dialect::resolve_configured_target](../../src/dialect.rs#L723) | function_item | `private` |  |
| [provider::dialect::epoch_profile](../../src/dialect.rs#L782) | function_item | `pub` |  |
| [provider::dialect::validate_epoch_target](../../src/dialect.rs#L813) | function_item | `pub` |  |
| [provider::dialect::tests::unlisted_sku_inherits_a_dialect_uniform_reasoning_capability](../../src/dialect.rs#L837) | function_item | `private` | test; #[cfg(test)] |

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
| `from_str` | `Ok` | [152](../../src/dialect.rs#L152), [153](../../src/dialect.rs#L153), [154](../../src/dialect.rs#L154), [155](../../src/dialect.rs#L155), [156](../../src/dialect.rs#L156), [157](../../src/dialect.rs#L157), [158](../../src/dialect.rs#L158), [159](../../src/dialect.rs#L159), [160](../../src/dialect.rs#L160), [161](../../src/dialect.rs#L161), [162](../../src/dialect.rs#L162), [163](../../src/dialect.rs#L163) | external-constructor-callback-or-unresolved |
| `from_str` | `Err` | [164](../../src/dialect.rs#L164) | external-constructor-callback-or-unresolved |
| `from_str` | `DialectError::UnknownDialect` | [164](../../src/dialect.rs#L164) | external-constructor-callback-or-unresolved |
| `from_str` | `other.to_owned` | [164](../../src/dialect.rs#L164) | receiver-type-required |
| `dialect` | `DialectId::from_str` | [189](../../src/dialect.rs#L189) | external-constructor-callback-or-unresolved |
| `family` | `self.dialect` | [193](../../src/dialect.rs#L193) | [provider::dialect::ProviderTarget::dialect](../../src/dialect.rs#L188) |
| `family` | `self.protocol_family.as_str` | [194](../../src/dialect.rs#L194) | receiver-type-required |
| `family` | `AdapterId::from_str(other)                 .map_err` | [197](../../src/dialect.rs#L197) | receiver-type-required |
| `family` | `AdapterId::from_str` | [197](../../src/dialect.rs#L197) | external-constructor-callback-or-unresolved |
| `family` | `DialectError::UnknownFamily` | [198](../../src/dialect.rs#L198) | external-constructor-callback-or-unresolved |
| `family` | `self.protocol_family.clone` | [198](../../src/dialect.rs#L198) | receiver-type-required |
| `family` | `dialect.family` | [200](../../src/dialect.rs#L200) | receiver-type-required |
| `family` | `Err` | [201](../../src/dialect.rs#L201) | external-constructor-callback-or-unresolved |
| `family` | `Ok` | [203](../../src/dialect.rs#L203) | external-constructor-callback-or-unresolved |
| `MODEL_CAPABILITIES` | `OnceLock::new` | [331](../../src/dialect.rs#L331) | external-constructor-callback-or-unresolved |
| `wire_model` | `self.wire_model             .as_deref()             .unwrap_or` | [440](../../src/dialect.rs#L440) | receiver-type-required |
| `wire_model` | `self.wire_model             .as_deref` | [440](../../src/dialect.rs#L440) | receiver-type-required |
| `default_reasoning_effort` | `self.default_reasoning_effort.as_deref` | [463](../../src/dialect.rs#L463) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find(&#124;definition&#124; definition.dialect == dialect)         .ok_or_else` | [512](../../src/dialect.rs#L512) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find` | [512](../../src/dialect.rs#L512) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter` | [512](../../src/dialect.rs#L512) | receiver-type-required |
| `definition_for` | `DialectError::UnknownDialect` | [515](../../src/dialect.rs#L515) | external-constructor-callback-or-unresolved |
| `definition_for` | `dialect.as_str().to_owned` | [515](../../src/dialect.rs#L515) | receiver-type-required |
| `definition_for` | `dialect.as_str` | [515](../../src/dialect.rs#L515) | receiver-type-required |
| `load_model_capabilities` | `Err` | [525](../../src/dialect.rs#L525), [532](../../src/dialect.rs#L532), [545](../../src/dialect.rs#L545), [553](../../src/dialect.rs#L553), [561](../../src/dialect.rs#L561), [575](../../src/dialect.rs#L575), [581](../../src/dialect.rs#L581), [592](../../src/dialect.rs#L592), [602](../../src/dialect.rs#L602) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `serde_json::from_slice(MODEL_CAPABILITY_CATALOG)         .map_err` | [529](../../src/dialect.rs#L529) | receiver-type-required |
| `load_model_capabilities` | `serde_json::from_slice` | [529](../../src/dialect.rs#L529) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `BTreeSet::new` | [537](../../src/dialect.rs#L537), [555](../../src/dialect.rs#L555) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.dialect_id.as_str` | [540](../../src/dialect.rs#L540) | receiver-type-required |
| `load_model_capabilities` | `profile.model_profile_id.as_str` | [541](../../src/dialect.rs#L541) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.as_str` | [542](../../src/dialect.rs#L542) | receiver-type-required |
| `load_model_capabilities` | `identities.insert` | [544](../../src/dialect.rs#L544) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id)             .map_err` | [550](../../src/dialect.rs#L550) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str` | [550](../../src/dialect.rs#L550), [567](../../src/dialect.rs#L567) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.model_profile_id.is_empty` | [552](../../src/dialect.rs#L552) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.is_empty` | [552](../../src/dialect.rs#L552) | receiver-type-required |
| `load_model_capabilities` | `"model capability identity must be nonempty".to_owned` | [553](../../src/dialect.rs#L553) | receiver-type-required |
| `load_model_capabilities` | `level.is_empty` | [557](../../src/dialect.rs#L557) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter().all` | [558](../../src/dialect.rs#L558) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter` | [558](../../src/dialect.rs#L558) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes` | [558](../../src/dialect.rs#L558) | receiver-type-required |
| `load_model_capabilities` | `levels.insert` | [559](../../src/dialect.rs#L559) | receiver-type-required |
| `load_model_capabilities` | `level.as_str` | [559](../../src/dialect.rs#L559) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id).expect` | [567](../../src/dialect.rs#L567) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref()             .is_some_and` | [568](../../src/dialect.rs#L568) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref` | [568](../../src/dialect.rs#L568) | receiver-type-required |
| `load_model_capabilities` | `model.is_empty` | [571](../../src/dialect.rs#L571) | receiver-type-required |
| `load_model_capabilities` | `profile.wire_model.is_none` | [573](../../src/dialect.rs#L573) | receiver-type-required |
| `load_model_capabilities` | `"invalid model wire variant capability".to_owned` | [575](../../src/dialect.rs#L575) | receiver-type-required |
| `load_model_capabilities` | `dialect.family` | [577](../../src/dialect.rs#L577) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.levels.is_empty` | [578](../../src/dialect.rs#L578) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.thinking.is_none` | [579](../../src/dialect.rs#L579) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref()             .is_some_and` | [586](../../src/dialect.rs#L586) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref` | [586](../../src/dialect.rs#L586) | receiver-type-required |
| `load_model_capabilities` | `levels.contains` | [590](../../src/dialect.rs#L590) | receiver-type-required |
| `load_model_capabilities` | `value.as_str` | [590](../../src/dialect.rs#L590) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref()             .is_some_and` | [597](../../src/dialect.rs#L597) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref` | [597](../../src/dialect.rs#L597) | receiver-type-required |
| `load_model_capabilities` | `url.starts_with` | [600](../../src/dialect.rs#L600) | receiver-type-required |
| `load_model_capabilities` | `Ok` | [608](../../src/dialect.rs#L608) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `MODEL_CAPABILITIES.get_or_init` | [614](../../src/dialect.rs#L614) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref()         .map_err` | [615](../../src/dialect.rs#L615) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref` | [615](../../src/dialect.rs#L615) | receiver-type-required |
| `model_capability_for` | `DialectError::CatalogUnavailable` | [617](../../src/dialect.rs#L617) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `error.clone` | [617](../../src/dialect.rs#L617) | receiver-type-required |
| `model_capability_for` | `catalog.iter().find` | [618](../../src/dialect.rs#L618) | receiver-type-required |
| `model_capability_for` | `catalog.iter` | [618](../../src/dialect.rs#L618) | receiver-type-required |
| `model_capability_for` | `Ok` | [623](../../src/dialect.rs#L623), [625](../../src/dialect.rs#L625) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `Some` | [623](../../src/dialect.rs#L623) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `exact.clone` | [623](../../src/dialect.rs#L623) | receiver-type-required |
| `model_capability_for` | `dialect_uniform_capability` | [625](../../src/dialect.rs#L625) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L637) |
| `dialect_uniform_capability` | `catalog         .iter()         .filter` | [642](../../src/dialect.rs#L642) | receiver-type-required |
| `dialect_uniform_capability` | `catalog         .iter` | [642](../../src/dialect.rs#L642) | receiver-type-required |
| `dialect_uniform_capability` | `family.next` | [645](../../src/dialect.rs#L645) | receiver-type-required |
| `dialect_uniform_capability` | `family.any` | [646](../../src/dialect.rs#L646) | receiver-type-required |
| `dialect_uniform_capability` | `Some` | [652](../../src/dialect.rs#L652) | external-constructor-callback-or-unresolved |
| `dialect_uniform_capability` | `dialect_id.to_owned` | [653](../../src/dialect.rs#L653) | receiver-type-required |
| `dialect_uniform_capability` | `exact_sku.to_owned` | [655](../../src/dialect.rs#L655) | receiver-type-required |
| `dialect_uniform_capability` | `first.reasoning.clone` | [658](../../src/dialect.rs#L658) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map(&#124;definition&#124; SupportedDialect {             dialect_id: definition.dialect.as_str().to_owned(),             protocol_family: definition.family.to_owned(),         })         .collect` | [670](../../src/dialect.rs#L670) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map` | [670](../../src/dialect.rs#L670) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter` | [670](../../src/dialect.rs#L670) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str().to_owned` | [673](../../src/dialect.rs#L673) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str` | [673](../../src/dialect.rs#L673) | receiver-type-required |
| `supported_dialects` | `definition.family.to_owned` | [674](../../src/dialect.rs#L674) | receiver-type-required |
| `validate_provider` | `DialectId::from_str` | [683](../../src/dialect.rs#L683) | external-constructor-callback-or-unresolved |
| `validate_provider` | `definition_for` | [684](../../src/dialect.rs#L684) | [provider::dialect::definition_for](../../src/dialect.rs#L511) |
| `validate_provider` | `Err` | [686](../../src/dialect.rs#L686) | external-constructor-callback-or-unresolved |
| `validate_provider` | `Ok` | [688](../../src/dialect.rs#L688) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `DialectId::from_str` | [695](../../src/dialect.rs#L695) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `definition_for` | [696](../../src/dialect.rs#L696) | [provider::dialect::definition_for](../../src/dialect.rs#L511) |
| `resolve_profile` | `Err` | [698](../../src/dialect.rs#L698) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `provider.adapter.clone` | [701](../../src/dialect.rs#L701) | receiver-type-required |
| `resolve_profile` | `provider.dialect.clone` | [702](../../src/dialect.rs#L702) | receiver-type-required |
| `resolve_profile` | `model.profile.clone` | [703](../../src/dialect.rs#L703) | receiver-type-required |
| `resolve_profile` | `provider.endpoint_owner.clone` | [705](../../src/dialect.rs#L705) | receiver-type-required |
| `resolve_profile` | `provider.gateway_translation.clone` | [706](../../src/dialect.rs#L706) | receiver-type-required |
| `resolve_profile` | `model.id.clone` | [707](../../src/dialect.rs#L707) | receiver-type-required |
| `resolve_profile` | `provider.evidence_revision.clone` | [708](../../src/dialect.rs#L708) | receiver-type-required |
| `resolve_profile` | `resolve_configured_target` | [711](../../src/dialect.rs#L711) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L723) |
| `validate_target` | `target.dialect` | [715](../../src/dialect.rs#L715) | receiver-type-required |
| `validate_target` | `definition_for` | [716](../../src/dialect.rs#L716) | [provider::dialect::definition_for](../../src/dialect.rs#L511) |
| `validate_target` | `target_matches_definition` | [717](../../src/dialect.rs#L717) | [provider::dialect::target_matches_definition](../../src/dialect.rs#L518) |
| `validate_target` | `Err` | [718](../../src/dialect.rs#L718) | external-constructor-callback-or-unresolved |
| `validate_target` | `resolve_configured_target` | [720](../../src/dialect.rs#L720) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L723) |
| `validate_target` | `target.clone` | [720](../../src/dialect.rs#L720) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_header.to_owned` | [728](../../src/dialect.rs#L728) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_prefix.to_owned` | [729](../../src/dialect.rs#L729) | receiver-type-required |
| `resolve_configured_target` | `"cf-aig-authorization".to_owned` | [734](../../src/dialect.rs#L734) | receiver-type-required |
| `resolve_configured_target` | `"Bearer ".to_owned` | [735](../../src/dialect.rs#L735) | receiver-type-required |
| `resolve_configured_target` | `model_capability_for` | [737](../../src/dialect.rs#L737) | [provider::dialect::model_capability_for](../../src/dialect.rs#L611) |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter(&#124;native&#124; {             native.mode.dialect() == dialect                 && native.routes.iter().any(&#124;route&#124; {                     route.endpoint_owner == target.route.endpoint_owner                         && route.gateway_translation == target.route.gateway_translation                 })         })         .map` | [738](../../src/dialect.rs#L738) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter` | [738](../../src/dialect.rs#L738) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then` | [738](../../src/dialect.rs#L738) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref` | [738](../../src/dialect.rs#L738) | receiver-type-required |
| `resolve_configured_target` | `value.native_deferred_tools.as_ref` | [740](../../src/dialect.rs#L740) | receiver-type-required |
| `resolve_configured_target` | `native.mode.dialect` | [742](../../src/dialect.rs#L742) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter().any` | [743](../../src/dialect.rs#L743) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter` | [743](../../src/dialect.rs#L743) | receiver-type-required |
| `resolve_configured_target` | `Ok` | [749](../../src/dialect.rs#L749) | external-constructor-callback-or-unresolved |
| `resolve_configured_target` | `capability             .as_ref()             .map(&#124;value&#124; value.reasoning.levels.clone())             .unwrap_or_default` | [755](../../src/dialect.rs#L755) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .map` | [755](../../src/dialect.rs#L755) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref` | [755](../../src/dialect.rs#L755), [759](../../src/dialect.rs#L759), [762](../../src/dialect.rs#L762), [765](../../src/dialect.rs#L765), [768](../../src/dialect.rs#L768), [771](../../src/dialect.rs#L771), [774](../../src/dialect.rs#L774) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.levels.clone` | [757](../../src/dialect.rs#L757) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .and_then` | [759](../../src/dialect.rs#L759), [762](../../src/dialect.rs#L762), [771](../../src/dialect.rs#L771), [774](../../src/dialect.rs#L774) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.default.clone` | [761](../../src/dialect.rs#L761) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_none_or` | [765](../../src/dialect.rs#L765) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_some_and` | [768](../../src/dialect.rs#L768) | receiver-type-required |
| `resolve_configured_target` | `value.wire_model.clone` | [776](../../src/dialect.rs#L776) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref().is_some_and` | [777](../../src/dialect.rs#L777) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref` | [777](../../src/dialect.rs#L777) | receiver-type-required |
| `epoch_profile` | `settings.and_then` | [787](../../src/dialect.rs#L787) | receiver-type-required |
| `epoch_profile` | `settings.reasoning_effort.as_deref` | [787](../../src/dialect.rs#L787) | receiver-type-required |
| `epoch_profile` | `requested.or_else` | [788](../../src/dialect.rs#L788) | receiver-type-required |
| `epoch_profile` | `profile.default_reasoning_effort` | [788](../../src/dialect.rs#L788) | receiver-type-required |
| `epoch_profile` | `effort.is_some_and` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter().any` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts` | [789](../../src/dialect.rs#L789) | receiver-type-required |
| `epoch_profile` | `Err` | [790](../../src/dialect.rs#L790) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::UnsupportedControl` | [790](../../src/dialect.rs#L790) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `"reasoning_effort".to_owned` | [791](../../src/dialect.rs#L791), [797](../../src/dialect.rs#L797) | receiver-type-required |
| `epoch_profile` | `serde_json::Map::new` | [794](../../src/dialect.rs#L794) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `controls.insert` | [796](../../src/dialect.rs#L796) | receiver-type-required |
| `epoch_profile` | `Value::String` | [798](../../src/dialect.rs#L798) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `value.to_owned` | [798](../../src/dialect.rs#L798) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse(         &serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err(&#124;error&#124; DialectError::InvalidEpoch(error.to_string()))?,     )     .map_err` | [801](../../src/dialect.rs#L801) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse` | [801](../../src/dialect.rs#L801) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `epoch_profile` | `serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err` | [802](../../src/dialect.rs#L802) | receiver-type-required |
| `epoch_profile` | `serde_json_canonicalizer::to_vec` | [802](../../src/dialect.rs#L802) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::InvalidEpoch` | [808](../../src/dialect.rs#L808), [810](../../src/dialect.rs#L810) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `error.to_string` | [808](../../src/dialect.rs#L808), [810](../../src/dialect.rs#L810) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(epoch)         .map_err` | [817](../../src/dialect.rs#L817) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value` | [817](../../src/dialect.rs#L817), [822](../../src/dialect.rs#L822) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `DialectError::InvalidEpoch` | [818](../../src/dialect.rs#L818), [823](../../src/dialect.rs#L823) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `error.to_string` | [818](../../src/dialect.rs#L818), [823](../../src/dialect.rs#L823) | receiver-type-required |
| `validate_epoch_target` | `value         .get("target")         .ok_or` | [819](../../src/dialect.rs#L819) | receiver-type-required |
| `validate_epoch_target` | `value         .get` | [819](../../src/dialect.rs#L819) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(&profile.target)         .map_err` | [822](../../src/dialect.rs#L822) | receiver-type-required |
| `validate_epoch_target` | `Err` | [825](../../src/dialect.rs#L825), [829](../../src/dialect.rs#L829) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `value.get("serializer_revision").and_then` | [827](../../src/dialect.rs#L827) | receiver-type-required |
| `validate_epoch_target` | `value.get` | [827](../../src/dialect.rs#L827) | receiver-type-required |
| `validate_epoch_target` | `Some` | [827](../../src/dialect.rs#L827) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `Ok` | [831](../../src/dialect.rs#L831) | external-constructor-callback-or-unresolved |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref()             .expect` | [838](../../src/dialect.rs#L838) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref` | [838](../../src/dialect.rs#L838) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init` | [838](../../src/dialect.rs#L838) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_chat_v1", "deepseek-flash")                 .expect` | [843](../../src/dialect.rs#L843) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability` | [843](../../src/dialect.rs#L843), [854](../../src/dialect.rs#L854) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L637) |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_responses_v1", "unlisted")                 .expect` | [854](../../src/dialect.rs#L854) | receiver-type-required |
