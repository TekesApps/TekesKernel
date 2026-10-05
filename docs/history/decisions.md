# Design decision records

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

Protocol succession: D-62 and D-68 record the V2 boundary decision. The current public
surface is [Session Endpoint v3](../../spec/session-endpoint.md), which supersedes V2
list/history, dual WebSocket and respond routes. V2-named durable carriers and the
separate worker-control version remain meaningful. Decision IDs and historical rationale
are preserved; current process/source mapping is in [Code architecture](../architecture/README.md).

Numbered, terse, with the one-line why. Amend by appending, not editing.

Current terminology is consolidated in [Thread fundamentals](../concepts/thread.md#thread-fundamentals-a).
The original process/log reduction does not identify a model final with a turn
settlement, or a session with one OS process. Current root validation behavior
is in [Turn flow](../flows/turn.md#turn-execution-and-validation) and the provider/event
contracts; historical rationale below is not a parallel current state machine.

## Settled

- **D-1. Five named authority classes; everything else is derived and rebuildable**
  *(amended R1/R6/Slice 6)*. Authorities: durable append-only logs — semantic
  thread ledgers (all thread history) and closed protocol-carrier journals
  (externally exposed wire identity/bytes and D-70 exact-retry receipts that
  cannot be reproduced) —,
  the file lock (liveness; the process table is its routing cache), user/config
  authorities (workspace files,
  settings, skills, schedules), secret stores (keychain), and the external
  world. The thread ledger is sole truth *for semantic thread history*; a
  protocol carrier is authoritative only for its versioned wire contract and
  is forbidden as worker replay, recovery, admission, or provider input.
  Every index, manifest, mirror, and cache must be deletable and rebuildable
  from its authority; a derived artifact that becomes load-bearing without
  being a closed named log recreates the second-store-plus-sync-seam bug class.
  Config snapshots/digests flow into
  `genesis`/`epoch` so runs are interpretable after config drifts.
  The named per-thread protocol carriers currently include
  `endpoint-v2.jsonl`, `endpoint-requests-v2.jsonl`, endpoint-management-v2
  rpc and crash-recoverable operation records, and D-70 control receipts;
  a new carrier requires an executable schema plus rewrite and GC duties.
- **D-2. Two questions, two sources** *(amended R4)*. Running? → **the file
  lock** (liveness authority); the process table is owned-worker routing
  cache with an explicit busy-unknown/quarantined state for locks the
  supervisor does not own (R4-2). Happened? → log. The log never contains
  "running"; the table never contains history.
- **D-3. `terminal_recorded` = settle event durable; `settled` =
  terminal_recorded ∧ no live lock holder** *(amended R1: named halves)*.
  Either half alone is torn state, detectable and reconcilable; UI settled-
  ness keys on `settled`, never on the event alone. Archive eligibility
  is spec/tail-lifecycle's archive column (settled and parked_hold —
  R10-5/R12-8); an archived thread rejects every delivery with a typed
  `archived` error until explicit unarchive (R13-6). *(Amended, lifecycle
  audit 2026-09-03:)* the lock half is a **bounded commit window**, not a
  UI state: the worker exits right after its settle (03 R3-5) and the sweep
  projects a settled tail within one tick, so `terminal_recorded ∧ live
  lock` lasts milliseconds and the sweep, the exit path, and boot detect and
  repair the projection half. The Client therefore keys turn presentation on
  the projected `terminal_recorded` (endpoint `turn/end`) and thread liveness
  on the classified tail (`SessionSummary.tail`, lock ∨ registered worker);
  it never derives "settled" from the event and the lock separately.
- **D-4. Only the worker, or the supervisor holding the lock, authors events.**
  "Whoever holds the flock writes" is the entire write-coordination story.
  Mirrors and clients never write.
- **D-5. File = line of work; process = one run.** Successive processes relay-
  append one file. "Session" is not a concept.
- **D-6. Root is a position, not a type.** All files schema-identical, one
  worker binary; parentage lives in events.
- **D-7. Seeds are materialized locally, never referenced back to the
  parent.** The child genesis references its own content-addressed seed
  snapshot asset; the asset contains canonical source-event lines whose seqs
  are provenance only (event-v1 §Seed snapshot asset format). Redundancy buys
  self-containment: the child folder replays and moves alone.
- **D-8. Cursors and references are `seq`, never byte offsets.** Offsets are
  manifest acceleration only. Leaves segmentation open without migration.
- **D-9. Streaming partials never touch disk.** Reattach is served from the
  supervisor's in-memory ring buffer; disk gets settled output only.
  *(Amended R17:)* the scope is the kernel **thread ledger**: partials never
  become thread events. Whether the endpoint projection durably retains or
  reproducibly materializes chunk identities for v2 history is that
  projection contract's obligation (15) — never a license to write partials
  into any thread file.
- **D-10. Cross-folder writes go through the supervisor only.** Delivery: alive
  → stdin; dead → locked append. Input is made durable before any spawn is
  attempted.
- **D-11. Assets are content-addressed per-folder blobs.** No inline binaries;
  no cross-thread dedup (self-containment > dedup); fork uses clonefile where
  the volume supports it, verified copy+sync elsewhere *(amended R1)*; GC by
  reference scan.
- **D-12. One idle policy for every hold** *(amended R3/R9/R11 — holds
  park, never settle)*. Exit 0 with **no settle** as soon as the
  `approval_request` barrier is durable *(amended, lifecycle audit
  2026-09-03: there is no idle timeout and no waiting on stdin — a parked
  hold has no live worker, per tail-lifecycle's `parked_hold` row)* — the
  `approval_request` is the durable hold marker (a
  barrier, R10-1) and the thread parks; an answer creates the
  `answered_hold` state and an ordinary same-turn resume (R11-1); unrelated
  input queues behind the open hold. The normative states and actions are
  [spec/tail-lifecycle.md](../../spec/tail-lifecycle.md). An empty queue at
  turn end is not a hold: settle `completed` and exit immediately — exactly
  one settle per turn (R3-5). Recovery is exercised daily by design.
- **D-13. Create before register.** Child file (fsync) precedes the parent's
  `spawn` event; the only possible residue is a harmless orphan file.
- **D-14. `supersedes` is first-class; rendering always goes through a
  projection.** Raw-tail rendering is forbidden; retry/rollback/compact retract
  by event, never by editing history.
- **D-15. Archive = mv under lock.** A live thread returns typed
  `session-running`; archive never implicitly stops or waits for it. Rename is
  metadata (folder names never change). The original ULID identity detail is
  superseded by D-64 before implementation.
- **D-16. Storage root is local and never iCloud-synced** *(location amended
  by D-72 from Application Support to `~/.agents`)*. Boot asserts this. Backup
  = Time Machine; interchange = copy the folder.
- **D-17. Daemon pool only for resources that cannot be multiply opened**
  (browser, computer-use, interactive-OAuth HTTP MCP). Everything else is a
  worker-owned child or in-process call.
- **D-18. Big integers are strings; timestamps are ISO8601 strings.** The 2^53
  and epoch-parsing lessons, made schema law.
- **D-19. Append-before-execute for side effects.** The fsynced `tool_call` is
  the write-ahead intent; replay reconciles unpaired intents as aborted, never
  re-executes blindly.
- **D-20. Rotation deferred, enabled.** No file segmentation now; `seq` +
  checkpoints keep it a later, migration-free change.
- **D-21. Terminal errors settle** *(scoped R7)*. In the **live loop**,
  `error {recoverable:false}` plus `settle {error}` is a legitimate outcome;
  recovery-authored terminal attempt errors instead settle
  `{interrupted, reason: recovered}` (03 §Recovery transitions). "Runtime
  says running" (a live process) vetoes settlement, nothing else does.
- **D-22. Unknown event kinds render as fallback, never drop.** Forward
  compatibility is a reader obligation.
- **D-23. Control-flow machinery is banned.** No action queues, no state-machine
  objects in the worker: the program counter is the state. If control flow
  needs to survive a crash, it must be derivable from the event tail (else add
  an event kind, not a mechanism).
- **D-24. Supervisor core stays under a few-thousand-line cap and holds no
  truth** *(amended R2)*. The cap binds the core (singleton, spawn/reap,
  delivery+receipts, stop reconciliation, admission); bundled services
  (schedule, catalogs, ring-buffer mux, daemon brokers) are contract-isolated
  behind files + the control protocol and individually evictable into
  separate processes. Core growth beyond the 04 list is harness regrowth —
  an architecture bug; a growing bundled service is evicted, never absorbed.
- **D-25. Exit codes are hints.** The supervisor trusts the log over the exit
  code, the mirror, and its own memory.

- **D-26. Epochs are events.** An `epoch` event freezes the SystemProfile,
  model/adapter identity, and startup tool set, with a named reset reason from
  a closed vocabulary; render always starts at the latest epoch, and earlier
  events reach the model only through its snapshot.
- **D-27. The provider projection is head + baseline + new events, and within
  an epoch the projection of a prefix is a prefix of the projection.**
  Prefix-stability is law, not an optimization; changing the renderer is an
  epoch.
- **D-28. Model-visibility is a schema dimension.** Kind defaults plus a
  per-event override; unknown kinds are runtime-only, so new event types can
  never leak to a provider through an old renderer.
- **D-29. Admission is explicit; coverage is the union of admitted ranges**
  *(rewritten R1 — the one-integer claim was wrong)*. Provider-born events
  (output, provider-emitted tool_call) are admitted at birth and never resent;
  host-born events are admitted by the `admits` seq-ranges on the `attempt`
  that rendered them; cumulative coverage = the union over attempts **settled
  by a durable output** — error/unknown settlements contribute nothing, so a
  failed send leaves its events unadmitted for the retry *(amended R2)*. Steered/queued inputs and post-output tool events are exactly
  the cases a single watermark cannot express.
- **D-30. Supersede over provider-admitted ranges closes the epoch; over
  un-admitted ranges it is free.** Appending cannot un-send bytes; retraction
  moves the conversation to a new epoch instead of editing history.
- **D-31. Compaction always closes the epoch, and anchors survive it
  verbatim.** The next epoch's snapshot is summary + anchors + durable facts;
  anchor-hood is a kind/event property, the structural fix for compaction
  semantic loss.

- **D-32. Ensure, never start.** Liveness operations are idempotent
  ensure-verbs arbitrated by the flock (losers exit silently); the process
  table caches the outcome and never arbitrates it.
- **D-33. Every externally-originated mutation carries an `origin_key`,
  deduped as a namespaced tuple** *(amended R1)*: (principal, client, target,
  operation, key). Conditional append under the lock drops duplicates and
  acks the original seq — at-least-once delivery, exactly-once effect.
  Retention is contractual: checkpoints carry the active key **map**
  (tuple → original seq) and expiry floor, so replay never loses dedup
  authority and duplicates re-ack their original seq *(amended R2)*.
  Successor of requestToken.
- **D-34. Attempt-before-send for provider calls** *(recovery shorthand
  superseded R8)*. An fsynced `attempt` event precedes every HTTP send; the
  baseline advances only on durable output. Recovery of unpaired attempts is
  governed by D-43 and 03 §Recovery transitions — resend exists only in
  ordinary runs.
- **D-35. Debounce only on watermarks.** Doorbells, manifests, rollups, and
  mirror pushes coalesce freely (high-water semantics); truth-path appends
  are deduplicated, never debounced.

- **D-36. Workspace is a config file; threads bind it in `genesis`.** One
  workspace = many threads and many working directories. The definition
  (`workspaces/<id>/workspace.json` beside `threads/`: name, stable folder
  bindings/current paths, policy) is
  user-authored config — the second legitimate truth class, supervisor-written,
  atomic-swapped, never derived. The thread's binding is a `genesis` field
  (rebind via `meta`); the worker receives the resolved cwd/writable-roots set
  as spawn config, and the resolved snapshot's version+digest is recorded in
  `genesis`/`epoch` *(amended R1)*. Benign edits take effect at the next spawn
  against a precisely recorded version; privilege reductions (removing a
  writable root, revoking a capability) force stop/respawn of affected
  workers; a change that must apply mid-epoch is an epoch reason (11).
  Resolution is atomic with launch: the stamped version is the contract, and
  an edit racing a resolved-but-unlaunched spawn lands as the next version —
  reaching that worker only as revocation or at its next spawn *(amended R2)*.
  The executable authority/publication/snapshot bytes are
  [config-v1](../../spec/config.md).
- **D-37. Stop cascades along spawn edges: whoever spawned it stops it.**
  The supervisor drives the cascade node-by-node; a worker on SIGTERM acts
  locally only — kill its foreground tool processes, settle `interrupted` if
  no work is unresolved, else exit without settle for the supervisor to
  reconcile (R3-6). SIGKILL escalation after timeout, then reconcile-on-reap. *(Amended R1/R2:)* all worker launches are supervisor-only, so the
  spawn gate has exactly one enforcement point; stop sets the in-memory gate
  there first, then makes `stop_requested` durable in each file (keyed
  delivery + receipt of the echo, or kill + locked append) **before
  propagating past that node or acking the stop** — a supervisor crash
  mid-cascade re-drives from any durable stop via the sweep, which
  recursively reconciles the durable spawn graph before terminalizing the
  root; no descendant outlives a settled root. *(Amended R8/R15:)* closure is
  tail-lifecycle's executable predicate: the turn open at G's append
  settling closes G; on a settled file G is born-closed; on an unstarted
  child G stays open until the cancel-before-start settle (R9-1/R13-1/
  R15-2); held pre-closure inputs ride only a post-closure trigger's
  `turn_open` (prefix-completeness — R9-3/R14-2; `queue_release` deleted;
  queue edits are keyed supersedes). Detached jobs and daemon-pool backends are deliberately
  outside the cascade.
- **D-38. Steer injects into the current turn.** A steer is an `input` event
  appended mid-turn and consumed at the next yield point (same as today);
  non-steer input waits for the turn boundary. Events append in arrival
  order; rendering gates may withhold, never reorder.
- **D-39. Compaction trigger is worker policy, unchanged from today**
  *(amended R1 — usage alone cannot judge the candidate request)*. Before
  each send: a conservative adapter-aware preflight estimate of the rendered
  candidate (the renderer knows its exact bytes) vs the config threshold →
  compact; provider-reported usage is calibration for that estimator, not the
  decision input. A provider context-overflow error first settles its
  `attempt` (D-43), then compacts and retries; manual compact arrives as a
  control message. Mechanism stays D-31.
- **D-40. The deletion unit is the thread folder.** Archive = mv, delete =
  rm of the folder. Finer-grained erasure (a secret pasted into a turn) is
  never an in-place mutation: it is an explicit offline **redact rewrite** —
  the materialized rewrite projection of D-50 *(rewritten R4; shared with
  fork, D-47)* minus the poisoned events, with affected sealed carriers
  dropped (fresh epochs make them unnecessary) and only content-scanned live
  assets carried; derived summaries embedding the secret are regenerated or
  dropped; then the original is retired. Fork-copies are audited and
  redacted separately. Endpoint projection and request journals are scanned
  but never copied; fork/redact rebuild them under the destination identity so
  no secret frame bytes or source wire ids survive. Root-scoped endpoint rpc
  and operation records are never copied; redact replaces each secret-bearing
  complete authority with endpoint-management-v2's content-free retired
  tombstone, so exact retry cannot resurrect erased bytes. No remap language
  survives — the 02 registry is the contract. Slice 5 ships the explicit
  operation under D-65.

- **D-41. One supervisor per storage root; workers are supervisor-bound.**
  The supervisor holds an exclusive lock on the root (two supervisors are
  structurally excluded). A worker treats stdin EOF/HUP as supervisor death:
  exit — a clean hold parks with no settle (R9-2); with unresolved work it
  exits **without settle** and recovery writes the facts before the one
  settle (R3-6). A restarted supervisor never reconstructs pids,
  pipes, or reap authority — it sweeps and respawns.
- **D-42. Stateless replay uses sealed native fragments.** `output` carries a
  bounded, versioned, adapter-owned native carrier (exact provider items,
  opaque reasoning signatures; asset-spill when large) beside the normalized
  content. Replay renders sealed fragments verbatim; an absent or
  version-incompatible carrier fails closed into a recovery epoch — never
  silent re-normalization.
- **D-43. Attempts are durably settled, and dispatch is durably marked**
  *(amended R2 — the "same unknown state" claim was reversed by review)*.
  Every `output`/`error` names the attempt it settles; exactly one terminal
  settlement per attempt. Non-idempotent server-managed adapters fsync an
  `attempt_dispatched` marker before HTTP (the predecessor's
  transportHandoffStarted): an unpaired attempt **without** it settles
  locally as `error {not_dispatched}` — nothing left the host, retry is
  free; **with** it, the adapter's three-way recovery applies (query by
  identity / recovery epoch / resend), and the chosen decision is recorded
  as a durable `attempt_recovery` event before acting, so a crash during
  recovery cannot strand two ambiguous attempts. Stateless adapters may
  skip the marker — resend is safe regardless. *(Amended R6:)* a
  reconcile-only run may take only the read-only arm (query-by-identity);
  resend belongs to ordinary runs; when query is unsupported or fails, the
  D-43-accepted terminal outcome is `error {unresolved_dispatch,
  recoverable:false}` with billing recorded as unknown. *(Amended R3:)* the terminal
  output/error is a barrier — baseline advance and lease release wait for
  its sync, and `attempt_settled {attempt, outcome seq}` on the
  non-coalescible stdout class is the sole lease-release receipt (R3-2).
- **D-44. Spawn↔call binding.** `spawn` and `child_result` both carry the
  originating tool-call id and a stable spawn id; exactly one terminal
  `child_result` per call id, enforced at append.
- **D-45. Durability barriers are explicit.** Checked full-write loops;
  reference protocol = write → file sync → rename → parent-dir sync →
  referencing-event sync. Barrier events — settle, `attempt`,
  `attempt_dispatched`, `attempt_recovery` (adopt commit point — R10-4),
  `approval_request` (parked-hold marker — R10-1), `approval_response`
  (ack-bearing answer — R16), `queue_edit` and
  `stop_requested` (ack-bearing controls — R12), manually-originated
  `compact` (ack-bearing — R13-4), externally-originated `meta`
  (rename/labels — R17k), acceleration-capable `checkpoint`, terminal output/error
  settling an attempt (preceded by their `usage`, one barrier committing
  both — R4-1), side-effect intents, `effective_execution`, `spawn`,
  `genesis`, `run_start`, upgrade `meta` (R4-3), ack-bearing inputs — use
  F_FULLFSYNC on macOS; plain fsync only where loss means re-run. This
  list and 02's are one list (R3-2). Intent-sync failure → the side
  effect is not executed; no acknowledgement before sync; persistent append
  failure → fail-stop worker, degraded read-only thread, reserved emergency
  capacity for terminal diagnostics.
- **D-46. The secret boundary ends before tool spawn.** Provider credentials
  reach the worker only through the inherited private descriptor channel in
  [credential-broker-v1](../../spec/credential-broker.md) — never ambient environment, anywhere;
  tools spawn with an allowlisted clean environment; tool output passes a
  heuristic secret scan before committing (defense-in-depth — the
  environment rules are the primary defense). The scan is a **total
  transform** *(amended R2)*: it always yields a terminal `tool_result`
  (clean / redacted / withheld), never an unpaired intent. *(Amended R3,
  R2-7:)* the precise env rule: secrets never enter the worker's environment
  and are never ambiently inherited; a **declared** integration's own child
  process may receive named entries injected at its spawn (targeted,
  per-declaration, audit-logged) — no other process sees them.
- **D-47. Copies are forks; moves are transfers.** A thread folder copied
  without ownership transfer is a fork: new identity via the **materialized
  rewrite projection of D-50** *(amended R5 — no remap language)*, with
  provenance and fresh epochs; a fork never touches its source. A move is a
  quiescent ownership transfer with a generation bump recorded in `meta`;
  two live same-identity copies are a spec violation, not a merge problem.

- **D-48. Delivery receipts are correlated and non-coalescible.** The worker
  confirms each durable delivery with `receipt {origin tuple, seq,
  deduplicated}`; doorbells stay coalescible watermarks. Only a receipt
  authorizes a client ack — a doorbell says the file grew, a receipt says
  *which* mutation became durable.
- **D-49. Format evolution is gated, and downgrade is read-only.** Effective
  reader/writer minima = genesis ∨ the latest upgrade `meta` event; events
  may carry `min_reader`. An upgrade `meta` is a mandatory barrier before
  any event governed by the raised minima (R4-3). A worker below the gate —
  or facing a gated visibility-bearing event it cannot interpret — renders
  and serves but never appends and never runs the provider loop. Forward-visible facts ride
  the `state` envelope so old readers render them generically instead of
  hiding them.
- **D-50. A rewrite is a materialized projection published through staging**
  *(amended R3/R6; executable in Slice 5)*. The exact projection, closed
  operation schema, source-prefix binding, carrier closure, type-specific
  phase machines, crash inference, retirement, and rewrite-only GC are
  [rewrite-publication-v1](../../spec/rewrite-publication.md). Fork closes
  without touching its source (D-47); redact removes its source only after a
  validated destination is durably discoverable. No unrecorded staging object
  can authorize source mutation.

- **D-51. The seam budget is five traits and three wire protocols.**
  `ProcessHost`, `ToolBackend`, `ProviderAdapter`, `SecretStore`, `Platform`
  (clock/entropy/sync policy for deterministic tests); the worker stdio
  control protocol, the client RPC, the hook contract (08). A trait exists
  only where a second implementation is committed. Storage, the event
  schema, and the supervisor are deliberately concrete — a storage trait
  would recreate the dual-backend complexity this design deletes. Adding a
  seam is a decision, not a refactor.
- **D-52. Secrets are a pluggable supervisor-held store; config references
  them by key id only.** Keychain is one backend (Security.framework C API —
  language-independent), not the architecture; Linux uses secret-service /
  systemd credentials / an encrypted file. Values reach workers by
  descriptor (D-46). Format 1 omits integrations that name `credential_env`;
  no value reaches an integration environment until a closed delivery contract
  exists. Global non-secret config
  lives in `config/*.json` — user-authored authority, supervisor-written,
  versioned into spawn snapshots; probe results live in deletable `cache/`.
  Exact result/record/Keychain identity and rotation semantics are owned by
  [secret-store-v1](../../spec/secret-store.md); the endpoint bearer is a
  separate credential authority.

- **D-53. Evolution rides external authority; the kernel is evolvable, not
  self-evolving** *(rewritten R3 — the "no new mechanism" claim was false)*.
  Evolvable surfaces are immutable artifacts under config/code authority —
  never liveness, secret values, thread truth, or the external world
  (R3-23). Their version store is git; candidate isolation is
  branches/worktrees outside the storage root; promotion, canary routing,
  and evidence binding are deployment concerns. The kernel contributes the
  substrate: `run_start` attribution by total order (R3-17), materialized
  instruction snapshots (R3-20), and the D-49 downgrade gates. Fitness stays
  a named projection over usage/settle events, joined through `run_start`.
- **D-54. The constitution is externally pinned** *(amended R3)*. Validation
  runs against a separately selected, immutable constitution version outside
  any candidate's write authority — a change cannot be judged by rules it
  relaxed. Constitution changes ship separately with human approval.
  Supervisor binary activation and last-known-good fallback belong to the
  external launcher (launchd/systemd), not to the kernel (R3-18/19).

- **D-55. Three-scope instruction plane, standard names** *(storage and
  collision semantics amended by D-72/D-73)*. User level `~/.agents/`;
  workspace level `~/.agents/workspaces/<id>/`; project level
  `<cwd>/AGENTS.md` + `<cwd>/.agents/` per workspace folder (legacy `.agent/`
  is fallback-only). The file
  is `AGENTS.md` — the open standard, for cross-tool interop. All levels
  load at spawn, labeled by origin; policy conflicts merge conservatively and
  same-name skills across scopes reject instead of overriding. Instruction content digests into the SystemProfile,
  so an instruction change is a named epoch (system-source change). Nested
  per-subdirectory files and managed/enterprise levels are deferred. Project
  `.agents/` files are ordinary workspace files — editable by ordinary tools
  under user visibility (a customization channel, not autonomous promotion —
  R3-16). *(Amended R3:)* resolution materializes **one content-addressed
  snapshot at launch** (deterministic cwd order = workspace list order;
  project > user; conservative policy meet), passed by descriptor, digest in
  `run_start` — no worker ever reads live instruction paths mid-run (R3-20).
  Discovery, normalization, merge, and exact snapshot bytes are owned by
  [instruction-snapshot-v1](../../spec/instruction-snapshot.md).

- **D-56. Spawn is not a sandbox.** Tool containment is four layers from one
  policy source: authorization (allowlist + approval), clean environment
  (D-46), advisory pre-check, and a mandatory OS profile (Seatbelt /
  landlock via the Platform seam) derived from the same writable-roots and
  network policy — one source, two enforcement points. Exec is argv-direct
  (shell interpretation only inside the shell tool); in-process tools rely
  on code-enforced path policy under an optionally profiled worker; profile
  strictness follows the authorship trust gradient, third-party
  plugins/MCP tightest.

- **D-57. Filesystem tools default to exec through one multi-call helper**
  *(revises the earlier in-process default — crash ≠ session loss, but kill
  is only reliable across a process boundary and spawn cost is noise against
  model latency)*. The helper is one binary (busybox-style), individually
  sandboxable; write/patch temp+rename inside it, so kills are
  crash-atomic. In-process remains only for the mobile seam and measured
  hot paths. Backend-independent anti-hang rules *(amended R3)*:
  descriptor-first `openat` with O_NONBLOCK/no-follow + post-open `fstat`
  (stat-then-open is a race), refuse FIFOs/devices, explicit
  materialize-or-refuse for dataless files, byte caps, and timeout
  escalation — an unkillable in-process timeout always converts to worker
  exit, never continue-past (R3-22).

- **D-58. The inner worker protocol is versioned bespoke JSONL** *(closes
  D-open-1, R4-4; outer half superseded by D-62)*. The protocol version is
  negotiated at spawn (exact handshake in 03); the daemon socket reuses the
  same framing; golden wire fixtures are part of the conformance suite. The
  client boundary is the Session Endpoint v2 contract (D-62/15), not ACP.
- **D-59. Swift first; fixtures are language-neutral** *(closes D-open-2;
  implementation-language choice superseded by D-63 before code)*. The event
  schema, wire protocol, and every golden fixture are language-neutral and
  reused unchanged by every implementation.
- **D-60. Resume policy is an immutable per-spawn field** *(closes
  D-open-3; amended R6)* on `genesis`/`spawn`: interactive mainlines `never`
  (recover then settle interrupted, wait for the user); validation/workers
  `bounded(n)`; the field is required — absent is invalid (R14-7);
  children inherit the spawn's declaration.
  Exhaustion accounting is durable and replay-checkable: every `run_start`
  carries `mode` and `recovery_ordinal` (count of prior recovery runs for
  the current unsettled tail); `bounded(n)` exhausts at ordinal ≥ n and the
  next recovery is reconcile-only. An unknown provider outcome always takes
  the adapter three-way regardless of policy.

- **D-61. All recovery is a worker run** *(new R5; closed R6)*. Semantic
  tail recovery — attempt three-way (D-43), aborted results, child_result
  synthesis, the settle — executes only inside a spawned run. The spawn's
  mode is a hint: the authoritative `mode` is **derived under the acquired
  lock** from durable state — resume/ordinal-permitting tail or unprocessed
  durable input → ordinary (reconcile, then continue; **ordinary
  dominates**), else reconcile-only — so mixed concurrent candidates
  converge and Rule 1 stays sound (R6-1). A reconcile-only run appends its
  own `run_start {mode: reconcile}`, reconciles in 03's single order, and
  writes exactly one `settle {interrupted, reason: recovered}`. The scoped
  endpoint management-recovery startup is the exception: after its attributed
  `run_start` it completes only the preloaded queue transaction, returns its
  result, and exits without generic reconciliation or settle; a later triage
  run handles unrelated recovery work. "No
  provider call" means **no new loop attempt**: read-only query-by-identity
  for D-43 is permitted; resend is not (R6-2). The supervisor inspects,
  lock-arbitrates, decides spawn-vs-bookkeeping, and may locked-append only
  the file-scoped externally-authored kinds admitted by tail-lifecycle's
  delivery matrix (`input`, `queue_edit`, external `meta`, manual `compact`)
  plus ack-bearing `approval_response`, stop intents, and the scoped genesis-
  on-create exception; it authors no recovery/provider/tool semantic facts and stays
  provider-ignorant; attribution is always to the run that writes.
  *(Closed R7:)* the complete transition table lives in 03 §Recovery
  transitions; precedence is stop → input → resume/ordinal → reconcile-only;
  endpoint-management-v2's incomplete queue transaction is an external pre-
  gate to that precedence: one reconcile-only management-startup worker
  completes or durably rejects the transaction before any later stop/input
  delivery or ordinary candidate proceeds;
  recovery launches are exempt from the stop spawn gate; the supervisor's
  locked-append whitelist gains the scoped genesis-on-create exception
  (R7-4); `unresolved_dispatch` closes the epoch (R7-3).

- **D-63. Contracts remain language-neutral; Rust is the first reference
  implementation** *(supersedes D-59's Swift selection before code)*. Rust
  crate, test, and OS-binding choices live only in the implementation profile
  (17). They cannot shape durable events, wire DTOs, lifecycle outcomes, or
  the language-neutral fixture oracle.
- **D-64. Native thread/session identity is one UUID.** The folder name and
  `genesis.thread` are the same lower-case canonical UUID; new native ids use
  UUIDv7. The kernel reports that UUID only. The Host/Client supplies the AS
  short name and constructs `tekes://<as-name>/<as-session-id>` (15), so no
  second native identity map or URI authority enters the kernel.
- **D-65. Explicit offline redact ships in v1.** Slice 5 implements D-40 and
  D-50 through [rewrite-publication-v1](../../spec/rewrite-publication.md):
  selector inventory, content-scanned carrier closure, staged publication,
  source retirement, crash recovery, and operation-owned GC. Folder deletion
  remains available but is no longer the only erasure mechanism.
- **D-66. Production provider parity closes before advertisement.** Slice 7
  implements the five config-visible protocol families in
  [provider-runtime-v1](../../spec/provider-runtime.md). Exact predecessor
  request/response/stream bytes for the four predecessor rows, the repository-
  owned oracle for the new Google Interactions row, context behavior, HTTP
  classification and recovery gates are mandatory; an incomplete adapter is unavailable,
  never a partially successful compatibility mode.
- **D-67. Production tools use one dispatcher and worker-control v2.** Slice 8
  keeps builtin-tools-v1 as the sole catalog and routes its backend/effect
  classes through [tool-runtime-v1](../../spec/tool-runtime.md). The new
  supervisor-control pair is negotiated only in
  [worker-control-v2](../../spec/worker-control.md); v1 bytes stay frozen and
  the worker remains the semantic-event author.
- **D-68. Session Endpoint v2 uses the Client-owned HTTP/WebSocket carrier.**
  Slice 9 binds the Slice-6 service to `POST /api/{method}` plus
  `/api/events.mux` and `/api/events.host` WebSockets as specified by
  [endpoint-management-v2](../../spec/session-endpoint.md) and
  [endpoint-transport-v2](../../spec/session-endpoint.md). DTOs and endpoint
  seq remain the imported Client authority; TekesAppServer v1 never enters the
  surface.
- **D-69. First production packaging is a local macOS launchd service.** Slice
  10 ships the three service executables plus the external `tekes-selector`
  resident launcher/management artifact, fixed `127.0.0.1:7347` loopback
  endpoint, immutable install signing/access-group identity, shared
  Client/supervisor Keychain bearer, recoverable installer transaction,
  explicit local-APFS root, readiness/observability and install lifecycle in
  [deployment-v1](../../spec/deployment.md). Binary activation, crash-loop
  observation and last-known-good rollback remain external authority (D-53),
  not supervisor self-evolution: launchd keeps `tekes-selector serve` alive,
  selector spawns/observes the supervisor from a frozen selection, and three
  independent candidate-attributable failed launches durably roll back without
  any implicit test or installer actor; environment/data failures stay
  not-ready without a generation switch. The deployment controller explicitly authors candidate
  canary input through Client; selector only attests it. Selector never writes
  Kernel semantic authority. Automatic rollback has an explicit serve-authored
  operation identity; it is not modeled as a nonexistent CLI/stdout actor.
- **D-70. Supervisor-control exact-retry receipts are a closed internal
  protocol carrier, never semantic truth.** A canonical request/result record
  is durably published before the supervisor replies and may be collected only
  after the paired terminal `tool_result`; this is the existing D-1
  protocol-carrier authority class, not a supervisor-authored event or replay
  input.
- **D-71. Endpoint live buffering is bounded and never an identity authority.**
  The executable endpoint-projection-v2 contract fixes 4 MiB or 4096 frames
  per subscriber, whichever comes first. Overflow emits typed `live-gap` when
  writable and closes; reconnect performs the bounded history refetch. Durable
  endpoint seq/chunk identity lives in the projection carrier, so a ring buffer
  is only an optimization and may never drop-and-continue.
- **D-72. Installed binaries and user data have different roots; project
  `.tekes` contains authored portable input only.** The signed installer,
  selector, immutable bundles and installer journal remain under
  `~/Library/Application Support/Tekes`; launchd's plist remains in
  `~/Library/LaunchAgents`. All non-secret Kernel durable authorities and the
  Client-owned mirror/runtime storage covered by D1/D2 live under the
  mode-0700 `~/.agents` root: thread ledgers and endpoint carriers, workspace
  authorities and stable folder bindings, jobs, config, cache, logs,
  safepoints, runtime staging, user skills, and the Client mirror. UI-only
  preferences may remain in `UserDefaults`; provider secrets remain in the
  platform secret store; signed install state and the LaunchAgent plist remain
  in the locations above. A project
  may contain `<cwd>/.agents/{skills,instructions,tools,rules,project.json}`
  because those files are human-authored and move with Git; it may not contain
  ledgers, mirrors, jobs, caches, safepoints, staging, logs, secrets,
  bookmarks, locks, sockets, or PIDs. Legacy Application Support data and
  project Composer staging migrate through fail-closed, idempotent journals;
  new and old authority at once is a conflict, never precedence.
- **D-73. Workspace identity and execution admission are explicit, separate axes.** A
  workspace stores one ordered list of folders with stable binding ids; the
  current absolute path is a machine binding, while genesis carries workspace
  id plus the selected folder binding. That binding selects execution cwd and
  per-binding safepoints without reordering or narrowing all-folder instruction
  capture. A binding-less legacy session is accepted only for one-folder
  workspaces. User, workspace and project instruction sources remain
  distinguishable; same-name skills across scopes inside one workspace fail
  explicitly, while unrelated workspaces have separate resource catalogs.
  Worker admission bounds simultaneously live line executors;
  provider admission independently bounds scarce outbound provider calls and
  is acquired/released per call. A default value of one for both does not make
  the queues equivalent and does not authorize merging their receipts or
  recovery semantics.

## Current terminology clarification (2026-09-04)

D-5's original "Session is not a concept" rejects a separate mutable session
store; it does not prohibit describing a model execution context. Under the
[Thread model](../concepts/thread.md#thread-fundamentals-a), a thread groups lines,
each line survives worker runs, and its context is reconstructed from durable
history and launch inputs. Public `session` identities and validator contexts
must be interpreted in their interface scope. D-3 still distinguishes a durable
turn outcome from process liveness; a candidate-final flag is neither condition.
This clarification preserves the original decision text and storage design.

## Open


- ~~D-open-4~~ closed as **D-62** (15): the client surface is the Session
  Endpoint v2 contract — method table, message-aligned history paging, mux
  stitch semantics; server-side projections included by contract.
- ~~D-open-5~~ closed as **D-71**: durable projection identity plus bounded
  gap-and-refetch replaces drop-oldest/snapshot ambiguity.
- ~~D-open-6~~ closed as **D-65**: explicit offline redact ships in v1.

External review R1 positions (recorded, not adopted and superseded where a
later decision says so): D-open-1 bespoke inner protocol + ACP outer;
D-open-2 Swift first with language-neutral golden
fixtures; D-open-3 immutable per-spawn resume policy (interactive: never
auto-resend; workers: bounded resume; unknown provider outcome: adapter
three-way); D-open-4 include server-side paginated list/search/usage
projections from day one (a remote client cannot ripgrep the server); D-open-5
bound by bytes+frames+age with gap-and-snapshot reconnect — and flags that
this is weaker than today's persisted mirror-first tail: restoring parity
would reopen D-9 as a deletable, non-authoritative live spool.

## Closed (historical)

- ~~D-open-1~~ closed as **D-58** (R4). ~~D-open-2~~ closed as **D-59**
  (R4; implementation selection superseded by **D-63** before code).
  ~~D-open-3~~ closed as **D-60** (R4).
