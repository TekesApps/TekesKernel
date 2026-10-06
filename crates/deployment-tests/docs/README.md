# deployment-tests — deployment test harness

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

> **Retired (2026-10-06).** The Kernel is launched only by its host application; see
> [Application-owned launch](../../../docs/builtin-launch.md). The launchd service, product
> installer and selector described here are no longer maintained, and their
> Keychain credential model no longer matches the code: the endpoint token and
> provider secrets come from the host's environment
> ([secret-store](../../../spec/secret-store.md)). The code remains until it is removed
> ([#28](https://github.com/TekesApps/TekesKernel/issues/28)); this document is kept for reference.

Distinguishes portable file/fixture tests from product acceptance requiring real signatures, launchd and user login.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `lib` | workspace_root, fixture checker and environment-variable interfaces for real binaries |

## Interfaces and calls

Main entry points: workspace_root, run_fixture_checker, configured_binary.

Tests → explicitly select build artifacts → fixture/installation/recovery checks; external release UAT remains separate.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Passing portable tests cannot substitute for restart, signature, Keychain or target-user login acceptance.

Behavior contract: [deployment.md](../../../spec/deployment.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
