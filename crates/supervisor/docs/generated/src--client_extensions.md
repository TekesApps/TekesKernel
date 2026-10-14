# tekes-supervisor::client_extensions

[Package atlas](index.md) · [Source](../../src/client_extensions.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::client_extensions::RESOURCE_METHODS](../../src/client_extensions.rs#L34) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::INITIAL_PRESET_METHODS](../../src/client_extensions.rs#L41) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::SESSION_PRESET_METHODS](../../src/client_extensions.rs#L43) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::RECOVERY_METHODS](../../src/client_extensions.rs#L47) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::ATTACHMENT_METHODS](../../src/client_extensions.rs#L49) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::APPROVAL_METHODS](../../src/client_extensions.rs#L54) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::FEEDBACK_METHODS](../../src/client_extensions.rs#L59) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::SETTINGS_METHODS](../../src/client_extensions.rs#L64) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::GOAL_METHODS](../../src/client_extensions.rs#L70) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::SUBAGENT_METHODS](../../src/client_extensions.rs#L77) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::HOST_FILE_METHODS](../../src/client_extensions.rs#L78) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::FILE_METHODS](../../src/client_extensions.rs#L84) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::FilePageRequest](../../src/client_extensions.rs#L92) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::AttachmentPolicyRequest](../../src/client_extensions.rs#L101) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ApprovalModeRequest](../../src/client_extensions.rs#L107) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ApprovalSelectRequest](../../src/client_extensions.rs#L113) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::UploadFileRequest](../../src/client_extensions.rs#L120) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::FileAttachmentRequest](../../src/client_extensions.rs#L129) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::RecoveryRequest](../../src/client_extensions.rs#L136) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::TOOL_METHODS](../../src/client_extensions.rs#L139) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::PLUGIN_METHODS](../../src/client_extensions.rs#L143) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::MCP_METHODS](../../src/client_extensions.rs#L153) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::SCHEDULE_METHODS](../../src/client_extensions.rs#L162) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::THREAD_SEARCH_METHODS](../../src/client_extensions.rs#L168) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::USAGE_METHODS](../../src/client_extensions.rs#L172) | const_item | `pub` |  |
| [tekes-supervisor::client_extensions::ClientExtensionAuthority](../../src/client_extensions.rs#L180) | trait_item | `pub` |  |
| [tekes-supervisor::client_extensions::ClientExtensionAuthority::execute](../../src/client_extensions.rs#L181) | function_signature_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes](../../src/client_extensions.rs#L191) | struct_item | `pub` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::new](../../src/client_extensions.rs#L199) | function_item | `pub` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::id](../../src/client_extensions.rs#L216) | function_item | `pub` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::class](../../src/client_extensions.rs#L220) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::capabilities](../../src/client_extensions.rs#L228) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::extension_method_class](../../src/client_extensions.rs#L235) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::validate_extension_payload](../../src/client_extensions.rs#L239) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::extension_failure_is_exact](../../src/client_extensions.rs#L254) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::execute](../../src/client_extensions.rs#L265) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::common_capability_routes](../../src/client_extensions.rs#L309) | function_item | `pub` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions](../../src/client_extensions.rs#L344) | struct_item | `pub` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::open](../../src/client_extensions.rs#L355) | function_item | `pub` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::attachments](../../src/client_extensions.rs#L380) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::routes](../../src/client_extensions.rs#L384) | function_item | `pub` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_store](../../src/client_extensions.rs#L423) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_snapshot](../../src/client_extensions.rs#L435) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog](../../src/client_extensions.rs#L450) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_operation](../../src/client_extensions.rs#L463) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal](../../src/client_extensions.rs#L560) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal_locked](../../src/client_extensions.rs#L573) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::complete_prepared_plugin_operation](../../src/client_extensions.rs#L636) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::OptionalWorkspace](../../src/client_extensions.rs#L713) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::EmptyRequest](../../src/client_extensions.rs#L718) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginIdRequest](../../src/client_extensions.rs#L721) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginInspectRequest](../../src/client_extensions.rs#L726) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginInstallRequest](../../src/client_extensions.rs#L731) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEnabledRequest](../../src/client_extensions.rs#L741) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginGrantsRequest](../../src/client_extensions.rs#L747) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ResourceListRequest](../../src/client_extensions.rs#L753) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ResourceReadRequest](../../src/client_extensions.rs#L761) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ResourceReference](../../src/client_extensions.rs#L768) | enum_item | `private` |  |
| [tekes-supervisor::client_extensions::ToolListRequest](../../src/client_extensions.rs#L786) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ToolResolveRequest](../../src/client_extensions.rs#L792) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ScheduleSaveRequest](../../src/client_extensions.rs#L799) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::TaskIdRequest](../../src/client_extensions.rs#L804) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::SessionSearchRequest](../../src/client_extensions.rs#L809) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::ThreadSearchRequest](../../src/client_extensions.rs#L814) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::SessionIdRequest](../../src/client_extensions.rs#L823) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointPhase](../../src/client_extensions.rs#L829) | enum_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointFailure](../../src/client_extensions.rs#L837) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointRecord](../../src/client_extensions.rs#L845) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation](../../src/client_extensions.rs#L860) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation::publish](../../src/client_extensions.rs#L868) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation::prepare](../../src/client_extensions.rs#L885) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation::complete](../../src/client_extensions.rs#L890) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation::fail](../../src/client_extensions.rs#L900) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::PluginEndpointOperation::completed](../../src/client_extensions.rs#L918) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::read_plugin_endpoint_record](../../src/client_extensions.rs#L955) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::plugin_view](../../src/client_extensions.rs#L1015) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::lock](../../src/client_extensions.rs#L1054) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::decode_resource_cursor](../../src/client_extensions.rs#L1063) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::normalize_mcp_resource](../../src/client_extensions.rs#L1086) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::numeric_token](../../src/client_extensions.rs#L1152) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::CacheAttributionGroup](../../src/client_extensions.rs#L1166) | struct_item | `private` |  |
| [tekes-supervisor::client_extensions::CacheAttributionGroup::new](../../src/client_extensions.rs#L1177) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::CacheAttributionGroup::add](../../src/client_extensions.rs#L1189) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::CacheAttributionGroup::value](../../src/client_extensions.rs#L1227) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::canonical_unsigned](../../src/client_extensions.rs#L1256) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::add_unsigned_decimal](../../src/client_extensions.rs#L1269) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::add_decimal_digits](../../src/client_extensions.rs#L1275) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::canonical_cost](../../src/client_extensions.rs#L1315) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::add_canonical_cost](../../src/client_extensions.rs#L1342) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::usage_route_target](../../src/client_extensions.rs#L1368) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::mcp_readiness_code](../../src/client_extensions.rs#L1488) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::map_mcp_resource_error](../../src/client_extensions.rs#L1495) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::map_plugin_error](../../src/client_extensions.rs#L1507) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::plugin_mutation_result](../../src/client_extensions.rs#L1528) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::plugin_error_is_precommit](../../src/client_extensions.rs#L1542) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::map_schedule_error](../../src/client_extensions.rs#L1560) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::map_search_error](../../src/client_extensions.rs#L1576) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::picker_home](../../src/client_extensions.rs#L1598) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::session_folder](../../src/client_extensions.rs#L1606) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::session_controls](../../src/client_extensions.rs#L1622) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::host_files](../../src/client_extensions.rs#L1656) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../src/client_extensions.rs#L1743) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::resources_list](../../src/client_extensions.rs#L2214) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::resources_read](../../src/client_extensions.rs#L2301) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::tools](../../src/client_extensions.rs#L2376) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_mutation_value](../../src/client_extensions.rs#L2443) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_readiness](../../src/client_extensions.rs#L2448) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_list](../../src/client_extensions.rs#L2457) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_save](../../src/client_extensions.rs#L2464) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_delete](../../src/client_extensions.rs#L2510) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_run_now](../../src/client_extensions.rs#L2532) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::thread_search](../../src/client_extensions.rs#L2561) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::ProductionClientExtensions::usage](../../src/client_extensions.rs#L2580) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::map_attachment_read_error](../../src/client_extensions.rs#L2801) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::error_allowed](../../src/client_extensions.rs#L2829) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::validate_payload](../../src/client_extensions.rs#L2922) | function_item | `private` |  |
| [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) | function_item | `pub(crate)` |  |
| [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::initial_presets_disclose_profile_text_without_creating_a_session](../../src/client_extensions.rs#L3127) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::session_presets_describe_and_resolve_without_creating_a_session](../../src/client_extensions.rs#L3170) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::session_preset_method_cases_are_the_runtime_results](../../src/client_extensions.rs#L3268) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::write_plugin](../../src/client_extensions.rs#L3311) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::package_digest](../../src/client_extensions.rs#L3348) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::install_payload](../../src/client_extensions.rs#L3357) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::approvals_select_persists_the_session_mode_and_refuses_archived_sessions](../../src/client_extensions.rs#L3390) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::tools_list_is_closed_and_includes_every_fixed_tool](../../src/client_extensions.rs#L3495) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::schedule_save_accepts_a_valid_multi_root_workspace](../../src/client_extensions.rs#L3545) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b](../../src/client_extensions.rs#L3585) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::plugin_journal_orders_remove_reinstall_and_install_update](../../src/client_extensions.rs#L3632) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::plugin_precommit_failures_are_terminal_and_do_not_wedge_management](../../src/client_extensions.rs#L3710) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::skill_resources_never_cross_configured_workspace_catalogs](../../src/client_extensions.rs#L3785) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::append_canonical](../../src/client_extensions.rs#L3896) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::epoch_profile_asset](../../src/client_extensions.rs#L3901) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_extensions::tests::usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets](../../src/client_extensions.rs#L3930) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `HashSet` | `std::collections::HashSet` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `_` | `base64::Engine` | `private` |
| `Utc` | `chrono::Utc` | `private` |
| `DurableHandoffProof` | `endpoint::DurableHandoffProof` | `private` |
| `EndpointHostCall` | `endpoint::EndpointHostCall` | `private` |
| `MethodClass` | `endpoint::MethodClass` | `private` |
| `RpcDurableIdentity` | `endpoint::RpcDurableIdentity` | `private` |
| `InstallOptions` | `plugins::InstallOptions` | `private` |
| `MacOsNativeHelperVerifier` | `plugins::MacOsNativeHelperVerifier` | `private` |
| `PluginReceipt` | `plugins::PluginReceipt` | `private` |
| `PluginSource` | `plugins::PluginSource` | `private` |
| `PluginStore` | `plugins::PluginStore` | `private` |
| `ResourceCatalog` | `profile::ResourceCatalog` | `private` |
| `EventKind` | `schema::EventKind` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `_` | `sha2::Digest` | `private` |
| `ArchiveVisibility` | `thread_search::ArchiveVisibility` | `private` |
| `SearchRequest` | `thread_search::SearchRequest` | `private` |
| `ThreadSearchAuthority` | `thread_search::ThreadSearchAuthority` | `private` |
| `BuiltinManifest` | `tools::BuiltinManifest` | `private` |
| `fixed_schema` | `tools::fixed_schema` | `private` |
| `ProductionEndpointRoutes` | `crate::endpoint_host::ProductionEndpointRoutes` | `private` |
| `ProductionRouteFailure` | `crate::endpoint_host::ProductionRouteFailure` | `private` |
| `McpManagementRoutes` | `crate::mcp_runtime::McpManagementRoutes` | `private` |
| `McpRuntime` | `crate::mcp_runtime::McpRuntime` | `private` |
| `ProductionProcessHost` | `crate::process_host::ProductionProcessHost` | `private` |
| `ClientResourceService` | `crate::resource_capability::ClientResourceService` | `private` |
| `CommandRunRequest` | `crate::resource_capability::CommandRunRequest` | `private` |
| `EndpointCommandInputAuthority` | `crate::resource_capability::EndpointCommandInputAuthority` | `private` |
| `map_prompt_materialize_error` | `crate::resource_capability::map_prompt_materialize_error` | `private` |
| `map_resource_failure` | `crate::resource_capability::map_resource_failure` | `private` |
| `fs` | `std::fs` | `private` |
| `EndpointHost` | `endpoint::EndpointHost` | `private` |
| `SessionHostDescription` | `endpoint::SessionHostDescription` | `private` |
| `SessionDeliveryAuthority` | `crate::endpoint_host::SessionDeliveryAuthority` | `private` |
| `SessionInputAdmissionAuthority` | `crate::endpoint_host::SessionInputAdmissionAuthority` | `private` |
| `assemble_application_endpoint_host` | `crate::host_runtime::assemble_application_endpoint_host` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::client_extensions::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 34 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::attachment::AttachmentAuthority::open"]
  n1["plugins::archive::PluginSource::from_path"]
  n2["profile::config::ConfigRepository::open"]
  n3["store::atomic::AtomicPublisher::replace"]
  n4["store::platform::NamedLock::exclusive"]
  n5["tekes-supervisor::client_extensions::decode"]
  n6["tekes-supervisor::client_extensions::lock"]
  n7["tekes-supervisor::client_extensions::plugin_mutation_result"]
  n8["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::new"]
  n9["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::id"]
  n10["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::class"]
  n11["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::capabilities"]
  n12["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::extension_method_class"]
  n13["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::validate_extension_payload"]
  n14["tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_mutation_value"]
  n15["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::extension_failure_is_exact"]
  n16["tekes-supervisor::client_extensions::AtomicCapabilityRoutes::execute"]
  n17["tekes-supervisor::client_extensions::error_allowed"]
  n18["tekes-supervisor::client_extensions::validate_payload"]
  n19["tekes-supervisor::client_extensions::failure"]
  n20["tekes-supervisor::client_extensions::common_capability_routes"]
  n21["tekes-supervisor::client_extensions::ProductionClientExtensions::open"]
  n22["tekes-supervisor::client_extensions::ProductionClientExtensions::attachments"]
  n23["tekes-supervisor::client_extensions::ProductionClientExtensions::routes"]
  n24["tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_store"]
  n25["tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_snapshot"]
  n26["tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog"]
  n27["tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_operation"]
  n28["tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal"]
  n29["tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal_locked"]
  n30["tekes-supervisor::client_extensions::ProductionClientExtensions::complete_prepared_plugin_operation"]
  n31["tekes-supervisor::client_extensions::PluginEndpointOperation::publish"]
  n32["tekes-supervisor::client_extensions::read_plugin_endpoint_record"]
  n33["tekes-supervisor::mcp_runtime::McpManagementRoutes::new"]
  n34["tekes-supervisor::resource_capability::ClientResourceService::new"]
  n35["thread-search::ThreadSearchAuthority::open"]
  n13 --> n18
  n13 --> n19
  n15 --> n17
  n16 --> n19
  n20 --> n8
  n21 --> n0
  n21 --> n34
  n21 --> n35
  n22 --> n0
  n23 --> n8
  n23 --> n33
  n24 --> n19
  n25 --> n2
  n25 --> n19
  n26 --> n19
  n27 --> n4
  n27 --> n19
  n27 --> n29
  n27 --> n32
  n28 --> n4
  n28 --> n19
  n28 --> n29
  n29 --> n19
  n29 --> n30
  n29 --> n32
  n30 --> n1
  n30 --> n5
  n30 --> n6
  n30 --> n7
  n30 --> n14
  n30 --> n19
  n30 --> n24
  n31 --> n3
  n31 --> n19
```

</details>

<details><summary>Functions 21–40: 25 direct edges</summary>

```mermaid
flowchart TD
  n0["schema::ijson::IJsonValue::parse"]
  n1["tekes-supervisor::client_extensions::plugin_view"]
  n2["tekes-supervisor::client_extensions::decode"]
  n3["tekes-supervisor::client_extensions::encode"]
  n4["tekes-supervisor::client_extensions::ijson_value"]
  n5["tekes-supervisor::client_extensions::bad_request"]
  n6["tekes-supervisor::client_extensions::lock"]
  n7["tekes-supervisor::client_extensions::decode_resource_cursor"]
  n8["tekes-supervisor::client_extensions::normalize_mcp_resource"]
  n9["tekes-supervisor::client_extensions::numeric_token"]
  n10["tekes-supervisor::client_extensions::CacheAttributionGroup::new"]
  n11["tekes-supervisor::client_extensions::CacheAttributionGroup::add"]
  n12["tekes-supervisor::client_extensions::CacheAttributionGroup::value"]
  n13["tekes-supervisor::client_extensions::canonical_unsigned"]
  n14["tekes-supervisor::client_extensions::add_unsigned_decimal"]
  n15["tekes-supervisor::client_extensions::add_decimal_digits"]
  n16["tekes-supervisor::client_extensions::canonical_cost"]
  n17["tekes-supervisor::client_extensions::add_canonical_cost"]
  n18["tekes-supervisor::client_extensions::failure"]
  n19["tekes-supervisor::client_extensions::PluginEndpointOperation::publish"]
  n20["tekes-supervisor::client_extensions::PluginEndpointOperation::prepare"]
  n21["tekes-supervisor::client_extensions::PluginEndpointOperation::complete"]
  n22["tekes-supervisor::client_extensions::PluginEndpointOperation::fail"]
  n23["tekes-supervisor::client_extensions::PluginEndpointOperation::completed"]
  n24["tekes-supervisor::client_extensions::read_plugin_endpoint_record"]
  n2 --> n5
  n3 --> n0
  n3 --> n18
  n4 --> n18
  n5 --> n18
  n6 --> n18
  n7 --> n18
  n8 --> n18
  n9 --> n18
  n11 --> n14
  n11 --> n16
  n11 --> n17
  n11 --> n18
  n12 --> n18
  n13 --> n18
  n14 --> n13
  n14 --> n15
  n15 --> n18
  n20 --> n19
  n21 --> n3
  n21 --> n19
  n22 --> n4
  n22 --> n19
  n23 --> n18
  n24 --> n18
```

</details>

<details><summary>Functions 41–60: 67 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::management::SessionPresetEnvironment::new"]
  n1["endpoint::management::ManagementStore::open_light"]
  n2["endpoint::service::NativeEndpoint::open"]
  n3["endpoint::types::validate_session_id"]
  n4["engine::permission_mode::write_permission_mode"]
  n5["engine::permission_mode::PermissionMode::parse"]
  n6["engine::permission_mode::read_permission_mode"]
  n7["host-files::list_directory"]
  n8["host-files::create_directory"]
  n9["host-files::file_references"]
  n10["host-files::session_references"]
  n11["plugins::archive::PluginSource::from_path"]
  n12["profile::config::ConfigRepository::open"]
  n13["profile::instruction::InstructionResolver::new_scoped"]
  n14["schema::ijson::IJsonValue::parse"]
  n15["session-controls::execute_goal"]
  n16["session-controls::subagent_catalog"]
  n17["tekes-supervisor::client_extensions::decode"]
  n18["tekes-supervisor::client_extensions::encode"]
  n19["tekes-supervisor::client_extensions::ijson_value"]
  n20["tekes-supervisor::client_extensions::bad_request"]
  n21["tekes-supervisor::client_extensions::lock"]
  n22["tekes-supervisor::client_extensions::decode_resource_cursor"]
  n23["tekes-supervisor::client_extensions::normalize_mcp_resource"]
  n24["tekes-supervisor::client_extensions::canonical_unsigned"]
  n25["tekes-supervisor::client_extensions::add_decimal_digits"]
  n26["tekes-supervisor::client_extensions::canonical_cost"]
  n27["tekes-supervisor::client_extensions::add_canonical_cost"]
  n28["tekes-supervisor::client_extensions::usage_route_target"]
  n29["tekes-supervisor::client_extensions::mcp_readiness_code"]
  n30["tekes-supervisor::client_extensions::map_mcp_resource_error"]
  n31["tekes-supervisor::client_extensions::map_plugin_error"]
  n32["tekes-supervisor::client_extensions::plugin_mutation_result"]
  n33["tekes-supervisor::client_extensions::plugin_error_is_precommit"]
  n34["tekes-supervisor::client_extensions::map_schedule_error"]
  n35["tekes-supervisor::client_extensions::map_search_error"]
  n36["tekes-supervisor::client_extensions::ProductionClientExtensions::picker_home"]
  n37["tekes-supervisor::client_extensions::ProductionClientExtensions::session_folder"]
  n38["tekes-supervisor::client_extensions::ProductionClientExtensions::session_controls"]
  n39["tekes-supervisor::client_extensions::ProductionClientExtensions::host_files"]
  n40["tekes-supervisor::client_extensions::ProductionClientExtensions::execute"]
  n41["tekes-supervisor::client_extensions::ProductionClientExtensions::resources_list"]
  n42["tekes-supervisor::client_extensions::ProductionClientExtensions::resources_read"]
  n43["tekes-supervisor::client_extensions::ProductionClientExtensions::tools"]
  n44["tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_mutation_value"]
  n45["tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_readiness"]
  n46["tekes-supervisor::client_extensions::map_attachment_read_error"]
  n47["tekes-supervisor::client_extensions::failure"]
  n48["tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog"]
  n49["tekes-supervisor::resource_capability::map_prompt_materialize_error"]
  n50["tools::builtin::BuiltinManifest::compiled"]
  n51["tools::preset::ResolvedPreset::resolve_request"]
  n52["tools::preset::describe_value"]
  n53["tools::schema_registry::fixed_schema"]
  n54["user-documents::execute"]
  n26 --> n24
  n26 --> n47
  n27 --> n25
  n27 --> n26
  n28 --> n14
  n28 --> n19
  n28 --> n47
  n30 --> n47
  n31 --> n47
  n32 --> n31
  n32 --> n33
  n34 --> n47
  n35 --> n47
  n37 --> n3
  n37 --> n20
  n37 --> n47
  n38 --> n15
  n38 --> n16
  n38 --> n18
  n38 --> n37
  n38 --> n47
  n39 --> n1
  n39 --> n2
  n39 --> n7
  n39 --> n8
  n39 --> n9
  n39 --> n10
  n39 --> n18
  n39 --> n20
  n39 --> n36
  n39 --> n47
  n40 --> n0
  n40 --> n4
  n40 --> n5
  n40 --> n6
  n40 --> n11
  n40 --> n12
  n40 --> n13
  n40 --> n17
  n40 --> n18
  n40 --> n20
  n40 --> n21
  n40 --> n46
  n40 --> n47
  n40 --> n49
  n40 --> n51
  n40 --> n52
  n40 --> n54
  n41 --> n17
  n41 --> n18
  n41 --> n20
  n41 --> n22
  n41 --> n47
  n41 --> n48
  n42 --> n17
  n42 --> n18
  n42 --> n19
  n42 --> n20
  n42 --> n23
  n42 --> n47
  n42 --> n48
  n43 --> n18
  n43 --> n19
  n43 --> n47
  n43 --> n50
  n43 --> n53
  n44 --> n45
```

</details>

<details><summary>Functions 61–70: 34 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["engine::permission_mode::PermissionMode::parse"]
  n2["host-files::validate"]
  n3["schema::ijson::IJsonValue::parse"]
  n4["session-controls::validate"]
  n5["store::asset::AssetStore::new"]
  n6["store::tail::scan_valid_prefix"]
  n7["tekes-supervisor::client_extensions::decode"]
  n8["tekes-supervisor::client_extensions::encode"]
  n9["tekes-supervisor::client_extensions::ijson_value"]
  n10["tekes-supervisor::client_extensions::bad_request"]
  n11["tekes-supervisor::client_extensions::numeric_token"]
  n12["tekes-supervisor::client_extensions::CacheAttributionGroup::new"]
  n13["tekes-supervisor::client_extensions::canonical_unsigned"]
  n14["tekes-supervisor::client_extensions::usage_route_target"]
  n15["tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_list"]
  n16["tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_save"]
  n17["tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_delete"]
  n18["tekes-supervisor::client_extensions::ProductionClientExtensions::schedule_run_now"]
  n19["tekes-supervisor::client_extensions::ProductionClientExtensions::thread_search"]
  n20["tekes-supervisor::client_extensions::ProductionClientExtensions::usage"]
  n21["tekes-supervisor::client_extensions::map_attachment_read_error"]
  n22["tekes-supervisor::client_extensions::error_allowed"]
  n23["tekes-supervisor::client_extensions::validate_payload"]
  n24["tekes-supervisor::client_extensions::failure"]
  n25["tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_snapshot"]
  n26["tekes-supervisor::endpoint_host::ProductionRouteFailure::new"]
  n27["user-documents::validate"]
  n15 --> n7
  n15 --> n8
  n16 --> n7
  n16 --> n8
  n16 --> n24
  n16 --> n25
  n17 --> n7
  n17 --> n8
  n18 --> n7
  n18 --> n8
  n18 --> n24
  n19 --> n7
  n19 --> n8
  n20 --> n5
  n20 --> n6
  n20 --> n7
  n20 --> n8
  n20 --> n9
  n20 --> n11
  n20 --> n12
  n20 --> n13
  n20 --> n14
  n20 --> n24
  n21 --> n24
  n23 --> n0
  n23 --> n1
  n23 --> n2
  n23 --> n4
  n23 --> n7
  n23 --> n10
  n23 --> n24
  n23 --> n27
  n24 --> n3
  n24 --> n26
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `class` | `self.methods             .iter()             .find_map` | [221](../../src/client_extensions.rs#L221) | receiver-type-required |
| `class` | `self.methods             .iter` | [221](../../src/client_extensions.rs#L221) | receiver-type-required |
| `class` | `(*name == operation).then_some` | [223](../../src/client_extensions.rs#L223) | receiver-type-required |
| `capabilities` | `self.methods             .iter()             .map(&#124;(name, _)&#124; (*name).to_owned())             .collect` | [229](../../src/client_extensions.rs#L229) | receiver-type-required |
| `capabilities` | `self.methods             .iter()             .map` | [229](../../src/client_extensions.rs#L229) | receiver-type-required |
| `capabilities` | `self.methods             .iter` | [229](../../src/client_extensions.rs#L229) | receiver-type-required |
| `capabilities` | `(*name).to_owned` | [231](../../src/client_extensions.rs#L231) | receiver-type-required |
| `extension_method_class` | `self.class` | [236](../../src/client_extensions.rs#L236) | receiver-type-required |
| `validate_extension_payload` | `self.class(operation).is_none` | [244](../../src/client_extensions.rs#L244) | receiver-type-required |
| `validate_extension_payload` | `self.class` | [244](../../src/client_extensions.rs#L244) | receiver-type-required |
| `validate_extension_payload` | `payload.is_object` | [244](../../src/client_extensions.rs#L244) | receiver-type-required |
| `validate_extension_payload` | `Err` | [245](../../src/client_extensions.rs#L245) | external-constructor-callback-or-unresolved |
| `validate_extension_payload` | `failure` | [245](../../src/client_extensions.rs#L245) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `validate_extension_payload` | `validate_payload` | [251](../../src/client_extensions.rs#L251) | [tekes-supervisor::client_extensions::validate_payload](../../src/client_extensions.rs#L2922) |
| `extension_failure_is_exact` | `error_allowed` | [262](../../src/client_extensions.rs#L262) | [tekes-supervisor::client_extensions::error_allowed](../../src/client_extensions.rs#L2829) |
| `extension_failure_is_exact` | `self.class` | [262](../../src/client_extensions.rs#L262) | receiver-type-required |
| `execute` | `self.class(&request.operation).is_none` | [271](../../src/client_extensions.rs#L271) | receiver-type-required |
| `execute` | `self.class` | [271](../../src/client_extensions.rs#L271), [285](../../src/client_extensions.rs#L285) | receiver-type-required |
| `execute` | `Err` | [272](../../src/client_extensions.rs#L272) | external-constructor-callback-or-unresolved |
| `execute` | `failure` | [272](../../src/client_extensions.rs#L272), [297](../../src/client_extensions.rs#L297) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `execute` | `self.authority.execute` | [278](../../src/client_extensions.rs#L278) | receiver-type-required |
| `execute` | `Some` | [285](../../src/client_extensions.rs#L285), [290](../../src/client_extensions.rs#L290) | external-constructor-callback-or-unresolved |
| `execute` | `request                 .handoff                 .mark_handed_off(DurableHandoffProof {                     delivery: "extension-authority-barrier".to_owned(),                     durable_identity: Some(RpcDurableIdentity {                         kind: "extension-rpc".to_owned(),                         id: request.rpc_id.clone(),                         seq: None,                     }),                 })                 .map_err` | [286](../../src/client_extensions.rs#L286) | receiver-type-required |
| `execute` | `request                 .handoff                 .mark_handed_off` | [286](../../src/client_extensions.rs#L286) | receiver-type-required |
| `execute` | `"extension-authority-barrier".to_owned` | [289](../../src/client_extensions.rs#L289) | receiver-type-required |
| `execute` | `"extension-rpc".to_owned` | [291](../../src/client_extensions.rs#L291) | receiver-type-required |
| `execute` | `request.rpc_id.clone` | [292](../../src/client_extensions.rs#L292) | receiver-type-required |
| `execute` | `Ok` | [304](../../src/client_extensions.rs#L304) | external-constructor-callback-or-unresolved |
| `common_capability_routes` | `[         ("resources.v1", RESOURCE_METHODS.as_slice()),         ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),         ("sessionPresets.v1", SESSION_PRESET_METHODS.as_slice()),         ("recovery.v1", RECOVERY_METHODS.as_slice()),         ("attachments.v1", ATTACHMENT_METHODS.as_slice()),         ("approvals.v1", APPROVAL_METHODS.as_slice()),         ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),         ("feedback.v1", FEEDBACK_METHODS.as_slice()),         ("settings.v1", SETTINGS_METHODS.as_slice()),         ("goals.v1", GOAL_METHODS.as_slice()),         ("subagents.v1", SUBAGENT_METHODS.as_slice()),         ("sessionFiles.v1", FILE_METHODS.as_slice()),         ("tools.v1", TOOL_METHODS.as_slice()),         ("plugins.v1", PLUGIN_METHODS.as_slice()),         ("schedule.v1", SCHEDULE_METHODS.as_slice()),         ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),         ("usage.v1", USAGE_METHODS.as_slice()),     ]     .into_iter()     .map(&#124;(id, methods)&#124; {         Arc::new(AtomicCapabilityRoutes::new(             id,             methods,             Arc::clone(&authority),         )) as Arc<dyn ProductionEndpointRoutes>     })     .collect` | [312](../../src/client_extensions.rs#L312) | receiver-type-required |
| `common_capability_routes` | `[         ("resources.v1", RESOURCE_METHODS.as_slice()),         ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),         ("sessionPresets.v1", SESSION_PRESET_METHODS.as_slice()),         ("recovery.v1", RECOVERY_METHODS.as_slice()),         ("attachments.v1", ATTACHMENT_METHODS.as_slice()),         ("approvals.v1", APPROVAL_METHODS.as_slice()),         ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),         ("feedback.v1", FEEDBACK_METHODS.as_slice()),         ("settings.v1", SETTINGS_METHODS.as_slice()),         ("goals.v1", GOAL_METHODS.as_slice()),         ("subagents.v1", SUBAGENT_METHODS.as_slice()),         ("sessionFiles.v1", FILE_METHODS.as_slice()),         ("tools.v1", TOOL_METHODS.as_slice()),         ("plugins.v1", PLUGIN_METHODS.as_slice()),         ("schedule.v1", SCHEDULE_METHODS.as_slice()),         ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),         ("usage.v1", USAGE_METHODS.as_slice()),     ]     .into_iter()     .map` | [312](../../src/client_extensions.rs#L312) | receiver-type-required |
| `common_capability_routes` | `[         ("resources.v1", RESOURCE_METHODS.as_slice()),         ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),         ("sessionPresets.v1", SESSION_PRESET_METHODS.as_slice()),         ("recovery.v1", RECOVERY_METHODS.as_slice()),         ("attachments.v1", ATTACHMENT_METHODS.as_slice()),         ("approvals.v1", APPROVAL_METHODS.as_slice()),         ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),         ("feedback.v1", FEEDBACK_METHODS.as_slice()),         ("settings.v1", SETTINGS_METHODS.as_slice()),         ("goals.v1", GOAL_METHODS.as_slice()),         ("subagents.v1", SUBAGENT_METHODS.as_slice()),         ("sessionFiles.v1", FILE_METHODS.as_slice()),         ("tools.v1", TOOL_METHODS.as_slice()),         ("plugins.v1", PLUGIN_METHODS.as_slice()),         ("schedule.v1", SCHEDULE_METHODS.as_slice()),         ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),         ("usage.v1", USAGE_METHODS.as_slice()),     ]     .into_iter` | [312](../../src/client_extensions.rs#L312) | receiver-type-required |
| `common_capability_routes` | `RESOURCE_METHODS.as_slice` | [313](../../src/client_extensions.rs#L313) | receiver-type-required |
| `common_capability_routes` | `INITIAL_PRESET_METHODS.as_slice` | [314](../../src/client_extensions.rs#L314) | receiver-type-required |
| `common_capability_routes` | `SESSION_PRESET_METHODS.as_slice` | [315](../../src/client_extensions.rs#L315) | receiver-type-required |
| `common_capability_routes` | `RECOVERY_METHODS.as_slice` | [316](../../src/client_extensions.rs#L316) | receiver-type-required |
| `common_capability_routes` | `ATTACHMENT_METHODS.as_slice` | [317](../../src/client_extensions.rs#L317) | receiver-type-required |
| `common_capability_routes` | `APPROVAL_METHODS.as_slice` | [318](../../src/client_extensions.rs#L318) | receiver-type-required |
| `common_capability_routes` | `HOST_FILE_METHODS.as_slice` | [319](../../src/client_extensions.rs#L319) | receiver-type-required |
| `common_capability_routes` | `FEEDBACK_METHODS.as_slice` | [320](../../src/client_extensions.rs#L320) | receiver-type-required |
| `common_capability_routes` | `SETTINGS_METHODS.as_slice` | [321](../../src/client_extensions.rs#L321) | receiver-type-required |
| `common_capability_routes` | `GOAL_METHODS.as_slice` | [322](../../src/client_extensions.rs#L322) | receiver-type-required |
| `common_capability_routes` | `SUBAGENT_METHODS.as_slice` | [323](../../src/client_extensions.rs#L323) | receiver-type-required |
| `common_capability_routes` | `FILE_METHODS.as_slice` | [324](../../src/client_extensions.rs#L324) | receiver-type-required |
| `common_capability_routes` | `TOOL_METHODS.as_slice` | [325](../../src/client_extensions.rs#L325) | receiver-type-required |
| `common_capability_routes` | `PLUGIN_METHODS.as_slice` | [326](../../src/client_extensions.rs#L326) | receiver-type-required |
| `common_capability_routes` | `SCHEDULE_METHODS.as_slice` | [327](../../src/client_extensions.rs#L327) | receiver-type-required |
| `common_capability_routes` | `THREAD_SEARCH_METHODS.as_slice` | [328](../../src/client_extensions.rs#L328) | receiver-type-required |
| `common_capability_routes` | `USAGE_METHODS.as_slice` | [329](../../src/client_extensions.rs#L329) | receiver-type-required |
| `common_capability_routes` | `Arc::new` | [333](../../src/client_extensions.rs#L333) | external-constructor-callback-or-unresolved |
| `common_capability_routes` | `AtomicCapabilityRoutes::new` | [333](../../src/client_extensions.rs#L333) | [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::new](../../src/client_extensions.rs#L199) |
| `common_capability_routes` | `Arc::clone` | [336](../../src/client_extensions.rs#L336) | external-constructor-callback-or-unresolved |
| `open` | `root.as_ref().to_path_buf` | [362](../../src/client_extensions.rs#L362) | receiver-type-required |
| `open` | `root.as_ref` | [362](../../src/client_extensions.rs#L362) | receiver-type-required |
| `open` | `process.plugin_store` | [363](../../src/client_extensions.rs#L363) | receiver-type-required |
| `open` | `process.mcp_runtime` | [364](../../src/client_extensions.rs#L364) | receiver-type-required |
| `open` | `commands.with_attachment_authority` | [366](../../src/client_extensions.rs#L366) | receiver-type-required |
| `open` | `endpoint::AttachmentAuthority::open` | [366](../../src/client_extensions.rs#L366) | [endpoint::attachment::AttachmentAuthority::open](../../../endpoint/src/attachment.rs#L242) |
| `open` | `Ok` | [367](../../src/client_extensions.rs#L367) | external-constructor-callback-or-unresolved |
| `open` | `ClientResourceService::new` | [368](../../src/client_extensions.rs#L368) | [tekes-supervisor::resource_capability::ClientResourceService::new](../../src/resource_capability.rs#L158) |
| `open` | `ThreadSearchAuthority::open(&root).map_err` | [372](../../src/client_extensions.rs#L372) | receiver-type-required |
| `open` | `ThreadSearchAuthority::open` | [372](../../src/client_extensions.rs#L372) | [thread-search::ThreadSearchAuthority::open](../../../thread-search/src/lib.rs#L95) |
| `attachments` | `endpoint::AttachmentAuthority::open` | [381](../../src/client_extensions.rs#L381) | [endpoint::attachment::AttachmentAuthority::open](../../../endpoint/src/attachment.rs#L242) |
| `routes` | `self.clone` | [385](../../src/client_extensions.rs#L385) | receiver-type-required |
| `routes` | `self.plugins.is_some` | [404](../../src/client_extensions.rs#L404) | receiver-type-required |
| `routes` | `groups.push` | [405](../../src/client_extensions.rs#L405) | receiver-type-required |
| `routes` | `PLUGIN_METHODS.as_slice` | [405](../../src/client_extensions.rs#L405) | receiver-type-required |
| `routes` | `groups             .into_iter()             .map(&#124;(id, methods)&#124; {                 Arc::new(AtomicCapabilityRoutes::new(                     id,                     methods,                     Arc::clone(&authority),                 )) as Arc<dyn ProductionEndpointRoutes>             })             .collect::<Vec<_>>` | [407](../../src/client_extensions.rs#L407) | receiver-type-required |
| `routes` | `groups             .into_iter()             .map` | [407](../../src/client_extensions.rs#L407) | receiver-type-required |
| `routes` | `groups             .into_iter` | [407](../../src/client_extensions.rs#L407) | receiver-type-required |
| `routes` | `Arc::new` | [410](../../src/client_extensions.rs#L410), [417](../../src/client_extensions.rs#L417) | external-constructor-callback-or-unresolved |
| `routes` | `AtomicCapabilityRoutes::new` | [410](../../src/client_extensions.rs#L410) | [tekes-supervisor::client_extensions::AtomicCapabilityRoutes::new](../../src/client_extensions.rs#L199) |
| `routes` | `Arc::clone` | [413](../../src/client_extensions.rs#L413) | external-constructor-callback-or-unresolved |
| `routes` | `routes.push` | [417](../../src/client_extensions.rs#L417) | receiver-type-required |
| `routes` | `McpManagementRoutes::new` | [417](../../src/client_extensions.rs#L417) | [tekes-supervisor::mcp_runtime::McpManagementRoutes::new](../../src/mcp_runtime.rs#L1367) |
| `routes` | `self.process.mcp_runtime` | [418](../../src/client_extensions.rs#L418) | receiver-type-required |
| `plugin_store` | `self.plugins.as_deref().ok_or_else` | [426](../../src/client_extensions.rs#L426) | receiver-type-required |
| `plugin_store` | `self.plugins.as_deref` | [426](../../src/client_extensions.rs#L426) | receiver-type-required |
| `plugin_store` | `failure` | [427](../../src/client_extensions.rs#L427) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `workspace_snapshot` | `profile::ConfigRepository::open(&self.root)             .and_then(&#124;repository&#124; repository.resolve(workspace_id))             .map_err` | [439](../../src/client_extensions.rs#L439) | receiver-type-required |
| `workspace_snapshot` | `profile::ConfigRepository::open(&self.root)             .and_then` | [439](../../src/client_extensions.rs#L439) | receiver-type-required |
| `workspace_snapshot` | `profile::ConfigRepository::open` | [439](../../src/client_extensions.rs#L439) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `workspace_snapshot` | `repository.resolve` | [440](../../src/client_extensions.rs#L440) | receiver-type-required |
| `workspace_snapshot` | `failure` | [442](../../src/client_extensions.rs#L442) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `workspace_resource_catalog` | `self.workspace_resources.get(workspace_id).ok_or_else` | [454](../../src/client_extensions.rs#L454) | receiver-type-required |
| `workspace_resource_catalog` | `self.workspace_resources.get` | [454](../../src/client_extensions.rs#L454) | receiver-type-required |
| `workspace_resource_catalog` | `failure` | [455](../../src/client_extensions.rs#L455) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `plugin_operation` | `self.root.join` | [469](../../src/client_extensions.rs#L469) | receiver-type-required |
| `plugin_operation` | `authority.join` | [471](../../src/client_extensions.rs#L471), [472](../../src/client_extensions.rs#L472), [501](../../src/client_extensions.rs#L501), [510](../../src/client_extensions.rs#L510), [511](../../src/client_extensions.rs#L511) | receiver-type-required |
| `plugin_operation` | `std::fs::create_dir_all(&directory).map_err` | [474](../../src/client_extensions.rs#L474) | receiver-type-required |
| `plugin_operation` | `std::fs::create_dir_all` | [474](../../src/client_extensions.rs#L474) | external-constructor-callback-or-unresolved |
| `plugin_operation` | `failure` | [475](../../src/client_extensions.rs#L475), [484](../../src/client_extensions.rs#L484), [494](../../src/client_extensions.rs#L494), [502](../../src/client_extensions.rs#L502), [532](../../src/client_extensions.rs#L532) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `plugin_operation` | `std::fs::File::open(&directory)                 .and_then(&#124;file&#124; file.sync_all())                 .map_err` | [481](../../src/client_extensions.rs#L481) | receiver-type-required |
| `plugin_operation` | `std::fs::File::open(&directory)                 .and_then` | [481](../../src/client_extensions.rs#L481) | receiver-type-required |
| `plugin_operation` | `std::fs::File::open` | [481](../../src/client_extensions.rs#L481), [491](../../src/client_extensions.rs#L491) | external-constructor-callback-or-unresolved |
| `plugin_operation` | `file.sync_all` | [482](../../src/client_extensions.rs#L482), [492](../../src/client_extensions.rs#L492) | receiver-type-required |
| `plugin_operation` | `std::fs::File::open(&authority)             .and_then(&#124;file&#124; file.sync_all())             .map_err` | [491](../../src/client_extensions.rs#L491) | receiver-type-required |
| `plugin_operation` | `std::fs::File::open(&authority)             .and_then` | [491](../../src/client_extensions.rs#L491) | receiver-type-required |
| `plugin_operation` | `store::NamedLock::exclusive(authority.join(".endpoint-rpc.lock")).map_err` | [501](../../src/client_extensions.rs#L501) | receiver-type-required |
| `plugin_operation` | `store::NamedLock::exclusive` | [501](../../src/client_extensions.rs#L501) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `plugin_operation` | `self.recover_plugin_endpoint_journal_locked` | [508](../../src/client_extensions.rs#L508) | [tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal_locked](../../src/client_extensions.rs#L573) |
| `plugin_operation` | `authority.join("endpoint-rpc").join` | [510](../../src/client_extensions.rs#L510) | receiver-type-required |
| `plugin_operation` | `authority.join("endpoint-sources").join` | [511](../../src/client_extensions.rs#L511) | receiver-type-required |
| `plugin_operation` | `path.is_file` | [526](../../src/client_extensions.rs#L526) | receiver-type-required |
| `plugin_operation` | `read_plugin_endpoint_record` | [527](../../src/client_extensions.rs#L527) | [tekes-supervisor::client_extensions::read_plugin_endpoint_record](../../src/client_extensions.rs#L955) |
| `plugin_operation` | `Err` | [532](../../src/client_extensions.rs#L532) | external-constructor-callback-or-unresolved |
| `plugin_operation` | `rpc_id.to_owned` | [542](../../src/client_extensions.rs#L542) | receiver-type-required |
| `plugin_operation` | `operation.to_owned` | [543](../../src/client_extensions.rs#L543) | receiver-type-required |
| `plugin_operation` | `payload.clone` | [546](../../src/client_extensions.rs#L546) | receiver-type-required |
| `plugin_operation` | `Ok` | [552](../../src/client_extensions.rs#L552) | external-constructor-callback-or-unresolved |
| `plugin_operation` | `Some` | [553](../../src/client_extensions.rs#L553) | external-constructor-callback-or-unresolved |
| `recover_plugin_endpoint_journal` | `self.root.join` | [561](../../src/client_extensions.rs#L561) | receiver-type-required |
| `recover_plugin_endpoint_journal` | `store::NamedLock::exclusive(authority.join(".endpoint-rpc.lock")).map_err` | [563](../../src/client_extensions.rs#L563) | receiver-type-required |
| `recover_plugin_endpoint_journal` | `store::NamedLock::exclusive` | [563](../../src/client_extensions.rs#L563) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `recover_plugin_endpoint_journal` | `authority.join` | [563](../../src/client_extensions.rs#L563) | receiver-type-required |
| `recover_plugin_endpoint_journal` | `failure` | [564](../../src/client_extensions.rs#L564) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `recover_plugin_endpoint_journal` | `self.recover_plugin_endpoint_journal_locked` | [570](../../src/client_extensions.rs#L570) | [tekes-supervisor::client_extensions::ProductionClientExtensions::recover_plugin_endpoint_journal_locked](../../src/client_extensions.rs#L573) |
| `recover_plugin_endpoint_journal_locked` | `authority.join` | [577](../../src/client_extensions.rs#L577), [628](../../src/client_extensions.rs#L628) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `Vec::new` | [578](../../src/client_extensions.rs#L578) | external-constructor-callback-or-unresolved |
| `recover_plugin_endpoint_journal_locked` | `directory.is_dir` | [579](../../src/client_extensions.rs#L579) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `std::fs::read_dir(&directory).map_err` | [580](../../src/client_extensions.rs#L580) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `std::fs::read_dir` | [580](../../src/client_extensions.rs#L580) | external-constructor-callback-or-unresolved |
| `recover_plugin_endpoint_journal_locked` | `failure` | [581](../../src/client_extensions.rs#L581), [589](../../src/client_extensions.rs#L589), [597](../../src/client_extensions.rs#L597), [610](../../src/client_extensions.rs#L610), [619](../../src/client_extensions.rs#L619) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `recover_plugin_endpoint_journal_locked` | `entry                     .map_err(&#124;error&#124; {                         failure(                             "internal",                             "Plugin receipt directory is unreadable",                             serde_json::json!({"reason":error.to_string()}),                         )                     })?                     .path` | [587](../../src/client_extensions.rs#L587) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `entry                     .map_err` | [587](../../src/client_extensions.rs#L587) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `path.extension().and_then` | [596](../../src/client_extensions.rs#L596) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `path.extension` | [596](../../src/client_extensions.rs#L596) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `value.to_str` | [596](../../src/client_extensions.rs#L596), [618](../../src/client_extensions.rs#L618) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `Some` | [596](../../src/client_extensions.rs#L596), [618](../../src/client_extensions.rs#L618) | external-constructor-callback-or-unresolved |
| `recover_plugin_endpoint_journal_locked` | `Err` | [597](../../src/client_extensions.rs#L597), [610](../../src/client_extensions.rs#L610), [619](../../src/client_extensions.rs#L619) | external-constructor-callback-or-unresolved |
| `recover_plugin_endpoint_journal_locked` | `read_plugin_endpoint_record` | [603](../../src/client_extensions.rs#L603) | [tekes-supervisor::client_extensions::read_plugin_endpoint_record](../../src/client_extensions.rs#L955) |
| `recover_plugin_endpoint_journal_locked` | `prepared.push` | [605](../../src/client_extensions.rs#L605) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `prepared.len` | [609](../../src/client_extensions.rs#L609) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `prepared.pop` | [616](../../src/client_extensions.rs#L616) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `path.file_stem().and_then` | [618](../../src/client_extensions.rs#L618) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `path.file_stem` | [618](../../src/client_extensions.rs#L618) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `authority.join("endpoint-sources").join` | [628](../../src/client_extensions.rs#L628) | receiver-type-required |
| `recover_plugin_endpoint_journal_locked` | `self.complete_prepared_plugin_operation` | [631](../../src/client_extensions.rs#L631) | [tekes-supervisor::client_extensions::ProductionClientExtensions::complete_prepared_plugin_operation](../../src/client_extensions.rs#L636) |
| `recover_plugin_endpoint_journal_locked` | `Ok` | [633](../../src/client_extensions.rs#L633) | external-constructor-callback-or-unresolved |
| `complete_prepared_plugin_operation` | `operation.record.operation.as_str` | [640](../../src/client_extensions.rs#L640) | receiver-type-required |
| `complete_prepared_plugin_operation` | `decode` | [642](../../src/client_extensions.rs#L642), [667](../../src/client_extensions.rs#L667), [675](../../src/client_extensions.rs#L675), [684](../../src/client_extensions.rs#L684) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `complete_prepared_plugin_operation` | `PluginSource::from_path(&operation.stage).map_err` | [643](../../src/client_extensions.rs#L643) | receiver-type-required |
| `complete_prepared_plugin_operation` | `PluginSource::from_path` | [643](../../src/client_extensions.rs#L643) | [plugins::archive::PluginSource::from_path](../../../plugins/src/archive.rs#L28) |
| `complete_prepared_plugin_operation` | `lock` | [644](../../src/client_extensions.rs#L644), [668](../../src/client_extensions.rs#L668), [676](../../src/client_extensions.rs#L676), [692](../../src/client_extensions.rs#L692) | [tekes-supervisor::client_extensions::lock](../../src/client_extensions.rs#L1054) |
| `complete_prepared_plugin_operation` | `self.plugin_store` | [644](../../src/client_extensions.rs#L644), [668](../../src/client_extensions.rs#L668), [676](../../src/client_extensions.rs#L676), [692](../../src/client_extensions.rs#L692) | [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_store](../../src/client_extensions.rs#L423) |
| `complete_prepared_plugin_operation` | `store.inspect(&source).map_err` | [645](../../src/client_extensions.rs#L645) | receiver-type-required |
| `complete_prepared_plugin_operation` | `store.inspect` | [645](../../src/client_extensions.rs#L645) | receiver-type-required |
| `complete_prepared_plugin_operation` | `Err` | [647](../../src/client_extensions.rs#L647), [699](../../src/client_extensions.rs#L699) | external-constructor-callback-or-unresolved |
| `complete_prepared_plugin_operation` | `failure` | [647](../../src/client_extensions.rs#L647), [686](../../src/client_extensions.rs#L686), [699](../../src/client_extensions.rs#L699) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `complete_prepared_plugin_operation` | `store.install` | [653](../../src/client_extensions.rs#L653) | receiver-type-required |
| `complete_prepared_plugin_operation` | `request.grants.into_iter().collect` | [656](../../src/client_extensions.rs#L656), [678](../../src/client_extensions.rs#L678) | receiver-type-required |
| `complete_prepared_plugin_operation` | `request.grants.into_iter` | [656](../../src/client_extensions.rs#L656), [678](../../src/client_extensions.rs#L678) | receiver-type-required |
| `complete_prepared_plugin_operation` | `plugin_mutation_result` | [662](../../src/client_extensions.rs#L662), [670](../../src/client_extensions.rs#L670), [679](../../src/client_extensions.rs#L679), [694](../../src/client_extensions.rs#L694) | [tekes-supervisor::client_extensions::plugin_mutation_result](../../src/client_extensions.rs#L1528) |
| `complete_prepared_plugin_operation` | `drop` | [663](../../src/client_extensions.rs#L663), [671](../../src/client_extensions.rs#L671), [680](../../src/client_extensions.rs#L680), [695](../../src/client_extensions.rs#L695) | external-constructor-callback-or-unresolved |
| `complete_prepared_plugin_operation` | `self.plugin_mutation_value` | [664](../../src/client_extensions.rs#L664), [672](../../src/client_extensions.rs#L672), [681](../../src/client_extensions.rs#L681) | [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_mutation_value](../../src/client_extensions.rs#L2443) |
| `complete_prepared_plugin_operation` | `store.set_enabled` | [669](../../src/client_extensions.rs#L669) | receiver-type-required |
| `complete_prepared_plugin_operation` | `store.set_grants` | [678](../../src/client_extensions.rs#L678) | receiver-type-required |
| `complete_prepared_plugin_operation` | `operation.record.originally_present.ok_or_else` | [685](../../src/client_extensions.rs#L685) | receiver-type-required |
| `complete_prepared_plugin_operation` | `store.remove` | [693](../../src/client_extensions.rs#L693) | receiver-type-required |
| `complete_prepared_plugin_operation` | `operation.complete` | [706](../../src/client_extensions.rs#L706) | receiver-type-required |
| `complete_prepared_plugin_operation` | `Ok` | [707](../../src/client_extensions.rs#L707) | external-constructor-callback-or-unresolved |
| `publish` | `serde_json_canonicalizer::to_vec(&self.record).map_err` | [869](../../src/client_extensions.rs#L869) | receiver-type-required |
| `publish` | `serde_json_canonicalizer::to_vec` | [869](../../src/client_extensions.rs#L869) | external-constructor-callback-or-unresolved |
| `publish` | `failure` | [870](../../src/client_extensions.rs#L870), [877](../../src/client_extensions.rs#L877) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `publish` | `store::AtomicPublisher::replace(&self.path, &bytes).map_err` | [876](../../src/client_extensions.rs#L876) | receiver-type-required |
| `publish` | `store::AtomicPublisher::replace` | [876](../../src/client_extensions.rs#L876) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `prepare` | `self.publish` | [887](../../src/client_extensions.rs#L887) | [tekes-supervisor::client_extensions::PluginEndpointOperation::publish](../../src/client_extensions.rs#L868) |
| `complete` | `Some` | [892](../../src/client_extensions.rs#L892) | external-constructor-callback-or-unresolved |
| `complete` | `result.clone` | [892](../../src/client_extensions.rs#L892) | receiver-type-required |
| `complete` | `self.publish` | [893](../../src/client_extensions.rs#L893) | [tekes-supervisor::client_extensions::PluginEndpointOperation::publish](../../src/client_extensions.rs#L868) |
| `complete` | `self.stage.is_dir` | [894](../../src/client_extensions.rs#L894) | receiver-type-required |
| `complete` | `std::fs::remove_dir_all` | [895](../../src/client_extensions.rs#L895) | external-constructor-callback-or-unresolved |
| `complete` | `encode` | [897](../../src/client_extensions.rs#L897) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `fail` | `ijson_value` | [904](../../src/client_extensions.rs#L904) | [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) |
| `fail` | `Some` | [906](../../src/client_extensions.rs#L906) | external-constructor-callback-or-unresolved |
| `fail` | `error.code.clone` | [907](../../src/client_extensions.rs#L907) | receiver-type-required |
| `fail` | `error.message.clone` | [908](../../src/client_extensions.rs#L908) | receiver-type-required |
| `fail` | `self.publish` | [911](../../src/client_extensions.rs#L911) | [tekes-supervisor::client_extensions::PluginEndpointOperation::publish](../../src/client_extensions.rs#L868) |
| `fail` | `self.stage.is_dir` | [912](../../src/client_extensions.rs#L912) | receiver-type-required |
| `fail` | `std::fs::remove_dir_all` | [913](../../src/client_extensions.rs#L913) | external-constructor-callback-or-unresolved |
| `fail` | `Ok` | [915](../../src/client_extensions.rs#L915) | external-constructor-callback-or-unresolved |
| `completed` | `Ok` | [920](../../src/client_extensions.rs#L920) | external-constructor-callback-or-unresolved |
| `completed` | `self.stage.is_dir` | [922](../../src/client_extensions.rs#L922) | receiver-type-required |
| `completed` | `std::fs::remove_dir_all` | [923](../../src/client_extensions.rs#L923) | external-constructor-callback-or-unresolved |
| `completed` | `self.record                     .result                     .as_ref()                     .map(encode)                     .transpose()                     .and_then(&#124;result&#124; {                         result.ok_or_else(&#124;&#124; {                             failure(                                 "internal",                                 "Complete plugin receipt omitted result",                                 serde_json::json!({}),                             )                         })                     })                     .map` | [925](../../src/client_extensions.rs#L925) | receiver-type-required |
| `completed` | `self.record                     .result                     .as_ref()                     .map(encode)                     .transpose()                     .and_then` | [925](../../src/client_extensions.rs#L925) | receiver-type-required |
| `completed` | `self.record                     .result                     .as_ref()                     .map(encode)                     .transpose` | [925](../../src/client_extensions.rs#L925) | receiver-type-required |
| `completed` | `self.record                     .result                     .as_ref()                     .map` | [925](../../src/client_extensions.rs#L925) | receiver-type-required |
| `completed` | `self.record                     .result                     .as_ref` | [925](../../src/client_extensions.rs#L925) | receiver-type-required |
| `completed` | `result.ok_or_else` | [931](../../src/client_extensions.rs#L931) | receiver-type-required |
| `completed` | `failure` | [932](../../src/client_extensions.rs#L932), [943](../../src/client_extensions.rs#L943), [949](../../src/client_extensions.rs#L949) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `completed` | `self.record.failure.as_ref().ok_or_else` | [942](../../src/client_extensions.rs#L942) | receiver-type-required |
| `completed` | `self.record.failure.as_ref` | [942](../../src/client_extensions.rs#L942) | receiver-type-required |
| `completed` | `Err` | [949](../../src/client_extensions.rs#L949) | external-constructor-callback-or-unresolved |
| `completed` | `error.details.clone` | [949](../../src/client_extensions.rs#L949) | receiver-type-required |
| `read_plugin_endpoint_record` | `std::fs::read(path).map_err` | [958](../../src/client_extensions.rs#L958) | receiver-type-required |
| `read_plugin_endpoint_record` | `std::fs::read` | [958](../../src/client_extensions.rs#L958) | external-constructor-callback-or-unresolved |
| `read_plugin_endpoint_record` | `failure` | [959](../../src/client_extensions.rs#L959), [966](../../src/client_extensions.rs#L966), [973](../../src/client_extensions.rs#L973), [985](../../src/client_extensions.rs#L985), [1006](../../src/client_extensions.rs#L1006) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `read_plugin_endpoint_record` | `serde_json::from_slice(&bytes).map_err` | [965](../../src/client_extensions.rs#L965) | receiver-type-required |
| `read_plugin_endpoint_record` | `serde_json::from_slice` | [965](../../src/client_extensions.rs#L965) | external-constructor-callback-or-unresolved |
| `read_plugin_endpoint_record` | `serde_json_canonicalizer::to_vec(&record).map_err` | [972](../../src/client_extensions.rs#L972) | receiver-type-required |
| `read_plugin_endpoint_record` | `serde_json_canonicalizer::to_vec` | [972](../../src/client_extensions.rs#L972) | external-constructor-callback-or-unresolved |
| `read_plugin_endpoint_record` | `record.result.is_none` | [980](../../src/client_extensions.rs#L980), [982](../../src/client_extensions.rs#L982) | receiver-type-required |
| `read_plugin_endpoint_record` | `record.failure.is_none` | [980](../../src/client_extensions.rs#L980), [981](../../src/client_extensions.rs#L981) | receiver-type-required |
| `read_plugin_endpoint_record` | `record.result.is_some` | [981](../../src/client_extensions.rs#L981) | receiver-type-required |
| `read_plugin_endpoint_record` | `record.failure.is_some` | [982](../../src/client_extensions.rs#L982) | receiver-type-required |
| `read_plugin_endpoint_record` | `Err` | [985](../../src/client_extensions.rs#L985), [1006](../../src/client_extensions.rs#L1006) | external-constructor-callback-or-unresolved |
| `read_plugin_endpoint_record` | `Ok` | [1012](../../src/client_extensions.rs#L1012) | external-constructor-callback-or-unresolved |
| `decode` | `serde_json::from_value(value.clone()).map_err` | [1025](../../src/client_extensions.rs#L1025) | receiver-type-required |
| `decode` | `serde_json::from_value` | [1025](../../src/client_extensions.rs#L1025) | external-constructor-callback-or-unresolved |
| `decode` | `value.clone` | [1025](../../src/client_extensions.rs#L1025) | receiver-type-required |
| `decode` | `bad_request` | [1025](../../src/client_extensions.rs#L1025) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `decode` | `error.to_string` | [1025](../../src/client_extensions.rs#L1025) | receiver-type-required |
| `encode` | `IJsonValue::parse(&serde_json::to_vec(value).map_err(&#124;_&#124; {         failure(             "internal",             "Response serialization failed",             serde_json::json!({}),         )     })?)     .map_err` | [1028](../../src/client_extensions.rs#L1028) | receiver-type-required |
| `encode` | `IJsonValue::parse` | [1028](../../src/client_extensions.rs#L1028) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `encode` | `serde_json::to_vec(value).map_err` | [1028](../../src/client_extensions.rs#L1028) | receiver-type-required |
| `encode` | `serde_json::to_vec` | [1028](../../src/client_extensions.rs#L1028) | external-constructor-callback-or-unresolved |
| `encode` | `failure` | [1029](../../src/client_extensions.rs#L1029), [1035](../../src/client_extensions.rs#L1035) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `ijson_value` | `serde_json::from_slice(&value.canonical_bytes().map_err(&#124;_&#124; {         failure(             "internal",             "I-JSON serialization failed",             serde_json::json!({}),         )     })?)     .map_err` | [1038](../../src/client_extensions.rs#L1038) | receiver-type-required |
| `ijson_value` | `serde_json::from_slice` | [1038](../../src/client_extensions.rs#L1038) | external-constructor-callback-or-unresolved |
| `ijson_value` | `value.canonical_bytes().map_err` | [1038](../../src/client_extensions.rs#L1038) | receiver-type-required |
| `ijson_value` | `value.canonical_bytes` | [1038](../../src/client_extensions.rs#L1038) | receiver-type-required |
| `ijson_value` | `failure` | [1039](../../src/client_extensions.rs#L1039), [1045](../../src/client_extensions.rs#L1045) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `bad_request` | `failure` | [1048](../../src/client_extensions.rs#L1048) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `lock` | `value.lock().map_err` | [1055](../../src/client_extensions.rs#L1055) | receiver-type-required |
| `lock` | `value.lock` | [1055](../../src/client_extensions.rs#L1055) | receiver-type-required |
| `lock` | `failure` | [1056](../../src/client_extensions.rs#L1056) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `decode_resource_cursor` | `value.rsplit_once(':').ok_or_else` | [1064](../../src/client_extensions.rs#L1064) | receiver-type-required |
| `decode_resource_cursor` | `value.rsplit_once` | [1064](../../src/client_extensions.rs#L1064) | receiver-type-required |
| `decode_resource_cursor` | `failure` | [1065](../../src/client_extensions.rs#L1065), [1072](../../src/client_extensions.rs#L1072), [1079](../../src/client_extensions.rs#L1079) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `decode_resource_cursor` | `Err` | [1072](../../src/client_extensions.rs#L1072) | external-constructor-callback-or-unresolved |
| `decode_resource_cursor` | `offset.parse().map_err` | [1078](../../src/client_extensions.rs#L1078) | receiver-type-required |
| `decode_resource_cursor` | `offset.parse` | [1078](../../src/client_extensions.rs#L1078) | receiver-type-required |
| `normalize_mcp_resource` | `value         .get("contents")         .and_then(Value::as_array)         .ok_or_else` | [1087](../../src/client_extensions.rs#L1087) | receiver-type-required |
| `normalize_mcp_resource` | `value         .get("contents")         .and_then` | [1087](../../src/client_extensions.rs#L1087) | receiver-type-required |
| `normalize_mcp_resource` | `value         .get` | [1087](../../src/client_extensions.rs#L1087) | receiver-type-required |
| `normalize_mcp_resource` | `failure` | [1091](../../src/client_extensions.rs#L1091), [1098](../../src/client_extensions.rs#L1098), [1105](../../src/client_extensions.rs#L1105), [1113](../../src/client_extensions.rs#L1113), [1126](../../src/client_extensions.rs#L1126), [1135](../../src/client_extensions.rs#L1135), [1142](../../src/client_extensions.rs#L1142) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `normalize_mcp_resource` | `contents.len` | [1097](../../src/client_extensions.rs#L1097) | receiver-type-required |
| `normalize_mcp_resource` | `Err` | [1098](../../src/client_extensions.rs#L1098), [1113](../../src/client_extensions.rs#L1113), [1142](../../src/client_extensions.rs#L1142) | external-constructor-callback-or-unresolved |
| `normalize_mcp_resource` | `contents[0].as_object().ok_or_else` | [1104](../../src/client_extensions.rs#L1104) | receiver-type-required |
| `normalize_mcp_resource` | `contents[0].as_object` | [1104](../../src/client_extensions.rs#L1104) | receiver-type-required |
| `normalize_mcp_resource` | `item.get("text").and_then` | [1111](../../src/client_extensions.rs#L1111) | receiver-type-required |
| `normalize_mcp_resource` | `item.get` | [1111](../../src/client_extensions.rs#L1111), [1120](../../src/client_extensions.rs#L1120), [1125](../../src/client_extensions.rs#L1125) | receiver-type-required |
| `normalize_mcp_resource` | `text.len` | [1112](../../src/client_extensions.rs#L1112) | receiver-type-required |
| `normalize_mcp_resource` | `item.get("mimeType").and_then` | [1120](../../src/client_extensions.rs#L1120) | receiver-type-required |
| `normalize_mcp_resource` | `Value::String` | [1121](../../src/client_extensions.rs#L1121) | external-constructor-callback-or-unresolved |
| `normalize_mcp_resource` | `mime.to_owned` | [1121](../../src/client_extensions.rs#L1121) | receiver-type-required |
| `normalize_mcp_resource` | `Ok` | [1123](../../src/client_extensions.rs#L1123), [1148](../../src/client_extensions.rs#L1148) | external-constructor-callback-or-unresolved |
| `normalize_mcp_resource` | `item.get("blob").and_then(Value::as_str).ok_or_else` | [1125](../../src/client_extensions.rs#L1125) | receiver-type-required |
| `normalize_mcp_resource` | `item.get("blob").and_then` | [1125](../../src/client_extensions.rs#L1125) | receiver-type-required |
| `normalize_mcp_resource` | `base64::engine::general_purpose::STANDARD         .decode(blob)         .map_err` | [1132](../../src/client_extensions.rs#L1132) | receiver-type-required |
| `normalize_mcp_resource` | `base64::engine::general_purpose::STANDARD         .decode` | [1132](../../src/client_extensions.rs#L1132) | receiver-type-required |
| `normalize_mcp_resource` | `decoded.len` | [1141](../../src/client_extensions.rs#L1141) | receiver-type-required |
| `numeric_token` | `value.and_then` | [1153](../../src/client_extensions.rs#L1153) | receiver-type-required |
| `numeric_token` | `Ok` | [1154](../../src/client_extensions.rs#L1154) | external-constructor-callback-or-unresolved |
| `numeric_token` | `value.parse().map_err` | [1155](../../src/client_extensions.rs#L1155) | receiver-type-required |
| `numeric_token` | `value.parse` | [1155](../../src/client_extensions.rs#L1155) | receiver-type-required |
| `numeric_token` | `failure` | [1156](../../src/client_extensions.rs#L1156) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `new` | `BTreeSet::new` | [1180](../../src/client_extensions.rs#L1180) | external-constructor-callback-or-unresolved |
| `new` | `"0".to_owned` | [1181](../../src/client_extensions.rs#L1181), [1182](../../src/client_extensions.rs#L1182), [1183](../../src/client_extensions.rs#L1183) | receiver-type-required |
| `add` | `self.attempts.insert` | [1197](../../src/client_extensions.rs#L1197) | receiver-type-required |
| `add` | `attempt.to_owned` | [1197](../../src/client_extensions.rs#L1197) | receiver-type-required |
| `add` | `Err` | [1198](../../src/client_extensions.rs#L1198), [1216](../../src/client_extensions.rs#L1216) | external-constructor-callback-or-unresolved |
| `add` | `failure` | [1198](../../src/client_extensions.rs#L1198), [1216](../../src/client_extensions.rs#L1216) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `add` | `add_unsigned_decimal` | [1204](../../src/client_extensions.rs#L1204), [1205](../../src/client_extensions.rs#L1205), [1206](../../src/client_extensions.rs#L1206) | [tekes-supervisor::client_extensions::add_unsigned_decimal](../../src/client_extensions.rs#L1269) |
| `add` | `Some` | [1209](../../src/client_extensions.rs#L1209) | external-constructor-callback-or-unresolved |
| `add` | `canonical_cost` | [1209](../../src/client_extensions.rs#L1209) | [tekes-supervisor::client_extensions::canonical_cost](../../src/client_extensions.rs#L1315) |
| `add` | `add_canonical_cost` | [1212](../../src/client_extensions.rs#L1212) | [tekes-supervisor::client_extensions::add_canonical_cost](../../src/client_extensions.rs#L1342) |
| `add` | `Ok` | [1224](../../src/client_extensions.rs#L1224) | external-constructor-callback-or-unresolved |
| `value` | `u64::try_from(self.attempts.len()).map_err` | [1228](../../src/client_extensions.rs#L1228) | receiver-type-required |
| `value` | `u64::try_from` | [1228](../../src/client_extensions.rs#L1228) | external-constructor-callback-or-unresolved |
| `value` | `self.attempts.len` | [1228](../../src/client_extensions.rs#L1228) | receiver-type-required |
| `value` | `failure` | [1229](../../src/client_extensions.rs#L1229), [1236](../../src/client_extensions.rs#L1236) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `value` | `Err` | [1236](../../src/client_extensions.rs#L1236) | external-constructor-callback-or-unresolved |
| `value` | `Value::String` | [1250](../../src/client_extensions.rs#L1250) | external-constructor-callback-or-unresolved |
| `value` | `Ok` | [1252](../../src/client_extensions.rs#L1252) | external-constructor-callback-or-unresolved |
| `canonical_unsigned` | `value.starts_with` | [1257](../../src/client_extensions.rs#L1257) | receiver-type-required |
| `canonical_unsigned` | `value.bytes().all` | [1257](../../src/client_extensions.rs#L1257) | receiver-type-required |
| `canonical_unsigned` | `value.bytes` | [1257](../../src/client_extensions.rs#L1257) | receiver-type-required |
| `canonical_unsigned` | `byte.is_ascii_digit` | [1257](../../src/client_extensions.rs#L1257) | receiver-type-required |
| `canonical_unsigned` | `Ok` | [1259](../../src/client_extensions.rs#L1259) | external-constructor-callback-or-unresolved |
| `canonical_unsigned` | `Err` | [1261](../../src/client_extensions.rs#L1261) | external-constructor-callback-or-unresolved |
| `canonical_unsigned` | `failure` | [1261](../../src/client_extensions.rs#L1261) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `add_unsigned_decimal` | `canonical_unsigned` | [1270](../../src/client_extensions.rs#L1270), [1271](../../src/client_extensions.rs#L1271) | [tekes-supervisor::client_extensions::canonical_unsigned](../../src/client_extensions.rs#L1256) |
| `add_unsigned_decimal` | `add_decimal_digits` | [1272](../../src/client_extensions.rs#L1272) | [tekes-supervisor::client_extensions::add_decimal_digits](../../src/client_extensions.rs#L1275) |
| `add_decimal_digits` | `left.bytes().all` | [1276](../../src/client_extensions.rs#L1276) | receiver-type-required |
| `add_decimal_digits` | `left.bytes` | [1276](../../src/client_extensions.rs#L1276), [1287](../../src/client_extensions.rs#L1287) | receiver-type-required |
| `add_decimal_digits` | `byte.is_ascii_digit` | [1276](../../src/client_extensions.rs#L1276), [1277](../../src/client_extensions.rs#L1277) | receiver-type-required |
| `add_decimal_digits` | `right.bytes().all` | [1277](../../src/client_extensions.rs#L1277) | receiver-type-required |
| `add_decimal_digits` | `right.bytes` | [1277](../../src/client_extensions.rs#L1277), [1288](../../src/client_extensions.rs#L1288) | receiver-type-required |
| `add_decimal_digits` | `Err` | [1279](../../src/client_extensions.rs#L1279) | external-constructor-callback-or-unresolved |
| `add_decimal_digits` | `failure` | [1279](../../src/client_extensions.rs#L1279), [1301](../../src/client_extensions.rs#L1301) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `add_decimal_digits` | `Vec::with_capacity` | [1286](../../src/client_extensions.rs#L1286) | external-constructor-callback-or-unresolved |
| `add_decimal_digits` | `left.len().max` | [1286](../../src/client_extensions.rs#L1286) | receiver-type-required |
| `add_decimal_digits` | `left.len` | [1286](../../src/client_extensions.rs#L1286) | receiver-type-required |
| `add_decimal_digits` | `right.len` | [1286](../../src/client_extensions.rs#L1286) | receiver-type-required |
| `add_decimal_digits` | `left.bytes().rev` | [1287](../../src/client_extensions.rs#L1287) | receiver-type-required |
| `add_decimal_digits` | `right.bytes().rev` | [1288](../../src/client_extensions.rs#L1288) | receiver-type-required |
| `add_decimal_digits` | `left.next().map` | [1290](../../src/client_extensions.rs#L1290) | receiver-type-required |
| `add_decimal_digits` | `left.next` | [1290](../../src/client_extensions.rs#L1290) | receiver-type-required |
| `add_decimal_digits` | `right.next().map` | [1291](../../src/client_extensions.rs#L1291) | receiver-type-required |
| `add_decimal_digits` | `right.next` | [1291](../../src/client_extensions.rs#L1291) | receiver-type-required |
| `add_decimal_digits` | `a.is_none` | [1292](../../src/client_extensions.rs#L1292) | receiver-type-required |
| `add_decimal_digits` | `b.is_none` | [1292](../../src/client_extensions.rs#L1292) | receiver-type-required |
| `add_decimal_digits` | `a.unwrap_or` | [1295](../../src/client_extensions.rs#L1295) | receiver-type-required |
| `add_decimal_digits` | `b.unwrap_or` | [1295](../../src/client_extensions.rs#L1295) | receiver-type-required |
| `add_decimal_digits` | `result.push` | [1296](../../src/client_extensions.rs#L1296) | receiver-type-required |
| `add_decimal_digits` | `result.reverse` | [1299](../../src/client_extensions.rs#L1299) | receiver-type-required |
| `add_decimal_digits` | `String::from_utf8(result).map_err` | [1300](../../src/client_extensions.rs#L1300) | receiver-type-required |
| `add_decimal_digits` | `String::from_utf8` | [1300](../../src/client_extensions.rs#L1300) | external-constructor-callback-or-unresolved |
| `add_decimal_digits` | `result.trim_start_matches` | [1307](../../src/client_extensions.rs#L1307) | receiver-type-required |
| `add_decimal_digits` | `Ok` | [1308](../../src/client_extensions.rs#L1308) | external-constructor-callback-or-unresolved |
| `add_decimal_digits` | `result.is_empty` | [1308](../../src/client_extensions.rs#L1308) | receiver-type-required |
| `add_decimal_digits` | `"0".to_owned` | [1309](../../src/client_extensions.rs#L1309) | receiver-type-required |
| `add_decimal_digits` | `result.to_owned` | [1311](../../src/client_extensions.rs#L1311) | receiver-type-required |
| `canonical_cost` | `value.split_once` | [1316](../../src/client_extensions.rs#L1316) | receiver-type-required |
| `canonical_cost` | `Some` | [1317](../../src/client_extensions.rs#L1317) | external-constructor-callback-or-unresolved |
| `canonical_cost` | `canonical_unsigned` | [1320](../../src/client_extensions.rs#L1320) | [tekes-supervisor::client_extensions::canonical_unsigned](../../src/client_extensions.rs#L1256) |
| `canonical_cost` | `fraction.is_empty` | [1322](../../src/client_extensions.rs#L1322) | receiver-type-required |
| `canonical_cost` | `fraction.bytes().all` | [1323](../../src/client_extensions.rs#L1323) | receiver-type-required |
| `canonical_cost` | `fraction.bytes` | [1323](../../src/client_extensions.rs#L1323) | receiver-type-required |
| `canonical_cost` | `byte.is_ascii_digit` | [1323](../../src/client_extensions.rs#L1323) | receiver-type-required |
| `canonical_cost` | `fraction.ends_with` | [1324](../../src/client_extensions.rs#L1324) | receiver-type-required |
| `canonical_cost` | `Err` | [1326](../../src/client_extensions.rs#L1326), [1333](../../src/client_extensions.rs#L1333) | external-constructor-callback-or-unresolved |
| `canonical_cost` | `failure` | [1326](../../src/client_extensions.rs#L1326), [1333](../../src/client_extensions.rs#L1333) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `canonical_cost` | `fraction.is_some_and` | [1332](../../src/client_extensions.rs#L1332) | receiver-type-required |
| `canonical_cost` | `digits.bytes().all` | [1332](../../src/client_extensions.rs#L1332) | receiver-type-required |
| `canonical_cost` | `digits.bytes` | [1332](../../src/client_extensions.rs#L1332) | receiver-type-required |
| `canonical_cost` | `Ok` | [1339](../../src/client_extensions.rs#L1339) | external-constructor-callback-or-unresolved |
| `canonical_cost` | `value.to_owned` | [1339](../../src/client_extensions.rs#L1339) | receiver-type-required |
| `add_canonical_cost` | `canonical_cost` | [1343](../../src/client_extensions.rs#L1343), [1344](../../src/client_extensions.rs#L1344) | [tekes-supervisor::client_extensions::canonical_cost](../../src/client_extensions.rs#L1315) |
| `add_canonical_cost` | `left.split_once('.').map_or` | [1345](../../src/client_extensions.rs#L1345) | receiver-type-required |
| `add_canonical_cost` | `left.split_once` | [1345](../../src/client_extensions.rs#L1345) | receiver-type-required |
| `add_canonical_cost` | `digits.len` | [1345](../../src/client_extensions.rs#L1345), [1346](../../src/client_extensions.rs#L1346) | receiver-type-required |
| `add_canonical_cost` | `right.split_once('.').map_or` | [1346](../../src/client_extensions.rs#L1346) | receiver-type-required |
| `add_canonical_cost` | `right.split_once` | [1346](../../src/client_extensions.rs#L1346) | receiver-type-required |
| `add_canonical_cost` | `left_scale.max` | [1347](../../src/client_extensions.rs#L1347) | receiver-type-required |
| `add_canonical_cost` | `left.replace` | [1348](../../src/client_extensions.rs#L1348) | receiver-type-required |
| `add_canonical_cost` | `"0".repeat` | [1348](../../src/client_extensions.rs#L1348), [1349](../../src/client_extensions.rs#L1349) | receiver-type-required |
| `add_canonical_cost` | `right.replace` | [1349](../../src/client_extensions.rs#L1349) | receiver-type-required |
| `add_canonical_cost` | `add_decimal_digits` | [1350](../../src/client_extensions.rs#L1350) | [tekes-supervisor::client_extensions::add_decimal_digits](../../src/client_extensions.rs#L1275) |
| `add_canonical_cost` | `Ok` | [1352](../../src/client_extensions.rs#L1352), [1365](../../src/client_extensions.rs#L1365) | external-constructor-callback-or-unresolved |
| `add_canonical_cost` | `sum.len` | [1354](../../src/client_extensions.rs#L1354), [1357](../../src/client_extensions.rs#L1357) | receiver-type-required |
| `add_canonical_cost` | `sum.insert` | [1358](../../src/client_extensions.rs#L1358) | receiver-type-required |
| `add_canonical_cost` | `sum.ends_with` | [1359](../../src/client_extensions.rs#L1359), [1362](../../src/client_extensions.rs#L1362) | receiver-type-required |
| `add_canonical_cost` | `sum.pop` | [1360](../../src/client_extensions.rs#L1360), [1363](../../src/client_extensions.rs#L1363) | receiver-type-required |
| `usage_route_target` | `epoch         .pointer("/system/asset")         .and_then(Value::as_str)         .ok_or_else` | [1372](../../src/client_extensions.rs#L1372) | receiver-type-required |
| `usage_route_target` | `epoch         .pointer("/system/asset")         .and_then` | [1372](../../src/client_extensions.rs#L1372) | receiver-type-required |
| `usage_route_target` | `epoch         .pointer` | [1372](../../src/client_extensions.rs#L1372), [1382](../../src/client_extensions.rs#L1382) | receiver-type-required |
| `usage_route_target` | `failure` | [1376](../../src/client_extensions.rs#L1376), [1386](../../src/client_extensions.rs#L1386), [1393](../../src/client_extensions.rs#L1393), [1400](../../src/client_extensions.rs#L1400), [1407](../../src/client_extensions.rs#L1407), [1414](../../src/client_extensions.rs#L1414), [1421](../../src/client_extensions.rs#L1421), [1432](../../src/client_extensions.rs#L1432), [1442](../../src/client_extensions.rs#L1442), [1455](../../src/client_extensions.rs#L1455), [1472](../../src/client_extensions.rs#L1472) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `usage_route_target` | `epoch         .pointer("/system/digest")         .and_then(Value::as_str)         .ok_or_else` | [1382](../../src/client_extensions.rs#L1382) | receiver-type-required |
| `usage_route_target` | `epoch         .pointer("/system/digest")         .and_then` | [1382](../../src/client_extensions.rs#L1382) | receiver-type-required |
| `usage_route_target` | `Err` | [1393](../../src/client_extensions.rs#L1393), [1421](../../src/client_extensions.rs#L1421), [1472](../../src/client_extensions.rs#L1472) | external-constructor-callback-or-unresolved |
| `usage_route_target` | `assets.read_verified(asset).map_err` | [1399](../../src/client_extensions.rs#L1399) | receiver-type-required |
| `usage_route_target` | `assets.read_verified` | [1399](../../src/client_extensions.rs#L1399) | receiver-type-required |
| `usage_route_target` | `IJsonValue::parse(&bytes).map_err` | [1406](../../src/client_extensions.rs#L1406) | receiver-type-required |
| `usage_route_target` | `IJsonValue::parse` | [1406](../../src/client_extensions.rs#L1406) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `usage_route_target` | `profile.canonical_bytes().map_err` | [1413](../../src/client_extensions.rs#L1413) | receiver-type-required |
| `usage_route_target` | `profile.canonical_bytes` | [1413](../../src/client_extensions.rs#L1413) | receiver-type-required |
| `usage_route_target` | `ijson_value` | [1427](../../src/client_extensions.rs#L1427) | [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) |
| `usage_route_target` | `profile         .get("target")         .and_then(Value::as_object)         .ok_or_else` | [1428](../../src/client_extensions.rs#L1428) | receiver-type-required |
| `usage_route_target` | `profile         .get("target")         .and_then` | [1428](../../src/client_extensions.rs#L1428) | receiver-type-required |
| `usage_route_target` | `profile         .get` | [1428](../../src/client_extensions.rs#L1428) | receiver-type-required |
| `usage_route_target` | `target         .get("route")         .and_then(Value::as_object)         .ok_or_else` | [1438](../../src/client_extensions.rs#L1438) | receiver-type-required |
| `usage_route_target` | `target         .get("route")         .and_then` | [1438](../../src/client_extensions.rs#L1438) | receiver-type-required |
| `usage_route_target` | `target         .get` | [1438](../../src/client_extensions.rs#L1438) | receiver-type-required |
| `usage_route_target` | `object             .get(name)             .and_then(Value::as_str)             .filter(&#124;value&#124; !value.is_empty())             .map(str::to_owned)             .ok_or_else` | [1449](../../src/client_extensions.rs#L1449) | receiver-type-required |
| `usage_route_target` | `object             .get(name)             .and_then(Value::as_str)             .filter(&#124;value&#124; !value.is_empty())             .map` | [1449](../../src/client_extensions.rs#L1449) | receiver-type-required |
| `usage_route_target` | `object             .get(name)             .and_then(Value::as_str)             .filter` | [1449](../../src/client_extensions.rs#L1449) | receiver-type-required |
| `usage_route_target` | `object             .get(name)             .and_then` | [1449](../../src/client_extensions.rs#L1449) | receiver-type-required |
| `usage_route_target` | `object             .get` | [1449](../../src/client_extensions.rs#L1449) | receiver-type-required |
| `usage_route_target` | `value.is_empty` | [1452](../../src/client_extensions.rs#L1452) | receiver-type-required |
| `usage_route_target` | `field` | [1462](../../src/client_extensions.rs#L1462), [1463](../../src/client_extensions.rs#L1463), [1464](../../src/client_extensions.rs#L1464), [1465](../../src/client_extensions.rs#L1465), [1466](../../src/client_extensions.rs#L1466), [1467](../../src/client_extensions.rs#L1467), [1468](../../src/client_extensions.rs#L1468) | external-constructor-callback-or-unresolved |
| `usage_route_target` | `epoch.get("adapter").and_then` | [1469](../../src/client_extensions.rs#L1469) | receiver-type-required |
| `usage_route_target` | `epoch.get` | [1469](../../src/client_extensions.rs#L1469), [1470](../../src/client_extensions.rs#L1470) | receiver-type-required |
| `usage_route_target` | `Some` | [1469](../../src/client_extensions.rs#L1469), [1470](../../src/client_extensions.rs#L1470) | external-constructor-callback-or-unresolved |
| `usage_route_target` | `dialect_id.as_str` | [1469](../../src/client_extensions.rs#L1469) | receiver-type-required |
| `usage_route_target` | `epoch.get("model").and_then` | [1470](../../src/client_extensions.rs#L1470) | receiver-type-required |
| `usage_route_target` | `exact_sku.as_str` | [1470](../../src/client_extensions.rs#L1470) | receiver-type-required |
| `usage_route_target` | `Ok` | [1478](../../src/client_extensions.rs#L1478) | external-constructor-callback-or-unresolved |
| `map_mcp_resource_error` | `failure` | [1501](../../src/client_extensions.rs#L1501) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `map_plugin_error` | `failure` | [1521](../../src/client_extensions.rs#L1521) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `plugin_mutation_result` | `Ok` | [1533](../../src/client_extensions.rs#L1533) | external-constructor-callback-or-unresolved |
| `plugin_mutation_result` | `plugin_error_is_precommit` | [1534](../../src/client_extensions.rs#L1534) | [tekes-supervisor::client_extensions::plugin_error_is_precommit](../../src/client_extensions.rs#L1542) |
| `plugin_mutation_result` | `map_plugin_error` | [1535](../../src/client_extensions.rs#L1535), [1538](../../src/client_extensions.rs#L1538) | [tekes-supervisor::client_extensions::map_plugin_error](../../src/client_extensions.rs#L1507) |
| `plugin_mutation_result` | `Err` | [1536](../../src/client_extensions.rs#L1536), [1538](../../src/client_extensions.rs#L1538) | external-constructor-callback-or-unresolved |
| `plugin_mutation_result` | `operation.fail` | [1536](../../src/client_extensions.rs#L1536) | receiver-type-required |
| `map_schedule_error` | `failure` | [1570](../../src/client_extensions.rs#L1570) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `map_search_error` | `failure` | [1589](../../src/client_extensions.rs#L1589) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `picker_home` | `std::env::var_os("HOME")             .map(PathBuf::from)             .filter(&#124;home&#124; home.is_absolute())             .unwrap_or_else` | [1599](../../src/client_extensions.rs#L1599) | receiver-type-required |
| `picker_home` | `std::env::var_os("HOME")             .map(PathBuf::from)             .filter` | [1599](../../src/client_extensions.rs#L1599) | receiver-type-required |
| `picker_home` | `std::env::var_os("HOME")             .map` | [1599](../../src/client_extensions.rs#L1599) | receiver-type-required |
| `picker_home` | `std::env::var_os` | [1599](../../src/client_extensions.rs#L1599) | external-constructor-callback-or-unresolved |
| `picker_home` | `home.is_absolute` | [1601](../../src/client_extensions.rs#L1601) | receiver-type-required |
| `picker_home` | `self.root.clone` | [1602](../../src/client_extensions.rs#L1602) | receiver-type-required |
| `session_folder` | `endpoint::validate_session_id(session_id)             .map_err` | [1607](../../src/client_extensions.rs#L1607) | receiver-type-required |
| `session_folder` | `endpoint::validate_session_id` | [1607](../../src/client_extensions.rs#L1607) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `session_folder` | `bad_request` | [1608](../../src/client_extensions.rs#L1608) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `session_folder` | `[("threads", false), ("archive", true)]             .into_iter()             .map(&#124;(area, archived)&#124; (self.root.join(area).join(session_id), archived))             .find(&#124;(folder, _)&#124; folder.is_dir())             .ok_or_else` | [1609](../../src/client_extensions.rs#L1609) | receiver-type-required |
| `session_folder` | `[("threads", false), ("archive", true)]             .into_iter()             .map(&#124;(area, archived)&#124; (self.root.join(area).join(session_id), archived))             .find` | [1609](../../src/client_extensions.rs#L1609) | receiver-type-required |
| `session_folder` | `[("threads", false), ("archive", true)]             .into_iter()             .map` | [1609](../../src/client_extensions.rs#L1609) | receiver-type-required |
| `session_folder` | `[("threads", false), ("archive", true)]             .into_iter` | [1609](../../src/client_extensions.rs#L1609) | receiver-type-required |
| `session_folder` | `self.root.join(area).join` | [1611](../../src/client_extensions.rs#L1611) | receiver-type-required |
| `session_folder` | `self.root.join` | [1611](../../src/client_extensions.rs#L1611) | receiver-type-required |
| `session_folder` | `folder.is_dir` | [1612](../../src/client_extensions.rs#L1612) | receiver-type-required |
| `session_folder` | `failure` | [1614](../../src/client_extensions.rs#L1614) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `session_controls` | `failure` | [1628](../../src/client_extensions.rs#L1628), [1634](../../src/client_extensions.rs#L1634) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `session_controls` | `error.code` | [1628](../../src/client_extensions.rs#L1628) | receiver-type-required |
| `session_controls` | `error.to_string` | [1628](../../src/client_extensions.rs#L1628) | receiver-type-required |
| `session_controls` | `error.details` | [1628](../../src/client_extensions.rs#L1628) | receiver-type-required |
| `session_controls` | `payload.get(key).and_then(Value::as_str).unwrap_or_default` | [1630](../../src/client_extensions.rs#L1630) | receiver-type-required |
| `session_controls` | `payload.get(key).and_then` | [1630](../../src/client_extensions.rs#L1630) | receiver-type-required |
| `session_controls` | `payload.get` | [1630](../../src/client_extensions.rs#L1630) | receiver-type-required |
| `session_controls` | `self.session_folder` | [1632](../../src/client_extensions.rs#L1632), [1643](../../src/client_extensions.rs#L1643) | [tekes-supervisor::client_extensions::ProductionClientExtensions::session_folder](../../src/client_extensions.rs#L1606) |
| `session_controls` | `text` | [1632](../../src/client_extensions.rs#L1632), [1640](../../src/client_extensions.rs#L1640), [1643](../../src/client_extensions.rs#L1643) | external-constructor-callback-or-unresolved |
| `session_controls` | `std::fs::read(folder.join("main.jsonl")).map_err` | [1633](../../src/client_extensions.rs#L1633) | receiver-type-required |
| `session_controls` | `std::fs::read` | [1633](../../src/client_extensions.rs#L1633) | external-constructor-callback-or-unresolved |
| `session_controls` | `folder.join` | [1633](../../src/client_extensions.rs#L1633) | receiver-type-required |
| `session_controls` | `session_controls::subagent_catalog(&bytes, text("parentSessionId"), archived)                 .map_err` | [1640](../../src/client_extensions.rs#L1640) | receiver-type-required |
| `session_controls` | `session_controls::subagent_catalog` | [1640](../../src/client_extensions.rs#L1640) | [session-controls::subagent_catalog](../../../session-controls/src/lib.rs#L516) |
| `session_controls` | `session_controls::execute_goal(&self.root, operation, payload, None)                 .map_err` | [1645](../../src/client_extensions.rs#L1645) | receiver-type-required |
| `session_controls` | `session_controls::execute_goal` | [1645](../../src/client_extensions.rs#L1645) | [session-controls::execute_goal](../../../session-controls/src/lib.rs#L245) |
| `session_controls` | `encode` | [1653](../../src/client_extensions.rs#L1653) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `host_files` | `failure` | [1662](../../src/client_extensions.rs#L1662), [1680](../../src/client_extensions.rs#L1680), [1692](../../src/client_extensions.rs#L1692), [1710](../../src/client_extensions.rs#L1710) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `host_files` | `error.code` | [1662](../../src/client_extensions.rs#L1662) | receiver-type-required |
| `host_files` | `error.message` | [1662](../../src/client_extensions.rs#L1662) | receiver-type-required |
| `host_files` | `payload.get(key).and_then(Value::as_str).unwrap_or_default` | [1664](../../src/client_extensions.rs#L1664) | receiver-type-required |
| `host_files` | `payload.get(key).and_then` | [1664](../../src/client_extensions.rs#L1664) | receiver-type-required |
| `host_files` | `payload.get` | [1664](../../src/client_extensions.rs#L1664), [1668](../../src/client_extensions.rs#L1668) | receiver-type-required |
| `host_files` | `host_files::list_directory(                 &self.picker_home(),                 payload.get("path").and_then(Value::as_str),             )             .map_err` | [1666](../../src/client_extensions.rs#L1666) | receiver-type-required |
| `host_files` | `host_files::list_directory` | [1666](../../src/client_extensions.rs#L1666) | [host-files::list_directory](../../../host-files/src/lib.rs#L228) |
| `host_files` | `self.picker_home` | [1667](../../src/client_extensions.rs#L1667) | [tekes-supervisor::client_extensions::ProductionClientExtensions::picker_home](../../src/client_extensions.rs#L1598) |
| `host_files` | `payload.get("path").and_then` | [1668](../../src/client_extensions.rs#L1668) | receiver-type-required |
| `host_files` | `host_files::create_directory(text("path"), text("name")).map_err` | [1672](../../src/client_extensions.rs#L1672) | receiver-type-required |
| `host_files` | `host_files::create_directory` | [1672](../../src/client_extensions.rs#L1672) | [host-files::create_directory](../../../host-files/src/lib.rs#L276) |
| `host_files` | `text` | [1672](../../src/client_extensions.rs#L1672), [1678](../../src/client_extensions.rs#L1678), [1686](../../src/client_extensions.rs#L1686), [1690](../../src/client_extensions.rs#L1690), [1732](../../src/client_extensions.rs#L1732) | external-constructor-callback-or-unresolved |
| `host_files` | `self                     .process                     .client_session_roots(text("sessionId"))                     .map_err` | [1676](../../src/client_extensions.rs#L1676) | receiver-type-required |
| `host_files` | `self                     .process                     .client_session_roots` | [1676](../../src/client_extensions.rs#L1676) | receiver-type-required |
| `host_files` | `host_files::file_references(&roots, text("query")).map_err` | [1686](../../src/client_extensions.rs#L1686) | receiver-type-required |
| `host_files` | `host_files::file_references` | [1686](../../src/client_extensions.rs#L1686) | [host-files::file_references](../../../host-files/src/lib.rs#L321) |
| `host_files` | `endpoint::NativeEndpoint::open(&self.root)                     .map_err` | [1698](../../src/client_extensions.rs#L1698) | receiver-type-required |
| `host_files` | `endpoint::NativeEndpoint::open` | [1698](../../src/client_extensions.rs#L1698) | [endpoint::service::NativeEndpoint::open](../../../endpoint/src/service.rs#L263) |
| `host_files` | `internal` | [1699](../../src/client_extensions.rs#L1699), [1702](../../src/client_extensions.rs#L1702), [1704](../../src/client_extensions.rs#L1704) | external-constructor-callback-or-unresolved |
| `host_files` | `error.to_string` | [1699](../../src/client_extensions.rs#L1699), [1702](../../src/client_extensions.rs#L1702), [1704](../../src/client_extensions.rs#L1704) | receiver-type-required |
| `host_files` | `native                     .list_sessions_cached(&HashSet::new())                     .map_err` | [1700](../../src/client_extensions.rs#L1700) | receiver-type-required |
| `host_files` | `native                     .list_sessions_cached` | [1700](../../src/client_extensions.rs#L1700) | receiver-type-required |
| `host_files` | `HashSet::new` | [1701](../../src/client_extensions.rs#L1701) | external-constructor-callback-or-unresolved |
| `host_files` | `endpoint::ManagementStore::open_light(&self.root)                     .map_err` | [1703](../../src/client_extensions.rs#L1703) | receiver-type-required |
| `host_files` | `endpoint::ManagementStore::open_light` | [1703](../../src/client_extensions.rs#L1703) | [endpoint::management::ManagementStore::open_light](../../../endpoint/src/management.rs#L500) |
| `host_files` | `items                     .iter()                     .find(&#124;item&#124; item.session_id == requesting)                     .map(&#124;item&#124; item.workspace_id.clone())                     .ok_or_else` | [1705](../../src/client_extensions.rs#L1705) | receiver-type-required |
| `host_files` | `items                     .iter()                     .find(&#124;item&#124; item.session_id == requesting)                     .map` | [1705](../../src/client_extensions.rs#L1705) | receiver-type-required |
| `host_files` | `items                     .iter()                     .find` | [1705](../../src/client_extensions.rs#L1705) | receiver-type-required |
| `host_files` | `items                     .iter` | [1705](../../src/client_extensions.rs#L1705), [1717](../../src/client_extensions.rs#L1717) | receiver-type-required |
| `host_files` | `item.workspace_id.clone` | [1708](../../src/client_extensions.rs#L1708), [1723](../../src/client_extensions.rs#L1723) | receiver-type-required |
| `host_files` | `management.workspace_paths().unwrap_or_default` | [1716](../../src/client_extensions.rs#L1716) | receiver-type-required |
| `host_files` | `management.workspace_paths` | [1716](../../src/client_extensions.rs#L1716) | receiver-type-required |
| `host_files` | `items                     .iter()                     .map(&#124;item&#124; host_files::SessionCandidate {                         session_id: item.session_id.clone(),                         title: item.title.clone(),                         cwd: paths.get(&item.workspace_id).cloned().unwrap_or_default(),                         workspace_id: item.workspace_id.clone(),                         created_at_ms: item.updated_at as f64,                         archived: item.archived,                     })                     .collect::<Vec<_>>` | [1717](../../src/client_extensions.rs#L1717) | receiver-type-required |
| `host_files` | `items                     .iter()                     .map` | [1717](../../src/client_extensions.rs#L1717) | receiver-type-required |
| `host_files` | `item.session_id.clone` | [1720](../../src/client_extensions.rs#L1720) | receiver-type-required |
| `host_files` | `item.title.clone` | [1721](../../src/client_extensions.rs#L1721) | receiver-type-required |
| `host_files` | `paths.get(&item.workspace_id).cloned().unwrap_or_default` | [1722](../../src/client_extensions.rs#L1722) | receiver-type-required |
| `host_files` | `paths.get(&item.workspace_id).cloned` | [1722](../../src/client_extensions.rs#L1722) | receiver-type-required |
| `host_files` | `paths.get` | [1722](../../src/client_extensions.rs#L1722) | receiver-type-required |
| `host_files` | `host_files::session_references` | [1728](../../src/client_extensions.rs#L1728) | [host-files::session_references](../../../host-files/src/lib.rs#L412) |
| `host_files` | `Err` | [1736](../../src/client_extensions.rs#L1736) | external-constructor-callback-or-unresolved |
| `host_files` | `bad_request` | [1736](../../src/client_extensions.rs#L1736) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `host_files` | `encode` | [1738](../../src/client_extensions.rs#L1738) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `execute` | `encode` | [1752](../../src/client_extensions.rs#L1752), [1826](../../src/client_extensions.rs#L1826), [1828](../../src/client_extensions.rs#L1828), [1865](../../src/client_extensions.rs#L1865), [1867](../../src/client_extensions.rs#L1867), [1879](../../src/client_extensions.rs#L1879), [1887](../../src/client_extensions.rs#L1887), [1889](../../src/client_extensions.rs#L1889), [1897](../../src/client_extensions.rs#L1897), [1924](../../src/client_extensions.rs#L1924), [1936](../../src/client_extensions.rs#L1936), [1949](../../src/client_extensions.rs#L1949), [1964](../../src/client_extensions.rs#L1964), [1970](../../src/client_extensions.rs#L1970), [1977](../../src/client_extensions.rs#L1977), [1979](../../src/client_extensions.rs#L1979), [1997](../../src/client_extensions.rs#L1997), [2023](../../src/client_extensions.rs#L2023), [2043](../../src/client_extensions.rs#L2043), [2052](../../src/client_extensions.rs#L2052), [2186](../../src/client_extensions.rs#L2186), [2195](../../src/client_extensions.rs#L2195) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `execute` | `tools::describe_presets` | [1752](../../src/client_extensions.rs#L1752) | [tools::preset::describe_value](../../../tools/src/preset.rs#L991) |
| `execute` | `payload["workspaceId"].as_str().unwrap_or_default` | [1754](../../src/client_extensions.rs#L1754) | receiver-type-required |
| `execute` | `payload["workspaceId"].as_str` | [1754](../../src/client_extensions.rs#L1754) | receiver-type-required |
| `execute` | `self.workspace_snapshot` | [1755](../../src/client_extensions.rs#L1755) | receiver-type-required |
| `execute` | `profile::InstructionResolver::new_scoped(                     self.process.user_agent_dir(),                     self.process                         .user_agent_dir()                         .join("workspaces")                         .join(workspace_id),                     snapshot.workspace.cwd.iter().map(Path::new),                 )                 .capture()                 .map_err` | [1756](../../src/client_extensions.rs#L1756) | receiver-type-required |
| `execute` | `profile::InstructionResolver::new_scoped(                     self.process.user_agent_dir(),                     self.process                         .user_agent_dir()                         .join("workspaces")                         .join(workspace_id),                     snapshot.workspace.cwd.iter().map(Path::new),                 )                 .capture` | [1756](../../src/client_extensions.rs#L1756) | receiver-type-required |
| `execute` | `profile::InstructionResolver::new_scoped` | [1756](../../src/client_extensions.rs#L1756) | [profile::instruction::InstructionResolver::new_scoped](../../../profile/src/instruction.rs#L300) |
| `execute` | `self.process.user_agent_dir` | [1757](../../src/client_extensions.rs#L1757) | receiver-type-required |
| `execute` | `self.process                         .user_agent_dir()                         .join("workspaces")                         .join` | [1758](../../src/client_extensions.rs#L1758) | receiver-type-required |
| `execute` | `self.process                         .user_agent_dir()                         .join` | [1758](../../src/client_extensions.rs#L1758) | receiver-type-required |
| `execute` | `self.process                         .user_agent_dir` | [1758](../../src/client_extensions.rs#L1758) | receiver-type-required |
| `execute` | `snapshot.workspace.cwd.iter().map` | [1762](../../src/client_extensions.rs#L1762) | receiver-type-required |
| `execute` | `snapshot.workspace.cwd.iter` | [1762](../../src/client_extensions.rs#L1762) | receiver-type-required |
| `execute` | `failure` | [1766](../../src/client_extensions.rs#L1766), [1783](../../src/client_extensions.rs#L1783), [1789](../../src/client_extensions.rs#L1789), [1802](../../src/client_extensions.rs#L1802), [1854](../../src/client_extensions.rs#L1854), [1905](../../src/client_extensions.rs#L1905), [1912](../../src/client_extensions.rs#L1912), [1935](../../src/client_extensions.rs#L1935), [1943](../../src/client_extensions.rs#L1943), [1957](../../src/client_extensions.rs#L1957), [1991](../../src/client_extensions.rs#L1991), [2037](../../src/client_extensions.rs#L2037), [2073](../../src/client_extensions.rs#L2073), [2084](../../src/client_extensions.rs#L2084), [2102](../../src/client_extensions.rs#L2102), [2120](../../src/client_extensions.rs#L2120), [2145](../../src/client_extensions.rs#L2145), [2166](../../src/client_extensions.rs#L2166), [2204](../../src/client_extensions.rs#L2204) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `execute` | `endpoint::SessionPresetEnvironment::new` | [1772](../../src/client_extensions.rs#L1772) | [endpoint::management::SessionPresetEnvironment::new](../../../endpoint/src/management.rs#L260) |
| `execute` | `Some` | [1775](../../src/client_extensions.rs#L1775), [2015](../../src/client_extensions.rs#L2015), [2140](../../src/client_extensions.rs#L2140) | external-constructor-callback-or-unresolved |
| `execute` | `self.mcp.as_ref` | [1775](../../src/client_extensions.rs#L1775) | receiver-type-required |
| `execute` | `profile::ConfigRepository::open(&self.root)                     .and_then(&#124;repository&#124; repository.workspace(workspace_id))                     .map_err(&#124;error&#124; match error {                         profile::ProfileError::Io(ref io)                             if io.kind() == std::io::ErrorKind::NotFound =>                         {                             failure(                                 "workspace-not-found",                                 "Workspace was not found",                                 serde_json::json!({"workspaceId": workspace_id}),                             )                         }                         _ => failure(                             "internal",                             "Endpoint operation failed",                             serde_json::json!({}),                         ),                     })?                     .policy                     .and_then(&#124;policy&#124; policy.base)                     .unwrap_or_default` | [1777](../../src/client_extensions.rs#L1777) | receiver-type-required |
| `execute` | `profile::ConfigRepository::open(&self.root)                     .and_then(&#124;repository&#124; repository.workspace(workspace_id))                     .map_err(&#124;error&#124; match error {                         profile::ProfileError::Io(ref io)                             if io.kind() == std::io::ErrorKind::NotFound =>                         {                             failure(                                 "workspace-not-found",                                 "Workspace was not found",                                 serde_json::json!({"workspaceId": workspace_id}),                             )                         }                         _ => failure(                             "internal",                             "Endpoint operation failed",                             serde_json::json!({}),                         ),                     })?                     .policy                     .and_then` | [1777](../../src/client_extensions.rs#L1777) | receiver-type-required |
| `execute` | `profile::ConfigRepository::open(&self.root)                     .and_then(&#124;repository&#124; repository.workspace(workspace_id))                     .map_err` | [1777](../../src/client_extensions.rs#L1777) | receiver-type-required |
| `execute` | `profile::ConfigRepository::open(&self.root)                     .and_then` | [1777](../../src/client_extensions.rs#L1777) | receiver-type-required |
| `execute` | `profile::ConfigRepository::open` | [1777](../../src/client_extensions.rs#L1777) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `execute` | `repository.workspace` | [1778](../../src/client_extensions.rs#L1778) | receiver-type-required |
| `execute` | `io.kind` | [1781](../../src/client_extensions.rs#L1781) | receiver-type-required |
| `execute` | `tools::ResolvedPreset::resolve_request(&payload["preset"], base, &environment)                         .map_err` | [1799](../../src/client_extensions.rs#L1799) | receiver-type-required |
| `execute` | `tools::ResolvedPreset::resolve_request` | [1799](../../src/client_extensions.rs#L1799) | [tools::preset::ResolvedPreset::resolve_request](../../../tools/src/preset.rs#L655) |
| `execute` | `bad_request` | [1801](../../src/client_extensions.rs#L1801), [1902](../../src/client_extensions.rs#L1902) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `execute` | `invalid.detail` | [1805](../../src/client_extensions.rs#L1805) | receiver-type-required |
| `execute` | `preset.mcp.is_empty` | [1817](../../src/client_extensions.rs#L1817), [1823](../../src/client_extensions.rs#L1823) | receiver-type-required |
| `execute` | `self.mcp.preset_readiness` | [1824](../../src/client_extensions.rs#L1824) | receiver-type-required |
| `execute` | `decode` | [1836](../../src/client_extensions.rs#L1836), [1869](../../src/client_extensions.rs#L1869), [1882](../../src/client_extensions.rs#L1882), [1891](../../src/client_extensions.rs#L1891), [1900](../../src/client_extensions.rs#L1900), [1939](../../src/client_extensions.rs#L1939), [1983](../../src/client_extensions.rs#L1983), [2007](../../src/client_extensions.rs#L2007), [2011](../../src/client_extensions.rs#L2011), [2019](../../src/client_extensions.rs#L2019), [2028](../../src/client_extensions.rs#L2028), [2046](../../src/client_extensions.rs#L2046), [2059](../../src/client_extensions.rs#L2059), [2092](../../src/client_extensions.rs#L2092), [2110](../../src/client_extensions.rs#L2110), [2128](../../src/client_extensions.rs#L2128), [2153](../../src/client_extensions.rs#L2153), [2194](../../src/client_extensions.rs#L2194) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `execute` | `self                     .process                     .client_file_page(                         &request.session_id,                         &request.path,                         text,                         request.offset.unwrap_or(if text { 1 } else { 0 }),                         request.limit.unwrap_or(if text {                             200                         } else if operation == "session.files.stat" {                             1                         } else {                             256 * 1024                         }),                     )                     .map_err` | [1838](../../src/client_extensions.rs#L1838) | receiver-type-required |
| `execute` | `self                     .process                     .client_file_page` | [1838](../../src/client_extensions.rs#L1838) | receiver-type-required |
| `execute` | `request.offset.unwrap_or` | [1844](../../src/client_extensions.rs#L1844) | receiver-type-required |
| `execute` | `request.limit.unwrap_or` | [1845](../../src/client_extensions.rs#L1845) | receiver-type-required |
| `execute` | `page.as_object_mut().unwrap().retain` | [1861](../../src/client_extensions.rs#L1861) | receiver-type-required |
| `execute` | `page.as_object_mut().unwrap` | [1861](../../src/client_extensions.rs#L1861) | receiver-type-required |
| `execute` | `page.as_object_mut` | [1861](../../src/client_extensions.rs#L1861) | receiver-type-required |
| `execute` | `["absolutePath", "version", "bytes"].contains` | [1862](../../src/client_extensions.rs#L1862) | receiver-type-required |
| `execute` | `key.as_str` | [1862](../../src/client_extensions.rs#L1862) | receiver-type-required |
| `execute` | `endpoint::AttachmentPolicy::default` | [1867](../../src/client_extensions.rs#L1867) | external-constructor-callback-or-unresolved |
| `execute` | `self                     .attachments()                     .upload_file(                         &request.session_id,                         &request.name,                         request.media_type.as_deref(),                         &request.data,                     )                     .map_err` | [1870](../../src/client_extensions.rs#L1870) | receiver-type-required |
| `execute` | `self                     .attachments()                     .upload_file` | [1870](../../src/client_extensions.rs#L1870) | receiver-type-required |
| `execute` | `self                     .attachments` | [1870](../../src/client_extensions.rs#L1870), [1883](../../src/client_extensions.rs#L1883) | receiver-type-required |
| `execute` | `request.media_type.as_deref` | [1875](../../src/client_extensions.rs#L1875) | receiver-type-required |
| `execute` | `map_prompt_materialize_error` | [1878](../../src/client_extensions.rs#L1878) | [tekes-supervisor::resource_capability::map_prompt_materialize_error](../../src/resource_capability.rs#L465) |
| `execute` | `self                     .attachments()                     .read_authorized_file(&request.session_id, &request.attachment_id)                     .map_err` | [1883](../../src/client_extensions.rs#L1883) | receiver-type-required |
| `execute` | `self                     .attachments()                     .read_authorized_file` | [1883](../../src/client_extensions.rs#L1883) | receiver-type-required |
| `execute` | `map_attachment_read_error` | [1886](../../src/client_extensions.rs#L1886) | [tekes-supervisor::client_extensions::map_attachment_read_error](../../src/client_extensions.rs#L2801) |
| `execute` | `endpoint::ApprovalPolicy::default` | [1889](../../src/client_extensions.rs#L1889) | external-constructor-callback-or-unresolved |
| `execute` | `self.session_folder` | [1892](../../src/client_extensions.rs#L1892), [1903](../../src/client_extensions.rs#L1903) | receiver-type-required |
| `execute` | `engine::read_permission_mode` | [1893](../../src/client_extensions.rs#L1893) | [engine::permission_mode::read_permission_mode](../../../engine/src/permission_mode.rs#L85) |
| `execute` | `engine::PermissionMode::parse(&request.mode)                     .ok_or_else` | [1901](../../src/client_extensions.rs#L1901) | receiver-type-required |
| `execute` | `engine::PermissionMode::parse` | [1901](../../src/client_extensions.rs#L1901) | [engine::permission_mode::PermissionMode::parse](../../../engine/src/permission_mode.rs#L51) |
| `execute` | `Err` | [1905](../../src/client_extensions.rs#L1905), [2073](../../src/client_extensions.rs#L2073), [2204](../../src/client_extensions.rs#L2204) | external-constructor-callback-or-unresolved |
| `execute` | `engine::write_permission_mode(&folder, mode).map_err` | [1911](../../src/client_extensions.rs#L1911) | receiver-type-required |
| `execute` | `engine::write_permission_mode` | [1911](../../src/client_extensions.rs#L1911) | [engine::permission_mode::write_permission_mode](../../../engine/src/permission_mode.rs#L117) |
| `execute` | `crate::endpoint_host::SessionDeliveryAuthority::session_metadata_changed` | [1920](../../src/client_extensions.rs#L1920) | external-constructor-callback-or-unresolved |
| `execute` | `self.process.as_ref` | [1921](../../src/client_extensions.rs#L1921) | receiver-type-required |
| `execute` | `self.host_files` | [1929](../../src/client_extensions.rs#L1929) | receiver-type-required |
| `execute` | `session_controls::METHODS.contains` | [1930](../../src/client_extensions.rs#L1930) | receiver-type-required |
| `execute` | `self.session_controls` | [1931](../../src/client_extensions.rs#L1931) | receiver-type-required |
| `execute` | `user_documents::METHODS.contains` | [1933](../../src/client_extensions.rs#L1933) | receiver-type-required |
| `execute` | `user_documents::execute(&self.root, operation, payload)                     .map_err` | [1934](../../src/client_extensions.rs#L1934) | receiver-type-required |
| `execute` | `user_documents::execute` | [1934](../../src/client_extensions.rs#L1934) | [user-documents::execute](../../../user-documents/src/lib.rs#L75) |
| `execute` | `self.process                     .recover_client_sessions(&request.session_ids)                     .map_err` | [1940](../../src/client_extensions.rs#L1940) | receiver-type-required |
| `execute` | `self.process                     .recover_client_sessions` | [1940](../../src/client_extensions.rs#L1940) | receiver-type-required |
| `execute` | `payload.get("sessionId").and_then` | [1952](../../src/client_extensions.rs#L1952) | receiver-type-required |
| `execute` | `payload.get` | [1952](../../src/client_extensions.rs#L1952) | receiver-type-required |
| `execute` | `self.process                             .client_resource_catalog(session_id)                             .map_err` | [1954](../../src/client_extensions.rs#L1954) | receiver-type-required |
| `execute` | `self.process                             .client_resource_catalog` | [1954](../../src/client_extensions.rs#L1954) | receiver-type-required |
| `execute` | `self.commands.skills_list` | [1977](../../src/client_extensions.rs#L1977) | receiver-type-required |
| `execute` | `self.commands.commands_list` | [1979](../../src/client_extensions.rs#L1979) | receiver-type-required |
| `execute` | `self.commands                     .validate_session(&input.session_id)                     .map_err` | [1984](../../src/client_extensions.rs#L1984) | receiver-type-required |
| `execute` | `self.commands                     .validate_session` | [1984](../../src/client_extensions.rs#L1984) | receiver-type-required |
| `execute` | `self                     .process                     .client_resource_catalog(&input.session_id)                     .map_err` | [1987](../../src/client_extensions.rs#L1987) | receiver-type-required |
| `execute` | `self                     .process                     .client_resource_catalog` | [1987](../../src/client_extensions.rs#L1987) | receiver-type-required |
| `execute` | `self                         .commands                         .commands_run_with_catalog(&catalog, &input, principal)                         .map_err` | [1998](../../src/client_extensions.rs#L1998) | receiver-type-required |
| `execute` | `self                         .commands                         .commands_run_with_catalog` | [1998](../../src/client_extensions.rs#L1998) | receiver-type-required |
| `execute` | `self.resources_list` | [2004](../../src/client_extensions.rs#L2004) | receiver-type-required |
| `execute` | `self.resources_read` | [2005](../../src/client_extensions.rs#L2005) | receiver-type-required |
| `execute` | `self.tools` | [2008](../../src/client_extensions.rs#L2008), [2012](../../src/client_extensions.rs#L2012) | receiver-type-required |
| `execute` | `request.session_id.as_deref` | [2008](../../src/client_extensions.rs#L2008), [2014](../../src/client_extensions.rs#L2014) | receiver-type-required |
| `execute` | `self.recover_plugin_endpoint_journal` | [2020](../../src/client_extensions.rs#L2020), [2029](../../src/client_extensions.rs#L2029), [2047](../../src/client_extensions.rs#L2047), [2154](../../src/client_extensions.rs#L2154) | receiver-type-required |
| `execute` | `lock` | [2021](../../src/client_extensions.rs#L2021), [2030](../../src/client_extensions.rs#L2030), [2050](../../src/client_extensions.rs#L2050), [2064](../../src/client_extensions.rs#L2064), [2133](../../src/client_extensions.rs#L2133), [2155](../../src/client_extensions.rs#L2155) | [tekes-supervisor::client_extensions::lock](../../src/client_extensions.rs#L1054) |
| `execute` | `self.plugin_store` | [2021](../../src/client_extensions.rs#L2021), [2030](../../src/client_extensions.rs#L2030), [2050](../../src/client_extensions.rs#L2050), [2064](../../src/client_extensions.rs#L2064), [2133](../../src/client_extensions.rs#L2133), [2155](../../src/client_extensions.rs#L2155) | receiver-type-required |
| `execute` | `store.list().map_err` | [2022](../../src/client_extensions.rs#L2022) | receiver-type-required |
| `execute` | `store.list` | [2022](../../src/client_extensions.rs#L2022) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err(map_plugin_error)?                     .into_iter()                     .find(&#124;item&#124; item.plugin_id == request.plugin_id)                     .ok_or_else` | [2031](../../src/client_extensions.rs#L2031) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err(map_plugin_error)?                     .into_iter()                     .find` | [2031](../../src/client_extensions.rs#L2031) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err(map_plugin_error)?                     .into_iter` | [2031](../../src/client_extensions.rs#L2031), [2156](../../src/client_extensions.rs#L2156) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err` | [2031](../../src/client_extensions.rs#L2031), [2156](../../src/client_extensions.rs#L2156) | receiver-type-required |
| `execute` | `store                     .list` | [2031](../../src/client_extensions.rs#L2031), [2156](../../src/client_extensions.rs#L2156) | receiver-type-required |
| `execute` | `PluginSource::from_path(&request.archive_path).map_err` | [2049](../../src/client_extensions.rs#L2049), [2067](../../src/client_extensions.rs#L2067) | receiver-type-required |
| `execute` | `PluginSource::from_path` | [2049](../../src/client_extensions.rs#L2049), [2067](../../src/client_extensions.rs#L2067) | [plugins::archive::PluginSource::from_path](../../../plugins/src/archive.rs#L28) |
| `execute` | `store.inspect(&source).map_err` | [2051](../../src/client_extensions.rs#L2051) | receiver-type-required |
| `execute` | `store.inspect` | [2051](../../src/client_extensions.rs#L2051) | receiver-type-required |
| `execute` | `self.plugin_operation` | [2060](../../src/client_extensions.rs#L2060), [2093](../../src/client_extensions.rs#L2093), [2111](../../src/client_extensions.rs#L2111), [2129](../../src/client_extensions.rs#L2129) | receiver-type-required |
| `execute` | `operation.completed` | [2061](../../src/client_extensions.rs#L2061), [2083](../../src/client_extensions.rs#L2083), [2094](../../src/client_extensions.rs#L2094), [2101](../../src/client_extensions.rs#L2101), [2112](../../src/client_extensions.rs#L2112), [2119](../../src/client_extensions.rs#L2119), [2130](../../src/client_extensions.rs#L2130), [2144](../../src/client_extensions.rs#L2144) | receiver-type-required |
| `execute` | `Ok` | [2062](../../src/client_extensions.rs#L2062), [2095](../../src/client_extensions.rs#L2095), [2113](../../src/client_extensions.rs#L2113), [2131](../../src/client_extensions.rs#L2131) | external-constructor-callback-or-unresolved |
| `execute` | `operation.path.is_file` | [2065](../../src/client_extensions.rs#L2065), [2097](../../src/client_extensions.rs#L2097), [2115](../../src/client_extensions.rs#L2115), [2134](../../src/client_extensions.rs#L2134) | receiver-type-required |
| `execute` | `store                         .freeze_source(&source, &operation.stage)                         .map_err` | [2068](../../src/client_extensions.rs#L2068) | receiver-type-required |
| `execute` | `store                         .freeze_source` | [2068](../../src/client_extensions.rs#L2068) | receiver-type-required |
| `execute` | `std::fs::remove_dir_all` | [2072](../../src/client_extensions.rs#L2072) | external-constructor-callback-or-unresolved |
| `execute` | `operation.prepare` | [2079](../../src/client_extensions.rs#L2079), [2098](../../src/client_extensions.rs#L2098), [2116](../../src/client_extensions.rs#L2116), [2140](../../src/client_extensions.rs#L2140) | receiver-type-required |
| `execute` | `drop` | [2081](../../src/client_extensions.rs#L2081), [2142](../../src/client_extensions.rs#L2142) | external-constructor-callback-or-unresolved |
| `execute` | `self.complete_prepared_plugin_operation` | [2082](../../src/client_extensions.rs#L2082), [2100](../../src/client_extensions.rs#L2100), [2118](../../src/client_extensions.rs#L2118), [2143](../../src/client_extensions.rs#L2143) | receiver-type-required |
| `execute` | `operation.completed()?.ok_or_else` | [2083](../../src/client_extensions.rs#L2083), [2101](../../src/client_extensions.rs#L2101), [2119](../../src/client_extensions.rs#L2119), [2144](../../src/client_extensions.rs#L2144) | receiver-type-required |
| `execute` | `store                         .list()                         .map_err(map_plugin_error)?                         .iter()                         .any` | [2135](../../src/client_extensions.rs#L2135) | receiver-type-required |
| `execute` | `store                         .list()                         .map_err(map_plugin_error)?                         .iter` | [2135](../../src/client_extensions.rs#L2135) | receiver-type-required |
| `execute` | `store                         .list()                         .map_err` | [2135](../../src/client_extensions.rs#L2135) | receiver-type-required |
| `execute` | `store                         .list` | [2135](../../src/client_extensions.rs#L2135) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err(map_plugin_error)?                     .into_iter()                     .map(&#124;receipt&#124; (receipt.plugin_id, receipt.package_digest))                     .collect::<std::collections::BTreeMap<_, _>>` | [2156](../../src/client_extensions.rs#L2156) | receiver-type-required |
| `execute` | `store                     .list()                     .map_err(map_plugin_error)?                     .into_iter()                     .map` | [2156](../../src/client_extensions.rs#L2156) | receiver-type-required |
| `execute` | `store.components().map_err` | [2162](../../src/client_extensions.rs#L2162) | receiver-type-required |
| `execute` | `store.components` | [2162](../../src/client_extensions.rs#L2162) | receiver-type-required |
| `execute` | `Vec::new` | [2163](../../src/client_extensions.rs#L2163) | external-constructor-callback-or-unresolved |
| `execute` | `generations.get(&item.owner_plugin_id).ok_or_else` | [2165](../../src/client_extensions.rs#L2165) | receiver-type-required |
| `execute` | `generations.get` | [2165](../../src/client_extensions.rs#L2165) | receiver-type-required |
| `execute` | `store                         .resolve_executable_component(&plugins::PluginComponentReference {                             plugin_id: item.owner_plugin_id.clone(),                             component_id: item.component_id.clone(),                         })                         .map_err(map_plugin_error)?                         .is_some` | [2172](../../src/client_extensions.rs#L2172) | receiver-type-required |
| `execute` | `store                         .resolve_executable_component(&plugins::PluginComponentReference {                             plugin_id: item.owner_plugin_id.clone(),                             component_id: item.component_id.clone(),                         })                         .map_err` | [2172](../../src/client_extensions.rs#L2172) | receiver-type-required |
| `execute` | `store                         .resolve_executable_component` | [2172](../../src/client_extensions.rs#L2172) | receiver-type-required |
| `execute` | `item.owner_plugin_id.clone` | [2174](../../src/client_extensions.rs#L2174) | receiver-type-required |
| `execute` | `item.component_id.clone` | [2175](../../src/client_extensions.rs#L2175) | receiver-type-required |
| `execute` | `views.push` | [2179](../../src/client_extensions.rs#L2179) | receiver-type-required |
| `execute` | `self.schedule_list` | [2188](../../src/client_extensions.rs#L2188) | receiver-type-required |
| `execute` | `self.schedule_save` | [2189](../../src/client_extensions.rs#L2189) | receiver-type-required |
| `execute` | `self.schedule_delete` | [2190](../../src/client_extensions.rs#L2190) | receiver-type-required |
| `execute` | `self.schedule_run_now` | [2191](../../src/client_extensions.rs#L2191) | receiver-type-required |
| `execute` | `self.thread_search` | [2192](../../src/client_extensions.rs#L2192) | receiver-type-required |
| `execute` | `self                         .search                         .search_sessions(&request.query)                         .map_err` | [2196](../../src/client_extensions.rs#L2196) | receiver-type-required |
| `execute` | `self                         .search                         .search_sessions` | [2196](../../src/client_extensions.rs#L2196) | receiver-type-required |
| `execute` | `self.usage` | [2202](../../src/client_extensions.rs#L2202), [2203](../../src/client_extensions.rs#L2203) | receiver-type-required |
| `resources_list` | `decode` | [2215](../../src/client_extensions.rs#L2215) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `resources_list` | `request.limit.unwrap_or` | [2216](../../src/client_extensions.rs#L2216) | receiver-type-required |
| `resources_list` | `(1..=100).contains` | [2217](../../src/client_extensions.rs#L2217) | receiver-type-required |
| `resources_list` | `Err` | [2218](../../src/client_extensions.rs#L2218), [2250](../../src/client_extensions.rs#L2250), [2274](../../src/client_extensions.rs#L2274), [2297](../../src/client_extensions.rs#L2297) | external-constructor-callback-or-unresolved |
| `resources_list` | `bad_request` | [2218](../../src/client_extensions.rs#L2218), [2297](../../src/client_extensions.rs#L2297) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `resources_list` | `request.source.as_str` | [2220](../../src/client_extensions.rs#L2220) | receiver-type-required |
| `resources_list` | `self.workspace_resource_catalog` | [2222](../../src/client_extensions.rs#L2222) | [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog](../../src/client_extensions.rs#L450) |
| `resources_list` | `Vec::new` | [2223](../../src/client_extensions.rs#L2223) | external-constructor-callback-or-unresolved |
| `resources_list` | `catalog.skill_packages` | [2224](../../src/client_extensions.rs#L2224) | receiver-type-required |
| `resources_list` | `items.push` | [2226](../../src/client_extensions.rs#L2226) | receiver-type-required |
| `resources_list` | `items.sort_by` | [2234](../../src/client_extensions.rs#L2234) | receiver-type-required |
| `resources_list` | `left["uri"].as_str().cmp` | [2234](../../src/client_extensions.rs#L2234) | receiver-type-required |
| `resources_list` | `left["uri"].as_str` | [2234](../../src/client_extensions.rs#L2234) | receiver-type-required |
| `resources_list` | `right["uri"].as_str` | [2234](../../src/client_extensions.rs#L2234) | receiver-type-required |
| `resources_list` | `decode_resource_cursor` | [2247](../../src/client_extensions.rs#L2247), [2271](../../src/client_extensions.rs#L2271) | [tekes-supervisor::client_extensions::decode_resource_cursor](../../src/client_extensions.rs#L1063) |
| `resources_list` | `items.len` | [2249](../../src/client_extensions.rs#L2249), [2256](../../src/client_extensions.rs#L2256), [2257](../../src/client_extensions.rs#L2257) | receiver-type-required |
| `resources_list` | `failure` | [2250](../../src/client_extensions.rs#L2250), [2274](../../src/client_extensions.rs#L2274) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `resources_list` | `start.saturating_add(limit).min` | [2256](../../src/client_extensions.rs#L2256), [2280](../../src/client_extensions.rs#L2280) | receiver-type-required |
| `resources_list` | `start.saturating_add` | [2256](../../src/client_extensions.rs#L2256), [2280](../../src/client_extensions.rs#L2280) | receiver-type-required |
| `resources_list` | `(end < items.len()).then` | [2257](../../src/client_extensions.rs#L2257) | receiver-type-required |
| `resources_list` | `Value::String` | [2260](../../src/client_extensions.rs#L2260), [2287](../../src/client_extensions.rs#L2287), [2293](../../src/client_extensions.rs#L2293) | external-constructor-callback-or-unresolved |
| `resources_list` | `encode` | [2262](../../src/client_extensions.rs#L2262), [2295](../../src/client_extensions.rs#L2295) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `resources_list` | `self                     .mcp                     .list_resource_catalog(&request.workspace_id)                     .map_err` | [2265](../../src/client_extensions.rs#L2265) | receiver-type-required |
| `resources_list` | `self                     .mcp                     .list_resource_catalog` | [2265](../../src/client_extensions.rs#L2265) | receiver-type-required |
| `resources_list` | `all.len` | [2273](../../src/client_extensions.rs#L2273), [2280](../../src/client_extensions.rs#L2280), [2292](../../src/client_extensions.rs#L2292) | receiver-type-required |
| `resources_list` | `all[start..end].iter().map(&#124;item&#124; {                     let mut row = serde_json::json!({                         "reference":{"kind":"mcp","server":item.server,"bindingDigest":item.binding_digest},                         "uri":item.resource.uri,"name":item.resource.name                     });                     if let Some(mime) = &item.resource.mime_type {                         row["mimeType"] = Value::String(mime.clone());                     }                     row                 }).collect::<Vec<_>>` | [2281](../../src/client_extensions.rs#L2281) | receiver-type-required |
| `resources_list` | `all[start..end].iter().map` | [2281](../../src/client_extensions.rs#L2281) | receiver-type-required |
| `resources_list` | `all[start..end].iter` | [2281](../../src/client_extensions.rs#L2281) | receiver-type-required |
| `resources_list` | `mime.clone` | [2287](../../src/client_extensions.rs#L2287) | receiver-type-required |
| `resources_read` | `decode` | [2302](../../src/client_extensions.rs#L2302) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `resources_read` | `serde_json::from_value::<ResourceReference>(request.reference)             .map_err` | [2303](../../src/client_extensions.rs#L2303) | receiver-type-required |
| `resources_read` | `serde_json::from_value::<ResourceReference>` | [2303](../../src/client_extensions.rs#L2303) | external-constructor-callback-or-unresolved |
| `resources_read` | `bad_request` | [2304](../../src/client_extensions.rs#L2304) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `resources_read` | `Err` | [2312](../../src/client_extensions.rs#L2312), [2327](../../src/client_extensions.rs#L2327), [2344](../../src/client_extensions.rs#L2344), [2359](../../src/client_extensions.rs#L2359) | external-constructor-callback-or-unresolved |
| `resources_read` | `failure` | [2312](../../src/client_extensions.rs#L2312), [2320](../../src/client_extensions.rs#L2320), [2327](../../src/client_extensions.rs#L2327), [2337](../../src/client_extensions.rs#L2337), [2344](../../src/client_extensions.rs#L2344), [2359](../../src/client_extensions.rs#L2359) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `resources_read` | `self.workspace_resource_catalog` | [2318](../../src/client_extensions.rs#L2318) | [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog](../../src/client_extensions.rs#L450) |
| `resources_read` | `catalog.skill(&name).map_err` | [2319](../../src/client_extensions.rs#L2319) | receiver-type-required |
| `resources_read` | `catalog.skill` | [2319](../../src/client_extensions.rs#L2319) | receiver-type-required |
| `resources_read` | `self                     .workspace_resource_catalog(&request.workspace_id)?                     .skill_resource(&name, &request.uri)                     .map_err` | [2333](../../src/client_extensions.rs#L2333) | receiver-type-required |
| `resources_read` | `self                     .workspace_resource_catalog(&request.workspace_id)?                     .skill_resource` | [2333](../../src/client_extensions.rs#L2333) | receiver-type-required |
| `resources_read` | `self                     .workspace_resource_catalog` | [2333](../../src/client_extensions.rs#L2333) | [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_resource_catalog](../../src/client_extensions.rs#L450) |
| `resources_read` | `text.len` | [2343](../../src/client_extensions.rs#L2343) | receiver-type-required |
| `resources_read` | `encode` | [2350](../../src/client_extensions.rs#L2350), [2371](../../src/client_extensions.rs#L2371) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `resources_read` | `self                     .mcp                     .read_resource(&server, &binding_digest, &request.uri)                     .map_err` | [2365](../../src/client_extensions.rs#L2365) | receiver-type-required |
| `resources_read` | `self                     .mcp                     .read_resource` | [2365](../../src/client_extensions.rs#L2365) | receiver-type-required |
| `resources_read` | `ijson_value` | [2369](../../src/client_extensions.rs#L2369) | [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) |
| `resources_read` | `normalize_mcp_resource` | [2370](../../src/client_extensions.rs#L2370) | [tekes-supervisor::client_extensions::normalize_mcp_resource](../../src/client_extensions.rs#L1086) |
| `tools` | `self             .process             .client_tool_catalog(workspace_id, session_id)             .map_err` | [2382](../../src/client_extensions.rs#L2382) | receiver-type-required |
| `tools` | `self             .process             .client_tool_catalog` | [2382](../../src/client_extensions.rs#L2382) | receiver-type-required |
| `tools` | `failure` | [2386](../../src/client_extensions.rs#L2386), [2396](../../src/client_extensions.rs#L2396), [2431](../../src/client_extensions.rs#L2431) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `tools` | `Vec::new` | [2392](../../src/client_extensions.rs#L2392) | external-constructor-callback-or-unresolved |
| `tools` | `BuiltinManifest::compiled` | [2393](../../src/client_extensions.rs#L2393) | [tools::builtin::BuiltinManifest::compiled](../../../tools/src/builtin.rs#L315) |
| `tools` | `fixed_schema(&tool.name).ok_or_else` | [2395](../../src/client_extensions.rs#L2395) | receiver-type-required |
| `tools` | `fixed_schema` | [2395](../../src/client_extensions.rs#L2395) | [tools::schema_registry::fixed_schema](../../../tools/src/schema_registry.rs#L1225) |
| `tools` | `rows.push` | [2402](../../src/client_extensions.rs#L2402), [2410](../../src/client_extensions.rs#L2410) | receiver-type-required |
| `tools` | `ijson_value` | [2409](../../src/client_extensions.rs#L2409) | [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) |
| `tools` | `rows.sort_by` | [2417](../../src/client_extensions.rs#L2417) | receiver-type-required |
| `tools` | `left["name"].as_str().cmp` | [2417](../../src/client_extensions.rs#L2417) | receiver-type-required |
| `tools` | `left["name"].as_str` | [2417](../../src/client_extensions.rs#L2417) | receiver-type-required |
| `tools` | `right["name"].as_str` | [2417](../../src/client_extensions.rs#L2417) | receiver-type-required |
| `tools` | `rows                 .into_iter()                 .find(&#124;row&#124; row["name"] == name)                 .ok_or_else` | [2427](../../src/client_extensions.rs#L2427) | receiver-type-required |
| `tools` | `rows                 .into_iter()                 .find` | [2427](../../src/client_extensions.rs#L2427) | receiver-type-required |
| `tools` | `rows                 .into_iter` | [2427](../../src/client_extensions.rs#L2427) | receiver-type-required |
| `tools` | `encode` | [2437](../../src/client_extensions.rs#L2437), [2439](../../src/client_extensions.rs#L2439) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `plugin_mutation_value` | `self.plugin_readiness` | [2444](../../src/client_extensions.rs#L2444) | [tekes-supervisor::client_extensions::ProductionClientExtensions::plugin_readiness](../../src/client_extensions.rs#L2448) |
| `plugin_readiness` | `self.mcp.reconcile_authority` | [2449](../../src/client_extensions.rs#L2449) | receiver-type-required |
| `schedule_list` | `decode` | [2458](../../src/client_extensions.rs#L2458) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `schedule_list` | `encode` | [2459](../../src/client_extensions.rs#L2459) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `schedule_save` | `decode` | [2470](../../src/client_extensions.rs#L2470) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `schedule_save` | `Err` | [2472](../../src/client_extensions.rs#L2472), [2479](../../src/client_extensions.rs#L2479) | external-constructor-callback-or-unresolved |
| `schedule_save` | `failure` | [2472](../../src/client_extensions.rs#L2472), [2479](../../src/client_extensions.rs#L2479), [2500](../../src/client_extensions.rs#L2500) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `schedule_save` | `request.definition.model_id.is_some` | [2478](../../src/client_extensions.rs#L2478) | receiver-type-required |
| `schedule_save` | `self.workspace_snapshot` | [2485](../../src/client_extensions.rs#L2485) | [tekes-supervisor::client_extensions::ProductionClientExtensions::workspace_snapshot](../../src/client_extensions.rs#L435) |
| `schedule_save` | `self             .process             .schedule_authority()             .save(                 schedule::OriginTuple {                     client_id: principal.to_owned(),                     key: rpc_id.to_owned(),                 },                 request.definition,                 Utc::now(),             )             .map_err` | [2486](../../src/client_extensions.rs#L2486) | receiver-type-required |
| `schedule_save` | `self             .process             .schedule_authority()             .save` | [2486](../../src/client_extensions.rs#L2486) | receiver-type-required |
| `schedule_save` | `self             .process             .schedule_authority` | [2486](../../src/client_extensions.rs#L2486) | receiver-type-required |
| `schedule_save` | `principal.to_owned` | [2491](../../src/client_extensions.rs#L2491) | receiver-type-required |
| `schedule_save` | `rpc_id.to_owned` | [2492](../../src/client_extensions.rs#L2492) | receiver-type-required |
| `schedule_save` | `Utc::now` | [2495](../../src/client_extensions.rs#L2495) | external-constructor-callback-or-unresolved |
| `schedule_save` | `self.process.start_schedule_timer().map_err` | [2499](../../src/client_extensions.rs#L2499) | receiver-type-required |
| `schedule_save` | `self.process.start_schedule_timer` | [2499](../../src/client_extensions.rs#L2499) | receiver-type-required |
| `schedule_save` | `encode` | [2507](../../src/client_extensions.rs#L2507) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `schedule_delete` | `decode` | [2516](../../src/client_extensions.rs#L2516) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `schedule_delete` | `self             .process             .schedule_authority()             .delete(                 schedule::OriginTuple {                     client_id: principal.to_owned(),                     key: rpc_id.to_owned(),                 },                 &request.task_id,                 Utc::now(),             )             .map_err` | [2517](../../src/client_extensions.rs#L2517) | receiver-type-required |
| `schedule_delete` | `self             .process             .schedule_authority()             .delete` | [2517](../../src/client_extensions.rs#L2517) | receiver-type-required |
| `schedule_delete` | `self             .process             .schedule_authority` | [2517](../../src/client_extensions.rs#L2517) | receiver-type-required |
| `schedule_delete` | `principal.to_owned` | [2522](../../src/client_extensions.rs#L2522) | receiver-type-required |
| `schedule_delete` | `rpc_id.to_owned` | [2523](../../src/client_extensions.rs#L2523) | receiver-type-required |
| `schedule_delete` | `Utc::now` | [2526](../../src/client_extensions.rs#L2526) | external-constructor-callback-or-unresolved |
| `schedule_delete` | `encode` | [2529](../../src/client_extensions.rs#L2529) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `schedule_run_now` | `decode` | [2538](../../src/client_extensions.rs#L2538) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `schedule_run_now` | `self             .process             .schedule_authority()             .run_now(                 schedule::OriginTuple {                     client_id: principal.to_owned(),                     key: rpc_id.to_owned(),                 },                 &request.task_id,                 Utc::now(),             )             .map_err` | [2539](../../src/client_extensions.rs#L2539) | receiver-type-required |
| `schedule_run_now` | `self             .process             .schedule_authority()             .run_now` | [2539](../../src/client_extensions.rs#L2539) | receiver-type-required |
| `schedule_run_now` | `self             .process             .schedule_authority` | [2539](../../src/client_extensions.rs#L2539) | receiver-type-required |
| `schedule_run_now` | `principal.to_owned` | [2544](../../src/client_extensions.rs#L2544) | receiver-type-required |
| `schedule_run_now` | `rpc_id.to_owned` | [2545](../../src/client_extensions.rs#L2545) | receiver-type-required |
| `schedule_run_now` | `Utc::now` | [2548](../../src/client_extensions.rs#L2548) | external-constructor-callback-or-unresolved |
| `schedule_run_now` | `self.process.start_schedule_timer().map_err` | [2551](../../src/client_extensions.rs#L2551) | receiver-type-required |
| `schedule_run_now` | `self.process.start_schedule_timer` | [2551](../../src/client_extensions.rs#L2551) | receiver-type-required |
| `schedule_run_now` | `failure` | [2552](../../src/client_extensions.rs#L2552) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `schedule_run_now` | `encode` | [2558](../../src/client_extensions.rs#L2558) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `thread_search` | `decode` | [2562](../../src/client_extensions.rs#L2562) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `thread_search` | `self             .search             .search(&SearchRequest {                 workspace_id: request.workspace_id,                 query: request.query,                 limit: request.limit,                 visibility: request.visibility,                 after: request.after,             })             .map_err` | [2563](../../src/client_extensions.rs#L2563) | receiver-type-required |
| `thread_search` | `self             .search             .search` | [2563](../../src/client_extensions.rs#L2563) | receiver-type-required |
| `thread_search` | `Value::String` | [2575](../../src/client_extensions.rs#L2575) | external-constructor-callback-or-unresolved |
| `thread_search` | `encode` | [2577](../../src/client_extensions.rs#L2577) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `usage` | `decode` | [2581](../../src/client_extensions.rs#L2581) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `usage` | `["threads", "archive"]             .into_iter()             .map(&#124;area&#124; self.root.join(area).join(&request.session_id))             .find(&#124;path&#124; path.is_dir())             .ok_or_else` | [2582](../../src/client_extensions.rs#L2582) | receiver-type-required |
| `usage` | `["threads", "archive"]             .into_iter()             .map(&#124;area&#124; self.root.join(area).join(&request.session_id))             .find` | [2582](../../src/client_extensions.rs#L2582) | receiver-type-required |
| `usage` | `["threads", "archive"]             .into_iter()             .map` | [2582](../../src/client_extensions.rs#L2582) | receiver-type-required |
| `usage` | `["threads", "archive"]             .into_iter` | [2582](../../src/client_extensions.rs#L2582) | receiver-type-required |
| `usage` | `self.root.join(area).join` | [2584](../../src/client_extensions.rs#L2584) | receiver-type-required |
| `usage` | `self.root.join` | [2584](../../src/client_extensions.rs#L2584) | receiver-type-required |
| `usage` | `path.is_dir` | [2585](../../src/client_extensions.rs#L2585) | receiver-type-required |
| `usage` | `failure` | [2587](../../src/client_extensions.rs#L2587), [2594](../../src/client_extensions.rs#L2594), [2601](../../src/client_extensions.rs#L2601), [2616](../../src/client_extensions.rs#L2616), [2623](../../src/client_extensions.rs#L2623), [2637](../../src/client_extensions.rs#L2637), [2645](../../src/client_extensions.rs#L2645), [2654](../../src/client_extensions.rs#L2654), [2661](../../src/client_extensions.rs#L2661), [2671](../../src/client_extensions.rs#L2671), [2701](../../src/client_extensions.rs#L2701), [2708](../../src/client_extensions.rs#L2708), [2726](../../src/client_extensions.rs#L2726), [2733](../../src/client_extensions.rs#L2733), [2740](../../src/client_extensions.rs#L2740), [2747](../../src/client_extensions.rs#L2747), [2775](../../src/client_extensions.rs#L2775), [2785](../../src/client_extensions.rs#L2785) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `usage` | `std::fs::read(folder.join("main.jsonl")).map_err` | [2593](../../src/client_extensions.rs#L2593) | receiver-type-required |
| `usage` | `std::fs::read` | [2593](../../src/client_extensions.rs#L2593) | external-constructor-callback-or-unresolved |
| `usage` | `folder.join` | [2593](../../src/client_extensions.rs#L2593), [2600](../../src/client_extensions.rs#L2600) | receiver-type-required |
| `usage` | `store::AssetStore::new(folder.join("assets")).map_err` | [2600](../../src/client_extensions.rs#L2600) | receiver-type-required |
| `usage` | `store::AssetStore::new` | [2600](../../src/client_extensions.rs#L2600) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `usage` | `store::scan_valid_prefix` | [2607](../../src/client_extensions.rs#L2607) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `usage` | `usize::try_from(scan.valid_bytes).unwrap_or` | [2610](../../src/client_extensions.rs#L2610) | receiver-type-required |
| `usage` | `usize::try_from` | [2610](../../src/client_extensions.rs#L2610) | external-constructor-callback-or-unresolved |
| `usage` | `scan.needs_repair` | [2611](../../src/client_extensions.rs#L2611) | receiver-type-required |
| `usage` | `bytes                 .get(valid_len..)                 .is_some_and` | [2612](../../src/client_extensions.rs#L2612) | receiver-type-required |
| `usage` | `bytes                 .get` | [2612](../../src/client_extensions.rs#L2612) | receiver-type-required |
| `usage` | `tail.contains` | [2614](../../src/client_extensions.rs#L2614) | receiver-type-required |
| `usage` | `Err` | [2616](../../src/client_extensions.rs#L2616), [2645](../../src/client_extensions.rs#L2645), [2671](../../src/client_extensions.rs#L2671), [2785](../../src/client_extensions.rs#L2785) | external-constructor-callback-or-unresolved |
| `usage` | `scan.projection.ok_or_else` | [2622](../../src/client_extensions.rs#L2622) | receiver-type-required |
| `usage` | `BTreeMap::<String, Value>::new` | [2629](../../src/client_extensions.rs#L2629) | external-constructor-callback-or-unresolved |
| `usage` | `BTreeMap::<String, String>::new` | [2630](../../src/client_extensions.rs#L2630) | external-constructor-callback-or-unresolved |
| `usage` | `event.kind` | [2633](../../src/client_extensions.rs#L2633) | receiver-type-required |
| `usage` | `ijson_value` | [2635](../../src/client_extensions.rs#L2635), [2692](../../src/client_extensions.rs#L2692) | [tekes-supervisor::client_extensions::ijson_value](../../src/client_extensions.rs#L1037) |
| `usage` | `event.raw` | [2635](../../src/client_extensions.rs#L2635), [2692](../../src/client_extensions.rs#L2692) | receiver-type-required |
| `usage` | `event.string_field("id").ok_or_else` | [2636](../../src/client_extensions.rs#L2636) | receiver-type-required |
| `usage` | `event.string_field` | [2636](../../src/client_extensions.rs#L2636), [2653](../../src/client_extensions.rs#L2653), [2660](../../src/client_extensions.rs#L2660), [2725](../../src/client_extensions.rs#L2725) | receiver-type-required |
| `usage` | `usage_route_target` | [2643](../../src/client_extensions.rs#L2643) | [tekes-supervisor::client_extensions::usage_route_target](../../src/client_extensions.rs#L1368) |
| `usage` | `epoch_targets.insert(id.to_owned(), target).is_some` | [2644](../../src/client_extensions.rs#L2644) | receiver-type-required |
| `usage` | `epoch_targets.insert` | [2644](../../src/client_extensions.rs#L2644) | receiver-type-required |
| `usage` | `id.to_owned` | [2644](../../src/client_extensions.rs#L2644) | receiver-type-required |
| `usage` | `event.string_field("attempt").ok_or_else` | [2653](../../src/client_extensions.rs#L2653), [2725](../../src/client_extensions.rs#L2725) | receiver-type-required |
| `usage` | `event.string_field("epoch").ok_or_else` | [2660](../../src/client_extensions.rs#L2660) | receiver-type-required |
| `usage` | `attempt_epochs                             .insert(attempt.to_owned(), epoch.to_owned())                             .is_some` | [2667](../../src/client_extensions.rs#L2667) | receiver-type-required |
| `usage` | `attempt_epochs                             .insert` | [2667](../../src/client_extensions.rs#L2667) | receiver-type-required |
| `usage` | `attempt.to_owned` | [2668](../../src/client_extensions.rs#L2668) | receiver-type-required |
| `usage` | `epoch.to_owned` | [2668](../../src/client_extensions.rs#L2668) | receiver-type-required |
| `usage` | `BTreeMap::<Vec<u8>, CacheAttributionGroup>::new` | [2684](../../src/client_extensions.rs#L2684) | external-constructor-callback-or-unresolved |
| `usage` | `event.has_field` | [2688](../../src/client_extensions.rs#L2688) | receiver-type-required |
| `usage` | `outcome.get("usage").cloned().unwrap_or` | [2693](../../src/client_extensions.rs#L2693) | receiver-type-required |
| `usage` | `outcome.get("usage").cloned` | [2693](../../src/client_extensions.rs#L2693) | receiver-type-required |
| `usage` | `outcome.get` | [2693](../../src/client_extensions.rs#L2693) | receiver-type-required |
| `usage` | `raw                 .get("availability")                 .and_then(Value::as_str)                 .unwrap_or` | [2694](../../src/client_extensions.rs#L2694) | receiver-type-required |
| `usage` | `raw                 .get("availability")                 .and_then` | [2694](../../src/client_extensions.rs#L2694) | receiver-type-required |
| `usage` | `raw                 .get` | [2694](../../src/client_extensions.rs#L2694) | receiver-type-required |
| `usage` | `numeric_token` | [2698](../../src/client_extensions.rs#L2698), [2699](../../src/client_extensions.rs#L2699) | [tekes-supervisor::client_extensions::numeric_token](../../src/client_extensions.rs#L1152) |
| `usage` | `raw.get` | [2698](../../src/client_extensions.rs#L2698), [2699](../../src/client_extensions.rs#L2699), [2716](../../src/client_extensions.rs#L2716), [2717](../../src/client_extensions.rs#L2717), [2718](../../src/client_extensions.rs#L2718), [2761](../../src/client_extensions.rs#L2761) | receiver-type-required |
| `usage` | `input.checked_add(parsed_input).ok_or_else` | [2700](../../src/client_extensions.rs#L2700) | receiver-type-required |
| `usage` | `input.checked_add` | [2700](../../src/client_extensions.rs#L2700), [2774](../../src/client_extensions.rs#L2774) | receiver-type-required |
| `usage` | `output.checked_add(parsed_output).ok_or_else` | [2707](../../src/client_extensions.rs#L2707) | receiver-type-required |
| `usage` | `output.checked_add` | [2707](../../src/client_extensions.rs#L2707) | receiver-type-required |
| `usage` | `raw.get("input_tokens").and_then` | [2716](../../src/client_extensions.rs#L2716) | receiver-type-required |
| `usage` | `raw.get("output_tokens").and_then` | [2717](../../src/client_extensions.rs#L2717) | receiver-type-required |
| `usage` | `raw.get("cache_read").and_then` | [2718](../../src/client_extensions.rs#L2718) | receiver-type-required |
| `usage` | `canonical_unsigned` | [2722](../../src/client_extensions.rs#L2722), [2723](../../src/client_extensions.rs#L2723), [2724](../../src/client_extensions.rs#L2724) | [tekes-supervisor::client_extensions::canonical_unsigned](../../src/client_extensions.rs#L1256) |
| `usage` | `attempt_epochs.get(attempt).ok_or_else` | [2732](../../src/client_extensions.rs#L2732) | receiver-type-required |
| `usage` | `attempt_epochs.get` | [2732](../../src/client_extensions.rs#L2732) | receiver-type-required |
| `usage` | `epoch_targets.get(epoch).ok_or_else` | [2739](../../src/client_extensions.rs#L2739) | receiver-type-required |
| `usage` | `epoch_targets.get` | [2739](../../src/client_extensions.rs#L2739) | receiver-type-required |
| `usage` | `serde_json_canonicalizer::to_vec(target).map_err` | [2746](../../src/client_extensions.rs#L2746) | receiver-type-required |
| `usage` | `serde_json_canonicalizer::to_vec` | [2746](../../src/client_extensions.rs#L2746) | external-constructor-callback-or-unresolved |
| `usage` | `groups                     .entry(key)                     .or_insert_with(&#124;&#124; CacheAttributionGroup::new(target.clone()))                     .add` | [2753](../../src/client_extensions.rs#L2753) | receiver-type-required |
| `usage` | `groups                     .entry(key)                     .or_insert_with` | [2753](../../src/client_extensions.rs#L2753) | receiver-type-required |
| `usage` | `groups                     .entry` | [2753](../../src/client_extensions.rs#L2753) | receiver-type-required |
| `usage` | `CacheAttributionGroup::new` | [2755](../../src/client_extensions.rs#L2755) | [tekes-supervisor::client_extensions::CacheAttributionGroup::new](../../src/client_extensions.rs#L1177) |
| `usage` | `target.clone` | [2755](../../src/client_extensions.rs#L2755) | receiver-type-required |
| `usage` | `raw.get("cost").and_then` | [2761](../../src/client_extensions.rs#L2761) | receiver-type-required |
| `usage` | `groups                 .into_values()                 .map(CacheAttributionGroup::value)                 .collect::<Result<Vec<_>, _>>` | [2766](../../src/client_extensions.rs#L2766) | receiver-type-required |
| `usage` | `groups                 .into_values()                 .map` | [2766](../../src/client_extensions.rs#L2766) | receiver-type-required |
| `usage` | `groups                 .into_values` | [2766](../../src/client_extensions.rs#L2766) | receiver-type-required |
| `usage` | `encode` | [2770](../../src/client_extensions.rs#L2770), [2791](../../src/client_extensions.rs#L2791) | [tekes-supervisor::client_extensions::encode](../../src/client_extensions.rs#L1027) |
| `usage` | `input.checked_add(output).ok_or_else` | [2774](../../src/client_extensions.rs#L2774) | receiver-type-required |
| `usage` | `[input, output, total]                 .into_iter()                 .any` | [2781](../../src/client_extensions.rs#L2781) | receiver-type-required |
| `usage` | `[input, output, total]                 .into_iter` | [2781](../../src/client_extensions.rs#L2781) | receiver-type-required |
| `map_attachment_read_error` | `failure` | [2806](../../src/client_extensions.rs#L2806), [2811](../../src/client_extensions.rs#L2811), [2816](../../src/client_extensions.rs#L2816), [2821](../../src/client_extensions.rs#L2821) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `error_allowed` | `Some` | [2831](../../src/client_extensions.rs#L2831) | external-constructor-callback-or-unresolved |
| `error_allowed` | `value.starts_with` | [2852](../../src/client_extensions.rs#L2852), [2853](../../src/client_extensions.rs#L2853), [2857](../../src/client_extensions.rs#L2857), [2884](../../src/client_extensions.rs#L2884), [2894](../../src/client_extensions.rs#L2894) | receiver-type-required |
| `validate_payload` | `payload.as_object().is_some_and` | [2925](../../src/client_extensions.rs#L2925), [2934](../../src/client_extensions.rs#L2934), [2946](../../src/client_extensions.rs#L2946) | receiver-type-required |
| `validate_payload` | `payload.as_object` | [2925](../../src/client_extensions.rs#L2925), [2934](../../src/client_extensions.rs#L2934), [2946](../../src/client_extensions.rs#L2946) | receiver-type-required |
| `validate_payload` | `Ok` | [2926](../../src/client_extensions.rs#L2926), [2940](../../src/client_extensions.rs#L2940), [2947](../../src/client_extensions.rs#L2947), [2973](../../src/client_extensions.rs#L2973), [2990](../../src/client_extensions.rs#L2990), [2996](../../src/client_extensions.rs#L2996), [3005](../../src/client_extensions.rs#L3005), [3014](../../src/client_extensions.rs#L3014), [3023](../../src/client_extensions.rs#L3023), [3037](../../src/client_extensions.rs#L3037), [3049](../../src/client_extensions.rs#L3049) | external-constructor-callback-or-unresolved |
| `validate_payload` | `Err` | [2928](../../src/client_extensions.rs#L2928), [2942](../../src/client_extensions.rs#L2942), [2949](../../src/client_extensions.rs#L2949), [2971](../../src/client_extensions.rs#L2971), [3003](../../src/client_extensions.rs#L3003), [3012](../../src/client_extensions.rs#L3012), [3021](../../src/client_extensions.rs#L3021), [3039](../../src/client_extensions.rs#L3039), [3051](../../src/client_extensions.rs#L3051), [3068](../../src/client_extensions.rs#L3068) | external-constructor-callback-or-unresolved |
| `validate_payload` | `bad_request` | [2928](../../src/client_extensions.rs#L2928), [2942](../../src/client_extensions.rs#L2942), [2949](../../src/client_extensions.rs#L2949), [2955](../../src/client_extensions.rs#L2955), [2971](../../src/client_extensions.rs#L2971), [2976](../../src/client_extensions.rs#L2976), [2979](../../src/client_extensions.rs#L2979), [2982](../../src/client_extensions.rs#L2982), [2988](../../src/client_extensions.rs#L2988), [2995](../../src/client_extensions.rs#L2995), [3001](../../src/client_extensions.rs#L3001), [3003](../../src/client_extensions.rs#L3003), [3010](../../src/client_extensions.rs#L3010), [3012](../../src/client_extensions.rs#L3012), [3019](../../src/client_extensions.rs#L3019), [3021](../../src/client_extensions.rs#L3021), [3029](../../src/client_extensions.rs#L3029), [3039](../../src/client_extensions.rs#L3039), [3051](../../src/client_extensions.rs#L3051) | [tekes-supervisor::client_extensions::bad_request](../../src/client_extensions.rs#L1047) |
| `validate_payload` | `object.len` | [2935](../../src/client_extensions.rs#L2935), [3031](../../src/client_extensions.rs#L3031) | receiver-type-required |
| `validate_payload` | `object.get("workspaceId").is_some_and` | [2936](../../src/client_extensions.rs#L2936) | receiver-type-required |
| `validate_payload` | `object.get` | [2936](../../src/client_extensions.rs#L2936) | receiver-type-required |
| `validate_payload` | `object.contains_key` | [2937](../../src/client_extensions.rs#L2937) | receiver-type-required |
| `validate_payload` | `decode` | [2953](../../src/client_extensions.rs#L2953), [2985](../../src/client_extensions.rs#L2985), [2993](../../src/client_extensions.rs#L2993), [2999](../../src/client_extensions.rs#L2999), [3008](../../src/client_extensions.rs#L3008), [3017](../../src/client_extensions.rs#L3017) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `endpoint::validate_session_id(&request.session_id)                 .map_err` | [2954](../../src/client_extensions.rs#L2954), [2994](../../src/client_extensions.rs#L2994), [3000](../../src/client_extensions.rs#L3000), [3009](../../src/client_extensions.rs#L3009), [3018](../../src/client_extensions.rs#L3018) | receiver-type-required |
| `validate_payload` | `endpoint::validate_session_id` | [2954](../../src/client_extensions.rs#L2954), [2987](../../src/client_extensions.rs#L2987), [2994](../../src/client_extensions.rs#L2994), [3000](../../src/client_extensions.rs#L3000), [3009](../../src/client_extensions.rs#L3009), [3018](../../src/client_extensions.rs#L3018) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `validate_payload` | `request.path.is_empty` | [2956](../../src/client_extensions.rs#L2956) | receiver-type-required |
| `validate_payload` | `request.path.contains` | [2957](../../src/client_extensions.rs#L2957) | receiver-type-required |
| `validate_payload` | `Some` | [2958](../../src/client_extensions.rs#L2958), [2959](../../src/client_extensions.rs#L2959) | external-constructor-callback-or-unresolved |
| `validate_payload` | `request.offset.is_some` | [2961](../../src/client_extensions.rs#L2961) | receiver-type-required |
| `validate_payload` | `request.limit.is_some` | [2961](../../src/client_extensions.rs#L2961) | receiver-type-required |
| `validate_payload` | `request.limit.is_some_and` | [2962](../../src/client_extensions.rs#L2962) | receiver-type-required |
| `validate_payload` | `session_controls::METHODS.contains` | [2975](../../src/client_extensions.rs#L2975) | receiver-type-required |
| `validate_payload` | `session_controls::validate(operation, payload).map_err` | [2976](../../src/client_extensions.rs#L2976) | receiver-type-required |
| `validate_payload` | `session_controls::validate` | [2976](../../src/client_extensions.rs#L2976) | [session-controls::validate](../../../session-controls/src/lib.rs#L180) |
| `validate_payload` | `user_documents::METHODS.contains` | [2978](../../src/client_extensions.rs#L2978) | receiver-type-required |
| `validate_payload` | `user_documents::validate(operation, payload).map_err` | [2979](../../src/client_extensions.rs#L2979) | receiver-type-required |
| `validate_payload` | `user_documents::validate` | [2979](../../src/client_extensions.rs#L2979) | [user-documents::validate](../../../user-documents/src/lib.rs#L64) |
| `validate_payload` | `host_files::METHODS.contains` | [2981](../../src/client_extensions.rs#L2981) | receiver-type-required |
| `validate_payload` | `host_files::validate(operation, payload).map_err` | [2982](../../src/client_extensions.rs#L2982) | receiver-type-required |
| `validate_payload` | `host_files::validate` | [2982](../../src/client_extensions.rs#L2982) | [host-files::validate](../../../host-files/src/lib.rs#L145) |
| `validate_payload` | `endpoint::validate_session_id(&id)                     .map_err` | [2987](../../src/client_extensions.rs#L2987) | receiver-type-required |
| `validate_payload` | `engine::PermissionMode::parse(&request.mode).is_none` | [3002](../../src/client_extensions.rs#L3002) | receiver-type-required |
| `validate_payload` | `engine::PermissionMode::parse` | [3002](../../src/client_extensions.rs#L3002) | [engine::permission_mode::PermissionMode::parse](../../../engine/src/permission_mode.rs#L51) |
| `validate_payload` | `request.name.is_empty` | [3011](../../src/client_extensions.rs#L3011), [3048](../../src/client_extensions.rs#L3048) | receiver-type-required |
| `validate_payload` | `request.data.is_empty` | [3011](../../src/client_extensions.rs#L3011) | receiver-type-required |
| `validate_payload` | `request.attachment_id.is_empty` | [3020](../../src/client_extensions.rs#L3020) | receiver-type-required |
| `validate_payload` | `decode::<RecoveryRequest>(payload).map` | [3025](../../src/client_extensions.rs#L3025) | receiver-type-required |
| `validate_payload` | `decode::<RecoveryRequest>` | [3025](../../src/client_extensions.rs#L3025) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `payload                 .as_object()                 .ok_or_else` | [3027](../../src/client_extensions.rs#L3027) | receiver-type-required |
| `validate_payload` | `payload                 .as_object` | [3027](../../src/client_extensions.rs#L3027) | receiver-type-required |
| `validate_payload` | `object.is_empty` | [3030](../../src/client_extensions.rs#L3030) | receiver-type-required |
| `validate_payload` | `object                         .get("sessionId")                         .and_then(Value::as_str)                         .is_some_and` | [3032](../../src/client_extensions.rs#L3032) | receiver-type-required |
| `validate_payload` | `object                         .get("sessionId")                         .and_then` | [3032](../../src/client_extensions.rs#L3032) | receiver-type-required |
| `validate_payload` | `object                         .get` | [3032](../../src/client_extensions.rs#L3032) | receiver-type-required |
| `validate_payload` | `id.is_empty` | [3035](../../src/client_extensions.rs#L3035) | receiver-type-required |
| `validate_payload` | `decode::<CommandRunRequest>(payload).map` | [3042](../../src/client_extensions.rs#L3042) | receiver-type-required |
| `validate_payload` | `decode::<CommandRunRequest>` | [3042](../../src/client_extensions.rs#L3042) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ResourceListRequest>(payload).map` | [3043](../../src/client_extensions.rs#L3043) | receiver-type-required |
| `validate_payload` | `decode::<ResourceListRequest>` | [3043](../../src/client_extensions.rs#L3043) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ResourceReadRequest>(payload).map` | [3044](../../src/client_extensions.rs#L3044) | receiver-type-required |
| `validate_payload` | `decode::<ResourceReadRequest>` | [3044](../../src/client_extensions.rs#L3044) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ToolListRequest>(payload).map` | [3045](../../src/client_extensions.rs#L3045) | receiver-type-required |
| `validate_payload` | `decode::<ToolListRequest>` | [3045](../../src/client_extensions.rs#L3045) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ToolResolveRequest>` | [3047](../../src/client_extensions.rs#L3047) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<EmptyRequest>(payload).map` | [3054](../../src/client_extensions.rs#L3054) | receiver-type-required |
| `validate_payload` | `decode::<EmptyRequest>` | [3054](../../src/client_extensions.rs#L3054) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<PluginIdRequest>(payload).map` | [3055](../../src/client_extensions.rs#L3055) | receiver-type-required |
| `validate_payload` | `decode::<PluginIdRequest>` | [3055](../../src/client_extensions.rs#L3055) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<PluginInspectRequest>(payload).map` | [3056](../../src/client_extensions.rs#L3056) | receiver-type-required |
| `validate_payload` | `decode::<PluginInspectRequest>` | [3056](../../src/client_extensions.rs#L3056) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<PluginInstallRequest>(payload).map` | [3057](../../src/client_extensions.rs#L3057) | receiver-type-required |
| `validate_payload` | `decode::<PluginInstallRequest>` | [3057](../../src/client_extensions.rs#L3057) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<PluginEnabledRequest>(payload).map` | [3058](../../src/client_extensions.rs#L3058) | receiver-type-required |
| `validate_payload` | `decode::<PluginEnabledRequest>` | [3058](../../src/client_extensions.rs#L3058) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<PluginGrantsRequest>(payload).map` | [3059](../../src/client_extensions.rs#L3059) | receiver-type-required |
| `validate_payload` | `decode::<PluginGrantsRequest>` | [3059](../../src/client_extensions.rs#L3059) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<OptionalWorkspace>(payload).map` | [3060](../../src/client_extensions.rs#L3060) | receiver-type-required |
| `validate_payload` | `decode::<OptionalWorkspace>` | [3060](../../src/client_extensions.rs#L3060) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ScheduleSaveRequest>(payload).map` | [3061](../../src/client_extensions.rs#L3061) | receiver-type-required |
| `validate_payload` | `decode::<ScheduleSaveRequest>` | [3061](../../src/client_extensions.rs#L3061) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<TaskIdRequest>(payload).map` | [3062](../../src/client_extensions.rs#L3062) | receiver-type-required |
| `validate_payload` | `decode::<TaskIdRequest>` | [3062](../../src/client_extensions.rs#L3062) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<ThreadSearchRequest>(payload).map` | [3063](../../src/client_extensions.rs#L3063) | receiver-type-required |
| `validate_payload` | `decode::<ThreadSearchRequest>` | [3063](../../src/client_extensions.rs#L3063) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<SessionSearchRequest>(payload).map` | [3064](../../src/client_extensions.rs#L3064) | receiver-type-required |
| `validate_payload` | `decode::<SessionSearchRequest>` | [3064](../../src/client_extensions.rs#L3064) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `decode::<SessionIdRequest>(payload).map` | [3066](../../src/client_extensions.rs#L3066) | receiver-type-required |
| `validate_payload` | `decode::<SessionIdRequest>` | [3066](../../src/client_extensions.rs#L3066) | [tekes-supervisor::client_extensions::decode](../../src/client_extensions.rs#L1024) |
| `validate_payload` | `failure` | [3068](../../src/client_extensions.rs#L3068) | [tekes-supervisor::client_extensions::failure](../../src/client_extensions.rs#L3076) |
| `failure` | `IJsonValue::parse(         &serde_json::to_vec(&details).expect("extension failure details are JSON"),     )     .expect` | [3077](../../src/client_extensions.rs#L3077) | receiver-type-required |
| `failure` | `IJsonValue::parse` | [3077](../../src/client_extensions.rs#L3077) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `failure` | `serde_json::to_vec(&details).expect` | [3078](../../src/client_extensions.rs#L3078) | receiver-type-required |
| `failure` | `serde_json::to_vec` | [3078](../../src/client_extensions.rs#L3078) | external-constructor-callback-or-unresolved |
| `failure` | `ProductionRouteFailure::new` | [3081](../../src/client_extensions.rs#L3081) | [tekes-supervisor::endpoint_host::ProductionRouteFailure::new](../../src/endpoint_host.rs#L513) |
| `production_extensions` | `root.join` | [3098](../../src/client_extensions.rs#L3098) | receiver-type-required |
| `production_extensions` | `fs::create_dir_all(&agent).expect` | [3099](../../src/client_extensions.rs#L3099) | receiver-type-required |
| `production_extensions` | `fs::create_dir_all` | [3099](../../src/client_extensions.rs#L3099) | external-constructor-callback-or-unresolved |
| `production_extensions` | `ProductionProcessHost::open(             root,             std::env::current_exe().expect("test executable"),             "1.0.0",             &agent,         )         .expect` | [3100](../../src/client_extensions.rs#L3100) | receiver-type-required |
| `production_extensions` | `ProductionProcessHost::open` | [3100](../../src/client_extensions.rs#L3100) | external-constructor-callback-or-unresolved |
| `production_extensions` | `std::env::current_exe().expect` | [3102](../../src/client_extensions.rs#L3102) | receiver-type-required |
| `production_extensions` | `std::env::current_exe` | [3102](../../src/client_extensions.rs#L3102) | external-constructor-callback-or-unresolved |
| `production_extensions` | `Arc::new` | [3107](../../src/client_extensions.rs#L3107), [3110](../../src/client_extensions.rs#L3110), [3113](../../src/client_extensions.rs#L3113) | external-constructor-callback-or-unresolved |
| `production_extensions` | `SessionInputAdmissionAuthority::new` | [3107](../../src/client_extensions.rs#L3107) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) |
| `production_extensions` | `root.to_path_buf` | [3107](../../src/client_extensions.rs#L3107) | receiver-type-required |
| `production_extensions` | `EndpointCommandInputAuthority::new` | [3108](../../src/client_extensions.rs#L3108) | external-constructor-callback-or-unresolved |
| `production_extensions` | `Arc::clone` | [3109](../../src/client_extensions.rs#L3109), [3119](../../src/client_extensions.rs#L3119) | external-constructor-callback-or-unresolved |
| `production_extensions` | `Ok` | [3110](../../src/client_extensions.rs#L3110) | external-constructor-callback-or-unresolved |
| `production_extensions` | `"2026-08-29T00:00:00.000Z".to_owned` | [3110](../../src/client_extensions.rs#L3110) | receiver-type-required |
| `production_extensions` | `ProductionClientExtensions::open(                 root,                 ResourceCatalog::default(),                 BTreeMap::new(),                 commands,                 Arc::clone(&process),             )             .expect` | [3114](../../src/client_extensions.rs#L3114) | receiver-type-required |
| `production_extensions` | `ProductionClientExtensions::open` | [3114](../../src/client_extensions.rs#L3114) | external-constructor-callback-or-unresolved |
| `production_extensions` | `ResourceCatalog::default` | [3116](../../src/client_extensions.rs#L3116) | external-constructor-callback-or-unresolved |
| `production_extensions` | `BTreeMap::new` | [3117](../../src/client_extensions.rs#L3117) | external-constructor-callback-or-unresolved |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `tempfile::tempdir().expect` | [3128](../../src/client_extensions.rs#L3128) | receiver-type-required |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `tempfile::tempdir` | [3128](../../src/client_extensions.rs#L3128) | external-constructor-callback-or-unresolved |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `production_extensions` | [3129](../../src/client_extensions.rs#L3129) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `root.path` | [3129](../../src/client_extensions.rs#L3129) | receiver-type-required |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `execute(             &extensions,             "session.initialPresets",             &serde_json::json!({}),             "presets",         )         .expect` | [3130](../../src/client_extensions.rs#L3130) | receiver-type-required |
| `initial_presets_disclose_profile_text_without_creating_a_session` | `execute` | [3130](../../src/client_extensions.rs#L3130) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `session_presets_describe_and_resolve_without_creating_a_session` | `tempfile::tempdir().expect` | [3171](../../src/client_extensions.rs#L3171) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `tempfile::tempdir` | [3171](../../src/client_extensions.rs#L3171) | external-constructor-callback-or-unresolved |
| `session_presets_describe_and_resolve_without_creating_a_session` | `production_extensions` | [3172](../../src/client_extensions.rs#L3172) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `session_presets_describe_and_resolve_without_creating_a_session` | `root.path` | [3172](../../src/client_extensions.rs#L3172), [3182](../../src/client_extensions.rs#L3182), [3184](../../src/client_extensions.rs#L3184) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.describe",             &serde_json::json!({}),             "describe",         )         .expect` | [3173](../../src/client_extensions.rs#L3173) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute` | [3173](../../src/client_extensions.rs#L3173), [3209](../../src/client_extensions.rs#L3209), [3219](../../src/client_extensions.rs#L3219), [3234](../../src/client_extensions.rs#L3234), [3242](../../src/client_extensions.rs#L3242), [3250](../../src/client_extensions.rs#L3250) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `session_presets_describe_and_resolve_without_creating_a_session` | `root.path().join` | [3182](../../src/client_extensions.rs#L3182) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `fs::create_dir_all(&project).expect` | [3183](../../src/client_extensions.rs#L3183) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `fs::create_dir_all` | [3183](../../src/client_extensions.rs#L3183) | external-constructor-callback-or-unresolved |
| `session_presets_describe_and_resolve_without_creating_a_session` | `profile::ConfigRepository::open(root.path()).expect` | [3184](../../src/client_extensions.rs#L3184) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `profile::ConfigRepository::open` | [3184](../../src/client_extensions.rs#L3184) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `session_presets_describe_and_resolve_without_creating_a_session` | `Some` | [3187](../../src/client_extensions.rs#L3187), [3199](../../src/client_extensions.rs#L3199) | external-constructor-callback-or-unresolved |
| `session_presets_describe_and_resolve_without_creating_a_session` | `repository                 .publish_workspace(                     0,                     &profile::WorkspaceConfig {                         format: 1,                         revision: 1,                         id: id.to_owned(),                         name: id.to_owned(),                         cwd: vec![project.to_string_lossy().into_owned()],                         folders: Vec::new(),                         policy: Some(profile::WorkspacePolicy {                             base,                             ..profile::WorkspacePolicy::default()                         }),                     },                 )                 .expect` | [3189](../../src/client_extensions.rs#L3189) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `repository                 .publish_workspace` | [3189](../../src/client_extensions.rs#L3189) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `id.to_owned` | [3195](../../src/client_extensions.rs#L3195), [3196](../../src/client_extensions.rs#L3196) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `Vec::new` | [3198](../../src/client_extensions.rs#L3198) | external-constructor-callback-or-unresolved |
| `session_presets_describe_and_resolve_without_creating_a_session` | `profile::WorkspacePolicy::default` | [3201](../../src/client_extensions.rs#L3201) | external-constructor-callback-or-unresolved |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.resolve",             &serde_json::json!({"workspaceId":"ws-general","preset":preset}),             "resolve-general",         )         .expect` | [3209](../../src/client_extensions.rs#L3209) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `general["presetText"].as_str().unwrap` | [3216](../../src/client_extensions.rs#L3216) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `general["presetText"].as_str` | [3216](../../src/client_extensions.rs#L3216) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.resolve",             &serde_json::json!({"workspaceId":"ws-blank","preset":preset}),             "resolve-blank",         )         .expect` | [3219](../../src/client_extensions.rs#L3219) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.resolve",             &serde_json::json!({"workspaceId":"ws-general","preset":{"tools":["report"]}}),             "resolve-role-tool",         )         .expect_err` | [3234](../../src/client_extensions.rs#L3234) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.resolve",             &serde_json::json!({"workspaceId":"ws-general","preset":{"tools":["read"]}}),             "resolve-tools",         )         .expect` | [3242](../../src/client_extensions.rs#L3242) | receiver-type-required |
| `session_presets_describe_and_resolve_without_creating_a_session` | `execute(             &extensions,             "session.presets.resolve",             &serde_json::json!({"workspaceId":"absent","preset":{}}),             "resolve-missing",         )         .expect_err` | [3250](../../src/client_extensions.rs#L3250) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `tempfile::tempdir().expect` | [3269](../../src/client_extensions.rs#L3269) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `tempfile::tempdir` | [3269](../../src/client_extensions.rs#L3269) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `production_extensions` | [3270](../../src/client_extensions.rs#L3270) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `session_preset_method_cases_are_the_runtime_results` | `root.path` | [3270](../../src/client_extensions.rs#L3270), [3271](../../src/client_extensions.rs#L3271), [3273](../../src/client_extensions.rs#L3273) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `root.path().join` | [3271](../../src/client_extensions.rs#L3271) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `fs::create_dir_all(&project).expect` | [3272](../../src/client_extensions.rs#L3272) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `fs::create_dir_all` | [3272](../../src/client_extensions.rs#L3272) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `profile::ConfigRepository::open(root.path())             .expect("repository")             .publish_workspace(                 0,                 &profile::WorkspaceConfig {                     format: 1,                     revision: 1,                     id: "workspace-1".to_owned(),                     name: "workspace-1".to_owned(),                     cwd: vec![project.to_string_lossy().into_owned()],                     folders: Vec::new(),                     policy: Some(profile::WorkspacePolicy::default()),                 },             )             .expect` | [3273](../../src/client_extensions.rs#L3273) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `profile::ConfigRepository::open(root.path())             .expect("repository")             .publish_workspace` | [3273](../../src/client_extensions.rs#L3273) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `profile::ConfigRepository::open(root.path())             .expect` | [3273](../../src/client_extensions.rs#L3273) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `profile::ConfigRepository::open` | [3273](../../src/client_extensions.rs#L3273) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `session_preset_method_cases_are_the_runtime_results` | `"workspace-1".to_owned` | [3280](../../src/client_extensions.rs#L3280), [3281](../../src/client_extensions.rs#L3281) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `Vec::new` | [3283](../../src/client_extensions.rs#L3283) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `Some` | [3284](../../src/client_extensions.rs#L3284) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `profile::WorkspacePolicy::default` | [3284](../../src/client_extensions.rs#L3284) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `serde_json::from_slice(             &fs::read(                 Path::new(env!("CARGO_MANIFEST_DIR"))                     .join("../../fixtures/client-extensions/method-cases.canonical.json"),             )             .expect("method cases"),         )         .expect` | [3288](../../src/client_extensions.rs#L3288) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `serde_json::from_slice` | [3288](../../src/client_extensions.rs#L3288) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `fs::read(                 Path::new(env!("CARGO_MANIFEST_DIR"))                     .join("../../fixtures/client-extensions/method-cases.canonical.json"),             )             .expect` | [3289](../../src/client_extensions.rs#L3289) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `fs::read` | [3289](../../src/client_extensions.rs#L3289) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `Path::new(env!("CARGO_MANIFEST_DIR"))                     .join` | [3290](../../src/client_extensions.rs#L3290) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `Path::new` | [3290](../../src/client_extensions.rs#L3290) | external-constructor-callback-or-unresolved |
| `session_preset_method_cases_are_the_runtime_results` | `cases["cases"]                 .as_array()                 .unwrap()                 .iter()                 .find(&#124;case&#124; case["method"] == method)                 .expect` | [3297](../../src/client_extensions.rs#L3297) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `cases["cases"]                 .as_array()                 .unwrap()                 .iter()                 .find` | [3297](../../src/client_extensions.rs#L3297) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `cases["cases"]                 .as_array()                 .unwrap()                 .iter` | [3297](../../src/client_extensions.rs#L3297) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `cases["cases"]                 .as_array()                 .unwrap` | [3297](../../src/client_extensions.rs#L3297) | receiver-type-required |
| `session_preset_method_cases_are_the_runtime_results` | `cases["cases"]                 .as_array` | [3297](../../src/client_extensions.rs#L3297) | receiver-type-required |
| `write_plugin` | `root.join` | [3312](../../src/client_extensions.rs#L3312) | receiver-type-required |
| `write_plugin` | `fs::create_dir_all(&package).expect` | [3313](../../src/client_extensions.rs#L3313) | receiver-type-required |
| `write_plugin` | `fs::create_dir_all` | [3313](../../src/client_extensions.rs#L3313) | external-constructor-callback-or-unresolved |
| `write_plugin` | `capabilities             .iter()             .map(&#124;id&#124; serde_json::json!({"id":id}))             .collect::<Vec<_>>` | [3326](../../src/client_extensions.rs#L3326) | receiver-type-required |
| `write_plugin` | `capabilities             .iter()             .map` | [3326](../../src/client_extensions.rs#L3326) | receiver-type-required |
| `write_plugin` | `capabilities             .iter` | [3326](../../src/client_extensions.rs#L3326) | receiver-type-required |
| `write_plugin` | `fs::create_dir(package.join("assets")).expect` | [3330](../../src/client_extensions.rs#L3330) | receiver-type-required |
| `write_plugin` | `fs::create_dir` | [3330](../../src/client_extensions.rs#L3330) | external-constructor-callback-or-unresolved |
| `write_plugin` | `package.join` | [3330](../../src/client_extensions.rs#L3330), [3332](../../src/client_extensions.rs#L3332) | receiver-type-required |
| `write_plugin` | `fs::write(             package.join("tekes-plugin.json"),             serde_json::to_vec(&serde_json::json!({                 "manifestVersion":1,                 "id":"com.example.journal",                 "version":version,                 "displayName":"Journal fixture",                 "platforms":[{"os":operating_system,"architectures":[architecture]}],                 "capabilities":capabilities,                 "components":[{"id":"assets","type":"assets","path":"assets","capabilities":[]}]             }))             .expect("manifest JSON"),         )         .expect` | [3331](../../src/client_extensions.rs#L3331) | receiver-type-required |
| `write_plugin` | `fs::write` | [3331](../../src/client_extensions.rs#L3331) | external-constructor-callback-or-unresolved |
| `write_plugin` | `serde_json::to_vec(&serde_json::json!({                 "manifestVersion":1,                 "id":"com.example.journal",                 "version":version,                 "displayName":"Journal fixture",                 "platforms":[{"os":operating_system,"architectures":[architecture]}],                 "capabilities":capabilities,                 "components":[{"id":"assets","type":"assets","path":"assets","capabilities":[]}]             }))             .expect` | [3333](../../src/client_extensions.rs#L3333) | receiver-type-required |
| `write_plugin` | `serde_json::to_vec` | [3333](../../src/client_extensions.rs#L3333) | external-constructor-callback-or-unresolved |
| `package_digest` | `PluginSource::from_path(package).expect` | [3349](../../src/client_extensions.rs#L3349) | receiver-type-required |
| `package_digest` | `PluginSource::from_path` | [3349](../../src/client_extensions.rs#L3349) | external-constructor-callback-or-unresolved |
| `package_digest` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .inspect(&source)             .expect` | [3350](../../src/client_extensions.rs#L3350) | receiver-type-required |
| `package_digest` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .inspect` | [3350](../../src/client_extensions.rs#L3350) | receiver-type-required |
| `package_digest` | `lock(extensions.plugin_store().expect("plugin store"))             .expect` | [3350](../../src/client_extensions.rs#L3350) | receiver-type-required |
| `package_digest` | `lock` | [3350](../../src/client_extensions.rs#L3350) | external-constructor-callback-or-unresolved |
| `package_digest` | `extensions.plugin_store().expect` | [3350](../../src/client_extensions.rs#L3350) | receiver-type-required |
| `package_digest` | `extensions.plugin_store` | [3350](../../src/client_extensions.rs#L3350) | receiver-type-required |
| `execute` | `ClientExtensionAuthority::execute` | [3374](../../src/client_extensions.rs#L3374) | external-constructor-callback-or-unresolved |
| `execute` | `ijson_value` | [3382](../../src/client_extensions.rs#L3382) | external-constructor-callback-or-unresolved |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `tempfile::tempdir().expect` | [3391](../../src/client_extensions.rs#L3391) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `tempfile::tempdir` | [3391](../../src/client_extensions.rs#L3391) | external-constructor-callback-or-unresolved |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `production_extensions` | [3392](../../src/client_extensions.rs#L3392) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `root.path` | [3392](../../src/client_extensions.rs#L3392), [3395](../../src/client_extensions.rs#L3395), [3397](../../src/client_extensions.rs#L3397) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `root.path().join("threads").join` | [3395](../../src/client_extensions.rs#L3395) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `root.path().join` | [3395](../../src/client_extensions.rs#L3395), [3397](../../src/client_extensions.rs#L3397) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `fs::create_dir_all(&folder).expect` | [3396](../../src/client_extensions.rs#L3396) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `fs::create_dir_all` | [3396](../../src/client_extensions.rs#L3396), [3397](../../src/client_extensions.rs#L3397) | external-constructor-callback-or-unresolved |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `fs::create_dir_all(root.path().join("archive").join(archived)).expect` | [3397](../../src/client_extensions.rs#L3397) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `root.path().join("archive").join` | [3397](../../src/client_extensions.rs#L3397) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.policy",             &serde_json::json!({}),             "policy",         )         .expect` | [3399](../../src/client_extensions.rs#L3399) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute` | [3399](../../src/client_extensions.rs#L3399), [3421](../../src/client_extensions.rs#L3421), [3430](../../src/client_extensions.rs#L3430), [3442](../../src/client_extensions.rs#L3442), [3451](../../src/client_extensions.rs#L3451), [3459](../../src/client_extensions.rs#L3459), [3475](../../src/client_extensions.rs#L3475) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `policy["threadLevel"]             .as_array()             .expect("thread options")             .iter()             .map(&#124;option&#124; option["value"].as_str().expect("value"))             .collect` | [3406](../../src/client_extensions.rs#L3406) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `policy["threadLevel"]             .as_array()             .expect("thread options")             .iter()             .map` | [3406](../../src/client_extensions.rs#L3406) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `policy["threadLevel"]             .as_array()             .expect("thread options")             .iter` | [3406](../../src/client_extensions.rs#L3406) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `policy["threadLevel"]             .as_array()             .expect` | [3406](../../src/client_extensions.rs#L3406) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `policy["threadLevel"]             .as_array` | [3406](../../src/client_extensions.rs#L3406) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `option["value"].as_str().expect` | [3410](../../src/client_extensions.rs#L3410) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `option["value"].as_str` | [3410](../../src/client_extensions.rs#L3410) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.mode",             &serde_json::json!({"sessionId":session}),             "mode-0",         )         .expect` | [3421](../../src/client_extensions.rs#L3421) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.select",             &serde_json::json!({"sessionId":session,"mode":"read-only"}),             "select-1",         )         .expect` | [3430](../../src/client_extensions.rs#L3430) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.mode",             &serde_json::json!({"sessionId":session}),             "mode-1",         )         .expect` | [3442](../../src/client_extensions.rs#L3442) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.select",             &serde_json::json!({"sessionId":session,"mode":"yolo"}),             "select-2",         )         .expect_err` | [3451](../../src/client_extensions.rs#L3451) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.select",             &serde_json::json!({"sessionId":archived,"mode":"read-only"}),             "select-3",         )         .expect_err` | [3459](../../src/client_extensions.rs#L3459) | receiver-type-required |
| `approvals_select_persists_the_session_mode_and_refuses_archived_sessions` | `execute(             &extensions,             "approvals.select",             &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033","mode":"read-only"}),             "select-4",         )         .expect_err` | [3475](../../src/client_extensions.rs#L3475) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `tempfile::tempdir().expect` | [3504](../../src/client_extensions.rs#L3504) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `tempfile::tempdir` | [3504](../../src/client_extensions.rs#L3504) | external-constructor-callback-or-unresolved |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `serde_json_canonicalizer::to_vec(&workspace).expect` | [3511](../../src/client_extensions.rs#L3511) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `serde_json_canonicalizer::to_vec` | [3511](../../src/client_extensions.rs#L3511) | external-constructor-callback-or-unresolved |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `workspace_bytes.push` | [3512](../../src/client_extensions.rs#L3512) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `fs::create_dir_all(root.path().join("workspaces/ws")).expect` | [3513](../../src/client_extensions.rs#L3513) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `fs::create_dir_all` | [3513](../../src/client_extensions.rs#L3513) | external-constructor-callback-or-unresolved |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `root.path().join` | [3513](../../src/client_extensions.rs#L3513), [3515](../../src/client_extensions.rs#L3515) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `root.path` | [3513](../../src/client_extensions.rs#L3513), [3515](../../src/client_extensions.rs#L3515), [3519](../../src/client_extensions.rs#L3519) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `fs::write(             root.path().join("workspaces/ws/workspace.json"),             workspace_bytes,         )         .expect` | [3514](../../src/client_extensions.rs#L3514) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `fs::write` | [3514](../../src/client_extensions.rs#L3514) | external-constructor-callback-or-unresolved |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `production_extensions` | [3519](../../src/client_extensions.rs#L3519) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `execute(             &extensions,             "tools/list",             &serde_json::json!({"workspaceId":"ws"}),             "rpc-tools-list",         )         .expect` | [3520](../../src/client_extensions.rs#L3520) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `execute` | [3520](../../src/client_extensions.rs#L3520), [3534](../../src/client_extensions.rs#L3534) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `result["tools"].as_array().expect` | [3527](../../src/client_extensions.rs#L3527) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `result["tools"].as_array` | [3527](../../src/client_extensions.rs#L3527) | receiver-type-required |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `BuiltinManifest::compiled` | [3528](../../src/client_extensions.rs#L3528) | external-constructor-callback-or-unresolved |
| `tools_list_is_closed_and_includes_every_fixed_tool` | `execute(             &extensions,             "tools/resolve",             &serde_json::json!({"workspaceId":"ws","name":"web_search"}),             "rpc-tools-resolve",         )         .expect` | [3534](../../src/client_extensions.rs#L3534) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `tempfile::tempdir().expect` | [3546](../../src/client_extensions.rs#L3546) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `tempfile::tempdir` | [3546](../../src/client_extensions.rs#L3546) | external-constructor-callback-or-unresolved |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `root.path().join` | [3547](../../src/client_extensions.rs#L3547), [3548](../../src/client_extensions.rs#L3548), [3551](../../src/client_extensions.rs#L3551), [3561](../../src/client_extensions.rs#L3561) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `root.path` | [3547](../../src/client_extensions.rs#L3547), [3548](../../src/client_extensions.rs#L3548), [3551](../../src/client_extensions.rs#L3551), [3561](../../src/client_extensions.rs#L3561), [3566](../../src/client_extensions.rs#L3566) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::create_dir_all(&first).expect` | [3549](../../src/client_extensions.rs#L3549) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::create_dir_all` | [3549](../../src/client_extensions.rs#L3549), [3550](../../src/client_extensions.rs#L3550), [3551](../../src/client_extensions.rs#L3551) | external-constructor-callback-or-unresolved |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::create_dir_all(&second).expect` | [3550](../../src/client_extensions.rs#L3550) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::create_dir_all(root.path().join("workspaces/multi")).expect` | [3551](../../src/client_extensions.rs#L3551) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `serde_json_canonicalizer::to_vec(&workspace).expect` | [3558](../../src/client_extensions.rs#L3558) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `serde_json_canonicalizer::to_vec` | [3558](../../src/client_extensions.rs#L3558) | external-constructor-callback-or-unresolved |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `workspace_bytes.push` | [3559](../../src/client_extensions.rs#L3559) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::write(             root.path().join("workspaces/multi/workspace.json"),             workspace_bytes,         )         .expect` | [3560](../../src/client_extensions.rs#L3560) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `fs::write` | [3560](../../src/client_extensions.rs#L3560) | external-constructor-callback-or-unresolved |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `production_extensions` | [3566](../../src/client_extensions.rs#L3566) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `execute(             &extensions,             "schedule.save",             &serde_json::json!({"definition":{                 "id":"018f0000-0000-7000-8000-000000000011",                 "name":"Daily","workspace_id":"multi","cron":"0 9 * * 1-5",                 "time_zone":"Asia/Shanghai","prompt":"review",                 "permission_mode":"inherit","enabled":false,                 "missed_policy":"skip_and_record"             }}),             "rpc-schedule-multi-root",         )         .expect` | [3567](../../src/client_extensions.rs#L3567) | receiver-type-required |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `execute` | [3567](../../src/client_extensions.rs#L3567) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `schedule_save_accepts_a_valid_multi_root_workspace` | `process.shutdown` | [3581](../../src/client_extensions.rs#L3581) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `tempfile::tempdir().expect` | [3586](../../src/client_extensions.rs#L3586) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `tempfile::tempdir` | [3586](../../src/client_extensions.rs#L3586) | external-constructor-callback-or-unresolved |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `production_extensions` | [3587](../../src/client_extensions.rs#L3587) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `root.path` | [3587](../../src/client_extensions.rs#L3587), [3588](../../src/client_extensions.rs#L3588) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `write_plugin` | [3588](../../src/client_extensions.rs#L3588) | [tekes-supervisor::client_extensions::tests::write_plugin](../../src/client_extensions.rs#L3311) |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `package_digest` | [3589](../../src/client_extensions.rs#L3589) | [tekes-supervisor::client_extensions::tests::package_digest](../../src/client_extensions.rs#L3348) |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute(             &extensions,             "plugin/install",             &install_payload(&v1, &digest, false),             "install-first",         )         .expect` | [3590](../../src/client_extensions.rs#L3590) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute` | [3590](../../src/client_extensions.rs#L3590), [3597](../../src/client_extensions.rs#L3597), [3616](../../src/client_extensions.rs#L3616), [3623](../../src/client_extensions.rs#L3623), [3626](../../src/client_extensions.rs#L3626) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `install_payload` | [3593](../../src/client_extensions.rs#L3593) | [tekes-supervisor::client_extensions::tests::install_payload](../../src/client_extensions.rs#L3357) |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute(             &extensions,             "plugin/setEnabled",             &serde_json::json!({"pluginId":"com.example.journal","enabled":true}),             "enable-initial",         )         .expect` | [3597](../../src/client_extensions.rs#L3597) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `extensions             .plugin_operation("plugin/setEnabled", &payload_a, "enable-a")             .expect` | [3606](../../src/client_extensions.rs#L3606) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `extensions             .plugin_operation` | [3606](../../src/client_extensions.rs#L3606) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `operation.prepare(None).expect` | [3609](../../src/client_extensions.rs#L3609) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `operation.prepare` | [3609](../../src/client_extensions.rs#L3609) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .set_enabled("com.example.journal", false)             .expect` | [3610](../../src/client_extensions.rs#L3610) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .set_enabled` | [3610](../../src/client_extensions.rs#L3610) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `lock(extensions.plugin_store().expect("plugin store"))             .expect` | [3610](../../src/client_extensions.rs#L3610) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `lock` | [3610](../../src/client_extensions.rs#L3610) | external-constructor-callback-or-unresolved |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `extensions.plugin_store().expect` | [3610](../../src/client_extensions.rs#L3610) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `extensions.plugin_store` | [3610](../../src/client_extensions.rs#L3610) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `drop` | [3614](../../src/client_extensions.rs#L3614) | external-constructor-callback-or-unresolved |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute(             &extensions,             "plugin/setEnabled",             &serde_json::json!({"pluginId":"com.example.journal","enabled":true}),             "enable-b",         )         .expect` | [3616](../../src/client_extensions.rs#L3616) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute(&extensions, "plugin/setEnabled", &payload_a, "enable-a")             .expect` | [3623](../../src/client_extensions.rs#L3623) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `execute(&extensions, "plugin/list", &serde_json::json!({}), "list").expect` | [3626](../../src/client_extensions.rs#L3626) | receiver-type-required |
| `plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b` | `process.shutdown` | [3628](../../src/client_extensions.rs#L3628) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `tempfile::tempdir().expect` | [3633](../../src/client_extensions.rs#L3633) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `tempfile::tempdir` | [3633](../../src/client_extensions.rs#L3633) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `production_extensions` | [3634](../../src/client_extensions.rs#L3634) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `root.path` | [3634](../../src/client_extensions.rs#L3634), [3635](../../src/client_extensions.rs#L3635), [3671](../../src/client_extensions.rs#L3671), [3690](../../src/client_extensions.rs#L3690) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `write_plugin` | [3635](../../src/client_extensions.rs#L3635), [3671](../../src/client_extensions.rs#L3671), [3690](../../src/client_extensions.rs#L3690) | [tekes-supervisor::client_extensions::tests::write_plugin](../../src/client_extensions.rs#L3311) |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `package_digest` | [3636](../../src/client_extensions.rs#L3636), [3672](../../src/client_extensions.rs#L3672), [3691](../../src/client_extensions.rs#L3691) | [tekes-supervisor::client_extensions::tests::package_digest](../../src/client_extensions.rs#L3348) |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute(             &extensions,             "plugin/install",             &install_payload(&v1, &digest_v1, false),             "install-first",         )         .expect` | [3637](../../src/client_extensions.rs#L3637) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute` | [3637](../../src/client_extensions.rs#L3637), [3656](../../src/client_extensions.rs#L3656), [3663](../../src/client_extensions.rs#L3663), [3692](../../src/client_extensions.rs#L3692), [3699](../../src/client_extensions.rs#L3699) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `install_payload` | [3640](../../src/client_extensions.rs#L3640), [3659](../../src/client_extensions.rs#L3659), [3673](../../src/client_extensions.rs#L3673), [3695](../../src/client_extensions.rs#L3695) | [tekes-supervisor::client_extensions::tests::install_payload](../../src/client_extensions.rs#L3357) |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `extensions             .plugin_operation("plugin/remove", &remove, "remove-a")             .expect` | [3646](../../src/client_extensions.rs#L3646) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `extensions             .plugin_operation` | [3646](../../src/client_extensions.rs#L3646), [3674](../../src/client_extensions.rs#L3674) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `operation.prepare(Some(true)).expect` | [3649](../../src/client_extensions.rs#L3649) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `operation.prepare` | [3649](../../src/client_extensions.rs#L3649), [3682](../../src/client_extensions.rs#L3682) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `Some` | [3649](../../src/client_extensions.rs#L3649) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .remove("com.example.journal")             .expect` | [3650](../../src/client_extensions.rs#L3650) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `lock(extensions.plugin_store().expect("plugin store"))             .expect("lock")             .remove` | [3650](../../src/client_extensions.rs#L3650) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `lock(extensions.plugin_store().expect("plugin store"))             .expect` | [3650](../../src/client_extensions.rs#L3650) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `lock` | [3650](../../src/client_extensions.rs#L3650), [3678](../../src/client_extensions.rs#L3678) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `extensions.plugin_store().expect` | [3650](../../src/client_extensions.rs#L3650), [3678](../../src/client_extensions.rs#L3678) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `extensions.plugin_store` | [3650](../../src/client_extensions.rs#L3650), [3678](../../src/client_extensions.rs#L3678) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `drop` | [3654](../../src/client_extensions.rs#L3654), [3687](../../src/client_extensions.rs#L3687), [3688](../../src/client_extensions.rs#L3688) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute(             &extensions,             "plugin/install",             &install_payload(&v1, &digest_v1, false),             "reinstall-b",         )         .expect` | [3656](../../src/client_extensions.rs#L3656) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute(&extensions, "plugin/remove", &remove, "remove-a")             .expect` | [3663](../../src/client_extensions.rs#L3663) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `extensions             .plugin_operation("plugin/install", &payload_v2, "update-a")             .expect` | [3674](../../src/client_extensions.rs#L3674) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `PluginSource::from_path(&v2).expect` | [3677](../../src/client_extensions.rs#L3677) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `PluginSource::from_path` | [3677](../../src/client_extensions.rs#L3677), [3683](../../src/client_extensions.rs#L3683) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `lock(extensions.plugin_store().expect("plugin store")).expect` | [3678](../../src/client_extensions.rs#L3678) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `store             .freeze_source(&source, &operation.stage)             .expect` | [3679](../../src/client_extensions.rs#L3679) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `store             .freeze_source` | [3679](../../src/client_extensions.rs#L3679) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `operation.prepare(None).expect` | [3682](../../src/client_extensions.rs#L3682) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `PluginSource::from_path(&operation.stage).expect` | [3683](../../src/client_extensions.rs#L3683) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `store             .install(&staged, InstallOptions::default())             .expect` | [3684](../../src/client_extensions.rs#L3684) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `store             .install` | [3684](../../src/client_extensions.rs#L3684) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `InstallOptions::default` | [3685](../../src/client_extensions.rs#L3685) | external-constructor-callback-or-unresolved |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute(             &extensions,             "plugin/install",             &install_payload(&v3, &digest_v3, false),             "update-b",         )         .expect` | [3692](../../src/client_extensions.rs#L3692) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `execute(&extensions, "plugin/install", &payload_v2, "update-a")             .expect` | [3699](../../src/client_extensions.rs#L3699) | receiver-type-required |
| `plugin_journal_orders_remove_reinstall_and_install_update` | `process.shutdown` | [3706](../../src/client_extensions.rs#L3706) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `tempfile::tempdir().expect` | [3711](../../src/client_extensions.rs#L3711) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `tempfile::tempdir` | [3711](../../src/client_extensions.rs#L3711) | external-constructor-callback-or-unresolved |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `production_extensions` | [3712](../../src/client_extensions.rs#L3712) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `root.path` | [3712](../../src/client_extensions.rs#L3712), [3729](../../src/client_extensions.rs#L3729), [3754](../../src/client_extensions.rs#L3754) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/setEnabled",             &serde_json::json!({"pluginId":"com.example.missing","enabled":true}),             "missing-enable",         )         .expect_err` | [3713](../../src/client_extensions.rs#L3713) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute` | [3713](../../src/client_extensions.rs#L3713), [3721](../../src/client_extensions.rs#L3721), [3731](../../src/client_extensions.rs#L3731), [3738](../../src/client_extensions.rs#L3738), [3746](../../src/client_extensions.rs#L3746), [3756](../../src/client_extensions.rs#L3756), [3764](../../src/client_extensions.rs#L3764), [3773](../../src/client_extensions.rs#L3773) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/list",             &serde_json::json!({}),             "list-after-missing",         )         .expect` | [3721](../../src/client_extensions.rs#L3721) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `write_plugin` | [3729](../../src/client_extensions.rs#L3729), [3754](../../src/client_extensions.rs#L3754) | [tekes-supervisor::client_extensions::tests::write_plugin](../../src/client_extensions.rs#L3311) |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `package_digest` | [3730](../../src/client_extensions.rs#L3730), [3755](../../src/client_extensions.rs#L3755) | [tekes-supervisor::client_extensions::tests::package_digest](../../src/client_extensions.rs#L3348) |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/install",             &install_payload(&v2, &digest_v2, false),             "install-v2",         )         .expect` | [3731](../../src/client_extensions.rs#L3731) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `install_payload` | [3734](../../src/client_extensions.rs#L3734), [3759](../../src/client_extensions.rs#L3759), [3776](../../src/client_extensions.rs#L3776) | [tekes-supervisor::client_extensions::tests::install_payload](../../src/client_extensions.rs#L3357) |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/setGrants",             &serde_json::json!({"pluginId":"com.example.journal","grants":["not.requested"]}),             "bad-grants",         )         .expect_err` | [3738](../../src/client_extensions.rs#L3738) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/setEnabled",             &serde_json::json!({"pluginId":"com.example.journal","enabled":false}),             "valid-after-grants",         )         .expect` | [3746](../../src/client_extensions.rs#L3746) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/install",             &install_payload(&v1, &digest_v1, false),             "bad-downgrade",         )         .expect_err` | [3756](../../src/client_extensions.rs#L3756) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/list",             &serde_json::json!({}),             "list-after-downgrade",         )         .expect` | [3764](../../src/client_extensions.rs#L3764) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `fs::write(v1.join("extra"), b"changed").expect` | [3772](../../src/client_extensions.rs#L3772) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `fs::write` | [3772](../../src/client_extensions.rs#L3772) | external-constructor-callback-or-unresolved |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `v1.join` | [3772](../../src/client_extensions.rs#L3772) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `execute(             &extensions,             "plugin/install",             &install_payload(&v1, &digest_v1, false),             "wrong-source",         )         .expect_err` | [3773](../../src/client_extensions.rs#L3773) | receiver-type-required |
| `plugin_precommit_failures_are_terminal_and_do_not_wedge_management` | `process.shutdown` | [3781](../../src/client_extensions.rs#L3781) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `tempfile::tempdir().expect` | [3786](../../src/client_extensions.rs#L3786) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `tempfile::tempdir` | [3786](../../src/client_extensions.rs#L3786) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `root.path().join` | [3787](../../src/client_extensions.rs#L3787), [3793](../../src/client_extensions.rs#L3793) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `root.path` | [3787](../../src/client_extensions.rs#L3787), [3789](../../src/client_extensions.rs#L3789), [3793](../../src/client_extensions.rs#L3793), [3831](../../src/client_extensions.rs#L3831), [3838](../../src/client_extensions.rs#L3838), [3841](../../src/client_extensions.rs#L3841), [3845](../../src/client_extensions.rs#L3845), [3862](../../src/client_extensions.rs#L3862), [3866](../../src/client_extensions.rs#L3866) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `fs::create_dir_all(&agent).expect` | [3788](../../src/client_extensions.rs#L3788) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `fs::create_dir_all` | [3788](../../src/client_extensions.rs#L3788), [3795](../../src/client_extensions.rs#L3795) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::ConfigRepository::open(root.path()).expect` | [3789](../../src/client_extensions.rs#L3789) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::ConfigRepository::open` | [3789](../../src/client_extensions.rs#L3789) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L688) |
| `skill_resources_never_cross_configured_workspace_catalogs` | `BTreeMap::new` | [3790](../../src/client_extensions.rs#L3790) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `project.join` | [3794](../../src/client_extensions.rs#L3794) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `fs::create_dir_all(&package).expect` | [3795](../../src/client_extensions.rs#L3795) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `fs::write(                 package.join("SKILL.md"),                 format!(                     "---\nname: {skill}\ndescription: {id} skill\n---\nresource owned by {id}\n"                 ),             )             .expect` | [3796](../../src/client_extensions.rs#L3796) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `fs::write` | [3796](../../src/client_extensions.rs#L3796) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `package.join` | [3797](../../src/client_extensions.rs#L3797) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `repository                 .publish_workspace(                     0,                     &profile::WorkspaceConfig {                         format: 1,                         revision: 1,                         id: id.to_owned(),                         name: id.to_owned(),                         cwd: vec![project.to_string_lossy().into_owned()],                         folders: Vec::new(),                         policy: Some(profile::WorkspacePolicy::default()),                     },                 )                 .expect` | [3803](../../src/client_extensions.rs#L3803) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `repository                 .publish_workspace` | [3803](../../src/client_extensions.rs#L3803) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `id.to_owned` | [3809](../../src/client_extensions.rs#L3809), [3810](../../src/client_extensions.rs#L3810) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `Vec::new` | [3812](../../src/client_extensions.rs#L3812) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `Some` | [3813](../../src/client_extensions.rs#L3813) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::WorkspacePolicy::default` | [3813](../../src/client_extensions.rs#L3813) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `repository             .resource_workspace_roots()             .expect` | [3818](../../src/client_extensions.rs#L3818) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `repository             .resource_workspace_roots` | [3818](../../src/client_extensions.rs#L3818) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::InstructionResolver::new(&agent, &roots)                 .capture()                 .expect` | [3822](../../src/client_extensions.rs#L3822) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::InstructionResolver::new(&agent, &roots)                 .capture` | [3822](../../src/client_extensions.rs#L3822) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `profile::InstructionResolver::new` | [3822](../../src/client_extensions.rs#L3822) | [profile::instruction::InstructionResolver::new](../../../profile/src/instruction.rs#L285) |
| `skill_resources_never_cross_configured_workspace_catalogs` | `catalogs.insert` | [3825](../../src/client_extensions.rs#L3825) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ResourceCatalog::from_snapshot(&snapshot).expect` | [3827](../../src/client_extensions.rs#L3827) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ResourceCatalog::from_snapshot` | [3827](../../src/client_extensions.rs#L3827) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ProductionProcessHost::open(             root.path(),             std::env::current_exe().expect("test executable"),             "1.0.0",             &agent,         )         .expect` | [3830](../../src/client_extensions.rs#L3830) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ProductionProcessHost::open` | [3830](../../src/client_extensions.rs#L3830) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `std::env::current_exe().expect` | [3832](../../src/client_extensions.rs#L3832) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `std::env::current_exe` | [3832](../../src/client_extensions.rs#L3832) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `assemble_application_endpoint_host(             root.path(),             SessionHostDescription {                 version: "1.0.0".to_owned(),                 cwd: root.path().display().to_string(),                 provider: None,                 model: None,                 attached_sessions: 0,                 home: root.path().display().to_string(),                 can_open_path: false,             },             Arc::new(&#124;&#124; Ok("2026-08-29T00:00:00.000Z".to_owned())),             &agent,             Arc::clone(&process),         )         .expect` | [3837](../../src/client_extensions.rs#L3837) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `assemble_application_endpoint_host` | [3837](../../src/client_extensions.rs#L3837) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L312) |
| `skill_resources_never_cross_configured_workspace_catalogs` | `"1.0.0".to_owned` | [3840](../../src/client_extensions.rs#L3840) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `root.path().display().to_string` | [3841](../../src/client_extensions.rs#L3841), [3845](../../src/client_extensions.rs#L3845) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `root.path().display` | [3841](../../src/client_extensions.rs#L3841), [3845](../../src/client_extensions.rs#L3845) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `Arc::new` | [3848](../../src/client_extensions.rs#L3848), [3860](../../src/client_extensions.rs#L3860), [3861](../../src/client_extensions.rs#L3861) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `Ok` | [3848](../../src/client_extensions.rs#L3848), [3860](../../src/client_extensions.rs#L3860) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `"2026-08-29T00:00:00.000Z".to_owned` | [3848](../../src/client_extensions.rs#L3848), [3860](../../src/client_extensions.rs#L3860) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `Arc::clone` | [3850](../../src/client_extensions.rs#L3850), [3859](../../src/client_extensions.rs#L3859), [3870](../../src/client_extensions.rs#L3870) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `EndpointCommandInputAuthority::new` | [3858](../../src/client_extensions.rs#L3858) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `SessionInputAdmissionAuthority::new` | [3861](../../src/client_extensions.rs#L3861) | [tekes-supervisor::endpoint_host::SessionInputAdmissionAuthority::new](../../src/endpoint_host.rs#L286) |
| `skill_resources_never_cross_configured_workspace_catalogs` | `root.path().to_path_buf` | [3862](../../src/client_extensions.rs#L3862) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ProductionClientExtensions::open(             root.path(),             ResourceCatalog::default(),             catalogs,             commands,             Arc::clone(&process),         )         .expect` | [3865](../../src/client_extensions.rs#L3865) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ProductionClientExtensions::open` | [3865](../../src/client_extensions.rs#L3865) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `ResourceCatalog::default` | [3867](../../src/client_extensions.rs#L3867) | external-constructor-callback-or-unresolved |
| `skill_resources_never_cross_configured_workspace_catalogs` | `execute(             &extensions,             "resources/list",             &serde_json::json!({"workspaceId":"workspace-a","source":"skill","limit":100}),             "resources-a",         )         .expect` | [3873](../../src/client_extensions.rs#L3873) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `execute` | [3873](../../src/client_extensions.rs#L3873), [3880](../../src/client_extensions.rs#L3880) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `skill_resources_never_cross_configured_workspace_catalogs` | `execute(             &extensions,             "resources/list",             &serde_json::json!({"workspaceId":"workspace-b","source":"skill","limit":100}),             "resources-b",         )         .expect` | [3880](../../src/client_extensions.rs#L3880) | receiver-type-required |
| `skill_resources_never_cross_configured_workspace_catalogs` | `process.shutdown` | [3893](../../src/client_extensions.rs#L3893) | receiver-type-required |
| `append_canonical` | `lines.extend` | [3897](../../src/client_extensions.rs#L3897) | receiver-type-required |
| `append_canonical` | `serde_json_canonicalizer::to_vec(&value).expect` | [3897](../../src/client_extensions.rs#L3897) | receiver-type-required |
| `append_canonical` | `serde_json_canonicalizer::to_vec` | [3897](../../src/client_extensions.rs#L3897) | external-constructor-callback-or-unresolved |
| `append_canonical` | `lines.push` | [3898](../../src/client_extensions.rs#L3898) | receiver-type-required |
| `epoch_profile_asset` | `assets             .publish(&serde_json_canonicalizer::to_vec(&value).expect("profile bytes"))             .expect` | [3924](../../src/client_extensions.rs#L3924) | receiver-type-required |
| `epoch_profile_asset` | `assets             .publish` | [3924](../../src/client_extensions.rs#L3924) | receiver-type-required |
| `epoch_profile_asset` | `serde_json_canonicalizer::to_vec(&value).expect` | [3925](../../src/client_extensions.rs#L3925) | receiver-type-required |
| `epoch_profile_asset` | `serde_json_canonicalizer::to_vec` | [3925](../../src/client_extensions.rs#L3925) | external-constructor-callback-or-unresolved |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `tempfile::tempdir().expect` | [3931](../../src/client_extensions.rs#L3931) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `tempfile::tempdir` | [3931](../../src/client_extensions.rs#L3931) | external-constructor-callback-or-unresolved |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `production_extensions` | [3932](../../src/client_extensions.rs#L3932) | [tekes-supervisor::client_extensions::tests::production_extensions](../../src/client_extensions.rs#L3095) |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `root.path` | [3932](../../src/client_extensions.rs#L3932), [3934](../../src/client_extensions.rs#L3934) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `root.path().join("threads").join` | [3934](../../src/client_extensions.rs#L3934) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `root.path().join` | [3934](../../src/client_extensions.rs#L3934) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `fs::create_dir_all(&folder).expect` | [3935](../../src/client_extensions.rs#L3935) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `fs::create_dir_all` | [3935](../../src/client_extensions.rs#L3935) | external-constructor-callback-or-unresolved |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `store::AssetStore::new(folder.join("assets")).expect` | [3936](../../src/client_extensions.rs#L3936) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `store::AssetStore::new` | [3936](../../src/client_extensions.rs#L3936) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `folder.join` | [3936](../../src/client_extensions.rs#L3936), [4048](../../src/client_extensions.rs#L4048) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `epoch_profile_asset` | [3937](../../src/client_extensions.rs#L3937), [3944](../../src/client_extensions.rs#L3944) | [tekes-supervisor::client_extensions::tests::epoch_profile_asset](../../src/client_extensions.rs#L3901) |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `Vec::new` | [3951](../../src/client_extensions.rs#L3951) | external-constructor-callback-or-unresolved |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `append_canonical` | [3952](../../src/client_extensions.rs#L3952), [3956](../../src/client_extensions.rs#L3956), [3960](../../src/client_extensions.rs#L3960), [3964](../../src/client_extensions.rs#L3964), [3968](../../src/client_extensions.rs#L3968), [3972](../../src/client_extensions.rs#L3972), [3976](../../src/client_extensions.rs#L3976), [3980](../../src/client_extensions.rs#L3980), [3984](../../src/client_extensions.rs#L3984), [3988](../../src/client_extensions.rs#L3988), [3992](../../src/client_extensions.rs#L3992), [3996](../../src/client_extensions.rs#L3996), [4000](../../src/client_extensions.rs#L4000), [4004](../../src/client_extensions.rs#L4004), [4008](../../src/client_extensions.rs#L4008), [4012](../../src/client_extensions.rs#L4012), [4016](../../src/client_extensions.rs#L4016), [4020](../../src/client_extensions.rs#L4020), [4024](../../src/client_extensions.rs#L4024), [4028](../../src/client_extensions.rs#L4028), [4032](../../src/client_extensions.rs#L4032), [4036](../../src/client_extensions.rs#L4036), [4040](../../src/client_extensions.rs#L4040), [4044](../../src/client_extensions.rs#L4044) | [tekes-supervisor::client_extensions::tests::append_canonical](../../src/client_extensions.rs#L3896) |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `fs::write(folder.join("main.jsonl"), ledger).expect` | [4048](../../src/client_extensions.rs#L4048) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `fs::write` | [4048](../../src/client_extensions.rs#L4048) | external-constructor-callback-or-unresolved |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `execute(             &extensions,             "usage.cacheAttribution",             &serde_json::json!({"sessionId":session}),             "usage-cache",         )         .expect` | [4050](../../src/client_extensions.rs#L4050) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `execute` | [4050](../../src/client_extensions.rs#L4050), [4072](../../src/client_extensions.rs#L4072) | [tekes-supervisor::client_extensions::tests::execute](../../src/client_extensions.rs#L3368) |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `execute(             &extensions,             "usage.summary",             &serde_json::json!({"sessionId":session}),             "usage-summary",         )         .expect` | [4072](../../src/client_extensions.rs#L4072) | receiver-type-required |
| `usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets` | `process.shutdown` | [4081](../../src/client_extensions.rs#L4081) | receiver-type-required |
