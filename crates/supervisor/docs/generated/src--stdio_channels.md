# tekes-supervisor::stdio_channels

[Package atlas](index.md) · [Source](../../src/stdio_channels.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::stdio_channels::UnixStdioChannels](../../src/stdio_channels.rs#L11) | struct_item | `pub` |  |
| [tekes-supervisor::stdio_channels::UnixStdioChannels::open](../../src/stdio_channels.rs#L14) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `File` | `std::fs::File` | `private` |
| `io` | `std::io` | `private` |
| `FileTypeExt` | `std::os::unix::fs::FileTypeExt` | `private` |
| `MetadataExt` | `std::os::unix::fs::MetadataExt` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `StdioConnector` | `crate::mcp_runtime::StdioConnector` | `private` |
| `StdioOpen` | `crate::mcp_runtime::StdioOpen` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::stdio_channels::UnixStdioChannels::open"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `open` | `Box::pin` | [15](../../src/stdio_channels.rs#L15) | external-constructor-callback-or-unresolved |
| `open` | `self                 .0                 .get(channel)                 .ok_or_else` | [16](../../src/stdio_channels.rs#L16) | receiver-type-required |
| `open` | `self                 .0                 .get` | [16](../../src/stdio_channels.rs#L16) | receiver-type-required |
| `open` | `io::Error::other` | [19](../../src/stdio_channels.rs#L19), [22](../../src/stdio_channels.rs#L22), [38](../../src/stdio_channels.rs#L38) | external-constructor-callback-or-unresolved |
| `open` | `path                 .parent()                 .ok_or_else` | [20](../../src/stdio_channels.rs#L20) | receiver-type-required |
| `open` | `path                 .parent` | [20](../../src/stdio_channels.rs#L20) | receiver-type-required |
| `open` | `std::fs::symlink_metadata` | [23](../../src/stdio_channels.rs#L23), [24](../../src/stdio_channels.rs#L24) | external-constructor-callback-or-unresolved |
| `open` | `libc::geteuid` | [28](../../src/stdio_channels.rs#L28) | external-constructor-callback-or-unresolved |
| `open` | `path.is_absolute` | [29](../../src/stdio_channels.rs#L29) | receiver-type-required |
| `open` | `parent.canonicalize` | [30](../../src/stdio_channels.rs#L30) | receiver-type-required |
| `open` | `parent_metadata.is_dir` | [31](../../src/stdio_channels.rs#L31) | receiver-type-required |
| `open` | `parent_metadata.uid` | [32](../../src/stdio_channels.rs#L32) | receiver-type-required |
| `open` | `parent_metadata.mode` | [33](../../src/stdio_channels.rs#L33) | receiver-type-required |
| `open` | `metadata.file_type().is_socket` | [34](../../src/stdio_channels.rs#L34) | receiver-type-required |
| `open` | `metadata.file_type` | [34](../../src/stdio_channels.rs#L34) | receiver-type-required |
| `open` | `metadata.uid` | [35](../../src/stdio_channels.rs#L35) | receiver-type-required |
| `open` | `metadata.mode` | [36](../../src/stdio_channels.rs#L36) | receiver-type-required |
| `open` | `Err` | [38](../../src/stdio_channels.rs#L38) | external-constructor-callback-or-unresolved |
| `open` | `tokio::net::UnixStream::connect(path).await?.into_std` | [40](../../src/stdio_channels.rs#L40) | receiver-type-required |
| `open` | `tokio::net::UnixStream::connect` | [40](../../src/stdio_channels.rs#L40) | external-constructor-callback-or-unresolved |
| `open` | `Ok` | [41](../../src/stdio_channels.rs#L41) | external-constructor-callback-or-unresolved |
| `open` | `File::from` | [42](../../src/stdio_channels.rs#L42), [43](../../src/stdio_channels.rs#L43) | external-constructor-callback-or-unresolved |
| `open` | `std::os::fd::OwnedFd::from` | [42](../../src/stdio_channels.rs#L42), [43](../../src/stdio_channels.rs#L43) | external-constructor-callback-or-unresolved |
| `open` | `stream.try_clone` | [42](../../src/stdio_channels.rs#L42) | receiver-type-required |
