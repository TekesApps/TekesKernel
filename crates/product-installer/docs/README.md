# product-installer — signed product installation

[Four-layer architecture](../../../docs/architecture/README.md) · [All crates](../../../docs/architecture/crates.md) · [Complete source index](generated/index.md)

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
