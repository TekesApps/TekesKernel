# profile::folder_authority

[Package atlas](index.md) · [Source](../../src/folder_authority.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [profile::folder_authority::FolderIdentity](../../src/folder_authority.rs#L15) | struct_item | `pub` |  |
| [profile::folder_authority::FolderTrust](../../src/folder_authority.rs#L25) | struct_item | `pub` |  |
| [profile::folder_authority::FolderTrustReceipt](../../src/folder_authority.rs#L35) | struct_item | `pub` |  |
| [profile::folder_authority::RecordedOperation](../../src/folder_authority.rs#L42) | struct_item | `private` |  |
| [profile::folder_authority::AuthorityFile](../../src/folder_authority.rs#L49) | struct_item | `private` |  |
| [profile::folder_authority::AuthorityFile::default](../../src/folder_authority.rs#L58) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore](../../src/folder_authority.rs#L70) | struct_item | `pub` |  |
| [profile::folder_authority::FolderEffectLease](../../src/folder_authority.rs#L76) | struct_item | `pub` |  |
| [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore::new](../../src/folder_authority.rs#L86) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::directory](../../src/folder_authority.rs#L92) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore::path](../../src/folder_authority.rs#L95) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore::status](../../src/folder_authority.rs#L133) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::record](../../src/folder_authority.rs#L145) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::registry_revision](../../src/folder_authority.rs#L159) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::pending](../../src/folder_authority.rs#L163) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::request](../../src/folder_authority.rs#L171) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::prior_grant](../../src/folder_authority.rs#L207) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::grant_from_host](../../src/folder_authority.rs#L226) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::revoke](../../src/folder_authority.rs#L270) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::mutate](../../src/folder_authority.rs#L295) | function_item | `private` |  |
| [profile::folder_authority::FolderAuthorityStore::admit](../../src/folder_authority.rs#L362) | function_item | `pub` |  |
| [profile::folder_authority::FolderAuthorityStore::effects_drained](../../src/folder_authority.rs#L379) | function_item | `pub` |  |
| [profile::folder_authority::valid_id](../../src/folder_authority.rs#L391) | function_item | `private` |  |
| [profile::folder_authority::identify](../../src/folder_authority.rs#L395) | function_item | `private` |  |
| [profile::folder_authority::tests::grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission](../../src/folder_authority.rs#L435) | function_item | `private` | test; #[cfg(test)] |
| [profile::folder_authority::tests::aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants](../../src/folder_authority.rs#L463) | function_item | `private` | test; #[cfg(test)] |
| [profile::folder_authority::tests::retained_receipt_capacity_cannot_disable_revocation](../../src/folder_authority.rs#L481) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `fs` | `std::fs` | `private` |
| `OpenOptions` | `std::fs::OpenOptions` | `private` |
| `io` | `std::io` | `private` |
| `Read` | `std::io::Read` | `private` |
| `MetadataExt` | `std::os::unix::fs::MetadataExt` | `private` |
| `OpenOptionsExt` | `std::os::unix::fs::OpenOptionsExt` | `private` |
| `Path` | `std::path::Path` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `UNIX_EPOCH` | `std::time::UNIX_EPOCH` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `AtomicPublisher` | `store::AtomicPublisher` | `private` |
| `NamedLock` | `store::NamedLock` | `private` |
| `*` | `super::*` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `profile::folder_authority::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–19: 43 direct edges</summary>

```mermaid
flowchart TD
  n0["profile::config::ConfigRepository::open"]
  n1["profile::folder_authority::FolderAuthorityStore::status"]
  n2["profile::folder_authority::FolderAuthorityStore::record"]
  n3["profile::folder_authority::FolderAuthorityStore::registry_revision"]
  n4["profile::folder_authority::FolderAuthorityStore::pending"]
  n5["profile::folder_authority::FolderAuthorityStore::request"]
  n6["profile::folder_authority::FolderAuthorityStore::prior_grant"]
  n7["profile::folder_authority::FolderAuthorityStore::grant_from_host"]
  n8["profile::folder_authority::FolderAuthorityStore::revoke"]
  n9["profile::folder_authority::FolderAuthorityStore::mutate"]
  n10["profile::folder_authority::FolderAuthorityStore::admit"]
  n11["profile::folder_authority::FolderAuthorityStore::effects_drained"]
  n12["profile::folder_authority::valid_id"]
  n13["profile::folder_authority::identify"]
  n14["profile::folder_authority::AuthorityFile::default"]
  n15["profile::folder_authority::invalid"]
  n16["profile::folder_authority::FolderAuthorityStore::new"]
  n17["profile::folder_authority::FolderAuthorityStore::directory"]
  n18["profile::folder_authority::FolderAuthorityStore::path"]
  n19["profile::folder_authority::FolderAuthorityStore::load"]
  n20["profile::digest"]
  n21["store::atomic::AtomicPublisher::replace"]
  n22["store::platform::NamedLock::exclusive"]
  n23["store::platform::NamedLock::try_exclusive"]
  n24["store::platform::NamedLock::shared"]
  n25["tools::sandbox::kernel_protected_roots"]
  n1 --> n13
  n1 --> n19
  n2 --> n1
  n2 --> n15
  n2 --> n19
  n3 --> n19
  n4 --> n15
  n4 --> n19
  n5 --> n9
  n5 --> n13
  n5 --> n15
  n6 --> n15
  n6 --> n19
  n6 --> n20
  n7 --> n9
  n7 --> n13
  n7 --> n15
  n7 --> n25
  n8 --> n9
  n8 --> n15
  n9 --> n0
  n9 --> n15
  n9 --> n17
  n9 --> n18
  n9 --> n19
  n9 --> n20
  n9 --> n21
  n9 --> n22
  n10 --> n1
  n10 --> n12
  n10 --> n15
  n10 --> n17
  n10 --> n24
  n11 --> n12
  n11 --> n15
  n11 --> n17
  n11 --> n23
  n13 --> n15
  n13 --> n20
  n18 --> n17
  n19 --> n12
  n19 --> n15
  n19 --> n18
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `default` | `BTreeMap::new` | [62](../../src/folder_authority.rs#L62), [63](../../src/folder_authority.rs#L63), [64](../../src/folder_authority.rs#L64) | external-constructor-callback-or-unresolved |
| `invalid` | `io::Error::new` | [81](../../src/folder_authority.rs#L81) | external-constructor-callback-or-unresolved |
| `new` | `config_root.into` | [88](../../src/folder_authority.rs#L88) | receiver-type-required |
| `directory` | `self.root.join` | [93](../../src/folder_authority.rs#L93) | receiver-type-required |
| `path` | `self.directory().join` | [96](../../src/folder_authority.rs#L96) | receiver-type-required |
| `path` | `self.directory` | [96](../../src/folder_authority.rs#L96) | [profile::folder_authority::FolderAuthorityStore::directory](../../src/folder_authority.rs#L92) |
| `load` | `OpenOptions::new()             .read(true)             .custom_flags(libc::O_NOFOLLOW &#124; libc::O_CLOEXEC)             .open` | [100](../../src/folder_authority.rs#L100) | receiver-type-required |
| `load` | `OpenOptions::new()             .read(true)             .custom_flags` | [100](../../src/folder_authority.rs#L100) | receiver-type-required |
| `load` | `OpenOptions::new()             .read` | [100](../../src/folder_authority.rs#L100) | receiver-type-required |
| `load` | `OpenOptions::new` | [100](../../src/folder_authority.rs#L100) | external-constructor-callback-or-unresolved |
| `load` | `self.path` | [103](../../src/folder_authority.rs#L103) | [profile::folder_authority::FolderAuthorityStore::path](../../src/folder_authority.rs#L95) |
| `load` | `error.kind` | [106](../../src/folder_authority.rs#L106) | receiver-type-required |
| `load` | `Ok` | [107](../../src/folder_authority.rs#L107), [129](../../src/folder_authority.rs#L129) | external-constructor-callback-or-unresolved |
| `load` | `AuthorityFile::default` | [107](../../src/folder_authority.rs#L107) | external-constructor-callback-or-unresolved |
| `load` | `Err` | [109](../../src/folder_authority.rs#L109), [112](../../src/folder_authority.rs#L112), [119](../../src/folder_authority.rs#L119), [127](../../src/folder_authority.rs#L127) | external-constructor-callback-or-unresolved |
| `load` | `file.metadata()?.is_file` | [111](../../src/folder_authority.rs#L111) | receiver-type-required |
| `load` | `file.metadata` | [111](../../src/folder_authority.rs#L111) | receiver-type-required |
| `load` | `file.metadata()?.len` | [111](../../src/folder_authority.rs#L111) | receiver-type-required |
| `load` | `invalid` | [112](../../src/folder_authority.rs#L112), [117](../../src/folder_authority.rs#L117), [119](../../src/folder_authority.rs#L119), [127](../../src/folder_authority.rs#L127) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `load` | `Vec::new` | [114](../../src/folder_authority.rs#L114) | external-constructor-callback-or-unresolved |
| `load` | `file.read_to_end` | [115](../../src/folder_authority.rs#L115) | receiver-type-required |
| `load` | `serde_json::from_slice(&bytes).map_err` | [117](../../src/folder_authority.rs#L117) | receiver-type-required |
| `load` | `serde_json::from_slice` | [117](../../src/folder_authority.rs#L117) | external-constructor-callback-or-unresolved |
| `load` | `serde_json_canonicalizer::to_vec(&value).map_err` | [118](../../src/folder_authority.rs#L118) | receiver-type-required |
| `load` | `serde_json_canonicalizer::to_vec` | [118](../../src/folder_authority.rs#L118) | external-constructor-callback-or-unresolved |
| `load` | `value                 .folders                 .iter()                 .any` | [122](../../src/folder_authority.rs#L122) | receiver-type-required |
| `load` | `value                 .folders                 .iter` | [122](../../src/folder_authority.rs#L122) | receiver-type-required |
| `load` | `valid_id` | [125](../../src/folder_authority.rs#L125) | [profile::folder_authority::valid_id](../../src/folder_authority.rs#L391) |
| `status` | `identify` | [134](../../src/folder_authority.rs#L134) | [profile::folder_authority::identify](../../src/folder_authority.rs#L395) |
| `status` | `self.load` | [135](../../src/folder_authority.rs#L135) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `status` | `state.folders.get` | [136](../../src/folder_authority.rs#L136) | receiver-type-required |
| `status` | `record.clone` | [140](../../src/folder_authority.rs#L140) | receiver-type-required |
| `status` | `Ok` | [142](../../src/folder_authority.rs#L142) | external-constructor-callback-or-unresolved |
| `record` | `self             .load()?             .folders             .get(id)             .cloned()             .ok_or_else` | [146](../../src/folder_authority.rs#L146) | receiver-type-required |
| `record` | `self             .load()?             .folders             .get(id)             .cloned` | [146](../../src/folder_authority.rs#L146) | receiver-type-required |
| `record` | `self             .load()?             .folders             .get` | [146](../../src/folder_authority.rs#L146) | receiver-type-required |
| `record` | `self             .load` | [146](../../src/folder_authority.rs#L146) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `record` | `invalid` | [151](../../src/folder_authority.rs#L151), [154](../../src/folder_authority.rs#L154) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `record` | `self.status` | [152](../../src/folder_authority.rs#L152) | [profile::folder_authority::FolderAuthorityStore::status](../../src/folder_authority.rs#L133) |
| `record` | `Path::new` | [152](../../src/folder_authority.rs#L152) | external-constructor-callback-or-unresolved |
| `record` | `Err` | [154](../../src/folder_authority.rs#L154) | external-constructor-callback-or-unresolved |
| `record` | `Ok` | [156](../../src/folder_authority.rs#L156) | external-constructor-callback-or-unresolved |
| `registry_revision` | `Ok` | [160](../../src/folder_authority.rs#L160) | external-constructor-callback-or-unresolved |
| `registry_revision` | `self.load` | [160](../../src/folder_authority.rs#L160) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `pending` | `self.load()?             .requests             .get(request)             .cloned()             .ok_or_else` | [164](../../src/folder_authority.rs#L164) | receiver-type-required |
| `pending` | `self.load()?             .requests             .get(request)             .cloned` | [164](../../src/folder_authority.rs#L164) | receiver-type-required |
| `pending` | `self.load()?             .requests             .get` | [164](../../src/folder_authority.rs#L164) | receiver-type-required |
| `pending` | `self.load` | [164](../../src/folder_authority.rs#L164) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `pending` | `invalid` | [168](../../src/folder_authority.rs#L168) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `request` | `identify` | [177](../../src/folder_authority.rs#L177) | [profile::folder_authority::identify](../../src/folder_authority.rs#L395) |
| `request` | `self.mutate` | [178](../../src/folder_authority.rs#L178) | [profile::folder_authority::FolderAuthorityStore::mutate](../../src/folder_authority.rs#L295) |
| `request` | `folder.clone` | [183](../../src/folder_authority.rs#L183) | receiver-type-required |
| `request` | `state.folders.get` | [184](../../src/folder_authority.rs#L184) | receiver-type-required |
| `request` | `existing.clone` | [186](../../src/folder_authority.rs#L186) | receiver-type-required |
| `request` | `existing                             .revision                             .checked_add(1)                             .ok_or_else` | [188](../../src/folder_authority.rs#L188) | receiver-type-required |
| `request` | `existing                             .revision                             .checked_add` | [188](../../src/folder_authority.rs#L188) | receiver-type-required |
| `request` | `invalid` | [191](../../src/folder_authority.rs#L191), [195](../../src/folder_authority.rs#L195) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `request` | `state.folders.contains_key` | [194](../../src/folder_authority.rs#L194) | receiver-type-required |
| `request` | `state.folders.len` | [194](../../src/folder_authority.rs#L194) | receiver-type-required |
| `request` | `Err` | [195](../../src/folder_authority.rs#L195) | external-constructor-callback-or-unresolved |
| `request` | `state.folders.insert` | [197](../../src/folder_authority.rs#L197) | receiver-type-required |
| `request` | `current.id.clone` | [197](../../src/folder_authority.rs#L197) | receiver-type-required |
| `request` | `current.clone` | [197](../../src/folder_authority.rs#L197), [199](../../src/folder_authority.rs#L199) | receiver-type-required |
| `request` | `state.requests.retain` | [198](../../src/folder_authority.rs#L198) | receiver-type-required |
| `request` | `state.requests.insert` | [199](../../src/folder_authority.rs#L199) | receiver-type-required |
| `request` | `operation.to_owned` | [199](../../src/folder_authority.rs#L199) | receiver-type-required |
| `request` | `Ok` | [200](../../src/folder_authority.rs#L200) | external-constructor-callback-or-unresolved |
| `prior_grant` | `crate::digest` | [213](../../src/folder_authority.rs#L213) | [profile::digest](../../src/lib.rs#L130) |
| `prior_grant` | `serde_json_canonicalizer::to_vec(                 &serde_json::json!({"expected":expected,"request":{"grant":request}}),             )             .map_err` | [214](../../src/folder_authority.rs#L214) | receiver-type-required |
| `prior_grant` | `serde_json_canonicalizer::to_vec` | [214](../../src/folder_authority.rs#L214) | external-constructor-callback-or-unresolved |
| `prior_grant` | `self.load()?.operations.get` | [219](../../src/folder_authority.rs#L219) | receiver-type-required |
| `prior_grant` | `self.load` | [219](../../src/folder_authority.rs#L219) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `prior_grant` | `Ok` | [220](../../src/folder_authority.rs#L220), [222](../../src/folder_authority.rs#L222) | external-constructor-callback-or-unresolved |
| `prior_grant` | `Some` | [220](../../src/folder_authority.rs#L220) | external-constructor-callback-or-unresolved |
| `prior_grant` | `prior.receipt.clone` | [220](../../src/folder_authority.rs#L220) | receiver-type-required |
| `prior_grant` | `Err` | [221](../../src/folder_authority.rs#L221) | external-constructor-callback-or-unresolved |
| `prior_grant` | `invalid` | [221](../../src/folder_authority.rs#L221) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `grant_from_host` | `self.mutate` | [232](../../src/folder_authority.rs#L232) | [profile::folder_authority::FolderAuthorityStore::mutate](../../src/folder_authority.rs#L295) |
| `grant_from_host` | `state                     .requests                     .get(request)                     .ok_or_else(&#124;&#124; invalid("folder-request-not-found"))?                     .clone` | [237](../../src/folder_authority.rs#L237) | receiver-type-required |
| `grant_from_host` | `state                     .requests                     .get(request)                     .ok_or_else` | [237](../../src/folder_authority.rs#L237) | receiver-type-required |
| `grant_from_host` | `state                     .requests                     .get` | [237](../../src/folder_authority.rs#L237) | receiver-type-required |
| `grant_from_host` | `invalid` | [240](../../src/folder_authority.rs#L240), [246](../../src/folder_authority.rs#L246), [251](../../src/folder_authority.rs#L251), [255](../../src/folder_authority.rs#L255), [262](../../src/folder_authority.rs#L262) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `grant_from_host` | `identify` | [242](../../src/folder_authority.rs#L242) | [profile::folder_authority::identify](../../src/folder_authority.rs#L395) |
| `grant_from_host` | `Path::new` | [242](../../src/folder_authority.rs#L242), [254](../../src/folder_authority.rs#L254) | external-constructor-callback-or-unresolved |
| `grant_from_host` | `state                     .folders                     .get_mut(&requested.id)                     .ok_or_else` | [243](../../src/folder_authority.rs#L243) | receiver-type-required |
| `grant_from_host` | `state                     .folders                     .get_mut` | [243](../../src/folder_authority.rs#L243) | receiver-type-required |
| `grant_from_host` | `Err` | [251](../../src/folder_authority.rs#L251), [255](../../src/folder_authority.rs#L255) | external-constructor-callback-or-unresolved |
| `grant_from_host` | `tools::kernel_protected_roots` | [253](../../src/folder_authority.rs#L253) | [tools::sandbox::kernel_protected_roots](../../../tools/src/sandbox.rs#L151) |
| `grant_from_host` | `Path::new(&requested.path).starts_with` | [254](../../src/folder_authority.rs#L254) | receiver-type-required |
| `grant_from_host` | `current                     .revision                     .checked_add(1)                     .ok_or_else` | [259](../../src/folder_authority.rs#L259) | receiver-type-required |
| `grant_from_host` | `current                     .revision                     .checked_add` | [259](../../src/folder_authority.rs#L259) | receiver-type-required |
| `grant_from_host` | `current.clone` | [263](../../src/folder_authority.rs#L263) | receiver-type-required |
| `grant_from_host` | `state.requests.remove` | [264](../../src/folder_authority.rs#L264) | receiver-type-required |
| `grant_from_host` | `Ok` | [265](../../src/folder_authority.rs#L265) | external-constructor-callback-or-unresolved |
| `revoke` | `self.mutate` | [276](../../src/folder_authority.rs#L276) | [profile::folder_authority::FolderAuthorityStore::mutate](../../src/folder_authority.rs#L295) |
| `revoke` | `state                     .folders                     .get_mut(folder)                     .ok_or_else` | [281](../../src/folder_authority.rs#L281) | receiver-type-required |
| `revoke` | `state                     .folders                     .get_mut` | [281](../../src/folder_authority.rs#L281) | receiver-type-required |
| `revoke` | `invalid` | [284](../../src/folder_authority.rs#L284), [289](../../src/folder_authority.rs#L289) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `revoke` | `current                     .revision                     .checked_add(1)                     .ok_or_else` | [286](../../src/folder_authority.rs#L286) | receiver-type-required |
| `revoke` | `current                     .revision                     .checked_add` | [286](../../src/folder_authority.rs#L286) | receiver-type-required |
| `revoke` | `Ok` | [290](../../src/folder_authority.rs#L290) | external-constructor-callback-or-unresolved |
| `revoke` | `current.clone` | [290](../../src/folder_authority.rs#L290) | receiver-type-required |
| `mutate` | `operation.is_empty` | [302](../../src/folder_authority.rs#L302) | receiver-type-required |
| `mutate` | `operation.len` | [302](../../src/folder_authority.rs#L302) | receiver-type-required |
| `mutate` | `operation.chars().any` | [302](../../src/folder_authority.rs#L302) | receiver-type-required |
| `mutate` | `operation.chars` | [302](../../src/folder_authority.rs#L302) | receiver-type-required |
| `mutate` | `Err` | [304](../../src/folder_authority.rs#L304), [321](../../src/folder_authority.rs#L321), [325](../../src/folder_authority.rs#L325), [356](../../src/folder_authority.rs#L356) | external-constructor-callback-or-unresolved |
| `mutate` | `invalid` | [304](../../src/folder_authority.rs#L304), [321](../../src/folder_authority.rs#L321), [325](../../src/folder_authority.rs#L325), [331](../../src/folder_authority.rs#L331), [351](../../src/folder_authority.rs#L351), [356](../../src/folder_authority.rs#L356) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `mutate` | `crate::ConfigRepository::open(&self.root).map_err` | [307](../../src/folder_authority.rs#L307) | receiver-type-required |
| `mutate` | `crate::ConfigRepository::open` | [307](../../src/folder_authority.rs#L307) | [profile::config::ConfigRepository::open](../../src/config.rs#L814) |
| `mutate` | `NamedLock::exclusive(self.directory().join(".folder-authority.lock"))             .map_err` | [308](../../src/folder_authority.rs#L308) | receiver-type-required |
| `mutate` | `NamedLock::exclusive` | [308](../../src/folder_authority.rs#L308) | [store::platform::NamedLock::exclusive](../../../store/src/platform.rs#L103) |
| `mutate` | `self.directory().join` | [308](../../src/folder_authority.rs#L308) | receiver-type-required |
| `mutate` | `self.directory` | [308](../../src/folder_authority.rs#L308) | [profile::folder_authority::FolderAuthorityStore::directory](../../src/folder_authority.rs#L92) |
| `mutate` | `self.load` | [310](../../src/folder_authority.rs#L310) | [profile::folder_authority::FolderAuthorityStore::load](../../src/folder_authority.rs#L99) |
| `mutate` | `crate::digest` | [311](../../src/folder_authority.rs#L311) | [profile::digest](../../src/lib.rs#L130) |
| `mutate` | `serde_json_canonicalizer::to_vec(                 &serde_json::json!({"expected":expected,"request":request}),             )             .map_err` | [312](../../src/folder_authority.rs#L312) | receiver-type-required |
| `mutate` | `serde_json_canonicalizer::to_vec` | [312](../../src/folder_authority.rs#L312), [354](../../src/folder_authority.rs#L354) | external-constructor-callback-or-unresolved |
| `mutate` | `state.operations.get` | [317](../../src/folder_authority.rs#L317) | receiver-type-required |
| `mutate` | `Ok` | [319](../../src/folder_authority.rs#L319), [359](../../src/folder_authority.rs#L359) | external-constructor-callback-or-unresolved |
| `mutate` | `prior.receipt.clone` | [319](../../src/folder_authority.rs#L319) | receiver-type-required |
| `mutate` | `apply` | [327](../../src/folder_authority.rs#L327) | external-constructor-callback-or-unresolved |
| `mutate` | `state             .revision             .checked_add(1)             .ok_or_else` | [328](../../src/folder_authority.rs#L328) | receiver-type-required |
| `mutate` | `state             .revision             .checked_add` | [328](../../src/folder_authority.rs#L328) | receiver-type-required |
| `mutate` | `state.operations.insert` | [336](../../src/folder_authority.rs#L336) | receiver-type-required |
| `mutate` | `operation.to_owned` | [337](../../src/folder_authority.rs#L337) | receiver-type-required |
| `mutate` | `receipt.clone` | [340](../../src/folder_authority.rs#L340) | receiver-type-required |
| `mutate` | `state.operations.len` | [345](../../src/folder_authority.rs#L345) | receiver-type-required |
| `mutate` | `state                 .operations                 .iter()                 .min_by_key(&#124;(_, record)&#124; record.receipt.revision)                 .map(&#124;(id, _)&#124; id.clone())                 .ok_or_else` | [346](../../src/folder_authority.rs#L346) | receiver-type-required |
| `mutate` | `state                 .operations                 .iter()                 .min_by_key(&#124;(_, record)&#124; record.receipt.revision)                 .map` | [346](../../src/folder_authority.rs#L346) | receiver-type-required |
| `mutate` | `state                 .operations                 .iter()                 .min_by_key` | [346](../../src/folder_authority.rs#L346) | receiver-type-required |
| `mutate` | `state                 .operations                 .iter` | [346](../../src/folder_authority.rs#L346) | receiver-type-required |
| `mutate` | `id.clone` | [350](../../src/folder_authority.rs#L350) | receiver-type-required |
| `mutate` | `state.operations.remove` | [352](../../src/folder_authority.rs#L352) | receiver-type-required |
| `mutate` | `serde_json_canonicalizer::to_vec(&state).map_err` | [354](../../src/folder_authority.rs#L354) | receiver-type-required |
| `mutate` | `bytes.len` | [355](../../src/folder_authority.rs#L355) | receiver-type-required |
| `mutate` | `AtomicPublisher::replace(self.path(), &bytes).map_err` | [358](../../src/folder_authority.rs#L358) | receiver-type-required |
| `mutate` | `AtomicPublisher::replace` | [358](../../src/folder_authority.rs#L358) | [store::atomic::AtomicPublisher::replace](../../../store/src/atomic.rs#L16) |
| `mutate` | `self.path` | [358](../../src/folder_authority.rs#L358) | [profile::folder_authority::FolderAuthorityStore::path](../../src/folder_authority.rs#L95) |
| `admit` | `valid_id` | [363](../../src/folder_authority.rs#L363) | [profile::folder_authority::valid_id](../../src/folder_authority.rs#L391) |
| `admit` | `Err` | [364](../../src/folder_authority.rs#L364), [373](../../src/folder_authority.rs#L373) | external-constructor-callback-or-unresolved |
| `admit` | `invalid` | [364](../../src/folder_authority.rs#L364), [373](../../src/folder_authority.rs#L373) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `admit` | `NamedLock::shared(             self.directory()                 .join(format!(".folder-effect-{}.lock", captured.id)),         )         .map_err` | [366](../../src/folder_authority.rs#L366) | receiver-type-required |
| `admit` | `NamedLock::shared` | [366](../../src/folder_authority.rs#L366) | [store::platform::NamedLock::shared](../../../store/src/platform.rs#L99) |
| `admit` | `self.directory()                 .join` | [367](../../src/folder_authority.rs#L367) | receiver-type-required |
| `admit` | `self.directory` | [367](../../src/folder_authority.rs#L367) | [profile::folder_authority::FolderAuthorityStore::directory](../../src/folder_authority.rs#L92) |
| `admit` | `self.status` | [371](../../src/folder_authority.rs#L371) | [profile::folder_authority::FolderAuthorityStore::status](../../src/folder_authority.rs#L133) |
| `admit` | `Path::new` | [371](../../src/folder_authority.rs#L371) | external-constructor-callback-or-unresolved |
| `admit` | `Ok` | [375](../../src/folder_authority.rs#L375) | external-constructor-callback-or-unresolved |
| `effects_drained` | `valid_id` | [380](../../src/folder_authority.rs#L380) | [profile::folder_authority::valid_id](../../src/folder_authority.rs#L391) |
| `effects_drained` | `Err` | [381](../../src/folder_authority.rs#L381), [386](../../src/folder_authority.rs#L386) | external-constructor-callback-or-unresolved |
| `effects_drained` | `invalid` | [381](../../src/folder_authority.rs#L381) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `effects_drained` | `NamedLock::try_exclusive` | [383](../../src/folder_authority.rs#L383) | [store::platform::NamedLock::try_exclusive](../../../store/src/platform.rs#L107) |
| `effects_drained` | `self.directory().join` | [383](../../src/folder_authority.rs#L383) | receiver-type-required |
| `effects_drained` | `self.directory` | [383](../../src/folder_authority.rs#L383) | [profile::folder_authority::FolderAuthorityStore::directory](../../src/folder_authority.rs#L92) |
| `effects_drained` | `Ok` | [384](../../src/folder_authority.rs#L384), [385](../../src/folder_authority.rs#L385) | external-constructor-callback-or-unresolved |
| `effects_drained` | `io::Error::other` | [386](../../src/folder_authority.rs#L386) | external-constructor-callback-or-unresolved |
| `valid_id` | `id.len` | [392](../../src/folder_authority.rs#L392) | receiver-type-required |
| `valid_id` | `id.bytes().all` | [392](../../src/folder_authority.rs#L392) | receiver-type-required |
| `valid_id` | `id.bytes` | [392](../../src/folder_authority.rs#L392) | receiver-type-required |
| `valid_id` | `b.is_ascii_hexdigit` | [392](../../src/folder_authority.rs#L392) | receiver-type-required |
| `identify` | `path.canonicalize` | [396](../../src/folder_authority.rs#L396) | receiver-type-required |
| `identify` | `path.parent().is_none` | [397](../../src/folder_authority.rs#L397) | receiver-type-required |
| `identify` | `path.parent` | [397](../../src/folder_authority.rs#L397) | receiver-type-required |
| `identify` | `Err` | [398](../../src/folder_authority.rs#L398), [402](../../src/folder_authority.rs#L402) | external-constructor-callback-or-unresolved |
| `identify` | `invalid` | [398](../../src/folder_authority.rs#L398), [402](../../src/folder_authority.rs#L402), [407](../../src/folder_authority.rs#L407), [416](../../src/folder_authority.rs#L416) | [profile::folder_authority::invalid](../../src/folder_authority.rs#L80) |
| `identify` | `fs::metadata` | [400](../../src/folder_authority.rs#L400) | external-constructor-callback-or-unresolved |
| `identify` | `meta.is_dir` | [401](../../src/folder_authority.rs#L401) | receiver-type-required |
| `identify` | `meta         .created()?         .duration_since(UNIX_EPOCH)         .map_err(&#124;_&#124; invalid("unsupported-folder-identity"))?         .as_nanos` | [404](../../src/folder_authority.rs#L404) | receiver-type-required |
| `identify` | `meta         .created()?         .duration_since(UNIX_EPOCH)         .map_err` | [404](../../src/folder_authority.rs#L404) | receiver-type-required |
| `identify` | `meta         .created()?         .duration_since` | [404](../../src/folder_authority.rs#L404) | receiver-type-required |
| `identify` | `meta         .created` | [404](../../src/folder_authority.rs#L404) | receiver-type-required |
| `identify` | `meta.dev().to_string` | [410](../../src/folder_authority.rs#L410) | receiver-type-required |
| `identify` | `meta.dev` | [410](../../src/folder_authority.rs#L410) | receiver-type-required |
| `identify` | `meta.ino().to_string` | [411](../../src/folder_authority.rs#L411) | receiver-type-required |
| `identify` | `meta.ino` | [411](../../src/folder_authority.rs#L411) | receiver-type-required |
| `identify` | `created_nanos.to_string` | [412](../../src/folder_authority.rs#L412) | receiver-type-required |
| `identify` | `path         .to_str()         .ok_or_else(&#124;&#124; invalid("non-utf8-folder"))?         .to_owned` | [414](../../src/folder_authority.rs#L414) | receiver-type-required |
| `identify` | `path         .to_str()         .ok_or_else` | [414](../../src/folder_authority.rs#L414) | receiver-type-required |
| `identify` | `path         .to_str` | [414](../../src/folder_authority.rs#L414) | receiver-type-required |
| `identify` | `crate::digest` | [418](../../src/folder_authority.rs#L418) | [profile::digest](../../src/lib.rs#L130) |
| `identify` | `serde_json_canonicalizer::to_vec(&("folder-identity/v1", &identity))             .map_err` | [419](../../src/folder_authority.rs#L419) | receiver-type-required |
| `identify` | `serde_json_canonicalizer::to_vec` | [419](../../src/folder_authority.rs#L419) | external-constructor-callback-or-unresolved |
| `identify` | `Ok` | [422](../../src/folder_authority.rs#L422) | external-constructor-callback-or-unresolved |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `tempfile::tempdir().unwrap` | [436](../../src/folder_authority.rs#L436), [437](../../src/folder_authority.rs#L437) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `tempfile::tempdir` | [436](../../src/folder_authority.rs#L436), [437](../../src/folder_authority.rs#L437) | external-constructor-callback-or-unresolved |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `FolderAuthorityStore::new` | [438](../../src/folder_authority.rs#L438) | external-constructor-callback-or-unresolved |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `state.path` | [438](../../src/folder_authority.rs#L438) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.request("request", 0, project.path()).unwrap` | [440](../../src/folder_authority.rs#L440) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.request` | [440](../../src/folder_authority.rs#L440) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `project.path` | [440](../../src/folder_authority.rs#L440) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.grant_from_host("grant", 1, "request").unwrap` | [441](../../src/folder_authority.rs#L441) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.grant_from_host` | [441](../../src/folder_authority.rs#L441) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.admit(&grant.folder).unwrap` | [442](../../src/folder_authority.rs#L442) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.admit` | [442](../../src/folder_authority.rs#L442) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.revoke("revoke", 2, &requested.folder.id).unwrap` | [443](../../src/folder_authority.rs#L443) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `authority.revoke` | [443](../../src/folder_authority.rs#L443) | receiver-type-required |
| `grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission` | `drop` | [458](../../src/folder_authority.rs#L458) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `tempfile::tempdir().unwrap` | [464](../../src/folder_authority.rs#L464), [465](../../src/folder_authority.rs#L465) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `tempfile::tempdir` | [464](../../src/folder_authority.rs#L464), [465](../../src/folder_authority.rs#L465) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `base.path().join` | [466](../../src/folder_authority.rs#L466), [468](../../src/folder_authority.rs#L468), [474](../../src/folder_authority.rs#L474) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `base.path` | [466](../../src/folder_authority.rs#L466), [468](../../src/folder_authority.rs#L468), [474](../../src/folder_authority.rs#L474) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::create_dir(&path).unwrap` | [467](../../src/folder_authority.rs#L467), [475](../../src/folder_authority.rs#L475) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::create_dir` | [467](../../src/folder_authority.rs#L467), [475](../../src/folder_authority.rs#L475) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `std::os::unix::fs::symlink(&path, &alias).unwrap` | [469](../../src/folder_authority.rs#L469) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `std::os::unix::fs::symlink` | [469](../../src/folder_authority.rs#L469) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `FolderAuthorityStore::new` | [470](../../src/folder_authority.rs#L470) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `state.path` | [470](../../src/folder_authority.rs#L470) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `authority.request("r", 0, &path).unwrap` | [471](../../src/folder_authority.rs#L471) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `authority.request` | [471](../../src/folder_authority.rs#L471) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `authority.grant_from_host("g", 1, "r").unwrap` | [472](../../src/folder_authority.rs#L472) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `authority.grant_from_host` | [472](../../src/folder_authority.rs#L472) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::rename(&path, base.path().join("old")).unwrap` | [474](../../src/folder_authority.rs#L474) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::rename` | [474](../../src/folder_authority.rs#L474) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::write(authority.path(), b"broken").unwrap` | [477](../../src/folder_authority.rs#L477) | receiver-type-required |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `fs::write` | [477](../../src/folder_authority.rs#L477) | external-constructor-callback-or-unresolved |
| `aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants` | `authority.path` | [477](../../src/folder_authority.rs#L477) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `tempfile::tempdir().unwrap` | [482](../../src/folder_authority.rs#L482), [483](../../src/folder_authority.rs#L483) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `tempfile::tempdir` | [482](../../src/folder_authority.rs#L482), [483](../../src/folder_authority.rs#L483) | external-constructor-callback-or-unresolved |
| `retained_receipt_capacity_cannot_disable_revocation` | `FolderAuthorityStore::new` | [484](../../src/folder_authority.rs#L484) | external-constructor-callback-or-unresolved |
| `retained_receipt_capacity_cannot_disable_revocation` | `root.path` | [484](../../src/folder_authority.rs#L484) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.request("r", 0, project.path()).unwrap` | [485](../../src/folder_authority.rs#L485) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.request` | [485](../../src/folder_authority.rs#L485) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `project.path` | [485](../../src/folder_authority.rs#L485) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.grant_from_host("g", 1, "r").unwrap` | [486](../../src/folder_authority.rs#L486) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.grant_from_host` | [486](../../src/folder_authority.rs#L486) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.load().unwrap` | [487](../../src/folder_authority.rs#L487) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.load` | [487](../../src/folder_authority.rs#L487) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `state.operations.clear` | [489](../../src/folder_authority.rs#L489) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `state.operations.insert` | [491](../../src/folder_authority.rs#L491) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `"0".repeat` | [494](../../src/folder_authority.rs#L494) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `grant.folder.clone` | [497](../../src/folder_authority.rs#L497) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `AtomicPublisher::replace(             authority.path(),             &serde_json_canonicalizer::to_vec(&state).unwrap(),         )         .unwrap` | [502](../../src/folder_authority.rs#L502) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `AtomicPublisher::replace` | [502](../../src/folder_authority.rs#L502) | external-constructor-callback-or-unresolved |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.path` | [503](../../src/folder_authority.rs#L503) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `serde_json_canonicalizer::to_vec(&state).unwrap` | [504](../../src/folder_authority.rs#L504) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `serde_json_canonicalizer::to_vec` | [504](../../src/folder_authority.rs#L504) | external-constructor-callback-or-unresolved |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.revoke("revoke", 4096, &grant.folder.id).unwrap` | [507](../../src/folder_authority.rs#L507) | receiver-type-required |
| `retained_receipt_capacity_cannot_disable_revocation` | `authority.revoke` | [507](../../src/folder_authority.rs#L507) | receiver-type-required |
