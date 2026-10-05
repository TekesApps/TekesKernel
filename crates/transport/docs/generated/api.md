# transport — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [transport::access::AccessLogRecord](../../src/access.rs#L6) | `pub` | not a function |
| [transport::access::AccessLogRecord::canonical_bytes](../../src/access.rs#L18) | `pub` | no resolved direct caller |
| [transport::access::AccessLogSink](../../src/access.rs#L23) | `pub` | not a function |
| [transport::access::NoopAccessLog](../../src/access.rs#L28) | `pub` | not a function |
| [transport::access::noop](../../src/access.rs#L34) | `pub(crate)` | [transport::server::TransportConfig::loopback](../../src/server.rs#L111) |
| [transport::auth::BearerToken](../../src/auth.rs#L18) | `pub` | not a function |
| [transport::auth::BearerToken::new](../../src/auth.rs#L36) | `pub` | [tekes-supervisor::examples::slice9_production_client_harness_server::main](../../../supervisor/examples/slice9_production_client_harness_server.rs#L176); [tekes-supervisor::builtin::run](../../../supervisor/src/builtin.rs#L29); [tekes-supervisor::daemon::run_daemon_inner](../../../supervisor/src/daemon.rs#L745); [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../../supervisor/src/process_live_tests.rs#L1003); [tekes-supervisor::process_live_tests::real_public_image_attachment](../../../supervisor/src/process_live_tests.rs#L1118); [tekes-supervisor::process_live_tests::real_public_flow_case](../../../supervisor/src/process_live_tests.rs#L1191); [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../../supervisor/src/process_live_tests.rs#L540); [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../../supervisor/src/process_live_tests.rs#L732); [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../../supervisor/src/process_live_tests.rs#L897); [tekes-supervisor::tests::slice10_daemon::deployment::slice10_gate_71_deployment_filesystem_and_ownership](../../../supervisor/tests/slice10_daemon.rs#L43); [tekes-supervisor::tests::slice9_endpoint_carrier::assembly](../../../supervisor/tests/slice9_endpoint_carrier.rs#L146); [transport::tests::slice9_transport::config](../../tests/slice9_transport.rs#L511); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L577); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L687) |
| [transport::auth::BearerToken::authenticates](../../src/auth.rs#L40) | `pub(crate)` | no resolved direct caller |
| [transport::server::MAX_REQUEST_BYTES](../../src/server.rs#L36) | `pub` | not a function |
| [transport::server::ROUTE_REGISTRY](../../src/server.rs#L43) | `pub` | not a function |
| [transport::server::TransportLimits](../../src/server.rs#L64) | `pub` | not a function |
| [transport::server::OriginPolicy](../../src/server.rs#L85) | `pub` | not a function |
| [transport::server::TransportConfig](../../src/server.rs#L94) | `pub` | not a function |
| [transport::server::TransportConfig::loopback](../../src/server.rs#L111) | `pub` | [tekes-supervisor::examples::slice9_production_client_harness_server::main](../../../supervisor/examples/slice9_production_client_harness_server.rs#L176); [tekes-supervisor::builtin::run](../../../supervisor/src/builtin.rs#L29); [tekes-supervisor::daemon::run_daemon_inner](../../../supervisor/src/daemon.rs#L745); [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../../supervisor/src/process_live_tests.rs#L1003); [tekes-supervisor::process_live_tests::real_public_image_attachment](../../../supervisor/src/process_live_tests.rs#L1118); [tekes-supervisor::process_live_tests::real_public_flow_case](../../../supervisor/src/process_live_tests.rs#L1191); [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../../supervisor/src/process_live_tests.rs#L540); [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../../supervisor/src/process_live_tests.rs#L732); [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../../supervisor/src/process_live_tests.rs#L897); [tekes-supervisor::tests::slice10_daemon::deployment::slice10_gate_71_deployment_filesystem_and_ownership](../../../supervisor/tests/slice10_daemon.rs#L43); [tekes-supervisor::tests::slice9_endpoint_carrier::assembly](../../../supervisor/tests/slice9_endpoint_carrier.rs#L146); [transport::tests::slice9_transport::config](../../tests/slice9_transport.rs#L511); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L577); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L687) |
| [transport::server::TransportConfig::with_access_log](../../src/server.rs#L123) | `pub` | no resolved direct caller |
| [transport::server::TransportConfig::with_readiness_identity](../../src/server.rs#L132) | `pub` | no resolved direct caller |
| [transport::server::TransportConfig::validate](../../src/server.rs#L140) | `pub` | no resolved direct caller |
| [transport::server::TransportHandle](../../src/server.rs#L228) | `pub` | not a function |
| [transport::server::TransportHandle::begin_drain](../../src/server.rs#L234) | `pub` | no resolved direct caller |
| [transport::server::TransportHandle::is_draining](../../src/server.rs#L240) | `pub` | no resolved direct caller |
| [transport::server::TransportServer](../../src/server.rs#L245) | `pub` | not a function |
| [transport::server::FileTransferAuthority](../../src/server.rs#L251) | `pub` | not a function |
| [transport::server::FileChangeFeed](../../src/server.rs#L257) | `pub` | not a function |
| [transport::server::FileChangeAuthority](../../src/server.rs#L261) | `pub` | not a function |
| [transport::server::WebClientConfig](../../src/server.rs#L266) | `pub` | not a function |
| [transport::server::WebClientConfig::loopback](../../src/server.rs#L272) | `pub` | [tekes-supervisor::daemon::run_daemon_inner](../../../supervisor/src/daemon.rs#L745); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L577) |
| [transport::server::WebClientService](../../src/server.rs#L290) | `pub` | not a function |
| [transport::server::TransportServer::new](../../src/server.rs#L296) | `pub` | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../../supervisor/src/endpoint_carrier.rs#L1691); [transport::tests::slice9_transport::deployment_health_uses_frozen_identity_and_closed_readiness_rows](../../tests/slice9_transport.rs#L1092); [transport::tests::slice9_transport::idle_server_drain_does_not_wait_the_full_response_timeout](../../tests/slice9_transport.rs#L1179); [transport::tests::slice9_transport::v3_mux_rejects_a_duplicate_baseline_on_one_logical_stream](../../tests/slice9_transport.rs#L1196); [transport::tests::slice9_transport::file_change_socket_is_authenticated_and_drops_subscription_on_close](../../tests/slice9_transport.rs#L28); [transport::tests::slice9_transport::web_client_is_direct_on_loopback_and_rejects_cross_origin_data](../../tests/slice9_transport.rs#L577); [transport::tests::slice9_transport::slice9_gate_65_endpoint_transport_authority](../../tests/slice9_transport.rs#L687); [transport::tests::slice9_transport::slice9_gate_66_endpoint_http_error_and_idempotency](../../tests/slice9_transport.rs#L754); [transport::tests::slice9_transport::preview_http_requires_auth_and_serves_raw_bytes_until_release](../../tests/slice9_transport.rs#L89) |
| [transport::server::TransportServer::with_file_transfer](../../src/server.rs#L343) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::set_file_changes](../../src/server.rs#L350) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::handle](../../src/server.rs#L357) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::router](../../src/server.rs#L363) | `pub` | [transport::server::TransportServer::serve](../../src/server.rs#L400) |
| [transport::server::TransportServer::web_client](../../src/server.rs#L386) | `pub` | no resolved direct caller |
| [transport::server::TransportServer::serve](../../src/server.rs#L400) | `pub` | no resolved direct caller |
| [transport::server::WebClientService::router](../../src/server.rs#L569) | `pub` | [transport::server::WebClientService::serve](../../src/server.rs#L584) |
| [transport::server::WebClientService::serve](../../src/server.rs#L584) | `pub` | no resolved direct caller |
| [transport::server::TransportConfigError](../../src/server.rs#L1365) | `pub` | not a function |
| [transport::web::INDEX_HTML](../../src/web.rs#L11) | `pub(crate)` | not a function |
| [transport::web::APP_CSS](../../src/web.rs#L12) | `pub(crate)` | not a function |
| [transport::web::APP_JS](../../src/web.rs#L13) | `pub(crate)` | not a function |
| [transport::web::WEB_CLIENT_SHA256](../../src/web.rs#L17) | `pub` | not a function |
| [transport::web::BrowserAccess](../../src/web.rs#L20) | `pub(crate)` | not a function |
| [transport::web::BrowserAccess::new](../../src/web.rs#L26) | `pub(crate)` | [transport::server::TransportServer::web_client](../../src/server.rs#L386) |
| [transport::web::BrowserAccess::permits_origin](../../src/web.rs#L34) | `pub(crate)` | no resolved direct caller |
| [transport::web::BrowserAccess::permits_host](../../src/web.rs#L38) | `pub(crate)` | no resolved direct caller |
| [transport::web::static_response](../../src/web.rs#L63) | `pub(crate)` | [transport::server::web_css](../../src/server.rs#L620); [transport::server::web_js](../../src/server.rs#L624) |
| [transport::web::index_response](../../src/web.rs#L74) | `pub(crate)` | [transport::server::web_index](../../src/server.rs#L612) |
| [transport::web::index_unauthorized](../../src/web.rs#L91) | `pub(crate)` | [transport::server::web_index](../../src/server.rs#L612) |
