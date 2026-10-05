# spec: provider-adapter contract v1 (r3 — R13-2/3/5 closure)

What every `ProviderAdapter` implementation must declare and guarantee.
Carried dialects (TekesRuntime `Network.ModelAPI.*`) are wrapped to this
contract; it is the load-bearing half of D-43's recovery.

## Capability declaration (static, per adapter/model/API path)

| capability | values | meaning |
|---|---|---|
| `continuation` | `stateless` \| `server_managed` | replay vs continuation-id chains |
| `query_by_identity` | `none` \| `available` | fetch a dispatched request's outcome read-only by identity |
| `dispatch_marker_required` | bool | when true, `attempt_dispatched` is written before HTTP; when false, dispatch may occur without a marker (R12-3) |
| `native_deferred_tools` | per REQ-014 taxonomy | carried unchanged |

Implementation status: explicit Anthropic custom-reference and OpenAI client
tool-search preparation, their adapter-direct live gates, and automatic worker
routing are implemented. `native_deferred_tools` is declared per exact route in
the reviewed model capability catalog (`fixtures/provider-dialects/model-capabilities.canonical.json`,
`native_deferred_tools: {mode, routes[{endpoint_owner, gateway_translation}]}`)
and surfaces as `ResolvedDialectProfile::native_deferred_tools()`; a mode is
honored only on its proved dialect and a listed route, never inferred from the
adapter family. In native mode the worker declares every deferred schema
(adapter marks them deferred), renders the whole prefix statelessly, and binds
this turn's durable successful `tool_search` offers as references (unbound
names and stale digests are dropped, not promoted). See the
[native protocol gap audit](../docs/verification/audits/native-deferred-tools-gap.md)
for the live evidence and residuals.

## Render and outcome

- `render(prefix, epoch_profile) → wire bytes` — pure, deterministic,
  prefix-stable within an epoch ([Model context](../docs/data/context.md)); reports exact candidate size for
  D-39 preflight.
- Stream decode yields frames (mirror-only) and exactly one terminal:
  content + sealed fragments + continuation id (server-managed) + usage
  figures or their absence.

## Query-result union (closed — R12-7)

`outcome(response)` | `not_found` | `expired` | `transient` (timeout, 5xx,
connection loss) | `auth_failure` | `malformed` | `unsupported`.

`transient` permits exactly 3 bounded retries of the **query only** in v1,
after delays of 250 ms, 1 s, and 4 s; if the remaining turn wall budget cannot
cover the next delay, retries are exhausted immediately. After exhaustion it
is treated as the `unsupported`
row. Query is read-only and never creates, extends, or confirms server
state.

## Recovery decision table (with [tail-lifecycle](tail-lifecycle.md))

**The key includes the marker policy** (R12-3). "Sent-state" per attempt:
- marker present → *maybe-sent*;
- marker absent ∧ `dispatch_marker_required=true` → **not-dispatched**
  (nothing left the host);
- marker absent ∧ `dispatch_marker_required=false` → *maybe-sent*
  (markerless dispatch is legal for this adapter — never `not_dispatched`).

| sent-state | query result | ordinary run | reconcile-only run |
|---|---|---|---|
| not-dispatched | — | `attempt_recovery {not_dispatched}` → close the attempt (see below) → retry freely | same, then settle `interrupted/recovered` |
| maybe-sent | `outcome` | `attempt_recovery {adopt, inventory, response}` → adopt transaction (event) | same, then settle `interrupted/recovered` |
| maybe-sent | `not_found` \| `expired` | treat as not received: stateless → `{resend}`; server_managed → `{recovery_epoch}` | `attempt_recovery {unresolved}` → close the attempt with `error {recoverable: false, classification: unresolved_dispatch}` → **epoch closed** → settle `interrupted/recovered` |
| maybe-sent | `transient` (exhausted) \| `auth_failure` \| `malformed` \| `unsupported` (incl. `query_by_identity = none`) | stateless → `{resend}` (duplicate billing accepted); server_managed → `{recovery_epoch}` | `{unresolved}` as above |

**Closing the old attempt (R12-7).** Every non-adopt recovery decision
closes the recovered attempt with one barrier transaction *before* any new
work: `error {recoverable: true, classification: transport, attempt,
usage: {availability: unavailable}}` (or the terminal
`unresolved_dispatch` form in the unresolved row) → one barrier →
`attempt_settled` (lease released). Only then may `resend` allocate a
**new** attempt id and re-enter the send order (R4-6), or `recovery_epoch`
append its `epoch` event (whose authority follows event §epoch —
R13-5). There is never a moment with two live chains or an unpaired old
attempt.

**Decision of record** (R13-2): a durable `attempt_recovery` is the
decision of record for its attempt. A later run finding one without its
closure pair completes it exactly as recorded — the closure barrier above
for non-adopt, the asset-driven adopt transaction for adopt — and never
re-queries or re-decides. At most one `attempt_recovery` per attempt id
ever exists (event constraint 2).

## Adopt material (R11-3)

- The single successful query response is written whole to an asset (raw
  native payload) and synced under the D-45 reference protocol before the
  `attempt_recovery {adopt}` barrier. Everything the adopt transaction
  writes — content blocks, sealed fragments, tool names/arguments, usage —
  derives from that asset. **No rule may require a second query.** Tool
  arguments are carried verbatim as `JsonValue` — R13-3 preserves `null`
  and native numeric forms; adoption never rewrites provider values.

## Obligations

1. Usage figures surfaced verbatim (strings for big ints); absence is
   `unavailable`, never zero.
2. Continuation identities and sealed fragments are opaque, version-stamped
   (D-42), fail-closed on mismatch.
3. Rate-limit/transport errors are `recoverable: true`; only
   `provider_terminal` ends a turn as `settle {error}`.
4. Adapters declare their epoch-reset triggers using 11's named reasons.
5. Every capability × sent-state × query-result × mode cell above is
   covered by a conformance case ([Conformance gates](../docs/verification/gates/README.md)).
6. Adapters never deliver a foreign value that collides with the spill
   discriminant — an object whose only key is `$spill` — at a spill
   position; such provider output is malformed (R14-3), like NaN.
7. Model-emitted tool argument text that is not usable JSON (duplicate
   members, truncation, a non-object) is not a malformed response: the call
   keeps its identity and its arguments become the invalid-arguments
   sentinel `{"$tekes":{"invalid_arguments":{error,raw}}}` (`raw` bounded to
   2 KiB), identically at readiness and at the terminal, and a terminal
   manifest must repeat the streamed text exactly. The worker never executes
   such a call: it writes a durable invalid-arguments `tool_result` error the
   model can answer by resending well-formed arguments. Only the provider's
   own envelope being unparseable is `malformed`.
