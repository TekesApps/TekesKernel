# store::management_root

[Package atlas](index.md) · [Source](../../src/management_root.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [store::management_root::ENDPOINT_RPC_DIR](../../src/management_root.rs#L6) | const_item | `pub` |  |
| [store::management_root::SESSION_SETTINGS_FILE](../../src/management_root.rs#L7) | const_item | `pub` |  |
| [store::management_root::retire_legacy_name](../../src/management_root.rs#L14) | function_item | `pub` |  |
| [store::management_root::endpoint_rpc_root](../../src/management_root.rs#L40) | function_item | `pub` |  |
| [store::management_root::session_settings_path](../../src/management_root.rs#L44) | function_item | `pub` |  |
| [store::management_root::tests::retired_state_is_never_imported_or_modified](../../src/management_root.rs#L52) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `fs` | `std::fs` | `private` |
| `io` | `std::io` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `store::management_root::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–3: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["store::management_root::retire_legacy_name"]
  n1["store::management_root::endpoint_rpc_root"]
  n2["store::management_root::session_settings_path"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `retire_legacy_name` | `parent.join` | [15](../../src/management_root.rs#L15), [16](../../src/management_root.rs#L16) | receiver-type-required |
| `retire_legacy_name` | `fs::symlink_metadata(&legacy_path).is_ok` | [17](../../src/management_root.rs#L17) | receiver-type-required |
| `retire_legacy_name` | `fs::symlink_metadata` | [17](../../src/management_root.rs#L17), [18](../../src/management_root.rs#L18) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `error.kind` | [19](../../src/management_root.rs#L19) | receiver-type-required |
| `retire_legacy_name` | `fs::rename` | [20](../../src/management_root.rs#L20), [25](../../src/management_root.rs#L25) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `fs::File::open(parent)?.sync_all` | [21](../../src/management_root.rs#L21), [26](../../src/management_root.rs#L26) | receiver-type-required |
| `retire_legacy_name` | `fs::File::open` | [21](../../src/management_root.rs#L21), [26](../../src/management_root.rs#L26) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `metadata.is_dir` | [23](../../src/management_root.rs#L23) | receiver-type-required |
| `retire_legacy_name` | `fs::read_dir(&current_path)?.next().is_none` | [23](../../src/management_root.rs#L23) | receiver-type-required |
| `retire_legacy_name` | `fs::read_dir(&current_path)?.next` | [23](../../src/management_root.rs#L23) | receiver-type-required |
| `retire_legacy_name` | `fs::read_dir` | [23](../../src/management_root.rs#L23) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `fs::remove_dir` | [24](../../src/management_root.rs#L24) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `Err` | [29](../../src/management_root.rs#L29), [34](../../src/management_root.rs#L34) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `io::Error::new` | [29](../../src/management_root.rs#L29) | external-constructor-callback-or-unresolved |
| `retire_legacy_name` | `Ok` | [37](../../src/management_root.rs#L37) | external-constructor-callback-or-unresolved |
| `endpoint_rpc_root` | `Ok` | [41](../../src/management_root.rs#L41) | external-constructor-callback-or-unresolved |
| `endpoint_rpc_root` | `root.join` | [41](../../src/management_root.rs#L41) | receiver-type-required |
| `session_settings_path` | `Ok` | [45](../../src/management_root.rs#L45) | external-constructor-callback-or-unresolved |
| `session_settings_path` | `thread_folder.join` | [45](../../src/management_root.rs#L45) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `tempfile::tempdir().unwrap` | [53](../../src/management_root.rs#L53) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `tempfile::tempdir` | [53](../../src/management_root.rs#L53) | external-constructor-callback-or-unresolved |
| `retired_state_is_never_imported_or_modified` | `root.path().join` | [54](../../src/management_root.rs#L54), [56](../../src/management_root.rs#L56) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `root.path` | [54](../../src/management_root.rs#L54), [56](../../src/management_root.rs#L56) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `fs::create_dir(&old).unwrap` | [55](../../src/management_root.rs#L55) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `fs::create_dir` | [55](../../src/management_root.rs#L55) | external-constructor-callback-or-unresolved |
| `retired_state_is_never_imported_or_modified` | `fs::write(root.path().join("session-settings-v1.json"), b"old").unwrap` | [56](../../src/management_root.rs#L56) | receiver-type-required |
| `retired_state_is_never_imported_or_modified` | `fs::write` | [56](../../src/management_root.rs#L56) | external-constructor-callback-or-unresolved |
