# transport — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [transport::access::AccessLogRecord](../../src/access.rs#L6) | `pub` | not a function |
| [transport::access::AccessLogRecord::canonical_bytes](../../src/access.rs#L18) | `pub` | no resolved direct caller |
| [transport::access::AccessLogSink](../../src/access.rs#L23) | `pub` | not a function |
| [transport::access::NoopAccessLog](../../src/access.rs#L28) | `pub` | not a function |
| [transport::access::noop](../../src/access.rs#L34) | `pub(crate)` | [transport::server::TransportConfig::loopback](../../src/server.rs#L108) |
| [transport::auth::BearerToken](../../src/auth.rs#L18) | `pub` | not a function |
| [transport::auth::BearerToken::new](../../src/auth.rs#L36) | `pub` | [tekes-supervisor::examples::slice9_production_client_harness_server::main](../../../supervisor/examples/slice9_production_client_harness_server.rs#L176); [tekes-supervisor::builtin::run](../../../supervisor/src/builtin.rs#L35); [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../../supervisor/src/process_live_tests.rs#L1073); [tekes-supervisor::process_live_tests::real_public_image_attachment](../../../supervisor/src/process_live_tests.rs#L1187); [tekes-supervisor::process_live_tests::real_public_flow_case](../../../supervisor/src/process_live_tests.rs#L1259); [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../../supervisor/src/process_live_tests.rs#L640); [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../../supervisor/src/process_live_tests.rs#L830); [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../../supervisor/src/process_live_tests.rs#L983); [tekes-supervisor::tests::s4_lazy_boot_streams::assembly](../../../supervisor/tests/s4_lazy_boot_streams.rs#L151); [tekes-supervisor::tests::slice9_endpoint_carrier::assembly](../../../supervisor/tests/slice9_endpoint_carrier.rs#L146); [transport::tests::slice9_transport::config](../../tests/slice9_transport.rs#L535); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L601); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L711) |
| [transport::auth::BearerToken::authenticates](../../src/auth.rs#L40) | `pub(crate)` | no resolved direct caller |
| [transport::server::MAX_REQUEST_BYTES](../../src/server.rs#L36) | `pub` | not a function |
| [transport::server::ROUTE_REGISTRY](../../src/server.rs#L43) | `pub` | not a function |
| [transport::server::TransportLimits](../../src/server.rs#L61) | `pub` | not a function |
| [transport::server::OriginPolicy](../../src/server.rs#L82) | `pub` | not a function |
| [transport::server::TransportConfig](../../src/server.rs#L91) | `pub` | not a function |
| [transport::server::TransportConfig::loopback](../../src/server.rs#L108) | `pub` | [tekes-supervisor::examples::slice9_production_client_harness_server::main](../../../supervisor/examples/slice9_production_client_harness_server.rs#L176); [tekes-supervisor::builtin::run](../../../supervisor/src/builtin.rs#L35); [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../../supervisor/src/process_live_tests.rs#L1073); [tekes-supervisor::process_live_tests::real_public_image_attachment](../../../supervisor/src/process_live_tests.rs#L1187); [tekes-supervisor::process_live_tests::real_public_flow_case](../../../supervisor/src/process_live_tests.rs#L1259); [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../../supervisor/src/process_live_tests.rs#L640); [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../../supervisor/src/process_live_tests.rs#L830); [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../../supervisor/src/process_live_tests.rs#L983); [tekes-supervisor::tests::s4_lazy_boot_streams::assembly](../../../supervisor/tests/s4_lazy_boot_streams.rs#L151); [tekes-supervisor::tests::slice9_endpoint_carrier::assembly](../../../supervisor/tests/slice9_endpoint_carrier.rs#L146); [transport::tests::slice9_transport::config](../../tests/slice9_transport.rs#L535); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L601); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L711) |
| [transport::server::TransportConfig::with_access_log](../../src/server.rs#L120) | `pub` | no resolved direct caller |
| [transport::server::TransportConfig::with_readiness_identity](../../src/server.rs#L129) | `pub` | no resolved direct caller |
| [transport::server::TransportConfig::validate](../../src/server.rs#L137) | `pub` | no resolved direct caller |
| [transport::server::TransportHandle](../../src/server.rs#L225) | `pub` | not a function |
| [transport::server::TransportHandle::begin_drain](../../src/server.rs#L231) | `pub` | no resolved direct caller |
| [transport::server::TransportHandle::is_draining](../../src/server.rs#L237) | `pub` | no resolved direct caller |
| [transport::server::TransportServer](../../src/server.rs#L242) | `pub` | not a function |
| [transport::server::FileTransferAuthority](../../src/server.rs#L248) | `pub` | not a function |
| [transport::server::FileChangeFeed](../../src/server.rs#L254) | `pub` | not a function |
| [transport::server::FileChangeAuthority](../../src/server.rs#L258) | `pub` | not a function |
| [transport::server::WebClientConfig](../../src/server.rs#L263) | `pub` | not a function |
| [transport::server::WebClientConfig::loopback](../../src/server.rs#L269) | `pub` | [tekes-supervisor::builtin::run](../../../supervisor/src/builtin.rs#L35); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L601) |
| [transport::server::WebClientService](../../src/server.rs#L287) | `pub` | not a function |
| [transport::server::TransportServer::new](../../src/server.rs#L293) | `pub` | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../../supervisor/src/endpoint_carrier.rs#L2785); [transport::tests::slice9_transport::deployment_health_uses_frozen_identity_and_closed_readiness_rows](../../tests/slice9_transport.rs#L1116); [transport::tests::slice9_transport::idle_server_drain_does_not_wait_the_full_response_timeout](../../tests/slice9_transport.rs#L1203); [transport::tests::slice9_transport::v3_mux_rejects_a_duplicate_baseline_on_one_logical_stream](../../tests/slice9_transport.rs#L1220); [transport::tests::slice9_transport::v3_mux_drops_frames_queued_for_a_stream_the_client_closed](../../tests/slice9_transport.rs#L1297); [transport::tests::slice9_transport::thread_open_failures_are_the_closed_tables_503_on_http](../../tests/slice9_transport.rs#L1376); [transport::tests::slice9_transport::file_change_socket_is_authenticated_and_drops_subscription_on_close](../../tests/slice9_transport.rs#L28); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L601); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L711); [transport::tests::slice9_transport::slice9_gate_66_endpoint_http_error_and_idempotency](../../tests/slice9_transport.rs#L778); [transport::tests::slice9_transport::preview_http_requires_auth_and_serves_raw_bytes_until_release](../../tests/slice9_transport.rs#L89) |
| [transport::server::TransportServer::with_file_transfer](../../src/server.rs#L340) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::set_file_changes](../../src/server.rs#L347) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::handle](../../src/server.rs#L354) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::router](../../src/server.rs#L360) | `pub` | [transport::server::TransportServer::serve](../../src/server.rs#L397) |
| [transport::server::TransportServer::web_client](../../src/server.rs#L383) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::serve](../../src/server.rs#L397) | `pub` | no resolved direct caller |
| [transport::server::WebClientService::router](../../src/server.rs#L566) | `pub` | [transport::server::WebClientService::serve](../../src/server.rs#L581) |
| [transport::server::WebClientService::serve](../../src/server.rs#L581) | `pub` | no resolved direct caller |
| [transport::server::TransportConfigError](../../src/server.rs#L1410) | `pub` | not a function |
| [transport::web::INDEX_HTML](../../src/web.rs#L11) | `pub(crate)` | not a function |
| [transport::web::APP_CSS](../../src/web.rs#L12) | `pub(crate)` | not a function |
| [transport::web::APP_JS](../../src/web.rs#L13) | `pub(crate)` | not a function |
| [transport::web::WEB_CLIENT_SHA256](../../src/web.rs#L17) | `pub` | not a function |
| [transport::web::BrowserAccess](../../src/web.rs#L20) | `pub(crate)` | not a function |
| [transport::web::BrowserAccess::new](../../src/web.rs#L26) | `pub(crate)` | [transport::server::TransportServer::web_client](../../src/server.rs#L383) |
| [transport::web::BrowserAccess::permits_origin](../../src/web.rs#L34) | `pub(crate)` | no resolved direct caller |
| [transport::web::BrowserAccess::permits_host](../../src/web.rs#L38) | `pub(crate)` | no resolved direct caller |
| [transport::web::static_response](../../src/web.rs#L63) | `pub(crate)` | [transport::server::web_css](../../src/server.rs#L617); [transport::server::web_js](../../src/server.rs#L621) |
| [transport::web::index_response](../../src/web.rs#L74) | `pub(crate)` | [transport::server::web_index](../../src/server.rs#L609) |
| [transport::web::index_unauthorized](../../src/web.rs#L91) | `pub(crate)` | [transport::server::web_index](../../src/server.rs#L609) |
