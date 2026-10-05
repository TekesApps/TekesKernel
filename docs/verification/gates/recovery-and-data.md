# Recovery, data and context gates

[All gates](README.md) · [Running tests](../running-tests.md)

This page retains the gates' original numbers and applicable baselines. It defines conformance rules, not the results of this run.

## Supervisor lifecycle

12. ▸ **SupervisorTotalFailure** — Setup: streaming worker A
    (`resume: never`), tool-running B (`resume: bounded(1)`), mid-flight
    delivery D, live child C under A; fake ToolBackend and child
    ProcessHost harnesses are explicitly permitted. Action: SIGKILL the
    supervisor; boot a new one. Assert: A/B exit without settle (unresolved
    work); recovery runs per **D-61** — A gets a reconcile-only run (own
    `run_start`, 03's order: spawn edges → tools → attempts, one
    `settle {interrupted, reason: recovered}`), B gets an ordinary respawn
    that reconciles then continues; each run's mode is re-derived under the
    lock and recorded in its `run_start {mode, recovery_ordinal}`; A's
    dispatched attempt resolves by read-only query or terminal
    `unresolved_dispatch` — never a resend from a reconcile-only run, and
    the unresolved_dispatch **closes the epoch** (R7-3), and a
    query-recovered response **containing a tool call** adopts via the
    journaled transaction — `attempt_recovery {adopt, inventory}` barrier,
    then the one output carrying usage (one barrier) → attempt_settled → per-call
    `tool_call` + `tool_result {aborted, recovered}`, never executed;
    **crash inside the transaction at every boundary** is completed
    idempotently from the inventory on replay (R9-4); the
    supervisor authors no semantic fact; D dedups; drain respects its
    bounded deadline with per-file quarantine; no fact ever lands after a
    settle. `[Worker lifecycle §Startup/Signals; Supervisor §Reaper/Boot; D-60/61; R6-2]`
13. ▸ **BusyUnknownWorker** — Setup: worker surviving a supervisor restart,
    holding its lock. Action: new supervisor boots; a delivery targets that
    thread. Assert: the thread is busy-unknown/quarantined — never treated
    as dead, never locked-append delivered — until the holder exits; then
    normal delivery resumes. `[Supervisor §Process table; D-2; R4-2]`
14. ▸ **StopMidGrandchildCrash** — Setup: stop during subagent spawn with a
    grandchild racing **and a durable queued input on the stopped tree**.
    Action: crash the supervisor after root `stop_requested` durability,
    before the child's. Assert: the gate blocks all **work** launches while
    recovery-run launches pass (R7-1); every recovery run derives
    reconcile-only (stop dominates the queued input, which stays pending in
    the queue projection); re-drive makes each node durable before
    signaling past it; no descendant outlives the settled root; no signal
    crosses an undurable node. **Closure (R9-1)**: run the graceful variant
    too — a clean SIGTERM's ordinary `settle {interrupted, user_stop}` also
    closes G (first final settle with seq > G). **Post-closure (R8-1/R9-3)**:
    the sweep never runs the held input; only a new keyed input makes the
    held queue runnable, consumed in seq order ahead of it.
    `[Worker lifecycle §Recovery transitions/Stop-generation lifecycle; Supervisor §Spawn
    service; Turn flow §Spawning; D-37/61; R7-1/R8-1/R9-1/R9-3]`

## Rewrite and evolution substrate

15. ⬟ **RedactRewriteClosure** —
    Setup: thread with attempts, checkpoints,
    compact, sealed fragments quoting a secret, a poisoned asset, and a
    fork copy. Action: redact. Assert: output is referentially closed (no
    dropped-event references), sealed carriers dropped, only content-
    scanned assets carried, fresh epochs, fork audited separately; the
    secret is byte-absent from the replacement. `[Thread definition R7; Event definitions §Registry;
    D-40/50; R3-9/10]`
16. ⬟ **RewritePublicationCrashMatrix** — Setup: one fork and one redact
    operation, each with `staging/<op-id>/op.json` synced first. Action:
    crash at every phase boundary — build → validate → publish (rename +
    dir-sync) → then per type: fork close (record removed, **source
    untouched**) vs redact retire-source → close. Assert: sweep discovers
    each operation by enumerating `staging/*/op.json` and resumes its
    recorded phase idempotently; a fork can never lose or move its source
    (D-47); no discoverable partial folder; GC touches only record-less
    debris. `[Thread definition R7; Supervisor §Sweep; D-47/50; R5-1]`
17. ▸ **DowngradeEvolution** — Setup: newer writer appends upgrade `meta`,
    a new `state` subkind, and a `min_reader`-gated kind. Action: old
    worker reopens. Assert: generic rendering of the state subkind;
    read-only fail-closed on the gate; zero provider sends that omit
    visible facts. `[Event definitions §Envelope; Model context §Visibility; D-49]`
18. ▸ **UpgradeGateBarrier** — Setup: writer appends upgrade `meta` then a
    dependent event. Action: power-loss between them (barrier removed in a
    fault-injection build must fail this test). Assert: the dependent event
    is never durable without its gate; an old writer can never append past
    a durable gate. `[Event definitions §Durability; D-45/49; R4-3]`
19. ▸ **CanaryPerRunAttribution** — Setup: same line file run under old
    then new binary. Action: respawn across versions; crash mid-recovery
    once so a reconcile-only run performs the repair. Assert: every event —
    recovery facts included — attributes to the `run_start` of the run that
    wrote it; the supervisor wrote none of them (D-61). `[02 run_start;
    Worker lifecycle §Startup; Supervisor §Reaper; D-61]`
20. ◆ **InstructionSnapshotRace** — Setup: two folder bindings with distinct
    `.agents/` entries (and a separate same-name-skill rejection case). Action:
    create a session on the second binding and mutate instruction files during
    spawn.
    Assert: the worker receives one immutable content-addressed snapshot
    (deterministic authored folder order, project>user, conservative meet),
    both project roots remain captured, and only the execution cwd selects
    the second binding; its digest is in `run_start`; mid-run edits change
    nothing until the next spawn. A legacy multi-folder genesis without a
    binding rejects before launch.
    `[Thread definition §Instruction plane; D-55; R3-20]`
21. ◆ **InstructionNextLaunchOnly** — (rewritten from R3's
    InstructionSelfEditGate to the repaired D-55 semantics.) Setup: agent
    edits a project skill/hook via ordinary tools. Action: continue the
    current run; then respawn. Assert: the live run's snapshot is
    unchanged; the next spawn's snapshot includes the edit and opens a
    named epoch. `[Thread definition §Instruction plane; D-55; R3-16]`

## Epoch and admission

22. **OverflowPendingInventory** — Setup: mid-turn steer S admitted-in-
    flight, withheld input N. Action: provider overflow → settle attempt →
    compact → epoch → retry; then complete the turn. Assert: S re-renders
    exactly once after the failed attempt, N never renders before its turn,
    both retain original seqs via the epoch's pending inventory.
    `[Model context §Epochs/Admission; D-29/39; R3-8]`
23. **SupersedeAdmittedOutput** — Setup: admitted output O. Action: retry
    superseding O. Assert: named epoch (`rollback`), post-supersede
    snapshot excludes O, prefix stability holds within each epoch.
    `[Model context §Supersedes; D-30]`
24. ⬟ **StagingOwnershipGC** — Setup: live rewrite operation (its `op.json`
    still present at `staging/<op-id>/op.json`, payload already published)
    + orphaned staging debris with no record. Action: boot GC + sweep.
    Assert: debris is collected; the live operation is discovered via
    `staging/*/op.json` and resumes at its recorded phase.
    `[Thread definition R7; Supervisor §Sweep; D-50; R3-24]`

## Format and protocol gates (added R5)

28. ▸ **CanonicalEnvelopeFixtures** — Setup: the `fixtures/events/` corpus —
    one named fixture per core-kind row of 02 (`effective_execution`
    included; `queue_release` does not exist — R9-3), turn-bound and
    file-scoped both. Action: round-trip every
    fixture through the schema library. Assert: byte-identical **RFC 8785
    (JCS)** re-encoding; `turn` present exactly on turn-bound kinds;
    big-int strings and ISO timestamps enforced. `[Event definitions §Envelope/Core kinds;
    D-18; R4-5; R6 blocking 4]`
29. ▸ **TornTailFaultInjection** — Setup: files ending in every torn shape
    (half a line, invalid UTF-8 tail, bare LF, and an invalid line followed by
    a valid-looking persisted line — `interleaved-loss`). Action: open under
    the lock; also open read-only. Assert: the lock holder truncates to the
    longest valid prefix before allocating the next seq; readers ignore the
    entire suffix after the first invalid point; nothing else changes.
    `[event §Tail framing; Event definitions §Durability/Concurrency]`
30. ▸ **CheckpointMarkerReplay** — Setup: file with checkpoint K, then
    a supersede touching K's covered range. Action: replay from genesis.
    Assert: the complete fold accepts both ledgers, K stays a plain covering
    marker, and the dedup key map is rebuilt from the full prefix. No
    implementation skips covered events on K's authority.
    `[Event definitions §checkpoint/Replay; Idempotency rules §Rule 2]`
31. ▸ **ProtocolNegotiationReject** — Setup: `fixtures/wire/` cases
    `hello-ok`, `hello-disjoint`: exact `{"hello":{...}}` /
    `{"selected":{...}}` frames. Action: spawn; handshake precedes lock
    acquisition and any append. Assert: highest mutual version selected and
    echoed; disjoint ranges → worker exits **76 (EX_PROTOCOL)**, the
    supervisor reports incompatibility, and the thread file is untouched.
    `[Worker lifecycle §stdio; D-58; R6 blocking 4]`
32. ▸ **ResumePolicyEncoding** — Setup: genesis/spawn fixtures with
    `resume: "never"` and `{"bounded": 2}` (the field is required —
    absent is invalid, R14-7). Action: crash a worker
    under each repeatedly; recover per D-61. Assert: absent rejects; each
    recovery `run_start` carries `mode` and an incremented
    `recovery_ordinal`; bounded exhausts at ordinal ≥ n into reconcile-only
    (replay-checkable from run_starts alone); the field is immutable and
    the child's genesis inherits the spawn's declaration.
    `[02 genesis/spawn/run_start; Turn flow §Spawning; D-60/61]`

33. ▸ **RunModeArbitration** — Setup: unsettled `resume: never` tail;
    concurrently, the sweep spawns a reconcile-only candidate while a keyed
    delivery appends new input and spawns an ordinary candidate. Action:
    race the flock both ways. Assert: exactly one winner; the winner
    derives its mode **under the lock** — with the durable input present it
    runs ordinary regardless of its spawn hint, the input is processed
    (never stranded), and the loser exits 0; run_start records the derived
    mode. `[Worker lifecycle §Startup; Idempotency rules §Rule 1; D-61; R6-1]`

34. ▸ **SlowHoldLifecycle** — Setup: ask-user hold parking at its `approval_request` barrier.
    Action: park (power-loss at the `approval_request` barrier boundary in
    one variant); let the reaper and sweeps observe the parked thread;
    deliver a non-answer input; then the answer; crash separately at answer
    append → `approval_response` barrier → receipt; after the resumed turn's
    provider/tool work, allow the queued input to start the next turn.
    Assert: the request is
    durable before any frame or idle exit (R10-1); reaper/sweep/ensure
    spawn nothing for a clean park (R10-2); the non-answer input queues —
    at most one bounded respawn-park cycle, input not consumed; the answer is
    never acknowledged before its barrier and a retry re-acks its original
    seq; it
    forces the `answered_hold` state and an ordinary same-turn resume
    regardless of `resume` policy (R11-1), pairs the call — never
    `aborted_by_crash` — and the turn reaches exactly one final settle
    **before** the queued input's prefix-complete next `turn_open`; the
    resumed attempt never admits that queued input. v2
    emits requested → (no turn/end while parked) → resolved → the same
    turn's remaining events → one `turn/end`. `[spec/tail-lifecycle;
    Event definitions §Durability; Worker lifecycle §Startup; Supervisor §Reaper; Tool execution §Ask-user; Client interface §Div. 4]`

## Config and instruction gates (Slice 2)

35. ◆ **ConfigContractFixtures** — Setup: the canonical and invalid
    `fixtures/config/` corpus. Action: decode/re-encode every valid authority
    document and reject every invalid one. Assert: canonical bytes are
    identical; closed fields, safe integers, absolute paths, secret-material
    exclusion, and transport-shape constraints are enforced. `[config
    §Authority/§Closed schemas]`
36. ◆ **ConfigAtomicPublishRevocation** — Setup: one workspace plus all three
    global config files and two concurrent writers at the same expected
    revision. Action: publish both, resolve a launch snapshot, then reduce
    network/writable-root privilege. Assert: exactly one writer wins, the
    loser gets `stale_revision`, the old snapshot asset remains immutable,
    and the reduction requires worker respawn. `[config
    §Publication/§Resolved spawn snapshot/§Revocation]`
37. ◆ **InstructionOracleRejections** — Setup: the committed two-level tree,
    canonical snapshot oracle, malformed settings/snapshots, and a symlink.
    Action: capture twice and validate the stored projection. Assert: source
    order, content hashes, conservative policy meet, shadow precedence, and
    effective maps equal the contract fold byte-for-byte; dangling/forged/
    reordered snapshots and symlinks reject. `[instruction-snapshot
    §Inputs/§Snapshot schema/§Stable capture]`

## Checkpoint marker gates (Slice 3)

38.–41. Retired (2026-09-06): checkpoint acceleration — state assets,
    digest preimages, seeded replay, invalidation fallback and origin-key
    retention floors — was removed; a `checkpoint` is a plain covering
    marker and every reader folds from genesis. The numbers are not reused.
42. ◇ **CheckpointMarkerBarrier** — Setup: a valid ledger and an
    instrumented sync policy. Action: create a checkpoint and reopen the
    ledger. Assert: the checkpoint covers the whole prior prefix, is one
    barrier (exactly one full sync), names no state asset or key map, and
    reopen folds from genesis through it. `[event §checkpoint; D-45]`
