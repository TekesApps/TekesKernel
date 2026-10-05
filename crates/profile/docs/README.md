# profile — configuration and immutable launch snapshots

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Fixes user/project configuration, instructions, skills and tool bindings into snapshots for an execution run.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `config` | ConfigRepository, workspace/model/provider configuration and revisions |
| `instruction` | InstructionResolver, instruction sources, policy merging and LaunchProfile |
| `launch` | LaunchBindings and dynamic tool catalog |
| `resources` | ResourceCatalog, skills and commands |

## Interfaces and calls

Main entry points: ConfigRepository, ConfigSnapshot, InstructionResolver, InstructionSnapshot, LaunchBindings, ResourceCatalog.

Read configuration → resolve instruction tree → produce immutable snapshots/bindings → supervisor delivers them to worker through launch descriptors.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

During execution, distinguish snapshots from later configuration edits; profile builds definitions, while supervisor owns concrete process wiring.

Behavior contract: [instruction-snapshot.md](../../../spec/instruction-snapshot.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.

Dynamic tool descriptions (MCP, plugin) are accepted up to
`MAX_DYNAMIC_DESCRIPTION_BYTES` (4096) with LF/CR/TAB preserved.

Workspace policy can explicitly declare read-only `toolchain_roots`. Resolution
canonicalizes, sorts and deduplicates them in the frozen snapshot; they do not
become writable roots or project folders. Changes require worker respawn so
process permissions and executable selection do not outlive their authority.
