# tools::runtime_backends

[Package atlas](index.md) · [Source](../../src/runtime_backends.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tools::runtime_backends::HARD_HELPER_BYTES](../../src/runtime_backends.rs#L30) | const_item | `pub` |  |
| [tools::runtime_backends::HARD_HELPER_READ_BYTES](../../src/runtime_backends.rs#L31) | const_item | `pub` |  |
| [tools::runtime_backends::HARD_HELPER_TIMEOUT_MS](../../src/runtime_backends.rs#L32) | const_item | `pub` |  |
| [tools::runtime_backends::HELPER_PROCESS_OVERHEAD_MS](../../src/runtime_backends.rs#L33) | const_item | `private` |  |
| [tools::runtime_backends::HARD_HTTP_BYTES](../../src/runtime_backends.rs#L34) | const_item | `pub` |  |
| [tools::runtime_backends::HARD_HTTP_TIMEOUT_MS](../../src/runtime_backends.rs#L35) | const_item | `pub` |  |
| [tools::runtime_backends::HARD_JOB_TAIL_BYTES](../../src/runtime_backends.rs#L36) | const_item | `pub` |  |
| [tools::runtime_backends::MAX_REDIRECTS](../../src/runtime_backends.rs#L37) | const_item | `pub` |  |
| [tools::runtime_backends::WEB_FETCH_ACCEPT](../../src/runtime_backends.rs#L38) | const_item | `pub` |  |
| [tools::runtime_backends::WEB_FETCH_MAX_CHARACTERS](../../src/runtime_backends.rs#L40) | const_item | `pub` |  |
| [tools::runtime_backends::WEB_FETCH_MIN_MEANINGFUL_CHARACTERS](../../src/runtime_backends.rs#L41) | const_item | `pub` |  |
| [tools::runtime_backends::WEB_FETCH_USER_AGENT](../../src/runtime_backends.rs#L42) | const_item | `pub` |  |
| [tools::runtime_backends::BackendOutcome](../../src/runtime_backends.rs#L45) | enum_item | `pub` |  |
| [tools::runtime_backends::BackendHold](../../src/runtime_backends.rs#L52) | struct_item | `pub` |  |
| [tools::runtime_backends::BackendUnavailable](../../src/runtime_backends.rs#L58) | struct_item | `pub` |  |
| [tools::runtime_backends::BackendGate](../../src/runtime_backends.rs#L64) | enum_item | `pub` |  |
| [tools::runtime_backends::web_fetch_status_failure](../../src/runtime_backends.rs#L74) | function_item | `private` |  |
| [tools::runtime_backends::BackendFailure](../../src/runtime_backends.rs#L90) | enum_item | `pub` |  |
| [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) | function_item | `private` |  |
| [tools::runtime_backends::CancellationToken](../../src/runtime_backends.rs#L128) | struct_item | `pub` |  |
| [tools::runtime_backends::CancellationToken::cancel](../../src/runtime_backends.rs#L131) | function_item | `pub` |  |
| [tools::runtime_backends::CancellationToken::is_cancelled](../../src/runtime_backends.rs#L136) | function_item | `pub` |  |
| [tools::runtime_backends::HelperInvoker](../../src/runtime_backends.rs#L141) | trait_item | `pub` |  |
| [tools::runtime_backends::HelperInvoker::invoke](../../src/runtime_backends.rs#L142) | function_signature_item | `private` |  |
| [tools::runtime_backends::HelperClient::invoke](../../src/runtime_backends.rs#L150) | function_item | `private` |  |
| [tools::runtime_backends::Arc::invoke](../../src/runtime_backends.rs#L183) | function_item | `private` |  |
| [tools::runtime_backends::BoundedHelper](../../src/runtime_backends.rs#L192) | struct_item | `pub` |  |
| [tools::runtime_backends::BoundedHelper::new](../../src/runtime_backends.rs#L198) | function_item | `pub` |  |
| [tools::runtime_backends::BoundedHelper::invoke](../../src/runtime_backends.rs#L202) | function_item | `pub` |  |
| [tools::runtime_backends::validate_helper_limits](../../src/runtime_backends.rs#L227) | function_item | `private` |  |
| [tools::runtime_backends::JobSpec](../../src/runtime_backends.rs#L294) | struct_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy](../../src/runtime_backends.rs#L312) | struct_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy::new](../../src/runtime_backends.rs#L319) | function_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy::primary_workspace_cwd](../../src/runtime_backends.rs#L358) | function_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy::base_sandbox](../../src/runtime_backends.rs#L363) | function_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy::permitted_write_roots](../../src/runtime_backends.rs#L368) | function_item | `pub` |  |
| [tools::runtime_backends::JobLaunchPolicy::resolve](../../src/runtime_backends.rs#L372) | function_item | `private` |  |
| [tools::runtime_backends::JobState](../../src/runtime_backends.rs#L425) | enum_item | `pub` |  |
| [tools::runtime_backends::JobRecord](../../src/runtime_backends.rs#L435) | struct_item | `pub` |  |
| [tools::runtime_backends::JobRunnerRequest](../../src/runtime_backends.rs#L450) | struct_item | `private` |  |
| [tools::runtime_backends::SandboxedJobLauncher](../../src/runtime_backends.rs#L465) | trait_item | `pub` |  |
| [tools::runtime_backends::SandboxedJobLauncher::uses_pid_namespace](../../src/runtime_backends.rs#L469) | function_item | `private` |  |
| [tools::runtime_backends::SandboxedJobLauncher::runner_command](../../src/runtime_backends.rs#L473) | function_signature_item | `private` |  |
| [tools::runtime_backends::HelperJobLauncher](../../src/runtime_backends.rs#L484) | struct_item | `pub` |  |
| [tools::runtime_backends::HelperJobLauncher::new](../../src/runtime_backends.rs#L490) | function_item | `pub` |  |
| [tools::runtime_backends::HelperJobLauncher::disabled](../../src/runtime_backends.rs#L508) | function_item | `pub` |  |
| [tools::runtime_backends::HelperJobLauncher::uses_pid_namespace](../../src/runtime_backends.rs#L526) | function_item | `private` |  |
| [tools::runtime_backends::HelperJobLauncher::runner_command](../../src/runtime_backends.rs#L531) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker](../../src/runtime_backends.rs#L550) | struct_item | `pub` |  |
| [tools::runtime_backends::JobBroker::new](../../src/runtime_backends.rs#L558) | function_item | `pub` |  |
| [tools::runtime_backends::JobBroker::open](../../src/runtime_backends.rs#L562) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::start](../../src/runtime_backends.rs#L602) | function_item | `pub` |  |
| [tools::runtime_backends::JobBroker::start_inner](../../src/runtime_backends.rs#L639) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::status](../../src/runtime_backends.rs#L843) | function_item | `pub` |  |
| [tools::runtime_backends::JobBroker::status_inner](../../src/runtime_backends.rs#L847) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::list](../../src/runtime_backends.rs#L875) | function_item | `pub` |  |
| [tools::runtime_backends::JobBroker::list_inner](../../src/runtime_backends.rs#L879) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::stop](../../src/runtime_backends.rs#L894) | function_item | `pub` |  |
| [tools::runtime_backends::JobBroker::stop_inner](../../src/runtime_backends.rs#L909) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::job_directory](../../src/runtime_backends.rs#L966) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::host_receipt](../../src/runtime_backends.rs#L970) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::read_record](../../src/runtime_backends.rs#L996) | function_item | `private` |  |
| [tools::runtime_backends::JobBroker::write_record](../../src/runtime_backends.rs#L1016) | function_item | `private` |  |
| [tools::runtime_backends::run_job_runner](../../src/runtime_backends.rs#L1028) | function_item | `pub` |  |
| [tools::runtime_backends::HttpLimits](../../src/runtime_backends.rs#L1139) | struct_item | `pub` |  |
| [tools::runtime_backends::HttpLimits::default](../../src/runtime_backends.rs#L1146) | function_item | `private` |  |
| [tools::runtime_backends::HttpLimits::validate](../../src/runtime_backends.rs#L1156) | function_item | `private` |  |
| [tools::runtime_backends::HttpFetchResult](../../src/runtime_backends.rs#L1177) | struct_item | `pub` |  |
| [tools::runtime_backends::WebExtractionKind](../../src/runtime_backends.rs#L1187) | enum_item | `pub` |  |
| [tools::runtime_backends::WebExtraction](../../src/runtime_backends.rs#L1198) | struct_item | `pub` |  |
| [tools::runtime_backends::BoundedHttpClient](../../src/runtime_backends.rs#L1206) | struct_item | `pub` |  |
| [tools::runtime_backends::BoundedHttpClient::new](../../src/runtime_backends.rs#L1211) | function_item | `pub` |  |
| [tools::runtime_backends::BoundedHttpClient::fetch](../../src/runtime_backends.rs#L1216) | function_item | `pub` |  |
| [tools::runtime_backends::BoundedHttpClient::search](../../src/runtime_backends.rs#L1249) | function_item | `pub` |  |
| [tools::runtime_backends::SearchTopic](../../src/runtime_backends.rs#L1298) | enum_item | `pub` |  |
| [tools::runtime_backends::SearchRequest](../../src/runtime_backends.rs#L1304) | struct_item | `pub` |  |
| [tools::runtime_backends::SearchHit](../../src/runtime_backends.rs#L1313) | struct_item | `pub` |  |
| [tools::runtime_backends::SearchProvider](../../src/runtime_backends.rs#L1322) | trait_item | `pub` |  |
| [tools::runtime_backends::SearchProvider::search](../../src/runtime_backends.rs#L1323) | function_signature_item | `private` |  |
| [tools::runtime_backends::fetch_async](../../src/runtime_backends.rs#L1331) | function_item | `private` |  |
| [tools::runtime_backends::resolve_web_redirect](../../src/runtime_backends.rs#L1424) | function_item | `pub` |  |
| [tools::runtime_backends::await_cancel_deadline](../../src/runtime_backends.rs#L1433) | function_item | `private` |  |
| [tools::runtime_backends::PublicRoute](../../src/runtime_backends.rs#L1459) | enum_item | `pub` |  |
| [tools::runtime_backends::PublicRoute::client_builder](../../src/runtime_backends.rs#L1471) | function_item | `pub` |  |
| [tools::runtime_backends::route_public_url](../../src/runtime_backends.rs#L1486) | function_item | `pub` |  |
| [tools::runtime_backends::route_public_url_with](../../src/runtime_backends.rs#L1491) | function_item | `pub` |  |
| [tools::runtime_backends::outbound_proxy_for](../../src/runtime_backends.rs#L1571) | function_item | `private` |  |
| [tools::runtime_backends::no_proxy_exempts](../../src/runtime_backends.rs#L1605) | function_item | `private` |  |
| [tools::runtime_backends::cidr_contains](../../src/runtime_backends.rs#L1655) | function_item | `private` |  |
| [tools::runtime_backends::is_public_internet_address](../../src/runtime_backends.rs#L1681) | function_item | `pub` |  |
| [tools::runtime_backends::extract_web_content](../../src/runtime_backends.rs#L1748) | function_item | `pub` |  |
| [tools::runtime_backends::decode_web_text](../../src/runtime_backends.rs#L1794) | function_item | `private` |  |
| [tools::runtime_backends::looks_like_html](../../src/runtime_backends.rs#L1801) | function_item | `private` |  |
| [tools::runtime_backends::extract_html_text](../../src/runtime_backends.rs#L1810) | function_item | `private` |  |
| [tools::runtime_backends::extract_html_title](../../src/runtime_backends.rs#L1872) | function_item | `private` |  |
| [tools::runtime_backends::is_html_block](../../src/runtime_backends.rs#L1889) | function_item | `private` |  |
| [tools::runtime_backends::decode_html_entities](../../src/runtime_backends.rs#L1914) | function_item | `private` |  |
| [tools::runtime_backends::decode_numeric_entity](../../src/runtime_backends.rs#L1963) | function_item | `private` |  |
| [tools::runtime_backends::collapse_web_whitespace](../../src/runtime_backends.rs#L1973) | function_item | `private` |  |
| [tools::runtime_backends::html_is_under_rendered](../../src/runtime_backends.rs#L1994) | function_item | `private` |  |
| [tools::runtime_backends::truncate_characters](../../src/runtime_backends.rs#L2006) | function_item | `private` |  |
| [tools::runtime_backends::require_canonical_directory](../../src/runtime_backends.rs#L2019) | function_item | `private` |  |
| [tools::runtime_backends::canonical_invocation_directory](../../src/runtime_backends.rs#L2046) | function_item | `private` |  |
| [tools::runtime_backends::normalize_policy_roots](../../src/runtime_backends.rs#L2081) | function_item | `private` |  |
| [tools::runtime_backends::covered_by](../../src/runtime_backends.rs#L2105) | function_item | `private` |  |
| [tools::runtime_backends::validate_job_id](../../src/runtime_backends.rs#L2111) | function_item | `private` |  |
| [tools::runtime_backends::drain_tail](../../src/runtime_backends.rs#L2125) | function_item | `private` |  |
| [tools::runtime_backends::signal_group](../../src/runtime_backends.rs#L2143) | function_item | `private` |  |
| [tools::runtime_backends::terminate_group](../../src/runtime_backends.rs#L2160) | function_item | `private` |  |
| [tools::runtime_backends::JobHostReceipt](../../src/runtime_backends.rs#L2178) | struct_item | `private` |  |
| [tools::runtime_backends::NamespaceRunnerIdentity](../../src/runtime_backends.rs#L2187) | struct_item | `private` |  |
| [tools::runtime_backends::LinuxProcessIdentity](../../src/runtime_backends.rs#L2196) | struct_item | `private` |  |
| [tools::runtime_backends::LinuxProcessIdentity::read](../../src/runtime_backends.rs#L2209) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::LinuxProcessIdentity::same_namespace](../../src/runtime_backends.rs#L2235) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::LinuxProcessIdentity::is_live](../../src/runtime_backends.rs#L2240) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceRunnerIdentity::matches_record](../../src/runtime_backends.rs#L2251) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceRunnerIdentity::matches_current_runner](../../src/runtime_backends.rs#L2262) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceRunnerIdentity::namespace_is_live](../../src/runtime_backends.rs#L2270) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceRunnerIdentity::stop_namespace](../../src/runtime_backends.rs#L2274) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceRunnerIdentity::stop_handle](../../src/runtime_backends.rs#L2281) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceStopHandle](../../src/runtime_backends.rs#L2334) | struct_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceStopHandle::kill](../../src/runtime_backends.rs#L2338) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::NamespaceStopHandle::send_signal](../../src/runtime_backends.rs#L2342) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::namespace_identity_channel](../../src/runtime_backends.rs#L2368) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::announce_namespace_runner](../../src/runtime_backends.rs#L2398) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::namespace_runner_peer](../../src/runtime_backends.rs#L2409) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::authenticate_namespace_runner](../../src/runtime_backends.rs#L2460) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::process_start_token](../../src/runtime_backends.rs#L2496) | function_item | `private` | #[cfg(target_os = "macos")] |
| [tools::runtime_backends::process_start_token](../../src/runtime_backends.rs#L2521) | function_item | `private` | #[cfg(target_os = "linux")] |
| [tools::runtime_backends::process_start_token](../../src/runtime_backends.rs#L2531) | function_item | `private` | #[cfg(not(any(target_os = "macos", target_os = "linux")))] |
| [tools::runtime_backends::unix_millis](../../src/runtime_backends.rs#L2535) | function_item | `private` |  |
| [tools::runtime_backends::sync_directory](../../src/runtime_backends.rs#L2543) | function_item | `private` |  |
| [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) | function_item | `private` |  |
| [tools::runtime_backends::store_failure](../../src/runtime_backends.rs#L2553) | function_item | `private` |  |
| [tools::runtime_backends::disabled_job_launcher_tests::mcp_only_launcher_refuses_command_execution](../../src/runtime_backends.rs#L2564) | function_item | `private` | test; #[cfg(test)] |
| [tools::runtime_backends::namespace_job_identity_tests::inherited_socket_reports_kernel_sender_identity](../../src/runtime_backends.rs#L2594) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::namespace_authentication_refuses_an_unrelated_sender](../../src/runtime_backends.rs#L2601) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::runner_identity_requires_matching_pid_start_and_namespace](../../src/runtime_backends.rs#L2612) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::stale_namespace_identity_never_signals_a_live_process](../../src/runtime_backends.rs#L2632) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::forged_namespace_receipt_cannot_select_a_host_process](../../src/runtime_backends.rs#L2650) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::test_record](../../src/runtime_backends.rs#L2664) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::sandbox_written_pid_and_job_id_cannot_replace_host_authority](../../src/runtime_backends.rs#L2686) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::writable_root_cannot_cover_private_launch_receipts](../../src/runtime_backends.rs#L2721) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::writable_root_cannot_cover_private_launch_receipts::NeverLaunch](../../src/runtime_backends.rs#L2722) | struct_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::namespace_job_identity_tests::writable_root_cannot_cover_private_launch_receipts::NeverLaunch::runner_command](../../src/runtime_backends.rs#L2724) | function_item | `private` | test; #[cfg(all(test, target_os = "linux"))] |
| [tools::runtime_backends::web_fetch_status_tests::non_success_statuses_classify_by_what_the_server_said](../../src/runtime_backends.rs#L2769) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `VecDeque` | `std::collections::VecDeque` | `private` |
| `fs` | `std::fs` | `private` |
| `File` | `std::fs::File` | `private` |
| `Read` | `std::io::Read` | `private` |
| `IpAddr` | `std::net::IpAddr` | `private` |
| `SocketAddr` | `std::net::SocketAddr` | `private` |
| `ToSocketAddrs` | `std::net::ToSocketAddrs` | `private` |
| `_` | `std::os::unix::process::CommandExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Child` | `std::process::Child` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Stdio` | `std::process::Stdio` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `thread` | `std::thread` | `private` |
| `Duration` | `std::time::Duration` | `private` |
| `Instant` | `std::time::Instant` | `private` |
| `SystemTime` | `std::time::SystemTime` | `private` |
| `UNIX_EPOCH` | `std::time::UNIX_EPOCH` | `private` |
| `ACCEPT` | `reqwest::header::ACCEPT` | `private` |
| `CONTENT_LENGTH` | `reqwest::header::CONTENT_LENGTH` | `private` |
| `CONTENT_TYPE` | `reqwest::header::CONTENT_TYPE` | `private` |
| `LOCATION` | `reqwest::header::LOCATION` | `private` |
| `USER_AGENT` | `reqwest::header::USER_AGENT` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `AtomicPublisher` | `store::AtomicPublisher` | `private` |
| `NamedLock` | `store::NamedLock` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `HelperClient` | `crate::helper::HelperClient` | `private` |
| `HelperClientError` | `crate::helper::HelperClientError` | `private` |
| `HelperErrorClass` | `crate::helper::HelperErrorClass` | `private` |
| `HelperOperation` | `crate::helper::HelperOperation` | `private` |
| `HelperRequest` | `crate::helper::HelperRequest` | `private` |
| `HelperResponse` | `crate::helper::HelperResponse` | `private` |
| `ProbeStatus` | `crate::sandbox::ProbeStatus` | `private` |
| `SandboxPolicy` | `crate::sandbox::SandboxPolicy` | `private` |
| `sandbox_command` | `crate::sandbox::sandbox_command` | `private` |
| `BackendFailure` | `super::BackendFailure` | `private` |
| `HelperJobLauncher` | `super::HelperJobLauncher` | `private` |
| `JobSpec` | `super::JobSpec` | `private` |
| `SandboxedJobLauncher` | `super::SandboxedJobLauncher` | `private` |
| `NetworkPolicy` | `crate::NetworkPolicy` | `private` |
| `SandboxPolicy` | `crate::SandboxPolicy` | `private` |
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `*` | `super::*` | `private` |
| `Write` | `std::io::Write` | `private` |
| `BackendFailure` | `super::BackendFailure` | `private` |
| `web_fetch_status_failure` | `super::web_fetch_status_failure` | `private` |
| `StatusCode` | `reqwest::StatusCode` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tools::runtime_backends::disabled_job_launcher_tests` | `private` | #[cfg(test)] |
| `tools::runtime_backends::namespace_job_identity_tests` | `private` | #[cfg(all(test, target_os = "linux"))] |
| `tools::runtime_backends::web_fetch_status_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 10 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::bubblewrap_sandbox::detached_command"]
  n1["tools::runtime_backends::BackendOutcome::enter"]
  n2["tools::runtime_backends::CancellationToken::cancel"]
  n3["tools::runtime_backends::CancellationToken::is_cancelled"]
  n4["tools::runtime_backends::HelperClient::invoke"]
  n5["tools::runtime_backends::Arc::invoke"]
  n6["tools::runtime_backends::BoundedHelper::new"]
  n7["tools::runtime_backends::require_canonical_directory"]
  n8["tools::runtime_backends::BoundedHelper::invoke"]
  n9["tools::runtime_backends::canonical_invocation_directory"]
  n10["tools::runtime_backends::normalize_policy_roots"]
  n11["tools::runtime_backends::covered_by"]
  n12["tools::runtime_backends::validate_helper_limits"]
  n13["tools::runtime_backends::JobLaunchPolicy::new"]
  n14["tools::runtime_backends::JobLaunchPolicy::primary_workspace_cwd"]
  n15["tools::runtime_backends::JobLaunchPolicy::base_sandbox"]
  n16["tools::runtime_backends::JobLaunchPolicy::permitted_write_roots"]
  n17["tools::runtime_backends::JobLaunchPolicy::resolve"]
  n18["tools::runtime_backends::SandboxedJobLauncher::uses_pid_namespace"]
  n19["tools::runtime_backends::HelperJobLauncher::new"]
  n20["tools::runtime_backends::HelperJobLauncher::disabled"]
  n21["tools::runtime_backends::HelperJobLauncher::uses_pid_namespace"]
  n22["tools::runtime_backends::HelperJobLauncher::runner_command"]
  n23["tools::runtime_backends::JobBroker::new"]
  n24["tools::runtime_backends::JobBroker::open"]
  n25["tools::runtime_backends::web_fetch_status_failure"]
  n26["tools::sandbox::sandbox_command"]
  n8 --> n1
  n8 --> n12
  n13 --> n7
  n13 --> n10
  n13 --> n11
  n17 --> n9
  n17 --> n11
  n22 --> n0
  n22 --> n26
  n23 --> n24
```

</details>

<details><summary>Functions 21–40: 49 direct edges</summary>

```mermaid
flowchart TD
  n0["store::atomic::AtomicPublisher::replace"]
  n1["store::platform::NamedLock::exclusive"]
  n2["store::platform::NamedLock::shared"]
  n3["tools::runtime_backends::JobBroker::write_record"]
  n4["tools::runtime_backends::run_job_runner"]
  n5["tools::runtime_backends::HttpLimits::default"]
  n6["tools::runtime_backends::HttpLimits::validate"]
  n7["tools::runtime_backends::BackendOutcome::enter"]
  n8["tools::runtime_backends::BoundedHttpClient::new"]
  n9["tools::runtime_backends::BoundedHttpClient::fetch"]
  n10["tools::runtime_backends::BoundedHttpClient::search"]
  n11["tools::runtime_backends::fetch_async"]
  n12["tools::runtime_backends::resolve_web_redirect"]
  n13["tools::runtime_backends::await_cancel_deadline"]
  n14["tools::runtime_backends::route_public_url"]
  n15["tools::runtime_backends::extract_web_content"]
  n16["tools::runtime_backends::require_canonical_directory"]
  n17["tools::runtime_backends::validate_job_id"]
  n18["tools::runtime_backends::drain_tail"]
  n19["tools::runtime_backends::signal_group"]
  n20["tools::runtime_backends::terminate_group"]
  n21["tools::runtime_backends::namespace_identity_channel"]
  n22["tools::runtime_backends::announce_namespace_runner"]
  n23["tools::runtime_backends::authenticate_namespace_runner"]
  n24["tools::runtime_backends::unix_millis"]
  n25["tools::runtime_backends::sync_directory"]
  n26["tools::runtime_backends::io_failure"]
  n27["tools::runtime_backends::store_failure"]
  n28["tools::runtime_backends::JobBroker::open"]
  n29["tools::runtime_backends::JobBroker::start"]
  n30["tools::runtime_backends::JobBroker::start_inner"]
  n31["tools::runtime_backends::web_fetch_status_failure"]
  n32["tools::runtime_backends::JobBroker::status"]
  n33["tools::runtime_backends::JobBroker::status_inner"]
  n34["tools::runtime_backends::JobBroker::list"]
  n35["tools::runtime_backends::JobBroker::list_inner"]
  n36["tools::runtime_backends::JobBroker::stop"]
  n37["tools::runtime_backends::JobBroker::stop_inner"]
  n38["tools::runtime_backends::JobBroker::job_directory"]
  n39["tools::runtime_backends::JobBroker::host_receipt"]
  n40["tools::runtime_backends::JobBroker::read_record"]
  n3 --> n0
  n3 --> n1
  n3 --> n38
  n4 --> n17
  n4 --> n18
  n4 --> n22
  n4 --> n24
  n4 --> n28
  n9 --> n7
  n9 --> n11
  n10 --> n7
  n11 --> n12
  n11 --> n13
  n11 --> n14
  n11 --> n15
  n11 --> n31
  n29 --> n7
  n29 --> n17
  n29 --> n30
  n30 --> n0
  n30 --> n3
  n30 --> n16
  n30 --> n20
  n30 --> n21
  n30 --> n23
  n30 --> n24
  n30 --> n25
  n30 --> n26
  n30 --> n27
  n30 --> n38
  n32 --> n33
  n33 --> n3
  n33 --> n17
  n33 --> n24
  n33 --> n39
  n33 --> n40
  n34 --> n35
  n35 --> n17
  n35 --> n33
  n36 --> n7
  n36 --> n37
  n37 --> n3
  n37 --> n19
  n37 --> n24
  n37 --> n33
  n37 --> n39
  n40 --> n2
  n40 --> n26
  n40 --> n38
```

</details>

<details><summary>Functions 41–60: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::runtime_backends::resolve_web_redirect"]
  n1["tools::runtime_backends::await_cancel_deadline"]
  n2["tools::runtime_backends::PublicRoute::client_builder"]
  n3["tools::runtime_backends::route_public_url"]
  n4["tools::runtime_backends::route_public_url_with"]
  n5["tools::runtime_backends::outbound_proxy_for"]
  n6["tools::runtime_backends::no_proxy_exempts"]
  n7["tools::runtime_backends::cidr_contains"]
  n8["tools::runtime_backends::is_public_internet_address"]
  n9["tools::runtime_backends::extract_web_content"]
  n10["tools::runtime_backends::decode_web_text"]
  n11["tools::runtime_backends::looks_like_html"]
  n12["tools::runtime_backends::extract_html_text"]
  n13["tools::runtime_backends::extract_html_title"]
  n14["tools::runtime_backends::is_html_block"]
  n15["tools::runtime_backends::decode_html_entities"]
  n16["tools::runtime_backends::decode_numeric_entity"]
  n17["tools::runtime_backends::collapse_web_whitespace"]
  n18["tools::runtime_backends::html_is_under_rendered"]
  n19["tools::runtime_backends::truncate_characters"]
  n3 --> n4
  n4 --> n5
  n4 --> n8
  n5 --> n6
  n6 --> n7
  n8 --> n8
  n9 --> n10
  n9 --> n11
  n9 --> n12
  n9 --> n18
  n9 --> n19
  n12 --> n13
  n12 --> n14
  n12 --> n15
  n12 --> n17
  n15 --> n16
```

</details>

<details><summary>Functions 61–80: 12 direct edges</summary>

```mermaid
flowchart TD
  n0["store::atomic::AtomicPublisher::replace"]
  n1["tools::runtime_backends::require_canonical_directory"]
  n2["tools::runtime_backends::canonical_invocation_directory"]
  n3["tools::runtime_backends::normalize_policy_roots"]
  n4["tools::runtime_backends::covered_by"]
  n5["tools::runtime_backends::validate_job_id"]
  n6["tools::runtime_backends::drain_tail"]
  n7["tools::runtime_backends::signal_group"]
  n8["tools::runtime_backends::terminate_group"]
  n9["tools::runtime_backends::LinuxProcessIdentity::read"]
  n10["tools::runtime_backends::LinuxProcessIdentity::same_namespace"]
  n11["tools::runtime_backends::LinuxProcessIdentity::is_live"]
  n12["tools::runtime_backends::NamespaceRunnerIdentity::matches_record"]
  n13["tools::runtime_backends::NamespaceRunnerIdentity::matches_current_runner"]
  n14["tools::runtime_backends::NamespaceRunnerIdentity::namespace_is_live"]
  n15["tools::runtime_backends::NamespaceRunnerIdentity::stop_namespace"]
  n16["tools::runtime_backends::NamespaceRunnerIdentity::stop_handle"]
  n17["tools::runtime_backends::NamespaceStopHandle::kill"]
  n18["tools::runtime_backends::NamespaceStopHandle::send_signal"]
  n19["tools::runtime_backends::namespace_identity_channel"]
  n20["tools::runtime_backends::announce_namespace_runner"]
  n21["tools::runtime_backends::io_failure"]
  n3 --> n1
  n3 --> n4
  n6 --> n0
  n7 --> n21
  n8 --> n7
  n11 --> n9
  n13 --> n9
  n15 --> n16
  n16 --> n9
  n17 --> n18
  n18 --> n21
  n19 --> n21
```

</details>

<details><summary>Functions 81–89: 3 direct edges</summary>

```mermaid
flowchart TD
  n0["tools::runtime_backends::LinuxProcessIdentity::read"]
  n1["tools::runtime_backends::namespace_runner_peer"]
  n2["tools::runtime_backends::authenticate_namespace_runner"]
  n3["tools::runtime_backends::process_start_token"]
  n4["tools::runtime_backends::process_start_token"]
  n5["tools::runtime_backends::process_start_token"]
  n6["tools::runtime_backends::unix_millis"]
  n7["tools::runtime_backends::sync_directory"]
  n8["tools::runtime_backends::io_failure"]
  n9["tools::runtime_backends::store_failure"]
  n1 --> n8
  n2 --> n0
  n2 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `web_fetch_status_failure` | `status.as_u16` | [75](../../src/runtime_backends.rs#L75) | receiver-type-required |
| `web_fetch_status_failure` | `BackendFailure::NotFound` | [77](../../src/runtime_backends.rs#L77) | external-constructor-callback-or-unresolved |
| `web_fetch_status_failure` | `BackendFailure::Denied` | [78](../../src/runtime_backends.rs#L78) | external-constructor-callback-or-unresolved |
| `web_fetch_status_failure` | `BackendFailure::Limit` | [80](../../src/runtime_backends.rs#L80) | external-constructor-callback-or-unresolved |
| `web_fetch_status_failure` | `status.is_client_error` | [81](../../src/runtime_backends.rs#L81) | receiver-type-required |
| `web_fetch_status_failure` | `BackendFailure::Invalid` | [81](../../src/runtime_backends.rs#L81) | external-constructor-callback-or-unresolved |
| `web_fetch_status_failure` | `status.is_server_error` | [84](../../src/runtime_backends.rs#L84) | receiver-type-required |
| `web_fetch_status_failure` | `BackendFailure::Unavailable` | [84](../../src/runtime_backends.rs#L84) | external-constructor-callback-or-unresolved |
| `web_fetch_status_failure` | `BackendFailure::Protocol` | [85](../../src/runtime_backends.rs#L85) | external-constructor-callback-or-unresolved |
| `enter` | `Ok` | [120](../../src/runtime_backends.rs#L120) | external-constructor-callback-or-unresolved |
| `enter` | `Err` | [121](../../src/runtime_backends.rs#L121), [122](../../src/runtime_backends.rs#L122) | external-constructor-callback-or-unresolved |
| `enter` | `Self::Hold` | [121](../../src/runtime_backends.rs#L121) | external-constructor-callback-or-unresolved |
| `enter` | `Self::Unavailable` | [122](../../src/runtime_backends.rs#L122) | external-constructor-callback-or-unresolved |
| `cancel` | `self.0.store` | [132](../../src/runtime_backends.rs#L132) | receiver-type-required |
| `is_cancelled` | `self.0.load` | [137](../../src/runtime_backends.rs#L137) | receiver-type-required |
| `invoke` | `exec.timeout_ms.map` | [159](../../src/runtime_backends.rs#L159) | receiver-type-required |
| `invoke` | `Duration::from_millis` | [160](../../src/runtime_backends.rs#L160) | external-constructor-callback-or-unresolved |
| `invoke` | `timeout.saturating_add` | [160](../../src/runtime_backends.rs#L160) | receiver-type-required |
| `invoke` | `Some` | [162](../../src/runtime_backends.rs#L162) | external-constructor-callback-or-unresolved |
| `invoke` | `Duration::from_secs` | [162](../../src/runtime_backends.rs#L162) | external-constructor-callback-or-unresolved |
| `invoke` | `self.execute_cancellable(request, timeout, &#124;&#124; cancellation.is_cancelled())             .map_err` | [164](../../src/runtime_backends.rs#L164) | receiver-type-required |
| `invoke` | `self.execute_cancellable` | [164](../../src/runtime_backends.rs#L164) | receiver-type-required |
| `invoke` | `cancellation.is_cancelled` | [164](../../src/runtime_backends.rs#L164) | receiver-type-required |
| `invoke` | `BackendFailure::Limit` | [169](../../src/runtime_backends.rs#L169) | external-constructor-callback-or-unresolved |
| `invoke` | `BackendFailure::Denied` | [170](../../src/runtime_backends.rs#L170) | external-constructor-callback-or-unresolved |
| `invoke` | `BackendFailure::NotFound` | [171](../../src/runtime_backends.rs#L171) | external-constructor-callback-or-unresolved |
| `invoke` | `BackendFailure::Unavailable` | [173](../../src/runtime_backends.rs#L173) | external-constructor-callback-or-unresolved |
| `invoke` | `BackendFailure::Protocol` | [175](../../src/runtime_backends.rs#L175) | external-constructor-callback-or-unresolved |
| `invoke` | `BackendFailure::Io` | [176](../../src/runtime_backends.rs#L176) | external-constructor-callback-or-unresolved |
| `invoke` | `error.to_string` | [176](../../src/runtime_backends.rs#L176) | receiver-type-required |
| `invoke` | `(**self).invoke` | [188](../../src/runtime_backends.rs#L188) | receiver-type-required |
| `invoke` | `BackendOutcome::enter` | [208](../../src/runtime_backends.rs#L208) | [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) |
| `invoke` | `cancellation.is_cancelled` | [211](../../src/runtime_backends.rs#L211) | receiver-type-required |
| `invoke` | `BackendOutcome::Completed` | [212](../../src/runtime_backends.rs#L212), [215](../../src/runtime_backends.rs#L215), [223](../../src/runtime_backends.rs#L223) | external-constructor-callback-or-unresolved |
| `invoke` | `Err` | [212](../../src/runtime_backends.rs#L212), [215](../../src/runtime_backends.rs#L215) | external-constructor-callback-or-unresolved |
| `invoke` | `validate_helper_limits` | [214](../../src/runtime_backends.rs#L214) | [tools::runtime_backends::validate_helper_limits](../../src/runtime_backends.rs#L227) |
| `invoke` | `BackendOutcome::Unavailable` | [218](../../src/runtime_backends.rs#L218) | external-constructor-callback-or-unresolved |
| `invoke` | `"exec-helper".to_owned` | [219](../../src/runtime_backends.rs#L219) | receiver-type-required |
| `invoke` | `"helper process is not configured".to_owned` | [220](../../src/runtime_backends.rs#L220) | receiver-type-required |
| `invoke` | `invoker.invoke` | [223](../../src/runtime_backends.rs#L223) | receiver-type-required |
| `validate_helper_limits` | `Err` | [230](../../src/runtime_backends.rs#L230), [251](../../src/runtime_backends.rs#L251), [259](../../src/runtime_backends.rs#L259), [270](../../src/runtime_backends.rs#L270), [282](../../src/runtime_backends.rs#L282) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `BackendFailure::Limit` | [230](../../src/runtime_backends.rs#L230), [251](../../src/runtime_backends.rs#L251), [259](../../src/runtime_backends.rs#L259), [270](../../src/runtime_backends.rs#L270), [282](../../src/runtime_backends.rs#L282) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `Ok` | [234](../../src/runtime_backends.rs#L234), [253](../../src/runtime_backends.rs#L253), [263](../../src/runtime_backends.rs#L263), [274](../../src/runtime_backends.rs#L274), [286](../../src/runtime_backends.rs#L286) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `bounded` | [239](../../src/runtime_backends.rs#L239), [241](../../src/runtime_backends.rs#L241), [248](../../src/runtime_backends.rs#L248), [249](../../src/runtime_backends.rs#L249), [256](../../src/runtime_backends.rs#L256), [257](../../src/runtime_backends.rs#L257) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `"grep context limit exceeded".into` | [251](../../src/runtime_backends.rs#L251) | receiver-type-required |
| `validate_helper_limits` | `Some` | [258](../../src/runtime_backends.rs#L258) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `"timeout_ms must be at least 1".into` | [260](../../src/runtime_backends.rs#L260) | receiver-type-required |
| `validate_helper_limits` | `content                 .decode()                 .map_err` | [266](../../src/runtime_backends.rs#L266) | receiver-type-required |
| `validate_helper_limits` | `content                 .decode` | [266](../../src/runtime_backends.rs#L266) | receiver-type-required |
| `validate_helper_limits` | `BackendFailure::Invalid` | [268](../../src/runtime_backends.rs#L268), [280](../../src/runtime_backends.rs#L280) | external-constructor-callback-or-unresolved |
| `validate_helper_limits` | `error.to_string` | [268](../../src/runtime_backends.rs#L268), [280](../../src/runtime_backends.rs#L280) | receiver-type-required |
| `validate_helper_limits` | `bytes.len` | [269](../../src/runtime_backends.rs#L269), [281](../../src/runtime_backends.rs#L281) | receiver-type-required |
| `validate_helper_limits` | `"content exceeds helper byte limit".into` | [271](../../src/runtime_backends.rs#L271) | receiver-type-required |
| `validate_helper_limits` | `replacement                 .decode()                 .map_err` | [278](../../src/runtime_backends.rs#L278) | receiver-type-required |
| `validate_helper_limits` | `replacement                 .decode` | [278](../../src/runtime_backends.rs#L278) | receiver-type-required |
| `validate_helper_limits` | `"replacement exceeds helper byte limit".into` | [283](../../src/runtime_backends.rs#L283) | receiver-type-required |
| `new` | `base_sandbox             .validate()             .map_err` | [324](../../src/runtime_backends.rs#L324) | receiver-type-required |
| `new` | `base_sandbox             .validate` | [324](../../src/runtime_backends.rs#L324) | receiver-type-required |
| `new` | `BackendFailure::Invalid` | [326](../../src/runtime_backends.rs#L326) | external-constructor-callback-or-unresolved |
| `new` | `error.to_string` | [326](../../src/runtime_backends.rs#L326) | receiver-type-required |
| `new` | `require_canonical_directory` | [328](../../src/runtime_backends.rs#L328), [332](../../src/runtime_backends.rs#L332), [335](../../src/runtime_backends.rs#L335), [338](../../src/runtime_backends.rs#L338) | [tools::runtime_backends::require_canonical_directory](../../src/runtime_backends.rs#L2019) |
| `new` | `primary_workspace_cwd.into` | [328](../../src/runtime_backends.rs#L328) | receiver-type-required |
| `new` | `base_sandbox             .read_roots             .iter()             .map(&#124;root&#124; require_canonical_directory(root, "sandbox read root"))             .collect::<Result<Vec<_>, _>>` | [329](../../src/runtime_backends.rs#L329) | receiver-type-required |
| `new` | `base_sandbox             .read_roots             .iter()             .map` | [329](../../src/runtime_backends.rs#L329) | receiver-type-required |
| `new` | `base_sandbox             .read_roots             .iter` | [329](../../src/runtime_backends.rs#L329) | receiver-type-required |
| `new` | `covered_by` | [340](../../src/runtime_backends.rs#L340) | [tools::runtime_backends::covered_by](../../src/runtime_backends.rs#L2105) |
| `new` | `Err` | [341](../../src/runtime_backends.rs#L341) | external-constructor-callback-or-unresolved |
| `new` | `BackendFailure::Denied` | [341](../../src/runtime_backends.rs#L341) | external-constructor-callback-or-unresolved |
| `new` | `"primary workspace cwd is outside sandbox read roots".to_owned` | [342](../../src/runtime_backends.rs#L342) | receiver-type-required |
| `new` | `normalize_policy_roots` | [345](../../src/runtime_backends.rs#L345) | [tools::runtime_backends::normalize_policy_roots](../../src/runtime_backends.rs#L2081) |
| `new` | `Ok` | [350](../../src/runtime_backends.rs#L350) | external-constructor-callback-or-unresolved |
| `resolve` | `spec.working_directory.as_deref` | [373](../../src/runtime_backends.rs#L373) | receiver-type-required |
| `resolve` | `canonical_invocation_directory` | [374](../../src/runtime_backends.rs#L374), [389](../../src/runtime_backends.rs#L389) | [tools::runtime_backends::canonical_invocation_directory](../../src/runtime_backends.rs#L2046) |
| `resolve` | `self.primary_workspace_cwd.clone` | [379](../../src/runtime_backends.rs#L379) | receiver-type-required |
| `resolve` | `covered_by` | [381](../../src/runtime_backends.rs#L381), [394](../../src/runtime_backends.rs#L394) | [tools::runtime_backends::covered_by](../../src/runtime_backends.rs#L2105) |
| `resolve` | `Err` | [382](../../src/runtime_backends.rs#L382), [395](../../src/runtime_backends.rs#L395), [400](../../src/runtime_backends.rs#L400) | external-constructor-callback-or-unresolved |
| `resolve` | `BackendFailure::Denied` | [382](../../src/runtime_backends.rs#L382), [395](../../src/runtime_backends.rs#L395) | external-constructor-callback-or-unresolved |
| `resolve` | `"job working directory is outside sandbox read roots".to_owned` | [383](../../src/runtime_backends.rs#L383) | receiver-type-required |
| `resolve` | `Vec::with_capacity` | [387](../../src/runtime_backends.rs#L387) | external-constructor-callback-or-unresolved |
| `resolve` | `spec.writable_paths.len` | [387](../../src/runtime_backends.rs#L387) | receiver-type-required |
| `resolve` | `requested.iter().any` | [399](../../src/runtime_backends.rs#L399) | receiver-type-required |
| `resolve` | `requested.iter` | [399](../../src/runtime_backends.rs#L399) | receiver-type-required |
| `resolve` | `BackendFailure::Invalid` | [400](../../src/runtime_backends.rs#L400), [418](../../src/runtime_backends.rs#L418) | external-constructor-callback-or-unresolved |
| `resolve` | `"job writable paths contain duplicate canonical roots".to_owned` | [401](../../src/runtime_backends.rs#L401) | receiver-type-required |
| `resolve` | `requested.push` | [404](../../src/runtime_backends.rs#L404) | receiver-type-required |
| `resolve` | `requested.sort_by` | [406](../../src/runtime_backends.rs#L406) | receiver-type-required |
| `resolve` | `left.as_bytes().cmp` | [406](../../src/runtime_backends.rs#L406), [414](../../src/runtime_backends.rs#L414) | receiver-type-required |
| `resolve` | `left.as_bytes` | [406](../../src/runtime_backends.rs#L406), [414](../../src/runtime_backends.rs#L414) | receiver-type-required |
| `resolve` | `right.as_bytes` | [406](../../src/runtime_backends.rs#L406), [414](../../src/runtime_backends.rs#L414) | receiver-type-required |
| `resolve` | `Some` | [408](../../src/runtime_backends.rs#L408) | external-constructor-callback-or-unresolved |
| `resolve` | `requested.clone` | [409](../../src/runtime_backends.rs#L409) | receiver-type-required |
| `resolve` | `self.base_sandbox.clone` | [410](../../src/runtime_backends.rs#L410) | receiver-type-required |
| `resolve` | `effective.write_roots.extend` | [411](../../src/runtime_backends.rs#L411) | receiver-type-required |
| `resolve` | `effective             .write_roots             .sort_by` | [412](../../src/runtime_backends.rs#L412) | receiver-type-required |
| `resolve` | `effective.write_roots.dedup` | [415](../../src/runtime_backends.rs#L415) | receiver-type-required |
| `resolve` | `effective             .validate()             .map_err` | [416](../../src/runtime_backends.rs#L416) | receiver-type-required |
| `resolve` | `effective             .validate` | [416](../../src/runtime_backends.rs#L416) | receiver-type-required |
| `resolve` | `error.to_string` | [418](../../src/runtime_backends.rs#L418) | receiver-type-required |
| `resolve` | `Ok` | [419](../../src/runtime_backends.rs#L419) | external-constructor-callback-or-unresolved |
| `new` | `executable.into` | [491](../../src/runtime_backends.rs#L491) | receiver-type-required |
| `new` | `executable.is_absolute` | [492](../../src/runtime_backends.rs#L492) | receiver-type-required |
| `new` | `executable.is_file` | [492](../../src/runtime_backends.rs#L492) | receiver-type-required |
| `new` | `Err` | [493](../../src/runtime_backends.rs#L493), [498](../../src/runtime_backends.rs#L498) | external-constructor-callback-or-unresolved |
| `new` | `BackendFailure::Unavailable` | [493](../../src/runtime_backends.rs#L493), [498](../../src/runtime_backends.rs#L498) | external-constructor-callback-or-unresolved |
| `new` | `"tekes-helper executable is not an absolute regular file".to_owned` | [494](../../src/runtime_backends.rs#L494) | receiver-type-required |
| `new` | `Ok` | [502](../../src/runtime_backends.rs#L502) | external-constructor-callback-or-unresolved |
| `disabled` | `executable.into` | [509](../../src/runtime_backends.rs#L509) | receiver-type-required |
| `disabled` | `executable.is_absolute` | [510](../../src/runtime_backends.rs#L510) | receiver-type-required |
| `disabled` | `executable.is_file` | [510](../../src/runtime_backends.rs#L510) | receiver-type-required |
| `disabled` | `Err` | [511](../../src/runtime_backends.rs#L511) | external-constructor-callback-or-unresolved |
| `disabled` | `BackendFailure::Unavailable` | [511](../../src/runtime_backends.rs#L511) | external-constructor-callback-or-unresolved |
| `disabled` | `"tekes-helper executable is not an absolute regular file".to_owned` | [512](../../src/runtime_backends.rs#L512) | receiver-type-required |
| `disabled` | `Ok` | [515](../../src/runtime_backends.rs#L515) | external-constructor-callback-or-unresolved |
| `disabled` | `"per-job sandbox unavailable in MCP-only App Sandbox host".to_owned` | [519](../../src/runtime_backends.rs#L519) | receiver-type-required |
| `runner_command` | `self.uses_pid_namespace` | [540](../../src/runtime_backends.rs#L540) | receiver-type-required |
| `runner_command` | `crate::bubblewrap_sandbox::detached_command(effective_policy, &self.executable)                 .map_err` | [541](../../src/runtime_backends.rs#L541) | receiver-type-required |
| `runner_command` | `crate::bubblewrap_sandbox::detached_command` | [541](../../src/runtime_backends.rs#L541) | [tools::bubblewrap_sandbox::detached_command](../../src/bubblewrap_sandbox.rs#L63) |
| `runner_command` | `BackendFailure::Unavailable` | [542](../../src/runtime_backends.rs#L542), [545](../../src/runtime_backends.rs#L545) | external-constructor-callback-or-unresolved |
| `runner_command` | `error.to_string` | [542](../../src/runtime_backends.rs#L542), [545](../../src/runtime_backends.rs#L545) | receiver-type-required |
| `runner_command` | `sandbox_command(effective_policy, &self.probe, &self.executable)             .map_err` | [544](../../src/runtime_backends.rs#L544) | receiver-type-required |
| `runner_command` | `sandbox_command` | [544](../../src/runtime_backends.rs#L544) | [tools::sandbox::sandbox_command](../../src/sandbox.rs#L289) |
| `new` | `Self::open` | [559](../../src/runtime_backends.rs#L559) | [tools::runtime_backends::JobBroker::open](../../src/runtime_backends.rs#L562) |
| `new` | `root.into` | [559](../../src/runtime_backends.rs#L559) | receiver-type-required |
| `open` | `Err` | [568](../../src/runtime_backends.rs#L568), [578](../../src/runtime_backends.rs#L578), [589](../../src/runtime_backends.rs#L589) | external-constructor-callback-or-unresolved |
| `open` | `BackendFailure::Limit` | [568](../../src/runtime_backends.rs#L568) | external-constructor-callback-or-unresolved |
| `open` | `fs::create_dir_all(&root).map_err` | [572](../../src/runtime_backends.rs#L572) | receiver-type-required |
| `open` | `fs::create_dir_all` | [572](../../src/runtime_backends.rs#L572), [583](../../src/runtime_backends.rs#L583) | external-constructor-callback-or-unresolved |
| `open` | `fs::symlink_metadata(&root)             .map_err(io_failure)?             .file_type()             .is_symlink` | [573](../../src/runtime_backends.rs#L573) | receiver-type-required |
| `open` | `fs::symlink_metadata(&root)             .map_err(io_failure)?             .file_type` | [573](../../src/runtime_backends.rs#L573) | receiver-type-required |
| `open` | `fs::symlink_metadata(&root)             .map_err` | [573](../../src/runtime_backends.rs#L573) | receiver-type-required |
| `open` | `fs::symlink_metadata` | [573](../../src/runtime_backends.rs#L573), [584](../../src/runtime_backends.rs#L584) | external-constructor-callback-or-unresolved |
| `open` | `BackendFailure::Denied` | [578](../../src/runtime_backends.rs#L578), [589](../../src/runtime_backends.rs#L589) | external-constructor-callback-or-unresolved |
| `open` | `"job root is a symlink".to_owned` | [578](../../src/runtime_backends.rs#L578) | receiver-type-required |
| `open` | `fs::canonicalize(root).map_err` | [580](../../src/runtime_backends.rs#L580) | receiver-type-required |
| `open` | `fs::canonicalize` | [580](../../src/runtime_backends.rs#L580) | external-constructor-callback-or-unresolved |
| `open` | `root.join` | [581](../../src/runtime_backends.rs#L581) | receiver-type-required |
| `open` | `fs::create_dir_all(&launch_root).map_err` | [583](../../src/runtime_backends.rs#L583) | receiver-type-required |
| `open` | `fs::symlink_metadata(&launch_root)                 .map_err(io_failure)?                 .file_type()                 .is_symlink` | [584](../../src/runtime_backends.rs#L584) | receiver-type-required |
| `open` | `fs::symlink_metadata(&launch_root)                 .map_err(io_failure)?                 .file_type` | [584](../../src/runtime_backends.rs#L584) | receiver-type-required |
| `open` | `fs::symlink_metadata(&launch_root)                 .map_err` | [584](../../src/runtime_backends.rs#L584) | receiver-type-required |
| `open` | `"job launch authority is a symlink".to_owned` | [590](../../src/runtime_backends.rs#L590) | receiver-type-required |
| `open` | `Ok` | [594](../../src/runtime_backends.rs#L594) | external-constructor-callback-or-unresolved |
| `open` | `Duration::from_millis` | [598](../../src/runtime_backends.rs#L598) | external-constructor-callback-or-unresolved |
| `start` | `BackendOutcome::enter` | [611](../../src/runtime_backends.rs#L611) | [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) |
| `start` | `cancellation.is_cancelled` | [614](../../src/runtime_backends.rs#L614) | receiver-type-required |
| `start` | `BackendOutcome::Completed` | [615](../../src/runtime_backends.rs#L615), [618](../../src/runtime_backends.rs#L618), [621](../../src/runtime_backends.rs#L621), [627](../../src/runtime_backends.rs#L627), [630](../../src/runtime_backends.rs#L630) | external-constructor-callback-or-unresolved |
| `start` | `Err` | [615](../../src/runtime_backends.rs#L615), [618](../../src/runtime_backends.rs#L618), [621](../../src/runtime_backends.rs#L621), [627](../../src/runtime_backends.rs#L627) | external-constructor-callback-or-unresolved |
| `start` | `validate_job_id` | [617](../../src/runtime_backends.rs#L617) | [tools::runtime_backends::validate_job_id](../../src/runtime_backends.rs#L2111) |
| `start` | `spec.program.is_empty` | [620](../../src/runtime_backends.rs#L620) | receiver-type-required |
| `start` | `spec.program.as_bytes().contains` | [620](../../src/runtime_backends.rs#L620) | receiver-type-required |
| `start` | `spec.program.as_bytes` | [620](../../src/runtime_backends.rs#L620) | receiver-type-required |
| `start` | `BackendFailure::Invalid` | [621](../../src/runtime_backends.rs#L621) | external-constructor-callback-or-unresolved |
| `start` | `"job program is empty or contains NUL".to_owned` | [622](../../src/runtime_backends.rs#L622) | receiver-type-required |
| `start` | `launch_policy.resolve` | [625](../../src/runtime_backends.rs#L625) | receiver-type-required |
| `start` | `self.start_inner` | [630](../../src/runtime_backends.rs#L630) | [tools::runtime_backends::JobBroker::start_inner](../../src/runtime_backends.rs#L639) |
| `start_inner` | `effective_policy             .write_roots             .iter()             .chain` | [649](../../src/runtime_backends.rs#L649) | receiver-type-required |
| `start_inner` | `effective_policy             .write_roots             .iter` | [649](../../src/runtime_backends.rs#L649) | receiver-type-required |
| `start_inner` | `effective_policy.scratch.iter` | [652](../../src/runtime_backends.rs#L652) | receiver-type-required |
| `start_inner` | `fs::canonicalize(root).map_err` | [654](../../src/runtime_backends.rs#L654) | receiver-type-required |
| `start_inner` | `fs::canonicalize` | [654](../../src/runtime_backends.rs#L654) | external-constructor-callback-or-unresolved |
| `start_inner` | `self.launch_root.starts_with` | [655](../../src/runtime_backends.rs#L655) | receiver-type-required |
| `start_inner` | `Err` | [656](../../src/runtime_backends.rs#L656), [694](../../src/runtime_backends.rs#L694), [747](../../src/runtime_backends.rs#L747), [764](../../src/runtime_backends.rs#L764), [781](../../src/runtime_backends.rs#L781), [800](../../src/runtime_backends.rs#L800), [813](../../src/runtime_backends.rs#L813), [824](../../src/runtime_backends.rs#L824), [832](../../src/runtime_backends.rs#L832) | external-constructor-callback-or-unresolved |
| `start_inner` | `BackendFailure::Denied` | [656](../../src/runtime_backends.rs#L656) | external-constructor-callback-or-unresolved |
| `start_inner` | `"job writable roots overlap private launch authority".to_owned` | [657](../../src/runtime_backends.rs#L657) | receiver-type-required |
| `start_inner` | `self.job_directory` | [661](../../src/runtime_backends.rs#L661) | [tools::runtime_backends::JobBroker::job_directory](../../src/runtime_backends.rs#L966) |
| `start_inner` | `fs::create_dir(&directory).map_err` | [662](../../src/runtime_backends.rs#L662) | receiver-type-required |
| `start_inner` | `fs::create_dir` | [662](../../src/runtime_backends.rs#L662) | external-constructor-callback-or-unresolved |
| `start_inner` | `error.kind` | [663](../../src/runtime_backends.rs#L663) | receiver-type-required |
| `start_inner` | `BackendFailure::Conflict` | [664](../../src/runtime_backends.rs#L664) | external-constructor-callback-or-unresolved |
| `start_inner` | `io_failure` | [666](../../src/runtime_backends.rs#L666) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `start_inner` | `sync_directory` | [669](../../src/runtime_backends.rs#L669) | [tools::runtime_backends::sync_directory](../../src/runtime_backends.rs#L2543) |
| `start_inner` | `unix_millis` | [670](../../src/runtime_backends.rs#L670), [692](../../src/runtime_backends.rs#L692), [806](../../src/runtime_backends.rs#L806), [822](../../src/runtime_backends.rs#L822) | [tools::runtime_backends::unix_millis](../../src/runtime_backends.rs#L2535) |
| `start_inner` | `id.to_owned` | [673](../../src/runtime_backends.rs#L673), [700](../../src/runtime_backends.rs#L700), [785](../../src/runtime_backends.rs#L785) | receiver-type-required |
| `start_inner` | `spec.clone` | [674](../../src/runtime_backends.rs#L674), [702](../../src/runtime_backends.rs#L702) | receiver-type-required |
| `start_inner` | `"stdout.tail".to_owned` | [680](../../src/runtime_backends.rs#L680) | receiver-type-required |
| `start_inner` | `"stderr.tail".to_owned` | [681](../../src/runtime_backends.rs#L681) | receiver-type-required |
| `start_inner` | `self.write_record` | [683](../../src/runtime_backends.rs#L683), [693](../../src/runtime_backends.rs#L693), [807](../../src/runtime_backends.rs#L807), [823](../../src/runtime_backends.rs#L823) | [tools::runtime_backends::JobBroker::write_record](../../src/runtime_backends.rs#L1016) |
| `start_inner` | `AtomicPublisher::replace(directory.join(&starting.stdout_tail), b"")             .map_err` | [684](../../src/runtime_backends.rs#L684) | receiver-type-required |
| `start_inner` | `AtomicPublisher::replace` | [684](../../src/runtime_backends.rs#L684), [686](../../src/runtime_backends.rs#L686), [708](../../src/runtime_backends.rs#L708), [777](../../src/runtime_backends.rs#L777), [793](../../src/runtime_backends.rs#L793), [826](../../src/runtime_backends.rs#L826) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `start_inner` | `directory.join` | [684](../../src/runtime_backends.rs#L684), [686](../../src/runtime_backends.rs#L686), [705](../../src/runtime_backends.rs#L705), [777](../../src/runtime_backends.rs#L777), [826](../../src/runtime_backends.rs#L826) | receiver-type-required |
| `start_inner` | `AtomicPublisher::replace(directory.join(&starting.stderr_tail), b"")             .map_err` | [686](../../src/runtime_backends.rs#L686) | receiver-type-required |
| `start_inner` | `cancellation.is_cancelled` | [689](../../src/runtime_backends.rs#L689), [815](../../src/runtime_backends.rs#L815) | receiver-type-required |
| `start_inner` | `self.root.to_string_lossy().into_owned` | [699](../../src/runtime_backends.rs#L699) | receiver-type-required |
| `start_inner` | `self.root.to_string_lossy` | [699](../../src/runtime_backends.rs#L699) | receiver-type-required |
| `start_inner` | `launcher.uses_pid_namespace` | [703](../../src/runtime_backends.rs#L703) | receiver-type-required |
| `start_inner` | `serde_json_canonicalizer::to_vec(&runner_request)             .map_err` | [706](../../src/runtime_backends.rs#L706) | receiver-type-required |
| `start_inner` | `serde_json_canonicalizer::to_vec` | [706](../../src/runtime_backends.rs#L706), [774](../../src/runtime_backends.rs#L774), [790](../../src/runtime_backends.rs#L790) | external-constructor-callback-or-unresolved |
| `start_inner` | `BackendFailure::Protocol` | [707](../../src/runtime_backends.rs#L707), [775](../../src/runtime_backends.rs#L775), [791](../../src/runtime_backends.rs#L791) | external-constructor-callback-or-unresolved |
| `start_inner` | `error.to_string` | [707](../../src/runtime_backends.rs#L707), [724](../../src/runtime_backends.rs#L724), [775](../../src/runtime_backends.rs#L775), [791](../../src/runtime_backends.rs#L791) | receiver-type-required |
| `start_inner` | `AtomicPublisher::replace(&request_path, &request_bytes).map_err` | [708](../../src/runtime_backends.rs#L708) | receiver-type-required |
| `start_inner` | `require_canonical_directory` | [711](../../src/runtime_backends.rs#L711) | [tools::runtime_backends::require_canonical_directory](../../src/runtime_backends.rs#L2019) |
| `start_inner` | `directory.to_string_lossy` | [711](../../src/runtime_backends.rs#L711) | receiver-type-required |
| `start_inner` | `effective_policy.read_roots.push` | [712](../../src/runtime_backends.rs#L712) | receiver-type-required |
| `start_inner` | `job_directory.clone` | [712](../../src/runtime_backends.rs#L712) | receiver-type-required |
| `start_inner` | `effective_policy.write_roots.push` | [713](../../src/runtime_backends.rs#L713) | receiver-type-required |
| `start_inner` | `effective_policy             .read_roots             .sort_by` | [714](../../src/runtime_backends.rs#L714) | receiver-type-required |
| `start_inner` | `left.as_bytes().cmp` | [716](../../src/runtime_backends.rs#L716), [720](../../src/runtime_backends.rs#L720) | receiver-type-required |
| `start_inner` | `left.as_bytes` | [716](../../src/runtime_backends.rs#L716), [720](../../src/runtime_backends.rs#L720) | receiver-type-required |
| `start_inner` | `right.as_bytes` | [716](../../src/runtime_backends.rs#L716), [720](../../src/runtime_backends.rs#L720) | receiver-type-required |
| `start_inner` | `effective_policy.read_roots.dedup` | [717](../../src/runtime_backends.rs#L717) | receiver-type-required |
| `start_inner` | `effective_policy             .write_roots             .sort_by` | [718](../../src/runtime_backends.rs#L718) | receiver-type-required |
| `start_inner` | `effective_policy.write_roots.dedup` | [721](../../src/runtime_backends.rs#L721) | receiver-type-required |
| `start_inner` | `effective_policy             .validate()             .map_err` | [722](../../src/runtime_backends.rs#L722) | receiver-type-required |
| `start_inner` | `effective_policy             .validate` | [722](../../src/runtime_backends.rs#L722) | receiver-type-required |
| `start_inner` | `BackendFailure::Invalid` | [724](../../src/runtime_backends.rs#L724) | external-constructor-callback-or-unresolved |
| `start_inner` | `launcher.runner_command` | [726](../../src/runtime_backends.rs#L726) | receiver-type-required |
| `start_inner` | `command             .arg("--job-runner")             .arg(&request_path)             .stdin(Stdio::null())             .stdout(Stdio::null())             .stderr` | [727](../../src/runtime_backends.rs#L727) | receiver-type-required |
| `start_inner` | `command             .arg("--job-runner")             .arg(&request_path)             .stdin(Stdio::null())             .stdout` | [727](../../src/runtime_backends.rs#L727) | receiver-type-required |
| `start_inner` | `command             .arg("--job-runner")             .arg(&request_path)             .stdin` | [727](../../src/runtime_backends.rs#L727) | receiver-type-required |
| `start_inner` | `command             .arg("--job-runner")             .arg` | [727](../../src/runtime_backends.rs#L727) | receiver-type-required |
| `start_inner` | `command             .arg` | [727](../../src/runtime_backends.rs#L727) | receiver-type-required |
| `start_inner` | `Stdio::null` | [730](../../src/runtime_backends.rs#L730), [731](../../src/runtime_backends.rs#L731), [732](../../src/runtime_backends.rs#L732) | external-constructor-callback-or-unresolved |
| `start_inner` | `namespace_identity_channel` | [736](../../src/runtime_backends.rs#L736) | [tools::runtime_backends::namespace_identity_channel](../../src/runtime_backends.rs#L2368) |
| `start_inner` | `command.stdin` | [737](../../src/runtime_backends.rs#L737) | receiver-type-required |
| `start_inner` | `Stdio::from` | [737](../../src/runtime_backends.rs#L737) | external-constructor-callback-or-unresolved |
| `start_inner` | `OwnedFd::from` | [737](../../src/runtime_backends.rs#L737) | external-constructor-callback-or-unresolved |
| `start_inner` | `Some` | [738](../../src/runtime_backends.rs#L738), [761](../../src/runtime_backends.rs#L761), [804](../../src/runtime_backends.rs#L804), [805](../../src/runtime_backends.rs#L805) | external-constructor-callback-or-unresolved |
| `start_inner` | `command.pre_exec` | [745](../../src/runtime_backends.rs#L745) | receiver-type-required |
| `start_inner` | `libc::setsid` | [746](../../src/runtime_backends.rs#L746) | external-constructor-callback-or-unresolved |
| `start_inner` | `std::io::Error::last_os_error` | [747](../../src/runtime_backends.rs#L747) | external-constructor-callback-or-unresolved |
| `start_inner` | `Ok` | [749](../../src/runtime_backends.rs#L749), [840](../../src/runtime_backends.rs#L840) | external-constructor-callback-or-unresolved |
| `start_inner` | `command.spawn().map_err` | [752](../../src/runtime_backends.rs#L752) | receiver-type-required |
| `start_inner` | `command.spawn` | [752](../../src/runtime_backends.rs#L752) | receiver-type-required |
| `start_inner` | `child.id` | [753](../../src/runtime_backends.rs#L753) | receiver-type-required |
| `start_inner` | `process_start_token(pid).ok_or_else` | [754](../../src/runtime_backends.rs#L754) | receiver-type-required |
| `start_inner` | `process_start_token` | [754](../../src/runtime_backends.rs#L754) | ambiguous-cfg-or-overload |
| `start_inner` | `terminate_group` | [755](../../src/runtime_backends.rs#L755), [763](../../src/runtime_backends.rs#L763), [780](../../src/runtime_backends.rs#L780), [799](../../src/runtime_backends.rs#L799), [812](../../src/runtime_backends.rs#L812), [820](../../src/runtime_backends.rs#L820), [831](../../src/runtime_backends.rs#L831) | [tools::runtime_backends::terminate_group](../../src/runtime_backends.rs#L2160) |
| `start_inner` | `Duration::from_millis` | [755](../../src/runtime_backends.rs#L755), [763](../../src/runtime_backends.rs#L763), [780](../../src/runtime_backends.rs#L780), [799](../../src/runtime_backends.rs#L799), [812](../../src/runtime_backends.rs#L812), [820](../../src/runtime_backends.rs#L820), [831](../../src/runtime_backends.rs#L831) | external-constructor-callback-or-unresolved |
| `start_inner` | `BackendFailure::Unknown` | [756](../../src/runtime_backends.rs#L756) | external-constructor-callback-or-unresolved |
| `start_inner` | `"could not obtain process start identity".to_owned` | [756](../../src/runtime_backends.rs#L756) | receiver-type-required |
| `start_inner` | `authenticate_namespace_runner` | [760](../../src/runtime_backends.rs#L760) | [tools::runtime_backends::authenticate_namespace_runner](../../src/runtime_backends.rs#L2460) |
| `start_inner` | `serde_json_canonicalizer::to_vec(identity)                 .map_err` | [774](../../src/runtime_backends.rs#L774) | receiver-type-required |
| `start_inner` | `identity.stop_namespace` | [779](../../src/runtime_backends.rs#L779), [797](../../src/runtime_backends.rs#L797), [810](../../src/runtime_backends.rs#L810), [818](../../src/runtime_backends.rs#L818), [829](../../src/runtime_backends.rs#L829) | receiver-type-required |
| `start_inner` | `store_failure` | [781](../../src/runtime_backends.rs#L781), [800](../../src/runtime_backends.rs#L800), [832](../../src/runtime_backends.rs#L832) | [tools::runtime_backends::store_failure](../../src/runtime_backends.rs#L2553) |
| `start_inner` | `token.clone` | [787](../../src/runtime_backends.rs#L787) | receiver-type-required |
| `start_inner` | `namespace_identity.clone` | [788](../../src/runtime_backends.rs#L788) | receiver-type-required |
| `start_inner` | `serde_json_canonicalizer::to_vec(&host_receipt)             .map_err` | [790](../../src/runtime_backends.rs#L790) | receiver-type-required |
| `start_inner` | `self.launch_root.join` | [793](../../src/runtime_backends.rs#L793) | receiver-type-required |
| `start_inner` | `thread::spawn` | [837](../../src/runtime_backends.rs#L837) | external-constructor-callback-or-unresolved |
| `start_inner` | `child.wait` | [838](../../src/runtime_backends.rs#L838) | receiver-type-required |
| `status` | `BackendOutcome::Completed` | [844](../../src/runtime_backends.rs#L844) | external-constructor-callback-or-unresolved |
| `status` | `self.status_inner` | [844](../../src/runtime_backends.rs#L844) | [tools::runtime_backends::JobBroker::status_inner](../../src/runtime_backends.rs#L847) |
| `status_inner` | `validate_job_id` | [848](../../src/runtime_backends.rs#L848) | [tools::runtime_backends::validate_job_id](../../src/runtime_backends.rs#L2111) |
| `status_inner` | `self.read_record` | [849](../../src/runtime_backends.rs#L849) | [tools::runtime_backends::JobBroker::read_record](../../src/runtime_backends.rs#L996) |
| `status_inner` | `unix_millis` | [855](../../src/runtime_backends.rs#L855), [868](../../src/runtime_backends.rs#L868) | [tools::runtime_backends::unix_millis](../../src/runtime_backends.rs#L2535) |
| `status_inner` | `self.write_record` | [856](../../src/runtime_backends.rs#L856), [869](../../src/runtime_backends.rs#L869) | [tools::runtime_backends::JobBroker::write_record](../../src/runtime_backends.rs#L1016) |
| `status_inner` | `self.host_receipt` | [858](../../src/runtime_backends.rs#L858) | [tools::runtime_backends::JobBroker::host_receipt](../../src/runtime_backends.rs#L970) |
| `status_inner` | `record                     .pid                     .zip(record.process_start.as_deref())                     .is_some_and` | [860](../../src/runtime_backends.rs#L860) | receiver-type-required |
| `status_inner` | `record                     .pid                     .zip` | [860](../../src/runtime_backends.rs#L860) | receiver-type-required |
| `status_inner` | `record.process_start.as_deref` | [862](../../src/runtime_backends.rs#L862) | receiver-type-required |
| `status_inner` | `process_start_token(pid).as_deref` | [864](../../src/runtime_backends.rs#L864) | receiver-type-required |
| `status_inner` | `process_start_token` | [864](../../src/runtime_backends.rs#L864) | ambiguous-cfg-or-overload |
| `status_inner` | `Some` | [864](../../src/runtime_backends.rs#L864) | external-constructor-callback-or-unresolved |
| `status_inner` | `Ok` | [872](../../src/runtime_backends.rs#L872) | external-constructor-callback-or-unresolved |
| `list` | `BackendOutcome::Completed` | [876](../../src/runtime_backends.rs#L876) | external-constructor-callback-or-unresolved |
| `list` | `self.list_inner` | [876](../../src/runtime_backends.rs#L876) | [tools::runtime_backends::JobBroker::list_inner](../../src/runtime_backends.rs#L879) |
| `list_inner` | `Vec::new` | [880](../../src/runtime_backends.rs#L880) | external-constructor-callback-or-unresolved |
| `list_inner` | `fs::read_dir(&self.root).map_err` | [881](../../src/runtime_backends.rs#L881) | receiver-type-required |
| `list_inner` | `fs::read_dir` | [881](../../src/runtime_backends.rs#L881) | external-constructor-callback-or-unresolved |
| `list_inner` | `entry.map_err` | [882](../../src/runtime_backends.rs#L882) | receiver-type-required |
| `list_inner` | `entry.file_type().map_err(io_failure)?.is_dir` | [883](../../src/runtime_backends.rs#L883) | receiver-type-required |
| `list_inner` | `entry.file_type().map_err` | [883](../../src/runtime_backends.rs#L883) | receiver-type-required |
| `list_inner` | `entry.file_type` | [883](../../src/runtime_backends.rs#L883) | receiver-type-required |
| `list_inner` | `entry.file_name().to_str` | [884](../../src/runtime_backends.rs#L884) | receiver-type-required |
| `list_inner` | `entry.file_name` | [884](../../src/runtime_backends.rs#L884) | receiver-type-required |
| `list_inner` | `validate_job_id(id).is_ok` | [885](../../src/runtime_backends.rs#L885) | receiver-type-required |
| `list_inner` | `validate_job_id` | [885](../../src/runtime_backends.rs#L885) | [tools::runtime_backends::validate_job_id](../../src/runtime_backends.rs#L2111) |
| `list_inner` | `ids.push` | [887](../../src/runtime_backends.rs#L887) | receiver-type-required |
| `list_inner` | `id.to_owned` | [887](../../src/runtime_backends.rs#L887) | receiver-type-required |
| `list_inner` | `ids.sort_by` | [890](../../src/runtime_backends.rs#L890) | receiver-type-required |
| `list_inner` | `left.as_bytes().cmp` | [890](../../src/runtime_backends.rs#L890) | receiver-type-required |
| `list_inner` | `left.as_bytes` | [890](../../src/runtime_backends.rs#L890) | receiver-type-required |
| `list_inner` | `right.as_bytes` | [890](../../src/runtime_backends.rs#L890) | receiver-type-required |
| `list_inner` | `ids.into_iter().map(&#124;id&#124; self.status_inner(&id)).collect` | [891](../../src/runtime_backends.rs#L891) | receiver-type-required |
| `list_inner` | `ids.into_iter().map` | [891](../../src/runtime_backends.rs#L891) | receiver-type-required |
| `list_inner` | `ids.into_iter` | [891](../../src/runtime_backends.rs#L891) | receiver-type-required |
| `list_inner` | `self.status_inner` | [891](../../src/runtime_backends.rs#L891) | [tools::runtime_backends::JobBroker::status_inner](../../src/runtime_backends.rs#L847) |
| `stop` | `BackendOutcome::enter` | [900](../../src/runtime_backends.rs#L900) | [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) |
| `stop` | `cancellation.is_cancelled` | [903](../../src/runtime_backends.rs#L903) | receiver-type-required |
| `stop` | `BackendOutcome::Completed` | [904](../../src/runtime_backends.rs#L904), [906](../../src/runtime_backends.rs#L906) | external-constructor-callback-or-unresolved |
| `stop` | `Err` | [904](../../src/runtime_backends.rs#L904) | external-constructor-callback-or-unresolved |
| `stop` | `self.stop_inner` | [906](../../src/runtime_backends.rs#L906) | [tools::runtime_backends::JobBroker::stop_inner](../../src/runtime_backends.rs#L909) |
| `stop_inner` | `self.status_inner` | [914](../../src/runtime_backends.rs#L914) | [tools::runtime_backends::JobBroker::status_inner](../../src/runtime_backends.rs#L847) |
| `stop_inner` | `Ok` | [916](../../src/runtime_backends.rs#L916), [963](../../src/runtime_backends.rs#L963) | external-constructor-callback-or-unresolved |
| `stop_inner` | `Err` | [919](../../src/runtime_backends.rs#L919), [938](../../src/runtime_backends.rs#L938) | external-constructor-callback-or-unresolved |
| `stop_inner` | `BackendFailure::Unknown` | [919](../../src/runtime_backends.rs#L919) | external-constructor-callback-or-unresolved |
| `stop_inner` | `record.pid.expect` | [923](../../src/runtime_backends.rs#L923) | receiver-type-required |
| `stop_inner` | `self.host_receipt` | [925](../../src/runtime_backends.rs#L925) | [tools::runtime_backends::JobBroker::host_receipt](../../src/runtime_backends.rs#L970) |
| `stop_inner` | `namespace_identity             .as_ref()             .map(NamespaceRunnerIdentity::stop_handle)             .transpose()?             .flatten` | [927](../../src/runtime_backends.rs#L927) | receiver-type-required |
| `stop_inner` | `namespace_identity             .as_ref()             .map(NamespaceRunnerIdentity::stop_handle)             .transpose` | [927](../../src/runtime_backends.rs#L927) | receiver-type-required |
| `stop_inner` | `namespace_identity             .as_ref()             .map` | [927](../../src/runtime_backends.rs#L927) | receiver-type-required |
| `stop_inner` | `namespace_identity             .as_ref` | [927](../../src/runtime_backends.rs#L927) | receiver-type-required |
| `stop_inner` | `process_start_token(pid).as_deref` | [932](../../src/runtime_backends.rs#L932), [940](../../src/runtime_backends.rs#L940), [951](../../src/runtime_backends.rs#L951) | receiver-type-required |
| `stop_inner` | `process_start_token` | [932](../../src/runtime_backends.rs#L932), [940](../../src/runtime_backends.rs#L940), [951](../../src/runtime_backends.rs#L951) | ambiguous-cfg-or-overload |
| `stop_inner` | `record.process_start.as_deref` | [932](../../src/runtime_backends.rs#L932), [940](../../src/runtime_backends.rs#L940), [951](../../src/runtime_backends.rs#L951) | receiver-type-required |
| `stop_inner` | `signal_group` | [933](../../src/runtime_backends.rs#L933), [952](../../src/runtime_backends.rs#L952) | [tools::runtime_backends::signal_group](../../src/runtime_backends.rs#L2143) |
| `stop_inner` | `Instant::now` | [935](../../src/runtime_backends.rs#L935), [936](../../src/runtime_backends.rs#L936) | external-constructor-callback-or-unresolved |
| `stop_inner` | `cancellation.is_cancelled` | [937](../../src/runtime_backends.rs#L937) | receiver-type-required |
| `stop_inner` | `namespace_identity                     .as_ref()                     .is_some_and` | [943](../../src/runtime_backends.rs#L943) | receiver-type-required |
| `stop_inner` | `namespace_identity                     .as_ref` | [943](../../src/runtime_backends.rs#L943) | receiver-type-required |
| `stop_inner` | `identity.namespace_is_live` | [945](../../src/runtime_backends.rs#L945) | receiver-type-required |
| `stop_inner` | `thread::sleep` | [949](../../src/runtime_backends.rs#L949) | external-constructor-callback-or-unresolved |
| `stop_inner` | `Duration::from_millis` | [949](../../src/runtime_backends.rs#L949) | external-constructor-callback-or-unresolved |
| `stop_inner` | `handle.kill` | [958](../../src/runtime_backends.rs#L958) | receiver-type-required |
| `stop_inner` | `unix_millis` | [961](../../src/runtime_backends.rs#L961) | [tools::runtime_backends::unix_millis](../../src/runtime_backends.rs#L2535) |
| `stop_inner` | `self.write_record` | [962](../../src/runtime_backends.rs#L962) | [tools::runtime_backends::JobBroker::write_record](../../src/runtime_backends.rs#L1016) |
| `job_directory` | `self.root.join` | [967](../../src/runtime_backends.rs#L967) | receiver-type-required |
| `host_receipt` | `serde_json::from_slice(             &fs::read(self.launch_root.join(format!("{}.json", record.id))).map_err(io_failure)?,         )         .map_err` | [971](../../src/runtime_backends.rs#L971) | receiver-type-required |
| `host_receipt` | `serde_json::from_slice` | [971](../../src/runtime_backends.rs#L971) | external-constructor-callback-or-unresolved |
| `host_receipt` | `fs::read(self.launch_root.join(format!("{}.json", record.id))).map_err` | [972](../../src/runtime_backends.rs#L972) | receiver-type-required |
| `host_receipt` | `fs::read` | [972](../../src/runtime_backends.rs#L972) | external-constructor-callback-or-unresolved |
| `host_receipt` | `self.launch_root.join` | [972](../../src/runtime_backends.rs#L972) | receiver-type-required |
| `host_receipt` | `BackendFailure::Protocol` | [974](../../src/runtime_backends.rs#L974) | external-constructor-callback-or-unresolved |
| `host_receipt` | `error.to_string` | [974](../../src/runtime_backends.rs#L974) | receiver-type-required |
| `host_receipt` | `Some` | [976](../../src/runtime_backends.rs#L976), [977](../../src/runtime_backends.rs#L977) | external-constructor-callback-or-unresolved |
| `host_receipt` | `record.process_start.as_deref` | [977](../../src/runtime_backends.rs#L977) | receiver-type-required |
| `host_receipt` | `receipt.process_start.as_str` | [977](../../src/runtime_backends.rs#L977) | receiver-type-required |
| `host_receipt` | `Err` | [979](../../src/runtime_backends.rs#L979), [989](../../src/runtime_backends.rs#L989) | external-constructor-callback-or-unresolved |
| `host_receipt` | `BackendFailure::Conflict` | [979](../../src/runtime_backends.rs#L979), [989](../../src/runtime_backends.rs#L989) | external-constructor-callback-or-unresolved |
| `host_receipt` | `"job record does not match private launch authority".to_owned` | [980](../../src/runtime_backends.rs#L980) | receiver-type-required |
| `host_receipt` | `receipt             .namespace_identity             .as_ref()             .is_some_and` | [984](../../src/runtime_backends.rs#L984) | receiver-type-required |
| `host_receipt` | `receipt             .namespace_identity             .as_ref` | [984](../../src/runtime_backends.rs#L984) | receiver-type-required |
| `host_receipt` | `identity.matches_record` | [987](../../src/runtime_backends.rs#L987) | receiver-type-required |
| `host_receipt` | `"namespace receipt does not match private launch authority".to_owned` | [990](../../src/runtime_backends.rs#L990) | receiver-type-required |
| `host_receipt` | `Ok` | [993](../../src/runtime_backends.rs#L993) | external-constructor-callback-or-unresolved |
| `read_record` | `self.job_directory` | [997](../../src/runtime_backends.rs#L997) | [tools::runtime_backends::JobBroker::job_directory](../../src/runtime_backends.rs#L966) |
| `read_record` | `NamedLock::shared(directory.join("job.lock")).map_err` | [998](../../src/runtime_backends.rs#L998) | receiver-type-required |
| `read_record` | `NamedLock::shared` | [998](../../src/runtime_backends.rs#L998) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `read_record` | `directory.join` | [998](../../src/runtime_backends.rs#L998), [999](../../src/runtime_backends.rs#L999) | receiver-type-required |
| `read_record` | `fs::read(directory.join("job.json")).map_err` | [999](../../src/runtime_backends.rs#L999) | receiver-type-required |
| `read_record` | `fs::read` | [999](../../src/runtime_backends.rs#L999) | external-constructor-callback-or-unresolved |
| `read_record` | `error.kind` | [1000](../../src/runtime_backends.rs#L1000) | receiver-type-required |
| `read_record` | `BackendFailure::NotFound` | [1001](../../src/runtime_backends.rs#L1001) | external-constructor-callback-or-unresolved |
| `read_record` | `io_failure` | [1003](../../src/runtime_backends.rs#L1003) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `read_record` | `serde_json::from_slice(&bytes)             .map_err` | [1006](../../src/runtime_backends.rs#L1006) | receiver-type-required |
| `read_record` | `serde_json::from_slice` | [1006](../../src/runtime_backends.rs#L1006) | external-constructor-callback-or-unresolved |
| `read_record` | `BackendFailure::Protocol` | [1007](../../src/runtime_backends.rs#L1007) | external-constructor-callback-or-unresolved |
| `read_record` | `error.to_string` | [1007](../../src/runtime_backends.rs#L1007) | receiver-type-required |
| `read_record` | `Err` | [1009](../../src/runtime_backends.rs#L1009) | external-constructor-callback-or-unresolved |
| `read_record` | `BackendFailure::Conflict` | [1009](../../src/runtime_backends.rs#L1009) | external-constructor-callback-or-unresolved |
| `read_record` | `"job record id does not match the requested job".to_owned` | [1010](../../src/runtime_backends.rs#L1010) | receiver-type-required |
| `read_record` | `Ok` | [1013](../../src/runtime_backends.rs#L1013) | external-constructor-callback-or-unresolved |
| `write_record` | `self.job_directory` | [1017](../../src/runtime_backends.rs#L1017) | [tools::runtime_backends::JobBroker::job_directory](../../src/runtime_backends.rs#L966) |
| `write_record` | `NamedLock::exclusive(directory.join("job.lock")).map_err` | [1018](../../src/runtime_backends.rs#L1018) | receiver-type-required |
| `write_record` | `NamedLock::exclusive` | [1018](../../src/runtime_backends.rs#L1018) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `write_record` | `directory.join` | [1018](../../src/runtime_backends.rs#L1018), [1021](../../src/runtime_backends.rs#L1021) | receiver-type-required |
| `write_record` | `serde_json_canonicalizer::to_vec(record)             .map_err` | [1019](../../src/runtime_backends.rs#L1019) | receiver-type-required |
| `write_record` | `serde_json_canonicalizer::to_vec` | [1019](../../src/runtime_backends.rs#L1019) | external-constructor-callback-or-unresolved |
| `write_record` | `BackendFailure::Protocol` | [1020](../../src/runtime_backends.rs#L1020) | external-constructor-callback-or-unresolved |
| `write_record` | `error.to_string` | [1020](../../src/runtime_backends.rs#L1020) | receiver-type-required |
| `write_record` | `AtomicPublisher::replace(directory.join("job.json"), &bytes).map_err` | [1021](../../src/runtime_backends.rs#L1021) | receiver-type-required |
| `write_record` | `AtomicPublisher::replace` | [1021](../../src/runtime_backends.rs#L1021) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `run_job_runner` | `request_path.as_ref` | [1029](../../src/runtime_backends.rs#L1029) | receiver-type-required |
| `run_job_runner` | `request_path.is_absolute` | [1030](../../src/runtime_backends.rs#L1030) | receiver-type-required |
| `run_job_runner` | `Err` | [1031](../../src/runtime_backends.rs#L1031), [1037](../../src/runtime_backends.rs#L1037), [1044](../../src/runtime_backends.rs#L1044), [1052](../../src/runtime_backends.rs#L1052), [1064](../../src/runtime_backends.rs#L1064), [1083](../../src/runtime_backends.rs#L1083) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `BackendFailure::Invalid` | [1031](../../src/runtime_backends.rs#L1031) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `"job runner request path must be absolute".to_owned` | [1032](../../src/runtime_backends.rs#L1032) | receiver-type-required |
| `run_job_runner` | `fs::read(request_path).map_err` | [1035](../../src/runtime_backends.rs#L1035) | receiver-type-required |
| `run_job_runner` | `fs::read` | [1035](../../src/runtime_backends.rs#L1035), [1075](../../src/runtime_backends.rs#L1075) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `bytes.len` | [1036](../../src/runtime_backends.rs#L1036) | receiver-type-required |
| `run_job_runner` | `BackendFailure::Limit` | [1037](../../src/runtime_backends.rs#L1037) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `"job runner request exceeds 1 MiB".to_owned` | [1038](../../src/runtime_backends.rs#L1038) | receiver-type-required |
| `run_job_runner` | `serde_json::from_slice(&bytes)         .map_err` | [1041](../../src/runtime_backends.rs#L1041) | receiver-type-required |
| `run_job_runner` | `serde_json::from_slice` | [1041](../../src/runtime_backends.rs#L1041), [1074](../../src/runtime_backends.rs#L1074) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `BackendFailure::Protocol` | [1042](../../src/runtime_backends.rs#L1042), [1044](../../src/runtime_backends.rs#L1044), [1077](../../src/runtime_backends.rs#L1077), [1089](../../src/runtime_backends.rs#L1089), [1104](../../src/runtime_backends.rs#L1104), [1108](../../src/runtime_backends.rs#L1108) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `error.to_string` | [1042](../../src/runtime_backends.rs#L1042), [1077](../../src/runtime_backends.rs#L1077) | receiver-type-required |
| `run_job_runner` | `"unsupported job runner request format".to_owned` | [1045](../../src/runtime_backends.rs#L1045) | receiver-type-required |
| `run_job_runner` | `validate_job_id` | [1048](../../src/runtime_backends.rs#L1048) | [tools::runtime_backends::validate_job_id](../../src/runtime_backends.rs#L2111) |
| `run_job_runner` | `JobBroker::open` | [1049](../../src/runtime_backends.rs#L1049) | [tools::runtime_backends::JobBroker::open](../../src/runtime_backends.rs#L562) |
| `run_job_runner` | `PathBuf::from` | [1049](../../src/runtime_backends.rs#L1049) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `broker.job_directory` | [1050](../../src/runtime_backends.rs#L1050) | receiver-type-required |
| `run_job_runner` | `directory.join` | [1051](../../src/runtime_backends.rs#L1051), [1056](../../src/runtime_backends.rs#L1056), [1075](../../src/runtime_backends.rs#L1075), [1109](../../src/runtime_backends.rs#L1109), [1110](../../src/runtime_backends.rs#L1110) | receiver-type-required |
| `run_job_runner` | `BackendFailure::Denied` | [1052](../../src/runtime_backends.rs#L1052) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `"job runner request is outside its job directory".to_owned` | [1053](../../src/runtime_backends.rs#L1053) | receiver-type-required |
| `run_job_runner` | `announce_namespace_runner` | [1059](../../src/runtime_backends.rs#L1059) | [tools::runtime_backends::announce_namespace_runner](../../src/runtime_backends.rs#L2398) |
| `run_job_runner` | `Instant::now` | [1061](../../src/runtime_backends.rs#L1061), [1063](../../src/runtime_backends.rs#L1063) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `Duration::from_secs` | [1061](../../src/runtime_backends.rs#L1061) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `permit.is_file` | [1062](../../src/runtime_backends.rs#L1062) | receiver-type-required |
| `run_job_runner` | `thread::sleep` | [1066](../../src/runtime_backends.rs#L1066) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `Duration::from_millis` | [1066](../../src/runtime_backends.rs#L1066) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `broker.read_record` | [1068](../../src/runtime_backends.rs#L1068), [1124](../../src/runtime_backends.rs#L1124) | receiver-type-required |
| `run_job_runner` | `std::process::id` | [1069](../../src/runtime_backends.rs#L1069) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `Some` | [1070](../../src/runtime_backends.rs#L1070) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `record.process_start.as_deref` | [1071](../../src/runtime_backends.rs#L1071) | receiver-type-required |
| `run_job_runner` | `process_start_token(runner_pid).as_deref` | [1071](../../src/runtime_backends.rs#L1071) | receiver-type-required |
| `run_job_runner` | `process_start_token` | [1071](../../src/runtime_backends.rs#L1071) | ambiguous-cfg-or-overload |
| `run_job_runner` | `serde_json::from_slice(             &fs::read(directory.join("runner.identity.json")).map_err(io_failure)?,         )         .map_err` | [1074](../../src/runtime_backends.rs#L1074) | receiver-type-required |
| `run_job_runner` | `fs::read(directory.join("runner.identity.json")).map_err` | [1075](../../src/runtime_backends.rs#L1075) | receiver-type-required |
| `run_job_runner` | `identity.matches_record` | [1078](../../src/runtime_backends.rs#L1078) | receiver-type-required |
| `run_job_runner` | `identity.matches_current_runner` | [1078](../../src/runtime_backends.rs#L1078) | receiver-type-required |
| `run_job_runner` | `BackendFailure::Conflict` | [1083](../../src/runtime_backends.rs#L1083) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `"job runner does not match the durable running record".to_owned` | [1084](../../src/runtime_backends.rs#L1084) | receiver-type-required |
| `run_job_runner` | `request.spec.working_directory.as_deref().ok_or_else` | [1088](../../src/runtime_backends.rs#L1088) | receiver-type-required |
| `run_job_runner` | `request.spec.working_directory.as_deref` | [1088](../../src/runtime_backends.rs#L1088) | receiver-type-required |
| `run_job_runner` | `"job runner request has no resolved working directory".to_owned` | [1089](../../src/runtime_backends.rs#L1089) | receiver-type-required |
| `run_job_runner` | `Command::new` | [1091](../../src/runtime_backends.rs#L1091) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir(working_directory)         .env_clear()         .envs(&request.spec.environment)         .stdin(Stdio::null())         .stdout(Stdio::piped())         .stderr` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir(working_directory)         .env_clear()         .envs(&request.spec.environment)         .stdin(Stdio::null())         .stdout` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir(working_directory)         .env_clear()         .envs(&request.spec.environment)         .stdin` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir(working_directory)         .env_clear()         .envs` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir(working_directory)         .env_clear` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args(&request.spec.args)         .current_dir` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `command         .args` | [1092](../../src/runtime_backends.rs#L1092) | receiver-type-required |
| `run_job_runner` | `Stdio::null` | [1097](../../src/runtime_backends.rs#L1097) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `Stdio::piped` | [1098](../../src/runtime_backends.rs#L1098), [1099](../../src/runtime_backends.rs#L1099) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `command.spawn().map_err` | [1100](../../src/runtime_backends.rs#L1100) | receiver-type-required |
| `run_job_runner` | `command.spawn` | [1100](../../src/runtime_backends.rs#L1100) | receiver-type-required |
| `run_job_runner` | `child         .stdout         .take()         .ok_or_else` | [1101](../../src/runtime_backends.rs#L1101) | receiver-type-required |
| `run_job_runner` | `child         .stdout         .take` | [1101](../../src/runtime_backends.rs#L1101) | receiver-type-required |
| `run_job_runner` | `"job stdout pipe missing".to_owned` | [1104](../../src/runtime_backends.rs#L1104) | receiver-type-required |
| `run_job_runner` | `child         .stderr         .take()         .ok_or_else` | [1105](../../src/runtime_backends.rs#L1105) | receiver-type-required |
| `run_job_runner` | `child         .stderr         .take` | [1105](../../src/runtime_backends.rs#L1105) | receiver-type-required |
| `run_job_runner` | `"job stderr pipe missing".to_owned` | [1108](../../src/runtime_backends.rs#L1108) | receiver-type-required |
| `run_job_runner` | `thread::spawn` | [1112](../../src/runtime_backends.rs#L1112), [1113](../../src/runtime_backends.rs#L1113) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `drain_tail` | [1112](../../src/runtime_backends.rs#L1112), [1113](../../src/runtime_backends.rs#L1113) | [tools::runtime_backends::drain_tail](../../src/runtime_backends.rs#L2125) |
| `run_job_runner` | `child.wait().map_err` | [1114](../../src/runtime_backends.rs#L1114) | receiver-type-required |
| `run_job_runner` | `child.wait` | [1114](../../src/runtime_backends.rs#L1114) | receiver-type-required |
| `run_job_runner` | `stdout_thread         .join()         .map_err` | [1115](../../src/runtime_backends.rs#L1115) | receiver-type-required |
| `run_job_runner` | `stdout_thread         .join` | [1115](../../src/runtime_backends.rs#L1115) | receiver-type-required |
| `run_job_runner` | `BackendFailure::Unknown` | [1117](../../src/runtime_backends.rs#L1117), [1120](../../src/runtime_backends.rs#L1120) | external-constructor-callback-or-unresolved |
| `run_job_runner` | `"job stdout reader panicked".to_owned` | [1117](../../src/runtime_backends.rs#L1117) | receiver-type-required |
| `run_job_runner` | `stderr_thread         .join()         .map_err` | [1118](../../src/runtime_backends.rs#L1118) | receiver-type-required |
| `run_job_runner` | `stderr_thread         .join` | [1118](../../src/runtime_backends.rs#L1118) | receiver-type-required |
| `run_job_runner` | `"job stderr reader panicked".to_owned` | [1120](../../src/runtime_backends.rs#L1120) | receiver-type-required |
| `run_job_runner` | `status.code().unwrap_or` | [1128](../../src/runtime_backends.rs#L1128) | receiver-type-required |
| `run_job_runner` | `status.code` | [1128](../../src/runtime_backends.rs#L1128) | receiver-type-required |
| `run_job_runner` | `unix_millis` | [1132](../../src/runtime_backends.rs#L1132) | [tools::runtime_backends::unix_millis](../../src/runtime_backends.rs#L2535) |
| `run_job_runner` | `broker.write_record` | [1133](../../src/runtime_backends.rs#L1133) | receiver-type-required |
| `run_job_runner` | `Ok` | [1135](../../src/runtime_backends.rs#L1135) | external-constructor-callback-or-unresolved |
| `default` | `Duration::from_secs` | [1148](../../src/runtime_backends.rs#L1148) | external-constructor-callback-or-unresolved |
| `validate` | `self.timeout.is_zero` | [1157](../../src/runtime_backends.rs#L1157) | receiver-type-required |
| `validate` | `Duration::from_millis` | [1157](../../src/runtime_backends.rs#L1157) | external-constructor-callback-or-unresolved |
| `validate` | `Err` | [1158](../../src/runtime_backends.rs#L1158), [1163](../../src/runtime_backends.rs#L1163), [1168](../../src/runtime_backends.rs#L1168) | external-constructor-callback-or-unresolved |
| `validate` | `BackendFailure::Limit` | [1158](../../src/runtime_backends.rs#L1158), [1163](../../src/runtime_backends.rs#L1163), [1168](../../src/runtime_backends.rs#L1168) | external-constructor-callback-or-unresolved |
| `validate` | `"HTTP timeout is outside hard bounds".to_owned` | [1159](../../src/runtime_backends.rs#L1159) | receiver-type-required |
| `validate` | `"HTTP byte cap is outside hard bounds".to_owned` | [1164](../../src/runtime_backends.rs#L1164) | receiver-type-required |
| `validate` | `"HTTP redirect cap is outside hard bounds".to_owned` | [1169](../../src/runtime_backends.rs#L1169) | receiver-type-required |
| `validate` | `Ok` | [1172](../../src/runtime_backends.rs#L1172) | external-constructor-callback-or-unresolved |
| `new` | `limits.validate` | [1212](../../src/runtime_backends.rs#L1212) | receiver-type-required |
| `new` | `Ok` | [1213](../../src/runtime_backends.rs#L1213) | external-constructor-callback-or-unresolved |
| `fetch` | `BackendOutcome::enter` | [1222](../../src/runtime_backends.rs#L1222) | [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) |
| `fetch` | `cancellation.is_cancelled` | [1225](../../src/runtime_backends.rs#L1225) | receiver-type-required |
| `fetch` | `BackendOutcome::Completed` | [1226](../../src/runtime_backends.rs#L1226), [1246](../../src/runtime_backends.rs#L1246) | external-constructor-callback-or-unresolved |
| `fetch` | `Err` | [1226](../../src/runtime_backends.rs#L1226) | external-constructor-callback-or-unresolved |
| `fetch` | `url.to_owned` | [1228](../../src/runtime_backends.rs#L1228) | receiver-type-required |
| `fetch` | `self.limits.clone` | [1229](../../src/runtime_backends.rs#L1229) | receiver-type-required |
| `fetch` | `cancellation.clone` | [1230](../../src/runtime_backends.rs#L1230) | receiver-type-required |
| `fetch` | `thread::Builder::new()             .name("tekes-http-backend".to_owned())             .spawn(move &#124;&#124; {                 let runtime = tokio::runtime::Builder::new_current_thread()                     .enable_all()                     .build()                     .map_err(io_failure)?;                 runtime.block_on(fetch_async(url, limits, cancellation))             })             .map_err(io_failure)             .and_then` | [1231](../../src/runtime_backends.rs#L1231) | receiver-type-required |
| `fetch` | `thread::Builder::new()             .name("tekes-http-backend".to_owned())             .spawn(move &#124;&#124; {                 let runtime = tokio::runtime::Builder::new_current_thread()                     .enable_all()                     .build()                     .map_err(io_failure)?;                 runtime.block_on(fetch_async(url, limits, cancellation))             })             .map_err` | [1231](../../src/runtime_backends.rs#L1231) | receiver-type-required |
| `fetch` | `thread::Builder::new()             .name("tekes-http-backend".to_owned())             .spawn` | [1231](../../src/runtime_backends.rs#L1231) | receiver-type-required |
| `fetch` | `thread::Builder::new()             .name` | [1231](../../src/runtime_backends.rs#L1231) | receiver-type-required |
| `fetch` | `thread::Builder::new` | [1231](../../src/runtime_backends.rs#L1231) | external-constructor-callback-or-unresolved |
| `fetch` | `"tekes-http-backend".to_owned` | [1232](../../src/runtime_backends.rs#L1232) | receiver-type-required |
| `fetch` | `tokio::runtime::Builder::new_current_thread()                     .enable_all()                     .build()                     .map_err` | [1234](../../src/runtime_backends.rs#L1234) | receiver-type-required |
| `fetch` | `tokio::runtime::Builder::new_current_thread()                     .enable_all()                     .build` | [1234](../../src/runtime_backends.rs#L1234) | receiver-type-required |
| `fetch` | `tokio::runtime::Builder::new_current_thread()                     .enable_all` | [1234](../../src/runtime_backends.rs#L1234) | receiver-type-required |
| `fetch` | `tokio::runtime::Builder::new_current_thread` | [1234](../../src/runtime_backends.rs#L1234) | external-constructor-callback-or-unresolved |
| `fetch` | `runtime.block_on` | [1238](../../src/runtime_backends.rs#L1238) | receiver-type-required |
| `fetch` | `fetch_async` | [1238](../../src/runtime_backends.rs#L1238) | [tools::runtime_backends::fetch_async](../../src/runtime_backends.rs#L1331) |
| `fetch` | `thread                     .join()                     .map_err` | [1242](../../src/runtime_backends.rs#L1242) | receiver-type-required |
| `fetch` | `thread                     .join` | [1242](../../src/runtime_backends.rs#L1242) | receiver-type-required |
| `fetch` | `BackendFailure::Unknown` | [1244](../../src/runtime_backends.rs#L1244) | external-constructor-callback-or-unresolved |
| `fetch` | `"HTTP worker panicked".to_owned` | [1244](../../src/runtime_backends.rs#L1244) | receiver-type-required |
| `search` | `BackendOutcome::enter` | [1256](../../src/runtime_backends.rs#L1256) | [tools::runtime_backends::BackendOutcome::enter](../../src/runtime_backends.rs#L118) |
| `search` | `request.query.trim().is_empty` | [1259](../../src/runtime_backends.rs#L1259) | receiver-type-required |
| `search` | `request.query.trim` | [1259](../../src/runtime_backends.rs#L1259) | receiver-type-required |
| `search` | `(1..=10).contains` | [1259](../../src/runtime_backends.rs#L1259) | receiver-type-required |
| `search` | `BackendOutcome::Completed` | [1260](../../src/runtime_backends.rs#L1260), [1265](../../src/runtime_backends.rs#L1265), [1293](../../src/runtime_backends.rs#L1293) | external-constructor-callback-or-unresolved |
| `search` | `Err` | [1260](../../src/runtime_backends.rs#L1260), [1265](../../src/runtime_backends.rs#L1265), [1286](../../src/runtime_backends.rs#L1286) | external-constructor-callback-or-unresolved |
| `search` | `BackendFailure::Invalid` | [1260](../../src/runtime_backends.rs#L1260) | external-constructor-callback-or-unresolved |
| `search` | `"search query must be nonempty and max_results in 1..=10".to_owned` | [1261](../../src/runtime_backends.rs#L1261) | receiver-type-required |
| `search` | `cancellation.is_cancelled` | [1264](../../src/runtime_backends.rs#L1264) | receiver-type-required |
| `search` | `BackendOutcome::Unavailable` | [1268](../../src/runtime_backends.rs#L1268) | external-constructor-callback-or-unresolved |
| `search` | `"web-search-provider".to_owned` | [1269](../../src/runtime_backends.rs#L1269) | receiver-type-required |
| `search` | `"no credential-backed search provider is configured".to_owned` | [1270](../../src/runtime_backends.rs#L1270) | receiver-type-required |
| `search` | `provider             .search(request, &self.limits, cancellation)             .and_then` | [1273](../../src/runtime_backends.rs#L1273) | receiver-type-required |
| `search` | `provider             .search` | [1273](../../src/runtime_backends.rs#L1273) | receiver-type-required |
| `search` | `hits.iter().try_fold` | [1276](../../src/runtime_backends.rs#L1276) | receiver-type-required |
| `search` | `hits.iter` | [1276](../../src/runtime_backends.rs#L1276) | receiver-type-required |
| `search` | `total                         .checked_add(hit.title.len())                         .and_then(&#124;value&#124; value.checked_add(hit.url.len()))                         .and_then(&#124;value&#124; value.checked_add(hit.snippet.len()))                         .ok_or_else` | [1277](../../src/runtime_backends.rs#L1277) | receiver-type-required |
| `search` | `total                         .checked_add(hit.title.len())                         .and_then(&#124;value&#124; value.checked_add(hit.url.len()))                         .and_then` | [1277](../../src/runtime_backends.rs#L1277) | receiver-type-required |
| `search` | `total                         .checked_add(hit.title.len())                         .and_then` | [1277](../../src/runtime_backends.rs#L1277) | receiver-type-required |
| `search` | `total                         .checked_add` | [1277](../../src/runtime_backends.rs#L1277) | receiver-type-required |
| `search` | `hit.title.len` | [1278](../../src/runtime_backends.rs#L1278) | receiver-type-required |
| `search` | `value.checked_add` | [1279](../../src/runtime_backends.rs#L1279), [1280](../../src/runtime_backends.rs#L1280) | receiver-type-required |
| `search` | `hit.url.len` | [1279](../../src/runtime_backends.rs#L1279) | receiver-type-required |
| `search` | `hit.snippet.len` | [1280](../../src/runtime_backends.rs#L1280) | receiver-type-required |
| `search` | `BackendFailure::Limit` | [1282](../../src/runtime_backends.rs#L1282), [1286](../../src/runtime_backends.rs#L1286) | external-constructor-callback-or-unresolved |
| `search` | `"search result size overflow".to_owned` | [1282](../../src/runtime_backends.rs#L1282) | receiver-type-required |
| `search` | `"search results exceed the HTTP byte cap".to_owned` | [1287](../../src/runtime_backends.rs#L1287) | receiver-type-required |
| `search` | `hits.truncate` | [1290](../../src/runtime_backends.rs#L1290) | receiver-type-required |
| `search` | `request.max_results.into` | [1290](../../src/runtime_backends.rs#L1290) | receiver-type-required |
| `search` | `Ok` | [1291](../../src/runtime_backends.rs#L1291) | external-constructor-callback-or-unresolved |
| `fetch_async` | `Instant::now` | [1336](../../src/runtime_backends.rs#L1336) | external-constructor-callback-or-unresolved |
| `fetch_async` | `cancellation.is_cancelled` | [1338](../../src/runtime_backends.rs#L1338) | receiver-type-required |
| `fetch_async` | `Err` | [1339](../../src/runtime_backends.rs#L1339), [1363](../../src/runtime_backends.rs#L1363), [1377](../../src/runtime_backends.rs#L1377), [1386](../../src/runtime_backends.rs#L1386), [1403](../../src/runtime_backends.rs#L1403), [1418](../../src/runtime_backends.rs#L1418) | external-constructor-callback-or-unresolved |
| `fetch_async` | `reqwest::Url::parse(&url)             .map_err` | [1341](../../src/runtime_backends.rs#L1341) | receiver-type-required |
| `fetch_async` | `reqwest::Url::parse` | [1341](../../src/runtime_backends.rs#L1341) | external-constructor-callback-or-unresolved |
| `fetch_async` | `BackendFailure::Invalid` | [1342](../../src/runtime_backends.rs#L1342) | external-constructor-callback-or-unresolved |
| `fetch_async` | `error.to_string` | [1342](../../src/runtime_backends.rs#L1342), [1348](../../src/runtime_backends.rs#L1348), [1359](../../src/runtime_backends.rs#L1359), [1400](../../src/runtime_backends.rs#L1400) | receiver-type-required |
| `fetch_async` | `route_public_url` | [1343](../../src/runtime_backends.rs#L1343) | [tools::runtime_backends::route_public_url](../../src/runtime_backends.rs#L1486) |
| `fetch_async` | `route             .client_builder()?             .redirect(reqwest::redirect::Policy::none())             .build()             .map_err` | [1344](../../src/runtime_backends.rs#L1344) | receiver-type-required |
| `fetch_async` | `route             .client_builder()?             .redirect(reqwest::redirect::Policy::none())             .build` | [1344](../../src/runtime_backends.rs#L1344) | receiver-type-required |
| `fetch_async` | `route             .client_builder()?             .redirect` | [1344](../../src/runtime_backends.rs#L1344) | receiver-type-required |
| `fetch_async` | `route             .client_builder` | [1344](../../src/runtime_backends.rs#L1344) | receiver-type-required |
| `fetch_async` | `reqwest::redirect::Policy::none` | [1346](../../src/runtime_backends.rs#L1346) | external-constructor-callback-or-unresolved |
| `fetch_async` | `BackendFailure::Unavailable` | [1348](../../src/runtime_backends.rs#L1348) | external-constructor-callback-or-unresolved |
| `fetch_async` | `await_cancel_deadline(             client                 .get(parsed.clone())                 .header(USER_AGENT, WEB_FETCH_USER_AGENT)                 .header(ACCEPT, WEB_FETCH_ACCEPT)                 .send(),             &cancellation,             deadline,         )         .await?         .map_err` | [1349](../../src/runtime_backends.rs#L1349) | receiver-type-required |
| `fetch_async` | `await_cancel_deadline` | [1349](../../src/runtime_backends.rs#L1349), [1398](../../src/runtime_backends.rs#L1398) | [tools::runtime_backends::await_cancel_deadline](../../src/runtime_backends.rs#L1433) |
| `fetch_async` | `client                 .get(parsed.clone())                 .header(USER_AGENT, WEB_FETCH_USER_AGENT)                 .header(ACCEPT, WEB_FETCH_ACCEPT)                 .send` | [1350](../../src/runtime_backends.rs#L1350) | receiver-type-required |
| `fetch_async` | `client                 .get(parsed.clone())                 .header(USER_AGENT, WEB_FETCH_USER_AGENT)                 .header` | [1350](../../src/runtime_backends.rs#L1350) | receiver-type-required |
| `fetch_async` | `client                 .get(parsed.clone())                 .header` | [1350](../../src/runtime_backends.rs#L1350) | receiver-type-required |
| `fetch_async` | `client                 .get` | [1350](../../src/runtime_backends.rs#L1350) | receiver-type-required |
| `fetch_async` | `parsed.clone` | [1351](../../src/runtime_backends.rs#L1351) | receiver-type-required |
| `fetch_async` | `BackendFailure::Io` | [1359](../../src/runtime_backends.rs#L1359), [1400](../../src/runtime_backends.rs#L1400) | external-constructor-callback-or-unresolved |
| `fetch_async` | `response.status().is_redirection` | [1361](../../src/runtime_backends.rs#L1361) | receiver-type-required |
| `fetch_async` | `response.status` | [1361](../../src/runtime_backends.rs#L1361), [1376](../../src/runtime_backends.rs#L1376), [1377](../../src/runtime_backends.rs#L1377), [1390](../../src/runtime_backends.rs#L1390) | receiver-type-required |
| `fetch_async` | `BackendFailure::Limit` | [1363](../../src/runtime_backends.rs#L1363), [1386](../../src/runtime_backends.rs#L1386), [1403](../../src/runtime_backends.rs#L1403), [1418](../../src/runtime_backends.rs#L1418) | external-constructor-callback-or-unresolved |
| `fetch_async` | `"redirect limit exceeded".to_owned` | [1363](../../src/runtime_backends.rs#L1363), [1418](../../src/runtime_backends.rs#L1418) | receiver-type-required |
| `fetch_async` | `response                 .headers()                 .get(LOCATION)                 .ok_or_else(&#124;&#124; BackendFailure::Protocol("redirect has no Location".to_owned()))?                 .to_str()                 .map_err` | [1365](../../src/runtime_backends.rs#L1365) | receiver-type-required |
| `fetch_async` | `response                 .headers()                 .get(LOCATION)                 .ok_or_else(&#124;&#124; BackendFailure::Protocol("redirect has no Location".to_owned()))?                 .to_str` | [1365](../../src/runtime_backends.rs#L1365) | receiver-type-required |
| `fetch_async` | `response                 .headers()                 .get(LOCATION)                 .ok_or_else` | [1365](../../src/runtime_backends.rs#L1365) | receiver-type-required |
| `fetch_async` | `response                 .headers()                 .get` | [1365](../../src/runtime_backends.rs#L1365) | receiver-type-required |
| `fetch_async` | `response                 .headers` | [1365](../../src/runtime_backends.rs#L1365) | receiver-type-required |
| `fetch_async` | `BackendFailure::Protocol` | [1368](../../src/runtime_backends.rs#L1368), [1371](../../src/runtime_backends.rs#L1371) | external-constructor-callback-or-unresolved |
| `fetch_async` | `"redirect has no Location".to_owned` | [1368](../../src/runtime_backends.rs#L1368) | receiver-type-required |
| `fetch_async` | `"redirect Location is not UTF-8".to_owned` | [1371](../../src/runtime_backends.rs#L1371) | receiver-type-required |
| `fetch_async` | `resolve_web_redirect` | [1373](../../src/runtime_backends.rs#L1373) | [tools::runtime_backends::resolve_web_redirect](../../src/runtime_backends.rs#L1424) |
| `fetch_async` | `parsed.as_str` | [1373](../../src/runtime_backends.rs#L1373) | receiver-type-required |
| `fetch_async` | `response.status().is_success` | [1376](../../src/runtime_backends.rs#L1376) | receiver-type-required |
| `fetch_async` | `web_fetch_status_failure` | [1377](../../src/runtime_backends.rs#L1377) | [tools::runtime_backends::web_fetch_status_failure](../../src/runtime_backends.rs#L74) |
| `fetch_async` | `response             .headers()             .get(CONTENT_LENGTH)             .and_then(&#124;value&#124; value.to_str().ok())             .and_then(&#124;value&#124; value.parse::<usize>().ok())             .is_some_and` | [1379](../../src/runtime_backends.rs#L1379) | receiver-type-required |
| `fetch_async` | `response             .headers()             .get(CONTENT_LENGTH)             .and_then(&#124;value&#124; value.to_str().ok())             .and_then` | [1379](../../src/runtime_backends.rs#L1379) | receiver-type-required |
| `fetch_async` | `response             .headers()             .get(CONTENT_LENGTH)             .and_then` | [1379](../../src/runtime_backends.rs#L1379) | receiver-type-required |
| `fetch_async` | `response             .headers()             .get` | [1379](../../src/runtime_backends.rs#L1379), [1391](../../src/runtime_backends.rs#L1391) | receiver-type-required |
| `fetch_async` | `response             .headers` | [1379](../../src/runtime_backends.rs#L1379), [1391](../../src/runtime_backends.rs#L1391) | receiver-type-required |
| `fetch_async` | `value.to_str().ok` | [1382](../../src/runtime_backends.rs#L1382), [1394](../../src/runtime_backends.rs#L1394) | receiver-type-required |
| `fetch_async` | `value.to_str` | [1382](../../src/runtime_backends.rs#L1382), [1394](../../src/runtime_backends.rs#L1394) | receiver-type-required |
| `fetch_async` | `value.parse::<usize>().ok` | [1383](../../src/runtime_backends.rs#L1383) | receiver-type-required |
| `fetch_async` | `value.parse::<usize>` | [1383](../../src/runtime_backends.rs#L1383) | receiver-type-required |
| `fetch_async` | `"response Content-Length exceeds cap".to_owned` | [1387](../../src/runtime_backends.rs#L1387) | receiver-type-required |
| `fetch_async` | `response.status().as_u16` | [1390](../../src/runtime_backends.rs#L1390) | receiver-type-required |
| `fetch_async` | `response             .headers()             .get(CONTENT_TYPE)             .and_then(&#124;value&#124; value.to_str().ok())             .map` | [1391](../../src/runtime_backends.rs#L1391) | receiver-type-required |
| `fetch_async` | `response             .headers()             .get(CONTENT_TYPE)             .and_then` | [1391](../../src/runtime_backends.rs#L1391) | receiver-type-required |
| `fetch_async` | `Vec::new` | [1397](../../src/runtime_backends.rs#L1397) | external-constructor-callback-or-unresolved |
| `fetch_async` | `await_cancel_deadline(response.chunk(), &cancellation, deadline)             .await?             .map_err` | [1398](../../src/runtime_backends.rs#L1398) | receiver-type-required |
| `fetch_async` | `response.chunk` | [1398](../../src/runtime_backends.rs#L1398) | receiver-type-required |
| `fetch_async` | `body.len().saturating_add` | [1402](../../src/runtime_backends.rs#L1402) | receiver-type-required |
| `fetch_async` | `body.len` | [1402](../../src/runtime_backends.rs#L1402) | receiver-type-required |
| `fetch_async` | `chunk.len` | [1402](../../src/runtime_backends.rs#L1402) | receiver-type-required |
| `fetch_async` | `"response body exceeds cap".to_owned` | [1404](../../src/runtime_backends.rs#L1404) | receiver-type-required |
| `fetch_async` | `body.extend_from_slice` | [1407](../../src/runtime_backends.rs#L1407) | receiver-type-required |
| `fetch_async` | `extract_web_content` | [1409](../../src/runtime_backends.rs#L1409) | [tools::runtime_backends::extract_web_content](../../src/runtime_backends.rs#L1748) |
| `fetch_async` | `content_type.as_deref` | [1409](../../src/runtime_backends.rs#L1409) | receiver-type-required |
| `fetch_async` | `Ok` | [1410](../../src/runtime_backends.rs#L1410) | external-constructor-callback-or-unresolved |
| `fetch_async` | `parsed.to_string` | [1411](../../src/runtime_backends.rs#L1411) | receiver-type-required |
| `resolve_web_redirect` | `reqwest::Url::parse(current)         .map_err` | [1425](../../src/runtime_backends.rs#L1425) | receiver-type-required |
| `resolve_web_redirect` | `reqwest::Url::parse` | [1425](../../src/runtime_backends.rs#L1425) | external-constructor-callback-or-unresolved |
| `resolve_web_redirect` | `BackendFailure::Protocol` | [1426](../../src/runtime_backends.rs#L1426), [1430](../../src/runtime_backends.rs#L1430) | external-constructor-callback-or-unresolved |
| `resolve_web_redirect` | `error.to_string` | [1426](../../src/runtime_backends.rs#L1426), [1430](../../src/runtime_backends.rs#L1430) | receiver-type-required |
| `resolve_web_redirect` | `current         .join(location)         .map(&#124;url&#124; url.to_string())         .map_err` | [1427](../../src/runtime_backends.rs#L1427) | receiver-type-required |
| `resolve_web_redirect` | `current         .join(location)         .map` | [1427](../../src/runtime_backends.rs#L1427) | receiver-type-required |
| `resolve_web_redirect` | `current         .join` | [1427](../../src/runtime_backends.rs#L1427) | receiver-type-required |
| `resolve_web_redirect` | `url.to_string` | [1429](../../src/runtime_backends.rs#L1429) | receiver-type-required |
| `await_cancel_deadline` | `cancellation.is_cancelled` | [1440](../../src/runtime_backends.rs#L1440) | receiver-type-required |
| `await_cancel_deadline` | `Err` | [1441](../../src/runtime_backends.rs#L1441), [1444](../../src/runtime_backends.rs#L1444) | external-constructor-callback-or-unresolved |
| `await_cancel_deadline` | `deadline.checked_duration_since` | [1443](../../src/runtime_backends.rs#L1443) | receiver-type-required |
| `await_cancel_deadline` | `Instant::now` | [1443](../../src/runtime_backends.rs#L1443) | external-constructor-callback-or-unresolved |
| `client_builder` | `reqwest::Client::builder().no_proxy` | [1472](../../src/runtime_backends.rs#L1472) | receiver-type-required |
| `client_builder` | `reqwest::Client::builder` | [1472](../../src/runtime_backends.rs#L1472) | external-constructor-callback-or-unresolved |
| `client_builder` | `Ok` | [1473](../../src/runtime_backends.rs#L1473) | external-constructor-callback-or-unresolved |
| `client_builder` | `builder.resolve` | [1474](../../src/runtime_backends.rs#L1474) | receiver-type-required |
| `client_builder` | `builder.proxy` | [1475](../../src/runtime_backends.rs#L1475) | receiver-type-required |
| `client_builder` | `reqwest::Proxy::all(proxy.clone())                     .map_err` | [1476](../../src/runtime_backends.rs#L1476) | receiver-type-required |
| `client_builder` | `reqwest::Proxy::all` | [1476](../../src/runtime_backends.rs#L1476) | external-constructor-callback-or-unresolved |
| `client_builder` | `proxy.clone` | [1476](../../src/runtime_backends.rs#L1476) | receiver-type-required |
| `client_builder` | `BackendFailure::Unavailable` | [1477](../../src/runtime_backends.rs#L1477) | external-constructor-callback-or-unresolved |
| `client_builder` | `error.to_string` | [1477](../../src/runtime_backends.rs#L1477) | receiver-type-required |
| `route_public_url` | `route_public_url_with` | [1487](../../src/runtime_backends.rs#L1487) | [tools::runtime_backends::route_public_url_with](../../src/runtime_backends.rs#L1491) |
| `route_public_url` | `std::env::var(name).ok` | [1487](../../src/runtime_backends.rs#L1487) | receiver-type-required |
| `route_public_url` | `std::env::var` | [1487](../../src/runtime_backends.rs#L1487) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `Err` | [1496](../../src/runtime_backends.rs#L1496), [1501](../../src/runtime_backends.rs#L1501), [1515](../../src/runtime_backends.rs#L1515), [1530](../../src/runtime_backends.rs#L1530), [1548](../../src/runtime_backends.rs#L1548), [1556](../../src/runtime_backends.rs#L1556) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `BackendFailure::Denied` | [1496](../../src/runtime_backends.rs#L1496), [1501](../../src/runtime_backends.rs#L1501), [1515](../../src/runtime_backends.rs#L1515), [1530](../../src/runtime_backends.rs#L1530), [1556](../../src/runtime_backends.rs#L1556) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `"only HTTP(S) URLs are allowed".to_owned` | [1497](../../src/runtime_backends.rs#L1497) | receiver-type-required |
| `route_public_url_with` | `url.username().is_empty` | [1500](../../src/runtime_backends.rs#L1500) | receiver-type-required |
| `route_public_url_with` | `url.username` | [1500](../../src/runtime_backends.rs#L1500) | receiver-type-required |
| `route_public_url_with` | `url.password().is_some` | [1500](../../src/runtime_backends.rs#L1500) | receiver-type-required |
| `route_public_url_with` | `url.password` | [1500](../../src/runtime_backends.rs#L1500) | receiver-type-required |
| `route_public_url_with` | `"URL userinfo is forbidden".to_owned` | [1502](../../src/runtime_backends.rs#L1502) | receiver-type-required |
| `route_public_url_with` | `url         .host_str()         .ok_or_else(&#124;&#124; BackendFailure::Invalid("URL has no host".to_owned()))?         .trim_end_matches('.')         .to_ascii_lowercase` | [1505](../../src/runtime_backends.rs#L1505) | receiver-type-required |
| `route_public_url_with` | `url         .host_str()         .ok_or_else(&#124;&#124; BackendFailure::Invalid("URL has no host".to_owned()))?         .trim_end_matches` | [1505](../../src/runtime_backends.rs#L1505) | receiver-type-required |
| `route_public_url_with` | `url         .host_str()         .ok_or_else` | [1505](../../src/runtime_backends.rs#L1505) | receiver-type-required |
| `route_public_url_with` | `url         .host_str` | [1505](../../src/runtime_backends.rs#L1505) | receiver-type-required |
| `route_public_url_with` | `BackendFailure::Invalid` | [1507](../../src/runtime_backends.rs#L1507), [1521](../../src/runtime_backends.rs#L1521) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `"URL has no host".to_owned` | [1507](../../src/runtime_backends.rs#L1507) | receiver-type-required |
| `route_public_url_with` | `host.ends_with` | [1511](../../src/runtime_backends.rs#L1511), [1512](../../src/runtime_backends.rs#L1512), [1513](../../src/runtime_backends.rs#L1513) | receiver-type-required |
| `route_public_url_with` | `"local hostnames are forbidden".to_owned` | [1516](../../src/runtime_backends.rs#L1516) | receiver-type-required |
| `route_public_url_with` | `url         .port_or_known_default()         .ok_or_else` | [1519](../../src/runtime_backends.rs#L1519) | receiver-type-required |
| `route_public_url_with` | `url         .port_or_known_default` | [1519](../../src/runtime_backends.rs#L1519) | receiver-type-required |
| `route_public_url_with` | `"URL has no usable port".to_owned` | [1521](../../src/runtime_backends.rs#L1521) | receiver-type-required |
| `route_public_url_with` | `host         .strip_prefix('[')         .and_then(&#124;value&#124; value.strip_suffix(']'))         .unwrap_or` | [1522](../../src/runtime_backends.rs#L1522) | receiver-type-required |
| `route_public_url_with` | `host         .strip_prefix('[')         .and_then` | [1522](../../src/runtime_backends.rs#L1522) | receiver-type-required |
| `route_public_url_with` | `host         .strip_prefix` | [1522](../../src/runtime_backends.rs#L1522) | receiver-type-required |
| `route_public_url_with` | `value.strip_suffix` | [1524](../../src/runtime_backends.rs#L1524) | receiver-type-required |
| `route_public_url_with` | `address_literal.parse::<IpAddr>().ok` | [1526](../../src/runtime_backends.rs#L1526) | receiver-type-required |
| `route_public_url_with` | `address_literal.parse::<IpAddr>` | [1526](../../src/runtime_backends.rs#L1526) | receiver-type-required |
| `route_public_url_with` | `is_public_internet_address` | [1528](../../src/runtime_backends.rs#L1528), [1554](../../src/runtime_backends.rs#L1554) | [tools::runtime_backends::is_public_internet_address](../../src/runtime_backends.rs#L1681) |
| `route_public_url_with` | `"URL names a non-public address".to_owned` | [1531](../../src/runtime_backends.rs#L1531) | receiver-type-required |
| `route_public_url_with` | `outbound_proxy_for` | [1534](../../src/runtime_backends.rs#L1534) | [tools::runtime_backends::outbound_proxy_for](../../src/runtime_backends.rs#L1571) |
| `route_public_url_with` | `url.scheme` | [1534](../../src/runtime_backends.rs#L1534) | receiver-type-required |
| `route_public_url_with` | `Ok` | [1535](../../src/runtime_backends.rs#L1535), [1538](../../src/runtime_backends.rs#L1538), [1560](../../src/runtime_backends.rs#L1560) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `address_literal.to_owned` | [1539](../../src/runtime_backends.rs#L1539) | receiver-type-required |
| `route_public_url_with` | `SocketAddr::new` | [1540](../../src/runtime_backends.rs#L1540) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `(host.as_str(), port)         .to_socket_addrs()         .map_err(&#124;error&#124; BackendFailure::Unavailable(error.to_string()))?         .collect` | [1543](../../src/runtime_backends.rs#L1543) | receiver-type-required |
| `route_public_url_with` | `(host.as_str(), port)         .to_socket_addrs()         .map_err` | [1543](../../src/runtime_backends.rs#L1543) | receiver-type-required |
| `route_public_url_with` | `(host.as_str(), port)         .to_socket_addrs` | [1543](../../src/runtime_backends.rs#L1543) | receiver-type-required |
| `route_public_url_with` | `host.as_str` | [1543](../../src/runtime_backends.rs#L1543) | receiver-type-required |
| `route_public_url_with` | `BackendFailure::Unavailable` | [1545](../../src/runtime_backends.rs#L1545), [1548](../../src/runtime_backends.rs#L1548) | external-constructor-callback-or-unresolved |
| `route_public_url_with` | `error.to_string` | [1545](../../src/runtime_backends.rs#L1545) | receiver-type-required |
| `route_public_url_with` | `addresses.is_empty` | [1547](../../src/runtime_backends.rs#L1547) | receiver-type-required |
| `route_public_url_with` | `"DNS returned no address".to_owned` | [1549](../../src/runtime_backends.rs#L1549) | receiver-type-required |
| `route_public_url_with` | `addresses         .iter()         .any` | [1552](../../src/runtime_backends.rs#L1552) | receiver-type-required |
| `route_public_url_with` | `addresses         .iter` | [1552](../../src/runtime_backends.rs#L1552) | receiver-type-required |
| `route_public_url_with` | `address.ip` | [1554](../../src/runtime_backends.rs#L1554) | receiver-type-required |
| `route_public_url_with` | `"DNS resolved to a non-public address".to_owned` | [1557](../../src/runtime_backends.rs#L1557) | receiver-type-required |
| `outbound_proxy_for` | `env(name)             .map(&#124;v&#124; v.trim().to_owned())             .filter` | [1578](../../src/runtime_backends.rs#L1578) | receiver-type-required |
| `outbound_proxy_for` | `env(name)             .map` | [1578](../../src/runtime_backends.rs#L1578) | receiver-type-required |
| `outbound_proxy_for` | `env` | [1578](../../src/runtime_backends.rs#L1578) | external-constructor-callback-or-unresolved |
| `outbound_proxy_for` | `v.trim().to_owned` | [1579](../../src/runtime_backends.rs#L1579) | receiver-type-required |
| `outbound_proxy_for` | `v.trim` | [1579](../../src/runtime_backends.rs#L1579) | receiver-type-required |
| `outbound_proxy_for` | `v.is_empty` | [1580](../../src/runtime_backends.rs#L1580) | receiver-type-required |
| `outbound_proxy_for` | `names.iter().find_map` | [1587](../../src/runtime_backends.rs#L1587) | receiver-type-required |
| `outbound_proxy_for` | `names.iter` | [1587](../../src/runtime_backends.rs#L1587) | receiver-type-required |
| `outbound_proxy_for` | `read` | [1587](../../src/runtime_backends.rs#L1587), [1592](../../src/runtime_backends.rs#L1592), [1593](../../src/runtime_backends.rs#L1593) | external-constructor-callback-or-unresolved |
| `outbound_proxy_for` | `reqwest::Url::parse(&proxy).ok` | [1588](../../src/runtime_backends.rs#L1588) | receiver-type-required |
| `outbound_proxy_for` | `reqwest::Url::parse` | [1588](../../src/runtime_backends.rs#L1588) | external-constructor-callback-or-unresolved |
| `outbound_proxy_for` | `proxy.host_str().is_none` | [1589](../../src/runtime_backends.rs#L1589) | receiver-type-required |
| `outbound_proxy_for` | `proxy.host_str` | [1589](../../src/runtime_backends.rs#L1589) | receiver-type-required |
| `outbound_proxy_for` | `read("NO_PROXY")         .or_else(&#124;&#124; read("no_proxy"))         .unwrap_or_default` | [1592](../../src/runtime_backends.rs#L1592) | receiver-type-required |
| `outbound_proxy_for` | `read("NO_PROXY")         .or_else` | [1592](../../src/runtime_backends.rs#L1592) | receiver-type-required |
| `outbound_proxy_for` | `no_proxy_exempts` | [1595](../../src/runtime_backends.rs#L1595) | [tools::runtime_backends::no_proxy_exempts](../../src/runtime_backends.rs#L1605) |
| `outbound_proxy_for` | `Some` | [1598](../../src/runtime_backends.rs#L1598) | external-constructor-callback-or-unresolved |
| `no_proxy_exempts` | `host.trim_end_matches('.').to_ascii_lowercase` | [1606](../../src/runtime_backends.rs#L1606) | receiver-type-required |
| `no_proxy_exempts` | `host.trim_end_matches` | [1606](../../src/runtime_backends.rs#L1606) | receiver-type-required |
| `no_proxy_exempts` | `no_proxy.split` | [1607](../../src/runtime_backends.rs#L1607) | receiver-type-required |
| `no_proxy_exempts` | `c.is_whitespace` | [1607](../../src/runtime_backends.rs#L1607) | receiver-type-required |
| `no_proxy_exempts` | `raw.trim().to_ascii_lowercase` | [1608](../../src/runtime_backends.rs#L1608) | receiver-type-required |
| `no_proxy_exempts` | `raw.trim` | [1608](../../src/runtime_backends.rs#L1608) | receiver-type-required |
| `no_proxy_exempts` | `entry.is_empty` | [1609](../../src/runtime_backends.rs#L1609), [1634](../../src/runtime_backends.rs#L1634) | receiver-type-required |
| `no_proxy_exempts` | `entry.split_once` | [1615](../../src/runtime_backends.rs#L1615) | receiver-type-required |
| `no_proxy_exempts` | `network.parse::<IpAddr>` | [1617](../../src/runtime_backends.rs#L1617) | receiver-type-required |
| `no_proxy_exempts` | `bits.parse::<u32>` | [1617](../../src/runtime_backends.rs#L1617) | receiver-type-required |
| `no_proxy_exempts` | `cidr_contains` | [1618](../../src/runtime_backends.rs#L1618) | [tools::runtime_backends::cidr_contains](../../src/runtime_backends.rs#L1655) |
| `no_proxy_exempts` | `entry.trim_start_matches("*.").trim_start_matches` | [1624](../../src/runtime_backends.rs#L1624) | receiver-type-required |
| `no_proxy_exempts` | `entry.trim_start_matches` | [1624](../../src/runtime_backends.rs#L1624) | receiver-type-required |
| `no_proxy_exempts` | `entry.rsplit_once` | [1625](../../src/runtime_backends.rs#L1625) | receiver-type-required |
| `no_proxy_exempts` | `name.contains` | [1627](../../src/runtime_backends.rs#L1627) | receiver-type-required |
| `no_proxy_exempts` | `port.chars().all` | [1627](../../src/runtime_backends.rs#L1627) | receiver-type-required |
| `no_proxy_exempts` | `port.chars` | [1627](../../src/runtime_backends.rs#L1627) | receiver-type-required |
| `no_proxy_exempts` | `c.is_ascii_digit` | [1627](../../src/runtime_backends.rs#L1627) | receiver-type-required |
| `no_proxy_exempts` | `entry.trim_end_matches` | [1633](../../src/runtime_backends.rs#L1633) | receiver-type-required |
| `no_proxy_exempts` | `entry                 .trim_matches(&#124;c&#124; c == '[' &#124;&#124; c == ']')                 .parse::<IpAddr>` | [1639](../../src/runtime_backends.rs#L1639) | receiver-type-required |
| `no_proxy_exempts` | `entry                 .trim_matches` | [1639](../../src/runtime_backends.rs#L1639) | receiver-type-required |
| `no_proxy_exempts` | `host.ends_with` | [1648](../../src/runtime_backends.rs#L1648) | receiver-type-required |
| `cidr_contains` | `u32::from` | [1663](../../src/runtime_backends.rs#L1663) | external-constructor-callback-or-unresolved |
| `cidr_contains` | `u128::from` | [1671](../../src/runtime_backends.rs#L1671) | external-constructor-callback-or-unresolved |
| `is_public_internet_address` | `ip.octets` | [1684](../../src/runtime_backends.rs#L1684), [1705](../../src/runtime_backends.rs#L1705) | receiver-type-required |
| `is_public_internet_address` | `(64..=127).contains` | [1688](../../src/runtime_backends.rs#L1688) | receiver-type-required |
| `is_public_internet_address` | `(16..=31).contains` | [1690](../../src/runtime_backends.rs#L1690) | receiver-type-required |
| `is_public_internet_address` | `ip.segments` | [1701](../../src/runtime_backends.rs#L1701) | receiver-type-required |
| `is_public_internet_address` | `ip.to_ipv4_mapped` | [1702](../../src/runtime_backends.rs#L1702) | receiver-type-required |
| `is_public_internet_address` | `is_public_internet_address` | [1703](../../src/runtime_backends.rs#L1703), [1707](../../src/runtime_backends.rs#L1707), [1714](../../src/runtime_backends.rs#L1714) | [tools::runtime_backends::is_public_internet_address](../../src/runtime_backends.rs#L1681) |
| `is_public_internet_address` | `IpAddr::V4` | [1703](../../src/runtime_backends.rs#L1703), [1707](../../src/runtime_backends.rs#L1707), [1714](../../src/runtime_backends.rs#L1714) | external-constructor-callback-or-unresolved |
| `is_public_internet_address` | `octets[..12].iter().all` | [1706](../../src/runtime_backends.rs#L1706) | receiver-type-required |
| `is_public_internet_address` | `octets[..12].iter` | [1706](../../src/runtime_backends.rs#L1706) | receiver-type-required |
| `is_public_internet_address` | `std::net::Ipv4Addr::new` | [1707](../../src/runtime_backends.rs#L1707), [1714](../../src/runtime_backends.rs#L1714) | external-constructor-callback-or-unresolved |
| `is_public_internet_address` | `octets[4..12].iter().all` | [1712](../../src/runtime_backends.rs#L1712) | receiver-type-required |
| `is_public_internet_address` | `octets[4..12].iter` | [1712](../../src/runtime_backends.rs#L1712) | receiver-type-required |
| `is_public_internet_address` | `ip.is_unspecified` | [1721](../../src/runtime_backends.rs#L1721) | receiver-type-required |
| `is_public_internet_address` | `ip.is_loopback` | [1722](../../src/runtime_backends.rs#L1722) | receiver-type-required |
| `is_public_internet_address` | `ip.is_multicast` | [1723](../../src/runtime_backends.rs#L1723) | receiver-type-required |
| `extract_web_content` | `content_type.unwrap_or_default().to_ascii_lowercase` | [1749](../../src/runtime_backends.rs#L1749) | receiver-type-required |
| `extract_web_content` | `content_type.unwrap_or_default` | [1749](../../src/runtime_backends.rs#L1749) | receiver-type-required |
| `extract_web_content` | `normalized_type.is_empty` | [1750](../../src/runtime_backends.rs#L1750), [1774](../../src/runtime_backends.rs#L1774) | receiver-type-required |
| `extract_web_content` | `normalized_type.contains` | [1751](../../src/runtime_backends.rs#L1751), [1753](../../src/runtime_backends.rs#L1753), [1754](../../src/runtime_backends.rs#L1754), [1755](../../src/runtime_backends.rs#L1755), [1774](../../src/runtime_backends.rs#L1774) | receiver-type-required |
| `extract_web_content` | `normalized_type.starts_with` | [1752](../../src/runtime_backends.rs#L1752) | receiver-type-required |
| `extract_web_content` | `decode_web_text` | [1772](../../src/runtime_backends.rs#L1772) | [tools::runtime_backends::decode_web_text](../../src/runtime_backends.rs#L1794) |
| `extract_web_content` | `looks_like_html` | [1774](../../src/runtime_backends.rs#L1774) | [tools::runtime_backends::looks_like_html](../../src/runtime_backends.rs#L1801) |
| `extract_web_content` | `extract_html_text` | [1776](../../src/runtime_backends.rs#L1776) | [tools::runtime_backends::extract_html_text](../../src/runtime_backends.rs#L1810) |
| `extract_web_content` | `raw.trim().to_owned` | [1778](../../src/runtime_backends.rs#L1778) | receiver-type-required |
| `extract_web_content` | `raw.trim` | [1778](../../src/runtime_backends.rs#L1778) | receiver-type-required |
| `extract_web_content` | `html_is_under_rendered` | [1780](../../src/runtime_backends.rs#L1780) | [tools::runtime_backends::html_is_under_rendered](../../src/runtime_backends.rs#L1994) |
| `extract_web_content` | `truncate_characters` | [1781](../../src/runtime_backends.rs#L1781) | [tools::runtime_backends::truncate_characters](../../src/runtime_backends.rs#L2006) |
| `decode_web_text` | `std::str::from_utf8(bytes).map_or_else` | [1795](../../src/runtime_backends.rs#L1795) | receiver-type-required |
| `decode_web_text` | `std::str::from_utf8` | [1795](../../src/runtime_backends.rs#L1795) | external-constructor-callback-or-unresolved |
| `decode_web_text` | `bytes.iter().map(&#124;byte&#124; char::from(*byte)).collect` | [1796](../../src/runtime_backends.rs#L1796) | receiver-type-required |
| `decode_web_text` | `bytes.iter().map` | [1796](../../src/runtime_backends.rs#L1796) | receiver-type-required |
| `decode_web_text` | `bytes.iter` | [1796](../../src/runtime_backends.rs#L1796) | receiver-type-required |
| `decode_web_text` | `char::from` | [1796](../../src/runtime_backends.rs#L1796) | external-constructor-callback-or-unresolved |
| `looks_like_html` | `value         .chars()         .take(512)         .collect::<String>()         .to_ascii_lowercase` | [1802](../../src/runtime_backends.rs#L1802) | receiver-type-required |
| `looks_like_html` | `value         .chars()         .take(512)         .collect::<String>` | [1802](../../src/runtime_backends.rs#L1802) | receiver-type-required |
| `looks_like_html` | `value         .chars()         .take` | [1802](../../src/runtime_backends.rs#L1802) | receiver-type-required |
| `looks_like_html` | `value         .chars` | [1802](../../src/runtime_backends.rs#L1802) | receiver-type-required |
| `looks_like_html` | `head.contains` | [1807](../../src/runtime_backends.rs#L1807) | receiver-type-required |
| `extract_html_text` | `String::with_capacity` | [1811](../../src/runtime_backends.rs#L1811) | external-constructor-callback-or-unresolved |
| `extract_html_text` | `html.len` | [1811](../../src/runtime_backends.rs#L1811) | receiver-type-required |
| `extract_html_text` | `extract_html_title` | [1812](../../src/runtime_backends.rs#L1812) | [tools::runtime_backends::extract_html_title](../../src/runtime_backends.rs#L1872) |
| `extract_html_text` | `Vec::new` | [1813](../../src/runtime_backends.rs#L1813) | external-constructor-callback-or-unresolved |
| `extract_html_text` | `html.as_bytes` | [1814](../../src/runtime_backends.rs#L1814) | receiver-type-required |
| `extract_html_text` | `bytes.len` | [1816](../../src/runtime_backends.rs#L1816), [1821](../../src/runtime_backends.rs#L1821), [1831](../../src/runtime_backends.rs#L1831) | receiver-type-required |
| `extract_html_text` | `bytes[cursor..]                 .iter()                 .position(&#124;byte&#124; *byte == b'<')                 .map_or` | [1818](../../src/runtime_backends.rs#L1818) | receiver-type-required |
| `extract_html_text` | `bytes[cursor..]                 .iter()                 .position` | [1818](../../src/runtime_backends.rs#L1818) | receiver-type-required |
| `extract_html_text` | `bytes[cursor..]                 .iter` | [1818](../../src/runtime_backends.rs#L1818) | receiver-type-required |
| `extract_html_text` | `suppressed.is_empty` | [1822](../../src/runtime_backends.rs#L1822), [1850](../../src/runtime_backends.rs#L1850), [1855](../../src/runtime_backends.rs#L1855) | receiver-type-required |
| `extract_html_text` | `output.push_str` | [1823](../../src/runtime_backends.rs#L1823), [1851](../../src/runtime_backends.rs#L1851), [1858](../../src/runtime_backends.rs#L1858) | receiver-type-required |
| `extract_html_text` | `html[cursor..].starts_with` | [1828](../../src/runtime_backends.rs#L1828) | receiver-type-required |
| `extract_html_text` | `html[cursor + 4..]                 .find("-->")                 .map_or` | [1829](../../src/runtime_backends.rs#L1829) | receiver-type-required |
| `extract_html_text` | `html[cursor + 4..]                 .find` | [1829](../../src/runtime_backends.rs#L1829) | receiver-type-required |
| `extract_html_text` | `html[cursor..].find` | [1834](../../src/runtime_backends.rs#L1834) | receiver-type-required |
| `extract_html_text` | `html[cursor + 1..end].trim` | [1838](../../src/runtime_backends.rs#L1838) | receiver-type-required |
| `extract_html_text` | `raw_tag.starts_with` | [1839](../../src/runtime_backends.rs#L1839) | receiver-type-required |
| `extract_html_text` | `raw_tag             .trim_start_matches('/')             .split(&#124;character: char&#124; character.is_ascii_whitespace() &#124;&#124; character == '/')             .next()             .unwrap_or_default()             .to_ascii_lowercase` | [1840](../../src/runtime_backends.rs#L1840) | receiver-type-required |
| `extract_html_text` | `raw_tag             .trim_start_matches('/')             .split(&#124;character: char&#124; character.is_ascii_whitespace() &#124;&#124; character == '/')             .next()             .unwrap_or_default` | [1840](../../src/runtime_backends.rs#L1840) | receiver-type-required |
| `extract_html_text` | `raw_tag             .trim_start_matches('/')             .split(&#124;character: char&#124; character.is_ascii_whitespace() &#124;&#124; character == '/')             .next` | [1840](../../src/runtime_backends.rs#L1840) | receiver-type-required |
| `extract_html_text` | `raw_tag             .trim_start_matches('/')             .split` | [1840](../../src/runtime_backends.rs#L1840) | receiver-type-required |
| `extract_html_text` | `raw_tag             .trim_start_matches` | [1840](../../src/runtime_backends.rs#L1840) | receiver-type-required |
| `extract_html_text` | `character.is_ascii_whitespace` | [1842](../../src/runtime_backends.rs#L1842) | receiver-type-required |
| `extract_html_text` | `suppressed.last().is_some_and` | [1847](../../src/runtime_backends.rs#L1847) | receiver-type-required |
| `extract_html_text` | `suppressed.last` | [1847](../../src/runtime_backends.rs#L1847) | receiver-type-required |
| `extract_html_text` | `suppressed.pop` | [1848](../../src/runtime_backends.rs#L1848) | receiver-type-required |
| `extract_html_text` | `is_html_block` | [1850](../../src/runtime_backends.rs#L1850) | [tools::runtime_backends::is_html_block](../../src/runtime_backends.rs#L1889) |
| `extract_html_text` | `suppressed.push` | [1854](../../src/runtime_backends.rs#L1854) | receiver-type-required |
| `extract_html_text` | `name.clone` | [1854](../../src/runtime_backends.rs#L1854) | receiver-type-required |
| `extract_html_text` | `name.as_str` | [1856](../../src/runtime_backends.rs#L1856) | receiver-type-required |
| `extract_html_text` | `output.push` | [1857](../../src/runtime_backends.rs#L1857) | receiver-type-required |
| `extract_html_text` | `collapse_web_whitespace` | [1864](../../src/runtime_backends.rs#L1864), [1865](../../src/runtime_backends.rs#L1865) | [tools::runtime_backends::collapse_web_whitespace](../../src/runtime_backends.rs#L1973) |
| `extract_html_text` | `decode_html_entities` | [1864](../../src/runtime_backends.rs#L1864), [1865](../../src/runtime_backends.rs#L1865) | [tools::runtime_backends::decode_html_entities](../../src/runtime_backends.rs#L1914) |
| `extract_html_text` | `title.is_empty` | [1866](../../src/runtime_backends.rs#L1866) | receiver-type-required |
| `extract_html_text` | `body.starts_with` | [1866](../../src/runtime_backends.rs#L1866) | receiver-type-required |
| `extract_html_title` | `html.to_ascii_lowercase` | [1873](../../src/runtime_backends.rs#L1873) | receiver-type-required |
| `extract_html_title` | `lower.find` | [1874](../../src/runtime_backends.rs#L1874) | receiver-type-required |
| `extract_html_title` | `String::new` | [1875](../../src/runtime_backends.rs#L1875), [1878](../../src/runtime_backends.rs#L1878), [1884](../../src/runtime_backends.rs#L1884) | external-constructor-callback-or-unresolved |
| `extract_html_title` | `lower[open..].find('>').map` | [1877](../../src/runtime_backends.rs#L1877) | receiver-type-required |
| `extract_html_title` | `lower[open..].find` | [1877](../../src/runtime_backends.rs#L1877) | receiver-type-required |
| `extract_html_title` | `lower[open_end..]         .find("</title>")         .map` | [1880](../../src/runtime_backends.rs#L1880) | receiver-type-required |
| `extract_html_title` | `lower[open_end..]         .find` | [1880](../../src/runtime_backends.rs#L1880) | receiver-type-required |
| `extract_html_title` | `html[open_end..close].to_owned` | [1886](../../src/runtime_backends.rs#L1886) | receiver-type-required |
| `decode_html_entities` | `String::with_capacity` | [1915](../../src/runtime_backends.rs#L1915) | external-constructor-callback-or-unresolved |
| `decode_html_entities` | `value.len` | [1915](../../src/runtime_backends.rs#L1915) | receiver-type-required |
| `decode_html_entities` | `value.as_bytes` | [1916](../../src/runtime_backends.rs#L1916) | receiver-type-required |
| `decode_html_entities` | `bytes.len` | [1918](../../src/runtime_backends.rs#L1918) | receiver-type-required |
| `decode_html_entities` | `value[cursor..].chars().next().expect` | [1920](../../src/runtime_backends.rs#L1920) | receiver-type-required |
| `decode_html_entities` | `value[cursor..].chars().next` | [1920](../../src/runtime_backends.rs#L1920) | receiver-type-required |
| `decode_html_entities` | `value[cursor..].chars` | [1920](../../src/runtime_backends.rs#L1920) | receiver-type-required |
| `decode_html_entities` | `output.push` | [1921](../../src/runtime_backends.rs#L1921), [1926](../../src/runtime_backends.rs#L1926), [1956](../../src/runtime_backends.rs#L1956) | receiver-type-required |
| `decode_html_entities` | `character.len_utf8` | [1922](../../src/runtime_backends.rs#L1922) | receiver-type-required |
| `decode_html_entities` | `value[cursor..].find` | [1925](../../src/runtime_backends.rs#L1925) | receiver-type-required |
| `decode_html_entities` | `Some` | [1933](../../src/runtime_backends.rs#L1933), [1934](../../src/runtime_backends.rs#L1934), [1935](../../src/runtime_backends.rs#L1935), [1936](../../src/runtime_backends.rs#L1936), [1937](../../src/runtime_backends.rs#L1937), [1938](../../src/runtime_backends.rs#L1938), [1939](../../src/runtime_backends.rs#L1939), [1940](../../src/runtime_backends.rs#L1940), [1941](../../src/runtime_backends.rs#L1941), [1942](../../src/runtime_backends.rs#L1942), [1943](../../src/runtime_backends.rs#L1943), [1944](../../src/runtime_backends.rs#L1944), [1945](../../src/runtime_backends.rs#L1945), [1946](../../src/runtime_backends.rs#L1946), [1947](../../src/runtime_backends.rs#L1947), [1948](../../src/runtime_backends.rs#L1948), [1949](../../src/runtime_backends.rs#L1949) | external-constructor-callback-or-unresolved |
| `decode_html_entities` | `"&".to_owned` | [1933](../../src/runtime_backends.rs#L1933) | receiver-type-required |
| `decode_html_entities` | `"<".to_owned` | [1934](../../src/runtime_backends.rs#L1934) | receiver-type-required |
| `decode_html_entities` | `">".to_owned` | [1935](../../src/runtime_backends.rs#L1935) | receiver-type-required |
| `decode_html_entities` | `"\"".to_owned` | [1936](../../src/runtime_backends.rs#L1936) | receiver-type-required |
| `decode_html_entities` | `"'".to_owned` | [1937](../../src/runtime_backends.rs#L1937) | receiver-type-required |
| `decode_html_entities` | `" ".to_owned` | [1938](../../src/runtime_backends.rs#L1938) | receiver-type-required |
| `decode_html_entities` | `"—".to_owned` | [1939](../../src/runtime_backends.rs#L1939) | receiver-type-required |
| `decode_html_entities` | `"–".to_owned` | [1940](../../src/runtime_backends.rs#L1940) | receiver-type-required |
| `decode_html_entities` | `"…".to_owned` | [1941](../../src/runtime_backends.rs#L1941) | receiver-type-required |
| `decode_html_entities` | `"©".to_owned` | [1942](../../src/runtime_backends.rs#L1942) | receiver-type-required |
| `decode_html_entities` | `"®".to_owned` | [1943](../../src/runtime_backends.rs#L1943) | receiver-type-required |
| `decode_html_entities` | `"™".to_owned` | [1944](../../src/runtime_backends.rs#L1944) | receiver-type-required |
| `decode_html_entities` | `"’".to_owned` | [1945](../../src/runtime_backends.rs#L1945) | receiver-type-required |
| `decode_html_entities` | `"‘".to_owned` | [1946](../../src/runtime_backends.rs#L1946) | receiver-type-required |
| `decode_html_entities` | `"“".to_owned` | [1947](../../src/runtime_backends.rs#L1947) | receiver-type-required |
| `decode_html_entities` | `"”".to_owned` | [1948](../../src/runtime_backends.rs#L1948) | receiver-type-required |
| `decode_html_entities` | `"·".to_owned` | [1949](../../src/runtime_backends.rs#L1949) | receiver-type-required |
| `decode_html_entities` | `decode_numeric_entity` | [1950](../../src/runtime_backends.rs#L1950) | [tools::runtime_backends::decode_numeric_entity](../../src/runtime_backends.rs#L1963) |
| `decode_html_entities` | `output.push_str` | [1953](../../src/runtime_backends.rs#L1953) | receiver-type-required |
| `decode_numeric_entity` | `entity.strip_prefix("&#")?.strip_suffix` | [1964](../../src/runtime_backends.rs#L1964) | receiver-type-required |
| `decode_numeric_entity` | `entity.strip_prefix` | [1964](../../src/runtime_backends.rs#L1964) | receiver-type-required |
| `decode_numeric_entity` | `body         .strip_prefix('x')         .or_else(&#124;&#124; body.strip_prefix('X'))         .map_or_else(&#124;&#124; body.parse::<u32>(), &#124;hex&#124; u32::from_str_radix(hex, 16))         .ok` | [1965](../../src/runtime_backends.rs#L1965) | receiver-type-required |
| `decode_numeric_entity` | `body         .strip_prefix('x')         .or_else(&#124;&#124; body.strip_prefix('X'))         .map_or_else` | [1965](../../src/runtime_backends.rs#L1965) | receiver-type-required |
| `decode_numeric_entity` | `body         .strip_prefix('x')         .or_else` | [1965](../../src/runtime_backends.rs#L1965) | receiver-type-required |
| `decode_numeric_entity` | `body         .strip_prefix` | [1965](../../src/runtime_backends.rs#L1965) | receiver-type-required |
| `decode_numeric_entity` | `body.strip_prefix` | [1967](../../src/runtime_backends.rs#L1967) | receiver-type-required |
| `decode_numeric_entity` | `body.parse::<u32>` | [1968](../../src/runtime_backends.rs#L1968) | receiver-type-required |
| `decode_numeric_entity` | `u32::from_str_radix` | [1968](../../src/runtime_backends.rs#L1968) | external-constructor-callback-or-unresolved |
| `decode_numeric_entity` | `char::from_u32(value).map` | [1970](../../src/runtime_backends.rs#L1970) | receiver-type-required |
| `decode_numeric_entity` | `char::from_u32` | [1970](../../src/runtime_backends.rs#L1970) | external-constructor-callback-or-unresolved |
| `decode_numeric_entity` | `value.to_string` | [1970](../../src/runtime_backends.rs#L1970) | receiver-type-required |
| `collapse_web_whitespace` | `Vec::new` | [1974](../../src/runtime_backends.rs#L1974) | external-constructor-callback-or-unresolved |
| `collapse_web_whitespace` | `value.lines` | [1976](../../src/runtime_backends.rs#L1976) | receiver-type-required |
| `collapse_web_whitespace` | `raw.split_whitespace().collect::<Vec<_>>().join` | [1977](../../src/runtime_backends.rs#L1977) | receiver-type-required |
| `collapse_web_whitespace` | `raw.split_whitespace().collect::<Vec<_>>` | [1977](../../src/runtime_backends.rs#L1977) | receiver-type-required |
| `collapse_web_whitespace` | `raw.split_whitespace` | [1977](../../src/runtime_backends.rs#L1977) | receiver-type-required |
| `collapse_web_whitespace` | `line.is_empty` | [1978](../../src/runtime_backends.rs#L1978) | receiver-type-required |
| `collapse_web_whitespace` | `lines.is_empty` | [1979](../../src/runtime_backends.rs#L1979) | receiver-type-required |
| `collapse_web_whitespace` | `lines.push` | [1980](../../src/runtime_backends.rs#L1980), [1984](../../src/runtime_backends.rs#L1984) | receiver-type-required |
| `collapse_web_whitespace` | `String::new` | [1980](../../src/runtime_backends.rs#L1980) | external-constructor-callback-or-unresolved |
| `collapse_web_whitespace` | `lines.last().is_some_and` | [1988](../../src/runtime_backends.rs#L1988) | receiver-type-required |
| `collapse_web_whitespace` | `lines.last` | [1988](../../src/runtime_backends.rs#L1988) | receiver-type-required |
| `collapse_web_whitespace` | `lines.pop` | [1989](../../src/runtime_backends.rs#L1989) | receiver-type-required |
| `collapse_web_whitespace` | `lines.join` | [1991](../../src/runtime_backends.rs#L1991) | receiver-type-required |
| `html_is_under_rendered` | `value         .strip_prefix("Title:")         .and_then(&#124;value&#124; value.split_once('\n').map(&#124;(_, body)&#124; body))         .unwrap_or(value)         .trim` | [1995](../../src/runtime_backends.rs#L1995) | receiver-type-required |
| `html_is_under_rendered` | `value         .strip_prefix("Title:")         .and_then(&#124;value&#124; value.split_once('\n').map(&#124;(_, body)&#124; body))         .unwrap_or` | [1995](../../src/runtime_backends.rs#L1995) | receiver-type-required |
| `html_is_under_rendered` | `value         .strip_prefix("Title:")         .and_then` | [1995](../../src/runtime_backends.rs#L1995) | receiver-type-required |
| `html_is_under_rendered` | `value         .strip_prefix` | [1995](../../src/runtime_backends.rs#L1995) | receiver-type-required |
| `html_is_under_rendered` | `value.split_once('\n').map` | [1997](../../src/runtime_backends.rs#L1997) | receiver-type-required |
| `html_is_under_rendered` | `value.split_once` | [1997](../../src/runtime_backends.rs#L1997) | receiver-type-required |
| `html_is_under_rendered` | `meaningful.to_ascii_lowercase` | [2000](../../src/runtime_backends.rs#L2000) | receiver-type-required |
| `html_is_under_rendered` | `meaningful.chars().count` | [2001](../../src/runtime_backends.rs#L2001) | receiver-type-required |
| `html_is_under_rendered` | `meaningful.chars` | [2001](../../src/runtime_backends.rs#L2001) | receiver-type-required |
| `html_is_under_rendered` | `lower.contains` | [2002](../../src/runtime_backends.rs#L2002), [2003](../../src/runtime_backends.rs#L2003) | receiver-type-required |
| `truncate_characters` | `value.chars` | [2007](../../src/runtime_backends.rs#L2007) | receiver-type-required |
| `truncate_characters` | `characters.by_ref().take(limit).collect::<String>` | [2008](../../src/runtime_backends.rs#L2008) | receiver-type-required |
| `truncate_characters` | `characters.by_ref().take` | [2008](../../src/runtime_backends.rs#L2008) | receiver-type-required |
| `truncate_characters` | `characters.by_ref` | [2008](../../src/runtime_backends.rs#L2008) | receiver-type-required |
| `truncate_characters` | `characters.next().is_none` | [2009](../../src/runtime_backends.rs#L2009) | receiver-type-required |
| `truncate_characters` | `characters.next` | [2009](../../src/runtime_backends.rs#L2009) | receiver-type-required |
| `require_canonical_directory` | `Path::new` | [2020](../../src/runtime_backends.rs#L2020) | external-constructor-callback-or-unresolved |
| `require_canonical_directory` | `value.is_empty` | [2021](../../src/runtime_backends.rs#L2021) | receiver-type-required |
| `require_canonical_directory` | `path.is_absolute` | [2022](../../src/runtime_backends.rs#L2022) | receiver-type-required |
| `require_canonical_directory` | `value.contains` | [2023](../../src/runtime_backends.rs#L2023) | receiver-type-required |
| `require_canonical_directory` | `value.chars().any` | [2024](../../src/runtime_backends.rs#L2024) | receiver-type-required |
| `require_canonical_directory` | `value.chars` | [2024](../../src/runtime_backends.rs#L2024) | receiver-type-required |
| `require_canonical_directory` | `path             .components()             .any` | [2025](../../src/runtime_backends.rs#L2025) | receiver-type-required |
| `require_canonical_directory` | `path             .components` | [2025](../../src/runtime_backends.rs#L2025) | receiver-type-required |
| `require_canonical_directory` | `Err` | [2029](../../src/runtime_backends.rs#L2029), [2036](../../src/runtime_backends.rs#L2036) | external-constructor-callback-or-unresolved |
| `require_canonical_directory` | `BackendFailure::Invalid` | [2029](../../src/runtime_backends.rs#L2029), [2034](../../src/runtime_backends.rs#L2034), [2036](../../src/runtime_backends.rs#L2036), [2043](../../src/runtime_backends.rs#L2043) | external-constructor-callback-or-unresolved |
| `require_canonical_directory` | `fs::canonicalize(path)         .map_err` | [2033](../../src/runtime_backends.rs#L2033) | receiver-type-required |
| `require_canonical_directory` | `fs::canonicalize` | [2033](../../src/runtime_backends.rs#L2033) | external-constructor-callback-or-unresolved |
| `require_canonical_directory` | `canonical.is_dir` | [2035](../../src/runtime_backends.rs#L2035) | receiver-type-required |
| `require_canonical_directory` | `canonical         .to_str()         .map(ToOwned::to_owned)         .ok_or_else` | [2040](../../src/runtime_backends.rs#L2040) | receiver-type-required |
| `require_canonical_directory` | `canonical         .to_str()         .map` | [2040](../../src/runtime_backends.rs#L2040) | receiver-type-required |
| `require_canonical_directory` | `canonical         .to_str` | [2040](../../src/runtime_backends.rs#L2040) | receiver-type-required |
| `canonical_invocation_directory` | `Path::new` | [2051](../../src/runtime_backends.rs#L2051), [2066](../../src/runtime_backends.rs#L2066) | external-constructor-callback-or-unresolved |
| `canonical_invocation_directory` | `value.is_empty` | [2052](../../src/runtime_backends.rs#L2052) | receiver-type-required |
| `canonical_invocation_directory` | `value.contains` | [2053](../../src/runtime_backends.rs#L2053) | receiver-type-required |
| `canonical_invocation_directory` | `value.chars().any` | [2054](../../src/runtime_backends.rs#L2054) | receiver-type-required |
| `canonical_invocation_directory` | `value.chars` | [2054](../../src/runtime_backends.rs#L2054) | receiver-type-required |
| `canonical_invocation_directory` | `path             .components()             .any` | [2055](../../src/runtime_backends.rs#L2055) | receiver-type-required |
| `canonical_invocation_directory` | `path             .components` | [2055](../../src/runtime_backends.rs#L2055) | receiver-type-required |
| `canonical_invocation_directory` | `Err` | [2059](../../src/runtime_backends.rs#L2059), [2071](../../src/runtime_backends.rs#L2071) | external-constructor-callback-or-unresolved |
| `canonical_invocation_directory` | `BackendFailure::Invalid` | [2059](../../src/runtime_backends.rs#L2059), [2069](../../src/runtime_backends.rs#L2069), [2071](../../src/runtime_backends.rs#L2071), [2078](../../src/runtime_backends.rs#L2078) | external-constructor-callback-or-unresolved |
| `canonical_invocation_directory` | `path.is_absolute` | [2063](../../src/runtime_backends.rs#L2063) | receiver-type-required |
| `canonical_invocation_directory` | `path.to_path_buf` | [2064](../../src/runtime_backends.rs#L2064) | receiver-type-required |
| `canonical_invocation_directory` | `Path::new(primary_workspace_cwd).join` | [2066](../../src/runtime_backends.rs#L2066) | receiver-type-required |
| `canonical_invocation_directory` | `fs::canonicalize(&resolved)         .map_err` | [2068](../../src/runtime_backends.rs#L2068) | receiver-type-required |
| `canonical_invocation_directory` | `fs::canonicalize` | [2068](../../src/runtime_backends.rs#L2068) | external-constructor-callback-or-unresolved |
| `canonical_invocation_directory` | `canonical.is_dir` | [2070](../../src/runtime_backends.rs#L2070) | receiver-type-required |
| `canonical_invocation_directory` | `canonical         .to_str()         .map(ToOwned::to_owned)         .ok_or_else` | [2075](../../src/runtime_backends.rs#L2075) | receiver-type-required |
| `canonical_invocation_directory` | `canonical         .to_str()         .map` | [2075](../../src/runtime_backends.rs#L2075) | receiver-type-required |
| `canonical_invocation_directory` | `canonical         .to_str` | [2075](../../src/runtime_backends.rs#L2075) | receiver-type-required |
| `normalize_policy_roots` | `Vec::with_capacity` | [2086](../../src/runtime_backends.rs#L2086) | external-constructor-callback-or-unresolved |
| `normalize_policy_roots` | `values.len` | [2086](../../src/runtime_backends.rs#L2086) | receiver-type-required |
| `normalize_policy_roots` | `require_canonical_directory` | [2088](../../src/runtime_backends.rs#L2088) | [tools::runtime_backends::require_canonical_directory](../../src/runtime_backends.rs#L2019) |
| `normalize_policy_roots` | `covered_by` | [2089](../../src/runtime_backends.rs#L2089) | [tools::runtime_backends::covered_by](../../src/runtime_backends.rs#L2105) |
| `normalize_policy_roots` | `Err` | [2090](../../src/runtime_backends.rs#L2090), [2095](../../src/runtime_backends.rs#L2095) | external-constructor-callback-or-unresolved |
| `normalize_policy_roots` | `BackendFailure::Denied` | [2090](../../src/runtime_backends.rs#L2090) | external-constructor-callback-or-unresolved |
| `normalize_policy_roots` | `roots.iter().any` | [2094](../../src/runtime_backends.rs#L2094) | receiver-type-required |
| `normalize_policy_roots` | `roots.iter` | [2094](../../src/runtime_backends.rs#L2094) | receiver-type-required |
| `normalize_policy_roots` | `BackendFailure::Invalid` | [2095](../../src/runtime_backends.rs#L2095) | external-constructor-callback-or-unresolved |
| `normalize_policy_roots` | `roots.push` | [2099](../../src/runtime_backends.rs#L2099) | receiver-type-required |
| `normalize_policy_roots` | `roots.sort_by` | [2101](../../src/runtime_backends.rs#L2101) | receiver-type-required |
| `normalize_policy_roots` | `left.as_bytes().cmp` | [2101](../../src/runtime_backends.rs#L2101) | receiver-type-required |
| `normalize_policy_roots` | `left.as_bytes` | [2101](../../src/runtime_backends.rs#L2101) | receiver-type-required |
| `normalize_policy_roots` | `right.as_bytes` | [2101](../../src/runtime_backends.rs#L2101) | receiver-type-required |
| `normalize_policy_roots` | `Ok` | [2102](../../src/runtime_backends.rs#L2102) | external-constructor-callback-or-unresolved |
| `covered_by` | `roots         .iter()         .any` | [2106](../../src/runtime_backends.rs#L2106) | receiver-type-required |
| `covered_by` | `roots         .iter` | [2106](../../src/runtime_backends.rs#L2106) | receiver-type-required |
| `covered_by` | `Path::new(path).starts_with` | [2108](../../src/runtime_backends.rs#L2108) | receiver-type-required |
| `covered_by` | `Path::new` | [2108](../../src/runtime_backends.rs#L2108) | external-constructor-callback-or-unresolved |
| `validate_job_id` | `id.is_empty` | [2112](../../src/runtime_backends.rs#L2112) | receiver-type-required |
| `validate_job_id` | `id.len` | [2113](../../src/runtime_backends.rs#L2113) | receiver-type-required |
| `validate_job_id` | `id             .bytes()             .all` | [2114](../../src/runtime_backends.rs#L2114) | receiver-type-required |
| `validate_job_id` | `id             .bytes` | [2114](../../src/runtime_backends.rs#L2114) | receiver-type-required |
| `validate_job_id` | `byte.is_ascii_alphanumeric` | [2116](../../src/runtime_backends.rs#L2116) | receiver-type-required |
| `validate_job_id` | `Err` | [2118](../../src/runtime_backends.rs#L2118) | external-constructor-callback-or-unresolved |
| `validate_job_id` | `BackendFailure::Invalid` | [2118](../../src/runtime_backends.rs#L2118) | external-constructor-callback-or-unresolved |
| `validate_job_id` | `"job id must match [A-Za-z0-9_-]{1,128}".to_owned` | [2119](../../src/runtime_backends.rs#L2119) | receiver-type-required |
| `validate_job_id` | `Ok` | [2122](../../src/runtime_backends.rs#L2122) | external-constructor-callback-or-unresolved |
| `drain_tail` | `VecDeque::with_capacity` | [2126](../../src/runtime_backends.rs#L2126) | external-constructor-callback-or-unresolved |
| `drain_tail` | `cap.min` | [2126](../../src/runtime_backends.rs#L2126) | receiver-type-required |
| `drain_tail` | `reader.read(&mut buffer).map_err` | [2129](../../src/runtime_backends.rs#L2129) | receiver-type-required |
| `drain_tail` | `reader.read` | [2129](../../src/runtime_backends.rs#L2129) | receiver-type-required |
| `drain_tail` | `tail.extend` | [2133](../../src/runtime_backends.rs#L2133) | receiver-type-required |
| `drain_tail` | `tail.len` | [2134](../../src/runtime_backends.rs#L2134) | receiver-type-required |
| `drain_tail` | `tail.pop_front` | [2135](../../src/runtime_backends.rs#L2135) | receiver-type-required |
| `drain_tail` | `tail.iter().copied().collect` | [2137](../../src/runtime_backends.rs#L2137) | receiver-type-required |
| `drain_tail` | `tail.iter().copied` | [2137](../../src/runtime_backends.rs#L2137) | receiver-type-required |
| `drain_tail` | `tail.iter` | [2137](../../src/runtime_backends.rs#L2137) | receiver-type-required |
| `drain_tail` | `AtomicPublisher::replace(path, &bytes).map_err` | [2138](../../src/runtime_backends.rs#L2138) | receiver-type-required |
| `drain_tail` | `AtomicPublisher::replace` | [2138](../../src/runtime_backends.rs#L2138) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `drain_tail` | `Ok` | [2140](../../src/runtime_backends.rs#L2140) | external-constructor-callback-or-unresolved |
| `signal_group` | `i32::try_from(pid).map_err` | [2144](../../src/runtime_backends.rs#L2144) | receiver-type-required |
| `signal_group` | `i32::try_from` | [2144](../../src/runtime_backends.rs#L2144) | external-constructor-callback-or-unresolved |
| `signal_group` | `BackendFailure::Invalid` | [2144](../../src/runtime_backends.rs#L2144) | external-constructor-callback-or-unresolved |
| `signal_group` | `"pid overflow".to_owned` | [2144](../../src/runtime_backends.rs#L2144) | receiver-type-required |
| `signal_group` | `libc::kill` | [2147](../../src/runtime_backends.rs#L2147) | external-constructor-callback-or-unresolved |
| `signal_group` | `Ok` | [2149](../../src/runtime_backends.rs#L2149), [2153](../../src/runtime_backends.rs#L2153) | external-constructor-callback-or-unresolved |
| `signal_group` | `std::io::Error::last_os_error` | [2151](../../src/runtime_backends.rs#L2151) | external-constructor-callback-or-unresolved |
| `signal_group` | `error.raw_os_error` | [2152](../../src/runtime_backends.rs#L2152) | receiver-type-required |
| `signal_group` | `Some` | [2152](../../src/runtime_backends.rs#L2152) | external-constructor-callback-or-unresolved |
| `signal_group` | `Err` | [2155](../../src/runtime_backends.rs#L2155) | external-constructor-callback-or-unresolved |
| `signal_group` | `io_failure` | [2155](../../src/runtime_backends.rs#L2155) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `terminate_group` | `signal_group` | [2161](../../src/runtime_backends.rs#L2161), [2169](../../src/runtime_backends.rs#L2169) | [tools::runtime_backends::signal_group](../../src/runtime_backends.rs#L2143) |
| `terminate_group` | `child.id` | [2161](../../src/runtime_backends.rs#L2161), [2169](../../src/runtime_backends.rs#L2169) | receiver-type-required |
| `terminate_group` | `Instant::now` | [2162](../../src/runtime_backends.rs#L2162), [2163](../../src/runtime_backends.rs#L2163) | external-constructor-callback-or-unresolved |
| `terminate_group` | `child.try_wait().ok().flatten().is_some` | [2164](../../src/runtime_backends.rs#L2164) | receiver-type-required |
| `terminate_group` | `child.try_wait().ok().flatten` | [2164](../../src/runtime_backends.rs#L2164) | receiver-type-required |
| `terminate_group` | `child.try_wait().ok` | [2164](../../src/runtime_backends.rs#L2164) | receiver-type-required |
| `terminate_group` | `child.try_wait` | [2164](../../src/runtime_backends.rs#L2164) | receiver-type-required |
| `terminate_group` | `thread::sleep` | [2167](../../src/runtime_backends.rs#L2167) | external-constructor-callback-or-unresolved |
| `terminate_group` | `Duration::from_millis` | [2167](../../src/runtime_backends.rs#L2167) | external-constructor-callback-or-unresolved |
| `terminate_group` | `child.wait` | [2170](../../src/runtime_backends.rs#L2170) | receiver-type-required |
| `read` | `fs::read_to_string(format!("/proc/{pid}/stat")).ok` | [2211](../../src/runtime_backends.rs#L2211) | receiver-type-required |
| `read` | `fs::read_to_string` | [2211](../../src/runtime_backends.rs#L2211), [2214](../../src/runtime_backends.rs#L2214) | external-constructor-callback-or-unresolved |
| `read` | `stat.rfind` | [2212](../../src/runtime_backends.rs#L2212) | receiver-type-required |
| `read` | `stat.get(end + 2..)?.split_whitespace().collect` | [2213](../../src/runtime_backends.rs#L2213) | receiver-type-required |
| `read` | `stat.get(end + 2..)?.split_whitespace` | [2213](../../src/runtime_backends.rs#L2213) | receiver-type-required |
| `read` | `stat.get` | [2213](../../src/runtime_backends.rs#L2213) | receiver-type-required |
| `read` | `fs::read_to_string(format!("/proc/{pid}/status")).ok` | [2214](../../src/runtime_backends.rs#L2214) | receiver-type-required |
| `read` | `status.lines().find_map` | [2215](../../src/runtime_backends.rs#L2215) | receiver-type-required |
| `read` | `status.lines` | [2215](../../src/runtime_backends.rs#L2215) | receiver-type-required |
| `read` | `line.strip_prefix("NSpid:")?                 .split_whitespace()                 .last()?                 .parse()                 .ok` | [2216](../../src/runtime_backends.rs#L2216) | receiver-type-required |
| `read` | `line.strip_prefix("NSpid:")?                 .split_whitespace()                 .last()?                 .parse` | [2216](../../src/runtime_backends.rs#L2216) | receiver-type-required |
| `read` | `line.strip_prefix("NSpid:")?                 .split_whitespace()                 .last` | [2216](../../src/runtime_backends.rs#L2216) | receiver-type-required |
| `read` | `line.strip_prefix("NSpid:")?                 .split_whitespace` | [2216](../../src/runtime_backends.rs#L2216) | receiver-type-required |
| `read` | `line.strip_prefix` | [2216](../../src/runtime_backends.rs#L2216) | receiver-type-required |
| `read` | `fs::metadata(format!("/proc/{pid}/ns/pid")).ok` | [2222](../../src/runtime_backends.rs#L2222) | receiver-type-required |
| `read` | `fs::metadata` | [2222](../../src/runtime_backends.rs#L2222) | external-constructor-callback-or-unresolved |
| `read` | `Some` | [2223](../../src/runtime_backends.rs#L2223) | external-constructor-callback-or-unresolved |
| `read` | `fields.get(1)?.parse().ok` | [2225](../../src/runtime_backends.rs#L2225) | receiver-type-required |
| `read` | `fields.get(1)?.parse` | [2225](../../src/runtime_backends.rs#L2225) | receiver-type-required |
| `read` | `fields.get` | [2225](../../src/runtime_backends.rs#L2225), [2226](../../src/runtime_backends.rs#L2226), [2227](../../src/runtime_backends.rs#L2227), [2228](../../src/runtime_backends.rs#L2228) | receiver-type-required |
| `read` | `fields.get(2)?.parse().ok` | [2226](../../src/runtime_backends.rs#L2226) | receiver-type-required |
| `read` | `fields.get(2)?.parse` | [2226](../../src/runtime_backends.rs#L2226) | receiver-type-required |
| `read` | `fields.get(3)?.parse().ok` | [2227](../../src/runtime_backends.rs#L2227) | receiver-type-required |
| `read` | `fields.get(3)?.parse` | [2227](../../src/runtime_backends.rs#L2227) | receiver-type-required |
| `read` | `fields.get(19)?.to_string` | [2228](../../src/runtime_backends.rs#L2228) | receiver-type-required |
| `read` | `namespace.dev` | [2230](../../src/runtime_backends.rs#L2230) | receiver-type-required |
| `read` | `namespace.ino` | [2231](../../src/runtime_backends.rs#L2231) | receiver-type-required |
| `is_live` | `Self::read(self.pid).is_some_and` | [2241](../../src/runtime_backends.rs#L2241) | receiver-type-required |
| `is_live` | `Self::read` | [2241](../../src/runtime_backends.rs#L2241) | [tools::runtime_backends::LinuxProcessIdentity::read](../../src/runtime_backends.rs#L2209) |
| `is_live` | `current.same_namespace` | [2244](../../src/runtime_backends.rs#L2244) | receiver-type-required |
| `matches_record` | `Some` | [2252](../../src/runtime_backends.rs#L2252), [2253](../../src/runtime_backends.rs#L2253) | external-constructor-callback-or-unresolved |
| `matches_record` | `record.process_start.as_deref` | [2253](../../src/runtime_backends.rs#L2253) | receiver-type-required |
| `matches_record` | `self.wrapper_start.as_str` | [2253](../../src/runtime_backends.rs#L2253) | receiver-type-required |
| `matches_record` | `self.runner.same_namespace` | [2254](../../src/runtime_backends.rs#L2254) | receiver-type-required |
| `matches_current_runner` | `LinuxProcessIdentity::read(std::process::id()).is_some_and` | [2263](../../src/runtime_backends.rs#L2263) | receiver-type-required |
| `matches_current_runner` | `LinuxProcessIdentity::read` | [2263](../../src/runtime_backends.rs#L2263) | [tools::runtime_backends::LinuxProcessIdentity::read](../../src/runtime_backends.rs#L2209) |
| `matches_current_runner` | `std::process::id` | [2263](../../src/runtime_backends.rs#L2263) | external-constructor-callback-or-unresolved |
| `matches_current_runner` | `current.same_namespace` | [2266](../../src/runtime_backends.rs#L2266) | receiver-type-required |
| `namespace_is_live` | `self.init.is_live` | [2271](../../src/runtime_backends.rs#L2271) | receiver-type-required |
| `stop_namespace` | `self.stop_handle` | [2275](../../src/runtime_backends.rs#L2275) | [tools::runtime_backends::NamespaceRunnerIdentity::stop_handle](../../src/runtime_backends.rs#L2281) |
| `stop_namespace` | `handle.kill` | [2276](../../src/runtime_backends.rs#L2276) | receiver-type-required |
| `stop_namespace` | `Ok` | [2278](../../src/runtime_backends.rs#L2278) | external-constructor-callback-or-unresolved |
| `stop_handle` | `self.init.is_live` | [2283](../../src/runtime_backends.rs#L2283), [2310](../../src/runtime_backends.rs#L2310) | receiver-type-required |
| `stop_handle` | `Ok` | [2284](../../src/runtime_backends.rs#L2284), [2292](../../src/runtime_backends.rs#L2292), [2313](../../src/runtime_backends.rs#L2313), [2329](../../src/runtime_backends.rs#L2329) | external-constructor-callback-or-unresolved |
| `stop_handle` | `libc::syscall` | [2288](../../src/runtime_backends.rs#L2288) | external-constructor-callback-or-unresolved |
| `stop_handle` | `std::io::Error::last_os_error` | [2290](../../src/runtime_backends.rs#L2290) | external-constructor-callback-or-unresolved |
| `stop_handle` | `error.raw_os_error` | [2291](../../src/runtime_backends.rs#L2291) | receiver-type-required |
| `stop_handle` | `Some` | [2291](../../src/runtime_backends.rs#L2291), [2329](../../src/runtime_backends.rs#L2329) | external-constructor-callback-or-unresolved |
| `stop_handle` | `Err` | [2294](../../src/runtime_backends.rs#L2294), [2311](../../src/runtime_backends.rs#L2311), [2326](../../src/runtime_backends.rs#L2326) | external-constructor-callback-or-unresolved |
| `stop_handle` | `BackendFailure::Unavailable` | [2294](../../src/runtime_backends.rs#L2294) | external-constructor-callback-or-unresolved |
| `stop_handle` | `NamespaceStopHandle` | [2299](../../src/runtime_backends.rs#L2299) | external-constructor-callback-or-unresolved |
| `stop_handle` | `std::os::fd::OwnedFd::from_raw_fd` | [2299](../../src/runtime_backends.rs#L2299) | external-constructor-callback-or-unresolved |
| `stop_handle` | `BackendFailure::Unknown` | [2301](../../src/runtime_backends.rs#L2301) | external-constructor-callback-or-unresolved |
| `stop_handle` | `"namespace launch lineage is no longer verifiable".to_owned` | [2301](../../src/runtime_backends.rs#L2301) | receiver-type-required |
| `stop_handle` | `LinuxProcessIdentity::read` | [2304](../../src/runtime_backends.rs#L2304), [2305](../../src/runtime_backends.rs#L2305), [2306](../../src/runtime_backends.rs#L2306) | [tools::runtime_backends::LinuxProcessIdentity::read](../../src/runtime_backends.rs#L2209) |
| `stop_handle` | `std::process::id` | [2306](../../src/runtime_backends.rs#L2306) | external-constructor-callback-or-unresolved |
| `stop_handle` | `invalid` | [2311](../../src/runtime_backends.rs#L2311), [2326](../../src/runtime_backends.rs#L2326) | external-constructor-callback-or-unresolved |
| `stop_handle` | `init.same_namespace` | [2322](../../src/runtime_backends.rs#L2322), [2323](../../src/runtime_backends.rs#L2323) | receiver-type-required |
| `stop_handle` | `wrapper.same_namespace` | [2324](../../src/runtime_backends.rs#L2324) | receiver-type-required |
| `stop_handle` | `handle.send_signal` | [2328](../../src/runtime_backends.rs#L2328) | receiver-type-required |
| `kill` | `self.send_signal` | [2339](../../src/runtime_backends.rs#L2339) | [tools::runtime_backends::NamespaceStopHandle::send_signal](../../src/runtime_backends.rs#L2342) |
| `send_signal` | `libc::syscall` | [2347](../../src/runtime_backends.rs#L2347) | external-constructor-callback-or-unresolved |
| `send_signal` | `self.0.as_raw_fd` | [2349](../../src/runtime_backends.rs#L2349) | receiver-type-required |
| `send_signal` | `std::ptr::null::<libc::siginfo_t>` | [2351](../../src/runtime_backends.rs#L2351) | external-constructor-callback-or-unresolved |
| `send_signal` | `Ok` | [2356](../../src/runtime_backends.rs#L2356), [2360](../../src/runtime_backends.rs#L2360) | external-constructor-callback-or-unresolved |
| `send_signal` | `std::io::Error::last_os_error` | [2358](../../src/runtime_backends.rs#L2358) | external-constructor-callback-or-unresolved |
| `send_signal` | `error.raw_os_error` | [2359](../../src/runtime_backends.rs#L2359) | receiver-type-required |
| `send_signal` | `Some` | [2359](../../src/runtime_backends.rs#L2359) | external-constructor-callback-or-unresolved |
| `send_signal` | `Err` | [2362](../../src/runtime_backends.rs#L2362) | external-constructor-callback-or-unresolved |
| `send_signal` | `io_failure` | [2362](../../src/runtime_backends.rs#L2362) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `namespace_identity_channel` | `std::os::unix::net::UnixStream::pair().map_err` | [2376](../../src/runtime_backends.rs#L2376) | receiver-type-required |
| `namespace_identity_channel` | `std::os::unix::net::UnixStream::pair` | [2376](../../src/runtime_backends.rs#L2376) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `broker         .set_read_timeout(Some(Duration::from_secs(10)))         .map_err` | [2377](../../src/runtime_backends.rs#L2377) | receiver-type-required |
| `namespace_identity_channel` | `broker         .set_read_timeout` | [2377](../../src/runtime_backends.rs#L2377) | receiver-type-required |
| `namespace_identity_channel` | `Some` | [2378](../../src/runtime_backends.rs#L2378) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `Duration::from_secs` | [2378](../../src/runtime_backends.rs#L2378) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `libc::setsockopt` | [2383](../../src/runtime_backends.rs#L2383) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `broker.as_raw_fd` | [2384](../../src/runtime_backends.rs#L2384) | receiver-type-required |
| `namespace_identity_channel` | `(&enabled as *const libc::c_int).cast` | [2387](../../src/runtime_backends.rs#L2387) | receiver-type-required |
| `namespace_identity_channel` | `std::mem::size_of_val` | [2388](../../src/runtime_backends.rs#L2388) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `Err` | [2392](../../src/runtime_backends.rs#L2392) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `io_failure` | [2392](../../src/runtime_backends.rs#L2392) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `namespace_identity_channel` | `std::io::Error::last_os_error` | [2392](../../src/runtime_backends.rs#L2392) | external-constructor-callback-or-unresolved |
| `namespace_identity_channel` | `Ok` | [2394](../../src/runtime_backends.rs#L2394) | external-constructor-callback-or-unresolved |
| `announce_namespace_runner` | `File::from_raw_fd` | [2404](../../src/runtime_backends.rs#L2404) | external-constructor-callback-or-unresolved |
| `announce_namespace_runner` | `stdin.write_all(&[0xa1]).map_err` | [2405](../../src/runtime_backends.rs#L2405) | receiver-type-required |
| `announce_namespace_runner` | `stdin.write_all` | [2405](../../src/runtime_backends.rs#L2405) | receiver-type-required |
| `namespace_runner_peer` | `byte.as_mut_ptr().cast` | [2413](../../src/runtime_backends.rs#L2413) | receiver-type-required |
| `namespace_runner_peer` | `byte.as_mut_ptr` | [2413](../../src/runtime_backends.rs#L2413) | receiver-type-required |
| `namespace_runner_peer` | `byte.len` | [2414](../../src/runtime_backends.rs#L2414) | receiver-type-required |
| `namespace_runner_peer` | `std::mem::zeroed` | [2420](../../src/runtime_backends.rs#L2420) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `control.as_mut_ptr().cast` | [2423](../../src/runtime_backends.rs#L2423) | receiver-type-required |
| `namespace_runner_peer` | `control.as_mut_ptr` | [2423](../../src/runtime_backends.rs#L2423) | receiver-type-required |
| `namespace_runner_peer` | `std::mem::size_of_val` | [2424](../../src/runtime_backends.rs#L2424) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `libc::recvmsg` | [2426](../../src/runtime_backends.rs#L2426) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `channel.as_raw_fd` | [2426](../../src/runtime_backends.rs#L2426) | receiver-type-required |
| `namespace_runner_peer` | `Err` | [2428](../../src/runtime_backends.rs#L2428), [2431](../../src/runtime_backends.rs#L2431), [2454](../../src/runtime_backends.rs#L2454) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `io_failure` | [2428](../../src/runtime_backends.rs#L2428) | [tools::runtime_backends::io_failure](../../src/runtime_backends.rs#L2549) |
| `namespace_runner_peer` | `std::io::Error::last_os_error` | [2428](../../src/runtime_backends.rs#L2428) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `BackendFailure::Protocol` | [2431](../../src/runtime_backends.rs#L2431) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `"missing namespace runner credentials".to_owned` | [2432](../../src/runtime_backends.rs#L2432) | receiver-type-required |
| `namespace_runner_peer` | `libc::CMSG_FIRSTHDR` | [2438](../../src/runtime_backends.rs#L2438) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `header.is_null` | [2439](../../src/runtime_backends.rs#L2439) | receiver-type-required |
| `namespace_runner_peer` | `libc::CMSG_LEN` | [2443](../../src/runtime_backends.rs#L2443) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `std::mem::size_of::<libc::ucred>` | [2443](../../src/runtime_backends.rs#L2443) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `std::ptr::read_unaligned` | [2446](../../src/runtime_backends.rs#L2446) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `libc::CMSG_DATA(header).cast::<libc::ucred>` | [2446](../../src/runtime_backends.rs#L2446) | receiver-type-required |
| `namespace_runner_peer` | `libc::CMSG_DATA` | [2446](../../src/runtime_backends.rs#L2446) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `libc::geteuid` | [2447](../../src/runtime_backends.rs#L2447) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `Ok` | [2448](../../src/runtime_backends.rs#L2448) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `libc::CMSG_NXTHDR` | [2451](../../src/runtime_backends.rs#L2451) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `BackendFailure::Denied` | [2454](../../src/runtime_backends.rs#L2454) | external-constructor-callback-or-unresolved |
| `namespace_runner_peer` | `"namespace runner credentials do not match the launching user".to_owned` | [2455](../../src/runtime_backends.rs#L2455) | receiver-type-required |
| `authenticate_namespace_runner` | `namespace_runner_peer` | [2465](../../src/runtime_backends.rs#L2465) | [tools::runtime_backends::namespace_runner_peer](../../src/runtime_backends.rs#L2409) |
| `authenticate_namespace_runner` | `BackendFailure::Conflict` | [2467](../../src/runtime_backends.rs#L2467) | external-constructor-callback-or-unresolved |
| `authenticate_namespace_runner` | `"namespace runner is not a child of the verified launch".to_owned` | [2468](../../src/runtime_backends.rs#L2468) | receiver-type-required |
| `authenticate_namespace_runner` | `LinuxProcessIdentity::read(peer).ok_or_else` | [2471](../../src/runtime_backends.rs#L2471) | receiver-type-required |
| `authenticate_namespace_runner` | `LinuxProcessIdentity::read` | [2471](../../src/runtime_backends.rs#L2471), [2472](../../src/runtime_backends.rs#L2472) | [tools::runtime_backends::LinuxProcessIdentity::read](../../src/runtime_backends.rs#L2209) |
| `authenticate_namespace_runner` | `LinuxProcessIdentity::read(runner.parent).ok_or_else` | [2472](../../src/runtime_backends.rs#L2472) | receiver-type-required |
| `authenticate_namespace_runner` | `runner.same_namespace` | [2477](../../src/runtime_backends.rs#L2477) | receiver-type-required |
| `authenticate_namespace_runner` | `process_start_token(wrapper_pid).as_deref` | [2479](../../src/runtime_backends.rs#L2479) | receiver-type-required |
| `authenticate_namespace_runner` | `process_start_token` | [2479](../../src/runtime_backends.rs#L2479) | ambiguous-cfg-or-overload |
| `authenticate_namespace_runner` | `Some` | [2479](../../src/runtime_backends.rs#L2479) | external-constructor-callback-or-unresolved |
| `authenticate_namespace_runner` | `Err` | [2481](../../src/runtime_backends.rs#L2481) | external-constructor-callback-or-unresolved |
| `authenticate_namespace_runner` | `invalid` | [2481](../../src/runtime_backends.rs#L2481) | external-constructor-callback-or-unresolved |
| `authenticate_namespace_runner` | `wrapper_start.to_owned` | [2485](../../src/runtime_backends.rs#L2485) | receiver-type-required |
| `authenticate_namespace_runner` | `identity.stop_handle()?.ok_or_else` | [2491](../../src/runtime_backends.rs#L2491) | receiver-type-required |
| `authenticate_namespace_runner` | `identity.stop_handle` | [2491](../../src/runtime_backends.rs#L2491) | receiver-type-required |
| `authenticate_namespace_runner` | `Ok` | [2492](../../src/runtime_backends.rs#L2492) | external-constructor-callback-or-unresolved |
| `process_start_token` | `std::mem::MaybeUninit::<libc::proc_bsdinfo>::zeroed` | [2497](../../src/runtime_backends.rs#L2497) | external-constructor-callback-or-unresolved |
| `process_start_token` | `libc::proc_pidinfo` | [2501](../../src/runtime_backends.rs#L2501) | external-constructor-callback-or-unresolved |
| `process_start_token` | `i32::try_from(pid).ok` | [2502](../../src/runtime_backends.rs#L2502) | receiver-type-required |
| `process_start_token` | `i32::try_from` | [2502](../../src/runtime_backends.rs#L2502), [2506](../../src/runtime_backends.rs#L2506) | external-constructor-callback-or-unresolved |
| `process_start_token` | `info.as_mut_ptr().cast` | [2505](../../src/runtime_backends.rs#L2505) | receiver-type-required |
| `process_start_token` | `info.as_mut_ptr` | [2505](../../src/runtime_backends.rs#L2505) | receiver-type-required |
| `process_start_token` | `i32::try_from(std::mem::size_of::<libc::proc_bsdinfo>()).ok` | [2506](../../src/runtime_backends.rs#L2506) | receiver-type-required |
| `process_start_token` | `std::mem::size_of::<libc::proc_bsdinfo>` | [2506](../../src/runtime_backends.rs#L2506), [2509](../../src/runtime_backends.rs#L2509) | external-constructor-callback-or-unresolved |
| `process_start_token` | `usize::try_from(count).ok` | [2509](../../src/runtime_backends.rs#L2509) | receiver-type-required |
| `process_start_token` | `usize::try_from` | [2509](../../src/runtime_backends.rs#L2509) | external-constructor-callback-or-unresolved |
| `process_start_token` | `info.assume_init` | [2513](../../src/runtime_backends.rs#L2513) | receiver-type-required |
| `process_start_token` | `Some` | [2514](../../src/runtime_backends.rs#L2514) | external-constructor-callback-or-unresolved |
| `process_start_token` | `fs::read_to_string(format!("/proc/{pid}/stat")).ok` | [2522](../../src/runtime_backends.rs#L2522) | receiver-type-required |
| `process_start_token` | `fs::read_to_string` | [2522](../../src/runtime_backends.rs#L2522) | external-constructor-callback-or-unresolved |
| `process_start_token` | `stat.rfind` | [2523](../../src/runtime_backends.rs#L2523) | receiver-type-required |
| `process_start_token` | `stat[end + 2..]         .split_whitespace()         .nth(19)         .map` | [2524](../../src/runtime_backends.rs#L2524) | receiver-type-required |
| `process_start_token` | `stat[end + 2..]         .split_whitespace()         .nth` | [2524](../../src/runtime_backends.rs#L2524) | receiver-type-required |
| `process_start_token` | `stat[end + 2..]         .split_whitespace` | [2524](../../src/runtime_backends.rs#L2524) | receiver-type-required |
| `unix_millis` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err(&#124;error&#124; BackendFailure::Io(error.to_string()))?         .as_millis` | [2536](../../src/runtime_backends.rs#L2536) | receiver-type-required |
| `unix_millis` | `SystemTime::now()         .duration_since(UNIX_EPOCH)         .map_err` | [2536](../../src/runtime_backends.rs#L2536) | receiver-type-required |
| `unix_millis` | `SystemTime::now()         .duration_since` | [2536](../../src/runtime_backends.rs#L2536) | receiver-type-required |
| `unix_millis` | `SystemTime::now` | [2536](../../src/runtime_backends.rs#L2536) | external-constructor-callback-or-unresolved |
| `unix_millis` | `BackendFailure::Io` | [2538](../../src/runtime_backends.rs#L2538) | external-constructor-callback-or-unresolved |
| `unix_millis` | `error.to_string` | [2538](../../src/runtime_backends.rs#L2538) | receiver-type-required |
| `unix_millis` | `u64::try_from(millis).map_err` | [2540](../../src/runtime_backends.rs#L2540) | receiver-type-required |
| `unix_millis` | `u64::try_from` | [2540](../../src/runtime_backends.rs#L2540) | external-constructor-callback-or-unresolved |
| `unix_millis` | `BackendFailure::Limit` | [2540](../../src/runtime_backends.rs#L2540) | external-constructor-callback-or-unresolved |
| `unix_millis` | `"clock overflow".to_owned` | [2540](../../src/runtime_backends.rs#L2540) | receiver-type-required |
| `sync_directory` | `File::open(path)         .and_then(&#124;file&#124; file.sync_all())         .map_err` | [2544](../../src/runtime_backends.rs#L2544) | receiver-type-required |
| `sync_directory` | `File::open(path)         .and_then` | [2544](../../src/runtime_backends.rs#L2544) | receiver-type-required |
| `sync_directory` | `File::open` | [2544](../../src/runtime_backends.rs#L2544) | external-constructor-callback-or-unresolved |
| `sync_directory` | `file.sync_all` | [2545](../../src/runtime_backends.rs#L2545) | receiver-type-required |
| `io_failure` | `BackendFailure::Io` | [2550](../../src/runtime_backends.rs#L2550) | external-constructor-callback-or-unresolved |
| `io_failure` | `error.to_string` | [2550](../../src/runtime_backends.rs#L2550) | receiver-type-required |
| `store_failure` | `BackendFailure::Io` | [2554](../../src/runtime_backends.rs#L2554) | external-constructor-callback-or-unresolved |
| `store_failure` | `error.to_string` | [2554](../../src/runtime_backends.rs#L2554) | receiver-type-required |
| `mcp_only_launcher_refuses_command_execution` | `HelperJobLauncher::disabled(std::env::current_exe().unwrap()).unwrap` | [2565](../../src/runtime_backends.rs#L2565) | receiver-type-required |
| `mcp_only_launcher_refuses_command_execution` | `HelperJobLauncher::disabled` | [2565](../../src/runtime_backends.rs#L2565) | [tools::runtime_backends::HelperJobLauncher::disabled](../../src/runtime_backends.rs#L508) |
| `mcp_only_launcher_refuses_command_execution` | `std::env::current_exe().unwrap` | [2565](../../src/runtime_backends.rs#L2565) | receiver-type-required |
| `mcp_only_launcher_refuses_command_execution` | `std::env::current_exe` | [2565](../../src/runtime_backends.rs#L2565) | external-constructor-callback-or-unresolved |
| `mcp_only_launcher_refuses_command_execution` | `"/bin/true".to_owned` | [2567](../../src/runtime_backends.rs#L2567) | receiver-type-required |
| `mcp_only_launcher_refuses_command_execution` | `Vec::new` | [2568](../../src/runtime_backends.rs#L2568), [2570](../../src/runtime_backends.rs#L2570), [2576](../../src/runtime_backends.rs#L2576) | external-constructor-callback-or-unresolved |
| `mcp_only_launcher_refuses_command_execution` | `BTreeMap::new` | [2571](../../src/runtime_backends.rs#L2571) | external-constructor-callback-or-unresolved |
| `inherited_socket_reports_kernel_sender_identity` | `namespace_identity_channel().unwrap` | [2595](../../src/runtime_backends.rs#L2595) | receiver-type-required |
| `inherited_socket_reports_kernel_sender_identity` | `namespace_identity_channel` | [2595](../../src/runtime_backends.rs#L2595) | external-constructor-callback-or-unresolved |
| `inherited_socket_reports_kernel_sender_identity` | `runner.write_all(&[0xa1]).unwrap` | [2596](../../src/runtime_backends.rs#L2596) | receiver-type-required |
| `inherited_socket_reports_kernel_sender_identity` | `runner.write_all` | [2596](../../src/runtime_backends.rs#L2596) | receiver-type-required |
| `namespace_authentication_refuses_an_unrelated_sender` | `namespace_identity_channel().unwrap` | [2602](../../src/runtime_backends.rs#L2602) | receiver-type-required |
| `namespace_authentication_refuses_an_unrelated_sender` | `namespace_identity_channel` | [2602](../../src/runtime_backends.rs#L2602) | external-constructor-callback-or-unresolved |
| `namespace_authentication_refuses_an_unrelated_sender` | `runner.write_all(&[0xa1]).unwrap` | [2603](../../src/runtime_backends.rs#L2603) | receiver-type-required |
| `namespace_authentication_refuses_an_unrelated_sender` | `runner.write_all` | [2603](../../src/runtime_backends.rs#L2603) | receiver-type-required |
| `namespace_authentication_refuses_an_unrelated_sender` | `std::process::id` | [2604](../../src/runtime_backends.rs#L2604) | external-constructor-callback-or-unresolved |
| `runner_identity_requires_matching_pid_start_and_namespace` | `LinuxProcessIdentity::read(std::process::id()).unwrap` | [2613](../../src/runtime_backends.rs#L2613) | receiver-type-required |
| `runner_identity_requires_matching_pid_start_and_namespace` | `LinuxProcessIdentity::read` | [2613](../../src/runtime_backends.rs#L2613) | external-constructor-callback-or-unresolved |
| `runner_identity_requires_matching_pid_start_and_namespace` | `std::process::id` | [2613](../../src/runtime_backends.rs#L2613) | external-constructor-callback-or-unresolved |
| `runner_identity_requires_matching_pid_start_and_namespace` | `"irrelevant-to-runner-check".to_owned` | [2616](../../src/runtime_backends.rs#L2616) | receiver-type-required |
| `runner_identity_requires_matching_pid_start_and_namespace` | `current.clone` | [2617](../../src/runtime_backends.rs#L2617) | receiver-type-required |
| `runner_identity_requires_matching_pid_start_and_namespace` | `identity.runner.start.push` | [2621](../../src/runtime_backends.rs#L2621) | receiver-type-required |
| `runner_identity_requires_matching_pid_start_and_namespace` | `identity.runner.start.pop` | [2623](../../src/runtime_backends.rs#L2623) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `Command::new("/bin/sleep").arg("10").spawn().unwrap` | [2633](../../src/runtime_backends.rs#L2633) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `Command::new("/bin/sleep").arg("10").spawn` | [2633](../../src/runtime_backends.rs#L2633) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `Command::new("/bin/sleep").arg` | [2633](../../src/runtime_backends.rs#L2633) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `Command::new` | [2633](../../src/runtime_backends.rs#L2633) | external-constructor-callback-or-unresolved |
| `stale_namespace_identity_never_signals_a_live_process` | `LinuxProcessIdentity::read(child.id()).unwrap` | [2634](../../src/runtime_backends.rs#L2634) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `LinuxProcessIdentity::read` | [2634](../../src/runtime_backends.rs#L2634) | external-constructor-callback-or-unresolved |
| `stale_namespace_identity_never_signals_a_live_process` | `child.id` | [2634](../../src/runtime_backends.rs#L2634) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `std::process::id` | [2636](../../src/runtime_backends.rs#L2636) | external-constructor-callback-or-unresolved |
| `stale_namespace_identity_never_signals_a_live_process` | `"unused".to_owned` | [2637](../../src/runtime_backends.rs#L2637) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `current.clone` | [2638](../../src/runtime_backends.rs#L2638) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `identity.init.start.push` | [2641](../../src/runtime_backends.rs#L2641) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `identity.stop_namespace().unwrap` | [2642](../../src/runtime_backends.rs#L2642) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `identity.stop_namespace` | [2642](../../src/runtime_backends.rs#L2642) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.try_wait().unwrap().is_none` | [2643](../../src/runtime_backends.rs#L2643) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.try_wait().unwrap` | [2643](../../src/runtime_backends.rs#L2643) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.try_wait` | [2643](../../src/runtime_backends.rs#L2643) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.kill().unwrap` | [2644](../../src/runtime_backends.rs#L2644) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.kill` | [2644](../../src/runtime_backends.rs#L2644) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.wait().unwrap` | [2645](../../src/runtime_backends.rs#L2645) | receiver-type-required |
| `stale_namespace_identity_never_signals_a_live_process` | `child.wait` | [2645](../../src/runtime_backends.rs#L2645) | receiver-type-required |
| `forged_namespace_receipt_cannot_select_a_host_process` | `LinuxProcessIdentity::read(std::process::id()).unwrap` | [2651](../../src/runtime_backends.rs#L2651) | receiver-type-required |
| `forged_namespace_receipt_cannot_select_a_host_process` | `LinuxProcessIdentity::read` | [2651](../../src/runtime_backends.rs#L2651) | external-constructor-callback-or-unresolved |
| `forged_namespace_receipt_cannot_select_a_host_process` | `std::process::id` | [2651](../../src/runtime_backends.rs#L2651) | external-constructor-callback-or-unresolved |
| `forged_namespace_receipt_cannot_select_a_host_process` | `current.start.clone` | [2654](../../src/runtime_backends.rs#L2654) | receiver-type-required |
| `forged_namespace_receipt_cannot_select_a_host_process` | `current.clone` | [2655](../../src/runtime_backends.rs#L2655) | receiver-type-required |
| `test_record` | `id.to_owned` | [2667](../../src/runtime_backends.rs#L2667) | receiver-type-required |
| `test_record` | `"/bin/true".to_owned` | [2669](../../src/runtime_backends.rs#L2669) | receiver-type-required |
| `test_record` | `BTreeMap::new` | [2673](../../src/runtime_backends.rs#L2673) | external-constructor-callback-or-unresolved |
| `test_record` | `Some` | [2676](../../src/runtime_backends.rs#L2676) | external-constructor-callback-or-unresolved |
| `test_record` | `std::process::id` | [2676](../../src/runtime_backends.rs#L2676), [2677](../../src/runtime_backends.rs#L2677) | external-constructor-callback-or-unresolved |
| `test_record` | `process_start_token` | [2677](../../src/runtime_backends.rs#L2677) | external-constructor-callback-or-unresolved |
| `test_record` | `"stdout.tail".to_owned` | [2680](../../src/runtime_backends.rs#L2680) | receiver-type-required |
| `test_record` | `"stderr.tail".to_owned` | [2681](../../src/runtime_backends.rs#L2681) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `tempfile::tempdir().unwrap` | [2687](../../src/runtime_backends.rs#L2687) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `tempfile::tempdir` | [2687](../../src/runtime_backends.rs#L2687) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `JobBroker::new(directory.path(), 4096).unwrap` | [2688](../../src/runtime_backends.rs#L2688) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `JobBroker::new` | [2688](../../src/runtime_backends.rs#L2688) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `directory.path` | [2688](../../src/runtime_backends.rs#L2688) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `test_record` | [2689](../../src/runtime_backends.rs#L2689), [2708](../../src/runtime_backends.rs#L2708) | [tools::runtime_backends::namespace_job_identity_tests::test_record](../../src/runtime_backends.rs#L2664) |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `fs::create_dir(broker.job_directory("owned")).unwrap` | [2690](../../src/runtime_backends.rs#L2690) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `fs::create_dir` | [2690](../../src/runtime_backends.rs#L2690) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `broker.job_directory` | [2690](../../src/runtime_backends.rs#L2690), [2710](../../src/runtime_backends.rs#L2710) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `record.id.clone` | [2692](../../src/runtime_backends.rs#L2692) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `record.pid.unwrap` | [2693](../../src/runtime_backends.rs#L2693), [2703](../../src/runtime_backends.rs#L2703) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `record.process_start.clone().unwrap` | [2694](../../src/runtime_backends.rs#L2694) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `record.process_start.clone` | [2694](../../src/runtime_backends.rs#L2694) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `fs::write(             broker.launch_root.join("owned.json"),             serde_json::to_vec(&receipt).unwrap(),         )         .unwrap` | [2697](../../src/runtime_backends.rs#L2697) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `fs::write` | [2697](../../src/runtime_backends.rs#L2697), [2709](../../src/runtime_backends.rs#L2709) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `broker.launch_root.join` | [2698](../../src/runtime_backends.rs#L2698) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `serde_json::to_vec(&receipt).unwrap` | [2699](../../src/runtime_backends.rs#L2699) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `serde_json::to_vec` | [2699](../../src/runtime_backends.rs#L2699), [2711](../../src/runtime_backends.rs#L2711) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `broker.host_receipt(&record).unwrap` | [2702](../../src/runtime_backends.rs#L2702) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `broker.host_receipt` | [2702](../../src/runtime_backends.rs#L2702) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `Some` | [2703](../../src/runtime_backends.rs#L2703) | external-constructor-callback-or-unresolved |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `fs::write(             broker.job_directory("owned").join("job.json"),             serde_json::to_vec(&record).unwrap(),         )         .unwrap` | [2709](../../src/runtime_backends.rs#L2709) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `broker.job_directory("owned").join` | [2710](../../src/runtime_backends.rs#L2710) | receiver-type-required |
| `sandbox_written_pid_and_job_id_cannot_replace_host_authority` | `serde_json::to_vec(&record).unwrap` | [2711](../../src/runtime_backends.rs#L2711) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `tempfile::tempdir().unwrap` | [2732](../../src/runtime_backends.rs#L2732) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `tempfile::tempdir` | [2732](../../src/runtime_backends.rs#L2732) | external-constructor-callback-or-unresolved |
| `writable_root_cannot_cover_private_launch_receipts` | `JobBroker::new(directory.path(), 4096).unwrap` | [2733](../../src/runtime_backends.rs#L2733) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `JobBroker::new` | [2733](../../src/runtime_backends.rs#L2733) | external-constructor-callback-or-unresolved |
| `writable_root_cannot_cover_private_launch_receipts` | `directory.path` | [2733](../../src/runtime_backends.rs#L2733), [2734](../../src/runtime_backends.rs#L2734) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `fs::canonicalize(directory.path())             .unwrap()             .to_string_lossy()             .into_owned` | [2734](../../src/runtime_backends.rs#L2734) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `fs::canonicalize(directory.path())             .unwrap()             .to_string_lossy` | [2734](../../src/runtime_backends.rs#L2734) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `fs::canonicalize(directory.path())             .unwrap` | [2734](../../src/runtime_backends.rs#L2734) | receiver-type-required |
| `writable_root_cannot_cover_private_launch_receipts` | `fs::canonicalize` | [2734](../../src/runtime_backends.rs#L2734) | external-constructor-callback-or-unresolved |
