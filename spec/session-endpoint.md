# spec: Session Endpoint

The one boundary between TekesKernel and its Client: the public multiplexed
stream and unary mutations (§Routes and streams), the durable projection of
the semantic ledger those streams deliver (§Journal projection), the physical
loopback listener that carries them (§Transport listener), and the management
operations, idempotency and recovery behind every mutation (§Management
authority and recovery). No file, module, directory or protocol under this
contract carries a version in its name; the only version is the
`protocolVersion` integer negotiated on the wire.

Imported authorities: the Client-owned DTO corpus under
`fixtures/endpoint/authority/` (pinned by `authority-lock.canonical.json`)
owns public DTO, `SessionEvent`, history, mux and stitching semantics; this
file owns Kernel's routes, projection, carrier and author paths. If they
disagree, the imported contract wins and Kernel fails closed.

## Routes and streams

The public protocol is the multiplexed stream and its unary mutations below;
`protocolVersion` is the exact compatibility integer negotiated in the `ready`
frame and the only place a version number appears. The internal projection
journal is `endpoint.jsonl`.

### Compatibility and capabilities

The only public stream route is authenticated WebSocket
`GET /api/remote.mux`. Its first application frame is:

```json
{
  "type": "ready",
  "generation": "1",
  "host": {
    "protocolVersion": 3,
    "product": {"name": "TekesKernel", "version": "<diagnostic build>"},
    "capabilities": [
      "actionable-sync", "session-control-sync", "session-inventory-sync",
      "session-journal", "workspace-sync"
    ],
    "cwd": "/...", "attachedSessions": 0,
    "home": "/...",
    "canOpenPath": false
  }
}
```

Compatibility is the exact integer `protocolVersion == 3` plus the closed
required capability set. `product.version` is diagnostic and MUST NOT be an
allowlist gate. Authentication, loopback restriction, I-JSON, 16 MiB frame
limit, admission, drain, and secret-redaction rules remain those of the native
endpoint carrier.

The public unary mutations are `workspace.create`, `workspace.rename`, `workspace.relocate`,
`workspace.archiveSession`, `workspace.unarchiveSession`, `session.create`,
`session.prompt`, `session.updateQueue`, `session.cancel`, `session.rename`,
`session.fork`, `session.discard`, `session.attachment`, `session.models`,
`models.list`, and `session.selectModel`. Public `host.describe`, `workspace.list`,
`session.list`, `session.history`, `/api/events.mux`, `/api/events.host`, and
`/api/respond` do not exist in V3.

### Built-in resource and recovery extensions

Authenticated POST routes accept registered slash-separated method names as
well as dot-separated names. `skills/list` and `commands/list` accept
`{"sessionId":"..."}` to resolve the session's durable workspace and folder
binding. Their results use the current Client `SessionSkill`/`SessionCommand`
fields. Empty requests remain the legacy global-catalog form; the native Client
uses the session form. `commands/run` resolves the same session catalog, admits
input through the session lifecycle gate, and returns the durable command key
as `commandId`. Unknown commands return `command-not-found`; archived sessions
return `session-archived` before catalog resolution.

`sessions.recover` takes `{"sessionIds":["..."]}` and returns
`{"recoveredSessionIds":["..."]}` after releasing those sessions. The native
Client calls it after control and actionable baselines are installed. Built-in
startup records existing sessions as deferred: projection repair remains
enabled, but boot/periodic sweeps cannot restart their workers until explicitly
released by this method. Repeated recovery retains an already-live worker.

### Multiplexing

Client frames use a client-chosen, nonempty, connection-unique `streamId`:

```text
open {streamId,target}
close {streamId}
journal-page {requestId,address,throughSequence,beforeSequence?,maxMessages}
actionable-respond {requestId,actionableId,expectedRevision,outcome}
```

The five targets are `workspace`, `session-inventory`, `session-journal`,
`session-control`, and `actionables`. Journal target additionally carries
`address:{sessionId}` and `maxMessages` in `1...500`; the normal initial value
is 50. A stream ID is scoped to one physical generation and cannot be reused
until closed. Connection loss cancels all logical streams and their cursors.

Server frames are `ready`, `stream`, `journal-page-result`,
`actionable-response-result`, or `error`. `stream` carries the matching
`streamId` and a typed semantic `frame`. Errors always contain
`category/code/message/details` and optional `retryability`; DSH- or
Kernel-private correlation IDs never appear in public semantics.

### Baseline and generation invariant

Every logical stream starts with its own baseline for the physical
`generation`, an opaque string on the public wire. No global all-history baseline exists:

| stream | first frame | bounded content |
|---|---|---|
| workspace | `baseline` | workspace metadata and membership |
| session-inventory | `baseline` | summaries only, no history |
| session-journal | `snapshot` | one Session, latest bounded window |
| session-control | `baseline` | current nonempty queue/jobs/projections |
| actionables | `reset`, then `upsert` per item | pending approval/question only |

Baseline is replace authority and occurs exactly once per logical stream. A
delta before baseline, a duplicate baseline, a frame from another generation,
or a semantic kind on the wrong stream is a protocol error. Rebaseline closes
and reopens that logical stream with a fresh stream ID; the physical
connection generation may remain. Disconnect discards generation, stream IDs,
cursors, and transport correlations; it does not delete Mirror data.

### Session journal

Opening `session-journal` is follow-first:

1. Register live delivery while excluding publishers.
2. Snapshot the durable journal tail as signed `throughSequence` (`-1` for an
   empty journal).
3. Read a message-aligned latest window frozen at that cut.
4. Emit `snapshot` containing `sessionId`, `generation`, `throughSequence`,
   `windowLimit`, `entries`, `hasMoreBefore`, and projections.
5. Emit later durable `event` frames strictly from `throughSequence + 1`.
   Provider output may additionally arrive as unsequenced `transient`
   frames after the baseline; these are process-local presentation data and do
   not move the durable journal cursor.

Any duplicate with different bytes or sequence gap terminates that logical
stream; the Client opens a fresh stream and reconciles from its new snapshot.
That sequence rule applies only to `event`; `transient` is
folded into the running presentation immediately and is superseded by the next
authoritative assistant/terminal event or reconnect snapshot.
`journal-page` MUST repeat the opening `throughSequence`. Its optional
`beforeSequence` accepts `-1` for an exhausted page and cannot exceed
`throughSequence`. Events appended after
the cut never enter that page, so scrolling older history cannot drift while
live output arrives.

### Workspace, inventory, and control

Workspace frames are baseline/upsert/remove/reorder/archived replacement
semantics. Inventory contains lightweight `SessionSummaryV3` values and uses
baseline/upsert/remove; each summary carries the classified tail state
(`tail`, §SessionSummary) so a parked hold is
distinguishable from a run without opening the journal. Control starts with a baseline containing separate `queues`, `jobs`, and
`projections` arrays; subsequent `queue`, `jobs`, and `projection` frames each
replace that part of one Session's control state.
The Client may display its durable Mirror immediately and apply baselines in
the background.

### Recoverable actionables

An actionable is:

```text
{id, sessionId, revision, kind: approval|question, payload}
```

`id` is stable for the lifetime of the pending Host request. Kernel derives it
from the hold in the semantic ledger; `revision` is that hold's causal
sequence.
`payload` is exactly the existing typed approval-requested or
question-requested UI DTO, so the protocol does not weaken presentation semantics.

Every new logical stream receives all and only pending actionables. A response
uses `actionableId + expectedRevision + outcome`. Once the response is durable
the request is no longer pending and the stream emits its `resolved` delta at
that moment, not at the worker's next `appended` doorbell (which inside a
turn rings only at settle). The Host injects transport-
private response correlation, compares revision, and settles first-wins.
Unknown, already-resolved, stale-revision, duplicate, or cross-Session
responses fail closed. The public protocol never exposes DSH `clientId`, DSH
`eventId`, WebSocket stream correlation, or legacy response `rpcId`.

Actionable business records are recoverable from the Host baseline. Physical
generation and response correlation are memory-only. Kernel process restart
may legitimately produce an empty baseline when the worker operation it was
waiting for no longer exists.

### Structured errors

Categories are `not-found`, `conflict`, `busy`, `invalid-request`,
`unsupported`, `cancelled`, `transport`, and `internal`. `code` is stable and
specific; `message` is diagnostic; `details` is always I-JSON; optional
retryability is `never`, `after-reconnect`, `after-delay`, or `reconcile`.
Host-specific causes belong in details and cannot redefine the category.

### Conformance requirements

Release tests cover: V3-only route/registry parity; ready/capabilities;
baseline-before-delta; per-stream generation rejection; follow-first journal
handoff; frozen pages during append; duplicate/gap behavior; independent
bounded streams; pending actionable replay after Client reconnect; stale and
duplicate response; cancellation/response race; structured errors; drain and
backpressure; and absence of all removed public V2 routes.

## Journal projection

This contract imports, without redefining, the implemented Tekes Client
Session Endpoint contract at
`Tekes/docs/session-event-conversation-transport-v2.md` and its
`session-endpoint-contract-v2/` corpus. The imported contract owns public RPC,
DTO, `SessionEvent`, history, mux, and Client stitching semantics. This file
owns only the deterministic projection and durable-author paths from Kernel
state to that surface. If the two disagree, the imported Client contract wins
and this projection fails closed.

Exact application request/result/error DTOs and mutation transactions are
owned by §Management authority and recovery. The mutation
table below is a projection summary; on any difference, the management
contract controls endpoint behavior and this file controls only projected
SessionEvent bytes.

### Versions and capabilities

The endpoint's `host.describe` value is exactly the imported
`SessionHostDescription`; its `version` is checked against the Client's
endpoint-version allowlist. Endpoint generation remains Client-local and the
wire gains no invented protocol or capability fields. The Client's endpoint
driver owns a closed, endpoint-kind-specific capability catalog, and release
tests compare that catalog with the transport's executable method registry in
both directions. A method is callable only when the driver declares it and the
server registers it. Unsupported methods return `unsupported-capability`;
they never return a successful empty value. Unknown required events and frames
fail closed; only a value carrying exactly `ignorable: true` may be skipped.

The native endpoint implements `workspace.archiveSession` and the independent
`workspace.unarchiveSession` extension. It never encodes unarchive as a flag on
archive. Direct DSH may omit the latter capability until DSH publishes that
route. Optional Git, plugin, and MCP-management surfaces are absent unless a
separately composed service implements them. A Client SideChat is not a
separate surface: it is `session.fork {ephemeral:true}` plus ordinary keyed
prompts, closed by `session.discard` (§Ephemeral sessions).

### Public identity and URI

Native `sessionId` is the canonical UUID in `genesis.thread` and the thread
folder name. The endpoint accepts and emits that UUID only. The Host/Client,
not Kernel, forms `tekes://tekes/<uuid>`. Internal compound storage identities
and the legacy `/threads/` URI never cross the endpoint.

### Durable projection journal

Each active or archived thread folder contains `endpoint.jsonl`: the
deterministic projection of the semantic ledger into public `SessionEvent`
rows, and nothing else. It owns durable endpoint sequence identities and replay
bytes: workers never read it, it cannot affect replay, admission, recovery,
provider input, or live provider cadence, and D-61 does not grant it semantic
authorship. The endpoint writer serializes it under `endpoint.lock`
independently of both the thread-line lock and the transient presentation
channel. A journal written under the earlier name (`endpoint-v2.jsonl`, which
also persisted provider chunks) is retired when the folder is next opened and
the projection is rebuilt from the ledger; its endpoint seqs are reassigned and
clients re-baseline.

Each line is canonical I-JSON plus LF:

```text
{v: 4, event: SessionEvent, kernel_seqs: [int], slot: str}
```

`v` is the projection record version. It is bumped whenever the deterministic
projection of an existing ledger changes shape (v3: durable reasoning folds
into `assistant/message`; v4: `tool/result.error` is absent on success and a
structured `{code, name}` on failure instead of a boolean); a journal whose records carry an earlier `v` is
retired when the folder is next opened and rebuilt from the ledger, its
endpoint seqs are reassigned and clients re-baseline, exactly like the
`endpoint-v2.jsonl` retirement.

`event` is the exact public `SessionEvent` byte shape. `kernel_seqs` is a
sorted, unique list of Kernel event seqs that caused the projection and is
internal provenance; it is not `SessionEvent.sourceEventSeqs`.
`slot` is the projection's fixed table slot name. `sourceEventSeqs`, when
present, always names earlier **endpoint** seqs as the imported contract
requires.

Journal invariants:

1. The first endpoint seq is `0`; every following line is exactly previous +
   1. A session with no projected events has `lastSeq = -1`.
2. The writer appends one complete batch, performs `F_FULLFSYNC` on Darwin
   (full-file sync elsewhere), and only then publishes the durable record.
   Semantic events are emitted after that barrier. Provider deltas are emitted
   earlier on `session/transient`, so neither this sync nor JSONL parsing may
   gate adapter-to-UI delivery. First creation syncs both journal/lock directory
   entries before any durable event can be replayed.
3. `(causal kernel anchor, projection slot)` is unique. The causal anchor is the greatest (last) value in the sorted
   `kernel_seqs` list; every slot in one Kernel event's 1:N projection shares
   that anchor. The endpoint writes all missing slots for that event under one
   lock and one full-sync barrier. Replay or
   receipt retry finds the existing record and reuses its endpoint seq; it
   never appends a duplicate.
4. On open, readers accept only the longest contiguous, canonical, schema-valid
   prefix. A final partial line is truncated by the lock holder. An invalid
   complete line, gap, provenance collision, or mismatch with an already
   durable Kernel prefix is corruption and fail-stop.
5. Reconciliation scans the validated Kernel ledger in seq order and appends
   missing deterministic projections. Existing records must byte-match a
   fresh projection for their slot; the whole journal is reproducible from
   the ledger.
6. Archive moves the journal and lock with the folder. Fork/redact never copy
   endpoint seqs: the destination starts an empty journal and projects its new
   materialized ledger. Carrier scans treat `endpoint.jsonl` as secret-
   bearing; redact drops it and rebuilds it from the materialized prefix.
7. Discard deletes the folder, journal and lock together; nothing is retained.

### Ephemeral sessions

`session.fork {ephemeral:true}` publishes an ordinary fork whose genesis
carries `ephemeral: true`. While it exists it is a session like any other:
same ledger, journal, mux, prompt, cancel, queue and model rules, same
`active-session-limit` accounting. It differs only in lifecycle:

- it is never workspace membership: `workspace.sessionIds` omits it and the
  fork's inventory phase leaves workspace metadata untouched;
- inventory reports `ephemeral:true` so a Client keeps it out of durable
  navigation;
- `workspace.archiveSession` refuses it with `ephemeral`; the only exit is
  `session.discard`, which refuses a durable session with `not-ephemeral` and
  a session with a live line holder or pending work with `session-running`;
- daemon startup deletes every active folder whose genesis is ephemeral before
  the endpoint is published. A crash therefore leaves no ephemeral session
  behind, and a Client must treat one it remembers as gone.

Forking an ephemeral session without the flag produces a durable session; the
flag is never inherited, and child ledgers never carry it. The ledger format
is not gated on the flag: a reader that predates it sees an ordinary session,
which is the documented downgrade behavior.

Provider chunks are transient: they exist only on the process-local
`session/transient` lane and are never journaled (the semantic ledger does not
carry them either). The journal remains categorically unable to author
semantic thread facts.

### Deterministic projection slots

Kernel events are visited in `(file identity, seq)` order; the main file
precedes child-result sources already materialized into it. Within one Kernel
event the following slot order is fixed:

| Kernel fact | endpoint slots |
|---|---|
| `turn_open` | `turn/start`, then one `user/message` per named input in the exact `inputs` order |
| `attempt` | close the preceding open step with `step/end`, then `step/start` |
| durable provider frame | one `assistant/chunk` stream record |
| `output` | `assistant/message` |
| `tool_call` | `tool/call` |
| `tool_result` | `tool/result` |
| `settle` | close an open step with `step/end`, then `turn/end` |
| keyed title `meta` | `session/title` |

New outputs with an explicit `final_answer` field project it as
`assistant/message.data.sessionFinal`. This is the producer's candidate-final
marker; the durable `settle` record closes the turn. An earlier turn already
inside independent validation may resume the same executor after feedback.
A direct settlement projects `turn/end.data.promotedMessageID` from its top-level
`promoted_output_seq`. An independent-validation settlement uses its nested
`validation.promoted_output_seq` and also projects `validationOutcome`. A missing or foreign
output reference is a projection error. Legacy records without these fields
retain their prior wire payload. Clients must consume this identity to promote
the held output, rather than inferring promotion from last-message order.

A worker `output` carrying `final_answer=true` is still an `assistant/message`;
it does not itself emit `turn/end`. Validation candidate, verdict, decision and repair
feedback states likewise cannot emit `turn/end`. Only the durable `settle`
record owns this completion slot. A failed candidate followed by repaired output
therefore remains within one client turn until its validation settlement.

The leading `turn_open` slot and all of its user-message slots share the
`turn_open` Kernel seq in internal provenance; each still receives its own
endpoint seq. The user-message payload is derived from the referenced input,
but its projection slot belongs to the `turn_open`; both seqs appear in
`kernel_seqs`. A steer that enters an existing turn emits `user/message` at
the qualifying admission point and uses both the input and admitting-attempt
seqs as provenance; it does not emit another `turn/start`.

An `attempt` maps to the v2 `step` number equal to its zero-based ordinal
within the turn. The first attempt emits no preceding `step/end`; later
attempts close exactly one prior step. `settle` closes the last open step if
one exists. Thus every step has one start and one end without inventing a
Kernel event. Recovery/adopt uses the same attempt ordinal and projection
rules.

`assistant/message` preserves normalized content, reasoning, usage, provider,
model, continuation/replay state, and interruption semantics available in the
attempt/output facts (usage rides on the output). Durable `reasoning` events of
an attempt project as leading `{type:"reasoning", text}` content blocks of that
attempt's message, in ledger order, ahead of the output's own blocks; the live
`reasoning-delta` chunks remain transient. It does not invent narration/final/thinking event
kinds. `tool/call` and `tool/result` preserve `callId`, name, arguments,
result/error/meta. `tool/result.error` is omitted for outcome `ok` and is
`{code, name}` otherwise (`tool-error`/`ToolError`, `aborted`/`Aborted`,
`denied`/`Denied`, `withheld`/`Withheld`); `meta.kernel_outcome` and
`meta.reason` keep the Kernel-level classification. An optional `ToolEventView` is a deterministic view
projection and never replaces source fields.

Fields typed `JsonValue | spilled` or `[Block] | spilled` are verified against
their content-addressed asset and decoded before they enter a public payload.
`$spill` is a Kernel storage discriminant, never a Session Endpoint value.
Missing, digest-mismatched, length-mismatched, non-I-JSON, or wrong-shaped
assets fail projection closed.
An image Block maps its asset digest, MIME type, decoded byte/dimension facts,
and optional durable `name` to `ImageAttachmentRef`; the name is copied, never
guessed from a path. `session.attachment` may omit that optional display name
because the content-only attachment id can be referenced under more than one
name; the selected message already retains its own reference bytes.

The settle-reason and non-success tool-result mappings in doc 15 are exhaustive
projection tables. Any new Kernel arm blocks projection until this contract and
fixtures add a mapping; a generic success is forbidden.

### File attachments

Non-image files travel as session-scoped uploads, never inline in a prompt.

`session.uploadFile {sessionId, name, mediaType?, data}` (in `attachments.v1`)
publishes the decoded bytes into the session's content-addressed asset store and
returns `{receiptId, file:{attachmentId, name, bytes}}`. `data` is RFC 4648
padded base64; decoded bytes MUST be `1...8,388,608`. `name` follows the image
name rule (1...255 UTF-8 bytes, no control scalars) and additionally MUST NOT
contain `/`. `mediaType` is an RFC 2045 `type/subtype` token, defaulting to
`application/octet-stream`; the four inline image media types are rejected with
`attachment-error {reason:"INLINE_MEDIA_TYPE"}` because images use the `image`
prompt part. `receiptId` is a UUID durable at
`threads/<sessionId>/uploads/<receiptId>.json` =
`{format:1, receiptId, attachmentId, name, bytes, mediaType, createdAt}`
(canonical JSON + LF). `attachmentId = "sha256:" + lowercase_sha256(bytes)`.
A receipt is reusable across prompts of its session and is unusable once the
session is archived; a receipt of another session is
`attachment-error {reason:"UNKNOWN_RECEIPT"}`.

A prompt or command `file` part resolves its receipt and authors the durable
block `{type:"file", asset, mime, name, bytes}`. The public projection of that
block inside `user/message.content` is
`{type:"file", attachment:{attachmentId, name, bytes}, mediaType}`.

`session.fileAttachment {sessionId, attachmentId}` returns
`{attachment:{attachmentId, bytes, mediaType, name?}, data}` under the same
reference authorization as `session.attachment`: the session's ledger or journal
must reference the digest through a file block or an upload receipt. The
`tekesWorkspace.fileTransfer` attachment source serves file attachments the same
way it serves images, so Client previews use one path.

Provider delivery is deterministic per dialect. The worker renders a file block
as the provider wire block `{type:"file", name, mime, data, bytes}`. A dialect
that accepts that media type (Anthropic: PDF; Responses: `input_file`; Google:
`inlineData`) receives the bytes. Otherwise the block degrades to text and the
turn still runs: valid UTF-8 content of at most 262,144 bytes is delivered as
`Attached file <name> (<mime>, <bytes> bytes):` followed by the content in a
fenced block; anything else is delivered as
`Attached file <name> (<mime>, <bytes> bytes) could not be delivered to this model.`
A file attachment never fails a turn on its own.

`commands/run` accepts an optional `attachments:[PromptPart]` (image parts or
file receipts). The expanded command text is the leading `text` block of the
authored input and the attachments follow in request order. The native Swift
client implements `SessionFileUploadEndpoint`, `SessionImageCommandEndpoint` and
`SessionAttachmentCommandEndpoint` with these methods.

### Stream-frame ingestion

Worker `frame` messages are coalescible before acceptance. Once accepted, the
supervisor emits an unsequenced `session/transient` frame and keeps nothing:
chunks are presentation data, superseded by the attempt's durable
`assistant/message` (or terminal error) and by the next reconnect snapshot.
The frame ordinal is a per-attempt presentation version, strictly increasing
while the supervisor holds the attempt, and is not a durable identity. A frame
for an attempt whose terminal output is already projected is dropped. A crash
or reconnect loses the transient tail; nothing is replayed from history.
For `channel: "tool"`, worker-control carries the stable `call_id` and
optional `name`; projection maps them directly to the v2 tool-call-delta `id`
and `name`. A tool delta without `call_id`, or those fields on text/reasoning,
is a protocol error rather than a guessed association.

Tool chunks also carry `assistantFrameId` equal to the source attempt identity.
An optional `argumentsComplete` boolean mirrors the worker's completion marker;
its empty `argumentsDelta` does not append duplicate parameter bytes. Calls in
the same provider response share this identity. The marker uses the same live
publication lane as other accepted chunks, and does not represent a tool
result, execution authorization, or turn completion.

### Surface provenance

Only `user/message`, `assistant/message`, and `tool/result` carry
`surfaceOp`/`sourceEventSeqs`. New surface rows use `surfaceOp: "append"`.
A replacement names an inclusive earlier endpoint range and lists every
earlier endpoint seq contributing to or replaced by the row. Both structures
refer only backward within the same session. Kernel `supersedes` ranges are
translated through the journal's internal kernel provenance map; a referenced
Kernel event with no surface row contributes no endpoint seq.

### History

`session.history {sessionId, beforeSeq?, maxMessages?}` reads one validated
journal snapshot. Reconciliation of the semantic slots in the current valid
Kernel prefix is owned by the supervisor's single projection path and runs on
every worker `appended` doorbell, on every main-line worker exit, on every
sweep tick for a main line with no live worker whose ledger `last_seq` is
ahead of the projector, and before a `session-journal` stream opens on a
session with no live worker; boot is the first sweep. A live worker's
doorbell alone orders projection against the frames still in its control
pipe, so no other path projects while it holds the line. Endpoint reads are
therefore never more than one sweep tick behind the ledger and never repair
in place. `beforeSeq` defaults past the tail; `maxMessages` defaults to
50. Eligible rows have `seq < beforeSeq`. If an arbitrary `beforeSeq` lands
inside one nonempty Kernel projection group, that whole group is excluded from
the older page; callers therefore never observe half of a 1:N projection.

Starting at the newest eligible row, scan backward until the page contains
`maxMessages` append-origin `user/message` plus `assistant/message` rows.
Extend the lower boundary backward to include every row referenced by the
boundary message's `sourceEventSeqs`; repeat to a fixed point. Return every
journal row in that inclusive contiguous range, ascending, including steps,
tools, and structural rows. Never split rows sharing the same nonempty
`kernel_seqs` projection group. `hasMore` is true exactly when a valid row
precedes the returned lower boundary. Older paging uses the returned minimum
seq as `beforeSeq`.

History and live serialize the exact same stored `event` and equivalent
optional `ToolEventView`. The endpoint performs one journal read per logical
page; it does not subdivide by event count.

### Mux, reconnect, and backpressure

On subscription the endpoint snapshots the durable tail and emits
`session/subscribed {sessionId,lastSeq}` before any later event. Events durable
after that snapshot are buffered and then emitted in seq order. Queue,
projection, approval/question requested/resolved, and typed `stream/error`
frames use the imported shapes and do not consume endpoint seq.
`session/transient` also consumes no endpoint seq and is delivered in arrival
order. Its storage copy is later published through the ordinary durable path;
clients keep transient and durable fold state separate so the confirmation
cannot double-append text. Therefore a subscriber that reconnects between
transient delivery and persistence still receives the durable frame.
Answerable requested/resolved frame identity is derived from the semantic
ledger's holds (§Cancel, rename and respond); such
frames are never inserted as non-seq lines into this journal.

Per-subscriber buffering is bounded by 4 MiB or 4096 frames, whichever occurs
first. Overflow sends `stream/error {code:"live-gap"}` when writable and
closes that subscription. It never drops a middle frame and continues. Client
recovery is the imported bounded algorithm: buffer, install tail history,
dedupe overlap, refetch once when baseline is ahead, and refetch on a live gap.
Endpoint restart changes only transport generation; durable seq never resets.

### Inventory and readiness

Workspace/session inventory is derived from config plus folder metadata and
validated semantic-ledger tails. It never reads, mutates, or clears endpoint
transcript projection state.
Expanding a workspace yields stable ids, name/title, `updatedAt`, archived,
running/parked metadata, and projections sufficient for selection. Transcript
history is loaded only after session selection.

`session.models` is session-scoped and independently ready. Catalog failure is
a typed failure/failure entry and cannot erase or downgrade transcript
readiness. The endpoint advertises only provider/model/control capabilities it
can execute.

### Typed mutations and receipts

Every externally authored mutation carries an `rpcId`; durable mutations also
carry an origin tuple `(client, rpcId, operation)`. For Kernel's transport,
`client` is the fixed `session-endpoint` namespace and rpcId uniqueness/
retention is governed by §Transport listener. A repeated tuple returns
the original result/seq. A mismatched tuple or payload returns
`idempotency-conflict`.

Queue item identity, the `edit | remove | steer` transactions, stable respond
rpcIds, create carrier ordering, fork `atSeq` translation, model-selection
storage and folder/config operation recovery are defined only by
§Management authority and recovery; they are not implementation choices left by this
summary.

| endpoint method | durable author path | success point |
|---|---|---|
| `session.create` | keyed genesis through create service | genesis barrier + published folder |
| `session.prompt` queue/steer | keyed input delivery | input barrier receipt; later history reconcile is separate |
| `session.updateQueue` | keyed `queue_edit` delivery | queue-edit barrier receipt; never releases/runs work |
| `session.cancel` | keyed `stop_requested` delivery | stop barrier receipt |
| `session.rename` | live worker `meta`, else supervisor locked-append whitelist | meta barrier receipt |
| `session.fork` | rewrite-publication fork operation | published destination and closed op record |
| `session.discard` | catalog membership lock + folder lifecycle lock + retire/delete | folder absent and closed op record |
| `workspace.archiveSession` | catalog membership lock + folder lifecycle lock + archive rename/sync | authoritative folder durable in archive |
| `workspace.unarchiveSession` | catalog membership lock + folder lifecycle lock + restore rename/sync | active folder durable; Client must rematerialize |
| `session.selectModel` | versioned workspace/session config publication | config revision durable |
| `/api/respond` | keyed `approval_response`/denial delivery | response barrier receipt |

`session.prompt` success means admission durability only. If subsequent
history reconciliation fails, the Client reports accepted-but-not-reconciled,
not Send failure. Archived sessions reject history, delivery, answer, and
queue editing with typed `archived` until explicit unarchive. Rename/archive/
unarchive/fork/model selection have typed request, result, and error envelopes;
no boolean-shaped implicit reversal is legal.

### Canonical fixtures and synchronization

`fixtures/endpoint/authority/` is a byte-for-byte vendor of Tekes commit
`adaab3088c1cb33cf864a3047942fcba491cb281` under
`docs/session-endpoint-contract-v2/`. `authority-lock.canonical.json` records
the commit and SHA-256 of every file. `scripts/check-endpoint-authority.py`
compares both directions against that pinned Git tree when the sibling Tekes
repository is available; standalone CI always verifies the committed hashes.

Kernel-specific fixtures cover 1:N allocation, stream durability, paging,
overlap/gap/reconnect, typed mutation receipts, archive/unarchive, unknown
required/ignorable behavior, and journal corruption. Implementations must
round-trip canonical files byte-identically and execute JSONL transcripts in
order; fixture bytes are never generated by the implementation under test.

A live provider frame's projection-cache refresh stops before its attempt's
response records (tool calls, output, or error). A newer disk tail alone
is not permission to publish semantic completion ahead of earlier control-pipe
frames. An already published semantic record is not rolled back; late frames
cannot reopen it. The ordered semantic publication path drains queued frame
storage before projecting the completed response.

## Transport listener

The public routes are those of §Routes and streams; the listener below is the
physical carrier those routes and the internal registrations share. Routes
retired from the public surface (`host.describe`, `workspace.list`,
`session.list`, `session.history`, `events.mux`, `events.host`, `respond`)
remain registrations only so the carrier fixtures and idempotency records that
name them stay decodable; they are not advertised.

This is the physical carrier for Slice 9. The Client-owned authority is
`Tekes/docs/session-event-conversation-transport-v2.md` and its
`session-endpoint-contract-v2/` corpus. This contract binds those exact DTOs to
TekesKernel's carrier-neutral §Journal projection;
it never imports frozen TekesAppServer v1 methods or DTOs.
The exact 20 registration payloads, results, semantic errors, durable author
paths and management recovery rules are owned by
§Management authority and recovery. This file owns only the
HTTP/WebSocket carrier.

### Listener and paths

The listener exposes one HTTP origin. Local production binds loopback only;
non-loopback deployment is unsupported until a later TLS contract exists. The
bearer below authenticates the loopback lane; it is not a substitute for TLS on
a remotely reachable listener.

- Unary RPC: `POST /api/{method}` with `Content-Type: application/json`.
- Session stream: WebSocket `GET /api/events.mux`.
- Host stream: WebSocket `GET /api/events.host`.
- Typed response to a server request: `POST /api/respond`.
- Liveness: `GET /health/live` returns no inventory or readiness claim.
- Readiness: `GET /health/ready` succeeds only after storage preflight,
  supervisor ownership, config load, endpoint-journal validation, and protocol
  registration complete.

Health routes are the only unauthenticated paths and expose no inventory.
Every `/api/*` HTTP request and WebSocket upgrade requires
`Authorization: Bearer <token>`, where token is 32 random bytes encoded
base64url without padding. The launching application generates it and passes
the same bytes as 64 hexadecimal characters in `TEKES_KERNEL_ENDPOINT_TOKEN`
([Application-owned launch](../docs/builtin-launch.md)); it is process-scoped.
Helper and worker processes never receive it, and it is never written under
the storage root or into argv, logs or discovery material. The listener performs
a constant-time comparison after headers and before body dispatch or upgrade.
Missing, malformed, or wrong credentials return the exact 401 row below.
Authentication precedes browser-Origin policy; authenticated forbidden origins
still return 403. Access/error logs record neither the header nor token hash.
The endpoint has no browser-origin allowlist: the native Client omits
`Origin`, and any request or WebSocket upgrade carrying an `Origin` header
(including the fixed endpoint origin and `null`) returns the exact 403 row.
A later browser client requires a versioned deployment policy and fixtures;
an implementation cannot infer same-origin permission.

Every RPC/WebSocket envelope is byte-shape compatible with the imported v2
fixtures. JSON objects may use any transport whitespace, but values, omission
versus null, unions, error codes, and integer limits are identical. Requests
larger than 16 MiB, non-UTF-8, duplicate JSON keys, non-I-JSON numbers, or a
body trailing a complete JSON value fail before dispatch.

### Negotiation and capabilities

`host.describe` is the first successful application call. Its value is exactly
the Client-owned `SessionHostDescription` DTO: `{version, cwd, provider?,
model?, attachedSessions, home, canOpenPath}`. The Client accepts `version`
only through its endpoint-version allowlist. Endpoint generation is
connection-local Client state and is not a wire field.

Capabilities are the endpoint driver's closed, endpoint-kind-specific method
catalog, not extra fields smuggled into `host.describe`. The `.tekes` driver
declares only routes that this transport registers and can execute; the DSH
driver keeps its separately accepted catalog. Authority sync compares the
driver catalog with the server registry in both directions before release.
The server never returns a successful empty value for an unimplemented method.
Unknown methods are `method-not-found`; a known driver operation unavailable
for this endpoint kind is `unsupported-capability`; an unknown required
SessionEvent fails closed. Only an event with explicit `ignorable: true` may
be skipped.

The Slice-9 `.tekes` driver/registry set is exactly the closed registration
table in §Management authority and recovery and is reproduced here only as a carrier
registry:

```text
host.describe
workspace.list
workspace.create
workspace.rename
workspace.relocate
workspace.archiveSession
workspace.unarchiveSession
session.list
session.create
session.history
session.prompt
session.updateQueue
session.cancel
session.rename
session.fork
session.discard
session.attachment
session.models
session.selectModel
events.mux
events.host
respond
```

`events.mux` and `events.host` name WebSocket registrations and `respond`
names `POST /api/respond`; the other names use `POST /api/{method}`. A later
route is a versioned Client-driver change and cannot appear only in the server.
Direct DSH keeps its own accepted catalog and does not acquire
`workspace.unarchiveSession` from this list.

The public `sessionId` is the canonical lowercase hyphenated UUID owned by the
Kernel. Internal folder/compound ids never cross the transport. The Client/host
constructs the locator `tekes://tekes/<sessionId>`; the endpoint returns the
UUID, not the URI and not a `/threads/` path.

### Unary execution

The request envelope's `rpcId` is required, bounded to 128 UTF-8 bytes, and
unique in the endpoint-wide namespace for the lifetime of retained
idempotency records. The origin tuple's `client` is the fixed
`session-endpoint` namespace and its other fields are the exact `rpcId` and
operation. Ledgers and operation records written before the namespace
lost its version suffix carry `session-endpoint-v2`; recovery reads it as the
same namespace and nothing writes it anymore. A retry after disconnect reuses the same rpcId and returns the
original durable receipt; reuse with another operation or payload is
`idempotency-conflict`. TCP/WebSocket connection identity never participates.

HTTP status reports carrier success, not semantic success:

- `200` carries either the imported success or typed error result;
- `400` malformed envelope/value;
- `404` unknown HTTP path;
- `405` wrong method;
- `413` request too large;
- `415` wrong content type;
- `429` connection/admission bound;
- `503` not ready or draining.

A non-200 carrier response has `Content-Type: application/json` and the exact
UTF-8 body `{"error":{"code":C,"details":{},"message":M}}`, serialized as
RFC 8785 JSON with no trailing newline. The closed `(status, C, M)` table is:

| status | code | message |
|---:|---|---|
| 400 | `invalid-request` | `Request is malformed` |
| 401 | `unauthorized` | `Authentication required` |
| 404 | `not-found` | `Path not found` |
| 405 | `method-not-allowed` | `Method is not allowed` |
| 413 | `payload-too-large` | `Request body is too large` |
| 415 | `unsupported-media-type` | `Content-Type must be application/json` |
| 403 | `forbidden-origin` | `Browser origin is not allowed` |
| 429 | `overloaded` | `Endpoint admission limit reached` |
| 503 | `not-ready` | `Endpoint is not ready` |
| 503 | `server-draining` | `Endpoint is draining` |

The `server-draining` row applies after drain begins; otherwise 503 uses
`not-ready`. Required response headers are `content-type:
application/json` and decimal `content-length`; header order and HTTP library
framing are not application authority.

HTTP 200 semantic failures retain the Client `server-response` envelope. The
Kernel-specific rows below are the transport-wide idempotency/drain subset;
the complete closed semantic-error table is §Closed
semantic errors:

| code | message | details |
|---|---|---|
| `idempotency-conflict` | `rpcId was already used for another request` | `{rpcId, operation}` |
| `archived` | `Session is archived` | `{sessionId}` |
| `accepted-but-not-confirmed` | `Mutation was accepted but its receipt was not confirmed` | `{rpcId, operation}` |

Fields in each details object are required strings and no extras are legal.

No semantic mutation is retried automatically by the carrier. Admission
receipt and later history reconciliation remain separate results.

### Stream handoff and backpressure

The accepted Client sends no application subscribe message. Opening
`/api/events.mux` registers one connection-level subscriber for the sessions
attached to that endpoint. For each attached session, registration occurs
before the server snapshots the durable endpoint-journal tail; the server then
sends `session/subscribed` and drains frames buffered after that snapshot in
strict seq order. A later attached session begins with its own
`session/subscribed`. `/api/events.host` likewise sends server frames only.
In every `server-request` stream envelope, `method` equals the payload's
required `type` (`session/subscribed`, `session/event`, `host/session-status`,
`stream/error`, and so on); `events.mux` and `events.host` are registration
route names only and never appear as an envelope method. A mismatch closes the
connection generation as `protocol-error`.
History and live use the same stored SessionEvent and optional ToolEventView
bytes. Chunks inside a selected raw history range are returned; Client
persistence policy does not authorize server omission.

The endpoint-projection 4 MiB/4096-frame subscriber bound applies per session
subscription. Overflow sends typed `live-gap` when possible and closes that
subscription. It never drops a middle frame and continues. Ping/pong transport
frames do not affect endpoint seq. A malformed/unknown-required application
frame closes with a typed protocol error; connection loss changes transport
generation but never resets durable `(sessionId, seq)`.

Before closing for an application condition, the server sends one canonical
`stream/error` envelope when the socket remains writable. The close table is:

| condition | error code | WebSocket close code | UTF-8 close reason |
|---|---|---:|---|
| live gap/backpressure | `live-gap` | 1013 | `live-gap` |
| server drain | `server-draining` | 1013 | `server-draining` |
| malformed/unknown required frame | `protocol-error` | 1002 | `protocol-error` |
| application frame over 16 MiB | `frame-too-large` | 1009 | `frame-too-large` |
| unexpected server failure | `internal-error` | 1011 | `internal-error` |

Normal Client detach uses 1000 with an empty reason. RFC 6455 masking, random
mask keys and fragmentation are transport-library concerns; the application
oracle fixes the decoded text payload and close code/reason, not random frame
bytes.

### Concurrency, draining, and security

The limits are fixed: 128 concurrent unary requests, 32 WebSockets, 256
session subscriptions per socket, 30 seconds for a unary request to produce a
response after its body is accepted, and 15 seconds to deliver a complete
request body. A later override requires a versioned deployment/config contract.
Admission happens
before reading an unbounded body. Slow clients cannot retain Kernel file locks;
history/journal snapshots are materialized under a bounded read and written
after locks are released.

On drain, readiness fails first, new upgrades/mutations receive `503`, existing
read-only calls get a bounded grace period, and WebSockets receive a typed
`server-draining` error before close when writable. Accepted mutations finish
to their durable receipt or return HTTP 200 with the typed error
`accepted-but-not-confirmed` and details `{rpcId, operation}` when authoring
handoff occurred but no receipt arrived before the drain deadline. The Client
retains admission as indeterminate, reconciles history, and may retry the
identical rpcId; shutdown never reports semantic success before the authoring
barrier.

Loopback does not relax secret handling. Authorization headers, provider keys,
tool output secrets, prompt bodies, and attachment bytes are excluded from
access logs. Origin checking follows the closed no-browser rule above; no
unversioned deployment setting may weaken it.

The access-log application record is canonical JSON with fields
`{v:1, request_id, operation, path, status, elapsed_ms}` and optional
`error_code`; `elapsed_ms` is a non-negative safe integer. `request_id` is the
rpcId only after a valid envelope is decoded, otherwise a transport-generated
opaque id. No header values, payload, query text, session content, tool output,
attachment bytes, provider/model credential, or socket frame body may appear.
Tests normalize `elapsed_ms` to zero before comparing the redaction oracle.

### Host seam

The `transport` package depends on the carrier-neutral endpoint package and a
narrow `EndpointHost` callback trait owned by that lower layer. The supervisor
implements the trait at executable assembly. Neither endpoint nor transport
imports the supervisor package; callbacks return typed futures/results and
never expose supervisor/process-table types or hold store locks across await.

### Fixture contract

`fixtures/endpoint/authority/` remains the DTO authority. Slice 9 adds
`fixtures/endpoint-transport/cases.canonical.json` naming every carrier trace.
Each `.request.raw` and `.response.raw` is a **transport transcript v1**:

1. first bytes exactly `# tekes-endpoint-transport\n`;
2. zero or more RFC-8785 canonical JSON records, each followed by LF;
3. each record has exactly one `kind` plus the fields for that kind:
   - `http-request {method,path,headers:[[lowercase-name,value],...],body_b64}`
     or the same record with `body_repeat:{byte,count}` instead of
     `body_b64`; exactly one body form is present;
   - `http-response {status,headers:[[lowercase-name,value],...],body_b64}`;
   - `ws-open {path,headers:[[lowercase-name,value],...]}`;
   - `ws-text {body_b64}` for one decoded UTF-8 application message;
   - `ws-close {code,reason}`;
   - `client-action {name,session_id?}` for Client-local open, reconnect,
     tail-refetch, retire-projection and rematerialize checkpoints;
   - `access-log {record}` where `record` is the normalized access-log object
     defined above;
   - `listener {address,result}` where `address` is an IP literal plus port
     and `result` is `accepted` or `rejected`;
   - `fault {name}` for the registry-defined deterministic race/failure point.

Header arrays are sorted by lowercase name then value; duplicate semantic
headers are invalid. Base64 uses RFC 4648 padded standard form. A
`body_repeat` byte is an integer 0...255 and count is a safe positive integer;
it expands to exactly `count` copies of that byte before decoder limits are
applied, and exists only to keep boundary fixtures small. HTTP body and
WebSocket text payload bytes are exact even though HTTP header order and RFC
6455 random masking are deliberately outside the oracle. A request transcript
may contain both client actions and server-directed HTTP/WS records; a response
transcript contains observed server records and Client recovery actions, so
races, reconnect and archive/rematerialization remain one executable case.

Each `.expected.canonical.json` is RFC-8785 JSON with exactly:

```text
{
  carrier_status: "open" | "http-NNN" | "ws-closed-NNNN",
  rpc_result: "none" | {ok:true,value:JsonValue} |
              {ok:false,error:{code,message,details:JsonValue}},
  durable_effect: "none" | {kind, identity, seq?},
  connection_action: "keep-open" | "reject" | "close" |
                     "tail-refetch" | "retire-projection" |
                     "rematerialize"
}
```

`kind` and `identity` are non-empty strings; `seq`, when present, is a safe
integer. No additional expected fields are legal. The registry covers every
status and close row above, host/version and driver/registry parity,
subscription ordering, history/live identity, chunk retention, overlap/gap
reconnect, typed respond, archive/archived rejection/unarchive/Client
rematerialization, malformed/oversized input, backpressure, drain, Client-local
generation replacement, endpoint-wide rpcId retry/conflict, and
`accepted-but-not-confirmed`. The `management` family additionally has
exactly one case for every §Management authority and recovery registration, and each such
transcript covers its request, success, and route-specific failure. Registry,
management table and disk are checked in both directions before the authority-
sync gate and transport tests.

## Management authority and recovery

This is the executable application contract for the 22 management
registrations below. It imports the Client-
owned Session Endpoint v2 DTOs from the authority lock under
`fixtures/endpoint/authority/` and binds them to Kernel durable operations.
The imported DTO names, omission rules, and JSON unions are normative. This
file owns the native Kernel author path, idempotency, crash recovery, and the
two native extensions `workspace.unarchiveSession` and the all-session stream
lifecycle.

There are exactly **22 registrations**, not 22 unary methods: 19 ordinary
`POST /api/{method}` RPCs, the bare-response `POST /api/respond`, and two
WebSocket registrations. `scripts/check-endpoint-transport-fixtures.py` compares
this table with its management route registry in both directions.

This table is not the public protocol version 3 registry. Under V3 the public
routes are the 16 unary methods in §Routes and streams plus `remote.mux`; that
list includes `models.list`, which this table does not. `host.describe`,
`workspace.list`, `session.list`, `session.history`, `events.mux`,
`events.host`, and `respond` are not public V3 routes. Their names remain in
internal code (the `respond` idempotency operation and the stream channel
names) and in the management fixtures.

### JSON notation and common rules

`?` means an optional field that is omitted, never JSON null. Objects are
closed unless a field is explicitly typed `JsonValue`. `int` is a nonnegative
I-JSON safe integer. `uuid` is a lowercase canonical hyphenated UUID; newly
allocated Kernel session ids are UUIDv7. `ts` is UTC RFC 3339 with exactly
milliseconds. `ContentBlock` is the imported merge-extensible object with a
required nonempty string `type`; `PromptPart` is the closed union:

```text
{type:"text", text:str} |
{type:"image", mediaType:"image/png"|"image/jpeg"|"image/webp"|"image/gif",
 data:str, name?:str} |
{type:"file", receiptId:str}
```

A `file` part names an upload receipt issued by `session.uploadFile` for the
same session (§File attachments). It materializes into the durable Kernel file
block `{type:"file", asset:"sha256-"+digest, mime, name, bytes}` before the
input barrier, exactly like an image part materializes into an image block.

An image `name`, when present, follows event's 1...255-byte/no-control rule.
It is copied into the durable Kernel image Block before the input barrier, so
restart, fork, redact, queue projection, and history reproduce the same public
attachment reference. It is display metadata only: asset identity remains the
digest of decoded bytes and provider rendering ignores the name.

`data` is RFC 4648 padded base64. Decoded image bytes MUST match the declared
media type, be at most 8,388,608 bytes, have width and height each in
`1...16,384`, and contain at most 67,108,864 pixels. For animated GIF, WebP,
or APNG, the decoder MUST consume every frame, each frame MUST fit the declared
canvas, and the sum of `frame_width * frame_height` over all frames MUST also
be at most 67,108,864. An invalid later frame is `DECODE_FAILED`; a frame or
cumulative-pixel limit violation is `INVALID_DIMENSIONS`. These limits are
fixed, not config. The image must be fully decoded within those bounds and published through
the event asset reference protocol before the input barrier. The endpoint
derives `attachmentId = "sha256:" + lowercase_sha256(bytes)` and the durable
Kernel image block `{type:"image",asset:"sha256-"+digest,mime:mediaType,name?}`.
The public attachment reference is:

```text
ImageAttachmentRef = {
  attachmentId:str, mediaType:media-type, bytes:int>=1,
  width:int>=1, height:int>=1, name?:str
}
ImageAttachment = {attachment:ImageAttachmentRef, data:str}
```

`media-type` is exactly the four-value PromptPart media-type union. Width and
height are decoded image dimensions, never trusted request fields. A
`session.attachment` response omits `name`: attachment id is content-only and
one digest may be referenced under different display names; each historical
message already carries its own durable name.
`session.attachment` authorizes a read only when the selected session's valid
ledger or endpoint journal references the exact digest. It then verifies file
digest, byte length, media type and decoded dimensions before returning base64.
It is not an upload route and a cross-session digest guess is `attachment-error`.

The `attachment-error.details.reason` value is the closed union
`INVALID_BASE64 | MEDIA_TYPE_MISMATCH | TOO_LARGE | INVALID_DIMENSIONS |
DECODE_FAILED | NOT_REFERENCED | CORRUPT | QUEUE_EDIT_NON_TEXT`. Prompt-image
validation uses the first five rows; attachment read uses `NOT_REFERENCED` or
`CORRUPT`; queue edit uses `QUEUE_EDIT_NON_TEXT`. A limit failure is
`TOO_LARGE` even if decoding would later expose another defect; base64 syntax
is checked first, then decoded-byte length, media signature, bounded decode,
dimensions/pixel count, and asset publication in that order.

Every ordinary request is the imported envelope
`{type:"client-request",rpcId:str,method:str,payload:Payload}` and every ordinary
response is `{type:"server-response",rpcId,result:{ok:true,value:Value} |
{ok:false,error:RemoteError}}`. `method` MUST equal the registration name and
the response MUST echo `rpcId`. `/api/respond` is the explicit exception in
§Respond. Transport validation and limits are owned by §Transport listener.
Client-authored rpcIds are nonempty UTF-8 of at most 128 bytes and MUST NOT
start with the reserved `request-` prefix. That prefix belongs exclusively to
server-authored answer frames, making ordinary idempotency identities disjoint
from `/api/respond` correlation; a violation is carrier `invalid-request`.

### Closed registration table

The request and success value columns below are exact. A later field or union
arm requires a versioned Client authority change and new fixtures.

| registration | request payload | success value |
|---|---|---|
| `host.describe` | `{}` | `{version,cwd,provider?,model?,attachedSessions,home,canOpenPath}` |
| `workspace.list` | `{}` | `{items:[Workspace],archivedSessionIds:[uuid]}` |
| `workspace.create` | `{path:str}` | `{workspace:Workspace,created:bool}` |
| `workspace.rename` | `{workspaceId:str,title:str}` | `{workspace:Workspace}` |
| `workspace.relocate` | `{workspaceId:str,previousPath:str,path:str}` | `{workspace:Workspace}` |
| `workspace.archiveSession` | `{sessionId:uuid}` | `{archivedSessionIds:[uuid]}` |
| `workspace.unarchiveSession` | `{sessionId:uuid}` | `{sessionId:uuid}` |
| `session.list` | `{cursor?:str}` | `{items:[SessionSummary]}` |
| `session.create` | `{workspaceId?:str,cwd?:str,sessionId?:uuid,agentPreset?:str,identityProfile?:"coding"|"general"}` | `{sessionId:uuid}` |
| `session.history` | `{sessionId:uuid,beforeSeq?:int,maxMessages?:int>=1}` | imported `SessionHistoryPage` |
| `session.prompt` | `{sessionId:uuid,mode:"queue"|"steer",content:[PromptPart],clientTimeZone?:str}` | `{accepted:true,command?:{kind:"success",text?:str}}` |
| `session.updateQueue` | `{sessionId:uuid,itemId:str,action:QueueAction}` | `{accepted:true}` |
| `session.cancel` | `{sessionId:uuid}` | `{accepted:true}` |
| `session.rename` | `{sessionId:uuid,title:str}` | `{title:str,seq:int}` |
| `session.fork` | `{sessionId:uuid,atSeq?:int,ephemeral?:bool}` | `{sessionId:uuid}` |
| `session.discard` | `{sessionId:uuid}` | `{sessionId:uuid}` |
| `session.attachment` | `{sessionId:uuid,attachmentId:str}` | `ImageAttachment` |
| `session.models` | `{sessionId:uuid}` | imported `SessionModels` |
| `session.selectModel` | `{sessionId:uuid,provider:str,model:str,reasoningEffort?:str}` | `{selected:{provider,model,reasoningEffort?}}` |
| `events.mux` | WebSocket open; no application request body | imported mux server-request frames |
| `events.host` | WebSocket open; no application request body | imported host server-request frames |
| `respond` | §Respond client-response envelope | bare `{accepted:bool,reason?:str}` |

```text
Workspace = {
  workspaceId:str, path:str, title:str, sessionIds:[uuid],
  createdAt:ts, updatedAt:ts
}
SessionSummary = {
  sessionId:uuid, updatedAt:number, running:bool, tail:str, blank:bool,
  parentSessionId?:uuid, origin?:str, ephemeral?:bool, cwd?:str, agentPreset?:str,
  projections?:SessionProjectionsBlock,
  permissionMode?:"read-only"|"workspace-write"|"danger-full-access"
}
QueueAction =
  {kind:"edit",content:[ContentBlock]} |
  {kind:"remove"} |
  {kind:"steer"}
QueuedInboxItem = {
  id:str, placement:"queued"|"steering"|"context", message:JsonValue
}
```

`session/queue` is the whole current snapshot of `QueuedInboxItem`; omission
on the initial baseline means empty and a transition to empty emits `items:[]`.
The `message` value is the complete imported user message. The endpoint never
reconstructs it from display text.

For an input at Kernel seq `n`, the queue item id is `"input:"+decimal(n)` and
the message is exactly the §Journal projection `user/message.data` value
that consumption would emit: `{id:"input-"+decimal(n),role:"user",content,
source}`. `content` uses the same Block-to-public-block mapping;
`source.kind` is the input source (default `user`) and `source.rpcId` is the
input origin tuple key when present. The native endpoint never adds `clientTimeZone` or
another endpoint-private field to this imported public source object. Live
unconsumed inputs sort by ascending
Kernel seq. `placement` is `steering` only while a steer is pending admission
to its target turn; after expiry it is ordinary `queued` (there is no expired
placement). An open hold does not alter placement. Every mutation emits one
whole snapshot after its durable receipt; reconnect may repeat the identical
snapshot, and transition to no live inputs emits `items:[]`.

### Closed semantic errors

For ordinary RPCs, semantic failures are HTTP 200 server-response errors. For
each row, the details object has exactly the shown fields. Messages are fixed;
dynamic identifiers belong only in details.

| code | message | details |
|---|---|---|
| `bad-request` | `Request payload is invalid` | `{}` |
| `unsupported-capability` | `Capability is unavailable` | `{operation}` |
| `idempotency-conflict` | `rpcId was already used for another request` | `{rpcId,operation}` |
| `session-not-found` | `Session was not found` | `{sessionId}` |
| `workspace-not-found` | `Workspace was not found` | `{workspaceId}` |
| `workspace-invalid-path` | `Workspace path is invalid` | `{path}` |
| `workspace-ambiguous` | `Workspace path matches more than one workspace` | `{path}` |
| `workspace-name-conflict` | `Workspace title already exists` | `{name}` |
| `workspace-title-invalid` | `Workspace title is invalid` | `{title}` |
| `workspace-busy` | `Workspace has a running session` | `{workspaceId}` |
| `session-conflict` | `Session identity conflicts with existing state` | `{sessionId,requestedCwd,existingCwd?}` |
| `session-running` | `Session has a running worker` | `{sessionId}` |
| `archived` | `Session is archived` | `{sessionId}` |
| `not-archived` | `Session is not archived` | `{sessionId}` |
| `ephemeral` | `Session is ephemeral and cannot be archived` | `{sessionId}` |
| `not-ephemeral` | `Session is not ephemeral` | `{sessionId}` |
| `invalid-cursor` | `Session cursor is invalid` | `{cursor}` |
| `invalid-at-seq` | `Fork position is not a complete projection boundary` | `{sessionId,atSeq}` |
| `active-session-limit` | `Active session limit reached` | `{limit}` |
| `queue-item-not-found` | `Queued item is no longer pending` | `{itemId}` |
| `steer-unavailable` | `Current turn no longer accepts steering` | `{itemId}` |
| `attachment-error` | `Image attachment is unavailable` | `{reason}` |
| `model-unavailable` | `Model is unavailable` | `{provider,model}` |
| `title-invalid` | `Session title is invalid` | `{sessionId}` |
| `accepted-but-not-confirmed` | `Mutation was accepted but its receipt was not confirmed` | `{rpcId,operation}` |
| `internal` | `Endpoint operation failed` | `{}` |

The allowed semantic codes are closed per registration (carrier validation
errors are separate):

| registration | allowed codes |
|---|---|
| `host.describe`, `workspace.list` | `internal` |
| `workspace.create` | `workspace-invalid-path`, `workspace-title-invalid`, `workspace-name-conflict`, `idempotency-conflict`, `internal` |
| `workspace.rename` | `workspace-not-found`, `workspace-title-invalid`, `workspace-name-conflict`, `idempotency-conflict`, `internal` |
| `workspace.relocate` | `workspace-not-found`, `workspace-invalid-path`, `workspace-ambiguous`, `workspace-busy`, `idempotency-conflict`, `internal` |
| `workspace.archiveSession` | `session-not-found`, `session-running`, `idempotency-conflict`, `internal` |
| `workspace.unarchiveSession` | `session-not-found`, `not-archived`, `active-session-limit`, `idempotency-conflict`, `internal` |
| `session.list` | `invalid-cursor`, `internal` |
| `session.create` | `bad-request`, `workspace-not-found`, `workspace-invalid-path`, `workspace-ambiguous`, `session-conflict`, `active-session-limit`, `unsupported-capability`, `idempotency-conflict`, `internal` |
| `session.history` | `session-not-found`, `archived`, `internal` |
| `session.prompt` | `session-not-found`, `archived`, `steer-unavailable`, `attachment-error`, `idempotency-conflict`, `accepted-but-not-confirmed`, `internal` |
| `session.updateQueue` | `session-not-found`, `archived`, `queue-item-not-found`, `steer-unavailable`, `attachment-error`, `idempotency-conflict`, `accepted-but-not-confirmed`, `internal` |
| `session.cancel` | `session-not-found`, `archived`, `idempotency-conflict`, `accepted-but-not-confirmed`, `internal` |
| `session.rename` | `session-not-found`, `archived`, `title-invalid`, `idempotency-conflict`, `accepted-but-not-confirmed`, `internal` |
| `session.fork` | `session-not-found`, `archived`, `invalid-at-seq`, `active-session-limit`, `idempotency-conflict`, `internal` |
| `session.discard` | `session-not-found`, `archived`, `not-ephemeral`, `session-running`, `idempotency-conflict`, `internal` |
| `session.attachment` | `session-not-found`, `archived`, `attachment-error`, `internal` |
| `session.models` | `session-not-found`, `archived`, `internal` |
| `session.selectModel` | `session-not-found`, `session-running`, `archived`, `model-unavailable`, `idempotency-conflict`, `internal` |

`events.mux` and `events.host` use only §Transport listener stream errors.
`respond` uses only its bare rejection reasons in §Respond. A code outside the
row is a protocol defect, not a generic fallback.

Malformed envelopes remain carrier errors. An archived session accepts only
`workspace.unarchiveSession`; every session read/mutation and `/api/respond`
for its pending request returns `archived`. Implementations do not translate a
closed error into success with an empty value.

### Idempotency and operation journal

Every request has one endpoint-wide in-flight idempotency identity `(rpcId,
operation, canonical request bytes)`. Read retries may recompute from a fresh
snapshot; read identities live only in the bounded in-memory in-flight map and
are released after response, so history/list bodies are never retained.
Mutations additionally use the durable carrier below and normally return the
first terminal response bytes forever. Reuse of a live or durable mutation
rpcId with another operation or request bytes is `idempotency-conflict`.

Every event-authored request uses this exact event origin tuple:

```text
principal = "uid:" + unsigned-decimal effective UID captured at readiness
client    = "session-endpoint"
target    = target session UUID (new UUID for session.create)
op        = exact registration name; /api/respond uses "respond"
key       = rpcId
```

Queue transactions replace `key` with `rpcId+"/retract"` and
`rpcId+"/replacement"`; respond replaces it with `rpcId+"/response"`.
All other tuple fields remain identical. Workspace-only config mutations do
not invent semantic event tuples. Fork's new genesis uses the same principal,
client, `op:"session.fork"`, target destination UUID and key rpcId.

#### Endpoint rpc exact-retry carrier

Exact retry is owned by this endpoint-scoped closed carrier, not a cache:

```text
endpoint-management/
  rpc.lock
  rpc/<first-two-sha256-hex>/<sha256(rpcId)>.jsonl
```

Each file is canonical JSONL with contiguous ordinals and this closed row:

```text
{
  v:2, ordinal:int, rpc_id?:str, rpc_sha256?:hex64,
  operation:str, request_sha256:hex64,
  target_session?:uuid,
  phase:"prepared"|"handed-off"|"complete"|"retired",
  delivery?:str, durable_identity?:{kind:str,id:str,seq?:int},
  projection_metadata?:{client_time_zone:str},
  response_b64?:str, retired_reason?:"redact"
}
```

Only mutations create these files. Non-retired rows require `rpc_id` and
forbid `rpc_sha256`; a retired row does the reverse so an rpcId containing
forbidden plaintext can itself be erased while lookup remains possible by the
carrier filename. Row 0 is `prepared`, durable before delivery or a management
operation starts.
For `session.prompt`, row 0 carries `projection_metadata` exactly when
`clientTimeZone` was present; later rows repeat it byte-for-byte. No other
operation may carry that field. `handed-off` is appended only after a live-worker delivery is accepted or the
supervisor has durably authored the locked-append event; it requires
`delivery`, and may name a known durable identity. `complete` requires the
terminal durable receipt/semantic barrier or management operation `complete`,
and contains the exact complete HTTP application-response bytes in padded
base64 plus the final durable identity. Each append full-syncs the whole prefix
before the corresponding effect/response is exposed. Directory creation and
file publication use temp+sync+rename+parent sync; `rpc.lock` serializes the
endpoint-wide namespace.

Recovery validates the request hash and probes the named event origin tuple or
management operation. Prepared with no effect resends/starts; an unknown live
handoff is re-delivered by the same delivery and origin key; a found durable
effect completes without repeating it. A complete retry returns the stored
response bytes exactly. Invalid lines, phase regression, conflicting fields,
or durable bytes disagreeing with the record are fail-stop. Records are kept
until storage-root retirement, so reuse remains conflicting across archive,
unarchive, restart and thread deletion. Fork does not copy them. D-40 redact
overrides exact-response retention: under the root management lock it scans
every rpc/operation record targeting the source, rewrites a secret-bearing rpc
file atomically to one ordinal-0 `retired` row (same operation/request/target,
`rpc_sha256 = sha256(rpcId)`, `retired_reason:"redact"`, no rpcId, delivery,
projection metadata,
durable identity, or response), and
retires the operation response below. The old file is unlinked only after
replacement and directory sync. An identical post-redact retry returns the
canonical `idempotency-conflict` response; the rpcId remains unavailable. GC
cannot drop a complete or retired identity.

Crash-recoverable config/folder mutations use:

```text
endpoint-management/
  lock
  workspaces/<workspace-id>.json
  operations/<first-two-sha256-hex>/<sha256(rpcId)>.json
```

Files are RFC-8785 canonical JSON plus LF and publish by temp write, full-file
sync, atomic rename and parent-directory sync. `lock` is acquired with the
same no-follow/inode-revalidation rules as config locks. Operation records are:

```text
{
  v:2, rpc_id?:str, rpc_sha256?:hex64, operation:str, request_sha256:hex64,
  phase:"prepared"|"carriers"|"semantic"|"inventory"|"complete",
  started_at:ts, intent:OperationIntent,
  response?:JsonValue, retired?:"redact"
}
```

`intent` is immutable after `prepared` and is exactly one arm selected by
`operation`:

```text
{kind:"workspace",action:"create"|"rename",workspace_id:str,
 canonical_path:str,title:str,config_digest:hex64} |
{kind:"session-create",session_id:uuid,workspace_id:str,folder_binding:str,cwd:str,
 config_asset:asset-name,instruction_asset:asset-name} |
{kind:"fork",source:uuid,dest:uuid,at_endpoint_seq?:int,
 kernel_anchor:int,rewrite_op_id:str,principal:"uid:"+unsigned-decimal-euid} |
{kind:"folder-move",action:"archive"|"unarchive",session_id:uuid,
 source_path:str,dest_path:str,directory_identity:str,
 valid_prefix_digest:"sha256-"+hex64} |
{kind:"select-model",session_id:uuid,expected_revision:int,
 next_revision:int,provider:str,model:str,reasoning_effort?:str} |
{kind:"queue-transaction",session_id:uuid,target_seq:int,action:QueueAction,
 retract_origin:origin_tuple,replacement_origin?:origin_tuple,
 asset_digests:[asset-name]} |
{kind:"retired",target_session?:uuid}
```

The staged candidate path is deterministically
`operations/<first-two-rpc-sha-hex>/<rpc-sha>/payload/`; digests and allocated
identities above make every probe/re-drive independent of request memory.
Operation-private candidate bytes are written and full-synced there **before**
the `prepared` record is published. They are not a carrier, semantic effect,
inventory fact, or readable endpoint state. Publishing `prepared` atomically
commits ownership of the already-durable candidate bytes: once that record is
visible, every byte needed to re-drive its immutable intent MUST already be
present and digest-valid without rereading mutable config, instruction files,
request memory, or the Client. A crash before `prepared` may leave only an
unowned payload directory; startup removes such recordless debris after
checking that no operation record names its rpc digest. A crash after
`prepared` must retain the payload until the operation reaches `complete` (or
is retired by the redact rule below).

Phase is monotonic. Normally `response` is present exactly at `complete` and is the
exact application response body. `prepared` is durable before the first
externally visible effect; the operation-private, recordless candidate staging
defined above is the sole exception. Every later phase is published only
**after** its tabled effect is durable. A crash between an effect and its next phase is resolved by
probing the exact target bytes/identity recorded at `prepared`, then advancing
without repeating a non-idempotent effect. On boot, readiness stays false
while records not at `complete` are resumed under the management lock.
Recovery never rolls a committed semantic/config/folder effect backward.
Unknown fields/version, a phase inconsistent with durable state, or two
records claiming the same rpcId fail-stop.
During redact, a complete source-targeted record containing a forbidden value
is atomically replaced under the management lock with the same
operation/request hash/phase, `rpc_sha256 = sha256(rpc_id)` and no `rpc_id`,
`intent:{kind:"retired",target_session}`, and `retired:"redact"` but no
`response`; its paired rpc file becomes retired in
the same rewrite operation. Retry returns `idempotency-conflict`. No prepared
or in-progress operation may be retired.

Operation-specific phase meanings are exact:

| operation | `carriers` | `semantic` | `inventory` |
|---|---|---|---|
| workspace create/rename | validated workspace config candidate durable in staging | config publication durable | workspace metadata durable |
| session create | config and instruction assets plus optional session settings durable in staging | genesis barrier and active-folder publication durable | workspace metadata updated |
| fork | rewrite `op.json` prepared and assets copied | destination folder published per rewrite-publication | workspace metadata updated |
| archive/unarchive | source UUID, expected active/archive path, directory identity and valid-prefix digest durably reserved | rename plus both required directory syncs durable | active/archive inventory metadata updated |
| discard | source UUID, directory identity and valid-prefix digest durably reserved | folder retired into `.rewrite-trash` and deleted, both directory syncs durable | no membership to update; op record closes |
| selectModel | session-settings candidate durable in staging | session-settings atomic publication durable | model/inventory projection revision updated |
| queue edit/remove/steer | target tail snapshot and any replacement assets durable | committed queue transaction result durable | one whole queue snapshot eligible for emission |

A queue transaction rejected by the worker's under-lock validation takes the
closed failure path `prepared` → `complete`: it skips `carriers`, `semantic`,
and `inventory`, authors no event, and stores the exact typed error response in
the operation record before the rpc record is completed and the gate is
released. The durable complete-error decision is replayed forever; recovery
never revalidates it against a later tail.

The other durable mutations use the event origin tuple as their semantic
deduplication proof and the endpoint rpc carrier above as the exact response
authority.
`session.rename`, prompt, cancel and respond are complete only at
the worker/supervisor receipt described below.

For archive/unarchive, `carriers` is only that durable reservation; it never
claims that a `flock` survived a crash. Every first execution and recovery
attempt reacquires the exclusive folder lifecycle lock, then revalidates the
path, directory identity and valid-prefix digest before rename. A mismatch is
corruption, not permission to operate on the newly occupying directory.

`endpoint-management/workspaces/<workspace-id>.json` contains
`{v:2,workspace_id,path,created_at,metadata_updated_at,config_digest}`. The id
is immutable and equals the config workspace id. A config without metadata is
imported once through a synthetic operation before readiness; `created_at` is
that operation's persisted `started_at`, not filesystem mtime. `Workspace`
uses the config's current path/title, the stable metadata `created_at`, and
`updatedAt = max(metadata_updated_at, every member session updatedAt)`.
Session `updatedAt` is epoch milliseconds derived from the maximum `ts` in the
longest valid semantic-ledger prefix (genesis included). The folder manifest
may cache that maximum only when it names the validated tail digest/seq;
inventory never opens or mutates endpoint transcript history to compute it.

Readiness and the config watcher recompute each workspace config digest. A
mismatch with metadata is reconciled by one synthetic journaled operation:
it preserves `created_at`, persists one `started_at`, validates the new config,
then atomically publishes the new canonical path/digest with
`metadata_updated_at == started_at` before emitting `host/workspace-changed`.
Retries reuse that timestamp. An invalid config keeps readiness false (or the
previous valid runtime snapshot active after readiness) and never derives a
time from mtime.

This endpoint metadata sidecar is not the workspace configuration authority.
The latter is `workspaces/<workspace-id>/workspace.json` under the data root
and owns the ordered `{id,path}` folder bindings. A session-create intent and
the resulting genesis record carry the selected stable `folder_binding` in
addition to workspace id. `cwd` remains the current resolved execution path
for compatibility and launch, not the permanent identity of the thread.

### Create, fork, archive and model publication

`workspace.create` canonicalizes an existing directory. Repeating a request
for the same canonical path returns its existing workspace with
`created:false`; a path that is a later folder of another workspace
([`workspaceFolders.v1`](client-extensions.md#workspace-administration)) is
`workspace-invalid-path`; otherwise it allocates a UUIDv7 workspace id, publishes the
workspace config, then metadata, and returns `created:true`. The created
config carries a seeded `policy`: `allowed_tools` is every fixed tool except
the role selectors (`plan`, `summary_artifact`, `verify`, `report`),
`writable_roots` is the created folder, and `network` is `true`; the
per-session permission mode ([approvals](#approval-policy)) decides what may
run. `workspace.policy.set` replaces the seed like any authored policy, and
`workspace.relocate` moves every writable root at or below the relocated
folder to the new directory. A workspace document that still has no `policy`
(created before seeding existed) receives the same seed once, when the
management store next opens; a present policy is never rewritten. Rename changes
only config `name`, then advances metadata time. Timestamps are the persisted
operation timestamp and therefore stable across retry.

`session.create` resolves workspace/cwd exactly once in `prepared`:

1. With `workspaceId`, that workspace MUST exist. Omitted `cwd` selects its
   first authored folder; a supplied `cwd` MUST canonicalize to exactly one of
   its current folder paths or the request is `workspace-invalid-path`. The
   matched stable folder id is frozen as `folder_binding`.
2. With `cwd` only, exactly one folder path across active workspaces MUST
   canonicalize equal to it. Zero matches is `workspace-invalid-path`;
   multiple matches is `workspace-ambiguous`.
3. With neither, exactly one active workspace MUST exist; otherwise the
   request is `bad-request`.
4. Nonempty `agentPreset` is `unsupported-capability
   {operation:"session.create.agentPreset"}` natively because no preset
   mapping is defined. Omission is the only supported arm and the success
   value omits `agentPreset`.
5. `identityProfile` explicitly selects a short model-facing identity;
   an unknown value is `bad-request`. Omission writes `auto` in genesis. Before
   the first model call, the worker classifies the first admitted user text as
   `coding` or `general` and records the selection in a runtime-only state
   event. Code signals take precedence; ambiguous requests use `coding`.
   The identity cannot change within a session. Older genesis records without
   this field read as `coding`. Forks retain the selection event, and
   delegated child sessions inherit the resolved identity.

It then allocates
or validates the requested UUID, and stages the complete folder. It resolves
and publishes the ConfigSnapshot asset and InstructionSnapshot asset **before**
authoring genesis; genesis names both digests. Only after the genesis barrier
does it publish the active folder and update workspace membership. A retry at
any boundary resumes the operation and returns the same UUID. A requested UUID
whose existing genesis/request hash differs is `session-conflict`.

The bound ConfigSnapshot retains every workspace folder in authored order and
stores the matched path separately as `selected_cwd`. Worker launch and the
tool cwd use that selected path;
instruction capture still freezes all folders. Recovery rejects a snapshot
whose selected path no longer corresponds to the recorded stable binding.

The endpoint has a fixed maximum of 256 active native sessions. Session create
and unarchive reserve one slot under the management lock before folder
publication; when no slot exists they return `active-session-limit {limit:256}`
without changing durable state. Archive releases its slot only after the
archive rename and inventory phase are durable. This matches the transport's
256-subscription-per-socket bound and makes all-session mux registration
atomic: the endpoint never truncates or silently omits an active session.
Boot, manual folder restore, and operation recovery apply the same cap before
readiness: discovering more than 256 active folders keeps readiness false with
`active-session-limit {limit:256}`. No mux upgrade or partial attachment is
permitted until an operator archives/removes enough folders through an offline
repair path.

`session.fork.atSeq` is an **endpoint seq**, not a Kernel seq. When absent it
means the last complete endpoint projection group when no turn is open, and
otherwise the final slot of the projection group that ended the last settled
turn, so a fork of a running source copies exactly the settled prefix; a
source whose only turn is still open is `invalid-at-seq`. When present it MUST name
the final slot of a non-stream projection group in the durable endpoint
journal. The fork anchor is the greatest kernel seq in that row's
`kernel_seqs`; stream-only seqs and a seq inside a 1:N group are
`invalid-at-seq`. Rewrite publication materializes the source Kernel prefix
through that anchor, remaps every registered reference, creates a new genesis
UUID and an empty endpoint journal, and publishes by rewrite-publication.

Archive and unarchive are independent operations. Archive waits for the
exclusive catalog-membership lock, then makes one nonblocking exclusive
lifecycle-lock acquisition; a held lifecycle lock returns
`session-running {sessionId}` without waiting, stopping, or queueing the
archive. After acquisition it revalidates the durable reservation, publishes
the folder under archive, emits host inventory removal
and archived-set change, and terminates its mux attachment. The Client retires
its local projection. Archived requests fail until explicit unarchive.
Unarchive restores the folder, emits host inventory/archived changes, and on
the next mux attachment sends a fresh `session/subscribed`; the Client
rematerializes from history. An approval answer arriving while archived is
rejected and must be redelivered after unarchive with the same rpcId.

Session model selection is stored in the thread folder as
`session-settings.json`:

```text
{format:1,revision:int>=1,provider:str,model:str,reasoning_effort?:str}
```

It is a config-authority file, moves with archive, is copied into a fork's
staged folder, and is included in later ConfigSnapshot resolution ahead of
workspace/global defaults. Selection publication requires the named
session-scoped catalog route to be executable. One per-session admission gate
serializes prompt, respond, cancel, `session.updateQueue`, model selection,
ensure-running, and worker spawn. The closed acquisition order is **admission
gate → management-operation lock → line/lifecycle lock**; no path may acquire
an earlier authority while holding a later one. Every handoff or locked append
rechecks the durable management pre-gate while still holding admission, so
publication of `prepared` and classification of a later delivery cannot pass
each other. Prompt takes the gate before resolving a worker target and retains
it through its durable input receipt. While a prompt is admitted, ensure and
spawn cannot pass the gate. Cancel has scheduling priority over newly waiting
ordinary operations, but never interrupts a queue transaction after that
transaction's `prepared` publication: it sets the volatile no-new-work stop
gate immediately, waits for the gate-exempt transaction completion, then is
the next admitted durable delivery. This cannot deadlock because the management
recovery worker does not require ordinary admission. Model selection succeeds
only when the locked tail is `settled`, has no live unconsumed input, has no
incomplete management/recovery operation, and no line has a live lock holder;
otherwise it returns `session-running` and publishes nothing. Thus a durable
prompt makes selection ineligible before releasing the gate, while a successful
selection necessarily precedes the next prompt's durable input. With an
eligible tail it publishes `actual revision + 1`, releases the gate, and
returns the exact selected value only after directory durability.
A live worker never rereads it: the selection
affects only a later worker spawn and that spawn's immutable ConfigSnapshot.
Therefore success guarantees that the next accepted prompt cannot reuse an old
worker snapshot: the prompt's durable input blocks another selection until the
spawn captures the setting in its immutable ConfigSnapshot. A caller must
explicitly stop/wait and retry selection when a turn is active; no implicit
stop or in-process model mutation is permitted.

### Prompt and queue transaction

Prompt text maps byte-for-byte to Kernel text blocks. Image prompt parts use
the asset derivation in §JSON notation. `mode:"queue"` authors one keyed
file-scoped `input`; `mode:"steer"` authors an input with `steer:true` and is
accepted only while the current turn can admit steering. The success response
means the input barrier receipt is durable, not that a turn completed. It is
owned by that durable input barrier: after the receipt exists, a worker
spawn failure cannot invert the RPC into an error. Sweep/reconcile owns later
ensure and exact rpcId retry returns the same cached success. The endpoint's
active-session gate and the delivery authority perform one eligibility decision;
there is no second pre-materialization delivery preflight.

`content` is nonempty. `clientTimeZone`, when present, is nonempty ASCII with
no leading or trailing whitespace and is either exactly `UTC` or matches
`[A-Za-z][A-Za-z0-9_+.-]*(/[A-Za-z0-9_+.-]+)+`; endpoint admission also
requires the host time-zone database to resolve it. Invalid content/timezone is
carrier `invalid-request` before an rpc carrier or asset is published. The canonical string
is stored as the prompt rpc carrier's `projection_metadata.client_time_zone`.
It is exact-retry and diagnostic correlation metadata only: it is omitted from
semantic input bytes, provider rendering, queue snapshots, SessionEvent bytes,
and rewrite destinations. Fork/redact do not copy the endpoint rpc carrier, so
no public projection may depend on this field.

For a live worker, the endpoint sends worker-control `input` and waits for its
non-coalescible `receipt {delivery,seq,deduplicated}`. For no holder, the
supervisor may perform only the locked-append path already allowed by D-4 and
then returns the equivalent durable receipt. After that receipt it releases
the admission gate and invokes ensure-running from the new durable tail; the
RPC response never waits for the worker process or provider turn to succeed.
Loss after authoring but before
receipt is `accepted-but-not-confirmed`; an identical retry deduplicates by
origin tuple and re-acks the original seq.

Each current unconsumed input projects to one queue item whose `id` is
`"input:" + decimal-input-seq`; placement is `queued`, or `steering` for a
pending steer. Kernel-internal injected context may use `context` and is not
mutable through this endpoint. `session.updateQueue` resolves `itemId` against
one locked tail snapshot:

- `edit` authors a keyed `queue_edit` superseding the unconsumed
  input, followed by a keyed replacement `input` carrying the new content;
  one operation succeeds only after both barriers are durable. The replacement
  has its own later ledger seq and therefore appears at the live queue tail;
  there is no durable position field with which to preserve the old row position.
  Editing a pending steer preserves `steer:true`; editing a queued item keeps
  it non-steer. Endpoint-private prompt metadata is not copied into the
  replacement or any public projection.
- The native endpoint requires nonempty edit content and accepts only
  `{type:"text",text:str}` blocks;
  another block type is `attachment-error
  {reason:"QUEUE_EDIT_NON_TEXT"}` and authors nothing.
- `remove` authors one keyed `queue_edit` superseding either an unconsumed
  queued input or a pending unadmitted steer.
- `steer` authors one keyed `queue_edit` superseding the queued input and one
  keyed `input {steer:true}` with identical content. It is valid only while
  the current turn admits steering.

The two-event edit/steer operation uses derived child keys
`rpcId+"/retract"` and `rpcId+"/replacement"`; the endpoint response is cached
under the original rpcId. The management operation is published `prepared`
before delivery of the single worker-control `queue_transaction`; only its
`queue_transaction_result` completes the endpoint response. Remove uses the
same operation journal and gate even though its committed result names a
one-event range.

The management lock is also a work-launch gate for the target session. While
one queue transaction is incomplete, no ordinary ensure, turn open, queue
projection, or later delivery may pass it. A live worker completes the whole
transaction at one yield while retaining its line lock. If it dies after the
retraction barrier, the supervisor starts a gate-exempt reconcile worker and
redelivers the same transaction; origin lookup appends only the missing
replacement. Boot operation recovery runs before sweep/readiness. The gate is
released only after a committed transaction result and operation `complete`
phase are durable, then one whole `session/queue` snapshot is emitted. A
rejected result instead publishes the exact complete-error operation/rpc
records, emits no queue snapshot for a nonexistent mutation, and releases the
gate. Thus the durable
intermediate prefix is neither runnable nor externally observable as a queue
success. Consumption and queue editing serialize on the line lock, so event-
v1 constraint 8 makes exactly one side win. `queue_edit` alone never releases
or starts work.

### Cancel, rename and respond

`session.cancel` enters the shared admission gate with the priority rule above,
sets the volatile no-new-work stop gate, then delivers keyed worker-control
`stop`; success follows the
`stop_requested` barrier receipt and does not claim the stop cascade has
completed. `session.rename` delivers keyed worker-control `meta`; when no
worker holds the line, the supervisor may use the locked-append whitelist.
The returned `seq` is the durable meta seq. Both retry through origin-tuple
dedup.

After the first accepted root input, the supervisor conditionally appends a
keyed automatic-title `meta` only when the session has no title. Its value is
the first input with Unicode whitespace collapsed and truncated to 40
characters. Winning that durable seed launches exactly one host-owned
presentation request: it reuses the configured DeepSeek adapter and credential,
pins model `deepseek-v4-flash`, supplies no tools, continuation or reasoning
control, and does not alter the session model selection. A nonempty response is
normalized to one line and at most 40 characters, then conditionally appended
as a second title `meta` only if the seed remains the latest title. Therefore a
racing `session.rename` always wins. Provider/config/credential failure leaves
the deterministic seed in place and later prompts do not retry the model call.
Each appended title is projected as a durable `session/title` row before its
inventory projection is published.

Answerable mux frames use a stable rpcId derived from the hold itself; there
is no request journal. Every `approval_request` on a session's main line is a
request with `rpcId = "request-" + lowercase_sha256(sessionId || 0x00 ||
frame-type || 0x00 || causal-kernel-seq)`, where the causal seq is the hold's
own seq. A hold on a spawned child line hashes the NUL-terminated components
`child-request`, root session UUID, source line (`<child-uuid>.jsonl`), frame
type and decimal causal seq; `sessionId` stays the public root session UUID.
The requested envelope is the exact server-request envelope projected from
the hold (method equals the frame type), rebuilt identically on every read,
so reconnect reuses the same bytes and rpcId.

A request is resolved by the ledger alone: the later `approval_response` for
the same `call` on the same line (its seq is the resolution's causal seq;
`grant:true` is `allowed-once`, `grant:false` is `rejected`, an answered
question is `answered`), or the aborting `tool_result` that closes the held
call after stop/recovery (`cancelled`). A hold with neither is pending.
Child lines are ancestry-validated against durable spawn records before
their requests are exposed or answered. Archive, fork and redact need no
carrier step: fork derives new rpcIds under the destination identity and
redact rebuilds requests from the materialized prefix.

An `approval_request` with absent `question` maps to
`approval/requested`: `approvalId = "approval:"+call`, `toolName` is the
paired `tool_call.name`, `callId=call`, and `reason=scope`. With `question`
present it maps to `question/requested`; the value MUST be exactly
`{questions:[QuestionItem,...]}` with a nonempty array:

```text
QuestionItem = {
  id:str, question:str, detail?:str, header?:str,
  options?:[{label:str,description?:str}], multiSelect?:bool,
  intent?:{kind:"plan-review",approve:str}
}
```

Question ids are nonempty and unique and a plan-review `approve` names one of
that item's option labels. A value outside this union is a projection failure,
not a guessed generic question.

For `allowed-once`, `rejected`, or `answered`, the resolution's causal seq is
the matching durable `approval_response` seq. For `cancelled`, it is the
terminal `tool_result {aborted}` seq that closes the held call after
stop/recovery. No resolved frame is exposed before that named semantic
barrier. After the paired `approval_response`, approval resolution emits
`approval/resolved {sessionId,approvalId,outcome}` where outcome is
`allowed-once` for `grant:true` and `rejected` for `grant:false`. Stop or
recovery abort emits `cancelled`. The native endpoint never emits `unavailable`: a
delivery/answerer failure before a semantic barrier leaves the request
unresolved and returns a typed RPC/transport failure so the same rpcId can be
retried.
Question resolution emits
`question/resolved {sessionId,questionRpcId:rpcId,outcome}` with the closed
outcome `answered | cancelled`; a durable answer is `answered`, while stop or
recovery abort is `cancelled`.

`POST /api/respond` accepts exactly:

```text
{type:"client-response",rpcId:str,result:{ok:true,value:RespondValue}}
RespondValue =
  {sessionId:uuid,approvalId:str,outcome:"allowed-once"|"rejected"} |
  {sessionId:uuid,answer:{answers:[JsonValue]}} |
  {sessionId:uuid,cancelQuestion:true}
```

The rpcId MUST name one unresolved requested frame, `sessionId` MUST match it,
and the value arm MUST match that frame type; otherwise the bare response is
`{accepted:false,reason:"unknown-rpc-id"|"already-resolved"|
"response-type-mismatch"|"session-mismatch"|"archived"|"stop-active"}`. No event is
authored in those cases. Approval `allowed-once` maps to keyed
`approval_response {grant:true}` and `rejected` to `{grant:false}`; question
answers author exactly one keyed `approval_response` for the request's causal
held call: `call` is that paired `approval_request.call`, `grant:true`,
`answer` is the request's exact `{answers:[JsonValue]}` object, and `scope` is
omitted. Its origin tuple uses `op:"respond"` and key
`rpcId+"/response"`. The only denial a question accepts is the explicit
`cancelQuestion` arm (§Question cancellation); stop/recovery uses the separate
cancelled resolution path.
Every `{accepted:false,...}` response is a pre-authoring rejection: it creates
or advances no endpoint rpc carrier, consumes no mutation idempotency identity,
and is never cached. The same server-authored rpcId may therefore be retried
with corrected bytes, or after an explicit unarchive. Only a successful
semantic `approval_response` barrier may create/advance the rpc carrier and
cache `{accepted:true}`; that successful request's canonical bytes then become
the sole mutation identity for the rpcId.
`stop-active` is selected under the admission gate whenever the tail's active
stop generation wins classification. It does not append an
`approval_response`; the stop/recovery run eventually records the cancelled
request resolution. A response waiting behind unrelated `recovery_needed`
work is redelivered with the same rpcId only if the ledger still holds the
request unresolved; otherwise it returns `already-resolved` or the cached
terminal result. The stop-active check precedes live-worker versus
locked-append target selection.
The live worker path uses worker-control `approval_response` and success waits
for its D-45 barrier receipt. With no holder, the endpoint holds the same
per-session admission gate, rejects `stop-active` before authoring, then the
supervisor acquires the line lock nonblocking, revalidates the still-unresolved
`parked_hold`, appends the keyed
`approval_response`, completes its D-45 barrier, and returns the equivalent
receipt before releasing the lock/gate and invoking ensure-running. If lock
acquisition loses to a new holder, the endpoint retries through the live-worker
path with the same delivery and origin tuple. If the locked tail is no longer
the named unresolved hold, the call returns the corresponding cached success
or `already-resolved` from the ledger's resolution; it never appends against
a stale frame. The success body is exactly `{accepted:true}`
(the optional `reason` is omitted). A retry after the durable resolution returns the
same success from the rpcId record and never authors a second response.

### Read routes

Native `host.describe` derives every field as follows: `version` is the exact
selected bundle version from deployment; `cwd` is the canonical process
working directory captured before readiness; `home` is the service account's
home from the OS account database (never an untrusted request/environment
override); `provider` and `model` are both present only when the validated
global config default contains both; `attachedSessions` is the number of
distinct session ids in the same complete active-session inventory snapshot
used by `events.mux` (including settled and parked sessions without a worker);
and `canOpenPath` is always false for this headless native endpoint.

Workspace titles are Unicode-whitespace-trimmed UTF-8 with 1...256 bytes and
no control scalar. `workspace.create` defaults title to the canonical path's
last component and applies the same validation. `workspace.list.items` are
ascending by `workspaceId`; each `sessionIds` and `archivedSessionIds` array is
ascending UUID order and duplicate-free. Active `sessionIds` exclude archived
folders. `session.list` excludes archived sessions.

`session.list` with omitted `cursor` returns the complete active list, ordered
by descending `updatedAt`, then ascending `sessionId`. The imported v2 cursor
field is reserved: any present, nonempty cursor is `invalid-cursor`; there is
no hidden page because `SessionList` has no next-cursor result field.

`SessionSummary` is a pure projection of authority:

- `running` is true exactly when any schema-valid line file in the session
  folder has a live file-lock holder, or the supervisor has registered a
  worker for the session that it has not yet reaped (the D-2 routing cache
  covers the window between spawn and lock acquisition; the sweep classifies
  with the same disjunction). Every registration and reap emits
  `host/session-status`, so the flag follows worker starts and exits instead
  of freezing at the stream baseline;
- `tail` is the spec/tail-lifecycle classification of the session's main
  line under that lock fact: one of `running`, `stopped_active`,
  `answered_hold`, `parked_hold`, `recovery_needed`, `unstarted`, `settled`.
  It is the only wire fact that separates a parked hold from a run;
  `running == (tail == "running")`;
- `blank` is true exactly when `main.jsonl` has no valid `turn_open` in its
  effective prefix;
- `cwd` is the current canonical `Workspace.path` named by genesis.workspace;
- `parentSessionId` is present only for a completed endpoint `session.fork`
  operation and equals that operation's source session; `origin` is then the
  literal `fork`, otherwise both fields are omitted;
- `ephemeral` is present as `true` exactly when the genesis carries the flag
  (§Ephemeral sessions), otherwise omitted;
- `agentPreset` is omitted natively because nonempty presets are rejected;
- `projections` is omitted unless a title projection exists. The only native
  inventory projection is
  `{asOfSeq:endpoint-seq,values:{sessionTitle:{title:str}}}`, derived from the
  latest effective keyed title `meta` and its durable `session/title` endpoint
  row. No other `values` key is emitted.

The semantic-ledger `updatedAt` rule above applies identically to create, fork,
list, host inventory and mux lifecycle frames.

#### Native context presentation

Control baselines and incremental control projections include `contextUsage`
with `{usedTokens:int|null,contextWindow:int|null,basis:str,provider?:str,model?:str}`.
Counts are nonnegative JSON safe integers; capacity, when present, is positive.
Basis is `reported`, `estimated`, `upperBound`, or `unknown`; unknown has a null
numerator. Native Kernel derives a conservative byte upper bound from the actual
prepared request and invalidates it when route/config/epoch or opaque continuation
prevents a defensible bound. This is separate from cumulative billing usage.

Context uses a persisted per-session projection clock, independent of journal
snapshot cuts. Clients retain it even without an attached history window. Cache
publication errors do not change the outcome of committed management commands;
control baseline refresh retries publication. Catalog models additionally expose
`contextWindow` from their resolved configuration.

Native also emits the optional `contextDetails` v1 replacement envelope (common
ClientKit Context details v1). Required fields are `schemaVersion:1`, `revision`,
`evidence`, `coverage`, `relation`, and `rows`; `usage` embeds the same legacy
`contextUsage` sample. Evidence uses `origin:harness`, `scope:preparedRequest`,
`sourceKey:native-prepared-request`, `clock:native-context-sample-v1`, selected
provider/model, and valid request/epoch/asset revision identity when observable.
Both keys are persisted together by the per-session clock and installed in the
control cache under one lock before emitting incremental frames. The frames have
one shared sequence; details is emitted first, and clients track each key and
use embedded usage for atomic interpretation. Reconnect/control baseline reconstructs both from one ledger and
configuration snapshot. Legacy saved usage-only values migrate on next refresh.

Rows use serialized JSON byte `upperBound` measures with accounting ID
`native-serialized-json-bytes-v1`: messages (including calls/results), system
prompt, and actual tool definitions. System/developer message entries are removed
from the message bucket before system measurement. Tools expose safe identifier
titles and name-identity children (schema identity only for unnamed declarations),
deduplicated within a declaration group;
Google functionDeclarations and Chat function wrappers are handled. Children
explain their parent and are never added again. Tools flagged `defer_loading`
appear as deferred declarations, outside occupied buckets; these flags alone do
not prove loaded/activation state. Loaded schema results remain message content.
No wire-name heuristic claims System/MCP provenance.

Details are `coverage:partial`, `relation:independentEstimate`: request component
bounds do not apportion provider framing, subsequent events, or the legacy total's
provider-observation maximum. No tokenizer or exact token claim is made. Skills,
Memory file and MCP-instruction manifests, server attribution, and reserve are
unavailable, explicitly explained by `unavailableReason`; compact trigger is not
a reserve. Raw request text, descriptions, schemas, file contents, endpoints and
credentials are not emitted. Missing/invalid/opaque request samples replace rows
with an empty list and embed unknown usage, retaining only current route evidence.
Example: `fixtures/context-details/native-v1.json` (synthetic content).


#### Native model projection

The provider-runtime readiness seam supplies, per configured provider, this
closed snapshot for the selected session:

```text
{provider:str,status:"ready"|"failed",
 models:[{id:str,efforts:[str],default_effort?:str}],
 failure?:"unavailable"|"dialect-unproved"|"invalid-credential"|"network"|"misconfigured"}
```

Provider ids follow config authored order; every enabled configured model is
eligible and follows config model order. Runtime model ids do not define the
catalog. Every configured provider emits one group whose `id` is the authored provider
id and whose `name` is the authored `Provider.name`; a legacy config that omits
`Provider.name` uses the id unchanged as its compatibility display value. Each
eligible model has `id == name == model`, omits description, and includes
`reasoning` only when `efforts` is nonempty. Efforts are ASCII-sorted unique
`{id,name}` pairs with `id == name`; `defaultEffort`, when present, must be a
member. A failed provider still emits its configured group and one failure
whose `id` is the provider id and whose `name` follows the same authored-name
rule; failures follow config order.

`current` is the immutable ConfigSnapshot selection and includes
`reasoningEffort` only when present. `routable` is configuration eligibility:
the provider and enabled model exist and a supplied effort belongs to the
resolved profile. `session.selectModel` uses only that predicate; an omitted
effort is valid. Credential availability and dialect/runtime readiness are
display failures here and become durable worker attempt errors, never a
selection preflight. No display label is guessed from an id.

History pagination is exactly §Journal projection. Models are computed
independently from transcript readiness. Attachment retrieval follows §JSON
notation and never returns bytes from another session merely because the
content digest exists globally.

### All-session stream lifecycle

Opening `events.mux` atomically attaches all currently active endpoint
sessions in ascending sessionId order. Each attachment emits
`session/subscribed` before any later event and replays unresolved approval or
question requests with their stable rpcIds. A newly created or unarchived
session is attached and subscribed before its first session frame. Archive
removes the attachment after all earlier durable frames; no later session
frame may follow for that generation. A subsequent unarchive is a fresh
attachment, not continuation of the retired Client projection.

`session/projection` is a non-seq push frame, but its required payload `seq` is
the durable endpoint seq of the causal row from which that value was derived
(for native title, the `session/title` row), never a Kernel seq or a new
allocation. Per projection name frames emit in increasing causal seq after the
named row is durable. Reconnect may replay the latest value after
`session/subscribed`; equal `(name,seq,value)` is idempotent and a lower seq is
ignored. A value ahead of subscribed `lastSeq` is a protocol error/live-gap.

Because active session publication is capped at 256, one mux connection always
has capacity for the complete active set. Failure to reserve the complete set
before the snapshot rejects the upgrade; partial attachment is forbidden.

`events.host` reports inventory invalidation. Creation/unarchive emits
`host/session-added`; archive emits `host/session-removed` and
`host/archived-sessions-changed`; running changes emit
`host/session-status`; workspace create/rename emits
`host/workspace-changed`. These are push frames and do not consume endpoint
seq. Every server-request envelope has `method == payload.type`.

Opening either stream while the endpoint cannot create one consistent
registration snapshot fails the WebSocket upgrade with the transport's 503
body. After upgrade, semantic stream failure is one `stream/error` envelope
followed by the matching close row in §Transport listener.

### Inline attachment policy

The `attachments.v1` capability provides the read-only `attachments.policy`
method. Its closed payload accepts an optional `sessionId`; omitting it requests
the deployment-wide policy for an unsaved draft. The current policy is deployment
wide even when a syntactically valid session identity is supplied.

The response fields are `inlineMediaTypes`, `maxAttachmentBytes`,
`maxImageDimension`, and `maxImagePixels`. They derive from the same constants as
image materialization. Absent optional count/aggregate limits do not advertise
an unlimited HTTP request body: transport body admission remains in effect.
The native Swift client implements `SessionAttachmentPolicyEndpoint` by calling
this method rather than embedding independent policy constants.

### Approval policy

The `approvals.v1` capability provides `approvals.policy` (read-only),
`approvals.mode` (read-only), and `approvals.select` (mutation).

A session has exactly one durable permission mode, stored as
`threads/<sessionId>/permission-mode.json` = `{format:1, mode}` (canonical
JSON + LF, atomic replace, file mode 0600). An absent file is
`workspace-write`. A corrupt file is `workspace-write` plus a diagnostic,
never a crash. The three modes, in menu order, are the ids the Tekes composer
already maps for other hosts:

| mode                 | title           | read-only / workflow | edit  | execute | destructive |
| -------------------- | --------------- | -------------------- | ----- | ------- | ----------- |
| `read-only`          | Read Only       | allow                | deny  | deny    | deny        |
| `workspace-write`    | Workspace Write | allow                | allow | allow   | durable hold |
| `danger-full-access` | Full Access     | allow                | allow | allow   | allow       |

The approval-class taxonomy of `builtin-tools.md` is never downgraded; only
the decision per class changes. Execution under `workspace-write` is already
confined by the immutable sandbox policy. A hold is the same durable per-call
approval request as before (`{call, tool, class}`), answered through the
existing actionable protocol. The worker reads the mode from the session
folder at the start of every tool batch, so a selection applies to the next
tool call of a running session without a restart.

`approvals.policy {sessionId?}` (session identity validated for shape only)
returns `{threadLevel:[ApprovalOption], serverLevel:{options:[], currentValue:null}}`
where an option is `{value, title, description}` and `threadLevel` lists the
three modes in the order above. `approvals.mode {sessionId}` returns `{mode}`
for a live or archived session. `approvals.select {sessionId, mode}` writes
the record and returns `{mode}`; an unknown mode is `bad-request`, an
archived session is `archived`, an unknown session is `session-not-found`.

`commands/run` treats the reserved verb `permission` exactly like `compact`:
a request whose name is `permission` with `arguments` = the mode (no catalog
entry is required, and `commands/list` never lists it), or a catalog command
whose expanded body is exactly `permission <mode>`, writes the same record
through the same session admission gate and returns the ordinary command
receipt (`seq` 0: the mode is session metadata, never an event) without
authoring model input. A missing or unknown mode is `invalid-arguments`. The
mode is reported as `permissionMode` on the session's inventory summary, and
both `approvals.select` and the `permission` verb re-announce the session so
every inventory subscriber receives an `upsert` carrying the new mode. The
native Swift client implements `SessionApprovalPolicyEndpoint` with
`approvals.policy` and applies a composer selection by running the
`permission` command with the mode as its argument.

### Host directories and mention candidates

`hostFiles.v1` registers `directory.list {path?}` -> `{path, home, crumbs, entries, truncated}`
(directories only, byte-ordered, hidden = leading dot, capped at 2000 entries; the
default and `home` are the user's home directory, not the Kernel state root),
`directory.create {path, name}` -> `{path}`, and the composer mention sources
`session.references.files {sessionId, query}` -> `{items:[{path, kind}]}` (paths
relative to the session's primary workspace root, bounded walk, no symlink
escape) and `session.references.sessions {sessionId, query}` ->
`{items:[{sessionId, label, cwd?, sameWorkspace, createdAt, mention}]}` where
`mention` is the host-authored token `@thread:<sessionId>` the model sees verbatim.
The native Swift client implements `SessionDirectoryPickerEndpoint` (its
`pickDirectory` is an AppKit open panel, not a Kernel method) and
`SessionReferenceEndpoint` with these methods.

### Message feedback

`feedback.v1` registers `feedback.list {sessionId}` -> `{items:[Feedback]}`,
`feedback.put {sessionId, messageId, rating, ifVersion, note?}` -> `Feedback`, and
`feedback.delete {sessionId, messageId, ifVersion}` -> `{deleted:true}`.
`Feedback = {messageId, rating:"positive"|"negative", version, note?, updatedAtMilliseconds}`.
Feedback is Client-facing durable metadata under `feedback/<sessionId>.json`
(canonical JSON, atomic replace, monotonic per-session version counter), never a
ledger event and never provider input. `ifVersion` is a compare-and-set: `null`
requires absence, a string must equal the current version; a mismatch is
`version-conflict {current}`. The native Swift client implements
`SessionFeedbackEndpoint` and `SessionFeedbackNoteEndpoint` with these methods.

### Instruction settings

`settings.v1` exposes the user-level instruction `settings.json` (see
`instruction-snapshot`) as one namespace `policy`: `settings.describe {}` ->
`{writable:true, hasDocument:true, namespaces:[Namespace]}`,
`settings.mutate {ns, operations, expectedRevision?}` and
`settings.update {ns, patch, expectedRevision?}` -> `Namespace`, and
`settings.document {}` -> `{path}`. `Namespace.applies` is `next-launch`: the
worker captures the snapshot at spawn. Revisions live in the sidecar
`settings.revision` (`<n> <sha256>`); an out-of-band edit bumps the revision so a
stale `expectedRevision` is rejected with `stale-revision {current}`. The native
Swift client implements the settings, settings-update, settings-invalidation and
settings-document protocols with these methods; opening the document is an AppKit
action on the returned path.

### Agent presets

The Kernel exposes immutable initial identity presets through the additive
`initialPresets.v1` capability: `session.initialPresets {}` returns
`{presets:[{id,title,configurationText}],defaultID:null}`. Coding and General
are sourced byte-for-byte from `crates/tools/prompts/coding.md` and `general.md`;
`{model}` remains unresolved in the catalog. A missing choice retains automatic
selection from the first user input. `session.create {identityProfile:"coding"|"general"}`
selects explicitly and stores the identity in genesis. Inventory exposes the
resolved `identityProfile`, absent while an automatic choice is pending. Follow-up
requests and restart retain the identity; no preset mutation operation is exposed.

These initial identity presets do not restore the retired `agentPresets.v1` or
`session-presets` subsystem. `session.create {agentPreset}` remains unsupported.
User instructions still come from the instruction snapshot's `AGENTS.md` scopes.

### Goals

`goals.v1` keeps one durable goal record per session at
`goals/sessions/<sessionId>.json` with a revision compare-and-set:
`goals.get {sessionId}` -> `{goal: GoalView|null}`, `goals.edit {sessionId, ref:{id,revision}, objective}`
(creates when no record exists and `ref.revision` is 0), `goals.clear {sessionId, ref}` ->
`{id, revision}`, `goals.pause`/`goals.resume {sessionId, ref}` -> `GoalView`.
`GoalView = {id, revision, objective, phase, maxGoalRounds, roundsStarted, createdAt, updatedAt, activation, blockedReason?}`.
Model-authored `new_goal`/`set_goal_state` records in `goals/log.jsonl` are folded
read-only into `phase` and `blockedReason` when their goal id matches. Goal
rounds are counted from durable `turn_open` events after creation. Kernel has
no separate activation state; `activation` is derived from `phase` and
`maxGoalRounds` is 16, bounding automatic goal turns. A stale `ref` is `goal-stale {current}`;
an impossible transition is `goal-invalid-transition {from,to}`. While the record is
`active`, `blocked`, or `paused`, its id is bound as the worker's host goal id at launch, so the
selection-controlled `new_goal`/`set_goal_state` tools are in that session's catalog;
launch bindings are immutable, so a goal edited mid-session binds at the next launch.

Each root model request with a bound goal receives the complete objective and
current phase from the separate goal record during prompt compilation. History
compaction does not alter this record, and continuation does not add a copy of
the objective to the transcript. A completed Final Answer settles only its
turn. After settlement, the worker opens a new goal-triggered turn while the
durable phase is `active`, the round bound is available, and no queued user
input or stop takes precedence. `complete`, `blocked`, `paused`, and an exhausted
round bound stop automatic continuation. `set_goal_state` changes phase but
does not edit the objective. No independent verifier is required by this flow.

### Subagent catalog

`subagents.v1` registers `subagents.list {parentSessionId}` ->
`{entries:[{kind:"parent"|"child", id, mode?, activity?, hasChildren?, label?}], parentAvailable}`
folded from the parent ledger: one child per `spawn` (`label` from the paired
delegation state, `activity` `running` until its `child_result`, `mode:"bounded"`).
Kernel children are bounded delegations without parent-owned continuation or
interrupt; the native Swift client reports `subagent-control-unavailable` for
`promptChild`/`cancelChild` and never cancels the parent.

### Question cancellation

A pending question can be rejected without stopping its session: the mux
`actionable-respond` outcome (and the `/api/respond` value) accepts
`{sessionId, cancelQuestion:true}` for a question request, authoring the keyed
`approval_response {grant:false}` for the question's causal held call with no
`answer`. The worker closes the call with a denied `tool_result`, the turn
continues, and the projection emits `question/resolved {outcome:"cancelled"}`.
The mux carrier supplies `sessionId` from the located request when the client
omits it. The native Swift client implements `SessionQuestionCancellationEndpoint`
this way.

### Session file reads

`sessionFiles.v1` registers read-only `session.files.stat`, `session.files.read`
and `session.files.readBytes`. Each request supplies `sessionId` and `path`;
read methods additionally accept optional `offset` and `limit`. Stat rejects
range fields. Text offsets begin at 1, byte offsets at 0. Text pages default to
200 lines (maximum 10,000); byte pages default to 256 KiB (maximum 4 MiB).
Text line/page allocation is bounded to 4 MiB.

Paths resolve against the session's persisted workspace/folder binding.
Absolute paths must be inside an authored workspace root. Parent traversal and
symlink escape are rejected. Results use the Swift file stat/text/byte DTOs and
share an opaque version derived from file identity and change metadata.
Native whole-file reads assemble byte pages up to 1 GiB while checking path,
version, size, offset and EOF consistency. Failure to access a scoped file is
reported as `file-unavailable`; invalid request shapes are `bad-request`.

Built-in file observation uses the authenticated WebSocket
`/api/session.files.changes?sessionId=...`. Each physical subscription emits
`{"kind":"ready"}` first. Subsequent messages use
`{"kind":"change","change":{"absolutePath":...,"version":...}}` or
`{"kind":"change","change":{"absolutePath":...,"absent":true}}`.
Scoped file reads register paths for active subscriptions. Each subscription
maintains independent versions and polls every 250 ms; ordinary client metadata
reads after ready populate the observed paths. Closing the socket destroys the
subscription. The existing connection admission limit and drain signal apply.
This stream is separate from the five baseline/delta synchronization streams.

### Fixture and gate obligations

`fixtures/endpoint-transport/management/` contains one case for each row in
the 20-registration table. Each case contains a valid request/success and a
route-specific failure in transcript order; streams contain an open, a valid
frame, then a typed failure. The checker verifies the registry/table in both
directions, exact request/result/error fields, route path/method, queue and
respond unions, and canonical bytes. Gate 65 cannot pass from carrier samples
alone: it requires all 20 management cases and crash recovery at every
operation-journal phase. The root
`management-control-races.canonical.json` additionally freezes shared lock
order, queue complete-error gate release, and stop-before-answer. Gates 66 and
70 reuse those cases for idempotency and archive/rematerialization.
