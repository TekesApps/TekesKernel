# Client V2: historical boundaries and storage mappings

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

## Historical V2 baseline: scope of the remaining sections

This historical document preserves the imported V2 design and Kernel storage mapping.
Their public V2 route/negotiation descriptions are superseded by [the current V3 boundary](../interfaces/client.md), including
`session.history`, `session/subscribed`, the dual mux/host routes and `/api/respond`.
Storage and event semantics remain applicable only where the current V3 contract
explicitly reuses them. Historical acceptance wording describes that baseline, not
new acceptance of the present worktree.

The Tekes client's boundary is fixed by the implemented-and-accepted external
contract owned by Tekes Client:
`Tekes/docs/session-event-conversation-transport-v2.md` (`docs/session-event-conversation-transport-v2.md` in the closed-source Tekes client repository),
with its language-neutral
v2 contract fixtures (`docs/session-endpoint-contract-v2` in the closed-source Tekes client repository)
and
implementation/acceptance record (`docs/direct-session-endpoint-client-refactor.md` in the closed-source Tekes client repository).
It is the DSH-compatible Session Endpoint authority for methods,
`SessionEvent`, mux frames, message-aligned history paging, the stitch
algorithm, and error envelopes; the frozen TekesAS JSON-RPC v1 contract is
not an alternative authority for this surface. The client owns the direct
`.dsh` driver and registers the native `.tekes` Kernel endpoint as a default
built-in instance over the same accepted carrier. Its fixed registration owns
the Kernel origin and Keychain bearer contract; there is no filesystem,
network, plugin, or inventory discovery step. The concrete Client adapter is
`TekesKernelHostEndpoint`, never `DSHHostEndpoint`. **The kernel is what stands
behind the native `.tekes` driver.** This doc pins how kernel primitives map to
that contract and supersedes the earlier "ACP at the client boundary" choice.

## Decisions

- **D-62 (closes D-open-4; amends D-58's outer half).** The supervisor's
  client interface implements — or is mapped by a thin driver to — the
  Session Endpoint v2 contract. The remote surface's thickness is therefore
  decided: the §4 method table, §7 history paging, §8–9 mux semantics. ACP
  is no longer the Tekes client boundary; it remains a candidate only for
  future third-party interop. D-58's inner worker protocol is untouched.

## Contract import rule

The Tekes-owned Session Endpoint v2 contract, fixtures, and accepted Client
implementation are the **sole interface authority**. This document specifies
only the projection from kernel primitives into that already-fixed interface;
it does not redefine method/DTO shapes, endpoint-version acceptance, event
semantics, sequence identity, paging, history/live stitching, error behavior,
or Client persistence rules. A kernel limitation is not a v2 adaptation: if it
cannot satisfy the imported contract, the endpoint slice remains blocked until
the mapping is closed. Any intentional incompatibility requires a separately
negotiated protocol version/capability and MUST NOT advertise v2 compatibility.

Slice 6 implements the carrier-neutral projection/service. The exact physical
HTTP/WebSocket binding is [endpoint-transport-v2](../../spec/session-endpoint.md),
and the exact 20-route DTO/author/recovery behavior is
[endpoint-management-v2](../../spec/session-endpoint.md). Both gate Slice
9; neither doc 15 nor transport code may fork the imported DTO authority.

## Canonical external session URI

The user-visible/cross-app session locator is:

```text
<appname>://<as-name>/<as-session-id>
```

For the current product, `appname` is `tekes`. `as-name` is the endpoint's
stable short name — `tekes` for the native Tekes AS and `dsh` for DeepSeek
Harness. `as-session-id` is the AS's opaque **public** Session Endpoint id,
encoded as one URI path segment. The native Kernel/Tekes AS exposes only its
canonical UUID thread id (D-64); it does not construct or report the URI. The
Host/Client supplies `appname` + `as-name` and constructs, for example:

```text
tekes://tekes/018f0000-0000-7000-8000-000000000001
```

DSH retains its own public id vocabulary. Its canonical example is:

```text
tekes://dsh/session-b73a4e1e-e9a4-4af1-8ea8-49300e541cb4
```

The URI host selects the AS/driver; the decoded path segment is the `sessionId`
sent to Session Endpoint methods and mux frames. The URI itself is not a
kernel thread-folder path and is not the `sessionId` field. For native Tekes,
`sessionId` equals `genesis.thread` and the UUID folder name — no second
identity map is introduced. Native disk paths such as `threads/<thread-id>/`
remain private storage layout (01).

Internal compound identities — storage row ids, backend ids, and join keys —
may exist inside a database/join projection but MUST NOT appear in a URI,
inventory DTO, history/live frame, error, clipboard value, or log intended for
users. The AS adapter owns the reversible mapping between public
`as-session-id` and any internal identity.

New code MUST NOT emit the old `/threads/` URI form. A client may accept a
legacy URI only as migration input: resolve it through persisted endpoint and
session mappings, then rewrite it to the canonical form. It must not derive a
new public id by splitting an internal compound id. If the mapping is absent,
fail closed with a typed legacy-URI error rather than opening a guessed
session. Query and fragment components are absent from the canonical form.

## The mapping (kernel → v2)

| v2 | Kernel | Note |
|---|---|---|
| `(sessionId, seq)` durable identity; contiguous, strictly increasing | [endpoint-projection-v2](../../spec/session-endpoint.md) journal sequence; Kernel seqs stay internal in `kernel_seqs` | **not isomorphic**: filtering, reordering, and 1:N projection forbid reusing per-file seq directly. `sourceEventSeqs` always names earlier endpoint seqs, never Kernel seqs |
| `turn` | `turn` | direct |
| `step` (one model/request step) | **attempt** | `step/start` ≈ attempt admitted; `step/end` ≈ attempt settled |
| `turn/start` | durable `turn_open` | the event names the exact consumed input prefix; provider admission happens later and is not the turn boundary |
| `user/message` (initial, steer, injected) | `input` (steer included — D-38) | unconsumed inputs project **only** in `session/queue`; a `turn_open` atomically removes its named inputs from the queue and emits their ordered `user/message` events for its turn; a steer moves on qualifying admission, and an expired steer stays queued marked expired (R15-9). Optional `clientTimeZone` remains endpoint-rpc exact-retry/diagnostic metadata only and never enters semantic input, queue, SessionEvent source, provider input, or rewrite output |
| `assistant/chunk` | provider stream frames projected into the endpoint carrier | history and live follow the imported v2 contract identically; D-9 keeps chunks out of the kernel thread ledger, but a process-local ring buffer alone cannot satisfy durable endpoint seq/history |
| `assistant/message {usage?}` | `output` (+ its `usage`) | per step/attempt, not final-turn-only — matches |
| `tool/call` / `tool/result` | `tool_call` / `tool_result` | call id preserved |
| `turn/end {reason}` | `settle {outcome}` | exhaustive map below (R8-4) |
| `surfaceOp: append \| replace[start,end]`; `sourceEventSeqs` | `supersedes` ranges via the UI projection | same retraction semantics |
| `session/subscribed.lastSeq` + buffer/stitch/dedupe/gap-repair (§9) | doorbell + tail-by-seq reads | kernel never had `conversation/freeze`; §9 is native |
| `approval/question requested/resolved` + `rpcId` + `/api/respond` | `approval_request` hold + keyed answers | `rpcId` ↔ correlation id; answers ride `origin_key` delivery |
| `session/queue` + `session.updateQueue` | pending deliveries + queued/held `input` events | queue projection; `updateQueue` edits are keyed **`queue_edit`** events (file-scoped, runtime-only, ack-bearing barrier; envelope `supersedes` over the edited inputs; reorder = edit + new input) — never a runnable trigger, never a release (R11-2) |
| `session.history {sessionId, beforeSeq?, maxMessages?}` message-aligned pages | derived query over the stable endpoint event range | count append-origin messages, return the complete contiguous raw range (chunks/steps/tools/structural events included), never split a `sourceEventSeqs` group, ascending endpoint seq, explicit `hasMore` |
| `session.fork` | the rewrite projection (01 R7, D-47/50) | |
| `session.attachment` | `assets/` staging via delivery | Prompt image `name`, when present, is preserved in the durable Kernel image Block and the historical `ImageAttachmentRef`; the content-only attachment read may omit it because the selected message already carries the display name |
| `workspace.archiveSession` | authoritative folder archive (01 R5/R8) | Explicit typed mutation. The authoritative AS retains the archived folder, log, assets, and archive state; success is not deletion |
| `workspace.unarchiveSession` | authoritative folder restore (01 R5/R8) | Separate typed mutation and capability, never `archiveSession` with a reverse boolean. Success makes the session eligible for complete rematerialization from the authoritative source |
| `workspace.list/create/rename/archiveSession/unarchiveSession`; `session.list/create/rename/fork` | `workspaces/` config + folder lifecycle + keyed `meta` events | these are the exact native v2 registrations. `session.rename` is a keyed external `meta` mutation and succeeds only after its barrier receipt |
| `session.models/selectModel` | config snapshot + session-scoped provider readiness | exact native projection is endpoint-management-v2 §Native model projection |
| `events.host`, `host.describe`, `stream/error` | supervisor status surface | |
| unknown required fails closed; `ignorable: true` skippable | exported kernel-extension kinds are marked `ignorable`; runtime-only marker events are not exported | |

## Settle → `turn/end` mapping (exhaustive — R8-4)

| Kernel final settle (02's closed reason vocabulary — R9-5) | v2 `turn/end.reason` |
|---|---|
| `completed` | `completed` |
| `error {classification: provider_terminal}` | `error` |
| `interrupted {user_stop}` | `aborted` |
| `interrupted {budget_tokens}` | `max-tokens` |
| `interrupted {budget_wall \| budget_tools}` | `interrupted` |
| `interrupted {recovered}` | `interrupted` |
| any future/unknown reason | **impossible in v1**; a later reason requires a min-reader/versioned projection mapping and otherwise fails closed |
| parked hold (no settle exists — R9-2) | **no `turn/end`**; the approval/question frame stays outstanding |

DSH-only or future methods such as `session.search`, workspace delete/reorder,
or `llm.*` are absent from the native registry and capability-gated; Kernel
never advertises them or returns successful empty placeholders.

Optional product-management routes are governed by
[client-extensions-v1](../../spec/client-extensions.md), not by this mapping.
They are additive, atomic `.tekes` driver capabilities over the same request
carrier and remain disjoint from the twenty methods above. They add no
SessionEvent, mux family, endpoint generation, or Mirror partition. A method
is enabled only after the Client catalog and production registry match in both
directions; retired predecessor names stay absent and fail typed.

**Non-success `tool/result` payloads (R9-5)** — one deterministic wire shape
per kernel outcome, always `error: true` with
`meta: {kernel_outcome, reason}`:

| Kernel `tool_result` | v2 `tool/result` |
|---|---|
| `outcome: {aborted: "crash"}` | `{message.content: [text("aborted: crash recovery")], error: true, meta: {kernel_outcome: "aborted", reason: "crash"}}` |
| `outcome: {aborted: "recovered"}` | `{message.content: [text("aborted: recovered, not executed")], error: true, meta: {kernel_outcome: "aborted", reason: "recovered"}}` |
| `outcome: {denied: reason}` | `{message.content: [text(reason)], error: true, meta: {kernel_outcome: "denied", reason}}` |
| `outcome: {withheld: "scan_failed" \| "quarantined"}` | `{message.content: [text("output withheld")], error: true, meta: {kernel_outcome: "withheld", reason: …}}` |

## Kernel storage adaptations (never wire divergences)

1. **Chunks follow v2 history/live semantics.** The endpoint returns every
   `assistant/chunk` in a selected contiguous history range and emits the
   identical `SessionEvent` shape live. The Client folds historical chunks but
   does not persist or replay-animation for chunks shadowed by a sealed
   `assistant/message`; live chunks use its process-local transient sink. D-9
   means only that streaming partials do not enter the kernel **thread log**.
   The endpoint projection journal retains their stable v2 identities before
   live exposure; the ring buffer is only a reattach optimization.
2. **Wire time is epoch milliseconds; storage stays ISO8601 strings**
   (D-18). Conversion is an endpoint concern; fixtures cover both sides.
3. **History pages are message-aligned projections, not raw reads**: the
   supervisor computes page boundaries (append-origin `user/message` +
   `assistant/message` counting) over the event log; the raw seq range in
   between ships whole, per §7.
4. **Holds emit no `turn/end`** (R8-4/R9-2): a parked hold has no settle at
   all — the v2 turn stays open with the approval/question frame
   outstanding, matching DSH's own hold behavior; the resumed run continues
   the same never-settled turn. The kernel's idle exit is a
   process-lifecycle detail invisible at the endpoint. Unrelated input
   during an open hold queues behind it (visible in `session/queue`).
   **Hold-frame lifecycle (R10-5)**: parked sessions list normally — their
   state is the outstanding frame; answer → `approval/resolved`; denial →
   resolved + the denied `tool_result`; stop or recovery abort → resolved +
   `tool_result {aborted}`; archive ends the subscription, and the frame
   re-emerges from projection on unarchive (the hold is log-derived).
   Once a stop generation is active, `/api/respond` fails pre-authoring with
   bare `stop-active`; no answer event or response identity is consumed, and
   the stop/recovery path emits the eventual cancelled resolution. A response
   held behind unrelated recovery is redelivered only while its request remains
   unresolved.
   Deliveries to an archived thread are rejected with a typed `archived`
   error (tail-lifecycle §Archived threads) — clients surface it and may
   offer unarchive (R13-6).
5. **Archive authority and Mirror lifecycle are asymmetric**: the AS is the
   sole authority for archive state and archived data. Tekes Mirror contains
   only the active projection. After a successful `workspace.archiveSession`,
   the Client removes that Thread's complete local projection and its sync
   checkpoint; it does not retain a second archival truth. The endpoint
   `.tekes` Client driver declares `workspace.archiveSession` and
   `workspace.unarchiveSession` as independent capabilities, and the endpoint
   implements them as independent typed
   request/result/error operations. After a successful
   `workspace.unarchiveSession`, the Client rematerializes the complete Mirror
   projection from the authoritative AS before treating local transcript
   coverage as ready. Direct DSH's driver currently declares only
   `workspace.archiveSession`; it MUST NOT advertise or emulate unarchive until
   DSH publishes a formal reverse route.

## Consequences

- The native `.tekes` endpoint implements this mapping over the supervisor's
  primitives; no client workflow or UI branch may distinguish event semantics
  by backend (v2 §1). Client assembly registers the endpoint as a fixed built-in
  instance; registration/type names are not Kernel wire authorities.
- The executable endpoint-projection and endpoint-management contracts and
  fixtures close canonical
  session-URI parsing/routing/migration, stable
  endpoint-seq allocation/provenance, full raw-range history (including
  chunks), history/live byte-shape equality, reconnect/gap/overlap behavior,
  and each typed mutation's durable author/receipt path.
- The v2 fixtures frozen from DSH surfaces are binding conformance inputs. They
  are owned by the Tekes Client contract package and referenced here, never
  copied into an independently drifting kernel corpus; CI uses the same corpus
  or a byte-identity synchronization gate.
- AppServer4DSH does not exist in the target architecture (v2 §1) — the
  kernel-as-AS serves the same contract natively.
