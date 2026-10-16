# endpoint::carrier_adapter

[Package atlas](index.md) · [Source](../../src/carrier_adapter.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::carrier_adapter::SESSION_ENDPOINT_PUBLIC_METHODS](../../src/carrier_adapter.rs#L14) | const_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthority](../../src/carrier_adapter.rs#L31) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthority::locate](../../src/carrier_adapter.rs#L35) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::RespondAuthority::author](../../src/carrier_adapter.rs#L40) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::RespondDelivery](../../src/carrier_adapter.rs#L47) | enum_item | `pub` |  |
| [endpoint::carrier_adapter::RespondDelivery::as_str](../../src/carrier_adapter.rs#L55) | function_item | `pub` |  |
| [endpoint::carrier_adapter::RespondAuthorReceipt](../../src/carrier_adapter.rs#L65) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::LocatedRespond](../../src/carrier_adapter.rs#L71) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler](../../src/carrier_adapter.rs#L82) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler::respond](../../src/carrier_adapter.rs#L83) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierRespondHandler::respond_mux](../../src/carrier_adapter.rs#L89) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::JournalRespondHandler](../../src/carrier_adapter.rs#L98) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::new](../../src/carrier_adapter.rs#L105) | function_item | `pub` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::respond](../../src/carrier_adapter.rs#L114) | function_item | `private` |  |
| [endpoint::carrier_adapter::JournalRespondHandler::respond_mux](../../src/carrier_adapter.rs#L252) | function_item | `private` |  |
| [endpoint::carrier_adapter::respond_handoff](../../src/carrier_adapter.rs#L328) | function_item | `private` |  |
| [endpoint::carrier_adapter::respond_claim](../../src/carrier_adapter.rs#L343) | function_item | `private` |  |
| [endpoint::carrier_adapter::accepted_result](../../src/carrier_adapter.rs#L357) | function_item | `private` |  |
| [endpoint::carrier_adapter::receipt_from_cached](../../src/carrier_adapter.rs#L368) | function_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler](../../src/carrier_adapter.rs#L391) | trait_item | `pub` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::open_stream](../../src/carrier_adapter.rs#L392) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::stream_error](../../src/carrier_adapter.rs#L397) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::mux_description](../../src/carrier_adapter.rs#L403) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::open_mux_stream](../../src/carrier_adapter.rs#L405) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::CarrierStreamHandler::journal_page](../../src/carrier_adapter.rs#L411) | function_signature_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost](../../src/carrier_adapter.rs#L420) | struct_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::new](../../src/carrier_adapter.rs#L430) | function_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::set_readiness](../../src/carrier_adapter.rs#L440) | function_item | `pub` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::readiness](../../src/carrier_adapter.rs#L452) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::registered_methods](../../src/carrier_adapter.rs#L460) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::advertised_extension_methods](../../src/carrier_adapter.rs#L476) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::classify_method](../../src/carrier_adapter.rs#L480) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::unary](../../src/carrier_adapter.rs#L493) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::respond](../../src/carrier_adapter.rs#L507) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_stream](../../src/carrier_adapter.rs#L515) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::stream_error](../../src/carrier_adapter.rs#L522) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_description](../../src/carrier_adapter.rs#L530) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::open_mux_stream](../../src/carrier_adapter.rs#L534) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::journal_page](../../src/carrier_adapter.rs#L542) | function_item | `private` |  |
| [endpoint::carrier_adapter::ComposedEndpointCarrierHost::mux_respond_actionable](../../src/carrier_adapter.rs#L549) | function_item | `private` |  |

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
| `respond` | `Box::pin` | [119](../../src/carrier_adapter.rs#L119) | external-constructor-callback-or-unresolved |
| `respond` | `response                 .validate()                 .map_err` | [120](../../src/carrier_adapter.rs#L120) | receiver-type-required |
| `respond` | `response                 .validate` | [120](../../src/carrier_adapter.rs#L120) | receiver-type-required |
| `respond` | `HostFailure::Protocol` | [122](../../src/carrier_adapter.rs#L122) | external-constructor-callback-or-unresolved |
| `respond` | `error.to_string` | [122](../../src/carrier_adapter.rs#L122), [157](../../src/carrier_adapter.rs#L157), [179](../../src/carrier_adapter.rs#L179), [183](../../src/carrier_adapter.rs#L183), [190](../../src/carrier_adapter.rs#L190), [203](../../src/carrier_adapter.rs#L203), [207](../../src/carrier_adapter.rs#L207), [221](../../src/carrier_adapter.rs#L221), [229](../../src/carrier_adapter.rs#L229), [238](../../src/carrier_adapter.rs#L238), [245](../../src/carrier_adapter.rs#L245) | receiver-type-required |
| `respond` | `self.authority.locate` | [123](../../src/carrier_adapter.rs#L123), [136](../../src/carrier_adapter.rs#L136) | receiver-type-required |
| `respond` | `Ok` | [124](../../src/carrier_adapter.rs#L124), [137](../../src/carrier_adapter.rs#L137), [191](../../src/carrier_adapter.rs#L191), [199](../../src/carrier_adapter.rs#L199), [210](../../src/carrier_adapter.rs#L210), [246](../../src/carrier_adapter.rs#L246) | external-constructor-callback-or-unresolved |
| `respond` | `Some` | [126](../../src/carrier_adapter.rs#L126), [139](../../src/carrier_adapter.rs#L139), [172](../../src/carrier_adapter.rs#L172), [178](../../src/carrier_adapter.rs#L178), [188](../../src/carrier_adapter.rs#L188), [201](../../src/carrier_adapter.rs#L201), [206](../../src/carrier_adapter.rs#L206), [228](../../src/carrier_adapter.rs#L228), [243](../../src/carrier_adapter.rs#L243) | external-constructor-callback-or-unresolved |
| `respond` | `"unknown-rpc-id".to_owned` | [126](../../src/carrier_adapter.rs#L126), [139](../../src/carrier_adapter.rs#L139) | receiver-type-required |
| `respond` | `Arc::clone` | [129](../../src/carrier_adapter.rs#L129) | external-constructor-callback-or-unresolved |
| `respond` | `admission                 .lock()                 .map_err` | [130](../../src/carrier_adapter.rs#L130) | receiver-type-required |
| `respond` | `admission                 .lock` | [130](../../src/carrier_adapter.rs#L130) | receiver-type-required |
| `respond` | `HostFailure::Internal` | [132](../../src/carrier_adapter.rs#L132), [143](../../src/carrier_adapter.rs#L143), [157](../../src/carrier_adapter.rs#L157), [179](../../src/carrier_adapter.rs#L179), [183](../../src/carrier_adapter.rs#L183), [190](../../src/carrier_adapter.rs#L190), [196](../../src/carrier_adapter.rs#L196), [203](../../src/carrier_adapter.rs#L203), [207](../../src/carrier_adapter.rs#L207), [216](../../src/carrier_adapter.rs#L216), [221](../../src/carrier_adapter.rs#L221), [229](../../src/carrier_adapter.rs#L229), [238](../../src/carrier_adapter.rs#L238), [245](../../src/carrier_adapter.rs#L245) | external-constructor-callback-or-unresolved |
| `respond` | `"respond admission gate poisoned".to_owned` | [132](../../src/carrier_adapter.rs#L132) | receiver-type-required |
| `respond` | `Arc::ptr_eq` | [142](../../src/carrier_adapter.rs#L142) | external-constructor-callback-or-unresolved |
| `respond` | `Err` | [143](../../src/carrier_adapter.rs#L143), [196](../../src/carrier_adapter.rs#L196), [203](../../src/carrier_adapter.rs#L203), [216](../../src/carrier_adapter.rs#L216), [221](../../src/carrier_adapter.rs#L221) | external-constructor-callback-or-unresolved |
| `respond` | `"respond admission authority changed during lookup".to_owned` | [144](../../src/carrier_adapter.rs#L144) | receiver-type-required |
| `respond` | `respond_claim` | [147](../../src/carrier_adapter.rs#L147) | [endpoint::carrier_adapter::respond_claim](../../src/carrier_adapter.rs#L343) |
| `respond` | `self.registry.lookup` | [150](../../src/carrier_adapter.rs#L150) | receiver-type-required |
| `respond` | `receipt_from_cached` | [151](../../src/carrier_adapter.rs#L151) | [endpoint::carrier_adapter::receipt_from_cached](../../src/carrier_adapter.rs#L368) |
| `respond` | `self                             .registry                             .handoff(&claim)                             .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?                             .map_or_else` | [154](../../src/carrier_adapter.rs#L154) | receiver-type-required |
| `respond` | `self                             .registry                             .handoff(&claim)                             .map_err` | [154](../../src/carrier_adapter.rs#L154) | receiver-type-required |
| `respond` | `self                             .registry                             .handoff` | [154](../../src/carrier_adapter.rs#L154) | receiver-type-required |
| `respond` | `respond_handoff` | [160](../../src/carrier_adapter.rs#L160), [226](../../src/carrier_adapter.rs#L226) | [endpoint::carrier_adapter::respond_handoff](../../src/carrier_adapter.rs#L328) |
| `respond` | `identity.unwrap_or_else` | [169](../../src/carrier_adapter.rs#L169) | receiver-type-required |
| `respond` | `"event".to_owned` | [170](../../src/carrier_adapter.rs#L170) | receiver-type-required |
| `respond` | `state.request.session_id.clone` | [171](../../src/carrier_adapter.rs#L171) | receiver-type-required |
| `respond` | `self.registry                             .mark_handed_off(&claim, &proof.0, Some(proof.1.clone()))                             .map_err` | [177](../../src/carrier_adapter.rs#L177) | receiver-type-required |
| `respond` | `self.registry                             .mark_handed_off` | [177](../../src/carrier_adapter.rs#L177) | receiver-type-required |
| `respond` | `proof.1.clone` | [178](../../src/carrier_adapter.rs#L178) | receiver-type-required |
| `respond` | `accepted_result` | [180](../../src/carrier_adapter.rs#L180), [237](../../src/carrier_adapter.rs#L237) | [endpoint::carrier_adapter::accepted_result](../../src/carrier_adapter.rs#L357) |
| `respond` | `self.registry                             .complete(&claim, &result)                             .map_err` | [181](../../src/carrier_adapter.rs#L181) | receiver-type-required |
| `respond` | `self.registry                             .complete` | [181](../../src/carrier_adapter.rs#L181) | receiver-type-required |
| `respond` | `context                             .handoff                             .mark_handed_off(crate::DurableHandoffProof {                                 delivery: proof.0,                                 durable_identity: Some(proof.1),                             })                             .map_err` | [184](../../src/carrier_adapter.rs#L184) | receiver-type-required |
| `respond` | `context                             .handoff                             .mark_handed_off` | [184](../../src/carrier_adapter.rs#L184) | receiver-type-required |
| `respond` | `"resolved endpoint request has no rpc carrier".to_owned` | [197](../../src/carrier_adapter.rs#L197) | receiver-type-required |
| `respond` | `"already-resolved".to_owned` | [201](../../src/carrier_adapter.rs#L201) | receiver-type-required |
| `respond` | `RespondLifecycle::prepare(Some(state), &response, &located.context)                 .map_err` | [206](../../src/carrier_adapter.rs#L206) | receiver-type-required |
| `respond` | `RespondLifecycle::prepare` | [206](../../src/carrier_adapter.rs#L206) | [endpoint::respond::RespondLifecycle::prepare](../../src/respond.rs#L68) |
| `respond` | `self.registry.begin` | [213](../../src/carrier_adapter.rs#L213) | receiver-type-required |
| `respond` | `"unresolved endpoint request has a completed rpc carrier"                                     .to_owned` | [217](../../src/carrier_adapter.rs#L217) | receiver-type-required |
| `respond` | `self.authority.author` | [223](../../src/carrier_adapter.rs#L223) | receiver-type-required |
| `respond` | `authorization.clone` | [223](../../src/carrier_adapter.rs#L223) | receiver-type-required |
| `respond` | `self.registry                         .mark_handed_off(&claim, &delivery, Some(durable_identity.clone()))                         .map_err` | [227](../../src/carrier_adapter.rs#L227) | receiver-type-required |
| `respond` | `self.registry                         .mark_handed_off` | [227](../../src/carrier_adapter.rs#L227) | receiver-type-required |
| `respond` | `durable_identity.clone` | [228](../../src/carrier_adapter.rs#L228) | receiver-type-required |
| `respond` | `self.registry                         .complete(&claim, &accepted_result()?)                         .map_err` | [236](../../src/carrier_adapter.rs#L236) | receiver-type-required |
| `respond` | `self.registry                         .complete` | [236](../../src/carrier_adapter.rs#L236) | receiver-type-required |
| `respond` | `context                         .handoff                         .mark_handed_off(crate::DurableHandoffProof {                             delivery,                             durable_identity: Some(durable_identity),                         })                         .map_err` | [239](../../src/carrier_adapter.rs#L239) | receiver-type-required |
| `respond` | `context                         .handoff                         .mark_handed_off` | [239](../../src/carrier_adapter.rs#L239) | receiver-type-required |
| `respond_mux` | `Box::pin` | [259](../../src/carrier_adapter.rs#L259) | external-constructor-callback-or-unresolved |
| `respond_mux` | `self.authority.locate` | [260](../../src/carrier_adapter.rs#L260) | receiver-type-required |
| `respond_mux` | `Err` | [261](../../src/carrier_adapter.rs#L261), [267](../../src/carrier_adapter.rs#L267), [272](../../src/carrier_adapter.rs#L272), [288](../../src/carrier_adapter.rs#L288), [317](../../src/carrier_adapter.rs#L317) | external-constructor-callback-or-unresolved |
| `respond_mux` | `HostFailure::Protocol` | [261](../../src/carrier_adapter.rs#L261), [267](../../src/carrier_adapter.rs#L267), [272](../../src/carrier_adapter.rs#L272), [280](../../src/carrier_adapter.rs#L280), [282](../../src/carrier_adapter.rs#L282), [284](../../src/carrier_adapter.rs#L284), [288](../../src/carrier_adapter.rs#L288), [310](../../src/carrier_adapter.rs#L310), [317](../../src/carrier_adapter.rs#L317) | external-constructor-callback-or-unresolved |
| `respond_mux` | `"actionable is no longer pending".to_owned` | [262](../../src/carrier_adapter.rs#L262) | receiver-type-required |
| `respond_mux` | `state.resolution.is_some` | [266](../../src/carrier_adapter.rs#L266) | receiver-type-required |
| `respond_mux` | `"actionable is already resolved".to_owned` | [268](../../src/carrier_adapter.rs#L268) | receiver-type-required |
| `respond_mux` | `serde_json::from_slice(                 &outcome                     .canonical_bytes()                     .map_err(&#124;error&#124; HostFailure::Protocol(error.to_string()))?,             )             .map_err` | [277](../../src/carrier_adapter.rs#L277) | receiver-type-required |
| `respond_mux` | `serde_json::from_slice` | [277](../../src/carrier_adapter.rs#L277) | external-constructor-callback-or-unresolved |
| `respond_mux` | `outcome                     .canonical_bytes()                     .map_err` | [278](../../src/carrier_adapter.rs#L278) | receiver-type-required |
| `respond_mux` | `outcome                     .canonical_bytes` | [278](../../src/carrier_adapter.rs#L278) | receiver-type-required |
| `respond_mux` | `error.to_string` | [280](../../src/carrier_adapter.rs#L280), [282](../../src/carrier_adapter.rs#L282), [308](../../src/carrier_adapter.rs#L308), [310](../../src/carrier_adapter.rs#L310) | receiver-type-required |
| `respond_mux` | `value.as_object_mut().ok_or_else` | [283](../../src/carrier_adapter.rs#L283) | receiver-type-required |
| `respond_mux` | `value.as_object_mut` | [283](../../src/carrier_adapter.rs#L283) | receiver-type-required |
| `respond_mux` | `"actionable outcome must be an object".to_owned` | [284](../../src/carrier_adapter.rs#L284) | receiver-type-required |
| `respond_mux` | `object.get("sessionId").and_then` | [286](../../src/carrier_adapter.rs#L286) | receiver-type-required |
| `respond_mux` | `object.get` | [286](../../src/carrier_adapter.rs#L286) | receiver-type-required |
| `respond_mux` | `"actionable outcome targets another session".to_owned` | [289](../../src/carrier_adapter.rs#L289) | receiver-type-required |
| `respond_mux` | `object.insert` | [294](../../src/carrier_adapter.rs#L294) | receiver-type-required |
| `respond_mux` | `"sessionId".to_owned` | [295](../../src/carrier_adapter.rs#L295) | receiver-type-required |
| `respond_mux` | `serde_json::Value::String` | [296](../../src/carrier_adapter.rs#L296) | external-constructor-callback-or-unresolved |
| `respond_mux` | `state.request.session_id.clone` | [296](../../src/carrier_adapter.rs#L296) | receiver-type-required |
| `respond_mux` | `"client-response".to_owned` | [301](../../src/carrier_adapter.rs#L301) | receiver-type-required |
| `respond_mux` | `Some` | [305](../../src/carrier_adapter.rs#L305) | external-constructor-callback-or-unresolved |
| `respond_mux` | `schema::IJsonValue::parse(                             &serde_json::to_vec(&value)                                 .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?,                         )                         .map_err` | [306](../../src/carrier_adapter.rs#L306) | receiver-type-required |
| `respond_mux` | `schema::IJsonValue::parse` | [306](../../src/carrier_adapter.rs#L306) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `respond_mux` | `serde_json::to_vec(&value)                                 .map_err` | [307](../../src/carrier_adapter.rs#L307) | receiver-type-required |
| `respond_mux` | `serde_json::to_vec` | [307](../../src/carrier_adapter.rs#L307) | external-constructor-callback-or-unresolved |
| `respond_mux` | `HostFailure::Internal` | [308](../../src/carrier_adapter.rs#L308) | external-constructor-callback-or-unresolved |
| `respond_mux` | `self.respond` | [315](../../src/carrier_adapter.rs#L315) | receiver-type-required |
| `respond_mux` | `receipt                         .reason                         .unwrap_or_else` | [318](../../src/carrier_adapter.rs#L318) | receiver-type-required |
| `respond_mux` | `"actionable response rejected".to_owned` | [320](../../src/carrier_adapter.rs#L320) | receiver-type-required |
| `respond_mux` | `Ok` | [323](../../src/carrier_adapter.rs#L323) | external-constructor-callback-or-unresolved |
| `respond_handoff` | `delivery.as_str().to_owned` | [334](../../src/carrier_adapter.rs#L334) | receiver-type-required |
| `respond_handoff` | `delivery.as_str` | [334](../../src/carrier_adapter.rs#L334) | receiver-type-required |
| `respond_handoff` | `"event".to_owned` | [336](../../src/carrier_adapter.rs#L336) | receiver-type-required |
| `respond_handoff` | `session_id.to_owned` | [337](../../src/carrier_adapter.rs#L337) | receiver-type-required |
| `respond_handoff` | `Some` | [338](../../src/carrier_adapter.rs#L338) | external-constructor-callback-or-unresolved |
| `respond_claim` | `schema::IJsonValue::parse(         &serde_json_canonicalizer::to_vec(response)             .map_err(&#124;error&#124; HostFailure::Internal(error.to_string()))?,     )     .map_err` | [344](../../src/carrier_adapter.rs#L344) | receiver-type-required |
| `respond_claim` | `schema::IJsonValue::parse` | [344](../../src/carrier_adapter.rs#L344) | [schema::ijson::IJsonValue::parse](../../../schema/src/ijson.rs#L16) |
| `respond_claim` | `serde_json_canonicalizer::to_vec(response)             .map_err` | [345](../../src/carrier_adapter.rs#L345) | receiver-type-required |
| `respond_claim` | `serde_json_canonicalizer::to_vec` | [345](../../src/carrier_adapter.rs#L345) | external-constructor-callback-or-unresolved |
| `respond_claim` | `HostFailure::Internal` | [346](../../src/carrier_adapter.rs#L346), [348](../../src/carrier_adapter.rs#L348) | external-constructor-callback-or-unresolved |
| `respond_claim` | `error.to_string` | [346](../../src/carrier_adapter.rs#L346), [348](../../src/carrier_adapter.rs#L348) | receiver-type-required |
| `respond_claim` | `Ok` | [349](../../src/carrier_adapter.rs#L349) | external-constructor-callback-or-unresolved |
| `respond_claim` | `"client-request".to_owned` | [350](../../src/carrier_adapter.rs#L350) | receiver-type-required |
| `respond_claim` | `response.rpc_id.clone` | [351](../../src/carrier_adapter.rs#L351) | receiver-type-required |
| `respond_claim` | `"respond".to_owned` | [352](../../src/carrier_adapter.rs#L352) | receiver-type-required |
| `accepted_result` | `Ok` | [358](../../src/carrier_adapter.rs#L358) | external-constructor-callback-or-unresolved |
| `accepted_result` | `Some` | [360](../../src/carrier_adapter.rs#L360) | external-constructor-callback-or-unresolved |
| `accepted_result` | `schema::IJsonValue::parse_str(r#"{"accepted":true}"#)                 .map_err` | [361](../../src/carrier_adapter.rs#L361) | receiver-type-required |
| `accepted_result` | `schema::IJsonValue::parse_str` | [361](../../src/carrier_adapter.rs#L361) | [schema::ijson::IJsonValue::parse_str](../../../schema/src/ijson.rs#L23) |
| `accepted_result` | `HostFailure::Internal` | [362](../../src/carrier_adapter.rs#L362) | external-constructor-callback-or-unresolved |
| `accepted_result` | `error.to_string` | [362](../../src/carrier_adapter.rs#L362) | receiver-type-required |
| `receipt_from_cached` | `result.error.is_some` | [369](../../src/carrier_adapter.rs#L369) | receiver-type-required |
| `receipt_from_cached` | `Err` | [370](../../src/carrier_adapter.rs#L370), [385](../../src/carrier_adapter.rs#L385) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `HostFailure::Internal` | [370](../../src/carrier_adapter.rs#L370), [375](../../src/carrier_adapter.rs#L375), [379](../../src/carrier_adapter.rs#L379), [381](../../src/carrier_adapter.rs#L381), [385](../../src/carrier_adapter.rs#L385) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `"respond rpc carrier cached a non-success result".to_owned` | [371](../../src/carrier_adapter.rs#L371) | receiver-type-required |
| `receipt_from_cached` | `result.value.ok_or_else` | [374](../../src/carrier_adapter.rs#L374) | receiver-type-required |
| `receipt_from_cached` | `"respond rpc carrier omitted its success value".to_owned` | [375](../../src/carrier_adapter.rs#L375) | receiver-type-required |
| `receipt_from_cached` | `value         .canonical_bytes()         .map_err` | [377](../../src/carrier_adapter.rs#L377) | receiver-type-required |
| `receipt_from_cached` | `value         .canonical_bytes` | [377](../../src/carrier_adapter.rs#L377) | receiver-type-required |
| `receipt_from_cached` | `error.to_string` | [379](../../src/carrier_adapter.rs#L379), [381](../../src/carrier_adapter.rs#L381) | receiver-type-required |
| `receipt_from_cached` | `serde_json::from_slice(&bytes).map_err` | [381](../../src/carrier_adapter.rs#L381) | receiver-type-required |
| `receipt_from_cached` | `serde_json::from_slice` | [381](../../src/carrier_adapter.rs#L381) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `receipt.reason.is_none` | [382](../../src/carrier_adapter.rs#L382) | receiver-type-required |
| `receipt_from_cached` | `Ok` | [383](../../src/carrier_adapter.rs#L383) | external-constructor-callback-or-unresolved |
| `receipt_from_cached` | `"respond rpc carrier cached invalid receipt bytes".to_owned` | [386](../../src/carrier_adapter.rs#L386) | receiver-type-required |
| `new` | `EndpointDispatcher::new` | [433](../../src/carrier_adapter.rs#L433) | [endpoint::host::EndpointDispatcher::new](../../src/host.rs#L451) |
| `new` | `AtomicBool::new` | [436](../../src/carrier_adapter.rs#L436) | external-constructor-callback-or-unresolved |
| `set_readiness` | `self.ready             .store` | [441](../../src/carrier_adapter.rs#L441) | receiver-type-required |
| `readiness` | `self.ready.load` | [453](../../src/carrier_adapter.rs#L453) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter()             .copied()             .collect::<BTreeSet<_>>` | [461](../../src/carrier_adapter.rs#L461) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter()             .copied` | [461](../../src/carrier_adapter.rs#L461) | receiver-type-required |
| `registered_methods` | `SESSION_ENDPOINT_PUBLIC_METHODS             .iter` | [461](../../src/carrier_adapter.rs#L461) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter()             .filter(&#124;method&#124; public.contains(method.as_str()))             .collect::<BTreeSet<_>>` | [465](../../src/carrier_adapter.rs#L465) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter()             .filter` | [465](../../src/carrier_adapter.rs#L465) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities()             .into_iter` | [465](../../src/carrier_adapter.rs#L465) | receiver-type-required |
| `registered_methods` | `self             .unary             .capabilities` | [465](../../src/carrier_adapter.rs#L465) | receiver-type-required |
| `registered_methods` | `public.contains` | [469](../../src/carrier_adapter.rs#L469) | receiver-type-required |
| `registered_methods` | `method.as_str` | [469](../../src/carrier_adapter.rs#L469) | receiver-type-required |
| `registered_methods` | `methods.insert` | [471](../../src/carrier_adapter.rs#L471) | receiver-type-required |
| `registered_methods` | `"remote.mux".to_owned` | [471](../../src/carrier_adapter.rs#L471) | receiver-type-required |
| `registered_methods` | `methods.extend` | [472](../../src/carrier_adapter.rs#L472) | receiver-type-required |
| `registered_methods` | `self.unary.extension_capabilities` | [472](../../src/carrier_adapter.rs#L472) | receiver-type-required |
| `advertised_extension_methods` | `self.unary.extension_capabilities` | [477](../../src/carrier_adapter.rs#L477) | receiver-type-required |
| `classify_method` | `SESSION_ENDPOINT_PUBLIC_METHODS.contains` | [483](../../src/carrier_adapter.rs#L483) | receiver-type-required |
| `classify_method` | `self.unary.capabilities().contains` | [484](../../src/carrier_adapter.rs#L484) | receiver-type-required |
| `classify_method` | `self.unary.capabilities` | [484](../../src/carrier_adapter.rs#L484) | receiver-type-required |
| `classify_method` | `self.unary.extension_capabilities().contains` | [485](../../src/carrier_adapter.rs#L485) | receiver-type-required |
| `classify_method` | `self.unary.extension_capabilities` | [485](../../src/carrier_adapter.rs#L485) | receiver-type-required |
| `classify_method` | `self.unary.method_class` | [487](../../src/carrier_adapter.rs#L487) | receiver-type-required |
| `unary` | `Box::pin` | [498](../../src/carrier_adapter.rs#L498) | external-constructor-callback-or-unresolved |
| `unary` | `self.unary.validate_request` | [499](../../src/carrier_adapter.rs#L499) | receiver-type-required |
| `unary` | `self.dispatcher                 .dispatch_with_handoff(&self.unary, request, context.handoff)                 .await                 .map_err` | [500](../../src/carrier_adapter.rs#L500) | receiver-type-required |
| `unary` | `self.dispatcher                 .dispatch_with_handoff` | [500](../../src/carrier_adapter.rs#L500) | receiver-type-required |
| `unary` | `HostFailure::Internal` | [503](../../src/carrier_adapter.rs#L503) | external-constructor-callback-or-unresolved |
| `unary` | `error.to_string` | [503](../../src/carrier_adapter.rs#L503) | receiver-type-required |
| `respond` | `self.respond.respond` | [512](../../src/carrier_adapter.rs#L512) | receiver-type-required |
| `open_stream` | `self.streams.open_stream` | [519](../../src/carrier_adapter.rs#L519) | receiver-type-required |
| `stream_error` | `self.streams.stream_error` | [527](../../src/carrier_adapter.rs#L527) | receiver-type-required |
| `mux_description` | `self.streams.mux_description` | [531](../../src/carrier_adapter.rs#L531) | receiver-type-required |
| `open_mux_stream` | `self.streams.open_mux_stream` | [539](../../src/carrier_adapter.rs#L539) | receiver-type-required |
| `journal_page` | `self.streams.journal_page` | [546](../../src/carrier_adapter.rs#L546) | receiver-type-required |
| `mux_respond_actionable` | `Box::pin` | [561](../../src/carrier_adapter.rs#L561) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `std::future::ready` | [561](../../src/carrier_adapter.rs#L561) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `Err` | [561](../../src/carrier_adapter.rs#L561) | external-constructor-callback-or-unresolved |
| `mux_respond_actionable` | `self.respond             .respond_mux` | [563](../../src/carrier_adapter.rs#L563) | receiver-type-required |
