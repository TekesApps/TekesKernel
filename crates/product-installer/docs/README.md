# product-installer — signed product installation

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

> **Retired (2026-10-06).** The Kernel is launched only by its host application; see
> [Application-owned launch](../../../docs/builtin-launch.md). The launchd service, product
> installer and selector described here are no longer maintained, and their
> Keychain credential model no longer matches the code: the endpoint token and
> provider secrets come from the host's environment
> ([secret-store](../../../spec/secret-store.md)). The code remains until it is removed
> ([#28](https://github.com/TekesApps/TekesKernel/issues/28)); this document is kept for reference.

Performs installation/upgrades, ensure-running, start/stop and credential migration, with transaction logs supporting recovery.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `main / lib` | Argument/operation protocol and execution entry point |
| `artifact` | Product manifests and input validation |
| `transaction` | Installation publication and recovery transactions |
| `fs / process` | File and child-process operations |
| `platform` | macOS service manager, identity and Keychain boundaries |
| `migration` | Credential migration |

## Interfaces and calls

Main entry points: Arguments, Operation, execute.

main → Arguments::parse → execute → artifact/caller/platform validation → corresponding installation or service operation.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Besides lib/bin, there is a build.rs target; do not conflate these responsibilities with the lower-level deployment state machine in selector::installer.

Behavior contract: [deployment.md](../../../spec/deployment.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
