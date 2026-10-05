# engine::supervisor

[Package atlas](index.md) · [Source](../../src/supervisor.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [engine::supervisor::DrainAction](../../src/supervisor.rs#L4) | enum_item | `pub` |  |
| [engine::supervisor::RecoveryStage](../../src/supervisor.rs#L12) | enum_item | `pub` |  |
| [engine::supervisor::RECOVERY_ORDER](../../src/supervisor.rs#L19) | const_item | `pub` |  |
| [engine::supervisor::DrainTracker](../../src/supervisor.rs#L27) | struct_item | `pub` |  |
| [engine::supervisor::DrainTracker::new](../../src/supervisor.rs#L34) | function_item | `pub` |  |
| [engine::supervisor::DrainTracker::observe](../../src/supervisor.rs#L41) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–2: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::supervisor::DrainTracker::new"]
  n1["engine::supervisor::DrainTracker::observe"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `BTreeMap::new` | [37](../../src/supervisor.rs#L37) | external-constructor-callback-or-unresolved |
| `observe` | `self.busy_since.remove` | [49](../../src/supervisor.rs#L49), [53](../../src/supervisor.rs#L53) | receiver-type-required |
| `observe` | `self.busy_since.entry(line.to_owned()).or_insert` | [56](../../src/supervisor.rs#L56) | receiver-type-required |
| `observe` | `self.busy_since.entry` | [56](../../src/supervisor.rs#L56) | receiver-type-required |
| `observe` | `line.to_owned` | [56](../../src/supervisor.rs#L56) | receiver-type-required |
| `observe` | `now_tick.saturating_sub` | [57](../../src/supervisor.rs#L57) | receiver-type-required |
