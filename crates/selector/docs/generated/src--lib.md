# tekes-selector

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Command` | `cli::Command` | `pub` |
| `parse_args` | `cli::parse_args` | `pub` |
| `run_command` | `cli::run_command` | `pub` |
| `ErrorCode` | `error::ErrorCode` | `pub` |
| `SelectorError` | `error::SelectorError` | `pub` |
| `INSTALLER_PHASES` | `installer::INSTALLER_PHASES` | `pub` |
| `InstallRequest` | `installer::InstallRequest` | `pub` |
| `InstallerEffects` | `installer::InstallerEffects` | `pub` |
| `InstallerOperation` | `installer::InstallerOperation` | `pub` |
| `InstallerOperationType` | `installer::InstallerOperationType` | `pub` |
| `InstallerStateMachine` | `installer::InstallerStateMachine` | `pub` |
| `durable_credential_file_for_test` | `installer::durable_credential_file_for_test` | `pub` |
| `random_credential` | `installer::random_credential` | `pub` |
| `*` | `model::*` | `pub` |
| `EMBEDDED_AUTHORITY_REGISTRY_SHA256` | `selector::EMBEDDED_AUTHORITY_REGISTRY_SHA256` | `pub` |
| `EMBEDDED_SELECTOR_CONFORMANCE_SHA256` | `selector::EMBEDDED_SELECTOR_CONFORMANCE_SHA256` | `pub` |
| `FailureDisposition` | `selector::FailureDisposition` | `pub` |
| `Selector` | `selector::Selector` | `pub` |
| `SelectorPaths` | `selector::SelectorPaths` | `pub` |
| `automatic_rollback_sha256` | `selector::automatic_rollback_sha256` | `pub` |
| `cli_command_sha256` | `selector::cli_command_sha256` | `pub` |
| `describe_conformance` | `selector::describe_conformance` | `pub` |
| `reply_bytes` | `selector::reply_bytes` | `pub` |
| `CodeSignature` | `signature::CodeSignature` | `pub` |
| `CodeSignatureVerifier` | `signature::CodeSignatureVerifier` | `pub` |
| `MacOsCodeSignatureVerifier` | `signature::MacOsCodeSignatureVerifier` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-selector::canary` | `private` |  |
| `tekes-selector::cli` | `private` |  |
| `tekes-selector::error` | `private` |  |
| `tekes-selector::fs` | `private` |  |
| `tekes-selector::installer` | `private` |  |
| `tekes-selector::model` | `private` |  |
| `tekes-selector::selector` | `private` |  |
| `tekes-selector::signature` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
