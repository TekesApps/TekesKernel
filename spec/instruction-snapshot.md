# Instruction snapshot v1 — deterministic three-scope capture

This contract owns discovery, normalization, merge, materialization, and
spawn-time use of the D-55 instruction plane. It is an executable contract.

## Inputs and discovery

The resolver receives explicit user and workspace data directories plus the
ordered, canonical folder path list from a ConfigSnapshot. Production passes
`~/.agents`, `~/.agents/workspaces/<workspace-id>`, and the current paths bound
by that workspace. It never consults an ambient home directory.

User level, beneath the supplied directory:

```text
AGENTS.md
settings.json
skills/**
commands/**
hooks/**
```

Workspace level, beneath the supplied workspace directory, has the same
optional `AGENTS.md`, `settings.json`, `skills/**`, `commands/**`, and
`hooks/**` shape. In the target layout only `skills/` is ordinarily authored;
the other names remain closed optional inputs rather than inferred behavior.

For each bound folder, in config order, project level is:

```text
<cwd>/AGENTS.md
<cwd>/.agents/settings.json
<cwd>/.agents/skills/**
<cwd>/.agents/commands/**
<cwd>/.agents/hooks/**
<cwd>/.agents/instructions/**
<cwd>/.agents/rules/**
```

If and only if `<cwd>/.agents/` is absent, format 1 reads the legacy
`<cwd>/.agent/{settings.json,skills/**,commands/**,hooks/**}` layout. It never
merges both project directories. `.agents/instructions/**` and `.agents/rules/**`
are ordered AGENTS-like sources. `.agents/tools/**` and `.agents/project.json`
are reserved portable project content but are not automatically executed or
captured by this format; executable tools require a separately declared
builtin, helper, MCP, or plugin contract.

Missing optional files/directories contribute nothing. Nested AGENTS.md files
outside these exact locations are ignored in v1. Directory symlinks, file
symlinks, non-regular files, non-UTF-8 names/content, NUL, and path components
`.` or `..` reject capture. A file is opened no-follow; descriptor metadata is
checked before/after read and revalidated against the pathname inode.

Logical relative paths use `/`, are Unicode NFC, and are sorted by UTF-8 byte
order. NFC collisions reject. Limits are 4096 files, 1 MiB per file, and 4 MiB
total source bytes. File content is preserved byte-for-byte in the snapshot;
there is no newline or Unicode normalization. `settings.json` and every
`hooks/**` binding must be RFC-8785 canonical JSON plus one LF. Hook binding
bytes and validation are owned by `tool-hook`; the instruction snapshot
retains them verbatim.

## Settings schema and conservative meet

Every instruction settings file has the closed schema:

```text
{
  format: 1,
  policy?: {
    network?: bool,
    allowed_tools?: [str, ...],
    writable_roots?: [AbsolutePath, ...],
    max_wall_seconds?: int>=1
  }
}
```

Absent fields impose no additional constraint. Across all settings sources,
policy is the conservative meet: booleans AND, string/path sets intersect,
and numeric caps take the minimum. A field present in only one source is that
source's constraint. Lists are deduplicated and sorted. Instruction policy is
then met with the ConfigSnapshot workspace policy by the same rules; it can
remove privilege but never add it.

## Snapshot schema and precedence

```text
InstructionSnapshot = {
  format: 1,
  sources: [Source, ...],
  effective: {
    agents: [source-index, ...],
    skills: {<logical-name>: source-index, ...},
    commands: {<logical-name>: source-index, ...},
    hooks: {<logical-name>: source-index, ...},
    policy: InstructionPolicy
  }
}

Source = {
  origin: {scope:"user"} |
          {scope:"workspace"} |
          {scope:"project", workspace_index:int},
  path: str,
  kind: "agents" | "skill" | "command" | "hook" | "settings",
  content: str,
  content_sha256: str
}
```

Sources are ordered user first, workspace second, then projects in workspace
folder order; within one
origin they are ordered `AGENTS.md`, `settings.json`, then resources by the
closed kind order `command`, `hook`, `skill`, and finally logical path by
UTF-8 bytes within a kind. `content_sha256` is lowercase SHA-256 of the exact
UTF-8 content bytes.

For a session-bound ConfigSnapshot, `workspace.selected_cwd` chooses the
execution scope only. The resolver MUST still capture all entries in
`workspace.cwd` in their authored order, so selecting a secondary binding
cannot change `workspace_index`, instruction precedence, or which project
resources are frozen.

The absolute folder path is deliberately not duplicated in a Source: the same
launch's ConfigSnapshot binds `workspace_index` to its canonical folder. This keeps an
instruction snapshot relocatable without weakening provenance.

Every AGENTS/instructions/rules source index appears in `effective.agents` in source order;
later entries have higher instruction precedence but earlier text is not
dropped. Commands and hooks are keyed by their path beneath the kind directory;
their legacy effective map retains later-source precedence. Skill package
names are different: the resource catalog groups the frozen package sources
by explicit origin and rejects the same skill name appearing in user,
workspace, or any project scope. No cross-scope skill silently replaces
another. Production builds a user-only daemon command catalog and a separate
catalog for each configured workspace. Each workspace catalog includes user,
workspace, and every bound project folder, so collisions inside one workspace
remain errors; two unrelated workspace catalogs may independently use the
same skill name and MUST NOT block daemon assembly. Settings sources are
retained for provenance and contribute only
through the policy meet.
Every hook source, including a shadowed one, must independently
validate as a format-1 tool-hook or [format-2 lifecycle-hook](lifecycle-hook.md) binding whose `id` equals its logical path below
`hooks/`.

At execution time, the worker iterates the effective hook map in logical-name
UTF-8 byte order, decodes each referenced binding from the frozen Source
`content` bytes, and preserves that order while filtering the `pre` and `post`
phases. Format-2 lifecycle bindings are dispatched separately by event; they never
enter the model tool pipeline. It never rescans a hook directory after launch.

Validation recomputes this entire effective projection from `sources` and
requires byte-for-value equality with the stored `effective` object. It does
not merely check that referenced indices are in range; a reordered source
list, an earlier shadow winner, an omitted effective resource, or a forged
policy meet rejects.

All maps have unique keys. Indices are zero-based safe integers and MUST name
a source of the matching kind. Every AGENTS source is referenced. Settings
need no direct reference. An unreferenced skill/command/hook is valid only
when a later source of the same kind and logical key shadows it; otherwise it
rejects as orphaned snapshot material.

Slice 11 does not change these snapshot bytes or the legacy effective maps.
[skill-package](skill-package.md) groups frozen
`skills/<name>/**` sources by origin, selects a whole package as one unit, and
uses `effective.skills` only for the predecessor flat-skill projection. Its
cross-scope duplicate-name check is the executable no-silent-override gate.
[command-catalog](command-catalog.md) consumes the top-level `.md`
winners already named by `effective.commands`. Neither projection performs a
second filesystem scan.

## Stable capture

Ordinary editors do not participate in a kernel lock, so the contract does
not invent cross-file transactionality. Each file is an independent authority
record. To prevent a spawn from accepting a file being edited during capture,
the resolver performs complete scans until two consecutive normalized source
inventories and byte contents are identical. It accepts the second scan,
otherwise retries up to 8 scans and returns `unstable_source`. Atomic editor
rename is observed wholly old or wholly new; in-place writes are detected by
the descriptor/path metadata and the consecutive-scan comparison. An
adversarial ABA writer is outside this cooperative user-config authority.

After stable capture, merge and canonicalization operate only on the captured
bytes. Later filesystem edits cannot change the snapshot.

## Bytes, publication, and epochs

Snapshot bytes are RFC-8785 canonical JSON plus exactly one LF. The snapshot
digest is lowercase SHA-256 of those exact bytes. Before launch the supervisor
publishes it as `assets/sha256-<digest>` in the target thread folder using the
D-45 reference protocol. `genesis.instruction.digest` when present and every
`run_start.instruction_digest` are implicit references; GC, fork, archive, and
redact carrier scans retain/process the asset.

The worker receives an already-open read-only descriptor, verifies and parses
the snapshot once before taking the line lock, and never reads instruction
paths. A run therefore uses exactly one immutable snapshot. If its digest
differs from the latest epoch profile, the worker opens a new epoch with the
instruction digest as a system-source reason before the first provider
attempt. Same digest means no instruction-driven epoch.

## Typed errors

The closed classes are `invalid_path`, `symlink`, `not_regular`,
`invalid_bytes` (invalid UTF-8 or malformed bytes), `invalid_schema` (a
`settings.json` or manifest that parses but violates its shape),
`invalid_reference`, `limit_exceeded`, `unstable_source`, `digest_mismatch`,
`unsupported_format`, `stale_revision`, `store`, and `io`
(`profile::ProfileError`). Error locations may name
origin and logical path but never include file content.
