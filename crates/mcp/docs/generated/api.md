# mcp — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [mcp::broker::McpBroker](../../src/broker.rs#L13) | `pub` | not a function |
| [mcp::broker::McpBrokerHandle](../../src/broker.rs#L20) | `pub` | not a function |
| [mcp::broker::McpBroker::start](../../src/broker.rs#L75) | `pub` | [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::slice13_gates::slice13_gate_92_closed_recovery_matrix](../../tests/slice13_gates.rs#L1643); [mcp::tests::slice13_gates::broker_task_operations_use_existing_peer_and_exact_task_identity](../../tests/slice13_gates.rs#L1940); [mcp::tests::slice13_gates::task_required_tools_are_declared_and_called_with_task_augmentation](../../tests/slice13_gates.rs#L2042); [mcp::tests::slice13_gates::cancellation_notification_failure_is_bounded_and_evicts_the_peer_generation](../../tests/slice13_gates.rs#L489); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L693); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::McpRuntime::open_with_authorities](../../../supervisor/src/mcp_runtime.rs#L258) |
| [mcp::broker::McpBroker::handle](../../src/broker.rs#L91) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::list_resources](../../src/broker.rs#L106) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::read_resource](../../src/broker.rs#L113) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::retain_snapshot](../../src/broker.rs#L125) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::register](../../src/broker.rs#L133) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool](../../src/broker.rs#L151) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool_augmented](../../src/broker.rs#L175) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool_with_context](../../src/broker.rs#L199) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::task_operation](../../src/broker.rs#L222) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::task_operation_cancellable](../../src/broker.rs#L231) | `pub` | [mcp::broker::McpBrokerHandle::task_operation](../../src/broker.rs#L222) |
| [mcp::broker::McpBrokerHandle::remove](../../src/broker.rs#L258) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::release](../../src/broker.rs#L266) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::catalog_generation](../../src/broker.rs#L274) | `pub` | no resolved direct caller |
| [mcp::client::McpPeerFuture](../../src/client.rs#L20) | `pub` | not a function |
| [mcp::client::McpPeer](../../src/client.rs#L22) | `pub` | not a function |
| [mcp::client::McpClient](../../src/client.rs#L132) | `pub` | not a function |
| [mcp::client::McpClient::new](../../src/client.rs#L149) | `pub` | [mcp::tests::handshake_lifecycle::cancellation_at_each_handshake_stage_closes_once_before_return](../../tests/handshake_lifecycle.rs#L139); [mcp::tests::handshake_lifecycle::pre_cancelled_handshake_closes_without_sending](../../tests/handshake_lifecycle.rs#L182); [mcp::tests::handshake_lifecycle::every_terminal_handshake_failure_closes_exactly_once](../../tests/handshake_lifecycle.rs#L61); [mcp::tests::http_discovery::client_owned_subscription_updates_generation_and_closes_with_peer](../../tests/http_discovery.rs#L1126); [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::task_cancellation_distinguishes_read_from_dispatched_mutation](../../tests/http_discovery.rs#L1334); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::http_discovery::probe](../../tests/http_discovery.rs#L349); [mcp::tests::http_discovery::legacy_expired_session_reinitializes_and_close_deletes_once](../../tests/http_discovery.rs#L545); [mcp::tests::http_discovery::http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once](../../tests/http_discovery.rs#L671); [mcp::tests::http_discovery::modern_scoped_http_calls_overlap_and_cancel_only_their_own_socket](../../tests/http_discovery.rs#L70); [mcp::tests::http_discovery::legacy_http_connects_lists_and_calls_tool_over_real_socket](../../tests/http_discovery.rs#L741); [mcp::tests::http_discovery::modern_http_continuation_preserves_input_and_task_resume_queries_identity](../../tests/http_discovery.rs#L842); [mcp::tests::http_discovery::modern_scoped_call_does_not_claim_known_failure_after_dispatch](../../tests/http_discovery.rs#L8); [mcp::tests::http_discovery::parameter_headers_follow_scoped_arguments_and_catalog_replacement](../../tests/http_discovery.rs#L943); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_forced_sse_round_trip](../../tests/live_official_sdk_tunnel.rs#L166); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::live_services::live_cloudflare_docs_parameter_contract](../../tests/live_services.rs#L122); [mcp::tests::live_services::public_round_trip](../../tests/live_services.rs#L5); [mcp::tests::live_services::live_cloudflare_bearer_catalog_and_search](../../tests/live_services.rs#L62); [mcp::tests::rmcp_interop::connect](../../tests/rmcp_interop.rs#L201); [mcp::tests::slice13_gates::effectful_tool_call_carries_host_owned_idempotency_key_in_request_metadata](../../tests/slice13_gates.rs#L1187); [mcp::tests::slice13_gates::slice13_gate_92_closed_recovery_matrix](../../tests/slice13_gates.rs#L1643); [mcp::tests::slice13_gates::slice13_gate_87_operations_and_cancellation_shapes](../../tests/slice13_gates.rs#L180); [mcp::tests::slice13_gates::fixture_client_with_args](../../tests/slice13_gates.rs#L25); [mcp::tests::slice13_gates::cancellation_notification_failure_is_bounded_and_evicts_the_peer_generation](../../tests/slice13_gates.rs#L489); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L693); [mcp::tests::tasks_extension::connected](../../tests/tasks_extension.rs#L120); [mcp::tests::tasks_extension::the_handshake_declares_the_tasks_extension_for_both_dialects](../../tests/tasks_extension.rs#L157); [mcp::tests::tasks_extension::the_modern_protocol_carries_the_declaration_in_request_metadata](../../tests/tasks_extension.rs#L341); [mcp::tests::tasks_extension::call_with](../../tests/tasks_extension.rs#L384); [mcp::tests::tasks_extension::a_completed_poll_without_a_result_is_invalid_for_an_extension_only_peer](../../tests/tasks_extension.rs#L422); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1951); [tekes-supervisor::mcp_runtime::tests::extension_task_result_binds_continuation_through_a_real_client](../../../supervisor/src/mcp_runtime.rs#L5482) |
| [mcp::client::McpClient::connect](../../src/client.rs#L167) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::connect_cancellable](../../src/client.rs#L175) | `pub` | [mcp::client::McpClient::connect](../../src/client.rs#L167) |
| [mcp::client::McpClient::server](../../src/client.rs#L285) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::supported_versions](../../src/client.rs#L290) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::protocol_version](../../src/client.rs#L294) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::catalog_generation](../../src/client.rs#L299) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::continue_tool_call](../../src/client.rs#L622) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::task_mutation_with_resumable_identity](../../src/client.rs#L720) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::call_tool_scoped](../../src/client.rs#L1446) | `pub` | no resolved direct caller |
| [mcp::client::McpPeerSnapshot](../../src/client.rs#L1461) | `pub` | not a function |
| [mcp::client::McpPeerSnapshot::capture](../../src/client.rs#L1470) | `pub` | [mcp::broker::run](../../src/broker.rs#L311); [tekes-supervisor::mcp_runtime::McpRuntime::prepare_pooled_server](../../../supervisor/src/mcp_runtime.rs#L874) |
| [mcp::LEGACY_PROTOCOL_VERSION](../../src/lib.rs#L39) | `pub` | not a function |
| [mcp::MODERN_PROTOCOL_VERSION](../../src/lib.rs#L40) | `pub` | not a function |
| [mcp::MAX_FRAME_BYTES](../../src/lib.rs#L41) | `pub` | not a function |
| [mcp::MAX_CATALOG_ITEMS](../../src/lib.rs#L42) | `pub` | not a function |
| [mcp::MAX_CATALOG_PAGES](../../src/lib.rs#L43) | `pub` | not a function |
| [mcp::MAX_CONTINUATION_ROUNDS](../../src/lib.rs#L44) | `pub` | not a function |
| [mcp::management::McpScope](../../src/management.rs#L19) | `pub` | not a function |
| [mcp::management::McpServerReference](../../src/management.rs#L27) | `pub` | not a function |
| [mcp::management::CredentialValue](../../src/management.rs#L35) | `pub` | not a function |
| [mcp::management::McpTransportConfig](../../src/management.rs#L77) | `pub` | not a function |
| [mcp::management::McpServerConfig](../../src/management.rs#L94) | `pub` | not a function |
| [mcp::management::McpRegistry](../../src/management.rs#L113) | `pub` | not a function |
| [mcp::management::McpRegistry::validate](../../src/management.rs#L128) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistry::resolve](../../src/management.rs#L154) | `pub` | no resolved direct caller |
| [mcp::management::McpCredentialReferenceState](../../src/management.rs#L189) | `pub` | not a function |
| [mcp::management::McpCredentialReference](../../src/management.rs#L196) | `pub` | not a function |
| [mcp::management::McpManagementMutation](../../src/management.rs#L305) | `pub` | not a function |
| [mcp::management::McpCredentialFieldOperation](../../src/management.rs#L332) | `pub` | not a function |
| [mcp::management::McpManagementResult](../../src/management.rs#L340) | `pub` | not a function |
| [mcp::management::McpMutationReceipt](../../src/management.rs#L362) | `pub` | not a function |
| [mcp::management::McpManagementFault](../../src/management.rs#L372) | `pub` | not a function |
| [mcp::management::McpRegistryStore](../../src/management.rs#L382) | `pub` | not a function |
| [mcp::management::McpRegistryStore::root](../../src/management.rs#L388) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::new](../../src/management.rs#L391) | `pub` | [mcp::tests::slice13_gates::slice13_gate_91_management_publication_and_recovery](../../tests/slice13_gates.rs#L1291); [mcp::tests::slice13_gates::slice13_gate_91_management_faults_recover_registry_and_credential_receipt](../../tests/slice13_gates.rs#L1340); [mcp::tests::slice13_gates::slice13_gate_91_credential_field_operations_preserve_replace_and_remove_references](../../tests/slice13_gates.rs#L1424); [mcp::tests::slice13_gates::slice13_gate_91_oauth_pending_to_bound_transition_is_exact](../../tests/slice13_gates.rs#L1512); [mcp::tests::slice13_gates::registry_file_keeps_the_last_entry_for_a_repeated_reference_and_saves_it_once](../../tests/slice13_gates.rs#L2139); [mcp::tests::slice13_gates::registry_file_tolerates_missing_trailing_newline_and_names_the_file_on_corruption](../../tests/slice13_gates.rs#L2204); [mcp::tests::slice13_gates::conflicting_active_user_alias_duplicates_are_not_silently_overwritten](../../tests/slice13_gates.rs#L2434); [tekes-supervisor::mcp_runtime::McpRuntime::open_with_authorities](../../../supervisor/src/mcp_runtime.rs#L258); [tekes-supervisor::process_live_tests::register_local_tools_server](../../../supervisor/src/process_live_tests.rs#L1076); [tekes-supervisor::process_live_tests::real_public_mcp_task_continuation](../../../supervisor/src/process_live_tests.rs#L1468); [tekes-supervisor::tests::slice13_management::slice13_gate_89_pending_oauth_requires_exact_bound_ownership](../../../supervisor/tests/slice13_management.rs#L444) |
| [mcp::management::McpRegistryStore::injecting_fault](../../src/management.rs#L399) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::load](../../src/management.rs#L404) | `pub` | [mcp::management::McpRegistryStore::list](../../src/management.rs#L416); [mcp::management::McpRegistryStore::get](../../src/management.rs#L425) |
| [mcp::management::McpRegistryStore::publish](../../src/management.rs#L408) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::list](../../src/management.rs#L416) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::get](../../src/management.rs#L425) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::credential_reference_state](../../src/management.rs#L436) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::committed_receipt](../../src/management.rs#L459) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::mutate_idempotent](../../src/management.rs#L483) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::recover](../../src/management.rs#L533) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryError](../../src/management.rs#L1215) | `pub` | not a function |
| [mcp::parameter_headers::ParameterHeaders](../../src/parameter_headers.rs#L13) | `pub(crate)` | not a function |
| [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) | `pub(crate)` | [mcp::transport::McpTransport::install_tool_catalog](../../src/transport.rs#L41); [mcp::transport::HttpTransport::install_tool_catalog](../../src/transport.rs#L944) |
| [mcp::parameter_headers::ParameterHeaders::is_empty](../../src/parameter_headers.rs#L127) | `pub(crate)` | no resolved direct caller |
| [mcp::parameter_headers::ParameterHeaders::project](../../src/parameter_headers.rs#L130) | `pub(crate)` | no resolved direct caller |
| [mcp::pool::McpPoolKey](../../src/pool.rs#L10) | `pub` | not a function |
| [mcp::pool::PooledPeer](../../src/pool.rs#L20) | `pub` | not a function |
| [mcp::pool::McpPool](../../src/pool.rs#L29) | `pub` | not a function |
| [mcp::pool::McpPoolLease](../../src/pool.rs#L39) | `pub` | not a function |
| [mcp::pool::McpPoolLease::peer](../../src/pool.rs#L48) | `pub` | no resolved direct caller |
| [mcp::pool::McpPoolLease::release](../../src/pool.rs#L52) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::new](../../src/pool.rs#L75) | `pub` | [mcp::broker::run](../../src/broker.rs#L311); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L693) |
| [mcp::pool::McpPool::get](../../src/pool.rs#L79) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::retain](../../src/pool.rs#L87) | `pub(crate)` | [mcp::pool::McpPool::acquire_or_create](../../src/pool.rs#L94) |
| [mcp::pool::McpPool::acquire_or_create](../../src/pool.rs#L94) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::reap_revoked](../../src/pool.rs#L175) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::insert](../../src/pool.rs#L194) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::replace_generation](../../src/pool.rs#L224) | `pub` | [mcp::pool::McpPool::acquire_or_create](../../src/pool.rs#L94) |
| [mcp::pool::McpPool::remove](../../src/pool.rs#L284) | `pub` | [mcp::pool::McpPool::reap_revoked](../../src/pool.rs#L175) |
| [mcp::pool::McpPool::release](../../src/pool.rs#L295) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::drain](../../src/pool.rs#L314) | `pub` | no resolved direct caller |
| [mcp::pool::McpPoolError](../../src/pool.rs#L330) | `pub` | not a function |
| [mcp::projection::project_name](../../src/projection.rs#L14) | `pub` | [mcp::projection::project_tool](../../src/projection.rs#L189); [tekes-supervisor::mcp_runtime::McpRuntime::tool_name](../../../supervisor/src/mcp_runtime.rs#L3057); [tekes-supervisor::mcp_runtime::McpRuntime::preset_readiness](../../../supervisor/src/mcp_runtime.rs#L356); [tekes-supervisor::mcp_runtime::McpRuntime::bind_prepared_server](../../../supervisor/src/mcp_runtime.rs#L941); [tekes-supervisor::process_host::mcp_failure_is_required](../../../supervisor/src/process_host.rs#L4785) |
| [mcp::projection::project_catalog](../../src/projection.rs#L28) | `pub` | [engine::tests::slice8_dynamic_catalog::mcp_destructive_hint_reaches_the_engine_destructive_approval_gate](../../../engine/tests/slice8_dynamic_catalog.rs#L130); [mcp::tests::slice13_gates::slice13_gate_90_catalog_projection_and_collision](../../tests/slice13_gates.rs#L1010); [mcp::tests::slice13_gates::pydantic_generated_catalog_projects_every_tool](../../tests/slice13_gates.rs#L1071); [mcp::tests::slice13_gates::external_effect_metadata_projects_only_with_authoritative_read_only_query](../../tests/slice13_gates.rs#L1132); [mcp::tests::slice13_gates::partial_projection_excludes_only_the_refused_tool_and_locates_the_cause](../../tests/slice13_gates.rs#L2271) |
| [mcp::projection::MAX_TOOL_REFUSAL_REASON_BYTES](../../src/projection.rs#L42) | `pub` | not a function |
| [mcp::projection::MAX_TOOL_REFUSAL_NAME_BYTES](../../src/projection.rs#L44) | `pub` | not a function |
| [mcp::projection::refusal_code::SCHEMA_REFUSED](../../src/projection.rs#L50) | `pub` | not a function |
| [mcp::projection::refusal_code::EXTERNAL_EFFECT_REFUSED](../../src/projection.rs#L52) | `pub` | not a function |
| [mcp::projection::refusal_code::DEPENDENCY_REFUSED](../../src/projection.rs#L54) | `pub` | not a function |
| [mcp::projection::McpToolRefusal](../../src/projection.rs#L60) | `pub` | not a function |
| [mcp::projection::PartialProjection](../../src/projection.rs#L100) | `pub` | not a function |
| [mcp::projection::project_catalog_partial](../../src/projection.rs#L115) | `pub` | [mcp::projection::project_catalog](../../src/projection.rs#L28); [mcp::tests::slice13_gates::partial_projection_excludes_only_the_refused_tool_and_locates_the_cause](../../tests/slice13_gates.rs#L2271); [mcp::tests::slice13_gates::partial_projection_reports_are_bounded_and_server_level_causes_stay_errors](../../tests/slice13_gates.rs#L2314); [mcp::tests::slice13_gates::partial_projection_refuses_an_effectful_tool_whose_reconcile_tool_was_refused](../../tests/slice13_gates.rs#L2348); [mcp::tests::slice13_gates::refusal_names_are_reported_only_when_the_launch_binding_accepts_them](../../tests/slice13_gates.rs#L2409); [tekes-supervisor::mcp_runtime::McpDynamicSupervisorAuthority::ensure_route_peer](../../../supervisor/src/mcp_runtime.rs#L2742); [tekes-supervisor::mcp_runtime::McpRuntime::bind_prepared_server](../../../supervisor/src/mcp_runtime.rs#L941) |
| [mcp::projection::McpProjectionError](../../src/projection.rs#L387) | `pub` | not a function |
| [mcp::recovery::McpLossState](../../src/recovery.rs#L2) | `pub` | not a function |
| [mcp::recovery::RecoveryAction](../../src/recovery.rs#L14) | `pub` | not a function |
| [mcp::recovery::recovery_action](../../src/recovery.rs#L23) | `pub` | [mcp::client::should_reconnect](../../src/client.rs#L1355); [mcp::client::mutation_error](../../src/client.rs#L1368); [mcp::client::McpClient::task_mutation_with_resumable_identity](../../src/client.rs#L720) |
| [mcp::subscription::McpSubscriptionFilter](../../src/subscription.rs#L7) | `pub` | not a function |
| [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) | `pub` | [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L643); [mcp::transport::HttpTransport::start_catalog_subscription](../../src/transport.rs#L925) |
| [mcp::subscription::McpSubscriptionFilter::request_params](../../src/subscription.rs#L48) | `pub` | no resolved direct caller |
| [mcp::subscription::McpSubscriptionFilter::is_empty](../../src/subscription.rs#L51) | `pub` | no resolved direct caller |
| [mcp::subscription::McpSubscriptionFilter::accepts](../../src/subscription.rs#L56) | `pub` | no resolved direct caller |
| [mcp::transport::McpMethodObserver](../../src/transport.rs#L15) | `pub` | not a function |
| [mcp::transport::McpValueObserver](../../src/transport.rs#L17) | `pub` | not a function |
| [mcp::transport::McpTransport](../../src/transport.rs#L28) | `pub` | not a function |
| [mcp::transport::StdioTransport](../../src/transport.rs#L86) | `pub` | not a function |
| [mcp::transport::StdioTransport::spawn](../../src/transport.rs#L137) | `pub` | [mcp::tests::rmcp_interop::connect](../../tests/rmcp_interop.rs#L201); [mcp::tests::slice13_gates::fixture_client_with_args](../../tests/slice13_gates.rs#L25); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1951); [tekes-supervisor::mcp_runtime::tests::extension_task_result_binds_continuation_through_a_real_client](../../../supervisor/src/mcp_runtime.rs#L5482) |
| [mcp::transport::HttpTransport](../../src/transport.rs#L500) | `pub` | not a function |
| [mcp::transport::HttpRequestAuthorization](../../src/transport.rs#L527) | `pub` | not a function |
| [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L547) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L557) | `pub(crate)` | [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L547) |
| [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L570) | `pub(crate)` | [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L557) |
| [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L643) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::with_notification_handler](../../src/transport.rs#L687) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::new](../../src/transport.rs#L694) | `pub` | [mcp::tests::http_discovery::dedicated_subscription_delivers_correlated_change_before_cancel_closes_socket](../../tests/http_discovery.rs#L1040); [mcp::tests::http_discovery::client_owned_subscription_updates_generation_and_closes_with_peer](../../tests/http_discovery.rs#L1126); [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::task_cancellation_distinguishes_read_from_dispatched_mutation](../../tests/http_discovery.rs#L1334); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::http_discovery::sse_progress_is_observed_before_server_releases_final_response](../../tests/http_discovery.rs#L302); [mcp::tests::http_discovery::probe](../../tests/http_discovery.rs#L349); [mcp::tests::http_discovery::legacy_expired_session_reinitializes_and_close_deletes_once](../../tests/http_discovery.rs#L545); [mcp::tests::http_discovery::http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once](../../tests/http_discovery.rs#L671); [mcp::tests::http_discovery::modern_scoped_http_calls_overlap_and_cancel_only_their_own_socket](../../tests/http_discovery.rs#L70); [mcp::tests::http_discovery::legacy_http_connects_lists_and_calls_tool_over_real_socket](../../tests/http_discovery.rs#L741); [mcp::tests::http_discovery::modern_http_continuation_preserves_input_and_task_resume_queries_identity](../../tests/http_discovery.rs#L842); [mcp::tests::http_discovery::modern_scoped_call_does_not_claim_known_failure_after_dispatch](../../tests/http_discovery.rs#L8); [mcp::tests::http_discovery::parameter_headers_follow_scoped_arguments_and_catalog_replacement](../../tests/http_discovery.rs#L943); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_forced_sse_round_trip](../../tests/live_official_sdk_tunnel.rs#L166); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::live_services::live_cloudflare_docs_parameter_contract](../../tests/live_services.rs#L122); [mcp::tests::live_services::public_round_trip](../../tests/live_services.rs#L5); [mcp::tests::live_services::live_cloudflare_bearer_catalog_and_search](../../tests/live_services.rs#L62); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L802) |
| [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L719) | `pub` | [mcp::transport::HttpTransport::new](../../src/transport.rs#L694); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L802) |
| [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L743) | `pub` | [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L719); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1951) |
| [mcp::types::McpCancellationToken](../../src/types.rs#L11) | `pub` | not a function |
| [mcp::types::McpCancellationToken::cancel](../../src/types.rs#L14) | `pub` | no resolved direct caller |
| [mcp::types::McpCancellationToken::is_cancelled](../../src/types.rs#L19) | `pub` | [mcp::types::McpCancellationToken::cancelled](../../src/types.rs#L23) |
| [mcp::types::McpCancellationToken::cancelled](../../src/types.rs#L23) | `pub` | no resolved direct caller |
| [mcp::types::ProtocolMode](../../src/types.rs#L32) | `pub` | not a function |
| [mcp::types::TransportKind](../../src/types.rs#L40) | `pub` | not a function |
| [mcp::types::JsonRpcRequest](../../src/types.rs#L47) | `pub` | not a function |
| [mcp::types::JsonRpcRequest::call](../../src/types.rs#L57) | `pub` | [mcp::client::McpClient::call_raw_once](../../src/client.rs#L398); [mcp::client::McpClient::call_raw_cancellable](../../src/client.rs#L440); [mcp::client::McpClient::start_scoped_tool](../../src/client.rs#L771); [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L643); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L802) |
| [mcp::types::JsonRpcRequest::notification](../../src/types.rs#L66) | `pub` | [mcp::client::McpClient::connect_legacy](../../src/client.rs#L304); [mcp::client::McpClient::call_raw_cancellable](../../src/client.rs#L440); [mcp::transport::HttpTransport::close](../../src/transport.rs#L1151) |
| [mcp::types::JsonRpcRequest::canonical_line](../../src/types.rs#L75) | `pub` | no resolved direct caller |
| [mcp::types::JsonRpcError](../../src/types.rs#L88) | `pub` | not a function |
| [mcp::types::JsonRpcResponse](../../src/types.rs#L97) | `pub` | not a function |
| [mcp::types::JsonRpcResponse::validate_for](../../src/types.rs#L107) | `pub` | no resolved direct caller |
| [mcp::types::JsonRpcResponse::into_result](../../src/types.rs#L121) | `pub` | no resolved direct caller |
| [mcp::types::McpImplementation](../../src/types.rs#L135) | `pub` | not a function |
| [mcp::types::McpIcon](../../src/types.rs#L149) | `pub` | not a function |
| [mcp::types::McpIconTheme](../../src/types.rs#L161) | `pub` | not a function |
| [mcp::types::McpCapabilities](../../src/types.rs#L298) | `pub` | not a function |
| [mcp::types::TASKS_EXTENSION](../../src/types.rs#L312) | `pub` | not a function |
| [mcp::types::McpCapabilities::supports_tasks](../../src/types.rs#L317) | `pub` | no resolved direct caller |
| [mcp::types::McpCapabilities::supports_task_extension](../../src/types.rs#L322) | `pub` | [mcp::types::McpCapabilities::supports_tasks](../../src/types.rs#L317) |
| [mcp::types::McpToolAnnotations](../../src/types.rs#L437) | `pub` | not a function |
| [mcp::types::McpTaskSupport](../../src/types.rs#L447) | `pub` | not a function |
| [mcp::types::McpToolExecution](../../src/types.rs#L455) | `pub` | not a function |
| [mcp::types::McpTool](../../src/types.rs#L461) | `pub` | not a function |
| [mcp::types::McpTool::requires_task](../../src/types.rs#L479) | `pub` | no resolved direct caller |
| [mcp::types::McpToolCallContext](../../src/types.rs#L533) | `pub` | not a function |
| [mcp::types::McpPrompt](../../src/types.rs#L539) | `pub` | not a function |
| [mcp::types::McpResource](../../src/types.rs#L547) | `pub` | not a function |
| [mcp::types::McpTask](../../src/types.rs#L556) | `pub` | not a function |
| [mcp::types::McpTask::validate_result](../../src/types.rs#L564) | `pub` | [mcp::client::McpClient::continue_tool_call](../../src/client.rs#L622); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::rmcp_interop::create](../../tests/rmcp_interop.rs#L218); [mcp::tests::rmcp_interop::get](../../tests/rmcp_interop.rs#L245); [mcp::tests::slice13_gates::task_poll_rejects_changed_identity_and_incomplete_terminal_state](../../tests/slice13_gates.rs#L1909); [mcp::tests::slice13_gates::official_sdk_task_shape_is_adopted_and_completed_with_tasks_result](../../tests/slice13_gates.rs#L1982); [mcp::tests::tasks_extension::validates](../../tests/tasks_extension.rs#L152); [tekes-supervisor::mcp_continuation::resolve_task](../../../supervisor/src/mcp_continuation.rs#L27); [tekes-supervisor::mcp_continuation::task_response](../../../supervisor/src/mcp_continuation.rs#L97); [tekes-supervisor::mcp_runtime::McpDynamicSupervisorAuthority::execute_outcome](../../../supervisor/src/mcp_runtime.rs#L2611) |
| [mcp::types::McpToolContinuation](../../src/types.rs#L633) | `pub` | not a function |
| [mcp::types::McpContent](../../src/types.rs#L643) | `pub` | not a function |
| [mcp::types::McpError](../../src/types.rs#L662) | `pub` | not a function |
