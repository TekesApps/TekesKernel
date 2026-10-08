# tekes-supervisor::resource_capability

[Package atlas](index.md) · [Source](../../src/resource_capability.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::resource_capability::SKILLS_LIST](../../src/resource_capability.rs#L26) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::COMMANDS_LIST](../../src/resource_capability.rs#L27) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::COMMANDS_RUN](../../src/resource_capability.rs#L28) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::RESOURCE_CAPABILITY_FORMAT](../../src/resource_capability.rs#L29) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::RESOURCE_CAPABILITIES](../../src/resource_capability.rs#L30) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::SkillsListResult](../../src/resource_capability.rs#L34) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::CommandsListResult](../../src/resource_capability.rs#L41) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::CommandRunRequest](../../src/resource_capability.rs#L48) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::CommandRunResult](../../src/resource_capability.rs#L62) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::KeyedCommandInput](../../src/resource_capability.rs#L74) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::COMPACT_COMMAND_VERB](../../src/resource_capability.rs#L85) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::PERMISSION_COMMAND_VERB](../../src/resource_capability.rs#L90) | const_item | `pub` |  |
| [tekes-supervisor::resource_capability::permission_verb_mode](../../src/resource_capability.rs#L95) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::CommandInputAuthority](../../src/resource_capability.rs#L102) | trait_item | `pub` |  |
| [tekes-supervisor::resource_capability::CommandInputAuthority::submit](../../src/resource_capability.rs#L103) | function_signature_item | `private` |  |
| [tekes-supervisor::resource_capability::CommandInputAuthority::submit_with_attachments](../../src/resource_capability.rs#L110) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::CommandInputAuthority::compact](../../src/resource_capability.rs#L127) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::CommandInputAuthority::select_permission](../../src/resource_capability.rs#L138) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ClientResourceService](../../src/resource_capability.rs#L148) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::new](../../src/resource_capability.rs#L158) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::capabilities](../../src/resource_capability.rs#L163) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::skills_list](../../src/resource_capability.rs#L171) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::commands_list](../../src/resource_capability.rs#L179) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::commands_run](../../src/resource_capability.rs#L186) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::commands_run_with_catalog](../../src/resource_capability.rs#L194) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::capabilities](../../src/resource_capability.rs#L283) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::extension_method_class](../../src/resource_capability.rs#L287) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::validate_extension_payload](../../src/resource_capability.rs#L295) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::extension_failure_is_exact](../../src/resource_capability.rs#L314) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::execute](../../src/resource_capability.rs#L327) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ResourceClock](../../src/resource_capability.rs#L359) | type_item | `pub` |  |
| [tekes-supervisor::resource_capability::ClientResourceService::validate_session](../../src/resource_capability.rs#L362) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority](../../src/resource_capability.rs#L373) | struct_item | `pub` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::new](../../src/resource_capability.rs#L384) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::with_attachment_authority](../../src/resource_capability.rs#L400) | function_item | `pub` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::deliver](../../src/resource_capability.rs#L405) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::map_prompt_materialize_error](../../src/resource_capability.rs#L465) | function_item | `pub(crate)` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::submit](../../src/resource_capability.rs#L491) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::submit_with_attachments](../../src/resource_capability.rs#L498) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::compact](../../src/resource_capability.rs#L506) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::select_permission](../../src/resource_capability.rs#L529) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::ResourceCapabilityError](../../src/resource_capability.rs#L560) | enum_item | `pub` |  |
| [tekes-supervisor::resource_capability::validate_run_request](../../src/resource_capability.rs#L571) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::decode_payload](../../src/resource_capability.rs#L589) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::encode_result](../../src/resource_capability.rs#L593) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::map_resource_failure](../../src/resource_capability.rs#L598) | function_item | `pub(crate)` |  |
| [tekes-supervisor::resource_capability::resource_failure_is_exact](../../src/resource_capability.rs#L628) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::base_failure_is_exact](../../src/resource_capability.rs#L651) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::json_details](../../src/resource_capability.rs#L667) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::bad_request](../../src/resource_capability.rs#L672) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) | function_item | `private` |  |
| [tekes-supervisor::resource_capability::tests::RecordingInput](../../src/resource_capability.rs#L702) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingInput::submit](../../src/resource_capability.rs#L709) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingInput::compact](../../src/resource_capability.rs#L720) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingInput::select_permission](../../src/resource_capability.rs#L731) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::source](../../src/resource_capability.rs#L769) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::session_catalog_controls_command_expansion](../../src/resource_capability.rs#L780) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::capabilities_are_separate_and_run_submits_one_keyed_input](../../src/resource_capability.rs#L800) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input](../../src/resource_capability.rs#L834) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::permission_verb_in_an_expanded_catalog_body_is_the_same_control_request](../../src/resource_capability.rs#L880) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::production_permission_selection_persists_the_mode_under_the_session_folder](../../src/resource_capability.rs#L929) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingDelivery](../../src/resource_capability.rs#L999) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingDelivery::session_metadata_changed](../../src/resource_capability.rs#L1005) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingDelivery::prompt](../../src/resource_capability.rs#L1009) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingDelivery::cancel](../../src/resource_capability.rs#L1025) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::RecordingDelivery::rename](../../src/resource_capability.rs#L1034) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::command_attachments_follow_the_expanded_text_in_request_order](../../src/resource_capability.rs#L1050) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::resource_capability::tests::compact_verb_maps_to_the_manual_compaction_request_not_an_input](../../src/resource_capability.rs#L1158) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `AttachmentAuthority` | `endpoint::AttachmentAuthority` | `private` |
| `DurableHandoffProof` | `endpoint::DurableHandoffProof` | `private` |
| `EndpointHostCall` | `endpoint::EndpointHostCall` | `private` |
| `MaterializedPrompt` | `endpoint::MaterializedPrompt` | `private` |
| `MethodClass` | `endpoint::MethodClass` | `private` |
| `MutationReceipt` | `endpoint::MutationReceipt` | `private` |
| `PromptMaterializeError` | `endpoint::PromptMaterializeError` | `private` |
| `PromptPart` | `endpoint::PromptPart` | `private` |
| `RpcDurableIdentity` | `endpoint::RpcDurableIdentity` | `private` |
| `CommandSummary` | `profile::CommandSummary` | `private` |
| `ResourceCatalog` | `profile::ResourceCatalog` | `private` |
| `ResourceError` | `profile::ResourceError` | `private` |
| `SkillSummary` | `profile::SkillSummary` | `private` |
| `Block` | `schema::Block` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `DeserializeOwned` | `serde::de::DeserializeOwned` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `_` | `sha2::Digest` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `ProductionEndpointRoutes` | `crate::endpoint_host::ProductionEndpointRoutes` | `private` |
| `ProductionRouteFailure` | `crate::endpoint_host::ProductionRouteFailure` | `private` |
| `SessionDeliveryAuthority` | `crate::endpoint_host::SessionDeliveryAuthority` | `private` |
| `SessionInputAdmissionAuthority` | `crate::endpoint_host::SessionInputAdmissionAuthority` | `private` |
| `production_failure_is_exact_for` | `crate::endpoint_host::production_failure_is_exact_for` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `EffectiveInstructions` | `profile::EffectiveInstructions` | `private` |
| `InstructionKind` | `profile::InstructionKind` | `private` |
| `InstructionOrigin` | `profile::InstructionOrigin` | `private` |
| `InstructionSnapshot` | `profile::InstructionSnapshot` | `private` |
| `InstructionSource` | `profile::InstructionSource` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::resource_capability::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 25 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::permission_mode::PermissionMode::parse"]
  n1["tekes-supervisor::endpoint_host::production_failure_is_exact_for"]
  n2["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n3["tekes-supervisor::resource_capability::CommandInputAuthority::submit_with_attachments"]
  n4["tekes-supervisor::resource_capability::CommandInputAuthority::compact"]
  n5["tekes-supervisor::resource_capability::CommandInputAuthority::select_permission"]
  n6["tekes-supervisor::resource_capability::ClientResourceService::new"]
  n7["tekes-supervisor::resource_capability::ClientResourceService::capabilities"]
  n8["tekes-supervisor::resource_capability::ClientResourceService::skills_list"]
  n9["tekes-supervisor::resource_capability::ClientResourceService::commands_list"]
  n10["tekes-supervisor::resource_capability::ClientResourceService::commands_run"]
  n11["tekes-supervisor::resource_capability::ClientResourceService::commands_run_with_catalog"]
  n12["tekes-supervisor::resource_capability::ClientResourceService::capabilities"]
  n13["tekes-supervisor::resource_capability::ClientResourceService::extension_method_class"]
  n14["tekes-supervisor::resource_capability::ClientResourceService::validate_extension_payload"]
  n15["tekes-supervisor::resource_capability::ClientResourceService::extension_failure_is_exact"]
  n16["tekes-supervisor::resource_capability::ClientResourceService::execute"]
  n17["tekes-supervisor::resource_capability::ClientResourceService::validate_session"]
  n18["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::new"]
  n19["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::with_attachment_authority"]
  n20["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::deliver"]
  n21["tekes-supervisor::resource_capability::map_prompt_materialize_error"]
  n22["tekes-supervisor::resource_capability::validate_run_request"]
  n23["tekes-supervisor::resource_capability::decode_payload"]
  n24["tekes-supervisor::resource_capability::encode_result"]
  n25["tekes-supervisor::resource_capability::resource_failure_is_exact"]
  n26["tekes-supervisor::resource_capability::base_failure_is_exact"]
  n27["tekes-supervisor::resource_capability::empty_details"]
  n28["tekes-supervisor::resource_capability::json_details"]
  n29["tekes-supervisor::resource_capability::bad_request"]
  n30["tekes-supervisor::resource_capability::internal_failure"]
  n31["tekes-supervisor::resource_capability::unsupported"]
  n32["tekes-supervisor::resource_capability::permission_verb_mode"]
  n3 --> n31
  n4 --> n31
  n5 --> n31
  n10 --> n11
  n11 --> n0
  n11 --> n2
  n11 --> n22
  n11 --> n27
  n11 --> n32
  n14 --> n22
  n14 --> n23
  n14 --> n29
  n15 --> n1
  n15 --> n25
  n15 --> n26
  n16 --> n23
  n16 --> n24
  n16 --> n30
  n16 --> n31
  n20 --> n21
  n20 --> n31
  n21 --> n2
  n21 --> n28
  n21 --> n29
  n21 --> n30
```

</details>

<details><summary>Functions 21–35: 20 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::permission_mode::write_permission_mode"]
  n1["schema::ijson::IJsonValue::parse"]
  n2["schema::ijson::IJsonValue::parse_str"]
  n3["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n4["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::submit"]
  n5["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::submit_with_attachments"]
  n6["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::compact"]
  n7["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::select_permission"]
  n8["tekes-supervisor::resource_capability::validate_run_request"]
  n9["tekes-supervisor::resource_capability::decode_payload"]
  n10["tekes-supervisor::resource_capability::encode_result"]
  n11["tekes-supervisor::resource_capability::map_resource_failure"]
  n12["tekes-supervisor::resource_capability::resource_failure_is_exact"]
  n13["tekes-supervisor::resource_capability::base_failure_is_exact"]
  n14["tekes-supervisor::resource_capability::empty_details"]
  n15["tekes-supervisor::resource_capability::json_details"]
  n16["tekes-supervisor::resource_capability::bad_request"]
  n17["tekes-supervisor::resource_capability::internal_failure"]
  n18["tekes-supervisor::resource_capability::unsupported"]
  n7 --> n0
  n7 --> n17
  n9 --> n16
  n10 --> n1
  n10 --> n17
  n11 --> n3
  n11 --> n14
  n11 --> n15
  n11 --> n16
  n11 --> n17
  n12 --> n14
  n13 --> n14
  n14 --> n2
  n15 --> n1
  n16 --> n3
  n16 --> n14
  n17 --> n3
  n17 --> n14
  n18 --> n1
  n18 --> n3
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `permission_verb_mode` | `text.split_whitespace` | [96](../../src/resource_capability.rs#L96) | receiver-type-required |
| `permission_verb_mode` | `tokens.next` | [97](../../src/resource_capability.rs#L97), [98](../../src/resource_capability.rs#L98), [99](../../src/resource_capability.rs#L99) | receiver-type-required |
| `permission_verb_mode` | `(verb == PERMISSION_COMMAND_VERB && tokens.next().is_none()).then_some` | [99](../../src/resource_capability.rs#L99) | receiver-type-required |
| `permission_verb_mode` | `tokens.next().is_none` | [99](../../src/resource_capability.rs#L99) | receiver-type-required |
| `submit_with_attachments` | `attachments.is_empty` | [115](../../src/resource_capability.rs#L115) | receiver-type-required |
| `submit_with_attachments` | `self.submit` | [116](../../src/resource_capability.rs#L116) | receiver-type-required |
| `submit_with_attachments` | `Err` | [118](../../src/resource_capability.rs#L118) | external-constructor-callback-or-unresolved |
| `submit_with_attachments` | `ResourceCapabilityError::Delivery` | [118](../../src/resource_capability.rs#L118) | external-constructor-callback-or-unresolved |
| `submit_with_attachments` | `unsupported` | [118](../../src/resource_capability.rs#L118) | [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) |
| `compact` | `Err` | [132](../../src/resource_capability.rs#L132) | external-constructor-callback-or-unresolved |
| `compact` | `ResourceCapabilityError::Delivery` | [132](../../src/resource_capability.rs#L132) | external-constructor-callback-or-unresolved |
| `compact` | `unsupported` | [132](../../src/resource_capability.rs#L132) | [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) |
| `select_permission` | `Err` | [144](../../src/resource_capability.rs#L144) | external-constructor-callback-or-unresolved |
| `select_permission` | `ResourceCapabilityError::Delivery` | [144](../../src/resource_capability.rs#L144) | external-constructor-callback-or-unresolved |
| `select_permission` | `unsupported` | [144](../../src/resource_capability.rs#L144) | [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) |
| `capabilities` | `RESOURCE_CAPABILITIES             .into_iter()             .map(str::to_owned)             .collect` | [164](../../src/resource_capability.rs#L164) | receiver-type-required |
| `capabilities` | `RESOURCE_CAPABILITIES             .into_iter()             .map` | [164](../../src/resource_capability.rs#L164) | receiver-type-required |
| `capabilities` | `RESOURCE_CAPABILITIES             .into_iter` | [164](../../src/resource_capability.rs#L164) | receiver-type-required |
| `skills_list` | `self.catalog.skill_summaries` | [174](../../src/resource_capability.rs#L174) | receiver-type-required |
| `commands_list` | `self.catalog.command_summaries` | [182](../../src/resource_capability.rs#L182) | receiver-type-required |
| `commands_run` | `self.commands_run_with_catalog` | [191](../../src/resource_capability.rs#L191) | [tekes-supervisor::resource_capability::ClientResourceService::commands_run_with_catalog](../../src/resource_capability.rs#L194) |
| `commands_run_with_catalog` | `validate_run_request` | [200](../../src/resource_capability.rs#L200) | [tekes-supervisor::resource_capability::validate_run_request](../../src/resource_capability.rs#L571) |
| `commands_run_with_catalog` | `catalog.command(&request.name).is_err` | [205](../../src/resource_capability.rs#L205) | receiver-type-required |
| `commands_run_with_catalog` | `catalog.command` | [205](../../src/resource_capability.rs#L205) | receiver-type-required |
| `commands_run_with_catalog` | `request.arguments.trim` | [206](../../src/resource_capability.rs#L206) | receiver-type-required |
| `commands_run_with_catalog` | `mode.is_empty` | [207](../../src/resource_capability.rs#L207) | receiver-type-required |
| `commands_run_with_catalog` | `Err` | [208](../../src/resource_capability.rs#L208), [233](../../src/resource_capability.rs#L233), [247](../../src/resource_capability.rs#L247), [252](../../src/resource_capability.rs#L252) | external-constructor-callback-or-unresolved |
| `commands_run_with_catalog` | `ResourceCapabilityError::Resource` | [208](../../src/resource_capability.rs#L208), [247](../../src/resource_capability.rs#L247) | external-constructor-callback-or-unresolved |
| `commands_run_with_catalog` | `ResourceError::InvalidArguments` | [209](../../src/resource_capability.rs#L209), [248](../../src/resource_capability.rs#L248) | external-constructor-callback-or-unresolved |
| `commands_run_with_catalog` | `"permission needs a mode".to_owned` | [209](../../src/resource_capability.rs#L209) | receiver-type-required |
| `commands_run_with_catalog` | `request.name.clone` | [214](../../src/resource_capability.rs#L214) | receiver-type-required |
| `commands_run_with_catalog` | `catalog.expand_command` | [219](../../src/resource_capability.rs#L219) | receiver-type-required |
| `commands_run_with_catalog` | `principal.to_owned` | [222](../../src/resource_capability.rs#L222) | receiver-type-required |
| `commands_run_with_catalog` | `request.session_id.clone` | [223](../../src/resource_capability.rs#L223) | receiver-type-required |
| `commands_run_with_catalog` | `request.key.clone` | [224](../../src/resource_capability.rs#L224), [268](../../src/resource_capability.rs#L268) | receiver-type-required |
| `commands_run_with_catalog` | `expanded.name.clone` | [225](../../src/resource_capability.rs#L225) | receiver-type-required |
| `commands_run_with_catalog` | `expanded.content_digest.clone` | [226](../../src/resource_capability.rs#L226) | receiver-type-required |
| `commands_run_with_catalog` | `keyed.text.trim` | [231](../../src/resource_capability.rs#L231) | receiver-type-required |
| `commands_run_with_catalog` | `request.attachments.is_empty` | [232](../../src/resource_capability.rs#L232), [251](../../src/resource_capability.rs#L251), [261](../../src/resource_capability.rs#L261) | receiver-type-required |
| `commands_run_with_catalog` | `ResourceCapabilityError::Delivery` | [233](../../src/resource_capability.rs#L233), [252](../../src/resource_capability.rs#L252) | external-constructor-callback-or-unresolved |
| `commands_run_with_catalog` | `ProductionRouteFailure::new` | [234](../../src/resource_capability.rs#L234), [253](../../src/resource_capability.rs#L253) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `commands_run_with_catalog` | `empty_details` | [237](../../src/resource_capability.rs#L237), [256](../../src/resource_capability.rs#L256) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `commands_run_with_catalog` | `self.input.compact` | [241](../../src/resource_capability.rs#L241) | receiver-type-required |
| `commands_run_with_catalog` | `permission_verb_mode` | [242](../../src/resource_capability.rs#L242) | [tekes-supervisor::resource_capability::permission_verb_mode](../../src/resource_capability.rs#L95) |
| `commands_run_with_catalog` | `engine::PermissionMode::parse` | [246](../../src/resource_capability.rs#L246) | [engine::permission_mode::PermissionMode::parse](../../../engine/src/permission_mode.rs#L51) |
| `commands_run_with_catalog` | `self.input.select_permission` | [260](../../src/resource_capability.rs#L260) | receiver-type-required |
| `commands_run_with_catalog` | `self.input.submit` | [262](../../src/resource_capability.rs#L262) | receiver-type-required |
| `commands_run_with_catalog` | `self.input                 .submit_with_attachments` | [264](../../src/resource_capability.rs#L264) | receiver-type-required |
| `commands_run_with_catalog` | `Ok` | [267](../../src/resource_capability.rs#L267) | external-constructor-callback-or-unresolved |
| `capabilities` | `Self::capabilities` | [284](../../src/resource_capability.rs#L284) | external-constructor-callback-or-unresolved |
| `extension_method_class` | `Some` | [289](../../src/resource_capability.rs#L289), [290](../../src/resource_capability.rs#L290) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `payload.as_object().is_some_and` | [302](../../src/resource_capability.rs#L302) | receiver-type-required |
| `validate_extension_payload` | `payload.as_object` | [302](../../src/resource_capability.rs#L302) | receiver-type-required |
| `validate_extension_payload` | `object.is_empty` | [302](../../src/resource_capability.rs#L302) | receiver-type-required |
| `validate_extension_payload` | `Ok` | [304](../../src/resource_capability.rs#L304) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `decode_payload::<CommandRunRequest>` | [307](../../src/resource_capability.rs#L307) | [tekes-supervisor::resource_capability::decode_payload](../../src/resource_capability.rs#L589) |
| `validate_extension_payload` | `validate_run_request(&request).map_err` | [308](../../src/resource_capability.rs#L308) | receiver-type-required |
| `validate_extension_payload` | `validate_run_request` | [308](../../src/resource_capability.rs#L308) | [tekes-supervisor::resource_capability::validate_run_request](../../src/resource_capability.rs#L571) |
| `validate_extension_payload` | `Err` | [310](../../src/resource_capability.rs#L310) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `bad_request` | [310](../../src/resource_capability.rs#L310) | [tekes-supervisor::resource_capability::bad_request](../../src/resource_capability.rs#L672) |
| `extension_failure_is_exact` | `base_failure_is_exact` | [320](../../src/resource_capability.rs#L320), [322](../../src/resource_capability.rs#L322) | [tekes-supervisor::resource_capability::base_failure_is_exact](../../src/resource_capability.rs#L651) |
| `extension_failure_is_exact` | `resource_failure_is_exact` | [323](../../src/resource_capability.rs#L323) | [tekes-supervisor::resource_capability::resource_failure_is_exact](../../src/resource_capability.rs#L628) |
| `extension_failure_is_exact` | `production_failure_is_exact_for` | [324](../../src/resource_capability.rs#L324) | [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1856) |
| `execute` | `request.operation.as_str` | [333](../../src/resource_capability.rs#L333) | receiver-type-required |
| `execute` | `encode_result` | [334](../../src/resource_capability.rs#L334), [335](../../src/resource_capability.rs#L335), [352](../../src/resource_capability.rs#L352) | [tekes-supervisor::resource_capability::encode_result](../../src/resource_capability.rs#L593) |
| `execute` | `self.skills_list` | [334](../../src/resource_capability.rs#L334) | receiver-type-required |
| `execute` | `self.commands_list` | [335](../../src/resource_capability.rs#L335) | receiver-type-required |
| `execute` | `decode_payload::<CommandRunRequest>` | [337](../../src/resource_capability.rs#L337) | [tekes-supervisor::resource_capability::decode_payload](../../src/resource_capability.rs#L589) |
| `execute` | `self                     .commands_run(&run, principal)                     .map_err` | [338](../../src/resource_capability.rs#L338) | receiver-type-required |
| `execute` | `self                     .commands_run` | [338](../../src/resource_capability.rs#L338) | receiver-type-required |
| `execute` | `request                     .handoff                     .mark_handed_off(DurableHandoffProof {                         delivery: "locked-append".to_owned(),                         durable_identity: Some(RpcDurableIdentity {                             kind: "event-origin".to_owned(),                             id: request.rpc_id.clone(),                             seq: Some(result.seq),                         }),                     })                     .map_err` | [341](../../src/resource_capability.rs#L341) | receiver-type-required |
| `execute` | `request                     .handoff                     .mark_handed_off` | [341](../../src/resource_capability.rs#L341) | receiver-type-required |
| `execute` | `"locked-append".to_owned` | [344](../../src/resource_capability.rs#L344) | receiver-type-required |
| `execute` | `Some` | [345](../../src/resource_capability.rs#L345), [348](../../src/resource_capability.rs#L348) | external-constructor-callback-or-unresolved |
| `execute` | `"event-origin".to_owned` | [346](../../src/resource_capability.rs#L346) | receiver-type-required |
| `execute` | `request.rpc_id.clone` | [347](../../src/resource_capability.rs#L347) | receiver-type-required |
| `execute` | `internal_failure` | [351](../../src/resource_capability.rs#L351) | [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) |
| `execute` | `Err` | [354](../../src/resource_capability.rs#L354) | external-constructor-callback-or-unresolved |
| `execute` | `unsupported` | [354](../../src/resource_capability.rs#L354) | [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) |
| `validate_session` | `self.input.admission.with_active_session` | [363](../../src/resource_capability.rs#L363) | receiver-type-required |
| `validate_session` | `Ok` | [366](../../src/resource_capability.rs#L366) | external-constructor-callback-or-unresolved |
| `with_attachment_authority` | `Some` | [401](../../src/resource_capability.rs#L401) | external-constructor-callback-or-unresolved |
| `deliver` | `self.admission.with_active_session` | [410](../../src/resource_capability.rs#L410) | receiver-type-required |
| `deliver` | `Vec::new` | [417](../../src/resource_capability.rs#L417), [452](../../src/resource_capability.rs#L452) | external-constructor-callback-or-unresolved |
| `deliver` | `attachments.is_empty` | [418](../../src/resource_capability.rs#L418) | receiver-type-required |
| `deliver` | `self.attachments.as_ref().ok_or_else` | [419](../../src/resource_capability.rs#L419) | receiver-type-required |
| `deliver` | `self.attachments.as_ref` | [419](../../src/resource_capability.rs#L419) | receiver-type-required |
| `deliver` | `ResourceCapabilityError::Delivery` | [420](../../src/resource_capability.rs#L420), [425](../../src/resource_capability.rs#L425) | external-constructor-callback-or-unresolved |
| `deliver` | `unsupported` | [420](../../src/resource_capability.rs#L420) | [tekes-supervisor::resource_capability::unsupported](../../src/resource_capability.rs#L680) |
| `deliver` | `authority                         .materialize_prompt_parts(&input.session_id, attachments)                         .map_err` | [422](../../src/resource_capability.rs#L422) | receiver-type-required |
| `deliver` | `authority                         .materialize_prompt_parts` | [422](../../src/resource_capability.rs#L422) | receiver-type-required |
| `deliver` | `map_prompt_materialize_error` | [425](../../src/resource_capability.rs#L425) | [tekes-supervisor::resource_capability::map_prompt_materialize_error](../../src/resource_capability.rs#L465) |
| `deliver` | `blocks.extend` | [430](../../src/resource_capability.rs#L430) | receiver-type-required |
| `deliver` | `(self.clock)().map_err` | [433](../../src/resource_capability.rs#L433) | receiver-type-required |
| `deliver` | `(self.clock)` | [433](../../src/resource_capability.rs#L433) | external-constructor-callback-or-unresolved |
| `deliver` | `input.principal.clone` | [435](../../src/resource_capability.rs#L435) | receiver-type-required |
| `deliver` | `"tekes-client-resource".to_owned` | [436](../../src/resource_capability.rs#L436) | receiver-type-required |
| `deliver` | `input.session_id.clone` | [437](../../src/resource_capability.rs#L437) | receiver-type-required |
| `deliver` | `"session.prompt".to_owned` | [441](../../src/resource_capability.rs#L441) | receiver-type-required |
| `deliver` | `input.key.clone` | [442](../../src/resource_capability.rs#L442) | receiver-type-required |
| `deliver` | `self.delivery                     .prompt(                         &input.session_id,                         &timestamp,                         &origin,                         &MaterializedPrompt {                             blocks,                             attachments: references,                             files: Vec::new(),                         },                         false,                     )                     .map_err` | [444](../../src/resource_capability.rs#L444) | receiver-type-required |
| `deliver` | `self.delivery                     .prompt` | [444](../../src/resource_capability.rs#L444) | receiver-type-required |
| `map_prompt_materialize_error` | `bad_request` | [470](../../src/resource_capability.rs#L470) | [tekes-supervisor::resource_capability::bad_request](../../src/resource_capability.rs#L672) |
| `map_prompt_materialize_error` | `ProductionRouteFailure::new` | [471](../../src/resource_capability.rs#L471), [476](../../src/resource_capability.rs#L476), [481](../../src/resource_capability.rs#L481) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `map_prompt_materialize_error` | `json_details` | [474](../../src/resource_capability.rs#L474), [479](../../src/resource_capability.rs#L479), [484](../../src/resource_capability.rs#L484) | [tekes-supervisor::resource_capability::json_details](../../src/resource_capability.rs#L667) |
| `map_prompt_materialize_error` | `internal_failure` | [486](../../src/resource_capability.rs#L486) | [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) |
| `submit` | `self.deliver` | [495](../../src/resource_capability.rs#L495) | receiver-type-required |
| `submit_with_attachments` | `self.deliver` | [503](../../src/resource_capability.rs#L503) | receiver-type-required |
| `compact` | `self.admission.with_active_session` | [510](../../src/resource_capability.rs#L510) | receiver-type-required |
| `compact` | `(self.clock)().map_err` | [514](../../src/resource_capability.rs#L514) | receiver-type-required |
| `compact` | `(self.clock)` | [514](../../src/resource_capability.rs#L514) | external-constructor-callback-or-unresolved |
| `compact` | `input.principal.clone` | [516](../../src/resource_capability.rs#L516) | receiver-type-required |
| `compact` | `"tekes-client-resource".to_owned` | [517](../../src/resource_capability.rs#L517) | receiver-type-required |
| `compact` | `input.session_id.clone` | [518](../../src/resource_capability.rs#L518) | receiver-type-required |
| `compact` | `COMMANDS_RUN.to_owned` | [519](../../src/resource_capability.rs#L519) | receiver-type-required |
| `compact` | `input.key.clone` | [520](../../src/resource_capability.rs#L520) | receiver-type-required |
| `compact` | `self.delivery                     .compact(&input.session_id, &timestamp, &origin)                     .map_err` | [522](../../src/resource_capability.rs#L522) | receiver-type-required |
| `compact` | `self.delivery                     .compact` | [522](../../src/resource_capability.rs#L522) | receiver-type-required |
| `select_permission` | `self.admission.with_active_session` | [534](../../src/resource_capability.rs#L534) | receiver-type-required |
| `select_permission` | `self                     .admission                     .root()                     .join("threads")                     .join` | [538](../../src/resource_capability.rs#L538) | receiver-type-required |
| `select_permission` | `self                     .admission                     .root()                     .join` | [538](../../src/resource_capability.rs#L538) | receiver-type-required |
| `select_permission` | `self                     .admission                     .root` | [538](../../src/resource_capability.rs#L538) | receiver-type-required |
| `select_permission` | `engine::write_permission_mode(&folder, mode)                     .map_err` | [543](../../src/resource_capability.rs#L543) | receiver-type-required |
| `select_permission` | `engine::write_permission_mode` | [543](../../src/resource_capability.rs#L543) | [engine::permission_mode::write_permission_mode](../../../engine/src/permission_mode.rs#L117) |
| `select_permission` | `ResourceCapabilityError::Delivery` | [544](../../src/resource_capability.rs#L544) | external-constructor-callback-or-unresolved |
| `select_permission` | `internal_failure` | [544](../../src/resource_capability.rs#L544) | [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) |
| `select_permission` | `self.delivery.session_metadata_changed` | [547](../../src/resource_capability.rs#L547) | receiver-type-required |
| `select_permission` | `Ok` | [550](../../src/resource_capability.rs#L550) | external-constructor-callback-or-unresolved |
| `validate_run_request` | `request.session_id.is_empty` | [572](../../src/resource_capability.rs#L572) | receiver-type-required |
| `validate_run_request` | `Err` | [573](../../src/resource_capability.rs#L573), [582](../../src/resource_capability.rs#L582) | external-constructor-callback-or-unresolved |
| `validate_run_request` | `ResourceCapabilityError::InvalidRequest` | [573](../../src/resource_capability.rs#L573), [582](../../src/resource_capability.rs#L582) | external-constructor-callback-or-unresolved |
| `validate_run_request` | `"session_id must be nonempty".to_owned` | [574](../../src/resource_capability.rs#L574) | receiver-type-required |
| `validate_run_request` | `request.key.is_empty` | [577](../../src/resource_capability.rs#L577) | receiver-type-required |
| `validate_run_request` | `request.key.len` | [578](../../src/resource_capability.rs#L578) | receiver-type-required |
| `validate_run_request` | `request.key.starts_with` | [579](../../src/resource_capability.rs#L579) | receiver-type-required |
| `validate_run_request` | `request.key.contains` | [580](../../src/resource_capability.rs#L580) | receiver-type-required |
| `validate_run_request` | `"key must be 1..128 bytes, contain no NUL, and not use request-".to_owned` | [583](../../src/resource_capability.rs#L583) | receiver-type-required |
| `validate_run_request` | `Ok` | [586](../../src/resource_capability.rs#L586) | external-constructor-callback-or-unresolved |
| `decode_payload` | `serde_json::from_value(payload.clone()).map_err` | [590](../../src/resource_capability.rs#L590) | receiver-type-required |
| `decode_payload` | `serde_json::from_value` | [590](../../src/resource_capability.rs#L590) | external-constructor-callback-or-unresolved |
| `decode_payload` | `payload.clone` | [590](../../src/resource_capability.rs#L590) | receiver-type-required |
| `decode_payload` | `bad_request` | [590](../../src/resource_capability.rs#L590) | [tekes-supervisor::resource_capability::bad_request](../../src/resource_capability.rs#L672) |
| `encode_result` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(&#124;_&#124; internal_failure())?)         .map_err` | [594](../../src/resource_capability.rs#L594) | receiver-type-required |
| `encode_result` | `IJsonValue::parse` | [594](../../src/resource_capability.rs#L594) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `encode_result` | `serde_json::to_vec(value).map_err` | [594](../../src/resource_capability.rs#L594) | receiver-type-required |
| `encode_result` | `serde_json::to_vec` | [594](../../src/resource_capability.rs#L594) | external-constructor-callback-or-unresolved |
| `encode_result` | `internal_failure` | [594](../../src/resource_capability.rs#L594), [595](../../src/resource_capability.rs#L595) | [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) |
| `map_resource_failure` | `ProductionRouteFailure::new` | [601](../../src/resource_capability.rs#L601), [604](../../src/resource_capability.rs#L604), [611](../../src/resource_capability.rs#L611), [622](../../src/resource_capability.rs#L622) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `map_resource_failure` | `empty_details` | [601](../../src/resource_capability.rs#L601), [607](../../src/resource_capability.rs#L607) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `map_resource_failure` | `json_details` | [614](../../src/resource_capability.rs#L614) | [tekes-supervisor::resource_capability::json_details](../../src/resource_capability.rs#L667) |
| `map_resource_failure` | `internal_failure` | [618](../../src/resource_capability.rs#L618) | [tekes-supervisor::resource_capability::internal_failure](../../src/resource_capability.rs#L676) |
| `map_resource_failure` | `bad_request` | [620](../../src/resource_capability.rs#L620) | [tekes-supervisor::resource_capability::bad_request](../../src/resource_capability.rs#L672) |
| `resource_failure_is_exact` | `failure.code.as_str` | [629](../../src/resource_capability.rs#L629) | receiver-type-required |
| `resource_failure_is_exact` | `empty_details` | [631](../../src/resource_capability.rs#L631), [634](../../src/resource_capability.rs#L634) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `resource_failure_is_exact` | `serde_json::from_slice::<Value>(                     &failure.details.canonical_bytes().unwrap_or_default(),                 )                 .ok()                 .and_then(&#124;value&#124; value.as_object().cloned())                 .is_some_and` | [638](../../src/resource_capability.rs#L638) | receiver-type-required |
| `resource_failure_is_exact` | `serde_json::from_slice::<Value>(                     &failure.details.canonical_bytes().unwrap_or_default(),                 )                 .ok()                 .and_then` | [638](../../src/resource_capability.rs#L638) | receiver-type-required |
| `resource_failure_is_exact` | `serde_json::from_slice::<Value>(                     &failure.details.canonical_bytes().unwrap_or_default(),                 )                 .ok` | [638](../../src/resource_capability.rs#L638) | receiver-type-required |
| `resource_failure_is_exact` | `serde_json::from_slice::<Value>` | [638](../../src/resource_capability.rs#L638) | external-constructor-callback-or-unresolved |
| `resource_failure_is_exact` | `failure.details.canonical_bytes().unwrap_or_default` | [639](../../src/resource_capability.rs#L639) | receiver-type-required |
| `resource_failure_is_exact` | `failure.details.canonical_bytes` | [639](../../src/resource_capability.rs#L639) | receiver-type-required |
| `resource_failure_is_exact` | `value.as_object().cloned` | [642](../../src/resource_capability.rs#L642) | receiver-type-required |
| `resource_failure_is_exact` | `value.as_object` | [642](../../src/resource_capability.rs#L642) | receiver-type-required |
| `resource_failure_is_exact` | `details.len` | [644](../../src/resource_capability.rs#L644) | receiver-type-required |
| `resource_failure_is_exact` | `details.get("name").is_some_and` | [644](../../src/resource_capability.rs#L644) | receiver-type-required |
| `resource_failure_is_exact` | `details.get` | [644](../../src/resource_capability.rs#L644) | receiver-type-required |
| `base_failure_is_exact` | `failure.code.as_str` | [652](../../src/resource_capability.rs#L652) | receiver-type-required |
| `base_failure_is_exact` | `empty_details` | [654](../../src/resource_capability.rs#L654), [657](../../src/resource_capability.rs#L657) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `empty_details` | `IJsonValue::parse_str("{}").expect` | [664](../../src/resource_capability.rs#L664) | receiver-type-required |
| `empty_details` | `IJsonValue::parse_str` | [664](../../src/resource_capability.rs#L664) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `json_details` | `IJsonValue::parse(&serde_json::to_vec(value).expect("JSON details"))         .expect` | [668](../../src/resource_capability.rs#L668) | receiver-type-required |
| `json_details` | `IJsonValue::parse` | [668](../../src/resource_capability.rs#L668) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `json_details` | `serde_json::to_vec(value).expect` | [668](../../src/resource_capability.rs#L668) | receiver-type-required |
| `json_details` | `serde_json::to_vec` | [668](../../src/resource_capability.rs#L668) | external-constructor-callback-or-unresolved |
| `bad_request` | `ProductionRouteFailure::new` | [673](../../src/resource_capability.rs#L673) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `bad_request` | `empty_details` | [673](../../src/resource_capability.rs#L673) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `internal_failure` | `ProductionRouteFailure::new` | [677](../../src/resource_capability.rs#L677) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `internal_failure` | `empty_details` | [677](../../src/resource_capability.rs#L677) | [tekes-supervisor::resource_capability::empty_details](../../src/resource_capability.rs#L663) |
| `unsupported` | `ProductionRouteFailure::new` | [681](../../src/resource_capability.rs#L681) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L491) |
| `unsupported` | `IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))             .expect` | [684](../../src/resource_capability.rs#L684) | receiver-type-required |
| `unsupported` | `IJsonValue::parse` | [684](../../src/resource_capability.rs#L684) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `unsupported` | `serde_json::to_vec(&json!({"operation":operation})).expect` | [684](../../src/resource_capability.rs#L684) | receiver-type-required |
| `unsupported` | `serde_json::to_vec` | [684](../../src/resource_capability.rs#L684) | external-constructor-callback-or-unresolved |
| `submit` | `self.calls.lock().expect("calls").push` | [713](../../src/resource_capability.rs#L713) | receiver-type-required |
| `submit` | `self.calls.lock().expect` | [713](../../src/resource_capability.rs#L713) | receiver-type-required |
| `submit` | `self.calls.lock` | [713](../../src/resource_capability.rs#L713) | receiver-type-required |
| `submit` | `input.clone` | [713](../../src/resource_capability.rs#L713) | receiver-type-required |
| `submit` | `Ok` | [714](../../src/resource_capability.rs#L714) | external-constructor-callback-or-unresolved |
| `compact` | `self.compacts.lock().expect("compacts").push` | [724](../../src/resource_capability.rs#L724) | receiver-type-required |
| `compact` | `self.compacts.lock().expect` | [724](../../src/resource_capability.rs#L724) | receiver-type-required |
| `compact` | `self.compacts.lock` | [724](../../src/resource_capability.rs#L724) | receiver-type-required |
| `compact` | `input.clone` | [724](../../src/resource_capability.rs#L724) | receiver-type-required |
| `compact` | `Ok` | [725](../../src/resource_capability.rs#L725) | external-constructor-callback-or-unresolved |
| `select_permission` | `self.permissions                 .lock()                 .expect("permissions")                 .push` | [736](../../src/resource_capability.rs#L736) | receiver-type-required |
| `select_permission` | `self.permissions                 .lock()                 .expect` | [736](../../src/resource_capability.rs#L736) | receiver-type-required |
| `select_permission` | `self.permissions                 .lock` | [736](../../src/resource_capability.rs#L736) | receiver-type-required |
| `select_permission` | `input.clone` | [739](../../src/resource_capability.rs#L739) | receiver-type-required |
| `select_permission` | `Ok` | [740](../../src/resource_capability.rs#L740) | external-constructor-callback-or-unresolved |
| `catalog` | `EffectiveInstructions::default` | [759](../../src/resource_capability.rs#L759) | external-constructor-callback-or-unresolved |
| `catalog` | `effective.commands.insert` | [760](../../src/resource_capability.rs#L760) | receiver-type-required |
| `catalog` | `"review.md".to_owned` | [760](../../src/resource_capability.rs#L760) | receiver-type-required |
| `catalog` | `ResourceCatalog::from_snapshot(&InstructionSnapshot {             format: 1,             sources,             effective,         })         .expect` | [761](../../src/resource_capability.rs#L761) | receiver-type-required |
| `catalog` | `ResourceCatalog::from_snapshot` | [761](../../src/resource_capability.rs#L761) | external-constructor-callback-or-unresolved |
| `source` | `path.to_owned` | [772](../../src/resource_capability.rs#L772) | receiver-type-required |
| `source` | `content.to_owned` | [774](../../src/resource_capability.rs#L774) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `ClientResourceService::new` | [782](../../src/resource_capability.rs#L782) | external-constructor-callback-or-unresolved |
| `session_catalog_controls_command_expansion` | `ResourceCatalog::default` | [782](../../src/resource_capability.rs#L782) | external-constructor-callback-or-unresolved |
| `session_catalog_controls_command_expansion` | `RecordingInput::default` | [782](../../src/resource_capability.rs#L782) | external-constructor-callback-or-unresolved |
| `session_catalog_controls_command_expansion` | `"018f0000-0000-7000-8000-000000000011".to_owned` | [784](../../src/resource_capability.rs#L784) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `"review".to_owned` | [785](../../src/resource_capability.rs#L785) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `"src/main.rs".to_owned` | [786](../../src/resource_capability.rs#L786) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `"session-command".to_owned` | [787](../../src/resource_capability.rs#L787) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `Vec::new` | [788](../../src/resource_capability.rs#L788) | external-constructor-callback-or-unresolved |
| `session_catalog_controls_command_expansion` | `service             .commands_run_with_catalog(&catalog(), &request, "uid:501")             .expect` | [791](../../src/resource_capability.rs#L791) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `service             .commands_run_with_catalog` | [791](../../src/resource_capability.rs#L791) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `catalog` | [792](../../src/resource_capability.rs#L792) | [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) |
| `session_catalog_controls_command_expansion` | `service.input.calls.lock().expect` | [794](../../src/resource_capability.rs#L794) | receiver-type-required |
| `session_catalog_controls_command_expansion` | `service.input.calls.lock` | [794](../../src/resource_capability.rs#L794) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `ClientResourceService::new` | [801](../../src/resource_capability.rs#L801) | external-constructor-callback-or-unresolved |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `catalog` | [801](../../src/resource_capability.rs#L801) | [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `RecordingInput::default` | [801](../../src/resource_capability.rs#L801) | external-constructor-callback-or-unresolved |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `service             .commands_run(                 &CommandRunRequest {                     session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),                     name: "review".to_owned(),                     arguments: "'src/lib.rs'".to_owned(),                     key: "command-1".to_owned(),                     attachments: Vec::new(),                 },                 "uid:501",             )             .expect` | [811](../../src/resource_capability.rs#L811) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `service             .commands_run` | [811](../../src/resource_capability.rs#L811) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `"018f0000-0000-7000-8000-000000000011".to_owned` | [814](../../src/resource_capability.rs#L814) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `"review".to_owned` | [815](../../src/resource_capability.rs#L815) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `"'src/lib.rs'".to_owned` | [816](../../src/resource_capability.rs#L816) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `"command-1".to_owned` | [817](../../src/resource_capability.rs#L817) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `Vec::new` | [818](../../src/resource_capability.rs#L818) | external-constructor-callback-or-unresolved |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `service.input.calls.lock().expect` | [824](../../src/resource_capability.rs#L824) | receiver-type-required |
| `capabilities_are_separate_and_run_submits_one_keyed_input` | `service.input.calls.lock` | [824](../../src/resource_capability.rs#L824) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `ClientResourceService::new` | [835](../../src/resource_capability.rs#L835) | external-constructor-callback-or-unresolved |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `catalog` | [835](../../src/resource_capability.rs#L835) | [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `RecordingInput::default` | [835](../../src/resource_capability.rs#L835) | external-constructor-callback-or-unresolved |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `service.commands_run` | [844](../../src/resource_capability.rs#L844) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `"018f0000-0000-7000-8000-000000000011".to_owned` | [846](../../src/resource_capability.rs#L846) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `name.to_owned` | [847](../../src/resource_capability.rs#L847) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `arguments.to_owned` | [848](../../src/resource_capability.rs#L848) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `key.to_owned` | [849](../../src/resource_capability.rs#L849) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `Vec::new` | [850](../../src/resource_capability.rs#L850) | external-constructor-callback-or-unresolved |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `run("permission", "read-only", "perm-1").expect` | [855](../../src/resource_capability.rs#L855) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `run` | [855](../../src/resource_capability.rs#L855), [860](../../src/resource_capability.rs#L860), [862](../../src/resource_capability.rs#L862) | external-constructor-callback-or-unresolved |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `run("permission", "yolo", "perm-2").expect_err` | [860](../../src/resource_capability.rs#L860) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `run("permission", "", "perm-3").expect_err` | [862](../../src/resource_capability.rs#L862) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `service.input.permissions.lock().expect` | [864](../../src/resource_capability.rs#L864) | receiver-type-required |
| `permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input` | `service.input.permissions.lock` | [864](../../src/resource_capability.rs#L864) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `EffectiveInstructions::default` | [888](../../src/resource_capability.rs#L888) | external-constructor-callback-or-unresolved |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `effective.commands.insert` | [889](../../src/resource_capability.rs#L889), [890](../../src/resource_capability.rs#L890) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `"lock.md".to_owned` | [889](../../src/resource_capability.rs#L889) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `"lock-talk.md".to_owned` | [890](../../src/resource_capability.rs#L890) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `ClientResourceService::new` | [891](../../src/resource_capability.rs#L891) | external-constructor-callback-or-unresolved |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `ResourceCatalog::from_snapshot(&InstructionSnapshot {                 format: 1,                 sources,                 effective,             })             .expect` | [892](../../src/resource_capability.rs#L892) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `ResourceCatalog::from_snapshot` | [892](../../src/resource_capability.rs#L892) | external-constructor-callback-or-unresolved |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `RecordingInput::default` | [898](../../src/resource_capability.rs#L898) | external-constructor-callback-or-unresolved |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service                 .commands_run(                     &CommandRunRequest {                         session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),                         name: name.to_owned(),                         arguments: arguments.to_owned(),                         key: key.to_owned(),                         attachments: Vec::new(),                     },                     "uid:501",                 )                 .expect` | [901](../../src/resource_capability.rs#L901) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service                 .commands_run` | [901](../../src/resource_capability.rs#L901) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `"018f0000-0000-7000-8000-000000000011".to_owned` | [904](../../src/resource_capability.rs#L904) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `name.to_owned` | [905](../../src/resource_capability.rs#L905) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `arguments.to_owned` | [906](../../src/resource_capability.rs#L906) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `key.to_owned` | [907](../../src/resource_capability.rs#L907) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `Vec::new` | [908](../../src/resource_capability.rs#L908) | external-constructor-callback-or-unresolved |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service.input.permissions.lock().expect` | [916](../../src/resource_capability.rs#L916) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service.input.permissions.lock` | [916](../../src/resource_capability.rs#L916) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service.input.calls.lock().expect` | [919](../../src/resource_capability.rs#L919) | receiver-type-required |
| `permission_verb_in_an_expanded_catalog_body_is_the_same_control_request` | `service.input.calls.lock` | [919](../../src/resource_capability.rs#L919) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `tempfile::tempdir().expect` | [930](../../src/resource_capability.rs#L930) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `tempfile::tempdir` | [930](../../src/resource_capability.rs#L930) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `std::fs::create_dir_all(root.path().join("threads").join(session)).expect` | [933](../../src/resource_capability.rs#L933) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `std::fs::create_dir_all` | [933](../../src/resource_capability.rs#L933), [934](../../src/resource_capability.rs#L934) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path().join("threads").join` | [933](../../src/resource_capability.rs#L933) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path().join` | [933](../../src/resource_capability.rs#L933), [934](../../src/resource_capability.rs#L934) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path` | [933](../../src/resource_capability.rs#L933), [934](../../src/resource_capability.rs#L934), [940](../../src/resource_capability.rs#L940), [964](../../src/resource_capability.rs#L964) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `std::fs::create_dir_all(root.path().join("archive").join(archived)).expect` | [934](../../src/resource_capability.rs#L934) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path().join("archive").join` | [934](../../src/resource_capability.rs#L934) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `Arc::new` | [935](../../src/resource_capability.rs#L935), [938](../../src/resource_capability.rs#L938), [939](../../src/resource_capability.rs#L939) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `RecordingDelivery` | [935](../../src/resource_capability.rs#L935) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `Mutex::new` | [935](../../src/resource_capability.rs#L935) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `Vec::new` | [935](../../src/resource_capability.rs#L935), [951](../../src/resource_capability.rs#L951) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `EndpointCommandInputAuthority::new` | [936](../../src/resource_capability.rs#L936) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `Arc::clone` | [937](../../src/resource_capability.rs#L937) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `Ok` | [938](../../src/resource_capability.rs#L938) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `"2026-08-29T00:00:00.000Z".to_owned` | [938](../../src/resource_capability.rs#L938) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `SessionInputAdmissionAuthority::new` | [939](../../src/resource_capability.rs#L939) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path().to_path_buf` | [940](../../src/resource_capability.rs#L940) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `ClientResourceService::new` | [943](../../src/resource_capability.rs#L943) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `catalog` | [943](../../src/resource_capability.rs#L943) | [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `service.commands_run` | [945](../../src/resource_capability.rs#L945) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `session_id.to_owned` | [947](../../src/resource_capability.rs#L947) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `"permission".to_owned` | [948](../../src/resource_capability.rs#L948) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `mode.to_owned` | [949](../../src/resource_capability.rs#L949) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `run(session, "read-only").expect` | [961](../../src/resource_capability.rs#L961) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `run` | [961](../../src/resource_capability.rs#L961), [975](../../src/resource_capability.rs#L975), [986](../../src/resource_capability.rs#L986) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `std::fs::read(             root.path()                 .join("threads")                 .join(session)                 .join("permission-mode.json"),         )         .expect` | [963](../../src/resource_capability.rs#L963) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `std::fs::read` | [963](../../src/resource_capability.rs#L963) | external-constructor-callback-or-unresolved |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path()                 .join("threads")                 .join(session)                 .join` | [964](../../src/resource_capability.rs#L964) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path()                 .join("threads")                 .join` | [964](../../src/resource_capability.rs#L964) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `root.path()                 .join` | [964](../../src/resource_capability.rs#L964) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `run(archived, "read-only").expect_err` | [975](../../src/resource_capability.rs#L975) | receiver-type-required |
| `production_permission_selection_persists_the_mode_under_the_session_folder` | `run("018f0000-0000-7000-8000-000000000074", "read-only").expect_err` | [986](../../src/resource_capability.rs#L986) | receiver-type-required |
| `session_metadata_changed` | `self.1.lock().expect("metadata").push` | [1006](../../src/resource_capability.rs#L1006) | receiver-type-required |
| `session_metadata_changed` | `self.1.lock().expect` | [1006](../../src/resource_capability.rs#L1006) | receiver-type-required |
| `session_metadata_changed` | `self.1.lock` | [1006](../../src/resource_capability.rs#L1006) | receiver-type-required |
| `session_metadata_changed` | `session_id.to_owned` | [1006](../../src/resource_capability.rs#L1006) | receiver-type-required |
| `prompt` | `self.0.lock().expect` | [1017](../../src/resource_capability.rs#L1017) | receiver-type-required |
| `prompt` | `self.0.lock` | [1017](../../src/resource_capability.rs#L1017) | receiver-type-required |
| `prompt` | `Some` | [1018](../../src/resource_capability.rs#L1018) | external-constructor-callback-or-unresolved |
| `prompt` | `session_id.to_owned` | [1018](../../src/resource_capability.rs#L1018) | receiver-type-required |
| `prompt` | `origin.clone` | [1018](../../src/resource_capability.rs#L1018) | receiver-type-required |
| `prompt` | `prompt.clone` | [1018](../../src/resource_capability.rs#L1018) | receiver-type-required |
| `prompt` | `Ok` | [1019](../../src/resource_capability.rs#L1019) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `tempfile::tempdir().expect` | [1051](../../src/resource_capability.rs#L1051) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `tempfile::tempdir` | [1051](../../src/resource_capability.rs#L1051) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `root.path().join("threads").join` | [1053](../../src/resource_capability.rs#L1053) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `root.path().join` | [1053](../../src/resource_capability.rs#L1053) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `root.path` | [1053](../../src/resource_capability.rs#L1053), [1061](../../src/resource_capability.rs#L1061), [1070](../../src/resource_capability.rs#L1070) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `std::fs::create_dir_all(folder.join("assets")).expect` | [1054](../../src/resource_capability.rs#L1054) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `std::fs::create_dir_all` | [1054](../../src/resource_capability.rs#L1054) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `folder.join` | [1054](../../src/resource_capability.rs#L1054), [1060](../../src/resource_capability.rs#L1060) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `serde_json_canonicalizer::to_vec(&genesis).expect` | [1058](../../src/resource_capability.rs#L1058) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `serde_json_canonicalizer::to_vec` | [1058](../../src/resource_capability.rs#L1058) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `ledger.push` | [1059](../../src/resource_capability.rs#L1059) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `std::fs::write(folder.join("main.jsonl"), ledger).expect` | [1060](../../src/resource_capability.rs#L1060) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `std::fs::write` | [1060](../../src/resource_capability.rs#L1060) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `AttachmentAuthority::open` | [1061](../../src/resource_capability.rs#L1061) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `attachments             .upload_file(session, "notes.txt", Some("text/plain"), "aGVsbG8gZmlsZQ==")             .expect` | [1062](../../src/resource_capability.rs#L1062) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `attachments             .upload_file` | [1062](../../src/resource_capability.rs#L1062) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Some` | [1063](../../src/resource_capability.rs#L1063) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Arc::new` | [1065](../../src/resource_capability.rs#L1065), [1068](../../src/resource_capability.rs#L1068), [1069](../../src/resource_capability.rs#L1069) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `RecordingDelivery` | [1065](../../src/resource_capability.rs#L1065) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Mutex::new` | [1065](../../src/resource_capability.rs#L1065) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Vec::new` | [1065](../../src/resource_capability.rs#L1065) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `EndpointCommandInputAuthority::new(             Arc::clone(&delivery) as Arc<dyn SessionDeliveryAuthority>,             Arc::new(&#124;&#124; Ok("2026-08-29T00:00:00.000Z".to_owned())),             Arc::new(SessionInputAdmissionAuthority::new(                 root.path().to_path_buf(),             )),         )         .with_attachment_authority` | [1066](../../src/resource_capability.rs#L1066) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `EndpointCommandInputAuthority::new` | [1066](../../src/resource_capability.rs#L1066) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Arc::clone` | [1067](../../src/resource_capability.rs#L1067) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `Ok` | [1068](../../src/resource_capability.rs#L1068) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"2026-08-29T00:00:00.000Z".to_owned` | [1068](../../src/resource_capability.rs#L1068) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `SessionInputAdmissionAuthority::new` | [1069](../../src/resource_capability.rs#L1069) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `root.path().to_path_buf` | [1070](../../src/resource_capability.rs#L1070) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `ClientResourceService::new` | [1074](../../src/resource_capability.rs#L1074) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `catalog` | [1074](../../src/resource_capability.rs#L1074) | [tekes-supervisor::resource_capability::tests::catalog](../../src/resource_capability.rs#L747) |
| `command_attachments_follow_the_expanded_text_in_request_order` | `session.to_owned` | [1077](../../src/resource_capability.rs#L1077) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"review".to_owned` | [1078](../../src/resource_capability.rs#L1078) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"src/lib.rs".to_owned` | [1079](../../src/resource_capability.rs#L1079) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"command-attachments-1".to_owned` | [1080](../../src/resource_capability.rs#L1080) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `service.commands_run(&request, "uid:501").expect` | [1092](../../src/resource_capability.rs#L1092) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `service.commands_run` | [1092](../../src/resource_capability.rs#L1092) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `delivery.0.lock().expect` | [1094](../../src/resource_capability.rs#L1094) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `delivery.0.lock` | [1094](../../src/resource_capability.rs#L1094) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `recorded.as_ref().expect` | [1095](../../src/resource_capability.rs#L1095) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `recorded.as_ref` | [1095](../../src/resource_capability.rs#L1095) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `serde_json::to_value(&prompt.blocks[1]).expect` | [1106](../../src/resource_capability.rs#L1106) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `serde_json::to_value` | [1106](../../src/resource_capability.rs#L1106), [1109](../../src/resource_capability.rs#L1109) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `serde_json::to_value(&prompt.blocks[2]).expect` | [1109](../../src/resource_capability.rs#L1109) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `EffectiveInstructions::default` | [1131](../../src/resource_capability.rs#L1131) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `effective.commands.insert` | [1132](../../src/resource_capability.rs#L1132) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"compact.md".to_owned` | [1132](../../src/resource_capability.rs#L1132) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `ResourceCatalog::from_snapshot(&InstructionSnapshot {             format: 1,             sources,             effective,         })         .expect` | [1133](../../src/resource_capability.rs#L1133) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `ResourceCatalog::from_snapshot` | [1133](../../src/resource_capability.rs#L1133) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `service             .commands_run_with_catalog(                 &compact_catalog,                 &CommandRunRequest {                     name: "compact".to_owned(),                     arguments: String::new(),                     key: "command-compact-attachments".to_owned(),                     ..request.clone()                 },                 "uid:501",             )             .expect_err` | [1139](../../src/resource_capability.rs#L1139) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `service             .commands_run_with_catalog` | [1139](../../src/resource_capability.rs#L1139) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"compact".to_owned` | [1143](../../src/resource_capability.rs#L1143) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `String::new` | [1144](../../src/resource_capability.rs#L1144) | external-constructor-callback-or-unresolved |
| `command_attachments_follow_the_expanded_text_in_request_order` | `"command-compact-attachments".to_owned` | [1145](../../src/resource_capability.rs#L1145) | receiver-type-required |
| `command_attachments_follow_the_expanded_text_in_request_order` | `request.clone` | [1146](../../src/resource_capability.rs#L1146) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `EffectiveInstructions::default` | [1165](../../src/resource_capability.rs#L1165) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `effective.commands.insert` | [1166](../../src/resource_capability.rs#L1166), [1167](../../src/resource_capability.rs#L1167) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `"compact.md".to_owned` | [1166](../../src/resource_capability.rs#L1166) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `"compact-now.md".to_owned` | [1167](../../src/resource_capability.rs#L1167) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `ClientResourceService::new` | [1173](../../src/resource_capability.rs#L1173) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `ResourceCatalog::from_snapshot(&snapshot).expect` | [1174](../../src/resource_capability.rs#L1174) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `ResourceCatalog::from_snapshot` | [1174](../../src/resource_capability.rs#L1174) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `RecordingInput::default` | [1175](../../src/resource_capability.rs#L1175) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service                 .commands_run(                     &CommandRunRequest {                         session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),                         name: name.to_owned(),                         arguments: String::new(),                         key: key.to_owned(),                         attachments: Vec::new(),                     },                     "uid:501",                 )                 .expect` | [1178](../../src/resource_capability.rs#L1178) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service                 .commands_run` | [1178](../../src/resource_capability.rs#L1178) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `"018f0000-0000-7000-8000-000000000011".to_owned` | [1181](../../src/resource_capability.rs#L1181) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `name.to_owned` | [1182](../../src/resource_capability.rs#L1182) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `String::new` | [1183](../../src/resource_capability.rs#L1183) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `key.to_owned` | [1184](../../src/resource_capability.rs#L1184) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `Vec::new` | [1185](../../src/resource_capability.rs#L1185) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `run` | [1191](../../src/resource_capability.rs#L1191), [1194](../../src/resource_capability.rs#L1194) | external-constructor-callback-or-unresolved |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service.input.compacts.lock().expect` | [1196](../../src/resource_capability.rs#L1196) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service.input.compacts.lock` | [1196](../../src/resource_capability.rs#L1196) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service.input.calls.lock().expect` | [1197](../../src/resource_capability.rs#L1197) | receiver-type-required |
| `compact_verb_maps_to_the_manual_compaction_request_not_an_input` | `service.input.calls.lock` | [1197](../../src/resource_capability.rs#L1197) | receiver-type-required |
