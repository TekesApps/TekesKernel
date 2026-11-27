# tekes-worker::tool_backends

[Package atlas](index.md) · [Source](../../src/tool_backends.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::tool_backends::AllowedDynamicBackend](../../src/tool_backends.rs#L3) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::suppress_interrupted_control_terminal](../../src/tool_backends.rs#L9) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedDynamicBackend::supports](../../src/tool_backends.rs#L27) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedDynamicBackend::execute](../../src/tool_backends.rs#L31) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend](../../src/tool_backends.rs#L53) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::supports](../../src/tool_backends.rs#L60) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::execute](../../src/tool_backends.rs#L64) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval](../../src/tool_backends.rs#L69) | function_item | `private` |  |
| [tekes-worker::tool_backends::DeferredChildBackend](../../src/tool_backends.rs#L86) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::DeferredChildBackend::supports](../../src/tool_backends.rs#L89) | function_item | `private` |  |
| [tekes-worker::tool_backends::DeferredChildBackend::execute](../../src/tool_backends.rs#L93) | function_item | `private` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend](../../src/tool_backends.rs#L98) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend::supports](../../src/tool_backends.rs#L103) | function_item | `private` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend::execute](../../src/tool_backends.rs#L107) | function_item | `private` |  |
| [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L142) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::tool_catalog_context](../../src/tool_backends.rs#L210) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ToolBackendPlan](../../src/tool_backends.rs#L259) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ToolBackendRuntime](../../src/tool_backends.rs#L269) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::execution_access_for_profile](../../src/tool_backends.rs#L275) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::assemble_tool_backends](../../src/tool_backends.rs#L286) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L562) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::invocation_write_roots](../../src/tool_backends.rs#L581) | function_item | `pub(crate)` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::invocation_write_roots_for_access](../../src/tool_backends.rs#L589) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L708) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::session_permission_policy](../../src/tool_backends.rs#L728) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_tests::Skills](../../src/tool_backends.rs#L740) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::Skills::skill_known](../../src/tool_backends.rs#L742) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::entry](../../src/tool_backends.rs#L747) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L757) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::a_preset_narrows_the_skill_catalog](../../src/tool_backends.rs#L774) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-worker::tool_backends::preset_skill_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–18: 10 direct edges</summary>

```mermaid
flowchart TD
  n0["store::tail::read_genesis"]
  n1["tools::preset::ResolvedPreset::from_genesis"]
  n2["tools::sandbox::exposed_protected_roots"]
  n3["tools::sandbox::protected_root_containing"]
  n4["tekes-worker::tool_backends::ReplayedChildBackend::supports"]
  n5["tekes-worker::tool_backends::ReplayedChildBackend::execute"]
  n6["tekes-worker::tool_backends::exchange_tool_control_runtime"]
  n7["tekes-worker::tool_backends::tool_catalog_context"]
  n8["tekes-worker::tool_backends::execution_access_for_profile"]
  n9["tekes-worker::tool_backends::AllowedDynamicBackend::supports"]
  n10["tekes-worker::tool_backends::assemble_tool_backends"]
  n11["tekes-worker::tool_backends::AllowedDynamicBackend::execute"]
  n12["tekes-worker::tool_backends::preset_skill_catalog"]
  n13["tekes-worker::tool_backends::invocation_write_roots_for_access"]
  n14["tekes-worker::tool_backends::AllowedToolBackend::supports"]
  n15["tekes-worker::tool_backends::AllowedToolBackend::execute"]
  n16["tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval"]
  n17["tekes-worker::tool_backends::storage_root_for_thread_folder"]
  n18["tekes-worker::tool_backends::session_permission_policy"]
  n19["tekes-worker::tool_backends::DeferredChildBackend::supports"]
  n20["tekes-worker::tool_backends::DeferredChildBackend::execute"]
  n21["tekes-worker::tool_backends::suppress_interrupted_control_terminal"]
  n10 --> n2
  n10 --> n6
  n10 --> n12
  n10 --> n17
  n11 --> n21
  n12 --> n0
  n12 --> n1
  n13 --> n3
  n15 --> n21
  n16 --> n21
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `suppress_interrupted_control_terminal` | `cancellation.protocol_failed` | [16](../../src/tool_backends.rs#L16) | receiver-type-required |
| `suppress_interrupted_control_terminal` | `cancellation.supervisor_lost` | [17](../../src/tool_backends.rs#L17) | receiver-type-required |
| `suppress_interrupted_control_terminal` | `cancellation.stop_requested` | [18](../../src/tool_backends.rs#L18) | receiver-type-required |
| `supports` | `self.allowed.contains` | [28](../../src/tool_backends.rs#L28) | receiver-type-required |
| `supports` | `self.inner.iter().any` | [28](../../src/tool_backends.rs#L28) | receiver-type-required |
| `supports` | `self.inner.iter` | [28](../../src/tool_backends.rs#L28) | receiver-type-required |
| `supports` | `backend.supports` | [28](../../src/tool_backends.rs#L28) | receiver-type-required |
| `execute` | `self             .inner             .iter_mut()             .find(&#124;backend&#124; backend.supports(tool))             .map_or_else` | [37](../../src/tool_backends.rs#L37) | receiver-type-required |
| `execute` | `self             .inner             .iter_mut()             .find` | [37](../../src/tool_backends.rs#L37) | receiver-type-required |
| `execute` | `self             .inner             .iter_mut` | [37](../../src/tool_backends.rs#L37) | receiver-type-required |
| `execute` | `backend.supports` | [40](../../src/tool_backends.rs#L40) | receiver-type-required |
| `execute` | `"dynamic_backend_unavailable".to_owned` | [43](../../src/tool_backends.rs#L43) | receiver-type-required |
| `execute` | `backend.execute` | [47](../../src/tool_backends.rs#L47) | receiver-type-required |
| `execute` | `suppress_interrupted_control_terminal` | [49](../../src/tool_backends.rs#L49) | [tekes-worker::tool_backends::suppress_interrupted_control_terminal](../../src/tool_backends.rs#L9) |
| `supports` | `self.allowed.contains` | [61](../../src/tool_backends.rs#L61) | receiver-type-required |
| `supports` | `self.inner.supports` | [61](../../src/tool_backends.rs#L61) | receiver-type-required |
| `execute` | `self.inner.execute` | [65](../../src/tool_backends.rs#L65) | receiver-type-required |
| `execute` | `suppress_interrupted_control_terminal` | [66](../../src/tool_backends.rs#L66) | [tekes-worker::tool_backends::suppress_interrupted_control_terminal](../../src/tool_backends.rs#L9) |
| `resume_after_approval` | `self             .inner             .resume_after_approval` | [75](../../src/tool_backends.rs#L75) | receiver-type-required |
| `resume_after_approval` | `suppress_interrupted_control_terminal` | [78](../../src/tool_backends.rs#L78) | [tekes-worker::tool_backends::suppress_interrupted_control_terminal](../../src/tool_backends.rs#L9) |
| `execute` | `"protocol".to_owned` | [110](../../src/tool_backends.rs#L110), [134](../../src/tool_backends.rs#L134) | receiver-type-required |
| `execute` | `"cached child launch result has the wrong call id".to_owned` | [111](../../src/tool_backends.rs#L111) | receiver-type-required |
| `execute` | `BackendTerminal::Completed` | [116](../../src/tool_backends.rs#L116) | external-constructor-callback-or-unresolved |
| `execute` | `value.clone` | [116](../../src/tool_backends.rs#L116) | receiver-type-required |
| `execute` | `match error.code {                     ToolControlErrorCode::Unsupported => "unsupported",                     ToolControlErrorCode::Denied => "denied",                     ToolControlErrorCode::NotFound => "not_found",                     ToolControlErrorCode::Conflict => "conflict",                     ToolControlErrorCode::Unavailable => "unavailable",                     ToolControlErrorCode::Timeout => "timeout",                     ToolControlErrorCode::EffectUnknown => "effect_unknown",                     ToolControlErrorCode::EffectConflicted => "effect_conflicted",                     ToolControlErrorCode::Internal => "internal",                 }                 .to_owned` | [118](../../src/tool_backends.rs#L118) | receiver-type-required |
| `execute` | `error.message.clone` | [130](../../src/tool_backends.rs#L130) | receiver-type-required |
| `execute` | `"cached child launch result has an invalid union".to_owned` | [135](../../src/tool_backends.rs#L135) | receiver-type-required |
| `exchange_tool_control_runtime` | `require_version` | [149](../../src/tool_backends.rs#L149) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `cancellation.stop_requested` | [150](../../src/tool_backends.rs#L150) | receiver-type-required |
| `exchange_tool_control_runtime` | `Err` | [151](../../src/tool_backends.rs#L151), [155](../../src/tool_backends.rs#L155), [165](../../src/tool_backends.rs#L165), [178](../../src/tool_backends.rs#L178), [191](../../src/tool_backends.rs#L191), [196](../../src/tool_backends.rs#L196), [203](../../src/tool_backends.rs#L203) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `"tool-control cancelled by stop".into` | [151](../../src/tool_backends.rs#L151), [191](../../src/tool_backends.rs#L191) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.supervisor_lost` | [153](../../src/tool_backends.rs#L153) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.mark_protocol_failed` | [154](../../src/tool_backends.rs#L154), [164](../../src/tool_backends.rs#L164), [171](../../src/tool_backends.rs#L171), [177](../../src/tool_backends.rs#L177), [195](../../src/tool_backends.rs#L195), [202](../../src/tool_backends.rs#L202) | receiver-type-required |
| `exchange_tool_control_runtime` | `Box::new` | [155](../../src/tool_backends.rs#L155), [165](../../src/tool_backends.rs#L165), [178](../../src/tool_backends.rs#L178), [196](../../src/tool_backends.rs#L196), [203](../../src/tool_backends.rs#L203) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `ProtocolFailure` | [155](../../src/tool_backends.rs#L155), [165](../../src/tool_backends.rs#L165), [172](../../src/tool_backends.rs#L172), [196](../../src/tool_backends.rs#L196) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `"supervisor EOF before tool-control request".to_owned` | [156](../../src/tool_backends.rs#L156) | receiver-type-required |
| `exchange_tool_control_runtime` | `stdout.write_all` | [159](../../src/tool_backends.rs#L159) | receiver-type-required |
| `exchange_tool_control_runtime` | `encode_tool_control` | [159](../../src/tool_backends.rs#L159) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `stdout.flush` | [160](../../src/tool_backends.rs#L160), [186](../../src/tool_backends.rs#L186) | receiver-type-required |
| `exchange_tool_control_runtime` | `lines.next` | [162](../../src/tool_backends.rs#L162) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.cancel` | [163](../../src/tool_backends.rs#L163), [170](../../src/tool_backends.rs#L170), [189](../../src/tool_backends.rs#L189) | receiver-type-required |
| `exchange_tool_control_runtime` | `"supervisor EOF while awaiting tool-control result".to_owned` | [166](../../src/tool_backends.rs#L166) | receiver-type-required |
| `exchange_tool_control_runtime` | `line.map_err` | [169](../../src/tool_backends.rs#L169) | receiver-type-required |
| `exchange_tool_control_runtime` | `decode_tool_control_result` | [174](../../src/tool_backends.rs#L174) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `line.as_bytes` | [174](../../src/tool_backends.rs#L174), [182](../../src/tool_backends.rs#L182) | receiver-type-required |
| `exchange_tool_control_runtime` | `result.validate_for` | [176](../../src/tool_backends.rs#L176) | receiver-type-required |
| `exchange_tool_control_runtime` | `Ok` | [180](../../src/tool_backends.rs#L180) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `decode_supervisor` | [182](../../src/tool_backends.rs#L182) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `stdout                         .write_all` | [184](../../src/tool_backends.rs#L184) | receiver-type-required |
| `exchange_tool_control_runtime` | `encode_line` | [185](../../src/tool_backends.rs#L185) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `cancellation.defer` | [190](../../src/tool_backends.rs#L190) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.park_delivery` | [193](../../src/tool_backends.rs#L193) | receiver-type-required |
| `exchange_tool_control_runtime` | `"unexpected supervisor message while awaiting tool-control result"                             .to_owned` | [197](../../src/tool_backends.rs#L197) | receiver-type-required |
| `tool_catalog_context` | `effective_allowed_tools` | [215](../../src/tool_backends.rs#L215) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter()     .filter(&#124;(name, _)&#124; selected_tools.contains(*name))     .collect::<Vec<_>>` | [216](../../src/tool_backends.rs#L216) | receiver-type-required |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter()     .filter` | [216](../../src/tool_backends.rs#L216) | receiver-type-required |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter` | [216](../../src/tool_backends.rs#L216) | receiver-type-required |
| `tool_catalog_context` | `selected_tools.contains` | [222](../../src/tool_backends.rs#L222), [224](../../src/tool_backends.rs#L224) | receiver-type-required |
| `tool_catalog_context` | `Err` | [225](../../src/tool_backends.rs#L225), [228](../../src/tool_backends.rs#L228) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `"report requires a durably delegated child role".into` | [225](../../src/tool_backends.rs#L225) | receiver-type-required |
| `tool_catalog_context` | `selectors.len` | [227](../../src/tool_backends.rs#L227) | receiver-type-required |
| `tool_catalog_context` | `format!(             "effective tool selection contains conflicting role selectors: {}",             selectors                 .iter()                 .map(&#124;(name, _)&#124; *name)                 .collect::<Vec<_>>()                 .join(", ")         )         .into` | [228](../../src/tool_backends.rs#L228) | receiver-type-required |
| `tool_catalog_context` | `Ok` | [238](../../src/tool_backends.rs#L238) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `selectors                 .first()                 .map_or` | [244](../../src/tool_backends.rs#L244) | receiver-type-required |
| `tool_catalog_context` | `selectors                 .first` | [244](../../src/tool_backends.rs#L244) | receiver-type-required |
| `tool_catalog_context` | `profile.config.providers.web_search.is_some` | [249](../../src/tool_backends.rs#L249) | receiver-type-required |
| `tool_catalog_context` | `profile                 .instruction                 .meet_workspace_policy` | [250](../../src/tool_backends.rs#L250) | receiver-type-required |
| `execution_access_for_profile` | `mode.execution_access` | [282](../../src/tool_backends.rs#L282) | receiver-type-required |
| `assemble_tool_backends` | `effective_allowed_tools` | [308](../../src/tool_backends.rs#L308) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `ToolBackendRouter::default` | [309](../../src/tool_backends.rs#L309) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root_for_thread_folder` | [310](../../src/tool_backends.rs#L310) | [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L708) |
| `assemble_tool_backends` | `profile         .instruction         .meet_workspace_policy` | [312](../../src/tool_backends.rs#L312) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first()         .ok_or` | [319](../../src/tool_backends.rs#L319) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first` | [319](../../src/tool_backends.rs#L319) | receiver-type-required |
| `assemble_tool_backends` | `sibling_helper_path` | [325](../../src/tool_backends.rs#L325) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.execution.cwd.clone` | [326](../../src/tool_backends.rs#L326) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.extend` | [327](../../src/tool_backends.rs#L327) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .cloned` | [328](../../src/tool_backends.rs#L328) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter` | [328](../../src/tool_backends.rs#L328), [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.sort` | [336](../../src/tool_backends.rs#L336) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.dedup` | [337](../../src/tool_backends.rs#L337) | receiver-type-required |
| `assemble_tool_backends` | `invocation_write_roots.to_vec` | [338](../../src/tool_backends.rs#L338) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.sort` | [339](../../src/tool_backends.rs#L339) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.dedup` | [340](../../src/tool_backends.rs#L340) | receiver-type-required |
| `assemble_tool_backends` | `tools::exposed_protected_roots` | [341](../../src/tool_backends.rs#L341) | [tools::sandbox::exposed_protected_roots](../../../tools/src/sandbox.rs#L215) |
| `assemble_tool_backends` | `profile.config.protected_roots` | [342](../../src/tool_backends.rs#L342) | receiver-type-required |
| `assemble_tool_backends` | `execution_access.policy` | [345](../../src/tool_backends.rs#L345) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.clone` | [347](../../src/tool_backends.rs#L347) | receiver-type-required |
| `assemble_tool_backends` | `probe_backend` | [364](../../src/tool_backends.rs#L364) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map(&#124;(index, path)&#124; {             (                 if index == 0 {                     "workspace".to_owned()                 } else {                     format!("workspace-{index}")                 },                 PathBuf::from(path),             )         })         .collect::<Vec<_>>` | [365](../../src/tool_backends.rs#L365) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map` | [365](../../src/tool_backends.rs#L365) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate` | [365](../../src/tool_backends.rs#L365), [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter` | [365](../../src/tool_backends.rs#L365), [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `"workspace".to_owned` | [374](../../src/tool_backends.rs#L374) | receiver-type-required |
| `assemble_tool_backends` | `PathBuf::from` | [378](../../src/tool_backends.rs#L378), [383](../../src/tool_backends.rs#L383), [472](../../src/tool_backends.rs#L472), [478](../../src/tool_backends.rs#L478), [497](../../src/tool_backends.rs#L497) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `roots.push` | [383](../../src/tool_backends.rs#L383) | receiver-type-required |
| `assemble_tool_backends` | `"host".to_owned` | [383](../../src/tool_backends.rs#L383), [477](../../src/tool_backends.rs#L477) | receiver-type-required |
| `assemble_tool_backends` | `helper_path.is_file` | [385](../../src/tool_backends.rs#L385) | receiver-type-required |
| `assemble_tool_backends` | `HelperClient::for_execution_access` | [389](../../src/tool_backends.rs#L389) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `client.scratch_path().map` | [396](../../src/tool_backends.rs#L396) | receiver-type-required |
| `assemble_tool_backends` | `client.scratch_path` | [396](../../src/tool_backends.rs#L396) | receiver-type-required |
| `assemble_tool_backends` | `Some` | [398](../../src/tool_backends.rs#L398), [421](../../src/tool_backends.rs#L421) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Arc::new` | [398](../../src/tool_backends.rs#L398), [409](../../src/tool_backends.rs#L409), [421](../../src/tool_backends.rs#L421), [424](../../src/tool_backends.rs#L424), [495](../../src/tool_backends.rs#L495) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Vec::new` | [404](../../src/tool_backends.rs#L404) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose()?         .map` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `DurableArtifactVersions::new` | [407](../../src/tool_backends.rs#L407) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root.join` | [407](../../src/tool_backends.rs#L407), [505](../../src/tool_backends.rs#L505) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then(&#124;&#124; BoundedHttpClient::new(HttpLimits::default()))         .transpose` | [410](../../src/tool_backends.rs#L410) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then` | [410](../../src/tool_backends.rs#L410) | receiver-type-required |
| `assemble_tool_backends` | `BoundedHttpClient::new` | [412](../../src/tool_backends.rs#L412) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `HttpLimits::default` | [412](../../src/tool_backends.rs#L412) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.providers.web_search.as_ref` | [417](../../src/tool_backends.rs#L417) | receiver-type-required |
| `assemble_tool_backends` | `config.clone` | [422](../../src/tool_backends.rs#L422) | receiver-type-required |
| `assemble_tool_backends` | `Arc::clone` | [423](../../src/tool_backends.rs#L423) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SystemToolConfig::workspace` | [429](../../src/tool_backends.rs#L429) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `system_config             .environment             .insert` | [431](../../src/tool_backends.rs#L431), [436](../../src/tool_backends.rs#L436), [459](../../src/tool_backends.rs#L459) | receiver-type-required |
| `assemble_tool_backends` | `"PYTHONDONTWRITEBYTECODE".into` | [433](../../src/tool_backends.rs#L433) | receiver-type-required |
| `assemble_tool_backends` | `"1".into` | [433](../../src/tool_backends.rs#L433) | receiver-type-required |
| `assemble_tool_backends` | `"TMPDIR".into` | [438](../../src/tool_backends.rs#L438) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.policy.toolchain_roots.is_empty` | [440](../../src/tool_backends.rs#L440) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map(&#124;root&#124; {                 std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned()             })             .collect::<Vec<_>>` | [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map` | [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned` | [448](../../src/tool_backends.rs#L448) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy` | [448](../../src/tool_backends.rs#L448) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join` | [448](../../src/tool_backends.rs#L448) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new` | [448](../../src/tool_backends.rs#L448) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `bins.extend` | [454](../../src/tool_backends.rs#L454) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter()                 .map` | [455](../../src/tool_backends.rs#L455) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter` | [455](../../src/tool_backends.rs#L455) | receiver-type-required |
| `assemble_tool_backends` | `"PATH".into` | [461](../../src/tool_backends.rs#L461) | receiver-type-required |
| `assemble_tool_backends` | `bins.join` | [461](../../src/tool_backends.rs#L461) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map(&#124;(index, path)&#124; engine::RootMount {             name: format!("workspace-{index}"),             path: PathBuf::from(path),         })         .collect` | [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map` | [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip` | [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `system_config.extra_roots.push` | [476](../../src/tool_backends.rs#L476) | receiver-type-required |
| `assemble_tool_backends` | `SystemToolBackend::new` | [481](../../src/tool_backends.rs#L481) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `cancellation.tools.clone` | [487](../../src/tool_backends.rs#L487) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or("worker executable has no parent directory")?         .join` | [489](../../src/tool_backends.rs#L489) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or` | [489](../../src/tool_backends.rs#L489) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent` | [489](../../src/tool_backends.rs#L489) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe` | [489](../../src/tool_backends.rs#L489) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `workspace_helper.is_file` | [493](../../src/tool_backends.rs#L493) | receiver-type-required |
| `assemble_tool_backends` | `system_backend.with_edit_recorder` | [495](../../src/tool_backends.rs#L495) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map(PathBuf::from)                     .collect` | [498](../../src/tool_backends.rs#L498) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map` | [498](../../src/tool_backends.rs#L498) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter` | [498](../../src/tool_backends.rs#L498) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.session_id.clone` | [506](../../src/tool_backends.rs#L506), [544](../../src/tool_backends.rs#L544) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or("thread folder has no session identity")?                     .to_owned` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `assemble_tool_backends` | `name.to_str` | [509](../../src/tool_backends.rs#L509), [518](../../src/tool_backends.rs#L518) | receiver-type-required |
| `assemble_tool_backends` | `router.push` | [514](../../src/tool_backends.rs#L514), [526](../../src/tool_backends.rs#L526), [542](../../src/tool_backends.rs#L542) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or("thread folder has no UTF-8 session UUID")?             .to_owned` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `assemble_tool_backends` | `Rc::new` | [521](../../src/tool_backends.rs#L521) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `RefCell::new` | [521](../../src/tool_backends.rs#L521) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange_tool_control_runtime(&selected, request, lines, stdout, cancellation)                 .map_err` | [522](../../src/tool_backends.rs#L522) | receiver-type-required |
| `assemble_tool_backends` | `exchange_tool_control_runtime` | [522](../../src/tool_backends.rs#L522) | [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L142) |
| `assemble_tool_backends` | `error.to_string` | [523](../../src/tool_backends.rs#L523) | receiver-type-required |
| `assemble_tool_backends` | `Rc::clone` | [525](../../src/tool_backends.rs#L525) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SupervisorControlBackend::new` | [526](../../src/tool_backends.rs#L526) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `session.clone` | [527](../../src/tool_backends.rs#L527) | receiver-type-required |
| `assemble_tool_backends` | `(fixed_exchange.borrow_mut())` | [528](../../src/tool_backends.rs#L528) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `fixed_exchange.borrow_mut` | [528](../../src/tool_backends.rs#L528) | receiver-type-required |
| `assemble_tool_backends` | `dynamic_backends.push` | [530](../../src/tool_backends.rs#L530) | receiver-type-required |
| `assemble_tool_backends` | `Box::new` | [530](../../src/tool_backends.rs#L530) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `DynamicSupervisorBackend::new` | [530](../../src/tool_backends.rs#L530) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `(exchange.borrow_mut())` | [532](../../src/tool_backends.rs#L532) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange.borrow_mut` | [532](../../src/tool_backends.rs#L532) | receiver-type-required |
| `assemble_tool_backends` | `allowed.clone` | [536](../../src/tool_backends.rs#L536) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)         .deferred_search_entries` | [540](../../src/tool_backends.rs#L540) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new` | [540](../../src/tool_backends.rs#L540) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `WorkflowBackend::new` | [542](../../src/tool_backends.rs#L542) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.bindings.goal_id.clone` | [545](../../src/tool_backends.rs#L545) | receiver-type-required |
| `assemble_tool_backends` | `preset_skill_catalog` | [546](../../src/tool_backends.rs#L546) | [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L562) |
| `assemble_tool_backends` | `frozen_skill_catalog` | [546](../../src/tool_backends.rs#L546) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Ok` | [549](../../src/tool_backends.rs#L549) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `thread_folder.join` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `preset_skill_catalog` | `store::read_genesis` | [567](../../src/tool_backends.rs#L567) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `preset_skill_catalog` | `Ok` | [568](../../src/tool_backends.rs#L568), [572](../../src/tool_backends.rs#L572), [577](../../src/tool_backends.rs#L577) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `serde_json::to_value` | [570](../../src/tool_backends.rs#L570) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `genesis.raw` | [570](../../src/tool_backends.rs#L570) | receiver-type-required |
| `preset_skill_catalog` | `raw.get` | [571](../../src/tool_backends.rs#L571) | receiver-type-required |
| `preset_skill_catalog` | `tools::ResolvedPreset::from_genesis` | [574](../../src/tool_backends.rs#L574) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `preset_skill_catalog` | `catalog.retain` | [575](../../src/tool_backends.rs#L575) | receiver-type-required |
| `preset_skill_catalog` | `skills.contains` | [575](../../src/tool_backends.rs#L575) | receiver-type-required |
| `invocation_write_roots` | `invocation_write_roots_for_access` | [586](../../src/tool_backends.rs#L586) | [tekes-worker::tool_backends::invocation_write_roots_for_access](../../src/tool_backends.rs#L589) |
| `invocation_write_roots_for_access` | `PathBuf::from` | [597](../../src/tool_backends.rs#L597), [667](../../src/tool_backends.rs#L667) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `profile             .config             .execution             .cwd             .first()             .ok_or` | [598](../../src/tool_backends.rs#L598) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .config             .execution             .cwd             .first` | [598](../../src/tool_backends.rs#L598) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile         .bindings         .dynamic_catalog         .tools         .iter()         .any` | [605](../../src/tool_backends.rs#L605) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile         .bindings         .dynamic_catalog         .tools         .iter` | [605](../../src/tool_backends.rs#L605) | receiver-type-required |
| `invocation_write_roots_for_access` | `Ok` | [613](../../src/tool_backends.rs#L613), [615](../../src/tool_backends.rs#L615), [705](../../src/tool_backends.rs#L705) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy` | [615](../../src/tool_backends.rs#L615), [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter()             .map(PathBuf::from)             .collect::<Vec<_>>` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter()             .map` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.name.as_str` | [631](../../src/tool_backends.rs#L631) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.arguments.get("writable_paths").is_none_or` | [633](../../src/tool_backends.rs#L633) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.arguments.get` | [633](../../src/tool_backends.rs#L633) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.is_null` | [634](../../src/tool_backends.rs#L634) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.as_array().is_some_and` | [634](../../src/tool_backends.rs#L634) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.as_array` | [634](../../src/tool_backends.rs#L634) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter()                 .map(&#124;path&#124; path.to_string_lossy().into_owned())                 .collect` | [639](../../src/tool_backends.rs#L639) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter()                 .map` | [639](../../src/tool_backends.rs#L639) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter` | [639](../../src/tool_backends.rs#L639) | receiver-type-required |
| `invocation_write_roots_for_access` | `path.to_string_lossy().into_owned` | [641](../../src/tool_backends.rs#L641) | receiver-type-required |
| `invocation_write_roots_for_access` | `path.to_string_lossy` | [641](../../src/tool_backends.rs#L641) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map(&#124;value&#124; {                 value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or("shell writable_paths must contain strings")             })             .collect::<Result<Vec<_>, _>>` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get` | [644](../../src/tool_backends.rs#L644), [657](../../src/tool_backends.rs#L657) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or` | [651](../../src/tool_backends.rs#L651) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str()                     .map` | [651](../../src/tool_backends.rs#L651) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str` | [651](../../src/tool_backends.rs#L651) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map(&#124;path&#124; vec![path.to_owned()])             .unwrap_or_default` | [657](../../src/tool_backends.rs#L657) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map` | [657](../../src/tool_backends.rs#L657) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then` | [657](../../src/tool_backends.rs#L657) | receiver-type-required |
| `invocation_write_roots_for_access` | `Vec::new` | [663](../../src/tool_backends.rs#L663), [665](../../src/tool_backends.rs#L665) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `raw_path             .components()             .any` | [668](../../src/tool_backends.rs#L668) | receiver-type-required |
| `invocation_write_roots_for_access` | `raw_path             .components` | [668](../../src/tool_backends.rs#L668) | receiver-type-required |
| `invocation_write_roots_for_access` | `Err` | [672](../../src/tool_backends.rs#L672), [680](../../src/tool_backends.rs#L680), [687](../../src/tool_backends.rs#L687) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `format!("writable path {raw:?} contains parent traversal").into` | [672](../../src/tool_backends.rs#L672) | receiver-type-required |
| `invocation_write_roots_for_access` | `raw_path.is_absolute` | [674](../../src/tool_backends.rs#L674) | receiver-type-required |
| `invocation_write_roots_for_access` | `workspace.join` | [677](../../src/tool_backends.rs#L677) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed.iter().any` | [679](../../src/tool_backends.rs#L679) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed.iter` | [679](../../src/tool_backends.rs#L679) | receiver-type-required |
| `invocation_write_roots_for_access` | `target.starts_with` | [679](../../src/tool_backends.rs#L679) | receiver-type-required |
| `invocation_write_roots_for_access` | `format!(                 "writable path {} is outside the effective writable roots",                 target.display()             )             .into` | [680](../../src/tool_backends.rs#L680) | receiver-type-required |
| `invocation_write_roots_for_access` | `tools::protected_root_containing` | [686](../../src/tool_backends.rs#L686) | [tools::sandbox::protected_root_containing](../../../tools/src/sandbox.rs#L241) |
| `invocation_write_roots_for_access` | `format!(                 "writable path {} is inside the Kernel-owned directory {protected}",                 target.display()             )             .into` | [687](../../src/tool_backends.rs#L687) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent()                 .ok_or("apply_patch target has no writable parent")?                 .to_path_buf` | [694](../../src/tool_backends.rs#L694) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent()                 .ok_or` | [694](../../src/tool_backends.rs#L694) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent` | [694](../../src/tool_backends.rs#L694) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.push` | [701](../../src/tool_backends.rs#L701) | receiver-type-required |
| `invocation_write_roots_for_access` | `authority.to_string_lossy().into_owned` | [701](../../src/tool_backends.rs#L701) | receiver-type-required |
| `invocation_write_roots_for_access` | `authority.to_string_lossy` | [701](../../src/tool_backends.rs#L701) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.sort` | [703](../../src/tool_backends.rs#L703) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.dedup` | [704](../../src/tool_backends.rs#L704) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent()         .ok_or` | [711](../../src/tool_backends.rs#L711) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent` | [711](../../src/tool_backends.rs#L711) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name().is_some_and` | [714](../../src/tool_backends.rs#L714) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name` | [714](../../src/tool_backends.rs#L714) | receiver-type-required |
| `storage_root_for_thread_folder` | `Ok` | [715](../../src/tool_backends.rs#L715), [722](../../src/tool_backends.rs#L722) | external-constructor-callback-or-unresolved |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or("threads directory has no storage root")?             .to_path_buf` | [715](../../src/tool_backends.rs#L715) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or` | [715](../../src/tool_backends.rs#L715) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent` | [715](../../src/tool_backends.rs#L715) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.to_path_buf` | [722](../../src/tool_backends.rs#L722) | receiver-type-required |
| `session_permission_policy` | `PermissionModePolicy::for_session_folder` | [729](../../src/tool_backends.rs#L729) | external-constructor-callback-or-unresolved |
| `entry` | `name.to_owned` | [749](../../src/tool_backends.rs#L749) | receiver-type-required |
| `entry` | `String::new` | [750](../../src/tool_backends.rs#L750) | external-constructor-callback-or-unresolved |
| `entry` | `Vec::new` | [751](../../src/tool_backends.rs#L751) | external-constructor-callback-or-unresolved |
| `entry` | `"d".to_owned` | [752](../../src/tool_backends.rs#L752) | receiver-type-required |
| `entry` | `IJsonValue::parse(b"{}").unwrap` | [753](../../src/tool_backends.rs#L753) | receiver-type-required |
| `entry` | `IJsonValue::parse` | [753](../../src/tool_backends.rs#L753) | external-constructor-callback-or-unresolved |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap()             .extend` | [764](../../src/tool_backends.rs#L764) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap` | [764](../../src/tool_backends.rs#L764) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut` | [764](../../src/tool_backends.rs#L764) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap().clone` | [767](../../src/tool_backends.rs#L767) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap` | [767](../../src/tool_backends.rs#L767) | receiver-type-required |
| `write_genesis` | `opening.as_object` | [767](../../src/tool_backends.rs#L767) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes().unwrap` | [768](../../src/tool_backends.rs#L768) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes` | [768](../../src/tool_backends.rs#L768) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap` | [768](../../src/tool_backends.rs#L768) | receiver-type-required |
| `write_genesis` | `make_event` | [768](../../src/tool_backends.rs#L768) | external-constructor-callback-or-unresolved |
| `write_genesis` | `bytes.push` | [769](../../src/tool_backends.rs#L769) | receiver-type-required |
| `write_genesis` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [770](../../src/tool_backends.rs#L770) | receiver-type-required |
| `write_genesis` | `std::fs::write` | [770](../../src/tool_backends.rs#L770) | external-constructor-callback-or-unresolved |
| `write_genesis` | `folder.join` | [770](../../src/tool_backends.rs#L770) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir().unwrap` | [775](../../src/tool_backends.rs#L775) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir` | [775](../../src/tool_backends.rs#L775) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `write_genesis` | [777](../../src/tool_backends.rs#L777), [790](../../src/tool_backends.rs#L790) | [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L757) |
| `a_preset_narrows_the_skill_catalog` | `folder.path` | [777](../../src/tool_backends.rs#L777), [790](../../src/tool_backends.rs#L790), [791](../../src/tool_backends.rs#L791) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request(             &json!({"skills":["style-guide","removed-later"]}),             tools::PresetBase::General,             &Skills,         )         .unwrap` | [784](../../src/tool_backends.rs#L784) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request` | [784](../../src/tool_backends.rs#L784) | [tools::preset::ResolvedPreset::resolve_request](../../../tools/src/preset.rs#L721) |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog(catalog(), folder.path()).unwrap` | [791](../../src/tool_backends.rs#L791) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog` | [791](../../src/tool_backends.rs#L791) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `catalog` | [791](../../src/tool_backends.rs#L791) | external-constructor-callback-or-unresolved |
