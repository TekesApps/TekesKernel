# Events: identity, input and results

[Data and reliability · documentation home](../README.md) · [Storage layout](storage.md) · [Next: durability and recovery](durability.md)

One event = one JSON object on one line, UTF-8, LF-terminated. No enclosing
array. A valid prefix of a file is always a valid history.

## Envelope

Every event carries:

```json
{"v": 1, "seq": 42, "turn": 7, "kind": "tool_call", "ts": "2026-08-26T09:00:00.000Z", ...}
```

- `v` — envelope version; kind payloads carry their own version where they
  evolve. `genesis` declares the file's format version plus minimum reader
  and writer versions; a later writer that extends the file with new
  semantics first appends an upgrade `meta` event raising the **effective
  minima** (= genesis ∨ latest upgrade). An event may carry `min_reader`.
  A worker below the effective minima — or one that meets a
  `min_reader`-gated, visibility-bearing event it cannot interpret — goes
  **read-only, fail-closed**: render and serve, never append, never run the
  provider loop (D-49). Forward-visible state extensions should instead ride
  the `state` envelope, which old readers render generically ([Model context](context.md)).

- `seq` — per-file, monotonic, starts at 1, no gaps. **Cursors, ranges, and
  references use `seq`, never byte offsets.** (Byte offsets appear only as
  rebuildable `seq→offset` samples in the manifest.) A gap is a corruption
  signal, not a feature. This is the day-1 decision that leaves the door open
  for future file segmentation.
- `turn` — the turn this event belongs to. **Turn-bound (carry `turn`)**:
  turn_open, output, reasoning, tool_call, tool_result, child_result, state,
  approval_request, approval_response, effective_execution, spawn, attempt,
  attempt_dispatched, attempt_recovery, settle, error, note.
  **File-scoped
  (canonically absent — no zero/null sentinels)**: genesis, run_start,
  epoch, input (a queue record — R14-1), checkpoint, compact, meta
  (upgrade included), stop_requested, queue_edit (R4-5/R11-2, exhaustive;
  `queue_release` deleted R9-3). A turn spans from its `turn_open` to its
  settle event (R14-1).
- **Canonical encoding = RFC 8785 (JCS)** over these envelope rules; the
  `fixtures/events/` corpus asserts byte-identical re-encoding (R6
  blocking 4).
- `kind` — event type. Open set; core kinds below.
- `ts` — ISO8601 UTC **string**. Never epoch numbers.
- **Integers that can exceed 2^53 are encoded as strings** (tokens, provider
  ids, epochs). JSON numbers are for small counts and human-scale values only.
- `supersedes` (optional) — `[{"from": s1, "to": s2}, …]` seq ranges this event
  retracts (retry, rollback, compaction). First-class: every projection must
  honor it; renderers never consume raw appends without applying supersedes.
  Covering provider-admitted ranges closes the epoch ([Model context](context.md)).
- `visibility` (optional) — overrides the kind's default model-visibility ([Model context](context.md)).
- `anchor` (optional) — this event must survive every compaction verbatim ([Model context](context.md)).
- `origin_key` (optional) — caller-supplied idempotency key on every
  externally-originated mutation, deduplicated as the namespaced tuple
  (principal, client, target, operation, key); duplicates are dropped at
  append and re-acked with the original seq ([Idempotency rules](idempotency.md)).

## Core kinds

| kind | Payload (essentials) | Notes |
|---|---|---|
| `genesis` | format version, thread id, workspace id, creator `origin_key`, `parent {file, seq, spawn_id}?`, seed provenance + canonical source-event snapshot asset, config snapshot, `identity_profile: "auto" \| "coding" \| "general"`, `resume: "never" \| {"bounded": n}` (immutable; required — absent is invalid, R14-7) | Always `seq` 1. Declares identity selection mode; `auto` is resolved once by a runtime-only `state{subkind:"identity.selected"}` before the first model call. Legacy records without this field use coding. The event defines seed asset bytes and source provenance; the origin key makes creation retryable ([Idempotency rules](idempotency.md)); `resume` is D-60's recovery policy |
| `run_start` | run id, `mode: ordinary \| reconcile`, `recovery_ordinal`, binary version, config digest, instruction-snapshot digest, policy | Appended + synced at every lock tenure; events belong to the latest preceding run_start — attribution by total order (R3-17); mode is derived under the lock (Worker lifecycle §Recovery transitions) and the ordinal makes `bounded(n)` exhaustion replay-checkable (D-60) |
| `epoch` | frozen SystemProfile + control envelope (asset ref), model/adapter identity, ordered tool digest, policy ids, named `reason` | Opens a provider-conversation segment; render starts at the latest one ([Model context](context.md)) |
| `input` | content blocks; attachments as asset refs; `origin_tuple` | File-scoped queue record (R14-1): consumed by the `turn_open` naming its seq (steer: by `attempt.admits`); a steer whose turn settles unadmitted is expired |
| `turn_open` | `trigger: "genesis" \| {inputs: [seq]}` | First event of every turn (R14-1); consumption is exactly-once, ascending, prefix-complete (release — R14-2) |
| `output` | `usage` (`availability: reported \| unavailable` + figures as strings — the attempt's accounting rides on the outcome that settles it), `final_answer` (optional for legacy records), model content blocks; **sealed native fragments** (adapter-owned, versioned, asset-spill — D-42); continuation identity; `attempt` ref | Completed provider response, not turn settlement; explicit final marks a candidate. Streaming frames never enter the thread ledger (D-9 as amended R17; endpoint chunk identity belongs to [the Client mapping](../interfaces/client.md)); the baseline carrier ([Model context](context.md)) |
| `reasoning` | reasoning display content | Only when the provider emits it as displayable |
| `tool_call` | tool name, arguments, call id | **Intent event — fsynced before the side effect executes** |
| `tool_result` | call id, outcome, output or asset ref | Pairs with `tool_call` |
| `approval_request` | what, why, requested scope | Worker blocks on stdin after appending |
| `approval_response` | grant/deny, scope | Ack-bearing barrier echo of the control message; durable before its receipt, for replay |
| `effective_execution` | call id, full effective invocation (asset-spilled when large) | Runtime-only write-ahead for every hook-mutated invocation ([Tool execution](../runtime/tools.md)); barrier event |
| `spawn` | child file name, call correlation (model tool-call id or host validation candidate), stable spawn id, seed description, `resume` declaration for the child (inherited into its genesis; required — absent is invalid, R14-7) | Appended **after** the child file exists ([Turn flow](../flows/turn.md)); D-44/D-60 |
| `child_result` | child file, call correlation + spawn id, outcome summary, artifact refs | Delegated terminal summary, exactly once per call id (D-44); root validation also records host-owned candidate/verdict/decision/feedback states |
| `stop_requested` | stop generation, origin | Durable stop intent; must be durable in a file (echo, or kill + locked append) before the cascade propagates past it, and before the stop is acked (D-37); the generation closes at the final settle of the turn open at its append — appended on a terminal tail it is **closed at birth** (R9-1/R13-1; normative predicates in spec/tail-lifecycle) |
| `attempt` | provider send intent: attempt id, epoch, wire digest, `admits` seq-ranges | fsynced before the HTTP request ([Idempotency rules](idempotency.md)); settled by exactly one output/error (D-43); `admits` counts toward admission **only when settled by a durable output** (D-29) |
| `attempt_dispatched` | attempt id | Adapter-capability-gated handoff marker, fsynced before HTTP: unpaired attempt without it settles `error {not_dispatched}` locally; with it → three-way recovery (D-43) |
| `attempt_recovery` | attempt id, chosen decision, adopt `inventory: [call ids]` + `response: {asset}` (the synced recovered response — R11-3) | Durable record of the recovery choice, appended before acting on it (D-43); **barrier event** — the adopt journal's self-sufficient commit point; completion never re-queries |
| `settle` | discriminated union ([spec/event](../../spec/event.md)): `{completed}` \| `{interrupted, reason: user_stop \| budget_tokens \| budget_wall \| recovered}` \| `{error, classification: provider_terminal}` — closed; `unresolved_dispatch` is an attempt-error classification, never a settle reason (R11-4) | Marks `terminal_recorded`. fsynced. Exactly one settle per turn — holds are **not** settles (R9-2) |
| `checkpoint` | summary, covers-up-to seq | Plain covering marker and barrier: the boundary a later `compact` may cover. Never a replay seed; readers fold from genesis. Older records carrying digests/keys/state fields remain valid markers (the fields are ignored). |
| `compact` | `covers` ranges, replacement summary; `origin_tuple` when manually originated (R13-4); optional `shadow` (compaction v5: frozen bundle digest, admission verdict, artifact, mode) | Lossy projection marker; covered events remain on disk; may cover only events a durable checkpoint already covers (retain-until-checkpoint); manual compaction is ack-bearing (barrier + receipt/dedup); `shadow` is telemetry unless the mode is `promote`, in which case the admitted continuation is the summary ([spec/event](../../spec/event.md) §compact) |
| `error` | recoverable flag, `classification: provider_terminal \| unresolved_dispatch \| transport \| rate_limit \| tool \| internal`, `attempt` ref + `usage` when provider-borne (an error that settles an attempt carries its usage) | Terminal error + `recoverable:false` is settlement evidence; the classification vocabulary is attempt-level, disjoint from settle reasons (R11-4) |
| `meta` | thread-level facts: title, labels, ownership generation (D-47); `origin_tuple` when externally originated (R17k); `notice: {severity: warning \| error, classification, operation, message}` — a host-authored runtime fact outside any turn (a launch that ran degraded, an input no worker will run), projected as `session/notice` | Supersedable; externally-originated meta is keyed + ack-bearing barrier; manifest caches it ([Thread definition](../concepts/thread.md)); model-invisible |
| `queue_edit` | `origin_tuple` + envelope `supersedes` over queued/held inputs | Keyed queue CRUD (R11-2): file-scoped, runtime-only, ack-bearing **barrier**, never a runnable trigger, never a release — release stays new-contentful-input-only |
| `state` | typed subkinds: goal, validation.feedback, memory.delta, tool.offer, tool.activated, … | Named model-visible state deltas — required-visible facts never ride unknown kinds |
| `note`, … | open extension kinds | |

**Seq-bearing fields registry** (the complete list a rewrite must handle —
D-50): `supersedes` ranges, `compact.covers`, checkpoint coverage,
`genesis.parent.seq`,
`turn_open.trigger.inputs` (a rewrite keeps a consumed input — possibly
content-redacted — or drops the whole turn; a dangling reference is
illegal — R15-1), `attempt.admits` (all covered seqs predate the attempt —
R16), `epoch.pending` eligible/withheld lists (predate the epoch — R16r),
`output`/`error` attempt refs. The registry is the source-side audit
list; [rewrite-publication](../../spec/rewrite-publication.md) is the
destination-shape authority. It applies and drops supersede/compact markers,
drops attempt/checkpoint/continuation state, normalizes retained historical
provider facts into `state`, remaps inputs/turns/parents, and validates the
result from genesis. A seq-bearing field added later must be added here and to
the rewrite contract before a writer may emit it.

Two flavors share the schema: **conversation events** (facts a model may see —
input/output/tool pairs/state deltas; REQ-014's `ConversationEvent` set) and
**marker events** (structure and lifecycle — genesis/epoch/checkpoint/compact/
settle/meta). The distinction is carried by the visibility dimension ([Model context](context.md)), not
by a separate format.

**Unknown-kind rule:** readers must tolerate kinds they do not know — preserve
them, render a fallback row, never drop them. Old readers over new files degrade
gracefully; nothing is ever unrepresentable.
