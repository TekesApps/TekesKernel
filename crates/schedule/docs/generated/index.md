# schedule — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `schedule` | lib | [crates/schedule/src/lib.rs](../../src/lib.rs) |
| `slice14c_gates` | test | [crates/schedule/tests/slice14c_gates.rs](../../tests/slice14c_gates.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `store` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/schedule/src/lib.rs](../../src/lib.rs) | 58 | 603 | [Symbols and calls](src--lib.md) |
| [crates/schedule/tests/slice14c_gates.rs](../../tests/slice14c_gates.rs) | 11 | 171 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  none["No resolved cross-file calls; inspect the call-site inventory"]
```
