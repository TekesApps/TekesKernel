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
| [tekes-worker::context_projection::seed_provider_items](../../src/context_projection.rs#L462) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::provider_wire_call_id](../../src/context_projection.rs#L565) | function_item | `private` |  |
| [tekes-worker::context_projection::render_sealed_carrier](../../src/context_projection.rs#L579) | function_item | `private` |  |
| [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L606) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L691) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L742) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_json_at](../../src/context_projection.rs#L749) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L770) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::seqs_from_ranges](../../src/context_projection.rs#L781) | function_item | `private` |  |
| [tekes-worker::context_projection::effective_system](../../src/context_projection.rs#L797) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::EpochAppend](../../src/context_projection.rs#L809) | struct_item | `pub(crate)` |  |
| [tekes-worker::context_projection::epoch_open_reason](../../src/context_projection.rs#L824) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::append_tool_result_trim](../../src/context_projection.rs#L864) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::append_provider_epoch](../../src/context_projection.rs#L904) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::latest_compatible_epoch](../../src/context_projection.rs#L950) | function_item | `pub(crate)` |  |
| [tekes-worker::context_projection::ranges](../../src/context_projection.rs#L985) | function_item | `pub(crate)` |  |

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
  n2["tekes-worker::context_projection::project_provider_context_mode"]
  n3["tekes-worker::context_projection::native_search_references"]
  n4["tekes-worker::context_projection::seed_provider_items"]
  n5["tekes-worker::context_projection::dynamic_catalog_revision"]
  n6["tekes-worker::context_projection::provider_wire_call_id"]
  n7["tekes-worker::context_projection::render_sealed_carrier"]
  n8["tekes-worker::context_projection::render_event_item"]
  n9["tekes-worker::context_projection::render_blocks"]
  n10["tekes-worker::context_projection::materialize_json"]
  n11["tekes-worker::context_projection::materialize_json_at"]
  n12["tekes-worker::context_projection::materialize_string"]
  n13["tekes-worker::context_projection::seqs_from_ranges"]
  n14["tekes-worker::context_projection::effective_system"]
  n15["tekes-worker::context_projection::epoch_open_reason"]
  n16["tekes-worker::context_projection::append_tool_result_trim"]
  n17["tekes-worker::context_projection::append_provider_epoch"]
  n18["tekes-worker::context_projection::latest_compatible_epoch"]
  n19["tekes-worker::context_projection::ranges"]
  n2 --> n4
  n2 --> n8
  n2 --> n12
  n2 --> n13
  n3 --> n10
  n4 --> n1
  n7 --> n12
  n8 --> n7
  n8 --> n9
  n9 --> n1
  n9 --> n10
  n10 --> n11
  n11 --> n1
  n12 --> n10
  n16 --> n0
  n16 --> n10
  n17 --> n1
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
| `native_search_references` | `materialize_json` | [56](../../src/context_projection.rs#L56) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L742) |
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
| `project_provider_context_mode` | `std::collections::BTreeSet::<String>::new` | [114](../../src/context_projection.rs#L114), [118](../../src/context_projection.rs#L118), [129](../../src/context_projection.rs#L129) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeMap::<String, u64>::new` | [121](../../src/context_projection.rs#L121), [127](../../src/context_projection.rs#L127) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeSet::<u64>::new` | [123](../../src/context_projection.rs#L123), [124](../../src/context_projection.rs#L124), [133](../../src/context_projection.rs#L133) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `std::collections::BTreeMap::<u64, u64>::new` | [125](../../src/context_projection.rs#L125), [126](../../src/context_projection.rs#L126), [309](../../src/context_projection.rs#L309) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `serde_json::to_value` | [136](../../src/context_projection.rs#L136), [280](../../src/context_projection.rs#L280), [314](../../src/context_projection.rs#L314), [350](../../src/context_projection.rs#L350) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `event.raw` | [136](../../src/context_projection.rs#L136), [280](../../src/context_projection.rs#L280), [314](../../src/context_projection.rs#L314), [350](../../src/context_projection.rs#L350) | receiver-type-required |
| `project_provider_context_mode` | `raw.as_object().ok_or` | [137](../../src/context_projection.rs#L137), [281](../../src/context_projection.rs#L281), [315](../../src/context_projection.rs#L315), [351](../../src/context_projection.rs#L351) | receiver-type-required |
| `project_provider_context_mode` | `raw.as_object` | [137](../../src/context_projection.rs#L137), [281](../../src/context_projection.rs#L281), [315](../../src/context_projection.rs#L315), [351](../../src/context_projection.rs#L351) | receiver-type-required |
| `project_provider_context_mode` | `seqs_from_ranges` | [138](../../src/context_projection.rs#L138), [142](../../src/context_projection.rs#L142), [198](../../src/context_projection.rs#L198), [316](../../src/context_projection.rs#L316) | [tekes-worker::context_projection::seqs_from_ranges](../../src/context_projection.rs#L781) |
| `project_provider_context_mode` | `object.get` | [138](../../src/context_projection.rs#L138), [142](../../src/context_projection.rs#L142), [181](../../src/context_projection.rs#L181), [198](../../src/context_projection.rs#L198), [284](../../src/context_projection.rs#L284), [316](../../src/context_projection.rs#L316), [398](../../src/context_projection.rs#L398) | receiver-type-required |
| `project_provider_context_mode` | `superseded.insert` | [139](../../src/context_projection.rs#L139) | receiver-type-required |
| `project_provider_context_mode` | `event.kind` | [141](../../src/context_projection.rs#L141), [146](../../src/context_projection.rs#L146), [274](../../src/context_projection.rs#L274), [311](../../src/context_projection.rs#L311), [358](../../src/context_projection.rs#L358), [371](../../src/context_projection.rs#L371), [381](../../src/context_projection.rs#L381), [396](../../src/context_projection.rs#L396), [412](../../src/context_projection.rs#L412) | receiver-type-required |
| `project_provider_context_mode` | `compacted.insert` | [143](../../src/context_projection.rs#L143) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field` | [147](../../src/context_projection.rs#L147), [227](../../src/context_projection.rs#L227), [228](../../src/context_projection.rs#L228), [335](../../src/context_projection.rs#L335), [381](../../src/context_projection.rs#L381), [433](../../src/context_projection.rs#L433) | receiver-type-required |
| `project_provider_context_mode` | `Some` | [147](../../src/context_projection.rs#L147), [148](../../src/context_projection.rs#L148), [163](../../src/context_projection.rs#L163), [181](../../src/context_projection.rs#L181), [381](../../src/context_projection.rs#L381), [398](../../src/context_projection.rs#L398), [421](../../src/context_projection.rs#L421) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `event.seq` | [148](../../src/context_projection.rs#L148), [173](../../src/context_projection.rs#L173), [183](../../src/context_projection.rs#L183), [207](../../src/context_projection.rs#L207), [224](../../src/context_projection.rs#L224), [275](../../src/context_projection.rs#L275), [276](../../src/context_projection.rs#L276), [296](../../src/context_projection.rs#L296), [320](../../src/context_projection.rs#L320), [334](../../src/context_projection.rs#L334), [340](../../src/context_projection.rs#L340), [352](../../src/context_projection.rs#L352), [353](../../src/context_projection.rs#L353), [397](../../src/context_projection.rs#L397), [399](../../src/context_projection.rs#L399), [414](../../src/context_projection.rs#L414), [424](../../src/context_projection.rs#L424), [427](../../src/context_projection.rs#L427), [446](../../src/context_projection.rs#L446), [447](../../src/context_projection.rs#L447) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten()                     .flat_map(&#124;value&#124; value.as_array().into_iter().flatten())                     .filter_map` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten()                     .flat_map` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map(&#124;pending&#124; [pending.get("eligible"), pending.get("withheld")])                     .flatten` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter()                     .flat_map` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then(Value::as_object)                     .into_iter` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("pending")                     .and_then` | [149](../../src/context_projection.rs#L149) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get` | [149](../../src/context_projection.rs#L149), [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `pending.get` | [153](../../src/context_projection.rs#L153) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array().into_iter().flatten` | [155](../../src/context_projection.rs#L155) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array().into_iter` | [155](../../src/context_projection.rs#L155) | receiver-type-required |
| `project_provider_context_mode` | `value.as_array` | [155](../../src/context_projection.rs#L155) | receiver-type-required |
| `project_provider_context_mode` | `epoch_pending.insert` | [158](../../src/context_projection.rs#L158) | receiver-type-required |
| `project_provider_context_mode` | `event.turn().ok_or` | [162](../../src/context_projection.rs#L162), [231](../../src/context_projection.rs#L231) | receiver-type-required |
| `project_provider_context_mode` | `event.turn` | [162](../../src/context_projection.rs#L162), [177](../../src/context_projection.rs#L177), [231](../../src/context_projection.rs#L231), [409](../../src/context_projection.rs#L409), [421](../../src/context_projection.rs#L421) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter()                     .flatten()                     .filter_map` | [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter()                     .flatten` | [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then(Value::as_array)                     .into_iter` | [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then(&#124;trigger&#124; trigger.get("inputs"))                     .and_then` | [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `object                     .get("trigger")                     .and_then` | [164](../../src/context_projection.rs#L164) | receiver-type-required |
| `project_provider_context_mode` | `trigger.get` | [166](../../src/context_projection.rs#L166) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.insert` | [172](../../src/context_projection.rs#L172), [183](../../src/context_projection.rs#L183) | receiver-type-required |
| `project_provider_context_mode` | `input_position.insert` | [173](../../src/context_projection.rs#L173) | receiver-type-required |
| `project_provider_context_mode` | `object.get("steer").and_then` | [181](../../src/context_projection.rs#L181), [398](../../src/context_projection.rs#L398) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field("attempt")                     .ok_or("attempt lacks id")?                     .to_owned` | [187](../../src/context_projection.rs#L187) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field("attempt")                     .ok_or` | [187](../../src/context_projection.rs#L187), [201](../../src/context_projection.rs#L201), [218](../../src/context_projection.rs#L218) | receiver-type-required |
| `project_provider_context_mode` | `event                     .string_field` | [187](../../src/context_projection.rs#L187), [201](../../src/context_projection.rs#L201), [218](../../src/context_projection.rs#L218) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.insert` | [191](../../src/context_projection.rs#L191) | receiver-type-required |
| `project_provider_context_mode` | `id.clone` | [192](../../src/context_projection.rs#L192) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("epoch")                         .ok_or("attempt lacks epoch")?                         .to_owned` | [193](../../src/context_projection.rs#L193) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("epoch")                         .ok_or` | [193](../../src/context_projection.rs#L193) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field` | [193](../../src/context_projection.rs#L193), [234](../../src/context_projection.rs#L234), [242](../../src/context_projection.rs#L242) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits.insert` | [198](../../src/context_projection.rs#L198) | receiver-type-required |
| `project_provider_context_mode` | `settled_outputs.insert` | [204](../../src/context_projection.rs#L204) | receiver-type-required |
| `project_provider_context_mode` | `attempt.to_owned` | [204](../../src/context_projection.rs#L204), [206](../../src/context_projection.rs#L206), [221](../../src/context_projection.rs#L221), [223](../../src/context_projection.rs#L223), [229](../../src/context_projection.rs#L229) | receiver-type-required |
| `project_provider_context_mode` | `output_positions                     .entry(attempt.to_owned())                     .or_insert` | [205](../../src/context_projection.rs#L205), [222](../../src/context_projection.rs#L222) | receiver-type-required |
| `project_provider_context_mode` | `output_positions                     .entry` | [205](../../src/context_projection.rs#L205), [222](../../src/context_projection.rs#L222) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get(attempt).is_some_and` | [208](../../src/context_projection.rs#L208) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get` | [208](../../src/context_projection.rs#L208), [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then(Value::as_str)                         .map(str::to_owned)                         .or` | [209](../../src/context_projection.rs#L209) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then(Value::as_str)                         .map` | [209](../../src/context_projection.rs#L209) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then(&#124;continuation&#124; continuation.get("id"))                         .and_then` | [209](../../src/context_projection.rs#L209) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get("continuation")                         .and_then` | [209](../../src/context_projection.rs#L209) | receiver-type-required |
| `project_provider_context_mode` | `object                         .get` | [209](../../src/context_projection.rs#L209) | receiver-type-required |
| `project_provider_context_mode` | `continuation.get` | [211](../../src/context_projection.rs#L211) | receiver-type-required |
| `project_provider_context_mode` | `object.contains_key` | [217](../../src/context_projection.rs#L217), [359](../../src/context_projection.rs#L359) | receiver-type-required |
| `project_provider_context_mode` | `partial_carriers.insert` | [221](../../src/context_projection.rs#L221) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field("call").ok_or` | [227](../../src/context_projection.rs#L227) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts.insert` | [229](../../src/context_projection.rs#L229) | receiver-type-required |
| `project_provider_context_mode` | `call.to_owned` | [229](../../src/context_projection.rs#L229), [231](../../src/context_projection.rs#L231), [233](../../src/context_projection.rs#L233) | receiver-type-required |
| `project_provider_context_mode` | `call_turn.insert` | [231](../../src/context_projection.rs#L231) | receiver-type-required |
| `project_provider_context_mode` | `call_name.insert` | [232](../../src/context_projection.rs#L232) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("name")                         .ok_or("tool_call lacks name")?                         .to_owned` | [234](../../src/context_projection.rs#L234) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("name")                         .ok_or` | [234](../../src/context_projection.rs#L234) | receiver-type-required |
| `project_provider_context_mode` | `spawned_calls.insert` | [241](../../src/context_projection.rs#L241) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("call")                         .ok_or("spawn lacks call")?                         .to_owned` | [242](../../src/context_projection.rs#L242) | receiver-type-required |
| `project_provider_context_mode` | `event                         .string_field("call")                         .ok_or` | [242](../../src/context_projection.rs#L242) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; settled_outputs.contains(*attempt))         .flat_map(&#124;(_, seqs)&#124; seqs.iter().copied())         .collect::<std::collections::BTreeSet<_>>` | [252](../../src/context_projection.rs#L252) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; settled_outputs.contains(*attempt))         .flat_map` | [252](../../src/context_projection.rs#L252) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter` | [252](../../src/context_projection.rs#L252), [257](../../src/context_projection.rs#L257) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter` | [252](../../src/context_projection.rs#L252), [257](../../src/context_projection.rs#L257) | receiver-type-required |
| `project_provider_context_mode` | `settled_outputs.contains` | [254](../../src/context_projection.rs#L254), [260](../../src/context_projection.rs#L260), [434](../../src/context_projection.rs#L434) | receiver-type-required |
| `project_provider_context_mode` | `seqs.iter().copied` | [255](../../src/context_projection.rs#L255), [263](../../src/context_projection.rs#L263) | receiver-type-required |
| `project_provider_context_mode` | `seqs.iter` | [255](../../src/context_projection.rs#L255), [263](../../src/context_projection.rs#L263) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; {             settled_outputs.contains(*attempt)                 && attempt_epoch.get(*attempt).is_some_and(&#124;id&#124; id == epoch_id)         })         .flat_map(&#124;(_, seqs)&#124; seqs.iter().copied())         .collect::<std::collections::BTreeSet<_>>` | [257](../../src/context_projection.rs#L257) | receiver-type-required |
| `project_provider_context_mode` | `attempt_admits         .iter()         .filter(&#124;(attempt, _)&#124; {             settled_outputs.contains(*attempt)                 && attempt_epoch.get(*attempt).is_some_and(&#124;id&#124; id == epoch_id)         })         .flat_map` | [257](../../src/context_projection.rs#L257) | receiver-type-required |
| `project_provider_context_mode` | `attempt_epoch.get(*attempt).is_some_and` | [261](../../src/context_projection.rs#L261) | receiver-type-required |
| `project_provider_context_mode` | `dialect.server_managed` | [265](../../src/context_projection.rs#L265) | receiver-type-required |
| `project_provider_context_mode` | `continuation_id.is_some` | [266](../../src/context_projection.rs#L266) | receiver-type-required |
| `project_provider_context_mode` | `epoch_seq.ok_or` | [267](../../src/context_projection.rs#L267) | receiver-type-required |
| `project_provider_context_mode` | `Vec::new` | [268](../../src/context_projection.rs#L268), [298](../../src/context_projection.rs#L298) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `items.extend` | [270](../../src/context_projection.rs#L270) | receiver-type-required |
| `project_provider_context_mode` | `seed_provider_items` | [270](../../src/context_projection.rs#L270) | [tekes-worker::context_projection::seed_provider_items](../../src/context_projection.rs#L462) |
| `project_provider_context_mode` | `superseded.contains` | [275](../../src/context_projection.rs#L275), [353](../../src/context_projection.rs#L353) | receiver-type-required |
| `project_provider_context_mode` | `compacted.contains` | [276](../../src/context_projection.rs#L276), [353](../../src/context_projection.rs#L353) | receiver-type-required |
| `project_provider_context_mode` | `materialize_string` | [282](../../src/context_projection.rs#L282) | [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L770) |
| `project_provider_context_mode` | `object.get("summary").ok_or` | [284](../../src/context_projection.rs#L284) | receiver-type-required |
| `project_provider_context_mode` | `items.push` | [286](../../src/context_projection.rs#L286), [449](../../src/context_projection.rs#L449) | receiver-type-required |
| `project_provider_context_mode` | `ijson` | [286](../../src/context_projection.rs#L286), [449](../../src/context_projection.rs#L449) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `projection         .events         .partition_point` | [294](../../src/context_projection.rs#L294) | receiver-type-required |
| `project_provider_context_mode` | `compaction_anchor_sequences` | [297](../../src/context_projection.rs#L297) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `seqs_from_ranges(object.get("supersedes"))?             .into_iter()             .max` | [316](../../src/context_projection.rs#L316) | receiver-type-required |
| `project_provider_context_mode` | `seqs_from_ranges(object.get("supersedes"))?             .into_iter` | [316](../../src/context_projection.rs#L316) | receiver-type-required |
| `project_provider_context_mode` | `replaced_result.insert` | [320](../../src/context_projection.rs#L320) | receiver-type-required |
| `project_provider_context_mode` | `replaced_result.get` | [325](../../src/context_projection.rs#L325) | receiver-type-required |
| `project_provider_context_mode` | `render_anchor` | [334](../../src/context_projection.rs#L334), [340](../../src/context_projection.rs#L340) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `output_positions.get` | [335](../../src/context_projection.rs#L335) | receiver-type-required |
| `project_provider_context_mode` | `call_attempts.get` | [335](../../src/context_projection.rs#L335) | receiver-type-required |
| `project_provider_context_mode` | `(*output > anchor).then_some` | [336](../../src/context_projection.rs#L336) | receiver-type-required |
| `project_provider_context_mode` | `projection.events.iter().collect::<Vec<_>>` | [338](../../src/context_projection.rs#L338) | receiver-type-required |
| `project_provider_context_mode` | `projection.events.iter` | [338](../../src/context_projection.rs#L338) | receiver-type-required |
| `project_provider_context_mode` | `ordered_events.sort_by_key` | [339](../../src/context_projection.rs#L339) | receiver-type-required |
| `project_provider_context_mode` | `input_position.get` | [341](../../src/context_projection.rs#L341) | receiver-type-required |
| `project_provider_context_mode` | `deferred_result_position` | [343](../../src/context_projection.rs#L343) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `semantic_anchors.contains` | [352](../../src/context_projection.rs#L352) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field("attempt")                 .is_some_and` | [360](../../src/context_projection.rs#L360) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field` | [360](../../src/context_projection.rs#L360), [372](../../src/context_projection.rs#L372) | receiver-type-required |
| `project_provider_context_mode` | `partial_carriers.contains` | [362](../../src/context_projection.rs#L362), [434](../../src/context_projection.rs#L434) | receiver-type-required |
| `project_provider_context_mode` | `event                 .string_field("call")                 .is_some_and` | [372](../../src/context_projection.rs#L372) | receiver-type-required |
| `project_provider_context_mode` | `spawned_calls.contains` | [374](../../src/context_projection.rs#L374) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.get(&event.seq()).is_some_and` | [397](../../src/context_projection.rs#L397) | receiver-type-required |
| `project_provider_context_mode` | `input_turn.get` | [397](../../src/context_projection.rs#L397) | receiver-type-required |
| `project_provider_context_mode` | `admitted_all.contains` | [399](../../src/context_projection.rs#L399) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then(Value::as_str)                 .and_then(&#124;call&#124; call_turn.get(call))                 .is_some_and` | [404](../../src/context_projection.rs#L404), [416](../../src/context_projection.rs#L416) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then(Value::as_str)                 .and_then` | [404](../../src/context_projection.rs#L404), [416](../../src/context_projection.rs#L416) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get("call")                 .and_then` | [404](../../src/context_projection.rs#L404), [416](../../src/context_projection.rs#L416) | receiver-type-required |
| `project_provider_context_mode` | `object                 .get` | [404](../../src/context_projection.rs#L404), [416](../../src/context_projection.rs#L416) | receiver-type-required |
| `project_provider_context_mode` | `call_turn.get` | [407](../../src/context_projection.rs#L407), [419](../../src/context_projection.rs#L419) | receiver-type-required |
| `project_provider_context_mode` | `event.turn().is_none_or` | [409](../../src/context_projection.rs#L409) | receiver-type-required |
| `project_provider_context_mode` | `input_turn                 .get(&event.seq())                 .is_some_and` | [413](../../src/context_projection.rs#L413) | receiver-type-required |
| `project_provider_context_mode` | `input_turn                 .get` | [413](../../src/context_projection.rs#L413) | receiver-type-required |
| `project_provider_context_mode` | `epoch_pending.contains` | [424](../../src/context_projection.rs#L424) | receiver-type-required |
| `project_provider_context_mode` | `admitted.contains` | [427](../../src/context_projection.rs#L427), [446](../../src/context_projection.rs#L446) | receiver-type-required |
| `project_provider_context_mode` | `event.string_field("attempt").is_some_and` | [433](../../src/context_projection.rs#L433) | receiver-type-required |
| `project_provider_context_mode` | `render_event_item(ledger, event, object, &call_name, dialect)?.ok_or_else` | [440](../../src/context_projection.rs#L440) | receiver-type-required |
| `project_provider_context_mode` | `render_event_item` | [440](../../src/context_projection.rs#L440) | [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L606) |
| `project_provider_context_mode` | `admits.push` | [447](../../src/context_projection.rs#L447) | receiver-type-required |
| `project_provider_context_mode` | `admits.sort_unstable` | [451](../../src/context_projection.rs#L451) | receiver-type-required |
| `project_provider_context_mode` | `admits.dedup` | [452](../../src/context_projection.rs#L452) | receiver-type-required |
| `project_provider_context_mode` | `Ok` | [453](../../src/context_projection.rs#L453) | external-constructor-callback-or-unresolved |
| `project_provider_context_mode` | `incremental_continuation             .then_some(continuation_id)             .flatten` | [456](../../src/context_projection.rs#L456) | receiver-type-required |
| `project_provider_context_mode` | `incremental_continuation             .then_some` | [456](../../src/context_projection.rs#L456) | receiver-type-required |
| `seed_provider_items` | `ledger.projection().ok_or` | [465](../../src/context_projection.rs#L465) | receiver-type-required |
| `seed_provider_items` | `ledger.projection` | [465](../../src/context_projection.rs#L465) | receiver-type-required |
| `seed_provider_items` | `projection.events.first` | [466](../../src/context_projection.rs#L466) | receiver-type-required |
| `seed_provider_items` | `Ok` | [467](../../src/context_projection.rs#L467), [471](../../src/context_projection.rs#L471), [562](../../src/context_projection.rs#L562) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `Vec::new` | [467](../../src/context_projection.rs#L467), [471](../../src/context_projection.rs#L471), [475](../../src/context_projection.rs#L475) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `serde_json::to_value` | [469](../../src/context_projection.rs#L469), [488](../../src/context_projection.rs#L488) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `genesis.raw` | [469](../../src/context_projection.rs#L469) | receiver-type-required |
| `seed_provider_items` | `raw.pointer("/seed/snapshot/asset").and_then` | [470](../../src/context_projection.rs#L470) | receiver-type-required |
| `seed_provider_items` | `raw.pointer` | [470](../../src/context_projection.rs#L470) | receiver-type-required |
| `seed_provider_items` | `ledger.path().parent().ok_or` | [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `seed_provider_items` | `ledger.path().parent` | [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `seed_provider_items` | `ledger.path` | [473](../../src/context_projection.rs#L473) | receiver-type-required |
| `seed_provider_items` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [474](../../src/context_projection.rs#L474) | receiver-type-required |
| `seed_provider_items` | `store::AssetStore::new` | [474](../../src/context_projection.rs#L474) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `seed_provider_items` | `folder.join` | [474](../../src/context_projection.rs#L474) | receiver-type-required |
| `seed_provider_items` | `bytes.split_inclusive` | [476](../../src/context_projection.rs#L476) | receiver-type-required |
| `seed_provider_items` | `line.strip_suffix(b"\n").ok_or` | [477](../../src/context_projection.rs#L477) | receiver-type-required |
| `seed_provider_items` | `line.strip_suffix` | [477](../../src/context_projection.rs#L477) | receiver-type-required |
| `seed_provider_items` | `canonical.is_empty` | [478](../../src/context_projection.rs#L478) | receiver-type-required |
| `seed_provider_items` | `Err` | [479](../../src/context_projection.rs#L479), [553](../../src/context_projection.rs#L553) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `"seed snapshot has an empty line".into` | [479](../../src/context_projection.rs#L479) | receiver-type-required |
| `seed_provider_items` | `Event::decode_canonical` | [481](../../src/context_projection.rs#L481) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `event.raw` | [488](../../src/context_projection.rs#L488) | receiver-type-required |
| `seed_provider_items` | `raw.as_object().ok_or` | [489](../../src/context_projection.rs#L489) | receiver-type-required |
| `seed_provider_items` | `raw.as_object` | [489](../../src/context_projection.rs#L489) | receiver-type-required |
| `seed_provider_items` | `event.kind` | [490](../../src/context_projection.rs#L490) | receiver-type-required |
| `seed_provider_items` | `event                     .string_field("call")                     .ok_or` | [504](../../src/context_projection.rs#L504) | receiver-type-required |
| `seed_provider_items` | `event                     .string_field` | [504](../../src/context_projection.rs#L504) | receiver-type-required |
| `seed_provider_items` | `object                     .get("outcome")                     .ok_or` | [507](../../src/context_projection.rs#L507) | receiver-type-required |
| `seed_provider_items` | `object                     .get` | [507](../../src/context_projection.rs#L507), [529](../../src/context_projection.rs#L529) | receiver-type-required |
| `seed_provider_items` | `event.string_field` | [528](../../src/context_projection.rs#L528) | receiver-type-required |
| `seed_provider_items` | `Some` | [528](../../src/context_projection.rs#L528) | external-constructor-callback-or-unresolved |
| `seed_provider_items` | `object                     .get("payload")                     .and_then(Value::as_object)                     .ok_or` | [529](../../src/context_projection.rs#L529) | receiver-type-required |
| `seed_provider_items` | `object                     .get("payload")                     .and_then` | [529](../../src/context_projection.rs#L529) | receiver-type-required |
| `seed_provider_items` | `format!(                     "model-visible seed kind {} has no provider projection",                     other.as_str()                 )                 .into` | [553](../../src/context_projection.rs#L553) | receiver-type-required |
| `seed_provider_items` | `items.push` | [560](../../src/context_projection.rs#L560) | receiver-type-required |
| `seed_provider_items` | `ijson` | [560](../../src/context_projection.rs#L560) | external-constructor-callback-or-unresolved |
| `provider_wire_call_id` | `ledger         .projection()         .and_then(&#124;projection&#124; {             projection.events.iter().find(&#124;event&#124; {                 *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)             })         })         .and_then(&#124;event&#124; event.string_field("provider_call"))         .unwrap_or` | [566](../../src/context_projection.rs#L566) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection()         .and_then(&#124;projection&#124; {             projection.events.iter().find(&#124;event&#124; {                 *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)             })         })         .and_then` | [566](../../src/context_projection.rs#L566) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection()         .and_then` | [566](../../src/context_projection.rs#L566) | receiver-type-required |
| `provider_wire_call_id` | `ledger         .projection` | [566](../../src/context_projection.rs#L566) | receiver-type-required |
| `provider_wire_call_id` | `projection.events.iter().find` | [569](../../src/context_projection.rs#L569) | receiver-type-required |
| `provider_wire_call_id` | `projection.events.iter` | [569](../../src/context_projection.rs#L569) | receiver-type-required |
| `provider_wire_call_id` | `event.kind` | [570](../../src/context_projection.rs#L570) | receiver-type-required |
| `provider_wire_call_id` | `event.string_field` | [570](../../src/context_projection.rs#L570), [573](../../src/context_projection.rs#L573) | receiver-type-required |
| `provider_wire_call_id` | `Some` | [570](../../src/context_projection.rs#L570) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `object         .get("sealed")         .and_then(Value::as_object)         .ok_or_else` | [585](../../src/context_projection.rs#L585) | receiver-type-required |
| `render_sealed_carrier` | `object         .get("sealed")         .and_then` | [585](../../src/context_projection.rs#L585) | receiver-type-required |
| `render_sealed_carrier` | `object         .get` | [585](../../src/context_projection.rs#L585) | receiver-type-required |
| `render_sealed_carrier` | `sealed.get("version").and_then` | [589](../../src/context_projection.rs#L589) | receiver-type-required |
| `render_sealed_carrier` | `sealed.get` | [589](../../src/context_projection.rs#L589), [590](../../src/context_projection.rs#L590) | receiver-type-required |
| `render_sealed_carrier` | `Some` | [589](../../src/context_projection.rs#L589), [590](../../src/context_projection.rs#L590) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `sealed.get("adapter").and_then` | [590](../../src/context_projection.rs#L590) | receiver-type-required |
| `render_sealed_carrier` | `dialect.as_str` | [590](../../src/context_projection.rs#L590) | receiver-type-required |
| `render_sealed_carrier` | `Err` | [592](../../src/context_projection.rs#L592) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `format!("{kind} sealed carrier is incompatible with the active adapter").into` | [593](../../src/context_projection.rs#L593) | receiver-type-required |
| `render_sealed_carrier` | `materialize_string` | [596](../../src/context_projection.rs#L596) | [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L770) |
| `render_sealed_carrier` | `sealed             .get("fragments")             .ok_or` | [598](../../src/context_projection.rs#L598) | receiver-type-required |
| `render_sealed_carrier` | `sealed             .get` | [598](../../src/context_projection.rs#L598) | receiver-type-required |
| `render_sealed_carrier` | `serde_json::to_value` | [602](../../src/context_projection.rs#L602) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `IJsonValue::parse` | [602](../../src/context_projection.rs#L602) | external-constructor-callback-or-unresolved |
| `render_sealed_carrier` | `fragments.as_bytes` | [602](../../src/context_projection.rs#L602) | receiver-type-required |
| `render_sealed_carrier` | `Ok` | [603](../../src/context_projection.rs#L603) | external-constructor-callback-or-unresolved |
| `render_event_item` | `event.kind` | [613](../../src/context_projection.rs#L613) | receiver-type-required |
| `render_event_item` | `render_sealed_carrier` | [618](../../src/context_projection.rs#L618), [621](../../src/context_projection.rs#L621) | [tekes-worker::context_projection::render_sealed_carrier](../../src/context_projection.rs#L579) |
| `render_event_item` | `object.contains_key` | [620](../../src/context_projection.rs#L620) | receiver-type-required |
| `render_event_item` | `event.string_field("call").ok_or` | [637](../../src/context_projection.rs#L637) | receiver-type-required |
| `render_event_item` | `event.string_field` | [637](../../src/context_projection.rs#L637) | receiver-type-required |
| `render_event_item` | `object.get("outcome").ok_or` | [638](../../src/context_projection.rs#L638) | receiver-type-required |
| `render_event_item` | `object.get` | [638](../../src/context_projection.rs#L638), [640](../../src/context_projection.rs#L640) | receiver-type-required |
| `render_event_item` | `call_names.get(call).map(String::as_str).unwrap_or` | [639](../../src/context_projection.rs#L639) | receiver-type-required |
| `render_event_item` | `call_names.get(call).map` | [639](../../src/context_projection.rs#L639) | receiver-type-required |
| `render_event_item` | `call_names.get` | [639](../../src/context_projection.rs#L639) | receiver-type-required |
| `render_event_item` | `render_blocks` | [641](../../src/context_projection.rs#L641) | [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L691) |
| `render_event_item` | `blocks.as_mut_slice` | [644](../../src/context_projection.rs#L644) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then(Value::as_str)                         .filter(&#124;_&#124; block.get("type").and_then(Value::as_str) == Some("text"))                         .and_then` | [645](../../src/context_projection.rs#L645) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then(Value::as_str)                         .filter` | [645](../../src/context_projection.rs#L645) | receiver-type-required |
| `render_event_item` | `block                         .get("text")                         .and_then` | [645](../../src/context_projection.rs#L645) | receiver-type-required |
| `render_event_item` | `block                         .get` | [645](../../src/context_projection.rs#L645) | receiver-type-required |
| `render_event_item` | `block.get("type").and_then` | [648](../../src/context_projection.rs#L648) | receiver-type-required |
| `render_event_item` | `block.get` | [648](../../src/context_projection.rs#L648) | receiver-type-required |
| `render_event_item` | `Some` | [648](../../src/context_projection.rs#L648), [688](../../src/context_projection.rs#L688) | external-constructor-callback-or-unresolved |
| `render_event_item` | `result_presentation::present` | [649](../../src/context_projection.rs#L649) | external-constructor-callback-or-unresolved |
| `render_event_item` | `Value::Array` | [654](../../src/context_projection.rs#L654) | external-constructor-callback-or-unresolved |
| `render_event_item` | `event                 .string_field("call")                 .ok_or` | [669](../../src/context_projection.rs#L669) | receiver-type-required |
| `render_event_item` | `event                 .string_field` | [669](../../src/context_projection.rs#L669) | receiver-type-required |
| `render_event_item` | `Ok` | [686](../../src/context_projection.rs#L686), [688](../../src/context_projection.rs#L688) | external-constructor-callback-or-unresolved |
| `render_blocks` | `materialize_json` | [695](../../src/context_projection.rs#L695) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L742) |
| `render_blocks` | `value.as_array().ok_or` | [696](../../src/context_projection.rs#L696) | receiver-type-required |
| `render_blocks` | `value.as_array` | [696](../../src/context_projection.rs#L696) | receiver-type-required |
| `render_blocks` | `blocks         .iter()         .map(&#124;block&#124; {             let object = block.as_object().ok_or("block is not an object")?;             Ok(match object.get("type").and_then(Value::as_str) {                 Some("text") => json!({"type":"text","text":object.get("text").and_then(Value::as_str).ok_or("text block lacks text")?}),                 Some("reasoning") => json!({"type":"reasoning","text":object.get("text").and_then(Value::as_str).ok_or("reasoning block lacks text")?}),                 Some("image") => {                     let asset = object.get("asset").and_then(Value::as_str).ok_or("image block lacks asset")?;                     let folder = ledger.path().parent().ok_or("ledger has no folder")?;                     let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;                     json!({                         "type":"file","name":asset,                         "data":base64::engine::general_purpose::STANDARD.encode(bytes),                         "mime":object.get("mime").and_then(Value::as_str).ok_or("image block lacks mime")?                     })                 },                 Some("file") => {                     let asset = object.get("asset").and_then(Value::as_str).ok_or("file block lacks asset")?;                     let folder = ledger.path().parent().ok_or("ledger has no folder")?;                     let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;                     json!({                         "type":"file",                         "name":object.get("name").and_then(Value::as_str).ok_or("file block lacks name")?,                         "mime":object.get("mime").and_then(Value::as_str).ok_or("file block lacks mime")?,                         "bytes":bytes.len(),                         "data":base64::engine::general_purpose::STANDARD.encode(bytes)                     })                 },                 Some("tool-call") => json!({                     "type":"tool_call","call_id":object.get("call").and_then(Value::as_str).ok_or("nested call lacks id")?,                     "name":object.get("name").and_then(Value::as_str).ok_or("nested call lacks name")?,                     "arguments":object.get("args").cloned().ok_or("nested call lacks args")?                 }),                 Some("tool-result") => json!({                     "type":"tool_result","call_id":object.get("call").and_then(Value::as_str).ok_or("nested result lacks call")?,                     "name":"tool","result":object.get("content").cloned().ok_or("nested result lacks content")?,                     "error":object.get("error").and_then(Value::as_bool).unwrap_or(false)                 }),                 _ => return Err("unknown block kind".into()),             })         })         .collect` | [697](../../src/context_projection.rs#L697) | receiver-type-required |
| `render_blocks` | `blocks         .iter()         .map` | [697](../../src/context_projection.rs#L697) | receiver-type-required |
| `render_blocks` | `blocks         .iter` | [697](../../src/context_projection.rs#L697) | receiver-type-required |
| `render_blocks` | `block.as_object().ok_or` | [700](../../src/context_projection.rs#L700) | receiver-type-required |
| `render_blocks` | `block.as_object` | [700](../../src/context_projection.rs#L700) | receiver-type-required |
| `render_blocks` | `Ok` | [701](../../src/context_projection.rs#L701) | external-constructor-callback-or-unresolved |
| `render_blocks` | `object.get("type").and_then` | [701](../../src/context_projection.rs#L701) | receiver-type-required |
| `render_blocks` | `object.get` | [701](../../src/context_projection.rs#L701), [705](../../src/context_projection.rs#L705), [715](../../src/context_projection.rs#L715) | receiver-type-required |
| `render_blocks` | `object.get("asset").and_then(Value::as_str).ok_or` | [705](../../src/context_projection.rs#L705), [715](../../src/context_projection.rs#L715) | receiver-type-required |
| `render_blocks` | `object.get("asset").and_then` | [705](../../src/context_projection.rs#L705), [715](../../src/context_projection.rs#L715) | receiver-type-required |
| `render_blocks` | `ledger.path().parent().ok_or` | [706](../../src/context_projection.rs#L706), [716](../../src/context_projection.rs#L716) | receiver-type-required |
| `render_blocks` | `ledger.path().parent` | [706](../../src/context_projection.rs#L706), [716](../../src/context_projection.rs#L716) | receiver-type-required |
| `render_blocks` | `ledger.path` | [706](../../src/context_projection.rs#L706), [716](../../src/context_projection.rs#L716) | receiver-type-required |
| `render_blocks` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [707](../../src/context_projection.rs#L707), [717](../../src/context_projection.rs#L717) | receiver-type-required |
| `render_blocks` | `store::AssetStore::new` | [707](../../src/context_projection.rs#L707), [717](../../src/context_projection.rs#L717) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `render_blocks` | `folder.join` | [707](../../src/context_projection.rs#L707), [717](../../src/context_projection.rs#L717) | receiver-type-required |
| `render_blocks` | `Err` | [736](../../src/context_projection.rs#L736) | external-constructor-callback-or-unresolved |
| `render_blocks` | `"unknown block kind".into` | [736](../../src/context_projection.rs#L736) | receiver-type-required |
| `materialize_json` | `materialize_json_at` | [746](../../src/context_projection.rs#L746) | [tekes-worker::context_projection::materialize_json_at](../../src/context_projection.rs#L749) |
| `materialize_json` | `ledger.path` | [746](../../src/context_projection.rs#L746) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then(&#124;object&#124; object.get("$spill"))         .and_then` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter(&#124;object&#124; object.len() == 1)         .and_then` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `materialize_json_at` | `value         .as_object()         .filter` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `materialize_json_at` | `value         .as_object` | [753](../../src/context_projection.rs#L753) | receiver-type-required |
| `materialize_json_at` | `object.len` | [755](../../src/context_projection.rs#L755) | receiver-type-required |
| `materialize_json_at` | `object.get` | [756](../../src/context_projection.rs#L756) | receiver-type-required |
| `materialize_json_at` | `Ok` | [759](../../src/context_projection.rs#L759), [767](../../src/context_projection.rs#L767) | external-constructor-callback-or-unresolved |
| `materialize_json_at` | `value.clone` | [759](../../src/context_projection.rs#L759) | receiver-type-required |
| `materialize_json_at` | `spill         .get("asset")         .and_then(Value::as_str)         .ok_or` | [761](../../src/context_projection.rs#L761) | receiver-type-required |
| `materialize_json_at` | `spill         .get("asset")         .and_then` | [761](../../src/context_projection.rs#L761) | receiver-type-required |
| `materialize_json_at` | `spill         .get` | [761](../../src/context_projection.rs#L761) | receiver-type-required |
| `materialize_json_at` | `ledger_path.parent().ok_or` | [765](../../src/context_projection.rs#L765) | receiver-type-required |
| `materialize_json_at` | `ledger_path.parent` | [765](../../src/context_projection.rs#L765) | receiver-type-required |
| `materialize_json_at` | `store::AssetStore::new(folder.join("assets"))?.read_verified` | [766](../../src/context_projection.rs#L766) | receiver-type-required |
| `materialize_json_at` | `store::AssetStore::new` | [766](../../src/context_projection.rs#L766) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `materialize_json_at` | `folder.join` | [766](../../src/context_projection.rs#L766) | receiver-type-required |
| `materialize_json_at` | `serde_json::to_value` | [767](../../src/context_projection.rs#L767) | external-constructor-callback-or-unresolved |
| `materialize_json_at` | `IJsonValue::parse` | [767](../../src/context_projection.rs#L767) | external-constructor-callback-or-unresolved |
| `materialize_string` | `materialize_json` | [774](../../src/context_projection.rs#L774) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L742) |
| `materialize_string` | `value         .as_str()         .map(str::to_owned)         .ok_or_else` | [775](../../src/context_projection.rs#L775) | receiver-type-required |
| `materialize_string` | `value         .as_str()         .map` | [775](../../src/context_projection.rs#L775) | receiver-type-required |
| `materialize_string` | `value         .as_str` | [775](../../src/context_projection.rs#L775) | receiver-type-required |
| `materialize_string` | `"spilled value is not a string".into` | [778](../../src/context_projection.rs#L778) | receiver-type-required |
| `seqs_from_ranges` | `Vec::new` | [782](../../src/context_projection.rs#L782) | external-constructor-callback-or-unresolved |
| `seqs_from_ranges` | `value.and_then(Value::as_array).into_iter().flatten` | [783](../../src/context_projection.rs#L783) | receiver-type-required |
| `seqs_from_ranges` | `value.and_then(Value::as_array).into_iter` | [783](../../src/context_projection.rs#L783) | receiver-type-required |
| `seqs_from_ranges` | `value.and_then` | [783](../../src/context_projection.rs#L783) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("from")             .and_then(Value::as_u64)             .ok_or` | [784](../../src/context_projection.rs#L784) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("from")             .and_then` | [784](../../src/context_projection.rs#L784) | receiver-type-required |
| `seqs_from_ranges` | `range             .get` | [784](../../src/context_projection.rs#L784), [788](../../src/context_projection.rs#L788) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("to")             .and_then(Value::as_u64)             .ok_or` | [788](../../src/context_projection.rs#L788) | receiver-type-required |
| `seqs_from_ranges` | `range             .get("to")             .and_then` | [788](../../src/context_projection.rs#L788) | receiver-type-required |
| `seqs_from_ranges` | `seqs.extend` | [792](../../src/context_projection.rs#L792) | receiver-type-required |
| `seqs_from_ranges` | `Ok` | [794](../../src/context_projection.rs#L794) | external-constructor-callback-or-unresolved |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map(&#124;source&#124; source.content.as_str())         .collect::<Vec<_>>()         .join` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map(&#124;source&#124; source.content.as_str())         .collect::<Vec<_>>` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter(&#124;source&#124; source.kind == InstructionKind::Agents)         .map` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map(&#124;index&#124; instruction.sources.get(*index as usize))         .filter` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter()         .filter_map` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction         .effective         .agents         .iter` | [798](../../src/context_projection.rs#L798) | receiver-type-required |
| `effective_system` | `instruction.sources.get` | [802](../../src/context_projection.rs#L802) | receiver-type-required |
| `effective_system` | `source.content.as_str` | [804](../../src/context_projection.rs#L804) | receiver-type-required |
| `epoch_open_reason` | `previous.integer_field` | [840](../../src/context_projection.rs#L840) | receiver-type-required |
| `epoch_open_reason` | `Some` | [840](../../src/context_projection.rs#L840), [851](../../src/context_projection.rs#L851), [854](../../src/context_projection.rs#L854) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `serde_json::to_value(previous.raw()).ok` | [843](../../src/context_projection.rs#L843) | receiver-type-required |
| `epoch_open_reason` | `serde_json::to_value` | [843](../../src/context_projection.rs#L843) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `previous.raw` | [843](../../src/context_projection.rs#L843) | receiver-type-required |
| `epoch_open_reason` | `head.as_ref().and_then` | [845](../../src/context_projection.rs#L845) | receiver-type-required |
| `epoch_open_reason` | `head.as_ref` | [845](../../src/context_projection.rs#L845) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer(pointer)                 .and_then(Value::as_str)                 .map` | [846](../../src/context_projection.rs#L846) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer(pointer)                 .and_then` | [846](../../src/context_projection.rs#L846) | receiver-type-required |
| `epoch_open_reason` | `raw.pointer` | [846](../../src/context_projection.rs#L846) | receiver-type-required |
| `epoch_open_reason` | `field("/tools/digest").as_deref` | [851](../../src/context_projection.rs#L851) | receiver-type-required |
| `epoch_open_reason` | `field` | [851](../../src/context_projection.rs#L851), [854](../../src/context_projection.rs#L854) | external-constructor-callback-or-unresolved |
| `epoch_open_reason` | `field("/system/digest").as_deref` | [854](../../src/context_projection.rs#L854) | receiver-type-required |
| `append_tool_result_trim` | `ledger.projection().ok_or` | [870](../../src/context_projection.rs#L870) | receiver-type-required |
| `append_tool_result_trim` | `ledger.projection` | [870](../../src/context_projection.rs#L870) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter()         .find(&#124;event&#124; event.seq() == trim.seq)         .ok_or` | [871](../../src/context_projection.rs#L871) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter()         .find` | [871](../../src/context_projection.rs#L871) | receiver-type-required |
| `append_tool_result_trim` | `projection         .events         .iter` | [871](../../src/context_projection.rs#L871) | receiver-type-required |
| `append_tool_result_trim` | `event.seq` | [874](../../src/context_projection.rs#L874) | receiver-type-required |
| `append_tool_result_trim` | `serde_json::to_value` | [876](../../src/context_projection.rs#L876) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `original.raw` | [876](../../src/context_projection.rs#L876) | receiver-type-required |
| `append_tool_result_trim` | `raw.as_object().ok_or` | [877](../../src/context_projection.rs#L877) | receiver-type-required |
| `append_tool_result_trim` | `raw.as_object` | [877](../../src/context_projection.rs#L877) | receiver-type-required |
| `append_tool_result_trim` | `materialize_json` | [882](../../src/context_projection.rs#L882) | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L742) |
| `append_tool_result_trim` | `object.get("content").ok_or` | [884](../../src/context_projection.rs#L884) | receiver-type-required |
| `append_tool_result_trim` | `object.get` | [884](../../src/context_projection.rs#L884), [894](../../src/context_projection.rs#L894) | receiver-type-required |
| `append_tool_result_trim` | `engine::trimmed_tool_result_content` | [886](../../src/context_projection.rs#L886) | [engine::context::trimmed_tool_result_content](../../../engine/src/context.rs#L502) |
| `append_tool_result_trim` | `replacement             .as_object_mut()             .expect("replacement object")             .insert` | [895](../../src/context_projection.rs#L895) | receiver-type-required |
| `append_tool_result_trim` | `replacement             .as_object_mut()             .expect` | [895](../../src/context_projection.rs#L895) | receiver-type-required |
| `append_tool_result_trim` | `replacement             .as_object_mut` | [895](../../src/context_projection.rs#L895) | receiver-type-required |
| `append_tool_result_trim` | `"meta".to_owned` | [898](../../src/context_projection.rs#L898) | receiver-type-required |
| `append_tool_result_trim` | `meta.clone` | [898](../../src/context_projection.rs#L898) | receiver-type-required |
| `append_tool_result_trim` | `ledger.append_contract` | [900](../../src/context_projection.rs#L900) | receiver-type-required |
| `append_tool_result_trim` | `make_event` | [900](../../src/context_projection.rs#L900) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `BarrierContext::default` | [900](../../src/context_projection.rs#L900) | external-constructor-callback-or-unresolved |
| `append_tool_result_trim` | `Ok` | [901](../../src/context_projection.rs#L901) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `ledger.path().parent().ok_or` | [918](../../src/context_projection.rs#L918) | receiver-type-required |
| `append_provider_epoch` | `ledger.path().parent` | [918](../../src/context_projection.rs#L918) | receiver-type-required |
| `append_provider_epoch` | `ledger.path` | [918](../../src/context_projection.rs#L918) | receiver-type-required |
| `append_provider_epoch` | `store::AssetStore::new` | [919](../../src/context_projection.rs#L919) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `append_provider_epoch` | `folder.join` | [919](../../src/context_projection.rs#L919) | receiver-type-required |
| `append_provider_epoch` | `profile.canonical_bytes` | [920](../../src/context_projection.rs#L920) | receiver-type-required |
| `append_provider_epoch` | `assets.publish` | [921](../../src/context_projection.rs#L921), [925](../../src/context_projection.rs#L925) | receiver-type-required |
| `append_provider_epoch` | `tools.canonical_bytes` | [924](../../src/context_projection.rs#L924) | receiver-type-required |
| `append_provider_epoch` | `make_event` | [926](../../src/context_projection.rs#L926) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `ledger.append_contract` | [946](../../src/context_projection.rs#L946) | receiver-type-required |
| `append_provider_epoch` | `BarrierContext::default` | [946](../../src/context_projection.rs#L946) | external-constructor-callback-or-unresolved |
| `append_provider_epoch` | `Ok` | [947](../../src/context_projection.rs#L947) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .rev()         .find` | [961](../../src/context_projection.rs#L961) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .rev` | [961](../../src/context_projection.rs#L961) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter` | [961](../../src/context_projection.rs#L961), [967](../../src/context_projection.rs#L967) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection` | [961](../../src/context_projection.rs#L961), [967](../../src/context_projection.rs#L967) | receiver-type-required |
| `latest_compatible_epoch` | `event.kind` | [966](../../src/context_projection.rs#L966) | receiver-type-required |
| `latest_compatible_epoch` | `ledger         .projection()?         .events         .iter()         .any` | [967](../../src/context_projection.rs#L967) | receiver-type-required |
| `latest_compatible_epoch` | `later.seq` | [971](../../src/context_projection.rs#L971) | receiver-type-required |
| `latest_compatible_epoch` | `event.seq` | [971](../../src/context_projection.rs#L971) | receiver-type-required |
| `latest_compatible_epoch` | `later.kind` | [971](../../src/context_projection.rs#L971) | receiver-type-required |
| `latest_compatible_epoch` | `serde_json::to_value(event.raw()).ok` | [975](../../src/context_projection.rs#L975) | receiver-type-required |
| `latest_compatible_epoch` | `serde_json::to_value` | [975](../../src/context_projection.rs#L975) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `event.raw` | [975](../../src/context_projection.rs#L975) | receiver-type-required |
| `latest_compatible_epoch` | `(event.string_field("adapter") == Some(dialect.as_str())         && event.string_field("model") == Some(model)         && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)         && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)         && event.integer_field("renderer") == Some(renderer))     .then(&#124;&#124; event.string_field("id").map(str::to_owned))     .flatten` | [976](../../src/context_projection.rs#L976) | receiver-type-required |
| `latest_compatible_epoch` | `(event.string_field("adapter") == Some(dialect.as_str())         && event.string_field("model") == Some(model)         && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)         && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)         && event.integer_field("renderer") == Some(renderer))     .then` | [976](../../src/context_projection.rs#L976) | receiver-type-required |
| `latest_compatible_epoch` | `event.string_field` | [976](../../src/context_projection.rs#L976), [977](../../src/context_projection.rs#L977), [981](../../src/context_projection.rs#L981) | receiver-type-required |
| `latest_compatible_epoch` | `Some` | [976](../../src/context_projection.rs#L976), [977](../../src/context_projection.rs#L977), [978](../../src/context_projection.rs#L978), [979](../../src/context_projection.rs#L979), [980](../../src/context_projection.rs#L980) | external-constructor-callback-or-unresolved |
| `latest_compatible_epoch` | `dialect.as_str` | [976](../../src/context_projection.rs#L976) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer("/system/digest").and_then` | [978](../../src/context_projection.rs#L978) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer` | [978](../../src/context_projection.rs#L978), [979](../../src/context_projection.rs#L979) | receiver-type-required |
| `latest_compatible_epoch` | `raw.pointer("/tools/digest").and_then` | [979](../../src/context_projection.rs#L979) | receiver-type-required |
| `latest_compatible_epoch` | `event.integer_field` | [980](../../src/context_projection.rs#L980) | receiver-type-required |
| `latest_compatible_epoch` | `event.string_field("id").map` | [981](../../src/context_projection.rs#L981) | receiver-type-required |
| `ranges` | `Vec::new` | [986](../../src/context_projection.rs#L986) | external-constructor-callback-or-unresolved |
| `ranges` | `ranges.last_mut().and_then` | [988](../../src/context_projection.rs#L988) | receiver-type-required |
| `ranges` | `ranges.last_mut` | [988](../../src/context_projection.rs#L988) | receiver-type-required |
| `ranges` | `range.get("to").and_then` | [990](../../src/context_projection.rs#L990) | receiver-type-required |
| `ranges` | `range.get` | [990](../../src/context_projection.rs#L990) | receiver-type-required |
| `ranges` | `Some` | [990](../../src/context_projection.rs#L990) | external-constructor-callback-or-unresolved |
| `ranges` | `seq.saturating_sub` | [990](../../src/context_projection.rs#L990) | receiver-type-required |
| `ranges` | `range.insert` | [992](../../src/context_projection.rs#L992) | receiver-type-required |
| `ranges` | `"to".to_owned` | [992](../../src/context_projection.rs#L992) | receiver-type-required |
| `ranges` | `Value::from` | [992](../../src/context_projection.rs#L992) | external-constructor-callback-or-unresolved |
| `ranges` | `ranges.push` | [994](../../src/context_projection.rs#L994) | receiver-type-required |
