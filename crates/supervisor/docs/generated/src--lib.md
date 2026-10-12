# tekes-supervisor

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::ChildLaunchState](../../src/lib.rs#L47) | enum_item | `pub` |  |
| [tekes-supervisor::ChildLaunchRegistry](../../src/lib.rs#L56) | struct_item | `pub` |  |
| [tekes-supervisor::ChildLaunchRegistry::record](../../src/lib.rs#L61) | function_item | `pub` |  |
| [tekes-supervisor::ChildLaunchRegistry::resolve](../../src/lib.rs#L71) | function_item | `pub` |  |
| [tekes-supervisor::launch_failure](../../src/lib.rs#L87) | function_item | `private` |  |
| [tekes-supervisor::WorkerLaunchSpec](../../src/lib.rs#L97) | struct_item | `pub` |  |
| [tekes-supervisor::ProfiledWorkerLaunchSpec](../../src/lib.rs#L109) | struct_item | `pub` |  |
| [tekes-supervisor::WorkerLaunch](../../src/lib.rs#L122) | struct_item | `pub` |  |
| [tekes-supervisor::WorkerLaunchBindings](../../src/lib.rs#L135) | struct_item | `pub` |  |
| [tekes-supervisor::WorkerLaunchBindingResolver](../../src/lib.rs#L140) | type_item | `private` |  |
| [tekes-supervisor::WorkerLaunchBindings::default](../../src/lib.rs#L144) | function_item | `private` |  |
| [tekes-supervisor::launch_profiled_worker_with_bindings](../../src/lib.rs#L155) | function_item | `pub` |  |
| [tekes-supervisor::launch_profiled_worker_with_credentials](../../src/lib.rs#L163) | function_item | `pub` |  |
| [tekes-supervisor::launch_profiled_worker_with_credentials_and_provider_test_redirect](../../src/lib.rs#L175) | function_item | `pub` |  |
| [tekes-supervisor::launch_profiled_worker_with_secret_store_and_binding_resolver](../../src/lib.rs#L196) | function_item | `pub` |  |
| [tekes-supervisor::launch_profiled_worker_inner](../../src/lib.rs#L216) | function_item | `private` |  |
| [tekes-supervisor::launch_worker_inner](../../src/lib.rs#L290) | function_item | `private` |  |
| [tekes-supervisor::closed_command](../../src/lib.rs#L429) | function_item | `private` |  |
| [tekes-supervisor::FORWARDED_PROXY_VARIABLES](../../src/lib.rs#L437) | const_item | `private` |  |
| [tekes-supervisor::forwarded_worker_environment](../../src/lib.rs#L456) | function_item | `pub` |  |
| [tekes-supervisor::forwarded_environment_from](../../src/lib.rs#L463) | function_item | `private` |  |
| [tekes-supervisor::snapshot_path](../../src/lib.rs#L470) | function_item | `private` |  |
| [tekes-supervisor::open_snapshot](../../src/lib.rs#L481) | function_item | `private` |  |
| [tekes-supervisor::clear_cloexec](../../src/lib.rs#L488) | function_item | `private` |  |
| [tekes-supervisor::LaunchError](../../src/lib.rs#L499) | enum_item | `pub` |  |
| [tekes-supervisor::tests::worker_environment_forwards_only_the_evidence_capture_directory](../../src/lib.rs#L526) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::tests::worker_command_drops_the_complete_ambient_environment](../../src/lib.rs#L557) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `File` | `std::fs::File` | `private` |
| `OpenOptions` | `std::fs::OpenOptions` | `private` |
| `io` | `std::io` | `private` |
| `AsRawFd` | `std::os::fd::AsRawFd` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `UnixStream` | `std::os::unix::net::UnixStream` | `private` |
| `CommandExt` | `std::os::unix::process::CommandExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Child` | `std::process::Child` | `private` |
| `Command` | `std::process::Command` | `private` |
| `Stdio` | `std::process::Stdio` | `private` |
| `ConfigRepository` | `profile::ConfigRepository` | `private` |
| `ConfigSnapshot` | `profile::ConfigSnapshot` | `private` |
| `DynamicToolCatalog` | `profile::DynamicToolCatalog` | `private` |
| `InstructionSnapshot` | `profile::InstructionSnapshot` | `private` |
| `LaunchBindings` | `profile::LaunchBindings` | `private` |
| `LaunchProfile` | `profile::LaunchProfile` | `private` |
| `ProfileError` | `profile::ProfileError` | `private` |
| `CredentialBroker` | `provider::CredentialBroker` | `private` |
| `CredentialBrokerControl` | `provider::CredentialBrokerControl` | `private` |
| `CredentialScope` | `provider::CredentialScope` | `private` |
| `ResolvedCredentialBindings` | `provider::ResolvedCredentialBindings` | `private` |
| `RevokedCredentialScope` | `provider::RevokedCredentialScope` | `private` |
| `SecretStore` | `provider::SecretStore` | `private` |
| `resolve_config_credentials` | `provider::resolve_config_credentials` | `private` |
| `start_credential_channel` | `provider::start_credential_channel` | `private` |
| `LaunchChild` | `worker_control::LaunchChild` | `private` |
| `LaunchResult` | `worker_control::LaunchResult` | `private` |
| `closed_command` | `super::closed_command` | `private` |
| `forwarded_environment_from` | `super::forwarded_environment_from` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::boot_trace` | `pub` |  |
| `tekes-supervisor::builtin` | `pub` |  |
| `tekes-supervisor::catalog` | `private` |  |
| `tekes-supervisor::client_admin` | `pub` |  |
| `tekes-supervisor::client_extensions` | `pub` |  |
| `tekes-supervisor::context_usage` | `private` |  |
| `tekes-supervisor::continuation_journal` | `pub` |  |
| `tekes-supervisor::dynamic_bindings` | `pub` |  |
| `tekes-supervisor::endpoint_carrier` | `pub` |  |
| `tekes-supervisor::endpoint_host` | `pub` |  |
| `tekes-supervisor::file_leases` | `pub` |  |
| `tekes-supervisor::file_observation` | `pub` |  |
| `tekes-supervisor::host_runtime` | `pub` |  |
| `tekes-supervisor::mcp_continuation` | `pub` |  |
| `tekes-supervisor::mcp_runtime` | `pub` |  |
| `tekes-supervisor::process_host` | `pub` |  |
| `tekes-supervisor::production_tool_control` | `pub` |  |
| `tekes-supervisor::request_index` | `private` |  |
| `tekes-supervisor::resource_capability` | `pub` |  |
| `tekes-supervisor::thread_card` | `private` |  |
| `tekes-supervisor::tool_control` | `pub` |  |
| `tekes-supervisor::workspace_routes` | `pub` |  |
| `tekes-supervisor::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–16: 24 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigSnapshot::decode"]
  n1["profile::instruction::LaunchProfile::resolve_and_publish_for_binding"]
  n2["profile::instruction::InstructionSnapshot::decode"]
  n3["profile::launch::LaunchBindings::bind"]
  n4["profile::launch::LaunchBindings::decode_verified"]
  n5["provider::credential::CredentialBroker::with_revoked"]
  n6["provider::credential::start_credential_channel"]
  n7["provider::request::endpoint_origin"]
  n8["provider::secret_store::resolve_config_credentials"]
  n9["store::asset::AssetStore::new"]
  n10["tekes-supervisor::WorkerLaunchBindings::default"]
  n11["tekes-supervisor::launch_profiled_worker_with_bindings"]
  n12["tekes-supervisor::launch_profiled_worker_with_credentials"]
  n13["tekes-supervisor::launch_profiled_worker_with_credentials_and_provider_test_redirect"]
  n14["tekes-supervisor::launch_profiled_worker_with_secret_store_and_binding_resolver"]
  n15["tekes-supervisor::launch_profiled_worker_inner"]
  n16["tekes-supervisor::launch_worker_inner"]
  n17["tekes-supervisor::closed_command"]
  n18["tekes-supervisor::forwarded_worker_environment"]
  n19["tekes-supervisor::forwarded_environment_from"]
  n20["tekes-supervisor::snapshot_path"]
  n21["tekes-supervisor::open_snapshot"]
  n22["tekes-supervisor::clear_cloexec"]
  n23["tekes-supervisor::ChildLaunchRegistry::record"]
  n24["tekes-supervisor::ChildLaunchRegistry::resolve"]
  n25["tekes-supervisor::launch_failure"]
  n11 --> n15
  n12 --> n15
  n13 --> n15
  n14 --> n15
  n15 --> n1
  n15 --> n3
  n15 --> n8
  n15 --> n9
  n15 --> n16
  n16 --> n0
  n16 --> n2
  n16 --> n3
  n16 --> n4
  n16 --> n5
  n16 --> n6
  n16 --> n7
  n16 --> n9
  n16 --> n17
  n16 --> n18
  n16 --> n20
  n16 --> n21
  n16 --> n22
  n18 --> n19
  n24 --> n25
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `record` | `self.children.insert` | [67](../../src/lib.rs#L67) | receiver-type-required |
| `record` | `child.into` | [67](../../src/lib.rs#L67) | receiver-type-required |
| `record` | `spawn_id.into` | [67](../../src/lib.rs#L67) | receiver-type-required |
| `resolve` | `self.children.get` | [72](../../src/lib.rs#L72) | receiver-type-required |
| `resolve` | `launch_failure` | [73](../../src/lib.rs#L73), [75](../../src/lib.rs#L75) | [tekes-supervisor::launch_failure](../../src/lib.rs#L87) |
| `resolve` | `request.child.clone` | [78](../../src/lib.rs#L78) | receiver-type-required |
| `resolve` | `request.spawn_id.clone` | [79](../../src/lib.rs#L79) | receiver-type-required |
| `launch_failure` | `request.child.clone` | [89](../../src/lib.rs#L89) | receiver-type-required |
| `launch_failure` | `request.spawn_id.clone` | [90](../../src/lib.rs#L90) | receiver-type-required |
| `launch_failure` | `Some` | [92](../../src/lib.rs#L92) | external-constructor-callback-or-unresolved |
| `launch_failure` | `error.to_owned` | [92](../../src/lib.rs#L92) | receiver-type-required |
| `default` | `Vec::new` | [149](../../src/lib.rs#L149) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_with_bindings` | `launch_profiled_worker_inner` | [160](../../src/lib.rs#L160) | [tekes-supervisor::launch_profiled_worker_inner](../../src/lib.rs#L216) |
| `launch_profiled_worker_with_bindings` | `Some` | [160](../../src/lib.rs#L160) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_with_credentials` | `launch_profiled_worker_inner` | [168](../../src/lib.rs#L168) | [tekes-supervisor::launch_profiled_worker_inner](../../src/lib.rs#L216) |
| `launch_profiled_worker_with_credentials` | `Some` | [168](../../src/lib.rs#L168) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_with_credentials_and_provider_test_redirect` | `launch_profiled_worker_inner` | [181](../../src/lib.rs#L181) | [tekes-supervisor::launch_profiled_worker_inner](../../src/lib.rs#L216) |
| `launch_profiled_worker_with_credentials_and_provider_test_redirect` | `Some` | [184](../../src/lib.rs#L184), [188](../../src/lib.rs#L188) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_with_secret_store_and_binding_resolver` | `launch_profiled_worker_inner` | [205](../../src/lib.rs#L205) | [tekes-supervisor::launch_profiled_worker_inner](../../src/lib.rs#L216) |
| `launch_profiled_worker_with_secret_store_and_binding_resolver` | `Some` | [209](../../src/lib.rs#L209), [211](../../src/lib.rs#L211) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `spec         .ledger         .parent()         .ok_or` | [225](../../src/lib.rs#L225) | receiver-type-required |
| `launch_profiled_worker_inner` | `spec         .ledger         .parent` | [225](../../src/lib.rs#L225) | receiver-type-required |
| `launch_profiled_worker_inner` | `store::AssetStore::new` | [229](../../src/lib.rs#L229) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `launch_profiled_worker_inner` | `folder.join` | [229](../../src/lib.rs#L229) | receiver-type-required |
| `launch_profiled_worker_inner` | `LaunchProfile::resolve_and_publish_for_binding` | [230](../../src/lib.rs#L230) | [profile::instruction::LaunchProfile::resolve_and_publish_for_binding](../../../profile/src/instruction.rs#L131) |
| `launch_profiled_worker_inner` | `spec.folder_binding.as_deref` | [235](../../src/lib.rs#L235) | receiver-type-required |
| `launch_profiled_worker_inner` | `resolver(&profile.config, &profile.instruction)             .map_err` | [239](../../src/lib.rs#L239) | receiver-type-required |
| `launch_profiled_worker_inner` | `resolver` | [239](../../src/lib.rs#L239) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `WorkerLaunchBindings::default` | [241](../../src/lib.rs#L241) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `Err` | [243](../../src/lib.rs#L243) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `LaunchError::BindingResolution` | [243](../../src/lib.rs#L243) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `"launch bindings and a binding resolver are mutually exclusive".into` | [244](../../src/lib.rs#L244) | receiver-type-required |
| `launch_profiled_worker_inner` | `LaunchBindings::bind` | [249](../../src/lib.rs#L249) | [profile::launch::LaunchBindings::bind](../../../profile/src/launch.rs#L170) |
| `launch_profiled_worker_inner` | `spec.binary.clone` | [251](../../src/lib.rs#L251) | receiver-type-required |
| `launch_profiled_worker_inner` | `spec.ledger.clone` | [252](../../src/lib.rs#L252) | receiver-type-required |
| `launch_profiled_worker_inner` | `spec.timestamp.clone` | [253](../../src/lib.rs#L253) | receiver-type-required |
| `launch_profiled_worker_inner` | `spec.run_id.clone` | [254](../../src/lib.rs#L254) | receiver-type-required |
| `launch_profiled_worker_inner` | `spec.binary_attribution.clone` | [255](../../src/lib.rs#L255) | receiver-type-required |
| `launch_profiled_worker_inner` | `assets.root().to_path_buf` | [256](../../src/lib.rs#L256) | receiver-type-required |
| `launch_profiled_worker_inner` | `assets.root` | [256](../../src/lib.rs#L256) | receiver-type-required |
| `launch_profiled_worker_inner` | `resolve_config_credentials(&profile.config, store)             .map_err` | [262](../../src/lib.rs#L262) | receiver-type-required |
| `launch_profiled_worker_inner` | `resolve_config_credentials` | [262](../../src/lib.rs#L262) | [provider::secret_store::resolve_config_credentials](../../../provider/src/secret_store.rs#L294) |
| `launch_profiled_worker_inner` | `bindings.active.is_empty` | [264](../../src/lib.rs#L264) | receiver-type-required |
| `launch_profiled_worker_inner` | `bindings.revoked.is_empty` | [264](../../src/lib.rs#L264) | receiver-type-required |
| `launch_profiled_worker_inner` | `Some` | [267](../../src/lib.rs#L267), [269](../../src/lib.rs#L269), [283](../../src/lib.rs#L283) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `bindings.active.clone` | [267](../../src/lib.rs#L267) | receiver-type-required |
| `launch_profiled_worker_inner` | `bindings.revoked.clone` | [267](../../src/lib.rs#L267) | receiver-type-required |
| `launch_profiled_worker_inner` | `scopes.map` | [272](../../src/lib.rs#L272) | receiver-type-required |
| `launch_profiled_worker_inner` | `Vec::new` | [272](../../src/lib.rs#L272) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `credentials         .map(&#124;(scopes, revoked)&#124; {             let (supervisor, worker) = UnixStream::pair()?;             Ok::<_, io::Error>((supervisor, worker, scopes, revoked))         })         .transpose` | [274](../../src/lib.rs#L274) | receiver-type-required |
| `launch_profiled_worker_inner` | `credentials         .map` | [274](../../src/lib.rs#L274) | receiver-type-required |
| `launch_profiled_worker_inner` | `UnixStream::pair` | [276](../../src/lib.rs#L276) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `Ok::<_, io::Error>` | [277](../../src/lib.rs#L277) | external-constructor-callback-or-unresolved |
| `launch_profiled_worker_inner` | `launch_worker_inner` | [280](../../src/lib.rs#L280) | [tekes-supervisor::launch_worker_inner](../../src/lib.rs#L290) |
| `launch_profiled_worker_inner` | `Ok` | [287](../../src/lib.rs#L287) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `snapshot_path` | [301](../../src/lib.rs#L301), [302](../../src/lib.rs#L302), [333](../../src/lib.rs#L333) | [tekes-supervisor::snapshot_path](../../src/lib.rs#L470) |
| `launch_worker_inner` | `std::fs::read` | [306](../../src/lib.rs#L306), [307](../../src/lib.rs#L307), [334](../../src/lib.rs#L334) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `ConfigSnapshot::decode` | [308](../../src/lib.rs#L308) | [profile::config::ConfigSnapshot::decode](../../../profile/src/config.rs#L289) |
| `launch_worker_inner` | `InstructionSnapshot::decode` | [309](../../src/lib.rs#L309) | [profile::instruction::InstructionSnapshot::decode](../../../profile/src/instruction.rs#L183) |
| `launch_worker_inner` | `instruction.validate_against_config` | [310](../../src/lib.rs#L310) | receiver-type-required |
| `launch_worker_inner` | `config.digest` | [311](../../src/lib.rs#L311) | receiver-type-required |
| `launch_worker_inner` | `Err` | [312](../../src/lib.rs#L312), [315](../../src/lib.rs#L315) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `LaunchError::DigestMismatch` | [312](../../src/lib.rs#L312), [315](../../src/lib.rs#L315) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `instruction.digest` | [314](../../src/lib.rs#L314) | receiver-type-required |
| `launch_worker_inner` | `bindings.validate_against` | [319](../../src/lib.rs#L319) | receiver-type-required |
| `launch_worker_inner` | `bindings.clone` | [320](../../src/lib.rs#L320) | receiver-type-required |
| `launch_worker_inner` | `LaunchBindings::bind` | [322](../../src/lib.rs#L322) | [profile::launch::LaunchBindings::bind](../../../profile/src/launch.rs#L170) |
| `launch_worker_inner` | `Vec::new` | [327](../../src/lib.rs#L327) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `store::AssetStore::new` | [331](../../src/lib.rs#L331) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `launch_worker_inner` | `launch_bindings.publish` | [332](../../src/lib.rs#L332) | receiver-type-required |
| `launch_worker_inner` | `LaunchBindings::decode_verified` | [335](../../src/lib.rs#L335) | [profile::launch::LaunchBindings::decode_verified](../../../profile/src/launch.rs#L198) |
| `launch_worker_inner` | `open_snapshot` | [337](../../src/lib.rs#L337), [338](../../src/lib.rs#L338), [339](../../src/lib.rs#L339) | [tekes-supervisor::open_snapshot](../../src/lib.rs#L481) |
| `launch_worker_inner` | `config_file.as_raw_fd` | [341](../../src/lib.rs#L341) | receiver-type-required |
| `launch_worker_inner` | `instruction_file.as_raw_fd` | [342](../../src/lib.rs#L342) | receiver-type-required |
| `launch_worker_inner` | `launch_bindings_file.as_raw_fd` | [343](../../src/lib.rs#L343) | receiver-type-required |
| `launch_worker_inner` | `credential         .as_ref()         .map` | [344](../../src/lib.rs#L344) | receiver-type-required |
| `launch_worker_inner` | `credential         .as_ref` | [344](../../src/lib.rs#L344) | receiver-type-required |
| `launch_worker_inner` | `worker.as_raw_fd` | [346](../../src/lib.rs#L346) | receiver-type-required |
| `launch_worker_inner` | `config.providers.web_search.as_ref().is_some_and` | [347](../../src/lib.rs#L347) | receiver-type-required |
| `launch_worker_inner` | `config.providers.web_search.as_ref` | [347](../../src/lib.rs#L347) | receiver-type-required |
| `launch_worker_inner` | `provider::endpoint_origin` | [348](../../src/lib.rs#L348) | [provider::request::endpoint_origin](../../../provider/src/request.rs#L113) |
| `launch_worker_inner` | `credential.as_ref().is_some_and` | [351](../../src/lib.rs#L351) | receiver-type-required |
| `launch_worker_inner` | `credential.as_ref` | [351](../../src/lib.rs#L351) | receiver-type-required |
| `launch_worker_inner` | `active.iter().any` | [352](../../src/lib.rs#L352) | receiver-type-required |
| `launch_worker_inner` | `active.iter` | [352](../../src/lib.rs#L352) | receiver-type-required |
| `launch_worker_inner` | `closed_command` | [360](../../src/lib.rs#L360) | [tekes-supervisor::closed_command](../../src/lib.rs#L429) |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args(["--instruction-fd", &instruction_fd.to_string()])         .args(["--launch-bindings-fd", &launch_bindings_fd.to_string()])         .args(["--launch-bindings-digest", &launch_bindings_digest])         .stdin(Stdio::piped())         .stdout(Stdio::piped())         .stderr` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args(["--instruction-fd", &instruction_fd.to_string()])         .args(["--launch-bindings-fd", &launch_bindings_fd.to_string()])         .args(["--launch-bindings-digest", &launch_bindings_digest])         .stdin(Stdio::piped())         .stdout` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args(["--instruction-fd", &instruction_fd.to_string()])         .args(["--launch-bindings-fd", &launch_bindings_fd.to_string()])         .args(["--launch-bindings-digest", &launch_bindings_digest])         .stdin` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args(["--instruction-fd", &instruction_fd.to_string()])         .args(["--launch-bindings-fd", &launch_bindings_fd.to_string()])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args(["--instruction-fd", &instruction_fd.to_string()])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args(["--config-fd", &config_fd.to_string()])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args(["--binary", &spec.binary_attribution])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args(["--run-id", &spec.run_id])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args(["--timestamp", &spec.timestamp])         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg(&spec.ledger)         .args` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `command         .arg` | [361](../../src/lib.rs#L361) | receiver-type-required |
| `launch_worker_inner` | `config_fd.to_string` | [366](../../src/lib.rs#L366) | receiver-type-required |
| `launch_worker_inner` | `instruction_fd.to_string` | [367](../../src/lib.rs#L367) | receiver-type-required |
| `launch_worker_inner` | `launch_bindings_fd.to_string` | [368](../../src/lib.rs#L368) | receiver-type-required |
| `launch_worker_inner` | `Stdio::piped` | [370](../../src/lib.rs#L370), [371](../../src/lib.rs#L371), [372](../../src/lib.rs#L372) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `command.args` | [374](../../src/lib.rs#L374), [377](../../src/lib.rs#L377), [380](../../src/lib.rs#L380) | receiver-type-required |
| `launch_worker_inner` | `fd.to_string` | [374](../../src/lib.rs#L374) | receiver-type-required |
| `launch_worker_inner` | `forwarded_worker_environment` | [382](../../src/lib.rs#L382) | [tekes-supervisor::forwarded_worker_environment](../../src/lib.rs#L456) |
| `launch_worker_inner` | `command.env` | [383](../../src/lib.rs#L383) | receiver-type-required |
| `launch_worker_inner` | `command.pre_exec` | [390](../../src/lib.rs#L390) | receiver-type-required |
| `launch_worker_inner` | `clear_cloexec` | [391](../../src/lib.rs#L391), [392](../../src/lib.rs#L392), [393](../../src/lib.rs#L393), [395](../../src/lib.rs#L395) | [tekes-supervisor::clear_cloexec](../../src/lib.rs#L488) |
| `launch_worker_inner` | `Ok` | [397](../../src/lib.rs#L397), [416](../../src/lib.rs#L416) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `command.spawn` | [400](../../src/lib.rs#L400) | receiver-type-required |
| `launch_worker_inner` | `credential.map_or` | [402](../../src/lib.rs#L402) | receiver-type-required |
| `launch_worker_inner` | `drop` | [403](../../src/lib.rs#L403) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `start_credential_channel` | [404](../../src/lib.rs#L404) | [provider::credential::start_credential_channel](../../../provider/src/credential.rs#L519) |
| `launch_worker_inner` | `CredentialBroker::with_revoked` | [406](../../src/lib.rs#L406) | [provider::credential::CredentialBroker::with_revoked](../../../provider/src/credential.rs#L360) |
| `launch_worker_inner` | `std::thread::spawn` | [408](../../src/lib.rs#L408) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `service                     .join()                     .map_err(&#124;_&#124; "credential broker service panicked".to_owned())?                     .map_err` | [409](../../src/lib.rs#L409) | receiver-type-required |
| `launch_worker_inner` | `service                     .join()                     .map_err` | [409](../../src/lib.rs#L409) | receiver-type-required |
| `launch_worker_inner` | `service                     .join` | [409](../../src/lib.rs#L409) | receiver-type-required |
| `launch_worker_inner` | `"credential broker service panicked".to_owned` | [411](../../src/lib.rs#L411) | receiver-type-required |
| `launch_worker_inner` | `error.to_string` | [412](../../src/lib.rs#L412) | receiver-type-required |
| `launch_worker_inner` | `Some` | [414](../../src/lib.rs#L414) | external-constructor-callback-or-unresolved |
| `launch_worker_inner` | `spec.config_digest.clone` | [420](../../src/lib.rs#L420) | receiver-type-required |
| `launch_worker_inner` | `spec.instruction_digest.clone` | [421](../../src/lib.rs#L421) | receiver-type-required |
| `closed_command` | `Command::new` | [430](../../src/lib.rs#L430) | external-constructor-callback-or-unresolved |
| `closed_command` | `command.env_clear` | [431](../../src/lib.rs#L431) | receiver-type-required |
| `forwarded_worker_environment` | `forwarded_environment_from` | [457](../../src/lib.rs#L457) | [tekes-supervisor::forwarded_environment_from](../../src/lib.rs#L463) |
| `forwarded_worker_environment` | `std::env::var(name).ok` | [457](../../src/lib.rs#L457) | receiver-type-required |
| `forwarded_worker_environment` | `std::env::var` | [457](../../src/lib.rs#L457) | external-constructor-callback-or-unresolved |
| `forwarded_environment_from` | `std::iter::once("TEKES_KERNEL_LIVE_ARTIFACT")         .chain(FORWARDED_PROXY_VARIABLES)         .filter_map(&#124;name&#124; lookup(name).map(&#124;value&#124; (name.to_owned(), value)))         .collect` | [464](../../src/lib.rs#L464) | receiver-type-required |
| `forwarded_environment_from` | `std::iter::once("TEKES_KERNEL_LIVE_ARTIFACT")         .chain(FORWARDED_PROXY_VARIABLES)         .filter_map` | [464](../../src/lib.rs#L464) | receiver-type-required |
| `forwarded_environment_from` | `std::iter::once("TEKES_KERNEL_LIVE_ARTIFACT")         .chain` | [464](../../src/lib.rs#L464) | receiver-type-required |
| `forwarded_environment_from` | `std::iter::once` | [464](../../src/lib.rs#L464) | external-constructor-callback-or-unresolved |
| `forwarded_environment_from` | `lookup(name).map` | [466](../../src/lib.rs#L466) | receiver-type-required |
| `forwarded_environment_from` | `lookup` | [466](../../src/lib.rs#L466) | external-constructor-callback-or-unresolved |
| `forwarded_environment_from` | `name.to_owned` | [466](../../src/lib.rs#L466) | receiver-type-required |
| `snapshot_path` | `digest.len` | [471](../../src/lib.rs#L471) | receiver-type-required |
| `snapshot_path` | `digest             .bytes()             .all` | [472](../../src/lib.rs#L472) | receiver-type-required |
| `snapshot_path` | `digest             .bytes` | [472](../../src/lib.rs#L472) | receiver-type-required |
| `snapshot_path` | `byte.is_ascii_hexdigit` | [474](../../src/lib.rs#L474) | receiver-type-required |
| `snapshot_path` | `byte.is_ascii_uppercase` | [474](../../src/lib.rs#L474) | receiver-type-required |
| `snapshot_path` | `Err` | [476](../../src/lib.rs#L476) | external-constructor-callback-or-unresolved |
| `snapshot_path` | `Ok` | [478](../../src/lib.rs#L478) | external-constructor-callback-or-unresolved |
| `snapshot_path` | `assets.join` | [478](../../src/lib.rs#L478) | receiver-type-required |
| `open_snapshot` | `Ok` | [482](../../src/lib.rs#L482) | external-constructor-callback-or-unresolved |
| `open_snapshot` | `OpenOptions::new()         .read(true)         .custom_flags(libc::O_CLOEXEC &#124; libc::O_NOFOLLOW)         .open` | [482](../../src/lib.rs#L482) | receiver-type-required |
| `open_snapshot` | `OpenOptions::new()         .read(true)         .custom_flags` | [482](../../src/lib.rs#L482) | receiver-type-required |
| `open_snapshot` | `OpenOptions::new()         .read` | [482](../../src/lib.rs#L482) | receiver-type-required |
| `open_snapshot` | `OpenOptions::new` | [482](../../src/lib.rs#L482) | external-constructor-callback-or-unresolved |
| `clear_cloexec` | `libc::fcntl` | [491](../../src/lib.rs#L491) | external-constructor-callback-or-unresolved |
| `clear_cloexec` | `Err` | [492](../../src/lib.rs#L492) | external-constructor-callback-or-unresolved |
| `clear_cloexec` | `io::Error::last_os_error` | [492](../../src/lib.rs#L492) | external-constructor-callback-or-unresolved |
| `clear_cloexec` | `Ok` | [494](../../src/lib.rs#L494) | external-constructor-callback-or-unresolved |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `std::collections::BTreeMap::from` | [528](../../src/lib.rs#L528) | external-constructor-callback-or-unresolved |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `closed_command` | [547](../../src/lib.rs#L547) | [tekes-supervisor::closed_command](../../src/lib.rs#L429) |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `std::path::Path::new` | [547](../../src/lib.rs#L547) | external-constructor-callback-or-unresolved |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `command.env` | [548](../../src/lib.rs#L548) | receiver-type-required |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `command.output().expect` | [549](../../src/lib.rs#L549) | receiver-type-required |
| `worker_environment_forwards_only_the_evidence_capture_directory` | `command.output` | [549](../../src/lib.rs#L549) | receiver-type-required |
| `worker_command_drops_the_complete_ambient_environment` | `closed_command(std::path::Path::new("/usr/bin/env"))             .output()             .expect` | [558](../../src/lib.rs#L558) | receiver-type-required |
| `worker_command_drops_the_complete_ambient_environment` | `closed_command(std::path::Path::new("/usr/bin/env"))             .output` | [558](../../src/lib.rs#L558) | receiver-type-required |
| `worker_command_drops_the_complete_ambient_environment` | `closed_command` | [558](../../src/lib.rs#L558) | [tekes-supervisor::closed_command](../../src/lib.rs#L429) |
| `worker_command_drops_the_complete_ambient_environment` | `std::path::Path::new` | [558](../../src/lib.rs#L558) | external-constructor-callback-or-unresolved |
