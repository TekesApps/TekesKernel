# endpoint::history

[Package atlas](index.md) · [Source](../../src/history.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::history::HistoryPage](../../src/history.rs#L8) | struct_item | `pub` |  |
| [endpoint::history::history_page](../../src/history.rs#L13) | function_item | `pub` |  |
| [endpoint::history::is_append_origin_message](../../src/history.rs#L117) | function_item | `private` |  |
| [endpoint::history::HistoryError](../../src/history.rs#L128) | enum_item | `pub` |  |
| [endpoint::history::tests::record](../../src/history.rs#L142) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::history::tests::page_is_message_aligned_and_keeps_projection_group](../../src/history.rs#L161) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `JournalRecord` | `crate::JournalRecord` | `private` |
| `SessionHistoryEntry` | `crate::SessionHistoryEntry` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `*` | `super::*` | `private` |
| `SessionEvent` | `crate::SessionEvent` | `private` |
| `SurfaceOperation` | `crate::SurfaceOperation` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `endpoint::history::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–2: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::history::is_append_origin_message"]
  n1["endpoint::history::history_page"]
  n1 --> n0
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `history_page` | `max_messages.unwrap_or` | [18](../../src/history.rs#L18) | receiver-type-required |
| `history_page` | `Err` | [20](../../src/history.rs#L20), [94](../../src/history.rs#L94) | external-constructor-callback-or-unresolved |
| `history_page` | `before_seq.unwrap_or` | [22](../../src/history.rs#L22) | receiver-type-required |
| `history_page` | `records.len` | [22](../../src/history.rs#L22), [24](../../src/history.rs#L24), [93](../../src/history.rs#L93) | receiver-type-required |
| `history_page` | `records.partition_point` | [23](../../src/history.rs#L23) | receiver-type-required |
| `history_page` | `records[eligible_end].kernel_seqs.is_empty` | [24](../../src/history.rs#L24) | receiver-type-required |
| `history_page` | `records[eligible_end]             .kernel_seqs             .iter()             .copied()             .collect::<BTreeSet<_>>` | [25](../../src/history.rs#L25) | receiver-type-required |
| `history_page` | `records[eligible_end]             .kernel_seqs             .iter()             .copied` | [25](../../src/history.rs#L25) | receiver-type-required |
| `history_page` | `records[eligible_end]             .kernel_seqs             .iter` | [25](../../src/history.rs#L25) | receiver-type-required |
| `history_page` | `records[..eligible_end]             .iter()             .position` | [30](../../src/history.rs#L30) | receiver-type-required |
| `history_page` | `records[..eligible_end]             .iter` | [30](../../src/history.rs#L30) | receiver-type-required |
| `history_page` | `record.kernel_seqs.iter().any` | [32](../../src/history.rs#L32), [68](../../src/history.rs#L68) | receiver-type-required |
| `history_page` | `record.kernel_seqs.iter` | [32](../../src/history.rs#L32), [64](../../src/history.rs#L64), [68](../../src/history.rs#L68), [74](../../src/history.rs#L74) | receiver-type-required |
| `history_page` | `boundary.contains` | [32](../../src/history.rs#L32) | receiver-type-required |
| `history_page` | `Ok` | [38](../../src/history.rs#L38), [105](../../src/history.rs#L105) | external-constructor-callback-or-unresolved |
| `history_page` | `Vec::new` | [39](../../src/history.rs#L39) | external-constructor-callback-or-unresolved |
| `history_page` | `is_append_origin_message` | [49](../../src/history.rs#L49) | [endpoint::history::is_append_origin_message](../../src/history.rs#L117) |
| `history_page` | `records[lower..eligible_end]             .iter()             .flat_map(&#124;record&#124; record.kernel_seqs.iter().copied())             .collect::<BTreeSet<_>>` | [62](../../src/history.rs#L62) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]             .iter()             .flat_map` | [62](../../src/history.rs#L62) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]             .iter` | [62](../../src/history.rs#L62), [106](../../src/history.rs#L106) | receiver-type-required |
| `history_page` | `record.kernel_seqs.iter().copied` | [64](../../src/history.rs#L64), [74](../../src/history.rs#L74) | receiver-type-required |
| `history_page` | `records[..lower]             .iter()             .position` | [66](../../src/history.rs#L66) | receiver-type-required |
| `history_page` | `records[..lower]             .iter` | [66](../../src/history.rs#L66) | receiver-type-required |
| `history_page` | `group.contains` | [68](../../src/history.rs#L68) | receiver-type-required |
| `history_page` | `group.extend` | [71](../../src/history.rs#L71) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]                     .iter()                     .flat_map` | [72](../../src/history.rs#L72) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]                     .iter` | [72](../../src/history.rs#L72) | receiver-type-required |
| `history_page` | `BTreeSet::new` | [77](../../src/history.rs#L77) | external-constructor-callback-or-unresolved |
| `history_page` | `record                 .event                 .source_event_seqs                 .as_deref()                 .unwrap_or_default` | [79](../../src/history.rs#L79) | receiver-type-required |
| `history_page` | `record                 .event                 .source_event_seqs                 .as_deref` | [79](../../src/history.rs#L79) | receiver-type-required |
| `history_page` | `needed.insert` | [86](../../src/history.rs#L86) | receiver-type-required |
| `history_page` | `needed.first` | [90](../../src/history.rs#L90), [95](../../src/history.rs#L95) | receiver-type-required |
| `history_page` | `usize::try_from(*first).map_err` | [92](../../src/history.rs#L92) | receiver-type-required |
| `history_page` | `usize::try_from` | [92](../../src/history.rs#L92) | external-constructor-callback-or-unresolved |
| `history_page` | `HistoryError::SourceSequence` | [92](../../src/history.rs#L92), [94](../../src/history.rs#L94) | external-constructor-callback-or-unresolved |
| `history_page` | `needed.first().expect` | [95](../../src/history.rs#L95) | receiver-type-required |
| `history_page` | `lower.min` | [98](../../src/history.rs#L98) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]             .iter()             .map(&#124;record&#124; SessionHistoryEntry {                 event: record.event.clone(),                 view: None,             })             .collect` | [106](../../src/history.rs#L106) | receiver-type-required |
| `history_page` | `records[lower..eligible_end]             .iter()             .map` | [106](../../src/history.rs#L106) | receiver-type-required |
| `history_page` | `record.event.clone` | [109](../../src/history.rs#L109) | receiver-type-required |
| `record` | `event_type.to_owned` | [146](../../src/history.rs#L146) | receiver-type-required |
| `record` | `IJsonValue::parse_str("{}").expect` | [149](../../src/history.rs#L149) | receiver-type-required |
| `record` | `IJsonValue::parse_str` | [149](../../src/history.rs#L149) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `record` | `matches!(event_type, "user/message" &#124; "assistant/message")                     .then` | [152](../../src/history.rs#L152) | receiver-type-required |
| `record` | `SurfaceOperation::Append` | [153](../../src/history.rs#L153) | external-constructor-callback-or-unresolved |
| `record` | `"append".to_owned` | [153](../../src/history.rs#L153) | receiver-type-required |
| `page_is_message_aligned_and_keeps_projection_group` | `history_page(&records, None, Some(1)).expect` | [170](../../src/history.rs#L170) | receiver-type-required |
| `page_is_message_aligned_and_keeps_projection_group` | `history_page` | [170](../../src/history.rs#L170), [173](../../src/history.rs#L173) | external-constructor-callback-or-unresolved |
| `page_is_message_aligned_and_keeps_projection_group` | `Some` | [170](../../src/history.rs#L170), [173](../../src/history.rs#L173) | external-constructor-callback-or-unresolved |
| `page_is_message_aligned_and_keeps_projection_group` | `history_page(&records, Some(4), Some(1)).expect` | [173](../../src/history.rs#L173) | receiver-type-required |
