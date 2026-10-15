# store — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `store` | lib | [crates/store/src/lib.rs](../../src/lib.rs) |

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
| [crates/store/src/asset.rs](../../src/asset.rs) | 13 | 100 | [Symbols and calls](src--asset.md) |
| [crates/store/src/atomic.rs](../../src/atomic.rs) | 4 | 29 | [Symbols and calls](src--atomic.md) |
| [crates/store/src/card/tests.rs](../../src/card/tests.rs) | 21 | 265 | [Symbols and calls](src--card--tests.md) |
| [crates/store/src/card.rs](../../src/card.rs) | 50 | 298 | [Symbols and calls](src--card.md) |
| [crates/store/src/folder.rs](../../src/folder.rs) | 47 | 736 | [Symbols and calls](src--folder.md) |
| [crates/store/src/folder_events/tests.rs](../../src/folder_events/tests.rs) | 13 | 179 | [Symbols and calls](src--folder_events--tests.md) |
| [crates/store/src/folder_events.rs](../../src/folder_events.rs) | 19 | 73 | [Symbols and calls](src--folder_events.md) |
| [crates/store/src/lib.rs](../../src/lib.rs) | 3 | 7 | [Symbols and calls](src--lib.md) |
| [crates/store/src/management_root.rs](../../src/management_root.rs) | 9 | 50 | [Symbols and calls](src--management_root.md) |
| [crates/store/src/platform.rs](../../src/platform.rs) | 37 | 152 | [Symbols and calls](src--platform.md) |
| [crates/store/src/rewrite.rs](../../src/rewrite.rs) | 93 | 1921 | [Symbols and calls](src--rewrite.md) |
| [crates/store/src/tail.rs](../../src/tail.rs) | 41 | 349 | [Symbols and calls](src--tail.md) |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  m0["asset.rs"]
  m1["atomic.rs"]
  m2["card.rs"]
  m3["folder.rs"]
  m4["folder_events.rs"]
  m5["lib.rs"]
  m6["management_root.rs"]
  m7["platform.rs"]
  m8["rewrite.rs"]
  m9["tail.rs"]
  m2 -->|"1"| m7
  m3 -->|"1"| m0
  m3 -->|"1"| m2
  m3 -->|"4"| m4
  m3 -->|"14"| m7
  m3 -->|"9"| m9
  m8 -->|"3"| m0
  m8 -->|"1"| m1
  m8 -->|"1"| m3
  m8 -->|"2"| m4
  m8 -->|"1"| m6
  m8 -->|"11"| m7
  m8 -->|"2"| m9
  m9 -->|"1"| m2
  m9 -->|"1"| m5
  m9 -->|"2"| m7
```
