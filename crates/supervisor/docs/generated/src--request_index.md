# tekes-supervisor::request_index

[Package atlas](index.md) · [Source](../../src/request_index.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::request_index::NEGATIVE_CAPACITY](../../src/request_index.rs#L18) | const_item | `private` |  |
| [tekes-supervisor::request_index::NEGATIVE_TTL](../../src/request_index.rs#L19) | const_item | `private` |  |
| [tekes-supervisor::request_index::HOLD_PATTERN](../../src/request_index.rs#L22) | const_item | `private` |  |
| [tekes-supervisor::request_index::IndexedRequest](../../src/request_index.rs#L25) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::State](../../src/request_index.rs#L32) | struct_item | `private` |  |
| [tekes-supervisor::request_index::RequestIndex](../../src/request_index.rs#L43) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) | function_item | `private` |  |
| [tekes-supervisor::request_index::RequestIndex::insert](../../src/request_index.rs#L54) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::lookup](../../src/request_index.rs#L79) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::remove](../../src/request_index.rs#L84) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::forget_session](../../src/request_index.rs#L95) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::is_known_missing](../../src/request_index.rs#L104) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::note_missing](../../src/request_index.rs#L119) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::examined](../../src/request_index.rs#L135) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::note_examined](../../src/request_index.rs#L139) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::RequestIndex::len](../../src/request_index.rs#L144) | function_item | `pub(crate)` | test; #[cfg(test)] |
| [tekes-supervisor::request_index::ledger_signature](../../src/request_index.rs#L152) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::children_may_hold_requests](../../src/request_index.rs#L174) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::may_hold_requests](../../src/request_index.rs#L182) | function_item | `pub(crate)` |  |
| [tekes-supervisor::request_index::scan_for_requests](../../src/request_index.rs#L186) | function_item | `private` |  |
| [tekes-supervisor::request_index::tests::insert_lookup_forget_and_negative_cache](../../src/request_index.rs#L223) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::request_index::tests::inserting_a_request_clears_its_negative_entry](../../src/request_index.rs#L245) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::request_index::tests::negative_cache_is_bounded](../../src/request_index.rs#L256) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::request_index::tests::prefilter_has_no_false_negative_and_ignores_the_torn_suffix](../../src/request_index.rs#L266) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::request_index::tests::ledger_signature_changes_when_a_file_grows_and_ignores_the_journal](../../src/request_index.rs#L303) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `HashMap` | `std::collections::HashMap` | `private` |
| `HashSet` | `std::collections::HashSet` | `private` |
| `VecDeque` | `std::collections::VecDeque` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `Instant` | `std::time::Instant` | `private` |
| `RequestFrameType` | `endpoint::RequestFrameType` | `private` |
| `derive_line_request_rpc_id` | `endpoint::derive_line_request_rpc_id` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::request_index::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–13: 11 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::requests::derive_line_request_rpc_id"]
  n1["tekes-supervisor::request_index::RequestIndex::is_known_missing"]
  n2["tekes-supervisor::request_index::RequestIndex::note_missing"]
  n3["tekes-supervisor::request_index::RequestIndex::examined"]
  n4["tekes-supervisor::request_index::RequestIndex::note_examined"]
  n5["tekes-supervisor::request_index::ledger_signature"]
  n6["tekes-supervisor::request_index::children_may_hold_requests"]
  n7["tekes-supervisor::request_index::may_hold_requests"]
  n8["tekes-supervisor::request_index::scan_for_requests"]
  n9["tekes-supervisor::request_index::RequestIndex::state"]
  n10["tekes-supervisor::request_index::RequestIndex::insert"]
  n11["tekes-supervisor::request_index::RequestIndex::lookup"]
  n12["tekes-supervisor::request_index::RequestIndex::remove"]
  n13["tekes-supervisor::request_index::RequestIndex::forget_session"]
  n1 --> n9
  n2 --> n9
  n3 --> n9
  n4 --> n9
  n6 --> n8
  n7 --> n8
  n10 --> n0
  n10 --> n9
  n11 --> n9
  n12 --> n9
  n13 --> n9
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `NEGATIVE_TTL` | `Duration::from_secs` | [19](../../src/request_index.rs#L19) | external-constructor-callback-or-unresolved |
| `state` | `self.state             .lock()             .unwrap_or_else` | [49](../../src/request_index.rs#L49) | receiver-type-required |
| `state` | `self.state             .lock` | [49](../../src/request_index.rs#L49) | receiver-type-required |
| `insert` | `derive_line_request_rpc_id` | [62](../../src/request_index.rs#L62) | [endpoint::requests::derive_line_request_rpc_id](../../../endpoint/src/requests.rs#L128) |
| `insert` | `self.state` | [63](../../src/request_index.rs#L63) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `insert` | `state.negative_ids.remove` | [64](../../src/request_index.rs#L64) | receiver-type-required |
| `insert` | `session.to_owned` | [66](../../src/request_index.rs#L66), [73](../../src/request_index.rs#L73) | receiver-type-required |
| `insert` | `line.map` | [67](../../src/request_index.rs#L67) | receiver-type-required |
| `insert` | `state.by_rpc.insert(rpc_id.clone(), entry).is_none` | [70](../../src/request_index.rs#L70) | receiver-type-required |
| `insert` | `state.by_rpc.insert` | [70](../../src/request_index.rs#L70) | receiver-type-required |
| `insert` | `rpc_id.clone` | [70](../../src/request_index.rs#L70) | receiver-type-required |
| `insert` | `state                 .by_session                 .entry(session.to_owned())                 .or_default()                 .push` | [71](../../src/request_index.rs#L71) | receiver-type-required |
| `insert` | `state                 .by_session                 .entry(session.to_owned())                 .or_default` | [71](../../src/request_index.rs#L71) | receiver-type-required |
| `insert` | `state                 .by_session                 .entry` | [71](../../src/request_index.rs#L71) | receiver-type-required |
| `lookup` | `self.state().by_rpc.get(rpc_id).cloned` | [80](../../src/request_index.rs#L80) | receiver-type-required |
| `lookup` | `self.state().by_rpc.get` | [80](../../src/request_index.rs#L80) | receiver-type-required |
| `lookup` | `self.state` | [80](../../src/request_index.rs#L80) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `remove` | `self.state` | [85](../../src/request_index.rs#L85) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `remove` | `state.by_rpc.remove` | [86](../../src/request_index.rs#L86) | receiver-type-required |
| `remove` | `state.by_session.get_mut` | [87](../../src/request_index.rs#L87) | receiver-type-required |
| `remove` | `ids.retain` | [88](../../src/request_index.rs#L88) | receiver-type-required |
| `forget_session` | `self.state` | [96](../../src/request_index.rs#L96) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `forget_session` | `state.by_session.remove(session).unwrap_or_default` | [97](../../src/request_index.rs#L97) | receiver-type-required |
| `forget_session` | `state.by_session.remove` | [97](../../src/request_index.rs#L97) | receiver-type-required |
| `forget_session` | `state.by_rpc.remove` | [98](../../src/request_index.rs#L98) | receiver-type-required |
| `forget_session` | `state.examined.remove` | [100](../../src/request_index.rs#L100), [101](../../src/request_index.rs#L101) | receiver-type-required |
| `is_known_missing` | `self.state` | [105](../../src/request_index.rs#L105) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `is_known_missing` | `Instant::now` | [106](../../src/request_index.rs#L106) | external-constructor-callback-or-unresolved |
| `is_known_missing` | `state             .negative             .front()             .is_some_and` | [107](../../src/request_index.rs#L107) | receiver-type-required |
| `is_known_missing` | `state             .negative             .front` | [107](../../src/request_index.rs#L107) | receiver-type-required |
| `is_known_missing` | `now.duration_since` | [110](../../src/request_index.rs#L110) | receiver-type-required |
| `is_known_missing` | `state.negative.pop_front` | [112](../../src/request_index.rs#L112) | receiver-type-required |
| `is_known_missing` | `state.negative_ids.remove` | [113](../../src/request_index.rs#L113) | receiver-type-required |
| `is_known_missing` | `state.negative_ids.contains` | [116](../../src/request_index.rs#L116) | receiver-type-required |
| `note_missing` | `self.state` | [120](../../src/request_index.rs#L120) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `note_missing` | `state.negative_ids.insert` | [121](../../src/request_index.rs#L121) | receiver-type-required |
| `note_missing` | `rpc_id.to_owned` | [121](../../src/request_index.rs#L121), [126](../../src/request_index.rs#L126) | receiver-type-required |
| `note_missing` | `state             .negative             .push_back` | [124](../../src/request_index.rs#L124) | receiver-type-required |
| `note_missing` | `Instant::now` | [126](../../src/request_index.rs#L126) | external-constructor-callback-or-unresolved |
| `note_missing` | `state.negative.len` | [127](../../src/request_index.rs#L127) | receiver-type-required |
| `note_missing` | `state.negative.pop_front` | [128](../../src/request_index.rs#L128) | receiver-type-required |
| `note_missing` | `state.negative_ids.remove` | [129](../../src/request_index.rs#L129) | receiver-type-required |
| `examined` | `self.state().examined.get` | [136](../../src/request_index.rs#L136) | receiver-type-required |
| `examined` | `self.state` | [136](../../src/request_index.rs#L136) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `examined` | `Some` | [136](../../src/request_index.rs#L136) | external-constructor-callback-or-unresolved |
| `note_examined` | `self.state().examined.insert` | [140](../../src/request_index.rs#L140) | receiver-type-required |
| `note_examined` | `self.state` | [140](../../src/request_index.rs#L140) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `note_examined` | `session.to_owned` | [140](../../src/request_index.rs#L140) | receiver-type-required |
| `len` | `self.state().by_rpc.len` | [145](../../src/request_index.rs#L145) | receiver-type-required |
| `len` | `self.state` | [145](../../src/request_index.rs#L145) | [tekes-supervisor::request_index::RequestIndex::state](../../src/request_index.rs#L48) |
| `ledger_signature` | `fs::read_dir(folder).ok()?.flatten` | [154](../../src/request_index.rs#L154) | receiver-type-required |
| `ledger_signature` | `fs::read_dir(folder).ok` | [154](../../src/request_index.rs#L154) | receiver-type-required |
| `ledger_signature` | `fs::read_dir` | [154](../../src/request_index.rs#L154) | external-constructor-callback-or-unresolved |
| `ledger_signature` | `entry.file_name` | [155](../../src/request_index.rs#L155) | receiver-type-required |
| `ledger_signature` | `name.to_string_lossy` | [156](../../src/request_index.rs#L156) | receiver-type-required |
| `ledger_signature` | `name.ends_with` | [157](../../src/request_index.rs#L157) | receiver-type-required |
| `ledger_signature` | `entry.metadata().ok()?.len` | [160](../../src/request_index.rs#L160) | receiver-type-required |
| `ledger_signature` | `entry.metadata().ok` | [160](../../src/request_index.rs#L160) | receiver-type-required |
| `ledger_signature` | `entry.metadata` | [160](../../src/request_index.rs#L160) | receiver-type-required |
| `ledger_signature` | `name.bytes().chain` | [163](../../src/request_index.rs#L163) | receiver-type-required |
| `ledger_signature` | `name.bytes` | [163](../../src/request_index.rs#L163) | receiver-type-required |
| `ledger_signature` | `length.to_le_bytes` | [163](../../src/request_index.rs#L163) | receiver-type-required |
| `ledger_signature` | `u64::from` | [164](../../src/request_index.rs#L164) | external-constructor-callback-or-unresolved |
| `ledger_signature` | `hash.wrapping_mul` | [165](../../src/request_index.rs#L165) | receiver-type-required |
| `ledger_signature` | `signature.wrapping_add` | [167](../../src/request_index.rs#L167) | receiver-type-required |
| `ledger_signature` | `Some` | [169](../../src/request_index.rs#L169) | external-constructor-callback-or-unresolved |
| `children_may_hold_requests` | `scan_for_requests` | [175](../../src/request_index.rs#L175) | [tekes-supervisor::request_index::scan_for_requests](../../src/request_index.rs#L186) |
| `may_hold_requests` | `scan_for_requests` | [183](../../src/request_index.rs#L183) | [tekes-supervisor::request_index::scan_for_requests](../../src/request_index.rs#L186) |
| `scan_for_requests` | `fs::read_dir` | [187](../../src/request_index.rs#L187) | external-constructor-callback-or-unresolved |
| `scan_for_requests` | `entries.flatten` | [190](../../src/request_index.rs#L190) | receiver-type-required |
| `scan_for_requests` | `entry.file_name` | [191](../../src/request_index.rs#L191) | receiver-type-required |
| `scan_for_requests` | `name.to_string_lossy` | [192](../../src/request_index.rs#L192) | receiver-type-required |
| `scan_for_requests` | `name.ends_with` | [193](../../src/request_index.rs#L193) | receiver-type-required |
| `scan_for_requests` | `fs::read` | [199](../../src/request_index.rs#L199) | external-constructor-callback-or-unresolved |
| `scan_for_requests` | `entry.path` | [199](../../src/request_index.rs#L199) | receiver-type-required |
| `scan_for_requests` | `bytes                     .iter()                     .rposition(&#124;byte&#124; *byte == b'\n')                     .map_or` | [201](../../src/request_index.rs#L201) | receiver-type-required |
| `scan_for_requests` | `bytes                     .iter()                     .rposition` | [201](../../src/request_index.rs#L201) | receiver-type-required |
| `scan_for_requests` | `bytes                     .iter` | [201](../../src/request_index.rs#L201) | receiver-type-required |
| `scan_for_requests` | `valid                     .windows(HOLD_PATTERN.len())                     .any` | [205](../../src/request_index.rs#L205) | receiver-type-required |
| `scan_for_requests` | `valid                     .windows` | [205](../../src/request_index.rs#L205) | receiver-type-required |
| `scan_for_requests` | `HOLD_PATTERN.len` | [206](../../src/request_index.rs#L206) | receiver-type-required |
| `insert_lookup_forget_and_negative_cache` | `RequestIndex::default` | [224](../../src/request_index.rs#L224) | external-constructor-callback-or-unresolved |
| `insert_lookup_forget_and_negative_cache` | `index.insert` | [226](../../src/request_index.rs#L226), [227](../../src/request_index.rs#L227) | receiver-type-required |
| `insert_lookup_forget_and_negative_cache` | `Some` | [229](../../src/request_index.rs#L229) | external-constructor-callback-or-unresolved |
| `insert_lookup_forget_and_negative_cache` | `derive_line_request_rpc_id` | [234](../../src/request_index.rs#L234) | external-constructor-callback-or-unresolved |
| `insert_lookup_forget_and_negative_cache` | `index.note_missing` | [237](../../src/request_index.rs#L237) | receiver-type-required |
| `insert_lookup_forget_and_negative_cache` | `index.forget_session` | [239](../../src/request_index.rs#L239) | receiver-type-required |
| `inserting_a_request_clears_its_negative_entry` | `RequestIndex::default` | [246](../../src/request_index.rs#L246) | external-constructor-callback-or-unresolved |
| `inserting_a_request_clears_its_negative_entry` | `derive_line_request_rpc_id` | [248](../../src/request_index.rs#L248) | external-constructor-callback-or-unresolved |
| `inserting_a_request_clears_its_negative_entry` | `index.note_missing` | [249](../../src/request_index.rs#L249) | receiver-type-required |
| `inserting_a_request_clears_its_negative_entry` | `index.insert` | [251](../../src/request_index.rs#L251) | receiver-type-required |
| `negative_cache_is_bounded` | `RequestIndex::default` | [257](../../src/request_index.rs#L257) | external-constructor-callback-or-unresolved |
| `negative_cache_is_bounded` | `index.note_missing` | [259](../../src/request_index.rs#L259) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `tempfile::tempdir().expect` | [267](../../src/request_index.rs#L267) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `tempfile::tempdir` | [267](../../src/request_index.rs#L267) | external-constructor-callback-or-unresolved |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::write(folder.path().join("main.jsonl"), "{\"kind\":\"input\"}\n").expect` | [269](../../src/request_index.rs#L269) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::write` | [269](../../src/request_index.rs#L269), [272](../../src/request_index.rs#L272), [287](../../src/request_index.rs#L287), [294](../../src/request_index.rs#L294) | external-constructor-callback-or-unresolved |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `folder.path().join` | [269](../../src/request_index.rs#L269), [288](../../src/request_index.rs#L288), [295](../../src/request_index.rs#L295) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `folder.path` | [269](../../src/request_index.rs#L269), [288](../../src/request_index.rs#L288), [295](../../src/request_index.rs#L295) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::write(             folder                 .path()                 .join("018f0000-0000-7000-8000-000000000002.jsonl"),             format!("{{\"kind\":\"input\"}}\n{pattern}\n"),         )         .expect` | [272](../../src/request_index.rs#L272) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `folder                 .path()                 .join` | [273](../../src/request_index.rs#L273), [282](../../src/request_index.rs#L282) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `folder                 .path` | [273](../../src/request_index.rs#L273), [282](../../src/request_index.rs#L282) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::remove_file(             folder                 .path()                 .join("018f0000-0000-7000-8000-000000000002.jsonl"),         )         .expect` | [281](../../src/request_index.rs#L281) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::remove_file` | [281](../../src/request_index.rs#L281) | external-constructor-callback-or-unresolved |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::write(             folder.path().join("main.jsonl"),             format!("{{\"kind\":\"input\"}}\n{pattern}"),         )         .expect` | [287](../../src/request_index.rs#L287) | receiver-type-required |
| `prefilter_has_no_false_negative_and_ignores_the_torn_suffix` | `fs::write(             folder.path().join(endpoint::JOURNAL_FILE),             format!("{pattern}\n"),         )         .expect` | [294](../../src/request_index.rs#L294) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `tempfile::tempdir().expect` | [304](../../src/request_index.rs#L304) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `tempfile::tempdir` | [304](../../src/request_index.rs#L304) | external-constructor-callback-or-unresolved |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `fs::write(folder.path().join("main.jsonl"), "a\n").expect` | [305](../../src/request_index.rs#L305) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `fs::write` | [305](../../src/request_index.rs#L305), [307](../../src/request_index.rs#L307), [309](../../src/request_index.rs#L309) | external-constructor-callback-or-unresolved |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `folder.path().join` | [305](../../src/request_index.rs#L305), [307](../../src/request_index.rs#L307), [309](../../src/request_index.rs#L309) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `folder.path` | [305](../../src/request_index.rs#L305), [306](../../src/request_index.rs#L306), [307](../../src/request_index.rs#L307), [309](../../src/request_index.rs#L309) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `ledger_signature(folder.path()).expect` | [306](../../src/request_index.rs#L306) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `ledger_signature` | [306](../../src/request_index.rs#L306) | external-constructor-callback-or-unresolved |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `fs::write(folder.path().join(endpoint::JOURNAL_FILE), "journal\n").expect` | [307](../../src/request_index.rs#L307) | receiver-type-required |
| `ledger_signature_changes_when_a_file_grows_and_ignores_the_journal` | `fs::write(folder.path().join("main.jsonl"), "a\nb\n").expect` | [309](../../src/request_index.rs#L309) | receiver-type-required |
