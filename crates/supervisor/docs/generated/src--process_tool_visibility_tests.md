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
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L147) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::process_tool_runtime_send_uses_receiver_authority_cold_and_hot](../../src/process_tool_visibility_tests.rs#L152) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot](../../src/process_tool_visibility_tests.rs#L194) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt](../../src/process_tool_visibility_tests.rs#L233) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::NoJobs](../../src/process_tool_visibility_tests.rs#L239) | struct_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::NoJobs::execute](../../src/process_tool_visibility_tests.rs#L241) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt::ACTOR](../../src/process_tool_visibility_tests.rs#L245) | const_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L360) | function_item | `private` | test; #[cfg(target_os = "macos")] |

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
| `live_receiver` | `Instant::now` | [108](../../src/process_tool_visibility_tests.rs#L108), [124](../../src/process_tool_visibility_tests.rs#L124) | external-constructor-callback-or-unresolved |
| `live_receiver` | `Duration::from_secs` | [108](../../src/process_tool_visibility_tests.rs#L108), [124](../../src/process_tool_visibility_tests.rs#L124) | external-constructor-callback-or-unresolved |
| `live_receiver` | `host.pending_workers.lock().unwrap().contains_key` | [109](../../src/process_tool_visibility_tests.rs#L109) | receiver-type-required |
| `live_receiver` | `host.pending_workers.lock().unwrap` | [109](../../src/process_tool_visibility_tests.rs#L109), [113](../../src/process_tool_visibility_tests.rs#L113) | receiver-type-required |
| `live_receiver` | `host.pending_workers.lock` | [109](../../src/process_tool_visibility_tests.rs#L109), [113](../../src/process_tool_visibility_tests.rs#L113) | receiver-type-required |
| `live_receiver` | `std::thread::sleep` | [111](../../src/process_tool_visibility_tests.rs#L111), [135](../../src/process_tool_visibility_tests.rs#L135) | external-constructor-callback-or-unresolved |
| `live_receiver` | `Duration::from_millis` | [111](../../src/process_tool_visibility_tests.rs#L111), [135](../../src/process_tool_visibility_tests.rs#L135) | external-constructor-callback-or-unresolved |
| `live_receiver` | `host.pending_workers.lock().unwrap().remove` | [113](../../src/process_tool_visibility_tests.rs#L113) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap().remove(SENDER).unwrap` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap().remove` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `host.workers.lock().unwrap` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `host.workers.lock` | [114](../../src/process_tool_visibility_tests.rs#L114) | receiver-type-required |
| `live_receiver` | `terminate_worker` | [115](../../src/process_tool_visibility_tests.rs#L115) | external-constructor-callback-or-unresolved |
| `live_receiver` | `host             .spawn_worker_at(RECEIVER, RECEIVER, ledger.to_owned(), None)             .unwrap` | [116](../../src/process_tool_visibility_tests.rs#L116) | receiver-type-required |
| `live_receiver` | `host             .spawn_worker_at` | [116](../../src/process_tool_visibility_tests.rs#L116) | receiver-type-required |
| `live_receiver` | `ledger.to_owned` | [117](../../src/process_tool_visibility_tests.rs#L117) | receiver-type-required |
| `live_receiver` | `host.workers             .lock()             .unwrap()             .insert` | [119](../../src/process_tool_visibility_tests.rs#L119) | receiver-type-required |
| `live_receiver` | `host.workers             .lock()             .unwrap` | [119](../../src/process_tool_visibility_tests.rs#L119) | receiver-type-required |
| `live_receiver` | `host.workers             .lock` | [119](../../src/process_tool_visibility_tests.rs#L119) | receiver-type-required |
| `live_receiver` | `RECEIVER.into` | [122](../../src/process_tool_visibility_tests.rs#L122) | receiver-type-required |
| `live_receiver` | `Arc::clone` | [122](../../src/process_tool_visibility_tests.rs#L122) | external-constructor-callback-or-unresolved |
| `live_receiver` | `PathBuf::from` | [123](../../src/process_tool_visibility_tests.rs#L123) | external-constructor-callback-or-unresolved |
| `live_receiver` | `worker.child.lock().unwrap().id` | [125](../../src/process_tool_visibility_tests.rs#L125) | receiver-type-required |
| `live_receiver` | `worker.child.lock().unwrap` | [125](../../src/process_tool_visibility_tests.rs#L125) | receiver-type-required |
| `live_receiver` | `worker.child.lock` | [125](../../src/process_tool_visibility_tests.rs#L125) | receiver-type-required |
| `live_receiver` | `fs::read_to_string(&ready)             .ok()             .and_then` | [126](../../src/process_tool_visibility_tests.rs#L126) | receiver-type-required |
| `live_receiver` | `fs::read_to_string(&ready)             .ok` | [126](../../src/process_tool_visibility_tests.rs#L126) | receiver-type-required |
| `live_receiver` | `fs::read_to_string` | [126](../../src/process_tool_visibility_tests.rs#L126) | external-constructor-callback-or-unresolved |
| `live_receiver` | `value.parse::<u32>().ok` | [128](../../src/process_tool_visibility_tests.rs#L128) | receiver-type-required |
| `live_receiver` | `value.parse::<u32>` | [128](../../src/process_tool_visibility_tests.rs#L128) | receiver-type-required |
| `live_receiver` | `Some` | [129](../../src/process_tool_visibility_tests.rs#L129) | external-constructor-callback-or-unresolved |
| `plain` | `serde_json::to_value(value).unwrap` | [148](../../src/process_tool_visibility_tests.rs#L148) | receiver-type-required |
| `plain` | `serde_json::to_value` | [148](../../src/process_tool_visibility_tests.rs#L148) | external-constructor-callback-or-unresolved |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `receiver_test` | [153](../../src/process_tool_visibility_tests.rs#L153) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `SENDER.into` | [155](../../src/process_tool_visibility_tests.rs#L155), [156](../../src/process_tool_visibility_tests.rs#L156) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `RECEIVER.into` | [157](../../src/process_tool_visibility_tests.rs#L157) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"context-cold-send".into` | [158](../../src/process_tool_visibility_tests.rs#L158) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"normal message".into` | [159](../../src/process_tool_visibility_tests.rs#L159) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `plain` | [162](../../src/process_tool_visibility_tests.rs#L162), [165](../../src/process_tool_visibility_tests.rs#L165), [171](../../src/process_tool_visibility_tests.rs#L171), [173](../../src/process_tool_visibility_tests.rs#L173) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L147) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `runtime.deliver_input(&request).unwrap` | [162](../../src/process_tool_visibility_tests.rs#L162), [165](../../src/process_tool_visibility_tests.rs#L165), [171](../../src/process_tool_visibility_tests.rs#L171), [173](../../src/process_tool_visibility_tests.rs#L173) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `runtime.deliver_input` | [162](../../src/process_tool_visibility_tests.rs#L162), [165](../../src/process_tool_visibility_tests.rs#L165), [171](../../src/process_tool_visibility_tests.rs#L171), [173](../../src/process_tool_visibility_tests.rs#L173) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `live_receiver` | [168](../../src/process_tool_visibility_tests.rs#L168) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::live_receiver](../../src/process_tool_visibility_tests.rs#L104) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `serde_json::to_vec(&worker.config_snapshot).unwrap` | [169](../../src/process_tool_visibility_tests.rs#L169) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `serde_json::to_vec` | [169](../../src/process_tool_visibility_tests.rs#L169) | external-constructor-callback-or-unresolved |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `"context-hot-send".into` | [170](../../src/process_tool_visibility_tests.rs#L170) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `read_root_events` | [184](../../src/process_tool_visibility_tests.rs#L184) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L360) |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `rows.iter().filter` | [186](../../src/process_tool_visibility_tests.rs#L186) | receiver-type-required |
| `process_tool_runtime_send_uses_receiver_authority_cold_and_hot` | `rows.iter` | [186](../../src/process_tool_visibility_tests.rs#L186) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `receiver_test` | [195](../../src/process_tool_visibility_tests.rs#L195) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `SENDER.into` | [197](../../src/process_tool_visibility_tests.rs#L197), [198](../../src/process_tool_visibility_tests.rs#L198) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `RECEIVER.into` | [199](../../src/process_tool_visibility_tests.rs#L199) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `"context-cold-stop".into` | [200](../../src/process_tool_visibility_tests.rs#L200) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `plain` | [202](../../src/process_tool_visibility_tests.rs#L202), [205](../../src/process_tool_visibility_tests.rs#L205), [210](../../src/process_tool_visibility_tests.rs#L210), [214](../../src/process_tool_visibility_tests.rs#L214) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L147) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `runtime.interrupt(&request).unwrap` | [202](../../src/process_tool_visibility_tests.rs#L202), [205](../../src/process_tool_visibility_tests.rs#L205), [210](../../src/process_tool_visibility_tests.rs#L210), [214](../../src/process_tool_visibility_tests.rs#L214) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `runtime.interrupt` | [202](../../src/process_tool_visibility_tests.rs#L202), [205](../../src/process_tool_visibility_tests.rs#L205), [210](../../src/process_tool_visibility_tests.rs#L210), [214](../../src/process_tool_visibility_tests.rs#L214) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `live_receiver` | [208](../../src/process_tool_visibility_tests.rs#L208) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::live_receiver](../../src/process_tool_visibility_tests.rs#L104) |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `"context-hot-stop".into` | [209](../../src/process_tool_visibility_tests.rs#L209) | receiver-type-required |
| `process_tool_runtime_interrupt_uses_receiver_authority_cold_and_hot` | `read_root_events` | [217](../../src/process_tool_visibility_tests.rs#L217) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L360) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `receiver_test` | [247](../../src/process_tool_visibility_tests.rs#L247) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::receiver_test](../../src/process_tool_visibility_tests.rs#L39) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `host.root.join("threads").join` | [249](../../src/process_tool_visibility_tests.rs#L249) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `host.root.join` | [249](../../src/process_tool_visibility_tests.rs#L249) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::create_dir_all(source.join("assets")).unwrap` | [250](../../src/process_tool_visibility_tests.rs#L250) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::create_dir_all` | [250](../../src/process_tool_visibility_tests.rs#L250) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `source.join` | [250](../../src/process_tool_visibility_tests.rs#L250), [263](../../src/process_tool_visibility_tests.rs#L263) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter()                     .map(&#124;line&#124; serde_json::from_str::<Value>(&line).unwrap())                     .collect::<Vec<_>>` | [251](../../src/process_tool_visibility_tests.rs#L251) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter()                     .map` | [251](../../src/process_tool_visibility_tests.rs#L251) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines(ACTOR)                     .into_iter` | [251](../../src/process_tool_visibility_tests.rs#L251) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_ledger_lines` | [251](../../src/process_tool_visibility_tests.rs#L251) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str::<Value>(&line).unwrap` | [253](../../src/process_tool_visibility_tests.rs#L253) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str::<Value>` | [253](../../src/process_tool_visibility_tests.rs#L253) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `args.clone` | [258](../../src/process_tool_visibility_tests.rs#L258) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str(&held_response_line(ACTOR)).unwrap` | [260](../../src/process_tool_visibility_tests.rs#L260) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::from_str` | [260](../../src/process_tool_visibility_tests.rs#L260) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `held_response_line` | [260](../../src/process_tool_visibility_tests.rs#L260) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.push` | [262](../../src/process_tool_visibility_tests.rs#L262) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `write_test_events` | [263](../../src/process_tool_visibility_tests.rs#L263) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControl::new(                     ACTOR,                     ACTOR,                     1,                     "ask1",                     "context",                     IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap(),                 )                 .unwrap` | [264](../../src/process_tool_visibility_tests.rs#L264) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControl::new` | [264](../../src/process_tool_visibility_tests.rs#L264) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap` | [270](../../src/process_tool_visibility_tests.rs#L270) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `IJsonValue::parse` | [270](../../src/process_tool_visibility_tests.rs#L270) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec(&args).unwrap` | [270](../../src/process_tool_visibility_tests.rs#L270) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec` | [270](../../src/process_tool_visibility_tests.rs#L270), [298](../../src/process_tool_visibility_tests.rs#L298) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlReceiptStore::new(&source, ACTOR).record_path` | [274](../../src/process_tool_visibility_tests.rs#L274) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlReceiptStore::new` | [274](../../src/process_tool_visibility_tests.rs#L274) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut().unwrap().remove` | [292](../../src/process_tool_visibility_tests.rs#L292) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut().unwrap` | [292](../../src/process_tool_visibility_tests.rs#L292) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fact.as_object_mut` | [292](../../src/process_tool_visibility_tests.rs#L292) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::read(ledger).unwrap` | [296](../../src/process_tool_visibility_tests.rs#L296), [348](../../src/process_tool_visibility_tests.rs#L348) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::read` | [296](../../src/process_tool_visibility_tests.rs#L296), [348](../../src/process_tool_visibility_tests.rs#L348) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.extend` | [297](../../src/process_tool_visibility_tests.rs#L297), [308](../../src/process_tool_visibility_tests.rs#L308) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap()                         .canonical_bytes()                         .unwrap` | [298](../../src/process_tool_visibility_tests.rs#L298) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap()                         .canonical_bytes` | [298](../../src/process_tool_visibility_tests.rs#L298) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode(&serde_json::to_vec(&fact).unwrap())                         .unwrap` | [298](../../src/process_tool_visibility_tests.rs#L298) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Event::decode` | [298](../../src/process_tool_visibility_tests.rs#L298) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `serde_json::to_vec(&fact).unwrap` | [298](../../src/process_tool_visibility_tests.rs#L298) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.push` | [303](../../src/process_tool_visibility_tests.rs#L303) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `schema::validate_ledger(&bytes, schema::READER_VERSION).unwrap` | [304](../../src/process_tool_visibility_tests.rs#L304) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `schema::validate_ledger` | [304](../../src/process_tool_visibility_tests.rs#L304) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1053) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::write(ledger, &bytes).unwrap` | [310](../../src/process_tool_visibility_tests.rs#L310), [336](../../src/process_tool_visibility_tests.rs#L336) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `fs::write` | [310](../../src/process_tool_visibility_tests.rs#L310), [336](../../src/process_tool_visibility_tests.rs#L336) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ToolControlSession::new` | [312](../../src/process_tool_visibility_tests.rs#L312) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ProductionToolControlHandler::new` | [315](../../src/process_tool_visibility_tests.rs#L315) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `ProductionToolControlPolicy::default` | [316](../../src/process_tool_visibility_tests.rs#L316) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `make_session` | [319](../../src/process_tool_visibility_tests.rs#L319), [349](../../src/process_tool_visibility_tests.rs#L349) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `runtime.clone` | [319](../../src/process_tool_visibility_tests.rs#L319) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `encode_tool_control(&request).unwrap` | [320](../../src/process_tool_visibility_tests.rs#L320) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `encode_tool_control` | [320](../../src/process_tool_visibility_tests.rs#L320) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `session.handle_line(&line).unwrap` | [321](../../src/process_tool_visibility_tests.rs#L321) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `session.handle_line` | [321](../../src/process_tool_visibility_tests.rs#L321) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `decode_tool_control_result(&first).unwrap` | [322](../../src/process_tool_visibility_tests.rs#L322) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `decode_tool_control_result` | [322](../../src/process_tool_visibility_tests.rs#L322) | external-constructor-callback-or-unresolved |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `plain` | [324](../../src/process_tool_visibility_tests.rs#L324) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::plain](../../src/process_tool_visibility_tests.rs#L147) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `result.value.unwrap` | [324](../../src/process_tool_visibility_tests.rs#L324) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.truncate` | [335](../../src/process_tool_visibility_tests.rs#L335) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter().rposition(&#124;byte&#124; *byte == b'\n').unwrap` | [335](../../src/process_tool_visibility_tests.rs#L335) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter().rposition` | [335](../../src/process_tool_visibility_tests.rs#L335) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `bytes.iter` | [335](../../src/process_tool_visibility_tests.rs#L335) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `read_root_events` | [342](../../src/process_tool_visibility_tests.rs#L342) | [tekes-supervisor::process_tool_visibility_tests::process_tool_visibility_tests::read_root_events](../../src/process_tool_visibility_tests.rs#L360) |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.last().unwrap` | [343](../../src/process_tool_visibility_tests.rs#L343) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `rows.last` | [343](../../src/process_tool_visibility_tests.rs#L343) | receiver-type-required |
| `legacy_hot_interrupt_replays_only_the_exact_stop_before_source_receipt` | `Weak::new` | [349](../../src/process_tool_visibility_tests.rs#L349) | external-constructor-callback-or-unresolved |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines()             .map(&#124;line&#124; serde_json::from_str(line).unwrap())             .collect` | [361](../../src/process_tool_visibility_tests.rs#L361) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines()             .map` | [361](../../src/process_tool_visibility_tests.rs#L361) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap()             .lines` | [361](../../src/process_tool_visibility_tests.rs#L361) | receiver-type-required |
| `read_root_events` | `fs::read_to_string(ledger)             .unwrap` | [361](../../src/process_tool_visibility_tests.rs#L361) | receiver-type-required |
| `read_root_events` | `fs::read_to_string` | [361](../../src/process_tool_visibility_tests.rs#L361) | external-constructor-callback-or-unresolved |
| `read_root_events` | `serde_json::from_str(line).unwrap` | [364](../../src/process_tool_visibility_tests.rs#L364) | receiver-type-required |
| `read_root_events` | `serde_json::from_str` | [364](../../src/process_tool_visibility_tests.rs#L364) | external-constructor-callback-or-unresolved |
