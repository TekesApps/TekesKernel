# tekes-worker — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `tekes-worker` | bin | [crates/worker/src/main.rs](../../src/main.rs) |
| `shell` | test | [crates/worker/tests/shell.rs](../../tests/shell.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `engine` | normal | all |
| `profile` | normal | all |
| `provider` | normal | all |
| `schema` | normal | all |
| `session-controls` | normal | all |
| `store` | normal | all |
| `tools` | normal | all |
| `worker-control` | normal | all |
| `workspace-service` | normal | all |
| `endpoint` | dev | all |
| `tekes-supervisor` | dev | all |
| `test-support` | dev | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/worker/src/identity.rs](../../src/identity.rs) | 8 | 131 | [Symbols and calls](src--identity.md) |
| [crates/worker/src/lifecycle_hooks.rs](../../src/lifecycle_hooks.rs) | 11 | 126 | [Symbols and calls](src--lifecycle_hooks.md) |
| [crates/worker/src/live_tests.rs](../../src/live_tests.rs) | 17 | 795 | [Symbols and calls](src--live_tests.md) |
| [crates/worker/src/main.rs](../../src/main.rs) | 353 | 7610 | [Symbols and calls](src--main.md) |
| [crates/worker/src/result_presentation.rs](../../src/result_presentation.rs) | 16 | 158 | [Symbols and calls](src--result_presentation.md) |
| [crates/worker/src/validation_runtime.rs](../../src/validation_runtime.rs) | 37 | 850 | [Symbols and calls](src--validation_runtime.md) |
| [crates/worker/src/validation_tests.rs](../../src/validation_tests.rs) | 19 | 451 | [Symbols and calls](src--validation_tests.md) |
| [crates/worker/src/workspace_edits.rs](../../src/workspace_edits.rs) | 10 | 153 | [Symbols and calls](src--workspace_edits.md) |
| [crates/worker/tests/shell.rs](../../tests/shell.rs) | 21 | 1093 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  none["No resolved cross-file calls; inspect the call-site inventory"]
```
