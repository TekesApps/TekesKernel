# endpoint::mux

[Package atlas](index.md) · [Source](../../src/mux.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::mux::SESSION_ENDPOINT_PROTOCOL_VERSION](../../src/mux.rs#L15) | const_item | `pub` |  |
| [endpoint::mux::DEFAULT_JOURNAL_WINDOW_MESSAGES](../../src/mux.rs#L16) | const_item | `pub` |  |
| [endpoint::mux::MAX_JOURNAL_WINDOW_MESSAGES](../../src/mux.rs#L17) | const_item | `pub` |  |
| [endpoint::mux::SessionEndpointCapability](../../src/mux.rs#L21) | enum_item | `pub` |  |
| [endpoint::mux::SessionEndpointCapability::required](../../src/mux.rs#L34) | function_item | `pub` |  |
| [endpoint::mux::MuxHostProduct](../../src/mux.rs#L49) | struct_item | `pub` |  |
| [endpoint::mux::MuxHostDescription](../../src/mux.rs#L56) | struct_item | `pub` |  |
| [endpoint::mux::MuxHostDescription::validate](../../src/mux.rs#L74) | function_item | `pub` |  |
| [endpoint::mux::SessionErrorCategory](../../src/mux.rs#L98) | enum_item | `pub` |  |
| [endpoint::mux::Retryability](../../src/mux.rs#L111) | enum_item | `pub` |  |
| [endpoint::mux::SessionRemoteError](../../src/mux.rs#L120) | struct_item | `pub` |  |
| [endpoint::mux::SessionAddress](../../src/mux.rs#L131) | struct_item | `pub` |  |
| [endpoint::mux::SessionAddress::validate](../../src/mux.rs#L137) | function_item | `pub` |  |
| [endpoint::mux::SessionStreamTarget](../../src/mux.rs#L144) | enum_item | `pub` |  |
| [endpoint::mux::SessionMuxClientFrame](../../src/mux.rs#L158) | enum_item | `pub` |  |
| [endpoint::mux::SessionMuxClientFrame::validate](../../src/mux.rs#L191) | function_item | `pub` |  |
| [endpoint::mux::WorkspaceSummary](../../src/mux.rs#L239) | struct_item | `pub` |  |
| [endpoint::mux::WorkspaceBaseline](../../src/mux.rs#L253) | struct_item | `pub` |  |
| [endpoint::mux::SessionSummary](../../src/mux.rs#L261) | struct_item | `pub` |  |
| [endpoint::mux::SessionJournalSnapshot](../../src/mux.rs#L309) | struct_item | `pub` |  |
| [endpoint::mux::SessionJournalPage](../../src/mux.rs#L323) | struct_item | `pub` |  |
| [endpoint::mux::SessionControlItem](../../src/mux.rs#L334) | struct_item | `pub` |  |
| [endpoint::mux::SessionActionableKind](../../src/mux.rs#L344) | enum_item | `pub` |  |
| [endpoint::mux::SessionActionable](../../src/mux.rs#L351) | struct_item | `pub` |  |
| [endpoint::mux::SessionActionable::validate](../../src/mux.rs#L361) | function_item | `pub` |  |
| [endpoint::mux::SessionSyncFrame](../../src/mux.rs#L370) | enum_item | `pub` |  |
| [endpoint::mux::SessionSyncFrame::generation](../../src/mux.rs#L455) | function_item | `pub` |  |
| [endpoint::mux::SessionSyncFrame::is_baseline](../../src/mux.rs#L478) | function_item | `pub` |  |
| [endpoint::mux::SessionMuxServerFrame](../../src/mux.rs#L492) | enum_item | `pub` |  |
| [endpoint::mux::SessionMuxGeneration](../../src/mux.rs#L526) | struct_item | `pub` |  |
| [endpoint::mux::StreamState](../../src/mux.rs#L532) | struct_item | `private` |  |
| [endpoint::mux::MuxStreamKind](../../src/mux.rs#L539) | enum_item | `private` |  |
| [endpoint::mux::SessionMuxGeneration::new](../../src/mux.rs#L548) | function_item | `pub` |  |
| [endpoint::mux::SessionMuxGeneration::generation](../../src/mux.rs#L559) | function_item | `pub` |  |
| [endpoint::mux::SessionMuxGeneration::open](../../src/mux.rs#L563) | function_item | `pub` |  |
| [endpoint::mux::SessionMuxGeneration::close](../../src/mux.rs#L598) | function_item | `pub` |  |
| [endpoint::mux::SessionMuxGeneration::accept](../../src/mux.rs#L605) | function_item | `pub` |  |
| [endpoint::mux::JournalFollow](../../src/mux.rs#L678) | struct_item | `pub` |  |
| [endpoint::mux::open_journal_follow](../../src/mux.rs#L683) | function_item | `pub` |  |
| [endpoint::mux::frozen_history_page](../../src/mux.rs#L714) | function_item | `pub` |  |
| [endpoint::mux::ActionableRegistry](../../src/mux.rs#L747) | struct_item | `pub` |  |
| [endpoint::mux::ActionableRegistry::upsert](../../src/mux.rs#L752) | function_item | `pub` |  |
| [endpoint::mux::ActionableRegistry::baseline](../../src/mux.rs#L778) | function_item | `pub` |  |
| [endpoint::mux::ActionableRegistry::resolve](../../src/mux.rs#L788) | function_item | `pub` |  |
| [endpoint::mux::frame_kind](../../src/mux.rs#L814) | function_item | `private` |  |
| [endpoint::mux::validate_identifier](../../src/mux.rs#L837) | function_item | `private` |  |
| [endpoint::mux::validate_window](../../src/mux.rs#L844) | function_item | `private` |  |
| [endpoint::mux::validate_journal_entries](../../src/mux.rs#L851) | function_item | `private` |  |
| [endpoint::mux::MuxProtocolError](../../src/mux.rs#L875) | enum_item | `pub` |  |
| [endpoint::mux::default_window_limit](../../src/mux.rs#L926) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `HistoryError` | `crate::history::HistoryError` | `private` |
| `EndpointTypeError` | `crate::types::EndpointTypeError` | `private` |
| `EndpointJournal` | `crate::EndpointJournal` | `private` |
| `EndpointSubscription` | `crate::EndpointSubscription` | `private` |
| `EndpointSubscriptionHub` | `crate::EndpointSubscriptionHub` | `private` |
| `HistoryPage` | `crate::HistoryPage` | `private` |
| `JournalRecord` | `crate::JournalRecord` | `private` |
| `SessionEvent` | `crate::SessionEvent` | `private` |
| `SessionHistoryEntry` | `crate::SessionHistoryEntry` | `private` |
| `SessionToolEventView` | `crate::SessionToolEventView` | `private` |
| `history_page` | `crate::history_page` | `private` |
| `validate_session_id` | `crate::validate_session_id` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 16 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::history::history_page"]
  n1["endpoint::mux::SessionAddress::validate"]
  n2["endpoint::mux::SessionMuxClientFrame::validate"]
  n3["endpoint::mux::SessionEndpointCapability::required"]
  n4["endpoint::mux::SessionActionable::validate"]
  n5["endpoint::mux::SessionSyncFrame::generation"]
  n6["endpoint::mux::SessionSyncFrame::is_baseline"]
  n7["endpoint::mux::SessionMuxGeneration::new"]
  n8["endpoint::mux::SessionMuxGeneration::generation"]
  n9["endpoint::mux::SessionMuxGeneration::open"]
  n10["endpoint::mux::SessionMuxGeneration::close"]
  n11["endpoint::mux::SessionMuxGeneration::accept"]
  n12["endpoint::mux::open_journal_follow"]
  n13["endpoint::mux::frozen_history_page"]
  n14["endpoint::mux::MuxHostDescription::validate"]
  n15["endpoint::mux::ActionableRegistry::upsert"]
  n16["endpoint::mux::ActionableRegistry::baseline"]
  n17["endpoint::mux::ActionableRegistry::resolve"]
  n18["endpoint::mux::frame_kind"]
  n19["endpoint::mux::validate_identifier"]
  n20["endpoint::mux::validate_window"]
  n21["endpoint::mux::validate_journal_entries"]
  n22["endpoint::types::validate_session_id"]
  n1 --> n22
  n2 --> n19
  n2 --> n20
  n4 --> n19
  n4 --> n22
  n9 --> n19
  n9 --> n20
  n11 --> n18
  n11 --> n19
  n11 --> n21
  n12 --> n13
  n12 --> n20
  n13 --> n0
  n13 --> n20
  n14 --> n3
  n17 --> n19
```

</details>

<details><summary>Functions 21–22: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::mux::validate_journal_entries"]
  n1["endpoint::mux::default_window_limit"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `required` | `[             Self::WorkspaceSync,             Self::SessionInventorySync,             Self::SessionJournal,             Self::SessionControlSync,             Self::ActionableSync,         ]         .into_iter()         .collect` | [35](../../src/mux.rs#L35) | receiver-type-required |
| `required` | `[             Self::WorkspaceSync,             Self::SessionInventorySync,             Self::SessionJournal,             Self::SessionControlSync,             Self::ActionableSync,         ]         .into_iter` | [35](../../src/mux.rs#L35) | receiver-type-required |
| `validate` | `Err` | [76](../../src/mux.rs#L76), [83](../../src/mux.rs#L83), [90](../../src/mux.rs#L90) | external-constructor-callback-or-unresolved |
| `validate` | `MuxProtocolError::ProtocolVersion` | [76](../../src/mux.rs#L76) | external-constructor-callback-or-unresolved |
| `validate` | `self.product.name.is_empty` | [78](../../src/mux.rs#L78) | receiver-type-required |
| `validate` | `self.product.version.is_empty` | [79](../../src/mux.rs#L79) | receiver-type-required |
| `validate` | `self.cwd.is_empty` | [80](../../src/mux.rs#L80) | receiver-type-required |
| `validate` | `self.home.is_empty` | [81](../../src/mux.rs#L81) | receiver-type-required |
| `validate` | `SessionEndpointCapability::required()             .difference(&self.capabilities)             .copied()             .collect::<Vec<_>>` | [85](../../src/mux.rs#L85) | receiver-type-required |
| `validate` | `SessionEndpointCapability::required()             .difference(&self.capabilities)             .copied` | [85](../../src/mux.rs#L85) | receiver-type-required |
| `validate` | `SessionEndpointCapability::required()             .difference` | [85](../../src/mux.rs#L85) | receiver-type-required |
| `validate` | `SessionEndpointCapability::required` | [85](../../src/mux.rs#L85) | [endpoint::mux::SessionEndpointCapability::required](../../src/mux.rs#L34) |
| `validate` | `missing.is_empty` | [89](../../src/mux.rs#L89) | receiver-type-required |
| `validate` | `MuxProtocolError::MissingCapabilities` | [90](../../src/mux.rs#L90) | external-constructor-callback-or-unresolved |
| `validate` | `Ok` | [92](../../src/mux.rs#L92) | external-constructor-callback-or-unresolved |
| `validate` | `validate_session_id(&self.session_id).map_err` | [138](../../src/mux.rs#L138) | receiver-type-required |
| `validate` | `validate_session_id` | [138](../../src/mux.rs#L138) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `validate` | `validate_identifier` | [194](../../src/mux.rs#L194), [204](../../src/mux.rs#L204), [212](../../src/mux.rs#L212), [228](../../src/mux.rs#L228), [229](../../src/mux.rs#L229) | [endpoint::mux::validate_identifier](../../src/mux.rs#L837) |
| `validate` | `address.validate` | [200](../../src/mux.rs#L200), [213](../../src/mux.rs#L213) | receiver-type-required |
| `validate` | `validate_window` | [201](../../src/mux.rs#L201), [214](../../src/mux.rs#L214) | [endpoint::mux::validate_window](../../src/mux.rs#L844) |
| `validate` | `before_sequence                         .is_some_and` | [216](../../src/mux.rs#L216) | receiver-type-required |
| `validate` | `Err` | [219](../../src/mux.rs#L219) | external-constructor-callback-or-unresolved |
| `validate` | `Ok` | [233](../../src/mux.rs#L233) | external-constructor-callback-or-unresolved |
| `validate` | `validate_identifier` | [362](../../src/mux.rs#L362) | [endpoint::mux::validate_identifier](../../src/mux.rs#L837) |
| `validate` | `validate_session_id(&self.session_id).map_err` | [363](../../src/mux.rs#L363) | receiver-type-required |
| `validate` | `validate_session_id` | [363](../../src/mux.rs#L363) | [endpoint::types::validate_session_id](../../src/types.rs#L130) |
| `validate` | `Ok` | [364](../../src/mux.rs#L364) | external-constructor-callback-or-unresolved |
| `new` | `Err` | [550](../../src/mux.rs#L550) | external-constructor-callback-or-unresolved |
| `new` | `Ok` | [552](../../src/mux.rs#L552) | external-constructor-callback-or-unresolved |
| `new` | `BTreeMap::new` | [554](../../src/mux.rs#L554) | external-constructor-callback-or-unresolved |
| `open` | `stream_id.into` | [568](../../src/mux.rs#L568) | receiver-type-required |
| `open` | `validate_identifier` | [569](../../src/mux.rs#L569) | [endpoint::mux::validate_identifier](../../src/mux.rs#L837) |
| `open` | `self.streams.contains_key` | [570](../../src/mux.rs#L570) | receiver-type-required |
| `open` | `Err` | [571](../../src/mux.rs#L571) | external-constructor-callback-or-unresolved |
| `open` | `MuxProtocolError::DuplicateStream` | [571](../../src/mux.rs#L571) | external-constructor-callback-or-unresolved |
| `open` | `address.validate` | [580](../../src/mux.rs#L580) | receiver-type-required |
| `open` | `validate_window` | [581](../../src/mux.rs#L581) | [endpoint::mux::validate_window](../../src/mux.rs#L844) |
| `open` | `self.streams.insert` | [587](../../src/mux.rs#L587) | receiver-type-required |
| `open` | `Ok` | [595](../../src/mux.rs#L595) | external-constructor-callback-or-unresolved |
| `close` | `self.streams             .remove(stream_id)             .ok_or_else` | [599](../../src/mux.rs#L599) | receiver-type-required |
| `close` | `self.streams             .remove` | [599](../../src/mux.rs#L599) | receiver-type-required |
| `close` | `MuxProtocolError::UnknownStream` | [601](../../src/mux.rs#L601) | external-constructor-callback-or-unresolved |
| `close` | `stream_id.to_owned` | [601](../../src/mux.rs#L601) | receiver-type-required |
| `close` | `Ok` | [602](../../src/mux.rs#L602) | external-constructor-callback-or-unresolved |
| `accept` | `frame.generation` | [610](../../src/mux.rs#L610), [613](../../src/mux.rs#L613) | receiver-type-required |
| `accept` | `Err` | [611](../../src/mux.rs#L611), [621](../../src/mux.rs#L621), [624](../../src/mux.rs#L624), [628](../../src/mux.rs#L628), [645](../../src/mux.rs#L645), [656](../../src/mux.rs#L656) | external-constructor-callback-or-unresolved |
| `accept` | `self             .streams             .get_mut(stream_id)             .ok_or_else` | [616](../../src/mux.rs#L616) | receiver-type-required |
| `accept` | `self             .streams             .get_mut` | [616](../../src/mux.rs#L616) | receiver-type-required |
| `accept` | `MuxProtocolError::UnknownStream` | [619](../../src/mux.rs#L619) | external-constructor-callback-or-unresolved |
| `accept` | `stream_id.to_owned` | [619](../../src/mux.rs#L619), [621](../../src/mux.rs#L621), [624](../../src/mux.rs#L624), [628](../../src/mux.rs#L628), [641](../../src/mux.rs#L641), [656](../../src/mux.rs#L656) | receiver-type-required |
| `accept` | `frame.is_baseline` | [620](../../src/mux.rs#L620), [623](../../src/mux.rs#L623) | receiver-type-required |
| `accept` | `MuxProtocolError::BaselineRequired` | [621](../../src/mux.rs#L621), [641](../../src/mux.rs#L641) | external-constructor-callback-or-unresolved |
| `accept` | `MuxProtocolError::DuplicateBaseline` | [624](../../src/mux.rs#L624) | external-constructor-callback-or-unresolved |
| `accept` | `frame_kind` | [626](../../src/mux.rs#L626) | [endpoint::mux::frame_kind](../../src/mux.rs#L814) |
| `accept` | `MuxProtocolError::StreamKind` | [628](../../src/mux.rs#L628), [656](../../src/mux.rs#L656) | external-constructor-callback-or-unresolved |
| `accept` | `snapshot.address.validate` | [632](../../src/mux.rs#L632) | receiver-type-required |
| `accept` | `validate_journal_entries` | [633](../../src/mux.rs#L633) | [endpoint::mux::validate_journal_entries](../../src/mux.rs#L851) |
| `accept` | `Some` | [634](../../src/mux.rs#L634), [644](../../src/mux.rs#L644), [650](../../src/mux.rs#L650) | external-constructor-callback-or-unresolved |
| `accept` | `address.validate` | [637](../../src/mux.rs#L637), [653](../../src/mux.rs#L653) | receiver-type-required |
| `accept` | `event.validate().map_err` | [638](../../src/mux.rs#L638), [654](../../src/mux.rs#L654) | receiver-type-required |
| `accept` | `event.validate` | [638](../../src/mux.rs#L638), [654](../../src/mux.rs#L654) | receiver-type-required |
| `accept` | `state                     .journal_tail                     .ok_or_else(&#124;&#124; MuxProtocolError::BaselineRequired(stream_id.to_owned()))?                     .checked_add(1)                     .ok_or` | [639](../../src/mux.rs#L639) | receiver-type-required |
| `accept` | `state                     .journal_tail                     .ok_or_else(&#124;&#124; MuxProtocolError::BaselineRequired(stream_id.to_owned()))?                     .checked_add` | [639](../../src/mux.rs#L639) | receiver-type-required |
| `accept` | `state                     .journal_tail                     .ok_or_else` | [639](../../src/mux.rs#L639) | receiver-type-required |
| `accept` | `i64::try_from(event.seq).ok` | [644](../../src/mux.rs#L644) | receiver-type-required |
| `accept` | `i64::try_from` | [644](../../src/mux.rs#L644) | external-constructor-callback-or-unresolved |
| `accept` | `actionable.validate` | [661](../../src/mux.rs#L661), [665](../../src/mux.rs#L665) | receiver-type-required |
| `accept` | `validate_identifier` | [668](../../src/mux.rs#L668) | [endpoint::mux::validate_identifier](../../src/mux.rs#L837) |
| `accept` | `Ok` | [674](../../src/mux.rs#L674) | external-constructor-callback-or-unresolved |
| `open_journal_follow` | `address.validate` | [691](../../src/mux.rs#L691) | receiver-type-required |
| `open_journal_follow` | `validate_window` | [692](../../src/mux.rs#L692) | [endpoint::mux::validate_window](../../src/mux.rs#L844) |
| `open_journal_follow` | `hub         .subscribe(&address.session_id, journal, subscribed_rpc_id.into())         .map_err` | [696](../../src/mux.rs#L696) | receiver-type-required |
| `open_journal_follow` | `hub         .subscribe` | [696](../../src/mux.rs#L696) | receiver-type-required |
| `open_journal_follow` | `subscribed_rpc_id.into` | [697](../../src/mux.rs#L697) | receiver-type-required |
| `open_journal_follow` | `subscription.baseline` | [699](../../src/mux.rs#L699) | receiver-type-required |
| `open_journal_follow` | `frozen_history_page` | [700](../../src/mux.rs#L700) | [endpoint::mux::frozen_history_page](../../src/mux.rs#L714) |
| `open_journal_follow` | `Ok` | [701](../../src/mux.rs#L701) | external-constructor-callback-or-unresolved |
| `frozen_history_page` | `validate_window` | [720](../../src/mux.rs#L720) | [endpoint::mux::validate_window](../../src/mux.rs#L844) |
| `frozen_history_page` | `before_sequence.is_some_and` | [722](../../src/mux.rs#L722) | receiver-type-required |
| `frozen_history_page` | `Err` | [724](../../src/mux.rs#L724), [732](../../src/mux.rs#L732) | external-constructor-callback-or-unresolved |
| `frozen_history_page` | `journal.records().map_err` | [726](../../src/mux.rs#L726) | receiver-type-required |
| `frozen_history_page` | `journal.records` | [726](../../src/mux.rs#L726) | receiver-type-required |
| `frozen_history_page` | `records         .last()         .map(&#124;record&#124; record.event.seq as i64)         .unwrap_or` | [727](../../src/mux.rs#L727) | receiver-type-required |
| `frozen_history_page` | `records         .last()         .map` | [727](../../src/mux.rs#L727) | receiver-type-required |
| `frozen_history_page` | `records         .last` | [727](../../src/mux.rs#L727) | receiver-type-required |
| `frozen_history_page` | `records         .into_iter()         .filter(&#124;record&#124; record.event.seq as i64 <= through_sequence)         .collect::<Vec<JournalRecord>>` | [734](../../src/mux.rs#L734) | receiver-type-required |
| `frozen_history_page` | `records         .into_iter()         .filter` | [734](../../src/mux.rs#L734) | receiver-type-required |
| `frozen_history_page` | `records         .into_iter` | [734](../../src/mux.rs#L734) | receiver-type-required |
| `frozen_history_page` | `history_page(         &frozen,         before_sequence.map(&#124;before&#124; before.max(0) as u64),         Some(max_messages),     )     .map_err` | [738](../../src/mux.rs#L738) | receiver-type-required |
| `frozen_history_page` | `history_page` | [738](../../src/mux.rs#L738) | [endpoint::history::history_page](../../src/history.rs#L13) |
| `frozen_history_page` | `before_sequence.map` | [740](../../src/mux.rs#L740) | receiver-type-required |
| `frozen_history_page` | `before.max` | [740](../../src/mux.rs#L740) | receiver-type-required |
| `frozen_history_page` | `Some` | [741](../../src/mux.rs#L741) | external-constructor-callback-or-unresolved |
| `upsert` | `actionable.validate` | [753](../../src/mux.rs#L753) | receiver-type-required |
| `upsert` | `self             .pending             .lock()             .map_err` | [754](../../src/mux.rs#L754) | receiver-type-required |
| `upsert` | `self             .pending             .lock` | [754](../../src/mux.rs#L754) | receiver-type-required |
| `upsert` | `pending.get` | [758](../../src/mux.rs#L758) | receiver-type-required |
| `upsert` | `Ok` | [759](../../src/mux.rs#L759), [769](../../src/mux.rs#L769), [773](../../src/mux.rs#L773) | external-constructor-callback-or-unresolved |
| `upsert` | `Err` | [761](../../src/mux.rs#L761) | external-constructor-callback-or-unresolved |
| `upsert` | `pending.insert` | [768](../../src/mux.rs#L768), [772](../../src/mux.rs#L772) | receiver-type-required |
| `upsert` | `actionable.id.clone` | [768](../../src/mux.rs#L768), [772](../../src/mux.rs#L772) | receiver-type-required |
| `baseline` | `Ok` | [779](../../src/mux.rs#L779) | external-constructor-callback-or-unresolved |
| `baseline` | `self             .pending             .lock()             .map_err(&#124;_&#124; MuxProtocolError::Poisoned)?             .values()             .cloned()             .collect` | [779](../../src/mux.rs#L779) | receiver-type-required |
| `baseline` | `self             .pending             .lock()             .map_err(&#124;_&#124; MuxProtocolError::Poisoned)?             .values()             .cloned` | [779](../../src/mux.rs#L779) | receiver-type-required |
| `baseline` | `self             .pending             .lock()             .map_err(&#124;_&#124; MuxProtocolError::Poisoned)?             .values` | [779](../../src/mux.rs#L779) | receiver-type-required |
| `baseline` | `self             .pending             .lock()             .map_err` | [779](../../src/mux.rs#L779) | receiver-type-required |
| `baseline` | `self             .pending             .lock` | [779](../../src/mux.rs#L779) | receiver-type-required |
| `resolve` | `validate_identifier` | [793](../../src/mux.rs#L793) | [endpoint::mux::validate_identifier](../../src/mux.rs#L837) |
| `resolve` | `self             .pending             .lock()             .map_err` | [794](../../src/mux.rs#L794) | receiver-type-required |
| `resolve` | `self             .pending             .lock` | [794](../../src/mux.rs#L794) | receiver-type-required |
| `resolve` | `pending             .get(actionable_id)             .ok_or_else` | [798](../../src/mux.rs#L798) | receiver-type-required |
| `resolve` | `pending             .get` | [798](../../src/mux.rs#L798) | receiver-type-required |
| `resolve` | `MuxProtocolError::UnknownActionable` | [800](../../src/mux.rs#L800), [810](../../src/mux.rs#L810) | external-constructor-callback-or-unresolved |
| `resolve` | `actionable_id.to_owned` | [800](../../src/mux.rs#L800), [803](../../src/mux.rs#L803), [810](../../src/mux.rs#L810) | receiver-type-required |
| `resolve` | `Err` | [802](../../src/mux.rs#L802) | external-constructor-callback-or-unresolved |
| `resolve` | `pending             .remove(actionable_id)             .ok_or_else` | [808](../../src/mux.rs#L808) | receiver-type-required |
| `resolve` | `pending             .remove` | [808](../../src/mux.rs#L808) | receiver-type-required |
| `validate_identifier` | `value.is_empty` | [838](../../src/mux.rs#L838) | receiver-type-required |
| `validate_identifier` | `value.len` | [838](../../src/mux.rs#L838) | receiver-type-required |
| `validate_identifier` | `value.chars().any` | [838](../../src/mux.rs#L838) | receiver-type-required |
| `validate_identifier` | `value.chars` | [838](../../src/mux.rs#L838) | receiver-type-required |
| `validate_identifier` | `Err` | [839](../../src/mux.rs#L839) | external-constructor-callback-or-unresolved |
| `validate_identifier` | `Ok` | [841](../../src/mux.rs#L841) | external-constructor-callback-or-unresolved |
| `validate_window` | `(1..=MAX_JOURNAL_WINDOW_MESSAGES).contains` | [845](../../src/mux.rs#L845) | receiver-type-required |
| `validate_window` | `Err` | [846](../../src/mux.rs#L846) | external-constructor-callback-or-unresolved |
| `validate_window` | `MuxProtocolError::JournalWindow` | [846](../../src/mux.rs#L846) | external-constructor-callback-or-unresolved |
| `validate_window` | `Ok` | [848](../../src/mux.rs#L848) | external-constructor-callback-or-unresolved |
| `validate_journal_entries` | `Err` | [856](../../src/mux.rs#L856), [867](../../src/mux.rs#L867) | external-constructor-callback-or-unresolved |
| `validate_journal_entries` | `entry             .event             .validate()             .map_err` | [860](../../src/mux.rs#L860) | receiver-type-required |
| `validate_journal_entries` | `entry             .event             .validate` | [860](../../src/mux.rs#L860) | receiver-type-required |
| `validate_journal_entries` | `previous.is_some_and` | [865](../../src/mux.rs#L865) | receiver-type-required |
| `validate_journal_entries` | `Some` | [869](../../src/mux.rs#L869) | external-constructor-callback-or-unresolved |
| `validate_journal_entries` | `Ok` | [871](../../src/mux.rs#L871) | external-constructor-callback-or-unresolved |
