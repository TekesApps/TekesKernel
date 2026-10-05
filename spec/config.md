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
threads/<session-uuid>/session-settings.json
```

`<workspace-id>` is 1–128 ASCII characters from `[A-Za-z0-9._-]`; it cannot
start with `.`. The workspace file's `id` MUST equal its parent directory
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

`allowed_tools` is interpreted by
[builtin-tools](builtin-tools.md). Config parsing validates its strings
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
  workspace: ResolvedWorkspace,
  providers: Providers,
  settings: Settings,
  session_settings?: SessionSettings,
  revisions: {
    workspace: int, providers: int, settings: int,
    session_settings?: int
  }
}
```

A snapshot published before `config/integrations.json` was removed carries an
`integrations` document and an `integrations` revision; readers ignore both
and never write them.

`ResolvedWorkspace` contains the canonical authored `cwd` list in the same
order as `Workspace.folders`. A session-bound snapshot additionally contains
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
