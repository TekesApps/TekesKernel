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
| [provider::dialect::NativeDeferredMode](../../src/dialect.rs#L268) | enum_item | `pub` |  |
| [provider::dialect::NativeDeferredMode::dialect](../../src/dialect.rs#L280) | function_item | `pub` |  |
| [provider::dialect::NativeDeferredRoute](../../src/dialect.rs#L290) | struct_item | `private` |  |
| [provider::dialect::NativeDeferredCapability](../../src/dialect.rs#L297) | struct_item | `private` |  |
| [provider::dialect::ModelCapabilityProfile](../../src/dialect.rs#L304) | struct_item | `private` |  |
| [provider::dialect::default_true](../../src/dialect.rs#L331) | function_item | `private` |  |
| [provider::dialect::ModelCapabilityCatalog](../../src/dialect.rs#L337) | struct_item | `private` |  |
| [provider::dialect::MODEL_CAPABILITIES](../../src/dialect.rs#L342) | static_item | `private` |  |
| [provider::dialect::DEFINITIONS](../../src/dialect.rs#L344) | const_item | `private` |  |
| [provider::dialect::ResolvedDialectProfile](../../src/dialect.rs#L432) | struct_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::wire_model](../../src/dialect.rs#L450) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::pro_reasoning](../../src/dialect.rs#L456) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::native_deferred_tools](../../src/dialect.rs#L463) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::reasoning_efforts](../../src/dialect.rs#L468) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::default_reasoning_effort](../../src/dialect.rs#L473) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::thinking_wire](../../src/dialect.rs#L479) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::forced_tool_choice](../../src/dialect.rs#L485) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::refusal_fallback](../../src/dialect.rs#L491) | function_item | `pub` |  |
| [provider::dialect::ResolvedDialectProfile::max_output_tokens](../../src/dialect.rs#L497) | function_item | `pub` |  |
| [provider::dialect::DialectError](../../src/dialect.rs#L503) | enum_item | `pub` |  |
| [provider::dialect::definition_for](../../src/dialect.rs#L522) | function_item | `private` |  |
| [provider::dialect::target_matches_definition](../../src/dialect.rs#L529) | function_item | `private` |  |
| [provider::dialect::load_model_capabilities](../../src/dialect.rs#L533) | function_item | `private` |  |
| [provider::dialect::model_capability_for](../../src/dialect.rs#L634) | function_item | `private` |  |
| [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L660) | function_item | `private` |  |
| [provider::dialect::supported_dialects](../../src/dialect.rs#L692) | function_item | `pub` |  |
| [provider::dialect::validate_provider](../../src/dialect.rs#L705) | function_item | `pub` |  |
| [provider::dialect::resolve_profile](../../src/dialect.rs#L714) | function_item | `pub` |  |
| [provider::dialect::validate_target](../../src/dialect.rs#L737) | function_item | `pub` |  |
| [provider::dialect::resolve_configured_target](../../src/dialect.rs#L746) | function_item | `private` |  |
| [provider::dialect::epoch_profile](../../src/dialect.rs#L813) | function_item | `pub` |  |
| [provider::dialect::validate_epoch_target](../../src/dialect.rs#L844) | function_item | `pub` |  |
| [provider::dialect::tests::unlisted_sku_inherits_a_dialect_uniform_reasoning_capability](../../src/dialect.rs#L868) | function_item | `private` | test; #[cfg(test)] |

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
| `MODEL_CAPABILITIES` | `OnceLock::new` | [342](../../src/dialect.rs#L342) | external-constructor-callback-or-unresolved |
| `wire_model` | `self.wire_model             .as_deref()             .unwrap_or` | [451](../../src/dialect.rs#L451) | receiver-type-required |
| `wire_model` | `self.wire_model             .as_deref` | [451](../../src/dialect.rs#L451) | receiver-type-required |
| `default_reasoning_effort` | `self.default_reasoning_effort.as_deref` | [474](../../src/dialect.rs#L474) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find(&#124;definition&#124; definition.dialect == dialect)         .ok_or_else` | [523](../../src/dialect.rs#L523) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter()         .find` | [523](../../src/dialect.rs#L523) | receiver-type-required |
| `definition_for` | `DEFINITIONS         .iter` | [523](../../src/dialect.rs#L523) | receiver-type-required |
| `definition_for` | `DialectError::UnknownDialect` | [526](../../src/dialect.rs#L526) | external-constructor-callback-or-unresolved |
| `definition_for` | `dialect.as_str().to_owned` | [526](../../src/dialect.rs#L526) | receiver-type-required |
| `definition_for` | `dialect.as_str` | [526](../../src/dialect.rs#L526) | receiver-type-required |
| `load_model_capabilities` | `Err` | [536](../../src/dialect.rs#L536), [543](../../src/dialect.rs#L543), [556](../../src/dialect.rs#L556), [564](../../src/dialect.rs#L564), [572](../../src/dialect.rs#L572), [586](../../src/dialect.rs#L586), [592](../../src/dialect.rs#L592), [604](../../src/dialect.rs#L604), [615](../../src/dialect.rs#L615), [625](../../src/dialect.rs#L625) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `serde_json::from_slice(MODEL_CAPABILITY_CATALOG)         .map_err` | [540](../../src/dialect.rs#L540) | receiver-type-required |
| `load_model_capabilities` | `serde_json::from_slice` | [540](../../src/dialect.rs#L540) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `BTreeSet::new` | [548](../../src/dialect.rs#L548), [566](../../src/dialect.rs#L566) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.dialect_id.as_str` | [551](../../src/dialect.rs#L551) | receiver-type-required |
| `load_model_capabilities` | `profile.model_profile_id.as_str` | [552](../../src/dialect.rs#L552) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.as_str` | [553](../../src/dialect.rs#L553) | receiver-type-required |
| `load_model_capabilities` | `identities.insert` | [555](../../src/dialect.rs#L555) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id)             .map_err` | [561](../../src/dialect.rs#L561) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str` | [561](../../src/dialect.rs#L561), [578](../../src/dialect.rs#L578) | external-constructor-callback-or-unresolved |
| `load_model_capabilities` | `profile.model_profile_id.is_empty` | [563](../../src/dialect.rs#L563) | receiver-type-required |
| `load_model_capabilities` | `profile.exact_sku.is_empty` | [563](../../src/dialect.rs#L563) | receiver-type-required |
| `load_model_capabilities` | `"model capability identity must be nonempty".to_owned` | [564](../../src/dialect.rs#L564) | receiver-type-required |
| `load_model_capabilities` | `level.is_empty` | [568](../../src/dialect.rs#L568) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter().all` | [569](../../src/dialect.rs#L569) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes().iter` | [569](../../src/dialect.rs#L569) | receiver-type-required |
| `load_model_capabilities` | `level.as_bytes` | [569](../../src/dialect.rs#L569) | receiver-type-required |
| `load_model_capabilities` | `levels.insert` | [570](../../src/dialect.rs#L570) | receiver-type-required |
| `load_model_capabilities` | `level.as_str` | [570](../../src/dialect.rs#L570) | receiver-type-required |
| `load_model_capabilities` | `DialectId::from_str(&profile.dialect_id).expect` | [578](../../src/dialect.rs#L578) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref()             .is_some_and` | [579](../../src/dialect.rs#L579) | receiver-type-required |
| `load_model_capabilities` | `profile             .wire_model             .as_ref` | [579](../../src/dialect.rs#L579) | receiver-type-required |
| `load_model_capabilities` | `model.is_empty` | [582](../../src/dialect.rs#L582) | receiver-type-required |
| `load_model_capabilities` | `profile.wire_model.is_none` | [584](../../src/dialect.rs#L584) | receiver-type-required |
| `load_model_capabilities` | `"invalid model wire variant capability".to_owned` | [586](../../src/dialect.rs#L586) | receiver-type-required |
| `load_model_capabilities` | `dialect.family` | [588](../../src/dialect.rs#L588) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.levels.is_empty` | [589](../../src/dialect.rs#L589), [598](../../src/dialect.rs#L598) | receiver-type-required |
| `load_model_capabilities` | `profile.reasoning.thinking.is_none` | [590](../../src/dialect.rs#L590) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref()             .is_some_and` | [609](../../src/dialect.rs#L609) | receiver-type-required |
| `load_model_capabilities` | `profile             .reasoning             .default             .as_ref` | [609](../../src/dialect.rs#L609) | receiver-type-required |
| `load_model_capabilities` | `levels.contains` | [613](../../src/dialect.rs#L613) | receiver-type-required |
| `load_model_capabilities` | `value.as_str` | [613](../../src/dialect.rs#L613) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref()             .is_some_and` | [620](../../src/dialect.rs#L620) | receiver-type-required |
| `load_model_capabilities` | `profile             .evidence_url             .as_ref` | [620](../../src/dialect.rs#L620) | receiver-type-required |
| `load_model_capabilities` | `url.starts_with` | [623](../../src/dialect.rs#L623) | receiver-type-required |
| `load_model_capabilities` | `Ok` | [631](../../src/dialect.rs#L631) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `MODEL_CAPABILITIES.get_or_init` | [637](../../src/dialect.rs#L637) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref()         .map_err` | [638](../../src/dialect.rs#L638) | receiver-type-required |
| `model_capability_for` | `catalog         .as_ref` | [638](../../src/dialect.rs#L638) | receiver-type-required |
| `model_capability_for` | `DialectError::CatalogUnavailable` | [640](../../src/dialect.rs#L640) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `error.clone` | [640](../../src/dialect.rs#L640) | receiver-type-required |
| `model_capability_for` | `catalog.iter().find` | [641](../../src/dialect.rs#L641) | receiver-type-required |
| `model_capability_for` | `catalog.iter` | [641](../../src/dialect.rs#L641) | receiver-type-required |
| `model_capability_for` | `Ok` | [646](../../src/dialect.rs#L646), [648](../../src/dialect.rs#L648) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `Some` | [646](../../src/dialect.rs#L646) | external-constructor-callback-or-unresolved |
| `model_capability_for` | `exact.clone` | [646](../../src/dialect.rs#L646) | receiver-type-required |
| `model_capability_for` | `dialect_uniform_capability` | [648](../../src/dialect.rs#L648) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L660) |
| `dialect_uniform_capability` | `catalog         .iter()         .filter` | [665](../../src/dialect.rs#L665) | receiver-type-required |
| `dialect_uniform_capability` | `catalog         .iter` | [665](../../src/dialect.rs#L665) | receiver-type-required |
| `dialect_uniform_capability` | `family.next` | [668](../../src/dialect.rs#L668) | receiver-type-required |
| `dialect_uniform_capability` | `family.any` | [669](../../src/dialect.rs#L669) | receiver-type-required |
| `dialect_uniform_capability` | `Some` | [675](../../src/dialect.rs#L675) | external-constructor-callback-or-unresolved |
| `dialect_uniform_capability` | `dialect_id.to_owned` | [676](../../src/dialect.rs#L676) | receiver-type-required |
| `dialect_uniform_capability` | `exact_sku.to_owned` | [678](../../src/dialect.rs#L678) | receiver-type-required |
| `dialect_uniform_capability` | `first.reasoning.clone` | [681](../../src/dialect.rs#L681) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map(&#124;definition&#124; SupportedDialect {             dialect_id: definition.dialect.as_str().to_owned(),             protocol_family: definition.family.to_owned(),         })         .collect` | [693](../../src/dialect.rs#L693) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter()         .map` | [693](../../src/dialect.rs#L693) | receiver-type-required |
| `supported_dialects` | `DEFINITIONS         .iter` | [693](../../src/dialect.rs#L693) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str().to_owned` | [696](../../src/dialect.rs#L696) | receiver-type-required |
| `supported_dialects` | `definition.dialect.as_str` | [696](../../src/dialect.rs#L696) | receiver-type-required |
| `supported_dialects` | `definition.family.to_owned` | [697](../../src/dialect.rs#L697) | receiver-type-required |
| `validate_provider` | `DialectId::from_str` | [706](../../src/dialect.rs#L706) | external-constructor-callback-or-unresolved |
| `validate_provider` | `definition_for` | [707](../../src/dialect.rs#L707) | [provider::dialect::definition_for](../../src/dialect.rs#L522) |
| `validate_provider` | `Err` | [709](../../src/dialect.rs#L709) | external-constructor-callback-or-unresolved |
| `validate_provider` | `Ok` | [711](../../src/dialect.rs#L711) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `DialectId::from_str` | [718](../../src/dialect.rs#L718) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `definition_for` | [719](../../src/dialect.rs#L719) | [provider::dialect::definition_for](../../src/dialect.rs#L522) |
| `resolve_profile` | `Err` | [721](../../src/dialect.rs#L721) | external-constructor-callback-or-unresolved |
| `resolve_profile` | `provider.adapter.clone` | [724](../../src/dialect.rs#L724) | receiver-type-required |
| `resolve_profile` | `provider.dialect.clone` | [725](../../src/dialect.rs#L725) | receiver-type-required |
| `resolve_profile` | `model.profile.clone` | [726](../../src/dialect.rs#L726) | receiver-type-required |
| `resolve_profile` | `provider.endpoint_owner.clone` | [728](../../src/dialect.rs#L728) | receiver-type-required |
| `resolve_profile` | `provider.gateway_translation.clone` | [729](../../src/dialect.rs#L729) | receiver-type-required |
| `resolve_profile` | `model.id.clone` | [730](../../src/dialect.rs#L730) | receiver-type-required |
| `resolve_profile` | `provider.evidence_revision.clone` | [731](../../src/dialect.rs#L731) | receiver-type-required |
| `resolve_profile` | `resolve_configured_target` | [734](../../src/dialect.rs#L734) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L746) |
| `validate_target` | `target.dialect` | [738](../../src/dialect.rs#L738) | receiver-type-required |
| `validate_target` | `definition_for` | [739](../../src/dialect.rs#L739) | [provider::dialect::definition_for](../../src/dialect.rs#L522) |
| `validate_target` | `target_matches_definition` | [740](../../src/dialect.rs#L740) | [provider::dialect::target_matches_definition](../../src/dialect.rs#L529) |
| `validate_target` | `Err` | [741](../../src/dialect.rs#L741) | external-constructor-callback-or-unresolved |
| `validate_target` | `resolve_configured_target` | [743](../../src/dialect.rs#L743) | [provider::dialect::resolve_configured_target](../../src/dialect.rs#L746) |
| `validate_target` | `target.clone` | [743](../../src/dialect.rs#L743) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_header.to_owned` | [751](../../src/dialect.rs#L751) | receiver-type-required |
| `resolve_configured_target` | `definition.credential_prefix.to_owned` | [752](../../src/dialect.rs#L752) | receiver-type-required |
| `resolve_configured_target` | `"cf-aig-authorization".to_owned` | [765](../../src/dialect.rs#L765) | receiver-type-required |
| `resolve_configured_target` | `"Bearer ".to_owned` | [766](../../src/dialect.rs#L766) | receiver-type-required |
| `resolve_configured_target` | `model_capability_for` | [768](../../src/dialect.rs#L768) | [provider::dialect::model_capability_for](../../src/dialect.rs#L634) |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter(&#124;native&#124; {             native.mode.dialect() == dialect                 && native.routes.iter().any(&#124;route&#124; {                     route.endpoint_owner == target.route.endpoint_owner                         && route.gateway_translation == target.route.gateway_translation                 })         })         .map` | [769](../../src/dialect.rs#L769) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then(&#124;value&#124; value.native_deferred_tools.as_ref())         .filter` | [769](../../src/dialect.rs#L769) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref()         .and_then` | [769](../../src/dialect.rs#L769) | receiver-type-required |
| `resolve_configured_target` | `capability         .as_ref` | [769](../../src/dialect.rs#L769) | receiver-type-required |
| `resolve_configured_target` | `value.native_deferred_tools.as_ref` | [771](../../src/dialect.rs#L771) | receiver-type-required |
| `resolve_configured_target` | `native.mode.dialect` | [773](../../src/dialect.rs#L773) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter().any` | [774](../../src/dialect.rs#L774) | receiver-type-required |
| `resolve_configured_target` | `native.routes.iter` | [774](../../src/dialect.rs#L774) | receiver-type-required |
| `resolve_configured_target` | `Ok` | [780](../../src/dialect.rs#L780) | external-constructor-callback-or-unresolved |
| `resolve_configured_target` | `capability             .as_ref()             .map(&#124;value&#124; value.reasoning.levels.clone())             .unwrap_or_default` | [786](../../src/dialect.rs#L786) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .map` | [786](../../src/dialect.rs#L786) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref` | [786](../../src/dialect.rs#L786), [790](../../src/dialect.rs#L790), [793](../../src/dialect.rs#L793), [796](../../src/dialect.rs#L796), [799](../../src/dialect.rs#L799), [802](../../src/dialect.rs#L802), [805](../../src/dialect.rs#L805) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.levels.clone` | [788](../../src/dialect.rs#L788) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .and_then` | [790](../../src/dialect.rs#L790), [793](../../src/dialect.rs#L793), [802](../../src/dialect.rs#L802), [805](../../src/dialect.rs#L805) | receiver-type-required |
| `resolve_configured_target` | `value.reasoning.default.clone` | [792](../../src/dialect.rs#L792) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_none_or` | [796](../../src/dialect.rs#L796) | receiver-type-required |
| `resolve_configured_target` | `capability             .as_ref()             .is_some_and` | [799](../../src/dialect.rs#L799) | receiver-type-required |
| `resolve_configured_target` | `value.wire_model.clone` | [807](../../src/dialect.rs#L807) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref().is_some_and` | [808](../../src/dialect.rs#L808) | receiver-type-required |
| `resolve_configured_target` | `capability.as_ref` | [808](../../src/dialect.rs#L808) | receiver-type-required |
| `epoch_profile` | `settings.and_then` | [818](../../src/dialect.rs#L818) | receiver-type-required |
| `epoch_profile` | `settings.reasoning_effort.as_deref` | [818](../../src/dialect.rs#L818) | receiver-type-required |
| `epoch_profile` | `requested.or_else` | [819](../../src/dialect.rs#L819) | receiver-type-required |
| `epoch_profile` | `profile.default_reasoning_effort` | [819](../../src/dialect.rs#L819) | receiver-type-required |
| `epoch_profile` | `effort.is_some_and` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter().any` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts().iter` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `epoch_profile` | `profile.reasoning_efforts` | [820](../../src/dialect.rs#L820) | receiver-type-required |
| `epoch_profile` | `Err` | [821](../../src/dialect.rs#L821) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::UnsupportedControl` | [821](../../src/dialect.rs#L821) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `"reasoning_effort".to_owned` | [822](../../src/dialect.rs#L822), [828](../../src/dialect.rs#L828) | receiver-type-required |
| `epoch_profile` | `serde_json::Map::new` | [825](../../src/dialect.rs#L825) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `controls.insert` | [827](../../src/dialect.rs#L827) | receiver-type-required |
| `epoch_profile` | `Value::String` | [829](../../src/dialect.rs#L829) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `value.to_owned` | [829](../../src/dialect.rs#L829) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse(         &serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err(&#124;error&#124; DialectError::InvalidEpoch(error.to_string()))?,     )     .map_err` | [832](../../src/dialect.rs#L832) | receiver-type-required |
| `epoch_profile` | `IJsonValue::parse` | [832](../../src/dialect.rs#L832) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `epoch_profile` | `serde_json_canonicalizer::to_vec(&json!({             "controls": controls,             "serializer_revision": profile.serializer_revision,             "system": system,             "target": profile.target,         }))         .map_err` | [833](../../src/dialect.rs#L833) | receiver-type-required |
| `epoch_profile` | `serde_json_canonicalizer::to_vec` | [833](../../src/dialect.rs#L833) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `DialectError::InvalidEpoch` | [839](../../src/dialect.rs#L839), [841](../../src/dialect.rs#L841) | external-constructor-callback-or-unresolved |
| `epoch_profile` | `error.to_string` | [839](../../src/dialect.rs#L839), [841](../../src/dialect.rs#L841) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(epoch)         .map_err` | [848](../../src/dialect.rs#L848) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value` | [848](../../src/dialect.rs#L848), [853](../../src/dialect.rs#L853) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `DialectError::InvalidEpoch` | [849](../../src/dialect.rs#L849), [854](../../src/dialect.rs#L854) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `error.to_string` | [849](../../src/dialect.rs#L849), [854](../../src/dialect.rs#L854) | receiver-type-required |
| `validate_epoch_target` | `value         .get("target")         .ok_or` | [850](../../src/dialect.rs#L850) | receiver-type-required |
| `validate_epoch_target` | `value         .get` | [850](../../src/dialect.rs#L850) | receiver-type-required |
| `validate_epoch_target` | `serde_json::to_value(&profile.target)         .map_err` | [853](../../src/dialect.rs#L853) | receiver-type-required |
| `validate_epoch_target` | `Err` | [856](../../src/dialect.rs#L856), [860](../../src/dialect.rs#L860) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `value.get("serializer_revision").and_then` | [858](../../src/dialect.rs#L858) | receiver-type-required |
| `validate_epoch_target` | `value.get` | [858](../../src/dialect.rs#L858) | receiver-type-required |
| `validate_epoch_target` | `Some` | [858](../../src/dialect.rs#L858) | external-constructor-callback-or-unresolved |
| `validate_epoch_target` | `Ok` | [862](../../src/dialect.rs#L862) | external-constructor-callback-or-unresolved |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref()             .expect` | [869](../../src/dialect.rs#L869) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init(super::load_model_capabilities)             .as_ref` | [869](../../src/dialect.rs#L869) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::MODEL_CAPABILITIES             .get_or_init` | [869](../../src/dialect.rs#L869) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_chat_v1", "deepseek-flash")                 .expect` | [874](../../src/dialect.rs#L874) | receiver-type-required |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability` | [874](../../src/dialect.rs#L874), [885](../../src/dialect.rs#L885) | [provider::dialect::dialect_uniform_capability](../../src/dialect.rs#L660) |
| `unlisted_sku_inherits_a_dialect_uniform_reasoning_capability` | `super::dialect_uniform_capability(catalog, "deepseek_responses_v1", "unlisted")                 .expect` | [885](../../src/dialect.rs#L885) | receiver-type-required |
