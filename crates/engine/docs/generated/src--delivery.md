# engine::delivery

[Package atlas](index.md) · [Source](../../src/delivery.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [engine::delivery::OriginIdentity](../../src/delivery.rs#L6) | struct_item | `private` |  |
| [engine::delivery::OriginIdentity::from](../../src/delivery.rs#L15) | function_item | `private` |  |
| [engine::delivery::DeliveryResolution](../../src/delivery.rs#L27) | enum_item | `pub` |  |
| [engine::delivery::DeliveryIndex](../../src/delivery.rs#L33) | struct_item | `pub` |  |
| [engine::delivery::DeliveryIndex::resolve](../../src/delivery.rs#L39) | function_item | `pub` |  |
| [engine::delivery::DeliveryIndex::commit](../../src/delivery.rs#L47) | function_item | `pub` |  |
| [engine::delivery::CreateResolution](../../src/delivery.rs#L55) | enum_item | `pub` |  |
| [engine::delivery::CreateIndex](../../src/delivery.rs#L61) | struct_item | `pub` |  |
| [engine::delivery::CreateIndex::resolve](../../src/delivery.rs#L66) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `HashMap` | `std::collections::HashMap` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::delivery::OriginIdentity::from"]
  n1["engine::delivery::DeliveryIndex::resolve"]
  n2["engine::delivery::DeliveryIndex::commit"]
  n3["engine::delivery::CreateIndex::resolve"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `from` | `value.principal.clone` | [17](../../src/delivery.rs#L17) | receiver-type-required |
| `from` | `value.client.clone` | [18](../../src/delivery.rs#L18) | receiver-type-required |
| `from` | `value.target.clone` | [19](../../src/delivery.rs#L19) | receiver-type-required |
| `from` | `value.op.clone` | [20](../../src/delivery.rs#L20) | receiver-type-required |
| `from` | `value.key.clone` | [21](../../src/delivery.rs#L21) | receiver-type-required |
| `resolve` | `self.committed             .get(&OriginIdentity::from(origin))             .map_or` | [40](../../src/delivery.rs#L40) | receiver-type-required |
| `resolve` | `self.committed             .get` | [40](../../src/delivery.rs#L40) | receiver-type-required |
| `resolve` | `OriginIdentity::from` | [41](../../src/delivery.rs#L41) | external-constructor-callback-or-unresolved |
| `resolve` | `DeliveryResolution::AppendAt` | [42](../../src/delivery.rs#L42) | external-constructor-callback-or-unresolved |
| `commit` | `self.committed             .entry(OriginIdentity::from(origin))             .or_insert` | [48](../../src/delivery.rs#L48) | receiver-type-required |
| `commit` | `self.committed             .entry` | [48](../../src/delivery.rs#L48) | receiver-type-required |
| `commit` | `OriginIdentity::from` | [49](../../src/delivery.rs#L49) | external-constructor-callback-or-unresolved |
| `resolve` | `OriginIdentity::from` | [71](../../src/delivery.rs#L71) | external-constructor-callback-or-unresolved |
| `resolve` | `self.created.get` | [72](../../src/delivery.rs#L72) | receiver-type-required |
| `resolve` | `thread_id.clone` | [74](../../src/delivery.rs#L74), [78](../../src/delivery.rs#L78) | receiver-type-required |
| `resolve` | `proposed.into` | [77](../../src/delivery.rs#L77) | receiver-type-required |
| `resolve` | `self.created.insert` | [78](../../src/delivery.rs#L78) | receiver-type-required |
