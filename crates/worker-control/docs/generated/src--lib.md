# worker-control

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [worker-control::PROTOCOL_NAME](../../src/lib.rs#L12) | const_item | `pub` |  |
| [worker-control::PROTOCOL_VERSION](../../src/lib.rs#L13) | const_item | `pub` |  |
| [worker-control::EX_PROTOCOL](../../src/lib.rs#L14) | const_item | `pub` |  |
| [worker-control::Hello](../../src/lib.rs#L17) | struct_item | `pub` |  |
| [worker-control::Hello::default](../../src/lib.rs#L24) | function_item | `private` |  |
| [worker-control::Selected](../../src/lib.rs#L34) | struct_item | `pub` |  |
| [worker-control::Reject](../../src/lib.rs#L39) | struct_item | `pub` |  |
| [worker-control::AssetRef](../../src/lib.rs#L45) | struct_item | `pub` |  |
| [worker-control::Input](../../src/lib.rs#L51) | struct_item | `pub` |  |
| [worker-control::ApprovalResponse](../../src/lib.rs#L64) | struct_item | `pub` |  |
| [worker-control::QueueEdit](../../src/lib.rs#L74) | struct_item | `pub` |  |
| [worker-control::Stop](../../src/lib.rs#L81) | struct_item | `pub` |  |
| [worker-control::Lease](../../src/lib.rs#L88) | struct_item | `pub` |  |
| [worker-control::Ping](../../src/lib.rs#L94) | struct_item | `pub` |  |
| [worker-control::Compact](../../src/lib.rs#L99) | struct_item | `pub` |  |
| [worker-control::Meta](../../src/lib.rs#L105) | struct_item | `pub` |  |
| [worker-control::LaunchResult](../../src/lib.rs#L115) | struct_item | `pub` |  |
| [worker-control::SupervisorMessage](../../src/lib.rs#L124) | enum_item | `pub` |  |
| [worker-control::Receipt](../../src/lib.rs#L139) | struct_item | `pub` |  |
| [worker-control::AttemptSettled](../../src/lib.rs#L148) | struct_item | `pub` |  |
| [worker-control::LeaseRequest](../../src/lib.rs#L154) | struct_item | `pub` |  |
| [worker-control::Appended](../../src/lib.rs#L160) | struct_item | `pub` |  |
| [worker-control::FrameChannel](../../src/lib.rs#L166) | enum_item | `pub` |  |
| [worker-control::Frame](../../src/lib.rs#L174) | struct_item | `pub` |  |
| [worker-control::Frame::validate](../../src/lib.rs#L195) | function_item | `pub` |  |
| [worker-control::State](../../src/lib.rs#L225) | struct_item | `pub` |  |
| [worker-control::Pong](../../src/lib.rs#L230) | struct_item | `pub` |  |
| [worker-control::LaunchChild](../../src/lib.rs#L235) | struct_item | `pub` |  |
| [worker-control::WorkerMessage](../../src/lib.rs#L242) | enum_item | `pub` |  |
| [worker-control::ProtocolError](../../src/lib.rs#L256) | enum_item | `pub` |  |
| [worker-control::negotiate](../../src/lib.rs#L278) | function_item | `pub` |  |
| [worker-control::decode_hello](../../src/lib.rs#L294) | function_item | `pub` |  |
| [worker-control::decode_selected](../../src/lib.rs#L298) | function_item | `pub` |  |
| [worker-control::decode_reject](../../src/lib.rs#L302) | function_item | `pub` |  |
| [worker-control::decode_supervisor](../../src/lib.rs#L306) | function_item | `pub` |  |
| [worker-control::decode_worker](../../src/lib.rs#L332) | function_item | `pub` |  |
| [worker-control::encode_line](../../src/lib.rs#L361) | function_item | `pub` |  |
| [worker-control::decode_named](../../src/lib.rs#L369) | function_item | `private` |  |
| [worker-control::split_line](../../src/lib.rs#L380) | function_item | `private` |  |
| [worker-control::from_payload](../../src/lib.rs#L393) | function_item | `private` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `Block` | `schema::Block` | `private` |
| `IJsonValue` | `schema::IJsonValue` | `private` |
| `OriginTuple` | `schema::OriginTuple` | `private` |
| `ResumePolicy` | `schema::ResumePolicy` | `private` |
| `SeqRange` | `schema::SeqRange` | `private` |
| `DeserializeOwned` | `serde::de::DeserializeOwned` | `private` |
| `Deserialize` | `serde::Deserialize` | `private` |
| `Serialize` | `serde::Serialize` | `private` |
| `Map` | `serde_json::Map` | `private` |
| `Value` | `serde_json::Value` | `private` |
| `Error` | `thiserror::Error` | `private` |
| `*` | `durable::*` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `worker-control::continuation` | `pub` |  |
| `worker-control::durable` | `pub` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

<details><summary>Functions 1–12: 9 direct edges</summary>

```mermaid
flowchart TD
  n0["worker-control::Frame::validate"]
  n1["worker-control::Hello::default"]
  n2["worker-control::negotiate"]
  n3["worker-control::decode_hello"]
  n4["worker-control::decode_selected"]
  n5["worker-control::decode_reject"]
  n6["worker-control::decode_supervisor"]
  n7["worker-control::decode_worker"]
  n8["worker-control::encode_line"]
  n9["worker-control::decode_named"]
  n10["worker-control::split_line"]
  n11["worker-control::from_payload"]
  n3 --> n9
  n4 --> n9
  n5 --> n9
  n6 --> n10
  n6 --> n11
  n7 --> n10
  n7 --> n11
  n9 --> n10
  n9 --> n11
```

</details>

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `default` | `PROTOCOL_NAME.to_owned` | [26](../../src/lib.rs#L26) | receiver-type-required |
| `validate` | `self                     .call_id                     .as_deref()                     .is_some_and` | [198](../../src/lib.rs#L198) | receiver-type-required |
| `validate` | `self                     .call_id                     .as_deref` | [198](../../src/lib.rs#L198) | receiver-type-required |
| `validate` | `value.is_empty` | [201](../../src/lib.rs#L201) | receiver-type-required |
| `validate` | `Ok` | [203](../../src/lib.rs#L203), [213](../../src/lib.rs#L213) | external-constructor-callback-or-unresolved |
| `validate` | `Err` | [205](../../src/lib.rs#L205), [216](../../src/lib.rs#L216) | external-constructor-callback-or-unresolved |
| `validate` | `ProtocolError::InvalidFrame` | [205](../../src/lib.rs#L205), [216](../../src/lib.rs#L216) | external-constructor-callback-or-unresolved |
| `validate` | `"tool frame requires nonempty call_id".to_owned` | [206](../../src/lib.rs#L206) | receiver-type-required |
| `validate` | `self.call_id.is_none` | [209](../../src/lib.rs#L209) | receiver-type-required |
| `validate` | `self.name.is_none` | [210](../../src/lib.rs#L210) | receiver-type-required |
| `validate` | `self.arguments_complete.is_none` | [211](../../src/lib.rs#L211) | receiver-type-required |
| `validate` | `"text/reasoning frame forbids call_id and name".to_owned` | [217](../../src/lib.rs#L217) | receiver-type-required |
| `negotiate` | `Err` | [284](../../src/lib.rs#L284), [289](../../src/lib.rs#L289) | external-constructor-callback-or-unresolved |
| `negotiate` | `ProtocolError::ProtocolName` | [284](../../src/lib.rs#L284) | external-constructor-callback-or-unresolved |
| `negotiate` | `hello.proto.clone` | [284](../../src/lib.rs#L284) | receiver-type-required |
| `negotiate` | `hello.min.max` | [286](../../src/lib.rs#L286) | receiver-type-required |
| `negotiate` | `hello.max.min` | [287](../../src/lib.rs#L287) | receiver-type-required |
| `negotiate` | `Ok` | [291](../../src/lib.rs#L291) | external-constructor-callback-or-unresolved |
| `decode_hello` | `decode_named` | [295](../../src/lib.rs#L295) | [worker-control::decode_named](../../src/lib.rs#L369) |
| `decode_selected` | `decode_named` | [299](../../src/lib.rs#L299) | [worker-control::decode_named](../../src/lib.rs#L369) |
| `decode_reject` | `decode_named` | [303](../../src/lib.rs#L303) | [worker-control::decode_named](../../src/lib.rs#L369) |
| `decode_supervisor` | `split_line` | [307](../../src/lib.rs#L307) | [worker-control::split_line](../../src/lib.rs#L380) |
| `decode_supervisor` | `Ok` | [308](../../src/lib.rs#L308) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `key.as_str` | [308](../../src/lib.rs#L308) | receiver-type-required |
| `decode_supervisor` | `SupervisorMessage::Input` | [309](../../src/lib.rs#L309) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `from_payload` | [309](../../src/lib.rs#L309), [310](../../src/lib.rs#L310), [311](../../src/lib.rs#L311), [312](../../src/lib.rs#L312), [313](../../src/lib.rs#L313), [314](../../src/lib.rs#L314), [315](../../src/lib.rs#L315), [316](../../src/lib.rs#L316), [317](../../src/lib.rs#L317), [319](../../src/lib.rs#L319) | [worker-control::from_payload](../../src/lib.rs#L393) |
| `decode_supervisor` | `SupervisorMessage::ApprovalResponse` | [310](../../src/lib.rs#L310) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::QueueEdit` | [311](../../src/lib.rs#L311) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::Stop` | [312](../../src/lib.rs#L312) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::Lease` | [313](../../src/lib.rs#L313) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::Ping` | [314](../../src/lib.rs#L314) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::Compact` | [315](../../src/lib.rs#L315) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::Meta` | [316](../../src/lib.rs#L316) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `SupervisorMessage::LaunchResult` | [317](../../src/lib.rs#L317) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `message                 .validate()                 .map_err` | [320](../../src/lib.rs#L320) | receiver-type-required |
| `decode_supervisor` | `message                 .validate` | [320](../../src/lib.rs#L320) | receiver-type-required |
| `decode_supervisor` | `ProtocolError::InvalidDurableControl` | [322](../../src/lib.rs#L322) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `source.to_string` | [322](../../src/lib.rs#L322) | receiver-type-required |
| `decode_supervisor` | `SupervisorMessage::QueueTransaction` | [323](../../src/lib.rs#L323) | external-constructor-callback-or-unresolved |
| `decode_supervisor` | `serde_json::from_value` | [327](../../src/lib.rs#L327) | external-constructor-callback-or-unresolved |
| `decode_worker` | `split_line` | [333](../../src/lib.rs#L333) | [worker-control::split_line](../../src/lib.rs#L380) |
| `decode_worker` | `Ok` | [334](../../src/lib.rs#L334) | external-constructor-callback-or-unresolved |
| `decode_worker` | `key.as_str` | [334](../../src/lib.rs#L334) | receiver-type-required |
| `decode_worker` | `WorkerMessage::Receipt` | [335](../../src/lib.rs#L335) | external-constructor-callback-or-unresolved |
| `decode_worker` | `from_payload` | [335](../../src/lib.rs#L335), [336](../../src/lib.rs#L336), [337](../../src/lib.rs#L337), [338](../../src/lib.rs#L338), [340](../../src/lib.rs#L340), [344](../../src/lib.rs#L344), [345](../../src/lib.rs#L345), [346](../../src/lib.rs#L346), [348](../../src/lib.rs#L348) | [worker-control::from_payload](../../src/lib.rs#L393) |
| `decode_worker` | `WorkerMessage::AttemptSettled` | [336](../../src/lib.rs#L336) | external-constructor-callback-or-unresolved |
| `decode_worker` | `WorkerMessage::LeaseRequest` | [337](../../src/lib.rs#L337) | external-constructor-callback-or-unresolved |
| `decode_worker` | `WorkerMessage::Appended` | [338](../../src/lib.rs#L338) | external-constructor-callback-or-unresolved |
| `decode_worker` | `frame.validate` | [341](../../src/lib.rs#L341) | receiver-type-required |
| `decode_worker` | `WorkerMessage::Frame` | [342](../../src/lib.rs#L342) | external-constructor-callback-or-unresolved |
| `decode_worker` | `WorkerMessage::State` | [344](../../src/lib.rs#L344) | external-constructor-callback-or-unresolved |
| `decode_worker` | `WorkerMessage::Pong` | [345](../../src/lib.rs#L345) | external-constructor-callback-or-unresolved |
| `decode_worker` | `WorkerMessage::LaunchChild` | [346](../../src/lib.rs#L346) | external-constructor-callback-or-unresolved |
| `decode_worker` | `message                 .validate()                 .map_err` | [349](../../src/lib.rs#L349) | receiver-type-required |
| `decode_worker` | `message                 .validate` | [349](../../src/lib.rs#L349) | receiver-type-required |
| `decode_worker` | `ProtocolError::InvalidDurableControl` | [351](../../src/lib.rs#L351) | external-constructor-callback-or-unresolved |
| `decode_worker` | `source.to_string` | [351](../../src/lib.rs#L351) | receiver-type-required |
| `decode_worker` | `WorkerMessage::QueueTransactionResult` | [352](../../src/lib.rs#L352) | external-constructor-callback-or-unresolved |
| `decode_worker` | `serde_json::from_value` | [356](../../src/lib.rs#L356) | external-constructor-callback-or-unresolved |
| `encode_line` | `Map::new` | [362](../../src/lib.rs#L362) | external-constructor-callback-or-unresolved |
| `encode_line` | `object.insert` | [363](../../src/lib.rs#L363) | receiver-type-required |
| `encode_line` | `key.to_owned` | [363](../../src/lib.rs#L363) | receiver-type-required |
| `encode_line` | `serde_json::to_value` | [363](../../src/lib.rs#L363) | external-constructor-callback-or-unresolved |
| `encode_line` | `serde_json_canonicalizer::to_vec` | [364](../../src/lib.rs#L364) | external-constructor-callback-or-unresolved |
| `encode_line` | `Value::Object` | [364](../../src/lib.rs#L364) | external-constructor-callback-or-unresolved |
| `encode_line` | `bytes.push` | [365](../../src/lib.rs#L365) | receiver-type-required |
| `encode_line` | `Ok` | [366](../../src/lib.rs#L366) | external-constructor-callback-or-unresolved |
| `decode_named` | `split_line` | [370](../../src/lib.rs#L370) | [worker-control::split_line](../../src/lib.rs#L380) |
| `decode_named` | `Err` | [372](../../src/lib.rs#L372) | external-constructor-callback-or-unresolved |
| `decode_named` | `expected.to_owned` | [373](../../src/lib.rs#L373) | receiver-type-required |
| `decode_named` | `from_payload` | [377](../../src/lib.rs#L377) | [worker-control::from_payload](../../src/lib.rs#L393) |
| `split_line` | `line.strip_suffix(b"\n").unwrap_or` | [381](../../src/lib.rs#L381) | receiver-type-required |
| `split_line` | `line.strip_suffix` | [381](../../src/lib.rs#L381) | receiver-type-required |
| `split_line` | `serde_json::from_slice` | [382](../../src/lib.rs#L382) | external-constructor-callback-or-unresolved |
| `split_line` | `value         .as_object()         .cloned()         .ok_or` | [383](../../src/lib.rs#L383) | receiver-type-required |
| `split_line` | `value         .as_object()         .cloned` | [383](../../src/lib.rs#L383) | receiver-type-required |
| `split_line` | `value         .as_object` | [383](../../src/lib.rs#L383) | receiver-type-required |
| `split_line` | `object.len` | [387](../../src/lib.rs#L387) | receiver-type-required |
| `split_line` | `Err` | [388](../../src/lib.rs#L388) | external-constructor-callback-or-unresolved |
| `split_line` | `Ok` | [390](../../src/lib.rs#L390) | external-constructor-callback-or-unresolved |
| `split_line` | `object.into_iter().next().expect` | [390](../../src/lib.rs#L390) | receiver-type-required |
| `split_line` | `object.into_iter().next` | [390](../../src/lib.rs#L390) | receiver-type-required |
| `split_line` | `object.into_iter` | [390](../../src/lib.rs#L390) | receiver-type-required |
| `from_payload` | `serde_json::from_value(payload).map_err` | [394](../../src/lib.rs#L394) | receiver-type-required |
| `from_payload` | `serde_json::from_value` | [394](../../src/lib.rs#L394) | external-constructor-callback-or-unresolved |
| `from_payload` | `message.to_owned` | [395](../../src/lib.rs#L395) | receiver-type-required |
