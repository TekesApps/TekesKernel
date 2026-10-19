# tekes-supervisor::process_tool_visibility_tests

[Package atlas](index.md) · [Source](../../src/process_tool_visibility_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::RECEIVER](../../src/process_tool_visibility_tests.rs#L7) | const_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::SENDER](../../src/process_tool_visibility_tests.rs#L8) | const_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::WORKER](../../src/process_tool_visibility_tests.rs#L12) | const_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::live_receiver](../../src/process_tool_visibility_tests.rs#L104) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L142) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::process_tool_runtime_send_uses_receiver_authority_cold_and_hot](../../src/process_tool_visibility_tests.rs#L147) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot](../../src/process_tool_visibility_tests.rs#L189) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt](../../src/process_tool_visibility_tests.rs#L228) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::NoJobs](../../src/process_tool_visibility_tests.rs#L234) | struct_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::NoJobs::execute](../../src/process_tool_visibility_tests.rs#L236) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::ACTOR](../../src/process_tool_visibility_tests.rs#L240) | const_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L355) | function_item | `private` | test; #[cfg(target_os = "macos")] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `SupervisorRuntimeAuthority` | `crate::production_tool_control::SupervisorRuntimeAuthority` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests` | `private` | #[cfg(target_os = "macos")] |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `receiver_test` | `tempfile::tempdir().unwrap` | [41](../../src/process_tool_visibility_tests.rs#L41), [42](../../src/process_tool_visibility_tests.rs#L42) | receiver-type-required |
| `receiver_test` | `tempfile::tempdir` | [41](../../src/process_tool_visibility_tests.rs#L41), [42](../../src/process_tool_visibility_tests.rs#L42) | external-constructor-callback-or-unresolved |
| `receiver_test` | `project_root.path().to_path_buf` | [43](../../src/process_tool_visibility_tests.rs#L43) | receiver-type-required |
| `receiver_test` | `project_root.path` | [43](../../src/process_tool_visibility_tests.rs#L43) | receiver-type-required |
| `receiver_test` | `fs::create_dir_all(&project).unwrap` | [44](../../src/process_tool_visibility_tests.rs#L44) | receiver-type-required |
| `receiver_test` | `fs::create_dir_all` | [44](../../src/process_tool_visibility_tests.rs#L44), [59](../../src/process_tool_visibility_tests.rs#L59), [63](../../src/process_tool_visibility_tests.rs#L63) | external-constructor-callback-or-unresolved |
| `receiver_test` | `write_test_execution_config` | [45](../../src/process_tool_visibility_tests.rs#L45) | external-constructor-callback-or-unresolved |
| `receiver_test` | `root.path` | [46](../../src/process_tool_visibility_tests.rs#L46), [52](../../src/process_tool_visibility_tests.rs#L52), [55](../../src/process_tool_visibility_tests.rs#L55), [58](../../src/process_tool_visibility_tests.rs#L58), [62](../../src/process_tool_visibility_tests.rs#L62), [69](../../src/process_tool_visibility_tests.rs#L69), [72](../../src/process_tool_visibility_tests.rs#L72) | receiver-type-required |
| `receiver_test` | `root.path().join` | [52](../../src/process_tool_visibility_tests.rs#L52), [55](../../src/process_tool_visibility_tests.rs#L55), [58](../../src/process_tool_visibility_tests.rs#L58), [62](../../src/process_tool_visibility_tests.rs#L62), [72](../../src/process_tool_visibility_tests.rs#L72) | receiver-type-required |
| `receiver_test` | `fs::write(&bin, WORKER).unwrap` | [53](../../src/process_tool_visibility_tests.rs#L53) | receiver-type-required |
| `receiver_test` | `fs::write` | [53](../../src/process_tool_visibility_tests.rs#L53), [56](../../src/process_tool_visibility_tests.rs#L56) | external-constructor-callback-or-unresolved |
| `receiver_test` | `fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap` | [54](../../src/process_tool_visibility_tests.rs#L54) | receiver-type-required |
| `receiver_test` | `fs::set_permissions` | [54](../../src/process_tool_visibility_tests.rs#L54), [57](../../src/process_tool_visibility_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `receiver_test` | `fs::Permissions::from_mode` | [54](../../src/process_tool_visibility_tests.rs#L54), [57](../../src/process_tool_visibility_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `receiver_test` | `fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap` | [56](../../src/process_tool_visibility_tests.rs#L56) | receiver-type-required |
| `receiver_test` | `fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap` | [57](../../src/process_tool_visibility_tests.rs#L57) | receiver-type-required |
| `receiver_test` | `root.path().join("threads").join` | [58](../../src/process_tool_visibility_tests.rs#L58), [62](../../src/process_tool_visibility_tests.rs#L62) | receiver-type-required |
| `receiver_test` | `fs::create_dir_all(folder.join("assets")).unwrap` | [59](../../src/process_tool_visibility_tests.rs#L59) | receiver-type-required |
| `receiver_test` | `folder.join` | [59](../../src/process_tool_visibility_tests.rs#L59), [60](../../src/process_tool_visibility_tests.rs#L60) | receiver-type-required |
| `receiver_test` | `write_test_genesis` | [61](../../src/process_tool_visibility_tests.rs#L61), [64](../../src/process_tool_visibility_tests.rs#L64) | external-constructor-callback-or-unresolved |
| `receiver_test` | `fs::create_dir_all(sender_folder.join("assets")).unwrap` | [63](../../src/process_tool_visibility_tests.rs#L63) | receiver-type-required |
| `receiver_test` | `sender_folder.join` | [63](../../src/process_tool_visibility_tests.rs#L63), [64](../../src/process_tool_visibility_tests.rs#L64), [78](../../src/process_tool_visibility_tests.rs#L78) | receiver-type-required |
| `receiver_test` | `fs::read(&ledger).unwrap` | [65](../../src/process_tool_visibility_tests.rs#L65), [92](../../src/process_tool_visibility_tests.rs#L92) | receiver-type-required |
| `receiver_test` | `fs::read` | [65](../../src/process_tool_visibility_tests.rs#L65), [67](../../src/process_tool_visibility_tests.rs#L67), [92](../../src/process_tool_visibility_tests.rs#L92) | external-constructor-callback-or-unresolved |
| `receiver_test` | `engine::write_permission_mode(&folder, mode).unwrap` | [66](../../src/process_tool_visibility_tests.rs#L66) | receiver-type-required |
| `receiver_test` | `engine::write_permission_mode` | [66](../../src/process_tool_visibility_tests.rs#L66) | [engine::permission_mode::write_permission_mode](../../../engine/src/permission_mode.rs#L126) |
| `receiver_test` | `fs::read(engine::permission_mode_path(&folder)).unwrap` | [67](../../src/process_tool_visibility_tests.rs#L67) | receiver-type-required |
| `receiver_test` | `engine::permission_mode_path` | [67](../../src/process_tool_visibility_tests.rs#L67) | [engine::permission_mode::permission_mode_path](../../../engine/src/permission_mode.rs#L88) |
| `receiver_test` | `ProductionProcessHost::open(                 root.path(),                 &bin,                 "test",                 root.path().join("missing-agent-home"),             )             .unwrap` | [68](../../src/process_tool_visibility_tests.rs#L68) | receiver-type-required |
| `receiver_test` | `ProductionProcessHost::open` | [68](../../src/process_tool_visibility_tests.rs#L68) | external-constructor-callback-or-unresolved |
| `receiver_test` | `host                 .spawn_worker_at(SENDER, SENDER, sender_folder.join("main.jsonl"), None)                 .unwrap` | [77](../../src/process_tool_visibility_tests.rs#L77) | receiver-type-required |
| `receiver_test` | `host                 .spawn_worker_at` | [77](../../src/process_tool_visibility_tests.rs#L77) | receiver-type-required |
| `receiver_test` | `host.workers.lock().unwrap().insert` | [80](../../src/process_tool_visibility_tests.rs#L80) | receiver-type-required |
| `receiver_test` | `host.workers.lock().unwrap` | [80](../../src/process_tool_visibility_tests.rs#L80) | receiver-type-required |
| `receiver_test` | `host.workers.lock` | [80](../../src/process_tool_visibility_tests.rs#L80) | receiver-type-required |
| `receiver_test` | `SENDER.into` | [80](../../src/process_tool_visibility_tests.rs#L80) | receiver-type-required |
| `receiver_test` | `Arc::downgrade` | [82](../../src/process_tool_visibility_tests.rs#L82) | external-constructor-callback-or-unresolved |
| `receiver_test` | `std::panic::catch_unwind` | [84](../../src/process_tool_visibility_tests.rs#L84) | external-constructor-callback-or-unresolved |
| `receiver_test` | `std::panic::AssertUnwindSafe` | [84](../../src/process_tool_visibility_tests.rs#L84) | external-constructor-callback-or-unresolved |
| `receiver_test` | `test` | [85](../../src/process_tool_visibility_tests.rs#L85) | external-constructor-callback-or-unresolved |
| `receiver_test` | `host.shutdown` | [87](../../src/process_tool_visibility_tests.rs#L87) | receiver-type-required |
| `receiver_test` | `schema::validate_ledger(&after, schema::READER_VERSION).unwrap` | [97](../../src/process_tool_visibility_tests.rs#L97) | receiver-type-required |
| `receiver_test` | `schema::validate_ledger` | [97](../../src/process_tool_visibility_tests.rs#L97) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1053) |
| `receiver_test` | `std::panic::resume_unwind` | [99](../../src/process_tool_visibility_tests.rs#L99) | external-constructor-callback-or-unresolved |
| `live_receiver` | `host.pending_workers.lock().unwrap().remove` | [108](../../src/process_tool_visibility_tests.rs#L108) | receiver-type-required |
| `live_receiver` | `host.pending_workers.lock().unwrap` | [108](../../src/process_tool_visibility_tests.rs#L108) | receiver-type-required |
| `live_receiver` | `host.pending_workers.lock` | [108](../../src/process_tool_visibility_tests.rs#L108) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap().remove(SENDER).unwrap` | [109](../../src/process_tool_visibility_tests.rs#L109) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap().remove` | [109](../../src/process_tool_visibility_tests.rs#L109) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap` | [109](../../src/process_tool_visibility_tests.rs#L109) | receiver-type-required |
| `live_receiver` | `host.workers.lock` | [109](../../src/process_tool_visibility_tests.rs#L109) | receiver-type-required |
| `live_receiver` | `terminate_worker` | [110](../../src/process_tool_visibility_tests.rs#L110) | external-constructor-callback-or-unresolved |
| `live_receiver` | `host             .spawn_worker_at(RECEIVER, RECEIVER, ledger.to_owned(), None)             .unwrap` | [111](../../src/process_tool_visibility_tests.rs#L111) | receiver-type-required |
| `live_receiver` | `host             .spawn_worker_at` | [111](../../src/process_tool_visibility_tests.rs#L111) | receiver-type-required |
| `live_receiver` | `ledger.to_owned` | [112](../../src/process_tool_visibility_tests.rs#L112) | receiver-type-required |
| `live_receiver` | `host.workers             .lock()             .unwrap()             .insert` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `host.workers             .lock()             .unwrap` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `host.workers             .lock` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `RECEIVER.into` | [117](../../src/process_tool_visibility_tests.rs#L117) | receiver-type-required |
| `live_receiver` | `Arc::clone` | [117](../../src/process_tool_visibility_tests.rs#L117) | external-constructor-callback-or-unresolved |
| `live_receiver` | `PathBuf::from` | [118](../../src/process_tool_visibility_tests.rs#L118) | external-constructor-callback-or-unresolved |
| `live_receiver` | `Instant::now` | [119](../../src/process_tool_visibility_tests.rs#L119) | external-constructor-callback-or-unresolved |
| `live_receiver` | `Duration::from_secs` | [119](../../src/process_tool_visibility_tests.rs#L119) | external-constructor-callback-or-unresolved |
| `live_receiver` | `worker.child.lock().unwrap().id` | [120](../../src/process_tool_visibility_tests.rs#L120) | receiver-type-required |
| `live_receiver` | `worker.child.lock().unwrap` | [120](../../src/process_tool_visibility_tests.rs#L120) | receiver-type-required |
| `live_receiver` | `worker.child.lock` | [120](../../src/process_tool_visibility_tests.rs#L120) | receiver-type-required |
| `live_receiver` | `fs::read_to_string(&ready)             .ok()             .and_then` | [121](../../src/process_tool_visibility_tests.rs#L121) | receiver-type-required |
| `live_receiver` | `fs::read_to_string(&ready)             .ok` | [121](../../src/process_tool_visibility_tests.rs#L121) | receiver-type-required |
| `live_receiver` | `fs::read_to_string` | [121](../../src/process_tool_visibility_tests.rs#L121) | external-constructor-callback-or-unresolved |
| `live_receiver` | `value.parse::<u32>().ok` | [123](../../src/process_tool_visibility_tests.rs#L123) | receiver-type-required |
| `live_receiver` | `value.parse::<u32>` | [123](../../src/process_tool_visibility_tests.rs#L123) | receiver-type-required |
| `live_receiver` | `Some` | [124](../../src/process_tool_visibility_tests.rs#L124) | external-constructor-callback-or-unresolved |
| `live_receiver` | `std::thread::sleep` | [130](../../src/process_tool_visibility_tests.rs#L130) | external-constructor-callback-or-unresolved |
| `live_receiver` | `Duration::from_millis` | [130](../../src/process_tool_visibility_tests.rs#L130) | external-constructor-callback-or-unresolved |
| `plain` | `serde_json::to_value(value).unwrap` | [143](../../src/process_tool_visibility_tests.rs#L143) | receiver-type-required |
| `plain` | `serde_json::to_value` | [143](../../src/process_tool_visibility_tests.rs#L143) | external-constructor-callback-or-unresolved |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `receiver_test` | [148](../../src/process_tool_visibility_tests.rs#L148) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `SENDER.into` | [150](../../src/process_tool_visibility_tests.rs#L150), [151](../../src/process_tool_visibility_tests.rs#L151) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `RECEIVER.into` | [152](../../src/process_tool_visibility_tests.rs#L152) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"context-cold-send".into` | [153](../../src/process_tool_visibility_tests.rs#L153) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"normal message".into` | [154](../../src/process_tool_visibility_tests.rs#L154) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `plain` | [157](../../src/process_tool_visibility_tests.rs#L157), [160](../../src/process_tool_visibility_tests.rs#L160), [166](../../src/process_tool_visibility_tests.rs#L166), [168](../../src/process_tool_visibility_tests.rs#L168) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L142) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `runtime.deliver_input(&request).unwrap` | [157](../../src/process_tool_visibility_tests.rs#L157), [160](../../src/process_tool_visibility_tests.rs#L160), [166](../../src/process_tool_visibility_tests.rs#L166), [168](../../src/process_tool_visibility_tests.rs#L168) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `runtime.deliver_input` | [157](../../src/process_tool_visibility_tests.rs#L157), [160](../../src/process_tool_visibility_tests.rs#L160), [166](../../src/process_tool_visibility_tests.rs#L166), [168](../../src/process_tool_visibility_tests.rs#L168) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `live_receiver` | [163](../../src/process_tool_visibility_tests.rs#L163) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::live_receiver](../../src/process_tool_visibility_tests.rs#L104) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `serde_json::to_vec(&worker.config_snapshot).unwrap` | [164](../../src/process_tool_visibility_tests.rs#L164) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `serde_json::to_vec` | [164](../../src/process_tool_visibility_tests.rs#L164) | external-constructor-callback-or-unresolved |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"context-hot-send".into` | [165](../../src/process_tool_visibility_tests.rs#L165) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `read_root_events` | [179](../../src/process_tool_visibility_tests.rs#L179) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L355) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `rows.iter().filter` | [181](../../src/process_tool_visibility_tests.rs#L181) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `rows.iter` | [181](../../src/process_tool_visibility_tests.rs#L181) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `receiver_test` | [190](../../src/process_tool_visibility_tests.rs#L190) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `SENDER.into` | [192](../../src/process_tool_visibility_tests.rs#L192), [193](../../src/process_tool_visibility_tests.rs#L193) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `RECEIVER.into` | [194](../../src/process_tool_visibility_tests.rs#L194) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `"context-cold-stop".into` | [195](../../src/process_tool_visibility_tests.rs#L195) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `plain` | [197](../../src/process_tool_visibility_tests.rs#L197), [200](../../src/process_tool_visibility_tests.rs#L200), [205](../../src/process_tool_visibility_tests.rs#L205), [209](../../src/process_tool_visibility_tests.rs#L209) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L142) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `runtime.interrupt(&request).unwrap` | [197](../../src/process_tool_visibility_tests.rs#L197), [200](../../src/process_tool_visibility_tests.rs#L200), [205](../../src/process_tool_visibility_tests.rs#L205), [209](../../src/process_tool_visibility_tests.rs#L209) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `runtime.interrupt` | [197](../../src/process_tool_visibility_tests.rs#L197), [200](../../src/process_tool_visibility_tests.rs#L200), [205](../../src/process_tool_visibility_tests.rs#L205), [209](../../src/process_tool_visibility_tests.rs#L209) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `live_receiver` | [203](../../src/process_tool_visibility_tests.rs#L203) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::live_receiver](../../src/process_tool_visibility_tests.rs#L104) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `"context-hot-stop".into` | [204](../../src/process_tool_visibility_tests.rs#L204) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `read_root_events` | [212](../../src/process_tool_visibility_tests.rs#L212) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L355) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `receiver_test` | [242](../../src/process_tool_visibility_tests.rs#L242) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `host.root.join("threads").join` | [244](../../src/process_tool_visibility_tests.rs#L244) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `host.root.join` | [244](../../src/process_tool_visibility_tests.rs#L244) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::create_dir_all(source.join("assets")).unwrap` | [245](../../src/process_tool_visibility_tests.rs#L245) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::create_dir_all` | [245](../../src/process_tool_visibility_tests.rs#L245) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `source.join` | [245](../../src/process_tool_visibility_tests.rs#L245), [258](../../src/process_tool_visibility_tests.rs#L258) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter()                     .map(&#124;line&#124; serde_json::from_str::<Value>(&line).unwrap())                     .collect::<Vec<_>>` | [246](../../src/process_tool_visibility_tests.rs#L246) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter()                     .map` | [246](../../src/process_tool_visibility_tests.rs#L246) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter` | [246](../../src/process_tool_visibility_tests.rs#L246) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines` | [246](../../src/process_tool_visibility_tests.rs#L246) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str::<Value>(&line).unwrap` | [248](../../src/process_tool_visibility_tests.rs#L248) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str::<Value>` | [248](../../src/process_tool_visibility_tests.rs#L248) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `args.clone` | [253](../../src/process_tool_visibility_tests.rs#L253) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str(&held_response_line(ACTOR)).unwrap` | [255](../../src/process_tool_visibility_tests.rs#L255) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str` | [255](../../src/process_tool_visibility_tests.rs#L255) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_response_line` | [255](../../src/process_tool_visibility_tests.rs#L255) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.push` | [257](../../src/process_tool_visibility_tests.rs#L257) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `write_test_events` | [258](../../src/process_tool_visibility_tests.rs#L258) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControl::new(                     ACTOR,                     ACTOR,                     1,                     "ask1",                     "context",                     IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap(),                 )                 .unwrap` | [259](../../src/process_tool_visibility_tests.rs#L259) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControl::new` | [259](../../src/process_tool_visibility_tests.rs#L259) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap` | [265](../../src/process_tool_visibility_tests.rs#L265) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `IJsonValue::parse` | [265](../../src/process_tool_visibility_tests.rs#L265) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec(&args).unwrap` | [265](../../src/process_tool_visibility_tests.rs#L265) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec` | [265](../../src/process_tool_visibility_tests.rs#L265), [293](../../src/process_tool_visibility_tests.rs#L293) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlReceiptStore::new(&source, ACTOR).record_path` | [269](../../src/process_tool_visibility_tests.rs#L269) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlReceiptStore::new` | [269](../../src/process_tool_visibility_tests.rs#L269) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut().unwrap().remove` | [287](../../src/process_tool_visibility_tests.rs#L287) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut().unwrap` | [287](../../src/process_tool_visibility_tests.rs#L287) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut` | [287](../../src/process_tool_visibility_tests.rs#L287) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::read(ledger).unwrap` | [291](../../src/process_tool_visibility_tests.rs#L291), [343](../../src/process_tool_visibility_tests.rs#L343) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::read` | [291](../../src/process_tool_visibility_tests.rs#L291), [343](../../src/process_tool_visibility_tests.rs#L343) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.extend` | [292](../../src/process_tool_visibility_tests.rs#L292), [303](../../src/process_tool_visibility_tests.rs#L303) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap()                         .canonical_bytes()                         .unwrap` | [293](../../src/process_tool_visibility_tests.rs#L293) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap()                         .canonical_bytes` | [293](../../src/process_tool_visibility_tests.rs#L293) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap` | [293](../../src/process_tool_visibility_tests.rs#L293) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode` | [293](../../src/process_tool_visibility_tests.rs#L293) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec(&fact).unwrap` | [293](../../src/process_tool_visibility_tests.rs#L293) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.push` | [298](../../src/process_tool_visibility_tests.rs#L298) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `schema::validate_ledger(&bytes, schema::READER_VERSION).unwrap` | [299](../../src/process_tool_visibility_tests.rs#L299) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `schema::validate_ledger` | [299](../../src/process_tool_visibility_tests.rs#L299) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1053) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::write(ledger, &bytes).unwrap` | [305](../../src/process_tool_visibility_tests.rs#L305), [331](../../src/process_tool_visibility_tests.rs#L331) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::write` | [305](../../src/process_tool_visibility_tests.rs#L305), [331](../../src/process_tool_visibility_tests.rs#L331) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlSession::new` | [307](../../src/process_tool_visibility_tests.rs#L307) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ProductionToolControlHandler::new` | [310](../../src/process_tool_visibility_tests.rs#L310) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ProductionToolControlPolicy::default` | [311](../../src/process_tool_visibility_tests.rs#L311) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `make_session` | [314](../../src/process_tool_visibility_tests.rs#L314), [344](../../src/process_tool_visibility_tests.rs#L344) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `runtime.clone` | [314](../../src/process_tool_visibility_tests.rs#L314) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `encode_tool_control(&request).unwrap` | [315](../../src/process_tool_visibility_tests.rs#L315) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `encode_tool_control` | [315](../../src/process_tool_visibility_tests.rs#L315) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `session.handle_line(&line).unwrap` | [316](../../src/process_tool_visibility_tests.rs#L316) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `session.handle_line` | [316](../../src/process_tool_visibility_tests.rs#L316) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `decode_tool_control_result(&first).unwrap` | [317](../../src/process_tool_visibility_tests.rs#L317) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `decode_tool_control_result` | [317](../../src/process_tool_visibility_tests.rs#L317) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `plain` | [319](../../src/process_tool_visibility_tests.rs#L319) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L142) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `result.value.unwrap` | [319](../../src/process_tool_visibility_tests.rs#L319) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.truncate` | [330](../../src/process_tool_visibility_tests.rs#L330) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter().rposition(&#124;byte&#124; *byte == b'\n').unwrap` | [330](../../src/process_tool_visibility_tests.rs#L330) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter().rposition` | [330](../../src/process_tool_visibility_tests.rs#L330) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter` | [330](../../src/process_tool_visibility_tests.rs#L330) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `read_root_events` | [337](../../src/process_tool_visibility_tests.rs#L337) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L355) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.last().unwrap` | [338](../../src/process_tool_visibility_tests.rs#L338) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.last` | [338](../../src/process_tool_visibility_tests.rs#L338) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Weak::new` | [344](../../src/process_tool_visibility_tests.rs#L344) | external-constructor-callback-or-unresolved |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines()             .map(&#124;line&#124; serde_json::from_str(line).unwrap())             .collect` | [356](../../src/process_tool_visibility_tests.rs#L356) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines()             .map` | [356](../../src/process_tool_visibility_tests.rs#L356) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines` | [356](../../src/process_tool_visibility_tests.rs#L356) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap` | [356](../../src/process_tool_visibility_tests.rs#L356) | receiver-type-required |
| `read_root_events` | `fs::read_to_string` | [356](../../src/process_tool_visibility_tests.rs#L356) | external-constructor-callback-or-unresolved |
| `read_root_events` | `serde_json::from_str(line).unwrap` | [359](../../src/process_tool_visibility_tests.rs#L359) | receiver-type-required |
| `read_root_events` | `serde_json::from_str` | [359](../../src/process_tool_visibility_tests.rs#L359) | external-constructor-callback-or-unresolved |
