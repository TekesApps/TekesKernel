# provider::secret_store

[Package atlas](index.md) · [Source](../../src/secret_store.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [provider::secret_store::MAX_SAFE_INTEGER](../../src/secret_store.rs#L11) | const_item | `private` |  |
| [provider::secret_store::RecordState](../../src/secret_store.rs#L15) | enum_item | `private` |  |
| [provider::secret_store::WireRecord](../../src/secret_store.rs#L22) | struct_item | `private` |  |
| [provider::secret_store::SecretRecord](../../src/secret_store.rs#L31) | enum_item | `pub` |  |
| [provider::secret_store::SecretRecord::clone](../../src/secret_store.rs#L37) | function_item | `private` |  |
| [provider::secret_store::SecretRecord::fmt](../../src/secret_store.rs#L54) | function_item | `private` |  |
| [provider::secret_store::SecretRecord::drop](../../src/secret_store.rs#L70) | function_item | `private` |  |
| [provider::secret_store::SecretRecord::decode](../../src/secret_store.rs#L78) | function_item | `pub` |  |
| [provider::secret_store::SecretRecord::encode](../../src/secret_store.rs#L104) | function_item | `pub` |  |
| [provider::secret_store::SecretRecord::generation](../../src/secret_store.rs#L131) | function_item | `pub` |  |
| [provider::secret_store::valid_material](../../src/secret_store.rs#L138) | function_item | `private` |  |
| [provider::secret_store::SecretResolution](../../src/secret_store.rs#L146) | enum_item | `pub` |  |
| [provider::secret_store::SecretStore](../../src/secret_store.rs#L152) | trait_item | `pub` |  |
| [provider::secret_store::SecretStore::resolve](../../src/secret_store.rs#L153) | function_signature_item | `private` |  |
| [provider::secret_store::SecretStoreError](../../src/secret_store.rs#L157) | enum_item | `pub` |  |
| [provider::secret_store::MemorySecretStore](../../src/secret_store.rs#L173) | struct_item | `pub` |  |
| [provider::secret_store::MemorySecretStore::new](../../src/secret_store.rs#L179) | function_item | `pub` |  |
| [provider::secret_store::MemorySecretStore::publish](../../src/secret_store.rs#L183) | function_item | `pub` |  |
| [provider::secret_store::MemorySecretStore::remove](../../src/secret_store.rs#L205) | function_item | `pub` |  |
| [provider::secret_store::MemorySecretStore::resolve](../../src/secret_store.rs#L214) | function_item | `private` |  |
| [provider::secret_store::SecretMutationAuthority](../../src/secret_store.rs#L239) | trait_item | `pub` |  |
| [provider::secret_store::SecretMutationAuthority::publish_record](../../src/secret_store.rs#L240) | function_signature_item | `private` |  |
| [provider::secret_store::MemorySecretStore::publish_record](../../src/secret_store.rs#L249) | function_item | `private` |  |
| [provider::secret_store::CredentialAvailability](../../src/secret_store.rs#L274) | enum_item | `pub` |  |
| [provider::secret_store::ResolvedCredentialBindings](../../src/secret_store.rs#L282) | struct_item | `pub` |  |
| [provider::secret_store::ResolveBindingsError](../../src/secret_store.rs#L289) | enum_item | `pub` |  |
| [provider::secret_store::resolve_config_credentials](../../src/secret_store.rs#L294) | function_item | `pub` |  |
| [provider::secret_store::push_resolved_scope](../../src/secret_store.rs#L366) | function_item | `private` |  |
| [provider::secret_store::scope_order](../../src/secret_store.rs#L400) | function_item | `private` |  |
| [provider::secret_store::revoked_scope_order](../../src/secret_store.rs#L415) | function_item | `private` |  |
| [provider::secret_store::validate_credential_id](../../src/secret_store.rs#L433) | function_item | `pub(crate)` |  |
| [provider::secret_store::tests::record_bytes_are_closed_canonical_and_redacted](../../src/secret_store.rs#L452) | function_item | `private` | test; #[cfg(test)] |
| [provider::secret_store::tests::memory_store_enforces_monotonic_rotation_and_revocation](../../src/secret_store.rs#L479) | function_item | `private` | test; #[cfg(test)] |
| [provider::secret_store::tests::config_with_shared_key](../../src/secret_store.rs#L507) | function_item | `private` | test; #[cfg(test)] |
| [provider::secret_store::tests::frozen_config_derives_model_and_independent_web_search_scopes](../../src/secret_store.rs#L534) | function_item | `private` | test; #[cfg(test)] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `ConfigSnapshot` | `profile::ConfigSnapshot` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `Zeroize` | `zeroize::Zeroize` | `private` |
| `CredentialScope` | `crate::CredentialScope` | `private` |
| `RevokedCredentialScope` | `crate::RevokedCredentialScope` | `private` |
| `endpoint_origin` | `crate::endpoint_origin` | `private` |
| `*` | `super::*` | `private` |
| `json` | `serde_json::json` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `provider::secret_store::tests` | `private` | #[cfg(test)] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–17: 7 direct edges</summary>

```mermaid
flowchart TD
  n0["provider::request::endpoint_origin"]
  n1["provider::secret_store::SecretRecord::encode"]
  n2["provider::secret_store::SecretRecord::generation"]
  n3["provider::secret_store::valid_material"]
  n4["provider::secret_store::MemorySecretStore::new"]
  n5["provider::secret_store::MemorySecretStore::publish"]
  n6["provider::secret_store::MemorySecretStore::remove"]
  n7["provider::secret_store::MemorySecretStore::resolve"]
  n8["provider::secret_store::MemorySecretStore::publish_record"]
  n9["provider::secret_store::resolve_config_credentials"]
  n10["provider::secret_store::push_resolved_scope"]
  n11["provider::secret_store::SecretRecord::clone"]
  n12["provider::secret_store::scope_order"]
  n13["provider::secret_store::revoked_scope_order"]
  n14["provider::secret_store::validate_credential_id"]
  n15["provider::secret_store::SecretRecord::fmt"]
  n16["provider::secret_store::SecretRecord::drop"]
  n17["provider::secret_store::SecretRecord::decode"]
  n1 --> n3
  n5 --> n14
  n7 --> n14
  n8 --> n14
  n9 --> n0
  n9 --> n10
  n17 --> n3
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `clone` | `material.clone` | [44](../../src/secret_store.rs#L44) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct("Active")                 .field("generation", generation)                 .field("material", &"<redacted>")                 .finish` | [56](../../src/secret_store.rs#L56) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct("Active")                 .field("generation", generation)                 .field` | [56](../../src/secret_store.rs#L56) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct("Active")                 .field` | [56](../../src/secret_store.rs#L56) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct` | [56](../../src/secret_store.rs#L56), [61](../../src/secret_store.rs#L61) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct("Revoked")                 .field("generation", generation)                 .finish` | [61](../../src/secret_store.rs#L61) | receiver-type-required |
| `fmt` | `formatter                 .debug_struct("Revoked")                 .field` | [61](../../src/secret_store.rs#L61) | receiver-type-required |
| `drop` | `material.zeroize` | [72](../../src/secret_store.rs#L72) | receiver-type-required |
| `decode` | `serde_json::from_slice(bytes).map_err` | [80](../../src/secret_store.rs#L80) | receiver-type-required |
| `decode` | `serde_json::from_slice` | [80](../../src/secret_store.rs#L80) | external-constructor-callback-or-unresolved |
| `decode` | `serde_json_canonicalizer::to_vec(&wire).map_err` | [82](../../src/secret_store.rs#L82) | receiver-type-required |
| `decode` | `serde_json_canonicalizer::to_vec` | [82](../../src/secret_store.rs#L82) | external-constructor-callback-or-unresolved |
| `decode` | `Err` | [88](../../src/secret_store.rs#L88), [100](../../src/secret_store.rs#L100) | external-constructor-callback-or-unresolved |
| `decode` | `valid_material` | [91](../../src/secret_store.rs#L91) | [provider::secret_store::valid_material](../../src/secret_store.rs#L138) |
| `decode` | `Ok` | [92](../../src/secret_store.rs#L92), [97](../../src/secret_store.rs#L97) | external-constructor-callback-or-unresolved |
| `encode` | `valid_material` | [109](../../src/secret_store.rs#L109) | [provider::secret_store::valid_material](../../src/secret_store.rs#L138) |
| `encode` | `Some` | [114](../../src/secret_store.rs#L114) | external-constructor-callback-or-unresolved |
| `encode` | `material.clone` | [114](../../src/secret_store.rs#L114) | receiver-type-required |
| `encode` | `Err` | [125](../../src/secret_store.rs#L125) | external-constructor-callback-or-unresolved |
| `encode` | `serde_json_canonicalizer::to_vec(&wire).map_err` | [127](../../src/secret_store.rs#L127) | receiver-type-required |
| `encode` | `serde_json_canonicalizer::to_vec` | [127](../../src/secret_store.rs#L127) | external-constructor-callback-or-unresolved |
| `valid_material` | `material.is_empty` | [139](../../src/secret_store.rs#L139) | receiver-type-required |
| `valid_material` | `material             .chars()             .any` | [140](../../src/secret_store.rs#L140) | receiver-type-required |
| `valid_material` | `material             .chars` | [140](../../src/secret_store.rs#L140) | receiver-type-required |
| `new` | `Self::default` | [180](../../src/secret_store.rs#L180) | external-constructor-callback-or-unresolved |
| `publish` | `validate_credential_id` | [188](../../src/secret_store.rs#L188) | [provider::secret_store::validate_credential_id](../../src/secret_store.rs#L433) |
| `publish` | `drop` | [190](../../src/secret_store.rs#L190) | external-constructor-callback-or-unresolved |
| `publish` | `record.encode` | [190](../../src/secret_store.rs#L190) | receiver-type-required |
| `publish` | `self             .records             .lock()             .unwrap_or_else` | [191](../../src/secret_store.rs#L191) | receiver-type-required |
| `publish` | `self             .records             .lock` | [191](../../src/secret_store.rs#L191) | receiver-type-required |
| `publish` | `records             .get(credential_id)             .is_some_and` | [195](../../src/secret_store.rs#L195) | receiver-type-required |
| `publish` | `records             .get` | [195](../../src/secret_store.rs#L195) | receiver-type-required |
| `publish` | `record.generation` | [197](../../src/secret_store.rs#L197) | receiver-type-required |
| `publish` | `prior.generation` | [197](../../src/secret_store.rs#L197) | receiver-type-required |
| `publish` | `Err` | [199](../../src/secret_store.rs#L199) | external-constructor-callback-or-unresolved |
| `publish` | `records.insert` | [201](../../src/secret_store.rs#L201) | receiver-type-required |
| `publish` | `credential_id.to_owned` | [201](../../src/secret_store.rs#L201) | receiver-type-required |
| `publish` | `Ok` | [202](../../src/secret_store.rs#L202) | external-constructor-callback-or-unresolved |
| `remove` | `self.records             .lock()             .unwrap_or_else(std::sync::PoisonError::into_inner)             .remove` | [206](../../src/secret_store.rs#L206) | receiver-type-required |
| `remove` | `self.records             .lock()             .unwrap_or_else` | [206](../../src/secret_store.rs#L206) | receiver-type-required |
| `remove` | `self.records             .lock` | [206](../../src/secret_store.rs#L206) | receiver-type-required |
| `resolve` | `validate_credential_id` | [215](../../src/secret_store.rs#L215) | [provider::secret_store::validate_credential_id](../../src/secret_store.rs#L433) |
| `resolve` | `Ok` | [216](../../src/secret_store.rs#L216) | external-constructor-callback-or-unresolved |
| `resolve` | `self                 .records                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .get(credential_id)                 .cloned` | [217](../../src/secret_store.rs#L217) | receiver-type-required |
| `resolve` | `self                 .records                 .lock()                 .unwrap_or_else(std::sync::PoisonError::into_inner)                 .get` | [217](../../src/secret_store.rs#L217) | receiver-type-required |
| `resolve` | `self                 .records                 .lock()                 .unwrap_or_else` | [217](../../src/secret_store.rs#L217) | receiver-type-required |
| `resolve` | `self                 .records                 .lock` | [217](../../src/secret_store.rs#L217) | receiver-type-required |
| `resolve` | `SecretResolution::Active` | [224](../../src/secret_store.rs#L224) | external-constructor-callback-or-unresolved |
| `publish_record` | `validate_credential_id` | [255](../../src/secret_store.rs#L255) | [provider::secret_store::validate_credential_id](../../src/secret_store.rs#L433) |
| `publish_record` | `drop` | [256](../../src/secret_store.rs#L256) | external-constructor-callback-or-unresolved |
| `publish_record` | `record.encode` | [256](../../src/secret_store.rs#L256) | receiver-type-required |
| `publish_record` | `self             .records             .lock()             .unwrap_or_else` | [257](../../src/secret_store.rs#L257) | receiver-type-required |
| `publish_record` | `self             .records             .lock` | [257](../../src/secret_store.rs#L257) | receiver-type-required |
| `publish_record` | `records.get(credential_id).map` | [261](../../src/secret_store.rs#L261) | receiver-type-required |
| `publish_record` | `records.get` | [261](../../src/secret_store.rs#L261) | receiver-type-required |
| `publish_record` | `Err` | [263](../../src/secret_store.rs#L263), [266](../../src/secret_store.rs#L266) | external-constructor-callback-or-unresolved |
| `publish_record` | `current.is_some_and` | [265](../../src/secret_store.rs#L265) | receiver-type-required |
| `publish_record` | `record.generation` | [265](../../src/secret_store.rs#L265) | receiver-type-required |
| `publish_record` | `records.insert` | [268](../../src/secret_store.rs#L268) | receiver-type-required |
| `publish_record` | `credential_id.to_owned` | [268](../../src/secret_store.rs#L268) | receiver-type-required |
| `publish_record` | `Ok` | [269](../../src/secret_store.rs#L269) | external-constructor-callback-or-unresolved |
| `resolve_config_credentials` | `BTreeMap::new` | [298](../../src/secret_store.rs#L298) | external-constructor-callback-or-unresolved |
| `resolve_config_credentials` | `resolutions                 .entry(credential_id.clone())                 .or_insert_with` | [301](../../src/secret_store.rs#L301) | receiver-type-required |
| `resolve_config_credentials` | `resolutions                 .entry` | [301](../../src/secret_store.rs#L301) | receiver-type-required |
| `resolve_config_credentials` | `credential_id.clone` | [302](../../src/secret_store.rs#L302), [329](../../src/secret_store.rs#L329) | receiver-type-required |
| `resolve_config_credentials` | `store.resolve` | [303](../../src/secret_store.rs#L303), [309](../../src/secret_store.rs#L309) | receiver-type-required |
| `resolve_config_credentials` | `resolutions             .entry(search.credential_key.clone())             .or_insert_with` | [307](../../src/secret_store.rs#L307) | receiver-type-required |
| `resolve_config_credentials` | `resolutions             .entry` | [307](../../src/secret_store.rs#L307) | receiver-type-required |
| `resolve_config_credentials` | `search.credential_key.clone` | [308](../../src/secret_store.rs#L308) | receiver-type-required |
| `resolve_config_credentials` | `ResolvedCredentialBindings::default` | [312](../../src/secret_store.rs#L312) | external-constructor-callback-or-unresolved |
| `resolve_config_credentials` | `result             .availability             .insert` | [327](../../src/secret_store.rs#L327) | receiver-type-required |
| `resolve_config_credentials` | `endpoint_origin(&configured.endpoint)             .map_err` | [336](../../src/secret_store.rs#L336) | receiver-type-required |
| `resolve_config_credentials` | `endpoint_origin` | [336](../../src/secret_store.rs#L336), [349](../../src/secret_store.rs#L349) | [provider::request::endpoint_origin](../../src/request.rs#L113) |
| `resolve_config_credentials` | `push_resolved_scope` | [338](../../src/secret_store.rs#L338), [350](../../src/secret_store.rs#L350) | [provider::secret_store::push_resolved_scope](../../src/secret_store.rs#L366) |
| `resolve_config_credentials` | `resolutions.get` | [340](../../src/secret_store.rs#L340), [352](../../src/secret_store.rs#L352) | receiver-type-required |
| `resolve_config_credentials` | `endpoint_origin(&search.endpoint).map_err` | [349](../../src/secret_store.rs#L349) | receiver-type-required |
| `resolve_config_credentials` | `result.active.sort_by` | [359](../../src/secret_store.rs#L359) | receiver-type-required |
| `resolve_config_credentials` | `result.active.dedup` | [360](../../src/secret_store.rs#L360) | receiver-type-required |
| `resolve_config_credentials` | `result.revoked.sort_by` | [361](../../src/secret_store.rs#L361) | receiver-type-required |
| `resolve_config_credentials` | `result.revoked.dedup` | [362](../../src/secret_store.rs#L362) | receiver-type-required |
| `resolve_config_credentials` | `Ok` | [363](../../src/secret_store.rs#L363) | external-constructor-callback-or-unresolved |
| `push_resolved_scope` | `result.active.push` | [378](../../src/secret_store.rs#L378) | receiver-type-required |
| `push_resolved_scope` | `credential_id.to_owned` | [379](../../src/secret_store.rs#L379), [390](../../src/secret_store.rs#L390) | receiver-type-required |
| `push_resolved_scope` | `adapter.to_owned` | [380](../../src/secret_store.rs#L380), [391](../../src/secret_store.rs#L391) | receiver-type-required |
| `push_resolved_scope` | `origin.to_owned` | [381](../../src/secret_store.rs#L381), [392](../../src/secret_store.rs#L392) | receiver-type-required |
| `push_resolved_scope` | `purpose.to_owned` | [382](../../src/secret_store.rs#L382), [393](../../src/secret_store.rs#L393) | receiver-type-required |
| `push_resolved_scope` | `generation.to_string` | [383](../../src/secret_store.rs#L383), [394](../../src/secret_store.rs#L394) | receiver-type-required |
| `push_resolved_scope` | `material.clone` | [384](../../src/secret_store.rs#L384) | receiver-type-required |
| `push_resolved_scope` | `result.revoked.push` | [389](../../src/secret_store.rs#L389) | receiver-type-required |
| `scope_order` | `(         &left.credential_id,         &left.adapter,         &left.endpoint_origin,         &left.purpose,     )         .cmp` | [401](../../src/secret_store.rs#L401) | receiver-type-required |
| `revoked_scope_order` | `(         &left.credential_id,         &left.adapter,         &left.endpoint_origin,         &left.purpose,     )         .cmp` | [419](../../src/secret_store.rs#L419) | receiver-type-required |
| `validate_credential_id` | `value.is_empty` | [434](../../src/secret_store.rs#L434) | receiver-type-required |
| `validate_credential_id` | `value.len` | [435](../../src/secret_store.rs#L435) | receiver-type-required |
| `validate_credential_id` | `value             .chars()             .any` | [436](../../src/secret_store.rs#L436) | receiver-type-required |
| `validate_credential_id` | `value             .chars` | [436](../../src/secret_store.rs#L436) | receiver-type-required |
| `validate_credential_id` | `character.is_control` | [438](../../src/secret_store.rs#L438) | receiver-type-required |
| `validate_credential_id` | `Err` | [440](../../src/secret_store.rs#L440) | external-constructor-callback-or-unresolved |
| `validate_credential_id` | `Ok` | [442](../../src/secret_store.rs#L442) | external-constructor-callback-or-unresolved |
| `record_bytes_are_closed_canonical_and_redacted` | `"fixture-secret-never-log".to_owned` | [455](../../src/secret_store.rs#L455) | receiver-type-required |
| `record_bytes_are_closed_canonical_and_redacted` | `active.encode().expect` | [457](../../src/secret_store.rs#L457) | receiver-type-required |
| `record_bytes_are_closed_canonical_and_redacted` | `active.encode` | [457](../../src/secret_store.rs#L457) | receiver-type-required |
| `memory_store_enforces_monotonic_rotation_and_revocation` | `MemorySecretStore::new` | [480](../../src/secret_store.rs#L480) | external-constructor-callback-or-unresolved |
| `memory_store_enforces_monotonic_rotation_and_revocation` | `store             .publish(                 "provider-main",                 SecretRecord::Active {                     generation: 1,                     material: "first".to_owned(),                 },             )             .expect` | [481](../../src/secret_store.rs#L481) | receiver-type-required |
| `memory_store_enforces_monotonic_rotation_and_revocation` | `store             .publish` | [481](../../src/secret_store.rs#L481), [498](../../src/secret_store.rs#L498) | receiver-type-required |
| `memory_store_enforces_monotonic_rotation_and_revocation` | `"first".to_owned` | [486](../../src/secret_store.rs#L486) | receiver-type-required |
| `memory_store_enforces_monotonic_rotation_and_revocation` | `store             .publish("provider-main", SecretRecord::Revoked { generation: 2 })             .expect` | [498](../../src/secret_store.rs#L498) | receiver-type-required |
| `config_with_shared_key` | `serde_json::from_value(json!({             "format": 1,             "workspace": {                 "format": 1,                 "revision": 1,                 "id": "workspace",                 "name": "Workspace",                 "cwd": ["/tmp"],                 "policy": {"network": true}             },             "providers": {                 "format": 1,                 "revision": 1,                 "providers": [                     {"id":"one","adapter":"responses","dialect":"openai_responses_v1","endpoint_owner":"openai","gateway_translation":"direct","evidence_revision":"openai-2026-08-01","endpoint":"https://one.example/v1","credential_key":"shared","models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,"context_window_tokens":100,"compact_trigger_tokens":50}]},                     {"id":"two","adapter":"anthropic_messages","dialect":"anthropic_messages_v1","endpoint_owner":"anthropic","gateway_translation":"direct","evidence_revision":"anthropic-2026-08-01","endpoint":"https://two.example/v1","credential_key":"shared","models":[{"id":"claude-sonnet-4-20250514","profile":"anthropic_messages_v1:claude-sonnet-4-20250514","enabled":true,"context_window_tokens":100,"compact_trigger_tokens":50}]}                 ],                 "web_search": {"adapter":"tavily_v1","endpoint":"https://search.example","credential_key":"shared"}             },                         "settings": {"format":1,"revision":0},             "revisions": {"workspace":1,"providers":1,"settings":0}         }))         .expect` | [508](../../src/secret_store.rs#L508) | receiver-type-required |
| `config_with_shared_key` | `serde_json::from_value` | [508](../../src/secret_store.rs#L508) | external-constructor-callback-or-unresolved |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `MemorySecretStore::new` | [535](../../src/secret_store.rs#L535) | external-constructor-callback-or-unresolved |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `store             .publish(                 "shared",                 SecretRecord::Active {                     generation: 9,                     material: "fixture-secret-never-log".to_owned(),                 },             )             .expect` | [536](../../src/secret_store.rs#L536) | receiver-type-required |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `store             .publish` | [536](../../src/secret_store.rs#L536) | receiver-type-required |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `"fixture-secret-never-log".to_owned` | [541](../../src/secret_store.rs#L541) | receiver-type-required |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `resolve_config_credentials(&config_with_shared_key(), &store).expect` | [546](../../src/secret_store.rs#L546) | receiver-type-required |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `resolve_config_credentials` | [546](../../src/secret_store.rs#L546) | external-constructor-callback-or-unresolved |
| `frozen_config_derives_model_and_independent_web_search_scopes` | `config_with_shared_key` | [546](../../src/secret_store.rs#L546) | [provider::secret_store::tests::config_with_shared_key](../../src/secret_store.rs#L507) |
