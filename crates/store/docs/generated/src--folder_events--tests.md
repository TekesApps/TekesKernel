# store::folder_events::tests

[Package atlas](index.md) · [Source](../../src/folder_events/tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [store::folder_events::tests::A](../../src/folder_events/tests.rs#L7) | const_item | `private` | test;  |
| [store::folder_events::tests::B](../../src/folder_events/tests.rs#L8) | const_item | `private` | test;  |
| [store::folder_events::tests::C](../../src/folder_events/tests.rs#L9) | const_item | `private` | test;  |
| [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) | function_item | `private` | test;  |
| [store::folder_events::tests::kinds](../../src/folder_events/tests.rs#L22) | function_item | `private` | test;  |
| [store::folder_events::tests::every_membership_change_is_announced_and_admission_follows_it](../../src/folder_events/tests.rs#L35) | function_item | `private` | test;  |
| [store::folder_events::tests::the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root](../../src/folder_events/tests.rs#L85) | function_item | `private` | test;  |
| [store::folder_events::tests::cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones](../../src/folder_events/tests.rs#L100) | function_item | `private` | test;  |
| [store::folder_events::tests::leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors](../../src/folder_events/tests.rs#L126) | function_item | `private` | test;  |
| [store::folder_events::tests::admission_is_keyed_by_the_canonical_root](../../src/folder_events/tests.rs#L145) | function_item | `private` | test;  |
| [store::folder_events::tests::tombstones_are_cleaned_even_when_the_ephemeral_scan_fails](../../src/folder_events/tests.rs#L161) | function_item | `private` | test;  |
| [store::folder_events::tests::a_real_removal_failure_is_not_waited_for](../../src/folder_events/tests.rs#L182) | function_item | `private` | test;  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `ThreadStore` | `crate::ThreadStore` | `private` |
| `move_thread_folder` | `crate::move_thread_folder` | `private` |
| `Event` | `schema::Event` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `fs` | `std::fs` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `genesis` | `Value::Bool` | [17](../../src/folder_events/tests.rs#L17) | external-constructor-callback-or-unresolved |
| `genesis` | `Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap` | [19](../../src/folder_events/tests.rs#L19) | receiver-type-required |
| `genesis` | `Event::decode_canonical` | [19](../../src/folder_events/tests.rs#L19) | [schema::event::Event::decode_canonical](../../../schema/src/event.rs#L168) |
| `genesis` | `serde_json_canonicalizer::to_vec(&value).unwrap` | [19](../../src/folder_events/tests.rs#L19) | receiver-type-required |
| `genesis` | `serde_json_canonicalizer::to_vec` | [19](../../src/folder_events/tests.rs#L19) | external-constructor-callback-or-unresolved |
| `kinds` | `seen.lock()         .unwrap()         .iter()         .map(&#124;event&#124; match event {             FolderEvent::Published { id, .. } => format!("published:{id}"),             FolderEvent::Moved { id, from, to, .. } => format!("moved:{id}:{from:?}>{to:?}"),             FolderEvent::Removed { id, reason, .. } => format!("removed:{id}:{reason:?}"),         })         .collect` | [23](../../src/folder_events/tests.rs#L23) | receiver-type-required |
| `kinds` | `seen.lock()         .unwrap()         .iter()         .map` | [23](../../src/folder_events/tests.rs#L23) | receiver-type-required |
| `kinds` | `seen.lock()         .unwrap()         .iter` | [23](../../src/folder_events/tests.rs#L23) | receiver-type-required |
| `kinds` | `seen.lock()         .unwrap` | [23](../../src/folder_events/tests.rs#L23) | receiver-type-required |
| `kinds` | `seen.lock` | [23](../../src/folder_events/tests.rs#L23) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `tempfile::tempdir().unwrap` | [36](../../src/folder_events/tests.rs#L36) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `tempfile::tempdir` | [36](../../src/folder_events/tests.rs#L36) | external-constructor-callback-or-unresolved |
| `every_membership_change_is_announced_and_admission_follows_it` | `dir.path().to_path_buf` | [37](../../src/folder_events/tests.rs#L37) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `dir.path` | [37](../../src/folder_events/tests.rs#L37) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `ThreadStore::open(&root).unwrap` | [38](../../src/folder_events/tests.rs#L38) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `ThreadStore::open` | [38](../../src/folder_events/tests.rs#L38) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `every_membership_change_is_announced_and_admission_follows_it` | `record_events` | [39](../../src/folder_events/tests.rs#L39) | external-constructor-callback-or-unresolved |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.create_thread(A, genesis(A, false)).unwrap` | [42](../../src/folder_events/tests.rs#L42) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.create_thread` | [42](../../src/folder_events/tests.rs#L42), [47](../../src/folder_events/tests.rs#L47), [61](../../src/folder_events/tests.rs#L61), [65](../../src/folder_events/tests.rs#L65) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `genesis` | [42](../../src/folder_events/tests.rs#L42), [47](../../src/folder_events/tests.rs#L47), [61](../../src/folder_events/tests.rs#L61), [65](../../src/folder_events/tests.rs#L65) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `every_membership_change_is_announced_and_admission_follows_it` | `arm_admission` | [44](../../src/folder_events/tests.rs#L44) | external-constructor-callback-or-unresolved |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.create_thread(B, genesis(B, false)).unwrap` | [47](../../src/folder_events/tests.rs#L47) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.archive(A).unwrap` | [51](../../src/folder_events/tests.rs#L51) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.archive` | [51](../../src/folder_events/tests.rs#L51) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.unarchive(A).unwrap` | [53](../../src/folder_events/tests.rs#L53) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.unarchive` | [53](../../src/folder_events/tests.rs#L53) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `move_thread_folder(&root.join("threads").join(B), &root.join("archive").join(B)).unwrap` | [57](../../src/folder_events/tests.rs#L57) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `move_thread_folder` | [57](../../src/folder_events/tests.rs#L57), [58](../../src/folder_events/tests.rs#L58) | [store::folder::move_thread_folder](../../src/folder.rs#L982) |
| `every_membership_change_is_announced_and_admission_follows_it` | `root.join("threads").join` | [57](../../src/folder_events/tests.rs#L57), [58](../../src/folder_events/tests.rs#L58) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `root.join` | [57](../../src/folder_events/tests.rs#L57), [58](../../src/folder_events/tests.rs#L58) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `root.join("archive").join` | [57](../../src/folder_events/tests.rs#L57), [58](../../src/folder_events/tests.rs#L58) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `move_thread_folder(&root.join("archive").join(B), &root.join("threads").join(B)).unwrap` | [58](../../src/folder_events/tests.rs#L58) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.create_thread(C, genesis(C, true)).unwrap` | [61](../../src/folder_events/tests.rs#L61), [65](../../src/folder_events/tests.rs#L65) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.discard_ephemeral(C).unwrap` | [63](../../src/folder_events/tests.rs#L63) | receiver-type-required |
| `every_membership_change_is_announced_and_admission_follows_it` | `store.discard_ephemeral` | [63](../../src/folder_events/tests.rs#L63) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `tempfile::tempdir().unwrap` | [86](../../src/folder_events/tests.rs#L86) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `tempfile::tempdir` | [86](../../src/folder_events/tests.rs#L86) | external-constructor-callback-or-unresolved |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `dir.path` | [87](../../src/folder_events/tests.rs#L87) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `ThreadStore::open(root).unwrap` | [88](../../src/folder_events/tests.rs#L88) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `ThreadStore::open` | [88](../../src/folder_events/tests.rs#L88) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `store.create_thread(A, genesis(A, false)).unwrap` | [89](../../src/folder_events/tests.rs#L89) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `store.create_thread` | [89](../../src/folder_events/tests.rs#L89) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `genesis` | [89](../../src/folder_events/tests.rs#L89) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `root.join("threads").join` | [91](../../src/folder_events/tests.rs#L91), [92](../../src/folder_events/tests.rs#L92) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `root.join` | [91](../../src/folder_events/tests.rs#L91), [92](../../src/folder_events/tests.rs#L92) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `root.join("staging").join` | [91](../../src/folder_events/tests.rs#L91) | receiver-type-required |
| `the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root` | `root.join("archive").join` | [92](../../src/folder_events/tests.rs#L92) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `tempfile::tempdir().unwrap` | [101](../../src/folder_events/tests.rs#L101) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `tempfile::tempdir` | [101](../../src/folder_events/tests.rs#L101) | external-constructor-callback-or-unresolved |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `dir.path().to_path_buf` | [102](../../src/folder_events/tests.rs#L102) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `dir.path` | [102](../../src/folder_events/tests.rs#L102) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `ThreadStore::open(&root).unwrap` | [103](../../src/folder_events/tests.rs#L103) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `ThreadStore::open` | [103](../../src/folder_events/tests.rs#L103) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.create_thread(id, genesis(id, true)).unwrap` | [109](../../src/folder_events/tests.rs#L109) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.create_thread` | [109](../../src/folder_events/tests.rs#L109), [111](../../src/folder_events/tests.rs#L111) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `genesis` | [109](../../src/folder_events/tests.rs#L109), [111](../../src/folder_events/tests.rs#L111) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.create_thread(A, genesis(A, false)).unwrap` | [111](../../src/folder_events/tests.rs#L111) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `crate::LockedLedger::open(root.join("threads").join(held).join("main.jsonl"), 1).unwrap` | [113](../../src/folder_events/tests.rs#L113) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `crate::LockedLedger::open` | [113](../../src/folder_events/tests.rs#L113) | [store::tail::LockedLedger::open](../../src/tail.rs#L186) |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join("threads").join(held).join` | [113](../../src/folder_events/tests.rs#L113) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join("threads").join` | [113](../../src/folder_events/tests.rs#L113) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join` | [113](../../src/folder_events/tests.rs#L113), [121](../../src/folder_events/tests.rs#L121) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.cleanup_stale_ephemeral(&&#124;id&#124; id == owned).unwrap` | [114](../../src/folder_events/tests.rs#L114) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.cleanup_stale_ephemeral` | [114](../../src/folder_events/tests.rs#L114) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir(root.join(".rewrite-trash")).unwrap().count` | [121](../../src/folder_events/tests.rs#L121) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir(root.join(".rewrite-trash")).unwrap` | [121](../../src/folder_events/tests.rs#L121) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir` | [121](../../src/folder_events/tests.rs#L121) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `tempfile::tempdir().unwrap` | [127](../../src/folder_events/tests.rs#L127) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `tempfile::tempdir` | [127](../../src/folder_events/tests.rs#L127) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `dir.path` | [128](../../src/folder_events/tests.rs#L128) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `ThreadStore::open(root).unwrap` | [129](../../src/folder_events/tests.rs#L129) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `ThreadStore::open` | [129](../../src/folder_events/tests.rs#L129) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `root.join` | [130](../../src/folder_events/tests.rs#L130) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join(format!("discard-{A}/assets"))).unwrap` | [131](../../src/folder_events/tests.rs#L131) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all` | [131](../../src/folder_events/tests.rs#L131), [133](../../src/folder_events/tests.rs#L133), [134](../../src/folder_events/tests.rs#L134) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `trash.join` | [131](../../src/folder_events/tests.rs#L131), [132](../../src/folder_events/tests.rs#L132), [133](../../src/folder_events/tests.rs#L133), [134](../../src/folder_events/tests.rs#L134), [139](../../src/folder_events/tests.rs#L139) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::write(trash.join(format!("discard-{A}/main.jsonl")), b"x").unwrap` | [132](../../src/folder_events/tests.rs#L132) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::write` | [132](../../src/folder_events/tests.rs#L132) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join(format!("discard-{B}.deleting-1-1"))).unwrap` | [133](../../src/folder_events/tests.rs#L133) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join("redact-op-keepme")).unwrap` | [134](../../src/folder_events/tests.rs#L134) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `crate::folder::remove_tree_if_present(&trash.join("nope")).unwrap` | [139](../../src/folder_events/tests.rs#L139) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `crate::folder::remove_tree_if_present` | [139](../../src/folder_events/tests.rs#L139) | [store::folder::remove_tree_if_present](../../src/folder.rs#L921) |
| `admission_is_keyed_by_the_canonical_root` | `tempfile::tempdir().unwrap` | [146](../../src/folder_events/tests.rs#L146) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `tempfile::tempdir` | [146](../../src/folder_events/tests.rs#L146) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `fs::canonicalize(dir.path()).unwrap` | [147](../../src/folder_events/tests.rs#L147) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `fs::canonicalize` | [147](../../src/folder_events/tests.rs#L147) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `dir.path` | [147](../../src/folder_events/tests.rs#L147) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `real.join` | [148](../../src/folder_events/tests.rs#L148), [150](../../src/folder_events/tests.rs#L150) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `std::os::unix::fs::symlink(&real, &alias).unwrap` | [149](../../src/folder_events/tests.rs#L149) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `std::os::unix::fs::symlink` | [149](../../src/folder_events/tests.rs#L149) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `real.join("threads").join` | [150](../../src/folder_events/tests.rs#L150) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `arm_admission` | [151](../../src/folder_events/tests.rs#L151) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `ThreadStore::open(&alias).unwrap` | [152](../../src/folder_events/tests.rs#L152) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `ThreadStore::open` | [152](../../src/folder_events/tests.rs#L152) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `admission_is_keyed_by_the_canonical_root` | `store.create_thread(C, genesis(C, false)).unwrap` | [153](../../src/folder_events/tests.rs#L153) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `store.create_thread` | [153](../../src/folder_events/tests.rs#L153) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `genesis` | [153](../../src/folder_events/tests.rs#L153) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tempfile::tempdir().unwrap` | [163](../../src/folder_events/tests.rs#L163) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tempfile::tempdir` | [163](../../src/folder_events/tests.rs#L163) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `dir.path().to_path_buf` | [164](../../src/folder_events/tests.rs#L164) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `dir.path` | [164](../../src/folder_events/tests.rs#L164) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `ThreadStore::open(&root).unwrap` | [165](../../src/folder_events/tests.rs#L165) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `ThreadStore::open` | [165](../../src/folder_events/tests.rs#L165) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `root.join(".rewrite-trash").join` | [166](../../src/folder_events/tests.rs#L166) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `root.join` | [166](../../src/folder_events/tests.rs#L166), [169](../../src/folder_events/tests.rs#L169), [171](../../src/folder_events/tests.rs#L171) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::create_dir_all(tombstone.join("assets")).unwrap` | [167](../../src/folder_events/tests.rs#L167) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::create_dir_all` | [167](../../src/folder_events/tests.rs#L167) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tombstone.join` | [167](../../src/folder_events/tests.rs#L167), [168](../../src/folder_events/tests.rs#L168) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::write(tombstone.join("main.jsonl"), b"old").unwrap` | [168](../../src/folder_events/tests.rs#L168) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::write` | [168](../../src/folder_events/tests.rs#L168) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o000)).unwrap` | [169](../../src/folder_events/tests.rs#L169) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions` | [169](../../src/folder_events/tests.rs#L169), [171](../../src/folder_events/tests.rs#L171) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::Permissions::from_mode` | [169](../../src/folder_events/tests.rs#L169), [171](../../src/folder_events/tests.rs#L171) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `store.cleanup_stale_ephemeral` | [170](../../src/folder_events/tests.rs#L170) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o700)).unwrap` | [171](../../src/folder_events/tests.rs#L171) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tempfile::tempdir().unwrap` | [184](../../src/folder_events/tests.rs#L184) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tempfile::tempdir` | [184](../../src/folder_events/tests.rs#L184) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `dir.path().join` | [185](../../src/folder_events/tests.rs#L185) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `dir.path` | [185](../../src/folder_events/tests.rs#L185) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tree.join` | [186](../../src/folder_events/tests.rs#L186) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::create_dir_all(locked.join("inner")).unwrap` | [187](../../src/folder_events/tests.rs#L187) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::create_dir_all` | [187](../../src/folder_events/tests.rs#L187) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `locked.join` | [187](../../src/folder_events/tests.rs#L187) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap` | [188](../../src/folder_events/tests.rs#L188) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions` | [188](../../src/folder_events/tests.rs#L188), [191](../../src/folder_events/tests.rs#L191) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `fs::Permissions::from_mode` | [188](../../src/folder_events/tests.rs#L188), [191](../../src/folder_events/tests.rs#L191) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `std::time::Instant::now` | [189](../../src/folder_events/tests.rs#L189) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `crate::folder::remove_tree_if_present` | [190](../../src/folder_events/tests.rs#L190) | [store::folder::remove_tree_if_present](../../src/folder.rs#L921) |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap` | [191](../../src/folder_events/tests.rs#L191) | receiver-type-required |
