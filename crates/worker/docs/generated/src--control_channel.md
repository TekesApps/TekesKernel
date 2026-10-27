# tekes-worker::control_channel

[Package atlas](index.md) · [Source](../../src/control_channel.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::control_channel::CANCEL_NONE](../../src/control_channel.rs#L3) | const_item | `private` |  |
| [tekes-worker::control_channel::CANCEL_STOP](../../src/control_channel.rs#L4) | const_item | `pub(crate)` |  |
| [tekes-worker::control_channel::CANCEL_SUPERVISOR_LOSS](../../src/control_channel.rs#L5) | const_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation](../../src/control_channel.rs#L8) | struct_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::cancel](../../src/control_channel.rs#L24) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::stop_requested](../../src/control_channel.rs#L32) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::supervisor_lost](../../src/control_channel.rs#L36) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::mark_protocol_failed](../../src/control_channel.rs#L40) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::protocol_failed](../../src/control_channel.rs#L44) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::defer](../../src/control_channel.rs#L48) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::park_delivery](../../src/control_channel.rs#L55) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::take_parked_deliveries](../../src/control_channel.rs#L62) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ControlLines](../../src/control_channel.rs#L67) | struct_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ControlLines::drain_ready](../../src/control_channel.rs#L76) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::Item](../../src/control_channel.rs#L93) | type_item | `private` |  |
| [tekes-worker::control_channel::ControlLines::next](../../src/control_channel.rs#L95) | function_item | `private` |  |
| [tekes-worker::control_channel::ProtocolFailure](../../src/control_channel.rs#L110) | struct_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ProtocolFailure::fmt](../../src/control_channel.rs#L113) | function_item | `private` |  |
| [tekes-worker::control_channel::spawn_control_reader](../../src/control_channel.rs#L120) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::read_control_lines](../../src/control_channel.rs#L133) | function_item | `private` |  |
| [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L138) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::control_line_key](../../src/control_channel.rs#L163) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::is_stop_control_line](../../src/control_channel.rs#L171) | function_item | `private` |  |
| [tekes-worker::control_channel::is_delivery_control_line](../../src/control_channel.rs#L178) | function_item | `pub(crate)` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–17: 5 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-worker::control_channel::ProtocolFailure::fmt"]
  n1["tekes-worker::control_channel::spawn_control_reader"]
  n2["tekes-worker::control_channel::read_control_lines"]
  n3["tekes-worker::control_channel::forward_control_lines"]
  n4["tekes-worker::control_channel::control_line_key"]
  n5["tekes-worker::control_channel::is_stop_control_line"]
  n6["tekes-worker::control_channel::is_delivery_control_line"]
  n7["tekes-worker::control_channel::RuntimeCancellation::cancel"]
  n8["tekes-worker::control_channel::RuntimeCancellation::stop_requested"]
  n9["tekes-worker::control_channel::RuntimeCancellation::supervisor_lost"]
  n10["tekes-worker::control_channel::RuntimeCancellation::mark_protocol_failed"]
  n11["tekes-worker::control_channel::RuntimeCancellation::protocol_failed"]
  n12["tekes-worker::control_channel::RuntimeCancellation::defer"]
  n13["tekes-worker::control_channel::RuntimeCancellation::park_delivery"]
  n14["tekes-worker::control_channel::RuntimeCancellation::take_parked_deliveries"]
  n15["tekes-worker::control_channel::ControlLines::drain_ready"]
  n16["tekes-worker::control_channel::ControlLines::next"]
  n1 --> n2
  n2 --> n3
  n3 --> n5
  n3 --> n6
  n5 --> n4
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `cancel` | `self.cause                 .compare_exchange` | [26](../../src/control_channel.rs#L26) | receiver-type-required |
| `cancel` | `self.tools.cancel` | [28](../../src/control_channel.rs#L28) | receiver-type-required |
| `cancel` | `self.provider.store` | [29](../../src/control_channel.rs#L29) | receiver-type-required |
| `stop_requested` | `self.cause.load` | [33](../../src/control_channel.rs#L33) | receiver-type-required |
| `supervisor_lost` | `self.cause.load` | [37](../../src/control_channel.rs#L37) | receiver-type-required |
| `mark_protocol_failed` | `self.protocol_failed.store` | [41](../../src/control_channel.rs#L41) | receiver-type-required |
| `protocol_failed` | `self.protocol_failed.load` | [45](../../src/control_channel.rs#L45) | receiver-type-required |
| `defer` | `self.deferred             .lock()             .expect("control defer lock")             .push_back` | [49](../../src/control_channel.rs#L49) | receiver-type-required |
| `defer` | `self.deferred             .lock()             .expect` | [49](../../src/control_channel.rs#L49) | receiver-type-required |
| `defer` | `self.deferred             .lock` | [49](../../src/control_channel.rs#L49) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock()             .expect("control park lock")             .push_back` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock()             .expect` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `take_parked_deliveries` | `std::mem::take(&mut *self.parked_deliveries.lock().expect("control park lock")).into` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `take_parked_deliveries` | `std::mem::take` | [63](../../src/control_channel.rs#L63) | external-constructor-callback-or-unresolved |
| `take_parked_deliveries` | `self.parked_deliveries.lock().expect` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `take_parked_deliveries` | `self.parked_deliveries.lock` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `drain_ready` | `self.cancellation.take_parked_deliveries` | [77](../../src/control_channel.rs#L77) | receiver-type-required |
| `drain_ready` | `ready.extend` | [78](../../src/control_channel.rs#L78) | receiver-type-required |
| `drain_ready` | `std::mem::take` | [78](../../src/control_channel.rs#L78) | external-constructor-callback-or-unresolved |
| `drain_ready` | `self                 .cancellation                 .deferred                 .lock()                 .expect` | [79](../../src/control_channel.rs#L79) | receiver-type-required |
| `drain_ready` | `self                 .cancellation                 .deferred                 .lock` | [79](../../src/control_channel.rs#L79) | receiver-type-required |
| `drain_ready` | `self.receiver.try_recv` | [85](../../src/control_channel.rs#L85) | receiver-type-required |
| `drain_ready` | `ready.push` | [86](../../src/control_channel.rs#L86) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock()             .expect("control defer lock")             .pop_front` | [96](../../src/control_channel.rs#L96) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock()             .expect` | [96](../../src/control_channel.rs#L96) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock` | [96](../../src/control_channel.rs#L96) | receiver-type-required |
| `next` | `Some` | [103](../../src/control_channel.rs#L103) | external-constructor-callback-or-unresolved |
| `next` | `Ok` | [103](../../src/control_channel.rs#L103) | external-constructor-callback-or-unresolved |
| `next` | `self.receiver.recv().ok` | [105](../../src/control_channel.rs#L105) | receiver-type-required |
| `next` | `self.receiver.recv` | [105](../../src/control_channel.rs#L105) | receiver-type-required |
| `fmt` | `formatter.write_str` | [114](../../src/control_channel.rs#L114) | receiver-type-required |
| `spawn_control_reader` | `sync_channel` | [124](../../src/control_channel.rs#L124) | external-constructor-callback-or-unresolved |
| `spawn_control_reader` | `cancellation.clone` | [125](../../src/control_channel.rs#L125) | receiver-type-required |
| `spawn_control_reader` | `std::thread::spawn` | [126](../../src/control_channel.rs#L126) | external-constructor-callback-or-unresolved |
| `spawn_control_reader` | `read_control_lines` | [126](../../src/control_channel.rs#L126) | [tekes-worker::control_channel::read_control_lines](../../src/control_channel.rs#L133) |
| `read_control_lines` | `io::stdin` | [134](../../src/control_channel.rs#L134) | external-constructor-callback-or-unresolved |
| `read_control_lines` | `forward_control_lines` | [135](../../src/control_channel.rs#L135) | [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L138) |
| `read_control_lines` | `stdin.lock().lines` | [135](../../src/control_channel.rs#L135) | receiver-type-required |
| `read_control_lines` | `stdin.lock` | [135](../../src/control_channel.rs#L135) | receiver-type-required |
| `forward_control_lines` | `is_stop_control_line` | [145](../../src/control_channel.rs#L145) | [tekes-worker::control_channel::is_stop_control_line](../../src/control_channel.rs#L171) |
| `forward_control_lines` | `cancellation.cancel` | [145](../../src/control_channel.rs#L145), [153](../../src/control_channel.rs#L153), [160](../../src/control_channel.rs#L160) | receiver-type-required |
| `forward_control_lines` | `is_delivery_control_line` | [146](../../src/control_channel.rs#L146) | [tekes-worker::control_channel::is_delivery_control_line](../../src/control_channel.rs#L178) |
| `forward_control_lines` | `cancellation.park_delivery` | [150](../../src/control_channel.rs#L150) | receiver-type-required |
| `forward_control_lines` | `line.clone` | [150](../../src/control_channel.rs#L150) | receiver-type-required |
| `forward_control_lines` | `sender.send(line).is_err` | [156](../../src/control_channel.rs#L156) | receiver-type-required |
| `forward_control_lines` | `sender.send` | [156](../../src/control_channel.rs#L156) | receiver-type-required |
| `control_line_key` | `serde_json::from_str::<Value>(line).ok` | [164](../../src/control_channel.rs#L164) | receiver-type-required |
| `control_line_key` | `serde_json::from_str::<Value>` | [164](../../src/control_channel.rs#L164) | external-constructor-callback-or-unresolved |
| `control_line_key` | `value.as_object` | [165](../../src/control_channel.rs#L165) | receiver-type-required |
| `control_line_key` | `(object.len() == 1)         .then(&#124;&#124; object.keys().next().cloned())         .flatten` | [166](../../src/control_channel.rs#L166) | receiver-type-required |
| `control_line_key` | `(object.len() == 1)         .then` | [166](../../src/control_channel.rs#L166) | receiver-type-required |
| `control_line_key` | `object.len` | [166](../../src/control_channel.rs#L166) | receiver-type-required |
| `control_line_key` | `object.keys().next().cloned` | [167](../../src/control_channel.rs#L167) | receiver-type-required |
| `control_line_key` | `object.keys().next` | [167](../../src/control_channel.rs#L167) | receiver-type-required |
| `control_line_key` | `object.keys` | [167](../../src/control_channel.rs#L167) | receiver-type-required |
| `is_stop_control_line` | `control_line_key(line).as_deref` | [172](../../src/control_channel.rs#L172) | receiver-type-required |
| `is_stop_control_line` | `control_line_key` | [172](../../src/control_channel.rs#L172) | [tekes-worker::control_channel::control_line_key](../../src/control_channel.rs#L163) |
| `is_stop_control_line` | `Some` | [172](../../src/control_channel.rs#L172) | external-constructor-callback-or-unresolved |
