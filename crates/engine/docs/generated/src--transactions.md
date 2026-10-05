# engine::transactions

[Package atlas](index.md) · [Source](../../src/transactions.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [engine::transactions::DeliveryPhase](../../src/transactions.rs#L4) | enum_item | `pub` |  |
| [engine::transactions::DeliveryCommit](../../src/transactions.rs#L12) | struct_item | `pub` |  |
| [engine::transactions::DeliveryCommit::new](../../src/transactions.rs#L19) | function_item | `pub` |  |
| [engine::transactions::DeliveryCommit::appended](../../src/transactions.rs#L26) | function_item | `pub` |  |
| [engine::transactions::DeliveryCommit::durable](../../src/transactions.rs#L34) | function_item | `pub` |  |
| [engine::transactions::DeliveryCommit::acknowledge](../../src/transactions.rs#L42) | function_item | `pub` |  |
| [engine::transactions::OutcomePhase](../../src/transactions.rs#L52) | enum_item | `pub` |  |
| [engine::transactions::AttemptPhase](../../src/transactions.rs#L60) | enum_item | `pub` |  |
| [engine::transactions::AttemptFlow](../../src/transactions.rs#L70) | struct_item | `pub` |  |
| [engine::transactions::AttemptFlow::new](../../src/transactions.rs#L78) | function_item | `pub` |  |
| [engine::transactions::AttemptFlow::lease_granted](../../src/transactions.rs#L86) | function_item | `pub` |  |
| [engine::transactions::AttemptFlow::attempt_durable](../../src/transactions.rs#L94) | function_item | `pub` |  |
| [engine::transactions::AttemptFlow::dispatch_durable](../../src/transactions.rs#L102) | function_item | `pub` |  |
| [engine::transactions::AttemptFlow::begin_http](../../src/transactions.rs#L113) | function_item | `pub` |  |
| [engine::transactions::AttemptFlow::terminal_received](../../src/transactions.rs#L126) | function_item | `pub` |  |
| [engine::transactions::OutcomeCommit](../../src/transactions.rs#L136) | struct_item | `pub` |  |
| [engine::transactions::OutcomeCommit::new](../../src/transactions.rs#L143) | function_item | `pub` |  |
| [engine::transactions::OutcomeCommit::outcome_appended](../../src/transactions.rs#L152) | function_item | `pub` |  |
| [engine::transactions::OutcomeCommit::durable](../../src/transactions.rs#L160) | function_item | `pub` |  |
| [engine::transactions::OutcomeCommit::release_lease](../../src/transactions.rs#L168) | function_item | `pub` |  |
| [engine::transactions::TransactionError](../../src/transactions.rs#L178) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–14: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::transactions::AttemptFlow::dispatch_durable"]
  n1["engine::transactions::AttemptFlow::begin_http"]
  n2["engine::transactions::AttemptFlow::terminal_received"]
  n3["engine::transactions::OutcomeCommit::new"]
  n4["engine::transactions::OutcomeCommit::outcome_appended"]
  n5["engine::transactions::OutcomeCommit::durable"]
  n6["engine::transactions::OutcomeCommit::release_lease"]
  n7["engine::transactions::DeliveryCommit::new"]
  n8["engine::transactions::DeliveryCommit::appended"]
  n9["engine::transactions::DeliveryCommit::durable"]
  n10["engine::transactions::DeliveryCommit::acknowledge"]
  n11["engine::transactions::AttemptFlow::new"]
  n12["engine::transactions::AttemptFlow::lease_granted"]
  n13["engine::transactions::AttemptFlow::attempt_durable"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `delivery.into` | [21](../../src/transactions.rs#L21) | receiver-type-required |
| `appended` | `Err` | [28](../../src/transactions.rs#L28) | external-constructor-callback-or-unresolved |
| `appended` | `TransactionError::Order` | [28](../../src/transactions.rs#L28) | external-constructor-callback-or-unresolved |
| `appended` | `Ok` | [31](../../src/transactions.rs#L31) | external-constructor-callback-or-unresolved |
| `durable` | `Err` | [36](../../src/transactions.rs#L36) | external-constructor-callback-or-unresolved |
| `durable` | `TransactionError::Order` | [36](../../src/transactions.rs#L36) | external-constructor-callback-or-unresolved |
| `durable` | `Ok` | [39](../../src/transactions.rs#L39) | external-constructor-callback-or-unresolved |
| `acknowledge` | `Err` | [44](../../src/transactions.rs#L44) | external-constructor-callback-or-unresolved |
| `acknowledge` | `Ok` | [47](../../src/transactions.rs#L47) | external-constructor-callback-or-unresolved |
| `new` | `attempt.into` | [80](../../src/transactions.rs#L80) | receiver-type-required |
| `lease_granted` | `Err` | [88](../../src/transactions.rs#L88) | external-constructor-callback-or-unresolved |
| `lease_granted` | `TransactionError::Order` | [88](../../src/transactions.rs#L88) | external-constructor-callback-or-unresolved |
| `lease_granted` | `Ok` | [91](../../src/transactions.rs#L91) | external-constructor-callback-or-unresolved |
| `attempt_durable` | `Err` | [96](../../src/transactions.rs#L96) | external-constructor-callback-or-unresolved |
| `attempt_durable` | `TransactionError::Order` | [96](../../src/transactions.rs#L96) | external-constructor-callback-or-unresolved |
| `attempt_durable` | `Ok` | [99](../../src/transactions.rs#L99) | external-constructor-callback-or-unresolved |
| `dispatch_durable` | `Err` | [104](../../src/transactions.rs#L104) | external-constructor-callback-or-unresolved |
| `dispatch_durable` | `TransactionError::Order` | [104](../../src/transactions.rs#L104) | external-constructor-callback-or-unresolved |
| `dispatch_durable` | `Ok` | [110](../../src/transactions.rs#L110) | external-constructor-callback-or-unresolved |
| `begin_http` | `Err` | [120](../../src/transactions.rs#L120) | external-constructor-callback-or-unresolved |
| `begin_http` | `Ok` | [123](../../src/transactions.rs#L123) | external-constructor-callback-or-unresolved |
| `terminal_received` | `Err` | [128](../../src/transactions.rs#L128) | external-constructor-callback-or-unresolved |
| `terminal_received` | `TransactionError::Order` | [128](../../src/transactions.rs#L128) | external-constructor-callback-or-unresolved |
| `terminal_received` | `Ok` | [131](../../src/transactions.rs#L131) | external-constructor-callback-or-unresolved |
| `new` | `attempt.into` | [145](../../src/transactions.rs#L145) | receiver-type-required |
| `outcome_appended` | `Err` | [154](../../src/transactions.rs#L154) | external-constructor-callback-or-unresolved |
| `outcome_appended` | `TransactionError::Order` | [154](../../src/transactions.rs#L154) | external-constructor-callback-or-unresolved |
| `outcome_appended` | `Ok` | [157](../../src/transactions.rs#L157) | external-constructor-callback-or-unresolved |
| `durable` | `Err` | [162](../../src/transactions.rs#L162) | external-constructor-callback-or-unresolved |
| `durable` | `TransactionError::Order` | [162](../../src/transactions.rs#L162) | external-constructor-callback-or-unresolved |
| `durable` | `Ok` | [165](../../src/transactions.rs#L165) | external-constructor-callback-or-unresolved |
| `release_lease` | `Err` | [170](../../src/transactions.rs#L170) | external-constructor-callback-or-unresolved |
| `release_lease` | `Ok` | [173](../../src/transactions.rs#L173) | external-constructor-callback-or-unresolved |
