# tekes-supervisor::builtin

[Package atlas](index.md) · [Source](../../src/builtin.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::builtin::Result](../../src/builtin.rs#L16) | type_item | `private` |  |
| [tekes-supervisor::builtin::BuiltInLaunch](../../src/builtin.rs#L20) | struct_item | `pub` |  |
| [tekes-supervisor::builtin::run](../../src/builtin.rs#L32) | function_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `ProductionCarrierAssembly` | `crate::endpoint_carrier::ProductionCarrierAssembly` | `private` |
| `host_runtime` | `crate::host_runtime` | `private` |
| `ProductionProcessHost` | `crate::process_host::ProductionProcessHost` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `DirBuilderExt` | `std::os::unix::fs::DirBuilderExt` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `io` | `std::io` | `private` |
| `Write` | `std::io::Write` | `private` |
| `SocketAddr` | `std::net::SocketAddr` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–1: 14 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::open"]
  n1["provider::environment_secrets::EnvironmentSecretStore::capture"]
  n2["tekes-supervisor::builtin::run"]
  n3["tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble"]
  n4["tekes-supervisor::host_runtime::preflight_storage"]
  n5["tekes-supervisor::host_runtime::assemble_application_endpoint_host"]
  n6["tekes-supervisor::host_runtime::wait_for_launcher_shutdown"]
  n7["tekes-supervisor::host_runtime::install_termination_handler"]
  n8["tekes-supervisor::host_runtime::system_timestamp"]
  n9["tekes-supervisor::host_runtime::endpoint_token_from_environment"]
  n10["tekes-supervisor::host_runtime::ProductionRootLock::acquire"]
  n11["tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities"]
  n12["transport::auth::BearerToken::new"]
  n13["transport::server::TransportConfig::loopback"]
  n14["transport::server::WebClientConfig::loopback"]
  n2 --> n0
  n2 --> n1
  n2 --> n3
  n2 --> n4
  n2 --> n5
  n2 --> n6
  n2 --> n7
  n2 --> n8
  n2 --> n9
  n2 --> n10
  n2 --> n11
  n2 --> n12
  n2 --> n13
  n2 --> n14
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `run` | `serde_json::from_slice` | [33](../../src/builtin.rs#L33) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::read` | [33](../../src/builtin.rs#L33) | external-constructor-callback-or-unresolved |
| `run` | `launch.root.is_absolute` | [35](../../src/builtin.rs#L35) | receiver-type-required |
| `run` | `launch.worker.is_absolute` | [36](../../src/builtin.rs#L36) | receiver-type-required |
| `run` | `launch.listen.ip().is_loopback` | [37](../../src/builtin.rs#L37) | receiver-type-required |
| `run` | `launch.listen.ip` | [37](../../src/builtin.rs#L37) | receiver-type-required |
| `run` | `launch             .web_listen             .is_some_and` | [38](../../src/builtin.rs#L38) | receiver-type-required |
| `run` | `address.ip().is_loopback` | [40](../../src/builtin.rs#L40) | receiver-type-required |
| `run` | `address.ip` | [40](../../src/builtin.rs#L40) | receiver-type-required |
| `run` | `Err` | [42](../../src/builtin.rs#L42), [63](../../src/builtin.rs#L63) | external-constructor-callback-or-unresolved |
| `run` | `io::Error::other("invalid built-in launch configuration").into` | [42](../../src/builtin.rs#L42) | receiver-type-required |
| `run` | `io::Error::other` | [42](../../src/builtin.rs#L42), [63](../../src/builtin.rs#L63), [90](../../src/builtin.rs#L90), [114](../../src/builtin.rs#L114), [122](../../src/builtin.rs#L122) | external-constructor-callback-or-unresolved |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain(             launch                 .providers                 .web_search                 .iter()                 .map(&#124;search&#124; search.credential_key.as_str()),         )         .collect` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `provider.credential_key.as_deref` | [48](../../src/builtin.rs#L48) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter()                 .map` | [50](../../src/builtin.rs#L50) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter` | [50](../../src/builtin.rs#L50) | receiver-type-required |
| `run` | `search.credential_key.as_str` | [54](../../src/builtin.rs#L54) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map(String::as_str)         .collect` | [57](../../src/builtin.rs#L57) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map` | [57](../../src/builtin.rs#L57) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys` | [57](../../src/builtin.rs#L57) | receiver-type-required |
| `run` | `io::Error::other("credential bindings must match launch configuration").into` | [63](../../src/builtin.rs#L63) | receiver-type-required |
| `run` | `Arc::new` | [66](../../src/builtin.rs#L66), [145](../../src/builtin.rs#L145) | external-constructor-callback-or-unresolved |
| `run` | `provider::EnvironmentSecretStore::capture` | [66](../../src/builtin.rs#L66) | [provider::environment_secrets::EnvironmentSecretStore::capture](../../../provider/src/environment_secrets.rs#L14) |
| `run` | `host_runtime::endpoint_token_from_environment` | [69](../../src/builtin.rs#L69) | [tekes-supervisor::host_runtime::endpoint_token_from_environment](../../src/host_runtime.rs#L65) |
| `run` | `launch.root.join` | [70](../../src/builtin.rs#L70) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode(0o700)         .create` | [71](../../src/builtin.rs#L71) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode` | [71](../../src/builtin.rs#L71) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive` | [71](../../src/builtin.rs#L71) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new` | [71](../../src/builtin.rs#L71) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags(libc::O_NOFOLLOW)         .open` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new` | [75](../../src/builtin.rs#L75) | external-constructor-callback-or-unresolved |
| `run` | `storage.join` | [81](../../src/builtin.rs#L81) | receiver-type-required |
| `run` | `host_runtime::ProductionRootLock::acquire` | [82](../../src/builtin.rs#L82) | [tekes-supervisor::host_runtime::ProductionRootLock::acquire](../../src/host_runtime.rs#L97) |
| `run` | `host_runtime::preflight_storage` | [83](../../src/builtin.rs#L83) | [tekes-supervisor::host_runtime::preflight_storage](../../src/host_runtime.rs#L238) |
| `run` | `profile::ConfigRepository::open` | [84](../../src/builtin.rs#L84) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L684) |
| `run` | `repository.providers` | [85](../../src/builtin.rs#L85) | receiver-type-required |
| `run` | `current         .revision         .checked_add(1)         .ok_or_else` | [87](../../src/builtin.rs#L87) | receiver-type-required |
| `run` | `current         .revision         .checked_add` | [87](../../src/builtin.rs#L87) | receiver-type-required |
| `run` | `repository.publish_providers` | [91](../../src/builtin.rs#L91) | receiver-type-required |
| `run` | `repository.settings` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `providers.providers.iter().any` | [95](../../src/builtin.rs#L95) | receiver-type-required |
| `run` | `providers.providers.iter` | [95](../../src/builtin.rs#L95), [102](../../src/builtin.rs#L102) | receiver-type-required |
| `run` | `settings.default_provider.as_deref` | [96](../../src/builtin.rs#L96) | receiver-type-required |
| `run` | `Some` | [96](../../src/builtin.rs#L96), [98](../../src/builtin.rs#L98), [109](../../src/builtin.rs#L109), [155](../../src/builtin.rs#L155), [177](../../src/builtin.rs#L177) | external-constructor-callback-or-unresolved |
| `run` | `provider.id.as_str` | [96](../../src/builtin.rs#L96) | receiver-type-required |
| `run` | `provider.models.iter().any` | [97](../../src/builtin.rs#L97) | receiver-type-required |
| `run` | `provider.models.iter` | [97](../../src/builtin.rs#L97) | receiver-type-required |
| `run` | `settings.default_model.as_deref` | [98](../../src/builtin.rs#L98) | receiver-type-required |
| `run` | `model.id.as_str` | [98](../../src/builtin.rs#L98) | receiver-type-required |
| `run` | `providers.providers.iter().find_map` | [102](../../src/builtin.rs#L102) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find(&#124;model&#124; model.enabled)                 .map` | [103](../../src/builtin.rs#L103) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find` | [103](../../src/builtin.rs#L103) | receiver-type-required |
| `run` | `provider                 .models                 .iter` | [103](../../src/builtin.rs#L103) | receiver-type-required |
| `run` | `provider.id.clone` | [107](../../src/builtin.rs#L107) | receiver-type-required |
| `run` | `model.id.clone` | [107](../../src/builtin.rs#L107) | receiver-type-required |
| `run` | `selected.map_or` | [109](../../src/builtin.rs#L109) | receiver-type-required |
| `run` | `revision                 .checked_add(1)                 .ok_or_else` | [112](../../src/builtin.rs#L112) | receiver-type-required |
| `run` | `revision                 .checked_add` | [112](../../src/builtin.rs#L112) | receiver-type-required |
| `run` | `repository.publish_settings` | [117](../../src/builtin.rs#L117) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else(&#124;&#124; io::Error::other("HOME is required for shared user skills"))?         .join` | [120](../../src/builtin.rs#L120) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else` | [120](../../src/builtin.rs#L120) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map` | [120](../../src/builtin.rs#L120) | receiver-type-required |
| `run` | `std::env::var_os` | [120](../../src/builtin.rs#L120) | external-constructor-callback-or-unresolved |
| `run` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `ProductionProcessHost::open_with_secret_authorities` | [125](../../src/builtin.rs#L125) | [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities](../../src/process_host.rs#L777) |
| `run` | `process.preflight_mandatory_authorities` | [133](../../src/builtin.rs#L133) | receiver-type-required |
| `run` | `host_runtime::assemble_application_endpoint_host` | [134](../../src/builtin.rs#L134) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L368) |
| `run` | `build.into` | [137](../../src/builtin.rs#L137) | receiver-type-required |
| `run` | `launch.root.to_string_lossy().into` | [138](../../src/builtin.rs#L138), [142](../../src/builtin.rs#L142) | receiver-type-required |
| `run` | `launch.root.to_string_lossy` | [138](../../src/builtin.rs#L138), [142](../../src/builtin.rs#L142) | receiver-type-required |
| `run` | `host_runtime::system_timestamp()                 .map_err` | [146](../../src/builtin.rs#L146) | receiver-type-required |
| `run` | `host_runtime::system_timestamp` | [146](../../src/builtin.rs#L146) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L551) |
| `run` | `Arc::clone` | [150](../../src/builtin.rs#L150) | external-constructor-callback-or-unresolved |
| `run` | `tokio::net::TcpListener::bind` | [152](../../src/builtin.rs#L152), [155](../../src/builtin.rs#L155) | external-constructor-callback-or-unresolved |
| `run` | `listener.local_addr` | [153](../../src/builtin.rs#L153), [175](../../src/builtin.rs#L175) | receiver-type-required |
| `run` | `transport::TransportConfig::loopback` | [158](../../src/builtin.rs#L158) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `run` | `transport::BearerToken::new` | [158](../../src/builtin.rs#L158) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `run` | `ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?             .with_file_changes` | [160](../../src/builtin.rs#L160) | receiver-type-required |
| `run` | `ProductionCarrierAssembly::assemble` | [160](../../src/builtin.rs#L160) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1691) |
| `run` | `process.clone` | [160](../../src/builtin.rs#L160), [161](../../src/builtin.rs#L161) | receiver-type-required |
| `run` | `process.attach_streams` | [162](../../src/builtin.rs#L162) | receiver-type-required |
| `run` | `assembly.streams().clone` | [162](../../src/builtin.rs#L162) | receiver-type-required |
| `run` | `assembly.streams` | [162](../../src/builtin.rs#L162) | receiver-type-required |
| `run` | `process.defer_existing_session_recovery` | [163](../../src/builtin.rs#L163) | receiver-type-required |
| `run` | `process.boot_sweep` | [164](../../src/builtin.rs#L164) | receiver-type-required |
| `run` | `assembly.finish_recovery` | [165](../../src/builtin.rs#L165) | receiver-type-required |
| `run` | `process.start_periodic_sweep` | [166](../../src/builtin.rs#L166) | receiver-type-required |
| `run` | `process.start_schedule_timer` | [167](../../src/builtin.rs#L167) | receiver-type-required |
| `run` | `host_runtime::install_termination_handler` | [168](../../src/builtin.rs#L168) | [tekes-supervisor::host_runtime::install_termination_handler](../../src/host_runtime.rs#L520) |
| `run` | `assembly.into_server` | [169](../../src/builtin.rs#L169) | receiver-type-required |
| `run` | `server.handle` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `server.web_client` | [176](../../src/builtin.rs#L176) | receiver-type-required |
| `run` | `transport::WebClientConfig::loopback` | [176](../../src/builtin.rs#L176) | [transport::server::WebClientConfig::loopback](../../../transport/src/server.rs#L272) |
| `run` | `format!("http://{bound}/").into` | [183](../../src/builtin.rs#L183) | receiver-type-required |
| `run` | `io::stdout().flush` | [186](../../src/builtin.rs#L186) | receiver-type-required |
| `run` | `io::stdout` | [186](../../src/builtin.rs#L186) | external-constructor-callback-or-unresolved |
| `run` | `server.serve` | [187](../../src/builtin.rs#L187) | receiver-type-required |
| `run` | `service.serve` | [191](../../src/builtin.rs#L191) | receiver-type-required |
| `run` | `std::future::pending` | [192](../../src/builtin.rs#L192) | external-constructor-callback-or-unresolved |
| `run` | `tokio::sync::oneshot::channel` | [199](../../src/builtin.rs#L199) | external-constructor-callback-or-unresolved |
| `run` | `std::thread::Builder::new()         .name("launcher-lifetime".into())         .spawn` | [200](../../src/builtin.rs#L200) | receiver-type-required |
| `run` | `std::thread::Builder::new()         .name` | [200](../../src/builtin.rs#L200) | receiver-type-required |
| `run` | `std::thread::Builder::new` | [200](../../src/builtin.rs#L200) | external-constructor-callback-or-unresolved |
| `run` | `"launcher-lifetime".into` | [201](../../src/builtin.rs#L201) | receiver-type-required |
| `run` | `lifetime_sender.send` | [203](../../src/builtin.rs#L203) | receiver-type-required |
| `run` | `host_runtime::wait_for_launcher_shutdown` | [203](../../src/builtin.rs#L203) | [tekes-supervisor::host_runtime::wait_for_launcher_shutdown](../../src/host_runtime.rs#L511) |
| `run` | `Ok` | [221](../../src/builtin.rs#L221) | external-constructor-callback-or-unresolved |
