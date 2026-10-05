# deployment-tests — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `deployment_tests` | lib | [crates/deployment-tests/src/lib.rs](../../src/lib.rs) |
| `gate76_installer` | test | [crates/deployment-tests/tests/gate76_installer.rs](../../tests/gate76_installer.rs) |
| `portable` | test | [crates/deployment-tests/tests/portable.rs](../../tests/portable.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `tekes-selector` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/deployment-tests/src/lib.rs](../../src/lib.rs) | 8 | 15 | [Symbols and calls](src--lib.md) |
| [crates/deployment-tests/tests/gate76_installer.rs](../../tests/gate76_installer.rs) | 26 | 365 | `inventory.json` |
| [crates/deployment-tests/tests/portable.rs](../../tests/portable.rs) | 12 | 313 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  none["No resolved cross-file calls; inspect the call-site inventory"]
```
