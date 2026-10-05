# user-documents — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `user_documents` | lib | [crates/user-documents/src/lib.rs](../../src/lib.rs) |
| `documents` | test | [crates/user-documents/tests/documents.rs](../../tests/documents.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `endpoint` | normal | all |
| `schema` | normal | all |
| `store` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/user-documents/src/feedback.rs](../../src/feedback.rs) | 15 | 115 | [Symbols and calls](src--feedback.md) |
| [crates/user-documents/src/lib.rs](../../src/lib.rs) | 13 | 57 | [Symbols and calls](src--lib.md) |
| [crates/user-documents/src/settings.rs](../../src/settings.rs) | 30 | 227 | [Symbols and calls](src--settings.md) |
| [crates/user-documents/tests/documents.rs](../../tests/documents.rs) | 14 | 167 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  m0["feedback.rs"]
  m1["lib.rs"]
  m2["settings.rs"]
  m0 -->|"18"| m1
  m1 -->|"4"| m0
  m1 -->|"5"| m2
  m2 -->|"15"| m1
```
