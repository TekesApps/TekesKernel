use super::*;
use crate::{ThreadStore, move_thread_folder};
use schema::Event;
use serde_json::{Value, json};
use std::fs;

const A: &str = "018f0000-0000-7000-8000-00000000000a";
const B: &str = "018f0000-0000-7000-8000-00000000000b";
const C: &str = "018f0000-0000-7000-8000-00000000000c";

fn genesis(thread: &str, ephemeral: bool) -> Event {
    let mut value = json!({"kind":"genesis","format":1,"min_reader":1,"min_writer":1,
        "thread":thread,"workspace":"ws","resume":"never","v":1,"seq":1,
        "ts":"2026-09-04T00:00:00.000Z","config":{"digest":"cfg"},"origin_key":format!("create-{thread}"),
        "origin_tuple":{"principal":"p","client":"c","target":thread,"op":"create","key":format!("create-{thread}")}});
    if ephemeral {
        value["ephemeral"] = Value::Bool(true);
    }
    Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap()
}

fn kinds(seen: &Arc<Mutex<Vec<FolderEvent>>>) -> Vec<String> {
    seen.lock()
        .unwrap()
        .iter()
        .map(|event| match event {
            FolderEvent::Published { id, .. } => format!("published:{id}"),
            FolderEvent::Moved { id, from, to, .. } => format!("moved:{id}:{from:?}>{to:?}"),
            FolderEvent::Removed { id, reason, .. } => format!("removed:{id}:{reason:?}"),
        })
        .collect()
}

#[test]
fn every_membership_change_is_announced_and_admission_follows_it() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    let store = ThreadStore::open(&root).unwrap();
    let seen = record_events(&root);

    // Created before arming: announced, but not admitted (a pre-existing session).
    store.create_thread(A, genesis(A, false)).unwrap();
    assert!(!is_admitted(&root, A));
    arm_admission(&root);
    assert!(!is_admitted(&root, A), "arming does not retro-admit");

    store.create_thread(B, genesis(B, false)).unwrap();
    assert!(is_admitted(&root, B));

    // archive then unarchive: unarchive admits (S6), including a pre-existing session
    store.archive(A).unwrap();
    assert!(!is_admitted(&root, A));
    store.unarchive(A).unwrap();
    assert!(is_admitted(&root, A), "an unarchived session is admitted");

    // direct use of the chokepoint (what `drive_folder_move` calls)
    move_thread_folder(&root.join("threads").join(B), &root.join("archive").join(B)).unwrap();
    move_thread_folder(&root.join("archive").join(B), &root.join("threads").join(B)).unwrap();

    // discard of an ephemeral session removes it from admission; reuse of the id re-admits
    store.create_thread(C, genesis(C, true)).unwrap();
    assert!(is_admitted(&root, C));
    store.discard_ephemeral(C).unwrap();
    assert!(!is_admitted(&root, C));
    store.create_thread(C, genesis(C, true)).unwrap();
    assert!(is_admitted(&root, C));

    assert_eq!(
        kinds(&seen),
        vec![
            format!("published:{A}"),
            format!("published:{B}"),
            format!("moved:{A}:Threads>Archive"),
            format!("moved:{A}:Archive>Threads"),
            format!("moved:{B}:Threads>Archive"),
            format!("moved:{B}:Archive>Threads"),
            format!("published:{C}"),
            format!("removed:{C}:Discarded"),
            format!("published:{C}"),
        ]
    );
}

#[test]
fn the_chokepoint_refuses_moves_that_are_not_between_areas_of_one_root() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let store = ThreadStore::open(root).unwrap();
    store.create_thread(A, genesis(A, false)).unwrap();
    for (from, to) in [
        (root.join("threads").join(A), root.join("staging").join(A)),
        (root.join("threads").join(A), root.join("archive").join(B)),
    ] {
        assert!(move_thread_folder(&from, &to).is_err());
        assert!(root.join("threads").join(A).is_dir());
    }
}

#[test]
fn cleanup_removes_only_stale_ephemeral_threads_and_never_owned_or_held_ones() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    let store = ThreadStore::open(&root).unwrap();
    // three ephemeral threads and one durable one exist from a previous process
    let owned = "018f0000-0000-7000-8000-0000000000d1";
    let held = "018f0000-0000-7000-8000-0000000000d2";
    let stale = "018f0000-0000-7000-8000-0000000000d3";
    for id in [owned, held, stale] {
        store.create_thread(id, genesis(id, true)).unwrap();
    }
    store.create_thread(A, genesis(A, false)).unwrap();
    let _worker =
        crate::LockedLedger::open(root.join("threads").join(held).join("main.jsonl"), 1).unwrap();
    let report = store.cleanup_stale_ephemeral(&|id| id == owned).unwrap();
    assert_eq!(report.removed, vec![stale.to_owned()]);
    assert_eq!(report.busy, vec![held.to_owned()]);
    assert!(root.join("threads").join(owned).is_dir());
    assert!(root.join("threads").join(held).is_dir());
    assert!(root.join("threads").join(A).is_dir());
    assert!(!root.join("threads").join(stale).exists());
    let leftovers = fs::read_dir(root.join(".rewrite-trash")).unwrap().count();
    assert_eq!(leftovers, 0, "the private tombstone is deleted");
}

/// O-final m1: the sweep announces the stale ephemeral thread once, when it leaves `threads/`
/// (`Discarded`). Deleting its tombstone afterwards announces nothing: a late event would
/// un-admit and forget a thread re-created under the same id while the tree was still being
/// deleted.
#[test]
fn deleting_a_swept_threads_tombstone_announces_nothing_more() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    let store = ThreadStore::open(&root).unwrap();
    store.create_thread(C, genesis(C, true)).unwrap();
    arm_admission(&root);
    let seen = record_events(&root);
    let report = store.cleanup_stale_ephemeral(&|_| false).unwrap();
    assert_eq!(report.removed, vec![C.to_owned()]);
    assert_eq!(kinds(&seen), vec![format!("removed:{C}:Discarded")]);
    // the id is re-created; nothing the sweep does afterwards may forget it
    store.create_thread(C, genesis(C, true)).unwrap();
    assert!(is_admitted(&root, C));
    assert_eq!(store.cleanup_discard_tombstones().unwrap(), 0);
    assert!(
        is_admitted(&root, C),
        "no late event un-admits the new thread"
    );
}

#[test]
fn leftover_tombstones_are_cleaned_and_vanishing_ones_are_not_errors() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let store = ThreadStore::open(root).unwrap();
    let trash = root.join(".rewrite-trash");
    fs::create_dir_all(trash.join(format!("discard-{A}/assets"))).unwrap();
    fs::write(trash.join(format!("discard-{A}/main.jsonl")), b"x").unwrap();
    fs::create_dir_all(trash.join(format!("discard-{B}.deleting-1-1"))).unwrap();
    fs::create_dir_all(trash.join("redact-op-keepme")).unwrap();
    assert_eq!(store.cleanup_discard_tombstones().unwrap(), 2);
    assert!(trash.join("redact-op-keepme").is_dir());
    assert!(!trash.join(format!("discard-{A}")).exists());
    // a path that is already gone is success
    crate::folder::remove_tree_if_present(&trash.join("nope")).unwrap();
}

/// E832 hardening: events and admission are keyed by the canonical root, so a store opened
/// through a symlink (or `..`) and a host armed with the real path agree.
#[test]
fn admission_is_keyed_by_the_canonical_root() {
    let dir = tempfile::tempdir().unwrap();
    let real = fs::canonicalize(dir.path()).unwrap();
    let alias = real.join("alias");
    std::os::unix::fs::symlink(&real, &alias).unwrap();
    let dotted = real.join("threads").join("..");
    arm_admission(&real);
    let store = ThreadStore::open(&alias).unwrap();
    store.create_thread(C, genesis(C, false)).unwrap();
    assert!(is_admitted(&real, C));
    assert!(is_admitted(&alias, C));
    assert!(is_admitted(&dotted, C));
}

/// E833 F4: tombstones are cleaned even when the ephemeral scan fails.
#[test]
fn tombstones_are_cleaned_even_when_the_ephemeral_scan_fails() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    let store = ThreadStore::open(&root).unwrap();
    let tombstone = root.join(".rewrite-trash").join(format!("discard-{A}"));
    fs::create_dir_all(tombstone.join("assets")).unwrap();
    fs::write(tombstone.join("main.jsonl"), b"old").unwrap();
    fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o000)).unwrap();
    let outcome = store.cleanup_stale_ephemeral(&|_| false);
    fs::set_permissions(root.join("threads"), fs::Permissions::from_mode(0o700)).unwrap();
    assert!(
        outcome.is_err(),
        "the unreadable threads directory is an error"
    );
    assert!(!tombstone.exists(), "the tombstone was still removed");
}

/// E833 F3: only the races of a concurrent deleter are waited for; a real failure returns at
/// once instead of after the 10 s bound.
#[test]
fn a_real_removal_failure_is_not_waited_for() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let tree = dir.path().join("tree");
    let locked = tree.join("locked");
    fs::create_dir_all(locked.join("inner")).unwrap();
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
    let started = std::time::Instant::now();
    let outcome = crate::folder::remove_tree_if_present(&tree);
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap();
    assert!(outcome.is_err(), "an unremovable tree is an error");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(2),
        "took {:?}",
        started.elapsed()
    );
}
