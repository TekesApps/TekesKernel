# endpoint::carrier_adapter

[Package atlas](index.md) · [Source](../../src/carrier_adapter.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::carrier_adapter::SESSION_ENDPOINT_PUBLIC_METHODS](../../src/carrier_adapter.rs#L14) | const_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthority](../../src/carrier_adapter.rs#L34) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthority::locate](../../src/carrier_adapter.rs#L38) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::RespondAuthority::author](../../src/carrier_adapter.rs#L43) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::RespondDelivery](../../src/carrier_adapter.rs#L50) | enum_item | `pub` |  |
| [endpoint::carrier_adapter::RespondDelivery::as_str](../../src/carrier_adapter.rs#L58) | function_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthorReceipt](../../src/carrier_adapter.rs#L68) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::LocatedRespond](../../src/carrier_adapter.rs#L74) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler](../../src/carrier_adapter.rs#L85) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler::respond](../../src/carrier_adapter.rs#L86) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler::respond_mux](../../src/carrier_adapter.rs#L92) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::JournalRespondHandler](../../src/carrier_adapter.rs#L101) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::new](../../src/carrier_adapter.rs#L108) | function_item | `pub` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::respond](../../src/carrier_adapter.rs#L117) | function_item | `private` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::respond_mux](../../src/carrier_adapter.rs#L255) | function_item | `private` |  |
| [endpoint::carrier_adapter::respond_handoff](../../src/carrier_adapter.rs#L331) | function_item | `private` |  |
| [endpoint::carrier_adapter::respond_claim](../../src/carrier_adapter.rs#L346) | function_item | `private` |  |
| [endpoint::carrier_adapter::accepted_result](../../src/carrier_adapter.rs#L360) | function_item | `private` |  |
| [endpoint::carrier_adapter::receipt_from_cached](../../src/carrier_adapter.rs#L371) | function_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler](../../src/carrier_adapter.rs#L394) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::open_stream](../../src/carrier_adapter.rs#L395) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::stream_error](../../src/carrier_adapter.rs#L400) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::mux_description](../../src/carrier_adapter.rs#L406) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::open_mux_stream](../../src/carrier_adapter.rs#L408) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::journal_page](../../src/carrier_adapter.rs#L414) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost](../../src/carrier_adapter.rs#L423) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::new](../../src/carrier_adapter.rs#L433) | function_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::set_readiness](../../src/carrier_adapter.rs#L443) | function_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::readiness](../../src/carrier_adapter.rs#L455) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::registered_methods](../../src/carrier_adapter.rs#L463) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::advertised_extension_methods](../../src/carrier_adapter.rs#L479) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::classify_method](../../src/carrier_adapter.rs#L483) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::unary](../../src/carrier_adapter.rs#L496) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::respond](../../src/carrier_adapter.rs#L510) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_stream](../../src/carrier_adapter.rs#L518) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::stream_error](../../src/carrier_adapter.rs#L525) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_description](../../src/carrier_adapter.rs#L533) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_mux_stream](../../src/carrier_adapter.rs#L537) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::journal_page](../../src/carrier_adapter.rs#L545) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_respond_actionable](../../src/carrier_adapter.rs#L552) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeSet` | `std::collections::BTreeSet` | `private` |
| `PathBuf` | `std::path::PathBuf` | `private` |
| `AtomicBool` | `std::sync::atomic::AtomicBool` | `private` |
| `Ordering` | `std::sync::atomic::Ordering` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `CallContext` | `crate::CallContext` | `private` |
| `CarrierHostFuture` | `crate::CarrierHostFuture` | `private` |
| `ClientRequest` | `crate::ClientRequest` | `private` |
| `ClientResponse` | `crate::ClientResponse` | `private` |
| `EndpointCarrierHost` | `crate::EndpointCarrierHost` | `private` |
| `EndpointDispatcher` | `crate::EndpointDispatcher` | `private` |
| `EndpointHost` | `crate::EndpointHost` | `private` |
| `EndpointStreamReceiver` | `crate::EndpointStreamReceiver` | `private` |
| `HostFailure` | `crate::HostFailure` | `private` |
| `HostReadiness` | `crate::HostReadiness` | `private` |
| `MethodClass` | `crate::MethodClass` | `private` |
| `RequestState` | `crate::RequestState` | `private` |
| `RespondAuthorization` | `crate::RespondAuthorization` | `private` |
| `RespondDecision` | `crate::RespondDecision` | `private` |
| `RespondLifecycle` | `crate::RespondLifecycle` | `private` |
| `RespondPrepareContext` | `crate::RespondPrepareContext` | `private` |
| `RespondReceipt` | `crate::RespondReceipt` | `private` |
| `RpcBegin` | `crate::RpcBegin` | `private` |
| `RpcDurableIdentity` | `crate::RpcDurableIdentity` | `private` |
| `RpcLookup` | `crate::RpcLookup` | `private` |
| `RpcRegistry` | `crate::RpcRegistry` | `private` |
| `RpcResult` | `crate::RpcResult` | `private` |
| `ServerRequest` | `crate::ServerRequest` | `private` |
| `ServerResponse` | `crate::ServerResponse` | `private` |
| `StreamChannel` | `crate::StreamChannel` | `private` |
| `StreamErrorCode` | `crate::StreamErrorCode` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–20: 9 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::carrier_adapter::JournalRespondHandler::new"]
  n1["endpoint::carrier_adapter::JournalRespondHandler::respond"]
  n2["endpoint::carrier_adapter::JournalRespondHandler::respond_mux"]
  n3["endpoint::carrier_adapter::respond_handoff"]
  n4["endpoint::carrier_adapter::respond_claim"]
  n5["endpoint::carrier_adapter::accepted_result"]
  n6["endpoint::carrier_adapter::receipt_from_cached"]
  n7["endpoint::carrier_adapter::ComposedEndpointCarrierHost::new"]
  n8["endpoint::carrier_adapter::ComposedEndpointCarrierHost::set_readiness"]
  n9["endpoint::carrier_adapter::ComposedEndpointCarrierHost::readiness"]
  n10["endpoint::carrier_adapter::ComposedEndpointCarrierHost::registered_methods"]
  n11["endpoint::carrier_adapter::ComposedEndpointCarrierHost::advertised_extension_methods"]
  n12["endpoint::carrier_adapter::ComposedEndpointCarrierHost::classify_method"]
  n13["endpoint::carrier_adapter::ComposedEndpointCarrierHost::unary"]
  n14["endpoint::carrier_adapter::ComposedEndpointCarrierHost::respond"]
  n15["endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_stream"]
  n16["endpoint::carrier_adapter::ComposedEndpointCarrierHost::stream_error"]
  n17["endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_description"]
  n18["endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_mux_stream"]
  n19["endpoint::carrier_adapter::RespondDelivery::as_str"]
  n20["endpoint::host::EndpointDispatcher::new"]
  n21["endpoint::respond::RespondLifecycle::prepare"]
  n22["schema::ijson::IJsonValue::parse"]
  n23["schema::ijson::IJsonValue::parse_str"]
  n1 --> n3
  n1 --> n4
  n1 --> n5
  n1 --> n6
  n1 --> n21
  n2 --> n22
  n4 --> n22
  n5 --> n23
  n7 --> n20
```

</details>

<details><summary>Functions 21–22: 0 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::carrier_adapter::ComposedEndpointCarrierHost::journal_page"]
  n1["endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_respond_actionable"]
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `respond` | `Box::pin` | [122](../../src/carrier_adapter.rs#L122) | external-constructor-callback-or-unresolved |
| `respond` | `response                 .validate()                 .map_err` | [123](../../src/carrier_adapter.rs#L123) | receiver-type-required |
| `respond` | `response                 .validate` | [123](../../src/carrier_adapter.rs#L123) | receiver-type-required |
| `respond` | `HostFailure::Protocol` | [125](../../src/carrier_adapter.rs#L125) | external-constructor-callback-or-unresolved |
| `respond` | `error.to_string` | [125](../../src/carrier_adapter.rs#L125), [160](../../src/carrier_adapter.rs#L160), [182](../../src/carrier_adapter.rs#L182), [186](../../src/carrier_adapter.rs#L186), [193](../../src/carrier_adapter.rs#L193), [206](../../src/carrier_adapter.rs#L206), [210](../../src/carrier_adapter.rs#L210), [224](../../src/carrier_adapter.rs#L224), [232](../../src/carrier_adapter.rs#L232), [241](../../src/carrier_adapter.rs#L241), [248](../../src/carrier_adapter.rs#L248) | receiver-type-required |
| `respond` | `self.authority.locate` | [126](../../src/carrier_adapter.rs#L126), [139](../../src/carrier_adapter.rs#L139) | receiver-type-required |
| `respond` | `Ok` | [127](../../src/carrier_adapter.rs#L127), [140](../../src/carrier_adapter.rs#L140), [194](../../src/carrier_adapter.rs#L194), [202](../../src/carrier_adapter.rs#L202), [213](../../src/carrier_adapter.rs#L213), [249](../../src/carrier_adapter.rs#L249) | external-constructor-callback-or-unresolved |
| `respond` | `Some` | [129](../../src/carrier_adapter.rs#L129), [142](../../src/carrier_adapter.rs#L142), [175](../../src/carrier_adapter.rs#L175), [181](../../src/carrier_adapter.rs#L181), [191](../../src/carrier_adapter.rs#L191), [204](../../src/carrier_adapter.rs#L204), [209](../../src/carrier_adapter.rs#L209), [231](../../src/carrier_adapter.rs#L231), [246](../../src/carrier_adapter.rs#L246) | external-constructor-callback-or-unresolved |
| `respond` | `"unknown-rpc-id".to_owned` | [129](../../src/carrier_adapter.rs#L129), [142](../../src/carrier_adapter.rs#L142) | receiver-type-required |
| `respond` | `Arc::clone` | [132](../../src/carrier_adapter.rs#L132) | external-constructor-callback-or-unresolved |
| `respond` | `admission                 .lock()                 .map_err` | [133](../../src/carrier_adapter.rs#L133) | receiver-type-required |
| `respond` | `admission                 .lock` | [133](../../src/carrier_adapter.rs#L133) | receiver-type-required |
| `respond` | `HostFailure::Internal` | [135](../../src/carrier_adapter.rs#L135), [146](../../src/carrier_adapter.rs#L146), [160](../../src/carrier_adapter.rs#L160), [182](../../src/carrier_adapter.rs#L182), [186](../../src/carrier_adapter.rs#L186), [193](../../src/carrier_adapter.rs#L193), [199](../../src/carrier_adapter.rs#L199), [206](../../src/carrier_adapter.rs#L206), [210](../../src/carrier_adapter.rs#L210), [219](../../src/carrier_adapter.rs#L219), [224](../../src/carrier_adapter.rs#L224), [232](../../src/carrier_adapter.rs#L232), [241](../../src/carrier_adapter.rs#L241), [248](../../src/carrier_adapter.rs#L248) | external-constructor-callback-or-unresolved |
| `respond` | `"respond admission gate poisoned".to_owned` | [135](../../src/carrier_adapter.rs#L135) | receiver-type-required |
| `respond` | `Arc::ptr_eq` | [145](../../src/carrier_adapter.rs#L145) | external-constructor-callback-or-unresolved |
| `respond` | `Err` | [146](../../src/carrier_adapter.rs#L146), [199](../../src/carrier_adapter.rs#L199), [206](../../src/carrier_adapter.rs#L206), [219](../../src/carrier_adapter.rs#L219), [224](../../src/carrier_adapter.rs#L224) | external-constructor-callback-or-unresolved |
| `respond` | `"respond admission authority changed during lookup".to_owned` | [147](../../src/carrier_adapter.rs#L147) | receiver-type-required |
| `respond` | `respond_claim` | [150](../../src/carrier_adapter.rs#L150) | [endpoint::carrier_adapter::respond_claim](../../src/carrier_adapter.rs#L346) |
| `respond` | `state.resolution.is_some` | [152](../../src/carrier_adapter.rs#L152) | receiver-type-required |
| `respond` | `self.registry.lookup` | [153](../../src/carrier_adapter.rs#L153) | receiver-type-required |
| `respond` | `receipt_from_cached` | [154](../../src/carrier_adapter.rs#L154) | [endpoint::carrier_adapter::receipt_from_cached](../../src/carrier_adapter.rs#L371) |
| `respond` | `state.resolution.expect` | [156](../../src/carrier_adapter.rs#L156) | receiver-type-required |
| `respond` | `self                             .registry                             .handoff(&claim)                             .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?                             .map_or_else` | [157](../../src/carrier_adapter.rs#L157) | receiver-type-required |
| `respond` | `self                             .registry                             .handoff(&claim)                             .map_err` | [157](../../src/carrier_adapter.rs#L157) | receiver-type-required |
| `respond` | `self                             .registry                             .handoff` | [157](../../src/carrier_adapter.rs#L157) | receiver-type-required |
| `respond` | `respond_handoff` | [163](../../src/carrier_adapter.rs#L163), [229](../../src/carrier_adapter.rs#L229) | [endpoint::carrier_adapter::respond_handoff](../../src/carrier_adapter.rs#L331) |
| `respond` | `identity.unwrap_or_else` | [172](../../src/carrier_adapter.rs#L172) | receiver-type-required |
| `respond` | `"event".to_owned` | [173](../../src/carrier_adapter.rs#L173) | receiver-type-required |
| `respond` | `state.request.session_id.clone` | [174](../../src/carrier_adapter.rs#L174) | receiver-type-required |
| `respond` | `self.registry                             .mark_handed_off(&claim, &proof.0, Some(proof.1.clone()))                             .map_err` | [180](../../src/carrier_adapter.rs#L180) | receiver-type-required |
| `respond` | `self.registry                             .mark_handed_off` | [180](../../src/carrier_adapter.rs#L180) | receiver-type-required |
| `respond` | `proof.1.clone` | [181](../../src/carrier_adapter.rs#L181) | receiver-type-required |
| `respond` | `accepted_result` | [183](../../src/carrier_adapter.rs#L183), [240](../../src/carrier_adapter.rs#L240) | [endpoint::carrier_adapter::accepted_result](../../src/carrier_adapter.rs#L360) |
| `respond` | `self.registry                             .complete(&claim, &result)                             .map_err` | [184](../../src/carrier_adapter.rs#L184) | receiver-type-required |
| `respond` | `self.registry                             .complete` | [184](../../src/carrier_adapter.rs#L184) | receiver-type-required |
| `respond` | `context                             .handoff                             .mark_handed_off(crate::DurableHandoffProof {                                 delivery: proof.0,                                 durable_identity: Some(proof.1),                             })                             .map_err` | [187](../../src/carrier_adapter.rs#L187) | receiver-type-required |
| `respond` | `context                             .handoff                             .mark_handed_off` | [187](../../src/carrier_adapter.rs#L187) | receiver-type-required |
| `respond` | `"resolved endpoint request has no rpc carrier".to_owned` | [200](../../src/carrier_adapter.rs#L200) | receiver-type-required |
| `respond` | `"already-resolved".to_owned` | [204](../../src/carrier_adapter.rs#L204) | receiver-type-required |
| `respond` | `RespondLifecycle::prepare(Some(state), &response, &located.context)                 .map_err` | [209](../../src/carrier_adapter.rs#L209) | receiver-type-required |
| `respond` | `RespondLifecycle::prepare` | [209](../../src/carrier_adapter.rs#L209) | [endpoint::respond::RespondLifecycle::prepare](../../src/respond.rs#L68) |
| `respond` | `self.registry.begin` | [216](../../src/carrier_adapter.rs#L216) | receiver-type-required |
| `respond` | `"unresolved endpoint request has a completed rpc carrier"                                     .to_owned` | [220](../../src/carrier_adapter.rs#L220) | receiver-type-required |
| `respond` | `self.authority.author` | [226](../../src/carrier_adapter.rs#L226) | receiver-type-required |
| `respond` | `authorization.clone` | [226](../../src/carrier_adapter.rs#L226) | receiver-type-required |
| `respond` | `self.registry                         .mark_handed_off(&claim, &delivery, Some(durable_identity.clone()))                         .map_err` | [230](../../src/carrier_adapter.rs#L230) | receiver-type-required |
| `respond` | `self.registry                         .mark_handed_off` | [230](../../src/carrier_adapter.rs#L230) | receiver-type-required |
| `respond` | `durable_identity.clone` | [231](../../src/carrier_adapter.rs#L231) | receiver-type-required |
| `respond` | `self.registry                         .complete(&claim, &accepted_result()?)                         .map_err` | [239](../../src/carrier_adapter.rs#L239) | receiver-type-required |
| `respond` | `self.registry                         .complete` | [239](../../src/carrier_adapter.rs#L239) | receiver-type-required |
| `respond` | `context                         .handoff                         .mark_handed_off(crate::DurableHandoffProof {                             delivery,                             durable_identity: Some(durable_identity),                         })                         .map_err` | [242](../../src/carrier_adapter.rs#L242) | receiver-type-required |
| `respond` | `context                         .handoff                         .mark_handed_off` | [242](../../src/carrier_adapter.rs#L242) | receiver-type-required |
| `respond_mux` | `Box::pin` | [262](../../src/carrier_adapter.rs#L262) | external-constructor-callback-or-unresolved |
| `respond_mux` | `self.authority.locate` | [263](../../src/carrier_adapter.rs#L263) | receiver-type-required |
| `respond_mux` | `Err` | [264](../../src/carrier_adapter.rs#L264), [270](../../src/carrier_adapter.rs#L270), [275](../../src/carrier_adapter.rs#L275), [291](../../src/carrier_adapter.rs#L291), [320](../../src/carrier_adapter.rs#L320) | external-constructor-callback-or-unresolved |
| `respond_mux` | `HostFailure::Protocol` | [264](../../src/carrier_adapter.rs#L264), [270](../../src/carrier_adapter.rs#L270), [275](../../src/carrier_adapter.rs#L275), [283](../../src/carrier_adapter.rs#L283), [285](../../src/carrier_adapter.rs#L285), [287](../../src/carrier_adapter.rs#L287), [291](../../src/carrier_adapter.rs#L291), [313](../../src/carrier_adapter.rs#L313), [320](../../src/carrier_adapter.rs#L320) | external-constructor-callback-or-unresolved |
| `respond_mux` | `"actionable is no longer pending".to_owned` | [265](../../src/carrier_adapter.rs#L265) | receiver-type-required |
| `respond_mux` | `state.resolution.is_some` | [269](../../src/carrier_adapter.rs#L269) | receiver-type-required |
| `respond_mux` | `"actionable is already resolved".to_owned` | [271](../../src/carrier_adapter.rs#L271) | receiver-type-required |
| `respond_mux` | `serde_json::from_slice(                 &outcome                     .canonical_bytes()                     .map_err(&#124;error&#124; HostFailure::Protocol(error.to_string()))?,             )             .map_err` | [280](../../src/carrier_adapter.rs#L280) | receiver-type-required |
| `respond_mux` | `serde_json::from_slice` | [280](../../src/carrier_adapter.rs#L280) | external-constructor-callback-or-unresolved |
| `respond_mux` | `outcome                     .canonical_bytes()                     .map_err` | [281](../../src/carrier_adapter.rs#L281) | receiver-type-required |
| `respond_mux` | `outcome                     .canonical_bytes` | [281](../../src/carrier_adapter.rs#L281) | receiver-type-required |
| `respond_mux` | `error.to_string` | [283](../../src/carrier_adapter.rs#L283), [285](../../src/carrier_adapter.rs#L285), [311](../../src/carrier_adapter.rs#L311), [313](../../src/carrier_adapter.rs#L313) | receiver-type-required |
| `respond_mux` | `value.as_object_mut().ok_or_else` | [286](../../src/carrier_adapter.rs#L286) | receiver-type-required |
| `respond_mux` | `value.as_object_mut` | [286](../../src/carrier_adapter.rs#L286) | receiver-type-required |
| `respond_mux` | `"actionable outcome must be an object".to_owned` | [287](../../src/carrier_adapter.rs#L287) | receiver-type-required |
| `respond_mux` | `object.get("sessionId").and_then` | [289](../../src/carrier_adapter.rs#L289) | receiver-type-required |
| `respond_mux` | `object.get` | [289](../../src/carrier_adapter.rs#L289) | receiver-type-required |
| `respond_mux` | `"actionable outcome targets another session".to_owned` | [292](../../src/carrier_adapter.rs#L292) | receiver-type-required |
| `respond_mux` | `object.insert` | [297](../../src/carrier_adapter.rs#L297) | receiver-type-required |
| `respond_mux` | `"sessionId".to_owned` | [298](../../src/carrier_adapter.rs#L298) | receiver-type-required |
| `respond_mux` | `serde_json::Value::String` | [299](../../src/carrier_adapter.rs#L299) | external-constructor-callback-or-unresolved |
| `respond_mux` | `state.request.session_id.clone` | [299](../../src/carrier_adapter.rs#L299) | receiver-type-required |
| `respond_mux` | `"client-response".to_owned` | [304](../../src/carrier_adapter.rs#L304) | receiver-type-required |
| `respond_mux` | `Some` | [308](../../src/carrier_adapter.rs#L308) | external-constructor-callback-or-unresolved |
| `respond_mux` | `schema::IJsonValue::parse(                             &serde_json::to_vec(&value)                                 .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?,                         )                         .map_err` | [309](../../src/carrier_adapter.rs#L309) | receiver-type-required |
| `respond_mux` | `schema::IJsonValue::parse` | [309](../../src/carrier_adapter.rs#L309) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `respond_mux` | `serde_json::to_vec(&value)                                 .map_err` | [310](../../src/carrier_adapter.rs#L310) | receiver-type-required |
| `respond_mux` | `serde_json::to_vec` | [310](../../src/carrier_adapter.rs#L310) | external-constructor-callback-or-unresolved |
| `respond_mux` | `HostFailure::Internal` | [311](../../src/carrier_adapter.rs#L311) | external-constructor-callback-or-unresolved |
| `respond_mux` | `self.respond` | [318](../../src/carrier_adapter.rs#L318) | receiver-type-required |
| `respond_mux` | `receipt                         .reason                         .unwrap_or_else` | [321](../../src/carrier_adapter.rs#L321) | receiver-type-required |
| `respond_mux` | `"actionable response rejected".to_owned` | [323](../../src/carrier_adapter.rs#L323) | receiver-type-required |
| `respond_mux` | `Ok` | [326](../../src/carrier_adapter.rs#L326) | external-constructor-callback-or-unresolved |
| `respond_handoff` | `delivery.as_str().to_owned` | [337](../../src/carrier_adapter.rs#L337) | receiver-type-required |
| `respond_handoff` | `delivery.as_str` | [337](../../src/carrier_adapter.rs#L337) | receiver-type-required |
| `respond_handoff` | `"event".to_owned` | [339](../../src/carrier_adapter.rs#L339) | receiver-type-required |
| `respond_handoff` | `session_id.to_owned` | [340](../../src/carrier_adapter.rs#L340) | receiver-type-required |
| `respond_handoff` | `Some` | [341](../../src/carrier_adapter.rs#L341) | external-constructor-callback-or-unresolved |
| `respond_claim` | `schema::IJsonValue::parse(         &serde_json_canonicalizer::to_vec(response)             .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?,     )     .map_err` | [347](../../src/carrier_adapter.rs#L347) | receiver-type-required |
| `respond_claim` | `schema::IJsonValue::parse` | [347](../../src/carrier_adapter.rs#L347) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `respond_claim` | `serde_json_canonicalizer::to_vec(response)             .map_err` | [348](../../src/carrier_adapter.rs#L348) | receiver-type-required |
| `respond_claim` | `serde_json_canonicalizer::to_vec` | [348](../../src/carrier_adapter.rs#L348) | external-constructor-callback-or-unresolved |
| `respond_claim` | `HostFailure::Internal` | [349](../../src/carrier_adapter.rs#L349), [351](../../src/carrier_adapter.rs#L351) | external-constructor-callback-or-unresolved |
| `respond_claim` | `error.to_string` | [349](../../src/carrier_adapter.rs#L349), [351](../../src/carrier_adapter.rs#L351) | receiver-type-required |
| `respond_claim` | `Ok` | [352](../../src/carrier_adapter.rs#L352) | external-constructor-callback-or-unresolved |
| `respond_claim` | `"client-request".to_owned` | [353](../../src/carrier_adapter.rs#L353) | receiver-type-required |
| `respond_claim` | `response.rpc_id.clone` | [354](../../src/carrier_adapter.rs#L354) | receiver-type-required |
| `respond_claim` | `"respond".to_owned` | [355](../../src/carrier_adapter.rs#L355) | receiver-type-required |
| `accepted_result` | `Ok` | [361](../../src/carrier_adapter.rs#L361) | external-constructor-callback-or-unresolved |
| `accepted_result` | `Some` | [363](../../src/carrier_adapter.rs#L363) | external-constructor-callback-or-unresolved |
| `accepted_result` | `schema::IJsonValue::parse_str(r#"{"accepted":true}"#)                 .map_err` | [364](../../src/carrier_adapter.rs#L364) | receiver-type-required |
| `accepted_result` | `schema::IJsonValue::parse_str` | [364](../../src/carrier_adapter.rs#L364) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `accepted_result` | `HostFailure::Internal` | [365](../../src/carrier_adapter.rs#L365) | external-constructor-callback-or-unresolved |
| `accepted_result` | `error.to_string` | [365](../../src/carrier_adapter.rs#L365) | receiver-type-required |
| `receipt_from_cached` | `result.error.is_some` | [372](../../src/carrier_adapter.rs#L372) | receiver-type-required |
| `receipt_from_cached` | `Err` | [373](../../src/carrier_adapter.rs#L373), [388](../../src/carrier_adapter.rs#L388) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `HostFailure::Internal` | [373](../../src/carrier_adapter.rs#L373), [378](../../src/carrier_adapter.rs#L378), [382](../../src/carrier_adapter.rs#L382), [384](../../src/carrier_adapter.rs#L384), [388](../../src/carrier_adapter.rs#L388) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `"respond rpc carrier cached a non-success result".to_owned` | [374](../../src/carrier_adapter.rs#L374) | receiver-type-required |
| `receipt_from_cached` | `result.value.ok_or_else` | [377](../../src/carrier_adapter.rs#L377) | receiver-type-required |
| `receipt_from_cached` | `"respond rpc carrier omitted its success value".to_owned` | [378](../../src/carrier_adapter.rs#L378) | receiver-type-required |
| `receipt_from_cached` | `value         .canonical_bytes()         .map_err` | [380](../../src/carrier_adapter.rs#L380) | receiver-type-required |
| `receipt_from_cached` | `value         .canonical_bytes` | [380](../../src/carrier_adapter.rs#L380) | receiver-type-required |
| `receipt_from_cached` | `error.to_string` | [382](../../src/carrier_adapter.rs#L382), [384](../../src/carrier_adapter.rs#L384) | receiver-type-required |
| `receipt_from_cached` | `serde_json::from_slice(&bytes).map_err` | [384](../../src/carrier_adapter.rs#L384) | receiver-type-required |
| `receipt_from_cached` | `serde_json::from_slice` | [384](../../src/carrier_adapter.rs#L384) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `receipt.reason.is_none` | [385](../../src/carrier_adapter.rs#L385) | receiver-type-required |
| `receipt_from_cached` | `Ok` | [386](../../src/carrier_adapter.rs#L386) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `"respond rpc carrier cached invalid receipt bytes".to_owned` | [389](../../src/carrier_adapter.rs#L389) | receiver-type-required |
| `new` | `EndpointDispatcher::new` | [436](../../src/carrier_adapter.rs#L436) | [endpoint::host::EndpointDispatcher::new](../../src/host.rs#L444) |
| `new` | `AtomicBool::new` | [439](../../src/carrier_adapter.rs#L439) | external-constructor-callback-or-unresolved |
| `set_readiness` | `self.ready             .store` | [444](../../src/carrier_adapter.rs#L444) | receiver-type-required |
| `readiness` | `self.ready.load` | [456](../../src/carrier_adapter.rs#L456) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter()             .copied()             .collect::<BTreeSet<_>>` | [464](../../src/carrier_adapter.rs#L464) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter()             .copied` | [464](../../src/carrier_adapter.rs#L464) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter` | [464](../../src/carrier_adapter.rs#L464) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter()             .filter(&#124;method&#124; public.contains(method.as_str()))             .collect::<BTreeSet<_>>` | [468](../../src/carrier_adapter.rs#L468) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter()             .filter` | [468](../../src/carrier_adapter.rs#L468) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter` | [468](../../src/carrier_adapter.rs#L468) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities` | [468](../../src/carrier_adapter.rs#L468) | receiver-type-required |
| `registered_methods` | `public.contains` | [472](../../src/carrier_adapter.rs#L472) | receiver-type-required |
| `registered_methods` | `method.as_str` | [472](../../src/carrier_adapter.rs#L472) | receiver-type-required |
| `registered_methods` | `methods.insert` | [474](../../src/carrier_adapter.rs#L474) | receiver-type-required |
| `registered_methods` | `"remote.mux".to_owned` | [474](../../src/carrier_adapter.rs#L474) | receiver-type-required |
| `registered_methods` | `methods.extend` | [475](../../src/carrier_adapter.rs#L475) | receiver-type-required |
| `registered_methods` | `self.unary.extension_capabilities` | [475](../../src/carrier_adapter.rs#L475) | receiver-type-required |
| `advertised_extension_methods` | `self.unary.extension_capabilities` | [480](../../src/carrier_adapter.rs#L480) | receiver-type-required |
| `classify_method` | `SESSION_ENDPOINT_PUBLIC_METHODS.contains` | [486](../../src/carrier_adapter.rs#L486) | receiver-type-required |
| `classify_method` | `self.unary.capabilities().contains` | [487](../../src/carrier_adapter.rs#L487) | receiver-type-required |
| `classify_method` | `self.unary.capabilities` | [487](../../src/carrier_adapter.rs#L487) | receiver-type-required |
| `classify_method` | `self.unary.extension_capabilities().contains` | [488](../../src/carrier_adapter.rs#L488) | receiver-type-required |
| `classify_method` | `self.unary.extension_capabilities` | [488](../../src/carrier_adapter.rs#L488) | receiver-type-required |
| `classify_method` | `self.unary.method_class` | [490](../../src/carrier_adapter.rs#L490) | receiver-type-required |
| `unary` | `Box::pin` | [501](../../src/carrier_adapter.rs#L501) | external-constructor-callback-or-unresolved |
| `unary` | `self.unary.validate_request` | [502](../../src/carrier_adapter.rs#L502) | receiver-type-required |
| `unary` | `self.dispatcher                 .dispatch_with_handoff(&self.unary, request, context.handoff)                 .await                 .map_err` | [503](../../src/carrier_adapter.rs#L503) | receiver-type-required |
| `unary` | `self.dispatcher                 .dispatch_with_handoff` | [503](../../src/carrier_adapter.rs#L503) | receiver-type-required |
| `unary` | `HostFailure::Internal` | [506](../../src/carrier_adapter.rs#L506) | external-constructor-callback-or-unresolved |
| `unary` | `error.to_string` | [506](../../src/carrier_adapter.rs#L506) | receiver-type-required |
| `respond` | `self.respond.respond` | [515](../../src/carrier_adapter.rs#L515) | receiver-type-required |
| `open_stream` | `self.streams.open_stream` | [522](../../src/carrier_adapter.rs#L522) | receiver-type-required |
| `stream_error` | `self.streams.stream_error` | [530](../../src/carrier_adapter.rs#L530) | receiver-type-required |
| `mux_description` | `self.streams.mux_description` | [534](../../src/carrier_adapter.rs#L534) | receiver-type-required |
| `open_mux_stream` | `self.streams.open_mux_stream` | [542](../../src/carrier_adapter.rs#L542) | receiver-type-required |
| `journal_page` | `self.streams.journal_page` | [549](../../src/carrier_adapter.rs#L549) | receiver-type-required |
| `mux_respond_actionable` | `Box::pin` | [564](../../src/carrier_adapter.rs#L564) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `std::future::ready` | [564](../../src/carrier_adapter.rs#L564) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `Err` | [564](../../src/carrier_adapter.rs#L564) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `self.respond             .respond_mux` | [566](../../src/carrier_adapter.rs#L566) | receiver-type-required |
