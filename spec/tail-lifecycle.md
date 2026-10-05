# spec: tail-lifecycle v1 (r5 — R15-2/4 closure)

The single normative tail-classification function and per-actor action
matrix. Every actor consults **this file and nothing else**; restatements in
docs 02/03/04/06/12 are non-normative narrative.

## External management pre-gate

Tail state remains computable from `(file bytes, lock)` alone. Before any
actor enters the matrix, however, the supervisor MUST inspect the durable
session-endpoint §Management authority and recovery operation registry for the target session. An
incomplete queue transaction is a launch/delivery pre-gate, not an eighth tail
state:

1. it blocks every ordinary candidate, ensure, queue projection and later
   delivery, including a stop or input that arrived after the transaction;
2. the supervisor ensures exactly one gate-exempt **reconcile-only** worker,
   preloads the same worker-control `queue_transaction` during negotiation;
   the worker acquires the line lock, appends its `run_start` attribution, and
   executes that buffered transaction before generic reconciliation;
3. after a committed `queue_transaction_result` and durable management
   `complete`, blocked deliveries resume in arrival order and the actor then
   evaluates this matrix from the new tail. A rejected result durably completes
   the endpoint operation with its typed error, authors no event, releases the
   gate, and then follows the same re-evaluation rule.

The live worker that began the transaction may complete it without another
run. If it dies after the retraction, no ordinary work can observe or consume
past that prefix before the reconcile worker appends the missing replacement.
Boot performs this pre-gate recovery before ordinary sweep/readiness. This is
the D-61 recovery-run path; the supervisor still authors no semantic fact.

## Inputs

All predicates are over the **logical tail**, derived from one file's events
(total order), plus two lock facts:

- `lock_held_by_other` — a live process other than the evaluator holds the
  file flock.
- `lock_held_by_caller` — the evaluator itself holds the flock (a winning
  candidate classifying under its own lock — R12-2).
- `latest_turn` — the highest `turn` value on any turn-bound event; absent
  if none.
- `terminal_tail` — `latest_turn` exists and carries its `settle`. **Later
  file-scoped events (queue_edit, meta, stop_requested, checkpoint, …)
  never un-terminalize a tail** (R12-1).
- `stop_active(G)` — a root `stop_requested(G)` exists and G is not
  closed. **Closure** (R13-1/R15-2): the turn open at G's append reaching
  its final settle closes G. A G appended when no turn is open splits by
  file kind: on a **settled** file (terminal tail, or a parentless
  turn-less file) it is **closed at birth** — nothing to stop; its one
  effect is that inputs with seq < G.seq are pre-closure, hence held. On
  an **unstarted** file (spawned child, no turn ever) it is **open** and
  closes at turn 1's final settle — the cancel-before-start run (see the
  stopped_active row). Closure is permanent. A generation's **closure
  point** is the closing settle's seq (born-closed: G's own seq).
- `open_hold` — the latest `approval_request` R has no matching
  `approval_response` and R's call has no terminal `tool_result`; the only
  turn-bound events after R are control echoes (file-scoped events —
  queued inputs, queue edits, … — are always hold-permitted).
- `answered_hold` — the latest `approval_request` R has a matching
  `approval_response` (grant or denial) and R's call has no terminal
  `tool_result`.
- `runnable_input` — a **live queued input** (contentful, non-steer,
  unconsumed — named by no `turn_open` — and unsuperseded) that (a) is
  not queued behind an open hold, and (b) has seq greater than the
  **closure point** of the latest stop generation — **vacuously satisfied
  when no stop generation has ever existed** (R12-8); no input is
  runnable while a generation is open (stop dominates). `queue_edit`s and
  steer inputs are never runnable triggers.
- **Release** (R14-2): only a runnable input triggers a run, but the
  `turn_open` that consumes it MUST also consume, in seq order ahead of
  it, every live unconsumed contentful non-steer input with smaller seq
  (event constraint 8's prefix-completeness) — held pre-closure inputs
  are never triggers yet always ride the next trigger's turn.
- `unstarted` — the file's genesis carries `parent` and no turn has ever
  opened (no `turn_open` exists — R14-5).
- `unresolved_work` — any of: unpaired non-spawn `tool_call` (excluding
  calls in an incomplete adopt inventory and the latest **open or answered**
  hold's ask call),
  unpaired `attempt`, incomplete adopt journal, `spawn` without
  `child_result` — all within or before `latest_turn`.
  A later call in the same serial provider batch is also excluded while an
  earlier call has an unanswered approval, but only when it declares
  `execution_tracked: true` and has no `tool_execution_started` record. Such a
  sibling is waiting behind the approval, not crash residue. Legacy calls,
  calls already started, and calls in other batches remain unresolved; an
  unfinished provider attempt remains an independent unresolved obligation.
- `resume_permits` — the spawn's `resume` policy and `recovery_ordinal`
  allow continuation (D-60).
- `continuation_wait_until` — the latest `poll_after` among unpaired
  `tool_call`s whose most recent `state{subkind: tool_continuation}` record is
  a park (event) and that have no `approval_request`; absent otherwise. A
  parked remote continuation is still `unresolved_work` (the line classifies
  `recovery_needed`), but the ensure/sweep evaluation with a clock spawns no
  candidate before that instant (`ensure_action_at`), and the run mode for
  such a line is **ordinary** regardless of the `resume` policy: finishing a
  durable continuation obligation is not an unsolicited resume.
- `admission_wait_until` — `next_attempt_at` of the latest
  `state{subkind: provider_admission}` record (event) not yet followed by an
  `attempt` or `settle`; absent otherwise. The worker writes it before sleeping
  out a rate-limited or transport retry, so a worker killed mid-wait leaves the retry as a
  durable obligation of the open turn (the line classifies `recovery_needed`
  with no unresolved work); it is evaluated exactly like
  `continuation_wait_until` — no candidate before the instant, **ordinary**
  run mode regardless of the `resume` policy — through
  `LifecycleFacts::durable_wait_until`, the later of the two.

## States (first match wins)

| # | State | Definition |
|---|---|---|
| 1 | `running` | `lock_held_by_other` |
| 2 | `stopped_active` | `stop_active` (an open generation normally has a turn in flight; the deliberate exception is a stopped unstarted child awaiting cancel-before-start) |
| 3 | `answered_hold` | answered-hold predicate |
| 4 | `parked_hold` | `open_hold` ∧ ¬`unresolved_work` |
| 5 | `recovery_needed` | ¬`terminal_tail` ∧ `latest_turn` exists ∧ (`unresolved_work` ∨ ¬`open_hold`) |
| 6 | `unstarted` | unstarted predicate (spawned child, no turn ever — R14-5) |
| 7 | `settled` | `terminal_tail` ∨ no `latest_turn` |

A caller holding the lock (`lock_held_by_caller`) skips row 1 and
classifies rows 2–7 — that IS the under-lock evaluation (R12-2); the
pre-lock classification is a hint only.

**Acquisition** (R13-8): a candidate takes the file flock **nonblocking**
(`LOCK_EX | LOCK_NB`). Contention — `EWOULDBLOCK`/`EAGAIN` (another
holder) — means the candidate exits 0 having appended nothing; candidates
never block on or poll the lock, and re-attempts come only from a fresh
ensure/sweep pass. `EINTR` retries the call. Any other failure (`EBADF`,
`ENOLCK`, an unsupported filesystem) is an **operational error**: exit
nonzero, nothing appended, surfaced by the supervisor — never classified
as lock loss.

## Action matrix (one action per cell)

| State | delivery of non-answer input / queue_edit | delivery of matching answer | ensure / reaper / sweeps | worker mode (under caller's lock) | archive request |
|---|---|---|---|---|---|
| `running` | forward to stdin | forward to stdin | none (busy) | lost acquisition → exit 0, no append (§Acquisition) | queue until not running |
| `stopped_active` | append+ack; stays held | reject `stop-active`, **no append**; stop/recovery supplies the eventual cancelled resolution | spawn **recovery** candidate (gate-exempt) | **reconcile-only, always**; on an unstarted file: open turn 1 with `trigger: "genesis"` **solely to settle it** `interrupted/recovered` — cancel-before-start (R15-2) | queue until closure |
| `answered_hold` | append+ack; queues behind the hold | (already answered) | spawn resume candidate | **ordinary, same-turn resume, regardless of `resume` policy** | **queue until the hold resolves** |
| `parked_hold` | append+ack; **no spawn**, stays parked | append+ack; spawn (state becomes `answered_hold`) | **no spawn** | exit 0 (a run here should not exist) | **allowed** — hold is log-derived, survives round-trips |
| `recovery_needed` | append+ack; spawn candidate | hold without append until recovery reclassifies; redeliver the same response only if the request remains unresolved | spawn recovery candidate | stop → reconcile-only; else answered/runnable rows above; else `resume_permits` → ordinary; else reconcile-only | **queue until recovery settles** |
| `unstarted` | append+ack; queued for a later turn | n/a | **spawn first-run candidate** | **ordinary — open turn 1 with `trigger: "genesis"`** | queue until first settle |
| `settled` | append+ack; spawn per runnable rules | n/a | no spawn unless runnable input | runnable input → ordinary; else exit 0 | **allowed** |

File-scoped keyed control mutations (`queue_edit`, externally-originated
`meta`, manual `compact`) take the input delivery column's append+ack in
every non-running state — the "held" clauses apply to inputs only;
`running` forwards them to stdin (R17k).

## Archived threads (R13-6)

Archive moves the folder under `archive/`; the mover MUST hold the flock
and re-verify the archive column under it (01's folder lifecycle lock
sequences the `mv` against live processes). An archived thread is
**outside this matrix**: every delivery — input, answer, queue_edit, stop,
compact — is rejected with a typed `archived` error, never appended and
never auto-unarchiving; ensure/reaper/sweeps skip archived folders.
Unarchive is an explicit supervisor operation (the reverse `mv`, no
append); jurisdiction resumes and the state is recomputed from the log —
a parked hold survives the round-trip, and the rejected answer may then be
redelivered.

Supervisor ensure failures are isolated per ledger and failure reason. Sweep
and exit reconciliation share monotonic delays of 1, 2, 4, then 8 seconds
(with a general 30-second cap); after five consecutive failures the ledger is not
automatically ensured again. Only a *recorded* worker failure (protocol
failure, crash, bootstrap error) counts: a worker that exits cleanly while its
tail still asks for a run — an answered hold, a queued input, a due wait — is
ordinary work, clears the backoff, and is re-ensured at once (an approval-heavy
turn restarts its worker many times without ever being a crash loop). An
explicit prompt, cancel, compact, or queue transaction clears that backoff as
well. One ledger's classification or spawn failure never aborts the sweep of
another session.

## Invariants

1. A clean `parked_hold` produces **no spawn from any actor** (R10-2).
2. An `answered_hold`'s ask call is never `aborted_by_crash`; it pairs from
   the answer (R11-1).
3. `stopped_active` dominates every non-running state (R7-1/R8-1).
4. Archive is allowed **exactly** where the matrix's archive column says
   "allowed": `settled` and `parked_hold`; every other state queues
   (R12-8 — this column is the single authority; D-3's "not-running"
   phrasing is narrative).
5. Writers evaluate this table under their own lock
   (`lock_held_by_caller`, rows 2–7); pre-lock evaluation is a hint (D-61).
6. File-scoped appends after a settle are legal and change nothing here
   (R12-1) — except that a `stop_requested` so appended creates a
   **born-closed** generation whose closure point holds earlier queued
   inputs (R13-1).
7. Closure is permanent: no later turn, input, or file-scoped event
   reopens a closed generation (R13-1).
8. An archived thread accepts no append from any actor; only unarchive
   restores matrix jurisdiction (R13-6).
9. A matching answer never overtakes an active stop. `stop-active` is a typed
   endpoint rejection that consumes no response identity; after recovery, the
   same response may be redelivered only if the ledger does not already
   carry the cancelled resolution (the aborting `tool_result`). A response held by
   `recovery_needed` follows the same unresolved-only redelivery rule.
