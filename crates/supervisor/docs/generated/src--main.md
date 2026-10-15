# tekes-supervisor::main

[Package atlas](index.md) · [Source](../../src/main.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::main::main](../../src/main.rs#L5) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `env` | `std::env` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `ExitCode` | `std::process::ExitCode` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 3 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::supported_dialects"]
  n1["tekes-supervisor::builtin::run"]
  n2["tekes-supervisor::folder_trust::cli"]
  n3["tekes-supervisor::main::main"]
  n3 --> n0
  n3 --> n1
  n3 --> n2
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `main` | `env::args_os().nth(1).as_deref` | [6](../../src/main.rs#L6), [16](../../src/main.rs#L16), [29](../../src/main.rs#L29), [53](../../src/main.rs#L53) | receiver-type-required |
| `main` | `env::args_os().nth` | [6](../../src/main.rs#L6), [16](../../src/main.rs#L16), [29](../../src/main.rs#L29), [53](../../src/main.rs#L53) | receiver-type-required |
| `main` | `env::args_os` | [6](../../src/main.rs#L6), [7](../../src/main.rs#L7), [15](../../src/main.rs#L15), [16](../../src/main.rs#L16), [29](../../src/main.rs#L29), [30](../../src/main.rs#L30), [52](../../src/main.rs#L52), [53](../../src/main.rs#L53) | external-constructor-callback-or-unresolved |
| `main` | `Some` | [6](../../src/main.rs#L6), [16](../../src/main.rs#L16), [29](../../src/main.rs#L29), [53](../../src/main.rs#L53) | external-constructor-callback-or-unresolved |
| `main` | `std::ffi::OsStr::new` | [6](../../src/main.rs#L6), [16](../../src/main.rs#L16), [29](../../src/main.rs#L29), [53](../../src/main.rs#L53) | external-constructor-callback-or-unresolved |
| `main` | `tekes_supervisor::folder_trust::cli` | [7](../../src/main.rs#L7) | [tekes-supervisor::folder_trust::cli](../../src/folder_trust.rs#L133) |
| `main` | `env::args_os().collect::<Vec<_>>` | [7](../../src/main.rs#L7), [30](../../src/main.rs#L30) | receiver-type-required |
| `main` | `ExitCode::from` | [11](../../src/main.rs#L11), [25](../../src/main.rs#L25), [32](../../src/main.rs#L32), [47](../../src/main.rs#L47), [69](../../src/main.rs#L69) | external-constructor-callback-or-unresolved |
| `main` | `env::args_os().len` | [15](../../src/main.rs#L15), [52](../../src/main.rs#L52) | receiver-type-required |
| `main` | `serde_json::to_string` | [18](../../src/main.rs#L18) | external-constructor-callback-or-unresolved |
| `main` | `provider::supported_dialects` | [18](../../src/main.rs#L18) | [provider::dialect::supported_dialects](../../../provider/src/dialect.rs#L670) |
| `main` | `args.len` | [31](../../src/main.rs#L31) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread()             .enable_all()             .build` | [34](../../src/main.rs#L34) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread()             .enable_all` | [34](../../src/main.rs#L34) | receiver-type-required |
| `main` | `tokio::runtime::Builder::new_multi_thread` | [34](../../src/main.rs#L34) | external-constructor-callback-or-unresolved |
| `main` | `runtime.block_on` | [39](../../src/main.rs#L39) | receiver-type-required |
| `main` | `tekes_supervisor::builtin::run` | [39](../../src/main.rs#L39) | [tekes-supervisor::builtin::run](../../src/builtin.rs#L33) |
| `main` | `PathBuf::from` | [39](../../src/main.rs#L39) | external-constructor-callback-or-unresolved |
| `main` | `Err` | [41](../../src/main.rs#L41) | external-constructor-callback-or-unresolved |
| `main` | `Box::new` | [41](../../src/main.rs#L41) | external-constructor-callback-or-unresolved |
| `main` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [56](../../src/main.rs#L56) | receiver-type-required |
| `main` | `tekes_supervisor::host_runtime::SOURCE_REVISION             .map(&#124;revision&#124; format!("\"source_revision\":\"{revision}\","))             .unwrap_or_default` | [57](../../src/main.rs#L57) | receiver-type-required |
| `main` | `tekes_supervisor::host_runtime::SOURCE_REVISION             .map` | [57](../../src/main.rs#L57) | receiver-type-required |
