# tekes-supervisor::builtin

[Package atlas](index.md) · [Source](../../src/builtin.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::builtin::Result](../../src/builtin.rs#L16) | type_item | `private` |  |
| [tekes-supervisor::builtin::BuiltInLaunch](../../src/builtin.rs#L20) | struct_item | `pub` |  |
| [tekes-supervisor::builtin::run](../../src/builtin.rs#L35) | function_item | `pub` |  |

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
  n13["tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_store"]
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
| `run` | `serde_json::from_slice` | [36](../../src/builtin.rs#L36) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::read` | [36](../../src/builtin.rs#L36) | external-constructor-callback-or-unresolved |
| `run` | `launch.root.is_absolute` | [39](../../src/builtin.rs#L39) | receiver-type-required |
| `run` | `launch.worker.is_absolute` | [40](../../src/builtin.rs#L40) | receiver-type-required |
| `run` | `launch.listen.ip().is_loopback` | [41](../../src/builtin.rs#L41) | receiver-type-required |
| `run` | `launch.listen.ip` | [41](../../src/builtin.rs#L41) | receiver-type-required |
| `run` | `launch             .web_listen             .is_some_and` | [42](../../src/builtin.rs#L42) | receiver-type-required |
| `run` | `address.ip().is_loopback` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `address.ip` | [44](../../src/builtin.rs#L44) | receiver-type-required |
| `run` | `Err` | [46](../../src/builtin.rs#L46), [51](../../src/builtin.rs#L51), [59](../../src/builtin.rs#L59), [62](../../src/builtin.rs#L62), [83](../../src/builtin.rs#L83) | external-constructor-callback-or-unresolved |
| `run` | `io::Error::other("invalid built-in launch configuration").into` | [46](../../src/builtin.rs#L46) | receiver-type-required |
| `run` | `io::Error::other` | [46](../../src/builtin.rs#L46), [51](../../src/builtin.rs#L51), [59](../../src/builtin.rs#L59), [62](../../src/builtin.rs#L62), [83](../../src/builtin.rs#L83), [116](../../src/builtin.rs#L116), [140](../../src/builtin.rs#L140), [149](../../src/builtin.rs#L149) | external-constructor-callback-or-unresolved |
| `run` | `launch.web_listen.is_some` | [50](../../src/builtin.rs#L50) | receiver-type-required |
| `run` | `io::Error::other("unauthenticated-web-client-disabled").into` | [51](../../src/builtin.rs#L51) | receiver-type-required |
| `run` | `launch.credential_bindings.values().any` | [53](../../src/builtin.rs#L53) | receiver-type-required |
| `run` | `launch.credential_bindings.values` | [53](../../src/builtin.rs#L53) | receiver-type-required |
| `run` | `name.is_empty` | [54](../../src/builtin.rs#L54) | receiver-type-required |
| `run` | `name.bytes().enumerate().all` | [55](../../src/builtin.rs#L55) | receiver-type-required |
| `run` | `name.bytes().enumerate` | [55](../../src/builtin.rs#L55) | receiver-type-required |
| `run` | `name.bytes` | [55](../../src/builtin.rs#L55) | receiver-type-required |
| `run` | `byte.is_ascii_alphabetic` | [56](../../src/builtin.rs#L56) | receiver-type-required |
| `run` | `byte.is_ascii_digit` | [56](../../src/builtin.rs#L56) | receiver-type-required |
| `run` | `io::Error::other("invalid-launch-credential-bindings").into` | [59](../../src/builtin.rs#L59) | receiver-type-required |
| `run` | `crate::bootstrap::rejects_legacy_environment` | [61](../../src/builtin.rs#L61) | [tekes-supervisor::bootstrap::rejects_legacy_environment](../../src/bootstrap.rs#L166) |
| `run` | `io::Error::other("legacy-secret-environment-rejected").into` | [62](../../src/builtin.rs#L62) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain(             launch                 .providers                 .web_search                 .iter()                 .map(&#124;search&#124; search.credential_key.as_str()),         )         .collect` | [64](../../src/builtin.rs#L64) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map(&#124;provider&#124; provider.credential_key.as_deref())         .chain` | [64](../../src/builtin.rs#L64) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter()         .filter_map` | [64](../../src/builtin.rs#L64) | receiver-type-required |
| `run` | `launch         .providers         .providers         .iter` | [64](../../src/builtin.rs#L64) | receiver-type-required |
| `run` | `provider.credential_key.as_deref` | [68](../../src/builtin.rs#L68) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter()                 .map` | [70](../../src/builtin.rs#L70) | receiver-type-required |
| `run` | `launch                 .providers                 .web_search                 .iter` | [70](../../src/builtin.rs#L70) | receiver-type-required |
| `run` | `search.credential_key.as_str` | [74](../../src/builtin.rs#L74) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map(String::as_str)         .collect` | [77](../../src/builtin.rs#L77) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys()         .map` | [77](../../src/builtin.rs#L77) | receiver-type-required |
| `run` | `launch         .credential_bindings         .keys` | [77](../../src/builtin.rs#L77) | receiver-type-required |
| `run` | `required.is_subset` | [82](../../src/builtin.rs#L82) | receiver-type-required |
| `run` | `io::Error::other(             "launch credentials must include every configured provider binding",         )         .into` | [83](../../src/builtin.rs#L83) | receiver-type-required |
| `run` | `crate::bootstrap::read` | [89](../../src/builtin.rs#L89) | [tekes-supervisor::bootstrap::read](../../src/bootstrap.rs#L70) |
| `run` | `transport::BearerToken::new` | [90](../../src/builtin.rs#L90) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `run` | `drop` | [91](../../src/builtin.rs#L91) | external-constructor-callback-or-unresolved |
| `run` | `Arc::new` | [92](../../src/builtin.rs#L92), [161](../../src/builtin.rs#L161), [178](../../src/builtin.rs#L178) | external-constructor-callback-or-unresolved |
| `run` | `crate::boot_trace::start` | [93](../../src/builtin.rs#L93) | [tekes-supervisor::boot_trace::start](../../src/boot_trace.rs#L28) |
| `run` | `launch.root.join` | [94](../../src/builtin.rs#L94) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode(0o700)         .create` | [95](../../src/builtin.rs#L95) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive(true)         .mode` | [95](../../src/builtin.rs#L95) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new()         .recursive` | [95](../../src/builtin.rs#L95) | receiver-type-required |
| `run` | `std::fs::DirBuilder::new` | [95](../../src/builtin.rs#L95) | external-constructor-callback-or-unresolved |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags(libc::O_NOFOLLOW)         .open` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode(0o600)         .custom_flags` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate(false)         .mode` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create(true)         .truncate` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write(true)         .create` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new()         .write` | [99](../../src/builtin.rs#L99) | receiver-type-required |
| `run` | `std::fs::OpenOptions::new` | [99](../../src/builtin.rs#L99) | external-constructor-callback-or-unresolved |
| `run` | `storage.join` | [105](../../src/builtin.rs#L105) | receiver-type-required |
| `run` | `host_runtime::ProductionRootLock::acquire` | [106](../../src/builtin.rs#L106) | [tekes-supervisor::host_runtime::ProductionRootLock::acquire](../../src/host_runtime.rs#L70) |
| `run` | `crate::boot_trace::phase` | [107](../../src/builtin.rs#L107), [109](../../src/builtin.rs#L109), [146](../../src/builtin.rs#L146), [164](../../src/builtin.rs#L164), [166](../../src/builtin.rs#L166), [185](../../src/builtin.rs#L185), [196](../../src/builtin.rs#L196), [203](../../src/builtin.rs#L203), [221](../../src/builtin.rs#L221) | [tekes-supervisor::boot_trace::phase](../../src/boot_trace.rs#L60) |
| `run` | `host_runtime::preflight_storage` | [108](../../src/builtin.rs#L108) | [tekes-supervisor::host_runtime::preflight_storage](../../src/host_runtime.rs#L210) |
| `run` | `profile::ConfigRepository::open` | [110](../../src/builtin.rs#L110) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L543) |
| `run` | `repository.providers` | [111](../../src/builtin.rs#L111) | receiver-type-required |
| `run` | `current         .revision         .checked_add(1)         .ok_or_else` | [113](../../src/builtin.rs#L113) | receiver-type-required |
| `run` | `current         .revision         .checked_add` | [113](../../src/builtin.rs#L113) | receiver-type-required |
| `run` | `repository.publish_providers` | [117](../../src/builtin.rs#L117) | receiver-type-required |
| `run` | `repository.settings` | [120](../../src/builtin.rs#L120) | receiver-type-required |
| `run` | `providers.providers.iter().any` | [121](../../src/builtin.rs#L121) | receiver-type-required |
| `run` | `providers.providers.iter` | [121](../../src/builtin.rs#L121), [128](../../src/builtin.rs#L128) | receiver-type-required |
| `run` | `settings.default_provider.as_deref` | [122](../../src/builtin.rs#L122) | receiver-type-required |
| `run` | `Some` | [122](../../src/builtin.rs#L122), [124](../../src/builtin.rs#L124), [135](../../src/builtin.rs#L135), [189](../../src/builtin.rs#L189), [213](../../src/builtin.rs#L213) | external-constructor-callback-or-unresolved |
| `run` | `provider.id.as_str` | [122](../../src/builtin.rs#L122) | receiver-type-required |
| `run` | `provider.models.iter().any` | [123](../../src/builtin.rs#L123) | receiver-type-required |
| `run` | `provider.models.iter` | [123](../../src/builtin.rs#L123) | receiver-type-required |
| `run` | `settings.default_model.as_deref` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `model.id.as_str` | [124](../../src/builtin.rs#L124) | receiver-type-required |
| `run` | `providers.providers.iter().find_map` | [128](../../src/builtin.rs#L128) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find(&#124;model&#124; model.enabled)                 .map` | [129](../../src/builtin.rs#L129) | receiver-type-required |
| `run` | `provider                 .models                 .iter()                 .find` | [129](../../src/builtin.rs#L129) | receiver-type-required |
| `run` | `provider                 .models                 .iter` | [129](../../src/builtin.rs#L129) | receiver-type-required |
| `run` | `provider.id.clone` | [133](../../src/builtin.rs#L133) | receiver-type-required |
| `run` | `model.id.clone` | [133](../../src/builtin.rs#L133) | receiver-type-required |
| `run` | `selected.map_or` | [135](../../src/builtin.rs#L135) | receiver-type-required |
| `run` | `revision                 .checked_add(1)                 .ok_or_else` | [138](../../src/builtin.rs#L138) | receiver-type-required |
| `run` | `revision                 .checked_add` | [138](../../src/builtin.rs#L138) | receiver-type-required |
| `run` | `repository.publish_settings` | [143](../../src/builtin.rs#L143) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else(&#124;&#124; io::Error::other("HOME is required for shared user skills"))?         .join` | [147](../../src/builtin.rs#L147) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map(PathBuf::from)         .ok_or_else` | [147](../../src/builtin.rs#L147) | receiver-type-required |
| `run` | `std::env::var_os("HOME")         .map` | [147](../../src/builtin.rs#L147) | receiver-type-required |
| `run` | `std::env::var_os` | [147](../../src/builtin.rs#L147) | external-constructor-callback-or-unresolved |
| `run` | `option_env!("TEKES_SELECTED_BUILD").unwrap_or` | [151](../../src/builtin.rs#L151) | receiver-type-required |
| `run` | `ProductionProcessHost::open_with_secret_store` | [152](../../src/builtin.rs#L152) | [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_store](../../src/process_host.rs#L802) |
| `run` | `process         .mcp_runtime()         .set_stdio_connector` | [159](../../src/builtin.rs#L159) | receiver-type-required |
| `run` | `process         .mcp_runtime` | [159](../../src/builtin.rs#L159) | receiver-type-required |
| `run` | `crate::stdio_channels::UnixStdioChannels` | [161](../../src/builtin.rs#L161) | external-constructor-callback-or-unresolved |
| `run` | `process.preflight_mandatory_authorities` | [165](../../src/builtin.rs#L165) | receiver-type-required |
| `run` | `host_runtime::assemble_application_endpoint_host` | [167](../../src/builtin.rs#L167) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `run` | `build.into` | [170](../../src/builtin.rs#L170) | receiver-type-required |
| `run` | `launch.root.to_string_lossy().into` | [171](../../src/builtin.rs#L171), [175](../../src/builtin.rs#L175) | receiver-type-required |
| `run` | `launch.root.to_string_lossy` | [171](../../src/builtin.rs#L171), [175](../../src/builtin.rs#L175) | receiver-type-required |
| `run` | `host_runtime::system_timestamp()                 .map_err` | [179](../../src/builtin.rs#L179) | receiver-type-required |
| `run` | `host_runtime::system_timestamp` | [179](../../src/builtin.rs#L179) | [tekes-supervisor::host_runtime::system_timestamp](../../src/host_runtime.rs#L406) |
| `run` | `Arc::clone` | [183](../../src/builtin.rs#L183) | external-constructor-callback-or-unresolved |
| `run` | `tokio::net::TcpListener::bind` | [186](../../src/builtin.rs#L186), [189](../../src/builtin.rs#L189) | external-constructor-callback-or-unresolved |
| `run` | `listener.local_addr` | [187](../../src/builtin.rs#L187), [211](../../src/builtin.rs#L211) | receiver-type-required |
| `run` | `transport::TransportConfig::loopback` | [192](../../src/builtin.rs#L192) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `run` | `ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?             .with_file_changes` | [194](../../src/builtin.rs#L194) | receiver-type-required |
| `run` | `ProductionCarrierAssembly::assemble` | [194](../../src/builtin.rs#L194) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2785) |
| `run` | `process.clone` | [194](../../src/builtin.rs#L194), [195](../../src/builtin.rs#L195) | receiver-type-required |
| `run` | `process.attach_streams` | [197](../../src/builtin.rs#L197) | receiver-type-required |
| `run` | `assembly.streams().clone` | [197](../../src/builtin.rs#L197) | receiver-type-required |
| `run` | `assembly.streams` | [197](../../src/builtin.rs#L197) | receiver-type-required |
| `run` | `process.arm_admission` | [200](../../src/builtin.rs#L200) | receiver-type-required |
| `run` | `assembly.mark_ready` | [201](../../src/builtin.rs#L201) | receiver-type-required |
| `run` | `process.start_periodic_sweep` | [202](../../src/builtin.rs#L202) | receiver-type-required |
| `run` | `host_runtime::install_termination_handler` | [204](../../src/builtin.rs#L204) | [tekes-supervisor::host_runtime::install_termination_handler](../../src/host_runtime.rs#L375) |
| `run` | `assembly.into_server` | [205](../../src/builtin.rs#L205) | receiver-type-required |
| `run` | `server.handle` | [206](../../src/builtin.rs#L206) | receiver-type-required |
| `run` | `server.web_client` | [212](../../src/builtin.rs#L212) | receiver-type-required |
| `run` | `transport::WebClientConfig::loopback` | [212](../../src/builtin.rs#L212) | [transport::server::WebClientConfig::loopback](../../../transport/src/server.rs#L269) |
| `run` | `format!("http://{bound}/").into` | [219](../../src/builtin.rs#L219) | receiver-type-required |
| `run` | `io::stdout().flush` | [223](../../src/builtin.rs#L223) | receiver-type-required |
| `run` | `io::stdout` | [223](../../src/builtin.rs#L223) | external-constructor-callback-or-unresolved |
| `run` | `process.run_post_ready` | [225](../../src/builtin.rs#L225) | receiver-type-required |
| `run` | `server.serve` | [226](../../src/builtin.rs#L226) | receiver-type-required |
| `run` | `service.serve` | [230](../../src/builtin.rs#L230) | receiver-type-required |
| `run` | `std::future::pending` | [231](../../src/builtin.rs#L231) | external-constructor-callback-or-unresolved |
| `run` | `tokio::sync::oneshot::channel` | [238](../../src/builtin.rs#L238) | external-constructor-callback-or-unresolved |
| `run` | `std::thread::Builder::new()         .name("launcher-lifetime".into())         .spawn` | [239](../../src/builtin.rs#L239) | receiver-type-required |
| `run` | `std::thread::Builder::new()         .name` | [239](../../src/builtin.rs#L239) | receiver-type-required |
| `run` | `std::thread::Builder::new` | [239](../../src/builtin.rs#L239) | external-constructor-callback-or-unresolved |
| `run` | `"launcher-lifetime".into` | [240](../../src/builtin.rs#L240) | receiver-type-required |
| `run` | `lifetime_sender.send` | [242](../../src/builtin.rs#L242) | receiver-type-required |
| `run` | `host_runtime::wait_for_launcher_shutdown` | [242](../../src/builtin.rs#L242) | [tekes-supervisor::host_runtime::wait_for_launcher_shutdown](../../src/host_runtime.rs#L366) |
| `run` | `Ok` | [260](../../src/builtin.rs#L260) | external-constructor-callback-or-unresolved |
