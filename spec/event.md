# spec: event v1

The machine-precise schema for every event. Doc 02 is the narrative; this
file is the contract. Canonical encoding is RFC 8785 (JCS); canonical bytes
exclude the JSONL LF. A ledger line that parses but is not byte-identical to
its RFC-8785 re-encoding is invalid, not an alternate accepted spelling.
`fixtures/events/` holds the hand-authored oracle.

## Conventions

- Types: `str`, `int` (|n| < 2^53; larger schema values are `str` by that
  field's rule), `bool`, `[T]`, and `JsonValue`; `?` = optional (omitted,
  never null). **Schema-defined fields are never `null`** — absence is
  omission (R13-3).
- `JsonValue` (the foreign-payload carrier — R13-3) = full I-JSON:
  `null | bool | number | str | [JsonValue] | {str: JsonValue}`. Numbers
  are finite IEEE-754 doubles, serialized per RFC 8785 §3.2.2.3 (ES6
  shortest form); NaN/Infinity are illegal. Provider-authored values
  (tool arguments, state payloads, answers, invocations) are preserved
  semantically — `null` and non-integer numbers are legal **inside
  `JsonValue` only**, never as schema-defined fields. Kernel-authored
  figures that may exceed double precision use `str` fields (e.g. usage).
- `Block` (closed union): `{type: "text", text: str}` \| `{type:
  "reasoning", text: str}` \| `{type: "image", asset: str, mime: str,
  name?: str}` \|
  `{type: "tool-call", call: str, name: str, args: JsonValue}` \|
  `{type: "tool-result", call: str, content: [Block], error?: bool}`.
  An image `name`, when present, is display metadata preserved byte-for-byte
  across replay, fork, redact, and endpoint projection; it never participates
  in asset identity or provider rendering. It is 1...255 UTF-8 bytes and
  contains no Unicode control scalar.
- **Spill rule** (deterministic, validator-checkable — R13-3/R14-3): a
  field typed `X | spilled` inlines the value when its RFC 8785 encoding
  is ≤ 16384 bytes, and MUST otherwise be `spilled = {"$spill": {asset:
  str, bytes: int}}` where `bytes` is that encoding's byte length
  (> 16384) and the asset's content is exactly those bytes (a spilled
  `str` or `JsonValue` is stored JSON-encoded). **`$spill` is the
  reserved discriminant**: at a spill position, an object whose only key
  is `$spill` IS a spill reference; a foreign value of that shape never
  reaches a ledger — adapters reject it as malformed provider output
  (provider-adapter §Obligations). Validators recompute the inline bound,
  require `bytes > 16384`, and, when the asset is on hand, require its
  length to equal `bytes`.
- **Unknown fields** (R14-4): the v1 **envelope field set is closed** —
  new envelope fields require an envelope version bump. Any top-level key
  outside that set is therefore a payload field of its kind: known ones
  validate per the kind's table; unknown ones preserve-ignore (forward
  compatibility). There is no "unknown envelope field" error;
  required-envelope validation (every envelope field present and
  well-formed) is unaffected. Payload top-level keys are kernel-chosen —
  foreign JSON lives nested inside `JsonValue` fields, never top-level.
- `seqrange` = `{"from": int, "to": int}` inclusive, `from ≤ to`.
- `ts` = ISO8601 UTC string with milliseconds (`2026-08-26T09:00:00.000Z`).
- Ids: `run_id`, `attempt` (attempt id), `call` (tool-call id), `spawn_id`,
  `origin_key` are opaque non-empty `str`. Native thread/session ids are
  lower-case canonical UUID strings (D-64); new ids use UUIDv7. The UUID is the
  folder/genesis identity and the only id the kernel exposes upward — URI
  scheme/AS routing is supplied by the host ([Client interface](../docs/interfaces/client.md)).
- `origin_tuple` = `{principal: str, client: str, target: str, op: str,
  key: str}` — dedup identity (D-33).

## Envelope (every event)

| field | type | req | rule |
|---|---|---|---|
| `v` | int | ✔ | envelope version, `1` |
| `seq` | int | ✔ | per-file, starts 1, +1, no gaps |
| `kind` | str | ✔ | one of the kinds below or an extension kind |
| `ts` | str | ✔ | ISO8601 UTC |
| `turn` | int | turn-bound kinds only | absent on file-scoped kinds — no 0/null sentinels |
| `visibility` | str? | — | `model \| runtime`; overrides the kind default ([Model context](../docs/data/context.md)) |
| `anchor` | bool? | — | survives compaction verbatim |
| `supersedes` | [seqrange]? | — | first-class retraction |
| `origin_key` | str? | — | externally-originated mutations; full tuple recorded in payload where the kind requires it |
| `min_reader` | int? | — | reader gate (D-49) |

Turn-bound kinds: `turn_open, output, reasoning, tool_call, tool_result,
child_result, state, approval_request, approval_response,
effective_execution, tool_execution_started, spawn, attempt, attempt_dispatched, attempt_recovery,
settle, error`. File-scoped: `genesis, run_start, epoch,
input, checkpoint, compact, meta, stop_requested, queue_edit` — `input`
is a queue record (R14-1); its turn membership is recorded by the
consuming `turn_open` (steer: by `attempt.admits`).

## Kinds

### genesis (seq 1 only; barrier)
`{format: int, min_reader: int, min_writer: int, thread: str,
workspace: str, origin_key: str, origin_tuple, resume: "never" |
{"bounded": int}, parent?: {file: str, seq: int, spawn_id: str},
seed?: {source: str, kinds: [str], snapshot: {asset: str, digest: str}},
config: {digest: str}, instruction?: {digest: str}}`

`parent.spawn_id` is the durable child-side binding checked by the launch
lifecycle (R15-10).

**Seed snapshot asset format (R16 closure).** `seed.snapshot` is UTF-8 JSONL
with one LF-terminated **source event** per line, in strictly increasing
source-`seq` order. Each line is the source event's exact RFC-8785 canonical
bytes and MUST pass the v1 envelope + kind-payload validation that can be
checked in isolation. The snapshot is a projected subset, so its source seqs
need not start at 1 or be contiguous and cross-event constraints 1–10 are
not run over the asset. Every line's `kind` MUST appear in `seed.kinds`.
`seed.kinds` MUST equal the ASCII-lexicographically sorted, duplicate-free
set of kinds that actually occur in the asset; it records concrete event
kinds, never projection-policy labels such as `artifact` or `verify`.

The retained `v`/`seq`/`ts`/`turn`, correlation ids, visibility, and origin
fields are **source provenance only**: they never allocate child seqs or
turns, participate in the child's validation fold, admission set, dedup map,
or continuation identity. The child renderer consumes the records in asset
line order as imported, provider-independent historical items, using their
normalized model-visible payloads; source attempt/continuation/sealed
carriers are not imported as the child's provider baseline. The first child
epoch folds that imported history into its snapshot/profile.

The asset bytes include the final LF. `snapshot.digest` is the lowercase
hex SHA-256 of those exact bytes, and `snapshot.asset` MUST equal
`"sha256-" + snapshot.digest`. The asset follows D-45's reference protocol
and is durable **before** genesis. Every asset reference reachable from a
snapshot line MUST resolve inside the child folder and be made durable before
the snapshot; the reference order is leaf assets → seed snapshot →
genesis. GC, fork, and redact scanners MUST parse the seed snapshot as a
schema-declared carrier, mark/copy its referenced assets, and content-scan or
regenerate it on redact. This is context stored beside the ledger, not
additional child-ledger events (R15-5/R16).

### run_start (barrier)
`{run: str, mode: "ordinary" | "reconcile", recovery_ordinal: int,
binary: str, config_digest: str, instruction_digest: str,
launch_bindings_digest?: str, policy: str}`

`launch_bindings_digest`, when present, is the 64-character lowercase SHA-256
of the exact canonical [launch-bindings](launch-bindings.md) bytes. It is
mandatory for every Slice-8 launch that supplies a goal identity or dynamic
catalog and is an implicit reference to
`assets/sha256-<launch_bindings_digest>`. The asset is durable before this
barrier and participates in GC, fork, archive, and redact carrier scans. A
reconcile run reopens and verifies these exact bytes; it never reconstructs a
goal identity or live catalog from process state.

### epoch
`{id: str, reason: str, adapter: str, model: str, system: {asset: str,
digest: str}, tools: {asset: str, digest: str}, renderer: int, pending?:
{eligible: [int], withheld: [int]}}`

`system` and `tools` are the frozen head stored in full as content-addressed
assets, not merely hashed: a request must be reconstructable from the ledger
plus the objects it references, and a live MCP server's schemas are not
derivable from configuration and code. Both dedupe across epochs, so an
unchanged head costs one blob per thread. `reason` names what moved —
`initial`, `recovery`, `compaction`, `renderer_change`, `tool_profile_change`,
`system_change`, `model_change`.

**Authority** (R13-5): a durable epoch is authoritative from its append;
its durability is merely *guaranteed* by the next attempt barrier (a
torn-away trailing epoch never existed — the resuming worker re-derives
it, a pure function of the ledger). At attempt time the worker recomputes
the required profile: equal to the latest durable epoch's → bind that
`id`; otherwise append a new epoch and bind it. Attempt-less trailing
epochs are legal and inert.

### input (file-scoped queue record — R14-1; barrier when ack-bearing)
`{content: [Block], steer?: bool, origin_tuple, source?: "user" |
"cross_thread" | {thread: str}}` — contentful (`content` non-empty).
Envelope `origin_key` MUST equal `origin_tuple.key` (R12-4) — this
equality holds for every kind carrying both.
An input has no `turn`. It is **consumed** by the `turn_open` naming its
seq (non-steer), or — `steer: true` — by admission into its **target
turn**: the turn open at its append (R15-3). Only the target turn's
attempts may admit a steer; a steer whose target turn settles unadmitted
is **expired**: never runnable, never consumed (the queue projection
reports it). Runnability of queued inputs is per
[tail-lifecycle](tail-lifecycle.md).

### turn_open (turn-bound; the first event of every turn — R14-1)
`{trigger: "genesis" | {inputs: [int]}}` — `"genesis"` only for turn 1 of
a spawned child (work implied by genesis/seed); otherwise `inputs` lists,
in strictly ascending seq order, the consumed contentful non-steer input
seqs (≥ 1), exactly-once and prefix-complete (constraint 8). Not a
barrier: its durability rides the turn's next barrier, and a torn-away
`turn_open` takes its whole turn with it.

### queue_edit (file-scoped; barrier — ack-bearing)
`{origin_tuple}` + envelope `supersedes` over **unconsumed** queued/held
inputs (constraint 8 — R16r). Never a runnable trigger, never a release;
reorder = edit + new input(s) (R11-2).

### output (turn-bound; barrier — settles an attempt)
`{attempt: str, usage: Usage, content: [Block], sealed: {version: int,
adapter: str, fragments: str | spilled}, continuation?: {id: str},
final_answer?: bool}` — exactly one output settles a given attempt (D-43).
New provider outputs carry `final_answer`; false means the same turn continues.
Its absence in historical records is not retroactively reinterpreted.

### reasoning
`{attempt: str, content: str | spilled}`

### tool_call (barrier when side-effectful)
`{call: str, name: str, args: JsonValue | spilled, source: "provider",
attempt: str, execution_tracked?: bool, provider_call?: str}`
`call` is the unique durable identity. `provider_call`, when present, retains
its original provider wire identity. Provider IDs are response-scoped: a fresh
response may reuse an earlier completed call's wire identity, with or without
compaction. The worker derives a new local identity from attempt plus wire
identity; native sealed output remains unchanged and results render the wire
identity in response order. Unresolved reuse and same-response duplicates remain
errors; reuse is never evidence that a new response repeats an old side effect.

### effective_execution (barrier)
`{call: str, invocation: JsonValue | spilled}`

### tool_execution_started (barrier)
`{call: str}` — turn-bound, runtime-visible. Appended and synced after the
approval gate and immediately before entering the backend. It requires an
unresolved call and no unanswered approval. A backend hold precedes a later
approval response and a new execution start for answer consumption.

New worker-authored `tool_call` records carry `execution_tracked: true`.
For those calls, absence of a start after the latest hold proves that the
backend has not begun that execution. For older calls without that flag,
absence of a marker is not evidence of nonexecution. A start without a result
remains an ambiguous side effect and must not be blindly replayed.

### tool_result
`{call: str, outcome: "ok" | "error" | {aborted: "crash" | "recovered"} |
{denied: str} | {withheld: "scan_failed" | "quarantined"},
content?: [Block] | spilled, meta?: JsonValue}` — exactly one terminal
result per call.

### approval_request (barrier — the hold marker)
`{call: str, scope: str, question?: JsonValue}`

### approval_response (barrier — ack-bearing answer)
`{call: str, grant: bool, scope?: str, answer?: JsonValue, origin_tuple}`

### spawn (barrier)
`{child: str, call: str, spawn_id: str, resume: "never" |
{"bounded": int}, seed: {kinds: [str]}}`

### child_result
`{child: str, call: str, spawn_id: str, outcome: "completed" |
"interrupted" | "error" | "failed", summary?: str, artifacts?: [{asset:
str, mime: str}]}` — exactly one per `call`.

### attempt (barrier)
`{attempt: str, epoch: str, wire_digest: str, request: {asset: str, bytes:
int}, admits: [seqrange]}` — `request` is REQUIRED: the exact transmitted
request body (provider-runtime §Send ordering) as a thread asset durable
before this event; `bytes` is its length. An attempt without its request body
is not evidence of what was sent and is rejected.

### attempt_dispatched (barrier)
`{attempt: str}`

### attempt_recovery (barrier — adopt commit point)
`{attempt: str, decision: "adopt" | "recovery_epoch" | "resend" |
"not_dispatched" | "unresolved", inventory?: [str],
response?: {asset: str}}` — `adopt` REQUIRES `inventory` and `response`
(the synced recovered-response asset; completion never re-queries — R11-3).

### Usage (object carried by the outcome that settles an attempt)
`{availability: "reported" | "unavailable", input_tokens?: str,
output_tokens?: str, cache_read?: str, cache_miss?: str,
reasoning_tokens?: str, cost?: str}` — figures as strings; present only when
`reported`. There is no standalone usage event: the `output` or
`error{attempt}` that settles an attempt carries the attempt's usage, so a
settled outcome can never exist without it and no crash window separates the
two (the same placement as the predecessor's per-step `assistant/message`
usage). `cache_miss` is retained only when explicitly reported by the
provider; it is not inferred from input minus cached tokens. The v3 public
usage DTO exposes its existing fields; this additional ledger counter does
not add a public DTO field.

### error (barrier when it settles an attempt)
`{recoverable: bool, classification: "provider_terminal" |
"unresolved_dispatch" | "transport" | "rate_limit" | "tool" |
"internal", attempt?: str, usage?: Usage, detail?: str, sealed?: {version:
int, adapter: str, fragments: str | spilled}}` — `usage` is
required exactly when `attempt` is present (the error settles that attempt)
and forbidden otherwise. `unresolved_dispatch` is an attempt classification
ONLY; it is never a settle reason (R11-4).

`sealed` is the **partial carrier**: allowed only on a recoverable
`transport` error that settles an attempt, and written only when that
response had eagerly dispatched calls (its `tool_call`/`tool_result` records
are durable) and the stream had completed native items covering every such
call. It has the shape of `output.sealed` and holds, in output order, the
native items the response completed before the loss (for example a Responses
`reasoning` item with its `encrypted_content`, then the `function_call`). The
error itself stays runtime-visible; the renderer projects the carrier in the
error's position as the attempt's assistant items, skips that attempt's
`reasoning`/`tool_call` projections exactly as it does for a settled
`output`, and renders the eager results right after it. A provider that binds
a call to the items before it (DeepSeek thinking mode) therefore sees the
replay it produced, not a normalized reconstruction. A transport loss with no
eager call needs no carrier: nothing of that response is replayed.

### settle (barrier; exactly one per turn)
Discriminated union on `outcome`:
- `{outcome: "completed"}`
- `{outcome: "interrupted", reason: "user_stop" | "budget_tokens" |
  "budget_wall" | "recovered"}`
- `{outcome: "error", classification: "provider_terminal" | "transport" |
  "rate_limit" | "internal"}`
A direct final-answer settlement may carry `promoted_output_seq: int` at the
top level. It must reference an earlier output in the same turn and cannot
coexist with `validation`. The endpoint uses it to promote the final message.
An independent-validation settlement may instead carry
`validation: {outcome: "pass" | "inconclusive" | "not_required",
 promoted_output_seq: int, candidate_seq: int, decision_seq: int}`.
These positive references satisfy
`promoted_output_seq < candidate_seq < decision_seq < settle.seq`.
The validation writer verifies exact-turn candidate/decision identity and promotes
the immutable output by reference. A failed verdict is repair input, never a
successful terminal validation outcome. Interrupted/error settlements cannot
carry this completed-validation payload. Legacy settlements without the optional
payload remain readable. The settle after an unresolved-dispatch recovery is
`interrupted/recovered` (R11-4).

### checkpoint (barrier)
`{covers: int, summary: str | spilled}` — a plain covering marker: the
boundary a later `compact` may cover. It names no state, expires no origin
key, and never seeds replay; a reader folds every ledger from genesis and
does not skip covered events. Older records carrying further fields
(`state_digest`, `dependency_digest`, `keys`, `key_floor`, `state`) remain
valid markers; those fields are ignored.

### compact (barrier and ack-bearing when manually originated — R13-4)
`{covers: [seqrange], summary: str | spilled, origin_tuple?}` —
`origin_tuple` present exactly when manually originated (envelope
`origin_key` = its `key`; receipt/dedup per worker-control); absent for
worker-initiated auto-compaction. A manual compact is authored by the live
worker at its next yield, or — when no worker is alive — by the supervisor
under the line lock in the same shape (tail-lifecycle D-61 whitelist;
command-catalog reserved verb `compact` is one such origin, op
`commands/run`). May cover only checkpoint-covered ranges; always closes the
epoch.

Optional `summary_request`: `{bundle: {covers: [seqrange], sha256: str,
bytes?: int, truncated?: bool}, accepted: bool, evidence_refs?: [int],
reason?: str, usage?, model?: str}` — the telemetry of the summary request
the author ran before writing the compact. The bundle is the planned covers
frozen before the request, `sha256` over their canonical bytes
(`bytes`/`truncated` describe the rendering the compactor read, capped at
60 % of the model window). The request runs in the compactor role with
`summary_artifact` as its only tool; admission
(`engine::admit_summary_artifact`) accepts the artifact only when
`evidence_refs` are `seq` addresses inside the bundle, unique and non-empty,
and the continuation is non-empty and within the summary bound. An accepted
continuation is written as `summary` (prefixed `[compacted history]`); a
provider failure, a missing or rejected artifact, or a bundle that no longer
matches the plan at write time is `accepted: false` with `reason`, and
`summary` is the deterministic quoted history. Replay is unaffected:
`summary` is the authority either way. A worker-authored manual compact at a
yield holds no provider lease and carries no `summary_request`; the worker's
auto-compaction (preflight, or after a context overflow) and the supervisor's
idle-line manual compact do.

### stop_requested (barrier)
`{generation: int, origin_tuple}`

### meta (supersedable; barrier and ack-bearing when externally originated — R17k)
`{title?: str, labels?: [str], ownership?: int, upgrade?: {min_reader:
int, min_writer: int}, origin_tuple?}` — `origin_tuple` present exactly
when externally originated (envelope `origin_key` = its `key`;
receipt/dedup per worker-control — the manual-compact pattern, R13-4);
`ownership`/`upgrade` are never client-settable; an upgrade meta is a
barrier (R4-3).

### state (model-visible; open subkinds)
`{subkind: str, payload: JsonValue}` — unknown subkinds render generically.

Defined subkind `tool_continuation` (written with `visibility: "runtime"`;
never model context): `payload: {call: str, continuation_id: hex64, step: int,
action: "bind" | "query" | "update" | "cancel", state: JsonValue | spilled}`
— one record per durable step of a remote tool continuation
(worker-control §Tool continuation). Step 0 is the pending binding from
the `tool_control_result`; step `n ≥ 1` is the receipt of continuation request
`n`. A `park` record repeats the current step with `poll_after` (RFC 3339 UTC
instant of the next poll) and `interval_ms` (the interval it used); the worker
exits cleanly after writing it and the line resumes when the instant passes
(tail-lifecycle `continuation_wait_until`). Steps for one `call` never regress
and name one `continuation_id`; the call still pairs 1:1 with exactly one
terminal `tool_result` (constraint 3 — a replacement that `supersedes` the
result it retracts keeps that pairing 1:1), and an unpaired call with continuation
records is unresolved work that a recovery run resumes rather than aborts.

Defined subkind `provider_admission` (written with `visibility: "runtime"`;
never model context): `payload: {attempt: str, classification: "rate_limit" | "transport",
next_attempt_at: str, wait_ms: int, retries_left: int, declared_retry_after:
bool}` — the durable admission wait the worker honors before retrying a
rate-limited attempt (HTTP 429, or the Cloudflare AI Gateway wholesale limit:
HTTP 402 whose body reports "Wholesale rate limit exceeded") or a recoverable
transport failure, including connection failures. It follows the attempt's
recoverable `error` with the same classification and precedes the
retry's `attempt`; `next_attempt_at` is the provider `Retry-After` when
declared, otherwise a doubling backoff from one second, clamped to thirty
seconds. A worker that dies during the wait leaves the retry to the line's
recovery run (tail-lifecycle `admission_wait_until`).

## Visibility defaults (normative — R12-5)

| default | kinds |
|---|---|
| model | input, output, tool_call, tool_result, child_result, state |
| dialect-dependent | reasoning |
| runtime | approval_request, approval_response, effective_execution, tool_execution_started, error (all classifications — R13-9), queue_edit, and every unknown kind (e.g. `note`) |
| never | genesis, run_start, epoch, turn_open, checkpoint, compact, meta, stop_requested, attempt, attempt_dispatched, attempt_recovery, settle |

`visibility` on the envelope may demote model→runtime; it may never promote
a never-kind. Unknown kinds are runtime; unknown `state` subkinds render
generically (F20).

## Tail framing and repair (normative — R14-9/R15-6)

A **barrier** event's F_FULLFSYNC makes the entire file prefix through it
durable and ordered (D-45); between barriers, appended lines are
best-effort and may be lost or persisted **out of order** by a crash. A
ledger's authoritative content is therefore its **longest valid prefix**:
complete LF-terminated lines that parse and validate, up to the first
invalid point — no final LF, invalid UTF-8, invalid JSON, non-canonical JSON,
a bare LF, or a failing line. Everything past that point is discarded: a valid-looking
line after an invalid one is necessarily post-barrier best-effort and
unreachable. Readers ignore the discarded suffix (it contains no events
and is never reported); the flock winner MUST, before its first append,
truncate the file to the end of the longest valid prefix and fsync; the
next event takes seq = last valid seq + 1. A non-barrier `turn_open` is
safe under this rule: losing it discards its whole turn. If out-of-band
facts — issued receipts, checkpoints, other files' references — point
beyond the longest valid prefix, that is **corruption**, not crash loss:
fail-stop and surface, never silent repair. `fixtures/torn/` is the
oracle.

## Cross-event constraints

1. Exactly one final `settle` per `turn` (holds park — a parked turn has
   no settle); post-settle legality is rule 8.
2. `attempt` chain per attempt id: `attempt` → `attempt_dispatched`? →
   `attempt_recovery`? (at most one, **any decision** — R13-2) →
   (`output` | `error{attempt}`, carrying the attempt's `usage`) → nothing
   else. Non-adopt recoveries close with `error {usage: {unavailable}}` per
   provider-adapter's closure transaction; adopt additionally requires its
   `response` asset durable before the `attempt_recovery` barrier and
   appends its per-call events after the wire `attempt_settled` (a message,
   never an event).
3. Every `tool_call.call` is unique in the file; `tool_call`/`tool_result`
   pair 1:1 by `call`; `spawn`/`child_result`
   pair 1:1 by `call`; `approval_request`/`approval_response` pair ≤1:1 by
   `call`.
4. `admits` ranges cover only host-born model-visible events; coverage
   counts only output-settled attempts (D-29). **Input eligibility**
   (R15-3): a non-steer `input` is admissible only by attempts of the
   turn whose `turn_open` names it; a steer input only by attempts of its
   target turn (the turn open at its append); every other queued input is
   ineligible regardless of visibility.
5. Every barrier kind listed above — including `checkpoint` — is in D-45's
   list; the two lists are one list.
6. **Origin equality**: an event whose payload carries a **top-level**
   `origin_tuple` field (its own origin — nested tuple values do not count)
   MUST carry envelope
   `origin_key` equal to that tuple's `key` (R12-4/9; strengthened R16r —
   a payload origin without the envelope key would be invisible to an
   envelope-scanning dedup index).
7. **Backward references only**: `supersedes` ranges, `compact.covers`,
   and checkpoint coverage reference seqs strictly less than the referring
   event's seq (R12-9); `attempt.epoch` names an `epoch` event with
   smaller seq (R13-5); `turn_open.trigger.inputs` name `input` events
   with seq strictly smaller than the turn_open's seq (R15-1); every seq
   covered by `attempt.admits` is strictly smaller than the attempt event's
   seq (R16 — a request cannot admit a future event); `epoch.pending`
   lists likewise name only seqs strictly smaller than their event's seq
   (R16r — same causality class).
8. **Turn allocation & consumption** (R14-1/R14-2): turn N's first event
   is its `turn_open` (turns start at 1, +1, no gaps); every other
   turn-bound event's `turn` equals the latest opened turn. A
   `turn_open.trigger.inputs` list is strictly ascending, names only
   **live** (unsuperseded at the turn_open's position) contentful
   non-steer `input` seqs never consumed before (exactly-once), and is
   **prefix-complete**: it includes every live
   (unsuperseded, unexpired) unconsumed contentful non-steer input with
   seq below its maximum. **Trigger check** (R15-4): the maximum named
   seq must be runnable at the turn_open's position — it postdates the
   closure point of the latest prior stop generation
   ([tail-lifecycle](tail-lifecycle.md)) and no hold is open there;
   earlier listed inputs need only be live and unconsumed. `trigger:
   "genesis"` is legal only for turn 1 of a file whose genesis carries
   `parent`. Turn N+1 cannot open until turn N has its final settle; after a
   turn's settle, only file-scoped events or the next turn's events are
   legal. Conversely, a `queue_edit` may supersede only inputs
   **unconsumed at its position** (R16r) — the consumption↔retraction
   race resolves by file order: whichever lands second is invalid.
9. **Ordinal bounds**: `run_start.recovery_ordinal` equals the count of
   prior reconcile-mode run_starts since the last settle; `bounded(n)`
   exhausts at ordinal ≥ n.
10. A validator MUST enforce 1–9 as a single fold over the file; two
    conforming validators accept exactly the same ledgers (R12-9).
