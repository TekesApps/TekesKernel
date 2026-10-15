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
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1075) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1119) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1139) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1198) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1228) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1266) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1355) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1412) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1428) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1469) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1492) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1499) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities](../../src/endpoint_host.rs#L1520) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities](../../src/endpoint_host.rs#L1541) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class](../../src/endpoint_host.rs#L1551) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request](../../src/endpoint_host.rs#L1567) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::call](../../src/endpoint_host.rs#L1591) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly](../../src/endpoint_host.rs#L1596) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../src/endpoint_host.rs#L1603) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../src/endpoint_host.rs#L1615) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../src/endpoint_host.rs#L1628) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../src/endpoint_host.rs#L1642) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities](../../src/endpoint_host.rs#L1664) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1682) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch](../../src/endpoint_host.rs#L1707) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub](../../src/endpoint_host.rs#L1715) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host](../../src/endpoint_host.rs#L1720) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates](../../src/endpoint_host.rs#L1725) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation](../../src/endpoint_host.rs#L1729) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1737) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1745) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels](../../src/endpoint_host.rs#L1842) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels::model_available](../../src/endpoint_host.rs#L1845) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1856) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::runtime_failure_name](../../src/endpoint_host.rs#L1887) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1896) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1918) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1938) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1945) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2028) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2059) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2080) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2191) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2212) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2223) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2233) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2260) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2302) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2316) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2331) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction](../../src/endpoint_host.rs#L2342) | enum_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction::with_origin](../../src/endpoint_host.rs#L2349) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2365) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2415) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2439) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2464) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2478) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_prompt_error](../../src/endpoint_host.rs#L2504) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_attachment_read_error](../../src/endpoint_host.rs#L2524) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2541) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_reason](../../src/endpoint_host.rs#L2549) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2564) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2616) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2637) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_management](../../src/endpoint_host.rs#L2654) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_json](../../src/endpoint_host.rs#L2757) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_schema](../../src/endpoint_host.rs#L2761) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_assembly](../../src/endpoint_host.rs#L2765) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure](../../src/endpoint_host.rs#L2769) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::system_timestamp](../../src/endpoint_host.rs#L2788) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2805) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointAssemblyError](../../src/endpoint_host.rs#L2821) | enum_item | `pub` |  |

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
| `recover_pending_management` | `crate::host_runtime::system_timestamp` | [202](../../src/endpoint_host.rs#L202) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L501) |
| `recover_pending_management` | `ManagementError::QueueRecoveryFailed` | [203](../../src/endpoint_host.rs#L203) | external-constructor-callback-or-unresolved |
| `recover_pending_management` | `error.to_string` | [203](../../src/endpoint_host.rs#L203) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again(         root,         &started_at,         Some(&QueueRecoveryAdapter { authority }),         validated,     )     .map` | [204](../../src/endpoint_host.rs#L204) | receiver-type-required |
| `recover_pending_management` | `ManagementStore::open_recovering_again` | [204](../../src/endpoint_host.rs#L204) | [endpoint::management::ManagementStore::open_recovering_again](../../../endpoint/src/management.rs#L484) |
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
| `execute` | `queue_decision` | [250](../../src/endpoint_host.rs#L250) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2439) |
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
| `open_with_route_ownership` | `ManagementStore::open_at_with_queue_driver` | [746](../../src/endpoint_host.rs#L746) | [endpoint::management::ManagementStore::open_at_with_queue_driver](../../../endpoint/src/management.rs#L473) |
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
| `execute` | `success` | [803](../../src/endpoint_host.rs#L803) | [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1737) |
| `execute` | `failure` | [806](../../src/endpoint_host.rs#L806) | [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2191) |
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
| `execute_inner` | `self.models` | [820](../../src/endpoint_host.rs#L820) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1119) |
| `execute_inner` | `self.draft_models` | [821](../../src/endpoint_host.rs#L821) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L1075) |
| `execute_inner` | `self.select_model` | [823](../../src/endpoint_host.rs#L823) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1139) |
| `execute_inner` | `self.delivery_authority.is_some` | [825](../../src/endpoint_host.rs#L825), [826](../../src/endpoint_host.rs#L826), [832](../../src/endpoint_host.rs#L832) | receiver-type-required |
| `execute_inner` | `self.cancel` | [825](../../src/endpoint_host.rs#L825) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1198) |
| `execute_inner` | `self.rename_session` | [827](../../src/endpoint_host.rs#L827) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1228) |
| `execute_inner` | `self.queue_authority.is_some` | [829](../../src/endpoint_host.rs#L829) | receiver-type-required |
| `execute_inner` | `self.update_queue` | [830](../../src/endpoint_host.rs#L830) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1266) |
| `execute_inner` | `self.prompt` | [832](../../src/endpoint_host.rs#L832) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1355) |
| `execute_inner` | `self.attachment` | [833](../../src/endpoint_host.rs#L833) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1412) |
| `execute_inner` | `self.fork_session` | [834](../../src/endpoint_host.rs#L834) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1428) |
| `execute_inner` | `self.discard_session` | [835](../../src/endpoint_host.rs#L835) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1469) |
| `execute_inner` | `self.execute_extension` | [836](../../src/endpoint_host.rs#L836) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L840) |
| `execute_extension` | `self.routes.as_ref` | [846](../../src/endpoint_host.rs#L846) | receiver-type-required |
| `execute_extension` | `Err` | [847](../../src/endpoint_host.rs#L847), [854](../../src/endpoint_host.rs#L854) | external-constructor-callback-or-unresolved |
| `execute_extension` | `EndpointRpcFailure::new` | [847](../../src/endpoint_host.rs#L847), [854](../../src/endpoint_host.rs#L854) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `execute_extension` | `routes.capabilities().contains` | [853](../../src/endpoint_host.rs#L853) | receiver-type-required |
| `execute_extension` | `routes.capabilities` | [853](../../src/endpoint_host.rs#L853) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter()             .any` | [860](../../src/endpoint_host.rs#L860) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter` | [860](../../src/endpoint_host.rs#L860) | receiver-type-required |
| `execute_extension` | `validate_extension_payload` | [864](../../src/endpoint_host.rs#L864) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2564) |
| `execute_extension` | `routes                 .validate_extension_payload(operation, payload)                 .map_err` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `routes                 .validate_extension_payload` | [866](../../src/endpoint_host.rs#L866) | receiver-type-required |
| `execute_extension` | `routes.extension_failure_is_exact` | [869](../../src/endpoint_host.rs#L869), [881](../../src/endpoint_host.rs#L881) | receiver-type-required |
| `execute_extension` | `map_production_route` | [870](../../src/endpoint_host.rs#L870), [882](../../src/endpoint_host.rs#L882) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1896) |
| `execute_extension` | `internal_failure` | [872](../../src/endpoint_host.rs#L872), [884](../../src/endpoint_host.rs#L884) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `execute_extension` | `routes             .execute(request, payload, &self.principal)             .map_err` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `execute_extension` | `routes             .execute` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `execute_extension` | `map_production_route_for` | [880](../../src/endpoint_host.rs#L880) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `create_workspace` | `require_object_fields` | [894](../../src/endpoint_host.rs#L894) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `create_workspace` | `required_string` | [895](../../src/endpoint_host.rs#L895) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `create_workspace` | `(self.clock)().map_err` | [896](../../src/endpoint_host.rs#L896) | receiver-type-required |
| `create_workspace` | `(self.clock)` | [896](../../src/endpoint_host.rs#L896) | external-constructor-callback-or-unresolved |
| `create_workspace` | `request_hash` | [897](../../src/endpoint_host.rs#L897) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `create_workspace` | `self             .management             .create_workspace(&request.rpc_id, &hash, path, &timestamp)             .map_err` | [898](../../src/endpoint_host.rs#L898) | receiver-type-required |
| `create_workspace` | `self             .management             .create_workspace` | [898](../../src/endpoint_host.rs#L898) | receiver-type-required |
| `create_workspace` | `mark_management_handoff` | [902](../../src/endpoint_host.rs#L902) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `create_workspace` | `to_ijson` | [903](../../src/endpoint_host.rs#L903) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `rename_workspace` | `require_object_fields` | [911](../../src/endpoint_host.rs#L911) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `rename_workspace` | `required_string` | [916](../../src/endpoint_host.rs#L916) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `rename_workspace` | `required_string_allow_whitespace` | [917](../../src/endpoint_host.rs#L917) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2212) |
| `rename_workspace` | `(self.clock)().map_err` | [918](../../src/endpoint_host.rs#L918) | receiver-type-required |
| `rename_workspace` | `(self.clock)` | [918](../../src/endpoint_host.rs#L918) | external-constructor-callback-or-unresolved |
| `rename_workspace` | `request_hash` | [919](../../src/endpoint_host.rs#L919) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `rename_workspace` | `self             .management             .rename_workspace(&request.rpc_id, &hash, workspace_id, title, &timestamp)             .map_err` | [920](../../src/endpoint_host.rs#L920) | receiver-type-required |
| `rename_workspace` | `self             .management             .rename_workspace` | [920](../../src/endpoint_host.rs#L920) | receiver-type-required |
| `rename_workspace` | `mark_management_handoff` | [924](../../src/endpoint_host.rs#L924) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `rename_workspace` | `to_ijson` | [925](../../src/endpoint_host.rs#L925) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `relocate_workspace` | `require_object_fields` | [933](../../src/endpoint_host.rs#L933) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `relocate_workspace` | `required_string` | [938](../../src/endpoint_host.rs#L938), [939](../../src/endpoint_host.rs#L939), [940](../../src/endpoint_host.rs#L940) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `relocate_workspace` | `store::NamedLock::try_exclusive(             crate::process_host::workspace_quiescence_lock_path(&self.storage_root, workspace_id),         )         .map_err` | [943](../../src/endpoint_host.rs#L943) | receiver-type-required |
| `relocate_workspace` | `store::NamedLock::try_exclusive` | [943](../../src/endpoint_host.rs#L943) | [store::platform::NamedLock::try_exclusive](../../../store/src/platform.rs#L107) |
| `relocate_workspace` | `crate::process_host::workspace_quiescence_lock_path` | [944](../../src/endpoint_host.rs#L944) | [tekes-supervisor::process_host::workspace_quiescence_lock_path](../../src/process_host.rs#L6364) |
| `relocate_workspace` | `EndpointRpcFailure::new` | [947](../../src/endpoint_host.rs#L947) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `relocate_workspace` | `internal_failure` | [952](../../src/endpoint_host.rs#L952) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `relocate_workspace` | `(self.clock)().map_err` | [954](../../src/endpoint_host.rs#L954) | receiver-type-required |
| `relocate_workspace` | `(self.clock)` | [954](../../src/endpoint_host.rs#L954) | external-constructor-callback-or-unresolved |
| `relocate_workspace` | `request_hash` | [955](../../src/endpoint_host.rs#L955) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `relocate_workspace` | `self             .management             .relocate_workspace(                 &request.rpc_id,                 &hash,                 workspace_id,                 previous_path,                 path,                 &timestamp,             )             .map_err` | [956](../../src/endpoint_host.rs#L956) | receiver-type-required |
| `relocate_workspace` | `self             .management             .relocate_workspace` | [956](../../src/endpoint_host.rs#L956) | receiver-type-required |
| `relocate_workspace` | `mark_management_handoff` | [967](../../src/endpoint_host.rs#L967) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `relocate_workspace` | `to_ijson` | [968](../../src/endpoint_host.rs#L968) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `archive_session` | `require_object_fields` | [976](../../src/endpoint_host.rs#L976) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `archive_session` | `required_string` | [977](../../src/endpoint_host.rs#L977) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `archive_session` | `self.input_admission.with_session_gate` | [978](../../src/endpoint_host.rs#L978) | receiver-type-required |
| `archive_session` | `map_production_route_for` | [980](../../src/endpoint_host.rs#L980) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `archive_session` | `(self.clock)().map_err` | [982](../../src/endpoint_host.rs#L982) | receiver-type-required |
| `archive_session` | `(self.clock)` | [982](../../src/endpoint_host.rs#L982) | external-constructor-callback-or-unresolved |
| `archive_session` | `request_hash` | [983](../../src/endpoint_host.rs#L983) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `archive_session` | `self                     .management                     .archive_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [984](../../src/endpoint_host.rs#L984) | receiver-type-required |
| `archive_session` | `self                     .management                     .archive_session` | [984](../../src/endpoint_host.rs#L984) | receiver-type-required |
| `archive_session` | `mark_management_handoff` | [988](../../src/endpoint_host.rs#L988) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `archive_session` | `to_ijson` | [989](../../src/endpoint_host.rs#L989) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `unarchive_session` | `require_object_fields` | [999](../../src/endpoint_host.rs#L999) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `unarchive_session` | `required_string` | [1000](../../src/endpoint_host.rs#L1000) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `unarchive_session` | `(self.clock)().map_err` | [1001](../../src/endpoint_host.rs#L1001) | receiver-type-required |
| `unarchive_session` | `(self.clock)` | [1001](../../src/endpoint_host.rs#L1001) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `request_hash` | [1002](../../src/endpoint_host.rs#L1002) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `unarchive_session` | `self             .management             .unarchive_session(&request.rpc_id, &hash, session_id, &timestamp)             .map_err` | [1003](../../src/endpoint_host.rs#L1003) | receiver-type-required |
| `unarchive_session` | `self             .management             .unarchive_session` | [1003](../../src/endpoint_host.rs#L1003) | receiver-type-required |
| `unarchive_session` | `mark_management_handoff` | [1007](../../src/endpoint_host.rs#L1007) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `unarchive_session` | `to_ijson` | [1008](../../src/endpoint_host.rs#L1008) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `create_session` | `require_object_fields` | [1016](../../src/endpoint_host.rs#L1016) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `create_session` | `payload.get("agentPreset").is_some` | [1028](../../src/endpoint_host.rs#L1028) | receiver-type-required |
| `create_session` | `payload.get` | [1028](../../src/endpoint_host.rs#L1028), [1045](../../src/endpoint_host.rs#L1045) | receiver-type-required |
| `create_session` | `Err` | [1029](../../src/endpoint_host.rs#L1029), [1047](../../src/endpoint_host.rs#L1047) | external-constructor-callback-or-unresolved |
| `create_session` | `EndpointRpcFailure::new` | [1029](../../src/endpoint_host.rs#L1029) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `create_session` | `optional_string` | [1035](../../src/endpoint_host.rs#L1035), [1036](../../src/endpoint_host.rs#L1036), [1037](../../src/endpoint_host.rs#L1037), [1038](../../src/endpoint_host.rs#L1038) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2233) |
| `create_session` | `optional_string(payload, "identityProfile")?             .map(&#124;value&#124; {                 tools::IdentityProfile::parse(value)                     .ok_or_else(&#124;&#124; invalid("identityProfile must be coding or general"))             })             .transpose` | [1038](../../src/endpoint_host.rs#L1038) | receiver-type-required |
| `create_session` | `optional_string(payload, "identityProfile")?             .map` | [1038](../../src/endpoint_host.rs#L1038) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse(value)                     .ok_or_else` | [1040](../../src/endpoint_host.rs#L1040) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse` | [1040](../../src/endpoint_host.rs#L1040) | [tools::guidance::IdentityProfile::parse](../../../tools/src/guidance.rs#L54) |
| `create_session` | `invalid` | [1041](../../src/endpoint_host.rs#L1041), [1047](../../src/endpoint_host.rs#L1047) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `create_session` | `preset.is_some` | [1046](../../src/endpoint_host.rs#L1046) | receiver-type-required |
| `create_session` | `identity_profile.is_some` | [1046](../../src/endpoint_host.rs#L1046) | receiver-type-required |
| `create_session` | `(self.clock)().map_err` | [1049](../../src/endpoint_host.rs#L1049) | receiver-type-required |
| `create_session` | `(self.clock)` | [1049](../../src/endpoint_host.rs#L1049) | external-constructor-callback-or-unresolved |
| `create_session` | `request_hash` | [1050](../../src/endpoint_host.rs#L1050) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `create_session` | `self             .management             .create_session(SessionCreateOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 requested_session_id: session_id,                 workspace_id,                 cwd,                 identity_profile,                 preset,                 preset_mcp: self                     .preset_mcp                     .as_deref()                     .map(&#124;source&#124; source as &dyn endpoint::PresetMcpSource),                 preset_models: Some(&ConfiguredModels),                 user_agent_dir: &self.user_agent_dir,                 started_at: &timestamp,                 principal: &self.principal,             })             .map_err` | [1051](../../src/endpoint_host.rs#L1051) | receiver-type-required |
| `create_session` | `self             .management             .create_session` | [1051](../../src/endpoint_host.rs#L1051) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref()                     .map` | [1061](../../src/endpoint_host.rs#L1061) | receiver-type-required |
| `create_session` | `self                     .preset_mcp                     .as_deref` | [1061](../../src/endpoint_host.rs#L1061) | receiver-type-required |
| `create_session` | `Some` | [1065](../../src/endpoint_host.rs#L1065) | external-constructor-callback-or-unresolved |
| `create_session` | `mark_management_handoff` | [1071](../../src/endpoint_host.rs#L1071) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `create_session` | `to_ijson` | [1072](../../src/endpoint_host.rs#L1072) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `draft_models` | `require_object_fields` | [1076](../../src/endpoint_host.rs#L1076) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `draft_models` | `profile::ConfigRepository::open(&self.storage_root).map_err` | [1078](../../src/endpoint_host.rs#L1078) | receiver-type-required |
| `draft_models` | `profile::ConfigRepository::open` | [1078](../../src/endpoint_host.rs#L1078) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L786) |
| `draft_models` | `internal_failure` | [1078](../../src/endpoint_host.rs#L1078), [1079](../../src/endpoint_host.rs#L1079), [1080](../../src/endpoint_host.rs#L1080) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `draft_models` | `repository.providers().map_err` | [1079](../../src/endpoint_host.rs#L1079) | receiver-type-required |
| `draft_models` | `repository.providers` | [1079](../../src/endpoint_host.rs#L1079) | receiver-type-required |
| `draft_models` | `repository.settings().map_err` | [1080](../../src/endpoint_host.rs#L1080) | receiver-type-required |
| `draft_models` | `repository.settings` | [1080](../../src/endpoint_host.rs#L1080) | receiver-type-required |
| `draft_models` | `String::new` | [1098](../../src/endpoint_host.rs#L1098), [1099](../../src/endpoint_host.rs#L1099) | external-constructor-callback-or-unresolved |
| `draft_models` | `Default::default` | [1103](../../src/endpoint_host.rs#L1103) | external-constructor-callback-or-unresolved |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness("", &config)             .map_err` | [1110](../../src/endpoint_host.rs#L1110) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness` | [1110](../../src/endpoint_host.rs#L1110) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else` | [1110](../../src/endpoint_host.rs#L1110) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref` | [1110](../../src/endpoint_host.rs#L1110) | receiver-type-required |
| `draft_models` | `map_production_route_for` | [1115](../../src/endpoint_host.rs#L1115) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `draft_models` | `model_projection` | [1116](../../src/endpoint_host.rs#L1116) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1745) |
| `models` | `require_object_fields` | [1120](../../src/endpoint_host.rs#L1120) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `models` | `required_string` | [1121](../../src/endpoint_host.rs#L1121) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `models` | `self.provider_readiness.as_ref().ok_or_else` | [1122](../../src/endpoint_host.rs#L1122) | receiver-type-required |
| `models` | `self.provider_readiness.as_ref` | [1122](../../src/endpoint_host.rs#L1122) | receiver-type-required |
| `models` | `EndpointRpcFailure::new` | [1123](../../src/endpoint_host.rs#L1123) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `models` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1129](../../src/endpoint_host.rs#L1129) | receiver-type-required |
| `models` | `self             .endpoint             .session_config_snapshot` | [1129](../../src/endpoint_host.rs#L1129) | receiver-type-required |
| `models` | `map_endpoint` | [1132](../../src/endpoint_host.rs#L1132) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2637) |
| `models` | `Some` | [1132](../../src/endpoint_host.rs#L1132) | external-constructor-callback-or-unresolved |
| `models` | `authority             .readiness(session_id, &config)             .map_err` | [1133](../../src/endpoint_host.rs#L1133) | receiver-type-required |
| `models` | `authority             .readiness` | [1133](../../src/endpoint_host.rs#L1133) | receiver-type-required |
| `models` | `map_production_route_for` | [1135](../../src/endpoint_host.rs#L1135) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `models` | `model_projection` | [1136](../../src/endpoint_host.rs#L1136) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1745) |
| `select_model` | `require_object_fields` | [1144](../../src/endpoint_host.rs#L1144) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `select_model` | `required_string` | [1149](../../src/endpoint_host.rs#L1149), [1150](../../src/endpoint_host.rs#L1150), [1151](../../src/endpoint_host.rs#L1151) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `select_model` | `optional_string` | [1152](../../src/endpoint_host.rs#L1152) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2233) |
| `select_model` | `self.provider_readiness.as_ref().ok_or_else` | [1153](../../src/endpoint_host.rs#L1153) | receiver-type-required |
| `select_model` | `self.provider_readiness.as_ref` | [1153](../../src/endpoint_host.rs#L1153) | receiver-type-required |
| `select_model` | `EndpointRpcFailure::new` | [1154](../../src/endpoint_host.rs#L1154), [1167](../../src/endpoint_host.rs#L1167) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `select_model` | `self.admission_gate` | [1160](../../src/endpoint_host.rs#L1160) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1492) |
| `select_model` | `gate.lock().map_err` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `select_model` | `gate.lock` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `select_model` | `internal_failure` | [1161](../../src/endpoint_host.rs#L1161) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `select_model` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1162](../../src/endpoint_host.rs#L1162) | receiver-type-required |
| `select_model` | `self             .endpoint             .session_config_snapshot` | [1162](../../src/endpoint_host.rs#L1162) | receiver-type-required |
| `select_model` | `map_endpoint` | [1165](../../src/endpoint_host.rs#L1165) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2637) |
| `select_model` | `Some` | [1165](../../src/endpoint_host.rs#L1165) | external-constructor-callback-or-unresolved |
| `select_model` | `model_is_configured` | [1166](../../src/endpoint_host.rs#L1166) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1856) |
| `select_model` | `Err` | [1167](../../src/endpoint_host.rs#L1167) | external-constructor-callback-or-unresolved |
| `select_model` | `(self.clock)().map_err` | [1173](../../src/endpoint_host.rs#L1173) | receiver-type-required |
| `select_model` | `(self.clock)` | [1173](../../src/endpoint_host.rs#L1173) | external-constructor-callback-or-unresolved |
| `select_model` | `request_hash` | [1174](../../src/endpoint_host.rs#L1174) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `select_model` | `self             .management             .select_model(SelectModelOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 provider,                 model,                 reasoning_effort: effort,                 started_at: &timestamp,             })             .map_err` | [1175](../../src/endpoint_host.rs#L1175) | receiver-type-required |
| `select_model` | `self             .management             .select_model` | [1175](../../src/endpoint_host.rs#L1175) | receiver-type-required |
| `select_model` | `mark_management_handoff` | [1187](../../src/endpoint_host.rs#L1187) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `select_model` | `authority             .config_mutation_succeeded(session_id)             .map_err` | [1188](../../src/endpoint_host.rs#L1188) | receiver-type-required |
| `select_model` | `authority             .config_mutation_succeeded` | [1188](../../src/endpoint_host.rs#L1188) | receiver-type-required |
| `select_model` | `map_production_route_for` | [1190](../../src/endpoint_host.rs#L1190) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `select_model` | `Value::String` | [1193](../../src/endpoint_host.rs#L1193) | external-constructor-callback-or-unresolved |
| `select_model` | `to_ijson` | [1195](../../src/endpoint_host.rs#L1195) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `cancel` | `require_object_fields` | [1203](../../src/endpoint_host.rs#L1203) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `cancel` | `required_string` | [1204](../../src/endpoint_host.rs#L1204) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `cancel` | `self.delivery_authority.as_ref().ok_or_else` | [1205](../../src/endpoint_host.rs#L1205) | receiver-type-required |
| `cancel` | `self.delivery_authority.as_ref` | [1205](../../src/endpoint_host.rs#L1205) | receiver-type-required |
| `cancel` | `EndpointRpcFailure::new` | [1206](../../src/endpoint_host.rs#L1206) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `cancel` | `self.admission_gate` | [1212](../../src/endpoint_host.rs#L1212) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1492) |
| `cancel` | `gate.lock().map_err` | [1213](../../src/endpoint_host.rs#L1213) | receiver-type-required |
| `cancel` | `gate.lock` | [1213](../../src/endpoint_host.rs#L1213) | receiver-type-required |
| `cancel` | `internal_failure` | [1213](../../src/endpoint_host.rs#L1213) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `cancel` | `(self.clock)().map_err` | [1214](../../src/endpoint_host.rs#L1214) | receiver-type-required |
| `cancel` | `(self.clock)` | [1214](../../src/endpoint_host.rs#L1214) | external-constructor-callback-or-unresolved |
| `cancel` | `endpoint_origin` | [1215](../../src/endpoint_host.rs#L1215) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2316) |
| `cancel` | `authority             .cancel(session_id, &timestamp, &origin)             .map_err` | [1221](../../src/endpoint_host.rs#L1221) | receiver-type-required |
| `cancel` | `authority             .cancel` | [1221](../../src/endpoint_host.rs#L1221) | receiver-type-required |
| `cancel` | `map_production_route_for` | [1223](../../src/endpoint_host.rs#L1223) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `cancel` | `mark_event_handoff` | [1224](../../src/endpoint_host.rs#L1224) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2302) |
| `cancel` | `to_ijson` | [1225](../../src/endpoint_host.rs#L1225) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `rename_session` | `require_object_fields` | [1233](../../src/endpoint_host.rs#L1233) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `rename_session` | `required_string` | [1234](../../src/endpoint_host.rs#L1234) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `rename_session` | `required_string_allow_whitespace` | [1235](../../src/endpoint_host.rs#L1235) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2212) |
| `rename_session` | `validate_session_title(title).map_err` | [1236](../../src/endpoint_host.rs#L1236) | receiver-type-required |
| `rename_session` | `validate_session_title` | [1236](../../src/endpoint_host.rs#L1236) | [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2331) |
| `rename_session` | `EndpointRpcFailure::new` | [1237](../../src/endpoint_host.rs#L1237), [1244](../../src/endpoint_host.rs#L1244) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `rename_session` | `self.delivery_authority.as_ref().ok_or_else` | [1243](../../src/endpoint_host.rs#L1243) | receiver-type-required |
| `rename_session` | `self.delivery_authority.as_ref` | [1243](../../src/endpoint_host.rs#L1243) | receiver-type-required |
| `rename_session` | `self.admission_gate` | [1250](../../src/endpoint_host.rs#L1250) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1492) |
| `rename_session` | `gate.lock().map_err` | [1251](../../src/endpoint_host.rs#L1251) | receiver-type-required |
| `rename_session` | `gate.lock` | [1251](../../src/endpoint_host.rs#L1251) | receiver-type-required |
| `rename_session` | `internal_failure` | [1251](../../src/endpoint_host.rs#L1251) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `rename_session` | `(self.clock)().map_err` | [1252](../../src/endpoint_host.rs#L1252) | receiver-type-required |
| `rename_session` | `(self.clock)` | [1252](../../src/endpoint_host.rs#L1252) | external-constructor-callback-or-unresolved |
| `rename_session` | `endpoint_origin` | [1253](../../src/endpoint_host.rs#L1253) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2316) |
| `rename_session` | `authority             .rename(session_id, &timestamp, &origin, title)             .map_err` | [1259](../../src/endpoint_host.rs#L1259) | receiver-type-required |
| `rename_session` | `authority             .rename` | [1259](../../src/endpoint_host.rs#L1259) | receiver-type-required |
| `rename_session` | `map_production_route_for` | [1261](../../src/endpoint_host.rs#L1261) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `rename_session` | `mark_event_handoff` | [1262](../../src/endpoint_host.rs#L1262) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2302) |
| `rename_session` | `to_ijson` | [1263](../../src/endpoint_host.rs#L1263) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `update_queue` | `require_object_fields` | [1271](../../src/endpoint_host.rs#L1271) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `update_queue` | `required_string` | [1276](../../src/endpoint_host.rs#L1276), [1277](../../src/endpoint_host.rs#L1277) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter(&#124;value&#124; *value > 0)             .ok_or_else` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix` | [1278](../../src/endpoint_host.rs#L1278) | receiver-type-required |
| `update_queue` | `value.parse::<u64>().ok` | [1280](../../src/endpoint_host.rs#L1280) | receiver-type-required |
| `update_queue` | `value.parse::<u64>` | [1280](../../src/endpoint_host.rs#L1280) | receiver-type-required |
| `update_queue` | `invalid` | [1282](../../src/endpoint_host.rs#L1282), [1283](../../src/endpoint_host.rs#L1283), [1312](../../src/endpoint_host.rs#L1312) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `update_queue` | `queue_action` | [1283](../../src/endpoint_host.rs#L1283) | [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2365) |
| `update_queue` | `payload.get("action").ok_or_else` | [1283](../../src/endpoint_host.rs#L1283) | receiver-type-required |
| `update_queue` | `payload.get` | [1283](../../src/endpoint_host.rs#L1283) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref().ok_or_else` | [1284](../../src/endpoint_host.rs#L1284) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref` | [1284](../../src/endpoint_host.rs#L1284) | receiver-type-required |
| `update_queue` | `EndpointRpcFailure::new` | [1285](../../src/endpoint_host.rs#L1285) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `update_queue` | `self.admission_gate` | [1291](../../src/endpoint_host.rs#L1291) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1492) |
| `update_queue` | `gate.lock().map_err` | [1292](../../src/endpoint_host.rs#L1292) | receiver-type-required |
| `update_queue` | `gate.lock` | [1292](../../src/endpoint_host.rs#L1292) | receiver-type-required |
| `update_queue` | `internal_failure` | [1292](../../src/endpoint_host.rs#L1292), [1338](../../src/endpoint_host.rs#L1338) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `update_queue` | `request.rpc_id.clone` | [1294](../../src/endpoint_host.rs#L1294), [1295](../../src/endpoint_host.rs#L1295) | receiver-type-required |
| `update_queue` | `endpoint_origin` | [1297](../../src/endpoint_host.rs#L1297), [1303](../../src/endpoint_host.rs#L1303) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2316) |
| `update_queue` | `action.with_origin` | [1303](../../src/endpoint_host.rs#L1303) | receiver-type-required |
| `update_queue` | `transaction             .validate()             .map_err` | [1310](../../src/endpoint_host.rs#L1310) | receiver-type-required |
| `update_queue` | `transaction             .validate` | [1310](../../src/endpoint_host.rs#L1310) | receiver-type-required |
| `update_queue` | `to_ijson` | [1313](../../src/endpoint_host.rs#L1313), [1347](../../src/endpoint_host.rs#L1347) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `update_queue` | `request_hash` | [1314](../../src/endpoint_host.rs#L1314) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `update_queue` | `(self.clock)().map_err` | [1315](../../src/endpoint_host.rs#L1315) | receiver-type-required |
| `update_queue` | `(self.clock)` | [1315](../../src/endpoint_host.rs#L1315) | external-constructor-callback-or-unresolved |
| `update_queue` | `self             .management             .prepare_queue_transaction(QueueTransactionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 target_seq,                 action: &action,                 retract_origin: &transaction.retract_origin,                 replacement_origin: transaction.replacement_origin(),                 asset_digests: &[],                 started_at: &timestamp,             })             .map_err` | [1316](../../src/endpoint_host.rs#L1316) | receiver-type-required |
| `update_queue` | `self             .management             .prepare_queue_transaction` | [1316](../../src/endpoint_host.rs#L1316) | receiver-type-required |
| `update_queue` | `transaction.replacement_origin` | [1325](../../src/endpoint_host.rs#L1325) | receiver-type-required |
| `update_queue` | `authority                     .execute(session_id, &transaction)                     .map_err` | [1333](../../src/endpoint_host.rs#L1333) | receiver-type-required |
| `update_queue` | `authority                     .execute` | [1333](../../src/endpoint_host.rs#L1333) | receiver-type-required |
| `update_queue` | `map_production_route_for` | [1335](../../src/endpoint_host.rs#L1335) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `update_queue` | `result                     .validate_for(&transaction)                     .map_err` | [1336](../../src/endpoint_host.rs#L1336) | receiver-type-required |
| `update_queue` | `result                     .validate_for` | [1336](../../src/endpoint_host.rs#L1336) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction(&request.rpc_id, queue_decision(&result.outcome))                     .map_err` | [1339](../../src/endpoint_host.rs#L1339) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction` | [1339](../../src/endpoint_host.rs#L1339) | receiver-type-required |
| `update_queue` | `queue_decision` | [1340](../../src/endpoint_host.rs#L1340) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2439) |
| `update_queue` | `mark_management_handoff` | [1346](../../src/endpoint_host.rs#L1346) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `update_queue` | `Err` | [1350](../../src/endpoint_host.rs#L1350) | external-constructor-callback-or-unresolved |
| `update_queue` | `map_persisted_queue_rejection` | [1350](../../src/endpoint_host.rs#L1350) | [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2464) |
| `prompt` | `require_object_fields` | [1360](../../src/endpoint_host.rs#L1360) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `prompt` | `required_string` | [1365](../../src/endpoint_host.rs#L1365), [1366](../../src/endpoint_host.rs#L1366) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `prompt` | `Err` | [1369](../../src/endpoint_host.rs#L1369) | external-constructor-callback-or-unresolved |
| `prompt` | `invalid` | [1369](../../src/endpoint_host.rs#L1369), [1378](../../src/endpoint_host.rs#L1378), [1380](../../src/endpoint_host.rs#L1380) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `prompt` | `optional_string` | [1371](../../src/endpoint_host.rs#L1371) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2233) |
| `prompt` | `validate_time_zone` | [1372](../../src/endpoint_host.rs#L1372) | [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2478) |
| `prompt` | `serde_json::from_value(             payload                 .get("content")                 .cloned()                 .ok_or_else(&#124;&#124; invalid("content is required"))?,         )         .map_err` | [1374](../../src/endpoint_host.rs#L1374) | receiver-type-required |
| `prompt` | `serde_json::from_value` | [1374](../../src/endpoint_host.rs#L1374) | external-constructor-callback-or-unresolved |
| `prompt` | `payload                 .get("content")                 .cloned()                 .ok_or_else` | [1375](../../src/endpoint_host.rs#L1375) | receiver-type-required |
| `prompt` | `payload                 .get("content")                 .cloned` | [1375](../../src/endpoint_host.rs#L1375) | receiver-type-required |
| `prompt` | `payload                 .get` | [1375](../../src/endpoint_host.rs#L1375) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref().ok_or_else` | [1381](../../src/endpoint_host.rs#L1381) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref` | [1381](../../src/endpoint_host.rs#L1381) | receiver-type-required |
| `prompt` | `EndpointRpcFailure::new` | [1382](../../src/endpoint_host.rs#L1382) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `prompt` | `self.input_admission.with_active_session` | [1388](../../src/endpoint_host.rs#L1388) | receiver-type-required |
| `prompt` | `map_production_route_for` | [1390](../../src/endpoint_host.rs#L1390), [1405](../../src/endpoint_host.rs#L1405) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts(session_id, &parts)                     .map_err` | [1392](../../src/endpoint_host.rs#L1392) | receiver-type-required |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts` | [1392](../../src/endpoint_host.rs#L1392) | receiver-type-required |
| `prompt` | `(self.clock)().map_err` | [1396](../../src/endpoint_host.rs#L1396) | receiver-type-required |
| `prompt` | `(self.clock)` | [1396](../../src/endpoint_host.rs#L1396) | external-constructor-callback-or-unresolved |
| `prompt` | `endpoint_origin` | [1397](../../src/endpoint_host.rs#L1397) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2316) |
| `prompt` | `authority                     .prompt(session_id, &timestamp, &origin, &materialized, steer)                     .map_err` | [1403](../../src/endpoint_host.rs#L1403) | receiver-type-required |
| `prompt` | `authority                     .prompt` | [1403](../../src/endpoint_host.rs#L1403) | receiver-type-required |
| `prompt` | `mark_event_handoff` | [1406](../../src/endpoint_host.rs#L1406) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2302) |
| `prompt` | `to_ijson` | [1407](../../src/endpoint_host.rs#L1407) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `attachment` | `require_object_fields` | [1413](../../src/endpoint_host.rs#L1413) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `attachment` | `required_string` | [1418](../../src/endpoint_host.rs#L1418), [1419](../../src/endpoint_host.rs#L1419) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `attachment` | `self.ensure_active_session` | [1420](../../src/endpoint_host.rs#L1420) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1499) |
| `attachment` | `self             .attachments             .read_authorized(session_id, attachment_id)             .map_err` | [1421](../../src/endpoint_host.rs#L1421) | receiver-type-required |
| `attachment` | `self             .attachments             .read_authorized` | [1421](../../src/endpoint_host.rs#L1421) | receiver-type-required |
| `attachment` | `to_ijson` | [1425](../../src/endpoint_host.rs#L1425) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `fork_session` | `require_object_fields` | [1433](../../src/endpoint_host.rs#L1433) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `fork_session` | `required_string` | [1438](../../src/endpoint_host.rs#L1438) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `fork_session` | `optional_u64` | [1439](../../src/endpoint_host.rs#L1439) | [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2260) |
| `fork_session` | `optional_bool(payload, "ephemeral")?.unwrap_or` | [1440](../../src/endpoint_host.rs#L1440) | receiver-type-required |
| `fork_session` | `optional_bool` | [1440](../../src/endpoint_host.rs#L1440) | [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2223) |
| `fork_session` | `(self.clock)().map_err` | [1441](../../src/endpoint_host.rs#L1441) | receiver-type-required |
| `fork_session` | `(self.clock)` | [1441](../../src/endpoint_host.rs#L1441) | external-constructor-callback-or-unresolved |
| `fork_session` | `request_hash` | [1442](../../src/endpoint_host.rs#L1442) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `fork_session` | `self             .management             .fork_session(ForkSessionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 source_session_id: session_id,                 at_endpoint_seq: at_seq,                 started_at: &timestamp,                 principal: &self.principal,                 ephemeral,                 preset: payload.get("preset"),                 preset_mcp: self                     .preset_mcp                     .as_deref()                     .map(&#124;source&#124; source as &dyn endpoint::PresetMcpSource),                 preset_models: Some(&ConfiguredModels),                 user_agent_dir: &self.user_agent_dir,             })             .map_err` | [1443](../../src/endpoint_host.rs#L1443) | receiver-type-required |
| `fork_session` | `self             .management             .fork_session` | [1443](../../src/endpoint_host.rs#L1443) | receiver-type-required |
| `fork_session` | `payload.get` | [1453](../../src/endpoint_host.rs#L1453) | receiver-type-required |
| `fork_session` | `self                     .preset_mcp                     .as_deref()                     .map` | [1454](../../src/endpoint_host.rs#L1454) | receiver-type-required |
| `fork_session` | `self                     .preset_mcp                     .as_deref` | [1454](../../src/endpoint_host.rs#L1454) | receiver-type-required |
| `fork_session` | `Some` | [1458](../../src/endpoint_host.rs#L1458) | external-constructor-callback-or-unresolved |
| `fork_session` | `mark_management_handoff` | [1462](../../src/endpoint_host.rs#L1462) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `fork_session` | `to_ijson` | [1463](../../src/endpoint_host.rs#L1463) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `discard_session` | `require_object_fields` | [1474](../../src/endpoint_host.rs#L1474) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `discard_session` | `required_string` | [1475](../../src/endpoint_host.rs#L1475) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2203) |
| `discard_session` | `self.input_admission.with_session_gate` | [1476](../../src/endpoint_host.rs#L1476) | receiver-type-required |
| `discard_session` | `map_production_route_for` | [1478](../../src/endpoint_host.rs#L1478) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1904) |
| `discard_session` | `(self.clock)().map_err` | [1480](../../src/endpoint_host.rs#L1480) | receiver-type-required |
| `discard_session` | `(self.clock)` | [1480](../../src/endpoint_host.rs#L1480) | external-constructor-callback-or-unresolved |
| `discard_session` | `request_hash` | [1481](../../src/endpoint_host.rs#L1481) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2274) |
| `discard_session` | `self                     .management                     .discard_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [1482](../../src/endpoint_host.rs#L1482) | receiver-type-required |
| `discard_session` | `self                     .management                     .discard_session` | [1482](../../src/endpoint_host.rs#L1482) | receiver-type-required |
| `discard_session` | `mark_management_handoff` | [1486](../../src/endpoint_host.rs#L1486) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2288) |
| `discard_session` | `to_ijson` | [1487](../../src/endpoint_host.rs#L1487) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `admission_gate` | `self.input_admission             .gates()             .gate(session_id)             .map_err` | [1493](../../src/endpoint_host.rs#L1493) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates()             .gate` | [1493](../../src/endpoint_host.rs#L1493) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates` | [1493](../../src/endpoint_host.rs#L1493) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id(session_id).map_err` | [1500](../../src/endpoint_host.rs#L1500) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id` | [1500](../../src/endpoint_host.rs#L1500) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `ensure_active_session` | `invalid` | [1500](../../src/endpoint_host.rs#L1500) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `ensure_active_session` | `self.storage_root.join("archive").join(session_id).is_dir` | [1501](../../src/endpoint_host.rs#L1501) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("archive").join` | [1501](../../src/endpoint_host.rs#L1501) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join` | [1501](../../src/endpoint_host.rs#L1501), [1508](../../src/endpoint_host.rs#L1508) | receiver-type-required |
| `ensure_active_session` | `Err` | [1502](../../src/endpoint_host.rs#L1502), [1509](../../src/endpoint_host.rs#L1509) | external-constructor-callback-or-unresolved |
| `ensure_active_session` | `EndpointRpcFailure::new` | [1502](../../src/endpoint_host.rs#L1502), [1509](../../src/endpoint_host.rs#L1509) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `ensure_active_session` | `self.storage_root.join("threads").join(session_id).is_dir` | [1508](../../src/endpoint_host.rs#L1508) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("threads").join` | [1508](../../src/endpoint_host.rs#L1508) | receiver-type-required |
| `ensure_active_session` | `Ok` | [1515](../../src/endpoint_host.rs#L1515) | external-constructor-callback-or-unresolved |
| `capabilities` | `Self::implemented_capabilities` | [1521](../../src/endpoint_host.rs#L1521) | external-constructor-callback-or-unresolved |
| `capabilities` | `self.routes.as_ref` | [1522](../../src/endpoint_host.rs#L1522) | receiver-type-required |
| `capabilities` | `capabilities.extend` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `capabilities` | `routes.capabilities` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `capabilities` | `self.provider_readiness.is_some` | [1525](../../src/endpoint_host.rs#L1525) | receiver-type-required |
| `capabilities` | `capabilities.insert` | [1526](../../src/endpoint_host.rs#L1526), [1527](../../src/endpoint_host.rs#L1527), [1528](../../src/endpoint_host.rs#L1528), [1531](../../src/endpoint_host.rs#L1531), [1532](../../src/endpoint_host.rs#L1532), [1533](../../src/endpoint_host.rs#L1533), [1536](../../src/endpoint_host.rs#L1536) | receiver-type-required |
| `capabilities` | `"session.models".to_owned` | [1526](../../src/endpoint_host.rs#L1526) | receiver-type-required |
| `capabilities` | `"models.list".to_owned` | [1527](../../src/endpoint_host.rs#L1527) | receiver-type-required |
| `capabilities` | `"session.selectModel".to_owned` | [1528](../../src/endpoint_host.rs#L1528) | receiver-type-required |
| `capabilities` | `self.delivery_authority.is_some` | [1530](../../src/endpoint_host.rs#L1530) | receiver-type-required |
| `capabilities` | `"session.prompt".to_owned` | [1531](../../src/endpoint_host.rs#L1531) | receiver-type-required |
| `capabilities` | `"session.cancel".to_owned` | [1532](../../src/endpoint_host.rs#L1532) | receiver-type-required |
| `capabilities` | `"session.rename".to_owned` | [1533](../../src/endpoint_host.rs#L1533) | receiver-type-required |
| `capabilities` | `self.queue_authority.is_some` | [1535](../../src/endpoint_host.rs#L1535) | receiver-type-required |
| `capabilities` | `"session.updateQueue".to_owned` | [1536](../../src/endpoint_host.rs#L1536) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref()                 .map_or_else` | [1543](../../src/endpoint_host.rs#L1543) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref` | [1543](../../src/endpoint_host.rs#L1543) | receiver-type-required |
| `extension_capabilities` | `routes.capabilities` | [1545](../../src/endpoint_host.rs#L1545) | receiver-type-required |
| `extension_capabilities` | `BTreeSet::new` | [1547](../../src/endpoint_host.rs#L1547) | external-constructor-callback-or-unresolved |
| `method_class` | `TEKES_UNARY_ROUTES.iter().find` | [1552](../../src/endpoint_host.rs#L1552) | receiver-type-required |
| `method_class` | `TEKES_UNARY_ROUTES.iter` | [1552](../../src/endpoint_host.rs#L1552) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then(&#124;routes&#124; routes.extension_method_class(method))                 .unwrap_or` | [1559](../../src/endpoint_host.rs#L1559) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then` | [1559](../../src/endpoint_host.rs#L1559) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref` | [1559](../../src/endpoint_host.rs#L1559) | receiver-type-required |
| `method_class` | `routes.extension_method_class` | [1562](../../src/endpoint_host.rs#L1562) | receiver-type-required |
| `validate_request` | `serde_json::from_slice(             &request                 .payload                 .canonical_bytes()                 .map_err(&#124;_&#124; HostFailure::InvalidRequest)?,         )         .map_err` | [1568](../../src/endpoint_host.rs#L1568) | receiver-type-required |
| `validate_request` | `serde_json::from_slice` | [1568](../../src/endpoint_host.rs#L1568) | external-constructor-callback-or-unresolved |
| `validate_request` | `request                 .payload                 .canonical_bytes()                 .map_err` | [1569](../../src/endpoint_host.rs#L1569) | receiver-type-required |
| `validate_request` | `request                 .payload                 .canonical_bytes` | [1569](../../src/endpoint_host.rs#L1569) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter()             .any` | [1575](../../src/endpoint_host.rs#L1575) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter` | [1575](../../src/endpoint_host.rs#L1575) | receiver-type-required |
| `validate_request` | `validate_production_payload(&request.method, &payload)                 .map_err` | [1579](../../src/endpoint_host.rs#L1579) | receiver-type-required |
| `validate_request` | `validate_production_payload` | [1579](../../src/endpoint_host.rs#L1579) | [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2616) |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload(&request.method, &payload)                 .map_err` | [1582](../../src/endpoint_host.rs#L1582) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload` | [1582](../../src/endpoint_host.rs#L1582) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or` | [1582](../../src/endpoint_host.rs#L1582) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter` | [1582](../../src/endpoint_host.rs#L1582) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref` | [1582](../../src/endpoint_host.rs#L1582) | receiver-type-required |
| `validate_request` | `routes.capabilities().contains` | [1584](../../src/endpoint_host.rs#L1584) | receiver-type-required |
| `validate_request` | `routes.capabilities` | [1584](../../src/endpoint_host.rs#L1584) | receiver-type-required |
| `call` | `Box::pin` | [1592](../../src/endpoint_host.rs#L1592) | external-constructor-callback-or-unresolved |
| `call` | `std::future::ready` | [1592](../../src/endpoint_host.rs#L1592) | external-constructor-callback-or-unresolved |
| `call` | `self.execute` | [1592](../../src/endpoint_host.rs#L1592) | receiver-type-required |
| `open` | `root.as_ref` | [1607](../../src/endpoint_host.rs#L1607) | receiver-type-required |
| `open` | `Ok` | [1608](../../src/endpoint_host.rs#L1608) | external-constructor-callback-or-unresolved |
| `open` | `ProductionEndpointHost::open` | [1609](../../src/endpoint_host.rs#L1609) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L587) |
| `open` | `EndpointDispatcher::new` | [1610](../../src/endpoint_host.rs#L1610) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open` | `RpcRegistry::open` | [1610](../../src/endpoint_host.rs#L1610) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open` | `EndpointSubscriptionHub::default` | [1611](../../src/endpoint_host.rs#L1611) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [1620](../../src/endpoint_host.rs#L1620) | receiver-type-required |
| `open_with_clock` | `Ok` | [1621](../../src/endpoint_host.rs#L1621) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `ProductionEndpointHost::open_with_clock` | [1622](../../src/endpoint_host.rs#L1622) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L602) |
| `open_with_clock` | `EndpointDispatcher::new` | [1623](../../src/endpoint_host.rs#L1623) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_clock` | `RpcRegistry::open` | [1623](../../src/endpoint_host.rs#L1623) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_clock` | `EndpointSubscriptionHub::default` | [1624](../../src/endpoint_host.rs#L1624) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `root.as_ref` | [1634](../../src/endpoint_host.rs#L1634) | receiver-type-required |
| `open_with_routes` | `Ok` | [1635](../../src/endpoint_host.rs#L1635) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `ProductionEndpointHost::open_with_routes` | [1636](../../src/endpoint_host.rs#L1636) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L611) |
| `open_with_routes` | `Some` | [1636](../../src/endpoint_host.rs#L1636) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `EndpointDispatcher::new` | [1637](../../src/endpoint_host.rs#L1637) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_routes` | `RpcRegistry::open` | [1637](../../src/endpoint_host.rs#L1637) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_routes` | `EndpointSubscriptionHub::default` | [1638](../../src/endpoint_host.rs#L1638) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `root.as_ref` | [1648](../../src/endpoint_host.rs#L1648) | receiver-type-required |
| `open_with_route_authority` | `Ok` | [1649](../../src/endpoint_host.rs#L1649) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `ProductionEndpointHost::open_with_route_authority` | [1650](../../src/endpoint_host.rs#L1650) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L687) |
| `open_with_route_authority` | `EndpointDispatcher::new` | [1659](../../src/endpoint_host.rs#L1659) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_route_authority` | `RpcRegistry::open` | [1659](../../src/endpoint_host.rs#L1659) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_route_authority` | `EndpointSubscriptionHub::default` | [1660](../../src/endpoint_host.rs#L1660) | external-constructor-callback-or-unresolved |
| `open_with_authorities` | `Self::open_with_full_authorities` | [1671](../../src/endpoint_host.rs#L1671) | [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1682) |
| `open_with_full_authorities` | `root.as_ref` | [1691](../../src/endpoint_host.rs#L1691) | receiver-type-required |
| `open_with_full_authorities` | `Ok` | [1692](../../src/endpoint_host.rs#L1692) | external-constructor-callback-or-unresolved |
| `open_with_full_authorities` | `ProductionEndpointHost::open_with_full_authorities` | [1693](../../src/endpoint_host.rs#L1693) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L638) |
| `open_with_full_authorities` | `EndpointDispatcher::new` | [1702](../../src/endpoint_host.rs#L1702) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L451) |
| `open_with_full_authorities` | `RpcRegistry::open` | [1702](../../src/endpoint_host.rs#L1702) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L123) |
| `open_with_full_authorities` | `EndpointSubscriptionHub::default` | [1703](../../src/endpoint_host.rs#L1703) | external-constructor-callback-or-unresolved |
| `dispatch` | `Ok` | [1711](../../src/endpoint_host.rs#L1711) | external-constructor-callback-or-unresolved |
| `dispatch` | `self.dispatcher.dispatch` | [1711](../../src/endpoint_host.rs#L1711) | receiver-type-required |
| `admission_gates` | `self.host.admission_gates` | [1726](../../src/endpoint_host.rs#L1726) | receiver-type-required |
| `has_incomplete_management_operation` | `self.host.has_incomplete_management_operation` | [1733](../../src/endpoint_host.rs#L1733) | receiver-type-required |
| `success` | `Some` | [1740](../../src/endpoint_host.rs#L1740) | external-constructor-callback-or-unresolved |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or(config.settings.default_provider.as_deref())         .unwrap_or_default` | [1749](../../src/endpoint_host.rs#L1749) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.workspace.policy.provider.as_deref())         .or` | [1749](../../src/endpoint_host.rs#L1749) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or` | [1749](../../src/endpoint_host.rs#L1749) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map` | [1749](../../src/endpoint_host.rs#L1749), [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref` | [1749](../../src/endpoint_host.rs#L1749), [1756](../../src/endpoint_host.rs#L1756), [1763](../../src/endpoint_host.rs#L1763) | receiver-type-required |
| `model_projection` | `settings.provider.as_str` | [1752](../../src/endpoint_host.rs#L1752) | receiver-type-required |
| `model_projection` | `config.workspace.policy.provider.as_deref` | [1753](../../src/endpoint_host.rs#L1753) | receiver-type-required |
| `model_projection` | `config.settings.default_provider.as_deref` | [1754](../../src/endpoint_host.rs#L1754) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or(config.settings.default_model.as_deref())         .unwrap_or_default` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.workspace.policy.model.as_deref())         .or` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or` | [1756](../../src/endpoint_host.rs#L1756) | receiver-type-required |
| `model_projection` | `settings.model.as_str` | [1759](../../src/endpoint_host.rs#L1759) | receiver-type-required |
| `model_projection` | `config.workspace.policy.model.as_deref` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `model_projection` | `config.settings.default_model.as_deref` | [1761](../../src/endpoint_host.rs#L1761) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .and_then` | [1763](../../src/endpoint_host.rs#L1763) | receiver-type-required |
| `model_projection` | `settings.reasoning_effort.as_deref` | [1766](../../src/endpoint_host.rs#L1766) | receiver-type-required |
| `model_projection` | `Vec::new` | [1767](../../src/endpoint_host.rs#L1767), [1768](../../src/endpoint_host.rs#L1768), [1774](../../src/endpoint_host.rs#L1774) | external-constructor-callback-or-unresolved |
| `model_projection` | `provider.name.as_deref().unwrap_or` | [1770](../../src/endpoint_host.rs#L1770) | receiver-type-required |
| `model_projection` | `provider.name.as_deref` | [1770](../../src/endpoint_host.rs#L1770) | receiver-type-required |
| `model_projection` | `readiness             .iter()             .find` | [1771](../../src/endpoint_host.rs#L1771) | receiver-type-required |
| `model_projection` | `readiness             .iter` | [1771](../../src/endpoint_host.rs#L1771) | receiver-type-required |
| `model_projection` | `provider.models.iter().filter` | [1775](../../src/endpoint_host.rs#L1775) | receiver-type-required |
| `model_projection` | `provider.models.iter` | [1775](../../src/endpoint_host.rs#L1775) | receiver-type-required |
| `model_projection` | `provider::resolve_profile(provider, configured).ok` | [1776](../../src/endpoint_host.rs#L1776) | receiver-type-required |
| `model_projection` | `provider::resolve_profile` | [1776](../../src/endpoint_host.rs#L1776) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_projection` | `profile                 .as_ref()                 .map(&#124;profile&#124; profile.reasoning_efforts().to_vec())                 .unwrap_or_default` | [1777](../../src/endpoint_host.rs#L1777) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .map` | [1777](../../src/endpoint_host.rs#L1777) | receiver-type-required |
| `model_projection` | `profile                 .as_ref` | [1777](../../src/endpoint_host.rs#L1777), [1781](../../src/endpoint_host.rs#L1781) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts().to_vec` | [1779](../../src/endpoint_host.rs#L1779) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts` | [1779](../../src/endpoint_host.rs#L1779) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .and_then` | [1781](../../src/endpoint_host.rs#L1781) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort().map` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `efforts                 .iter()                 .any` | [1784](../../src/endpoint_host.rs#L1784) | receiver-type-required |
| `model_projection` | `efforts                 .iter` | [1784](../../src/endpoint_host.rs#L1784) | receiver-type-required |
| `model_projection` | `effort.is_empty` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter().all` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `effort.as_bytes` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `Err` | [1788](../../src/endpoint_host.rs#L1788), [1791](../../src/endpoint_host.rs#L1791), [1797](../../src/endpoint_host.rs#L1797) | external-constructor-callback-or-unresolved |
| `model_projection` | `internal_failure` | [1788](../../src/endpoint_host.rs#L1788), [1791](../../src/endpoint_host.rs#L1791), [1797](../../src/endpoint_host.rs#L1797) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>().len` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `efforts.iter` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `efforts.len` | [1790](../../src/endpoint_host.rs#L1790) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref()                 .is_some_and` | [1793](../../src/endpoint_host.rs#L1793) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref` | [1793](../../src/endpoint_host.rs#L1793) | receiver-type-required |
| `model_projection` | `efforts.contains` | [1795](../../src/endpoint_host.rs#L1795) | receiver-type-required |
| `model_projection` | `efforts.is_empty` | [1801](../../src/endpoint_host.rs#L1801) | receiver-type-required |
| `model_projection` | `default_effort.as_ref` | [1807](../../src/endpoint_host.rs#L1807) | receiver-type-required |
| `model_projection` | `Value::String` | [1808](../../src/endpoint_host.rs#L1808), [1830](../../src/endpoint_host.rs#L1830) | external-constructor-callback-or-unresolved |
| `model_projection` | `default.clone` | [1808](../../src/endpoint_host.rs#L1808) | receiver-type-required |
| `model_projection` | `models.push` | [1811](../../src/endpoint_host.rs#L1811) | receiver-type-required |
| `model_projection` | `groups.push` | [1813](../../src/endpoint_host.rs#L1813) | receiver-type-required |
| `model_projection` | `runtime.map` | [1814](../../src/endpoint_host.rs#L1814) | receiver-type-required |
| `model_projection` | `failures.push` | [1816](../../src/endpoint_host.rs#L1816), [1821](../../src/endpoint_host.rs#L1821) | receiver-type-required |
| `model_projection` | `effort.to_owned` | [1830](../../src/endpoint_host.rs#L1830) | receiver-type-required |
| `model_projection` | `model_is_configured` | [1832](../../src/endpoint_host.rs#L1832) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1856) |
| `model_projection` | `to_ijson` | [1833](../../src/endpoint_host.rs#L1833) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2270) |
| `model_available` | `model_is_configured` | [1852](../../src/endpoint_host.rs#L1852) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1856) |
| `model_is_configured` | `config         .providers         .providers         .iter()         .find` | [1862](../../src/endpoint_host.rs#L1862) | receiver-type-required |
| `model_is_configured` | `config         .providers         .providers         .iter` | [1862](../../src/endpoint_host.rs#L1862) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter()         .find` | [1870](../../src/endpoint_host.rs#L1870) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter` | [1870](../../src/endpoint_host.rs#L1870) | receiver-type-required |
| `model_is_configured` | `effort.is_none_or` | [1877](../../src/endpoint_host.rs#L1877) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile(provider, model).is_ok_and` | [1878](../../src/endpoint_host.rs#L1878) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile` | [1878](../../src/endpoint_host.rs#L1878) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter()                 .any` | [1879](../../src/endpoint_host.rs#L1879) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter` | [1879](../../src/endpoint_host.rs#L1879) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts` | [1879](../../src/endpoint_host.rs#L1879) | receiver-type-required |
| `map_production_route_for` | `production_failure_is_exact_for` | [1905](../../src/endpoint_host.rs#L1905) | [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1938) |
| `map_production_route_for` | `sanitized_route_log_message` | [1906](../../src/endpoint_host.rs#L1906) | [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1918) |
| `map_production_route_for` | `internal_failure` | [1913](../../src/endpoint_host.rs#L1913) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `map_production_route_for` | `map_production_route` | [1915](../../src/endpoint_host.rs#L1915) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1896) |
| `sanitized_route_log_message` | `message.len().min` | [1919](../../src/endpoint_host.rs#L1919) | receiver-type-required |
| `sanitized_route_log_message` | `message.len` | [1919](../../src/endpoint_host.rs#L1919) | receiver-type-required |
| `sanitized_route_log_message` | `message.is_char_boundary` | [1920](../../src/endpoint_host.rs#L1920) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse(         &serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect("route log message is JSON"),     )     .expect` | [1923](../../src/endpoint_host.rs#L1923) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse` | [1923](../../src/endpoint_host.rs#L1923) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `sanitized_route_log_message` | `serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect` | [1924](../../src/endpoint_host.rs#L1924) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_vec` | [1924](../../src/endpoint_host.rs#L1924) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `tools::SecretScanner::default().scan` | [1928](../../src/endpoint_host.rs#L1928) | receiver-type-required |
| `sanitized_route_log_message` | `tools::SecretScanner::default` | [1928](../../src/endpoint_host.rs#L1928) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `"<withheld>".to_owned` | [1930](../../src/endpoint_host.rs#L1930), [1935](../../src/endpoint_host.rs#L1935) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then(&#124;value&#124; value["detail"].as_str().map(str::to_owned))         .unwrap_or_else` | [1932](../../src/endpoint_host.rs#L1932) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then` | [1932](../../src/endpoint_host.rs#L1932) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok` | [1932](../../src/endpoint_host.rs#L1932) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value` | [1932](../../src/endpoint_host.rs#L1932) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `value["detail"].as_str().map` | [1934](../../src/endpoint_host.rs#L1934) | receiver-type-required |
| `sanitized_route_log_message` | `value["detail"].as_str` | [1934](../../src/endpoint_host.rs#L1934) | receiver-type-required |
| `production_failure_is_exact_for` | `route_allows_error` | [1942](../../src/endpoint_host.rs#L1942) | [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2080) |
| `production_failure_is_exact_for` | `route_error_is_exact` | [1942](../../src/endpoint_host.rs#L1942) | [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1945) |
| `route_error_is_exact` | `route_error_message` | [1946](../../src/endpoint_host.rs#L1946) | [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2028) |
| `route_error_is_exact` | `error.details.canonical_bytes` | [1952](../../src/endpoint_host.rs#L1952) | receiver-type-required |
| `route_error_is_exact` | `serde_json::from_slice::<Value>` | [1955](../../src/endpoint_host.rs#L1955) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `error.code.as_str` | [1959](../../src/endpoint_host.rs#L1959) | receiver-type-required |
| `route_error_is_exact` | `details.is_empty` | [1960](../../src/endpoint_host.rs#L1960) | receiver-type-required |
| `route_error_is_exact` | `exact_string_fields` | [1961](../../src/endpoint_host.rs#L1961), [1963](../../src/endpoint_host.rs#L1963), [1966](../../src/endpoint_host.rs#L1966), [1969](../../src/endpoint_host.rs#L1969), [1972](../../src/endpoint_host.rs#L1972), [1974](../../src/endpoint_host.rs#L1974), [1975](../../src/endpoint_host.rs#L1975), [1977](../../src/endpoint_host.rs#L1977), [1979](../../src/endpoint_host.rs#L1979), [1991](../../src/endpoint_host.rs#L1991), [2012](../../src/endpoint_host.rs#L2012), [2014](../../src/endpoint_host.rs#L2014) | [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2059) |
| `route_error_is_exact` | `details.len` | [1981](../../src/endpoint_host.rs#L1981), [1988](../../src/endpoint_host.rs#L1988), [1994](../../src/endpoint_host.rs#L1994) | receiver-type-required |
| `route_error_is_exact` | `details.get("sessionId").is_some_and` | [1982](../../src/endpoint_host.rs#L1982) | receiver-type-required |
| `route_error_is_exact` | `details.get` | [1982](../../src/endpoint_host.rs#L1982), [1988](../../src/endpoint_host.rs#L1988) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("atSeq")                     .is_some_and` | [1983](../../src/endpoint_host.rs#L1983) | receiver-type-required |
| `route_error_is_exact` | `details                     .get` | [1983](../../src/endpoint_host.rs#L1983), [1995](../../src/endpoint_host.rs#L1995), [2015](../../src/endpoint_host.rs#L2015) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64().is_some` | [1985](../../src/endpoint_host.rs#L1985) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64` | [1985](../../src/endpoint_host.rs#L1985) | receiver-type-required |
| `route_error_is_exact` | `details.get("limit").and_then` | [1988](../../src/endpoint_host.rs#L1988) | receiver-type-required |
| `route_error_is_exact` | `Some` | [1988](../../src/endpoint_host.rs#L1988) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then(Value::as_str)                     .is_some_and` | [1995](../../src/endpoint_host.rs#L1995), [2015](../../src/endpoint_host.rs#L2015) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then` | [1995](../../src/endpoint_host.rs#L1995), [2015](../../src/endpoint_host.rs#L2015) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter()                             .any` | [2019](../../src/endpoint_host.rs#L2019) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter` | [2019](../../src/endpoint_host.rs#L2019) | receiver-type-required |
| `route_error_is_exact` | `known.as_str` | [2021](../../src/endpoint_host.rs#L2021) | receiver-type-required |
| `route_error_message` | `Some` | [2029](../../src/endpoint_host.rs#L2029) | external-constructor-callback-or-unresolved |
| `exact_string_fields` | `required         .iter()         .any` | [2064](../../src/endpoint_host.rs#L2064) | receiver-type-required |
| `exact_string_fields` | `required         .iter` | [2064](../../src/endpoint_host.rs#L2064) | receiver-type-required |
| `exact_string_fields` | `details.get(*key).is_some_and` | [2066](../../src/endpoint_host.rs#L2066), [2069](../../src/endpoint_host.rs#L2069) | receiver-type-required |
| `exact_string_fields` | `details.get` | [2066](../../src/endpoint_host.rs#L2066), [2069](../../src/endpoint_host.rs#L2069) | receiver-type-required |
| `exact_string_fields` | `optional             .iter()             .any` | [2067](../../src/endpoint_host.rs#L2067) | receiver-type-required |
| `exact_string_fields` | `optional             .iter` | [2067](../../src/endpoint_host.rs#L2067) | receiver-type-required |
| `exact_string_fields` | `value.is_string` | [2069](../../src/endpoint_host.rs#L2069) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter(&#124;key&#124; details.contains_key(**key))         .count` | [2073](../../src/endpoint_host.rs#L2073) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter` | [2073](../../src/endpoint_host.rs#L2073) | receiver-type-required |
| `exact_string_fields` | `optional         .iter` | [2073](../../src/endpoint_host.rs#L2073) | receiver-type-required |
| `exact_string_fields` | `details.contains_key` | [2075](../../src/endpoint_host.rs#L2075) | receiver-type-required |
| `exact_string_fields` | `details.len` | [2077](../../src/endpoint_host.rs#L2077) | receiver-type-required |
| `exact_string_fields` | `required.len` | [2077](../../src/endpoint_host.rs#L2077) | receiver-type-required |
| `internal_failure` | `EndpointRpcFailure::new` | [2188](../../src/endpoint_host.rs#L2188) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `failure` | `Some` | [2195](../../src/endpoint_host.rs#L2195) | external-constructor-callback-or-unresolved |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [2204](../../src/endpoint_host.rs#L2204) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter` | [2204](../../src/endpoint_host.rs#L2204) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2204](../../src/endpoint_host.rs#L2204) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then` | [2204](../../src/endpoint_host.rs#L2204) | receiver-type-required |
| `required_string` | `value         .as_object` | [2204](../../src/endpoint_host.rs#L2204) | receiver-type-required |
| `required_string` | `object.get` | [2206](../../src/endpoint_host.rs#L2206) | receiver-type-required |
| `required_string` | `value.is_empty` | [2208](../../src/endpoint_host.rs#L2208) | receiver-type-required |
| `required_string` | `invalid` | [2209](../../src/endpoint_host.rs#L2209) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .ok_or_else` | [2216](../../src/endpoint_host.rs#L2216) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2216](../../src/endpoint_host.rs#L2216) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then` | [2216](../../src/endpoint_host.rs#L2216) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object` | [2216](../../src/endpoint_host.rs#L2216) | receiver-type-required |
| `required_string_allow_whitespace` | `object.get` | [2218](../../src/endpoint_host.rs#L2218) | receiver-type-required |
| `required_string_allow_whitespace` | `invalid` | [2220](../../src/endpoint_host.rs#L2220) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `optional_bool` | `value.as_object().and_then` | [2224](../../src/endpoint_host.rs#L2224) | receiver-type-required |
| `optional_bool` | `value.as_object` | [2224](../../src/endpoint_host.rs#L2224) | receiver-type-required |
| `optional_bool` | `object.get` | [2224](../../src/endpoint_host.rs#L2224) | receiver-type-required |
| `optional_bool` | `Ok` | [2225](../../src/endpoint_host.rs#L2225) | external-constructor-callback-or-unresolved |
| `optional_bool` | `value         .as_bool()         .map(Some)         .ok_or_else` | [2227](../../src/endpoint_host.rs#L2227) | receiver-type-required |
| `optional_bool` | `value         .as_bool()         .map` | [2227](../../src/endpoint_host.rs#L2227) | receiver-type-required |
| `optional_bool` | `value         .as_bool` | [2227](../../src/endpoint_host.rs#L2227) | receiver-type-required |
| `optional_bool` | `invalid` | [2230](../../src/endpoint_host.rs#L2230) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `optional_string` | `value.as_object().and_then` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_string` | `value.as_object` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_string` | `object.get` | [2234](../../src/endpoint_host.rs#L2234) | receiver-type-required |
| `optional_string` | `Ok` | [2235](../../src/endpoint_host.rs#L2235) | external-constructor-callback-or-unresolved |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map(Some)         .ok_or_else` | [2237](../../src/endpoint_host.rs#L2237) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map` | [2237](../../src/endpoint_host.rs#L2237) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter` | [2237](../../src/endpoint_host.rs#L2237) | receiver-type-required |
| `optional_string` | `value         .as_str` | [2237](../../src/endpoint_host.rs#L2237) | receiver-type-required |
| `optional_string` | `value.is_empty` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `optional_string` | `invalid` | [2241](../../src/endpoint_host.rs#L2241) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `require_object_fields` | `value         .as_object()         .ok_or_else` | [2249](../../src/endpoint_host.rs#L2249) | receiver-type-required |
| `require_object_fields` | `value         .as_object` | [2249](../../src/endpoint_host.rs#L2249) | receiver-type-required |
| `require_object_fields` | `invalid` | [2251](../../src/endpoint_host.rs#L2251), [2255](../../src/endpoint_host.rs#L2255) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `require_object_fields` | `object.keys().any` | [2252](../../src/endpoint_host.rs#L2252) | receiver-type-required |
| `require_object_fields` | `object.keys` | [2252](../../src/endpoint_host.rs#L2252) | receiver-type-required |
| `require_object_fields` | `allowed.contains` | [2252](../../src/endpoint_host.rs#L2252) | receiver-type-required |
| `require_object_fields` | `key.as_str` | [2252](../../src/endpoint_host.rs#L2252) | receiver-type-required |
| `require_object_fields` | `required.iter().any` | [2253](../../src/endpoint_host.rs#L2253) | receiver-type-required |
| `require_object_fields` | `required.iter` | [2253](../../src/endpoint_host.rs#L2253) | receiver-type-required |
| `require_object_fields` | `object.contains_key` | [2253](../../src/endpoint_host.rs#L2253) | receiver-type-required |
| `require_object_fields` | `Err` | [2255](../../src/endpoint_host.rs#L2255) | external-constructor-callback-or-unresolved |
| `require_object_fields` | `Ok` | [2257](../../src/endpoint_host.rs#L2257) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value.as_object().and_then` | [2261](../../src/endpoint_host.rs#L2261) | receiver-type-required |
| `optional_u64` | `value.as_object` | [2261](../../src/endpoint_host.rs#L2261) | receiver-type-required |
| `optional_u64` | `object.get` | [2261](../../src/endpoint_host.rs#L2261) | receiver-type-required |
| `optional_u64` | `Ok` | [2262](../../src/endpoint_host.rs#L2262) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value         .as_u64()         .map(Some)         .ok_or_else` | [2264](../../src/endpoint_host.rs#L2264) | receiver-type-required |
| `optional_u64` | `value         .as_u64()         .map` | [2264](../../src/endpoint_host.rs#L2264) | receiver-type-required |
| `optional_u64` | `value         .as_u64` | [2264](../../src/endpoint_host.rs#L2264) | receiver-type-required |
| `optional_u64` | `invalid` | [2267](../../src/endpoint_host.rs#L2267) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `to_ijson` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(internal_json)?).map_err` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [2271](../../src/endpoint_host.rs#L2271) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value).map_err` | [2271](../../src/endpoint_host.rs#L2271) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [2271](../../src/endpoint_host.rs#L2271) | external-constructor-callback-or-unresolved |
| `request_hash` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)             .map_err` | [2276](../../src/endpoint_host.rs#L2276) | receiver-type-required |
| `request_hash` | `serde_json::from_slice` | [2276](../../src/endpoint_host.rs#L2276) | external-constructor-callback-or-unresolved |
| `request_hash` | `request.payload.canonical_bytes().map_err` | [2276](../../src/endpoint_host.rs#L2276) | receiver-type-required |
| `request_hash` | `request.payload.canonical_bytes` | [2276](../../src/endpoint_host.rs#L2276) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec(&json!({         "type":"client-request",         "rpcId":request.rpc_id,         "method":request.operation,         "payload":payload,     }))     .map_err` | [2278](../../src/endpoint_host.rs#L2278) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec` | [2278](../../src/endpoint_host.rs#L2278) | external-constructor-callback-or-unresolved |
| `request_hash` | `EndpointRpcFailure::new` | [2284](../../src/endpoint_host.rs#L2284) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `request_hash` | `Ok` | [2285](../../src/endpoint_host.rs#L2285) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "management".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "management-operation".to_owned(),                 id: request.rpc_id.clone(),                 seq: None,             }),         })         .map_err` | [2289](../../src/endpoint_host.rs#L2289) | receiver-type-required |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off` | [2289](../../src/endpoint_host.rs#L2289) | receiver-type-required |
| `mark_management_handoff` | `"management".to_owned` | [2292](../../src/endpoint_host.rs#L2292) | receiver-type-required |
| `mark_management_handoff` | `Some` | [2293](../../src/endpoint_host.rs#L2293) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `"management-operation".to_owned` | [2294](../../src/endpoint_host.rs#L2294) | receiver-type-required |
| `mark_management_handoff` | `request.rpc_id.clone` | [2295](../../src/endpoint_host.rs#L2295) | receiver-type-required |
| `mark_management_handoff` | `EndpointRpcFailure::new` | [2299](../../src/endpoint_host.rs#L2299) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "locked-append".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "event-origin".to_owned(),                 id: request.rpc_id.clone(),                 seq: Some(seq),             }),         })         .map_err` | [2303](../../src/endpoint_host.rs#L2303) | receiver-type-required |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off` | [2303](../../src/endpoint_host.rs#L2303) | receiver-type-required |
| `mark_event_handoff` | `"locked-append".to_owned` | [2306](../../src/endpoint_host.rs#L2306) | receiver-type-required |
| `mark_event_handoff` | `Some` | [2307](../../src/endpoint_host.rs#L2307), [2310](../../src/endpoint_host.rs#L2310) | external-constructor-callback-or-unresolved |
| `mark_event_handoff` | `"event-origin".to_owned` | [2308](../../src/endpoint_host.rs#L2308) | receiver-type-required |
| `mark_event_handoff` | `request.rpc_id.clone` | [2309](../../src/endpoint_host.rs#L2309) | receiver-type-required |
| `mark_event_handoff` | `internal_failure` | [2313](../../src/endpoint_host.rs#L2313) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `endpoint_origin` | `principal.to_owned` | [2323](../../src/endpoint_host.rs#L2323) | receiver-type-required |
| `endpoint_origin` | `endpoint::ORIGIN_CLIENT.to_owned` | [2324](../../src/endpoint_host.rs#L2324) | receiver-type-required |
| `endpoint_origin` | `session_id.to_owned` | [2325](../../src/endpoint_host.rs#L2325) | receiver-type-required |
| `endpoint_origin` | `operation.to_owned` | [2326](../../src/endpoint_host.rs#L2326) | receiver-type-required |
| `endpoint_origin` | `rpc_id.to_owned` | [2327](../../src/endpoint_host.rs#L2327) | receiver-type-required |
| `validate_session_title` | `title.trim` | [2332](../../src/endpoint_host.rs#L2332) | receiver-type-required |
| `validate_session_title` | `title.is_empty` | [2333](../../src/endpoint_host.rs#L2333) | receiver-type-required |
| `validate_session_title` | `title.len` | [2334](../../src/endpoint_host.rs#L2334) | receiver-type-required |
| `validate_session_title` | `title.chars().any` | [2335](../../src/endpoint_host.rs#L2335) | receiver-type-required |
| `validate_session_title` | `title.chars` | [2335](../../src/endpoint_host.rs#L2335) | receiver-type-required |
| `validate_session_title` | `Err` | [2337](../../src/endpoint_host.rs#L2337) | external-constructor-callback-or-unresolved |
| `validate_session_title` | `Ok` | [2339](../../src/endpoint_host.rs#L2339) | external-constructor-callback-or-unresolved |
| `queue_action` | `value         .as_object()         .ok_or_else` | [2366](../../src/endpoint_host.rs#L2366) | receiver-type-required |
| `queue_action` | `value         .as_object` | [2366](../../src/endpoint_host.rs#L2366) | receiver-type-required |
| `queue_action` | `invalid` | [2368](../../src/endpoint_host.rs#L2368), [2372](../../src/endpoint_host.rs#L2372), [2387](../../src/endpoint_host.rs#L2387), [2411](../../src/endpoint_host.rs#L2411) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `queue_action` | `object         .get("kind")         .and_then(Value::as_str)         .ok_or_else` | [2369](../../src/endpoint_host.rs#L2369) | receiver-type-required |
| `queue_action` | `object         .get("kind")         .and_then` | [2369](../../src/endpoint_host.rs#L2369) | receiver-type-required |
| `queue_action` | `object         .get` | [2369](../../src/endpoint_host.rs#L2369) | receiver-type-required |
| `queue_action` | `object.len` | [2374](../../src/endpoint_host.rs#L2374), [2375](../../src/endpoint_host.rs#L2375), [2377](../../src/endpoint_host.rs#L2377) | receiver-type-required |
| `queue_action` | `Ok` | [2374](../../src/endpoint_host.rs#L2374), [2375](../../src/endpoint_host.rs#L2375), [2409](../../src/endpoint_host.rs#L2409) | external-constructor-callback-or-unresolved |
| `queue_action` | `object.contains_key` | [2378](../../src/endpoint_host.rs#L2378) | receiver-type-required |
| `queue_action` | `object                     .keys()                     .all` | [2379](../../src/endpoint_host.rs#L2379) | receiver-type-required |
| `queue_action` | `object                     .keys` | [2379](../../src/endpoint_host.rs#L2379) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter(&#124;values&#124; !values.is_empty())                 .ok_or_else` | [2383](../../src/endpoint_host.rs#L2383) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter` | [2383](../../src/endpoint_host.rs#L2383) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then` | [2383](../../src/endpoint_host.rs#L2383) | receiver-type-required |
| `queue_action` | `object                 .get` | [2383](../../src/endpoint_host.rs#L2383) | receiver-type-required |
| `queue_action` | `values.is_empty` | [2386](../../src/endpoint_host.rs#L2386) | receiver-type-required |
| `queue_action` | `Vec::with_capacity` | [2388](../../src/endpoint_host.rs#L2388) | external-constructor-callback-or-unresolved |
| `queue_action` | `values.len` | [2388](../../src/endpoint_host.rs#L2388) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then(Value::as_str)                     .ok_or_else` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `value                     .as_object` | [2390](../../src/endpoint_host.rs#L2390) | receiver-type-required |
| `queue_action` | `block.len` | [2393](../../src/endpoint_host.rs#L2393) | receiver-type-required |
| `queue_action` | `block.get("type").and_then` | [2394](../../src/endpoint_host.rs#L2394) | receiver-type-required |
| `queue_action` | `block.get` | [2394](../../src/endpoint_host.rs#L2394), [2396](../../src/endpoint_host.rs#L2396) | receiver-type-required |
| `queue_action` | `Some` | [2394](../../src/endpoint_host.rs#L2394) | external-constructor-callback-or-unresolved |
| `queue_action` | `EndpointRpcFailure::new` | [2399](../../src/endpoint_host.rs#L2399) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `queue_action` | `content.push` | [2405](../../src/endpoint_host.rs#L2405) | receiver-type-required |
| `queue_action` | `block.to_owned` | [2406](../../src/endpoint_host.rs#L2406) | receiver-type-required |
| `queue_action` | `Err` | [2411](../../src/endpoint_host.rs#L2411) | external-constructor-callback-or-unresolved |
| `map_queue_rejection` | `EndpointRpcFailure::new` | [2421](../../src/endpoint_host.rs#L2421), [2426](../../src/endpoint_host.rs#L2426), [2431](../../src/endpoint_host.rs#L2431) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `queue_decision` | `match code {                     worker_control::QueueTransactionRejectCode::QueueItemNotFound => {                         "queue-item-not-found"                     }                     worker_control::QueueTransactionRejectCode::SteerUnavailable => {                         "steer-unavailable"                     }                     worker_control::QueueTransactionRejectCode::AttachmentError => {                         "attachment-error"                     }                 }                 .to_owned` | [2446](../../src/endpoint_host.rs#L2446) | receiver-type-required |
| `queue_decision` | `reason.clone` | [2458](../../src/endpoint_host.rs#L2458) | receiver-type-required |
| `map_persisted_queue_rejection` | `internal_failure` | [2473](../../src/endpoint_host.rs#L2473) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `map_persisted_queue_rejection` | `map_queue_rejection` | [2475](../../src/endpoint_host.rs#L2475) | [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2415) |
| `validate_time_zone` | `Ok` | [2480](../../src/endpoint_host.rs#L2480), [2501](../../src/endpoint_host.rs#L2501) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `value.trim` | [2482](../../src/endpoint_host.rs#L2482) | receiver-type-required |
| `validate_time_zone` | `value.is_ascii` | [2483](../../src/endpoint_host.rs#L2483) | receiver-type-required |
| `validate_time_zone` | `value.split('/').count` | [2484](../../src/endpoint_host.rs#L2484) | receiver-type-required |
| `validate_time_zone` | `value.split` | [2484](../../src/endpoint_host.rs#L2484), [2485](../../src/endpoint_host.rs#L2485) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate().any` | [2485](../../src/endpoint_host.rs#L2485) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate` | [2485](../../src/endpoint_host.rs#L2485) | receiver-type-required |
| `validate_time_zone` | `segment.is_empty` | [2486](../../src/endpoint_host.rs#L2486) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate().any` | [2489](../../src/endpoint_host.rs#L2489) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate` | [2489](../../src/endpoint_host.rs#L2489) | receiver-type-required |
| `validate_time_zone` | `segment.bytes` | [2489](../../src/endpoint_host.rs#L2489) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphanumeric` | [2490](../../src/endpoint_host.rs#L2490) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphabetic` | [2491](../../src/endpoint_host.rs#L2491) | receiver-type-required |
| `validate_time_zone` | `Err` | [2495](../../src/endpoint_host.rs#L2495), [2499](../../src/endpoint_host.rs#L2499) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `invalid` | [2495](../../src/endpoint_host.rs#L2495), [2499](../../src/endpoint_host.rs#L2499) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `validate_time_zone` | `Path::new("/usr/share/zoneinfo").join` | [2497](../../src/endpoint_host.rs#L2497) | receiver-type-required |
| `validate_time_zone` | `Path::new` | [2497](../../src/endpoint_host.rs#L2497) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `zone.is_file` | [2498](../../src/endpoint_host.rs#L2498) | receiver-type-required |
| `map_prompt_error` | `invalid` | [2508](../../src/endpoint_host.rs#L2508) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2633) |
| `map_prompt_error` | `attachment_failure` | [2509](../../src/endpoint_host.rs#L2509) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2541) |
| `map_prompt_error` | `EndpointRpcFailure::new` | [2510](../../src/endpoint_host.rs#L2510), [2515](../../src/endpoint_host.rs#L2515) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `map_prompt_error` | `internal_failure` | [2520](../../src/endpoint_host.rs#L2520) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `map_attachment_read_error` | `EndpointRpcFailure::new` | [2526](../../src/endpoint_host.rs#L2526), [2531](../../src/endpoint_host.rs#L2531) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `map_attachment_read_error` | `attachment_failure` | [2536](../../src/endpoint_host.rs#L2536) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2541) |
| `map_attachment_read_error` | `internal_failure` | [2537](../../src/endpoint_host.rs#L2537) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2187) |
| `attachment_failure` | `EndpointRpcFailure::new` | [2542](../../src/endpoint_host.rs#L2542) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `validate_extension_payload` | `Err` | [2605](../../src/endpoint_host.rs#L2605) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `EndpointRpcFailure::new` | [2605](../../src/endpoint_host.rs#L2605) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `validate_extension_payload` | `require_object_fields` | [2612](../../src/endpoint_host.rs#L2612) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `validate_extension_payload` | `Ok` | [2613](../../src/endpoint_host.rs#L2613) | external-constructor-callback-or-unresolved |
| `validate_production_payload` | `require_object_fields` | [2618](../../src/endpoint_host.rs#L2618), [2619](../../src/endpoint_host.rs#L2619), [2624](../../src/endpoint_host.rs#L2624) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2244) |
| `validate_production_payload` | `validate_extension_payload` | [2629](../../src/endpoint_host.rs#L2629) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2564) |
| `invalid` | `EndpointRpcFailure::new` | [2634](../../src/endpoint_host.rs#L2634) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `map_endpoint` | `session.unwrap_or_default` | [2638](../../src/endpoint_host.rs#L2638) | receiver-type-required |
| `map_endpoint` | `EndpointRpcFailure::new` | [2640](../../src/endpoint_host.rs#L2640), [2645](../../src/endpoint_host.rs#L2645), [2650](../../src/endpoint_host.rs#L2650) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `map_management` | `EndpointRpcFailure::new` | [2656](../../src/endpoint_host.rs#L2656), [2661](../../src/endpoint_host.rs#L2661), [2666](../../src/endpoint_host.rs#L2666), [2671](../../src/endpoint_host.rs#L2671), [2676](../../src/endpoint_host.rs#L2676), [2682](../../src/endpoint_host.rs#L2682), [2684](../../src/endpoint_host.rs#L2684), [2689](../../src/endpoint_host.rs#L2689), [2694](../../src/endpoint_host.rs#L2694), [2699](../../src/endpoint_host.rs#L2699), [2704](../../src/endpoint_host.rs#L2704), [2709](../../src/endpoint_host.rs#L2709), [2714](../../src/endpoint_host.rs#L2714), [2719](../../src/endpoint_host.rs#L2719), [2736](../../src/endpoint_host.rs#L2736), [2742](../../src/endpoint_host.rs#L2742), [2748](../../src/endpoint_host.rs#L2748), [2751](../../src/endpoint_host.rs#L2751), [2753](../../src/endpoint_host.rs#L2753) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `map_management` | `invalid.detail` | [2687](../../src/endpoint_host.rs#L2687) | receiver-type-required |
| `map_management` | `Value::String` | [2734](../../src/endpoint_host.rs#L2734) | external-constructor-callback-or-unresolved |
| `internal_json` | `EndpointRpcFailure::new` | [2758](../../src/endpoint_host.rs#L2758) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `internal_schema` | `EndpointRpcFailure::new` | [2762](../../src/endpoint_host.rs#L2762) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `internal_assembly` | `EndpointRpcFailure::new` | [2766](../../src/endpoint_host.rs#L2766) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2776) |
| `new` | `IJsonValue::parse(&serde_json::to_vec(&details).unwrap_or_else(&#124;_&#124; b"{}".to_vec()))                 .unwrap_or_else` | [2778](../../src/endpoint_host.rs#L2778) | receiver-type-required |
| `new` | `IJsonValue::parse` | [2778](../../src/endpoint_host.rs#L2778) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `new` | `serde_json::to_vec(&details).unwrap_or_else` | [2778](../../src/endpoint_host.rs#L2778) | receiver-type-required |
| `new` | `serde_json::to_vec` | [2778](../../src/endpoint_host.rs#L2778) | external-constructor-callback-or-unresolved |
| `new` | `b"{}".to_vec` | [2778](../../src/endpoint_host.rs#L2778) | receiver-type-required |
| `new` | `IJsonValue::parse_str("{}").expect` | [2779](../../src/endpoint_host.rs#L2779) | receiver-type-required |
| `new` | `IJsonValue::parse_str` | [2779](../../src/endpoint_host.rs#L2779) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `new` | `code.to_owned` | [2781](../../src/endpoint_host.rs#L2781) | receiver-type-required |
| `new` | `message.to_owned` | [2782](../../src/endpoint_host.rs#L2782) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; EndpointAssemblyError::Clock(error.to_string()))?         .as_secs` | [2789](../../src/endpoint_host.rs#L2789) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [2789](../../src/endpoint_host.rs#L2789) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since` | [2789](../../src/endpoint_host.rs#L2789) | receiver-type-required |
| `system_timestamp` | `SystemTime::now` | [2789](../../src/endpoint_host.rs#L2789) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `EndpointAssemblyError::Clock` | [2791](../../src/endpoint_host.rs#L2791), [2794](../../src/endpoint_host.rs#L2794) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `error.to_string` | [2791](../../src/endpoint_host.rs#L2791) | receiver-type-required |
| `system_timestamp` | `i64::try_from(seconds / 86_400)         .map_err` | [2793](../../src/endpoint_host.rs#L2793) | receiver-type-required |
| `system_timestamp` | `i64::try_from` | [2793](../../src/endpoint_host.rs#L2793) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `"timestamp exceeds i64".to_owned` | [2794](../../src/endpoint_host.rs#L2794) | receiver-type-required |
| `system_timestamp` | `civil_from_days` | [2796](../../src/endpoint_host.rs#L2796) | [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2805) |
| `system_timestamp` | `Ok` | [2800](../../src/endpoint_host.rs#L2800) | external-constructor-callback-or-unresolved |
| `civil_from_days` | `i64::from` | [2816](../../src/endpoint_host.rs#L2816) | external-constructor-callback-or-unresolved |
