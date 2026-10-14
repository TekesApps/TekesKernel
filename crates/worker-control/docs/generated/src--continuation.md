# worker-control::continuation

[Package atlas](index.md) · [Source](../../src/continuation.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [worker-control::continuation::DOMAIN](../../src/continuation.rs#L9) | const_item | `private` |  |
| [worker-control::continuation::MAX_SAFE_INTEGER](../../src/continuation.rs#L10) | const_item | `private` |  |
| [worker-control::continuation::ContinuationOperation](../../src/continuation.rs#L14) | enum_item | `pub` |  |
| [worker-control::continuation::ToolContinuationRequest](../../src/continuation.rs#L22) | struct_item | `pub` |  |
| [worker-control::continuation::ToolContinuationRequest::new](../../src/continuation.rs#L32) | function_item | `pub` |  |
| [worker-control::continuation::ToolContinuationRequest::validate_identity](../../src/continuation.rs#L50) | function_item | `private` |  |
| [worker-control::continuation::ToolContinuationRequest::derived_id](../../src/continuation.rs#L67) | function_item | `private` |  |
| [worker-control::continuation::ToolContinuationRequest::validate](../../src/continuation.rs#L79) | function_item | `pub` |  |
| [worker-control::continuation::ToolContinuationRequest::matches_receipt](../../src/continuation.rs#L88) | function_item | `pub` |  |
| [worker-control::continuation::tests::original](../../src/continuation.rs#L98) | function_item | `private` | test; #[cfg(test)] |
| [worker-control::continuation::tests::step_identity_is_stable_but_changed_action_is_not_an_exact_retry](../../src/continuation.rs#L110) | function_item | `private` | test; #[cfg(test)] |
| [worker-control::continuation::ToolContinuationOutcome](../../src/continuation.rs#L163) | enum_item | `pub` |  |
| [worker-control::continuation::ToolContinuationResponse](../../src/continuation.rs#L179) | struct_item | `pub` |  |
| [worker-control::continuation::ToolContinuationResponse::validate_for](../../src/continuation.rs#L186) | function_item | `pub` |  |
| [worker-control::continuation::outcome_tests::pending_outcomes_bind_identity_and_advance_exactly_one_step](../../src/continuation.rs#L210) | function_item | `private` | test; #[cfg(test)] |
| [worker-control::continuation::TOOL_CONTINUATION_KEY](../../src/continuation.rs#L264) | const_item | `pub` |  |
| [worker-control::continuation::TOOL_CONTINUATION_RESULT_KEY](../../src/continuation.rs#L266) | const_item | `pub` |  |
| [worker-control::continuation::encode_tool_continuation](../../src/continuation.rs#L268) | function_item | `pub` |  |
| [worker-control::continuation::decode_tool_continuation](../../src/continuation.rs#L277) | function_item | `pub` |  |
| [worker-control::continuation::encode_tool_continuation_result](../../src/continuation.rs#L287) | function_item | `pub` |  |
| [worker-control::continuation::decode_tool_continuation_result](../../src/continuation.rs#L293) | function_item | `pub` |  |
| [worker-control::continuation::wire_tests::continuation_wire_pair_round_trips_and_rejects_foreign_keys](../../src/continuation.rs#L303) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `ToolControl` | `crate::durable::ToolControl` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `worker-control::continuation::tests` | `private` | #[cfg(test)] |
| `worker-control::continuation::outcome_tests` | `private` | #[cfg(test)] |
| `worker-control::continuation::wire_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–10: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["worker-control::continuation::ToolContinuationResponse::validate_for"]
  n1["worker-control::continuation::encode_tool_continuation"]
  n2["worker-control::continuation::decode_tool_continuation"]
  n3["worker-control::continuation::encode_tool_continuation_result"]
  n4["worker-control::continuation::decode_tool_continuation_result"]
  n5["worker-control::continuation::ToolContinuationRequest::new"]
  n6["worker-control::continuation::ToolContinuationRequest::validate_identity"]
  n7["worker-control::continuation::ToolContinuationRequest::derived_id"]
  n8["worker-control::continuation::ToolContinuationRequest::validate"]
  n9["worker-control::continuation::ToolContinuationRequest::matches_receipt"]
  n10["worker-control::encode_line"]
  n11["worker-control::decode_named"]
  n1 --> n10
  n2 --> n11
  n3 --> n10
  n4 --> n11
  n8 --> n6
  n8 --> n7
  n9 --> n8
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `String::new` | [40](../../src/continuation.rs#L40) | external-constructor-callback-or-unresolved |
| `new` | `request.validate_identity` | [46](../../src/continuation.rs#L46) | receiver-type-required |
| `new` | `request.derived_id` | [47](../../src/continuation.rs#L47) | receiver-type-required |
| `new` | `Ok` | [48](../../src/continuation.rs#L48) | external-constructor-callback-or-unresolved |
| `validate_identity` | `self.original             .validate()             .map_err` | [51](../../src/continuation.rs#L51) | receiver-type-required |
| `validate_identity` | `self.original             .validate` | [51](../../src/continuation.rs#L51) | receiver-type-required |
| `validate_identity` | `error.to_string` | [53](../../src/continuation.rs#L53) | receiver-type-required |
| `validate_identity` | `Err` | [55](../../src/continuation.rs#L55), [63](../../src/continuation.rs#L63) | external-constructor-callback-or-unresolved |
| `validate_identity` | `"invalid continuation format or step".into` | [55](../../src/continuation.rs#L55) | receiver-type-required |
| `validate_identity` | `self.continuation_id.len` | [57](../../src/continuation.rs#L57) | receiver-type-required |
| `validate_identity` | `self                 .continuation_id                 .bytes()                 .all` | [58](../../src/continuation.rs#L58) | receiver-type-required |
| `validate_identity` | `self                 .continuation_id                 .bytes` | [58](../../src/continuation.rs#L58) | receiver-type-required |
| `validate_identity` | `byte.is_ascii_digit` | [61](../../src/continuation.rs#L61) | receiver-type-required |
| `validate_identity` | `(b'a'..=b'f').contains` | [61](../../src/continuation.rs#L61) | receiver-type-required |
| `validate_identity` | `"invalid host continuation identity".into` | [63](../../src/continuation.rs#L63) | receiver-type-required |
| `validate_identity` | `Ok` | [65](../../src/continuation.rs#L65) | external-constructor-callback-or-unresolved |
| `derived_id` | `serde_json_canonicalizer::to_vec(&(             &self.original.request_id,             &self.continuation_id,             self.step,         ))         .map_err` | [68](../../src/continuation.rs#L68) | receiver-type-required |
| `derived_id` | `serde_json_canonicalizer::to_vec` | [68](../../src/continuation.rs#L68) | external-constructor-callback-or-unresolved |
| `derived_id` | `error.to_string` | [73](../../src/continuation.rs#L73) | receiver-type-required |
| `derived_id` | `Sha256::new` | [74](../../src/continuation.rs#L74) | external-constructor-callback-or-unresolved |
| `derived_id` | `digest.update` | [75](../../src/continuation.rs#L75), [76](../../src/continuation.rs#L76) | receiver-type-required |
| `derived_id` | `Ok` | [77](../../src/continuation.rs#L77) | external-constructor-callback-or-unresolved |
| `validate` | `self.validate_identity` | [80](../../src/continuation.rs#L80) | [worker-control::continuation::ToolContinuationRequest::validate_identity](../../src/continuation.rs#L50) |
| `validate` | `self.derived_id` | [81](../../src/continuation.rs#L81) | [worker-control::continuation::ToolContinuationRequest::derived_id](../../src/continuation.rs#L67) |
| `validate` | `Err` | [82](../../src/continuation.rs#L82) | external-constructor-callback-or-unresolved |
| `validate` | `"continuation request identity mismatch".into` | [82](../../src/continuation.rs#L82) | receiver-type-required |
| `validate` | `Ok` | [84](../../src/continuation.rs#L84) | external-constructor-callback-or-unresolved |
| `matches_receipt` | `self.validate` | [89](../../src/continuation.rs#L89) | [worker-control::continuation::ToolContinuationRequest::validate](../../src/continuation.rs#L79) |
| `matches_receipt` | `stored.validate` | [90](../../src/continuation.rs#L90) | receiver-type-required |
| `matches_receipt` | `Ok` | [91](../../src/continuation.rs#L91) | external-constructor-callback-or-unresolved |
| `original` | `ToolControl::new(             "018f0000-0000-7000-8000-000000000003",             "018f0000-0000-7000-8000-000000000003",             1,             "call-1",             "mcp__test__task",             IJsonValue::parse_str("{}").unwrap(),         )         .unwrap` | [99](../../src/continuation.rs#L99) | receiver-type-required |
| `original` | `ToolControl::new` | [99](../../src/continuation.rs#L99) | external-constructor-callback-or-unresolved |
| `original` | `IJsonValue::parse_str("{}").unwrap` | [105](../../src/continuation.rs#L105) | receiver-type-required |
| `original` | `IJsonValue::parse_str` | [105](../../src/continuation.rs#L105) | external-constructor-callback-or-unresolved |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `ToolContinuationRequest::new(             original(),             "a".repeat(64),             1,             ContinuationOperation::Query,         )         .unwrap` | [111](../../src/continuation.rs#L111) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `ToolContinuationRequest::new` | [111](../../src/continuation.rs#L111), [122](../../src/continuation.rs#L122), [131](../../src/continuation.rs#L131) | external-constructor-callback-or-unresolved |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `original` | [112](../../src/continuation.rs#L112), [123](../../src/continuation.rs#L123), [132](../../src/continuation.rs#L132) | [worker-control::continuation::tests::original](../../src/continuation.rs#L98) |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `"a".repeat` | [113](../../src/continuation.rs#L113), [124](../../src/continuation.rs#L124), [133](../../src/continuation.rs#L133) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `serde_json::from_slice(&serde_json::to_vec(&first).unwrap()).unwrap` | [119](../../src/continuation.rs#L119) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `serde_json::from_slice` | [119](../../src/continuation.rs#L119) | external-constructor-callback-or-unresolved |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `serde_json::to_vec(&first).unwrap` | [119](../../src/continuation.rs#L119) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `serde_json::to_vec` | [119](../../src/continuation.rs#L119) | external-constructor-callback-or-unresolved |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `ToolContinuationRequest::new(             original(),             "a".repeat(64),             1,             ContinuationOperation::Cancel,         )         .unwrap` | [122](../../src/continuation.rs#L122) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `ToolContinuationRequest::new(             original(),             "a".repeat(64),             2,             ContinuationOperation::Query,         )         .unwrap` | [131](../../src/continuation.rs#L131) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `first.clone` | [139](../../src/continuation.rs#L139), [143](../../src/continuation.rs#L143) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `IJsonValue::parse_str(r#"{"changed":true}"#).unwrap` | [141](../../src/continuation.rs#L141) | receiver-type-required |
| `step_identity_is_stable_but_changed_action_is_not_an_exact_retry` | `IJsonValue::parse_str` | [141](../../src/continuation.rs#L141) | external-constructor-callback-or-unresolved |
| `validate_for` | `request.validate` | [187](../../src/continuation.rs#L187) | receiver-type-required |
| `validate_for` | `Err` | [189](../../src/continuation.rs#L189), [200](../../src/continuation.rs#L200) | external-constructor-callback-or-unresolved |
| `validate_for` | `"continuation response binding mismatch".into` | [189](../../src/continuation.rs#L189) | receiver-type-required |
| `validate_for` | `"pending continuation identity or sequence mismatch".into` | [200](../../src/continuation.rs#L200) | receiver-type-required |
| `validate_for` | `Ok` | [202](../../src/continuation.rs#L202) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `ToolControl::new(             "018f0000-0000-7000-8000-000000000003",             "018f0000-0000-7000-8000-000000000003",             1,             "call-1",             "mcp__test__task",             IJsonValue::parse_str("{}").unwrap(),         )         .unwrap` | [211](../../src/continuation.rs#L211) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `ToolControl::new` | [211](../../src/continuation.rs#L211) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `IJsonValue::parse_str("{}").unwrap` | [217](../../src/continuation.rs#L217) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `IJsonValue::parse_str` | [217](../../src/continuation.rs#L217), [229](../../src/continuation.rs#L229) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `ToolContinuationRequest::new(original, "a".repeat(64), 1, ContinuationOperation::Query)                 .unwrap` | [221](../../src/continuation.rs#L221) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `ToolContinuationRequest::new` | [221](../../src/continuation.rs#L221) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `"a".repeat` | [221](../../src/continuation.rs#L221), [239](../../src/continuation.rs#L239), [240](../../src/continuation.rs#L240) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `request.request_id.clone` | [224](../../src/continuation.rs#L224) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `request.original.call_id.clone` | [225](../../src/continuation.rs#L225) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `request.continuation_id.clone` | [227](../../src/continuation.rs#L227) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `IJsonValue::parse_str(r#"{"taskId":"remote-1","status":"working"}"#)                     .unwrap` | [229](../../src/continuation.rs#L229) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `response.validate_for(&request).unwrap` | [233](../../src/continuation.rs#L233) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `response.validate_for` | [233](../../src/continuation.rs#L233) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `serde_json::to_vec(&response).unwrap` | [234](../../src/continuation.rs#L234) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `serde_json::to_vec` | [234](../../src/continuation.rs#L234) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `serde_json::from_slice(&bytes).unwrap` | [235](../../src/continuation.rs#L235), [257](../../src/continuation.rs#L257) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `serde_json::from_slice` | [235](../../src/continuation.rs#L235), [257](../../src/continuation.rs#L257) | external-constructor-callback-or-unresolved |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `"b".repeat` | [238](../../src/continuation.rs#L238) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `response.clone` | [242](../../src/continuation.rs#L242), [254](../../src/continuation.rs#L254) | receiver-type-required |
| `pending_outcomes_bind_identity_and_advance_exactly_one_step` | `"other".into` | [255](../../src/continuation.rs#L255) | receiver-type-required |
| `encode_tool_continuation` | `request         .validate()         .map_err` | [271](../../src/continuation.rs#L271) | receiver-type-required |
| `encode_tool_continuation` | `request         .validate` | [271](../../src/continuation.rs#L271) | receiver-type-required |
| `encode_tool_continuation` | `crate::encode_line` | [274](../../src/continuation.rs#L274) | [worker-control::encode_line](../../src/lib.rs#L359) |
| `decode_tool_continuation` | `crate::decode_named` | [280](../../src/continuation.rs#L280) | [worker-control::decode_named](../../src/lib.rs#L367) |
| `decode_tool_continuation` | `request         .validate()         .map_err` | [281](../../src/continuation.rs#L281) | receiver-type-required |
| `decode_tool_continuation` | `request         .validate` | [281](../../src/continuation.rs#L281) | receiver-type-required |
| `decode_tool_continuation` | `Ok` | [284](../../src/continuation.rs#L284) | external-constructor-callback-or-unresolved |
| `encode_tool_continuation_result` | `crate::encode_line` | [290](../../src/continuation.rs#L290) | [worker-control::encode_line](../../src/lib.rs#L359) |
| `decode_tool_continuation_result` | `crate::decode_named` | [296](../../src/continuation.rs#L296) | [worker-control::decode_named](../../src/lib.rs#L367) |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `ToolControl::new(             "018f0000-0000-7000-8000-000000000003",             "018f0000-0000-7000-8000-000000000003",             1,             "call-1",             "mcp__test__task",             IJsonValue::parse_str("{}").unwrap(),         )         .unwrap` | [304](../../src/continuation.rs#L304) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `ToolControl::new` | [304](../../src/continuation.rs#L304) | external-constructor-callback-or-unresolved |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `IJsonValue::parse_str("{}").unwrap` | [310](../../src/continuation.rs#L310) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `IJsonValue::parse_str` | [310](../../src/continuation.rs#L310), [327](../../src/continuation.rs#L327) | external-constructor-callback-or-unresolved |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `ToolContinuationRequest::new(original, "a".repeat(64), 1, ContinuationOperation::Query)                 .unwrap` | [314](../../src/continuation.rs#L314) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `ToolContinuationRequest::new` | [314](../../src/continuation.rs#L314) | external-constructor-callback-or-unresolved |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `"a".repeat` | [314](../../src/continuation.rs#L314) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `encode_tool_continuation(&request).unwrap` | [316](../../src/continuation.rs#L316) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `encode_tool_continuation` | [316](../../src/continuation.rs#L316) | external-constructor-callback-or-unresolved |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `request.request_id.clone` | [324](../../src/continuation.rs#L324) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `"call-1".into` | [325](../../src/continuation.rs#L325) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `IJsonValue::parse_str(r#"{"content":[]}"#).unwrap` | [327](../../src/continuation.rs#L327) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `encode_tool_continuation_result(&response).unwrap` | [330](../../src/continuation.rs#L330) | receiver-type-required |
| `continuation_wire_pair_round_trips_and_rejects_foreign_keys` | `encode_tool_continuation_result` | [330](../../src/continuation.rs#L330) | external-constructor-callback-or-unresolved |
