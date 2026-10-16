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
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads](../../src/production_tool_control.rs#L550) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read](../../src/production_tool_control.rs#L586) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records](../../src/production_tool_control.rs#L601) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search](../../src/production_tool_control.rs#L630) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start](../../src/production_tool_control.rs#L711) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork](../../src/production_tool_control.rs#L741) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get](../../src/production_tool_control.rs#L761) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L839) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy](../../src/production_tool_control.rs#L847) | struct_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::new](../../src/production_tool_control.rs#L853) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply_continuation](../../src/production_tool_control.rs#L863) | function_item | `pub` |  |
| [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::apply](../../src/production_tool_control.rs#L905) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::policy_withheld](../../src/production_tool_control.rs#L944) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::child_launch_proof](../../src/production_tool_control.rs#L956) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::parent_report_proof](../../src/production_tool_control.rs#L1028) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1090) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::resolve_visible_line](../../src/production_tool_control.rs#L1111) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1132) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::require_active_session](../../src/production_tool_control.rs#L1152) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1162) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1208) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1216) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::event_record](../../src/production_tool_control.rs#L1232) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1238) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::string_array](../../src/production_tool_control.rs#L1246) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::job_record_outcome](../../src/production_tool_control.rs#L1261) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1274) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1293) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1306) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1311) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::context_start_genesis](../../src/production_tool_control.rs#L1337) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1364) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1384) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::bounded_snippet](../../src/production_tool_control.rs#L1397) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_io](../../src/production_tool_control.rs#L1416) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_store](../../src/production_tool_control.rs#L1424) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::map_schema](../../src/production_tool_control.rs#L1436) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::SupervisorOperationError](../../src/production_tool_control.rs#L1441) | enum_item | `pub` |  |
| [tekes-supervisor::production_tool_control::SupervisorOperationError::into_control_error](../../src/production_tool_control.rs#L1471) | function_item | `private` |  |
| [tekes-supervisor::production_tool_control::tool_failure_tests::definitive_remote_tool_failure_is_not_retryable_transport_loss](../../src/production_tool_control.rs#L1498) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::continuation_policy_tests::value](../../src/production_tool_control.rs#L1517) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::continuation_policy_tests::pending_results_and_continuation_responses_pass_the_secret_policy](../../src/production_tool_control.rs#L1524) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::production_tool_control::context_start_tests::a_started_session_inherits_execution_without_legacy_management](../../src/production_tool_control.rs#L1594) | function_item | `private` | test; #[cfg(test)] |

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

<details><summary>Functions 1–20: 51 direct edges</summary>

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
  n9["tekes-supervisor::production_tool_control::genesis_line"]
  n10["tekes-supervisor::production_tool_control::required_string"]
  n11["tekes-supervisor::production_tool_control::string_array"]
  n12["tekes-supervisor::production_tool_control::job_record_outcome"]
  n13["tekes-supervisor::production_tool_control::map_backend"]
  n14["tekes-supervisor::production_tool_control::argument_limit"]
  n15["tekes-supervisor::production_tool_control::encode_cursor"]
  n16["tekes-supervisor::production_tool_control::decode_cursor"]
  n17["tekes-supervisor::production_tool_control::to_ijson"]
  n18["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::execute_outcome"]
  n19["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::continue_task"]
  n20["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::reconcile"]
  n21["tekes-supervisor::production_tool_control::DynamicSupervisorAuthority::cancel_inflight"]
  n22["tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::new"]
  n23["tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::execute"]
  n24["tekes-supervisor::production_tool_control::ProductionToolControlHandler::new"]
  n25["tekes-supervisor::production_tool_control::ProductionToolControlHandler::with_dynamic_routes"]
  n26["tekes-supervisor::production_tool_control::ProductionToolControlHandler::runtime"]
  n27["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute"]
  n28["tekes-supervisor::production_tool_control::ProductionToolControlHandler::recovery"]
  n29["tekes-supervisor::production_tool_control::ProductionToolControlHandler::reconcile"]
  n30["tekes-supervisor::production_tool_control::ProductionToolControlHandler::continue_task"]
  n31["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_inner"]
  n32["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic"]
  n33["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context"]
  n34["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads"]
  n35["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read"]
  n36["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records"]
  n37["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search"]
  n38["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start"]
  n39["tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork"]
  n40["tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get"]
  n41["tekes-supervisor::production_tool_control::is_fixed_supervisor_tool"]
  n42["tekes-supervisor::production_tool_control::child_launch_proof"]
  n43["tekes-supervisor::tool_control::resolve_durable_binding"]
  n44["tools::runtime_backends::JobBroker::new"]
  n45["tools::schema_registry::validate_fixed_arguments"]
  n46["worker-control::durable::ToolControlResult::pending"]
  n47["worker-control::durable::ToolControlResult::success"]
  n48["worker-control::durable::ToolControlResult::failure"]
  n22 --> n44
  n23 --> n10
  n23 --> n11
  n23 --> n12
  n23 --> n13
  n23 --> n17
  n27 --> n46
  n27 --> n47
  n27 --> n48
  n28 --> n41
  n31 --> n2
  n31 --> n17
  n31 --> n32
  n31 --> n33
  n31 --> n40
  n31 --> n41
  n31 --> n42
  n31 --> n43
  n31 --> n45
  n32 --> n0
  n33 --> n1
  n33 --> n5
  n33 --> n7
  n33 --> n10
  n33 --> n17
  n33 --> n34
  n33 --> n35
  n33 --> n36
  n33 --> n37
  n33 --> n38
  n33 --> n39
  n34 --> n6
  n34 --> n8
  n34 --> n17
  n35 --> n7
  n35 --> n8
  n35 --> n17
  n36 --> n10
  n36 --> n14
  n36 --> n15
  n36 --> n16
  n36 --> n17
  n37 --> n3
  n37 --> n4
  n37 --> n7
  n37 --> n9
  n37 --> n10
  n37 --> n14
  n37 --> n15
  n37 --> n16
  n37 --> n17
```

</details>

<details><summary>Functions 21–40: 33 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["schema::effective_session_genesis"]
  n2["store::folder::ThreadStore::open"]
  n3["store::tail::scan_valid_prefix"]
  n4["store::tail::read_genesis"]
  n5["tekes-supervisor::production_tool_control::parent_report_proof"]
  n6["tekes-supervisor::production_tool_control::session_lines"]
  n7["tekes-supervisor::production_tool_control::resolve_visible_line"]
  n8["tekes-supervisor::production_tool_control::visible_sessions"]
  n9["tekes-supervisor::production_tool_control::require_active_session"]
  n10["tekes-supervisor::production_tool_control::validate_session_folder"]
  n11["tekes-supervisor::production_tool_control::session_folder"]
  n12["tekes-supervisor::production_tool_control::read_projection"]
  n13["tekes-supervisor::production_tool_control::genesis_line"]
  n14["tekes-supervisor::production_tool_control::paired_tool_timestamp"]
  n15["tekes-supervisor::production_tool_control::event_record"]
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
  n5 --> n12
  n5 --> n13
  n5 --> n20
  n6 --> n12
  n7 --> n6
  n7 --> n8
  n7 --> n11
  n7 --> n13
  n8 --> n10
  n9 --> n11
  n10 --> n0
  n10 --> n4
  n11 --> n0
  n11 --> n10
  n12 --> n3
  n22 --> n1
  n22 --> n2
  n22 --> n14
  n22 --> n18
  n22 --> n19
  n23 --> n2
  n23 --> n14
  n23 --> n19
  n23 --> n21
  n24 --> n7
  n24 --> n16
  n24 --> n17
  n24 --> n21
  n28 --> n29
  n29 --> n31
  n30 --> n12
  n30 --> n13
  n30 --> n20
```

</details>

<details><summary>Functions 41–56: 6 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::event::Event::from_value"]
  n1["schema::ijson::IJsonValue::parse"]
  n2["tekes-supervisor::production_tool_control::required_string"]
  n3["tekes-supervisor::production_tool_control::string_array"]
  n4["tekes-supervisor::production_tool_control::job_record_outcome"]
  n5["tekes-supervisor::production_tool_control::map_backend"]
  n6["tekes-supervisor::production_tool_control::argument_limit"]
  n7["tekes-supervisor::production_tool_control::encode_cursor"]
  n8["tekes-supervisor::production_tool_control::decode_cursor"]
  n9["tekes-supervisor::production_tool_control::context_start_genesis"]
  n10["tekes-supervisor::production_tool_control::deterministic_uuid"]
  n11["tekes-supervisor::production_tool_control::validate_line_file_name"]
  n12["tekes-supervisor::production_tool_control::bounded_snippet"]
  n13["tekes-supervisor::production_tool_control::to_ijson"]
  n14["tekes-supervisor::production_tool_control::map_io"]
  n15["tekes-supervisor::production_tool_control::map_store"]
  n16["tekes-supervisor::production_tool_control::map_schema"]
  n17["tekes-supervisor::production_tool_control::SupervisorOperationError::into_control_error"]
  n4 --> n5
  n4 --> n13
  n8 --> n7
  n9 --> n0
  n9 --> n13
  n13 --> n1
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
| `execute` | `required_string` | [208](../../src/production_tool_control.rs#L208), [218](../../src/production_tool_control.rs#L218), [248](../../src/production_tool_control.rs#L248), [252](../../src/production_tool_control.rs#L252) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1238) |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter(&#124;value&#124; !value.is_empty())                     .map(ToOwned::to_owned)                     .unwrap_or_else` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter(&#124;value&#124; !value.is_empty())                     .map` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then(Value::as_str)                     .filter` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get("job_id")                     .and_then` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `arguments                     .get` | [211](../../src/production_tool_control.rs#L211) | receiver-type-required |
| `execute` | `value.is_empty` | [214](../../src/production_tool_control.rs#L214), [223](../../src/production_tool_control.rs#L223) | receiver-type-required |
| `execute` | `required_string(&arguments, "program")?.to_owned` | [218](../../src/production_tool_control.rs#L218) | receiver-type-required |
| `execute` | `string_array` | [219](../../src/production_tool_control.rs#L219), [225](../../src/production_tool_control.rs#L225) | [tekes-supervisor::production_tool_control::string_array](../../src/production_tool_control.rs#L1246) |
| `execute` | `arguments                         .get("working_directory")                         .and_then(Value::as_str)                         .filter(&#124;value&#124; !value.is_empty())                         .map` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get("working_directory")                         .and_then(Value::as_str)                         .filter` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get("working_directory")                         .and_then` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `arguments                         .get` | [220](../../src/production_tool_control.rs#L220) | receiver-type-required |
| `execute` | `Default::default` | [226](../../src/production_tool_control.rs#L226) | external-constructor-callback-or-unresolved |
| `execute` | `job_record_outcome` | [228](../../src/production_tool_control.rs#L228), [249](../../src/production_tool_control.rs#L249), [253](../../src/production_tool_control.rs#L253) | [tekes-supervisor::production_tool_control::job_record_outcome](../../src/production_tool_control.rs#L1261) |
| `execute` | `self.broker.start` | [228](../../src/production_tool_control.rs#L228) | receiver-type-required |
| `execute` | `Arc::clone` | [232](../../src/production_tool_control.rs#L232) | external-constructor-callback-or-unresolved |
| `execute` | `self.broker.list` | [237](../../src/production_tool_control.rs#L237) | receiver-type-required |
| `execute` | `to_ijson` | [238](../../src/production_tool_control.rs#L238) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `execute` | `Err` | [239](../../src/production_tool_control.rs#L239), [240](../../src/production_tool_control.rs#L240), [244](../../src/production_tool_control.rs#L244), [255](../../src/production_tool_control.rs#L255) | external-constructor-callback-or-unresolved |
| `execute` | `map_backend` | [239](../../src/production_tool_control.rs#L239) | [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1274) |
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
| `recovery` | `is_fixed_supervisor_tool` | [328](../../src/production_tool_control.rs#L328) | [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L839) |
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
| `execute_inner` | `is_fixed_supervisor_tool` | [420](../../src/production_tool_control.rs#L420) | [tekes-supervisor::production_tool_control::is_fixed_supervisor_tool](../../src/production_tool_control.rs#L839) |
| `execute_inner` | `self.execute_dynamic` | [421](../../src/production_tool_control.rs#L421) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic](../../src/production_tool_control.rs#L448) |
| `execute_inner` | `session_folder.join` | [421](../../src/production_tool_control.rs#L421) | receiver-type-required |
| `execute_inner` | `validate_fixed_arguments(&request.name, &arguments)             .map_err` | [423](../../src/production_tool_control.rs#L423) | receiver-type-required |
| `execute_inner` | `validate_fixed_arguments` | [423](../../src/production_tool_control.rs#L423) | [tools::schema_registry::validate_fixed_arguments](../../../tools/src/schema_registry.rs#L1228) |
| `execute_inner` | `SupervisorOperationError::Invalid` | [424](../../src/production_tool_control.rs#L424), [436](../../src/production_tool_control.rs#L436) | external-constructor-callback-or-unresolved |
| `execute_inner` | `request.name.as_str` | [425](../../src/production_tool_control.rs#L425) | receiver-type-required |
| `execute_inner` | `self.execute_context` | [426](../../src/production_tool_control.rs#L426) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context](../../src/production_tool_control.rs#L479) |
| `execute_inner` | `self.execute_context_get` | [427](../../src/production_tool_control.rs#L427) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_context_get](../../src/production_tool_control.rs#L761) |
| `execute_inner` | `self.jobs.execute` | [428](../../src/production_tool_control.rs#L428) | receiver-type-required |
| `execute_inner` | `child_launch_proof` | [430](../../src/production_tool_control.rs#L430) | [tekes-supervisor::production_tool_control::child_launch_proof](../../src/production_tool_control.rs#L956) |
| `execute_inner` | `self.runtime.ensure_child` | [431](../../src/production_tool_control.rs#L431) | receiver-type-required |
| `execute_inner` | `parent_report_proof` | [434](../../src/production_tool_control.rs#L434) | [tekes-supervisor::production_tool_control::parent_report_proof](../../src/production_tool_control.rs#L1028) |
| `execute_inner` | `arguments.get("result").ok_or_else` | [435](../../src/production_tool_control.rs#L435) | receiver-type-required |
| `execute_inner` | `arguments.get` | [435](../../src/production_tool_control.rs#L435) | receiver-type-required |
| `execute_inner` | `"report.result missing".into` | [436](../../src/production_tool_control.rs#L436) | receiver-type-required |
| `execute_inner` | `to_ijson` | [438](../../src/production_tool_control.rs#L438) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
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
| `execute_context` | `self.context_threads` | [490](../../src/production_tool_control.rs#L490) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_threads](../../src/production_tool_control.rs#L550) |
| `execute_context` | `required_string` | [492](../../src/production_tool_control.rs#L492), [500](../../src/production_tool_control.rs#L500), [502](../../src/production_tool_control.rs#L502), [516](../../src/production_tool_control.rs#L516), [521](../../src/production_tool_control.rs#L521), [535](../../src/production_tool_control.rs#L535) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1238) |
| `execute_context` | `self.context_read` | [493](../../src/production_tool_control.rs#L493) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_read](../../src/production_tool_control.rs#L586) |
| `execute_context` | `self.context_records` | [495](../../src/production_tool_control.rs#L495) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_records](../../src/production_tool_control.rs#L601) |
| `execute_context` | `self.context_search` | [496](../../src/production_tool_control.rs#L496) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_search](../../src/production_tool_control.rs#L630) |
| `execute_context` | `self.context_start` | [497](../../src/production_tool_control.rs#L497) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_start](../../src/production_tool_control.rs#L711) |
| `execute_context` | `self.context_fork` | [498](../../src/production_tool_control.rs#L498) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::context_fork](../../src/production_tool_control.rs#L741) |
| `execute_context` | `session_folder` | [501](../../src/production_tool_control.rs#L501), [517](../../src/production_tool_control.rs#L517), [536](../../src/production_tool_control.rs#L536) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) |
| `execute_context` | `self.runtime.deliver_input` | [503](../../src/production_tool_control.rs#L503) | receiver-type-required |
| `execute_context` | `request.session.clone` | [504](../../src/production_tool_control.rs#L504), [538](../../src/production_tool_control.rs#L538) | receiver-type-required |
| `execute_context` | `request.thread.clone` | [505](../../src/production_tool_control.rs#L505), [539](../../src/production_tool_control.rs#L539) | receiver-type-required |
| `execute_context` | `target.to_owned` | [506](../../src/production_tool_control.rs#L506), [540](../../src/production_tool_control.rs#L540) | receiver-type-required |
| `execute_context` | `request.request_id.clone` | [507](../../src/production_tool_control.rs#L507), [541](../../src/production_tool_control.rs#L541) | receiver-type-required |
| `execute_context` | `message.to_owned` | [508](../../src/production_tool_control.rs#L508) | receiver-type-required |
| `execute_context` | `arguments                         .get("goal")                         .and_then(Value::as_bool)                         .unwrap_or` | [509](../../src/production_tool_control.rs#L509) | receiver-type-required |
| `execute_context` | `arguments                         .get("goal")                         .and_then` | [509](../../src/production_tool_control.rs#L509) | receiver-type-required |
| `execute_context` | `arguments                         .get` | [509](../../src/production_tool_control.rs#L509) | receiver-type-required |
| `execute_context` | `self.runtime.ensure_running` | [518](../../src/production_tool_control.rs#L518) | receiver-type-required |
| `execute_context` | `Err` | [523](../../src/production_tool_control.rs#L523), [544](../../src/production_tool_control.rs#L544) | external-constructor-callback-or-unresolved |
| `execute_context` | `SupervisorOperationError::Denied` | [523](../../src/production_tool_control.rs#L523) | external-constructor-callback-or-unresolved |
| `execute_context` | `"a running worker cannot archive its containing session".into` | [524](../../src/production_tool_control.rs#L524) | receiver-type-required |
| `execute_context` | `require_active_session` | [527](../../src/production_tool_control.rs#L527) | [tekes-supervisor::production_tool_control::require_active_session](../../src/production_tool_control.rs#L1152) |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err(map_store)?                     .archive(target)                     .map_err` | [528](../../src/production_tool_control.rs#L528) | receiver-type-required |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err(map_store)?                     .archive` | [528](../../src/production_tool_control.rs#L528) | receiver-type-required |
| `execute_context` | `ThreadStore::open(&self.root)                     .map_err` | [528](../../src/production_tool_control.rs#L528) | receiver-type-required |
| `execute_context` | `ThreadStore::open` | [528](../../src/production_tool_control.rs#L528) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `execute_context` | `to_ijson` | [532](../../src/production_tool_control.rs#L532) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `execute_context` | `self.runtime.interrupt` | [537](../../src/production_tool_control.rs#L537) | receiver-type-required |
| `execute_context` | `SupervisorOperationError::Unsupported` | [544](../../src/production_tool_control.rs#L544) | external-constructor-callback-or-unresolved |
| `context_threads` | `Vec::new` | [551](../../src/production_tool_control.rs#L551) | external-constructor-callback-or-unresolved |
| `context_threads` | `fs::read_dir(self.root.join(area)).map_err` | [553](../../src/production_tool_control.rs#L553) | receiver-type-required |
| `context_threads` | `fs::read_dir` | [553](../../src/production_tool_control.rs#L553) | external-constructor-callback-or-unresolved |
| `context_threads` | `self.root.join` | [553](../../src/production_tool_control.rs#L553) | receiver-type-required |
| `context_threads` | `entry.map_err` | [554](../../src/production_tool_control.rs#L554) | receiver-type-required |
| `context_threads` | `entry.file_type().map_err(map_io)?.is_dir` | [555](../../src/production_tool_control.rs#L555) | receiver-type-required |
| `context_threads` | `entry.file_type().map_err` | [555](../../src/production_tool_control.rs#L555) | receiver-type-required |
| `context_threads` | `entry.file_type` | [555](../../src/production_tool_control.rs#L555) | receiver-type-required |
| `context_threads` | `entry.file_name().to_string_lossy().into_owned` | [558](../../src/production_tool_control.rs#L558) | receiver-type-required |
| `context_threads` | `entry.file_name().to_string_lossy` | [558](../../src/production_tool_control.rs#L558) | receiver-type-required |
| `context_threads` | `entry.file_name` | [558](../../src/production_tool_control.rs#L558) | receiver-type-required |
| `context_threads` | `validate_session_folder` | [559](../../src/production_tool_control.rs#L559) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1162) |
| `context_threads` | `entry.path` | [559](../../src/production_tool_control.rs#L559), [560](../../src/production_tool_control.rs#L560) | receiver-type-required |
| `context_threads` | `read_projection` | [560](../../src/production_tool_control.rs#L560) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) |
| `context_threads` | `entry.path().join` | [560](../../src/production_tool_control.rs#L560) | receiver-type-required |
| `context_threads` | `projection.events.first().expect` | [561](../../src/production_tool_control.rs#L561) | receiver-type-required |
| `context_threads` | `projection.events.first` | [561](../../src/production_tool_control.rs#L561) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter()                     .rev()                     .find_map` | [562](../../src/production_tool_control.rs#L562) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter()                     .rev` | [562](../../src/production_tool_control.rs#L562) | receiver-type-required |
| `context_threads` | `projection                     .events                     .iter` | [562](../../src/production_tool_control.rs#L562) | receiver-type-required |
| `context_threads` | `event.string_field` | [566](../../src/production_tool_control.rs#L566) | receiver-type-required |
| `context_threads` | `items.push` | [567](../../src/production_tool_control.rs#L567) | receiver-type-required |
| `context_threads` | `items.sort_by` | [578](../../src/production_tool_control.rs#L578) | receiver-type-required |
| `context_threads` | `left.get("session_id")                 .and_then(Value::as_str)                 .cmp` | [579](../../src/production_tool_control.rs#L579) | receiver-type-required |
| `context_threads` | `left.get("session_id")                 .and_then` | [579](../../src/production_tool_control.rs#L579) | receiver-type-required |
| `context_threads` | `left.get` | [579](../../src/production_tool_control.rs#L579) | receiver-type-required |
| `context_threads` | `right.get("session_id").and_then` | [581](../../src/production_tool_control.rs#L581) | receiver-type-required |
| `context_threads` | `right.get` | [581](../../src/production_tool_control.rs#L581) | receiver-type-required |
| `context_threads` | `to_ijson` | [583](../../src/production_tool_control.rs#L583) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `context_read` | `session_folder` | [587](../../src/production_tool_control.rs#L587) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) |
| `context_read` | `read_projection` | [588](../../src/production_tool_control.rs#L588) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) |
| `context_read` | `folder.join` | [588](../../src/production_tool_control.rs#L588) | receiver-type-required |
| `context_read` | `projection.events.first().expect` | [589](../../src/production_tool_control.rs#L589) | receiver-type-required |
| `context_read` | `projection.events.first` | [589](../../src/production_tool_control.rs#L589) | receiver-type-required |
| `context_read` | `to_ijson` | [590](../../src/production_tool_control.rs#L590) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `context_records` | `required_string(arguments, "turn_id")?             .parse::<u64>()             .map_err` | [606](../../src/production_tool_control.rs#L606) | receiver-type-required |
| `context_records` | `required_string(arguments, "turn_id")?             .parse::<u64>` | [606](../../src/production_tool_control.rs#L606) | receiver-type-required |
| `context_records` | `required_string` | [606](../../src/production_tool_control.rs#L606) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1238) |
| `context_records` | `SupervisorOperationError::Invalid` | [609](../../src/production_tool_control.rs#L609) | external-constructor-callback-or-unresolved |
| `context_records` | `"turn_id is not a positive integer".into` | [609](../../src/production_tool_control.rs#L609) | receiver-type-required |
| `context_records` | `decode_cursor` | [611](../../src/production_tool_control.rs#L611) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1311) |
| `context_records` | `arguments.get` | [611](../../src/production_tool_control.rs#L611) | receiver-type-required |
| `context_records` | `argument_limit` | [612](../../src/production_tool_control.rs#L612) | [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1293) |
| `context_records` | `resolved             .projection             .events             .iter()             .filter(&#124;event&#124; event.turn() == Some(turn))             .collect::<Vec<_>>` | [613](../../src/production_tool_control.rs#L613) | receiver-type-required |
| `context_records` | `resolved             .projection             .events             .iter()             .filter` | [613](../../src/production_tool_control.rs#L613) | receiver-type-required |
| `context_records` | `resolved             .projection             .events             .iter` | [613](../../src/production_tool_control.rs#L613) | receiver-type-required |
| `context_records` | `event.turn` | [617](../../src/production_tool_control.rs#L617) | receiver-type-required |
| `context_records` | `Some` | [617](../../src/production_tool_control.rs#L617) | external-constructor-callback-or-unresolved |
| `context_records` | `matching             .iter()             .skip(offset)             .take(limit)             .map(event_record)             .collect::<Result<Vec<_>, _>>` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip(offset)             .take(limit)             .map` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip(offset)             .take` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `matching             .iter()             .skip` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `matching             .iter` | [619](../../src/production_tool_control.rs#L619) | receiver-type-required |
| `context_records` | `(offset + records.len() < matching.len())             .then` | [625](../../src/production_tool_control.rs#L625) | receiver-type-required |
| `context_records` | `records.len` | [625](../../src/production_tool_control.rs#L625), [626](../../src/production_tool_control.rs#L626) | receiver-type-required |
| `context_records` | `matching.len` | [625](../../src/production_tool_control.rs#L625) | receiver-type-required |
| `context_records` | `encode_cursor` | [626](../../src/production_tool_control.rs#L626) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1306) |
| `context_records` | `to_ijson` | [627](../../src/production_tool_control.rs#L627) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `context_search` | `required_string` | [635](../../src/production_tool_control.rs#L635), [644](../../src/production_tool_control.rs#L644), [659](../../src/production_tool_control.rs#L659) | [tekes-supervisor::production_tool_control::required_string](../../src/production_tool_control.rs#L1238) |
| `context_search` | `query.to_lowercase` | [636](../../src/production_tool_control.rs#L636) | receiver-type-required |
| `context_search` | `arguments             .get("scope")             .and_then(Value::as_str)             .unwrap_or` | [637](../../src/production_tool_control.rs#L637) | receiver-type-required |
| `context_search` | `arguments             .get("scope")             .and_then` | [637](../../src/production_tool_control.rs#L637) | receiver-type-required |
| `context_search` | `arguments             .get` | [637](../../src/production_tool_control.rs#L637) | receiver-type-required |
| `context_search` | `session_folder` | [645](../../src/production_tool_control.rs#L645), [668](../../src/production_tool_control.rs#L668) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) |
| `context_search` | `visible_sessions` | [648](../../src/production_tool_control.rs#L648) | [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1132) |
| `context_search` | `Err` | [650](../../src/production_tool_control.rs#L650) | external-constructor-callback-or-unresolved |
| `context_search` | `SupervisorOperationError::Invalid` | [650](../../src/production_tool_control.rs#L650) | external-constructor-callback-or-unresolved |
| `context_search` | `"invalid context.search scope".into` | [651](../../src/production_tool_control.rs#L651) | receiver-type-required |
| `context_search` | `request.session.as_str` | [658](../../src/production_tool_control.rs#L658) | receiver-type-required |
| `context_search` | `json!(["context-search-v2", scope, target, query]).to_string` | [663](../../src/production_tool_control.rs#L663) | receiver-type-required |
| `context_search` | `decode_cursor` | [664](../../src/production_tool_control.rs#L664) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1311) |
| `context_search` | `arguments.get` | [664](../../src/production_tool_control.rs#L664) | receiver-type-required |
| `context_search` | `argument_limit` | [665](../../src/production_tool_control.rs#L665) | [tekes-supervisor::production_tool_control::argument_limit](../../src/production_tool_control.rs#L1293) |
| `context_search` | `Vec::new` | [666](../../src/production_tool_control.rs#L666) | external-constructor-callback-or-unresolved |
| `context_search` | `session_lines` | [669](../../src/production_tool_control.rs#L669) | [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1090) |
| `context_search` | `genesis_line` | [670](../../src/production_tool_control.rs#L670) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1208) |
| `context_search` | `String::from_utf8(event.canonical_bytes().map_err(map_schema)?)                         .map_err` | [672](../../src/production_tool_control.rs#L672) | receiver-type-required |
| `context_search` | `String::from_utf8` | [672](../../src/production_tool_control.rs#L672) | external-constructor-callback-or-unresolved |
| `context_search` | `event.canonical_bytes().map_err` | [672](../../src/production_tool_control.rs#L672) | receiver-type-required |
| `context_search` | `event.canonical_bytes` | [672](../../src/production_tool_control.rs#L672) | receiver-type-required |
| `context_search` | `SupervisorOperationError::Protocol` | [673](../../src/production_tool_control.rs#L673) | external-constructor-callback-or-unresolved |
| `context_search` | `error.to_string` | [673](../../src/production_tool_control.rs#L673) | receiver-type-required |
| `context_search` | `canonical.to_lowercase().contains` | [674](../../src/production_tool_control.rs#L674) | receiver-type-required |
| `context_search` | `canonical.to_lowercase` | [674](../../src/production_tool_control.rs#L674) | receiver-type-required |
| `context_search` | `hits.push` | [675](../../src/production_tool_control.rs#L675) | receiver-type-required |
| `context_search` | `hits.sort_by` | [688](../../src/production_tool_control.rs#L688) | receiver-type-required |
| `context_search` | `(                 left.get("session_id").and_then(Value::as_str),                 left.get("thread_id").and_then(Value::as_str),                 left.get("record_id").and_then(Value::as_u64),             )                 .cmp` | [689](../../src/production_tool_control.rs#L689) | receiver-type-required |
| `context_search` | `left.get("session_id").and_then` | [690](../../src/production_tool_control.rs#L690) | receiver-type-required |
| `context_search` | `left.get` | [690](../../src/production_tool_control.rs#L690), [691](../../src/production_tool_control.rs#L691), [692](../../src/production_tool_control.rs#L692) | receiver-type-required |
| `context_search` | `left.get("thread_id").and_then` | [691](../../src/production_tool_control.rs#L691) | receiver-type-required |
| `context_search` | `left.get("record_id").and_then` | [692](../../src/production_tool_control.rs#L692) | receiver-type-required |
| `context_search` | `right.get("session_id").and_then` | [695](../../src/production_tool_control.rs#L695) | receiver-type-required |
| `context_search` | `right.get` | [695](../../src/production_tool_control.rs#L695), [696](../../src/production_tool_control.rs#L696), [697](../../src/production_tool_control.rs#L697) | receiver-type-required |
| `context_search` | `right.get("thread_id").and_then` | [696](../../src/production_tool_control.rs#L696) | receiver-type-required |
| `context_search` | `right.get("record_id").and_then` | [697](../../src/production_tool_control.rs#L697) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take(limit)             .cloned()             .collect::<Vec<_>>` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take(limit)             .cloned` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip(offset)             .take` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `hits             .iter()             .skip` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `hits             .iter` | [700](../../src/production_tool_control.rs#L700) | receiver-type-required |
| `context_search` | `(offset + page.len() < hits.len())             .then` | [706](../../src/production_tool_control.rs#L706) | receiver-type-required |
| `context_search` | `page.len` | [706](../../src/production_tool_control.rs#L706), [707](../../src/production_tool_control.rs#L707) | receiver-type-required |
| `context_search` | `hits.len` | [706](../../src/production_tool_control.rs#L706) | receiver-type-required |
| `context_search` | `encode_cursor` | [707](../../src/production_tool_control.rs#L707) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1306) |
| `context_search` | `to_ijson` | [708](../../src/production_tool_control.rs#L708) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `context_start` | `resolved             .projection             .events             .first()             .ok_or_else` | [716](../../src/production_tool_control.rs#L716) | receiver-type-required |
| `context_start` | `resolved             .projection             .events             .first` | [716](../../src/production_tool_control.rs#L716) | receiver-type-required |
| `context_start` | `SupervisorOperationError::Corruption` | [720](../../src/production_tool_control.rs#L720) | external-constructor-callback-or-unresolved |
| `context_start` | `"caller has no genesis".into` | [720](../../src/production_tool_control.rs#L720) | receiver-type-required |
| `context_start` | `schema::effective_session_genesis(&resolved.projection.events)             .map_err(map_schema)?             .expect` | [721](../../src/production_tool_control.rs#L721) | receiver-type-required |
| `context_start` | `schema::effective_session_genesis(&resolved.projection.events)             .map_err` | [721](../../src/production_tool_control.rs#L721) | receiver-type-required |
| `context_start` | `schema::effective_session_genesis` | [721](../../src/production_tool_control.rs#L721) | [schema::effective_session_genesis](../../../schema/src/lib.rs#L54) |
| `context_start` | `deterministic_uuid` | [724](../../src/production_tool_control.rs#L724) | [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1364) |
| `context_start` | `paired_tool_timestamp` | [725](../../src/production_tool_control.rs#L725) | [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1216) |
| `context_start` | `"kernel-worker".into` | [727](../../src/production_tool_control.rs#L727) | receiver-type-required |
| `context_start` | `"context".into` | [728](../../src/production_tool_control.rs#L728) | receiver-type-required |
| `context_start` | `session.clone` | [729](../../src/production_tool_control.rs#L729) | receiver-type-required |
| `context_start` | `"create".into` | [730](../../src/production_tool_control.rs#L730) | receiver-type-required |
| `context_start` | `request.request_id.clone` | [731](../../src/production_tool_control.rs#L731) | receiver-type-required |
| `context_start` | `context_start_genesis` | [733](../../src/production_tool_control.rs#L733) | [tekes-supervisor::production_tool_control::context_start_genesis](../../src/production_tool_control.rs#L1337) |
| `context_start` | `ThreadStore::open(&self.root)             .map_err(map_store)?             .create_thread(&session, event)             .map_err` | [734](../../src/production_tool_control.rs#L734) | receiver-type-required |
| `context_start` | `ThreadStore::open(&self.root)             .map_err(map_store)?             .create_thread` | [734](../../src/production_tool_control.rs#L734) | receiver-type-required |
| `context_start` | `ThreadStore::open(&self.root)             .map_err` | [734](../../src/production_tool_control.rs#L734) | receiver-type-required |
| `context_start` | `ThreadStore::open` | [734](../../src/production_tool_control.rs#L734) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `context_start` | `self.runtime.ensure_running` | [738](../../src/production_tool_control.rs#L738) | receiver-type-required |
| `context_fork` | `deterministic_uuid` | [746](../../src/production_tool_control.rs#L746) | [tekes-supervisor::production_tool_control::deterministic_uuid](../../src/production_tool_control.rs#L1364) |
| `context_fork` | `paired_tool_timestamp` | [747](../../src/production_tool_control.rs#L747) | [tekes-supervisor::production_tool_control::paired_tool_timestamp](../../src/production_tool_control.rs#L1216) |
| `context_fork` | `ThreadStore::open(&self.root).map_err` | [748](../../src/production_tool_control.rs#L748) | receiver-type-required |
| `context_fork` | `ThreadStore::open` | [748](../../src/production_tool_control.rs#L748) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `context_fork` | `store             .begin_fork(                 &request.request_id,                 &request.session,                 &destination,                 timestamp,             )             .map_err` | [749](../../src/production_tool_control.rs#L749) | receiver-type-required |
| `context_fork` | `store             .begin_fork` | [749](../../src/production_tool_control.rs#L749) | receiver-type-required |
| `context_fork` | `store.recover_rewrites().map_err` | [757](../../src/production_tool_control.rs#L757) | receiver-type-required |
| `context_fork` | `store.recover_rewrites` | [757](../../src/production_tool_control.rs#L757) | receiver-type-required |
| `context_fork` | `to_ijson` | [758](../../src/production_tool_control.rs#L758) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `execute_context_get` | `arguments.get("thread_id").and_then` | [767](../../src/production_tool_control.rs#L767) | receiver-type-required |
| `execute_context_get` | `arguments.get` | [767](../../src/production_tool_control.rs#L767), [796](../../src/production_tool_control.rs#L796) | receiver-type-required |
| `execute_context_get` | `resolve_visible_line` | [768](../../src/production_tool_control.rs#L768) | [tekes-supervisor::production_tool_control::resolve_visible_line](../../src/production_tool_control.rs#L1111) |
| `execute_context_get` | `resolved.projection.clone` | [769](../../src/production_tool_control.rs#L769) | receiver-type-required |
| `execute_context_get` | `arguments             .get("record_ids")             .and_then(Value::as_array)             .ok_or_else` | [771](../../src/production_tool_control.rs#L771) | receiver-type-required |
| `execute_context_get` | `arguments             .get("record_ids")             .and_then` | [771](../../src/production_tool_control.rs#L771) | receiver-type-required |
| `execute_context_get` | `arguments             .get` | [771](../../src/production_tool_control.rs#L771), [797](../../src/production_tool_control.rs#L797) | receiver-type-required |
| `execute_context_get` | `SupervisorOperationError::Invalid` | [775](../../src/production_tool_control.rs#L775), [781](../../src/production_tool_control.rs#L781), [808](../../src/production_tool_control.rs#L808) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `"context_get.record_ids missing".into` | [775](../../src/production_tool_control.rs#L775) | receiver-type-required |
| `execute_context_get` | `Vec::new` | [777](../../src/production_tool_control.rs#L777), [801](../../src/production_tool_control.rs#L801) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `BTreeSet::new` | [778](../../src/production_tool_control.rs#L778) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `id.as_u64().ok_or_else` | [780](../../src/production_tool_control.rs#L780) | receiver-type-required |
| `execute_context_get` | `id.as_u64` | [780](../../src/production_tool_control.rs#L780) | receiver-type-required |
| `execute_context_get` | `"record id is not an integer".into` | [781](../../src/production_tool_control.rs#L781) | receiver-type-required |
| `execute_context_get` | `seen.insert` | [783](../../src/production_tool_control.rs#L783) | receiver-type-required |
| `execute_context_get` | `ordered.push` | [784](../../src/production_tool_control.rs#L784) | receiver-type-required |
| `execute_context_get` | `decode_cursor` | [796](../../src/production_tool_control.rs#L796) | [tekes-supervisor::production_tool_control::decode_cursor](../../src/production_tool_control.rs#L1311) |
| `execute_context_get` | `arguments             .get("page_bytes")             .and_then(Value::as_u64)             .map_or` | [797](../../src/production_tool_control.rs#L797) | receiver-type-required |
| `execute_context_get` | `arguments             .get("page_bytes")             .and_then` | [797](../../src/production_tool_control.rs#L797) | receiver-type-required |
| `execute_context_get` | `ordered.iter().skip` | [804](../../src/production_tool_control.rs#L804) | receiver-type-required |
| `execute_context_get` | `ordered.iter` | [804](../../src/production_tool_control.rs#L804) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get(usize::try_from(*id - 1).map_err(&#124;_&#124; {                     SupervisorOperationError::Invalid("record id does not fit this platform".into())                 })?)                 .filter(&#124;event&#124; event.seq() == *id)                 .ok_or_else` | [805](../../src/production_tool_control.rs#L805) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get(usize::try_from(*id - 1).map_err(&#124;_&#124; {                     SupervisorOperationError::Invalid("record id does not fit this platform".into())                 })?)                 .filter` | [805](../../src/production_tool_control.rs#L805) | receiver-type-required |
| `execute_context_get` | `projection                 .events                 .get` | [805](../../src/production_tool_control.rs#L805) | receiver-type-required |
| `execute_context_get` | `usize::try_from(*id - 1).map_err` | [807](../../src/production_tool_control.rs#L807) | receiver-type-required |
| `execute_context_get` | `usize::try_from` | [807](../../src/production_tool_control.rs#L807) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `"record id does not fit this platform".into` | [808](../../src/production_tool_control.rs#L808) | receiver-type-required |
| `execute_context_get` | `event.seq` | [810](../../src/production_tool_control.rs#L810) | receiver-type-required |
| `execute_context_get` | `SupervisorOperationError::NotFound` | [811](../../src/production_tool_control.rs#L811) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `event.canonical_bytes().map_err` | [812](../../src/production_tool_control.rs#L812) | receiver-type-required |
| `execute_context_get` | `event.canonical_bytes` | [812](../../src/production_tool_control.rs#L812) | receiver-type-required |
| `execute_context_get` | `canonical.len` | [813](../../src/production_tool_control.rs#L813), [818](../../src/production_tool_control.rs#L818), [821](../../src/production_tool_control.rs#L821) | receiver-type-required |
| `execute_context_get` | `Err` | [814](../../src/production_tool_control.rs#L814) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `SupervisorOperationError::Limit` | [814](../../src/production_tool_control.rs#L814) | external-constructor-callback-or-unresolved |
| `execute_context_get` | `records.push` | [823](../../src/production_tool_control.rs#L823) | receiver-type-required |
| `execute_context_get` | `(offset + consumed < ordered.len())             .then` | [829](../../src/production_tool_control.rs#L829) | receiver-type-required |
| `execute_context_get` | `ordered.len` | [829](../../src/production_tool_control.rs#L829) | receiver-type-required |
| `execute_context_get` | `encode_cursor` | [830](../../src/production_tool_control.rs#L830) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1306) |
| `execute_context_get` | `to_ijson` | [831](../../src/production_tool_control.rs#L831) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `apply_continuation` | `response.request_id.clone` | [868](../../src/production_tool_control.rs#L868) | receiver-type-required |
| `apply_continuation` | `response.call_id.clone` | [869](../../src/production_tool_control.rs#L869) | receiver-type-required |
| `apply_continuation` | `"tool continuation result withheld by secret policy".into` | [873](../../src/production_tool_control.rs#L873) | receiver-type-required |
| `apply_continuation` | `self.scanner.scan` | [879](../../src/production_tool_control.rs#L879), [883](../../src/production_tool_control.rs#L883), [889](../../src/production_tool_control.rs#L889) | receiver-type-required |
| `apply_continuation` | `withheld` | [881](../../src/production_tool_control.rs#L881), [885](../../src/production_tool_control.rs#L885), [896](../../src/production_tool_control.rs#L896) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `IJsonValue::from` | [888](../../src/production_tool_control.rs#L888) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `error.message.clone` | [888](../../src/production_tool_control.rs#L888) | receiver-type-required |
| `apply_continuation` | `serde_json::from_value(                             serde_json::to_value(message).unwrap_or(Value::Null),                         )                         .unwrap_or_else` | [891](../../src/production_tool_control.rs#L891) | receiver-type-required |
| `apply_continuation` | `serde_json::from_value` | [891](../../src/production_tool_control.rs#L891) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `serde_json::to_value(message).unwrap_or` | [892](../../src/production_tool_control.rs#L892) | receiver-type-required |
| `apply_continuation` | `serde_json::to_value` | [892](../../src/production_tool_control.rs#L892) | external-constructor-callback-or-unresolved |
| `apply_continuation` | `"continuation error was redacted".to_owned` | [894](../../src/production_tool_control.rs#L894) | receiver-type-required |
| `apply` | `result.pending.as_mut` | [906](../../src/production_tool_control.rs#L906) | receiver-type-required |
| `apply` | `self.scanner.scan` | [909](../../src/production_tool_control.rs#L909), [918](../../src/production_tool_control.rs#L918), [928](../../src/production_tool_control.rs#L928) | receiver-type-required |
| `apply` | `policy_withheld` | [914](../../src/production_tool_control.rs#L914), [923](../../src/production_tool_control.rs#L923), [936](../../src/production_tool_control.rs#L936), [939](../../src/production_tool_control.rs#L939) | [tekes-supervisor::production_tool_control::policy_withheld](../../src/production_tool_control.rs#L944) |
| `apply` | `result.value.take` | [917](../../src/production_tool_control.rs#L917) | receiver-type-required |
| `apply` | `Some` | [920](../../src/production_tool_control.rs#L920) | external-constructor-callback-or-unresolved |
| `apply` | `result.error.as_mut` | [926](../../src/production_tool_control.rs#L926) | receiver-type-required |
| `apply` | `IJsonValue::from` | [927](../../src/production_tool_control.rs#L927) | external-constructor-callback-or-unresolved |
| `apply` | `error.message.clone` | [927](../../src/production_tool_control.rs#L927) | receiver-type-required |
| `apply` | `serde_json::from_value(                         serde_json::to_value(message).unwrap_or(Value::Null),                     )                     .unwrap_or_else` | [930](../../src/production_tool_control.rs#L930) | receiver-type-required |
| `apply` | `serde_json::from_value` | [930](../../src/production_tool_control.rs#L930) | external-constructor-callback-or-unresolved |
| `apply` | `serde_json::to_value(message).unwrap_or` | [931](../../src/production_tool_control.rs#L931) | receiver-type-required |
| `apply` | `serde_json::to_value` | [931](../../src/production_tool_control.rs#L931) | external-constructor-callback-or-unresolved |
| `apply` | `"backend error was redacted".to_owned` | [933](../../src/production_tool_control.rs#L933) | receiver-type-required |
| `policy_withheld` | `ToolControlResult::failure` | [945](../../src/production_tool_control.rs#L945) | [worker-control::durable::ToolControlResult::failure](../../../worker-control/src/durable.rs#L375) |
| `policy_withheld` | `"tool-control result withheld by secret policy".into` | [950](../../src/production_tool_control.rs#L950) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::Spawn)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .ok_or_else` | [960](../../src/production_tool_control.rs#L960) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter()         .find` | [960](../../src/production_tool_control.rs#L960) | receiver-type-required |
| `child_launch_proof` | `resolved         .projection         .events         .iter` | [960](../../src/production_tool_control.rs#L960) | receiver-type-required |
| `child_launch_proof` | `event.turn` | [966](../../src/production_tool_control.rs#L966) | receiver-type-required |
| `child_launch_proof` | `Some` | [966](../../src/production_tool_control.rs#L966), [967](../../src/production_tool_control.rs#L967), [976](../../src/production_tool_control.rs#L976), [1008](../../src/production_tool_control.rs#L1008), [1009](../../src/production_tool_control.rs#L1009), [1010](../../src/production_tool_control.rs#L1010) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `event.string_field` | [967](../../src/production_tool_control.rs#L967), [976](../../src/production_tool_control.rs#L976) | receiver-type-required |
| `child_launch_proof` | `request.call_id.as_str` | [967](../../src/production_tool_control.rs#L967), [976](../../src/production_tool_control.rs#L976) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Conflict` | [970](../../src/production_tool_control.rs#L970), [978](../../src/production_tool_control.rs#L978), [997](../../src/production_tool_control.rs#L997), [1012](../../src/production_tool_control.rs#L1012) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"task/subagent has no durable parent spawn proof".into` | [971](../../src/production_tool_control.rs#L971) | receiver-type-required |
| `child_launch_proof` | `resolved.projection.events.iter().any` | [974](../../src/production_tool_control.rs#L974) | receiver-type-required |
| `child_launch_proof` | `resolved.projection.events.iter` | [974](../../src/production_tool_control.rs#L974) | receiver-type-required |
| `child_launch_proof` | `Err` | [978](../../src/production_tool_control.rs#L978), [1012](../../src/production_tool_control.rs#L1012) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"child_result already terminalized this spawn".into` | [979](../../src/production_tool_control.rs#L979) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field("child")         .ok_or_else` | [982](../../src/production_tool_control.rs#L982) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field` | [982](../../src/production_tool_control.rs#L982), [1005](../../src/production_tool_control.rs#L1005) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Corruption` | [984](../../src/production_tool_control.rs#L984), [991](../../src/production_tool_control.rs#L991), [1003](../../src/production_tool_control.rs#L1003), [1007](../../src/production_tool_control.rs#L1007) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `"spawn.child missing".into` | [984](../../src/production_tool_control.rs#L984) | receiver-type-required |
| `child_launch_proof` | `validate_line_file_name` | [985](../../src/production_tool_control.rs#L985) | [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1384) |
| `child_launch_proof` | `resolved.thread_folder.join` | [986](../../src/production_tool_control.rs#L986) | receiver-type-required |
| `child_launch_proof` | `read_projection` | [987](../../src/production_tool_control.rs#L987) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) |
| `child_launch_proof` | `child         .events         .first()         .ok_or_else` | [988](../../src/production_tool_control.rs#L988) | receiver-type-required |
| `child_launch_proof` | `child         .events         .first` | [988](../../src/production_tool_control.rs#L988) | receiver-type-required |
| `child_launch_proof` | `"child genesis missing".into` | [991](../../src/production_tool_control.rs#L991) | receiver-type-required |
| `child_launch_proof` | `serde_json::to_value(genesis.raw())         .map_err` | [992](../../src/production_tool_control.rs#L992) | receiver-type-required |
| `child_launch_proof` | `serde_json::to_value` | [992](../../src/production_tool_control.rs#L992) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `genesis.raw` | [992](../../src/production_tool_control.rs#L992) | receiver-type-required |
| `child_launch_proof` | `SupervisorOperationError::Protocol` | [993](../../src/production_tool_control.rs#L993) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `error.to_string` | [993](../../src/production_tool_control.rs#L993) | receiver-type-required |
| `child_launch_proof` | `value         .get("parent")         .and_then(Value::as_object)         .ok_or_else` | [994](../../src/production_tool_control.rs#L994) | receiver-type-required |
| `child_launch_proof` | `value         .get("parent")         .and_then` | [994](../../src/production_tool_control.rs#L994) | receiver-type-required |
| `child_launch_proof` | `value         .get` | [994](../../src/production_tool_control.rs#L994) | receiver-type-required |
| `child_launch_proof` | `"child parent binding missing".into` | [997](../../src/production_tool_control.rs#L997) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name()         .and_then(&#124;value&#124; value.to_str())         .ok_or_else` | [998](../../src/production_tool_control.rs#L998) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name()         .and_then` | [998](../../src/production_tool_control.rs#L998) | receiver-type-required |
| `child_launch_proof` | `resolved         .line_path         .file_name` | [998](../../src/production_tool_control.rs#L998) | receiver-type-required |
| `child_launch_proof` | `value.to_str` | [1001](../../src/production_tool_control.rs#L1001) | receiver-type-required |
| `child_launch_proof` | `"parent file name is not UTF-8".into` | [1003](../../src/production_tool_control.rs#L1003) | receiver-type-required |
| `child_launch_proof` | `spawn         .string_field("spawn_id")         .ok_or_else` | [1005](../../src/production_tool_control.rs#L1005) | receiver-type-required |
| `child_launch_proof` | `"spawn_id missing".into` | [1007](../../src/production_tool_control.rs#L1007) | receiver-type-required |
| `child_launch_proof` | `parent.get("file").and_then` | [1008](../../src/production_tool_control.rs#L1008) | receiver-type-required |
| `child_launch_proof` | `parent.get` | [1008](../../src/production_tool_control.rs#L1008), [1009](../../src/production_tool_control.rs#L1009), [1010](../../src/production_tool_control.rs#L1010) | receiver-type-required |
| `child_launch_proof` | `parent.get("seq").and_then` | [1009](../../src/production_tool_control.rs#L1009) | receiver-type-required |
| `child_launch_proof` | `spawn.seq` | [1009](../../src/production_tool_control.rs#L1009), [1020](../../src/production_tool_control.rs#L1020) | receiver-type-required |
| `child_launch_proof` | `parent.get("spawn_id").and_then` | [1010](../../src/production_tool_control.rs#L1010) | receiver-type-required |
| `child_launch_proof` | `"child genesis does not match the durable parent spawn".into` | [1013](../../src/production_tool_control.rs#L1013) | receiver-type-required |
| `child_launch_proof` | `Ok` | [1016](../../src/production_tool_control.rs#L1016) | external-constructor-callback-or-unresolved |
| `child_launch_proof` | `request.session.clone` | [1017](../../src/production_tool_control.rs#L1017) | receiver-type-required |
| `child_launch_proof` | `request.thread.clone` | [1018](../../src/production_tool_control.rs#L1018) | receiver-type-required |
| `child_launch_proof` | `parent_file.to_owned` | [1019](../../src/production_tool_control.rs#L1019) | receiver-type-required |
| `child_launch_proof` | `genesis_line(&child)?.to_owned` | [1021](../../src/production_tool_control.rs#L1021) | receiver-type-required |
| `child_launch_proof` | `genesis_line` | [1021](../../src/production_tool_control.rs#L1021) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1208) |
| `child_launch_proof` | `child_file.to_owned` | [1022](../../src/production_tool_control.rs#L1022) | receiver-type-required |
| `child_launch_proof` | `spawn_id.to_owned` | [1023](../../src/production_tool_control.rs#L1023) | receiver-type-required |
| `child_launch_proof` | `request.call_id.clone` | [1024](../../src/production_tool_control.rs#L1024) | receiver-type-required |
| `parent_report_proof` | `resolved.projection.events.first().ok_or_else` | [1032](../../src/production_tool_control.rs#L1032) | receiver-type-required |
| `parent_report_proof` | `resolved.projection.events.first` | [1032](../../src/production_tool_control.rs#L1032) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Corruption` | [1033](../../src/production_tool_control.rs#L1033), [1046](../../src/production_tool_control.rs#L1046), [1051](../../src/production_tool_control.rs#L1051), [1055](../../src/production_tool_control.rs#L1055), [1060](../../src/production_tool_control.rs#L1060), [1069](../../src/production_tool_control.rs#L1069) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"reporting line has no genesis".into` | [1033](../../src/production_tool_control.rs#L1033) | receiver-type-required |
| `parent_report_proof` | `serde_json::to_value(child_genesis.raw())         .map_err` | [1035](../../src/production_tool_control.rs#L1035) | receiver-type-required |
| `parent_report_proof` | `serde_json::to_value` | [1035](../../src/production_tool_control.rs#L1035) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `child_genesis.raw` | [1035](../../src/production_tool_control.rs#L1035) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Protocol` | [1036](../../src/production_tool_control.rs#L1036) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `error.to_string` | [1036](../../src/production_tool_control.rs#L1036) | receiver-type-required |
| `parent_report_proof` | `value         .get("parent")         .and_then(Value::as_object)         .ok_or_else` | [1037](../../src/production_tool_control.rs#L1037) | receiver-type-required |
| `parent_report_proof` | `value         .get("parent")         .and_then` | [1037](../../src/production_tool_control.rs#L1037) | receiver-type-required |
| `parent_report_proof` | `value         .get` | [1037](../../src/production_tool_control.rs#L1037) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Denied` | [1041](../../src/production_tool_control.rs#L1041) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"report is valid only in a child line".into` | [1041](../../src/production_tool_control.rs#L1041) | receiver-type-required |
| `parent_report_proof` | `parent         .get("file")         .and_then(Value::as_str)         .ok_or_else` | [1043](../../src/production_tool_control.rs#L1043) | receiver-type-required |
| `parent_report_proof` | `parent         .get("file")         .and_then` | [1043](../../src/production_tool_control.rs#L1043) | receiver-type-required |
| `parent_report_proof` | `parent         .get` | [1043](../../src/production_tool_control.rs#L1043), [1048](../../src/production_tool_control.rs#L1048), [1052](../../src/production_tool_control.rs#L1052) | receiver-type-required |
| `parent_report_proof` | `"parent.file missing".into` | [1046](../../src/production_tool_control.rs#L1046) | receiver-type-required |
| `parent_report_proof` | `validate_line_file_name` | [1047](../../src/production_tool_control.rs#L1047) | [tekes-supervisor::production_tool_control::validate_line_file_name](../../src/production_tool_control.rs#L1384) |
| `parent_report_proof` | `parent         .get("seq")         .and_then(Value::as_u64)         .ok_or_else` | [1048](../../src/production_tool_control.rs#L1048) | receiver-type-required |
| `parent_report_proof` | `parent         .get("seq")         .and_then` | [1048](../../src/production_tool_control.rs#L1048) | receiver-type-required |
| `parent_report_proof` | `"parent.seq missing".into` | [1051](../../src/production_tool_control.rs#L1051) | receiver-type-required |
| `parent_report_proof` | `parent         .get("spawn_id")         .and_then(Value::as_str)         .ok_or_else` | [1052](../../src/production_tool_control.rs#L1052) | receiver-type-required |
| `parent_report_proof` | `parent         .get("spawn_id")         .and_then` | [1052](../../src/production_tool_control.rs#L1052) | receiver-type-required |
| `parent_report_proof` | `"parent.spawn_id missing".into` | [1055](../../src/production_tool_control.rs#L1055) | receiver-type-required |
| `parent_report_proof` | `read_projection` | [1056](../../src/production_tool_control.rs#L1056) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) |
| `parent_report_proof` | `resolved.thread_folder.join` | [1056](../../src/production_tool_control.rs#L1056) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get(usize::try_from(parent_seq - 1).map_err(&#124;_&#124; {             SupervisorOperationError::Corruption("parent seq does not fit this platform".into())         })?)         .filter(&#124;event&#124; event.seq() == parent_seq && matches!(event.kind(), EventKind::Spawn))         .ok_or_else` | [1057](../../src/production_tool_control.rs#L1057) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get(usize::try_from(parent_seq - 1).map_err(&#124;_&#124; {             SupervisorOperationError::Corruption("parent seq does not fit this platform".into())         })?)         .filter` | [1057](../../src/production_tool_control.rs#L1057) | receiver-type-required |
| `parent_report_proof` | `parent_projection         .events         .get` | [1057](../../src/production_tool_control.rs#L1057) | receiver-type-required |
| `parent_report_proof` | `usize::try_from(parent_seq - 1).map_err` | [1059](../../src/production_tool_control.rs#L1059) | receiver-type-required |
| `parent_report_proof` | `usize::try_from` | [1059](../../src/production_tool_control.rs#L1059) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"parent seq does not fit this platform".into` | [1060](../../src/production_tool_control.rs#L1060) | receiver-type-required |
| `parent_report_proof` | `event.seq` | [1062](../../src/production_tool_control.rs#L1062) | receiver-type-required |
| `parent_report_proof` | `SupervisorOperationError::Conflict` | [1063](../../src/production_tool_control.rs#L1063), [1074](../../src/production_tool_control.rs#L1074) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"parent spawn proof missing".into` | [1063](../../src/production_tool_control.rs#L1063) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name()         .and_then(&#124;value&#124; value.to_str())         .ok_or_else` | [1064](../../src/production_tool_control.rs#L1064) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name()         .and_then` | [1064](../../src/production_tool_control.rs#L1064) | receiver-type-required |
| `parent_report_proof` | `resolved         .line_path         .file_name` | [1064](../../src/production_tool_control.rs#L1064) | receiver-type-required |
| `parent_report_proof` | `value.to_str` | [1067](../../src/production_tool_control.rs#L1067) | receiver-type-required |
| `parent_report_proof` | `"child file name is not UTF-8".into` | [1069](../../src/production_tool_control.rs#L1069) | receiver-type-required |
| `parent_report_proof` | `spawn.string_field` | [1071](../../src/production_tool_control.rs#L1071), [1072](../../src/production_tool_control.rs#L1072) | receiver-type-required |
| `parent_report_proof` | `Some` | [1071](../../src/production_tool_control.rs#L1071), [1072](../../src/production_tool_control.rs#L1072) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `Err` | [1074](../../src/production_tool_control.rs#L1074) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `"reporting child does not match parent spawn".into` | [1075](../../src/production_tool_control.rs#L1075) | receiver-type-required |
| `parent_report_proof` | `Ok` | [1078](../../src/production_tool_control.rs#L1078) | external-constructor-callback-or-unresolved |
| `parent_report_proof` | `request.session.clone` | [1079](../../src/production_tool_control.rs#L1079) | receiver-type-required |
| `parent_report_proof` | `request.thread.clone` | [1080](../../src/production_tool_control.rs#L1080) | receiver-type-required |
| `parent_report_proof` | `child_file.to_owned` | [1081](../../src/production_tool_control.rs#L1081) | receiver-type-required |
| `parent_report_proof` | `genesis_line(&parent_projection)?.to_owned` | [1082](../../src/production_tool_control.rs#L1082) | receiver-type-required |
| `parent_report_proof` | `genesis_line` | [1082](../../src/production_tool_control.rs#L1082) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1208) |
| `parent_report_proof` | `parent_file.to_owned` | [1083](../../src/production_tool_control.rs#L1083) | receiver-type-required |
| `parent_report_proof` | `spawn_id.to_owned` | [1085](../../src/production_tool_control.rs#L1085) | receiver-type-required |
| `parent_report_proof` | `request.call_id.clone` | [1086](../../src/production_tool_control.rs#L1086) | receiver-type-required |
| `session_lines` | `Vec::new` | [1093](../../src/production_tool_control.rs#L1093) | external-constructor-callback-or-unresolved |
| `session_lines` | `fs::read_dir(folder).map_err` | [1094](../../src/production_tool_control.rs#L1094) | receiver-type-required |
| `session_lines` | `fs::read_dir` | [1094](../../src/production_tool_control.rs#L1094) | external-constructor-callback-or-unresolved |
| `session_lines` | `entry.map_err` | [1095](../../src/production_tool_control.rs#L1095) | receiver-type-required |
| `session_lines` | `entry.file_type().map_err(map_io)?.is_file` | [1096](../../src/production_tool_control.rs#L1096) | receiver-type-required |
| `session_lines` | `entry.file_type().map_err` | [1096](../../src/production_tool_control.rs#L1096) | receiver-type-required |
| `session_lines` | `entry.file_type` | [1096](../../src/production_tool_control.rs#L1096) | receiver-type-required |
| `session_lines` | `entry.path` | [1099](../../src/production_tool_control.rs#L1099) | receiver-type-required |
| `session_lines` | `path.extension().and_then` | [1100](../../src/production_tool_control.rs#L1100) | receiver-type-required |
| `session_lines` | `path.extension` | [1100](../../src/production_tool_control.rs#L1100) | receiver-type-required |
| `session_lines` | `value.to_str` | [1100](../../src/production_tool_control.rs#L1100), [1101](../../src/production_tool_control.rs#L1101) | receiver-type-required |
| `session_lines` | `Some` | [1100](../../src/production_tool_control.rs#L1100), [1101](../../src/production_tool_control.rs#L1101) | external-constructor-callback-or-unresolved |
| `session_lines` | `path.file_name().and_then` | [1101](../../src/production_tool_control.rs#L1101) | receiver-type-required |
| `session_lines` | `path.file_name` | [1101](../../src/production_tool_control.rs#L1101) | receiver-type-required |
| `session_lines` | `read_projection` | [1102](../../src/production_tool_control.rs#L1102) | [tekes-supervisor::production_tool_control::read_projection](../../src/production_tool_control.rs#L1197) |
| `session_lines` | `lines.push` | [1104](../../src/production_tool_control.rs#L1104) | receiver-type-required |
| `session_lines` | `lines.sort_by` | [1107](../../src/production_tool_control.rs#L1107) | receiver-type-required |
| `session_lines` | `left.0.as_os_str().cmp` | [1107](../../src/production_tool_control.rs#L1107) | receiver-type-required |
| `session_lines` | `left.0.as_os_str` | [1107](../../src/production_tool_control.rs#L1107) | receiver-type-required |
| `session_lines` | `right.0.as_os_str` | [1107](../../src/production_tool_control.rs#L1107) | receiver-type-required |
| `session_lines` | `Ok` | [1108](../../src/production_tool_control.rs#L1108) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `visible_sessions` | [1116](../../src/production_tool_control.rs#L1116) | [tekes-supervisor::production_tool_control::visible_sessions](../../src/production_tool_control.rs#L1132) |
| `resolve_visible_line` | `session_folder` | [1117](../../src/production_tool_control.rs#L1117) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) |
| `resolve_visible_line` | `session_lines` | [1118](../../src/production_tool_control.rs#L1118) | [tekes-supervisor::production_tool_control::session_lines](../../src/production_tool_control.rs#L1090) |
| `resolve_visible_line` | `genesis_line` | [1119](../../src/production_tool_control.rs#L1119) | [tekes-supervisor::production_tool_control::genesis_line](../../src/production_tool_control.rs#L1208) |
| `resolve_visible_line` | `found.is_some` | [1120](../../src/production_tool_control.rs#L1120) | receiver-type-required |
| `resolve_visible_line` | `Err` | [1121](../../src/production_tool_control.rs#L1121) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `SupervisorOperationError::Corruption` | [1121](../../src/production_tool_control.rs#L1121) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `"line identity is duplicated across sessions".into` | [1122](../../src/production_tool_control.rs#L1122) | receiver-type-required |
| `resolve_visible_line` | `Some` | [1125](../../src/production_tool_control.rs#L1125) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `found.ok_or_else` | [1129](../../src/production_tool_control.rs#L1129) | receiver-type-required |
| `resolve_visible_line` | `SupervisorOperationError::NotFound` | [1129](../../src/production_tool_control.rs#L1129) | external-constructor-callback-or-unresolved |
| `resolve_visible_line` | `"line not found".into` | [1129](../../src/production_tool_control.rs#L1129) | receiver-type-required |
| `visible_sessions` | `BTreeSet::new` | [1133](../../src/production_tool_control.rs#L1133) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `fs::read_dir(root.join(area)).map_err` | [1135](../../src/production_tool_control.rs#L1135) | receiver-type-required |
| `visible_sessions` | `fs::read_dir` | [1135](../../src/production_tool_control.rs#L1135) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `root.join` | [1135](../../src/production_tool_control.rs#L1135) | receiver-type-required |
| `visible_sessions` | `entry.map_err` | [1136](../../src/production_tool_control.rs#L1136) | receiver-type-required |
| `visible_sessions` | `entry.file_type().map_err(map_io)?.is_dir` | [1137](../../src/production_tool_control.rs#L1137) | receiver-type-required |
| `visible_sessions` | `entry.file_type().map_err` | [1137](../../src/production_tool_control.rs#L1137) | receiver-type-required |
| `visible_sessions` | `entry.file_type` | [1137](../../src/production_tool_control.rs#L1137) | receiver-type-required |
| `visible_sessions` | `entry.file_name().to_string_lossy().into_owned` | [1140](../../src/production_tool_control.rs#L1140) | receiver-type-required |
| `visible_sessions` | `entry.file_name().to_string_lossy` | [1140](../../src/production_tool_control.rs#L1140) | receiver-type-required |
| `visible_sessions` | `entry.file_name` | [1140](../../src/production_tool_control.rs#L1140) | receiver-type-required |
| `visible_sessions` | `validate_session_folder` | [1141](../../src/production_tool_control.rs#L1141) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1162) |
| `visible_sessions` | `entry.path` | [1141](../../src/production_tool_control.rs#L1141) | receiver-type-required |
| `visible_sessions` | `sessions.insert` | [1142](../../src/production_tool_control.rs#L1142) | receiver-type-required |
| `visible_sessions` | `Err` | [1143](../../src/production_tool_control.rs#L1143) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `SupervisorOperationError::Corruption` | [1143](../../src/production_tool_control.rs#L1143) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `"session exists in active and archive catalogs".into` | [1144](../../src/production_tool_control.rs#L1144) | receiver-type-required |
| `visible_sessions` | `Ok` | [1149](../../src/production_tool_control.rs#L1149) | external-constructor-callback-or-unresolved |
| `visible_sessions` | `sessions.into_iter().collect` | [1149](../../src/production_tool_control.rs#L1149) | receiver-type-required |
| `visible_sessions` | `sessions.into_iter` | [1149](../../src/production_tool_control.rs#L1149) | receiver-type-required |
| `require_active_session` | `session_folder` | [1153](../../src/production_tool_control.rs#L1153) | [tekes-supervisor::production_tool_control::session_folder](../../src/production_tool_control.rs#L1182) |
| `require_active_session` | `Err` | [1155](../../src/production_tool_control.rs#L1155) | external-constructor-callback-or-unresolved |
| `require_active_session` | `SupervisorOperationError::NotFound` | [1155](../../src/production_tool_control.rs#L1155) | external-constructor-callback-or-unresolved |
| `require_active_session` | `"active thread not found".into` | [1156](../../src/production_tool_control.rs#L1156) | receiver-type-required |
| `require_active_session` | `Ok` | [1159](../../src/production_tool_control.rs#L1159) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `endpoint::validate_session_id(session)         .map_err` | [1163](../../src/production_tool_control.rs#L1163) | receiver-type-required |
| `validate_session_folder` | `endpoint::validate_session_id` | [1163](../../src/production_tool_control.rs#L1163) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `validate_session_folder` | `SupervisorOperationError::Invalid` | [1164](../../src/production_tool_control.rs#L1164) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"invalid session identity".into` | [1164](../../src/production_tool_control.rs#L1164) | receiver-type-required |
| `validate_session_folder` | `fs::symlink_metadata(folder).map_err` | [1165](../../src/production_tool_control.rs#L1165) | receiver-type-required |
| `validate_session_folder` | `fs::symlink_metadata` | [1165](../../src/production_tool_control.rs#L1165) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `metadata.is_dir` | [1166](../../src/production_tool_control.rs#L1166) | receiver-type-required |
| `validate_session_folder` | `metadata.file_type().is_symlink` | [1166](../../src/production_tool_control.rs#L1166) | receiver-type-required |
| `validate_session_folder` | `metadata.file_type` | [1166](../../src/production_tool_control.rs#L1166) | receiver-type-required |
| `validate_session_folder` | `Err` | [1167](../../src/production_tool_control.rs#L1167), [1175](../../src/production_tool_control.rs#L1175) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `SupervisorOperationError::NotFound` | [1167](../../src/production_tool_control.rs#L1167) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"thread not found".into` | [1168](../../src/production_tool_control.rs#L1168) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis(folder.join("main.jsonl"))         .map_err(map_store)?         .ok_or_else` | [1171](../../src/production_tool_control.rs#L1171) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis(folder.join("main.jsonl"))         .map_err` | [1171](../../src/production_tool_control.rs#L1171) | receiver-type-required |
| `validate_session_folder` | `store::read_genesis` | [1171](../../src/production_tool_control.rs#L1171) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `validate_session_folder` | `folder.join` | [1171](../../src/production_tool_control.rs#L1171) | receiver-type-required |
| `validate_session_folder` | `SupervisorOperationError::Corruption` | [1173](../../src/production_tool_control.rs#L1173), [1175](../../src/production_tool_control.rs#L1175) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"genesis missing".into` | [1173](../../src/production_tool_control.rs#L1173) | receiver-type-required |
| `validate_session_folder` | `genesis.string_field` | [1174](../../src/production_tool_control.rs#L1174) | receiver-type-required |
| `validate_session_folder` | `Some` | [1174](../../src/production_tool_control.rs#L1174) | external-constructor-callback-or-unresolved |
| `validate_session_folder` | `"folder/genesis identity mismatch".into` | [1176](../../src/production_tool_control.rs#L1176) | receiver-type-required |
| `validate_session_folder` | `Ok` | [1179](../../src/production_tool_control.rs#L1179) | external-constructor-callback-or-unresolved |
| `session_folder` | `endpoint::validate_session_id(session)         .map_err` | [1183](../../src/production_tool_control.rs#L1183) | receiver-type-required |
| `session_folder` | `endpoint::validate_session_id` | [1183](../../src/production_tool_control.rs#L1183) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `session_folder` | `SupervisorOperationError::Invalid` | [1184](../../src/production_tool_control.rs#L1184) | external-constructor-callback-or-unresolved |
| `session_folder` | `"invalid session identity".into` | [1184](../../src/production_tool_control.rs#L1184) | receiver-type-required |
| `session_folder` | `root.join(area).join` | [1186](../../src/production_tool_control.rs#L1186) | receiver-type-required |
| `session_folder` | `root.join` | [1186](../../src/production_tool_control.rs#L1186) | receiver-type-required |
| `session_folder` | `fs::symlink_metadata(&folder).is_ok` | [1187](../../src/production_tool_control.rs#L1187) | receiver-type-required |
| `session_folder` | `fs::symlink_metadata` | [1187](../../src/production_tool_control.rs#L1187) | external-constructor-callback-or-unresolved |
| `session_folder` | `validate_session_folder` | [1188](../../src/production_tool_control.rs#L1188) | [tekes-supervisor::production_tool_control::validate_session_folder](../../src/production_tool_control.rs#L1162) |
| `session_folder` | `Ok` | [1189](../../src/production_tool_control.rs#L1189) | external-constructor-callback-or-unresolved |
| `session_folder` | `Err` | [1192](../../src/production_tool_control.rs#L1192) | external-constructor-callback-or-unresolved |
| `session_folder` | `SupervisorOperationError::NotFound` | [1192](../../src/production_tool_control.rs#L1192) | external-constructor-callback-or-unresolved |
| `session_folder` | `"thread not found".into` | [1193](../../src/production_tool_control.rs#L1193) | receiver-type-required |
| `read_projection` | `fs::read(path).map_err` | [1198](../../src/production_tool_control.rs#L1198) | receiver-type-required |
| `read_projection` | `fs::read` | [1198](../../src/production_tool_control.rs#L1198) | external-constructor-callback-or-unresolved |
| `read_projection` | `scan_valid_prefix` | [1199](../../src/production_tool_control.rs#L1199) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `read_projection` | `scan.projection.ok_or_else` | [1200](../../src/production_tool_control.rs#L1200) | receiver-type-required |
| `read_projection` | `SupervisorOperationError::Corruption` | [1201](../../src/production_tool_control.rs#L1201) | external-constructor-callback-or-unresolved |
| `genesis_line` | `projection         .events         .first()         .and_then(&#124;event&#124; event.string_field("thread"))         .ok_or_else` | [1209](../../src/production_tool_control.rs#L1209) | receiver-type-required |
| `genesis_line` | `projection         .events         .first()         .and_then` | [1209](../../src/production_tool_control.rs#L1209) | receiver-type-required |
| `genesis_line` | `projection         .events         .first` | [1209](../../src/production_tool_control.rs#L1209) | receiver-type-required |
| `genesis_line` | `event.string_field` | [1212](../../src/production_tool_control.rs#L1212) | receiver-type-required |
| `genesis_line` | `SupervisorOperationError::Corruption` | [1213](../../src/production_tool_control.rs#L1213) | external-constructor-callback-or-unresolved |
| `genesis_line` | `"genesis thread missing".into` | [1213](../../src/production_tool_control.rs#L1213) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::ToolCall)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .and_then(&#124;event&#124; event.string_field("ts"))         .ok_or_else` | [1220](../../src/production_tool_control.rs#L1220) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find(&#124;event&#124; {             matches!(event.kind(), EventKind::ToolCall)                 && event.turn() == Some(request.turn)                 && event.string_field("call") == Some(request.call_id.as_str())         })         .and_then` | [1220](../../src/production_tool_control.rs#L1220) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter()         .find` | [1220](../../src/production_tool_control.rs#L1220) | receiver-type-required |
| `paired_tool_timestamp` | `projection         .events         .iter` | [1220](../../src/production_tool_control.rs#L1220) | receiver-type-required |
| `paired_tool_timestamp` | `event.turn` | [1225](../../src/production_tool_control.rs#L1225) | receiver-type-required |
| `paired_tool_timestamp` | `Some` | [1225](../../src/production_tool_control.rs#L1225), [1226](../../src/production_tool_control.rs#L1226) | external-constructor-callback-or-unresolved |
| `paired_tool_timestamp` | `event.string_field` | [1226](../../src/production_tool_control.rs#L1226), [1228](../../src/production_tool_control.rs#L1228) | receiver-type-required |
| `paired_tool_timestamp` | `request.call_id.as_str` | [1226](../../src/production_tool_control.rs#L1226) | receiver-type-required |
| `paired_tool_timestamp` | `SupervisorOperationError::Corruption` | [1229](../../src/production_tool_control.rs#L1229) | external-constructor-callback-or-unresolved |
| `paired_tool_timestamp` | `"tool_call timestamp missing".into` | [1229](../../src/production_tool_control.rs#L1229) | receiver-type-required |
| `event_record` | `event.canonical_bytes().map_err` | [1233](../../src/production_tool_control.rs#L1233) | receiver-type-required |
| `event_record` | `event.canonical_bytes` | [1233](../../src/production_tool_control.rs#L1233) | receiver-type-required |
| `event_record` | `serde_json::from_slice(&canonical)         .map_err` | [1234](../../src/production_tool_control.rs#L1234) | receiver-type-required |
| `event_record` | `serde_json::from_slice` | [1234](../../src/production_tool_control.rs#L1234) | external-constructor-callback-or-unresolved |
| `event_record` | `SupervisorOperationError::Protocol` | [1235](../../src/production_tool_control.rs#L1235) | external-constructor-callback-or-unresolved |
| `event_record` | `error.to_string` | [1235](../../src/production_tool_control.rs#L1235) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [1239](../../src/production_tool_control.rs#L1239) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then(Value::as_str)         .filter` | [1239](../../src/production_tool_control.rs#L1239) | receiver-type-required |
| `required_string` | `value         .get(field)         .and_then` | [1239](../../src/production_tool_control.rs#L1239) | receiver-type-required |
| `required_string` | `value         .get` | [1239](../../src/production_tool_control.rs#L1239) | receiver-type-required |
| `required_string` | `value.is_empty` | [1242](../../src/production_tool_control.rs#L1242) | receiver-type-required |
| `required_string` | `SupervisorOperationError::Invalid` | [1243](../../src/production_tool_control.rs#L1243) | external-constructor-callback-or-unresolved |
| `string_array` | `value.get(field).map_or` | [1247](../../src/production_tool_control.rs#L1247) | receiver-type-required |
| `string_array` | `value.get` | [1247](../../src/production_tool_control.rs#L1247) | receiver-type-required |
| `string_array` | `Ok` | [1247](../../src/production_tool_control.rs#L1247) | external-constructor-callback-or-unresolved |
| `string_array` | `Vec::new` | [1247](../../src/production_tool_control.rs#L1247) | external-constructor-callback-or-unresolved |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter()             .map(&#124;value&#124; {                 value.as_str().map(ToOwned::to_owned).ok_or_else(&#124;&#124; {                     SupervisorOperationError::Invalid(format!("{field} contains a non-string"))                 })             })             .collect` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter()             .map` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else(&#124;&#124; SupervisorOperationError::Invalid(format!("{field} must be an array")))?             .iter` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `string_array` | `value             .as_array()             .ok_or_else` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `string_array` | `value             .as_array` | [1248](../../src/production_tool_control.rs#L1248) | receiver-type-required |
| `string_array` | `SupervisorOperationError::Invalid` | [1250](../../src/production_tool_control.rs#L1250), [1254](../../src/production_tool_control.rs#L1254) | external-constructor-callback-or-unresolved |
| `string_array` | `value.as_str().map(ToOwned::to_owned).ok_or_else` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `string_array` | `value.as_str().map` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `string_array` | `value.as_str` | [1253](../../src/production_tool_control.rs#L1253) | receiver-type-required |
| `job_record_outcome` | `to_ijson` | [1265](../../src/production_tool_control.rs#L1265) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `job_record_outcome` | `Err` | [1266](../../src/production_tool_control.rs#L1266), [1267](../../src/production_tool_control.rs#L1267), [1269](../../src/production_tool_control.rs#L1269) | external-constructor-callback-or-unresolved |
| `job_record_outcome` | `map_backend` | [1266](../../src/production_tool_control.rs#L1266) | [tekes-supervisor::production_tool_control::map_backend](../../src/production_tool_control.rs#L1274) |
| `job_record_outcome` | `SupervisorOperationError::Denied` | [1267](../../src/production_tool_control.rs#L1267) | external-constructor-callback-or-unresolved |
| `job_record_outcome` | `SupervisorOperationError::Unavailable` | [1269](../../src/production_tool_control.rs#L1269) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Invalid` | [1276](../../src/production_tool_control.rs#L1276) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Denied` | [1277](../../src/production_tool_control.rs#L1277) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Unavailable` | [1278](../../src/production_tool_control.rs#L1278), [1283](../../src/production_tool_control.rs#L1283), [1285](../../src/production_tool_control.rs#L1285), [1287](../../src/production_tool_control.rs#L1287), [1289](../../src/production_tool_control.rs#L1289) | external-constructor-callback-or-unresolved |
| `map_backend` | `"job cancelled".into` | [1278](../../src/production_tool_control.rs#L1278) | receiver-type-required |
| `map_backend` | `SupervisorOperationError::Timeout` | [1279](../../src/production_tool_control.rs#L1279) | external-constructor-callback-or-unresolved |
| `map_backend` | `"job timed out".into` | [1279](../../src/production_tool_control.rs#L1279) | receiver-type-required |
| `map_backend` | `SupervisorOperationError::Limit` | [1280](../../src/production_tool_control.rs#L1280) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::NotFound` | [1281](../../src/production_tool_control.rs#L1281) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Conflict` | [1282](../../src/production_tool_control.rs#L1282) | external-constructor-callback-or-unresolved |
| `map_backend` | `SupervisorOperationError::Protocol` | [1288](../../src/production_tool_control.rs#L1288) | external-constructor-callback-or-unresolved |
| `argument_limit` | `value         .get("limit")         .and_then(Value::as_u64)         .map_or` | [1294](../../src/production_tool_control.rs#L1294) | receiver-type-required |
| `argument_limit` | `value         .get("limit")         .and_then` | [1294](../../src/production_tool_control.rs#L1294) | receiver-type-required |
| `argument_limit` | `value         .get` | [1294](../../src/production_tool_control.rs#L1294) | receiver-type-required |
| `argument_limit` | `Err` | [1299](../../src/production_tool_control.rs#L1299) | external-constructor-callback-or-unresolved |
| `argument_limit` | `SupervisorOperationError::Limit` | [1299](../../src/production_tool_control.rs#L1299) | external-constructor-callback-or-unresolved |
| `argument_limit` | `Ok` | [1303](../../src/production_tool_control.rs#L1303) | external-constructor-callback-or-unresolved |
| `encode_cursor` | `Sha256::digest` | [1307](../../src/production_tool_control.rs#L1307) | external-constructor-callback-or-unresolved |
| `encode_cursor` | `identity.as_bytes` | [1307](../../src/production_tool_control.rs#L1307) | receiver-type-required |
| `decode_cursor` | `value         .and_then(Value::as_str)         .filter` | [1312](../../src/production_tool_control.rs#L1312) | receiver-type-required |
| `decode_cursor` | `value         .and_then` | [1312](../../src/production_tool_control.rs#L1312) | receiver-type-required |
| `decode_cursor` | `value.is_empty` | [1314](../../src/production_tool_control.rs#L1314) | receiver-type-required |
| `decode_cursor` | `Ok` | [1316](../../src/production_tool_control.rs#L1316) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `encode_cursor` | [1318](../../src/production_tool_control.rs#L1318) | [tekes-supervisor::production_tool_control::encode_cursor](../../src/production_tool_control.rs#L1306) |
| `decode_cursor` | `cursor_zero         .rsplit_once('-')         .map(&#124;(prefix, _)&#124; prefix)         .expect` | [1319](../../src/production_tool_control.rs#L1319) | receiver-type-required |
| `decode_cursor` | `cursor_zero         .rsplit_once('-')         .map` | [1319](../../src/production_tool_control.rs#L1319) | receiver-type-required |
| `decode_cursor` | `cursor_zero         .rsplit_once` | [1319](../../src/production_tool_control.rs#L1319) | receiver-type-required |
| `decode_cursor` | `cursor         .rsplit_once('-')         .ok_or_else` | [1323](../../src/production_tool_control.rs#L1323) | receiver-type-required |
| `decode_cursor` | `cursor         .rsplit_once` | [1323](../../src/production_tool_control.rs#L1323) | receiver-type-required |
| `decode_cursor` | `SupervisorOperationError::Invalid` | [1325](../../src/production_tool_control.rs#L1325), [1333](../../src/production_tool_control.rs#L1333) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `"invalid cursor".into` | [1325](../../src/production_tool_control.rs#L1325) | receiver-type-required |
| `decode_cursor` | `Err` | [1327](../../src/production_tool_control.rs#L1327) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `SupervisorOperationError::Conflict` | [1327](../../src/production_tool_control.rs#L1327) | external-constructor-callback-or-unresolved |
| `decode_cursor` | `"cursor does not belong to this query".into` | [1328](../../src/production_tool_control.rs#L1328) | receiver-type-required |
| `decode_cursor` | `offset         .parse()         .map_err` | [1331](../../src/production_tool_control.rs#L1331) | receiver-type-required |
| `decode_cursor` | `offset         .parse` | [1331](../../src/production_tool_control.rs#L1331) | receiver-type-required |
| `decode_cursor` | `"invalid cursor offset".into` | [1333](../../src/production_tool_control.rs#L1333) | receiver-type-required |
| `context_start_genesis` | `serde_json::from_value(source.get("execution").cloned().ok_or_else(&#124;&#124; {             SupervisorOperationError::Corruption("caller execution missing".into())         })?)         .map_err` | [1344](../../src/production_tool_control.rs#L1344) | receiver-type-required |
| `context_start_genesis` | `serde_json::from_value` | [1344](../../src/production_tool_control.rs#L1344) | external-constructor-callback-or-unresolved |
| `context_start_genesis` | `source.get("execution").cloned().ok_or_else` | [1344](../../src/production_tool_control.rs#L1344) | receiver-type-required |
| `context_start_genesis` | `source.get("execution").cloned` | [1344](../../src/production_tool_control.rs#L1344) | receiver-type-required |
| `context_start_genesis` | `source.get` | [1344](../../src/production_tool_control.rs#L1344), [1357](../../src/production_tool_control.rs#L1357) | receiver-type-required |
| `context_start_genesis` | `SupervisorOperationError::Corruption` | [1345](../../src/production_tool_control.rs#L1345), [1347](../../src/production_tool_control.rs#L1347), [1350](../../src/production_tool_control.rs#L1350) | external-constructor-callback-or-unresolved |
| `context_start_genesis` | `"caller execution missing".into` | [1345](../../src/production_tool_control.rs#L1345) | receiver-type-required |
| `context_start_genesis` | `"invalid caller execution".into` | [1347](../../src/production_tool_control.rs#L1347), [1350](../../src/production_tool_control.rs#L1350) | receiver-type-required |
| `context_start_genesis` | `execution         .validate()         .map_err` | [1348](../../src/production_tool_control.rs#L1348) | receiver-type-required |
| `context_start_genesis` | `execution         .validate` | [1348](../../src/production_tool_control.rs#L1348) | receiver-type-required |
| `context_start_genesis` | `value.clone` | [1358](../../src/production_tool_control.rs#L1358) | receiver-type-required |
| `context_start_genesis` | `Event::from_value(to_ijson(&event)?).map_err` | [1361](../../src/production_tool_control.rs#L1361) | receiver-type-required |
| `context_start_genesis` | `Event::from_value` | [1361](../../src/production_tool_control.rs#L1361) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `context_start_genesis` | `to_ijson` | [1361](../../src/production_tool_control.rs#L1361) | [tekes-supervisor::production_tool_control::to_ijson](../../src/production_tool_control.rs#L1408) |
| `deterministic_uuid` | `request_id.as_bytes()[..32].to_vec` | [1365](../../src/production_tool_control.rs#L1365) | receiver-type-required |
| `deterministic_uuid` | `request_id.as_bytes` | [1365](../../src/production_tool_control.rs#L1365) | receiver-type-required |
| `deterministic_uuid` | `String::from_utf8(bytes).expect` | [1373](../../src/production_tool_control.rs#L1373) | receiver-type-required |
| `deterministic_uuid` | `String::from_utf8` | [1373](../../src/production_tool_control.rs#L1373) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `Path::new` | [1385](../../src/production_tool_control.rs#L1385) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `path.components().count` | [1386](../../src/production_tool_control.rs#L1386) | receiver-type-required |
| `validate_line_file_name` | `path.components` | [1386](../../src/production_tool_control.rs#L1386) | receiver-type-required |
| `validate_line_file_name` | `path.extension().and_then` | [1388](../../src/production_tool_control.rs#L1388) | receiver-type-required |
| `validate_line_file_name` | `path.extension` | [1388](../../src/production_tool_control.rs#L1388) | receiver-type-required |
| `validate_line_file_name` | `value.to_str` | [1388](../../src/production_tool_control.rs#L1388) | receiver-type-required |
| `validate_line_file_name` | `Some` | [1388](../../src/production_tool_control.rs#L1388) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `Err` | [1390](../../src/production_tool_control.rs#L1390) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `SupervisorOperationError::Denied` | [1390](../../src/production_tool_control.rs#L1390) | external-constructor-callback-or-unresolved |
| `validate_line_file_name` | `"line file escapes the session folder".into` | [1391](../../src/production_tool_control.rs#L1391) | receiver-type-required |
| `validate_line_file_name` | `Ok` | [1394](../../src/production_tool_control.rs#L1394) | external-constructor-callback-or-unresolved |
| `bounded_snippet` | `value.len` | [1398](../../src/production_tool_control.rs#L1398) | receiver-type-required |
| `bounded_snippet` | `value.to_owned` | [1399](../../src/production_tool_control.rs#L1399) | receiver-type-required |
| `bounded_snippet` | `value.is_char_boundary` | [1402](../../src/production_tool_control.rs#L1402) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse(         &serde_json::to_vec(value)             .map_err(&#124;error&#124; SupervisorOperationError::Protocol(error.to_string()))?,     )     .map_err` | [1409](../../src/production_tool_control.rs#L1409) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [1409](../../src/production_tool_control.rs#L1409) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value)             .map_err` | [1410](../../src/production_tool_control.rs#L1410) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [1410](../../src/production_tool_control.rs#L1410) | external-constructor-callback-or-unresolved |
| `to_ijson` | `SupervisorOperationError::Protocol` | [1411](../../src/production_tool_control.rs#L1411) | external-constructor-callback-or-unresolved |
| `to_ijson` | `error.to_string` | [1411](../../src/production_tool_control.rs#L1411) | receiver-type-required |
| `map_io` | `error.kind` | [1417](../../src/production_tool_control.rs#L1417) | receiver-type-required |
| `map_io` | `SupervisorOperationError::NotFound` | [1418](../../src/production_tool_control.rs#L1418) | external-constructor-callback-or-unresolved |
| `map_io` | `"resource not found".into` | [1418](../../src/production_tool_control.rs#L1418) | receiver-type-required |
| `map_io` | `SupervisorOperationError::Unavailable` | [1420](../../src/production_tool_control.rs#L1420) | external-constructor-callback-or-unresolved |
| `map_io` | `error.to_string` | [1420](../../src/production_tool_control.rs#L1420) | receiver-type-required |
| `map_store` | `SupervisorOperationError::NotFound` | [1427](../../src/production_tool_control.rs#L1427) | external-constructor-callback-or-unresolved |
| `map_store` | `"thread not found".into` | [1427](../../src/production_tool_control.rs#L1427) | receiver-type-required |
| `map_store` | `SupervisorOperationError::Conflict` | [1430](../../src/production_tool_control.rs#L1430) | external-constructor-callback-or-unresolved |
| `map_store` | `"thread is archived".into` | [1430](../../src/production_tool_control.rs#L1430) | receiver-type-required |
| `map_store` | `SupervisorOperationError::Unavailable` | [1432](../../src/production_tool_control.rs#L1432) | external-constructor-callback-or-unresolved |
| `map_store` | `other.to_string` | [1432](../../src/production_tool_control.rs#L1432) | receiver-type-required |
| `map_schema` | `SupervisorOperationError::Corruption` | [1437](../../src/production_tool_control.rs#L1437) | external-constructor-callback-or-unresolved |
| `map_schema` | `error.to_string` | [1437](../../src/production_tool_control.rs#L1437) | receiver-type-required |
| `into_control_error` | `self.to_string` | [1488](../../src/production_tool_control.rs#L1488) | receiver-type-required |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `SupervisorOperationError::ToolFailed("remote details".into()).into_control_error` | [1500](../../src/production_tool_control.rs#L1500) | receiver-type-required |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `SupervisorOperationError::ToolFailed` | [1500](../../src/production_tool_control.rs#L1500) | external-constructor-callback-or-unresolved |
| `definitive_remote_tool_failure_is_not_retryable_transport_loss` | `"remote details".into` | [1500](../../src/production_tool_control.rs#L1500) | receiver-type-required |
| `value` | `IJsonValue::parse(raw.as_bytes()).unwrap` | [1518](../../src/production_tool_control.rs#L1518) | receiver-type-required |
| `value` | `IJsonValue::parse` | [1518](../../src/production_tool_control.rs#L1518) | external-constructor-callback-or-unresolved |
| `value` | `raw.as_bytes` | [1518](../../src/production_tool_control.rs#L1518) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ProductionToolControlPolicy::new` | [1525](../../src/production_tool_control.rs#L1525), [1576](../../src/production_tool_control.rs#L1576) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `tools::SecretScanner::default` | [1525](../../src/production_tool_control.rs#L1525) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControlResult::pending` | [1526](../../src/production_tool_control.rs#L1526) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"a".repeat` | [1527](../../src/production_tool_control.rs#L1527) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"b".repeat` | [1529](../../src/production_tool_control.rs#L1529), [1544](../../src/production_tool_control.rs#L1544) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `value` | [1530](../../src/production_tool_control.rs#L1530), [1540](../../src/production_tool_control.rs#L1540), [1550](../../src/production_tool_control.rs#L1550), [1558](../../src/production_tool_control.rs#L1558) | [tekes-supervisor::production_tool_control::continuation_policy_tests::value](../../src/production_tool_control.rs#L1517) |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `policy.apply` | [1532](../../src/production_tool_control.rs#L1532) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `pending.clone` | [1532](../../src/production_tool_control.rs#L1532) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControl::new(             "018f0000-0000-7000-8000-000000000003",             "018f0000-0000-7000-8000-000000000003",             1,             "call-1",             "mcp__t__x",             value("{}"),         )         .unwrap` | [1534](../../src/production_tool_control.rs#L1534) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolControl::new` | [1534](../../src/production_tool_control.rs#L1534) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolContinuationRequest::new(original, "b".repeat(64), 1, ContinuationOperation::Query)                 .unwrap` | [1544](../../src/production_tool_control.rs#L1544) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `ToolContinuationRequest::new` | [1544](../../src/production_tool_control.rs#L1544) | [worker-control::continuation::ToolContinuationRequest::new](../../../worker-control/src/continuation.rs#L32) |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `request.request_id.clone` | [1547](../../src/production_tool_control.rs#L1547), [1555](../../src/production_tool_control.rs#L1555) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `"call-1".into` | [1548](../../src/production_tool_control.rs#L1548), [1556](../../src/production_tool_control.rs#L1556) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `policy.apply_continuation` | [1563](../../src/production_tool_control.rs#L1563) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str()                     .unwrap()                     .to_owned` | [1565](../../src/production_tool_control.rs#L1565) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str()                     .unwrap` | [1565](../../src/production_tool_control.rs#L1565) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap()["content"][0]["text"]                     .as_str` | [1565](../../src/production_tool_control.rs#L1565) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value(value).unwrap` | [1565](../../src/production_tool_control.rs#L1565) | receiver-type-required |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `serde_json::to_value` | [1565](../../src/production_tool_control.rs#L1565) | external-constructor-callback-or-unresolved |
| `pending_results_and_continuation_responses_pass_the_secret_policy` | `tools::SecretScanner::failing` | [1576](../../src/production_tool_control.rs#L1576) | [tools::pipeline::SecretScanner::failing](../../../tools/src/pipeline.rs#L107) |
| `a_started_session_inherits_execution_without_legacy_management` | `"kernel-worker".into` | [1597](../../src/production_tool_control.rs#L1597) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `"context".into` | [1598](../../src/production_tool_control.rs#L1598) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `session.into` | [1599](../../src/production_tool_control.rs#L1599) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `"create".into` | [1600](../../src/production_tool_control.rs#L1600) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `"request".into` | [1601](../../src/production_tool_control.rs#L1601) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `context_start_genesis(&source, session, "2026-10-15T00:00:00.000Z", &origin).unwrap` | [1615](../../src/production_tool_control.rs#L1615) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `context_start_genesis` | [1615](../../src/production_tool_control.rs#L1615) | external-constructor-callback-or-unresolved |
| `a_started_session_inherits_execution_without_legacy_management` | `serde_json::to_value(event.raw()).unwrap` | [1616](../../src/production_tool_control.rs#L1616) | receiver-type-required |
| `a_started_session_inherits_execution_without_legacy_management` | `serde_json::to_value` | [1616](../../src/production_tool_control.rs#L1616) | external-constructor-callback-or-unresolved |
| `a_started_session_inherits_execution_without_legacy_management` | `event.raw` | [1616](../../src/production_tool_control.rs#L1616) | receiver-type-required |
