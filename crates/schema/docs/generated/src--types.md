# schema::types

[Package atlas](index.md) · [Source](../../src/types.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::types::OriginTuple](../../src/types.rs#L11) | struct_item | `pub` |  |
| [schema::types::SeqRange](../../src/types.rs#L20) | struct_item | `pub` |  |
| [schema::types::Block](../../src/types.rs#L27) | enum_item | `pub` |  |
| [schema::types::ResumePolicy](../../src/types.rs#L62) | enum_item | `pub` |  |
| [schema::types::ResumePolicy::permits](../../src/types.rs#L69) | function_item | `pub` |  |
| [schema::types::ResumePolicy::serialize](../../src/types.rs#L78) | function_item | `private` |  |
| [schema::types::ResumePolicy::serialize::Bounded](../../src/types.rs#L86) | struct_item | `private` |  |
| [schema::types::ResumePolicy::deserialize](../../src/types.rs#L96) | function_item | `private` |  |
| [schema::types::ResumePolicy::fmt](../../src/types.rs#L116) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `fmt` | `std::fmt` | `private` |
| `_` | `serde::de::Error` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Deserializer` | `serde::Deserializer` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Serializer` | `serde::Serializer` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `IJsonValue` | `crate::IJsonValue` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::types::ResumePolicy::fmt"]
  n1["schema::types::ResumePolicy::permits"]
  n2["schema::types::ResumePolicy::serialize"]
  n3["schema::types::ResumePolicy::deserialize"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `serialize` | `serializer.serialize_str` | [83](../../src/types.rs#L83) | receiver-type-required |
| `serialize` | `Bounded { bounded: *limit }.serialize` | [89](../../src/types.rs#L89) | receiver-type-required |
| `deserialize` | `Value::deserialize` | [100](../../src/types.rs#L100) | external-constructor-callback-or-unresolved |
| `deserialize` | `Ok` | [102](../../src/types.rs#L102) | external-constructor-callback-or-unresolved |
| `deserialize` | `value.len` | [103](../../src/types.rs#L103) | receiver-type-required |
| `deserialize` | `value                 .get("bounded")                 .and_then(Value::as_u64)                 .map(Self::Bounded)                 .ok_or_else` | [103](../../src/types.rs#L103) | receiver-type-required |
| `deserialize` | `value                 .get("bounded")                 .and_then(Value::as_u64)                 .map` | [103](../../src/types.rs#L103) | receiver-type-required |
| `deserialize` | `value                 .get("bounded")                 .and_then` | [103](../../src/types.rs#L103) | receiver-type-required |
| `deserialize` | `value                 .get` | [103](../../src/types.rs#L103) | receiver-type-required |
| `deserialize` | `D::Error::custom` | [107](../../src/types.rs#L107), [108](../../src/types.rs#L108) | external-constructor-callback-or-unresolved |
| `deserialize` | `Err` | [108](../../src/types.rs#L108) | external-constructor-callback-or-unresolved |
| `fmt` | `formatter.write_str` | [118](../../src/types.rs#L118) | receiver-type-required |
