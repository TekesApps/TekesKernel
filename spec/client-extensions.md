# Client extensions v1

> 0.9.0 execution and responsibility changes are defined in
> [Session execution](session-execution.md). Its Session creation and scope rules
> replace the management-backed rules below; 0.8/0.7 compatibility is not a goal.


Status: executable 0.9.0 contract; no 0.8/0.7 compatibility.

This contract closes the optional product-management surface above
[Session Endpoint](session-endpoint.md#routes-and-streams). It does not modify that
contract, its semantic stream frames, or its frozen base registrations. An
extension is callable only when the `.tekes` Client driver declares the exact
capability below and the Kernel production registry owns every method in that
capability. Either side missing a method makes the whole capability
unavailable. Returning an empty successful value for an unavailable authority
is forbidden.

All request and result values are I-JSON objects with closed field sets.
Optional fields are absent, never `null`, except where a referenced contract
requires an explicit nullable field. Identifiers are NFC, nonempty UTF-8 and
at most 512 bytes unless the referenced authority is stricter. Mutation
methods use the ordinary endpoint request `rpcId`; the extension payload never
duplicates it. The supervisor is the sole durable writer and returns success
only after the referenced authority's barrier/receipt. Reads never create
authority bytes merely to answer a query.

`commands/run` retains the imported `command-catalog` keyed-input `key`.
That key is the durable ledger origin/dedup identity; the outer `rpcId` remains
the endpoint operation-journal/handoff identity. It is the sole v1 mutation
whose referenced authority requires both identities, and the two MUST NOT be
substituted for one another.

## Frozen base and negotiation

The base set remains exactly:

```text
session.archive session.unarchive
session.create session.prompt session.updateQueue session.cancel
session.rename session.fork session.discard session.attachment session.models
models.list session.selectModel remote.mux
```

The executable extension catalog is `fixtures/client-extensions/catalog.canonical.json`:
a 14-method base (including mux) and 14 capabilities with 37 extension methods. The retained native-client capabilities are
specified in Session Endpoint and listed under
[Native-client capabilities](#native-client-capabilities).
The Session Endpoint protocol version is 4. Capability ids and versions are Client metadata, not fields added to
`host.describe`. Release qualification compares the Client driver's declared
catalog with the server registry in both directions and rejects a collision
with the base set or another extension. The v1 capability groups are atomic:
partial method registration or advertisement is a protocol defect. Extensions
use the existing transport request/result/error envelope and do not define a
new event stream, mux frame, endpoint generation, or Mirror partition.
The `tekesWorkspace.*` routes of the
[workspace service](../docs/workspace-service-wse.md) are a separate extension
surface outside this catalog.

Every extension supports `bad-request`, `unsupported-capability`, and
`internal`. A mutation also supports `idempotency-conflict`. The tables below
add only the named semantic errors. Unknown request/result fields, unknown
enum arms, and an error outside the method's closed set fail closed.

## Initial identity presets

`initialPresets.v1` registers the read-only `session.initialPresets {}` method.
Its result is `{presets:[{id,title,configurationText}],defaultID:null}`. The two
rows are Coding and General; `configurationText` is the profile text
(identity line, general rules, then the profile's own rules, as composed by
`IdentityProfile::profile_text` from `crates/tools/prompts/`; see
[Built-in Tools § Model-facing tool guidance](builtin-tools.md#model-facing-tool-guidance)).
Catalog reads create no session or configuration bytes. `{model}` is resolved
only in the worker's instructions.
An explicit choice is applied in `session.create {identityProfile}`; omission
retains automatic first-input selection. The resolved `identityProfile` is
returned in inventory and stays fixed for the session. See
[Session Endpoint § Session presets](session-endpoint.md#session-presets) for
persistence and legacy behavior.

## Session presets

`sessionPresets.v1` registers two read-only methods; neither creates a
session or writes configuration. The preset model, limits and render order are
[Session Endpoint § Session presets](session-endpoint.md#session-presets).

| method | request | result |
|---|---|---|
| `session.presets.describe` | `{}` | `{format:1, renderVersion:1, bases:[{id:"general",identity,rules},{id:"blank"}], kernelLayers:[{kind:"kernel",id,version,title,identity,duty}], limits:{maxLayers,maxIdBytes,maxVersionBytes,maxIdentityBytes,maxDutyBytes,maxTextBytes,maxTools,maxMcpServers,maxMcpToolsPerServer,maxSkills}, invalidReasons:[str], selectableTools:[str]}` |
| `session.presets.resolve` | `{execution:ExecutionConfig, preset:Preset}` | `{base, layers:[{kind,id,version}], presetText:str, digest:str, tools?:[str], mcp?:[{server,digest,tools}], skills?:[str], defaults?:{provider,model,reasoningEffort?}, mcpReadiness?:[McpReadiness]}` |

`session.presets.resolve {execution,preset}` resolves against explicit execution,
host-owned instructions and selected MCP descriptors. It returns the same frozen
preset digest that creation records. It does not open registries or launch local
processes. Invalid selections are `preset-invalid`; malformed values are
`bad-request`. See [Session execution](session-execution.md).

## Resource and tool catalogs

`resources.v1` retains the existing Slice-11 methods and adds unified reads:

| method | class | request | result authority |
|---|---|---|---|
| `skills/list` | read | `{}` | `skill-package` `SkillsListResult` |
| `commands/list` | read | `{}` | `command-catalog` `CommandsListResult` |
| `commands/run` | mutation | `command-catalog` `CommandRunRequest` | its durable keyed-input result |
| `resources/list` | read | `{sessionId,source:"skill",after?,limit}` | `{format:1,items:[ResourceSummary],next?}` |
| `resources/read` | read | `{sessionId,reference,uri}` | `{format:1,content:ResourceContent}` |

`limit` is `1...100`; absence means 100. `ResourceSummary` is
`{reference,uri,name,mimeType?,description?}` and the closed reference union is:

```text
ResourceReference =
  {kind:"skill",sessionId,name,contentDigest}
```

The skill digest is the frozen `skill-package` content digest. References must
match the requested Session and frozen digest; stale or cross-Session references
fail closed. `ResourceContent` is `{text,mimeType?}` or `{blobBase64,mimeType}`,
capped at 4 MiB decoded. `uri` is a safe frozen relative resource path. Additional
errors are `resource-not-found`, `resource-stale`, and `resource-too-large`.
MCP resource/prompt browsing is not a Kernel endpoint capability in 0.9.0.

`tools.v1` is read-only:

| method | request | result |
|---|---|---|
| `tools/list` | `{sessionId}` | `{format:1,catalogDigest,tools:[ToolSummary]}` |
| `tools/resolve` | `{sessionId,name}` | `{format:1,catalogDigest,tool:ToolSummary}` |

`ToolSummary` is `{name,description,arguments,availability,backend,effect,
source}`. `source` is exactly `{kind:"builtin"}` for a fixed row or
`{kind:"mcp",id}` from the immutable `DynamicToolSource`.
Fixed rows use `builtin-tools`; dynamic rows use the immutable launch-binding
projection. The endpoint never resolves or exposes credentials
and never claims that a catalog row is executable after its generation is
revoked. `tools/resolve` returns `tool-not-found` rather than a successful
empty result. Neither method is a model tool invocation path.
The fixed `availability` string is the exact `builtin-tools` enum value
(for example `read` is `default`); the endpoint defines no `always` alias.

## Session search

`threadSearch.v1` has two read methods. `session.search {query}` returns
`{items:[{sessionId,snippet}],hasMore}`: a host-wide search over visible
conversation text in active and archived sessions, capped at 50 results.
`thread.search` takes the exact `thread-search` internal request with
camel-case field names:
`{sessionId?,query,limit,visibility,after?}`. Omitting `sessionId` searches all visible Sessions; supplying it filters by Session identity. Cursors bind this complete selection and cannot
widen a filtered query to a global query. The result is
`{format:1,results,nextCursor?,reachedEnd,catalogDigest}`; internal index
diagnostics are omitted. Every named internal error maps to the same
kebab-case endpoint code. No transcript/body search is implied.

## Usage projection

`usage.v1` is read-only:

```text
usage.summary           {sessionId}
usage.cacheAttribution  {sessionId}
```

Both scan the schema-valid semantic ledger and read the `usage` carried by
every `output`/`error` that settles an attempt under `event`. The result carries
`{format:1,sessionId,asOfSeq,...}` and integer token totals. Cache attribution
groups by the provider-reported cache fields and exact attempt/profile
identity; absent provider data is `unavailable`, never inferred. Archived
sessions are readable through this management capability because the
authoritative folder remains present. Errors are `session-not-found` and
`source-corrupt`.

`RouteTarget` names one exact provider route:

```text
RouteTarget = {protocolFamily,dialectId,modelProfileId,endpointOwner,
               gatewayTranslation,exactSku,evidenceRevision}
```

`protocolFamily`, `dialectId`, `endpointOwner`, `gatewayTranslation`, and
`evidenceRevision` are the camel-case projection of Provider `adapter`,
`dialect`, `endpoint_owner`, `gateway_translation`, and `evidence_revision` in
`config`; `modelProfileId` and `exactSku` are Model `profile` and exact Model
`id`.

`usage.cacheAttribution` returns `entries:[CacheAttributionEntry]` in canonical
`RouteTarget` order. The closed entry is
`{target:RouteTarget,attempts:int,inputTokens:str,outputTokens:str,
cacheReadTokens:str,cost?:str}`. It groups only reported usage whose
attempt's epoch profile contains that exact target. Decimal strings are summed
without binary floating point; unavailable usage contributes to no group.
`cost` is present only when every member reports the same currency/unit
contract, otherwise the source is `source-corrupt`. `attempts` is an I-JSON
safe integer; token strings are canonical unsigned decimal with no leading
zero except `"0"`.
A reported usage joins an attribution group only when
`input_tokens`, `output_tokens`, and `cache_read` are all present; a missing
token field is unavailable for this projection and the usage is excluded, not
filled with zero. Summary may still count whichever reported totals its own
contract permits. If every included member omits cost, the group omits `cost`;
if every member supplies cost under one identical currency/unit contract, the
group sums it; a present/absent mixture or heterogeneous contract is
`source-corrupt`.

## Native-client capabilities

These retained capabilities are atomic catalog groups like the ones above. Their
requests, results and rules are specified in Session Endpoint; this table
fixes only membership, method class and the closed capability errors from
`errors.canonical.json`.

| capability | methods (class) | additional errors | specified in |
|---|---|---|---|
| `recovery.v1` | `sessions.recover` (mutation) | `session-not-found` | [Built-in resource and recovery extensions](session-endpoint.md#built-in-resource-and-recovery-extensions) |
| `attachments.v1` | `attachments.policy` (read), `session.uploadFile` (mutation), `session.fileAttachment` (read) | `archived`, `attachment-error`, `session-not-found` | [Inline attachment policy](session-endpoint.md#inline-attachment-policy), [File attachments](session-endpoint.md#file-attachments) |
| `approvals.v1` | `approvals.policy` (read), `approvals.mode` (read), `approvals.select` (mutation) | `archived`, `session-not-found` | [Approval policy](session-endpoint.md#approval-policy) |
| `sessionFiles.v1` | `session.files.stat`, `session.files.read`, `session.files.readBytes` (all read) | `file-unavailable` | [Session file reads](session-endpoint.md#session-file-reads) |
| `feedback.v1` | `feedback.list` (read), `feedback.put` (mutation), `feedback.delete` (mutation) | `io`, `version-conflict` | [Message feedback](session-endpoint.md#message-feedback) |
| `settings.v1` | `settings.describe` (read), `settings.mutate` (mutation), `settings.update` (mutation), `settings.document` (read) | `io`, `settings-invalid`, `stale-revision`, `unknown-namespace` | [Instruction settings](session-endpoint.md#instruction-settings) |
| `goals.v1` | `goals.get` (read), `goals.edit`, `goals.clear`, `goals.pause`, `goals.resume` (mutation) | `goal-invalid-transition`, `goal-not-found`, `goal-stale`, `ledger-corrupt`, `session-not-found` | [Goals](session-endpoint.md#goals) |
| `subagents.v1` | `subagents.list` (read) | `ledger-corrupt`, `session-not-found` | [Subagent catalog](session-endpoint.md#subagent-catalog) |

## Permanent dispositions

`fixtures/client-extensions/dispositions.canonical.json` is the exhaustive
route-disposition authority for the pinned TekesAppServer/TekesRuntime
baseline. It disposes the predecessor behavior classes as follows; a retired
class has no Kernel v1 endpoint method:

- legacy agent presets: retired; a session preset is frozen once at
  `session.create` and described by `sessionPresets.v1`, and project rules
  come from `AGENTS.md` scopes;
- sidechat: replaced by `session.fork {ephemeral:true}` plus ordinary keyed
  prompt and `session.discard` (all base routes);
- message feedback: implemented as `feedback.v1` (Kernel-durable metadata
  under `feedback/`, outside the ledger); the legacy `turn/evaluation/set` is
  retired;
- remote Git UI and Git/GitHub RPCs: retired for the built-in local Kernel;
  Client-local Git UI and model `shell` are the replacements;
- marketplace/distribution policy and direct TCC actions: retired from the
  Kernel endpoint as described above;
- safepoint list/restore: retired; safepoint snapshots were removed from the
  Kernel and the user's version control owns file history;
- credential describe/set/unset: retired; secrets never cross extension RPC;
- process-private `initialize`/`initialized`/`shutdown`: retired;
- legacy conversation freeze/read/message, events read/notify,
  inventory changes, offload/restore, context query, and response-retry:
  replaced by the base Session Endpoint, always-file folders, fork/history,
  and recovery contracts named in the fixture;
- legacy goal and subagent routes: goal state is managed by `goals.v1` and
  delegation children are listed by `subagents.v1`; subagent prompt and
  interrupt have no replacement beyond `session.prompt` and `session.cancel`.

A retired method is absent from both Client catalog and server registry and
returns `unsupported-capability` if the Client nevertheless invokes it. A
retirement never returns an empty success placeholder.

## Gates

`fixtures/client-extensions/` is normative.

- Gate 113: base-set immutability, extension catalog byte identity, atomic
  capability negotiation, method-class and collision checks.
- Gate 114: every retained request/result/error DTO is closed and is bound to
  one production authority; mutation retry re-acks and wrong-byte reuse
  conflicts.
- Gate 115: route inventory is complete in both directions and every legacy
  route is base, replaced, implemented, or retired; no TCU special case and no
  fake empty success exists.
- Gate 116: actual production capability assembly, rejection of removed management
  routes without storage side effects, Session lifecycle/recovery, archived
  usage/search behavior and revoked connection generations fail closed.
