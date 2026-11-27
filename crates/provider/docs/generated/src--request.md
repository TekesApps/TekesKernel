# provider::request

[Package atlas](index.md) · [Source](../../src/request.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [provider::request::AdapterId](../../src/request.rs#L14) | enum_item | `pub` |  |
| [provider::request::AdapterId::as_str](../../src/request.rs#L24) | function_item | `pub` |  |
| [provider::request::AdapterId::capabilities](../../src/request.rs#L35) | function_item | `pub` |  |
| [provider::request::Err](../../src/request.rs#L54) | type_item | `private` |  |
| [provider::request::AdapterId::from_str](../../src/request.rs#L56) | function_item | `private` |  |
| [provider::request::ProviderCapabilities](../../src/request.rs#L69) | struct_item | `pub` |  |
| [provider::request::PrepareInput](../../src/request.rs#L76) | struct_item | `pub` |  |
| [provider::request::PreparedRequest](../../src/request.rs#L88) | struct_item | `pub` |  |
| [provider::request::PrepareError](../../src/request.rs#L102) | enum_item | `pub` |  |
| [provider::request::endpoint_origin](../../src/request.rs#L113) | function_item | `pub` |  |
| [provider::request::ToolChoice](../../src/request.rs#L126) | enum_item | `pub` |  |
| [provider::request::prepare](../../src/request.rs#L132) | function_item | `pub` |  |
| [provider::request::prepare_with_tool_choice](../../src/request.rs#L137) | function_item | `pub` |  |
| [provider::request::prepare_with_native_deferred_tools](../../src/request.rs#L144) | function_item | `pub` |  |
| [provider::request::prepare_inner](../../src/request.rs#L152) | function_item | `private` |  |
| [provider::request::encode_request_body](../../src/request.rs#L451) | function_item | `private` |  |
| [provider::request::provider_request_digest](../../src/request.rs#L504) | function_item | `pub` |  |
| [provider::request::provider_request_digest_for_dialect](../../src/request.rs#L515) | function_item | `pub` |  |
| [provider::request::provider_query_key](../../src/request.rs#L550) | function_item | `pub` |  |
| [provider::request::value](../../src/request.rs#L557) | function_item | `private` |  |
| [provider::request::values](../../src/request.rs#L561) | function_item | `private` |  |
| [provider::request::anthropic_max_tokens](../../src/request.rs#L571) | function_item | `private` |  |
| [provider::request::anthropic_budget_for_effort](../../src/request.rs#L591) | function_item | `private` |  |
| [provider::request::apply_dialect_controls](../../src/request.rs#L603) | function_item | `private` |  |
| [provider::request::is_non_reasoning_profile](../../src/request.rs#L738) | function_item | `private` |  |
| [provider::request::apply_reasoning_disabled](../../src/request.rs#L751) | function_item | `private` |  |
| [provider::request::apply_title_output_cap](../../src/request.rs#L790) | function_item | `private` |  |
| [provider::request::unsupported_controls](../../src/request.rs#L838) | function_item | `private` |  |
| [provider::request::attach_anthropic_breakpoints](../../src/request.rs#L861) | function_item | `private` |  |
| [provider::request::merge_controls](../../src/request.rs#L894) | function_item | `private` |  |
| [provider::request::render_responses](../../src/request.rs#L912) | function_item | `private` |  |
| [provider::request::render_anthropic](../../src/request.rs#L972) | function_item | `private` |  |
| [provider::request::render_chat](../../src/request.rs#L1065) | function_item | `private` |  |
| [provider::request::render_google](../../src/request.rs#L1211) | function_item | `private` |  |
| [provider::request::google_function_response](../../src/request.rs#L1292) | function_item | `private` |  |
| [provider::request::render_interactions](../../src/request.rs#L1309) | function_item | `private` |  |
| [provider::request::render_tools](../../src/request.rs#L1434) | function_item | `private` |  |
| [provider::request::normalize_tool_parameters](../../src/request.rs#L1484) | function_item | `private` |  |
| [provider::request::ANTHROPIC_STRICT_UNSUPPORTED](../../src/request.rs#L1501) | const_item | `private` |  |
| [provider::request::anthropic_strict_subset](../../src/request.rs#L1513) | function_item | `private` |  |
| [provider::request::anthropic_root_conditional](../../src/request.rs#L1540) | function_item | `private` |  |
| [provider::request::kimi_nullable_constraints](../../src/request.rs#L1579) | function_item | `private` |  |
| [provider::request::anthropic_conditional_tests::kimi_nullable_bounds_preserve_null_enum_and_reject_composition_collision](../../src/request.rs#L1654) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::anthropic_conditional_tests::only_isolated_single_conditional_is_hoisted_without_losing_constraints](../../src/request.rs#L1673) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::contains_composition_constraints](../../src/request.rs#L1696) | function_item | `private` |  |
| [provider::request::contains_non_strict_object](../../src/request.rs#L1711) | function_item | `private` |  |
| [provider::request::strict_optional_tests::mcp_schema_metadata_and_omitted_empty_required_render_without_losing_constraints](../../src/request.rs#L1746) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::strict_optional_tests::unique_items_in_nullable_array_preserves_constraints_without_strict_mode](../../src/request.rs#L1767) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::strict_optional_tests::optional_fields_keep_their_schema_and_disable_strict_mode](../../src/request.rs#L1784) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::validate_tool_schema](../../src/request.rs#L1809) | function_item | `private` |  |
| [provider::request::item_role](../../src/request.rs#L1852) | function_item | `private` |  |
| [provider::request::sealed_fragments](../../src/request.rs#L1859) | function_item | `private` |  |
| [provider::request::sealed_fragment_value](../../src/request.rs#L1870) | function_item | `private` |  |
| [provider::request::DEGRADED_FILE_TEXT_LIMIT](../../src/request.rs#L1885) | const_item | `private` |  |
| [provider::request::degraded_file_text](../../src/request.rs#L1892) | function_item | `private` |  |
| [provider::request::data_url](../../src/request.rs#L1921) | function_item | `private` |  |
| [provider::request::item_content](../../src/request.rs#L1929) | function_item | `private` |  |
| [provider::request::block_type](../../src/request.rs#L1936) | function_item | `private` |  |
| [provider::request::required_text](../../src/request.rs#L1943) | function_item | `private` |  |
| [provider::request::required_value](../../src/request.rs#L1952) | function_item | `private` |  |
| [provider::request::tool_result_text](../../src/request.rs#L1962) | function_item | `private` |  |
| [provider::request::canonical_argument_string](../../src/request.rs#L1973) | function_item | `private` |  |
| [provider::request::invalid](../../src/request.rs#L1979) | function_item | `private` |  |
| [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::pdf_on_anthropic_is_still_a_document](../../src/request.rs#L2000) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::text_file_on_anthropic_degrades_to_fenced_text](../../src/request.rs#L2014) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::binary_file_on_chat_completions_degrades_to_the_stub](../../src/request.rs#L2027) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::oversized_utf8_file_degrades_to_the_stub](../../src/request.rs#L2045) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::text_file_on_responses_is_delivered_as_input_file](../../src/request.rs#L2073) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::google_and_interactions_degrade_in_their_native_text_forms](../../src/request.rs#L2098) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::file_degradation_tests::image_files_are_never_degraded](../../src/request.rs#L2121) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::anthropic_tool_adjacency_tests::a_bounded_schema_is_offered_to_anthropic_without_the_strict_claim](../../src/request.rs#L2161) | function_item | `private` | test; #[cfg(test)] |
| [provider::request::anthropic_tool_adjacency_tests::host_state_between_call_and_result_does_not_break_anthropic_adjacency](../../src/request.rs#L2200) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `FromStr` | `std::str::FromStr` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `DialectId` | `crate::DialectId` | `private` |
| `ProviderTarget` | `crate::ProviderTarget` | `private` |
| `validate_target` | `crate::validate_target` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |
| `_` | `base64::Engine` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `provider::request::anthropic_conditional_tests` | `private` | #[cfg(test)] |
| `provider::request::strict_optional_tests` | `private` | #[cfg(test)] |
| `provider::request::file_degradation_tests` | `private` | #[cfg(test)] |
| `provider::request::anthropic_tool_adjacency_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 26 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::validate_target"]
  n1["provider::native_deferred::apply"]
  n2["provider::request::endpoint_origin"]
  n3["provider::request::prepare"]
  n4["provider::request::prepare_with_tool_choice"]
  n5["provider::request::prepare_with_native_deferred_tools"]
  n6["provider::request::prepare_inner"]
  n7["provider::request::invalid"]
  n8["provider::request::AdapterId::as_str"]
  n9["provider::request::AdapterId::capabilities"]
  n10["provider::request::encode_request_body"]
  n11["provider::request::provider_request_digest"]
  n12["provider::request::provider_request_digest_for_dialect"]
  n13["provider::request::provider_query_key"]
  n14["provider::request::value"]
  n15["provider::request::values"]
  n16["provider::request::AdapterId::from_str"]
  n17["provider::request::anthropic_max_tokens"]
  n18["provider::request::anthropic_budget_for_effort"]
  n19["provider::request::apply_dialect_controls"]
  n20["provider::request::is_non_reasoning_profile"]
  n21["provider::request::apply_reasoning_disabled"]
  n22["provider::request::apply_title_output_cap"]
  n23["provider::request::unsupported_controls"]
  n24["provider::request::attach_anthropic_breakpoints"]
  n25["provider::request::merge_controls"]
  n3 --> n4
  n4 --> n6
  n5 --> n6
  n6 --> n0
  n6 --> n1
  n6 --> n7
  n6 --> n10
  n6 --> n12
  n6 --> n13
  n6 --> n14
  n6 --> n15
  n6 --> n19
  n6 --> n22
  n6 --> n24
  n6 --> n25
  n10 --> n7
  n11 --> n12
  n18 --> n7
  n19 --> n7
  n19 --> n18
  n19 --> n21
  n19 --> n23
  n21 --> n7
  n21 --> n20
  n22 --> n7
  n22 --> n20
```

</details>

<details><summary>Functions 21–40: 51 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::request::render_chat"]
  n1["provider::request::render_google"]
  n2["provider::request::google_function_response"]
  n3["provider::request::render_interactions"]
  n4["provider::request::render_tools"]
  n5["provider::request::normalize_tool_parameters"]
  n6["provider::request::anthropic_strict_subset"]
  n7["provider::request::anthropic_root_conditional"]
  n8["provider::request::kimi_nullable_constraints"]
  n9["provider::request::contains_composition_constraints"]
  n10["provider::request::contains_non_strict_object"]
  n11["provider::request::validate_tool_schema"]
  n12["provider::request::item_role"]
  n13["provider::request::sealed_fragments"]
  n14["provider::request::sealed_fragment_value"]
  n15["provider::request::degraded_file_text"]
  n16["provider::request::item_content"]
  n17["provider::request::block_type"]
  n18["provider::request::required_text"]
  n19["provider::request::required_value"]
  n20["provider::request::invalid"]
  n21["provider::request::unsupported_controls"]
  n22["provider::request::attach_anthropic_breakpoints"]
  n23["provider::request::merge_controls"]
  n24["provider::request::render_responses"]
  n25["provider::request::render_anthropic"]
  n0 --> n12
  n0 --> n14
  n0 --> n15
  n0 --> n16
  n0 --> n17
  n0 --> n18
  n0 --> n20
  n1 --> n12
  n1 --> n14
  n1 --> n16
  n1 --> n17
  n1 --> n18
  n1 --> n20
  n3 --> n12
  n3 --> n13
  n3 --> n16
  n3 --> n17
  n3 --> n18
  n3 --> n20
  n4 --> n5
  n4 --> n6
  n4 --> n8
  n4 --> n9
  n4 --> n10
  n4 --> n11
  n4 --> n18
  n4 --> n19
  n4 --> n20
  n6 --> n6
  n8 --> n8
  n8 --> n20
  n9 --> n9
  n11 --> n20
  n12 --> n20
  n13 --> n14
  n13 --> n20
  n14 --> n20
  n21 --> n20
  n22 --> n20
  n24 --> n12
  n24 --> n13
  n24 --> n16
  n24 --> n17
  n24 --> n18
  n24 --> n20
  n25 --> n12
  n25 --> n13
  n25 --> n16
  n25 --> n17
  n25 --> n18
  n25 --> n20
```

</details>

<details><summary>Functions 41–49: 8 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::request::degraded_file_text"]
  n1["provider::request::data_url"]
  n2["provider::request::item_content"]
  n3["provider::request::block_type"]
  n4["provider::request::required_text"]
  n5["provider::request::required_value"]
  n6["provider::request::tool_result_text"]
  n7["provider::request::canonical_argument_string"]
  n8["provider::request::invalid"]
  n0 --> n4
  n0 --> n8
  n2 --> n8
  n3 --> n8
  n4 --> n8
  n5 --> n8
  n6 --> n7
  n7 --> n8
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `from_str` | `Ok` | [58](../../src/request.rs#L58), [59](../../src/request.rs#L59), [60](../../src/request.rs#L60), [61](../../src/request.rs#L61), [62](../../src/request.rs#L62) | external-constructor-callback-or-unresolved |
| `from_str` | `Err` | [63](../../src/request.rs#L63) | external-constructor-callback-or-unresolved |
| `from_str` | `PrepareError::UnknownAdapter` | [63](../../src/request.rs#L63) | external-constructor-callback-or-unresolved |
| `from_str` | `other.to_owned` | [63](../../src/request.rs#L63) | receiver-type-required |
| `endpoint_origin` | `reqwest::Url::parse(endpoint)         .map_err` | [114](../../src/request.rs#L114) | receiver-type-required |
| `endpoint_origin` | `reqwest::Url::parse` | [114](../../src/request.rs#L114) | external-constructor-callback-or-unresolved |
| `endpoint_origin` | `PrepareError::InvalidEndpoint` | [115](../../src/request.rs#L115), [118](../../src/request.rs#L118), [121](../../src/request.rs#L121) | external-constructor-callback-or-unresolved |
| `endpoint_origin` | `error.to_string` | [115](../../src/request.rs#L115) | receiver-type-required |
| `endpoint_origin` | `url         .host_str()         .ok_or_else` | [116](../../src/request.rs#L116) | receiver-type-required |
| `endpoint_origin` | `url         .host_str` | [116](../../src/request.rs#L116) | receiver-type-required |
| `endpoint_origin` | `"endpoint has no host".to_owned` | [118](../../src/request.rs#L118) | receiver-type-required |
| `endpoint_origin` | `url         .port_or_known_default()         .ok_or_else` | [119](../../src/request.rs#L119) | receiver-type-required |
| `endpoint_origin` | `url         .port_or_known_default` | [119](../../src/request.rs#L119) | receiver-type-required |
| `endpoint_origin` | `"endpoint has no known port".to_owned` | [121](../../src/request.rs#L121) | receiver-type-required |
| `endpoint_origin` | `Ok` | [122](../../src/request.rs#L122) | external-constructor-callback-or-unresolved |
| `prepare` | `prepare_with_tool_choice` | [133](../../src/request.rs#L133) | [provider::request::prepare_with_tool_choice](../../src/request.rs#L137) |
| `prepare_with_tool_choice` | `prepare_inner` | [141](../../src/request.rs#L141) | [provider::request::prepare_inner](../../src/request.rs#L152) |
| `prepare_with_native_deferred_tools` | `prepare_inner` | [149](../../src/request.rs#L149) | [provider::request::prepare_inner](../../src/request.rs#L152) |
| `prepare_with_native_deferred_tools` | `Some` | [149](../../src/request.rs#L149) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `input.endpoint.trim_end_matches` | [157](../../src/request.rs#L157) | receiver-type-required |
| `prepare_inner` | `configured_base.starts_with` | [158](../../src/request.rs#L158), [159](../../src/request.rs#L159) | receiver-type-required |
| `prepare_inner` | `Err` | [161](../../src/request.rs#L161), [175](../../src/request.rs#L175), [182](../../src/request.rs#L182), [201](../../src/request.rs#L201), [227](../../src/request.rs#L227), [321](../../src/request.rs#L321), [385](../../src/request.rs#L385) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `PrepareError::InvalidEndpoint` | [161](../../src/request.rs#L161) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `input.endpoint.clone` | [161](../../src/request.rs#L161) | receiver-type-required |
| `prepare_inner` | `validate_target(&input.target)         .map_err` | [163](../../src/request.rs#L163) | receiver-type-required |
| `prepare_inner` | `validate_target` | [163](../../src/request.rs#L163) | [provider::dialect::validate_target](../../src/dialect.rs#L737) |
| `prepare_inner` | `PrepareError::InvalidTarget` | [164](../../src/request.rs#L164), [175](../../src/request.rs#L175), [182](../../src/request.rs#L182) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `error.to_string` | [164](../../src/request.rs#L164), [172](../../src/request.rs#L172) | receiver-type-required |
| `prepare_inner` | `dialect.family` | [166](../../src/request.rs#L166) | receiver-type-required |
| `prepare_inner` | `resolved.wire_model` | [167](../../src/request.rs#L167) | receiver-type-required |
| `prepare_inner` | `value` | [168](../../src/request.rs#L168), [196](../../src/request.rs#L196) | [provider::request::value](../../src/request.rs#L557) |
| `prepare_inner` | `profile.get` | [169](../../src/request.rs#L169), [179](../../src/request.rs#L179) | receiver-type-required |
| `prepare_inner` | `Some` | [170](../../src/request.rs#L170), [180](../../src/request.rs#L180), [337](../../src/request.rs#L337) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `serde_json::to_value(&input.target)                 .map_err` | [171](../../src/request.rs#L171) | receiver-type-required |
| `prepare_inner` | `serde_json::to_value` | [171](../../src/request.rs#L171) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `PrepareError::InvalidJson` | [172](../../src/request.rs#L172), [227](../../src/request.rs#L227) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `"epoch target does not match request target".to_owned` | [176](../../src/request.rs#L176) | receiver-type-required |
| `prepare_inner` | `profile.get("serializer_revision").and_then` | [179](../../src/request.rs#L179) | receiver-type-required |
| `prepare_inner` | `"epoch serializer revision does not match request target".to_owned` | [183](../../src/request.rs#L183) | receiver-type-required |
| `prepare_inner` | `profile         .get("system")         .and_then(Value::as_str)         .unwrap_or_default` | [186](../../src/request.rs#L186) | receiver-type-required |
| `prepare_inner` | `profile         .get("system")         .and_then` | [186](../../src/request.rs#L186) | receiver-type-required |
| `prepare_inner` | `profile         .get` | [186](../../src/request.rs#L186), [190](../../src/request.rs#L190) | receiver-type-required |
| `prepare_inner` | `profile         .get("controls")         .and_then(Value::as_object)         .cloned()         .unwrap_or_default` | [190](../../src/request.rs#L190) | receiver-type-required |
| `prepare_inner` | `profile         .get("controls")         .and_then(Value::as_object)         .cloned` | [190](../../src/request.rs#L190) | receiver-type-required |
| `prepare_inner` | `profile         .get("controls")         .and_then` | [190](../../src/request.rs#L190) | receiver-type-required |
| `prepare_inner` | `values` | [195](../../src/request.rs#L195) | [provider::request::values](../../src/request.rs#L561) |
| `prepare_inner` | `controls.remove` | [197](../../src/request.rs#L197), [198](../../src/request.rs#L198) | receiver-type-required |
| `prepare_inner` | `invalid` | [201](../../src/request.rs#L201), [211](../../src/request.rs#L211), [321](../../src/request.rs#L321), [365](../../src/request.rs#L365), [385](../../src/request.rs#L385) | [provider::request::invalid](../../src/request.rs#L1979) |
| `prepare_inner` | `controls         .remove("title_max_output_tokens")         .map(&#124;value&#124; {             value                 .as_u64()                 .filter(&#124;cap&#124; *cap > 0)                 .ok_or_else(&#124;&#124; invalid("title_max_output_tokens must be a positive integer"))         })         .transpose` | [205](../../src/request.rs#L205) | receiver-type-required |
| `prepare_inner` | `controls         .remove("title_max_output_tokens")         .map` | [205](../../src/request.rs#L205) | receiver-type-required |
| `prepare_inner` | `controls         .remove` | [205](../../src/request.rs#L205) | receiver-type-required |
| `prepare_inner` | `value                 .as_u64()                 .filter(&#124;cap&#124; *cap > 0)                 .ok_or_else` | [208](../../src/request.rs#L208) | receiver-type-required |
| `prepare_inner` | `value                 .as_u64()                 .filter` | [208](../../src/request.rs#L208) | receiver-type-required |
| `prepare_inner` | `value                 .as_u64` | [208](../../src/request.rs#L208) | receiver-type-required |
| `prepare_inner` | `apply_dialect_controls` | [214](../../src/request.rs#L214) | [provider::request::apply_dialect_controls](../../src/request.rs#L603) |
| `prepare_inner` | `controls         .get("thinking")         .and_then(&#124;thinking&#124; thinking.get("budget_tokens"))         .and_then` | [220](../../src/request.rs#L220) | receiver-type-required |
| `prepare_inner` | `controls         .get("thinking")         .and_then` | [220](../../src/request.rs#L220) | receiver-type-required |
| `prepare_inner` | `controls         .get` | [220](../../src/request.rs#L220) | receiver-type-required |
| `prepare_inner` | `thinking.get` | [222](../../src/request.rs#L222) | receiver-type-required |
| `prepare_inner` | `Value::from` | [225](../../src/request.rs#L225), [265](../../src/request.rs#L265) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `anthropic_max_tokens` | [225](../../src/request.rs#L225) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `input.continuation_id.is_some` | [226](../../src/request.rs#L226) | receiver-type-required |
| `prepare_inner` | `dialect.server_managed` | [226](../../src/request.rs#L226) | receiver-type-required |
| `prepare_inner` | `"stateless adapter cannot receive continuation_id".to_owned` | [228](../../src/request.rs#L228) | receiver-type-required |
| `prepare_inner` | `"/responses".to_owned` | [233](../../src/request.rs#L233) | receiver-type-required |
| `prepare_inner` | `"/messages".to_owned` | [237](../../src/request.rs#L237) | receiver-type-required |
| `prepare_inner` | `"/interactions".to_owned` | [252](../../src/request.rs#L252) | receiver-type-required |
| `prepare_inner` | `"/chat/completions".to_owned` | [256](../../src/request.rs#L256) | receiver-type-required |
| `prepare_inner` | `resolved.max_output_tokens` | [261](../../src/request.rs#L261) | receiver-type-required |
| `prepare_inner` | `body.as_object_mut()             .expect("chat body is an object")             .insert` | [263](../../src/request.rs#L263), [268](../../src/request.rs#L268) | receiver-type-required |
| `prepare_inner` | `body.as_object_mut()             .expect` | [263](../../src/request.rs#L263), [268](../../src/request.rs#L268), [273](../../src/request.rs#L273), [295](../../src/request.rs#L295) | receiver-type-required |
| `prepare_inner` | `body.as_object_mut` | [263](../../src/request.rs#L263), [268](../../src/request.rs#L268), [273](../../src/request.rs#L273), [282](../../src/request.rs#L282), [295](../../src/request.rs#L295) | receiver-type-required |
| `prepare_inner` | `"max_tokens".to_owned` | [265](../../src/request.rs#L265) | receiver-type-required |
| `prepare_inner` | `"stream_options".to_owned` | [270](../../src/request.rs#L270) | receiver-type-required |
| `prepare_inner` | `body.as_object_mut()             .expect("Responses body is an object")             .insert` | [273](../../src/request.rs#L273) | receiver-type-required |
| `prepare_inner` | `"store".to_owned` | [275](../../src/request.rs#L275) | receiver-type-required |
| `prepare_inner` | `Value::Bool` | [275](../../src/request.rs#L275) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `attach_anthropic_breakpoints` | [281](../../src/request.rs#L281) | [provider::request::attach_anthropic_breakpoints](../../src/request.rs#L861) |
| `prepare_inner` | `body.as_object_mut().expect` | [282](../../src/request.rs#L282) | receiver-type-required |
| `prepare_inner` | `resolved.refusal_fallback` | [283](../../src/request.rs#L283), [408](../../src/request.rs#L408) | receiver-type-required |
| `prepare_inner` | `object.insert` | [284](../../src/request.rs#L284) | receiver-type-required |
| `prepare_inner` | `"fallbacks".to_owned` | [284](../../src/request.rs#L284) | receiver-type-required |
| `prepare_inner` | `Value::String` | [284](../../src/request.rs#L284), [297](../../src/request.rs#L297) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `"default".to_owned` | [284](../../src/request.rs#L284) | receiver-type-required |
| `prepare_inner` | `body.as_object_mut()             .expect("adapter request bodies are objects")             .insert` | [295](../../src/request.rs#L295) | receiver-type-required |
| `prepare_inner` | `field.to_owned` | [297](../../src/request.rs#L297) | receiver-type-required |
| `prepare_inner` | `continuation.clone` | [297](../../src/request.rs#L297) | receiver-type-required |
| `prepare_inner` | `merge_controls` | [299](../../src/request.rs#L299) | [provider::request::merge_controls](../../src/request.rs#L894) |
| `prepare_inner` | `apply_title_output_cap` | [301](../../src/request.rs#L301) | [provider::request::apply_title_output_cap](../../src/request.rs#L790) |
| `prepare_inner` | `tools.as_array().is_none_or` | [320](../../src/request.rs#L320) | receiver-type-required |
| `prepare_inner` | `tools.as_array` | [320](../../src/request.rs#L320) | receiver-type-required |
| `prepare_inner` | `body.pointer("/thinking/type").and_then` | [337](../../src/request.rs#L337) | receiver-type-required |
| `prepare_inner` | `body.pointer` | [337](../../src/request.rs#L337) | receiver-type-required |
| `prepare_inner` | `resolved.forced_tool_choice` | [338](../../src/request.rs#L338) | receiver-type-required |
| `prepare_inner` | `body                     .as_object_mut()                     .expect("Interactions body is an object")                     .entry("generation_config".to_owned())                     .or_insert_with` | [358](../../src/request.rs#L358) | receiver-type-required |
| `prepare_inner` | `body                     .as_object_mut()                     .expect("Interactions body is an object")                     .entry` | [358](../../src/request.rs#L358) | receiver-type-required |
| `prepare_inner` | `body                     .as_object_mut()                     .expect` | [358](../../src/request.rs#L358) | receiver-type-required |
| `prepare_inner` | `body                     .as_object_mut` | [358](../../src/request.rs#L358) | receiver-type-required |
| `prepare_inner` | `"generation_config".to_owned` | [361](../../src/request.rs#L361) | receiver-type-required |
| `prepare_inner` | `config                     .as_object_mut()                     .ok_or_else(&#124;&#124; invalid("generation_config must be an object"))?                     .insert` | [363](../../src/request.rs#L363) | receiver-type-required |
| `prepare_inner` | `config                     .as_object_mut()                     .ok_or_else` | [363](../../src/request.rs#L363) | receiver-type-required |
| `prepare_inner` | `config                     .as_object_mut` | [363](../../src/request.rs#L363) | receiver-type-required |
| `prepare_inner` | `"tool_choice".to_owned` | [366](../../src/request.rs#L366) | receiver-type-required |
| `prepare_inner` | `item                 .get("content")                 .and_then(Value::as_array)                 .into_iter()                 .flatten` | [372](../../src/request.rs#L372) | receiver-type-required |
| `prepare_inner` | `item                 .get("content")                 .and_then(Value::as_array)                 .into_iter` | [372](../../src/request.rs#L372) | receiver-type-required |
| `prepare_inner` | `item                 .get("content")                 .and_then` | [372](../../src/request.rs#L372) | receiver-type-required |
| `prepare_inner` | `item                 .get` | [372](../../src/request.rs#L372) | receiver-type-required |
| `prepare_inner` | `native                         .references                         .iter()                         .any` | [380](../../src/request.rs#L380) | receiver-type-required |
| `prepare_inner` | `native                         .references                         .iter` | [380](../../src/request.rs#L380) | receiver-type-required |
| `prepare_inner` | `crate::native_deferred::apply` | [391](../../src/request.rs#L391) | [provider::native_deferred::apply](../../src/native_deferred.rs#L36) |
| `prepare_inner` | `encode_request_body` | [393](../../src/request.rs#L393) | [provider::request::encode_request_body](../../src/request.rs#L451) |
| `prepare_inner` | `BTreeMap::from` | [394](../../src/request.rs#L394) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `"accept".to_owned` | [396](../../src/request.rs#L396) | receiver-type-required |
| `prepare_inner` | `if input.stream {                 "text/event-stream"             } else {                 "application/json"             }             .to_owned` | [397](../../src/request.rs#L397) | receiver-type-required |
| `prepare_inner` | `"content-type".to_owned` | [404](../../src/request.rs#L404) | receiver-type-required |
| `prepare_inner` | `"application/json".to_owned` | [404](../../src/request.rs#L404) | receiver-type-required |
| `prepare_inner` | `headers_without_secret.insert` | [407](../../src/request.rs#L407), [409](../../src/request.rs#L409) | receiver-type-required |
| `prepare_inner` | `"anthropic-version".to_owned` | [407](../../src/request.rs#L407) | receiver-type-required |
| `prepare_inner` | `"2023-06-01".to_owned` | [407](../../src/request.rs#L407) | receiver-type-required |
| `prepare_inner` | `"anthropic-beta".to_owned` | [410](../../src/request.rs#L410), [418](../../src/request.rs#L418) | receiver-type-required |
| `prepare_inner` | `"server-side-fallback-2026-07-01".to_owned` | [411](../../src/request.rs#L411) | receiver-type-required |
| `prepare_inner` | `headers_without_secret             .entry("anthropic-beta".to_owned())             .or_default` | [417](../../src/request.rs#L417) | receiver-type-required |
| `prepare_inner` | `headers_without_secret             .entry` | [417](../../src/request.rs#L417) | receiver-type-required |
| `prepare_inner` | `beta.is_empty` | [420](../../src/request.rs#L420) | receiver-type-required |
| `prepare_inner` | `beta.push` | [421](../../src/request.rs#L421) | receiver-type-required |
| `prepare_inner` | `beta.push_str` | [423](../../src/request.rs#L423) | receiver-type-required |
| `prepare_inner` | `provider_request_digest_for_dialect` | [425](../../src/request.rs#L425) | [provider::request::provider_request_digest_for_dialect](../../src/request.rs#L515) |
| `prepare_inner` | `dialect.as_str` | [426](../../src/request.rs#L426) | receiver-type-required |
| `prepare_inner` | `adapter.capabilities` | [433](../../src/request.rs#L433) | receiver-type-required |
| `prepare_inner` | `capabilities         .query_by_identity         .then` | [434](../../src/request.rs#L434) | receiver-type-required |
| `prepare_inner` | `provider_query_key` | [436](../../src/request.rs#L436) | [provider::request::provider_query_key](../../src/request.rs#L550) |
| `prepare_inner` | `Ok` | [437](../../src/request.rs#L437) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `"POST".to_owned` | [438](../../src/request.rs#L438) | receiver-type-required |
| `prepare_inner` | `u64::try_from(body.len()).unwrap_or` | [443](../../src/request.rs#L443) | receiver-type-required |
| `prepare_inner` | `u64::try_from` | [443](../../src/request.rs#L443) | external-constructor-callback-or-unresolved |
| `prepare_inner` | `body.len` | [443](../../src/request.rs#L443) | receiver-type-required |
| `encode_request_body` | `serde_json_canonicalizer::to_vec(body)             .map_err` | [456](../../src/request.rs#L456) | receiver-type-required |
| `encode_request_body` | `serde_json_canonicalizer::to_vec` | [456](../../src/request.rs#L456), [496](../../src/request.rs#L496) | external-constructor-callback-or-unresolved |
| `encode_request_body` | `PrepareError::InvalidJson` | [457](../../src/request.rs#L457), [492](../../src/request.rs#L492), [497](../../src/request.rs#L497) | external-constructor-callback-or-unresolved |
| `encode_request_body` | `error.to_string` | [457](../../src/request.rs#L457), [492](../../src/request.rs#L492), [497](../../src/request.rs#L497) | receiver-type-required |
| `encode_request_body` | `body         .as_object()         .ok_or_else` | [459](../../src/request.rs#L459) | receiver-type-required |
| `encode_request_body` | `body         .as_object` | [459](../../src/request.rs#L459) | receiver-type-required |
| `encode_request_body` | `invalid` | [461](../../src/request.rs#L461), [475](../../src/request.rs#L475) | [provider::request::invalid](../../src/request.rs#L1979) |
| `encode_request_body` | `object.keys().any` | [474](../../src/request.rs#L474) | receiver-type-required |
| `encode_request_body` | `object.keys` | [474](../../src/request.rs#L474) | receiver-type-required |
| `encode_request_body` | `order.contains` | [474](../../src/request.rs#L474) | receiver-type-required |
| `encode_request_body` | `key.as_str` | [474](../../src/request.rs#L474) | receiver-type-required |
| `encode_request_body` | `Err` | [475](../../src/request.rs#L475) | external-constructor-callback-or-unresolved |
| `encode_request_body` | `object.get` | [483](../../src/request.rs#L483) | receiver-type-required |
| `encode_request_body` | `bytes.push` | [487](../../src/request.rs#L487), [494](../../src/request.rs#L494), [500](../../src/request.rs#L500) | receiver-type-required |
| `encode_request_body` | `bytes.extend_from_slice` | [490](../../src/request.rs#L490), [495](../../src/request.rs#L495) | receiver-type-required |
| `encode_request_body` | `serde_json::to_vec(key)                 .map_err` | [491](../../src/request.rs#L491) | receiver-type-required |
| `encode_request_body` | `serde_json::to_vec` | [491](../../src/request.rs#L491) | external-constructor-callback-or-unresolved |
| `encode_request_body` | `serde_json_canonicalizer::to_vec(value)                 .map_err` | [496](../../src/request.rs#L496) | receiver-type-required |
| `encode_request_body` | `Ok` | [501](../../src/request.rs#L501) | external-constructor-callback-or-unresolved |
| `provider_request_digest` | `provider_request_digest_for_dialect` | [512](../../src/request.rs#L512) | [provider::request::provider_request_digest_for_dialect](../../src/request.rs#L515) |
| `provider_request_digest` | `adapter.as_str` | [512](../../src/request.rs#L512) | receiver-type-required |
| `provider_request_digest_for_dialect` | `headers         .keys()         .any` | [523](../../src/request.rs#L523) | receiver-type-required |
| `provider_request_digest_for_dialect` | `headers         .keys` | [523](../../src/request.rs#L523) | receiver-type-required |
| `provider_request_digest_for_dialect` | `name.to_ascii_lowercase` | [525](../../src/request.rs#L525) | receiver-type-required |
| `provider_request_digest_for_dialect` | `Err` | [527](../../src/request.rs#L527) | external-constructor-callback-or-unresolved |
| `provider_request_digest_for_dialect` | `PrepareError::InvalidJson` | [527](../../src/request.rs#L527), [532](../../src/request.rs#L532) | external-constructor-callback-or-unresolved |
| `provider_request_digest_for_dialect` | `"header names must be lowercase".to_owned` | [528](../../src/request.rs#L528) | receiver-type-required |
| `provider_request_digest_for_dialect` | `serde_json_canonicalizer::to_vec(headers)         .map_err` | [531](../../src/request.rs#L531) | receiver-type-required |
| `provider_request_digest_for_dialect` | `serde_json_canonicalizer::to_vec` | [531](../../src/request.rs#L531) | external-constructor-callback-or-unresolved |
| `provider_request_digest_for_dialect` | `error.to_string` | [532](../../src/request.rs#L532) | receiver-type-required |
| `provider_request_digest_for_dialect` | `Sha256::new` | [533](../../src/request.rs#L533) | external-constructor-callback-or-unresolved |
| `provider_request_digest_for_dialect` | `hasher.update` | [534](../../src/request.rs#L534), [535](../../src/request.rs#L535), [536](../../src/request.rs#L536), [537](../../src/request.rs#L537), [538](../../src/request.rs#L538), [539](../../src/request.rs#L539), [540](../../src/request.rs#L540), [541](../../src/request.rs#L541), [542](../../src/request.rs#L542), [543](../../src/request.rs#L543), [544](../../src/request.rs#L544), [545](../../src/request.rs#L545) | receiver-type-required |
| `provider_request_digest_for_dialect` | `dialect_id.as_bytes` | [535](../../src/request.rs#L535) | receiver-type-required |
| `provider_request_digest_for_dialect` | `method.to_ascii_uppercase().as_bytes` | [537](../../src/request.rs#L537) | receiver-type-required |
| `provider_request_digest_for_dialect` | `method.to_ascii_uppercase` | [537](../../src/request.rs#L537) | receiver-type-required |
| `provider_request_digest_for_dialect` | `final_url.as_bytes` | [539](../../src/request.rs#L539) | receiver-type-required |
| `provider_request_digest_for_dialect` | `model.as_bytes` | [541](../../src/request.rs#L541) | receiver-type-required |
| `provider_request_digest_for_dialect` | `Ok` | [546](../../src/request.rs#L546) | external-constructor-callback-or-unresolved |
| `provider_query_key` | `Sha256::new` | [551](../../src/request.rs#L551) | external-constructor-callback-or-unresolved |
| `provider_query_key` | `hasher.update` | [552](../../src/request.rs#L552), [553](../../src/request.rs#L553) | receiver-type-required |
| `provider_query_key` | `attempt_id.as_bytes` | [553](../../src/request.rs#L553) | receiver-type-required |
| `value` | `serde_json::to_value(value).expect` | [558](../../src/request.rs#L558) | receiver-type-required |
| `value` | `serde_json::to_value` | [558](../../src/request.rs#L558) | external-constructor-callback-or-unresolved |
| `values` | `values.iter().map(value).collect` | [562](../../src/request.rs#L562) | receiver-type-required |
| `values` | `values.iter().map` | [562](../../src/request.rs#L562) | receiver-type-required |
| `values` | `values.iter` | [562](../../src/request.rs#L562) | receiver-type-required |
| `anthropic_max_tokens` | `profile.max_output_tokens` | [576](../../src/request.rs#L576) | receiver-type-required |
| `anthropic_max_tokens` | `base.max` | [586](../../src/request.rs#L586) | receiver-type-required |
| `anthropic_max_tokens` | `budget.saturating_add` | [586](../../src/request.rs#L586) | receiver-type-required |
| `anthropic_budget_for_effort` | `Ok` | [593](../../src/request.rs#L593), [594](../../src/request.rs#L594), [595](../../src/request.rs#L595), [596](../../src/request.rs#L596) | external-constructor-callback-or-unresolved |
| `anthropic_budget_for_effort` | `Err` | [597](../../src/request.rs#L597) | external-constructor-callback-or-unresolved |
| `anthropic_budget_for_effort` | `invalid` | [597](../../src/request.rs#L597) | [provider::request::invalid](../../src/request.rs#L1979) |
| `apply_dialect_controls` | `controls.is_empty` | [613](../../src/request.rs#L613) | receiver-type-required |
| `apply_dialect_controls` | `unsupported_controls` | [614](../../src/request.rs#L614) | [provider::request::unsupported_controls](../../src/request.rs#L838) |
| `apply_dialect_controls` | `apply_reasoning_disabled` | [617](../../src/request.rs#L617) | [provider::request::apply_reasoning_disabled](../../src/request.rs#L751) |
| `apply_dialect_controls` | `reasoning_effort.is_some` | [617](../../src/request.rs#L617) | receiver-type-required |
| `apply_dialect_controls` | `reasoning_effort         .as_ref()         .map(&#124;value&#124; {             let value = value                 .as_str()                 .ok_or_else(&#124;&#124; invalid("reasoning_effort must be a string"))?;             profile                 .reasoning_efforts()                 .iter()                 .any(&#124;item&#124; item == value)                 .then_some(value)                 .ok_or_else(&#124;&#124; {                     invalid(format!(                         "reasoning_effort {value} is unsupported by exact dialect {}",                         dialect.as_str()                     ))                 })         })         .transpose` | [619](../../src/request.rs#L619) | receiver-type-required |
| `apply_dialect_controls` | `reasoning_effort         .as_ref()         .map` | [619](../../src/request.rs#L619) | receiver-type-required |
| `apply_dialect_controls` | `reasoning_effort         .as_ref` | [619](../../src/request.rs#L619) | receiver-type-required |
| `apply_dialect_controls` | `value                 .as_str()                 .ok_or_else` | [622](../../src/request.rs#L622) | receiver-type-required |
| `apply_dialect_controls` | `value                 .as_str` | [622](../../src/request.rs#L622) | receiver-type-required |
| `apply_dialect_controls` | `invalid` | [624](../../src/request.rs#L624), [631](../../src/request.rs#L631), [706](../../src/request.rs#L706), [714](../../src/request.rs#L714) | [provider::request::invalid](../../src/request.rs#L1979) |
| `apply_dialect_controls` | `profile                 .reasoning_efforts()                 .iter()                 .any(&#124;item&#124; item == value)                 .then_some(value)                 .ok_or_else` | [625](../../src/request.rs#L625) | receiver-type-required |
| `apply_dialect_controls` | `profile                 .reasoning_efforts()                 .iter()                 .any(&#124;item&#124; item == value)                 .then_some` | [625](../../src/request.rs#L625) | receiver-type-required |
| `apply_dialect_controls` | `profile                 .reasoning_efforts()                 .iter()                 .any` | [625](../../src/request.rs#L625) | receiver-type-required |
| `apply_dialect_controls` | `profile                 .reasoning_efforts()                 .iter` | [625](../../src/request.rs#L625) | receiver-type-required |
| `apply_dialect_controls` | `profile                 .reasoning_efforts` | [625](../../src/request.rs#L625) | receiver-type-required |
| `apply_dialect_controls` | `effort.is_some` | [639](../../src/request.rs#L639), [642](../../src/request.rs#L642), [646](../../src/request.rs#L646), [649](../../src/request.rs#L649), [690](../../src/request.rs#L690), [724](../../src/request.rs#L724) | receiver-type-required |
| `apply_dialect_controls` | `controls.insert` | [640](../../src/request.rs#L640), [643](../../src/request.rs#L643), [644](../../src/request.rs#L644), [647](../../src/request.rs#L647), [650](../../src/request.rs#L650), [651](../../src/request.rs#L651), [663](../../src/request.rs#L663), [668](../../src/request.rs#L668), [674](../../src/request.rs#L674), [682](../../src/request.rs#L682), [683](../../src/request.rs#L683), [719](../../src/request.rs#L719), [725](../../src/request.rs#L725) | receiver-type-required |
| `apply_dialect_controls` | `"reasoning".to_owned` | [640](../../src/request.rs#L640) | receiver-type-required |
| `apply_dialect_controls` | `"reasoning_effort".to_owned` | [643](../../src/request.rs#L643), [647](../../src/request.rs#L647), [650](../../src/request.rs#L650) | receiver-type-required |
| `apply_dialect_controls` | `"thinking".to_owned` | [644](../../src/request.rs#L644), [652](../../src/request.rs#L652), [664](../../src/request.rs#L664), [675](../../src/request.rs#L675), [682](../../src/request.rs#L682) | receiver-type-required |
| `apply_dialect_controls` | `profile.thinking_wire` | [657](../../src/request.rs#L657), [692](../../src/request.rs#L692) | receiver-type-required |
| `apply_dialect_controls` | `"output_config".to_owned` | [668](../../src/request.rs#L668), [683](../../src/request.rs#L683) | receiver-type-required |
| `apply_dialect_controls` | `anthropic_budget_for_effort` | [673](../../src/request.rs#L673) | [provider::request::anthropic_budget_for_effort](../../src/request.rs#L591) |
| `apply_dialect_controls` | `effort.expect` | [691](../../src/request.rs#L691) | receiver-type-required |
| `apply_dialect_controls` | `Err` | [706](../../src/request.rs#L706), [714](../../src/request.rs#L714) | external-constructor-callback-or-unresolved |
| `apply_dialect_controls` | `"generationConfig".to_owned` | [720](../../src/request.rs#L720) | receiver-type-required |
| `apply_dialect_controls` | `"generation_config".to_owned` | [726](../../src/request.rs#L726) | receiver-type-required |
| `apply_dialect_controls` | `profile.pro_reasoning` | [732](../../src/request.rs#L732) | receiver-type-required |
| `apply_dialect_controls` | `controls.entry("reasoning").or_insert_with` | [733](../../src/request.rs#L733) | receiver-type-required |
| `apply_dialect_controls` | `controls.entry` | [733](../../src/request.rs#L733) | receiver-type-required |
| `apply_dialect_controls` | `Ok` | [735](../../src/request.rs#L735) | external-constructor-callback-or-unresolved |
| `is_non_reasoning_profile` | `profile.reasoning_efforts().is_empty` | [739](../../src/request.rs#L739) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.reasoning_efforts` | [739](../../src/request.rs#L739) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.default_reasoning_effort().is_none` | [740](../../src/request.rs#L740) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.default_reasoning_effort` | [740](../../src/request.rs#L740) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.thinking_wire().is_none` | [741](../../src/request.rs#L741) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.thinking_wire` | [741](../../src/request.rs#L741) | receiver-type-required |
| `is_non_reasoning_profile` | `profile.pro_reasoning` | [742](../../src/request.rs#L742) | receiver-type-required |
| `apply_reasoning_disabled` | `Err` | [757](../../src/request.rs#L757), [775](../../src/request.rs#L775) | external-constructor-callback-or-unresolved |
| `apply_reasoning_disabled` | `invalid` | [757](../../src/request.rs#L757), [775](../../src/request.rs#L775) | [provider::request::invalid](../../src/request.rs#L1979) |
| `apply_reasoning_disabled` | `controls.insert` | [764](../../src/request.rs#L764), [767](../../src/request.rs#L767) | receiver-type-required |
| `apply_reasoning_disabled` | `"reasoning".to_owned` | [764](../../src/request.rs#L764) | receiver-type-required |
| `apply_reasoning_disabled` | `"thinking".to_owned` | [767](../../src/request.rs#L767) | receiver-type-required |
| `apply_reasoning_disabled` | `is_non_reasoning_profile` | [769](../../src/request.rs#L769) | [provider::request::is_non_reasoning_profile](../../src/request.rs#L738) |
| `apply_reasoning_disabled` | `Ok` | [781](../../src/request.rs#L781) | external-constructor-callback-or-unresolved |
| `apply_title_output_cap` | `body         .as_object_mut()         .ok_or_else` | [796](../../src/request.rs#L796) | receiver-type-required |
| `apply_title_output_cap` | `body         .as_object_mut` | [796](../../src/request.rs#L796) | receiver-type-required |
| `apply_title_output_cap` | `invalid` | [798](../../src/request.rs#L798), [822](../../src/request.rs#L822), [829](../../src/request.rs#L829) | [provider::request::invalid](../../src/request.rs#L1979) |
| `apply_title_output_cap` | `object.insert` | [801](../../src/request.rs#L801), [808](../../src/request.rs#L808), [812](../../src/request.rs#L812), [819](../../src/request.rs#L819) | receiver-type-required |
| `apply_title_output_cap` | `"max_output_tokens".to_owned` | [801](../../src/request.rs#L801), [812](../../src/request.rs#L812) | receiver-type-required |
| `apply_title_output_cap` | `Value::from` | [801](../../src/request.rs#L801), [808](../../src/request.rs#L808), [812](../../src/request.rs#L812), [819](../../src/request.rs#L819) | external-constructor-callback-or-unresolved |
| `apply_title_output_cap` | `object                 .get("max_tokens")                 .and_then(Value::as_u64)                 .map_or` | [804](../../src/request.rs#L804) | receiver-type-required |
| `apply_title_output_cap` | `object                 .get("max_tokens")                 .and_then` | [804](../../src/request.rs#L804) | receiver-type-required |
| `apply_title_output_cap` | `object                 .get` | [804](../../src/request.rs#L804) | receiver-type-required |
| `apply_title_output_cap` | `c.min` | [807](../../src/request.rs#L807), [818](../../src/request.rs#L818) | receiver-type-required |
| `apply_title_output_cap` | `"max_tokens".to_owned` | [808](../../src/request.rs#L808), [819](../../src/request.rs#L819) | receiver-type-required |
| `apply_title_output_cap` | `is_non_reasoning_profile` | [810](../../src/request.rs#L810) | [provider::request::is_non_reasoning_profile](../../src/request.rs#L738) |
| `apply_title_output_cap` | `dialect.family` | [810](../../src/request.rs#L810) | receiver-type-required |
| `apply_title_output_cap` | `object                     .get("max_tokens")                     .and_then(Value::as_u64)                     .map_or` | [815](../../src/request.rs#L815) | receiver-type-required |
| `apply_title_output_cap` | `object                     .get("max_tokens")                     .and_then` | [815](../../src/request.rs#L815) | receiver-type-required |
| `apply_title_output_cap` | `object                     .get` | [815](../../src/request.rs#L815) | receiver-type-required |
| `apply_title_output_cap` | `Err` | [822](../../src/request.rs#L822), [829](../../src/request.rs#L829) | external-constructor-callback-or-unresolved |
| `apply_title_output_cap` | `Ok` | [835](../../src/request.rs#L835) | external-constructor-callback-or-unresolved |
| `unsupported_controls` | `Err` | [842](../../src/request.rs#L842) | external-constructor-callback-or-unresolved |
| `unsupported_controls` | `invalid` | [842](../../src/request.rs#L842) | [provider::request::invalid](../../src/request.rs#L1979) |
| `attach_anthropic_breakpoints` | `body         .get_mut("tools")         .and_then(Value::as_array_mut)         .and_then` | [863](../../src/request.rs#L863) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("tools")         .and_then` | [863](../../src/request.rs#L863) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut` | [863](../../src/request.rs#L863), [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `tools.last_mut` | [866](../../src/request.rs#L866) | receiver-type-required |
| `attach_anthropic_breakpoints` | `tool.as_object_mut()             .ok_or_else(&#124;&#124; invalid("tool declaration must be an object"))?             .insert` | [868](../../src/request.rs#L868) | receiver-type-required |
| `attach_anthropic_breakpoints` | `tool.as_object_mut()             .ok_or_else` | [868](../../src/request.rs#L868) | receiver-type-required |
| `attach_anthropic_breakpoints` | `tool.as_object_mut` | [868](../../src/request.rs#L868) | receiver-type-required |
| `attach_anthropic_breakpoints` | `invalid` | [869](../../src/request.rs#L869), [887](../../src/request.rs#L887) | [provider::request::invalid](../../src/request.rs#L1979) |
| `attach_anthropic_breakpoints` | `"cache_control".to_owned` | [870](../../src/request.rs#L870), [888](../../src/request.rs#L888) | receiver-type-required |
| `attach_anthropic_breakpoints` | `marker.clone` | [870](../../src/request.rs#L870) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("messages")         .and_then(Value::as_array_mut)         .and_then(&#124;messages&#124; messages.last_mut())         .and_then(&#124;message&#124; message.get_mut("content"))         .and_then(Value::as_array_mut)         .and_then` | [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("messages")         .and_then(Value::as_array_mut)         .and_then(&#124;messages&#124; messages.last_mut())         .and_then(&#124;message&#124; message.get_mut("content"))         .and_then` | [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("messages")         .and_then(Value::as_array_mut)         .and_then(&#124;messages&#124; messages.last_mut())         .and_then` | [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("messages")         .and_then(Value::as_array_mut)         .and_then` | [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `body         .get_mut("messages")         .and_then` | [872](../../src/request.rs#L872) | receiver-type-required |
| `attach_anthropic_breakpoints` | `messages.last_mut` | [875](../../src/request.rs#L875) | receiver-type-required |
| `attach_anthropic_breakpoints` | `message.get_mut` | [876](../../src/request.rs#L876) | receiver-type-required |
| `attach_anthropic_breakpoints` | `content.last_mut` | [878](../../src/request.rs#L878) | receiver-type-required |
| `attach_anthropic_breakpoints` | `block                 .as_object_mut()                 .ok_or_else(&#124;&#124; invalid("content block must be an object"))?                 .insert` | [885](../../src/request.rs#L885) | receiver-type-required |
| `attach_anthropic_breakpoints` | `block                 .as_object_mut()                 .ok_or_else` | [885](../../src/request.rs#L885) | receiver-type-required |
| `attach_anthropic_breakpoints` | `block                 .as_object_mut` | [885](../../src/request.rs#L885) | receiver-type-required |
| `attach_anthropic_breakpoints` | `Ok` | [891](../../src/request.rs#L891) | external-constructor-callback-or-unresolved |
| `merge_controls` | `body         .as_object_mut()         .ok_or_else` | [898](../../src/request.rs#L898) | receiver-type-required |
| `merge_controls` | `body         .as_object_mut` | [898](../../src/request.rs#L898) | receiver-type-required |
| `merge_controls` | `PrepareError::InvalidJson` | [900](../../src/request.rs#L900), [903](../../src/request.rs#L903) | external-constructor-callback-or-unresolved |
| `merge_controls` | `"request body is not an object".to_owned` | [900](../../src/request.rs#L900) | receiver-type-required |
| `merge_controls` | `object.contains_key` | [902](../../src/request.rs#L902) | receiver-type-required |
| `merge_controls` | `Err` | [903](../../src/request.rs#L903) | external-constructor-callback-or-unresolved |
| `merge_controls` | `object.insert` | [907](../../src/request.rs#L907) | receiver-type-required |
| `merge_controls` | `Ok` | [909](../../src/request.rs#L909) | external-constructor-callback-or-unresolved |
| `render_responses` | `Vec::new` | [913](../../src/request.rs#L913), [921](../../src/request.rs#L921) | external-constructor-callback-or-unresolved |
| `render_responses` | `sealed_fragments` | [915](../../src/request.rs#L915) | [provider::request::sealed_fragments](../../src/request.rs#L1859) |
| `render_responses` | `output.extend` | [916](../../src/request.rs#L916) | receiver-type-required |
| `render_responses` | `item_role` | [919](../../src/request.rs#L919) | [provider::request::item_role](../../src/request.rs#L1852) |
| `render_responses` | `item_content` | [920](../../src/request.rs#L920) | [provider::request::item_content](../../src/request.rs#L1929) |
| `render_responses` | `block_type` | [923](../../src/request.rs#L923) | [provider::request::block_type](../../src/request.rs#L1936) |
| `render_responses` | `blocks.push` | [924](../../src/request.rs#L924), [939](../../src/request.rs#L939), [947](../../src/request.rs#L947), [952](../../src/request.rs#L952), [958](../../src/request.rs#L958) | receiver-type-required |
| `render_responses` | `output.push` | [928](../../src/request.rs#L928), [934](../../src/request.rs#L934), [966](../../src/request.rs#L966) | receiver-type-required |
| `render_responses` | `required_text(block, "mime")?.starts_with` | [944](../../src/request.rs#L944) | receiver-type-required |
| `render_responses` | `required_text` | [944](../../src/request.rs#L944) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_responses` | `dialect.input_blocks().contains` | [945](../../src/request.rs#L945), [952](../../src/request.rs#L952) | receiver-type-required |
| `render_responses` | `dialect.input_blocks` | [945](../../src/request.rs#L945), [952](../../src/request.rs#L952) | receiver-type-required |
| `render_responses` | `Err` | [962](../../src/request.rs#L962) | external-constructor-callback-or-unresolved |
| `render_responses` | `invalid` | [962](../../src/request.rs#L962) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_responses` | `blocks.is_empty` | [965](../../src/request.rs#L965) | receiver-type-required |
| `render_responses` | `Ok` | [969](../../src/request.rs#L969) | external-constructor-callback-or-unresolved |
| `render_anthropic` | `Vec::new` | [973](../../src/request.rs#L973), [980](../../src/request.rs#L980), [1043](../../src/request.rs#L1043) | external-constructor-callback-or-unresolved |
| `render_anthropic` | `sealed_fragments` | [975](../../src/request.rs#L975) | [provider::request::sealed_fragments](../../src/request.rs#L1859) |
| `render_anthropic` | `messages.push` | [976](../../src/request.rs#L976), [1037](../../src/request.rs#L1037) | receiver-type-required |
| `render_anthropic` | `item_role` | [979](../../src/request.rs#L979) | [provider::request::item_role](../../src/request.rs#L1852) |
| `render_anthropic` | `item_content` | [981](../../src/request.rs#L981) | [provider::request::item_content](../../src/request.rs#L1929) |
| `render_anthropic` | `block_type` | [982](../../src/request.rs#L982) | [provider::request::block_type](../../src/request.rs#L1936) |
| `render_anthropic` | `blocks.push` | [984](../../src/request.rs#L984), [986](../../src/request.rs#L986), [1004](../../src/request.rs#L1004), [1010](../../src/request.rs#L1010), [1015](../../src/request.rs#L1015), [1021](../../src/request.rs#L1021), [1027](../../src/request.rs#L1027) | receiver-type-required |
| `render_anthropic` | `dialect.supports_reasoning_blocks` | [986](../../src/request.rs#L986) | receiver-type-required |
| `render_anthropic` | `Err` | [990](../../src/request.rs#L990), [999](../../src/request.rs#L999), [1033](../../src/request.rs#L1033) | external-constructor-callback-or-unresolved |
| `render_anthropic` | `invalid` | [990](../../src/request.rs#L990), [999](../../src/request.rs#L999), [1033](../../src/request.rs#L1033) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_anthropic` | `required_text` | [996](../../src/request.rs#L996) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_anthropic` | `mime.starts_with` | [997](../../src/request.rs#L997) | receiver-type-required |
| `render_anthropic` | `dialect.input_blocks().contains` | [998](../../src/request.rs#L998), [1008](../../src/request.rs#L1008) | receiver-type-required |
| `render_anthropic` | `dialect.input_blocks` | [998](../../src/request.rs#L998), [1008](../../src/request.rs#L1008) | receiver-type-required |
| `render_anthropic` | `merged.last().is_some_and` | [1045](../../src/request.rs#L1045) | receiver-type-required |
| `render_anthropic` | `merged.last` | [1045](../../src/request.rs#L1045) | receiver-type-required |
| `render_anthropic` | `merged.last_mut().unwrap()["content"]                 .as_array_mut()                 .unwrap()                 .extend` | [1046](../../src/request.rs#L1046) | receiver-type-required |
| `render_anthropic` | `merged.last_mut().unwrap()["content"]                 .as_array_mut()                 .unwrap` | [1046](../../src/request.rs#L1046) | receiver-type-required |
| `render_anthropic` | `merged.last_mut().unwrap()["content"]                 .as_array_mut` | [1046](../../src/request.rs#L1046) | receiver-type-required |
| `render_anthropic` | `merged.last_mut().unwrap` | [1046](../../src/request.rs#L1046) | receiver-type-required |
| `render_anthropic` | `merged.last_mut` | [1046](../../src/request.rs#L1046) | receiver-type-required |
| `render_anthropic` | `message["content"].as_array().unwrap().iter().cloned` | [1049](../../src/request.rs#L1049) | receiver-type-required |
| `render_anthropic` | `message["content"].as_array().unwrap().iter` | [1049](../../src/request.rs#L1049) | receiver-type-required |
| `render_anthropic` | `message["content"].as_array().unwrap` | [1049](../../src/request.rs#L1049) | receiver-type-required |
| `render_anthropic` | `message["content"].as_array` | [1049](../../src/request.rs#L1049) | receiver-type-required |
| `render_anthropic` | `merged.push` | [1051](../../src/request.rs#L1051) | receiver-type-required |
| `render_anthropic` | `message["content"]                 .as_array_mut()                 .unwrap()                 .sort_by_key` | [1056](../../src/request.rs#L1056) | receiver-type-required |
| `render_anthropic` | `message["content"]                 .as_array_mut()                 .unwrap` | [1056](../../src/request.rs#L1056) | receiver-type-required |
| `render_anthropic` | `message["content"]                 .as_array_mut` | [1056](../../src/request.rs#L1056) | receiver-type-required |
| `render_anthropic` | `Ok` | [1062](../../src/request.rs#L1062) | external-constructor-callback-or-unresolved |
| `render_chat` | `Vec::new` | [1070](../../src/request.rs#L1070), [1084](../../src/request.rs#L1084), [1125](../../src/request.rs#L1125) | external-constructor-callback-or-unresolved |
| `render_chat` | `system.is_empty` | [1071](../../src/request.rs#L1071) | receiver-type-required |
| `render_chat` | `messages.push` | [1072](../../src/request.rs#L1072), [1076](../../src/request.rs#L1076), [1122](../../src/request.rs#L1122), [1133](../../src/request.rs#L1133), [1143](../../src/request.rs#L1143), [1148](../../src/request.rs#L1148), [1182](../../src/request.rs#L1182), [1197](../../src/request.rs#L1197) | receiver-type-required |
| `render_chat` | `sealed_fragment_value` | [1075](../../src/request.rs#L1075) | [provider::request::sealed_fragment_value](../../src/request.rs#L1870) |
| `render_chat` | `item_role` | [1079](../../src/request.rs#L1079) | [provider::request::item_role](../../src/request.rs#L1852) |
| `render_chat` | `item_content` | [1080](../../src/request.rs#L1080) | [provider::request::item_content](../../src/request.rs#L1929) |
| `render_chat` | `String::new` | [1082](../../src/request.rs#L1082), [1083](../../src/request.rs#L1083) | external-constructor-callback-or-unresolved |
| `render_chat` | `block_type` | [1086](../../src/request.rs#L1086), [1152](../../src/request.rs#L1152) | [provider::request::block_type](../../src/request.rs#L1936) |
| `render_chat` | `text.push_str` | [1087](../../src/request.rs#L1087), [1105](../../src/request.rs#L1105) | receiver-type-required |
| `render_chat` | `required_text` | [1087](../../src/request.rs#L1087), [1095](../../src/request.rs#L1095), [1167](../../src/request.rs#L1167) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_chat` | `dialect.supports_reasoning_blocks` | [1089](../../src/request.rs#L1089), [1158](../../src/request.rs#L1158) | receiver-type-required |
| `render_chat` | `Err` | [1090](../../src/request.rs#L1090), [1106](../../src/request.rs#L1106), [1159](../../src/request.rs#L1159), [1164](../../src/request.rs#L1164), [1203](../../src/request.rs#L1203) | external-constructor-callback-or-unresolved |
| `render_chat` | `invalid` | [1090](../../src/request.rs#L1090), [1106](../../src/request.rs#L1106), [1159](../../src/request.rs#L1159), [1164](../../src/request.rs#L1164), [1203](../../src/request.rs#L1203) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_chat` | `reasoning.push_str` | [1095](../../src/request.rs#L1095) | receiver-type-required |
| `render_chat` | `tool_calls.push` | [1097](../../src/request.rs#L1097) | receiver-type-required |
| `render_chat` | `degraded_file_text` | [1105](../../src/request.rs#L1105) | [provider::request::degraded_file_text](../../src/request.rs#L1892) |
| `render_chat` | `text.is_empty` | [1109](../../src/request.rs#L1109) | receiver-type-required |
| `render_chat` | `tool_calls.is_empty` | [1109](../../src/request.rs#L1109), [1119](../../src/request.rs#L1119) | receiver-type-required |
| `render_chat` | `Value::String` | [1112](../../src/request.rs#L1112), [1117](../../src/request.rs#L1117) | external-constructor-callback-or-unresolved |
| `render_chat` | `message.as_object_mut().expect` | [1115](../../src/request.rs#L1115) | receiver-type-required |
| `render_chat` | `message.as_object_mut` | [1115](../../src/request.rs#L1115) | receiver-type-required |
| `render_chat` | `reasoning.is_empty` | [1116](../../src/request.rs#L1116) | receiver-type-required |
| `render_chat` | `object.insert` | [1117](../../src/request.rs#L1117), [1120](../../src/request.rs#L1120) | receiver-type-required |
| `render_chat` | `"reasoning_content".to_owned` | [1117](../../src/request.rs#L1117) | receiver-type-required |
| `render_chat` | `"tool_calls".to_owned` | [1120](../../src/request.rs#L1120) | receiver-type-required |
| `render_chat` | `Value::Array` | [1120](../../src/request.rs#L1120) | external-constructor-callback-or-unresolved |
| `render_chat` | `parts.is_empty` | [1129](../../src/request.rs#L1129) | receiver-type-required |
| `render_chat` | `parts                         .drain(..)                         .filter_map(&#124;p&#124; p.get("text").and_then(Value::as_str).map(str::to_owned))                         .collect::<Vec<_>>()                         .join` | [1138](../../src/request.rs#L1138) | receiver-type-required |
| `render_chat` | `parts                         .drain(..)                         .filter_map(&#124;p&#124; p.get("text").and_then(Value::as_str).map(str::to_owned))                         .collect::<Vec<_>>` | [1138](../../src/request.rs#L1138) | receiver-type-required |
| `render_chat` | `parts                         .drain(..)                         .filter_map` | [1138](../../src/request.rs#L1138) | receiver-type-required |
| `render_chat` | `parts                         .drain` | [1138](../../src/request.rs#L1138) | receiver-type-required |
| `render_chat` | `p.get("text").and_then(Value::as_str).map` | [1140](../../src/request.rs#L1140) | receiver-type-required |
| `render_chat` | `p.get("text").and_then` | [1140](../../src/request.rs#L1140) | receiver-type-required |
| `render_chat` | `p.get` | [1140](../../src/request.rs#L1140) | receiver-type-required |
| `render_chat` | `content.is_empty` | [1147](../../src/request.rs#L1147) | receiver-type-required |
| `render_chat` | `user_parts.push` | [1153](../../src/request.rs#L1153), [1171](../../src/request.rs#L1171), [1176](../../src/request.rs#L1176) | receiver-type-required |
| `render_chat` | `required_text(block, "mime")?.starts_with` | [1167](../../src/request.rs#L1167) | receiver-type-required |
| `render_chat` | `dialect.input_blocks().contains` | [1168](../../src/request.rs#L1168) | receiver-type-required |
| `render_chat` | `dialect.input_blocks` | [1168](../../src/request.rs#L1168) | receiver-type-required |
| `render_chat` | `flush_user_parts` | [1181](../../src/request.rs#L1181), [1196](../../src/request.rs#L1196), [1206](../../src/request.rs#L1206) | external-constructor-callback-or-unresolved |
| `render_chat` | `Ok` | [1208](../../src/request.rs#L1208) | external-constructor-callback-or-unresolved |
| `render_google` | `Vec::new` | [1212](../../src/request.rs#L1212), [1219](../../src/request.rs#L1219), [1270](../../src/request.rs#L1270) | external-constructor-callback-or-unresolved |
| `render_google` | `sealed_fragment_value` | [1214](../../src/request.rs#L1214) | [provider::request::sealed_fragment_value](../../src/request.rs#L1870) |
| `render_google` | `contents.push` | [1215](../../src/request.rs#L1215), [1264](../../src/request.rs#L1264) | receiver-type-required |
| `render_google` | `item_role` | [1218](../../src/request.rs#L1218) | [provider::request::item_role](../../src/request.rs#L1852) |
| `render_google` | `item_content` | [1220](../../src/request.rs#L1220) | [provider::request::item_content](../../src/request.rs#L1929) |
| `render_google` | `block_type` | [1221](../../src/request.rs#L1221) | [provider::request::block_type](../../src/request.rs#L1936) |
| `render_google` | `parts.push` | [1223](../../src/request.rs#L1223), [1226](../../src/request.rs#L1226), [1242](../../src/request.rs#L1242), [1244](../../src/request.rs#L1244), [1246](../../src/request.rs#L1246), [1251](../../src/request.rs#L1251), [1256](../../src/request.rs#L1256) | receiver-type-required |
| `render_google` | `dialect.supports_reasoning_blocks` | [1225](../../src/request.rs#L1225) | receiver-type-required |
| `render_google` | `Err` | [1229](../../src/request.rs#L1229), [1261](../../src/request.rs#L1261) | external-constructor-callback-or-unresolved |
| `render_google` | `invalid` | [1229](../../src/request.rs#L1229), [1261](../../src/request.rs#L1261) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_google` | `required_text` | [1235](../../src/request.rs#L1235) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_google` | `mime.starts_with` | [1236](../../src/request.rs#L1236) | receiver-type-required |
| `render_google` | `dialect.input_blocks().contains` | [1237](../../src/request.rs#L1237), [1239](../../src/request.rs#L1239) | receiver-type-required |
| `render_google` | `dialect.input_blocks` | [1237](../../src/request.rs#L1237), [1239](../../src/request.rs#L1239) | receiver-type-required |
| `render_google` | `block.get("data").and_then` | [1243](../../src/request.rs#L1243) | receiver-type-required |
| `render_google` | `block.get` | [1243](../../src/request.rs#L1243) | receiver-type-required |
| `render_google` | `merged.last().is_some_and` | [1272](../../src/request.rs#L1272) | receiver-type-required |
| `render_google` | `merged.last` | [1272](../../src/request.rs#L1272) | receiver-type-required |
| `render_google` | `merged.last_mut().unwrap()["parts"]                 .as_array_mut()                 .unwrap()                 .extend` | [1273](../../src/request.rs#L1273) | receiver-type-required |
| `render_google` | `merged.last_mut().unwrap()["parts"]                 .as_array_mut()                 .unwrap` | [1273](../../src/request.rs#L1273) | receiver-type-required |
| `render_google` | `merged.last_mut().unwrap()["parts"]                 .as_array_mut` | [1273](../../src/request.rs#L1273) | receiver-type-required |
| `render_google` | `merged.last_mut().unwrap` | [1273](../../src/request.rs#L1273) | receiver-type-required |
| `render_google` | `merged.last_mut` | [1273](../../src/request.rs#L1273) | receiver-type-required |
| `render_google` | `entry["parts"].as_array().unwrap().iter().cloned` | [1276](../../src/request.rs#L1276) | receiver-type-required |
| `render_google` | `entry["parts"].as_array().unwrap().iter` | [1276](../../src/request.rs#L1276) | receiver-type-required |
| `render_google` | `entry["parts"].as_array().unwrap` | [1276](../../src/request.rs#L1276) | receiver-type-required |
| `render_google` | `entry["parts"].as_array` | [1276](../../src/request.rs#L1276) | receiver-type-required |
| `render_google` | `merged.push` | [1278](../../src/request.rs#L1278) | receiver-type-required |
| `render_google` | `entry["parts"]                 .as_array_mut()                 .unwrap()                 .sort_by_key` | [1283](../../src/request.rs#L1283) | receiver-type-required |
| `render_google` | `entry["parts"]                 .as_array_mut()                 .unwrap` | [1283](../../src/request.rs#L1283) | receiver-type-required |
| `render_google` | `entry["parts"]                 .as_array_mut` | [1283](../../src/request.rs#L1283) | receiver-type-required |
| `render_google` | `part.get("functionResponse").is_none` | [1286](../../src/request.rs#L1286) | receiver-type-required |
| `render_google` | `part.get` | [1286](../../src/request.rs#L1286) | receiver-type-required |
| `render_google` | `Ok` | [1289](../../src/request.rs#L1289) | external-constructor-callback-or-unresolved |
| `google_function_response` | `result.is_object` | [1293](../../src/request.rs#L1293) | receiver-type-required |
| `render_interactions` | `Vec::new` | [1314](../../src/request.rs#L1314) | external-constructor-callback-or-unresolved |
| `render_interactions` | `sealed_fragments` | [1316](../../src/request.rs#L1316) | [provider::request::sealed_fragments](../../src/request.rs#L1859) |
| `render_interactions` | `steps.extend` | [1317](../../src/request.rs#L1317) | receiver-type-required |
| `render_interactions` | `item_role` | [1320](../../src/request.rs#L1320) | [provider::request::item_role](../../src/request.rs#L1852) |
| `render_interactions` | `item_content` | [1321](../../src/request.rs#L1321) | [provider::request::item_content](../../src/request.rs#L1929) |
| `render_interactions` | `block_type` | [1322](../../src/request.rs#L1322) | [provider::request::block_type](../../src/request.rs#L1936) |
| `render_interactions` | `steps.push` | [1325](../../src/request.rs#L1325), [1329](../../src/request.rs#L1329), [1333](../../src/request.rs#L1333), [1361](../../src/request.rs#L1361), [1366](../../src/request.rs#L1366), [1372](../../src/request.rs#L1372), [1380](../../src/request.rs#L1380) | receiver-type-required |
| `render_interactions` | `dialect.supports_reasoning_blocks` | [1333](../../src/request.rs#L1333) | receiver-type-required |
| `render_interactions` | `Err` | [1338](../../src/request.rs#L1338), [1389](../../src/request.rs#L1389), [1405](../../src/request.rs#L1405) | external-constructor-callback-or-unresolved |
| `render_interactions` | `invalid` | [1338](../../src/request.rs#L1338), [1389](../../src/request.rs#L1389), [1405](../../src/request.rs#L1405) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_interactions` | `required_text` | [1344](../../src/request.rs#L1344), [1397](../../src/request.rs#L1397), [1402](../../src/request.rs#L1402) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_interactions` | `mime.starts_with` | [1345](../../src/request.rs#L1345) | receiver-type-required |
| `render_interactions` | `dialect.input_blocks().contains` | [1346](../../src/request.rs#L1346), [1349](../../src/request.rs#L1349) | receiver-type-required |
| `render_interactions` | `dialect.input_blocks` | [1346](../../src/request.rs#L1346), [1349](../../src/request.rs#L1349) | receiver-type-required |
| `render_interactions` | `Some` | [1348](../../src/request.rs#L1348), [1351](../../src/request.rs#L1351) | external-constructor-callback-or-unresolved |
| `render_interactions` | `block.get("data").and_then` | [1356](../../src/request.rs#L1356) | receiver-type-required |
| `render_interactions` | `block.get` | [1356](../../src/request.rs#L1356) | receiver-type-required |
| `render_interactions` | `steps             .iter()             .filter(&#124;step&#124; step["type"] == "function_call")             .map(&#124;step&#124; required_text(step, "id"))             .collect::<Result<std::collections::BTreeSet<_>, _>>` | [1394](../../src/request.rs#L1394) | receiver-type-required |
| `render_interactions` | `steps             .iter()             .filter(&#124;step&#124; step["type"] == "function_call")             .map` | [1394](../../src/request.rs#L1394) | receiver-type-required |
| `render_interactions` | `steps             .iter()             .filter` | [1394](../../src/request.rs#L1394), [1399](../../src/request.rs#L1399) | receiver-type-required |
| `render_interactions` | `steps             .iter` | [1394](../../src/request.rs#L1394), [1399](../../src/request.rs#L1399) | receiver-type-required |
| `render_interactions` | `steps             .iter()             .filter(&#124;step&#124; step["type"] == "function_result")             .map(&#124;step&#124; required_text(step, "call_id"))             .collect::<Result<std::collections::BTreeSet<_>, _>>` | [1399](../../src/request.rs#L1399) | receiver-type-required |
| `render_interactions` | `steps             .iter()             .filter(&#124;step&#124; step["type"] == "function_result")             .map` | [1399](../../src/request.rs#L1399) | receiver-type-required |
| `render_interactions` | `step["type"].as_str` | [1410](../../src/request.rs#L1410) | receiver-type-required |
| `render_interactions` | `Ok` | [1431](../../src/request.rs#L1431) | external-constructor-callback-or-unresolved |
| `render_tools` | `dialect.family` | [1435](../../src/request.rs#L1435) | receiver-type-required |
| `render_tools` | `tools         .as_array()         .ok_or_else` | [1436](../../src/request.rs#L1436) | receiver-type-required |
| `render_tools` | `tools         .as_array` | [1436](../../src/request.rs#L1436) | receiver-type-required |
| `render_tools` | `invalid` | [1438](../../src/request.rs#L1438) | [provider::request::invalid](../../src/request.rs#L1979) |
| `render_tools` | `Vec::new` | [1439](../../src/request.rs#L1439) | external-constructor-callback-or-unresolved |
| `render_tools` | `required_text` | [1441](../../src/request.rs#L1441), [1442](../../src/request.rs#L1442) | [provider::request::required_text](../../src/request.rs#L1943) |
| `render_tools` | `normalize_tool_parameters` | [1443](../../src/request.rs#L1443) | [provider::request::normalize_tool_parameters](../../src/request.rs#L1484) |
| `render_tools` | `required_value` | [1443](../../src/request.rs#L1443) | [provider::request::required_value](../../src/request.rs#L1952) |
| `render_tools` | `kimi_nullable_constraints` | [1445](../../src/request.rs#L1445) | [provider::request::kimi_nullable_constraints](../../src/request.rs#L1579) |
| `render_tools` | `validate_tool_schema` | [1449](../../src/request.rs#L1449) | [provider::request::validate_tool_schema](../../src/request.rs#L1809) |
| `render_tools` | `contains_composition_constraints` | [1453](../../src/request.rs#L1453) | [provider::request::contains_composition_constraints](../../src/request.rs#L1696) |
| `render_tools` | `contains_non_strict_object` | [1454](../../src/request.rs#L1454) | [provider::request::contains_non_strict_object](../../src/request.rs#L1711) |
| `render_tools` | `rendered.push` | [1455](../../src/request.rs#L1455) | receiver-type-required |
| `render_tools` | `anthropic_strict_subset` | [1461](../../src/request.rs#L1461) | [provider::request::anthropic_strict_subset](../../src/request.rs#L1513) |
| `render_tools` | `Ok` | [1472](../../src/request.rs#L1472) | external-constructor-callback-or-unresolved |
| `render_tools` | `rendered.is_empty` | [1473](../../src/request.rs#L1473) | receiver-type-required |
| `render_tools` | `Value::Array` | [1476](../../src/request.rs#L1476) | external-constructor-callback-or-unresolved |
| `normalize_tool_parameters` | `schema.remove` | [1488](../../src/request.rs#L1488), [1489](../../src/request.rs#L1489) | receiver-type-required |
| `normalize_tool_parameters` | `schema.entry("properties").or_insert_with` | [1490](../../src/request.rs#L1490) | receiver-type-required |
| `normalize_tool_parameters` | `schema.entry` | [1490](../../src/request.rs#L1490), [1491](../../src/request.rs#L1491) | receiver-type-required |
| `normalize_tool_parameters` | `schema.entry("required").or_insert_with` | [1491](../../src/request.rs#L1491) | receiver-type-required |
| `normalize_tool_parameters` | `Value::Object` | [1492](../../src/request.rs#L1492) | external-constructor-callback-or-unresolved |
| `anthropic_strict_subset` | `items.iter().all` | [1515](../../src/request.rs#L1515) | receiver-type-required |
| `anthropic_strict_subset` | `items.iter` | [1515](../../src/request.rs#L1515) | receiver-type-required |
| `anthropic_strict_subset` | `object                 .keys()                 .any` | [1517](../../src/request.rs#L1517) | receiver-type-required |
| `anthropic_strict_subset` | `object                 .keys` | [1517](../../src/request.rs#L1517) | receiver-type-required |
| `anthropic_strict_subset` | `ANTHROPIC_STRICT_UNSUPPORTED.contains` | [1519](../../src/request.rs#L1519) | receiver-type-required |
| `anthropic_strict_subset` | `key.as_str` | [1519](../../src/request.rs#L1519), [1523](../../src/request.rs#L1523) | receiver-type-required |
| `anthropic_strict_subset` | `object.iter().all` | [1523](../../src/request.rs#L1523) | receiver-type-required |
| `anthropic_strict_subset` | `object.iter` | [1523](../../src/request.rs#L1523) | receiver-type-required |
| `anthropic_strict_subset` | `value                     .as_object()                     .is_none_or` | [1526](../../src/request.rs#L1526) | receiver-type-required |
| `anthropic_strict_subset` | `value                     .as_object` | [1526](../../src/request.rs#L1526) | receiver-type-required |
| `anthropic_strict_subset` | `members.values().all` | [1528](../../src/request.rs#L1528) | receiver-type-required |
| `anthropic_strict_subset` | `members.values` | [1528](../../src/request.rs#L1528) | receiver-type-required |
| `anthropic_strict_subset` | `anthropic_strict_subset` | [1529](../../src/request.rs#L1529) | [provider::request::anthropic_strict_subset](../../src/request.rs#L1513) |
| `anthropic_root_conditional` | `schema.as_object_mut` | [1541](../../src/request.rs#L1541) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get("allOf")         .and_then(Value::as_array)         .filter(&#124;branches&#124; branches.len() == 1)         .and_then(&#124;branches&#124; branches[0].as_object())         .filter(&#124;branch&#124; {             branch.contains_key("if")                 && branch.contains_key("then")                 && branch                     .keys()                     .all(&#124;key&#124; matches!(key.as_str(), "if" &#124; "then" &#124; "else"))                 && ["if", "then", "else"]                     .iter()                     .all(&#124;key&#124; !root.contains_key(*key))         })         .cloned` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get("allOf")         .and_then(Value::as_array)         .filter(&#124;branches&#124; branches.len() == 1)         .and_then(&#124;branches&#124; branches[0].as_object())         .filter` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get("allOf")         .and_then(Value::as_array)         .filter(&#124;branches&#124; branches.len() == 1)         .and_then` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get("allOf")         .and_then(Value::as_array)         .filter` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get("allOf")         .and_then` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `root         .get` | [1544](../../src/request.rs#L1544) | receiver-type-required |
| `anthropic_root_conditional` | `branches.len` | [1547](../../src/request.rs#L1547) | receiver-type-required |
| `anthropic_root_conditional` | `branches[0].as_object` | [1548](../../src/request.rs#L1548) | receiver-type-required |
| `anthropic_root_conditional` | `branch.contains_key` | [1550](../../src/request.rs#L1550), [1551](../../src/request.rs#L1551) | receiver-type-required |
| `anthropic_root_conditional` | `branch                     .keys()                     .all` | [1552](../../src/request.rs#L1552) | receiver-type-required |
| `anthropic_root_conditional` | `branch                     .keys` | [1552](../../src/request.rs#L1552) | receiver-type-required |
| `anthropic_root_conditional` | `["if", "then", "else"]                     .iter()                     .all` | [1555](../../src/request.rs#L1555) | receiver-type-required |
| `anthropic_root_conditional` | `["if", "then", "else"]                     .iter` | [1555](../../src/request.rs#L1555) | receiver-type-required |
| `anthropic_root_conditional` | `root.contains_key` | [1557](../../src/request.rs#L1557), [1565](../../src/request.rs#L1565) | receiver-type-required |
| `anthropic_root_conditional` | `root.remove` | [1561](../../src/request.rs#L1561), [1569](../../src/request.rs#L1569) | receiver-type-required |
| `anthropic_root_conditional` | `root.extend` | [1562](../../src/request.rs#L1562) | receiver-type-required |
| `anthropic_root_conditional` | `["allOf", "oneOf", "anyOf"]         .iter()         .any` | [1563](../../src/request.rs#L1563) | receiver-type-required |
| `anthropic_root_conditional` | `["allOf", "oneOf", "anyOf"]         .iter` | [1563](../../src/request.rs#L1563) | receiver-type-required |
| `anthropic_root_conditional` | `serde_json::Map::new` | [1567](../../src/request.rs#L1567) | external-constructor-callback-or-unresolved |
| `anthropic_root_conditional` | `constraints.insert` | [1570](../../src/request.rs#L1570) | receiver-type-required |
| `anthropic_root_conditional` | `key.to_owned` | [1570](../../src/request.rs#L1570) | receiver-type-required |
| `anthropic_root_conditional` | `root.insert` | [1573](../../src/request.rs#L1573), [1574](../../src/request.rs#L1574) | receiver-type-required |
| `anthropic_root_conditional` | `"if".to_owned` | [1573](../../src/request.rs#L1573) | receiver-type-required |
| `anthropic_root_conditional` | `"then".to_owned` | [1574](../../src/request.rs#L1574) | receiver-type-required |
| `anthropic_root_conditional` | `Value::Object` | [1574](../../src/request.rs#L1574) | external-constructor-callback-or-unresolved |
| `kimi_nullable_constraints` | `kimi_nullable_constraints` | [1583](../../src/request.rs#L1583), [1588](../../src/request.rs#L1588) | [provider::request::kimi_nullable_constraints](../../src/request.rs#L1579) |
| `kimi_nullable_constraints` | `item.take` | [1583](../../src/request.rs#L1583) | receiver-type-required |
| `kimi_nullable_constraints` | `object.values_mut` | [1587](../../src/request.rs#L1587) | receiver-type-required |
| `kimi_nullable_constraints` | `child.take` | [1588](../../src/request.rs#L1588) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get("type")                 .and_then(Value::as_array)                 .filter(&#124;types&#124; types.len() == 2 && types.iter().any(&#124;t&#124; t == "null"))                 .and_then(&#124;types&#124; types.iter().find(&#124;t&#124; *t != "null"))                 .and_then(Value::as_str)                 .map` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get("type")                 .and_then(Value::as_array)                 .filter(&#124;types&#124; types.len() == 2 && types.iter().any(&#124;t&#124; t == "null"))                 .and_then(&#124;types&#124; types.iter().find(&#124;t&#124; *t != "null"))                 .and_then` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get("type")                 .and_then(Value::as_array)                 .filter(&#124;types&#124; types.len() == 2 && types.iter().any(&#124;t&#124; t == "null"))                 .and_then` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get("type")                 .and_then(Value::as_array)                 .filter` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get("type")                 .and_then` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `object                 .get` | [1590](../../src/request.rs#L1590) | receiver-type-required |
| `kimi_nullable_constraints` | `types.len` | [1593](../../src/request.rs#L1593) | receiver-type-required |
| `kimi_nullable_constraints` | `types.iter().any` | [1593](../../src/request.rs#L1593) | receiver-type-required |
| `kimi_nullable_constraints` | `types.iter` | [1593](../../src/request.rs#L1593), [1594](../../src/request.rs#L1594) | receiver-type-required |
| `kimi_nullable_constraints` | `types.iter().find` | [1594](../../src/request.rs#L1594) | receiver-type-required |
| `kimi_nullable_constraints` | `concrete.as_deref` | [1597](../../src/request.rs#L1597) | receiver-type-required |
| `kimi_nullable_constraints` | `keys.iter().any` | [1627](../../src/request.rs#L1627) | receiver-type-required |
| `kimi_nullable_constraints` | `keys.iter` | [1627](../../src/request.rs#L1627) | receiver-type-required |
| `kimi_nullable_constraints` | `object.contains_key` | [1627](../../src/request.rs#L1627), [1628](../../src/request.rs#L1628) | receiver-type-required |
| `kimi_nullable_constraints` | `Err` | [1629](../../src/request.rs#L1629) | external-constructor-callback-or-unresolved |
| `kimi_nullable_constraints` | `invalid` | [1629](../../src/request.rs#L1629) | [provider::request::invalid](../../src/request.rs#L1979) |
| `kimi_nullable_constraints` | `serde_json::Map::new` | [1633](../../src/request.rs#L1633) | external-constructor-callback-or-unresolved |
| `kimi_nullable_constraints` | `branch.insert` | [1634](../../src/request.rs#L1634), [1637](../../src/request.rs#L1637) | receiver-type-required |
| `kimi_nullable_constraints` | `"type".into` | [1634](../../src/request.rs#L1634) | receiver-type-required |
| `kimi_nullable_constraints` | `object.remove` | [1636](../../src/request.rs#L1636), [1640](../../src/request.rs#L1640) | receiver-type-required |
| `kimi_nullable_constraints` | `(*key).into` | [1637](../../src/request.rs#L1637) | receiver-type-required |
| `kimi_nullable_constraints` | `object.insert` | [1641](../../src/request.rs#L1641) | receiver-type-required |
| `kimi_nullable_constraints` | `"anyOf".into` | [1641](../../src/request.rs#L1641) | receiver-type-required |
| `kimi_nullable_constraints` | `Ok` | [1646](../../src/request.rs#L1646) | external-constructor-callback-or-unresolved |
| `kimi_nullable_bounds_preserve_null_enum_and_reject_composition_collision` | `kimi_nullable_constraints(input).unwrap` | [1656](../../src/request.rs#L1656) | receiver-type-required |
| `kimi_nullable_bounds_preserve_null_enum_and_reject_composition_collision` | `kimi_nullable_constraints` | [1656](../../src/request.rs#L1656) | external-constructor-callback-or-unresolved |
| `only_isolated_single_conditional_is_hoisted_without_losing_constraints` | `anthropic_root_conditional` | [1676](../../src/request.rs#L1676), [1690](../../src/request.rs#L1690) | external-constructor-callback-or-unresolved |
| `only_isolated_single_conditional_is_hoisted_without_losing_constraints` | `schema.clone` | [1676](../../src/request.rs#L1676) | receiver-type-required |
| `only_isolated_single_conditional_is_hoisted_without_losing_constraints` | `unsafe_schema.clone` | [1690](../../src/request.rs#L1690) | receiver-type-required |
| `contains_composition_constraints` | `object.iter().any` | [1698](../../src/request.rs#L1698) | receiver-type-required |
| `contains_composition_constraints` | `object.iter` | [1698](../../src/request.rs#L1698) | receiver-type-required |
| `contains_composition_constraints` | `contains_composition_constraints` | [1702](../../src/request.rs#L1702) | [provider::request::contains_composition_constraints](../../src/request.rs#L1696) |
| `contains_composition_constraints` | `values.iter().any` | [1704](../../src/request.rs#L1704) | receiver-type-required |
| `contains_composition_constraints` | `values.iter` | [1704](../../src/request.rs#L1704) | receiver-type-required |
| `contains_non_strict_object` | `object.get("type").and_then` | [1714](../../src/request.rs#L1714) | receiver-type-required |
| `contains_non_strict_object` | `object.get` | [1714](../../src/request.rs#L1714), [1717](../../src/request.rs#L1717), [1718](../../src/request.rs#L1718), [1719](../../src/request.rs#L1719) | receiver-type-required |
| `contains_non_strict_object` | `Some` | [1714](../../src/request.rs#L1714), [1719](../../src/request.rs#L1719), [1727](../../src/request.rs#L1727) | external-constructor-callback-or-unresolved |
| `contains_non_strict_object` | `object.contains_key` | [1715](../../src/request.rs#L1715) | receiver-type-required |
| `contains_non_strict_object` | `object.get("properties").and_then` | [1717](../../src/request.rs#L1717) | receiver-type-required |
| `contains_non_strict_object` | `object.get("required").and_then` | [1718](../../src/request.rs#L1718) | receiver-type-required |
| `contains_non_strict_object` | `Value::Bool` | [1719](../../src/request.rs#L1719) | external-constructor-callback-or-unresolved |
| `contains_non_strict_object` | `properties                         .zip(required)                         .is_none_or` | [1720](../../src/request.rs#L1720) | receiver-type-required |
| `contains_non_strict_object` | `properties                         .zip` | [1720](../../src/request.rs#L1720) | receiver-type-required |
| `contains_non_strict_object` | `properties.len` | [1723](../../src/request.rs#L1723) | receiver-type-required |
| `contains_non_strict_object` | `required.len` | [1723](../../src/request.rs#L1723) | receiver-type-required |
| `contains_non_strict_object` | `properties.keys().any` | [1724](../../src/request.rs#L1724) | receiver-type-required |
| `contains_non_strict_object` | `properties.keys` | [1724](../../src/request.rs#L1724) | receiver-type-required |
| `contains_non_strict_object` | `required                                         .iter()                                         .any` | [1725](../../src/request.rs#L1725) | receiver-type-required |
| `contains_non_strict_object` | `required                                         .iter` | [1725](../../src/request.rs#L1725) | receiver-type-required |
| `contains_non_strict_object` | `value.as_str` | [1727](../../src/request.rs#L1727) | receiver-type-required |
| `contains_non_strict_object` | `key.as_str` | [1727](../../src/request.rs#L1727) | receiver-type-required |
| `contains_non_strict_object` | `object.values().any` | [1734](../../src/request.rs#L1734) | receiver-type-required |
| `contains_non_strict_object` | `object.values` | [1734](../../src/request.rs#L1734) | receiver-type-required |
| `contains_non_strict_object` | `values.iter().any` | [1736](../../src/request.rs#L1736) | receiver-type-required |
| `contains_non_strict_object` | `values.iter` | [1736](../../src/request.rs#L1736) | receiver-type-required |
| `mcp_schema_metadata_and_omitted_empty_required_render_without_losing_constraints` | `render_tools(             DialectId::OpenaiResponsesV1,             &json!([{"name":"mcp__fixture__get_capabilities","description":"Inspect",                 "parameters":schema}]),         )         .expect` | [1753](../../src/request.rs#L1753) | receiver-type-required |
| `mcp_schema_metadata_and_omitted_empty_required_render_without_losing_constraints` | `render_tools` | [1753](../../src/request.rs#L1753) | external-constructor-callback-or-unresolved |
| `unique_items_in_nullable_array_preserves_constraints_without_strict_mode` | `render_tools(                 dialect,                 &json!([{"name":"ask_user_questions","description":"ask","parameters":parameters}]),             )             .unwrap` | [1772](../../src/request.rs#L1772) | receiver-type-required |
| `unique_items_in_nullable_array_preserves_constraints_without_strict_mode` | `render_tools` | [1772](../../src/request.rs#L1772) | external-constructor-callback-or-unresolved |
| `unique_items_in_nullable_array_preserves_constraints_without_strict_mode` | `rendered[0].get("function").unwrap_or` | [1777](../../src/request.rs#L1777) | receiver-type-required |
| `unique_items_in_nullable_array_preserves_constraints_without_strict_mode` | `rendered[0].get` | [1777](../../src/request.rs#L1777) | receiver-type-required |
| `optional_fields_keep_their_schema_and_disable_strict_mode` | `render_tools(                 dialect,                 &json!([{"name":"context_get","description":"read", "parameters":optional}]),             )             .unwrap` | [1789](../../src/request.rs#L1789) | receiver-type-required |
| `optional_fields_keep_their_schema_and_disable_strict_mode` | `render_tools` | [1789](../../src/request.rs#L1789) | external-constructor-callback-or-unresolved |
| `optional_fields_keep_their_schema_and_disable_strict_mode` | `rendered[0].get("function").unwrap_or` | [1794](../../src/request.rs#L1794) | receiver-type-required |
| `optional_fields_keep_their_schema_and_disable_strict_mode` | `rendered[0].get` | [1794](../../src/request.rs#L1794) | receiver-type-required |
| `optional_fields_keep_their_schema_and_disable_strict_mode` | `optional.clone` | [1801](../../src/request.rs#L1801) | receiver-type-required |
| `validate_tool_schema` | `schema         .as_object()         .ok_or_else` | [1810](../../src/request.rs#L1810) | receiver-type-required |
| `validate_tool_schema` | `schema         .as_object` | [1810](../../src/request.rs#L1810) | receiver-type-required |
| `validate_tool_schema` | `invalid` | [1812](../../src/request.rs#L1812), [1814](../../src/request.rs#L1814), [1819](../../src/request.rs#L1819), [1823](../../src/request.rs#L1823), [1828](../../src/request.rs#L1828), [1838](../../src/request.rs#L1838), [1845](../../src/request.rs#L1845) | [provider::request::invalid](../../src/request.rs#L1979) |
| `validate_tool_schema` | `object.get("type").and_then` | [1813](../../src/request.rs#L1813) | receiver-type-required |
| `validate_tool_schema` | `object.get` | [1813](../../src/request.rs#L1813), [1840](../../src/request.rs#L1840) | receiver-type-required |
| `validate_tool_schema` | `Some` | [1813](../../src/request.rs#L1813) | external-constructor-callback-or-unresolved |
| `validate_tool_schema` | `Err` | [1814](../../src/request.rs#L1814), [1828](../../src/request.rs#L1828), [1838](../../src/request.rs#L1838), [1845](../../src/request.rs#L1845) | external-constructor-callback-or-unresolved |
| `validate_tool_schema` | `object         .get("properties")         .and_then(Value::as_object)         .ok_or_else` | [1816](../../src/request.rs#L1816) | receiver-type-required |
| `validate_tool_schema` | `object         .get("properties")         .and_then` | [1816](../../src/request.rs#L1816) | receiver-type-required |
| `validate_tool_schema` | `object         .get` | [1816](../../src/request.rs#L1816), [1820](../../src/request.rs#L1820) | receiver-type-required |
| `validate_tool_schema` | `object         .get("required")         .and_then(Value::as_array)         .ok_or_else` | [1820](../../src/request.rs#L1820) | receiver-type-required |
| `validate_tool_schema` | `object         .get("required")         .and_then` | [1820](../../src/request.rs#L1820) | receiver-type-required |
| `validate_tool_schema` | `required.iter().any` | [1824](../../src/request.rs#L1824) | receiver-type-required |
| `validate_tool_schema` | `required.iter` | [1824](../../src/request.rs#L1824) | receiver-type-required |
| `validate_tool_schema` | `name.as_str()             .is_none_or` | [1825](../../src/request.rs#L1825) | receiver-type-required |
| `validate_tool_schema` | `name.as_str` | [1825](../../src/request.rs#L1825) | receiver-type-required |
| `validate_tool_schema` | `name.is_empty` | [1826](../../src/request.rs#L1826) | receiver-type-required |
| `validate_tool_schema` | `properties.contains_key` | [1826](../../src/request.rs#L1826) | receiver-type-required |
| `validate_tool_schema` | `object.keys().any` | [1832](../../src/request.rs#L1832) | receiver-type-required |
| `validate_tool_schema` | `object.keys` | [1832](../../src/request.rs#L1832) | receiver-type-required |
| `validate_tool_schema` | `rules             .as_array()             .is_some_and` | [1841](../../src/request.rs#L1841) | receiver-type-required |
| `validate_tool_schema` | `rules             .as_array` | [1841](../../src/request.rs#L1841) | receiver-type-required |
| `validate_tool_schema` | `rules.is_empty` | [1843](../../src/request.rs#L1843) | receiver-type-required |
| `validate_tool_schema` | `rules.iter().all` | [1843](../../src/request.rs#L1843) | receiver-type-required |
| `validate_tool_schema` | `rules.iter` | [1843](../../src/request.rs#L1843) | receiver-type-required |
| `validate_tool_schema` | `Ok` | [1849](../../src/request.rs#L1849) | external-constructor-callback-or-unresolved |
| `item_role` | `item.get("role")         .and_then(Value::as_str)         .filter(&#124;role&#124; matches!(*role, "user" &#124; "assistant" &#124; "tool"))         .ok_or_else` | [1853](../../src/request.rs#L1853) | receiver-type-required |
| `item_role` | `item.get("role")         .and_then(Value::as_str)         .filter` | [1853](../../src/request.rs#L1853) | receiver-type-required |
| `item_role` | `item.get("role")         .and_then` | [1853](../../src/request.rs#L1853) | receiver-type-required |
| `item_role` | `item.get` | [1853](../../src/request.rs#L1853) | receiver-type-required |
| `item_role` | `invalid` | [1856](../../src/request.rs#L1856) | [provider::request::invalid](../../src/request.rs#L1979) |
| `sealed_fragments` | `sealed_fragment_value` | [1860](../../src/request.rs#L1860) | [provider::request::sealed_fragment_value](../../src/request.rs#L1870) |
| `sealed_fragments` | `Ok` | [1861](../../src/request.rs#L1861) | external-constructor-callback-or-unresolved |
| `sealed_fragments` | `value         .as_array()         .cloned()         .map(Some)         .ok_or_else` | [1863](../../src/request.rs#L1863) | receiver-type-required |
| `sealed_fragments` | `value         .as_array()         .cloned()         .map` | [1863](../../src/request.rs#L1863) | receiver-type-required |
| `sealed_fragments` | `value         .as_array()         .cloned` | [1863](../../src/request.rs#L1863) | receiver-type-required |
| `sealed_fragments` | `value         .as_array` | [1863](../../src/request.rs#L1863) | receiver-type-required |
| `sealed_fragments` | `invalid` | [1867](../../src/request.rs#L1867) | [provider::request::invalid](../../src/request.rs#L1979) |
| `sealed_fragment_value` | `item.get("role").and_then` | [1871](../../src/request.rs#L1871) | receiver-type-required |
| `sealed_fragment_value` | `item.get` | [1871](../../src/request.rs#L1871), [1874](../../src/request.rs#L1874), [1877](../../src/request.rs#L1877) | receiver-type-required |
| `sealed_fragment_value` | `Some` | [1871](../../src/request.rs#L1871), [1874](../../src/request.rs#L1874) | external-constructor-callback-or-unresolved |
| `sealed_fragment_value` | `Ok` | [1872](../../src/request.rs#L1872) | external-constructor-callback-or-unresolved |
| `sealed_fragment_value` | `item.get("adapter").and_then` | [1874](../../src/request.rs#L1874) | receiver-type-required |
| `sealed_fragment_value` | `dialect.as_str` | [1874](../../src/request.rs#L1874) | receiver-type-required |
| `sealed_fragment_value` | `Err` | [1875](../../src/request.rs#L1875) | external-constructor-callback-or-unresolved |
| `sealed_fragment_value` | `invalid` | [1875](../../src/request.rs#L1875), [1880](../../src/request.rs#L1880) | [provider::request::invalid](../../src/request.rs#L1979) |
| `sealed_fragment_value` | `item.get("fragments")         .cloned()         .map(Some)         .ok_or_else` | [1877](../../src/request.rs#L1877) | receiver-type-required |
| `sealed_fragment_value` | `item.get("fragments")         .cloned()         .map` | [1877](../../src/request.rs#L1877) | receiver-type-required |
| `sealed_fragment_value` | `item.get("fragments")         .cloned` | [1877](../../src/request.rs#L1877) | receiver-type-required |
| `degraded_file_text` | `required_text` | [1894](../../src/request.rs#L1894), [1901](../../src/request.rs#L1901), [1903](../../src/request.rs#L1903) | [provider::request::required_text](../../src/request.rs#L1943) |
| `degraded_file_text` | `mime.starts_with` | [1895](../../src/request.rs#L1895) | receiver-type-required |
| `degraded_file_text` | `Err` | [1896](../../src/request.rs#L1896) | external-constructor-callback-or-unresolved |
| `degraded_file_text` | `invalid` | [1896](../../src/request.rs#L1896), [1904](../../src/request.rs#L1904) | [provider::request::invalid](../../src/request.rs#L1979) |
| `degraded_file_text` | `required_text(block, "name").unwrap_or_else` | [1901](../../src/request.rs#L1901) | receiver-type-required |
| `degraded_file_text` | `"attachment".to_owned` | [1901](../../src/request.rs#L1901) | receiver-type-required |
| `degraded_file_text` | `base64::engine::general_purpose::STANDARD         .decode(required_text(block, "data")?)         .map_err` | [1902](../../src/request.rs#L1902) | receiver-type-required |
| `degraded_file_text` | `base64::engine::general_purpose::STANDARD         .decode` | [1902](../../src/request.rs#L1902) | receiver-type-required |
| `degraded_file_text` | `block         .get("bytes")         .and_then(Value::as_u64)         .unwrap_or` | [1905](../../src/request.rs#L1905) | receiver-type-required |
| `degraded_file_text` | `block         .get("bytes")         .and_then` | [1905](../../src/request.rs#L1905) | receiver-type-required |
| `degraded_file_text` | `block         .get` | [1905](../../src/request.rs#L1905) | receiver-type-required |
| `degraded_file_text` | `decoded.len` | [1908](../../src/request.rs#L1908), [1909](../../src/request.rs#L1909) | receiver-type-required |
| `degraded_file_text` | `std::str::from_utf8` | [1910](../../src/request.rs#L1910) | external-constructor-callback-or-unresolved |
| `degraded_file_text` | `Ok` | [1912](../../src/request.rs#L1912), [1916](../../src/request.rs#L1916) | external-constructor-callback-or-unresolved |
| `data_url` | `Ok` | [1922](../../src/request.rs#L1922) | external-constructor-callback-or-unresolved |
| `item_content` | `item.get("content")         .and_then(Value::as_array)         .map(Vec::as_slice)         .ok_or_else` | [1930](../../src/request.rs#L1930) | receiver-type-required |
| `item_content` | `item.get("content")         .and_then(Value::as_array)         .map` | [1930](../../src/request.rs#L1930) | receiver-type-required |
| `item_content` | `item.get("content")         .and_then` | [1930](../../src/request.rs#L1930) | receiver-type-required |
| `item_content` | `item.get` | [1930](../../src/request.rs#L1930) | receiver-type-required |
| `item_content` | `invalid` | [1933](../../src/request.rs#L1933) | [provider::request::invalid](../../src/request.rs#L1979) |
| `block_type` | `block         .get("type")         .and_then(Value::as_str)         .ok_or_else` | [1937](../../src/request.rs#L1937) | receiver-type-required |
| `block_type` | `block         .get("type")         .and_then` | [1937](../../src/request.rs#L1937) | receiver-type-required |
| `block_type` | `block         .get` | [1937](../../src/request.rs#L1937) | receiver-type-required |
| `block_type` | `invalid` | [1940](../../src/request.rs#L1940) | [provider::request::invalid](../../src/request.rs#L1979) |
| `required_text` | `value         .get(field)         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .map(str::to_owned)         .ok_or_else` | [1944](../../src/request.rs#L1944) | receiver-type-required |
| `required_text` | `value         .get(field)         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .map` | [1944](../../src/request.rs#L1944) | receiver-type-required |
| `required_text` | `value         .get(field)         .and_then(Value::as_str)         .filter` | [1944](../../src/request.rs#L1944) | receiver-type-required |
| `required_text` | `value         .get(field)         .and_then` | [1944](../../src/request.rs#L1944) | receiver-type-required |
| `required_text` | `value         .get` | [1944](../../src/request.rs#L1944) | receiver-type-required |
| `required_text` | `value.is_empty` | [1947](../../src/request.rs#L1947) | receiver-type-required |
| `required_text` | `invalid` | [1949](../../src/request.rs#L1949) | [provider::request::invalid](../../src/request.rs#L1979) |
| `required_value` | `value         .get(field)         .cloned()         .ok_or_else` | [1953](../../src/request.rs#L1953) | receiver-type-required |
| `required_value` | `value         .get(field)         .cloned` | [1953](../../src/request.rs#L1953) | receiver-type-required |
| `required_value` | `value         .get` | [1953](../../src/request.rs#L1953) | receiver-type-required |
| `required_value` | `invalid` | [1956](../../src/request.rs#L1956) | [provider::request::invalid](../../src/request.rs#L1979) |
| `tool_result_text` | `block.get` | [1963](../../src/request.rs#L1963) | receiver-type-required |
| `tool_result_text` | `result.and_then(Value::as_array).map` | [1964](../../src/request.rs#L1964) | receiver-type-required |
| `tool_result_text` | `result.and_then` | [1964](../../src/request.rs#L1964) | receiver-type-required |
| `tool_result_text` | `only.get("type").and_then` | [1965](../../src/request.rs#L1965) | receiver-type-required |
| `tool_result_text` | `only.get` | [1965](../../src/request.rs#L1965), [1966](../../src/request.rs#L1966) | receiver-type-required |
| `tool_result_text` | `Some` | [1965](../../src/request.rs#L1965) | external-constructor-callback-or-unresolved |
| `tool_result_text` | `only.get("text").and_then` | [1966](../../src/request.rs#L1966) | receiver-type-required |
| `tool_result_text` | `Ok` | [1968](../../src/request.rs#L1968) | external-constructor-callback-or-unresolved |
| `tool_result_text` | `text.to_owned` | [1968](../../src/request.rs#L1968) | receiver-type-required |
| `tool_result_text` | `canonical_argument_string` | [1970](../../src/request.rs#L1970) | [provider::request::canonical_argument_string](../../src/request.rs#L1973) |
| `canonical_argument_string` | `value.ok_or_else` | [1974](../../src/request.rs#L1974) | receiver-type-required |
| `canonical_argument_string` | `invalid` | [1974](../../src/request.rs#L1974) | [provider::request::invalid](../../src/request.rs#L1979) |
| `canonical_argument_string` | `serde_json_canonicalizer::to_string(value)         .map_err` | [1975](../../src/request.rs#L1975) | receiver-type-required |
| `canonical_argument_string` | `serde_json_canonicalizer::to_string` | [1975](../../src/request.rs#L1975) | external-constructor-callback-or-unresolved |
| `canonical_argument_string` | `PrepareError::InvalidJson` | [1976](../../src/request.rs#L1976) | external-constructor-callback-or-unresolved |
| `canonical_argument_string` | `error.to_string` | [1976](../../src/request.rs#L1976) | receiver-type-required |
| `invalid` | `PrepareError::InvalidJson` | [1980](../../src/request.rs#L1980) | external-constructor-callback-or-unresolved |
| `invalid` | `message.into` | [1980](../../src/request.rs#L1980) | receiver-type-required |
| `pdf_on_anthropic_is_still_a_document` | `render_anthropic(             &user(file("application/pdf", "x.pdf", b"%PDF")),             DialectId::AnthropicMessagesV1,         )         .unwrap` | [2001](../../src/request.rs#L2001) | receiver-type-required |
| `pdf_on_anthropic_is_still_a_document` | `render_anthropic` | [2001](../../src/request.rs#L2001) | external-constructor-callback-or-unresolved |
| `pdf_on_anthropic_is_still_a_document` | `user` | [2002](../../src/request.rs#L2002) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `pdf_on_anthropic_is_still_a_document` | `file` | [2002](../../src/request.rs#L2002) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `text_file_on_anthropic_degrades_to_fenced_text` | `render_anthropic(             &user(file("text/plain", "notes.txt", b"line one\nline two")),             DialectId::AnthropicMessagesV1,         )         .unwrap` | [2015](../../src/request.rs#L2015) | receiver-type-required |
| `text_file_on_anthropic_degrades_to_fenced_text` | `render_anthropic` | [2015](../../src/request.rs#L2015) | external-constructor-callback-or-unresolved |
| `text_file_on_anthropic_degrades_to_fenced_text` | `user` | [2016](../../src/request.rs#L2016) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `text_file_on_anthropic_degrades_to_fenced_text` | `file` | [2016](../../src/request.rs#L2016) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `binary_file_on_chat_completions_degrades_to_the_stub` | `render_chat(             &user(file(                 "application/octet-stream",                 "blob.bin",                 &[0xff, 0xfe, 0x00],             )),             "",             DialectId::OpenaiChatV1,         )         .unwrap` | [2028](../../src/request.rs#L2028) | receiver-type-required |
| `binary_file_on_chat_completions_degrades_to_the_stub` | `render_chat` | [2028](../../src/request.rs#L2028) | external-constructor-callback-or-unresolved |
| `binary_file_on_chat_completions_degrades_to_the_stub` | `user` | [2029](../../src/request.rs#L2029) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `binary_file_on_chat_completions_degrades_to_the_stub` | `file` | [2029](../../src/request.rs#L2029) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `oversized_utf8_file_degrades_to_the_stub` | `render_chat(             &user(file("text/plain", "big.txt", &big)),             "",             DialectId::DeepseekChatV1,         )         .unwrap` | [2047](../../src/request.rs#L2047) | receiver-type-required |
| `oversized_utf8_file_degrades_to_the_stub` | `render_chat` | [2047](../../src/request.rs#L2047), [2058](../../src/request.rs#L2058) | external-constructor-callback-or-unresolved |
| `oversized_utf8_file_degrades_to_the_stub` | `user` | [2048](../../src/request.rs#L2048), [2059](../../src/request.rs#L2059) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `oversized_utf8_file_degrades_to_the_stub` | `file` | [2048](../../src/request.rs#L2048), [2059](../../src/request.rs#L2059) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `oversized_utf8_file_degrades_to_the_stub` | `render_chat(             &user(file("text/plain", "max.txt", &exact)),             "",             DialectId::DeepseekChatV1,         )         .unwrap` | [2058](../../src/request.rs#L2058) | receiver-type-required |
| `text_file_on_responses_is_delivered_as_input_file` | `render_responses(             &user(file("text/plain", "notes.txt", b"hi")),             DialectId::OpenaiResponsesV1,         )         .unwrap` | [2074](../../src/request.rs#L2074) | receiver-type-required |
| `text_file_on_responses_is_delivered_as_input_file` | `render_responses` | [2074](../../src/request.rs#L2074), [2086](../../src/request.rs#L2086) | external-constructor-callback-or-unresolved |
| `text_file_on_responses_is_delivered_as_input_file` | `user` | [2075](../../src/request.rs#L2075), [2087](../../src/request.rs#L2087) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `text_file_on_responses_is_delivered_as_input_file` | `file` | [2075](../../src/request.rs#L2075), [2087](../../src/request.rs#L2087) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `text_file_on_responses_is_delivered_as_input_file` | `render_responses(             &user(file("text/plain", "notes.txt", b"hi")),             DialectId::DeepseekResponsesV1,         )         .unwrap` | [2086](../../src/request.rs#L2086) | receiver-type-required |
| `google_and_interactions_degrade_in_their_native_text_forms` | `render_google(             &user(file("text/csv", "t.csv", b"a,b")),             DialectId::GoogleGenerationV1,         )         .unwrap` | [2099](../../src/request.rs#L2099) | receiver-type-required |
| `google_and_interactions_degrade_in_their_native_text_forms` | `render_google` | [2099](../../src/request.rs#L2099) | external-constructor-callback-or-unresolved |
| `google_and_interactions_degrade_in_their_native_text_forms` | `user` | [2100](../../src/request.rs#L2100), [2109](../../src/request.rs#L2109) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `google_and_interactions_degrade_in_their_native_text_forms` | `file` | [2100](../../src/request.rs#L2100), [2109](../../src/request.rs#L2109) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `google_and_interactions_degrade_in_their_native_text_forms` | `render_interactions(             &user(file("text/csv", "t.csv", b"a,b")),             DialectId::GoogleInteractionsV1,             true,         )         .unwrap` | [2108](../../src/request.rs#L2108) | receiver-type-required |
| `google_and_interactions_degrade_in_their_native_text_forms` | `render_interactions` | [2108](../../src/request.rs#L2108) | external-constructor-callback-or-unresolved |
| `image_files_are_never_degraded` | `render_chat(             &user(file("image/png", "p.png", b"\x89PNG")),             "",             DialectId::OllamaChatV1,         )         .unwrap_err` | [2122](../../src/request.rs#L2122) | receiver-type-required |
| `image_files_are_never_degraded` | `render_chat` | [2122](../../src/request.rs#L2122), [2134](../../src/request.rs#L2134) | external-constructor-callback-or-unresolved |
| `image_files_are_never_degraded` | `user` | [2123](../../src/request.rs#L2123), [2135](../../src/request.rs#L2135) | [provider::request::file_degradation_tests::user](../../src/request.rs#L1988) |
| `image_files_are_never_degraded` | `file` | [2123](../../src/request.rs#L2123), [2135](../../src/request.rs#L2135) | [provider::request::file_degradation_tests::file](../../src/request.rs#L1992) |
| `image_files_are_never_degraded` | `render_chat(             &user(file("image/png", "p.png", b"\x89PNG")),             "",             DialectId::DeepseekChatV1,         )         .unwrap` | [2134](../../src/request.rs#L2134) | receiver-type-required |
| `a_bounded_schema_is_offered_to_anthropic_without_the_strict_claim` | `render_tools(             DialectId::AnthropicMessagesV1,             &json!([                 {"name":"apply_patch","description":"d","parameters":bounded},                 {"name":"think","description":"d","parameters":plain}             ]),         )         .expect` | [2180](../../src/request.rs#L2180) | receiver-type-required |
| `a_bounded_schema_is_offered_to_anthropic_without_the_strict_claim` | `render_tools` | [2180](../../src/request.rs#L2180) | external-constructor-callback-or-unresolved |
| `host_state_between_call_and_result_does_not_break_anthropic_adjacency` | `render_anthropic(&[             json!({"role":"assistant","content":[{"type":"tool_call","call_id":"task-1","name":"task","arguments":{}}]}),             json!({"role":"user","content":[{"type":"text","text":"delegation state"}]}),             json!({"role":"tool","content":[{"type":"tool_result","call_id":"task-1","result":{"outcome":"completed"}}]}),             json!({"role":"assistant","content":[{"type":"text","text":"done"}]}),             json!({"role":"user","content":[{"type":"text","text":"next turn"}]}),         ], DialectId::AnthropicMessagesV1).unwrap` | [2201](../../src/request.rs#L2201) | receiver-type-required |
| `host_state_between_call_and_result_does_not_break_anthropic_adjacency` | `render_anthropic` | [2201](../../src/request.rs#L2201) | external-constructor-callback-or-unresolved |
