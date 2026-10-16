# schema::execution

[Package atlas](index.md) · [Source](../../src/execution.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::execution::ExecutionConfig](../../src/execution.rs#L6) | struct_item | `pub` |  |
| [schema::execution::ExecutionConfig::validate](../../src/execution.rs#L25) | function_item | `pub` |  |
| [schema::execution::McpConnection](../../src/execution.rs#L56) | struct_item | `pub` |  |
| [schema::execution::McpConnection::validate](../../src/execution.rs#L61) | function_item | `pub` |  |
| [schema::execution::McpConnectionTransport](../../src/execution.rs#L113) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–2: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::execution::ExecutionConfig::validate"]
  n1["schema::execution::McpConnection::validate"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `validate` | `std::iter::once(&self.cwd)             .chain(&self.read_roots)             .chain(&self.writable_roots)             .chain` | [26](../../src/execution.rs#L26) | receiver-type-required |
| `validate` | `std::iter::once(&self.cwd)             .chain(&self.read_roots)             .chain` | [26](../../src/execution.rs#L26) | receiver-type-required |
| `validate` | `std::iter::once(&self.cwd)             .chain` | [26](../../src/execution.rs#L26) | receiver-type-required |
| `validate` | `std::iter::once` | [26](../../src/execution.rs#L26) | external-constructor-callback-or-unresolved |
| `validate` | `std::path::Path::new(path).is_absolute` | [31](../../src/execution.rs#L31) | receiver-type-required |
| `validate` | `std::path::Path::new` | [31](../../src/execution.rs#L31) | external-constructor-callback-or-unresolved |
| `validate` | `path.contains` | [31](../../src/execution.rs#L31) | receiver-type-required |
| `validate` | `Err` | [32](../../src/execution.rs#L32), [39](../../src/execution.rs#L39), [45](../../src/execution.rs#L45) | external-constructor-callback-or-unresolved |
| `validate` | `self             .max_wall_seconds             .is_some_and` | [35](../../src/execution.rs#L35) | receiver-type-required |
| `validate` | `std::collections::BTreeSet::new` | [41](../../src/execution.rs#L41) | external-constructor-callback-or-unresolved |
| `validate` | `connection.validate` | [43](../../src/execution.rs#L43) | receiver-type-required |
| `validate` | `names.insert` | [44](../../src/execution.rs#L44) | receiver-type-required |
| `validate` | `Ok` | [48](../../src/execution.rs#L48) | external-constructor-callback-or-unresolved |
| `validate` | `self.name.is_empty` | [62](../../src/execution.rs#L62) | receiver-type-required |
| `validate` | `self.name.len` | [63](../../src/execution.rs#L63) | receiver-type-required |
| `validate` | `self                 .name                 .bytes()                 .all` | [64](../../src/execution.rs#L64) | receiver-type-required |
| `validate` | `self                 .name                 .bytes` | [64](../../src/execution.rs#L64) | receiver-type-required |
| `validate` | `b.is_ascii_alphanumeric` | [67](../../src/execution.rs#L67), [101](../../src/execution.rs#L101) | receiver-type-required |
| `validate` | `Err` | [69](../../src/execution.rs#L69), [86](../../src/execution.rs#L86), [93](../../src/execution.rs#L93), [103](../../src/execution.rs#L103) | external-constructor-callback-or-unresolved |
| `validate` | `url::Url::parse(url).map_err` | [73](../../src/execution.rs#L73) | receiver-type-required |
| `validate` | `url::Url::parse` | [73](../../src/execution.rs#L73) | external-constructor-callback-or-unresolved |
| `validate` | `url.host().is_some_and` | [74](../../src/execution.rs#L74) | receiver-type-required |
| `validate` | `url.host` | [74](../../src/execution.rs#L74), [80](../../src/execution.rs#L80) | receiver-type-required |
| `validate` | `address.is_loopback` | [76](../../src/execution.rs#L76), [77](../../src/execution.rs#L77) | receiver-type-required |
| `validate` | `url.scheme` | [79](../../src/execution.rs#L79) | receiver-type-required |
| `validate` | `url.host().is_none` | [80](../../src/execution.rs#L80) | receiver-type-required |
| `validate` | `url.username().is_empty` | [81](../../src/execution.rs#L81) | receiver-type-required |
| `validate` | `url.username` | [81](../../src/execution.rs#L81) | receiver-type-required |
| `validate` | `url.password().is_some` | [82](../../src/execution.rs#L82) | receiver-type-required |
| `validate` | `url.password` | [82](../../src/execution.rs#L82) | receiver-type-required |
| `validate` | `url.fragment().is_some` | [83](../../src/execution.rs#L83) | receiver-type-required |
| `validate` | `url.fragment` | [83](../../src/execution.rs#L83) | receiver-type-required |
| `validate` | `url.query().is_some` | [84](../../src/execution.rs#L84) | receiver-type-required |
| `validate` | `url.query` | [84](../../src/execution.rs#L84) | receiver-type-required |
| `validate` | `credential_id.as_ref().is_some_and` | [90](../../src/execution.rs#L90) | receiver-type-required |
| `validate` | `credential_id.as_ref` | [90](../../src/execution.rs#L90) | receiver-type-required |
| `validate` | `id.is_empty` | [91](../../src/execution.rs#L91) | receiver-type-required |
| `validate` | `id.len` | [91](../../src/execution.rs#L91) | receiver-type-required |
| `validate` | `id.chars().any` | [91](../../src/execution.rs#L91) | receiver-type-required |
| `validate` | `id.chars` | [91](../../src/execution.rs#L91) | receiver-type-required |
| `validate` | `channel.is_empty` | [97](../../src/execution.rs#L97) | receiver-type-required |
| `validate` | `channel.len` | [98](../../src/execution.rs#L98) | receiver-type-required |
| `validate` | `channel                         .bytes()                         .all` | [99](../../src/execution.rs#L99) | receiver-type-required |
| `validate` | `channel                         .bytes` | [99](../../src/execution.rs#L99) | receiver-type-required |
| `validate` | `Ok` | [107](../../src/execution.rs#L107) | external-constructor-callback-or-unresolved |
