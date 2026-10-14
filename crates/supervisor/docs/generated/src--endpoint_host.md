# tekes-supervisor::endpoint_host

[Package atlas](index.md) · [Source](../../src/endpoint_host.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::endpoint_host::EndpointRouteClass](../../src/endpoint_host.rs#L29) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::EndpointRoute](../../src/endpoint_host.rs#L37) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::TEKES_UNARY_ROUTES](../../src/endpoint_host.rs#L43) | const_item | `pub` |  |
| [tekes-supervisor::endpoint_host::route](../../src/endpoint_host.rs#L70) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointClock](../../src/endpoint_host.rs#L78) | type_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeModelReadiness](../../src/endpoint_host.rs#L81) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderStatus](../../src/endpoint_host.rs#L88) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderFailure](../../src/endpoint_host.rs#L94) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderReadiness](../../src/endpoint_host.rs#L102) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority](../../src/endpoint_host.rs#L111) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority::readiness](../../src/endpoint_host.rs#L112) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority::config_mutation_succeeded](../../src/endpoint_host.rs#L121) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority](../../src/endpoint_host.rs#L130) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::session_metadata_changed](../../src/endpoint_host.rs#L134) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::prompt](../../src/endpoint_host.rs#L136) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::cancel](../../src/endpoint_host.rs#L145) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::rename](../../src/endpoint_host.rs#L152) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::compact](../../src/endpoint_host.rs#L166) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::QueueTransactionAuthority](../../src/endpoint_host.rs#L183) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::QueueTransactionAuthority::execute](../../src/endpoint_host.rs#L184) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::recover_pending_management](../../src/endpoint_host.rs#L197) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::QueueRecoveryAdapter](../../src/endpoint_host.rs#L213) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::QueueRecoveryAdapter::execute](../../src/endpoint_host.rs#L218) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionAdmissionGates](../../src/endpoint_host.rs#L255) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionAdmissionGates::gate](../../src/endpoint_host.rs#L260) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority](../../src/endpoint_host.rs#L279) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::gates](../../src/endpoint_host.rs#L294) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::root](../../src/endpoint_host.rs#L300) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_active_session](../../src/endpoint_host.rs#L304) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate](../../src/endpoint_host.rs#L329) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::session_details](../../src/endpoint_host.rs#L354) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes](../../src/endpoint_host.rs#L364) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::capabilities](../../src/endpoint_host.rs#L365) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_method_class](../../src/endpoint_host.rs#L370) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::validate_extension_payload](../../src/endpoint_host.rs#L376) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_failure_is_exact](../../src/endpoint_host.rs#L391) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::execute](../../src/endpoint_host.rs#L399) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes](../../src/endpoint_host.rs#L410) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose](../../src/endpoint_host.rs#L415) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::owner](../../src/endpoint_host.rs#L443) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::capabilities](../../src/endpoint_host.rs#L449) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_method_class](../../src/endpoint_host.rs#L453) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::validate_extension_payload](../../src/endpoint_host.rs#L457) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_failure_is_exact](../../src/endpoint_host.rs#L467) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::execute](../../src/endpoint_host.rs#L476) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteCompositionError](../../src/endpoint_host.rs#L489) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L496) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteFailure](../../src/endpoint_host.rs#L506) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost](../../src/endpoint_host.rs#L522) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::with_preset_mcp](../../src/endpoint_host.rs#L544) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description](../../src/endpoint_host.rs#L552) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L577) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L592) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L601) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities](../../src/endpoint_host.rs#L610) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission](../../src/endpoint_host.rs#L651) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L677) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L700) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities](../../src/endpoint_host.rs#L769) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates](../../src/endpoint_host.rs#L778) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation](../../src/endpoint_host.rs#L782) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute](../../src/endpoint_host.rs#L791) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L799) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L830) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace](../../src/endpoint_host.rs#L879) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace](../../src/endpoint_host.rs#L896) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace](../../src/endpoint_host.rs#L918) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L961) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L984) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L1001) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1064) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1105) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1125) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1184) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1214) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1252) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1341) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1398) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1414) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1448) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1471) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1478) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities](../../src/endpoint_host.rs#L1499) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities](../../src/endpoint_host.rs#L1520) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class](../../src/endpoint_host.rs#L1530) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request](../../src/endpoint_host.rs#L1546) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::call](../../src/endpoint_host.rs#L1570) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly](../../src/endpoint_host.rs#L1575) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../src/endpoint_host.rs#L1582) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../src/endpoint_host.rs#L1594) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../src/endpoint_host.rs#L1607) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../src/endpoint_host.rs#L1621) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities](../../src/endpoint_host.rs#L1643) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1661) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch](../../src/endpoint_host.rs#L1686) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub](../../src/endpoint_host.rs#L1694) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host](../../src/endpoint_host.rs#L1699) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates](../../src/endpoint_host.rs#L1704) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation](../../src/endpoint_host.rs#L1708) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1716) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1724) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1820) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::runtime_failure_name](../../src/endpoint_host.rs#L1851) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1860) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1882) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1902) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1909) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L1992) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2023) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2044) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2153) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2174) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2185) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2195) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2222) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2264) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2278) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2293) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction](../../src/endpoint_host.rs#L2304) | enum_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction::with_origin](../../src/endpoint_host.rs#L2311) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2327) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2377) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2401) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2426) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2440) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_prompt_error](../../src/endpoint_host.rs#L2466) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_attachment_read_error](../../src/endpoint_host.rs#L2486) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2503) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_reason](../../src/endpoint_host.rs#L2511) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2526) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2575) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2596) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_management](../../src/endpoint_host.rs#L2613) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_json](../../src/endpoint_host.rs#L2716) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_schema](../../src/endpoint_host.rs#L2720) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_assembly](../../src/endpoint_host.rs#L2724) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure](../../src/endpoint_host.rs#L2728) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::system_timestamp](../../src/endpoint_host.rs#L2747) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2764) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointAssemblyError](../../src/endpoint_host.rs#L2780) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `HashMap` | `std::collections::HashMap` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `SystemTime` | `std::time::SystemTime` | `private` |
| `UNIX_EPOCH` | `std::time::UNIX_EPOCH` | `private` |
| `AttachmentAuthority` | `endpoint::AttachmentAuthority` | `private` |
| `AttachmentErrorReason` | `endpoint::AttachmentErrorReason` | `private` |
| `AttachmentReadError` | `endpoint::AttachmentReadError` | `private` |
| `ClientRequest` | `endpoint::ClientRequest` | `private` |
| `DurableHandoffProof` | `endpoint::DurableHandoffProof` | `private` |
| `EndpointDispatcher` | `endpoint::EndpointDispatcher` | `private` |
| `EndpointHost` | `endpoint::EndpointHost` | `private` |
| `EndpointHostCall` | `endpoint::EndpointHostCall` | `private` |
| `EndpointHostFuture` | `endpoint::EndpointHostFuture` | `private` |
| `EndpointSubscriptionHub` | `endpoint::EndpointSubscriptionHub` | `private` |
| `ForkSessionOperation` | `endpoint::ForkSessionOperation` | `private` |
| `HostFailure` | `endpoint::HostFailure` | `private` |
| `ManagementError` | `endpoint::ManagementError` | `private` |
| `ManagementStore` | `endpoint::ManagementStore` | `private` |
| `MaterializedPrompt` | `endpoint::MaterializedPrompt` | `private` |
| `NativeEndpoint` | `endpoint::NativeEndpoint` | `private` |
| `NativeEndpointError` | `endpoint::NativeEndpointError` | `private` |
| `PendingQueueTransaction` | `endpoint::PendingQueueTransaction` | `private` |
| `PromptMaterializeError` | `endpoint::PromptMaterializeError` | `private` |
| `PromptPart` | `endpoint::PromptPart` | `private` |
| `QueueRecoveryDriver` | `endpoint::QueueRecoveryDriver` | `private` |
| `QueueTransactionCompletion` | `endpoint::QueueTransactionCompletion` | `private` |
| `QueueTransactionDecision` | `endpoint::QueueTransactionDecision` | `private` |
| `QueueTransactionOperation` | `endpoint::QueueTransactionOperation` | `private` |
| `QueueTransactionState` | `endpoint::QueueTransactionState` | `private` |
| `RpcDurableIdentity` | `endpoint::RpcDurableIdentity` | `private` |
| `RpcError` | `endpoint::RpcError` | `private` |
| `RpcRegistry` | `endpoint::RpcRegistry` | `private` |
| `RpcResult` | `endpoint::RpcResult` | `private` |
| `SelectModelOperation` | `endpoint::SelectModelOperation` | `private` |
| `ServerResponse` | `endpoint::ServerResponse` | `private` |
| `SessionCreateOperation` | `endpoint::SessionCreateOperation` | `private` |
| `SessionHostDescription` | `endpoint::SessionHostDescription` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::ManagementStore::open_recovering_again"]
  n1["endpoint::types::validate_session_id"]
  n2["schema::ijson::IJsonValue::parse"]
  n3["schema::ijson::IJsonValue::parse_str"]
  n4["tekes-supervisor::endpoint_host::ProviderReadinessAuthority::config_mutation_succeeded"]
  n5["tekes-supervisor::endpoint_host::SessionDeliveryAuthority::session_metadata_changed"]
  n6["tekes-supervisor::endpoint_host::SessionDeliveryAuthority::compact"]
  n7["tekes-supervisor::endpoint_host::recover_pending_management"]
  n8["tekes-supervisor::endpoint_host::QueueRecoveryAdapter::execute"]
  n9["tekes-supervisor::endpoint_host::queue_decision"]
  n10["tekes-supervisor::endpoint_host::SessionAdmissionGates::gate"]
  n11["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n12["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::gates"]
  n13["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::root"]
  n14["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_active_session"]
  n15["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate"]
  n16["tekes-supervisor::endpoint_host::session_details"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_method_class"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::validate_extension_payload"]
  n19["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_failure_is_exact"]
  n20["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose"]
  n21["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::owner"]
  n22["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::capabilities"]
  n23["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_method_class"]
  n24["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n25["tekes-supervisor::endpoint_host::route"]
  n26["tekes-supervisor::host_runtime::system_timestamp"]
  n6 --> n3
  n6 --> n24
  n7 --> n0
  n7 --> n26
  n8 --> n9
  n10 --> n3
  n10 --> n24
  n14 --> n15
  n14 --> n16
  n14 --> n24
  n15 --> n1
  n15 --> n3
  n15 --> n24
  n16 --> n2
  n18 --> n2
  n18 --> n24
```

</details>

<details><summary>Functions 21–40: 36 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::attachment::AttachmentAuthority::open"]
  n1["endpoint::management::ManagementStore::open_at_with_queue_driver"]
  n2["endpoint::mux::SessionEndpointCapability::required"]
  n3["endpoint::service::NativeEndpoint::open"]
  n4["schema::ijson::IJsonValue::parse"]
  n5["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointHost::models"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session"]
  n16["tekes-supervisor::endpoint_host::success"]
  n17["tekes-supervisor::endpoint_host::failure"]
  n18["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n19["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::validate_extension_payload"]
  n20["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_failure_is_exact"]
  n21["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::execute"]
  n22["tekes-supervisor::endpoint_host::unsupported_route"]
  n23["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n24["tekes-supervisor::endpoint_host::ProductionEndpointHost::with_preset_mcp"]
  n25["tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description"]
  n26["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n27["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n28["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n29["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities"]
  n30["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n31["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission"]
  n32["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n33["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership"]
  n34["tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities"]
  n35["tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates"]
  n36["tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation"]
  n37["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute"]
  n38["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner"]
  n39["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension"]
  n40["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace"]
  n41["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace"]
  n42["tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace"]
  n43["tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session"]
  n44["tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session"]
  n19 --> n22
  n21 --> n22
  n22 --> n4
  n22 --> n23
  n25 --> n2
  n26 --> n30
  n27 --> n30
  n28 --> n30
  n29 --> n30
  n30 --> n33
  n31 --> n33
  n32 --> n33
  n33 --> n0
  n33 --> n1
  n33 --> n3
  n33 --> n18
  n37 --> n16
  n37 --> n17
  n37 --> n38
  n38 --> n5
  n38 --> n6
  n38 --> n7
  n38 --> n8
  n38 --> n9
  n38 --> n10
  n38 --> n11
  n38 --> n12
  n38 --> n13
  n38 --> n14
  n38 --> n15
  n38 --> n39
  n38 --> n40
  n38 --> n41
  n38 --> n42
  n38 --> n43
  n38 --> n44
```

</details>

<details><summary>Functions 41–60: 131 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["profile::config::ConfigRepository::open"]
  n2["store::platform::NamedLock::try_exclusive"]
  n3["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session"]
  n4["tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models"]
  n5["tekes-supervisor::endpoint_host::ProductionEndpointHost::models"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities"]
  n17["tekes-supervisor::endpoint_host::model_projection"]
  n18["tekes-supervisor::endpoint_host::model_is_configured"]
  n19["tekes-supervisor::endpoint_host::map_production_route"]
  n20["tekes-supervisor::endpoint_host::map_production_route_for"]
  n21["tekes-supervisor::endpoint_host::internal_failure"]
  n22["tekes-supervisor::endpoint_host::required_string"]
  n23["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n24["tekes-supervisor::endpoint_host::optional_bool"]
  n25["tekes-supervisor::endpoint_host::optional_string"]
  n26["tekes-supervisor::endpoint_host::require_object_fields"]
  n27["tekes-supervisor::endpoint_host::optional_u64"]
  n28["tekes-supervisor::endpoint_host::to_ijson"]
  n29["tekes-supervisor::endpoint_host::request_hash"]
  n30["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n31["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n32["tekes-supervisor::endpoint_host::endpoint_origin"]
  n33["tekes-supervisor::endpoint_host::validate_session_title"]
  n34["tekes-supervisor::endpoint_host::queue_action"]
  n35["tekes-supervisor::endpoint_host::queue_decision"]
  n36["tekes-supervisor::endpoint_host::map_persisted_queue_rejection"]
  n37["tekes-supervisor::endpoint_host::validate_time_zone"]
  n38["tekes-supervisor::endpoint_host::validate_extension_payload"]
  n39["tekes-supervisor::endpoint_host::invalid"]
  n40["tekes-supervisor::endpoint_host::map_endpoint"]
  n41["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n42["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension"]
  n43["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace"]
  n44["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace"]
  n45["tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace"]
  n46["tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session"]
  n47["tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session"]
  n48["tekes-supervisor::process_host::workspace_quiescence_lock_path"]
  n49["tools::guidance::IdentityProfile::parse"]
  n3 --> n25
  n3 --> n26
  n3 --> n28
  n3 --> n29
  n3 --> n30
  n3 --> n39
  n3 --> n41
  n3 --> n49
  n4 --> n1
  n4 --> n17
  n4 --> n20
  n4 --> n21
  n4 --> n26
  n5 --> n17
  n5 --> n20
  n5 --> n22
  n5 --> n26
  n5 --> n40
  n5 --> n41
  n6 --> n14
  n6 --> n18
  n6 --> n20
  n6 --> n21
  n6 --> n22
  n6 --> n25
  n6 --> n26
  n6 --> n28
  n6 --> n29
  n6 --> n30
  n6 --> n40
  n6 --> n41
  n7 --> n14
  n7 --> n20
  n7 --> n21
  n7 --> n22
  n7 --> n26
  n7 --> n28
  n7 --> n31
  n7 --> n32
  n7 --> n41
  n8 --> n14
  n8 --> n20
  n8 --> n21
  n8 --> n22
  n8 --> n23
  n8 --> n26
  n8 --> n28
  n8 --> n31
  n8 --> n32
  n8 --> n33
  n8 --> n41
  n9 --> n14
  n9 --> n20
  n9 --> n21
  n9 --> n22
  n9 --> n26
  n9 --> n28
  n9 --> n29
  n9 --> n30
  n9 --> n32
  n9 --> n34
  n9 --> n35
  n9 --> n36
  n9 --> n39
  n9 --> n41
  n10 --> n20
  n10 --> n22
  n10 --> n25
  n10 --> n26
  n10 --> n28
  n10 --> n31
  n10 --> n32
  n10 --> n37
  n10 --> n39
  n10 --> n41
  n11 --> n15
  n11 --> n22
  n11 --> n26
  n11 --> n28
  n12 --> n22
  n12 --> n24
  n12 --> n26
  n12 --> n27
  n12 --> n28
  n12 --> n29
  n12 --> n30
  n13 --> n20
  n13 --> n22
  n13 --> n26
  n13 --> n28
  n13 --> n29
  n13 --> n30
  n15 --> n0
  n15 --> n39
  n15 --> n41
  n42 --> n19
  n42 --> n20
  n42 --> n21
  n42 --> n38
  n42 --> n41
  n43 --> n22
  n43 --> n26
  n43 --> n28
  n43 --> n29
  n43 --> n30
  n44 --> n22
  n44 --> n23
  n44 --> n26
  n44 --> n28
  n44 --> n29
  n44 --> n30
  n45 --> n2
  n45 --> n21
  n45 --> n22
  n45 --> n26
  n45 --> n28
  n45 --> n29
  n45 --> n30
  n45 --> n41
  n45 --> n48
  n46 --> n20
  n46 --> n22
  n46 --> n26
  n46 --> n28
  n46 --> n29
  n46 --> n30
  n47 --> n22
  n47 --> n26
  n47 --> n28
  n47 --> n29
  n47 --> n30
```

</details>

<details><summary>Functions 61–80: 22 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::host::EndpointDispatcher::new"]
  n1["endpoint::idempotency::RpcRegistry::open"]
  n2["provider::dialect::resolve_profile"]
  n3["tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities"]
  n4["tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class"]
  n5["tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::call"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation"]
  n18["tekes-supervisor::endpoint_host::success"]
  n19["tekes-supervisor::endpoint_host::model_projection"]
  n20["tekes-supervisor::endpoint_host::model_is_configured"]
  n21["tekes-supervisor::endpoint_host::runtime_failure_name"]
  n22["tekes-supervisor::endpoint_host::map_production_route"]
  n23["tekes-supervisor::endpoint_host::internal_failure"]
  n24["tekes-supervisor::endpoint_host::to_ijson"]
  n25["tekes-supervisor::endpoint_host::validate_production_payload"]
  n26["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n27["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n28["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n29["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n30["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n5 --> n25
  n7 --> n0
  n7 --> n1
  n7 --> n26
  n8 --> n0
  n8 --> n1
  n8 --> n27
  n9 --> n0
  n9 --> n1
  n9 --> n28
  n10 --> n0
  n10 --> n1
  n10 --> n30
  n11 --> n12
  n12 --> n0
  n12 --> n1
  n12 --> n29
  n19 --> n2
  n19 --> n20
  n19 --> n23
  n19 --> n24
  n20 --> n2
```

</details>

<details><summary>Functions 81–100: 20 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["tekes-supervisor::endpoint_host::map_production_route"]
  n2["tekes-supervisor::endpoint_host::map_production_route_for"]
  n3["tekes-supervisor::endpoint_host::sanitized_route_log_message"]
  n4["tekes-supervisor::endpoint_host::production_failure_is_exact_for"]
  n5["tekes-supervisor::endpoint_host::route_error_is_exact"]
  n6["tekes-supervisor::endpoint_host::route_error_message"]
  n7["tekes-supervisor::endpoint_host::exact_string_fields"]
  n8["tekes-supervisor::endpoint_host::route_allows_error"]
  n9["tekes-supervisor::endpoint_host::internal_failure"]
  n10["tekes-supervisor::endpoint_host::failure"]
  n11["tekes-supervisor::endpoint_host::required_string"]
  n12["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n13["tekes-supervisor::endpoint_host::optional_bool"]
  n14["tekes-supervisor::endpoint_host::optional_string"]
  n15["tekes-supervisor::endpoint_host::require_object_fields"]
  n16["tekes-supervisor::endpoint_host::optional_u64"]
  n17["tekes-supervisor::endpoint_host::to_ijson"]
  n18["tekes-supervisor::endpoint_host::request_hash"]
  n19["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n20["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n21["tekes-supervisor::endpoint_host::endpoint_origin"]
  n22["tekes-supervisor::endpoint_host::invalid"]
  n23["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n2 --> n1
  n2 --> n3
  n2 --> n4
  n2 --> n9
  n3 --> n0
  n4 --> n5
  n4 --> n8
  n5 --> n6
  n5 --> n7
  n9 --> n23
  n11 --> n22
  n12 --> n22
  n13 --> n22
  n14 --> n22
  n15 --> n22
  n16 --> n22
  n17 --> n0
  n18 --> n23
  n19 --> n23
  n20 --> n9
```

</details>

<details><summary>Functions 101–120: 26 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["schema::ijson::IJsonValue::parse_str"]
  n2["tekes-supervisor::endpoint_host::internal_failure"]
  n3["tekes-supervisor::endpoint_host::require_object_fields"]
  n4["tekes-supervisor::endpoint_host::validate_session_title"]
  n5["tekes-supervisor::endpoint_host::PendingQueueAction::with_origin"]
  n6["tekes-supervisor::endpoint_host::queue_action"]
  n7["tekes-supervisor::endpoint_host::map_queue_rejection"]
  n8["tekes-supervisor::endpoint_host::queue_decision"]
  n9["tekes-supervisor::endpoint_host::map_persisted_queue_rejection"]
  n10["tekes-supervisor::endpoint_host::validate_time_zone"]
  n11["tekes-supervisor::endpoint_host::map_prompt_error"]
  n12["tekes-supervisor::endpoint_host::map_attachment_read_error"]
  n13["tekes-supervisor::endpoint_host::attachment_failure"]
  n14["tekes-supervisor::endpoint_host::attachment_reason"]
  n15["tekes-supervisor::endpoint_host::validate_extension_payload"]
  n16["tekes-supervisor::endpoint_host::validate_production_payload"]
  n17["tekes-supervisor::endpoint_host::invalid"]
  n18["tekes-supervisor::endpoint_host::map_endpoint"]
  n19["tekes-supervisor::endpoint_host::map_management"]
  n20["tekes-supervisor::endpoint_host::internal_json"]
  n21["tekes-supervisor::endpoint_host::internal_schema"]
  n22["tekes-supervisor::endpoint_host::internal_assembly"]
  n23["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n6 --> n17
  n6 --> n23
  n7 --> n23
  n9 --> n2
  n9 --> n7
  n10 --> n17
  n11 --> n2
  n11 --> n13
  n11 --> n17
  n11 --> n23
  n12 --> n2
  n12 --> n13
  n12 --> n23
  n13 --> n23
  n15 --> n3
  n15 --> n23
  n16 --> n3
  n16 --> n15
  n17 --> n23
  n18 --> n23
  n19 --> n23
  n20 --> n23
  n21 --> n23
  n22 --> n23
  n23 --> n0
  n23 --> n1
```

</details>

<details><summary>Functions 121–122: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::endpoint_host::system_timestamp"]
  n1["tekes-supervisor::endpoint_host::civil_from_days"]
  n0 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `TEKES_UNARY_ROUTES` | `route` | [44](../../src/endpoint_host.rs#L44), [45](../../src/endpoint_host.rs#L45), [46](../../src/endpoint_host.rs#L46), [47](../../src/endpoint_host.rs#L47), [52](../../src/endpoint_host.rs#L52), [57](../../src/endpoint_host.rs#L57), [58](../../src/endpoint_host.rs#L58), [59](../../src/endpoint_host.rs#L59), [60](../../src/endpoint_host.rs#L60), [61](../../src/endpoint_host.rs#L61), [62](../../src/endpoint_host.rs#L62), [63](../../src/endpoint_host.rs#L63), [64](../../src/endpoint_host.rs#L64), [65](../../src/endpoint_host.rs#L65), [66](../../src/endpoint_host.rs#L66), [67](../../src/endpoint_host.rs#L67) | [tekes-supervisor::endpoint_host::route](../../src/endpoint_host.rs#L70) |
| `config_mutation_succeeded` | `Ok` | [122](../../src/endpoint_host.rs#L122) | external-constructor-callback-or-unresolved |
| `compact` | `Err` | [173](../../src/endpoint_host.rs#L173) | external-constructor-callback-or-unresolved |
| `compact` | `ProductionRouteFailure::new` | [173](../../src/endpoint_host.rs#L173) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `compact` | `IJsonValue::parse_str(r#"{"operation":"compact"}"#).expect` | [176](../../src/endpoint_host.rs#L176) | receiver-type-required |
| `compact` | `IJsonValue::parse_str` | [176](../../src/endpoint_host.rs#L176) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `recover_pending_management` | `crate::host_runtime::system_timestamp()         .map_err` | [202](../../src/endpoint_host.rs#L202) | receiver-type-required |
| `recover_pending_management` | `crate::host_runtime::system_timestamp` | [202](../../src/endpoint_host.rs#L202) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L497) |
| `recover_pending_management` | `ManagementError::QueueRecoveryFailed` | [203](../../src/endpoint_host.rs#L203) | external-constructor-callback-or-unresolved |
| `recover_pending_management` | `error.to_string` | [203](../../src/endpoint_host.rs#L203) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again(         root,         &started_at,         Some(&QueueRecoveryAdapter { authority }),         validated,     )     .map` | [204](../../src/endpoint_host.rs#L204) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again` | [204](../../src/endpoint_host.rs#L204) | [endpoint::management::ManagementStore::open_recovering_again](../../../endpoint/src/management.rs#L440) |
| `recover_pending_management` | `Some` | [207](../../src/endpoint_host.rs#L207) | external-constructor-callback-or-unresolved |
| `execute` | `serde_json::from_slice(&pending.action.canonical_bytes()?)             .map_err` | [222](../../src/endpoint_host.rs#L222) | receiver-type-required |
| `execute` | `serde_json::from_slice` | [222](../../src/endpoint_host.rs#L222) | external-constructor-callback-or-unresolved |
| `execute` | `pending.action.canonical_bytes` | [222](../../src/endpoint_host.rs#L222) | receiver-type-required |
| `execute` | `pending.rpc_id.clone` | [225](../../src/endpoint_host.rs#L225), [226](../../src/endpoint_host.rs#L226) | receiver-type-required |
| `execute` | `pending.retract_origin.clone` | [228](../../src/endpoint_host.rs#L228) | receiver-type-required |
| `execute` | `transaction.validate().map_err` | [231](../../src/endpoint_host.rs#L231) | receiver-type-required |
| `execute` | `transaction.validate` | [231](../../src/endpoint_host.rs#L231) | receiver-type-required |
| `execute` | `ManagementError::QueueRecoveryFailed` | [232](../../src/endpoint_host.rs#L232), [237](../../src/endpoint_host.rs#L237), [244](../../src/endpoint_host.rs#L244), [246](../../src/endpoint_host.rs#L246) | external-constructor-callback-or-unresolved |
| `execute` | `transaction.replacement_origin` | [236](../../src/endpoint_host.rs#L236) | receiver-type-required |
| `execute` | `pending.replacement_origin.as_ref` | [236](../../src/endpoint_host.rs#L236) | receiver-type-required |
| `execute` | `Err` | [237](../../src/endpoint_host.rs#L237) | external-constructor-callback-or-unresolved |
| `execute` | `"prepared replacement origin disagrees with action".to_owned` | [238](../../src/endpoint_host.rs#L238) | receiver-type-required |
| `execute` | `self             .authority             .execute(&pending.session_id, &transaction)             .map_err` | [241](../../src/endpoint_host.rs#L241) | receiver-type-required |
| `execute` | `self             .authority             .execute` | [241](../../src/endpoint_host.rs#L241) | receiver-type-required |
| `execute` | `result.validate_for(&transaction).map_err` | [245](../../src/endpoint_host.rs#L245) | receiver-type-required |
| `execute` | `result.validate_for` | [245](../../src/endpoint_host.rs#L245) | receiver-type-required |
| `execute` | `Ok` | [250](../../src/endpoint_host.rs#L250) | external-constructor-callback-or-unresolved |
| `execute` | `queue_decision` | [250](../../src/endpoint_host.rs#L250) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2401) |
| `gate` | `self.gates.lock().map_err` | [261](../../src/endpoint_host.rs#L261) | receiver-type-required |
| `gate` | `self.gates.lock` | [261](../../src/endpoint_host.rs#L261) | receiver-type-required |
| `gate` | `ProductionRouteFailure::new` | [262](../../src/endpoint_host.rs#L262) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `gate` | `IJsonValue::parse_str("{}").expect` | [265](../../src/endpoint_host.rs#L265) | receiver-type-required |
| `gate` | `IJsonValue::parse_str` | [265](../../src/endpoint_host.rs#L265) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `gate` | `Ok` | [268](../../src/endpoint_host.rs#L268) | external-constructor-callback-or-unresolved |
| `gate` | `gates             .entry(session_id.to_owned())             .or_insert_with(&#124;&#124; Arc::new(Mutex::new(())))             .clone` | [268](../../src/endpoint_host.rs#L268) | receiver-type-required |
| `gate` | `gates             .entry(session_id.to_owned())             .or_insert_with` | [268](../../src/endpoint_host.rs#L268) | receiver-type-required |
| `gate` | `gates             .entry` | [268](../../src/endpoint_host.rs#L268) | receiver-type-required |
| `gate` | `session_id.to_owned` | [269](../../src/endpoint_host.rs#L269) | receiver-type-required |
| `gate` | `Arc::new` | [270](../../src/endpoint_host.rs#L270) | external-constructor-callback-or-unresolved |
| `gate` | `Mutex::new` | [270](../../src/endpoint_host.rs#L270) | external-constructor-callback-or-unresolved |
| `new` | `root.into` | [288](../../src/endpoint_host.rs#L288) | receiver-type-required |
| `new` | `Arc::new` | [289](../../src/endpoint_host.rs#L289) | external-constructor-callback-or-unresolved |
| `new` | `SessionAdmissionGates::default` | [289](../../src/endpoint_host.rs#L289) | external-constructor-callback-or-unresolved |
| `gates` | `Arc::clone` | [295](../../src/endpoint_host.rs#L295) | external-constructor-callback-or-unresolved |
| `with_active_session` | `self.with_session_gate` | [310](../../src/endpoint_host.rs#L310) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate](../../src/endpoint_host.rs#L329) |
| `with_active_session` | `self.root.join("archive").join(session_id).is_dir` | [311](../../src/endpoint_host.rs#L311) | receiver-type-required |
| `with_active_session` | `self.root.join("archive").join` | [311](../../src/endpoint_host.rs#L311) | receiver-type-required |
| `with_active_session` | `self.root.join` | [311](../../src/endpoint_host.rs#L311), [318](../../src/endpoint_host.rs#L318) | receiver-type-required |
| `with_active_session` | `Err` | [312](../../src/endpoint_host.rs#L312), [319](../../src/endpoint_host.rs#L319) | external-constructor-callback-or-unresolved |
| `with_active_session` | `map_failure` | [312](../../src/endpoint_host.rs#L312), [319](../../src/endpoint_host.rs#L319) | external-constructor-callback-or-unresolved |
| `with_active_session` | `ProductionRouteFailure::new` | [312](../../src/endpoint_host.rs#L312), [319](../../src/endpoint_host.rs#L319) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `with_active_session` | `session_details` | [315](../../src/endpoint_host.rs#L315), [322](../../src/endpoint_host.rs#L322) | [tekes-supervisor::endpoint_host::session_details](../../src/endpoint_host.rs#L354) |
| `with_active_session` | `self.root.join("threads").join(session_id).is_dir` | [318](../../src/endpoint_host.rs#L318) | receiver-type-required |
| `with_active_session` | `self.root.join("threads").join` | [318](../../src/endpoint_host.rs#L318) | receiver-type-required |
| `with_active_session` | `operation` | [325](../../src/endpoint_host.rs#L325) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `endpoint::validate_session_id(session_id).is_err` | [335](../../src/endpoint_host.rs#L335) | receiver-type-required |
| `with_session_gate` | `endpoint::validate_session_id` | [335](../../src/endpoint_host.rs#L335) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `with_session_gate` | `Err` | [336](../../src/endpoint_host.rs#L336) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `map_failure` | [336](../../src/endpoint_host.rs#L336), [344](../../src/endpoint_host.rs#L344) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `ProductionRouteFailure::new` | [336](../../src/endpoint_host.rs#L336), [344](../../src/endpoint_host.rs#L344) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `with_session_gate` | `IJsonValue::parse_str("{}").expect` | [339](../../src/endpoint_host.rs#L339), [347](../../src/endpoint_host.rs#L347) | receiver-type-required |
| `with_session_gate` | `IJsonValue::parse_str` | [339](../../src/endpoint_host.rs#L339), [347](../../src/endpoint_host.rs#L347) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `with_session_gate` | `self.gates.gate(session_id).map_err` | [342](../../src/endpoint_host.rs#L342) | receiver-type-required |
| `with_session_gate` | `self.gates.gate` | [342](../../src/endpoint_host.rs#L342) | receiver-type-required |
| `with_session_gate` | `gate.lock().map_err` | [343](../../src/endpoint_host.rs#L343) | receiver-type-required |
| `with_session_gate` | `gate.lock` | [343](../../src/endpoint_host.rs#L343) | receiver-type-required |
| `with_session_gate` | `operation` | [350](../../src/endpoint_host.rs#L350) | external-constructor-callback-or-unresolved |
| `session_details` | `IJsonValue::parse(&serde_json::to_vec(&json!({"sessionId":session_id})).expect("JSON"))         .expect` | [355](../../src/endpoint_host.rs#L355) | receiver-type-required |
| `session_details` | `IJsonValue::parse` | [355](../../src/endpoint_host.rs#L355) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `session_details` | `serde_json::to_vec(&json!({"sessionId":session_id})).expect` | [355](../../src/endpoint_host.rs#L355) | receiver-type-required |
| `session_details` | `serde_json::to_vec` | [355](../../src/endpoint_host.rs#L355) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `Err` | [381](../../src/endpoint_host.rs#L381) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `ProductionRouteFailure::new` | [381](../../src/endpoint_host.rs#L381) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `validate_extension_payload` | `IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))                 .expect` | [384](../../src/endpoint_host.rs#L384) | receiver-type-required |
| `validate_extension_payload` | `IJsonValue::parse` | [384](../../src/endpoint_host.rs#L384) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `validate_extension_payload` | `serde_json::to_vec(&json!({"operation":operation})).expect` | [384](../../src/endpoint_host.rs#L384) | receiver-type-required |
| `validate_extension_payload` | `serde_json::to_vec` | [384](../../src/endpoint_host.rs#L384) | external-constructor-callback-or-unresolved |
| `compose` | `HashMap::new` | [418](../../src/endpoint_host.rs#L418) | external-constructor-callback-or-unresolved |
| `compose` | `TEKES_UNARY_ROUTES             .iter()             .map(&#124;route&#124; route.name)             .collect::<BTreeSet<_>>` | [419](../../src/endpoint_host.rs#L419) | receiver-type-required |
| `compose` | `TEKES_UNARY_ROUTES             .iter()             .map` | [419](../../src/endpoint_host.rs#L419) | receiver-type-required |
| `compose` | `TEKES_UNARY_ROUTES             .iter` | [419](../../src/endpoint_host.rs#L419) | receiver-type-required |
| `compose` | `route.capabilities` | [424](../../src/endpoint_host.rs#L424) | receiver-type-required |
| `compose` | `frozen.contains` | [425](../../src/endpoint_host.rs#L425) | receiver-type-required |
| `compose` | `capability.as_str` | [425](../../src/endpoint_host.rs#L425) | receiver-type-required |
| `compose` | `Err` | [426](../../src/endpoint_host.rs#L426), [434](../../src/endpoint_host.rs#L434) | external-constructor-callback-or-unresolved |
| `compose` | `ProductionRouteCompositionError::FrozenCapability` | [426](../../src/endpoint_host.rs#L426) | external-constructor-callback-or-unresolved |
| `compose` | `owners                     .insert(capability.clone(), Arc::clone(&route))                     .is_some` | [430](../../src/endpoint_host.rs#L430) | receiver-type-required |
| `compose` | `owners                     .insert` | [430](../../src/endpoint_host.rs#L430) | receiver-type-required |
| `compose` | `capability.clone` | [431](../../src/endpoint_host.rs#L431) | receiver-type-required |
| `compose` | `Arc::clone` | [431](../../src/endpoint_host.rs#L431) | external-constructor-callback-or-unresolved |
| `compose` | `ProductionRouteCompositionError::DuplicateCapability` | [434](../../src/endpoint_host.rs#L434) | external-constructor-callback-or-unresolved |
| `compose` | `Ok` | [440](../../src/endpoint_host.rs#L440) | external-constructor-callback-or-unresolved |
| `owner` | `self.owners.get` | [444](../../src/endpoint_host.rs#L444) | receiver-type-required |
| `capabilities` | `self.owners.keys().cloned().collect` | [450](../../src/endpoint_host.rs#L450) | receiver-type-required |
| `capabilities` | `self.owners.keys().cloned` | [450](../../src/endpoint_host.rs#L450) | receiver-type-required |
| `capabilities` | `self.owners.keys` | [450](../../src/endpoint_host.rs#L450) | receiver-type-required |
| `extension_method_class` | `self.owner(method)?.extension_method_class` | [454](../../src/endpoint_host.rs#L454) | receiver-type-required |
| `extension_method_class` | `self.owner` | [454](../../src/endpoint_host.rs#L454) | receiver-type-required |
| `validate_extension_payload` | `self.owner(operation)             .ok_or_else(&#124;&#124; unsupported_route(operation))?             .validate_extension_payload` | [462](../../src/endpoint_host.rs#L462) | receiver-type-required |
| `validate_extension_payload` | `self.owner(operation)             .ok_or_else` | [462](../../src/endpoint_host.rs#L462) | receiver-type-required |
| `validate_extension_payload` | `self.owner` | [462](../../src/endpoint_host.rs#L462) | receiver-type-required |
| `validate_extension_payload` | `unsupported_route` | [463](../../src/endpoint_host.rs#L463) | [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L496) |
| `extension_failure_is_exact` | `self.owner(operation)             .is_some_and` | [472](../../src/endpoint_host.rs#L472) | receiver-type-required |
| `extension_failure_is_exact` | `self.owner` | [472](../../src/endpoint_host.rs#L472) | receiver-type-required |
| `extension_failure_is_exact` | `owner.extension_failure_is_exact` | [473](../../src/endpoint_host.rs#L473) | receiver-type-required |
| `execute` | `self.owner(&request.operation)             .ok_or_else(&#124;&#124; unsupported_route(&request.operation))?             .execute` | [482](../../src/endpoint_host.rs#L482) | receiver-type-required |
| `execute` | `self.owner(&request.operation)             .ok_or_else` | [482](../../src/endpoint_host.rs#L482) | receiver-type-required |
| `execute` | `self.owner` | [482](../../src/endpoint_host.rs#L482) | receiver-type-required |
| `execute` | `unsupported_route` | [483](../../src/endpoint_host.rs#L483) | [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L496) |
| `unsupported_route` | `ProductionRouteFailure::new` | [497](../../src/endpoint_host.rs#L497) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `unsupported_route` | `IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))             .expect` | [500](../../src/endpoint_host.rs#L500) | receiver-type-required |
| `unsupported_route` | `IJsonValue::parse` | [500](../../src/endpoint_host.rs#L500) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `unsupported_route` | `serde_json::to_vec(&json!({"operation":operation})).expect` | [500](../../src/endpoint_host.rs#L500) | receiver-type-required |
| `unsupported_route` | `serde_json::to_vec` | [500](../../src/endpoint_host.rs#L500) | external-constructor-callback-or-unresolved |
| `new` | `code.into` | [515](../../src/endpoint_host.rs#L515) | receiver-type-required |
| `new` | `message.into` | [516](../../src/endpoint_host.rs#L516) | receiver-type-required |
| `with_preset_mcp` | `Some` | [548](../../src/endpoint_host.rs#L548) | external-constructor-callback-or-unresolved |
| `mux_description` | `endpoint::SessionEndpointCapability::required` | [553](../../src/endpoint_host.rs#L553) | [endpoint::mux::SessionEndpointCapability::required](../../../endpoint/src/mux.rs#L34) |
| `mux_description` | `capabilities.extend` | [554](../../src/endpoint_host.rs#L554) | receiver-type-required |
| `mux_description` | `self.provider_readiness.is_some` | [558](../../src/endpoint_host.rs#L558) | receiver-type-required |
| `mux_description` | `capabilities.insert` | [559](../../src/endpoint_host.rs#L559) | receiver-type-required |
| `mux_description` | `"TekesKernel".to_owned` | [564](../../src/endpoint_host.rs#L564) | receiver-type-required |
| `mux_description` | `self.description.version.clone` | [565](../../src/endpoint_host.rs#L565) | receiver-type-required |
| `mux_description` | `self.description.cwd.clone` | [568](../../src/endpoint_host.rs#L568) | receiver-type-required |
| `mux_description` | `self.description.provider.clone` | [569](../../src/endpoint_host.rs#L569) | receiver-type-required |
| `mux_description` | `self.description.model.clone` | [570](../../src/endpoint_host.rs#L570) | receiver-type-required |
| `mux_description` | `self.description.home.clone` | [572](../../src/endpoint_host.rs#L572) | receiver-type-required |
| `open` | `Self::open_with_full_authorities` | [581](../../src/endpoint_host.rs#L581) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) |
| `open` | `Arc::new` | [584](../../src/endpoint_host.rs#L584) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [597](../../src/endpoint_host.rs#L597) | receiver-type-required |
| `open_with_clock` | `Self::open_with_full_authorities` | [598](../../src/endpoint_host.rs#L598) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) |
| `open_with_routes` | `Self::open_with_full_authorities` | [607](../../src/endpoint_host.rs#L607) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) |
| `open_with_authorities` | `Self::open_with_full_authorities` | [617](../../src/endpoint_host.rs#L617) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) |
| `open_with_full_authorities` | `Self::open_with_route_ownership` | [637](../../src/endpoint_host.rs#L637) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L700) |
| `open_with_full_authorities_and_session_admission` | `Self::open_with_route_ownership` | [661](../../src/endpoint_host.rs#L661) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L700) |
| `open_with_full_authorities_and_session_admission` | `Some` | [670](../../src/endpoint_host.rs#L670) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `Self::open_with_route_ownership` | [686](../../src/endpoint_host.rs#L686) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L700) |
| `open_with_route_authority` | `Some` | [690](../../src/endpoint_host.rs#L690) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.as_ref` | [711](../../src/endpoint_host.rs#L711) | receiver-type-required |
| `open_with_route_ownership` | `routes.as_ref` | [712](../../src/endpoint_host.rs#L712) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map(&#124;route&#124; route.name)                 .collect::<BTreeSet<_>>` | [713](../../src/endpoint_host.rs#L713) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map` | [713](../../src/endpoint_host.rs#L713) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter` | [713](../../src/endpoint_host.rs#L713) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter().any` | [717](../../src/endpoint_host.rs#L717) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter` | [717](../../src/endpoint_host.rs#L717) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities` | [717](../../src/endpoint_host.rs#L717) | receiver-type-required |
| `open_with_route_ownership` | `frozen.contains` | [719](../../src/endpoint_host.rs#L719), [721](../../src/endpoint_host.rs#L721) | receiver-type-required |
| `open_with_route_ownership` | `method.as_str` | [719](../../src/endpoint_host.rs#L719), [721](../../src/endpoint_host.rs#L721) | receiver-type-required |
| `open_with_route_ownership` | `Err` | [728](../../src/endpoint_host.rs#L728) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `clock` | [731](../../src/endpoint_host.rs#L731) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `PathBuf::from(&description.home).join` | [732](../../src/endpoint_host.rs#L732) | receiver-type-required |
| `open_with_route_ownership` | `PathBuf::from` | [732](../../src/endpoint_host.rs#L732) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `queue_authority             .as_deref()             .map` | [733](../../src/endpoint_host.rs#L733) | receiver-type-required |
| `open_with_route_ownership` | `queue_authority             .as_deref` | [733](../../src/endpoint_host.rs#L733) | receiver-type-required |
| `open_with_route_ownership` | `ManagementStore::open_at_with_queue_driver` | [736](../../src/endpoint_host.rs#L736) | [endpoint::management::ManagementStore::open_at_with_queue_driver](../../../endpoint/src/management.rs#L429) |
| `open_with_route_ownership` | `queue_recovery                 .as_ref()                 .map` | [739](../../src/endpoint_host.rs#L739) | receiver-type-required |
| `open_with_route_ownership` | `queue_recovery                 .as_ref` | [739](../../src/endpoint_host.rs#L739) | receiver-type-required |
| `open_with_route_ownership` | `Ok` | [745](../../src/endpoint_host.rs#L745) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.to_path_buf` | [746](../../src/endpoint_host.rs#L746), [762](../../src/endpoint_host.rs#L762) | receiver-type-required |
| `open_with_route_ownership` | `NativeEndpoint::open` | [747](../../src/endpoint_host.rs#L747) | [endpoint::service::NativeEndpoint::open](../../../endpoint/src/service.rs#L263) |
| `open_with_route_ownership` | `AttachmentAuthority::open` | [748](../../src/endpoint_host.rs#L748) | [endpoint::attachment::AttachmentAuthority::open](../../../endpoint/src/attachment.rs#L242) |
| `open_with_route_ownership` | `input_admission.unwrap_or_else` | [761](../../src/endpoint_host.rs#L761) | receiver-type-required |
| `open_with_route_ownership` | `Arc::new` | [762](../../src/endpoint_host.rs#L762) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `SessionInputAdmissionAuthority::new` | [762](../../src/endpoint_host.rs#L762) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map(&#124;route&#124; route.name.to_owned())             .collect` | [770](../../src/endpoint_host.rs#L770) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map` | [770](../../src/endpoint_host.rs#L770) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter` | [770](../../src/endpoint_host.rs#L770) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter` | [770](../../src/endpoint_host.rs#L770) | receiver-type-required |
| `implemented_capabilities` | `route.name.to_owned` | [773](../../src/endpoint_host.rs#L773) | receiver-type-required |
| `admission_gates` | `self.input_admission.gates` | [779](../../src/endpoint_host.rs#L779) | receiver-type-required |
| `has_incomplete_management_operation` | `Ok` | [786](../../src/endpoint_host.rs#L786) | external-constructor-callback-or-unresolved |
| `has_incomplete_management_operation` | `self             .management             .has_incomplete_session_operation` | [786](../../src/endpoint_host.rs#L786) | receiver-type-required |
| `execute` | `self.execute_inner` | [792](../../src/endpoint_host.rs#L792) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L799) |
| `execute` | `success` | [793](../../src/endpoint_host.rs#L793) | [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1716) |
| `execute` | `failure` | [796](../../src/endpoint_host.rs#L796) | [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2153) |
| `execute_inner` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)                 .map_err` | [801](../../src/endpoint_host.rs#L801) | receiver-type-required |
| `execute_inner` | `serde_json::from_slice` | [801](../../src/endpoint_host.rs#L801) | external-constructor-callback-or-unresolved |
| `execute_inner` | `request.payload.canonical_bytes().map_err` | [801](../../src/endpoint_host.rs#L801) | receiver-type-required |
| `execute_inner` | `request.payload.canonical_bytes` | [801](../../src/endpoint_host.rs#L801) | receiver-type-required |
| `execute_inner` | `request.operation.as_str` | [803](../../src/endpoint_host.rs#L803) | receiver-type-required |
| `execute_inner` | `self.create_workspace` | [804](../../src/endpoint_host.rs#L804) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace](../../src/endpoint_host.rs#L879) |
| `execute_inner` | `self.rename_workspace` | [805](../../src/endpoint_host.rs#L805) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace](../../src/endpoint_host.rs#L896) |
| `execute_inner` | `self.relocate_workspace` | [806](../../src/endpoint_host.rs#L806) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace](../../src/endpoint_host.rs#L918) |
| `execute_inner` | `self.archive_session` | [807](../../src/endpoint_host.rs#L807) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L961) |
| `execute_inner` | `self.unarchive_session` | [808](../../src/endpoint_host.rs#L808) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L984) |
| `execute_inner` | `self.create_session` | [809](../../src/endpoint_host.rs#L809) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L1001) |
| `execute_inner` | `self.provider_readiness.is_some` | [810](../../src/endpoint_host.rs#L810), [811](../../src/endpoint_host.rs#L811), [812](../../src/endpoint_host.rs#L812) | receiver-type-required |
| `execute_inner` | `self.models` | [810](../../src/endpoint_host.rs#L810) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1105) |
| `execute_inner` | `self.draft_models` | [811](../../src/endpoint_host.rs#L811) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1064) |
| `execute_inner` | `self.select_model` | [813](../../src/endpoint_host.rs#L813) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1125) |
| `execute_inner` | `self.delivery_authority.is_some` | [815](../../src/endpoint_host.rs#L815), [816](../../src/endpoint_host.rs#L816), [822](../../src/endpoint_host.rs#L822) | receiver-type-required |
| `execute_inner` | `self.cancel` | [815](../../src/endpoint_host.rs#L815) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1184) |
| `execute_inner` | `self.rename_session` | [817](../../src/endpoint_host.rs#L817) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1214) |
| `execute_inner` | `self.queue_authority.is_some` | [819](../../src/endpoint_host.rs#L819) | receiver-type-required |
| `execute_inner` | `self.update_queue` | [820](../../src/endpoint_host.rs#L820) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1252) |
| `execute_inner` | `self.prompt` | [822](../../src/endpoint_host.rs#L822) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1341) |
| `execute_inner` | `self.attachment` | [823](../../src/endpoint_host.rs#L823) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1398) |
| `execute_inner` | `self.fork_session` | [824](../../src/endpoint_host.rs#L824) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1414) |
| `execute_inner` | `self.discard_session` | [825](../../src/endpoint_host.rs#L825) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1448) |
| `execute_inner` | `self.execute_extension` | [826](../../src/endpoint_host.rs#L826) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L830) |
| `execute_extension` | `self.routes.as_ref` | [836](../../src/endpoint_host.rs#L836) | receiver-type-required |
| `execute_extension` | `Err` | [837](../../src/endpoint_host.rs#L837), [844](../../src/endpoint_host.rs#L844) | external-constructor-callback-or-unresolved |
| `execute_extension` | `EndpointRpcFailure::new` | [837](../../src/endpoint_host.rs#L837), [844](../../src/endpoint_host.rs#L844) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `execute_extension` | `routes.capabilities().contains` | [843](../../src/endpoint_host.rs#L843) | receiver-type-required |
| `execute_extension` | `routes.capabilities` | [843](../../src/endpoint_host.rs#L843) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter()             .any` | [850](../../src/endpoint_host.rs#L850) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter` | [850](../../src/endpoint_host.rs#L850) | receiver-type-required |
| `execute_extension` | `validate_extension_payload` | [854](../../src/endpoint_host.rs#L854) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2526) |
| `execute_extension` | `routes                 .validate_extension_payload(operation, payload)                 .map_err` | [856](../../src/endpoint_host.rs#L856) | receiver-type-required |
| `execute_extension` | `routes                 .validate_extension_payload` | [856](../../src/endpoint_host.rs#L856) | receiver-type-required |
| `execute_extension` | `routes.extension_failure_is_exact` | [859](../../src/endpoint_host.rs#L859), [871](../../src/endpoint_host.rs#L871) | receiver-type-required |
| `execute_extension` | `map_production_route` | [860](../../src/endpoint_host.rs#L860), [872](../../src/endpoint_host.rs#L872) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1860) |
| `execute_extension` | `internal_failure` | [862](../../src/endpoint_host.rs#L862), [874](../../src/endpoint_host.rs#L874) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `execute_extension` | `routes             .execute(request, payload, &self.principal)             .map_err` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `routes             .execute` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `map_production_route_for` | [870](../../src/endpoint_host.rs#L870) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `create_workspace` | `require_object_fields` | [884](../../src/endpoint_host.rs#L884) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `create_workspace` | `required_string` | [885](../../src/endpoint_host.rs#L885) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `create_workspace` | `(self.clock)().map_err` | [886](../../src/endpoint_host.rs#L886) | receiver-type-required |
| `create_workspace` | `(self.clock)` | [886](../../src/endpoint_host.rs#L886) | external-constructor-callback-or-unresolved |
| `create_workspace` | `request_hash` | [887](../../src/endpoint_host.rs#L887) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `create_workspace` | `self             .management             .create_workspace(&request.rpc_id, &hash, path, &timestamp)             .map_err` | [888](../../src/endpoint_host.rs#L888) | receiver-type-required |
| `create_workspace` | `self             .management             .create_workspace` | [888](../../src/endpoint_host.rs#L888) | receiver-type-required |
| `create_workspace` | `mark_management_handoff` | [892](../../src/endpoint_host.rs#L892) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `create_workspace` | `to_ijson` | [893](../../src/endpoint_host.rs#L893) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `rename_workspace` | `require_object_fields` | [901](../../src/endpoint_host.rs#L901) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `rename_workspace` | `required_string` | [906](../../src/endpoint_host.rs#L906) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `rename_workspace` | `required_string_allow_whitespace` | [907](../../src/endpoint_host.rs#L907) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2174) |
| `rename_workspace` | `(self.clock)().map_err` | [908](../../src/endpoint_host.rs#L908) | receiver-type-required |
| `rename_workspace` | `(self.clock)` | [908](../../src/endpoint_host.rs#L908) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `request_hash` | [909](../../src/endpoint_host.rs#L909) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `rename_workspace` | `self             .management             .rename_workspace(&request.rpc_id, &hash, workspace_id, title, &timestamp)             .map_err` | [910](../../src/endpoint_host.rs#L910) | receiver-type-required |
| `rename_workspace` | `self             .management             .rename_workspace` | [910](../../src/endpoint_host.rs#L910) | receiver-type-required |
| `rename_workspace` | `mark_management_handoff` | [914](../../src/endpoint_host.rs#L914) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `rename_workspace` | `to_ijson` | [915](../../src/endpoint_host.rs#L915) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `relocate_workspace` | `require_object_fields` | [923](../../src/endpoint_host.rs#L923) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `relocate_workspace` | `required_string` | [928](../../src/endpoint_host.rs#L928), [929](../../src/endpoint_host.rs#L929), [930](../../src/endpoint_host.rs#L930) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `relocate_workspace` | `store::NamedLock::try_exclusive(             crate::process_host::workspace_quiescence_lock_path(&self.storage_root, workspace_id),         )         .map_err` | [933](../../src/endpoint_host.rs#L933) | receiver-type-required |
| `relocate_workspace` | `store::NamedLock::try_exclusive` | [933](../../src/endpoint_host.rs#L933) | [store::platform::NamedLock::try_exclusive](../../../store/src/platform.rs#L107) |
| `relocate_workspace` | `crate::process_host::workspace_quiescence_lock_path` | [934](../../src/endpoint_host.rs#L934) | [tekes-supervisor::process_host::workspace_quiescence_lock_path](../../src/process_host.rs#L6369) |
| `relocate_workspace` | `EndpointRpcFailure::new` | [937](../../src/endpoint_host.rs#L937) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `relocate_workspace` | `internal_failure` | [942](../../src/endpoint_host.rs#L942) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `relocate_workspace` | `(self.clock)().map_err` | [944](../../src/endpoint_host.rs#L944) | receiver-type-required |
| `relocate_workspace` | `(self.clock)` | [944](../../src/endpoint_host.rs#L944) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `request_hash` | [945](../../src/endpoint_host.rs#L945) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `relocate_workspace` | `self             .management             .relocate_workspace(                 &request.rpc_id,                 &hash,                 workspace_id,                 previous_path,                 path,                 &timestamp,             )             .map_err` | [946](../../src/endpoint_host.rs#L946) | receiver-type-required |
| `relocate_workspace` | `self             .management             .relocate_workspace` | [946](../../src/endpoint_host.rs#L946) | receiver-type-required |
| `relocate_workspace` | `mark_management_handoff` | [957](../../src/endpoint_host.rs#L957) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `relocate_workspace` | `to_ijson` | [958](../../src/endpoint_host.rs#L958) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `archive_session` | `require_object_fields` | [966](../../src/endpoint_host.rs#L966) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `archive_session` | `required_string` | [967](../../src/endpoint_host.rs#L967) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `archive_session` | `self.input_admission.with_session_gate` | [968](../../src/endpoint_host.rs#L968) | receiver-type-required |
| `archive_session` | `map_production_route_for` | [970](../../src/endpoint_host.rs#L970) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `archive_session` | `(self.clock)().map_err` | [972](../../src/endpoint_host.rs#L972) | receiver-type-required |
| `archive_session` | `(self.clock)` | [972](../../src/endpoint_host.rs#L972) | external-constructor-callback-or-unresolved |
| `archive_session` | `request_hash` | [973](../../src/endpoint_host.rs#L973) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `archive_session` | `self                     .management                     .archive_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [974](../../src/endpoint_host.rs#L974) | receiver-type-required |
| `archive_session` | `self                     .management                     .archive_session` | [974](../../src/endpoint_host.rs#L974) | receiver-type-required |
| `archive_session` | `mark_management_handoff` | [978](../../src/endpoint_host.rs#L978) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `archive_session` | `to_ijson` | [979](../../src/endpoint_host.rs#L979) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `unarchive_session` | `require_object_fields` | [989](../../src/endpoint_host.rs#L989) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `unarchive_session` | `required_string` | [990](../../src/endpoint_host.rs#L990) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `unarchive_session` | `(self.clock)().map_err` | [991](../../src/endpoint_host.rs#L991) | receiver-type-required |
| `unarchive_session` | `(self.clock)` | [991](../../src/endpoint_host.rs#L991) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `request_hash` | [992](../../src/endpoint_host.rs#L992) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `unarchive_session` | `self             .management             .unarchive_session(&request.rpc_id, &hash, session_id, &timestamp)             .map_err` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `unarchive_session` | `self             .management             .unarchive_session` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `unarchive_session` | `mark_management_handoff` | [997](../../src/endpoint_host.rs#L997) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `unarchive_session` | `to_ijson` | [998](../../src/endpoint_host.rs#L998) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `create_session` | `require_object_fields` | [1006](../../src/endpoint_host.rs#L1006) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `create_session` | `payload.get("agentPreset").is_some` | [1018](../../src/endpoint_host.rs#L1018) | receiver-type-required |
| `create_session` | `payload.get` | [1018](../../src/endpoint_host.rs#L1018), [1035](../../src/endpoint_host.rs#L1035) | receiver-type-required |
| `create_session` | `Err` | [1019](../../src/endpoint_host.rs#L1019), [1037](../../src/endpoint_host.rs#L1037) | external-constructor-callback-or-unresolved |
| `create_session` | `EndpointRpcFailure::new` | [1019](../../src/endpoint_host.rs#L1019) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `create_session` | `optional_string` | [1025](../../src/endpoint_host.rs#L1025), [1026](../../src/endpoint_host.rs#L1026), [1027](../../src/endpoint_host.rs#L1027), [1028](../../src/endpoint_host.rs#L1028) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2195) |
| `create_session` | `optional_string(payload, "identityProfile")?             .map(&#124;value&#124; {                 tools::IdentityProfile::parse(value)                     .ok_or_else(&#124;&#124; invalid("identityProfile must be coding or general"))             })             .transpose` | [1028](../../src/endpoint_host.rs#L1028) | receiver-type-required |
| `create_session` | `optional_string(payload, "identityProfile")?             .map` | [1028](../../src/endpoint_host.rs#L1028) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse(value)                     .ok_or_else` | [1030](../../src/endpoint_host.rs#L1030) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse` | [1030](../../src/endpoint_host.rs#L1030) | [tools::guidance::IdentityProfile::parse](../../../tools/src/guidance.rs#L54) |
| `create_session` | `invalid` | [1031](../../src/endpoint_host.rs#L1031), [1037](../../src/endpoint_host.rs#L1037) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `create_session` | `preset.is_some` | [1036](../../src/endpoint_host.rs#L1036) | receiver-type-required |
| `create_session` | `identity_profile.is_some` | [1036](../../src/endpoint_host.rs#L1036) | receiver-type-required |
| `create_session` | `(self.clock)().map_err` | [1039](../../src/endpoint_host.rs#L1039) | receiver-type-required |
| `create_session` | `(self.clock)` | [1039](../../src/endpoint_host.rs#L1039) | external-constructor-callback-or-unresolved |
| `create_session` | `request_hash` | [1040](../../src/endpoint_host.rs#L1040) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `create_session` | `self             .management             .create_session(SessionCreateOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 requested_session_id: session_id,                 workspace_id,                 cwd,                 identity_profile,                 preset,                 preset_mcp: self                     .preset_mcp                     .as_deref()                     .map(&#124;source&#124; source as &dyn endpoint::PresetMcpSource),                 user_agent_dir: &self.user_agent_dir,                 started_at: &timestamp,                 principal: &self.principal,             })             .map_err` | [1041](../../src/endpoint_host.rs#L1041) | receiver-type-required |
| `create_session` | `self             .management             .create_session` | [1041](../../src/endpoint_host.rs#L1041) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref()                     .map` | [1051](../../src/endpoint_host.rs#L1051) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref` | [1051](../../src/endpoint_host.rs#L1051) | receiver-type-required |
| `create_session` | `mark_management_handoff` | [1060](../../src/endpoint_host.rs#L1060) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `create_session` | `to_ijson` | [1061](../../src/endpoint_host.rs#L1061) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `draft_models` | `require_object_fields` | [1065](../../src/endpoint_host.rs#L1065) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `draft_models` | `profile::ConfigRepository::open(&self.storage_root).map_err` | [1067](../../src/endpoint_host.rs#L1067) | receiver-type-required |
| `draft_models` | `profile::ConfigRepository::open` | [1067](../../src/endpoint_host.rs#L1067) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `draft_models` | `internal_failure` | [1067](../../src/endpoint_host.rs#L1067), [1068](../../src/endpoint_host.rs#L1068), [1069](../../src/endpoint_host.rs#L1069) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `draft_models` | `repository.providers().map_err` | [1068](../../src/endpoint_host.rs#L1068) | receiver-type-required |
| `draft_models` | `repository.providers` | [1068](../../src/endpoint_host.rs#L1068) | receiver-type-required |
| `draft_models` | `repository.settings().map_err` | [1069](../../src/endpoint_host.rs#L1069) | receiver-type-required |
| `draft_models` | `repository.settings` | [1069](../../src/endpoint_host.rs#L1069) | receiver-type-required |
| `draft_models` | `String::new` | [1084](../../src/endpoint_host.rs#L1084), [1085](../../src/endpoint_host.rs#L1085) | external-constructor-callback-or-unresolved |
| `draft_models` | `Default::default` | [1089](../../src/endpoint_host.rs#L1089) | external-constructor-callback-or-unresolved |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness("", &config)             .map_err` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `draft_models` | `map_production_route_for` | [1101](../../src/endpoint_host.rs#L1101) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `draft_models` | `model_projection` | [1102](../../src/endpoint_host.rs#L1102) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1724) |
| `models` | `require_object_fields` | [1106](../../src/endpoint_host.rs#L1106) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `models` | `required_string` | [1107](../../src/endpoint_host.rs#L1107) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `models` | `self.provider_readiness.as_ref().ok_or_else` | [1108](../../src/endpoint_host.rs#L1108) | receiver-type-required |
| `models` | `self.provider_readiness.as_ref` | [1108](../../src/endpoint_host.rs#L1108) | receiver-type-required |
| `models` | `EndpointRpcFailure::new` | [1109](../../src/endpoint_host.rs#L1109) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `models` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1115](../../src/endpoint_host.rs#L1115) | receiver-type-required |
| `models` | `self             .endpoint             .session_config_snapshot` | [1115](../../src/endpoint_host.rs#L1115) | receiver-type-required |
| `models` | `map_endpoint` | [1118](../../src/endpoint_host.rs#L1118) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2596) |
| `models` | `Some` | [1118](../../src/endpoint_host.rs#L1118) | external-constructor-callback-or-unresolved |
| `models` | `authority             .readiness(session_id, &config)             .map_err` | [1119](../../src/endpoint_host.rs#L1119) | receiver-type-required |
| `models` | `authority             .readiness` | [1119](../../src/endpoint_host.rs#L1119) | receiver-type-required |
| `models` | `map_production_route_for` | [1121](../../src/endpoint_host.rs#L1121) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `models` | `model_projection` | [1122](../../src/endpoint_host.rs#L1122) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1724) |
| `select_model` | `require_object_fields` | [1130](../../src/endpoint_host.rs#L1130) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `select_model` | `required_string` | [1135](../../src/endpoint_host.rs#L1135), [1136](../../src/endpoint_host.rs#L1136), [1137](../../src/endpoint_host.rs#L1137) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `select_model` | `optional_string` | [1138](../../src/endpoint_host.rs#L1138) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2195) |
| `select_model` | `self.provider_readiness.as_ref().ok_or_else` | [1139](../../src/endpoint_host.rs#L1139) | receiver-type-required |
| `select_model` | `self.provider_readiness.as_ref` | [1139](../../src/endpoint_host.rs#L1139) | receiver-type-required |
| `select_model` | `EndpointRpcFailure::new` | [1140](../../src/endpoint_host.rs#L1140), [1153](../../src/endpoint_host.rs#L1153) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `select_model` | `self.admission_gate` | [1146](../../src/endpoint_host.rs#L1146) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1471) |
| `select_model` | `gate.lock().map_err` | [1147](../../src/endpoint_host.rs#L1147) | receiver-type-required |
| `select_model` | `gate.lock` | [1147](../../src/endpoint_host.rs#L1147) | receiver-type-required |
| `select_model` | `internal_failure` | [1147](../../src/endpoint_host.rs#L1147) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `select_model` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1148](../../src/endpoint_host.rs#L1148) | receiver-type-required |
| `select_model` | `self             .endpoint             .session_config_snapshot` | [1148](../../src/endpoint_host.rs#L1148) | receiver-type-required |
| `select_model` | `map_endpoint` | [1151](../../src/endpoint_host.rs#L1151) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2596) |
| `select_model` | `Some` | [1151](../../src/endpoint_host.rs#L1151) | external-constructor-callback-or-unresolved |
| `select_model` | `model_is_configured` | [1152](../../src/endpoint_host.rs#L1152) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1820) |
| `select_model` | `Err` | [1153](../../src/endpoint_host.rs#L1153) | external-constructor-callback-or-unresolved |
| `select_model` | `(self.clock)().map_err` | [1159](../../src/endpoint_host.rs#L1159) | receiver-type-required |
| `select_model` | `(self.clock)` | [1159](../../src/endpoint_host.rs#L1159) | external-constructor-callback-or-unresolved |
| `select_model` | `request_hash` | [1160](../../src/endpoint_host.rs#L1160) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `select_model` | `self             .management             .select_model(SelectModelOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 provider,                 model,                 reasoning_effort: effort,                 started_at: &timestamp,             })             .map_err` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `select_model` | `self             .management             .select_model` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `select_model` | `mark_management_handoff` | [1173](../../src/endpoint_host.rs#L1173) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `select_model` | `authority             .config_mutation_succeeded(session_id)             .map_err` | [1174](../../src/endpoint_host.rs#L1174) | receiver-type-required |
| `select_model` | `authority             .config_mutation_succeeded` | [1174](../../src/endpoint_host.rs#L1174) | receiver-type-required |
| `select_model` | `map_production_route_for` | [1176](../../src/endpoint_host.rs#L1176) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `select_model` | `Value::String` | [1179](../../src/endpoint_host.rs#L1179) | external-constructor-callback-or-unresolved |
| `select_model` | `to_ijson` | [1181](../../src/endpoint_host.rs#L1181) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `cancel` | `require_object_fields` | [1189](../../src/endpoint_host.rs#L1189) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `cancel` | `required_string` | [1190](../../src/endpoint_host.rs#L1190) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `cancel` | `self.delivery_authority.as_ref().ok_or_else` | [1191](../../src/endpoint_host.rs#L1191) | receiver-type-required |
| `cancel` | `self.delivery_authority.as_ref` | [1191](../../src/endpoint_host.rs#L1191) | receiver-type-required |
| `cancel` | `EndpointRpcFailure::new` | [1192](../../src/endpoint_host.rs#L1192) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `cancel` | `self.admission_gate` | [1198](../../src/endpoint_host.rs#L1198) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1471) |
| `cancel` | `gate.lock().map_err` | [1199](../../src/endpoint_host.rs#L1199) | receiver-type-required |
| `cancel` | `gate.lock` | [1199](../../src/endpoint_host.rs#L1199) | receiver-type-required |
| `cancel` | `internal_failure` | [1199](../../src/endpoint_host.rs#L1199) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `cancel` | `(self.clock)().map_err` | [1200](../../src/endpoint_host.rs#L1200) | receiver-type-required |
| `cancel` | `(self.clock)` | [1200](../../src/endpoint_host.rs#L1200) | external-constructor-callback-or-unresolved |
| `cancel` | `endpoint_origin` | [1201](../../src/endpoint_host.rs#L1201) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2278) |
| `cancel` | `authority             .cancel(session_id, &timestamp, &origin)             .map_err` | [1207](../../src/endpoint_host.rs#L1207) | receiver-type-required |
| `cancel` | `authority             .cancel` | [1207](../../src/endpoint_host.rs#L1207) | receiver-type-required |
| `cancel` | `map_production_route_for` | [1209](../../src/endpoint_host.rs#L1209) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `cancel` | `mark_event_handoff` | [1210](../../src/endpoint_host.rs#L1210) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2264) |
| `cancel` | `to_ijson` | [1211](../../src/endpoint_host.rs#L1211) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `rename_session` | `require_object_fields` | [1219](../../src/endpoint_host.rs#L1219) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `rename_session` | `required_string` | [1220](../../src/endpoint_host.rs#L1220) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `rename_session` | `required_string_allow_whitespace` | [1221](../../src/endpoint_host.rs#L1221) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2174) |
| `rename_session` | `validate_session_title(title).map_err` | [1222](../../src/endpoint_host.rs#L1222) | receiver-type-required |
| `rename_session` | `validate_session_title` | [1222](../../src/endpoint_host.rs#L1222) | [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2293) |
| `rename_session` | `EndpointRpcFailure::new` | [1223](../../src/endpoint_host.rs#L1223), [1230](../../src/endpoint_host.rs#L1230) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `rename_session` | `self.delivery_authority.as_ref().ok_or_else` | [1229](../../src/endpoint_host.rs#L1229) | receiver-type-required |
| `rename_session` | `self.delivery_authority.as_ref` | [1229](../../src/endpoint_host.rs#L1229) | receiver-type-required |
| `rename_session` | `self.admission_gate` | [1236](../../src/endpoint_host.rs#L1236) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1471) |
| `rename_session` | `gate.lock().map_err` | [1237](../../src/endpoint_host.rs#L1237) | receiver-type-required |
| `rename_session` | `gate.lock` | [1237](../../src/endpoint_host.rs#L1237) | receiver-type-required |
| `rename_session` | `internal_failure` | [1237](../../src/endpoint_host.rs#L1237) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `rename_session` | `(self.clock)().map_err` | [1238](../../src/endpoint_host.rs#L1238) | receiver-type-required |
| `rename_session` | `(self.clock)` | [1238](../../src/endpoint_host.rs#L1238) | external-constructor-callback-or-unresolved |
| `rename_session` | `endpoint_origin` | [1239](../../src/endpoint_host.rs#L1239) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2278) |
| `rename_session` | `authority             .rename(session_id, &timestamp, &origin, title)             .map_err` | [1245](../../src/endpoint_host.rs#L1245) | receiver-type-required |
| `rename_session` | `authority             .rename` | [1245](../../src/endpoint_host.rs#L1245) | receiver-type-required |
| `rename_session` | `map_production_route_for` | [1247](../../src/endpoint_host.rs#L1247) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `rename_session` | `mark_event_handoff` | [1248](../../src/endpoint_host.rs#L1248) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2264) |
| `rename_session` | `to_ijson` | [1249](../../src/endpoint_host.rs#L1249) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `update_queue` | `require_object_fields` | [1257](../../src/endpoint_host.rs#L1257) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `update_queue` | `required_string` | [1262](../../src/endpoint_host.rs#L1262), [1263](../../src/endpoint_host.rs#L1263) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter(&#124;value&#124; *value > 0)             .ok_or_else` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `update_queue` | `value.parse::<u64>().ok` | [1266](../../src/endpoint_host.rs#L1266) | receiver-type-required |
| `update_queue` | `value.parse::<u64>` | [1266](../../src/endpoint_host.rs#L1266) | receiver-type-required |
| `update_queue` | `invalid` | [1268](../../src/endpoint_host.rs#L1268), [1269](../../src/endpoint_host.rs#L1269), [1298](../../src/endpoint_host.rs#L1298) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `update_queue` | `queue_action` | [1269](../../src/endpoint_host.rs#L1269) | [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2327) |
| `update_queue` | `payload.get("action").ok_or_else` | [1269](../../src/endpoint_host.rs#L1269) | receiver-type-required |
| `update_queue` | `payload.get` | [1269](../../src/endpoint_host.rs#L1269) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref().ok_or_else` | [1270](../../src/endpoint_host.rs#L1270) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref` | [1270](../../src/endpoint_host.rs#L1270) | receiver-type-required |
| `update_queue` | `EndpointRpcFailure::new` | [1271](../../src/endpoint_host.rs#L1271) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `update_queue` | `self.admission_gate` | [1277](../../src/endpoint_host.rs#L1277) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1471) |
| `update_queue` | `gate.lock().map_err` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `gate.lock` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `internal_failure` | [1278](../../src/endpoint_host.rs#L1278), [1324](../../src/endpoint_host.rs#L1324) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `update_queue` | `request.rpc_id.clone` | [1280](../../src/endpoint_host.rs#L1280), [1281](../../src/endpoint_host.rs#L1281) | receiver-type-required |
| `update_queue` | `endpoint_origin` | [1283](../../src/endpoint_host.rs#L1283), [1289](../../src/endpoint_host.rs#L1289) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2278) |
| `update_queue` | `action.with_origin` | [1289](../../src/endpoint_host.rs#L1289) | receiver-type-required |
| `update_queue` | `transaction             .validate()             .map_err` | [1296](../../src/endpoint_host.rs#L1296) | receiver-type-required |
| `update_queue` | `transaction             .validate` | [1296](../../src/endpoint_host.rs#L1296) | receiver-type-required |
| `update_queue` | `to_ijson` | [1299](../../src/endpoint_host.rs#L1299), [1333](../../src/endpoint_host.rs#L1333) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `update_queue` | `request_hash` | [1300](../../src/endpoint_host.rs#L1300) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `update_queue` | `(self.clock)().map_err` | [1301](../../src/endpoint_host.rs#L1301) | receiver-type-required |
| `update_queue` | `(self.clock)` | [1301](../../src/endpoint_host.rs#L1301) | external-constructor-callback-or-unresolved |
| `update_queue` | `self             .management             .prepare_queue_transaction(QueueTransactionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 target_seq,                 action: &action,                 retract_origin: &transaction.retract_origin,                 replacement_origin: transaction.replacement_origin(),                 asset_digests: &[],                 started_at: &timestamp,             })             .map_err` | [1302](../../src/endpoint_host.rs#L1302) | receiver-type-required |
| `update_queue` | `self             .management             .prepare_queue_transaction` | [1302](../../src/endpoint_host.rs#L1302) | receiver-type-required |
| `update_queue` | `transaction.replacement_origin` | [1311](../../src/endpoint_host.rs#L1311) | receiver-type-required |
| `update_queue` | `authority                     .execute(session_id, &transaction)                     .map_err` | [1319](../../src/endpoint_host.rs#L1319) | receiver-type-required |
| `update_queue` | `authority                     .execute` | [1319](../../src/endpoint_host.rs#L1319) | receiver-type-required |
| `update_queue` | `map_production_route_for` | [1321](../../src/endpoint_host.rs#L1321) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `update_queue` | `result                     .validate_for(&transaction)                     .map_err` | [1322](../../src/endpoint_host.rs#L1322) | receiver-type-required |
| `update_queue` | `result                     .validate_for` | [1322](../../src/endpoint_host.rs#L1322) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction(&request.rpc_id, queue_decision(&result.outcome))                     .map_err` | [1325](../../src/endpoint_host.rs#L1325) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction` | [1325](../../src/endpoint_host.rs#L1325) | receiver-type-required |
| `update_queue` | `queue_decision` | [1326](../../src/endpoint_host.rs#L1326) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2401) |
| `update_queue` | `mark_management_handoff` | [1332](../../src/endpoint_host.rs#L1332) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `update_queue` | `Err` | [1336](../../src/endpoint_host.rs#L1336) | external-constructor-callback-or-unresolved |
| `update_queue` | `map_persisted_queue_rejection` | [1336](../../src/endpoint_host.rs#L1336) | [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2426) |
| `prompt` | `require_object_fields` | [1346](../../src/endpoint_host.rs#L1346) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `prompt` | `required_string` | [1351](../../src/endpoint_host.rs#L1351), [1352](../../src/endpoint_host.rs#L1352) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `prompt` | `Err` | [1355](../../src/endpoint_host.rs#L1355) | external-constructor-callback-or-unresolved |
| `prompt` | `invalid` | [1355](../../src/endpoint_host.rs#L1355), [1364](../../src/endpoint_host.rs#L1364), [1366](../../src/endpoint_host.rs#L1366) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `prompt` | `optional_string` | [1357](../../src/endpoint_host.rs#L1357) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2195) |
| `prompt` | `validate_time_zone` | [1358](../../src/endpoint_host.rs#L1358) | [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2440) |
| `prompt` | `serde_json::from_value(             payload                 .get("content")                 .cloned()                 .ok_or_else(&#124;&#124; invalid("content is required"))?,         )         .map_err` | [1360](../../src/endpoint_host.rs#L1360) | receiver-type-required |
| `prompt` | `serde_json::from_value` | [1360](../../src/endpoint_host.rs#L1360) | external-constructor-callback-or-unresolved |
| `prompt` | `payload                 .get("content")                 .cloned()                 .ok_or_else` | [1361](../../src/endpoint_host.rs#L1361) | receiver-type-required |
| `prompt` | `payload                 .get("content")                 .cloned` | [1361](../../src/endpoint_host.rs#L1361) | receiver-type-required |
| `prompt` | `payload                 .get` | [1361](../../src/endpoint_host.rs#L1361) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref().ok_or_else` | [1367](../../src/endpoint_host.rs#L1367) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref` | [1367](../../src/endpoint_host.rs#L1367) | receiver-type-required |
| `prompt` | `EndpointRpcFailure::new` | [1368](../../src/endpoint_host.rs#L1368) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `prompt` | `self.input_admission.with_active_session` | [1374](../../src/endpoint_host.rs#L1374) | receiver-type-required |
| `prompt` | `map_production_route_for` | [1376](../../src/endpoint_host.rs#L1376), [1391](../../src/endpoint_host.rs#L1391) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts(session_id, &parts)                     .map_err` | [1378](../../src/endpoint_host.rs#L1378) | receiver-type-required |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts` | [1378](../../src/endpoint_host.rs#L1378) | receiver-type-required |
| `prompt` | `(self.clock)().map_err` | [1382](../../src/endpoint_host.rs#L1382) | receiver-type-required |
| `prompt` | `(self.clock)` | [1382](../../src/endpoint_host.rs#L1382) | external-constructor-callback-or-unresolved |
| `prompt` | `endpoint_origin` | [1383](../../src/endpoint_host.rs#L1383) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2278) |
| `prompt` | `authority                     .prompt(session_id, &timestamp, &origin, &materialized, steer)                     .map_err` | [1389](../../src/endpoint_host.rs#L1389) | receiver-type-required |
| `prompt` | `authority                     .prompt` | [1389](../../src/endpoint_host.rs#L1389) | receiver-type-required |
| `prompt` | `mark_event_handoff` | [1392](../../src/endpoint_host.rs#L1392) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2264) |
| `prompt` | `to_ijson` | [1393](../../src/endpoint_host.rs#L1393) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `attachment` | `require_object_fields` | [1399](../../src/endpoint_host.rs#L1399) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `attachment` | `required_string` | [1404](../../src/endpoint_host.rs#L1404), [1405](../../src/endpoint_host.rs#L1405) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `attachment` | `self.ensure_active_session` | [1406](../../src/endpoint_host.rs#L1406) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1478) |
| `attachment` | `self             .attachments             .read_authorized(session_id, attachment_id)             .map_err` | [1407](../../src/endpoint_host.rs#L1407) | receiver-type-required |
| `attachment` | `self             .attachments             .read_authorized` | [1407](../../src/endpoint_host.rs#L1407) | receiver-type-required |
| `attachment` | `to_ijson` | [1411](../../src/endpoint_host.rs#L1411) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `fork_session` | `require_object_fields` | [1419](../../src/endpoint_host.rs#L1419) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `fork_session` | `required_string` | [1424](../../src/endpoint_host.rs#L1424) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `fork_session` | `optional_u64` | [1425](../../src/endpoint_host.rs#L1425) | [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2222) |
| `fork_session` | `optional_bool(payload, "ephemeral")?.unwrap_or` | [1426](../../src/endpoint_host.rs#L1426) | receiver-type-required |
| `fork_session` | `optional_bool` | [1426](../../src/endpoint_host.rs#L1426) | [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2185) |
| `fork_session` | `(self.clock)().map_err` | [1427](../../src/endpoint_host.rs#L1427) | receiver-type-required |
| `fork_session` | `(self.clock)` | [1427](../../src/endpoint_host.rs#L1427) | external-constructor-callback-or-unresolved |
| `fork_session` | `request_hash` | [1428](../../src/endpoint_host.rs#L1428) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `fork_session` | `self             .management             .fork_session(ForkSessionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 source_session_id: session_id,                 at_endpoint_seq: at_seq,                 started_at: &timestamp,                 principal: &self.principal,                 ephemeral,             })             .map_err` | [1429](../../src/endpoint_host.rs#L1429) | receiver-type-required |
| `fork_session` | `self             .management             .fork_session` | [1429](../../src/endpoint_host.rs#L1429) | receiver-type-required |
| `fork_session` | `mark_management_handoff` | [1441](../../src/endpoint_host.rs#L1441) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `fork_session` | `to_ijson` | [1442](../../src/endpoint_host.rs#L1442) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `discard_session` | `require_object_fields` | [1453](../../src/endpoint_host.rs#L1453) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `discard_session` | `required_string` | [1454](../../src/endpoint_host.rs#L1454) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2165) |
| `discard_session` | `self.input_admission.with_session_gate` | [1455](../../src/endpoint_host.rs#L1455) | receiver-type-required |
| `discard_session` | `map_production_route_for` | [1457](../../src/endpoint_host.rs#L1457) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1868) |
| `discard_session` | `(self.clock)().map_err` | [1459](../../src/endpoint_host.rs#L1459) | receiver-type-required |
| `discard_session` | `(self.clock)` | [1459](../../src/endpoint_host.rs#L1459) | external-constructor-callback-or-unresolved |
| `discard_session` | `request_hash` | [1460](../../src/endpoint_host.rs#L1460) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2236) |
| `discard_session` | `self                     .management                     .discard_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [1461](../../src/endpoint_host.rs#L1461) | receiver-type-required |
| `discard_session` | `self                     .management                     .discard_session` | [1461](../../src/endpoint_host.rs#L1461) | receiver-type-required |
| `discard_session` | `mark_management_handoff` | [1465](../../src/endpoint_host.rs#L1465) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2250) |
| `discard_session` | `to_ijson` | [1466](../../src/endpoint_host.rs#L1466) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `admission_gate` | `self.input_admission             .gates()             .gate(session_id)             .map_err` | [1472](../../src/endpoint_host.rs#L1472) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates()             .gate` | [1472](../../src/endpoint_host.rs#L1472) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates` | [1472](../../src/endpoint_host.rs#L1472) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id(session_id).map_err` | [1479](../../src/endpoint_host.rs#L1479) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id` | [1479](../../src/endpoint_host.rs#L1479) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `ensure_active_session` | `invalid` | [1479](../../src/endpoint_host.rs#L1479) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `ensure_active_session` | `self.storage_root.join("archive").join(session_id).is_dir` | [1480](../../src/endpoint_host.rs#L1480) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("archive").join` | [1480](../../src/endpoint_host.rs#L1480) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join` | [1480](../../src/endpoint_host.rs#L1480), [1487](../../src/endpoint_host.rs#L1487) | receiver-type-required |
| `ensure_active_session` | `Err` | [1481](../../src/endpoint_host.rs#L1481), [1488](../../src/endpoint_host.rs#L1488) | external-constructor-callback-or-unresolved |
| `ensure_active_session` | `EndpointRpcFailure::new` | [1481](../../src/endpoint_host.rs#L1481), [1488](../../src/endpoint_host.rs#L1488) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `ensure_active_session` | `self.storage_root.join("threads").join(session_id).is_dir` | [1487](../../src/endpoint_host.rs#L1487) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("threads").join` | [1487](../../src/endpoint_host.rs#L1487) | receiver-type-required |
| `ensure_active_session` | `Ok` | [1494](../../src/endpoint_host.rs#L1494) | external-constructor-callback-or-unresolved |
| `capabilities` | `Self::implemented_capabilities` | [1500](../../src/endpoint_host.rs#L1500) | external-constructor-callback-or-unresolved |
| `capabilities` | `self.routes.as_ref` | [1501](../../src/endpoint_host.rs#L1501) | receiver-type-required |
| `capabilities` | `capabilities.extend` | [1502](../../src/endpoint_host.rs#L1502) | receiver-type-required |
| `capabilities` | `routes.capabilities` | [1502](../../src/endpoint_host.rs#L1502) | receiver-type-required |
| `capabilities` | `self.provider_readiness.is_some` | [1504](../../src/endpoint_host.rs#L1504) | receiver-type-required |
| `capabilities` | `capabilities.insert` | [1505](../../src/endpoint_host.rs#L1505), [1506](../../src/endpoint_host.rs#L1506), [1507](../../src/endpoint_host.rs#L1507), [1510](../../src/endpoint_host.rs#L1510), [1511](../../src/endpoint_host.rs#L1511), [1512](../../src/endpoint_host.rs#L1512), [1515](../../src/endpoint_host.rs#L1515) | receiver-type-required |
| `capabilities` | `"session.models".to_owned` | [1505](../../src/endpoint_host.rs#L1505) | receiver-type-required |
| `capabilities` | `"models.list".to_owned` | [1506](../../src/endpoint_host.rs#L1506) | receiver-type-required |
| `capabilities` | `"session.selectModel".to_owned` | [1507](../../src/endpoint_host.rs#L1507) | receiver-type-required |
| `capabilities` | `self.delivery_authority.is_some` | [1509](../../src/endpoint_host.rs#L1509) | receiver-type-required |
| `capabilities` | `"session.prompt".to_owned` | [1510](../../src/endpoint_host.rs#L1510) | receiver-type-required |
| `capabilities` | `"session.cancel".to_owned` | [1511](../../src/endpoint_host.rs#L1511) | receiver-type-required |
| `capabilities` | `"session.rename".to_owned` | [1512](../../src/endpoint_host.rs#L1512) | receiver-type-required |
| `capabilities` | `self.queue_authority.is_some` | [1514](../../src/endpoint_host.rs#L1514) | receiver-type-required |
| `capabilities` | `"session.updateQueue".to_owned` | [1515](../../src/endpoint_host.rs#L1515) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref()                 .map_or_else` | [1522](../../src/endpoint_host.rs#L1522) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref` | [1522](../../src/endpoint_host.rs#L1522) | receiver-type-required |
| `extension_capabilities` | `routes.capabilities` | [1524](../../src/endpoint_host.rs#L1524) | receiver-type-required |
| `extension_capabilities` | `BTreeSet::new` | [1526](../../src/endpoint_host.rs#L1526) | external-constructor-callback-or-unresolved |
| `method_class` | `TEKES_UNARY_ROUTES.iter().find` | [1531](../../src/endpoint_host.rs#L1531) | receiver-type-required |
| `method_class` | `TEKES_UNARY_ROUTES.iter` | [1531](../../src/endpoint_host.rs#L1531) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then(&#124;routes&#124; routes.extension_method_class(method))                 .unwrap_or` | [1538](../../src/endpoint_host.rs#L1538) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then` | [1538](../../src/endpoint_host.rs#L1538) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref` | [1538](../../src/endpoint_host.rs#L1538) | receiver-type-required |
| `method_class` | `routes.extension_method_class` | [1541](../../src/endpoint_host.rs#L1541) | receiver-type-required |
| `validate_request` | `serde_json::from_slice(             &request                 .payload                 .canonical_bytes()                 .map_err(&#124;_&#124; HostFailure::InvalidRequest)?,         )         .map_err` | [1547](../../src/endpoint_host.rs#L1547) | receiver-type-required |
| `validate_request` | `serde_json::from_slice` | [1547](../../src/endpoint_host.rs#L1547) | external-constructor-callback-or-unresolved |
| `validate_request` | `request                 .payload                 .canonical_bytes()                 .map_err` | [1548](../../src/endpoint_host.rs#L1548) | receiver-type-required |
| `validate_request` | `request                 .payload                 .canonical_bytes` | [1548](../../src/endpoint_host.rs#L1548) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter()             .any` | [1554](../../src/endpoint_host.rs#L1554) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter` | [1554](../../src/endpoint_host.rs#L1554) | receiver-type-required |
| `validate_request` | `validate_production_payload(&request.method, &payload)                 .map_err` | [1558](../../src/endpoint_host.rs#L1558) | receiver-type-required |
| `validate_request` | `validate_production_payload` | [1558](../../src/endpoint_host.rs#L1558) | [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2575) |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload(&request.method, &payload)                 .map_err` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `validate_request` | `routes.capabilities().contains` | [1563](../../src/endpoint_host.rs#L1563) | receiver-type-required |
| `validate_request` | `routes.capabilities` | [1563](../../src/endpoint_host.rs#L1563) | receiver-type-required |
| `call` | `Box::pin` | [1571](../../src/endpoint_host.rs#L1571) | external-constructor-callback-or-unresolved |
| `call` | `std::future::ready` | [1571](../../src/endpoint_host.rs#L1571) | external-constructor-callback-or-unresolved |
| `call` | `self.execute` | [1571](../../src/endpoint_host.rs#L1571) | receiver-type-required |
| `open` | `root.as_ref` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `open` | `Ok` | [1587](../../src/endpoint_host.rs#L1587) | external-constructor-callback-or-unresolved |
| `open` | `ProductionEndpointHost::open` | [1588](../../src/endpoint_host.rs#L1588) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L577) |
| `open` | `EndpointDispatcher::new` | [1589](../../src/endpoint_host.rs#L1589) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open` | `RpcRegistry::open` | [1589](../../src/endpoint_host.rs#L1589) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open` | `EndpointSubscriptionHub::default` | [1590](../../src/endpoint_host.rs#L1590) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [1599](../../src/endpoint_host.rs#L1599) | receiver-type-required |
| `open_with_clock` | `Ok` | [1600](../../src/endpoint_host.rs#L1600) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `ProductionEndpointHost::open_with_clock` | [1601](../../src/endpoint_host.rs#L1601) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L592) |
| `open_with_clock` | `EndpointDispatcher::new` | [1602](../../src/endpoint_host.rs#L1602) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_clock` | `RpcRegistry::open` | [1602](../../src/endpoint_host.rs#L1602) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_clock` | `EndpointSubscriptionHub::default` | [1603](../../src/endpoint_host.rs#L1603) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `root.as_ref` | [1613](../../src/endpoint_host.rs#L1613) | receiver-type-required |
| `open_with_routes` | `Ok` | [1614](../../src/endpoint_host.rs#L1614) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `ProductionEndpointHost::open_with_routes` | [1615](../../src/endpoint_host.rs#L1615) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L601) |
| `open_with_routes` | `Some` | [1615](../../src/endpoint_host.rs#L1615) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `EndpointDispatcher::new` | [1616](../../src/endpoint_host.rs#L1616) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_routes` | `RpcRegistry::open` | [1616](../../src/endpoint_host.rs#L1616) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_routes` | `EndpointSubscriptionHub::default` | [1617](../../src/endpoint_host.rs#L1617) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `root.as_ref` | [1627](../../src/endpoint_host.rs#L1627) | receiver-type-required |
| `open_with_route_authority` | `Ok` | [1628](../../src/endpoint_host.rs#L1628) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `ProductionEndpointHost::open_with_route_authority` | [1629](../../src/endpoint_host.rs#L1629) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L677) |
| `open_with_route_authority` | `EndpointDispatcher::new` | [1638](../../src/endpoint_host.rs#L1638) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_route_authority` | `RpcRegistry::open` | [1638](../../src/endpoint_host.rs#L1638) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_route_authority` | `EndpointSubscriptionHub::default` | [1639](../../src/endpoint_host.rs#L1639) | external-constructor-callback-or-unresolved |
| `open_with_authorities` | `Self::open_with_full_authorities` | [1650](../../src/endpoint_host.rs#L1650) | [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1661) |
| `open_with_full_authorities` | `root.as_ref` | [1670](../../src/endpoint_host.rs#L1670) | receiver-type-required |
| `open_with_full_authorities` | `Ok` | [1671](../../src/endpoint_host.rs#L1671) | external-constructor-callback-or-unresolved |
| `open_with_full_authorities` | `ProductionEndpointHost::open_with_full_authorities` | [1672](../../src/endpoint_host.rs#L1672) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L628) |
| `open_with_full_authorities` | `EndpointDispatcher::new` | [1681](../../src/endpoint_host.rs#L1681) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_full_authorities` | `RpcRegistry::open` | [1681](../../src/endpoint_host.rs#L1681) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_full_authorities` | `EndpointSubscriptionHub::default` | [1682](../../src/endpoint_host.rs#L1682) | external-constructor-callback-or-unresolved |
| `dispatch` | `Ok` | [1690](../../src/endpoint_host.rs#L1690) | external-constructor-callback-or-unresolved |
| `dispatch` | `self.dispatcher.dispatch` | [1690](../../src/endpoint_host.rs#L1690) | receiver-type-required |
| `admission_gates` | `self.host.admission_gates` | [1705](../../src/endpoint_host.rs#L1705) | receiver-type-required |
| `has_incomplete_management_operation` | `self.host.has_incomplete_management_operation` | [1712](../../src/endpoint_host.rs#L1712) | receiver-type-required |
| `success` | `Some` | [1719](../../src/endpoint_host.rs#L1719) | external-constructor-callback-or-unresolved |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or(config.settings.default_provider.as_deref())         .unwrap_or_default` | [1728](../../src/endpoint_host.rs#L1728) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or` | [1728](../../src/endpoint_host.rs#L1728) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or` | [1728](../../src/endpoint_host.rs#L1728) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map` | [1728](../../src/endpoint_host.rs#L1728), [1735](../../src/endpoint_host.rs#L1735) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref` | [1728](../../src/endpoint_host.rs#L1728), [1735](../../src/endpoint_host.rs#L1735), [1742](../../src/endpoint_host.rs#L1742) | receiver-type-required |
| `model_projection` | `settings.provider.as_str` | [1731](../../src/endpoint_host.rs#L1731) | receiver-type-required |
| `model_projection` | `config.workspace.policy.provider.as_deref` | [1732](../../src/endpoint_host.rs#L1732) | receiver-type-required |
| `model_projection` | `config.settings.default_provider.as_deref` | [1733](../../src/endpoint_host.rs#L1733) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or(config.settings.default_model.as_deref())         .unwrap_or_default` | [1735](../../src/endpoint_host.rs#L1735) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or` | [1735](../../src/endpoint_host.rs#L1735) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or` | [1735](../../src/endpoint_host.rs#L1735) | receiver-type-required |
| `model_projection` | `settings.model.as_str` | [1738](../../src/endpoint_host.rs#L1738) | receiver-type-required |
| `model_projection` | `config.workspace.policy.model.as_deref` | [1739](../../src/endpoint_host.rs#L1739) | receiver-type-required |
| `model_projection` | `config.settings.default_model.as_deref` | [1740](../../src/endpoint_host.rs#L1740) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .and_then` | [1742](../../src/endpoint_host.rs#L1742) | receiver-type-required |
| `model_projection` | `settings.reasoning_effort.as_deref` | [1745](../../src/endpoint_host.rs#L1745) | receiver-type-required |
| `model_projection` | `Vec::new` | [1746](../../src/endpoint_host.rs#L1746), [1747](../../src/endpoint_host.rs#L1747), [1753](../../src/endpoint_host.rs#L1753) | external-constructor-callback-or-unresolved |
| `model_projection` | `provider.name.as_deref().unwrap_or` | [1749](../../src/endpoint_host.rs#L1749) | receiver-type-required |
| `model_projection` | `provider.name.as_deref` | [1749](../../src/endpoint_host.rs#L1749) | receiver-type-required |
| `model_projection` | `readiness             .iter()             .find` | [1750](../../src/endpoint_host.rs#L1750) | receiver-type-required |
| `model_projection` | `readiness             .iter` | [1750](../../src/endpoint_host.rs#L1750) | receiver-type-required |
| `model_projection` | `provider.models.iter().filter` | [1754](../../src/endpoint_host.rs#L1754) | receiver-type-required |
| `model_projection` | `provider.models.iter` | [1754](../../src/endpoint_host.rs#L1754) | receiver-type-required |
| `model_projection` | `provider::resolve_profile(provider, configured).ok` | [1755](../../src/endpoint_host.rs#L1755) | receiver-type-required |
| `model_projection` | `provider::resolve_profile` | [1755](../../src/endpoint_host.rs#L1755) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_projection` | `profile                 .as_ref()                 .map(&#124;profile&#124; profile.reasoning_efforts().to_vec())                 .unwrap_or_default` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .map` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `profile                 .as_ref` | [1756](../../src/endpoint_host.rs#L1756), [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts().to_vec` | [1758](../../src/endpoint_host.rs#L1758) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts` | [1758](../../src/endpoint_host.rs#L1758) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .and_then` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort().map` | [1762](../../src/endpoint_host.rs#L1762) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort` | [1762](../../src/endpoint_host.rs#L1762) | receiver-type-required |
| `model_projection` | `efforts                 .iter()                 .any` | [1763](../../src/endpoint_host.rs#L1763) | receiver-type-required |
| `model_projection` | `efforts                 .iter` | [1763](../../src/endpoint_host.rs#L1763) | receiver-type-required |
| `model_projection` | `effort.is_empty` | [1765](../../src/endpoint_host.rs#L1765) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter().all` | [1765](../../src/endpoint_host.rs#L1765) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter` | [1765](../../src/endpoint_host.rs#L1765) | receiver-type-required |
| `model_projection` | `effort.as_bytes` | [1765](../../src/endpoint_host.rs#L1765) | receiver-type-required |
| `model_projection` | `Err` | [1767](../../src/endpoint_host.rs#L1767), [1770](../../src/endpoint_host.rs#L1770), [1776](../../src/endpoint_host.rs#L1776) | external-constructor-callback-or-unresolved |
| `model_projection` | `internal_failure` | [1767](../../src/endpoint_host.rs#L1767), [1770](../../src/endpoint_host.rs#L1770), [1776](../../src/endpoint_host.rs#L1776) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>().len` | [1769](../../src/endpoint_host.rs#L1769) | receiver-type-required |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>` | [1769](../../src/endpoint_host.rs#L1769) | receiver-type-required |
| `model_projection` | `efforts.iter` | [1769](../../src/endpoint_host.rs#L1769) | receiver-type-required |
| `model_projection` | `efforts.len` | [1769](../../src/endpoint_host.rs#L1769) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref()                 .is_some_and` | [1772](../../src/endpoint_host.rs#L1772) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref` | [1772](../../src/endpoint_host.rs#L1772) | receiver-type-required |
| `model_projection` | `efforts.contains` | [1774](../../src/endpoint_host.rs#L1774) | receiver-type-required |
| `model_projection` | `efforts.is_empty` | [1780](../../src/endpoint_host.rs#L1780) | receiver-type-required |
| `model_projection` | `default_effort.as_ref` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `Value::String` | [1787](../../src/endpoint_host.rs#L1787), [1809](../../src/endpoint_host.rs#L1809) | external-constructor-callback-or-unresolved |
| `model_projection` | `default.clone` | [1787](../../src/endpoint_host.rs#L1787) | receiver-type-required |
| `model_projection` | `models.push` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `groups.push` | [1792](../../src/endpoint_host.rs#L1792) | receiver-type-required |
| `model_projection` | `runtime.map` | [1793](../../src/endpoint_host.rs#L1793) | receiver-type-required |
| `model_projection` | `failures.push` | [1795](../../src/endpoint_host.rs#L1795), [1800](../../src/endpoint_host.rs#L1800) | receiver-type-required |
| `model_projection` | `effort.to_owned` | [1809](../../src/endpoint_host.rs#L1809) | receiver-type-required |
| `model_projection` | `model_is_configured` | [1811](../../src/endpoint_host.rs#L1811) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1820) |
| `model_projection` | `to_ijson` | [1812](../../src/endpoint_host.rs#L1812) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2232) |
| `model_is_configured` | `config         .providers         .providers         .iter()         .find` | [1826](../../src/endpoint_host.rs#L1826) | receiver-type-required |
| `model_is_configured` | `config         .providers         .providers         .iter` | [1826](../../src/endpoint_host.rs#L1826) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter()         .find` | [1834](../../src/endpoint_host.rs#L1834) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter` | [1834](../../src/endpoint_host.rs#L1834) | receiver-type-required |
| `model_is_configured` | `effort.is_none_or` | [1841](../../src/endpoint_host.rs#L1841) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile(provider, model).is_ok_and` | [1842](../../src/endpoint_host.rs#L1842) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile` | [1842](../../src/endpoint_host.rs#L1842) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter()                 .any` | [1843](../../src/endpoint_host.rs#L1843) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter` | [1843](../../src/endpoint_host.rs#L1843) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts` | [1843](../../src/endpoint_host.rs#L1843) | receiver-type-required |
| `map_production_route_for` | `production_failure_is_exact_for` | [1869](../../src/endpoint_host.rs#L1869) | [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1902) |
| `map_production_route_for` | `sanitized_route_log_message` | [1870](../../src/endpoint_host.rs#L1870) | [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1882) |
| `map_production_route_for` | `internal_failure` | [1877](../../src/endpoint_host.rs#L1877) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `map_production_route_for` | `map_production_route` | [1879](../../src/endpoint_host.rs#L1879) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1860) |
| `sanitized_route_log_message` | `message.len().min` | [1883](../../src/endpoint_host.rs#L1883) | receiver-type-required |
| `sanitized_route_log_message` | `message.len` | [1883](../../src/endpoint_host.rs#L1883) | receiver-type-required |
| `sanitized_route_log_message` | `message.is_char_boundary` | [1884](../../src/endpoint_host.rs#L1884) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse(         &serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect("route log message is JSON"),     )     .expect` | [1887](../../src/endpoint_host.rs#L1887) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse` | [1887](../../src/endpoint_host.rs#L1887) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `sanitized_route_log_message` | `serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect` | [1888](../../src/endpoint_host.rs#L1888) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_vec` | [1888](../../src/endpoint_host.rs#L1888) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `tools::SecretScanner::default().scan` | [1892](../../src/endpoint_host.rs#L1892) | receiver-type-required |
| `sanitized_route_log_message` | `tools::SecretScanner::default` | [1892](../../src/endpoint_host.rs#L1892) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `"<withheld>".to_owned` | [1894](../../src/endpoint_host.rs#L1894), [1899](../../src/endpoint_host.rs#L1899) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then(&#124;value&#124; value["detail"].as_str().map(str::to_owned))         .unwrap_or_else` | [1896](../../src/endpoint_host.rs#L1896) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then` | [1896](../../src/endpoint_host.rs#L1896) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok` | [1896](../../src/endpoint_host.rs#L1896) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value` | [1896](../../src/endpoint_host.rs#L1896) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `value["detail"].as_str().map` | [1898](../../src/endpoint_host.rs#L1898) | receiver-type-required |
| `sanitized_route_log_message` | `value["detail"].as_str` | [1898](../../src/endpoint_host.rs#L1898) | receiver-type-required |
| `production_failure_is_exact_for` | `route_allows_error` | [1906](../../src/endpoint_host.rs#L1906) | [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2044) |
| `production_failure_is_exact_for` | `route_error_is_exact` | [1906](../../src/endpoint_host.rs#L1906) | [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1909) |
| `route_error_is_exact` | `route_error_message` | [1910](../../src/endpoint_host.rs#L1910) | [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L1992) |
| `route_error_is_exact` | `error.details.canonical_bytes` | [1916](../../src/endpoint_host.rs#L1916) | receiver-type-required |
| `route_error_is_exact` | `serde_json::from_slice::<Value>` | [1919](../../src/endpoint_host.rs#L1919) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `error.code.as_str` | [1923](../../src/endpoint_host.rs#L1923) | receiver-type-required |
| `route_error_is_exact` | `details.is_empty` | [1924](../../src/endpoint_host.rs#L1924) | receiver-type-required |
| `route_error_is_exact` | `exact_string_fields` | [1925](../../src/endpoint_host.rs#L1925), [1927](../../src/endpoint_host.rs#L1927), [1930](../../src/endpoint_host.rs#L1930), [1933](../../src/endpoint_host.rs#L1933), [1936](../../src/endpoint_host.rs#L1936), [1938](../../src/endpoint_host.rs#L1938), [1939](../../src/endpoint_host.rs#L1939), [1941](../../src/endpoint_host.rs#L1941), [1943](../../src/endpoint_host.rs#L1943), [1955](../../src/endpoint_host.rs#L1955), [1976](../../src/endpoint_host.rs#L1976), [1978](../../src/endpoint_host.rs#L1978) | [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2023) |
| `route_error_is_exact` | `details.len` | [1945](../../src/endpoint_host.rs#L1945), [1952](../../src/endpoint_host.rs#L1952), [1958](../../src/endpoint_host.rs#L1958) | receiver-type-required |
| `route_error_is_exact` | `details.get("sessionId").is_some_and` | [1946](../../src/endpoint_host.rs#L1946) | receiver-type-required |
| `route_error_is_exact` | `details.get` | [1946](../../src/endpoint_host.rs#L1946), [1952](../../src/endpoint_host.rs#L1952) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("atSeq")                     .is_some_and` | [1947](../../src/endpoint_host.rs#L1947) | receiver-type-required |
| `route_error_is_exact` | `details                     .get` | [1947](../../src/endpoint_host.rs#L1947), [1959](../../src/endpoint_host.rs#L1959), [1979](../../src/endpoint_host.rs#L1979) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64().is_some` | [1949](../../src/endpoint_host.rs#L1949) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64` | [1949](../../src/endpoint_host.rs#L1949) | receiver-type-required |
| `route_error_is_exact` | `details.get("limit").and_then` | [1952](../../src/endpoint_host.rs#L1952) | receiver-type-required |
| `route_error_is_exact` | `Some` | [1952](../../src/endpoint_host.rs#L1952) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then(Value::as_str)                     .is_some_and` | [1959](../../src/endpoint_host.rs#L1959), [1979](../../src/endpoint_host.rs#L1979) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then` | [1959](../../src/endpoint_host.rs#L1959), [1979](../../src/endpoint_host.rs#L1979) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter()                             .any` | [1983](../../src/endpoint_host.rs#L1983) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter` | [1983](../../src/endpoint_host.rs#L1983) | receiver-type-required |
| `route_error_is_exact` | `known.as_str` | [1985](../../src/endpoint_host.rs#L1985) | receiver-type-required |
| `route_error_message` | `Some` | [1993](../../src/endpoint_host.rs#L1993) | external-constructor-callback-or-unresolved |
| `exact_string_fields` | `required         .iter()         .any` | [2028](../../src/endpoint_host.rs#L2028) | receiver-type-required |
| `exact_string_fields` | `required         .iter` | [2028](../../src/endpoint_host.rs#L2028) | receiver-type-required |
| `exact_string_fields` | `details.get(*key).is_some_and` | [2030](../../src/endpoint_host.rs#L2030), [2033](../../src/endpoint_host.rs#L2033) | receiver-type-required |
| `exact_string_fields` | `details.get` | [2030](../../src/endpoint_host.rs#L2030), [2033](../../src/endpoint_host.rs#L2033) | receiver-type-required |
| `exact_string_fields` | `optional             .iter()             .any` | [2031](../../src/endpoint_host.rs#L2031) | receiver-type-required |
| `exact_string_fields` | `optional             .iter` | [2031](../../src/endpoint_host.rs#L2031) | receiver-type-required |
| `exact_string_fields` | `value.is_string` | [2033](../../src/endpoint_host.rs#L2033) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter(&#124;key&#124; details.contains_key(**key))         .count` | [2037](../../src/endpoint_host.rs#L2037) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter` | [2037](../../src/endpoint_host.rs#L2037) | receiver-type-required |
| `exact_string_fields` | `optional         .iter` | [2037](../../src/endpoint_host.rs#L2037) | receiver-type-required |
| `exact_string_fields` | `details.contains_key` | [2039](../../src/endpoint_host.rs#L2039) | receiver-type-required |
| `exact_string_fields` | `details.len` | [2041](../../src/endpoint_host.rs#L2041) | receiver-type-required |
| `exact_string_fields` | `required.len` | [2041](../../src/endpoint_host.rs#L2041) | receiver-type-required |
| `internal_failure` | `EndpointRpcFailure::new` | [2150](../../src/endpoint_host.rs#L2150) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `failure` | `Some` | [2157](../../src/endpoint_host.rs#L2157) | external-constructor-callback-or-unresolved |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [2166](../../src/endpoint_host.rs#L2166) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter` | [2166](../../src/endpoint_host.rs#L2166) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2166](../../src/endpoint_host.rs#L2166) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then` | [2166](../../src/endpoint_host.rs#L2166) | receiver-type-required |
| `required_string` | `value         .as_object` | [2166](../../src/endpoint_host.rs#L2166) | receiver-type-required |
| `required_string` | `object.get` | [2168](../../src/endpoint_host.rs#L2168) | receiver-type-required |
| `required_string` | `value.is_empty` | [2170](../../src/endpoint_host.rs#L2170) | receiver-type-required |
| `required_string` | `invalid` | [2171](../../src/endpoint_host.rs#L2171) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .ok_or_else` | [2178](../../src/endpoint_host.rs#L2178) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2178](../../src/endpoint_host.rs#L2178) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then` | [2178](../../src/endpoint_host.rs#L2178) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object` | [2178](../../src/endpoint_host.rs#L2178) | receiver-type-required |
| `required_string_allow_whitespace` | `object.get` | [2180](../../src/endpoint_host.rs#L2180) | receiver-type-required |
| `required_string_allow_whitespace` | `invalid` | [2182](../../src/endpoint_host.rs#L2182) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `optional_bool` | `value.as_object().and_then` | [2186](../../src/endpoint_host.rs#L2186) | receiver-type-required |
| `optional_bool` | `value.as_object` | [2186](../../src/endpoint_host.rs#L2186) | receiver-type-required |
| `optional_bool` | `object.get` | [2186](../../src/endpoint_host.rs#L2186) | receiver-type-required |
| `optional_bool` | `Ok` | [2187](../../src/endpoint_host.rs#L2187) | external-constructor-callback-or-unresolved |
| `optional_bool` | `value         .as_bool()         .map(Some)         .ok_or_else` | [2189](../../src/endpoint_host.rs#L2189) | receiver-type-required |
| `optional_bool` | `value         .as_bool()         .map` | [2189](../../src/endpoint_host.rs#L2189) | receiver-type-required |
| `optional_bool` | `value         .as_bool` | [2189](../../src/endpoint_host.rs#L2189) | receiver-type-required |
| `optional_bool` | `invalid` | [2192](../../src/endpoint_host.rs#L2192) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `optional_string` | `value.as_object().and_then` | [2196](../../src/endpoint_host.rs#L2196) | receiver-type-required |
| `optional_string` | `value.as_object` | [2196](../../src/endpoint_host.rs#L2196) | receiver-type-required |
| `optional_string` | `object.get` | [2196](../../src/endpoint_host.rs#L2196) | receiver-type-required |
| `optional_string` | `Ok` | [2197](../../src/endpoint_host.rs#L2197) | external-constructor-callback-or-unresolved |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map(Some)         .ok_or_else` | [2199](../../src/endpoint_host.rs#L2199) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map` | [2199](../../src/endpoint_host.rs#L2199) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter` | [2199](../../src/endpoint_host.rs#L2199) | receiver-type-required |
| `optional_string` | `value         .as_str` | [2199](../../src/endpoint_host.rs#L2199) | receiver-type-required |
| `optional_string` | `value.is_empty` | [2201](../../src/endpoint_host.rs#L2201) | receiver-type-required |
| `optional_string` | `invalid` | [2203](../../src/endpoint_host.rs#L2203) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `require_object_fields` | `value         .as_object()         .ok_or_else` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `require_object_fields` | `value         .as_object` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `require_object_fields` | `invalid` | [2213](../../src/endpoint_host.rs#L2213), [2217](../../src/endpoint_host.rs#L2217) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `require_object_fields` | `object.keys().any` | [2214](../../src/endpoint_host.rs#L2214) | receiver-type-required |
| `require_object_fields` | `object.keys` | [2214](../../src/endpoint_host.rs#L2214) | receiver-type-required |
| `require_object_fields` | `allowed.contains` | [2214](../../src/endpoint_host.rs#L2214) | receiver-type-required |
| `require_object_fields` | `key.as_str` | [2214](../../src/endpoint_host.rs#L2214) | receiver-type-required |
| `require_object_fields` | `required.iter().any` | [2215](../../src/endpoint_host.rs#L2215) | receiver-type-required |
| `require_object_fields` | `required.iter` | [2215](../../src/endpoint_host.rs#L2215) | receiver-type-required |
| `require_object_fields` | `object.contains_key` | [2215](../../src/endpoint_host.rs#L2215) | receiver-type-required |
| `require_object_fields` | `Err` | [2217](../../src/endpoint_host.rs#L2217) | external-constructor-callback-or-unresolved |
| `require_object_fields` | `Ok` | [2219](../../src/endpoint_host.rs#L2219) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value.as_object().and_then` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `optional_u64` | `value.as_object` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `optional_u64` | `object.get` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `optional_u64` | `Ok` | [2224](../../src/endpoint_host.rs#L2224) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value         .as_u64()         .map(Some)         .ok_or_else` | [2226](../../src/endpoint_host.rs#L2226) | receiver-type-required |
| `optional_u64` | `value         .as_u64()         .map` | [2226](../../src/endpoint_host.rs#L2226) | receiver-type-required |
| `optional_u64` | `value         .as_u64` | [2226](../../src/endpoint_host.rs#L2226) | receiver-type-required |
| `optional_u64` | `invalid` | [2229](../../src/endpoint_host.rs#L2229) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `to_ijson` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(internal_json)?).map_err` | [2233](../../src/endpoint_host.rs#L2233) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [2233](../../src/endpoint_host.rs#L2233) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value).map_err` | [2233](../../src/endpoint_host.rs#L2233) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [2233](../../src/endpoint_host.rs#L2233) | external-constructor-callback-or-unresolved |
| `request_hash` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)             .map_err` | [2238](../../src/endpoint_host.rs#L2238) | receiver-type-required |
| `request_hash` | `serde_json::from_slice` | [2238](../../src/endpoint_host.rs#L2238) | external-constructor-callback-or-unresolved |
| `request_hash` | `request.payload.canonical_bytes().map_err` | [2238](../../src/endpoint_host.rs#L2238) | receiver-type-required |
| `request_hash` | `request.payload.canonical_bytes` | [2238](../../src/endpoint_host.rs#L2238) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec(&json!({         "type":"client-request",         "rpcId":request.rpc_id,         "method":request.operation,         "payload":payload,     }))     .map_err` | [2240](../../src/endpoint_host.rs#L2240) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec` | [2240](../../src/endpoint_host.rs#L2240) | external-constructor-callback-or-unresolved |
| `request_hash` | `EndpointRpcFailure::new` | [2246](../../src/endpoint_host.rs#L2246) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `request_hash` | `Ok` | [2247](../../src/endpoint_host.rs#L2247) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "management".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "management-operation".to_owned(),                 id: request.rpc_id.clone(),                 seq: None,             }),         })         .map_err` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `mark_management_handoff` | `"management".to_owned` | [2254](../../src/endpoint_host.rs#L2254) | receiver-type-required |
| `mark_management_handoff` | `Some` | [2255](../../src/endpoint_host.rs#L2255) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `"management-operation".to_owned` | [2256](../../src/endpoint_host.rs#L2256) | receiver-type-required |
| `mark_management_handoff` | `request.rpc_id.clone` | [2257](../../src/endpoint_host.rs#L2257) | receiver-type-required |
| `mark_management_handoff` | `EndpointRpcFailure::new` | [2261](../../src/endpoint_host.rs#L2261) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "locked-append".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "event-origin".to_owned(),                 id: request.rpc_id.clone(),                 seq: Some(seq),             }),         })         .map_err` | [2265](../../src/endpoint_host.rs#L2265) | receiver-type-required |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off` | [2265](../../src/endpoint_host.rs#L2265) | receiver-type-required |
| `mark_event_handoff` | `"locked-append".to_owned` | [2268](../../src/endpoint_host.rs#L2268) | receiver-type-required |
| `mark_event_handoff` | `Some` | [2269](../../src/endpoint_host.rs#L2269), [2272](../../src/endpoint_host.rs#L2272) | external-constructor-callback-or-unresolved |
| `mark_event_handoff` | `"event-origin".to_owned` | [2270](../../src/endpoint_host.rs#L2270) | receiver-type-required |
| `mark_event_handoff` | `request.rpc_id.clone` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `mark_event_handoff` | `internal_failure` | [2275](../../src/endpoint_host.rs#L2275) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `endpoint_origin` | `principal.to_owned` | [2285](../../src/endpoint_host.rs#L2285) | receiver-type-required |
| `endpoint_origin` | `endpoint::ORIGIN_CLIENT.to_owned` | [2286](../../src/endpoint_host.rs#L2286) | receiver-type-required |
| `endpoint_origin` | `session_id.to_owned` | [2287](../../src/endpoint_host.rs#L2287) | receiver-type-required |
| `endpoint_origin` | `operation.to_owned` | [2288](../../src/endpoint_host.rs#L2288) | receiver-type-required |
| `endpoint_origin` | `rpc_id.to_owned` | [2289](../../src/endpoint_host.rs#L2289) | receiver-type-required |
| `validate_session_title` | `title.trim` | [2294](../../src/endpoint_host.rs#L2294) | receiver-type-required |
| `validate_session_title` | `title.is_empty` | [2295](../../src/endpoint_host.rs#L2295) | receiver-type-required |
| `validate_session_title` | `title.len` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `validate_session_title` | `title.chars().any` | [2297](../../src/endpoint_host.rs#L2297) | receiver-type-required |
| `validate_session_title` | `title.chars` | [2297](../../src/endpoint_host.rs#L2297) | receiver-type-required |
| `validate_session_title` | `Err` | [2299](../../src/endpoint_host.rs#L2299) | external-constructor-callback-or-unresolved |
| `validate_session_title` | `Ok` | [2301](../../src/endpoint_host.rs#L2301) | external-constructor-callback-or-unresolved |
| `queue_action` | `value         .as_object()         .ok_or_else` | [2328](../../src/endpoint_host.rs#L2328) | receiver-type-required |
| `queue_action` | `value         .as_object` | [2328](../../src/endpoint_host.rs#L2328) | receiver-type-required |
| `queue_action` | `invalid` | [2330](../../src/endpoint_host.rs#L2330), [2334](../../src/endpoint_host.rs#L2334), [2349](../../src/endpoint_host.rs#L2349), [2373](../../src/endpoint_host.rs#L2373) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `queue_action` | `object         .get("kind")         .and_then(Value::as_str)         .ok_or_else` | [2331](../../src/endpoint_host.rs#L2331) | receiver-type-required |
| `queue_action` | `object         .get("kind")         .and_then` | [2331](../../src/endpoint_host.rs#L2331) | receiver-type-required |
| `queue_action` | `object         .get` | [2331](../../src/endpoint_host.rs#L2331) | receiver-type-required |
| `queue_action` | `object.len` | [2336](../../src/endpoint_host.rs#L2336), [2337](../../src/endpoint_host.rs#L2337), [2339](../../src/endpoint_host.rs#L2339) | receiver-type-required |
| `queue_action` | `Ok` | [2336](../../src/endpoint_host.rs#L2336), [2337](../../src/endpoint_host.rs#L2337), [2371](../../src/endpoint_host.rs#L2371) | external-constructor-callback-or-unresolved |
| `queue_action` | `object.contains_key` | [2340](../../src/endpoint_host.rs#L2340) | receiver-type-required |
| `queue_action` | `object                     .keys()                     .all` | [2341](../../src/endpoint_host.rs#L2341) | receiver-type-required |
| `queue_action` | `object                     .keys` | [2341](../../src/endpoint_host.rs#L2341) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter(&#124;values&#124; !values.is_empty())                 .ok_or_else` | [2345](../../src/endpoint_host.rs#L2345) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter` | [2345](../../src/endpoint_host.rs#L2345) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then` | [2345](../../src/endpoint_host.rs#L2345) | receiver-type-required |
| `queue_action` | `object                 .get` | [2345](../../src/endpoint_host.rs#L2345) | receiver-type-required |
| `queue_action` | `values.is_empty` | [2348](../../src/endpoint_host.rs#L2348) | receiver-type-required |
| `queue_action` | `Vec::with_capacity` | [2350](../../src/endpoint_host.rs#L2350) | external-constructor-callback-or-unresolved |
| `queue_action` | `values.len` | [2350](../../src/endpoint_host.rs#L2350) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then(Value::as_str)                     .ok_or_else` | [2352](../../src/endpoint_host.rs#L2352) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then` | [2352](../../src/endpoint_host.rs#L2352) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then` | [2352](../../src/endpoint_host.rs#L2352) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter` | [2352](../../src/endpoint_host.rs#L2352) | receiver-type-required |
| `queue_action` | `value                     .as_object` | [2352](../../src/endpoint_host.rs#L2352) | receiver-type-required |
| `queue_action` | `block.len` | [2355](../../src/endpoint_host.rs#L2355) | receiver-type-required |
| `queue_action` | `block.get("type").and_then` | [2356](../../src/endpoint_host.rs#L2356) | receiver-type-required |
| `queue_action` | `block.get` | [2356](../../src/endpoint_host.rs#L2356), [2358](../../src/endpoint_host.rs#L2358) | receiver-type-required |
| `queue_action` | `Some` | [2356](../../src/endpoint_host.rs#L2356) | external-constructor-callback-or-unresolved |
| `queue_action` | `EndpointRpcFailure::new` | [2361](../../src/endpoint_host.rs#L2361) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `queue_action` | `content.push` | [2367](../../src/endpoint_host.rs#L2367) | receiver-type-required |
| `queue_action` | `block.to_owned` | [2368](../../src/endpoint_host.rs#L2368) | receiver-type-required |
| `queue_action` | `Err` | [2373](../../src/endpoint_host.rs#L2373) | external-constructor-callback-or-unresolved |
| `map_queue_rejection` | `EndpointRpcFailure::new` | [2383](../../src/endpoint_host.rs#L2383), [2388](../../src/endpoint_host.rs#L2388), [2393](../../src/endpoint_host.rs#L2393) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `queue_decision` | `match code {                     worker_control::QueueTransactionRejectCode::QueueItemNotFound => {                         "queue-item-not-found"                     }                     worker_control::QueueTransactionRejectCode::SteerUnavailable => {                         "steer-unavailable"                     }                     worker_control::QueueTransactionRejectCode::AttachmentError => {                         "attachment-error"                     }                 }                 .to_owned` | [2408](../../src/endpoint_host.rs#L2408) | receiver-type-required |
| `queue_decision` | `reason.clone` | [2420](../../src/endpoint_host.rs#L2420) | receiver-type-required |
| `map_persisted_queue_rejection` | `internal_failure` | [2435](../../src/endpoint_host.rs#L2435) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `map_persisted_queue_rejection` | `map_queue_rejection` | [2437](../../src/endpoint_host.rs#L2437) | [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2377) |
| `validate_time_zone` | `Ok` | [2442](../../src/endpoint_host.rs#L2442), [2463](../../src/endpoint_host.rs#L2463) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `value.trim` | [2444](../../src/endpoint_host.rs#L2444) | receiver-type-required |
| `validate_time_zone` | `value.is_ascii` | [2445](../../src/endpoint_host.rs#L2445) | receiver-type-required |
| `validate_time_zone` | `value.split('/').count` | [2446](../../src/endpoint_host.rs#L2446) | receiver-type-required |
| `validate_time_zone` | `value.split` | [2446](../../src/endpoint_host.rs#L2446), [2447](../../src/endpoint_host.rs#L2447) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate().any` | [2447](../../src/endpoint_host.rs#L2447) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate` | [2447](../../src/endpoint_host.rs#L2447) | receiver-type-required |
| `validate_time_zone` | `segment.is_empty` | [2448](../../src/endpoint_host.rs#L2448) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate().any` | [2451](../../src/endpoint_host.rs#L2451) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate` | [2451](../../src/endpoint_host.rs#L2451) | receiver-type-required |
| `validate_time_zone` | `segment.bytes` | [2451](../../src/endpoint_host.rs#L2451) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphanumeric` | [2452](../../src/endpoint_host.rs#L2452) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphabetic` | [2453](../../src/endpoint_host.rs#L2453) | receiver-type-required |
| `validate_time_zone` | `Err` | [2457](../../src/endpoint_host.rs#L2457), [2461](../../src/endpoint_host.rs#L2461) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `invalid` | [2457](../../src/endpoint_host.rs#L2457), [2461](../../src/endpoint_host.rs#L2461) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `validate_time_zone` | `Path::new("/usr/share/zoneinfo").join` | [2459](../../src/endpoint_host.rs#L2459) | receiver-type-required |
| `validate_time_zone` | `Path::new` | [2459](../../src/endpoint_host.rs#L2459) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `zone.is_file` | [2460](../../src/endpoint_host.rs#L2460) | receiver-type-required |
| `map_prompt_error` | `invalid` | [2470](../../src/endpoint_host.rs#L2470) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2592) |
| `map_prompt_error` | `attachment_failure` | [2471](../../src/endpoint_host.rs#L2471) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2503) |
| `map_prompt_error` | `EndpointRpcFailure::new` | [2472](../../src/endpoint_host.rs#L2472), [2477](../../src/endpoint_host.rs#L2477) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `map_prompt_error` | `internal_failure` | [2482](../../src/endpoint_host.rs#L2482) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `map_attachment_read_error` | `EndpointRpcFailure::new` | [2488](../../src/endpoint_host.rs#L2488), [2493](../../src/endpoint_host.rs#L2493) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `map_attachment_read_error` | `attachment_failure` | [2498](../../src/endpoint_host.rs#L2498) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2503) |
| `map_attachment_read_error` | `internal_failure` | [2499](../../src/endpoint_host.rs#L2499) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2149) |
| `attachment_failure` | `EndpointRpcFailure::new` | [2504](../../src/endpoint_host.rs#L2504) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `validate_extension_payload` | `Err` | [2564](../../src/endpoint_host.rs#L2564) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `EndpointRpcFailure::new` | [2564](../../src/endpoint_host.rs#L2564) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `validate_extension_payload` | `require_object_fields` | [2571](../../src/endpoint_host.rs#L2571) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `validate_extension_payload` | `Ok` | [2572](../../src/endpoint_host.rs#L2572) | external-constructor-callback-or-unresolved |
| `validate_production_payload` | `require_object_fields` | [2577](../../src/endpoint_host.rs#L2577), [2578](../../src/endpoint_host.rs#L2578), [2583](../../src/endpoint_host.rs#L2583) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2206) |
| `validate_production_payload` | `validate_extension_payload` | [2588](../../src/endpoint_host.rs#L2588) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2526) |
| `invalid` | `EndpointRpcFailure::new` | [2593](../../src/endpoint_host.rs#L2593) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `map_endpoint` | `session.unwrap_or_default` | [2597](../../src/endpoint_host.rs#L2597) | receiver-type-required |
| `map_endpoint` | `EndpointRpcFailure::new` | [2599](../../src/endpoint_host.rs#L2599), [2604](../../src/endpoint_host.rs#L2604), [2609](../../src/endpoint_host.rs#L2609) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `map_management` | `EndpointRpcFailure::new` | [2615](../../src/endpoint_host.rs#L2615), [2620](../../src/endpoint_host.rs#L2620), [2625](../../src/endpoint_host.rs#L2625), [2630](../../src/endpoint_host.rs#L2630), [2635](../../src/endpoint_host.rs#L2635), [2641](../../src/endpoint_host.rs#L2641), [2643](../../src/endpoint_host.rs#L2643), [2648](../../src/endpoint_host.rs#L2648), [2653](../../src/endpoint_host.rs#L2653), [2658](../../src/endpoint_host.rs#L2658), [2663](../../src/endpoint_host.rs#L2663), [2668](../../src/endpoint_host.rs#L2668), [2673](../../src/endpoint_host.rs#L2673), [2678](../../src/endpoint_host.rs#L2678), [2695](../../src/endpoint_host.rs#L2695), [2701](../../src/endpoint_host.rs#L2701), [2707](../../src/endpoint_host.rs#L2707), [2710](../../src/endpoint_host.rs#L2710), [2712](../../src/endpoint_host.rs#L2712) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `map_management` | `invalid.detail` | [2646](../../src/endpoint_host.rs#L2646) | receiver-type-required |
| `map_management` | `Value::String` | [2693](../../src/endpoint_host.rs#L2693) | external-constructor-callback-or-unresolved |
| `internal_json` | `EndpointRpcFailure::new` | [2717](../../src/endpoint_host.rs#L2717) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `internal_schema` | `EndpointRpcFailure::new` | [2721](../../src/endpoint_host.rs#L2721) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `internal_assembly` | `EndpointRpcFailure::new` | [2725](../../src/endpoint_host.rs#L2725) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2735) |
| `new` | `IJsonValue::parse(&serde_json::to_vec(&details).unwrap_or_else(&#124;_&#124; b"{}".to_vec()))                 .unwrap_or_else` | [2737](../../src/endpoint_host.rs#L2737) | receiver-type-required |
| `new` | `IJsonValue::parse` | [2737](../../src/endpoint_host.rs#L2737) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `new` | `serde_json::to_vec(&details).unwrap_or_else` | [2737](../../src/endpoint_host.rs#L2737) | receiver-type-required |
| `new` | `serde_json::to_vec` | [2737](../../src/endpoint_host.rs#L2737) | external-constructor-callback-or-unresolved |
| `new` | `b"{}".to_vec` | [2737](../../src/endpoint_host.rs#L2737) | receiver-type-required |
| `new` | `IJsonValue::parse_str("{}").expect` | [2738](../../src/endpoint_host.rs#L2738) | receiver-type-required |
| `new` | `IJsonValue::parse_str` | [2738](../../src/endpoint_host.rs#L2738) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `new` | `code.to_owned` | [2740](../../src/endpoint_host.rs#L2740) | receiver-type-required |
| `new` | `message.to_owned` | [2741](../../src/endpoint_host.rs#L2741) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; EndpointAssemblyError::Clock(error.to_string()))?         .as_secs` | [2748](../../src/endpoint_host.rs#L2748) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [2748](../../src/endpoint_host.rs#L2748) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since` | [2748](../../src/endpoint_host.rs#L2748) | receiver-type-required |
| `system_timestamp` | `SystemTime::now` | [2748](../../src/endpoint_host.rs#L2748) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `EndpointAssemblyError::Clock` | [2750](../../src/endpoint_host.rs#L2750), [2753](../../src/endpoint_host.rs#L2753) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `error.to_string` | [2750](../../src/endpoint_host.rs#L2750) | receiver-type-required |
| `system_timestamp` | `i64::try_from(seconds / 86_400)         .map_err` | [2752](../../src/endpoint_host.rs#L2752) | receiver-type-required |
| `system_timestamp` | `i64::try_from` | [2752](../../src/endpoint_host.rs#L2752) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `"timestamp exceeds i64".to_owned` | [2753](../../src/endpoint_host.rs#L2753) | receiver-type-required |
| `system_timestamp` | `civil_from_days` | [2755](../../src/endpoint_host.rs#L2755) | [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2764) |
| `system_timestamp` | `Ok` | [2759](../../src/endpoint_host.rs#L2759) | external-constructor-callback-or-unresolved |
| `civil_from_days` | `i64::from` | [2775](../../src/endpoint_host.rs#L2775) | external-constructor-callback-or-unresolved |
