# Config v1 — authoritative files and spawn snapshots

This contract owns the bytes and behavior of the user-authored configuration
under one storage root. It is an executable contract. Narrative docs may
explain it but cannot redefine it.

## Authority and layout

The authoritative files are:

```text
workspaces/<workspace-id>/workspace.json
config/providers.json
config/settings.json
config/environments.json
threads/<session-uuid>/session-settings.json
```

`<workspace-id>` is 1–128 ASCII characters from `[A-Za-z0-9._-]`; it cannot
start with `.`. Endpoint clients edit `folders` only through
[`workspaceFolders.v1`](client-extensions.md#workspace-administration). The workspace file's `id` MUST equal its parent directory
name. The sibling `state/` and `skills/` directories are
private workspace-scoped data, not additional workspace identity records.
Secrets never occur in these files: `credential_key` values are identifiers
resolved through
[secret-store](secret-store.md), not secret values.

Every stored file is one RFC-8785 canonical JSON object followed by exactly
one LF. Duplicate keys, non-I-JSON numbers, a BOM, noncanonical bytes,
unknown fields, a missing required field, and an unsupported `format` reject
the file. Integers are nonnegative safe integers (`<= 9007199254740991`).

## Closed schemas

Notation: `?` means optional and absent; it never means JSON null. Every
object below has a closed field set.

```text
Workspace = {
  format: 1, revision: int>=1, id: str, name: str,
  folders: [{id:str, path:AbsolutePath}, ...],
  policy?: WorkspacePolicy
}
WorkspacePolicy = {
  writable_roots?: [AbsolutePath, ...],
  toolchain_roots?: [AbsolutePath, ...],
  network?: bool,
  allowed_tools?: [str, ...],
  provider?: str,
  model?: str,
  max_wall_seconds?: int>=1,
  base?: "general" | "blank"
}

Environments = {
  format: 1, revision: int>=1, environments: [Environment, ...]
}
Environment = {
  id: str, network: bool,
  toolchain_roots?: [AbsolutePath, ...],
  max_wall_seconds?: int>=1
}

Providers = {
  format: 1, revision: int>=1, providers: [Provider, ...],
  web_search?: WebSearch
}
Provider = {
  id: str, name?: str, adapter: str, dialect: str,
  endpoint_owner: str, gateway_translation: str, evidence_revision: str,
  endpoint: str, credential_key?: str,
  models: [Model, ...]
}
WebSearch = {
  adapter: "tavily_v1", endpoint: HttpsPublicOrigin, credential_key: str
}
Model = {
  id: str, profile: str, enabled: bool,
  context_window_tokens: int>=2,
  compact_trigger_tokens: int>=1
}

`adapter` is the protocol family. `dialect`, `Model.profile`,
`endpoint_owner`, `gateway_translation`, the exact model `id`, and
`evidence_revision` form the exact runtime identity defined by
`provider-dialect-profiles.md`. A structurally valid but unproved tuple is
configuration data only: it is not runnable or advertisable. Format 1 uses
`gateway_translation:"direct"`; a gateway needs its own proved route evidence.
`Provider.name`, when present, is the user-facing provider name authored by the
model-list item. It is presentation data only: `Provider.id` remains the stable
connection and routing identity. Older configs may omit `name`.

Settings = {
  format: 1, revision: int>=1,
  default_provider?: str, default_model?: str,
  limits?: {max_workers?: int>=1, max_provider_leases?: int>=1}
}
SessionSettings = {
  format: 1, revision: int>=1, provider: str, model: str,
  reasoning_effort?: str
}
```

`max_workers` limits ordinary runnable/active-work admission, not structured
dependency waiters. When a live parent sends `launch_child` for its durable
unpaired `spawn`, that one direct child bypasses ordinary pending admission:
the parent retains its slot but performs no other semantic work while tailing
the child. The rule applies independently at each bounded spawn edge, so a
parent → child → grandchild chain progresses even when
`max_workers == 1`. Unrelated roots and queued workers never receive this
override and cannot take the dependency slot when a child exits; they remain
subject to `max_workers` until the waiting chain unwinds.

Validation beyond shape:

1. `folders` is nonempty. Folder binding ids are unique and stable when a
   directory moves; `path` is only its current absolute machine binding.
   Paths are absolute UTF-8 paths with no NUL; resolution canonicalizes each
   existing directory. Duplicate canonical paths reject. For compatibility,
   a legacy document may contain nonempty `cwd` and omit `folders`; the two
   fields may never coexist. The resolver deterministically exposes legacy
   entries as `folder-0001`, `folder-0002`, ... in authored order. A future
   explicit format migration must preserve those derived ids; ordinary reads
   and policy updates do not silently rewrite the legacy document.
2. `writable_roots` are existing directories, canonicalized and deduplicated;
   every root must be equal to or below one canonical folder path.
3. IDs are nonempty, unique within their list, and contain no ASCII control.
   Model IDs are unique within a provider. Each model's two token limits are
   safe integers and `compact_trigger_tokens < context_window_tokens`; D-39
   compares the adapter's conservative candidate estimate with the former and
   treats the latter as the hard provider window. Neither value is inferred
   from a model name or remote catalog response.
   `adapter`, `dialect`, `endpoint_owner`, `gateway_translation`,
   `evidence_revision`, and Model `profile` are nonempty NFC strings. Together
   with exact Model `id`, they are durable identity, not labels inferred from
   endpoint host, provider response, or display names. Before a configured
   model may be advertised or enter a spawn snapshot, the installed
   `provider-dialect-profiles` registry must accept the complete tuple
   `(adapter,dialect,profile,endpoint_owner,gateway_translation,model.id,
   evidence_revision)`. A missing or mismatched proof fails
   `invalid_reference`; no field is defaulted or inferred.
4. Local tools are MCP stdio servers ([mcp-runtime](mcp-runtime.md));
   the config authority carries no other integration transport.
5. `credential_key` is a nonempty ID.
6. Workspace/default/session provider and model references must name enabled
   entries. Session settings override workspace policy, which overrides global
   defaults. `reasoning_effort`, when present, must be a nonempty effort
   advertised by the selected session-scoped model route.
7. Lists whose order is not semantic (`allowed_tools`, `writable_roots`) are
   sorted and deduplicated in a resolved snapshot. `cwd`, providers, models
   and command argv preserve authored order.
8. `web_search`, when present, is independent of every model `Provider`.
   Format 1 supports only `adapter:"tavily_v1"`. `endpoint` is an HTTPS
   public origin with no path, query, fragment, userinfo, or trailing slash;
   the adapter appends the fixed `/search` path. Literal loopback, private,
   link-local, multicast, unspecified, and localhost names reject. The
   credential id is resolved only for the `web_search` purpose under
   [web-search-provider](web-search-provider.md).

`base` is the session preset base for sessions created in the workspace;
omitted is `general`. It is read once when a session is created and frozen in
that session's preset ([session-endpoint § Session presets](session-endpoint.md#session-presets)),
so changing it affects only later sessions and never respawns a worker.

`allowed_tools` is interpreted by
[builtin-tools](builtin-tools.md). For a session whose preset selects tools,
the preset's selection replaces it in that session's snapshot
([session-endpoint § Session presets](session-endpoint.md#session-presets)); the
instruction policy meets either one. Config parsing validates its strings
but does not synthesize or connect dynamic catalogs. After fixed tools and the
configured external/plugin/MCP catalogs resolve, every allowed name must name
one effective catalog entry or launch fails `invalid_reference`; policy can
remove availability but cannot shadow a fixed name or alter schema, backend,
effect, approval, or sandbox classification.

If a global file is absent, resolution uses this in-memory default and does
not create authority bytes:

```text
Providers = {format:1, revision:0, providers:[]}
Settings  = {format:1, revision:0}
```

Revision 0 is valid only for these synthesized defaults, never on disk.

## Publication and revisions

All config readers take the shared `config/.lock`; all config writers take it
exclusive. The lock file is opened no-follow, locked, then path→inode
revalidated. A mutation supplies `expected_revision`. Under the exclusive
lock, it either fails `stale_revision {expected, actual}` without writing or
publishes exactly `actual + 1`.

A writer of a workspace document that does not run inside an endpoint
management operation (`workspace.policy.set`) additionally holds the endpoint
management lock around its publication. Lock order is management, then
`config/.lock`. A management operation stages a workspace snapshot and publishes
it under the management lock, so an unserialized publication in between would
leave that operation unable to complete.

Publication is same-directory temporary file → checked full write →
`F_FULLFSYNC` on Darwin → atomic replace rename → parent-directory sync.
Success is returned only after the directory sync. A crash exposes either the
complete old canonical file or the complete new canonical file. Temporary
debris is ignored and may be removed by sweep. Writers never edit in place.

Format 1 has no predecessor. Future format migration is an explicit pure
`N -> N+1` transform with fixtures and uses the same expected-revision atomic
publication. A reader never guesses, partially consumes, or silently
downgrades an unknown format.

## Resolved spawn snapshot

While holding the shared config lock, the supervisor reads the workspace,
the two global files, and the target thread's optional session settings,
validates all references, canonicalizes paths, and
constructs:

```text
ConfigSnapshot = {
  format: 1,
  user_agent_dir?: absolute path,
  workspace: ResolvedWorkspace,
  providers: Providers,
  settings: Settings,
  session_settings?: SessionSettings,
  revisions: {
    workspace: int, providers: int, settings: int,
    session_settings?: int, environments?: int
  }
}
```

The launch authority freezes the actual shared instruction directory in
`user_agent_dir`; session settings cannot set it. Every file write guard and
foreground or detached execution sandbox protects that directory, the default
service user's `~/.agents`, Kernel storage and platform credential directories
even beneath a writable home. Older snapshots omit this field and retain the
default protected roots. Missing paths resolve through their nearest existing
ancestor so path aliases do not evade protection. Linux launches that expose a
missing protected directory fail closed until the host prepares it; they do
not silently omit the read-only mount.

A snapshot published before `config/integrations.json` was removed carries an
`integrations` document and an `integrations` revision; readers ignore both
and never write them.

`ResolvedWorkspace` contains the canonical authored `cwd` list in the same
order as `Workspace.folders`, narrowed to a session's recorded folders as
[§Environments](#environments) describes. A session-bound snapshot additionally contains
the paired optional fields `folder_binding` and `selected_cwd`; both are
present or both absent. `folder_binding` MUST name a current stable folder id,
and `selected_cwd` MUST be that folder's canonical path and a member of `cwd`.
It selects the execution/default cwd without moving that path to index zero.
A workspace-wide snapshot omits both fields. A legacy snapshot without a
binding is executable only when `cwd` has exactly one entry; a multi-folder
legacy session is ambiguous and fails closed rather than selecting the first
entry.

`ResolvedWorkspace` always contains `policy` with these defaults: `writable_roots: []`,
`network:false`, `allowed_tools:[]`; optional provider/model/max-wall remain
absent. These defaults apply to an authored document that omits `policy`; a
workspace created through the endpoint's `workspace.create` is published with
an explicit seeded policy ([session-endpoint](session-endpoint.md)). These are semantic defaults under the WorkspacePolicy omission rules:
the policy object is always present, `network:false` is encoded, and an empty
optional set is omitted rather than encoded as an empty array. The embedded
global objects retain their source revisions, including revision 0 defaults.
`network` governs network access exposed to workspace tools. It does not gate
or preflight the configured model-provider transport; provider HTTP,
authentication, and timeout failures are reported by the provider send itself.

Snapshot bytes are RFC-8785 canonical JSON plus one LF. Its digest is the
lowercase SHA-256 hex of those exact bytes. Before launch, the bytes are
published by the D-45 reference protocol as
`assets/sha256-<digest>` in the target thread folder. `genesis.config.digest`
and every `run_start.config_digest` are implicit references to that asset;
asset GC, fork, archive, and redact carrier scans MUST retain/process it.
Because `providers` is embedded verbatim, the digest commits every exact-route
identity field above as well as the selected adapter, endpoint, credential
reference, limits, and enabled state. A proof-registry update never rewrites an
old snapshot or changes its attribution.

The worker receives an already-open read-only snapshot descriptor, verifies
the digest, parses once before taking the line lock, and never reads live
config paths. The digest therefore identifies the exact revisions and
resolved paths used by that run. An edit after snapshot materialization
affects only a later spawn.

A thread folder holding the earlier `session-settings-v1.json` name has it
renamed in place the first time the document is resolved or published; two
populated names fail closed.

`session-settings.json` is published by session-endpoint §Management authority and recovery
`session.selectModel` under the same atomic config publication rules. It moves
with archive/fork folder operations and is read only from the selected thread
folder; no global session-id map is a second authority. Absence means the
workspace/global selection applies and is omitted from the snapshot.

## Session authority and workspace compatibility (0.8)

Session is the primary entity. `workspaceId` remains a compatibility creation
address and inventory label. A new genesis freezes `session_scope.seed` (the
creation policy and ordered folder bindings) and `session_scope.folder_refs`
(binding to logical Folder id). Launch, file tools, project resources and MCP
resolve that session seed, current `config/folders.json` paths, current
Environment, global providers/settings and session settings. They do not read
the mutable workspace document. `ResolvedWorkspace` is retained as a snapshot
wire shape; its name does not confer shared authority.

A path-only `session.create` selects only that path and the local interactive
policy defaults, even when a compatibility workspace already contains it.
An explicit `workspaceId` copies its configuration once. Workspace policy and
folder edits are templates for future creates and cannot widen, tighten or
stop detached sessions. Explicit workspace creates may still opt into their
legacy instruction directory; native path creates use user and Folder
instructions, with a protected per-session host instruction directory.

Old sessions lazily read their checksum-verified original config asset when
available. No startup ledger migration or full thread scan is performed.
Legacy records lacking a usable creation asset retain the workspace adapter;
if that adapter is unavailable, resolution fails closed. Old effective policy
limits remain conservative when their original authored values are unknown.

`config/folders.json` owns logical Folder identity and current paths. Old
workspace/binding pairs are aliases only. Relocation changes the path while
retaining the logical id; trust stays bound to the actual directory identity
and is never transferred. The existing management journal updates a
workspace relocation's catalog overlay before marking its semantic receipt;
catalog-only relocations publish path and exact retry receipt atomically.
Workers hold shared locks for their logical folders for their complete
lifetime, preventing relocation while any of them is running.

Per-session Folder edits are ledger `meta.scope_change` events, effective at
the next launch. `config/.folder-capture.json` is a private recovery intent,
not a second scope authority: the exact checksummed ledger event determines
whether its catalog state is published. Catalog access first settles a pending
intent under the exclusive catalog lock; preparing or rejecting an edit alone
cannot publish a new Folder. Catalog recovery also recognizes an archived
session's ledger. They never edit the compatibility workspace or another
session. See [sessionFolders.v1](client-extensions.md#session-folder-administration).

## Environments

`config/environments.json` names the environments sessions run in. A missing
file is revision 0 with no entries. Environment ids are unique, nonempty and
free of control characters; toolchain roots are absolute and contain no
`:`. The built-in `local` (network on, no toolchain roots, no wall-time cap)
exists unless the file defines its own `local`, which then replaces it.

A session resolves in its genesis scope
([event §Genesis](event.md#genesis-seq-1-only-barrier)): its `environment`
(default `local`) and, when recorded, its `folders`. In that scope:

- the environment is looked up by id; an unknown id fails closed;
- `policy.network` is the session creation seed value and the environment's;
  `policy.max_wall_seconds` is the smaller of the two caps; the environment's
  toolchain roots (canonicalized) are added to `policy.toolchain_roots`. An
  environment never adds writable roots;
- recorded `folders` replace `cwd` with those folders' canonical paths in
  recorded order. Every id MUST name a folder of the session seed (or the legacy adapter), the
  bound folder MUST be one of them, and only writable roots inside them
  remain;
- `ResolvedWorkspace` records `environment` and, when recorded, `folders`,
  and `revisions.environments` records the environments revision.

`session.create` freezes the snapshot it resolves in the scope it records, so
it equals what the first launch resolves. The environment is re-resolved at every launch and meets the frozen
creation seed; it does not depend on later workspace publications.

## Revocation

After a successful config mutation, the supervisor compares affected live
runs' snapshots. Additive/benign changes wait until next spawn. Removing a
writable root, changing `network` true→false, removing an allowed tool,
disabling the selected provider/model, or lowering a numeric cap
is a privilege reduction: the supervisor requests stop and respawn for each
affected worker. The old snapshot remains the attribution record; it is never
rewritten. A failure to signal is surfaced and re-driven by sweep.

## Typed errors

The closed operational classes are `invalid_bytes`, `invalid_schema`,
`invalid_path`, `invalid_reference`, `secret_material`, `stale_revision`,
`unsupported_format`, `io`, and `unstable_source`. Errors identify the file
and JSON field/path but never include credential values or snapshot content.

## Explicit workspace toolchains

`toolchain_roots` names read-only runtime installations used by workspace
process tools. Roots resolve to existing canonical directories, deduplicate
and sort in the frozen snapshot. Colon-containing canonical paths reject.
They are not project folders or filesystem-tool mounts and grant no writes.
The worker adds them to process sandbox read roots. When nonempty, it builds
PATH from each canonical root's `bin`, in sorted root order, followed by
`/usr/bin:/bin:/usr/sbin:/sbin`. It never copies the host PATH or other host
environment values. Absence preserves the existing empty tool environment.
Changing this list requires worker respawn because it changes executable
selection and read authority. Installations must contain the dependencies
needed by their executables; listing a root is not proof a program runs.


## Retired memory tool selectors

Historical `allowed_tools` entries `note` and `recall` are accepted during
launch validation for upgrade compatibility. They are excluded from worker
execution permissions and are absent from the fixed catalog and new workspace
seeds. Existing config bytes and snapshots are not rewritten. Remove these
entries through the revisioned workspace policy API when editing the policy;
see [retired Kernel memory](../docs/history/retired-kernel-memory.md).

### Environment administration

The optional [environments.v1](client-extensions.md#environments) client group
reads effective definitions and upserts one authored definition with document
revision CAS. `session.create {environment}` chooses a reference (default
`local`); the creation intent fixes that reference, and each subsequent launch
resolves the current definition. Save changes the next launch only, without
changing folder trust, writable roots or running descriptors.

## Folder execution authority (0.8)

See [Folder trust](../docs/folder-trust.md) for the protected format-1 authority,
local consent protocol and `folderTrust.v1` closed DTOs. Permission mode and
`project_trusted` do not grant project execution. Instruction snapshots may add
`folder_authority:{storage_root,folders}`; `folders` maps project indices to
captured canonical identity/trust records. Legacy snapshots omit it and still
decode, but cannot enable project hooks or skills. Discovery excludes untrusted
executable sources before parsing; project AGENTS/settings/rules/commands remain
readable. Runtime admission compares the captured identity and revision again,
including continuation and resource use. Shared effect barriers report pending
revocation separately from completed peer cleanup.
