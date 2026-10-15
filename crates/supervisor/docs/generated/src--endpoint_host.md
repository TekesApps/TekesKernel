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
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::with_user_agent_dir](../../src/endpoint_host.rs#L557) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description](../../src/endpoint_host.rs#L562) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L587) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L602) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L611) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities](../../src/endpoint_host.rs#L620) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission](../../src/endpoint_host.rs#L661) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L687) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L710) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities](../../src/endpoint_host.rs#L779) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates](../../src/endpoint_host.rs#L788) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation](../../src/endpoint_host.rs#L792) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute](../../src/endpoint_host.rs#L801) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L809) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L840) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace](../../src/endpoint_host.rs#L889) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace](../../src/endpoint_host.rs#L906) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace](../../src/endpoint_host.rs#L928) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L971) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L994) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L1011) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1078) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1123) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1143) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1202) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1232) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1270) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1359) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1416) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1432) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1473) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1496) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1503) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities](../../src/endpoint_host.rs#L1524) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities](../../src/endpoint_host.rs#L1545) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class](../../src/endpoint_host.rs#L1555) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request](../../src/endpoint_host.rs#L1571) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::call](../../src/endpoint_host.rs#L1595) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly](../../src/endpoint_host.rs#L1600) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../src/endpoint_host.rs#L1607) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../src/endpoint_host.rs#L1619) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../src/endpoint_host.rs#L1632) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../src/endpoint_host.rs#L1646) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities](../../src/endpoint_host.rs#L1668) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1686) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch](../../src/endpoint_host.rs#L1711) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub](../../src/endpoint_host.rs#L1719) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host](../../src/endpoint_host.rs#L1724) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates](../../src/endpoint_host.rs#L1729) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation](../../src/endpoint_host.rs#L1733) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1741) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1749) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels](../../src/endpoint_host.rs#L1846) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels::model_available](../../src/endpoint_host.rs#L1849) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1860) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::runtime_failure_name](../../src/endpoint_host.rs#L1891) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1900) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1922) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1942) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1949) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2033) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2065) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2086) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2198) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2219) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2230) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2240) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2267) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2309) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2323) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2338) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction](../../src/endpoint_host.rs#L2349) | enum_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction::with_origin](../../src/endpoint_host.rs#L2356) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2372) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2422) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2446) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2471) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2485) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_prompt_error](../../src/endpoint_host.rs#L2511) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_attachment_read_error](../../src/endpoint_host.rs#L2531) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2548) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_reason](../../src/endpoint_host.rs#L2556) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2571) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2624) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2645) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_management](../../src/endpoint_host.rs#L2662) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_json](../../src/endpoint_host.rs#L2770) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_schema](../../src/endpoint_host.rs#L2774) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_assembly](../../src/endpoint_host.rs#L2778) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure](../../src/endpoint_host.rs#L2782) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::system_timestamp](../../src/endpoint_host.rs#L2801) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2818) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointAssemblyError](../../src/endpoint_host.rs#L2834) | enum_item | `pub` |  |

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

<details><summary>Functions 21–40: 19 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::attachment::AttachmentAuthority::open"]
  n1["endpoint::management::ManagementStore::open_at_with_queue_driver"]
  n2["endpoint::mux::SessionEndpointCapability::required"]
  n3["endpoint::service::NativeEndpoint::open"]
  n4["schema::ijson::IJsonValue::parse"]
  n5["tekes-supervisor::endpoint_host::success"]
  n6["tekes-supervisor::endpoint_host::failure"]
  n7["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n8["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::validate_extension_payload"]
  n9["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_failure_is_exact"]
  n10["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::execute"]
  n11["tekes-supervisor::endpoint_host::unsupported_route"]
  n12["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointHost::with_preset_mcp"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointHost::with_user_agent_dir"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n19["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities"]
  n20["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n21["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission"]
  n22["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n23["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership"]
  n24["tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities"]
  n25["tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates"]
  n26["tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation"]
  n27["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute"]
  n28["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner"]
  n8 --> n11
  n10 --> n11
  n11 --> n4
  n11 --> n12
  n15 --> n2
  n16 --> n20
  n17 --> n20
  n18 --> n20
  n19 --> n20
  n20 --> n23
  n21 --> n23
  n22 --> n23
  n23 --> n0
  n23 --> n1
  n23 --> n3
  n23 --> n7
  n27 --> n5
  n27 --> n6
  n27 --> n28
```

</details>

<details><summary>Functions 41–60: 148 direct edges</summary>

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
  n16["tekes-supervisor::endpoint_host::model_projection"]
  n17["tekes-supervisor::endpoint_host::model_is_configured"]
  n18["tekes-supervisor::endpoint_host::map_production_route"]
  n19["tekes-supervisor::endpoint_host::map_production_route_for"]
  n20["tekes-supervisor::endpoint_host::internal_failure"]
  n21["tekes-supervisor::endpoint_host::required_string"]
  n22["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n23["tekes-supervisor::endpoint_host::optional_bool"]
  n24["tekes-supervisor::endpoint_host::optional_string"]
  n25["tekes-supervisor::endpoint_host::require_object_fields"]
  n26["tekes-supervisor::endpoint_host::optional_u64"]
  n27["tekes-supervisor::endpoint_host::to_ijson"]
  n28["tekes-supervisor::endpoint_host::request_hash"]
  n29["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n30["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n31["tekes-supervisor::endpoint_host::endpoint_origin"]
  n32["tekes-supervisor::endpoint_host::validate_session_title"]
  n33["tekes-supervisor::endpoint_host::queue_action"]
  n34["tekes-supervisor::endpoint_host::queue_decision"]
  n35["tekes-supervisor::endpoint_host::map_persisted_queue_rejection"]
  n36["tekes-supervisor::endpoint_host::validate_time_zone"]
  n37["tekes-supervisor::endpoint_host::validate_extension_payload"]
  n38["tekes-supervisor::endpoint_host::invalid"]
  n39["tekes-supervisor::endpoint_host::map_endpoint"]
  n40["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n41["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner"]
  n42["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension"]
  n43["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace"]
  n44["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace"]
  n45["tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace"]
  n46["tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session"]
  n47["tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session"]
  n48["tekes-supervisor::process_host::workspace_quiescence_lock_path"]
  n49["tools::guidance::IdentityProfile::parse"]
  n3 --> n24
  n3 --> n25
  n3 --> n27
  n3 --> n28
  n3 --> n29
  n3 --> n38
  n3 --> n40
  n3 --> n49
  n4 --> n1
  n4 --> n16
  n4 --> n19
  n4 --> n20
  n4 --> n25
  n5 --> n16
  n5 --> n19
  n5 --> n21
  n5 --> n25
  n5 --> n39
  n5 --> n40
  n6 --> n14
  n6 --> n17
  n6 --> n19
  n6 --> n20
  n6 --> n21
  n6 --> n24
  n6 --> n25
  n6 --> n27
  n6 --> n28
  n6 --> n29
  n6 --> n39
  n6 --> n40
  n7 --> n14
  n7 --> n19
  n7 --> n20
  n7 --> n21
  n7 --> n25
  n7 --> n27
  n7 --> n30
  n7 --> n31
  n7 --> n40
  n8 --> n14
  n8 --> n19
  n8 --> n20
  n8 --> n21
  n8 --> n22
  n8 --> n25
  n8 --> n27
  n8 --> n30
  n8 --> n31
  n8 --> n32
  n8 --> n40
  n9 --> n14
  n9 --> n19
  n9 --> n20
  n9 --> n21
  n9 --> n25
  n9 --> n27
  n9 --> n28
  n9 --> n29
  n9 --> n31
  n9 --> n33
  n9 --> n34
  n9 --> n35
  n9 --> n38
  n9 --> n40
  n10 --> n19
  n10 --> n21
  n10 --> n24
  n10 --> n25
  n10 --> n27
  n10 --> n30
  n10 --> n31
  n10 --> n36
  n10 --> n38
  n10 --> n40
  n11 --> n15
  n11 --> n21
  n11 --> n25
  n11 --> n27
  n12 --> n21
  n12 --> n23
  n12 --> n25
  n12 --> n26
  n12 --> n27
  n12 --> n28
  n12 --> n29
  n13 --> n19
  n13 --> n21
  n13 --> n25
  n13 --> n27
  n13 --> n28
  n13 --> n29
  n15 --> n0
  n15 --> n38
  n15 --> n40
  n41 --> n3
  n41 --> n4
  n41 --> n5
  n41 --> n6
  n41 --> n7
  n41 --> n8
  n41 --> n9
  n41 --> n10
  n41 --> n11
  n41 --> n12
  n41 --> n13
  n41 --> n42
  n41 --> n43
  n41 --> n44
  n41 --> n45
  n41 --> n46
  n41 --> n47
  n42 --> n18
  n42 --> n19
  n42 --> n20
  n42 --> n37
  n42 --> n40
  n43 --> n21
  n43 --> n25
  n43 --> n27
  n43 --> n28
  n43 --> n29
  n44 --> n21
  n44 --> n22
  n44 --> n25
  n44 --> n27
  n44 --> n28
  n44 --> n29
  n45 --> n2
  n45 --> n20
  n45 --> n21
  n45 --> n25
  n45 --> n27
  n45 --> n28
  n45 --> n29
  n45 --> n40
  n45 --> n48
  n46 --> n19
  n46 --> n21
  n46 --> n25
  n46 --> n27
  n46 --> n28
  n46 --> n29
  n47 --> n21
  n47 --> n25
  n47 --> n27
  n47 --> n28
  n47 --> n29
```

</details>

<details><summary>Functions 61–80: 23 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::host::EndpointDispatcher::new"]
  n1["endpoint::idempotency::RpcRegistry::open"]
  n2["provider::dialect::resolve_profile"]
  n3["tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities"]
  n4["tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities"]
  n5["tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointHost::call"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation"]
  n19["tekes-supervisor::endpoint_host::success"]
  n20["tekes-supervisor::endpoint_host::model_projection"]
  n21["tekes-supervisor::endpoint_host::ConfiguredModels::model_available"]
  n22["tekes-supervisor::endpoint_host::model_is_configured"]
  n23["tekes-supervisor::endpoint_host::internal_failure"]
  n24["tekes-supervisor::endpoint_host::to_ijson"]
  n25["tekes-supervisor::endpoint_host::validate_production_payload"]
  n26["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n27["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n28["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n29["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n30["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n6 --> n25
  n8 --> n0
  n8 --> n1
  n8 --> n26
  n9 --> n0
  n9 --> n1
  n9 --> n27
  n10 --> n0
  n10 --> n1
  n10 --> n28
  n11 --> n0
  n11 --> n1
  n11 --> n30
  n12 --> n13
  n13 --> n0
  n13 --> n1
  n13 --> n29
  n20 --> n2
  n20 --> n22
  n20 --> n23
  n20 --> n24
  n21 --> n22
  n22 --> n2
```

</details>

<details><summary>Functions 81–100: 19 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["tekes-supervisor::endpoint_host::runtime_failure_name"]
  n2["tekes-supervisor::endpoint_host::map_production_route"]
  n3["tekes-supervisor::endpoint_host::map_production_route_for"]
  n4["tekes-supervisor::endpoint_host::sanitized_route_log_message"]
  n5["tekes-supervisor::endpoint_host::production_failure_is_exact_for"]
  n6["tekes-supervisor::endpoint_host::route_error_is_exact"]
  n7["tekes-supervisor::endpoint_host::route_error_message"]
  n8["tekes-supervisor::endpoint_host::exact_string_fields"]
  n9["tekes-supervisor::endpoint_host::route_allows_error"]
  n10["tekes-supervisor::endpoint_host::internal_failure"]
  n11["tekes-supervisor::endpoint_host::failure"]
  n12["tekes-supervisor::endpoint_host::required_string"]
  n13["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n14["tekes-supervisor::endpoint_host::optional_bool"]
  n15["tekes-supervisor::endpoint_host::optional_string"]
  n16["tekes-supervisor::endpoint_host::require_object_fields"]
  n17["tekes-supervisor::endpoint_host::optional_u64"]
  n18["tekes-supervisor::endpoint_host::to_ijson"]
  n19["tekes-supervisor::endpoint_host::request_hash"]
  n20["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n21["tekes-supervisor::endpoint_host::invalid"]
  n22["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n3 --> n2
  n3 --> n4
  n3 --> n5
  n3 --> n10
  n4 --> n0
  n5 --> n6
  n5 --> n9
  n6 --> n7
  n6 --> n8
  n10 --> n22
  n12 --> n21
  n13 --> n21
  n14 --> n21
  n15 --> n21
  n16 --> n21
  n17 --> n21
  n18 --> n0
  n19 --> n22
  n20 --> n22
```

</details>

<details><summary>Functions 101–120: 24 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::endpoint_host::internal_failure"]
  n1["tekes-supervisor::endpoint_host::require_object_fields"]
  n2["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n3["tekes-supervisor::endpoint_host::endpoint_origin"]
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
  n22["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n2 --> n0
  n6 --> n17
  n6 --> n22
  n7 --> n22
  n9 --> n0
  n9 --> n7
  n10 --> n17
  n11 --> n0
  n11 --> n13
  n11 --> n17
  n11 --> n22
  n12 --> n0
  n12 --> n13
  n12 --> n22
  n13 --> n22
  n15 --> n1
  n15 --> n22
  n16 --> n1
  n16 --> n15
  n17 --> n22
  n18 --> n22
  n19 --> n22
  n20 --> n22
  n21 --> n22
```

</details>

<details><summary>Functions 121–124: 4 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["schema::ijson::IJsonValue::parse_str"]
  n2["tekes-supervisor::endpoint_host::internal_assembly"]
  n3["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n4["tekes-supervisor::endpoint_host::system_timestamp"]
  n5["tekes-supervisor::endpoint_host::civil_from_days"]
  n2 --> n3
  n3 --> n0
  n3 --> n1
  n4 --> n5
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
| `recover_pending_management` | `crate::host_runtime::system_timestamp` | [202](../../src/endpoint_host.rs#L202) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L475) |
| `recover_pending_management` | `ManagementError::QueueRecoveryFailed` | [203](../../src/endpoint_host.rs#L203) | external-constructor-callback-or-unresolved |
| `recover_pending_management` | `error.to_string` | [203](../../src/endpoint_host.rs#L203) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again(         root,         &started_at,         Some(&QueueRecoveryAdapter { authority }),         validated,     )     .map` | [204](../../src/endpoint_host.rs#L204) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again` | [204](../../src/endpoint_host.rs#L204) | [endpoint::management::ManagementStore::open_recovering_again](../../../endpoint/src/management.rs#L487) |
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
| `execute` | `queue_decision` | [250](../../src/endpoint_host.rs#L250) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2446) |
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
| `with_user_agent_dir` | `user_agent_dir.to_path_buf` | [558](../../src/endpoint_host.rs#L558) | receiver-type-required |
| `mux_description` | `endpoint::SessionEndpointCapability::required` | [563](../../src/endpoint_host.rs#L563) | [endpoint::mux::SessionEndpointCapability::required](../../../endpoint/src/mux.rs#L34) |
| `mux_description` | `capabilities.extend` | [564](../../src/endpoint_host.rs#L564) | receiver-type-required |
| `mux_description` | `self.provider_readiness.is_some` | [568](../../src/endpoint_host.rs#L568) | receiver-type-required |
| `mux_description` | `capabilities.insert` | [569](../../src/endpoint_host.rs#L569) | receiver-type-required |
| `mux_description` | `"TekesKernel".to_owned` | [574](../../src/endpoint_host.rs#L574) | receiver-type-required |
| `mux_description` | `self.description.version.clone` | [575](../../src/endpoint_host.rs#L575) | receiver-type-required |
| `mux_description` | `self.description.cwd.clone` | [578](../../src/endpoint_host.rs#L578) | receiver-type-required |
| `mux_description` | `self.description.provider.clone` | [579](../../src/endpoint_host.rs#L579) | receiver-type-required |
| `mux_description` | `self.description.model.clone` | [580](../../src/endpoint_host.rs#L580) | receiver-type-required |
| `mux_description` | `self.description.home.clone` | [582](../../src/endpoint_host.rs#L582) | receiver-type-required |
| `open` | `Self::open_with_full_authorities` | [591](../../src/endpoint_host.rs#L591) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open` | `Arc::new` | [594](../../src/endpoint_host.rs#L594) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [607](../../src/endpoint_host.rs#L607) | receiver-type-required |
| `open_with_clock` | `Self::open_with_full_authorities` | [608](../../src/endpoint_host.rs#L608) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open_with_routes` | `Self::open_with_full_authorities` | [617](../../src/endpoint_host.rs#L617) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open_with_authorities` | `Self::open_with_full_authorities` | [627](../../src/endpoint_host.rs#L627) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open_with_full_authorities` | `Self::open_with_route_ownership` | [647](../../src/endpoint_host.rs#L647) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L710) |
| `open_with_full_authorities_and_session_admission` | `Self::open_with_route_ownership` | [671](../../src/endpoint_host.rs#L671) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L710) |
| `open_with_full_authorities_and_session_admission` | `Some` | [680](../../src/endpoint_host.rs#L680) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `Self::open_with_route_ownership` | [696](../../src/endpoint_host.rs#L696) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L710) |
| `open_with_route_authority` | `Some` | [700](../../src/endpoint_host.rs#L700) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.as_ref` | [721](../../src/endpoint_host.rs#L721) | receiver-type-required |
| `open_with_route_ownership` | `routes.as_ref` | [722](../../src/endpoint_host.rs#L722) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map(&#124;route&#124; route.name)                 .collect::<BTreeSet<_>>` | [723](../../src/endpoint_host.rs#L723) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map` | [723](../../src/endpoint_host.rs#L723) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter` | [723](../../src/endpoint_host.rs#L723) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter().any` | [727](../../src/endpoint_host.rs#L727) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter` | [727](../../src/endpoint_host.rs#L727) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities` | [727](../../src/endpoint_host.rs#L727) | receiver-type-required |
| `open_with_route_ownership` | `frozen.contains` | [729](../../src/endpoint_host.rs#L729), [731](../../src/endpoint_host.rs#L731) | receiver-type-required |
| `open_with_route_ownership` | `method.as_str` | [729](../../src/endpoint_host.rs#L729), [731](../../src/endpoint_host.rs#L731) | receiver-type-required |
| `open_with_route_ownership` | `Err` | [738](../../src/endpoint_host.rs#L738) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `clock` | [741](../../src/endpoint_host.rs#L741) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `PathBuf::from(&description.home).join` | [742](../../src/endpoint_host.rs#L742) | receiver-type-required |
| `open_with_route_ownership` | `PathBuf::from` | [742](../../src/endpoint_host.rs#L742) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `queue_authority             .as_deref()             .map` | [743](../../src/endpoint_host.rs#L743) | receiver-type-required |
| `open_with_route_ownership` | `queue_authority             .as_deref` | [743](../../src/endpoint_host.rs#L743) | receiver-type-required |
| `open_with_route_ownership` | `ManagementStore::open_at_with_queue_driver` | [746](../../src/endpoint_host.rs#L746) | [endpoint::management::ManagementStore::open_at_with_queue_driver](../../../endpoint/src/management.rs#L476) |
| `open_with_route_ownership` | `queue_recovery                 .as_ref()                 .map` | [749](../../src/endpoint_host.rs#L749) | receiver-type-required |
| `open_with_route_ownership` | `queue_recovery                 .as_ref` | [749](../../src/endpoint_host.rs#L749) | receiver-type-required |
| `open_with_route_ownership` | `Ok` | [755](../../src/endpoint_host.rs#L755) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.to_path_buf` | [756](../../src/endpoint_host.rs#L756), [772](../../src/endpoint_host.rs#L772) | receiver-type-required |
| `open_with_route_ownership` | `NativeEndpoint::open` | [757](../../src/endpoint_host.rs#L757) | [endpoint::service::NativeEndpoint::open](../../../endpoint/src/service.rs#L263) |
| `open_with_route_ownership` | `AttachmentAuthority::open` | [758](../../src/endpoint_host.rs#L758) | [endpoint::attachment::AttachmentAuthority::open](../../../endpoint/src/attachment.rs#L242) |
| `open_with_route_ownership` | `input_admission.unwrap_or_else` | [771](../../src/endpoint_host.rs#L771) | receiver-type-required |
| `open_with_route_ownership` | `Arc::new` | [772](../../src/endpoint_host.rs#L772) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `SessionInputAdmissionAuthority::new` | [772](../../src/endpoint_host.rs#L772) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map(&#124;route&#124; route.name.to_owned())             .collect` | [780](../../src/endpoint_host.rs#L780) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map` | [780](../../src/endpoint_host.rs#L780) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter` | [780](../../src/endpoint_host.rs#L780) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter` | [780](../../src/endpoint_host.rs#L780) | receiver-type-required |
| `implemented_capabilities` | `route.name.to_owned` | [783](../../src/endpoint_host.rs#L783) | receiver-type-required |
| `admission_gates` | `self.input_admission.gates` | [789](../../src/endpoint_host.rs#L789) | receiver-type-required |
| `has_incomplete_management_operation` | `Ok` | [796](../../src/endpoint_host.rs#L796) | external-constructor-callback-or-unresolved |
| `has_incomplete_management_operation` | `self             .management             .has_incomplete_session_operation` | [796](../../src/endpoint_host.rs#L796) | receiver-type-required |
| `execute` | `self.execute_inner` | [802](../../src/endpoint_host.rs#L802) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L809) |
| `execute` | `success` | [803](../../src/endpoint_host.rs#L803) | [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1741) |
| `execute` | `failure` | [806](../../src/endpoint_host.rs#L806) | [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2198) |
| `execute_inner` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)                 .map_err` | [811](../../src/endpoint_host.rs#L811) | receiver-type-required |
| `execute_inner` | `serde_json::from_slice` | [811](../../src/endpoint_host.rs#L811) | external-constructor-callback-or-unresolved |
| `execute_inner` | `request.payload.canonical_bytes().map_err` | [811](../../src/endpoint_host.rs#L811) | receiver-type-required |
| `execute_inner` | `request.payload.canonical_bytes` | [811](../../src/endpoint_host.rs#L811) | receiver-type-required |
| `execute_inner` | `request.operation.as_str` | [813](../../src/endpoint_host.rs#L813) | receiver-type-required |
| `execute_inner` | `self.create_workspace` | [814](../../src/endpoint_host.rs#L814) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_workspace](../../src/endpoint_host.rs#L889) |
| `execute_inner` | `self.rename_workspace` | [815](../../src/endpoint_host.rs#L815) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_workspace](../../src/endpoint_host.rs#L906) |
| `execute_inner` | `self.relocate_workspace` | [816](../../src/endpoint_host.rs#L816) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::relocate_workspace](../../src/endpoint_host.rs#L928) |
| `execute_inner` | `self.archive_session` | [817](../../src/endpoint_host.rs#L817) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L971) |
| `execute_inner` | `self.unarchive_session` | [818](../../src/endpoint_host.rs#L818) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L994) |
| `execute_inner` | `self.create_session` | [819](../../src/endpoint_host.rs#L819) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L1011) |
| `execute_inner` | `self.provider_readiness.is_some` | [820](../../src/endpoint_host.rs#L820), [821](../../src/endpoint_host.rs#L821), [822](../../src/endpoint_host.rs#L822) | receiver-type-required |
| `execute_inner` | `self.models` | [820](../../src/endpoint_host.rs#L820) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1123) |
| `execute_inner` | `self.draft_models` | [821](../../src/endpoint_host.rs#L821) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1078) |
| `execute_inner` | `self.select_model` | [823](../../src/endpoint_host.rs#L823) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1143) |
| `execute_inner` | `self.delivery_authority.is_some` | [825](../../src/endpoint_host.rs#L825), [826](../../src/endpoint_host.rs#L826), [832](../../src/endpoint_host.rs#L832) | receiver-type-required |
| `execute_inner` | `self.cancel` | [825](../../src/endpoint_host.rs#L825) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1202) |
| `execute_inner` | `self.rename_session` | [827](../../src/endpoint_host.rs#L827) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1232) |
| `execute_inner` | `self.queue_authority.is_some` | [829](../../src/endpoint_host.rs#L829) | receiver-type-required |
| `execute_inner` | `self.update_queue` | [830](../../src/endpoint_host.rs#L830) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1270) |
| `execute_inner` | `self.prompt` | [832](../../src/endpoint_host.rs#L832) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1359) |
| `execute_inner` | `self.attachment` | [833](../../src/endpoint_host.rs#L833) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1416) |
| `execute_inner` | `self.fork_session` | [834](../../src/endpoint_host.rs#L834) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1432) |
| `execute_inner` | `self.discard_session` | [835](../../src/endpoint_host.rs#L835) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1473) |
| `execute_inner` | `self.execute_extension` | [836](../../src/endpoint_host.rs#L836) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L840) |
| `execute_extension` | `self.routes.as_ref` | [846](../../src/endpoint_host.rs#L846) | receiver-type-required |
| `execute_extension` | `Err` | [847](../../src/endpoint_host.rs#L847), [854](../../src/endpoint_host.rs#L854) | external-constructor-callback-or-unresolved |
| `execute_extension` | `EndpointRpcFailure::new` | [847](../../src/endpoint_host.rs#L847), [854](../../src/endpoint_host.rs#L854) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `execute_extension` | `routes.capabilities().contains` | [853](../../src/endpoint_host.rs#L853) | receiver-type-required |
| `execute_extension` | `routes.capabilities` | [853](../../src/endpoint_host.rs#L853) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter()             .any` | [860](../../src/endpoint_host.rs#L860) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter` | [860](../../src/endpoint_host.rs#L860) | receiver-type-required |
| `execute_extension` | `validate_extension_payload` | [864](../../src/endpoint_host.rs#L864) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2571) |
| `execute_extension` | `routes                 .validate_extension_payload(operation, payload)                 .map_err` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `routes                 .validate_extension_payload` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `routes.extension_failure_is_exact` | [869](../../src/endpoint_host.rs#L869), [881](../../src/endpoint_host.rs#L881) | receiver-type-required |
| `execute_extension` | `map_production_route` | [870](../../src/endpoint_host.rs#L870), [882](../../src/endpoint_host.rs#L882) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1900) |
| `execute_extension` | `internal_failure` | [872](../../src/endpoint_host.rs#L872), [884](../../src/endpoint_host.rs#L884) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `execute_extension` | `routes             .execute(request, payload, &self.principal)             .map_err` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `execute_extension` | `routes             .execute` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `execute_extension` | `map_production_route_for` | [880](../../src/endpoint_host.rs#L880) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `create_workspace` | `require_object_fields` | [894](../../src/endpoint_host.rs#L894) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `create_workspace` | `required_string` | [895](../../src/endpoint_host.rs#L895) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `create_workspace` | `(self.clock)().map_err` | [896](../../src/endpoint_host.rs#L896) | receiver-type-required |
| `create_workspace` | `(self.clock)` | [896](../../src/endpoint_host.rs#L896) | external-constructor-callback-or-unresolved |
| `create_workspace` | `request_hash` | [897](../../src/endpoint_host.rs#L897) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `create_workspace` | `self             .management             .create_workspace(&request.rpc_id, &hash, path, &timestamp)             .map_err` | [898](../../src/endpoint_host.rs#L898) | receiver-type-required |
| `create_workspace` | `self             .management             .create_workspace` | [898](../../src/endpoint_host.rs#L898) | receiver-type-required |
| `create_workspace` | `mark_management_handoff` | [902](../../src/endpoint_host.rs#L902) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `create_workspace` | `to_ijson` | [903](../../src/endpoint_host.rs#L903) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `rename_workspace` | `require_object_fields` | [911](../../src/endpoint_host.rs#L911) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `rename_workspace` | `required_string` | [916](../../src/endpoint_host.rs#L916) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `rename_workspace` | `required_string_allow_whitespace` | [917](../../src/endpoint_host.rs#L917) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2219) |
| `rename_workspace` | `(self.clock)().map_err` | [918](../../src/endpoint_host.rs#L918) | receiver-type-required |
| `rename_workspace` | `(self.clock)` | [918](../../src/endpoint_host.rs#L918) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `request_hash` | [919](../../src/endpoint_host.rs#L919) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `rename_workspace` | `self             .management             .rename_workspace(&request.rpc_id, &hash, workspace_id, title, &timestamp)             .map_err` | [920](../../src/endpoint_host.rs#L920) | receiver-type-required |
| `rename_workspace` | `self             .management             .rename_workspace` | [920](../../src/endpoint_host.rs#L920) | receiver-type-required |
| `rename_workspace` | `mark_management_handoff` | [924](../../src/endpoint_host.rs#L924) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `rename_workspace` | `to_ijson` | [925](../../src/endpoint_host.rs#L925) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `relocate_workspace` | `require_object_fields` | [933](../../src/endpoint_host.rs#L933) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `relocate_workspace` | `required_string` | [938](../../src/endpoint_host.rs#L938), [939](../../src/endpoint_host.rs#L939), [940](../../src/endpoint_host.rs#L940) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `relocate_workspace` | `store::NamedLock::try_exclusive(             crate::process_host::workspace_quiescence_lock_path(&self.storage_root, workspace_id),         )         .map_err` | [943](../../src/endpoint_host.rs#L943) | receiver-type-required |
| `relocate_workspace` | `store::NamedLock::try_exclusive` | [943](../../src/endpoint_host.rs#L943) | [store::platform::NamedLock::try_exclusive](../../../store/src/platform.rs#L107) |
| `relocate_workspace` | `crate::process_host::workspace_quiescence_lock_path` | [944](../../src/endpoint_host.rs#L944) | [tekes-supervisor::process_host::workspace_quiescence_lock_path](../../src/process_host.rs#L6462) |
| `relocate_workspace` | `EndpointRpcFailure::new` | [947](../../src/endpoint_host.rs#L947) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `relocate_workspace` | `internal_failure` | [952](../../src/endpoint_host.rs#L952) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `relocate_workspace` | `(self.clock)().map_err` | [954](../../src/endpoint_host.rs#L954) | receiver-type-required |
| `relocate_workspace` | `(self.clock)` | [954](../../src/endpoint_host.rs#L954) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `request_hash` | [955](../../src/endpoint_host.rs#L955) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `relocate_workspace` | `self             .management             .relocate_workspace(                 &request.rpc_id,                 &hash,                 workspace_id,                 previous_path,                 path,                 &timestamp,             )             .map_err` | [956](../../src/endpoint_host.rs#L956) | receiver-type-required |
| `relocate_workspace` | `self             .management             .relocate_workspace` | [956](../../src/endpoint_host.rs#L956) | receiver-type-required |
| `relocate_workspace` | `mark_management_handoff` | [967](../../src/endpoint_host.rs#L967) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `relocate_workspace` | `to_ijson` | [968](../../src/endpoint_host.rs#L968) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `archive_session` | `require_object_fields` | [976](../../src/endpoint_host.rs#L976) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `archive_session` | `required_string` | [977](../../src/endpoint_host.rs#L977) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `archive_session` | `self.input_admission.with_session_gate` | [978](../../src/endpoint_host.rs#L978) | receiver-type-required |
| `archive_session` | `map_production_route_for` | [980](../../src/endpoint_host.rs#L980) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `archive_session` | `(self.clock)().map_err` | [982](../../src/endpoint_host.rs#L982) | receiver-type-required |
| `archive_session` | `(self.clock)` | [982](../../src/endpoint_host.rs#L982) | external-constructor-callback-or-unresolved |
| `archive_session` | `request_hash` | [983](../../src/endpoint_host.rs#L983) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `archive_session` | `self                     .management                     .archive_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [984](../../src/endpoint_host.rs#L984) | receiver-type-required |
| `archive_session` | `self                     .management                     .archive_session` | [984](../../src/endpoint_host.rs#L984) | receiver-type-required |
| `archive_session` | `mark_management_handoff` | [988](../../src/endpoint_host.rs#L988) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `archive_session` | `to_ijson` | [989](../../src/endpoint_host.rs#L989) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `unarchive_session` | `require_object_fields` | [999](../../src/endpoint_host.rs#L999) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `unarchive_session` | `required_string` | [1000](../../src/endpoint_host.rs#L1000) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `unarchive_session` | `(self.clock)().map_err` | [1001](../../src/endpoint_host.rs#L1001) | receiver-type-required |
| `unarchive_session` | `(self.clock)` | [1001](../../src/endpoint_host.rs#L1001) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `request_hash` | [1002](../../src/endpoint_host.rs#L1002) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `unarchive_session` | `self             .management             .unarchive_session(&request.rpc_id, &hash, session_id, &timestamp)             .map_err` | [1003](../../src/endpoint_host.rs#L1003) | receiver-type-required |
| `unarchive_session` | `self             .management             .unarchive_session` | [1003](../../src/endpoint_host.rs#L1003) | receiver-type-required |
| `unarchive_session` | `mark_management_handoff` | [1007](../../src/endpoint_host.rs#L1007) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `unarchive_session` | `to_ijson` | [1008](../../src/endpoint_host.rs#L1008) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `create_session` | `require_object_fields` | [1016](../../src/endpoint_host.rs#L1016) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `create_session` | `payload.get("agentPreset").is_some` | [1029](../../src/endpoint_host.rs#L1029) | receiver-type-required |
| `create_session` | `payload.get` | [1029](../../src/endpoint_host.rs#L1029), [1047](../../src/endpoint_host.rs#L1047) | receiver-type-required |
| `create_session` | `Err` | [1030](../../src/endpoint_host.rs#L1030), [1049](../../src/endpoint_host.rs#L1049) | external-constructor-callback-or-unresolved |
| `create_session` | `EndpointRpcFailure::new` | [1030](../../src/endpoint_host.rs#L1030) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `create_session` | `optional_string` | [1036](../../src/endpoint_host.rs#L1036), [1037](../../src/endpoint_host.rs#L1037), [1038](../../src/endpoint_host.rs#L1038), [1039](../../src/endpoint_host.rs#L1039), [1040](../../src/endpoint_host.rs#L1040) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2240) |
| `create_session` | `optional_string(payload, "identityProfile")?             .map(&#124;value&#124; {                 tools::IdentityProfile::parse(value)                     .ok_or_else(&#124;&#124; invalid("identityProfile must be coding or general"))             })             .transpose` | [1040](../../src/endpoint_host.rs#L1040) | receiver-type-required |
| `create_session` | `optional_string(payload, "identityProfile")?             .map` | [1040](../../src/endpoint_host.rs#L1040) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse(value)                     .ok_or_else` | [1042](../../src/endpoint_host.rs#L1042) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse` | [1042](../../src/endpoint_host.rs#L1042) | [tools::guidance::IdentityProfile::parse](../../../tools/src/guidance.rs#L54) |
| `create_session` | `invalid` | [1043](../../src/endpoint_host.rs#L1043), [1049](../../src/endpoint_host.rs#L1049) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `create_session` | `preset.is_some` | [1048](../../src/endpoint_host.rs#L1048) | receiver-type-required |
| `create_session` | `identity_profile.is_some` | [1048](../../src/endpoint_host.rs#L1048) | receiver-type-required |
| `create_session` | `(self.clock)().map_err` | [1051](../../src/endpoint_host.rs#L1051) | receiver-type-required |
| `create_session` | `(self.clock)` | [1051](../../src/endpoint_host.rs#L1051) | external-constructor-callback-or-unresolved |
| `create_session` | `request_hash` | [1052](../../src/endpoint_host.rs#L1052) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `create_session` | `self             .management             .create_session(SessionCreateOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 requested_session_id: session_id,                 workspace_id,                 cwd,                 environment,                 identity_profile,                 preset,                 preset_mcp: self                     .preset_mcp                     .as_deref()                     .map(&#124;source&#124; source as &dyn endpoint::PresetMcpSource),                 preset_models: Some(&ConfiguredModels),                 user_agent_dir: &self.user_agent_dir,                 started_at: &timestamp,                 principal: &self.principal,             })             .map_err` | [1053](../../src/endpoint_host.rs#L1053) | receiver-type-required |
| `create_session` | `self             .management             .create_session` | [1053](../../src/endpoint_host.rs#L1053) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref()                     .map` | [1064](../../src/endpoint_host.rs#L1064) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref` | [1064](../../src/endpoint_host.rs#L1064) | receiver-type-required |
| `create_session` | `Some` | [1068](../../src/endpoint_host.rs#L1068) | external-constructor-callback-or-unresolved |
| `create_session` | `mark_management_handoff` | [1074](../../src/endpoint_host.rs#L1074) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `create_session` | `to_ijson` | [1075](../../src/endpoint_host.rs#L1075) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `draft_models` | `require_object_fields` | [1079](../../src/endpoint_host.rs#L1079) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `draft_models` | `profile::ConfigRepository::open(&self.storage_root).map_err` | [1081](../../src/endpoint_host.rs#L1081) | receiver-type-required |
| `draft_models` | `profile::ConfigRepository::open` | [1081](../../src/endpoint_host.rs#L1081) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L814) |
| `draft_models` | `internal_failure` | [1081](../../src/endpoint_host.rs#L1081), [1082](../../src/endpoint_host.rs#L1082), [1083](../../src/endpoint_host.rs#L1083) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `draft_models` | `repository.providers().map_err` | [1082](../../src/endpoint_host.rs#L1082) | receiver-type-required |
| `draft_models` | `repository.providers` | [1082](../../src/endpoint_host.rs#L1082) | receiver-type-required |
| `draft_models` | `repository.settings().map_err` | [1083](../../src/endpoint_host.rs#L1083) | receiver-type-required |
| `draft_models` | `repository.settings` | [1083](../../src/endpoint_host.rs#L1083) | receiver-type-required |
| `draft_models` | `String::new` | [1102](../../src/endpoint_host.rs#L1102), [1103](../../src/endpoint_host.rs#L1103) | external-constructor-callback-or-unresolved |
| `draft_models` | `Default::default` | [1107](../../src/endpoint_host.rs#L1107) | external-constructor-callback-or-unresolved |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness("", &config)             .map_err` | [1114](../../src/endpoint_host.rs#L1114) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness` | [1114](../../src/endpoint_host.rs#L1114) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else` | [1114](../../src/endpoint_host.rs#L1114) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref` | [1114](../../src/endpoint_host.rs#L1114) | receiver-type-required |
| `draft_models` | `map_production_route_for` | [1119](../../src/endpoint_host.rs#L1119) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `draft_models` | `model_projection` | [1120](../../src/endpoint_host.rs#L1120) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1749) |
| `models` | `require_object_fields` | [1124](../../src/endpoint_host.rs#L1124) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `models` | `required_string` | [1125](../../src/endpoint_host.rs#L1125) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `models` | `self.provider_readiness.as_ref().ok_or_else` | [1126](../../src/endpoint_host.rs#L1126) | receiver-type-required |
| `models` | `self.provider_readiness.as_ref` | [1126](../../src/endpoint_host.rs#L1126) | receiver-type-required |
| `models` | `EndpointRpcFailure::new` | [1127](../../src/endpoint_host.rs#L1127) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `models` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1133](../../src/endpoint_host.rs#L1133) | receiver-type-required |
| `models` | `self             .endpoint             .session_config_snapshot` | [1133](../../src/endpoint_host.rs#L1133) | receiver-type-required |
| `models` | `map_endpoint` | [1136](../../src/endpoint_host.rs#L1136) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2645) |
| `models` | `Some` | [1136](../../src/endpoint_host.rs#L1136) | external-constructor-callback-or-unresolved |
| `models` | `authority             .readiness(session_id, &config)             .map_err` | [1137](../../src/endpoint_host.rs#L1137) | receiver-type-required |
| `models` | `authority             .readiness` | [1137](../../src/endpoint_host.rs#L1137) | receiver-type-required |
| `models` | `map_production_route_for` | [1139](../../src/endpoint_host.rs#L1139) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `models` | `model_projection` | [1140](../../src/endpoint_host.rs#L1140) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1749) |
| `select_model` | `require_object_fields` | [1148](../../src/endpoint_host.rs#L1148) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `select_model` | `required_string` | [1153](../../src/endpoint_host.rs#L1153), [1154](../../src/endpoint_host.rs#L1154), [1155](../../src/endpoint_host.rs#L1155) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `select_model` | `optional_string` | [1156](../../src/endpoint_host.rs#L1156) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2240) |
| `select_model` | `self.provider_readiness.as_ref().ok_or_else` | [1157](../../src/endpoint_host.rs#L1157) | receiver-type-required |
| `select_model` | `self.provider_readiness.as_ref` | [1157](../../src/endpoint_host.rs#L1157) | receiver-type-required |
| `select_model` | `EndpointRpcFailure::new` | [1158](../../src/endpoint_host.rs#L1158), [1171](../../src/endpoint_host.rs#L1171) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `select_model` | `self.admission_gate` | [1164](../../src/endpoint_host.rs#L1164) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1496) |
| `select_model` | `gate.lock().map_err` | [1165](../../src/endpoint_host.rs#L1165) | receiver-type-required |
| `select_model` | `gate.lock` | [1165](../../src/endpoint_host.rs#L1165) | receiver-type-required |
| `select_model` | `internal_failure` | [1165](../../src/endpoint_host.rs#L1165) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `select_model` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1166](../../src/endpoint_host.rs#L1166) | receiver-type-required |
| `select_model` | `self             .endpoint             .session_config_snapshot` | [1166](../../src/endpoint_host.rs#L1166) | receiver-type-required |
| `select_model` | `map_endpoint` | [1169](../../src/endpoint_host.rs#L1169) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2645) |
| `select_model` | `Some` | [1169](../../src/endpoint_host.rs#L1169) | external-constructor-callback-or-unresolved |
| `select_model` | `model_is_configured` | [1170](../../src/endpoint_host.rs#L1170) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1860) |
| `select_model` | `Err` | [1171](../../src/endpoint_host.rs#L1171) | external-constructor-callback-or-unresolved |
| `select_model` | `(self.clock)().map_err` | [1177](../../src/endpoint_host.rs#L1177) | receiver-type-required |
| `select_model` | `(self.clock)` | [1177](../../src/endpoint_host.rs#L1177) | external-constructor-callback-or-unresolved |
| `select_model` | `request_hash` | [1178](../../src/endpoint_host.rs#L1178) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `select_model` | `self             .management             .select_model(SelectModelOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 provider,                 model,                 reasoning_effort: effort,                 started_at: &timestamp,             })             .map_err` | [1179](../../src/endpoint_host.rs#L1179) | receiver-type-required |
| `select_model` | `self             .management             .select_model` | [1179](../../src/endpoint_host.rs#L1179) | receiver-type-required |
| `select_model` | `mark_management_handoff` | [1191](../../src/endpoint_host.rs#L1191) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `select_model` | `authority             .config_mutation_succeeded(session_id)             .map_err` | [1192](../../src/endpoint_host.rs#L1192) | receiver-type-required |
| `select_model` | `authority             .config_mutation_succeeded` | [1192](../../src/endpoint_host.rs#L1192) | receiver-type-required |
| `select_model` | `map_production_route_for` | [1194](../../src/endpoint_host.rs#L1194) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `select_model` | `Value::String` | [1197](../../src/endpoint_host.rs#L1197) | external-constructor-callback-or-unresolved |
| `select_model` | `to_ijson` | [1199](../../src/endpoint_host.rs#L1199) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `cancel` | `require_object_fields` | [1207](../../src/endpoint_host.rs#L1207) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `cancel` | `required_string` | [1208](../../src/endpoint_host.rs#L1208) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `cancel` | `self.delivery_authority.as_ref().ok_or_else` | [1209](../../src/endpoint_host.rs#L1209) | receiver-type-required |
| `cancel` | `self.delivery_authority.as_ref` | [1209](../../src/endpoint_host.rs#L1209) | receiver-type-required |
| `cancel` | `EndpointRpcFailure::new` | [1210](../../src/endpoint_host.rs#L1210) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `cancel` | `self.admission_gate` | [1216](../../src/endpoint_host.rs#L1216) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1496) |
| `cancel` | `gate.lock().map_err` | [1217](../../src/endpoint_host.rs#L1217) | receiver-type-required |
| `cancel` | `gate.lock` | [1217](../../src/endpoint_host.rs#L1217) | receiver-type-required |
| `cancel` | `internal_failure` | [1217](../../src/endpoint_host.rs#L1217) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `cancel` | `(self.clock)().map_err` | [1218](../../src/endpoint_host.rs#L1218) | receiver-type-required |
| `cancel` | `(self.clock)` | [1218](../../src/endpoint_host.rs#L1218) | external-constructor-callback-or-unresolved |
| `cancel` | `endpoint_origin` | [1219](../../src/endpoint_host.rs#L1219) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2323) |
| `cancel` | `authority             .cancel(session_id, &timestamp, &origin)             .map_err` | [1225](../../src/endpoint_host.rs#L1225) | receiver-type-required |
| `cancel` | `authority             .cancel` | [1225](../../src/endpoint_host.rs#L1225) | receiver-type-required |
| `cancel` | `map_production_route_for` | [1227](../../src/endpoint_host.rs#L1227) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `cancel` | `mark_event_handoff` | [1228](../../src/endpoint_host.rs#L1228) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2309) |
| `cancel` | `to_ijson` | [1229](../../src/endpoint_host.rs#L1229) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `rename_session` | `require_object_fields` | [1237](../../src/endpoint_host.rs#L1237) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `rename_session` | `required_string` | [1238](../../src/endpoint_host.rs#L1238) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `rename_session` | `required_string_allow_whitespace` | [1239](../../src/endpoint_host.rs#L1239) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2219) |
| `rename_session` | `validate_session_title(title).map_err` | [1240](../../src/endpoint_host.rs#L1240) | receiver-type-required |
| `rename_session` | `validate_session_title` | [1240](../../src/endpoint_host.rs#L1240) | [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2338) |
| `rename_session` | `EndpointRpcFailure::new` | [1241](../../src/endpoint_host.rs#L1241), [1248](../../src/endpoint_host.rs#L1248) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `rename_session` | `self.delivery_authority.as_ref().ok_or_else` | [1247](../../src/endpoint_host.rs#L1247) | receiver-type-required |
| `rename_session` | `self.delivery_authority.as_ref` | [1247](../../src/endpoint_host.rs#L1247) | receiver-type-required |
| `rename_session` | `self.admission_gate` | [1254](../../src/endpoint_host.rs#L1254) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1496) |
| `rename_session` | `gate.lock().map_err` | [1255](../../src/endpoint_host.rs#L1255) | receiver-type-required |
| `rename_session` | `gate.lock` | [1255](../../src/endpoint_host.rs#L1255) | receiver-type-required |
| `rename_session` | `internal_failure` | [1255](../../src/endpoint_host.rs#L1255) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `rename_session` | `(self.clock)().map_err` | [1256](../../src/endpoint_host.rs#L1256) | receiver-type-required |
| `rename_session` | `(self.clock)` | [1256](../../src/endpoint_host.rs#L1256) | external-constructor-callback-or-unresolved |
| `rename_session` | `endpoint_origin` | [1257](../../src/endpoint_host.rs#L1257) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2323) |
| `rename_session` | `authority             .rename(session_id, &timestamp, &origin, title)             .map_err` | [1263](../../src/endpoint_host.rs#L1263) | receiver-type-required |
| `rename_session` | `authority             .rename` | [1263](../../src/endpoint_host.rs#L1263) | receiver-type-required |
| `rename_session` | `map_production_route_for` | [1265](../../src/endpoint_host.rs#L1265) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `rename_session` | `mark_event_handoff` | [1266](../../src/endpoint_host.rs#L1266) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2309) |
| `rename_session` | `to_ijson` | [1267](../../src/endpoint_host.rs#L1267) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `update_queue` | `require_object_fields` | [1275](../../src/endpoint_host.rs#L1275) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `update_queue` | `required_string` | [1280](../../src/endpoint_host.rs#L1280), [1281](../../src/endpoint_host.rs#L1281) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter(&#124;value&#124; *value > 0)             .ok_or_else` | [1282](../../src/endpoint_host.rs#L1282) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter` | [1282](../../src/endpoint_host.rs#L1282) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then` | [1282](../../src/endpoint_host.rs#L1282) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix` | [1282](../../src/endpoint_host.rs#L1282) | receiver-type-required |
| `update_queue` | `value.parse::<u64>().ok` | [1284](../../src/endpoint_host.rs#L1284) | receiver-type-required |
| `update_queue` | `value.parse::<u64>` | [1284](../../src/endpoint_host.rs#L1284) | receiver-type-required |
| `update_queue` | `invalid` | [1286](../../src/endpoint_host.rs#L1286), [1287](../../src/endpoint_host.rs#L1287), [1316](../../src/endpoint_host.rs#L1316) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `update_queue` | `queue_action` | [1287](../../src/endpoint_host.rs#L1287) | [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2372) |
| `update_queue` | `payload.get("action").ok_or_else` | [1287](../../src/endpoint_host.rs#L1287) | receiver-type-required |
| `update_queue` | `payload.get` | [1287](../../src/endpoint_host.rs#L1287) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref().ok_or_else` | [1288](../../src/endpoint_host.rs#L1288) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref` | [1288](../../src/endpoint_host.rs#L1288) | receiver-type-required |
| `update_queue` | `EndpointRpcFailure::new` | [1289](../../src/endpoint_host.rs#L1289) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `update_queue` | `self.admission_gate` | [1295](../../src/endpoint_host.rs#L1295) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1496) |
| `update_queue` | `gate.lock().map_err` | [1296](../../src/endpoint_host.rs#L1296) | receiver-type-required |
| `update_queue` | `gate.lock` | [1296](../../src/endpoint_host.rs#L1296) | receiver-type-required |
| `update_queue` | `internal_failure` | [1296](../../src/endpoint_host.rs#L1296), [1342](../../src/endpoint_host.rs#L1342) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `update_queue` | `request.rpc_id.clone` | [1298](../../src/endpoint_host.rs#L1298), [1299](../../src/endpoint_host.rs#L1299) | receiver-type-required |
| `update_queue` | `endpoint_origin` | [1301](../../src/endpoint_host.rs#L1301), [1307](../../src/endpoint_host.rs#L1307) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2323) |
| `update_queue` | `action.with_origin` | [1307](../../src/endpoint_host.rs#L1307) | receiver-type-required |
| `update_queue` | `transaction             .validate()             .map_err` | [1314](../../src/endpoint_host.rs#L1314) | receiver-type-required |
| `update_queue` | `transaction             .validate` | [1314](../../src/endpoint_host.rs#L1314) | receiver-type-required |
| `update_queue` | `to_ijson` | [1317](../../src/endpoint_host.rs#L1317), [1351](../../src/endpoint_host.rs#L1351) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `update_queue` | `request_hash` | [1318](../../src/endpoint_host.rs#L1318) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `update_queue` | `(self.clock)().map_err` | [1319](../../src/endpoint_host.rs#L1319) | receiver-type-required |
| `update_queue` | `(self.clock)` | [1319](../../src/endpoint_host.rs#L1319) | external-constructor-callback-or-unresolved |
| `update_queue` | `self             .management             .prepare_queue_transaction(QueueTransactionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 target_seq,                 action: &action,                 retract_origin: &transaction.retract_origin,                 replacement_origin: transaction.replacement_origin(),                 asset_digests: &[],                 started_at: &timestamp,             })             .map_err` | [1320](../../src/endpoint_host.rs#L1320) | receiver-type-required |
| `update_queue` | `self             .management             .prepare_queue_transaction` | [1320](../../src/endpoint_host.rs#L1320) | receiver-type-required |
| `update_queue` | `transaction.replacement_origin` | [1329](../../src/endpoint_host.rs#L1329) | receiver-type-required |
| `update_queue` | `authority                     .execute(session_id, &transaction)                     .map_err` | [1337](../../src/endpoint_host.rs#L1337) | receiver-type-required |
| `update_queue` | `authority                     .execute` | [1337](../../src/endpoint_host.rs#L1337) | receiver-type-required |
| `update_queue` | `map_production_route_for` | [1339](../../src/endpoint_host.rs#L1339) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `update_queue` | `result                     .validate_for(&transaction)                     .map_err` | [1340](../../src/endpoint_host.rs#L1340) | receiver-type-required |
| `update_queue` | `result                     .validate_for` | [1340](../../src/endpoint_host.rs#L1340) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction(&request.rpc_id, queue_decision(&result.outcome))                     .map_err` | [1343](../../src/endpoint_host.rs#L1343) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction` | [1343](../../src/endpoint_host.rs#L1343) | receiver-type-required |
| `update_queue` | `queue_decision` | [1344](../../src/endpoint_host.rs#L1344) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2446) |
| `update_queue` | `mark_management_handoff` | [1350](../../src/endpoint_host.rs#L1350) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `update_queue` | `Err` | [1354](../../src/endpoint_host.rs#L1354) | external-constructor-callback-or-unresolved |
| `update_queue` | `map_persisted_queue_rejection` | [1354](../../src/endpoint_host.rs#L1354) | [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2471) |
| `prompt` | `require_object_fields` | [1364](../../src/endpoint_host.rs#L1364) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `prompt` | `required_string` | [1369](../../src/endpoint_host.rs#L1369), [1370](../../src/endpoint_host.rs#L1370) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `prompt` | `Err` | [1373](../../src/endpoint_host.rs#L1373) | external-constructor-callback-or-unresolved |
| `prompt` | `invalid` | [1373](../../src/endpoint_host.rs#L1373), [1382](../../src/endpoint_host.rs#L1382), [1384](../../src/endpoint_host.rs#L1384) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `prompt` | `optional_string` | [1375](../../src/endpoint_host.rs#L1375) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2240) |
| `prompt` | `validate_time_zone` | [1376](../../src/endpoint_host.rs#L1376) | [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2485) |
| `prompt` | `serde_json::from_value(             payload                 .get("content")                 .cloned()                 .ok_or_else(&#124;&#124; invalid("content is required"))?,         )         .map_err` | [1378](../../src/endpoint_host.rs#L1378) | receiver-type-required |
| `prompt` | `serde_json::from_value` | [1378](../../src/endpoint_host.rs#L1378) | external-constructor-callback-or-unresolved |
| `prompt` | `payload                 .get("content")                 .cloned()                 .ok_or_else` | [1379](../../src/endpoint_host.rs#L1379) | receiver-type-required |
| `prompt` | `payload                 .get("content")                 .cloned` | [1379](../../src/endpoint_host.rs#L1379) | receiver-type-required |
| `prompt` | `payload                 .get` | [1379](../../src/endpoint_host.rs#L1379) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref().ok_or_else` | [1385](../../src/endpoint_host.rs#L1385) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref` | [1385](../../src/endpoint_host.rs#L1385) | receiver-type-required |
| `prompt` | `EndpointRpcFailure::new` | [1386](../../src/endpoint_host.rs#L1386) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `prompt` | `self.input_admission.with_active_session` | [1392](../../src/endpoint_host.rs#L1392) | receiver-type-required |
| `prompt` | `map_production_route_for` | [1394](../../src/endpoint_host.rs#L1394), [1409](../../src/endpoint_host.rs#L1409) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts(session_id, &parts)                     .map_err` | [1396](../../src/endpoint_host.rs#L1396) | receiver-type-required |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts` | [1396](../../src/endpoint_host.rs#L1396) | receiver-type-required |
| `prompt` | `(self.clock)().map_err` | [1400](../../src/endpoint_host.rs#L1400) | receiver-type-required |
| `prompt` | `(self.clock)` | [1400](../../src/endpoint_host.rs#L1400) | external-constructor-callback-or-unresolved |
| `prompt` | `endpoint_origin` | [1401](../../src/endpoint_host.rs#L1401) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2323) |
| `prompt` | `authority                     .prompt(session_id, &timestamp, &origin, &materialized, steer)                     .map_err` | [1407](../../src/endpoint_host.rs#L1407) | receiver-type-required |
| `prompt` | `authority                     .prompt` | [1407](../../src/endpoint_host.rs#L1407) | receiver-type-required |
| `prompt` | `mark_event_handoff` | [1410](../../src/endpoint_host.rs#L1410) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2309) |
| `prompt` | `to_ijson` | [1411](../../src/endpoint_host.rs#L1411) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `attachment` | `require_object_fields` | [1417](../../src/endpoint_host.rs#L1417) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `attachment` | `required_string` | [1422](../../src/endpoint_host.rs#L1422), [1423](../../src/endpoint_host.rs#L1423) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `attachment` | `self.ensure_active_session` | [1424](../../src/endpoint_host.rs#L1424) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1503) |
| `attachment` | `self             .attachments             .read_authorized(session_id, attachment_id)             .map_err` | [1425](../../src/endpoint_host.rs#L1425) | receiver-type-required |
| `attachment` | `self             .attachments             .read_authorized` | [1425](../../src/endpoint_host.rs#L1425) | receiver-type-required |
| `attachment` | `to_ijson` | [1429](../../src/endpoint_host.rs#L1429) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `fork_session` | `require_object_fields` | [1437](../../src/endpoint_host.rs#L1437) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `fork_session` | `required_string` | [1442](../../src/endpoint_host.rs#L1442) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `fork_session` | `optional_u64` | [1443](../../src/endpoint_host.rs#L1443) | [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2267) |
| `fork_session` | `optional_bool(payload, "ephemeral")?.unwrap_or` | [1444](../../src/endpoint_host.rs#L1444) | receiver-type-required |
| `fork_session` | `optional_bool` | [1444](../../src/endpoint_host.rs#L1444) | [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2230) |
| `fork_session` | `(self.clock)().map_err` | [1445](../../src/endpoint_host.rs#L1445) | receiver-type-required |
| `fork_session` | `(self.clock)` | [1445](../../src/endpoint_host.rs#L1445) | external-constructor-callback-or-unresolved |
| `fork_session` | `request_hash` | [1446](../../src/endpoint_host.rs#L1446) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `fork_session` | `self             .management             .fork_session(ForkSessionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 source_session_id: session_id,                 at_endpoint_seq: at_seq,                 started_at: &timestamp,                 principal: &self.principal,                 ephemeral,                 preset: payload.get("preset"),                 preset_mcp: self                     .preset_mcp                     .as_deref()                     .map(&#124;source&#124; source as &dyn endpoint::PresetMcpSource),                 preset_models: Some(&ConfiguredModels),                 user_agent_dir: &self.user_agent_dir,             })             .map_err` | [1447](../../src/endpoint_host.rs#L1447) | receiver-type-required |
| `fork_session` | `self             .management             .fork_session` | [1447](../../src/endpoint_host.rs#L1447) | receiver-type-required |
| `fork_session` | `payload.get` | [1457](../../src/endpoint_host.rs#L1457) | receiver-type-required |
| `fork_session` | `self                     .preset_mcp                     .as_deref()                     .map` | [1458](../../src/endpoint_host.rs#L1458) | receiver-type-required |
| `fork_session` | `self                     .preset_mcp                     .as_deref` | [1458](../../src/endpoint_host.rs#L1458) | receiver-type-required |
| `fork_session` | `Some` | [1462](../../src/endpoint_host.rs#L1462) | external-constructor-callback-or-unresolved |
| `fork_session` | `mark_management_handoff` | [1466](../../src/endpoint_host.rs#L1466) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `fork_session` | `to_ijson` | [1467](../../src/endpoint_host.rs#L1467) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `discard_session` | `require_object_fields` | [1478](../../src/endpoint_host.rs#L1478) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `discard_session` | `required_string` | [1479](../../src/endpoint_host.rs#L1479) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2210) |
| `discard_session` | `self.input_admission.with_session_gate` | [1480](../../src/endpoint_host.rs#L1480) | receiver-type-required |
| `discard_session` | `map_production_route_for` | [1482](../../src/endpoint_host.rs#L1482) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1908) |
| `discard_session` | `(self.clock)().map_err` | [1484](../../src/endpoint_host.rs#L1484) | receiver-type-required |
| `discard_session` | `(self.clock)` | [1484](../../src/endpoint_host.rs#L1484) | external-constructor-callback-or-unresolved |
| `discard_session` | `request_hash` | [1485](../../src/endpoint_host.rs#L1485) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2281) |
| `discard_session` | `self                     .management                     .discard_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [1486](../../src/endpoint_host.rs#L1486) | receiver-type-required |
| `discard_session` | `self                     .management                     .discard_session` | [1486](../../src/endpoint_host.rs#L1486) | receiver-type-required |
| `discard_session` | `mark_management_handoff` | [1490](../../src/endpoint_host.rs#L1490) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2295) |
| `discard_session` | `to_ijson` | [1491](../../src/endpoint_host.rs#L1491) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `admission_gate` | `self.input_admission             .gates()             .gate(session_id)             .map_err` | [1497](../../src/endpoint_host.rs#L1497) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates()             .gate` | [1497](../../src/endpoint_host.rs#L1497) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates` | [1497](../../src/endpoint_host.rs#L1497) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id(session_id).map_err` | [1504](../../src/endpoint_host.rs#L1504) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id` | [1504](../../src/endpoint_host.rs#L1504) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `ensure_active_session` | `invalid` | [1504](../../src/endpoint_host.rs#L1504) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `ensure_active_session` | `self.storage_root.join("archive").join(session_id).is_dir` | [1505](../../src/endpoint_host.rs#L1505) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("archive").join` | [1505](../../src/endpoint_host.rs#L1505) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join` | [1505](../../src/endpoint_host.rs#L1505), [1512](../../src/endpoint_host.rs#L1512) | receiver-type-required |
| `ensure_active_session` | `Err` | [1506](../../src/endpoint_host.rs#L1506), [1513](../../src/endpoint_host.rs#L1513) | external-constructor-callback-or-unresolved |
| `ensure_active_session` | `EndpointRpcFailure::new` | [1506](../../src/endpoint_host.rs#L1506), [1513](../../src/endpoint_host.rs#L1513) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `ensure_active_session` | `self.storage_root.join("threads").join(session_id).is_dir` | [1512](../../src/endpoint_host.rs#L1512) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("threads").join` | [1512](../../src/endpoint_host.rs#L1512) | receiver-type-required |
| `ensure_active_session` | `Ok` | [1519](../../src/endpoint_host.rs#L1519) | external-constructor-callback-or-unresolved |
| `capabilities` | `Self::implemented_capabilities` | [1525](../../src/endpoint_host.rs#L1525) | external-constructor-callback-or-unresolved |
| `capabilities` | `self.routes.as_ref` | [1526](../../src/endpoint_host.rs#L1526) | receiver-type-required |
| `capabilities` | `capabilities.extend` | [1527](../../src/endpoint_host.rs#L1527) | receiver-type-required |
| `capabilities` | `routes.capabilities` | [1527](../../src/endpoint_host.rs#L1527) | receiver-type-required |
| `capabilities` | `self.provider_readiness.is_some` | [1529](../../src/endpoint_host.rs#L1529) | receiver-type-required |
| `capabilities` | `capabilities.insert` | [1530](../../src/endpoint_host.rs#L1530), [1531](../../src/endpoint_host.rs#L1531), [1532](../../src/endpoint_host.rs#L1532), [1535](../../src/endpoint_host.rs#L1535), [1536](../../src/endpoint_host.rs#L1536), [1537](../../src/endpoint_host.rs#L1537), [1540](../../src/endpoint_host.rs#L1540) | receiver-type-required |
| `capabilities` | `"session.models".to_owned` | [1530](../../src/endpoint_host.rs#L1530) | receiver-type-required |
| `capabilities` | `"models.list".to_owned` | [1531](../../src/endpoint_host.rs#L1531) | receiver-type-required |
| `capabilities` | `"session.selectModel".to_owned` | [1532](../../src/endpoint_host.rs#L1532) | receiver-type-required |
| `capabilities` | `self.delivery_authority.is_some` | [1534](../../src/endpoint_host.rs#L1534) | receiver-type-required |
| `capabilities` | `"session.prompt".to_owned` | [1535](../../src/endpoint_host.rs#L1535) | receiver-type-required |
| `capabilities` | `"session.cancel".to_owned` | [1536](../../src/endpoint_host.rs#L1536) | receiver-type-required |
| `capabilities` | `"session.rename".to_owned` | [1537](../../src/endpoint_host.rs#L1537) | receiver-type-required |
| `capabilities` | `self.queue_authority.is_some` | [1539](../../src/endpoint_host.rs#L1539) | receiver-type-required |
| `capabilities` | `"session.updateQueue".to_owned` | [1540](../../src/endpoint_host.rs#L1540) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref()                 .map_or_else` | [1547](../../src/endpoint_host.rs#L1547) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref` | [1547](../../src/endpoint_host.rs#L1547) | receiver-type-required |
| `extension_capabilities` | `routes.capabilities` | [1549](../../src/endpoint_host.rs#L1549) | receiver-type-required |
| `extension_capabilities` | `BTreeSet::new` | [1551](../../src/endpoint_host.rs#L1551) | external-constructor-callback-or-unresolved |
| `method_class` | `TEKES_UNARY_ROUTES.iter().find` | [1556](../../src/endpoint_host.rs#L1556) | receiver-type-required |
| `method_class` | `TEKES_UNARY_ROUTES.iter` | [1556](../../src/endpoint_host.rs#L1556) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then(&#124;routes&#124; routes.extension_method_class(method))                 .unwrap_or` | [1563](../../src/endpoint_host.rs#L1563) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then` | [1563](../../src/endpoint_host.rs#L1563) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref` | [1563](../../src/endpoint_host.rs#L1563) | receiver-type-required |
| `method_class` | `routes.extension_method_class` | [1566](../../src/endpoint_host.rs#L1566) | receiver-type-required |
| `validate_request` | `serde_json::from_slice(             &request                 .payload                 .canonical_bytes()                 .map_err(&#124;_&#124; HostFailure::InvalidRequest)?,         )         .map_err` | [1572](../../src/endpoint_host.rs#L1572) | receiver-type-required |
| `validate_request` | `serde_json::from_slice` | [1572](../../src/endpoint_host.rs#L1572) | external-constructor-callback-or-unresolved |
| `validate_request` | `request                 .payload                 .canonical_bytes()                 .map_err` | [1573](../../src/endpoint_host.rs#L1573) | receiver-type-required |
| `validate_request` | `request                 .payload                 .canonical_bytes` | [1573](../../src/endpoint_host.rs#L1573) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter()             .any` | [1579](../../src/endpoint_host.rs#L1579) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter` | [1579](../../src/endpoint_host.rs#L1579) | receiver-type-required |
| `validate_request` | `validate_production_payload(&request.method, &payload)                 .map_err` | [1583](../../src/endpoint_host.rs#L1583) | receiver-type-required |
| `validate_request` | `validate_production_payload` | [1583](../../src/endpoint_host.rs#L1583) | [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2624) |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload(&request.method, &payload)                 .map_err` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref` | [1586](../../src/endpoint_host.rs#L1586) | receiver-type-required |
| `validate_request` | `routes.capabilities().contains` | [1588](../../src/endpoint_host.rs#L1588) | receiver-type-required |
| `validate_request` | `routes.capabilities` | [1588](../../src/endpoint_host.rs#L1588) | receiver-type-required |
| `call` | `Box::pin` | [1596](../../src/endpoint_host.rs#L1596) | external-constructor-callback-or-unresolved |
| `call` | `std::future::ready` | [1596](../../src/endpoint_host.rs#L1596) | external-constructor-callback-or-unresolved |
| `call` | `self.execute` | [1596](../../src/endpoint_host.rs#L1596) | receiver-type-required |
| `open` | `root.as_ref` | [1611](../../src/endpoint_host.rs#L1611) | receiver-type-required |
| `open` | `Ok` | [1612](../../src/endpoint_host.rs#L1612) | external-constructor-callback-or-unresolved |
| `open` | `ProductionEndpointHost::open` | [1613](../../src/endpoint_host.rs#L1613) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L587) |
| `open` | `EndpointDispatcher::new` | [1614](../../src/endpoint_host.rs#L1614) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open` | `RpcRegistry::open` | [1614](../../src/endpoint_host.rs#L1614) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open` | `EndpointSubscriptionHub::default` | [1615](../../src/endpoint_host.rs#L1615) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [1624](../../src/endpoint_host.rs#L1624) | receiver-type-required |
| `open_with_clock` | `Ok` | [1625](../../src/endpoint_host.rs#L1625) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `ProductionEndpointHost::open_with_clock` | [1626](../../src/endpoint_host.rs#L1626) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L602) |
| `open_with_clock` | `EndpointDispatcher::new` | [1627](../../src/endpoint_host.rs#L1627) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_clock` | `RpcRegistry::open` | [1627](../../src/endpoint_host.rs#L1627) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_clock` | `EndpointSubscriptionHub::default` | [1628](../../src/endpoint_host.rs#L1628) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `root.as_ref` | [1638](../../src/endpoint_host.rs#L1638) | receiver-type-required |
| `open_with_routes` | `Ok` | [1639](../../src/endpoint_host.rs#L1639) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `ProductionEndpointHost::open_with_routes` | [1640](../../src/endpoint_host.rs#L1640) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L611) |
| `open_with_routes` | `Some` | [1640](../../src/endpoint_host.rs#L1640) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `EndpointDispatcher::new` | [1641](../../src/endpoint_host.rs#L1641) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_routes` | `RpcRegistry::open` | [1641](../../src/endpoint_host.rs#L1641) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_routes` | `EndpointSubscriptionHub::default` | [1642](../../src/endpoint_host.rs#L1642) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `root.as_ref` | [1652](../../src/endpoint_host.rs#L1652) | receiver-type-required |
| `open_with_route_authority` | `Ok` | [1653](../../src/endpoint_host.rs#L1653) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `ProductionEndpointHost::open_with_route_authority` | [1654](../../src/endpoint_host.rs#L1654) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L687) |
| `open_with_route_authority` | `EndpointDispatcher::new` | [1663](../../src/endpoint_host.rs#L1663) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_route_authority` | `RpcRegistry::open` | [1663](../../src/endpoint_host.rs#L1663) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_route_authority` | `EndpointSubscriptionHub::default` | [1664](../../src/endpoint_host.rs#L1664) | external-constructor-callback-or-unresolved |
| `open_with_authorities` | `Self::open_with_full_authorities` | [1675](../../src/endpoint_host.rs#L1675) | [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1686) |
| `open_with_full_authorities` | `root.as_ref` | [1695](../../src/endpoint_host.rs#L1695) | receiver-type-required |
| `open_with_full_authorities` | `Ok` | [1696](../../src/endpoint_host.rs#L1696) | external-constructor-callback-or-unresolved |
| `open_with_full_authorities` | `ProductionEndpointHost::open_with_full_authorities` | [1697](../../src/endpoint_host.rs#L1697) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open_with_full_authorities` | `EndpointDispatcher::new` | [1706](../../src/endpoint_host.rs#L1706) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_full_authorities` | `RpcRegistry::open` | [1706](../../src/endpoint_host.rs#L1706) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_full_authorities` | `EndpointSubscriptionHub::default` | [1707](../../src/endpoint_host.rs#L1707) | external-constructor-callback-or-unresolved |
| `dispatch` | `Ok` | [1715](../../src/endpoint_host.rs#L1715) | external-constructor-callback-or-unresolved |
| `dispatch` | `self.dispatcher.dispatch` | [1715](../../src/endpoint_host.rs#L1715) | receiver-type-required |
| `admission_gates` | `self.host.admission_gates` | [1730](../../src/endpoint_host.rs#L1730) | receiver-type-required |
| `has_incomplete_management_operation` | `self.host.has_incomplete_management_operation` | [1737](../../src/endpoint_host.rs#L1737) | receiver-type-required |
| `success` | `Some` | [1744](../../src/endpoint_host.rs#L1744) | external-constructor-callback-or-unresolved |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or(config.settings.default_provider.as_deref())         .unwrap_or_default` | [1753](../../src/endpoint_host.rs#L1753) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or` | [1753](../../src/endpoint_host.rs#L1753) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or` | [1753](../../src/endpoint_host.rs#L1753) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map` | [1753](../../src/endpoint_host.rs#L1753), [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref` | [1753](../../src/endpoint_host.rs#L1753), [1760](../../src/endpoint_host.rs#L1760), [1767](../../src/endpoint_host.rs#L1767) | receiver-type-required |
| `model_projection` | `settings.provider.as_str` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `config.workspace.policy.provider.as_deref` | [1757](../../src/endpoint_host.rs#L1757) | receiver-type-required |
| `model_projection` | `config.settings.default_provider.as_deref` | [1758](../../src/endpoint_host.rs#L1758) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or(config.settings.default_model.as_deref())         .unwrap_or_default` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `settings.model.as_str` | [1763](../../src/endpoint_host.rs#L1763) | receiver-type-required |
| `model_projection` | `config.workspace.policy.model.as_deref` | [1764](../../src/endpoint_host.rs#L1764) | receiver-type-required |
| `model_projection` | `config.settings.default_model.as_deref` | [1765](../../src/endpoint_host.rs#L1765) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .and_then` | [1767](../../src/endpoint_host.rs#L1767) | receiver-type-required |
| `model_projection` | `settings.reasoning_effort.as_deref` | [1770](../../src/endpoint_host.rs#L1770) | receiver-type-required |
| `model_projection` | `Vec::new` | [1771](../../src/endpoint_host.rs#L1771), [1772](../../src/endpoint_host.rs#L1772), [1778](../../src/endpoint_host.rs#L1778) | external-constructor-callback-or-unresolved |
| `model_projection` | `provider.name.as_deref().unwrap_or` | [1774](../../src/endpoint_host.rs#L1774) | receiver-type-required |
| `model_projection` | `provider.name.as_deref` | [1774](../../src/endpoint_host.rs#L1774) | receiver-type-required |
| `model_projection` | `readiness             .iter()             .find` | [1775](../../src/endpoint_host.rs#L1775) | receiver-type-required |
| `model_projection` | `readiness             .iter` | [1775](../../src/endpoint_host.rs#L1775) | receiver-type-required |
| `model_projection` | `provider.models.iter().filter` | [1779](../../src/endpoint_host.rs#L1779) | receiver-type-required |
| `model_projection` | `provider.models.iter` | [1779](../../src/endpoint_host.rs#L1779) | receiver-type-required |
| `model_projection` | `provider::resolve_profile(provider, configured).ok` | [1780](../../src/endpoint_host.rs#L1780) | receiver-type-required |
| `model_projection` | `provider::resolve_profile` | [1780](../../src/endpoint_host.rs#L1780) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_projection` | `profile                 .as_ref()                 .map(&#124;profile&#124; profile.reasoning_efforts().to_vec())                 .unwrap_or_default` | [1781](../../src/endpoint_host.rs#L1781) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .map` | [1781](../../src/endpoint_host.rs#L1781) | receiver-type-required |
| `model_projection` | `profile                 .as_ref` | [1781](../../src/endpoint_host.rs#L1781), [1785](../../src/endpoint_host.rs#L1785) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts().to_vec` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .and_then` | [1785](../../src/endpoint_host.rs#L1785) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort().map` | [1787](../../src/endpoint_host.rs#L1787) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort` | [1787](../../src/endpoint_host.rs#L1787) | receiver-type-required |
| `model_projection` | `efforts                 .iter()                 .any` | [1788](../../src/endpoint_host.rs#L1788) | receiver-type-required |
| `model_projection` | `efforts                 .iter` | [1788](../../src/endpoint_host.rs#L1788) | receiver-type-required |
| `model_projection` | `effort.is_empty` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter().all` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `effort.as_bytes` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `Err` | [1792](../../src/endpoint_host.rs#L1792), [1795](../../src/endpoint_host.rs#L1795), [1801](../../src/endpoint_host.rs#L1801) | external-constructor-callback-or-unresolved |
| `model_projection` | `internal_failure` | [1792](../../src/endpoint_host.rs#L1792), [1795](../../src/endpoint_host.rs#L1795), [1801](../../src/endpoint_host.rs#L1801) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>().len` | [1794](../../src/endpoint_host.rs#L1794) | receiver-type-required |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>` | [1794](../../src/endpoint_host.rs#L1794) | receiver-type-required |
| `model_projection` | `efforts.iter` | [1794](../../src/endpoint_host.rs#L1794) | receiver-type-required |
| `model_projection` | `efforts.len` | [1794](../../src/endpoint_host.rs#L1794) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref()                 .is_some_and` | [1797](../../src/endpoint_host.rs#L1797) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref` | [1797](../../src/endpoint_host.rs#L1797) | receiver-type-required |
| `model_projection` | `efforts.contains` | [1799](../../src/endpoint_host.rs#L1799) | receiver-type-required |
| `model_projection` | `efforts.is_empty` | [1805](../../src/endpoint_host.rs#L1805) | receiver-type-required |
| `model_projection` | `default_effort.as_ref` | [1811](../../src/endpoint_host.rs#L1811) | receiver-type-required |
| `model_projection` | `Value::String` | [1812](../../src/endpoint_host.rs#L1812), [1834](../../src/endpoint_host.rs#L1834) | external-constructor-callback-or-unresolved |
| `model_projection` | `default.clone` | [1812](../../src/endpoint_host.rs#L1812) | receiver-type-required |
| `model_projection` | `models.push` | [1815](../../src/endpoint_host.rs#L1815) | receiver-type-required |
| `model_projection` | `groups.push` | [1817](../../src/endpoint_host.rs#L1817) | receiver-type-required |
| `model_projection` | `runtime.map` | [1818](../../src/endpoint_host.rs#L1818) | receiver-type-required |
| `model_projection` | `failures.push` | [1820](../../src/endpoint_host.rs#L1820), [1825](../../src/endpoint_host.rs#L1825) | receiver-type-required |
| `model_projection` | `effort.to_owned` | [1834](../../src/endpoint_host.rs#L1834) | receiver-type-required |
| `model_projection` | `model_is_configured` | [1836](../../src/endpoint_host.rs#L1836) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1860) |
| `model_projection` | `to_ijson` | [1837](../../src/endpoint_host.rs#L1837) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2277) |
| `model_available` | `model_is_configured` | [1856](../../src/endpoint_host.rs#L1856) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1860) |
| `model_is_configured` | `config         .providers         .providers         .iter()         .find` | [1866](../../src/endpoint_host.rs#L1866) | receiver-type-required |
| `model_is_configured` | `config         .providers         .providers         .iter` | [1866](../../src/endpoint_host.rs#L1866) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter()         .find` | [1874](../../src/endpoint_host.rs#L1874) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter` | [1874](../../src/endpoint_host.rs#L1874) | receiver-type-required |
| `model_is_configured` | `effort.is_none_or` | [1881](../../src/endpoint_host.rs#L1881) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile(provider, model).is_ok_and` | [1882](../../src/endpoint_host.rs#L1882) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile` | [1882](../../src/endpoint_host.rs#L1882) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter()                 .any` | [1883](../../src/endpoint_host.rs#L1883) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter` | [1883](../../src/endpoint_host.rs#L1883) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts` | [1883](../../src/endpoint_host.rs#L1883) | receiver-type-required |
| `map_production_route_for` | `production_failure_is_exact_for` | [1909](../../src/endpoint_host.rs#L1909) | [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1942) |
| `map_production_route_for` | `sanitized_route_log_message` | [1910](../../src/endpoint_host.rs#L1910) | [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1922) |
| `map_production_route_for` | `internal_failure` | [1917](../../src/endpoint_host.rs#L1917) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `map_production_route_for` | `map_production_route` | [1919](../../src/endpoint_host.rs#L1919) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1900) |
| `sanitized_route_log_message` | `message.len().min` | [1923](../../src/endpoint_host.rs#L1923) | receiver-type-required |
| `sanitized_route_log_message` | `message.len` | [1923](../../src/endpoint_host.rs#L1923) | receiver-type-required |
| `sanitized_route_log_message` | `message.is_char_boundary` | [1924](../../src/endpoint_host.rs#L1924) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse(         &serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect("route log message is JSON"),     )     .expect` | [1927](../../src/endpoint_host.rs#L1927) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse` | [1927](../../src/endpoint_host.rs#L1927) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `sanitized_route_log_message` | `serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect` | [1928](../../src/endpoint_host.rs#L1928) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_vec` | [1928](../../src/endpoint_host.rs#L1928) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `tools::SecretScanner::default().scan` | [1932](../../src/endpoint_host.rs#L1932) | receiver-type-required |
| `sanitized_route_log_message` | `tools::SecretScanner::default` | [1932](../../src/endpoint_host.rs#L1932) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `"<withheld>".to_owned` | [1934](../../src/endpoint_host.rs#L1934), [1939](../../src/endpoint_host.rs#L1939) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then(&#124;value&#124; value["detail"].as_str().map(str::to_owned))         .unwrap_or_else` | [1936](../../src/endpoint_host.rs#L1936) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then` | [1936](../../src/endpoint_host.rs#L1936) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok` | [1936](../../src/endpoint_host.rs#L1936) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value` | [1936](../../src/endpoint_host.rs#L1936) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `value["detail"].as_str().map` | [1938](../../src/endpoint_host.rs#L1938) | receiver-type-required |
| `sanitized_route_log_message` | `value["detail"].as_str` | [1938](../../src/endpoint_host.rs#L1938) | receiver-type-required |
| `production_failure_is_exact_for` | `route_allows_error` | [1946](../../src/endpoint_host.rs#L1946) | [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2086) |
| `production_failure_is_exact_for` | `route_error_is_exact` | [1946](../../src/endpoint_host.rs#L1946) | [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1949) |
| `route_error_is_exact` | `route_error_message` | [1950](../../src/endpoint_host.rs#L1950) | [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2033) |
| `route_error_is_exact` | `error.details.canonical_bytes` | [1956](../../src/endpoint_host.rs#L1956) | receiver-type-required |
| `route_error_is_exact` | `serde_json::from_slice::<Value>` | [1959](../../src/endpoint_host.rs#L1959) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `error.code.as_str` | [1963](../../src/endpoint_host.rs#L1963) | receiver-type-required |
| `route_error_is_exact` | `details.is_empty` | [1964](../../src/endpoint_host.rs#L1964) | receiver-type-required |
| `route_error_is_exact` | `exact_string_fields` | [1965](../../src/endpoint_host.rs#L1965), [1967](../../src/endpoint_host.rs#L1967), [1970](../../src/endpoint_host.rs#L1970), [1972](../../src/endpoint_host.rs#L1972), [1974](../../src/endpoint_host.rs#L1974), [1977](../../src/endpoint_host.rs#L1977), [1979](../../src/endpoint_host.rs#L1979), [1980](../../src/endpoint_host.rs#L1980), [1982](../../src/endpoint_host.rs#L1982), [1984](../../src/endpoint_host.rs#L1984), [1996](../../src/endpoint_host.rs#L1996), [2017](../../src/endpoint_host.rs#L2017), [2019](../../src/endpoint_host.rs#L2019) | [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2065) |
| `route_error_is_exact` | `details.len` | [1986](../../src/endpoint_host.rs#L1986), [1993](../../src/endpoint_host.rs#L1993), [1999](../../src/endpoint_host.rs#L1999) | receiver-type-required |
| `route_error_is_exact` | `details.get("sessionId").is_some_and` | [1987](../../src/endpoint_host.rs#L1987) | receiver-type-required |
| `route_error_is_exact` | `details.get` | [1987](../../src/endpoint_host.rs#L1987), [1993](../../src/endpoint_host.rs#L1993) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("atSeq")                     .is_some_and` | [1988](../../src/endpoint_host.rs#L1988) | receiver-type-required |
| `route_error_is_exact` | `details                     .get` | [1988](../../src/endpoint_host.rs#L1988), [2000](../../src/endpoint_host.rs#L2000), [2020](../../src/endpoint_host.rs#L2020) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64().is_some` | [1990](../../src/endpoint_host.rs#L1990) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64` | [1990](../../src/endpoint_host.rs#L1990) | receiver-type-required |
| `route_error_is_exact` | `details.get("limit").and_then` | [1993](../../src/endpoint_host.rs#L1993) | receiver-type-required |
| `route_error_is_exact` | `Some` | [1993](../../src/endpoint_host.rs#L1993) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then(Value::as_str)                     .is_some_and` | [2000](../../src/endpoint_host.rs#L2000), [2020](../../src/endpoint_host.rs#L2020) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then` | [2000](../../src/endpoint_host.rs#L2000), [2020](../../src/endpoint_host.rs#L2020) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter()                             .any` | [2024](../../src/endpoint_host.rs#L2024) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter` | [2024](../../src/endpoint_host.rs#L2024) | receiver-type-required |
| `route_error_is_exact` | `known.as_str` | [2026](../../src/endpoint_host.rs#L2026) | receiver-type-required |
| `route_error_message` | `Some` | [2034](../../src/endpoint_host.rs#L2034) | external-constructor-callback-or-unresolved |
| `exact_string_fields` | `required         .iter()         .any` | [2070](../../src/endpoint_host.rs#L2070) | receiver-type-required |
| `exact_string_fields` | `required         .iter` | [2070](../../src/endpoint_host.rs#L2070) | receiver-type-required |
| `exact_string_fields` | `details.get(*key).is_some_and` | [2072](../../src/endpoint_host.rs#L2072), [2075](../../src/endpoint_host.rs#L2075) | receiver-type-required |
| `exact_string_fields` | `details.get` | [2072](../../src/endpoint_host.rs#L2072), [2075](../../src/endpoint_host.rs#L2075) | receiver-type-required |
| `exact_string_fields` | `optional             .iter()             .any` | [2073](../../src/endpoint_host.rs#L2073) | receiver-type-required |
| `exact_string_fields` | `optional             .iter` | [2073](../../src/endpoint_host.rs#L2073) | receiver-type-required |
| `exact_string_fields` | `value.is_string` | [2075](../../src/endpoint_host.rs#L2075) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter(&#124;key&#124; details.contains_key(**key))         .count` | [2079](../../src/endpoint_host.rs#L2079) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter` | [2079](../../src/endpoint_host.rs#L2079) | receiver-type-required |
| `exact_string_fields` | `optional         .iter` | [2079](../../src/endpoint_host.rs#L2079) | receiver-type-required |
| `exact_string_fields` | `details.contains_key` | [2081](../../src/endpoint_host.rs#L2081) | receiver-type-required |
| `exact_string_fields` | `details.len` | [2083](../../src/endpoint_host.rs#L2083) | receiver-type-required |
| `exact_string_fields` | `required.len` | [2083](../../src/endpoint_host.rs#L2083) | receiver-type-required |
| `internal_failure` | `EndpointRpcFailure::new` | [2195](../../src/endpoint_host.rs#L2195) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `failure` | `Some` | [2202](../../src/endpoint_host.rs#L2202) | external-constructor-callback-or-unresolved |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `required_string` | `value         .as_object` | [2211](../../src/endpoint_host.rs#L2211) | receiver-type-required |
| `required_string` | `object.get` | [2213](../../src/endpoint_host.rs#L2213) | receiver-type-required |
| `required_string` | `value.is_empty` | [2215](../../src/endpoint_host.rs#L2215) | receiver-type-required |
| `required_string` | `invalid` | [2216](../../src/endpoint_host.rs#L2216) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .ok_or_else` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object` | [2223](../../src/endpoint_host.rs#L2223) | receiver-type-required |
| `required_string_allow_whitespace` | `object.get` | [2225](../../src/endpoint_host.rs#L2225) | receiver-type-required |
| `required_string_allow_whitespace` | `invalid` | [2227](../../src/endpoint_host.rs#L2227) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `optional_bool` | `value.as_object().and_then` | [2231](../../src/endpoint_host.rs#L2231) | receiver-type-required |
| `optional_bool` | `value.as_object` | [2231](../../src/endpoint_host.rs#L2231) | receiver-type-required |
| `optional_bool` | `object.get` | [2231](../../src/endpoint_host.rs#L2231) | receiver-type-required |
| `optional_bool` | `Ok` | [2232](../../src/endpoint_host.rs#L2232) | external-constructor-callback-or-unresolved |
| `optional_bool` | `value         .as_bool()         .map(Some)         .ok_or_else` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_bool` | `value         .as_bool()         .map` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_bool` | `value         .as_bool` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_bool` | `invalid` | [2237](../../src/endpoint_host.rs#L2237) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `optional_string` | `value.as_object().and_then` | [2241](../../src/endpoint_host.rs#L2241) | receiver-type-required |
| `optional_string` | `value.as_object` | [2241](../../src/endpoint_host.rs#L2241) | receiver-type-required |
| `optional_string` | `object.get` | [2241](../../src/endpoint_host.rs#L2241) | receiver-type-required |
| `optional_string` | `Ok` | [2242](../../src/endpoint_host.rs#L2242) | external-constructor-callback-or-unresolved |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map(Some)         .ok_or_else` | [2244](../../src/endpoint_host.rs#L2244) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map` | [2244](../../src/endpoint_host.rs#L2244) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter` | [2244](../../src/endpoint_host.rs#L2244) | receiver-type-required |
| `optional_string` | `value         .as_str` | [2244](../../src/endpoint_host.rs#L2244) | receiver-type-required |
| `optional_string` | `value.is_empty` | [2246](../../src/endpoint_host.rs#L2246) | receiver-type-required |
| `optional_string` | `invalid` | [2248](../../src/endpoint_host.rs#L2248) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `require_object_fields` | `value         .as_object()         .ok_or_else` | [2256](../../src/endpoint_host.rs#L2256) | receiver-type-required |
| `require_object_fields` | `value         .as_object` | [2256](../../src/endpoint_host.rs#L2256) | receiver-type-required |
| `require_object_fields` | `invalid` | [2258](../../src/endpoint_host.rs#L2258), [2262](../../src/endpoint_host.rs#L2262) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `require_object_fields` | `object.keys().any` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `require_object_fields` | `object.keys` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `require_object_fields` | `allowed.contains` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `require_object_fields` | `key.as_str` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `require_object_fields` | `required.iter().any` | [2260](../../src/endpoint_host.rs#L2260) | receiver-type-required |
| `require_object_fields` | `required.iter` | [2260](../../src/endpoint_host.rs#L2260) | receiver-type-required |
| `require_object_fields` | `object.contains_key` | [2260](../../src/endpoint_host.rs#L2260) | receiver-type-required |
| `require_object_fields` | `Err` | [2262](../../src/endpoint_host.rs#L2262) | external-constructor-callback-or-unresolved |
| `require_object_fields` | `Ok` | [2264](../../src/endpoint_host.rs#L2264) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value.as_object().and_then` | [2268](../../src/endpoint_host.rs#L2268) | receiver-type-required |
| `optional_u64` | `value.as_object` | [2268](../../src/endpoint_host.rs#L2268) | receiver-type-required |
| `optional_u64` | `object.get` | [2268](../../src/endpoint_host.rs#L2268) | receiver-type-required |
| `optional_u64` | `Ok` | [2269](../../src/endpoint_host.rs#L2269) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value         .as_u64()         .map(Some)         .ok_or_else` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `optional_u64` | `value         .as_u64()         .map` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `optional_u64` | `value         .as_u64` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `optional_u64` | `invalid` | [2274](../../src/endpoint_host.rs#L2274) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `to_ijson` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(internal_json)?).map_err` | [2278](../../src/endpoint_host.rs#L2278) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [2278](../../src/endpoint_host.rs#L2278) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value).map_err` | [2278](../../src/endpoint_host.rs#L2278) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [2278](../../src/endpoint_host.rs#L2278) | external-constructor-callback-or-unresolved |
| `request_hash` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)             .map_err` | [2283](../../src/endpoint_host.rs#L2283) | receiver-type-required |
| `request_hash` | `serde_json::from_slice` | [2283](../../src/endpoint_host.rs#L2283) | external-constructor-callback-or-unresolved |
| `request_hash` | `request.payload.canonical_bytes().map_err` | [2283](../../src/endpoint_host.rs#L2283) | receiver-type-required |
| `request_hash` | `request.payload.canonical_bytes` | [2283](../../src/endpoint_host.rs#L2283) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec(&json!({         "type":"client-request",         "rpcId":request.rpc_id,         "method":request.operation,         "payload":payload,     }))     .map_err` | [2285](../../src/endpoint_host.rs#L2285) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec` | [2285](../../src/endpoint_host.rs#L2285) | external-constructor-callback-or-unresolved |
| `request_hash` | `EndpointRpcFailure::new` | [2291](../../src/endpoint_host.rs#L2291) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `request_hash` | `Ok` | [2292](../../src/endpoint_host.rs#L2292) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "management".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "management-operation".to_owned(),                 id: request.rpc_id.clone(),                 seq: None,             }),         })         .map_err` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `mark_management_handoff` | `"management".to_owned` | [2299](../../src/endpoint_host.rs#L2299) | receiver-type-required |
| `mark_management_handoff` | `Some` | [2300](../../src/endpoint_host.rs#L2300) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `"management-operation".to_owned` | [2301](../../src/endpoint_host.rs#L2301) | receiver-type-required |
| `mark_management_handoff` | `request.rpc_id.clone` | [2302](../../src/endpoint_host.rs#L2302) | receiver-type-required |
| `mark_management_handoff` | `EndpointRpcFailure::new` | [2306](../../src/endpoint_host.rs#L2306) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "locked-append".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "event-origin".to_owned(),                 id: request.rpc_id.clone(),                 seq: Some(seq),             }),         })         .map_err` | [2310](../../src/endpoint_host.rs#L2310) | receiver-type-required |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off` | [2310](../../src/endpoint_host.rs#L2310) | receiver-type-required |
| `mark_event_handoff` | `"locked-append".to_owned` | [2313](../../src/endpoint_host.rs#L2313) | receiver-type-required |
| `mark_event_handoff` | `Some` | [2314](../../src/endpoint_host.rs#L2314), [2317](../../src/endpoint_host.rs#L2317) | external-constructor-callback-or-unresolved |
| `mark_event_handoff` | `"event-origin".to_owned` | [2315](../../src/endpoint_host.rs#L2315) | receiver-type-required |
| `mark_event_handoff` | `request.rpc_id.clone` | [2316](../../src/endpoint_host.rs#L2316) | receiver-type-required |
| `mark_event_handoff` | `internal_failure` | [2320](../../src/endpoint_host.rs#L2320) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `endpoint_origin` | `principal.to_owned` | [2330](../../src/endpoint_host.rs#L2330) | receiver-type-required |
| `endpoint_origin` | `endpoint::ORIGIN_CLIENT.to_owned` | [2331](../../src/endpoint_host.rs#L2331) | receiver-type-required |
| `endpoint_origin` | `session_id.to_owned` | [2332](../../src/endpoint_host.rs#L2332) | receiver-type-required |
| `endpoint_origin` | `operation.to_owned` | [2333](../../src/endpoint_host.rs#L2333) | receiver-type-required |
| `endpoint_origin` | `rpc_id.to_owned` | [2334](../../src/endpoint_host.rs#L2334) | receiver-type-required |
| `validate_session_title` | `title.trim` | [2339](../../src/endpoint_host.rs#L2339) | receiver-type-required |
| `validate_session_title` | `title.is_empty` | [2340](../../src/endpoint_host.rs#L2340) | receiver-type-required |
| `validate_session_title` | `title.len` | [2341](../../src/endpoint_host.rs#L2341) | receiver-type-required |
| `validate_session_title` | `title.chars().any` | [2342](../../src/endpoint_host.rs#L2342) | receiver-type-required |
| `validate_session_title` | `title.chars` | [2342](../../src/endpoint_host.rs#L2342) | receiver-type-required |
| `validate_session_title` | `Err` | [2344](../../src/endpoint_host.rs#L2344) | external-constructor-callback-or-unresolved |
| `validate_session_title` | `Ok` | [2346](../../src/endpoint_host.rs#L2346) | external-constructor-callback-or-unresolved |
| `queue_action` | `value         .as_object()         .ok_or_else` | [2373](../../src/endpoint_host.rs#L2373) | receiver-type-required |
| `queue_action` | `value         .as_object` | [2373](../../src/endpoint_host.rs#L2373) | receiver-type-required |
| `queue_action` | `invalid` | [2375](../../src/endpoint_host.rs#L2375), [2379](../../src/endpoint_host.rs#L2379), [2394](../../src/endpoint_host.rs#L2394), [2418](../../src/endpoint_host.rs#L2418) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `queue_action` | `object         .get("kind")         .and_then(Value::as_str)         .ok_or_else` | [2376](../../src/endpoint_host.rs#L2376) | receiver-type-required |
| `queue_action` | `object         .get("kind")         .and_then` | [2376](../../src/endpoint_host.rs#L2376) | receiver-type-required |
| `queue_action` | `object         .get` | [2376](../../src/endpoint_host.rs#L2376) | receiver-type-required |
| `queue_action` | `object.len` | [2381](../../src/endpoint_host.rs#L2381), [2382](../../src/endpoint_host.rs#L2382), [2384](../../src/endpoint_host.rs#L2384) | receiver-type-required |
| `queue_action` | `Ok` | [2381](../../src/endpoint_host.rs#L2381), [2382](../../src/endpoint_host.rs#L2382), [2416](../../src/endpoint_host.rs#L2416) | external-constructor-callback-or-unresolved |
| `queue_action` | `object.contains_key` | [2385](../../src/endpoint_host.rs#L2385) | receiver-type-required |
| `queue_action` | `object                     .keys()                     .all` | [2386](../../src/endpoint_host.rs#L2386) | receiver-type-required |
| `queue_action` | `object                     .keys` | [2386](../../src/endpoint_host.rs#L2386) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter(&#124;values&#124; !values.is_empty())                 .ok_or_else` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `object                 .get` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `values.is_empty` | [2393](../../src/endpoint_host.rs#L2393) | receiver-type-required |
| `queue_action` | `Vec::with_capacity` | [2395](../../src/endpoint_host.rs#L2395) | external-constructor-callback-or-unresolved |
| `queue_action` | `values.len` | [2395](../../src/endpoint_host.rs#L2395) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then(Value::as_str)                     .ok_or_else` | [2397](../../src/endpoint_host.rs#L2397) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then` | [2397](../../src/endpoint_host.rs#L2397) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then` | [2397](../../src/endpoint_host.rs#L2397) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter` | [2397](../../src/endpoint_host.rs#L2397) | receiver-type-required |
| `queue_action` | `value                     .as_object` | [2397](../../src/endpoint_host.rs#L2397) | receiver-type-required |
| `queue_action` | `block.len` | [2400](../../src/endpoint_host.rs#L2400) | receiver-type-required |
| `queue_action` | `block.get("type").and_then` | [2401](../../src/endpoint_host.rs#L2401) | receiver-type-required |
| `queue_action` | `block.get` | [2401](../../src/endpoint_host.rs#L2401), [2403](../../src/endpoint_host.rs#L2403) | receiver-type-required |
| `queue_action` | `Some` | [2401](../../src/endpoint_host.rs#L2401) | external-constructor-callback-or-unresolved |
| `queue_action` | `EndpointRpcFailure::new` | [2406](../../src/endpoint_host.rs#L2406) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `queue_action` | `content.push` | [2412](../../src/endpoint_host.rs#L2412) | receiver-type-required |
| `queue_action` | `block.to_owned` | [2413](../../src/endpoint_host.rs#L2413) | receiver-type-required |
| `queue_action` | `Err` | [2418](../../src/endpoint_host.rs#L2418) | external-constructor-callback-or-unresolved |
| `map_queue_rejection` | `EndpointRpcFailure::new` | [2428](../../src/endpoint_host.rs#L2428), [2433](../../src/endpoint_host.rs#L2433), [2438](../../src/endpoint_host.rs#L2438) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `queue_decision` | `match code {                     worker_control::QueueTransactionRejectCode::QueueItemNotFound => {                         "queue-item-not-found"                     }                     worker_control::QueueTransactionRejectCode::SteerUnavailable => {                         "steer-unavailable"                     }                     worker_control::QueueTransactionRejectCode::AttachmentError => {                         "attachment-error"                     }                 }                 .to_owned` | [2453](../../src/endpoint_host.rs#L2453) | receiver-type-required |
| `queue_decision` | `reason.clone` | [2465](../../src/endpoint_host.rs#L2465) | receiver-type-required |
| `map_persisted_queue_rejection` | `internal_failure` | [2480](../../src/endpoint_host.rs#L2480) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `map_persisted_queue_rejection` | `map_queue_rejection` | [2482](../../src/endpoint_host.rs#L2482) | [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2422) |
| `validate_time_zone` | `Ok` | [2487](../../src/endpoint_host.rs#L2487), [2508](../../src/endpoint_host.rs#L2508) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `value.trim` | [2489](../../src/endpoint_host.rs#L2489) | receiver-type-required |
| `validate_time_zone` | `value.is_ascii` | [2490](../../src/endpoint_host.rs#L2490) | receiver-type-required |
| `validate_time_zone` | `value.split('/').count` | [2491](../../src/endpoint_host.rs#L2491) | receiver-type-required |
| `validate_time_zone` | `value.split` | [2491](../../src/endpoint_host.rs#L2491), [2492](../../src/endpoint_host.rs#L2492) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate().any` | [2492](../../src/endpoint_host.rs#L2492) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate` | [2492](../../src/endpoint_host.rs#L2492) | receiver-type-required |
| `validate_time_zone` | `segment.is_empty` | [2493](../../src/endpoint_host.rs#L2493) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate().any` | [2496](../../src/endpoint_host.rs#L2496) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate` | [2496](../../src/endpoint_host.rs#L2496) | receiver-type-required |
| `validate_time_zone` | `segment.bytes` | [2496](../../src/endpoint_host.rs#L2496) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphanumeric` | [2497](../../src/endpoint_host.rs#L2497) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphabetic` | [2498](../../src/endpoint_host.rs#L2498) | receiver-type-required |
| `validate_time_zone` | `Err` | [2502](../../src/endpoint_host.rs#L2502), [2506](../../src/endpoint_host.rs#L2506) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `invalid` | [2502](../../src/endpoint_host.rs#L2502), [2506](../../src/endpoint_host.rs#L2506) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `validate_time_zone` | `Path::new("/usr/share/zoneinfo").join` | [2504](../../src/endpoint_host.rs#L2504) | receiver-type-required |
| `validate_time_zone` | `Path::new` | [2504](../../src/endpoint_host.rs#L2504) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `zone.is_file` | [2505](../../src/endpoint_host.rs#L2505) | receiver-type-required |
| `map_prompt_error` | `invalid` | [2515](../../src/endpoint_host.rs#L2515) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2641) |
| `map_prompt_error` | `attachment_failure` | [2516](../../src/endpoint_host.rs#L2516) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2548) |
| `map_prompt_error` | `EndpointRpcFailure::new` | [2517](../../src/endpoint_host.rs#L2517), [2522](../../src/endpoint_host.rs#L2522) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `map_prompt_error` | `internal_failure` | [2527](../../src/endpoint_host.rs#L2527) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `map_attachment_read_error` | `EndpointRpcFailure::new` | [2533](../../src/endpoint_host.rs#L2533), [2538](../../src/endpoint_host.rs#L2538) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `map_attachment_read_error` | `attachment_failure` | [2543](../../src/endpoint_host.rs#L2543) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2548) |
| `map_attachment_read_error` | `internal_failure` | [2544](../../src/endpoint_host.rs#L2544) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2194) |
| `attachment_failure` | `EndpointRpcFailure::new` | [2549](../../src/endpoint_host.rs#L2549) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `validate_extension_payload` | `Err` | [2613](../../src/endpoint_host.rs#L2613) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `EndpointRpcFailure::new` | [2613](../../src/endpoint_host.rs#L2613) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `validate_extension_payload` | `require_object_fields` | [2620](../../src/endpoint_host.rs#L2620) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `validate_extension_payload` | `Ok` | [2621](../../src/endpoint_host.rs#L2621) | external-constructor-callback-or-unresolved |
| `validate_production_payload` | `require_object_fields` | [2626](../../src/endpoint_host.rs#L2626), [2627](../../src/endpoint_host.rs#L2627), [2632](../../src/endpoint_host.rs#L2632) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2251) |
| `validate_production_payload` | `validate_extension_payload` | [2637](../../src/endpoint_host.rs#L2637) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2571) |
| `invalid` | `EndpointRpcFailure::new` | [2642](../../src/endpoint_host.rs#L2642) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `map_endpoint` | `session.unwrap_or_default` | [2646](../../src/endpoint_host.rs#L2646) | receiver-type-required |
| `map_endpoint` | `EndpointRpcFailure::new` | [2648](../../src/endpoint_host.rs#L2648), [2653](../../src/endpoint_host.rs#L2653), [2658](../../src/endpoint_host.rs#L2658) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `map_management` | `EndpointRpcFailure::new` | [2664](../../src/endpoint_host.rs#L2664), [2669](../../src/endpoint_host.rs#L2669), [2674](../../src/endpoint_host.rs#L2674), [2679](../../src/endpoint_host.rs#L2679), [2684](../../src/endpoint_host.rs#L2684), [2689](../../src/endpoint_host.rs#L2689), [2695](../../src/endpoint_host.rs#L2695), [2697](../../src/endpoint_host.rs#L2697), [2702](../../src/endpoint_host.rs#L2702), [2707](../../src/endpoint_host.rs#L2707), [2712](../../src/endpoint_host.rs#L2712), [2717](../../src/endpoint_host.rs#L2717), [2722](../../src/endpoint_host.rs#L2722), [2727](../../src/endpoint_host.rs#L2727), [2732](../../src/endpoint_host.rs#L2732), [2749](../../src/endpoint_host.rs#L2749), [2755](../../src/endpoint_host.rs#L2755), [2761](../../src/endpoint_host.rs#L2761), [2764](../../src/endpoint_host.rs#L2764), [2766](../../src/endpoint_host.rs#L2766) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `map_management` | `invalid.detail` | [2700](../../src/endpoint_host.rs#L2700) | receiver-type-required |
| `map_management` | `Value::String` | [2747](../../src/endpoint_host.rs#L2747) | external-constructor-callback-or-unresolved |
| `internal_json` | `EndpointRpcFailure::new` | [2771](../../src/endpoint_host.rs#L2771) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `internal_schema` | `EndpointRpcFailure::new` | [2775](../../src/endpoint_host.rs#L2775) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `internal_assembly` | `EndpointRpcFailure::new` | [2779](../../src/endpoint_host.rs#L2779) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2789) |
| `new` | `IJsonValue::parse(&serde_json::to_vec(&details).unwrap_or_else(&#124;_&#124; b"{}".to_vec()))                 .unwrap_or_else` | [2791](../../src/endpoint_host.rs#L2791) | receiver-type-required |
| `new` | `IJsonValue::parse` | [2791](../../src/endpoint_host.rs#L2791) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `new` | `serde_json::to_vec(&details).unwrap_or_else` | [2791](../../src/endpoint_host.rs#L2791) | receiver-type-required |
| `new` | `serde_json::to_vec` | [2791](../../src/endpoint_host.rs#L2791) | external-constructor-callback-or-unresolved |
| `new` | `b"{}".to_vec` | [2791](../../src/endpoint_host.rs#L2791) | receiver-type-required |
| `new` | `IJsonValue::parse_str("{}").expect` | [2792](../../src/endpoint_host.rs#L2792) | receiver-type-required |
| `new` | `IJsonValue::parse_str` | [2792](../../src/endpoint_host.rs#L2792) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `new` | `code.to_owned` | [2794](../../src/endpoint_host.rs#L2794) | receiver-type-required |
| `new` | `message.to_owned` | [2795](../../src/endpoint_host.rs#L2795) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; EndpointAssemblyError::Clock(error.to_string()))?         .as_secs` | [2802](../../src/endpoint_host.rs#L2802) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [2802](../../src/endpoint_host.rs#L2802) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since` | [2802](../../src/endpoint_host.rs#L2802) | receiver-type-required |
| `system_timestamp` | `SystemTime::now` | [2802](../../src/endpoint_host.rs#L2802) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `EndpointAssemblyError::Clock` | [2804](../../src/endpoint_host.rs#L2804), [2807](../../src/endpoint_host.rs#L2807) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `error.to_string` | [2804](../../src/endpoint_host.rs#L2804) | receiver-type-required |
| `system_timestamp` | `i64::try_from(seconds / 86_400)         .map_err` | [2806](../../src/endpoint_host.rs#L2806) | receiver-type-required |
| `system_timestamp` | `i64::try_from` | [2806](../../src/endpoint_host.rs#L2806) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `"timestamp exceeds i64".to_owned` | [2807](../../src/endpoint_host.rs#L2807) | receiver-type-required |
| `system_timestamp` | `civil_from_days` | [2809](../../src/endpoint_host.rs#L2809) | [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2818) |
| `system_timestamp` | `Ok` | [2813](../../src/endpoint_host.rs#L2813) | external-constructor-callback-or-unresolved |
| `civil_from_days` | `i64::from` | [2829](../../src/endpoint_host.rs#L2829) | external-constructor-callback-or-unresolved |
