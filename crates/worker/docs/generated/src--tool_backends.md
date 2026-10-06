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
| [tekes-worker::tool_backends::invocation_write_roots](../../src/tool_backends.rs#L495) | function_item | `pub(crate)` |  |
| [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L596) | function_item | `private` |  |
| [tekes-worker::tool_backends::session_permission_policy](../../src/tool_backends.rs#L616) | function_item | `pub(crate)` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–15: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-worker::tool_backends::exchange_tool_control_runtime"]
  n1["tekes-worker::tool_backends::AllowedDynamicBackend::execute"]
  n2["tekes-worker::tool_backends::tool_catalog_context"]
  n3["tekes-worker::tool_backends::assemble_tool_backends"]
  n4["tekes-worker::tool_backends::AllowedToolBackend::supports"]
  n5["tekes-worker::tool_backends::AllowedToolBackend::execute"]
  n6["tekes-worker::tool_backends::AllowedToolBackend::resume_after_approval"]
  n7["tekes-worker::tool_backends::invocation_write_roots"]
  n8["tekes-worker::tool_backends::storage_root_for_thread_folder"]
  n9["tekes-worker::tool_backends::session_permission_policy"]
  n10["tekes-worker::tool_backends::DeferredChildBackend::supports"]
  n11["tekes-worker::tool_backends::DeferredChildBackend::execute"]
  n12["tekes-worker::tool_backends::ReplayedChildBackend::supports"]
  n13["tekes-worker::tool_backends::ReplayedChildBackend::execute"]
  n14["tekes-worker::tool_backends::AllowedDynamicBackend::supports"]
  n3 --> n0
  n3 --> n8
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
| `assemble_tool_backends` | `storage_root_for_thread_folder` | [273](../../src/tool_backends.rs#L273) | [tekes-worker::tool_backends::storage_root_for_thread_folder](../../src/tool_backends.rs#L596) |
| `assemble_tool_backends` | `profile         .instruction         .meet_workspace_policy` | [275](../../src/tool_backends.rs#L275) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .first()         .ok_or` | [278](../../src/tool_backends.rs#L278) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .first` | [278](../../src/tool_backends.rs#L278) | receiver-type-required |
| `assemble_tool_backends` | `sibling_helper_path` | [284](../../src/tool_backends.rs#L284) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.workspace.cwd.clone` | [285](../../src/tool_backends.rs#L285) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.extend` | [286](../../src/tool_backends.rs#L286) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .workspace             .policy             .toolchain_roots             .iter()             .cloned` | [287](../../src/tool_backends.rs#L287) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .workspace             .policy             .toolchain_roots             .iter` | [287](../../src/tool_backends.rs#L287), [385](../../src/tool_backends.rs#L385) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.sort` | [295](../../src/tool_backends.rs#L295) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.dedup` | [296](../../src/tool_backends.rs#L296) | receiver-type-required |
| `assemble_tool_backends` | `invocation_write_roots.to_vec` | [297](../../src/tool_backends.rs#L297) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.sort` | [298](../../src/tool_backends.rs#L298) | receiver-type-required |
| `assemble_tool_backends` | `write_roots.dedup` | [299](../../src/tool_backends.rs#L299) | receiver-type-required |
| `assemble_tool_backends` | `read_roots.clone` | [302](../../src/tool_backends.rs#L302) | receiver-type-required |
| `assemble_tool_backends` | `probe_backend` | [318](../../src/tool_backends.rs#L318) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate()         .map(&#124;(index, path)&#124; {             (                 if index == 0 {                     "workspace".to_owned()                 } else {                     format!("workspace-{index}")                 },                 PathBuf::from(path),             )         })         .collect::<Vec<_>>` | [319](../../src/tool_backends.rs#L319) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate()         .map` | [319](../../src/tool_backends.rs#L319) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate` | [319](../../src/tool_backends.rs#L319), [407](../../src/tool_backends.rs#L407) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter` | [319](../../src/tool_backends.rs#L319), [407](../../src/tool_backends.rs#L407) | receiver-type-required |
| `assemble_tool_backends` | `"workspace".to_owned` | [328](../../src/tool_backends.rs#L328) | receiver-type-required |
| `assemble_tool_backends` | `PathBuf::from` | [332](../../src/tool_backends.rs#L332), [416](../../src/tool_backends.rs#L416), [435](../../src/tool_backends.rs#L435) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper_path.is_file` | [337](../../src/tool_backends.rs#L337) | receiver-type-required |
| `assemble_tool_backends` | `HelperClient::sandboxed_with_scratch` | [339](../../src/tool_backends.rs#L339) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `client.scratch_path().map` | [340](../../src/tool_backends.rs#L340) | receiver-type-required |
| `assemble_tool_backends` | `client.scratch_path` | [340](../../src/tool_backends.rs#L340) | receiver-type-required |
| `assemble_tool_backends` | `Some` | [342](../../src/tool_backends.rs#L342), [365](../../src/tool_backends.rs#L365) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Arc::new` | [342](../../src/tool_backends.rs#L342), [353](../../src/tool_backends.rs#L353), [365](../../src/tool_backends.rs#L365), [368](../../src/tool_backends.rs#L368), [433](../../src/tool_backends.rs#L433) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Vec::new` | [348](../../src/tool_backends.rs#L348) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose()?         .map` | [349](../../src/tool_backends.rs#L349) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map(&#124;_&#124; DurableArtifactVersions::new(storage_root.join("tool-state")))         .transpose` | [349](../../src/tool_backends.rs#L349) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref()         .map` | [349](../../src/tool_backends.rs#L349) | receiver-type-required |
| `assemble_tool_backends` | `helper         .as_ref` | [349](../../src/tool_backends.rs#L349) | receiver-type-required |
| `assemble_tool_backends` | `DurableArtifactVersions::new` | [351](../../src/tool_backends.rs#L351) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `storage_root.join` | [351](../../src/tool_backends.rs#L351), [443](../../src/tool_backends.rs#L443) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then(&#124;&#124; BoundedHttpClient::new(HttpLimits::default()))         .transpose` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `effective         .network         .then` | [354](../../src/tool_backends.rs#L354) | receiver-type-required |
| `assemble_tool_backends` | `BoundedHttpClient::new` | [356](../../src/tool_backends.rs#L356) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `HttpLimits::default` | [356](../../src/tool_backends.rs#L356) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.config.providers.web_search.as_ref` | [361](../../src/tool_backends.rs#L361) | receiver-type-required |
| `assemble_tool_backends` | `config.clone` | [366](../../src/tool_backends.rs#L366) | receiver-type-required |
| `assemble_tool_backends` | `Arc::clone` | [367](../../src/tool_backends.rs#L367) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SystemToolConfig::workspace` | [373](../../src/tool_backends.rs#L373) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `system_config             .environment             .insert` | [375](../../src/tool_backends.rs#L375), [380](../../src/tool_backends.rs#L380), [403](../../src/tool_backends.rs#L403) | receiver-type-required |
| `assemble_tool_backends` | `"PYTHONDONTWRITEBYTECODE".into` | [377](../../src/tool_backends.rs#L377) | receiver-type-required |
| `assemble_tool_backends` | `"1".into` | [377](../../src/tool_backends.rs#L377) | receiver-type-required |
| `assemble_tool_backends` | `"TMPDIR".into` | [382](../../src/tool_backends.rs#L382) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.workspace.policy.toolchain_roots.is_empty` | [384](../../src/tool_backends.rs#L384) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .workspace             .policy             .toolchain_roots             .iter()             .map(&#124;root&#124; {                 std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned()             })             .collect::<Vec<_>>` | [385](../../src/tool_backends.rs#L385) | receiver-type-required |
| `assemble_tool_backends` | `profile             .config             .workspace             .policy             .toolchain_roots             .iter()             .map` | [385](../../src/tool_backends.rs#L385) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy()                     .into_owned` | [392](../../src/tool_backends.rs#L392) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join("bin")                     .to_string_lossy` | [392](../../src/tool_backends.rs#L392) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new(root)                     .join` | [392](../../src/tool_backends.rs#L392) | receiver-type-required |
| `assemble_tool_backends` | `std::path::Path::new` | [392](../../src/tool_backends.rs#L392) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `bins.extend` | [398](../../src/tool_backends.rs#L398) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter()                 .map` | [399](../../src/tool_backends.rs#L399) | receiver-type-required |
| `assemble_tool_backends` | `["/usr/bin", "/bin", "/usr/sbin", "/sbin"]                 .into_iter` | [399](../../src/tool_backends.rs#L399) | receiver-type-required |
| `assemble_tool_backends` | `"PATH".into` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `bins.join` | [405](../../src/tool_backends.rs#L405) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate()         .skip(1)         .map(&#124;(index, path)&#124; engine::RootMount {             name: format!("workspace-{index}"),             path: PathBuf::from(path),         })         .collect` | [407](../../src/tool_backends.rs#L407) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate()         .skip(1)         .map` | [407](../../src/tool_backends.rs#L407) | receiver-type-required |
| `assemble_tool_backends` | `profile         .config         .workspace         .cwd         .iter()         .enumerate()         .skip` | [407](../../src/tool_backends.rs#L407) | receiver-type-required |
| `assemble_tool_backends` | `SystemToolBackend::new` | [419](../../src/tool_backends.rs#L419) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `cancellation.tools.clone` | [425](../../src/tool_backends.rs#L425) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or("worker executable has no parent directory")?         .join` | [427](../../src/tool_backends.rs#L427) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent()         .ok_or` | [427](../../src/tool_backends.rs#L427) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe()?         .parent` | [427](../../src/tool_backends.rs#L427) | receiver-type-required |
| `assemble_tool_backends` | `std::env::current_exe` | [427](../../src/tool_backends.rs#L427) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `workspace_helper.is_file` | [431](../../src/tool_backends.rs#L431) | receiver-type-required |
| `assemble_tool_backends` | `system_backend.with_edit_recorder` | [433](../../src/tool_backends.rs#L433) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .workspace                     .cwd                     .iter()                     .map(PathBuf::from)                     .collect` | [436](../../src/tool_backends.rs#L436) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .workspace                     .cwd                     .iter()                     .map` | [436](../../src/tool_backends.rs#L436) | receiver-type-required |
| `assemble_tool_backends` | `profile                     .config                     .workspace                     .cwd                     .iter` | [436](../../src/tool_backends.rs#L436) | receiver-type-required |
| `assemble_tool_backends` | `profile.config.workspace.id.clone` | [444](../../src/tool_backends.rs#L444), [481](../../src/tool_backends.rs#L481) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or("thread folder has no session identity")?                     .to_owned` | [445](../../src/tool_backends.rs#L445) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then(&#124;name&#124; name.to_str())                     .ok_or` | [445](../../src/tool_backends.rs#L445) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name()                     .and_then` | [445](../../src/tool_backends.rs#L445) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder                     .file_name` | [445](../../src/tool_backends.rs#L445) | receiver-type-required |
| `assemble_tool_backends` | `name.to_str` | [447](../../src/tool_backends.rs#L447), [456](../../src/tool_backends.rs#L456) | receiver-type-required |
| `assemble_tool_backends` | `router.push` | [452](../../src/tool_backends.rs#L452), [464](../../src/tool_backends.rs#L464), [479](../../src/tool_backends.rs#L479) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or("thread folder has no UTF-8 session UUID")?             .to_owned` | [454](../../src/tool_backends.rs#L454) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then(&#124;name&#124; name.to_str())             .ok_or` | [454](../../src/tool_backends.rs#L454) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name()             .and_then` | [454](../../src/tool_backends.rs#L454) | receiver-type-required |
| `assemble_tool_backends` | `thread_folder             .file_name` | [454](../../src/tool_backends.rs#L454) | receiver-type-required |
| `assemble_tool_backends` | `Rc::new` | [459](../../src/tool_backends.rs#L459) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `RefCell::new` | [459](../../src/tool_backends.rs#L459) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange_tool_control_runtime(&selected, request, lines, stdout, cancellation)                 .map_err` | [460](../../src/tool_backends.rs#L460) | receiver-type-required |
| `assemble_tool_backends` | `exchange_tool_control_runtime` | [460](../../src/tool_backends.rs#L460) | [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L118) |
| `assemble_tool_backends` | `error.to_string` | [461](../../src/tool_backends.rs#L461) | receiver-type-required |
| `assemble_tool_backends` | `Rc::clone` | [463](../../src/tool_backends.rs#L463) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `SupervisorControlBackend::new` | [464](../../src/tool_backends.rs#L464) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `session.clone` | [465](../../src/tool_backends.rs#L465) | receiver-type-required |
| `assemble_tool_backends` | `(fixed_exchange.borrow_mut())` | [466](../../src/tool_backends.rs#L466) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `fixed_exchange.borrow_mut` | [466](../../src/tool_backends.rs#L466) | receiver-type-required |
| `assemble_tool_backends` | `dynamic_backends.push` | [468](../../src/tool_backends.rs#L468) | receiver-type-required |
| `assemble_tool_backends` | `Box::new` | [468](../../src/tool_backends.rs#L468) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `DynamicSupervisorBackend::new` | [468](../../src/tool_backends.rs#L468) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `(exchange.borrow_mut())` | [470](../../src/tool_backends.rs#L470) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `exchange.borrow_mut` | [470](../../src/tool_backends.rs#L470) | receiver-type-required |
| `assemble_tool_backends` | `allowed.clone` | [474](../../src/tool_backends.rs#L474) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)         .deferred_search_entries` | [477](../../src/tool_backends.rs#L477) | receiver-type-required |
| `assemble_tool_backends` | `DynamicToolDispatcher::new` | [477](../../src/tool_backends.rs#L477) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `WorkflowBackend::new` | [479](../../src/tool_backends.rs#L479) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `profile.bindings.goal_id.clone` | [482](../../src/tool_backends.rs#L482) | receiver-type-required |
| `assemble_tool_backends` | `frozen_skill_catalog` | [483](../../src/tool_backends.rs#L483) | external-constructor-callback-or-unresolved |
| `assemble_tool_backends` | `Ok` | [486](../../src/tool_backends.rs#L486) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `PathBuf::from` | [499](../../src/tool_backends.rs#L499), [562](../../src/tool_backends.rs#L562) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `profile             .config             .workspace             .cwd             .first()             .ok_or` | [500](../../src/tool_backends.rs#L500) | receiver-type-required |
| `invocation_write_roots` | `profile             .config             .workspace             .cwd             .first` | [500](../../src/tool_backends.rs#L500) | receiver-type-required |
| `invocation_write_roots` | `profile         .bindings         .dynamic_catalog         .tools         .iter()         .any` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `invocation_write_roots` | `profile         .bindings         .dynamic_catalog         .tools         .iter` | [507](../../src/tool_backends.rs#L507) | receiver-type-required |
| `invocation_write_roots` | `Ok` | [514](../../src/tool_backends.rs#L514), [593](../../src/tool_backends.rs#L593) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `profile             .instruction             .meet_workspace_policy` | [514](../../src/tool_backends.rs#L514) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.workspace.policy)         .writable_roots         .into_iter()         .map(PathBuf::from)         .collect::<Vec<_>>` | [519](../../src/tool_backends.rs#L519) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.workspace.policy)         .writable_roots         .into_iter()         .map` | [519](../../src/tool_backends.rs#L519) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy(&profile.config.workspace.policy)         .writable_roots         .into_iter` | [519](../../src/tool_backends.rs#L519) | receiver-type-required |
| `invocation_write_roots` | `profile         .instruction         .meet_workspace_policy` | [519](../../src/tool_backends.rs#L519) | receiver-type-required |
| `invocation_write_roots` | `call.name.as_str` | [526](../../src/tool_backends.rs#L526) | receiver-type-required |
| `invocation_write_roots` | `call.arguments.get("writable_paths").is_none_or` | [528](../../src/tool_backends.rs#L528) | receiver-type-required |
| `invocation_write_roots` | `call.arguments.get` | [528](../../src/tool_backends.rs#L528) | receiver-type-required |
| `invocation_write_roots` | `value.is_null` | [529](../../src/tool_backends.rs#L529) | receiver-type-required |
| `invocation_write_roots` | `value.as_array().is_some_and` | [529](../../src/tool_backends.rs#L529) | receiver-type-required |
| `invocation_write_roots` | `value.as_array` | [529](../../src/tool_backends.rs#L529) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter()                 .map(&#124;path&#124; path.to_string_lossy().into_owned())                 .collect` | [534](../../src/tool_backends.rs#L534) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter()                 .map` | [534](../../src/tool_backends.rs#L534) | receiver-type-required |
| `invocation_write_roots` | `allowed                 .iter` | [534](../../src/tool_backends.rs#L534) | receiver-type-required |
| `invocation_write_roots` | `path.to_string_lossy().into_owned` | [536](../../src/tool_backends.rs#L536) | receiver-type-required |
| `invocation_write_roots` | `path.to_string_lossy` | [536](../../src/tool_backends.rs#L536) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map(&#124;value&#124; {                 value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or("shell writable_paths must contain strings")             })             .collect::<Result<Vec<_>, _>>` | [539](../../src/tool_backends.rs#L539) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten()             .map` | [539](../../src/tool_backends.rs#L539) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter()             .flatten` | [539](../../src/tool_backends.rs#L539) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then(Value::as_array)             .into_iter` | [539](../../src/tool_backends.rs#L539) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("writable_paths")             .and_then` | [539](../../src/tool_backends.rs#L539) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get` | [539](../../src/tool_backends.rs#L539), [552](../../src/tool_backends.rs#L552) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str()                     .map(ToOwned::to_owned)                     .ok_or` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str()                     .map` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `value                     .as_str` | [546](../../src/tool_backends.rs#L546) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map(&#124;path&#124; vec![path.to_owned()])             .unwrap_or_default` | [552](../../src/tool_backends.rs#L552) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then(Value::as_str)             .map` | [552](../../src/tool_backends.rs#L552) | receiver-type-required |
| `invocation_write_roots` | `call             .arguments             .get("path")             .and_then` | [552](../../src/tool_backends.rs#L552) | receiver-type-required |
| `invocation_write_roots` | `Vec::new` | [558](../../src/tool_backends.rs#L558), [560](../../src/tool_backends.rs#L560) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `raw_path             .components()             .any` | [563](../../src/tool_backends.rs#L563) | receiver-type-required |
| `invocation_write_roots` | `raw_path             .components` | [563](../../src/tool_backends.rs#L563) | receiver-type-required |
| `invocation_write_roots` | `Err` | [567](../../src/tool_backends.rs#L567), [575](../../src/tool_backends.rs#L575) | external-constructor-callback-or-unresolved |
| `invocation_write_roots` | `format!("writable path {raw:?} contains parent traversal").into` | [567](../../src/tool_backends.rs#L567) | receiver-type-required |
| `invocation_write_roots` | `raw_path.is_absolute` | [569](../../src/tool_backends.rs#L569) | receiver-type-required |
| `invocation_write_roots` | `workspace.join` | [572](../../src/tool_backends.rs#L572) | receiver-type-required |
| `invocation_write_roots` | `allowed.iter().any` | [574](../../src/tool_backends.rs#L574) | receiver-type-required |
| `invocation_write_roots` | `allowed.iter` | [574](../../src/tool_backends.rs#L574) | receiver-type-required |
| `invocation_write_roots` | `target.starts_with` | [574](../../src/tool_backends.rs#L574) | receiver-type-required |
| `invocation_write_roots` | `format!(                 "writable path {} is outside the effective writable roots",                 target.display()             )             .into` | [575](../../src/tool_backends.rs#L575) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent()                 .ok_or("apply_patch target has no writable parent")?                 .to_path_buf` | [582](../../src/tool_backends.rs#L582) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent()                 .ok_or` | [582](../../src/tool_backends.rs#L582) | receiver-type-required |
| `invocation_write_roots` | `target                 .parent` | [582](../../src/tool_backends.rs#L582) | receiver-type-required |
| `invocation_write_roots` | `roots.push` | [589](../../src/tool_backends.rs#L589) | receiver-type-required |
| `invocation_write_roots` | `authority.to_string_lossy().into_owned` | [589](../../src/tool_backends.rs#L589) | receiver-type-required |
| `invocation_write_roots` | `authority.to_string_lossy` | [589](../../src/tool_backends.rs#L589) | receiver-type-required |
| `invocation_write_roots` | `roots.sort` | [591](../../src/tool_backends.rs#L591) | receiver-type-required |
| `invocation_write_roots` | `roots.dedup` | [592](../../src/tool_backends.rs#L592) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent()         .ok_or` | [599](../../src/tool_backends.rs#L599) | receiver-type-required |
| `storage_root_for_thread_folder` | `thread_folder         .parent` | [599](../../src/tool_backends.rs#L599) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name().is_some_and` | [602](../../src/tool_backends.rs#L602) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.file_name` | [602](../../src/tool_backends.rs#L602) | receiver-type-required |
| `storage_root_for_thread_folder` | `Ok` | [603](../../src/tool_backends.rs#L603), [610](../../src/tool_backends.rs#L610) | external-constructor-callback-or-unresolved |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or("threads directory has no storage root")?             .to_path_buf` | [603](../../src/tool_backends.rs#L603) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent()             .ok_or` | [603](../../src/tool_backends.rs#L603) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent             .parent` | [603](../../src/tool_backends.rs#L603) | receiver-type-required |
| `storage_root_for_thread_folder` | `parent.to_path_buf` | [610](../../src/tool_backends.rs#L610) | receiver-type-required |
| `session_permission_policy` | `PermissionModePolicy::for_session_folder` | [617](../../src/tool_backends.rs#L617) | external-constructor-callback-or-unresolved |
