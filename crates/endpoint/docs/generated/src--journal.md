# endpoint::journal

[Package atlas](index.md) · [Source](../../src/journal.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::journal::JOURNAL_RECORD_VERSION](../../src/journal.rs#L17) | const_item | `pub` |  |
| [endpoint::journal::JournalRecord](../../src/journal.rs#L21) | struct_item | `pub` |  |
| [endpoint::journal::JournalRecord::validate](../../src/journal.rs#L29) | function_item | `pub` |  |
| [endpoint::journal::JournalRecord::canonical_bytes](../../src/journal.rs#L52) | function_item | `pub` |  |
| [endpoint::journal::JOURNAL_FILE](../../src/journal.rs#L60) | const_item | `pub` |  |
| [endpoint::journal::LOCK_FILE](../../src/journal.rs#L61) | const_item | `pub` |  |
| [endpoint::journal::retire_outdated_journal](../../src/journal.rs#L73) | function_item | `private` |  |
| [endpoint::journal::retire_legacy_journal](../../src/journal.rs#L98) | function_item | `private` |  |
| [endpoint::journal::EndpointJournal](../../src/journal.rs#L121) | struct_item | `pub` |  |
| [endpoint::journal::JournalCache](../../src/journal.rs#L128) | struct_item | `private` |  |
| [endpoint::journal::JournalCache::new](../../src/journal.rs#L135) | function_item | `private` |  |
| [endpoint::journal::EndpointJournal::open](../../src/journal.rs#L154) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::path](../../src/journal.rs#L188) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::thread_folder](../../src/journal.rs#L193) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::records](../../src/journal.rs#L199) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::event](../../src/journal.rs#L212) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::last_seq](../../src/journal.rs#L230) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::last_settled_endpoint_seq](../../src/journal.rs#L242) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::kernel_anchor_for_endpoint_seq](../../src/journal.rs#L275) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::append_kernel](../../src/journal.rs#L294) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::append_kernel_batch](../../src/journal.rs#L307) | function_item | `pub` |  |
| [endpoint::journal::EndpointJournal::append_batch](../../src/journal.rs#L335) | function_item | `private` |  |
| [endpoint::journal::EndpointJournal::repair_tail](../../src/journal.rs#L390) | function_item | `private` |  |
| [endpoint::journal::EndpointJournal::refresh_cache_unlocked](../../src/journal.rs#L406) | function_item | `private` |  |
| [endpoint::journal::parse_records](../../src/journal.rs#L424) | function_item | `private` |  |
| [endpoint::journal::identity](../../src/journal.rs#L462) | function_item | `private` |  |
| [endpoint::journal::JournalError](../../src/journal.rs#L470) | enum_item | `pub` |  |
| [endpoint::journal::tests::event](../../src/journal.rs#L497) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::journal::tests::append_is_durable_and_idempotent](../../src/journal.rs#L510) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::journal::tests::an_outdated_record_version_retires_the_journal_on_open](../../src/journal.rs#L525) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::journal::tests::a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger](../../src/journal.rs#L544) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::journal::tests::partial_tail_is_truncated](../../src/journal.rs#L556) | function_item | `private` | test; #[cfg(test)] |
| [endpoint::journal::tests::fork_anchor_requires_the_last_projection_slot](../../src/journal.rs#L573) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `HashMap` | `std::collections::HashMap` | `private` |
| `HashSet` | `std::collections::HashSet` | `private` |
| `fs` | `std::fs` | `private` |
| `OpenOptions` | `std::fs::OpenOptions` | `private` |
| `Write` | `std::io::Write` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `DirectoryLock` | `store::DirectoryLock` | `private` |
| `FullSync` | `store::FullSync` | `private` |
| `NamedLock` | `store::NamedLock` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `EndpointTypeError` | `crate::types::EndpointTypeError` | `private` |
| `MAX_SAFE_SEQUENCE` | `crate::types::MAX_SAFE_SEQUENCE` | `private` |
| `SessionEvent` | `crate::types::SessionEvent` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `endpoint::journal::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 21 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::journal::JournalCache::new"]
  n1["endpoint::journal::EndpointJournal::open"]
  n2["endpoint::journal::EndpointJournal::path"]
  n3["endpoint::journal::EndpointJournal::thread_folder"]
  n4["endpoint::journal::EndpointJournal::records"]
  n5["endpoint::journal::EndpointJournal::event"]
  n6["endpoint::journal::EndpointJournal::last_seq"]
  n7["endpoint::journal::EndpointJournal::last_settled_endpoint_seq"]
  n8["endpoint::journal::EndpointJournal::kernel_anchor_for_endpoint_seq"]
  n9["endpoint::journal::EndpointJournal::append_kernel"]
  n10["endpoint::journal::JournalRecord::validate"]
  n11["endpoint::journal::EndpointJournal::append_kernel_batch"]
  n12["endpoint::journal::EndpointJournal::append_batch"]
  n13["endpoint::journal::EndpointJournal::repair_tail"]
  n14["endpoint::journal::EndpointJournal::refresh_cache_unlocked"]
  n15["endpoint::journal::parse_records"]
  n16["endpoint::journal::identity"]
  n17["endpoint::journal::JournalRecord::canonical_bytes"]
  n18["endpoint::journal::retire_outdated_journal"]
  n19["endpoint::journal::retire_legacy_journal"]
  n20["store::platform::NamedLock::exclusive"]
  n21["store::platform::FullSync::full_sync"]
  n22["store::platform::DirectoryLock::shared"]
  n23["store::platform::NamedLock::shared"]
  n1 --> n18
  n1 --> n19
  n1 --> n22
  n4 --> n14
  n4 --> n23
  n5 --> n14
  n5 --> n23
  n6 --> n4
  n7 --> n4
  n8 --> n4
  n9 --> n11
  n11 --> n12
  n12 --> n14
  n12 --> n16
  n12 --> n20
  n12 --> n21
  n13 --> n15
  n13 --> n20
  n13 --> n21
  n14 --> n0
  n14 --> n15
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `validate` | `Err` | [31](../../src/journal.rs#L31), [40](../../src/journal.rs#L40), [45](../../src/journal.rs#L45) | external-constructor-callback-or-unresolved |
| `validate` | `JournalError::Corruption` | [31](../../src/journal.rs#L31), [40](../../src/journal.rs#L40), [45](../../src/journal.rs#L45) | external-constructor-callback-or-unresolved |
| `validate` | `self.event.validate` | [36](../../src/journal.rs#L36) | receiver-type-required |
| `validate` | `self.kernel_seqs.iter().any` | [37](../../src/journal.rs#L37) | receiver-type-required |
| `validate` | `self.kernel_seqs.iter` | [37](../../src/journal.rs#L37) | receiver-type-required |
| `validate` | `self.kernel_seqs.windows(2).any` | [38](../../src/journal.rs#L38) | receiver-type-required |
| `validate` | `self.kernel_seqs.windows` | [38](../../src/journal.rs#L38) | receiver-type-required |
| `validate` | `"kernel_seqs must be sorted and unique".to_owned` | [41](../../src/journal.rs#L41) | receiver-type-required |
| `validate` | `self.slot.is_empty` | [44](../../src/journal.rs#L44) | receiver-type-required |
| `validate` | `self.kernel_seqs.is_empty` | [44](../../src/journal.rs#L44) | receiver-type-required |
| `validate` | `"record must name its kernel slot and causal seqs".to_owned` | [46](../../src/journal.rs#L46) | receiver-type-required |
| `validate` | `Ok` | [49](../../src/journal.rs#L49) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec(self)             .map_err` | [53](../../src/journal.rs#L53) | receiver-type-required |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec` | [53](../../src/journal.rs#L53) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `JournalError::Canonical` | [54](../../src/journal.rs#L54) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `error.to_string` | [54](../../src/journal.rs#L54) | receiver-type-required |
| `retire_outdated_journal` | `journal.exists` | [74](../../src/journal.rs#L74) | receiver-type-required |
| `retire_outdated_journal` | `Ok` | [75](../../src/journal.rs#L75), [82](../../src/journal.rs#L82), [88](../../src/journal.rs#L88), [95](../../src/journal.rs#L95) | external-constructor-callback-or-unresolved |
| `retire_outdated_journal` | `fs::read` | [77](../../src/journal.rs#L77) | external-constructor-callback-or-unresolved |
| `retire_outdated_journal` | `bytes         .split(&#124;byte&#124; *byte == b'\n')         .find` | [78](../../src/journal.rs#L78) | receiver-type-required |
| `retire_outdated_journal` | `bytes         .split` | [78](../../src/journal.rs#L78) | receiver-type-required |
| `retire_outdated_journal` | `line.is_empty` | [80](../../src/journal.rs#L80) | receiver-type-required |
| `retire_outdated_journal` | `serde_json::from_slice::<serde_json::Value>(first)         .ok()         .and_then` | [84](../../src/journal.rs#L84) | receiver-type-required |
| `retire_outdated_journal` | `serde_json::from_slice::<serde_json::Value>(first)         .ok` | [84](../../src/journal.rs#L84) | receiver-type-required |
| `retire_outdated_journal` | `serde_json::from_slice::<serde_json::Value>` | [84](../../src/journal.rs#L84) | external-constructor-callback-or-unresolved |
| `retire_outdated_journal` | `value.get("v").and_then` | [86](../../src/journal.rs#L86) | receiver-type-required |
| `retire_outdated_journal` | `value.get` | [86](../../src/journal.rs#L86) | receiver-type-required |
| `retire_outdated_journal` | `Some` | [87](../../src/journal.rs#L87) | external-constructor-callback-or-unresolved |
| `retire_outdated_journal` | `fs::remove_file` | [90](../../src/journal.rs#L90), [92](../../src/journal.rs#L92) | external-constructor-callback-or-unresolved |
| `retire_outdated_journal` | `lock.exists` | [91](../../src/journal.rs#L91) | receiver-type-required |
| `retire_outdated_journal` | `fs::File::open(folder)?.sync_all` | [94](../../src/journal.rs#L94) | receiver-type-required |
| `retire_outdated_journal` | `fs::File::open` | [94](../../src/journal.rs#L94) | external-constructor-callback-or-unresolved |
| `retire_legacy_journal` | `journal.exists` | [99](../../src/journal.rs#L99) | receiver-type-required |
| `retire_legacy_journal` | `Ok` | [100](../../src/journal.rs#L100), [118](../../src/journal.rs#L118) | external-constructor-callback-or-unresolved |
| `retire_legacy_journal` | `folder.join` | [109](../../src/journal.rs#L109) | receiver-type-required |
| `retire_legacy_journal` | `legacy.exists` | [110](../../src/journal.rs#L110) | receiver-type-required |
| `retire_legacy_journal` | `fs::remove_file` | [111](../../src/journal.rs#L111) | external-constructor-callback-or-unresolved |
| `retire_legacy_journal` | `fs::File::open(folder)?.sync_all` | [116](../../src/journal.rs#L116) | receiver-type-required |
| `retire_legacy_journal` | `fs::File::open` | [116](../../src/journal.rs#L116) | external-constructor-callback-or-unresolved |
| `new` | `HashMap::with_capacity` | [136](../../src/journal.rs#L136) | external-constructor-callback-or-unresolved |
| `new` | `records.len` | [136](../../src/journal.rs#L136) | receiver-type-required |
| `new` | `records.iter().enumerate` | [137](../../src/journal.rs#L137) | receiver-type-required |
| `new` | `records.iter` | [137](../../src/journal.rs#L137) | receiver-type-required |
| `new` | `identity` | [138](../../src/journal.rs#L138) | external-constructor-callback-or-unresolved |
| `new` | `identities.insert(identity, index).is_some` | [139](../../src/journal.rs#L139) | receiver-type-required |
| `new` | `identities.insert` | [139](../../src/journal.rs#L139) | receiver-type-required |
| `new` | `Err` | [140](../../src/journal.rs#L140) | external-constructor-callback-or-unresolved |
| `new` | `JournalError::Corruption` | [140](../../src/journal.rs#L140) | external-constructor-callback-or-unresolved |
| `new` | `"duplicate projection identity".to_owned` | [141](../../src/journal.rs#L141) | receiver-type-required |
| `new` | `Ok` | [145](../../src/journal.rs#L145) | external-constructor-callback-or-unresolved |
| `open` | `thread_folder.as_ref` | [155](../../src/journal.rs#L155) | receiver-type-required |
| `open` | `folder.is_dir` | [156](../../src/journal.rs#L156) | receiver-type-required |
| `open` | `Err` | [157](../../src/journal.rs#L157) | external-constructor-callback-or-unresolved |
| `open` | `JournalError::Corruption` | [157](../../src/journal.rs#L157) | external-constructor-callback-or-unresolved |
| `open` | `DirectoryLock::shared` | [162](../../src/journal.rs#L162) | [store::platform::DirectoryLock::shared](../../../store/src/platform.rs#L57) |
| `open` | `folder.join` | [163](../../src/journal.rs#L163), [164](../../src/journal.rs#L164) | receiver-type-required |
| `open` | `retire_legacy_journal` | [165](../../src/journal.rs#L165) | [endpoint::journal::retire_legacy_journal](../../src/journal.rs#L98) |
| `open` | `retire_outdated_journal` | [166](../../src/journal.rs#L166) | [endpoint::journal::retire_outdated_journal](../../src/journal.rs#L73) |
| `open` | `path.exists` | [167](../../src/journal.rs#L167) | receiver-type-required |
| `open` | `lock_path.exists` | [168](../../src/journal.rs#L168) | receiver-type-required |
| `open` | `OpenOptions::new().create(true).append(true).open` | [169](../../src/journal.rs#L169) | receiver-type-required |
| `open` | `OpenOptions::new().create(true).append` | [169](../../src/journal.rs#L169) | receiver-type-required |
| `open` | `OpenOptions::new().create` | [169](../../src/journal.rs#L169) | receiver-type-required |
| `open` | `OpenOptions::new` | [169](../../src/journal.rs#L169), [170](../../src/journal.rs#L170) | external-constructor-callback-or-unresolved |
| `open` | `OpenOptions::new()             .create(true)             .append(true)             .open` | [170](../../src/journal.rs#L170) | receiver-type-required |
| `open` | `OpenOptions::new()             .create(true)             .append` | [170](../../src/journal.rs#L170) | receiver-type-required |
| `open` | `OpenOptions::new()             .create` | [170](../../src/journal.rs#L170) | receiver-type-required |
| `open` | `std::fs::File::open(folder)?.sync_all` | [175](../../src/journal.rs#L175) | receiver-type-required |
| `open` | `std::fs::File::open` | [175](../../src/journal.rs#L175) | external-constructor-callback-or-unresolved |
| `open` | `Mutex::new` | [181](../../src/journal.rs#L181) | external-constructor-callback-or-unresolved |
| `open` | `journal.repair_tail` | [183](../../src/journal.rs#L183) | receiver-type-required |
| `open` | `Ok` | [184](../../src/journal.rs#L184) | external-constructor-callback-or-unresolved |
| `thread_folder` | `self.path             .parent()             .expect` | [194](../../src/journal.rs#L194) | receiver-type-required |
| `thread_folder` | `self.path             .parent` | [194](../../src/journal.rs#L194) | receiver-type-required |
| `records` | `NamedLock::shared` | [200](../../src/journal.rs#L200) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L98) |
| `records` | `self             .cache             .lock()             .unwrap_or_else` | [201](../../src/journal.rs#L201) | receiver-type-required |
| `records` | `self             .cache             .lock` | [201](../../src/journal.rs#L201) | receiver-type-required |
| `records` | `self.refresh_cache_unlocked` | [205](../../src/journal.rs#L205) | [endpoint::journal::EndpointJournal::refresh_cache_unlocked](../../src/journal.rs#L406) |
| `records` | `Ok` | [206](../../src/journal.rs#L206) | external-constructor-callback-or-unresolved |
| `records` | `cache.as_ref().expect("cache was refreshed").records.clone` | [206](../../src/journal.rs#L206) | receiver-type-required |
| `records` | `cache.as_ref().expect` | [206](../../src/journal.rs#L206) | receiver-type-required |
| `records` | `cache.as_ref` | [206](../../src/journal.rs#L206) | receiver-type-required |
| `event` | `NamedLock::shared` | [213](../../src/journal.rs#L213) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L98) |
| `event` | `self             .cache             .lock()             .unwrap_or_else` | [214](../../src/journal.rs#L214) | receiver-type-required |
| `event` | `self             .cache             .lock` | [214](../../src/journal.rs#L214) | receiver-type-required |
| `event` | `self.refresh_cache_unlocked` | [218](../../src/journal.rs#L218) | [endpoint::journal::EndpointJournal::refresh_cache_unlocked](../../src/journal.rs#L406) |
| `event` | `usize::try_from` | [219](../../src/journal.rs#L219) | external-constructor-callback-or-unresolved |
| `event` | `Ok` | [220](../../src/journal.rs#L220), [222](../../src/journal.rs#L222) | external-constructor-callback-or-unresolved |
| `event` | `cache             .as_ref()             .expect("cache was refreshed")             .records             .get(index)             .map` | [222](../../src/journal.rs#L222) | receiver-type-required |
| `event` | `cache             .as_ref()             .expect("cache was refreshed")             .records             .get` | [222](../../src/journal.rs#L222) | receiver-type-required |
| `event` | `cache             .as_ref()             .expect` | [222](../../src/journal.rs#L222) | receiver-type-required |
| `event` | `cache             .as_ref` | [222](../../src/journal.rs#L222) | receiver-type-required |
| `event` | `record.event.clone` | [227](../../src/journal.rs#L227) | receiver-type-required |
| `last_seq` | `Ok` | [231](../../src/journal.rs#L231) | external-constructor-callback-or-unresolved |
| `last_seq` | `self.records()?.last().map` | [231](../../src/journal.rs#L231) | receiver-type-required |
| `last_seq` | `self.records()?.last` | [231](../../src/journal.rs#L231) | receiver-type-required |
| `last_seq` | `self.records` | [231](../../src/journal.rs#L231) | [endpoint::journal::EndpointJournal::records](../../src/journal.rs#L199) |
| `last_settled_endpoint_seq` | `self.records` | [243](../../src/journal.rs#L243) | [endpoint::journal::EndpointJournal::records](../../src/journal.rs#L199) |
| `last_settled_endpoint_seq` | `records.last` | [244](../../src/journal.rs#L244) | receiver-type-required |
| `last_settled_endpoint_seq` | `Ok` | [245](../../src/journal.rs#L245), [259](../../src/journal.rs#L259), [262](../../src/journal.rs#L262), [272](../../src/journal.rs#L272) | external-constructor-callback-or-unresolved |
| `last_settled_endpoint_seq` | `records             .iter()             .rposition` | [247](../../src/journal.rs#L247), [250](../../src/journal.rs#L250) | receiver-type-required |
| `last_settled_endpoint_seq` | `records             .iter` | [247](../../src/journal.rs#L247), [250](../../src/journal.rs#L250) | receiver-type-required |
| `last_settled_endpoint_seq` | `Some` | [259](../../src/journal.rs#L259), [272](../../src/journal.rs#L272) | external-constructor-callback-or-unresolved |
| `last_settled_endpoint_seq` | `records[end].kernel_seqs.last().copied` | [264](../../src/journal.rs#L264) | receiver-type-required |
| `last_settled_endpoint_seq` | `records[end].kernel_seqs.last` | [264](../../src/journal.rs#L264) | receiver-type-required |
| `last_settled_endpoint_seq` | `records             .get(index + 1)             .is_some_and` | [266](../../src/journal.rs#L266) | receiver-type-required |
| `last_settled_endpoint_seq` | `records             .get` | [266](../../src/journal.rs#L266) | receiver-type-required |
| `last_settled_endpoint_seq` | `next.kernel_seqs.last().copied` | [268](../../src/journal.rs#L268) | receiver-type-required |
| `last_settled_endpoint_seq` | `next.kernel_seqs.last` | [268](../../src/journal.rs#L268) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `self.records` | [276](../../src/journal.rs#L276) | [endpoint::journal::EndpointJournal::records](../../src/journal.rs#L199) |
| `kernel_anchor_for_endpoint_seq` | `usize::try_from(endpoint_seq)             .map_err` | [277](../../src/journal.rs#L277) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `usize::try_from` | [277](../../src/journal.rs#L277) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `JournalError::EndpointSeqNotFound` | [278](../../src/journal.rs#L278), [281](../../src/journal.rs#L281) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `records             .get(index)             .ok_or` | [279](../../src/journal.rs#L279) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `records             .get` | [279](../../src/journal.rs#L279), [285](../../src/journal.rs#L285) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `record.kernel_seqs.last().copied().ok_or_else` | [282](../../src/journal.rs#L282) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `record.kernel_seqs.last().copied` | [282](../../src/journal.rs#L282) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `record.kernel_seqs.last` | [282](../../src/journal.rs#L282) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `JournalError::Corruption` | [283](../../src/journal.rs#L283) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `"kernel projection lacks causal anchor".to_owned` | [283](../../src/journal.rs#L283) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `records             .get(index + 1)             .is_some_and` | [285](../../src/journal.rs#L285) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `next.kernel_seqs.last().copied` | [287](../../src/journal.rs#L287) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `next.kernel_seqs.last` | [287](../../src/journal.rs#L287) | receiver-type-required |
| `kernel_anchor_for_endpoint_seq` | `Some` | [287](../../src/journal.rs#L287) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `Err` | [289](../../src/journal.rs#L289) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `JournalError::IncompleteProjectionGroup` | [289](../../src/journal.rs#L289) | external-constructor-callback-or-unresolved |
| `kernel_anchor_for_endpoint_seq` | `Ok` | [291](../../src/journal.rs#L291) | external-constructor-callback-or-unresolved |
| `append_kernel` | `self.append_kernel_batch(vec![(kernel_seqs, slot.into(), event)])             .map` | [300](../../src/journal.rs#L300) | receiver-type-required |
| `append_kernel` | `self.append_kernel_batch` | [300](../../src/journal.rs#L300) | [endpoint::journal::EndpointJournal::append_kernel_batch](../../src/journal.rs#L307) |
| `append_kernel` | `events.remove` | [301](../../src/journal.rs#L301) | receiver-type-required |
| `append_kernel_batch` | `projections.is_empty` | [311](../../src/journal.rs#L311) | receiver-type-required |
| `append_kernel_batch` | `Ok` | [312](../../src/journal.rs#L312) | external-constructor-callback-or-unresolved |
| `append_kernel_batch` | `Vec::new` | [312](../../src/journal.rs#L312) | external-constructor-callback-or-unresolved |
| `append_kernel_batch` | `projections             .iter()             .map` | [314](../../src/journal.rs#L314) | receiver-type-required |
| `append_kernel_batch` | `projections             .iter` | [314](../../src/journal.rs#L314) | receiver-type-required |
| `append_kernel_batch` | `kernel_seqs.last().copied` | [316](../../src/journal.rs#L316) | receiver-type-required |
| `append_kernel_batch` | `kernel_seqs.last` | [316](../../src/journal.rs#L316) | receiver-type-required |
| `append_kernel_batch` | `anchors.next().flatten` | [317](../../src/journal.rs#L317) | receiver-type-required |
| `append_kernel_batch` | `anchors.next` | [317](../../src/journal.rs#L317) | receiver-type-required |
| `append_kernel_batch` | `anchor.is_none` | [318](../../src/journal.rs#L318) | receiver-type-required |
| `append_kernel_batch` | `anchors.any` | [318](../../src/journal.rs#L318) | receiver-type-required |
| `append_kernel_batch` | `Err` | [319](../../src/journal.rs#L319) | external-constructor-callback-or-unresolved |
| `append_kernel_batch` | `JournalError::Corruption` | [319](../../src/journal.rs#L319) | external-constructor-callback-or-unresolved |
| `append_kernel_batch` | `"kernel projection batch must share one causal anchor".to_owned` | [320](../../src/journal.rs#L320) | receiver-type-required |
| `append_kernel_batch` | `projections             .into_iter()             .map(&#124;(kernel_seqs, slot, event)&#124; JournalRecord {                 v: JOURNAL_RECORD_VERSION,                 event,                 kernel_seqs,                 slot,             })             .collect` | [323](../../src/journal.rs#L323) | receiver-type-required |
| `append_kernel_batch` | `projections             .into_iter()             .map` | [323](../../src/journal.rs#L323) | receiver-type-required |
| `append_kernel_batch` | `projections             .into_iter` | [323](../../src/journal.rs#L323) | receiver-type-required |
| `append_kernel_batch` | `self.append_batch` | [332](../../src/journal.rs#L332) | [endpoint::journal::EndpointJournal::append_batch](../../src/journal.rs#L335) |
| `append_batch` | `candidates.is_empty` | [339](../../src/journal.rs#L339) | receiver-type-required |
| `append_batch` | `Ok` | [340](../../src/journal.rs#L340), [387](../../src/journal.rs#L387) | external-constructor-callback-or-unresolved |
| `append_batch` | `Vec::new` | [340](../../src/journal.rs#L340), [350](../../src/journal.rs#L350) | external-constructor-callback-or-unresolved |
| `append_batch` | `NamedLock::exclusive` | [342](../../src/journal.rs#L342) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L102) |
| `append_batch` | `self             .cache             .lock()             .unwrap_or_else` | [343](../../src/journal.rs#L343) | receiver-type-required |
| `append_batch` | `self             .cache             .lock` | [343](../../src/journal.rs#L343) | receiver-type-required |
| `append_batch` | `self.refresh_cache_unlocked` | [347](../../src/journal.rs#L347) | [endpoint::journal::EndpointJournal::refresh_cache_unlocked](../../src/journal.rs#L406) |
| `append_batch` | `cache_slot.as_mut().expect` | [348](../../src/journal.rs#L348) | receiver-type-required |
| `append_batch` | `cache_slot.as_mut` | [348](../../src/journal.rs#L348) | receiver-type-required |
| `append_batch` | `Vec::with_capacity` | [349](../../src/journal.rs#L349) | external-constructor-callback-or-unresolved |
| `append_batch` | `candidates.len` | [349](../../src/journal.rs#L349) | receiver-type-required |
| `append_batch` | `candidates.drain` | [352](../../src/journal.rs#L352) | receiver-type-required |
| `append_batch` | `identity` | [353](../../src/journal.rs#L353) | [endpoint::journal::identity](../../src/journal.rs#L462) |
| `append_batch` | `cache.identities.get(&candidate_identity).copied` | [354](../../src/journal.rs#L354) | receiver-type-required |
| `append_batch` | `cache.identities.get` | [354](../../src/journal.rs#L354) | receiver-type-required |
| `append_batch` | `Err` | [357](../../src/journal.rs#L357), [363](../../src/journal.rs#L363) | external-constructor-callback-or-unresolved |
| `append_batch` | `JournalError::Corruption` | [357](../../src/journal.rs#L357), [370](../../src/journal.rs#L370) | external-constructor-callback-or-unresolved |
| `append_batch` | `"projection batch has an existing slot after a missing slot".to_owned` | [358](../../src/journal.rs#L358) | receiver-type-required |
| `append_batch` | `candidate.canonical_bytes` | [362](../../src/journal.rs#L362), [372](../../src/journal.rs#L372) | receiver-type-required |
| `append_batch` | `existing.canonical_bytes` | [362](../../src/journal.rs#L362) | receiver-type-required |
| `append_batch` | `JournalError::IdentityConflict` | [363](../../src/journal.rs#L363) | external-constructor-callback-or-unresolved |
| `append_batch` | `results.push` | [365](../../src/journal.rs#L365), [375](../../src/journal.rs#L375) | receiver-type-required |
| `append_batch` | `existing.event.clone` | [365](../../src/journal.rs#L365) | receiver-type-required |
| `append_batch` | `u64::try_from(cache.records.len())                 .map_err` | [369](../../src/journal.rs#L369) | receiver-type-required |
| `append_batch` | `u64::try_from` | [369](../../src/journal.rs#L369) | external-constructor-callback-or-unresolved |
| `append_batch` | `cache.records.len` | [369](../../src/journal.rs#L369), [378](../../src/journal.rs#L378) | receiver-type-required |
| `append_batch` | `"journal length overflow".to_owned` | [370](../../src/journal.rs#L370) | receiver-type-required |
| `append_batch` | `candidate.validate` | [371](../../src/journal.rs#L371) | receiver-type-required |
| `append_batch` | `line.push` | [373](../../src/journal.rs#L373) | receiver-type-required |
| `append_batch` | `bytes.extend_from_slice` | [374](../../src/journal.rs#L374) | receiver-type-required |
| `append_batch` | `candidate.event.clone` | [375](../../src/journal.rs#L375) | receiver-type-required |
| `append_batch` | `cache                 .identities                 .insert` | [376](../../src/journal.rs#L376) | receiver-type-required |
| `append_batch` | `cache.records.push` | [379](../../src/journal.rs#L379) | receiver-type-required |
| `append_batch` | `bytes.is_empty` | [381](../../src/journal.rs#L381) | receiver-type-required |
| `append_batch` | `OpenOptions::new().append(true).open` | [382](../../src/journal.rs#L382) | receiver-type-required |
| `append_batch` | `OpenOptions::new().append` | [382](../../src/journal.rs#L382) | receiver-type-required |
| `append_batch` | `OpenOptions::new` | [382](../../src/journal.rs#L382) | external-constructor-callback-or-unresolved |
| `append_batch` | `file.write_all` | [383](../../src/journal.rs#L383) | receiver-type-required |
| `append_batch` | `FullSync::full_sync` | [384](../../src/journal.rs#L384) | [store::platform::FullSync::full_sync](../../../store/src/platform.rs#L30) |
| `append_batch` | `cache.file_len.saturating_add` | [385](../../src/journal.rs#L385) | receiver-type-required |
| `append_batch` | `bytes.len` | [385](../../src/journal.rs#L385) | receiver-type-required |
| `repair_tail` | `NamedLock::exclusive` | [391](../../src/journal.rs#L391) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L102) |
| `repair_tail` | `fs::read` | [392](../../src/journal.rs#L392) | external-constructor-callback-or-unresolved |
| `repair_tail` | `parse_records` | [393](../../src/journal.rs#L393) | [endpoint::journal::parse_records](../../src/journal.rs#L424) |
| `repair_tail` | `bytes.len` | [394](../../src/journal.rs#L394) | receiver-type-required |
| `repair_tail` | `OpenOptions::new().write(true).open` | [395](../../src/journal.rs#L395) | receiver-type-required |
| `repair_tail` | `OpenOptions::new().write` | [395](../../src/journal.rs#L395) | receiver-type-required |
| `repair_tail` | `OpenOptions::new` | [395](../../src/journal.rs#L395) | external-constructor-callback-or-unresolved |
| `repair_tail` | `file.set_len` | [396](../../src/journal.rs#L396) | receiver-type-required |
| `repair_tail` | `FullSync::full_sync` | [397](../../src/journal.rs#L397) | [store::platform::FullSync::full_sync](../../../store/src/platform.rs#L30) |
| `repair_tail` | `self             .cache             .lock()             .unwrap_or_else` | [399](../../src/journal.rs#L399) | receiver-type-required |
| `repair_tail` | `self             .cache             .lock` | [399](../../src/journal.rs#L399) | receiver-type-required |
| `repair_tail` | `Ok` | [403](../../src/journal.rs#L403) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `fs::metadata(&self.path)?.len` | [407](../../src/journal.rs#L407) | receiver-type-required |
| `refresh_cache_unlocked` | `fs::metadata` | [407](../../src/journal.rs#L407) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `cache             .as_ref()             .is_some_and` | [408](../../src/journal.rs#L408) | receiver-type-required |
| `refresh_cache_unlocked` | `cache             .as_ref` | [408](../../src/journal.rs#L408) | receiver-type-required |
| `refresh_cache_unlocked` | `Ok` | [412](../../src/journal.rs#L412), [420](../../src/journal.rs#L420) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `fs::read` | [414](../../src/journal.rs#L414) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `parse_records` | [415](../../src/journal.rs#L415) | [endpoint::journal::parse_records](../../src/journal.rs#L424) |
| `refresh_cache_unlocked` | `bytes.len` | [416](../../src/journal.rs#L416) | receiver-type-required |
| `refresh_cache_unlocked` | `Err` | [417](../../src/journal.rs#L417) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `JournalError::Corruption` | [417](../../src/journal.rs#L417) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `"partial journal tail".to_owned` | [417](../../src/journal.rs#L417) | receiver-type-required |
| `refresh_cache_unlocked` | `Some` | [419](../../src/journal.rs#L419) | external-constructor-callback-or-unresolved |
| `refresh_cache_unlocked` | `JournalCache::new` | [419](../../src/journal.rs#L419) | [endpoint::journal::JournalCache::new](../../src/journal.rs#L135) |
| `parse_records` | `Vec::new` | [428](../../src/journal.rs#L428) | external-constructor-callback-or-unresolved |
| `parse_records` | `HashSet::new` | [430](../../src/journal.rs#L430) | external-constructor-callback-or-unresolved |
| `parse_records` | `bytes.len` | [431](../../src/journal.rs#L431) | receiver-type-required |
| `parse_records` | `bytes[offset..].iter().position` | [432](../../src/journal.rs#L432) | receiver-type-required |
| `parse_records` | `bytes[offset..].iter` | [432](../../src/journal.rs#L432) | receiver-type-required |
| `parse_records` | `Ok` | [434](../../src/journal.rs#L434), [459](../../src/journal.rs#L459) | external-constructor-callback-or-unresolved |
| `parse_records` | `Err` | [436](../../src/journal.rs#L436), [440](../../src/journal.rs#L440), [445](../../src/journal.rs#L445), [452](../../src/journal.rs#L452) | external-constructor-callback-or-unresolved |
| `parse_records` | `JournalError::Corruption` | [436](../../src/journal.rs#L436), [440](../../src/journal.rs#L440), [445](../../src/journal.rs#L445), [452](../../src/journal.rs#L452) | external-constructor-callback-or-unresolved |
| `parse_records` | `"partial journal tail".to_owned` | [436](../../src/journal.rs#L436) | receiver-type-required |
| `parse_records` | `"blank journal line".to_owned` | [440](../../src/journal.rs#L440) | receiver-type-required |
| `parse_records` | `serde_json::from_slice` | [443](../../src/journal.rs#L443) | external-constructor-callback-or-unresolved |
| `parse_records` | `record.canonical_bytes` | [444](../../src/journal.rs#L444) | receiver-type-required |
| `parse_records` | `"journal line is not canonical JSON".to_owned` | [446](../../src/journal.rs#L446) | receiver-type-required |
| `parse_records` | `record.validate` | [449](../../src/journal.rs#L449) | receiver-type-required |
| `parse_records` | `records.len` | [449](../../src/journal.rs#L449) | receiver-type-required |
| `parse_records` | `identity` | [450](../../src/journal.rs#L450) | external-constructor-callback-or-unresolved |
| `parse_records` | `identities.insert` | [451](../../src/journal.rs#L451) | receiver-type-required |
| `parse_records` | `"duplicate projection identity".to_owned` | [453](../../src/journal.rs#L453) | receiver-type-required |
| `parse_records` | `records.push` | [456](../../src/journal.rs#L456) | receiver-type-required |
| `identity` | `record.kernel_seqs.last().ok_or_else` | [463](../../src/journal.rs#L463) | receiver-type-required |
| `identity` | `record.kernel_seqs.last` | [463](../../src/journal.rs#L463) | receiver-type-required |
| `identity` | `JournalError::Corruption` | [464](../../src/journal.rs#L464) | external-constructor-callback-or-unresolved |
| `identity` | `"kernel projection lacks causal anchor".to_owned` | [464](../../src/journal.rs#L464) | receiver-type-required |
| `identity` | `Ok` | [466](../../src/journal.rs#L466) | external-constructor-callback-or-unresolved |
| `event` | `event_type.to_owned` | [499](../../src/journal.rs#L499) | receiver-type-required |
| `event` | `IJsonValue::parse_str("{}").expect` | [502](../../src/journal.rs#L502) | receiver-type-required |
| `event` | `IJsonValue::parse_str` | [502](../../src/journal.rs#L502) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `append_is_durable_and_idempotent` | `tempfile::tempdir().expect` | [511](../../src/journal.rs#L511) | receiver-type-required |
| `append_is_durable_and_idempotent` | `tempfile::tempdir` | [511](../../src/journal.rs#L511) | external-constructor-callback-or-unresolved |
| `append_is_durable_and_idempotent` | `EndpointJournal::open(folder.path()).expect` | [512](../../src/journal.rs#L512) | receiver-type-required |
| `append_is_durable_and_idempotent` | `EndpointJournal::open` | [512](../../src/journal.rs#L512) | external-constructor-callback-or-unresolved |
| `append_is_durable_and_idempotent` | `folder.path` | [512](../../src/journal.rs#L512) | receiver-type-required |
| `append_is_durable_and_idempotent` | `journal             .append_kernel(vec![3], "turn-start", event("turn/start"))             .expect` | [513](../../src/journal.rs#L513), [517](../../src/journal.rs#L517) | receiver-type-required |
| `append_is_durable_and_idempotent` | `journal             .append_kernel` | [513](../../src/journal.rs#L513), [517](../../src/journal.rs#L517) | receiver-type-required |
| `append_is_durable_and_idempotent` | `event` | [514](../../src/journal.rs#L514), [518](../../src/journal.rs#L518) | [endpoint::journal::tests::event](../../src/journal.rs#L497) |
| `an_outdated_record_version_retires_the_journal_on_open` | `tempfile::tempdir().expect` | [526](../../src/journal.rs#L526) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `tempfile::tempdir` | [526](../../src/journal.rs#L526) | external-constructor-callback-or-unresolved |
| `an_outdated_record_version_retires_the_journal_on_open` | `fs::write(             folder.path().join(JOURNAL_FILE),             b"{\"event\":{\"data\":{},\"eventType\":\"turn/start\",\"seq\":0,\"time\":1.0},\"kernel_seqs\":[1],\"slot\":\"turn-start\",\"v\":2}\n",         )         .expect` | [527](../../src/journal.rs#L527) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `fs::write` | [527](../../src/journal.rs#L527), [532](../../src/journal.rs#L532) | external-constructor-callback-or-unresolved |
| `an_outdated_record_version_retires_the_journal_on_open` | `folder.path().join` | [528](../../src/journal.rs#L528), [532](../../src/journal.rs#L532) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `folder.path` | [528](../../src/journal.rs#L528), [532](../../src/journal.rs#L532), [533](../../src/journal.rs#L533), [539](../../src/journal.rs#L539) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `fs::write(folder.path().join(LOCK_FILE), b"").expect` | [532](../../src/journal.rs#L532) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `EndpointJournal::open(folder.path()).expect` | [533](../../src/journal.rs#L533), [539](../../src/journal.rs#L539) | receiver-type-required |
| `an_outdated_record_version_retires_the_journal_on_open` | `EndpointJournal::open` | [533](../../src/journal.rs#L533), [539](../../src/journal.rs#L539) | external-constructor-callback-or-unresolved |
| `an_outdated_record_version_retires_the_journal_on_open` | `drop` | [538](../../src/journal.rs#L538) | external-constructor-callback-or-unresolved |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `tempfile::tempdir().expect` | [545](../../src/journal.rs#L545) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `tempfile::tempdir` | [545](../../src/journal.rs#L545) | external-constructor-callback-or-unresolved |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `fs::write(folder.path().join("endpoint-v2.jsonl"), b"{\"v\":2}\n").expect` | [546](../../src/journal.rs#L546) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `fs::write` | [546](../../src/journal.rs#L546), [547](../../src/journal.rs#L547) | external-constructor-callback-or-unresolved |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `folder.path().join` | [546](../../src/journal.rs#L546), [547](../../src/journal.rs#L547) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `folder.path` | [546](../../src/journal.rs#L546), [547](../../src/journal.rs#L547), [548](../../src/journal.rs#L548) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `fs::write(folder.path().join("endpoint-v2.lock"), b"").expect` | [547](../../src/journal.rs#L547) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `EndpointJournal::open(folder.path()).expect` | [548](../../src/journal.rs#L548) | receiver-type-required |
| `a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger` | `EndpointJournal::open` | [548](../../src/journal.rs#L548) | external-constructor-callback-or-unresolved |
| `partial_tail_is_truncated` | `tempfile::tempdir().expect` | [557](../../src/journal.rs#L557) | receiver-type-required |
| `partial_tail_is_truncated` | `tempfile::tempdir` | [557](../../src/journal.rs#L557) | external-constructor-callback-or-unresolved |
| `partial_tail_is_truncated` | `EndpointJournal::open(folder.path()).expect` | [558](../../src/journal.rs#L558), [568](../../src/journal.rs#L568) | receiver-type-required |
| `partial_tail_is_truncated` | `EndpointJournal::open` | [558](../../src/journal.rs#L558), [568](../../src/journal.rs#L568) | external-constructor-callback-or-unresolved |
| `partial_tail_is_truncated` | `folder.path` | [558](../../src/journal.rs#L558), [568](../../src/journal.rs#L568) | receiver-type-required |
| `partial_tail_is_truncated` | `journal             .append_kernel(vec![3], "turn-start", event("turn/start"))             .expect` | [559](../../src/journal.rs#L559) | receiver-type-required |
| `partial_tail_is_truncated` | `journal             .append_kernel` | [559](../../src/journal.rs#L559) | receiver-type-required |
| `partial_tail_is_truncated` | `event` | [560](../../src/journal.rs#L560) | [endpoint::journal::tests::event](../../src/journal.rs#L497) |
| `partial_tail_is_truncated` | `OpenOptions::new()             .append(true)             .open(journal.path())             .expect` | [562](../../src/journal.rs#L562) | receiver-type-required |
| `partial_tail_is_truncated` | `OpenOptions::new()             .append(true)             .open` | [562](../../src/journal.rs#L562) | receiver-type-required |
| `partial_tail_is_truncated` | `OpenOptions::new()             .append` | [562](../../src/journal.rs#L562) | receiver-type-required |
| `partial_tail_is_truncated` | `OpenOptions::new` | [562](../../src/journal.rs#L562) | external-constructor-callback-or-unresolved |
| `partial_tail_is_truncated` | `journal.path` | [564](../../src/journal.rs#L564) | receiver-type-required |
| `partial_tail_is_truncated` | `file.write_all(b"{\"v\":2").expect` | [566](../../src/journal.rs#L566) | receiver-type-required |
| `partial_tail_is_truncated` | `file.write_all` | [566](../../src/journal.rs#L566) | receiver-type-required |
| `partial_tail_is_truncated` | `drop` | [567](../../src/journal.rs#L567) | external-constructor-callback-or-unresolved |
| `fork_anchor_requires_the_last_projection_slot` | `tempfile::tempdir().expect` | [574](../../src/journal.rs#L574) | receiver-type-required |
| `fork_anchor_requires_the_last_projection_slot` | `tempfile::tempdir` | [574](../../src/journal.rs#L574) | external-constructor-callback-or-unresolved |
| `fork_anchor_requires_the_last_projection_slot` | `EndpointJournal::open(folder.path()).expect` | [575](../../src/journal.rs#L575) | receiver-type-required |
| `fork_anchor_requires_the_last_projection_slot` | `EndpointJournal::open` | [575](../../src/journal.rs#L575) | external-constructor-callback-or-unresolved |
| `fork_anchor_requires_the_last_projection_slot` | `folder.path` | [575](../../src/journal.rs#L575) | receiver-type-required |
| `fork_anchor_requires_the_last_projection_slot` | `journal             .append_kernel_batch(vec![                 (vec![3, 4], "turn-start".to_owned(), event("turn/start")),                 (vec![3, 4], "user-message".to_owned(), event("user/message")),             ])             .expect` | [576](../../src/journal.rs#L576) | receiver-type-required |
| `fork_anchor_requires_the_last_projection_slot` | `journal             .append_kernel_batch` | [576](../../src/journal.rs#L576) | receiver-type-required |
