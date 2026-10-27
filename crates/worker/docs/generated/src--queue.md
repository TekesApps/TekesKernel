# tekes-worker::queue

[Package atlas](index.md) · [Source](../../src/queue.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::queue::QueueTarget](../../src/queue.rs#L4) | struct_item | `private` |  |
| [tekes-worker::queue::execute_queue_transaction](../../src/queue.rs#L11) | function_item | `pub(crate)` |  |
| [tekes-worker::queue::queue_rejected](../../src/queue.rs#L173) | function_item | `private` |  |
| [tekes-worker::queue::queue_target](../../src/queue.rs#L184) | function_item | `private` |  |
| [tekes-worker::queue::queue_steer_window_open](../../src/queue.rs#L279) | function_item | `private` |  |
| [tekes-worker::queue::verify_queue_assets](../../src/queue.rs#L288) | function_item | `private` |  |
| [tekes-worker::queue::validate_existing_retraction](../../src/queue.rs#L304) | function_item | `private` |  |
| [tekes-worker::queue::validate_existing_replacement](../../src/queue.rs#L323) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–7: 6 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-worker::queue::execute_queue_transaction"]
  n1["tekes-worker::queue::queue_rejected"]
  n2["tekes-worker::queue::queue_target"]
  n3["tekes-worker::queue::queue_steer_window_open"]
  n4["tekes-worker::queue::verify_queue_assets"]
  n5["tekes-worker::queue::validate_existing_retraction"]
  n6["tekes-worker::queue::validate_existing_replacement"]
  n0 --> n1
  n0 --> n2
  n0 --> n3
  n0 --> n4
  n0 --> n5
  n0 --> n6
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `execute_queue_transaction` | `transaction.validate` | [16](../../src/queue.rs#L16) | receiver-type-required |
| `execute_queue_transaction` | `ledger         .projection()         .ok_or` | [17](../../src/queue.rs#L17) | receiver-type-required |
| `execute_queue_transaction` | `ledger         .projection` | [17](../../src/queue.rs#L17) | receiver-type-required |
| `execute_queue_transaction` | `projection         .origin_tuples         .get(&transaction.retract_origin)         .copied` | [20](../../src/queue.rs#L20) | receiver-type-required |
| `execute_queue_transaction` | `projection         .origin_tuples         .get` | [20](../../src/queue.rs#L20) | receiver-type-required |
| `execute_queue_transaction` | `transaction         .replacement_origin()         .and_then(&#124;origin&#124; projection.origin_tuples.get(origin))         .copied` | [24](../../src/queue.rs#L24) | receiver-type-required |
| `execute_queue_transaction` | `transaction         .replacement_origin()         .and_then` | [24](../../src/queue.rs#L24) | receiver-type-required |
| `execute_queue_transaction` | `transaction         .replacement_origin` | [24](../../src/queue.rs#L24) | receiver-type-required |
| `execute_queue_transaction` | `projection.origin_tuples.get` | [26](../../src/queue.rs#L26) | receiver-type-required |
| `execute_queue_transaction` | `replacement_seq.is_some` | [28](../../src/queue.rs#L28) | receiver-type-required |
| `execute_queue_transaction` | `retract_seq.is_none` | [28](../../src/queue.rs#L28), [37](../../src/queue.rs#L37) | receiver-type-required |
| `execute_queue_transaction` | `Err` | [29](../../src/queue.rs#L29), [45](../../src/queue.rs#L45), [56](../../src/queue.rs#L56), [76](../../src/queue.rs#L76), [101](../../src/queue.rs#L101) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `StoreError::Corruption(             "queue transaction replacement exists without its retraction".to_owned(),         )         .into` | [29](../../src/queue.rs#L29) | receiver-type-required |
| `execute_queue_transaction` | `StoreError::Corruption` | [29](../../src/queue.rs#L29), [45](../../src/queue.rs#L45), [56](../../src/queue.rs#L56), [76](../../src/queue.rs#L76), [101](../../src/queue.rs#L101) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `"queue transaction replacement exists without its retraction".to_owned` | [30](../../src/queue.rs#L30) | receiver-type-required |
| `execute_queue_transaction` | `queue_target` | [35](../../src/queue.rs#L35) | [tekes-worker::queue::queue_target](../../src/queue.rs#L184) |
| `execute_queue_transaction` | `retract_seq.is_some` | [35](../../src/queue.rs#L35), [55](../../src/queue.rs#L55), [100](../../src/queue.rs#L100), [120](../../src/queue.rs#L120) | receiver-type-required |
| `execute_queue_transaction` | `Ok` | [38](../../src/queue.rs#L38), [61](../../src/queue.rs#L61), [106](../../src/queue.rs#L106), [163](../../src/queue.rs#L163) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `queue_rejected` | [38](../../src/queue.rs#L38), [61](../../src/queue.rs#L61), [106](../../src/queue.rs#L106) | [tekes-worker::queue::queue_rejected](../../src/queue.rs#L173) |
| `execute_queue_transaction` | `StoreError::Corruption(                 "durable queue retraction no longer has a validator-checkable target".to_owned(),             )             .into` | [45](../../src/queue.rs#L45) | receiver-type-required |
| `execute_queue_transaction` | `"durable queue retraction no longer has a validator-checkable target".to_owned` | [46](../../src/queue.rs#L46) | receiver-type-required |
| `execute_queue_transaction` | `queue_steer_window_open` | [53](../../src/queue.rs#L53) | [tekes-worker::queue::queue_steer_window_open](../../src/queue.rs#L279) |
| `execute_queue_transaction` | `StoreError::Corruption(                 "durable queue retraction belongs to an invalid steer transaction".to_owned(),             )             .into` | [56](../../src/queue.rs#L56) | receiver-type-required |
| `execute_queue_transaction` | `"durable queue retraction belongs to an invalid steer transaction".to_owned` | [57](../../src/queue.rs#L57) | receiver-type-required |
| `execute_queue_transaction` | `StoreError::Corruption(                     "queue edit steer bit does not match its target".to_owned(),                 )                 .into` | [76](../../src/queue.rs#L76) | receiver-type-required |
| `execute_queue_transaction` | `"queue edit steer bit does not match its target".to_owned` | [77](../../src/queue.rs#L77) | receiver-type-required |
| `execute_queue_transaction` | `Some` | [81](../../src/queue.rs#L81), [89](../../src/queue.rs#L89), [109](../../src/queue.rs#L109) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `content.clone` | [82](../../src/queue.rs#L82) | receiver-type-required |
| `execute_queue_transaction` | `assets.clone` | [84](../../src/queue.rs#L84) | receiver-type-required |
| `execute_queue_transaction` | `target.submission.clone` | [85](../../src/queue.rs#L85), [93](../../src/queue.rs#L93) | receiver-type-required |
| `execute_queue_transaction` | `target.content.clone` | [90](../../src/queue.rs#L90) | receiver-type-required |
| `execute_queue_transaction` | `target.assets.clone` | [92](../../src/queue.rs#L92) | receiver-type-required |
| `execute_queue_transaction` | `verify_queue_assets` | [98](../../src/queue.rs#L98) | [tekes-worker::queue::verify_queue_assets](../../src/queue.rs#L288) |
| `execute_queue_transaction` | `assets.as_deref().unwrap_or_default` | [98](../../src/queue.rs#L98) | receiver-type-required |
| `execute_queue_transaction` | `assets.as_deref` | [98](../../src/queue.rs#L98) | receiver-type-required |
| `execute_queue_transaction` | `StoreError::Corruption(format!(                 "queue replacement asset became invalid after retraction: {error}"             ))             .into` | [101](../../src/queue.rs#L101) | receiver-type-required |
| `execute_queue_transaction` | `"CORRUPT".to_owned` | [109](../../src/queue.rs#L109) | receiver-type-required |
| `execute_queue_transaction` | `validate_existing_retraction` | [114](../../src/queue.rs#L114) | [tekes-worker::queue::validate_existing_retraction](../../src/queue.rs#L304) |
| `execute_queue_transaction` | `replacement.as_ref` | [116](../../src/queue.rs#L116) | receiver-type-required |
| `execute_queue_transaction` | `validate_existing_replacement` | [117](../../src/queue.rs#L117) | [tekes-worker::queue::validate_existing_replacement](../../src/queue.rs#L323) |
| `execute_queue_transaction` | `ledger.next_seq` | [124](../../src/queue.rs#L124), [144](../../src/queue.rs#L144) | receiver-type-required |
| `execute_queue_transaction` | `origin_event` | [125](../../src/queue.rs#L125), [145](../../src/queue.rs#L145) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `object.insert` | [126](../../src/queue.rs#L126), [146](../../src/queue.rs#L146), [147](../../src/queue.rs#L147), [149](../../src/queue.rs#L149), [152](../../src/queue.rs#L152) | receiver-type-required |
| `execute_queue_transaction` | `"supersedes".to_owned` | [127](../../src/queue.rs#L127) | receiver-type-required |
| `execute_queue_transaction` | `ledger.append_contract` | [130](../../src/queue.rs#L130), [154](../../src/queue.rs#L154) | receiver-type-required |
| `execute_queue_transaction` | `make_event` | [131](../../src/queue.rs#L131), [155](../../src/queue.rs#L155) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `Value::Object` | [131](../../src/queue.rs#L131), [155](../../src/queue.rs#L155) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `BarrierContext::default` | [132](../../src/queue.rs#L132), [156](../../src/queue.rs#L156) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `transaction                 .replacement_origin()                 .expect` | [141](../../src/queue.rs#L141) | receiver-type-required |
| `execute_queue_transaction` | `transaction                 .replacement_origin` | [141](../../src/queue.rs#L141) | receiver-type-required |
| `execute_queue_transaction` | `"content".to_owned` | [146](../../src/queue.rs#L146) | receiver-type-required |
| `execute_queue_transaction` | `serde_json::to_value` | [146](../../src/queue.rs#L146), [149](../../src/queue.rs#L149) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `"steer".to_owned` | [147](../../src/queue.rs#L147) | receiver-type-required |
| `execute_queue_transaction` | `Value::Bool` | [147](../../src/queue.rs#L147) | external-constructor-callback-or-unresolved |
| `execute_queue_transaction` | `"assets".to_owned` | [149](../../src/queue.rs#L149) | receiver-type-required |
| `execute_queue_transaction` | `"submission".to_owned` | [152](../../src/queue.rs#L152) | receiver-type-required |
| `execute_queue_transaction` | `transaction.delivery.clone` | [164](../../src/queue.rs#L164) | receiver-type-required |
| `queue_rejected` | `transaction.delivery.clone` | [179](../../src/queue.rs#L179) | receiver-type-required |
| `queue_target` | `ledger         .projection()         .ok_or` | [189](../../src/queue.rs#L189) | receiver-type-required |
| `queue_target` | `ledger         .projection` | [189](../../src/queue.rs#L189) | receiver-type-required |
| `queue_target` | `projection         .events         .iter()         .find` | [192](../../src/queue.rs#L192) | receiver-type-required |
| `queue_target` | `projection         .events         .iter` | [192](../../src/queue.rs#L192) | receiver-type-required |
| `queue_target` | `event.seq` | [195](../../src/queue.rs#L195) | receiver-type-required |
| `queue_target` | `Ok` | [197](../../src/queue.rs#L197), [200](../../src/queue.rs#L200), [232](../../src/queue.rs#L232), [256](../../src/queue.rs#L256), [271](../../src/queue.rs#L271) | external-constructor-callback-or-unresolved |
| `queue_target` | `event.string_field` | [199](../../src/queue.rs#L199) | receiver-type-required |
| `queue_target` | `Some` | [199](../../src/queue.rs#L199), [207](../../src/queue.rs#L207), [216](../../src/queue.rs#L216), [219](../../src/queue.rs#L219), [225](../../src/queue.rs#L225), [227](../../src/queue.rs#L227), [236](../../src/queue.rs#L236), [240](../../src/queue.rs#L240), [250](../../src/queue.rs#L250), [251](../../src/queue.rs#L251), [271](../../src/queue.rs#L271) | external-constructor-callback-or-unresolved |
| `queue_target` | `projection.events.iter().any` | [203](../../src/queue.rs#L203), [235](../../src/queue.rs#L235) | receiver-type-required |
| `queue_target` | `projection.events.iter` | [203](../../src/queue.rs#L203), [235](../../src/queue.rs#L235) | receiver-type-required |
| `queue_target` | `event_json` | [204](../../src/queue.rs#L204), [244](../../src/queue.rs#L244), [259](../../src/queue.rs#L259) | external-constructor-callback-or-unresolved |
| `queue_target` | `candidate.string_field` | [207](../../src/queue.rs#L207), [219](../../src/queue.rs#L219), [236](../../src/queue.rs#L236) | receiver-type-required |
| `queue_target` | `value                 .get("trigger")                 .and_then(Value::as_object)                 .and_then(&#124;trigger&#124; trigger.get("inputs"))                 .and_then(Value::as_array)                 .is_some_and` | [208](../../src/queue.rs#L208) | receiver-type-required |
| `queue_target` | `value                 .get("trigger")                 .and_then(Value::as_object)                 .and_then(&#124;trigger&#124; trigger.get("inputs"))                 .and_then` | [208](../../src/queue.rs#L208) | receiver-type-required |
| `queue_target` | `value                 .get("trigger")                 .and_then(Value::as_object)                 .and_then` | [208](../../src/queue.rs#L208) | receiver-type-required |
| `queue_target` | `value                 .get("trigger")                 .and_then` | [208](../../src/queue.rs#L208) | receiver-type-required |
| `queue_target` | `value                 .get` | [208](../../src/queue.rs#L208), [220](../../src/queue.rs#L220) | receiver-type-required |
| `queue_target` | `trigger.get` | [211](../../src/queue.rs#L211) | receiver-type-required |
| `queue_target` | `inputs                         .iter()                         .any` | [214](../../src/queue.rs#L214) | receiver-type-required |
| `queue_target` | `inputs                         .iter` | [214](../../src/queue.rs#L214) | receiver-type-required |
| `queue_target` | `seq.as_u64` | [216](../../src/queue.rs#L216) | receiver-type-required |
| `queue_target` | `value                 .get("admits")                 .and_then(Value::as_array)                 .is_some_and` | [220](../../src/queue.rs#L220) | receiver-type-required |
| `queue_target` | `value                 .get("admits")                 .and_then` | [220](../../src/queue.rs#L220) | receiver-type-required |
| `queue_target` | `ranges.iter().any` | [224](../../src/queue.rs#L224), [249](../../src/queue.rs#L249) | receiver-type-required |
| `queue_target` | `ranges.iter` | [224](../../src/queue.rs#L224), [249](../../src/queue.rs#L249) | receiver-type-required |
| `queue_target` | `range.get("from").and_then` | [225](../../src/queue.rs#L225), [250](../../src/queue.rs#L250) | receiver-type-required |
| `queue_target` | `range.get` | [225](../../src/queue.rs#L225), [226](../../src/queue.rs#L226), [250](../../src/queue.rs#L250), [251](../../src/queue.rs#L251) | receiver-type-required |
| `queue_target` | `range.get("to").and_then` | [226](../../src/queue.rs#L226), [251](../../src/queue.rs#L251) | receiver-type-required |
| `queue_target` | `candidate.origin_tuple().ok().flatten().as_ref` | [240](../../src/queue.rs#L240) | receiver-type-required |
| `queue_target` | `candidate.origin_tuple().ok().flatten` | [240](../../src/queue.rs#L240) | receiver-type-required |
| `queue_target` | `candidate.origin_tuple().ok` | [240](../../src/queue.rs#L240) | receiver-type-required |
| `queue_target` | `candidate.origin_tuple` | [240](../../src/queue.rs#L240) | receiver-type-required |
| `queue_target` | `event_json(candidate)             .ok()             .and_then(&#124;value&#124; value.get("supersedes").cloned())             .and_then(&#124;value&#124; value.as_array().cloned())             .is_some_and` | [244](../../src/queue.rs#L244) | receiver-type-required |
| `queue_target` | `event_json(candidate)             .ok()             .and_then(&#124;value&#124; value.get("supersedes").cloned())             .and_then` | [244](../../src/queue.rs#L244) | receiver-type-required |
| `queue_target` | `event_json(candidate)             .ok()             .and_then` | [244](../../src/queue.rs#L244) | receiver-type-required |
| `queue_target` | `event_json(candidate)             .ok` | [244](../../src/queue.rs#L244) | receiver-type-required |
| `queue_target` | `value.get("supersedes").cloned` | [246](../../src/queue.rs#L246) | receiver-type-required |
| `queue_target` | `value.get` | [246](../../src/queue.rs#L246), [273](../../src/queue.rs#L273), [275](../../src/queue.rs#L275) | receiver-type-required |
| `queue_target` | `value.as_array().cloned` | [247](../../src/queue.rs#L247) | receiver-type-required |
| `queue_target` | `value.as_array` | [247](../../src/queue.rs#L247) | receiver-type-required |
| `queue_target` | `serde_json::from_value` | [260](../../src/queue.rs#L260) | external-constructor-callback-or-unresolved |
| `queue_target` | `value             .get("content")             .cloned()             .ok_or` | [261](../../src/queue.rs#L261) | receiver-type-required |
| `queue_target` | `value             .get("content")             .cloned` | [261](../../src/queue.rs#L261) | receiver-type-required |
| `queue_target` | `value             .get` | [261](../../src/queue.rs#L261) | receiver-type-required |
| `queue_target` | `value         .get("assets")         .cloned()         .map(serde_json::from_value)         .transpose` | [266](../../src/queue.rs#L266) | receiver-type-required |
| `queue_target` | `value         .get("assets")         .cloned()         .map` | [266](../../src/queue.rs#L266) | receiver-type-required |
| `queue_target` | `value         .get("assets")         .cloned` | [266](../../src/queue.rs#L266) | receiver-type-required |
| `queue_target` | `value         .get` | [266](../../src/queue.rs#L266) | receiver-type-required |
| `queue_target` | `value.get("steer").and_then(Value::as_bool).unwrap_or` | [273](../../src/queue.rs#L273) | receiver-type-required |
| `queue_target` | `value.get("steer").and_then` | [273](../../src/queue.rs#L273) | receiver-type-required |
| `queue_target` | `value.get("submission").cloned` | [275](../../src/queue.rs#L275) | receiver-type-required |
| `queue_steer_window_open` | `ledger.projection().is_some_and` | [280](../../src/queue.rs#L280) | receiver-type-required |
| `queue_steer_window_open` | `ledger.projection` | [280](../../src/queue.rs#L280) | receiver-type-required |
| `queue_steer_window_open` | `projection.latest_turn.is_some` | [281](../../src/queue.rs#L281) | receiver-type-required |
| `verify_queue_assets` | `ledger         .path()         .parent()         .ok_or_else(&#124;&#124; StoreError::Corruption("ledger has no thread folder".to_owned()))?         .join` | [292](../../src/queue.rs#L292) | receiver-type-required |
| `verify_queue_assets` | `ledger         .path()         .parent()         .ok_or_else` | [292](../../src/queue.rs#L292) | receiver-type-required |
| `verify_queue_assets` | `ledger         .path()         .parent` | [292](../../src/queue.rs#L292) | receiver-type-required |
| `verify_queue_assets` | `ledger         .path` | [292](../../src/queue.rs#L292) | receiver-type-required |
| `verify_queue_assets` | `StoreError::Corruption` | [295](../../src/queue.rs#L295) | external-constructor-callback-or-unresolved |
| `verify_queue_assets` | `"ledger has no thread folder".to_owned` | [295](../../src/queue.rs#L295) | receiver-type-required |
| `verify_queue_assets` | `AssetStore::new` | [297](../../src/queue.rs#L297) | external-constructor-callback-or-unresolved |
| `verify_queue_assets` | `store.verify_named` | [299](../../src/queue.rs#L299) | receiver-type-required |
| `verify_queue_assets` | `Ok` | [301](../../src/queue.rs#L301) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `event_at` | [309](../../src/queue.rs#L309) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `event_json(event).map_err` | [310](../../src/queue.rs#L310) | receiver-type-required |
| `validate_existing_retraction` | `event_json` | [310](../../src/queue.rs#L310) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `StoreError::Corruption` | [310](../../src/queue.rs#L310), [316](../../src/queue.rs#L316) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `error.to_string` | [310](../../src/queue.rs#L310) | receiver-type-required |
| `validate_existing_retraction` | `event.string_field` | [312](../../src/queue.rs#L312) | receiver-type-required |
| `validate_existing_retraction` | `Some` | [312](../../src/queue.rs#L312), [313](../../src/queue.rs#L313), [314](../../src/queue.rs#L314) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `event.origin_tuple` | [313](../../src/queue.rs#L313) | receiver-type-required |
| `validate_existing_retraction` | `transaction.retract_origin.clone` | [313](../../src/queue.rs#L313) | receiver-type-required |
| `validate_existing_retraction` | `value.get` | [314](../../src/queue.rs#L314) | receiver-type-required |
| `validate_existing_retraction` | `Err` | [316](../../src/queue.rs#L316) | external-constructor-callback-or-unresolved |
| `validate_existing_retraction` | `"queue transaction retraction origin resolves to mismatched bytes".to_owned` | [317](../../src/queue.rs#L317) | receiver-type-required |
| `validate_existing_retraction` | `Ok` | [320](../../src/queue.rs#L320) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `event_at` | [334](../../src/queue.rs#L334) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `event_json(event).map_err` | [335](../../src/queue.rs#L335) | receiver-type-required |
| `validate_existing_replacement` | `event_json` | [335](../../src/queue.rs#L335) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `StoreError::Corruption` | [335](../../src/queue.rs#L335), [337](../../src/queue.rs#L337), [343](../../src/queue.rs#L343), [351](../../src/queue.rs#L351) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `error.to_string` | [335](../../src/queue.rs#L335), [337](../../src/queue.rs#L337), [343](../../src/queue.rs#L343) | receiver-type-required |
| `validate_existing_replacement` | `serde_json::to_value(&replacement.0)         .map_err` | [336](../../src/queue.rs#L336) | receiver-type-required |
| `validate_existing_replacement` | `serde_json::to_value` | [336](../../src/queue.rs#L336) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `replacement         .2         .as_ref()         .map(serde_json::to_value)         .transpose()         .map_err` | [338](../../src/queue.rs#L338) | receiver-type-required |
| `validate_existing_replacement` | `replacement         .2         .as_ref()         .map(serde_json::to_value)         .transpose` | [338](../../src/queue.rs#L338) | receiver-type-required |
| `validate_existing_replacement` | `replacement         .2         .as_ref()         .map` | [338](../../src/queue.rs#L338) | receiver-type-required |
| `validate_existing_replacement` | `replacement         .2         .as_ref` | [338](../../src/queue.rs#L338) | receiver-type-required |
| `validate_existing_replacement` | `event.string_field` | [344](../../src/queue.rs#L344) | receiver-type-required |
| `validate_existing_replacement` | `Some` | [344](../../src/queue.rs#L344), [346](../../src/queue.rs#L346), [347](../../src/queue.rs#L347) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `event.origin_tuple` | [345](../../src/queue.rs#L345) | receiver-type-required |
| `validate_existing_replacement` | `transaction.replacement_origin().cloned` | [345](../../src/queue.rs#L345) | receiver-type-required |
| `validate_existing_replacement` | `transaction.replacement_origin` | [345](../../src/queue.rs#L345) | receiver-type-required |
| `validate_existing_replacement` | `value.get` | [346](../../src/queue.rs#L346), [347](../../src/queue.rs#L347), [348](../../src/queue.rs#L348), [349](../../src/queue.rs#L349) | receiver-type-required |
| `validate_existing_replacement` | `value.get("steer").and_then` | [347](../../src/queue.rs#L347) | receiver-type-required |
| `validate_existing_replacement` | `assets.as_ref` | [348](../../src/queue.rs#L348) | receiver-type-required |
| `validate_existing_replacement` | `replacement.3.as_ref` | [349](../../src/queue.rs#L349) | receiver-type-required |
| `validate_existing_replacement` | `Err` | [351](../../src/queue.rs#L351) | external-constructor-callback-or-unresolved |
| `validate_existing_replacement` | `"queue transaction replacement origin resolves to mismatched bytes".to_owned` | [352](../../src/queue.rs#L352) | receiver-type-required |
| `validate_existing_replacement` | `Ok` | [355](../../src/queue.rs#L355) | external-constructor-callback-or-unresolved |
