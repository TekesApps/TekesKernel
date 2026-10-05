# spec: plugin package v1

This contract defines Slice 12's inert plugin package and lifecycle authority.
It is compatible with the existing `.tekesplugin` envelope and
`tekes-plugin.json` manifest shipped by TekesAppServer. It does not define an
MCP transport, start a helper, register a dynamic tool, or grant a macOS TCC
permission. Slice 13 owns the generic MCP carrier; after Slice 14F closes the
Client surface, Slice 14A separately qualifies the existing Computer Use
executable as an ordinary user-configured stdio MCP server. Computer Use is not
a plugin-package conformance input.

## Package bytes

A development package is a directory whose root contains
`tekes-plugin.json`. A distribution package is one JSON file ending in
`.tekesplugin`:

```text
Archive = {archiveVersion:1,entries:[Entry,...]}
Entry = {path:relative,kind:"directory"}
      | {path:relative,kind:"file",executable:bool,data:base64}
```

The envelope limits are inherited unchanged from the existing implementation:
384 MiB encoded, 256 MiB expanded, 128 MiB per file and 4096 entries. Paths are
nonempty normalized `/`-separated relative paths. Empty components, `.`, `..`,
absolute paths, backslashes, NUL, duplicates, links, devices and path-kind
collisions are rejected before publication. An archive source is opened once
with `O_NOFOLLOW`, classified with `fstat`, and read through that same bounded
descriptor. Expansion walks and creates every destination component relative
to the held staging-root descriptor with `mkdirat`/`openat` and `O_NOFOLLOW`,
so replacing a directory with a symlink cannot redirect bytes outside staging.
Expansion uses only `0700` directories, `0700` executable files and `0600`
other files.

The manifest is format 1 and retains the existing camel-case fields:

```text
Manifest = {manifestVersion:1,id:reverse-dns,version:semver,
  displayName:str,description?:str,minimumHostVersion?:semver,
  platforms:[{os:"macos"|"linux"|"windows",minimumVersion?:numeric-dotted,
              architectures:[str,...]},...],
  capabilities:[{id:qualified,reason?:str},...],
  components:[{id:str,type:ComponentType,path:relative,
               capabilities:[qualified,...]},...]}
ComponentType = "mcp-server"|"external-tool"|"skill"|"command"|"hook"|
                "native-helper"|"assets"
```

IDs, paths, platforms, architectures, capabilities and component paths are
duplicate-free. Component capability use is a subset of package requests.
Every `native-helper` requests and uses `native-helper.execute`. Skill and
assets components are directories; a skill contains a regular `SKILL.md`.
Other component types are regular files, and a native helper is executable.
Every path is revalidated below the immutable package root after expansion.

Semantic versions follow SemVer 2.0. Build metadata remains part of exact
package/version identity but is ignored by precedence comparisons. Host
compatibility is evaluated before a receipt is published. The plugin host
version is the Kernel product release version, not the independent Session
Endpoint `host.describe` protocol build version.

## Integrity and trust

Every package has a content digest. Entries, including root `.`, are sorted by
relative path. Each directory hashes length-framed UTF-8 fields
`directory,path`; each file hashes `file,path,executable-bit` followed by its
length-framed bytes. Lengths are unsigned 64-bit big endian. Links and special
files are rejected.

An optional existing publisher attestation at
`.codex-plugin/publisher.sig.json` contains
`{publisherID,publicKey,signature,contentDigest}`. The content digest excludes
only the attestation file, the public key and signature are base64 Ed25519,
and the signature covers the lowercase digest's UTF-8 bytes. Installation
records the publisher id, SHA-256 public-key fingerprint and signed digest.
Trust policy fails closed by default. It may explicitly accept a locally
authorized unsigned package, whose immutable digest is then pinned in the
receipt, or require a publisher id plus the SHA-256 fingerprint of its Ed25519
public key. A self-asserted publisher id is not trust, and an untrusted package
cannot claim verified integrity.

Every executable component that requests `native-helper.execute` is
independently strict-code-signature verified before publication. This includes
an `mcp-server` whose signed application identity owns platform permission
state; it does not imply that every native executable speaks MCP. Its component
id, path, designated requirement, signing identifier and optional team
identifier are pinned in the receipt and checked again before enabled
projection. Package digest and publisher signature do not replace the native
executable's stable identity.

No current Computer Use archive or manifest participates in runtime. The
frozen predecessor 0.1.5 archive may be supplied only as historical carrier
compatibility evidence; it creates no current plugin binding. Stable Apple
signing of the separately distributed Computer Use executable is checked by
Slice 14A through the generic MCP process path, not by the plugin manager.

## Store and receipts

The manager owns one root:

```text
registry.canonical.json
operations/current.canonical.json
.staging/<operation>/
packages/<plugin-id>/<version>/<digest>/...
data/<plugin-id>/
```

Registry and operation documents are RFC-8785 canonical JSON plus LF, format
1, published by create-new temporary file, `store::FullSync`, atomic rename and
`FullSync` of the parent. Every newly created layout/package/data directory is
fully synced with its parent; staging-to-digest rename fully syncs both source
and destination parents before and after the rename. Installed package trees
are read-only. Writable data is outside the
immutable tree and never contributes executable content or grants.

A receipt pins plugin/version/display name, package digest/path, source,
integrity, requested/granted capabilities, native-helper identities, optional
publisher identity and enabled state. Plugin IDs are unique. Enabling requires
every requested capability granted. Grant replacement rejects undeclared
capabilities and automatically disables an installation if any required grant
is removed. Grants authorize later activation only; they do not bypass ordinary
tool approval or operating-system consent.

Install is update when the plugin id already exists. Lower precedence requires
downgrade authorization. Changed content at equal SemVer precedence—including
different build metadata—requires same-version-replacement authorization.
Remove is idempotent. Registry removal is the activation boundary; immutable
package and data cleanup follows it.

## Transactions and recovery

Every install/update/remove/enable/grants mutation first recovers the single
operation journal. Install expands into staging, validates all bytes and
signatures, publishes the immutable digest path, then publishes an operation
containing complete previous/next receipts. A crash after durable package/data
publication but before the operation journal leaves only unreferenced state and
therefore rolls back through ordinary orphan collection. Registry publication
is the commit point. The operation advances from `package-published` to
`registry-published`; the injected registry-commit fault is before that phase
update, and recovery resolves the window by comparing the complete next receipt
with the registry.

Recovery is deterministic:

- registry equals next receipt: commit, remove superseded package or removed
  plugin data, then clear the operation;
- otherwise: rollback, remove an unreferenced candidate, keep the previous
  receipt/package, then clear the operation;
- clear inert staging and garbage-collect only package/data paths unreferenced
  by the recovered registry.

Unknown formats, noncanonical state, escaping paths, duplicate receipts,
undeclared grants, enabled receipts missing grants, digest/signature drift or
cleanup outside the owned roots fail closed.

## Component projection and collision ownership

Only enabled, fully granted, revalidated receipts project components. Each row
names the owner plugin/version, component id/type, immutable package and
component paths, writable data path and grants. Slice 12 projections always
carry `launchable:false`.

Runtime configuration refers to an executable component only by the generic
tuple `{pluginId,componentId}`. `PluginStore::resolve_executable_component`
revalidates the enabled, fully granted receipt and resolves that tuple to the
immutable executable path, writable plugin data path, component type, grants,
and `pluginGeneration` equal to the package digest. It returns no launch
authority. Slice 13 may use this relation for an ordinary plugin-owned MCP
stdio server and must include `pluginGeneration` in its peer/pool key; no
product-specific component mapping is permitted.

The collision key is `(component type, component id)`. Two enabled plugins
claiming one key fail the whole projection with both owners named; install
bytes remain inert. Fixed builtin tool names are never modified by this
projection, and MCP tool names do not exist until Slice 13 supplies a live
catalog.

Typed management operations are `install`, `set-enabled`, `set-grants`,
`remove`, `list`, `components` and `recover`. This is the Kernel library
authority and is owned by one mutable supervisor-side manager instance. Public
Session Endpoint exposure composes these operations through the additive
`ProductionEndpointRoutes` extension validator/execute seam. It is a Slice-14F
capability decision and must not add to or otherwise change the frozen v2
20-registration authority implicitly.

## Conformance

Gates 81–85 are fixed:

81. hermetic manifest/directory/archive carrier compatibility and inert projection;
82. hermetic native-helper/publisher signature, trust and grants rejection;
83. install/update/remove/enable, SemVer precedence and collision ownership;
84. install/update/remove/enable/grants crash recovery, controlled temporary
    cleanup, both publication/journal gaps and commit-or-rollback disk results;
85. typed management, generic executable relation, canonical durable state and
    no-launch boundary.

Gates 81/82 use a synthetic plugin helper and prove only the carrier and policy
logic. They make no claim about Computer Use. Slice 14A's separate mandatory
macOS lane receives explicit executable paths and applies strict `codesign`;
missing reference input is `NOT QUALIFIED`, never a passed or skipped
qualification.
