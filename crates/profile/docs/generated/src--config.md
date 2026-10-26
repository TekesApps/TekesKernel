# profile::config

[Package atlas](index.md) · [Source](../../src/config.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [profile::config::FORMAT](../../src/config.rs#L14) | const_item | `private` |  |
| [profile::config::MAX_SAFE_INTEGER](../../src/config.rs#L15) | const_item | `private` |  |
| [profile::config::ExecutionPolicy](../../src/config.rs#L19) | struct_item | `pub` |  |
| [profile::config::Model](../../src/config.rs#L37) | struct_item | `pub` |  |
| [profile::config::Provider](../../src/config.rs#L47) | struct_item | `pub` |  |
| [profile::config::WebSearch](../../src/config.rs#L66) | struct_item | `pub` |  |
| [profile::config::ProvidersConfig](../../src/config.rs#L74) | struct_item | `pub` |  |
| [profile::config::ProvidersConfig::default](../../src/config.rs#L83) | function_item | `private` |  |
| [profile::config::Limits](../../src/config.rs#L95) | struct_item | `pub` |  |
| [profile::config::SettingsConfig](../../src/config.rs#L104) | struct_item | `pub` |  |
| [profile::config::SessionSettings](../../src/config.rs#L117) | struct_item | `pub` |  |
| [profile::config::SettingsConfig::default](../../src/config.rs#L127) | function_item | `private` |  |
| [profile::config::ResolvedExecution](../../src/config.rs#L140) | struct_item | `pub` |  |
| [profile::config::RevisionVector](../../src/config.rs#L148) | struct_item | `pub` |  |
| [profile::config::ConfigSnapshot](../../src/config.rs#L157) | struct_item | `pub` |  |
| [profile::config::ConfigSnapshot::protected_roots](../../src/config.rs#L173) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::execution_cwd](../../src/config.rs#L181) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L185) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::digest](../../src/config.rs#L190) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::decode](../../src/config.rs#L194) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::publish](../../src/config.rs#L206) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::requires_respawn_from](../../src/config.rs#L220) | function_item | `pub` |  |
| [profile::config::config_document](../../src/config.rs#L244) | macro_definition | `private` |  |
| [profile::config::ConfigRepository](../../src/config.rs#L270) | struct_item | `pub` |  |
| [profile::config::invalid_private_directory](../../src/config.rs#L274) | function_item | `private` |  |
| [profile::config::validate_private_directory_fd](../../src/config.rs#L281) | function_item | `private` |  |
| [profile::config::rustix_profile_error](../../src/config.rs#L311) | function_item | `private` |  |
| [profile::config::open_directory_at](../../src/config.rs#L315) | function_item | `private` |  |
| [profile::config::open_private_root](../../src/config.rs#L349) | function_item | `private` |  |
| [profile::config::open_private_root_darwin](../../src/config.rs#L446) | function_item | `private` | #[cfg(target_os = "macos")] |
| [profile::config::open_private_root_sandboxed](../../src/config.rs#L511) | function_item | `private` | #[cfg(target_os = "macos")] |
| [profile::config::prepare_repository_directories](../../src/config.rs#L529) | function_item | `private` |  |
| [profile::config::ConfigRepository::open](../../src/config.rs#L543) | function_item | `pub` |  |
| [profile::config::ConfigRepository::root](../../src/config.rs#L550) | function_item | `pub` |  |
| [profile::config::ConfigRepository::providers](../../src/config.rs#L556) | function_item | `pub` |  |
| [profile::config::ConfigRepository::settings](../../src/config.rs#L565) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_providers](../../src/config.rs#L573) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_providers_checked](../../src/config.rs#L588) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_settings](../../src/config.rs#L619) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_settings_checked](../../src/config.rs#L632) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_session_settings](../../src/config.rs#L655) | function_item | `pub` |  |
| [profile::config::ConfigRepository::session_settings](../../src/config.rs#L682) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_global](../../src/config.rs#L695) | function_item | `private` |  |
| [profile::config::ConfigRepository::resolve_execution_snapshot](../../src/config.rs#L719) | function_item | `pub(crate)` |  |
| [profile::config::ConfigRepository::read_optional](../../src/config.rs#L777) | function_item | `private` |  |
| [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) | function_item | `private` |  |
| [profile::config::ConfigRepository::validate_all_provider_references_unlocked](../../src/config.rs#L799) | function_item | `private` |  |
| [profile::config::check_publication_revision](../../src/config.rs#L808) | function_item | `private` |  |
| [profile::config::revision_from_value](../../src/config.rs#L830) | function_item | `private` |  |
| [profile::config::revision_from_value::RevisionOnly](../../src/config.rs#L832) | struct_item | `private` |  |
| [profile::config::validate_common](../../src/config.rs#L847) | function_item | `private` |  |
| [profile::config::validate_providers](../../src/config.rs#L869) | function_item | `private` |  |
| [profile::config::validate_web_search_origin](../../src/config.rs#L931) | function_item | `private` |  |
| [profile::config::validate_settings](../../src/config.rs#L966) | function_item | `private` |  |
| [profile::config::validate_session_settings](../../src/config.rs#L988) | function_item | `private` |  |
| [profile::config::validate_session_reference](../../src/config.rs#L1002) | function_item | `private` |  |
| [profile::config::validate_references](../../src/config.rs#L1028) | function_item | `private` |  |
| [profile::config::session_preset_tools](../../src/config.rs#L1060) | function_item | `private` |  |
| [profile::config::validate_snapshot](../../src/config.rs#L1084) | function_item | `private` |  |
| [profile::config::validate_safe_integer](../../src/config.rs#L1176) | function_item | `private` |  |
| [profile::config::is_sorted_unique](../../src/config.rs#L1187) | function_item | `private` |  |
| [profile::config::canonical_directory](../../src/config.rs#L1191) | function_item | `private` |  |
| [profile::config::validate_absolute](../../src/config.rs#L1212) | function_item | `private` |  |
| [profile::config::validate_id](../../src/config.rs#L1222) | function_item | `private` |  |
| [profile::config::validate_unique](../../src/config.rs#L1233) | function_item | `private` |  |
| [profile::config::invalid](../../src/config.rs#L1242) | function_item | `private` |  |
| [profile::config::invalid_reference](../../src/config.rs#L1249) | function_item | `private` |  |
| [profile::config::removed](../../src/config.rs#L1256) | function_item | `private` |  |
| [profile::config::lowered](../../src/config.rs#L1261) | function_item | `private` |  |
| [profile::config::enabled_provider_models](../../src/config.rs#L1269) | function_item | `private` |  |
| [profile::config::private_directory_tests::private_directory_rejects_wrong_owner](../../src/config.rs#L1294) | function_item | `private` | test; #[cfg(test)] |
| [profile::config::private_directory_tests::sandbox_fallback_rejects_a_symlinked_path](../../src/config.rs#L1306) | function_item | `private` | test; #[cfg(test)] #[cfg(target_os = "macos")] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `fs` | `std::fs` | `private` |
| `OwnedFd` | `std::os::fd::OwnedFd` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Mode` | `rustix::fs::Mode` | `private` |
| `OFlags` | `rustix::fs::OFlags` | `private` |
| `fchmod` | `rustix::fs::fchmod` | `private` |
| `fstat` | `rustix::fs::fstat` | `private` |
| `mkdirat` | `rustix::fs::mkdirat` | `private` |
| `open` | `rustix::fs::open` | `private` |
| `openat` | `rustix::fs::openat` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `AssetRef` | `store::AssetRef` | `private` |
| `AssetStore` | `store::AssetStore` | `private` |
| `AtomicPublisher` | `store::AtomicPublisher` | `private` |
| `NamedLock` | `store::NamedLock` | `private` |
| `ProfileError` | `crate::ProfileError` | `private` |
| `canonical_line` | `crate::canonical_line` | `private` |
| `digest` | `crate::digest` | `private` |
| `parse_canonical` | `crate::parse_canonical` | `private` |
| `parse_global_json` | `crate::parse_global_json` | `private` |
| `open_private_root` | `super::open_private_root` | `private` |
| `validate_private_directory_fd` | `super::validate_private_directory_fd` | `private` |
| `open_private_root_sandboxed` | `super::open_private_root_sandboxed` | `private` |
| `OFlags` | `rustix::fs::OFlags` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `profile::config::private_directory_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 32 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::validate_snapshot"]
  n1["profile::config::removed"]
  n2["profile::config::lowered"]
  n3["profile::config::enabled_provider_models"]
  n4["profile::config::SettingsConfig::default"]
  n5["profile::config::ConfigSnapshot::protected_roots"]
  n6["profile::config::ConfigSnapshot::execution_cwd"]
  n7["profile::config::ConfigSnapshot::canonical_bytes"]
  n8["profile::config::ConfigSnapshot::digest"]
  n9["profile::config::ConfigSnapshot::decode"]
  n10["profile::config::ConfigSnapshot::publish"]
  n11["profile::config::ConfigSnapshot::requires_respawn_from"]
  n12["profile::config::invalid_private_directory"]
  n13["profile::config::validate_private_directory_fd"]
  n14["profile::config::rustix_profile_error"]
  n15["profile::config::open_directory_at"]
  n16["profile::config::open_private_root"]
  n17["profile::config::open_private_root_darwin"]
  n18["profile::config::open_private_root_sandboxed"]
  n19["profile::config::prepare_repository_directories"]
  n20["profile::config::ConfigRepository::open"]
  n21["profile::config::ConfigRepository::root"]
  n22["profile::config::ConfigRepository::providers"]
  n23["profile::config::ConfigRepository::read_global_optional"]
  n24["profile::config::ProvidersConfig::default"]
  n25["profile::config::validate_providers"]
  n26["profile::canonical_line"]
  n27["profile::digest"]
  n28["profile::parse_canonical"]
  n29["store::platform::NamedLock::shared"]
  n30["tools::sandbox::kernel_protected_roots"]
  n31["tools::sandbox::kernel_protected_roots_with_user_agent_dir"]
  n5 --> n30
  n5 --> n31
  n7 --> n0
  n7 --> n26
  n8 --> n7
  n8 --> n27
  n9 --> n0
  n9 --> n28
  n10 --> n7
  n11 --> n1
  n11 --> n2
  n11 --> n3
  n13 --> n12
  n15 --> n12
  n15 --> n13
  n15 --> n14
  n16 --> n12
  n16 --> n13
  n16 --> n14
  n16 --> n17
  n16 --> n18
  n17 --> n12
  n17 --> n13
  n17 --> n14
  n18 --> n12
  n18 --> n13
  n19 --> n15
  n19 --> n16
  n20 --> n19
  n22 --> n23
  n22 --> n25
  n22 --> n29
```

</details>

<details><summary>Functions 21–40: 66 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::validate_session_reference"]
  n1["profile::config::validate_references"]
  n2["profile::config::validate_snapshot"]
  n3["profile::config::validate_safe_integer"]
  n4["profile::config::validate_id"]
  n5["profile::config::invalid"]
  n6["profile::config::invalid_reference"]
  n7["profile::config::ConfigRepository::settings"]
  n8["profile::config::ConfigRepository::publish_providers"]
  n9["profile::config::ConfigRepository::publish_providers_checked"]
  n10["profile::config::ConfigRepository::publish_settings"]
  n11["profile::config::ConfigRepository::publish_settings_checked"]
  n12["profile::config::ConfigRepository::publish_session_settings"]
  n13["profile::config::ConfigRepository::session_settings"]
  n14["profile::config::ConfigRepository::publish_global"]
  n15["profile::config::ConfigRepository::resolve_execution_snapshot"]
  n16["profile::config::ConfigRepository::read_optional"]
  n17["profile::config::ConfigRepository::read_global_optional"]
  n18["profile::config::ConfigRepository::validate_all_provider_references_unlocked"]
  n19["profile::config::check_publication_revision"]
  n20["profile::config::revision_from_value"]
  n21["profile::config::validate_common"]
  n22["profile::config::validate_providers"]
  n23["profile::config::validate_web_search_origin"]
  n24["profile::config::validate_settings"]
  n25["profile::config::validate_session_settings"]
  n26["profile::parse_global_json"]
  n27["profile::canonical_line"]
  n28["profile::parse_canonical"]
  n29["store::atomic::AtomicPublisher::replace"]
  n30["store::management_root::session_settings_path"]
  n31["store::platform::NamedLock::exclusive"]
  n32["store::platform::NamedLock::shared"]
  n33["tools::runtime_backends::is_public_internet_address"]
  n0 --> n6
  n7 --> n17
  n7 --> n24
  n7 --> n32
  n8 --> n14
  n8 --> n22
  n9 --> n17
  n9 --> n18
  n9 --> n19
  n9 --> n22
  n9 --> n27
  n9 --> n29
  n9 --> n31
  n9 --> n32
  n10 --> n14
  n10 --> n24
  n11 --> n1
  n11 --> n17
  n11 --> n19
  n11 --> n22
  n11 --> n24
  n11 --> n27
  n11 --> n29
  n11 --> n31
  n12 --> n0
  n12 --> n16
  n12 --> n17
  n12 --> n19
  n12 --> n22
  n12 --> n25
  n12 --> n27
  n12 --> n29
  n12 --> n30
  n12 --> n31
  n13 --> n16
  n13 --> n25
  n13 --> n30
  n13 --> n32
  n14 --> n19
  n14 --> n20
  n14 --> n26
  n14 --> n27
  n14 --> n29
  n14 --> n31
  n15 --> n2
  n15 --> n16
  n15 --> n17
  n15 --> n32
  n16 --> n28
  n17 --> n26
  n18 --> n1
  n19 --> n3
  n21 --> n3
  n22 --> n3
  n22 --> n4
  n22 --> n5
  n22 --> n21
  n22 --> n23
  n23 --> n5
  n23 --> n33
  n24 --> n3
  n24 --> n5
  n24 --> n21
  n25 --> n4
  n25 --> n5
  n25 --> n21
```

</details>

<details><summary>Functions 41–54: 19 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::validate_session_reference"]
  n1["profile::config::validate_references"]
  n2["profile::config::session_preset_tools"]
  n3["profile::config::validate_snapshot"]
  n4["profile::config::validate_safe_integer"]
  n5["profile::config::is_sorted_unique"]
  n6["profile::config::canonical_directory"]
  n7["profile::config::validate_absolute"]
  n8["profile::config::validate_id"]
  n9["profile::config::validate_unique"]
  n10["profile::config::invalid"]
  n11["profile::config::invalid_reference"]
  n12["profile::config::removed"]
  n13["profile::config::lowered"]
  n14["profile::config::enabled_provider_models"]
  n15["profile::config::validate_providers"]
  n16["profile::config::validate_settings"]
  n17["profile::config::validate_session_settings"]
  n18["store::tail::read_genesis"]
  n19["tools::preset::ResolvedPreset::from_genesis"]
  n1 --> n11
  n2 --> n18
  n2 --> n19
  n3 --> n0
  n3 --> n1
  n3 --> n4
  n3 --> n5
  n3 --> n6
  n3 --> n7
  n3 --> n8
  n3 --> n9
  n3 --> n10
  n3 --> n15
  n3 --> n16
  n3 --> n17
  n4 --> n10
  n6 --> n7
  n8 --> n10
  n9 --> n10
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `default` | `Vec::new` | [87](../../src/config.rs#L87) | external-constructor-callback-or-unresolved |
| `protected_roots` | `self.user_agent_dir.as_deref().map_or_else` | [174](../../src/config.rs#L174) | receiver-type-required |
| `protected_roots` | `self.user_agent_dir.as_deref` | [174](../../src/config.rs#L174) | receiver-type-required |
| `protected_roots` | `tools::kernel_protected_roots` | [175](../../src/config.rs#L175) | [tools::sandbox::kernel_protected_roots](../../../tools/src/sandbox.rs#L179) |
| `protected_roots` | `tools::kernel_protected_roots_with_user_agent_dir` | [176](../../src/config.rs#L176) | [tools::sandbox::kernel_protected_roots_with_user_agent_dir](../../../tools/src/sandbox.rs#L200) |
| `protected_roots` | `Path::new` | [176](../../src/config.rs#L176) | external-constructor-callback-or-unresolved |
| `execution_cwd` | `self.execution.cwd.first().map` | [182](../../src/config.rs#L182) | receiver-type-required |
| `execution_cwd` | `self.execution.cwd.first` | [182](../../src/config.rs#L182) | receiver-type-required |
| `canonical_bytes` | `validate_snapshot` | [186](../../src/config.rs#L186) | [profile::config::validate_snapshot](../../src/config.rs#L1084) |
| `canonical_bytes` | `canonical_line` | [187](../../src/config.rs#L187) | [profile::canonical_line](../../src/lib.rs#L138) |
| `digest` | `Ok` | [191](../../src/config.rs#L191) | external-constructor-callback-or-unresolved |
| `digest` | `digest` | [191](../../src/config.rs#L191) | [profile::digest](../../src/lib.rs#L152) |
| `digest` | `self.canonical_bytes` | [191](../../src/config.rs#L191) | [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L185) |
| `decode` | `parse_canonical` | [195](../../src/config.rs#L195) | [profile::parse_canonical](../../src/lib.rs#L69) |
| `decode` | `Err` | [197](../../src/config.rs#L197) | external-constructor-callback-or-unresolved |
| `decode` | `PathBuf::from` | [198](../../src/config.rs#L198) | external-constructor-callback-or-unresolved |
| `decode` | `validate_snapshot` | [202](../../src/config.rs#L202) | [profile::config::validate_snapshot](../../src/config.rs#L1084) |
| `decode` | `Ok` | [203](../../src/config.rs#L203) | external-constructor-callback-or-unresolved |
| `publish` | `self.canonical_bytes` | [207](../../src/config.rs#L207) | [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L185) |
| `publish` | `digest` | [208](../../src/config.rs#L208) | external-constructor-callback-or-unresolved |
| `publish` | `assets.publish` | [209](../../src/config.rs#L209) | receiver-type-required |
| `publish` | `Err` | [211](../../src/config.rs#L211) | external-constructor-callback-or-unresolved |
| `publish` | `Ok` | [216](../../src/config.rs#L216) | external-constructor-callback-or-unresolved |
| `requires_respawn_from` | `removed` | [224](../../src/config.rs#L224), [226](../../src/config.rs#L226) | [profile::config::removed](../../src/config.rs#L1256) |
| `requires_respawn_from` | `lowered` | [227](../../src/config.rs#L227) | [profile::config::lowered](../../src/config.rs#L1261) |
| `requires_respawn_from` | `enabled_provider_models` | [233](../../src/config.rs#L233), [234](../../src/config.rs#L234) | [profile::config::enabled_provider_models](../../src/config.rs#L1269) |
| `requires_respawn_from` | `old_providers.is_subset` | [235](../../src/config.rs#L235) | receiver-type-required |
| `requires_respawn_from` | `previous                 .providers                 .web_search                 .as_ref()                 .is_some_and` | [236](../../src/config.rs#L236) | receiver-type-required |
| `requires_respawn_from` | `previous                 .providers                 .web_search                 .as_ref` | [236](../../src/config.rs#L236) | receiver-type-required |
| `requires_respawn_from` | `self.providers.web_search.as_ref` | [240](../../src/config.rs#L240) | receiver-type-required |
| `requires_respawn_from` | `Some` | [240](../../src/config.rs#L240) | external-constructor-callback-or-unresolved |
| `invalid_private_directory` | `path.to_path_buf` | [276](../../src/config.rs#L276) | receiver-type-required |
| `invalid_private_directory` | `reason.to_owned` | [277](../../src/config.rs#L277) | receiver-type-required |
| `validate_private_directory_fd` | `fstat(descriptor).map_err` | [286](../../src/config.rs#L286) | receiver-type-required |
| `validate_private_directory_fd` | `fstat` | [286](../../src/config.rs#L286) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `Err` | [288](../../src/config.rs#L288), [294](../../src/config.rs#L294), [300](../../src/config.rs#L300) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `invalid_private_directory` | [288](../../src/config.rs#L288), [294](../../src/config.rs#L294), [300](../../src/config.rs#L300) | [profile::config::invalid_private_directory](../../src/config.rs#L274) |
| `validate_private_directory_fd` | `fchmod(descriptor, Mode::RWXU).map_err` | [307](../../src/config.rs#L307) | receiver-type-required |
| `validate_private_directory_fd` | `fchmod` | [307](../../src/config.rs#L307) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `Ok` | [308](../../src/config.rs#L308) | external-constructor-callback-or-unresolved |
| `rustix_profile_error` | `std::io::Error::from_raw_os_error(error.raw_os_error()).into` | [312](../../src/config.rs#L312) | receiver-type-required |
| `rustix_profile_error` | `std::io::Error::from_raw_os_error` | [312](../../src/config.rs#L312) | external-constructor-callback-or-unresolved |
| `rustix_profile_error` | `error.raw_os_error` | [312](../../src/config.rs#L312) | receiver-type-required |
| `open_directory_at` | `openat` | [323](../../src/config.rs#L323), [333](../../src/config.rs#L333) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `Mode::empty` | [323](../../src/config.rs#L323), [333](../../src/config.rs#L333) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `result         .as_ref()         .is_err_and` | [324](../../src/config.rs#L324) | receiver-type-required |
| `open_directory_at` | `result         .as_ref` | [324](../../src/config.rs#L324) | receiver-type-required |
| `open_directory_at` | `mkdirat` | [329](../../src/config.rs#L329) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `Err` | [331](../../src/config.rs#L331), [338](../../src/config.rs#L338), [343](../../src/config.rs#L343) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `rustix_profile_error` | [331](../../src/config.rs#L331), [343](../../src/config.rs#L343) | [profile::config::rustix_profile_error](../../src/config.rs#L311) |
| `open_directory_at` | `invalid_private_directory` | [338](../../src/config.rs#L338) | [profile::config::invalid_private_directory](../../src/config.rs#L274) |
| `open_directory_at` | `validate_private_directory_fd` | [345](../../src/config.rs#L345) | [profile::config::validate_private_directory_fd](../../src/config.rs#L281) |
| `open_directory_at` | `Ok` | [346](../../src/config.rs#L346) | external-constructor-callback-or-unresolved |
| `open_private_root` | `path.is_absolute` | [355](../../src/config.rs#L355), [366](../../src/config.rs#L366), [403](../../src/config.rs#L403) | receiver-type-required |
| `open_private_root` | `open_private_root_darwin` | [356](../../src/config.rs#L356) | [profile::config::open_private_root_darwin](../../src/config.rs#L446) |
| `open_private_root` | `path.strip_prefix("/var").map_or_else` | [359](../../src/config.rs#L359) | receiver-type-required |
| `open_private_root` | `path.strip_prefix` | [359](../../src/config.rs#L359) | receiver-type-required |
| `open_private_root` | `path.to_path_buf` | [360](../../src/config.rs#L360), [364](../../src/config.rs#L364) | receiver-type-required |
| `open_private_root` | `Path::new("/private/var").join` | [361](../../src/config.rs#L361) | receiver-type-required |
| `open_private_root` | `Path::new` | [361](../../src/config.rs#L361) | external-constructor-callback-or-unresolved |
| `open_private_root` | `open` | [367](../../src/config.rs#L367), [376](../../src/config.rs#L376) | external-constructor-callback-or-unresolved |
| `open_private_root` | `Mode::empty` | [367](../../src/config.rs#L367), [376](../../src/config.rs#L376), [411](../../src/config.rs#L411), [422](../../src/config.rs#L422) | external-constructor-callback-or-unresolved |
| `open_private_root` | `open_private_root_sandboxed` | [371](../../src/config.rs#L371), [428](../../src/config.rs#L428) | [profile::config::open_private_root_sandboxed](../../src/config.rs#L511) |
| `open_private_root` | `Err` | [373](../../src/config.rs#L373), [389](../../src/config.rs#L389), [397](../../src/config.rs#L397), [420](../../src/config.rs#L420), [433](../../src/config.rs#L433), [438](../../src/config.rs#L438) | external-constructor-callback-or-unresolved |
| `open_private_root` | `rustix_profile_error` | [373](../../src/config.rs#L373), [420](../../src/config.rs#L420), [438](../../src/config.rs#L438) | [profile::config::rustix_profile_error](../../src/config.rs#L311) |
| `open_private_root` | `open(".", flags, Mode::empty()).map_err` | [376](../../src/config.rs#L376) | receiver-type-required |
| `open_private_root` | `Vec::new` | [378](../../src/config.rs#L378) | external-constructor-callback-or-unresolved |
| `open_private_root` | `traversal_path.components` | [379](../../src/config.rs#L379) | receiver-type-required |
| `open_private_root` | `value.to_str().ok_or_else` | [383](../../src/config.rs#L383) | receiver-type-required |
| `open_private_root` | `value.to_str` | [383](../../src/config.rs#L383) | receiver-type-required |
| `open_private_root` | `invalid_private_directory` | [384](../../src/config.rs#L384), [389](../../src/config.rs#L389), [397](../../src/config.rs#L397), [433](../../src/config.rs#L433) | [profile::config::invalid_private_directory](../../src/config.rs#L274) |
| `open_private_root` | `components.push` | [386](../../src/config.rs#L386) | receiver-type-required |
| `open_private_root` | `value.to_owned` | [386](../../src/config.rs#L386) | receiver-type-required |
| `open_private_root` | `components.is_empty` | [396](../../src/config.rs#L396) | receiver-type-required |
| `open_private_root` | `PathBuf::from` | [404](../../src/config.rs#L404), [406](../../src/config.rs#L406) | external-constructor-callback-or-unresolved |
| `open_private_root` | `components.len` | [408](../../src/config.rs#L408) | receiver-type-required |
| `open_private_root` | `components.into_iter().enumerate` | [409](../../src/config.rs#L409) | receiver-type-required |
| `open_private_root` | `components.into_iter` | [409](../../src/config.rs#L409) | receiver-type-required |
| `open_private_root` | `traversed.push` | [410](../../src/config.rs#L410) | receiver-type-required |
| `open_private_root` | `openat` | [411](../../src/config.rs#L411), [422](../../src/config.rs#L422) | external-constructor-callback-or-unresolved |
| `open_private_root` | `result             .as_ref()             .is_err_and` | [412](../../src/config.rs#L412) | receiver-type-required |
| `open_private_root` | `result             .as_ref` | [412](../../src/config.rs#L412) | receiver-type-required |
| `open_private_root` | `mkdirat` | [418](../../src/config.rs#L418) | external-constructor-callback-or-unresolved |
| `open_private_root` | `validate_private_directory_fd` | [441](../../src/config.rs#L441) | [profile::config::validate_private_directory_fd](../../src/config.rs#L281) |
| `open_private_root` | `Ok` | [442](../../src/config.rs#L442) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `path.strip_prefix("/var").map_or_else` | [451](../../src/config.rs#L451) | receiver-type-required |
| `open_private_root_darwin` | `path.strip_prefix` | [451](../../src/config.rs#L451) | receiver-type-required |
| `open_private_root_darwin` | `path.to_path_buf` | [452](../../src/config.rs#L452) | receiver-type-required |
| `open_private_root_darwin` | `Path::new("/private/var").join` | [453](../../src/config.rs#L453) | receiver-type-required |
| `open_private_root_darwin` | `Path::new` | [453](../../src/config.rs#L453) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `path.components().any` | [455](../../src/config.rs#L455) | receiver-type-required |
| `open_private_root_darwin` | `path.components` | [455](../../src/config.rs#L455) | receiver-type-required |
| `open_private_root_darwin` | `Err` | [461](../../src/config.rs#L461), [467](../../src/config.rs#L467), [494](../../src/config.rs#L494), [499](../../src/config.rs#L499), [504](../../src/config.rs#L504) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `invalid_private_directory` | [461](../../src/config.rs#L461), [467](../../src/config.rs#L467), [486](../../src/config.rs#L486), [491](../../src/config.rs#L491), [499](../../src/config.rs#L499) | [profile::config::invalid_private_directory](../../src/config.rs#L274) |
| `open_private_root_darwin` | `path.file_name().is_none` | [466](../../src/config.rs#L466) | receiver-type-required |
| `open_private_root_darwin` | `path.file_name` | [466](../../src/config.rs#L466) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read(true)             .custom_flags(libc::O_DIRECTORY &#124; libc::O_NOFOLLOW_ANY &#124; libc::O_CLOEXEC)             .open` | [476](../../src/config.rs#L476) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read(true)             .custom_flags` | [476](../../src/config.rs#L476) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read` | [476](../../src/config.rs#L476) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new` | [476](../../src/config.rs#L476) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `Ok` | [480](../../src/config.rs#L480), [507](../../src/config.rs#L507) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `file.into` | [480](../../src/config.rs#L480) | receiver-type-required |
| `open_private_root_darwin` | `open_directory` | [482](../../src/config.rs#L482), [488](../../src/config.rs#L488), [496](../../src/config.rs#L496) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `error.kind` | [484](../../src/config.rs#L484) | receiver-type-required |
| `open_private_root_darwin` | `path.parent().ok_or_else` | [485](../../src/config.rs#L485) | receiver-type-required |
| `open_private_root_darwin` | `path.parent` | [485](../../src/config.rs#L485) | receiver-type-required |
| `open_private_root_darwin` | `open_directory(parent_path).map_err` | [488](../../src/config.rs#L488) | receiver-type-required |
| `open_private_root_darwin` | `path                 .file_name()                 .ok_or_else` | [489](../../src/config.rs#L489) | receiver-type-required |
| `open_private_root_darwin` | `path                 .file_name` | [489](../../src/config.rs#L489) | receiver-type-required |
| `open_private_root_darwin` | `mkdirat` | [492](../../src/config.rs#L492) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `rustix_profile_error` | [494](../../src/config.rs#L494) | [profile::config::rustix_profile_error](../../src/config.rs#L311) |
| `open_private_root_darwin` | `open_directory(&path).map_err` | [496](../../src/config.rs#L496) | receiver-type-required |
| `open_private_root_darwin` | `error.raw_os_error` | [498](../../src/config.rs#L498) | receiver-type-required |
| `open_private_root_darwin` | `Some` | [498](../../src/config.rs#L498) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `ProfileError::from` | [504](../../src/config.rs#L504) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `validate_private_directory_fd` | [506](../../src/config.rs#L506) | [profile::config::validate_private_directory_fd](../../src/config.rs#L281) |
| `open_private_root_sandboxed` | `fs::canonicalize` | [517](../../src/config.rs#L517) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `Err` | [519](../../src/config.rs#L519) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `invalid_private_directory` | [519](../../src/config.rs#L519) | [profile::config::invalid_private_directory](../../src/config.rs#L274) |
| `open_private_root_sandboxed` | `open(&canonical, flags, Mode::empty()).map_err` | [524](../../src/config.rs#L524) | receiver-type-required |
| `open_private_root_sandboxed` | `open` | [524](../../src/config.rs#L524) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `Mode::empty` | [524](../../src/config.rs#L524) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `validate_private_directory_fd` | [525](../../src/config.rs#L525) | [profile::config::validate_private_directory_fd](../../src/config.rs#L281) |
| `open_private_root_sandboxed` | `Ok` | [526](../../src/config.rs#L526) | external-constructor-callback-or-unresolved |
| `prepare_repository_directories` | `rustix::process::geteuid().as_raw` | [530](../../src/config.rs#L530) | receiver-type-required |
| `prepare_repository_directories` | `rustix::process::geteuid` | [530](../../src/config.rs#L530) | external-constructor-callback-or-unresolved |
| `prepare_repository_directories` | `open_private_root` | [531](../../src/config.rs#L531) | [profile::config::open_private_root](../../src/config.rs#L349) |
| `prepare_repository_directories` | `open_directory_at` | [532](../../src/config.rs#L532) | [profile::config::open_directory_at](../../src/config.rs#L315) |
| `prepare_repository_directories` | `root.join` | [535](../../src/config.rs#L535) | receiver-type-required |
| `prepare_repository_directories` | `Ok` | [539](../../src/config.rs#L539) | external-constructor-callback-or-unresolved |
| `open` | `root.as_ref().to_path_buf` | [544](../../src/config.rs#L544) | receiver-type-required |
| `open` | `root.as_ref` | [544](../../src/config.rs#L544) | receiver-type-required |
| `open` | `prepare_repository_directories` | [545](../../src/config.rs#L545) | [profile::config::prepare_repository_directories](../../src/config.rs#L529) |
| `open` | `Ok` | [546](../../src/config.rs#L546) | external-constructor-callback-or-unresolved |
| `providers` | `NamedLock::shared` | [557](../../src/config.rs#L557) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `providers` | `self.root.join` | [557](../../src/config.rs#L557), [558](../../src/config.rs#L558) | receiver-type-required |
| `providers` | `self.read_global_optional(&path)?.unwrap_or_default` | [559](../../src/config.rs#L559) | receiver-type-required |
| `providers` | `self.read_global_optional` | [559](../../src/config.rs#L559) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `providers` | `validate_providers` | [560](../../src/config.rs#L560) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `providers` | `Ok` | [561](../../src/config.rs#L561) | external-constructor-callback-or-unresolved |
| `settings` | `NamedLock::shared` | [566](../../src/config.rs#L566) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `settings` | `self.root.join` | [566](../../src/config.rs#L566), [567](../../src/config.rs#L567) | receiver-type-required |
| `settings` | `self.read_global_optional(&path)?.unwrap_or_default` | [568](../../src/config.rs#L568) | receiver-type-required |
| `settings` | `self.read_global_optional` | [568](../../src/config.rs#L568) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `settings` | `validate_settings` | [569](../../src/config.rs#L569) | [profile::config::validate_settings](../../src/config.rs#L966) |
| `settings` | `Ok` | [570](../../src/config.rs#L570) | external-constructor-callback-or-unresolved |
| `publish_providers` | `self.root.join` | [578](../../src/config.rs#L578) | receiver-type-required |
| `publish_providers` | `self.publish_global` | [579](../../src/config.rs#L579) | [profile::config::ConfigRepository::publish_global](../../src/config.rs#L695) |
| `publish_providers` | `validate_providers` | [580](../../src/config.rs#L580) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `publish_providers_checked` | `self.root.join` | [593](../../src/config.rs#L593), [597](../../src/config.rs#L597), [602](../../src/config.rs#L602), [612](../../src/config.rs#L612) | receiver-type-required |
| `publish_providers_checked` | `management_path             .exists()             .then(&#124;&#124; NamedLock::shared(&management_path))             .transpose` | [598](../../src/config.rs#L598) | receiver-type-required |
| `publish_providers_checked` | `management_path             .exists()             .then` | [598](../../src/config.rs#L598) | receiver-type-required |
| `publish_providers_checked` | `management_path             .exists` | [598](../../src/config.rs#L598) | receiver-type-required |
| `publish_providers_checked` | `NamedLock::shared` | [600](../../src/config.rs#L600) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `publish_providers_checked` | `NamedLock::exclusive` | [602](../../src/config.rs#L602) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_providers_checked` | `self             .read_global_optional::<ProvidersConfig>(&path)?             .unwrap_or_default` | [603](../../src/config.rs#L603) | receiver-type-required |
| `publish_providers_checked` | `self             .read_global_optional::<ProvidersConfig>` | [603](../../src/config.rs#L603) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `publish_providers_checked` | `validate_providers` | [606](../../src/config.rs#L606), [608](../../src/config.rs#L608) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `publish_providers_checked` | `check_publication_revision` | [607](../../src/config.rs#L607) | [profile::config::check_publication_revision](../../src/config.rs#L808) |
| `publish_providers_checked` | `self.validate_all_provider_references_unlocked` | [609](../../src/config.rs#L609) | [profile::config::ConfigRepository::validate_all_provider_references_unlocked](../../src/config.rs#L799) |
| `publish_providers_checked` | `self                 .read_global_optional::<SettingsConfig>(&self.root.join("config/settings.json"))?                 .unwrap_or_default` | [611](../../src/config.rs#L611) | receiver-type-required |
| `publish_providers_checked` | `self                 .read_global_optional::<SettingsConfig>` | [611](../../src/config.rs#L611) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `publish_providers_checked` | `AtomicPublisher::replace` | [615](../../src/config.rs#L615) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_providers_checked` | `canonical_line` | [615](../../src/config.rs#L615) | [profile::canonical_line](../../src/lib.rs#L138) |
| `publish_providers_checked` | `Ok` | [616](../../src/config.rs#L616) | external-constructor-callback-or-unresolved |
| `publish_settings` | `self.root.join` | [624](../../src/config.rs#L624) | receiver-type-required |
| `publish_settings` | `self.publish_global` | [625](../../src/config.rs#L625) | [profile::config::ConfigRepository::publish_global](../../src/config.rs#L695) |
| `publish_settings` | `validate_settings` | [626](../../src/config.rs#L626) | [profile::config::validate_settings](../../src/config.rs#L966) |
| `publish_settings_checked` | `self.root.join` | [637](../../src/config.rs#L637), [638](../../src/config.rs#L638), [639](../../src/config.rs#L639) | receiver-type-required |
| `publish_settings_checked` | `NamedLock::exclusive` | [639](../../src/config.rs#L639) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_settings_checked` | `self             .read_global_optional::<SettingsConfig>(&path)?             .unwrap_or_default` | [640](../../src/config.rs#L640) | receiver-type-required |
| `publish_settings_checked` | `self             .read_global_optional::<SettingsConfig>` | [640](../../src/config.rs#L640) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `publish_settings_checked` | `validate_settings` | [643](../../src/config.rs#L643), [645](../../src/config.rs#L645) | [profile::config::validate_settings](../../src/config.rs#L966) |
| `publish_settings_checked` | `check_publication_revision` | [644](../../src/config.rs#L644) | [profile::config::check_publication_revision](../../src/config.rs#L808) |
| `publish_settings_checked` | `self             .read_global_optional::<ProvidersConfig>(&providers_path)?             .unwrap_or_default` | [646](../../src/config.rs#L646) | receiver-type-required |
| `publish_settings_checked` | `self             .read_global_optional::<ProvidersConfig>` | [646](../../src/config.rs#L646) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `publish_settings_checked` | `validate_providers` | [649](../../src/config.rs#L649) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `publish_settings_checked` | `validate_references` | [650](../../src/config.rs#L650) | [profile::config::validate_references](../../src/config.rs#L1028) |
| `publish_settings_checked` | `AtomicPublisher::replace` | [651](../../src/config.rs#L651) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_settings_checked` | `canonical_line` | [651](../../src/config.rs#L651) | [profile::canonical_line](../../src/lib.rs#L138) |
| `publish_settings_checked` | `Ok` | [652](../../src/config.rs#L652) | external-constructor-callback-or-unresolved |
| `publish_session_settings` | `store::session_settings_path` | [661](../../src/config.rs#L661) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L44) |
| `publish_session_settings` | `session_folder.as_ref` | [661](../../src/config.rs#L661) | receiver-type-required |
| `publish_session_settings` | `NamedLock::exclusive` | [662](../../src/config.rs#L662) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_session_settings` | `self.root.join` | [662](../../src/config.rs#L662), [672](../../src/config.rs#L672) | receiver-type-required |
| `publish_session_settings` | `self.read_optional::<SessionSettings>` | [663](../../src/config.rs#L663) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L777) |
| `publish_session_settings` | `validate_session_settings` | [665](../../src/config.rs#L665), [671](../../src/config.rs#L671) | [profile::config::validate_session_settings](../../src/config.rs#L988) |
| `publish_session_settings` | `check_publication_revision` | [670](../../src/config.rs#L670) | [profile::config::check_publication_revision](../../src/config.rs#L808) |
| `publish_session_settings` | `self             .read_global_optional(&providers_path)?             .unwrap_or_default` | [673](../../src/config.rs#L673) | receiver-type-required |
| `publish_session_settings` | `self             .read_global_optional` | [673](../../src/config.rs#L673) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `publish_session_settings` | `validate_providers` | [676](../../src/config.rs#L676) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `publish_session_settings` | `validate_session_reference` | [677](../../src/config.rs#L677) | [profile::config::validate_session_reference](../../src/config.rs#L1002) |
| `publish_session_settings` | `Some` | [677](../../src/config.rs#L677) | external-constructor-callback-or-unresolved |
| `publish_session_settings` | `value.clone` | [677](../../src/config.rs#L677) | receiver-type-required |
| `publish_session_settings` | `AtomicPublisher::replace` | [678](../../src/config.rs#L678) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_session_settings` | `canonical_line` | [678](../../src/config.rs#L678) | [profile::canonical_line](../../src/lib.rs#L138) |
| `publish_session_settings` | `Ok` | [679](../../src/config.rs#L679) | external-constructor-callback-or-unresolved |
| `session_settings` | `NamedLock::shared` | [686](../../src/config.rs#L686) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `session_settings` | `self.root.join` | [686](../../src/config.rs#L686) | receiver-type-required |
| `session_settings` | `store::session_settings_path` | [687](../../src/config.rs#L687) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L44) |
| `session_settings` | `session_folder.as_ref` | [687](../../src/config.rs#L687) | receiver-type-required |
| `session_settings` | `self.read_optional::<SessionSettings>` | [688](../../src/config.rs#L688) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L777) |
| `session_settings` | `validate_session_settings` | [690](../../src/config.rs#L690) | [profile::config::validate_session_settings](../../src/config.rs#L988) |
| `session_settings` | `Ok` | [692](../../src/config.rs#L692) | external-constructor-callback-or-unresolved |
| `publish_global` | `NamedLock::exclusive` | [703](../../src/config.rs#L703) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_global` | `self.root.join` | [703](../../src/config.rs#L703) | receiver-type-required |
| `publish_global` | `fs::read` | [704](../../src/config.rs#L704) | external-constructor-callback-or-unresolved |
| `publish_global` | `parse_global_json` | [706](../../src/config.rs#L706) | [profile::parse_global_json](../../src/lib.rs#L119) |
| `publish_global` | `validate` | [707](../../src/config.rs#L707), [714](../../src/config.rs#L714) | external-constructor-callback-or-unresolved |
| `publish_global` | `revision_from_value` | [708](../../src/config.rs#L708) | [profile::config::revision_from_value](../../src/config.rs#L830) |
| `publish_global` | `error.kind` | [710](../../src/config.rs#L710) | receiver-type-required |
| `publish_global` | `Err` | [711](../../src/config.rs#L711) | external-constructor-callback-or-unresolved |
| `publish_global` | `error.into` | [711](../../src/config.rs#L711) | receiver-type-required |
| `publish_global` | `check_publication_revision` | [713](../../src/config.rs#L713) | [profile::config::check_publication_revision](../../src/config.rs#L808) |
| `publish_global` | `AtomicPublisher::replace` | [715](../../src/config.rs#L715) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_global` | `canonical_line` | [715](../../src/config.rs#L715) | [profile::canonical_line](../../src/lib.rs#L138) |
| `publish_global` | `Ok` | [716](../../src/config.rs#L716) | external-constructor-callback-or-unresolved |
| `resolve_execution_snapshot` | `NamedLock::shared` | [725](../../src/config.rs#L725) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve_execution_snapshot` | `self.root.join` | [725](../../src/config.rs#L725), [727](../../src/config.rs#L727), [730](../../src/config.rs#L730) | receiver-type-required |
| `resolve_execution_snapshot` | `self             .read_global_optional(&self.root.join("config/providers.json"))?             .unwrap_or_default` | [726](../../src/config.rs#L726) | receiver-type-required |
| `resolve_execution_snapshot` | `self             .read_global_optional` | [726](../../src/config.rs#L726), [729](../../src/config.rs#L729) | [profile::config::ConfigRepository::read_global_optional](../../src/config.rs#L788) |
| `resolve_execution_snapshot` | `self             .read_global_optional(&self.root.join("config/settings.json"))?             .unwrap_or_default` | [729](../../src/config.rs#L729) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(store::session_settings_path)             .transpose()?             .map(&#124;path&#124; self.read_optional::<SessionSettings>(&path))             .transpose()?             .flatten` | [732](../../src/config.rs#L732) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(store::session_settings_path)             .transpose()?             .map(&#124;path&#124; self.read_optional::<SessionSettings>(&path))             .transpose` | [732](../../src/config.rs#L732) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(store::session_settings_path)             .transpose()?             .map` | [732](../../src/config.rs#L732) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(store::session_settings_path)             .transpose` | [732](../../src/config.rs#L732) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map` | [732](../../src/config.rs#L732), [739](../../src/config.rs#L739) | receiver-type-required |
| `resolve_execution_snapshot` | `self.read_optional::<SessionSettings>` | [735](../../src/config.rs#L735) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L777) |
| `resolve_execution_snapshot` | `execution.allowed_tools.clone` | [738](../../src/config.rs#L738) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(session_preset_tools)             .transpose()?             .flatten` | [739](../../src/config.rs#L739) | receiver-type-required |
| `resolve_execution_snapshot` | `session_folder             .map(session_preset_tools)             .transpose` | [739](../../src/config.rs#L739) | receiver-type-required |
| `resolve_execution_snapshot` | `allowed_tools.retain` | [744](../../src/config.rs#L744) | receiver-type-required |
| `resolve_execution_snapshot` | `selection.contains` | [744](../../src/config.rs#L744) | receiver-type-required |
| `resolve_execution_snapshot` | `session_id.to_owned` | [750](../../src/config.rs#L750) | receiver-type-required |
| `resolve_execution_snapshot` | `std::iter::once(&execution.cwd)                     .chain(&execution.read_roots)                     .cloned()                     .collect` | [752](../../src/config.rs#L752) | receiver-type-required |
| `resolve_execution_snapshot` | `std::iter::once(&execution.cwd)                     .chain(&execution.read_roots)                     .cloned` | [752](../../src/config.rs#L752) | receiver-type-required |
| `resolve_execution_snapshot` | `std::iter::once(&execution.cwd)                     .chain` | [752](../../src/config.rs#L752) | receiver-type-required |
| `resolve_execution_snapshot` | `std::iter::once` | [752](../../src/config.rs#L752) | external-constructor-callback-or-unresolved |
| `resolve_execution_snapshot` | `execution.writable_roots.clone` | [757](../../src/config.rs#L757) | receiver-type-required |
| `resolve_execution_snapshot` | `execution.toolchain_roots.clone` | [758](../../src/config.rs#L758) | receiver-type-required |
| `resolve_execution_snapshot` | `session_settings.as_ref().map` | [767](../../src/config.rs#L767) | receiver-type-required |
| `resolve_execution_snapshot` | `session_settings.as_ref` | [767](../../src/config.rs#L767) | receiver-type-required |
| `resolve_execution_snapshot` | `validate_snapshot` | [773](../../src/config.rs#L773) | [profile::config::validate_snapshot](../../src/config.rs#L1084) |
| `resolve_execution_snapshot` | `Ok` | [774](../../src/config.rs#L774) | external-constructor-callback-or-unresolved |
| `read_optional` | `fs::read` | [781](../../src/config.rs#L781) | external-constructor-callback-or-unresolved |
| `read_optional` | `Ok` | [782](../../src/config.rs#L782), [783](../../src/config.rs#L783) | external-constructor-callback-or-unresolved |
| `read_optional` | `Some` | [782](../../src/config.rs#L782) | external-constructor-callback-or-unresolved |
| `read_optional` | `parse_canonical` | [782](../../src/config.rs#L782) | [profile::parse_canonical](../../src/lib.rs#L69) |
| `read_optional` | `error.kind` | [783](../../src/config.rs#L783) | receiver-type-required |
| `read_optional` | `Err` | [784](../../src/config.rs#L784) | external-constructor-callback-or-unresolved |
| `read_optional` | `error.into` | [784](../../src/config.rs#L784) | receiver-type-required |
| `read_global_optional` | `fs::read` | [792](../../src/config.rs#L792) | external-constructor-callback-or-unresolved |
| `read_global_optional` | `Ok` | [793](../../src/config.rs#L793), [794](../../src/config.rs#L794) | external-constructor-callback-or-unresolved |
| `read_global_optional` | `Some` | [793](../../src/config.rs#L793) | external-constructor-callback-or-unresolved |
| `read_global_optional` | `parse_global_json` | [793](../../src/config.rs#L793) | [profile::parse_global_json](../../src/lib.rs#L119) |
| `read_global_optional` | `error.kind` | [794](../../src/config.rs#L794) | receiver-type-required |
| `read_global_optional` | `Err` | [795](../../src/config.rs#L795) | external-constructor-callback-or-unresolved |
| `read_global_optional` | `error.into` | [795](../../src/config.rs#L795) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `validate_references` | [804](../../src/config.rs#L804) | [profile::config::validate_references](../../src/config.rs#L1028) |
| `validate_all_provider_references_unlocked` | `Path::new` | [804](../../src/config.rs#L804) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `validate_safe_integer` | [809](../../src/config.rs#L809), [810](../../src/config.rs#L810), [811](../../src/config.rs#L811) | [profile::config::validate_safe_integer](../../src/config.rs#L1176) |
| `check_publication_revision` | `Path::new` | [809](../../src/config.rs#L809), [810](../../src/config.rs#L810), [811](../../src/config.rs#L811) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `Err` | [813](../../src/config.rs#L813), [822](../../src/config.rs#L822) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `actual         .checked_add(1)         .ok_or_else` | [815](../../src/config.rs#L815) | receiver-type-required |
| `check_publication_revision` | `actual         .checked_add` | [815](../../src/config.rs#L815) | receiver-type-required |
| `check_publication_revision` | `PathBuf::from` | [818](../../src/config.rs#L818), [823](../../src/config.rs#L823) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `"revision cannot advance beyond the I-JSON safe-integer range".to_owned` | [819](../../src/config.rs#L819) | receiver-type-required |
| `check_publication_revision` | `Ok` | [827](../../src/config.rs#L827) | external-constructor-callback-or-unresolved |
| `revision_from_value` | `serde_json::to_value(value).map_err` | [835](../../src/config.rs#L835) | receiver-type-required |
| `revision_from_value` | `serde_json::to_value` | [835](../../src/config.rs#L835) | external-constructor-callback-or-unresolved |
| `revision_from_value` | `path.to_path_buf` | [836](../../src/config.rs#L836), [842](../../src/config.rs#L842) | receiver-type-required |
| `revision_from_value` | `error.to_string` | [837](../../src/config.rs#L837), [843](../../src/config.rs#L843) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>(value)         .map(&#124;value&#124; value.revision)         .map_err` | [839](../../src/config.rs#L839) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>(value)         .map` | [839](../../src/config.rs#L839) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>` | [839](../../src/config.rs#L839) | external-constructor-callback-or-unresolved |
| `validate_common` | `Err` | [854](../../src/config.rs#L854), [860](../../src/config.rs#L860) | external-constructor-callback-or-unresolved |
| `validate_common` | `path.to_path_buf` | [855](../../src/config.rs#L855), [861](../../src/config.rs#L861) | receiver-type-required |
| `validate_common` | `"stored revision must be at least 1".to_owned` | [862](../../src/config.rs#L862) | receiver-type-required |
| `validate_common` | `validate_safe_integer` | [865](../../src/config.rs#L865) | [profile::config::validate_safe_integer](../../src/config.rs#L1176) |
| `validate_common` | `Ok` | [866](../../src/config.rs#L866) | external-constructor-callback-or-unresolved |
| `validate_providers` | `validate_common` | [874](../../src/config.rs#L874) | [profile::config::validate_common](../../src/config.rs#L847) |
| `validate_providers` | `BTreeSet::new` | [875](../../src/config.rs#L875), [901](../../src/config.rs#L901) | external-constructor-callback-or-unresolved |
| `validate_providers` | `validate_id` | [877](../../src/config.rs#L877), [879](../../src/config.rs#L879), [881](../../src/config.rs#L881), [882](../../src/config.rs#L882), [883](../../src/config.rs#L883), [884](../../src/config.rs#L884), [889](../../src/config.rs#L889), [894](../../src/config.rs#L894), [899](../../src/config.rs#L899), [903](../../src/config.rs#L903), [904](../../src/config.rs#L904), [925](../../src/config.rs#L925) | [profile::config::validate_id](../../src/config.rs#L1222) |
| `validate_providers` | `ids.insert` | [895](../../src/config.rs#L895) | receiver-type-required |
| `validate_providers` | `provider.id.as_str` | [895](../../src/config.rs#L895) | receiver-type-required |
| `validate_providers` | `invalid` | [896](../../src/config.rs#L896), [911](../../src/config.rs#L911), [917](../../src/config.rs#L917), [923](../../src/config.rs#L923) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_providers` | `validate_safe_integer` | [905](../../src/config.rs#L905), [906](../../src/config.rs#L906) | [profile::config::validate_safe_integer](../../src/config.rs#L1176) |
| `validate_providers` | `models.insert` | [916](../../src/config.rs#L916) | receiver-type-required |
| `validate_providers` | `model.id.as_str` | [916](../../src/config.rs#L916) | receiver-type-required |
| `validate_providers` | `validate_web_search_origin` | [926](../../src/config.rs#L926) | [profile::config::validate_web_search_origin](../../src/config.rs#L931) |
| `validate_providers` | `Ok` | [928](../../src/config.rs#L928) | external-constructor-callback-or-unresolved |
| `validate_web_search_origin` | `url::Url::parse(value).map_err` | [932](../../src/config.rs#L932) | receiver-type-required |
| `validate_web_search_origin` | `url::Url::parse` | [932](../../src/config.rs#L932) | external-constructor-callback-or-unresolved |
| `validate_web_search_origin` | `path.to_path_buf` | [933](../../src/config.rs#L933) | receiver-type-required |
| `validate_web_search_origin` | `"web_search endpoint is not an absolute URL".to_owned` | [934](../../src/config.rs#L934) | receiver-type-required |
| `validate_web_search_origin` | `parsed.scheme` | [936](../../src/config.rs#L936) | receiver-type-required |
| `validate_web_search_origin` | `parsed.username` | [937](../../src/config.rs#L937) | receiver-type-required |
| `validate_web_search_origin` | `parsed.password().is_some` | [938](../../src/config.rs#L938) | receiver-type-required |
| `validate_web_search_origin` | `parsed.password` | [938](../../src/config.rs#L938) | receiver-type-required |
| `validate_web_search_origin` | `parsed.query().is_some` | [939](../../src/config.rs#L939) | receiver-type-required |
| `validate_web_search_origin` | `parsed.query` | [939](../../src/config.rs#L939) | receiver-type-required |
| `validate_web_search_origin` | `parsed.fragment().is_some` | [940](../../src/config.rs#L940) | receiver-type-required |
| `validate_web_search_origin` | `parsed.fragment` | [940](../../src/config.rs#L940) | receiver-type-required |
| `validate_web_search_origin` | `parsed.path` | [941](../../src/config.rs#L941) | receiver-type-required |
| `validate_web_search_origin` | `parsed.host_str().is_none` | [942](../../src/config.rs#L942) | receiver-type-required |
| `validate_web_search_origin` | `parsed.host_str` | [942](../../src/config.rs#L942), [950](../../src/config.rs#L950) | receiver-type-required |
| `validate_web_search_origin` | `value.ends_with` | [943](../../src/config.rs#L943) | receiver-type-required |
| `validate_web_search_origin` | `invalid` | [945](../../src/config.rs#L945), [961](../../src/config.rs#L961) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_web_search_origin` | `parsed.host_str().expect` | [950](../../src/config.rs#L950) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix('[')         .and_then(&#124;value&#124; value.strip_suffix(']'))         .unwrap_or` | [951](../../src/config.rs#L951) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix('[')         .and_then` | [951](../../src/config.rs#L951) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix` | [951](../../src/config.rs#L951) | receiver-type-required |
| `validate_web_search_origin` | `value.strip_suffix` | [953](../../src/config.rs#L953) | receiver-type-required |
| `validate_web_search_origin` | `host.eq_ignore_ascii_case` | [955](../../src/config.rs#L955) | receiver-type-required |
| `validate_web_search_origin` | `host.ends_with` | [956](../../src/config.rs#L956) | receiver-type-required |
| `validate_web_search_origin` | `address_literal             .parse::<std::net::IpAddr>()             .is_ok_and` | [957](../../src/config.rs#L957) | receiver-type-required |
| `validate_web_search_origin` | `address_literal             .parse::<std::net::IpAddr>` | [957](../../src/config.rs#L957) | receiver-type-required |
| `validate_web_search_origin` | `tools::is_public_internet_address` | [959](../../src/config.rs#L959) | [tools::runtime_backends::is_public_internet_address](../../../tools/src/runtime_backends.rs#L1744) |
| `validate_web_search_origin` | `Ok` | [963](../../src/config.rs#L963) | external-constructor-callback-or-unresolved |
| `validate_settings` | `validate_common` | [971](../../src/config.rs#L971) | [profile::config::validate_common](../../src/config.rs#L847) |
| `validate_settings` | `Some` | [973](../../src/config.rs#L973) | external-constructor-callback-or-unresolved |
| `validate_settings` | `invalid` | [974](../../src/config.rs#L974) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_settings` | `validate_safe_integer` | [981](../../src/config.rs#L981) | [profile::config::validate_safe_integer](../../src/config.rs#L1176) |
| `validate_settings` | `Ok` | [985](../../src/config.rs#L985) | external-constructor-callback-or-unresolved |
| `validate_session_settings` | `validate_common` | [993](../../src/config.rs#L993) | [profile::config::validate_common](../../src/config.rs#L847) |
| `validate_session_settings` | `validate_id` | [994](../../src/config.rs#L994), [995](../../src/config.rs#L995) | [profile::config::validate_id](../../src/config.rs#L1222) |
| `validate_session_settings` | `value.reasoning_effort.as_deref().is_some_and` | [996](../../src/config.rs#L996) | receiver-type-required |
| `validate_session_settings` | `value.reasoning_effort.as_deref` | [996](../../src/config.rs#L996) | receiver-type-required |
| `validate_session_settings` | `invalid` | [997](../../src/config.rs#L997) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_session_settings` | `Ok` | [999](../../src/config.rs#L999) | external-constructor-callback-or-unresolved |
| `validate_session_reference` | `Ok` | [1008](../../src/config.rs#L1008), [1025](../../src/config.rs#L1025) | external-constructor-callback-or-unresolved |
| `validate_session_reference` | `providers         .providers         .iter()         .find(&#124;provider&#124; provider.id == settings.provider)         .ok_or_else` | [1010](../../src/config.rs#L1010) | receiver-type-required |
| `validate_session_reference` | `providers         .providers         .iter()         .find` | [1010](../../src/config.rs#L1010) | receiver-type-required |
| `validate_session_reference` | `providers         .providers         .iter` | [1010](../../src/config.rs#L1010) | receiver-type-required |
| `validate_session_reference` | `path.to_path_buf` | [1015](../../src/config.rs#L1015) | receiver-type-required |
| `validate_session_reference` | `provider         .models         .iter()         .any` | [1018](../../src/config.rs#L1018) | receiver-type-required |
| `validate_session_reference` | `provider         .models         .iter` | [1018](../../src/config.rs#L1018) | receiver-type-required |
| `validate_session_reference` | `invalid_reference` | [1023](../../src/config.rs#L1023) | [profile::config::invalid_reference](../../src/config.rs#L1249) |
| `validate_references` | `settings.default_provider.as_deref` | [1033](../../src/config.rs#L1033) | receiver-type-required |
| `validate_references` | `settings.default_model.is_some` | [1034](../../src/config.rs#L1034) | receiver-type-required |
| `validate_references` | `invalid_reference` | [1035](../../src/config.rs#L1035), [1054](../../src/config.rs#L1054) | [profile::config::invalid_reference](../../src/config.rs#L1249) |
| `validate_references` | `Ok` | [1037](../../src/config.rs#L1037), [1056](../../src/config.rs#L1056) | external-constructor-callback-or-unresolved |
| `validate_references` | `providers         .providers         .iter()         .find(&#124;provider&#124; provider.id == id)         .ok_or_else` | [1040](../../src/config.rs#L1040) | receiver-type-required |
| `validate_references` | `providers         .providers         .iter()         .find` | [1040](../../src/config.rs#L1040) | receiver-type-required |
| `validate_references` | `providers         .providers         .iter` | [1040](../../src/config.rs#L1040) | receiver-type-required |
| `validate_references` | `path.to_owned` | [1045](../../src/config.rs#L1045) | receiver-type-required |
| `validate_references` | `settings.default_model.as_deref` | [1048](../../src/config.rs#L1048) | receiver-type-required |
| `validate_references` | `provider             .models             .iter()             .any` | [1049](../../src/config.rs#L1049) | receiver-type-required |
| `validate_references` | `provider             .models             .iter` | [1049](../../src/config.rs#L1049) | receiver-type-required |
| `session_preset_tools` | `session_folder.join` | [1061](../../src/config.rs#L1061) | receiver-type-required |
| `session_preset_tools` | `ledger.is_file` | [1062](../../src/config.rs#L1062) | receiver-type-required |
| `session_preset_tools` | `Ok` | [1063](../../src/config.rs#L1063), [1066](../../src/config.rs#L1066), [1073](../../src/config.rs#L1073), [1081](../../src/config.rs#L1081) | external-constructor-callback-or-unresolved |
| `session_preset_tools` | `store::read_genesis` | [1065](../../src/config.rs#L1065) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `session_preset_tools` | `serde_json::to_value(genesis.raw()).map_err` | [1068](../../src/config.rs#L1068) | receiver-type-required |
| `session_preset_tools` | `serde_json::to_value` | [1068](../../src/config.rs#L1068) | external-constructor-callback-or-unresolved |
| `session_preset_tools` | `genesis.raw` | [1068](../../src/config.rs#L1068) | receiver-type-required |
| `session_preset_tools` | `ledger.clone` | [1069](../../src/config.rs#L1069) | receiver-type-required |
| `session_preset_tools` | `error.to_string` | [1070](../../src/config.rs#L1070) | receiver-type-required |
| `session_preset_tools` | `raw.get` | [1072](../../src/config.rs#L1072) | receiver-type-required |
| `session_preset_tools` | `tools::ResolvedPreset::from_genesis(preset).map_err` | [1075](../../src/config.rs#L1075) | receiver-type-required |
| `session_preset_tools` | `tools::ResolvedPreset::from_genesis` | [1075](../../src/config.rs#L1075) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `validate_snapshot` | `Path::new` | [1085](../../src/config.rs#L1085), [1130](../../src/config.rs#L1130) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `validate_absolute` | [1087](../../src/config.rs#L1087), [1101](../../src/config.rs#L1101), [1113](../../src/config.rs#L1113), [1122](../../src/config.rs#L1122) | [profile::config::validate_absolute](../../src/config.rs#L1212) |
| `validate_snapshot` | `Err` | [1090](../../src/config.rs#L1090) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `path.to_path_buf` | [1091](../../src/config.rs#L1091) | receiver-type-required |
| `validate_snapshot` | `validate_id` | [1095](../../src/config.rs#L1095) | [profile::config::validate_id](../../src/config.rs#L1222) |
| `validate_snapshot` | `snapshot.execution.cwd.is_empty` | [1096](../../src/config.rs#L1096) | receiver-type-required |
| `validate_snapshot` | `invalid` | [1097](../../src/config.rs#L1097), [1103](../../src/config.rs#L1103), [1110](../../src/config.rs#L1110), [1115](../../src/config.rs#L1115), [1124](../../src/config.rs#L1124), [1132](../../src/config.rs#L1132), [1137](../../src/config.rs#L1137), [1147](../../src/config.rs#L1147), [1153](../../src/config.rs#L1153), [1168](../../src/config.rs#L1168) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_snapshot` | `validate_unique` | [1099](../../src/config.rs#L1099) | [profile::config::validate_unique](../../src/config.rs#L1233) |
| `validate_snapshot` | `canonical_directory` | [1102](../../src/config.rs#L1102), [1114](../../src/config.rs#L1114), [1123](../../src/config.rs#L1123) | [profile::config::canonical_directory](../../src/config.rs#L1191) |
| `validate_snapshot` | `is_sorted_unique` | [1106](../../src/config.rs#L1106), [1107](../../src/config.rs#L1107), [1108](../../src/config.rs#L1108) | [profile::config::is_sorted_unique](../../src/config.rs#L1187) |
| `validate_snapshot` | `root.contains` | [1114](../../src/config.rs#L1114) | receiver-type-required |
| `validate_snapshot` | `snapshot             .execution             .cwd             .iter()             .any` | [1126](../../src/config.rs#L1126) | receiver-type-required |
| `validate_snapshot` | `snapshot             .execution             .cwd             .iter` | [1126](../../src/config.rs#L1126) | receiver-type-required |
| `validate_snapshot` | `Path::new(root).starts_with` | [1130](../../src/config.rs#L1130) | receiver-type-required |
| `validate_snapshot` | `validate_safe_integer` | [1139](../../src/config.rs#L1139) | [profile::config::validate_safe_integer](../../src/config.rs#L1176) |
| `validate_snapshot` | `validate_providers` | [1141](../../src/config.rs#L1141) | [profile::config::validate_providers](../../src/config.rs#L869) |
| `validate_snapshot` | `validate_settings` | [1142](../../src/config.rs#L1142) | [profile::config::validate_settings](../../src/config.rs#L966) |
| `validate_snapshot` | `validate_session_settings` | [1144](../../src/config.rs#L1144) | [profile::config::validate_session_settings](../../src/config.rs#L988) |
| `validate_snapshot` | `ProvidersConfig::default` | [1146](../../src/config.rs#L1146) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `SettingsConfig::default` | [1152](../../src/config.rs#L1152) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `validate_references` | [1158](../../src/config.rs#L1158) | [profile::config::validate_references](../../src/config.rs#L1028) |
| `validate_snapshot` | `validate_session_reference` | [1159](../../src/config.rs#L1159) | [profile::config::validate_session_reference](../../src/config.rs#L1002) |
| `validate_snapshot` | `snapshot                 .session_settings                 .as_ref()                 .map` | [1163](../../src/config.rs#L1163) | receiver-type-required |
| `validate_snapshot` | `snapshot                 .session_settings                 .as_ref` | [1163](../../src/config.rs#L1163) | receiver-type-required |
| `validate_snapshot` | `Ok` | [1173](../../src/config.rs#L1173) | external-constructor-callback-or-unresolved |
| `validate_safe_integer` | `Ok` | [1178](../../src/config.rs#L1178) | external-constructor-callback-or-unresolved |
| `validate_safe_integer` | `invalid` | [1180](../../src/config.rs#L1180) | [profile::config::invalid](../../src/config.rs#L1242) |
| `is_sorted_unique` | `values.windows(2).all` | [1188](../../src/config.rs#L1188) | receiver-type-required |
| `is_sorted_unique` | `values.windows` | [1188](../../src/config.rs#L1188) | receiver-type-required |
| `canonical_directory` | `validate_absolute` | [1192](../../src/config.rs#L1192) | [profile::config::validate_absolute](../../src/config.rs#L1212) |
| `canonical_directory` | `fs::canonicalize(value).map_err` | [1193](../../src/config.rs#L1193) | receiver-type-required |
| `canonical_directory` | `fs::canonicalize` | [1193](../../src/config.rs#L1193) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `PathBuf::from` | [1194](../../src/config.rs#L1194) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `error.to_string` | [1195](../../src/config.rs#L1195) | receiver-type-required |
| `canonical_directory` | `canonical.is_dir` | [1197](../../src/config.rs#L1197) | receiver-type-required |
| `canonical_directory` | `Err` | [1198](../../src/config.rs#L1198) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `"expected a directory".to_owned` | [1200](../../src/config.rs#L1200) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str()         .map(ToOwned::to_owned)         .ok_or_else` | [1203](../../src/config.rs#L1203) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str()         .map` | [1203](../../src/config.rs#L1203) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str` | [1203](../../src/config.rs#L1203) | receiver-type-required |
| `canonical_directory` | `"path is not UTF-8".to_owned` | [1208](../../src/config.rs#L1208) | receiver-type-required |
| `validate_absolute` | `value.contains` | [1213](../../src/config.rs#L1213) | receiver-type-required |
| `validate_absolute` | `Path::new(value).is_absolute` | [1213](../../src/config.rs#L1213) | receiver-type-required |
| `validate_absolute` | `Path::new` | [1213](../../src/config.rs#L1213) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `Err` | [1214](../../src/config.rs#L1214) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `source.to_path_buf` | [1215](../../src/config.rs#L1215) | receiver-type-required |
| `validate_absolute` | `Ok` | [1219](../../src/config.rs#L1219) | external-constructor-callback-or-unresolved |
| `validate_id` | `value.is_empty` | [1223](../../src/config.rs#L1223) | receiver-type-required |
| `validate_id` | `value.chars().any` | [1223](../../src/config.rs#L1223) | receiver-type-required |
| `validate_id` | `value.chars` | [1223](../../src/config.rs#L1223) | receiver-type-required |
| `validate_id` | `invalid` | [1224](../../src/config.rs#L1224) | [profile::config::invalid](../../src/config.rs#L1242) |
| `validate_id` | `Ok` | [1229](../../src/config.rs#L1229) | external-constructor-callback-or-unresolved |
| `validate_unique` | `values.iter().collect::<BTreeSet<_>>` | [1234](../../src/config.rs#L1234) | receiver-type-required |
| `validate_unique` | `values.iter` | [1234](../../src/config.rs#L1234) | receiver-type-required |
| `validate_unique` | `unique.len` | [1235](../../src/config.rs#L1235) | receiver-type-required |
| `validate_unique` | `values.len` | [1235](../../src/config.rs#L1235) | receiver-type-required |
| `validate_unique` | `Ok` | [1236](../../src/config.rs#L1236) | external-constructor-callback-or-unresolved |
| `validate_unique` | `invalid` | [1238](../../src/config.rs#L1238) | [profile::config::invalid](../../src/config.rs#L1242) |
| `invalid` | `Err` | [1243](../../src/config.rs#L1243) | external-constructor-callback-or-unresolved |
| `invalid` | `path.to_path_buf` | [1244](../../src/config.rs#L1244) | receiver-type-required |
| `invalid` | `reason.to_owned` | [1245](../../src/config.rs#L1245) | receiver-type-required |
| `invalid_reference` | `Err` | [1250](../../src/config.rs#L1250) | external-constructor-callback-or-unresolved |
| `invalid_reference` | `path.to_path_buf` | [1251](../../src/config.rs#L1251) | receiver-type-required |
| `invalid_reference` | `reason.to_owned` | [1252](../../src/config.rs#L1252) | receiver-type-required |
| `removed` | `current.iter().collect::<BTreeSet<_>>` | [1257](../../src/config.rs#L1257) | receiver-type-required |
| `removed` | `current.iter` | [1257](../../src/config.rs#L1257) | receiver-type-required |
| `removed` | `previous.iter().any` | [1258](../../src/config.rs#L1258) | receiver-type-required |
| `removed` | `previous.iter` | [1258](../../src/config.rs#L1258) | receiver-type-required |
| `removed` | `current.contains` | [1258](../../src/config.rs#L1258) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter()         .flat_map(&#124;provider&#124; {             provider                 .models                 .iter()                 .filter(&#124;model&#124; model.enabled)                 .map(move &#124;model&#124; (provider.id.as_str(), model.id.as_str()))         })         .collect` | [1270](../../src/config.rs#L1270) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter()         .flat_map` | [1270](../../src/config.rs#L1270) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter` | [1270](../../src/config.rs#L1270) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter()                 .filter(&#124;model&#124; model.enabled)                 .map` | [1274](../../src/config.rs#L1274) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter()                 .filter` | [1274](../../src/config.rs#L1274) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter` | [1274](../../src/config.rs#L1274) | receiver-type-required |
| `enabled_provider_models` | `provider.id.as_str` | [1278](../../src/config.rs#L1278) | receiver-type-required |
| `enabled_provider_models` | `model.id.as_str` | [1278](../../src/config.rs#L1278) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `tempfile::tempdir().expect` | [1295](../../src/config.rs#L1295) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `tempfile::tempdir` | [1295](../../src/config.rs#L1295) | external-constructor-callback-or-unresolved |
| `private_directory_rejects_wrong_owner` | `rustix::process::geteuid().as_raw` | [1296](../../src/config.rs#L1296) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `rustix::process::geteuid` | [1296](../../src/config.rs#L1296) | external-constructor-callback-or-unresolved |
| `private_directory_rejects_wrong_owner` | `open_private_root(root.path(), actual_uid, false).expect` | [1297](../../src/config.rs#L1297) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `open_private_root` | [1297](../../src/config.rs#L1297) | [profile::config::open_private_root](../../src/config.rs#L349) |
| `private_directory_rejects_wrong_owner` | `root.path` | [1297](../../src/config.rs#L1297), [1299](../../src/config.rs#L1299) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `validate_private_directory_fd(&descriptor, root.path(), actual_uid.wrapping_add(1))                 .expect_err` | [1299](../../src/config.rs#L1299) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `validate_private_directory_fd` | [1299](../../src/config.rs#L1299) | [profile::config::validate_private_directory_fd](../../src/config.rs#L281) |
| `private_directory_rejects_wrong_owner` | `actual_uid.wrapping_add` | [1299](../../src/config.rs#L1299) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `tempfile::tempdir().expect` | [1307](../../src/config.rs#L1307) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `tempfile::tempdir` | [1307](../../src/config.rs#L1307) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `root.path().join` | [1308](../../src/config.rs#L1308), [1310](../../src/config.rs#L1310) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `root.path` | [1308](../../src/config.rs#L1308), [1310](../../src/config.rs#L1310) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::fs::create_dir(&directory).expect` | [1309](../../src/config.rs#L1309) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::fs::create_dir` | [1309](../../src/config.rs#L1309) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::os::unix::fs::symlink(&directory, &alias).expect` | [1311](../../src/config.rs#L1311) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::os::unix::fs::symlink` | [1311](../../src/config.rs#L1311) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `open_private_root_sandboxed(             &alias,             &alias,             OFlags::RDONLY &#124; OFlags::DIRECTORY &#124; OFlags::CLOEXEC &#124; OFlags::NOFOLLOW,             rustix::process::geteuid().as_raw(),         )         .expect_err` | [1312](../../src/config.rs#L1312) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `open_private_root_sandboxed` | [1312](../../src/config.rs#L1312) | [profile::config::open_private_root_sandboxed](../../src/config.rs#L511) |
| `sandbox_fallback_rejects_a_symlinked_path` | `rustix::process::geteuid().as_raw` | [1316](../../src/config.rs#L1316) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `rustix::process::geteuid` | [1316](../../src/config.rs#L1316) | external-constructor-callback-or-unresolved |
