# schema

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::READER_VERSION](../../src/lib.rs#L25) | const_item | `pub` |  |
| [schema::EXECUTION_READER](../../src/lib.rs#L28) | const_item | `pub` |  |
| [schema::SESSION_SCOPE_READER](../../src/lib.rs#L31) | const_item | `pub` |  |
| [schema::FOLDERS_READER](../../src/lib.rs#L34) | const_item | `pub` |  |
| [schema::GENESIS_INHERITED_FIELDS](../../src/lib.rs#L40) | const_item | `pub` |  |
| [schema::effective_session_genesis](../../src/lib.rs#L57) | function_item | `pub` |  |
| [schema::GENESIS_OPERATION_FIELDS](../../src/lib.rs#L97) | const_item | `pub` |  |
| [schema::DEFAULT_ENVIRONMENT](../../src/lib.rs#L114) | const_item | `pub` |  |
| [schema::SchemaError](../../src/lib.rs#L119) | enum_item | `pub` |  |
| [schema::SchemaError::event](../../src/lib.rs#L131) | function_item | `pub(crate)` |  |
| [schema::SchemaError::constraint](../../src/lib.rs#L138) | function_item | `pub(crate)` |  |
| [schema::SchemaError::rule](../../src/lib.rs#L147) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Event` | `event::Event` | `pub` |
| `EventKind` | `event::EventKind` | `pub` |
| `Visibility` | `event::Visibility` | `pub` |
| `ExecutionConfig` | `execution::ExecutionConfig` | `pub` |
| `McpConnection` | `execution::McpConnection` | `pub` |
| `McpConnectionTransport` | `execution::McpConnectionTransport` | `pub` |
| `HistoricalLedger` | `fold::HistoricalLedger` | `pub` |
| `LedgerProjection` | `fold::LedgerProjection` | `pub` |
| `LedgerValidator` | `fold::LedgerValidator` | `pub` |
| `LifecycleFacts` | `fold::LifecycleFacts` | `pub` |
| `RunRange` | `fold::RunRange` | `pub` |
| `validate_historical_ledger` | `fold::validate_historical_ledger` | `pub` |
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
| `schema::execution` | `private` |  |
| `schema::fold` | `private` |  |
| `schema::ijson` | `private` |  |
| `schema::metrics` | `pub` |  |
| `schema::types` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::SchemaError::event"]
  n1["schema::SchemaError::constraint"]
  n2["schema::SchemaError::rule"]
  n3["schema::effective_session_genesis"]
  n3 --> n0
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `effective_session_genesis` | `events.first` | [60](../../src/lib.rs#L60) | receiver-type-required |
| `effective_session_genesis` | `Ok` | [61](../../src/lib.rs#L61), [93](../../src/lib.rs#L93) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `serde_json::to_value` | [63](../../src/lib.rs#L63), [68](../../src/lib.rs#L68) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `first.raw` | [63](../../src/lib.rs#L63) | receiver-type-required |
| `effective_session_genesis` | `events.iter().skip` | [64](../../src/lib.rs#L64) | receiver-type-required |
| `effective_session_genesis` | `events.iter` | [64](../../src/lib.rs#L64) | receiver-type-required |
| `effective_session_genesis` | `event.kind` | [65](../../src/lib.rs#L65) | receiver-type-required |
| `effective_session_genesis` | `event.raw` | [68](../../src/lib.rs#L68) | receiver-type-required |
| `effective_session_genesis` | `raw             .get("scope_change")             .and_then` | [69](../../src/lib.rs#L69) | receiver-type-required |
| `effective_session_genesis` | `raw             .get` | [69](../../src/lib.rs#L69) | receiver-type-required |
| `effective_session_genesis` | `value.get("execution").is_some` | [73](../../src/lib.rs#L73) | receiver-type-required |
| `effective_session_genesis` | `value.get` | [73](../../src/lib.rs#L73) | receiver-type-required |
| `effective_session_genesis` | `Err` | [74](../../src/lib.rs#L74) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `SchemaError::event` | [74](../../src/lib.rs#L74) | [schema::SchemaError::event](../../src/lib.rs#L131) |
| `effective_session_genesis` | `Some` | [75](../../src/lib.rs#L75), [93](../../src/lib.rs#L93) | external-constructor-callback-or-unresolved |
| `effective_session_genesis` | `event.seq` | [75](../../src/lib.rs#L75) | receiver-type-required |
| `effective_session_genesis` | `change.get` | [86](../../src/lib.rs#L86) | receiver-type-required |
| `effective_session_genesis` | `next.clone` | [87](../../src/lib.rs#L87) | receiver-type-required |
| `event` | `message.into` | [134](../../src/lib.rs#L134) | receiver-type-required |
| `constraint` | `message.into` | [142](../../src/lib.rs#L142) | receiver-type-required |
| `rule` | `Some` | [149](../../src/lib.rs#L149) | external-constructor-callback-or-unresolved |
