# tekes-worker::tool_backends

[Package atlas](index.md) · [Source](../../src/tool_backends.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::tool_backends::AllowedDynamicBackend](../../src/tool_backends.rs#L3) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::AllowedDynamicBackend::supports](../../src/tool_backends.rs#L9) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedDynamicBackend::execute](../../src/tool_backends.rs#L13) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend](../../src/tool_backends.rs#L33) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::supports](../../src/tool_backends.rs#L39) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::execute](../../src/tool_backends.rs#L43) | function_item | `private` |  |
| [tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval](../../src/tool_backends.rs#L47) | function_item | `private` |  |
| [tekes-worker::tool_backends::DeferredChildBackend](../../src/tool_backends.rs#L62) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::DeferredChildBackend::supports](../../src/tool_backends.rs#L65) | function_item | `private` |  |
| [tekes-worker::tool_backends::DeferredChildBackend::execute](../../src/tool_backends.rs#L69) | function_item | `private` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend](../../src/tool_backends.rs#L74) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend::supports](../../src/tool_backends.rs#L79) | function_item | `private` |  |
| [tekes-worker::tool_backends::ReplayedChildBackend::execute](../../src/tool_backends.rs#L83) | function_item | `private` |  |
| [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L118) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::tool_catalog_context](../../src/tool_backends.rs#L186) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ToolBackendPlan](../../src/tool_backends.rs#L235) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::ToolBackendRuntime](../../src/tool_backends.rs#L244) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::assemble_tool_backends](../../src/tool_backends.rs#L250) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L503) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::invocation_write_roots](../../src/tool_backends.rs#L521) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L630) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::session_permission_policy](../../src/tool_backends.rs#L650) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_tests::Skills](../../src/tool_backends.rs#L662) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::Skills::skill_known](../../src/tool_backends.rs#L664) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::entry](../../src/tool_backends.rs#L669) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L679) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::a_preset_narrows_the_skill_catalog](../../src/tool_backends.rs#L696) | function_item | `private` | test; #[cfg(test)] |

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

<details><summary>Functions 1–16: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["store::tail::read_genesis"]
  n1["tools::preset::ResolvedPreset::from_genesis"]
  n2["tools::sandbox::exposed_protected_roots"]
  n3["tools::sandbox::protected_root_containing"]
  n4["tekes-worker::tool_backends::exchange_tool_control_runtime"]
  n5["tekes-worker::tool_backends::AllowedDynamicBackend::execute"]
  n6["tekes-worker::tool_backends::tool_catalog_context"]
  n7["tekes-worker::tool_backends::assemble_tool_backends"]
  n8["tekes-worker::tool_backends::AllowedToolBackend::supports"]
  n9["tekes-worker::tool_backends::AllowedToolBackend::execute"]
  n10["tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval"]
  n11["tekes-worker::tool_backends::preset_skill_catalog"]
  n12["tekes-worker::tool_backends::invocation_write_roots"]
  n13["tekes-worker::tool_backends::storage_root_for_thread_folder"]
  n14["tekes-worker::tool_backends::session_permission_policy"]
  n15["tekes-worker::tool_backends::DeferredChildBackend::supports"]
  n16["tekes-worker::tool_backends::DeferredChildBackend::execute"]
  n17["tekes-worker::tool_backends::ReplayedChildBackend::supports"]
  n18["tekes-worker::tool_backends::ReplayedChildBackend::execute"]
  n19["tekes-worker::tool_backends::AllowedDynamicBackend::supports"]
  n7 --> n2
  n7 --> n4
  n7 --> n11
  n7 --> n13
  n11 --> n0
  n11 --> n1
  n12 --> n3
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `supports` | `self.allowed.contains` | [10](../../src/tool_backends.rs#L10) | receiver-type-required |
| `supports` | `self.inner.iter().any` | [10](../../src/tool_backends.rs#L10) | receiver-type-required |
| `supports` | `self.inner.iter` | [10](../../src/tool_backends.rs#L10) | receiver-type-required |
| `supports` | `backend.supports` | [10](../../src/tool_backends.rs#L10) | receiver-type-required |
| `execute` | `self.inner             .iter_mut()             .find(&#124;backend&#124; backend.supports(tool))             .map_or_else` | [19](../../src/tool_backends.rs#L19) | receiver-type-required |
| `execute` | `self.inner             .iter_mut()             .find` | [19](../../src/tool_backends.rs#L19) | receiver-type-required |
| `execute` | `self.inner             .iter_mut` | [19](../../src/tool_backends.rs#L19) | receiver-type-required |
| `execute` | `backend.supports` | [21](../../src/tool_backends.rs#L21) | receiver-type-required |
| `execute` | `"dynamic_backend_unavailable".to_owned` | [24](../../src/tool_backends.rs#L24) | receiver-type-required |
| `execute` | `backend.execute` | [28](../../src/tool_backends.rs#L28) | receiver-type-required |
| `supports` | `self.allowed.contains` | [40](../../src/tool_backends.rs#L40) | receiver-type-required |
| `supports` | `self.inner.supports` | [40](../../src/tool_backends.rs#L40) | receiver-type-required |
| `execute` | `self.inner.execute` | [44](../../src/tool_backends.rs#L44) | receiver-type-required |
| `resume_after_approval` | `self.inner             .resume_after_approval` | [53](../../src/tool_backends.rs#L53) | receiver-type-required |
| `execute` | `"protocol".to_owned` | [86](../../src/tool_backends.rs#L86), [110](../../src/tool_backends.rs#L110) | receiver-type-required |
| `execute` | `"cached child launch result has the wrong call id".to_owned` | [87](../../src/tool_backends.rs#L87) | receiver-type-required |
| `execute` | `BackendTerminal::Completed` | [92](../../src/tool_backends.rs#L92) | external-constructor-callback-or-unresolved |
| `execute` | `value.clone` | [92](../../src/tool_backends.rs#L92) | receiver-type-required |
| `execute` | `match error.code {                     ToolControlErrorCode::Unsupported => "unsupported",                     ToolControlErrorCode::Denied => "denied",                     ToolControlErrorCode::NotFound => "not_found",                     ToolControlErrorCode::Conflict => "conflict",                     ToolControlErrorCode::Unavailable => "unavailable",                     ToolControlErrorCode::Timeout => "timeout",                     ToolControlErrorCode::EffectUnknown => "effect_unknown",                     ToolControlErrorCode::EffectConflicted => "effect_conflicted",                     ToolControlErrorCode::Internal => "internal",                 }                 .to_owned` | [94](../../src/tool_backends.rs#L94) | receiver-type-required |
| `execute` | `error.message.clone` | [106](../../src/tool_backends.rs#L106) | receiver-type-required |
| `execute` | `"cached child launch result has an invalid union".to_owned` | [111](../../src/tool_backends.rs#L111) | receiver-type-required |
| `exchange_tool_control_runtime` | `require_version` | [125](../../src/tool_backends.rs#L125) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `cancellation.stop_requested` | [126](../../src/tool_backends.rs#L126) | receiver-type-required |
| `exchange_tool_control_runtime` | `Err` | [127](../../src/tool_backends.rs#L127), [131](../../src/tool_backends.rs#L131), [141](../../src/tool_backends.rs#L141), [154](../../src/tool_backends.rs#L154), [167](../../src/tool_backends.rs#L167), [172](../../src/tool_backends.rs#L172), [179](../../src/tool_backends.rs#L179) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `"tool-control cancelled by stop".into` | [127](../../src/tool_backends.rs#L127), [167](../../src/tool_backends.rs#L167) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.supervisor_lost` | [129](../../src/tool_backends.rs#L129) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.mark_protocol_failed` | [130](../../src/tool_backends.rs#L130), [140](../../src/tool_backends.rs#L140), [147](../../src/tool_backends.rs#L147), [153](../../src/tool_backends.rs#L153), [171](../../src/tool_backends.rs#L171), [178](../../src/tool_backends.rs#L178) | receiver-type-required |
| `exchange_tool_control_runtime` | `Box::new` | [131](../../src/tool_backends.rs#L131), [141](../../src/tool_backends.rs#L141), [154](../../src/tool_backends.rs#L154), [172](../../src/tool_backends.rs#L172), [179](../../src/tool_backends.rs#L179) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `ProtocolFailure` | [131](../../src/tool_backends.rs#L131), [141](../../src/tool_backends.rs#L141), [148](../../src/tool_backends.rs#L148), [172](../../src/tool_backends.rs#L172) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `"supervisor EOF before tool-control request".to_owned` | [132](../../src/tool_backends.rs#L132) | receiver-type-required |
| `exchange_tool_control_runtime` | `stdout.write_all` | [135](../../src/tool_backends.rs#L135) | receiver-type-required |
| `exchange_tool_control_runtime` | `encode_tool_control` | [135](../../src/tool_backends.rs#L135) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `stdout.flush` | [136](../../src/tool_backends.rs#L136), [162](../../src/tool_backends.rs#L162) | receiver-type-required |
| `exchange_tool_control_runtime` | `lines.next` | [138](../../src/tool_backends.rs#L138) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.cancel` | [139](../../src/tool_backends.rs#L139), [146](../../src/tool_backends.rs#L146), [165](../../src/tool_backends.rs#L165) | receiver-type-required |
| `exchange_tool_control_runtime` | `"supervisor EOF while awaiting tool-control result".to_owned` | [142](../../src/tool_backends.rs#L142) | receiver-type-required |
| `exchange_tool_control_runtime` | `line.map_err` | [145](../../src/tool_backends.rs#L145) | receiver-type-required |
| `exchange_tool_control_runtime` | `decode_tool_control_result` | [150](../../src/tool_backends.rs#L150) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `line.as_bytes` | [150](../../src/tool_backends.rs#L150), [158](../../src/tool_backends.rs#L158) | receiver-type-required |
| `exchange_tool_control_runtime` | `result.validate_for` | [152](../../src/tool_backends.rs#L152) | receiver-type-required |
| `exchange_tool_control_runtime` | `Ok` | [156](../../src/tool_backends.rs#L156) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `decode_supervisor` | [158](../../src/tool_backends.rs#L158) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `stdout                         .write_all` | [160](../../src/tool_backends.rs#L160) | receiver-type-required |
| `exchange_tool_control_runtime` | `encode_line` | [161](../../src/tool_backends.rs#L161) | external-constructor-callback-or-unresolved |
| `exchange_tool_control_runtime` | `cancellation.defer` | [166](../../src/tool_backends.rs#L166) | receiver-type-required |
| `exchange_tool_control_runtime` | `cancellation.park_delivery` | [169](../../src/tool_backends.rs#L169) | receiver-type-required |
| `exchange_tool_control_runtime` | `"unexpected supervisor message while awaiting tool-control result"                             .to_owned` | [173](../../src/tool_backends.rs#L173) | receiver-type-required |
| `tool_catalog_context` | `effective_allowed_tools` | [191](../../src/tool_backends.rs#L191) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter()     .filter(&#124;(name, _)&#124; selected_tools.contains(*name))     .collect::<Vec<_>>` | [192](../../src/tool_backends.rs#L192) | receiver-type-required |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter()     .filter` | [192](../../src/tool_backends.rs#L192) | receiver-type-required |
| `tool_catalog_context` | `[         ("plan", CatalogRole::Plan),         ("summary_artifact", CatalogRole::Compactor),         ("verify", CatalogRole::Validator),     ]     .into_iter` | [192](../../src/tool_backends.rs#L192) | receiver-type-required |
| `tool_catalog_context` | `selected_tools.contains` | [198](../../src/tool_backends.rs#L198), [200](../../src/tool_backends.rs#L200) | receiver-type-required |
| `tool_catalog_context` | `Err` | [201](../../src/tool_backends.rs#L201), [204](../../src/tool_backends.rs#L204) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `"report requires a durably delegated child role".into` | [201](../../src/tool_backends.rs#L201) | receiver-type-required |
| `tool_catalog_context` | `selectors.len` | [203](../../src/tool_backends.rs#L203) | receiver-type-required |
| `tool_catalog_context` | `format!(             "effective tool selection contains conflicting role selectors: {}",             selectors                 .iter()                 .map(&#124;(name, _)&#124; *name)                 .collect::<Vec<_>>()                 .join(", ")         )         .into` | [204](../../src/tool_backends.rs#L204) | receiver-type-required |
| `tool_catalog_context` | `Ok` | [214](../../src/tool_backends.rs#L214) | external-constructor-callback-or-unresolved |
| `tool_catalog_context` | `selectors                 .first()                 .map_or` | [220](../../src/tool_backends.rs#L220) | receiver-type-required |
| `tool_catalog_context` | `selectors                 .first` | [220](../../src/tool_backends.rs#L220) | receiver-type-required |
| `tool_catalog_context` | `profile.config.providers.web_search.is_some` | [225](../../src/tool_backends.rs#L225) | receiver-type-required |
| `tool_catalog_context` | `profile                 .instruction                 .meet_workspace_policy` | [226](../../src/tool_backends.rs#L226) | receiver-type-required |
| `assemble_tool_backends` | `effective_allowed_tools` | [271](../../src/tool_backends.rs#L271) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `ToolBackendRouter::default` | [272](../../src/tool_backends.rs#L272) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root_for_thread_folder` | [273](../../src/tool_backends.rs#L273) | [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L630) |
| `assemble_tool_backends` | `profile         .instruction         .meet_workspace_policy` | [275](../../src/tool_backends.rs#L275) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first()         .ok_or` | [278](../../src/tool_backends.rs#L278) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first` | [278](../../src/tool_backends.rs#L278) | receiver-type-required |
| `assemble_tool_backends` | `sibling_helper_path` | [284](../../src/tool_backends.rs#L284) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.execution.cwd.clone` | [285](../../src/tool_backends.rs#L285) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.extend` | [286](../../src/tool_backends.rs#L286) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .cloned` | [287](../../src/tool_backends.rs#L287) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter` | [287](../../src/tool_backends.rs#L287), [390](../../src/tool_backends.rs#L390) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.sort` | [295](../../src/tool_backends.rs#L295) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.dedup` | [296](../../src/tool_backends.rs#L296) | receiver-type-required |
| `assemble_tool_backends` | `invocation_write_roots.to_vec` | [297](../../src/tool_backends.rs#L297) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.sort` | [298](../../src/tool_backends.rs#L298) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.dedup` | [299](../../src/tool_backends.rs#L299) | receiver-type-required |
| `assemble_tool_backends` | `tools::exposed_protected_roots` | [300](../../src/tool_backends.rs#L300) | [tools::sandbox::exposed_protected_roots](../../../tools/src/sandbox.rs#L187) |
| `assemble_tool_backends` | `profile.config.protected_roots` | [301](../../src/tool_backends.rs#L301) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.clone` | [306](../../src/tool_backends.rs#L306) | receiver-type-required |
| `assemble_tool_backends` | `probe_backend` | [323](../../src/tool_backends.rs#L323) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map(&#124;(index, path)&#124; {             (                 if index == 0 {                     "workspace".to_owned()                 } else {                     format!("workspace-{index}")                 },                 PathBuf::from(path),             )         })         .collect::<Vec<_>>` | [324](../../src/tool_backends.rs#L324) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map` | [324](../../src/tool_backends.rs#L324) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate` | [324](../../src/tool_backends.rs#L324), [412](../../src/tool_backends.rs#L412) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter` | [324](../../src/tool_backends.rs#L324), [412](../../src/tool_backends.rs#L412) | receiver-type-required |
| `assemble_tool_backends` | `"workspace".to_owned` | [333](../../src/tool_backends.rs#L333) | receiver-type-required |
| `assemble_tool_backends` | `PathBuf::from` | [337](../../src/tool_backends.rs#L337), [421](../../src/tool_backends.rs#L421), [440](../../src/tool_backends.rs#L440) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper_path.is_file` | [342](../../src/tool_backends.rs#L342) | receiver-type-required |
| `assemble_tool_backends` | `HelperClient::sandboxed_with_scratch` | [344](../../src/tool_backends.rs#L344) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `client.scratch_path().map` | [345](../../src/tool_backends.rs#L345) | receiver-type-required |
| `assemble_tool_backends` | `client.scratch_path` | [345](../../src/tool_backends.rs#L345) | receiver-type-required |
| `assemble_tool_backends` | `Some` | [347](../../src/tool_backends.rs#L347), [370](../../src/tool_backends.rs#L370) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Arc::new` | [347](../../src/tool_backends.rs#L347), [358](../../src/tool_backends.rs#L358), [370](../../src/tool_backends.rs#L370), [373](../../src/tool_backends.rs#L373), [438](../../src/tool_backends.rs#L438) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Vec::new` | [353](../../src/tool_backends.rs#L353) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose()?         .map` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `DurableArtifactVersions::new` | [356](../../src/tool_backends.rs#L356) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root.join` | [356](../../src/tool_backends.rs#L356), [448](../../src/tool_backends.rs#L448) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then(&#124;&#124; BoundedHttpClient::new(HttpLimits::default()))         .transpose` | [359](../../src/tool_backends.rs#L359) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then` | [359](../../src/tool_backends.rs#L359) | receiver-type-required |
| `assemble_tool_backends` | `BoundedHttpClient::new` | [361](../../src/tool_backends.rs#L361) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `HttpLimits::default` | [361](../../src/tool_backends.rs#L361) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.providers.web_search.as_ref` | [366](../../src/tool_backends.rs#L366) | receiver-type-required |
| `assemble_tool_backends` | `config.clone` | [371](../../src/tool_backends.rs#L371) | receiver-type-required |
| `assemble_tool_backends` | `Arc::clone` | [372](../../src/tool_backends.rs#L372) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SystemToolConfig::workspace` | [378](../../src/tool_backends.rs#L378) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `system_config             .environment             .insert` | [380](../../src/tool_backends.rs#L380), [385](../../src/tool_backends.rs#L385), [408](../../src/tool_backends.rs#L408) | receiver-type-required |
| `assemble_tool_backends` | `"PYTHONDONTWRITEBYTECODE".into` | [382](../../src/tool_backends.rs#L382) | receiver-type-required |
| `assemble_tool_backends` | `"1".into` | [382](../../src/tool_backends.rs#L382) | receiver-type-required |
| `assemble_tool_backends` | `"TMPDIR".into` | [387](../../src/tool_backends.rs#L387) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.policy.toolchain_roots.is_empty` | [389](../../src/tool_backends.rs#L389) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map(&#124;root&#124; {                 std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned()             })             .collect::<Vec<_>>` | [390](../../src/tool_backends.rs#L390) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map` | [390](../../src/tool_backends.rs#L390) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned` | [397](../../src/tool_backends.rs#L397) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy` | [397](../../src/tool_backends.rs#L397) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join` | [397](../../src/tool_backends.rs#L397) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new` | [397](../../src/tool_backends.rs#L397) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `bins.extend` | [403](../../src/tool_backends.rs#L403) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter()                 .map` | [404](../../src/tool_backends.rs#L404) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter` | [404](../../src/tool_backends.rs#L404) | receiver-type-required |
| `assemble_tool_backends` | `"PATH".into` | [410](../../src/tool_backends.rs#L410) | receiver-type-required |
| `assemble_tool_backends` | `bins.join` | [410](../../src/tool_backends.rs#L410) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map(&#124;(index, path)&#124; engine::RootMount {             name: format!("workspace-{index}"),             path: PathBuf::from(path),         })         .collect` | [412](../../src/tool_backends.rs#L412) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map` | [412](../../src/tool_backends.rs#L412) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip` | [412](../../src/tool_backends.rs#L412) | receiver-type-required |
| `assemble_tool_backends` | `SystemToolBackend::new` | [424](../../src/tool_backends.rs#L424) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `cancellation.tools.clone` | [430](../../src/tool_backends.rs#L430) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or("worker executable has no parent directory")?         .join` | [432](../../src/tool_backends.rs#L432) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or` | [432](../../src/tool_backends.rs#L432) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent` | [432](../../src/tool_backends.rs#L432) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe` | [432](../../src/tool_backends.rs#L432) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `workspace_helper.is_file` | [436](../../src/tool_backends.rs#L436) | receiver-type-required |
| `assemble_tool_backends` | `system_backend.with_edit_recorder` | [438](../../src/tool_backends.rs#L438) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map(PathBuf::from)                     .collect` | [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map` | [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter` | [441](../../src/tool_backends.rs#L441) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.session_id.clone` | [449](../../src/tool_backends.rs#L449), [486](../../src/tool_backends.rs#L486) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or("thread folder has no session identity")?                     .to_owned` | [450](../../src/tool_backends.rs#L450) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or` | [450](../../src/tool_backends.rs#L450) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then` | [450](../../src/tool_backends.rs#L450) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name` | [450](../../src/tool_backends.rs#L450) | receiver-type-required |
| `assemble_tool_backends` | `name.to_str` | [452](../../src/tool_backends.rs#L452), [461](../../src/tool_backends.rs#L461) | receiver-type-required |
| `assemble_tool_backends` | `router.push` | [457](../../src/tool_backends.rs#L457), [469](../../src/tool_backends.rs#L469), [484](../../src/tool_backends.rs#L484) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or("thread folder has no UTF-8 session UUID")?             .to_owned` | [459](../../src/tool_backends.rs#L459) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or` | [459](../../src/tool_backends.rs#L459) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then` | [459](../../src/tool_backends.rs#L459) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name` | [459](../../src/tool_backends.rs#L459) | receiver-type-required |
| `assemble_tool_backends` | `Rc::new` | [464](../../src/tool_backends.rs#L464) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `RefCell::new` | [464](../../src/tool_backends.rs#L464) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange_tool_control_runtime(&selected, request, lines, stdout, cancellation)                 .map_err` | [465](../../src/tool_backends.rs#L465) | receiver-type-required |
| `assemble_tool_backends` | `exchange_tool_control_runtime` | [465](../../src/tool_backends.rs#L465) | [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L118) |
| `assemble_tool_backends` | `error.to_string` | [466](../../src/tool_backends.rs#L466) | receiver-type-required |
| `assemble_tool_backends` | `Rc::clone` | [468](../../src/tool_backends.rs#L468) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SupervisorControlBackend::new` | [469](../../src/tool_backends.rs#L469) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `session.clone` | [470](../../src/tool_backends.rs#L470) | receiver-type-required |
| `assemble_tool_backends` | `(fixed_exchange.borrow_mut())` | [471](../../src/tool_backends.rs#L471) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `fixed_exchange.borrow_mut` | [471](../../src/tool_backends.rs#L471) | receiver-type-required |
| `assemble_tool_backends` | `dynamic_backends.push` | [473](../../src/tool_backends.rs#L473) | receiver-type-required |
| `assemble_tool_backends` | `Box::new` | [473](../../src/tool_backends.rs#L473) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `DynamicSupervisorBackend::new` | [473](../../src/tool_backends.rs#L473) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `(exchange.borrow_mut())` | [475](../../src/tool_backends.rs#L475) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange.borrow_mut` | [475](../../src/tool_backends.rs#L475) | receiver-type-required |
| `assemble_tool_backends` | `allowed.clone` | [479](../../src/tool_backends.rs#L479) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)         .deferred_search_entries` | [482](../../src/tool_backends.rs#L482) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new` | [482](../../src/tool_backends.rs#L482) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `WorkflowBackend::new` | [484](../../src/tool_backends.rs#L484) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.bindings.goal_id.clone` | [487](../../src/tool_backends.rs#L487) | receiver-type-required |
| `assemble_tool_backends` | `preset_skill_catalog` | [488](../../src/tool_backends.rs#L488) | [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L503) |
| `assemble_tool_backends` | `frozen_skill_catalog` | [488](../../src/tool_backends.rs#L488) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Ok` | [491](../../src/tool_backends.rs#L491) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `thread_folder.join` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `preset_skill_catalog` | `store::read_genesis` | [508](../../src/tool_backends.rs#L508) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `preset_skill_catalog` | `Ok` | [509](../../src/tool_backends.rs#L509), [513](../../src/tool_backends.rs#L513), [518](../../src/tool_backends.rs#L518) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `serde_json::to_value` | [511](../../src/tool_backends.rs#L511) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `genesis.raw` | [511](../../src/tool_backends.rs#L511) | receiver-type-required |
| `preset_skill_catalog` | `raw.get` | [512](../../src/tool_backends.rs#L512) | receiver-type-required |
| `preset_skill_catalog` | `tools::ResolvedPreset::from_genesis` | [515](../../src/tool_backends.rs#L515) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `preset_skill_catalog` | `catalog.retain` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `preset_skill_catalog` | `skills.contains` | [516](../../src/tool_backends.rs#L516) | receiver-type-required |
| `invocation_write_roots` | `PathBuf::from` | [526](../../src/tool_backends.rs#L526), [589](../../src/tool_backends.rs#L589) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `profile             .config             .execution             .cwd             .first()             .ok_or` | [527](../../src/tool_backends.rs#L527) | receiver-type-required |
| `invocation_write_roots` | `profile             .config             .execution             .cwd             .first` | [527](../../src/tool_backends.rs#L527) | receiver-type-required |
| `invocation_write_roots` | `profile         .bindings         .dynamic_catalog         .tools         .iter()         .any` | [534](../../src/tool_backends.rs#L534) | receiver-type-required |
| `invocation_write_roots` | `profile         .bindings         .dynamic_catalog         .tools         .iter` | [534](../../src/tool_backends.rs#L534) | receiver-type-required |
| `invocation_write_roots` | `Ok` | [541](../../src/tool_backends.rs#L541), [627](../../src/tool_backends.rs#L627) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `profile             .instruction             .meet_workspace_policy` | [541](../../src/tool_backends.rs#L541) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.execution.policy)         .writable_roots         .into_iter()         .map(PathBuf::from)         .collect::<Vec<_>>` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.execution.policy)         .writable_roots         .into_iter()         .map` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.execution.policy)         .writable_roots         .into_iter` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `call.name.as_str` | [553](../../src/tool_backends.rs#L553) | receiver-type-required |
| `invocation_write_roots` | `call.arguments.get("writable_paths").is_none_or` | [555](../../src/tool_backends.rs#L555) | receiver-type-required |
| `invocation_write_roots` | `call.arguments.get` | [555](../../src/tool_backends.rs#L555) | receiver-type-required |
| `invocation_write_roots` | `value.is_null` | [556](../../src/tool_backends.rs#L556) | receiver-type-required |
| `invocation_write_roots` | `value.as_array().is_some_and` | [556](../../src/tool_backends.rs#L556) | receiver-type-required |
| `invocation_write_roots` | `value.as_array` | [556](../../src/tool_backends.rs#L556) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter()                 .map(&#124;path&#124; path.to_string_lossy().into_owned())                 .collect` | [561](../../src/tool_backends.rs#L561) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter()                 .map` | [561](../../src/tool_backends.rs#L561) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter` | [561](../../src/tool_backends.rs#L561) | receiver-type-required |
| `invocation_write_roots` | `path.to_string_lossy().into_owned` | [563](../../src/tool_backends.rs#L563) | receiver-type-required |
| `invocation_write_roots` | `path.to_string_lossy` | [563](../../src/tool_backends.rs#L563) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map(&#124;value&#124; {                 value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or("shell writable_paths must contain strings")             })             .collect::<Result<Vec<_>, _>>` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then` | [566](../../src/tool_backends.rs#L566) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get` | [566](../../src/tool_backends.rs#L566), [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or` | [573](../../src/tool_backends.rs#L573) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str()                     .map` | [573](../../src/tool_backends.rs#L573) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str` | [573](../../src/tool_backends.rs#L573) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map(&#124;path&#124; vec![path.to_owned()])             .unwrap_or_default` | [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map` | [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then` | [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots` | `Vec::new` | [585](../../src/tool_backends.rs#L585), [587](../../src/tool_backends.rs#L587) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `raw_path             .components()             .any` | [590](../../src/tool_backends.rs#L590) | receiver-type-required |
| `invocation_write_roots` | `raw_path             .components` | [590](../../src/tool_backends.rs#L590) | receiver-type-required |
| `invocation_write_roots` | `Err` | [594](../../src/tool_backends.rs#L594), [602](../../src/tool_backends.rs#L602), [609](../../src/tool_backends.rs#L609) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `format!("writable path {raw:?} contains parent traversal").into` | [594](../../src/tool_backends.rs#L594) | receiver-type-required |
| `invocation_write_roots` | `raw_path.is_absolute` | [596](../../src/tool_backends.rs#L596) | receiver-type-required |
| `invocation_write_roots` | `workspace.join` | [599](../../src/tool_backends.rs#L599) | receiver-type-required |
| `invocation_write_roots` | `allowed.iter().any` | [601](../../src/tool_backends.rs#L601) | receiver-type-required |
| `invocation_write_roots` | `allowed.iter` | [601](../../src/tool_backends.rs#L601) | receiver-type-required |
| `invocation_write_roots` | `target.starts_with` | [601](../../src/tool_backends.rs#L601) | receiver-type-required |
| `invocation_write_roots` | `format!(                 "writable path {} is outside the effective writable roots",                 target.display()             )             .into` | [602](../../src/tool_backends.rs#L602) | receiver-type-required |
| `invocation_write_roots` | `tools::protected_root_containing` | [608](../../src/tool_backends.rs#L608) | [tools::sandbox::protected_root_containing](../../../tools/src/sandbox.rs#L213) |
| `invocation_write_roots` | `format!(                 "writable path {} is inside the Kernel-owned directory {protected}",                 target.display()             )             .into` | [609](../../src/tool_backends.rs#L609) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent()                 .ok_or("apply_patch target has no writable parent")?                 .to_path_buf` | [616](../../src/tool_backends.rs#L616) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent()                 .ok_or` | [616](../../src/tool_backends.rs#L616) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent` | [616](../../src/tool_backends.rs#L616) | receiver-type-required |
| `invocation_write_roots` | `roots.push` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots` | `authority.to_string_lossy().into_owned` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots` | `authority.to_string_lossy` | [623](../../src/tool_backends.rs#L623) | receiver-type-required |
| `invocation_write_roots` | `roots.sort` | [625](../../src/tool_backends.rs#L625) | receiver-type-required |
| `invocation_write_roots` | `roots.dedup` | [626](../../src/tool_backends.rs#L626) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent()         .ok_or` | [633](../../src/tool_backends.rs#L633) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent` | [633](../../src/tool_backends.rs#L633) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name().is_some_and` | [636](../../src/tool_backends.rs#L636) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name` | [636](../../src/tool_backends.rs#L636) | receiver-type-required |
| `storage_root_for_thread_folder` | `Ok` | [637](../../src/tool_backends.rs#L637), [644](../../src/tool_backends.rs#L644) | external-constructor-callback-or-unresolved |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or("threads directory has no storage root")?             .to_path_buf` | [637](../../src/tool_backends.rs#L637) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or` | [637](../../src/tool_backends.rs#L637) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent` | [637](../../src/tool_backends.rs#L637) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.to_path_buf` | [644](../../src/tool_backends.rs#L644) | receiver-type-required |
| `session_permission_policy` | `PermissionModePolicy::for_session_folder` | [651](../../src/tool_backends.rs#L651) | external-constructor-callback-or-unresolved |
| `entry` | `name.to_owned` | [671](../../src/tool_backends.rs#L671) | receiver-type-required |
| `entry` | `String::new` | [672](../../src/tool_backends.rs#L672) | external-constructor-callback-or-unresolved |
| `entry` | `Vec::new` | [673](../../src/tool_backends.rs#L673) | external-constructor-callback-or-unresolved |
| `entry` | `"d".to_owned` | [674](../../src/tool_backends.rs#L674) | receiver-type-required |
| `entry` | `IJsonValue::parse(b"{}").unwrap` | [675](../../src/tool_backends.rs#L675) | receiver-type-required |
| `entry` | `IJsonValue::parse` | [675](../../src/tool_backends.rs#L675) | external-constructor-callback-or-unresolved |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap()             .extend` | [686](../../src/tool_backends.rs#L686) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap` | [686](../../src/tool_backends.rs#L686) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut` | [686](../../src/tool_backends.rs#L686) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap().clone` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `write_genesis` | `opening.as_object` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes().unwrap` | [690](../../src/tool_backends.rs#L690) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes` | [690](../../src/tool_backends.rs#L690) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap` | [690](../../src/tool_backends.rs#L690) | receiver-type-required |
| `write_genesis` | `make_event` | [690](../../src/tool_backends.rs#L690) | external-constructor-callback-or-unresolved |
| `write_genesis` | `bytes.push` | [691](../../src/tool_backends.rs#L691) | receiver-type-required |
| `write_genesis` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [692](../../src/tool_backends.rs#L692) | receiver-type-required |
| `write_genesis` | `std::fs::write` | [692](../../src/tool_backends.rs#L692) | external-constructor-callback-or-unresolved |
| `write_genesis` | `folder.join` | [692](../../src/tool_backends.rs#L692) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir().unwrap` | [697](../../src/tool_backends.rs#L697) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir` | [697](../../src/tool_backends.rs#L697) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `write_genesis` | [699](../../src/tool_backends.rs#L699), [712](../../src/tool_backends.rs#L712) | [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L679) |
| `a_preset_narrows_the_skill_catalog` | `folder.path` | [699](../../src/tool_backends.rs#L699), [712](../../src/tool_backends.rs#L712), [713](../../src/tool_backends.rs#L713) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request(             &json!({"skills":["style-guide","removed-later"]}),             tools::PresetBase::General,             &Skills,         )         .unwrap` | [706](../../src/tool_backends.rs#L706) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request` | [706](../../src/tool_backends.rs#L706) | [tools::preset::ResolvedPreset::resolve_request](../../../tools/src/preset.rs#L721) |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog(catalog(), folder.path()).unwrap` | [713](../../src/tool_backends.rs#L713) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog` | [713](../../src/tool_backends.rs#L713) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `catalog` | [713](../../src/tool_backends.rs#L713) | external-constructor-callback-or-unresolved |
