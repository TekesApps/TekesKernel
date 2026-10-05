# engine::seams

[Package atlas](index.md) · [Source](../../src/seams.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [engine::seams::ProviderTerminal](../../src/seams.rs#L9) | struct_item | `pub` |  |
| [engine::seams::ProviderAdapter](../../src/seams.rs#L16) | trait_item | `pub` |  |
| [engine::seams::ProviderAdapter::capabilities](../../src/seams.rs#L17) | function_signature_item | `private` |  |
| [engine::seams::ProviderAdapter::render](../../src/seams.rs#L18) | function_signature_item | `private` |  |
| [engine::seams::ProviderAdapter::query](../../src/seams.rs#L19) | function_signature_item | `private` |  |
| [engine::seams::ProviderAdapter::adopted_response](../../src/seams.rs#L20) | function_signature_item | `private` |  |
| [engine::seams::FakeProvider](../../src/seams.rs#L24) | struct_item | `pub` |  |
| [engine::seams::FakeProvider::new](../../src/seams.rs#L32) | function_item | `pub` |  |
| [engine::seams::FakeProvider::push_query_result](../../src/seams.rs#L40) | function_item | `pub` |  |
| [engine::seams::FakeProvider::insert_response](../../src/seams.rs#L44) | function_item | `pub` |  |
| [engine::seams::FakeProvider::capabilities](../../src/seams.rs#L50) | function_item | `private` |  |
| [engine::seams::FakeProvider::render](../../src/seams.rs#L54) | function_item | `private` |  |
| [engine::seams::FakeProvider::query](../../src/seams.rs#L58) | function_item | `private` |  |
| [engine::seams::FakeProvider::adopted_response](../../src/seams.rs#L64) | function_item | `private` |  |
| [engine::seams::ToolBackend](../../src/seams.rs#L69) | trait_item | `pub` |  |
| [engine::seams::ToolBackend::supports](../../src/seams.rs#L70) | function_item | `private` |  |
| [engine::seams::ToolBackend::execute](../../src/seams.rs#L74) | function_signature_item | `private` |  |
| [engine::seams::ToolBackend::resume_after_approval](../../src/seams.rs#L82) | function_item | `private` |  |
| [engine::seams::ToolBackendRouter](../../src/seams.rs#L93) | struct_item | `pub` |  |
| [engine::seams::ToolBackendRouter::new](../../src/seams.rs#L99) | function_item | `pub` |  |
| [engine::seams::ToolBackendRouter::push](../../src/seams.rs#L103) | function_item | `pub` |  |
| [engine::seams::ToolBackendRouter::supports](../../src/seams.rs#L109) | function_item | `private` |  |
| [engine::seams::ToolBackendRouter::execute](../../src/seams.rs#L113) | function_item | `private` |  |
| [engine::seams::ToolBackendRouter::resume_after_approval](../../src/seams.rs#L128) | function_item | `private` |  |
| [engine::seams::FakeToolBackend](../../src/seams.rs#L150) | struct_item | `pub` |  |
| [engine::seams::FakeToolBackend::set_outcome](../../src/seams.rs#L156) | function_item | `pub` |  |
| [engine::seams::FakeToolBackend::set_terminal](../../src/seams.rs#L170) | function_item | `pub` |  |
| [engine::seams::FakeToolBackend::execution_count](../../src/seams.rs#L175) | function_item | `pub` |  |
| [engine::seams::FakeToolBackend::execute](../../src/seams.rs#L181) | function_item | `private` |  |
| [engine::seams::ProcessState](../../src/seams.rs#L195) | enum_item | `pub` |  |
| [engine::seams::ProcessHost](../../src/seams.rs#L201) | trait_item | `pub` |  |
| [engine::seams::ProcessHost::spawn](../../src/seams.rs#L202) | function_signature_item | `private` |  |
| [engine::seams::ProcessHost::signal](../../src/seams.rs#L203) | function_signature_item | `private` |  |
| [engine::seams::ProcessHost::wait](../../src/seams.rs#L204) | function_signature_item | `private` |  |
| [engine::seams::ProcessHost::state](../../src/seams.rs#L205) | function_signature_item | `private` |  |
| [engine::seams::FakeProcessHost](../../src/seams.rs#L209) | struct_item | `pub` |  |
| [engine::seams::FakeProcessHost::exit](../../src/seams.rs#L214) | function_item | `pub` |  |
| [engine::seams::FakeProcessHost::spawn](../../src/seams.rs#L221) | function_item | `private` |  |
| [engine::seams::FakeProcessHost::signal](../../src/seams.rs#L228) | function_item | `private` |  |
| [engine::seams::FakeProcessHost::wait](../../src/seams.rs#L239) | function_item | `private` |  |
| [engine::seams::FakeProcessHost::state](../../src/seams.rs#L249) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `HashMap` | `std::collections::HashMap` | `private` |
| `VecDeque` | `std::collections::VecDeque` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `BackendTerminal` | `tools::BackendTerminal` | `private` |
| `DurableApprovalResponse` | `tools::DurableApprovalResponse` | `private` |
| `ToolExecution` | `tools::ToolExecution` | `private` |
| `AdapterCapabilities` | `crate::AdapterCapabilities` | `private` |
| `QueryResult` | `crate::QueryResult` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::seams::ToolBackendRouter::push"]
  n1["engine::seams::ToolBackendRouter::supports"]
  n2["engine::seams::ToolBackendRouter::execute"]
  n3["engine::seams::ToolBackendRouter::resume_after_approval"]
  n4["engine::seams::FakeToolBackend::set_outcome"]
  n5["engine::seams::FakeToolBackend::set_terminal"]
  n6["engine::seams::FakeToolBackend::execution_count"]
  n7["engine::seams::FakeToolBackend::execute"]
  n8["engine::seams::FakeProcessHost::exit"]
  n9["engine::seams::FakeProcessHost::spawn"]
  n10["engine::seams::FakeProvider::new"]
  n11["engine::seams::FakeProvider::push_query_result"]
  n12["engine::seams::FakeProvider::insert_response"]
  n13["engine::seams::FakeProvider::capabilities"]
  n14["engine::seams::FakeProvider::render"]
  n15["engine::seams::FakeProvider::query"]
  n16["engine::seams::FakeProvider::adopted_response"]
  n17["engine::seams::ToolBackend::supports"]
  n18["engine::seams::ToolBackend::resume_after_approval"]
  n19["engine::seams::ToolBackendRouter::new"]
```

</details>

<details><summary>Functions 21–23: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::seams::FakeProcessHost::signal"]
  n1["engine::seams::FakeProcessHost::wait"]
  n2["engine::seams::FakeProcessHost::state"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `new` | `VecDeque::new` | [35](../../src/seams.rs#L35) | external-constructor-callback-or-unresolved |
| `new` | `HashMap::new` | [36](../../src/seams.rs#L36) | external-constructor-callback-or-unresolved |
| `push_query_result` | `self.query_results.push_back` | [41](../../src/seams.rs#L41) | receiver-type-required |
| `insert_response` | `self.responses.insert` | [45](../../src/seams.rs#L45) | receiver-type-required |
| `insert_response` | `attempt.into` | [45](../../src/seams.rs#L45) | receiver-type-required |
| `render` | `format!("prefix={prefix_digest};epoch={epoch_profile}").into_bytes` | [55](../../src/seams.rs#L55) | receiver-type-required |
| `query` | `self.query_results             .pop_front()             .unwrap_or` | [59](../../src/seams.rs#L59) | receiver-type-required |
| `query` | `self.query_results             .pop_front` | [59](../../src/seams.rs#L59) | receiver-type-required |
| `adopted_response` | `self.responses.get` | [65](../../src/seams.rs#L65) | receiver-type-required |
| `resume_after_approval` | `self.execute` | [88](../../src/seams.rs#L88) | receiver-type-required |
| `push` | `self.backends.push` | [104](../../src/seams.rs#L104) | receiver-type-required |
| `push` | `Box::new` | [104](../../src/seams.rs#L104) | external-constructor-callback-or-unresolved |
| `supports` | `self.backends.iter().any` | [110](../../src/seams.rs#L110) | receiver-type-required |
| `supports` | `self.backends.iter` | [110](../../src/seams.rs#L110) | receiver-type-required |
| `supports` | `backend.supports` | [110](../../src/seams.rs#L110) | receiver-type-required |
| `execute` | `self             .backends             .iter_mut()             .find` | [114](../../src/seams.rs#L114) | receiver-type-required |
| `execute` | `self             .backends             .iter_mut` | [114](../../src/seams.rs#L114) | receiver-type-required |
| `execute` | `backend.supports` | [117](../../src/seams.rs#L117) | receiver-type-required |
| `execute` | `"unsupported".to_owned` | [120](../../src/seams.rs#L120) | receiver-type-required |
| `execute` | `backend.execute` | [125](../../src/seams.rs#L125) | receiver-type-required |
| `resume_after_approval` | `self             .backends             .iter_mut()             .find` | [134](../../src/seams.rs#L134) | receiver-type-required |
| `resume_after_approval` | `self             .backends             .iter_mut` | [134](../../src/seams.rs#L134) | receiver-type-required |
| `resume_after_approval` | `backend.supports` | [137](../../src/seams.rs#L137) | receiver-type-required |
| `resume_after_approval` | `"unsupported".to_owned` | [140](../../src/seams.rs#L140) | receiver-type-required |
| `resume_after_approval` | `backend.resume_after_approval` | [145](../../src/seams.rs#L145) | receiver-type-required |
| `set_outcome` | `self.outcomes.insert` | [157](../../src/seams.rs#L157) | receiver-type-required |
| `set_outcome` | `call.into` | [158](../../src/seams.rs#L158) | receiver-type-required |
| `set_outcome` | `BackendTerminal::Completed` | [160](../../src/seams.rs#L160) | external-constructor-callback-or-unresolved |
| `set_outcome` | `"fake_error".to_owned` | [162](../../src/seams.rs#L162) | receiver-type-required |
| `set_terminal` | `self.outcomes.insert` | [171](../../src/seams.rs#L171) | receiver-type-required |
| `set_terminal` | `call.into` | [171](../../src/seams.rs#L171) | receiver-type-required |
| `execution_count` | `self.executions.get(call).copied().unwrap_or` | [176](../../src/seams.rs#L176) | receiver-type-required |
| `execution_count` | `self.executions.get(call).copied` | [176](../../src/seams.rs#L176) | receiver-type-required |
| `execution_count` | `self.executions.get` | [176](../../src/seams.rs#L176) | receiver-type-required |
| `execute` | `self.executions.entry(execution.call.clone()).or_default` | [182](../../src/seams.rs#L182) | receiver-type-required |
| `execute` | `self.executions.entry` | [182](../../src/seams.rs#L182) | receiver-type-required |
| `execute` | `execution.call.clone` | [182](../../src/seams.rs#L182) | receiver-type-required |
| `execute` | `self.outcomes             .get(&execution.call)             .cloned()             .unwrap_or_else` | [183](../../src/seams.rs#L183) | receiver-type-required |
| `execute` | `self.outcomes             .get(&execution.call)             .cloned` | [183](../../src/seams.rs#L183) | receiver-type-required |
| `execute` | `self.outcomes             .get` | [183](../../src/seams.rs#L183) | receiver-type-required |
| `execute` | `"unconfigured".to_owned` | [187](../../src/seams.rs#L187) | receiver-type-required |
| `execute` | `"unconfigured fake tool".to_owned` | [188](../../src/seams.rs#L188) | receiver-type-required |
| `exit` | `self.processes             .insert` | [215](../../src/seams.rs#L215) | receiver-type-required |
| `exit` | `key.to_owned` | [216](../../src/seams.rs#L216) | receiver-type-required |
| `exit` | `ProcessState::Exited` | [216](../../src/seams.rs#L216) | external-constructor-callback-or-unresolved |
| `spawn` | `self.processes             .entry(key.to_owned())             .or_insert` | [222](../../src/seams.rs#L222) | receiver-type-required |
| `spawn` | `self.processes             .entry` | [222](../../src/seams.rs#L222) | receiver-type-required |
| `spawn` | `key.to_owned` | [223](../../src/seams.rs#L223) | receiver-type-required |
| `spawn` | `Ok` | [225](../../src/seams.rs#L225) | external-constructor-callback-or-unresolved |
| `signal` | `self.processes.get_mut` | [229](../../src/seams.rs#L229) | receiver-type-required |
| `signal` | `Ok` | [232](../../src/seams.rs#L232), [234](../../src/seams.rs#L234) | external-constructor-callback-or-unresolved |
| `signal` | `Err` | [235](../../src/seams.rs#L235) | external-constructor-callback-or-unresolved |
| `wait` | `self.processes.get` | [240](../../src/seams.rs#L240) | receiver-type-required |
| `wait` | `Ok` | [241](../../src/seams.rs#L241) | external-constructor-callback-or-unresolved |
| `wait` | `Err` | [243](../../src/seams.rs#L243), [245](../../src/seams.rs#L245) | external-constructor-callback-or-unresolved |
| `state` | `self.processes.get(key).copied` | [250](../../src/seams.rs#L250) | receiver-type-required |
| `state` | `self.processes.get` | [250](../../src/seams.rs#L250) | receiver-type-required |
