# thread-search::host

[Package atlas](index.md) · [Source](../../src/host.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [thread-search::host::SessionSearchHit](../../src/host.rs#L6) | struct_item | `pub` |  |
| [thread-search::host::SessionSearchResults](../../src/host.rs#L13) | struct_item | `pub` |  |
| [thread-search::host::ThreadSearchAuthority::search_sessions](../../src/host.rs#L21) | function_item | `pub` |  |
| [thread-search::host::matching_snippet](../../src/host.rs#L107) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–2: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["thread-search::host::matching_snippet"]
  n1["thread-search::host::ThreadSearchAuthority::search_sessions"]
  n1 --> n0
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `search_sessions` | `validate_request` | [22](../../src/host.rs#L22) | external-constructor-callback-or-unresolved |
| `search_sessions` | `query.into` | [24](../../src/host.rs#L24) | receiver-type-required |
| `search_sessions` | `normalize` | [29](../../src/host.rs#L29) | external-constructor-callback-or-unresolved |
| `search_sessions` | `NamedLock::exclusive` | [30](../../src/host.rs#L30) | external-constructor-callback-or-unresolved |
| `search_sessions` | `self.store.root().join` | [30](../../src/host.rs#L30), [31](../../src/host.rs#L31), [32](../../src/host.rs#L32) | receiver-type-required |
| `search_sessions` | `self.store.root` | [30](../../src/host.rs#L30), [31](../../src/host.rs#L31), [32](../../src/host.rs#L32), [33](../../src/host.rs#L33) | receiver-type-required |
| `search_sessions` | `NamedLock::shared` | [31](../../src/host.rs#L31), [32](../../src/host.rs#L32) | external-constructor-callback-or-unresolved |
| `search_sessions` | `scan_sources` | [33](../../src/host.rs#L33) | external-constructor-callback-or-unresolved |
| `search_sessions` | `Vec::new` | [34](../../src/host.rs#L34) | external-constructor-callback-or-unresolved |
| `search_sessions` | `source                 .title                 .as_deref()                 .and_then` | [36](../../src/host.rs#L36) | receiver-type-required |
| `search_sessions` | `source                 .title                 .as_deref` | [36](../../src/host.rs#L36) | receiver-type-required |
| `search_sessions` | `matching_snippet` | [39](../../src/host.rs#L39), [76](../../src/host.rs#L76) | [thread-search::host::matching_snippet](../../src/host.rs#L107) |
| `search_sessions` | `snippet.is_none` | [40](../../src/host.rs#L40) | receiver-type-required |
| `search_sessions` | `fs::read` | [41](../../src/host.rs#L41) | external-constructor-callback-or-unresolved |
| `search_sessions` | `source.folder.join` | [41](../../src/host.rs#L41) | receiver-type-required |
| `search_sessions` | `scan_valid_prefix(&bytes, schema::READER_VERSION)                     .projection                     .ok_or_else` | [42](../../src/host.rs#L42) | receiver-type-required |
| `search_sessions` | `scan_valid_prefix` | [42](../../src/host.rs#L42) | external-constructor-callback-or-unresolved |
| `search_sessions` | `SearchError::SourceCorrupt` | [45](../../src/host.rs#L45), [64](../../src/host.rs#L64), [66](../../src/host.rs#L66) | external-constructor-callback-or-unresolved |
| `search_sessions` | `"Conversation has no valid prefix".into` | [45](../../src/host.rs#L45) | receiver-type-required |
| `search_sessions` | `projection                     .events                     .into_iter()                     .filter(&#124;event&#124; event.seq() <= source.as_of_seq)                     .collect` | [48](../../src/host.rs#L48) | receiver-type-required |
| `search_sessions` | `projection                     .events                     .into_iter()                     .filter` | [48](../../src/host.rs#L48) | receiver-type-required |
| `search_sessions` | `projection                     .events                     .into_iter` | [48](../../src/host.rs#L48) | receiver-type-required |
| `search_sessions` | `event.seq` | [51](../../src/host.rs#L51), [55](../../src/host.rs#L55) | receiver-type-required |
| `search_sessions` | `superseded_sequences` | [53](../../src/host.rs#L53) | external-constructor-callback-or-unresolved |
| `search_sessions` | `events.iter().rev` | [54](../../src/host.rs#L54) | receiver-type-required |
| `search_sessions` | `events.iter` | [54](../../src/host.rs#L54) | receiver-type-required |
| `search_sessions` | `superseded.contains` | [55](../../src/host.rs#L55) | receiver-type-required |
| `search_sessions` | `serde_json::from_slice(                         &event                             .raw()                             .canonical_bytes()                             .map_err(&#124;error&#124; SearchError::SourceCorrupt(error.to_string()))?,                     )                     .map_err` | [60](../../src/host.rs#L60) | receiver-type-required |
| `search_sessions` | `serde_json::from_slice` | [60](../../src/host.rs#L60) | external-constructor-callback-or-unresolved |
| `search_sessions` | `event                             .raw()                             .canonical_bytes()                             .map_err` | [61](../../src/host.rs#L61) | receiver-type-required |
| `search_sessions` | `event                             .raw()                             .canonical_bytes` | [61](../../src/host.rs#L61) | receiver-type-required |
| `search_sessions` | `event                             .raw` | [61](../../src/host.rs#L61) | receiver-type-required |
| `search_sessions` | `error.to_string` | [64](../../src/host.rs#L64), [66](../../src/host.rs#L66) | receiver-type-required |
| `search_sessions` | `raw.get("content").and_then` | [67](../../src/host.rs#L67) | receiver-type-required |
| `search_sessions` | `raw.get` | [67](../../src/host.rs#L67) | receiver-type-required |
| `search_sessions` | `block.get("type").and_then` | [72](../../src/host.rs#L72) | receiver-type-required |
| `search_sessions` | `block.get` | [72](../../src/host.rs#L72), [75](../../src/host.rs#L75) | receiver-type-required |
| `search_sessions` | `Some` | [72](../../src/host.rs#L72) | external-constructor-callback-or-unresolved |
| `search_sessions` | `block.get("text").and_then` | [75](../../src/host.rs#L75) | receiver-type-required |
| `search_sessions` | `snippet.is_some` | [77](../../src/host.rs#L77), [82](../../src/host.rs#L82) | receiver-type-required |
| `search_sessions` | `items.len` | [88](../../src/host.rs#L88) | receiver-type-required |
| `search_sessions` | `Ok` | [89](../../src/host.rs#L89), [100](../../src/host.rs#L100) | external-constructor-callback-or-unresolved |
| `search_sessions` | `items.push` | [94](../../src/host.rs#L94) | receiver-type-required |
| `matching_snippet` | `normalize` | [110](../../src/host.rs#L110) | external-constructor-callback-or-unresolved |
| `matching_snippet` | `text.find` | [111](../../src/host.rs#L111) | receiver-type-required |
| `matching_snippet` | `text[..position].chars().count` | [112](../../src/host.rs#L112) | receiver-type-required |
| `matching_snippet` | `text[..position].chars` | [112](../../src/host.rs#L112) | receiver-type-required |
| `matching_snippet` | `before.saturating_sub` | [113](../../src/host.rs#L113) | receiver-type-required |
| `matching_snippet` | `query.chars().count().saturating_add` | [114](../../src/host.rs#L114) | receiver-type-required |
| `matching_snippet` | `query.chars().count` | [114](../../src/host.rs#L114) | receiver-type-required |
| `matching_snippet` | `query.chars` | [114](../../src/host.rs#L114) | receiver-type-required |
| `matching_snippet` | `text.chars().skip(start).take(count).collect` | [115](../../src/host.rs#L115) | receiver-type-required |
| `matching_snippet` | `text.chars().skip(start).take` | [115](../../src/host.rs#L115) | receiver-type-required |
| `matching_snippet` | `text.chars().skip` | [115](../../src/host.rs#L115) | receiver-type-required |
| `matching_snippet` | `text.chars` | [115](../../src/host.rs#L115), [119](../../src/host.rs#L119) | receiver-type-required |
| `matching_snippet` | `result.insert` | [117](../../src/host.rs#L117) | receiver-type-required |
| `matching_snippet` | `text.chars().count` | [119](../../src/host.rs#L119) | receiver-type-required |
| `matching_snippet` | `result.push` | [120](../../src/host.rs#L120) | receiver-type-required |
| `matching_snippet` | `Some` | [122](../../src/host.rs#L122) | external-constructor-callback-or-unresolved |
