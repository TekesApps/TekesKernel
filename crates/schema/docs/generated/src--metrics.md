# schema::metrics

[Package atlas](index.md) · [Source](../../src/metrics.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::metrics::LEDGER_DECODES](../../src/metrics.rs#L7) | static_item | `private` |  |
| [schema::metrics::LEDGER_DECODE_BYTES](../../src/metrics.rs#L8) | static_item | `private` |  |
| [schema::metrics::JOURNAL_PARSES](../../src/metrics.rs#L9) | static_item | `private` |  |
| [schema::metrics::JOURNAL_PARSE_BYTES](../../src/metrics.rs#L10) | static_item | `private` |  |
| [schema::metrics::WorkCounters](../../src/metrics.rs#L14) | struct_item | `pub` |  |
| [schema::metrics::WorkCounters::since](../../src/metrics.rs#L26) | function_item | `pub` |  |
| [schema::metrics::snapshot](../../src/metrics.rs#L41) | function_item | `pub` |  |
| [schema::metrics::note_ledger_decode](../../src/metrics.rs#L50) | function_item | `pub(crate)` |  |
| [schema::metrics::note_journal_parse](../../src/metrics.rs#L56) | function_item | `pub` |  |
| [schema::metrics::tests::counters_are_monotonic_and_since_subtracts](../../src/metrics.rs#L66) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `AtomicU64` | `std::sync::atomic::AtomicU64` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `schema::metrics::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::metrics::WorkCounters::since"]
  n1["schema::metrics::snapshot"]
  n2["schema::metrics::note_ledger_decode"]
  n3["schema::metrics::note_journal_parse"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `LEDGER_DECODES` | `AtomicU64::new` | [7](../../src/metrics.rs#L7) | external-constructor-callback-or-unresolved |
| `LEDGER_DECODE_BYTES` | `AtomicU64::new` | [8](../../src/metrics.rs#L8) | external-constructor-callback-or-unresolved |
| `JOURNAL_PARSES` | `AtomicU64::new` | [9](../../src/metrics.rs#L9) | external-constructor-callback-or-unresolved |
| `JOURNAL_PARSE_BYTES` | `AtomicU64::new` | [10](../../src/metrics.rs#L10) | external-constructor-callback-or-unresolved |
| `since` | `self.ledger_decodes.saturating_sub` | [28](../../src/metrics.rs#L28) | receiver-type-required |
| `since` | `self                 .ledger_decode_bytes                 .saturating_sub` | [29](../../src/metrics.rs#L29) | receiver-type-required |
| `since` | `self.journal_parses.saturating_sub` | [32](../../src/metrics.rs#L32) | receiver-type-required |
| `since` | `self                 .journal_parse_bytes                 .saturating_sub` | [33](../../src/metrics.rs#L33) | receiver-type-required |
| `snapshot` | `LEDGER_DECODES.load` | [43](../../src/metrics.rs#L43) | receiver-type-required |
| `snapshot` | `LEDGER_DECODE_BYTES.load` | [44](../../src/metrics.rs#L44) | receiver-type-required |
| `snapshot` | `JOURNAL_PARSES.load` | [45](../../src/metrics.rs#L45) | receiver-type-required |
| `snapshot` | `JOURNAL_PARSE_BYTES.load` | [46](../../src/metrics.rs#L46) | receiver-type-required |
| `note_ledger_decode` | `LEDGER_DECODES.fetch_add` | [51](../../src/metrics.rs#L51) | receiver-type-required |
| `note_ledger_decode` | `LEDGER_DECODE_BYTES.fetch_add` | [52](../../src/metrics.rs#L52) | receiver-type-required |
| `note_journal_parse` | `JOURNAL_PARSES.fetch_add` | [57](../../src/metrics.rs#L57) | receiver-type-required |
| `note_journal_parse` | `JOURNAL_PARSE_BYTES.fetch_add` | [58](../../src/metrics.rs#L58) | receiver-type-required |
| `counters_are_monotonic_and_since_subtracts` | `snapshot` | [67](../../src/metrics.rs#L67), [70](../../src/metrics.rs#L70) | external-constructor-callback-or-unresolved |
| `counters_are_monotonic_and_since_subtracts` | `note_ledger_decode` | [68](../../src/metrics.rs#L68) | external-constructor-callback-or-unresolved |
| `counters_are_monotonic_and_since_subtracts` | `note_journal_parse` | [69](../../src/metrics.rs#L69) | external-constructor-callback-or-unresolved |
| `counters_are_monotonic_and_since_subtracts` | `snapshot().since` | [70](../../src/metrics.rs#L70) | receiver-type-required |
