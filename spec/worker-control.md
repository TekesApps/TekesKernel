# spec: worker-control

The complete inner wire protocol (D-58) between the supervisor and a worker:
JSONL both directions, one JSON object per LF-terminated line, UTF-8. The
protocol version is negotiated first (an integer carried by `hello` and
`selected`; the current version is **2**); every later line is one of the
messages below. `fixtures/wire/` is the oracle for the base messages and
`fixtures/wire/durable/` for the durable control messages.
**Type imports (normative)**: `Block`, `origin_tuple`, and `seqrange` are
exactly event's definitions (R12-6); envelope `origin_key` == the
carried tuple's `key` everywhere (R12-4).

## Negotiation (before locks, before any append)

1. Worker → supervisor, first line:
   `{"hello": {"proto": "tekes-worker", "min": 2, "max": 2}}`
2. Supervisor → worker, first line:
   `{"selected": {"version": 2}}` — highest mutual; no mutual → the
   supervisor writes `{"reject": {"reason": "no_mutual_version"}}`, the
   worker exits **76**, appends nothing, and the supervisor quarantines the
   (binary, range) tuple (R7-6).

Selecting the version is also the supervisor's capability promise that its
production handler implements the complete fixed supervisor-owned set
`context`, `context_get`, `job`, `report`, `subagent`, and `task`. A
fail-closed unavailable handler is valid for tests or library embedding only;
a production supervisor must not select the version with that fallback
installed. A worker whose effective catalog exposes any `supervisor_control`
tool, and a supervisor advertising `session.updateQueue`, both require the
durable control messages below; every durable control message is defined at
the negotiated version and a peer that negotiated anything else neither sends
nor accepts one.

A worker launched solely to recover an incomplete endpoint queue transaction
uses the selection extension
`{"selected":{"startup":"queue-transaction","version":2}}`. The supervisor
then sends the byte-identical `queue_transaction` as the next line, before any
other post-selection control. The worker buffers that line before opening
snapshots or locks; it does not validate its tail-dependent fields yet. A
normal selection omits `startup`. Unknown startup values or a missing,
malformed, or non-transaction next line are protocol failure: exit 76 before
append when detected before locking, otherwise fail-stop without a settle and
leave the durable management operation incomplete for a later candidate.

The quarantine is a process-table fail-stop keyed by the SHA-256 digest of the
worker executable bytes plus the advertised `[min, max]` range. During that
supervisor lifetime, no ensure or sweep may launch the same executable bytes
again; atomically replacing the executable changes the tuple. A supervisor
restart may probe the tuple once, then reconstructs the same quarantine from
the rejected `hello`. Quarantine is not semantic thread truth.

The supervisor waits at most **10 seconds after the launcher returns the
spawned candidate** for the
LF-terminated `hello`, whose encoded line is at most **4096 bytes including
LF**. Timeout, early EOF, an oversized or malformed line, and any pre-adoption
setup failure are protocol failures: the supervisor kills and waits the
candidate, closes every pipe and private credential-broker channel, and
returns only after those resources are reaped. No ledger append is authorized
before negotiation succeeds.

## Supervisor → worker (stdin)

| message | shape | rules |
|---|---|---|
| `input` | `{"input": {delivery: str, origin: origin_tuple, content: [Block], steer?: bool, assets?: [{asset, mime}]}}` | worker appends the file-scoped `input` queue record (staged assets already durable), then sends `receipt`; consumption is by `turn_open` / steer admission (event — R14-1) |
| `approval_response` | `{"approval_response": {delivery: str, origin: origin_tuple, call: str, grant: bool, answer?: JsonValue}}` | worker echoes the **barrier** event, pairs per tail-lifecycle, and sends `receipt` only after that barrier completes |
| `queue_edit` | `{"queue_edit": {delivery: str, origin: origin_tuple, supersedes: [seqrange]}}` | worker appends `queue_edit`, sends `receipt` |
| `stop` | `{"stop": {delivery: str, origin: origin_tuple, generation: int}}` | worker echoes `stop_requested`, sends `receipt`, then acts per D-37 |
| `lease` | `{"lease": {attempt: str, granted: bool}}` | reply to `lease_request`; denial = wait or abandon per budget |
| `ping` | `{"ping": {id: str}}` | worker replies `pong` |
| `compact` | `{"compact": {delivery: str, origin: origin_tuple}}` | manual compaction request (D-39); worker runs the compact projection at the next yield, appends the `compact` event **with the message's origin tuple** (event — R13-4), then sends `receipt` |
| `meta` | `{"meta": {delivery: str, origin: origin_tuple, title?: str, labels?: [str]}}` | worker appends the keyed supersedable `meta` (**barrier**, R17k), sends `receipt`; `ownership`/`upgrade` are never accepted from clients |
| `launch_result` | `{"launch_result": {child: str, spawn_id: str, ok: bool, error?: str}}` | reply to `launch_child`, correlated by `(child, spawn_id)` (R14-8); on `ok: false` the worker records `child_result {failed}` |

## Worker → supervisor (stdout)

| message | shape | rules |
|---|---|---|
| `receipt` | `{"receipt": {delivery: str, seq: int, deduplicated: bool}}` | **non-coalescible**; sent only after the event's required durability; the supervisor acks its client only on this (D-48) |
| `attempt_settled` | `{"attempt_settled": {attempt: str, outcome_seq: int}}` | **non-coalescible**; after the outcome barrier; sole lease release; stale/unknown = idempotent no-op at the supervisor (R8-3) |
| `lease_request` | `{"lease_request": {attempt: str, class: str}}` | before the attempt barrier (R4-6) |
| `appended` | `{"appended": {seq: int}}` | doorbell; coalescible. The worker sends it after each run phase that ended with a durable append (queue transaction, cancel-before-start settle, recovered tool calls, reconcile, the post-turn settle/park, every delivered control event). `seq` is the ledger high-water at send time and is informational: the supervisor re-reads the file and projects only what the endpoint journal lacks; its sweep is the retry when the doorbell or its projection is lost |
| `frame` | `{"frame": {attempt: str, channel: "text" \| "reasoning" \| "tool", block: int, delta: str, call_id?: str, name?: str}}` | direct presentation stream (`block` = index in the attempt's forming content); once accepted it goes to the transient UI channel immediately, while JSONL persistence runs on the independent batched storage lane defined by session-endpoint §Journal projection. `tool` requires `call_id`, may carry `name` on any fragment, and maps them to the endpoint `id`/`name`; other channels forbid both fields |
| `state` | `{"state": {phase: str}}` | reserved: decodable and ignored by the supervisor, emitted by no worker; a future use must define its consumer first |
| `pong` | `{"pong": {id: str}}` | |
| `launch_child` | `{"launch_child": {child: str, spawn_id: str, resume: "never" \| {"bounded": int}}}` | after durable child genesis + parent `spawn` ([Turn flow](../docs/flows/turn.md)); answered by `launch_result`; retry-safe — the supervisor dedups on `(child, spawn_id)` (ensure semantics); a child id binds to exactly one spawn_id (its genesis origin) — reuse is invalid (R14-8) |

Tool frames may additionally carry `arguments_complete: true` when the provider
has completed that call's arguments. The marker is presentation-only, carries
an empty delta, and is forbidden on text/reasoning channels. Absence retains the
legacy fragment behavior. It neither authorizes execution nor settles a turn:
the worker's eager dispatch is keyed on the provider's own completion event and
its durable `tool_call` (tool-runtime), never on this frame, and the frame
always leaves the pipe before that record exists.

Any frame may carry `ledger_seq`, the worker ledger's last durable seq at the
moment the frame was written. It is a causal cut, not a receipt: when the
supervisor refreshes its projection because of this frame, it publishes durable
events only up to `ledger_seq` (or up to what the endpoint journal already
holds, whichever is later) so that events the worker appended *after* sending
the frame — an eagerly dispatched `tool_call`, its `tool_result`, the
response's own `output` — never overtake the frame on the public
stream. Absence retains the legacy boundary: the refresh stops before the
attempt's first `tool_call`/`output`/`error` record.

## Launch lifecycle (R13-7)

Keyed by `(child, spawn_id)`:

- **Parent duty**: every parent run — ordinary or recovery — whose ledger
  holds a `spawn` without its `child_result` (re-)sends `launch_child`.
  The send is an idempotent ensure, safe across supervisor restarts and
  duplicate deliveries.
- **Supervisor duty** on `launch_child`: first complete any pending stop
  propagation to the child — the tree's durable root stop must have its
  durable child echo before classification (D-37 — R14-6); then classify
  per [tail-lifecycle](tail-lifecycle.md) and reply — running, or a
  spawnable state (**unstarted** / runnable / recovery_needed /
  answered_hold / stopped_active): ensure a candidate (dedup by
  `(child, spawn_id)`; a stopped child's candidate is reconcile-mode per
  the matrix — a stopped **unstarted** child's candidate performs the
  cancel-before-start settle, never first work) and send `launch_result
  {ok: true, spawn_id}`; settled or parked_hold: `{ok: true, spawn_id}`
  with **no spawn**; child folder missing: `{ok: false, spawn_id, error:
  "missing"}`; request `spawn_id` differing from the child genesis's
  `parent.spawn_id` binding: `{ok: false, spawn_id, error:
  "spawn_mismatch"}` (R15-10) — the parent appends `child_result
  {failed}`.
- **Dependency admission**: `launch_child` from the live durable parent is a
  structured dependency override, not ordinary worker admission. It starts
  the direct child even when the parent's own slot has reached
  `max_workers`; the parent does no other semantic work while tailing. The
  same rule composes for bounded descendants. An unrelated pending worker has
  no override and MUST NOT be started merely because one child in the waiting
  chain exits ([config](config.md)).
- **Result semantics**: `ok: true` acknowledges liveness-or-terminality
  only. The parent observes the child's outcome by tailing the child
  ledger (its right as spawner) and appends **exactly one** `child_result`
  when the child's root final settle is durable; on `ok: false` it appends
  `child_result {failed}`. A parent crash before `child_result` simply
  re-enters this table on its next run.

## Rules

- **Correlation**: `delivery` ids are supervisor-generated and unique per
  delivery attempt; retries of the same origin tuple use new delivery ids;
  the worker dedups on the origin tuple, not the delivery id, and re-acks
  the original seq with `deduplicated: true`.
- **Unknown message**: a receiver MUST ignore an unknown top-level key and
  continue (forward compatibility); a malformed line (non-JSON, multiple
  keys) is a protocol error — worker exits 76 / supervisor kills and
  reconciles.
- **Backpressure**: stdin is read continuously in every phase (R2-8);
  stdout writes are blocking — the supervisor must drain or the worker may
  stall (mirror-class messages may be dropped by the supervisor, never by
  the worker; non-coalescible messages are never dropped by either side).
- **EOF**: stdin EOF = supervisor death (D-41): cancel phase, per
  [tail-lifecycle](tail-lifecycle.md) exit paths. Worker exit closes
  stdout; the supervisor treats missing receipts as retryable (Idempotency rules §Rule 2).
- **Versioning**: the protocol version is the `hello`/`selected` integer
  (currently 2). Additive fields never bump it; new message types do; unknown
  fields are preserved-ignored. There is no other version anywhere in the
  protocol's names.

## Durable control: endpoint queue transaction

```text
queue_transaction {
  delivery: str,
  rpc_id: str,
  target_seq: int,
  retract_origin: origin_tuple,
  action:
    {kind:"edit", replacement_origin:origin_tuple,
     content:[Block], steer:bool, assets?:[{asset,mime}]} |
    {kind:"remove"} |
    {kind:"steer", replacement_origin:origin_tuple}
}
```

The target is one live unconsumed input. `retract_origin.key` is
`rpc_id+"/retract"`; each replacement key is
`rpc_id+"/replacement"`. Edit carries replacement content. Steer copies the
target's exact content/assets and authors the replacement with `steer:true`.
For edit, `steer` equals the target input's steer bit, so editing pending
steering remains steering; for a queued input it is false. Remove accepts
either placement and has no replacement. The steer action itself requires a
non-steer queued target whose current turn still admits steering.

The worker validates the target and all origin tuples while holding the line
lock. It appends the keyed `queue_edit` and completes its barrier, then for
edit/steer appends the keyed replacement input and completes that barrier,
without running a turn, accepting another delivery, or emitting a queue frame
between them. A crash may leave only the retraction durable; this is an
incomplete management transaction, not a successful queue mutation.
On retry, origin-key lookup yields exactly one of: neither event (append both),
retraction only (append the byte-identical replacement), or both (re-ack).
Replacement-without-retraction or mismatched bytes is corruption and fail-stop.

When no holder exists or the first worker dies mid-transaction, the supervisor
starts a reconcile-only worker under tail-lifecycle's external management pre-
gate using the startup extension above. After repair/replay and under the line
lock, that worker appends and syncs its normal
`run_start {mode:"reconcile"}`, then executes the buffered transaction as the
**highest-priority startup step**. It bypasses generic tail reconciliation,
provider/tool work, turn opening, and settlement; it completes only the missing
event(s), returns the result below, and exits. A later ordinary tail-triage pass
handles any unrelated recovery work. Failure before a committed or rejected
result is fail-stop with no settle and leaves the operation pre-gate in force.

The only completion message is:

```text
queue_transaction_result {
  delivery: str,
  outcome:
    {kind:"committed", first_seq:int, last_seq:int, deduplicated:bool} |
    {kind:"rejected", code:"queue-item-not-found"|"steer-unavailable"} |
    {kind:"rejected", code:"attachment-error", reason:str}
}
```

The worker sends `committed` only after the complete pair (or remove event) is
durable; for remove, `first_seq == last_seq`. A target consumed/superseded
before the worker's under-lock validation is `queue-item-not-found`; an expired
steer window is `steer-unavailable`. `attachment-error` is available only when
the transaction carries or copies an asset whose closed endpoint attachment
validation fails; native text-only edit validation normally rejects before the
operation is prepared. A rejection authors **zero** semantic events. The
supervisor first durably records the management operation's exact complete
error response, then releases the pre-gate; retry returns those bytes and never
revalidates against a later tail. This message is non-coalescible in every arm.
A plain `receipt` for either child event cannot complete the endpoint operation.

## Durable control: tool control (worker → supervisor)

```text
tool_control {
  request_id: str,
  session: uuid,
  thread: uuid,
  turn: int,
  call_id: str,
  name: str,
  arguments: JsonValue
}
```

`request_id` is the lowercase hex SHA-256 of:

```text
"tekes-tool-control-v2\0" || session-uuid || 0x00 ||
thread-uuid || 0x00 ||
decimal-turn || 0x00 || call-id
```

`session` is the containing root thread-folder UUID. `thread` is the line
genesis UUID: it equals `session` for `main.jsonl` and is the child UUID for a
flat child line. The supervisor resolves only `main.jsonl` or the UUID-derived
child filename inside `threads/<session>/`; no caller-provided path participates.

All text components are UTF-8; `session-uuid` and `thread-uuid` are canonical lowercase hyphenated
form and `decimal-turn` is unsigned base-10 without leading zeroes.
The message's `session`, `thread`, and `turn` are the exact preimage values and
MUST match the worker process binding and paired event envelope. Before receipt
lookup or backend execution, the supervisor verifies the selected line's
genesis UUID and paired durable `tool_call` or `effective_execution`. It is therefore stable
across worker/supervisor restart for one durable call and independently
validator-checkable.
`call_id` and `name` must equal the paired durable `tool_call`; `arguments`
must equal the durable effective invocation (`effective_execution` when
present, otherwise `tool_call`). A mismatch is a protocol error and no backend
runs.

## Durable control: tool-control result (supervisor → worker)

Exactly one of:

```text
tool_control_result {
  request_id: str,
  call_id: str,
  value: JsonValue
}

tool_control_result {
  request_id: str,
  call_id: str,
  error: {
    code: "unsupported" | "denied" | "not_found" |
          "conflict" | "unavailable" | "timeout" |
          "effect_unknown" | "effect_conflicted" | "internal",
    message: str,
    retryable: bool
  }
}

tool_control_result {
  request_id: str,
  call_id: str,
  pending: {continuation_id: hex64, next_step: 1, state: JsonValue}
}
```

`pending` is the third, host-owned arm of the union: the supervisor bound a
remote continuation (an MCP task, mcp-runtime §Task results) instead of a
terminal value. The initial request keeps its immutable receipt; `state` is the
remote task state at binding. A `value` can never impersonate this arm.

## Tool continuation (worker ↔ supervisor)

After a `pending` result the worker drives the continuation with steps:

```text
tool_continuation {
  format: 1, request_id: hex64,
  original: tool_control,            # the exact initial request
  continuation_id: hex64, step: int ≥ 1,
  action: {operation: "query"} | {operation: "update", input_responses: JsonValue}
        | {operation: "cancel"}
}

tool_continuation_result {
  request_id: hex64, call_id: str,
  result: {outcome: "pending", continuation_id: hex64, next_step: step + 1, state: JsonValue}
        | {outcome: "completed", value: JsonValue}
        | {outcome: "failed", error: <tool_control error>}
}
```

`request_id` is SHA-256 over `"tekes-tool-continuation-v1\0"` and the
canonical `(original.request_id, continuation_id, step)`. A step's identity
is stable: replaying it returns the immutable receipt without a remote
request; changing its action or the original arguments conflicts. Steps are
strictly sequential (`step` must equal the previous pending `next_step`); no
step follows a terminal receipt. The worker records every pending response as
a durable `state{subkind: tool_continuation}` (event) before requesting the
next step, so a restarted worker resumes at the durable step and the initial
side effect never repeats. `input_required` is published as an ordinary
`approval_request` with scope `mcp_task_input` whose answer becomes exactly one
`update` step; a stop dispatches `cancel` and terminalizes the call as an
error carrying the remote reply. The call gets exactly one `tool_result`, on
`completed` (the embedded tool result, `isError=true` mapped to a failure) or
`failed`.

The result is a backend response, not a semantic `tool_result`. The worker
validates it, performs post-hooks/secret scan/spill, then authors the one event.
`internal` details never include payloads or secrets.

## Dedup and lifetime

Every tool-control request has one durable record under the requesting
thread folder at `control/<first-two-hex>/<request_id>.json`: the complete
request tuple (the intent) and, once settled, the exact response (the
receipt). For an effectful `supervisor_control` dynamic tool, the supervisor
first publishes the intent there. `request_id` is also the
host-owned business operation/idempotency key delivered to the external
authority. The effect may begin only after that publication barrier. A
pre-existing intent without a receipt invokes the declared authoritative
query and consumes exactly one closed result:

- `confirmed {value}` publishes that value as the receipt;
- `not_found` permits one execution with the same key;
- `unknown {reason}` returns `effect_unknown`, retains the intent, and stops;
- `conflicted {reason}` returns `effect_conflicted`, retains the intent, and
  stops for manual review.

An effectful dynamic tool without this query authority returns
`effect_unknown` before its backend runs. Timeout/transport loss or explicit
cancellation after dispatch is `effect_unknown`, not proof of failure, and
enters the same reconciliation path. Cancellation already set before dispatch
remains an ordinary cancelled call because no external operation began.
Cancelling in-flight work rotates the MCP cancellation generation before
signalling the old token; reconciliation and future calls use the fresh token.

Before replying, the supervisor applies the same effective secret/redaction
policy to the backend value (withholding rather than persisting a match), then
publishes the settled record (the receipt) at the same
`control/<first-two-hex>/<request_id>.json` path using temp-write, file
sync, rename and parent-directory sync. It contains the complete request tuple
and exact result/error bytes. A thread folder written before the intent and
receipt carriers were merged (`control-intents/`, `control-receipts/`) is
folded into `control/` under the carrier lock on first use; the record bytes
are unchanged. This closed internal protocol receipt belongs to
D-1's protocol-carrier authority class; it is never semantic history or worker
replay input. A live bounded cache may accelerate it but is not authority.
Repeating an identical `(request_id, session, thread, turn, call_id, name, arguments)`
returns the original bytes from that receipt after any supervisor restart.
Reuse of `request_id` with different fields returns `conflict` and performs no
work. Receipt GC is permitted only after the paired thread has a durable
terminal `tool_result` and no recovery run can still request the id. Ownerless
job brokers retain their own stronger lifecycle records.

EOF, malformed or unknown-required durable control messages, and
backpressure follow the rules above. A lost result leaves the worker's durable call unpaired; recovery queries
the named backend by the same request id or records the backend-specific
aborted/unknown outcome. It never blindly starts a second effect.

`fixtures/wire/durable/cases.canonical.json` names the exact transcript and
receipt artifacts for tool-control success/error, identical re-ack, conflict,
EOF, queue transaction success, management startup, under-lock rejection, and
crash/race recovery completion; the manifest and registry are checked in both
directions (`scripts/check-worker-control-fixtures.py`).

### No presentation usage preview

A worker `frame` carries no token count. Output usage has one owner: the `usage`
object on the durable `output` event, written from the normalized provider terminal.
A mid-stream provider usage report is not forwarded as a frame, and no local
character-count estimate is produced; the endpoint's transient `assistant/chunk`
therefore has no `usagePreview`. (A weighted-scalar estimate with `≈` presentation
existed until 2026-09-20 and was removed: it was a second, guessed owner of a fact
the provider reports exactly.)
