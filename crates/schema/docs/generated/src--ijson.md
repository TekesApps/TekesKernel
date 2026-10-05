# schema::ijson

[Package atlas](index.md) · [Source](../../src/ijson.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [schema::ijson::IJsonValue](../../src/ijson.rs#L13) | struct_item | `pub` |  |
| [schema::ijson::IJsonValue::parse](../../src/ijson.rs#L16) | function_item | `pub` |  |
| [schema::ijson::IJsonValue::parse_str](../../src/ijson.rs#L23) | function_item | `pub` |  |
| [schema::ijson::IJsonValue::canonical_bytes](../../src/ijson.rs#L27) | function_item | `pub` |  |
| [schema::ijson::IJsonValue::canonical_string](../../src/ijson.rs#L32) | function_item | `pub` |  |
| [schema::ijson::IJsonValue::is_null](../../src/ijson.rs#L38) | function_item | `pub` |  |
| [schema::ijson::IJsonValue::value](../../src/ijson.rs#L42) | function_item | `pub(crate)` |  |
| [schema::ijson::IJsonValue::fmt](../../src/ijson.rs#L48) | function_item | `private` |  |
| [schema::ijson::IJsonValue::serialize](../../src/ijson.rs#L54) | function_item | `private` |  |
| [schema::ijson::IJsonValue::deserialize](../../src/ijson.rs#L63) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor](../../src/ijson.rs#L71) | struct_item | `private` |  |
| [schema::ijson::Value](../../src/ijson.rs#L74) | type_item | `private` |  |
| [schema::ijson::IJsonVisitor::expecting](../../src/ijson.rs#L76) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_bool](../../src/ijson.rs#L80) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_i64](../../src/ijson.rs#L84) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_u64](../../src/ijson.rs#L91) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_f64](../../src/ijson.rs#L98) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_str](../../src/ijson.rs#L107) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_string](../../src/ijson.rs#L114) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_none](../../src/ijson.rs#L118) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_unit](../../src/ijson.rs#L122) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_seq](../../src/ijson.rs#L126) | function_item | `private` |  |
| [schema::ijson::IJsonVisitor::visit_map](../../src/ijson.rs#L137) | function_item | `private` |  |
| [schema::ijson::IJsonValue::from](../../src/ijson.rs#L156) | function_item | `private` |  |
| [schema::ijson::IJsonValue::from](../../src/ijson.rs#L162) | function_item | `private` |  |
| [schema::ijson::IJsonValue::from](../../src/ijson.rs#L168) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `fmt` | `std::fmt` | `private` |
| `MapAccess` | `serde::de::MapAccess` | `private` |
| `SeqAccess` | `serde::de::SeqAccess` | `private` |
| `Visitor` | `serde::de::Visitor` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Deserializer` | `serde::Deserializer` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Serializer` | `serde::Serializer` | `private` |
| `Map` | `serde_json::Map` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `SchemaError` | `crate::SchemaError` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonVisitor::visit_str"]
  n1["schema::ijson::IJsonVisitor::visit_string"]
  n2["schema::ijson::IJsonVisitor::visit_none"]
  n3["schema::ijson::IJsonVisitor::visit_unit"]
  n4["schema::ijson::IJsonVisitor::visit_seq"]
  n5["schema::ijson::IJsonVisitor::visit_map"]
  n6["schema::ijson::IJsonValue::parse"]
  n7["schema::ijson::IJsonValue::parse_str"]
  n8["schema::ijson::IJsonValue::canonical_bytes"]
  n9["schema::ijson::IJsonValue::canonical_string"]
  n10["schema::ijson::IJsonValue::is_null"]
  n11["schema::ijson::IJsonValue::value"]
  n12["schema::ijson::IJsonValue::fmt"]
  n13["schema::ijson::IJsonValue::serialize"]
  n14["schema::ijson::IJsonValue::deserialize"]
  n15["schema::ijson::IJsonVisitor::expecting"]
  n16["schema::ijson::IJsonVisitor::visit_bool"]
  n17["schema::ijson::IJsonVisitor::visit_i64"]
  n18["schema::ijson::IJsonVisitor::visit_u64"]
  n19["schema::ijson::IJsonVisitor::visit_f64"]
  n7 --> n6
  n9 --> n8
```

</details>

<details><summary>Functions 21–23: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::from"]
  n1["schema::ijson::IJsonValue::from"]
  n2["schema::ijson::IJsonValue::from"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `parse` | `serde_json::Deserializer::from_slice` | [17](../../src/ijson.rs#L17) | external-constructor-callback-or-unresolved |
| `parse` | `Self::deserialize` | [18](../../src/ijson.rs#L18) | external-constructor-callback-or-unresolved |
| `parse` | `deserializer.end` | [19](../../src/ijson.rs#L19) | receiver-type-required |
| `parse` | `Ok` | [20](../../src/ijson.rs#L20) | external-constructor-callback-or-unresolved |
| `parse_str` | `Self::parse` | [24](../../src/ijson.rs#L24) | [schema::ijson::IJsonValue::parse](../../src/ijson.rs#L16) |
| `parse_str` | `text.as_bytes` | [24](../../src/ijson.rs#L24) | receiver-type-required |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec(&self.0)             .map_err` | [28](../../src/ijson.rs#L28) | receiver-type-required |
| `canonical_bytes` | `serde_json_canonicalizer::to_vec` | [28](../../src/ijson.rs#L28) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `SchemaError::Canonical` | [29](../../src/ijson.rs#L29) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `error.to_string` | [29](../../src/ijson.rs#L29) | receiver-type-required |
| `canonical_string` | `self.canonical_bytes` | [33](../../src/ijson.rs#L33) | [schema::ijson::IJsonValue::canonical_bytes](../../src/ijson.rs#L27) |
| `canonical_string` | `String::from_utf8(bytes).map_err` | [34](../../src/ijson.rs#L34) | receiver-type-required |
| `canonical_string` | `String::from_utf8` | [34](../../src/ijson.rs#L34) | external-constructor-callback-or-unresolved |
| `canonical_string` | `SchemaError::Canonical` | [34](../../src/ijson.rs#L34) | external-constructor-callback-or-unresolved |
| `canonical_string` | `error.to_string` | [34](../../src/ijson.rs#L34) | receiver-type-required |
| `is_null` | `self.0.is_null` | [39](../../src/ijson.rs#L39) | receiver-type-required |
| `fmt` | `self.0.fmt` | [49](../../src/ijson.rs#L49) | receiver-type-required |
| `serialize` | `self.0.serialize` | [58](../../src/ijson.rs#L58) | receiver-type-required |
| `deserialize` | `deserializer.deserialize_any(IJsonVisitor).map` | [67](../../src/ijson.rs#L67) | receiver-type-required |
| `deserialize` | `deserializer.deserialize_any` | [67](../../src/ijson.rs#L67) | receiver-type-required |
| `expecting` | `formatter.write_str` | [77](../../src/ijson.rs#L77) | receiver-type-required |
| `visit_bool` | `Ok` | [81](../../src/ijson.rs#L81) | external-constructor-callback-or-unresolved |
| `visit_bool` | `Value::Bool` | [81](../../src/ijson.rs#L81) | external-constructor-callback-or-unresolved |
| `visit_i64` | `Ok` | [88](../../src/ijson.rs#L88) | external-constructor-callback-or-unresolved |
| `visit_i64` | `Value::Number` | [88](../../src/ijson.rs#L88) | external-constructor-callback-or-unresolved |
| `visit_i64` | `value.into` | [88](../../src/ijson.rs#L88) | receiver-type-required |
| `visit_u64` | `Ok` | [95](../../src/ijson.rs#L95) | external-constructor-callback-or-unresolved |
| `visit_u64` | `Value::Number` | [95](../../src/ijson.rs#L95) | external-constructor-callback-or-unresolved |
| `visit_u64` | `value.into` | [95](../../src/ijson.rs#L95) | receiver-type-required |
| `visit_f64` | `serde_json::Number::from_f64(value)             .map(Value::Number)             .ok_or_else` | [102](../../src/ijson.rs#L102) | receiver-type-required |
| `visit_f64` | `serde_json::Number::from_f64(value)             .map` | [102](../../src/ijson.rs#L102) | receiver-type-required |
| `visit_f64` | `serde_json::Number::from_f64` | [102](../../src/ijson.rs#L102) | external-constructor-callback-or-unresolved |
| `visit_f64` | `E::custom` | [104](../../src/ijson.rs#L104) | external-constructor-callback-or-unresolved |
| `visit_str` | `Ok` | [111](../../src/ijson.rs#L111) | external-constructor-callback-or-unresolved |
| `visit_str` | `Value::String` | [111](../../src/ijson.rs#L111) | external-constructor-callback-or-unresolved |
| `visit_str` | `value.to_owned` | [111](../../src/ijson.rs#L111) | receiver-type-required |
| `visit_string` | `Ok` | [115](../../src/ijson.rs#L115) | external-constructor-callback-or-unresolved |
| `visit_string` | `Value::String` | [115](../../src/ijson.rs#L115) | external-constructor-callback-or-unresolved |
| `visit_none` | `Ok` | [119](../../src/ijson.rs#L119) | external-constructor-callback-or-unresolved |
| `visit_unit` | `Ok` | [123](../../src/ijson.rs#L123) | external-constructor-callback-or-unresolved |
| `visit_seq` | `Vec::new` | [130](../../src/ijson.rs#L130) | external-constructor-callback-or-unresolved |
| `visit_seq` | `sequence.next_element::<IJsonValue>` | [131](../../src/ijson.rs#L131) | receiver-type-required |
| `visit_seq` | `values.push` | [132](../../src/ijson.rs#L132) | receiver-type-required |
| `visit_seq` | `Ok` | [134](../../src/ijson.rs#L134) | external-constructor-callback-or-unresolved |
| `visit_seq` | `Value::Array` | [134](../../src/ijson.rs#L134) | external-constructor-callback-or-unresolved |
| `visit_map` | `Map::new` | [141](../../src/ijson.rs#L141) | external-constructor-callback-or-unresolved |
| `visit_map` | `map.next_key::<String>` | [142](../../src/ijson.rs#L142) | receiver-type-required |
| `visit_map` | `values.contains_key` | [143](../../src/ijson.rs#L143) | receiver-type-required |
| `visit_map` | `Err` | [144](../../src/ijson.rs#L144) | external-constructor-callback-or-unresolved |
| `visit_map` | `serde::de::Error::custom` | [144](../../src/ijson.rs#L144) | external-constructor-callback-or-unresolved |
| `visit_map` | `map.next_value::<IJsonValue>` | [148](../../src/ijson.rs#L148) | receiver-type-required |
| `visit_map` | `values.insert` | [149](../../src/ijson.rs#L149) | receiver-type-required |
| `visit_map` | `Ok` | [151](../../src/ijson.rs#L151) | external-constructor-callback-or-unresolved |
| `visit_map` | `Value::Object` | [151](../../src/ijson.rs#L151) | external-constructor-callback-or-unresolved |
| `from` | `Self` | [157](../../src/ijson.rs#L157) | external-constructor-callback-or-unresolved |
| `from` | `Value::Bool` | [157](../../src/ijson.rs#L157) | external-constructor-callback-or-unresolved |
| `from` | `Self` | [163](../../src/ijson.rs#L163) | external-constructor-callback-or-unresolved |
| `from` | `Value::String` | [163](../../src/ijson.rs#L163) | external-constructor-callback-or-unresolved |
| `from` | `Self` | [169](../../src/ijson.rs#L169) | external-constructor-callback-or-unresolved |
| `from` | `Value::String` | [169](../../src/ijson.rs#L169) | external-constructor-callback-or-unresolved |
| `from` | `value.to_owned` | [169](../../src/ijson.rs#L169) | receiver-type-required |
