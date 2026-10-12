# store::folder_events

[Package atlas](index.md) · [Source](../../src/folder_events.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [store::folder_events::Area](../../src/folder_events.rs#L14) | enum_item | `pub` |  |
| [store::folder_events::RemoveReason](../../src/folder_events.rs#L20) | enum_item | `pub` |  |
| [store::folder_events::FolderEvent](../../src/folder_events.rs#L30) | enum_item | `pub` |  |
| [store::folder_events::FolderEvent::root](../../src/folder_events.rs#L51) | function_item | `pub` |  |
| [store::folder_events::FolderEvent::id](../../src/folder_events.rs#L60) | function_item | `pub` |  |
| [store::folder_events::canonical_root](../../src/folder_events.rs#L71) | function_item | `pub` |  |
| [store::folder_events::canonical_root::MEMO](../../src/folder_events.rs#L74) | static_item | `private` |  |
| [store::folder_events::FolderEvent::with_root](../../src/folder_events.rs#L97) | function_item | `private` |  |
| [store::folder_events::Observer](../../src/folder_events.rs#L116) | type_item | `pub` |  |
| [store::folder_events::Registry](../../src/folder_events.rs#L119) | struct_item | `private` |  |
| [store::folder_events::registry](../../src/folder_events.rs#L125) | function_item | `private` |  |
| [store::folder_events::registry::REGISTRY](../../src/folder_events.rs#L126) | static_item | `private` |  |
| [store::folder_events::lock](../../src/folder_events.rs#L130) | function_item | `private` |  |
| [store::folder_events::subscribe](../../src/folder_events.rs#L137) | function_item | `pub` |  |
| [store::folder_events::arm_admission](../../src/folder_events.rs#L143) | function_item | `pub` |  |
| [store::folder_events::is_admitted](../../src/folder_events.rs#L150) | function_item | `pub` |  |
| [store::folder_events::admitted_ids](../../src/folder_events.rs#L160) | function_item | `pub` |  |
| [store::folder_events::emit](../../src/folder_events.rs#L169) | function_item | `pub(crate)` |  |
| [store::folder_events::record_events](../../src/folder_events.rs#L213) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `HashMap` | `std::collections::HashMap` | `private` |
| `HashSet` | `std::collections::HashSet` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `OnceLock` | `std::sync::OnceLock` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `store::folder_events::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–12: 12 direct edges</summary>

```mermaid
flowchart TD
  n0["store::folder_events::registry"]
  n1["store::folder_events::lock"]
  n2["store::folder_events::subscribe"]
  n3["store::folder_events::arm_admission"]
  n4["store::folder_events::is_admitted"]
  n5["store::folder_events::admitted_ids"]
  n6["store::folder_events::emit"]
  n7["store::folder_events::record_events"]
  n8["store::folder_events::FolderEvent::root"]
  n9["store::folder_events::FolderEvent::id"]
  n10["store::folder_events::canonical_root"]
  n11["store::folder_events::FolderEvent::with_root"]
  n1 --> n0
  n2 --> n1
  n3 --> n1
  n3 --> n10
  n4 --> n1
  n4 --> n10
  n5 --> n1
  n5 --> n10
  n6 --> n1
  n6 --> n10
  n7 --> n2
  n7 --> n10
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `canonical_root` | `MEMO.get_or_init` | [75](../../src/folder_events.rs#L75) | receiver-type-required |
| `canonical_root` | `memo         .lock()         .unwrap_or_else(std::sync::PoisonError::into_inner)         .get` | [76](../../src/folder_events.rs#L76) | receiver-type-required |
| `canonical_root` | `memo         .lock()         .unwrap_or_else` | [76](../../src/folder_events.rs#L76) | receiver-type-required |
| `canonical_root` | `memo         .lock` | [76](../../src/folder_events.rs#L76) | receiver-type-required |
| `canonical_root` | `known.clone` | [81](../../src/folder_events.rs#L81) | receiver-type-required |
| `canonical_root` | `std::fs::canonicalize` | [83](../../src/folder_events.rs#L83) | external-constructor-callback-or-unresolved |
| `canonical_root` | `memo.lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .insert` | [85](../../src/folder_events.rs#L85) | receiver-type-required |
| `canonical_root` | `memo.lock()                 .unwrap_or_else` | [85](../../src/folder_events.rs#L85) | receiver-type-required |
| `canonical_root` | `memo.lock` | [85](../../src/folder_events.rs#L85) | receiver-type-required |
| `canonical_root` | `root.to_path_buf` | [87](../../src/folder_events.rs#L87), [92](../../src/folder_events.rs#L92) | receiver-type-required |
| `canonical_root` | `resolved.clone` | [87](../../src/folder_events.rs#L87) | receiver-type-required |
| `MEMO` | `OnceLock::new` | [74](../../src/folder_events.rs#L74) | external-constructor-callback-or-unresolved |
| `registry` | `REGISTRY.get_or_init` | [127](../../src/folder_events.rs#L127) | receiver-type-required |
| `registry` | `Mutex::new` | [127](../../src/folder_events.rs#L127) | external-constructor-callback-or-unresolved |
| `registry` | `Registry::default` | [127](../../src/folder_events.rs#L127) | external-constructor-callback-or-unresolved |
| `REGISTRY` | `OnceLock::new` | [126](../../src/folder_events.rs#L126) | external-constructor-callback-or-unresolved |
| `lock` | `registry()         .lock()         .unwrap_or_else` | [131](../../src/folder_events.rs#L131) | receiver-type-required |
| `lock` | `registry()         .lock` | [131](../../src/folder_events.rs#L131) | receiver-type-required |
| `lock` | `registry` | [131](../../src/folder_events.rs#L131) | [store::folder_events::registry](../../src/folder_events.rs#L125) |
| `subscribe` | `lock().observers.push` | [138](../../src/folder_events.rs#L138) | receiver-type-required |
| `subscribe` | `lock` | [138](../../src/folder_events.rs#L138) | [store::folder_events::lock](../../src/folder_events.rs#L130) |
| `arm_admission` | `lock().admission.entry(canonical_root(root)).or_default` | [144](../../src/folder_events.rs#L144) | receiver-type-required |
| `arm_admission` | `lock().admission.entry` | [144](../../src/folder_events.rs#L144) | receiver-type-required |
| `arm_admission` | `lock` | [144](../../src/folder_events.rs#L144) | [store::folder_events::lock](../../src/folder_events.rs#L130) |
| `arm_admission` | `canonical_root` | [144](../../src/folder_events.rs#L144) | [store::folder_events::canonical_root](../../src/folder_events.rs#L71) |
| `is_admitted` | `canonical_root` | [151](../../src/folder_events.rs#L151) | [store::folder_events::canonical_root](../../src/folder_events.rs#L71) |
| `is_admitted` | `lock()         .admission         .get(&root)         .is_some_and` | [152](../../src/folder_events.rs#L152) | receiver-type-required |
| `is_admitted` | `lock()         .admission         .get` | [152](../../src/folder_events.rs#L152) | receiver-type-required |
| `is_admitted` | `lock` | [152](../../src/folder_events.rs#L152) | [store::folder_events::lock](../../src/folder_events.rs#L130) |
| `is_admitted` | `admitted.contains` | [155](../../src/folder_events.rs#L155) | receiver-type-required |
| `admitted_ids` | `canonical_root` | [161](../../src/folder_events.rs#L161) | [store::folder_events::canonical_root](../../src/folder_events.rs#L71) |
| `admitted_ids` | `lock()         .admission         .get(&root)         .map(&#124;admitted&#124; admitted.iter().cloned().collect())         .unwrap_or_default` | [162](../../src/folder_events.rs#L162) | receiver-type-required |
| `admitted_ids` | `lock()         .admission         .get(&root)         .map` | [162](../../src/folder_events.rs#L162) | receiver-type-required |
| `admitted_ids` | `lock()         .admission         .get` | [162](../../src/folder_events.rs#L162) | receiver-type-required |
| `admitted_ids` | `lock` | [162](../../src/folder_events.rs#L162) | [store::folder_events::lock](../../src/folder_events.rs#L130) |
| `admitted_ids` | `admitted.iter().cloned().collect` | [165](../../src/folder_events.rs#L165) | receiver-type-required |
| `admitted_ids` | `admitted.iter().cloned` | [165](../../src/folder_events.rs#L165) | receiver-type-required |
| `admitted_ids` | `admitted.iter` | [165](../../src/folder_events.rs#L165) | receiver-type-required |
| `emit` | `canonical_root` | [171](../../src/folder_events.rs#L171) | [store::folder_events::canonical_root](../../src/folder_events.rs#L71) |
| `emit` | `event.root` | [171](../../src/folder_events.rs#L171), [176](../../src/folder_events.rs#L176) | receiver-type-required |
| `emit` | `event.with_root` | [172](../../src/folder_events.rs#L172) | receiver-type-required |
| `emit` | `lock` | [175](../../src/folder_events.rs#L175), [205](../../src/folder_events.rs#L205) | [store::folder_events::lock](../../src/folder_events.rs#L130) |
| `emit` | `registry.admission.get_mut` | [176](../../src/folder_events.rs#L176) | receiver-type-required |
| `emit` | `admitted.insert` | [184](../../src/folder_events.rs#L184) | receiver-type-required |
| `emit` | `id.clone` | [184](../../src/folder_events.rs#L184) | receiver-type-required |
| `emit` | `admitted.remove` | [189](../../src/folder_events.rs#L189) | receiver-type-required |
| `emit` | `registry.observers.clone` | [196](../../src/folder_events.rs#L196) | receiver-type-required |
| `emit` | `observers         .iter()         .filter(&#124;observer&#124; !observer(&event))         .cloned()         .collect::<Vec<_>>` | [199](../../src/folder_events.rs#L199) | receiver-type-required |
| `emit` | `observers         .iter()         .filter(&#124;observer&#124; !observer(&event))         .cloned` | [199](../../src/folder_events.rs#L199) | receiver-type-required |
| `emit` | `observers         .iter()         .filter` | [199](../../src/folder_events.rs#L199) | receiver-type-required |
| `emit` | `observers         .iter` | [199](../../src/folder_events.rs#L199) | receiver-type-required |
| `emit` | `observer` | [201](../../src/folder_events.rs#L201) | external-constructor-callback-or-unresolved |
| `emit` | `gone.is_empty` | [204](../../src/folder_events.rs#L204) | receiver-type-required |
| `emit` | `lock()             .observers             .retain` | [205](../../src/folder_events.rs#L205) | receiver-type-required |
| `emit` | `gone.iter().any` | [207](../../src/folder_events.rs#L207) | receiver-type-required |
| `emit` | `gone.iter` | [207](../../src/folder_events.rs#L207) | receiver-type-required |
| `emit` | `Arc::ptr_eq` | [207](../../src/folder_events.rs#L207) | external-constructor-callback-or-unresolved |
| `record_events` | `Arc::new` | [214](../../src/folder_events.rs#L214), [217](../../src/folder_events.rs#L217) | external-constructor-callback-or-unresolved |
| `record_events` | `Mutex::new` | [214](../../src/folder_events.rs#L214) | external-constructor-callback-or-unresolved |
| `record_events` | `Vec::new` | [214](../../src/folder_events.rs#L214) | external-constructor-callback-or-unresolved |
| `record_events` | `Arc::clone` | [215](../../src/folder_events.rs#L215) | external-constructor-callback-or-unresolved |
| `record_events` | `canonical_root` | [216](../../src/folder_events.rs#L216) | [store::folder_events::canonical_root](../../src/folder_events.rs#L71) |
| `record_events` | `subscribe` | [217](../../src/folder_events.rs#L217) | [store::folder_events::subscribe](../../src/folder_events.rs#L137) |
| `record_events` | `event.root` | [218](../../src/folder_events.rs#L218) | receiver-type-required |
| `record_events` | `sink.lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .push` | [219](../../src/folder_events.rs#L219) | receiver-type-required |
| `record_events` | `sink.lock()                 .unwrap_or_else` | [219](../../src/folder_events.rs#L219) | receiver-type-required |
| `record_events` | `sink.lock` | [219](../../src/folder_events.rs#L219) | receiver-type-required |
| `record_events` | `event.clone` | [221](../../src/folder_events.rs#L221) | receiver-type-required |
| `record_events` | `Arc::strong_count` | [223](../../src/folder_events.rs#L223) | external-constructor-callback-or-unresolved |
