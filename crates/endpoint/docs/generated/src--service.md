# endpoint::service

[Package atlas](index.md) · [Source](../../src/service.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::service::AUTOMATIC_TITLE_SEED_OPERATION](../../src/service.rs#L13) | const_item | `pub` |  |
| [endpoint::service::AUTOMATIC_TITLE_REFINE_OPERATION](../../src/service.rs#L14) | const_item | `pub` |  |
| [endpoint::service::SESSION_NOTICE_OPERATION](../../src/service.rs#L15) | const_item | `pub` |  |
| [endpoint::service::SessionNotice](../../src/service.rs#L22) | struct_item | `pub` |  |
| [endpoint::service::SessionNoticeSeverity](../../src/service.rs#L30) | enum_item | `pub` |  |
| [endpoint::service::SessionNoticeSeverity::as_str](../../src/service.rs#L37) | function_item | `pub` |  |
| [endpoint::service::MutationReceipt](../../src/service.rs#L46) | struct_item | `pub` |  |
| [endpoint::service::MutationReceipt::from](../../src/service.rs#L52) | function_item | `private` |  |
| [endpoint::service::ArchiveResult](../../src/service.rs#L61) | struct_item | `pub` |  |
| [endpoint::service::UnarchiveResult](../../src/service.rs#L66) | struct_item | `pub` |  |
| [endpoint::service::SessionInventoryItem](../../src/service.rs#L71) | struct_item | `pub` |  |
| [endpoint::service::PERMISSION_MODE_FILE](../../src/service.rs#L96) | const_item | `private` |  |
| [endpoint::service::DEFAULT_PERMISSION_MODE](../../src/service.rs#L97) | const_item | `private` |  |
| [endpoint::service::genesis_cwd](../../src/service.rs#L101) | function_item | `pub` |  |
| [endpoint::service::genesis_preset_summary](../../src/service.rs#L110) | function_item | `pub` |  |
| [endpoint::service::permission_mode_of](../../src/service.rs#L116) | function_item | `pub` |  |
| [endpoint::service::card_item](../../src/service.rs#L133) | function_item | `private` |  |
| [endpoint::service::card_updated_ms](../../src/service.rs#L172) | function_item | `pub` |  |
| [endpoint::service::last_record_updated_ms](../../src/service.rs#L187) | function_item | `pub` |  |
| [endpoint::service::last_record_updated_ms::WINDOW](../../src/service.rs#L189) | const_item | `private` |  |
| [endpoint::service::card_summary](../../src/service.rs#L210) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint](../../src/service.rs#L258) | struct_item | `pub` |  |
| [endpoint::service::NativeEndpoint::open](../../src/service.rs#L263) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::capabilities](../../src/service.rs#L270) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::list_sessions](../../src/service.rs#L289) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::list_sessions_cached](../../src/service.rs#L299) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::models](../../src/service.rs#L370) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::session_config_snapshot](../../src/service.rs#L434) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::submission_snapshot](../../src/service.rs#L446) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::submission_metadata](../../src/service.rs#L453) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::hydrate_historical_submissions](../../src/service.rs#L476) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::create_session](../../src/service.rs#L584) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::prompt](../../src/service.rs#L609) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::prompt_for_endpoint](../../src/service.rs#L620) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::prompt_with_operation](../../src/service.rs#L638) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::update_queue](../../src/service.rs#L672) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::rename_session](../../src/service.rs#L687) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::rename_session_for_endpoint](../../src/service.rs#L697) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::rename_session_with_operation](../../src/service.rs#L707) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::seed_automatic_title_if_missing](../../src/service.rs#L726) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::record_session_notice](../../src/service.rs#L759) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::refine_automatic_title](../../src/service.rs#L813) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::author_keyed_with_ledger](../../src/service.rs#L854) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::cancel](../../src/service.rs#L877) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::cancel_for_endpoint](../../src/service.rs#L886) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::cancel_with_operation](../../src/service.rs#L895) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::fork_session](../../src/service.rs#L925) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::archive_session](../../src/service.rs#L941) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::unarchive_session](../../src/service.rs#L949) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::reconcile_projection](../../src/service.rs#L960) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::reconciled_journal](../../src/service.rs#L968) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::history](../../src/service.rs#L982) | function_item | `pub` |  |
| [endpoint::service::NativeEndpoint::keyed](../../src/service.rs#L992) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) | function_item | `private` |  |
| [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1023) | function_item | `private` |  |
| [endpoint::service::event](../../src/service.rs#L1036) | function_item | `private` |  |
| [endpoint::service::store_error](../../src/service.rs#L1041) | function_item | `private` |  |
| [endpoint::service::map_store_error](../../src/service.rs#L1045) | function_item | `private` |  |
| [endpoint::service::NativeEndpointError](../../src/service.rs#L1054) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `HashSet` | `std::collections::HashSet` | `private` |
| `fs` | `std::fs` | `private` |
| `Path` | `std::path::Path` | `private` |
| `Event` | `schema::Event` | `private` |
| `EventKind` | `schema::EventKind` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `ResumePolicy` | `schema::ResumePolicy` | `private` |
| `SeqRange` | `schema::SeqRange` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `json` | `serde_json::json` | `private` |
| `AppendOutcome` | `store::AppendOutcome` | `private` |
| `ConditionalAppendOutcome` | `store::ConditionalAppendOutcome` | `private` |
| `ThreadStore` | `store::ThreadStore` | `private` |
| `scan_valid_prefix` | `store::scan_valid_prefix` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `timestamp_millis` | `crate::projection::timestamp_millis` | `private` |
| `EndpointJournal` | `crate::EndpointJournal` | `private` |
| `HistoryPage` | `crate::HistoryPage` | `private` |
| `Projector` | `crate::Projector` | `private` |
| `history_page` | `crate::history_page` | `private` |
| `validate_session_id` | `crate::validate_session_id` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 39 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::projection::timestamp_millis"]
  n1["endpoint::service::genesis_cwd"]
  n2["endpoint::service::NativeEndpoint::active_folder"]
  n3["endpoint::service::event"]
  n4["endpoint::service::genesis_preset_summary"]
  n5["endpoint::service::permission_mode_of"]
  n6["endpoint::service::card_item"]
  n7["endpoint::service::card_updated_ms"]
  n8["endpoint::service::last_record_updated_ms"]
  n9["endpoint::service::card_summary"]
  n10["endpoint::service::NativeEndpoint::open"]
  n11["endpoint::service::NativeEndpoint::capabilities"]
  n12["endpoint::service::NativeEndpoint::list_sessions"]
  n13["endpoint::service::NativeEndpoint::list_sessions_cached"]
  n14["endpoint::service::NativeEndpoint::list_sessions_with"]
  n15["endpoint::service::NativeEndpoint::models"]
  n16["endpoint::service::SessionNoticeSeverity::as_str"]
  n17["endpoint::service::NativeEndpoint::session_config_snapshot"]
  n18["endpoint::service::NativeEndpoint::submission_snapshot"]
  n19["endpoint::service::NativeEndpoint::submission_metadata"]
  n20["endpoint::service::NativeEndpoint::hydrate_historical_submissions"]
  n21["endpoint::service::MutationReceipt::from"]
  n22["endpoint::service::NativeEndpoint::create_session"]
  n23["endpoint::types::validate_session_id"]
  n24["profile::config::ConfigSnapshot::decode"]
  n25["profile::config::ConfigRepository::open"]
  n26["profile::scope::SessionScope::for_session_folder"]
  n27["schema::ijson::IJsonValue::parse"]
  n28["schema::effective_session_genesis"]
  n29["store::asset::AssetStore::new"]
  n30["store::card::read_card"]
  n31["store::card::classify"]
  n32["store::folder::ThreadStore::open"]
  n33["store::tail::scan_valid_prefix"]
  n34["store::tail::read_genesis"]
  n35["tools::preset::ResolvedPreset::from_genesis"]
  n4 --> n35
  n6 --> n1
  n6 --> n5
  n6 --> n30
  n6 --> n31
  n6 --> n34
  n7 --> n0
  n8 --> n0
  n9 --> n4
  n9 --> n7
  n9 --> n28
  n10 --> n32
  n12 --> n14
  n13 --> n14
  n14 --> n1
  n14 --> n5
  n14 --> n6
  n14 --> n9
  n14 --> n23
  n14 --> n33
  n15 --> n27
  n17 --> n2
  n17 --> n23
  n17 --> n25
  n17 --> n26
  n18 --> n2
  n18 --> n17
  n18 --> n19
  n18 --> n29
  n19 --> n27
  n20 --> n2
  n20 --> n19
  n20 --> n24
  n20 --> n27
  n20 --> n29
  n20 --> n33
  n22 --> n3
  n22 --> n23
  n22 --> n25
```

</details>

<details><summary>Functions 21–40: 32 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::history::history_page"]
  n1["endpoint::journal::EndpointJournal::open"]
  n2["endpoint::service::NativeEndpoint::validate_origin"]
  n3["endpoint::service::NativeEndpoint::active_folder"]
  n4["endpoint::service::event"]
  n5["endpoint::service::NativeEndpoint::submission_snapshot"]
  n6["endpoint::service::NativeEndpoint::prompt"]
  n7["endpoint::service::NativeEndpoint::prompt_for_endpoint"]
  n8["endpoint::service::NativeEndpoint::prompt_with_operation"]
  n9["endpoint::service::NativeEndpoint::update_queue"]
  n10["endpoint::service::NativeEndpoint::rename_session"]
  n11["endpoint::service::NativeEndpoint::rename_session_for_endpoint"]
  n12["endpoint::service::NativeEndpoint::rename_session_with_operation"]
  n13["endpoint::service::NativeEndpoint::seed_automatic_title_if_missing"]
  n14["endpoint::service::NativeEndpoint::record_session_notice"]
  n15["endpoint::service::NativeEndpoint::refine_automatic_title"]
  n16["endpoint::service::NativeEndpoint::author_keyed_with_ledger"]
  n17["endpoint::service::NativeEndpoint::cancel"]
  n18["endpoint::service::NativeEndpoint::cancel_for_endpoint"]
  n19["endpoint::service::NativeEndpoint::cancel_with_operation"]
  n20["endpoint::service::NativeEndpoint::fork_session"]
  n21["endpoint::service::NativeEndpoint::archive_session"]
  n22["endpoint::service::NativeEndpoint::unarchive_session"]
  n23["endpoint::service::NativeEndpoint::reconcile_projection"]
  n24["endpoint::service::NativeEndpoint::reconciled_journal"]
  n25["endpoint::service::NativeEndpoint::history"]
  n26["endpoint::service::NativeEndpoint::keyed"]
  n27["endpoint::types::validate_session_id"]
  n28["store::tail::scan_valid_prefix"]
  n6 --> n8
  n7 --> n8
  n8 --> n2
  n8 --> n4
  n8 --> n5
  n9 --> n4
  n9 --> n26
  n10 --> n12
  n11 --> n12
  n12 --> n4
  n12 --> n26
  n13 --> n2
  n13 --> n4
  n14 --> n2
  n14 --> n4
  n15 --> n2
  n15 --> n4
  n16 --> n27
  n17 --> n19
  n18 --> n19
  n19 --> n2
  n19 --> n4
  n19 --> n27
  n20 --> n27
  n21 --> n27
  n22 --> n27
  n23 --> n24
  n24 --> n1
  n24 --> n3
  n24 --> n28
  n25 --> n0
  n25 --> n24
```

</details>

<details><summary>Functions 41–46: 5 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::service::NativeEndpoint::validate_origin"]
  n1["endpoint::service::NativeEndpoint::active_folder"]
  n2["endpoint::service::event"]
  n3["endpoint::service::store_error"]
  n4["endpoint::service::map_store_error"]
  n5["endpoint::service::NativeEndpoint::keyed"]
  n6["endpoint::types::validate_session_id"]
  n7["schema::event::Event::from_value"]
  n8["schema::ijson::IJsonValue::parse"]
  n0 --> n6
  n1 --> n6
  n2 --> n7
  n2 --> n8
  n5 --> n0
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `genesis_cwd` | `genesis.string_field("cwd").unwrap_or_default().to_owned` | [102](../../src/service.rs#L102) | receiver-type-required |
| `genesis_cwd` | `genesis.string_field("cwd").unwrap_or_default` | [102](../../src/service.rs#L102) | receiver-type-required |
| `genesis_cwd` | `genesis.string_field` | [102](../../src/service.rs#L102) | receiver-type-required |
| `genesis_preset_summary` | `serde_json::to_value(genesis.raw()).ok` | [111](../../src/service.rs#L111) | receiver-type-required |
| `genesis_preset_summary` | `serde_json::to_value` | [111](../../src/service.rs#L111) | external-constructor-callback-or-unresolved |
| `genesis_preset_summary` | `genesis.raw` | [111](../../src/service.rs#L111) | receiver-type-required |
| `genesis_preset_summary` | `tools::ResolvedPreset::from_genesis(raw.get("preset")?).ok` | [112](../../src/service.rs#L112) | receiver-type-required |
| `genesis_preset_summary` | `tools::ResolvedPreset::from_genesis` | [112](../../src/service.rs#L112) | [tools::preset::ResolvedPreset::from_genesis](../../../tools/src/preset.rs#L855) |
| `genesis_preset_summary` | `raw.get` | [112](../../src/service.rs#L112) | receiver-type-required |
| `genesis_preset_summary` | `Some` | [113](../../src/service.rs#L113) | external-constructor-callback-or-unresolved |
| `genesis_preset_summary` | `preset.summary` | [113](../../src/service.rs#L113) | receiver-type-required |
| `permission_mode_of` | `fs::read` | [117](../../src/service.rs#L117) | external-constructor-callback-or-unresolved |
| `permission_mode_of` | `folder.join` | [117](../../src/service.rs#L117) | receiver-type-required |
| `permission_mode_of` | `DEFAULT_PERMISSION_MODE.to_owned` | [118](../../src/service.rs#L118), [129](../../src/service.rs#L129) | receiver-type-required |
| `permission_mode_of` | `serde_json::from_slice::<serde_json::Value>(&bytes)         .ok()         .and_then(&#124;value&#124; value.get("mode")?.as_str().map(str::to_owned))         .filter(&#124;mode&#124; {             matches!(                 mode.as_str(),                 "read-only" &#124; "workspace-write" &#124; "danger-full-access"             )         })         .unwrap_or_else` | [120](../../src/service.rs#L120) | receiver-type-required |
| `permission_mode_of` | `serde_json::from_slice::<serde_json::Value>(&bytes)         .ok()         .and_then(&#124;value&#124; value.get("mode")?.as_str().map(str::to_owned))         .filter` | [120](../../src/service.rs#L120) | receiver-type-required |
| `permission_mode_of` | `serde_json::from_slice::<serde_json::Value>(&bytes)         .ok()         .and_then` | [120](../../src/service.rs#L120) | receiver-type-required |
| `permission_mode_of` | `serde_json::from_slice::<serde_json::Value>(&bytes)         .ok` | [120](../../src/service.rs#L120) | receiver-type-required |
| `permission_mode_of` | `serde_json::from_slice::<serde_json::Value>` | [120](../../src/service.rs#L120) | external-constructor-callback-or-unresolved |
| `permission_mode_of` | `value.get("mode")?.as_str().map` | [122](../../src/service.rs#L122) | receiver-type-required |
| `permission_mode_of` | `value.get("mode")?.as_str` | [122](../../src/service.rs#L122) | receiver-type-required |
| `permission_mode_of` | `value.get` | [122](../../src/service.rs#L122) | receiver-type-required |
| `card_item` | `store::card::read_card` | [139](../../src/service.rs#L139) | [store::card::read_card](../../../store/src/card.rs#L176) |
| `card_item` | `store::card::classify(folder, &card).ok` | [140](../../src/service.rs#L140) | receiver-type-required |
| `card_item` | `store::card::classify` | [140](../../src/service.rs#L140) | [store::card::classify](../../../store/src/card.rs#L395) |
| `card_item` | `classified.freshness` | [141](../../src/service.rs#L141) | receiver-type-required |
| `card_item` | `card.lines.first` | [144](../../src/service.rs#L144) | receiver-type-required |
| `card_item` | `Some` | [148](../../src/service.rs#L148) | external-constructor-callback-or-unresolved |
| `card_item` | `session_id.to_owned` | [149](../../src/service.rs#L149) | receiver-type-required |
| `card_item` | `card.summary.cwd.clone().unwrap_or_else` | [150](../../src/service.rs#L150) | receiver-type-required |
| `card_item` | `card.summary.cwd.clone` | [150](../../src/service.rs#L150) | receiver-type-required |
| `card_item` | `store::read_genesis(folder.join("main.jsonl"))                 .ok()                 .flatten()                 .map(&#124;g&#124; genesis_cwd(folder, &g))                 .unwrap_or_default` | [151](../../src/service.rs#L151) | receiver-type-required |
| `card_item` | `store::read_genesis(folder.join("main.jsonl"))                 .ok()                 .flatten()                 .map` | [151](../../src/service.rs#L151) | receiver-type-required |
| `card_item` | `store::read_genesis(folder.join("main.jsonl"))                 .ok()                 .flatten` | [151](../../src/service.rs#L151) | receiver-type-required |
| `card_item` | `store::read_genesis(folder.join("main.jsonl"))                 .ok` | [151](../../src/service.rs#L151) | receiver-type-required |
| `card_item` | `store::read_genesis` | [151](../../src/service.rs#L151) | [store::tail::read_genesis](../../../store/src/tail.rs#L86) |
| `card_item` | `folder.join` | [151](../../src/service.rs#L151) | receiver-type-required |
| `card_item` | `genesis_cwd` | [154](../../src/service.rs#L154) | [endpoint::service::genesis_cwd](../../src/service.rs#L101) |
| `card_item` | `card.summary.title.clone` | [157](../../src/service.rs#L157) | receiver-type-required |
| `card_item` | `main.facts.clone` | [163](../../src/service.rs#L163) | receiver-type-required |
| `card_item` | `permission_mode_of` | [164](../../src/service.rs#L164) | [endpoint::service::permission_mode_of](../../src/service.rs#L116) |
| `card_item` | `card.summary.identity_profile.clone` | [165](../../src/service.rs#L165) | receiver-type-required |
| `card_item` | `card.summary.preset.clone` | [166](../../src/service.rs#L166) | receiver-type-required |
| `card_updated_ms` | `Ok` | [173](../../src/service.rs#L173) | external-constructor-callback-or-unresolved |
| `card_updated_ms` | `projection         .events         .last()         .and_then(&#124;event&#124; event.string_field("ts"))         .ok_or(NativeEndpointError::InventoryTimestamp)         .and_then` | [173](../../src/service.rs#L173) | receiver-type-required |
| `card_updated_ms` | `projection         .events         .last()         .and_then(&#124;event&#124; event.string_field("ts"))         .ok_or` | [173](../../src/service.rs#L173) | receiver-type-required |
| `card_updated_ms` | `projection         .events         .last()         .and_then` | [173](../../src/service.rs#L173) | receiver-type-required |
| `card_updated_ms` | `projection         .events         .last` | [173](../../src/service.rs#L173) | receiver-type-required |
| `card_updated_ms` | `event.string_field` | [176](../../src/service.rs#L176) | receiver-type-required |
| `card_updated_ms` | `timestamp_millis(value).map_err` | [178](../../src/service.rs#L178) | receiver-type-required |
| `card_updated_ms` | `timestamp_millis` | [178](../../src/service.rs#L178) | [endpoint::projection::timestamp_millis](../../src/projection.rs#L1106) |
| `last_record_updated_ms` | `fs::File::open(path).ok` | [190](../../src/service.rs#L190) | receiver-type-required |
| `last_record_updated_ms` | `fs::File::open` | [190](../../src/service.rs#L190) | external-constructor-callback-or-unresolved |
| `last_record_updated_ms` | `file.metadata().ok()?.len` | [191](../../src/service.rs#L191) | receiver-type-required |
| `last_record_updated_ms` | `file.metadata().ok` | [191](../../src/service.rs#L191) | receiver-type-required |
| `last_record_updated_ms` | `file.metadata` | [191](../../src/service.rs#L191) | receiver-type-required |
| `last_record_updated_ms` | `length.saturating_sub` | [192](../../src/service.rs#L192) | receiver-type-required |
| `last_record_updated_ms` | `file.seek(SeekFrom::Start(start)).ok` | [193](../../src/service.rs#L193) | receiver-type-required |
| `last_record_updated_ms` | `file.seek` | [193](../../src/service.rs#L193) | receiver-type-required |
| `last_record_updated_ms` | `SeekFrom::Start` | [193](../../src/service.rs#L193) | external-constructor-callback-or-unresolved |
| `last_record_updated_ms` | `Vec::new` | [194](../../src/service.rs#L194) | external-constructor-callback-or-unresolved |
| `last_record_updated_ms` | `file.take(WINDOW).read_to_end(&mut bytes).ok` | [195](../../src/service.rs#L195) | receiver-type-required |
| `last_record_updated_ms` | `file.take(WINDOW).read_to_end` | [195](../../src/service.rs#L195) | receiver-type-required |
| `last_record_updated_ms` | `file.take` | [195](../../src/service.rs#L195) | receiver-type-required |
| `last_record_updated_ms` | `bytes.iter().rposition` | [197](../../src/service.rs#L197) | receiver-type-required |
| `last_record_updated_ms` | `bytes.iter` | [197](../../src/service.rs#L197) | receiver-type-required |
| `last_record_updated_ms` | `bytes[..end]         .iter()         .rposition(&#124;byte&#124; *byte == b'\n')         .map(&#124;index&#124; index + 1)         .or` | [198](../../src/service.rs#L198) | receiver-type-required |
| `last_record_updated_ms` | `bytes[..end]         .iter()         .rposition(&#124;byte&#124; *byte == b'\n')         .map` | [198](../../src/service.rs#L198) | receiver-type-required |
| `last_record_updated_ms` | `bytes[..end]         .iter()         .rposition` | [198](../../src/service.rs#L198) | receiver-type-required |
| `last_record_updated_ms` | `bytes[..end]         .iter` | [198](../../src/service.rs#L198) | receiver-type-required |
| `last_record_updated_ms` | `(start == 0).then_some` | [202](../../src/service.rs#L202) | receiver-type-required |
| `last_record_updated_ms` | `serde_json::from_slice(&bytes[line_start..end]).ok` | [203](../../src/service.rs#L203) | receiver-type-required |
| `last_record_updated_ms` | `serde_json::from_slice` | [203](../../src/service.rs#L203) | external-constructor-callback-or-unresolved |
| `last_record_updated_ms` | `timestamp_millis(record.get("ts")?.as_str()?).ok` | [204](../../src/service.rs#L204) | receiver-type-required |
| `last_record_updated_ms` | `timestamp_millis` | [204](../../src/service.rs#L204) | [endpoint::projection::timestamp_millis](../../src/projection.rs#L1106) |
| `last_record_updated_ms` | `record.get("ts")?.as_str` | [204](../../src/service.rs#L204) | receiver-type-required |
| `last_record_updated_ms` | `record.get` | [204](../../src/service.rs#L204) | receiver-type-required |
| `last_record_updated_ms` | `Some` | [205](../../src/service.rs#L205) | external-constructor-callback-or-unresolved |
| `card_summary` | `projection.events.first().ok_or_else` | [213](../../src/service.rs#L213) | receiver-type-required |
| `card_summary` | `projection.events.first` | [213](../../src/service.rs#L213) | receiver-type-required |
| `card_summary` | `NativeEndpointError::Store` | [214](../../src/service.rs#L214) | external-constructor-callback-or-unresolved |
| `card_summary` | `store::StoreError::Corruption` | [214](../../src/service.rs#L214) | external-constructor-callback-or-unresolved |
| `card_summary` | `"ledger has no genesis".to_owned` | [215](../../src/service.rs#L215) | receiver-type-required |
| `card_summary` | `projection         .events         .iter()         .rev()         .find_map(&#124;event&#124; event.string_field("title"))         .map` | [218](../../src/service.rs#L218) | receiver-type-required |
| `card_summary` | `projection         .events         .iter()         .rev()         .find_map` | [218](../../src/service.rs#L218) | receiver-type-required |
| `card_summary` | `projection         .events         .iter()         .rev` | [218](../../src/service.rs#L218) | receiver-type-required |
| `card_summary` | `projection         .events         .iter` | [218](../../src/service.rs#L218) | receiver-type-required |
| `card_summary` | `event.string_field` | [222](../../src/service.rs#L222), [232](../../src/service.rs#L232) | receiver-type-required |
| `card_summary` | `card_updated_ms` | [224](../../src/service.rs#L224) | [endpoint::service::card_updated_ms](../../src/service.rs#L172) |
| `card_summary` | `genesis_preset_summary` | [225](../../src/service.rs#L225) | [endpoint::service::genesis_preset_summary](../../src/service.rs#L110) |
| `card_summary` | `genesis.string_field` | [226](../../src/service.rs#L226), [229](../../src/service.rs#L229), [248](../../src/service.rs#L248) | receiver-type-required |
| `card_summary` | `preset.is_some` | [227](../../src/service.rs#L227) | receiver-type-required |
| `card_summary` | `Some` | [228](../../src/service.rs#L228), [232](../../src/service.rs#L232) | external-constructor-callback-or-unresolved |
| `card_summary` | `"coding".to_owned` | [228](../../src/service.rs#L228) | receiver-type-required |
| `card_summary` | `genesis.string_field("identity_profile").map` | [229](../../src/service.rs#L229) | receiver-type-required |
| `card_summary` | `projection.events.iter().find_map` | [230](../../src/service.rs#L230) | receiver-type-required |
| `card_summary` | `projection.events.iter` | [230](../../src/service.rs#L230) | receiver-type-required |
| `card_summary` | `event.kind` | [231](../../src/service.rs#L231) | receiver-type-required |
| `card_summary` | `serde_json::to_value(event.raw())                 .ok()?                 .get("payload")?                 .get("profile")?                 .as_str()                 .map` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `serde_json::to_value(event.raw())                 .ok()?                 .get("payload")?                 .get("profile")?                 .as_str` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `serde_json::to_value(event.raw())                 .ok()?                 .get("payload")?                 .get` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `serde_json::to_value(event.raw())                 .ok()?                 .get` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `serde_json::to_value(event.raw())                 .ok` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `serde_json::to_value` | [236](../../src/service.rs#L236) | external-constructor-callback-or-unresolved |
| `card_summary` | `event.raw` | [236](../../src/service.rs#L236) | receiver-type-required |
| `card_summary` | `schema::effective_session_genesis(&projection.events)?.expect` | [245](../../src/service.rs#L245) | receiver-type-required |
| `card_summary` | `schema::effective_session_genesis` | [245](../../src/service.rs#L245) | [schema::effective_session_genesis](../../../schema/src/lib.rs#L57) |
| `card_summary` | `Ok` | [246](../../src/service.rs#L246) | external-constructor-callback-or-unresolved |
| `card_summary` | `scope.get("cwd").and_then(Value::as_str).map` | [247](../../src/service.rs#L247) | receiver-type-required |
| `card_summary` | `scope.get("cwd").and_then` | [247](../../src/service.rs#L247) | receiver-type-required |
| `card_summary` | `scope.get` | [247](../../src/service.rs#L247) | receiver-type-required |
| `card_summary` | `genesis.string_field("ts").unwrap_or_default().to_owned` | [248](../../src/service.rs#L248) | receiver-type-required |
| `card_summary` | `genesis.string_field("ts").unwrap_or_default` | [248](../../src/service.rs#L248) | receiver-type-required |
| `card_summary` | `genesis.is_ephemeral_genesis` | [253](../../src/service.rs#L253) | receiver-type-required |
| `card_summary` | `projection.lifecycle.latest_turn.is_none` | [254](../../src/service.rs#L254) | receiver-type-required |
| `open` | `Ok` | [264](../../src/service.rs#L264) | external-constructor-callback-or-unresolved |
| `open` | `ThreadStore::open` | [265](../../src/service.rs#L265) | [store::folder::ThreadStore::open](../../../store/src/folder.rs#L36) |
| `capabilities` | `[             "session.models",             "session.create",             "session.prompt",             "session.updateQueue",             "session.cancel",             "session.rename",             "session.fork",             "session.discard",             "workspace.archiveSession",             "workspace.unarchiveSession",         ]         .into_iter()         .collect` | [271](../../src/service.rs#L271) | receiver-type-required |
| `capabilities` | `[             "session.models",             "session.create",             "session.prompt",             "session.updateQueue",             "session.cancel",             "session.rename",             "session.fork",             "session.discard",             "workspace.archiveSession",             "workspace.unarchiveSession",         ]         .into_iter` | [271](../../src/service.rs#L271) | receiver-type-required |
| `list_sessions` | `self.list_sessions_with` | [293](../../src/service.rs#L293) | [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306) |
| `list_sessions_cached` | `self.list_sessions_with` | [303](../../src/service.rs#L303) | [endpoint::service::NativeEndpoint::list_sessions_with](../../src/service.rs#L306) |
| `list_sessions_with` | `Vec::new` | [311](../../src/service.rs#L311) | external-constructor-callback-or-unresolved |
| `list_sessions_with` | `fs::read_dir` | [313](../../src/service.rs#L313) | external-constructor-callback-or-unresolved |
| `list_sessions_with` | `self.store.root().join` | [313](../../src/service.rs#L313) | receiver-type-required |
| `list_sessions_with` | `self.store.root` | [313](../../src/service.rs#L313) | receiver-type-required |
| `list_sessions_with` | `entry.file_type()?.is_dir` | [315](../../src/service.rs#L315) | receiver-type-required |
| `list_sessions_with` | `entry.file_type` | [315](../../src/service.rs#L315) | receiver-type-required |
| `list_sessions_with` | `entry.file_name().to_string_lossy().into_owned` | [318](../../src/service.rs#L318) | receiver-type-required |
| `list_sessions_with` | `entry.file_name().to_string_lossy` | [318](../../src/service.rs#L318) | receiver-type-required |
| `list_sessions_with` | `entry.file_name` | [318](../../src/service.rs#L318) | receiver-type-required |
| `list_sessions_with` | `validate_session_id` | [319](../../src/service.rs#L319) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `list_sessions_with` | `running.contains` | [321](../../src/service.rs#L321) | receiver-type-required |
| `list_sessions_with` | `self.store.session_has_live_line_holder` | [322](../../src/service.rs#L322) | receiver-type-required |
| `list_sessions_with` | `card_item` | [324](../../src/service.rs#L324) | [endpoint::service::card_item](../../src/service.rs#L133) |
| `list_sessions_with` | `entry.path` | [324](../../src/service.rs#L324), [329](../../src/service.rs#L329), [345](../../src/service.rs#L345), [351](../../src/service.rs#L351) | receiver-type-required |
| `list_sessions_with` | `items.push` | [326](../../src/service.rs#L326), [344](../../src/service.rs#L344) | receiver-type-required |
| `list_sessions_with` | `fs::read` | [329](../../src/service.rs#L329) | external-constructor-callback-or-unresolved |
| `list_sessions_with` | `entry.path().join` | [329](../../src/service.rs#L329) | receiver-type-required |
| `list_sessions_with` | `scan_valid_prefix` | [330](../../src/service.rs#L330) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `list_sessions_with` | `scan.projection.ok_or_else` | [331](../../src/service.rs#L331) | receiver-type-required |
| `list_sessions_with` | `NativeEndpointError::Store` | [332](../../src/service.rs#L332), [338](../../src/service.rs#L338) | external-constructor-callback-or-unresolved |
| `list_sessions_with` | `store::StoreError::Corruption` | [332](../../src/service.rs#L332), [338](../../src/service.rs#L338) | external-constructor-callback-or-unresolved |
| `list_sessions_with` | `card_summary(&projection).map_err` | [336](../../src/service.rs#L336) | receiver-type-required |
| `list_sessions_with` | `card_summary` | [336](../../src/service.rs#L336) | [endpoint::service::card_summary](../../src/service.rs#L210) |
| `list_sessions_with` | `permission_mode_of` | [345](../../src/service.rs#L345) | [endpoint::service::permission_mode_of](../../src/service.rs#L116) |
| `list_sessions_with` | `summary.cwd.unwrap_or_else` | [349](../../src/service.rs#L349) | receiver-type-required |
| `list_sessions_with` | `genesis_cwd` | [350](../../src/service.rs#L350) | [endpoint::service::genesis_cwd](../../src/service.rs#L101) |
| `list_sessions_with` | `projection.events.first().expect` | [352](../../src/service.rs#L352) | receiver-type-required |
| `list_sessions_with` | `projection.events.first` | [352](../../src/service.rs#L352) | receiver-type-required |
| `list_sessions_with` | `items.sort_by` | [366](../../src/service.rs#L366) | receiver-type-required |
| `list_sessions_with` | `left.session_id.as_bytes().cmp` | [366](../../src/service.rs#L366) | receiver-type-required |
| `list_sessions_with` | `left.session_id.as_bytes` | [366](../../src/service.rs#L366) | receiver-type-required |
| `list_sessions_with` | `right.session_id.as_bytes` | [366](../../src/service.rs#L366) | receiver-type-required |
| `list_sessions_with` | `Ok` | [367](../../src/service.rs#L367) | external-constructor-callback-or-unresolved |
| `models` | `snapshot             .session_settings             .as_ref()             .map(&#124;settings&#124; &settings.provider)             .or` | [374](../../src/service.rs#L374) | receiver-type-required |
| `models` | `snapshot             .session_settings             .as_ref()             .map` | [374](../../src/service.rs#L374), [379](../../src/service.rs#L379) | receiver-type-required |
| `models` | `snapshot             .session_settings             .as_ref` | [374](../../src/service.rs#L374), [379](../../src/service.rs#L379), [414](../../src/service.rs#L414) | receiver-type-required |
| `models` | `snapshot.settings.default_provider.as_ref` | [378](../../src/service.rs#L378) | receiver-type-required |
| `models` | `snapshot             .session_settings             .as_ref()             .map(&#124;settings&#124; &settings.model)             .or` | [379](../../src/service.rs#L379) | receiver-type-required |
| `models` | `snapshot.settings.default_model.as_ref` | [383](../../src/service.rs#L383) | receiver-type-required |
| `models` | `snapshot             .providers             .providers             .iter()             .map(&#124;provider&#124; {                 json!({                     "id": provider.id,                     "name": provider.id,                     "models": provider.models.iter().filter(&#124;model&#124; model.enabled).map(&#124;model&#124; {                         json!({"id": model.id, "name": model.id,                             "contextWindow": model.context_window_tokens})                     }).collect::<Vec<_>>()                 })             })             .collect::<Vec<_>>` | [384](../../src/service.rs#L384) | receiver-type-required |
| `models` | `snapshot             .providers             .providers             .iter()             .map` | [384](../../src/service.rs#L384) | receiver-type-required |
| `models` | `snapshot             .providers             .providers             .iter` | [384](../../src/service.rs#L384) | receiver-type-required |
| `models` | `selected_provider             .zip(selected_model)             .is_some_and` | [399](../../src/service.rs#L399) | receiver-type-required |
| `models` | `selected_provider             .zip` | [399](../../src/service.rs#L399) | receiver-type-required |
| `models` | `snapshot.providers.providers.iter().any` | [402](../../src/service.rs#L402) | receiver-type-required |
| `models` | `snapshot.providers.providers.iter` | [402](../../src/service.rs#L402) | receiver-type-required |
| `models` | `candidate                             .models                             .iter()                             .any` | [404](../../src/service.rs#L404) | receiver-type-required |
| `models` | `candidate                             .models                             .iter` | [404](../../src/service.rs#L404) | receiver-type-required |
| `models` | `snapshot             .session_settings             .as_ref()             .and_then` | [414](../../src/service.rs#L414) | receiver-type-required |
| `models` | `settings.reasoning_effort.as_ref` | [417](../../src/service.rs#L417) | receiver-type-required |
| `models` | `Value::String` | [419](../../src/service.rs#L419) | external-constructor-callback-or-unresolved |
| `models` | `effort.clone` | [419](../../src/service.rs#L419) | receiver-type-required |
| `models` | `Ok` | [427](../../src/service.rs#L427) | external-constructor-callback-or-unresolved |
| `models` | `IJsonValue::parse` | [427](../../src/service.rs#L427) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `models` | `serde_json::to_vec` | [427](../../src/service.rs#L427) | external-constructor-callback-or-unresolved |
| `session_config_snapshot` | `validate_session_id` | [438](../../src/service.rs#L438) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `session_config_snapshot` | `profile::SessionScope::for_session_folder` | [439](../../src/service.rs#L439) | [profile::scope::SessionScope::for_session_folder](../../../profile/src/scope.rs#L22) |
| `session_config_snapshot` | `self.active_folder` | [439](../../src/service.rs#L439) | [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1023) |
| `session_config_snapshot` | `profile::ConfigRepository::open` | [440](../../src/service.rs#L440) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L568) |
| `session_config_snapshot` | `self.store.root` | [440](../../src/service.rs#L440) | receiver-type-required |
| `session_config_snapshot` | `Ok` | [441](../../src/service.rs#L441) | external-constructor-callback-or-unresolved |
| `session_config_snapshot` | `scope.config` | [441](../../src/service.rs#L441) | receiver-type-required |
| `submission_snapshot` | `self.session_config_snapshot` | [447](../../src/service.rs#L447) | [endpoint::service::NativeEndpoint::session_config_snapshot](../../src/service.rs#L434) |
| `submission_snapshot` | `store::AssetStore::new` | [448](../../src/service.rs#L448) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `submission_snapshot` | `self.active_folder(session_id)?.join` | [448](../../src/service.rs#L448) | receiver-type-required |
| `submission_snapshot` | `self.active_folder` | [448](../../src/service.rs#L448) | [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1023) |
| `submission_snapshot` | `config.publish` | [449](../../src/service.rs#L449) | receiver-type-required |
| `submission_snapshot` | `Self::submission_metadata` | [450](../../src/service.rs#L450) | [endpoint::service::NativeEndpoint::submission_metadata](../../src/service.rs#L453) |
| `submission_metadata` | `config             .session_settings             .as_ref()             .map(&#124;s&#124; s.provider.as_str())             .or` | [457](../../src/service.rs#L457) | receiver-type-required |
| `submission_metadata` | `config             .session_settings             .as_ref()             .map` | [457](../../src/service.rs#L457), [462](../../src/service.rs#L462) | receiver-type-required |
| `submission_metadata` | `config             .session_settings             .as_ref` | [457](../../src/service.rs#L457), [462](../../src/service.rs#L462) | receiver-type-required |
| `submission_metadata` | `s.provider.as_str` | [460](../../src/service.rs#L460) | receiver-type-required |
| `submission_metadata` | `config.settings.default_provider.as_deref` | [461](../../src/service.rs#L461) | receiver-type-required |
| `submission_metadata` | `config             .session_settings             .as_ref()             .map(&#124;s&#124; s.model.as_str())             .or` | [462](../../src/service.rs#L462) | receiver-type-required |
| `submission_metadata` | `s.model.as_str` | [465](../../src/service.rs#L465) | receiver-type-required |
| `submission_metadata` | `config.settings.default_model.as_deref` | [466](../../src/service.rs#L466) | receiver-type-required |
| `submission_metadata` | `Ok` | [467](../../src/service.rs#L467) | external-constructor-callback-or-unresolved |
| `submission_metadata` | `IJsonValue::parse` | [467](../../src/service.rs#L467) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `submission_metadata` | `serde_json::to_vec` | [467](../../src/service.rs#L467) | external-constructor-callback-or-unresolved |
| `hydrate_historical_submissions` | `entries.iter().any` | [484](../../src/service.rs#L484) | receiver-type-required |
| `hydrate_historical_submissions` | `entries.iter` | [484](../../src/service.rs#L484) | receiver-type-required |
| `hydrate_historical_submissions` | `serde_json::to_value(&entry.event.data)                     .is_ok_and` | [486](../../src/service.rs#L486) | receiver-type-required |
| `hydrate_historical_submissions` | `serde_json::to_value` | [486](../../src/service.rs#L486), [508](../../src/service.rs#L508), [559](../../src/service.rs#L559), [572](../../src/service.rs#L572) | external-constructor-callback-or-unresolved |
| `hydrate_historical_submissions` | `data.get("submission").is_none` | [487](../../src/service.rs#L487) | receiver-type-required |
| `hydrate_historical_submissions` | `data.get` | [487](../../src/service.rs#L487), [562](../../src/service.rs#L562) | receiver-type-required |
| `hydrate_historical_submissions` | `self.active_folder` | [492](../../src/service.rs#L492) | [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1023) |
| `hydrate_historical_submissions` | `fs::read` | [495](../../src/service.rs#L495) | external-constructor-callback-or-unresolved |
| `hydrate_historical_submissions` | `folder.join` | [495](../../src/service.rs#L495), [501](../../src/service.rs#L501) | receiver-type-required |
| `hydrate_historical_submissions` | `scan_valid_prefix` | [498](../../src/service.rs#L498) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `hydrate_historical_submissions` | `store::AssetStore::new` | [501](../../src/service.rs#L501) | [store::asset::AssetStore::new](../../../store/src/asset.rs#L27) |
| `hydrate_historical_submissions` | `std::collections::BTreeMap::new` | [505](../../src/service.rs#L505), [506](../../src/service.rs#L506) | external-constructor-callback-or-unresolved |
| `hydrate_historical_submissions` | `event.raw` | [508](../../src/service.rs#L508) | receiver-type-required |
| `hydrate_historical_submissions` | `event.kind` | [511](../../src/service.rs#L511) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("config")                         .and_then(&#124;c&#124; c.get("digest"))                         .and_then(Value::as_str)                         .map` | [513](../../src/service.rs#L513) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("config")                         .and_then(&#124;c&#124; c.get("digest"))                         .and_then` | [513](../../src/service.rs#L513) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("config")                         .and_then` | [513](../../src/service.rs#L513) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get` | [513](../../src/service.rs#L513), [520](../../src/service.rs#L520), [529](../../src/service.rs#L529) | receiver-type-required |
| `hydrate_historical_submissions` | `c.get` | [515](../../src/service.rs#L515) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("config_digest")                         .and_then(Value::as_str)                         .map` | [520](../../src/service.rs#L520) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("config_digest")                         .and_then` | [520](../../src/service.rs#L520) | receiver-type-required |
| `hydrate_historical_submissions` | `inputs.insert` | [526](../../src/service.rs#L526) | receiver-type-required |
| `hydrate_historical_submissions` | `event.seq` | [526](../../src/service.rs#L526) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("trigger")                         .and_then(&#124;t&#124; t.get("inputs"))                         .and_then` | [529](../../src/service.rs#L529) | receiver-type-required |
| `hydrate_historical_submissions` | `value                         .get("trigger")                         .and_then` | [529](../../src/service.rs#L529) | receiver-type-required |
| `hydrate_historical_submissions` | `t.get` | [531](../../src/service.rs#L531) | receiver-type-required |
| `hydrate_historical_submissions` | `digest.as_deref().and_then` | [536](../../src/service.rs#L536) | receiver-type-required |
| `hydrate_historical_submissions` | `digest.as_deref` | [536](../../src/service.rs#L536) | receiver-type-required |
| `hydrate_historical_submissions` | `assets.read_verified(&format!("sha256-{digest}")).ok` | [537](../../src/service.rs#L537) | receiver-type-required |
| `hydrate_historical_submissions` | `assets.read_verified` | [537](../../src/service.rs#L537) | receiver-type-required |
| `hydrate_historical_submissions` | `profile::ConfigSnapshot::decode(&bytes).ok` | [538](../../src/service.rs#L538) | receiver-type-required |
| `hydrate_historical_submissions` | `profile::ConfigSnapshot::decode` | [538](../../src/service.rs#L538) | [profile::config::ConfigSnapshot::decode](../../../profile/src/config.rs#L219) |
| `hydrate_historical_submissions` | `Self::submission_metadata(&config, digest).ok` | [539](../../src/service.rs#L539) | receiver-type-required |
| `hydrate_historical_submissions` | `Self::submission_metadata` | [539](../../src/service.rs#L539) | [endpoint::service::NativeEndpoint::submission_metadata](../../src/service.rs#L453) |
| `hydrate_historical_submissions` | `seqs.iter().filter_map` | [541](../../src/service.rs#L541) | receiver-type-required |
| `hydrate_historical_submissions` | `seqs.iter` | [541](../../src/service.rs#L541) | receiver-type-required |
| `hydrate_historical_submissions` | `inputs                             .get(&seq)                             .and_then(&#124;input&#124; input.get("submission"))                             .and_then(&#124;s&#124; IJsonValue::parse(&serde_json::to_vec(s).ok()?).ok())                             .or_else` | [542](../../src/service.rs#L542) | receiver-type-required |
| `hydrate_historical_submissions` | `inputs                             .get(&seq)                             .and_then(&#124;input&#124; input.get("submission"))                             .and_then` | [542](../../src/service.rs#L542) | receiver-type-required |
| `hydrate_historical_submissions` | `inputs                             .get(&seq)                             .and_then` | [542](../../src/service.rs#L542) | receiver-type-required |
| `hydrate_historical_submissions` | `inputs                             .get` | [542](../../src/service.rs#L542) | receiver-type-required |
| `hydrate_historical_submissions` | `input.get` | [544](../../src/service.rs#L544) | receiver-type-required |
| `hydrate_historical_submissions` | `IJsonValue::parse(&serde_json::to_vec(s).ok()?).ok` | [545](../../src/service.rs#L545) | receiver-type-required |
| `hydrate_historical_submissions` | `IJsonValue::parse` | [545](../../src/service.rs#L545), [577](../../src/service.rs#L577) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `hydrate_historical_submissions` | `serde_json::to_vec(s).ok` | [545](../../src/service.rs#L545) | receiver-type-required |
| `hydrate_historical_submissions` | `serde_json::to_vec` | [545](../../src/service.rs#L545), [576](../../src/service.rs#L576) | external-constructor-callback-or-unresolved |
| `hydrate_historical_submissions` | `historical.clone` | [546](../../src/service.rs#L546) | receiver-type-required |
| `hydrate_historical_submissions` | `submissions.insert` | [548](../../src/service.rs#L548) | receiver-type-required |
| `hydrate_historical_submissions` | `data.get("submission").is_some` | [562](../../src/service.rs#L562) | receiver-type-required |
| `hydrate_historical_submissions` | `data                 .get("id")                 .and_then(Value::as_str)                 .and_then` | [565](../../src/service.rs#L565) | receiver-type-required |
| `hydrate_historical_submissions` | `data                 .get("id")                 .and_then` | [565](../../src/service.rs#L565) | receiver-type-required |
| `hydrate_historical_submissions` | `data                 .get` | [565](../../src/service.rs#L565) | receiver-type-required |
| `hydrate_historical_submissions` | `submissions.get` | [568](../../src/service.rs#L568) | receiver-type-required |
| `create_session` | `validate_session_id` | [593](../../src/service.rs#L593) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `create_session` | `Err` | [595](../../src/service.rs#L595) | external-constructor-callback-or-unresolved |
| `create_session` | `profile::ConfigRepository::open(self.store.root())?.normalize_execution` | [598](../../src/service.rs#L598) | receiver-type-required |
| `create_session` | `profile::ConfigRepository::open` | [598](../../src/service.rs#L598) | [profile::config::ConfigRepository::open](../../../profile/src/config.rs#L568) |
| `create_session` | `self.store.root` | [598](../../src/service.rs#L598) | receiver-type-required |
| `create_session` | `event` | [599](../../src/service.rs#L599) | [endpoint::service::event](../../src/service.rs#L1036) |
| `create_session` | `Ok` | [602](../../src/service.rs#L602) | external-constructor-callback-or-unresolved |
| `create_session` | `self             .store             .create_thread(session_id, genesis)             .map_err` | [602](../../src/service.rs#L602) | receiver-type-required |
| `create_session` | `self             .store             .create_thread` | [602](../../src/service.rs#L602) | receiver-type-required |
| `prompt` | `self.prompt_with_operation` | [617](../../src/service.rs#L617) | [endpoint::service::NativeEndpoint::prompt_with_operation](../../src/service.rs#L638) |
| `prompt_for_endpoint` | `self.prompt_with_operation` | [628](../../src/service.rs#L628) | [endpoint::service::NativeEndpoint::prompt_with_operation](../../src/service.rs#L638) |
| `prompt_with_operation` | `self.validate_origin` | [647](../../src/service.rs#L647) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `prompt_with_operation` | `self             .store             .append_keyed_with_projection_if(session_id, origin, &#124;seq, projection&#124; {                 if steer && !projection.lifecycle.steer_window_open() {                     return Ok(None);                 }                 let submission = self.submission_snapshot(session_id).map_err(store_error)?;                 let mut value = json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "input",                     "content": content, "origin_key": origin.key, "origin_tuple": origin,                     "submission": submission,                 });                 if steer {                     value["steer"] = Value::Bool(true);                 }                 event(value).map(Some).map_err(store_error)             })             .map_err` | [648](../../src/service.rs#L648) | receiver-type-required |
| `prompt_with_operation` | `self             .store             .append_keyed_with_projection_if` | [648](../../src/service.rs#L648) | receiver-type-required |
| `prompt_with_operation` | `projection.lifecycle.steer_window_open` | [651](../../src/service.rs#L651) | receiver-type-required |
| `prompt_with_operation` | `Ok` | [652](../../src/service.rs#L652), [667](../../src/service.rs#L667) | external-constructor-callback-or-unresolved |
| `prompt_with_operation` | `self.submission_snapshot(session_id).map_err` | [654](../../src/service.rs#L654) | receiver-type-required |
| `prompt_with_operation` | `self.submission_snapshot` | [654](../../src/service.rs#L654) | [endpoint::service::NativeEndpoint::submission_snapshot](../../src/service.rs#L446) |
| `prompt_with_operation` | `Value::Bool` | [661](../../src/service.rs#L661) | external-constructor-callback-or-unresolved |
| `prompt_with_operation` | `event(value).map(Some).map_err` | [663](../../src/service.rs#L663) | receiver-type-required |
| `prompt_with_operation` | `event(value).map` | [663](../../src/service.rs#L663) | receiver-type-required |
| `prompt_with_operation` | `event` | [663](../../src/service.rs#L663) | [endpoint::service::event](../../src/service.rs#L1036) |
| `prompt_with_operation` | `appended.into` | [667](../../src/service.rs#L667) | receiver-type-required |
| `prompt_with_operation` | `Err` | [668](../../src/service.rs#L668) | external-constructor-callback-or-unresolved |
| `update_queue` | `self.keyed` | [679](../../src/service.rs#L679) | [endpoint::service::NativeEndpoint::keyed](../../src/service.rs#L992) |
| `update_queue` | `event` | [680](../../src/service.rs#L680) | [endpoint::service::event](../../src/service.rs#L1036) |
| `rename_session` | `self.rename_session_with_operation` | [694](../../src/service.rs#L694) | [endpoint::service::NativeEndpoint::rename_session_with_operation](../../src/service.rs#L707) |
| `rename_session_for_endpoint` | `self.rename_session_with_operation` | [704](../../src/service.rs#L704) | [endpoint::service::NativeEndpoint::rename_session_with_operation](../../src/service.rs#L707) |
| `rename_session_with_operation` | `self.keyed` | [715](../../src/service.rs#L715) | [endpoint::service::NativeEndpoint::keyed](../../src/service.rs#L992) |
| `rename_session_with_operation` | `event` | [716](../../src/service.rs#L716) | [endpoint::service::event](../../src/service.rs#L1036) |
| `seed_automatic_title_if_missing` | `self.validate_origin` | [733](../../src/service.rs#L733) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `seed_automatic_title_if_missing` | `self             .store             .append_keyed_with_projection_if(session_id, origin, &#124;seq, projection&#124; {                 if projection.events.iter().any(&#124;item&#124; {                     *item.kind() == EventKind::Meta && item.string_field("title").is_some()                 }) {                     return Ok(None);                 }                 event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map(Some)                 .map_err(store_error)             })             .map_err` | [734](../../src/service.rs#L734) | receiver-type-required |
| `seed_automatic_title_if_missing` | `self             .store             .append_keyed_with_projection_if` | [734](../../src/service.rs#L734) | receiver-type-required |
| `seed_automatic_title_if_missing` | `projection.events.iter().any` | [737](../../src/service.rs#L737) | receiver-type-required |
| `seed_automatic_title_if_missing` | `projection.events.iter` | [737](../../src/service.rs#L737) | receiver-type-required |
| `seed_automatic_title_if_missing` | `item.kind` | [738](../../src/service.rs#L738) | receiver-type-required |
| `seed_automatic_title_if_missing` | `item.string_field("title").is_some` | [738](../../src/service.rs#L738) | receiver-type-required |
| `seed_automatic_title_if_missing` | `item.string_field` | [738](../../src/service.rs#L738) | receiver-type-required |
| `seed_automatic_title_if_missing` | `Ok` | [740](../../src/service.rs#L740), [750](../../src/service.rs#L750) | external-constructor-callback-or-unresolved |
| `seed_automatic_title_if_missing` | `event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map(Some)                 .map_err` | [742](../../src/service.rs#L742) | receiver-type-required |
| `seed_automatic_title_if_missing` | `event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map` | [742](../../src/service.rs#L742) | receiver-type-required |
| `seed_automatic_title_if_missing` | `event` | [742](../../src/service.rs#L742) | [endpoint::service::event](../../src/service.rs#L1036) |
| `seed_automatic_title_if_missing` | `Some` | [751](../../src/service.rs#L751) | external-constructor-callback-or-unresolved |
| `seed_automatic_title_if_missing` | `outcome.into` | [751](../../src/service.rs#L751) | receiver-type-required |
| `record_session_notice` | `self.validate_origin` | [766](../../src/service.rs#L766) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `record_session_notice` | `self.store                 .append_keyed_with_projection_if(session_id, origin, &#124;seq, projection&#124; {                     // A notice that repeats the newest one, with no input in between,                     // says nothing new (the same degraded launch on every worker                     // restart); fold it. An input after it is a fresh attempt the                     // user made, and its outcome must be shown again.                     let latest_notice = projection.events.iter().rposition(&#124;item&#124; {                         *item.kind() == EventKind::Meta && item.has_field("notice")                     });                     if let Some(index) = latest_notice {                         let repeats = serde_json::to_value(projection.events[index].raw())                             .ok()                             .and_then(&#124;value&#124; value.get("notice").cloned())                             .is_some_and(&#124;previous&#124; previous == payload);                         let input_since = projection.events[index + 1..]                             .iter()                             .any(&#124;item&#124; *item.kind() == EventKind::Input);                         if repeats && !input_since {                             return Ok(None);                         }                     }                     event(json!({                         "v": 1, "seq": seq, "ts": timestamp, "kind": "meta",                         "notice": payload,                         "origin_key": origin.key, "origin_tuple": origin,                     }))                     .map(Some)                     .map_err(store_error)                 })                 .map_err` | [774](../../src/service.rs#L774) | receiver-type-required |
| `record_session_notice` | `self.store                 .append_keyed_with_projection_if` | [774](../../src/service.rs#L774) | receiver-type-required |
| `record_session_notice` | `projection.events.iter().rposition` | [780](../../src/service.rs#L780) | receiver-type-required |
| `record_session_notice` | `projection.events.iter` | [780](../../src/service.rs#L780) | receiver-type-required |
| `record_session_notice` | `item.kind` | [781](../../src/service.rs#L781), [790](../../src/service.rs#L790) | receiver-type-required |
| `record_session_notice` | `item.has_field` | [781](../../src/service.rs#L781) | receiver-type-required |
| `record_session_notice` | `serde_json::to_value(projection.events[index].raw())                             .ok()                             .and_then(&#124;value&#124; value.get("notice").cloned())                             .is_some_and` | [784](../../src/service.rs#L784) | receiver-type-required |
| `record_session_notice` | `serde_json::to_value(projection.events[index].raw())                             .ok()                             .and_then` | [784](../../src/service.rs#L784) | receiver-type-required |
| `record_session_notice` | `serde_json::to_value(projection.events[index].raw())                             .ok` | [784](../../src/service.rs#L784) | receiver-type-required |
| `record_session_notice` | `serde_json::to_value` | [784](../../src/service.rs#L784) | external-constructor-callback-or-unresolved |
| `record_session_notice` | `projection.events[index].raw` | [784](../../src/service.rs#L784) | receiver-type-required |
| `record_session_notice` | `value.get("notice").cloned` | [786](../../src/service.rs#L786) | receiver-type-required |
| `record_session_notice` | `value.get` | [786](../../src/service.rs#L786) | receiver-type-required |
| `record_session_notice` | `projection.events[index + 1..]                             .iter()                             .any` | [788](../../src/service.rs#L788) | receiver-type-required |
| `record_session_notice` | `projection.events[index + 1..]                             .iter` | [788](../../src/service.rs#L788) | receiver-type-required |
| `record_session_notice` | `Ok` | [792](../../src/service.rs#L792), [804](../../src/service.rs#L804) | external-constructor-callback-or-unresolved |
| `record_session_notice` | `event(json!({                         "v": 1, "seq": seq, "ts": timestamp, "kind": "meta",                         "notice": payload,                         "origin_key": origin.key, "origin_tuple": origin,                     }))                     .map(Some)                     .map_err` | [795](../../src/service.rs#L795) | receiver-type-required |
| `record_session_notice` | `event(json!({                         "v": 1, "seq": seq, "ts": timestamp, "kind": "meta",                         "notice": payload,                         "origin_key": origin.key, "origin_tuple": origin,                     }))                     .map` | [795](../../src/service.rs#L795) | receiver-type-required |
| `record_session_notice` | `event` | [795](../../src/service.rs#L795) | [endpoint::service::event](../../src/service.rs#L1036) |
| `record_session_notice` | `Some` | [805](../../src/service.rs#L805) | external-constructor-callback-or-unresolved |
| `record_session_notice` | `outcome.into` | [805](../../src/service.rs#L805) | receiver-type-required |
| `refine_automatic_title` | `self.validate_origin` | [820](../../src/service.rs#L820) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `refine_automatic_title` | `self             .store             .append_keyed_with_projection_if(session_id, origin, &#124;seq, projection&#124; {                 let latest_title = projection.events.iter().rev().find(&#124;item&#124; {                     *item.kind() == EventKind::Meta && item.string_field("title").is_some()                 });                 let remains_automatic_seed = latest_title                     .map(Event::origin_tuple)                     .transpose()                     .map_err(store::StoreError::from)?                     .flatten()                     .is_some_and(&#124;tuple&#124; tuple.op == AUTOMATIC_TITLE_SEED_OPERATION);                 if !remains_automatic_seed {                     return Ok(None);                 }                 event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map(Some)                 .map_err(store_error)             })             .map_err` | [821](../../src/service.rs#L821) | receiver-type-required |
| `refine_automatic_title` | `self             .store             .append_keyed_with_projection_if` | [821](../../src/service.rs#L821) | receiver-type-required |
| `refine_automatic_title` | `projection.events.iter().rev().find` | [824](../../src/service.rs#L824) | receiver-type-required |
| `refine_automatic_title` | `projection.events.iter().rev` | [824](../../src/service.rs#L824) | receiver-type-required |
| `refine_automatic_title` | `projection.events.iter` | [824](../../src/service.rs#L824) | receiver-type-required |
| `refine_automatic_title` | `item.kind` | [825](../../src/service.rs#L825) | receiver-type-required |
| `refine_automatic_title` | `item.string_field("title").is_some` | [825](../../src/service.rs#L825) | receiver-type-required |
| `refine_automatic_title` | `item.string_field` | [825](../../src/service.rs#L825) | receiver-type-required |
| `refine_automatic_title` | `latest_title                     .map(Event::origin_tuple)                     .transpose()                     .map_err(store::StoreError::from)?                     .flatten()                     .is_some_and` | [827](../../src/service.rs#L827) | receiver-type-required |
| `refine_automatic_title` | `latest_title                     .map(Event::origin_tuple)                     .transpose()                     .map_err(store::StoreError::from)?                     .flatten` | [827](../../src/service.rs#L827) | receiver-type-required |
| `refine_automatic_title` | `latest_title                     .map(Event::origin_tuple)                     .transpose()                     .map_err` | [827](../../src/service.rs#L827) | receiver-type-required |
| `refine_automatic_title` | `latest_title                     .map(Event::origin_tuple)                     .transpose` | [827](../../src/service.rs#L827) | receiver-type-required |
| `refine_automatic_title` | `latest_title                     .map` | [827](../../src/service.rs#L827) | receiver-type-required |
| `refine_automatic_title` | `Ok` | [834](../../src/service.rs#L834), [844](../../src/service.rs#L844) | external-constructor-callback-or-unresolved |
| `refine_automatic_title` | `event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map(Some)                 .map_err` | [836](../../src/service.rs#L836) | receiver-type-required |
| `refine_automatic_title` | `event(json!({                     "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,                     "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map` | [836](../../src/service.rs#L836) | receiver-type-required |
| `refine_automatic_title` | `event` | [836](../../src/service.rs#L836) | [endpoint::service::event](../../src/service.rs#L1036) |
| `refine_automatic_title` | `Some` | [845](../../src/service.rs#L845) | external-constructor-callback-or-unresolved |
| `refine_automatic_title` | `outcome.into` | [845](../../src/service.rs#L845) | receiver-type-required |
| `author_keyed_with_ledger` | `validate_session_id` | [863](../../src/service.rs#L863) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `author_keyed_with_ledger` | `Err` | [865](../../src/service.rs#L865) | external-constructor-callback-or-unresolved |
| `author_keyed_with_ledger` | `self             .store             .append_keyed_with_ledger_if(session_id, origin, build)             .map_err` | [867](../../src/service.rs#L867) | receiver-type-required |
| `author_keyed_with_ledger` | `self             .store             .append_keyed_with_ledger_if` | [867](../../src/service.rs#L867) | receiver-type-required |
| `author_keyed_with_ledger` | `Ok` | [871](../../src/service.rs#L871) | external-constructor-callback-or-unresolved |
| `author_keyed_with_ledger` | `Some` | [872](../../src/service.rs#L872) | external-constructor-callback-or-unresolved |
| `author_keyed_with_ledger` | `outcome.into` | [872](../../src/service.rs#L872) | receiver-type-required |
| `cancel` | `self.cancel_with_operation` | [883](../../src/service.rs#L883) | [endpoint::service::NativeEndpoint::cancel_with_operation](../../src/service.rs#L895) |
| `cancel_for_endpoint` | `self.cancel_with_operation` | [892](../../src/service.rs#L892) | [endpoint::service::NativeEndpoint::cancel_with_operation](../../src/service.rs#L895) |
| `cancel_with_operation` | `validate_session_id` | [902](../../src/service.rs#L902) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `cancel_with_operation` | `self.validate_origin` | [903](../../src/service.rs#L903) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `cancel_with_operation` | `self             .store             .append_keyed_with_projection(session_id, origin, &#124;seq, projection&#124; {                 let generation = projection                     .events                     .iter()                     .filter(&#124;item&#124; matches!(item.kind(), schema::EventKind::StopRequested))                     .filter_map(&#124;item&#124; item.integer_field("generation"))                     .max()                     .unwrap_or(0)                     + 1;                 event(json!({                 "v": 1, "seq": seq, "ts": timestamp, "kind": "stop_requested",                 "generation": generation, "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map_err(store_error)             })             .map_err` | [904](../../src/service.rs#L904) | receiver-type-required |
| `cancel_with_operation` | `self             .store             .append_keyed_with_projection` | [904](../../src/service.rs#L904) | receiver-type-required |
| `cancel_with_operation` | `projection                     .events                     .iter()                     .filter(&#124;item&#124; matches!(item.kind(), schema::EventKind::StopRequested))                     .filter_map(&#124;item&#124; item.integer_field("generation"))                     .max()                     .unwrap_or` | [907](../../src/service.rs#L907) | receiver-type-required |
| `cancel_with_operation` | `projection                     .events                     .iter()                     .filter(&#124;item&#124; matches!(item.kind(), schema::EventKind::StopRequested))                     .filter_map(&#124;item&#124; item.integer_field("generation"))                     .max` | [907](../../src/service.rs#L907) | receiver-type-required |
| `cancel_with_operation` | `projection                     .events                     .iter()                     .filter(&#124;item&#124; matches!(item.kind(), schema::EventKind::StopRequested))                     .filter_map` | [907](../../src/service.rs#L907) | receiver-type-required |
| `cancel_with_operation` | `projection                     .events                     .iter()                     .filter` | [907](../../src/service.rs#L907) | receiver-type-required |
| `cancel_with_operation` | `projection                     .events                     .iter` | [907](../../src/service.rs#L907) | receiver-type-required |
| `cancel_with_operation` | `item.integer_field` | [911](../../src/service.rs#L911) | receiver-type-required |
| `cancel_with_operation` | `event(json!({                 "v": 1, "seq": seq, "ts": timestamp, "kind": "stop_requested",                 "generation": generation, "origin_key": origin.key, "origin_tuple": origin,                 }))                 .map_err` | [915](../../src/service.rs#L915) | receiver-type-required |
| `cancel_with_operation` | `event` | [915](../../src/service.rs#L915) | [endpoint::service::event](../../src/service.rs#L1036) |
| `cancel_with_operation` | `Ok` | [922](../../src/service.rs#L922) | external-constructor-callback-or-unresolved |
| `cancel_with_operation` | `outcome.into` | [922](../../src/service.rs#L922) | receiver-type-required |
| `fork_session` | `validate_session_id` | [932](../../src/service.rs#L932), [933](../../src/service.rs#L933) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `fork_session` | `self.store             .begin_fork(operation_id, source, destination, timestamp)             .map_err` | [934](../../src/service.rs#L934) | receiver-type-required |
| `fork_session` | `self.store             .begin_fork` | [934](../../src/service.rs#L934) | receiver-type-required |
| `fork_session` | `self.store.recover_rewrites().map_err` | [937](../../src/service.rs#L937) | receiver-type-required |
| `fork_session` | `self.store.recover_rewrites` | [937](../../src/service.rs#L937) | receiver-type-required |
| `fork_session` | `Ok` | [938](../../src/service.rs#L938) | external-constructor-callback-or-unresolved |
| `fork_session` | `destination.to_owned` | [938](../../src/service.rs#L938) | receiver-type-required |
| `archive_session` | `validate_session_id` | [942](../../src/service.rs#L942) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `archive_session` | `self.store.archive(session_id).map_err` | [943](../../src/service.rs#L943) | receiver-type-required |
| `archive_session` | `self.store.archive` | [943](../../src/service.rs#L943) | receiver-type-required |
| `archive_session` | `Ok` | [944](../../src/service.rs#L944) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `validate_session_id` | [953](../../src/service.rs#L953) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `unarchive_session` | `self.store.unarchive(session_id).map_err` | [954](../../src/service.rs#L954) | receiver-type-required |
| `unarchive_session` | `self.store.unarchive` | [954](../../src/service.rs#L954) | receiver-type-required |
| `unarchive_session` | `Ok` | [955](../../src/service.rs#L955) | external-constructor-callback-or-unresolved |
| `unarchive_session` | `session_id.to_owned` | [956](../../src/service.rs#L956) | receiver-type-required |
| `reconcile_projection` | `self.reconciled_journal` | [961](../../src/service.rs#L961) | [endpoint::service::NativeEndpoint::reconciled_journal](../../src/service.rs#L968) |
| `reconcile_projection` | `journal.last_seq` | [962](../../src/service.rs#L962) | receiver-type-required |
| `reconcile_projection` | `i64::try_from(seq).map_err` | [963](../../src/service.rs#L963) | receiver-type-required |
| `reconcile_projection` | `i64::try_from` | [963](../../src/service.rs#L963) | external-constructor-callback-or-unresolved |
| `reconcile_projection` | `NativeEndpointError::SequenceOverflow` | [963](../../src/service.rs#L963) | external-constructor-callback-or-unresolved |
| `reconcile_projection` | `Ok` | [964](../../src/service.rs#L964) | external-constructor-callback-or-unresolved |
| `reconciled_journal` | `self.active_folder` | [969](../../src/service.rs#L969) | [endpoint::service::NativeEndpoint::active_folder](../../src/service.rs#L1023) |
| `reconciled_journal` | `fs::read` | [970](../../src/service.rs#L970) | external-constructor-callback-or-unresolved |
| `reconciled_journal` | `folder.join` | [970](../../src/service.rs#L970) | receiver-type-required |
| `reconciled_journal` | `scan_valid_prefix` | [971](../../src/service.rs#L971) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `reconciled_journal` | `scan.projection.ok_or_else` | [972](../../src/service.rs#L972) | receiver-type-required |
| `reconciled_journal` | `NativeEndpointError::Store` | [973](../../src/service.rs#L973) | external-constructor-callback-or-unresolved |
| `reconciled_journal` | `store::StoreError::Corruption` | [973](../../src/service.rs#L973) | external-constructor-callback-or-unresolved |
| `reconciled_journal` | `"thread ledger has no valid genesis prefix".to_owned` | [974](../../src/service.rs#L974) | receiver-type-required |
| `reconciled_journal` | `EndpointJournal::open` | [977](../../src/service.rs#L977) | [endpoint::journal::EndpointJournal::open](../../src/journal.rs#L225) |
| `reconciled_journal` | `Projector::default().reconcile` | [978](../../src/service.rs#L978) | receiver-type-required |
| `reconciled_journal` | `Projector::default` | [978](../../src/service.rs#L978) | external-constructor-callback-or-unresolved |
| `reconciled_journal` | `Ok` | [979](../../src/service.rs#L979) | external-constructor-callback-or-unresolved |
| `history` | `self.reconciled_journal` | [988](../../src/service.rs#L988) | [endpoint::service::NativeEndpoint::reconciled_journal](../../src/service.rs#L968) |
| `history` | `Ok` | [989](../../src/service.rs#L989) | external-constructor-callback-or-unresolved |
| `history` | `history_page` | [989](../../src/service.rs#L989) | [endpoint::history::history_page](../../src/history.rs#L13) |
| `history` | `journal.records` | [989](../../src/service.rs#L989) | receiver-type-required |
| `keyed` | `self.validate_origin` | [1002](../../src/service.rs#L1002) | [endpoint::service::NativeEndpoint::validate_origin](../../src/service.rs#L1010) |
| `keyed` | `self             .store             .append_keyed(session_id, origin, &#124;seq&#124; build(seq).map_err(store_error))             .map_err` | [1003](../../src/service.rs#L1003) | receiver-type-required |
| `keyed` | `self             .store             .append_keyed` | [1003](../../src/service.rs#L1003) | receiver-type-required |
| `keyed` | `build(seq).map_err` | [1005](../../src/service.rs#L1005) | receiver-type-required |
| `keyed` | `build` | [1005](../../src/service.rs#L1005) | external-constructor-callback-or-unresolved |
| `keyed` | `Ok` | [1007](../../src/service.rs#L1007) | external-constructor-callback-or-unresolved |
| `keyed` | `outcome.into` | [1007](../../src/service.rs#L1007) | receiver-type-required |
| `validate_origin` | `validate_session_id` | [1016](../../src/service.rs#L1016) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `validate_origin` | `Err` | [1018](../../src/service.rs#L1018) | external-constructor-callback-or-unresolved |
| `validate_origin` | `Ok` | [1020](../../src/service.rs#L1020) | external-constructor-callback-or-unresolved |
| `active_folder` | `validate_session_id` | [1024](../../src/service.rs#L1024) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `active_folder` | `self.store.root().join("archive").join(session_id).is_dir` | [1025](../../src/service.rs#L1025) | receiver-type-required |
| `active_folder` | `self.store.root().join("archive").join` | [1025](../../src/service.rs#L1025) | receiver-type-required |
| `active_folder` | `self.store.root().join` | [1025](../../src/service.rs#L1025), [1028](../../src/service.rs#L1028) | receiver-type-required |
| `active_folder` | `self.store.root` | [1025](../../src/service.rs#L1025), [1028](../../src/service.rs#L1028) | receiver-type-required |
| `active_folder` | `Err` | [1026](../../src/service.rs#L1026), [1030](../../src/service.rs#L1030) | external-constructor-callback-or-unresolved |
| `active_folder` | `self.store.root().join("threads").join` | [1028](../../src/service.rs#L1028) | receiver-type-required |
| `active_folder` | `folder.is_dir` | [1029](../../src/service.rs#L1029) | receiver-type-required |
| `active_folder` | `Ok` | [1032](../../src/service.rs#L1032) | external-constructor-callback-or-unresolved |
| `event` | `serde_json::to_vec` | [1037](../../src/service.rs#L1037) | external-constructor-callback-or-unresolved |
| `event` | `Ok` | [1038](../../src/service.rs#L1038) | external-constructor-callback-or-unresolved |
| `event` | `Event::from_value` | [1038](../../src/service.rs#L1038) | [schema::event::Event::from_value](../../../schema/src/event.rs#L182) |
| `event` | `IJsonValue::parse` | [1038](../../src/service.rs#L1038) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `store_error` | `store::StoreError::Corruption` | [1042](../../src/service.rs#L1042) | external-constructor-callback-or-unresolved |
| `store_error` | `error.to_string` | [1042](../../src/service.rs#L1042) | receiver-type-required |
| `map_store_error` | `NativeEndpointError::Store` | [1049](../../src/service.rs#L1049) | external-constructor-callback-or-unresolved |
