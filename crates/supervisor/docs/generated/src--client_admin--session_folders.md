# tekes-supervisor::client_admin::session_folders

[Package atlas](index.md) · [Source](../../src/client_admin/session_folders.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::client_admin::session_folders::SESSION_FOLDER_METHODS](../../src/client_admin/session_folders.rs#L6) | const_item | `pub` |  |
| [tekes-supervisor::client_admin::session_folders::SessionFoldersGet](../../src/client_admin/session_folders.rs#L14) | struct_item | `pub(super)` |  |
| [tekes-supervisor::client_admin::session_folders::SessionFolderChange](../../src/client_admin/session_folders.rs#L20) | struct_item | `pub(super)` |  |
| [tekes-supervisor::client_admin::session_folders::revision](../../src/client_admin/session_folders.rs#L26) | function_item | `private` |  |
| [tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::session_folders](../../src/client_admin/session_folders.rs#L40) | function_item | `pub(super)` |  |
| [tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::change_session_folder](../../src/client_admin/session_folders.rs#L67) | function_item | `pub(super)` |  |
| [tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::complete_session_folder_change](../../src/client_admin/session_folders.rs#L226) | function_item | `private` |  |
| [tekes-supervisor::client_admin::session_folders::tests::request](../../src/client_admin/session_folders.rs#L259) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_admin::session_folders::tests::create](../../src/client_admin/session_folders.rs#L268) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_admin::session_folders::tests::json](../../src/client_admin/session_folders.rs#L288) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_admin::session_folders::tests::rejected_folder_edit_does_not_publish_catalog_or_ledger](../../src/client_admin/session_folders.rs#L293) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_admin::session_folders::tests::phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited](../../src/client_admin/session_folders.rs#L378) | function_item | `private` | test; #[cfg(test)] |
| [tekes-supervisor::client_admin::session_folders::tests::phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust](../../src/client_admin/session_folders.rs#L601) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `SupervisorSessionAuthority` | `crate::endpoint_carrier::SupervisorSessionAuthority` | `private` |
| `*` | `super::*` | `private` |
| `DurableHandoffSignal` | `endpoint::DurableHandoffSignal` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::client_admin::session_folders::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–4: 18 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::types::validate_session_id"]
  n1["profile::folder_catalog::FolderCatalog::new"]
  n2["profile::scope::SessionScope::locate"]
  n3["profile::scope::SessionScope::from_genesis_value"]
  n4["schema::event::Event::from_value"]
  n5["schema::effective_session_genesis"]
  n6["store::folder::ThreadStore::open"]
  n7["store::tail::scan_valid_prefix"]
  n8["tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::complete_session_folder_change"]
  n9["tekes-supervisor::client_admin::session_folders::revision"]
  n10["tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::session_folders"]
  n11["tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::change_session_folder"]
  n12["tekes-supervisor::process_host::workspace_quiescence_lock_path"]
  n13["tools::sandbox::kernel_protected_roots"]
  n14["tools::sandbox::protected_root_containing"]
  n10 --> n0
  n10 --> n2
  n10 --> n3
  n10 --> n5
  n10 --> n7
  n11 --> n0
  n11 --> n1
  n11 --> n2
  n11 --> n3
  n11 --> n4
  n11 --> n5
  n11 --> n6
  n11 --> n7
  n11 --> n8
  n11 --> n9
  n11 --> n12
  n11 --> n13
  n11 --> n14
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `revision` | `events         .iter()         .rev()         .find(&#124;event&#124; {             event.kind() == &schema::EventKind::Meta                 && serde_json::to_value(event.raw())                     .ok()                     .is_some_and(&#124;raw&#124; raw.get("scope_change").is_some())         })         .map_or` | [27](../../src/client_admin/session_folders.rs#L27) | receiver-type-required |
| `revision` | `events         .iter()         .rev()         .find` | [27](../../src/client_admin/session_folders.rs#L27) | receiver-type-required |
| `revision` | `events         .iter()         .rev` | [27](../../src/client_admin/session_folders.rs#L27) | receiver-type-required |
| `revision` | `events         .iter` | [27](../../src/client_admin/session_folders.rs#L27) | receiver-type-required |
| `revision` | `event.kind` | [31](../../src/client_admin/session_folders.rs#L31) | receiver-type-required |
| `revision` | `serde_json::to_value(event.raw())                     .ok()                     .is_some_and` | [32](../../src/client_admin/session_folders.rs#L32) | receiver-type-required |
| `revision` | `serde_json::to_value(event.raw())                     .ok` | [32](../../src/client_admin/session_folders.rs#L32) | receiver-type-required |
| `revision` | `serde_json::to_value` | [32](../../src/client_admin/session_folders.rs#L32) | external-constructor-callback-or-unresolved |
| `revision` | `event.raw` | [32](../../src/client_admin/session_folders.rs#L32) | receiver-type-required |
| `revision` | `raw.get("scope_change").is_some` | [34](../../src/client_admin/session_folders.rs#L34) | receiver-type-required |
| `revision` | `raw.get` | [34](../../src/client_admin/session_folders.rs#L34) | receiver-type-required |
| `session_folders` | `endpoint::validate_session_id(&input.session_id)             .map_err` | [44](../../src/client_admin/session_folders.rs#L44) | receiver-type-required |
| `session_folders` | `endpoint::validate_session_id` | [44](../../src/client_admin/session_folders.rs#L44) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `session_folders` | `internal` | [45](../../src/client_admin/session_folders.rs#L45), [51](../../src/client_admin/session_folders.rs#L51), [53](../../src/client_admin/session_folders.rs#L53) | external-constructor-callback-or-unresolved |
| `session_folders` | `error.to_string` | [45](../../src/client_admin/session_folders.rs#L45), [53](../../src/client_admin/session_folders.rs#L53) | receiver-type-required |
| `session_folders` | `profile::SessionScope::locate(&self.root, &input.session_id)             .map_err` | [46](../../src/client_admin/session_folders.rs#L46) | receiver-type-required |
| `session_folders` | `profile::SessionScope::locate` | [46](../../src/client_admin/session_folders.rs#L46) | [profile::scope::SessionScope::locate](../../../profile/src/scope.rs#L34) |
| `session_folders` | `fs::read(scope.session_folder().join("main.jsonl")).map_err` | [48](../../src/client_admin/session_folders.rs#L48) | receiver-type-required |
| `session_folders` | `fs::read` | [48](../../src/client_admin/session_folders.rs#L48) | external-constructor-callback-or-unresolved |
| `session_folders` | `scope.session_folder().join` | [48](../../src/client_admin/session_folders.rs#L48) | receiver-type-required |
| `session_folders` | `scope.session_folder` | [48](../../src/client_admin/session_folders.rs#L48), [55](../../src/client_admin/session_folders.rs#L55) | receiver-type-required |
| `session_folders` | `store::scan_valid_prefix(&bytes, schema::READER_VERSION)             .projection             .ok_or_else` | [49](../../src/client_admin/session_folders.rs#L49) | receiver-type-required |
| `session_folders` | `store::scan_valid_prefix` | [49](../../src/client_admin/session_folders.rs#L49) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `session_folders` | `schema::effective_session_genesis(&projection.events)             .map_err(&#124;error&#124; internal(error.to_string()))?             .expect` | [52](../../src/client_admin/session_folders.rs#L52) | receiver-type-required |
| `session_folders` | `schema::effective_session_genesis(&projection.events)             .map_err` | [52](../../src/client_admin/session_folders.rs#L52) | receiver-type-required |
| `session_folders` | `schema::effective_session_genesis` | [52](../../src/client_admin/session_folders.rs#L52) | [schema::effective_session_genesis](../../../schema/src/lib.rs#L48) |
| `session_folders` | `profile::SessionScope::from_genesis_value(scope.session_folder(), &raw)             .map_err` | [55](../../src/client_admin/session_folders.rs#L55) | receiver-type-required |
| `session_folders` | `profile::SessionScope::from_genesis_value` | [55](../../src/client_admin/session_folders.rs#L55) | [profile::scope::SessionScope::from_genesis_value](../../../profile/src/scope.rs#L87) |
| `session_folders` | `scope.folders(&self.repository).map_err` | [57](../../src/client_admin/session_folders.rs#L57) | receiver-type-required |
| `session_folders` | `scope.folders` | [57](../../src/client_admin/session_folders.rs#L57) | receiver-type-required |
| `session_folders` | `to_ijson` | [58](../../src/client_admin/session_folders.rs#L58) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `endpoint::validate_session_id(&input.session_id)             .map_err` | [74](../../src/client_admin/session_folders.rs#L74) | receiver-type-required |
| `change_session_folder` | `endpoint::validate_session_id` | [74](../../src/client_admin/session_folders.rs#L74) | [endpoint::types::validate_session_id](../../../endpoint/src/types.rs#L130) |
| `change_session_folder` | `internal` | [75](../../src/client_admin/session_folders.rs#L75), [79](../../src/client_admin/session_folders.rs#L79), [97](../../src/client_admin/session_folders.rs#L97), [132](../../src/client_admin/session_folders.rs#L132), [133](../../src/client_admin/session_folders.rs#L133), [185](../../src/client_admin/session_folders.rs#L185), [215](../../src/client_admin/session_folders.rs#L215) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `error.to_string` | [75](../../src/client_admin/session_folders.rs#L75), [132](../../src/client_admin/session_folders.rs#L132), [185](../../src/client_admin/session_folders.rs#L185) | receiver-type-required |
| `change_session_folder` | `self             .mutation_gate             .lock()             .map_err` | [76](../../src/client_admin/session_folders.rs#L76) | receiver-type-required |
| `change_session_folder` | `self             .mutation_gate             .lock` | [76](../../src/client_admin/session_folders.rs#L76) | receiver-type-required |
| `change_session_folder` | `management_request_hash` | [80](../../src/client_admin/session_folders.rs#L80) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `store::ThreadStore::open(&self.root).map_err` | [81](../../src/client_admin/session_folders.rs#L81) | receiver-type-required |
| `change_session_folder` | `store::ThreadStore::open` | [81](../../src/client_admin/session_folders.rs#L81) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `change_session_folder` | `principal.to_owned` | [83](../../src/client_admin/session_folders.rs#L83) | receiver-type-required |
| `change_session_folder` | `"session-folders".to_owned` | [84](../../src/client_admin/session_folders.rs#L84) | receiver-type-required |
| `change_session_folder` | `input.session_id.clone` | [85](../../src/client_admin/session_folders.rs#L85) | receiver-type-required |
| `change_session_folder` | `request.operation.clone` | [86](../../src/client_admin/session_folders.rs#L86) | receiver-type-required |
| `change_session_folder` | `request.rpc_id.clone` | [87](../../src/client_admin/session_folders.rs#L87) | receiver-type-required |
| `change_session_folder` | `profile::SessionScope::locate(&self.root, &input.session_id)             .map_err` | [91](../../src/client_admin/session_folders.rs#L91) | receiver-type-required |
| `change_session_folder` | `profile::SessionScope::locate` | [91](../../src/client_admin/session_folders.rs#L91) | [profile::scope::SessionScope::locate](../../../profile/src/scope.rs#L34) |
| `change_session_folder` | `fs::read(old_scope.session_folder().join("main.jsonl")).map_err` | [94](../../src/client_admin/session_folders.rs#L94) | receiver-type-required |
| `change_session_folder` | `fs::read` | [94](../../src/client_admin/session_folders.rs#L94) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `old_scope.session_folder().join` | [94](../../src/client_admin/session_folders.rs#L94) | receiver-type-required |
| `change_session_folder` | `old_scope.session_folder` | [94](../../src/client_admin/session_folders.rs#L94) | receiver-type-required |
| `change_session_folder` | `store::scan_valid_prefix(&prefix, schema::READER_VERSION)             .projection             .ok_or_else` | [95](../../src/client_admin/session_folders.rs#L95) | receiver-type-required |
| `change_session_folder` | `store::scan_valid_prefix` | [95](../../src/client_admin/session_folders.rs#L95) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `change_session_folder` | `projection.origin_tuples.get` | [98](../../src/client_admin/session_folders.rs#L98) | receiver-type-required |
| `change_session_folder` | `projection                 .events                 .iter()                 .find(&#124;event&#124; event.seq() == *seq)                 .expect` | [99](../../src/client_admin/session_folders.rs#L99) | receiver-type-required |
| `change_session_folder` | `projection                 .events                 .iter()                 .find` | [99](../../src/client_admin/session_folders.rs#L99) | receiver-type-required |
| `change_session_folder` | `projection                 .events                 .iter` | [99](../../src/client_admin/session_folders.rs#L99) | receiver-type-required |
| `change_session_folder` | `event.seq` | [102](../../src/client_admin/session_folders.rs#L102), [214](../../src/client_admin/session_folders.rs#L214) | receiver-type-required |
| `change_session_folder` | `event.string_field` | [104](../../src/client_admin/session_folders.rs#L104), [216](../../src/client_admin/session_folders.rs#L216) | receiver-type-required |
| `change_session_folder` | `Some` | [104](../../src/client_admin/session_folders.rs#L104), [143](../../src/client_admin/session_folders.rs#L143), [174](../../src/client_admin/session_folders.rs#L174), [175](../../src/client_admin/session_folders.rs#L175), [187](../../src/client_admin/session_folders.rs#L187), [190](../../src/client_admin/session_folders.rs#L190), [216](../../src/client_admin/session_folders.rs#L216) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `Err` | [105](../../src/client_admin/session_folders.rs#L105), [131](../../src/client_admin/session_folders.rs#L131), [153](../../src/client_admin/session_folders.rs#L153), [159](../../src/client_admin/session_folders.rs#L159), [167](../../src/client_admin/session_folders.rs#L167), [198](../../src/client_admin/session_folders.rs#L198), [217](../../src/client_admin/session_folders.rs#L217) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `failure` | [105](../../src/client_admin/session_folders.rs#L105), [119](../../src/client_admin/session_folders.rs#L119), [151](../../src/client_admin/session_folders.rs#L151), [153](../../src/client_admin/session_folders.rs#L153), [159](../../src/client_admin/session_folders.rs#L159), [167](../../src/client_admin/session_folders.rs#L167), [176](../../src/client_admin/session_folders.rs#L176), [201](../../src/client_admin/session_folders.rs#L201), [217](../../src/client_admin/session_folders.rs#L217) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `self.complete_session_folder_change` | [111](../../src/client_admin/session_folders.rs#L111), [223](../../src/client_admin/session_folders.rs#L223) | [tekes-supervisor::client_admin::session_folders::ClientAdminRoutes::complete_session_folder_change](../../src/client_admin/session_folders.rs#L226) |
| `change_session_folder` | `NamedLock::try_exclusive(crate::process_host::workspace_quiescence_lock_path(                 &self.root,                 &format!("session:{}", input.session_id),             ))             .map_err` | [114](../../src/client_admin/session_folders.rs#L114) | receiver-type-required |
| `change_session_folder` | `NamedLock::try_exclusive` | [114](../../src/client_admin/session_folders.rs#L114) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `crate::process_host::workspace_quiescence_lock_path` | [114](../../src/client_admin/session_folders.rs#L114) | [tekes-supervisor::process_host::workspace_quiescence_lock_path](../../src/process_host.rs#L6541) |
| `change_session_folder` | `internal_store` | [124](../../src/client_admin/session_folders.rs#L124), [206](../../src/client_admin/session_folders.rs#L206) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `store.append_keyed_with_projection` | [128](../../src/client_admin/session_folders.rs#L128) | receiver-type-required |
| `change_session_folder` | `revision` | [130](../../src/client_admin/session_folders.rs#L130) | [tekes-supervisor::client_admin::session_folders::revision](../../src/client_admin/session_folders.rs#L26) |
| `change_session_folder` | `stale` | [131](../../src/client_admin/session_folders.rs#L131) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `schema::effective_session_genesis(&projection.events).map_err(&#124;error&#124; internal(error.to_string()))?                     .ok_or_else` | [132](../../src/client_admin/session_folders.rs#L132) | receiver-type-required |
| `change_session_folder` | `schema::effective_session_genesis(&projection.events).map_err` | [132](../../src/client_admin/session_folders.rs#L132) | receiver-type-required |
| `change_session_folder` | `schema::effective_session_genesis` | [132](../../src/client_admin/session_folders.rs#L132) | [schema::effective_session_genesis](../../../schema/src/lib.rs#L48) |
| `change_session_folder` | `profile::SessionScope::from_genesis_value(self.root.join("threads").join(&input.session_id), &raw).map_err` | [134](../../src/client_admin/session_folders.rs#L134) | receiver-type-required |
| `change_session_folder` | `profile::SessionScope::from_genesis_value` | [134](../../src/client_admin/session_folders.rs#L134) | [profile::scope::SessionScope::from_genesis_value](../../../profile/src/scope.rs#L87) |
| `change_session_folder` | `self.root.join("threads").join` | [134](../../src/client_admin/session_folders.rs#L134) | receiver-type-required |
| `change_session_folder` | `self.root.join` | [134](../../src/client_admin/session_folders.rs#L134) | receiver-type-required |
| `change_session_folder` | `profile::FolderCatalog::new` | [135](../../src/client_admin/session_folders.rs#L135) | [profile::folder_catalog::FolderCatalog::new](../../../profile/src/folder_catalog.rs#L138) |
| `change_session_folder` | `catalog.revision().map_err` | [136](../../src/client_admin/session_folders.rs#L136) | receiver-type-required |
| `change_session_folder` | `catalog.revision` | [136](../../src/client_admin/session_folders.rs#L136) | receiver-type-required |
| `change_session_folder` | `scope.session_config` | [137](../../src/client_admin/session_folders.rs#L137), [180](../../src/client_admin/session_folders.rs#L180) | receiver-type-required |
| `change_session_folder` | `catalog.resolve(config).map_err` | [138](../../src/client_admin/session_folders.rs#L138) | receiver-type-required |
| `change_session_folder` | `catalog.resolve` | [138](../../src/client_admin/session_folders.rs#L138) | receiver-type-required |
| `change_session_folder` | `scope.workspace_config(&self.repository).map_err` | [141](../../src/client_admin/session_folders.rs#L141) | receiver-type-required |
| `change_session_folder` | `scope.workspace_config` | [141](../../src/client_admin/session_folders.rs#L141) | receiver-type-required |
| `change_session_folder` | `scope.workspace_id().to_owned` | [142](../../src/client_admin/session_folders.rs#L142) | receiver-type-required |
| `change_session_folder` | `scope.workspace_id` | [142](../../src/client_admin/session_folders.rs#L142) | receiver-type-required |
| `change_session_folder` | `Vec::new` | [143](../../src/client_admin/session_folders.rs#L143) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `scope.folders(&self.repository).map_err(internal_profile)?.into_iter().map(&#124;folder&#124; profile::WorkspaceFolder{id:folder.id,path:folder.path}).collect` | [143](../../src/client_admin/session_folders.rs#L143) | receiver-type-required |
| `change_session_folder` | `scope.folders(&self.repository).map_err(internal_profile)?.into_iter().map` | [143](../../src/client_admin/session_folders.rs#L143) | receiver-type-required |
| `change_session_folder` | `scope.folders(&self.repository).map_err(internal_profile)?.into_iter` | [143](../../src/client_admin/session_folders.rs#L143) | receiver-type-required |
| `change_session_folder` | `scope.folders(&self.repository).map_err` | [143](../../src/client_admin/session_folders.rs#L143) | receiver-type-required |
| `change_session_folder` | `scope.folders` | [143](../../src/client_admin/session_folders.rs#L143) | receiver-type-required |
| `change_session_folder` | `seed.folders.is_empty` | [146](../../src/client_admin/session_folders.rs#L146) | receiver-type-required |
| `change_session_folder` | `seed.folder_ids().into_iter().zip(seed.folder_paths()).map(&#124;(id,path)&#124; profile::WorkspaceFolder {id,path:path.to_owned()}).collect` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `seed.folder_ids().into_iter().zip(seed.folder_paths()).map` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `seed.folder_ids().into_iter().zip` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `seed.folder_ids().into_iter` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `seed.folder_ids` | [147](../../src/client_admin/session_folders.rs#L147), [173](../../src/client_admin/session_folders.rs#L173) | receiver-type-required |
| `change_session_folder` | `seed.folder_paths` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `path.to_owned` | [147](../../src/client_admin/session_folders.rs#L147) | receiver-type-required |
| `change_session_folder` | `seed.cwd.clear` | [148](../../src/client_admin/session_folders.rs#L148) | receiver-type-required |
| `change_session_folder` | `Path::new(&input.path).canonicalize().map_err` | [151](../../src/client_admin/session_folders.rs#L151) | receiver-type-required |
| `change_session_folder` | `Path::new(&input.path).canonicalize` | [151](../../src/client_admin/session_folders.rs#L151) | receiver-type-required |
| `change_session_folder` | `Path::new` | [151](../../src/client_admin/session_folders.rs#L151), [170](../../src/client_admin/session_folders.rs#L170) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `canonical.is_dir` | [152](../../src/client_admin/session_folders.rs#L152) | receiver-type-required |
| `change_session_folder` | `canonical.parent().is_none` | [152](../../src/client_admin/session_folders.rs#L152) | receiver-type-required |
| `change_session_folder` | `canonical.parent` | [152](../../src/client_admin/session_folders.rs#L152) | receiver-type-required |
| `change_session_folder` | `tools::protected_root_containing(&tools::kernel_protected_roots(&self.root), &canonical).is_some` | [152](../../src/client_admin/session_folders.rs#L152) | receiver-type-required |
| `change_session_folder` | `tools::protected_root_containing` | [152](../../src/client_admin/session_folders.rs#L152) | [tools::sandbox::protected_root_containing](../../../tools/src/sandbox.rs#L213) |
| `change_session_folder` | `tools::kernel_protected_roots` | [152](../../src/client_admin/session_folders.rs#L152) | [tools::sandbox::kernel_protected_roots](../../../tools/src/sandbox.rs#L151) |
| `change_session_folder` | `canonical.to_string_lossy().into_owned` | [155](../../src/client_admin/session_folders.rs#L155) | receiver-type-required |
| `change_session_folder` | `canonical.to_string_lossy` | [155](../../src/client_admin/session_folders.rs#L155) | receiver-type-required |
| `change_session_folder` | `input.path.clone` | [156](../../src/client_admin/session_folders.rs#L156) | receiver-type-required |
| `change_session_folder` | `seed.folders.iter().any` | [157](../../src/client_admin/session_folders.rs#L157), [169](../../src/client_admin/session_folders.rs#L169), [170](../../src/client_admin/session_folders.rs#L170) | receiver-type-required |
| `change_session_folder` | `seed.folders.iter` | [157](../../src/client_admin/session_folders.rs#L157), [169](../../src/client_admin/session_folders.rs#L169), [170](../../src/client_admin/session_folders.rs#L170) | receiver-type-required |
| `change_session_folder` | `scope.folder_binding().map(str::to_owned).unwrap_or_else` | [161](../../src/client_admin/session_folders.rs#L161) | receiver-type-required |
| `change_session_folder` | `scope.folder_binding().map` | [161](../../src/client_admin/session_folders.rs#L161) | receiver-type-required |
| `change_session_folder` | `scope.folder_binding` | [161](../../src/client_admin/session_folders.rs#L161) | receiver-type-required |
| `change_session_folder` | `seed.folders[0].id.clone` | [161](../../src/client_admin/session_folders.rs#L161), [169](../../src/client_admin/session_folders.rs#L169) | receiver-type-required |
| `change_session_folder` | `seed.folders.push` | [164](../../src/client_admin/session_folders.rs#L164) | receiver-type-required |
| `change_session_folder` | `path.clone` | [164](../../src/client_admin/session_folders.rs#L164) | receiver-type-required |
| `change_session_folder` | `seed.policy.get_or_insert_with(profile::WorkspacePolicy::default).writable_roots.push` | [165](../../src/client_admin/session_folders.rs#L165) | receiver-type-required |
| `change_session_folder` | `seed.policy.get_or_insert_with` | [165](../../src/client_admin/session_folders.rs#L165) | receiver-type-required |
| `change_session_folder` | `seed.folders.len` | [167](../../src/client_admin/session_folders.rs#L167) | receiver-type-required |
| `change_session_folder` | `seed.folders.retain` | [168](../../src/client_admin/session_folders.rs#L168) | receiver-type-required |
| `change_session_folder` | `policy.writable_roots.retain` | [170](../../src/client_admin/session_folders.rs#L170) | receiver-type-required |
| `change_session_folder` | `Path::new(root).starts_with` | [170](../../src/client_admin/session_folders.rs#L170) | receiver-type-required |
| `change_session_folder` | `self.repository.resolve_session_seed(&seed, Some(scope.session_folder()), Some(&binding), profile::ScopeSelection { environment:scope.environment(),folders:Some(&folders) }).map_err` | [174](../../src/client_admin/session_folders.rs#L174) | receiver-type-required |
| `change_session_folder` | `self.repository.resolve_session_seed` | [174](../../src/client_admin/session_folders.rs#L174) | receiver-type-required |
| `change_session_folder` | `scope.session_folder` | [174](../../src/client_admin/session_folders.rs#L174), [186](../../src/client_admin/session_folders.rs#L186) | receiver-type-required |
| `change_session_folder` | `scope.environment` | [174](../../src/client_admin/session_folders.rs#L174) | receiver-type-required |
| `change_session_folder` | `catalog.begin_capture(&seed, Some(catalog_revision)).map_err` | [175](../../src/client_admin/session_folders.rs#L175) | receiver-type-required |
| `change_session_folder` | `catalog.begin_capture` | [175](../../src/client_admin/session_folders.rs#L175) | receiver-type-required |
| `change_session_folder` | `internal_profile` | [177](../../src/client_admin/session_folders.rs#L177) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `staged.scope().clone` | [179](../../src/client_admin/session_folders.rs#L179) | receiver-type-required |
| `change_session_folder` | `staged.scope` | [179](../../src/client_admin/session_folders.rs#L179) | receiver-type-required |
| `change_session_folder` | `scope.session_config().is_none_or` | [180](../../src/client_admin/session_folders.rs#L180) | receiver-type-required |
| `change_session_folder` | `chrono::Utc::now().to_rfc3339_opts` | [181](../../src/client_admin/session_folders.rs#L181) | receiver-type-required |
| `change_session_folder` | `chrono::Utc::now` | [181](../../src/client_admin/session_folders.rs#L181) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `schema::Event::from_value(to_ijson(&value)?).map_err` | [185](../../src/client_admin/session_folders.rs#L185) | receiver-type-required |
| `change_session_folder` | `schema::Event::from_value` | [185](../../src/client_admin/session_folders.rs#L185) | [schema::event::Event::from_value](../../../schema/src/event.rs#L179) |
| `change_session_folder` | `to_ijson` | [185](../../src/client_admin/session_folders.rs#L185) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `staged.prepare(&scope.session_folder().join("main.jsonl"), &event).map_err` | [186](../../src/client_admin/session_folders.rs#L186) | receiver-type-required |
| `change_session_folder` | `staged.prepare` | [186](../../src/client_admin/session_folders.rs#L186) | receiver-type-required |
| `change_session_folder` | `scope.session_folder().join` | [186](../../src/client_admin/session_folders.rs#L186) | receiver-type-required |
| `change_session_folder` | `Ok` | [188](../../src/client_admin/session_folders.rs#L188) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `build().map_err` | [190](../../src/client_admin/session_folders.rs#L190) | receiver-type-required |
| `change_session_folder` | `build` | [190](../../src/client_admin/session_folders.rs#L190) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `store::StoreError::Corruption` | [190](../../src/client_admin/session_folders.rs#L190) | external-constructor-callback-or-unresolved |
| `change_session_folder` | `"session folder change rejected".to_owned` | [190](../../src/client_admin/session_folders.rs#L190) | receiver-type-required |
| `change_session_folder` | `capture.finish().map_err` | [195](../../src/client_admin/session_folders.rs#L195) | receiver-type-required |
| `change_session_folder` | `capture.finish` | [195](../../src/client_admin/session_folders.rs#L195) | receiver-type-required |
| `change_session_folder` | `appended.map_err` | [200](../../src/client_admin/session_folders.rs#L200) | receiver-type-required |
| `change_session_folder` | `store             .validated_line_projection(&input.session_id, "main.jsonl")             .map_err` | [208](../../src/client_admin/session_folders.rs#L208) | receiver-type-required |
| `change_session_folder` | `store             .validated_line_projection` | [208](../../src/client_admin/session_folders.rs#L208) | receiver-type-required |
| `change_session_folder` | `projection             .events             .iter()             .find(&#124;event&#124; event.seq() == outcome.seq)             .ok_or_else` | [211](../../src/client_admin/session_folders.rs#L211) | receiver-type-required |
| `change_session_folder` | `projection             .events             .iter()             .find` | [211](../../src/client_admin/session_folders.rs#L211) | receiver-type-required |
| `change_session_folder` | `projection             .events             .iter` | [211](../../src/client_admin/session_folders.rs#L211) | receiver-type-required |
| `complete_session_folder_change` | `request             .handoff             .mark_handed_off(DurableHandoffProof {                 delivery: request.rpc_id.clone(),                 durable_identity: Some(RpcDurableIdentity {                     kind: "ledger".to_owned(),                     id: session.to_owned(),                     seq: Some(seq),                 }),             })             .map_err` | [232](../../src/client_admin/session_folders.rs#L232) | receiver-type-required |
| `complete_session_folder_change` | `request             .handoff             .mark_handed_off` | [232](../../src/client_admin/session_folders.rs#L232) | receiver-type-required |
| `complete_session_folder_change` | `request.rpc_id.clone` | [235](../../src/client_admin/session_folders.rs#L235) | receiver-type-required |
| `complete_session_folder_change` | `Some` | [236](../../src/client_admin/session_folders.rs#L236), [239](../../src/client_admin/session_folders.rs#L239) | external-constructor-callback-or-unresolved |
| `complete_session_folder_change` | `"ledger".to_owned` | [237](../../src/client_admin/session_folders.rs#L237) | receiver-type-required |
| `complete_session_folder_change` | `session.to_owned` | [238](../../src/client_admin/session_folders.rs#L238) | receiver-type-required |
| `complete_session_folder_change` | `internal` | [242](../../src/client_admin/session_folders.rs#L242) | external-constructor-callback-or-unresolved |
| `complete_session_folder_change` | `error.to_string` | [242](../../src/client_admin/session_folders.rs#L242) | receiver-type-required |
| `complete_session_folder_change` | `self.process_host             .reconcile_projection(session)             .map_err` | [243](../../src/client_admin/session_folders.rs#L243) | receiver-type-required |
| `complete_session_folder_change` | `self.process_host             .reconcile_projection` | [243](../../src/client_admin/session_folders.rs#L243) | receiver-type-required |
| `complete_session_folder_change` | `crate::endpoint_host::SessionDeliveryAuthority::session_metadata_changed` | [246](../../src/client_admin/session_folders.rs#L246) | external-constructor-callback-or-unresolved |
| `complete_session_folder_change` | `self.process_host.as_ref` | [247](../../src/client_admin/session_folders.rs#L247) | receiver-type-required |
| `complete_session_folder_change` | `to_ijson` | [250](../../src/client_admin/session_folders.rs#L250) | external-constructor-callback-or-unresolved |
| `request` | `id.into` | [261](../../src/client_admin/session_folders.rs#L261) | receiver-type-required |
| `request` | `method.into` | [262](../../src/client_admin/session_folders.rs#L262) | receiver-type-required |
| `request` | `to_ijson(&payload).unwrap` | [263](../../src/client_admin/session_folders.rs#L263) | receiver-type-required |
| `request` | `to_ijson` | [263](../../src/client_admin/session_folders.rs#L263) | external-constructor-callback-or-unresolved |
| `request` | `DurableHandoffSignal::new` | [265](../../src/client_admin/session_folders.rs#L265) | [endpoint::host::DurableHandoffSignal::new](../../../endpoint/src/host.rs#L202) |
| `create` | `endpoint::ManagementStore::open_light(root)             .unwrap()             .create_session(endpoint::SessionCreateOperation {                 rpc_id: key,                 request_sha256: &"a".repeat(64),                 requested_session_id: None,                 workspace_id: None,                 cwd: Some(&path.to_string_lossy()),                 environment: None,                 identity_profile: None,                 preset: None,                 preset_mcp: None,                 preset_models: None,                 user_agent_dir: agents,                 started_at: "2026-10-16T00:00:00.000Z",                 principal: "test",             })             .unwrap` | [269](../../src/client_admin/session_folders.rs#L269) | receiver-type-required |
| `create` | `endpoint::ManagementStore::open_light(root)             .unwrap()             .create_session` | [269](../../src/client_admin/session_folders.rs#L269) | receiver-type-required |
| `create` | `endpoint::ManagementStore::open_light(root)             .unwrap` | [269](../../src/client_admin/session_folders.rs#L269) | receiver-type-required |
| `create` | `endpoint::ManagementStore::open_light` | [269](../../src/client_admin/session_folders.rs#L269) | [endpoint::management::ManagementStore::open_light](../../../endpoint/src/management.rs#L536) |
| `create` | `"a".repeat` | [273](../../src/client_admin/session_folders.rs#L273) | receiver-type-required |
| `create` | `Some` | [276](../../src/client_admin/session_folders.rs#L276) | external-constructor-callback-or-unresolved |
| `create` | `path.to_string_lossy` | [276](../../src/client_admin/session_folders.rs#L276) | receiver-type-required |
| `json` | `serde_json::to_value(value).unwrap` | [289](../../src/client_admin/session_folders.rs#L289) | receiver-type-required |
| `json` | `serde_json::to_value` | [289](../../src/client_admin/session_folders.rs#L289) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `tempfile::tempdir().unwrap` | [294](../../src/client_admin/session_folders.rs#L294) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `tempfile::tempdir` | [294](../../src/client_admin/session_folders.rs#L294) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `temp.path().join` | [295](../../src/client_admin/session_folders.rs#L295), [296](../../src/client_admin/session_folders.rs#L296), [297](../../src/client_admin/session_folders.rs#L297), [298](../../src/client_admin/session_folders.rs#L298) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `temp.path` | [295](../../src/client_admin/session_folders.rs#L295), [296](../../src/client_admin/session_folders.rs#L296), [297](../../src/client_admin/session_folders.rs#L297), [298](../../src/client_admin/session_folders.rs#L298) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::create_dir_all(path).unwrap` | [300](../../src/client_admin/session_folders.rs#L300) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::create_dir_all` | [300](../../src/client_admin/session_folders.rs#L300) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `ProductionProcessHost::open(&root, "/usr/bin/true", "atomicity", &agents).unwrap` | [303](../../src/client_admin/session_folders.rs#L303), [351](../../src/client_admin/session_folders.rs#L351) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `ProductionProcessHost::open` | [303](../../src/client_admin/session_folders.rs#L303), [351](../../src/client_admin/session_folders.rs#L351) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `ClientAdminRoutes::new(&root, host).unwrap` | [304](../../src/client_admin/session_folders.rs#L304), [352](../../src/client_admin/session_folders.rs#L352) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `ClientAdminRoutes::new` | [304](../../src/client_admin/session_folders.rs#L304), [352](../../src/client_admin/session_folders.rs#L352) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `create` | [305](../../src/client_admin/session_folders.rs#L305) | [tekes-supervisor::client_admin::session_folders::tests::create](../../src/client_admin/session_folders.rs#L268) |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `root.join("threads").join` | [306](../../src/client_admin/session_folders.rs#L306) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `root.join` | [306](../../src/client_admin/session_folders.rs#L306), [308](../../src/client_admin/session_folders.rs#L308), [348](../../src/client_admin/session_folders.rs#L348) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::read(folder.join("main.jsonl")).unwrap` | [307](../../src/client_admin/session_folders.rs#L307), [347](../../src/client_admin/session_folders.rs#L347) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::read` | [307](../../src/client_admin/session_folders.rs#L307), [308](../../src/client_admin/session_folders.rs#L308), [347](../../src/client_admin/session_folders.rs#L347), [348](../../src/client_admin/session_folders.rs#L348) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `folder.join` | [307](../../src/client_admin/session_folders.rs#L307), [347](../../src/client_admin/session_folders.rs#L347) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::read(root.join("config/folders.json")).unwrap` | [308](../../src/client_admin/session_folders.rs#L308), [348](../../src/client_admin/session_folders.rs#L348) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::write(             store::session_settings_path(&folder).unwrap(),             b"invalid settings",         )         .unwrap` | [310](../../src/client_admin/session_folders.rs#L310) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::write` | [310](../../src/client_admin/session_folders.rs#L310) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `store::session_settings_path(&folder).unwrap` | [311](../../src/client_admin/session_folders.rs#L311), [331](../../src/client_admin/session_folders.rs#L331) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `store::session_settings_path` | [311](../../src/client_admin/session_folders.rs#L311), [331](../../src/client_admin/session_folders.rs#L331) | [store::management_root::session_settings_path](../../../store/src/management_root.rs#L57) |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `request` | [315](../../src/client_admin/session_folders.rs#L315), [332](../../src/client_admin/session_folders.rs#L332), [353](../../src/client_admin/session_folders.rs#L353) | [tekes-supervisor::client_admin::session_folders::tests::request](../../src/client_admin/session_folders.rs#L259) |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::remove_file(store::session_settings_path(&folder).unwrap()).unwrap` | [331](../../src/client_admin/session_folders.rs#L331) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `fs::remove_file` | [331](../../src/client_admin/session_folders.rs#L331) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `json` | [337](../../src/client_admin/session_folders.rs#L337) | [tekes-supervisor::client_admin::session_folders::tests::json](../../src/client_admin/session_folders.rs#L288) |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `routes                 .execute(                     &retry,                     &serde_json::to_value(&retry.payload).unwrap(),                     "test",                 )                 .unwrap` | [338](../../src/client_admin/session_folders.rs#L338) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `routes                 .execute` | [338](../../src/client_admin/session_folders.rs#L338) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `serde_json::to_value(&retry.payload).unwrap` | [341](../../src/client_admin/session_folders.rs#L341) | receiver-type-required |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `serde_json::to_value` | [341](../../src/client_admin/session_folders.rs#L341) | external-constructor-callback-or-unresolved |
| `rejected_folder_edit_does_not_publish_catalog_or_ledger` | `drop` | [349](../../src/client_admin/session_folders.rs#L349) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `tempfile::tempdir().unwrap` | [380](../../src/client_admin/session_folders.rs#L380) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `tempfile::tempdir` | [380](../../src/client_admin/session_folders.rs#L380) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `temp.path().join` | [381](../../src/client_admin/session_folders.rs#L381), [382](../../src/client_admin/session_folders.rs#L382), [383](../../src/client_admin/session_folders.rs#L383), [384](../../src/client_admin/session_folders.rs#L384) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `temp.path` | [381](../../src/client_admin/session_folders.rs#L381), [382](../../src/client_admin/session_folders.rs#L382), [383](../../src/client_admin/session_folders.rs#L383), [384](../../src/client_admin/session_folders.rs#L384) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::create_dir_all(path).unwrap` | [386](../../src/client_admin/session_folders.rs#L386) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::create_dir_all` | [386](../../src/client_admin/session_folders.rs#L386) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `a.canonicalize().unwrap` | [388](../../src/client_admin/session_folders.rs#L388) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `a.canonicalize` | [388](../../src/client_admin/session_folders.rs#L388) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `b.canonicalize().unwrap` | [389](../../src/client_admin/session_folders.rs#L389) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `b.canonicalize` | [389](../../src/client_admin/session_folders.rs#L389) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `ProductionProcessHost::open(&root, "/usr/bin/true", "phase7", &agents).unwrap` | [390](../../src/client_admin/session_folders.rs#L390) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `ProductionProcessHost::open` | [390](../../src/client_admin/session_folders.rs#L390) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `ClientAdminRoutes::new(&root, host).unwrap` | [391](../../src/client_admin/session_folders.rs#L391) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `ClientAdminRoutes::new` | [391](../../src/client_admin/session_folders.rs#L391) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `create` | [392](../../src/client_admin/session_folders.rs#L392), [393](../../src/client_admin/session_folders.rs#L393) | [tekes-supervisor::client_admin::session_folders::tests::create](../../src/client_admin/session_folders.rs#L268) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `profile::SessionScope::locate(&root, &first).unwrap` | [394](../../src/client_admin/session_folders.rs#L394) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `profile::SessionScope::locate` | [394](../../src/client_admin/session_folders.rs#L394), [553](../../src/client_admin/session_folders.rs#L553) | [profile::scope::SessionScope::locate](../../../profile/src/scope.rs#L34) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `scope.workspace_id().to_owned` | [396](../../src/client_admin/session_folders.rs#L396) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `scope.workspace_id` | [396](../../src/client_admin/session_folders.rs#L396) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.repository.workspace(&workspace).unwrap` | [397](../../src/client_admin/session_folders.rs#L397) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.repository.workspace` | [397](../../src/client_admin/session_folders.rs#L397) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `compatibility.policy.as_mut().unwrap` | [399](../../src/client_admin/session_folders.rs#L399) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `compatibility.policy.as_mut` | [399](../../src/client_admin/session_folders.rs#L399) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `compatibility.folders.push` | [400](../../src/client_admin/session_folders.rs#L400) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `"other".into` | [401](../../src/client_admin/session_folders.rs#L401) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `b.to_string_lossy().into_owned` | [402](../../src/client_admin/session_folders.rs#L402) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `b.to_string_lossy` | [402](../../src/client_admin/session_folders.rs#L402) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes             .repository             .publish_workspace(compatibility.revision - 1, &compatibility)             .unwrap` | [404](../../src/client_admin/session_folders.rs#L404) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes             .repository             .publish_workspace` | [404](../../src/client_admin/session_folders.rs#L404) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `scope.config(&routes.repository).unwrap` | [408](../../src/client_admin/session_folders.rs#L408) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `scope.config` | [408](../../src/client_admin/session_folders.rs#L408) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::remove_dir_all(root.join("workspaces").join(&workspace)).unwrap` | [411](../../src/client_admin/session_folders.rs#L411) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::remove_dir_all` | [411](../../src/client_admin/session_folders.rs#L411) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `root.join("workspaces").join` | [411](../../src/client_admin/session_folders.rs#L411) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `root.join` | [411](../../src/client_admin/session_folders.rs#L411), [524](../../src/client_admin/session_folders.rs#L524) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `request` | [413](../../src/client_admin/session_folders.rs#L413), [425](../../src/client_admin/session_folders.rs#L425), [459](../../src/client_admin/session_folders.rs#L459), [476](../../src/client_admin/session_folders.rs#L476), [492](../../src/client_admin/session_folders.rs#L492), [541](../../src/client_admin/session_folders.rs#L541), [578](../../src/client_admin/session_folders.rs#L578) | [tekes-supervisor::client_admin::session_folders::tests::request](../../src/client_admin/session_folders.rs#L259) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `json` | [418](../../src/client_admin/session_folders.rs#L418), [569](../../src/client_admin/session_folders.rs#L569) | [tekes-supervisor::client_admin::session_folders::tests::json](../../src/client_admin/session_folders.rs#L288) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes                 .execute(&add, &serde_json::to_value(&add.payload).unwrap(), "test")                 .unwrap` | [419](../../src/client_admin/session_folders.rs#L419) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes                 .execute` | [419](../../src/client_admin/session_folders.rs#L419) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `serde_json::to_value(&add.payload).unwrap` | [420](../../src/client_admin/session_folders.rs#L420) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `serde_json::to_value` | [420](../../src/client_admin/session_folders.rs#L420), [549](../../src/client_admin/session_folders.rs#L549) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `NamedLock::shared(crate::process_host::workspace_quiescence_lock_path(             &root,             &format!("session:{first}"),         ))         .unwrap` | [442](../../src/client_admin/session_folders.rs#L442) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `NamedLock::shared` | [442](../../src/client_admin/session_folders.rs#L442) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `crate::process_host::workspace_quiescence_lock_path` | [442](../../src/client_admin/session_folders.rs#L442) | [tekes-supervisor::process_host::workspace_quiescence_lock_path](../../src/process_host.rs#L6541) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `drop` | [475](../../src/client_admin/session_folders.rs#L475) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::read(root.join("threads").join(&first).join("main.jsonl")).unwrap` | [524](../../src/client_admin/session_folders.rs#L524) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `fs::read` | [524](../../src/client_admin/session_folders.rs#L524) | external-constructor-callback-or-unresolved |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `root.join("threads").join(&first).join` | [524](../../src/client_admin/session_folders.rs#L524) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `root.join("threads").join` | [524](../../src/client_admin/session_folders.rs#L524) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store::ThreadStore::open(&root).unwrap` | [527](../../src/client_admin/session_folders.rs#L527) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store::ThreadStore::open` | [527](../../src/client_admin/session_folders.rs#L527) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store             .begin_fork("phase7-fork", &first, fork, "2026-10-16T01:00:00.000Z")             .unwrap` | [529](../../src/client_admin/session_folders.rs#L529) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store             .begin_fork` | [529](../../src/client_admin/session_folders.rs#L529) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store.recover_rewrites().unwrap` | [532](../../src/client_admin/session_folders.rs#L532) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `store.recover_rewrites` | [532](../../src/client_admin/session_folders.rs#L532) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes             .execute(                 &remove,                 &serde_json::to_value(&remove.payload).unwrap(),                 "test",             )             .unwrap` | [546](../../src/client_admin/session_folders.rs#L546) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes             .execute` | [546](../../src/client_admin/session_folders.rs#L546) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `serde_json::to_value(&remove.payload).unwrap` | [549](../../src/client_admin/session_folders.rs#L549) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `profile::SessionScope::locate(&root, &first)             .unwrap()             .config(&routes.repository)             .unwrap` | [553](../../src/client_admin/session_folders.rs#L553) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `profile::SessionScope::locate(&root, &first)             .unwrap()             .config` | [553](../../src/client_admin/session_folders.rs#L553) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `profile::SessionScope::locate(&root, &first)             .unwrap` | [553](../../src/client_admin/session_folders.rs#L553) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.process_host.inventory_items(true).unwrap().unwrap` | [562](../../src/client_admin/session_folders.rs#L562) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.process_host.inventory_items(true).unwrap` | [562](../../src/client_admin/session_folders.rs#L562) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.process_host.inventory_items` | [562](../../src/client_admin/session_folders.rs#L562) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `inventory             .iter()             .find(&#124;row&#124; row.session_id == first)             .unwrap` | [563](../../src/client_admin/session_folders.rs#L563) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `inventory             .iter()             .find` | [563](../../src/client_admin/session_folders.rs#L563) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `inventory             .iter` | [563](../../src/client_admin/session_folders.rs#L563) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.management().unwrap().folder_paths().unwrap` | [567](../../src/client_admin/session_folders.rs#L567) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.management().unwrap().folder_paths` | [567](../../src/client_admin/session_folders.rs#L567) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.management().unwrap` | [567](../../src/client_admin/session_folders.rs#L567) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes.management` | [567](../../src/client_admin/session_folders.rs#L567) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `json(             routes                 .session_folders(SessionFoldersGet {                     session_id: first.clone(),                 })                 .unwrap(),         )["revision"]             .as_u64()             .unwrap` | [569](../../src/client_admin/session_folders.rs#L569) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `json(             routes                 .session_folders(SessionFoldersGet {                     session_id: first.clone(),                 })                 .unwrap(),         )["revision"]             .as_u64` | [569](../../src/client_admin/session_folders.rs#L569) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes                 .session_folders(SessionFoldersGet {                     session_id: first.clone(),                 })                 .unwrap` | [570](../../src/client_admin/session_folders.rs#L570) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `routes                 .session_folders` | [570](../../src/client_admin/session_folders.rs#L570) | receiver-type-required |
| `phase7_session_scope_survives_workspace_changes_and_folder_edits_are_local_durable_and_inherited` | `first.clone` | [572](../../src/client_admin/session_folders.rs#L572) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `tempfile::tempdir().unwrap` | [602](../../src/client_admin/session_folders.rs#L602) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `tempfile::tempdir` | [602](../../src/client_admin/session_folders.rs#L602) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `temp.path().join` | [603](../../src/client_admin/session_folders.rs#L603), [604](../../src/client_admin/session_folders.rs#L604), [605](../../src/client_admin/session_folders.rs#L605), [606](../../src/client_admin/session_folders.rs#L606), [607](../../src/client_admin/session_folders.rs#L607) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `temp.path` | [603](../../src/client_admin/session_folders.rs#L603), [604](../../src/client_admin/session_folders.rs#L604), [605](../../src/client_admin/session_folders.rs#L605), [606](../../src/client_admin/session_folders.rs#L606), [607](../../src/client_admin/session_folders.rs#L607) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `fs::create_dir_all(path).unwrap` | [609](../../src/client_admin/session_folders.rs#L609) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `fs::create_dir_all` | [609](../../src/client_admin/session_folders.rs#L609) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `ProductionProcessHost::open(&root, "/usr/bin/true", "phase7", &agents).unwrap` | [611](../../src/client_admin/session_folders.rs#L611) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `ProductionProcessHost::open` | [611](../../src/client_admin/session_folders.rs#L611) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `ClientAdminRoutes::new(&root, host).unwrap` | [612](../../src/client_admin/session_folders.rs#L612) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `ClientAdminRoutes::new` | [612](../../src/client_admin/session_folders.rs#L612) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `create` | [613](../../src/client_admin/session_folders.rs#L613) | [tekes-supervisor::client_admin::session_folders::tests::create](../../src/client_admin/session_folders.rs#L268) |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `[Path::new("/"), root.as_path()].into_iter().enumerate` | [614](../../src/client_admin/session_folders.rs#L614) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `[Path::new("/"), root.as_path()].into_iter` | [614](../../src/client_admin/session_folders.rs#L614) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `Path::new` | [614](../../src/client_admin/session_folders.rs#L614) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `root.as_path` | [614](../../src/client_admin/session_folders.rs#L614) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `request` | [615](../../src/client_admin/session_folders.rs#L615), [629](../../src/client_admin/session_folders.rs#L629), [644](../../src/client_admin/session_folders.rs#L644) | [tekes-supervisor::client_admin::session_folders::tests::request](../../src/client_admin/session_folders.rs#L259) |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `routes             .execute(&add, &serde_json::to_value(&add.payload).unwrap(), "test")             .unwrap` | [634](../../src/client_admin/session_folders.rs#L634) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `routes             .execute` | [634](../../src/client_admin/session_folders.rs#L634), [649](../../src/client_admin/session_folders.rs#L649), [656](../../src/client_admin/session_folders.rs#L656) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `serde_json::to_value(&add.payload).unwrap` | [635](../../src/client_admin/session_folders.rs#L635) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `serde_json::to_value` | [635](../../src/client_admin/session_folders.rs#L635), [652](../../src/client_admin/session_folders.rs#L652), [659](../../src/client_admin/session_folders.rs#L659) | external-constructor-callback-or-unresolved |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `profile::FolderCatalog::new` | [637](../../src/client_admin/session_folders.rs#L637) | [profile::folder_catalog::FolderCatalog::new](../../../profile/src/folder_catalog.rs#L138) |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders()             .unwrap()             .into_iter()             .find(&#124;folder&#124; folder.path == b.canonicalize().unwrap().to_string_lossy())             .unwrap` | [638](../../src/client_admin/session_folders.rs#L638) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders()             .unwrap()             .into_iter()             .find` | [638](../../src/client_admin/session_folders.rs#L638), [663](../../src/client_admin/session_folders.rs#L663) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders()             .unwrap()             .into_iter` | [638](../../src/client_admin/session_folders.rs#L638), [663](../../src/client_admin/session_folders.rs#L663) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders()             .unwrap` | [638](../../src/client_admin/session_folders.rs#L638), [663](../../src/client_admin/session_folders.rs#L663) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders` | [638](../../src/client_admin/session_folders.rs#L638), [663](../../src/client_admin/session_folders.rs#L663) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `b.canonicalize().unwrap().to_string_lossy` | [642](../../src/client_admin/session_folders.rs#L642) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `b.canonicalize().unwrap` | [642](../../src/client_admin/session_folders.rs#L642) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `b.canonicalize` | [642](../../src/client_admin/session_folders.rs#L642) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `routes             .execute(                 &relocate,                 &serde_json::to_value(&relocate.payload).unwrap(),                 "test",             )             .unwrap` | [649](../../src/client_admin/session_folders.rs#L649), [656](../../src/client_admin/session_folders.rs#L656) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `serde_json::to_value(&relocate.payload).unwrap` | [652](../../src/client_admin/session_folders.rs#L652), [659](../../src/client_admin/session_folders.rs#L659) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `catalog             .folders()             .unwrap()             .into_iter()             .find(&#124;folder&#124; folder.id == before.id)             .unwrap` | [663](../../src/client_admin/session_folders.rs#L663) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `profile::SessionScope::locate(&root, &session)             .unwrap()             .config(&routes.repository)             .unwrap` | [676](../../src/client_admin/session_folders.rs#L676) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `profile::SessionScope::locate(&root, &session)             .unwrap()             .config` | [676](../../src/client_admin/session_folders.rs#L676) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `profile::SessionScope::locate(&root, &session)             .unwrap` | [676](../../src/client_admin/session_folders.rs#L676) | receiver-type-required |
| `phase7_folder_guards_and_catalog_only_relocation_keep_identity_and_do_not_grant_trust` | `profile::SessionScope::locate` | [676](../../src/client_admin/session_folders.rs#L676) | [profile::scope::SessionScope::locate](../../../profile/src/scope.rs#L34) |
