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
| [tekes-worker::tool_backends::ToolBackendRuntime](../../src/tool_backends.rs#L245) | struct_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::execution_access_for_profile](../../src/tool_backends.rs#L251) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::assemble_tool_backends](../../src/tool_backends.rs#L262) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L536) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::invocation_write_roots](../../src/tool_backends.rs#L555) | function_item | `pub(crate)` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::invocation_write_roots_for_access](../../src/tool_backends.rs#L563) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L682) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::session_permission_policy](../../src/tool_backends.rs#L702) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::preset_skill_tests::Skills](../../src/tool_backends.rs#L714) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::Skills::skill_known](../../src/tool_backends.rs#L716) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::entry](../../src/tool_backends.rs#L721) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L731) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::tool_backends::preset_skill_tests::a_preset_narrows_the_skill_catalog](../../src/tool_backends.rs#L748) | function_item | `private` | test; #[cfg(test)] |

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

<details><summary>Functions 1–17: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["store::tail::read_genesis"]
  n1["tools::preset::ResolvedPreset::from_genesis"]
  n2["tools::sandbox::exposed_protected_roots"]
  n3["tools::sandbox::protected_root_containing"]
  n4["tekes-worker::tool_backends::exchange_tool_control_runtime"]
  n5["tekes-worker::tool_backends::AllowedDynamicBackend::execute"]
  n6["tekes-worker::tool_backends::tool_catalog_context"]
  n7["tekes-worker::tool_backends::execution_access_for_profile"]
  n8["tekes-worker::tool_backends::assemble_tool_backends"]
  n9["tekes-worker::tool_backends::AllowedToolBackend::supports"]
  n10["tekes-worker::tool_backends::AllowedToolBackend::execute"]
  n11["tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval"]
  n12["tekes-worker::tool_backends::preset_skill_catalog"]
  n13["tekes-worker::tool_backends::invocation_write_roots_for_access"]
  n14["tekes-worker::tool_backends::DeferredChildBackend::supports"]
  n15["tekes-worker::tool_backends::storage_root_for_thread_folder"]
  n16["tekes-worker::tool_backends::DeferredChildBackend::execute"]
  n17["tekes-worker::tool_backends::session_permission_policy"]
  n18["tekes-worker::tool_backends::ReplayedChildBackend::supports"]
  n19["tekes-worker::tool_backends::ReplayedChildBackend::execute"]
  n20["tekes-worker::tool_backends::AllowedDynamicBackend::supports"]
  n8 --> n2
  n8 --> n4
  n8 --> n12
  n8 --> n15
  n12 --> n0
  n12 --> n1
  n13 --> n3
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
| `execution_access_for_profile` | `mode.execution_access` | [258](../../src/tool_backends.rs#L258) | receiver-type-required |
| `assemble_tool_backends` | `effective_allowed_tools` | [284](../../src/tool_backends.rs#L284) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `ToolBackendRouter::default` | [285](../../src/tool_backends.rs#L285) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root_for_thread_folder` | [286](../../src/tool_backends.rs#L286) | [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L682) |
| `assemble_tool_backends` | `profile         .instruction         .meet_workspace_policy` | [288](../../src/tool_backends.rs#L288) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first()         .ok_or` | [295](../../src/tool_backends.rs#L295) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .first` | [295](../../src/tool_backends.rs#L295) | receiver-type-required |
| `assemble_tool_backends` | `sibling_helper_path` | [301](../../src/tool_backends.rs#L301) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.execution.cwd.clone` | [302](../../src/tool_backends.rs#L302) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.extend` | [303](../../src/tool_backends.rs#L303) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .cloned` | [304](../../src/tool_backends.rs#L304) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter` | [304](../../src/tool_backends.rs#L304), [417](../../src/tool_backends.rs#L417) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.sort` | [312](../../src/tool_backends.rs#L312) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.dedup` | [313](../../src/tool_backends.rs#L313) | receiver-type-required |
| `assemble_tool_backends` | `invocation_write_roots.to_vec` | [314](../../src/tool_backends.rs#L314) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.sort` | [315](../../src/tool_backends.rs#L315) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.dedup` | [316](../../src/tool_backends.rs#L316) | receiver-type-required |
| `assemble_tool_backends` | `tools::exposed_protected_roots` | [317](../../src/tool_backends.rs#L317) | [tools::sandbox::exposed_protected_roots](../../../tools/src/sandbox.rs#L215) |
| `assemble_tool_backends` | `profile.config.protected_roots` | [318](../../src/tool_backends.rs#L318) | receiver-type-required |
| `assemble_tool_backends` | `execution_access.policy` | [321](../../src/tool_backends.rs#L321) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.clone` | [323](../../src/tool_backends.rs#L323) | receiver-type-required |
| `assemble_tool_backends` | `probe_backend` | [340](../../src/tool_backends.rs#L340) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map(&#124;(index, path)&#124; {             (                 if index == 0 {                     "workspace".to_owned()                 } else {                     format!("workspace-{index}")                 },                 PathBuf::from(path),             )         })         .collect::<Vec<_>>` | [341](../../src/tool_backends.rs#L341) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .map` | [341](../../src/tool_backends.rs#L341) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate` | [341](../../src/tool_backends.rs#L341), [439](../../src/tool_backends.rs#L439) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter` | [341](../../src/tool_backends.rs#L341), [439](../../src/tool_backends.rs#L439) | receiver-type-required |
| `assemble_tool_backends` | `"workspace".to_owned` | [350](../../src/tool_backends.rs#L350) | receiver-type-required |
| `assemble_tool_backends` | `PathBuf::from` | [354](../../src/tool_backends.rs#L354), [359](../../src/tool_backends.rs#L359), [448](../../src/tool_backends.rs#L448), [454](../../src/tool_backends.rs#L454), [473](../../src/tool_backends.rs#L473) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `roots.push` | [359](../../src/tool_backends.rs#L359) | receiver-type-required |
| `assemble_tool_backends` | `"host".to_owned` | [359](../../src/tool_backends.rs#L359), [453](../../src/tool_backends.rs#L453) | receiver-type-required |
| `assemble_tool_backends` | `helper_path.is_file` | [361](../../src/tool_backends.rs#L361) | receiver-type-required |
| `assemble_tool_backends` | `HelperClient::for_execution_access` | [365](../../src/tool_backends.rs#L365) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `client.scratch_path().map` | [372](../../src/tool_backends.rs#L372) | receiver-type-required |
| `assemble_tool_backends` | `client.scratch_path` | [372](../../src/tool_backends.rs#L372) | receiver-type-required |
| `assemble_tool_backends` | `Some` | [374](../../src/tool_backends.rs#L374), [397](../../src/tool_backends.rs#L397) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Arc::new` | [374](../../src/tool_backends.rs#L374), [385](../../src/tool_backends.rs#L385), [397](../../src/tool_backends.rs#L397), [400](../../src/tool_backends.rs#L400), [471](../../src/tool_backends.rs#L471) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Vec::new` | [380](../../src/tool_backends.rs#L380) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose()?         .map` | [381](../../src/tool_backends.rs#L381) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose` | [381](../../src/tool_backends.rs#L381) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map` | [381](../../src/tool_backends.rs#L381) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref` | [381](../../src/tool_backends.rs#L381) | receiver-type-required |
| `assemble_tool_backends` | `DurableArtifactVersions::new` | [383](../../src/tool_backends.rs#L383) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root.join` | [383](../../src/tool_backends.rs#L383), [481](../../src/tool_backends.rs#L481) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then(&#124;&#124; BoundedHttpClient::new(HttpLimits::default()))         .transpose` | [386](../../src/tool_backends.rs#L386) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then` | [386](../../src/tool_backends.rs#L386) | receiver-type-required |
| `assemble_tool_backends` | `BoundedHttpClient::new` | [388](../../src/tool_backends.rs#L388) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `HttpLimits::default` | [388](../../src/tool_backends.rs#L388) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.providers.web_search.as_ref` | [393](../../src/tool_backends.rs#L393) | receiver-type-required |
| `assemble_tool_backends` | `config.clone` | [398](../../src/tool_backends.rs#L398) | receiver-type-required |
| `assemble_tool_backends` | `Arc::clone` | [399](../../src/tool_backends.rs#L399) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SystemToolConfig::workspace` | [405](../../src/tool_backends.rs#L405) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `system_config             .environment             .insert` | [407](../../src/tool_backends.rs#L407), [412](../../src/tool_backends.rs#L412), [435](../../src/tool_backends.rs#L435) | receiver-type-required |
| `assemble_tool_backends` | `"PYTHONDONTWRITEBYTECODE".into` | [409](../../src/tool_backends.rs#L409) | receiver-type-required |
| `assemble_tool_backends` | `"1".into` | [409](../../src/tool_backends.rs#L409) | receiver-type-required |
| `assemble_tool_backends` | `"TMPDIR".into` | [414](../../src/tool_backends.rs#L414) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.policy.toolchain_roots.is_empty` | [416](../../src/tool_backends.rs#L416) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map(&#124;root&#124; {                 std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned()             })             .collect::<Vec<_>>` | [417](../../src/tool_backends.rs#L417) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .execution             .policy             .toolchain_roots             .iter()             .map` | [417](../../src/tool_backends.rs#L417) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned` | [424](../../src/tool_backends.rs#L424) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy` | [424](../../src/tool_backends.rs#L424) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join` | [424](../../src/tool_backends.rs#L424) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new` | [424](../../src/tool_backends.rs#L424) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `bins.extend` | [430](../../src/tool_backends.rs#L430) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter()                 .map` | [431](../../src/tool_backends.rs#L431) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter` | [431](../../src/tool_backends.rs#L431) | receiver-type-required |
| `assemble_tool_backends` | `"PATH".into` | [437](../../src/tool_backends.rs#L437) | receiver-type-required |
| `assemble_tool_backends` | `bins.join` | [437](../../src/tool_backends.rs#L437) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map(&#124;(index, path)&#124; engine::RootMount {             name: format!("workspace-{index}"),             path: PathBuf::from(path),         })         .collect` | [439](../../src/tool_backends.rs#L439) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip(1)         .map` | [439](../../src/tool_backends.rs#L439) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .execution         .cwd         .iter()         .enumerate()         .skip` | [439](../../src/tool_backends.rs#L439) | receiver-type-required |
| `assemble_tool_backends` | `system_config.extra_roots.push` | [452](../../src/tool_backends.rs#L452) | receiver-type-required |
| `assemble_tool_backends` | `SystemToolBackend::new` | [457](../../src/tool_backends.rs#L457) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `cancellation.tools.clone` | [463](../../src/tool_backends.rs#L463) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or("worker executable has no parent directory")?         .join` | [465](../../src/tool_backends.rs#L465) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or` | [465](../../src/tool_backends.rs#L465) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent` | [465](../../src/tool_backends.rs#L465) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe` | [465](../../src/tool_backends.rs#L465) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `workspace_helper.is_file` | [469](../../src/tool_backends.rs#L469) | receiver-type-required |
| `assemble_tool_backends` | `system_backend.with_edit_recorder` | [471](../../src/tool_backends.rs#L471) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map(PathBuf::from)                     .collect` | [474](../../src/tool_backends.rs#L474) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter()                     .map` | [474](../../src/tool_backends.rs#L474) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .execution                     .cwd                     .iter` | [474](../../src/tool_backends.rs#L474) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.execution.session_id.clone` | [482](../../src/tool_backends.rs#L482), [519](../../src/tool_backends.rs#L519) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or("thread folder has no session identity")?                     .to_owned` | [483](../../src/tool_backends.rs#L483) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or` | [483](../../src/tool_backends.rs#L483) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then` | [483](../../src/tool_backends.rs#L483) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name` | [483](../../src/tool_backends.rs#L483) | receiver-type-required |
| `assemble_tool_backends` | `name.to_str` | [485](../../src/tool_backends.rs#L485), [494](../../src/tool_backends.rs#L494) | receiver-type-required |
| `assemble_tool_backends` | `router.push` | [490](../../src/tool_backends.rs#L490), [502](../../src/tool_backends.rs#L502), [517](../../src/tool_backends.rs#L517) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or("thread folder has no UTF-8 session UUID")?             .to_owned` | [492](../../src/tool_backends.rs#L492) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or` | [492](../../src/tool_backends.rs#L492) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then` | [492](../../src/tool_backends.rs#L492) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name` | [492](../../src/tool_backends.rs#L492) | receiver-type-required |
| `assemble_tool_backends` | `Rc::new` | [497](../../src/tool_backends.rs#L497) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `RefCell::new` | [497](../../src/tool_backends.rs#L497) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange_tool_control_runtime(&selected, request, lines, stdout, cancellation)                 .map_err` | [498](../../src/tool_backends.rs#L498) | receiver-type-required |
| `assemble_tool_backends` | `exchange_tool_control_runtime` | [498](../../src/tool_backends.rs#L498) | [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L118) |
| `assemble_tool_backends` | `error.to_string` | [499](../../src/tool_backends.rs#L499) | receiver-type-required |
| `assemble_tool_backends` | `Rc::clone` | [501](../../src/tool_backends.rs#L501) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SupervisorControlBackend::new` | [502](../../src/tool_backends.rs#L502) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `session.clone` | [503](../../src/tool_backends.rs#L503) | receiver-type-required |
| `assemble_tool_backends` | `(fixed_exchange.borrow_mut())` | [504](../../src/tool_backends.rs#L504) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `fixed_exchange.borrow_mut` | [504](../../src/tool_backends.rs#L504) | receiver-type-required |
| `assemble_tool_backends` | `dynamic_backends.push` | [506](../../src/tool_backends.rs#L506) | receiver-type-required |
| `assemble_tool_backends` | `Box::new` | [506](../../src/tool_backends.rs#L506) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `DynamicSupervisorBackend::new` | [506](../../src/tool_backends.rs#L506) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `(exchange.borrow_mut())` | [508](../../src/tool_backends.rs#L508) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange.borrow_mut` | [508](../../src/tool_backends.rs#L508) | receiver-type-required |
| `assemble_tool_backends` | `allowed.clone` | [512](../../src/tool_backends.rs#L512) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)         .deferred_search_entries` | [515](../../src/tool_backends.rs#L515) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new` | [515](../../src/tool_backends.rs#L515) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `WorkflowBackend::new` | [517](../../src/tool_backends.rs#L517) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.bindings.goal_id.clone` | [520](../../src/tool_backends.rs#L520) | receiver-type-required |
| `assemble_tool_backends` | `preset_skill_catalog` | [521](../../src/tool_backends.rs#L521) | [tekes-worker::tool_backends::preset_skill_catalog](../../src/tool_backends.rs#L536) |
| `assemble_tool_backends` | `frozen_skill_catalog` | [521](../../src/tool_backends.rs#L521) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Ok` | [524](../../src/tool_backends.rs#L524) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `thread_folder.join` | [540](../../src/tool_backends.rs#L540) | receiver-type-required |
| `preset_skill_catalog` | `store::read_genesis` | [541](../../src/tool_backends.rs#L541) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `preset_skill_catalog` | `Ok` | [542](../../src/tool_backends.rs#L542), [546](../../src/tool_backends.rs#L546), [551](../../src/tool_backends.rs#L551) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `serde_json::to_value` | [544](../../src/tool_backends.rs#L544) | external-constructor-callback-or-unresolved |
| `preset_skill_catalog` | `genesis.raw` | [544](../../src/tool_backends.rs#L544) | receiver-type-required |
| `preset_skill_catalog` | `raw.get` | [545](../../src/tool_backends.rs#L545) | receiver-type-required |
| `preset_skill_catalog` | `tools::ResolvedPreset::from_genesis` | [548](../../src/tool_backends.rs#L548) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `preset_skill_catalog` | `catalog.retain` | [549](../../src/tool_backends.rs#L549) | receiver-type-required |
| `preset_skill_catalog` | `skills.contains` | [549](../../src/tool_backends.rs#L549) | receiver-type-required |
| `invocation_write_roots` | `invocation_write_roots_for_access` | [560](../../src/tool_backends.rs#L560) | [tekes-worker::tool_backends::invocation_write_roots_for_access](../../src/tool_backends.rs#L563) |
| `invocation_write_roots_for_access` | `PathBuf::from` | [571](../../src/tool_backends.rs#L571), [641](../../src/tool_backends.rs#L641) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `profile             .config             .execution             .cwd             .first()             .ok_or` | [572](../../src/tool_backends.rs#L572) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .config             .execution             .cwd             .first` | [572](../../src/tool_backends.rs#L572) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile         .bindings         .dynamic_catalog         .tools         .iter()         .any` | [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile         .bindings         .dynamic_catalog         .tools         .iter` | [579](../../src/tool_backends.rs#L579) | receiver-type-required |
| `invocation_write_roots_for_access` | `Ok` | [587](../../src/tool_backends.rs#L587), [589](../../src/tool_backends.rs#L589), [679](../../src/tool_backends.rs#L679) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy` | [589](../../src/tool_backends.rs#L589), [597](../../src/tool_backends.rs#L597) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter()             .map(PathBuf::from)             .collect::<Vec<_>>` | [597](../../src/tool_backends.rs#L597) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter()             .map` | [597](../../src/tool_backends.rs#L597) | receiver-type-required |
| `invocation_write_roots_for_access` | `profile             .instruction             .meet_workspace_policy(&profile.config.execution.policy)             .writable_roots             .into_iter` | [597](../../src/tool_backends.rs#L597) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.name.as_str` | [605](../../src/tool_backends.rs#L605) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.arguments.get("writable_paths").is_none_or` | [607](../../src/tool_backends.rs#L607) | receiver-type-required |
| `invocation_write_roots_for_access` | `call.arguments.get` | [607](../../src/tool_backends.rs#L607) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.is_null` | [608](../../src/tool_backends.rs#L608) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.as_array().is_some_and` | [608](../../src/tool_backends.rs#L608) | receiver-type-required |
| `invocation_write_roots_for_access` | `value.as_array` | [608](../../src/tool_backends.rs#L608) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter()                 .map(&#124;path&#124; path.to_string_lossy().into_owned())                 .collect` | [613](../../src/tool_backends.rs#L613) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter()                 .map` | [613](../../src/tool_backends.rs#L613) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed                 .iter` | [613](../../src/tool_backends.rs#L613) | receiver-type-required |
| `invocation_write_roots_for_access` | `path.to_string_lossy().into_owned` | [615](../../src/tool_backends.rs#L615) | receiver-type-required |
| `invocation_write_roots_for_access` | `path.to_string_lossy` | [615](../../src/tool_backends.rs#L615) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map(&#124;value&#124; {                 value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or("shell writable_paths must contain strings")             })             .collect::<Result<Vec<_>, _>>` | [618](../../src/tool_backends.rs#L618) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map` | [618](../../src/tool_backends.rs#L618) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten` | [618](../../src/tool_backends.rs#L618) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter` | [618](../../src/tool_backends.rs#L618) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("writable_paths")             .and_then` | [618](../../src/tool_backends.rs#L618) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get` | [618](../../src/tool_backends.rs#L618), [631](../../src/tool_backends.rs#L631) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or` | [625](../../src/tool_backends.rs#L625) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str()                     .map` | [625](../../src/tool_backends.rs#L625) | receiver-type-required |
| `invocation_write_roots_for_access` | `value                     .as_str` | [625](../../src/tool_backends.rs#L625) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map(&#124;path&#124; vec![path.to_owned()])             .unwrap_or_default` | [631](../../src/tool_backends.rs#L631) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map` | [631](../../src/tool_backends.rs#L631) | receiver-type-required |
| `invocation_write_roots_for_access` | `call             .arguments             .get("path")             .and_then` | [631](../../src/tool_backends.rs#L631) | receiver-type-required |
| `invocation_write_roots_for_access` | `Vec::new` | [637](../../src/tool_backends.rs#L637), [639](../../src/tool_backends.rs#L639) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `raw_path             .components()             .any` | [642](../../src/tool_backends.rs#L642) | receiver-type-required |
| `invocation_write_roots_for_access` | `raw_path             .components` | [642](../../src/tool_backends.rs#L642) | receiver-type-required |
| `invocation_write_roots_for_access` | `Err` | [646](../../src/tool_backends.rs#L646), [654](../../src/tool_backends.rs#L654), [661](../../src/tool_backends.rs#L661) | external-constructor-callback-or-unresolved |
| `invocation_write_roots_for_access` | `format!("writable path {raw:?} contains parent traversal").into` | [646](../../src/tool_backends.rs#L646) | receiver-type-required |
| `invocation_write_roots_for_access` | `raw_path.is_absolute` | [648](../../src/tool_backends.rs#L648) | receiver-type-required |
| `invocation_write_roots_for_access` | `workspace.join` | [651](../../src/tool_backends.rs#L651) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed.iter().any` | [653](../../src/tool_backends.rs#L653) | receiver-type-required |
| `invocation_write_roots_for_access` | `allowed.iter` | [653](../../src/tool_backends.rs#L653) | receiver-type-required |
| `invocation_write_roots_for_access` | `target.starts_with` | [653](../../src/tool_backends.rs#L653) | receiver-type-required |
| `invocation_write_roots_for_access` | `format!(                 "writable path {} is outside the effective writable roots",                 target.display()             )             .into` | [654](../../src/tool_backends.rs#L654) | receiver-type-required |
| `invocation_write_roots_for_access` | `tools::protected_root_containing` | [660](../../src/tool_backends.rs#L660) | [tools::sandbox::protected_root_containing](../../../tools/src/sandbox.rs#L241) |
| `invocation_write_roots_for_access` | `format!(                 "writable path {} is inside the Kernel-owned directory {protected}",                 target.display()             )             .into` | [661](../../src/tool_backends.rs#L661) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent()                 .ok_or("apply_patch target has no writable parent")?                 .to_path_buf` | [668](../../src/tool_backends.rs#L668) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent()                 .ok_or` | [668](../../src/tool_backends.rs#L668) | receiver-type-required |
| `invocation_write_roots_for_access` | `target                 .parent` | [668](../../src/tool_backends.rs#L668) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.push` | [675](../../src/tool_backends.rs#L675) | receiver-type-required |
| `invocation_write_roots_for_access` | `authority.to_string_lossy().into_owned` | [675](../../src/tool_backends.rs#L675) | receiver-type-required |
| `invocation_write_roots_for_access` | `authority.to_string_lossy` | [675](../../src/tool_backends.rs#L675) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.sort` | [677](../../src/tool_backends.rs#L677) | receiver-type-required |
| `invocation_write_roots_for_access` | `roots.dedup` | [678](../../src/tool_backends.rs#L678) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent()         .ok_or` | [685](../../src/tool_backends.rs#L685) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent` | [685](../../src/tool_backends.rs#L685) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name().is_some_and` | [688](../../src/tool_backends.rs#L688) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name` | [688](../../src/tool_backends.rs#L688) | receiver-type-required |
| `storage_root_for_thread_folder` | `Ok` | [689](../../src/tool_backends.rs#L689), [696](../../src/tool_backends.rs#L696) | external-constructor-callback-or-unresolved |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or("threads directory has no storage root")?             .to_path_buf` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent` | [689](../../src/tool_backends.rs#L689) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.to_path_buf` | [696](../../src/tool_backends.rs#L696) | receiver-type-required |
| `session_permission_policy` | `PermissionModePolicy::for_session_folder` | [703](../../src/tool_backends.rs#L703) | external-constructor-callback-or-unresolved |
| `entry` | `name.to_owned` | [723](../../src/tool_backends.rs#L723) | receiver-type-required |
| `entry` | `String::new` | [724](../../src/tool_backends.rs#L724) | external-constructor-callback-or-unresolved |
| `entry` | `Vec::new` | [725](../../src/tool_backends.rs#L725) | external-constructor-callback-or-unresolved |
| `entry` | `"d".to_owned` | [726](../../src/tool_backends.rs#L726) | receiver-type-required |
| `entry` | `IJsonValue::parse(b"{}").unwrap` | [727](../../src/tool_backends.rs#L727) | receiver-type-required |
| `entry` | `IJsonValue::parse` | [727](../../src/tool_backends.rs#L727) | external-constructor-callback-or-unresolved |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap()             .extend` | [738](../../src/tool_backends.rs#L738) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut()             .unwrap` | [738](../../src/tool_backends.rs#L738) | receiver-type-required |
| `write_genesis` | `genesis             .as_object_mut` | [738](../../src/tool_backends.rs#L738) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap().clone` | [741](../../src/tool_backends.rs#L741) | receiver-type-required |
| `write_genesis` | `opening.as_object().unwrap` | [741](../../src/tool_backends.rs#L741) | receiver-type-required |
| `write_genesis` | `opening.as_object` | [741](../../src/tool_backends.rs#L741) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes().unwrap` | [742](../../src/tool_backends.rs#L742) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap().canonical_bytes` | [742](../../src/tool_backends.rs#L742) | receiver-type-required |
| `write_genesis` | `make_event(genesis).unwrap` | [742](../../src/tool_backends.rs#L742) | receiver-type-required |
| `write_genesis` | `make_event` | [742](../../src/tool_backends.rs#L742) | external-constructor-callback-or-unresolved |
| `write_genesis` | `bytes.push` | [743](../../src/tool_backends.rs#L743) | receiver-type-required |
| `write_genesis` | `std::fs::write(folder.join("main.jsonl"), bytes).unwrap` | [744](../../src/tool_backends.rs#L744) | receiver-type-required |
| `write_genesis` | `std::fs::write` | [744](../../src/tool_backends.rs#L744) | external-constructor-callback-or-unresolved |
| `write_genesis` | `folder.join` | [744](../../src/tool_backends.rs#L744) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir().unwrap` | [749](../../src/tool_backends.rs#L749) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tempfile::tempdir` | [749](../../src/tool_backends.rs#L749) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `write_genesis` | [751](../../src/tool_backends.rs#L751), [764](../../src/tool_backends.rs#L764) | [tekes-worker::tool_backends::preset_skill_tests::write_genesis](../../src/tool_backends.rs#L731) |
| `a_preset_narrows_the_skill_catalog` | `folder.path` | [751](../../src/tool_backends.rs#L751), [764](../../src/tool_backends.rs#L764), [765](../../src/tool_backends.rs#L765) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request(             &json!({"skills":["style-guide","removed-later"]}),             tools::PresetBase::General,             &Skills,         )         .unwrap` | [758](../../src/tool_backends.rs#L758) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `tools::ResolvedPreset::resolve_request` | [758](../../src/tool_backends.rs#L758) | [tools::preset::ResolvedPreset::resolve_request](../../../tools/src/preset.rs#L721) |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog(catalog(), folder.path()).unwrap` | [765](../../src/tool_backends.rs#L765) | receiver-type-required |
| `a_preset_narrows_the_skill_catalog` | `preset_skill_catalog` | [765](../../src/tool_backends.rs#L765) | external-constructor-callback-or-unresolved |
| `a_preset_narrows_the_skill_catalog` | `catalog` | [765](../../src/tool_backends.rs#L765) | external-constructor-callback-or-unresolved |
