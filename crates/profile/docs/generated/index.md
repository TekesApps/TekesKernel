# profile — generated atlas

[All packages](../../../../docs/architecture/generated/index.md) · [Maintained explanation](../README.md)

## Cargo targets

| Target | Kind | Entry |
|---|---|---|
| `profile` | lib | [crates/profile/src/lib.rs](../../src/lib.rs) |
| `folder_trust` | test | [crates/profile/tests/folder_trust.rs](../../tests/folder_trust.rs) |
| `session_scope_resolution` | test | [crates/profile/tests/session_scope_resolution.rs](../../tests/session_scope_resolution.rs) |
| `slice8_launch_bindings` | test | [crates/profile/tests/slice8_launch_bindings.rs](../../tests/slice8_launch_bindings.rs) |
| `storage_layout` | test | [crates/profile/tests/storage_layout.rs](../../tests/storage_layout.rs) |
| `web_search_config` | test | [crates/profile/tests/web_search_config.rs](../../tests/web_search_config.rs) |

## Direct workspace dependencies

| Dependency | Kind | Target cfg |
|---|---|---|
| `schema` | normal | all |
| `store` | normal | all |
| `tools` | normal | all |

[Public/restricted API and direct callers](api.md)

## Source files / lexical modules

`src` files have full symbol/call tables and partitioned function graphs. Integration tests and examples remain in the JSON inventory.
File-derived paths below are lexical navigation, not a compiler-resolved module tree; inline module declarations are listed separately.

| File | Symbols | Call sites | Detail |
|---|---:|---:|---|
| [crates/profile/src/config.rs](../../src/config.rs) | 113 | 1051 | [Symbols and calls](src--config.md) |
| [crates/profile/src/folder_authority.rs](../../src/folder_authority.rs) | 29 | 290 | [Symbols and calls](src--folder_authority.md) |
| [crates/profile/src/folder_catalog.rs](../../src/folder_catalog.rs) | 18 | 214 | [Symbols and calls](src--folder_catalog.md) |
| [crates/profile/src/instruction.rs](../../src/instruction.rs) | 56 | 487 | [Symbols and calls](src--instruction.md) |
| [crates/profile/src/launch.rs](../../src/launch.rs) | 46 | 328 | [Symbols and calls](src--launch.md) |
| [crates/profile/src/lib.rs](../../src/lib.rs) | 5 | 38 | [Symbols and calls](src--lib.md) |
| [crates/profile/src/resources.rs](../../src/resources.rs) | 43 | 359 | [Symbols and calls](src--resources.md) |
| [crates/profile/src/scope.rs](../../src/scope.rs) | 27 | 404 | [Symbols and calls](src--scope.md) |
| [crates/profile/tests/folder_trust.rs](../../tests/folder_trust.rs) | 1 | 51 | `inventory.json` |
| [crates/profile/tests/session_scope_resolution.rs](../../tests/session_scope_resolution.rs) | 8 | 56 | `inventory.json` |
| [crates/profile/tests/slice8_launch_bindings.rs](../../tests/slice8_launch_bindings.rs) | 18 | 146 | `inventory.json` |
| [crates/profile/tests/storage_layout.rs](../../tests/storage_layout.rs) | 19 | 330 | `inventory.json` |
| [crates/profile/tests/web_search_config.rs](../../tests/web_search_config.rs) | 1 | 4 | `inventory.json` |

## Cross-file direct calls

Source files stand in for out-of-line modules; see declarations for inline modules. Counts are syntactic call sites, not runtime frequency.

```mermaid
flowchart LR
  m0["config.rs"]
  m1["folder_authority.rs"]
  m2["folder_catalog.rs"]
  m3["instruction.rs"]
  m4["launch.rs"]
  m5["lib.rs"]
  m6["scope.rs"]
  m0 -->|"14"| m5
  m1 -->|"1"| m0
  m1 -->|"3"| m5
  m2 -->|"6"| m5
  m3 -->|"2"| m1
  m3 -->|"12"| m5
  m4 -->|"2"| m5
  m6 -->|"6"| m2
  m6 -->|"2"| m3
  m6 -->|"2"| m5
```
