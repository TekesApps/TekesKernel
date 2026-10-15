# schema

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::READER_VERSION](../../src/lib.rs#L20) | const_item | `pub` |  |
| [schema::SESSION_SCOPE_READER](../../src/lib.rs#L23) | const_item | `pub` |  |
| [schema::FOLDERS_READER](../../src/lib.rs#L26) | const_item | `pub` |  |
| [schema::GENESIS_INHERITED_FIELDS](../../src/lib.rs#L32) | const_item | `pub` |  |
| [schema::effective_session_genesis](../../src/lib.rs#L48) | function_item | `pub` |  |
| [schema::GENESIS_OPERATION_FIELDS](../../src/lib.rs#L82) | const_item | `pub` |  |
| [schema::DEFAULT_ENVIRONMENT](../../src/lib.rs#L99) | const_item | `pub` |  |
| [schema::SchemaError](../../src/lib.rs#L104) | enum_item | `pub` |  |
| [schema::SchemaError::event](../../src/lib.rs#L116) | function_item | `pub(crate)` |  |
| [schema::SchemaError::constraint](../../src/lib.rs#L123) | function_item | `pub(crate)` |  |
| [schema::SchemaError::rule](../../src/lib.rs#L132) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Event` | `event::Event` | `pub` |
| `EventKind` | `event::EventKind` | `pub` |
| `Visibility` | `event::Visibility` | `pub` |
| `LedgerProjection` | `fold::LedgerProjection` | `pub` |
| `LedgerValidator` | `fold::LedgerValidator` | `pub` |
| `LifecycleFacts` | `fold::LifecycleFacts` | `pub` |
| `RunRange` | `fold::RunRange` | `pub` |
| `validate_ledger` | `fold::validate_ledger` | `pub` |
| `IJsonValue` | `ijson::IJsonValue` | `pub` |
| `Block` | `types::Block` | `pub` |
| `OriginTuple` | `types::OriginTuple` | `pub` |
| `ResumePolicy` | `types::ResumePolicy` | `pub` |
| `SeqRange` | `types::SeqRange` | `pub` |
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `schema::event` | `private` |  |
| `schema::fold` | `private` |  |
| `schema::ijson` | `private` |  |
| `schema::metrics` | `pub` |  |
| `schema::types` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::SchemaError::event"]
  n1["schema::SchemaError::constraint"]
  n2["schema::SchemaError::rule"]
  n3["schema::effective_session_genesis"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `effective_session_genesis` | `events.first` | [51](../../src/lib.rs#L51) | receiver-type-required |
| `effective_session_genesis` | `Ok` | [52](../../src/lib.rs#L52), [78](../../src/lib.rs#L78) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `serde_json::to_value` | [54](../../src/lib.rs#L54), [59](../../src/lib.rs#L59) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `first.raw` | [54](../../src/lib.rs#L54) | receiver-type-required |
| `effective_session_genesis` | `events.iter().skip` | [55](../../src/lib.rs#L55) | receiver-type-required |
| `effective_session_genesis` | `events.iter` | [55](../../src/lib.rs#L55) | receiver-type-required |
| `effective_session_genesis` | `event.kind` | [56](../../src/lib.rs#L56) | receiver-type-required |
| `effective_session_genesis` | `event.raw` | [59](../../src/lib.rs#L59) | receiver-type-required |
| `effective_session_genesis` | `raw             .get("scope_change")             .and_then` | [60](../../src/lib.rs#L60) | receiver-type-required |
| `effective_session_genesis` | `raw             .get` | [60](../../src/lib.rs#L60) | receiver-type-required |
| `effective_session_genesis` | `change.get` | [71](../../src/lib.rs#L71) | receiver-type-required |
| `effective_session_genesis` | `next.clone` | [72](../../src/lib.rs#L72) | receiver-type-required |
| `effective_session_genesis` | `Some` | [78](../../src/lib.rs#L78) | external-constructor-callback-or-unresolved |
| `event` | `message.into` | [119](../../src/lib.rs#L119) | receiver-type-required |
| `constraint` | `message.into` | [127](../../src/lib.rs#L127) | receiver-type-required |
| `rule` | `Some` | [134](../../src/lib.rs#L134) | external-constructor-callback-or-unresolved |
