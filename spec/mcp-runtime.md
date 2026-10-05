# MCP runtime and management v1

This is the executable contract for Slice 13. It defines the generic MCP
carrier independently of any plugin package or first-party extension.
No product package is named or special-cased by this contract. Reference
packages are qualified later only through the same configuration, lifecycle,
and transport paths defined here.

The protocol-facing value domain is I-JSON. JSON-RPC objects may contain
`null` and finite ES6-representable numbers. Duplicate object keys,
non-finite numbers, malformed UTF-8, multiple JSON values in one stdio line,
and unknown required union discriminants fail closed.

## Configuration and identity

An MCP server has the immutable reference
`{workspace_id, scope, name}`, where `scope` is `user | project`. Names are
nonempty NFC strings of at most 128 UTF-8 bytes. A project entry of the same
name shadows a user entry; it never mutates the user entry. Plugin entries use
`scope:"plugin"`, include immutable `owner`, and are read-only through the
standalone management surface.

The closed transport union is:

```text
stdio {
  command: [nonempty string, ...],
  cwd?: absolute path,
  environment: {name: CredentialOrLiteral, ...}
}
http {
  url: absolute https URL,
  headers: {name: CredentialOrLiteral, ...},
  oauth?: credential reference
}
CredentialOrLiteral = {literal: string} | {credential: nonempty id}
```

Loopback HTTP may use `http` only in an explicit test profile. Production
non-loopback HTTP is HTTPS. Raw credential values never enter config,
fixtures, logs, catalogs, launch bindings, or management responses. The
supervisor resolves each named credential into a one-use launch/request
scope; a worker receives no ambient credential environment.

Each entry also carries `enabled`, `always_on`, and
`protocol_mode: "auto" | "legacy" | "modern"`. Because v1 defines no
transport-side modern advertisement for stdio, stdio `auto` performs the standard
legacy initialization below. HTTP `auto` probes modern discovery first and may
fall back only on explicit protocol-era evidence, never on a bare HTTP error or
timeout. Explicit `modern` never falls back; `legacy` directly initializes.
Protocol-era evidence is method-not-found, a structured unsupported-version error
listing a legacy-era version, complete discovery with no mutual modern version
and explicit legacy-era support, or — since 2026-09-05 — a JSON-RPC `-32600`
error whose message reports an unsupported protocol version and enumerates
supported versions of which at least one predates the modern protocol (every
`YYYY-MM-DD` token in the message is read; public servers such as DeepWiki
answer discovery this way with a placeholder response id). Non-authentication
4xx responses may carry a JSON-RPC error as the response only when id/version
match; the textual rejection above is surfaced as the remote error instead.
Error bodies are bounded to 16 KiB. 401, 403, 5xx and any other mismatched
identity or free-text hint never authorize downgrade. A configured but untrusted project entry is blocked by project trust: the
registry omits it from resolution, so it is neither launched nor advertised,
and a route whose trust is revoked fails its next reconnect; `mcp.list` still
returns the entry with `project_trusted: false`.

## JSON-RPC and transport bounds

Every request is a JSON-RPC 2.0 object with a positive integer id scoped to
one peer generation. Responses name the same id and contain exactly one of
`result` or `error`; error is `{code: integer, message: string, data?:
JsonValue}`. Notifications omit id. Unknown responses are protocol errors;
late responses for an explicitly cancelled request are ignored. Duplicate
live response ids, server requests for unadvertised client capabilities, and
unknown required notifications close the peer.

Limits are fixed for v1:

- handshake: 15 seconds;
- catalog page or resource read: 30 seconds;
- tool/prompt/task operation: 10 minutes unless the invoking turn has a
  smaller remaining deadline;
- one frame/result: 4 MiB; stderr retained for diagnosis: 64 KiB;
- pagination: at most 1,000 items and 100 pages per method;
- one multi-round tool call: 32 rounds.

Cancellation sends `notifications/cancelled` with the outstanding request id
when supported, terminates the local wait, and never converts a late result
into success. EOF, child exit, HTTP disconnect, timeout, and malformed bytes
fail every in-flight request in that peer generation. Retry is a caller
decision; mutating `tools/call` and task updates are never automatically
resent without their operation-specific idempotency proof.

The transport retains a cancelled-id tombstone until the corresponding late
response is ignored once or the peer generation closes. Every protocol error
closes that generation. Handshake, catalog, prompt and resource reads restart
the complete operation at most once on a newly initialized transport; a
protocol-invalid response is terminal and is not retried.

### stdio

The supervisor launches exactly the configured argv without a
shell. stdin/stdout are UTF-8 JSONL, one complete JSON-RPC object plus LF per
frame. stdout contains protocol only; stderr is bounded diagnostic output.
The child inherits only the frozen cwd, minimal platform environment, and
resolved named entries. EOF closes the generation. The supervisor owns every
MCP peer generation and exposes its frozen catalog to workers through
`supervisor_control`; no MCP child is a worker child. This single ownership
rule covers multiply-openable, singleton, plugin-native, stdio, and HTTP
servers without adding a second recovery authority.

### Streamable HTTP

The supervisor owns HTTP peers. Requests are POST with `Content-Type:
application/json` and `Accept: application/json, text/event-stream`. A bearer,
OAuth token, or configured header is injected only at send time. Redirects to
a different origin fail closed. The optional MCP session id is opaque and
bound to one authorization identity; reconnect never reuses it across a
credential-generation change. SSE event data is decoded as the same JSON-RPC
response/notification union and obeys the same size, id, and cancellation
rules.

An HTTP authorization provider returns an authorization identity and ephemeral
bearer for each send. The transport keeps neither bearer nor resolved header
values in a debug representation. An identity change discards the prior MCP
session id before the request is emitted. An SSE request completes on its first
complete matching response event; it does not wait for stream EOF.

## Handshake and capabilities

Legacy peers perform, in order:

1. `initialize` with the newest supported legacy version, client identity,
   and only the client capabilities whose handlers are installed;
2. validate a mutually supported response version and closed capability map;
3. send `notifications/initialized`;
4. only then issue catalog or invocation requests.

Modern peers first issue `server/discover`, select the newest mutual version,
and attach the selected protocol metadata to every request. An unsupported
version or required capability is terminal for that configuration generation;
the Kernel does not advertise the server.

The server capability map is authoritative. The Kernel calls or exposes only
advertised families: tools, prompts, resources, tasks, subscriptions, and
logging. Roots and sampling callbacks are advertised only when a bounded host
handler exists. A successful empty catalog is distinct from an unsupported
capability.

`serverInfo` is the closed standard MCP `Implementation` shape:
`{name,version,title?,icons?,description?,websiteUrl?}`. Each icon is the closed
shape `{src,mimeType?,sizes?,theme?}`; `src` is an absolute `data`, `http`, or
`https` URI, sizes are positive `<width>x<height>` values or `any`, and theme is
`light | dark`. Unknown fields, malformed URLs, sizes, MIME types, and themes
fail the peer generation. At most 16 bounded icons are retained. This metadata
is diagnostic/presentation state only: it cannot change projected tool names,
schemas, effects, approval classes, capability grants, or executable identity.

## Catalogs, names, and dynamic launch binding

Catalog methods are `tools/list`, `prompts/list`, and `resources/list` with
opaque cursor pagination. Repeated cursors, item/name duplicates, invalid
schemas, or changing server identity within one pagination pass fail the
complete source. `notifications/*/list_changed` invalidates the corresponding
cache generation; it does not mutate an already-running worker's frozen
launch binding.

The peer exposes a monotonic catalog generation. A supervisor cache may reuse
a binding only while that generation remains unchanged; an ignored
`list_changed` notification is a contract violation.

Workspace preparation isolates credential-binding, peer-preparation and
catalog-projection failure per server. A failed server contributes no tools or
routes and projects `readiness:{state:"refresh-failed",code}` with `code` one
of `credential-unavailable`, `refresh-failed` (the process did not start, the
handshake or `tools/list` failed or timed out) or `projection-failed` (a tool's
schema or external-effect contract was refused by the dynamic launch
validator); other servers remain usable, and the session receives one
`mcp_server` notice per skipped server naming the server, the tool and the
reason. Launch fails only when the effective workspace `allowed_tools` names
the failed server's `mcp__<server>__` namespace. A degraded binding is retried
on the next preparation demand rather than cached as ready.

MCP tool names project as `mcp__<server>__<tool>`. Both components use the
same reversible escaping locked by fixtures: ASCII letters, digits and `_`
are retained; every other UTF-8 byte becomes `_hh` lowercase hex. A projected
name colliding with a fixed tool, another dynamic source, or another projected
name rejects the whole worker launch.

Each tool becomes a `DynamicTool` with `source:{kind:"mcp",id:<server>}`,
executed through supervisor control (every MCP peer is supervisor-owned; no
MCP child is a worker child). `inputSchema` is the provider-facing
parameters value and its digest uses the existing launch-binding rule.
Tool descriptions preserve text bytes, including LF, CR and TAB. The dynamic
launch validator requires nonblank text of at most 4096 bytes and rejects other
ASCII controls; description text is not validated as an identifier. (Raised
from 1024 on 2026-09-05: Cloudflare's `search` tool ships a 1760-byte
description; a provider with a tighter function-description limit rejects the
request, the catalog is not the place to truncate.)
The standard optional `_meta` value is retained as bounded I-JSON catalog
metadata but cannot rename a tool, alter its schema/effect, or grant execution.
`destructiveHint:true` maps to the generic `destructive` effect and the engine's
destructive approval class. Otherwise `readOnlyHint:true` maps to `read_only`;
unknown annotations map to the stricter configured approval class. An annotation may tighten but
never relax configured policy. Deferred tools remain absent from the resident
catalog and become discoverable through the existing causal `tool_search`
offer path.

An effectful MCP tool may opt into the closed external-effect contract with
tool metadata
`{"io.tekes/externalEffect":{"version":1,"reconcileTool":<name>}}`.
The named tool must exist in the same catalog, be explicitly read-only, and
require one string `idempotencyKey`. Before the mutating `tools/call`, Kernel
durably binds the invocation and adds the host-owned stable key at request
metadata `_meta["io.tekes/idempotencyKey"]`; the model cannot supply or
override it. The server promises to unique/deduplicate the business operation
by that key and to return the original result on a duplicate call.

Catalog cache keys are `(server reference, config digest, protocol version,
server identity, authorization identity, catalog family)`. Private or absent
cache scope never crosses authorization identity. Cache bytes are rebuildable
and cannot authorize execution after config/grant/credential generation
changes.

### Schema-directed HTTP parameter headers

At tool catalog load, Kernel compiles `x-mcp-header` annotations reachable
through object `properties`. Header names must be nonempty HTTP tokens and
case-insensitively unique. Only string, integer and boolean properties are
supported; annotations in conditional, array or definition schemas fail closed.
Modern HTTP tool requests derive `Mcp-Param-<name>` from the same original
arguments sent in the body, including resumed calls. Missing/null properties
omit the header; wrong types and integers outside the exact I-JSON range reject
the request. Non-ASCII, control, surrounding whitespace, empty and encoded-looking
strings use the protocol base64 header representation. Transport configuration
cannot override these reserved headers. Scoped HTTP requests retain their catalog
projection; transports without HTTP header support reject such catalogs.

### Official task shapes

The released official SDKs implement SEP-1686 as shipped in protocol
2025-11-25: a task carries `ttl` and `pollInterval` (milliseconds), `tasks/get`
returns status only, and the payload of a `completed` task is fetched with
`tasks/result`. The Kernel task vocabulary carries `ttlMs`, `pollIntervalMs`
and the inline `result`. The peer adopts the official keys into the Kernel
keys on every task object it returns (Kernel keys win when both are present)
and completes a `completed` poll that lacks an inline `result` with exactly one
`tasks/result` for the same identity before validation. Supervisor and worker
therefore see one vocabulary regardless of the server's SDK. A peer that
already speaks the Kernel shape is untouched.

### Task extension capability

Kernel preserves advertised `extensions` in capability snapshots. Task methods
are enabled by either the legacy top-level `tasks` object or an object-valued
`extensions["io.modelcontextprotocol/tasks"]`. An unrelated extension or a
non-object task extension does not enable task execution. Serialization retains
the original advertisement location rather than inventing a top-level tasks
capability from the extension.

### Subscription capability details

Capability parsing retains `tools.listChanged`, `prompts.listChanged`,
`resources.listChanged` and `resources.subscribe` as explicit boolean flags.
Absent or false flags do not enable subscriptions, and non-boolean flags are
protocol errors. Presence of a tools/resources capability alone never implies
support for its change notifications. These flags survive serialization;
explicit false and absent both serialize as disabled/omitted.

`McpSubscriptionFilter` constructs the requested notification set from those
flags and explicit resource URIs. It accepts changes only after the first
correlated acknowledgement, for the exact numeric subscription ID, and within
the intersection of requested and acknowledged notifications/resources. A
change before acknowledgement cannot activate the filter. This filter alone
does not establish an active transport subscription.

`HttpTransport::listen_subscription` opens a dedicated modern request and feeds
complete SSE notifications through that filter into its sink before the final
response. Cancellation drops the request socket; subscription notifications do
not accumulate in the ordinary method-name queue or bypass the filter through
its observer. The broker starts one catalog subscription after registering a modern peer
with advertised change flags. It runs on the long-lived broker runtime, not
the temporary connection runtime. Accepted changes increment catalog generation;
stream termination also invalidates that generation so the next prepare builds
a fresh peer. Peer close cancels its request generation. Automatic resource-URI
subscriptions are not started by this catalog-only path.

Task continuation validates the polled task before returning its raw result:
the ID must match the requested identity, status must be a defined task state,
and creation/update timestamps must parse as RFC 3339. Poll intervals and TTL
must be nonnegative integers (TTL may be null). Input-required states need a
nonempty inputRequests object; other states cannot carry it. Completed states
require a result object and failed states require an error payload. This does
not by itself turn task state into a durable Kernel hold or promote its result.

### Task results bind a continuation

A `tools/call` whose result carries a `task` object (the tasks extension's
`CreateTaskResult`) with status `working` or `input_required` does not
complete the call. The supervisor validates the task (identity, status,
timestamps), binds an immutable continuation journal entry under the
session's `continuations/` directory (original tool-control request,
host continuation id = SHA-256 over `"tekes-mcp-continuation-v1\0"`,
request id and task id, the pool authority, the initial task state) and
returns a `pending` tool-control result (worker-control). Continuation
steps run `tasks/get`, `tasks/update` (input responses) and `tasks/cancel`
through the bound peer under the same peer-generation and credential-floor
checks as the initial call; `completed` yields the embedded tool result
(`isError=true` is a failure), `failed`/`cancelled` is a failure, and a
`completed` task returned directly by `tools/call` yields its `result`.
Kernel requests task augmentation only for tools whose `tools/list` entry
declares `execution.taskSupport = "required"` (`McpTool::requires_task`): the
`tools/call` then carries `task: {ttl: 600000}` (plus the idempotency `_meta`
for external-effect tools) and the returned `CreateTaskResult` binds the
continuation as above. `optional` tools stay synchronous; `forbidden` or
undeclared tools never carry `task`. A server that returns a task for an
ordinary call is still honored.

Broker task operations expose cooperative cancellation. Before dispatch,
cancellation returns cancelled without a request. Once a task query is in
flight, cancellation closes the peer and returns cancelled. For dispatched
update/cancel mutations, the same loss returns unknown_effect: cancellation
cannot prove the remote mutation did not happen. The broker removes the closed
peer generation before reuse.

## Operations and recovery

Supported operations are:

- `tools/call` with exact name and I-JSON arguments;
- external-effect reconciliation by the declared read-only tool with
  `{idempotencyKey}`, returning exactly one of
  `{status:"confirmed",value}`, `{status:"not_found"}`,
  `{status:"unknown",reason}`, or `{status:"conflicted",reason}`;
- `prompts/get` with string arguments;
- `resources/read` with one URI and text/blob result arms;
- `tasks/get`, `tasks/update`, and `tasks/cancel` when advertised;
- bounded multi-round tool continuation carrying opaque request state,
  input responses, round number, and optional task identity.

The continuation DTO is
`{requestState,inputResponses,round,taskId?}`. `round` is in `1...32`; the
Kernel rejects any larger value before sending it. This is a local DTO, not a
wire envelope. `continue_tool_call` also requires the original arguments; a
multi-round `tools/call` sends top-level `name`, `arguments`, `requestState`
and `inputResponses`. Neither `round` nor a `continuation` wrapper is sent.
A continuation with a server-issued task ID queries `tasks/get` for that exact
ID instead of resending the tool, retaining cancellation and the round bound.
 A task mutation without a
server-issued resumable identity returns `unknown_effect` on loss. With such
an identity, loss causes one reconnect followed by `tasks/get` for that exact
identity; the mutation itself is never resent.

Tool results preserve every MCP content block and structured content before
the normal Kernel result spill and secret scan. MCP errors become typed tool
errors; transport/protocol failures become retryable only when this contract's
recovery table says no side effect may have been accepted. Approval, durable
`tool_call`, hooks, output scan, `tool_result`, stop, and worker recovery
remain `tool-runtime`; MCP is a backend, not a second turn protocol.

The recovery table is closed:

| state at loss | automatic action |
|---|---|
| handshake or catalog request | reconnect once in the same config generation, then fail |
| resource/prompt read before any response | reconnect once, then fail |
| effectful tool call without `idempotency-reconcile` | fail closed before dispatch |
| effectful tool call with an interrupted intent, timeout/transport loss, or cancellation after dispatch | reconnect the same authority, query by idempotency key; execute once only after authoritative `not_found` |
| effectful tool call cancelled before dispatch | return cancelled; no reconciliation is required because the external operation did not begin |
| task mutation without server idempotency proof | return `unknown_effect`; never resend |
| tool call with a server-issued resumable task/request identity | reconnect, query that identity, then continue or return terminal state |
| credential/config/grant generation changed | close old peer; create a fresh generation; never resume old authority |
| stdio child exits | fail in-flight calls, reap, and restart only on next demand or always-on policy |

Cancellation is generation-scoped. The host atomically replaces the active
token before cancelling the previous generation, so reconciliation and later
calls cannot inherit a permanently cancelled token. Once a transport request
future has been polled, cancellation returns `unknown_effect`. For serialized
legacy or stdio calls, the peer generation is closed and evicted even when the best-effort
`notifications/cancelled` write succeeds: stdio cancellation may interrupt a
partial frame. That notification and the close attempt each have the same
short host timeout and cannot delay the result indefinitely. Modern HTTP tool
calls own independent request state: the broker releases its peer lock before
awaiting IO and keeps sibling requests alive after request-local failure.
Explicit peer removal, close or reconnect cancels all requests from that generation.

The supervisor daemon pool key includes server reference, frozen config
digest, authorization identity, protocol mode, and plugin generation. One
creation wins per key; losers await it. Last-client release starts a bounded
idle timer unless `always_on`; v1 fixes that timer to zero seconds, so the
zero-client peer closes before release returns. Stop, disable, uninstall, credential revocation,
or drain closes immediately. A crashed daemon cannot leave a catalog advertised
without a live generation or a cache entry valid for the new generation.

Pool creation is keyed single-flight: the factory runs once and all waiters
receive reference-counted leases for the winning peer. Explicit lease release
waits for the zero-client close; drain closes always-on peers too. A fatal peer
error evicts the generation before the typed error is returned.

## Management capability

The optional endpoint capability is `mcp.v1`. Its typed methods are
`mcp.list`, `mcp.get`, `mcp.save`, `mcp.remove`, `mcp.probe`,
`mcp.oauth.start`, and `mcp.oauth.remove`. All responses are secret-free.
`save`, `remove`, and OAuth mutations require rpcId idempotency and use the
existing endpoint rpc exact-retry carrier. A mutation returns success only after the
canonical registry document and credential reference transaction are durable;
peer refresh may complete afterward and has a separate typed readiness state.

`save` uses field operations `preserve | replace | remove`; omission never
silently deletes a secret. `probe` uses a one-shot isolated peer, returns the
negotiated identity/capabilities and catalog summary, and publishes nothing.
Plugin-owned entries are listed with owner and `editable:false`; standalone
management cannot overwrite or remove them.

These seven methods are additive Client extensions mounted through the
collision-checked production route composite. They do not add to or replace
the Session Endpoint base registrations. Their closed payloads are:

| method | payload |
|---|---|
| `mcp.list` | `{workspaceId:string}` |
| `mcp.get`, `mcp.remove`, `mcp.probe`, `mcp.oauth.remove` | `{reference:McpServerReference}` |
| `mcp.save` | `{server:McpServerConfig,credentialFields:{field:FieldOperation}}` |
| `mcp.oauth.start` | `{reference:McpServerReference,credentialId:string,authorizationUrl:https-url}` |

`FieldOperation` is exactly `{operation:"preserve"}`, `{operation:"replace",
credential_id:string}`, or `{operation:"remove"}`. Credential field keys are
`environment.<name>`, `headers.<name>`, or `oauth`, as appropriate for the
transport. A submitted `server` must not embed credential ids: replace uses
the operation map, omission preserves an existing reference, and remove
deletes only the binding. Literal non-secret configuration remains in the
server DTO. Unknown top-level or nested fields are `bad-request`.

`oauth.start` persists a pending credential reference and returns the supplied
credential-free HTTPS authorization URL. It neither accepts nor stores a token.
`oauth.remove` removes the MCP binding/reference. The platform SecretStore is
the token-byte authority for platform-installed tokens and its contract is
read-only `resolve`; these methods do not create or delete Keychain material.
The Kernel's own token writer is the separately versioned
secret-store §OAuth secret mutation authority, and it covers only records
the Kernel minted: a successful MCP receipt never implies a token write.

An `oauth` binding resolves in two forms at send time. Platform-installed
material (any active record that is not a Kernel-minted grant) is injected
as the bearer as-is, its generation in the authorization identity. A
Kernel-minted refresh grant (secret-store `tekes-oauth-refresh-grant-v1`)
is exchanged: one `grant_type=refresh_token` POST at the grant's token
endpoint (client id, optional client secret and `resource`), the access token
cached per stored generation until it expires, a response carrying a new
refresh token rotated into the store (generation +1) before the token is
used, and the authorization identity `oauth:<credential>:<client_id>` stable
across rotations (the MCP session survives a rotation; the peer's credential
floor is the generation the store holds after the connect). HTTP 400 from the
token endpoint (`invalid_grant`) is the exact failure
`token endpoint returned HTTP 400`: the Kernel publishes the revoked record
(no retry, peer generation closed) and every later connector on that binding
fails as unavailable until a new grant is minted. HTTP 429 with a
`Retry-After` hint (a refresh overlapping a just-completed one on a rotating
server) is retried a bounded number of times after the hint; the exact 400
failure stays sticky on the connector that observed it, so the transport's
single retry reports the same text. Any other token-endpoint status or
transport failure fails the connect without touching the record.
A Kernel-minted grant behind an `oauth` binding requires the mutation
authority to be installed; without it the connect fails closed. Interactive
authorization (registration, consent, code exchange) stays outside the
Kernel; `scripts/run-live-mcp-oauth.py` performs it once for the live gate
(`crates/supervisor/tests/live_oauth.rs`).
Pending is an execution-denying ownership state: an already-active SecretStore
record with the same id does not satisfy it. After the platform OAuth authority
has durably installed the token, it invokes the internal, token-free
`oauth_bind(reference,credentialId)` completion transaction. That transaction
requires the exact pending `(server,"oauth",credentialId)` relation and an
active SecretStore record, then publishes `bound`; it is not an eighth Client
route and cannot be invoked through the management endpoint.

Publication is staged as `intent -> registry temp+sync -> credential
transaction -> atomic registry rename+directory sync -> committed receipt ->
peer generation refresh`. Recovery discovers every intent before serving
management calls and either completes the exact recorded bytes or rolls back
unpublished credential material. No state in an adjudication file or cache is
an authority.

The concrete v1 authority stores `mcp-operation.json`, canonical
`mcp-servers.json`, canonical `mcp-credential-references.json`, and a canonical
`receipts/<rpcId>.json`. The operation contains the internally computed digest
of the closed mutation plus the exact next registry, reference registry,
secret-free result, and phase. Recovery completes those recorded bytes; it
never recomputes a mutation against newer state. Same rpcId plus same computed
digest re-acks the receipt, while a different digest is
`idempotency-conflict`.

`mcp-servers.json` is written reference-sorted with one entry per
`{workspace_id, scope, name}`; a save replaces the entry with that reference,
so repeated saves are idempotent. The file is also edited by hand and by
external writers, and the reader repairs the two deviations they produce
rather than losing the catalog: a missing final newline is tolerated, and a
repeated reference keeps its last entry (the newest write), is logged as
`mcp-registry-duplicate-reference`, and is published deduplicated by the next
mutation. The same server registered under two workspaces is two references,
not a repeat. Every other deviation (non-canonical bytes, an invalid server)
fails the load and names the file; the launch proceeds without the registry's
servers and the session receives a notice.

The reference document projects every credential-valued environment, header,
and OAuth field as `(server,field,credentialId,state)`. Ordinary bindings are
`bound`; `oauth.start` creates `pending`. Replace, preserve, remove, OAuth start,
and OAuth remove therefore update the same recorded registry/reference pair;
neither document contains resolved material.

Plugin rows carry `owner` and `plugin_component:{pluginId,componentId}`. They
are a read-only runtime projection of every enabled, fully granted Slice-12
`mcp-server` component into the current workspace, named by the component id;
they are never copied into the standalone registry. On every binding generation
the supervisor enumerates and resolves Slice 12 again. The resolver's
immutable executable path and per-plugin data directory are used for launch,
and its package digest is the pool's `plugin_generation`. Configured plugin
command/cwd values are never executable authority.

Standalone save/remove/disable commits close the affected cached generation.
Plugin disable/uninstall/grant revoke/package replacement and credential
rotate/revoke are revalidated before every effect, so a stale authority cannot
execute. Slice 13 also exposes an explicit generation-reconcile API that closes
matching peers after an external lifecycle commit. The production plugin-
management owner does not exist until Slice 14F; that owner MUST invoke this API
after its durable plugin mutation commits. Until then, Slice 13 claims the
explicit API and per-effect fail-close boundary, not an already-wired proactive
production hook. The binding cache also records every live peer's catalog
generation and a `list_changed` increment invalidates and closes that cached
generation before reuse.

## Required fixtures and Slice-13 gates

`fixtures/mcp-runtime/` is normative and contains canonical JSON-RPC
transcripts, config/management values, catalog projection, recovery matrices,
and rejection cases. Secrets use sentinel references only.

- Gate 86: protocol handshake, framing, bounds, and capability negotiation.
- Gate 87: tool/resource/prompt/task operation shapes and cancellation.
- Gate 88: stdio child lifecycle and supervisor daemon-pool ownership.
- Gate 89: HTTP/OAuth credential isolation, reconnect, and authorization cache
  partitioning.
- Gate 90: catalog projection, collision, causal `tool_search`, and immutable
  launch binding.
- Gate 91: management publication/idempotency/recovery and plugin ownership.
- Gate 92: crash/reconnect recovery matrix and full predecessor parity audit.

All earlier Slice gates run before these seven in `scripts/ci-slice13.sh`.

HTTP SSE transports may install a synchronous notification-method observer. It runs when a validated notification is decoded, before awaiting the final RPC response; the method remains available to the existing notification drain. The observer must return promptly and does not replace full notification payload delivery or enable concurrent calls on an exclusively borrowed peer.

Modern HTTP callers may use request-scoped execution with explicit2026-07-28 metadata. Response/session state belongs to each request; cancellation drops only that request future. Legacy session state is not copied. Negotiated modern HTTP peers opt into owned requests through McpPeer::start_scoped_tool. The broker releases the peer lock before awaiting these requests; legacy and stdio calls retain serialized dispatch.

The shared client returns `Cancelled` for a token already cancelled before request construction. Once scoped execution begins, cancellation, transport loss, timeout or an invalid response identity returns `UnknownEffect` for tools: socket isolation does not prove absence of an external effect. The broker preserves the modern HTTP generation for independent requests after such request-local failures. Explicit removal and authority rotation still close the generation.
