# Supervisor: hosting, delivery and reaping

[Runtime · documentation home](../README.md) · [Worker](worker.md) · [Next: supervisor implementation](../../crates/supervisor/docs/README.md)

The resident execution-core component, started by the host application
([Application-owned launch](../builtin-launch.md)).
This chapter describes the logical design introduced by this
architecture. Everything it owns is either an OS primitive wrapped thinly or a
rebuildable cache. It holds no truth: kill the supervisor, restart it, sweep,
and the system is whole.

## Components

1. **Process table** — `{line file → pid}` routing cache for workers this
   supervisor owns. **Liveness authority is the file lock** (D-2/R4-2): a
   lock held by a process the table does not know is an explicit
   `busy-unknown/quarantined` state — never "dead", never a locked-delivery
   target. There is no run-state field anywhere on disk. The supervisor is a **singleton per storage root**
   (exclusive root lock, D-41), and workers are supervisor-bound: a restarted
   supervisor never reconstructs pids, pipes, or reap authority — surviving
   workers see stdin EOF and exit (a clean hold parks with no settle —
   R9-2); the sweep respawns what should run. A flock probe answers only "busy", never "whose".
2. **Spawn service** — the **only** launcher of workers (R2-2): resolves the
   workspace snapshot atomically with launch (the stamped version is the
   contract — D-36), captures the instruction plane, publishes both exact
   canonical snapshots into the target thread's content-addressed assets,
   and passes already-open read-only descriptors plus expected digests. It
   passes credentials separately (descriptor — D-46) and never exposes live
   config paths to the worker. The executable byte contracts are
   [config](../../spec/config.md) and
   [instruction-snapshot](../../spec/instruction-snapshot.md). It rejects
   **work** launches for trees with an active stop
   generation — recovery-run launches are exempt (R7-1), an active stop
   forces their mode to reconcile-only, and the generation **closes** at
   the final settle of the turn open at its append — born closed on a
   settled tail, while a stopped unstarted child takes the gate-exempt
   cancel-before-start run before closure (R9-1/R13-1/R15-2); held pre-closure inputs run only
   after a post-closure user action (Worker lifecycle §Stop-generation lifecycle, R8-1). A worker exiting **76**
   (protocol mismatch, D-58) quarantines its (binary, protocol-range)
   tuple in the process-table cache: no re-ensure during that supervisor
   lifetime until the executable bytes change, surfaced as an operational
   error; a restarted supervisor may probe once and reconstruct the
   quarantine (R7-6).
   The stop protocol: set the in-memory gate here → deliver the keyed stop →
   require durable `stop_requested` in the target file (receipt of the echo,
   or kill + locked append) **before** propagating to that node's children
   or acking the stop; the sweep re-drives from any durable stop. Provider
   concurrency is a separate **admission lease service**: leases identified
   by {class, attempt id, worker}, granted/queued/denied over the control
   channel, released **only** on the worker's `attempt_settled` message
   (sent after the terminal output/error barrier — R3-2) or reclaimed at
   reap; all leases are void at supervisor restart (F26) — the supervisor
   stays provider-ignorant, and spawn never holds capacity.
3. **Reaper/reconciler** — `wait()`s on children and applies **D-61: all
   recovery is a worker run**. On a dead worker with an unsettled tail the
   supervisor triages per
   [spec/tail-lifecycle.md](../../spec/tail-lifecycle.md) (R11-5): a
   `parked_hold` spawns nothing; an `answered_hold` spawns an ordinary
   resume candidate; otherwise a **recovery candidate** (mode hint:
   reconcile) — the authoritative mode is always re-derived under the
   acquired lock (R7: the supervisor never decides mode pre-lock). A normal
   tail-recovery reconcile-only run appends its own
   `run_start {mode: reconcile}`, reconciles in [the worker recovery order](worker.md#recovery-transitions-d-61-closed-r7), and
   writes exactly one `settle {interrupted, reason: recovered}`. The supervisor stays provider-ignorant; recovery facts are
   attributed to the run that writes them (R3-17); no fact ever lands after
   the terminal record. Only "we stopped it / we reaped it" may cause an
   outcome to be authored.
   Before tail classification, the endpoint-management queue-transaction
   pre-gate blocks ordinary ensure/delivery and admits one reconcile-only
   management-startup worker to complete an interrupted
   retraction/replacement pair before generic reconciliation.
4. **Delivery** — the single externally-authored cross-folder writer for the
   closed tail-lifecycle delivery matrix (`input`, `queue_edit`, external
   `meta`, manual `compact`, approval responses, and stop intents), with durable-first
   acknowledgement (F2): target worker alive → write to its stdin and ack the
   client only after the worker's **correlated `receipt {origin tuple, seq,
   deduplicated}`** — never a doorbell, which may coalesce (D-48) — comes
   back (broken pipe or no receipt → retry under the same `origin_key`);
   dead → take the lock, append durably, sync, ack, then (per policy)
   respawn. No path acknowledges before durability, and user input is never
   held hostage to a spawn succeeding.
5. **Stream lane** — accepted worker frames are published to the transient
   presentation channel and nowhere else (session-endpoint §Journal projection
   §Stream-frame ingestion); the attempt's durable `assistant/message`
   supersedes them. Client reattach mid-stream = journal snapshot + live
   subscribe; a crash or reconnect loses the transient tail. There is no
   in-memory ring buffer and no chunk storage.
   A frame's projection refresh publishes durable events only up to the
   frame's own `ledger_seq` (worker-control), or up to what the journal
   already holds if that is later: eagerly dispatched `tool_call`/`tool_result`
   records and the response terminal ring their own doorbell and never
   overtake an earlier frame still in the pipe; a legacy frame without the
   stamp keeps the older cut before the attempt's first tool/output
   record.
6. **MCP peer pool** — every MCP peer generation (stdio child processes,
   HTTP transports, plugin-native servers, the Computer Use reference server)
   is supervisor-owned (`mcp_runtime`, `McpBroker`); workers reach them only
   through `supervisor_control` tool calls on the control channel, and the
   tool ABI is unchanged ([Tool execution](tools.md)). No MCP child is a
   worker child (mcp-runtime).
7. **Sweep** (boot + periodic) — walk folders and the process table:
   - main line with no live worker whose endpoint journal is behind the
     ledger's `last_seq` → run the one endpoint-projection path
     (`publish_appended`). Worker doorbell, exit reconciliation, and boot are
     the fast paths; the sweep is the deterministic retry, so a projection
     that failed after a settle is repaired within one tick instead of
     waiting for the next frame or restart. A V3 journal stream open runs
     the same path first for the same reason.
   - unsettled tail + no live process → triage per
     [spec/tail-lifecycle.md](../../spec/tail-lifecycle.md) (R11-5, the only
     authority): `parked_hold` → no spawn; `answered_hold` → ordinary
     same-turn resume candidate; otherwise a recovery candidate whose mode
     is derived under the lock. The supervisor never reconciles in place.
   - child file with `genesis` but no matching parent `spawn` event → harmless
     by construction ([delegation](../flows/delegation.md)); the designed
     `orphans/` quarantine move is not implemented (the file is left in place
     and never scheduled).
   - `spawn` event with no child file → schedule a recovery run on the
     parent, which appends the `child_result {failed}` (D-61 — the sweep
     never authors it).
   - `spawn` without `child_result` where the child is settled → schedule a
     **recovery run on the parent** to synthesize the `child_result`
     idempotently (F17/D-61); where the child is live under a
     `stop_requested` root → terminate and join recursively along the
     durable spawn graph, then terminalize the root via its recovery run
     (D-37/61) — no descendant outlives a settled root.
   - rewrite operations: enumerate `staging/*/op.json` and resume each at
     its recorded phase idempotently — fork closes after publication
     (source untouched, D-47); redact retires the source before close;
     staging debris with no operation record is GC'd (R3-11/R5-1).
   - manifest refresh. Asset GC (unreferenced blobs past a grace period) is
     part of the design but not implemented: nothing deletes thread assets
     today; rewrite debris GC (`gc_rewrite_debris`) is the only collector.
8. **Schedule** — [schedule](../../spec/schedule.md) owns the global JSONL,
   cron/IANA-zone evaluation, claim-before-launch, keyed redrive and explicit
   missed-occurrence policy. The supervisor owns the internal authority; 14F
   alone may expose versioned Client management DTOs.
9. **Client interface** — a thin read/control RPC: read event ranges by seq,
   subscribe to doorbells/frames, submit control messages, thread CRUD
   (create folder, archive = mv under catalog + folder lifecycle locks, fork = the
   rewrite projection with staged publication — [storage](../data/storage.md)). Creation is keyed like
   every mutation (R3-1): `genesis` records the creator's origin key, and
   the creation-receipt index (origin tuple → thread id; a rebuildable cache
   over genesis scans) makes a retried create return the existing thread
   instead of minting a second one; the index check and the folder creation
   are **one atomic step of the serialized creation path** (R3-1), so two
   concurrent same-tuple creates cannot both miss and both mint. The native
   interface reports the canonical UUID thread/session id only (D-64);
   it does not report a URI or an AS name. The Host/Client composes the
   external locator per [the Client mapping](../history/client-v2.md#canonical-external-session-uri). Local same-machine renderers may read files
   directly (read-only); remote clients
   must use this interface; **writes never bypass the supervisor/worker pair.**
   The current client boundary implements [Session Endpoint](../../spec/session-endpoint.md#routes-and-streams):
   unary mutations and one multiplexed stream for workspace, inventory, journal,
   control and actionables. D-62 and [the historical Client mapping](../history/client-v2.md) retain the historical V2 mapping;
   the inner worker protocol remains separately versioned bespoke JSONL (D-58).
10. **Catalogs** — model catalog, capability probes, MCP catalogs: probe
    results cached as rebuildable files; never truth.

## Rules

- **Restart backoff counts recorded failures only.** Exit reconciliation feeds
  the crash-loop backoff (1, 2, 4, 8 s, five strikes) only when the worker
  recorded a failure; a clean exit whose tail still asks for a run (answered
  hold, queued input, due wait) clears the backoff and is re-ensured at once.
  SWE-bench exposed the old behavior: an approval-heavy turn hit
  `process-host-restart-limit` after nine clean approval resumes.
- **OAuth-bound MCP servers.** Production startup (both `builtin.rs` and
  `daemon.rs`) installs no secret mutation authority, so an `oauth` binding
  carries a platform-installed token, injected as-is; the application owns
  minting, rotation and revocation. With a mutation authority, as tests and
  explicit embedders supply, a Kernel-minted refresh grant
  (`provider::OAuthGrant`) is exchanged for access tokens by
  `provider::OAuthTokenExchange` inside the HTTP transport's authorization
  provider (`http_request_authorization`), and rotation and revocation write
  through `SecretAccess.mutation`. Live gate: `scripts/run-live-mcp-oauth.py`.
- **Manual compaction (the `/compact` command).** `commands/run` with a body
  that is exactly `compact` is a control request, not an input
  (`SessionDeliveryAuthority::compact`). A live worker receives worker-control
  `compact` and receipts it after applying it at its next yield; an idle line
  is compacted by the supervisor itself (the model summary request first,
  outside the lock — `ProductionProcessHost::summary_for_manual_compaction` —
  then the compact carrying its `summary_request` telemetry, event §compact) under the line lock
  (`locked_compact`): checkpoint marker, `engine::plan_context_compaction`
  for the turn after the settled tail, summary inline or spilled, one
  origin-keyed `compact` event. The same origin key returns the original
  receipt. The next run's epoch carries `reason: "compaction"`.
- **Automatic thread title (session-endpoint §session.rename).** After
  the first accepted root input the supervisor seeds a keyed title `meta`
  (the input, whitespace-collapsed, ≤ 40 chars) only when the session has no
  title, then runs one host-owned presentation request on its own thread —
  the configured DeepSeek route with model pinned to `deepseek-v4-flash`, no
  tools/continuation/reasoning — and appends the refined title as a second
  keyed `meta` only if the seed is still the latest title, so a racing
  `session.rename` wins. Provider failure leaves the seed; later prompts never
  retry (`wait_and_seed_automatic_title`, `refine_automatic_title`,
  `automatic_title_route`; stderr `process-host-automatic-title-*`).
- **Post-turn memorization.** A root line that exits `settled` with
  `completed` spawns one helper thread: every candidate the thread noted
  (`note`) and left unconsumed is memorized through
  `WorkflowBackend::automatic_memory_merge` in one merge keyed by
  `memory-merge-<turn>-memorize`, so a repeat is a receipt. There is no model
  request: the model records candidates during the turn, the host memorizes
  them when the turn completes, and nothing is written when there is none.
  The settled ledger is not written (every turn-bound kind is closed by the
  settle); the memory log is the store of record. Outcome and failures are
  reported on stderr (`process-host-memory-merge*`). Live proof:
  `scripts/run-public-flow.py --live --case text --memory`.

- **Whoever holds the flock writes.** The supervisor takes a lock only when no
  worker is alive; it never races a live worker and never writes content rows
  beyond the exact D-61/tail-lifecycle delivery whitelist: delivered `input`,
  `queue_edit`, external `meta`, manual `compact`, `approval_response`,
  `stop_requested`, plus genesis-on-create. Semantic recovery facts are always
  worker-authored (D-61); an endpoint queue transaction is completed by its
  attributed management-recovery worker, never by supervisor semantic append.
- **Trust the log over everything** — exit codes, mirrors, its own memory. Any
  supervisor state must survive `rm` of all caches + a sweep.
- Archive/mv only under lock; live thread → queue the request ([Thread definition](../concepts/thread.md)).
- Boot: take the exclusive root lock or exit (D-41); assert the root is a
  supported local filesystem and not iCloud-synced, probing lock/append
  capability (F10); create the diagnostics reserve (F19); **drain legacy
  lock holders under a short bounded deadline** — holders exit promptly on
  stdin EOF (EOF cancels every worker phase, R3-7); a file still busy at the
  deadline is **quarantined individually** while the rest of the root
  proceeds — one stuck thread never blocks RPC, sweep, or admission (R3-7);
  admission counts rebuild as the drain completes; run the sweep; start
  schedule; open the client interface.

Through Slice 6, “client interface” means the carrier-neutral Endpoint service.
The concrete loopback HTTP/WebSocket listener and its drain/readiness behavior
are Slice 9's [session-endpoint §Transport listener](../../spec/session-endpoint.md#transport-listener), while
the exact management author/recovery paths are
[session-endpoint §Management authority and recovery](../../spec/session-endpoint.md#management-authority-and-recovery); Slice
10 made that listener launchd-owned production packaging, since retired in
favour of [application-owned launch](../builtin-launch.md). The current public
route and stream contract is [V3](../../spec/session-endpoint.md#routes-and-streams), which supersedes
the V2 public list/history, dual-stream and respond routes. The concrete launcher
and daemon composition is mapped in [processes](../architecture/processes.md).

## Core vs bundled services (R2 cap ruling)

The components split into a **core** — root singleton, spawn/reap/reconcile,
durable delivery + receipts, stop reconciliation, admission leases — and
**bundled services**: schedule, catalogs/search projections, ring-buffer mux,
daemon brokers, and rewrite/GC (staged-operation execution and per-phase
recovery — R3-11/13). Bundled services speak to the core only through public
authorities (files + the control protocol) and are individually evictable
into separate processes without any contract change. The D-24 size cap and
"holds no truth" apply to the core; a bundled service that grows is evicted,
never absorbed.

## What the supervisor is not

Not an orchestrator (no turn logic, no rendering, no provider knowledge), not a
store (no truth), not a message bus beyond delivery, not a plugin host (an
extension backend is just another process/daemon). Core growth beyond the
component list above is harness regrowth — treat as an architecture bug.
