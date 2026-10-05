# Worker: startup, waiting and recovery

[Runtime · documentation home](../README.md) · [Execution flow](../flows/turn.md) · [Next: hosting and recovery](supervisor.md)

The worker is the process body: it owns exactly one line file for the duration
of one run, executes the loop, and exits. It has no knowledge of threads other
than its own folder, no registry, no RPC surface. One binary serves mainline and
children alike.

## Invocation

```
tekes-worker <path/to/line.jsonl> --config-fd N --instruction-fd M
```

- argv names only the line file and inherited read-only snapshot descriptor
  numbers. Config and instructions follow
  [config](../../spec/config.md) and
  [instruction-snapshot](../../spec/instruction-snapshot.md): the worker
  verifies both content digests and parses each once before taking the line
  lock; it never reopens live config/instruction paths. **Credentials via a
  distinct descriptor only — never argv, never ambient
  environment, never events** (D-46): the worker reads them into memory once,
  so spawned tools cannot inherit them.
- Every fd the worker opens — the flock fd included — is `O_CLOEXEC`, and tool
  spawn actions close-from (F8): no child can keep the worker's lock or
  descriptors alive past exec.
- The worker writes only this one file (plus `assets/` blobs and its own child
  files' genesis at spawn time). It never touches the manifest, other threads,
  or any global state.

## Startup sequence

1. Complete worker-control negotiation. A disjoint version exits 76 before
   opening snapshots, taking locks, or appending (D-58).
2. Read and validate the inherited config/instruction snapshot descriptors,
   verify the requested digests, validate project-source indices against the
   resolved cwd list, and derive the effective conservative policy. Descriptor
   absence is allowed only in the explicit injected Slice-1 test seam.
3. Take the folder lifecycle lock **shared** ([Thread definition](../concepts/thread.md)), then `flock(LOCK_EX)` the
   file — failure means another worker is live: exit immediately (the file
   lock *is* the lease). Revalidate path→inode after both ([Event definitions](../data/events.md)).
4. Torn-tail repair ([Event definitions](../data/events.md)).
5. Replay the longest-valid prefix from genesis and build the in-memory
   index (Event definitions §Replay); a `checkpoint` is a covering marker,
   never a replay seed.
6. **Derive the run mode under the lock** from the normative table in
   [spec/tail-lifecycle.md](../../spec/tail-lifecycle.md) (D-61; the spawn's
   mode is a hint only). Summary, first match wins: `stopped_active` →
   reconcile-only, always; **`answered_hold`** (a matching
   `approval_response`/denial exists, the ask call not yet result-paired —
   R11-1) → **ordinary, same-turn resume, regardless of `resume` policy**;
   `parked_hold` → this run should not exist: exit 0; runnable input →
   ordinary; `resume`/ordinal permit → ordinary; else reconcile-only.
   Runnable-ness, hold-permitted tails, and `queue_edit` non-triggering are
   defined there and only there. Then append + sync `run_start {run,
   mode, recovery_ordinal, binary, config_digest, instruction_digest,
   policy}` (R3-17) — before reconciliation, so recovery facts
   belong to the run that writes them; every later event belongs to this
   run by total order.
   **Management-recovery pre-step (worker-control §Negotiation):** when negotiation
   selected `startup:"queue-transaction"`, the next line was buffered before
   lock acquisition. Immediately after the `run_start` barrier, validate and
   execute that one transaction, return its non-coalescible result, and exit.
   This branch has priority over step 7 and bypasses generic reconciliation,
   turn opening, provider/tools, and settle. A missing/malformed preloaded
   transaction or failure before its result is fail-stop with no settle; the
   durable operation remains for a later candidate.
7. Reconcile the tail, **in this order** (R2-6):
   1. `spawn` without `child_result` → poll the child file's tail: child
      settled → synthesize the `child_result` idempotently; child live →
      adopt by polling (poll-not-wait — the child is no longer our OS child),
      do not abort it.
   2. Remaining unpaired non-spawn `tool_call` → append `tool_result
      {outcome: {aborted: "crash"}}` (event; "aborted_by_crash" in prose;
      side effect may or may not have run; the
      model decides on retry) — **excluding** calls named in an incomplete
      adopt inventory (R10-4, completed in step 3) **and the latest open or
      answered hold's ask call** (paired only by its response/answer path).
      Detached jobs
      reconcile from their disk state.
   3. Unpaired `attempt` → per the provider-adapter decision table
      ([spec/provider-adapter](../../spec/provider-adapter.md), R12-3): a
      marker-required adapter's markerless attempt is not-dispatched —
      `attempt_recovery {not_dispatched}`, close the attempt (a recoverable
      transport error carrying unavailable usage, one barrier,
      attempt_settled) — while markerless-legal adapters treat it as
      maybe-sent; dispatched/maybe-sent attempts take the table's
      query-result row, recording `attempt_recovery` before acting (D-43). A **reconcile-only** run may perform read-only
      query-by-identity but never resends (no new loop attempt — D-61/R6-2);
      when query is unsupported or fails, it settles the attempt with
      terminal `error {unresolved_dispatch, recoverable:false}` — billing
      recorded as unknown. An adopt `attempt_recovery` whose inventory is
      not fully event-ified (crash inside ②–⑤) is **completed idempotently
      here** (R9-4).
   4. Trailing `approval_request` → mode-specific (R7-5): an **ordinary**
      run resumes waiting; a **reconcile-only** run pairs the trailing ask
      call with `tool_result {aborted, reason: recovered}` and proceeds to
      its settle — never a wait; holds park, they do not settle (R9-2).
8. Establish an open turn **before any render/provider attempt**:
   - an existing unsettled turn (ordinary recovery or answered hold) resumes
     in place — no new `turn_open`;
   - an ordinary unstarted child appends `turn_open {trigger: "genesis"}`;
   - otherwise a runnable queued input appends the prefix-complete
     `turn_open {trigger: {inputs: [...]}}`;
   - the stopped-unstarted reconcile path has already opened and settled its
     cancel-before-start turn; any other state with no open turn and no
     runnable input exits 0.
   Only then enter the provider loop. This is the executable bridge from
   tail-lifecycle's mode row to event constraint 8.

## stdio protocol

JSONL both ways — the **versioned bespoke inner protocol** (D-58); the
client boundary is [Session Endpoint](../../spec/session-endpoint.md#routes-and-streams);
D-62 and [the historical Client mapping](../history/client-v2.md) retain the historical V2 mapping. Negotiation precedes lock acquisition and any append. First line each way:
`{"hello": {"proto": "tekes-worker", "min": M, "max": N}}`; the supervisor
selects the highest mutual version and replies
`{"selected": {"version": V}}`; the worker proceeds on V (currently 2). The
only extended selection shape is worker-control's management-recovery
`startup:"queue-transaction"`. No mutual
version → the worker exits with code **76 (EX_PROTOCOL)** having appended
nothing, and the supervisor reports incompatibility — never a silent
fallback. Golden wire fixtures live in `fixtures/wire/` ([Conformance gates](../verification/gates/README.md)).

- **stdin (control):** exactly
  [spec/worker-control](../../spec/worker-control.md): the base messages
  `input` (with `steer` flag; attachments staged first), `approval_response`,
  `queue_edit`, `stop`, `lease`, `launch_result`, `compact`, `meta`, `ping`,
  and the durable control messages (`tool_control`, the startup-preloaded
  endpoint `queue_transaction`, tool continuations), all defined at the one
  negotiated version.
- **stdin (control, read continuously — R2-8):** the reader thread takes
  every line off the pipe at once. `stop` sets the cancellation cause and
  stays on the channel for the loop's waits; deliveries (`input`,
  `queue_edit`, `meta`, `approval_response`, `queue_transaction`) are parked
  and never reach a wait that expects only `lease`, `tool_control_result`, or
  `launch_result`; the main thread appends and receipts them at its yield
  points — before each attempt's lease request, continuously while the
  provider request streams (the request runs on its own thread; the main
  thread forwards frames and services deliveries between them), after tool
  execution, and at the post-turn exit. A delivery is therefore receipted
  within milliseconds in every phase, never after the turn.
- **stdout (mirror + receipts):** `frame` (streaming delta) and
  `appended {seq}` (doorbell) — lossy, coalescible, unauthoritative. Accepted
  frames go to the transient presentation channel at once and to the durable
  storage lane behind it (session-endpoint §Stream-frame ingestion);
  the doorbell is only the fast path that projects the ledger into the
  endpoint journal — the exit path, the sweep, and boot re-project without
  it. `state {phase}` is reserved on the wire and neither emitted nor
  consumed. Plus
  one **non-coalescible** message class whose v1 members are `receipt {origin
  tuple, seq, deduplicated}` — the durable-delivery confirmation the
  supervisor binds to a specific delivery before acking its client (D-48) —
  and `attempt_settled {attempt, outcome seq}` — sent after the terminal
  output/error barrier, the sole lease-release signal (R3-2); a supervisor
  receiving it for an unknown or already-void lease treats it as an
  **idempotent no-op** (R8-3) — every attempt-terminalizing transaction
  emits it regardless of lease state, and it is never a protocol error.
  `queue_transaction_result` is also in this class;
  committed and rejected arms are both never merged, dropped, or replaced by
  doorbells.
  stderr: diagnostics only.

## Waiting and idling

One unified policy for every **hold** (approval, question to user): once
the `approval_request` barrier is durable, **exit 0 — no settle** (R9-2),
immediately and without waiting on stdin. The trailing `approval_request` is
itself the durable hold marker, the thread parks (not running, not settled:
tail-lifecycle `parked_hold`, which spawns nothing), and the answer's
delivery respawns an ordinary same-turn resume. Unrelated new input arriving
while a hold is open **queues behind the hold** and is consumed only after
the hold resolves (answer, or deny/stop). Every approval thereby exercises
the replay/respawn path — recovery never rots.

**Tail triage is one shared predicate** (R10-2/R11-5), normatively defined
in [spec/tail-lifecycle.md](../../spec/tail-lifecycle.md): running / settled /
stopped_active / **answered_hold** / **parked_hold** / recovery_needed.
Every actor — mode derivation, reaper, sweeps, ensure-running, delivery,
archive — uses that table; restatements anywhere are non-normative. A clean
`parked_hold` produces **no spawn** from any of them; an `answered_hold`
always spawns an ordinary same-turn resume.

Before shared tail triage, actors apply tail-lifecycle's external management
pre-gate: an incomplete endpoint queue transaction admits only the attributed
reconcile-only completion run; ordinary work and later stop/input delivery
wait for its committed or durably rejected transaction result.

An **empty queue at turn end is not a hold** (R3-5): the worker settles
`completed` and exits immediately — exactly one post-turn transition, no
linger, no second settle. The worker never waits on stdin after its turn:
its post-turn transition is derived once from the ledger and the
cancellation cause (settled or open hold → exit 0; stop without unresolved
work → `settle {interrupted, user_stop}`; stop with unresolved work → exit
nonzero without settle; an open turn with neither — provider lease denied
while the supervisor drains, no provider outcome — → exit without settle and
the tail classifies `recovery_needed`). A stop that arrived during the turn
is echoed as `stop_requested` with its receipt before the exit. Every
delivery that has already reached the process is appended and receipted at
that exit, as at every other yield point; a delivery still in flight in the
pipe is re-driven by the supervisor, whose receipt wait ends with the worker
and whose dead-target locked append owns it.
Holds never settle — they park (R9-2); the next
resolving answer, denial, or stop drives the next classified run, and v1
reconstructs the hold from the full log.

## Signals and exit

| Event | Behavior |
|---|---|
| `stop` (control message) | The graceful path: the worker cancels its foreground tool processes and provider request; no unresolved work → `settle {interrupted, reason: user_stop}` (R9-5; this ordinary settle also closes an active stop generation — R9-1), fsync, exit 0; unresolved work (live child workers, unpaired dispatched attempt) → exit nonzero **without settle** — the supervisor drives the cascade node-by-node (durable stop before deeper propagation, D-37), and a **recovery run** appends the missing terminal facts and the one settle (D-61/R3-6). Detached jobs and daemon backends stay alive |
| `SIGTERM` | The worker installs no signal handler: the process dies as on `SIGKILL` and the supervisor reconciles. The supervisor's `terminate_worker` (drain, cascade kill, failed worker) sends `SIGTERM`, waits one second, then `SIGKILL` |
| `SIGKILL` / crash | Nothing; supervisor reaps and reconciles ([Supervisor](supervisor.md)) |
| stdin EOF/HUP | Supervisor died (D-41). The control pipe is monitored **concurrently with every phase** — provider I/O and tool waits alike (R2-8/R3-7): EOF cancels immediately — abandon the HTTP attempt (left unpaired; its lease died with the supervisor), kill foreground tool children. From a clean hold: exit 0 — the hold parks (R9-2). With unresolved work: exit nonzero **without settle** — a recovery run appends all terminal facts, then exactly one settle (D-61/R3-6). The next supervisor sweeps and respawns |
| Budget exceeded | `settle {interrupted, reason: budget_tokens \| budget_wall}` (the exact exceeded budget — R9-5) and exit 0 |
| Provider terminal error | `error {recoverable:false, classification: provider_terminal}` + `settle {error, classification: provider_terminal}` (R11-4) — a terminal error is settlement evidence. **Live-loop only** (D-21 scoped): recovery-authored terminal attempt errors settle `{interrupted, reason: recovered}` (§Recovery transitions) |

Exit codes are hints only (0 = log self-consistent; nonzero = reconcile me).
**The supervisor always trusts the log over the exit code.**

## Recovery transitions (D-61, closed R7)

One table; no other recovery path exists. "Query" = read-only
query-by-identity where the adapter supports it.

| Tail state | Ordinary run | Reconcile-only run |
|---|---|---|
| unpaired `attempt`, no dispatch marker | settle it `error {not_dispatched}`, continue loop | same, then proceed to settle |
| unpaired dispatched `attempt`, query recovers the response | **adopt, in live-path order with a journaled commit point** (R9-4): ① sync the recovered response whole to an asset (D-45 reference protocol), then `attempt_recovery {decision: adopt, inventory: [call ids], response: {asset}}` — barrier, the self-sufficient commit point; completion never re-queries (R11-3) → ② `usage {reported}` → ③ the one recovered `output` (sealed fragments included; one barrier with ②; settles the attempt per D-43) → ④ `attempt_settled` → ⑤ per inventoried call: `tool_call` + `tool_result {aborted, reason: recovered}`, **never executed**. Ordinary continues (and may act on the aborted results) | same transaction ①–⑤, then settle `{interrupted, reason: recovered}` — never continues, never executes |
| unpaired dispatched `attempt`, query unsupported/failed | `attempt_recovery` then adapter three-way — **resend allowed here only** (Idempotency rules §Rule 3) | `usage {unavailable}` + `error {unresolved_dispatch, recoverable:false}`, **which closes the epoch** (R7-3: the possibly-advanced continuation is never reused; the next ordinary run opens a recovery epoch), then settle `{interrupted, reason: recovered}` |
| unpaired non-spawn `tool_call` | `tool_result {aborted_by_crash}`, model may retry | same, then settle |
| `spawn` without `child_result`, child settled | synthesize `child_result`, continue | synthesize, then settle |
| trailing `approval_request` | resume waiting | pair with `tool_result {aborted, recovered}`, then settle |
| after all rows | continue the loop | exactly one `settle {interrupted, reason: recovered}`, exit |

A reconcile-only run performs no new loop attempt; recovery launches are
**exempt from the stop spawn gate** (they are how a cascade terminalizes —
R7-1), and an active stop forces reconcile-only regardless of queued input.

**Stop-generation lifecycle (R8-1/R9-1/R13-1).** Generation G is *active*
from its durable root `stop_requested` until the turn open at its append
reaches its final settle (*closed*) — graceful-ordinary and recovery
paths alike; a G appended when **no turn is open** (terminal tail) is
**closed at birth**: there is nothing to stop, but inputs with seq < G.seq
are pre-closure, hence held. G's **closure point** is the closing settle's
seq (born-closed: G's own seq). Inputs predating the closure point are
**held**: visible in the queue projection, never run by the sweep. A
post-closure user action releases them — a **new keyed `input`** whose
`turn_open` consumes the held queue in seq order ahead of it
(prefix-completeness, event constraint 8 — R9-3/R14-2; queue edits map
to keyed supersedes of held inputs; [Client mapping](../history/client-v2.md)). Normative predicates: [spec/tail-lifecycle](../../spec/tail-lifecycle.md).
Stop means stop: nothing the user halted restarts without the user.

The no-open-turn case above means a **settled** file (including a parentless
turn-less root). A spawned, unstarted child is the explicit exception: its
generation remains open until the gate-exempt cancel-before-start run opens
turn 1 with `trigger: "genesis"` and settles it `interrupted/recovered`
(tail-lifecycle §Closure).

## Budgets, retry, compaction

Turn budgets (tokens, wall time, tool count) are worker-local counters from
config. Provider retry/failover lives inside the provider call; each attempt
appends an `error {recoverable:true}` event so the ledger is the retry history.

Compaction is worker policy (D-39): after the request is prepared and the
credential lease taken, before the provider admission lease and the `attempt`
event, the worker compares the prepared candidate's exact byte size — the
provider-runtime `candidate_tokens` upper bound, one token per transmitted
byte (`preflight_compaction_due`, `PreparedRequest.candidate_bytes`) — with
the model's `compact_trigger_tokens`; over it, with compactable history, it runs
the compaction summary request with the same attempt's credential material, writes the
checkpoint + `compact`, opens the `reason: "compaction"` epoch and re-enters
the loop (one compaction per turn, the same budget the overflow path uses).
Provider-reported usage does not feed this estimate. A provider
context-overflow error settles its `attempt` first (D-43), then compacts and
retries the same way; a manual compact arrives as a control message. The reader parks it until a yield point; the worker
writes a checkpoint and an origin-bearing compact event before acknowledging
it. Re-delivery returns the original sequence without another compact, including
after restart. Empty history still receives a durable no-op compact receipt.
A compact invalidates the old epoch; if it arrives after preparation but before
lease admission, the worker discards that request and rebuilds it first.

Provider concurrency is admitted per HTTP attempt in a **fixed order**
(R4-6): allocate the attempt id → acquire the lease from the supervisor over
the control channel (opaque admission class) → append/sync `attempt` →
append/sync `attempt_dispatched` where required → HTTP. A crash while queued
leaves no attempt event; a crash after the barrier leaves `not_dispatched`.
The lease is released **only** by `attempt_settled` or reap (F26). Spawn
never holds admission capacity.

## Compaction summary

Auto-compaction (`build_compaction_event`) — before a send whose prepared
candidate exceeds `compact_trigger_tokens`, or after a context overflow — runs
one bounded compactor request before the attempt's provider lease is released
(`run_compaction_summary`: `summary_artifact` as the only tool, the frozen
source bundle rendered at most at 60 % of the model window, 120 s) and admits
the artifact against the bundle (`engine::admit_summary_artifact`). An
admitted continuation is the compact summary; the verdict is the compact
event's `summary_request` record, and a failed request or rejected artifact
falls back to the deterministic quoted history. A manual compact applied at a
worker yield has no lease and carries no `summary_request`. Live proof:
`scripts/run-public-flow.py --live --case text --compaction` (four private
codes recalled across the supervisor-authored manual compact).
