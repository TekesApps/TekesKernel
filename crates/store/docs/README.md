# store — durable logs and assets

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Owns JSONL files, content-addressed assets, locks, durable synchronization and rewrite recovery; it uses schema to interpret data.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `folder` | ThreadStore creation, conditional append and thread directories |
| `tail` | LockedLedger, valid-prefix scanning and checkpoint replay |
| `asset` | AssetStore content-addressed assets |
| `atomic` | AtomicPublisher atomic publication |
| `platform` | System file locks and synchronization |
| `rewrite` | Fork and redaction publication, recovery and debris collection |

## Interfaces and calls

Main entry points: ThreadStore, LockedLedger, AssetStore, AtomicPublisher, scan_valid_prefix_seeded.

Open log → validate valid prefix → schema fold; appending validates the event before writing and syncing according to barrier rules.

`LockedLedger` checks reader and writer minima before checkpoint replay or tail
repair, and rejects unsupported versions before append. Its writer capability is
fixed by the implementation; selecting a newer reader does not enable a newer
writer. Read-only scans remain available without opening a writer.
Thread creation applies the same gate to the supplied genesis and to existing or
staged targets before publishing retry assets. Rewrite checks source and rebuilt
payload versions; redaction revalidates its source before retirement.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Disk mutation and semantic interpretation belong to store and schema respectively; memory caches cannot replace file locks or durable logs.

`RewriteKind` contains `Fork` and `Redact`. Automatic context compaction is
authored by the worker as a checkpoint and `compact` event through `LockedLedger`;
it does not run a store rewrite transaction.

`ThreadStore::append_keyed_with_ledger_if` is the keyed conditional append
whose builder owns the locked main ledger for the write (checkpoint and asset
publication before the event), with the same lifecycle lock, rewrite gate,
origin dedup and seq/origin verification as the projection-only form; the
supervisor uses it to author a manual `compact` on a line with no live worker.

Behavior contract: [event.md](../../../spec/event.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
