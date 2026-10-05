# plugins — plugin package lifecycle

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Inspects, installs, updates, removes, enables and authorizes plugin packages, producing component projections.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `archive` | Package reading and archive boundaries |
| `model` | Manifests, components, platform requirements and executable references |
| `signature` | Signature, publisher and native helper verification |
| `store` | PluginStore transactions, authorization, integrity and recovery |

## Interfaces and calls

Main entry points: PluginStore, PluginArchive, Manifest, ComponentProjection, SignaturePolicy.

Read package → verify manifest/signatures → installation transaction → component projection; runtime consumers then decide whether launch is allowed.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Enabling a plugin is not process startup. The generic MCP carrier consumes authorized mcp-server components; this does not imply every component can run.

Behavior contract: [plugin-package.md](../../../spec/plugin-package.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
