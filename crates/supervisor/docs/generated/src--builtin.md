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

<details><summary>Functions 1–1: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::open"]
  n1["provider::environment_secrets::EnvironmentSecretStore::capture"]
  n2["tekes-supervisor::boot_trace::start"]
  n3["tekes-supervisor::boot_trace::phase"]
  n4["tekes-supervisor::builtin::run"]
  n5["tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble"]
  n6["tekes-supervisor::host_runtime::preflight_storage"]
  n7["tekes-supervisor::host_runtime::assemble_application_endpoint_host"]
  n8["tekes-supervisor::host_runtime::wait_for_launcher_shutdown"]
  n9["tekes-supervisor::host_runtime::install_termination_handler"]
  n10["tekes-supervisor::host_runtime::system_timestamp"]
  n11["tekes-supervisor::host_runtime::endpoint_token_from_environment"]
  n12["tekes-supervisor::host_runtime::ProductionRootLock::acquire"]
  n13["tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities"]
  n14["transport::auth::BearerToken::new"]
  n15["transport::server::TransportConfig::loopback"]
  n16["transport::server::WebClientConfig::loopback"]
  n4 --> n0
  n4 --> n1
  n4 --> n2
  n4 --> n3
  n4 --> n5
  n4 --> n6
  n4 --> n7
  n4 --> n8
  n4 --> n9
  n4 --> n10
  n4 --> n11
  n4 --> n12
  n4 --> n13
  n4 --> n14
  n4 --> n15
  n4 --> n16
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `run` | `crate::boot_trace::start` | [33](../../src/builtin.rs#L33) | [tekes-supervisor::boot_trace::start](../../src/boot_trace.rs#L28) |
| `run` | `serde_json::from_slice` | [34](../../src/builtin.rs#L34) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::read` | [34](../../src/builtin.rs#L34) | external-constructor-callback-or-unresolved |
| `run` | `launch.root.is_absolute` | [36](../../src/builtin.rs#L36) | receiver-type-required |
| `run` | `launch.worker.is_absolute` | [37](../../src/builtin.rs#L37) | receiver-type-required |
| `run` | `launch.listen.ip().is_loopback` | [38](../../src/builtin.rs#L38) | receiver-type-required |
| `run` | `launch.listen.ip` | [38](../../src/builtin.rs#L38) | receiver-type-required |
| `run` | `launch             .web_listen             .is_some_and` | [39](../../src/builtin.rs#L39) | receiver-type-required |
| `run` | `address.ip().is_loopback` | [41](../../src/builtin.rs#L41) | receiver-type-required |
| `run` | `address.ip` | [41](../../src/builtin.rs#L41) | receiver-type-required |
| `run` | `Err` | [43](../../src/builtin.rs#L43), [64](../../src/builtin.rs#L64) | external-constructor-callback-or-unresolved |
| `run` | `io::Error::other("invalid built-in launch configuration").into` | [43](../../src/builtin.rs#L43) | receiver-type-required |
| `run` | `io::Error::other` | [43](../../src/builtin.rs#L43), [64](../../src/builtin.rs#L64), [93](../../src/builtin.rs#L93), [117](../../src/builtin.rs#L117), [126](../../src/builtin.rs#L126) | external-constructor-callback-or-unresolved |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain(             launch                 .providers                 .web_search                 .iter()                 .map(&#124;search&#124; search.credential_key.as_str()),         )         .collect` | [45](../../src/builtin.rs#L45) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain` | [45](../../src/builtin.rs#L45) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map` | [45](../../src/builtin.rs#L45) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter` | [45](../../src/builtin.rs#L45) | receiver-type-required |
| `run` | `provider.credential_key.as_deref` | [49](../../src/builtin.rs#L49) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter()                 .map` | [51](../../src/builtin.rs#L51) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter` | [51](../../src/builtin.rs#L51) | receiver-type-required |
| `run` | `search.credential_key.as_str` | [55](../../src/builtin.rs#L55) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map(String::as_str)         .collect` | [58](../../src/builtin.rs#L58) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map` | [58](../../src/builtin.rs#L58) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys` | [58](../../src/builtin.rs#L58) | receiver-type-required |
| `run` | `io::Error::other("credential bindings must match launch configuration").into` | [64](../../src/builtin.rs#L64) | receiver-type-required |
| `run` | `Arc::new` | [67](../../src/builtin.rs#L67), [151](../../src/builtin.rs#L151) | external-constructor-callback-or-unresolved |
| `run` | `provider::EnvironmentSecretStore::capture` | [67](../../src/builtin.rs#L67) | [provider::environment_secrets::EnvironmentSecretStore::capture](../../../provider/src/environment_secrets.rs#L14) |
| `run` | `host_runtime::endpoint_token_from_environment` | [70](../../src/builtin.rs#L70) | [tekes-supervisor::host_runtime::endpoint_token_from_environment](../../src/host_runtime.rs#L65) |
| `run` | `launch.root.join` | [71](../../src/builtin.rs#L71) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode(0o700)         .create` | [72](../../src/builtin.rs#L72) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode` | [72](../../src/builtin.rs#L72) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive` | [72](../../src/builtin.rs#L72) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new` | [72](../../src/builtin.rs#L72) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags(libc::O_NOFOLLOW)         .open` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write` | [76](../../src/builtin.rs#L76) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new` | [76](../../src/builtin.rs#L76) | external-constructor-callback-or-unresolved |
| `run` | `storage.join` | [82](../../src/builtin.rs#L82) | receiver-type-required |
| `run` | `host_runtime::ProductionRootLock::acquire` | [83](../../src/builtin.rs#L83) | [tekes-supervisor::host_runtime::ProductionRootLock::acquire](../../src/host_runtime.rs#L97) |
| `run` | `crate::boot_trace::phase` | [84](../../src/builtin.rs#L84), [86](../../src/builtin.rs#L86), [123](../../src/builtin.rs#L123), [137](../../src/builtin.rs#L137), [139](../../src/builtin.rs#L139), [158](../../src/builtin.rs#L158), [169](../../src/builtin.rs#L169), [176](../../src/builtin.rs#L176), [194](../../src/builtin.rs#L194) | [tekes-supervisor::boot_trace::phase](../../src/boot_trace.rs#L60) |
| `run` | `host_runtime::preflight_storage` | [85](../../src/builtin.rs#L85) | [tekes-supervisor::host_runtime::preflight_storage](../../src/host_runtime.rs#L239) |
| `run` | `profile::ConfigRepository::open` | [87](../../src/builtin.rs#L87) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L684) |
| `run` | `repository.providers` | [88](../../src/builtin.rs#L88) | receiver-type-required |
| `run` | `current         .revision         .checked_add(1)         .ok_or_else` | [90](../../src/builtin.rs#L90) | receiver-type-required |
| `run` | `current         .revision         .checked_add` | [90](../../src/builtin.rs#L90) | receiver-type-required |
| `run` | `repository.publish_providers` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `repository.settings` | [97](../../src/builtin.rs#L97) | receiver-type-required |
| `run` | `providers.providers.iter().any` | [98](../../src/builtin.rs#L98) | receiver-type-required |
| `run` | `providers.providers.iter` | [98](../../src/builtin.rs#L98), [105](../../src/builtin.rs#L105) | receiver-type-required |
| `run` | `settings.default_provider.as_deref` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `Some` | [99](../../src/builtin.rs#L99), [101](../../src/builtin.rs#L101), [112](../../src/builtin.rs#L112), [162](../../src/builtin.rs#L162), [186](../../src/builtin.rs#L186) | external-constructor-callback-or-unresolved |
| `run` | `provider.id.as_str` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `provider.models.iter().any` | [100](../../src/builtin.rs#L100) | receiver-type-required |
| `run` | `provider.models.iter` | [100](../../src/builtin.rs#L100) | receiver-type-required |
| `run` | `settings.default_model.as_deref` | [101](../../src/builtin.rs#L101) | receiver-type-required |
| `run` | `model.id.as_str` | [101](../../src/builtin.rs#L101) | receiver-type-required |
| `run` | `providers.providers.iter().find_map` | [105](../../src/builtin.rs#L105) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find(&#124;model&#124; model.enabled)                 .map` | [106](../../src/builtin.rs#L106) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find` | [106](../../src/builtin.rs#L106) | receiver-type-required |
| `run` | `provider                 .models                 .iter` | [106](../../src/builtin.rs#L106) | receiver-type-required |
| `run` | `provider.id.clone` | [110](../../src/builtin.rs#L110) | receiver-type-required |
| `run` | `model.id.clone` | [110](../../src/builtin.rs#L110) | receiver-type-required |
| `run` | `selected.map_or` | [112](../../src/builtin.rs#L112) | receiver-type-required |
| `run` | `revision                 .checked_add(1)                 .ok_or_else` | [115](../../src/builtin.rs#L115) | receiver-type-required |
| `run` | `revision                 .checked_add` | [115](../../src/builtin.rs#L115) | receiver-type-required |
| `run` | `repository.publish_settings` | [120](../../src/builtin.rs#L120) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else(&#124;&#124; io::Error::other("HOME is required for shared user skills"))?         .join` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `std::env::var_os` | [124](../../src/builtin.rs#L124) | external-constructor-callback-or-unresolved |
| `run` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [128](../../src/builtin.rs#L128) | receiver-type-required |
| `run` | `ProductionProcessHost::open_with_secret_authorities` | [129](../../src/builtin.rs#L129) | [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities](../../src/process_host.rs#L802) |
| `run` | `process.preflight_mandatory_authorities` | [138](../../src/builtin.rs#L138) | receiver-type-required |
| `run` | `host_runtime::assemble_application_endpoint_host` | [140](../../src/builtin.rs#L140) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L312) |
| `run` | `build.into` | [143](../../src/builtin.rs#L143) | receiver-type-required |
| `run` | `launch.root.to_string_lossy().into` | [144](../../src/builtin.rs#L144), [148](../../src/builtin.rs#L148) | receiver-type-required |
| `run` | `launch.root.to_string_lossy` | [144](../../src/builtin.rs#L144), [148](../../src/builtin.rs#L148) | receiver-type-required |
| `run` | `host_runtime::system_timestamp()                 .map_err` | [152](../../src/builtin.rs#L152) | receiver-type-required |
| `run` | `host_runtime::system_timestamp` | [152](../../src/builtin.rs#L152) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L495) |
| `run` | `Arc::clone` | [156](../../src/builtin.rs#L156) | external-constructor-callback-or-unresolved |
| `run` | `tokio::net::TcpListener::bind` | [159](../../src/builtin.rs#L159), [162](../../src/builtin.rs#L162) | external-constructor-callback-or-unresolved |
| `run` | `listener.local_addr` | [160](../../src/builtin.rs#L160), [184](../../src/builtin.rs#L184) | receiver-type-required |
| `run` | `transport::TransportConfig::loopback` | [165](../../src/builtin.rs#L165) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `run` | `transport::BearerToken::new` | [165](../../src/builtin.rs#L165) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `run` | `ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?             .with_file_changes` | [167](../../src/builtin.rs#L167) | receiver-type-required |
| `run` | `ProductionCarrierAssembly::assemble` | [167](../../src/builtin.rs#L167) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2562) |
| `run` | `process.clone` | [167](../../src/builtin.rs#L167), [168](../../src/builtin.rs#L168) | receiver-type-required |
| `run` | `process.attach_streams` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `assembly.streams().clone` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `assembly.streams` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `process.arm_admission` | [173](../../src/builtin.rs#L173) | receiver-type-required |
| `run` | `assembly.mark_ready` | [174](../../src/builtin.rs#L174) | receiver-type-required |
| `run` | `process.start_periodic_sweep` | [175](../../src/builtin.rs#L175) | receiver-type-required |
| `run` | `host_runtime::install_termination_handler` | [177](../../src/builtin.rs#L177) | [tekes-supervisor::host_runtime::install_termination_handler](../../src/host_runtime.rs#L464) |
| `run` | `assembly.into_server` | [178](../../src/builtin.rs#L178) | receiver-type-required |
| `run` | `server.handle` | [179](../../src/builtin.rs#L179) | receiver-type-required |
| `run` | `server.web_client` | [185](../../src/builtin.rs#L185) | receiver-type-required |
| `run` | `transport::WebClientConfig::loopback` | [185](../../src/builtin.rs#L185) | [transport::server::WebClientConfig::loopback](../../../transport/src/server.rs#L272) |
| `run` | `format!("http://{bound}/").into` | [192](../../src/builtin.rs#L192) | receiver-type-required |
| `run` | `io::stdout().flush` | [196](../../src/builtin.rs#L196) | receiver-type-required |
| `run` | `io::stdout` | [196](../../src/builtin.rs#L196) | external-constructor-callback-or-unresolved |
| `run` | `process.run_post_ready` | [198](../../src/builtin.rs#L198) | receiver-type-required |
| `run` | `server.serve` | [199](../../src/builtin.rs#L199) | receiver-type-required |
| `run` | `service.serve` | [203](../../src/builtin.rs#L203) | receiver-type-required |
| `run` | `std::future::pending` | [204](../../src/builtin.rs#L204) | external-constructor-callback-or-unresolved |
| `run` | `tokio::sync::oneshot::channel` | [211](../../src/builtin.rs#L211) | external-constructor-callback-or-unresolved |
| `run` | `std::thread::Builder::new()         .name("launcher-lifetime".into())         .spawn` | [212](../../src/builtin.rs#L212) | receiver-type-required |
| `run` | `std::thread::Builder::new()         .name` | [212](../../src/builtin.rs#L212) | receiver-type-required |
| `run` | `std::thread::Builder::new` | [212](../../src/builtin.rs#L212) | external-constructor-callback-or-unresolved |
| `run` | `"launcher-lifetime".into` | [213](../../src/builtin.rs#L213) | receiver-type-required |
| `run` | `lifetime_sender.send` | [215](../../src/builtin.rs#L215) | receiver-type-required |
| `run` | `host_runtime::wait_for_launcher_shutdown` | [215](../../src/builtin.rs#L215) | [tekes-supervisor::host_runtime::wait_for_launcher_shutdown](../../src/host_runtime.rs#L455) |
| `run` | `Ok` | [233](../../src/builtin.rs#L233) | external-constructor-callback-or-unresolved |
