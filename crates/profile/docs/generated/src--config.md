# profile::config

[Package atlas](index.md) · [Source](../../src/config.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [profile::config::FORMAT](../../src/config.rs#L15) | const_item | `private` |  |
| [profile::config::MAX_SAFE_INTEGER](../../src/config.rs#L16) | const_item | `private` |  |
| [profile::config::WorkspacePolicy](../../src/config.rs#L20) | struct_item | `pub` |  |
| [profile::config::WorkspaceFolder](../../src/config.rs#L46) | struct_item | `pub` |  |
| [profile::config::WorkspaceConfig](../../src/config.rs#L53) | struct_item | `pub` |  |
| [profile::config::WorkspaceConfig::folder_paths](../../src/config.rs#L68) | function_item | `pub` |  |
| [profile::config::WorkspaceConfig::folder_ids](../../src/config.rs#L82) | function_item | `pub` |  |
| [profile::config::WorkspaceConfig::binding_for_path](../../src/config.rs#L96) | function_item | `pub` |  |
| [profile::config::WorkspaceConfig::path_for_binding](../../src/config.rs#L110) | function_item | `pub` |  |
| [profile::config::Model](../../src/config.rs#L128) | struct_item | `pub` |  |
| [profile::config::Provider](../../src/config.rs#L138) | struct_item | `pub` |  |
| [profile::config::WebSearch](../../src/config.rs#L157) | struct_item | `pub` |  |
| [profile::config::ProvidersConfig](../../src/config.rs#L165) | struct_item | `pub` |  |
| [profile::config::ProvidersConfig::default](../../src/config.rs#L174) | function_item | `private` |  |
| [profile::config::Limits](../../src/config.rs#L186) | struct_item | `pub` |  |
| [profile::config::SettingsConfig](../../src/config.rs#L195) | struct_item | `pub` |  |
| [profile::config::SessionSettings](../../src/config.rs#L208) | struct_item | `pub` |  |
| [profile::config::SettingsConfig::default](../../src/config.rs#L218) | function_item | `private` |  |
| [profile::config::EnvironmentsConfig](../../src/config.rs#L234) | struct_item | `pub` |  |
| [profile::config::EnvironmentsConfig::default](../../src/config.rs#L241) | function_item | `private` |  |
| [profile::config::EnvironmentsConfig::canonical_bytes](../../src/config.rs#L252) | function_item | `pub` |  |
| [profile::config::EnvironmentsConfig::decode](../../src/config.rs#L259) | function_item | `pub` |  |
| [profile::config::EnvironmentsConfig::environment](../../src/config.rs#L269) | function_item | `pub` |  |
| [profile::config::Environment](../../src/config.rs#L283) | struct_item | `pub` |  |
| [profile::config::Environment::local](../../src/config.rs#L296) | function_item | `pub` |  |
| [profile::config::ScopeSelection](../../src/config.rs#L309) | struct_item | `pub` |  |
| [profile::config::ResolvedWorkspace](../../src/config.rs#L317) | struct_item | `pub` |  |
| [profile::config::RevisionVector](../../src/config.rs#L345) | struct_item | `pub` |  |
| [profile::config::ignore_legacy_field](../../src/config.rs#L364) | function_item | `private` |  |
| [profile::config::ConfigSnapshot](../../src/config.rs#L370) | struct_item | `pub` |  |
| [profile::config::ConfigSnapshot::protected_roots](../../src/config.rs#L395) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::execution_cwd](../../src/config.rs#L403) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L410) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::digest](../../src/config.rs#L415) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::decode](../../src/config.rs#L419) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::publish](../../src/config.rs#L431) | function_item | `pub` |  |
| [profile::config::ConfigSnapshot::requires_respawn_from](../../src/config.rs#L445) | function_item | `pub` |  |
| [profile::config::config_document](../../src/config.rs#L469) | macro_definition | `private` |  |
| [profile::config::ConfigRepository](../../src/config.rs#L496) | struct_item | `pub` |  |
| [profile::config::invalid_private_directory](../../src/config.rs#L500) | function_item | `private` |  |
| [profile::config::validate_private_directory_fd](../../src/config.rs#L507) | function_item | `private` |  |
| [profile::config::rustix_profile_error](../../src/config.rs#L537) | function_item | `private` |  |
| [profile::config::open_directory_at](../../src/config.rs#L541) | function_item | `private` |  |
| [profile::config::open_private_root](../../src/config.rs#L575) | function_item | `private` |  |
| [profile::config::open_private_root_darwin](../../src/config.rs#L672) | function_item | `private` | #[cfg(target_os = "macos")] |
| [profile::config::open_private_root_sandboxed](../../src/config.rs#L737) | function_item | `private` | #[cfg(target_os = "macos")] |
| [profile::config::prepare_repository_directories](../../src/config.rs#L755) | function_item | `private` |  |
| [profile::config::prepare_private_directory](../../src/config.rs#L764) | function_item | `private` |  |
| [profile::config::prepare_workspace_directory](../../src/config.rs#L780) | function_item | `private` |  |
| [profile::config::prepare_existing_workspace_directories](../../src/config.rs#L790) | function_item | `private` |  |
| [profile::config::ConfigRepository::open](../../src/config.rs#L814) | function_item | `pub` |  |
| [profile::config::ConfigRepository::workspace_data_dir](../../src/config.rs#L821) | function_item | `pub` |  |
| [profile::config::ConfigRepository::root](../../src/config.rs#L827) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resolve](../../src/config.rs#L831) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resolve_for_binding](../../src/config.rs#L840) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resolve_for_session](../../src/config.rs#L850) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resolve_for_session_binding](../../src/config.rs#L871) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resolve_scoped](../../src/config.rs#L892) | function_item | `pub` |  |
| [profile::config::ConfigRepository::environments](../../src/config.rs#L917) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_environments](../../src/config.rs#L925) | function_item | `pub` |  |
| [profile::config::ConfigRepository::providers](../../src/config.rs#L938) | function_item | `pub` |  |
| [profile::config::ConfigRepository::settings](../../src/config.rs#L947) | function_item | `pub` |  |
| [profile::config::ConfigRepository::workspace](../../src/config.rs#L957) | function_item | `pub` |  |
| [profile::config::ConfigRepository::workspaces](../../src/config.rs#L972) | function_item | `pub` |  |
| [profile::config::ConfigRepository::resource_workspace_roots](../../src/config.rs#L1006) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_workspace](../../src/config.rs#L1047) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_providers](../../src/config.rs#L1069) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_providers_checked](../../src/config.rs#L1084) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_settings](../../src/config.rs#L1110) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_settings_checked](../../src/config.rs#L1123) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_workspace_policy](../../src/config.rs#L1158) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_session_settings](../../src/config.rs#L1196) | function_item | `pub` |  |
| [profile::config::ConfigRepository::session_settings](../../src/config.rs#L1221) | function_item | `pub` |  |
| [profile::config::ConfigRepository::publish_global](../../src/config.rs#L1234) | function_item | `private` |  |
| [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) | function_item | `private` |  |
| [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) | function_item | `private` |  |
| [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) | function_item | `private` |  |
| [profile::config::ConfigRepository::validate_all_provider_references_unlocked](../../src/config.rs#L1377) | function_item | `private` |  |
| [profile::config::check_publication_revision](../../src/config.rs#L1425) | function_item | `private` |  |
| [profile::config::revision_from_value](../../src/config.rs#L1447) | function_item | `private` |  |
| [profile::config::revision_from_value::RevisionOnly](../../src/config.rs#L1449) | struct_item | `private` |  |
| [profile::config::validate_workspace_id](../../src/config.rs#L1464) | function_item | `private` |  |
| [profile::config::workspace_document_path](../../src/config.rs#L1481) | function_item | `private` |  |
| [profile::config::validate_common](../../src/config.rs#L1487) | function_item | `private` |  |
| [profile::config::validate_workspace](../../src/config.rs#L1509) | function_item | `private` |  |
| [profile::config::validate_providers](../../src/config.rs#L1549) | function_item | `private` |  |
| [profile::config::validate_web_search_origin](../../src/config.rs#L1611) | function_item | `private` |  |
| [profile::config::validate_settings](../../src/config.rs#L1646) | function_item | `private` |  |
| [profile::config::validate_session_settings](../../src/config.rs#L1668) | function_item | `private` |  |
| [profile::config::validate_session_reference](../../src/config.rs#L1682) | function_item | `private` |  |
| [profile::config::validate_references](../../src/config.rs#L1708) | function_item | `private` |  |
| [profile::config::session_preset_tools](../../src/config.rs#L1757) | function_item | `private` |  |
| [profile::config::apply_scope](../../src/config.rs#L1785) | function_item | `private` |  |
| [profile::config::validate_environments](../../src/config.rs#L1850) | function_item | `private` |  |
| [profile::config::resolve_workspace](../../src/config.rs#L1878) | function_item | `private` |  |
| [profile::config::validate_snapshot](../../src/config.rs#L1932) | function_item | `private` |  |
| [profile::config::validate_safe_integer](../../src/config.rs#L2060) | function_item | `private` |  |
| [profile::config::is_sorted_unique](../../src/config.rs#L2071) | function_item | `private` |  |
| [profile::config::canonical_directory](../../src/config.rs#L2075) | function_item | `private` |  |
| [profile::config::validate_absolute](../../src/config.rs#L2096) | function_item | `private` |  |
| [profile::config::validate_id](../../src/config.rs#L2106) | function_item | `private` |  |
| [profile::config::validate_unique](../../src/config.rs#L2117) | function_item | `private` |  |
| [profile::config::invalid](../../src/config.rs#L2126) | function_item | `private` |  |
| [profile::config::invalid_reference](../../src/config.rs#L2133) | function_item | `private` |  |
| [profile::config::removed](../../src/config.rs#L2140) | function_item | `private` |  |
| [profile::config::lowered](../../src/config.rs#L2145) | function_item | `private` |  |
| [profile::config::enabled_provider_models](../../src/config.rs#L2153) | function_item | `private` |  |
| [profile::config::private_directory_tests::private_directory_rejects_wrong_owner](../../src/config.rs#L2178) | function_item | `private` | test; #[cfg(test)] |
| [profile::config::private_directory_tests::sandbox_fallback_rejects_a_symlinked_path](../../src/config.rs#L2190) | function_item | `private` | test; #[cfg(test)] #[cfg(target_os = "macos")] |
| [profile::config::toolchain_tests::toolchain_roots_are_canonical_deduplicated_and_never_writable_roots](../../src/config.rs#L2212) | function_item | `private` | test; #[cfg(test)] |
| [profile::config::legacy_snapshot_tests::a_snapshot_published_with_the_integrations_file_still_decodes](../../src/config.rs#L2260) | function_item | `private` | test; #[cfg(test)] |

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
| `UnicodeNormalization` | `unicode_normalization::UnicodeNormalization` | `private` |
| `ProfileError` | `crate::ProfileError` | `private` |
| `canonical_line` | `crate::canonical_line` | `private` |
| `digest` | `crate::digest` | `private` |
| `parse_canonical` | `crate::parse_canonical` | `private` |
| `open_private_root` | `super::open_private_root` | `private` |
| `validate_private_directory_fd` | `super::validate_private_directory_fd` | `private` |
| `open_private_root_sandboxed` | `super::open_private_root_sandboxed` | `private` |
| `OFlags` | `rustix::fs::OFlags` | `private` |
| `*` | `super::*` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `profile::config::private_directory_tests` | `private` | #[cfg(test)] |
| `profile::config::toolchain_tests` | `private` | #[cfg(test)] |
| `profile::config::legacy_snapshot_tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::WorkspaceConfig::path_for_binding"]
  n1["profile::config::ProvidersConfig::default"]
  n2["profile::config::validate_environments"]
  n3["profile::config::validate_snapshot"]
  n4["profile::config::removed"]
  n5["profile::config::lowered"]
  n6["profile::config::enabled_provider_models"]
  n7["profile::config::SettingsConfig::default"]
  n8["profile::config::EnvironmentsConfig::default"]
  n9["profile::config::EnvironmentsConfig::canonical_bytes"]
  n10["profile::config::EnvironmentsConfig::decode"]
  n11["profile::config::EnvironmentsConfig::environment"]
  n12["profile::config::Environment::local"]
  n13["profile::config::ignore_legacy_field"]
  n14["profile::config::ConfigSnapshot::protected_roots"]
  n15["profile::config::ConfigSnapshot::execution_cwd"]
  n16["profile::config::ConfigSnapshot::canonical_bytes"]
  n17["profile::config::ConfigSnapshot::digest"]
  n18["profile::config::ConfigSnapshot::decode"]
  n19["profile::config::ConfigSnapshot::publish"]
  n20["profile::config::ConfigSnapshot::requires_respawn_from"]
  n21["profile::config::invalid_private_directory"]
  n22["profile::config::WorkspaceConfig::folder_paths"]
  n23["profile::config::WorkspaceConfig::folder_ids"]
  n24["profile::config::WorkspaceConfig::binding_for_path"]
  n25["profile::canonical_line"]
  n26["profile::digest"]
  n27["profile::parse_canonical"]
  n28["tools::sandbox::kernel_protected_roots"]
  n29["tools::sandbox::kernel_protected_roots_with_user_agent_dir"]
  n9 --> n2
  n9 --> n25
  n10 --> n2
  n10 --> n27
  n14 --> n28
  n14 --> n29
  n16 --> n3
  n16 --> n25
  n17 --> n16
  n17 --> n26
  n18 --> n3
  n18 --> n27
  n19 --> n16
  n20 --> n4
  n20 --> n5
  n20 --> n6
```

</details>

<details><summary>Functions 21–40: 45 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::publish_global"]
  n1["profile::config::ConfigRepository::resolve_unlocked"]
  n2["profile::config::ConfigRepository::read_optional"]
  n3["profile::config::validate_workspace_id"]
  n4["profile::config::validate_environments"]
  n5["profile::config::invalid_private_directory"]
  n6["profile::config::validate_private_directory_fd"]
  n7["profile::config::rustix_profile_error"]
  n8["profile::config::open_directory_at"]
  n9["profile::config::open_private_root"]
  n10["profile::config::open_private_root_darwin"]
  n11["profile::config::open_private_root_sandboxed"]
  n12["profile::config::prepare_repository_directories"]
  n13["profile::config::prepare_private_directory"]
  n14["profile::config::prepare_workspace_directory"]
  n15["profile::config::prepare_existing_workspace_directories"]
  n16["profile::config::ConfigRepository::open"]
  n17["profile::config::ConfigRepository::workspace_data_dir"]
  n18["profile::config::ConfigRepository::root"]
  n19["profile::config::ConfigRepository::resolve"]
  n20["profile::config::ConfigRepository::resolve_for_binding"]
  n21["profile::config::ConfigRepository::resolve_for_session"]
  n22["profile::config::ConfigRepository::resolve_for_session_binding"]
  n23["profile::config::ConfigRepository::resolve_scoped"]
  n24["profile::config::ConfigRepository::environments"]
  n25["profile::config::ConfigRepository::publish_environments"]
  n26["store::platform::NamedLock::shared"]
  n6 --> n5
  n8 --> n5
  n8 --> n6
  n8 --> n7
  n9 --> n5
  n9 --> n6
  n9 --> n7
  n9 --> n10
  n9 --> n11
  n10 --> n5
  n10 --> n6
  n10 --> n7
  n11 --> n5
  n11 --> n6
  n12 --> n8
  n12 --> n9
  n13 --> n5
  n13 --> n8
  n13 --> n9
  n14 --> n3
  n14 --> n13
  n15 --> n14
  n16 --> n12
  n16 --> n15
  n17 --> n3
  n19 --> n1
  n19 --> n3
  n19 --> n26
  n20 --> n1
  n20 --> n3
  n20 --> n26
  n21 --> n1
  n21 --> n3
  n21 --> n26
  n22 --> n1
  n22 --> n3
  n22 --> n26
  n23 --> n1
  n23 --> n3
  n23 --> n26
  n24 --> n2
  n24 --> n4
  n24 --> n26
  n25 --> n0
  n25 --> n4
```

</details>

<details><summary>Functions 41–60: 110 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::resource_workspace_roots"]
  n1["profile::config::ConfigRepository::publish_workspace"]
  n2["profile::config::ConfigRepository::publish_providers"]
  n3["profile::config::ConfigRepository::publish_providers_checked"]
  n4["profile::config::ConfigRepository::publish_settings"]
  n5["profile::config::ConfigRepository::publish_settings_checked"]
  n6["profile::config::ConfigRepository::publish_workspace_policy"]
  n7["profile::config::ConfigRepository::publish_session_settings"]
  n8["profile::config::ConfigRepository::session_settings"]
  n9["profile::config::ConfigRepository::publish_global"]
  n10["profile::config::ConfigRepository::resolve_unlocked"]
  n11["profile::config::ConfigRepository::read_required"]
  n12["profile::config::ConfigRepository::read_optional"]
  n13["profile::config::ConfigRepository::validate_all_provider_references_unlocked"]
  n14["profile::config::check_publication_revision"]
  n15["profile::config::revision_from_value"]
  n16["profile::config::validate_workspace_id"]
  n17["profile::config::workspace_document_path"]
  n18["profile::config::validate_workspace"]
  n19["profile::config::validate_providers"]
  n20["profile::config::validate_settings"]
  n21["profile::config::validate_session_settings"]
  n22["profile::config::validate_session_reference"]
  n23["profile::config::validate_references"]
  n24["profile::config::apply_scope"]
  n25["profile::config::validate_environments"]
  n26["profile::config::resolve_workspace"]
  n27["profile::config::validate_safe_integer"]
  n28["profile::config::canonical_directory"]
  n29["profile::config::validate_id"]
  n30["profile::config::invalid"]
  n31["profile::config::prepare_workspace_directory"]
  n32["profile::config::ConfigRepository::providers"]
  n33["profile::config::ConfigRepository::settings"]
  n34["profile::config::ConfigRepository::workspace"]
  n35["profile::config::ConfigRepository::workspaces"]
  n36["profile::canonical_line"]
  n37["profile::parse_canonical"]
  n38["store::atomic::AtomicPublisher::replace"]
  n39["store::management_root::endpoint_management_root"]
  n40["store::management_root::session_settings_path"]
  n41["store::platform::NamedLock::exclusive"]
  n42["store::platform::NamedLock::shared"]
  n0 --> n11
  n0 --> n16
  n0 --> n17
  n0 --> n18
  n0 --> n26
  n0 --> n42
  n1 --> n12
  n1 --> n14
  n1 --> n16
  n1 --> n17
  n1 --> n18
  n1 --> n31
  n1 --> n36
  n1 --> n38
  n1 --> n41
  n2 --> n9
  n2 --> n19
  n3 --> n12
  n3 --> n13
  n3 --> n14
  n3 --> n19
  n3 --> n36
  n3 --> n38
  n3 --> n39
  n3 --> n41
  n3 --> n42
  n4 --> n9
  n4 --> n20
  n5 --> n12
  n5 --> n14
  n5 --> n19
  n5 --> n20
  n5 --> n23
  n5 --> n36
  n5 --> n38
  n5 --> n41
  n6 --> n11
  n6 --> n12
  n6 --> n14
  n6 --> n16
  n6 --> n17
  n6 --> n18
  n6 --> n19
  n6 --> n20
  n6 --> n23
  n6 --> n36
  n6 --> n38
  n6 --> n41
  n7 --> n12
  n7 --> n14
  n7 --> n19
  n7 --> n21
  n7 --> n22
  n7 --> n36
  n7 --> n38
  n7 --> n40
  n7 --> n41
  n8 --> n12
  n8 --> n21
  n8 --> n40
  n8 --> n42
  n9 --> n14
  n9 --> n15
  n9 --> n36
  n9 --> n37
  n9 --> n38
  n9 --> n41
  n10 --> n11
  n10 --> n12
  n10 --> n17
  n10 --> n18
  n10 --> n19
  n10 --> n20
  n10 --> n21
  n10 --> n22
  n10 --> n23
  n10 --> n24
  n10 --> n25
  n10 --> n26
  n10 --> n28
  n10 --> n29
  n11 --> n37
  n12 --> n37
  n13 --> n11
  n13 --> n12
  n13 --> n18
  n13 --> n20
  n13 --> n21
  n13 --> n22
  n13 --> n23
  n13 --> n30
  n13 --> n40
  n14 --> n27
  n32 --> n12
  n32 --> n19
  n32 --> n42
  n33 --> n12
  n33 --> n20
  n33 --> n42
  n34 --> n11
  n34 --> n16
  n34 --> n17
  n34 --> n18
  n34 --> n30
  n34 --> n42
  n35 --> n11
  n35 --> n16
  n35 --> n18
  n35 --> n30
  n35 --> n42
```

</details>

<details><summary>Functions 61–80: 49 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::validate_workspace_id"]
  n1["profile::config::workspace_document_path"]
  n2["profile::config::validate_common"]
  n3["profile::config::validate_workspace"]
  n4["profile::config::validate_providers"]
  n5["profile::config::validate_web_search_origin"]
  n6["profile::config::validate_settings"]
  n7["profile::config::validate_session_settings"]
  n8["profile::config::validate_session_reference"]
  n9["profile::config::validate_references"]
  n10["profile::config::session_preset_tools"]
  n11["profile::config::apply_scope"]
  n12["profile::config::validate_environments"]
  n13["profile::config::resolve_workspace"]
  n14["profile::config::validate_snapshot"]
  n15["profile::config::validate_safe_integer"]
  n16["profile::config::is_sorted_unique"]
  n17["profile::config::canonical_directory"]
  n18["profile::config::validate_absolute"]
  n19["profile::config::validate_id"]
  n20["profile::config::validate_unique"]
  n21["profile::config::invalid"]
  n22["profile::config::invalid_reference"]
  n23["store::tail::read_genesis"]
  n24["tools::preset::ResolvedPreset::from_genesis"]
  n25["tools::runtime_backends::is_public_internet_address"]
  n2 --> n15
  n3 --> n0
  n3 --> n2
  n3 --> n15
  n3 --> n18
  n3 --> n19
  n3 --> n20
  n3 --> n21
  n4 --> n2
  n4 --> n5
  n4 --> n15
  n4 --> n19
  n4 --> n21
  n5 --> n21
  n5 --> n25
  n6 --> n2
  n6 --> n15
  n6 --> n21
  n7 --> n2
  n7 --> n19
  n7 --> n21
  n8 --> n22
  n9 --> n22
  n10 --> n23
  n10 --> n24
  n11 --> n17
  n11 --> n21
  n12 --> n2
  n12 --> n19
  n12 --> n21
  n13 --> n17
  n13 --> n21
  n14 --> n0
  n14 --> n2
  n14 --> n4
  n14 --> n6
  n14 --> n7
  n14 --> n8
  n14 --> n9
  n14 --> n15
  n14 --> n16
  n14 --> n17
  n14 --> n18
  n14 --> n19
  n14 --> n20
  n14 --> n21
  n15 --> n21
  n17 --> n18
  n19 --> n21
```

</details>

<details><summary>Functions 81–86: 1 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::validate_unique"]
  n1["profile::config::invalid"]
  n2["profile::config::invalid_reference"]
  n3["profile::config::removed"]
  n4["profile::config::lowered"]
  n5["profile::config::enabled_provider_models"]
  n0 --> n1
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `folder_paths` | `self.folders.is_empty` | [69](../../src/config.rs#L69) | receiver-type-required |
| `folder_paths` | `self.cwd.iter().map(String::as_str).collect` | [70](../../src/config.rs#L70) | receiver-type-required |
| `folder_paths` | `self.cwd.iter().map` | [70](../../src/config.rs#L70) | receiver-type-required |
| `folder_paths` | `self.cwd.iter` | [70](../../src/config.rs#L70) | receiver-type-required |
| `folder_paths` | `self.folders                 .iter()                 .map(&#124;folder&#124; folder.path.as_str())                 .collect` | [72](../../src/config.rs#L72) | receiver-type-required |
| `folder_paths` | `self.folders                 .iter()                 .map` | [72](../../src/config.rs#L72) | receiver-type-required |
| `folder_paths` | `self.folders                 .iter` | [72](../../src/config.rs#L72) | receiver-type-required |
| `folder_paths` | `folder.path.as_str` | [74](../../src/config.rs#L74) | receiver-type-required |
| `folder_ids` | `self.folders.is_empty` | [83](../../src/config.rs#L83) | receiver-type-required |
| `folder_ids` | `(1..=self.cwd.len())                 .map(&#124;index&#124; format!("folder-{index:04}"))                 .collect` | [84](../../src/config.rs#L84) | receiver-type-required |
| `folder_ids` | `(1..=self.cwd.len())                 .map` | [84](../../src/config.rs#L84) | receiver-type-required |
| `folder_ids` | `self.cwd.len` | [84](../../src/config.rs#L84) | receiver-type-required |
| `folder_ids` | `self.folders                 .iter()                 .map(&#124;folder&#124; folder.id.clone())                 .collect` | [88](../../src/config.rs#L88) | receiver-type-required |
| `folder_ids` | `self.folders                 .iter()                 .map` | [88](../../src/config.rs#L88) | receiver-type-required |
| `folder_ids` | `self.folders                 .iter` | [88](../../src/config.rs#L88) | receiver-type-required |
| `folder_ids` | `folder.id.clone` | [90](../../src/config.rs#L90) | receiver-type-required |
| `binding_for_path` | `self.folders             .iter()             .find(&#124;folder&#124; folder.path == path)             .map(&#124;folder&#124; folder.id.clone())             .or_else` | [97](../../src/config.rs#L97) | receiver-type-required |
| `binding_for_path` | `self.folders             .iter()             .find(&#124;folder&#124; folder.path == path)             .map` | [97](../../src/config.rs#L97) | receiver-type-required |
| `binding_for_path` | `self.folders             .iter()             .find` | [97](../../src/config.rs#L97) | receiver-type-required |
| `binding_for_path` | `self.folders             .iter` | [97](../../src/config.rs#L97) | receiver-type-required |
| `binding_for_path` | `folder.id.clone` | [100](../../src/config.rs#L100) | receiver-type-required |
| `binding_for_path` | `self.cwd                     .iter()                     .position(&#124;candidate&#124; candidate == path)                     .map` | [102](../../src/config.rs#L102) | receiver-type-required |
| `binding_for_path` | `self.cwd                     .iter()                     .position` | [102](../../src/config.rs#L102) | receiver-type-required |
| `binding_for_path` | `self.cwd                     .iter` | [102](../../src/config.rs#L102) | receiver-type-required |
| `path_for_binding` | `self.folders.is_empty` | [111](../../src/config.rs#L111) | receiver-type-required |
| `path_for_binding` | `self.cwd                 .iter()                 .enumerate()                 .find(&#124;(index, _)&#124; binding == format!("folder-{:04}", index + 1))                 .map` | [112](../../src/config.rs#L112) | receiver-type-required |
| `path_for_binding` | `self.cwd                 .iter()                 .enumerate()                 .find` | [112](../../src/config.rs#L112) | receiver-type-required |
| `path_for_binding` | `self.cwd                 .iter()                 .enumerate` | [112](../../src/config.rs#L112) | receiver-type-required |
| `path_for_binding` | `self.cwd                 .iter` | [112](../../src/config.rs#L112) | receiver-type-required |
| `path_for_binding` | `path.as_str` | [116](../../src/config.rs#L116) | receiver-type-required |
| `path_for_binding` | `self.folders                 .iter()                 .find(&#124;folder&#124; folder.id == binding)                 .map` | [118](../../src/config.rs#L118) | receiver-type-required |
| `path_for_binding` | `self.folders                 .iter()                 .find` | [118](../../src/config.rs#L118) | receiver-type-required |
| `path_for_binding` | `self.folders                 .iter` | [118](../../src/config.rs#L118) | receiver-type-required |
| `path_for_binding` | `folder.path.as_str` | [121](../../src/config.rs#L121) | receiver-type-required |
| `default` | `Vec::new` | [178](../../src/config.rs#L178) | external-constructor-callback-or-unresolved |
| `default` | `Vec::new` | [245](../../src/config.rs#L245) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `Path::new` | [253](../../src/config.rs#L253) | external-constructor-callback-or-unresolved |
| `canonical_bytes` | `validate_environments` | [254](../../src/config.rs#L254) | [profile::config::validate_environments](../../src/config.rs#L1850) |
| `canonical_bytes` | `canonical_line` | [255](../../src/config.rs#L255) | [profile::canonical_line](../../src/lib.rs#L116) |
| `decode` | `Path::new` | [260](../../src/config.rs#L260) | external-constructor-callback-or-unresolved |
| `decode` | `parse_canonical` | [261](../../src/config.rs#L261) | [profile::parse_canonical](../../src/lib.rs#L71) |
| `decode` | `validate_environments` | [262](../../src/config.rs#L262) | [profile::config::validate_environments](../../src/config.rs#L1850) |
| `decode` | `Ok` | [263](../../src/config.rs#L263) | external-constructor-callback-or-unresolved |
| `environment` | `self.environments             .iter()             .find(&#124;environment&#124; environment.id == id)             .cloned()             .or_else` | [270](../../src/config.rs#L270) | receiver-type-required |
| `environment` | `self.environments             .iter()             .find(&#124;environment&#124; environment.id == id)             .cloned` | [270](../../src/config.rs#L270) | receiver-type-required |
| `environment` | `self.environments             .iter()             .find` | [270](../../src/config.rs#L270) | receiver-type-required |
| `environment` | `self.environments             .iter` | [270](../../src/config.rs#L270) | receiver-type-required |
| `environment` | `(id == schema::DEFAULT_ENVIRONMENT).then` | [274](../../src/config.rs#L274) | receiver-type-required |
| `local` | `schema::DEFAULT_ENVIRONMENT.to_owned` | [298](../../src/config.rs#L298) | receiver-type-required |
| `local` | `Vec::new` | [300](../../src/config.rs#L300) | external-constructor-callback-or-unresolved |
| `ignore_legacy_field` | `serde::de::IgnoredAny::deserialize(deserializer).map` | [365](../../src/config.rs#L365) | receiver-type-required |
| `ignore_legacy_field` | `serde::de::IgnoredAny::deserialize` | [365](../../src/config.rs#L365) | external-constructor-callback-or-unresolved |
| `protected_roots` | `self.user_agent_dir.as_deref().map_or_else` | [396](../../src/config.rs#L396) | receiver-type-required |
| `protected_roots` | `self.user_agent_dir.as_deref` | [396](../../src/config.rs#L396) | receiver-type-required |
| `protected_roots` | `tools::kernel_protected_roots` | [397](../../src/config.rs#L397) | [tools::sandbox::kernel_protected_roots](../../../tools/src/sandbox.rs#L151) |
| `protected_roots` | `tools::kernel_protected_roots_with_user_agent_dir` | [398](../../src/config.rs#L398) | [tools::sandbox::kernel_protected_roots_with_user_agent_dir](../../../tools/src/sandbox.rs#L172) |
| `protected_roots` | `Path::new` | [398](../../src/config.rs#L398) | external-constructor-callback-or-unresolved |
| `execution_cwd` | `self.workspace             .selected_cwd             .as_deref()             .or_else` | [404](../../src/config.rs#L404) | receiver-type-required |
| `execution_cwd` | `self.workspace             .selected_cwd             .as_deref` | [404](../../src/config.rs#L404) | receiver-type-required |
| `execution_cwd` | `self.workspace.cwd.first().map` | [407](../../src/config.rs#L407) | receiver-type-required |
| `execution_cwd` | `self.workspace.cwd.first` | [407](../../src/config.rs#L407) | receiver-type-required |
| `canonical_bytes` | `validate_snapshot` | [411](../../src/config.rs#L411) | [profile::config::validate_snapshot](../../src/config.rs#L1932) |
| `canonical_bytes` | `canonical_line` | [412](../../src/config.rs#L412) | [profile::canonical_line](../../src/lib.rs#L116) |
| `digest` | `Ok` | [416](../../src/config.rs#L416) | external-constructor-callback-or-unresolved |
| `digest` | `digest` | [416](../../src/config.rs#L416) | [profile::digest](../../src/lib.rs#L130) |
| `digest` | `self.canonical_bytes` | [416](../../src/config.rs#L416) | [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L410) |
| `decode` | `parse_canonical` | [420](../../src/config.rs#L420) | [profile::parse_canonical](../../src/lib.rs#L71) |
| `decode` | `Err` | [422](../../src/config.rs#L422) | external-constructor-callback-or-unresolved |
| `decode` | `PathBuf::from` | [423](../../src/config.rs#L423) | external-constructor-callback-or-unresolved |
| `decode` | `validate_snapshot` | [427](../../src/config.rs#L427) | [profile::config::validate_snapshot](../../src/config.rs#L1932) |
| `decode` | `Ok` | [428](../../src/config.rs#L428) | external-constructor-callback-or-unresolved |
| `publish` | `self.canonical_bytes` | [432](../../src/config.rs#L432) | [profile::config::ConfigSnapshot::canonical_bytes](../../src/config.rs#L410) |
| `publish` | `digest` | [433](../../src/config.rs#L433) | external-constructor-callback-or-unresolved |
| `publish` | `assets.publish` | [434](../../src/config.rs#L434) | receiver-type-required |
| `publish` | `Err` | [436](../../src/config.rs#L436) | external-constructor-callback-or-unresolved |
| `publish` | `Ok` | [441](../../src/config.rs#L441) | external-constructor-callback-or-unresolved |
| `requires_respawn_from` | `removed` | [449](../../src/config.rs#L449), [451](../../src/config.rs#L451) | [profile::config::removed](../../src/config.rs#L2140) |
| `requires_respawn_from` | `lowered` | [452](../../src/config.rs#L452) | [profile::config::lowered](../../src/config.rs#L2145) |
| `requires_respawn_from` | `enabled_provider_models` | [458](../../src/config.rs#L458), [459](../../src/config.rs#L459) | [profile::config::enabled_provider_models](../../src/config.rs#L2153) |
| `requires_respawn_from` | `old_providers.is_subset` | [460](../../src/config.rs#L460) | receiver-type-required |
| `requires_respawn_from` | `previous                 .providers                 .web_search                 .as_ref()                 .is_some_and` | [461](../../src/config.rs#L461) | receiver-type-required |
| `requires_respawn_from` | `previous                 .providers                 .web_search                 .as_ref` | [461](../../src/config.rs#L461) | receiver-type-required |
| `requires_respawn_from` | `self.providers.web_search.as_ref` | [465](../../src/config.rs#L465) | receiver-type-required |
| `requires_respawn_from` | `Some` | [465](../../src/config.rs#L465) | external-constructor-callback-or-unresolved |
| `invalid_private_directory` | `path.to_path_buf` | [502](../../src/config.rs#L502) | receiver-type-required |
| `invalid_private_directory` | `reason.to_owned` | [503](../../src/config.rs#L503) | receiver-type-required |
| `validate_private_directory_fd` | `fstat(descriptor).map_err` | [512](../../src/config.rs#L512) | receiver-type-required |
| `validate_private_directory_fd` | `fstat` | [512](../../src/config.rs#L512) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `Err` | [514](../../src/config.rs#L514), [520](../../src/config.rs#L520), [526](../../src/config.rs#L526) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `invalid_private_directory` | [514](../../src/config.rs#L514), [520](../../src/config.rs#L520), [526](../../src/config.rs#L526) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `validate_private_directory_fd` | `fchmod(descriptor, Mode::RWXU).map_err` | [533](../../src/config.rs#L533) | receiver-type-required |
| `validate_private_directory_fd` | `fchmod` | [533](../../src/config.rs#L533) | external-constructor-callback-or-unresolved |
| `validate_private_directory_fd` | `Ok` | [534](../../src/config.rs#L534) | external-constructor-callback-or-unresolved |
| `rustix_profile_error` | `std::io::Error::from_raw_os_error(error.raw_os_error()).into` | [538](../../src/config.rs#L538) | receiver-type-required |
| `rustix_profile_error` | `std::io::Error::from_raw_os_error` | [538](../../src/config.rs#L538) | external-constructor-callback-or-unresolved |
| `rustix_profile_error` | `error.raw_os_error` | [538](../../src/config.rs#L538) | receiver-type-required |
| `open_directory_at` | `openat` | [549](../../src/config.rs#L549), [559](../../src/config.rs#L559) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `Mode::empty` | [549](../../src/config.rs#L549), [559](../../src/config.rs#L559) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `result         .as_ref()         .is_err_and` | [550](../../src/config.rs#L550) | receiver-type-required |
| `open_directory_at` | `result         .as_ref` | [550](../../src/config.rs#L550) | receiver-type-required |
| `open_directory_at` | `mkdirat` | [555](../../src/config.rs#L555) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `Err` | [557](../../src/config.rs#L557), [564](../../src/config.rs#L564), [569](../../src/config.rs#L569) | external-constructor-callback-or-unresolved |
| `open_directory_at` | `rustix_profile_error` | [557](../../src/config.rs#L557), [569](../../src/config.rs#L569) | [profile::config::rustix_profile_error](../../src/config.rs#L537) |
| `open_directory_at` | `invalid_private_directory` | [564](../../src/config.rs#L564) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `open_directory_at` | `validate_private_directory_fd` | [571](../../src/config.rs#L571) | [profile::config::validate_private_directory_fd](../../src/config.rs#L507) |
| `open_directory_at` | `Ok` | [572](../../src/config.rs#L572) | external-constructor-callback-or-unresolved |
| `open_private_root` | `path.is_absolute` | [581](../../src/config.rs#L581), [592](../../src/config.rs#L592), [629](../../src/config.rs#L629) | receiver-type-required |
| `open_private_root` | `open_private_root_darwin` | [582](../../src/config.rs#L582) | [profile::config::open_private_root_darwin](../../src/config.rs#L672) |
| `open_private_root` | `path.strip_prefix("/var").map_or_else` | [585](../../src/config.rs#L585) | receiver-type-required |
| `open_private_root` | `path.strip_prefix` | [585](../../src/config.rs#L585) | receiver-type-required |
| `open_private_root` | `path.to_path_buf` | [586](../../src/config.rs#L586), [590](../../src/config.rs#L590) | receiver-type-required |
| `open_private_root` | `Path::new("/private/var").join` | [587](../../src/config.rs#L587) | receiver-type-required |
| `open_private_root` | `Path::new` | [587](../../src/config.rs#L587) | external-constructor-callback-or-unresolved |
| `open_private_root` | `open` | [593](../../src/config.rs#L593), [602](../../src/config.rs#L602) | external-constructor-callback-or-unresolved |
| `open_private_root` | `Mode::empty` | [593](../../src/config.rs#L593), [602](../../src/config.rs#L602), [637](../../src/config.rs#L637), [648](../../src/config.rs#L648) | external-constructor-callback-or-unresolved |
| `open_private_root` | `open_private_root_sandboxed` | [597](../../src/config.rs#L597), [654](../../src/config.rs#L654) | [profile::config::open_private_root_sandboxed](../../src/config.rs#L737) |
| `open_private_root` | `Err` | [599](../../src/config.rs#L599), [615](../../src/config.rs#L615), [623](../../src/config.rs#L623), [646](../../src/config.rs#L646), [659](../../src/config.rs#L659), [664](../../src/config.rs#L664) | external-constructor-callback-or-unresolved |
| `open_private_root` | `rustix_profile_error` | [599](../../src/config.rs#L599), [646](../../src/config.rs#L646), [664](../../src/config.rs#L664) | [profile::config::rustix_profile_error](../../src/config.rs#L537) |
| `open_private_root` | `open(".", flags, Mode::empty()).map_err` | [602](../../src/config.rs#L602) | receiver-type-required |
| `open_private_root` | `Vec::new` | [604](../../src/config.rs#L604) | external-constructor-callback-or-unresolved |
| `open_private_root` | `traversal_path.components` | [605](../../src/config.rs#L605) | receiver-type-required |
| `open_private_root` | `value.to_str().ok_or_else` | [609](../../src/config.rs#L609) | receiver-type-required |
| `open_private_root` | `value.to_str` | [609](../../src/config.rs#L609) | receiver-type-required |
| `open_private_root` | `invalid_private_directory` | [610](../../src/config.rs#L610), [615](../../src/config.rs#L615), [623](../../src/config.rs#L623), [659](../../src/config.rs#L659) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `open_private_root` | `components.push` | [612](../../src/config.rs#L612) | receiver-type-required |
| `open_private_root` | `value.to_owned` | [612](../../src/config.rs#L612) | receiver-type-required |
| `open_private_root` | `components.is_empty` | [622](../../src/config.rs#L622) | receiver-type-required |
| `open_private_root` | `PathBuf::from` | [630](../../src/config.rs#L630), [632](../../src/config.rs#L632) | external-constructor-callback-or-unresolved |
| `open_private_root` | `components.len` | [634](../../src/config.rs#L634) | receiver-type-required |
| `open_private_root` | `components.into_iter().enumerate` | [635](../../src/config.rs#L635) | receiver-type-required |
| `open_private_root` | `components.into_iter` | [635](../../src/config.rs#L635) | receiver-type-required |
| `open_private_root` | `traversed.push` | [636](../../src/config.rs#L636) | receiver-type-required |
| `open_private_root` | `openat` | [637](../../src/config.rs#L637), [648](../../src/config.rs#L648) | external-constructor-callback-or-unresolved |
| `open_private_root` | `result             .as_ref()             .is_err_and` | [638](../../src/config.rs#L638) | receiver-type-required |
| `open_private_root` | `result             .as_ref` | [638](../../src/config.rs#L638) | receiver-type-required |
| `open_private_root` | `mkdirat` | [644](../../src/config.rs#L644) | external-constructor-callback-or-unresolved |
| `open_private_root` | `validate_private_directory_fd` | [667](../../src/config.rs#L667) | [profile::config::validate_private_directory_fd](../../src/config.rs#L507) |
| `open_private_root` | `Ok` | [668](../../src/config.rs#L668) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `path.strip_prefix("/var").map_or_else` | [677](../../src/config.rs#L677) | receiver-type-required |
| `open_private_root_darwin` | `path.strip_prefix` | [677](../../src/config.rs#L677) | receiver-type-required |
| `open_private_root_darwin` | `path.to_path_buf` | [678](../../src/config.rs#L678) | receiver-type-required |
| `open_private_root_darwin` | `Path::new("/private/var").join` | [679](../../src/config.rs#L679) | receiver-type-required |
| `open_private_root_darwin` | `Path::new` | [679](../../src/config.rs#L679) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `path.components().any` | [681](../../src/config.rs#L681) | receiver-type-required |
| `open_private_root_darwin` | `path.components` | [681](../../src/config.rs#L681) | receiver-type-required |
| `open_private_root_darwin` | `Err` | [687](../../src/config.rs#L687), [693](../../src/config.rs#L693), [720](../../src/config.rs#L720), [725](../../src/config.rs#L725), [730](../../src/config.rs#L730) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `invalid_private_directory` | [687](../../src/config.rs#L687), [693](../../src/config.rs#L693), [712](../../src/config.rs#L712), [717](../../src/config.rs#L717), [725](../../src/config.rs#L725) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `open_private_root_darwin` | `path.file_name().is_none` | [692](../../src/config.rs#L692) | receiver-type-required |
| `open_private_root_darwin` | `path.file_name` | [692](../../src/config.rs#L692) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read(true)             .custom_flags(libc::O_DIRECTORY &#124; libc::O_NOFOLLOW_ANY &#124; libc::O_CLOEXEC)             .open` | [702](../../src/config.rs#L702) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read(true)             .custom_flags` | [702](../../src/config.rs#L702) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new()             .read` | [702](../../src/config.rs#L702) | receiver-type-required |
| `open_private_root_darwin` | `fs::OpenOptions::new` | [702](../../src/config.rs#L702) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `Ok` | [706](../../src/config.rs#L706), [733](../../src/config.rs#L733) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `file.into` | [706](../../src/config.rs#L706) | receiver-type-required |
| `open_private_root_darwin` | `open_directory` | [708](../../src/config.rs#L708), [714](../../src/config.rs#L714), [722](../../src/config.rs#L722) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `error.kind` | [710](../../src/config.rs#L710) | receiver-type-required |
| `open_private_root_darwin` | `path.parent().ok_or_else` | [711](../../src/config.rs#L711) | receiver-type-required |
| `open_private_root_darwin` | `path.parent` | [711](../../src/config.rs#L711) | receiver-type-required |
| `open_private_root_darwin` | `open_directory(parent_path).map_err` | [714](../../src/config.rs#L714) | receiver-type-required |
| `open_private_root_darwin` | `path                 .file_name()                 .ok_or_else` | [715](../../src/config.rs#L715) | receiver-type-required |
| `open_private_root_darwin` | `path                 .file_name` | [715](../../src/config.rs#L715) | receiver-type-required |
| `open_private_root_darwin` | `mkdirat` | [718](../../src/config.rs#L718) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `rustix_profile_error` | [720](../../src/config.rs#L720) | [profile::config::rustix_profile_error](../../src/config.rs#L537) |
| `open_private_root_darwin` | `open_directory(&path).map_err` | [722](../../src/config.rs#L722) | receiver-type-required |
| `open_private_root_darwin` | `error.raw_os_error` | [724](../../src/config.rs#L724) | receiver-type-required |
| `open_private_root_darwin` | `Some` | [724](../../src/config.rs#L724) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `ProfileError::from` | [730](../../src/config.rs#L730) | external-constructor-callback-or-unresolved |
| `open_private_root_darwin` | `validate_private_directory_fd` | [732](../../src/config.rs#L732) | [profile::config::validate_private_directory_fd](../../src/config.rs#L507) |
| `open_private_root_sandboxed` | `fs::canonicalize` | [743](../../src/config.rs#L743) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `Err` | [745](../../src/config.rs#L745) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `invalid_private_directory` | [745](../../src/config.rs#L745) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `open_private_root_sandboxed` | `open(&canonical, flags, Mode::empty()).map_err` | [750](../../src/config.rs#L750) | receiver-type-required |
| `open_private_root_sandboxed` | `open` | [750](../../src/config.rs#L750) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `Mode::empty` | [750](../../src/config.rs#L750) | external-constructor-callback-or-unresolved |
| `open_private_root_sandboxed` | `validate_private_directory_fd` | [751](../../src/config.rs#L751) | [profile::config::validate_private_directory_fd](../../src/config.rs#L507) |
| `open_private_root_sandboxed` | `Ok` | [752](../../src/config.rs#L752) | external-constructor-callback-or-unresolved |
| `prepare_repository_directories` | `rustix::process::geteuid().as_raw` | [756](../../src/config.rs#L756) | receiver-type-required |
| `prepare_repository_directories` | `rustix::process::geteuid` | [756](../../src/config.rs#L756) | external-constructor-callback-or-unresolved |
| `prepare_repository_directories` | `open_private_root` | [757](../../src/config.rs#L757) | [profile::config::open_private_root](../../src/config.rs#L575) |
| `prepare_repository_directories` | `open_directory_at` | [759](../../src/config.rs#L759) | [profile::config::open_directory_at](../../src/config.rs#L541) |
| `prepare_repository_directories` | `root.join` | [759](../../src/config.rs#L759) | receiver-type-required |
| `prepare_repository_directories` | `Ok` | [761](../../src/config.rs#L761) | external-constructor-callback-or-unresolved |
| `prepare_private_directory` | `path         .parent()         .ok_or_else` | [765](../../src/config.rs#L765) | receiver-type-required |
| `prepare_private_directory` | `path         .parent` | [765](../../src/config.rs#L765) | receiver-type-required |
| `prepare_private_directory` | `invalid_private_directory` | [767](../../src/config.rs#L767), [772](../../src/config.rs#L772) | [profile::config::invalid_private_directory](../../src/config.rs#L500) |
| `prepare_private_directory` | `path         .file_name()         .and_then(std::ffi::OsStr::to_str)         .ok_or_else` | [768](../../src/config.rs#L768) | receiver-type-required |
| `prepare_private_directory` | `path         .file_name()         .and_then` | [768](../../src/config.rs#L768) | receiver-type-required |
| `prepare_private_directory` | `path         .file_name` | [768](../../src/config.rs#L768) | receiver-type-required |
| `prepare_private_directory` | `rustix::process::geteuid().as_raw` | [774](../../src/config.rs#L774) | receiver-type-required |
| `prepare_private_directory` | `rustix::process::geteuid` | [774](../../src/config.rs#L774) | external-constructor-callback-or-unresolved |
| `prepare_private_directory` | `open_private_root` | [775](../../src/config.rs#L775) | [profile::config::open_private_root](../../src/config.rs#L575) |
| `prepare_private_directory` | `open_directory_at` | [776](../../src/config.rs#L776) | [profile::config::open_directory_at](../../src/config.rs#L541) |
| `prepare_private_directory` | `Ok` | [777](../../src/config.rs#L777) | external-constructor-callback-or-unresolved |
| `prepare_workspace_directory` | `validate_workspace_id` | [781](../../src/config.rs#L781) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `prepare_workspace_directory` | `root.join("workspaces").join` | [782](../../src/config.rs#L782) | receiver-type-required |
| `prepare_workspace_directory` | `root.join` | [782](../../src/config.rs#L782) | receiver-type-required |
| `prepare_workspace_directory` | `prepare_private_directory` | [783](../../src/config.rs#L783), [785](../../src/config.rs#L785) | [profile::config::prepare_private_directory](../../src/config.rs#L764) |
| `prepare_workspace_directory` | `directory.join` | [785](../../src/config.rs#L785) | receiver-type-required |
| `prepare_workspace_directory` | `Ok` | [787](../../src/config.rs#L787) | external-constructor-callback-or-unresolved |
| `prepare_existing_workspace_directories` | `root.join` | [791](../../src/config.rs#L791) | receiver-type-required |
| `prepare_existing_workspace_directories` | `fs::read_dir` | [792](../../src/config.rs#L792) | external-constructor-callback-or-unresolved |
| `prepare_existing_workspace_directories` | `entry.path` | [794](../../src/config.rs#L794), [805](../../src/config.rs#L805) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry.file_type()?.is_symlink` | [795](../../src/config.rs#L795) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry.file_type` | [795](../../src/config.rs#L795) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry.file_type()?.is_dir` | [795](../../src/config.rs#L795) | receiver-type-required |
| `prepare_existing_workspace_directories` | `Err` | [796](../../src/config.rs#L796) | external-constructor-callback-or-unresolved |
| `prepare_existing_workspace_directories` | `"workspace authority entry must be a directory".to_owned` | [798](../../src/config.rs#L798) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry             .file_name()             .into_string()             .map_err` | [801](../../src/config.rs#L801) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry             .file_name()             .into_string` | [801](../../src/config.rs#L801) | receiver-type-required |
| `prepare_existing_workspace_directories` | `entry             .file_name` | [801](../../src/config.rs#L801) | receiver-type-required |
| `prepare_existing_workspace_directories` | `"workspace authority directory must be UTF-8".to_owned` | [806](../../src/config.rs#L806) | receiver-type-required |
| `prepare_existing_workspace_directories` | `prepare_workspace_directory` | [808](../../src/config.rs#L808) | [profile::config::prepare_workspace_directory](../../src/config.rs#L780) |
| `prepare_existing_workspace_directories` | `Ok` | [810](../../src/config.rs#L810) | external-constructor-callback-or-unresolved |
| `open` | `root.as_ref().to_path_buf` | [815](../../src/config.rs#L815) | receiver-type-required |
| `open` | `root.as_ref` | [815](../../src/config.rs#L815) | receiver-type-required |
| `open` | `prepare_repository_directories` | [816](../../src/config.rs#L816) | [profile::config::prepare_repository_directories](../../src/config.rs#L755) |
| `open` | `prepare_existing_workspace_directories` | [817](../../src/config.rs#L817) | [profile::config::prepare_existing_workspace_directories](../../src/config.rs#L790) |
| `open` | `Ok` | [818](../../src/config.rs#L818) | external-constructor-callback-or-unresolved |
| `workspace_data_dir` | `validate_workspace_id` | [822](../../src/config.rs#L822) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `workspace_data_dir` | `Ok` | [823](../../src/config.rs#L823) | external-constructor-callback-or-unresolved |
| `workspace_data_dir` | `self.root.join("workspaces").join` | [823](../../src/config.rs#L823) | receiver-type-required |
| `workspace_data_dir` | `self.root.join` | [823](../../src/config.rs#L823) | receiver-type-required |
| `resolve` | `validate_workspace_id` | [832](../../src/config.rs#L832) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resolve` | `NamedLock::shared` | [833](../../src/config.rs#L833) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve` | `self.root.join` | [833](../../src/config.rs#L833) | receiver-type-required |
| `resolve` | `self.resolve_unlocked` | [834](../../src/config.rs#L834) | [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) |
| `resolve_for_binding` | `validate_workspace_id` | [845](../../src/config.rs#L845) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resolve_for_binding` | `NamedLock::shared` | [846](../../src/config.rs#L846) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve_for_binding` | `self.root.join` | [846](../../src/config.rs#L846) | receiver-type-required |
| `resolve_for_binding` | `self.resolve_unlocked` | [847](../../src/config.rs#L847) | [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) |
| `resolve_for_binding` | `Some` | [847](../../src/config.rs#L847) | external-constructor-callback-or-unresolved |
| `resolve_for_session` | `validate_workspace_id` | [855](../../src/config.rs#L855) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resolve_for_session` | `NamedLock::shared` | [856](../../src/config.rs#L856) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve_for_session` | `self.root.join` | [856](../../src/config.rs#L856) | receiver-type-required |
| `resolve_for_session` | `self.resolve_unlocked` | [858](../../src/config.rs#L858) | [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) |
| `resolve_for_session` | `Some` | [858](../../src/config.rs#L858) | external-constructor-callback-or-unresolved |
| `resolve_for_session` | `session_folder.as_ref` | [858](../../src/config.rs#L858), [861](../../src/config.rs#L861) | receiver-type-required |
| `resolve_for_session` | `snapshot.workspace.cwd.len` | [859](../../src/config.rs#L859) | receiver-type-required |
| `resolve_for_session` | `Err` | [860](../../src/config.rs#L860) | external-constructor-callback-or-unresolved |
| `resolve_for_session` | `session_folder.as_ref().to_path_buf` | [861](../../src/config.rs#L861) | receiver-type-required |
| `resolve_for_session` | `"multi-folder session has no stable folder binding".to_owned` | [862](../../src/config.rs#L862) | receiver-type-required |
| `resolve_for_session` | `Ok` | [865](../../src/config.rs#L865) | external-constructor-callback-or-unresolved |
| `resolve_for_session_binding` | `validate_workspace_id` | [877](../../src/config.rs#L877) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resolve_for_session_binding` | `NamedLock::shared` | [878](../../src/config.rs#L878) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve_for_session_binding` | `self.root.join` | [878](../../src/config.rs#L878) | receiver-type-required |
| `resolve_for_session_binding` | `self.resolve_unlocked` | [879](../../src/config.rs#L879) | [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) |
| `resolve_for_session_binding` | `Some` | [881](../../src/config.rs#L881), [882](../../src/config.rs#L882) | external-constructor-callback-or-unresolved |
| `resolve_for_session_binding` | `session_folder.as_ref` | [881](../../src/config.rs#L881) | receiver-type-required |
| `resolve_scoped` | `validate_workspace_id` | [899](../../src/config.rs#L899) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resolve_scoped` | `NamedLock::shared` | [900](../../src/config.rs#L900) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resolve_scoped` | `self.root.join` | [900](../../src/config.rs#L900) | receiver-type-required |
| `resolve_scoped` | `self.resolve_unlocked` | [902](../../src/config.rs#L902) | [profile::config::ConfigRepository::resolve_unlocked](../../src/config.rs#L1258) |
| `resolve_scoped` | `Some` | [902](../../src/config.rs#L902) | external-constructor-callback-or-unresolved |
| `resolve_scoped` | `folder_binding.is_none` | [904](../../src/config.rs#L904) | receiver-type-required |
| `resolve_scoped` | `snapshot.workspace.cwd.len` | [905](../../src/config.rs#L905) | receiver-type-required |
| `resolve_scoped` | `Err` | [907](../../src/config.rs#L907) | external-constructor-callback-or-unresolved |
| `resolve_scoped` | `session_folder.to_path_buf` | [908](../../src/config.rs#L908) | receiver-type-required |
| `resolve_scoped` | `"multi-folder session has no stable folder binding".to_owned` | [909](../../src/config.rs#L909) | receiver-type-required |
| `resolve_scoped` | `Ok` | [912](../../src/config.rs#L912) | external-constructor-callback-or-unresolved |
| `environments` | `NamedLock::shared` | [918](../../src/config.rs#L918) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `environments` | `self.root.join` | [918](../../src/config.rs#L918), [919](../../src/config.rs#L919) | receiver-type-required |
| `environments` | `self.read_optional(&path)?.unwrap_or_default` | [920](../../src/config.rs#L920) | receiver-type-required |
| `environments` | `self.read_optional` | [920](../../src/config.rs#L920) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `environments` | `validate_environments` | [921](../../src/config.rs#L921) | [profile::config::validate_environments](../../src/config.rs#L1850) |
| `environments` | `Ok` | [922](../../src/config.rs#L922) | external-constructor-callback-or-unresolved |
| `publish_environments` | `self.root.join` | [930](../../src/config.rs#L930) | receiver-type-required |
| `publish_environments` | `self.publish_global` | [931](../../src/config.rs#L931) | [profile::config::ConfigRepository::publish_global](../../src/config.rs#L1234) |
| `publish_environments` | `validate_environments` | [932](../../src/config.rs#L932) | [profile::config::validate_environments](../../src/config.rs#L1850) |
| `providers` | `NamedLock::shared` | [939](../../src/config.rs#L939) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `providers` | `self.root.join` | [939](../../src/config.rs#L939), [940](../../src/config.rs#L940) | receiver-type-required |
| `providers` | `self.read_optional(&path)?.unwrap_or_default` | [941](../../src/config.rs#L941) | receiver-type-required |
| `providers` | `self.read_optional` | [941](../../src/config.rs#L941) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `providers` | `validate_providers` | [942](../../src/config.rs#L942) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `providers` | `Ok` | [943](../../src/config.rs#L943) | external-constructor-callback-or-unresolved |
| `settings` | `NamedLock::shared` | [948](../../src/config.rs#L948) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `settings` | `self.root.join` | [948](../../src/config.rs#L948), [949](../../src/config.rs#L949) | receiver-type-required |
| `settings` | `self.read_optional(&path)?.unwrap_or_default` | [950](../../src/config.rs#L950) | receiver-type-required |
| `settings` | `self.read_optional` | [950](../../src/config.rs#L950) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `settings` | `validate_settings` | [951](../../src/config.rs#L951) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `settings` | `Ok` | [952](../../src/config.rs#L952) | external-constructor-callback-or-unresolved |
| `workspace` | `validate_workspace_id` | [958](../../src/config.rs#L958) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `workspace` | `NamedLock::shared` | [959](../../src/config.rs#L959) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `workspace` | `self.root.join` | [959](../../src/config.rs#L959) | receiver-type-required |
| `workspace` | `workspace_document_path` | [960](../../src/config.rs#L960) | [profile::config::workspace_document_path](../../src/config.rs#L1481) |
| `workspace` | `self.read_required` | [961](../../src/config.rs#L961) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `workspace` | `validate_workspace` | [962](../../src/config.rs#L962) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `workspace` | `invalid` | [964](../../src/config.rs#L964) | [profile::config::invalid](../../src/config.rs#L2126) |
| `workspace` | `Ok` | [966](../../src/config.rs#L966) | external-constructor-callback-or-unresolved |
| `workspaces` | `NamedLock::shared` | [973](../../src/config.rs#L973) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `workspaces` | `self.root.join` | [973](../../src/config.rs#L973), [975](../../src/config.rs#L975) | receiver-type-required |
| `workspaces` | `fs::read_dir(self.root.join("workspaces"))?.collect::<Result<Vec<_>, _>>` | [975](../../src/config.rs#L975) | receiver-type-required |
| `workspaces` | `fs::read_dir` | [975](../../src/config.rs#L975) | external-constructor-callback-or-unresolved |
| `workspaces` | `entries.sort_by_key` | [976](../../src/config.rs#L976) | receiver-type-required |
| `workspaces` | `Vec::with_capacity` | [977](../../src/config.rs#L977) | external-constructor-callback-or-unresolved |
| `workspaces` | `entries.len` | [977](../../src/config.rs#L977) | receiver-type-required |
| `workspaces` | `entry.path` | [979](../../src/config.rs#L979) | receiver-type-required |
| `workspaces` | `entry.file_type()?.is_symlink` | [980](../../src/config.rs#L980) | receiver-type-required |
| `workspaces` | `entry.file_type` | [980](../../src/config.rs#L980) | receiver-type-required |
| `workspaces` | `entry.file_type()?.is_dir` | [980](../../src/config.rs#L980) | receiver-type-required |
| `workspaces` | `invalid` | [981](../../src/config.rs#L981), [996](../../src/config.rs#L996) | [profile::config::invalid](../../src/config.rs#L2126) |
| `workspaces` | `entry                     .file_name()                     .into_string()                     .map_err` | [984](../../src/config.rs#L984) | receiver-type-required |
| `workspaces` | `entry                     .file_name()                     .into_string` | [984](../../src/config.rs#L984) | receiver-type-required |
| `workspaces` | `entry                     .file_name` | [984](../../src/config.rs#L984) | receiver-type-required |
| `workspaces` | `directory.clone` | [988](../../src/config.rs#L988) | receiver-type-required |
| `workspaces` | `"workspace authority directory must be UTF-8".to_owned` | [989](../../src/config.rs#L989) | receiver-type-required |
| `workspaces` | `validate_workspace_id` | [991](../../src/config.rs#L991) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `workspaces` | `directory.join` | [992](../../src/config.rs#L992) | receiver-type-required |
| `workspaces` | `self.read_required` | [993](../../src/config.rs#L993) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `workspaces` | `validate_workspace` | [994](../../src/config.rs#L994) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `workspaces` | `values.push` | [998](../../src/config.rs#L998) | receiver-type-required |
| `workspaces` | `Ok` | [1000](../../src/config.rs#L1000) | external-constructor-callback-or-unresolved |
| `resource_workspace_roots` | `NamedLock::shared` | [1007](../../src/config.rs#L1007) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `resource_workspace_roots` | `self.root.join` | [1007](../../src/config.rs#L1007), [1008](../../src/config.rs#L1008) | receiver-type-required |
| `resource_workspace_roots` | `fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>` | [1009](../../src/config.rs#L1009) | receiver-type-required |
| `resource_workspace_roots` | `fs::read_dir` | [1009](../../src/config.rs#L1009) | external-constructor-callback-or-unresolved |
| `resource_workspace_roots` | `entries.sort_by_key` | [1010](../../src/config.rs#L1010) | receiver-type-required |
| `resource_workspace_roots` | `Vec::new` | [1011](../../src/config.rs#L1011) | external-constructor-callback-or-unresolved |
| `resource_workspace_roots` | `entry.path` | [1013](../../src/config.rs#L1013) | receiver-type-required |
| `resource_workspace_roots` | `entry.file_type()?.is_dir` | [1014](../../src/config.rs#L1014) | receiver-type-required |
| `resource_workspace_roots` | `entry.file_type` | [1014](../../src/config.rs#L1014) | receiver-type-required |
| `resource_workspace_roots` | `Err` | [1015](../../src/config.rs#L1015), [1032](../../src/config.rs#L1032) | external-constructor-callback-or-unresolved |
| `resource_workspace_roots` | `"workspace authority entry must be a directory".to_owned` | [1017](../../src/config.rs#L1017) | receiver-type-required |
| `resource_workspace_roots` | `directory_path                 .file_name()                 .and_then(&#124;value&#124; value.to_str())                 .ok_or_else` | [1020](../../src/config.rs#L1020) | receiver-type-required |
| `resource_workspace_roots` | `directory_path                 .file_name()                 .and_then` | [1020](../../src/config.rs#L1020) | receiver-type-required |
| `resource_workspace_roots` | `directory_path                 .file_name` | [1020](../../src/config.rs#L1020) | receiver-type-required |
| `resource_workspace_roots` | `value.to_str` | [1022](../../src/config.rs#L1022) | receiver-type-required |
| `resource_workspace_roots` | `directory_path.clone` | [1024](../../src/config.rs#L1024) | receiver-type-required |
| `resource_workspace_roots` | `"workspace authority directory must be UTF-8".to_owned` | [1025](../../src/config.rs#L1025) | receiver-type-required |
| `resource_workspace_roots` | `validate_workspace_id` | [1027](../../src/config.rs#L1027) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `resource_workspace_roots` | `workspace_document_path` | [1028](../../src/config.rs#L1028) | [profile::config::workspace_document_path](../../src/config.rs#L1481) |
| `resource_workspace_roots` | `self.read_required` | [1029](../../src/config.rs#L1029) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `resource_workspace_roots` | `validate_workspace` | [1030](../../src/config.rs#L1030) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `resource_workspace_roots` | `"workspace id does not match filename".to_owned` | [1034](../../src/config.rs#L1034) | receiver-type-required |
| `resource_workspace_roots` | `resolve_workspace(&workspace, &path)?                 .cwd                 .into_iter()                 .map(PathBuf::from)                 .collect::<Vec<_>>` | [1037](../../src/config.rs#L1037) | receiver-type-required |
| `resource_workspace_roots` | `resolve_workspace(&workspace, &path)?                 .cwd                 .into_iter()                 .map` | [1037](../../src/config.rs#L1037) | receiver-type-required |
| `resource_workspace_roots` | `resolve_workspace(&workspace, &path)?                 .cwd                 .into_iter` | [1037](../../src/config.rs#L1037) | receiver-type-required |
| `resource_workspace_roots` | `resolve_workspace` | [1037](../../src/config.rs#L1037) | [profile::config::resolve_workspace](../../src/config.rs#L1878) |
| `resource_workspace_roots` | `workspaces.push` | [1042](../../src/config.rs#L1042) | receiver-type-required |
| `resource_workspace_roots` | `workspace_id.to_owned` | [1042](../../src/config.rs#L1042) | receiver-type-required |
| `resource_workspace_roots` | `Ok` | [1044](../../src/config.rs#L1044) | external-constructor-callback-or-unresolved |
| `publish_workspace` | `validate_workspace_id` | [1052](../../src/config.rs#L1052) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `publish_workspace` | `workspace_document_path` | [1053](../../src/config.rs#L1053) | [profile::config::workspace_document_path](../../src/config.rs#L1481) |
| `publish_workspace` | `NamedLock::exclusive` | [1054](../../src/config.rs#L1054) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_workspace` | `self.root.join` | [1054](../../src/config.rs#L1054) | receiver-type-required |
| `publish_workspace` | `prepare_workspace_directory` | [1055](../../src/config.rs#L1055) | [profile::config::prepare_workspace_directory](../../src/config.rs#L780) |
| `publish_workspace` | `self.read_optional::<WorkspaceConfig>` | [1056](../../src/config.rs#L1056) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_workspace` | `validate_workspace` | [1058](../../src/config.rs#L1058), [1064](../../src/config.rs#L1064) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `publish_workspace` | `check_publication_revision` | [1063](../../src/config.rs#L1063) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_workspace` | `AtomicPublisher::replace` | [1065](../../src/config.rs#L1065) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_workspace` | `canonical_line` | [1065](../../src/config.rs#L1065) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_workspace` | `Ok` | [1066](../../src/config.rs#L1066) | external-constructor-callback-or-unresolved |
| `publish_providers` | `self.root.join` | [1074](../../src/config.rs#L1074) | receiver-type-required |
| `publish_providers` | `self.publish_global` | [1075](../../src/config.rs#L1075) | [profile::config::ConfigRepository::publish_global](../../src/config.rs#L1234) |
| `publish_providers` | `validate_providers` | [1076](../../src/config.rs#L1076) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `publish_providers_checked` | `self.root.join` | [1089](../../src/config.rs#L1089), [1098](../../src/config.rs#L1098) | receiver-type-required |
| `publish_providers_checked` | `store::endpoint_management_root(&self.root)?.join` | [1093](../../src/config.rs#L1093) | receiver-type-required |
| `publish_providers_checked` | `store::endpoint_management_root` | [1093](../../src/config.rs#L1093) | [store::management_root::endpoint_management_root](../../../store/src/management_root.rs#L47) |
| `publish_providers_checked` | `management_path             .exists()             .then(&#124;&#124; NamedLock::shared(&management_path))             .transpose` | [1094](../../src/config.rs#L1094) | receiver-type-required |
| `publish_providers_checked` | `management_path             .exists()             .then` | [1094](../../src/config.rs#L1094) | receiver-type-required |
| `publish_providers_checked` | `management_path             .exists` | [1094](../../src/config.rs#L1094) | receiver-type-required |
| `publish_providers_checked` | `NamedLock::shared` | [1096](../../src/config.rs#L1096) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `publish_providers_checked` | `NamedLock::exclusive` | [1098](../../src/config.rs#L1098) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_providers_checked` | `self             .read_optional::<ProvidersConfig>(&path)?             .unwrap_or_default` | [1099](../../src/config.rs#L1099) | receiver-type-required |
| `publish_providers_checked` | `self             .read_optional::<ProvidersConfig>` | [1099](../../src/config.rs#L1099) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_providers_checked` | `validate_providers` | [1102](../../src/config.rs#L1102), [1104](../../src/config.rs#L1104) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `publish_providers_checked` | `check_publication_revision` | [1103](../../src/config.rs#L1103) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_providers_checked` | `self.validate_all_provider_references_unlocked` | [1105](../../src/config.rs#L1105) | [profile::config::ConfigRepository::validate_all_provider_references_unlocked](../../src/config.rs#L1377) |
| `publish_providers_checked` | `AtomicPublisher::replace` | [1106](../../src/config.rs#L1106) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_providers_checked` | `canonical_line` | [1106](../../src/config.rs#L1106) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_providers_checked` | `Ok` | [1107](../../src/config.rs#L1107) | external-constructor-callback-or-unresolved |
| `publish_settings` | `self.root.join` | [1115](../../src/config.rs#L1115) | receiver-type-required |
| `publish_settings` | `self.publish_global` | [1116](../../src/config.rs#L1116) | [profile::config::ConfigRepository::publish_global](../../src/config.rs#L1234) |
| `publish_settings` | `validate_settings` | [1117](../../src/config.rs#L1117) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `publish_settings_checked` | `self.root.join` | [1128](../../src/config.rs#L1128), [1129](../../src/config.rs#L1129), [1130](../../src/config.rs#L1130) | receiver-type-required |
| `publish_settings_checked` | `NamedLock::exclusive` | [1130](../../src/config.rs#L1130) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_settings_checked` | `self             .read_optional::<SettingsConfig>(&path)?             .unwrap_or_default` | [1131](../../src/config.rs#L1131) | receiver-type-required |
| `publish_settings_checked` | `self             .read_optional::<SettingsConfig>` | [1131](../../src/config.rs#L1131) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_settings_checked` | `validate_settings` | [1134](../../src/config.rs#L1134), [1136](../../src/config.rs#L1136) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `publish_settings_checked` | `check_publication_revision` | [1135](../../src/config.rs#L1135) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_settings_checked` | `self             .read_optional::<ProvidersConfig>(&providers_path)?             .unwrap_or_default` | [1137](../../src/config.rs#L1137) | receiver-type-required |
| `publish_settings_checked` | `self             .read_optional::<ProvidersConfig>` | [1137](../../src/config.rs#L1137) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_settings_checked` | `validate_providers` | [1140](../../src/config.rs#L1140) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `publish_settings_checked` | `"reference-check".to_owned` | [1144](../../src/config.rs#L1144), [1145](../../src/config.rs#L1145) | receiver-type-required |
| `publish_settings_checked` | `Vec::new` | [1147](../../src/config.rs#L1147) | external-constructor-callback-or-unresolved |
| `publish_settings_checked` | `validate_references` | [1150](../../src/config.rs#L1150) | [profile::config::validate_references](../../src/config.rs#L1708) |
| `publish_settings_checked` | `AtomicPublisher::replace` | [1151](../../src/config.rs#L1151) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_settings_checked` | `canonical_line` | [1151](../../src/config.rs#L1151) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_settings_checked` | `Ok` | [1152](../../src/config.rs#L1152) | external-constructor-callback-or-unresolved |
| `publish_workspace_policy` | `validate_workspace_id` | [1164](../../src/config.rs#L1164) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `publish_workspace_policy` | `workspace_document_path` | [1165](../../src/config.rs#L1165) | [profile::config::workspace_document_path](../../src/config.rs#L1481) |
| `publish_workspace_policy` | `self.root.join` | [1166](../../src/config.rs#L1166), [1167](../../src/config.rs#L1167), [1168](../../src/config.rs#L1168) | receiver-type-required |
| `publish_workspace_policy` | `NamedLock::exclusive` | [1168](../../src/config.rs#L1168) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_workspace_policy` | `self.read_required` | [1169](../../src/config.rs#L1169) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `publish_workspace_policy` | `validate_workspace` | [1170](../../src/config.rs#L1170), [1182](../../src/config.rs#L1182) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `publish_workspace_policy` | `value                 .revision                 .checked_add(1)                 .ok_or_else` | [1172](../../src/config.rs#L1172) | receiver-type-required |
| `publish_workspace_policy` | `value                 .revision                 .checked_add` | [1172](../../src/config.rs#L1172) | receiver-type-required |
| `publish_workspace_policy` | `path.clone` | [1176](../../src/config.rs#L1176) | receiver-type-required |
| `publish_workspace_policy` | `"workspace revision overflow".to_owned` | [1177](../../src/config.rs#L1177) | receiver-type-required |
| `publish_workspace_policy` | `check_publication_revision` | [1179](../../src/config.rs#L1179) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_workspace_policy` | `Some` | [1181](../../src/config.rs#L1181) | external-constructor-callback-or-unresolved |
| `publish_workspace_policy` | `self             .read_optional::<ProvidersConfig>(&providers_path)?             .unwrap_or_default` | [1183](../../src/config.rs#L1183) | receiver-type-required |
| `publish_workspace_policy` | `self             .read_optional::<ProvidersConfig>` | [1183](../../src/config.rs#L1183) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_workspace_policy` | `self             .read_optional::<SettingsConfig>(&settings_path)?             .unwrap_or_default` | [1186](../../src/config.rs#L1186) | receiver-type-required |
| `publish_workspace_policy` | `self             .read_optional::<SettingsConfig>` | [1186](../../src/config.rs#L1186) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_workspace_policy` | `validate_providers` | [1189](../../src/config.rs#L1189) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `publish_workspace_policy` | `validate_settings` | [1190](../../src/config.rs#L1190) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `publish_workspace_policy` | `validate_references` | [1191](../../src/config.rs#L1191) | [profile::config::validate_references](../../src/config.rs#L1708) |
| `publish_workspace_policy` | `AtomicPublisher::replace` | [1192](../../src/config.rs#L1192) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_workspace_policy` | `canonical_line` | [1192](../../src/config.rs#L1192) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_workspace_policy` | `Ok` | [1193](../../src/config.rs#L1193) | external-constructor-callback-or-unresolved |
| `publish_session_settings` | `store::session_settings_path` | [1202](../../src/config.rs#L1202) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L57) |
| `publish_session_settings` | `session_folder.as_ref` | [1202](../../src/config.rs#L1202) | receiver-type-required |
| `publish_session_settings` | `NamedLock::exclusive` | [1203](../../src/config.rs#L1203) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_session_settings` | `self.root.join` | [1203](../../src/config.rs#L1203), [1213](../../src/config.rs#L1213) | receiver-type-required |
| `publish_session_settings` | `self.read_optional::<SessionSettings>` | [1204](../../src/config.rs#L1204) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_session_settings` | `validate_session_settings` | [1206](../../src/config.rs#L1206), [1212](../../src/config.rs#L1212) | [profile::config::validate_session_settings](../../src/config.rs#L1668) |
| `publish_session_settings` | `check_publication_revision` | [1211](../../src/config.rs#L1211) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_session_settings` | `self.read_optional(&providers_path)?.unwrap_or_default` | [1214](../../src/config.rs#L1214) | receiver-type-required |
| `publish_session_settings` | `self.read_optional` | [1214](../../src/config.rs#L1214) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `publish_session_settings` | `validate_providers` | [1215](../../src/config.rs#L1215) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `publish_session_settings` | `validate_session_reference` | [1216](../../src/config.rs#L1216) | [profile::config::validate_session_reference](../../src/config.rs#L1682) |
| `publish_session_settings` | `Some` | [1216](../../src/config.rs#L1216) | external-constructor-callback-or-unresolved |
| `publish_session_settings` | `value.clone` | [1216](../../src/config.rs#L1216) | receiver-type-required |
| `publish_session_settings` | `AtomicPublisher::replace` | [1217](../../src/config.rs#L1217) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_session_settings` | `canonical_line` | [1217](../../src/config.rs#L1217) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_session_settings` | `Ok` | [1218](../../src/config.rs#L1218) | external-constructor-callback-or-unresolved |
| `session_settings` | `NamedLock::shared` | [1225](../../src/config.rs#L1225) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `session_settings` | `self.root.join` | [1225](../../src/config.rs#L1225) | receiver-type-required |
| `session_settings` | `store::session_settings_path` | [1226](../../src/config.rs#L1226) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L57) |
| `session_settings` | `session_folder.as_ref` | [1226](../../src/config.rs#L1226) | receiver-type-required |
| `session_settings` | `self.read_optional::<SessionSettings>` | [1227](../../src/config.rs#L1227) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `session_settings` | `validate_session_settings` | [1229](../../src/config.rs#L1229) | [profile::config::validate_session_settings](../../src/config.rs#L1668) |
| `session_settings` | `Ok` | [1231](../../src/config.rs#L1231) | external-constructor-callback-or-unresolved |
| `publish_global` | `NamedLock::exclusive` | [1242](../../src/config.rs#L1242) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `publish_global` | `self.root.join` | [1242](../../src/config.rs#L1242) | receiver-type-required |
| `publish_global` | `fs::read` | [1243](../../src/config.rs#L1243) | external-constructor-callback-or-unresolved |
| `publish_global` | `parse_canonical` | [1245](../../src/config.rs#L1245) | [profile::parse_canonical](../../src/lib.rs#L71) |
| `publish_global` | `validate` | [1246](../../src/config.rs#L1246), [1253](../../src/config.rs#L1253) | external-constructor-callback-or-unresolved |
| `publish_global` | `revision_from_value` | [1247](../../src/config.rs#L1247) | [profile::config::revision_from_value](../../src/config.rs#L1447) |
| `publish_global` | `error.kind` | [1249](../../src/config.rs#L1249) | receiver-type-required |
| `publish_global` | `Err` | [1250](../../src/config.rs#L1250) | external-constructor-callback-or-unresolved |
| `publish_global` | `error.into` | [1250](../../src/config.rs#L1250) | receiver-type-required |
| `publish_global` | `check_publication_revision` | [1252](../../src/config.rs#L1252) | [profile::config::check_publication_revision](../../src/config.rs#L1425) |
| `publish_global` | `AtomicPublisher::replace` | [1254](../../src/config.rs#L1254) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `publish_global` | `canonical_line` | [1254](../../src/config.rs#L1254) | [profile::canonical_line](../../src/lib.rs#L116) |
| `publish_global` | `Ok` | [1255](../../src/config.rs#L1255) | external-constructor-callback-or-unresolved |
| `resolve_unlocked` | `workspace_document_path` | [1265](../../src/config.rs#L1265) | [profile::config::workspace_document_path](../../src/config.rs#L1481) |
| `resolve_unlocked` | `self.read_required` | [1266](../../src/config.rs#L1266) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `resolve_unlocked` | `Err` | [1268](../../src/config.rs#L1268), [1318](../../src/config.rs#L1318) | external-constructor-callback-or-unresolved |
| `resolve_unlocked` | `"workspace id does not match filename".to_owned` | [1270](../../src/config.rs#L1270) | receiver-type-required |
| `resolve_unlocked` | `self.root.join` | [1273](../../src/config.rs#L1273), [1274](../../src/config.rs#L1274), [1328](../../src/config.rs#L1328) | receiver-type-required |
| `resolve_unlocked` | `self.read_optional(&providers_path)?.unwrap_or_default` | [1275](../../src/config.rs#L1275) | receiver-type-required |
| `resolve_unlocked` | `self.read_optional` | [1275](../../src/config.rs#L1275), [1276](../../src/config.rs#L1276), [1330](../../src/config.rs#L1330) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `resolve_unlocked` | `self.read_optional(&settings_path)?.unwrap_or_default` | [1276](../../src/config.rs#L1276) | receiver-type-required |
| `resolve_unlocked` | `session_folder             .map(store::session_settings_path)             .transpose` | [1277](../../src/config.rs#L1277) | receiver-type-required |
| `resolve_unlocked` | `session_folder             .map` | [1277](../../src/config.rs#L1277), [1301](../../src/config.rs#L1301) | receiver-type-required |
| `resolve_unlocked` | `session_settings_path.as_ref` | [1280](../../src/config.rs#L1280) | receiver-type-required |
| `resolve_unlocked` | `self.read_optional::<SessionSettings>` | [1281](../../src/config.rs#L1281) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `resolve_unlocked` | `validate_workspace` | [1285](../../src/config.rs#L1285) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `resolve_unlocked` | `validate_providers` | [1286](../../src/config.rs#L1286) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `resolve_unlocked` | `validate_settings` | [1287](../../src/config.rs#L1287) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `resolve_unlocked` | `validate_session_settings` | [1289](../../src/config.rs#L1289) | [profile::config::validate_session_settings](../../src/config.rs#L1668) |
| `resolve_unlocked` | `validate_references` | [1291](../../src/config.rs#L1291) | [profile::config::validate_references](../../src/config.rs#L1708) |
| `resolve_unlocked` | `validate_session_reference` | [1292](../../src/config.rs#L1292) | [profile::config::validate_session_reference](../../src/config.rs#L1682) |
| `resolve_unlocked` | `session_settings_path.as_deref().unwrap_or` | [1295](../../src/config.rs#L1295) | receiver-type-required |
| `resolve_unlocked` | `session_settings_path.as_deref` | [1295](../../src/config.rs#L1295) | receiver-type-required |
| `resolve_unlocked` | `resolve_workspace` | [1297](../../src/config.rs#L1297) | [profile::config::resolve_workspace](../../src/config.rs#L1878) |
| `resolve_unlocked` | `session_folder             .map(session_preset_tools)             .transpose()?             .flatten` | [1301](../../src/config.rs#L1301) | receiver-type-required |
| `resolve_unlocked` | `session_folder             .map(session_preset_tools)             .transpose` | [1301](../../src/config.rs#L1301) | receiver-type-required |
| `resolve_unlocked` | `validate_id` | [1309](../../src/config.rs#L1309) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `resolve_unlocked` | `workspace.path_for_binding(binding).ok_or_else` | [1310](../../src/config.rs#L1310) | receiver-type-required |
| `resolve_unlocked` | `workspace.path_for_binding` | [1310](../../src/config.rs#L1310) | receiver-type-required |
| `resolve_unlocked` | `workspace_path.clone` | [1312](../../src/config.rs#L1312), [1319](../../src/config.rs#L1319) | receiver-type-required |
| `resolve_unlocked` | `canonical_directory` | [1316](../../src/config.rs#L1316) | [profile::config::canonical_directory](../../src/config.rs#L2075) |
| `resolve_unlocked` | `resolved.cwd.contains` | [1317](../../src/config.rs#L1317) | receiver-type-required |
| `resolve_unlocked` | `Some` | [1323](../../src/config.rs#L1323), [1324](../../src/config.rs#L1324), [1339](../../src/config.rs#L1339) | external-constructor-callback-or-unresolved |
| `resolve_unlocked` | `binding.to_owned` | [1323](../../src/config.rs#L1323) | receiver-type-required |
| `resolve_unlocked` | `self.read_optional(&path)?.unwrap_or_default` | [1330](../../src/config.rs#L1330) | receiver-type-required |
| `resolve_unlocked` | `validate_environments` | [1331](../../src/config.rs#L1331) | [profile::config::validate_environments](../../src/config.rs#L1850) |
| `resolve_unlocked` | `apply_scope` | [1332](../../src/config.rs#L1332) | [profile::config::apply_scope](../../src/config.rs#L1785) |
| `resolve_unlocked` | `Ok` | [1343](../../src/config.rs#L1343) | external-constructor-callback-or-unresolved |
| `resolve_unlocked` | `session_settings.as_ref().map` | [1350](../../src/config.rs#L1350) | receiver-type-required |
| `resolve_unlocked` | `session_settings.as_ref` | [1350](../../src/config.rs#L1350) | receiver-type-required |
| `read_required` | `parse_canonical` | [1363](../../src/config.rs#L1363) | [profile::parse_canonical](../../src/lib.rs#L71) |
| `read_required` | `fs::read` | [1363](../../src/config.rs#L1363) | external-constructor-callback-or-unresolved |
| `read_optional` | `fs::read` | [1370](../../src/config.rs#L1370) | external-constructor-callback-or-unresolved |
| `read_optional` | `Ok` | [1371](../../src/config.rs#L1371), [1372](../../src/config.rs#L1372) | external-constructor-callback-or-unresolved |
| `read_optional` | `Some` | [1371](../../src/config.rs#L1371) | external-constructor-callback-or-unresolved |
| `read_optional` | `parse_canonical` | [1371](../../src/config.rs#L1371) | [profile::parse_canonical](../../src/lib.rs#L71) |
| `read_optional` | `error.kind` | [1372](../../src/config.rs#L1372) | receiver-type-required |
| `read_optional` | `Err` | [1373](../../src/config.rs#L1373) | external-constructor-callback-or-unresolved |
| `read_optional` | `error.into` | [1373](../../src/config.rs#L1373) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `self.root.join` | [1381](../../src/config.rs#L1381), [1388](../../src/config.rs#L1388), [1404](../../src/config.rs#L1404) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `self             .read_optional::<SettingsConfig>(&settings_path)?             .unwrap_or_default` | [1382](../../src/config.rs#L1382) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `self             .read_optional::<SettingsConfig>` | [1382](../../src/config.rs#L1382) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `validate_all_provider_references_unlocked` | `validate_settings` | [1385](../../src/config.rs#L1385) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `validate_all_provider_references_unlocked` | `fs::read_dir(self.root.join("workspaces"))?.collect::<Result<Vec<_>, _>>` | [1388](../../src/config.rs#L1388) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `fs::read_dir` | [1388](../../src/config.rs#L1388), [1405](../../src/config.rs#L1405) | external-constructor-callback-or-unresolved |
| `validate_all_provider_references_unlocked` | `workspace_entries.sort_by_key` | [1389](../../src/config.rs#L1389) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `entry.file_type()?.is_dir` | [1391](../../src/config.rs#L1391), [1411](../../src/config.rs#L1411) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `entry.file_type` | [1391](../../src/config.rs#L1391), [1411](../../src/config.rs#L1411) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `invalid` | [1392](../../src/config.rs#L1392) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_all_provider_references_unlocked` | `entry.path` | [1393](../../src/config.rs#L1393), [1397](../../src/config.rs#L1397), [1414](../../src/config.rs#L1414) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `entry.path().join` | [1397](../../src/config.rs#L1397) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `self.read_required` | [1398](../../src/config.rs#L1398) | [profile::config::ConfigRepository::read_required](../../src/config.rs#L1362) |
| `validate_all_provider_references_unlocked` | `validate_workspace` | [1399](../../src/config.rs#L1399) | [profile::config::validate_workspace](../../src/config.rs#L1509) |
| `validate_all_provider_references_unlocked` | `validate_references` | [1400](../../src/config.rs#L1400) | [profile::config::validate_references](../../src/config.rs#L1708) |
| `validate_all_provider_references_unlocked` | `entries.collect::<Result<Vec<_>, _>>` | [1406](../../src/config.rs#L1406) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `error.kind` | [1407](../../src/config.rs#L1407) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `Err` | [1408](../../src/config.rs#L1408) | external-constructor-callback-or-unresolved |
| `validate_all_provider_references_unlocked` | `error.into` | [1408](../../src/config.rs#L1408) | receiver-type-required |
| `validate_all_provider_references_unlocked` | `store::session_settings_path` | [1414](../../src/config.rs#L1414) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L57) |
| `validate_all_provider_references_unlocked` | `self.read_optional::<SessionSettings>` | [1415](../../src/config.rs#L1415) | [profile::config::ConfigRepository::read_optional](../../src/config.rs#L1366) |
| `validate_all_provider_references_unlocked` | `validate_session_settings` | [1416](../../src/config.rs#L1416) | [profile::config::validate_session_settings](../../src/config.rs#L1668) |
| `validate_all_provider_references_unlocked` | `validate_session_reference` | [1417](../../src/config.rs#L1417) | [profile::config::validate_session_reference](../../src/config.rs#L1682) |
| `validate_all_provider_references_unlocked` | `Some` | [1417](../../src/config.rs#L1417) | external-constructor-callback-or-unresolved |
| `validate_all_provider_references_unlocked` | `Ok` | [1421](../../src/config.rs#L1421) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `validate_safe_integer` | [1426](../../src/config.rs#L1426), [1427](../../src/config.rs#L1427), [1428](../../src/config.rs#L1428) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `check_publication_revision` | `Path::new` | [1426](../../src/config.rs#L1426), [1427](../../src/config.rs#L1427), [1428](../../src/config.rs#L1428) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `Err` | [1430](../../src/config.rs#L1430), [1439](../../src/config.rs#L1439) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `actual         .checked_add(1)         .ok_or_else` | [1432](../../src/config.rs#L1432) | receiver-type-required |
| `check_publication_revision` | `actual         .checked_add` | [1432](../../src/config.rs#L1432) | receiver-type-required |
| `check_publication_revision` | `PathBuf::from` | [1435](../../src/config.rs#L1435), [1440](../../src/config.rs#L1440) | external-constructor-callback-or-unresolved |
| `check_publication_revision` | `"revision cannot advance beyond the I-JSON safe-integer range".to_owned` | [1436](../../src/config.rs#L1436) | receiver-type-required |
| `check_publication_revision` | `Ok` | [1444](../../src/config.rs#L1444) | external-constructor-callback-or-unresolved |
| `revision_from_value` | `serde_json::to_value(value).map_err` | [1452](../../src/config.rs#L1452) | receiver-type-required |
| `revision_from_value` | `serde_json::to_value` | [1452](../../src/config.rs#L1452) | external-constructor-callback-or-unresolved |
| `revision_from_value` | `path.to_path_buf` | [1453](../../src/config.rs#L1453), [1459](../../src/config.rs#L1459) | receiver-type-required |
| `revision_from_value` | `error.to_string` | [1454](../../src/config.rs#L1454), [1460](../../src/config.rs#L1460) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>(value)         .map(&#124;value&#124; value.revision)         .map_err` | [1456](../../src/config.rs#L1456) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>(value)         .map` | [1456](../../src/config.rs#L1456) | receiver-type-required |
| `revision_from_value` | `serde_json::from_value::<RevisionOnly>` | [1456](../../src/config.rs#L1456) | external-constructor-callback-or-unresolved |
| `validate_workspace_id` | `id.is_empty` | [1465](../../src/config.rs#L1465) | receiver-type-required |
| `validate_workspace_id` | `id.len` | [1466](../../src/config.rs#L1466) | receiver-type-required |
| `validate_workspace_id` | `id.starts_with` | [1467](../../src/config.rs#L1467) | receiver-type-required |
| `validate_workspace_id` | `id             .bytes()             .all` | [1468](../../src/config.rs#L1468) | receiver-type-required |
| `validate_workspace_id` | `id             .bytes` | [1468](../../src/config.rs#L1468) | receiver-type-required |
| `validate_workspace_id` | `byte.is_ascii_alphanumeric` | [1470](../../src/config.rs#L1470) | receiver-type-required |
| `validate_workspace_id` | `Ok` | [1472](../../src/config.rs#L1472) | external-constructor-callback-or-unresolved |
| `validate_workspace_id` | `Err` | [1474](../../src/config.rs#L1474) | external-constructor-callback-or-unresolved |
| `validate_workspace_id` | `PathBuf::from` | [1475](../../src/config.rs#L1475) | external-constructor-callback-or-unresolved |
| `validate_workspace_id` | `"workspace id is outside [A-Za-z0-9._-]".to_owned` | [1476](../../src/config.rs#L1476) | receiver-type-required |
| `workspace_document_path` | `root.join("workspaces")         .join(workspace_id)         .join` | [1482](../../src/config.rs#L1482) | receiver-type-required |
| `workspace_document_path` | `root.join("workspaces")         .join` | [1482](../../src/config.rs#L1482) | receiver-type-required |
| `workspace_document_path` | `root.join` | [1482](../../src/config.rs#L1482) | receiver-type-required |
| `validate_common` | `Err` | [1494](../../src/config.rs#L1494), [1500](../../src/config.rs#L1500) | external-constructor-callback-or-unresolved |
| `validate_common` | `path.to_path_buf` | [1495](../../src/config.rs#L1495), [1501](../../src/config.rs#L1501) | receiver-type-required |
| `validate_common` | `"stored revision must be at least 1".to_owned` | [1502](../../src/config.rs#L1502) | receiver-type-required |
| `validate_common` | `validate_safe_integer` | [1505](../../src/config.rs#L1505) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `validate_common` | `Ok` | [1506](../../src/config.rs#L1506) | external-constructor-callback-or-unresolved |
| `validate_workspace` | `validate_common` | [1514](../../src/config.rs#L1514) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_workspace` | `validate_workspace_id` | [1515](../../src/config.rs#L1515) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `validate_workspace` | `validate_id` | [1516](../../src/config.rs#L1516), [1525](../../src/config.rs#L1525) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `validate_workspace` | `value.cwd.is_empty` | [1517](../../src/config.rs#L1517), [1520](../../src/config.rs#L1520) | receiver-type-required |
| `validate_workspace` | `value.folders.is_empty` | [1517](../../src/config.rs#L1517), [1520](../../src/config.rs#L1520) | receiver-type-required |
| `validate_workspace` | `invalid` | [1518](../../src/config.rs#L1518), [1521](../../src/config.rs#L1521), [1527](../../src/config.rs#L1527), [1540](../../src/config.rs#L1540) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_workspace` | `BTreeSet::new` | [1523](../../src/config.rs#L1523) | external-constructor-callback-or-unresolved |
| `validate_workspace` | `binding_ids.insert` | [1526](../../src/config.rs#L1526) | receiver-type-required |
| `validate_workspace` | `folder.id.as_str` | [1526](../../src/config.rs#L1526) | receiver-type-required |
| `validate_workspace` | `validate_absolute` | [1529](../../src/config.rs#L1529), [1532](../../src/config.rs#L1532), [1536](../../src/config.rs#L1536) | [profile::config::validate_absolute](../../src/config.rs#L2096) |
| `validate_workspace` | `value.folder_paths` | [1531](../../src/config.rs#L1531) | receiver-type-required |
| `validate_workspace` | `policy.writable_roots.iter().chain` | [1535](../../src/config.rs#L1535) | receiver-type-required |
| `validate_workspace` | `policy.writable_roots.iter` | [1535](../../src/config.rs#L1535) | receiver-type-required |
| `validate_workspace` | `validate_unique` | [1538](../../src/config.rs#L1538) | [profile::config::validate_unique](../../src/config.rs#L2117) |
| `validate_workspace` | `Some` | [1539](../../src/config.rs#L1539) | external-constructor-callback-or-unresolved |
| `validate_workspace` | `validate_safe_integer` | [1543](../../src/config.rs#L1543) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `validate_workspace` | `Ok` | [1546](../../src/config.rs#L1546) | external-constructor-callback-or-unresolved |
| `validate_providers` | `validate_common` | [1554](../../src/config.rs#L1554) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_providers` | `BTreeSet::new` | [1555](../../src/config.rs#L1555), [1581](../../src/config.rs#L1581) | external-constructor-callback-or-unresolved |
| `validate_providers` | `validate_id` | [1557](../../src/config.rs#L1557), [1559](../../src/config.rs#L1559), [1561](../../src/config.rs#L1561), [1562](../../src/config.rs#L1562), [1563](../../src/config.rs#L1563), [1564](../../src/config.rs#L1564), [1569](../../src/config.rs#L1569), [1574](../../src/config.rs#L1574), [1579](../../src/config.rs#L1579), [1583](../../src/config.rs#L1583), [1584](../../src/config.rs#L1584), [1605](../../src/config.rs#L1605) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `validate_providers` | `ids.insert` | [1575](../../src/config.rs#L1575) | receiver-type-required |
| `validate_providers` | `provider.id.as_str` | [1575](../../src/config.rs#L1575) | receiver-type-required |
| `validate_providers` | `invalid` | [1576](../../src/config.rs#L1576), [1591](../../src/config.rs#L1591), [1597](../../src/config.rs#L1597), [1603](../../src/config.rs#L1603) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_providers` | `validate_safe_integer` | [1585](../../src/config.rs#L1585), [1586](../../src/config.rs#L1586) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `validate_providers` | `models.insert` | [1596](../../src/config.rs#L1596) | receiver-type-required |
| `validate_providers` | `model.id.as_str` | [1596](../../src/config.rs#L1596) | receiver-type-required |
| `validate_providers` | `validate_web_search_origin` | [1606](../../src/config.rs#L1606) | [profile::config::validate_web_search_origin](../../src/config.rs#L1611) |
| `validate_providers` | `Ok` | [1608](../../src/config.rs#L1608) | external-constructor-callback-or-unresolved |
| `validate_web_search_origin` | `url::Url::parse(value).map_err` | [1612](../../src/config.rs#L1612) | receiver-type-required |
| `validate_web_search_origin` | `url::Url::parse` | [1612](../../src/config.rs#L1612) | external-constructor-callback-or-unresolved |
| `validate_web_search_origin` | `path.to_path_buf` | [1613](../../src/config.rs#L1613) | receiver-type-required |
| `validate_web_search_origin` | `"web_search endpoint is not an absolute URL".to_owned` | [1614](../../src/config.rs#L1614) | receiver-type-required |
| `validate_web_search_origin` | `parsed.scheme` | [1616](../../src/config.rs#L1616) | receiver-type-required |
| `validate_web_search_origin` | `parsed.username` | [1617](../../src/config.rs#L1617) | receiver-type-required |
| `validate_web_search_origin` | `parsed.password().is_some` | [1618](../../src/config.rs#L1618) | receiver-type-required |
| `validate_web_search_origin` | `parsed.password` | [1618](../../src/config.rs#L1618) | receiver-type-required |
| `validate_web_search_origin` | `parsed.query().is_some` | [1619](../../src/config.rs#L1619) | receiver-type-required |
| `validate_web_search_origin` | `parsed.query` | [1619](../../src/config.rs#L1619) | receiver-type-required |
| `validate_web_search_origin` | `parsed.fragment().is_some` | [1620](../../src/config.rs#L1620) | receiver-type-required |
| `validate_web_search_origin` | `parsed.fragment` | [1620](../../src/config.rs#L1620) | receiver-type-required |
| `validate_web_search_origin` | `parsed.path` | [1621](../../src/config.rs#L1621) | receiver-type-required |
| `validate_web_search_origin` | `parsed.host_str().is_none` | [1622](../../src/config.rs#L1622) | receiver-type-required |
| `validate_web_search_origin` | `parsed.host_str` | [1622](../../src/config.rs#L1622), [1630](../../src/config.rs#L1630) | receiver-type-required |
| `validate_web_search_origin` | `value.ends_with` | [1623](../../src/config.rs#L1623) | receiver-type-required |
| `validate_web_search_origin` | `invalid` | [1625](../../src/config.rs#L1625), [1641](../../src/config.rs#L1641) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_web_search_origin` | `parsed.host_str().expect` | [1630](../../src/config.rs#L1630) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix('[')         .and_then(&#124;value&#124; value.strip_suffix(']'))         .unwrap_or` | [1631](../../src/config.rs#L1631) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix('[')         .and_then` | [1631](../../src/config.rs#L1631) | receiver-type-required |
| `validate_web_search_origin` | `host         .strip_prefix` | [1631](../../src/config.rs#L1631) | receiver-type-required |
| `validate_web_search_origin` | `value.strip_suffix` | [1633](../../src/config.rs#L1633) | receiver-type-required |
| `validate_web_search_origin` | `host.eq_ignore_ascii_case` | [1635](../../src/config.rs#L1635) | receiver-type-required |
| `validate_web_search_origin` | `host.ends_with` | [1636](../../src/config.rs#L1636) | receiver-type-required |
| `validate_web_search_origin` | `address_literal             .parse::<std::net::IpAddr>()             .is_ok_and` | [1637](../../src/config.rs#L1637) | receiver-type-required |
| `validate_web_search_origin` | `address_literal             .parse::<std::net::IpAddr>` | [1637](../../src/config.rs#L1637) | receiver-type-required |
| `validate_web_search_origin` | `tools::is_public_internet_address` | [1639](../../src/config.rs#L1639) | [tools::runtime_backends::is_public_internet_address](../../../tools/src/runtime_backends.rs#L1689) |
| `validate_web_search_origin` | `Ok` | [1643](../../src/config.rs#L1643) | external-constructor-callback-or-unresolved |
| `validate_settings` | `validate_common` | [1651](../../src/config.rs#L1651) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_settings` | `Some` | [1653](../../src/config.rs#L1653) | external-constructor-callback-or-unresolved |
| `validate_settings` | `invalid` | [1654](../../src/config.rs#L1654) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_settings` | `validate_safe_integer` | [1661](../../src/config.rs#L1661) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `validate_settings` | `Ok` | [1665](../../src/config.rs#L1665) | external-constructor-callback-or-unresolved |
| `validate_session_settings` | `validate_common` | [1673](../../src/config.rs#L1673) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_session_settings` | `validate_id` | [1674](../../src/config.rs#L1674), [1675](../../src/config.rs#L1675) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `validate_session_settings` | `value.reasoning_effort.as_deref().is_some_and` | [1676](../../src/config.rs#L1676) | receiver-type-required |
| `validate_session_settings` | `value.reasoning_effort.as_deref` | [1676](../../src/config.rs#L1676) | receiver-type-required |
| `validate_session_settings` | `invalid` | [1677](../../src/config.rs#L1677) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_session_settings` | `Ok` | [1679](../../src/config.rs#L1679) | external-constructor-callback-or-unresolved |
| `validate_session_reference` | `Ok` | [1688](../../src/config.rs#L1688), [1705](../../src/config.rs#L1705) | external-constructor-callback-or-unresolved |
| `validate_session_reference` | `providers         .providers         .iter()         .find(&#124;provider&#124; provider.id == settings.provider)         .ok_or_else` | [1690](../../src/config.rs#L1690) | receiver-type-required |
| `validate_session_reference` | `providers         .providers         .iter()         .find` | [1690](../../src/config.rs#L1690) | receiver-type-required |
| `validate_session_reference` | `providers         .providers         .iter` | [1690](../../src/config.rs#L1690) | receiver-type-required |
| `validate_session_reference` | `path.to_path_buf` | [1695](../../src/config.rs#L1695) | receiver-type-required |
| `validate_session_reference` | `provider         .models         .iter()         .any` | [1698](../../src/config.rs#L1698) | receiver-type-required |
| `validate_session_reference` | `provider         .models         .iter` | [1698](../../src/config.rs#L1698) | receiver-type-required |
| `validate_session_reference` | `invalid_reference` | [1703](../../src/config.rs#L1703) | [profile::config::invalid_reference](../../src/config.rs#L2133) |
| `validate_references` | `settings.default_provider.as_deref` | [1716](../../src/config.rs#L1716) | receiver-type-required |
| `validate_references` | `settings.default_model.as_deref` | [1717](../../src/config.rs#L1717) | receiver-type-required |
| `validate_references` | `workspace                 .policy                 .as_ref()                 .and_then` | [1720](../../src/config.rs#L1720), [1724](../../src/config.rs#L1724) | receiver-type-required |
| `validate_references` | `workspace                 .policy                 .as_ref` | [1720](../../src/config.rs#L1720), [1724](../../src/config.rs#L1724) | receiver-type-required |
| `validate_references` | `value.provider.as_deref` | [1723](../../src/config.rs#L1723) | receiver-type-required |
| `validate_references` | `value.model.as_deref` | [1727](../../src/config.rs#L1727) | receiver-type-required |
| `validate_references` | `model.is_some` | [1731](../../src/config.rs#L1731) | receiver-type-required |
| `validate_references` | `invalid_reference` | [1732](../../src/config.rs#L1732), [1750](../../src/config.rs#L1750) | [profile::config::invalid_reference](../../src/config.rs#L2133) |
| `validate_references` | `providers             .providers             .iter()             .find(&#124;value&#124; value.id == provider_id)             .ok_or_else` | [1736](../../src/config.rs#L1736) | receiver-type-required |
| `validate_references` | `providers             .providers             .iter()             .find` | [1736](../../src/config.rs#L1736) | receiver-type-required |
| `validate_references` | `providers             .providers             .iter` | [1736](../../src/config.rs#L1736) | receiver-type-required |
| `validate_references` | `path.to_path_buf` | [1741](../../src/config.rs#L1741) | receiver-type-required |
| `validate_references` | `provider                 .models                 .iter()                 .any` | [1745](../../src/config.rs#L1745) | receiver-type-required |
| `validate_references` | `provider                 .models                 .iter` | [1745](../../src/config.rs#L1745) | receiver-type-required |
| `validate_references` | `Ok` | [1753](../../src/config.rs#L1753) | external-constructor-callback-or-unresolved |
| `session_preset_tools` | `session_folder.join` | [1758](../../src/config.rs#L1758) | receiver-type-required |
| `session_preset_tools` | `ledger.is_file` | [1759](../../src/config.rs#L1759) | receiver-type-required |
| `session_preset_tools` | `Ok` | [1760](../../src/config.rs#L1760), [1763](../../src/config.rs#L1763), [1770](../../src/config.rs#L1770), [1778](../../src/config.rs#L1778) | external-constructor-callback-or-unresolved |
| `session_preset_tools` | `store::read_genesis` | [1762](../../src/config.rs#L1762) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `session_preset_tools` | `serde_json::to_value(genesis.raw()).map_err` | [1765](../../src/config.rs#L1765) | receiver-type-required |
| `session_preset_tools` | `serde_json::to_value` | [1765](../../src/config.rs#L1765) | external-constructor-callback-or-unresolved |
| `session_preset_tools` | `genesis.raw` | [1765](../../src/config.rs#L1765) | receiver-type-required |
| `session_preset_tools` | `ledger.clone` | [1766](../../src/config.rs#L1766) | receiver-type-required |
| `session_preset_tools` | `error.to_string` | [1767](../../src/config.rs#L1767) | receiver-type-required |
| `session_preset_tools` | `raw.get` | [1769](../../src/config.rs#L1769) | receiver-type-required |
| `session_preset_tools` | `tools::ResolvedPreset::from_genesis(preset).map_err` | [1772](../../src/config.rs#L1772) | receiver-type-required |
| `session_preset_tools` | `tools::ResolvedPreset::from_genesis` | [1772](../../src/config.rs#L1772) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `apply_scope` | `environments.environment(scope.environment).ok_or_else` | [1792](../../src/config.rs#L1792) | receiver-type-required |
| `apply_scope` | `environments.environment` | [1792](../../src/config.rs#L1792) | receiver-type-required |
| `apply_scope` | `path.to_path_buf` | [1794](../../src/config.rs#L1794), [1805](../../src/config.rs#L1805), [1818](../../src/config.rs#L1818) | receiver-type-required |
| `apply_scope` | `Vec::with_capacity` | [1799](../../src/config.rs#L1799) | external-constructor-callback-or-unresolved |
| `apply_scope` | `ids.len` | [1799](../../src/config.rs#L1799) | receiver-type-required |
| `apply_scope` | `workspace                     .path_for_binding(id)                     .ok_or_else` | [1802](../../src/config.rs#L1802) | receiver-type-required |
| `apply_scope` | `workspace                     .path_for_binding` | [1802](../../src/config.rs#L1802) | receiver-type-required |
| `apply_scope` | `canonical_directory` | [1808](../../src/config.rs#L1808), [1838](../../src/config.rs#L1838) | [profile::config::canonical_directory](../../src/config.rs#L2075) |
| `apply_scope` | `cwd.contains` | [1809](../../src/config.rs#L1809), [1815](../../src/config.rs#L1815) | receiver-type-required |
| `apply_scope` | `invalid` | [1810](../../src/config.rs#L1810), [1840](../../src/config.rs#L1840) | [profile::config::invalid](../../src/config.rs#L2126) |
| `apply_scope` | `cwd.push` | [1812](../../src/config.rs#L1812) | receiver-type-required |
| `apply_scope` | `Err` | [1817](../../src/config.rs#L1817) | external-constructor-callback-or-unresolved |
| `apply_scope` | `"folder binding is not one of the session's folders".to_owned` | [1819](../../src/config.rs#L1819) | receiver-type-required |
| `apply_scope` | `resolved             .policy             .writable_roots             .retain` | [1822](../../src/config.rs#L1822) | receiver-type-required |
| `apply_scope` | `cwd.iter().any` | [1825](../../src/config.rs#L1825) | receiver-type-required |
| `apply_scope` | `cwd.iter` | [1825](../../src/config.rs#L1825) | receiver-type-required |
| `apply_scope` | `Path::new(root).starts_with` | [1825](../../src/config.rs#L1825) | receiver-type-required |
| `apply_scope` | `Path::new` | [1825](../../src/config.rs#L1825) | external-constructor-callback-or-unresolved |
| `apply_scope` | `Some` | [1827](../../src/config.rs#L1827), [1834](../../src/config.rs#L1834), [1846](../../src/config.rs#L1846) | external-constructor-callback-or-unresolved |
| `apply_scope` | `ids.to_vec` | [1827](../../src/config.rs#L1827) | receiver-type-required |
| `apply_scope` | `left.min` | [1834](../../src/config.rs#L1834) | receiver-type-required |
| `apply_scope` | `left.or` | [1835](../../src/config.rs#L1835) | receiver-type-required |
| `apply_scope` | `canonical.contains` | [1839](../../src/config.rs#L1839) | receiver-type-required |
| `apply_scope` | `resolved.policy.toolchain_roots.push` | [1842](../../src/config.rs#L1842) | receiver-type-required |
| `apply_scope` | `resolved.policy.toolchain_roots.sort` | [1844](../../src/config.rs#L1844) | receiver-type-required |
| `apply_scope` | `resolved.policy.toolchain_roots.dedup` | [1845](../../src/config.rs#L1845) | receiver-type-required |
| `apply_scope` | `Ok` | [1847](../../src/config.rs#L1847) | external-constructor-callback-or-unresolved |
| `validate_environments` | `validate_common` | [1855](../../src/config.rs#L1855) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_environments` | `BTreeSet::new` | [1856](../../src/config.rs#L1856) | external-constructor-callback-or-unresolved |
| `validate_environments` | `validate_id` | [1858](../../src/config.rs#L1858) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `validate_environments` | `environment.id.len` | [1859](../../src/config.rs#L1859) | receiver-type-required |
| `validate_environments` | `environment.id.nfc().collect::<String>` | [1859](../../src/config.rs#L1859) | receiver-type-required |
| `validate_environments` | `environment.id.nfc` | [1859](../../src/config.rs#L1859) | receiver-type-required |
| `validate_environments` | `invalid` | [1861](../../src/config.rs#L1861), [1864](../../src/config.rs#L1864), [1868](../../src/config.rs#L1868), [1872](../../src/config.rs#L1872) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_environments` | `ids.insert` | [1863](../../src/config.rs#L1863) | receiver-type-required |
| `validate_environments` | `environment.id.as_str` | [1863](../../src/config.rs#L1863) | receiver-type-required |
| `validate_environments` | `Path::new(root).is_absolute` | [1867](../../src/config.rs#L1867) | receiver-type-required |
| `validate_environments` | `Path::new` | [1867](../../src/config.rs#L1867) | external-constructor-callback-or-unresolved |
| `validate_environments` | `root.contains` | [1867](../../src/config.rs#L1867) | receiver-type-required |
| `validate_environments` | `Some` | [1871](../../src/config.rs#L1871) | external-constructor-callback-or-unresolved |
| `validate_environments` | `Ok` | [1875](../../src/config.rs#L1875) | external-constructor-callback-or-unresolved |
| `resolve_workspace` | `Vec::new` | [1882](../../src/config.rs#L1882), [1891](../../src/config.rs#L1891), [1905](../../src/config.rs#L1905) | external-constructor-callback-or-unresolved |
| `resolve_workspace` | `value.folder_paths` | [1883](../../src/config.rs#L1883) | receiver-type-required |
| `resolve_workspace` | `canonical_directory` | [1884](../../src/config.rs#L1884), [1893](../../src/config.rs#L1893), [1907](../../src/config.rs#L1907) | [profile::config::canonical_directory](../../src/config.rs#L2075) |
| `resolve_workspace` | `cwd.contains` | [1885](../../src/config.rs#L1885) | receiver-type-required |
| `resolve_workspace` | `invalid` | [1886](../../src/config.rs#L1886), [1898](../../src/config.rs#L1898), [1909](../../src/config.rs#L1909) | [profile::config::invalid](../../src/config.rs#L2126) |
| `resolve_workspace` | `cwd.push` | [1888](../../src/config.rs#L1888) | receiver-type-required |
| `resolve_workspace` | `value.policy.clone().unwrap_or_default` | [1890](../../src/config.rs#L1890) | receiver-type-required |
| `resolve_workspace` | `value.policy.clone` | [1890](../../src/config.rs#L1890) | receiver-type-required |
| `resolve_workspace` | `cwd             .iter()             .any` | [1894](../../src/config.rs#L1894) | receiver-type-required |
| `resolve_workspace` | `cwd             .iter` | [1894](../../src/config.rs#L1894) | receiver-type-required |
| `resolve_workspace` | `Path::new(&resolved).starts_with` | [1896](../../src/config.rs#L1896) | receiver-type-required |
| `resolve_workspace` | `Path::new` | [1896](../../src/config.rs#L1896) | external-constructor-callback-or-unresolved |
| `resolve_workspace` | `roots.push` | [1900](../../src/config.rs#L1900) | receiver-type-required |
| `resolve_workspace` | `roots.sort` | [1902](../../src/config.rs#L1902) | receiver-type-required |
| `resolve_workspace` | `roots.dedup` | [1903](../../src/config.rs#L1903) | receiver-type-required |
| `resolve_workspace` | `canonical.contains` | [1908](../../src/config.rs#L1908) | receiver-type-required |
| `resolve_workspace` | `toolchains.push` | [1911](../../src/config.rs#L1911) | receiver-type-required |
| `resolve_workspace` | `toolchains.sort` | [1913](../../src/config.rs#L1913) | receiver-type-required |
| `resolve_workspace` | `toolchains.dedup` | [1914](../../src/config.rs#L1914) | receiver-type-required |
| `resolve_workspace` | `policy.allowed_tools.sort` | [1916](../../src/config.rs#L1916) | receiver-type-required |
| `resolve_workspace` | `policy.allowed_tools.dedup` | [1917](../../src/config.rs#L1917) | receiver-type-required |
| `resolve_workspace` | `Ok` | [1918](../../src/config.rs#L1918) | external-constructor-callback-or-unresolved |
| `resolve_workspace` | `value.id.clone` | [1921](../../src/config.rs#L1921) | receiver-type-required |
| `resolve_workspace` | `value.name.clone` | [1922](../../src/config.rs#L1922) | receiver-type-required |
| `validate_snapshot` | `Path::new` | [1933](../../src/config.rs#L1933), [2004](../../src/config.rs#L2004) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `validate_absolute` | [1935](../../src/config.rs#L1935), [1959](../../src/config.rs#L1959), [1975](../../src/config.rs#L1975), [1987](../../src/config.rs#L1987), [1996](../../src/config.rs#L1996) | [profile::config::validate_absolute](../../src/config.rs#L2096) |
| `validate_snapshot` | `Err` | [1938](../../src/config.rs#L1938) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `path.to_path_buf` | [1939](../../src/config.rs#L1939) | receiver-type-required |
| `validate_snapshot` | `validate_common` | [1943](../../src/config.rs#L1943) | [profile::config::validate_common](../../src/config.rs#L1487) |
| `validate_snapshot` | `validate_workspace_id` | [1949](../../src/config.rs#L1949) | [profile::config::validate_workspace_id](../../src/config.rs#L1464) |
| `validate_snapshot` | `validate_id` | [1950](../../src/config.rs#L1950), [1952](../../src/config.rs#L1952) | [profile::config::validate_id](../../src/config.rs#L2106) |
| `validate_snapshot` | `snapshot.workspace.folder_binding.as_ref` | [1955](../../src/config.rs#L1955) | receiver-type-required |
| `validate_snapshot` | `snapshot.workspace.selected_cwd.as_ref` | [1956](../../src/config.rs#L1956) | receiver-type-required |
| `validate_snapshot` | `canonical_directory` | [1960](../../src/config.rs#L1960), [1976](../../src/config.rs#L1976), [1988](../../src/config.rs#L1988), [1997](../../src/config.rs#L1997) | [profile::config::canonical_directory](../../src/config.rs#L2075) |
| `validate_snapshot` | `invalid` | [1961](../../src/config.rs#L1961), [1964](../../src/config.rs#L1964), [1968](../../src/config.rs#L1968), [1971](../../src/config.rs#L1971), [1977](../../src/config.rs#L1977), [1984](../../src/config.rs#L1984), [1989](../../src/config.rs#L1989), [1998](../../src/config.rs#L1998), [2006](../../src/config.rs#L2006), [2011](../../src/config.rs#L2011), [2021](../../src/config.rs#L2021), [2027](../../src/config.rs#L2027), [2052](../../src/config.rs#L2052) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_snapshot` | `snapshot.workspace.cwd.contains` | [1963](../../src/config.rs#L1963) | receiver-type-required |
| `validate_snapshot` | `snapshot.workspace.cwd.is_empty` | [1970](../../src/config.rs#L1970) | receiver-type-required |
| `validate_snapshot` | `validate_unique` | [1973](../../src/config.rs#L1973) | [profile::config::validate_unique](../../src/config.rs#L2117) |
| `validate_snapshot` | `is_sorted_unique` | [1980](../../src/config.rs#L1980), [1981](../../src/config.rs#L1981), [1982](../../src/config.rs#L1982) | [profile::config::is_sorted_unique](../../src/config.rs#L2071) |
| `validate_snapshot` | `root.contains` | [1988](../../src/config.rs#L1988) | receiver-type-required |
| `validate_snapshot` | `snapshot             .workspace             .cwd             .iter()             .any` | [2000](../../src/config.rs#L2000) | receiver-type-required |
| `validate_snapshot` | `snapshot             .workspace             .cwd             .iter` | [2000](../../src/config.rs#L2000) | receiver-type-required |
| `validate_snapshot` | `Path::new(root).starts_with` | [2004](../../src/config.rs#L2004) | receiver-type-required |
| `validate_snapshot` | `validate_safe_integer` | [2013](../../src/config.rs#L2013) | [profile::config::validate_safe_integer](../../src/config.rs#L2060) |
| `validate_snapshot` | `validate_providers` | [2015](../../src/config.rs#L2015) | [profile::config::validate_providers](../../src/config.rs#L1549) |
| `validate_snapshot` | `validate_settings` | [2016](../../src/config.rs#L2016) | [profile::config::validate_settings](../../src/config.rs#L1646) |
| `validate_snapshot` | `validate_session_settings` | [2018](../../src/config.rs#L2018) | [profile::config::validate_session_settings](../../src/config.rs#L1668) |
| `validate_snapshot` | `ProvidersConfig::default` | [2020](../../src/config.rs#L2020) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `SettingsConfig::default` | [2026](../../src/config.rs#L2026) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `snapshot.workspace.id.clone` | [2035](../../src/config.rs#L2035) | receiver-type-required |
| `validate_snapshot` | `snapshot.workspace.name.clone` | [2036](../../src/config.rs#L2036) | receiver-type-required |
| `validate_snapshot` | `snapshot.workspace.cwd.clone` | [2037](../../src/config.rs#L2037) | receiver-type-required |
| `validate_snapshot` | `Vec::new` | [2038](../../src/config.rs#L2038) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `Some` | [2039](../../src/config.rs#L2039) | external-constructor-callback-or-unresolved |
| `validate_snapshot` | `snapshot.workspace.policy.clone` | [2039](../../src/config.rs#L2039) | receiver-type-required |
| `validate_snapshot` | `validate_references` | [2041](../../src/config.rs#L2041) | [profile::config::validate_references](../../src/config.rs#L1708) |
| `validate_snapshot` | `validate_session_reference` | [2042](../../src/config.rs#L2042) | [profile::config::validate_session_reference](../../src/config.rs#L1682) |
| `validate_snapshot` | `snapshot                 .session_settings                 .as_ref()                 .map` | [2047](../../src/config.rs#L2047) | receiver-type-required |
| `validate_snapshot` | `snapshot                 .session_settings                 .as_ref` | [2047](../../src/config.rs#L2047) | receiver-type-required |
| `validate_snapshot` | `Ok` | [2057](../../src/config.rs#L2057) | external-constructor-callback-or-unresolved |
| `validate_safe_integer` | `Ok` | [2062](../../src/config.rs#L2062) | external-constructor-callback-or-unresolved |
| `validate_safe_integer` | `invalid` | [2064](../../src/config.rs#L2064) | [profile::config::invalid](../../src/config.rs#L2126) |
| `is_sorted_unique` | `values.windows(2).all` | [2072](../../src/config.rs#L2072) | receiver-type-required |
| `is_sorted_unique` | `values.windows` | [2072](../../src/config.rs#L2072) | receiver-type-required |
| `canonical_directory` | `validate_absolute` | [2076](../../src/config.rs#L2076) | [profile::config::validate_absolute](../../src/config.rs#L2096) |
| `canonical_directory` | `fs::canonicalize(value).map_err` | [2077](../../src/config.rs#L2077) | receiver-type-required |
| `canonical_directory` | `fs::canonicalize` | [2077](../../src/config.rs#L2077) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `PathBuf::from` | [2078](../../src/config.rs#L2078) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `error.to_string` | [2079](../../src/config.rs#L2079) | receiver-type-required |
| `canonical_directory` | `canonical.is_dir` | [2081](../../src/config.rs#L2081) | receiver-type-required |
| `canonical_directory` | `Err` | [2082](../../src/config.rs#L2082) | external-constructor-callback-or-unresolved |
| `canonical_directory` | `"expected a directory".to_owned` | [2084](../../src/config.rs#L2084) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str()         .map(ToOwned::to_owned)         .ok_or_else` | [2087](../../src/config.rs#L2087) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str()         .map` | [2087](../../src/config.rs#L2087) | receiver-type-required |
| `canonical_directory` | `canonical         .to_str` | [2087](../../src/config.rs#L2087) | receiver-type-required |
| `canonical_directory` | `"path is not UTF-8".to_owned` | [2092](../../src/config.rs#L2092) | receiver-type-required |
| `validate_absolute` | `value.contains` | [2097](../../src/config.rs#L2097) | receiver-type-required |
| `validate_absolute` | `Path::new(value).is_absolute` | [2097](../../src/config.rs#L2097) | receiver-type-required |
| `validate_absolute` | `Path::new` | [2097](../../src/config.rs#L2097) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `Err` | [2098](../../src/config.rs#L2098) | external-constructor-callback-or-unresolved |
| `validate_absolute` | `source.to_path_buf` | [2099](../../src/config.rs#L2099) | receiver-type-required |
| `validate_absolute` | `Ok` | [2103](../../src/config.rs#L2103) | external-constructor-callback-or-unresolved |
| `validate_id` | `value.is_empty` | [2107](../../src/config.rs#L2107) | receiver-type-required |
| `validate_id` | `value.chars().any` | [2107](../../src/config.rs#L2107) | receiver-type-required |
| `validate_id` | `value.chars` | [2107](../../src/config.rs#L2107) | receiver-type-required |
| `validate_id` | `invalid` | [2108](../../src/config.rs#L2108) | [profile::config::invalid](../../src/config.rs#L2126) |
| `validate_id` | `Ok` | [2113](../../src/config.rs#L2113) | external-constructor-callback-or-unresolved |
| `validate_unique` | `values.iter().collect::<BTreeSet<_>>` | [2118](../../src/config.rs#L2118) | receiver-type-required |
| `validate_unique` | `values.iter` | [2118](../../src/config.rs#L2118) | receiver-type-required |
| `validate_unique` | `unique.len` | [2119](../../src/config.rs#L2119) | receiver-type-required |
| `validate_unique` | `values.len` | [2119](../../src/config.rs#L2119) | receiver-type-required |
| `validate_unique` | `Ok` | [2120](../../src/config.rs#L2120) | external-constructor-callback-or-unresolved |
| `validate_unique` | `invalid` | [2122](../../src/config.rs#L2122) | [profile::config::invalid](../../src/config.rs#L2126) |
| `invalid` | `Err` | [2127](../../src/config.rs#L2127) | external-constructor-callback-or-unresolved |
| `invalid` | `path.to_path_buf` | [2128](../../src/config.rs#L2128) | receiver-type-required |
| `invalid` | `reason.to_owned` | [2129](../../src/config.rs#L2129) | receiver-type-required |
| `invalid_reference` | `Err` | [2134](../../src/config.rs#L2134) | external-constructor-callback-or-unresolved |
| `invalid_reference` | `path.to_path_buf` | [2135](../../src/config.rs#L2135) | receiver-type-required |
| `invalid_reference` | `reason.to_owned` | [2136](../../src/config.rs#L2136) | receiver-type-required |
| `removed` | `current.iter().collect::<BTreeSet<_>>` | [2141](../../src/config.rs#L2141) | receiver-type-required |
| `removed` | `current.iter` | [2141](../../src/config.rs#L2141) | receiver-type-required |
| `removed` | `previous.iter().any` | [2142](../../src/config.rs#L2142) | receiver-type-required |
| `removed` | `previous.iter` | [2142](../../src/config.rs#L2142) | receiver-type-required |
| `removed` | `current.contains` | [2142](../../src/config.rs#L2142) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter()         .flat_map(&#124;provider&#124; {             provider                 .models                 .iter()                 .filter(&#124;model&#124; model.enabled)                 .map(move &#124;model&#124; (provider.id.as_str(), model.id.as_str()))         })         .collect` | [2154](../../src/config.rs#L2154) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter()         .flat_map` | [2154](../../src/config.rs#L2154) | receiver-type-required |
| `enabled_provider_models` | `providers         .providers         .iter` | [2154](../../src/config.rs#L2154) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter()                 .filter(&#124;model&#124; model.enabled)                 .map` | [2158](../../src/config.rs#L2158) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter()                 .filter` | [2158](../../src/config.rs#L2158) | receiver-type-required |
| `enabled_provider_models` | `provider                 .models                 .iter` | [2158](../../src/config.rs#L2158) | receiver-type-required |
| `enabled_provider_models` | `provider.id.as_str` | [2162](../../src/config.rs#L2162) | receiver-type-required |
| `enabled_provider_models` | `model.id.as_str` | [2162](../../src/config.rs#L2162) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `tempfile::tempdir().expect` | [2179](../../src/config.rs#L2179) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `tempfile::tempdir` | [2179](../../src/config.rs#L2179) | external-constructor-callback-or-unresolved |
| `private_directory_rejects_wrong_owner` | `rustix::process::geteuid().as_raw` | [2180](../../src/config.rs#L2180) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `rustix::process::geteuid` | [2180](../../src/config.rs#L2180) | external-constructor-callback-or-unresolved |
| `private_directory_rejects_wrong_owner` | `open_private_root(root.path(), actual_uid, false).expect` | [2181](../../src/config.rs#L2181) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `open_private_root` | [2181](../../src/config.rs#L2181) | [profile::config::open_private_root](../../src/config.rs#L575) |
| `private_directory_rejects_wrong_owner` | `root.path` | [2181](../../src/config.rs#L2181), [2183](../../src/config.rs#L2183) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `validate_private_directory_fd(&descriptor, root.path(), actual_uid.wrapping_add(1))                 .expect_err` | [2183](../../src/config.rs#L2183) | receiver-type-required |
| `private_directory_rejects_wrong_owner` | `validate_private_directory_fd` | [2183](../../src/config.rs#L2183) | [profile::config::validate_private_directory_fd](../../src/config.rs#L507) |
| `private_directory_rejects_wrong_owner` | `actual_uid.wrapping_add` | [2183](../../src/config.rs#L2183) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `tempfile::tempdir().expect` | [2191](../../src/config.rs#L2191) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `tempfile::tempdir` | [2191](../../src/config.rs#L2191) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `root.path().join` | [2192](../../src/config.rs#L2192), [2194](../../src/config.rs#L2194) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `root.path` | [2192](../../src/config.rs#L2192), [2194](../../src/config.rs#L2194) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::fs::create_dir(&directory).expect` | [2193](../../src/config.rs#L2193) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::fs::create_dir` | [2193](../../src/config.rs#L2193) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::os::unix::fs::symlink(&directory, &alias).expect` | [2195](../../src/config.rs#L2195) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `std::os::unix::fs::symlink` | [2195](../../src/config.rs#L2195) | external-constructor-callback-or-unresolved |
| `sandbox_fallback_rejects_a_symlinked_path` | `open_private_root_sandboxed(             &alias,             &alias,             OFlags::RDONLY &#124; OFlags::DIRECTORY &#124; OFlags::CLOEXEC &#124; OFlags::NOFOLLOW,             rustix::process::geteuid().as_raw(),         )         .expect_err` | [2196](../../src/config.rs#L2196) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `open_private_root_sandboxed` | [2196](../../src/config.rs#L2196) | [profile::config::open_private_root_sandboxed](../../src/config.rs#L737) |
| `sandbox_fallback_rejects_a_symlinked_path` | `rustix::process::geteuid().as_raw` | [2200](../../src/config.rs#L2200) | receiver-type-required |
| `sandbox_fallback_rejects_a_symlinked_path` | `rustix::process::geteuid` | [2200](../../src/config.rs#L2200) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `tempfile::tempdir().unwrap` | [2213](../../src/config.rs#L2213) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `tempfile::tempdir` | [2213](../../src/config.rs#L2213) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `root.path().join` | [2214](../../src/config.rs#L2214), [2215](../../src/config.rs#L2215), [2218](../../src/config.rs#L2218), [2247](../../src/config.rs#L2247) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `root.path` | [2214](../../src/config.rs#L2214), [2215](../../src/config.rs#L2215), [2218](../../src/config.rs#L2218), [2247](../../src/config.rs#L2247) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `fs::create_dir_all(&workspace).unwrap` | [2216](../../src/config.rs#L2216) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `fs::create_dir_all` | [2216](../../src/config.rs#L2216), [2217](../../src/config.rs#L2217) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `fs::create_dir_all(chain.join("bin")).unwrap` | [2217](../../src/config.rs#L2217) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `chain.join` | [2217](../../src/config.rs#L2217) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `std::os::unix::fs::symlink(&chain, &alias).unwrap` | [2219](../../src/config.rs#L2219) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `std::os::unix::fs::symlink` | [2219](../../src/config.rs#L2219) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `"ws".into` | [2223](../../src/config.rs#L2223), [2224](../../src/config.rs#L2224) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `Some` | [2227](../../src/config.rs#L2227) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `Default::default` | [2232](../../src/config.rs#L2232) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `resolve_workspace(&config, Path::new("test.json")).unwrap` | [2235](../../src/config.rs#L2235) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `resolve_workspace` | [2235](../../src/config.rs#L2235) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `Path::new` | [2235](../../src/config.rs#L2235) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `fs::create_dir(&ambiguous).unwrap` | [2248](../../src/config.rs#L2248) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `fs::create_dir` | [2248](../../src/config.rs#L2248) | external-constructor-callback-or-unresolved |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `config.policy.as_mut().unwrap` | [2249](../../src/config.rs#L2249) | receiver-type-required |
| `toolchain_roots_are_canonical_deduplicated_and_never_writable_roots` | `config.policy.as_mut` | [2249](../../src/config.rs#L2249) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `tempfile::tempdir().unwrap` | [2261](../../src/config.rs#L2261) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `tempfile::tempdir` | [2261](../../src/config.rs#L2261) | external-constructor-callback-or-unresolved |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `std::fs::canonicalize(root.path()).unwrap` | [2262](../../src/config.rs#L2262) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `std::fs::canonicalize` | [2262](../../src/config.rs#L2262) | external-constructor-callback-or-unresolved |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `root.path` | [2262](../../src/config.rs#L2262) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `schema::IJsonValue::parse(&serde_json::to_vec(&legacy).unwrap())             .expect("i-json")             .canonical_bytes()             .expect` | [2274](../../src/config.rs#L2274) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `schema::IJsonValue::parse(&serde_json::to_vec(&legacy).unwrap())             .expect("i-json")             .canonical_bytes` | [2274](../../src/config.rs#L2274) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `schema::IJsonValue::parse(&serde_json::to_vec(&legacy).unwrap())             .expect` | [2274](../../src/config.rs#L2274) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `schema::IJsonValue::parse` | [2274](../../src/config.rs#L2274) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `serde_json::to_vec(&legacy).unwrap` | [2274](../../src/config.rs#L2274) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `serde_json::to_vec` | [2274](../../src/config.rs#L2274) | external-constructor-callback-or-unresolved |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `bytes.push` | [2278](../../src/config.rs#L2278) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `ConfigSnapshot::decode(&bytes).expect` | [2279](../../src/config.rs#L2279) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `ConfigSnapshot::decode` | [2279](../../src/config.rs#L2279) | external-constructor-callback-or-unresolved |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `snapshot.canonical_bytes().expect` | [2280](../../src/config.rs#L2280) | receiver-type-required |
| `a_snapshot_published_with_the_integrations_file_still_decodes` | `snapshot.canonical_bytes` | [2280](../../src/config.rs#L2280) | receiver-type-required |
