use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use schema::{Event, EventKind};
use store::{BarrierContext, LockedLedger, SyncPolicy, requires_barrier};
use test_support::{FixtureRoot, read};

/// Gate 42 — a `checkpoint` is a plain covering marker: appending one is a
/// barrier (one full sync for the complete prefix), it names no state asset,
/// and reopening the ledger folds from genesis through it.
#[test]
fn slice3_gate_42_checkpoint_marker_barrier() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("checkpoint folder");
    fs::create_dir(directory.path().join("assets")).expect("assets");
    let source = read(&fixtures.join("replay/checkpoint-valid.jsonl")).expect("ledger");
    fs::write(directory.path().join("main.jsonl"), &source).expect("ledger");
    let sync = Arc::new(CountingSync::default());
    let mut ledger =
        LockedLedger::open_with_sync(directory.path().join("main.jsonl"), 1, sync.clone())
            .expect("ledger");
    let next = ledger.next_seq();
    let seq = ledger
        .create_checkpoint("2026-08-27T09:00:08.000Z", "durable")
        .expect("checkpoint append");
    assert_eq!(seq, next);
    assert_eq!(
        sync.calls.load(Ordering::SeqCst),
        1,
        "the checkpoint is one barrier"
    );
    let checkpoint = ledger
        .projection()
        .expect("projection")
        .events
        .last()
        .cloned()
        .expect("checkpoint event");
    assert_eq!(checkpoint.kind(), &EventKind::Checkpoint);
    assert_eq!(checkpoint.integer_field("covers"), Some(seq - 1));
    assert!(requires_barrier(&checkpoint, BarrierContext::default()));
    assert!(!checkpoint.has_field("state") && !checkpoint.has_field("keys"));
    assert!(
        fs::read_dir(directory.path().join("assets"))
            .expect("assets")
            .next()
            .is_none()
    );
    drop(ledger);
    let reopened = LockedLedger::open(directory.path().join("main.jsonl"), 1)
        .expect("checkpoint reopen folds from genesis");
    assert_eq!(reopened.next_seq(), seq + 1);
    let reopened_checkpoint: Event = reopened
        .projection()
        .expect("projection")
        .events
        .last()
        .cloned()
        .expect("checkpoint event");
    assert_eq!(
        reopened_checkpoint
            .canonical_bytes()
            .expect("reopened bytes"),
        checkpoint.canonical_bytes().expect("checkpoint bytes")
    );
}

#[derive(Default)]
struct CountingSync {
    calls: AtomicUsize,
}

impl SyncPolicy for CountingSync {
    fn full_sync(&self, file: &fs::File) -> std::io::Result<()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        file.sync_all()
    }
}
