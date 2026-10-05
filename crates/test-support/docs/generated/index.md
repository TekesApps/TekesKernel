# test-support — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `test_support` | lib | [crates/test-support/src/lib.rs](../../src/lib.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `engine` | normal | all |
| `schema` | normal | all |
| `store` | normal | all |
| `worker-control` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/test-support/src/lib.rs](../../src/lib.rs) | 11 | 130 | [Symbols and calls](src--lib.md) |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  none["No resolved cross-file calls; inspect the call-site inventory"]
```
