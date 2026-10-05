# Client extensions v1

Status: executable contract for Slice 14F.

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
workspace.create workspace.rename workspace.relocate
workspace.archiveSession workspace.unarchiveSession
session.create session.prompt session.updateQueue session.cancel
session.rename session.fork session.attachment session.models session.selectModel
remote.mux
```

The executable extension catalog is `fixtures/client-extensions/catalog.canonical.json`.
Capability ids and versions are Client metadata, not fields added to
`host.describe`. Release qualification compares the Client driver's declared
catalog with the server registry in both directions and rejects a collision
with the base set or another extension. The v1 capability groups are atomic:
partial method registration or advertisement is a protocol defect. Extensions
use the existing transport request/result/error envelope and do not define a
new event stream, mux frame, endpoint generation, or Mirror partition.

Every extension supports `bad-request`, `unsupported-capability`, and
`internal`. A mutation also supports `idempotency-conflict`. The tables below
add only the named semantic errors. Unknown request/result fields, unknown
enum arms, and an error outside the method's closed set fail closed.

## Initial identity presets

`initialPresets.v1` registers the read-only `session.initialPresets {}` method.
Its result is `{presets:[{id,title,configurationText}],defaultID:null}`. The two
rows are Coding and General, with the complete raw contents of
`crates/tools/prompts/coding.md` and `general.md`. Catalog reads create no session
or configuration bytes. `{model}` is resolved only in the worker's instructions.
An explicit choice is applied in `session.create {identityProfile}`; omission
retains automatic first-input selection. The resolved `identityProfile` is
returned in inventory and stays fixed for the session. See Session Endpoint's
initial-preset section for persistence and legacy behavior.

## Resource and tool catalogs

`resources.v1` retains the existing Slice-11 methods and adds unified reads:

| method | class | request | result authority |
|---|---|---|---|
| `skills/list` | read | `{}` | `skill-package` `SkillsListResult` |
| `commands/list` | read | `{}` | `command-catalog` `CommandsListResult` |
| `commands/run` | mutation | `command-catalog` `CommandRunRequest` | its durable keyed-input result |
| `resources/list` | read | `{workspaceId,source:"skill"|"mcp",after?,limit}` | `{format:1,items:[ResourceSummary],next?}` |
| `resources/read` | read | `{workspaceId,reference,uri}` | `{format:1,content:ResourceContent}` |

`limit` is `1...100`; absence means 100. `ResourceSummary` is
`{reference,uri,name,mimeType?,description?}` and the closed reference union is:

```text
ResourceReference =
  {kind:"skill",workspaceId,name,contentDigest}
| {kind:"mcp",server:McpServerReference,bindingDigest}
```

The skill digest is `skill-package` `content_digest`. The MCP binding digest
is `"sha256-" + lowercase_hex(SHA-256(RFC8785({server,configDigest,
protocolVersion,serverIdentity,authorizationIdentity,catalogGeneration})))`;
the preimage contains no credential bytes. A restart or catalog-generation
change makes the old reference stale rather than reusing it for different
content. `ResourceContent` is exactly one of `{text,mimeType?}` or
`{blobBase64,mimeType}` and is capped at 4 MiB decoded. Listing and reads use
one frozen skill snapshot or one live MCP peer generation respectively;
pagination never mixes generations. Closed additional errors are
`resource-not-found`, `resource-stale`, and `resource-too-large`.
For a skill reference, `uri` is its exact safe frozen relative resource path
(for example `references/api.md`); for MCP it is the server-advertised URI.
Neither arm rewrites a URI into a Kernel-private scheme.

`tools.v1` is read-only:

| method | request | result |
|---|---|---|
| `tools/list` | `{workspaceId,sessionId?}` | `{format:1,catalogDigest,tools:[ToolSummary]}` |
| `tools/resolve` | `{workspaceId,sessionId?,name}` | `{format:1,catalogDigest,tool:ToolSummary}` |

`ToolSummary` is `{name,description,arguments,availability,backend,effect,
source}`. `source` is exactly `{kind:"builtin"}` for a fixed row or
`{kind:"plugin"|"mcp",id}` from the immutable `DynamicToolSource`.
Fixed rows use `builtin-tools`; dynamic rows use the immutable launch-binding
projection. The endpoint never resolves or exposes credentials
and never claims that a catalog row is executable after its generation is
revoked. `tools/resolve` returns `tool-not-found` rather than a successful
empty result. Neither method is a model tool invocation path.
The fixed `availability` string is the exact `builtin-tools` enum value
(for example `read` is `default`); the endpoint defines no `always` alias.

## Plugin and MCP management

`plugins.v1` binds only the generic Slice-12 `PluginStore` authority:

```text
plugin/list          read      {}
plugin/get           read      {pluginId}
plugin/inspect       read      {archivePath}
plugin/install       mutation  {archivePath,packageDigest,enable,grants,
                                 allowDowngrade,allowSameVersionReplacement}
plugin/setEnabled    mutation  {pluginId,enabled}
plugin/setGrants     mutation  {pluginId,grants}
plugin/remove        mutation  {pluginId}
plugin/components    read      {}
```

Results are the canonical receipt/projection types from
`plugin-package`, projected into these closed secret-free public values:

```text
PluginView = {pluginId,version,displayName,packageDigest,
              integrity:"local-unverified"|"publisher-verified",
              requestedCapabilities:[CapabilityRequest],
              grantedCapabilities:[str],enabled}
PluginComponentView = {ownerPluginId,ownerVersion,componentId,componentKind,
                       pluginGeneration,grantedCapabilities,launchable}
Readiness = {state:"ready"} | {state:"refresh-failed",code}
```

`plugin/list` returns `{format:1,plugins:[PluginView]}` in plugin-id order;
`get`, `install`, `setEnabled`, and `setGrants` return
`{format:1,plugin:PluginView,readiness?}`; `remove` returns
`{format:1,removed,readiness}`; `components` returns
`{format:1,components:[PluginComponentView]}` in the Slice-12 projection
order. `inspect` returns `{format:1,manifest,packageDigest,integrity,
requestedCapabilities}` without publishing the package. Filesystem source and
package paths, data paths, native signing requirement strings, and publisher keys do
not enter Client results. The host revalidates those facts internally.
`archivePath` is an absolute local path and this capability is advertised only
by the built-in local `.tekes` endpoint. Every successful mutation invokes
`mcp-runtime` generation reconcile before its response; if peer close or
reconcile fails, durable plugin success is returned with
`readiness:{state:"refresh-failed",code}` rather than rolled back or reported
as an install failure. Execution revalidates the new generation before every
effect. Closed additional errors are `plugin-not-found`, `plugin-invalid`,
`plugin-untrusted`, `plugin-grants`, `plugin-collision`,
`plugin-version-conflict`, and `plugin-busy`.

`plugin/inspect` safely stages the source and returns `packageDigest`, the exact
canonical expanded-package tree digest computed by
`plugin-package`/`PluginStore`; it is not a hash of the container/archive byte
stream. The subsequent install request supplies that digest and rejects a source
whose newly staged tree no longer matches it. Before install mutates
`PluginStore`, the endpoint copies, safely expands, and verifies that exact tree
into an rpcId-owned staging object, then durably prepares a plugin operation
record keyed by `(rpcId, canonical request digest)` and naming the staged digest.
The same journal records the exact PluginView/readiness result before endpoint
handoff. Recovery never re-reads `archivePath`: it completes from the staged
object or re-acks the committed result. Same-rpc different bytes conflict;
remove/set mutations likewise require their journal's exact committed result,
and absence/current state alone is never proof that this request committed.

There is no Computer-Use method, DTO, dispatcher, or special component arm.
Marketplace fetching, distribution-policy administration, and direct
native-helper TCC controls are permanently absent in v1. A generic plugin's
native MCP executable owns its platform permission prompts; Slice 14A later
qualifies that behavior through ordinary install/MCP discovery/call and may
repair only generic lifecycle code.

`mcp.v1` is exactly the seven methods and DTOs in `mcp-runtime`:
`mcp.list`, `mcp.get`, `mcp.save`, `mcp.remove`, `mcp.probe`,
`mcp.oauth.start`, and `mcp.oauth.remove`. This contract neither aliases their
older slash spellings nor adds an eighth token-writing method.

## Schedule and search

`schedule.v1` binds `schedule` without changing its durable definition:

| method | class | request | result |
|---|---|---|---|
| `schedule.list` | read | `{workspaceId?}` | `{format:1,schedules:[ScheduleView]}` |
| `schedule.save` | mutation | `{definition}` | `{format:1,schedule:ScheduleView}` |
| `schedule.delete` | mutation | `{taskId}` | `{format:1,deleted}` |
| `schedule.runNow` | mutation | `{taskId}` | `{format:1,claim:LaunchClaim}` |

The endpoint origin tuple is `(principal,rpcId)` and is the internal
`origin_tuple`; no Client field supplies it. Before `save`, the handler proves
the workspace exists. The format-1 public profile accepts exactly
`permission_mode:"inherit"` and an absent `model_id`: the claim launcher then
uses the ordinary effective workspace/global policy and model selection when
it creates the session. Any other permission value returns `schedule-policy`;
any present `model_id` returns `schedule-model` before the schedule log
changes. This is deliberate fail-closed behavior: `schedule` stores no
provider id, so applying a bare model id after the effective provider changes
would be ambiguous. A later public profile may add an atomic provider+model
binding only by versioning the durable definition and this DTO. Ignoring
either stored field is a protocol defect. The remaining error mapping is
one-to-one from `schedule`.

`threadSearch.v1` has one read method, `thread.search`, whose request is the
exact `thread-search` internal request with camel-case field names:
`{workspaceId,query,limit,visibility,after?}`. The result is
`{format:1,results,nextCursor?,reachedEnd,catalogDigest}`; internal index
diagnostics are omitted. Every named internal error maps to the same
kebab-case endpoint code. No transcript/body search is implied.

## Provider and workspace administration

`providerAdmin.v1` exists only when `provider-dialect-profiles` proof
loading and the config/secret-reference writers are installed. Its methods
are:

```text
providers.list                 read
providers.verify               read
providers.connections          read
providers.connection.save      mutation
providers.connection.delete    mutation
providers.profiles             read
providers.profile.save         mutation
providers.profile.delete       mutation
providers.profile.default      mutation
```

List requests are `{}`; verify is `{connectionId,exactSku}`. The secret-free
public types map one-to-one to `config`:

```text
Connection = {id,protocolFamily,dialectId,endpointOwner,gatewayTranslation,
              evidenceRevision,endpoint,credentialId?}
ModelProfile = {connectionId,modelProfileId,exactSku,enabled,
                contextWindowTokens,compactTriggerTokens}
RouteTarget = {protocolFamily,dialectId,modelProfileId,endpointOwner,
               gatewayTranslation,exactSku,evidenceRevision}
ProviderReadiness = {state:"ready"}
                  | {state:"unavailable",reason:"dialect-unproved"|
                     "credential-unavailable"|"route-mismatch"}
ConnectionView = Connection plus {readiness:ProviderReadiness}
ModelProfileView = ModelProfile plus {readiness:ProviderReadiness}
```

The mapping is byte-authoritative rather than inferred: `protocolFamily`,
`dialectId`, `endpointOwner`, `gatewayTranslation`, and `evidenceRevision` are
the camel-case projection of Provider `adapter`, `dialect`, `endpoint_owner`,
`gateway_translation`, and `evidence_revision` in `config`.
`modelProfileId` and `exactSku` are Model `profile` and exact Model `id`;
`connectionId` is the containing Provider `id`. A read returns those stored fields even if the currently
installed proof is missing, but marks the row unready; it never reconstructs
them from endpoint/model names.

Connection readiness is independent of its model array. It is `ready` when
the credential reference resolves and at least one installed advertised proof
shares the connection's exact provider-level fields and normalized endpoint:
`(protocolFamily,dialectId,endpointOwner,gatewayTranslation,evidenceRevision,
endpoint)`. Therefore a connection with zero models can be ready, and adding
multiple model rows does not change its connection readiness. Profile
readiness additionally requires the one exact `RouteTarget` for that row.
`ModelProfile.enabled` is orthogonal: disabling a proved profile makes it
unadvertisable but does not make its proof/credential readiness false.

`providers.profiles` is the read authority for both revision domains. It
returns `{format:1,providersRevision,settingsRevision,profiles,default?}`;
`default`, when present, is `{connectionId,exactSku}` and names the current
global default. Its absence means no default is configured. A client MUST use
the returned `settingsRevision` for `providers.profile.default`; it never
guesses a revision.

Connection save/delete are `{expectedProvidersRevision,connection}` and
`{expectedProvidersRevision,connectionId}`. Profile save/delete are
`{expectedProvidersRevision,profile}` and
`{expectedProvidersRevision,connectionId,exactSku}`; they atomically rewrite
the models inside the named Provider. Creating a connection writes an empty
`models` array. Updating one preserves that array only when every row still
matches an installed proof under the submitted route; otherwise it returns
`route-mismatch` without publishing. Connection delete requires an empty model
array and no workspace/default/session reference; profile delete likewise
requires no such reference to that exact provider/model pair. The respective
failures are `provider-in-use` and `profile-in-use`; callers remove references
and profiles explicitly rather than receiving a cascading delete. Profile save
requires the containing connection and one exact matching proof. Default is
`{expectedSettingsRevision,connectionId,exactSku}` and updates `config`
Settings only after resolving that exact provider/model pair. Results name the
changed `providersRevision` or `settingsRevision`; `providers.list` returns
the installed proof-registry entries and has no config revision, while
connections/profiles return `ConnectionView`/`ModelProfileView` rows plus
`providersRevision`. Save results also return the corresponding View after
publication and readiness re-evaluation. A connection contains only
a credential reference, never secret material. All durable config/reference
scans and the publication occur under `config`'s single exclusive lock.
Proof-registry and credential readiness are ephemeral observations outside
that lock and are re-evaluated after publication; neither can authorize or
change the durable bytes.
Verify is a local, non-sending readiness check. It resolves the exact configured
connection/profile tuple, requires an exact proof-registry match, validates the
route-evidence revision, and proves that the referenced credential exists and
is usable for the configured scope. The exact successful `target` is the
`RouteTarget` above and must equal the stored fields. It never opens a provider connection,
starts a billable request, or mutates proof/config/credential state. Its result
is `{format:1,verified:true,target,credentialReady:true}`. Optional exact-route
live smoke is a separately authorized release/UAT operation outside this
endpoint contract; it is never implemented by `providers.verify`. The method
does not upgrade an unproved tuple. `stale-revision`, `dialect-unproved`,
`route-mismatch`, `credential-unavailable`, `provider-in-use`, and
`profile-in-use` are closed errors. Until these
checks are real, the capability and every method above remain absent; broad
protocol-family fixtures are not readiness.
Each list row is `{proofId,target}` where `proofId` is the exact
`provider-dialect-profiles` oracle `proof_id`; it is never synthesized by
the endpoint.

Every provider/settings mutation and `workspace.policy.set` durably prepares,
before config publication, an operation record keyed by `(rpcId, canonical
request digest)` containing the authority path, expected/next revision, exact
desired canonical bytes and digest, and exact result. Startup and every later
mutation first finish any prepared record from those bytes, perform or
idempotently reperform credential/policy lifecycle effects, then commit the
record; they do not wait for the original caller to retry. A committed retry
re-acks its recorded result without consulting mutable current state. A later
unrelated config revision is never inferred to prove the old request.

`workspacePolicy.v1` consists of:

```text
workspace.policy.get  read      {workspaceId}
workspace.policy.set  mutation  {workspaceId,expectedRevision,policy}
```

The policy is exactly `config` `WorkspacePolicy`. Set replaces the complete
workspace-local policy. Relative to the previous workspace revision it may
tighten or relax that local policy, including enabling network or tools that
were previously absent, but it can never exceed immutable/deployment-global
safety limits, installed catalog availability, plugin grants, credential
scope, TCC consent, or operating-system permission. Thus the default
`network:false` and empty tool set are not monotonic ceilings. The handler
cannot grant an unavailable tool, credential, MCP server, plugin component,
filesystem root, or OS permission. The mutation uses `config`
locking/publication and triggers its privilege-reduction stop/respawn rule
only when the replacement actually reduces a live run's effective authority.
Errors are `workspace-not-found`,
`stale-revision`, `policy-invalid`, and `policy-escalation`.

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

## Permanent dispositions

`fixtures/client-extensions/dispositions.canonical.json` is the exhaustive
route-disposition authority for the pinned TekesAppServer/TekesRuntime
baseline. The following behavior classes have no Kernel v1 endpoint method:

- legacy mutable agent preset/type and thread-agent inventory: retired; instructions and
  explicit child topology remain the supported mechanisms;
- sidechat: replaced by `session.fork {ephemeral:true}` plus ordinary keyed
  prompt and `session.discard` (all base routes);
- message feedback: Client-owned local product metadata, not AS truth;
- remote Git UI and Git/GitHub RPCs: retired for the built-in local Kernel;
  Client-local Git UI and model `shell` are the replacements;
- interactive debug queries: replaced by the bounded redacted Slice-10
  support bundle and readiness diagnostics;
- marketplace/distribution policy and direct TCC actions: retired from the
  Kernel endpoint as described above;
- legacy conversation freeze/read/message, events read/notify,
  inventory/change aliases, offload/restore, context query, direct shell,
  response-retry, goal mutations, subagent mutations, and compatibility
  aliases: replaced by the base Session Endpoint, always-file folders,
  ordinary tools, fork/history, and recovery contracts named in the fixture.

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
- Gate 116: cross-capability lifecycle — plugin mutation reconciles MCP,
  schedule validates/applies policy+profile, archived usage/search behavior is
  exact, and all revoked generations fail closed.
