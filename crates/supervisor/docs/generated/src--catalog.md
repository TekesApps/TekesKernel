# tekes-supervisor::catalog

[Package atlas](index.md) · [Source](../../src/catalog.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::catalog::Trust](../../src/catalog.rs#L20) | enum_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::LineView](../../src/catalog.rs#L30) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::EntryView](../../src/catalog.rs#L42) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::EntryView::main](../../src/catalog.rs#L54) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::EntryView::stop_active](../../src/catalog.rs#L58) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Entry](../../src/catalog.rs#L64) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Entry::cold](../../src/catalog.rs#L73) | function_item | `private` |  |
| [tekes-supervisor::catalog::Entry::unread](../../src/catalog.rs#L81) | function_item | `private` |  |
| [tekes-supervisor::catalog::Load](../../src/catalog.rs#L91) | enum_item | `private` |  |
| [tekes-supervisor::catalog::Catalog](../../src/catalog.rs#L97) | struct_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::new](../../src/catalog.rs#L105) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::folder](../../src/catalog.rs#L114) | function_item | `private` |  |
| [tekes-supervisor::catalog::Catalog::read_entry](../../src/catalog.rs#L119) | function_item | `private` |  |
| [tekes-supervisor::catalog::Catalog::get](../../src/catalog.rs#L144) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::note_present](../../src/catalog.rs#L179) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::ensure_loaded](../../src/catalog.rs#L187) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::is_loaded](../../src/catalog.rs#L249) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::entries_now](../../src/catalog.rs#L258) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::set_fresh_main](../../src/catalog.rs#L272) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::refresh_light](../../src/catalog.rs#L310) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::set_children](../../src/catalog.rs#L362) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::mark_corrupt](../../src/catalog.rs#L381) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::forget](../../src/catalog.rs#L398) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::Catalog::unverified](../../src/catalog.rs#L406) | function_item | `pub(crate)` |  |
| [tekes-supervisor::catalog::view_of_card](../../src/catalog.rs#L424) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Condvar` | `std::sync::Condvar` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `RwLock` | `std::sync::RwLock` | `private` |
| `LifecycleFacts` | `schema::LifecycleFacts` | `private` |
| `CardSummary` | `store::card::CardSummary` | `private` |
| `Freshness` | `store::card::Freshness` | `private` |
| `Hold` | `store::card::Hold` | `private` |
| `JournalInput` | `store::card::JournalInput` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–19: 14 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::service::card_updated_ms"]
  n1["endpoint::types::validate_session_id"]
  n2["store::card::read_card"]
  n3["store::card::classify"]
  n4["tekes-supervisor::boot_trace::per_item_enabled"]
  n5["tekes-supervisor::catalog::Catalog::new"]
  n6["tekes-supervisor::catalog::Catalog::folder"]
  n7["tekes-supervisor::catalog::Catalog::read_entry"]
  n8["tekes-supervisor::catalog::Catalog::get"]
  n9["tekes-supervisor::catalog::Catalog::note_present"]
  n10["tekes-supervisor::catalog::Catalog::ensure_loaded"]
  n11["tekes-supervisor::catalog::Catalog::is_loaded"]
  n12["tekes-supervisor::catalog::Catalog::entries_now"]
  n13["tekes-supervisor::catalog::Catalog::set_fresh_main"]
  n14["tekes-supervisor::catalog::Catalog::refresh_light"]
  n15["tekes-supervisor::catalog::Catalog::set_children"]
  n16["tekes-supervisor::catalog::Catalog::mark_corrupt"]
  n17["tekes-supervisor::catalog::Catalog::forget"]
  n18["tekes-supervisor::catalog::Catalog::unverified"]
  n19["tekes-supervisor::catalog::view_of_card"]
  n20["tekes-supervisor::catalog::EntryView::main"]
  n21["tekes-supervisor::catalog::EntryView::stop_active"]
  n22["tekes-supervisor::catalog::Entry::cold"]
  n23["tekes-supervisor::catalog::Entry::unread"]
  n7 --> n2
  n7 --> n3
  n7 --> n6
  n7 --> n19
  n7 --> n22
  n8 --> n6
  n8 --> n7
  n9 --> n23
  n10 --> n1
  n10 --> n4
  n10 --> n8
  n14 --> n0
  n18 --> n6
  n18 --> n12
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `main` | `self.lines.first` | [55](../../src/catalog.rs#L55) | receiver-type-required |
| `stop_active` | `self.lines.iter().any` | [59](../../src/catalog.rs#L59) | receiver-type-required |
| `stop_active` | `self.lines.iter` | [59](../../src/catalog.rs#L59) | receiver-type-required |
| `new` | `root.to_path_buf` | [107](../../src/catalog.rs#L107) | receiver-type-required |
| `new` | `RwLock::new` | [108](../../src/catalog.rs#L108) | external-constructor-callback-or-unresolved |
| `new` | `BTreeMap::new` | [108](../../src/catalog.rs#L108) | external-constructor-callback-or-unresolved |
| `new` | `Mutex::new` | [109](../../src/catalog.rs#L109) | external-constructor-callback-or-unresolved |
| `new` | `Condvar::new` | [110](../../src/catalog.rs#L110) | external-constructor-callback-or-unresolved |
| `folder` | `self.root.join("threads").join` | [115](../../src/catalog.rs#L115) | receiver-type-required |
| `folder` | `self.root.join` | [115](../../src/catalog.rs#L115) | receiver-type-required |
| `read_entry` | `self.folder` | [120](../../src/catalog.rs#L120) | [tekes-supervisor::catalog::Catalog::folder](../../src/catalog.rs#L114) |
| `read_entry` | `folder.is_dir` | [121](../../src/catalog.rs#L121) | receiver-type-required |
| `read_entry` | `store::card::read_card` | [124](../../src/catalog.rs#L124) | [store::card::read_card](../../../store/src/card.rs#L176) |
| `read_entry` | `Some` | [125](../../src/catalog.rs#L125), [128](../../src/catalog.rs#L128), [133](../../src/catalog.rs#L133), [135](../../src/catalog.rs#L135), [137](../../src/catalog.rs#L137) | external-constructor-callback-or-unresolved |
| `read_entry` | `Entry::cold` | [125](../../src/catalog.rs#L125), [128](../../src/catalog.rs#L128), [133](../../src/catalog.rs#L133) | [tekes-supervisor::catalog::Entry::cold](../../src/catalog.rs#L73) |
| `read_entry` | `store::card::classify` | [127](../../src/catalog.rs#L127) | [store::card::classify](../../../store/src/card.rs#L391) |
| `read_entry` | `classified.freshness` | [130](../../src/catalog.rs#L130) | receiver-type-required |
| `read_entry` | `Arc::new` | [137](../../src/catalog.rs#L137) | external-constructor-callback-or-unresolved |
| `read_entry` | `view_of_card` | [137](../../src/catalog.rs#L137) | [tekes-supervisor::catalog::view_of_card](../../src/catalog.rs#L424) |
| `get` | `self             .entries             .read()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .get(id)             .filter` | [145](../../src/catalog.rs#L145) | receiver-type-required |
| `get` | `self             .entries             .read()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .get` | [145](../../src/catalog.rs#L145) | receiver-type-required |
| `get` | `self             .entries             .read()             .unwrap_or_else` | [145](../../src/catalog.rs#L145) | receiver-type-required |
| `get` | `self             .entries             .read` | [145](../../src/catalog.rs#L145) | receiver-type-required |
| `get` | `Some` | [152](../../src/catalog.rs#L152), [161](../../src/catalog.rs#L161), [171](../../src/catalog.rs#L171) | external-constructor-callback-or-unresolved |
| `get` | `entry.clone` | [152](../../src/catalog.rs#L152), [170](../../src/catalog.rs#L170) | receiver-type-required |
| `get` | `self.read_entry` | [154](../../src/catalog.rs#L154) | [tekes-supervisor::catalog::Catalog::read_entry](../../src/catalog.rs#L119) |
| `get` | `self             .entries             .write()             .unwrap_or_else` | [155](../../src/catalog.rs#L155) | receiver-type-required |
| `get` | `self             .entries             .write` | [155](../../src/catalog.rs#L155) | receiver-type-required |
| `get` | `entries.get` | [159](../../src/catalog.rs#L159) | receiver-type-required |
| `get` | `existing.clone` | [161](../../src/catalog.rs#L161) | receiver-type-required |
| `get` | `self.folder(id).is_dir` | [166](../../src/catalog.rs#L166) | receiver-type-required |
| `get` | `self.folder` | [166](../../src/catalog.rs#L166) | [tekes-supervisor::catalog::Catalog::folder](../../src/catalog.rs#L114) |
| `get` | `entries.remove` | [167](../../src/catalog.rs#L167) | receiver-type-required |
| `get` | `entries.insert` | [170](../../src/catalog.rs#L170) | receiver-type-required |
| `get` | `id.to_owned` | [170](../../src/catalog.rs#L170) | receiver-type-required |
| `note_present` | `self.entries             .write()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .insert` | [180](../../src/catalog.rs#L180) | receiver-type-required |
| `note_present` | `self.entries             .write()             .unwrap_or_else` | [180](../../src/catalog.rs#L180) | receiver-type-required |
| `note_present` | `self.entries             .write` | [180](../../src/catalog.rs#L180) | receiver-type-required |
| `note_present` | `id.to_owned` | [183](../../src/catalog.rs#L183) | receiver-type-required |
| `note_present` | `Entry::unread` | [183](../../src/catalog.rs#L183) | [tekes-supervisor::catalog::Entry::unread](../../src/catalog.rs#L81) |
| `ensure_loaded` | `self             .load             .lock()             .unwrap_or_else` | [188](../../src/catalog.rs#L188), [242](../../src/catalog.rs#L242) | receiver-type-required |
| `ensure_loaded` | `self             .load             .lock` | [188](../../src/catalog.rs#L188), [242](../../src/catalog.rs#L242) | receiver-type-required |
| `ensure_loaded` | `self                         .loaded                         .wait(state)                         .unwrap_or_else` | [196](../../src/catalog.rs#L196) | receiver-type-required |
| `ensure_loaded` | `self                         .loaded                         .wait` | [196](../../src/catalog.rs#L196) | receiver-type-required |
| `ensure_loaded` | `drop` | [207](../../src/catalog.rs#L207) | external-constructor-callback-or-unresolved |
| `ensure_loaded` | `std::time::Instant::now` | [208](../../src/catalog.rs#L208) | external-constructor-callback-or-unresolved |
| `ensure_loaded` | `Vec::new` | [209](../../src/catalog.rs#L209) | external-constructor-callback-or-unresolved |
| `ensure_loaded` | `std::fs::read_dir` | [210](../../src/catalog.rs#L210) | external-constructor-callback-or-unresolved |
| `ensure_loaded` | `self.root.join` | [210](../../src/catalog.rs#L210) | receiver-type-required |
| `ensure_loaded` | `entries.flatten` | [211](../../src/catalog.rs#L211) | receiver-type-required |
| `ensure_loaded` | `entry.file_name().to_string_lossy().into_owned` | [212](../../src/catalog.rs#L212) | receiver-type-required |
| `ensure_loaded` | `entry.file_name().to_string_lossy` | [212](../../src/catalog.rs#L212) | receiver-type-required |
| `ensure_loaded` | `entry.file_name` | [212](../../src/catalog.rs#L212) | receiver-type-required |
| `ensure_loaded` | `endpoint::validate_session_id(&name).is_ok` | [213](../../src/catalog.rs#L213) | receiver-type-required |
| `ensure_loaded` | `endpoint::validate_session_id` | [213](../../src/catalog.rs#L213) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `ensure_loaded` | `entry.file_type().is_ok_and` | [214](../../src/catalog.rs#L214) | receiver-type-required |
| `ensure_loaded` | `entry.file_type` | [214](../../src/catalog.rs#L214) | receiver-type-required |
| `ensure_loaded` | `kind.is_dir` | [214](../../src/catalog.rs#L214) | receiver-type-required |
| `ensure_loaded` | `ids.push` | [216](../../src/catalog.rs#L216) | receiver-type-required |
| `ensure_loaded` | `ids.sort` | [220](../../src/catalog.rs#L220) | receiver-type-required |
| `ensure_loaded` | `self.get` | [222](../../src/catalog.rs#L222) | [tekes-supervisor::catalog::Catalog::get](../../src/catalog.rs#L144) |
| `ensure_loaded` | `crate::boot_trace::per_item_enabled` | [224](../../src/catalog.rs#L224) | [tekes-supervisor::boot_trace::per_item_enabled](../../src/boot_trace.rs#L93) |
| `ensure_loaded` | `self                 .entries                 .read()                 .unwrap_or_else` | [225](../../src/catalog.rs#L225) | receiver-type-required |
| `ensure_loaded` | `self                 .entries                 .read` | [225](../../src/catalog.rs#L225) | receiver-type-required |
| `ensure_loaded` | `entries.values().filter(&#124;entry&#124; trust(&entry.trust)).count` | [230](../../src/catalog.rs#L230) | receiver-type-required |
| `ensure_loaded` | `entries.values().filter` | [230](../../src/catalog.rs#L230) | receiver-type-required |
| `ensure_loaded` | `entries.values` | [230](../../src/catalog.rs#L230) | receiver-type-required |
| `ensure_loaded` | `trust` | [230](../../src/catalog.rs#L230) | external-constructor-callback-or-unresolved |
| `ensure_loaded` | `self.loaded.notify_all` | [246](../../src/catalog.rs#L246) | receiver-type-required |
| `is_loaded` | `self             .load             .lock()             .unwrap_or_else` | [250](../../src/catalog.rs#L250) | receiver-type-required |
| `is_loaded` | `self             .load             .lock` | [250](../../src/catalog.rs#L250) | receiver-type-required |
| `entries_now` | `self.entries             .read()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .iter()             .map(&#124;(id, entry)&#124; (id.clone(), entry.clone()))             .collect` | [259](../../src/catalog.rs#L259) | receiver-type-required |
| `entries_now` | `self.entries             .read()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .iter()             .map` | [259](../../src/catalog.rs#L259) | receiver-type-required |
| `entries_now` | `self.entries             .read()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .iter` | [259](../../src/catalog.rs#L259) | receiver-type-required |
| `entries_now` | `self.entries             .read()             .unwrap_or_else` | [259](../../src/catalog.rs#L259) | receiver-type-required |
| `entries_now` | `self.entries             .read` | [259](../../src/catalog.rs#L259) | receiver-type-required |
| `entries_now` | `id.clone` | [263](../../src/catalog.rs#L263) | receiver-type-required |
| `entries_now` | `entry.clone` | [263](../../src/catalog.rs#L263) | receiver-type-required |
| `set_fresh_main` | `self             .entries             .write()             .unwrap_or_else` | [273](../../src/catalog.rs#L273) | receiver-type-required |
| `set_fresh_main` | `self             .entries             .write` | [273](../../src/catalog.rs#L273) | receiver-type-required |
| `set_fresh_main` | `entries.get` | [277](../../src/catalog.rs#L277) | receiver-type-required |
| `set_fresh_main` | `previous_view.main().map_or` | [282](../../src/catalog.rs#L282) | receiver-type-required |
| `set_fresh_main` | `previous_view.main` | [282](../../src/catalog.rs#L282) | receiver-type-required |
| `set_fresh_main` | `view.main().map_or` | [283](../../src/catalog.rs#L283) | receiver-type-required |
| `set_fresh_main` | `view.main` | [283](../../src/catalog.rs#L283) | receiver-type-required |
| `set_fresh_main` | `view.lines                     .extend` | [287](../../src/catalog.rs#L287) | receiver-type-required |
| `set_fresh_main` | `previous_view.lines.iter().skip(1).cloned` | [288](../../src/catalog.rs#L288) | receiver-type-required |
| `set_fresh_main` | `previous_view.lines.iter().skip` | [288](../../src/catalog.rs#L288) | receiver-type-required |
| `set_fresh_main` | `previous_view.lines.iter` | [288](../../src/catalog.rs#L288) | receiver-type-required |
| `set_fresh_main` | `entries.insert` | [293](../../src/catalog.rs#L293) | receiver-type-required |
| `set_fresh_main` | `id.to_owned` | [294](../../src/catalog.rs#L294) | receiver-type-required |
| `set_fresh_main` | `Some` | [297](../../src/catalog.rs#L297) | external-constructor-callback-or-unresolved |
| `set_fresh_main` | `Arc::new` | [297](../../src/catalog.rs#L297) | external-constructor-callback-or-unresolved |
| `refresh_light` | `self             .entries             .write()             .unwrap_or_else` | [311](../../src/catalog.rs#L311) | receiver-type-required |
| `refresh_light` | `self             .entries             .write` | [311](../../src/catalog.rs#L311) | receiver-type-required |
| `refresh_light` | `entries.get_mut` | [315](../../src/catalog.rs#L315) | receiver-type-required |
| `refresh_light` | `view.main` | [321](../../src/catalog.rs#L321) | receiver-type-required |
| `refresh_light` | `usize::try_from(main.last_seq).unwrap_or` | [328](../../src/catalog.rs#L328) | receiver-type-required |
| `refresh_light` | `usize::try_from` | [328](../../src/catalog.rs#L328) | external-constructor-callback-or-unresolved |
| `refresh_light` | `projection.events.get` | [329](../../src/catalog.rs#L329) | receiver-type-required |
| `refresh_light` | `new_events.iter().any` | [332](../../src/catalog.rs#L332) | receiver-type-required |
| `refresh_light` | `new_events.iter` | [332](../../src/catalog.rs#L332) | receiver-type-required |
| `refresh_light` | `event.string_field("title").is_some` | [333](../../src/catalog.rs#L333) | receiver-type-required |
| `refresh_light` | `event.string_field` | [333](../../src/catalog.rs#L333) | receiver-type-required |
| `refresh_light` | `event.kind` | [334](../../src/catalog.rs#L334) | receiver-type-required |
| `refresh_light` | `event.has_field` | [334](../../src/catalog.rs#L334) | receiver-type-required |
| `refresh_light` | `endpoint::card_updated_ms` | [345](../../src/catalog.rs#L345) | [endpoint::service::card_updated_ms](../../../endpoint/src/service.rs#L172) |
| `refresh_light` | `(**view).clone` | [348](../../src/catalog.rs#L348) | receiver-type-required |
| `refresh_light` | `projection.lifecycle.latest_turn.is_none` | [351](../../src/catalog.rs#L351) | receiver-type-required |
| `refresh_light` | `next.lines.first_mut` | [352](../../src/catalog.rs#L352) | receiver-type-required |
| `refresh_light` | `projection.lifecycle.clone` | [354](../../src/catalog.rs#L354) | receiver-type-required |
| `refresh_light` | `Some` | [356](../../src/catalog.rs#L356) | external-constructor-callback-or-unresolved |
| `refresh_light` | `Arc::new` | [356](../../src/catalog.rs#L356) | external-constructor-callback-or-unresolved |
| `set_children` | `self             .entries             .write()             .unwrap_or_else` | [363](../../src/catalog.rs#L363) | receiver-type-required |
| `set_children` | `self             .entries             .write` | [363](../../src/catalog.rs#L363) | receiver-type-required |
| `set_children` | `entries.get_mut` | [367](../../src/catalog.rs#L367) | receiver-type-required |
| `set_children` | `(**view).clone` | [373](../../src/catalog.rs#L373) | receiver-type-required |
| `set_children` | `next.lines.truncate` | [374](../../src/catalog.rs#L374) | receiver-type-required |
| `set_children` | `next.lines.extend` | [375](../../src/catalog.rs#L375) | receiver-type-required |
| `set_children` | `Some` | [377](../../src/catalog.rs#L377) | external-constructor-callback-or-unresolved |
| `set_children` | `Arc::new` | [377](../../src/catalog.rs#L377) | external-constructor-callback-or-unresolved |
| `mark_corrupt` | `self             .entries             .write()             .unwrap_or_else` | [382](../../src/catalog.rs#L382) | receiver-type-required |
| `mark_corrupt` | `self             .entries             .write` | [382](../../src/catalog.rs#L382) | receiver-type-required |
| `mark_corrupt` | `entries.get(id).and_then` | [386](../../src/catalog.rs#L386) | receiver-type-required |
| `mark_corrupt` | `entries.get` | [386](../../src/catalog.rs#L386) | receiver-type-required |
| `mark_corrupt` | `entry.view.clone` | [386](../../src/catalog.rs#L386) | receiver-type-required |
| `mark_corrupt` | `entries.insert` | [387](../../src/catalog.rs#L387) | receiver-type-required |
| `mark_corrupt` | `id.to_owned` | [388](../../src/catalog.rs#L388) | receiver-type-required |
| `mark_corrupt` | `Trust::Corrupt` | [390](../../src/catalog.rs#L390) | external-constructor-callback-or-unresolved |
| `mark_corrupt` | `reason.to_owned` | [390](../../src/catalog.rs#L390) | receiver-type-required |
| `forget` | `self.entries             .write()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .remove` | [399](../../src/catalog.rs#L399) | receiver-type-required |
| `forget` | `self.entries             .write()             .unwrap_or_else` | [399](../../src/catalog.rs#L399) | receiver-type-required |
| `forget` | `self.entries             .write` | [399](../../src/catalog.rs#L399) | receiver-type-required |
| `unverified` | `self             .entries_now()             .into_iter()             .filter(&#124;(_, entry)&#124; matches!(entry.trust, Trust::Cold &#124; Trust::Ahead))             .map(&#124;(id, _)&#124; id)             .collect::<Vec<_>>` | [407](../../src/catalog.rs#L407) | receiver-type-required |
| `unverified` | `self             .entries_now()             .into_iter()             .filter(&#124;(_, entry)&#124; matches!(entry.trust, Trust::Cold &#124; Trust::Ahead))             .map` | [407](../../src/catalog.rs#L407) | receiver-type-required |
| `unverified` | `self             .entries_now()             .into_iter()             .filter` | [407](../../src/catalog.rs#L407) | receiver-type-required |
| `unverified` | `self             .entries_now()             .into_iter` | [407](../../src/catalog.rs#L407) | receiver-type-required |
| `unverified` | `self             .entries_now` | [407](../../src/catalog.rs#L407) | [tekes-supervisor::catalog::Catalog::entries_now](../../src/catalog.rs#L258) |
| `unverified` | `ids.sort_by_key` | [413](../../src/catalog.rs#L413) | receiver-type-required |
| `unverified` | `std::cmp::Reverse` | [414](../../src/catalog.rs#L414) | external-constructor-callback-or-unresolved |
| `unverified` | `std::fs::metadata(self.folder(id).join("main.jsonl"))                     .and_then(&#124;metadata&#124; metadata.modified())                     .ok` | [415](../../src/catalog.rs#L415) | receiver-type-required |
| `unverified` | `std::fs::metadata(self.folder(id).join("main.jsonl"))                     .and_then` | [415](../../src/catalog.rs#L415) | receiver-type-required |
| `unverified` | `std::fs::metadata` | [415](../../src/catalog.rs#L415) | external-constructor-callback-or-unresolved |
| `unverified` | `self.folder(id).join` | [415](../../src/catalog.rs#L415) | receiver-type-required |
| `unverified` | `self.folder` | [415](../../src/catalog.rs#L415) | [tekes-supervisor::catalog::Catalog::folder](../../src/catalog.rs#L114) |
| `unverified` | `metadata.modified` | [416](../../src/catalog.rs#L416) | receiver-type-required |
| `view_of_card` | `card.summary.clone` | [427](../../src/catalog.rs#L427) | receiver-type-required |
| `view_of_card` | `card             .lines             .iter()             .map(&#124;line&#124; LineView {                 file: line.file.clone(),                 last_seq: line.last_seq,                 line: line.line.clone(),                 parent_file: line.parent_file.clone(),                 facts: line.facts.clone(),                 holds: line.holds.clone(),             })             .collect` | [428](../../src/catalog.rs#L428) | receiver-type-required |
| `view_of_card` | `card             .lines             .iter()             .map` | [428](../../src/catalog.rs#L428) | receiver-type-required |
| `view_of_card` | `card             .lines             .iter` | [428](../../src/catalog.rs#L428) | receiver-type-required |
| `view_of_card` | `line.file.clone` | [432](../../src/catalog.rs#L432) | receiver-type-required |
| `view_of_card` | `line.line.clone` | [434](../../src/catalog.rs#L434) | receiver-type-required |
| `view_of_card` | `line.parent_file.clone` | [435](../../src/catalog.rs#L435) | receiver-type-required |
| `view_of_card` | `line.facts.clone` | [436](../../src/catalog.rs#L436) | receiver-type-required |
| `view_of_card` | `line.holds.clone` | [437](../../src/catalog.rs#L437) | receiver-type-required |
| `view_of_card` | `card.journal.as_ref().map` | [440](../../src/catalog.rs#L440) | receiver-type-required |
| `view_of_card` | `card.journal.as_ref` | [440](../../src/catalog.rs#L440) | receiver-type-required |
