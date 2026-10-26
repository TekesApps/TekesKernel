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
| [store::folder_events::tests::deleting_a_swept_threads_tombstone_announces_nothing_more](../../src/folder_events/tests.rs#L133) | function_item | `private` | test;  |
| [store::folder_events::tests::leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors](../../src/folder_events/tests.rs#L154) | function_item | `private` | test;  |
| [store::folder_events::tests::admission_is_keyed_by_the_canonical_root](../../src/folder_events/tests.rs#L173) | function_item | `private` | test;  |
| [store::folder_events::tests::tombstones_are_cleaned_even_when_the_ephemeral_scan_fails](../../src/folder_events/tests.rs#L189) | function_item | `private` | test;  |
| [store::folder_events::tests::a_real_removal_failure_is_not_waited_for](../../src/folder_events/tests.rs#L210) | function_item | `private` | test;  |

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
| `genesis` | `Event::decode_canonical` | [19](../../src/folder_events/tests.rs#L19) | [schema::event::Event::decode_canonical](../../../schema/src/event.rs#L171) |
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
| `every_membership_change_is_announced_and_admission_follows_it` | `move_thread_folder` | [57](../../src/folder_events/tests.rs#L57), [58](../../src/folder_events/tests.rs#L58) | [store::folder::move_thread_folder](../../src/folder.rs#L986) |
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
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `crate::LockedLedger::open(         root.join("threads").join(held).join("main.jsonl"),         schema::READER_VERSION,     )     .unwrap` | [112](../../src/folder_events/tests.rs#L112) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `crate::LockedLedger::open` | [112](../../src/folder_events/tests.rs#L112) | [store::tail::LockedLedger::open](../../src/tail.rs#L186) |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join("threads").join(held).join` | [113](../../src/folder_events/tests.rs#L113) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join("threads").join` | [113](../../src/folder_events/tests.rs#L113) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `root.join` | [113](../../src/folder_events/tests.rs#L113), [124](../../src/folder_events/tests.rs#L124) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.cleanup_stale_ephemeral(&&#124;id&#124; id == owned).unwrap` | [117](../../src/folder_events/tests.rs#L117) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `store.cleanup_stale_ephemeral` | [117](../../src/folder_events/tests.rs#L117) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir(root.join(".rewrite-trash")).unwrap().count` | [124](../../src/folder_events/tests.rs#L124) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir(root.join(".rewrite-trash")).unwrap` | [124](../../src/folder_events/tests.rs#L124) | receiver-type-required |
| `cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones` | `fs::read_dir` | [124](../../src/folder_events/tests.rs#L124) | external-constructor-callback-or-unresolved |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `tempfile::tempdir().unwrap` | [134](../../src/folder_events/tests.rs#L134) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `tempfile::tempdir` | [134](../../src/folder_events/tests.rs#L134) | external-constructor-callback-or-unresolved |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `dir.path().to_path_buf` | [135](../../src/folder_events/tests.rs#L135) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `dir.path` | [135](../../src/folder_events/tests.rs#L135) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `ThreadStore::open(&root).unwrap` | [136](../../src/folder_events/tests.rs#L136) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `ThreadStore::open` | [136](../../src/folder_events/tests.rs#L136) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `store.create_thread(C, genesis(C, true)).unwrap` | [137](../../src/folder_events/tests.rs#L137), [144](../../src/folder_events/tests.rs#L144) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `store.create_thread` | [137](../../src/folder_events/tests.rs#L137), [144](../../src/folder_events/tests.rs#L144) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `genesis` | [137](../../src/folder_events/tests.rs#L137), [144](../../src/folder_events/tests.rs#L144) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `arm_admission` | [138](../../src/folder_events/tests.rs#L138) | external-constructor-callback-or-unresolved |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `record_events` | [139](../../src/folder_events/tests.rs#L139) | external-constructor-callback-or-unresolved |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `store.cleanup_stale_ephemeral(&&#124;_&#124; false).unwrap` | [140](../../src/folder_events/tests.rs#L140) | receiver-type-required |
| `deleting_a_swept_threads_tombstone_announces_nothing_more` | `store.cleanup_stale_ephemeral` | [140](../../src/folder_events/tests.rs#L140) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `tempfile::tempdir().unwrap` | [155](../../src/folder_events/tests.rs#L155) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `tempfile::tempdir` | [155](../../src/folder_events/tests.rs#L155) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `dir.path` | [156](../../src/folder_events/tests.rs#L156) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `ThreadStore::open(root).unwrap` | [157](../../src/folder_events/tests.rs#L157) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `ThreadStore::open` | [157](../../src/folder_events/tests.rs#L157) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `root.join` | [158](../../src/folder_events/tests.rs#L158) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join(format!("discard-{A}/assets"))).unwrap` | [159](../../src/folder_events/tests.rs#L159) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all` | [159](../../src/folder_events/tests.rs#L159), [161](../../src/folder_events/tests.rs#L161), [162](../../src/folder_events/tests.rs#L162) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `trash.join` | [159](../../src/folder_events/tests.rs#L159), [160](../../src/folder_events/tests.rs#L160), [161](../../src/folder_events/tests.rs#L161), [162](../../src/folder_events/tests.rs#L162), [167](../../src/folder_events/tests.rs#L167) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::write(trash.join(format!("discard-{A}/main.jsonl")), b"x").unwrap` | [160](../../src/folder_events/tests.rs#L160) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::write` | [160](../../src/folder_events/tests.rs#L160) | external-constructor-callback-or-unresolved |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join(format!("discard-{B}.deleting-1-1"))).unwrap` | [161](../../src/folder_events/tests.rs#L161) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `fs::create_dir_all(trash.join("redact-op-keepme")).unwrap` | [162](../../src/folder_events/tests.rs#L162) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `crate::folder::remove_tree_if_present(&trash.join("nope")).unwrap` | [167](../../src/folder_events/tests.rs#L167) | receiver-type-required |
| `leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors` | `crate::folder::remove_tree_if_present` | [167](../../src/folder_events/tests.rs#L167) | [store::folder::remove_tree_if_present](../../src/folder.rs#L925) |
| `admission_is_keyed_by_the_canonical_root` | `tempfile::tempdir().unwrap` | [174](../../src/folder_events/tests.rs#L174) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `tempfile::tempdir` | [174](../../src/folder_events/tests.rs#L174) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `fs::canonicalize(dir.path()).unwrap` | [175](../../src/folder_events/tests.rs#L175) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `fs::canonicalize` | [175](../../src/folder_events/tests.rs#L175) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `dir.path` | [175](../../src/folder_events/tests.rs#L175) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `real.join` | [176](../../src/folder_events/tests.rs#L176), [178](../../src/folder_events/tests.rs#L178) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `std::os::unix::fs::symlink(&real, &alias).unwrap` | [177](../../src/folder_events/tests.rs#L177) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `std::os::unix::fs::symlink` | [177](../../src/folder_events/tests.rs#L177) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `real.join("threads").join` | [178](../../src/folder_events/tests.rs#L178) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `arm_admission` | [179](../../src/folder_events/tests.rs#L179) | external-constructor-callback-or-unresolved |
| `admission_is_keyed_by_the_canonical_root` | `ThreadStore::open(&alias).unwrap` | [180](../../src/folder_events/tests.rs#L180) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `ThreadStore::open` | [180](../../src/folder_events/tests.rs#L180) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `admission_is_keyed_by_the_canonical_root` | `store.create_thread(C, genesis(C, false)).unwrap` | [181](../../src/folder_events/tests.rs#L181) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `store.create_thread` | [181](../../src/folder_events/tests.rs#L181) | receiver-type-required |
| `admission_is_keyed_by_the_canonical_root` | `genesis` | [181](../../src/folder_events/tests.rs#L181) | [store::folder_events::tests::genesis](../../src/folder_events/tests.rs#L11) |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tempfile::tempdir().unwrap` | [191](../../src/folder_events/tests.rs#L191) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tempfile::tempdir` | [191](../../src/folder_events/tests.rs#L191) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `dir.path().to_path_buf` | [192](../../src/folder_events/tests.rs#L192) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `dir.path` | [192](../../src/folder_events/tests.rs#L192) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `ThreadStore::open(&root).unwrap` | [193](../../src/folder_events/tests.rs#L193) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `ThreadStore::open` | [193](../../src/folder_events/tests.rs#L193) | [store::folder::ThreadStore::open](../../src/folder.rs#L36) |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `root.join(".rewrite-trash").join` | [194](../../src/folder_events/tests.rs#L194) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `root.join` | [194](../../src/folder_events/tests.rs#L194), [197](../../src/folder_events/tests.rs#L197), [199](../../src/folder_events/tests.rs#L199) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::create_dir_all(tombstone.join("assets")).unwrap` | [195](../../src/folder_events/tests.rs#L195) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::create_dir_all` | [195](../../src/folder_events/tests.rs#L195) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `tombstone.join` | [195](../../src/folder_events/tests.rs#L195), [196](../../src/folder_events/tests.rs#L196) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::write(tombstone.join("main.jsonl"), b"old").unwrap` | [196](../../src/folder_events/tests.rs#L196) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::write` | [196](../../src/folder_events/tests.rs#L196) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o000)).unwrap` | [197](../../src/folder_events/tests.rs#L197) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions` | [197](../../src/folder_events/tests.rs#L197), [199](../../src/folder_events/tests.rs#L199) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::Permissions::from_mode` | [197](../../src/folder_events/tests.rs#L197), [199](../../src/folder_events/tests.rs#L199) | external-constructor-callback-or-unresolved |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `store.cleanup_stale_ephemeral` | [198](../../src/folder_events/tests.rs#L198) | receiver-type-required |
| `tombstones_are_cleaned_even_when_the_ephemeral_scan_fails` | `fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o700)).unwrap` | [199](../../src/folder_events/tests.rs#L199) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tempfile::tempdir().unwrap` | [212](../../src/folder_events/tests.rs#L212) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tempfile::tempdir` | [212](../../src/folder_events/tests.rs#L212) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `dir.path().join` | [213](../../src/folder_events/tests.rs#L213) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `dir.path` | [213](../../src/folder_events/tests.rs#L213) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `tree.join` | [214](../../src/folder_events/tests.rs#L214) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::create_dir_all(locked.join("inner")).unwrap` | [215](../../src/folder_events/tests.rs#L215) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::create_dir_all` | [215](../../src/folder_events/tests.rs#L215) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `locked.join` | [215](../../src/folder_events/tests.rs#L215) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap` | [216](../../src/folder_events/tests.rs#L216) | receiver-type-required |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions` | [216](../../src/folder_events/tests.rs#L216), [219](../../src/folder_events/tests.rs#L219) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `fs::Permissions::from_mode` | [216](../../src/folder_events/tests.rs#L216), [219](../../src/folder_events/tests.rs#L219) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `std::time::Instant::now` | [217](../../src/folder_events/tests.rs#L217) | external-constructor-callback-or-unresolved |
| `a_real_removal_failure_is_not_waited_for` | `crate::folder::remove_tree_if_present` | [218](../../src/folder_events/tests.rs#L218) | [store::folder::remove_tree_if_present](../../src/folder.rs#L925) |
| `a_real_removal_failure_is_not_waited_for` | `fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap` | [219](../../src/folder_events/tests.rs#L219) | receiver-type-required |
