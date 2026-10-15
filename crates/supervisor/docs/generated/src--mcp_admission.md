# tekes-supervisor::mcp_admission

[Package atlas](index.md) · [Source](../../src/mcp_admission.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::mcp_admission::Guard](../../src/mcp_admission.rs#L8) | type_item | `private` |  |
| [tekes-supervisor::mcp_admission::admit](../../src/mcp_admission.rs#L9) | function_item | `pub(crate)` |  |
| [tekes-supervisor::mcp_admission::Check](../../src/mcp_admission.rs#L82) | type_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer](../../src/mcp_admission.rs#L83) | struct_item | `pub` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::new](../../src/mcp_admission.rs#L88) | function_item | `pub` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::cached_tools](../../src/mcp_admission.rs#L99) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::admit](../../src/mcp_admission.rs#L102) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::server_name](../../src/mcp_admission.rs#L105) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::protocol_version](../../src/mcp_admission.rs#L108) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::server_identity](../../src/mcp_admission.rs#L111) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::capabilities](../../src/mcp_admission.rs#L114) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::catalog_generation](../../src/mcp_admission.rs#L117) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::start_catalog_subscription](../../src/mcp_admission.rs#L120) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::start_scoped_tool](../../src/mcp_admission.rs#L123) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::list_tools](../../src/mcp_admission.rs#L133) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::list_prompts](../../src/mcp_admission.rs#L136) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::list_resources](../../src/mcp_admission.rs#L139) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::call_tool](../../src/mcp_admission.rs#L142) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::call_tool_with_context](../../src/mcp_admission.rs#L150) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::call_tool_augmented](../../src/mcp_admission.rs#L160) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::get_prompt](../../src/mcp_admission.rs#L171) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::read_resource](../../src/mcp_admission.rs#L174) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::task_operation](../../src/mcp_admission.rs#L177) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::task_operation_cancellable](../../src/mcp_admission.rs#L184) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::AdmissionPeer::close](../../src/mcp_admission.rs#L193) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer](../../src/mcp_admission.rs#L200) | struct_item | `pub` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::new](../../src/mcp_admission.rs#L206) | function_item | `pub` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::server_name](../../src/mcp_admission.rs#L215) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::protocol_version](../../src/mcp_admission.rs#L218) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::server_identity](../../src/mcp_admission.rs#L221) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::capabilities](../../src/mcp_admission.rs#L224) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::catalog_generation](../../src/mcp_admission.rs#L227) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::list_resources](../../src/mcp_admission.rs#L234) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::read_resource](../../src/mcp_admission.rs#L237) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::close](../../src/mcp_admission.rs#L241) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::list_tools](../../src/mcp_admission.rs#L244) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::list_prompts](../../src/mcp_admission.rs#L247) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::call_tool](../../src/mcp_admission.rs#L250) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::get_prompt](../../src/mcp_admission.rs#L258) | function_item | `private` |  |
| [tekes-supervisor::mcp_admission::ResourcePeer::task_operation](../../src/mcp_admission.rs#L261) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `SecretAccess` | `crate::mcp_runtime::SecretAccess` | `private` |
| `server_pool_key` | `crate::mcp_runtime::server_pool_key` | `private` |
| `*` | `mcp::*` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Arc` | `std::sync::Arc` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::folder_authority::FolderAuthorityStore::new"]
  n1["tekes-supervisor::mcp_admission::AdmissionPeer::admit"]
  n2["tekes-supervisor::mcp_admission::AdmissionPeer::server_name"]
  n3["tekes-supervisor::mcp_admission::AdmissionPeer::protocol_version"]
  n4["tekes-supervisor::mcp_admission::AdmissionPeer::server_identity"]
  n5["tekes-supervisor::mcp_admission::AdmissionPeer::capabilities"]
  n6["tekes-supervisor::mcp_admission::AdmissionPeer::catalog_generation"]
  n7["tekes-supervisor::mcp_admission::AdmissionPeer::start_catalog_subscription"]
  n8["tekes-supervisor::mcp_admission::AdmissionPeer::start_scoped_tool"]
  n9["tekes-supervisor::mcp_admission::AdmissionPeer::list_tools"]
  n10["tekes-supervisor::mcp_admission::AdmissionPeer::list_prompts"]
  n11["tekes-supervisor::mcp_admission::AdmissionPeer::list_resources"]
  n12["tekes-supervisor::mcp_admission::AdmissionPeer::call_tool"]
  n13["tekes-supervisor::mcp_admission::AdmissionPeer::call_tool_with_context"]
  n14["tekes-supervisor::mcp_admission::AdmissionPeer::call_tool_augmented"]
  n15["tekes-supervisor::mcp_admission::AdmissionPeer::get_prompt"]
  n16["tekes-supervisor::mcp_admission::AdmissionPeer::read_resource"]
  n17["tekes-supervisor::mcp_admission::AdmissionPeer::task_operation"]
  n18["tekes-supervisor::mcp_admission::AdmissionPeer::new"]
  n19["tekes-supervisor::mcp_admission::AdmissionPeer::cached_tools"]
  n20["tekes-supervisor::mcp_admission::admit"]
  n21["tekes-supervisor::mcp_runtime::credential_fields"]
  n22["tekes-supervisor::mcp_runtime::server_pool_key"]
  n23["tekes-supervisor::mcp_scope::storage_root"]
  n24["tekes-supervisor::mcp_scope::validate_user"]
  n25["tekes-supervisor::mcp_scope::validate_project"]
  n26["tekes-supervisor::mcp_scope::folder"]
  n20 --> n0
  n20 --> n21
  n20 --> n22
  n20 --> n23
  n20 --> n24
  n20 --> n25
  n20 --> n26
```

</details>

<details><summary>Functions 21–36: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["tekes-supervisor::mcp_admission::AdmissionPeer::task_operation_cancellable"]
  n1["tekes-supervisor::mcp_admission::AdmissionPeer::close"]
  n2["tekes-supervisor::mcp_admission::ResourcePeer::new"]
  n3["tekes-supervisor::mcp_admission::ResourcePeer::server_name"]
  n4["tekes-supervisor::mcp_admission::ResourcePeer::protocol_version"]
  n5["tekes-supervisor::mcp_admission::ResourcePeer::server_identity"]
  n6["tekes-supervisor::mcp_admission::ResourcePeer::capabilities"]
  n7["tekes-supervisor::mcp_admission::ResourcePeer::catalog_generation"]
  n8["tekes-supervisor::mcp_admission::ResourcePeer::list_resources"]
  n9["tekes-supervisor::mcp_admission::ResourcePeer::read_resource"]
  n10["tekes-supervisor::mcp_admission::ResourcePeer::close"]
  n11["tekes-supervisor::mcp_admission::ResourcePeer::list_tools"]
  n12["tekes-supervisor::mcp_admission::ResourcePeer::list_prompts"]
  n13["tekes-supervisor::mcp_admission::ResourcePeer::call_tool"]
  n14["tekes-supervisor::mcp_admission::ResourcePeer::get_prompt"]
  n15["tekes-supervisor::mcp_admission::ResourcePeer::task_operation"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `admit` | `secrets.registry.as_ref` | [15](../../src/mcp_admission.rs#L15) | receiver-type-required |
| `admit` | `registry             .get(&server.reference)             .map_err(&#124;error&#124; McpError::Conflict(error.to_string()))?             .ok_or_else` | [18](../../src/mcp_admission.rs#L18) | receiver-type-required |
| `admit` | `registry             .get(&server.reference)             .map_err` | [18](../../src/mcp_admission.rs#L18) | receiver-type-required |
| `admit` | `registry             .get` | [18](../../src/mcp_admission.rs#L18) | receiver-type-required |
| `admit` | `McpError::Conflict` | [20](../../src/mcp_admission.rs#L20), [21](../../src/mcp_admission.rs#L21), [23](../../src/mcp_admission.rs#L23), [34](../../src/mcp_admission.rs#L34), [37](../../src/mcp_admission.rs#L37), [46](../../src/mcp_admission.rs#L46), [56](../../src/mcp_admission.rs#L56), [70](../../src/mcp_admission.rs#L70), [73](../../src/mcp_admission.rs#L73) | external-constructor-callback-or-unresolved |
| `admit` | `error.to_string` | [20](../../src/mcp_admission.rs#L20), [34](../../src/mcp_admission.rs#L34), [70](../../src/mcp_admission.rs#L70) | receiver-type-required |
| `admit` | `"MCP server removed".into` | [21](../../src/mcp_admission.rs#L21) | receiver-type-required |
| `admit` | `Err` | [23](../../src/mcp_admission.rs#L23), [37](../../src/mcp_admission.rs#L37), [46](../../src/mcp_admission.rs#L46), [56](../../src/mcp_admission.rs#L56), [73](../../src/mcp_admission.rs#L73) | external-constructor-callback-or-unresolved |
| `admit` | `"MCP configuration changed".into` | [23](../../src/mcp_admission.rs#L23) | receiver-type-required |
| `admit` | `crate::mcp_scope::validate_user` | [26](../../src/mcp_admission.rs#L26) | [tekes-supervisor::mcp_scope::validate_user](../../src/mcp_scope.rs#L176) |
| `admit` | `crate::mcp_scope::validate_project` | [29](../../src/mcp_admission.rs#L29) | [tekes-supervisor::mcp_scope::validate_project](../../src/mcp_scope.rs#L196) |
| `admit` | `crate::mcp_runtime::credential_fields` | [31](../../src/mcp_admission.rs#L31) | [tekes-supervisor::mcp_runtime::credential_fields](../../src/mcp_runtime.rs#L2143) |
| `admit` | `registry                 .credential_reference_state(&server.reference, &field, &id)                 .map_err` | [32](../../src/mcp_admission.rs#L32) | receiver-type-required |
| `admit` | `registry                 .credential_reference_state` | [32](../../src/mcp_admission.rs#L32) | receiver-type-required |
| `admit` | `Some` | [35](../../src/mcp_admission.rs#L35), [77](../../src/mcp_admission.rs#L77) | external-constructor-callback-or-unresolved |
| `admit` | `"MCP credential not bound".into` | [37](../../src/mcp_admission.rs#L37) | receiver-type-required |
| `admit` | `resolver.resolve(reference).map_err` | [44](../../src/mcp_admission.rs#L44) | receiver-type-required |
| `admit` | `resolver.resolve` | [44](../../src/mcp_admission.rs#L44) | receiver-type-required |
| `admit` | `plugin.is_none` | [45](../../src/mcp_admission.rs#L45) | receiver-type-required |
| `admit` | `"MCP plugin unavailable".into` | [46](../../src/mcp_admission.rs#L46) | receiver-type-required |
| `admit` | `server_pool_key` | [49](../../src/mcp_admission.rs#L49), [71](../../src/mcp_admission.rs#L71) | [tekes-supervisor::mcp_runtime::server_pool_key](../../src/mcp_runtime.rs#L3080) |
| `admit` | `plugin.as_ref` | [49](../../src/mcp_admission.rs#L49), [71](../../src/mcp_admission.rs#L71) | receiver-type-required |
| `admit` | `secrets.plugin_resolver.is_none` | [52](../../src/mcp_admission.rs#L52) | receiver-type-required |
| `admit` | `key.plugin_generation.clone` | [53](../../src/mcp_admission.rs#L53) | receiver-type-required |
| `admit` | `"MCP owner or authority generation changed".into` | [57](../../src/mcp_admission.rs#L57) | receiver-type-required |
| `admit` | `Ok` | [61](../../src/mcp_admission.rs#L61), [77](../../src/mcp_admission.rs#L77), [79](../../src/mcp_admission.rs#L79) | external-constructor-callback-or-unresolved |
| `admit` | `crate::mcp_scope::folder` | [64](../../src/mcp_admission.rs#L64) | [tekes-supervisor::mcp_scope::folder](../../src/mcp_scope.rs#L34) |
| `admit` | `profile::FolderAuthorityStore::new` | [67](../../src/mcp_admission.rs#L67) | [profile::folder_authority::FolderAuthorityStore::new](../../../profile/src/folder_authority.rs#L86) |
| `admit` | `crate::mcp_scope::storage_root` | [67](../../src/mcp_admission.rs#L67) | [tekes-supervisor::mcp_scope::storage_root](../../src/mcp_scope.rs#L13) |
| `admit` | `authority             .admit(&folder)             .map_err` | [68](../../src/mcp_admission.rs#L68) | receiver-type-required |
| `admit` | `authority             .admit` | [68](../../src/mcp_admission.rs#L68) | receiver-type-required |
| `admit` | `"MCP trust changed during admission".into` | [74](../../src/mcp_admission.rs#L74) | receiver-type-required |
| `admit` | `Box::new` | [77](../../src/mcp_admission.rs#L77) | external-constructor-callback-or-unresolved |
| `new` | `Arc::new` | [94](../../src/mcp_admission.rs#L94) | external-constructor-callback-or-unresolved |
| `cached_tools` | `self.peer.cached_tools` | [100](../../src/mcp_admission.rs#L100) | receiver-type-required |
| `admit` | `(self.check)` | [103](../../src/mcp_admission.rs#L103) | external-constructor-callback-or-unresolved |
| `server_name` | `self.peer.server_name` | [106](../../src/mcp_admission.rs#L106) | receiver-type-required |
| `protocol_version` | `self.peer.protocol_version` | [109](../../src/mcp_admission.rs#L109) | receiver-type-required |
| `server_identity` | `self.peer.server_identity` | [112](../../src/mcp_admission.rs#L112) | receiver-type-required |
| `capabilities` | `self.peer.capabilities` | [115](../../src/mcp_admission.rs#L115) | receiver-type-required |
| `catalog_generation` | `self.peer.catalog_generation` | [118](../../src/mcp_admission.rs#L118) | receiver-type-required |
| `start_catalog_subscription` | `self.peer.start_catalog_subscription` | [121](../../src/mcp_admission.rs#L121) | receiver-type-required |
| `start_scoped_tool` | `self.peer             .start_scoped_tool` | [130](../../src/mcp_admission.rs#L130) | receiver-type-required |
| `list_tools` | `self.peer.list_tools` | [134](../../src/mcp_admission.rs#L134) | receiver-type-required |
| `list_prompts` | `self.peer.list_prompts` | [137](../../src/mcp_admission.rs#L137) | receiver-type-required |
| `list_resources` | `self.peer.list_resources` | [140](../../src/mcp_admission.rs#L140) | receiver-type-required |
| `call_tool` | `self.peer.call_tool` | [148](../../src/mcp_admission.rs#L148) | receiver-type-required |
| `call_tool_with_context` | `self.peer             .call_tool_with_context` | [157](../../src/mcp_admission.rs#L157) | receiver-type-required |
| `call_tool_augmented` | `self.peer             .call_tool_augmented` | [168](../../src/mcp_admission.rs#L168) | receiver-type-required |
| `get_prompt` | `self.peer.get_prompt` | [172](../../src/mcp_admission.rs#L172) | receiver-type-required |
| `read_resource` | `self.peer.read_resource` | [175](../../src/mcp_admission.rs#L175) | receiver-type-required |
| `task_operation` | `self.peer.task_operation` | [182](../../src/mcp_admission.rs#L182) | receiver-type-required |
| `task_operation_cancellable` | `self.peer             .task_operation_cancellable` | [190](../../src/mcp_admission.rs#L190) | receiver-type-required |
| `close` | `self.peer.close` | [194](../../src/mcp_admission.rs#L194) | receiver-type-required |
| `server_name` | `self.metadata.server_name` | [216](../../src/mcp_admission.rs#L216) | receiver-type-required |
| `protocol_version` | `self.metadata.protocol_version` | [219](../../src/mcp_admission.rs#L219) | receiver-type-required |
| `server_identity` | `self.metadata.server_identity` | [222](../../src/mcp_admission.rs#L222) | receiver-type-required |
| `capabilities` | `self.metadata.capabilities` | [225](../../src/mcp_admission.rs#L225) | receiver-type-required |
| `catalog_generation` | `self.broker             .catalog_generation(self.key.clone())             .ok()             .flatten()             .unwrap_or_else` | [228](../../src/mcp_admission.rs#L228) | receiver-type-required |
| `catalog_generation` | `self.broker             .catalog_generation(self.key.clone())             .ok()             .flatten` | [228](../../src/mcp_admission.rs#L228) | receiver-type-required |
| `catalog_generation` | `self.broker             .catalog_generation(self.key.clone())             .ok` | [228](../../src/mcp_admission.rs#L228) | receiver-type-required |
| `catalog_generation` | `self.broker             .catalog_generation` | [228](../../src/mcp_admission.rs#L228) | receiver-type-required |
| `catalog_generation` | `self.key.clone` | [229](../../src/mcp_admission.rs#L229) | receiver-type-required |
| `catalog_generation` | `self.metadata.catalog_generation` | [232](../../src/mcp_admission.rs#L232) | receiver-type-required |
| `list_resources` | `Box::pin` | [235](../../src/mcp_admission.rs#L235) | external-constructor-callback-or-unresolved |
| `list_resources` | `self.broker.list_resources` | [235](../../src/mcp_admission.rs#L235) | receiver-type-required |
| `list_resources` | `self.key.clone` | [235](../../src/mcp_admission.rs#L235) | receiver-type-required |
| `read_resource` | `uri.to_owned` | [238](../../src/mcp_admission.rs#L238) | receiver-type-required |
| `read_resource` | `Box::pin` | [239](../../src/mcp_admission.rs#L239) | external-constructor-callback-or-unresolved |
| `read_resource` | `self.broker.read_resource` | [239](../../src/mcp_admission.rs#L239) | receiver-type-required |
| `read_resource` | `self.key.clone` | [239](../../src/mcp_admission.rs#L239) | receiver-type-required |
| `close` | `Box::pin` | [242](../../src/mcp_admission.rs#L242) | external-constructor-callback-or-unresolved |
| `close` | `Ok` | [242](../../src/mcp_admission.rs#L242) | external-constructor-callback-or-unresolved |
| `list_tools` | `self.metadata.list_tools` | [245](../../src/mcp_admission.rs#L245) | receiver-type-required |
| `list_prompts` | `self.metadata.list_prompts` | [248](../../src/mcp_admission.rs#L248) | receiver-type-required |
| `call_tool` | `self.metadata.call_tool` | [256](../../src/mcp_admission.rs#L256) | receiver-type-required |
| `get_prompt` | `self.metadata.get_prompt` | [259](../../src/mcp_admission.rs#L259) | receiver-type-required |
| `task_operation` | `self.metadata.task_operation` | [266](../../src/mcp_admission.rs#L266) | receiver-type-required |
