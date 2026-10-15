# tekes-supervisor::builtin

[Package atlas](index.md) · [Source](../../src/builtin.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::builtin::Result](../../src/builtin.rs#L16) | type_item | `private` |  |
| [tekes-supervisor::builtin::BuiltInLaunch](../../src/builtin.rs#L20) | struct_item | `pub` |  |
| [tekes-supervisor::builtin::run](../../src/builtin.rs#L33) | function_item | `pub` |  |

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
  n1["tekes-supervisor::boot_trace::start"]
  n2["tekes-supervisor::boot_trace::phase"]
  n3["tekes-supervisor::bootstrap::rejects_legacy_environment"]
  n4["tekes-supervisor::bootstrap::read"]
  n5["tekes-supervisor::builtin::run"]
  n6["tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble"]
  n7["tekes-supervisor::host_runtime::preflight_storage"]
  n8["tekes-supervisor::host_runtime::assemble_application_endpoint_host"]
  n9["tekes-supervisor::host_runtime::wait_for_launcher_shutdown"]
  n10["tekes-supervisor::host_runtime::install_termination_handler"]
  n11["tekes-supervisor::host_runtime::system_timestamp"]
  n12["tekes-supervisor::host_runtime::ProductionRootLock::acquire"]
  n13["tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities"]
  n14["transport::auth::BearerToken::new"]
  n15["transport::server::TransportConfig::loopback"]
  n16["transport::server::WebClientConfig::loopback"]
  n5 --> n0
  n5 --> n1
  n5 --> n2
  n5 --> n3
  n5 --> n4
  n5 --> n6
  n5 --> n7
  n5 --> n8
  n5 --> n9
  n5 --> n10
  n5 --> n11
  n5 --> n12
  n5 --> n13
  n5 --> n14
  n5 --> n15
  n5 --> n16
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `run` | `serde_json::from_slice` | [34](../../src/builtin.rs#L34) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::read` | [34](../../src/builtin.rs#L34) | external-constructor-callback-or-unresolved |
| `run` | `launch.root.is_absolute` | [37](../../src/builtin.rs#L37) | receiver-type-required |
| `run` | `launch.worker.is_absolute` | [38](../../src/builtin.rs#L38) | receiver-type-required |
| `run` | `launch.listen.ip().is_loopback` | [39](../../src/builtin.rs#L39) | receiver-type-required |
| `run` | `launch.listen.ip` | [39](../../src/builtin.rs#L39) | receiver-type-required |
| `run` | `launch             .web_listen             .is_some_and` | [40](../../src/builtin.rs#L40) | receiver-type-required |
| `run` | `address.ip().is_loopback` | [42](../../src/builtin.rs#L42) | receiver-type-required |
| `run` | `address.ip` | [42](../../src/builtin.rs#L42) | receiver-type-required |
| `run` | `Err` | [44](../../src/builtin.rs#L44), [49](../../src/builtin.rs#L49), [57](../../src/builtin.rs#L57), [60](../../src/builtin.rs#L60), [81](../../src/builtin.rs#L81) | external-constructor-callback-or-unresolved |
| `run` | `io::Error::other("invalid built-in launch configuration").into` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `io::Error::other` | [44](../../src/builtin.rs#L44), [49](../../src/builtin.rs#L49), [57](../../src/builtin.rs#L57), [60](../../src/builtin.rs#L60), [81](../../src/builtin.rs#L81), [111](../../src/builtin.rs#L111), [135](../../src/builtin.rs#L135), [144](../../src/builtin.rs#L144) | external-constructor-callback-or-unresolved |
| `run` | `launch.web_listen.is_some` | [48](../../src/builtin.rs#L48) | receiver-type-required |
| `run` | `io::Error::other("unauthenticated-web-client-disabled").into` | [49](../../src/builtin.rs#L49) | receiver-type-required |
| `run` | `launch.credential_bindings.values().any` | [51](../../src/builtin.rs#L51) | receiver-type-required |
| `run` | `launch.credential_bindings.values` | [51](../../src/builtin.rs#L51) | receiver-type-required |
| `run` | `name.is_empty` | [52](../../src/builtin.rs#L52) | receiver-type-required |
| `run` | `name.bytes().enumerate().all` | [53](../../src/builtin.rs#L53) | receiver-type-required |
| `run` | `name.bytes().enumerate` | [53](../../src/builtin.rs#L53) | receiver-type-required |
| `run` | `name.bytes` | [53](../../src/builtin.rs#L53) | receiver-type-required |
| `run` | `byte.is_ascii_alphabetic` | [54](../../src/builtin.rs#L54) | receiver-type-required |
| `run` | `byte.is_ascii_digit` | [54](../../src/builtin.rs#L54) | receiver-type-required |
| `run` | `io::Error::other("invalid-launch-credential-bindings").into` | [57](../../src/builtin.rs#L57) | receiver-type-required |
| `run` | `crate::bootstrap::rejects_legacy_environment` | [59](../../src/builtin.rs#L59) | [tekes-supervisor::bootstrap::rejects_legacy_environment](../../src/bootstrap.rs#L166) |
| `run` | `io::Error::other("legacy-secret-environment-rejected").into` | [60](../../src/builtin.rs#L60) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain(             launch                 .providers                 .web_search                 .iter()                 .map(&#124;search&#124; search.credential_key.as_str()),         )         .collect` | [62](../../src/builtin.rs#L62) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain` | [62](../../src/builtin.rs#L62) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map` | [62](../../src/builtin.rs#L62) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter` | [62](../../src/builtin.rs#L62) | receiver-type-required |
| `run` | `provider.credential_key.as_deref` | [66](../../src/builtin.rs#L66) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter()                 .map` | [68](../../src/builtin.rs#L68) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter` | [68](../../src/builtin.rs#L68) | receiver-type-required |
| `run` | `search.credential_key.as_str` | [72](../../src/builtin.rs#L72) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map(String::as_str)         .collect` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys` | [75](../../src/builtin.rs#L75) | receiver-type-required |
| `run` | `io::Error::other("credential bindings must match launch configuration").into` | [81](../../src/builtin.rs#L81) | receiver-type-required |
| `run` | `crate::bootstrap::read` | [84](../../src/builtin.rs#L84) | [tekes-supervisor::bootstrap::read](../../src/bootstrap.rs#L70) |
| `run` | `transport::BearerToken::new` | [85](../../src/builtin.rs#L85) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `run` | `drop` | [86](../../src/builtin.rs#L86) | external-constructor-callback-or-unresolved |
| `run` | `Arc::new` | [87](../../src/builtin.rs#L87), [169](../../src/builtin.rs#L169) | external-constructor-callback-or-unresolved |
| `run` | `crate::boot_trace::start` | [88](../../src/builtin.rs#L88) | [tekes-supervisor::boot_trace::start](../../src/boot_trace.rs#L28) |
| `run` | `launch.root.join` | [89](../../src/builtin.rs#L89) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode(0o700)         .create` | [90](../../src/builtin.rs#L90) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode` | [90](../../src/builtin.rs#L90) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive` | [90](../../src/builtin.rs#L90) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new` | [90](../../src/builtin.rs#L90) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags(libc::O_NOFOLLOW)         .open` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new` | [94](../../src/builtin.rs#L94) | external-constructor-callback-or-unresolved |
| `run` | `storage.join` | [100](../../src/builtin.rs#L100) | receiver-type-required |
| `run` | `host_runtime::ProductionRootLock::acquire` | [101](../../src/builtin.rs#L101) | [tekes-supervisor::host_runtime::ProductionRootLock::acquire](../../src/host_runtime.rs#L71) |
| `run` | `crate::boot_trace::phase` | [102](../../src/builtin.rs#L102), [104](../../src/builtin.rs#L104), [141](../../src/builtin.rs#L141), [155](../../src/builtin.rs#L155), [157](../../src/builtin.rs#L157), [176](../../src/builtin.rs#L176), [187](../../src/builtin.rs#L187), [194](../../src/builtin.rs#L194), [212](../../src/builtin.rs#L212) | [tekes-supervisor::boot_trace::phase](../../src/boot_trace.rs#L60) |
| `run` | `host_runtime::preflight_storage` | [103](../../src/builtin.rs#L103) | [tekes-supervisor::host_runtime::preflight_storage](../../src/host_runtime.rs#L213) |
| `run` | `profile::ConfigRepository::open` | [105](../../src/builtin.rs#L105) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L814) |
| `run` | `repository.providers` | [106](../../src/builtin.rs#L106) | receiver-type-required |
| `run` | `current         .revision         .checked_add(1)         .ok_or_else` | [108](../../src/builtin.rs#L108) | receiver-type-required |
| `run` | `current         .revision         .checked_add` | [108](../../src/builtin.rs#L108) | receiver-type-required |
| `run` | `repository.publish_providers` | [112](../../src/builtin.rs#L112) | receiver-type-required |
| `run` | `repository.settings` | [115](../../src/builtin.rs#L115) | receiver-type-required |
| `run` | `providers.providers.iter().any` | [116](../../src/builtin.rs#L116) | receiver-type-required |
| `run` | `providers.providers.iter` | [116](../../src/builtin.rs#L116), [123](../../src/builtin.rs#L123) | receiver-type-required |
| `run` | `settings.default_provider.as_deref` | [117](../../src/builtin.rs#L117) | receiver-type-required |
| `run` | `Some` | [117](../../src/builtin.rs#L117), [119](../../src/builtin.rs#L119), [130](../../src/builtin.rs#L130), [180](../../src/builtin.rs#L180), [204](../../src/builtin.rs#L204) | external-constructor-callback-or-unresolved |
| `run` | `provider.id.as_str` | [117](../../src/builtin.rs#L117) | receiver-type-required |
| `run` | `provider.models.iter().any` | [118](../../src/builtin.rs#L118) | receiver-type-required |
| `run` | `provider.models.iter` | [118](../../src/builtin.rs#L118) | receiver-type-required |
| `run` | `settings.default_model.as_deref` | [119](../../src/builtin.rs#L119) | receiver-type-required |
| `run` | `model.id.as_str` | [119](../../src/builtin.rs#L119) | receiver-type-required |
| `run` | `providers.providers.iter().find_map` | [123](../../src/builtin.rs#L123) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find(&#124;model&#124; model.enabled)                 .map` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `provider                 .models                 .iter` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `provider.id.clone` | [128](../../src/builtin.rs#L128) | receiver-type-required |
| `run` | `model.id.clone` | [128](../../src/builtin.rs#L128) | receiver-type-required |
| `run` | `selected.map_or` | [130](../../src/builtin.rs#L130) | receiver-type-required |
| `run` | `revision                 .checked_add(1)                 .ok_or_else` | [133](../../src/builtin.rs#L133) | receiver-type-required |
| `run` | `revision                 .checked_add` | [133](../../src/builtin.rs#L133) | receiver-type-required |
| `run` | `repository.publish_settings` | [138](../../src/builtin.rs#L138) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else(&#124;&#124; io::Error::other("HOME is required for shared user skills"))?         .join` | [142](../../src/builtin.rs#L142) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else` | [142](../../src/builtin.rs#L142) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map` | [142](../../src/builtin.rs#L142) | receiver-type-required |
| `run` | `std::env::var_os` | [142](../../src/builtin.rs#L142) | external-constructor-callback-or-unresolved |
| `run` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [146](../../src/builtin.rs#L146) | receiver-type-required |
| `run` | `ProductionProcessHost::open_with_secret_authorities` | [147](../../src/builtin.rs#L147) | [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities](../../src/process_host.rs#L805) |
| `run` | `process.preflight_mandatory_authorities` | [156](../../src/builtin.rs#L156) | receiver-type-required |
| `run` | `host_runtime::assemble_application_endpoint_host` | [158](../../src/builtin.rs#L158) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `run` | `build.into` | [161](../../src/builtin.rs#L161) | receiver-type-required |
| `run` | `launch.root.to_string_lossy().into` | [162](../../src/builtin.rs#L162), [166](../../src/builtin.rs#L166) | receiver-type-required |
| `run` | `launch.root.to_string_lossy` | [162](../../src/builtin.rs#L162), [166](../../src/builtin.rs#L166) | receiver-type-required |
| `run` | `host_runtime::system_timestamp()                 .map_err` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `host_runtime::system_timestamp` | [170](../../src/builtin.rs#L170) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L475) |
| `run` | `Arc::clone` | [174](../../src/builtin.rs#L174) | external-constructor-callback-or-unresolved |
| `run` | `tokio::net::TcpListener::bind` | [177](../../src/builtin.rs#L177), [180](../../src/builtin.rs#L180) | external-constructor-callback-or-unresolved |
| `run` | `listener.local_addr` | [178](../../src/builtin.rs#L178), [202](../../src/builtin.rs#L202) | receiver-type-required |
| `run` | `transport::TransportConfig::loopback` | [183](../../src/builtin.rs#L183) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `run` | `ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?             .with_file_changes` | [185](../../src/builtin.rs#L185) | receiver-type-required |
| `run` | `ProductionCarrierAssembly::assemble` | [185](../../src/builtin.rs#L185) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2629) |
| `run` | `process.clone` | [185](../../src/builtin.rs#L185), [186](../../src/builtin.rs#L186) | receiver-type-required |
| `run` | `process.attach_streams` | [188](../../src/builtin.rs#L188) | receiver-type-required |
| `run` | `assembly.streams().clone` | [188](../../src/builtin.rs#L188) | receiver-type-required |
| `run` | `assembly.streams` | [188](../../src/builtin.rs#L188) | receiver-type-required |
| `run` | `process.arm_admission` | [191](../../src/builtin.rs#L191) | receiver-type-required |
| `run` | `assembly.mark_ready` | [192](../../src/builtin.rs#L192) | receiver-type-required |
| `run` | `process.start_periodic_sweep` | [193](../../src/builtin.rs#L193) | receiver-type-required |
| `run` | `host_runtime::install_termination_handler` | [195](../../src/builtin.rs#L195) | [tekes-supervisor::host_runtime::install_termination_handler](../../src/host_runtime.rs#L444) |
| `run` | `assembly.into_server` | [196](../../src/builtin.rs#L196) | receiver-type-required |
| `run` | `server.handle` | [197](../../src/builtin.rs#L197) | receiver-type-required |
| `run` | `server.web_client` | [203](../../src/builtin.rs#L203) | receiver-type-required |
| `run` | `transport::WebClientConfig::loopback` | [203](../../src/builtin.rs#L203) | [transport::server::WebClientConfig::loopback](../../../transport/src/server.rs#L272) |
| `run` | `format!("http://{bound}/").into` | [210](../../src/builtin.rs#L210) | receiver-type-required |
| `run` | `io::stdout().flush` | [214](../../src/builtin.rs#L214) | receiver-type-required |
| `run` | `io::stdout` | [214](../../src/builtin.rs#L214) | external-constructor-callback-or-unresolved |
| `run` | `process.run_post_ready` | [216](../../src/builtin.rs#L216) | receiver-type-required |
| `run` | `server.serve` | [217](../../src/builtin.rs#L217) | receiver-type-required |
| `run` | `service.serve` | [221](../../src/builtin.rs#L221) | receiver-type-required |
| `run` | `std::future::pending` | [222](../../src/builtin.rs#L222) | external-constructor-callback-or-unresolved |
| `run` | `tokio::sync::oneshot::channel` | [229](../../src/builtin.rs#L229) | external-constructor-callback-or-unresolved |
| `run` | `std::thread::Builder::new()         .name("launcher-lifetime".into())         .spawn` | [230](../../src/builtin.rs#L230) | receiver-type-required |
| `run` | `std::thread::Builder::new()         .name` | [230](../../src/builtin.rs#L230) | receiver-type-required |
| `run` | `std::thread::Builder::new` | [230](../../src/builtin.rs#L230) | external-constructor-callback-or-unresolved |
| `run` | `"launcher-lifetime".into` | [231](../../src/builtin.rs#L231) | receiver-type-required |
| `run` | `lifetime_sender.send` | [233](../../src/builtin.rs#L233) | receiver-type-required |
| `run` | `host_runtime::wait_for_launcher_shutdown` | [233](../../src/builtin.rs#L233) | [tekes-supervisor::host_runtime::wait_for_launcher_shutdown](../../src/host_runtime.rs#L435) |
| `run` | `Ok` | [251](../../src/builtin.rs#L251) | external-constructor-callback-or-unresolved |
