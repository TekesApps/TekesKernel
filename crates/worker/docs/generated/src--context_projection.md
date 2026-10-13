# tekes-worker::context_projection

[Package atlas](index.md) · [Source](../../src/context_projection.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::context_projection::dynamic_catalog_revision](../../src/context_projection.rs#L4) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::native_search_references](../../src/context_projection.rs#L20) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::project_provider_context](../../src/context_projection.rs#L94) | function_item | `pub(crate)` | test; #[cfg(test)] |
| [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::seed_provider_items](../../src/context_projection.rs#L519) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::provider_wire_call_id](../../src/context_projection.rs#L622) | function_item | `private` |  |
| [tekes-worker::context_projection::render_sealed_carrier](../../src/context_projection.rs#L636) | function_item | `private` |  |
| [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L663) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L748) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L799) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_json_at](../../src/context_projection.rs#L806) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L827) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::seqs_from_ranges](../../src/context_projection.rs#L838) | function_item | `private` |  |
| [tekes-worker::context_projection::effective_system](../../src/context_projection.rs#L854) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::EpochAppend](../../src/context_projection.rs#L866) | struct_item | `pub(crate)` |  |
| [tekes-worker::context_projection::epoch_open_reason](../../src/context_projection.rs#L881) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::append_tool_result_trim](../../src/context_projection.rs#L921) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::append_provider_epoch](../../src/context_projection.rs#L961) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::latest_compatible_epoch](../../src/context_projection.rs#L1007) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::ranges](../../src/context_projection.rs#L1042) | function_item | `pub(crate)` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–18: 17 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::context::trimmed_tool_result_content"]
  n1["store::asset::AssetStore::new"]
  n2["tekes-worker::context_projection::latest_compatible_epoch"]
  n3["tekes-worker::context_projection::ranges"]
  n4["tekes-worker::context_projection::project_provider_context_mode"]
  n5["tekes-worker::context_projection::native_search_references"]
  n6["tekes-worker::context_projection::dynamic_catalog_revision"]
  n7["tekes-worker::context_projection::seed_provider_items"]
  n8["tekes-worker::context_projection::provider_wire_call_id"]
  n9["tekes-worker::context_projection::render_sealed_carrier"]
  n10["tekes-worker::context_projection::render_event_item"]
  n11["tekes-worker::context_projection::render_blocks"]
  n12["tekes-worker::context_projection::materialize_json"]
  n13["tekes-worker::context_projection::materialize_json_at"]
  n14["tekes-worker::context_projection::materialize_string"]
  n15["tekes-worker::context_projection::seqs_from_ranges"]
  n16["tekes-worker::context_projection::effective_system"]
  n17["tekes-worker::context_projection::epoch_open_reason"]
  n18["tekes-worker::context_projection::append_tool_result_trim"]
  n19["tekes-worker::context_projection::append_provider_epoch"]
  n4 --> n7
  n4 --> n10
  n4 --> n14
  n4 --> n15
  n5 --> n12
  n7 --> n1
  n9 --> n14
  n10 --> n9
  n10 --> n11
  n11 --> n1
  n11 --> n12
  n12 --> n13
  n13 --> n1
  n14 --> n12
  n18 --> n0
  n18 --> n12
  n19 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `dynamic_catalog_revision` | `Sha256::new` | [5](../../src/context_projection.rs#L5) | external-constructor-callback-or-unresolved |
| `dynamic_catalog_revision` | `digest.update` | [6](../../src/context_projection.rs#L6), [8](../../src/context_projection.rs#L8), [9](../../src/context_projection.rs#L9), [10](../../src/context_projection.rs#L10), [11](../../src/context_projection.rs#L11) | receiver-type-required |
| `dynamic_catalog_revision` | `entry.name.as_bytes` | [8](../../src/context_projection.rs#L8) | receiver-type-required |
| `dynamic_catalog_revision` | `entry.schema_digest.as_bytes` | [10](../../src/context_projection.rs#L10) | receiver-type-required |
| `native_search_references` | `ledger.projection().ok_or` | [27](../../src/context_projection.rs#L27) | receiver-type-required |
| `native_search_references` | `ledger.projection` | [27](../../src/context_projection.rs#L27) | receiver-type-required |
| `native_search_references` | `projection         .events         .iter()         .filter(&#124;event&#124; {             event.seq() < before_seq                 && event.turn() == Some(turn)                 && *event.kind() == EventKind::ToolCall                 && event.string_field("name") == Some("tool_search")         })         .filter_map(&#124;event&#124; event.string_field("call").map(str::to_owned))         .collect::<Vec<_>>` | [28](../../src/context_projection.rs#L28) | receiver-type-required |
| `native_search_references` | `projection         .events         .iter()         .filter(&#124;event&#124; {             event.seq() < before_seq                 && event.turn() == Some(turn)                 && *event.kind() == EventKind::ToolCall                 && event.string_field("name") == Some("tool_search")         })         .filter_map` | [28](../../src/context_projection.rs#L28) | receiver-type-required |
| `native_search_references` | `projection         .events         .iter()         .filter` | [28](../../src/context_projection.rs#L28) | receiver-type-required |
| `native_search_references` | `projection         .events         .iter` | [28](../../src/context_projection.rs#L28) | receiver-type-required |
| `native_search_references` | `event.seq` | [32](../../src/context_projection.rs#L32), [43](../../src/context_projection.rs#L43) | receiver-type-required |
| `native_search_references` | `event.turn` | [33](../../src/context_projection.rs#L33) | receiver-type-required |
| `native_search_references` | `Some` | [33](../../src/context_projection.rs#L33), [35](../../src/context_projection.rs#L35), [45](../../src/context_projection.rs#L45), [50](../../src/context_projection.rs#L50) | external-constructor-callback-or-unresolved |
| `native_search_references` | `event.kind` | [34](../../src/context_projection.rs#L34), [44](../../src/context_projection.rs#L44) | receiver-type-required |
| `native_search_references` | `event.string_field` | [35](../../src/context_projection.rs#L35), [37](../../src/context_projection.rs#L37), [45](../../src/context_projection.rs#L45) | receiver-type-required |
| `native_search_references` | `event.string_field("call").map` | [37](../../src/context_projection.rs#L37) | receiver-type-required |
| `native_search_references` | `Vec::new` | [39](../../src/context_projection.rs#L39) | external-constructor-callback-or-unresolved |
| `native_search_references` | `BTreeSet::new` | [40](../../src/context_projection.rs#L40) | external-constructor-callback-or-unresolved |
| `native_search_references` | `projection.events.iter().find` | [42](../../src/context_projection.rs#L42) | receiver-type-required |
| `native_search_references` | `projection.events.iter` | [42](../../src/context_projection.rs#L42) | receiver-type-required |
| `native_search_references` | `call.as_str` | [45](../../src/context_projection.rs#L45) | receiver-type-required |
| `native_search_references` | `serde_json::to_value` | [49](../../src/context_projection.rs#L49) | external-constructor-callback-or-unresolved |
| `native_search_references` | `result.raw` | [49](../../src/context_projection.rs#L49) | receiver-type-required |
| `native_search_references` | `raw.get("outcome").and_then` | [50](../../src/context_projection.rs#L50) | receiver-type-required |
| `native_search_references` | `raw.get` | [50](../../src/context_projection.rs#L50), [53](../../src/context_projection.rs#L53) | receiver-type-required |
| `native_search_references` | `materialize_json` | [56](../../src/context_projection.rs#L56) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L799) |
| `native_search_references` | `content.as_array().into_iter().flatten` | [57](../../src/context_projection.rs#L57) | receiver-type-required |
| `native_search_references` | `content.as_array().into_iter` | [57](../../src/context_projection.rs#L57) | receiver-type-required |
| `native_search_references` | `content.as_array` | [57](../../src/context_projection.rs#L57) | receiver-type-required |
| `native_search_references` | `block.get("text").and_then` | [58](../../src/context_projection.rs#L58) | receiver-type-required |
| `native_search_references` | `block.get` | [58](../../src/context_projection.rs#L58) | receiver-type-required |
| `native_search_references` | `serde_json::from_str::<Value>` | [61](../../src/context_projection.rs#L61) | external-constructor-callback-or-unresolved |
| `native_search_references` | `offer                 .get("items")                 .and_then(Value::as_array)                 .into_iter()                 .flatten` | [64](../../src/context_projection.rs#L64) | receiver-type-required |
| `native_search_references` | `offer                 .get("items")                 .and_then(Value::as_array)                 .into_iter` | [64](../../src/context_projection.rs#L64) | receiver-type-required |
| `native_search_references` | `offer                 .get("items")                 .and_then` | [64](../../src/context_projection.rs#L64) | receiver-type-required |
| `native_search_references` | `offer                 .get` | [64](../../src/context_projection.rs#L64) | receiver-type-required |
| `native_search_references` | `item.get("name").and_then` | [71](../../src/context_projection.rs#L71) | receiver-type-required |
| `native_search_references` | `item.get` | [71](../../src/context_projection.rs#L71), [72](../../src/context_projection.rs#L72) | receiver-type-required |
| `native_search_references` | `item.get("schema_digest").and_then` | [72](../../src/context_projection.rs#L72) | receiver-type-required |
| `native_search_references` | `deferred                     .iter()                     .any` | [76](../../src/context_projection.rs#L76) | receiver-type-required |
| `native_search_references` | `deferred                     .iter` | [76](../../src/context_projection.rs#L76) | receiver-type-required |
| `native_search_references` | `seen.insert` | [79](../../src/context_projection.rs#L79) | receiver-type-required |
| `native_search_references` | `call.clone` | [79](../../src/context_projection.rs#L79), [84](../../src/context_projection.rs#L84) | receiver-type-required |
| `native_search_references` | `name.to_owned` | [79](../../src/context_projection.rs#L79), [81](../../src/context_projection.rs#L81) | receiver-type-required |
| `native_search_references` | `references.push` | [80](../../src/context_projection.rs#L80) | receiver-type-required |
| `native_search_references` | `digest.to_owned` | [82](../../src/context_projection.rs#L82) | receiver-type-required |
| `native_search_references` | `catalog_revision.to_owned` | [83](../../src/context_projection.rs#L83) | receiver-type-required |
| `native_search_references` | `Ok` | [90](../../src/context_projection.rs#L90) | external-constructor-callback-or-unresolved |
| `project_provider_context` | `project_provider_context_mode` | [100](../../src/context_projection.rs#L100) | [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104) |
| `project_provider_context_mode` | `ledger.projection().ok_or` | [111](../../src/context_projection.rs#L111) | receiver-type-required |
| `project_provider_context_mode` | `ledger.projection` | [111](../../src/context_projection.rs#L111) | receiver-type-required |
| `project_provider_context_mode` | `std::collections::BTreeMap::<String, String>::new` | [112](../../src/context_projection.rs#L112), [122](../../src/context_projection.rs#L122), [128](../../src/context_projection.rs#L128) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeMap::<String, Vec<u64>>::new` | [113](../../src/context_projection.rs#L113) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeSet::<String>::new` | [114](../../src/context_projection.rs#L114), [118](../../src/context_projection.rs#L118), [131](../../src/context_projection.rs#L131) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeMap::<String, u64>::new` | [121](../../src/context_projection.rs#L121), [127](../../src/context_projection.rs#L127), [130](../../src/context_projection.rs#L130) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeSet::<u64>::new` | [123](../../src/context_projection.rs#L123), [124](../../src/context_projection.rs#L124), [135](../../src/context_projection.rs#L135) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeMap::<u64, u64>::new` | [125](../../src/context_projection.rs#L125), [126](../../src/context_projection.rs#L126), [313](../../src/context_projection.rs#L313), [358](../../src/context_projection.rs#L358) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `Vec::<u64>::new` | [129](../../src/context_projection.rs#L129) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `serde_json::to_value` | [138](../../src/context_projection.rs#L138), [284](../../src/context_projection.rs#L284), [318](../../src/context_projection.rs#L318), [407](../../src/context_projection.rs#L407) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `event.raw` | [138](../../src/context_projection.rs#L138), [284](../../src/context_projection.rs#L284), [318](../../src/context_projection.rs#L318), [407](../../src/context_projection.rs#L407) | receiver-type-required |
| `project_provider_context_mode` | `raw.as_object().ok_or` | [139](../../src/context_projection.rs#L139), [285](../../src/context_projection.rs#L285), [319](../../src/context_projection.rs#L319), [408](../../src/context_projection.rs#L408) | receiver-type-required |
| `project_provider_context_mode` | `raw.as_object` | [139](../../src/context_projection.rs#L139), [285](../../src/context_projection.rs#L285), [319](../../src/context_projection.rs#L319), [408](../../src/context_projection.rs#L408) | receiver-type-required |
| `project_provider_context_mode` | `seqs_from_ranges` | [140](../../src/context_projection.rs#L140), [144](../../src/context_projection.rs#L144), [201](../../src/context_projection.rs#L201), [320](../../src/context_projection.rs#L320) | [tekes-worker::context_projection::seqs_from_ranges](../../src/context_projection.rs#L838) |
| `project_provider_context_mode` | `object.get` | [140](../../src/context_projection.rs#L140), [144](../../src/context_projection.rs#L144), [183](../../src/context_projection.rs#L183), [201](../../src/context_projection.rs#L201), [288](../../src/context_projection.rs#L288), [320](../../src/context_projection.rs#L320), [455](../../src/context_projection.rs#L455) | receiver-type-required |
| `project_provider_context_mode` | `superseded.insert` | [141](../../src/context_projection.rs#L141) | receiver-type-required |
| `project_provider_context_mode` | `event.kind` | [143](../../src/context_projection.rs#L143), [148](../../src/context_projection.rs#L148), [278](../../src/context_projection.rs#L278), [315](../../src/context_projection.rs#L315), [371](../../src/context_projection.rs#L371), [415](../../src/context_projection.rs#L415), [428](../../src/context_projection.rs#L428), [438](../../src/context_projection.rs#L438), [453](../../src/context_projection.rs#L453), [469](../../src/context_projection.rs#L469) | receiver-type-required |
| `project_provider_context_mode` | `compacted.insert` | [145](../../src/context_projection.rs#L145) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field` | [149](../../src/context_projection.rs#L149), [230](../../src/context_projection.rs#L230), [231](../../src/context_projection.rs#L231), [339](../../src/context_projection.rs#L339), [380](../../src/context_projection.rs#L380), [438](../../src/context_projection.rs#L438), [490](../../src/context_projection.rs#L490) | receiver-type-required |
| `project_provider_context_mode` | `Some` | [149](../../src/context_projection.rs#L149), [150](../../src/context_projection.rs#L150), [165](../../src/context_projection.rs#L165), [183](../../src/context_projection.rs#L183), [387](../../src/context_projection.rs#L387), [438](../../src/context_projection.rs#L438), [455](../../src/context_projection.rs#L455), [478](../../src/context_projection.rs#L478) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `event.seq` | [150](../../src/context_projection.rs#L150), [175](../../src/context_projection.rs#L175), [184](../../src/context_projection.rs#L184), [186](../../src/context_projection.rs#L186), [210](../../src/context_projection.rs#L210), [227](../../src/context_projection.rs#L227), [235](../../src/context_projection.rs#L235), [279](../../src/context_projection.rs#L279), [280](../../src/context_projection.rs#L280), [300](../../src/context_projection.rs#L300), [324](../../src/context_projection.rs#L324), [338](../../src/context_projection.rs#L338), [343](../../src/context_projection.rs#L343), [370](../../src/context_projection.rs#L370), [402](../../src/context_projection.rs#L402), [403](../../src/context_projection.rs#L403), [409](../../src/context_projection.rs#L409), [410](../../src/context_projection.rs#L410), [454](../../src/context_projection.rs#L454), [456](../../src/context_projection.rs#L456), [471](../../src/context_projection.rs#L471), [481](../../src/context_projection.rs#L481), [484](../../src/context_projection.rs#L484), [503](../../src/context_projection.rs#L503), [504](../../src/context_projection.rs#L504) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten()                     .flat_map(&#124;value&#124; value.as_array().into_iter().flatten())                     .filter_map` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten()                     .flat_map` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then` | [151](../../src/context_projection.rs#L151) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get` | [151](../../src/context_projection.rs#L151), [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `pending.get` | [155](../../src/context_projection.rs#L155) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array().into_iter().flatten` | [157](../../src/context_projection.rs#L157) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array().into_iter` | [157](../../src/context_projection.rs#L157) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array` | [157](../../src/context_projection.rs#L157) | receiver-type-required |
| `project_provider_context_mode` | `epoch_pending.insert` | [160](../../src/context_projection.rs#L160) | receiver-type-required |
| `project_provider_context_mode` | `event.turn().ok_or` | [164](../../src/context_projection.rs#L164), [234](../../src/context_projection.rs#L234) | receiver-type-required |
| `project_provider_context_mode` | `event.turn` | [164](../../src/context_projection.rs#L164), [179](../../src/context_projection.rs#L179), [234](../../src/context_projection.rs#L234), [466](../../src/context_projection.rs#L466), [478](../../src/context_projection.rs#L478) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter()                     .flatten()                     .filter_map` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter()                     .flatten` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `trigger.get` | [168](../../src/context_projection.rs#L168) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.insert` | [174](../../src/context_projection.rs#L174), [186](../../src/context_projection.rs#L186) | receiver-type-required |
| `project_provider_context_mode` | `input_position.insert` | [175](../../src/context_projection.rs#L175) | receiver-type-required |
| `project_provider_context_mode` | `object.get("steer").and_then` | [183](../../src/context_projection.rs#L183), [455](../../src/context_projection.rs#L455) | receiver-type-required |
| `project_provider_context_mode` | `steer_seqs.push` | [184](../../src/context_projection.rs#L184) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field("attempt")                     .ok_or("attempt lacks id")?                     .to_owned` | [190](../../src/context_projection.rs#L190) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field("attempt")                     .ok_or` | [190](../../src/context_projection.rs#L190), [204](../../src/context_projection.rs#L204), [221](../../src/context_projection.rs#L221) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field` | [190](../../src/context_projection.rs#L190), [204](../../src/context_projection.rs#L204), [221](../../src/context_projection.rs#L221) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.insert` | [194](../../src/context_projection.rs#L194) | receiver-type-required |
| `project_provider_context_mode` | `id.clone` | [195](../../src/context_projection.rs#L195) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("epoch")                         .ok_or("attempt lacks epoch")?                         .to_owned` | [196](../../src/context_projection.rs#L196) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("epoch")                         .ok_or` | [196](../../src/context_projection.rs#L196) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field` | [196](../../src/context_projection.rs#L196), [238](../../src/context_projection.rs#L238), [246](../../src/context_projection.rs#L246) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits.insert` | [201](../../src/context_projection.rs#L201) | receiver-type-required |
| `project_provider_context_mode` | `settled_outputs.insert` | [207](../../src/context_projection.rs#L207) | receiver-type-required |
| `project_provider_context_mode` | `attempt.to_owned` | [207](../../src/context_projection.rs#L207), [209](../../src/context_projection.rs#L209), [224](../../src/context_projection.rs#L224), [226](../../src/context_projection.rs#L226), [232](../../src/context_projection.rs#L232) | receiver-type-required |
| `project_provider_context_mode` | `output_positions                     .entry(attempt.to_owned())                     .or_insert` | [208](../../src/context_projection.rs#L208), [225](../../src/context_projection.rs#L225) | receiver-type-required |
| `project_provider_context_mode` | `output_positions                     .entry` | [208](../../src/context_projection.rs#L208), [225](../../src/context_projection.rs#L225) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get(attempt).is_some_and` | [211](../../src/context_projection.rs#L211) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get` | [211](../../src/context_projection.rs#L211), [265](../../src/context_projection.rs#L265) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then(Value::as_str)                         .map(str::to_owned)                         .or` | [212](../../src/context_projection.rs#L212) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then(Value::as_str)                         .map` | [212](../../src/context_projection.rs#L212) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then` | [212](../../src/context_projection.rs#L212) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then` | [212](../../src/context_projection.rs#L212) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get` | [212](../../src/context_projection.rs#L212) | receiver-type-required |
| `project_provider_context_mode` | `continuation.get` | [214](../../src/context_projection.rs#L214) | receiver-type-required |
| `project_provider_context_mode` | `object.contains_key` | [220](../../src/context_projection.rs#L220), [416](../../src/context_projection.rs#L416) | receiver-type-required |
| `project_provider_context_mode` | `partial_carriers.insert` | [224](../../src/context_projection.rs#L224) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field("call").ok_or` | [230](../../src/context_projection.rs#L230) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts.insert` | [232](../../src/context_projection.rs#L232) | receiver-type-required |
| `project_provider_context_mode` | `call.to_owned` | [232](../../src/context_projection.rs#L232), [234](../../src/context_projection.rs#L234), [235](../../src/context_projection.rs#L235), [237](../../src/context_projection.rs#L237) | receiver-type-required |
| `project_provider_context_mode` | `call_turn.insert` | [234](../../src/context_projection.rs#L234) | receiver-type-required |
| `project_provider_context_mode` | `call_seqs.insert` | [235](../../src/context_projection.rs#L235) | receiver-type-required |
| `project_provider_context_mode` | `call_name.insert` | [236](../../src/context_projection.rs#L236) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("name")                         .ok_or("tool_call lacks name")?                         .to_owned` | [238](../../src/context_projection.rs#L238) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("name")                         .ok_or` | [238](../../src/context_projection.rs#L238) | receiver-type-required |
| `project_provider_context_mode` | `spawned_calls.insert` | [245](../../src/context_projection.rs#L245) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("call")                         .ok_or("spawn lacks call")?                         .to_owned` | [246](../../src/context_projection.rs#L246) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("call")                         .ok_or` | [246](../../src/context_projection.rs#L246) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; settled_outputs.contains(*attempt))         .flat_map(&#124;(_, seqs)&#124; seqs.iter().copied())         .collect::<std::collections::BTreeSet<_>>` | [256](../../src/context_projection.rs#L256) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; settled_outputs.contains(*attempt))         .flat_map` | [256](../../src/context_projection.rs#L256) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter` | [256](../../src/context_projection.rs#L256), [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter` | [256](../../src/context_projection.rs#L256), [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `settled_outputs.contains` | [258](../../src/context_projection.rs#L258), [264](../../src/context_projection.rs#L264), [491](../../src/context_projection.rs#L491) | receiver-type-required |
| `project_provider_context_mode` | `seqs.iter().copied` | [259](../../src/context_projection.rs#L259), [267](../../src/context_projection.rs#L267) | receiver-type-required |
| `project_provider_context_mode` | `seqs.iter` | [259](../../src/context_projection.rs#L259), [267](../../src/context_projection.rs#L267) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; {             settled_outputs.contains(*attempt)                 && attempt_epoch.get(*attempt).is_some_and(&#124;id&#124; id == epoch_id)         })         .flat_map(&#124;(_, seqs)&#124; seqs.iter().copied())         .collect::<std::collections::BTreeSet<_>>` | [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; {             settled_outputs.contains(*attempt)                 && attempt_epoch.get(*attempt).is_some_and(&#124;id&#124; id == epoch_id)         })         .flat_map` | [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get(*attempt).is_some_and` | [265](../../src/context_projection.rs#L265) | receiver-type-required |
| `project_provider_context_mode` | `dialect.server_managed` | [269](../../src/context_projection.rs#L269) | receiver-type-required |
| `project_provider_context_mode` | `continuation_id.is_some` | [270](../../src/context_projection.rs#L270) | receiver-type-required |
| `project_provider_context_mode` | `epoch_seq.ok_or` | [271](../../src/context_projection.rs#L271) | receiver-type-required |
| `project_provider_context_mode` | `Vec::new` | [272](../../src/context_projection.rs#L272), [302](../../src/context_projection.rs#L302) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `items.extend` | [274](../../src/context_projection.rs#L274) | receiver-type-required |
| `project_provider_context_mode` | `seed_provider_items` | [274](../../src/context_projection.rs#L274) | [tekes-worker::context_projection::seed_provider_items](../../src/context_projection.rs#L519) |
| `project_provider_context_mode` | `superseded.contains` | [279](../../src/context_projection.rs#L279), [362](../../src/context_projection.rs#L362), [370](../../src/context_projection.rs#L370), [410](../../src/context_projection.rs#L410) | receiver-type-required |
| `project_provider_context_mode` | `compacted.contains` | [280](../../src/context_projection.rs#L280), [410](../../src/context_projection.rs#L410) | receiver-type-required |
| `project_provider_context_mode` | `materialize_string` | [286](../../src/context_projection.rs#L286) | [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L827) |
| `project_provider_context_mode` | `object.get("summary").ok_or` | [288](../../src/context_projection.rs#L288) | receiver-type-required |
| `project_provider_context_mode` | `items.push` | [290](../../src/context_projection.rs#L290), [506](../../src/context_projection.rs#L506) | receiver-type-required |
| `project_provider_context_mode` | `ijson` | [290](../../src/context_projection.rs#L290), [506](../../src/context_projection.rs#L506) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `projection         .events         .partition_point` | [298](../../src/context_projection.rs#L298) | receiver-type-required |
| `project_provider_context_mode` | `compaction_anchor_sequences` | [301](../../src/context_projection.rs#L301) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `seqs_from_ranges(object.get("supersedes"))?             .into_iter()             .max` | [320](../../src/context_projection.rs#L320) | receiver-type-required |
| `project_provider_context_mode` | `seqs_from_ranges(object.get("supersedes"))?             .into_iter` | [320](../../src/context_projection.rs#L320) | receiver-type-required |
| `project_provider_context_mode` | `replaced_result.insert` | [324](../../src/context_projection.rs#L324) | receiver-type-required |
| `project_provider_context_mode` | `replaced_result.get` | [329](../../src/context_projection.rs#L329) | receiver-type-required |
| `project_provider_context_mode` | `render_anchor` | [338](../../src/context_projection.rs#L338), [343](../../src/context_projection.rs#L343) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `output_positions.get` | [339](../../src/context_projection.rs#L339), [385](../../src/context_projection.rs#L385) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts.get` | [339](../../src/context_projection.rs#L339) | receiver-type-required |
| `project_provider_context_mode` | `(*output > anchor).then_some` | [340](../../src/context_projection.rs#L340) | receiver-type-required |
| `project_provider_context_mode` | `input_position.get` | [344](../../src/context_projection.rs#L344) | receiver-type-required |
| `project_provider_context_mode` | `deferred_result_position` | [346](../../src/context_projection.rs#L346) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `steer_seqs         .iter()         .copied()         .filter(&#124;seq&#124; !superseded.contains(seq) && !input_position.contains_key(seq))         .collect::<Vec<_>>` | [359](../../src/context_projection.rs#L359) | receiver-type-required |
| `project_provider_context_mode` | `steer_seqs         .iter()         .copied()         .filter` | [359](../../src/context_projection.rs#L359) | receiver-type-required |
| `project_provider_context_mode` | `steer_seqs         .iter()         .copied` | [359](../../src/context_projection.rs#L359) | receiver-type-required |
| `project_provider_context_mode` | `steer_seqs         .iter` | [359](../../src/context_projection.rs#L359) | receiver-type-required |
| `project_provider_context_mode` | `input_position.contains_key` | [362](../../src/context_projection.rs#L362) | receiver-type-required |
| `project_provider_context_mode` | `steers.is_empty` | [364](../../src/context_projection.rs#L364) | receiver-type-required |
| `project_provider_context_mode` | `projection             .events             .iter()             .filter(&#124;event&#124; {                 !superseded.contains(&event.seq())                     && match event.kind() {                         EventKind::ChildResult => true,                         EventKind::ToolResult => event                             .string_field("call")                             .is_some_and(&#124;call&#124; !spawned_calls.contains(call)),                         _ => false,                     }             })             .filter_map(&#124;event&#124; {                 let call = event.string_field("call")?;                 // An attempt's sealed output carries its calls; before one                 // exists the call renders at its own event.                 let call_position = call_attempts                     .get(call)                     .and_then(&#124;attempt&#124; output_positions.get(attempt))                     .or_else(&#124;&#124; call_seqs.get(call))?;                 Some((*call_position, base_key(event).0))             })             .collect::<Vec<_>>` | [366](../../src/context_projection.rs#L366) | receiver-type-required |
| `project_provider_context_mode` | `projection             .events             .iter()             .filter(&#124;event&#124; {                 !superseded.contains(&event.seq())                     && match event.kind() {                         EventKind::ChildResult => true,                         EventKind::ToolResult => event                             .string_field("call")                             .is_some_and(&#124;call&#124; !spawned_calls.contains(call)),                         _ => false,                     }             })             .filter_map` | [366](../../src/context_projection.rs#L366) | receiver-type-required |
| `project_provider_context_mode` | `projection             .events             .iter()             .filter` | [366](../../src/context_projection.rs#L366) | receiver-type-required |
| `project_provider_context_mode` | `projection             .events             .iter` | [366](../../src/context_projection.rs#L366) | receiver-type-required |
| `project_provider_context_mode` | `event                             .string_field("call")                             .is_some_and` | [373](../../src/context_projection.rs#L373) | receiver-type-required |
| `project_provider_context_mode` | `event                             .string_field` | [373](../../src/context_projection.rs#L373) | receiver-type-required |
| `project_provider_context_mode` | `spawned_calls.contains` | [375](../../src/context_projection.rs#L375), [431](../../src/context_projection.rs#L431) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts                     .get(call)                     .and_then(&#124;attempt&#124; output_positions.get(attempt))                     .or_else` | [383](../../src/context_projection.rs#L383) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts                     .get(call)                     .and_then` | [383](../../src/context_projection.rs#L383) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts                     .get` | [383](../../src/context_projection.rs#L383) | receiver-type-required |
| `project_provider_context_mode` | `call_seqs.get` | [386](../../src/context_projection.rs#L386) | receiver-type-required |
| `project_provider_context_mode` | `base_key` | [387](../../src/context_projection.rs#L387), [404](../../src/context_projection.rs#L404) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `spans                 .iter()                 .filter(&#124;(call, result)&#124; *call < steer && steer < *result)                 .map(&#124;(_, result)&#124; *result)                 .max` | [391](../../src/context_projection.rs#L391) | receiver-type-required |
| `project_provider_context_mode` | `spans                 .iter()                 .filter(&#124;(call, result)&#124; *call < steer && steer < *result)                 .map` | [391](../../src/context_projection.rs#L391) | receiver-type-required |
| `project_provider_context_mode` | `spans                 .iter()                 .filter` | [391](../../src/context_projection.rs#L391) | receiver-type-required |
| `project_provider_context_mode` | `spans                 .iter` | [391](../../src/context_projection.rs#L391) | receiver-type-required |
| `project_provider_context_mode` | `steer_anchor.insert` | [397](../../src/context_projection.rs#L397) | receiver-type-required |
| `project_provider_context_mode` | `projection.events.iter().collect::<Vec<_>>` | [401](../../src/context_projection.rs#L401) | receiver-type-required |
| `project_provider_context_mode` | `projection.events.iter` | [401](../../src/context_projection.rs#L401) | receiver-type-required |
| `project_provider_context_mode` | `ordered_events.sort_by_key` | [402](../../src/context_projection.rs#L402) | receiver-type-required |
| `project_provider_context_mode` | `steer_anchor.get` | [402](../../src/context_projection.rs#L402) | receiver-type-required |
| `project_provider_context_mode` | `semantic_anchors.contains` | [409](../../src/context_projection.rs#L409) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field("attempt")                 .is_some_and` | [417](../../src/context_projection.rs#L417) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field` | [417](../../src/context_projection.rs#L417), [429](../../src/context_projection.rs#L429) | receiver-type-required |
| `project_provider_context_mode` | `partial_carriers.contains` | [419](../../src/context_projection.rs#L419), [491](../../src/context_projection.rs#L491) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field("call")                 .is_some_and` | [429](../../src/context_projection.rs#L429) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.get(&event.seq()).is_some_and` | [454](../../src/context_projection.rs#L454) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.get` | [454](../../src/context_projection.rs#L454) | receiver-type-required |
| `project_provider_context_mode` | `admitted_all.contains` | [456](../../src/context_projection.rs#L456) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then(Value::as_str)                 .and_then(&#124;call&#124; call_turn.get(call))                 .is_some_and` | [461](../../src/context_projection.rs#L461), [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then(Value::as_str)                 .and_then` | [461](../../src/context_projection.rs#L461), [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then` | [461](../../src/context_projection.rs#L461), [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get` | [461](../../src/context_projection.rs#L461), [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `project_provider_context_mode` | `call_turn.get` | [464](../../src/context_projection.rs#L464), [476](../../src/context_projection.rs#L476) | receiver-type-required |
| `project_provider_context_mode` | `event.turn().is_none_or` | [466](../../src/context_projection.rs#L466) | receiver-type-required |
| `project_provider_context_mode` | `input_turn                 .get(&event.seq())                 .is_some_and` | [470](../../src/context_projection.rs#L470) | receiver-type-required |
| `project_provider_context_mode` | `input_turn                 .get` | [470](../../src/context_projection.rs#L470) | receiver-type-required |
| `project_provider_context_mode` | `epoch_pending.contains` | [481](../../src/context_projection.rs#L481) | receiver-type-required |
| `project_provider_context_mode` | `admitted.contains` | [484](../../src/context_projection.rs#L484), [503](../../src/context_projection.rs#L503) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field("attempt").is_some_and` | [490](../../src/context_projection.rs#L490) | receiver-type-required |
| `project_provider_context_mode` | `render_event_item(ledger, event, object, &call_name, dialect)?.ok_or_else` | [497](../../src/context_projection.rs#L497) | receiver-type-required |
| `project_provider_context_mode` | `render_event_item` | [497](../../src/context_projection.rs#L497) | [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L663) |
| `project_provider_context_mode` | `admits.push` | [504](../../src/context_projection.rs#L504) | receiver-type-required |
| `project_provider_context_mode` | `admits.sort_unstable` | [508](../../src/context_projection.rs#L508) | receiver-type-required |
| `project_provider_context_mode` | `admits.dedup` | [509](../../src/context_projection.rs#L509) | receiver-type-required |
| `project_provider_context_mode` | `Ok` | [510](../../src/context_projection.rs#L510) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `incremental_continuation             .then_some(continuation_id)             .flatten` | [513](../../src/context_projection.rs#L513) | receiver-type-required |
| `project_provider_context_mode` | `incremental_continuation             .then_some` | [513](../../src/context_projection.rs#L513) | receiver-type-required |
| `seed_provider_items` | `ledger.projection().ok_or` | [522](../../src/context_projection.rs#L522) | receiver-type-required |
| `seed_provider_items` | `ledger.projection` | [522](../../src/context_projection.rs#L522) | receiver-type-required |
| `seed_provider_items` | `projection.events.first` | [523](../../src/context_projection.rs#L523) | receiver-type-required |
| `seed_provider_items` | `Ok` | [524](../../src/context_projection.rs#L524), [528](../../src/context_projection.rs#L528), [619](../../src/context_projection.rs#L619) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `Vec::new` | [524](../../src/context_projection.rs#L524), [528](../../src/context_projection.rs#L528), [532](../../src/context_projection.rs#L532) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `serde_json::to_value` | [526](../../src/context_projection.rs#L526), [545](../../src/context_projection.rs#L545) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `genesis.raw` | [526](../../src/context_projection.rs#L526) | receiver-type-required |
| `seed_provider_items` | `raw.pointer("/seed/snapshot/asset").and_then` | [527](../../src/context_projection.rs#L527) | receiver-type-required |
| `seed_provider_items` | `raw.pointer` | [527](../../src/context_projection.rs#L527) | receiver-type-required |
| `seed_provider_items` | `ledger.path().parent().ok_or` | [530](../../src/context_projection.rs#L530) | receiver-type-required |
| `seed_provider_items` | `ledger.path().parent` | [530](../../src/context_projection.rs#L530) | receiver-type-required |
| `seed_provider_items` | `ledger.path` | [530](../../src/context_projection.rs#L530) | receiver-type-required |
| `seed_provider_items` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [531](../../src/context_projection.rs#L531) | receiver-type-required |
| `seed_provider_items` | `store::AssetStore::new` | [531](../../src/context_projection.rs#L531) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `seed_provider_items` | `folder.join` | [531](../../src/context_projection.rs#L531) | receiver-type-required |
| `seed_provider_items` | `bytes.split_inclusive` | [533](../../src/context_projection.rs#L533) | receiver-type-required |
| `seed_provider_items` | `line.strip_suffix(b"\n").ok_or` | [534](../../src/context_projection.rs#L534) | receiver-type-required |
| `seed_provider_items` | `line.strip_suffix` | [534](../../src/context_projection.rs#L534) | receiver-type-required |
| `seed_provider_items` | `canonical.is_empty` | [535](../../src/context_projection.rs#L535) | receiver-type-required |
| `seed_provider_items` | `Err` | [536](../../src/context_projection.rs#L536), [610](../../src/context_projection.rs#L610) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `"seed snapshot has an empty line".into` | [536](../../src/context_projection.rs#L536) | receiver-type-required |
| `seed_provider_items` | `Event::decode_canonical` | [538](../../src/context_projection.rs#L538) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `event.raw` | [545](../../src/context_projection.rs#L545) | receiver-type-required |
| `seed_provider_items` | `raw.as_object().ok_or` | [546](../../src/context_projection.rs#L546) | receiver-type-required |
| `seed_provider_items` | `raw.as_object` | [546](../../src/context_projection.rs#L546) | receiver-type-required |
| `seed_provider_items` | `event.kind` | [547](../../src/context_projection.rs#L547) | receiver-type-required |
| `seed_provider_items` | `event                     .string_field("call")                     .ok_or` | [561](../../src/context_projection.rs#L561) | receiver-type-required |
| `seed_provider_items` | `event                     .string_field` | [561](../../src/context_projection.rs#L561) | receiver-type-required |
| `seed_provider_items` | `object                     .get("outcome")                     .ok_or` | [564](../../src/context_projection.rs#L564) | receiver-type-required |
| `seed_provider_items` | `object                     .get` | [564](../../src/context_projection.rs#L564), [586](../../src/context_projection.rs#L586) | receiver-type-required |
| `seed_provider_items` | `event.string_field` | [585](../../src/context_projection.rs#L585) | receiver-type-required |
| `seed_provider_items` | `Some` | [585](../../src/context_projection.rs#L585) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `object                     .get("payload")                     .and_then(Value::as_object)                     .ok_or` | [586](../../src/context_projection.rs#L586) | receiver-type-required |
| `seed_provider_items` | `object                     .get("payload")                     .and_then` | [586](../../src/context_projection.rs#L586) | receiver-type-required |
| `seed_provider_items` | `format!(                     "model-visible seed kind {} has no provider projection",                     other.as_str()                 )                 .into` | [610](../../src/context_projection.rs#L610) | receiver-type-required |
| `seed_provider_items` | `items.push` | [617](../../src/context_projection.rs#L617) | receiver-type-required |
| `seed_provider_items` | `ijson` | [617](../../src/context_projection.rs#L617) | external-constructor-callback-or-unresolved |
| `provider_wire_call_id` | `ledger         .projection()         .and_then(&#124;projection&#124; {             projection.events.iter().find(&#124;event&#124; {                 *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)             })         })         .and_then(&#124;event&#124; event.string_field("provider_call"))         .unwrap_or` | [623](../../src/context_projection.rs#L623) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection()         .and_then(&#124;projection&#124; {             projection.events.iter().find(&#124;event&#124; {                 *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)             })         })         .and_then` | [623](../../src/context_projection.rs#L623) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection()         .and_then` | [623](../../src/context_projection.rs#L623) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection` | [623](../../src/context_projection.rs#L623) | receiver-type-required |
| `provider_wire_call_id` | `projection.events.iter().find` | [626](../../src/context_projection.rs#L626) | receiver-type-required |
| `provider_wire_call_id` | `projection.events.iter` | [626](../../src/context_projection.rs#L626) | receiver-type-required |
| `provider_wire_call_id` | `event.kind` | [627](../../src/context_projection.rs#L627) | receiver-type-required |
| `provider_wire_call_id` | `event.string_field` | [627](../../src/context_projection.rs#L627), [630](../../src/context_projection.rs#L630) | receiver-type-required |
| `provider_wire_call_id` | `Some` | [627](../../src/context_projection.rs#L627) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `object         .get("sealed")         .and_then(Value::as_object)         .ok_or_else` | [642](../../src/context_projection.rs#L642) | receiver-type-required |
| `render_sealed_carrier` | `object         .get("sealed")         .and_then` | [642](../../src/context_projection.rs#L642) | receiver-type-required |
| `render_sealed_carrier` | `object         .get` | [642](../../src/context_projection.rs#L642) | receiver-type-required |
| `render_sealed_carrier` | `sealed.get("version").and_then` | [646](../../src/context_projection.rs#L646) | receiver-type-required |
| `render_sealed_carrier` | `sealed.get` | [646](../../src/context_projection.rs#L646), [647](../../src/context_projection.rs#L647) | receiver-type-required |
| `render_sealed_carrier` | `Some` | [646](../../src/context_projection.rs#L646), [647](../../src/context_projection.rs#L647) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `sealed.get("adapter").and_then` | [647](../../src/context_projection.rs#L647) | receiver-type-required |
| `render_sealed_carrier` | `dialect.as_str` | [647](../../src/context_projection.rs#L647) | receiver-type-required |
| `render_sealed_carrier` | `Err` | [649](../../src/context_projection.rs#L649) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `format!("{kind} sealed carrier is incompatible with the active adapter").into` | [650](../../src/context_projection.rs#L650) | receiver-type-required |
| `render_sealed_carrier` | `materialize_string` | [653](../../src/context_projection.rs#L653) | [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L827) |
| `render_sealed_carrier` | `sealed             .get("fragments")             .ok_or` | [655](../../src/context_projection.rs#L655) | receiver-type-required |
| `render_sealed_carrier` | `sealed             .get` | [655](../../src/context_projection.rs#L655) | receiver-type-required |
| `render_sealed_carrier` | `serde_json::to_value` | [659](../../src/context_projection.rs#L659) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `IJsonValue::parse` | [659](../../src/context_projection.rs#L659) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `fragments.as_bytes` | [659](../../src/context_projection.rs#L659) | receiver-type-required |
| `render_sealed_carrier` | `Ok` | [660](../../src/context_projection.rs#L660) | external-constructor-callback-or-unresolved |
| `render_event_item` | `event.kind` | [670](../../src/context_projection.rs#L670) | receiver-type-required |
| `render_event_item` | `render_sealed_carrier` | [675](../../src/context_projection.rs#L675), [678](../../src/context_projection.rs#L678) | [tekes-worker::context_projection::render_sealed_carrier](../../src/context_projection.rs#L636) |
| `render_event_item` | `object.contains_key` | [677](../../src/context_projection.rs#L677) | receiver-type-required |
| `render_event_item` | `event.string_field("call").ok_or` | [694](../../src/context_projection.rs#L694) | receiver-type-required |
| `render_event_item` | `event.string_field` | [694](../../src/context_projection.rs#L694) | receiver-type-required |
| `render_event_item` | `object.get("outcome").ok_or` | [695](../../src/context_projection.rs#L695) | receiver-type-required |
| `render_event_item` | `object.get` | [695](../../src/context_projection.rs#L695), [697](../../src/context_projection.rs#L697) | receiver-type-required |
| `render_event_item` | `call_names.get(call).map(String::as_str).unwrap_or` | [696](../../src/context_projection.rs#L696) | receiver-type-required |
| `render_event_item` | `call_names.get(call).map` | [696](../../src/context_projection.rs#L696) | receiver-type-required |
| `render_event_item` | `call_names.get` | [696](../../src/context_projection.rs#L696) | receiver-type-required |
| `render_event_item` | `render_blocks` | [698](../../src/context_projection.rs#L698) | [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L748) |
| `render_event_item` | `blocks.as_mut_slice` | [701](../../src/context_projection.rs#L701) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then(Value::as_str)                         .filter(&#124;_&#124; block.get("type").and_then(Value::as_str) == Some("text"))                         .and_then` | [702](../../src/context_projection.rs#L702) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then(Value::as_str)                         .filter` | [702](../../src/context_projection.rs#L702) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then` | [702](../../src/context_projection.rs#L702) | receiver-type-required |
| `render_event_item` | `block                         .get` | [702](../../src/context_projection.rs#L702) | receiver-type-required |
| `render_event_item` | `block.get("type").and_then` | [705](../../src/context_projection.rs#L705) | receiver-type-required |
| `render_event_item` | `block.get` | [705](../../src/context_projection.rs#L705) | receiver-type-required |
| `render_event_item` | `Some` | [705](../../src/context_projection.rs#L705), [745](../../src/context_projection.rs#L745) | external-constructor-callback-or-unresolved |
| `render_event_item` | `result_presentation::present` | [706](../../src/context_projection.rs#L706) | external-constructor-callback-or-unresolved |
| `render_event_item` | `Value::Array` | [711](../../src/context_projection.rs#L711) | external-constructor-callback-or-unresolved |
| `render_event_item` | `event                 .string_field("call")                 .ok_or` | [726](../../src/context_projection.rs#L726) | receiver-type-required |
| `render_event_item` | `event                 .string_field` | [726](../../src/context_projection.rs#L726) | receiver-type-required |
| `render_event_item` | `Ok` | [743](../../src/context_projection.rs#L743), [745](../../src/context_projection.rs#L745) | external-constructor-callback-or-unresolved |
| `render_blocks` | `materialize_json` | [752](../../src/context_projection.rs#L752) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L799) |
| `render_blocks` | `value.as_array().ok_or` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `render_blocks` | `value.as_array` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `render_blocks` | `blocks         .iter()         .map(&#124;block&#124; {             let object = block.as_object().ok_or("block is not an object")?;             Ok(match object.get("type").and_then(Value::as_str) {                 Some("text") => json!({"type":"text","text":object.get("text").and_then(Value::as_str).ok_or("text block lacks text")?}),                 Some("reasoning") => json!({"type":"reasoning","text":object.get("text").and_then(Value::as_str).ok_or("reasoning block lacks text")?}),                 Some("image") => {                     let asset = object.get("asset").and_then(Value::as_str).ok_or("image block lacks asset")?;                     let folder = ledger.path().parent().ok_or("ledger has no folder")?;                     let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;                     json!({                         "type":"file","name":asset,                         "data":base64::engine::general_purpose::STANDARD.encode(bytes),                         "mime":object.get("mime").and_then(Value::as_str).ok_or("image block lacks mime")?                     })                 },                 Some("file") => {                     let asset = object.get("asset").and_then(Value::as_str).ok_or("file block lacks asset")?;                     let folder = ledger.path().parent().ok_or("ledger has no folder")?;                     let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;                     json!({                         "type":"file",                         "name":object.get("name").and_then(Value::as_str).ok_or("file block lacks name")?,                         "mime":object.get("mime").and_then(Value::as_str).ok_or("file block lacks mime")?,                         "bytes":bytes.len(),                         "data":base64::engine::general_purpose::STANDARD.encode(bytes)                     })                 },                 Some("tool-call") => json!({                     "type":"tool_call","call_id":object.get("call").and_then(Value::as_str).ok_or("nested call lacks id")?,                     "name":object.get("name").and_then(Value::as_str).ok_or("nested call lacks name")?,                     "arguments":object.get("args").cloned().ok_or("nested call lacks args")?                 }),                 Some("tool-result") => json!({                     "type":"tool_result","call_id":object.get("call").and_then(Value::as_str).ok_or("nested result lacks call")?,                     "name":"tool","result":object.get("content").cloned().ok_or("nested result lacks content")?,                     "error":object.get("error").and_then(Value::as_bool).unwrap_or(false)                 }),                 _ => return Err("unknown block kind".into()),             })         })         .collect` | [754](../../src/context_projection.rs#L754) | receiver-type-required |
| `render_blocks` | `blocks         .iter()         .map` | [754](../../src/context_projection.rs#L754) | receiver-type-required |
| `render_blocks` | `blocks         .iter` | [754](../../src/context_projection.rs#L754) | receiver-type-required |
| `render_blocks` | `block.as_object().ok_or` | [757](../../src/context_projection.rs#L757) | receiver-type-required |
| `render_blocks` | `block.as_object` | [757](../../src/context_projection.rs#L757) | receiver-type-required |
| `render_blocks` | `Ok` | [758](../../src/context_projection.rs#L758) | external-constructor-callback-or-unresolved |
| `render_blocks` | `object.get("type").and_then` | [758](../../src/context_projection.rs#L758) | receiver-type-required |
| `render_blocks` | `object.get` | [758](../../src/context_projection.rs#L758), [762](../../src/context_projection.rs#L762), [772](../../src/context_projection.rs#L772) | receiver-type-required |
| `render_blocks` | `object.get("asset").and_then(Value::as_str).ok_or` | [762](../../src/context_projection.rs#L762), [772](../../src/context_projection.rs#L772) | receiver-type-required |
| `render_blocks` | `object.get("asset").and_then` | [762](../../src/context_projection.rs#L762), [772](../../src/context_projection.rs#L772) | receiver-type-required |
| `render_blocks` | `ledger.path().parent().ok_or` | [763](../../src/context_projection.rs#L763), [773](../../src/context_projection.rs#L773) | receiver-type-required |
| `render_blocks` | `ledger.path().parent` | [763](../../src/context_projection.rs#L763), [773](../../src/context_projection.rs#L773) | receiver-type-required |
| `render_blocks` | `ledger.path` | [763](../../src/context_projection.rs#L763), [773](../../src/context_projection.rs#L773) | receiver-type-required |
| `render_blocks` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [764](../../src/context_projection.rs#L764), [774](../../src/context_projection.rs#L774) | receiver-type-required |
| `render_blocks` | `store::AssetStore::new` | [764](../../src/context_projection.rs#L764), [774](../../src/context_projection.rs#L774) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `render_blocks` | `folder.join` | [764](../../src/context_projection.rs#L764), [774](../../src/context_projection.rs#L774) | receiver-type-required |
| `render_blocks` | `Err` | [793](../../src/context_projection.rs#L793) | external-constructor-callback-or-unresolved |
| `render_blocks` | `"unknown block kind".into` | [793](../../src/context_projection.rs#L793) | receiver-type-required |
| `materialize_json` | `materialize_json_at` | [803](../../src/context_projection.rs#L803) | [tekes-worker::context_projection::materialize_json_at](../../src/context_projection.rs#L806) |
| `materialize_json` | `ledger.path` | [803](../../src/context_projection.rs#L803) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("$spill"))         .and_then` | [810](../../src/context_projection.rs#L810) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then` | [810](../../src/context_projection.rs#L810) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter` | [810](../../src/context_projection.rs#L810) | receiver-type-required |
| `materialize_json_at` | `value         .as_object` | [810](../../src/context_projection.rs#L810) | receiver-type-required |
| `materialize_json_at` | `object.len` | [812](../../src/context_projection.rs#L812) | receiver-type-required |
| `materialize_json_at` | `object.get` | [813](../../src/context_projection.rs#L813) | receiver-type-required |
| `materialize_json_at` | `Ok` | [816](../../src/context_projection.rs#L816), [824](../../src/context_projection.rs#L824) | external-constructor-callback-or-unresolved |
| `materialize_json_at` | `value.clone` | [816](../../src/context_projection.rs#L816) | receiver-type-required |
| `materialize_json_at` | `spill         .get("asset")         .and_then(Value::as_str)         .ok_or` | [818](../../src/context_projection.rs#L818) | receiver-type-required |
| `materialize_json_at` | `spill         .get("asset")         .and_then` | [818](../../src/context_projection.rs#L818) | receiver-type-required |
| `materialize_json_at` | `spill         .get` | [818](../../src/context_projection.rs#L818) | receiver-type-required |
| `materialize_json_at` | `ledger_path.parent().ok_or` | [822](../../src/context_projection.rs#L822) | receiver-type-required |
| `materialize_json_at` | `ledger_path.parent` | [822](../../src/context_projection.rs#L822) | receiver-type-required |
| `materialize_json_at` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [823](../../src/context_projection.rs#L823) | receiver-type-required |
| `materialize_json_at` | `store::AssetStore::new` | [823](../../src/context_projection.rs#L823) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `materialize_json_at` | `folder.join` | [823](../../src/context_projection.rs#L823) | receiver-type-required |
| `materialize_json_at` | `serde_json::to_value` | [824](../../src/context_projection.rs#L824) | external-constructor-callback-or-unresolved |
| `materialize_json_at` | `IJsonValue::parse` | [824](../../src/context_projection.rs#L824) | external-constructor-callback-or-unresolved |
| `materialize_string` | `materialize_json` | [831](../../src/context_projection.rs#L831) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L799) |
| `materialize_string` | `value         .as_str()         .map(str::to_owned)         .ok_or_else` | [832](../../src/context_projection.rs#L832) | receiver-type-required |
| `materialize_string` | `value         .as_str()         .map` | [832](../../src/context_projection.rs#L832) | receiver-type-required |
| `materialize_string` | `value         .as_str` | [832](../../src/context_projection.rs#L832) | receiver-type-required |
| `materialize_string` | `"spilled value is not a string".into` | [835](../../src/context_projection.rs#L835) | receiver-type-required |
| `seqs_from_ranges` | `Vec::new` | [839](../../src/context_projection.rs#L839) | external-constructor-callback-or-unresolved |
| `seqs_from_ranges` | `value.and_then(Value::as_array).into_iter().flatten` | [840](../../src/context_projection.rs#L840) | receiver-type-required |
| `seqs_from_ranges` | `value.and_then(Value::as_array).into_iter` | [840](../../src/context_projection.rs#L840) | receiver-type-required |
| `seqs_from_ranges` | `value.and_then` | [840](../../src/context_projection.rs#L840) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("from")             .and_then(Value::as_u64)             .ok_or` | [841](../../src/context_projection.rs#L841) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("from")             .and_then` | [841](../../src/context_projection.rs#L841) | receiver-type-required |
| `seqs_from_ranges` | `range             .get` | [841](../../src/context_projection.rs#L841), [845](../../src/context_projection.rs#L845) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("to")             .and_then(Value::as_u64)             .ok_or` | [845](../../src/context_projection.rs#L845) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("to")             .and_then` | [845](../../src/context_projection.rs#L845) | receiver-type-required |
| `seqs_from_ranges` | `seqs.extend` | [849](../../src/context_projection.rs#L849) | receiver-type-required |
| `seqs_from_ranges` | `Ok` | [851](../../src/context_projection.rs#L851) | external-constructor-callback-or-unresolved |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map(&#124;source&#124; source.content.as_str())         .collect::<Vec<_>>()         .join` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map(&#124;source&#124; source.content.as_str())         .collect::<Vec<_>>` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter` | [855](../../src/context_projection.rs#L855) | receiver-type-required |
| `effective_system` | `instruction.sources.get` | [859](../../src/context_projection.rs#L859) | receiver-type-required |
| `effective_system` | `source.content.as_str` | [861](../../src/context_projection.rs#L861) | receiver-type-required |
| `epoch_open_reason` | `previous.integer_field` | [897](../../src/context_projection.rs#L897) | receiver-type-required |
| `epoch_open_reason` | `Some` | [897](../../src/context_projection.rs#L897), [908](../../src/context_projection.rs#L908), [911](../../src/context_projection.rs#L911) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `serde_json::to_value(previous.raw()).ok` | [900](../../src/context_projection.rs#L900) | receiver-type-required |
| `epoch_open_reason` | `serde_json::to_value` | [900](../../src/context_projection.rs#L900) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `previous.raw` | [900](../../src/context_projection.rs#L900) | receiver-type-required |
| `epoch_open_reason` | `head.as_ref().and_then` | [902](../../src/context_projection.rs#L902) | receiver-type-required |
| `epoch_open_reason` | `head.as_ref` | [902](../../src/context_projection.rs#L902) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer(pointer)                 .and_then(Value::as_str)                 .map` | [903](../../src/context_projection.rs#L903) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer(pointer)                 .and_then` | [903](../../src/context_projection.rs#L903) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer` | [903](../../src/context_projection.rs#L903) | receiver-type-required |
| `epoch_open_reason` | `field("/tools/digest").as_deref` | [908](../../src/context_projection.rs#L908) | receiver-type-required |
| `epoch_open_reason` | `field` | [908](../../src/context_projection.rs#L908), [911](../../src/context_projection.rs#L911) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `field("/system/digest").as_deref` | [911](../../src/context_projection.rs#L911) | receiver-type-required |
| `append_tool_result_trim` | `ledger.projection().ok_or` | [927](../../src/context_projection.rs#L927) | receiver-type-required |
| `append_tool_result_trim` | `ledger.projection` | [927](../../src/context_projection.rs#L927) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter()         .find(&#124;event&#124; event.seq() == trim.seq)         .ok_or` | [928](../../src/context_projection.rs#L928) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter()         .find` | [928](../../src/context_projection.rs#L928) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter` | [928](../../src/context_projection.rs#L928) | receiver-type-required |
| `append_tool_result_trim` | `event.seq` | [931](../../src/context_projection.rs#L931) | receiver-type-required |
| `append_tool_result_trim` | `serde_json::to_value` | [933](../../src/context_projection.rs#L933) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `original.raw` | [933](../../src/context_projection.rs#L933) | receiver-type-required |
| `append_tool_result_trim` | `raw.as_object().ok_or` | [934](../../src/context_projection.rs#L934) | receiver-type-required |
| `append_tool_result_trim` | `raw.as_object` | [934](../../src/context_projection.rs#L934) | receiver-type-required |
| `append_tool_result_trim` | `materialize_json` | [939](../../src/context_projection.rs#L939) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L799) |
| `append_tool_result_trim` | `object.get("content").ok_or` | [941](../../src/context_projection.rs#L941) | receiver-type-required |
| `append_tool_result_trim` | `object.get` | [941](../../src/context_projection.rs#L941), [951](../../src/context_projection.rs#L951) | receiver-type-required |
| `append_tool_result_trim` | `engine::trimmed_tool_result_content` | [943](../../src/context_projection.rs#L943) | [engine::context::trimmed_tool_result_content](../../../engine/src/context.rs#L502) |
| `append_tool_result_trim` | `replacement             .as_object_mut()             .expect("replacement object")             .insert` | [952](../../src/context_projection.rs#L952) | receiver-type-required |
| `append_tool_result_trim` | `replacement             .as_object_mut()             .expect` | [952](../../src/context_projection.rs#L952) | receiver-type-required |
| `append_tool_result_trim` | `replacement             .as_object_mut` | [952](../../src/context_projection.rs#L952) | receiver-type-required |
| `append_tool_result_trim` | `"meta".to_owned` | [955](../../src/context_projection.rs#L955) | receiver-type-required |
| `append_tool_result_trim` | `meta.clone` | [955](../../src/context_projection.rs#L955) | receiver-type-required |
| `append_tool_result_trim` | `ledger.append_contract` | [957](../../src/context_projection.rs#L957) | receiver-type-required |
| `append_tool_result_trim` | `make_event` | [957](../../src/context_projection.rs#L957) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `BarrierContext::default` | [957](../../src/context_projection.rs#L957) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `Ok` | [958](../../src/context_projection.rs#L958) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `ledger.path().parent().ok_or` | [975](../../src/context_projection.rs#L975) | receiver-type-required |
| `append_provider_epoch` | `ledger.path().parent` | [975](../../src/context_projection.rs#L975) | receiver-type-required |
| `append_provider_epoch` | `ledger.path` | [975](../../src/context_projection.rs#L975) | receiver-type-required |
| `append_provider_epoch` | `store::AssetStore::new` | [976](../../src/context_projection.rs#L976) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `append_provider_epoch` | `folder.join` | [976](../../src/context_projection.rs#L976) | receiver-type-required |
| `append_provider_epoch` | `profile.canonical_bytes` | [977](../../src/context_projection.rs#L977) | receiver-type-required |
| `append_provider_epoch` | `assets.publish` | [978](../../src/context_projection.rs#L978), [982](../../src/context_projection.rs#L982) | receiver-type-required |
| `append_provider_epoch` | `tools.canonical_bytes` | [981](../../src/context_projection.rs#L981) | receiver-type-required |
| `append_provider_epoch` | `make_event` | [983](../../src/context_projection.rs#L983) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `ledger.append_contract` | [1003](../../src/context_projection.rs#L1003) | receiver-type-required |
| `append_provider_epoch` | `BarrierContext::default` | [1003](../../src/context_projection.rs#L1003) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `Ok` | [1004](../../src/context_projection.rs#L1004) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .rev()         .find` | [1018](../../src/context_projection.rs#L1018) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .rev` | [1018](../../src/context_projection.rs#L1018) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter` | [1018](../../src/context_projection.rs#L1018), [1024](../../src/context_projection.rs#L1024) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection` | [1018](../../src/context_projection.rs#L1018), [1024](../../src/context_projection.rs#L1024) | receiver-type-required |
| `latest_compatible_epoch` | `event.kind` | [1023](../../src/context_projection.rs#L1023) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .any` | [1024](../../src/context_projection.rs#L1024) | receiver-type-required |
| `latest_compatible_epoch` | `later.seq` | [1028](../../src/context_projection.rs#L1028) | receiver-type-required |
| `latest_compatible_epoch` | `event.seq` | [1028](../../src/context_projection.rs#L1028) | receiver-type-required |
| `latest_compatible_epoch` | `later.kind` | [1028](../../src/context_projection.rs#L1028) | receiver-type-required |
| `latest_compatible_epoch` | `serde_json::to_value(event.raw()).ok` | [1032](../../src/context_projection.rs#L1032) | receiver-type-required |
| `latest_compatible_epoch` | `serde_json::to_value` | [1032](../../src/context_projection.rs#L1032) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `event.raw` | [1032](../../src/context_projection.rs#L1032) | receiver-type-required |
| `latest_compatible_epoch` | `(event.string_field("adapter") == Some(dialect.as_str())         && event.string_field("model") == Some(model)         && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)         && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)         && event.integer_field("renderer") == Some(renderer))     .then(&#124;&#124; event.string_field("id").map(str::to_owned))     .flatten` | [1033](../../src/context_projection.rs#L1033) | receiver-type-required |
| `latest_compatible_epoch` | `(event.string_field("adapter") == Some(dialect.as_str())         && event.string_field("model") == Some(model)         && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)         && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)         && event.integer_field("renderer") == Some(renderer))     .then` | [1033](../../src/context_projection.rs#L1033) | receiver-type-required |
| `latest_compatible_epoch` | `event.string_field` | [1033](../../src/context_projection.rs#L1033), [1034](../../src/context_projection.rs#L1034), [1038](../../src/context_projection.rs#L1038) | receiver-type-required |
| `latest_compatible_epoch` | `Some` | [1033](../../src/context_projection.rs#L1033), [1034](../../src/context_projection.rs#L1034), [1035](../../src/context_projection.rs#L1035), [1036](../../src/context_projection.rs#L1036), [1037](../../src/context_projection.rs#L1037) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `dialect.as_str` | [1033](../../src/context_projection.rs#L1033) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer("/system/digest").and_then` | [1035](../../src/context_projection.rs#L1035) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer` | [1035](../../src/context_projection.rs#L1035), [1036](../../src/context_projection.rs#L1036) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer("/tools/digest").and_then` | [1036](../../src/context_projection.rs#L1036) | receiver-type-required |
| `latest_compatible_epoch` | `event.integer_field` | [1037](../../src/context_projection.rs#L1037) | receiver-type-required |
| `latest_compatible_epoch` | `event.string_field("id").map` | [1038](../../src/context_projection.rs#L1038) | receiver-type-required |
| `ranges` | `Vec::new` | [1043](../../src/context_projection.rs#L1043) | external-constructor-callback-or-unresolved |
| `ranges` | `ranges.last_mut().and_then` | [1045](../../src/context_projection.rs#L1045) | receiver-type-required |
| `ranges` | `ranges.last_mut` | [1045](../../src/context_projection.rs#L1045) | receiver-type-required |
| `ranges` | `range.get("to").and_then` | [1047](../../src/context_projection.rs#L1047) | receiver-type-required |
| `ranges` | `range.get` | [1047](../../src/context_projection.rs#L1047) | receiver-type-required |
| `ranges` | `Some` | [1047](../../src/context_projection.rs#L1047) | external-constructor-callback-or-unresolved |
| `ranges` | `seq.saturating_sub` | [1047](../../src/context_projection.rs#L1047) | receiver-type-required |
| `ranges` | `range.insert` | [1049](../../src/context_projection.rs#L1049) | receiver-type-required |
| `ranges` | `"to".to_owned` | [1049](../../src/context_projection.rs#L1049) | receiver-type-required |
| `ranges` | `Value::from` | [1049](../../src/context_projection.rs#L1049) | external-constructor-callback-or-unresolved |
| `ranges` | `ranges.push` | [1051](../../src/context_projection.rs#L1051) | receiver-type-required |
