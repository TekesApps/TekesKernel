# tekes-supervisor::host_runtime

[Package atlas](index.md) · [Source](../../src/host_runtime.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::host_runtime::SOURCE_REVISION](../../src/host_runtime.rs#L37) | const_item | `pub` |  |
| [tekes-supervisor::host_runtime::is_source_revision](../../src/host_runtime.rs#L48) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::TERMINATE_REQUESTED](../../src/host_runtime.rs#L63) | static_item | `private` |  |
| [tekes-supervisor::host_runtime::endpoint_token_from_environment](../../src/host_runtime.rs#L65) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::decode_endpoint_token](../../src/host_runtime.rs#L73) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::ProductionRootLock](../../src/host_runtime.rs#L91) | struct_item | `pub` |  |
| [tekes-supervisor::host_runtime::ProductionRootLock::acquire](../../src/host_runtime.rs#L97) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::ProductionRootLock::path](../../src/host_runtime.rs#L129) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::ProductionRootLock::drop](../../src/host_runtime.rs#L135) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::validate_directory](../../src/host_runtime.rs#L141) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::validate_file_metadata](../../src/host_runtime.rs#L157) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::effective_uid](../../src/host_runtime.rs#L176) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::authority_root](../../src/host_runtime.rs#L181) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::prepare_storage](../../src/host_runtime.rs#L187) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::preflight_storage](../../src/host_runtime.rs#L239) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::require_production_apfs](../../src/host_runtime.rs#L244) | function_item | `private` | #[cfg(target_os = "macos")] |
| [tekes-supervisor::host_runtime::require_production_apfs](../../src/host_runtime.rs#L268) | function_item | `private` | #[cfg(not(target_os = "macos"))] |
| [tekes-supervisor::host_runtime::require_production_filesystem_name](../../src/host_runtime.rs#L273) | function_item | `private` | test; #[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))] |
| [tekes-supervisor::host_runtime::reject_cloud_managed_storage](../../src/host_runtime.rs#L283) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::is_icloud_mobile_documents_path](../../src/host_runtime.rs#L293) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L312) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::watch_shutdown](../../src/host_runtime.rs#L418) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::wait_for_launcher_shutdown](../../src/host_runtime.rs#L455) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::request_termination](../../src/host_runtime.rs#L460) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::install_termination_handler](../../src/host_runtime.rs#L464) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::duplicate_fd](../../src/host_runtime.rs#L481) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L495) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::DaemonError](../../src/host_runtime.rs#L523) | struct_item | `pub` |  |
| [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::DaemonError::protocol](../../src/host_runtime.rs#L536) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::store](../../src/host_runtime.rs#L544) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::already_running](../../src/host_runtime.rs#L563) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::DaemonError::invalid_install](../../src/host_runtime.rs#L570) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::invalid_install_reason](../../src/host_runtime.rs#L574) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::invalid_config](../../src/host_runtime.rs#L578) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::corrupt](../../src/host_runtime.rs#L582) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::required_broker](../../src/host_runtime.rs#L586) | function_item | `pub(crate)` |  |
| [tekes-supervisor::host_runtime::DaemonError::credential](../../src/host_runtime.rs#L590) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::DaemonError::bootstrap_code](../../src/host_runtime.rs#L595) | function_item | `pub` |  |
| [tekes-supervisor::host_runtime::DaemonError::from](../../src/host_runtime.rs#L601) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::DaemonError::from](../../src/host_runtime.rs#L611) | function_item | `private` |  |
| [tekes-supervisor::host_runtime::tests::macos_storage_accepts_apfs_and_rejects_hfs](../../src/host_runtime.rs#L629) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::host_runtime::tests::production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias](../../src/host_runtime.rs#L638) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::host_runtime::tests::production_timestamp_is_event_millisecond_utc](../../src/host_runtime.rs#L664) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `CString` | `std::ffi::CString` | `private` |
| `OsStr` | `std::ffi::OsStr` | `private` |
| `fs` | `std::fs` | `private` |
| `File` | `std::fs::File` | `private` |
| `OpenOptions` | `std::fs::OpenOptions` | `private` |
| `io` | `std::io` | `private` |
| `Read` | `std::io::Read` | `private` |
| `AsRawFd` | `std::os::fd::AsRawFd` | `private` |
| `FromRawFd` | `std::os::fd::FromRawFd` | `private` |
| `RawFd` | `std::os::fd::RawFd` | `private` |
| `OsStrExt` | `std::os::unix::ffi::OsStrExt` | `private` |
| `DirBuilderExt` | `std::os::unix::fs::DirBuilderExt` | `private` |
| `MetadataExt` | `std::os::unix::fs::MetadataExt` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `SessionHostDescription` | `endpoint::SessionHostDescription` | `private` |
| `ConfigRepository` | `profile::ConfigRepository` | `private` |
| `InstructionResolver` | `profile::InstructionResolver` | `private` |
| `ResourceCatalog` | `profile::ResourceCatalog` | `private` |
| `StoreError` | `store::StoreError` | `private` |
| `ThreadStore` | `store::ThreadStore` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `ClientAdminRoutes` | `crate::client_admin::ClientAdminRoutes` | `private` |
| `ProductionClientExtensions` | `crate::client_extensions::ProductionClientExtensions` | `private` |
| `ProductionCarrierError` | `crate::endpoint_carrier::ProductionCarrierError` | `private` |
| `CompositeProductionEndpointRoutes` | `crate::endpoint_host::CompositeProductionEndpointRoutes` | `private` |
| `EndpointClock` | `crate::endpoint_host::EndpointClock` | `private` |
| `ProductionEndpointHost` | `crate::endpoint_host::ProductionEndpointHost` | `private` |
| `ProviderReadinessAuthority` | `crate::endpoint_host::ProviderReadinessAuthority` | `private` |
| `QueueTransactionAuthority` | `crate::endpoint_host::QueueTransactionAuthority` | `private` |
| `SessionDeliveryAuthority` | `crate::endpoint_host::SessionDeliveryAuthority` | `private` |
| `SessionInputAdmissionAuthority` | `crate::endpoint_host::SessionInputAdmissionAuthority` | `private` |
| `ProductionProcessHost` | `crate::process_host::ProductionProcessHost` | `private` |
| `EndpointCommandInputAuthority` | `crate::resource_capability::EndpointCommandInputAuthority` | `private` |
| `fs` | `std::fs` | `private` |
| `symlink` | `std::os::unix::fs::symlink` | `private` |
| `reject_cloud_managed_storage` | `super::reject_cloud_managed_storage` | `private` |
| `require_production_filesystem_name` | `super::require_production_filesystem_name` | `private` |
| `system_timestamp` | `super::system_timestamp` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::host_runtime::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 46 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::open"]
  n1["profile::instruction::InstructionResolver::new"]
  n2["profile::instruction::InstructionResolver::new_scoped"]
  n3["profile::resources::ResourceCatalog::from_snapshot"]
  n4["store::folder::ThreadStore::open"]
  n5["store::platform::probe_local_filesystem"]
  n6["tekes-supervisor::boot_trace::phase"]
  n7["tekes-supervisor::client_admin::ClientAdminRoutes::new"]
  n8["tekes-supervisor::client_extensions::ProductionClientExtensions::open"]
  n9["tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new"]
  n10["tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose"]
  n11["tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission"]
  n12["tekes-supervisor::host_runtime::ProductionRootLock::path"]
  n13["tekes-supervisor::host_runtime::ProductionRootLock::drop"]
  n14["tekes-supervisor::host_runtime::validate_directory"]
  n15["tekes-supervisor::host_runtime::validate_file_metadata"]
  n16["tekes-supervisor::host_runtime::effective_uid"]
  n17["tekes-supervisor::host_runtime::authority_root"]
  n18["tekes-supervisor::host_runtime::prepare_storage"]
  n19["tekes-supervisor::host_runtime::preflight_storage"]
  n20["tekes-supervisor::host_runtime::require_production_apfs"]
  n21["tekes-supervisor::host_runtime::require_production_apfs"]
  n22["tekes-supervisor::host_runtime::require_production_filesystem_name"]
  n23["tekes-supervisor::host_runtime::reject_cloud_managed_storage"]
  n24["tekes-supervisor::host_runtime::is_icloud_mobile_documents_path"]
  n25["tekes-supervisor::host_runtime::assemble_application_endpoint_host"]
  n26["tekes-supervisor::host_runtime::watch_shutdown"]
  n27["tekes-supervisor::host_runtime::wait_for_launcher_shutdown"]
  n28["tekes-supervisor::host_runtime::request_termination"]
  n29["tekes-supervisor::host_runtime::duplicate_fd"]
  n30["tekes-supervisor::host_runtime::is_source_revision"]
  n31["tekes-supervisor::host_runtime::DaemonError::io"]
  n32["tekes-supervisor::host_runtime::DaemonError::store"]
  n33["tekes-supervisor::host_runtime::DaemonError::already_running"]
  n34["tekes-supervisor::host_runtime::DaemonError::invalid_install"]
  n35["tekes-supervisor::host_runtime::DaemonError::invalid_install_reason"]
  n36["tekes-supervisor::host_runtime::DaemonError::invalid_config"]
  n37["tekes-supervisor::host_runtime::DaemonError::corrupt"]
  n38["tekes-supervisor::host_runtime::DaemonError::credential"]
  n39["tekes-supervisor::host_runtime::endpoint_token_from_environment"]
  n40["tekes-supervisor::host_runtime::decode_endpoint_token"]
  n41["tekes-supervisor::host_runtime::ProductionRootLock::acquire"]
  n42["tekes-supervisor::resource_capability::EndpointCommandInputAuthority::new"]
  n43["tekes-supervisor::workspace_routes::WorkspaceRoutes::open"]
  n14 --> n16
  n14 --> n34
  n14 --> n35
  n15 --> n16
  n15 --> n34
  n15 --> n35
  n17 --> n35
  n18 --> n0
  n18 --> n4
  n18 --> n5
  n18 --> n6
  n18 --> n14
  n18 --> n17
  n18 --> n23
  n18 --> n31
  n18 --> n36
  n19 --> n18
  n20 --> n22
  n20 --> n31
  n20 --> n32
  n23 --> n24
  n23 --> n32
  n25 --> n0
  n25 --> n1
  n25 --> n2
  n25 --> n3
  n25 --> n7
  n25 --> n8
  n25 --> n9
  n25 --> n10
  n25 --> n11
  n25 --> n36
  n25 --> n37
  n25 --> n42
  n25 --> n43
  n26 --> n31
  n27 --> n26
  n27 --> n29
  n39 --> n38
  n39 --> n40
  n40 --> n38
  n41 --> n14
  n41 --> n15
  n41 --> n31
  n41 --> n33
  n41 --> n34
```

</details>

<details><summary>Functions 21–37: 12 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::host_runtime::install_termination_handler"]
  n1["tekes-supervisor::host_runtime::duplicate_fd"]
  n2["tekes-supervisor::host_runtime::system_timestamp"]
  n3["tekes-supervisor::host_runtime::DaemonError::new"]
  n4["tekes-supervisor::host_runtime::DaemonError::protocol"]
  n5["tekes-supervisor::host_runtime::DaemonError::io"]
  n6["tekes-supervisor::host_runtime::DaemonError::store"]
  n7["tekes-supervisor::host_runtime::DaemonError::already_running"]
  n8["tekes-supervisor::host_runtime::DaemonError::invalid_install"]
  n9["tekes-supervisor::host_runtime::DaemonError::invalid_install_reason"]
  n10["tekes-supervisor::host_runtime::DaemonError::invalid_config"]
  n11["tekes-supervisor::host_runtime::DaemonError::corrupt"]
  n12["tekes-supervisor::host_runtime::DaemonError::required_broker"]
  n13["tekes-supervisor::host_runtime::DaemonError::credential"]
  n14["tekes-supervisor::host_runtime::DaemonError::bootstrap_code"]
  n15["tekes-supervisor::host_runtime::DaemonError::from"]
  n16["tekes-supervisor::host_runtime::DaemonError::from"]
  n0 --> n5
  n4 --> n3
  n5 --> n3
  n6 --> n3
  n6 --> n5
  n7 --> n3
  n8 --> n3
  n9 --> n3
  n10 --> n3
  n11 --> n3
  n12 --> n3
  n13 --> n3
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `SOURCE_REVISION` | `Some` | [43](../../src/host_runtime.rs#L43) | external-constructor-callback-or-unresolved |
| `is_source_revision` | `value.as_bytes` | [49](../../src/host_runtime.rs#L49) | receiver-type-required |
| `is_source_revision` | `bytes.len` | [50](../../src/host_runtime.rs#L50), [54](../../src/host_runtime.rs#L54) | receiver-type-required |
| `TERMINATE_REQUESTED` | `AtomicBool::new` | [63](../../src/host_runtime.rs#L63) | external-constructor-callback-or-unresolved |
| `endpoint_token_from_environment` | `zeroize::Zeroizing::new` | [66](../../src/host_runtime.rs#L66) | external-constructor-callback-or-unresolved |
| `endpoint_token_from_environment` | `std::env::var("TEKES_KERNEL_ENDPOINT_TOKEN")             .map_err` | [67](../../src/host_runtime.rs#L67) | receiver-type-required |
| `endpoint_token_from_environment` | `std::env::var` | [67](../../src/host_runtime.rs#L67) | external-constructor-callback-or-unresolved |
| `endpoint_token_from_environment` | `DaemonError::credential` | [68](../../src/host_runtime.rs#L68) | [tekes-supervisor::host_runtime::DaemonError::credential](../../src/host_runtime.rs#L590) |
| `endpoint_token_from_environment` | `decode_endpoint_token` | [70](../../src/host_runtime.rs#L70) | [tekes-supervisor::host_runtime::decode_endpoint_token](../../src/host_runtime.rs#L73) |
| `decode_endpoint_token` | `value.len` | [74](../../src/host_runtime.rs#L74) | receiver-type-required |
| `decode_endpoint_token` | `value.bytes().all` | [74](../../src/host_runtime.rs#L74) | receiver-type-required |
| `decode_endpoint_token` | `value.bytes` | [74](../../src/host_runtime.rs#L74) | receiver-type-required |
| `decode_endpoint_token` | `byte.is_ascii_hexdigit` | [74](../../src/host_runtime.rs#L74) | receiver-type-required |
| `decode_endpoint_token` | `Err` | [75](../../src/host_runtime.rs#L75) | external-constructor-callback-or-unresolved |
| `decode_endpoint_token` | `DaemonError::credential` | [75](../../src/host_runtime.rs#L75), [83](../../src/host_runtime.rs#L83), [86](../../src/host_runtime.rs#L86) | [tekes-supervisor::host_runtime::DaemonError::credential](../../src/host_runtime.rs#L590) |
| `decode_endpoint_token` | `value.as_bytes().chunks_exact(2).enumerate` | [80](../../src/host_runtime.rs#L80) | receiver-type-required |
| `decode_endpoint_token` | `value.as_bytes().chunks_exact` | [80](../../src/host_runtime.rs#L80) | receiver-type-required |
| `decode_endpoint_token` | `value.as_bytes` | [80](../../src/host_runtime.rs#L80) | receiver-type-required |
| `decode_endpoint_token` | `u8::from_str_radix(             std::str::from_utf8(pair)                 .map_err(&#124;_&#124; DaemonError::credential("invalid-endpoint-environment-token"))?,             16,         )         .map_err` | [81](../../src/host_runtime.rs#L81) | receiver-type-required |
| `decode_endpoint_token` | `u8::from_str_radix` | [81](../../src/host_runtime.rs#L81) | external-constructor-callback-or-unresolved |
| `decode_endpoint_token` | `std::str::from_utf8(pair)                 .map_err` | [82](../../src/host_runtime.rs#L82) | receiver-type-required |
| `decode_endpoint_token` | `std::str::from_utf8` | [82](../../src/host_runtime.rs#L82) | external-constructor-callback-or-unresolved |
| `decode_endpoint_token` | `Ok` | [88](../../src/host_runtime.rs#L88) | external-constructor-callback-or-unresolved |
| `acquire` | `validate_directory` | [98](../../src/host_runtime.rs#L98) | [tekes-supervisor::host_runtime::validate_directory](../../src/host_runtime.rs#L141) |
| `acquire` | `storage_root.join` | [99](../../src/host_runtime.rs#L99) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open(&path)             .map_err` | [100](../../src/host_runtime.rs#L100) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)             .open` | [100](../../src/host_runtime.rs#L100) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write(true)             .custom_flags` | [100](../../src/host_runtime.rs#L100) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read(true)             .write` | [100](../../src/host_runtime.rs#L100) | receiver-type-required |
| `acquire` | `OpenOptions::new()             .read` | [100](../../src/host_runtime.rs#L100) | receiver-type-required |
| `acquire` | `OpenOptions::new` | [100](../../src/host_runtime.rs#L100) | external-constructor-callback-or-unresolved |
| `acquire` | `DaemonError::invalid_install` | [105](../../src/host_runtime.rs#L105) | [tekes-supervisor::host_runtime::DaemonError::invalid_install](../../src/host_runtime.rs#L570) |
| `acquire` | `path.clone` | [105](../../src/host_runtime.rs#L105) | receiver-type-required |
| `acquire` | `validate_file_metadata` | [106](../../src/host_runtime.rs#L106), [124](../../src/host_runtime.rs#L124) | [tekes-supervisor::host_runtime::validate_file_metadata](../../src/host_runtime.rs#L157) |
| `acquire` | `libc::flock` | [109](../../src/host_runtime.rs#L109) | external-constructor-callback-or-unresolved |
| `acquire` | `file.as_raw_fd` | [109](../../src/host_runtime.rs#L109) | receiver-type-required |
| `acquire` | `io::Error::last_os_error` | [112](../../src/host_runtime.rs#L112) | external-constructor-callback-or-unresolved |
| `acquire` | `error.kind` | [113](../../src/host_runtime.rs#L113) | receiver-type-required |
| `acquire` | `error                 .raw_os_error()                 .is_some_and` | [116](../../src/host_runtime.rs#L116) | receiver-type-required |
| `acquire` | `error                 .raw_os_error` | [116](../../src/host_runtime.rs#L116) | receiver-type-required |
| `acquire` | `Err` | [120](../../src/host_runtime.rs#L120), [122](../../src/host_runtime.rs#L122) | external-constructor-callback-or-unresolved |
| `acquire` | `DaemonError::already_running` | [120](../../src/host_runtime.rs#L120) | [tekes-supervisor::host_runtime::DaemonError::already_running](../../src/host_runtime.rs#L563) |
| `acquire` | `DaemonError::io` | [122](../../src/host_runtime.rs#L122) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `acquire` | `Ok` | [125](../../src/host_runtime.rs#L125) | external-constructor-callback-or-unresolved |
| `drop` | `libc::flock` | [137](../../src/host_runtime.rs#L137) | external-constructor-callback-or-unresolved |
| `drop` | `self.file.as_raw_fd` | [137](../../src/host_runtime.rs#L137) | receiver-type-required |
| `validate_directory` | `fs::symlink_metadata(path)         .map_err` | [142](../../src/host_runtime.rs#L142) | receiver-type-required |
| `validate_directory` | `fs::symlink_metadata` | [142](../../src/host_runtime.rs#L142) | external-constructor-callback-or-unresolved |
| `validate_directory` | `DaemonError::invalid_install` | [143](../../src/host_runtime.rs#L143) | [tekes-supervisor::host_runtime::DaemonError::invalid_install](../../src/host_runtime.rs#L570) |
| `validate_directory` | `path.to_path_buf` | [143](../../src/host_runtime.rs#L143) | receiver-type-required |
| `validate_directory` | `metadata.file_type().is_symlink` | [144](../../src/host_runtime.rs#L144) | receiver-type-required |
| `validate_directory` | `metadata.file_type` | [144](../../src/host_runtime.rs#L144), [145](../../src/host_runtime.rs#L145) | receiver-type-required |
| `validate_directory` | `metadata.file_type().is_dir` | [145](../../src/host_runtime.rs#L145) | receiver-type-required |
| `validate_directory` | `metadata.uid` | [146](../../src/host_runtime.rs#L146) | receiver-type-required |
| `validate_directory` | `effective_uid` | [146](../../src/host_runtime.rs#L146) | [tekes-supervisor::host_runtime::effective_uid](../../src/host_runtime.rs#L176) |
| `validate_directory` | `metadata.mode` | [147](../../src/host_runtime.rs#L147) | receiver-type-required |
| `validate_directory` | `Err` | [149](../../src/host_runtime.rs#L149) | external-constructor-callback-or-unresolved |
| `validate_directory` | `DaemonError::invalid_install_reason` | [149](../../src/host_runtime.rs#L149) | [tekes-supervisor::host_runtime::DaemonError::invalid_install_reason](../../src/host_runtime.rs#L574) |
| `validate_directory` | `Ok` | [154](../../src/host_runtime.rs#L154) | external-constructor-callback-or-unresolved |
| `validate_file_metadata` | `file.metadata().map_err` | [158](../../src/host_runtime.rs#L158) | receiver-type-required |
| `validate_file_metadata` | `file.metadata` | [158](../../src/host_runtime.rs#L158) | receiver-type-required |
| `validate_file_metadata` | `fs::symlink_metadata(path)         .map_err` | [159](../../src/host_runtime.rs#L159) | receiver-type-required |
| `validate_file_metadata` | `fs::symlink_metadata` | [159](../../src/host_runtime.rs#L159) | external-constructor-callback-or-unresolved |
| `validate_file_metadata` | `DaemonError::invalid_install` | [160](../../src/host_runtime.rs#L160) | [tekes-supervisor::host_runtime::DaemonError::invalid_install](../../src/host_runtime.rs#L570) |
| `validate_file_metadata` | `path.to_path_buf` | [160](../../src/host_runtime.rs#L160) | receiver-type-required |
| `validate_file_metadata` | `pathname.file_type().is_symlink` | [161](../../src/host_runtime.rs#L161) | receiver-type-required |
| `validate_file_metadata` | `pathname.file_type` | [161](../../src/host_runtime.rs#L161), [162](../../src/host_runtime.rs#L162) | receiver-type-required |
| `validate_file_metadata` | `pathname.file_type().is_file` | [162](../../src/host_runtime.rs#L162) | receiver-type-required |
| `validate_file_metadata` | `descriptor.dev` | [163](../../src/host_runtime.rs#L163) | receiver-type-required |
| `validate_file_metadata` | `pathname.dev` | [163](../../src/host_runtime.rs#L163) | receiver-type-required |
| `validate_file_metadata` | `descriptor.ino` | [164](../../src/host_runtime.rs#L164) | receiver-type-required |
| `validate_file_metadata` | `pathname.ino` | [164](../../src/host_runtime.rs#L164) | receiver-type-required |
| `validate_file_metadata` | `descriptor.uid` | [165](../../src/host_runtime.rs#L165) | receiver-type-required |
| `validate_file_metadata` | `effective_uid` | [165](../../src/host_runtime.rs#L165) | [tekes-supervisor::host_runtime::effective_uid](../../src/host_runtime.rs#L176) |
| `validate_file_metadata` | `descriptor.mode` | [166](../../src/host_runtime.rs#L166) | receiver-type-required |
| `validate_file_metadata` | `Err` | [168](../../src/host_runtime.rs#L168) | external-constructor-callback-or-unresolved |
| `validate_file_metadata` | `DaemonError::invalid_install_reason` | [168](../../src/host_runtime.rs#L168) | [tekes-supervisor::host_runtime::DaemonError::invalid_install_reason](../../src/host_runtime.rs#L574) |
| `validate_file_metadata` | `Ok` | [173](../../src/host_runtime.rs#L173) | external-constructor-callback-or-unresolved |
| `effective_uid` | `libc::geteuid` | [178](../../src/host_runtime.rs#L178) | external-constructor-callback-or-unresolved |
| `authority_root` | `storage_root.parent().ok_or_else` | [182](../../src/host_runtime.rs#L182) | receiver-type-required |
| `authority_root` | `storage_root.parent` | [182](../../src/host_runtime.rs#L182) | receiver-type-required |
| `authority_root` | `DaemonError::invalid_install_reason` | [183](../../src/host_runtime.rs#L183) | [tekes-supervisor::host_runtime::DaemonError::invalid_install_reason](../../src/host_runtime.rs#L574) |
| `prepare_storage` | `validate_directory` | [188](../../src/host_runtime.rs#L188), [190](../../src/host_runtime.rs#L190), [210](../../src/host_runtime.rs#L210), [215](../../src/host_runtime.rs#L215) | [tekes-supervisor::host_runtime::validate_directory](../../src/host_runtime.rs#L141) |
| `prepare_storage` | `authority_root` | [189](../../src/host_runtime.rs#L189) | [tekes-supervisor::host_runtime::authority_root](../../src/host_runtime.rs#L181) |
| `prepare_storage` | `reject_cloud_managed_storage` | [191](../../src/host_runtime.rs#L191) | [tekes-supervisor::host_runtime::reject_cloud_managed_storage](../../src/host_runtime.rs#L283) |
| `prepare_storage` | `require_production_apfs` | [192](../../src/host_runtime.rs#L192) | ambiguous-cfg-or-overload |
| `prepare_storage` | `root.join` | [208](../../src/host_runtime.rs#L208) | receiver-type-required |
| `prepare_storage` | `fs::symlink_metadata` | [209](../../src/host_runtime.rs#L209) | external-constructor-callback-or-unresolved |
| `prepare_storage` | `error.kind` | [211](../../src/host_runtime.rs#L211) | receiver-type-required |
| `prepare_storage` | `fs::DirBuilder::new` | [212](../../src/host_runtime.rs#L212) | external-constructor-callback-or-unresolved |
| `prepare_storage` | `builder.mode` | [213](../../src/host_runtime.rs#L213) | receiver-type-required |
| `prepare_storage` | `builder.create(&path).map_err` | [214](../../src/host_runtime.rs#L214) | receiver-type-required |
| `prepare_storage` | `builder.create` | [214](../../src/host_runtime.rs#L214) | receiver-type-required |
| `prepare_storage` | `Err` | [217](../../src/host_runtime.rs#L217) | external-constructor-callback-or-unresolved |
| `prepare_storage` | `DaemonError::io` | [217](../../src/host_runtime.rs#L217) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `prepare_storage` | `store::probe_local_filesystem(root).map_err` | [220](../../src/host_runtime.rs#L220) | receiver-type-required |
| `prepare_storage` | `store::probe_local_filesystem` | [220](../../src/host_runtime.rs#L220) | [store::platform::probe_local_filesystem](../../../store/src/platform.rs#L231) |
| `prepare_storage` | `ThreadStore::open(root).map_err` | [221](../../src/host_runtime.rs#L221) | receiver-type-required |
| `prepare_storage` | `ThreadStore::open` | [221](../../src/host_runtime.rs#L221) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `prepare_storage` | `store         .recover_rewrites_for_startup()         .map_err` | [222](../../src/host_runtime.rs#L222) | receiver-type-required |
| `prepare_storage` | `store         .recover_rewrites_for_startup` | [222](../../src/host_runtime.rs#L222) | receiver-type-required |
| `prepare_storage` | `store.gc_rewrite_debris().map_err` | [225](../../src/host_runtime.rs#L225) | receiver-type-required |
| `prepare_storage` | `store.gc_rewrite_debris` | [225](../../src/host_runtime.rs#L225) | receiver-type-required |
| `prepare_storage` | `crate::boot_trace::phase` | [226](../../src/host_runtime.rs#L226) | [tekes-supervisor::boot_trace::phase](../../src/boot_trace.rs#L60) |
| `prepare_storage` | `profile::ConfigRepository::open(root)         .map_err` | [231](../../src/host_runtime.rs#L231) | receiver-type-required |
| `prepare_storage` | `profile::ConfigRepository::open` | [231](../../src/host_runtime.rs#L231) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `prepare_storage` | `DaemonError::invalid_config` | [232](../../src/host_runtime.rs#L232) | [tekes-supervisor::host_runtime::DaemonError::invalid_config](../../src/host_runtime.rs#L578) |
| `prepare_storage` | `error.to_string` | [232](../../src/host_runtime.rs#L232) | receiver-type-required |
| `prepare_storage` | `Ok` | [233](../../src/host_runtime.rs#L233) | external-constructor-callback-or-unresolved |
| `preflight_storage` | `prepare_storage(storage_root).map` | [240](../../src/host_runtime.rs#L240) | receiver-type-required |
| `preflight_storage` | `prepare_storage` | [240](../../src/host_runtime.rs#L240) | [tekes-supervisor::host_runtime::prepare_storage](../../src/host_runtime.rs#L187) |
| `require_production_apfs` | `CString::new(root.as_os_str().as_bytes()).map_err` | [245](../../src/host_runtime.rs#L245) | receiver-type-required |
| `require_production_apfs` | `CString::new` | [245](../../src/host_runtime.rs#L245) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `root.as_os_str().as_bytes` | [245](../../src/host_runtime.rs#L245) | receiver-type-required |
| `require_production_apfs` | `root.as_os_str` | [245](../../src/host_runtime.rs#L245) | receiver-type-required |
| `require_production_apfs` | `DaemonError::store` | [246](../../src/host_runtime.rs#L246) | [tekes-supervisor::host_runtime::DaemonError::store](../../src/host_runtime.rs#L544) |
| `require_production_apfs` | `StoreError::UnsupportedFilesystem` | [246](../../src/host_runtime.rs#L246) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `"storage path contains NUL".to_owned` | [247](../../src/host_runtime.rs#L247) | receiver-type-required |
| `require_production_apfs` | `std::mem::zeroed::<libc::statfs>` | [252](../../src/host_runtime.rs#L252) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `libc::statfs` | [254](../../src/host_runtime.rs#L254) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `path.as_ptr` | [254](../../src/host_runtime.rs#L254) | receiver-type-required |
| `require_production_apfs` | `Err` | [255](../../src/host_runtime.rs#L255) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `DaemonError::io` | [255](../../src/host_runtime.rs#L255) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `require_production_apfs` | `io::Error::last_os_error` | [255](../../src/host_runtime.rs#L255) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `stat         .f_fstypename         .iter()         .map(&#124;value&#124; *value as u8)         .take_while(&#124;value&#124; *value != 0)         .collect::<Vec<_>>` | [257](../../src/host_runtime.rs#L257) | receiver-type-required |
| `require_production_apfs` | `stat         .f_fstypename         .iter()         .map(&#124;value&#124; *value as u8)         .take_while` | [257](../../src/host_runtime.rs#L257) | receiver-type-required |
| `require_production_apfs` | `stat         .f_fstypename         .iter()         .map` | [257](../../src/host_runtime.rs#L257) | receiver-type-required |
| `require_production_apfs` | `stat         .f_fstypename         .iter` | [257](../../src/host_runtime.rs#L257) | receiver-type-required |
| `require_production_apfs` | `require_production_filesystem_name` | [263](../../src/host_runtime.rs#L263) | [tekes-supervisor::host_runtime::require_production_filesystem_name](../../src/host_runtime.rs#L273) |
| `require_production_apfs` | `String::from_utf8_lossy(&name).to_ascii_lowercase` | [263](../../src/host_runtime.rs#L263) | receiver-type-required |
| `require_production_apfs` | `String::from_utf8_lossy` | [263](../../src/host_runtime.rs#L263) | external-constructor-callback-or-unresolved |
| `require_production_apfs` | `Ok` | [269](../../src/host_runtime.rs#L269) | external-constructor-callback-or-unresolved |
| `require_production_filesystem_name` | `Ok` | [275](../../src/host_runtime.rs#L275) | external-constructor-callback-or-unresolved |
| `require_production_filesystem_name` | `Err` | [277](../../src/host_runtime.rs#L277) | external-constructor-callback-or-unresolved |
| `require_production_filesystem_name` | `DaemonError::store` | [277](../../src/host_runtime.rs#L277) | [tekes-supervisor::host_runtime::DaemonError::store](../../src/host_runtime.rs#L544) |
| `require_production_filesystem_name` | `StoreError::UnsupportedFilesystem` | [277](../../src/host_runtime.rs#L277) | external-constructor-callback-or-unresolved |
| `reject_cloud_managed_storage` | `fs::canonicalize(storage_root).map_err` | [284](../../src/host_runtime.rs#L284) | receiver-type-required |
| `reject_cloud_managed_storage` | `fs::canonicalize` | [284](../../src/host_runtime.rs#L284) | external-constructor-callback-or-unresolved |
| `reject_cloud_managed_storage` | `is_icloud_mobile_documents_path` | [285](../../src/host_runtime.rs#L285) | [tekes-supervisor::host_runtime::is_icloud_mobile_documents_path](../../src/host_runtime.rs#L293) |
| `reject_cloud_managed_storage` | `Err` | [286](../../src/host_runtime.rs#L286) | external-constructor-callback-or-unresolved |
| `reject_cloud_managed_storage` | `DaemonError::store` | [286](../../src/host_runtime.rs#L286) | [tekes-supervisor::host_runtime::DaemonError::store](../../src/host_runtime.rs#L544) |
| `reject_cloud_managed_storage` | `StoreError::UnsupportedFilesystem` | [286](../../src/host_runtime.rs#L286) | external-constructor-callback-or-unresolved |
| `reject_cloud_managed_storage` | `"production storage is inside Library/Mobile Documents".to_owned` | [287](../../src/host_runtime.rs#L287) | receiver-type-required |
| `reject_cloud_managed_storage` | `Ok` | [290](../../src/host_runtime.rs#L290) | external-constructor-callback-or-unresolved |
| `is_icloud_mobile_documents_path` | `path.components` | [295](../../src/host_runtime.rs#L295) | receiver-type-required |
| `is_icloud_mobile_documents_path` | `OsStr::new` | [300](../../src/host_runtime.rs#L300), [303](../../src/host_runtime.rs#L303) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `ConfigRepository::open(authority_root)         .map_err` | [319](../../src/host_runtime.rs#L319) | receiver-type-required |
| `assemble_application_endpoint_host` | `ConfigRepository::open` | [319](../../src/host_runtime.rs#L319) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `assemble_application_endpoint_host` | `DaemonError::invalid_config` | [320](../../src/host_runtime.rs#L320), [323](../../src/host_runtime.rs#L323), [325](../../src/host_runtime.rs#L325), [328](../../src/host_runtime.rs#L328), [352](../../src/host_runtime.rs#L352), [365](../../src/host_runtime.rs#L365), [368](../../src/host_runtime.rs#L368), [392](../../src/host_runtime.rs#L392), [400](../../src/host_runtime.rs#L400) | [tekes-supervisor::host_runtime::DaemonError::invalid_config](../../src/host_runtime.rs#L578) |
| `assemble_application_endpoint_host` | `error.to_string` | [320](../../src/host_runtime.rs#L320), [323](../../src/host_runtime.rs#L323), [325](../../src/host_runtime.rs#L325), [328](../../src/host_runtime.rs#L328), [352](../../src/host_runtime.rs#L352), [365](../../src/host_runtime.rs#L365), [368](../../src/host_runtime.rs#L368), [383](../../src/host_runtime.rs#L383), [404](../../src/host_runtime.rs#L404), [415](../../src/host_runtime.rs#L415) | receiver-type-required |
| `assemble_application_endpoint_host` | `InstructionResolver::new(user_agent_dir, std::iter::empty::<&Path>())         .capture()         .map_err` | [321](../../src/host_runtime.rs#L321) | receiver-type-required |
| `assemble_application_endpoint_host` | `InstructionResolver::new(user_agent_dir, std::iter::empty::<&Path>())         .capture` | [321](../../src/host_runtime.rs#L321) | receiver-type-required |
| `assemble_application_endpoint_host` | `InstructionResolver::new` | [321](../../src/host_runtime.rs#L321) | [profile::instruction::InstructionResolver::new](../../../profile/src/instruction.rs#L285) |
| `assemble_application_endpoint_host` | `std::iter::empty::<&Path>` | [321](../../src/host_runtime.rs#L321) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `ResourceCatalog::from_snapshot(&instruction)         .map_err` | [324](../../src/host_runtime.rs#L324) | receiver-type-required |
| `assemble_application_endpoint_host` | `ResourceCatalog::from_snapshot` | [324](../../src/host_runtime.rs#L324), [367](../../src/host_runtime.rs#L367) | [profile::resources::ResourceCatalog::from_snapshot](../../../profile/src/resources.rs#L98) |
| `assemble_application_endpoint_host` | `repository         .workspaces()         .map_err(&#124;error&#124; DaemonError::invalid_config(error.to_string()))?         .into_iter()         .map(&#124;workspace&#124; {             let workspace_id = workspace.id.clone();             let authored_roots = workspace                 .folder_paths()                 .into_iter()                 .map(PathBuf::from)                 .collect::<Vec<_>>();              // A workspace may refer to a removable volume, a deleted temporary             // checkout, or another path that is not mounted at startup.             // It remains inventory authority, but it cannot contribute project             // resources until its roots are available again. Do not let that             // workspace prevent the host from binding its endpoint.             if authored_roots.iter().any(&#124;root&#124; !root.is_dir()) {                 return Ok((workspace_id, ResourceCatalog::default()));             }              let resolved = match repository.resolve(&workspace_id) {                 Ok(snapshot) => snapshot.workspace.cwd,                 Err(_) if authored_roots.iter().any(&#124;root&#124; !root.is_dir()) => {                     return Ok((workspace_id, ResourceCatalog::default()));                 }                 Err(error) => return Err(DaemonError::invalid_config(error.to_string())),             };             let snapshot = match InstructionResolver::new_scoped(                 user_agent_dir,                 authority_root.join("workspaces").join(&workspace_id),                 &resolved,             )             .capture()             {                 Ok(snapshot) => snapshot,                 Err(_) if authored_roots.iter().any(&#124;root&#124; !root.is_dir()) => {                     return Ok((workspace_id, ResourceCatalog::default()));                 }                 Err(error) => return Err(DaemonError::invalid_config(error.to_string())),             };             let catalog = ResourceCatalog::from_snapshot(&snapshot)                 .map_err(&#124;error&#124; DaemonError::invalid_config(error.to_string()))?;             Ok((workspace_id, catalog))         })         .collect::<Result<std::collections::BTreeMap<_, _>, DaemonError>>` | [326](../../src/host_runtime.rs#L326) | receiver-type-required |
| `assemble_application_endpoint_host` | `repository         .workspaces()         .map_err(&#124;error&#124; DaemonError::invalid_config(error.to_string()))?         .into_iter()         .map` | [326](../../src/host_runtime.rs#L326) | receiver-type-required |
| `assemble_application_endpoint_host` | `repository         .workspaces()         .map_err(&#124;error&#124; DaemonError::invalid_config(error.to_string()))?         .into_iter` | [326](../../src/host_runtime.rs#L326) | receiver-type-required |
| `assemble_application_endpoint_host` | `repository         .workspaces()         .map_err` | [326](../../src/host_runtime.rs#L326) | receiver-type-required |
| `assemble_application_endpoint_host` | `repository         .workspaces` | [326](../../src/host_runtime.rs#L326) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace.id.clone` | [331](../../src/host_runtime.rs#L331) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace                 .folder_paths()                 .into_iter()                 .map(PathBuf::from)                 .collect::<Vec<_>>` | [332](../../src/host_runtime.rs#L332) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace                 .folder_paths()                 .into_iter()                 .map` | [332](../../src/host_runtime.rs#L332) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace                 .folder_paths()                 .into_iter` | [332](../../src/host_runtime.rs#L332) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace                 .folder_paths` | [332](../../src/host_runtime.rs#L332) | receiver-type-required |
| `assemble_application_endpoint_host` | `authored_roots.iter().any` | [343](../../src/host_runtime.rs#L343), [349](../../src/host_runtime.rs#L349), [362](../../src/host_runtime.rs#L362) | receiver-type-required |
| `assemble_application_endpoint_host` | `authored_roots.iter` | [343](../../src/host_runtime.rs#L343), [349](../../src/host_runtime.rs#L349), [362](../../src/host_runtime.rs#L362) | receiver-type-required |
| `assemble_application_endpoint_host` | `root.is_dir` | [343](../../src/host_runtime.rs#L343), [349](../../src/host_runtime.rs#L349), [362](../../src/host_runtime.rs#L362) | receiver-type-required |
| `assemble_application_endpoint_host` | `Ok` | [344](../../src/host_runtime.rs#L344), [350](../../src/host_runtime.rs#L350), [363](../../src/host_runtime.rs#L363), [369](../../src/host_runtime.rs#L369) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `ResourceCatalog::default` | [344](../../src/host_runtime.rs#L344), [350](../../src/host_runtime.rs#L350), [363](../../src/host_runtime.rs#L363) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `repository.resolve` | [347](../../src/host_runtime.rs#L347) | receiver-type-required |
| `assemble_application_endpoint_host` | `Err` | [352](../../src/host_runtime.rs#L352), [365](../../src/host_runtime.rs#L365) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `InstructionResolver::new_scoped(                 user_agent_dir,                 authority_root.join("workspaces").join(&workspace_id),                 &resolved,             )             .capture` | [354](../../src/host_runtime.rs#L354) | receiver-type-required |
| `assemble_application_endpoint_host` | `InstructionResolver::new_scoped` | [354](../../src/host_runtime.rs#L354) | [profile::instruction::InstructionResolver::new_scoped](../../../profile/src/instruction.rs#L300) |
| `assemble_application_endpoint_host` | `authority_root.join("workspaces").join` | [356](../../src/host_runtime.rs#L356) | receiver-type-required |
| `assemble_application_endpoint_host` | `authority_root.join` | [356](../../src/host_runtime.rs#L356) | receiver-type-required |
| `assemble_application_endpoint_host` | `ResourceCatalog::from_snapshot(&snapshot)                 .map_err` | [367](../../src/host_runtime.rs#L367) | receiver-type-required |
| `assemble_application_endpoint_host` | `Arc::clone` | [372](../../src/host_runtime.rs#L372), [382](../../src/host_runtime.rs#L382), [384](../../src/host_runtime.rs#L384), [386](../../src/host_runtime.rs#L386), [391](../../src/host_runtime.rs#L391), [410](../../src/host_runtime.rs#L410), [411](../../src/host_runtime.rs#L411) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `Arc::new` | [373](../../src/host_runtime.rs#L373), [376](../../src/host_runtime.rs#L376), [383](../../src/host_runtime.rs#L383), [390](../../src/host_runtime.rs#L390), [398](../../src/host_runtime.rs#L398), [409](../../src/host_runtime.rs#L409) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `SessionInputAdmissionAuthority::new` | [373](../../src/host_runtime.rs#L373) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) |
| `assemble_application_endpoint_host` | `authority_root.to_path_buf` | [374](../../src/host_runtime.rs#L374) | receiver-type-required |
| `assemble_application_endpoint_host` | `ProductionClientExtensions::open(             authority_root,             catalog,             workspace_resources,             EndpointCommandInputAuthority::new(                 Arc::clone(&process_host) as Arc<dyn SessionDeliveryAuthority>,                 Arc::new(move &#124;&#124; command_clock().map_err(&#124;error&#124; error.to_string())),                 Arc::clone(&input_admission),             ),             Arc::clone(&process_host),         )         .map_err` | [377](../../src/host_runtime.rs#L377) | receiver-type-required |
| `assemble_application_endpoint_host` | `ProductionClientExtensions::open` | [377](../../src/host_runtime.rs#L377) | [tekes-supervisor::client_extensions::ProductionClientExtensions::open](../../src/client_extensions.rs#L355) |
| `assemble_application_endpoint_host` | `EndpointCommandInputAuthority::new` | [381](../../src/host_runtime.rs#L381) | [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::new](../../src/resource_capability.rs#L384) |
| `assemble_application_endpoint_host` | `command_clock().map_err` | [383](../../src/host_runtime.rs#L383) | receiver-type-required |
| `assemble_application_endpoint_host` | `command_clock` | [383](../../src/host_runtime.rs#L383) | external-constructor-callback-or-unresolved |
| `assemble_application_endpoint_host` | `DaemonError::corrupt` | [388](../../src/host_runtime.rs#L388), [404](../../src/host_runtime.rs#L404), [415](../../src/host_runtime.rs#L415) | [tekes-supervisor::host_runtime::DaemonError::corrupt](../../src/host_runtime.rs#L582) |
| `assemble_application_endpoint_host` | `ClientAdminRoutes::new(authority_root, Arc::clone(&process_host))             .map_err` | [391](../../src/host_runtime.rs#L391) | receiver-type-required |
| `assemble_application_endpoint_host` | `ClientAdminRoutes::new` | [391](../../src/host_runtime.rs#L391) | [tekes-supervisor::client_admin::ClientAdminRoutes::new](../../src/client_admin.rs#L41) |
| `assemble_application_endpoint_host` | `extensions.routes` | [394](../../src/host_runtime.rs#L394) | receiver-type-required |
| `assemble_application_endpoint_host` | `extension_routes.extend` | [395](../../src/host_runtime.rs#L395) | receiver-type-required |
| `assemble_application_endpoint_host` | `admin_routes.routes` | [395](../../src/host_runtime.rs#L395) | receiver-type-required |
| `assemble_application_endpoint_host` | `process_host.workspace_service_binary` | [396](../../src/host_runtime.rs#L396) | receiver-type-required |
| `assemble_application_endpoint_host` | `workspace_helper.is_file` | [397](../../src/host_runtime.rs#L397) | receiver-type-required |
| `assemble_application_endpoint_host` | `extension_routes.push` | [398](../../src/host_runtime.rs#L398) | receiver-type-required |
| `assemble_application_endpoint_host` | `crate::workspace_routes::WorkspaceRoutes::open(authority_root, workspace_helper)                 .map_err` | [399](../../src/host_runtime.rs#L399) | receiver-type-required |
| `assemble_application_endpoint_host` | `crate::workspace_routes::WorkspaceRoutes::open` | [399](../../src/host_runtime.rs#L399) | [tekes-supervisor::workspace_routes::WorkspaceRoutes::open](../../src/workspace_routes.rs#L42) |
| `assemble_application_endpoint_host` | `CompositeProductionEndpointRoutes::compose(extension_routes)         .map_err` | [403](../../src/host_runtime.rs#L403) | receiver-type-required |
| `assemble_application_endpoint_host` | `CompositeProductionEndpointRoutes::compose` | [403](../../src/host_runtime.rs#L403) | [tekes-supervisor::endpoint_host::CompositeProductionEndpointRoutes::compose](../../src/endpoint_host.rs#L415) |
| `assemble_application_endpoint_host` | `ProductionEndpointHost::open_with_full_authorities_and_session_admission(         authority_root,         description,         clock,         Some(Arc::new(routes)),         Some(Arc::clone(&process_host) as Arc<dyn ProviderReadinessAuthority>),         Some(Arc::clone(&process_host) as Arc<dyn SessionDeliveryAuthority>),         Some(process_host as Arc<dyn QueueTransactionAuthority>),         input_admission,     )     .map_err` | [405](../../src/host_runtime.rs#L405) | receiver-type-required |
| `assemble_application_endpoint_host` | `ProductionEndpointHost::open_with_full_authorities_and_session_admission` | [405](../../src/host_runtime.rs#L405) | [tekes-supervisor::endpoint_host::ProductionEndpointHost::open_with_full_authorities_and_session_admission](../../src/endpoint_host.rs#L639) |
| `assemble_application_endpoint_host` | `Some` | [409](../../src/host_runtime.rs#L409), [410](../../src/host_runtime.rs#L410), [411](../../src/host_runtime.rs#L411), [412](../../src/host_runtime.rs#L412) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `File::from_raw_fd` | [420](../../src/host_runtime.rs#L420) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `file.as_raw_fd` | [422](../../src/host_runtime.rs#L422) | receiver-type-required |
| `watch_shutdown` | `TERMINATE_REQUESTED.load` | [428](../../src/host_runtime.rs#L428) | receiver-type-required |
| `watch_shutdown` | `Ok` | [429](../../src/host_runtime.rs#L429), [444](../../src/host_runtime.rs#L444) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `libc::poll` | [432](../../src/host_runtime.rs#L432) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `io::Error::last_os_error` | [437](../../src/host_runtime.rs#L437) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `error.kind` | [438](../../src/host_runtime.rs#L438), [446](../../src/host_runtime.rs#L446) | receiver-type-required |
| `watch_shutdown` | `Err` | [441](../../src/host_runtime.rs#L441), [447](../../src/host_runtime.rs#L447) | external-constructor-callback-or-unresolved |
| `watch_shutdown` | `DaemonError::io` | [441](../../src/host_runtime.rs#L441), [447](../../src/host_runtime.rs#L447) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `watch_shutdown` | `file.read` | [443](../../src/host_runtime.rs#L443) | receiver-type-required |
| `wait_for_launcher_shutdown` | `duplicate_fd(fd).map_err` | [456](../../src/host_runtime.rs#L456) | receiver-type-required |
| `wait_for_launcher_shutdown` | `duplicate_fd` | [456](../../src/host_runtime.rs#L456) | [tekes-supervisor::host_runtime::duplicate_fd](../../src/host_runtime.rs#L481) |
| `wait_for_launcher_shutdown` | `watch_shutdown` | [457](../../src/host_runtime.rs#L457) | [tekes-supervisor::host_runtime::watch_shutdown](../../src/host_runtime.rs#L418) |
| `request_termination` | `TERMINATE_REQUESTED.store` | [461](../../src/host_runtime.rs#L461) | receiver-type-required |
| `install_termination_handler` | `TERMINATE_REQUESTED.store` | [465](../../src/host_runtime.rs#L465) | receiver-type-required |
| `install_termination_handler` | `std::mem::zeroed::<libc::sigaction>` | [470](../../src/host_runtime.rs#L470) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `libc::sigemptyset` | [473](../../src/host_runtime.rs#L473) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `libc::sigaction` | [474](../../src/host_runtime.rs#L474) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `std::ptr::null_mut` | [474](../../src/host_runtime.rs#L474) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `Err` | [475](../../src/host_runtime.rs#L475) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `DaemonError::io` | [475](../../src/host_runtime.rs#L475) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `install_termination_handler` | `io::Error::last_os_error` | [475](../../src/host_runtime.rs#L475) | external-constructor-callback-or-unresolved |
| `install_termination_handler` | `Ok` | [478](../../src/host_runtime.rs#L478) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `libc::fcntl` | [484](../../src/host_runtime.rs#L484) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `Ok` | [486](../../src/host_runtime.rs#L486) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `io::Error::last_os_error` | [488](../../src/host_runtime.rs#L488) | external-constructor-callback-or-unresolved |
| `duplicate_fd` | `error.kind` | [489](../../src/host_runtime.rs#L489) | receiver-type-required |
| `duplicate_fd` | `Err` | [490](../../src/host_runtime.rs#L490) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `std::time::SystemTime::now()         .duration_since(std::time::UNIX_EPOCH)         .map_err` | [496](../../src/host_runtime.rs#L496) | receiver-type-required |
| `system_timestamp` | `std::time::SystemTime::now()         .duration_since` | [496](../../src/host_runtime.rs#L496) | receiver-type-required |
| `system_timestamp` | `std::time::SystemTime::now` | [496](../../src/host_runtime.rs#L496) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `error.to_string` | [498](../../src/host_runtime.rs#L498), [500](../../src/host_runtime.rs#L500) | receiver-type-required |
| `system_timestamp` | `duration.as_secs` | [499](../../src/host_runtime.rs#L499) | receiver-type-required |
| `system_timestamp` | `i64::try_from(seconds / 86_400).map_err` | [500](../../src/host_runtime.rs#L500) | receiver-type-required |
| `system_timestamp` | `i64::try_from` | [500](../../src/host_runtime.rs#L500) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `i64::from` | [510](../../src/host_runtime.rs#L510) | external-constructor-callback-or-unresolved |
| `system_timestamp` | `Ok` | [512](../../src/host_runtime.rs#L512) | external-constructor-callback-or-unresolved |
| `new` | `message.into` | [532](../../src/host_runtime.rs#L532) | receiver-type-required |
| `protocol` | `Self::new` | [537](../../src/host_runtime.rs#L537) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `io` | `Self::new` | [541](../../src/host_runtime.rs#L541) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `io` | `error.to_string` | [541](../../src/host_runtime.rs#L541) | receiver-type-required |
| `store` | `Self::new` | [546](../../src/host_runtime.rs#L546), [547](../../src/host_runtime.rs#L547), [549](../../src/host_runtime.rs#L549), [556](../../src/host_runtime.rs#L556), [559](../../src/host_runtime.rs#L559) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `store` | `error.to_string` | [546](../../src/host_runtime.rs#L546), [547](../../src/host_runtime.rs#L547), [549](../../src/host_runtime.rs#L549), [556](../../src/host_runtime.rs#L556) | receiver-type-required |
| `store` | `Self::io` | [558](../../src/host_runtime.rs#L558) | [tekes-supervisor::host_runtime::DaemonError::io](../../src/host_runtime.rs#L540) |
| `store` | `other.to_string` | [559](../../src/host_runtime.rs#L559) | receiver-type-required |
| `already_running` | `Self::new` | [564](../../src/host_runtime.rs#L564) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `invalid_install` | `Self::new` | [571](../../src/host_runtime.rs#L571) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `invalid_install_reason` | `Self::new` | [575](../../src/host_runtime.rs#L575) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `invalid_config` | `Self::new` | [579](../../src/host_runtime.rs#L579) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `corrupt` | `Self::new` | [583](../../src/host_runtime.rs#L583) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `required_broker` | `Self::new` | [587](../../src/host_runtime.rs#L587) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `credential` | `Self::new` | [591](../../src/host_runtime.rs#L591) | [tekes-supervisor::host_runtime::DaemonError::new](../../src/host_runtime.rs#L529) |
| `from` | `Self::io` | [603](../../src/host_runtime.rs#L603) | external-constructor-callback-or-unresolved |
| `from` | `Self::store` | [604](../../src/host_runtime.rs#L604) | external-constructor-callback-or-unresolved |
| `from` | `Self::corrupt` | [605](../../src/host_runtime.rs#L605) | external-constructor-callback-or-unresolved |
| `from` | `other.to_string` | [605](../../src/host_runtime.rs#L605) | receiver-type-required |
| `from` | `Self::io` | [613](../../src/host_runtime.rs#L613) | external-constructor-callback-or-unresolved |
| `from` | `Self::protocol` | [614](../../src/host_runtime.rs#L614) | external-constructor-callback-or-unresolved |
| `from` | `other.to_string` | [614](../../src/host_runtime.rs#L614) | receiver-type-required |
| `macos_storage_accepts_apfs_and_rejects_hfs` | `require_production_filesystem_name("apfs").expect` | [630](../../src/host_runtime.rs#L630) | receiver-type-required |
| `macos_storage_accepts_apfs_and_rejects_hfs` | `require_production_filesystem_name` | [630](../../src/host_runtime.rs#L630), [631](../../src/host_runtime.rs#L631) | [tekes-supervisor::host_runtime::require_production_filesystem_name](../../src/host_runtime.rs#L273) |
| `macos_storage_accepts_apfs_and_rejects_hfs` | `require_production_filesystem_name("hfs")             .expect_err` | [631](../../src/host_runtime.rs#L631) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `tempfile::tempdir().expect` | [639](../../src/host_runtime.rs#L639) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `tempfile::tempdir` | [639](../../src/host_runtime.rs#L639) | external-constructor-callback-or-unresolved |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory             .path()             .join("Library")             .join("Mobile Documents")             .join("Tekes")             .join` | [640](../../src/host_runtime.rs#L640) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory             .path()             .join("Library")             .join("Mobile Documents")             .join` | [640](../../src/host_runtime.rs#L640) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory             .path()             .join("Library")             .join` | [640](../../src/host_runtime.rs#L640) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory             .path()             .join` | [640](../../src/host_runtime.rs#L640) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory             .path` | [640](../../src/host_runtime.rs#L640) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `fs::create_dir_all(&cloud).expect` | [646](../../src/host_runtime.rs#L646) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `fs::create_dir_all` | [646](../../src/host_runtime.rs#L646) | external-constructor-callback-or-unresolved |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `reject_cloud_managed_storage(&cloud)             .expect_err` | [647](../../src/host_runtime.rs#L647) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `reject_cloud_managed_storage` | [647](../../src/host_runtime.rs#L647), [658](../../src/host_runtime.rs#L658) | [tekes-supervisor::host_runtime::reject_cloud_managed_storage](../../src/host_runtime.rs#L283) |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory.path().join` | [651](../../src/host_runtime.rs#L651), [653](../../src/host_runtime.rs#L653) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory.path` | [651](../../src/host_runtime.rs#L651), [653](../../src/host_runtime.rs#L653) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `symlink(             directory.path().join("Library").join("Mobile Documents"),             &alias,         )         .expect` | [652](../../src/host_runtime.rs#L652) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `symlink` | [652](../../src/host_runtime.rs#L652) | external-constructor-callback-or-unresolved |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `directory.path().join("Library").join` | [653](../../src/host_runtime.rs#L653) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `alias.join("Tekes").join` | [657](../../src/host_runtime.rs#L657) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `alias.join` | [657](../../src/host_runtime.rs#L657) | receiver-type-required |
| `production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias` | `reject_cloud_managed_storage(&aliased_storage)             .expect_err` | [658](../../src/host_runtime.rs#L658) | receiver-type-required |
| `production_timestamp_is_event_millisecond_utc` | `system_timestamp().expect` | [665](../../src/host_runtime.rs#L665) | receiver-type-required |
| `production_timestamp_is_event_millisecond_utc` | `system_timestamp` | [665](../../src/host_runtime.rs#L665) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L495) |
| `production_timestamp_is_event_millisecond_utc` | `schema::Event::decode(             &serde_json::to_vec(&serde_json::json!({                 "v":1,"seq":1,"kind":"stop_requested","ts":timestamp,                 "generation":1,"origin_key":"clock-test",                 "origin_tuple":{"principal":"test","client":"test","target":"test",                     "op":"cancel","key":"clock-test"}             }))             .expect("timestamp fixture JSON"),         )         .expect` | [666](../../src/host_runtime.rs#L666) | receiver-type-required |
| `production_timestamp_is_event_millisecond_utc` | `schema::Event::decode` | [666](../../src/host_runtime.rs#L666) | [schema::event::Event::decode](../../../schema/src/event.rs#L163) |
| `production_timestamp_is_event_millisecond_utc` | `serde_json::to_vec(&serde_json::json!({                 "v":1,"seq":1,"kind":"stop_requested","ts":timestamp,                 "generation":1,"origin_key":"clock-test",                 "origin_tuple":{"principal":"test","client":"test","target":"test",                     "op":"cancel","key":"clock-test"}             }))             .expect` | [667](../../src/host_runtime.rs#L667) | receiver-type-required |
| `production_timestamp_is_event_millisecond_utc` | `serde_json::to_vec` | [667](../../src/host_runtime.rs#L667) | external-constructor-callback-or-unresolved |
