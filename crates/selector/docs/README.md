# selector — external deployment selection and supervision

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

> **Retired (2026-10-06).** The Kernel is launched only by its host application; see
> [Application-owned launch](../../../docs/builtin-launch.md). The launchd service, product
> installer and selector described here are no longer maintained, and their
> Keychain credential model no longer matches the code: the endpoint token and
> provider secrets come from the host's environment
> ([secret-store](../../../spec/secret-store.md)). The code remains until it is removed
> ([#28](https://github.com/TekesApps/TekesKernel/issues/28)); this document is kept for reference.

Selects a supervisor from installed, verified products and handles startup observation, failures and rollback.

## Modules and responsibilities

| Module / file group | Responsibility |
|---|---|
| `main / cli` | CLI, environment cleanup and serve dispatch |
| `selector` | Selection, launch, bootstrap, crash loops and rollback |
| `model / error` | Deployment data model and errors |
| `signature / fs` | Signature verification and secure file operations |
| `installer` | Deployment transaction state machine and effect interfaces |
| `canary` | Deployment probes |

## Interfaces and calls

Main entry points: Selector, SelectorPaths, InstallerStateMachine, CodeSignatureVerifier, run_command.

main → parse_args/run_command → Selector → inspect installation state and signatures → spawn supervisor → monitor bootstrap/lifetime.

For complete declarations (including private functions), pub/re-export paths and call sites, see the [generated index](generated/index.md). Each src page contains grouped function call graphs; cross-file graphs live in the package index.

## Boundary

Selector has no production source dependency on supervisor/worker; launching an executable does not appear in the Cargo dependency graph.

Behavior contract: [deployment.md](../../../spec/deployment.md).

These are static descriptions of the worktree source. An unresolved method in a diagram does not imply no calls; runtime outcomes require separate evidence.
