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
| [tekes-worker::control_channel::RuntimeCancellation::take_deferred](../../src/control_channel.rs#L55) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::park_delivery](../../src/control_channel.rs#L62) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::RuntimeCancellation::take_parked_deliveries](../../src/control_channel.rs#L69) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ControlLines](../../src/control_channel.rs#L74) | struct_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ControlLines::drain_ready](../../src/control_channel.rs#L83) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::Item](../../src/control_channel.rs#L100) | type_item | `private` |  |
| [tekes-worker::control_channel::ControlLines::next](../../src/control_channel.rs#L102) | function_item | `private` |  |
| [tekes-worker::control_channel::ProtocolFailure](../../src/control_channel.rs#L117) | struct_item | `pub(crate)` |  |
| [tekes-worker::control_channel::ProtocolFailure::fmt](../../src/control_channel.rs#L120) | function_item | `private` |  |
| [tekes-worker::control_channel::spawn_control_reader](../../src/control_channel.rs#L127) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::read_control_lines](../../src/control_channel.rs#L140) | function_item | `private` |  |
| [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L145) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::control_line_key](../../src/control_channel.rs#L170) | function_item | `pub(crate)` |  |
| [tekes-worker::control_channel::is_stop_control_line](../../src/control_channel.rs#L178) | function_item | `private` |  |
| [tekes-worker::control_channel::is_delivery_control_line](../../src/control_channel.rs#L185) | function_item | `pub(crate)` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–18: 5 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-worker::control_channel::ControlLines::next"]
  n1["tekes-worker::control_channel::ProtocolFailure::fmt"]
  n2["tekes-worker::control_channel::spawn_control_reader"]
  n3["tekes-worker::control_channel::read_control_lines"]
  n4["tekes-worker::control_channel::forward_control_lines"]
  n5["tekes-worker::control_channel::control_line_key"]
  n6["tekes-worker::control_channel::is_stop_control_line"]
  n7["tekes-worker::control_channel::is_delivery_control_line"]
  n8["tekes-worker::control_channel::RuntimeCancellation::cancel"]
  n9["tekes-worker::control_channel::RuntimeCancellation::stop_requested"]
  n10["tekes-worker::control_channel::RuntimeCancellation::supervisor_lost"]
  n11["tekes-worker::control_channel::RuntimeCancellation::mark_protocol_failed"]
  n12["tekes-worker::control_channel::RuntimeCancellation::protocol_failed"]
  n13["tekes-worker::control_channel::RuntimeCancellation::defer"]
  n14["tekes-worker::control_channel::RuntimeCancellation::take_deferred"]
  n15["tekes-worker::control_channel::RuntimeCancellation::park_delivery"]
  n16["tekes-worker::control_channel::RuntimeCancellation::take_parked_deliveries"]
  n17["tekes-worker::control_channel::ControlLines::drain_ready"]
  n2 --> n3
  n3 --> n4
  n4 --> n6
  n4 --> n7
  n6 --> n5
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
| `take_deferred` | `self.deferred             .lock()             .expect("control defer lock")             .pop_front` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `take_deferred` | `self.deferred             .lock()             .expect` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `take_deferred` | `self.deferred             .lock` | [56](../../src/control_channel.rs#L56) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock()             .expect("control park lock")             .push_back` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock()             .expect` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `park_delivery` | `self.parked_deliveries             .lock` | [63](../../src/control_channel.rs#L63) | receiver-type-required |
| `take_parked_deliveries` | `std::mem::take(&mut *self.parked_deliveries.lock().expect("control park lock")).into` | [70](../../src/control_channel.rs#L70) | receiver-type-required |
| `take_parked_deliveries` | `std::mem::take` | [70](../../src/control_channel.rs#L70) | external-constructor-callback-or-unresolved |
| `take_parked_deliveries` | `self.parked_deliveries.lock().expect` | [70](../../src/control_channel.rs#L70) | receiver-type-required |
| `take_parked_deliveries` | `self.parked_deliveries.lock` | [70](../../src/control_channel.rs#L70) | receiver-type-required |
| `drain_ready` | `self.cancellation.take_parked_deliveries` | [84](../../src/control_channel.rs#L84) | receiver-type-required |
| `drain_ready` | `ready.extend` | [85](../../src/control_channel.rs#L85) | receiver-type-required |
| `drain_ready` | `std::mem::take` | [85](../../src/control_channel.rs#L85) | external-constructor-callback-or-unresolved |
| `drain_ready` | `self                 .cancellation                 .deferred                 .lock()                 .expect` | [86](../../src/control_channel.rs#L86) | receiver-type-required |
| `drain_ready` | `self                 .cancellation                 .deferred                 .lock` | [86](../../src/control_channel.rs#L86) | receiver-type-required |
| `drain_ready` | `self.receiver.try_recv` | [92](../../src/control_channel.rs#L92) | receiver-type-required |
| `drain_ready` | `ready.push` | [93](../../src/control_channel.rs#L93) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock()             .expect("control defer lock")             .pop_front` | [103](../../src/control_channel.rs#L103) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock()             .expect` | [103](../../src/control_channel.rs#L103) | receiver-type-required |
| `next` | `self             .cancellation             .deferred             .lock` | [103](../../src/control_channel.rs#L103) | receiver-type-required |
| `next` | `Some` | [110](../../src/control_channel.rs#L110) | external-constructor-callback-or-unresolved |
| `next` | `Ok` | [110](../../src/control_channel.rs#L110) | external-constructor-callback-or-unresolved |
| `next` | `self.receiver.recv().ok` | [112](../../src/control_channel.rs#L112) | receiver-type-required |
| `next` | `self.receiver.recv` | [112](../../src/control_channel.rs#L112) | receiver-type-required |
| `fmt` | `formatter.write_str` | [121](../../src/control_channel.rs#L121) | receiver-type-required |
| `spawn_control_reader` | `sync_channel` | [131](../../src/control_channel.rs#L131) | external-constructor-callback-or-unresolved |
| `spawn_control_reader` | `cancellation.clone` | [132](../../src/control_channel.rs#L132) | receiver-type-required |
| `spawn_control_reader` | `std::thread::spawn` | [133](../../src/control_channel.rs#L133) | external-constructor-callback-or-unresolved |
| `spawn_control_reader` | `read_control_lines` | [133](../../src/control_channel.rs#L133) | [tekes-worker::control_channel::read_control_lines](../../src/control_channel.rs#L140) |
| `read_control_lines` | `io::stdin` | [141](../../src/control_channel.rs#L141) | external-constructor-callback-or-unresolved |
| `read_control_lines` | `forward_control_lines` | [142](../../src/control_channel.rs#L142) | [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L145) |
| `read_control_lines` | `stdin.lock().lines` | [142](../../src/control_channel.rs#L142) | receiver-type-required |
| `read_control_lines` | `stdin.lock` | [142](../../src/control_channel.rs#L142) | receiver-type-required |
| `forward_control_lines` | `is_stop_control_line` | [152](../../src/control_channel.rs#L152) | [tekes-worker::control_channel::is_stop_control_line](../../src/control_channel.rs#L178) |
| `forward_control_lines` | `cancellation.cancel` | [152](../../src/control_channel.rs#L152), [160](../../src/control_channel.rs#L160), [167](../../src/control_channel.rs#L167) | receiver-type-required |
| `forward_control_lines` | `is_delivery_control_line` | [153](../../src/control_channel.rs#L153) | [tekes-worker::control_channel::is_delivery_control_line](../../src/control_channel.rs#L185) |
| `forward_control_lines` | `cancellation.park_delivery` | [157](../../src/control_channel.rs#L157) | receiver-type-required |
| `forward_control_lines` | `line.clone` | [157](../../src/control_channel.rs#L157) | receiver-type-required |
| `forward_control_lines` | `sender.send(line).is_err` | [163](../../src/control_channel.rs#L163) | receiver-type-required |
| `forward_control_lines` | `sender.send` | [163](../../src/control_channel.rs#L163) | receiver-type-required |
| `control_line_key` | `serde_json::from_str::<Value>(line).ok` | [171](../../src/control_channel.rs#L171) | receiver-type-required |
| `control_line_key` | `serde_json::from_str::<Value>` | [171](../../src/control_channel.rs#L171) | external-constructor-callback-or-unresolved |
| `control_line_key` | `value.as_object` | [172](../../src/control_channel.rs#L172) | receiver-type-required |
| `control_line_key` | `(object.len() == 1)         .then(&#124;&#124; object.keys().next().cloned())         .flatten` | [173](../../src/control_channel.rs#L173) | receiver-type-required |
| `control_line_key` | `(object.len() == 1)         .then` | [173](../../src/control_channel.rs#L173) | receiver-type-required |
| `control_line_key` | `object.len` | [173](../../src/control_channel.rs#L173) | receiver-type-required |
| `control_line_key` | `object.keys().next().cloned` | [174](../../src/control_channel.rs#L174) | receiver-type-required |
| `control_line_key` | `object.keys().next` | [174](../../src/control_channel.rs#L174) | receiver-type-required |
| `control_line_key` | `object.keys` | [174](../../src/control_channel.rs#L174) | receiver-type-required |
| `is_stop_control_line` | `control_line_key(line).as_deref` | [179](../../src/control_channel.rs#L179) | receiver-type-required |
| `is_stop_control_line` | `control_line_key` | [179](../../src/control_channel.rs#L179) | [tekes-worker::control_channel::control_line_key](../../src/control_channel.rs#L170) |
| `is_stop_control_line` | `Some` | [179](../../src/control_channel.rs#L179) | external-constructor-callback-or-unresolved |
