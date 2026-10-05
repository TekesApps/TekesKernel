# Durability, replay and settlement

[Data and reliability · documentation home](../README.md) · [Event format](events.md) · [Next: idempotency and retries](idempotency.md)

## Durability

- Writer opens with `O_APPEND` and `O_CLOEXEC`; one event = one checked
  full-write loop (partial writes retried to completion or fail-stop).
- **Barrier events** — settle, `attempt`, `attempt_dispatched`,
  `attempt_recovery` (the adopt commit point — R10-4), `approval_request`
  (the parked-hold marker, synced before the frame is exposed or an idle
  exit is permitted — R10-1), `approval_response` (the ack-bearing answer),
  `queue_edit`, `stop_requested`, manually-originated `compact`,
  externally-originated `meta` (R17k), `checkpoint`,
  **terminal `output`/`error` that settle an
  attempt, carrying the attempt's `usage`, committed by one barrier** (R4-1: a settled outcome can never exist
  without its usage; baseline advance, `attempt_settled`, and lease release
  all wait for this sync — R3-2), side-effect `tool_call` intents,
  `effective_execution`, `spawn`, `genesis`, `run_start`, **upgrade `meta`
  before any event governed by its raised minima** (R4-3), and any `input`
  being acknowledged to a client — get a **durability barrier** before the
  dependent action:
  F_FULLFSYNC on macOS (plain `fsync` does not order power-loss on Darwin),
  configurable only downward where loss provably means re-run (D-45).
- **Reference protocol** for any event referencing a file created for it
  (assets, child files, workspace snapshots): full write → file sync →
  `rename` → parent-directory sync → then sync the referencing event. A
  durable event must never point at a possibly-missing object (F9).
- Other events: flushed per line, sync opportunistic. Losing a non-synced
  tail on power failure re-runs a turn; it never corrupts one.
- **Write failure (ENOSPC etc.):** intent-sync failure → the side effect is
  not executed; no acknowledgement before sync; persistent append failure →
  the worker fail-stops (exit nonzero, no settle) and the supervisor holds
  the thread in a degraded read-only state — deliveries refused with an
  explicit error until space clears, then the sweep resumes. Terminal
  diagnostics write into a **preallocated reserve file** at the storage root
  (created at boot, truncated into on demand) so the failure itself is
  recordable (F19).
- **Torn-tail repair:** truncate to the **longest valid prefix** — a
  barrier's F_FULLFSYNC orders everything through it; lines past the first
  invalid point are post-barrier best-effort and are discarded, valid-looking
  or not (event §Tail framing — R15-6). Only the lock holder repairs;
  readers simply ignore the discarded suffix. Out-of-band facts pointing
  beyond the longest valid prefix mean corruption: fail-stop.

## Concurrency

- **One writer per file, enforced by `flock(LOCK_EX)` on the file itself.** The
  writer is the worker; when no worker is alive, the supervisor may take the
  lock to append **delivered inputs, `queue_edit`s, stop intents, and
  genesis-on-create only** (the serialized creation path authors `genesis`
  before any worker exists — R7-4); semantic recovery facts are
  worker-authored (D-61). "Whoever holds the
  lock writes" is the entire write-coordination story.
- **Lock hygiene (F8):** the lock fd — like every worker-held fd — is
  `O_CLOEXEC`, and spawn actions close-from, so no tool or MCP child can keep
  a dead worker's lock alive (Darwin inherits flocks across fork and plain
  fds across exec). After acquiring a lock, revalidate path→inode against the
  live folder before writing — a rename (archive) between open and lock means
  release and retry through the folder lifecycle lock (defined in [storage](storage.md): a flock
  on the thread directory fd — workers hold it shared for their lifetime,
  the supervisor takes it exclusive for archive/delete/publish).
- Readers open read-only, never lock, read to the last complete line. Tail
  consumers resume from a `seq` cursor.
- These semantics are guaranteed only on a **single host, supported local
  filesystem**; boot probes lock/append capability and refuses network or
  unknown volumes (F10).

## Replay

Replay is the pure fold of the complete longest-valid prefix, always from
genesis; a `checkpoint` is a covering marker, never a replay seed. Same file
and assets must produce the same in-memory state. Requirements:

- Side effects are **not** re-executed on replay; `tool_call` without
  `tool_result` at the tail is reconciled (see [worker recovery](../runtime/worker.md#recovery-transitions-d-61-closed-r7)), not re-run blindly.
- All state the loop needs next must be derivable from the tail: an unpaired
  `tool_call` means "mid-tool", an unresolved `approval_request` means a
  hold — parked or answered per
  [spec/tail-lifecycle](../../spec/tail-lifecycle.md) (R9-2/R11-1) — and a
  settle event means "idle". If the loop ever needs state that replay cannot
  reconstruct, that state must become an event.

## Settlement

The domain meaning is defined in [Thread fundamentals](../concepts/thread.md#progress-output-candidates-and-settlement).
The [event contract](../../spec/event.md#settle-barrier-exactly-one-per-turn) owns
its exact fields: `completed | interrupted | error`; a completed validation
settlement carries `pass | inconclusive | not_required` and the exact promoted
output/candidate/decision references. A final provider output or a validator's
own completion cannot substitute for the root turn's settle. A failed verdict
is repair input; capped failure is `inconclusive`, never fabricated `pass`.

The following lifecycle distinction is governed by
[tail-lifecycle](../../spec/tail-lifecycle.md); it does not mean that recording a
settle instantly terminates the writer process.

Two named conditions (D-3): a turn is **terminal_recorded** iff its settle
event is durable on disk; it is **settled** iff terminal_recorded ∧ no live
process holds the file lock. Client turn presentation follows the projected
terminal record (`turn/end`); liveness and sweep follow the classified tail,
not an inference from the final-answer flag. A worker that recorded its settle
but hangs still holds the line. A settle event
is authored only by a run — ordinary or reconcile-only recovery (D-61). The
supervisor never writes one; no mirror, client, or projection ever writes
one. **Archive keys on not-running** (no lock holder), not on settled:
settled and parked threads are both archivable, and a parked hold —
log-derived — survives archive/unarchive (R10-5).
