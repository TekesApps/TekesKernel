# test-support — shared test infrastructure

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

Provides fixture discovery, manifest/asset validation and shared test helpers.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | FixtureRoot, error handling and shared test helpers |

## Interfaces and calls

Main entry points: FixtureRoot, FixtureError.

Test entry point → discover fixture root → validate handwritten oracles → build test environment.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

publish=false does not prevent other packages from depending on this crate; consult Cargo's normal/dev dependency classifications for actual relationships.

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
