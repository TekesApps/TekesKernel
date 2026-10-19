# tekes-worker::workspace_edits

[Package atlas](index.md) · [Source](../../src/workspace_edits.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::workspace_edits::edit_payload](../../src/workspace_edits.rs#L8) | function_item | `private` |  |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder](../../src/workspace_edits.rs#L30) | struct_item | `pub` |  |
| [tekes-worker::workspace_edits::canonical_edit_target](../../src/workspace_edits.rs#L39) | function_item | `private` |  |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder::abort](../../src/workspace_edits.rs#L73) | function_item | `private` |  |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder::prepare](../../src/workspace_edits.rs#L82) | function_item | `private` |  |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder::record](../../src/workspace_edits.rs#L91) | function_item | `private` |  |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder::invoke](../../src/workspace_edits.rs#L103) | function_item | `private` |  |
| [tekes-worker::workspace_edits::tests::edit_identity_accepts_missing_nested_parent_under_canonical_workspace](../../src/workspace_edits.rs#L174) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::workspace_edits::tests::large_edit_capture_is_bounded_and_retains_content_identity](../../src/workspace_edits.rs#L187) | function_item | `private` | test; #[cfg(test)] |
| [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../src/workspace_edits.rs#L214) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `json` | `serde_json::json` | `private` |
| `io` | `std::io` | `private` |
| `Write` | `std::io::Write` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `BackendFailure` | `tools::BackendFailure` | `private` |
| `ToolExecution` | `tools::ToolExecution` | `private` |
| `*` | `super::*` | `private` |
| `DurableArtifactVersions` | `engine::DurableArtifactVersions` | `private` |
| `SystemToolBackend` | `engine::SystemToolBackend` | `private` |
| `SystemToolConfig` | `engine::SystemToolConfig` | `private` |
| `ToolBackend` | `engine::ToolBackend` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `BackendTerminal` | `tools::BackendTerminal` | `private` |
| `CancellationToken` | `tools::CancellationToken` | `private` |
| `HelperClient` | `tools::HelperClient` | `private` |
| `HelperInvoker` | `tools::HelperInvoker` | `private` |
| `NetworkPolicy` | `tools::NetworkPolicy` | `private` |
| `SandboxBackend` | `tools::SandboxBackend` | `private` |
| `SandboxPolicy` | `tools::SandboxPolicy` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-worker::workspace_edits::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–6: 2 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-worker::workspace_edits::WorkspaceEditRecorder::invoke"]
  n1["tekes-worker::workspace_edits::canonical_edit_target"]
  n2["tekes-worker::workspace_edits::WorkspaceEditRecorder::abort"]
  n3["tekes-worker::workspace_edits::WorkspaceEditRecorder::prepare"]
  n4["tekes-worker::workspace_edits::edit_payload"]
  n5["tekes-worker::workspace_edits::WorkspaceEditRecorder::record"]
  n6["workspace-service::process::invoke_with_authority"]
  n0 --> n1
  n0 --> n6
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `edit_payload` | `before         .map_or(0, str::len)         .saturating_add` | [16](../../src/workspace_edits.rs#L16) | receiver-type-required |
| `edit_payload` | `before         .map_or` | [16](../../src/workspace_edits.rs#L16) | receiver-type-required |
| `edit_payload` | `after.map_or` | [18](../../src/workspace_edits.rs#L18) | receiver-type-required |
| `edit_payload` | `[before, after]             .map` | [22](../../src/workspace_edits.rs#L22) | receiver-type-required |
| `edit_payload` | `content.map` | [23](../../src/workspace_edits.rs#L23) | receiver-type-required |
| `canonical_edit_target` | `target         .parent()         .ok_or_else` | [40](../../src/workspace_edits.rs#L40) | receiver-type-required |
| `canonical_edit_target` | `target         .parent` | [40](../../src/workspace_edits.rs#L40) | receiver-type-required |
| `canonical_edit_target` | `io::Error::from` | [42](../../src/workspace_edits.rs#L42), [67](../../src/workspace_edits.rs#L67) | external-constructor-callback-or-unresolved |
| `canonical_edit_target` | `Vec::new` | [43](../../src/workspace_edits.rs#L43) | external-constructor-callback-or-unresolved |
| `canonical_edit_target` | `parent.canonicalize` | [45](../../src/workspace_edits.rs#L45) | receiver-type-required |
| `canonical_edit_target` | `error.kind` | [47](../../src/workspace_edits.rs#L47) | receiver-type-required |
| `canonical_edit_target` | `parent.file_name` | [48](../../src/workspace_edits.rs#L48) | receiver-type-required |
| `canonical_edit_target` | `Err` | [49](../../src/workspace_edits.rs#L49), [53](../../src/workspace_edits.rs#L53), [57](../../src/workspace_edits.rs#L57) | external-constructor-callback-or-unresolved |
| `canonical_edit_target` | `missing.push` | [51](../../src/workspace_edits.rs#L51) | receiver-type-required |
| `canonical_edit_target` | `component.to_os_string` | [51](../../src/workspace_edits.rs#L51) | receiver-type-required |
| `canonical_edit_target` | `parent.parent` | [52](../../src/workspace_edits.rs#L52) | receiver-type-required |
| `canonical_edit_target` | `missing.into_iter().rev` | [61](../../src/workspace_edits.rs#L61) | receiver-type-required |
| `canonical_edit_target` | `missing.into_iter` | [61](../../src/workspace_edits.rs#L61) | receiver-type-required |
| `canonical_edit_target` | `path.push` | [62](../../src/workspace_edits.rs#L62), [64](../../src/workspace_edits.rs#L64) | receiver-type-required |
| `canonical_edit_target` | `target             .file_name()             .ok_or_else` | [65](../../src/workspace_edits.rs#L65) | receiver-type-required |
| `canonical_edit_target` | `target             .file_name` | [65](../../src/workspace_edits.rs#L65) | receiver-type-required |
| `canonical_edit_target` | `Ok` | [69](../../src/workspace_edits.rs#L69) | external-constructor-callback-or-unresolved |
| `abort` | `self.invoke` | [80](../../src/workspace_edits.rs#L80) | receiver-type-required |
| `prepare` | `self.invoke` | [89](../../src/workspace_edits.rs#L89) | receiver-type-required |
| `record` | `self.invoke` | [98](../../src/workspace_edits.rs#L98) | receiver-type-required |
| `invoke` | `self.executable.clone` | [111](../../src/workspace_edits.rs#L111) | receiver-type-required |
| `invoke` | `self.workspace.clone` | [112](../../src/workspace_edits.rs#L112) | receiver-type-required |
| `invoke` | `std::path::Path::new` | [114](../../src/workspace_edits.rs#L114) | external-constructor-callback-or-unresolved |
| `invoke` | `canonical_edit_target(target).map_err` | [115](../../src/workspace_edits.rs#L115) | receiver-type-required |
| `invoke` | `canonical_edit_target` | [115](../../src/workspace_edits.rs#L115) | [tekes-worker::workspace_edits::canonical_edit_target](../../src/workspace_edits.rs#L39) |
| `invoke` | `BackendFailure::Unknown` | [116](../../src/workspace_edits.rs#L116), [156](../../src/workspace_edits.rs#L156), [158](../../src/workspace_edits.rs#L158) | external-constructor-callback-or-unresolved |
| `invoke` | `self             .workspace_roots             .iter()             .any` | [120](../../src/workspace_edits.rs#L120) | receiver-type-required |
| `invoke` | `self             .workspace_roots             .iter` | [120](../../src/workspace_edits.rs#L120) | receiver-type-required |
| `invoke` | `path.starts_with` | [123](../../src/workspace_edits.rs#L123) | receiver-type-required |
| `invoke` | `Ok` | [125](../../src/workspace_edits.rs#L125), [153](../../src/workspace_edits.rs#L153) | external-constructor-callback-or-unresolved |
| `invoke` | `std::thread::spawn(move &#124;&#124; -> Result<(), String> {             let mut file = tempfile::NamedTempFile::new().map_err(&#124;e&#124; e.to_string())?;             file.write_all(&serde_json::to_vec(&authority).map_err(&#124;e&#124; e.to_string())?)                 .map_err(&#124;e&#124; e.to_string())?;             file.flush().map_err(&#124;e&#124; e.to_string())?;             let runtime = tokio::runtime::Builder::new_current_thread()                 .enable_all()                 .build()                 .map_err(&#124;e&#124; e.to_string())?;             let response = runtime                 .block_on(workspace_service::process::invoke_with_authority(                     &executable,                     &workspace,                     Some(file.path()),                     method,                     request,                     std::time::Duration::from_secs(30),                 ))                 .map_err(&#124;e&#124; e.message)?;             if let Some(error) = response.get("error") {                 return Err(error.to_string());             }             Ok(())         })         .join()         .map_err` | [131](../../src/workspace_edits.rs#L131) | receiver-type-required |
| `invoke` | `std::thread::spawn(move &#124;&#124; -> Result<(), String> {             let mut file = tempfile::NamedTempFile::new().map_err(&#124;e&#124; e.to_string())?;             file.write_all(&serde_json::to_vec(&authority).map_err(&#124;e&#124; e.to_string())?)                 .map_err(&#124;e&#124; e.to_string())?;             file.flush().map_err(&#124;e&#124; e.to_string())?;             let runtime = tokio::runtime::Builder::new_current_thread()                 .enable_all()                 .build()                 .map_err(&#124;e&#124; e.to_string())?;             let response = runtime                 .block_on(workspace_service::process::invoke_with_authority(                     &executable,                     &workspace,                     Some(file.path()),                     method,                     request,                     std::time::Duration::from_secs(30),                 ))                 .map_err(&#124;e&#124; e.message)?;             if let Some(error) = response.get("error") {                 return Err(error.to_string());             }             Ok(())         })         .join` | [131](../../src/workspace_edits.rs#L131) | receiver-type-required |
| `invoke` | `std::thread::spawn` | [131](../../src/workspace_edits.rs#L131) | external-constructor-callback-or-unresolved |
| `invoke` | `tempfile::NamedTempFile::new().map_err` | [132](../../src/workspace_edits.rs#L132) | receiver-type-required |
| `invoke` | `tempfile::NamedTempFile::new` | [132](../../src/workspace_edits.rs#L132) | external-constructor-callback-or-unresolved |
| `invoke` | `e.to_string` | [132](../../src/workspace_edits.rs#L132), [133](../../src/workspace_edits.rs#L133), [134](../../src/workspace_edits.rs#L134), [135](../../src/workspace_edits.rs#L135), [139](../../src/workspace_edits.rs#L139) | receiver-type-required |
| `invoke` | `file.write_all(&serde_json::to_vec(&authority).map_err(&#124;e&#124; e.to_string())?)                 .map_err` | [133](../../src/workspace_edits.rs#L133) | receiver-type-required |
| `invoke` | `file.write_all` | [133](../../src/workspace_edits.rs#L133) | receiver-type-required |
| `invoke` | `serde_json::to_vec(&authority).map_err` | [133](../../src/workspace_edits.rs#L133) | receiver-type-required |
| `invoke` | `serde_json::to_vec` | [133](../../src/workspace_edits.rs#L133) | external-constructor-callback-or-unresolved |
| `invoke` | `file.flush().map_err` | [135](../../src/workspace_edits.rs#L135) | receiver-type-required |
| `invoke` | `file.flush` | [135](../../src/workspace_edits.rs#L135) | receiver-type-required |
| `invoke` | `tokio::runtime::Builder::new_current_thread()                 .enable_all()                 .build()                 .map_err` | [136](../../src/workspace_edits.rs#L136) | receiver-type-required |
| `invoke` | `tokio::runtime::Builder::new_current_thread()                 .enable_all()                 .build` | [136](../../src/workspace_edits.rs#L136) | receiver-type-required |
| `invoke` | `tokio::runtime::Builder::new_current_thread()                 .enable_all` | [136](../../src/workspace_edits.rs#L136) | receiver-type-required |
| `invoke` | `tokio::runtime::Builder::new_current_thread` | [136](../../src/workspace_edits.rs#L136) | external-constructor-callback-or-unresolved |
| `invoke` | `runtime                 .block_on(workspace_service::process::invoke_with_authority(                     &executable,                     &workspace,                     Some(file.path()),                     method,                     request,                     std::time::Duration::from_secs(30),                 ))                 .map_err` | [140](../../src/workspace_edits.rs#L140) | receiver-type-required |
| `invoke` | `runtime                 .block_on` | [140](../../src/workspace_edits.rs#L140) | receiver-type-required |
| `invoke` | `workspace_service::process::invoke_with_authority` | [141](../../src/workspace_edits.rs#L141) | [workspace-service::process::invoke_with_authority](../../../workspace-service/src/process.rs#L36) |
| `invoke` | `Some` | [144](../../src/workspace_edits.rs#L144) | external-constructor-callback-or-unresolved |
| `invoke` | `file.path` | [144](../../src/workspace_edits.rs#L144) | receiver-type-required |
| `invoke` | `std::time::Duration::from_secs` | [147](../../src/workspace_edits.rs#L147) | external-constructor-callback-or-unresolved |
| `invoke` | `response.get` | [150](../../src/workspace_edits.rs#L150) | receiver-type-required |
| `invoke` | `Err` | [151](../../src/workspace_edits.rs#L151) | external-constructor-callback-or-unresolved |
| `invoke` | `error.to_string` | [151](../../src/workspace_edits.rs#L151) | receiver-type-required |
| `invoke` | `"file committed but edit recorder panicked".into` | [156](../../src/workspace_edits.rs#L156) | receiver-type-required |
| `invoke` | `result.map_err` | [157](../../src/workspace_edits.rs#L157) | receiver-type-required |
| `edit_identity_accepts_missing_nested_parent_under_canonical_workspace` | `tempfile::tempdir().unwrap` | [175](../../src/workspace_edits.rs#L175) | receiver-type-required |
| `edit_identity_accepts_missing_nested_parent_under_canonical_workspace` | `tempfile::tempdir` | [175](../../src/workspace_edits.rs#L175) | external-constructor-callback-or-unresolved |
| `edit_identity_accepts_missing_nested_parent_under_canonical_workspace` | `workspace.path().join` | [176](../../src/workspace_edits.rs#L176) | receiver-type-required |
| `edit_identity_accepts_missing_nested_parent_under_canonical_workspace` | `workspace.path` | [176](../../src/workspace_edits.rs#L176) | receiver-type-required |
| `large_edit_capture_is_bounded_and_retains_content_identity` | `"\0".repeat` | [188](../../src/workspace_edits.rs#L188) | receiver-type-required |
| `large_edit_capture_is_bounded_and_retains_content_identity` | `edit_payload` | [189](../../src/workspace_edits.rs#L189), [194](../../src/workspace_edits.rs#L194) | external-constructor-callback-or-unresolved |
| `large_edit_capture_is_bounded_and_retains_content_identity` | `std::path::Path::new` | [189](../../src/workspace_edits.rs#L189), [196](../../src/workspace_edits.rs#L196) | external-constructor-callback-or-unresolved |
| `large_edit_capture_is_bounded_and_retains_content_identity` | `Some` | [189](../../src/workspace_edits.rs#L189), [198](../../src/workspace_edits.rs#L198) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `PathBuf::from` | [215](../../src/workspace_edits.rs#L215), [218](../../src/workspace_edits.rs#L218) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `std::env::var_os("TEKES_WORKSPACE_SERVICE_BIN").expect` | [216](../../src/workspace_edits.rs#L216) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `std::env::var_os` | [216](../../src/workspace_edits.rs#L216), [218](../../src/workspace_edits.rs#L218) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `std::env::var_os("TEKES_TOOL_HELPER_BIN").expect` | [218](../../src/workspace_edits.rs#L218) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tempfile::tempdir().unwrap` | [219](../../src/workspace_edits.rs#L219), [220](../../src/workspace_edits.rs#L220) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tempfile::tempdir` | [219](../../src/workspace_edits.rs#L219), [220](../../src/workspace_edits.rs#L220) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `project.path().canonicalize().unwrap` | [221](../../src/workspace_edits.rs#L221) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `project.path().canonicalize` | [221](../../src/workspace_edits.rs#L221) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `project.path` | [221](../../src/workspace_edits.rs#L221) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `HelperClient::sandboxed_with_scratch(             helper,             vec![("workspace".into(), root.clone())],             SandboxPolicy {                 format: 1,                 read_roots: vec![root.to_string_lossy().into_owned()],                 write_roots: vec![root.to_string_lossy().into_owned()],                 network: NetworkPolicy::Deny,                 allow_process: true,                 scratch: None,                 protected_roots: Vec::new(),             },             tools::probe_backend(sandbox),         )         .unwrap` | [226](../../src/workspace_edits.rs#L226) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `HelperClient::sandboxed_with_scratch` | [226](../../src/workspace_edits.rs#L226) | [tools::helper::HelperClient::sandboxed_with_scratch](../../../tools/src/helper.rs#L712) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `Vec::new` | [236](../../src/workspace_edits.rs#L236) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tools::probe_backend` | [238](../../src/workspace_edits.rs#L238) | [tools::sandbox::probe_backend](../../../tools/src/sandbox.rs#L534) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `Arc::new` | [241](../../src/workspace_edits.rs#L241), [251](../../src/workspace_edits.rs#L251), [254](../../src/workspace_edits.rs#L254) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `executable.clone` | [242](../../src/workspace_edits.rs#L242) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `root.clone` | [243](../../src/workspace_edits.rs#L243) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `storage.path().to_owned` | [245](../../src/workspace_edits.rs#L245) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `storage.path` | [245](../../src/workspace_edits.rs#L245), [255](../../src/workspace_edits.rs#L255) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"workspace".into` | [246](../../src/workspace_edits.rs#L246) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"session".into` | [247](../../src/workspace_edits.rs#L247), [286](../../src/workspace_edits.rs#L286) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `SystemToolBackend::new(             SystemToolConfig::workspace("workspace", &root),             Some(Arc::new(client) as Arc<dyn HelperInvoker>),             None,             None,             Some(Arc::new(                 DurableArtifactVersions::new(storage.path().join("versions")).unwrap(),             )),             CancellationToken::default(),         )         .with_edit_recorder` | [249](../../src/workspace_edits.rs#L249) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `SystemToolBackend::new` | [249](../../src/workspace_edits.rs#L249) | [engine::system_tools::SystemToolBackend::new](../../../engine/src/system_tools.rs#L477) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `SystemToolConfig::workspace` | [250](../../src/workspace_edits.rs#L250) | [engine::system_tools::SystemToolConfig::workspace](../../../engine/src/system_tools.rs#L87) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `Some` | [251](../../src/workspace_edits.rs#L251), [254](../../src/workspace_edits.rs#L254), [325](../../src/workspace_edits.rs#L325), [338](../../src/workspace_edits.rs#L338) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `DurableArtifactVersions::new(storage.path().join("versions")).unwrap` | [255](../../src/workspace_edits.rs#L255) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `DurableArtifactVersions::new` | [255](../../src/workspace_edits.rs#L255) | [engine::system_tools::DurableArtifactVersions::new](../../../engine/src/system_tools.rs#L179) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `storage.path().join` | [255](../../src/workspace_edits.rs#L255) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `CancellationToken::default` | [257](../../src/workspace_edits.rs#L257) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"large file line\n".repeat` | [260](../../src/workspace_edits.rs#L260) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `large_content             .lines()             .map(&#124;line&#124; format!("+{line}"))             .collect::<Vec<_>>()             .join` | [261](../../src/workspace_edits.rs#L261) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `large_content             .lines()             .map(&#124;line&#124; format!("+{line}"))             .collect::<Vec<_>>` | [261](../../src/workspace_edits.rs#L261) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `large_content             .lines()             .map` | [261](../../src/workspace_edits.rs#L261) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `large_content             .lines` | [261](../../src/workspace_edits.rs#L261) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `schema::IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap` | [284](../../src/workspace_edits.rs#L284) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `schema::IJsonValue::parse` | [284](../../src/workspace_edits.rs#L284) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `serde_json::to_vec(&args).unwrap` | [284](../../src/workspace_edits.rs#L284) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `serde_json::to_vec` | [284](../../src/workspace_edits.rs#L284), [315](../../src/workspace_edits.rs#L315) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `call.into` | [287](../../src/workspace_edits.rs#L287) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"apply_patch".into` | [288](../../src/workspace_edits.rs#L288) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"attempt".into` | [289](../../src/workspace_edits.rs#L289) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `invocation.clone` | [290](../../src/workspace_edits.rs#L290) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `"2026-09-08T00:00:00Z".into` | [293](../../src/workspace_edits.rs#L293) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `backend.execute` | [295](../../src/workspace_edits.rs#L295) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tempfile::NamedTempFile::new().unwrap` | [314](../../src/workspace_edits.rs#L314) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tempfile::NamedTempFile::new` | [314](../../src/workspace_edits.rs#L314) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `authority.write_all(&serde_json::to_vec(&json!({"stateRoot":storage.path(),"allowedOperations":[],"allowedRepositoryRoots":[]})).unwrap()).unwrap` | [315](../../src/workspace_edits.rs#L315) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `authority.write_all` | [315](../../src/workspace_edits.rs#L315) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `serde_json::to_vec(&json!({"stateRoot":storage.path(),"allowedOperations":[],"allowedRepositoryRoots":[]})).unwrap` | [315](../../src/workspace_edits.rs#L315) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build()             .unwrap` | [316](../../src/workspace_edits.rs#L316) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tokio::runtime::Builder::new_current_thread()             .enable_all()             .build` | [316](../../src/workspace_edits.rs#L316) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tokio::runtime::Builder::new_current_thread()             .enable_all` | [316](../../src/workspace_edits.rs#L316) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `tokio::runtime::Builder::new_current_thread` | [316](../../src/workspace_edits.rs#L316) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `runtime                 .block_on(workspace_service::process::invoke_with_authority(                     &executable,                     &root,                     Some(authority.path()),                     "turnChanges",                     json!({"workspaceId":"workspace","sessionId":"session","turnId":turn}),                     std::time::Duration::from_secs(10),                 ))                 .unwrap` | [321](../../src/workspace_edits.rs#L321) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `runtime                 .block_on` | [321](../../src/workspace_edits.rs#L321) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `workspace_service::process::invoke_with_authority` | [322](../../src/workspace_edits.rs#L322), [335](../../src/workspace_edits.rs#L335) | [workspace-service::process::invoke_with_authority](../../../workspace-service/src/process.rs#L36) |
| `actual_patch_helper_to_turn_ledger_round_trip` | `authority.path` | [325](../../src/workspace_edits.rs#L325), [338](../../src/workspace_edits.rs#L338) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `std::time::Duration::from_secs` | [328](../../src/workspace_edits.rs#L328), [341](../../src/workspace_edits.rs#L341) | external-constructor-callback-or-unresolved |
| `actual_patch_helper_to_turn_ledger_round_trip` | `runtime             .block_on(workspace_service::process::invoke_with_authority(                 &executable,                 &root,                 Some(authority.path()),                 "turnChanges",                 json!({"workspaceId":"workspace","sessionId":"session","turnId":"3"}),                 std::time::Duration::from_secs(10),             ))             .unwrap` | [334](../../src/workspace_edits.rs#L334) | receiver-type-required |
| `actual_patch_helper_to_turn_ledger_round_trip` | `runtime             .block_on` | [334](../../src/workspace_edits.rs#L334) | receiver-type-required |
