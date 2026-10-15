# tekes-supervisor::production_tool_control

[Package atlas](index.md) · [Source](../../src/production_tool_control.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::production_tool_control::DEFAULT_PAGE_BYTES](../../src/production_tool_control.rs#L32) | const_item | `private` |  |
| [tekes-supervisor::production_tool_control::DEFAULT_SEARCH_LIMIT](../../src/production_tool_control.rs#L33) | const_item | `private` |  |
| [tekes-supervisor::production_tool_control::MAX_SEARCH_LIMIT](../../src/production_tool_control.rs#L34) | const_item | `private` |  |
| [tekes-supervisor::production_tool_control::DeliveryRequest](../../src/production_tool_control.rs#L37) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::InterruptRequest](../../src/production_tool_control.rs#L47) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ChildLaunchProof](../../src/production_tool_control.rs#L55) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ParentReportProof](../../src/production_tool_control.rs#L67) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority](../../src/production_tool_control.rs#L81) | trait_item | `pub` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority::ensure_running](../../src/production_tool_control.rs#L82) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority::deliver_input](../../src/production_tool_control.rs#L88) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority::interrupt](../../src/production_tool_control.rs#L93) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority::ensure_child](../../src/production_tool_control.rs#L98) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorRuntimeAuthority::deliver_report](../../src/production_tool_control.rs#L103) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorJobAuthority](../../src/production_tool_control.rs#L113) | trait_item | `pub` |  |
| [tekes-supervisor::production_tool_control::SupervisorJobAuthority::execute](../../src/production_tool_control.rs#L114) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicExecutionOutcome](../../src/production_tool_control.rs#L124) | enum_item | `pub` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority](../../src/production_tool_control.rs#L132) | trait_item | `pub` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::supports](../../src/production_tool_control.rs#L133) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::execute](../../src/production_tool_control.rs#L135) | function_signature_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::execute_outcome](../../src/production_tool_control.rs#L144) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::continue_task](../../src/production_tool_control.rs#L156) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::reconcile](../../src/production_tool_control.rs#L168) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::cancel_inflight](../../src/production_tool_control.rs#L177) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority](../../src/production_tool_control.rs#L180) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::new](../../src/production_tool_control.rs#L188) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::execute](../../src/production_tool_control.rs#L205) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler](../../src/production_tool_control.rs#L263) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::new](../../src/production_tool_control.rs#L273) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::with_dynamic_routes](../../src/production_tool_control.rs#L284) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::runtime](../../src/production_tool_control.rs#L295) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute](../../src/production_tool_control.rs#L303) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::recovery](../../src/production_tool_control.rs#L327) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::reconcile](../../src/production_tool_control.rs#L351) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::continue_task](../../src/production_tool_control.rs#L375) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_inner](../../src/production_tool_control.rs#L411) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic](../../src/production_tool_control.rs#L448) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context](../../src/production_tool_control.rs#L479) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads](../../src/production_tool_control.rs#L551) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read](../../src/production_tool_control.rs#L587) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records](../../src/production_tool_control.rs#L603) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search](../../src/production_tool_control.rs#L632) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start](../../src/production_tool_control.rs#L716) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork](../../src/production_tool_control.rs#L750) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get](../../src/production_tool_control.rs#L770) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L848) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy](../../src/production_tool_control.rs#L856) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::new](../../src/production_tool_control.rs#L862) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply_continuation](../../src/production_tool_control.rs#L872) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply](../../src/production_tool_control.rs#L914) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::policy_withheld](../../src/production_tool_control.rs#L953) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::child_launch_proof](../../src/production_tool_control.rs#L965) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::parent_report_proof](../../src/production_tool_control.rs#L1037) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1099) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::resolve_visible_line](../../src/production_tool_control.rs#L1120) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1141) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::require_active_session](../../src/production_tool_control.rs#L1169) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1179) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::folder_workspace](../../src/production_tool_control.rs#L1227) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::genesis_workspace](../../src/production_tool_control.rs#L1242) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1252) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1260) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::event_record](../../src/production_tool_control.rs#L1276) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1282) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::string_array](../../src/production_tool_control.rs#L1290) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::job_record_outcome](../../src/production_tool_control.rs#L1305) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1318) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1337) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1350) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1355) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::context_start_genesis](../../src/production_tool_control.rs#L1384) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1412) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1432) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::bounded_snippet](../../src/production_tool_control.rs#L1445) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_io](../../src/production_tool_control.rs#L1464) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_store](../../src/production_tool_control.rs#L1472) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_schema](../../src/production_tool_control.rs#L1484) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorOperationError](../../src/production_tool_control.rs#L1489) | enum_item | `pub` |  |
| [tekes-supervisor::production_tool_control::SupervisorOperationError::into_control_error](../../src/production_tool_control.rs#L1519) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::tool_failure_tests::definitive_remote_tool_failure_is_not_retryable_transport_loss](../../src/production_tool_control.rs#L1546) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::continuation_policy_tests::value](../../src/production_tool_control.rs#L1565) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::continuation_policy_tests::pending_results_and_continuation_responses_pass_the_secret_policy](../../src/production_tool_control.rs#L1572) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::context_start_tests::a_started_session_inherits_the_callers_scope_and_preset](../../src/production_tool_control.rs#L1644) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `fs` | `std::fs` | `private` |
| `Component` | `std::path::Component` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `DynamicTool` | `profile::DynamicTool` | `private` |
| `DynamicToolCatalog` | `profile::DynamicToolCatalog` | `private` |
| `DynamicToolEffect` | `profile::DynamicToolEffect` | `private` |
| `Event` | `schema::Event` | `private` |
| `EventKind` | `schema::EventKind` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `ThreadStore` | `store::ThreadStore` | `private` |
| `scan_valid_prefix` | `store::scan_valid_prefix` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `BackendFailure` | `tools::BackendFailure` | `private` |
| `BackendGate` | `tools::BackendGate` | `private` |
| `BackendOutcome` | `tools::BackendOutcome` | `private` |
| `CancellationToken` | `tools::CancellationToken` | `private` |
| `JobBroker` | `tools::JobBroker` | `private` |
| `JobLaunchPolicy` | `tools::JobLaunchPolicy` | `private` |
| `JobSpec` | `tools::JobSpec` | `private` |
| `SandboxedJobLauncher` | `tools::SandboxedJobLauncher` | `private` |
| `SecretScan` | `tools::SecretScan` | `private` |
| `SecretScanner` | `tools::SecretScanner` | `private` |
| `validate_fixed_arguments` | `tools::validate_fixed_arguments` | `private` |
| `ToolContinuationOutcome` | `worker_control::continuation::ToolContinuationOutcome` | `private` |
| `ToolContinuationRequest` | `worker_control::continuation::ToolContinuationRequest` | `private` |
| `ToolContinuationResponse` | `worker_control::continuation::ToolContinuationResponse` | `private` |
| `ToolControl` | `worker_control::ToolControl` | `private` |
| `ToolControlError` | `worker_control::ToolControlError` | `private` |
| `ToolControlErrorCode` | `worker_control::ToolControlErrorCode` | `private` |
| `ToolControlResult` | `worker_control::ToolControlResult` | `private` |
| `ExternalEffectResolution` | `crate::tool_control::ExternalEffectResolution` | `private` |
| `ToolControlHandler` | `crate::tool_control::ToolControlHandler` | `private` |
| `ToolControlPolicy` | `crate::tool_control::ToolControlPolicy` | `private` |
| `ToolControlRecovery` | `crate::tool_control::ToolControlRecovery` | `private` |
| `resolve_durable_binding` | `crate::tool_control::resolve_durable_binding` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |
| `ContinuationOperation` | `worker_control::continuation::ContinuationOperation` | `private` |
| `ToolContinuationRequest` | `worker_control::continuation::ToolContinuationRequest` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::production_tool_control::tool_failure_tests` | `private` | #[cfg(test)] |
| `tekes-supervisor::production_tool_control::continuation_policy_tests` | `private` | #[cfg(test)] |
| `tekes-supervisor::production_tool_control::context_start_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 52 direct edges</summary>

```mermaid
flowchart TD
  n0["engine::dynamic_catalog::validate_dynamic_invocation"]
  n1["store::folder::ThreadStore::open"]
  n2["tekes-supervisor::production_tool_control::parent_report_proof"]
  n3["tekes-supervisor::production_tool_control::session_lines"]
  n4["tekes-supervisor::production_tool_control::visible_sessions"]
  n5["tekes-supervisor::production_tool_control::require_active_session"]
  n6["tekes-supervisor::production_tool_control::validate_session_folder"]
  n7["tekes-supervisor::production_tool_control::session_folder"]
  n8["tekes-supervisor::production_tool_control::read_projection"]
  n9["tekes-supervisor::production_tool_control::genesis_workspace"]
  n10["tekes-supervisor::production_tool_control::genesis_line"]
  n11["tekes-supervisor::production_tool_control::required_string"]
  n12["tekes-supervisor::production_tool_control::string_array"]
  n13["tekes-supervisor::production_tool_control::job_record_outcome"]
  n14["tekes-supervisor::production_tool_control::map_backend"]
  n15["tekes-supervisor::production_tool_control::argument_limit"]
  n16["tekes-supervisor::production_tool_control::encode_cursor"]
  n17["tekes-supervisor::production_tool_control::decode_cursor"]
  n18["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::execute_outcome"]
  n19["tekes-supervisor::production_tool_control::to_ijson"]
  n20["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::continue_task"]
  n21["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::reconcile"]
  n22["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::cancel_inflight"]
  n23["tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::new"]
  n24["tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::execute"]
  n25["tekes-supervisor::production_tool_control::ProductionToolControlHandler::new"]
  n26["tekes-supervisor::production_tool_control::ProductionToolControlHandler::with_dynamic_routes"]
  n27["tekes-supervisor::production_tool_control::ProductionToolControlHandler::runtime"]
  n28["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute"]
  n29["tekes-supervisor::production_tool_control::ProductionToolControlHandler::recovery"]
  n30["tekes-supervisor::production_tool_control::ProductionToolControlHandler::reconcile"]
  n31["tekes-supervisor::production_tool_control::ProductionToolControlHandler::continue_task"]
  n32["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_inner"]
  n33["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic"]
  n34["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context"]
  n35["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads"]
  n36["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read"]
  n37["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records"]
  n38["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search"]
  n39["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start"]
  n40["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork"]
  n41["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get"]
  n42["tekes-supervisor::production_tool_control::is_fixed_supervisor_tool"]
  n43["tekes-supervisor::production_tool_control::child_launch_proof"]
  n44["tekes-supervisor::tool_control::resolve_durable_binding"]
  n45["tools::runtime_backends::JobBroker::new"]
  n46["tools::schema_registry::validate_fixed_arguments"]
  n47["worker-control::durable::ToolControlResult::pending"]
  n48["worker-control::durable::ToolControlResult::success"]
  n49["worker-control::durable::ToolControlResult::failure"]
  n23 --> n45
  n24 --> n11
  n24 --> n12
  n24 --> n13
  n24 --> n14
  n24 --> n19
  n28 --> n47
  n28 --> n48
  n28 --> n49
  n29 --> n42
  n32 --> n2
  n32 --> n19
  n32 --> n33
  n32 --> n34
  n32 --> n41
  n32 --> n42
  n32 --> n43
  n32 --> n44
  n32 --> n46
  n33 --> n0
  n34 --> n1
  n34 --> n5
  n34 --> n7
  n34 --> n9
  n34 --> n11
  n34 --> n19
  n34 --> n35
  n34 --> n36
  n34 --> n37
  n34 --> n38
  n34 --> n39
  n34 --> n40
  n35 --> n6
  n35 --> n8
  n35 --> n19
  n36 --> n7
  n36 --> n8
  n36 --> n19
  n37 --> n11
  n37 --> n15
  n37 --> n16
  n37 --> n17
  n37 --> n19
  n38 --> n3
  n38 --> n4
  n38 --> n7
  n38 --> n10
  n38 --> n11
  n38 --> n15
  n38 --> n16
  n38 --> n17
  n38 --> n19
```

</details>

<details><summary>Functions 21–40: 35 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["store::folder::ThreadStore::open"]
  n2["store::tail::scan_valid_prefix"]
  n3["store::tail::read_genesis"]
  n4["tekes-supervisor::production_tool_control::parent_report_proof"]
  n5["tekes-supervisor::production_tool_control::session_lines"]
  n6["tekes-supervisor::production_tool_control::resolve_visible_line"]
  n7["tekes-supervisor::production_tool_control::visible_sessions"]
  n8["tekes-supervisor::production_tool_control::require_active_session"]
  n9["tekes-supervisor::production_tool_control::validate_session_folder"]
  n10["tekes-supervisor::production_tool_control::session_folder"]
  n11["tekes-supervisor::production_tool_control::read_projection"]
  n12["tekes-supervisor::production_tool_control::folder_workspace"]
  n13["tekes-supervisor::production_tool_control::genesis_workspace"]
  n14["tekes-supervisor::production_tool_control::genesis_line"]
  n15["tekes-supervisor::production_tool_control::paired_tool_timestamp"]
  n16["tekes-supervisor::production_tool_control::encode_cursor"]
  n17["tekes-supervisor::production_tool_control::decode_cursor"]
  n18["tekes-supervisor::production_tool_control::context_start_genesis"]
  n19["tekes-supervisor::production_tool_control::deterministic_uuid"]
  n20["tekes-supervisor::production_tool_control::validate_line_file_name"]
  n21["tekes-supervisor::production_tool_control::to_ijson"]
  n22["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start"]
  n23["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork"]
  n24["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get"]
  n25["tekes-supervisor::production_tool_control::is_fixed_supervisor_tool"]
  n26["tekes-supervisor::production_tool_control::ProductionToolControlPolicy::new"]
  n27["tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply_continuation"]
  n28["tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply"]
  n29["tekes-supervisor::production_tool_control::policy_withheld"]
  n30["tekes-supervisor::production_tool_control::child_launch_proof"]
  n31["worker-control::durable::ToolControlResult::failure"]
  n4 --> n11
  n4 --> n14
  n4 --> n20
  n5 --> n11
  n6 --> n5
  n6 --> n7
  n6 --> n10
  n6 --> n14
  n7 --> n9
  n7 --> n12
  n8 --> n10
  n9 --> n0
  n9 --> n3
  n10 --> n0
  n10 --> n9
  n11 --> n2
  n12 --> n3
  n22 --> n1
  n22 --> n13
  n22 --> n15
  n22 --> n18
  n22 --> n19
  n23 --> n1
  n23 --> n15
  n23 --> n19
  n23 --> n21
  n24 --> n6
  n24 --> n16
  n24 --> n17
  n24 --> n21
  n28 --> n29
  n29 --> n31
  n30 --> n11
  n30 --> n14
  n30 --> n20
```

</details>

<details><summary>Functions 41–58: 6 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::event::Event::from_value"]
  n1["schema::ijson::IJsonValue::parse"]
  n2["tekes-supervisor::production_tool_control::paired_tool_timestamp"]
  n3["tekes-supervisor::production_tool_control::event_record"]
  n4["tekes-supervisor::production_tool_control::required_string"]
  n5["tekes-supervisor::production_tool_control::string_array"]
  n6["tekes-supervisor::production_tool_control::job_record_outcome"]
  n7["tekes-supervisor::production_tool_control::map_backend"]
  n8["tekes-supervisor::production_tool_control::argument_limit"]
  n9["tekes-supervisor::production_tool_control::encode_cursor"]
  n10["tekes-supervisor::production_tool_control::decode_cursor"]
  n11["tekes-supervisor::production_tool_control::context_start_genesis"]
  n12["tekes-supervisor::production_tool_control::deterministic_uuid"]
  n13["tekes-supervisor::production_tool_control::validate_line_file_name"]
  n14["tekes-supervisor::production_tool_control::bounded_snippet"]
  n15["tekes-supervisor::production_tool_control::to_ijson"]
  n16["tekes-supervisor::production_tool_control::map_io"]
  n17["tekes-supervisor::production_tool_control::map_store"]
  n18["tekes-supervisor::production_tool_control::map_schema"]
  n19["tekes-supervisor::production_tool_control::SupervisorOperationError::into_control_error"]
  n6 --> n7
  n6 --> n15
  n10 --> n9
  n11 --> n0
  n11 --> n15
  n15 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `execute_outcome` | `self.execute(tool, request)             .map` | [150](../../src/production_tool_control.rs#L150) | receiver-type-required |
| `execute_outcome` | `self.execute` | [150](../../src/production_tool_control.rs#L150) | receiver-type-required |
| `continue_task` | `Err` | [162](../../src/production_tool_control.rs#L162) | external-constructor-callback-or-unresolved |
| `continue_task` | `SupervisorOperationError::Unsupported` | [162](../../src/production_tool_control.rs#L162) | external-constructor-callback-or-unresolved |
| `new` | `Ok` | [195](../../src/production_tool_control.rs#L195) | external-constructor-callback-or-unresolved |
| `new` | `JobBroker::new` | [196](../../src/production_tool_control.rs#L196) | [tools::runtime_backends::JobBroker::new](../../../tools/src/runtime_backends.rs#L566) |
| `execute` | `serde_json::to_value(&request.arguments)             .map_err` | [206](../../src/production_tool_control.rs#L206) | receiver-type-required |
| `execute` | `serde_json::to_value` | [206](../../src/production_tool_control.rs#L206) | external-constructor-callback-or-unresolved |
| `execute` | `SupervisorOperationError::Protocol` | [207](../../src/production_tool_control.rs#L207) | external-constructor-callback-or-unresolved |
| `execute` | `error.to_string` | [207](../../src/production_tool_control.rs#L207) | receiver-type-required |
| `execute` | `required_string` | [208](../../src/production_tool_control.rs#L208), [218](../../src/production_tool_control.rs#L218), [248](../../src/production_tool_control.rs#L248), [252](../../src/production_tool_control.rs#L252) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1282) |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter(&#124;value&#124; !value.is_empty())                     .map(ToOwned::to_owned)                     .unwrap_or_else` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter(&#124;value&#124; !value.is_empty())                     .map` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `value.is_empty` | [214](../../src/production_tool_control.rs#L214), [223](../../src/production_tool_control.rs#L223) | receiver-type-required |
| `execute` | `required_string(&arguments, "program")?.to_owned` | [218](../../src/production_tool_control.rs#L218) | receiver-type-required |
| `execute` | `string_array` | [219](../../src/production_tool_control.rs#L219), [225](../../src/production_tool_control.rs#L225) | [tekes-supervisor::production_tool_control::string_array](../../src/production_tool_control.rs#L1290) |
| `execute` | `arguments                         .get("working_directory")                         .and_then(Value::as_str)                         .filter(&#124;value&#124; !value.is_empty())                         .map` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get("working_directory")                         .and_then(Value::as_str)                         .filter` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get("working_directory")                         .and_then` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `Default::default` | [226](../../src/production_tool_control.rs#L226) | external-constructor-callback-or-unresolved |
| `execute` | `job_record_outcome` | [228](../../src/production_tool_control.rs#L228), [249](../../src/production_tool_control.rs#L249), [253](../../src/production_tool_control.rs#L253) | [tekes-supervisor::production_tool_control::job_record_outcome](../../src/production_tool_control.rs#L1305) |
| `execute` | `self.broker.start` | [228](../../src/production_tool_control.rs#L228) | receiver-type-required |
| `execute` | `Arc::clone` | [232](../../src/production_tool_control.rs#L232) | external-constructor-callback-or-unresolved |
| `execute` | `self.broker.list` | [237](../../src/production_tool_control.rs#L237) | receiver-type-required |
| `execute` | `to_ijson` | [238](../../src/production_tool_control.rs#L238) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `execute` | `Err` | [239](../../src/production_tool_control.rs#L239), [240](../../src/production_tool_control.rs#L240), [244](../../src/production_tool_control.rs#L244), [255](../../src/production_tool_control.rs#L255) | external-constructor-callback-or-unresolved |
| `execute` | `map_backend` | [239](../../src/production_tool_control.rs#L239) | [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1318) |
| `execute` | `SupervisorOperationError::Denied` | [240](../../src/production_tool_control.rs#L240) | external-constructor-callback-or-unresolved |
| `execute` | `"job list unexpectedly requested approval".into` | [241](../../src/production_tool_control.rs#L241) | receiver-type-required |
| `execute` | `SupervisorOperationError::Unavailable` | [244](../../src/production_tool_control.rs#L244) | external-constructor-callback-or-unresolved |
| `execute` | `self.broker.status` | [249](../../src/production_tool_control.rs#L249) | receiver-type-required |
| `execute` | `self.broker.stop` | [253](../../src/production_tool_control.rs#L253) | receiver-type-required |
| `execute` | `SupervisorOperationError::Unsupported` | [255](../../src/production_tool_control.rs#L255) | external-constructor-callback-or-unresolved |
| `execute` | `Ok` | [259](../../src/production_tool_control.rs#L259) | external-constructor-callback-or-unresolved |
| `new` | `root.into` | [275](../../src/production_tool_control.rs#L275) | receiver-type-required |
| `new` | `DynamicToolCatalog::default` | [278](../../src/production_tool_control.rs#L278) | external-constructor-callback-or-unresolved |
| `with_dynamic_routes` | `Some` | [290](../../src/production_tool_control.rs#L290) | external-constructor-callback-or-unresolved |
| `execute` | `self.execute_inner` | [304](../../src/production_tool_control.rs#L304) | receiver-type-required |
| `execute` | `ToolControlResult::success` | [305](../../src/production_tool_control.rs#L305) | [worker-control::durable::ToolControlResult::success](../../../worker-control/src/durable.rs#L360) |
| `execute` | `request.request_id.clone` | [306](../../src/production_tool_control.rs#L306), [314](../../src/production_tool_control.rs#L314), [320](../../src/production_tool_control.rs#L320) | receiver-type-required |
| `execute` | `request.call_id.clone` | [307](../../src/production_tool_control.rs#L307), [315](../../src/production_tool_control.rs#L315), [321](../../src/production_tool_control.rs#L321) | receiver-type-required |
| `execute` | `ToolControlResult::pending` | [313](../../src/production_tool_control.rs#L313) | [worker-control::durable::ToolControlResult::pending](../../../worker-control/src/durable.rs#L340) |
| `execute` | `ToolControlResult::failure` | [319](../../src/production_tool_control.rs#L319) | [worker-control::durable::ToolControlResult::failure](../../../worker-control/src/durable.rs#L375) |
| `execute` | `error.into_control_error` | [322](../../src/production_tool_control.rs#L322) | receiver-type-required |
| `recovery` | `is_fixed_supervisor_tool` | [328](../../src/production_tool_control.rs#L328) | [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L848) |
| `recovery` | `self             .dynamic_catalog             .tools             .iter()             .find` | [331](../../src/production_tool_control.rs#L331) | receiver-type-required |
| `recovery` | `self             .dynamic_catalog             .tools             .iter` | [331](../../src/production_tool_control.rs#L331) | receiver-type-required |
| `recovery` | `tool.external_effect.is_some` | [344](../../src/production_tool_control.rs#L344) | receiver-type-required |
| `reconcile` | `self             .dynamic_catalog             .tools             .iter()             .find` | [352](../../src/production_tool_control.rs#L352) | receiver-type-required |
| `reconcile` | `self             .dynamic_catalog             .tools             .iter` | [352](../../src/production_tool_control.rs#L352) | receiver-type-required |
| `reconcile` | `self.dynamic.as_ref().filter` | [362](../../src/production_tool_control.rs#L362) | receiver-type-required |
| `reconcile` | `self.dynamic.as_ref` | [362](../../src/production_tool_control.rs#L362) | receiver-type-required |
| `reconcile` | `route.supports` | [362](../../src/production_tool_control.rs#L362) | receiver-type-required |
| `reconcile` | `authority.reconcile` | [367](../../src/production_tool_control.rs#L367) | receiver-type-required |
| `continue_task` | `request.request_id.clone` | [377](../../src/production_tool_control.rs#L377) | receiver-type-required |
| `continue_task` | `request.original.call_id.clone` | [378](../../src/production_tool_control.rs#L378) | receiver-type-required |
| `continue_task` | `error.into_control_error` | [380](../../src/production_tool_control.rs#L380) | receiver-type-required |
| `continue_task` | `self             .dynamic_catalog             .tools             .iter()             .find` | [383](../../src/production_tool_control.rs#L383) | receiver-type-required |
| `continue_task` | `self             .dynamic_catalog             .tools             .iter` | [383](../../src/production_tool_control.rs#L383) | receiver-type-required |
| `continue_task` | `failed` | [389](../../src/production_tool_control.rs#L389), [395](../../src/production_tool_control.rs#L395), [407](../../src/production_tool_control.rs#L407) | external-constructor-callback-or-unresolved |
| `continue_task` | `SupervisorOperationError::Unsupported` | [389](../../src/production_tool_control.rs#L389), [395](../../src/production_tool_control.rs#L395) | external-constructor-callback-or-unresolved |
| `continue_task` | `self.dynamic.as_ref().filter` | [394](../../src/production_tool_control.rs#L394) | receiver-type-required |
| `continue_task` | `self.dynamic.as_ref` | [394](../../src/production_tool_control.rs#L394) | receiver-type-required |
| `continue_task` | `route.supports` | [394](../../src/production_tool_control.rs#L394) | receiver-type-required |
| `continue_task` | `self             .root             .join("threads")             .join(&request.original.session)             .join` | [400](../../src/production_tool_control.rs#L400) | receiver-type-required |
| `continue_task` | `self             .root             .join("threads")             .join` | [400](../../src/production_tool_control.rs#L400) | receiver-type-required |
| `continue_task` | `self             .root             .join` | [400](../../src/production_tool_control.rs#L400) | receiver-type-required |
| `continue_task` | `authority.continue_task` | [405](../../src/production_tool_control.rs#L405) | receiver-type-required |
| `execute_inner` | `serde_json::to_value(&request.arguments)             .map_err` | [415](../../src/production_tool_control.rs#L415) | receiver-type-required |
| `execute_inner` | `serde_json::to_value` | [415](../../src/production_tool_control.rs#L415) | external-constructor-callback-or-unresolved |
| `execute_inner` | `SupervisorOperationError::Protocol` | [416](../../src/production_tool_control.rs#L416) | external-constructor-callback-or-unresolved |
| `execute_inner` | `error.to_string` | [416](../../src/production_tool_control.rs#L416), [419](../../src/production_tool_control.rs#L419), [424](../../src/production_tool_control.rs#L424) | receiver-type-required |
| `execute_inner` | `self.root.join("threads").join` | [417](../../src/production_tool_control.rs#L417) | receiver-type-required |
| `execute_inner` | `self.root.join` | [417](../../src/production_tool_control.rs#L417) | receiver-type-required |
| `execute_inner` | `resolve_durable_binding(&session_folder, request)             .map_err` | [418](../../src/production_tool_control.rs#L418) | receiver-type-required |
| `execute_inner` | `resolve_durable_binding` | [418](../../src/production_tool_control.rs#L418) | [tekes-supervisor::tool_control::resolve_durable_binding](../../src/tool_control.rs#L546) |
| `execute_inner` | `SupervisorOperationError::Conflict` | [419](../../src/production_tool_control.rs#L419) | external-constructor-callback-or-unresolved |
| `execute_inner` | `is_fixed_supervisor_tool` | [420](../../src/production_tool_control.rs#L420) | [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L848) |
| `execute_inner` | `self.execute_dynamic` | [421](../../src/production_tool_control.rs#L421) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic](../../src/production_tool_control.rs#L448) |
| `execute_inner` | `session_folder.join` | [421](../../src/production_tool_control.rs#L421) | receiver-type-required |
| `execute_inner` | `validate_fixed_arguments(&request.name, &arguments)             .map_err` | [423](../../src/production_tool_control.rs#L423) | receiver-type-required |
| `execute_inner` | `validate_fixed_arguments` | [423](../../src/production_tool_control.rs#L423) | [tools::schema_registry::validate_fixed_arguments](../../../tools/src/schema_registry.rs#L1231) |
| `execute_inner` | `SupervisorOperationError::Invalid` | [424](../../src/production_tool_control.rs#L424), [436](../../src/production_tool_control.rs#L436) | external-constructor-callback-or-unresolved |
| `execute_inner` | `request.name.as_str` | [425](../../src/production_tool_control.rs#L425) | receiver-type-required |
| `execute_inner` | `self.execute_context` | [426](../../src/production_tool_control.rs#L426) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context](../../src/production_tool_control.rs#L479) |
| `execute_inner` | `self.execute_context_get` | [427](../../src/production_tool_control.rs#L427) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get](../../src/production_tool_control.rs#L770) |
| `execute_inner` | `self.jobs.execute` | [428](../../src/production_tool_control.rs#L428) | receiver-type-required |
| `execute_inner` | `child_launch_proof` | [430](../../src/production_tool_control.rs#L430) | [tekes-supervisor::production_tool_control::child_launch_proof](../../src/production_tool_control.rs#L965) |
| `execute_inner` | `self.runtime.ensure_child` | [431](../../src/production_tool_control.rs#L431) | receiver-type-required |
| `execute_inner` | `parent_report_proof` | [434](../../src/production_tool_control.rs#L434) | [tekes-supervisor::production_tool_control::parent_report_proof](../../src/production_tool_control.rs#L1037) |
| `execute_inner` | `arguments.get("result").ok_or_else` | [435](../../src/production_tool_control.rs#L435) | receiver-type-required |
| `execute_inner` | `arguments.get` | [435](../../src/production_tool_control.rs#L435) | receiver-type-required |
| `execute_inner` | `"report.result missing".into` | [436](../../src/production_tool_control.rs#L436) | receiver-type-required |
| `execute_inner` | `to_ijson` | [438](../../src/production_tool_control.rs#L438) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `execute_inner` | `self.runtime.deliver_report` | [439](../../src/production_tool_control.rs#L439) | receiver-type-required |
| `execute_inner` | `Err` | [441](../../src/production_tool_control.rs#L441) | external-constructor-callback-or-unresolved |
| `execute_inner` | `SupervisorOperationError::Unsupported` | [441](../../src/production_tool_control.rs#L441) | external-constructor-callback-or-unresolved |
| `execute_inner` | `Ok` | [445](../../src/production_tool_control.rs#L445) | external-constructor-callback-or-unresolved |
| `execute_inner` | `DynamicExecutionOutcome::Value` | [445](../../src/production_tool_control.rs#L445) | external-constructor-callback-or-unresolved |
| `execute_dynamic` | `self             .dynamic_catalog             .tools             .iter()             .find(&#124;tool&#124; tool.name == request.name)             .ok_or_else` | [453](../../src/production_tool_control.rs#L453) | receiver-type-required |
| `execute_dynamic` | `self             .dynamic_catalog             .tools             .iter()             .find` | [453](../../src/production_tool_control.rs#L453) | receiver-type-required |
| `execute_dynamic` | `self             .dynamic_catalog             .tools             .iter` | [453](../../src/production_tool_control.rs#L453) | receiver-type-required |
| `execute_dynamic` | `SupervisorOperationError::Unsupported` | [459](../../src/production_tool_control.rs#L459), [471](../../src/production_tool_control.rs#L471) | external-constructor-callback-or-unresolved |
| `execute_dynamic` | `engine::validate_dynamic_invocation(tool, &request.arguments)             .map_err` | [464](../../src/production_tool_control.rs#L464) | receiver-type-required |
| `execute_dynamic` | `engine::validate_dynamic_invocation` | [464](../../src/production_tool_control.rs#L464) | [engine::dynamic_catalog::validate_dynamic_invocation](../../../engine/src/dynamic_catalog.rs#L140) |
| `execute_dynamic` | `SupervisorOperationError::Invalid` | [465](../../src/production_tool_control.rs#L465) | external-constructor-callback-or-unresolved |
| `execute_dynamic` | `error.to_string` | [465](../../src/production_tool_control.rs#L465) | receiver-type-required |
| `execute_dynamic` | `self             .dynamic             .as_ref()             .filter(&#124;route&#124; route.supports(tool))             .ok_or_else` | [466](../../src/production_tool_control.rs#L466) | receiver-type-required |
| `execute_dynamic` | `self             .dynamic             .as_ref()             .filter` | [466](../../src/production_tool_control.rs#L466) | receiver-type-required |
| `execute_dynamic` | `self             .dynamic             .as_ref` | [466](../../src/production_tool_control.rs#L466) | receiver-type-required |
| `execute_dynamic` | `route.supports` | [469](../../src/production_tool_control.rs#L469) | receiver-type-required |
| `execute_dynamic` | `authority.execute_outcome` | [476](../../src/production_tool_control.rs#L476) | receiver-type-required |
| `execute_context` | `arguments             .get("operation")             .and_then(Value::as_str)             .ok_or_else` | [485](../../src/production_tool_control.rs#L485) | receiver-type-required |
| `execute_context` | `arguments             .get("operation")             .and_then` | [485](../../src/production_tool_control.rs#L485) | receiver-type-required |
| `execute_context` | `arguments             .get` | [485](../../src/production_tool_control.rs#L485) | receiver-type-required |
| `execute_context` | `SupervisorOperationError::Invalid` | [488](../../src/production_tool_control.rs#L488) | external-constructor-callback-or-unresolved |
| `execute_context` | `"context.operation missing".into` | [488](../../src/production_tool_control.rs#L488) | receiver-type-required |
| `execute_context` | `genesis_workspace` | [489](../../src/production_tool_control.rs#L489) | [tekes-supervisor::production_tool_control::genesis_workspace](../../src/production_tool_control.rs#L1242) |
| `execute_context` | `self.context_threads` | [491](../../src/production_tool_control.rs#L491) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads](../../src/production_tool_control.rs#L551) |
| `execute_context` | `required_string` | [493](../../src/production_tool_control.rs#L493), [501](../../src/production_tool_control.rs#L501), [503](../../src/production_tool_control.rs#L503), [517](../../src/production_tool_control.rs#L517), [522](../../src/production_tool_control.rs#L522), [536](../../src/production_tool_control.rs#L536) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1282) |
| `execute_context` | `self.context_read` | [494](../../src/production_tool_control.rs#L494) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read](../../src/production_tool_control.rs#L587) |
| `execute_context` | `self.context_records` | [496](../../src/production_tool_control.rs#L496) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records](../../src/production_tool_control.rs#L603) |
| `execute_context` | `self.context_search` | [497](../../src/production_tool_control.rs#L497) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search](../../src/production_tool_control.rs#L632) |
| `execute_context` | `self.context_start` | [498](../../src/production_tool_control.rs#L498) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start](../../src/production_tool_control.rs#L716) |
| `execute_context` | `self.context_fork` | [499](../../src/production_tool_control.rs#L499) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork](../../src/production_tool_control.rs#L750) |
| `execute_context` | `session_folder` | [502](../../src/production_tool_control.rs#L502), [518](../../src/production_tool_control.rs#L518), [537](../../src/production_tool_control.rs#L537) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) |
| `execute_context` | `self.runtime.deliver_input` | [504](../../src/production_tool_control.rs#L504) | receiver-type-required |
| `execute_context` | `request.session.clone` | [505](../../src/production_tool_control.rs#L505), [539](../../src/production_tool_control.rs#L539) | receiver-type-required |
| `execute_context` | `request.thread.clone` | [506](../../src/production_tool_control.rs#L506), [540](../../src/production_tool_control.rs#L540) | receiver-type-required |
| `execute_context` | `target.to_owned` | [507](../../src/production_tool_control.rs#L507), [541](../../src/production_tool_control.rs#L541) | receiver-type-required |
| `execute_context` | `request.request_id.clone` | [508](../../src/production_tool_control.rs#L508), [542](../../src/production_tool_control.rs#L542) | receiver-type-required |
| `execute_context` | `message.to_owned` | [509](../../src/production_tool_control.rs#L509) | receiver-type-required |
| `execute_context` | `arguments                         .get("goal")                         .and_then(Value::as_bool)                         .unwrap_or` | [510](../../src/production_tool_control.rs#L510) | receiver-type-required |
| `execute_context` | `arguments                         .get("goal")                         .and_then` | [510](../../src/production_tool_control.rs#L510) | receiver-type-required |
| `execute_context` | `arguments                         .get` | [510](../../src/production_tool_control.rs#L510) | receiver-type-required |
| `execute_context` | `self.runtime.ensure_running` | [519](../../src/production_tool_control.rs#L519) | receiver-type-required |
| `execute_context` | `Err` | [524](../../src/production_tool_control.rs#L524), [545](../../src/production_tool_control.rs#L545) | external-constructor-callback-or-unresolved |
| `execute_context` | `SupervisorOperationError::Denied` | [524](../../src/production_tool_control.rs#L524) | external-constructor-callback-or-unresolved |
| `execute_context` | `"a running worker cannot archive its containing session".into` | [525](../../src/production_tool_control.rs#L525) | receiver-type-required |
| `execute_context` | `require_active_session` | [528](../../src/production_tool_control.rs#L528) | [tekes-supervisor::production_tool_control::require_active_session](../../src/production_tool_control.rs#L1169) |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err(map_store)?                     .archive(target)                     .map_err` | [529](../../src/production_tool_control.rs#L529) | receiver-type-required |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err(map_store)?                     .archive` | [529](../../src/production_tool_control.rs#L529) | receiver-type-required |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err` | [529](../../src/production_tool_control.rs#L529) | receiver-type-required |
| `execute_context` | `ThreadStore::open` | [529](../../src/production_tool_control.rs#L529) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `execute_context` | `to_ijson` | [533](../../src/production_tool_control.rs#L533) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `execute_context` | `self.runtime.interrupt` | [538](../../src/production_tool_control.rs#L538) | receiver-type-required |
| `execute_context` | `SupervisorOperationError::Unsupported` | [545](../../src/production_tool_control.rs#L545) | external-constructor-callback-or-unresolved |
| `context_threads` | `Vec::new` | [552](../../src/production_tool_control.rs#L552) | external-constructor-callback-or-unresolved |
| `context_threads` | `fs::read_dir(self.root.join(area)).map_err` | [554](../../src/production_tool_control.rs#L554) | receiver-type-required |
| `context_threads` | `fs::read_dir` | [554](../../src/production_tool_control.rs#L554) | external-constructor-callback-or-unresolved |
| `context_threads` | `self.root.join` | [554](../../src/production_tool_control.rs#L554) | receiver-type-required |
| `context_threads` | `entry.map_err` | [555](../../src/production_tool_control.rs#L555) | receiver-type-required |
| `context_threads` | `entry.file_type().map_err(map_io)?.is_dir` | [556](../../src/production_tool_control.rs#L556) | receiver-type-required |
| `context_threads` | `entry.file_type().map_err` | [556](../../src/production_tool_control.rs#L556) | receiver-type-required |
| `context_threads` | `entry.file_type` | [556](../../src/production_tool_control.rs#L556) | receiver-type-required |
| `context_threads` | `entry.file_name().to_string_lossy().into_owned` | [559](../../src/production_tool_control.rs#L559) | receiver-type-required |
| `context_threads` | `entry.file_name().to_string_lossy` | [559](../../src/production_tool_control.rs#L559) | receiver-type-required |
| `context_threads` | `entry.file_name` | [559](../../src/production_tool_control.rs#L559) | receiver-type-required |
| `context_threads` | `validate_session_folder` | [560](../../src/production_tool_control.rs#L560) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1179) |
| `context_threads` | `entry.path` | [560](../../src/production_tool_control.rs#L560), [561](../../src/production_tool_control.rs#L561) | receiver-type-required |
| `context_threads` | `read_projection` | [561](../../src/production_tool_control.rs#L561) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) |
| `context_threads` | `entry.path().join` | [561](../../src/production_tool_control.rs#L561) | receiver-type-required |
| `context_threads` | `projection.events.first().expect` | [562](../../src/production_tool_control.rs#L562) | receiver-type-required |
| `context_threads` | `projection.events.first` | [562](../../src/production_tool_control.rs#L562) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter()                     .rev()                     .find_map` | [563](../../src/production_tool_control.rs#L563) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter()                     .rev` | [563](../../src/production_tool_control.rs#L563) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter` | [563](../../src/production_tool_control.rs#L563) | receiver-type-required |
| `context_threads` | `event.string_field` | [567](../../src/production_tool_control.rs#L567) | receiver-type-required |
| `context_threads` | `items.push` | [568](../../src/production_tool_control.rs#L568) | receiver-type-required |
| `context_threads` | `items.sort_by` | [579](../../src/production_tool_control.rs#L579) | receiver-type-required |
| `context_threads` | `left.get("session_id")                 .and_then(Value::as_str)                 .cmp` | [580](../../src/production_tool_control.rs#L580) | receiver-type-required |
| `context_threads` | `left.get("session_id")                 .and_then` | [580](../../src/production_tool_control.rs#L580) | receiver-type-required |
| `context_threads` | `left.get` | [580](../../src/production_tool_control.rs#L580) | receiver-type-required |
| `context_threads` | `right.get("session_id").and_then` | [582](../../src/production_tool_control.rs#L582) | receiver-type-required |
| `context_threads` | `right.get` | [582](../../src/production_tool_control.rs#L582) | receiver-type-required |
| `context_threads` | `to_ijson` | [584](../../src/production_tool_control.rs#L584) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `context_read` | `session_folder` | [588](../../src/production_tool_control.rs#L588) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) |
| `context_read` | `read_projection` | [589](../../src/production_tool_control.rs#L589) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) |
| `context_read` | `folder.join` | [589](../../src/production_tool_control.rs#L589) | receiver-type-required |
| `context_read` | `projection.events.first().expect` | [590](../../src/production_tool_control.rs#L590) | receiver-type-required |
| `context_read` | `projection.events.first` | [590](../../src/production_tool_control.rs#L590) | receiver-type-required |
| `context_read` | `to_ijson` | [591](../../src/production_tool_control.rs#L591) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `context_records` | `required_string(arguments, "turn_id")?             .parse::<u64>()             .map_err` | [608](../../src/production_tool_control.rs#L608) | receiver-type-required |
| `context_records` | `required_string(arguments, "turn_id")?             .parse::<u64>` | [608](../../src/production_tool_control.rs#L608) | receiver-type-required |
| `context_records` | `required_string` | [608](../../src/production_tool_control.rs#L608) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1282) |
| `context_records` | `SupervisorOperationError::Invalid` | [611](../../src/production_tool_control.rs#L611) | external-constructor-callback-or-unresolved |
| `context_records` | `"turn_id is not a positive integer".into` | [611](../../src/production_tool_control.rs#L611) | receiver-type-required |
| `context_records` | `decode_cursor` | [613](../../src/production_tool_control.rs#L613) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1355) |
| `context_records` | `arguments.get` | [613](../../src/production_tool_control.rs#L613) | receiver-type-required |
| `context_records` | `argument_limit` | [614](../../src/production_tool_control.rs#L614) | [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1337) |
| `context_records` | `resolved             .projection             .events             .iter()             .filter(&#124;event&#124; event.turn() == Some(turn))             .collect::<Vec<_>>` | [615](../../src/production_tool_control.rs#L615) | receiver-type-required |
| `context_records` | `resolved             .projection             .events             .iter()             .filter` | [615](../../src/production_tool_control.rs#L615) | receiver-type-required |
| `context_records` | `resolved             .projection             .events             .iter` | [615](../../src/production_tool_control.rs#L615) | receiver-type-required |
| `context_records` | `event.turn` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `Some` | [619](../../src/production_tool_control.rs#L619) | external-constructor-callback-or-unresolved |
| `context_records` | `matching             .iter()             .skip(offset)             .take(limit)             .map(event_record)             .collect::<Result<Vec<_>, _>>` | [621](../../src/production_tool_control.rs#L621) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip(offset)             .take(limit)             .map` | [621](../../src/production_tool_control.rs#L621) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip(offset)             .take` | [621](../../src/production_tool_control.rs#L621) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip` | [621](../../src/production_tool_control.rs#L621) | receiver-type-required |
| `context_records` | `matching             .iter` | [621](../../src/production_tool_control.rs#L621) | receiver-type-required |
| `context_records` | `(offset + records.len() < matching.len())             .then` | [627](../../src/production_tool_control.rs#L627) | receiver-type-required |
| `context_records` | `records.len` | [627](../../src/production_tool_control.rs#L627), [628](../../src/production_tool_control.rs#L628) | receiver-type-required |
| `context_records` | `matching.len` | [627](../../src/production_tool_control.rs#L627) | receiver-type-required |
| `context_records` | `encode_cursor` | [628](../../src/production_tool_control.rs#L628) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1350) |
| `context_records` | `to_ijson` | [629](../../src/production_tool_control.rs#L629) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `context_search` | `required_string` | [638](../../src/production_tool_control.rs#L638), [647](../../src/production_tool_control.rs#L647), [663](../../src/production_tool_control.rs#L663) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1282) |
| `context_search` | `query.to_lowercase` | [639](../../src/production_tool_control.rs#L639) | receiver-type-required |
| `context_search` | `arguments             .get("scope")             .and_then(Value::as_str)             .unwrap_or` | [640](../../src/production_tool_control.rs#L640) | receiver-type-required |
| `context_search` | `arguments             .get("scope")             .and_then` | [640](../../src/production_tool_control.rs#L640) | receiver-type-required |
| `context_search` | `arguments             .get` | [640](../../src/production_tool_control.rs#L640) | receiver-type-required |
| `context_search` | `session_folder` | [648](../../src/production_tool_control.rs#L648), [673](../../src/production_tool_control.rs#L673) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) |
| `context_search` | `visible_sessions` | [651](../../src/production_tool_control.rs#L651), [652](../../src/production_tool_control.rs#L652) | [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1141) |
| `context_search` | `Some` | [651](../../src/production_tool_control.rs#L651) | external-constructor-callback-or-unresolved |
| `context_search` | `Err` | [654](../../src/production_tool_control.rs#L654) | external-constructor-callback-or-unresolved |
| `context_search` | `SupervisorOperationError::Invalid` | [654](../../src/production_tool_control.rs#L654) | external-constructor-callback-or-unresolved |
| `context_search` | `"invalid context.search scope".into` | [655](../../src/production_tool_control.rs#L655) | receiver-type-required |
| `context_search` | `request.session.as_str` | [662](../../src/production_tool_control.rs#L662) | receiver-type-required |
| `context_search` | `json!(["context-search-v2", scope, target, query]).to_string` | [668](../../src/production_tool_control.rs#L668) | receiver-type-required |
| `context_search` | `decode_cursor` | [669](../../src/production_tool_control.rs#L669) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1355) |
| `context_search` | `arguments.get` | [669](../../src/production_tool_control.rs#L669) | receiver-type-required |
| `context_search` | `argument_limit` | [670](../../src/production_tool_control.rs#L670) | [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1337) |
| `context_search` | `Vec::new` | [671](../../src/production_tool_control.rs#L671) | external-constructor-callback-or-unresolved |
| `context_search` | `session_lines` | [674](../../src/production_tool_control.rs#L674) | [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1099) |
| `context_search` | `genesis_line` | [675](../../src/production_tool_control.rs#L675) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1252) |
| `context_search` | `String::from_utf8(event.canonical_bytes().map_err(map_schema)?)                         .map_err` | [677](../../src/production_tool_control.rs#L677) | receiver-type-required |
| `context_search` | `String::from_utf8` | [677](../../src/production_tool_control.rs#L677) | external-constructor-callback-or-unresolved |
| `context_search` | `event.canonical_bytes().map_err` | [677](../../src/production_tool_control.rs#L677) | receiver-type-required |
| `context_search` | `event.canonical_bytes` | [677](../../src/production_tool_control.rs#L677) | receiver-type-required |
| `context_search` | `SupervisorOperationError::Protocol` | [678](../../src/production_tool_control.rs#L678) | external-constructor-callback-or-unresolved |
| `context_search` | `error.to_string` | [678](../../src/production_tool_control.rs#L678) | receiver-type-required |
| `context_search` | `canonical.to_lowercase().contains` | [679](../../src/production_tool_control.rs#L679) | receiver-type-required |
| `context_search` | `canonical.to_lowercase` | [679](../../src/production_tool_control.rs#L679) | receiver-type-required |
| `context_search` | `hits.push` | [680](../../src/production_tool_control.rs#L680) | receiver-type-required |
| `context_search` | `hits.sort_by` | [693](../../src/production_tool_control.rs#L693) | receiver-type-required |
| `context_search` | `(                 left.get("session_id").and_then(Value::as_str),                 left.get("thread_id").and_then(Value::as_str),                 left.get("record_id").and_then(Value::as_u64),             )                 .cmp` | [694](../../src/production_tool_control.rs#L694) | receiver-type-required |
| `context_search` | `left.get("session_id").and_then` | [695](../../src/production_tool_control.rs#L695) | receiver-type-required |
| `context_search` | `left.get` | [695](../../src/production_tool_control.rs#L695), [696](../../src/production_tool_control.rs#L696), [697](../../src/production_tool_control.rs#L697) | receiver-type-required |
| `context_search` | `left.get("thread_id").and_then` | [696](../../src/production_tool_control.rs#L696) | receiver-type-required |
| `context_search` | `left.get("record_id").and_then` | [697](../../src/production_tool_control.rs#L697) | receiver-type-required |
| `context_search` | `right.get("session_id").and_then` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `right.get` | [700](../../src/production_tool_control.rs#L700), [701](../../src/production_tool_control.rs#L701), [702](../../src/production_tool_control.rs#L702) | receiver-type-required |
| `context_search` | `right.get("thread_id").and_then` | [701](../../src/production_tool_control.rs#L701) | receiver-type-required |
| `context_search` | `right.get("record_id").and_then` | [702](../../src/production_tool_control.rs#L702) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take(limit)             .cloned()             .collect::<Vec<_>>` | [705](../../src/production_tool_control.rs#L705) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take(limit)             .cloned` | [705](../../src/production_tool_control.rs#L705) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take` | [705](../../src/production_tool_control.rs#L705) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip` | [705](../../src/production_tool_control.rs#L705) | receiver-type-required |
| `context_search` | `hits             .iter` | [705](../../src/production_tool_control.rs#L705) | receiver-type-required |
| `context_search` | `(offset + page.len() < hits.len())             .then` | [711](../../src/production_tool_control.rs#L711) | receiver-type-required |
| `context_search` | `page.len` | [711](../../src/production_tool_control.rs#L711), [712](../../src/production_tool_control.rs#L712) | receiver-type-required |
| `context_search` | `hits.len` | [711](../../src/production_tool_control.rs#L711) | receiver-type-required |
| `context_search` | `encode_cursor` | [712](../../src/production_tool_control.rs#L712) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1350) |
| `context_search` | `to_ijson` | [713](../../src/production_tool_control.rs#L713) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `context_start` | `resolved.projection.events.first().ok_or_else` | [722](../../src/production_tool_control.rs#L722) | receiver-type-required |
| `context_start` | `resolved.projection.events.first` | [722](../../src/production_tool_control.rs#L722) | receiver-type-required |
| `context_start` | `SupervisorOperationError::Corruption` | [723](../../src/production_tool_control.rs#L723) | external-constructor-callback-or-unresolved |
| `context_start` | `"caller has no genesis".into` | [723](../../src/production_tool_control.rs#L723) | receiver-type-required |
| `context_start` | `serde_json::to_value(genesis.raw())             .map_err` | [725](../../src/production_tool_control.rs#L725) | receiver-type-required |
| `context_start` | `serde_json::to_value` | [725](../../src/production_tool_control.rs#L725) | external-constructor-callback-or-unresolved |
| `context_start` | `genesis.raw` | [725](../../src/production_tool_control.rs#L725) | receiver-type-required |
| `context_start` | `SupervisorOperationError::Protocol` | [726](../../src/production_tool_control.rs#L726) | external-constructor-callback-or-unresolved |
| `context_start` | `error.to_string` | [726](../../src/production_tool_control.rs#L726) | receiver-type-required |
| `context_start` | `deterministic_uuid` | [727](../../src/production_tool_control.rs#L727) | [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1412) |
| `context_start` | `paired_tool_timestamp` | [728](../../src/production_tool_control.rs#L728) | [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1260) |
| `context_start` | `"kernel-worker".into` | [730](../../src/production_tool_control.rs#L730) | receiver-type-required |
| `context_start` | `"context".into` | [731](../../src/production_tool_control.rs#L731) | receiver-type-required |
| `context_start` | `session.clone` | [732](../../src/production_tool_control.rs#L732) | receiver-type-required |
| `context_start` | `"create".into` | [733](../../src/production_tool_control.rs#L733) | receiver-type-required |
| `context_start` | `request.request_id.clone` | [734](../../src/production_tool_control.rs#L734) | receiver-type-required |
| `context_start` | `context_start_genesis` | [736](../../src/production_tool_control.rs#L736) | [tekes-supervisor::production_tool_control::context_start_genesis](../../src/production_tool_control.rs#L1384) |
| `context_start` | `genesis_workspace` | [738](../../src/production_tool_control.rs#L738) | [tekes-supervisor::production_tool_control::genesis_workspace](../../src/production_tool_control.rs#L1242) |
| `context_start` | `ThreadStore::open(&self.root)             .map_err(map_store)?             .create_thread(&session, event)             .map_err` | [743](../../src/production_tool_control.rs#L743) | receiver-type-required |
| `context_start` | `ThreadStore::open(&self.root)             .map_err(map_store)?             .create_thread` | [743](../../src/production_tool_control.rs#L743) | receiver-type-required |
| `context_start` | `ThreadStore::open(&self.root)             .map_err` | [743](../../src/production_tool_control.rs#L743) | receiver-type-required |
| `context_start` | `ThreadStore::open` | [743](../../src/production_tool_control.rs#L743) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `context_start` | `self.runtime.ensure_running` | [747](../../src/production_tool_control.rs#L747) | receiver-type-required |
| `context_fork` | `deterministic_uuid` | [755](../../src/production_tool_control.rs#L755) | [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1412) |
| `context_fork` | `paired_tool_timestamp` | [756](../../src/production_tool_control.rs#L756) | [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1260) |
| `context_fork` | `ThreadStore::open(&self.root).map_err` | [757](../../src/production_tool_control.rs#L757) | receiver-type-required |
| `context_fork` | `ThreadStore::open` | [757](../../src/production_tool_control.rs#L757) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `context_fork` | `store             .begin_fork(                 &request.request_id,                 &request.session,                 &destination,                 timestamp,             )             .map_err` | [758](../../src/production_tool_control.rs#L758) | receiver-type-required |
| `context_fork` | `store             .begin_fork` | [758](../../src/production_tool_control.rs#L758) | receiver-type-required |
| `context_fork` | `store.recover_rewrites().map_err` | [766](../../src/production_tool_control.rs#L766) | receiver-type-required |
| `context_fork` | `store.recover_rewrites` | [766](../../src/production_tool_control.rs#L766) | receiver-type-required |
| `context_fork` | `to_ijson` | [767](../../src/production_tool_control.rs#L767) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `execute_context_get` | `arguments.get("thread_id").and_then` | [776](../../src/production_tool_control.rs#L776) | receiver-type-required |
| `execute_context_get` | `arguments.get` | [776](../../src/production_tool_control.rs#L776), [805](../../src/production_tool_control.rs#L805) | receiver-type-required |
| `execute_context_get` | `resolve_visible_line` | [777](../../src/production_tool_control.rs#L777) | [tekes-supervisor::production_tool_control::resolve_visible_line](../../src/production_tool_control.rs#L1120) |
| `execute_context_get` | `resolved.projection.clone` | [778](../../src/production_tool_control.rs#L778) | receiver-type-required |
| `execute_context_get` | `arguments             .get("record_ids")             .and_then(Value::as_array)             .ok_or_else` | [780](../../src/production_tool_control.rs#L780) | receiver-type-required |
| `execute_context_get` | `arguments             .get("record_ids")             .and_then` | [780](../../src/production_tool_control.rs#L780) | receiver-type-required |
| `execute_context_get` | `arguments             .get` | [780](../../src/production_tool_control.rs#L780), [806](../../src/production_tool_control.rs#L806) | receiver-type-required |
| `execute_context_get` | `SupervisorOperationError::Invalid` | [784](../../src/production_tool_control.rs#L784), [790](../../src/production_tool_control.rs#L790), [817](../../src/production_tool_control.rs#L817) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `"context_get.record_ids missing".into` | [784](../../src/production_tool_control.rs#L784) | receiver-type-required |
| `execute_context_get` | `Vec::new` | [786](../../src/production_tool_control.rs#L786), [810](../../src/production_tool_control.rs#L810) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `BTreeSet::new` | [787](../../src/production_tool_control.rs#L787) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `id.as_u64().ok_or_else` | [789](../../src/production_tool_control.rs#L789) | receiver-type-required |
| `execute_context_get` | `id.as_u64` | [789](../../src/production_tool_control.rs#L789) | receiver-type-required |
| `execute_context_get` | `"record id is not an integer".into` | [790](../../src/production_tool_control.rs#L790) | receiver-type-required |
| `execute_context_get` | `seen.insert` | [792](../../src/production_tool_control.rs#L792) | receiver-type-required |
| `execute_context_get` | `ordered.push` | [793](../../src/production_tool_control.rs#L793) | receiver-type-required |
| `execute_context_get` | `decode_cursor` | [805](../../src/production_tool_control.rs#L805) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1355) |
| `execute_context_get` | `arguments             .get("page_bytes")             .and_then(Value::as_u64)             .map_or` | [806](../../src/production_tool_control.rs#L806) | receiver-type-required |
| `execute_context_get` | `arguments             .get("page_bytes")             .and_then` | [806](../../src/production_tool_control.rs#L806) | receiver-type-required |
| `execute_context_get` | `ordered.iter().skip` | [813](../../src/production_tool_control.rs#L813) | receiver-type-required |
| `execute_context_get` | `ordered.iter` | [813](../../src/production_tool_control.rs#L813) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get(usize::try_from(*id - 1).map_err(&#124;_&#124; {                     SupervisorOperationError::Invalid("record id does not fit this platform".into())                 })?)                 .filter(&#124;event&#124; event.seq() == *id)                 .ok_or_else` | [814](../../src/production_tool_control.rs#L814) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get(usize::try_from(*id - 1).map_err(&#124;_&#124; {                     SupervisorOperationError::Invalid("record id does not fit this platform".into())                 })?)                 .filter` | [814](../../src/production_tool_control.rs#L814) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get` | [814](../../src/production_tool_control.rs#L814) | receiver-type-required |
| `execute_context_get` | `usize::try_from(*id - 1).map_err` | [816](../../src/production_tool_control.rs#L816) | receiver-type-required |
| `execute_context_get` | `usize::try_from` | [816](../../src/production_tool_control.rs#L816) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `"record id does not fit this platform".into` | [817](../../src/production_tool_control.rs#L817) | receiver-type-required |
| `execute_context_get` | `event.seq` | [819](../../src/production_tool_control.rs#L819) | receiver-type-required |
| `execute_context_get` | `SupervisorOperationError::NotFound` | [820](../../src/production_tool_control.rs#L820) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `event.canonical_bytes().map_err` | [821](../../src/production_tool_control.rs#L821) | receiver-type-required |
| `execute_context_get` | `event.canonical_bytes` | [821](../../src/production_tool_control.rs#L821) | receiver-type-required |
| `execute_context_get` | `canonical.len` | [822](../../src/production_tool_control.rs#L822), [827](../../src/production_tool_control.rs#L827), [830](../../src/production_tool_control.rs#L830) | receiver-type-required |
| `execute_context_get` | `Err` | [823](../../src/production_tool_control.rs#L823) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `SupervisorOperationError::Limit` | [823](../../src/production_tool_control.rs#L823) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `records.push` | [832](../../src/production_tool_control.rs#L832) | receiver-type-required |
| `execute_context_get` | `(offset + consumed < ordered.len())             .then` | [838](../../src/production_tool_control.rs#L838) | receiver-type-required |
| `execute_context_get` | `ordered.len` | [838](../../src/production_tool_control.rs#L838) | receiver-type-required |
| `execute_context_get` | `encode_cursor` | [839](../../src/production_tool_control.rs#L839) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1350) |
| `execute_context_get` | `to_ijson` | [840](../../src/production_tool_control.rs#L840) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `apply_continuation` | `response.request_id.clone` | [877](../../src/production_tool_control.rs#L877) | receiver-type-required |
| `apply_continuation` | `response.call_id.clone` | [878](../../src/production_tool_control.rs#L878) | receiver-type-required |
| `apply_continuation` | `"tool continuation result withheld by secret policy".into` | [882](../../src/production_tool_control.rs#L882) | receiver-type-required |
| `apply_continuation` | `self.scanner.scan` | [888](../../src/production_tool_control.rs#L888), [892](../../src/production_tool_control.rs#L892), [898](../../src/production_tool_control.rs#L898) | receiver-type-required |
| `apply_continuation` | `withheld` | [890](../../src/production_tool_control.rs#L890), [894](../../src/production_tool_control.rs#L894), [905](../../src/production_tool_control.rs#L905) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `IJsonValue::from` | [897](../../src/production_tool_control.rs#L897) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `error.message.clone` | [897](../../src/production_tool_control.rs#L897) | receiver-type-required |
| `apply_continuation` | `serde_json::from_value(                             serde_json::to_value(message).unwrap_or(Value::Null),                         )                         .unwrap_or_else` | [900](../../src/production_tool_control.rs#L900) | receiver-type-required |
| `apply_continuation` | `serde_json::from_value` | [900](../../src/production_tool_control.rs#L900) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `serde_json::to_value(message).unwrap_or` | [901](../../src/production_tool_control.rs#L901) | receiver-type-required |
| `apply_continuation` | `serde_json::to_value` | [901](../../src/production_tool_control.rs#L901) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `"continuation error was redacted".to_owned` | [903](../../src/production_tool_control.rs#L903) | receiver-type-required |
| `apply` | `result.pending.as_mut` | [915](../../src/production_tool_control.rs#L915) | receiver-type-required |
| `apply` | `self.scanner.scan` | [918](../../src/production_tool_control.rs#L918), [927](../../src/production_tool_control.rs#L927), [937](../../src/production_tool_control.rs#L937) | receiver-type-required |
| `apply` | `policy_withheld` | [923](../../src/production_tool_control.rs#L923), [932](../../src/production_tool_control.rs#L932), [945](../../src/production_tool_control.rs#L945), [948](../../src/production_tool_control.rs#L948) | [tekes-supervisor::production_tool_control::policy_withheld](../../src/production_tool_control.rs#L953) |
| `apply` | `result.value.take` | [926](../../src/production_tool_control.rs#L926) | receiver-type-required |
| `apply` | `Some` | [929](../../src/production_tool_control.rs#L929) | external-constructor-callback-or-unresolved |
| `apply` | `result.error.as_mut` | [935](../../src/production_tool_control.rs#L935) | receiver-type-required |
| `apply` | `IJsonValue::from` | [936](../../src/production_tool_control.rs#L936) | external-constructor-callback-or-unresolved |
| `apply` | `error.message.clone` | [936](../../src/production_tool_control.rs#L936) | receiver-type-required |
| `apply` | `serde_json::from_value(                         serde_json::to_value(message).unwrap_or(Value::Null),                     )                     .unwrap_or_else` | [939](../../src/production_tool_control.rs#L939) | receiver-type-required |
| `apply` | `serde_json::from_value` | [939](../../src/production_tool_control.rs#L939) | external-constructor-callback-or-unresolved |
| `apply` | `serde_json::to_value(message).unwrap_or` | [940](../../src/production_tool_control.rs#L940) | receiver-type-required |
| `apply` | `serde_json::to_value` | [940](../../src/production_tool_control.rs#L940) | external-constructor-callback-or-unresolved |
| `apply` | `"backend error was redacted".to_owned` | [942](../../src/production_tool_control.rs#L942) | receiver-type-required |
| `policy_withheld` | `ToolControlResult::failure` | [954](../../src/production_tool_control.rs#L954) | [worker-control::durable::ToolControlResult::failure](../../../worker-control/src/durable.rs#L375) |
| `policy_withheld` | `"tool-control result withheld by secret policy".into` | [959](../../src/production_tool_control.rs#L959) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::Spawn)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .ok_or_else` | [969](../../src/production_tool_control.rs#L969) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter()         .find` | [969](../../src/production_tool_control.rs#L969) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter` | [969](../../src/production_tool_control.rs#L969) | receiver-type-required |
| `child_launch_proof` | `event.turn` | [975](../../src/production_tool_control.rs#L975) | receiver-type-required |
| `child_launch_proof` | `Some` | [975](../../src/production_tool_control.rs#L975), [976](../../src/production_tool_control.rs#L976), [985](../../src/production_tool_control.rs#L985), [1017](../../src/production_tool_control.rs#L1017), [1018](../../src/production_tool_control.rs#L1018), [1019](../../src/production_tool_control.rs#L1019) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `event.string_field` | [976](../../src/production_tool_control.rs#L976), [985](../../src/production_tool_control.rs#L985) | receiver-type-required |
| `child_launch_proof` | `request.call_id.as_str` | [976](../../src/production_tool_control.rs#L976), [985](../../src/production_tool_control.rs#L985) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Conflict` | [979](../../src/production_tool_control.rs#L979), [987](../../src/production_tool_control.rs#L987), [1006](../../src/production_tool_control.rs#L1006), [1021](../../src/production_tool_control.rs#L1021) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"task/subagent has no durable parent spawn proof".into` | [980](../../src/production_tool_control.rs#L980) | receiver-type-required |
| `child_launch_proof` | `resolved.projection.events.iter().any` | [983](../../src/production_tool_control.rs#L983) | receiver-type-required |
| `child_launch_proof` | `resolved.projection.events.iter` | [983](../../src/production_tool_control.rs#L983) | receiver-type-required |
| `child_launch_proof` | `Err` | [987](../../src/production_tool_control.rs#L987), [1021](../../src/production_tool_control.rs#L1021) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"child_result already terminalized this spawn".into` | [988](../../src/production_tool_control.rs#L988) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field("child")         .ok_or_else` | [991](../../src/production_tool_control.rs#L991) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field` | [991](../../src/production_tool_control.rs#L991), [1014](../../src/production_tool_control.rs#L1014) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Corruption` | [993](../../src/production_tool_control.rs#L993), [1000](../../src/production_tool_control.rs#L1000), [1012](../../src/production_tool_control.rs#L1012), [1016](../../src/production_tool_control.rs#L1016) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"spawn.child missing".into` | [993](../../src/production_tool_control.rs#L993) | receiver-type-required |
| `child_launch_proof` | `validate_line_file_name` | [994](../../src/production_tool_control.rs#L994) | [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1432) |
| `child_launch_proof` | `resolved.thread_folder.join` | [995](../../src/production_tool_control.rs#L995) | receiver-type-required |
| `child_launch_proof` | `read_projection` | [996](../../src/production_tool_control.rs#L996) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) |
| `child_launch_proof` | `child         .events         .first()         .ok_or_else` | [997](../../src/production_tool_control.rs#L997) | receiver-type-required |
| `child_launch_proof` | `child         .events         .first` | [997](../../src/production_tool_control.rs#L997) | receiver-type-required |
| `child_launch_proof` | `"child genesis missing".into` | [1000](../../src/production_tool_control.rs#L1000) | receiver-type-required |
| `child_launch_proof` | `serde_json::to_value(genesis.raw())         .map_err` | [1001](../../src/production_tool_control.rs#L1001) | receiver-type-required |
| `child_launch_proof` | `serde_json::to_value` | [1001](../../src/production_tool_control.rs#L1001) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `genesis.raw` | [1001](../../src/production_tool_control.rs#L1001) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Protocol` | [1002](../../src/production_tool_control.rs#L1002) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `error.to_string` | [1002](../../src/production_tool_control.rs#L1002) | receiver-type-required |
| `child_launch_proof` | `value         .get("parent")         .and_then(Value::as_object)         .ok_or_else` | [1003](../../src/production_tool_control.rs#L1003) | receiver-type-required |
| `child_launch_proof` | `value         .get("parent")         .and_then` | [1003](../../src/production_tool_control.rs#L1003) | receiver-type-required |
| `child_launch_proof` | `value         .get` | [1003](../../src/production_tool_control.rs#L1003) | receiver-type-required |
| `child_launch_proof` | `"child parent binding missing".into` | [1006](../../src/production_tool_control.rs#L1006) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name()         .and_then(&#124;value&#124; value.to_str())         .ok_or_else` | [1007](../../src/production_tool_control.rs#L1007) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name()         .and_then` | [1007](../../src/production_tool_control.rs#L1007) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name` | [1007](../../src/production_tool_control.rs#L1007) | receiver-type-required |
| `child_launch_proof` | `value.to_str` | [1010](../../src/production_tool_control.rs#L1010) | receiver-type-required |
| `child_launch_proof` | `"parent file name is not UTF-8".into` | [1012](../../src/production_tool_control.rs#L1012) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field("spawn_id")         .ok_or_else` | [1014](../../src/production_tool_control.rs#L1014) | receiver-type-required |
| `child_launch_proof` | `"spawn_id missing".into` | [1016](../../src/production_tool_control.rs#L1016) | receiver-type-required |
| `child_launch_proof` | `parent.get("file").and_then` | [1017](../../src/production_tool_control.rs#L1017) | receiver-type-required |
| `child_launch_proof` | `parent.get` | [1017](../../src/production_tool_control.rs#L1017), [1018](../../src/production_tool_control.rs#L1018), [1019](../../src/production_tool_control.rs#L1019) | receiver-type-required |
| `child_launch_proof` | `parent.get("seq").and_then` | [1018](../../src/production_tool_control.rs#L1018) | receiver-type-required |
| `child_launch_proof` | `spawn.seq` | [1018](../../src/production_tool_control.rs#L1018), [1029](../../src/production_tool_control.rs#L1029) | receiver-type-required |
| `child_launch_proof` | `parent.get("spawn_id").and_then` | [1019](../../src/production_tool_control.rs#L1019) | receiver-type-required |
| `child_launch_proof` | `"child genesis does not match the durable parent spawn".into` | [1022](../../src/production_tool_control.rs#L1022) | receiver-type-required |
| `child_launch_proof` | `Ok` | [1025](../../src/production_tool_control.rs#L1025) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `request.session.clone` | [1026](../../src/production_tool_control.rs#L1026) | receiver-type-required |
| `child_launch_proof` | `request.thread.clone` | [1027](../../src/production_tool_control.rs#L1027) | receiver-type-required |
| `child_launch_proof` | `parent_file.to_owned` | [1028](../../src/production_tool_control.rs#L1028) | receiver-type-required |
| `child_launch_proof` | `genesis_line(&child)?.to_owned` | [1030](../../src/production_tool_control.rs#L1030) | receiver-type-required |
| `child_launch_proof` | `genesis_line` | [1030](../../src/production_tool_control.rs#L1030) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1252) |
| `child_launch_proof` | `child_file.to_owned` | [1031](../../src/production_tool_control.rs#L1031) | receiver-type-required |
| `child_launch_proof` | `spawn_id.to_owned` | [1032](../../src/production_tool_control.rs#L1032) | receiver-type-required |
| `child_launch_proof` | `request.call_id.clone` | [1033](../../src/production_tool_control.rs#L1033) | receiver-type-required |
| `parent_report_proof` | `resolved.projection.events.first().ok_or_else` | [1041](../../src/production_tool_control.rs#L1041) | receiver-type-required |
| `parent_report_proof` | `resolved.projection.events.first` | [1041](../../src/production_tool_control.rs#L1041) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Corruption` | [1042](../../src/production_tool_control.rs#L1042), [1055](../../src/production_tool_control.rs#L1055), [1060](../../src/production_tool_control.rs#L1060), [1064](../../src/production_tool_control.rs#L1064), [1069](../../src/production_tool_control.rs#L1069), [1078](../../src/production_tool_control.rs#L1078) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"reporting line has no genesis".into` | [1042](../../src/production_tool_control.rs#L1042) | receiver-type-required |
| `parent_report_proof` | `serde_json::to_value(child_genesis.raw())         .map_err` | [1044](../../src/production_tool_control.rs#L1044) | receiver-type-required |
| `parent_report_proof` | `serde_json::to_value` | [1044](../../src/production_tool_control.rs#L1044) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `child_genesis.raw` | [1044](../../src/production_tool_control.rs#L1044) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Protocol` | [1045](../../src/production_tool_control.rs#L1045) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `error.to_string` | [1045](../../src/production_tool_control.rs#L1045) | receiver-type-required |
| `parent_report_proof` | `value         .get("parent")         .and_then(Value::as_object)         .ok_or_else` | [1046](../../src/production_tool_control.rs#L1046) | receiver-type-required |
| `parent_report_proof` | `value         .get("parent")         .and_then` | [1046](../../src/production_tool_control.rs#L1046) | receiver-type-required |
| `parent_report_proof` | `value         .get` | [1046](../../src/production_tool_control.rs#L1046) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Denied` | [1050](../../src/production_tool_control.rs#L1050) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"report is valid only in a child line".into` | [1050](../../src/production_tool_control.rs#L1050) | receiver-type-required |
| `parent_report_proof` | `parent         .get("file")         .and_then(Value::as_str)         .ok_or_else` | [1052](../../src/production_tool_control.rs#L1052) | receiver-type-required |
| `parent_report_proof` | `parent         .get("file")         .and_then` | [1052](../../src/production_tool_control.rs#L1052) | receiver-type-required |
| `parent_report_proof` | `parent         .get` | [1052](../../src/production_tool_control.rs#L1052), [1057](../../src/production_tool_control.rs#L1057), [1061](../../src/production_tool_control.rs#L1061) | receiver-type-required |
| `parent_report_proof` | `"parent.file missing".into` | [1055](../../src/production_tool_control.rs#L1055) | receiver-type-required |
| `parent_report_proof` | `validate_line_file_name` | [1056](../../src/production_tool_control.rs#L1056) | [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1432) |
| `parent_report_proof` | `parent         .get("seq")         .and_then(Value::as_u64)         .ok_or_else` | [1057](../../src/production_tool_control.rs#L1057) | receiver-type-required |
| `parent_report_proof` | `parent         .get("seq")         .and_then` | [1057](../../src/production_tool_control.rs#L1057) | receiver-type-required |
| `parent_report_proof` | `"parent.seq missing".into` | [1060](../../src/production_tool_control.rs#L1060) | receiver-type-required |
| `parent_report_proof` | `parent         .get("spawn_id")         .and_then(Value::as_str)         .ok_or_else` | [1061](../../src/production_tool_control.rs#L1061) | receiver-type-required |
| `parent_report_proof` | `parent         .get("spawn_id")         .and_then` | [1061](../../src/production_tool_control.rs#L1061) | receiver-type-required |
| `parent_report_proof` | `"parent.spawn_id missing".into` | [1064](../../src/production_tool_control.rs#L1064) | receiver-type-required |
| `parent_report_proof` | `read_projection` | [1065](../../src/production_tool_control.rs#L1065) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) |
| `parent_report_proof` | `resolved.thread_folder.join` | [1065](../../src/production_tool_control.rs#L1065) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get(usize::try_from(parent_seq - 1).map_err(&#124;_&#124; {             SupervisorOperationError::Corruption("parent seq does not fit this platform".into())         })?)         .filter(&#124;event&#124; event.seq() == parent_seq && matches!(event.kind(), EventKind::Spawn))         .ok_or_else` | [1066](../../src/production_tool_control.rs#L1066) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get(usize::try_from(parent_seq - 1).map_err(&#124;_&#124; {             SupervisorOperationError::Corruption("parent seq does not fit this platform".into())         })?)         .filter` | [1066](../../src/production_tool_control.rs#L1066) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get` | [1066](../../src/production_tool_control.rs#L1066) | receiver-type-required |
| `parent_report_proof` | `usize::try_from(parent_seq - 1).map_err` | [1068](../../src/production_tool_control.rs#L1068) | receiver-type-required |
| `parent_report_proof` | `usize::try_from` | [1068](../../src/production_tool_control.rs#L1068) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"parent seq does not fit this platform".into` | [1069](../../src/production_tool_control.rs#L1069) | receiver-type-required |
| `parent_report_proof` | `event.seq` | [1071](../../src/production_tool_control.rs#L1071) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Conflict` | [1072](../../src/production_tool_control.rs#L1072), [1083](../../src/production_tool_control.rs#L1083) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"parent spawn proof missing".into` | [1072](../../src/production_tool_control.rs#L1072) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name()         .and_then(&#124;value&#124; value.to_str())         .ok_or_else` | [1073](../../src/production_tool_control.rs#L1073) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name()         .and_then` | [1073](../../src/production_tool_control.rs#L1073) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name` | [1073](../../src/production_tool_control.rs#L1073) | receiver-type-required |
| `parent_report_proof` | `value.to_str` | [1076](../../src/production_tool_control.rs#L1076) | receiver-type-required |
| `parent_report_proof` | `"child file name is not UTF-8".into` | [1078](../../src/production_tool_control.rs#L1078) | receiver-type-required |
| `parent_report_proof` | `spawn.string_field` | [1080](../../src/production_tool_control.rs#L1080), [1081](../../src/production_tool_control.rs#L1081) | receiver-type-required |
| `parent_report_proof` | `Some` | [1080](../../src/production_tool_control.rs#L1080), [1081](../../src/production_tool_control.rs#L1081) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `Err` | [1083](../../src/production_tool_control.rs#L1083) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"reporting child does not match parent spawn".into` | [1084](../../src/production_tool_control.rs#L1084) | receiver-type-required |
| `parent_report_proof` | `Ok` | [1087](../../src/production_tool_control.rs#L1087) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `request.session.clone` | [1088](../../src/production_tool_control.rs#L1088) | receiver-type-required |
| `parent_report_proof` | `request.thread.clone` | [1089](../../src/production_tool_control.rs#L1089) | receiver-type-required |
| `parent_report_proof` | `child_file.to_owned` | [1090](../../src/production_tool_control.rs#L1090) | receiver-type-required |
| `parent_report_proof` | `genesis_line(&parent_projection)?.to_owned` | [1091](../../src/production_tool_control.rs#L1091) | receiver-type-required |
| `parent_report_proof` | `genesis_line` | [1091](../../src/production_tool_control.rs#L1091) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1252) |
| `parent_report_proof` | `parent_file.to_owned` | [1092](../../src/production_tool_control.rs#L1092) | receiver-type-required |
| `parent_report_proof` | `spawn_id.to_owned` | [1094](../../src/production_tool_control.rs#L1094) | receiver-type-required |
| `parent_report_proof` | `request.call_id.clone` | [1095](../../src/production_tool_control.rs#L1095) | receiver-type-required |
| `session_lines` | `Vec::new` | [1102](../../src/production_tool_control.rs#L1102) | external-constructor-callback-or-unresolved |
| `session_lines` | `fs::read_dir(folder).map_err` | [1103](../../src/production_tool_control.rs#L1103) | receiver-type-required |
| `session_lines` | `fs::read_dir` | [1103](../../src/production_tool_control.rs#L1103) | external-constructor-callback-or-unresolved |
| `session_lines` | `entry.map_err` | [1104](../../src/production_tool_control.rs#L1104) | receiver-type-required |
| `session_lines` | `entry.file_type().map_err(map_io)?.is_file` | [1105](../../src/production_tool_control.rs#L1105) | receiver-type-required |
| `session_lines` | `entry.file_type().map_err` | [1105](../../src/production_tool_control.rs#L1105) | receiver-type-required |
| `session_lines` | `entry.file_type` | [1105](../../src/production_tool_control.rs#L1105) | receiver-type-required |
| `session_lines` | `entry.path` | [1108](../../src/production_tool_control.rs#L1108) | receiver-type-required |
| `session_lines` | `path.extension().and_then` | [1109](../../src/production_tool_control.rs#L1109) | receiver-type-required |
| `session_lines` | `path.extension` | [1109](../../src/production_tool_control.rs#L1109) | receiver-type-required |
| `session_lines` | `value.to_str` | [1109](../../src/production_tool_control.rs#L1109), [1110](../../src/production_tool_control.rs#L1110) | receiver-type-required |
| `session_lines` | `Some` | [1109](../../src/production_tool_control.rs#L1109), [1110](../../src/production_tool_control.rs#L1110) | external-constructor-callback-or-unresolved |
| `session_lines` | `path.file_name().and_then` | [1110](../../src/production_tool_control.rs#L1110) | receiver-type-required |
| `session_lines` | `path.file_name` | [1110](../../src/production_tool_control.rs#L1110) | receiver-type-required |
| `session_lines` | `read_projection` | [1111](../../src/production_tool_control.rs#L1111) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1214) |
| `session_lines` | `lines.push` | [1113](../../src/production_tool_control.rs#L1113) | receiver-type-required |
| `session_lines` | `lines.sort_by` | [1116](../../src/production_tool_control.rs#L1116) | receiver-type-required |
| `session_lines` | `left.0.as_os_str().cmp` | [1116](../../src/production_tool_control.rs#L1116) | receiver-type-required |
| `session_lines` | `left.0.as_os_str` | [1116](../../src/production_tool_control.rs#L1116) | receiver-type-required |
| `session_lines` | `right.0.as_os_str` | [1116](../../src/production_tool_control.rs#L1116) | receiver-type-required |
| `session_lines` | `Ok` | [1117](../../src/production_tool_control.rs#L1117) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `visible_sessions` | [1125](../../src/production_tool_control.rs#L1125) | [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1141) |
| `resolve_visible_line` | `session_folder` | [1126](../../src/production_tool_control.rs#L1126) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) |
| `resolve_visible_line` | `session_lines` | [1127](../../src/production_tool_control.rs#L1127) | [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1099) |
| `resolve_visible_line` | `genesis_line` | [1128](../../src/production_tool_control.rs#L1128) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1252) |
| `resolve_visible_line` | `found.is_some` | [1129](../../src/production_tool_control.rs#L1129) | receiver-type-required |
| `resolve_visible_line` | `Err` | [1130](../../src/production_tool_control.rs#L1130) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `SupervisorOperationError::Corruption` | [1130](../../src/production_tool_control.rs#L1130) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `"line identity is duplicated across sessions".into` | [1131](../../src/production_tool_control.rs#L1131) | receiver-type-required |
| `resolve_visible_line` | `Some` | [1134](../../src/production_tool_control.rs#L1134) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `found.ok_or_else` | [1138](../../src/production_tool_control.rs#L1138) | receiver-type-required |
| `resolve_visible_line` | `SupervisorOperationError::NotFound` | [1138](../../src/production_tool_control.rs#L1138) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `"line not found".into` | [1138](../../src/production_tool_control.rs#L1138) | receiver-type-required |
| `visible_sessions` | `BTreeSet::new` | [1145](../../src/production_tool_control.rs#L1145) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `fs::read_dir(root.join(area)).map_err` | [1147](../../src/production_tool_control.rs#L1147) | receiver-type-required |
| `visible_sessions` | `fs::read_dir` | [1147](../../src/production_tool_control.rs#L1147) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `root.join` | [1147](../../src/production_tool_control.rs#L1147) | receiver-type-required |
| `visible_sessions` | `entry.map_err` | [1148](../../src/production_tool_control.rs#L1148) | receiver-type-required |
| `visible_sessions` | `entry.file_type().map_err(map_io)?.is_dir` | [1149](../../src/production_tool_control.rs#L1149) | receiver-type-required |
| `visible_sessions` | `entry.file_type().map_err` | [1149](../../src/production_tool_control.rs#L1149) | receiver-type-required |
| `visible_sessions` | `entry.file_type` | [1149](../../src/production_tool_control.rs#L1149) | receiver-type-required |
| `visible_sessions` | `entry.file_name().to_string_lossy().into_owned` | [1152](../../src/production_tool_control.rs#L1152) | receiver-type-required |
| `visible_sessions` | `entry.file_name().to_string_lossy` | [1152](../../src/production_tool_control.rs#L1152) | receiver-type-required |
| `visible_sessions` | `entry.file_name` | [1152](../../src/production_tool_control.rs#L1152) | receiver-type-required |
| `visible_sessions` | `validate_session_folder` | [1153](../../src/production_tool_control.rs#L1153) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1179) |
| `visible_sessions` | `entry.path` | [1153](../../src/production_tool_control.rs#L1153), [1155](../../src/production_tool_control.rs#L1155) | receiver-type-required |
| `visible_sessions` | `folder_workspace` | [1155](../../src/production_tool_control.rs#L1155) | [tekes-supervisor::production_tool_control::folder_workspace](../../src/production_tool_control.rs#L1227) |
| `visible_sessions` | `sessions.insert` | [1159](../../src/production_tool_control.rs#L1159) | receiver-type-required |
| `visible_sessions` | `Err` | [1160](../../src/production_tool_control.rs#L1160) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `SupervisorOperationError::Corruption` | [1160](../../src/production_tool_control.rs#L1160) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `"session exists in active and archive catalogs".into` | [1161](../../src/production_tool_control.rs#L1161) | receiver-type-required |
| `visible_sessions` | `Ok` | [1166](../../src/production_tool_control.rs#L1166) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `sessions.into_iter().collect` | [1166](../../src/production_tool_control.rs#L1166) | receiver-type-required |
| `visible_sessions` | `sessions.into_iter` | [1166](../../src/production_tool_control.rs#L1166) | receiver-type-required |
| `require_active_session` | `session_folder` | [1170](../../src/production_tool_control.rs#L1170) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1199) |
| `require_active_session` | `Err` | [1172](../../src/production_tool_control.rs#L1172) | external-constructor-callback-or-unresolved |
| `require_active_session` | `SupervisorOperationError::NotFound` | [1172](../../src/production_tool_control.rs#L1172) | external-constructor-callback-or-unresolved |
| `require_active_session` | `"active thread not found".into` | [1173](../../src/production_tool_control.rs#L1173) | receiver-type-required |
| `require_active_session` | `Ok` | [1176](../../src/production_tool_control.rs#L1176) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `endpoint::validate_session_id(session)         .map_err` | [1180](../../src/production_tool_control.rs#L1180) | receiver-type-required |
| `validate_session_folder` | `endpoint::validate_session_id` | [1180](../../src/production_tool_control.rs#L1180) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `validate_session_folder` | `SupervisorOperationError::Invalid` | [1181](../../src/production_tool_control.rs#L1181) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"invalid session identity".into` | [1181](../../src/production_tool_control.rs#L1181) | receiver-type-required |
| `validate_session_folder` | `fs::symlink_metadata(folder).map_err` | [1182](../../src/production_tool_control.rs#L1182) | receiver-type-required |
| `validate_session_folder` | `fs::symlink_metadata` | [1182](../../src/production_tool_control.rs#L1182) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `metadata.is_dir` | [1183](../../src/production_tool_control.rs#L1183) | receiver-type-required |
| `validate_session_folder` | `metadata.file_type().is_symlink` | [1183](../../src/production_tool_control.rs#L1183) | receiver-type-required |
| `validate_session_folder` | `metadata.file_type` | [1183](../../src/production_tool_control.rs#L1183) | receiver-type-required |
| `validate_session_folder` | `Err` | [1184](../../src/production_tool_control.rs#L1184), [1192](../../src/production_tool_control.rs#L1192) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `SupervisorOperationError::NotFound` | [1184](../../src/production_tool_control.rs#L1184) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"thread not found".into` | [1185](../../src/production_tool_control.rs#L1185) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis(folder.join("main.jsonl"))         .map_err(map_store)?         .ok_or_else` | [1188](../../src/production_tool_control.rs#L1188) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis(folder.join("main.jsonl"))         .map_err` | [1188](../../src/production_tool_control.rs#L1188) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis` | [1188](../../src/production_tool_control.rs#L1188) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `validate_session_folder` | `folder.join` | [1188](../../src/production_tool_control.rs#L1188) | receiver-type-required |
| `validate_session_folder` | `SupervisorOperationError::Corruption` | [1190](../../src/production_tool_control.rs#L1190), [1192](../../src/production_tool_control.rs#L1192) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"genesis missing".into` | [1190](../../src/production_tool_control.rs#L1190) | receiver-type-required |
| `validate_session_folder` | `genesis.string_field` | [1191](../../src/production_tool_control.rs#L1191) | receiver-type-required |
| `validate_session_folder` | `Some` | [1191](../../src/production_tool_control.rs#L1191) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"folder/genesis identity mismatch".into` | [1193](../../src/production_tool_control.rs#L1193) | receiver-type-required |
| `validate_session_folder` | `Ok` | [1196](../../src/production_tool_control.rs#L1196) | external-constructor-callback-or-unresolved |
| `session_folder` | `endpoint::validate_session_id(session)         .map_err` | [1200](../../src/production_tool_control.rs#L1200) | receiver-type-required |
| `session_folder` | `endpoint::validate_session_id` | [1200](../../src/production_tool_control.rs#L1200) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `session_folder` | `SupervisorOperationError::Invalid` | [1201](../../src/production_tool_control.rs#L1201) | external-constructor-callback-or-unresolved |
| `session_folder` | `"invalid session identity".into` | [1201](../../src/production_tool_control.rs#L1201) | receiver-type-required |
| `session_folder` | `root.join(area).join` | [1203](../../src/production_tool_control.rs#L1203) | receiver-type-required |
| `session_folder` | `root.join` | [1203](../../src/production_tool_control.rs#L1203) | receiver-type-required |
| `session_folder` | `fs::symlink_metadata(&folder).is_ok` | [1204](../../src/production_tool_control.rs#L1204) | receiver-type-required |
| `session_folder` | `fs::symlink_metadata` | [1204](../../src/production_tool_control.rs#L1204) | external-constructor-callback-or-unresolved |
| `session_folder` | `validate_session_folder` | [1205](../../src/production_tool_control.rs#L1205) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1179) |
| `session_folder` | `Ok` | [1206](../../src/production_tool_control.rs#L1206) | external-constructor-callback-or-unresolved |
| `session_folder` | `Err` | [1209](../../src/production_tool_control.rs#L1209) | external-constructor-callback-or-unresolved |
| `session_folder` | `SupervisorOperationError::NotFound` | [1209](../../src/production_tool_control.rs#L1209) | external-constructor-callback-or-unresolved |
| `session_folder` | `"thread not found".into` | [1210](../../src/production_tool_control.rs#L1210) | receiver-type-required |
| `read_projection` | `fs::read(path).map_err` | [1215](../../src/production_tool_control.rs#L1215) | receiver-type-required |
| `read_projection` | `fs::read` | [1215](../../src/production_tool_control.rs#L1215) | external-constructor-callback-or-unresolved |
| `read_projection` | `scan_valid_prefix` | [1216](../../src/production_tool_control.rs#L1216) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `read_projection` | `scan.projection.ok_or_else` | [1217](../../src/production_tool_control.rs#L1217) | receiver-type-required |
| `read_projection` | `SupervisorOperationError::Corruption` | [1218](../../src/production_tool_control.rs#L1218) | external-constructor-callback-or-unresolved |
| `folder_workspace` | `store::read_genesis(folder.join("main.jsonl"))         .map_err(map_store)?         .ok_or_else` | [1228](../../src/production_tool_control.rs#L1228) | receiver-type-required |
| `folder_workspace` | `store::read_genesis(folder.join("main.jsonl"))         .map_err` | [1228](../../src/production_tool_control.rs#L1228) | receiver-type-required |
| `folder_workspace` | `store::read_genesis` | [1228](../../src/production_tool_control.rs#L1228) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `folder_workspace` | `folder.join` | [1228](../../src/production_tool_control.rs#L1228) | receiver-type-required |
| `folder_workspace` | `SupervisorOperationError::Corruption` | [1231](../../src/production_tool_control.rs#L1231), [1239](../../src/production_tool_control.rs#L1239) | external-constructor-callback-or-unresolved |
| `folder_workspace` | `genesis         .string_field("workspace")         .map(str::to_owned)         .ok_or_else` | [1236](../../src/production_tool_control.rs#L1236) | receiver-type-required |
| `folder_workspace` | `genesis         .string_field("workspace")         .map` | [1236](../../src/production_tool_control.rs#L1236) | receiver-type-required |
| `folder_workspace` | `genesis         .string_field` | [1236](../../src/production_tool_control.rs#L1236) | receiver-type-required |
| `folder_workspace` | `"genesis workspace missing".into` | [1239](../../src/production_tool_control.rs#L1239) | receiver-type-required |
| `genesis_workspace` | `projection         .events         .first()         .and_then(&#124;event&#124; event.string_field("workspace"))         .ok_or_else` | [1245](../../src/production_tool_control.rs#L1245) | receiver-type-required |
| `genesis_workspace` | `projection         .events         .first()         .and_then` | [1245](../../src/production_tool_control.rs#L1245) | receiver-type-required |
| `genesis_workspace` | `projection         .events         .first` | [1245](../../src/production_tool_control.rs#L1245) | receiver-type-required |
| `genesis_workspace` | `event.string_field` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `genesis_workspace` | `SupervisorOperationError::Corruption` | [1249](../../src/production_tool_control.rs#L1249) | external-constructor-callback-or-unresolved |
| `genesis_workspace` | `"genesis workspace missing".into` | [1249](../../src/production_tool_control.rs#L1249) | receiver-type-required |
| `genesis_line` | `projection         .events         .first()         .and_then(&#124;event&#124; event.string_field("thread"))         .ok_or_else` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `genesis_line` | `projection         .events         .first()         .and_then` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `genesis_line` | `projection         .events         .first` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `genesis_line` | `event.string_field` | [1256](../../src/production_tool_control.rs#L1256) | receiver-type-required |
| `genesis_line` | `SupervisorOperationError::Corruption` | [1257](../../src/production_tool_control.rs#L1257) | external-constructor-callback-or-unresolved |
| `genesis_line` | `"genesis thread missing".into` | [1257](../../src/production_tool_control.rs#L1257) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::ToolCall)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .and_then(&#124;event&#124; event.string_field("ts"))         .ok_or_else` | [1264](../../src/production_tool_control.rs#L1264) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::ToolCall)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .and_then` | [1264](../../src/production_tool_control.rs#L1264) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find` | [1264](../../src/production_tool_control.rs#L1264) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter` | [1264](../../src/production_tool_control.rs#L1264) | receiver-type-required |
| `paired_tool_timestamp` | `event.turn` | [1269](../../src/production_tool_control.rs#L1269) | receiver-type-required |
| `paired_tool_timestamp` | `Some` | [1269](../../src/production_tool_control.rs#L1269), [1270](../../src/production_tool_control.rs#L1270) | external-constructor-callback-or-unresolved |
| `paired_tool_timestamp` | `event.string_field` | [1270](../../src/production_tool_control.rs#L1270), [1272](../../src/production_tool_control.rs#L1272) | receiver-type-required |
| `paired_tool_timestamp` | `request.call_id.as_str` | [1270](../../src/production_tool_control.rs#L1270) | receiver-type-required |
| `paired_tool_timestamp` | `SupervisorOperationError::Corruption` | [1273](../../src/production_tool_control.rs#L1273) | external-constructor-callback-or-unresolved |
| `paired_tool_timestamp` | `"tool_call timestamp missing".into` | [1273](../../src/production_tool_control.rs#L1273) | receiver-type-required |
| `event_record` | `event.canonical_bytes().map_err` | [1277](../../src/production_tool_control.rs#L1277) | receiver-type-required |
| `event_record` | `event.canonical_bytes` | [1277](../../src/production_tool_control.rs#L1277) | receiver-type-required |
| `event_record` | `serde_json::from_slice(&canonical)         .map_err` | [1278](../../src/production_tool_control.rs#L1278) | receiver-type-required |
| `event_record` | `serde_json::from_slice` | [1278](../../src/production_tool_control.rs#L1278) | external-constructor-callback-or-unresolved |
| `event_record` | `SupervisorOperationError::Protocol` | [1279](../../src/production_tool_control.rs#L1279) | external-constructor-callback-or-unresolved |
| `event_record` | `error.to_string` | [1279](../../src/production_tool_control.rs#L1279) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [1283](../../src/production_tool_control.rs#L1283) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then(Value::as_str)         .filter` | [1283](../../src/production_tool_control.rs#L1283) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then` | [1283](../../src/production_tool_control.rs#L1283) | receiver-type-required |
| `required_string` | `value         .get` | [1283](../../src/production_tool_control.rs#L1283) | receiver-type-required |
| `required_string` | `value.is_empty` | [1286](../../src/production_tool_control.rs#L1286) | receiver-type-required |
| `required_string` | `SupervisorOperationError::Invalid` | [1287](../../src/production_tool_control.rs#L1287) | external-constructor-callback-or-unresolved |
| `string_array` | `value.get(field).map_or` | [1291](../../src/production_tool_control.rs#L1291) | receiver-type-required |
| `string_array` | `value.get` | [1291](../../src/production_tool_control.rs#L1291) | receiver-type-required |
| `string_array` | `Ok` | [1291](../../src/production_tool_control.rs#L1291) | external-constructor-callback-or-unresolved |
| `string_array` | `Vec::new` | [1291](../../src/production_tool_control.rs#L1291) | external-constructor-callback-or-unresolved |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter()             .map(&#124;value&#124; {                 value.as_str().map(ToOwned::to_owned).ok_or_else(&#124;&#124; {                     SupervisorOperationError::Invalid(format!("{field} contains a non-string"))                 })             })             .collect` | [1292](../../src/production_tool_control.rs#L1292) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter()             .map` | [1292](../../src/production_tool_control.rs#L1292) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter` | [1292](../../src/production_tool_control.rs#L1292) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else` | [1292](../../src/production_tool_control.rs#L1292) | receiver-type-required |
| `string_array` | `value             .as_array` | [1292](../../src/production_tool_control.rs#L1292) | receiver-type-required |
| `string_array` | `SupervisorOperationError::Invalid` | [1294](../../src/production_tool_control.rs#L1294), [1298](../../src/production_tool_control.rs#L1298) | external-constructor-callback-or-unresolved |
| `string_array` | `value.as_str().map(ToOwned::to_owned).ok_or_else` | [1297](../../src/production_tool_control.rs#L1297) | receiver-type-required |
| `string_array` | `value.as_str().map` | [1297](../../src/production_tool_control.rs#L1297) | receiver-type-required |
| `string_array` | `value.as_str` | [1297](../../src/production_tool_control.rs#L1297) | receiver-type-required |
| `job_record_outcome` | `to_ijson` | [1309](../../src/production_tool_control.rs#L1309) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `job_record_outcome` | `Err` | [1310](../../src/production_tool_control.rs#L1310), [1311](../../src/production_tool_control.rs#L1311), [1313](../../src/production_tool_control.rs#L1313) | external-constructor-callback-or-unresolved |
| `job_record_outcome` | `map_backend` | [1310](../../src/production_tool_control.rs#L1310) | [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1318) |
| `job_record_outcome` | `SupervisorOperationError::Denied` | [1311](../../src/production_tool_control.rs#L1311) | external-constructor-callback-or-unresolved |
| `job_record_outcome` | `SupervisorOperationError::Unavailable` | [1313](../../src/production_tool_control.rs#L1313) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Invalid` | [1320](../../src/production_tool_control.rs#L1320) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Denied` | [1321](../../src/production_tool_control.rs#L1321) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Unavailable` | [1322](../../src/production_tool_control.rs#L1322), [1327](../../src/production_tool_control.rs#L1327), [1329](../../src/production_tool_control.rs#L1329), [1331](../../src/production_tool_control.rs#L1331), [1333](../../src/production_tool_control.rs#L1333) | external-constructor-callback-or-unresolved |
| `map_backend` | `"job cancelled".into` | [1322](../../src/production_tool_control.rs#L1322) | receiver-type-required |
| `map_backend` | `SupervisorOperationError::Timeout` | [1323](../../src/production_tool_control.rs#L1323) | external-constructor-callback-or-unresolved |
| `map_backend` | `"job timed out".into` | [1323](../../src/production_tool_control.rs#L1323) | receiver-type-required |
| `map_backend` | `SupervisorOperationError::Limit` | [1324](../../src/production_tool_control.rs#L1324) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::NotFound` | [1325](../../src/production_tool_control.rs#L1325) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Conflict` | [1326](../../src/production_tool_control.rs#L1326) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Protocol` | [1332](../../src/production_tool_control.rs#L1332) | external-constructor-callback-or-unresolved |
| `argument_limit` | `value         .get("limit")         .and_then(Value::as_u64)         .map_or` | [1338](../../src/production_tool_control.rs#L1338) | receiver-type-required |
| `argument_limit` | `value         .get("limit")         .and_then` | [1338](../../src/production_tool_control.rs#L1338) | receiver-type-required |
| `argument_limit` | `value         .get` | [1338](../../src/production_tool_control.rs#L1338) | receiver-type-required |
| `argument_limit` | `Err` | [1343](../../src/production_tool_control.rs#L1343) | external-constructor-callback-or-unresolved |
| `argument_limit` | `SupervisorOperationError::Limit` | [1343](../../src/production_tool_control.rs#L1343) | external-constructor-callback-or-unresolved |
| `argument_limit` | `Ok` | [1347](../../src/production_tool_control.rs#L1347) | external-constructor-callback-or-unresolved |
| `encode_cursor` | `Sha256::digest` | [1351](../../src/production_tool_control.rs#L1351) | external-constructor-callback-or-unresolved |
| `encode_cursor` | `identity.as_bytes` | [1351](../../src/production_tool_control.rs#L1351) | receiver-type-required |
| `decode_cursor` | `value         .and_then(Value::as_str)         .filter` | [1356](../../src/production_tool_control.rs#L1356) | receiver-type-required |
| `decode_cursor` | `value         .and_then` | [1356](../../src/production_tool_control.rs#L1356) | receiver-type-required |
| `decode_cursor` | `value.is_empty` | [1358](../../src/production_tool_control.rs#L1358) | receiver-type-required |
| `decode_cursor` | `Ok` | [1360](../../src/production_tool_control.rs#L1360) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `encode_cursor` | [1362](../../src/production_tool_control.rs#L1362) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1350) |
| `decode_cursor` | `cursor_zero         .rsplit_once('-')         .map(&#124;(prefix, _)&#124; prefix)         .expect` | [1363](../../src/production_tool_control.rs#L1363) | receiver-type-required |
| `decode_cursor` | `cursor_zero         .rsplit_once('-')         .map` | [1363](../../src/production_tool_control.rs#L1363) | receiver-type-required |
| `decode_cursor` | `cursor_zero         .rsplit_once` | [1363](../../src/production_tool_control.rs#L1363) | receiver-type-required |
| `decode_cursor` | `cursor         .rsplit_once('-')         .ok_or_else` | [1367](../../src/production_tool_control.rs#L1367) | receiver-type-required |
| `decode_cursor` | `cursor         .rsplit_once` | [1367](../../src/production_tool_control.rs#L1367) | receiver-type-required |
| `decode_cursor` | `SupervisorOperationError::Invalid` | [1369](../../src/production_tool_control.rs#L1369), [1377](../../src/production_tool_control.rs#L1377) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `"invalid cursor".into` | [1369](../../src/production_tool_control.rs#L1369) | receiver-type-required |
| `decode_cursor` | `Err` | [1371](../../src/production_tool_control.rs#L1371) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `SupervisorOperationError::Conflict` | [1371](../../src/production_tool_control.rs#L1371) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `"cursor does not belong to this query".into` | [1372](../../src/production_tool_control.rs#L1372) | receiver-type-required |
| `decode_cursor` | `offset         .parse()         .map_err` | [1375](../../src/production_tool_control.rs#L1375) | receiver-type-required |
| `decode_cursor` | `offset         .parse` | [1375](../../src/production_tool_control.rs#L1375) | receiver-type-required |
| `decode_cursor` | `"invalid cursor offset".into` | [1377](../../src/production_tool_control.rs#L1377) | receiver-type-required |
| `context_start_genesis` | `source.get` | [1400](../../src/production_tool_control.rs#L1400), [1404](../../src/production_tool_control.rs#L1404), [1406](../../src/production_tool_control.rs#L1406) | receiver-type-required |
| `context_start_genesis` | `value.clone` | [1401](../../src/production_tool_control.rs#L1401) | receiver-type-required |
| `context_start_genesis` | `source.get("folders").is_some` | [1404](../../src/production_tool_control.rs#L1404) | receiver-type-required |
| `context_start_genesis` | `source.get("preset").is_some` | [1406](../../src/production_tool_control.rs#L1406) | receiver-type-required |
| `context_start_genesis` | `Event::from_value(to_ijson(&event)?).map_err` | [1409](../../src/production_tool_control.rs#L1409) | receiver-type-required |
| `context_start_genesis` | `Event::from_value` | [1409](../../src/production_tool_control.rs#L1409) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `context_start_genesis` | `to_ijson` | [1409](../../src/production_tool_control.rs#L1409) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1456) |
| `deterministic_uuid` | `request_id.as_bytes()[..32].to_vec` | [1413](../../src/production_tool_control.rs#L1413) | receiver-type-required |
| `deterministic_uuid` | `request_id.as_bytes` | [1413](../../src/production_tool_control.rs#L1413) | receiver-type-required |
| `deterministic_uuid` | `String::from_utf8(bytes).expect` | [1421](../../src/production_tool_control.rs#L1421) | receiver-type-required |
| `deterministic_uuid` | `String::from_utf8` | [1421](../../src/production_tool_control.rs#L1421) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `Path::new` | [1433](../../src/production_tool_control.rs#L1433) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `path.components().count` | [1434](../../src/production_tool_control.rs#L1434) | receiver-type-required |
| `validate_line_file_name` | `path.components` | [1434](../../src/production_tool_control.rs#L1434) | receiver-type-required |
| `validate_line_file_name` | `path.extension().and_then` | [1436](../../src/production_tool_control.rs#L1436) | receiver-type-required |
| `validate_line_file_name` | `path.extension` | [1436](../../src/production_tool_control.rs#L1436) | receiver-type-required |
| `validate_line_file_name` | `value.to_str` | [1436](../../src/production_tool_control.rs#L1436) | receiver-type-required |
| `validate_line_file_name` | `Some` | [1436](../../src/production_tool_control.rs#L1436) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `Err` | [1438](../../src/production_tool_control.rs#L1438) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `SupervisorOperationError::Denied` | [1438](../../src/production_tool_control.rs#L1438) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `"line file escapes the session folder".into` | [1439](../../src/production_tool_control.rs#L1439) | receiver-type-required |
| `validate_line_file_name` | `Ok` | [1442](../../src/production_tool_control.rs#L1442) | external-constructor-callback-or-unresolved |
| `bounded_snippet` | `value.len` | [1446](../../src/production_tool_control.rs#L1446) | receiver-type-required |
| `bounded_snippet` | `value.to_owned` | [1447](../../src/production_tool_control.rs#L1447) | receiver-type-required |
| `bounded_snippet` | `value.is_char_boundary` | [1450](../../src/production_tool_control.rs#L1450) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse(         &serde_json::to_vec(value)             .map_err(&#124;error&#124; SupervisorOperationError::Protocol(error.to_string()))?,     )     .map_err` | [1457](../../src/production_tool_control.rs#L1457) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [1457](../../src/production_tool_control.rs#L1457) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value)             .map_err` | [1458](../../src/production_tool_control.rs#L1458) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [1458](../../src/production_tool_control.rs#L1458) | external-constructor-callback-or-unresolved |
| `to_ijson` | `SupervisorOperationError::Protocol` | [1459](../../src/production_tool_control.rs#L1459) | external-constructor-callback-or-unresolved |
| `to_ijson` | `error.to_string` | [1459](../../src/production_tool_control.rs#L1459) | receiver-type-required |
| `map_io` | `error.kind` | [1465](../../src/production_tool_control.rs#L1465) | receiver-type-required |
| `map_io` | `SupervisorOperationError::NotFound` | [1466](../../src/production_tool_control.rs#L1466) | external-constructor-callback-or-unresolved |
| `map_io` | `"resource not found".into` | [1466](../../src/production_tool_control.rs#L1466) | receiver-type-required |
| `map_io` | `SupervisorOperationError::Unavailable` | [1468](../../src/production_tool_control.rs#L1468) | external-constructor-callback-or-unresolved |
| `map_io` | `error.to_string` | [1468](../../src/production_tool_control.rs#L1468) | receiver-type-required |
| `map_store` | `SupervisorOperationError::NotFound` | [1475](../../src/production_tool_control.rs#L1475) | external-constructor-callback-or-unresolved |
| `map_store` | `"thread not found".into` | [1475](../../src/production_tool_control.rs#L1475) | receiver-type-required |
| `map_store` | `SupervisorOperationError::Conflict` | [1478](../../src/production_tool_control.rs#L1478) | external-constructor-callback-or-unresolved |
| `map_store` | `"thread is archived".into` | [1478](../../src/production_tool_control.rs#L1478) | receiver-type-required |
| `map_store` | `SupervisorOperationError::Unavailable` | [1480](../../src/production_tool_control.rs#L1480) | external-constructor-callback-or-unresolved |
| `map_store` | `other.to_string` | [1480](../../src/production_tool_control.rs#L1480) | receiver-type-required |
| `map_schema` | `SupervisorOperationError::Corruption` | [1485](../../src/production_tool_control.rs#L1485) | external-constructor-callback-or-unresolved |
| `map_schema` | `error.to_string` | [1485](../../src/production_tool_control.rs#L1485) | receiver-type-required |
| `into_control_error` | `self.to_string` | [1536](../../src/production_tool_control.rs#L1536) | receiver-type-required |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `SupervisorOperationError::ToolFailed("remote details".into()).into_control_error` | [1548](../../src/production_tool_control.rs#L1548) | receiver-type-required |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `SupervisorOperationError::ToolFailed` | [1548](../../src/production_tool_control.rs#L1548) | external-constructor-callback-or-unresolved |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `"remote details".into` | [1548](../../src/production_tool_control.rs#L1548) | receiver-type-required |
| `value` | `IJsonValue::parse(raw.as_bytes()).unwrap` | [1566](../../src/production_tool_control.rs#L1566) | receiver-type-required |
| `value` | `IJsonValue::parse` | [1566](../../src/production_tool_control.rs#L1566) | external-constructor-callback-or-unresolved |
| `value` | `raw.as_bytes` | [1566](../../src/production_tool_control.rs#L1566) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ProductionToolControlPolicy::new` | [1573](../../src/production_tool_control.rs#L1573), [1624](../../src/production_tool_control.rs#L1624) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `tools::SecretScanner::default` | [1573](../../src/production_tool_control.rs#L1573) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControlResult::pending` | [1574](../../src/production_tool_control.rs#L1574) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"a".repeat` | [1575](../../src/production_tool_control.rs#L1575) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"b".repeat` | [1577](../../src/production_tool_control.rs#L1577), [1592](../../src/production_tool_control.rs#L1592) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `value` | [1578](../../src/production_tool_control.rs#L1578), [1588](../../src/production_tool_control.rs#L1588), [1598](../../src/production_tool_control.rs#L1598), [1606](../../src/production_tool_control.rs#L1606) | [tekes-supervisor::production_tool_control::continuation_policy_tests::value](../../src/production_tool_control.rs#L1565) |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `policy.apply` | [1580](../../src/production_tool_control.rs#L1580) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `pending.clone` | [1580](../../src/production_tool_control.rs#L1580) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControl::new(             "018f0000-0000-7000-8000-000000000003",             "018f0000-0000-7000-8000-000000000003",             1,             "call-1",             "mcp__t__x",             value("{}"),         )         .unwrap` | [1582](../../src/production_tool_control.rs#L1582) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControl::new` | [1582](../../src/production_tool_control.rs#L1582) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolContinuationRequest::new(original, "b".repeat(64), 1, ContinuationOperation::Query)                 .unwrap` | [1592](../../src/production_tool_control.rs#L1592) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolContinuationRequest::new` | [1592](../../src/production_tool_control.rs#L1592) | [worker-control::continuation::ToolContinuationRequest::new](../../../worker-control/src/continuation.rs#L32) |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `request.request_id.clone` | [1595](../../src/production_tool_control.rs#L1595), [1603](../../src/production_tool_control.rs#L1603) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"call-1".into` | [1596](../../src/production_tool_control.rs#L1596), [1604](../../src/production_tool_control.rs#L1604) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `policy.apply_continuation` | [1611](../../src/production_tool_control.rs#L1611) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str()                     .unwrap()                     .to_owned` | [1613](../../src/production_tool_control.rs#L1613) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str()                     .unwrap` | [1613](../../src/production_tool_control.rs#L1613) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str` | [1613](../../src/production_tool_control.rs#L1613) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap` | [1613](../../src/production_tool_control.rs#L1613) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value` | [1613](../../src/production_tool_control.rs#L1613) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `tools::SecretScanner::failing` | [1624](../../src/production_tool_control.rs#L1624) | [tools::pipeline::SecretScanner::failing](../../../tools/src/pipeline.rs#L107) |
| `a_started_session_inherits_the_callers_scope_and_preset` | `"kernel-worker".into` | [1653](../../src/production_tool_control.rs#L1653) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `"context".into` | [1654](../../src/production_tool_control.rs#L1654) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `session.into` | [1655](../../src/production_tool_control.rs#L1655) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `"create".into` | [1656](../../src/production_tool_control.rs#L1656) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `"request".into` | [1657](../../src/production_tool_control.rs#L1657) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `context_start_genesis(&source, "ws", session, "2026-10-15T00:00:00.000Z", &origin)                 .expect` | [1660](../../src/production_tool_control.rs#L1660) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `context_start_genesis` | [1660](../../src/production_tool_control.rs#L1660), [1677](../../src/production_tool_control.rs#L1677) | external-constructor-callback-or-unresolved |
| `a_started_session_inherits_the_callers_scope_and_preset` | `serde_json::to_value(event.raw()).unwrap` | [1662](../../src/production_tool_control.rs#L1662), [1679](../../src/production_tool_control.rs#L1679) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `serde_json::to_value` | [1662](../../src/production_tool_control.rs#L1662), [1679](../../src/production_tool_control.rs#L1679) | external-constructor-callback-or-unresolved |
| `a_started_session_inherits_the_callers_scope_and_preset` | `event.raw` | [1662](../../src/production_tool_control.rs#L1662), [1679](../../src/production_tool_control.rs#L1679) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `source.clone` | [1671](../../src/production_tool_control.rs#L1671) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `plain.as_object_mut().unwrap` | [1672](../../src/production_tool_control.rs#L1672) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `plain.as_object_mut` | [1672](../../src/production_tool_control.rs#L1672) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `object.remove` | [1673](../../src/production_tool_control.rs#L1673), [1675](../../src/production_tool_control.rs#L1675) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `object.insert` | [1674](../../src/production_tool_control.rs#L1674) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `"identity_profile".into` | [1674](../../src/production_tool_control.rs#L1674) | receiver-type-required |
| `a_started_session_inherits_the_callers_scope_and_preset` | `context_start_genesis(&plain, "ws", session, "2026-10-15T00:00:00.000Z", &origin)                 .expect` | [1677](../../src/production_tool_control.rs#L1677) | receiver-type-required |
