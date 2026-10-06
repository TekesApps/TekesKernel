# mcp — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [mcp::broker::McpBroker](../../src/broker.rs#L13) | `pub` | not a function |
| [mcp::broker::McpBrokerHandle](../../src/broker.rs#L20) | `pub` | not a function |
| [mcp::broker::McpBroker::start](../../src/broker.rs#L62) | `pub` | [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::slice13_gates::slice13_gate_92_closed_recovery_matrix](../../tests/slice13_gates.rs#L1637); [mcp::tests::slice13_gates::broker_task_operations_use_existing_peer_and_exact_task_identity](../../tests/slice13_gates.rs#L1934); [mcp::tests::slice13_gates::task_required_tools_are_declared_and_called_with_task_augmentation](../../tests/slice13_gates.rs#L2036); [mcp::tests::slice13_gates::cancellation_notification_failure_is_bounded_and_evicts_the_peer_generation](../../tests/slice13_gates.rs#L488); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L692); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::McpRuntime::open_with_authorities](../../../supervisor/src/mcp_runtime.rs#L202) |
| [mcp::broker::McpBroker::handle](../../src/broker.rs#L78) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::register](../../src/broker.rs#L93) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool](../../src/broker.rs#L111) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool_augmented](../../src/broker.rs#L135) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::call_tool_with_context](../../src/broker.rs#L159) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::task_operation](../../src/broker.rs#L182) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::task_operation_cancellable](../../src/broker.rs#L191) | `pub` | [mcp::broker::McpBrokerHandle::task_operation](../../src/broker.rs#L182) |
| [mcp::broker::McpBrokerHandle::remove](../../src/broker.rs#L218) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::release](../../src/broker.rs#L226) | `pub` | no resolved direct caller |
| [mcp::broker::McpBrokerHandle::catalog_generation](../../src/broker.rs#L234) | `pub` | no resolved direct caller |
| [mcp::client::McpPeerFuture](../../src/client.rs#L20) | `pub` | not a function |
| [mcp::client::McpPeer](../../src/client.rs#L22) | `pub` | not a function |
| [mcp::client::McpClient](../../src/client.rs#L124) | `pub` | not a function |
| [mcp::client::McpClient::new](../../src/client.rs#L140) | `pub` | [mcp::tests::handshake_lifecycle::cancellation_at_each_handshake_stage_closes_once_before_return](../../tests/handshake_lifecycle.rs#L139); [mcp::tests::handshake_lifecycle::pre_cancelled_handshake_closes_without_sending](../../tests/handshake_lifecycle.rs#L182); [mcp::tests::handshake_lifecycle::every_terminal_handshake_failure_closes_exactly_once](../../tests/handshake_lifecycle.rs#L61); [mcp::tests::http_discovery::client_owned_subscription_updates_generation_and_closes_with_peer](../../tests/http_discovery.rs#L1126); [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::task_cancellation_distinguishes_read_from_dispatched_mutation](../../tests/http_discovery.rs#L1334); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::http_discovery::probe](../../tests/http_discovery.rs#L349); [mcp::tests::http_discovery::legacy_expired_session_reinitializes_and_close_deletes_once](../../tests/http_discovery.rs#L545); [mcp::tests::http_discovery::http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once](../../tests/http_discovery.rs#L671); [mcp::tests::http_discovery::modern_scoped_http_calls_overlap_and_cancel_only_their_own_socket](../../tests/http_discovery.rs#L70); [mcp::tests::http_discovery::legacy_http_connects_lists_and_calls_tool_over_real_socket](../../tests/http_discovery.rs#L741); [mcp::tests::http_discovery::modern_http_continuation_preserves_input_and_task_resume_queries_identity](../../tests/http_discovery.rs#L842); [mcp::tests::http_discovery::modern_scoped_call_does_not_claim_known_failure_after_dispatch](../../tests/http_discovery.rs#L8); [mcp::tests::http_discovery::parameter_headers_follow_scoped_arguments_and_catalog_replacement](../../tests/http_discovery.rs#L943); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_forced_sse_round_trip](../../tests/live_official_sdk_tunnel.rs#L166); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::live_services::live_cloudflare_docs_parameter_contract](../../tests/live_services.rs#L122); [mcp::tests::live_services::public_round_trip](../../tests/live_services.rs#L5); [mcp::tests::live_services::live_cloudflare_bearer_catalog_and_search](../../tests/live_services.rs#L62); [mcp::tests::slice13_gates::effectful_tool_call_carries_host_owned_idempotency_key_in_request_metadata](../../tests/slice13_gates.rs#L1186); [mcp::tests::slice13_gates::slice13_gate_92_closed_recovery_matrix](../../tests/slice13_gates.rs#L1637); [mcp::tests::slice13_gates::slice13_gate_87_operations_and_cancellation_shapes](../../tests/slice13_gates.rs#L179); [mcp::tests::slice13_gates::fixture_client_with_args](../../tests/slice13_gates.rs#L24); [mcp::tests::slice13_gates::cancellation_notification_failure_is_bounded_and_evicts_the_peer_generation](../../tests/slice13_gates.rs#L488); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L692); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1518) |
| [mcp::client::McpClient::connect](../../src/client.rs#L157) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::connect_cancellable](../../src/client.rs#L165) | `pub` | [mcp::client::McpClient::connect](../../src/client.rs#L157) |
| [mcp::client::McpClient::server](../../src/client.rs#L275) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::supported_versions](../../src/client.rs#L280) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::protocol_version](../../src/client.rs#L284) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::catalog_generation](../../src/client.rs#L289) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::continue_tool_call](../../src/client.rs#L612) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::task_mutation_with_resumable_identity](../../src/client.rs#L704) | `pub` | no resolved direct caller |
| [mcp::client::McpClient::call_tool_scoped](../../src/client.rs#L1367) | `pub` | no resolved direct caller |
| [mcp::LEGACY_PROTOCOL_VERSION](../../src/lib.rs#L36) | `pub` | not a function |
| [mcp::MODERN_PROTOCOL_VERSION](../../src/lib.rs#L37) | `pub` | not a function |
| [mcp::MAX_FRAME_BYTES](../../src/lib.rs#L38) | `pub` | not a function |
| [mcp::MAX_CATALOG_ITEMS](../../src/lib.rs#L39) | `pub` | not a function |
| [mcp::MAX_CATALOG_PAGES](../../src/lib.rs#L40) | `pub` | not a function |
| [mcp::MAX_CONTINUATION_ROUNDS](../../src/lib.rs#L41) | `pub` | not a function |
| [mcp::management::McpScope](../../src/management.rs#L19) | `pub` | not a function |
| [mcp::management::McpServerReference](../../src/management.rs#L27) | `pub` | not a function |
| [mcp::management::CredentialValue](../../src/management.rs#L35) | `pub` | not a function |
| [mcp::management::McpTransportConfig](../../src/management.rs#L77) | `pub` | not a function |
| [mcp::management::McpServerConfig](../../src/management.rs#L94) | `pub` | not a function |
| [mcp::management::McpRegistry](../../src/management.rs#L110) | `pub` | not a function |
| [mcp::management::McpRegistry::validate](../../src/management.rs#L125) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistry::resolve](../../src/management.rs#L151) | `pub` | no resolved direct caller |
| [mcp::management::McpCredentialReferenceState](../../src/management.rs#L184) | `pub` | not a function |
| [mcp::management::McpCredentialReference](../../src/management.rs#L191) | `pub` | not a function |
| [mcp::management::McpManagementMutation](../../src/management.rs#L300) | `pub` | not a function |
| [mcp::management::McpCredentialFieldOperation](../../src/management.rs#L327) | `pub` | not a function |
| [mcp::management::McpManagementResult](../../src/management.rs#L335) | `pub` | not a function |
| [mcp::management::McpMutationReceipt](../../src/management.rs#L357) | `pub` | not a function |
| [mcp::management::McpManagementFault](../../src/management.rs#L367) | `pub` | not a function |
| [mcp::management::McpRegistryStore](../../src/management.rs#L377) | `pub` | not a function |
| [mcp::management::McpRegistryStore::new](../../src/management.rs#L383) | `pub` | [mcp::tests::slice13_gates::slice13_gate_91_management_publication_and_recovery](../../tests/slice13_gates.rs#L1290); [mcp::tests::slice13_gates::slice13_gate_91_management_faults_recover_registry_and_credential_receipt](../../tests/slice13_gates.rs#L1338); [mcp::tests::slice13_gates::slice13_gate_91_credential_field_operations_preserve_replace_and_remove_references](../../tests/slice13_gates.rs#L1421); [mcp::tests::slice13_gates::slice13_gate_91_oauth_pending_to_bound_transition_is_exact](../../tests/slice13_gates.rs#L1508); [mcp::tests::slice13_gates::registry_file_keeps_the_last_entry_for_a_repeated_reference_and_saves_it_once](../../tests/slice13_gates.rs#L2133); [mcp::tests::slice13_gates::registry_file_tolerates_missing_trailing_newline_and_names_the_file_on_corruption](../../tests/slice13_gates.rs#L2197); [tekes-supervisor::mcp_runtime::McpRuntime::open_with_authorities](../../../supervisor/src/mcp_runtime.rs#L202); [tekes-supervisor::process_live_tests::real_public_mcp_task_continuation](../../../supervisor/src/process_live_tests.rs#L1372); [tekes-supervisor::process_live_tests::register_local_tools_server](../../../supervisor/src/process_live_tests.rs#L981); [tekes-supervisor::tests::slice13_management::slice13_gate_89_pending_oauth_requires_exact_bound_ownership](../../../supervisor/tests/slice13_management.rs#L435) |
| [mcp::management::McpRegistryStore::injecting_fault](../../src/management.rs#L391) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::load](../../src/management.rs#L396) | `pub` | [mcp::management::McpRegistryStore::list](../../src/management.rs#L408); [mcp::management::McpRegistryStore::get](../../src/management.rs#L417) |
| [mcp::management::McpRegistryStore::publish](../../src/management.rs#L400) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::list](../../src/management.rs#L408) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::get](../../src/management.rs#L417) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::credential_reference_state](../../src/management.rs#L428) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::committed_receipt](../../src/management.rs#L451) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::mutate_idempotent](../../src/management.rs#L475) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryStore::recover](../../src/management.rs#L525) | `pub` | no resolved direct caller |
| [mcp::management::McpRegistryError](../../src/management.rs#L1180) | `pub` | not a function |
| [mcp::parameter_headers::ParameterHeaders](../../src/parameter_headers.rs#L13) | `pub(crate)` | not a function |
| [mcp::parameter_headers::ParameterHeaders::catalog](../../src/parameter_headers.rs#L20) | `pub(crate)` | [mcp::transport::McpTransport::install_tool_catalog](../../src/transport.rs#L40); [mcp::transport::HttpTransport::install_tool_catalog](../../src/transport.rs#L929) |
| [mcp::parameter_headers::ParameterHeaders::is_empty](../../src/parameter_headers.rs#L127) | `pub(crate)` | no resolved direct caller |
| [mcp::parameter_headers::ParameterHeaders::project](../../src/parameter_headers.rs#L130) | `pub(crate)` | no resolved direct caller |
| [mcp::pool::McpPoolKey](../../src/pool.rs#L10) | `pub` | not a function |
| [mcp::pool::PooledPeer](../../src/pool.rs#L20) | `pub` | not a function |
| [mcp::pool::McpPool](../../src/pool.rs#L29) | `pub` | not a function |
| [mcp::pool::McpPoolLease](../../src/pool.rs#L39) | `pub` | not a function |
| [mcp::pool::McpPoolLease::peer](../../src/pool.rs#L48) | `pub` | no resolved direct caller |
| [mcp::pool::McpPoolLease::release](../../src/pool.rs#L52) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::new](../../src/pool.rs#L75) | `pub` | [mcp::broker::run](../../src/broker.rs#L271); [mcp::tests::slice13_gates::slice13_gate_88_stdio_and_daemon_pool_ownership](../../tests/slice13_gates.rs#L692) |
| [mcp::pool::McpPool::get](../../src/pool.rs#L79) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::acquire_or_create](../../src/pool.rs#L94) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::insert](../../src/pool.rs#L173) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::replace_generation](../../src/pool.rs#L203) | `pub` | [mcp::pool::McpPool::acquire_or_create](../../src/pool.rs#L94) |
| [mcp::pool::McpPool::remove](../../src/pool.rs#L263) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::release](../../src/pool.rs#L274) | `pub` | no resolved direct caller |
| [mcp::pool::McpPool::drain](../../src/pool.rs#L293) | `pub` | no resolved direct caller |
| [mcp::pool::McpPoolError](../../src/pool.rs#L309) | `pub` | not a function |
| [mcp::projection::project_name](../../src/projection.rs#L14) | `pub` | [mcp::projection::project_catalog](../../src/projection.rs#L25); [tekes-supervisor::mcp_runtime::McpRuntime::bind_prepared_server](../../../supervisor/src/mcp_runtime.rs#L635); [tekes-supervisor::process_host::mcp_failure_is_required](../../../supervisor/src/process_host.rs#L3986) |
| [mcp::projection::project_catalog](../../src/projection.rs#L25) | `pub` | [engine::tests::slice8_dynamic_catalog::mcp_destructive_hint_reaches_the_engine_destructive_approval_gate](../../../engine/tests/slice8_dynamic_catalog.rs#L128); [mcp::tests::slice13_gates::slice13_gate_90_catalog_projection_and_collision](../../tests/slice13_gates.rs#L1009); [mcp::tests::slice13_gates::pydantic_generated_catalog_projects_every_tool](../../tests/slice13_gates.rs#L1070); [mcp::tests::slice13_gates::external_effect_metadata_projects_only_with_authoritative_read_only_query](../../tests/slice13_gates.rs#L1131); [tekes-supervisor::mcp_runtime::McpDynamicSupervisorAuthority::ensure_route_peer](../../../supervisor/src/mcp_runtime.rs#L2292); [tekes-supervisor::mcp_runtime::McpRuntime::bind_prepared_server](../../../supervisor/src/mcp_runtime.rs#L635) |
| [mcp::projection::McpProjectionError](../../src/projection.rs#L175) | `pub` | not a function |
| [mcp::recovery::McpLossState](../../src/recovery.rs#L2) | `pub` | not a function |
| [mcp::recovery::RecoveryAction](../../src/recovery.rs#L14) | `pub` | not a function |
| [mcp::recovery::recovery_action](../../src/recovery.rs#L23) | `pub` | [mcp::client::should_reconnect](../../src/client.rs#L1276); [mcp::client::mutation_error](../../src/client.rs#L1289); [mcp::client::McpClient::task_mutation_with_resumable_identity](../../src/client.rs#L704) |
| [mcp::subscription::McpSubscriptionFilter](../../src/subscription.rs#L7) | `pub` | not a function |
| [mcp::subscription::McpSubscriptionFilter::new](../../src/subscription.rs#L16) | `pub` | [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L627); [mcp::transport::HttpTransport::start_catalog_subscription](../../src/transport.rs#L910) |
| [mcp::subscription::McpSubscriptionFilter::request_params](../../src/subscription.rs#L48) | `pub` | no resolved direct caller |
| [mcp::subscription::McpSubscriptionFilter::is_empty](../../src/subscription.rs#L51) | `pub` | no resolved direct caller |
| [mcp::subscription::McpSubscriptionFilter::accepts](../../src/subscription.rs#L56) | `pub` | no resolved direct caller |
| [mcp::transport::McpMethodObserver](../../src/transport.rs#L14) | `pub` | not a function |
| [mcp::transport::McpValueObserver](../../src/transport.rs#L16) | `pub` | not a function |
| [mcp::transport::McpTransport](../../src/transport.rs#L27) | `pub` | not a function |
| [mcp::transport::StdioTransport](../../src/transport.rs#L85) | `pub` | not a function |
| [mcp::transport::StdioTransport::spawn](../../src/transport.rs#L136) | `pub` | [mcp::tests::slice13_gates::fixture_client_with_args](../../tests/slice13_gates.rs#L24); [tekes-supervisor::mcp_continuation::tests::journal_drives_bound_task_query_update_and_terminal_replay](../../../supervisor/src/mcp_continuation.rs#L233); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1518) |
| [mcp::transport::HttpTransport](../../src/transport.rs#L484) | `pub` | not a function |
| [mcp::transport::HttpRequestAuthorization](../../src/transport.rs#L511) | `pub` | not a function |
| [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L531) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L541) | `pub(crate)` | [mcp::transport::HttpTransport::request_modern_scoped](../../src/transport.rs#L531) |
| [mcp::transport::HttpTransport::start_modern_scoped](../../src/transport.rs#L554) | `pub(crate)` | [mcp::transport::HttpTransport::request_modern_scoped_observed](../../src/transport.rs#L541) |
| [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L627) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::with_notification_handler](../../src/transport.rs#L671) | `pub` | no resolved direct caller |
| [mcp::transport::HttpTransport::new](../../src/transport.rs#L678) | `pub` | [mcp::tests::http_discovery::dedicated_subscription_delivers_correlated_change_before_cancel_closes_socket](../../tests/http_discovery.rs#L1040); [mcp::tests::http_discovery::client_owned_subscription_updates_generation_and_closes_with_peer](../../tests/http_discovery.rs#L1126); [mcp::tests::http_discovery::broker_registration_owns_subscription_until_removal](../../tests/http_discovery.rs#L1215); [mcp::tests::http_discovery::task_cancellation_distinguishes_read_from_dispatched_mutation](../../tests/http_discovery.rs#L1334); [mcp::tests::http_discovery::broker_modern_calls_overlap_and_cancel_without_evicting_siblings](../../tests/http_discovery.rs#L169); [mcp::tests::http_discovery::sse_progress_is_observed_before_server_releases_final_response](../../tests/http_discovery.rs#L302); [mcp::tests::http_discovery::probe](../../tests/http_discovery.rs#L349); [mcp::tests::http_discovery::legacy_expired_session_reinitializes_and_close_deletes_once](../../tests/http_discovery.rs#L545); [mcp::tests::http_discovery::http_handshake_cancellation_closes_socket_and_deletes_allocated_session_once](../../tests/http_discovery.rs#L671); [mcp::tests::http_discovery::modern_scoped_http_calls_overlap_and_cancel_only_their_own_socket](../../tests/http_discovery.rs#L70); [mcp::tests::http_discovery::legacy_http_connects_lists_and_calls_tool_over_real_socket](../../tests/http_discovery.rs#L741); [mcp::tests::http_discovery::modern_http_continuation_preserves_input_and_task_resume_queries_identity](../../tests/http_discovery.rs#L842); [mcp::tests::http_discovery::modern_scoped_call_does_not_claim_known_failure_after_dispatch](../../tests/http_discovery.rs#L8); [mcp::tests::http_discovery::parameter_headers_follow_scoped_arguments_and_catalog_replacement](../../tests/http_discovery.rs#L943); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_forced_sse_round_trip](../../tests/live_official_sdk_tunnel.rs#L166); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::live_services::live_cloudflare_docs_parameter_contract](../../tests/live_services.rs#L122); [mcp::tests::live_services::public_round_trip](../../tests/live_services.rs#L5); [mcp::tests::live_services::live_cloudflare_bearer_catalog_and_search](../../tests/live_services.rs#L62); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L801) |
| [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L703) | `pub` | [mcp::transport::HttpTransport::new](../../src/transport.rs#L678); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L801) |
| [mcp::transport::HttpTransport::new_with_request_authorization_provider](../../src/transport.rs#L727) | `pub` | [mcp::transport::HttpTransport::new_with_authorization_provider](../../src/transport.rs#L703); [tekes-supervisor::mcp_runtime::prepare_server_cancellable](../../../supervisor/src/mcp_runtime.rs#L1518) |
| [mcp::types::McpCancellationToken](../../src/types.rs#L11) | `pub` | not a function |
| [mcp::types::McpCancellationToken::cancel](../../src/types.rs#L14) | `pub` | no resolved direct caller |
| [mcp::types::McpCancellationToken::is_cancelled](../../src/types.rs#L19) | `pub` | [mcp::types::McpCancellationToken::cancelled](../../src/types.rs#L23) |
| [mcp::types::McpCancellationToken::cancelled](../../src/types.rs#L23) | `pub` | no resolved direct caller |
| [mcp::types::ProtocolMode](../../src/types.rs#L32) | `pub` | not a function |
| [mcp::types::TransportKind](../../src/types.rs#L40) | `pub` | not a function |
| [mcp::types::JsonRpcRequest](../../src/types.rs#L47) | `pub` | not a function |
| [mcp::types::JsonRpcRequest::call](../../src/types.rs#L57) | `pub` | [mcp::client::McpClient::call_raw_once](../../src/client.rs#L388); [mcp::client::McpClient::call_raw_cancellable](../../src/client.rs#L430); [mcp::client::McpClient::start_scoped_tool](../../src/client.rs#L748); [mcp::transport::HttpTransport::listen_subscription](../../src/transport.rs#L627); [mcp::tests::slice13_gates::slice13_gate_89_http_oauth_authorization_partition](../../tests/slice13_gates.rs#L801) |
| [mcp::types::JsonRpcRequest::notification](../../src/types.rs#L66) | `pub` | [mcp::client::McpClient::connect_legacy](../../src/client.rs#L294); [mcp::client::McpClient::call_raw_cancellable](../../src/client.rs#L430); [mcp::transport::HttpTransport::close](../../src/transport.rs#L1137) |
| [mcp::types::JsonRpcRequest::canonical_line](../../src/types.rs#L75) | `pub` | no resolved direct caller |
| [mcp::types::JsonRpcError](../../src/types.rs#L88) | `pub` | not a function |
| [mcp::types::JsonRpcResponse](../../src/types.rs#L97) | `pub` | not a function |
| [mcp::types::JsonRpcResponse::validate_for](../../src/types.rs#L107) | `pub` | no resolved direct caller |
| [mcp::types::JsonRpcResponse::into_result](../../src/types.rs#L121) | `pub` | no resolved direct caller |
| [mcp::types::McpImplementation](../../src/types.rs#L135) | `pub` | not a function |
| [mcp::types::McpIcon](../../src/types.rs#L149) | `pub` | not a function |
| [mcp::types::McpIconTheme](../../src/types.rs#L161) | `pub` | not a function |
| [mcp::types::McpCapabilities](../../src/types.rs#L298) | `pub` | not a function |
| [mcp::types::McpCapabilities::supports_tasks](../../src/types.rs#L312) | `pub` | no resolved direct caller |
| [mcp::types::McpToolAnnotations](../../src/types.rs#L429) | `pub` | not a function |
| [mcp::types::McpTaskSupport](../../src/types.rs#L439) | `pub` | not a function |
| [mcp::types::McpToolExecution](../../src/types.rs#L447) | `pub` | not a function |
| [mcp::types::McpTool](../../src/types.rs#L453) | `pub` | not a function |
| [mcp::types::McpTool::requires_task](../../src/types.rs#L471) | `pub` | no resolved direct caller |
| [mcp::types::McpToolCallContext](../../src/types.rs#L525) | `pub` | not a function |
| [mcp::types::McpPrompt](../../src/types.rs#L531) | `pub` | not a function |
| [mcp::types::McpResource](../../src/types.rs#L539) | `pub` | not a function |
| [mcp::types::McpTask](../../src/types.rs#L548) | `pub` | not a function |
| [mcp::types::McpTask::validate_result](../../src/types.rs#L556) | `pub` | [mcp::client::McpClient::continue_tool_call](../../src/client.rs#L612); [mcp::tests::live_official_sdk_tunnel::official_sdk_public_tunnel_completes_task_lifecycle](../../tests/live_official_sdk_tunnel.rs#L45); [mcp::tests::slice13_gates::task_poll_rejects_changed_identity_and_incomplete_terminal_state](../../tests/slice13_gates.rs#L1903); [mcp::tests::slice13_gates::official_sdk_task_shape_is_adopted_and_completed_with_tasks_result](../../tests/slice13_gates.rs#L1976); [tekes-supervisor::mcp_continuation::resolve_task](../../../supervisor/src/mcp_continuation.rs#L27); [tekes-supervisor::mcp_continuation::task_response](../../../supervisor/src/mcp_continuation.rs#L97); [tekes-supervisor::mcp_runtime::McpDynamicSupervisorAuthority::execute_outcome](../../../supervisor/src/mcp_runtime.rs#L2161) |
| [mcp::types::McpToolContinuation](../../src/types.rs#L625) | `pub` | not a function |
| [mcp::types::McpContent](../../src/types.rs#L635) | `pub` | not a function |
| [mcp::types::McpError](../../src/types.rs#L654) | `pub` | not a function |
