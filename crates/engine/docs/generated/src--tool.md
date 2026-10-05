# engine::tool

[Package atlas](index.md) · [Source](../../src/tool.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [engine::tool::execute_tool](../../src/tool.rs#L5) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `ToolBackend` | `crate::ToolBackend` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::tool::execute_tool"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `execute_tool` | `pipeline.execute_terminal` | [11](../../src/tool.rs#L11) | receiver-type-required |
| `execute_tool` | `backend.execute` | [12](../../src/tool.rs#L12) | receiver-type-required |
