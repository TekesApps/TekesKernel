# endpoint::all_session

[Package atlas](index.md) · [Source](../../src/all_session.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [endpoint::all_session::AllSessionMux](../../src/all_session.rs#L19) | struct_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::open](../../src/all_session.rs#L27) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::open_with_replay](../../src/all_session.rs#L39) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::attach](../../src/all_session.rs#L51) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::attach_with_replay](../../src/all_session.rs#L85) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::detach_for_archive](../../src/all_session.rs#L120) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::poll](../../src/all_session.rs#L134) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::baselines](../../src/all_session.rs#L143) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::contains](../../src/all_session.rs#L151) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::len](../../src/all_session.rs#L156) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::is_empty](../../src/all_session.rs#L161) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::into_stream](../../src/all_session.rs#L170) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMux::wake_stream](../../src/all_session.rs#L178) | function_item | `private` |  |
| [endpoint::all_session::AllSessionMuxHandle](../../src/all_session.rs#L186) | struct_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::contains](../../src/all_session.rs#L189) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::attach](../../src/all_session.rs#L197) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::attach_with_replay](../../src/all_session.rs#L210) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::detach_for_archive](../../src/all_session.rs#L224) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::baselines](../../src/all_session.rs#L231) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxHandle::close](../../src/all_session.rs#L239) | function_item | `pub` |  |
| [endpoint::all_session::AllSessionMuxStream](../../src/all_session.rs#L253) | struct_item | `private` |  |
| [endpoint::all_session::AllSessionMuxStream::recv](../../src/all_session.rs#L259) | function_item | `private` |  |
| [endpoint::all_session::AllSessionMuxError](../../src/all_session.rs#L348) | enum_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `BTreeMap` | `std::collections::BTreeMap` | `private` |
| `poll_fn` | `std::future::poll_fn` | `private` |
| `Arc` | `std::sync::Arc` | `private` |
| `Mutex` | `std::sync::Mutex` | `private` |
| `Poll` | `std::task::Poll` | `private` |
| `Waker` | `std::task::Waker` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `CarrierHostFuture` | `crate::CarrierHostFuture` | `private` |
| `EndpointJournal` | `crate::EndpointJournal` | `private` |
| `EndpointStream` | `crate::EndpointStream` | `private` |
| `EndpointStreamReceiver` | `crate::EndpointStreamReceiver` | `private` |
| `EndpointSubscription` | `crate::EndpointSubscription` | `private` |
| `EndpointSubscriptionHub` | `crate::EndpointSubscriptionHub` | `private` |
| `HubError` | `crate::HubError` | `private` |
| `MuxRegistration` | `crate::MuxRegistration` | `private` |
| `MuxReplayRegistration` | `crate::MuxReplayRegistration` | `private` |
| `ServerRequest` | `crate::ServerRequest` | `private` |
| `StreamErrorCode` | `crate::StreamErrorCode` | `private` |
| `StreamFailure` | `crate::StreamFailure` | `private` |
| `SubscriptionBaseline` | `crate::SubscriptionBaseline` | `private` |
| `SubscriptionPoll` | `crate::SubscriptionPoll` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–19: 5 direct edges</summary>

```mermaid
flowchart TD
  n0["endpoint::all_session::AllSessionMux::detach_for_archive"]
  n1["endpoint::all_session::AllSessionMux::poll"]
  n2["endpoint::all_session::AllSessionMux::baselines"]
  n3["endpoint::all_session::AllSessionMux::contains"]
  n4["endpoint::all_session::AllSessionMux::len"]
  n5["endpoint::all_session::AllSessionMux::is_empty"]
  n6["endpoint::all_session::AllSessionMux::into_stream"]
  n7["endpoint::all_session::AllSessionMux::wake_stream"]
  n8["endpoint::all_session::AllSessionMuxHandle::contains"]
  n9["endpoint::all_session::AllSessionMuxHandle::attach"]
  n10["endpoint::all_session::AllSessionMuxHandle::attach_with_replay"]
  n11["endpoint::all_session::AllSessionMuxHandle::detach_for_archive"]
  n12["endpoint::all_session::AllSessionMuxHandle::baselines"]
  n13["endpoint::all_session::AllSessionMuxHandle::close"]
  n14["endpoint::all_session::AllSessionMuxStream::recv"]
  n15["endpoint::all_session::AllSessionMux::open"]
  n16["endpoint::all_session::AllSessionMux::open_with_replay"]
  n17["endpoint::all_session::AllSessionMux::attach"]
  n18["endpoint::all_session::AllSessionMux::attach_with_replay"]
  n19["endpoint::host::StreamFailure::new"]
  n20["endpoint::host::StreamFailure::internal"]
  n0 --> n7
  n14 --> n19
  n14 --> n20
  n17 --> n7
  n18 --> n7
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `open` | `Ok` | [31](../../src/all_session.rs#L31) | external-constructor-callback-or-unresolved |
| `open` | `hub.subscribe_all` | [32](../../src/all_session.rs#L32) | receiver-type-required |
| `open` | `Vec::new` | [33](../../src/all_session.rs#L33) | external-constructor-callback-or-unresolved |
| `open_with_replay` | `Ok` | [43](../../src/all_session.rs#L43) | external-constructor-callback-or-unresolved |
| `open_with_replay` | `hub.subscribe_all_with_replay` | [44](../../src/all_session.rs#L44) | receiver-type-required |
| `open_with_replay` | `Vec::new` | [45](../../src/all_session.rs#L45) | external-constructor-callback-or-unresolved |
| `attach` | `self.subscriptions.contains_key` | [58](../../src/all_session.rs#L58) | receiver-type-required |
| `attach` | `Err` | [59](../../src/all_session.rs#L59), [62](../../src/all_session.rs#L62) | external-constructor-callback-or-unresolved |
| `attach` | `AllSessionMuxError::AlreadyAttached` | [59](../../src/all_session.rs#L59) | external-constructor-callback-or-unresolved |
| `attach` | `session_id.to_owned` | [59](../../src/all_session.rs#L59), [71](../../src/all_session.rs#L71), [74](../../src/all_session.rs#L74) | receiver-type-required |
| `attach` | `hub.subscribe_all` | [64](../../src/all_session.rs#L64) | receiver-type-required |
| `attach` | `registered             .remove(session_id)             .ok_or_else` | [69](../../src/all_session.rs#L69) | receiver-type-required |
| `attach` | `registered             .remove` | [69](../../src/all_session.rs#L69) | receiver-type-required |
| `attach` | `AllSessionMuxError::MissingRegistration` | [71](../../src/all_session.rs#L71) | external-constructor-callback-or-unresolved |
| `attach` | `subscription.baseline().clone` | [72](../../src/all_session.rs#L72) | receiver-type-required |
| `attach` | `subscription.baseline` | [72](../../src/all_session.rs#L72) | receiver-type-required |
| `attach` | `self.subscriptions             .insert` | [73](../../src/all_session.rs#L73) | receiver-type-required |
| `attach` | `self.subscriptions.values` | [78](../../src/all_session.rs#L78) | receiver-type-required |
| `attach` | `subscription.wake` | [79](../../src/all_session.rs#L79) | receiver-type-required |
| `attach` | `self.wake_stream` | [81](../../src/all_session.rs#L81) | [endpoint::all_session::AllSessionMux::wake_stream](../../src/all_session.rs#L178) |
| `attach` | `Ok` | [82](../../src/all_session.rs#L82) | external-constructor-callback-or-unresolved |
| `attach_with_replay` | `self.subscriptions.contains_key` | [93](../../src/all_session.rs#L93) | receiver-type-required |
| `attach_with_replay` | `Err` | [94](../../src/all_session.rs#L94), [97](../../src/all_session.rs#L97) | external-constructor-callback-or-unresolved |
| `attach_with_replay` | `AllSessionMuxError::AlreadyAttached` | [94](../../src/all_session.rs#L94) | external-constructor-callback-or-unresolved |
| `attach_with_replay` | `session_id.to_owned` | [94](../../src/all_session.rs#L94), [109](../../src/all_session.rs#L109), [112](../../src/all_session.rs#L112) | receiver-type-required |
| `attach_with_replay` | `hub.subscribe_all_with_replay` | [99](../../src/all_session.rs#L99) | receiver-type-required |
| `attach_with_replay` | `registered             .remove(session_id)             .ok_or_else` | [107](../../src/all_session.rs#L107) | receiver-type-required |
| `attach_with_replay` | `registered             .remove` | [107](../../src/all_session.rs#L107) | receiver-type-required |
| `attach_with_replay` | `AllSessionMuxError::MissingRegistration` | [109](../../src/all_session.rs#L109) | external-constructor-callback-or-unresolved |
| `attach_with_replay` | `subscription.baseline().clone` | [110](../../src/all_session.rs#L110) | receiver-type-required |
| `attach_with_replay` | `subscription.baseline` | [110](../../src/all_session.rs#L110) | receiver-type-required |
| `attach_with_replay` | `self.subscriptions             .insert` | [111](../../src/all_session.rs#L111) | receiver-type-required |
| `attach_with_replay` | `self.subscriptions.values` | [113](../../src/all_session.rs#L113) | receiver-type-required |
| `attach_with_replay` | `subscription.wake` | [114](../../src/all_session.rs#L114) | receiver-type-required |
| `attach_with_replay` | `self.wake_stream` | [116](../../src/all_session.rs#L116) | [endpoint::all_session::AllSessionMux::wake_stream](../../src/all_session.rs#L178) |
| `attach_with_replay` | `Ok` | [117](../../src/all_session.rs#L117) | external-constructor-callback-or-unresolved |
| `detach_for_archive` | `self.subscriptions.remove` | [121](../../src/all_session.rs#L121) | receiver-type-required |
| `detach_for_archive` | `Ok` | [122](../../src/all_session.rs#L122), [131](../../src/all_session.rs#L131) | external-constructor-callback-or-unresolved |
| `detach_for_archive` | `subscription.close` | [124](../../src/all_session.rs#L124) | receiver-type-required |
| `detach_for_archive` | `self.retired.push` | [129](../../src/all_session.rs#L129) | receiver-type-required |
| `detach_for_archive` | `self.wake_stream` | [130](../../src/all_session.rs#L130) | [endpoint::all_session::AllSessionMux::wake_stream](../../src/all_session.rs#L178) |
| `poll` | `self.subscriptions             .get(session_id)             .ok_or_else(&#124;&#124; AllSessionMuxError::NotAttached(session_id.to_owned()))?             .poll()             .map_err` | [135](../../src/all_session.rs#L135) | receiver-type-required |
| `poll` | `self.subscriptions             .get(session_id)             .ok_or_else(&#124;&#124; AllSessionMuxError::NotAttached(session_id.to_owned()))?             .poll` | [135](../../src/all_session.rs#L135) | receiver-type-required |
| `poll` | `self.subscriptions             .get(session_id)             .ok_or_else` | [135](../../src/all_session.rs#L135) | receiver-type-required |
| `poll` | `self.subscriptions             .get` | [135](../../src/all_session.rs#L135) | receiver-type-required |
| `poll` | `AllSessionMuxError::NotAttached` | [137](../../src/all_session.rs#L137) | external-constructor-callback-or-unresolved |
| `poll` | `session_id.to_owned` | [137](../../src/all_session.rs#L137) | receiver-type-required |
| `baselines` | `self.subscriptions             .values()             .map(&#124;subscription&#124; subscription.baseline().clone())             .collect` | [144](../../src/all_session.rs#L144) | receiver-type-required |
| `baselines` | `self.subscriptions             .values()             .map` | [144](../../src/all_session.rs#L144) | receiver-type-required |
| `baselines` | `self.subscriptions             .values` | [144](../../src/all_session.rs#L144) | receiver-type-required |
| `baselines` | `subscription.baseline().clone` | [146](../../src/all_session.rs#L146) | receiver-type-required |
| `baselines` | `subscription.baseline` | [146](../../src/all_session.rs#L146) | receiver-type-required |
| `contains` | `self.subscriptions.contains_key` | [152](../../src/all_session.rs#L152) | receiver-type-required |
| `len` | `self.subscriptions.len` | [157](../../src/all_session.rs#L157) | receiver-type-required |
| `is_empty` | `self.subscriptions.is_empty` | [162](../../src/all_session.rs#L162) | receiver-type-required |
| `into_stream` | `Arc::new` | [171](../../src/all_session.rs#L171) | external-constructor-callback-or-unresolved |
| `into_stream` | `Mutex::new` | [171](../../src/all_session.rs#L171) | external-constructor-callback-or-unresolved |
| `into_stream` | `AllSessionMuxHandle` | [173](../../src/all_session.rs#L173) | external-constructor-callback-or-unresolved |
| `into_stream` | `Arc::clone` | [173](../../src/all_session.rs#L173) | external-constructor-callback-or-unresolved |
| `into_stream` | `Box::new` | [174](../../src/all_session.rs#L174) | external-constructor-callback-or-unresolved |
| `wake_stream` | `self.stream_waker.take` | [179](../../src/all_session.rs#L179) | receiver-type-required |
| `wake_stream` | `waker.wake` | [180](../../src/all_session.rs#L180) | receiver-type-required |
| `contains` | `Ok` | [190](../../src/all_session.rs#L190) | external-constructor-callback-or-unresolved |
| `contains` | `self             .0             .lock()             .map_err(&#124;_&#124; AllSessionMuxError::Poisoned)?             .contains` | [190](../../src/all_session.rs#L190) | receiver-type-required |
| `contains` | `self             .0             .lock()             .map_err` | [190](../../src/all_session.rs#L190) | receiver-type-required |
| `contains` | `self             .0             .lock` | [190](../../src/all_session.rs#L190) | receiver-type-required |
| `attach` | `self.0             .lock()             .map_err(&#124;_&#124; AllSessionMuxError::Poisoned)?             .attach` | [204](../../src/all_session.rs#L204) | receiver-type-required |
| `attach` | `self.0             .lock()             .map_err` | [204](../../src/all_session.rs#L204) | receiver-type-required |
| `attach` | `self.0             .lock` | [204](../../src/all_session.rs#L204) | receiver-type-required |
| `attach_with_replay` | `self.0             .lock()             .map_err(&#124;_&#124; AllSessionMuxError::Poisoned)?             .attach_with_replay` | [218](../../src/all_session.rs#L218) | receiver-type-required |
| `attach_with_replay` | `self.0             .lock()             .map_err` | [218](../../src/all_session.rs#L218) | receiver-type-required |
| `attach_with_replay` | `self.0             .lock` | [218](../../src/all_session.rs#L218) | receiver-type-required |
| `detach_for_archive` | `self.0             .lock()             .map_err(&#124;_&#124; AllSessionMuxError::Poisoned)?             .detach_for_archive` | [225](../../src/all_session.rs#L225) | receiver-type-required |
| `detach_for_archive` | `self.0             .lock()             .map_err` | [225](../../src/all_session.rs#L225) | receiver-type-required |
| `detach_for_archive` | `self.0             .lock` | [225](../../src/all_session.rs#L225) | receiver-type-required |
| `baselines` | `Ok` | [232](../../src/all_session.rs#L232) | external-constructor-callback-or-unresolved |
| `baselines` | `self             .0             .lock()             .map_err(&#124;_&#124; AllSessionMuxError::Poisoned)?             .baselines` | [232](../../src/all_session.rs#L232) | receiver-type-required |
| `baselines` | `self             .0             .lock()             .map_err` | [232](../../src/all_session.rs#L232) | receiver-type-required |
| `baselines` | `self             .0             .lock` | [232](../../src/all_session.rs#L232) | receiver-type-required |
| `close` | `self.0.lock().map_err` | [240](../../src/all_session.rs#L240) | receiver-type-required |
| `close` | `self.0.lock` | [240](../../src/all_session.rs#L240) | receiver-type-required |
| `close` | `mux.subscriptions.values` | [242](../../src/all_session.rs#L242) | receiver-type-required |
| `close` | `subscription.close` | [243](../../src/all_session.rs#L243), [246](../../src/all_session.rs#L246) | receiver-type-required |
| `close` | `mux.wake_stream` | [248](../../src/all_session.rs#L248) | receiver-type-required |
| `close` | `Ok` | [249](../../src/all_session.rs#L249) | external-constructor-callback-or-unresolved |
| `recv` | `Box::pin` | [262](../../src/all_session.rs#L262) | external-constructor-callback-or-unresolved |
| `recv` | `poll_fn` | [262](../../src/all_session.rs#L262) | external-constructor-callback-or-unresolved |
| `recv` | `self.mux.lock` | [263](../../src/all_session.rs#L263) | receiver-type-required |
| `recv` | `Poll::Ready` | [266](../../src/all_session.rs#L266), [274](../../src/all_session.rs#L274), [277](../../src/all_session.rs#L277), [288](../../src/all_session.rs#L288), [294](../../src/all_session.rs#L294), [316](../../src/all_session.rs#L316), [320](../../src/all_session.rs#L320), [327](../../src/all_session.rs#L327), [332](../../src/all_session.rs#L332) | external-constructor-callback-or-unresolved |
| `recv` | `Some` | [266](../../src/all_session.rs#L266), [274](../../src/all_session.rs#L274), [277](../../src/all_session.rs#L277), [288](../../src/all_session.rs#L288), [301](../../src/all_session.rs#L301), [316](../../src/all_session.rs#L316), [320](../../src/all_session.rs#L320), [327](../../src/all_session.rs#L327), [339](../../src/all_session.rs#L339) | external-constructor-callback-or-unresolved |
| `recv` | `Err` | [266](../../src/all_session.rs#L266), [277](../../src/all_session.rs#L277), [288](../../src/all_session.rs#L288), [320](../../src/all_session.rs#L320), [327](../../src/all_session.rs#L327) | external-constructor-callback-or-unresolved |
| `recv` | `StreamFailure::internal` | [266](../../src/all_session.rs#L266), [288](../../src/all_session.rs#L288), [327](../../src/all_session.rs#L327) | [endpoint::host::StreamFailure::internal](../../src/host.rs#L304) |
| `recv` | `"all-session mux mutex was poisoned".to_owned` | [267](../../src/all_session.rs#L267) | receiver-type-required |
| `recv` | `mux.retired.first` | [271](../../src/all_session.rs#L271) | receiver-type-required |
| `recv` | `subscription.poll_with_context` | [272](../../src/all_session.rs#L272), [313](../../src/all_session.rs#L313) | receiver-type-required |
| `recv` | `Ok` | [274](../../src/all_session.rs#L274), [316](../../src/all_session.rs#L316) | external-constructor-callback-or-unresolved |
| `recv` | `StreamFailure::new` | [277](../../src/all_session.rs#L277), [320](../../src/all_session.rs#L320) | [endpoint::host::StreamFailure::new](../../src/host.rs#L296) |
| `recv` | `mux.retired.remove` | [282](../../src/all_session.rs#L282) | receiver-type-required |
| `recv` | `error.to_string` | [288](../../src/all_session.rs#L288), [327](../../src/all_session.rs#L327) | receiver-type-required |
| `recv` | `mux.subscriptions.is_empty` | [292](../../src/all_session.rs#L292) | receiver-type-required |
| `recv` | `mux                     .stream_waker                     .as_ref()                     .is_none_or` | [296](../../src/all_session.rs#L296), [334](../../src/all_session.rs#L334) | receiver-type-required |
| `recv` | `mux                     .stream_waker                     .as_ref` | [296](../../src/all_session.rs#L296), [334](../../src/all_session.rs#L334) | receiver-type-required |
| `recv` | `waker.will_wake` | [299](../../src/all_session.rs#L299), [337](../../src/all_session.rs#L337) | receiver-type-required |
| `recv` | `context.waker` | [299](../../src/all_session.rs#L299), [301](../../src/all_session.rs#L301), [337](../../src/all_session.rs#L337), [339](../../src/all_session.rs#L339) | receiver-type-required |
| `recv` | `context.waker().clone` | [301](../../src/all_session.rs#L301), [339](../../src/all_session.rs#L339) | receiver-type-required |
| `recv` | `mux.subscriptions.len` | [306](../../src/all_session.rs#L306) | receiver-type-required |
| `recv` | `mux.subscriptions.values().nth` | [310](../../src/all_session.rs#L310) | receiver-type-required |
| `recv` | `mux.subscriptions.values` | [310](../../src/all_session.rs#L310) | receiver-type-required |
