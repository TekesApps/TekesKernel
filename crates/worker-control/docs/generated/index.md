# worker-control — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `worker_control` | lib | [crates/worker-control/src/lib.rs](../../src/lib.rs) |
| `durable_control` | test | [crates/worker-control/tests/durable_control.rs](../../tests/durable_control.rs) |
| `tool_completion` | test | [crates/worker-control/tests/tool_completion.rs](../../tests/tool_completion.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `schema` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/worker-control/src/continuation.rs](../../src/continuation.rs) | 22 | 112 | [Symbols and calls](src--continuation.md) |
| [crates/worker-control/src/durable.rs](../../src/durable.rs) | 56 | 237 | [Symbols and calls](src--durable.md) |
| [crates/worker-control/src/lib.rs](../../src/lib.rs) | 40 | 107 | [Symbols and calls](src--lib.md) |
| [crates/worker-control/tests/durable_control.rs](../../tests/durable_control.rs) | 10 | 45 | `inventory.json` |
| [crates/worker-control/tests/tool_completion.rs](../../tests/tool_completion.rs) | 1 | 12 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  m0["continuation.rs"]
  m1["lib.rs"]
  m0 -->|"4"| m1
```
