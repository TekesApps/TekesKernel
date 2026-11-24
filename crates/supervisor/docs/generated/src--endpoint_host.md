# tekes-supervisor::endpoint_host

[Package atlas](index.md) · [Source](../../src/endpoint_host.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::endpoint_host::EndpointRouteClass](../../src/endpoint_host.rs#L29) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::EndpointRoute](../../src/endpoint_host.rs#L37) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::TEKES_UNARY_ROUTES](../../src/endpoint_host.rs#L43) | const_item | `pub` |  |
| [tekes-supervisor::endpoint_host::route](../../src/endpoint_host.rs#L59) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointClock](../../src/endpoint_host.rs#L67) | type_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeModelReadiness](../../src/endpoint_host.rs#L70) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderStatus](../../src/endpoint_host.rs#L77) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderFailure](../../src/endpoint_host.rs#L83) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::RuntimeProviderReadiness](../../src/endpoint_host.rs#L91) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority](../../src/endpoint_host.rs#L100) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority::readiness](../../src/endpoint_host.rs#L101) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProviderReadinessAuthority::config_mutation_succeeded](../../src/endpoint_host.rs#L110) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority](../../src/endpoint_host.rs#L119) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::session_metadata_changed](../../src/endpoint_host.rs#L123) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::workspace_metadata_changed](../../src/endpoint_host.rs#L127) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::prompt](../../src/endpoint_host.rs#L129) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::cancel](../../src/endpoint_host.rs#L138) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::rename](../../src/endpoint_host.rs#L145) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionDeliveryAuthority::compact](../../src/endpoint_host.rs#L159) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::QueueTransactionAuthority](../../src/endpoint_host.rs#L176) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::QueueTransactionAuthority::execute](../../src/endpoint_host.rs#L177) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::recover_pending_management](../../src/endpoint_host.rs#L190) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::QueueRecoveryAdapter](../../src/endpoint_host.rs#L199) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::QueueRecoveryAdapter::execute](../../src/endpoint_host.rs#L204) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::SessionAdmissionGates](../../src/endpoint_host.rs#L241) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionAdmissionGates::gate](../../src/endpoint_host.rs#L246) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority](../../src/endpoint_host.rs#L265) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L272) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::gates](../../src/endpoint_host.rs#L280) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::root](../../src/endpoint_host.rs#L286) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_active_session](../../src/endpoint_host.rs#L290) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate](../../src/endpoint_host.rs#L315) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::session_details](../../src/endpoint_host.rs#L340) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes](../../src/endpoint_host.rs#L350) | trait_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::capabilities](../../src/endpoint_host.rs#L351) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_method_class](../../src/endpoint_host.rs#L356) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::validate_extension_payload](../../src/endpoint_host.rs#L362) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_failure_is_exact](../../src/endpoint_host.rs#L377) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointRoutes::execute](../../src/endpoint_host.rs#L385) | function_signature_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes](../../src/endpoint_host.rs#L396) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose](../../src/endpoint_host.rs#L401) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::owner](../../src/endpoint_host.rs#L429) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::capabilities](../../src/endpoint_host.rs#L435) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_method_class](../../src/endpoint_host.rs#L439) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::validate_extension_payload](../../src/endpoint_host.rs#L443) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_failure_is_exact](../../src/endpoint_host.rs#L453) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::execute](../../src/endpoint_host.rs#L462) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteCompositionError](../../src/endpoint_host.rs#L475) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L482) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteFailure](../../src/endpoint_host.rs#L492) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost](../../src/endpoint_host.rs#L508) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::with_user_agent_dir](../../src/endpoint_host.rs#L531) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description](../../src/endpoint_host.rs#L536) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L561) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L576) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L585) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities](../../src/endpoint_host.rs#L594) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission](../../src/endpoint_host.rs#L635) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L661) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L684) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities](../../src/endpoint_host.rs#L751) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates](../../src/endpoint_host.rs#L760) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::session_admission_authority](../../src/endpoint_host.rs#L765) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation](../../src/endpoint_host.rs#L769) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute](../../src/endpoint_host.rs#L778) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L786) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L814) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L863) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L886) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L903) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L965) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1002) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1022) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1081) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1111) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1149) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1238) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1295) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1311) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1348) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1371) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1378) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::reconcile_legacy_internal_prompt_inner](../../src/endpoint_host.rs#L1397) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::find_unique_matching_legacy_input](../../src/endpoint_host.rs#L1467) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::verify_and_sync_legacy_input](../../src/endpoint_host.rs#L1502) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities](../../src/endpoint_host.rs#L1546) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities](../../src/endpoint_host.rs#L1567) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class](../../src/endpoint_host.rs#L1577) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request](../../src/endpoint_host.rs#L1593) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::reconcile_legacy_internal_prompt](../../src/endpoint_host.rs#L1617) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointHost::call](../../src/endpoint_host.rs#L1625) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly](../../src/endpoint_host.rs#L1630) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open](../../src/endpoint_host.rs#L1637) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock](../../src/endpoint_host.rs#L1649) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes](../../src/endpoint_host.rs#L1662) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority](../../src/endpoint_host.rs#L1676) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities](../../src/endpoint_host.rs#L1698) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1716) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch](../../src/endpoint_host.rs#L1741) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub](../../src/endpoint_host.rs#L1749) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host](../../src/endpoint_host.rs#L1754) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates](../../src/endpoint_host.rs#L1759) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation](../../src/endpoint_host.rs#L1763) | function_item | `pub` |  |
| [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1771) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1779) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels](../../src/endpoint_host.rs#L1874) | struct_item | `pub` |  |
| [tekes-supervisor::endpoint_host::ConfiguredModels::model_available](../../src/endpoint_host.rs#L1877) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1888) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::runtime_failure_name](../../src/endpoint_host.rs#L1919) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1928) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1950) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1970) | function_item | `pub(crate)` |  |
| [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1977) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2061) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2093) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2114) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2226) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2247) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2258) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2268) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2295) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2337) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2366) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction](../../src/endpoint_host.rs#L2377) | enum_item | `private` |  |
| [tekes-supervisor::endpoint_host::PendingQueueAction::with_origin](../../src/endpoint_host.rs#L2384) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2400) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2450) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2474) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2499) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2513) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_prompt_error](../../src/endpoint_host.rs#L2539) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_attachment_read_error](../../src/endpoint_host.rs#L2559) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2576) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::attachment_reason](../../src/endpoint_host.rs#L2584) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2599) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2655) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2663) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::map_management](../../src/endpoint_host.rs#L2680) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_json](../../src/endpoint_host.rs#L2760) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_schema](../../src/endpoint_host.rs#L2764) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::internal_assembly](../../src/endpoint_host.rs#L2768) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure](../../src/endpoint_host.rs#L2772) | struct_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::system_timestamp](../../src/endpoint_host.rs#L2791) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2808) | function_item | `private` |  |
| [tekes-supervisor::endpoint_host::EndpointAssemblyError](../../src/endpoint_host.rs#L2824) | enum_item | `pub` |  |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::ToggleSync](../../src/endpoint_host.rs#L2845) | struct_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::ToggleSync::new](../../src/endpoint_host.rs#L2851) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::ToggleSync::full_sync](../../src/endpoint_host.rs#L2860) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::write_test_ledger](../../src/endpoint_host.rs#L2869) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears](../../src/endpoint_host.rs#L2917) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes](../../src/endpoint_host.rs#L3002) | function_item | `private` | test; #[cfg(test)] |

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
| `ExecutionCreateOperation` | `endpoint::ExecutionCreateOperation` | `private` |
| `ForkSessionOperation` | `endpoint::ForkSessionOperation` | `private` |
| `HostFailure` | `endpoint::HostFailure` | `private` |
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
| `SessionHostDescription` | `endpoint::SessionHostDescription` | `private` |
| `SessionOperationError` | `endpoint::SessionOperationError` | `private` |
| `SessionOperations` | `endpoint::SessionOperations` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `Digest` | `sha2::Digest` | `private` |
| `Sha256` | `sha2::Sha256` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `AtomicUsize` | `std::sync::atomic::AtomicUsize` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 15 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::session_operations::SessionOperations::open_sessions"]
  n1["endpoint::types::validate_session_id"]
  n2["schema::ijson::IJsonValue::parse"]
  n3["schema::ijson::IJsonValue::parse_str"]
  n4["tekes-supervisor::endpoint_host::ProviderReadinessAuthority::config_mutation_succeeded"]
  n5["tekes-supervisor::endpoint_host::SessionDeliveryAuthority::session_metadata_changed"]
  n6["tekes-supervisor::endpoint_host::SessionDeliveryAuthority::workspace_metadata_changed"]
  n7["tekes-supervisor::endpoint_host::SessionDeliveryAuthority::compact"]
  n8["tekes-supervisor::endpoint_host::recover_pending_management"]
  n9["tekes-supervisor::endpoint_host::QueueRecoveryAdapter::execute"]
  n10["tekes-supervisor::endpoint_host::SessionAdmissionGates::gate"]
  n11["tekes-supervisor::endpoint_host::queue_decision"]
  n12["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n13["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::gates"]
  n14["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::root"]
  n15["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_active_session"]
  n16["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate"]
  n17["tekes-supervisor::endpoint_host::session_details"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_method_class"]
  n19["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::validate_extension_payload"]
  n20["tekes-supervisor::endpoint_host::ProductionEndpointRoutes::extension_failure_is_exact"]
  n21["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose"]
  n22["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::owner"]
  n23["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::capabilities"]
  n24["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n25["tekes-supervisor::endpoint_host::route"]
  n7 --> n3
  n7 --> n24
  n8 --> n0
  n9 --> n11
  n10 --> n3
  n10 --> n24
  n15 --> n16
  n15 --> n17
  n15 --> n24
  n16 --> n1
  n16 --> n3
  n16 --> n24
  n17 --> n2
  n19 --> n2
  n19 --> n24
```

</details>

<details><summary>Functions 21–40: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::attachment::AttachmentAuthority::open"]
  n1["endpoint::mux::SessionEndpointCapability::required"]
  n2["endpoint::service::NativeEndpoint::open"]
  n3["endpoint::session_operations::SessionOperations::open_sessions"]
  n4["schema::ijson::IJsonValue::parse"]
  n5["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n6["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_method_class"]
  n7["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::validate_extension_payload"]
  n8["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::extension_failure_is_exact"]
  n9["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::execute"]
  n10["tekes-supervisor::endpoint_host::unsupported_route"]
  n11["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointHost::with_user_agent_dir"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointHost::mux_description"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_authorities"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n19["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission"]
  n20["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n21["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership"]
  n22["tekes-supervisor::endpoint_host::ProductionEndpointHost::implemented_capabilities"]
  n23["tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gates"]
  n24["tekes-supervisor::endpoint_host::ProductionEndpointHost::session_admission_authority"]
  n25["tekes-supervisor::endpoint_host::ProductionEndpointHost::has_incomplete_management_operation"]
  n7 --> n10
  n9 --> n10
  n10 --> n4
  n10 --> n11
  n13 --> n1
  n14 --> n18
  n15 --> n18
  n16 --> n18
  n17 --> n18
  n18 --> n21
  n19 --> n21
  n20 --> n21
  n21 --> n0
  n21 --> n2
  n21 --> n3
  n21 --> n5
```

</details>

<details><summary>Functions 41–60: 136 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["profile::config::ConfigRepository::open"]
  n2["store::platform::DirectoryLock::shared"]
  n3["tekes-supervisor::endpoint_host::ProductionEndpointHost::models"]
  n4["tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model"]
  n5["tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointHost::reconcile_legacy_internal_prompt_inner"]
  n15["tekes-supervisor::endpoint_host::find_unique_matching_legacy_input"]
  n16["tekes-supervisor::endpoint_host::verify_and_sync_legacy_input"]
  n17["tekes-supervisor::endpoint_host::success"]
  n18["tekes-supervisor::endpoint_host::model_projection"]
  n19["tekes-supervisor::endpoint_host::model_is_configured"]
  n20["tekes-supervisor::endpoint_host::map_production_route"]
  n21["tekes-supervisor::endpoint_host::map_production_route_for"]
  n22["tekes-supervisor::endpoint_host::internal_failure"]
  n23["tekes-supervisor::endpoint_host::failure"]
  n24["tekes-supervisor::endpoint_host::required_string"]
  n25["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n26["tekes-supervisor::endpoint_host::optional_bool"]
  n27["tekes-supervisor::endpoint_host::optional_string"]
  n28["tekes-supervisor::endpoint_host::require_object_fields"]
  n29["tekes-supervisor::endpoint_host::optional_u64"]
  n30["tekes-supervisor::endpoint_host::to_ijson"]
  n31["tekes-supervisor::endpoint_host::request_hash"]
  n32["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n33["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n34["tekes-supervisor::endpoint_host::endpoint_origin"]
  n35["tekes-supervisor::endpoint_host::validate_session_title"]
  n36["tekes-supervisor::endpoint_host::queue_action"]
  n37["tekes-supervisor::endpoint_host::queue_decision"]
  n38["tekes-supervisor::endpoint_host::map_persisted_queue_rejection"]
  n39["tekes-supervisor::endpoint_host::validate_time_zone"]
  n40["tekes-supervisor::endpoint_host::validate_extension_payload"]
  n41["tekes-supervisor::endpoint_host::invalid"]
  n42["tekes-supervisor::endpoint_host::map_endpoint"]
  n43["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n44["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute"]
  n45["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner"]
  n46["tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension"]
  n47["tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session"]
  n48["tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session"]
  n49["tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session"]
  n50["tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models"]
  n51["tools::guidance::IdentityProfile::parse"]
  n3 --> n18
  n3 --> n21
  n3 --> n24
  n3 --> n28
  n3 --> n42
  n3 --> n43
  n4 --> n12
  n4 --> n19
  n4 --> n21
  n4 --> n22
  n4 --> n24
  n4 --> n27
  n4 --> n28
  n4 --> n30
  n4 --> n31
  n4 --> n32
  n4 --> n42
  n4 --> n43
  n5 --> n12
  n5 --> n21
  n5 --> n22
  n5 --> n24
  n5 --> n28
  n5 --> n30
  n5 --> n33
  n5 --> n34
  n5 --> n43
  n6 --> n12
  n6 --> n21
  n6 --> n22
  n6 --> n24
  n6 --> n25
  n6 --> n28
  n6 --> n30
  n6 --> n33
  n6 --> n34
  n6 --> n35
  n6 --> n43
  n7 --> n12
  n7 --> n21
  n7 --> n22
  n7 --> n24
  n7 --> n28
  n7 --> n30
  n7 --> n31
  n7 --> n32
  n7 --> n34
  n7 --> n36
  n7 --> n37
  n7 --> n38
  n7 --> n41
  n7 --> n43
  n8 --> n21
  n8 --> n24
  n8 --> n27
  n8 --> n28
  n8 --> n30
  n8 --> n33
  n8 --> n34
  n8 --> n39
  n8 --> n41
  n8 --> n43
  n9 --> n13
  n9 --> n24
  n9 --> n28
  n9 --> n30
  n10 --> n24
  n10 --> n26
  n10 --> n28
  n10 --> n29
  n10 --> n30
  n10 --> n31
  n10 --> n32
  n11 --> n21
  n11 --> n24
  n11 --> n28
  n11 --> n30
  n11 --> n31
  n11 --> n32
  n13 --> n0
  n13 --> n41
  n13 --> n43
  n14 --> n0
  n14 --> n2
  n14 --> n16
  n14 --> n24
  n14 --> n27
  n14 --> n28
  n14 --> n34
  n14 --> n39
  n44 --> n17
  n44 --> n23
  n44 --> n45
  n45 --> n3
  n45 --> n4
  n45 --> n5
  n45 --> n6
  n45 --> n7
  n45 --> n8
  n45 --> n9
  n45 --> n10
  n45 --> n11
  n45 --> n46
  n45 --> n47
  n45 --> n48
  n45 --> n49
  n45 --> n50
  n46 --> n20
  n46 --> n21
  n46 --> n22
  n46 --> n40
  n46 --> n43
  n47 --> n21
  n47 --> n24
  n47 --> n28
  n47 --> n30
  n47 --> n31
  n47 --> n32
  n48 --> n24
  n48 --> n28
  n48 --> n30
  n48 --> n31
  n48 --> n32
  n49 --> n27
  n49 --> n28
  n49 --> n30
  n49 --> n31
  n49 --> n32
  n49 --> n41
  n49 --> n43
  n49 --> n51
  n50 --> n1
  n50 --> n18
  n50 --> n21
  n50 --> n22
  n50 --> n28
```

</details>

<details><summary>Functions 61–80: 23 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::host::EndpointDispatcher::new"]
  n1["endpoint::idempotency::RpcRegistry::open"]
  n2["provider::dialect::resolve_profile"]
  n3["store::tail::scan_valid_prefix"]
  n4["tekes-supervisor::endpoint_host::find_unique_matching_legacy_input"]
  n5["tekes-supervisor::endpoint_host::verify_and_sync_legacy_input"]
  n6["tekes-supervisor::endpoint_host::ProductionEndpointHost::capabilities"]
  n7["tekes-supervisor::endpoint_host::ProductionEndpointHost::extension_capabilities"]
  n8["tekes-supervisor::endpoint_host::ProductionEndpointHost::method_class"]
  n9["tekes-supervisor::endpoint_host::ProductionEndpointHost::validate_request"]
  n10["tekes-supervisor::endpoint_host::ProductionEndpointHost::reconcile_legacy_internal_prompt"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointHost::call"]
  n12["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open"]
  n13["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_clock"]
  n14["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_routes"]
  n15["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_route_authority"]
  n16["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_authorities"]
  n17["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities"]
  n18["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::dispatch"]
  n19["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::hub"]
  n20["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::host"]
  n21["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::admission_gates"]
  n22["tekes-supervisor::endpoint_host::ProductionEndpointAssembly::has_incomplete_management_operation"]
  n23["tekes-supervisor::endpoint_host::success"]
  n24["tekes-supervisor::endpoint_host::model_projection"]
  n25["tekes-supervisor::endpoint_host::model_is_configured"]
  n26["tekes-supervisor::endpoint_host::internal_failure"]
  n27["tekes-supervisor::endpoint_host::to_ijson"]
  n28["tekes-supervisor::endpoint_host::validate_production_payload"]
  n29["tekes-supervisor::endpoint_host::ProductionEndpointHost::open"]
  n30["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock"]
  n31["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes"]
  n32["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities"]
  n33["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority"]
  n5 --> n3
  n5 --> n4
  n9 --> n28
  n12 --> n0
  n12 --> n1
  n12 --> n29
  n13 --> n0
  n13 --> n1
  n13 --> n30
  n14 --> n0
  n14 --> n1
  n14 --> n31
  n15 --> n0
  n15 --> n1
  n15 --> n33
  n16 --> n17
  n17 --> n0
  n17 --> n1
  n17 --> n32
  n24 --> n2
  n24 --> n25
  n24 --> n26
  n24 --> n27
```

</details>

<details><summary>Functions 81–100: 19 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::dialect::resolve_profile"]
  n1["schema::ijson::IJsonValue::parse"]
  n2["tekes-supervisor::endpoint_host::ConfiguredModels::model_available"]
  n3["tekes-supervisor::endpoint_host::model_is_configured"]
  n4["tekes-supervisor::endpoint_host::runtime_failure_name"]
  n5["tekes-supervisor::endpoint_host::map_production_route"]
  n6["tekes-supervisor::endpoint_host::map_production_route_for"]
  n7["tekes-supervisor::endpoint_host::sanitized_route_log_message"]
  n8["tekes-supervisor::endpoint_host::production_failure_is_exact_for"]
  n9["tekes-supervisor::endpoint_host::route_error_is_exact"]
  n10["tekes-supervisor::endpoint_host::route_error_message"]
  n11["tekes-supervisor::endpoint_host::exact_string_fields"]
  n12["tekes-supervisor::endpoint_host::route_allows_error"]
  n13["tekes-supervisor::endpoint_host::internal_failure"]
  n14["tekes-supervisor::endpoint_host::failure"]
  n15["tekes-supervisor::endpoint_host::required_string"]
  n16["tekes-supervisor::endpoint_host::required_string_allow_whitespace"]
  n17["tekes-supervisor::endpoint_host::optional_bool"]
  n18["tekes-supervisor::endpoint_host::optional_string"]
  n19["tekes-supervisor::endpoint_host::require_object_fields"]
  n20["tekes-supervisor::endpoint_host::optional_u64"]
  n21["tekes-supervisor::endpoint_host::to_ijson"]
  n22["tekes-supervisor::endpoint_host::invalid"]
  n23["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n2 --> n3
  n3 --> n0
  n6 --> n5
  n6 --> n7
  n6 --> n8
  n6 --> n13
  n7 --> n1
  n8 --> n9
  n8 --> n12
  n9 --> n10
  n9 --> n11
  n13 --> n23
  n15 --> n22
  n16 --> n22
  n17 --> n22
  n18 --> n22
  n19 --> n22
  n20 --> n22
  n21 --> n1
```

</details>

<details><summary>Functions 101–120: 24 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::endpoint_host::internal_failure"]
  n1["tekes-supervisor::endpoint_host::require_object_fields"]
  n2["tekes-supervisor::endpoint_host::request_hash"]
  n3["tekes-supervisor::endpoint_host::mark_management_handoff"]
  n4["tekes-supervisor::endpoint_host::mark_event_handoff"]
  n5["tekes-supervisor::endpoint_host::endpoint_origin"]
  n6["tekes-supervisor::endpoint_host::validate_session_title"]
  n7["tekes-supervisor::endpoint_host::PendingQueueAction::with_origin"]
  n8["tekes-supervisor::endpoint_host::queue_action"]
  n9["tekes-supervisor::endpoint_host::map_queue_rejection"]
  n10["tekes-supervisor::endpoint_host::queue_decision"]
  n11["tekes-supervisor::endpoint_host::map_persisted_queue_rejection"]
  n12["tekes-supervisor::endpoint_host::validate_time_zone"]
  n13["tekes-supervisor::endpoint_host::map_prompt_error"]
  n14["tekes-supervisor::endpoint_host::map_attachment_read_error"]
  n15["tekes-supervisor::endpoint_host::attachment_failure"]
  n16["tekes-supervisor::endpoint_host::attachment_reason"]
  n17["tekes-supervisor::endpoint_host::validate_extension_payload"]
  n18["tekes-supervisor::endpoint_host::validate_production_payload"]
  n19["tekes-supervisor::endpoint_host::invalid"]
  n20["tekes-supervisor::endpoint_host::map_endpoint"]
  n21["tekes-supervisor::endpoint_host::map_management"]
  n22["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n2 --> n22
  n3 --> n22
  n4 --> n0
  n8 --> n19
  n8 --> n22
  n9 --> n22
  n11 --> n0
  n11 --> n9
  n12 --> n19
  n13 --> n0
  n13 --> n15
  n13 --> n19
  n13 --> n22
  n14 --> n0
  n14 --> n15
  n14 --> n22
  n15 --> n22
  n17 --> n1
  n17 --> n19
  n17 --> n22
  n18 --> n17
  n19 --> n22
  n20 --> n22
  n21 --> n22
```

</details>

<details><summary>Functions 121–126: 6 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["schema::ijson::IJsonValue::parse_str"]
  n2["tekes-supervisor::endpoint_host::internal_json"]
  n3["tekes-supervisor::endpoint_host::internal_schema"]
  n4["tekes-supervisor::endpoint_host::internal_assembly"]
  n5["tekes-supervisor::endpoint_host::EndpointRpcFailure::new"]
  n6["tekes-supervisor::endpoint_host::system_timestamp"]
  n7["tekes-supervisor::endpoint_host::civil_from_days"]
  n2 --> n5
  n3 --> n5
  n4 --> n5
  n5 --> n0
  n5 --> n1
  n6 --> n7
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `TEKES_UNARY_ROUTES` | `route` | [44](../../src/endpoint_host.rs#L44), [45](../../src/endpoint_host.rs#L45), [46](../../src/endpoint_host.rs#L46), [47](../../src/endpoint_host.rs#L47), [48](../../src/endpoint_host.rs#L48), [49](../../src/endpoint_host.rs#L49), [50](../../src/endpoint_host.rs#L50), [51](../../src/endpoint_host.rs#L51), [52](../../src/endpoint_host.rs#L52), [53](../../src/endpoint_host.rs#L53), [54](../../src/endpoint_host.rs#L54), [55](../../src/endpoint_host.rs#L55), [56](../../src/endpoint_host.rs#L56) | [tekes-supervisor::endpoint_host::route](../../src/endpoint_host.rs#L59) |
| `config_mutation_succeeded` | `Ok` | [111](../../src/endpoint_host.rs#L111) | external-constructor-callback-or-unresolved |
| `compact` | `Err` | [166](../../src/endpoint_host.rs#L166) | external-constructor-callback-or-unresolved |
| `compact` | `ProductionRouteFailure::new` | [166](../../src/endpoint_host.rs#L166) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `compact` | `IJsonValue::parse_str(r#"{"operation":"compact"}"#).expect` | [169](../../src/endpoint_host.rs#L169) | receiver-type-required |
| `compact` | `IJsonValue::parse_str` | [169](../../src/endpoint_host.rs#L169) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `recover_pending_management` | `SessionOperations::open_sessions(root)?         .recover_sessions` | [195](../../src/endpoint_host.rs#L195) | receiver-type-required |
| `recover_pending_management` | `SessionOperations::open_sessions` | [195](../../src/endpoint_host.rs#L195) | [endpoint::session_operations::SessionOperations::open_sessions](../../../endpoint/src/session_operations.rs#L364) |
| `recover_pending_management` | `Some` | [196](../../src/endpoint_host.rs#L196) | external-constructor-callback-or-unresolved |
| `execute` | `serde_json::from_slice(&pending.action.canonical_bytes()?)             .map_err` | [208](../../src/endpoint_host.rs#L208) | receiver-type-required |
| `execute` | `serde_json::from_slice` | [208](../../src/endpoint_host.rs#L208) | external-constructor-callback-or-unresolved |
| `execute` | `pending.action.canonical_bytes` | [208](../../src/endpoint_host.rs#L208) | receiver-type-required |
| `execute` | `pending.rpc_id.clone` | [211](../../src/endpoint_host.rs#L211), [212](../../src/endpoint_host.rs#L212) | receiver-type-required |
| `execute` | `pending.retract_origin.clone` | [214](../../src/endpoint_host.rs#L214) | receiver-type-required |
| `execute` | `transaction.validate().map_err` | [217](../../src/endpoint_host.rs#L217) | receiver-type-required |
| `execute` | `transaction.validate` | [217](../../src/endpoint_host.rs#L217) | receiver-type-required |
| `execute` | `SessionOperationError::QueueRecoveryFailed` | [218](../../src/endpoint_host.rs#L218), [223](../../src/endpoint_host.rs#L223), [230](../../src/endpoint_host.rs#L230), [232](../../src/endpoint_host.rs#L232) | external-constructor-callback-or-unresolved |
| `execute` | `transaction.replacement_origin` | [222](../../src/endpoint_host.rs#L222) | receiver-type-required |
| `execute` | `pending.replacement_origin.as_ref` | [222](../../src/endpoint_host.rs#L222) | receiver-type-required |
| `execute` | `Err` | [223](../../src/endpoint_host.rs#L223) | external-constructor-callback-or-unresolved |
| `execute` | `"prepared replacement origin disagrees with action".to_owned` | [224](../../src/endpoint_host.rs#L224) | receiver-type-required |
| `execute` | `self             .authority             .execute(&pending.session_id, &transaction)             .map_err` | [227](../../src/endpoint_host.rs#L227) | receiver-type-required |
| `execute` | `self             .authority             .execute` | [227](../../src/endpoint_host.rs#L227) | receiver-type-required |
| `execute` | `result.validate_for(&transaction).map_err` | [231](../../src/endpoint_host.rs#L231) | receiver-type-required |
| `execute` | `result.validate_for` | [231](../../src/endpoint_host.rs#L231) | receiver-type-required |
| `execute` | `Ok` | [236](../../src/endpoint_host.rs#L236) | external-constructor-callback-or-unresolved |
| `execute` | `queue_decision` | [236](../../src/endpoint_host.rs#L236) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2474) |
| `gate` | `self.gates.lock().map_err` | [247](../../src/endpoint_host.rs#L247) | receiver-type-required |
| `gate` | `self.gates.lock` | [247](../../src/endpoint_host.rs#L247) | receiver-type-required |
| `gate` | `ProductionRouteFailure::new` | [248](../../src/endpoint_host.rs#L248) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `gate` | `IJsonValue::parse_str("{}").expect` | [251](../../src/endpoint_host.rs#L251) | receiver-type-required |
| `gate` | `IJsonValue::parse_str` | [251](../../src/endpoint_host.rs#L251) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `gate` | `Ok` | [254](../../src/endpoint_host.rs#L254) | external-constructor-callback-or-unresolved |
| `gate` | `gates             .entry(session_id.to_owned())             .or_insert_with(&#124;&#124; Arc::new(Mutex::new(())))             .clone` | [254](../../src/endpoint_host.rs#L254) | receiver-type-required |
| `gate` | `gates             .entry(session_id.to_owned())             .or_insert_with` | [254](../../src/endpoint_host.rs#L254) | receiver-type-required |
| `gate` | `gates             .entry` | [254](../../src/endpoint_host.rs#L254) | receiver-type-required |
| `gate` | `session_id.to_owned` | [255](../../src/endpoint_host.rs#L255) | receiver-type-required |
| `gate` | `Arc::new` | [256](../../src/endpoint_host.rs#L256) | external-constructor-callback-or-unresolved |
| `gate` | `Mutex::new` | [256](../../src/endpoint_host.rs#L256) | external-constructor-callback-or-unresolved |
| `new` | `root.into` | [274](../../src/endpoint_host.rs#L274) | receiver-type-required |
| `new` | `Arc::new` | [275](../../src/endpoint_host.rs#L275) | external-constructor-callback-or-unresolved |
| `new` | `SessionAdmissionGates::default` | [275](../../src/endpoint_host.rs#L275) | external-constructor-callback-or-unresolved |
| `gates` | `Arc::clone` | [281](../../src/endpoint_host.rs#L281) | external-constructor-callback-or-unresolved |
| `with_active_session` | `self.with_session_gate` | [296](../../src/endpoint_host.rs#L296) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::with_session_gate](../../src/endpoint_host.rs#L315) |
| `with_active_session` | `self.root.join("archive").join(session_id).is_dir` | [297](../../src/endpoint_host.rs#L297) | receiver-type-required |
| `with_active_session` | `self.root.join("archive").join` | [297](../../src/endpoint_host.rs#L297) | receiver-type-required |
| `with_active_session` | `self.root.join` | [297](../../src/endpoint_host.rs#L297), [304](../../src/endpoint_host.rs#L304) | receiver-type-required |
| `with_active_session` | `Err` | [298](../../src/endpoint_host.rs#L298), [305](../../src/endpoint_host.rs#L305) | external-constructor-callback-or-unresolved |
| `with_active_session` | `map_failure` | [298](../../src/endpoint_host.rs#L298), [305](../../src/endpoint_host.rs#L305) | external-constructor-callback-or-unresolved |
| `with_active_session` | `ProductionRouteFailure::new` | [298](../../src/endpoint_host.rs#L298), [305](../../src/endpoint_host.rs#L305) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `with_active_session` | `session_details` | [301](../../src/endpoint_host.rs#L301), [308](../../src/endpoint_host.rs#L308) | [tekes-supervisor::endpoint_host::session_details](../../src/endpoint_host.rs#L340) |
| `with_active_session` | `self.root.join("threads").join(session_id).is_dir` | [304](../../src/endpoint_host.rs#L304) | receiver-type-required |
| `with_active_session` | `self.root.join("threads").join` | [304](../../src/endpoint_host.rs#L304) | receiver-type-required |
| `with_active_session` | `operation` | [311](../../src/endpoint_host.rs#L311) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `endpoint::validate_session_id(session_id).is_err` | [321](../../src/endpoint_host.rs#L321) | receiver-type-required |
| `with_session_gate` | `endpoint::validate_session_id` | [321](../../src/endpoint_host.rs#L321) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `with_session_gate` | `Err` | [322](../../src/endpoint_host.rs#L322) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `map_failure` | [322](../../src/endpoint_host.rs#L322), [330](../../src/endpoint_host.rs#L330) | external-constructor-callback-or-unresolved |
| `with_session_gate` | `ProductionRouteFailure::new` | [322](../../src/endpoint_host.rs#L322), [330](../../src/endpoint_host.rs#L330) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `with_session_gate` | `IJsonValue::parse_str("{}").expect` | [325](../../src/endpoint_host.rs#L325), [333](../../src/endpoint_host.rs#L333) | receiver-type-required |
| `with_session_gate` | `IJsonValue::parse_str` | [325](../../src/endpoint_host.rs#L325), [333](../../src/endpoint_host.rs#L333) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `with_session_gate` | `self.gates.gate(session_id).map_err` | [328](../../src/endpoint_host.rs#L328) | receiver-type-required |
| `with_session_gate` | `self.gates.gate` | [328](../../src/endpoint_host.rs#L328) | receiver-type-required |
| `with_session_gate` | `gate.lock().map_err` | [329](../../src/endpoint_host.rs#L329) | receiver-type-required |
| `with_session_gate` | `gate.lock` | [329](../../src/endpoint_host.rs#L329) | receiver-type-required |
| `with_session_gate` | `operation` | [336](../../src/endpoint_host.rs#L336) | external-constructor-callback-or-unresolved |
| `session_details` | `IJsonValue::parse(&serde_json::to_vec(&json!({"sessionId":session_id})).expect("JSON"))         .expect` | [341](../../src/endpoint_host.rs#L341) | receiver-type-required |
| `session_details` | `IJsonValue::parse` | [341](../../src/endpoint_host.rs#L341) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `session_details` | `serde_json::to_vec(&json!({"sessionId":session_id})).expect` | [341](../../src/endpoint_host.rs#L341) | receiver-type-required |
| `session_details` | `serde_json::to_vec` | [341](../../src/endpoint_host.rs#L341) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `Err` | [367](../../src/endpoint_host.rs#L367) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `ProductionRouteFailure::new` | [367](../../src/endpoint_host.rs#L367) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `validate_extension_payload` | `IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))                 .expect` | [370](../../src/endpoint_host.rs#L370) | receiver-type-required |
| `validate_extension_payload` | `IJsonValue::parse` | [370](../../src/endpoint_host.rs#L370) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `validate_extension_payload` | `serde_json::to_vec(&json!({"operation":operation})).expect` | [370](../../src/endpoint_host.rs#L370) | receiver-type-required |
| `validate_extension_payload` | `serde_json::to_vec` | [370](../../src/endpoint_host.rs#L370) | external-constructor-callback-or-unresolved |
| `compose` | `HashMap::new` | [404](../../src/endpoint_host.rs#L404) | external-constructor-callback-or-unresolved |
| `compose` | `TEKES_UNARY_ROUTES             .iter()             .map(&#124;route&#124; route.name)             .collect::<BTreeSet<_>>` | [405](../../src/endpoint_host.rs#L405) | receiver-type-required |
| `compose` | `TEKES_UNARY_ROUTES             .iter()             .map` | [405](../../src/endpoint_host.rs#L405) | receiver-type-required |
| `compose` | `TEKES_UNARY_ROUTES             .iter` | [405](../../src/endpoint_host.rs#L405) | receiver-type-required |
| `compose` | `route.capabilities` | [410](../../src/endpoint_host.rs#L410) | receiver-type-required |
| `compose` | `frozen.contains` | [411](../../src/endpoint_host.rs#L411) | receiver-type-required |
| `compose` | `capability.as_str` | [411](../../src/endpoint_host.rs#L411) | receiver-type-required |
| `compose` | `Err` | [412](../../src/endpoint_host.rs#L412), [420](../../src/endpoint_host.rs#L420) | external-constructor-callback-or-unresolved |
| `compose` | `ProductionRouteCompositionError::FrozenCapability` | [412](../../src/endpoint_host.rs#L412) | external-constructor-callback-or-unresolved |
| `compose` | `owners                     .insert(capability.clone(), Arc::clone(&route))                     .is_some` | [416](../../src/endpoint_host.rs#L416) | receiver-type-required |
| `compose` | `owners                     .insert` | [416](../../src/endpoint_host.rs#L416) | receiver-type-required |
| `compose` | `capability.clone` | [417](../../src/endpoint_host.rs#L417) | receiver-type-required |
| `compose` | `Arc::clone` | [417](../../src/endpoint_host.rs#L417) | external-constructor-callback-or-unresolved |
| `compose` | `ProductionRouteCompositionError::DuplicateCapability` | [420](../../src/endpoint_host.rs#L420) | external-constructor-callback-or-unresolved |
| `compose` | `Ok` | [426](../../src/endpoint_host.rs#L426) | external-constructor-callback-or-unresolved |
| `owner` | `self.owners.get` | [430](../../src/endpoint_host.rs#L430) | receiver-type-required |
| `capabilities` | `self.owners.keys().cloned().collect` | [436](../../src/endpoint_host.rs#L436) | receiver-type-required |
| `capabilities` | `self.owners.keys().cloned` | [436](../../src/endpoint_host.rs#L436) | receiver-type-required |
| `capabilities` | `self.owners.keys` | [436](../../src/endpoint_host.rs#L436) | receiver-type-required |
| `extension_method_class` | `self.owner(method)?.extension_method_class` | [440](../../src/endpoint_host.rs#L440) | receiver-type-required |
| `extension_method_class` | `self.owner` | [440](../../src/endpoint_host.rs#L440) | receiver-type-required |
| `validate_extension_payload` | `self.owner(operation)             .ok_or_else(&#124;&#124; unsupported_route(operation))?             .validate_extension_payload` | [448](../../src/endpoint_host.rs#L448) | receiver-type-required |
| `validate_extension_payload` | `self.owner(operation)             .ok_or_else` | [448](../../src/endpoint_host.rs#L448) | receiver-type-required |
| `validate_extension_payload` | `self.owner` | [448](../../src/endpoint_host.rs#L448) | receiver-type-required |
| `validate_extension_payload` | `unsupported_route` | [449](../../src/endpoint_host.rs#L449) | [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L482) |
| `extension_failure_is_exact` | `self.owner(operation)             .is_some_and` | [458](../../src/endpoint_host.rs#L458) | receiver-type-required |
| `extension_failure_is_exact` | `self.owner` | [458](../../src/endpoint_host.rs#L458) | receiver-type-required |
| `extension_failure_is_exact` | `owner.extension_failure_is_exact` | [459](../../src/endpoint_host.rs#L459) | receiver-type-required |
| `execute` | `self.owner(&request.operation)             .ok_or_else(&#124;&#124; unsupported_route(&request.operation))?             .execute` | [468](../../src/endpoint_host.rs#L468) | receiver-type-required |
| `execute` | `self.owner(&request.operation)             .ok_or_else` | [468](../../src/endpoint_host.rs#L468) | receiver-type-required |
| `execute` | `self.owner` | [468](../../src/endpoint_host.rs#L468) | receiver-type-required |
| `execute` | `unsupported_route` | [469](../../src/endpoint_host.rs#L469) | [tekes-supervisor::endpoint_host::unsupported_route](../../src/endpoint_host.rs#L482) |
| `unsupported_route` | `ProductionRouteFailure::new` | [483](../../src/endpoint_host.rs#L483) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L499) |
| `unsupported_route` | `IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))             .expect` | [486](../../src/endpoint_host.rs#L486) | receiver-type-required |
| `unsupported_route` | `IJsonValue::parse` | [486](../../src/endpoint_host.rs#L486) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `unsupported_route` | `serde_json::to_vec(&json!({"operation":operation})).expect` | [486](../../src/endpoint_host.rs#L486) | receiver-type-required |
| `unsupported_route` | `serde_json::to_vec` | [486](../../src/endpoint_host.rs#L486) | external-constructor-callback-or-unresolved |
| `new` | `code.into` | [501](../../src/endpoint_host.rs#L501) | receiver-type-required |
| `new` | `message.into` | [502](../../src/endpoint_host.rs#L502) | receiver-type-required |
| `with_user_agent_dir` | `user_agent_dir.to_path_buf` | [532](../../src/endpoint_host.rs#L532) | receiver-type-required |
| `mux_description` | `endpoint::SessionEndpointCapability::required` | [537](../../src/endpoint_host.rs#L537) | [endpoint::mux::SessionEndpointCapability::required](../../../endpoint/src/mux.rs#L34) |
| `mux_description` | `capabilities.extend` | [538](../../src/endpoint_host.rs#L538) | receiver-type-required |
| `mux_description` | `self.provider_readiness.is_some` | [542](../../src/endpoint_host.rs#L542) | receiver-type-required |
| `mux_description` | `capabilities.insert` | [543](../../src/endpoint_host.rs#L543) | receiver-type-required |
| `mux_description` | `"TekesKernel".to_owned` | [548](../../src/endpoint_host.rs#L548) | receiver-type-required |
| `mux_description` | `self.description.version.clone` | [549](../../src/endpoint_host.rs#L549) | receiver-type-required |
| `mux_description` | `self.description.cwd.clone` | [552](../../src/endpoint_host.rs#L552) | receiver-type-required |
| `mux_description` | `self.description.provider.clone` | [553](../../src/endpoint_host.rs#L553) | receiver-type-required |
| `mux_description` | `self.description.model.clone` | [554](../../src/endpoint_host.rs#L554) | receiver-type-required |
| `mux_description` | `self.description.home.clone` | [556](../../src/endpoint_host.rs#L556) | receiver-type-required |
| `open` | `Self::open_with_full_authorities` | [565](../../src/endpoint_host.rs#L565) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) |
| `open` | `Arc::new` | [568](../../src/endpoint_host.rs#L568) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [581](../../src/endpoint_host.rs#L581) | receiver-type-required |
| `open_with_clock` | `Self::open_with_full_authorities` | [582](../../src/endpoint_host.rs#L582) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) |
| `open_with_routes` | `Self::open_with_full_authorities` | [591](../../src/endpoint_host.rs#L591) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) |
| `open_with_authorities` | `Self::open_with_full_authorities` | [601](../../src/endpoint_host.rs#L601) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) |
| `open_with_full_authorities` | `Self::open_with_route_ownership` | [621](../../src/endpoint_host.rs#L621) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L684) |
| `open_with_full_authorities_and_session_admission` | `Self::open_with_route_ownership` | [645](../../src/endpoint_host.rs#L645) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L684) |
| `open_with_full_authorities_and_session_admission` | `Some` | [654](../../src/endpoint_host.rs#L654) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `Self::open_with_route_ownership` | [670](../../src/endpoint_host.rs#L670) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_ownership](../../src/endpoint_host.rs#L684) |
| `open_with_route_authority` | `Some` | [674](../../src/endpoint_host.rs#L674) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.as_ref` | [695](../../src/endpoint_host.rs#L695) | receiver-type-required |
| `open_with_route_ownership` | `routes.as_ref` | [696](../../src/endpoint_host.rs#L696) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map(&#124;route&#124; route.name)                 .collect::<BTreeSet<_>>` | [697](../../src/endpoint_host.rs#L697) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter()                 .map` | [697](../../src/endpoint_host.rs#L697) | receiver-type-required |
| `open_with_route_ownership` | `TEKES_UNARY_ROUTES                 .iter` | [697](../../src/endpoint_host.rs#L697) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter().any` | [701](../../src/endpoint_host.rs#L701) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities().iter` | [701](../../src/endpoint_host.rs#L701) | receiver-type-required |
| `open_with_route_ownership` | `extension.capabilities` | [701](../../src/endpoint_host.rs#L701) | receiver-type-required |
| `open_with_route_ownership` | `frozen.contains` | [703](../../src/endpoint_host.rs#L703), [705](../../src/endpoint_host.rs#L705) | receiver-type-required |
| `open_with_route_ownership` | `method.as_str` | [703](../../src/endpoint_host.rs#L703), [705](../../src/endpoint_host.rs#L705) | receiver-type-required |
| `open_with_route_ownership` | `Err` | [712](../../src/endpoint_host.rs#L712) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `PathBuf::from(&description.home).join` | [715](../../src/endpoint_host.rs#L715) | receiver-type-required |
| `open_with_route_ownership` | `PathBuf::from` | [715](../../src/endpoint_host.rs#L715) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `queue_authority             .as_deref()             .map` | [716](../../src/endpoint_host.rs#L716) | receiver-type-required |
| `open_with_route_ownership` | `queue_authority             .as_deref` | [716](../../src/endpoint_host.rs#L716) | receiver-type-required |
| `open_with_route_ownership` | `SessionOperations::open_sessions` | [719](../../src/endpoint_host.rs#L719) | [endpoint::session_operations::SessionOperations::open_sessions](../../../endpoint/src/session_operations.rs#L364) |
| `open_with_route_ownership` | `management.recover_sessions` | [720](../../src/endpoint_host.rs#L720) | receiver-type-required |
| `open_with_route_ownership` | `queue_recovery                 .as_ref()                 .map` | [721](../../src/endpoint_host.rs#L721) | receiver-type-required |
| `open_with_route_ownership` | `queue_recovery                 .as_ref` | [721](../../src/endpoint_host.rs#L721) | receiver-type-required |
| `open_with_route_ownership` | `Ok` | [728](../../src/endpoint_host.rs#L728) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `root.to_path_buf` | [729](../../src/endpoint_host.rs#L729), [745](../../src/endpoint_host.rs#L745) | receiver-type-required |
| `open_with_route_ownership` | `NativeEndpoint::open` | [730](../../src/endpoint_host.rs#L730) | [endpoint::service::NativeEndpoint::open](../../../endpoint/src/service.rs#L263) |
| `open_with_route_ownership` | `AttachmentAuthority::open` | [731](../../src/endpoint_host.rs#L731) | [endpoint::attachment::AttachmentAuthority::open](../../../endpoint/src/attachment.rs#L242) |
| `open_with_route_ownership` | `input_admission.unwrap_or_else` | [744](../../src/endpoint_host.rs#L744) | receiver-type-required |
| `open_with_route_ownership` | `Arc::new` | [745](../../src/endpoint_host.rs#L745) | external-constructor-callback-or-unresolved |
| `open_with_route_ownership` | `SessionInputAdmissionAuthority::new` | [745](../../src/endpoint_host.rs#L745) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L272) |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map(&#124;route&#124; route.name.to_owned())             .collect` | [752](../../src/endpoint_host.rs#L752) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter(&#124;route&#124; route.implemented)             .map` | [752](../../src/endpoint_host.rs#L752) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter()             .filter` | [752](../../src/endpoint_host.rs#L752) | receiver-type-required |
| `implemented_capabilities` | `TEKES_UNARY_ROUTES             .iter` | [752](../../src/endpoint_host.rs#L752) | receiver-type-required |
| `implemented_capabilities` | `route.name.to_owned` | [755](../../src/endpoint_host.rs#L755) | receiver-type-required |
| `admission_gates` | `self.input_admission.gates` | [761](../../src/endpoint_host.rs#L761) | receiver-type-required |
| `session_admission_authority` | `Arc::clone` | [766](../../src/endpoint_host.rs#L766) | external-constructor-callback-or-unresolved |
| `has_incomplete_management_operation` | `Ok` | [773](../../src/endpoint_host.rs#L773) | external-constructor-callback-or-unresolved |
| `has_incomplete_management_operation` | `self             .management             .has_incomplete_session_operation` | [773](../../src/endpoint_host.rs#L773) | receiver-type-required |
| `execute` | `self.execute_inner` | [779](../../src/endpoint_host.rs#L779) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_inner](../../src/endpoint_host.rs#L786) |
| `execute` | `success` | [780](../../src/endpoint_host.rs#L780) | [tekes-supervisor::endpoint_host::success](../../src/endpoint_host.rs#L1771) |
| `execute` | `failure` | [783](../../src/endpoint_host.rs#L783) | [tekes-supervisor::endpoint_host::failure](../../src/endpoint_host.rs#L2226) |
| `execute_inner` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)                 .map_err` | [788](../../src/endpoint_host.rs#L788) | receiver-type-required |
| `execute_inner` | `serde_json::from_slice` | [788](../../src/endpoint_host.rs#L788) | external-constructor-callback-or-unresolved |
| `execute_inner` | `request.payload.canonical_bytes().map_err` | [788](../../src/endpoint_host.rs#L788) | receiver-type-required |
| `execute_inner` | `request.payload.canonical_bytes` | [788](../../src/endpoint_host.rs#L788) | receiver-type-required |
| `execute_inner` | `request.operation.as_str` | [790](../../src/endpoint_host.rs#L790) | receiver-type-required |
| `execute_inner` | `self.archive_session` | [791](../../src/endpoint_host.rs#L791) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::archive_session](../../src/endpoint_host.rs#L863) |
| `execute_inner` | `self.unarchive_session` | [792](../../src/endpoint_host.rs#L792) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::unarchive_session](../../src/endpoint_host.rs#L886) |
| `execute_inner` | `self.create_session` | [793](../../src/endpoint_host.rs#L793) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../src/endpoint_host.rs#L903) |
| `execute_inner` | `self.provider_readiness.is_some` | [794](../../src/endpoint_host.rs#L794), [795](../../src/endpoint_host.rs#L795), [796](../../src/endpoint_host.rs#L796) | receiver-type-required |
| `execute_inner` | `self.models` | [794](../../src/endpoint_host.rs#L794) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::models](../../src/endpoint_host.rs#L1002) |
| `execute_inner` | `self.draft_models` | [795](../../src/endpoint_host.rs#L795) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::draft_models](../../src/endpoint_host.rs#L965) |
| `execute_inner` | `self.select_model` | [797](../../src/endpoint_host.rs#L797) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::select_model](../../src/endpoint_host.rs#L1022) |
| `execute_inner` | `self.delivery_authority.is_some` | [799](../../src/endpoint_host.rs#L799), [800](../../src/endpoint_host.rs#L800), [806](../../src/endpoint_host.rs#L806) | receiver-type-required |
| `execute_inner` | `self.cancel` | [799](../../src/endpoint_host.rs#L799) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::cancel](../../src/endpoint_host.rs#L1081) |
| `execute_inner` | `self.rename_session` | [801](../../src/endpoint_host.rs#L801) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::rename_session](../../src/endpoint_host.rs#L1111) |
| `execute_inner` | `self.queue_authority.is_some` | [803](../../src/endpoint_host.rs#L803) | receiver-type-required |
| `execute_inner` | `self.update_queue` | [804](../../src/endpoint_host.rs#L804) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::update_queue](../../src/endpoint_host.rs#L1149) |
| `execute_inner` | `self.prompt` | [806](../../src/endpoint_host.rs#L806) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::prompt](../../src/endpoint_host.rs#L1238) |
| `execute_inner` | `self.attachment` | [807](../../src/endpoint_host.rs#L807) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::attachment](../../src/endpoint_host.rs#L1295) |
| `execute_inner` | `self.fork_session` | [808](../../src/endpoint_host.rs#L808) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::fork_session](../../src/endpoint_host.rs#L1311) |
| `execute_inner` | `self.discard_session` | [809](../../src/endpoint_host.rs#L809) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::discard_session](../../src/endpoint_host.rs#L1348) |
| `execute_inner` | `self.execute_extension` | [810](../../src/endpoint_host.rs#L810) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::execute_extension](../../src/endpoint_host.rs#L814) |
| `execute_extension` | `self.routes.as_ref` | [820](../../src/endpoint_host.rs#L820) | receiver-type-required |
| `execute_extension` | `Err` | [821](../../src/endpoint_host.rs#L821), [828](../../src/endpoint_host.rs#L828) | external-constructor-callback-or-unresolved |
| `execute_extension` | `EndpointRpcFailure::new` | [821](../../src/endpoint_host.rs#L821), [828](../../src/endpoint_host.rs#L828) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `execute_extension` | `routes.capabilities().contains` | [827](../../src/endpoint_host.rs#L827) | receiver-type-required |
| `execute_extension` | `routes.capabilities` | [827](../../src/endpoint_host.rs#L827) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter()             .any` | [834](../../src/endpoint_host.rs#L834) | receiver-type-required |
| `execute_extension` | `TEKES_UNARY_ROUTES             .iter` | [834](../../src/endpoint_host.rs#L834) | receiver-type-required |
| `execute_extension` | `validate_extension_payload` | [838](../../src/endpoint_host.rs#L838) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2599) |
| `execute_extension` | `routes                 .validate_extension_payload(operation, payload)                 .map_err` | [840](../../src/endpoint_host.rs#L840) | receiver-type-required |
| `execute_extension` | `routes                 .validate_extension_payload` | [840](../../src/endpoint_host.rs#L840) | receiver-type-required |
| `execute_extension` | `routes.extension_failure_is_exact` | [843](../../src/endpoint_host.rs#L843), [855](../../src/endpoint_host.rs#L855) | receiver-type-required |
| `execute_extension` | `map_production_route` | [844](../../src/endpoint_host.rs#L844), [856](../../src/endpoint_host.rs#L856) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1928) |
| `execute_extension` | `internal_failure` | [846](../../src/endpoint_host.rs#L846), [858](../../src/endpoint_host.rs#L858) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `execute_extension` | `routes             .execute(request, payload, &self.principal)             .map_err` | [850](../../src/endpoint_host.rs#L850) | receiver-type-required |
| `execute_extension` | `routes             .execute` | [850](../../src/endpoint_host.rs#L850) | receiver-type-required |
| `execute_extension` | `map_production_route_for` | [854](../../src/endpoint_host.rs#L854) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `archive_session` | `require_object_fields` | [868](../../src/endpoint_host.rs#L868) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `archive_session` | `required_string` | [869](../../src/endpoint_host.rs#L869) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `archive_session` | `self.input_admission.with_session_gate` | [870](../../src/endpoint_host.rs#L870) | receiver-type-required |
| `archive_session` | `map_production_route_for` | [872](../../src/endpoint_host.rs#L872) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `archive_session` | `(self.clock)().map_err` | [874](../../src/endpoint_host.rs#L874) | receiver-type-required |
| `archive_session` | `(self.clock)` | [874](../../src/endpoint_host.rs#L874) | external-constructor-callback-or-unresolved |
| `archive_session` | `request_hash` | [875](../../src/endpoint_host.rs#L875) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `archive_session` | `self                     .management                     .archive_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `archive_session` | `self                     .management                     .archive_session` | [876](../../src/endpoint_host.rs#L876) | receiver-type-required |
| `archive_session` | `mark_management_handoff` | [880](../../src/endpoint_host.rs#L880) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `archive_session` | `to_ijson` | [881](../../src/endpoint_host.rs#L881) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `unarchive_session` | `require_object_fields` | [891](../../src/endpoint_host.rs#L891) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `unarchive_session` | `required_string` | [892](../../src/endpoint_host.rs#L892) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `unarchive_session` | `(self.clock)().map_err` | [893](../../src/endpoint_host.rs#L893) | receiver-type-required |
| `unarchive_session` | `(self.clock)` | [893](../../src/endpoint_host.rs#L893) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `request_hash` | [894](../../src/endpoint_host.rs#L894) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `unarchive_session` | `self             .management             .unarchive_session(&request.rpc_id, &hash, session_id, &timestamp)             .map_err` | [895](../../src/endpoint_host.rs#L895) | receiver-type-required |
| `unarchive_session` | `self             .management             .unarchive_session` | [895](../../src/endpoint_host.rs#L895) | receiver-type-required |
| `unarchive_session` | `mark_management_handoff` | [899](../../src/endpoint_host.rs#L899) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `unarchive_session` | `to_ijson` | [900](../../src/endpoint_host.rs#L900) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `create_session` | `require_object_fields` | [908](../../src/endpoint_host.rs#L908) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `create_session` | `payload.get("agentPreset").is_some` | [919](../../src/endpoint_host.rs#L919) | receiver-type-required |
| `create_session` | `payload.get` | [919](../../src/endpoint_host.rs#L919), [938](../../src/endpoint_host.rs#L938) | receiver-type-required |
| `create_session` | `Err` | [920](../../src/endpoint_host.rs#L920), [940](../../src/endpoint_host.rs#L940) | external-constructor-callback-or-unresolved |
| `create_session` | `EndpointRpcFailure::new` | [920](../../src/endpoint_host.rs#L920) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `create_session` | `serde_json::from_value(payload["execution"].clone())                 .map_err` | [927](../../src/endpoint_host.rs#L927) | receiver-type-required |
| `create_session` | `serde_json::from_value` | [927](../../src/endpoint_host.rs#L927) | external-constructor-callback-or-unresolved |
| `create_session` | `payload["execution"].clone` | [927](../../src/endpoint_host.rs#L927) | receiver-type-required |
| `create_session` | `invalid` | [928](../../src/endpoint_host.rs#L928), [934](../../src/endpoint_host.rs#L934), [940](../../src/endpoint_host.rs#L940) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `create_session` | `execution.validate().map_err` | [929](../../src/endpoint_host.rs#L929) | receiver-type-required |
| `create_session` | `execution.validate` | [929](../../src/endpoint_host.rs#L929) | receiver-type-required |
| `create_session` | `optional_string` | [930](../../src/endpoint_host.rs#L930), [931](../../src/endpoint_host.rs#L931) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2268) |
| `create_session` | `optional_string(payload, "identityProfile")?             .map(&#124;value&#124; {                 tools::IdentityProfile::parse(value)                     .ok_or_else(&#124;&#124; invalid("identityProfile must be coding or general"))             })             .transpose` | [931](../../src/endpoint_host.rs#L931) | receiver-type-required |
| `create_session` | `optional_string(payload, "identityProfile")?             .map` | [931](../../src/endpoint_host.rs#L931) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse(value)                     .ok_or_else` | [933](../../src/endpoint_host.rs#L933) | receiver-type-required |
| `create_session` | `tools::IdentityProfile::parse` | [933](../../src/endpoint_host.rs#L933) | [tools::guidance::IdentityProfile::parse](../../../tools/src/guidance.rs#L54) |
| `create_session` | `preset.is_some` | [939](../../src/endpoint_host.rs#L939) | receiver-type-required |
| `create_session` | `identity_profile.is_some` | [939](../../src/endpoint_host.rs#L939) | receiver-type-required |
| `create_session` | `(self.clock)().map_err` | [942](../../src/endpoint_host.rs#L942) | receiver-type-required |
| `create_session` | `(self.clock)` | [942](../../src/endpoint_host.rs#L942) | external-constructor-callback-or-unresolved |
| `create_session` | `request_hash` | [943](../../src/endpoint_host.rs#L943) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `create_session` | `self             .management             .create_execution_session(                 ExecutionCreateOperation {                     rpc_id: &request.rpc_id,                     request_sha256: &hash,                     requested_session_id: session_id,                     identity_profile,                     preset,                     preset_models: Some(&ConfiguredModels),                     user_agent_dir: &self.user_agent_dir,                     started_at: &timestamp,                     principal: &self.principal,                 },                 &execution,             )             .map_err` | [944](../../src/endpoint_host.rs#L944) | receiver-type-required |
| `create_session` | `self             .management             .create_execution_session` | [944](../../src/endpoint_host.rs#L944) | receiver-type-required |
| `create_session` | `Some` | [953](../../src/endpoint_host.rs#L953) | external-constructor-callback-or-unresolved |
| `create_session` | `mark_management_handoff` | [961](../../src/endpoint_host.rs#L961) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `create_session` | `to_ijson` | [962](../../src/endpoint_host.rs#L962) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `draft_models` | `require_object_fields` | [966](../../src/endpoint_host.rs#L966) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `draft_models` | `profile::ConfigRepository::open(&self.storage_root).map_err` | [968](../../src/endpoint_host.rs#L968) | receiver-type-required |
| `draft_models` | `profile::ConfigRepository::open` | [968](../../src/endpoint_host.rs#L968) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L568) |
| `draft_models` | `internal_failure` | [968](../../src/endpoint_host.rs#L968), [969](../../src/endpoint_host.rs#L969), [970](../../src/endpoint_host.rs#L970) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `draft_models` | `repository.providers().map_err` | [969](../../src/endpoint_host.rs#L969) | receiver-type-required |
| `draft_models` | `repository.providers` | [969](../../src/endpoint_host.rs#L969) | receiver-type-required |
| `draft_models` | `repository.settings().map_err` | [970](../../src/endpoint_host.rs#L970) | receiver-type-required |
| `draft_models` | `repository.settings` | [970](../../src/endpoint_host.rs#L970) | receiver-type-required |
| `draft_models` | `String::new` | [983](../../src/endpoint_host.rs#L983) | external-constructor-callback-or-unresolved |
| `draft_models` | `Default::default` | [986](../../src/endpoint_host.rs#L986) | external-constructor-callback-or-unresolved |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness("", &config)             .map_err` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else(internal_failure)?             .readiness` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref()             .ok_or_else` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `draft_models` | `self             .provider_readiness             .as_ref` | [993](../../src/endpoint_host.rs#L993) | receiver-type-required |
| `draft_models` | `map_production_route_for` | [998](../../src/endpoint_host.rs#L998) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `draft_models` | `model_projection` | [999](../../src/endpoint_host.rs#L999) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1779) |
| `models` | `require_object_fields` | [1003](../../src/endpoint_host.rs#L1003) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `models` | `required_string` | [1004](../../src/endpoint_host.rs#L1004) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `models` | `self.provider_readiness.as_ref().ok_or_else` | [1005](../../src/endpoint_host.rs#L1005) | receiver-type-required |
| `models` | `self.provider_readiness.as_ref` | [1005](../../src/endpoint_host.rs#L1005) | receiver-type-required |
| `models` | `EndpointRpcFailure::new` | [1006](../../src/endpoint_host.rs#L1006) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `models` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1012](../../src/endpoint_host.rs#L1012) | receiver-type-required |
| `models` | `self             .endpoint             .session_config_snapshot` | [1012](../../src/endpoint_host.rs#L1012) | receiver-type-required |
| `models` | `map_endpoint` | [1015](../../src/endpoint_host.rs#L1015) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2663) |
| `models` | `Some` | [1015](../../src/endpoint_host.rs#L1015) | external-constructor-callback-or-unresolved |
| `models` | `authority             .readiness(session_id, &config)             .map_err` | [1016](../../src/endpoint_host.rs#L1016) | receiver-type-required |
| `models` | `authority             .readiness` | [1016](../../src/endpoint_host.rs#L1016) | receiver-type-required |
| `models` | `map_production_route_for` | [1018](../../src/endpoint_host.rs#L1018) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `models` | `model_projection` | [1019](../../src/endpoint_host.rs#L1019) | [tekes-supervisor::endpoint_host::model_projection](../../src/endpoint_host.rs#L1779) |
| `select_model` | `require_object_fields` | [1027](../../src/endpoint_host.rs#L1027) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `select_model` | `required_string` | [1032](../../src/endpoint_host.rs#L1032), [1033](../../src/endpoint_host.rs#L1033), [1034](../../src/endpoint_host.rs#L1034) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `select_model` | `optional_string` | [1035](../../src/endpoint_host.rs#L1035) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2268) |
| `select_model` | `self.provider_readiness.as_ref().ok_or_else` | [1036](../../src/endpoint_host.rs#L1036) | receiver-type-required |
| `select_model` | `self.provider_readiness.as_ref` | [1036](../../src/endpoint_host.rs#L1036) | receiver-type-required |
| `select_model` | `EndpointRpcFailure::new` | [1037](../../src/endpoint_host.rs#L1037), [1050](../../src/endpoint_host.rs#L1050) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `select_model` | `self.admission_gate` | [1043](../../src/endpoint_host.rs#L1043) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1371) |
| `select_model` | `gate.lock().map_err` | [1044](../../src/endpoint_host.rs#L1044) | receiver-type-required |
| `select_model` | `gate.lock` | [1044](../../src/endpoint_host.rs#L1044) | receiver-type-required |
| `select_model` | `internal_failure` | [1044](../../src/endpoint_host.rs#L1044) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `select_model` | `self             .endpoint             .session_config_snapshot(session_id)             .map_err` | [1045](../../src/endpoint_host.rs#L1045) | receiver-type-required |
| `select_model` | `self             .endpoint             .session_config_snapshot` | [1045](../../src/endpoint_host.rs#L1045) | receiver-type-required |
| `select_model` | `map_endpoint` | [1048](../../src/endpoint_host.rs#L1048) | [tekes-supervisor::endpoint_host::map_endpoint](../../src/endpoint_host.rs#L2663) |
| `select_model` | `Some` | [1048](../../src/endpoint_host.rs#L1048) | external-constructor-callback-or-unresolved |
| `select_model` | `model_is_configured` | [1049](../../src/endpoint_host.rs#L1049) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1888) |
| `select_model` | `Err` | [1050](../../src/endpoint_host.rs#L1050) | external-constructor-callback-or-unresolved |
| `select_model` | `(self.clock)().map_err` | [1056](../../src/endpoint_host.rs#L1056) | receiver-type-required |
| `select_model` | `(self.clock)` | [1056](../../src/endpoint_host.rs#L1056) | external-constructor-callback-or-unresolved |
| `select_model` | `request_hash` | [1057](../../src/endpoint_host.rs#L1057) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `select_model` | `self             .management             .select_model(SelectModelOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 provider,                 model,                 reasoning_effort: effort,                 started_at: &timestamp,             })             .map_err` | [1058](../../src/endpoint_host.rs#L1058) | receiver-type-required |
| `select_model` | `self             .management             .select_model` | [1058](../../src/endpoint_host.rs#L1058) | receiver-type-required |
| `select_model` | `mark_management_handoff` | [1070](../../src/endpoint_host.rs#L1070) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `select_model` | `authority             .config_mutation_succeeded(session_id)             .map_err` | [1071](../../src/endpoint_host.rs#L1071) | receiver-type-required |
| `select_model` | `authority             .config_mutation_succeeded` | [1071](../../src/endpoint_host.rs#L1071) | receiver-type-required |
| `select_model` | `map_production_route_for` | [1073](../../src/endpoint_host.rs#L1073) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `select_model` | `Value::String` | [1076](../../src/endpoint_host.rs#L1076) | external-constructor-callback-or-unresolved |
| `select_model` | `to_ijson` | [1078](../../src/endpoint_host.rs#L1078) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `cancel` | `require_object_fields` | [1086](../../src/endpoint_host.rs#L1086) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `cancel` | `required_string` | [1087](../../src/endpoint_host.rs#L1087) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `cancel` | `self.delivery_authority.as_ref().ok_or_else` | [1088](../../src/endpoint_host.rs#L1088) | receiver-type-required |
| `cancel` | `self.delivery_authority.as_ref` | [1088](../../src/endpoint_host.rs#L1088) | receiver-type-required |
| `cancel` | `EndpointRpcFailure::new` | [1089](../../src/endpoint_host.rs#L1089) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `cancel` | `self.admission_gate` | [1095](../../src/endpoint_host.rs#L1095) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1371) |
| `cancel` | `gate.lock().map_err` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `cancel` | `gate.lock` | [1096](../../src/endpoint_host.rs#L1096) | receiver-type-required |
| `cancel` | `internal_failure` | [1096](../../src/endpoint_host.rs#L1096) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `cancel` | `(self.clock)().map_err` | [1097](../../src/endpoint_host.rs#L1097) | receiver-type-required |
| `cancel` | `(self.clock)` | [1097](../../src/endpoint_host.rs#L1097) | external-constructor-callback-or-unresolved |
| `cancel` | `endpoint_origin` | [1098](../../src/endpoint_host.rs#L1098) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) |
| `cancel` | `authority             .cancel(session_id, &timestamp, &origin)             .map_err` | [1104](../../src/endpoint_host.rs#L1104) | receiver-type-required |
| `cancel` | `authority             .cancel` | [1104](../../src/endpoint_host.rs#L1104) | receiver-type-required |
| `cancel` | `map_production_route_for` | [1106](../../src/endpoint_host.rs#L1106) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `cancel` | `mark_event_handoff` | [1107](../../src/endpoint_host.rs#L1107) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2337) |
| `cancel` | `to_ijson` | [1108](../../src/endpoint_host.rs#L1108) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `rename_session` | `require_object_fields` | [1116](../../src/endpoint_host.rs#L1116) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `rename_session` | `required_string` | [1117](../../src/endpoint_host.rs#L1117) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `rename_session` | `required_string_allow_whitespace` | [1118](../../src/endpoint_host.rs#L1118) | [tekes-supervisor::endpoint_host::required_string_allow_whitespace](../../src/endpoint_host.rs#L2247) |
| `rename_session` | `validate_session_title(title).map_err` | [1119](../../src/endpoint_host.rs#L1119) | receiver-type-required |
| `rename_session` | `validate_session_title` | [1119](../../src/endpoint_host.rs#L1119) | [tekes-supervisor::endpoint_host::validate_session_title](../../src/endpoint_host.rs#L2366) |
| `rename_session` | `EndpointRpcFailure::new` | [1120](../../src/endpoint_host.rs#L1120), [1127](../../src/endpoint_host.rs#L1127) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `rename_session` | `self.delivery_authority.as_ref().ok_or_else` | [1126](../../src/endpoint_host.rs#L1126) | receiver-type-required |
| `rename_session` | `self.delivery_authority.as_ref` | [1126](../../src/endpoint_host.rs#L1126) | receiver-type-required |
| `rename_session` | `self.admission_gate` | [1133](../../src/endpoint_host.rs#L1133) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1371) |
| `rename_session` | `gate.lock().map_err` | [1134](../../src/endpoint_host.rs#L1134) | receiver-type-required |
| `rename_session` | `gate.lock` | [1134](../../src/endpoint_host.rs#L1134) | receiver-type-required |
| `rename_session` | `internal_failure` | [1134](../../src/endpoint_host.rs#L1134) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `rename_session` | `(self.clock)().map_err` | [1135](../../src/endpoint_host.rs#L1135) | receiver-type-required |
| `rename_session` | `(self.clock)` | [1135](../../src/endpoint_host.rs#L1135) | external-constructor-callback-or-unresolved |
| `rename_session` | `endpoint_origin` | [1136](../../src/endpoint_host.rs#L1136) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) |
| `rename_session` | `authority             .rename(session_id, &timestamp, &origin, title)             .map_err` | [1142](../../src/endpoint_host.rs#L1142) | receiver-type-required |
| `rename_session` | `authority             .rename` | [1142](../../src/endpoint_host.rs#L1142) | receiver-type-required |
| `rename_session` | `map_production_route_for` | [1144](../../src/endpoint_host.rs#L1144) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `rename_session` | `mark_event_handoff` | [1145](../../src/endpoint_host.rs#L1145) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2337) |
| `rename_session` | `to_ijson` | [1146](../../src/endpoint_host.rs#L1146) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `update_queue` | `require_object_fields` | [1154](../../src/endpoint_host.rs#L1154) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `update_queue` | `required_string` | [1159](../../src/endpoint_host.rs#L1159), [1160](../../src/endpoint_host.rs#L1160) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter(&#124;value&#124; *value > 0)             .ok_or_else` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then(&#124;value&#124; value.parse::<u64>().ok())             .filter` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix("input:")             .and_then` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `update_queue` | `item_id             .strip_prefix` | [1161](../../src/endpoint_host.rs#L1161) | receiver-type-required |
| `update_queue` | `value.parse::<u64>().ok` | [1163](../../src/endpoint_host.rs#L1163) | receiver-type-required |
| `update_queue` | `value.parse::<u64>` | [1163](../../src/endpoint_host.rs#L1163) | receiver-type-required |
| `update_queue` | `invalid` | [1165](../../src/endpoint_host.rs#L1165), [1166](../../src/endpoint_host.rs#L1166), [1195](../../src/endpoint_host.rs#L1195) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `update_queue` | `queue_action` | [1166](../../src/endpoint_host.rs#L1166) | [tekes-supervisor::endpoint_host::queue_action](../../src/endpoint_host.rs#L2400) |
| `update_queue` | `payload.get("action").ok_or_else` | [1166](../../src/endpoint_host.rs#L1166) | receiver-type-required |
| `update_queue` | `payload.get` | [1166](../../src/endpoint_host.rs#L1166) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref().ok_or_else` | [1167](../../src/endpoint_host.rs#L1167) | receiver-type-required |
| `update_queue` | `self.queue_authority.as_ref` | [1167](../../src/endpoint_host.rs#L1167) | receiver-type-required |
| `update_queue` | `EndpointRpcFailure::new` | [1168](../../src/endpoint_host.rs#L1168) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `update_queue` | `self.admission_gate` | [1174](../../src/endpoint_host.rs#L1174) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::admission_gate](../../src/endpoint_host.rs#L1371) |
| `update_queue` | `gate.lock().map_err` | [1175](../../src/endpoint_host.rs#L1175) | receiver-type-required |
| `update_queue` | `gate.lock` | [1175](../../src/endpoint_host.rs#L1175) | receiver-type-required |
| `update_queue` | `internal_failure` | [1175](../../src/endpoint_host.rs#L1175), [1221](../../src/endpoint_host.rs#L1221) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `update_queue` | `request.rpc_id.clone` | [1177](../../src/endpoint_host.rs#L1177), [1178](../../src/endpoint_host.rs#L1178) | receiver-type-required |
| `update_queue` | `endpoint_origin` | [1180](../../src/endpoint_host.rs#L1180), [1186](../../src/endpoint_host.rs#L1186) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) |
| `update_queue` | `action.with_origin` | [1186](../../src/endpoint_host.rs#L1186) | receiver-type-required |
| `update_queue` | `transaction             .validate()             .map_err` | [1193](../../src/endpoint_host.rs#L1193) | receiver-type-required |
| `update_queue` | `transaction             .validate` | [1193](../../src/endpoint_host.rs#L1193) | receiver-type-required |
| `update_queue` | `to_ijson` | [1196](../../src/endpoint_host.rs#L1196), [1230](../../src/endpoint_host.rs#L1230) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `update_queue` | `request_hash` | [1197](../../src/endpoint_host.rs#L1197) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `update_queue` | `(self.clock)().map_err` | [1198](../../src/endpoint_host.rs#L1198) | receiver-type-required |
| `update_queue` | `(self.clock)` | [1198](../../src/endpoint_host.rs#L1198) | external-constructor-callback-or-unresolved |
| `update_queue` | `self             .management             .prepare_queue_transaction(QueueTransactionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 session_id,                 target_seq,                 action: &action,                 retract_origin: &transaction.retract_origin,                 replacement_origin: transaction.replacement_origin(),                 asset_digests: &[],                 started_at: &timestamp,             })             .map_err` | [1199](../../src/endpoint_host.rs#L1199) | receiver-type-required |
| `update_queue` | `self             .management             .prepare_queue_transaction` | [1199](../../src/endpoint_host.rs#L1199) | receiver-type-required |
| `update_queue` | `transaction.replacement_origin` | [1208](../../src/endpoint_host.rs#L1208) | receiver-type-required |
| `update_queue` | `authority                     .execute(session_id, &transaction)                     .map_err` | [1216](../../src/endpoint_host.rs#L1216) | receiver-type-required |
| `update_queue` | `authority                     .execute` | [1216](../../src/endpoint_host.rs#L1216) | receiver-type-required |
| `update_queue` | `map_production_route_for` | [1218](../../src/endpoint_host.rs#L1218) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `update_queue` | `result                     .validate_for(&transaction)                     .map_err` | [1219](../../src/endpoint_host.rs#L1219) | receiver-type-required |
| `update_queue` | `result                     .validate_for` | [1219](../../src/endpoint_host.rs#L1219) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction(&request.rpc_id, queue_decision(&result.outcome))                     .map_err` | [1222](../../src/endpoint_host.rs#L1222) | receiver-type-required |
| `update_queue` | `self.management                     .complete_queue_transaction` | [1222](../../src/endpoint_host.rs#L1222) | receiver-type-required |
| `update_queue` | `queue_decision` | [1223](../../src/endpoint_host.rs#L1223) | [tekes-supervisor::endpoint_host::queue_decision](../../src/endpoint_host.rs#L2474) |
| `update_queue` | `mark_management_handoff` | [1229](../../src/endpoint_host.rs#L1229) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `update_queue` | `Err` | [1233](../../src/endpoint_host.rs#L1233) | external-constructor-callback-or-unresolved |
| `update_queue` | `map_persisted_queue_rejection` | [1233](../../src/endpoint_host.rs#L1233) | [tekes-supervisor::endpoint_host::map_persisted_queue_rejection](../../src/endpoint_host.rs#L2499) |
| `prompt` | `require_object_fields` | [1243](../../src/endpoint_host.rs#L1243) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `prompt` | `required_string` | [1248](../../src/endpoint_host.rs#L1248), [1249](../../src/endpoint_host.rs#L1249) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `prompt` | `Err` | [1252](../../src/endpoint_host.rs#L1252) | external-constructor-callback-or-unresolved |
| `prompt` | `invalid` | [1252](../../src/endpoint_host.rs#L1252), [1261](../../src/endpoint_host.rs#L1261), [1263](../../src/endpoint_host.rs#L1263) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `prompt` | `optional_string` | [1254](../../src/endpoint_host.rs#L1254) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2268) |
| `prompt` | `validate_time_zone` | [1255](../../src/endpoint_host.rs#L1255) | [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2513) |
| `prompt` | `serde_json::from_value(             payload                 .get("content")                 .cloned()                 .ok_or_else(&#124;&#124; invalid("content is required"))?,         )         .map_err` | [1257](../../src/endpoint_host.rs#L1257) | receiver-type-required |
| `prompt` | `serde_json::from_value` | [1257](../../src/endpoint_host.rs#L1257) | external-constructor-callback-or-unresolved |
| `prompt` | `payload                 .get("content")                 .cloned()                 .ok_or_else` | [1258](../../src/endpoint_host.rs#L1258) | receiver-type-required |
| `prompt` | `payload                 .get("content")                 .cloned` | [1258](../../src/endpoint_host.rs#L1258) | receiver-type-required |
| `prompt` | `payload                 .get` | [1258](../../src/endpoint_host.rs#L1258) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref().ok_or_else` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `prompt` | `self.delivery_authority.as_ref` | [1264](../../src/endpoint_host.rs#L1264) | receiver-type-required |
| `prompt` | `EndpointRpcFailure::new` | [1265](../../src/endpoint_host.rs#L1265) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `prompt` | `self.input_admission.with_active_session` | [1271](../../src/endpoint_host.rs#L1271) | receiver-type-required |
| `prompt` | `map_production_route_for` | [1273](../../src/endpoint_host.rs#L1273), [1288](../../src/endpoint_host.rs#L1288) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts(session_id, &parts)                     .map_err` | [1275](../../src/endpoint_host.rs#L1275) | receiver-type-required |
| `prompt` | `self                     .attachments                     .materialize_prompt_parts` | [1275](../../src/endpoint_host.rs#L1275) | receiver-type-required |
| `prompt` | `(self.clock)().map_err` | [1279](../../src/endpoint_host.rs#L1279) | receiver-type-required |
| `prompt` | `(self.clock)` | [1279](../../src/endpoint_host.rs#L1279) | external-constructor-callback-or-unresolved |
| `prompt` | `endpoint_origin` | [1280](../../src/endpoint_host.rs#L1280) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) |
| `prompt` | `authority                     .prompt(session_id, &timestamp, &origin, &materialized, steer)                     .map_err` | [1286](../../src/endpoint_host.rs#L1286) | receiver-type-required |
| `prompt` | `authority                     .prompt` | [1286](../../src/endpoint_host.rs#L1286) | receiver-type-required |
| `prompt` | `mark_event_handoff` | [1289](../../src/endpoint_host.rs#L1289) | [tekes-supervisor::endpoint_host::mark_event_handoff](../../src/endpoint_host.rs#L2337) |
| `prompt` | `to_ijson` | [1290](../../src/endpoint_host.rs#L1290) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `attachment` | `require_object_fields` | [1296](../../src/endpoint_host.rs#L1296) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `attachment` | `required_string` | [1301](../../src/endpoint_host.rs#L1301), [1302](../../src/endpoint_host.rs#L1302) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `attachment` | `self.ensure_active_session` | [1303](../../src/endpoint_host.rs#L1303) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::ensure_active_session](../../src/endpoint_host.rs#L1378) |
| `attachment` | `self             .attachments             .read_authorized(session_id, attachment_id)             .map_err` | [1304](../../src/endpoint_host.rs#L1304) | receiver-type-required |
| `attachment` | `self             .attachments             .read_authorized` | [1304](../../src/endpoint_host.rs#L1304) | receiver-type-required |
| `attachment` | `to_ijson` | [1308](../../src/endpoint_host.rs#L1308) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `fork_session` | `require_object_fields` | [1316](../../src/endpoint_host.rs#L1316) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `fork_session` | `required_string` | [1321](../../src/endpoint_host.rs#L1321) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `fork_session` | `optional_u64` | [1322](../../src/endpoint_host.rs#L1322) | [tekes-supervisor::endpoint_host::optional_u64](../../src/endpoint_host.rs#L2295) |
| `fork_session` | `optional_bool(payload, "ephemeral")?.unwrap_or` | [1323](../../src/endpoint_host.rs#L1323) | receiver-type-required |
| `fork_session` | `optional_bool` | [1323](../../src/endpoint_host.rs#L1323) | [tekes-supervisor::endpoint_host::optional_bool](../../src/endpoint_host.rs#L2258) |
| `fork_session` | `(self.clock)().map_err` | [1324](../../src/endpoint_host.rs#L1324) | receiver-type-required |
| `fork_session` | `(self.clock)` | [1324](../../src/endpoint_host.rs#L1324) | external-constructor-callback-or-unresolved |
| `fork_session` | `request_hash` | [1325](../../src/endpoint_host.rs#L1325) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `fork_session` | `self             .management             .fork_session(ForkSessionOperation {                 rpc_id: &request.rpc_id,                 request_sha256: &hash,                 source_session_id: session_id,                 at_endpoint_seq: at_seq,                 started_at: &timestamp,                 principal: &self.principal,                 ephemeral,                 preset: payload.get("preset"),                 preset_models: Some(&ConfiguredModels),                 user_agent_dir: &self.user_agent_dir,             })             .map_err` | [1326](../../src/endpoint_host.rs#L1326) | receiver-type-required |
| `fork_session` | `self             .management             .fork_session` | [1326](../../src/endpoint_host.rs#L1326) | receiver-type-required |
| `fork_session` | `payload.get` | [1336](../../src/endpoint_host.rs#L1336) | receiver-type-required |
| `fork_session` | `Some` | [1337](../../src/endpoint_host.rs#L1337) | external-constructor-callback-or-unresolved |
| `fork_session` | `mark_management_handoff` | [1341](../../src/endpoint_host.rs#L1341) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `fork_session` | `to_ijson` | [1342](../../src/endpoint_host.rs#L1342) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `discard_session` | `require_object_fields` | [1353](../../src/endpoint_host.rs#L1353) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `discard_session` | `required_string` | [1354](../../src/endpoint_host.rs#L1354) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `discard_session` | `self.input_admission.with_session_gate` | [1355](../../src/endpoint_host.rs#L1355) | receiver-type-required |
| `discard_session` | `map_production_route_for` | [1357](../../src/endpoint_host.rs#L1357) | [tekes-supervisor::endpoint_host::map_production_route_for](../../src/endpoint_host.rs#L1936) |
| `discard_session` | `(self.clock)().map_err` | [1359](../../src/endpoint_host.rs#L1359) | receiver-type-required |
| `discard_session` | `(self.clock)` | [1359](../../src/endpoint_host.rs#L1359) | external-constructor-callback-or-unresolved |
| `discard_session` | `request_hash` | [1360](../../src/endpoint_host.rs#L1360) | [tekes-supervisor::endpoint_host::request_hash](../../src/endpoint_host.rs#L2309) |
| `discard_session` | `self                     .management                     .discard_session(&request.rpc_id, &hash, session_id, &timestamp)                     .map_err` | [1361](../../src/endpoint_host.rs#L1361) | receiver-type-required |
| `discard_session` | `self                     .management                     .discard_session` | [1361](../../src/endpoint_host.rs#L1361) | receiver-type-required |
| `discard_session` | `mark_management_handoff` | [1365](../../src/endpoint_host.rs#L1365) | [tekes-supervisor::endpoint_host::mark_management_handoff](../../src/endpoint_host.rs#L2323) |
| `discard_session` | `to_ijson` | [1366](../../src/endpoint_host.rs#L1366) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `admission_gate` | `self.input_admission             .gates()             .gate(session_id)             .map_err` | [1372](../../src/endpoint_host.rs#L1372) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates()             .gate` | [1372](../../src/endpoint_host.rs#L1372) | receiver-type-required |
| `admission_gate` | `self.input_admission             .gates` | [1372](../../src/endpoint_host.rs#L1372) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id(session_id).map_err` | [1379](../../src/endpoint_host.rs#L1379) | receiver-type-required |
| `ensure_active_session` | `endpoint::validate_session_id` | [1379](../../src/endpoint_host.rs#L1379) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `ensure_active_session` | `invalid` | [1379](../../src/endpoint_host.rs#L1379) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `ensure_active_session` | `self.storage_root.join("archive").join(session_id).is_dir` | [1380](../../src/endpoint_host.rs#L1380) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("archive").join` | [1380](../../src/endpoint_host.rs#L1380) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join` | [1380](../../src/endpoint_host.rs#L1380), [1387](../../src/endpoint_host.rs#L1387) | receiver-type-required |
| `ensure_active_session` | `Err` | [1381](../../src/endpoint_host.rs#L1381), [1388](../../src/endpoint_host.rs#L1388) | external-constructor-callback-or-unresolved |
| `ensure_active_session` | `EndpointRpcFailure::new` | [1381](../../src/endpoint_host.rs#L1381), [1388](../../src/endpoint_host.rs#L1388) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `ensure_active_session` | `self.storage_root.join("threads").join(session_id).is_dir` | [1387](../../src/endpoint_host.rs#L1387) | receiver-type-required |
| `ensure_active_session` | `self.storage_root.join("threads").join` | [1387](../../src/endpoint_host.rs#L1387) | receiver-type-required |
| `ensure_active_session` | `Ok` | [1394](../../src/endpoint_host.rs#L1394) | external-constructor-callback-or-unresolved |
| `reconcile_legacy_internal_prompt_inner` | `claim.method` | [1403](../../src/endpoint_host.rs#L1403) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `claim.rpc_id` | [1404](../../src/endpoint_host.rs#L1404) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `serde_json::from_slice(&request.payload.canonical_bytes().ok()?).ok` | [1409](../../src/endpoint_host.rs#L1409) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `serde_json::from_slice` | [1409](../../src/endpoint_host.rs#L1409) | external-constructor-callback-or-unresolved |
| `reconcile_legacy_internal_prompt_inner` | `request.payload.canonical_bytes().ok` | [1409](../../src/endpoint_host.rs#L1409) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `request.payload.canonical_bytes` | [1409](../../src/endpoint_host.rs#L1409) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `require_object_fields(             &payload,             &["sessionId", "mode", "content", "clientTimeZone"],             &["sessionId", "mode", "content"],         )         .ok` | [1410](../../src/endpoint_host.rs#L1410) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `require_object_fields` | [1410](../../src/endpoint_host.rs#L1410) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `reconcile_legacy_internal_prompt_inner` | `required_string(&payload, "sessionId").ok` | [1416](../../src/endpoint_host.rs#L1416) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `required_string` | [1416](../../src/endpoint_host.rs#L1416), [1422](../../src/endpoint_host.rs#L1422) | [tekes-supervisor::endpoint_host::required_string](../../src/endpoint_host.rs#L2238) |
| `reconcile_legacy_internal_prompt_inner` | `claim.target_session` | [1417](../../src/endpoint_host.rs#L1417) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `Some` | [1417](../../src/endpoint_host.rs#L1417) | external-constructor-callback-or-unresolved |
| `reconcile_legacy_internal_prompt_inner` | `endpoint::validate_session_id(session_id).is_err` | [1418](../../src/endpoint_host.rs#L1418) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `endpoint::validate_session_id` | [1418](../../src/endpoint_host.rs#L1418) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `reconcile_legacy_internal_prompt_inner` | `required_string(&payload, "mode").ok` | [1422](../../src/endpoint_host.rs#L1422) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `optional_string(&payload, "clientTimeZone").ok` | [1427](../../src/endpoint_host.rs#L1427) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `optional_string` | [1427](../../src/endpoint_host.rs#L1427) | [tekes-supervisor::endpoint_host::optional_string](../../src/endpoint_host.rs#L2268) |
| `reconcile_legacy_internal_prompt_inner` | `validate_time_zone(time_zone).ok` | [1428](../../src/endpoint_host.rs#L1428) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `validate_time_zone` | [1428](../../src/endpoint_host.rs#L1428) | [tekes-supervisor::endpoint_host::validate_time_zone](../../src/endpoint_host.rs#L2513) |
| `reconcile_legacy_internal_prompt_inner` | `serde_json::from_value(payload.get("content")?.clone()).ok` | [1431](../../src/endpoint_host.rs#L1431) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `serde_json::from_value` | [1431](../../src/endpoint_host.rs#L1431) | external-constructor-callback-or-unresolved |
| `reconcile_legacy_internal_prompt_inner` | `payload.get("content")?.clone` | [1431](../../src/endpoint_host.rs#L1431) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `payload.get` | [1431](../../src/endpoint_host.rs#L1431) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `parts.is_empty` | [1432](../../src/endpoint_host.rs#L1432) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `self.storage_root.join("threads").join` | [1435](../../src/endpoint_host.rs#L1435) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `self.storage_root.join` | [1435](../../src/endpoint_host.rs#L1435), [1436](../../src/endpoint_host.rs#L1436) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `self.storage_root.join("archive").join` | [1436](../../src/endpoint_host.rs#L1436) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `active.is_dir` | [1437](../../src/endpoint_host.rs#L1437) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `archived.is_dir` | [1439](../../src/endpoint_host.rs#L1439) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `store::DirectoryLock::shared(&folder).ok` | [1444](../../src/endpoint_host.rs#L1444) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `store::DirectoryLock::shared` | [1444](../../src/endpoint_host.rs#L1444) | [store::platform::DirectoryLock::shared](../../../store/src/platform.rs#L58) |
| `reconcile_legacy_internal_prompt_inner` | `folder.join` | [1445](../../src/endpoint_host.rs#L1445) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `self             .attachments             .expected_prompt_blocks_read_only(&folder, &parts)             .ok` | [1446](../../src/endpoint_host.rs#L1446) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `self             .attachments             .expected_prompt_blocks_read_only` | [1446](../../src/endpoint_host.rs#L1446) | receiver-type-required |
| `reconcile_legacy_internal_prompt_inner` | `endpoint_origin` | [1450](../../src/endpoint_host.rs#L1450) | [tekes-supervisor::endpoint_host::endpoint_origin](../../src/endpoint_host.rs#L2351) |
| `reconcile_legacy_internal_prompt_inner` | `verify_and_sync_legacy_input` | [1456](../../src/endpoint_host.rs#L1456) | [tekes-supervisor::endpoint_host::verify_and_sync_legacy_input](../../src/endpoint_host.rs#L1502) |
| `reconcile_legacy_internal_prompt_inner` | `Arc::new` | [1462](../../src/endpoint_host.rs#L1462) | external-constructor-callback-or-unresolved |
| `find_unique_matching_legacy_input` | `projection.events.iter().filter` | [1474](../../src/endpoint_host.rs#L1474) | receiver-type-required |
| `find_unique_matching_legacy_input` | `projection.events.iter` | [1474](../../src/endpoint_host.rs#L1474) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event.kind` | [1475](../../src/endpoint_host.rs#L1475) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event.origin_key` | [1476](../../src/endpoint_host.rs#L1476) | receiver-type-required |
| `find_unique_matching_legacy_input` | `Some` | [1476](../../src/endpoint_host.rs#L1476), [1477](../../src/endpoint_host.rs#L1477), [1493](../../src/endpoint_host.rs#L1493), [1499](../../src/endpoint_host.rs#L1499) | external-constructor-callback-or-unresolved |
| `find_unique_matching_legacy_input` | `event.origin_tuple().ok().flatten().as_ref` | [1477](../../src/endpoint_host.rs#L1477) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event.origin_tuple().ok().flatten` | [1477](../../src/endpoint_host.rs#L1477) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event.origin_tuple().ok` | [1477](../../src/endpoint_host.rs#L1477) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event.origin_tuple` | [1477](../../src/endpoint_host.rs#L1477) | receiver-type-required |
| `find_unique_matching_legacy_input` | `serde_json::to_value` | [1481](../../src/endpoint_host.rs#L1481) | external-constructor-callback-or-unresolved |
| `find_unique_matching_legacy_input` | `event.raw` | [1481](../../src/endpoint_host.rs#L1481) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value             .get("steer")             .and_then(Value::as_bool)             .unwrap_or` | [1484](../../src/endpoint_host.rs#L1484) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value             .get("steer")             .and_then` | [1484](../../src/endpoint_host.rs#L1484) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value             .get` | [1484](../../src/endpoint_host.rs#L1484) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value                 .get("content")                 .and_then(&#124;value&#124; serde_json::from_value::<Vec<schema::Block>>(value.clone()).ok())                 .as_deref` | [1489](../../src/endpoint_host.rs#L1489) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value                 .get("content")                 .and_then` | [1489](../../src/endpoint_host.rs#L1489) | receiver-type-required |
| `find_unique_matching_legacy_input` | `event_value                 .get` | [1489](../../src/endpoint_host.rs#L1489) | receiver-type-required |
| `find_unique_matching_legacy_input` | `serde_json::from_value::<Vec<schema::Block>>(value.clone()).ok` | [1491](../../src/endpoint_host.rs#L1491) | receiver-type-required |
| `find_unique_matching_legacy_input` | `serde_json::from_value::<Vec<schema::Block>>` | [1491](../../src/endpoint_host.rs#L1491) | external-constructor-callback-or-unresolved |
| `find_unique_matching_legacy_input` | `value.clone` | [1491](../../src/endpoint_host.rs#L1491) | receiver-type-required |
| `find_unique_matching_legacy_input` | `matches.next` | [1495](../../src/endpoint_host.rs#L1495), [1496](../../src/endpoint_host.rs#L1496) | receiver-type-required |
| `find_unique_matching_legacy_input` | `matches.next().is_some` | [1496](../../src/endpoint_host.rs#L1496) | receiver-type-required |
| `find_unique_matching_legacy_input` | `matched.seq` | [1499](../../src/endpoint_host.rs#L1499) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::symlink_metadata(main_path)         .ok()?         .file_type()         .is_file` | [1513](../../src/endpoint_host.rs#L1513) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::symlink_metadata(main_path)         .ok()?         .file_type` | [1513](../../src/endpoint_host.rs#L1513) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::symlink_metadata(main_path)         .ok` | [1513](../../src/endpoint_host.rs#L1513) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::symlink_metadata` | [1513](../../src/endpoint_host.rs#L1513) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input` | `std::fs::OpenOptions::new()         .read(true)         .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)         .open(main_path)         .ok` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::OpenOptions::new()         .read(true)         .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)         .open` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::OpenOptions::new()         .read(true)         .custom_flags` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::OpenOptions::new()         .read` | [1523](../../src/endpoint_host.rs#L1523) | receiver-type-required |
| `verify_and_sync_legacy_input` | `std::fs::OpenOptions::new` | [1523](../../src/endpoint_host.rs#L1523) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input` | `file.metadata().ok()?.is_file` | [1528](../../src/endpoint_host.rs#L1528) | receiver-type-required |
| `verify_and_sync_legacy_input` | `file.metadata().ok` | [1528](../../src/endpoint_host.rs#L1528) | receiver-type-required |
| `verify_and_sync_legacy_input` | `file.metadata` | [1528](../../src/endpoint_host.rs#L1528) | receiver-type-required |
| `verify_and_sync_legacy_input` | `Vec::new` | [1531](../../src/endpoint_host.rs#L1531) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input` | `file.read_to_end(&mut bytes).ok` | [1532](../../src/endpoint_host.rs#L1532) | receiver-type-required |
| `verify_and_sync_legacy_input` | `file.read_to_end` | [1532](../../src/endpoint_host.rs#L1532) | receiver-type-required |
| `verify_and_sync_legacy_input` | `store::scan_valid_prefix` | [1533](../../src/endpoint_host.rs#L1533) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `verify_and_sync_legacy_input` | `find_unique_matching_legacy_input` | [1534](../../src/endpoint_host.rs#L1534) | [tekes-supervisor::endpoint_host::find_unique_matching_legacy_input](../../src/endpoint_host.rs#L1467) |
| `verify_and_sync_legacy_input` | `scan.projection.as_ref` | [1535](../../src/endpoint_host.rs#L1535) | receiver-type-required |
| `verify_and_sync_legacy_input` | `sync.full_sync(&file).ok` | [1541](../../src/endpoint_host.rs#L1541) | receiver-type-required |
| `verify_and_sync_legacy_input` | `sync.full_sync` | [1541](../../src/endpoint_host.rs#L1541) | receiver-type-required |
| `verify_and_sync_legacy_input` | `Some` | [1542](../../src/endpoint_host.rs#L1542) | external-constructor-callback-or-unresolved |
| `capabilities` | `Self::implemented_capabilities` | [1547](../../src/endpoint_host.rs#L1547) | external-constructor-callback-or-unresolved |
| `capabilities` | `self.routes.as_ref` | [1548](../../src/endpoint_host.rs#L1548) | receiver-type-required |
| `capabilities` | `capabilities.extend` | [1549](../../src/endpoint_host.rs#L1549) | receiver-type-required |
| `capabilities` | `routes.capabilities` | [1549](../../src/endpoint_host.rs#L1549) | receiver-type-required |
| `capabilities` | `self.provider_readiness.is_some` | [1551](../../src/endpoint_host.rs#L1551) | receiver-type-required |
| `capabilities` | `capabilities.insert` | [1552](../../src/endpoint_host.rs#L1552), [1553](../../src/endpoint_host.rs#L1553), [1554](../../src/endpoint_host.rs#L1554), [1557](../../src/endpoint_host.rs#L1557), [1558](../../src/endpoint_host.rs#L1558), [1559](../../src/endpoint_host.rs#L1559), [1562](../../src/endpoint_host.rs#L1562) | receiver-type-required |
| `capabilities` | `"session.models".to_owned` | [1552](../../src/endpoint_host.rs#L1552) | receiver-type-required |
| `capabilities` | `"models.list".to_owned` | [1553](../../src/endpoint_host.rs#L1553) | receiver-type-required |
| `capabilities` | `"session.selectModel".to_owned` | [1554](../../src/endpoint_host.rs#L1554) | receiver-type-required |
| `capabilities` | `self.delivery_authority.is_some` | [1556](../../src/endpoint_host.rs#L1556) | receiver-type-required |
| `capabilities` | `"session.prompt".to_owned` | [1557](../../src/endpoint_host.rs#L1557) | receiver-type-required |
| `capabilities` | `"session.cancel".to_owned` | [1558](../../src/endpoint_host.rs#L1558) | receiver-type-required |
| `capabilities` | `"session.rename".to_owned` | [1559](../../src/endpoint_host.rs#L1559) | receiver-type-required |
| `capabilities` | `self.queue_authority.is_some` | [1561](../../src/endpoint_host.rs#L1561) | receiver-type-required |
| `capabilities` | `"session.updateQueue".to_owned` | [1562](../../src/endpoint_host.rs#L1562) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref()                 .map_or_else` | [1569](../../src/endpoint_host.rs#L1569) | receiver-type-required |
| `extension_capabilities` | `self.routes                 .as_ref` | [1569](../../src/endpoint_host.rs#L1569) | receiver-type-required |
| `extension_capabilities` | `routes.capabilities` | [1571](../../src/endpoint_host.rs#L1571) | receiver-type-required |
| `extension_capabilities` | `BTreeSet::new` | [1573](../../src/endpoint_host.rs#L1573) | external-constructor-callback-or-unresolved |
| `method_class` | `TEKES_UNARY_ROUTES.iter().find` | [1578](../../src/endpoint_host.rs#L1578) | receiver-type-required |
| `method_class` | `TEKES_UNARY_ROUTES.iter` | [1578](../../src/endpoint_host.rs#L1578) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then(&#124;routes&#124; routes.extension_method_class(method))                 .unwrap_or` | [1585](../../src/endpoint_host.rs#L1585) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref()                 .and_then` | [1585](../../src/endpoint_host.rs#L1585) | receiver-type-required |
| `method_class` | `self                 .routes                 .as_ref` | [1585](../../src/endpoint_host.rs#L1585) | receiver-type-required |
| `method_class` | `routes.extension_method_class` | [1588](../../src/endpoint_host.rs#L1588) | receiver-type-required |
| `validate_request` | `serde_json::from_slice(             &request                 .payload                 .canonical_bytes()                 .map_err(&#124;_&#124; HostFailure::InvalidRequest)?,         )         .map_err` | [1594](../../src/endpoint_host.rs#L1594) | receiver-type-required |
| `validate_request` | `serde_json::from_slice` | [1594](../../src/endpoint_host.rs#L1594) | external-constructor-callback-or-unresolved |
| `validate_request` | `request                 .payload                 .canonical_bytes()                 .map_err` | [1595](../../src/endpoint_host.rs#L1595) | receiver-type-required |
| `validate_request` | `request                 .payload                 .canonical_bytes` | [1595](../../src/endpoint_host.rs#L1595) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter()             .any` | [1601](../../src/endpoint_host.rs#L1601) | receiver-type-required |
| `validate_request` | `TEKES_UNARY_ROUTES             .iter` | [1601](../../src/endpoint_host.rs#L1601) | receiver-type-required |
| `validate_request` | `validate_production_payload(&request.method, &payload)                 .map_err` | [1605](../../src/endpoint_host.rs#L1605) | receiver-type-required |
| `validate_request` | `validate_production_payload` | [1605](../../src/endpoint_host.rs#L1605) | [tekes-supervisor::endpoint_host::validate_production_payload](../../src/endpoint_host.rs#L2655) |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload(&request.method, &payload)                 .map_err` | [1608](../../src/endpoint_host.rs#L1608) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or(HostFailure::InvalidRequest)?                 .validate_extension_payload` | [1608](../../src/endpoint_host.rs#L1608) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter(&#124;routes&#124; routes.capabilities().contains(&request.method))                 .ok_or` | [1608](../../src/endpoint_host.rs#L1608) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref()                 .filter` | [1608](../../src/endpoint_host.rs#L1608) | receiver-type-required |
| `validate_request` | `self.routes                 .as_ref` | [1608](../../src/endpoint_host.rs#L1608) | receiver-type-required |
| `validate_request` | `routes.capabilities().contains` | [1610](../../src/endpoint_host.rs#L1610) | receiver-type-required |
| `validate_request` | `routes.capabilities` | [1610](../../src/endpoint_host.rs#L1610) | receiver-type-required |
| `reconcile_legacy_internal_prompt` | `self.reconcile_legacy_internal_prompt_inner` | [1622](../../src/endpoint_host.rs#L1622) | receiver-type-required |
| `call` | `Box::pin` | [1626](../../src/endpoint_host.rs#L1626) | external-constructor-callback-or-unresolved |
| `call` | `std::future::ready` | [1626](../../src/endpoint_host.rs#L1626) | external-constructor-callback-or-unresolved |
| `call` | `self.execute` | [1626](../../src/endpoint_host.rs#L1626) | receiver-type-required |
| `open` | `root.as_ref` | [1641](../../src/endpoint_host.rs#L1641) | receiver-type-required |
| `open` | `Ok` | [1642](../../src/endpoint_host.rs#L1642) | external-constructor-callback-or-unresolved |
| `open` | `ProductionEndpointHost::open` | [1643](../../src/endpoint_host.rs#L1643) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open](../../src/endpoint_host.rs#L561) |
| `open` | `EndpointDispatcher::new` | [1644](../../src/endpoint_host.rs#L1644) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L464) |
| `open` | `RpcRegistry::open` | [1644](../../src/endpoint_host.rs#L1644) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L138) |
| `open` | `EndpointSubscriptionHub::default` | [1645](../../src/endpoint_host.rs#L1645) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `root.as_ref` | [1654](../../src/endpoint_host.rs#L1654) | receiver-type-required |
| `open_with_clock` | `Ok` | [1655](../../src/endpoint_host.rs#L1655) | external-constructor-callback-or-unresolved |
| `open_with_clock` | `ProductionEndpointHost::open_with_clock` | [1656](../../src/endpoint_host.rs#L1656) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_clock](../../src/endpoint_host.rs#L576) |
| `open_with_clock` | `EndpointDispatcher::new` | [1657](../../src/endpoint_host.rs#L1657) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L464) |
| `open_with_clock` | `RpcRegistry::open` | [1657](../../src/endpoint_host.rs#L1657) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L138) |
| `open_with_clock` | `EndpointSubscriptionHub::default` | [1658](../../src/endpoint_host.rs#L1658) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `root.as_ref` | [1668](../../src/endpoint_host.rs#L1668) | receiver-type-required |
| `open_with_routes` | `Ok` | [1669](../../src/endpoint_host.rs#L1669) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `ProductionEndpointHost::open_with_routes` | [1670](../../src/endpoint_host.rs#L1670) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_routes](../../src/endpoint_host.rs#L585) |
| `open_with_routes` | `Some` | [1670](../../src/endpoint_host.rs#L1670) | external-constructor-callback-or-unresolved |
| `open_with_routes` | `EndpointDispatcher::new` | [1671](../../src/endpoint_host.rs#L1671) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L464) |
| `open_with_routes` | `RpcRegistry::open` | [1671](../../src/endpoint_host.rs#L1671) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L138) |
| `open_with_routes` | `EndpointSubscriptionHub::default` | [1672](../../src/endpoint_host.rs#L1672) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `root.as_ref` | [1682](../../src/endpoint_host.rs#L1682) | receiver-type-required |
| `open_with_route_authority` | `Ok` | [1683](../../src/endpoint_host.rs#L1683) | external-constructor-callback-or-unresolved |
| `open_with_route_authority` | `ProductionEndpointHost::open_with_route_authority` | [1684](../../src/endpoint_host.rs#L1684) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_route_authority](../../src/endpoint_host.rs#L661) |
| `open_with_route_authority` | `EndpointDispatcher::new` | [1693](../../src/endpoint_host.rs#L1693) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L464) |
| `open_with_route_authority` | `RpcRegistry::open` | [1693](../../src/endpoint_host.rs#L1693) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L138) |
| `open_with_route_authority` | `EndpointSubscriptionHub::default` | [1694](../../src/endpoint_host.rs#L1694) | external-constructor-callback-or-unresolved |
| `open_with_authorities` | `Self::open_with_full_authorities` | [1705](../../src/endpoint_host.rs#L1705) | [tekes-supervisor::endpoint_host::ProductionEndpointAssembly::open_with_full_authorities](../../src/endpoint_host.rs#L1716) |
| `open_with_full_authorities` | `root.as_ref` | [1725](../../src/endpoint_host.rs#L1725) | receiver-type-required |
| `open_with_full_authorities` | `Ok` | [1726](../../src/endpoint_host.rs#L1726) | external-constructor-callback-or-unresolved |
| `open_with_full_authorities` | `ProductionEndpointHost::open_with_full_authorities` | [1727](../../src/endpoint_host.rs#L1727) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities](../../src/endpoint_host.rs#L612) |
| `open_with_full_authorities` | `EndpointDispatcher::new` | [1736](../../src/endpoint_host.rs#L1736) | [endpoint::host::EndpointDispatcher::new](../../../endpoint/src/host.rs#L464) |
| `open_with_full_authorities` | `RpcRegistry::open` | [1736](../../src/endpoint_host.rs#L1736) | [endpoint::idempotency::RpcRegistry::open](../../../endpoint/src/idempotency.rs#L138) |
| `open_with_full_authorities` | `EndpointSubscriptionHub::default` | [1737](../../src/endpoint_host.rs#L1737) | external-constructor-callback-or-unresolved |
| `dispatch` | `Ok` | [1745](../../src/endpoint_host.rs#L1745) | external-constructor-callback-or-unresolved |
| `dispatch` | `self.dispatcher.dispatch` | [1745](../../src/endpoint_host.rs#L1745) | receiver-type-required |
| `admission_gates` | `self.host.admission_gates` | [1760](../../src/endpoint_host.rs#L1760) | receiver-type-required |
| `has_incomplete_management_operation` | `self.host.has_incomplete_management_operation` | [1767](../../src/endpoint_host.rs#L1767) | receiver-type-required |
| `success` | `Some` | [1774](../../src/endpoint_host.rs#L1774) | external-constructor-callback-or-unresolved |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or(config.settings.default_provider.as_deref())         .unwrap_or_default` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.provider.as_str())         .or` | [1783](../../src/endpoint_host.rs#L1783) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map` | [1783](../../src/endpoint_host.rs#L1783), [1789](../../src/endpoint_host.rs#L1789) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref` | [1783](../../src/endpoint_host.rs#L1783), [1789](../../src/endpoint_host.rs#L1789), [1795](../../src/endpoint_host.rs#L1795) | receiver-type-required |
| `model_projection` | `settings.provider.as_str` | [1786](../../src/endpoint_host.rs#L1786) | receiver-type-required |
| `model_projection` | `config.settings.default_provider.as_deref` | [1787](../../src/endpoint_host.rs#L1787) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or(config.settings.default_model.as_deref())         .unwrap_or_default` | [1789](../../src/endpoint_host.rs#L1789) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .map(&#124;settings&#124; settings.model.as_str())         .or` | [1789](../../src/endpoint_host.rs#L1789) | receiver-type-required |
| `model_projection` | `settings.model.as_str` | [1792](../../src/endpoint_host.rs#L1792) | receiver-type-required |
| `model_projection` | `config.settings.default_model.as_deref` | [1793](../../src/endpoint_host.rs#L1793) | receiver-type-required |
| `model_projection` | `config         .session_settings         .as_ref()         .and_then` | [1795](../../src/endpoint_host.rs#L1795) | receiver-type-required |
| `model_projection` | `settings.reasoning_effort.as_deref` | [1798](../../src/endpoint_host.rs#L1798) | receiver-type-required |
| `model_projection` | `Vec::new` | [1799](../../src/endpoint_host.rs#L1799), [1800](../../src/endpoint_host.rs#L1800), [1806](../../src/endpoint_host.rs#L1806) | external-constructor-callback-or-unresolved |
| `model_projection` | `provider.name.as_deref().unwrap_or` | [1802](../../src/endpoint_host.rs#L1802) | receiver-type-required |
| `model_projection` | `provider.name.as_deref` | [1802](../../src/endpoint_host.rs#L1802) | receiver-type-required |
| `model_projection` | `readiness             .iter()             .find` | [1803](../../src/endpoint_host.rs#L1803) | receiver-type-required |
| `model_projection` | `readiness             .iter` | [1803](../../src/endpoint_host.rs#L1803) | receiver-type-required |
| `model_projection` | `provider.models.iter().filter` | [1807](../../src/endpoint_host.rs#L1807) | receiver-type-required |
| `model_projection` | `provider.models.iter` | [1807](../../src/endpoint_host.rs#L1807) | receiver-type-required |
| `model_projection` | `provider::resolve_profile(provider, configured).ok` | [1808](../../src/endpoint_host.rs#L1808) | receiver-type-required |
| `model_projection` | `provider::resolve_profile` | [1808](../../src/endpoint_host.rs#L1808) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L714) |
| `model_projection` | `profile                 .as_ref()                 .map(&#124;profile&#124; profile.reasoning_efforts().to_vec())                 .unwrap_or_default` | [1809](../../src/endpoint_host.rs#L1809) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .map` | [1809](../../src/endpoint_host.rs#L1809) | receiver-type-required |
| `model_projection` | `profile                 .as_ref` | [1809](../../src/endpoint_host.rs#L1809), [1813](../../src/endpoint_host.rs#L1813) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts().to_vec` | [1811](../../src/endpoint_host.rs#L1811) | receiver-type-required |
| `model_projection` | `profile.reasoning_efforts` | [1811](../../src/endpoint_host.rs#L1811) | receiver-type-required |
| `model_projection` | `profile                 .as_ref()                 .and_then` | [1813](../../src/endpoint_host.rs#L1813) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort().map` | [1815](../../src/endpoint_host.rs#L1815) | receiver-type-required |
| `model_projection` | `profile.default_reasoning_effort` | [1815](../../src/endpoint_host.rs#L1815) | receiver-type-required |
| `model_projection` | `efforts                 .iter()                 .any` | [1816](../../src/endpoint_host.rs#L1816) | receiver-type-required |
| `model_projection` | `efforts                 .iter` | [1816](../../src/endpoint_host.rs#L1816) | receiver-type-required |
| `model_projection` | `effort.is_empty` | [1818](../../src/endpoint_host.rs#L1818) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter().all` | [1818](../../src/endpoint_host.rs#L1818) | receiver-type-required |
| `model_projection` | `effort.as_bytes().iter` | [1818](../../src/endpoint_host.rs#L1818) | receiver-type-required |
| `model_projection` | `effort.as_bytes` | [1818](../../src/endpoint_host.rs#L1818) | receiver-type-required |
| `model_projection` | `Err` | [1820](../../src/endpoint_host.rs#L1820), [1823](../../src/endpoint_host.rs#L1823), [1829](../../src/endpoint_host.rs#L1829) | external-constructor-callback-or-unresolved |
| `model_projection` | `internal_failure` | [1820](../../src/endpoint_host.rs#L1820), [1823](../../src/endpoint_host.rs#L1823), [1829](../../src/endpoint_host.rs#L1829) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>().len` | [1822](../../src/endpoint_host.rs#L1822) | receiver-type-required |
| `model_projection` | `efforts.iter().collect::<BTreeSet<_>>` | [1822](../../src/endpoint_host.rs#L1822) | receiver-type-required |
| `model_projection` | `efforts.iter` | [1822](../../src/endpoint_host.rs#L1822) | receiver-type-required |
| `model_projection` | `efforts.len` | [1822](../../src/endpoint_host.rs#L1822) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref()                 .is_some_and` | [1825](../../src/endpoint_host.rs#L1825) | receiver-type-required |
| `model_projection` | `default_effort                 .as_ref` | [1825](../../src/endpoint_host.rs#L1825) | receiver-type-required |
| `model_projection` | `efforts.contains` | [1827](../../src/endpoint_host.rs#L1827) | receiver-type-required |
| `model_projection` | `efforts.is_empty` | [1833](../../src/endpoint_host.rs#L1833) | receiver-type-required |
| `model_projection` | `default_effort.as_ref` | [1839](../../src/endpoint_host.rs#L1839) | receiver-type-required |
| `model_projection` | `Value::String` | [1840](../../src/endpoint_host.rs#L1840), [1862](../../src/endpoint_host.rs#L1862) | external-constructor-callback-or-unresolved |
| `model_projection` | `default.clone` | [1840](../../src/endpoint_host.rs#L1840) | receiver-type-required |
| `model_projection` | `models.push` | [1843](../../src/endpoint_host.rs#L1843) | receiver-type-required |
| `model_projection` | `groups.push` | [1845](../../src/endpoint_host.rs#L1845) | receiver-type-required |
| `model_projection` | `runtime.map` | [1846](../../src/endpoint_host.rs#L1846) | receiver-type-required |
| `model_projection` | `failures.push` | [1848](../../src/endpoint_host.rs#L1848), [1853](../../src/endpoint_host.rs#L1853) | receiver-type-required |
| `model_projection` | `effort.to_owned` | [1862](../../src/endpoint_host.rs#L1862) | receiver-type-required |
| `model_projection` | `model_is_configured` | [1864](../../src/endpoint_host.rs#L1864) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1888) |
| `model_projection` | `to_ijson` | [1865](../../src/endpoint_host.rs#L1865) | [tekes-supervisor::endpoint_host::to_ijson](../../src/endpoint_host.rs#L2305) |
| `model_available` | `model_is_configured` | [1884](../../src/endpoint_host.rs#L1884) | [tekes-supervisor::endpoint_host::model_is_configured](../../src/endpoint_host.rs#L1888) |
| `model_is_configured` | `config         .providers         .providers         .iter()         .find` | [1894](../../src/endpoint_host.rs#L1894) | receiver-type-required |
| `model_is_configured` | `config         .providers         .providers         .iter` | [1894](../../src/endpoint_host.rs#L1894) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter()         .find` | [1902](../../src/endpoint_host.rs#L1902) | receiver-type-required |
| `model_is_configured` | `provider         .models         .iter` | [1902](../../src/endpoint_host.rs#L1902) | receiver-type-required |
| `model_is_configured` | `effort.is_none_or` | [1909](../../src/endpoint_host.rs#L1909) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile(provider, model).is_ok_and` | [1910](../../src/endpoint_host.rs#L1910) | receiver-type-required |
| `model_is_configured` | `provider::resolve_profile` | [1910](../../src/endpoint_host.rs#L1910) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L714) |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter()                 .any` | [1911](../../src/endpoint_host.rs#L1911) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts()                 .iter` | [1911](../../src/endpoint_host.rs#L1911) | receiver-type-required |
| `model_is_configured` | `profile                 .reasoning_efforts` | [1911](../../src/endpoint_host.rs#L1911) | receiver-type-required |
| `map_production_route_for` | `production_failure_is_exact_for` | [1937](../../src/endpoint_host.rs#L1937) | [tekes-supervisor::endpoint_host::production_failure_is_exact_for](../../src/endpoint_host.rs#L1970) |
| `map_production_route_for` | `sanitized_route_log_message` | [1938](../../src/endpoint_host.rs#L1938) | [tekes-supervisor::endpoint_host::sanitized_route_log_message](../../src/endpoint_host.rs#L1950) |
| `map_production_route_for` | `internal_failure` | [1945](../../src/endpoint_host.rs#L1945) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `map_production_route_for` | `map_production_route` | [1947](../../src/endpoint_host.rs#L1947) | [tekes-supervisor::endpoint_host::map_production_route](../../src/endpoint_host.rs#L1928) |
| `sanitized_route_log_message` | `message.len().min` | [1951](../../src/endpoint_host.rs#L1951) | receiver-type-required |
| `sanitized_route_log_message` | `message.len` | [1951](../../src/endpoint_host.rs#L1951) | receiver-type-required |
| `sanitized_route_log_message` | `message.is_char_boundary` | [1952](../../src/endpoint_host.rs#L1952) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse(         &serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect("route log message is JSON"),     )     .expect` | [1955](../../src/endpoint_host.rs#L1955) | receiver-type-required |
| `sanitized_route_log_message` | `IJsonValue::parse` | [1955](../../src/endpoint_host.rs#L1955) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `sanitized_route_log_message` | `serde_json::to_vec(&serde_json::json!({"detail":&message[..end]}))             .expect` | [1956](../../src/endpoint_host.rs#L1956) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_vec` | [1956](../../src/endpoint_host.rs#L1956) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `tools::SecretScanner::default().scan` | [1960](../../src/endpoint_host.rs#L1960) | receiver-type-required |
| `sanitized_route_log_message` | `tools::SecretScanner::default` | [1960](../../src/endpoint_host.rs#L1960) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `"<withheld>".to_owned` | [1962](../../src/endpoint_host.rs#L1962), [1967](../../src/endpoint_host.rs#L1967) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then(&#124;value&#124; value["detail"].as_str().map(str::to_owned))         .unwrap_or_else` | [1964](../../src/endpoint_host.rs#L1964) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok()         .and_then` | [1964](../../src/endpoint_host.rs#L1964) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value(scanned)         .ok` | [1964](../../src/endpoint_host.rs#L1964) | receiver-type-required |
| `sanitized_route_log_message` | `serde_json::to_value` | [1964](../../src/endpoint_host.rs#L1964) | external-constructor-callback-or-unresolved |
| `sanitized_route_log_message` | `value["detail"].as_str().map` | [1966](../../src/endpoint_host.rs#L1966) | receiver-type-required |
| `sanitized_route_log_message` | `value["detail"].as_str` | [1966](../../src/endpoint_host.rs#L1966) | receiver-type-required |
| `production_failure_is_exact_for` | `route_allows_error` | [1974](../../src/endpoint_host.rs#L1974) | [tekes-supervisor::endpoint_host::route_allows_error](../../src/endpoint_host.rs#L2114) |
| `production_failure_is_exact_for` | `route_error_is_exact` | [1974](../../src/endpoint_host.rs#L1974) | [tekes-supervisor::endpoint_host::route_error_is_exact](../../src/endpoint_host.rs#L1977) |
| `route_error_is_exact` | `route_error_message` | [1978](../../src/endpoint_host.rs#L1978) | [tekes-supervisor::endpoint_host::route_error_message](../../src/endpoint_host.rs#L2061) |
| `route_error_is_exact` | `error.details.canonical_bytes` | [1984](../../src/endpoint_host.rs#L1984) | receiver-type-required |
| `route_error_is_exact` | `serde_json::from_slice::<Value>` | [1987](../../src/endpoint_host.rs#L1987) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `error.code.as_str` | [1991](../../src/endpoint_host.rs#L1991) | receiver-type-required |
| `route_error_is_exact` | `details.is_empty` | [1992](../../src/endpoint_host.rs#L1992) | receiver-type-required |
| `route_error_is_exact` | `exact_string_fields` | [1993](../../src/endpoint_host.rs#L1993), [1995](../../src/endpoint_host.rs#L1995), [1998](../../src/endpoint_host.rs#L1998), [2000](../../src/endpoint_host.rs#L2000), [2002](../../src/endpoint_host.rs#L2002), [2005](../../src/endpoint_host.rs#L2005), [2007](../../src/endpoint_host.rs#L2007), [2008](../../src/endpoint_host.rs#L2008), [2010](../../src/endpoint_host.rs#L2010), [2012](../../src/endpoint_host.rs#L2012), [2024](../../src/endpoint_host.rs#L2024), [2045](../../src/endpoint_host.rs#L2045), [2047](../../src/endpoint_host.rs#L2047) | [tekes-supervisor::endpoint_host::exact_string_fields](../../src/endpoint_host.rs#L2093) |
| `route_error_is_exact` | `details.len` | [2014](../../src/endpoint_host.rs#L2014), [2021](../../src/endpoint_host.rs#L2021), [2027](../../src/endpoint_host.rs#L2027) | receiver-type-required |
| `route_error_is_exact` | `details.get("sessionId").is_some_and` | [2015](../../src/endpoint_host.rs#L2015) | receiver-type-required |
| `route_error_is_exact` | `details.get` | [2015](../../src/endpoint_host.rs#L2015), [2021](../../src/endpoint_host.rs#L2021) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("atSeq")                     .is_some_and` | [2016](../../src/endpoint_host.rs#L2016) | receiver-type-required |
| `route_error_is_exact` | `details                     .get` | [2016](../../src/endpoint_host.rs#L2016), [2028](../../src/endpoint_host.rs#L2028), [2048](../../src/endpoint_host.rs#L2048) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64().is_some` | [2018](../../src/endpoint_host.rs#L2018) | receiver-type-required |
| `route_error_is_exact` | `value.as_u64` | [2018](../../src/endpoint_host.rs#L2018) | receiver-type-required |
| `route_error_is_exact` | `details.get("limit").and_then` | [2021](../../src/endpoint_host.rs#L2021) | receiver-type-required |
| `route_error_is_exact` | `Some` | [2021](../../src/endpoint_host.rs#L2021) | external-constructor-callback-or-unresolved |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then(Value::as_str)                     .is_some_and` | [2028](../../src/endpoint_host.rs#L2028), [2048](../../src/endpoint_host.rs#L2048) | receiver-type-required |
| `route_error_is_exact` | `details                     .get("reason")                     .and_then` | [2028](../../src/endpoint_host.rs#L2028), [2048](../../src/endpoint_host.rs#L2048) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter()                             .any` | [2052](../../src/endpoint_host.rs#L2052) | receiver-type-required |
| `route_error_is_exact` | `tools::PresetInvalidReason::ALL                             .iter` | [2052](../../src/endpoint_host.rs#L2052) | receiver-type-required |
| `route_error_is_exact` | `known.as_str` | [2054](../../src/endpoint_host.rs#L2054) | receiver-type-required |
| `route_error_message` | `Some` | [2062](../../src/endpoint_host.rs#L2062) | external-constructor-callback-or-unresolved |
| `exact_string_fields` | `required         .iter()         .any` | [2098](../../src/endpoint_host.rs#L2098) | receiver-type-required |
| `exact_string_fields` | `required         .iter` | [2098](../../src/endpoint_host.rs#L2098) | receiver-type-required |
| `exact_string_fields` | `details.get(*key).is_some_and` | [2100](../../src/endpoint_host.rs#L2100), [2103](../../src/endpoint_host.rs#L2103) | receiver-type-required |
| `exact_string_fields` | `details.get` | [2100](../../src/endpoint_host.rs#L2100), [2103](../../src/endpoint_host.rs#L2103) | receiver-type-required |
| `exact_string_fields` | `optional             .iter()             .any` | [2101](../../src/endpoint_host.rs#L2101) | receiver-type-required |
| `exact_string_fields` | `optional             .iter` | [2101](../../src/endpoint_host.rs#L2101) | receiver-type-required |
| `exact_string_fields` | `value.is_string` | [2103](../../src/endpoint_host.rs#L2103) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter(&#124;key&#124; details.contains_key(**key))         .count` | [2107](../../src/endpoint_host.rs#L2107) | receiver-type-required |
| `exact_string_fields` | `optional         .iter()         .filter` | [2107](../../src/endpoint_host.rs#L2107) | receiver-type-required |
| `exact_string_fields` | `optional         .iter` | [2107](../../src/endpoint_host.rs#L2107) | receiver-type-required |
| `exact_string_fields` | `details.contains_key` | [2109](../../src/endpoint_host.rs#L2109) | receiver-type-required |
| `exact_string_fields` | `details.len` | [2111](../../src/endpoint_host.rs#L2111) | receiver-type-required |
| `exact_string_fields` | `required.len` | [2111](../../src/endpoint_host.rs#L2111) | receiver-type-required |
| `internal_failure` | `EndpointRpcFailure::new` | [2223](../../src/endpoint_host.rs#L2223) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `failure` | `Some` | [2230](../../src/endpoint_host.rs#L2230) | external-constructor-callback-or-unresolved |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter(&#124;value&#124; !value.is_empty())         .ok_or_else` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .filter` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `required_string` | `value         .as_object()         .and_then` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `required_string` | `value         .as_object` | [2239](../../src/endpoint_host.rs#L2239) | receiver-type-required |
| `required_string` | `object.get` | [2241](../../src/endpoint_host.rs#L2241) | receiver-type-required |
| `required_string` | `value.is_empty` | [2243](../../src/endpoint_host.rs#L2243) | receiver-type-required |
| `required_string` | `invalid` | [2244](../../src/endpoint_host.rs#L2244) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then(Value::as_str)         .ok_or_else` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then(&#124;object&#124; object.get(key))         .and_then` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object()         .and_then` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `required_string_allow_whitespace` | `value         .as_object` | [2251](../../src/endpoint_host.rs#L2251) | receiver-type-required |
| `required_string_allow_whitespace` | `object.get` | [2253](../../src/endpoint_host.rs#L2253) | receiver-type-required |
| `required_string_allow_whitespace` | `invalid` | [2255](../../src/endpoint_host.rs#L2255) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `optional_bool` | `value.as_object().and_then` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `optional_bool` | `value.as_object` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `optional_bool` | `object.get` | [2259](../../src/endpoint_host.rs#L2259) | receiver-type-required |
| `optional_bool` | `Ok` | [2260](../../src/endpoint_host.rs#L2260) | external-constructor-callback-or-unresolved |
| `optional_bool` | `value         .as_bool()         .map(Some)         .ok_or_else` | [2262](../../src/endpoint_host.rs#L2262) | receiver-type-required |
| `optional_bool` | `value         .as_bool()         .map` | [2262](../../src/endpoint_host.rs#L2262) | receiver-type-required |
| `optional_bool` | `value         .as_bool` | [2262](../../src/endpoint_host.rs#L2262) | receiver-type-required |
| `optional_bool` | `invalid` | [2265](../../src/endpoint_host.rs#L2265) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `optional_string` | `value.as_object().and_then` | [2269](../../src/endpoint_host.rs#L2269) | receiver-type-required |
| `optional_string` | `value.as_object` | [2269](../../src/endpoint_host.rs#L2269) | receiver-type-required |
| `optional_string` | `object.get` | [2269](../../src/endpoint_host.rs#L2269) | receiver-type-required |
| `optional_string` | `Ok` | [2270](../../src/endpoint_host.rs#L2270) | external-constructor-callback-or-unresolved |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map(Some)         .ok_or_else` | [2272](../../src/endpoint_host.rs#L2272) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter(&#124;value&#124; !value.is_empty())         .map` | [2272](../../src/endpoint_host.rs#L2272) | receiver-type-required |
| `optional_string` | `value         .as_str()         .filter` | [2272](../../src/endpoint_host.rs#L2272) | receiver-type-required |
| `optional_string` | `value         .as_str` | [2272](../../src/endpoint_host.rs#L2272) | receiver-type-required |
| `optional_string` | `value.is_empty` | [2274](../../src/endpoint_host.rs#L2274) | receiver-type-required |
| `optional_string` | `invalid` | [2276](../../src/endpoint_host.rs#L2276) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `require_object_fields` | `value         .as_object()         .ok_or_else` | [2284](../../src/endpoint_host.rs#L2284) | receiver-type-required |
| `require_object_fields` | `value         .as_object` | [2284](../../src/endpoint_host.rs#L2284) | receiver-type-required |
| `require_object_fields` | `invalid` | [2286](../../src/endpoint_host.rs#L2286), [2290](../../src/endpoint_host.rs#L2290) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `require_object_fields` | `object.keys().any` | [2287](../../src/endpoint_host.rs#L2287) | receiver-type-required |
| `require_object_fields` | `object.keys` | [2287](../../src/endpoint_host.rs#L2287) | receiver-type-required |
| `require_object_fields` | `allowed.contains` | [2287](../../src/endpoint_host.rs#L2287) | receiver-type-required |
| `require_object_fields` | `key.as_str` | [2287](../../src/endpoint_host.rs#L2287) | receiver-type-required |
| `require_object_fields` | `required.iter().any` | [2288](../../src/endpoint_host.rs#L2288) | receiver-type-required |
| `require_object_fields` | `required.iter` | [2288](../../src/endpoint_host.rs#L2288) | receiver-type-required |
| `require_object_fields` | `object.contains_key` | [2288](../../src/endpoint_host.rs#L2288) | receiver-type-required |
| `require_object_fields` | `Err` | [2290](../../src/endpoint_host.rs#L2290) | external-constructor-callback-or-unresolved |
| `require_object_fields` | `Ok` | [2292](../../src/endpoint_host.rs#L2292) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value.as_object().and_then` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `optional_u64` | `value.as_object` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `optional_u64` | `object.get` | [2296](../../src/endpoint_host.rs#L2296) | receiver-type-required |
| `optional_u64` | `Ok` | [2297](../../src/endpoint_host.rs#L2297) | external-constructor-callback-or-unresolved |
| `optional_u64` | `value         .as_u64()         .map(Some)         .ok_or_else` | [2299](../../src/endpoint_host.rs#L2299) | receiver-type-required |
| `optional_u64` | `value         .as_u64()         .map` | [2299](../../src/endpoint_host.rs#L2299) | receiver-type-required |
| `optional_u64` | `value         .as_u64` | [2299](../../src/endpoint_host.rs#L2299) | receiver-type-required |
| `optional_u64` | `invalid` | [2302](../../src/endpoint_host.rs#L2302) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `to_ijson` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(internal_json)?).map_err` | [2306](../../src/endpoint_host.rs#L2306) | receiver-type-required |
| `to_ijson` | `IJsonValue::parse` | [2306](../../src/endpoint_host.rs#L2306) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `to_ijson` | `serde_json::to_vec(value).map_err` | [2306](../../src/endpoint_host.rs#L2306) | receiver-type-required |
| `to_ijson` | `serde_json::to_vec` | [2306](../../src/endpoint_host.rs#L2306) | external-constructor-callback-or-unresolved |
| `request_hash` | `serde_json::from_slice(&request.payload.canonical_bytes().map_err(internal_schema)?)             .map_err` | [2311](../../src/endpoint_host.rs#L2311) | receiver-type-required |
| `request_hash` | `serde_json::from_slice` | [2311](../../src/endpoint_host.rs#L2311) | external-constructor-callback-or-unresolved |
| `request_hash` | `request.payload.canonical_bytes().map_err` | [2311](../../src/endpoint_host.rs#L2311) | receiver-type-required |
| `request_hash` | `request.payload.canonical_bytes` | [2311](../../src/endpoint_host.rs#L2311) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec(&json!({         "type":"client-request",         "rpcId":request.rpc_id,         "method":request.operation,         "payload":payload,     }))     .map_err` | [2313](../../src/endpoint_host.rs#L2313) | receiver-type-required |
| `request_hash` | `serde_json_canonicalizer::to_vec` | [2313](../../src/endpoint_host.rs#L2313) | external-constructor-callback-or-unresolved |
| `request_hash` | `EndpointRpcFailure::new` | [2319](../../src/endpoint_host.rs#L2319) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `request_hash` | `Ok` | [2320](../../src/endpoint_host.rs#L2320) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "management".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "management-operation".to_owned(),                 id: request.rpc_id.clone(),                 seq: None,             }),         })         .map_err` | [2324](../../src/endpoint_host.rs#L2324) | receiver-type-required |
| `mark_management_handoff` | `request         .handoff         .mark_handed_off` | [2324](../../src/endpoint_host.rs#L2324) | receiver-type-required |
| `mark_management_handoff` | `"management".to_owned` | [2327](../../src/endpoint_host.rs#L2327) | receiver-type-required |
| `mark_management_handoff` | `Some` | [2328](../../src/endpoint_host.rs#L2328) | external-constructor-callback-or-unresolved |
| `mark_management_handoff` | `"management-operation".to_owned` | [2329](../../src/endpoint_host.rs#L2329) | receiver-type-required |
| `mark_management_handoff` | `request.rpc_id.clone` | [2330](../../src/endpoint_host.rs#L2330) | receiver-type-required |
| `mark_management_handoff` | `EndpointRpcFailure::new` | [2334](../../src/endpoint_host.rs#L2334) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off(DurableHandoffProof {             delivery: "locked-append".to_owned(),             durable_identity: Some(RpcDurableIdentity {                 kind: "event-origin".to_owned(),                 id: request.rpc_id.clone(),                 seq: Some(seq),             }),         })         .map_err` | [2338](../../src/endpoint_host.rs#L2338) | receiver-type-required |
| `mark_event_handoff` | `request         .handoff         .mark_handed_off` | [2338](../../src/endpoint_host.rs#L2338) | receiver-type-required |
| `mark_event_handoff` | `"locked-append".to_owned` | [2341](../../src/endpoint_host.rs#L2341) | receiver-type-required |
| `mark_event_handoff` | `Some` | [2342](../../src/endpoint_host.rs#L2342), [2345](../../src/endpoint_host.rs#L2345) | external-constructor-callback-or-unresolved |
| `mark_event_handoff` | `"event-origin".to_owned` | [2343](../../src/endpoint_host.rs#L2343) | receiver-type-required |
| `mark_event_handoff` | `request.rpc_id.clone` | [2344](../../src/endpoint_host.rs#L2344) | receiver-type-required |
| `mark_event_handoff` | `internal_failure` | [2348](../../src/endpoint_host.rs#L2348) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `endpoint_origin` | `principal.to_owned` | [2358](../../src/endpoint_host.rs#L2358) | receiver-type-required |
| `endpoint_origin` | `endpoint::ORIGIN_CLIENT.to_owned` | [2359](../../src/endpoint_host.rs#L2359) | receiver-type-required |
| `endpoint_origin` | `session_id.to_owned` | [2360](../../src/endpoint_host.rs#L2360) | receiver-type-required |
| `endpoint_origin` | `operation.to_owned` | [2361](../../src/endpoint_host.rs#L2361) | receiver-type-required |
| `endpoint_origin` | `rpc_id.to_owned` | [2362](../../src/endpoint_host.rs#L2362) | receiver-type-required |
| `validate_session_title` | `title.trim` | [2367](../../src/endpoint_host.rs#L2367) | receiver-type-required |
| `validate_session_title` | `title.is_empty` | [2368](../../src/endpoint_host.rs#L2368) | receiver-type-required |
| `validate_session_title` | `title.len` | [2369](../../src/endpoint_host.rs#L2369) | receiver-type-required |
| `validate_session_title` | `title.chars().any` | [2370](../../src/endpoint_host.rs#L2370) | receiver-type-required |
| `validate_session_title` | `title.chars` | [2370](../../src/endpoint_host.rs#L2370) | receiver-type-required |
| `validate_session_title` | `Err` | [2372](../../src/endpoint_host.rs#L2372) | external-constructor-callback-or-unresolved |
| `validate_session_title` | `Ok` | [2374](../../src/endpoint_host.rs#L2374) | external-constructor-callback-or-unresolved |
| `queue_action` | `value         .as_object()         .ok_or_else` | [2401](../../src/endpoint_host.rs#L2401) | receiver-type-required |
| `queue_action` | `value         .as_object` | [2401](../../src/endpoint_host.rs#L2401) | receiver-type-required |
| `queue_action` | `invalid` | [2403](../../src/endpoint_host.rs#L2403), [2407](../../src/endpoint_host.rs#L2407), [2422](../../src/endpoint_host.rs#L2422), [2446](../../src/endpoint_host.rs#L2446) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `queue_action` | `object         .get("kind")         .and_then(Value::as_str)         .ok_or_else` | [2404](../../src/endpoint_host.rs#L2404) | receiver-type-required |
| `queue_action` | `object         .get("kind")         .and_then` | [2404](../../src/endpoint_host.rs#L2404) | receiver-type-required |
| `queue_action` | `object         .get` | [2404](../../src/endpoint_host.rs#L2404) | receiver-type-required |
| `queue_action` | `object.len` | [2409](../../src/endpoint_host.rs#L2409), [2410](../../src/endpoint_host.rs#L2410), [2412](../../src/endpoint_host.rs#L2412) | receiver-type-required |
| `queue_action` | `Ok` | [2409](../../src/endpoint_host.rs#L2409), [2410](../../src/endpoint_host.rs#L2410), [2444](../../src/endpoint_host.rs#L2444) | external-constructor-callback-or-unresolved |
| `queue_action` | `object.contains_key` | [2413](../../src/endpoint_host.rs#L2413) | receiver-type-required |
| `queue_action` | `object                     .keys()                     .all` | [2414](../../src/endpoint_host.rs#L2414) | receiver-type-required |
| `queue_action` | `object                     .keys` | [2414](../../src/endpoint_host.rs#L2414) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter(&#124;values&#124; !values.is_empty())                 .ok_or_else` | [2418](../../src/endpoint_host.rs#L2418) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then(Value::as_array)                 .filter` | [2418](../../src/endpoint_host.rs#L2418) | receiver-type-required |
| `queue_action` | `object                 .get("content")                 .and_then` | [2418](../../src/endpoint_host.rs#L2418) | receiver-type-required |
| `queue_action` | `object                 .get` | [2418](../../src/endpoint_host.rs#L2418) | receiver-type-required |
| `queue_action` | `values.is_empty` | [2421](../../src/endpoint_host.rs#L2421) | receiver-type-required |
| `queue_action` | `Vec::with_capacity` | [2423](../../src/endpoint_host.rs#L2423) | external-constructor-callback-or-unresolved |
| `queue_action` | `values.len` | [2423](../../src/endpoint_host.rs#L2423) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then(Value::as_str)                     .ok_or_else` | [2425](../../src/endpoint_host.rs#L2425) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then(&#124;block&#124; block.get("text"))                     .and_then` | [2425](../../src/endpoint_host.rs#L2425) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter(&#124;block&#124; {                         block.len() == 2                             && block.get("type").and_then(Value::as_str) == Some("text")                     })                     .and_then` | [2425](../../src/endpoint_host.rs#L2425) | receiver-type-required |
| `queue_action` | `value                     .as_object()                     .filter` | [2425](../../src/endpoint_host.rs#L2425) | receiver-type-required |
| `queue_action` | `value                     .as_object` | [2425](../../src/endpoint_host.rs#L2425) | receiver-type-required |
| `queue_action` | `block.len` | [2428](../../src/endpoint_host.rs#L2428) | receiver-type-required |
| `queue_action` | `block.get("type").and_then` | [2429](../../src/endpoint_host.rs#L2429) | receiver-type-required |
| `queue_action` | `block.get` | [2429](../../src/endpoint_host.rs#L2429), [2431](../../src/endpoint_host.rs#L2431) | receiver-type-required |
| `queue_action` | `Some` | [2429](../../src/endpoint_host.rs#L2429) | external-constructor-callback-or-unresolved |
| `queue_action` | `EndpointRpcFailure::new` | [2434](../../src/endpoint_host.rs#L2434) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `queue_action` | `content.push` | [2440](../../src/endpoint_host.rs#L2440) | receiver-type-required |
| `queue_action` | `block.to_owned` | [2441](../../src/endpoint_host.rs#L2441) | receiver-type-required |
| `queue_action` | `Err` | [2446](../../src/endpoint_host.rs#L2446) | external-constructor-callback-or-unresolved |
| `map_queue_rejection` | `EndpointRpcFailure::new` | [2456](../../src/endpoint_host.rs#L2456), [2461](../../src/endpoint_host.rs#L2461), [2466](../../src/endpoint_host.rs#L2466) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `queue_decision` | `match code {                     worker_control::QueueTransactionRejectCode::QueueItemNotFound => {                         "queue-item-not-found"                     }                     worker_control::QueueTransactionRejectCode::SteerUnavailable => {                         "steer-unavailable"                     }                     worker_control::QueueTransactionRejectCode::AttachmentError => {                         "attachment-error"                     }                 }                 .to_owned` | [2481](../../src/endpoint_host.rs#L2481) | receiver-type-required |
| `queue_decision` | `reason.clone` | [2493](../../src/endpoint_host.rs#L2493) | receiver-type-required |
| `map_persisted_queue_rejection` | `internal_failure` | [2508](../../src/endpoint_host.rs#L2508) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `map_persisted_queue_rejection` | `map_queue_rejection` | [2510](../../src/endpoint_host.rs#L2510) | [tekes-supervisor::endpoint_host::map_queue_rejection](../../src/endpoint_host.rs#L2450) |
| `validate_time_zone` | `Ok` | [2515](../../src/endpoint_host.rs#L2515), [2536](../../src/endpoint_host.rs#L2536) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `value.trim` | [2517](../../src/endpoint_host.rs#L2517) | receiver-type-required |
| `validate_time_zone` | `value.is_ascii` | [2518](../../src/endpoint_host.rs#L2518) | receiver-type-required |
| `validate_time_zone` | `value.split('/').count` | [2519](../../src/endpoint_host.rs#L2519) | receiver-type-required |
| `validate_time_zone` | `value.split` | [2519](../../src/endpoint_host.rs#L2519), [2520](../../src/endpoint_host.rs#L2520) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate().any` | [2520](../../src/endpoint_host.rs#L2520) | receiver-type-required |
| `validate_time_zone` | `value.split('/').enumerate` | [2520](../../src/endpoint_host.rs#L2520) | receiver-type-required |
| `validate_time_zone` | `segment.is_empty` | [2521](../../src/endpoint_host.rs#L2521) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate().any` | [2524](../../src/endpoint_host.rs#L2524) | receiver-type-required |
| `validate_time_zone` | `segment.bytes().enumerate` | [2524](../../src/endpoint_host.rs#L2524) | receiver-type-required |
| `validate_time_zone` | `segment.bytes` | [2524](../../src/endpoint_host.rs#L2524) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphanumeric` | [2525](../../src/endpoint_host.rs#L2525) | receiver-type-required |
| `validate_time_zone` | `byte.is_ascii_alphabetic` | [2526](../../src/endpoint_host.rs#L2526) | receiver-type-required |
| `validate_time_zone` | `Err` | [2530](../../src/endpoint_host.rs#L2530), [2534](../../src/endpoint_host.rs#L2534) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `invalid` | [2530](../../src/endpoint_host.rs#L2530), [2534](../../src/endpoint_host.rs#L2534) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `validate_time_zone` | `Path::new("/usr/share/zoneinfo").join` | [2532](../../src/endpoint_host.rs#L2532) | receiver-type-required |
| `validate_time_zone` | `Path::new` | [2532](../../src/endpoint_host.rs#L2532) | external-constructor-callback-or-unresolved |
| `validate_time_zone` | `zone.is_file` | [2533](../../src/endpoint_host.rs#L2533) | receiver-type-required |
| `map_prompt_error` | `invalid` | [2543](../../src/endpoint_host.rs#L2543) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `map_prompt_error` | `attachment_failure` | [2544](../../src/endpoint_host.rs#L2544) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2576) |
| `map_prompt_error` | `EndpointRpcFailure::new` | [2545](../../src/endpoint_host.rs#L2545), [2550](../../src/endpoint_host.rs#L2550) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `map_prompt_error` | `internal_failure` | [2555](../../src/endpoint_host.rs#L2555) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `map_attachment_read_error` | `EndpointRpcFailure::new` | [2561](../../src/endpoint_host.rs#L2561), [2566](../../src/endpoint_host.rs#L2566) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `map_attachment_read_error` | `attachment_failure` | [2571](../../src/endpoint_host.rs#L2571) | [tekes-supervisor::endpoint_host::attachment_failure](../../src/endpoint_host.rs#L2576) |
| `map_attachment_read_error` | `internal_failure` | [2572](../../src/endpoint_host.rs#L2572) | [tekes-supervisor::endpoint_host::internal_failure](../../src/endpoint_host.rs#L2222) |
| `attachment_failure` | `EndpointRpcFailure::new` | [2577](../../src/endpoint_host.rs#L2577) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `validate_extension_payload` | `Err` | [2638](../../src/endpoint_host.rs#L2638) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `EndpointRpcFailure::new` | [2638](../../src/endpoint_host.rs#L2638) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `validate_extension_payload` | `require_object_fields` | [2645](../../src/endpoint_host.rs#L2645) | [tekes-supervisor::endpoint_host::require_object_fields](../../src/endpoint_host.rs#L2279) |
| `validate_extension_payload` | `serde_json::from_value(payload["execution"].clone())                 .map_err` | [2648](../../src/endpoint_host.rs#L2648) | receiver-type-required |
| `validate_extension_payload` | `serde_json::from_value` | [2648](../../src/endpoint_host.rs#L2648) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `payload["execution"].clone` | [2648](../../src/endpoint_host.rs#L2648) | receiver-type-required |
| `validate_extension_payload` | `invalid` | [2649](../../src/endpoint_host.rs#L2649) | [tekes-supervisor::endpoint_host::invalid](../../src/endpoint_host.rs#L2659) |
| `validate_extension_payload` | `execution.validate().map_err` | [2650](../../src/endpoint_host.rs#L2650) | receiver-type-required |
| `validate_extension_payload` | `execution.validate` | [2650](../../src/endpoint_host.rs#L2650) | receiver-type-required |
| `validate_extension_payload` | `Ok` | [2652](../../src/endpoint_host.rs#L2652) | external-constructor-callback-or-unresolved |
| `validate_production_payload` | `validate_extension_payload` | [2656](../../src/endpoint_host.rs#L2656) | [tekes-supervisor::endpoint_host::validate_extension_payload](../../src/endpoint_host.rs#L2599) |
| `invalid` | `EndpointRpcFailure::new` | [2660](../../src/endpoint_host.rs#L2660) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `map_endpoint` | `session.unwrap_or_default` | [2664](../../src/endpoint_host.rs#L2664) | receiver-type-required |
| `map_endpoint` | `EndpointRpcFailure::new` | [2666](../../src/endpoint_host.rs#L2666), [2671](../../src/endpoint_host.rs#L2671), [2676](../../src/endpoint_host.rs#L2676) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `map_management` | `EndpointRpcFailure::new` | [2683](../../src/endpoint_host.rs#L2683), [2685](../../src/endpoint_host.rs#L2685), [2691](../../src/endpoint_host.rs#L2691), [2697](../../src/endpoint_host.rs#L2697), [2702](../../src/endpoint_host.rs#L2702), [2707](../../src/endpoint_host.rs#L2707), [2712](../../src/endpoint_host.rs#L2712), [2717](../../src/endpoint_host.rs#L2717), [2722](../../src/endpoint_host.rs#L2722), [2739](../../src/endpoint_host.rs#L2739), [2745](../../src/endpoint_host.rs#L2745), [2751](../../src/endpoint_host.rs#L2751), [2754](../../src/endpoint_host.rs#L2754), [2756](../../src/endpoint_host.rs#L2756) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `map_management` | `invalid.detail` | [2688](../../src/endpoint_host.rs#L2688) | receiver-type-required |
| `map_management` | `Value::String` | [2737](../../src/endpoint_host.rs#L2737) | external-constructor-callback-or-unresolved |
| `internal_json` | `EndpointRpcFailure::new` | [2761](../../src/endpoint_host.rs#L2761) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `internal_schema` | `EndpointRpcFailure::new` | [2765](../../src/endpoint_host.rs#L2765) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `internal_assembly` | `EndpointRpcFailure::new` | [2769](../../src/endpoint_host.rs#L2769) | [tekes-supervisor::endpoint_host::EndpointRpcFailure::new](../../src/endpoint_host.rs#L2779) |
| `new` | `IJsonValue::parse(&serde_json::to_vec(&details).unwrap_or_else(&#124;_&#124; b"{}".to_vec()))                 .unwrap_or_else` | [2781](../../src/endpoint_host.rs#L2781) | receiver-type-required |
| `new` | `IJsonValue::parse` | [2781](../../src/endpoint_host.rs#L2781) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `new` | `serde_json::to_vec(&details).unwrap_or_else` | [2781](../../src/endpoint_host.rs#L2781) | receiver-type-required |
| `new` | `serde_json::to_vec` | [2781](../../src/endpoint_host.rs#L2781) | external-constructor-callback-or-unresolved |
| `new` | `b"{}".to_vec` | [2781](../../src/endpoint_host.rs#L2781) | receiver-type-required |
| `new` | `IJsonValue::parse_str("{}").expect` | [2782](../../src/endpoint_host.rs#L2782) | receiver-type-required |
| `new` | `IJsonValue::parse_str` | [2782](../../src/endpoint_host.rs#L2782) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `new` | `code.to_owned` | [2784](../../src/endpoint_host.rs#L2784) | receiver-type-required |
| `new` | `message.to_owned` | [2785](../../src/endpoint_host.rs#L2785) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; EndpointAssemblyError::Clock(error.to_string()))?         .as_secs` | [2792](../../src/endpoint_host.rs#L2792) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [2792](../../src/endpoint_host.rs#L2792) | receiver-type-required |
| `system_timestamp` | `SystemTime::now()         .duration_since` | [2792](../../src/endpoint_host.rs#L2792) | receiver-type-required |
| `system_timestamp` | `SystemTime::now` | [2792](../../src/endpoint_host.rs#L2792) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `EndpointAssemblyError::Clock` | [2794](../../src/endpoint_host.rs#L2794), [2797](../../src/endpoint_host.rs#L2797) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `error.to_string` | [2794](../../src/endpoint_host.rs#L2794) | receiver-type-required |
| `system_timestamp` | `i64::try_from(seconds / 86_400)         .map_err` | [2796](../../src/endpoint_host.rs#L2796) | receiver-type-required |
| `system_timestamp` | `i64::try_from` | [2796](../../src/endpoint_host.rs#L2796) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `"timestamp exceeds i64".to_owned` | [2797](../../src/endpoint_host.rs#L2797) | receiver-type-required |
| `system_timestamp` | `civil_from_days` | [2799](../../src/endpoint_host.rs#L2799) | [tekes-supervisor::endpoint_host::civil_from_days](../../src/endpoint_host.rs#L2808) |
| `system_timestamp` | `Ok` | [2803](../../src/endpoint_host.rs#L2803) | external-constructor-callback-or-unresolved |
| `civil_from_days` | `i64::from` | [2819](../../src/endpoint_host.rs#L2819) | external-constructor-callback-or-unresolved |
| `new` | `AtomicBool::new` | [2853](../../src/endpoint_host.rs#L2853) | external-constructor-callback-or-unresolved |
| `new` | `AtomicUsize::new` | [2854](../../src/endpoint_host.rs#L2854) | external-constructor-callback-or-unresolved |
| `full_sync` | `self.calls.fetch_add` | [2861](../../src/endpoint_host.rs#L2861) | receiver-type-required |
| `full_sync` | `self.fail.load` | [2862](../../src/endpoint_host.rs#L2862) | receiver-type-required |
| `full_sync` | `Err` | [2863](../../src/endpoint_host.rs#L2863) | external-constructor-callback-or-unresolved |
| `full_sync` | `std::io::Error::from_raw_os_error` | [2863](../../src/endpoint_host.rs#L2863) | external-constructor-callback-or-unresolved |
| `full_sync` | `store::FullSync::full_sync` | [2865](../../src/endpoint_host.rs#L2865) | [store::platform::FullSync::full_sync](../../../store/src/platform.rs#L31) |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&genesis).unwrap())             .unwrap()             .canonical_bytes()             .unwrap` | [2901](../../src/endpoint_host.rs#L2901) | receiver-type-required |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&genesis).unwrap())             .unwrap()             .canonical_bytes` | [2901](../../src/endpoint_host.rs#L2901) | receiver-type-required |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&genesis).unwrap())             .unwrap` | [2901](../../src/endpoint_host.rs#L2901) | receiver-type-required |
| `write_test_ledger` | `schema::Event::decode` | [2901](../../src/endpoint_host.rs#L2901), [2905](../../src/endpoint_host.rs#L2905) | [schema::event::Event::decode](../../../schema/src/event.rs#L166) |
| `write_test_ledger` | `serde_json::to_vec(&genesis).unwrap` | [2901](../../src/endpoint_host.rs#L2901) | receiver-type-required |
| `write_test_ledger` | `serde_json::to_vec` | [2901](../../src/endpoint_host.rs#L2901), [2905](../../src/endpoint_host.rs#L2905) | external-constructor-callback-or-unresolved |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&input).unwrap())             .unwrap()             .canonical_bytes()             .unwrap` | [2905](../../src/endpoint_host.rs#L2905) | receiver-type-required |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&input).unwrap())             .unwrap()             .canonical_bytes` | [2905](../../src/endpoint_host.rs#L2905) | receiver-type-required |
| `write_test_ledger` | `schema::Event::decode(&serde_json::to_vec(&input).unwrap())             .unwrap` | [2905](../../src/endpoint_host.rs#L2905) | receiver-type-required |
| `write_test_ledger` | `serde_json::to_vec(&input).unwrap` | [2905](../../src/endpoint_host.rs#L2905) | receiver-type-required |
| `write_test_ledger` | `bytes.push` | [2910](../../src/endpoint_host.rs#L2910), [2912](../../src/endpoint_host.rs#L2912) | receiver-type-required |
| `write_test_ledger` | `bytes.extend_from_slice` | [2911](../../src/endpoint_host.rs#L2911) | receiver-type-required |
| `write_test_ledger` | `std::fs::write(main_path, bytes).unwrap` | [2913](../../src/endpoint_host.rs#L2913) | receiver-type-required |
| `write_test_ledger` | `std::fs::write` | [2913](../../src/endpoint_host.rs#L2913) | external-constructor-callback-or-unresolved |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `tempfile::tempdir().expect` | [2918](../../src/endpoint_host.rs#L2918) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `tempfile::tempdir` | [2918](../../src/endpoint_host.rs#L2918) | external-constructor-callback-or-unresolved |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `root.path().join("threads").join` | [2920](../../src/endpoint_host.rs#L2920) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `root.path().join` | [2920](../../src/endpoint_host.rs#L2920) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `root.path` | [2920](../../src/endpoint_host.rs#L2920) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `std::fs::create_dir_all(&folder).expect` | [2921](../../src/endpoint_host.rs#L2921) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `std::fs::create_dir_all` | [2921](../../src/endpoint_host.rs#L2921) | external-constructor-callback-or-unresolved |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `folder.join` | [2922](../../src/endpoint_host.rs#L2922) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `"uid:501".to_owned` | [2924](../../src/endpoint_host.rs#L2924) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `endpoint::ORIGIN_CLIENT.to_owned` | [2925](../../src/endpoint_host.rs#L2925) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `session.to_owned` | [2926](../../src/endpoint_host.rs#L2926) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `"session.prompt".to_owned` | [2927](../../src/endpoint_host.rs#L2927) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `"rpc-sync-barrier".to_owned` | [2928](../../src/endpoint_host.rs#L2928) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `write_test_ledger` | [2930](../../src/endpoint_host.rs#L2930) | [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::write_test_ledger](../../src/endpoint_host.rs#L2869) |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `std::fs::read(&main_path).expect` | [2931](../../src/endpoint_host.rs#L2931) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `std::fs::read` | [2931](../../src/endpoint_host.rs#L2931) | external-constructor-callback-or-unresolved |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `Arc::new` | [2936](../../src/endpoint_host.rs#L2936) | external-constructor-callback-or-unresolved |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `ToggleSync::new` | [2936](../../src/endpoint_host.rs#L2936) | [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::ToggleSync::new](../../src/endpoint_host.rs#L2851) |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `sync.fail.store` | [2953](../../src/endpoint_host.rs#L2953), [2972](../../src/endpoint_host.rs#L2972), [2986](../../src/endpoint_host.rs#L2986) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `store::LockedLedger::open(&main_path, schema::READER_VERSION).expect` | [2971](../../src/endpoint_host.rs#L2971) | receiver-type-required |
| `legacy_reconcile_requires_sync_barrier_and_recovers_after_eio_clears` | `store::LockedLedger::open` | [2971](../../src/endpoint_host.rs#L2971) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L186) |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `tempfile::tempdir().expect` | [3005](../../src/endpoint_host.rs#L3005) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `tempfile::tempdir` | [3005](../../src/endpoint_host.rs#L3005) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `root.path().join("threads").join` | [3007](../../src/endpoint_host.rs#L3007) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `root.path().join` | [3007](../../src/endpoint_host.rs#L3007) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `root.path` | [3007](../../src/endpoint_host.rs#L3007) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::create_dir_all(&folder).expect` | [3008](../../src/endpoint_host.rs#L3008) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::create_dir_all` | [3008](../../src/endpoint_host.rs#L3008) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `folder.join` | [3009](../../src/endpoint_host.rs#L3009) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `"uid:501".to_owned` | [3011](../../src/endpoint_host.rs#L3011) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `endpoint::ORIGIN_CLIENT.to_owned` | [3012](../../src/endpoint_host.rs#L3012) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `session.to_owned` | [3013](../../src/endpoint_host.rs#L3013) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `"session.prompt".to_owned` | [3014](../../src/endpoint_host.rs#L3014) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `"rpc-readonly-tail".to_owned` | [3015](../../src/endpoint_host.rs#L3015) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `write_test_ledger` | [3017](../../src/endpoint_host.rs#L3017) | [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::write_test_ledger](../../src/endpoint_host.rs#L2869) |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::metadata(&main_path).expect("valid meta").len` | [3018](../../src/endpoint_host.rs#L3018) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::metadata(&main_path).expect` | [3018](../../src/endpoint_host.rs#L3018) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::metadata` | [3018](../../src/endpoint_host.rs#L3018) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::OpenOptions::new()                 .append(true)                 .open(&main_path)                 .expect` | [3024](../../src/endpoint_host.rs#L3024) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::OpenOptions::new()                 .append(true)                 .open` | [3024](../../src/endpoint_host.rs#L3024) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::OpenOptions::new()                 .append` | [3024](../../src/endpoint_host.rs#L3024) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::OpenOptions::new` | [3024](../../src/endpoint_host.rs#L3024) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `file.write_all(torn_tail).expect` | [3028](../../src/endpoint_host.rs#L3028) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `file.write_all` | [3028](../../src/endpoint_host.rs#L3028) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `file.sync_all().expect` | [3029](../../src/endpoint_host.rs#L3029) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `file.sync_all` | [3029](../../src/endpoint_host.rs#L3029) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::read(&main_path).expect` | [3031](../../src/endpoint_host.rs#L3031) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `std::fs::read` | [3031](../../src/endpoint_host.rs#L3031) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `Sha256::digest` | [3032](../../src/endpoint_host.rs#L3032) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `Arc::new` | [3038](../../src/endpoint_host.rs#L3038) | external-constructor-callback-or-unresolved |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `ToggleSync::new` | [3038](../../src/endpoint_host.rs#L3038) | [tekes-supervisor::endpoint_host::legacy_reconcile_sync_tests::ToggleSync::new](../../src/endpoint_host.rs#L2851) |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `sync.fail.store` | [3076](../../src/endpoint_host.rs#L3076) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `store::LockedLedger::open(&main_path, schema::READER_VERSION).expect` | [3097](../../src/endpoint_host.rs#L3097) | receiver-type-required |
| `verify_and_sync_legacy_input_preserves_torn_tail_across_all_outcomes` | `store::LockedLedger::open` | [3097](../../src/endpoint_host.rs#L3097) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L186) |
