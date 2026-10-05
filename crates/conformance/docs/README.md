# conformance — cross-crate conformance tests

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

The tests directory holds cross-component gates; lib.rs only explains the test entry points.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | Empty library entry point; execution lives under tests/ |

## Interfaces and calls

Main entry points: no production public service; Cargo integration-test targets are the primary entry points.

cargo test → selected gate → fixtures / multiple crates → assert durability, ordering, recovery and protocol outcomes.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Test targets are separate compilation units; the generated target table lists them all, but this indexing task did not execute them.

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
