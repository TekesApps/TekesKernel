# tekes-supervisor::process_live_tests

[Package atlas](index.md) · [Source](../../src/process_live_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::process_live_tests::real_worker_repeated_context_releases_queued_body](../../src/process_live_tests.rs#L4) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_process_promotes_frozen_candidate](../../src/process_live_tests.rs#L183) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_feedback_repairs_same_worker](../../src/process_live_tests.rs#L190) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L195) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::delivery_fallback_requires_confirmed_child_exit](../../src/process_live_tests.rs#L439) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::approval_closed_stdin_waits_for_actual_exit_before_fallback](../../src/process_live_tests.rs#L465) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::cancel_stage_failure_names_the_stage_and_keeps_typed_failures](../../src/process_live_tests.rs#L538) | function_item | `private` | test;  |
| [tekes-supervisor::process_live_tests::cancel_after_the_worker_closed_stdin_returns_the_durable_receipt](../../src/process_live_tests.rs#L565) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../src/process_live_tests.rs#L646) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../src/process_live_tests.rs#L838) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../src/process_live_tests.rs#L992) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1076) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../src/process_live_tests.rs#L1099) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_image_attachment](../../src/process_live_tests.rs#L1214) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_flow_case](../../src/process_live_tests.rs#L1287) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_mcp_task_continuation](../../src/process_live_tests.rs#L1468) | function_item | `private` | test; #[cfg(target_os = "macos")] |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `real_worker_repeated_context_releases_queued_body` | `PathBuf::from` | [5](../../src/process_live_tests.rs#L5) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [5](../../src/process_live_tests.rs#L5) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var` | [5](../../src/process_live_tests.rs#L5), [12](../../src/process_live_tests.rs#L12), [45](../../src/process_live_tests.rs#L45), [51](../../src/process_live_tests.rs#L51) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(&root).unwrap` | [6](../../src/process_live_tests.rs#L6) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all` | [6](../../src/process_live_tests.rs#L6), [8](../../src/process_live_tests.rs#L8), [9](../../src/process_live_tests.rs#L9), [10](../../src/process_live_tests.rs#L10), [28](../../src/process_live_tests.rs#L28) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `root.parent().unwrap().join` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.parent().unwrap` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.parent` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(&workspace).unwrap` | [8](../../src/process_live_tests.rs#L8) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(root.join("config")).unwrap` | [9](../../src/process_live_tests.rs#L9) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.join` | [9](../../src/process_live_tests.rs#L9), [10](../../src/process_live_tests.rs#L10), [16](../../src/process_live_tests.rs#L16), [20](../../src/process_live_tests.rs#L20), [27](../../src/process_live_tests.rs#L27), [53](../../src/process_live_tests.rs#L53), [177](../../src/process_live_tests.rs#L177) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [10](../../src/process_live_tests.rs#L10) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [11](../../src/process_live_tests.rs#L11) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_slice` | [11](../../src/process_live_tests.rs#L11) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [12](../../src/process_live_tests.rs#L12) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read` | [12](../../src/process_live_tests.rs#L12) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [12](../../src/process_live_tests.rs#L12) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `write_canonical_test_json` | [15](../../src/process_live_tests.rs#L15), [19](../../src/process_live_tests.rs#L19) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `root.join("threads").join` | [27](../../src/process_live_tests.rs#L27) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(folder.join("assets")).unwrap` | [28](../../src/process_live_tests.rs#L28) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `folder.join` | [28](../../src/process_live_tests.rs#L28), [29](../../src/process_live_tests.rs#L29) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `write_test_genesis` | [30](../../src/process_live_tests.rs#L30) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `append_test_input` | [31](../../src/process_live_tests.rs#L31) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [32](../../src/process_live_tests.rs#L32), [109](../../src/process_live_tests.rs#L109) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map` | [32](../../src/process_live_tests.rs#L32), [109](../../src/process_live_tests.rs#L109) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines` | [32](../../src/process_live_tests.rs#L32), [109](../../src/process_live_tests.rs#L109) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap` | [32](../../src/process_live_tests.rs#L32), [109](../../src/process_live_tests.rs#L109) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string` | [32](../../src/process_live_tests.rs#L32), [65](../../src/process_live_tests.rs#L65), [109](../../src/process_live_tests.rs#L109) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str(s).unwrap` | [35](../../src/process_live_tests.rs#L35), [112](../../src/process_live_tests.rs#L112) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str` | [35](../../src/process_live_tests.rs#L35), [68](../../src/process_live_tests.rs#L68), [112](../../src/process_live_tests.rs#L112) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `write_test_events` | [38](../../src/process_live_tests.rs#L38) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Arc::new` | [39](../../src/process_live_tests.rs#L39) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `provider::MemorySecretStore::new` | [39](../../src/process_live_tests.rs#L39) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_worker_repeated_context_releases_queued_body` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [40](../../src/process_live_tests.rs#L40) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `secrets         .publish` | [40](../../src/process_live_tests.rs#L40) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `provider["credential_key"].as_str().unwrap` | [42](../../src/process_live_tests.rs#L42) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `provider["credential_key"].as_str` | [42](../../src/process_live_tests.rs#L42) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [45](../../src/process_live_tests.rs#L45) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-process",         root.join(".agent"),         secrets,     )     .unwrap` | [49](../../src/process_live_tests.rs#L49) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `ProductionProcessHost::open_with_secret_store` | [49](../../src/process_live_tests.rs#L49) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [51](../../src/process_live_tests.rs#L51) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::catch_unwind` | [57](../../src/process_live_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::AssertUnwindSafe` | [57](../../src/process_live_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main(session)             .unwrap()             .expect` | [58](../../src/process_live_tests.rs#L58) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main(session)             .unwrap` | [58](../../src/process_live_tests.rs#L58) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main` | [58](../../src/process_live_tests.rs#L58) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.start_periodic_sweep` | [61](../../src/process_live_tests.rs#L61) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `Instant::now` | [62](../../src/process_live_tests.rs#L62) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Duration::from_secs` | [62](../../src/process_live_tests.rs#L62) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map(&#124;s&#124; serde_json::from_str(s).ok())                 .collect` | [65](../../src/process_live_tests.rs#L65) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map` | [65](../../src/process_live_tests.rs#L65) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines` | [65](../../src/process_live_tests.rs#L65) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap` | [65](../../src/process_live_tests.rs#L65) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str(s).ok` | [68](../../src/process_live_tests.rs#L68) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().any` | [70](../../src/process_live_tests.rs#L70) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter` | [70](../../src/process_live_tests.rs#L70), [94](../../src/process_live_tests.rs#L94) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"test".into` | [72](../../src/process_live_tests.rs#L72) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"live-process".into` | [73](../../src/process_live_tests.rs#L73) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `session.into` | [74](../../src/process_live_tests.rs#L74) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"session.prompt".into` | [75](../../src/process_live_tests.rs#L75) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"queued-body".into` | [76](../../src/process_live_tests.rs#L76) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.prompt(                     session,                     "2026-09-04T10:00:00.000Z",                     &origin,                     &MaterializedPrompt {                         blocks: vec![Block::Text {                             text: "重试".into(),                         }],                         attachments: vec![],                         files: Vec::new(),                     },                     false,                 )                 .unwrap` | [78](../../src/process_live_tests.rs#L78) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.prompt` | [78](../../src/process_live_tests.rs#L78) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `Vec::new` | [87](../../src/process_live_tests.rs#L87) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().filter(&#124;e&#124; e["kind"] == "settle").count` | [94](../../src/process_live_tests.rs#L94) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().filter` | [94](../../src/process_live_tests.rs#L94) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::thread::sleep` | [102](../../src/process_live_tests.rs#L102) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Duration::from_millis` | [102](../../src/process_live_tests.rs#L102) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `host.shutdown` | [105](../../src/process_live_tests.rs#L105) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::resume_unwind` | [107](../../src/process_live_tests.rs#L107) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 1)         .unwrap` | [115](../../src/process_live_tests.rs#L115) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find` | [115](../../src/process_live_tests.rs#L115), [119](../../src/process_live_tests.rs#L119), [133](../../src/process_live_tests.rs#L133), [137](../../src/process_live_tests.rs#L137) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter` | [115](../../src/process_live_tests.rs#L115), [119](../../src/process_live_tests.rs#L119), [133](../../src/process_live_tests.rs#L133), [137](../../src/process_live_tests.rs#L137), [144](../../src/process_live_tests.rs#L144) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 2)         .unwrap` | [119](../../src/process_live_tests.rs#L119) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find(&#124;e&#124; e["seq"] == settle["promoted_output_seq"])             .unwrap` | [126](../../src/process_live_tests.rs#L126) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find` | [126](../../src/process_live_tests.rs#L126), [155](../../src/process_live_tests.rs#L155) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter` | [126](../../src/process_live_tests.rs#L126), [155](../../src/process_live_tests.rs#L155) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "turn_open" && e["turn"] == 2)         .unwrap` | [133](../../src/process_live_tests.rs#L133) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "input" && e["content"][0]["text"] == "重试")         .unwrap` | [137](../../src/process_live_tests.rs#L137) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .filter(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "context_get")         .collect` | [144](../../src/process_live_tests.rs#L144) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .filter` | [144](../../src/process_live_tests.rs#L144) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.iter().enumerate` | [154](../../src/process_live_tests.rs#L154) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.iter` | [154](../../src/process_live_tests.rs#L154) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find(&#124;e&#124; e["kind"] == "tool_result" && e["call"] == call["call"])             .unwrap` | [155](../../src/process_live_tests.rs#L155) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls             .get(index + 1)             .map_or` | [161](../../src/process_live_tests.rs#L161) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls             .get` | [161](../../src/process_live_tests.rs#L161) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `first["seq"].as_u64().unwrap` | [163](../../src/process_live_tests.rs#L163) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `first["seq"].as_u64` | [163](../../src/process_live_tests.rs#L163) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `next["seq"].as_u64().unwrap` | [164](../../src/process_live_tests.rs#L164) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `next["seq"].as_u64` | [164](../../src/process_live_tests.rs#L164) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.get` | [170](../../src/process_live_tests.rs#L170) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"Real worker process and supervisor context_get, four model responses then queued ordinary input; no artifact validator or external client transport.","first_settle":first["seq"],"second_settle":second["seq"]})).unwrap()).unwrap` | [177](../../src/process_live_tests.rs#L177) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::write` | [177](../../src/process_live_tests.rs#L177) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::to_vec_pretty(&json!({"status":"passed","scope":"Real worker process and supervisor context_get, four model responses then queued ordinary input; no artifact validator or external client transport.","first_settle":first["seq"],"second_settle":second["seq"]})).unwrap` | [177](../../src/process_live_tests.rs#L177) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::to_vec_pretty` | [177](../../src/process_live_tests.rs#L177) | external-constructor-callback-or-unresolved |
| `real_validator_process_promotes_frozen_candidate` | `run_real_validator_process` | [184](../../src/process_live_tests.rs#L184) | [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L195) |
| `real_validator_feedback_repairs_same_worker` | `run_real_validator_process` | [191](../../src/process_live_tests.rs#L191) | [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L195) |
| `run_real_validator_process` | `PathBuf::from` | [196](../../src/process_live_tests.rs#L196), [322](../../src/process_live_tests.rs#L322) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [196](../../src/process_live_tests.rs#L196) | receiver-type-required |
| `run_real_validator_process` | `std::env::var` | [196](../../src/process_live_tests.rs#L196), [203](../../src/process_live_tests.rs#L203), [276](../../src/process_live_tests.rs#L276), [282](../../src/process_live_tests.rs#L282) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::create_dir_all(&root).unwrap` | [197](../../src/process_live_tests.rs#L197) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all` | [197](../../src/process_live_tests.rs#L197), [199](../../src/process_live_tests.rs#L199), [200](../../src/process_live_tests.rs#L200), [201](../../src/process_live_tests.rs#L201), [221](../../src/process_live_tests.rs#L221) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `root.parent().unwrap().join` | [198](../../src/process_live_tests.rs#L198) | receiver-type-required |
| `run_real_validator_process` | `root.parent().unwrap` | [198](../../src/process_live_tests.rs#L198) | receiver-type-required |
| `run_real_validator_process` | `root.parent` | [198](../../src/process_live_tests.rs#L198) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(&workspace).unwrap` | [199](../../src/process_live_tests.rs#L199) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(root.join("config")).unwrap` | [200](../../src/process_live_tests.rs#L200) | receiver-type-required |
| `run_real_validator_process` | `root.join` | [200](../../src/process_live_tests.rs#L200), [201](../../src/process_live_tests.rs#L201), [207](../../src/process_live_tests.rs#L207), [211](../../src/process_live_tests.rs#L211), [220](../../src/process_live_tests.rs#L220), [284](../../src/process_live_tests.rs#L284), [434](../../src/process_live_tests.rs#L434) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [201](../../src/process_live_tests.rs#L201) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [202](../../src/process_live_tests.rs#L202) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_slice` | [202](../../src/process_live_tests.rs#L202) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [203](../../src/process_live_tests.rs#L203) | receiver-type-required |
| `run_real_validator_process` | `fs::read` | [203](../../src/process_live_tests.rs#L203) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [203](../../src/process_live_tests.rs#L203) | receiver-type-required |
| `run_real_validator_process` | `write_canonical_test_json` | [206](../../src/process_live_tests.rs#L206), [210](../../src/process_live_tests.rs#L210) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `root.join("threads").join` | [220](../../src/process_live_tests.rs#L220) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(folder.join("assets")).unwrap` | [221](../../src/process_live_tests.rs#L221) | receiver-type-required |
| `run_real_validator_process` | `folder.join` | [221](../../src/process_live_tests.rs#L221), [222](../../src/process_live_tests.rs#L222), [238](../../src/process_live_tests.rs#L238), [355](../../src/process_live_tests.rs#L355), [418](../../src/process_live_tests.rs#L418) | receiver-type-required |
| `run_real_validator_process` | `write_test_genesis` | [223](../../src/process_live_tests.rs#L223) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `append_test_input` | [224](../../src/process_live_tests.rs#L224) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [225](../../src/process_live_tests.rs#L225), [387](../../src/process_live_tests.rs#L387) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map` | [225](../../src/process_live_tests.rs#L225), [387](../../src/process_live_tests.rs#L387) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines` | [225](../../src/process_live_tests.rs#L225), [387](../../src/process_live_tests.rs#L387) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap` | [225](../../src/process_live_tests.rs#L225), [387](../../src/process_live_tests.rs#L387) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string` | [225](../../src/process_live_tests.rs#L225), [302](../../src/process_live_tests.rs#L302), [355](../../src/process_live_tests.rs#L355), [387](../../src/process_live_tests.rs#L387), [419](../../src/process_live_tests.rs#L419) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `serde_json::from_str(s).unwrap` | [228](../../src/process_live_tests.rs#L228), [390](../../src/process_live_tests.rs#L390), [422](../../src/process_live_tests.rs#L422) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str` | [228](../../src/process_live_tests.rs#L228), [305](../../src/process_live_tests.rs#L305), [358](../../src/process_live_tests.rs#L358), [390](../../src/process_live_tests.rs#L390), [422](../../src/process_live_tests.rs#L422) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `workspace.join` | [230](../../src/process_live_tests.rs#L230), [326](../../src/process_live_tests.rs#L326) | receiver-type-required |
| `run_real_validator_process` | `b"WRONG".as_slice` | [232](../../src/process_live_tests.rs#L232) | receiver-type-required |
| `run_real_validator_process` | `b"KERNEL_VALIDATOR_OK".as_slice` | [234](../../src/process_live_tests.rs#L234) | receiver-type-required |
| `run_real_validator_process` | `fs::write(&artifact, initial).unwrap` | [236](../../src/process_live_tests.rs#L236) | receiver-type-required |
| `run_real_validator_process` | `fs::write` | [236](../../src/process_live_tests.rs#L236), [434](../../src/process_live_tests.rs#L434) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap()         .publish(b"{}")         .unwrap` | [238](../../src/process_live_tests.rs#L238) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap()         .publish` | [238](../../src/process_live_tests.rs#L238) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap` | [238](../../src/process_live_tests.rs#L238) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new` | [238](../../src/process_live_tests.rs#L238) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `events.extend` | [242](../../src/process_live_tests.rs#L242) | receiver-type-required |
| `run_real_validator_process` | `events.iter_mut().enumerate` | [249](../../src/process_live_tests.rs#L249) | receiver-type-required |
| `run_real_validator_process` | `events.iter_mut` | [249](../../src/process_live_tests.rs#L249) | receiver-type-required |
| `run_real_validator_process` | `write_test_events` | [254](../../src/process_live_tests.rs#L254) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `store::LockedLedger::open(&path, schema::READER_VERSION).unwrap` | [255](../../src/process_live_tests.rs#L255) | receiver-type-required |
| `run_real_validator_process` | `store::LockedLedger::open` | [255](../../src/process_live_tests.rs#L255) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L186) |
| `run_real_validator_process` | `session.into` | [257](../../src/process_live_tests.rs#L257), [258](../../src/process_live_tests.rs#L258), [337](../../src/process_live_tests.rs#L337) | receiver-type-required |
| `run_real_validator_process` | `[(             artifact.to_string_lossy().into_owned(),             format!("sha256-{:x}", sha2::Sha256::digest(initial)),         )]         .into_iter()         .collect` | [261](../../src/process_live_tests.rs#L261) | receiver-type-required |
| `run_real_validator_process` | `[(             artifact.to_string_lossy().into_owned(),             format!("sha256-{:x}", sha2::Sha256::digest(initial)),         )]         .into_iter` | [261](../../src/process_live_tests.rs#L261) | receiver-type-required |
| `run_real_validator_process` | `artifact.to_string_lossy().into_owned` | [262](../../src/process_live_tests.rs#L262) | receiver-type-required |
| `run_real_validator_process` | `artifact.to_string_lossy` | [262](../../src/process_live_tests.rs#L262) | receiver-type-required |
| `run_real_validator_process` | `engine::begin_validation(&mut ledger, "2026-09-04T00:00:01.000Z", &binding).unwrap` | [268](../../src/process_live_tests.rs#L268) | receiver-type-required |
| `run_real_validator_process` | `engine::begin_validation` | [268](../../src/process_live_tests.rs#L268) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `run_real_validator_process` | `drop` | [269](../../src/process_live_tests.rs#L269) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Arc::new` | [270](../../src/process_live_tests.rs#L270), [290](../../src/process_live_tests.rs#L290) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `provider::MemorySecretStore::new` | [270](../../src/process_live_tests.rs#L270) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `run_real_validator_process` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [271](../../src/process_live_tests.rs#L271) | receiver-type-required |
| `run_real_validator_process` | `secrets         .publish` | [271](../../src/process_live_tests.rs#L271) | receiver-type-required |
| `run_real_validator_process` | `provider["credential_key"].as_str().unwrap` | [273](../../src/process_live_tests.rs#L273) | receiver-type-required |
| `run_real_validator_process` | `provider["credential_key"].as_str` | [273](../../src/process_live_tests.rs#L273) | receiver-type-required |
| `run_real_validator_process` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [276](../../src/process_live_tests.rs#L276) | receiver-type-required |
| `run_real_validator_process` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     )     .unwrap` | [280](../../src/process_live_tests.rs#L280) | receiver-type-required |
| `run_real_validator_process` | `ProductionProcessHost::open_with_secret_store` | [280](../../src/process_live_tests.rs#L280) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [282](../../src/process_live_tests.rs#L282) | receiver-type-required |
| `run_real_validator_process` | `crate::endpoint_carrier::ProductionRespondAuthority::open(         &root,         Arc::new(crate::endpoint_host::SessionAdmissionGates::default()),         host.clone(),     )     .unwrap` | [288](../../src/process_live_tests.rs#L288) | receiver-type-required |
| `run_real_validator_process` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [288](../../src/process_live_tests.rs#L288) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L177) |
| `run_real_validator_process` | `crate::endpoint_host::SessionAdmissionGates::default` | [290](../../src/process_live_tests.rs#L290) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.clone` | [291](../../src/process_live_tests.rs#L291) | receiver-type-required |
| `run_real_validator_process` | `Vec::new` | [294](../../src/process_live_tests.rs#L294) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::panic::catch_unwind` | [295](../../src/process_live_tests.rs#L295) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::panic::AssertUnwindSafe` | [295](../../src/process_live_tests.rs#L295) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.schedule_main(session)             .unwrap()             .expect` | [296](../../src/process_live_tests.rs#L296) | receiver-type-required |
| `run_real_validator_process` | `host.schedule_main(session)             .unwrap` | [296](../../src/process_live_tests.rs#L296) | receiver-type-required |
| `run_real_validator_process` | `host.schedule_main` | [296](../../src/process_live_tests.rs#L296) | receiver-type-required |
| `run_real_validator_process` | `host.start_periodic_sweep` | [299](../../src/process_live_tests.rs#L299) | receiver-type-required |
| `run_real_validator_process` | `Instant::now` | [300](../../src/process_live_tests.rs#L300) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Duration::from_secs` | [300](../../src/process_live_tests.rs#L300) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map(&#124;s&#124; serde_json::from_str(s).ok())                 .collect` | [302](../../src/process_live_tests.rs#L302) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map` | [302](../../src/process_live_tests.rs#L302) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines` | [302](../../src/process_live_tests.rs#L302) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap` | [302](../../src/process_live_tests.rs#L302) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str(s).ok` | [305](../../src/process_live_tests.rs#L305) | receiver-type-required |
| `run_real_validator_process` | `events.iter().filter` | [307](../../src/process_live_tests.rs#L307) | receiver-type-required |
| `run_real_validator_process` | `events.iter` | [307](../../src/process_live_tests.rs#L307), [351](../../src/process_live_tests.rs#L351), [392](../../src/process_live_tests.rs#L392), [417](../../src/process_live_tests.rs#L417) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .any` | [308](../../src/process_live_tests.rs#L308) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter` | [308](../../src/process_live_tests.rs#L308), [314](../../src/process_live_tests.rs#L314) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .find(&#124;e&#124; e["kind"] == "tool_call" && e["call"] == held["call"])                     .unwrap` | [314](../../src/process_live_tests.rs#L314) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .find` | [314](../../src/process_live_tests.rs#L314) | receiver-type-required |
| `run_real_validator_process` | `call["args"]["path"].as_str().unwrap` | [322](../../src/process_live_tests.rs#L322) | receiver-type-required |
| `run_real_validator_process` | `call["args"]["path"].as_str` | [322](../../src/process_live_tests.rs#L322) | receiver-type-required |
| `run_real_validator_process` | `requested.is_absolute` | [323](../../src/process_live_tests.rs#L323) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap().canonicalize().unwrap` | [328](../../src/process_live_tests.rs#L328) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap().canonicalize` | [328](../../src/process_live_tests.rs#L328) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap` | [328](../../src/process_live_tests.rs#L328) | receiver-type-required |
| `run_real_validator_process` | `requested.parent` | [328](../../src/process_live_tests.rs#L328) | receiver-type-required |
| `run_real_validator_process` | `key.clone` | [335](../../src/process_live_tests.rs#L335) | receiver-type-required |
| `run_real_validator_process` | `"live-disposable-approval".into` | [336](../../src/process_live_tests.rs#L336) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str().unwrap().into` | [338](../../src/process_live_tests.rs#L338) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str().unwrap` | [338](../../src/process_live_tests.rs#L338) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str` | [338](../../src/process_live_tests.rs#L338) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author(&responder, authorization.clone())                     .expect` | [344](../../src/process_live_tests.rs#L344) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author` | [344](../../src/process_live_tests.rs#L344), [346](../../src/process_live_tests.rs#L346) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `authorization.clone` | [344](../../src/process_live_tests.rs#L344) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author(&responder, authorization)                     .expect` | [346](../../src/process_live_tests.rs#L346) | receiver-type-required |
| `run_real_validator_process` | `approval_deliveries.push` | [349](../../src/process_live_tests.rs#L349) | receiver-type-required |
| `run_real_validator_process` | `events.iter().find` | [351](../../src/process_live_tests.rs#L351), [392](../../src/process_live_tests.rs#L392) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines()                     .filter_map(&#124;line&#124; serde_json::from_str(line).ok())                     .collect` | [355](../../src/process_live_tests.rs#L355) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines()                     .filter_map` | [355](../../src/process_live_tests.rs#L355) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines` | [355](../../src/process_live_tests.rs#L355) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default` | [355](../../src/process_live_tests.rs#L355) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str(line).ok` | [358](../../src/process_live_tests.rs#L358) | receiver-type-required |
| `run_real_validator_process` | `journal.iter().find` | [360](../../src/process_live_tests.rs#L360), [364](../../src/process_live_tests.rs#L364) | receiver-type-required |
| `run_real_validator_process` | `journal.iter` | [360](../../src/process_live_tests.rs#L360), [364](../../src/process_live_tests.rs#L364) | receiver-type-required |
| `run_real_validator_process` | `journal.iter().find(&#124;e&#124;                         e["event"]["type"] == "assistant/message"                             && &e["event"]["data"]["message"]["id"] == promoted                     ).expect` | [364](../../src/process_live_tests.rs#L364) | receiver-type-required |
| `run_real_validator_process` | `std::thread::sleep` | [380](../../src/process_live_tests.rs#L380) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Duration::from_millis` | [380](../../src/process_live_tests.rs#L380) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.shutdown` | [383](../../src/process_live_tests.rs#L383) | receiver-type-required |
| `run_real_validator_process` | `std::panic::resume_unwind` | [385](../../src/process_live_tests.rs#L385) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `events.iter().find(&#124;e&#124; e["kind"] == "settle").unwrap` | [392](../../src/process_live_tests.rs#L392) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev().find(&#124;e&#124; e["kind"] == "spawn").unwrap` | [417](../../src/process_live_tests.rs#L417) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev().find` | [417](../../src/process_live_tests.rs#L417) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev` | [417](../../src/process_live_tests.rs#L417) | receiver-type-required |
| `run_real_validator_process` | `spawn["child"].as_str().unwrap` | [418](../../src/process_live_tests.rs#L418) | receiver-type-required |
| `run_real_validator_process` | `spawn["child"].as_str` | [418](../../src/process_live_tests.rs#L418) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [419](../../src/process_live_tests.rs#L419) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines()         .map` | [419](../../src/process_live_tests.rs#L419) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines` | [419](../../src/process_live_tests.rs#L419) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap` | [419](../../src/process_live_tests.rs#L419) | receiver-type-required |
| `run_real_validator_process` | `child         .iter()         .find(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "verify")         .unwrap` | [426](../../src/process_live_tests.rs#L426) | receiver-type-required |
| `run_real_validator_process` | `child         .iter()         .find` | [426](../../src/process_live_tests.rs#L426) | receiver-type-required |
| `run_real_validator_process` | `child         .iter` | [426](../../src/process_live_tests.rs#L426) | receiver-type-required |
| `run_real_validator_process` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","repair":repair,"approval_deliveries":approval_deliveries,"scope":"Recovered root candidate and real independently spawned validator process with live model; frozen artifact pass and exact promotion. Root generation is seeded, not a live mutation.","settle_seq":settle["seq"],"child":child_path})).unwrap()).unwrap` | [434](../../src/process_live_tests.rs#L434) | receiver-type-required |
| `run_real_validator_process` | `serde_json::to_vec_pretty(&json!({"status":"passed","repair":repair,"approval_deliveries":approval_deliveries,"scope":"Recovered root candidate and real independently spawned validator process with live model; frozen artifact pass and exact promotion. Root generation is seeded, not a live mutation.","settle_seq":settle["seq"],"child":child_path})).unwrap` | [434](../../src/process_live_tests.rs#L434) | receiver-type-required |
| `run_real_validator_process` | `serde_json::to_vec_pretty` | [434](../../src/process_live_tests.rs#L434) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `Mutex::new` | [440](../../src/process_live_tests.rs#L440), [447](../../src/process_live_tests.rs#L447) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args(["-c", "sleep 0.05; exit 0"])             .spawn()             .unwrap` | [441](../../src/process_live_tests.rs#L441) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args(["-c", "sleep 0.05; exit 0"])             .spawn` | [441](../../src/process_live_tests.rs#L441) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args` | [441](../../src/process_live_tests.rs#L441) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new` | [441](../../src/process_live_tests.rs#L441), [448](../../src/process_live_tests.rs#L448) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg("5")             .spawn()             .unwrap` | [448](../../src/process_live_tests.rs#L448) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg("5")             .spawn` | [448](../../src/process_live_tests.rs#L448) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg` | [448](../../src/process_live_tests.rs#L448) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child_exited_within` | [453](../../src/process_live_tests.rs#L453) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `Duration::from_millis` | [453](../../src/process_live_tests.rs#L453) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `live.lock().unwrap` | [454](../../src/process_live_tests.rs#L454) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `live.lock` | [454](../../src/process_live_tests.rs#L454) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.kill().unwrap` | [455](../../src/process_live_tests.rs#L455) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.kill` | [455](../../src/process_live_tests.rs#L455) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.wait().unwrap` | [456](../../src/process_live_tests.rs#L456) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.wait` | [456](../../src/process_live_tests.rs#L456) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `tempfile::tempdir().unwrap` | [466](../../src/process_live_tests.rs#L466) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `tempfile::tempdir` | [466](../../src/process_live_tests.rs#L466) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join` | [467](../../src/process_live_tests.rs#L467), [469](../../src/process_live_tests.rs#L469), [471](../../src/process_live_tests.rs#L471), [474](../../src/process_live_tests.rs#L474), [489](../../src/process_live_tests.rs#L489), [493](../../src/process_live_tests.rs#L493), [498](../../src/process_live_tests.rs#L498), [502](../../src/process_live_tests.rs#L502), [504](../../src/process_live_tests.rs#L504) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path` | [467](../../src/process_live_tests.rs#L467), [469](../../src/process_live_tests.rs#L469), [471](../../src/process_live_tests.rs#L471), [474](../../src/process_live_tests.rs#L474), [489](../../src/process_live_tests.rs#L489), [493](../../src/process_live_tests.rs#L493), [498](../../src/process_live_tests.rs#L498), [502](../../src/process_live_tests.rs#L502), [504](../../src/process_live_tests.rs#L504) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all(&workspace).unwrap` | [468](../../src/process_live_tests.rs#L468) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all` | [468](../../src/process_live_tests.rs#L468), [469](../../src/process_live_tests.rs#L469), [494](../../src/process_live_tests.rs#L494) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all(root.path().join("workspaces/ws")).unwrap` | [469](../../src/process_live_tests.rs#L469) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `write_canonical_test_json` | [470](../../src/process_live_tests.rs#L470) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(         &bin,         br##"#!/bin/sh printf '%s\n' '{"hello":{"max":2,"min":2,"proto":"tekes-worker"}}' IFS= read -r selected while [ ! -f "$0.exit" ]; do sleep 0.01; done exec 0<&- : > "$0.closed" sleep 0.2 exit 0 "##,     )     .unwrap` | [475](../../src/process_live_tests.rs#L475) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write` | [475](../../src/process_live_tests.rs#L475), [490](../../src/process_live_tests.rs#L490), [502](../../src/process_live_tests.rs#L502) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap` | [488](../../src/process_live_tests.rs#L488) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions` | [488](../../src/process_live_tests.rs#L488), [491](../../src/process_live_tests.rs#L491) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::Permissions::from_mode` | [488](../../src/process_live_tests.rs#L488), [491](../../src/process_live_tests.rs#L491) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap` | [490](../../src/process_live_tests.rs#L490) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap` | [491](../../src/process_live_tests.rs#L491) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join("threads").join` | [493](../../src/process_live_tests.rs#L493) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all(folder.join("assets")).unwrap` | [494](../../src/process_live_tests.rs#L494) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `folder.join` | [494](../../src/process_live_tests.rs#L494), [495](../../src/process_live_tests.rs#L495), [496](../../src/process_live_tests.rs#L496) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `write_test_genesis` | [495](../../src/process_live_tests.rs#L495) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `append_test_input` | [496](../../src/process_live_tests.rs#L496) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent")).unwrap` | [498](../../src/process_live_tests.rs#L498) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `ProductionProcessHost::open` | [498](../../src/process_live_tests.rs#L498) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::catch_unwind` | [499](../../src/process_live_tests.rs#L499) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::AssertUnwindSafe` | [499](../../src/process_live_tests.rs#L499) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main(session).unwrap().unwrap` | [500](../../src/process_live_tests.rs#L500) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main(session).unwrap` | [500](../../src/process_live_tests.rs#L500) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main` | [500](../../src/process_live_tests.rs#L500) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.live_worker(session).unwrap` | [501](../../src/process_live_tests.rs#L501) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.live_worker` | [501](../../src/process_live_tests.rs#L501) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(root.path().join("fake-worker.exit"), b"exit").unwrap` | [502](../../src/process_live_tests.rs#L502) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Instant::now` | [503](../../src/process_live_tests.rs#L503) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Duration::from_secs` | [503](../../src/process_live_tests.rs#L503) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join("fake-worker.closed").exists` | [504](../../src/process_live_tests.rs#L504) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::thread::sleep` | [506](../../src/process_live_tests.rs#L506) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Duration::from_millis` | [506](../../src/process_live_tests.rs#L506) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"approval".into` | [513](../../src/process_live_tests.rs#L513), [519](../../src/process_live_tests.rs#L519) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"test".into` | [515](../../src/process_live_tests.rs#L515), [516](../../src/process_live_tests.rs#L516) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `session.into` | [517](../../src/process_live_tests.rs#L517) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"respond".into` | [518](../../src/process_live_tests.rs#L518) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"call".into` | [521](../../src/process_live_tests.rs#L521) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.shutdown` | [531](../../src/process_live_tests.rs#L531) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::resume_unwind` | [533](../../src/process_live_tests.rs#L533) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `Instant::now` | [539](../../src/process_live_tests.rs#L539) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `cancel_stage_failure` | [540](../../src/process_live_tests.rs#L540), [553](../../src/process_live_tests.rs#L553) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `internal` | [543](../../src/process_live_tests.rs#L543) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `failure` | [556](../../src/process_live_tests.rs#L556) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `tempfile::tempdir().unwrap` | [566](../../src/process_live_tests.rs#L566) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `tempfile::tempdir` | [566](../../src/process_live_tests.rs#L566) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join` | [567](../../src/process_live_tests.rs#L567), [569](../../src/process_live_tests.rs#L569), [571](../../src/process_live_tests.rs#L571), [574](../../src/process_live_tests.rs#L574), [589](../../src/process_live_tests.rs#L589), [593](../../src/process_live_tests.rs#L593), [598](../../src/process_live_tests.rs#L598), [602](../../src/process_live_tests.rs#L602), [604](../../src/process_live_tests.rs#L604) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path` | [567](../../src/process_live_tests.rs#L567), [569](../../src/process_live_tests.rs#L569), [571](../../src/process_live_tests.rs#L571), [574](../../src/process_live_tests.rs#L574), [589](../../src/process_live_tests.rs#L589), [593](../../src/process_live_tests.rs#L593), [598](../../src/process_live_tests.rs#L598), [602](../../src/process_live_tests.rs#L602), [604](../../src/process_live_tests.rs#L604) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all(&workspace).unwrap` | [568](../../src/process_live_tests.rs#L568) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all` | [568](../../src/process_live_tests.rs#L568), [569](../../src/process_live_tests.rs#L569), [594](../../src/process_live_tests.rs#L594) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all(root.path().join("workspaces/ws")).unwrap` | [569](../../src/process_live_tests.rs#L569) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `write_canonical_test_json` | [570](../../src/process_live_tests.rs#L570) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(         &bin,         br##"#!/bin/sh printf '%s\n' '{"hello":{"max":2,"min":2,"proto":"tekes-worker"}}' IFS= read -r selected while [ ! -f "$0.exit" ]; do sleep 0.01; done exec 0<&- : > "$0.closed" sleep 0.2 exit 0 "##,     )     .unwrap` | [575](../../src/process_live_tests.rs#L575) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write` | [575](../../src/process_live_tests.rs#L575), [590](../../src/process_live_tests.rs#L590), [602](../../src/process_live_tests.rs#L602) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap` | [588](../../src/process_live_tests.rs#L588) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions` | [588](../../src/process_live_tests.rs#L588), [591](../../src/process_live_tests.rs#L591) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::Permissions::from_mode` | [588](../../src/process_live_tests.rs#L588), [591](../../src/process_live_tests.rs#L591) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap` | [590](../../src/process_live_tests.rs#L590) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap` | [591](../../src/process_live_tests.rs#L591) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join("threads").join` | [593](../../src/process_live_tests.rs#L593) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all(folder.join("assets")).unwrap` | [594](../../src/process_live_tests.rs#L594) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `folder.join` | [594](../../src/process_live_tests.rs#L594), [595](../../src/process_live_tests.rs#L595), [596](../../src/process_live_tests.rs#L596), [626](../../src/process_live_tests.rs#L626) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `write_test_genesis` | [595](../../src/process_live_tests.rs#L595) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `append_test_input` | [596](../../src/process_live_tests.rs#L596) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent")).unwrap` | [598](../../src/process_live_tests.rs#L598) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `ProductionProcessHost::open` | [598](../../src/process_live_tests.rs#L598) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::catch_unwind` | [599](../../src/process_live_tests.rs#L599) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::AssertUnwindSafe` | [599](../../src/process_live_tests.rs#L599) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main(session).unwrap().unwrap` | [600](../../src/process_live_tests.rs#L600) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main(session).unwrap` | [600](../../src/process_live_tests.rs#L600) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main` | [600](../../src/process_live_tests.rs#L600) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.live_worker(session).unwrap` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.live_worker` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(root.path().join("fake-worker.exit"), b"exit").unwrap` | [602](../../src/process_live_tests.rs#L602) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Instant::now` | [603](../../src/process_live_tests.rs#L603) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Duration::from_secs` | [603](../../src/process_live_tests.rs#L603) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join("fake-worker.closed").exists` | [604](../../src/process_live_tests.rs#L604) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::thread::sleep` | [606](../../src/process_live_tests.rs#L606) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Duration::from_millis` | [606](../../src/process_live_tests.rs#L606) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"test".into` | [613](../../src/process_live_tests.rs#L613), [614](../../src/process_live_tests.rs#L614) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `session.into` | [615](../../src/process_live_tests.rs#L615) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"session.cancel".into` | [616](../../src/process_live_tests.rs#L616) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"stop-after-exit".into` | [617](../../src/process_live_tests.rs#L617) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host             .cancel(session, "2026-08-28T00:00:05.000Z", &origin)             .expect` | [619](../../src/process_live_tests.rs#L619) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host             .cancel` | [619](../../src/process_live_tests.rs#L619) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::read(folder.join("main.jsonl")).unwrap` | [626](../../src/process_live_tests.rs#L626) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::read` | [626](../../src/process_live_tests.rs#L626) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `scan_valid_prefix(&bytes, schema::READER_VERSION).projection.unwrap` | [627](../../src/process_live_tests.rs#L627) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `scan_valid_prefix` | [627](../../src/process_live_tests.rs#L627) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter()             .filter(&#124;event&#124; event.kind() == &EventKind::StopRequested)             .collect::<Vec<_>>` | [628](../../src/process_live_tests.rs#L628) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter()             .filter` | [628](../../src/process_live_tests.rs#L628) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter` | [628](../../src/process_live_tests.rs#L628) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `event.kind` | [631](../../src/process_live_tests.rs#L631) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.shutdown` | [637](../../src/process_live_tests.rs#L637) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::resume_unwind` | [639](../../src/process_live_tests.rs#L639) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `PathBuf::from` | [647](../../src/process_live_tests.rs#L647), [748](../../src/process_live_tests.rs#L748) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [647](../../src/process_live_tests.rs#L647) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var` | [647](../../src/process_live_tests.rs#L647), [659](../../src/process_live_tests.rs#L659), [691](../../src/process_live_tests.rs#L691), [697](../../src/process_live_tests.rs#L697), [724](../../src/process_live_tests.rs#L724) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&root).unwrap` | [648](../../src/process_live_tests.rs#L648) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all` | [648](../../src/process_live_tests.rs#L648), [650](../../src/process_live_tests.rs#L650), [651](../../src/process_live_tests.rs#L651), [653](../../src/process_live_tests.rs#L653), [654](../../src/process_live_tests.rs#L654), [676](../../src/process_live_tests.rs#L676) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap().join` | [649](../../src/process_live_tests.rs#L649) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap` | [649](../../src/process_live_tests.rs#L649) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent` | [649](../../src/process_live_tests.rs#L649) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&workspace).unwrap` | [650](../../src/process_live_tests.rs#L650) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(workspace.join(".agent/skills")).unwrap` | [651](../../src/process_live_tests.rs#L651) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.join` | [651](../../src/process_live_tests.rs#L651), [652](../../src/process_live_tests.rs#L652), [683](../../src/process_live_tests.rs#L683), [752](../../src/process_live_tests.rs#L752) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(workspace.join(".agent/skills/live-proof.md"), "Live proof skill\nSKILL_LIVE_BODY_OK\n").unwrap` | [652](../../src/process_live_tests.rs#L652) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write` | [652](../../src/process_live_tests.rs#L652), [716](../../src/process_live_tests.rs#L716), [831](../../src/process_live_tests.rs#L831) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(root.join("config")).unwrap` | [653](../../src/process_live_tests.rs#L653) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join` | [653](../../src/process_live_tests.rs#L653), [654](../../src/process_live_tests.rs#L654), [655](../../src/process_live_tests.rs#L655), [663](../../src/process_live_tests.rs#L663), [667](../../src/process_live_tests.rs#L667), [675](../../src/process_live_tests.rs#L675), [699](../../src/process_live_tests.rs#L699), [708](../../src/process_live_tests.rs#L708), [716](../../src/process_live_tests.rs#L716), [718](../../src/process_live_tests.rs#L718), [790](../../src/process_live_tests.rs#L790), [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [654](../../src/process_live_tests.rs#L654) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_canonical_test_json` | [655](../../src/process_live_tests.rs#L655), [662](../../src/process_live_tests.rs#L662), [666](../../src/process_live_tests.rs#L666) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [658](../../src/process_live_tests.rs#L658) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice` | [658](../../src/process_live_tests.rs#L658) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [659](../../src/process_live_tests.rs#L659) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read` | [659](../../src/process_live_tests.rs#L659) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [659](../../src/process_live_tests.rs#L659) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join("threads").join` | [675](../../src/process_live_tests.rs#L675) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(folder.join("assets")).unwrap` | [676](../../src/process_live_tests.rs#L676) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `folder.join` | [676](../../src/process_live_tests.rs#L676), [677](../../src/process_live_tests.rs#L677), [783](../../src/process_live_tests.rs#L783), [822](../../src/process_live_tests.rs#L822), [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_test_genesis` | [678](../../src/process_live_tests.rs#L678) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `append_test_input` | [679](../../src/process_live_tests.rs#L679) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map` | [680](../../src/process_live_tests.rs#L680), [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines` | [680](../../src/process_live_tests.rs#L680), [732](../../src/process_live_tests.rs#L732), [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap` | [680](../../src/process_live_tests.rs#L680), [732](../../src/process_live_tests.rs#L732), [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string` | [680](../../src/process_live_tests.rs#L680), [732](../../src/process_live_tests.rs#L732), [783](../../src/process_live_tests.rs#L783), [801](../../src/process_live_tests.rs#L801), [822](../../src/process_live_tests.rs#L822), [828](../../src/process_live_tests.rs#L828) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).unwrap` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str` | [680](../../src/process_live_tests.rs#L680), [732](../../src/process_live_tests.rs#L732), [783](../../src/process_live_tests.rs#L783), [801](../../src/process_live_tests.rs#L801), [822](../../src/process_live_tests.rs#L822), [828](../../src/process_live_tests.rs#L828) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `write_test_events` | [682](../../src/process_live_tests.rs#L682) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Arc::new` | [685](../../src/process_live_tests.rs#L685), [708](../../src/process_live_tests.rs#L708), [723](../../src/process_live_tests.rs#L723) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `provider::MemorySecretStore::new` | [685](../../src/process_live_tests.rs#L685) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [686](../../src/process_live_tests.rs#L686) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish` | [686](../../src/process_live_tests.rs#L686) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str().unwrap` | [688](../../src/process_live_tests.rs#L688) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str` | [688](../../src/process_live_tests.rs#L688) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [691](../../src/process_live_tests.rs#L691) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     )     .unwrap` | [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store` | [695](../../src/process_live_tests.rs#L695) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [697](../../src/process_live_tests.rs#L697) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new().unwrap` | [703](../../src/process_live_tests.rs#L703) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new` | [703](../../src/process_live_tests.rs#L703) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [704](../../src/process_live_tests.rs#L704) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on` | [704](../../src/process_live_tests.rs#L704), [799](../../src/process_live_tests.rs#L799) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::net::TcpListener::bind` | [704](../../src/process_live_tests.rs#L704) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr().unwrap` | [705](../../src/process_live_tests.rs#L705) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr` | [705](../../src/process_live_tests.rs#L705) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [706](../../src/process_live_tests.rs#L706) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::host_runtime::assemble_application_endpoint_host` | [706](../../src/process_live_tests.rs#L706) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_validator_exhaustion_releases_queued_input` | `"test".into` | [707](../../src/process_live_tests.rs#L707), [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy().into_owned` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy().into_owned` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Ok` | [708](../../src/process_live_tests.rs#L708), [724](../../src/process_live_tests.rs#L724) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `"2026-09-04T10:00:00.000Z".into` | [708](../../src/process_live_tests.rs#L708) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.clone` | [708](../../src/process_live_tests.rs#L708), [709](../../src/process_live_tests.rs#L709), [723](../../src/process_live_tests.rs#L723) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),         transport::TransportConfig::loopback(address, transport::BearerToken::new([42; 32]))).unwrap` | [709](../../src/process_live_tests.rs#L709) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [709](../../src/process_live_tests.rs#L709) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_validator_exhaustion_releases_queued_input` | `transport::TransportConfig::loopback` | [710](../../src/process_live_tests.rs#L710) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_validator_exhaustion_releases_queued_input` | `transport::BearerToken::new` | [710](../../src/process_live_tests.rs#L710) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_validator_exhaustion_releases_queued_input` | `host.attach_streams` | [711](../../src/process_live_tests.rs#L711) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams().clone` | [711](../../src/process_live_tests.rs#L711) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams` | [711](../../src/process_live_tests.rs#L711) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery().unwrap` | [712](../../src/process_live_tests.rs#L712) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery` | [712](../../src/process_live_tests.rs#L712) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.into_server` | [713](../../src/process_live_tests.rs#L713) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.handle` | [714](../../src/process_live_tests.rs#L714) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.spawn` | [715](../../src/process_live_tests.rs#L715) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.serve` | [715](../../src/process_live_tests.rs#L715) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec` | [716](../../src/process_live_tests.rs#L716) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Instant::now` | [717](../../src/process_live_tests.rs#L717), [730](../../src/process_live_tests.rs#L730) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_secs` | [717](../../src/process_live_tests.rs#L717), [730](../../src/process_live_tests.rs#L730) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-ready").exists` | [718](../../src/process_live_tests.rs#L718) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::thread::sleep` | [720](../../src/process_live_tests.rs#L720), [794](../../src/process_live_tests.rs#L794) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_millis` | [720](../../src/process_live_tests.rs#L720), [794](../../src/process_live_tests.rs#L794) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open(         &root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [722](../../src/process_live_tests.rs#L722) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L177) |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_host::SessionAdmissionGates::default` | [723](../../src/process_live_tests.rs#L723) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [724](../../src/process_live_tests.rs#L724) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Vec::new` | [725](../../src/process_live_tests.rs#L725) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::catch_unwind` | [726](../../src/process_live_tests.rs#L726) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::AssertUnwindSafe` | [726](../../src/process_live_tests.rs#L726) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap().expect` | [727](../../src/process_live_tests.rs#L727) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap` | [727](../../src/process_live_tests.rs#L727) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main` | [727](../../src/process_live_tests.rs#L727) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.start_periodic_sweep` | [728](../../src/process_live_tests.rs#L728) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).ok` | [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter` | [733](../../src/process_live_tests.rs#L733), [782](../../src/process_live_tests.rs#L782), [786](../../src/process_live_tests.rs#L786), [806](../../src/process_live_tests.rs#L806), [826](../../src/process_live_tests.rs#L826) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter` | [733](../../src/process_live_tests.rs#L733), [777](../../src/process_live_tests.rs#L777), [782](../../src/process_live_tests.rs#L782), [786](../../src/process_live_tests.rs#L786), [806](../../src/process_live_tests.rs#L806), [813](../../src/process_live_tests.rs#L813), [814](../../src/process_live_tests.rs#L814), [817](../../src/process_live_tests.rs#L817), [819](../../src/process_live_tests.rs#L819), [821](../../src/process_live_tests.rs#L821), [826](../../src/process_live_tests.rs#L826) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .any` | [734](../../src/process_live_tests.rs#L734) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter` | [734](../../src/process_live_tests.rs#L734), [740](../../src/process_live_tests.rs#L740) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find(&#124;e&#124; e["kind"] == "tool_call" && e["call"] == held["call"])                     .unwrap` | [740](../../src/process_live_tests.rs#L740) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find` | [740](../../src/process_live_tests.rs#L740) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str().unwrap` | [748](../../src/process_live_tests.rs#L748) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str` | [748](../../src/process_live_tests.rs#L748) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.is_absolute` | [749](../../src/process_live_tests.rs#L749) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize().unwrap` | [754](../../src/process_live_tests.rs#L754) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize` | [754](../../src/process_live_tests.rs#L754) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap` | [754](../../src/process_live_tests.rs#L754) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent` | [754](../../src/process_live_tests.rs#L754) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `key.clone` | [761](../../src/process_live_tests.rs#L761) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"live-disposable-approval".into` | [762](../../src/process_live_tests.rs#L762) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `session.into` | [763](../../src/process_live_tests.rs#L763), [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap().into` | [764](../../src/process_live_tests.rs#L764) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap` | [764](../../src/process_live_tests.rs#L764) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str` | [764](../../src/process_live_tests.rs#L764) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization.clone())                     .expect` | [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author` | [770](../../src/process_live_tests.rs#L770), [772](../../src/process_live_tests.rs#L772) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `authorization.clone` | [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization)                     .expect` | [772](../../src/process_live_tests.rs#L772) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `approval_deliveries.push` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().any` | [777](../../src/process_live_tests.rs#L777) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"exhaustion-process".into` | [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"session.prompt".into` | [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"queued-body".into` | [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt(session,"2026-09-04T10:00:00.000Z",&origin,&MaterializedPrompt {blocks:vec![Block::Text {text:"重试".into()}],attachments:vec![],files:vec![]},false).unwrap` | [779](../../src/process_live_tests.rs#L779) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt` | [779](../../src/process_live_tests.rs#L779) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "settle").count` | [782](../../src/process_live_tests.rs#L782) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [783](../../src/process_live_tests.rs#L783) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map` | [783](../../src/process_live_tests.rs#L783) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines` | [783](../../src/process_live_tests.rs#L783) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default` | [783](../../src/process_live_tests.rs#L783) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).ok` | [783](../../src/process_live_tests.rs#L783) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter(&#124;e&#124; e["event"]["type"] == "turn/end").collect` | [784](../../src/process_live_tests.rs#L784) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter` | [784](../../src/process_live_tests.rs#L784) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter` | [784](../../src/process_live_tests.rs#L784) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.len` | [785](../../src/process_live_tests.rs#L785) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter().zip` | [786](../../src/process_live_tests.rs#L786) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter` | [786](../../src/process_live_tests.rs#L786) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-receipt.json").exists` | [790](../../src/process_live_tests.rs#L790) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.shutdown` | [797](../../src/process_live_tests.rs#L797) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_handle.begin_drain` | [798](../../src/process_live_tests.rs#L798) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap().unwrap` | [799](../../src/process_live_tests.rs#L799) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap` | [799](../../src/process_live_tests.rs#L799) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::resume_unwind` | [800](../../src/process_live_tests.rs#L800) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).unwrap` | [801](../../src/process_live_tests.rs#L801), [822](../../src/process_live_tests.rs#L822), [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "approval_response").collect` | [806](../../src/process_live_tests.rs#L806) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 1).unwrap` | [813](../../src/process_live_tests.rs#L813) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find` | [813](../../src/process_live_tests.rs#L813), [814](../../src/process_live_tests.rs#L814), [817](../../src/process_live_tests.rs#L817), [819](../../src/process_live_tests.rs#L819), [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 2).unwrap` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "turn_open" && e["turn"] == 2).unwrap` | [817](../../src/process_live_tests.rs#L817) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "input" && e["content"][0]["text"] == "重试").unwrap` | [819](../../src/process_live_tests.rs#L819) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "spawn").unwrap` | [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str().unwrap` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str` | [822](../../src/process_live_tests.rs#L822) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "spawn").collect` | [826](../../src/process_live_tests.rs#L826) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str().unwrap` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "verify").expect` | [829](../../src/process_live_tests.rs#L829) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find` | [829](../../src/process_live_tests.rs#L829) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter` | [829](../../src/process_live_tests.rs#L829) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap()).unwrap` | [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap` | [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty` | [831](../../src/process_live_tests.rs#L831) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `PathBuf::from` | [839](../../src/process_live_tests.rs#L839), [931](../../src/process_live_tests.rs#L931) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [839](../../src/process_live_tests.rs#L839) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var` | [839](../../src/process_live_tests.rs#L839), [844](../../src/process_live_tests.rs#L844), [855](../../src/process_live_tests.rs#L855), [890](../../src/process_live_tests.rs#L890), [891](../../src/process_live_tests.rs#L891), [892](../../src/process_live_tests.rs#L892) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.parent().unwrap().join` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent().unwrap` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("docs")).unwrap` | [841](../../src/process_live_tests.rs#L841) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all` | [841](../../src/process_live_tests.rs#L841), [842](../../src/process_live_tests.rs#L842), [843](../../src/process_live_tests.rs#L843), [850](../../src/process_live_tests.rs#L850), [859](../../src/process_live_tests.rs#L859), [862](../../src/process_live_tests.rs#L862) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `workspace.join` | [841](../../src/process_live_tests.rs#L841), [859](../../src/process_live_tests.rs#L859), [862](../../src/process_live_tests.rs#L862), [864](../../src/process_live_tests.rs#L864), [865](../../src/process_live_tests.rs#L865), [866](../../src/process_live_tests.rs#L866), [870](../../src/process_live_tests.rs#L870), [871](../../src/process_live_tests.rs#L871), [872](../../src/process_live_tests.rs#L872), [932](../../src/process_live_tests.rs#L932), [960](../../src/process_live_tests.rs#L960) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(root.join("config")).unwrap` | [842](../../src/process_live_tests.rs#L842) | receiver-type-required |
| `real_legacy_simple_task` | `root.join` | [842](../../src/process_live_tests.rs#L842), [843](../../src/process_live_tests.rs#L843), [845](../../src/process_live_tests.rs#L845), [846](../../src/process_live_tests.rs#L846), [847](../../src/process_live_tests.rs#L847), [849](../../src/process_live_tests.rs#L849), [891](../../src/process_live_tests.rs#L891), [899](../../src/process_live_tests.rs#L899), [907](../../src/process_live_tests.rs#L907), [909](../../src/process_live_tests.rs#L909), [940](../../src/process_live_tests.rs#L940), [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [843](../../src/process_live_tests.rs#L843) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [844](../../src/process_live_tests.rs#L844) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice` | [844](../../src/process_live_tests.rs#L844) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [844](../../src/process_live_tests.rs#L844) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read` | [844](../../src/process_live_tests.rs#L844) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [844](../../src/process_live_tests.rs#L844) | receiver-type-required |
| `real_legacy_simple_task` | `write_canonical_test_json` | [845](../../src/process_live_tests.rs#L845), [846](../../src/process_live_tests.rs#L846), [847](../../src/process_live_tests.rs#L847) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("threads").join` | [849](../../src/process_live_tests.rs#L849) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(folder.join("assets")).unwrap` | [850](../../src/process_live_tests.rs#L850) | receiver-type-required |
| `real_legacy_simple_task` | `folder.join` | [850](../../src/process_live_tests.rs#L850), [851](../../src/process_live_tests.rs#L851) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_genesis` | [852](../../src/process_live_tests.rs#L852) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `append_test_input` | [853](../../src/process_live_tests.rs#L853) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map` | [854](../../src/process_live_tests.rs#L854), [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines` | [854](../../src/process_live_tests.rs#L854), [921](../../src/process_live_tests.rs#L921), [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap` | [854](../../src/process_live_tests.rs#L854), [921](../../src/process_live_tests.rs#L921), [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string` | [854](../../src/process_live_tests.rs#L854), [921](../../src/process_live_tests.rs#L921), [925](../../src/process_live_tests.rs#L925), [951](../../src/process_live_tests.rs#L951), [960](../../src/process_live_tests.rs#L960) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `serde_json::from_str(line).unwrap` | [854](../../src/process_live_tests.rs#L854), [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str` | [854](../../src/process_live_tests.rs#L854), [921](../../src/process_live_tests.rs#L921), [925](../../src/process_live_tests.rs#L925), [951](../../src/process_live_tests.rs#L951) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_LEGACY_TASK_SCENARIO").unwrap_or_default` | [855](../../src/process_live_tests.rs#L855) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("src")).unwrap` | [859](../../src/process_live_tests.rs#L859) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("markdown")).unwrap` | [862](../../src/process_live_tests.rs#L862) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/1.md"), source).unwrap` | [864](../../src/process_live_tests.rs#L864) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write` | [864](../../src/process_live_tests.rs#L864), [865](../../src/process_live_tests.rs#L865), [866](../../src/process_live_tests.rs#L866), [870](../../src/process_live_tests.rs#L870), [871](../../src/process_live_tests.rs#L871), [872](../../src/process_live_tests.rs#L872), [907](../../src/process_live_tests.rs#L907), [986](../../src/process_live_tests.rs#L986) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/2.md"), "# Population\nBeijing has 21,000,000 people, based on markdown/1.md.\n").unwrap` | [865](../../src/process_live_tests.rs#L865) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/3.md"), "# People aged 60+\nBeijing has 4,200,000 people aged 60+, assuming 20% of the population in markdown/2.md.\n").unwrap` | [866](../../src/process_live_tests.rs#L866) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/1.py"), python_source).unwrap` | [870](../../src/process_live_tests.rs#L870) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/2.py"), "import importlib\ndef make_capital_sentence():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    return 'Beijing is the capital of China.'\n").unwrap` | [871](../../src/process_live_tests.rs#L871) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/3.py"), "import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    sentence = importlib.import_module('src.2').make_capital_sentence()\n    return 'China / Beijing: ' + sentence\n").unwrap` | [872](../../src/process_live_tests.rs#L872) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_events` | [888](../../src/process_live_tests.rs#L888) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Arc::new` | [889](../../src/process_live_tests.rs#L889), [899](../../src/process_live_tests.rs#L899), [915](../../src/process_live_tests.rs#L915) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `provider::MemorySecretStore::new` | [889](../../src/process_live_tests.rs#L889) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_legacy_simple_task` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `secrets.publish` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str().unwrap` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"live-task",root.join(".agent"),secrets).unwrap` | [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store` | [891](../../src/process_live_tests.rs#L891) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [892](../../src/process_live_tests.rs#L892) | receiver-type-required |
| `real_legacy_simple_task` | `Ok` | [892](../../src/process_live_tests.rs#L892), [899](../../src/process_live_tests.rs#L899) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new().unwrap` | [894](../../src/process_live_tests.rs#L894) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new` | [894](../../src/process_live_tests.rs#L894) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [895](../../src/process_live_tests.rs#L895) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on` | [895](../../src/process_live_tests.rs#L895), [948](../../src/process_live_tests.rs#L948) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::net::TcpListener::bind` | [895](../../src/process_live_tests.rs#L895) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `listener.local_addr().unwrap` | [896](../../src/process_live_tests.rs#L896) | receiver-type-required |
| `real_legacy_simple_task` | `listener.local_addr` | [896](../../src/process_live_tests.rs#L896) | receiver-type-required |
| `real_legacy_simple_task` | `crate::host_runtime::assemble_application_endpoint_host(&root,             endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},             Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [897](../../src/process_live_tests.rs#L897) | receiver-type-required |
| `real_legacy_simple_task` | `crate::host_runtime::assemble_application_endpoint_host` | [897](../../src/process_live_tests.rs#L897) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_legacy_simple_task` | `"test".into` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy().into_owned` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy().into_owned` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `"2026-09-04T10:00:00.000Z".into` | [899](../../src/process_live_tests.rs#L899) | receiver-type-required |
| `real_legacy_simple_task` | `host.clone` | [899](../../src/process_live_tests.rs#L899), [900](../../src/process_live_tests.rs#L900), [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),             transport::TransportConfig::loopback(address, transport::BearerToken::new([42;32]))).unwrap` | [900](../../src/process_live_tests.rs#L900) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [900](../../src/process_live_tests.rs#L900) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_legacy_simple_task` | `transport::TransportConfig::loopback` | [901](../../src/process_live_tests.rs#L901) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_legacy_simple_task` | `transport::BearerToken::new` | [901](../../src/process_live_tests.rs#L901) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_legacy_simple_task` | `host.attach_streams` | [902](../../src/process_live_tests.rs#L902) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams().clone` | [902](../../src/process_live_tests.rs#L902) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams` | [902](../../src/process_live_tests.rs#L902) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery().unwrap` | [903](../../src/process_live_tests.rs#L903) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery` | [903](../../src/process_live_tests.rs#L903) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.into_server` | [904](../../src/process_live_tests.rs#L904) | receiver-type-required |
| `real_legacy_simple_task` | `server.handle` | [905](../../src/process_live_tests.rs#L905) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.spawn` | [906](../../src/process_live_tests.rs#L906) | receiver-type-required |
| `real_legacy_simple_task` | `server.serve` | [906](../../src/process_live_tests.rs#L906) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [907](../../src/process_live_tests.rs#L907) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [907](../../src/process_live_tests.rs#L907) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec` | [907](../../src/process_live_tests.rs#L907) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Instant::now` | [908](../../src/process_live_tests.rs#L908), [919](../../src/process_live_tests.rs#L919) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_secs` | [908](../../src/process_live_tests.rs#L908), [919](../../src/process_live_tests.rs#L919) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("client-ready").exists` | [909](../../src/process_live_tests.rs#L909) | receiver-type-required |
| `real_legacy_simple_task` | `std::thread::sleep` | [911](../../src/process_live_tests.rs#L911), [942](../../src/process_live_tests.rs#L942) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_millis` | [911](../../src/process_live_tests.rs#L911), [942](../../src/process_live_tests.rs#L942) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Some` | [913](../../src/process_live_tests.rs#L913), [924](../../src/process_live_tests.rs#L924) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open(&root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [915](../../src/process_live_tests.rs#L915) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L177) |
| `real_legacy_simple_task` | `crate::endpoint_host::SessionAdmissionGates::default` | [915](../../src/process_live_tests.rs#L915) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::catch_unwind` | [916](../../src/process_live_tests.rs#L916) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::AssertUnwindSafe` | [916](../../src/process_live_tests.rs#L916) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap().expect` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_legacy_simple_task` | `host.start_periodic_sweep` | [918](../../src/process_live_tests.rs#L918) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [921](../../src/process_live_tests.rs#L921) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [921](../../src/process_live_tests.rs#L921) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str(line).ok` | [921](../../src/process_live_tests.rs#L921), [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir(&folder).unwrap` | [922](../../src/process_live_tests.rs#L922) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir` | [922](../../src/process_live_tests.rs#L922) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `entry.unwrap().path` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_legacy_simple_task` | `entry.unwrap` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension().and_then` | [924](../../src/process_live_tests.rs#L924) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension` | [924](../../src/process_live_tests.rs#L924) | receiver-type-required |
| `real_legacy_simple_task` | `v.to_str` | [924](../../src/process_live_tests.rs#L924) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name().unwrap` | [924](../../src/process_live_tests.rs#L924), [935](../../src/process_live_tests.rs#L935) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name` | [924](../../src/process_live_tests.rs#L924), [935](../../src/process_live_tests.rs#L935) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map` | [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines` | [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap` | [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().filter` | [926](../../src/process_live_tests.rs#L926) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter` | [926](../../src/process_live_tests.rs#L926), [927](../../src/process_live_tests.rs#L927), [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().any` | [927](../../src/process_live_tests.rs#L927) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["call"]==held["call"]).unwrap` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str().unwrap` | [931](../../src/process_live_tests.rs#L931) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str` | [931](../../src/process_live_tests.rs#L931) | receiver-type-required |
| `real_legacy_simple_task` | `raw.is_absolute` | [932](../../src/process_live_tests.rs#L932) | receiver-type-required |
| `real_legacy_simple_task` | `session.to_owned` | [935](../../src/process_live_tests.rs#L935) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author(&responder,endpoint::RespondAuthorization {rpc_id:key.clone(),request_sha256:"live-task-approval".into(),session_id:target,call:held["call"].as_str().unwrap().into(),grant:true,answer:None,origin_key:key,resolution:endpoint::ResolutionOutcome::AllowedOnce}).expect` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author` | [937](../../src/process_live_tests.rs#L937) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `key.clone` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `"live-task-approval".into` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap().into` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter().any` | [940](../../src/process_live_tests.rs#L940) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter` | [940](../../src/process_live_tests.rs#L940) | receiver-type-required |
| `real_legacy_simple_task` | `root.join("client-task-receipt.json").exists` | [940](../../src/process_live_tests.rs#L940) | receiver-type-required |
| `real_legacy_simple_task` | `host.shutdown` | [945](../../src/process_live_tests.rs#L945) | receiver-type-required |
| `real_legacy_simple_task` | `handle.begin_drain` | [947](../../src/process_live_tests.rs#L947) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap().unwrap` | [948](../../src/process_live_tests.rs#L948) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap` | [948](../../src/process_live_tests.rs#L948) | receiver-type-required |
| `real_legacy_simple_task` | `std::panic::resume_unwind` | [950](../../src/process_live_tests.rs#L950) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::Map::new` | [958](../../src/process_live_tests.rs#L958) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(workspace.join(relative)).expect` | [960](../../src/process_live_tests.rs#L960) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase().replace` | [970](../../src/process_live_tests.rs#L970) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase` | [970](../../src/process_live_tests.rs#L970) | receiver-type-required |
| `real_legacy_simple_task` | `artifacts.insert` | [980](../../src/process_live_tests.rs#L980) | receiver-type-required |
| `real_legacy_simple_task` | `(*relative).into` | [980](../../src/process_live_tests.rs#L980) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap()).unwrap` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty` | [986](../../src/process_live_tests.rs#L986) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PUBLIC_QUESTION").is_ok_and` | [993](../../src/process_live_tests.rs#L993) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var` | [993](../../src/process_live_tests.rs#L993), [994](../../src/process_live_tests.rs#L994), [999](../../src/process_live_tests.rs#L999), [1008](../../src/process_live_tests.rs#L1008), [1009](../../src/process_live_tests.rs#L1009) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `PathBuf::from` | [994](../../src/process_live_tests.rs#L994) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [994](../../src/process_live_tests.rs#L994) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap().join` | [995](../../src/process_live_tests.rs#L995) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap` | [995](../../src/process_live_tests.rs#L995) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent` | [995](../../src/process_live_tests.rs#L995) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(&workspace).unwrap` | [996](../../src/process_live_tests.rs#L996) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all` | [996](../../src/process_live_tests.rs#L996), [997](../../src/process_live_tests.rs#L997), [998](../../src/process_live_tests.rs#L998), [1004](../../src/process_live_tests.rs#L1004) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(root.join("config")).unwrap` | [997](../../src/process_live_tests.rs#L997) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join` | [997](../../src/process_live_tests.rs#L997), [998](../../src/process_live_tests.rs#L998), [1000](../../src/process_live_tests.rs#L1000), [1001](../../src/process_live_tests.rs#L1001), [1003](../../src/process_live_tests.rs#L1003), [1009](../../src/process_live_tests.rs#L1009), [1015](../../src/process_live_tests.rs#L1015), [1023](../../src/process_live_tests.rs#L1023), [1030](../../src/process_live_tests.rs#L1030), [1032](../../src/process_live_tests.rs#L1032), [1069](../../src/process_live_tests.rs#L1069) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice` | [999](../../src/process_live_tests.rs#L999) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read` | [999](../../src/process_live_tests.rs#L999) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_canonical_test_json` | [1000](../../src/process_live_tests.rs#L1000), [1001](../../src/process_live_tests.rs#L1001) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("threads").join` | [1003](../../src/process_live_tests.rs#L1003) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `folder.join` | [1004](../../src/process_live_tests.rs#L1004), [1005](../../src/process_live_tests.rs#L1005) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_test_genesis` | [1006](../../src/process_live_tests.rs#L1006) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Arc::new` | [1007](../../src/process_live_tests.rs#L1007), [1015](../../src/process_live_tests.rs#L1015) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `provider::MemorySecretStore::new` | [1007](../../src/process_live_tests.rs#L1007) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str().unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap` | [1009](../../src/process_live_tests.rs#L1009) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store` | [1009](../../src/process_live_tests.rs#L1009) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1009](../../src/process_live_tests.rs#L1009) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new().unwrap` | [1010](../../src/process_live_tests.rs#L1010) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new` | [1010](../../src/process_live_tests.rs#L1010) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1011](../../src/process_live_tests.rs#L1011) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on` | [1011](../../src/process_live_tests.rs#L1011), [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::net::TcpListener::bind` | [1011](../../src/process_live_tests.rs#L1011) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr().unwrap` | [1012](../../src/process_live_tests.rs#L1012) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr` | [1012](../../src/process_live_tests.rs#L1012) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1013](../../src/process_live_tests.rs#L1013) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::host_runtime::assemble_application_endpoint_host` | [1013](../../src/process_live_tests.rs#L1013) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_provider_400_releases_queue_over_public_transport` | `"test".into` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy().into_owned` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy().into_owned` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `Ok` | [1015](../../src/process_live_tests.rs#L1015) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `"2026-09-04T10:00:00.000Z".into` | [1015](../../src/process_live_tests.rs#L1015) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `host.clone` | [1015](../../src/process_live_tests.rs#L1015), [1016](../../src/process_live_tests.rs#L1016) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1016](../../src/process_live_tests.rs#L1016) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1016](../../src/process_live_tests.rs#L1016) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::TransportConfig::loopback` | [1017](../../src/process_live_tests.rs#L1017) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::BearerToken::new` | [1017](../../src/process_live_tests.rs#L1017) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_provider_400_releases_queue_over_public_transport` | `host.attach_streams` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams().clone` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery().unwrap` | [1019](../../src/process_live_tests.rs#L1019) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery` | [1019](../../src/process_live_tests.rs#L1019) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.into_server` | [1020](../../src/process_live_tests.rs#L1020) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.handle` | [1021](../../src/process_live_tests.rs#L1021) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.spawn` | [1022](../../src/process_live_tests.rs#L1022) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.serve` | [1022](../../src/process_live_tests.rs#L1022) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1023](../../src/process_live_tests.rs#L1023) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write` | [1023](../../src/process_live_tests.rs#L1023), [1030](../../src/process_live_tests.rs#L1030), [1069](../../src/process_live_tests.rs#L1069) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1023](../../src/process_live_tests.rs#L1023) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec` | [1023](../../src/process_live_tests.rs#L1023) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.start_periodic_sweep` | [1024](../../src/process_live_tests.rs#L1024) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::catch_unwind` | [1025](../../src/process_live_tests.rs#L1025) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::AssertUnwindSafe` | [1025](../../src/process_live_tests.rs#L1025) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Instant::now` | [1026](../../src/process_live_tests.rs#L1026) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_secs` | [1026](../../src/process_live_tests.rs#L1026) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [1028](../../src/process_live_tests.rs#L1028) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [1028](../../src/process_live_tests.rs#L1028) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines` | [1028](../../src/process_live_tests.rs#L1028), [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap` | [1028](../../src/process_live_tests.rs#L1028), [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string` | [1028](../../src/process_live_tests.rs#L1028), [1041](../../src/process_live_tests.rs#L1041) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).ok` | [1028](../../src/process_live_tests.rs#L1028) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str` | [1028](../../src/process_live_tests.rs#L1028), [1041](../../src/process_live_tests.rs#L1041) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").count` | [1029](../../src/process_live_tests.rs#L1029) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter` | [1029](../../src/process_live_tests.rs#L1029), [1042](../../src/process_live_tests.rs#L1042), [1046](../../src/process_live_tests.rs#L1046), [1049](../../src/process_live_tests.rs#L1049), [1061](../../src/process_live_tests.rs#L1061) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter` | [1029](../../src/process_live_tests.rs#L1029), [1042](../../src/process_live_tests.rs#L1042), [1046](../../src/process_live_tests.rs#L1046), [1049](../../src/process_live_tests.rs#L1049), [1056](../../src/process_live_tests.rs#L1056), [1060](../../src/process_live_tests.rs#L1060), [1061](../../src/process_live_tests.rs#L1061), [1066](../../src/process_live_tests.rs#L1066) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("queued-input-observed"),"durable").unwrap` | [1030](../../src/process_live_tests.rs#L1030) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("client-error-queue-receipt.json").exists` | [1032](../../src/process_live_tests.rs#L1032) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::thread::sleep` | [1034](../../src/process_live_tests.rs#L1034) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_millis` | [1034](../../src/process_live_tests.rs#L1034) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.shutdown` | [1037](../../src/process_live_tests.rs#L1037) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `handle.begin_drain` | [1038](../../src/process_live_tests.rs#L1038) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap().unwrap` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::resume_unwind` | [1040](../../src/process_live_tests.rs#L1040) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map` | [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).unwrap` | [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1042](../../src/process_live_tests.rs#L1042) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").collect` | [1046](../../src/process_live_tests.rs#L1046) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="turn_open").collect` | [1049](../../src/process_live_tests.rs#L1049) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="think").unwrap` | [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find` | [1056](../../src/process_live_tests.rs#L1056), [1060](../../src/process_live_tests.rs#L1060), [1066](../../src/process_live_tests.rs#L1066) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="ask_user_questions").unwrap` | [1060](../../src/process_live_tests.rs#L1060) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="approval_response" && e["call"]==question_call["call"]).collect` | [1061](../../src/process_live_tests.rs#L1061) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="output" && e["final_answer"]==true).unwrap` | [1066](../../src/process_live_tests.rs#L1066) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap()).unwrap` | [1069](../../src/process_live_tests.rs#L1069) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap` | [1069](../../src/process_live_tests.rs#L1069) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty` | [1069](../../src/process_live_tests.rs#L1069) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect` | [1077](../../src/process_live_tests.rs#L1077) | receiver-type-required |
| `register_local_tools_server` | `std::env::var` | [1077](../../src/process_live_tests.rs#L1077) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `fs::create_dir_all(root.join("config")).unwrap` | [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `register_local_tools_server` | `fs::create_dir_all` | [1078](../../src/process_live_tests.rs#L1078) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `root.join` | [1078](../../src/process_live_tests.rs#L1078), [1079](../../src/process_live_tests.rs#L1079) | receiver-type-required |
| `register_local_tools_server` | `mcp::McpRegistryStore::new` | [1079](../../src/process_live_tests.rs#L1079) | [mcp::management::McpRegistryStore::new](../../../mcp/src/management.rs#L391) |
| `register_local_tools_server` | `registry.mutate_idempotent("save-local-tools", &mcp::McpManagementMutation::Save {         server: mcp::McpServerConfig {             folder_id: None,             reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "local".to_owned() },             transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "local-tools".to_owned()], cwd: None, environment: BTreeMap::new() },             enabled: true,             always_on,             protocol_mode: mcp::ProtocolMode::Legacy,             owner: None,             plugin_component: None,             project_trusted: true,         },         credential_fields: BTreeMap::new(),     }).expect` | [1080](../../src/process_live_tests.rs#L1080) | receiver-type-required |
| `register_local_tools_server` | `registry.mutate_idempotent` | [1080](../../src/process_live_tests.rs#L1080) | receiver-type-required |
| `register_local_tools_server` | `"ws".to_owned` | [1083](../../src/process_live_tests.rs#L1083) | receiver-type-required |
| `register_local_tools_server` | `"local".to_owned` | [1083](../../src/process_live_tests.rs#L1083) | receiver-type-required |
| `register_local_tools_server` | `BTreeMap::new` | [1084](../../src/process_live_tests.rs#L1084), [1092](../../src/process_live_tests.rs#L1092) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `PathBuf::from` | [1100](../../src/process_live_tests.rs#L1100) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1100](../../src/process_live_tests.rs#L1100) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var` | [1100](../../src/process_live_tests.rs#L1100), [1105](../../src/process_live_tests.rs#L1105), [1110](../../src/process_live_tests.rs#L1110), [1130](../../src/process_live_tests.rs#L1130), [1131](../../src/process_live_tests.rs#L1131) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.parent().unwrap().join` | [1101](../../src/process_live_tests.rs#L1101) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent().unwrap` | [1101](../../src/process_live_tests.rs#L1101) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent` | [1101](../../src/process_live_tests.rs#L1101) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(&workspace).unwrap` | [1102](../../src/process_live_tests.rs#L1102) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all` | [1102](../../src/process_live_tests.rs#L1102), [1103](../../src/process_live_tests.rs#L1103), [1104](../../src/process_live_tests.rs#L1104), [1126](../../src/process_live_tests.rs#L1126) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::create_dir_all(root.join("config")).unwrap` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.join` | [1103](../../src/process_live_tests.rs#L1103), [1104](../../src/process_live_tests.rs#L1104), [1106](../../src/process_live_tests.rs#L1106), [1122](../../src/process_live_tests.rs#L1122), [1125](../../src/process_live_tests.rs#L1125), [1131](../../src/process_live_tests.rs#L1131), [1137](../../src/process_live_tests.rs#L1137), [1145](../../src/process_live_tests.rs#L1145), [1150](../../src/process_live_tests.rs#L1150), [1208](../../src/process_live_tests.rs#L1208) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice` | [1105](../../src/process_live_tests.rs#L1105) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read` | [1105](../../src/process_live_tests.rs#L1105) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_canonical_test_json` | [1106](../../src/process_live_tests.rs#L1106), [1122](../../src/process_live_tests.rs#L1122) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TOOL_SEARCH_SCENARIO").is_ok_and` | [1110](../../src/process_live_tests.rs#L1110) | receiver-type-required |
| `real_public_deferred_tool_search` | `register_local_tools_server` | [1123](../../src/process_live_tests.rs#L1123) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1076) |
| `real_public_deferred_tool_search` | `root.join("threads").join` | [1125](../../src/process_live_tests.rs#L1125) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1126](../../src/process_live_tests.rs#L1126) | receiver-type-required |
| `real_public_deferred_tool_search` | `folder.join` | [1126](../../src/process_live_tests.rs#L1126), [1127](../../src/process_live_tests.rs#L1127) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_test_genesis` | [1128](../../src/process_live_tests.rs#L1128) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Arc::new` | [1129](../../src/process_live_tests.rs#L1129), [1137](../../src/process_live_tests.rs#L1137) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider::MemorySecretStore::new` | [1129](../../src/process_live_tests.rs#L1129) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_deferred_tool_search` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1130](../../src/process_live_tests.rs#L1130) | receiver-type-required |
| `real_public_deferred_tool_search` | `secrets.publish` | [1130](../../src/process_live_tests.rs#L1130) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str().unwrap` | [1130](../../src/process_live_tests.rs#L1130) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str` | [1130](../../src/process_live_tests.rs#L1130) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1130](../../src/process_live_tests.rs#L1130) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-deferred-tool",root.join(".agent"),secrets).unwrap` | [1131](../../src/process_live_tests.rs#L1131) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store` | [1131](../../src/process_live_tests.rs#L1131) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1131](../../src/process_live_tests.rs#L1131) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new().unwrap` | [1132](../../src/process_live_tests.rs#L1132) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new` | [1132](../../src/process_live_tests.rs#L1132) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1133](../../src/process_live_tests.rs#L1133) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on` | [1133](../../src/process_live_tests.rs#L1133), [1162](../../src/process_live_tests.rs#L1162) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::net::TcpListener::bind` | [1133](../../src/process_live_tests.rs#L1133) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `listener.local_addr().unwrap` | [1134](../../src/process_live_tests.rs#L1134) | receiver-type-required |
| `real_public_deferred_tool_search` | `listener.local_addr` | [1134](../../src/process_live_tests.rs#L1134) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1135](../../src/process_live_tests.rs#L1135) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::host_runtime::assemble_application_endpoint_host` | [1135](../../src/process_live_tests.rs#L1135) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_public_deferred_tool_search` | `"test".into` | [1136](../../src/process_live_tests.rs#L1136) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy().into_owned` | [1136](../../src/process_live_tests.rs#L1136) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy` | [1136](../../src/process_live_tests.rs#L1136) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy().into_owned` | [1136](../../src/process_live_tests.rs#L1136) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy` | [1136](../../src/process_live_tests.rs#L1136) | receiver-type-required |
| `real_public_deferred_tool_search` | `Ok` | [1137](../../src/process_live_tests.rs#L1137) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `"2026-09-04T10:00:00.000Z".into` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.clone` | [1137](../../src/process_live_tests.rs#L1137), [1138](../../src/process_live_tests.rs#L1138) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1138](../../src/process_live_tests.rs#L1138) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1138](../../src/process_live_tests.rs#L1138) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_public_deferred_tool_search` | `transport::TransportConfig::loopback` | [1139](../../src/process_live_tests.rs#L1139) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_deferred_tool_search` | `transport::BearerToken::new` | [1139](../../src/process_live_tests.rs#L1139) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_deferred_tool_search` | `host.attach_streams` | [1140](../../src/process_live_tests.rs#L1140) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams().clone` | [1140](../../src/process_live_tests.rs#L1140) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams` | [1140](../../src/process_live_tests.rs#L1140) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery().unwrap` | [1141](../../src/process_live_tests.rs#L1141) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery` | [1141](../../src/process_live_tests.rs#L1141) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.into_server` | [1142](../../src/process_live_tests.rs#L1142) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.handle` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.spawn` | [1144](../../src/process_live_tests.rs#L1144) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.serve` | [1144](../../src/process_live_tests.rs#L1144) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1145](../../src/process_live_tests.rs#L1145) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write` | [1145](../../src/process_live_tests.rs#L1145), [1208](../../src/process_live_tests.rs#L1208) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1145](../../src/process_live_tests.rs#L1145) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec` | [1145](../../src/process_live_tests.rs#L1145) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.start_periodic_sweep` | [1146](../../src/process_live_tests.rs#L1146) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::catch_unwind` | [1147](../../src/process_live_tests.rs#L1147) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::panic::AssertUnwindSafe` | [1147](../../src/process_live_tests.rs#L1147) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Instant::now` | [1148](../../src/process_live_tests.rs#L1148) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_secs` | [1148](../../src/process_live_tests.rs#L1148) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.join("client-tool-search-receipt.json").exists` | [1150](../../src/process_live_tests.rs#L1150) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().count` | [1152](../../src/process_live_tests.rs#L1152) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines` | [1152](../../src/process_live_tests.rs#L1152), [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap` | [1152](../../src/process_live_tests.rs#L1152), [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string` | [1152](../../src/process_live_tests.rs#L1152), [1164](../../src/process_live_tests.rs#L1164) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.ensure_running(session).expect` | [1153](../../src/process_live_tests.rs#L1153) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.ensure_running` | [1153](../../src/process_live_tests.rs#L1153) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::thread::sleep` | [1157](../../src/process_live_tests.rs#L1157) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_millis` | [1157](../../src/process_live_tests.rs#L1157) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.shutdown` | [1160](../../src/process_live_tests.rs#L1160) | receiver-type-required |
| `real_public_deferred_tool_search` | `handle.begin_drain` | [1161](../../src/process_live_tests.rs#L1161) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap().unwrap` | [1162](../../src/process_live_tests.rs#L1162) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap` | [1162](../../src/process_live_tests.rs#L1162) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::resume_unwind` | [1163](../../src/process_live_tests.rs#L1163) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(line).unwrap` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str` | [1164](../../src/process_live_tests.rs#L1164), [1178](../../src/process_live_tests.rs#L1178) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1165](../../src/process_live_tests.rs#L1165) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter` | [1165](../../src/process_live_tests.rs#L1165), [1183](../../src/process_live_tests.rs#L1183), [1201](../../src/process_live_tests.rs#L1201) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter` | [1165](../../src/process_live_tests.rs#L1165), [1170](../../src/process_live_tests.rs#L1170), [1171](../../src/process_live_tests.rs#L1171), [1172](../../src/process_live_tests.rs#L1172), [1175](../../src/process_live_tests.rs#L1175), [1183](../../src/process_live_tests.rs#L1183), [1201](../../src/process_live_tests.rs#L1201) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from(turn).unwrap` | [1168](../../src/process_live_tests.rs#L1168) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from` | [1168](../../src/process_live_tests.rs#L1168) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="tool_search" && e["turn"]==turn).expect` | [1170](../../src/process_live_tests.rs#L1170) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find` | [1170](../../src/process_live_tests.rs#L1170), [1171](../../src/process_live_tests.rs#L1171), [1172](../../src/process_live_tests.rs#L1172), [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==search["call"] && e["outcome"]=="ok").unwrap` | [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]==deferred_tool && e["turn"]==turn).expect` | [1172](../../src/process_live_tests.rs#L1172) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==call["call"] && e["outcome"]=="ok").unwrap` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(output["content"][0]["text"].as_str().unwrap()).unwrap` | [1178](../../src/process_live_tests.rs#L1178) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str().unwrap` | [1178](../../src/process_live_tests.rs#L1178) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str` | [1178](../../src/process_live_tests.rs#L1178) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str().expect` | [1180](../../src/process_live_tests.rs#L1180) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str` | [1180](../../src/process_live_tests.rs#L1180) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="epoch").collect` | [1183](../../src/process_live_tests.rs#L1183) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value(provider.clone()).unwrap` | [1190](../../src/process_live_tests.rs#L1190) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value` | [1190](../../src/process_live_tests.rs#L1190) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider.clone` | [1190](../../src/process_live_tests.rs#L1190) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools().is_some` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile` | [1191](../../src/process_live_tests.rs#L1191) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap().to_owned` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find(&#124;e&#124;e["id"]==attempt["epoch"]).unwrap` | [1202](../../src/process_live_tests.rs#L1202) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find` | [1202](../../src/process_live_tests.rs#L1202) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter` | [1202](../../src/process_live_tests.rs#L1202) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap()).unwrap` | [1208](../../src/process_live_tests.rs#L1208) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap` | [1208](../../src/process_live_tests.rs#L1208) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty` | [1208](../../src/process_live_tests.rs#L1208) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `PathBuf::from` | [1215](../../src/process_live_tests.rs#L1215) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1215](../../src/process_live_tests.rs#L1215) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var` | [1215](../../src/process_live_tests.rs#L1215), [1220](../../src/process_live_tests.rs#L1220), [1234](../../src/process_live_tests.rs#L1234), [1235](../../src/process_live_tests.rs#L1235) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.parent().unwrap().join` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `root.parent().unwrap` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `root.parent` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(&workspace).unwrap` | [1217](../../src/process_live_tests.rs#L1217) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all` | [1217](../../src/process_live_tests.rs#L1217), [1218](../../src/process_live_tests.rs#L1218), [1219](../../src/process_live_tests.rs#L1219), [1230](../../src/process_live_tests.rs#L1230) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::create_dir_all(root.join("config")).unwrap` | [1218](../../src/process_live_tests.rs#L1218) | receiver-type-required |
| `real_public_image_attachment` | `root.join` | [1218](../../src/process_live_tests.rs#L1218), [1219](../../src/process_live_tests.rs#L1219), [1221](../../src/process_live_tests.rs#L1221), [1227](../../src/process_live_tests.rs#L1227), [1229](../../src/process_live_tests.rs#L1229), [1235](../../src/process_live_tests.rs#L1235), [1241](../../src/process_live_tests.rs#L1241), [1249](../../src/process_live_tests.rs#L1249), [1254](../../src/process_live_tests.rs#L1254), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1219](../../src/process_live_tests.rs#L1219) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1220](../../src/process_live_tests.rs#L1220) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice` | [1220](../../src/process_live_tests.rs#L1220) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1220](../../src/process_live_tests.rs#L1220) | receiver-type-required |
| `real_public_image_attachment` | `fs::read` | [1220](../../src/process_live_tests.rs#L1220) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1220](../../src/process_live_tests.rs#L1220) | receiver-type-required |
| `real_public_image_attachment` | `write_canonical_test_json` | [1221](../../src/process_live_tests.rs#L1221), [1227](../../src/process_live_tests.rs#L1227) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map(&#124;tool&#124; tool.name).collect` | [1224](../../src/process_live_tests.rs#L1224) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map` | [1224](../../src/process_live_tests.rs#L1224) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter` | [1224](../../src/process_live_tests.rs#L1224) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter` | [1224](../../src/process_live_tests.rs#L1224) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled` | [1224](../../src/process_live_tests.rs#L1224) | [tools::builtin::BuiltinManifest::compiled](../../../tools/src/builtin.rs#L315) |
| `real_public_image_attachment` | `root.join("threads").join` | [1229](../../src/process_live_tests.rs#L1229) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1230](../../src/process_live_tests.rs#L1230) | receiver-type-required |
| `real_public_image_attachment` | `folder.join` | [1230](../../src/process_live_tests.rs#L1230), [1231](../../src/process_live_tests.rs#L1231) | receiver-type-required |
| `real_public_image_attachment` | `write_test_genesis` | [1232](../../src/process_live_tests.rs#L1232) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Arc::new` | [1233](../../src/process_live_tests.rs#L1233), [1241](../../src/process_live_tests.rs#L1241) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `provider::MemorySecretStore::new` | [1233](../../src/process_live_tests.rs#L1233) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_image_attachment` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1234](../../src/process_live_tests.rs#L1234) | receiver-type-required |
| `real_public_image_attachment` | `secrets.publish` | [1234](../../src/process_live_tests.rs#L1234) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str().unwrap` | [1234](../../src/process_live_tests.rs#L1234) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str` | [1234](../../src/process_live_tests.rs#L1234) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1234](../../src/process_live_tests.rs#L1234) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-image",root.join(".agent"),secrets).unwrap` | [1235](../../src/process_live_tests.rs#L1235) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store` | [1235](../../src/process_live_tests.rs#L1235) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1235](../../src/process_live_tests.rs#L1235) | receiver-type-required |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new().unwrap` | [1236](../../src/process_live_tests.rs#L1236) | receiver-type-required |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new` | [1236](../../src/process_live_tests.rs#L1236) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1237](../../src/process_live_tests.rs#L1237) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on` | [1237](../../src/process_live_tests.rs#L1237), [1266](../../src/process_live_tests.rs#L1266) | receiver-type-required |
| `real_public_image_attachment` | `tokio::net::TcpListener::bind` | [1237](../../src/process_live_tests.rs#L1237) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `listener.local_addr().unwrap` | [1238](../../src/process_live_tests.rs#L1238) | receiver-type-required |
| `real_public_image_attachment` | `listener.local_addr` | [1238](../../src/process_live_tests.rs#L1238) | receiver-type-required |
| `real_public_image_attachment` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1239](../../src/process_live_tests.rs#L1239) | receiver-type-required |
| `real_public_image_attachment` | `crate::host_runtime::assemble_application_endpoint_host` | [1239](../../src/process_live_tests.rs#L1239) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_public_image_attachment` | `"test".into` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy().into_owned` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy().into_owned` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `Ok` | [1241](../../src/process_live_tests.rs#L1241) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `"2026-09-04T10:00:00.000Z".into` | [1241](../../src/process_live_tests.rs#L1241) | receiver-type-required |
| `real_public_image_attachment` | `host.clone` | [1241](../../src/process_live_tests.rs#L1241), [1242](../../src/process_live_tests.rs#L1242) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1242](../../src/process_live_tests.rs#L1242) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1242](../../src/process_live_tests.rs#L1242) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_public_image_attachment` | `transport::TransportConfig::loopback` | [1243](../../src/process_live_tests.rs#L1243) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_image_attachment` | `transport::BearerToken::new` | [1243](../../src/process_live_tests.rs#L1243) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_image_attachment` | `host.attach_streams` | [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams().clone` | [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams` | [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery().unwrap` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `assembly.into_server` | [1246](../../src/process_live_tests.rs#L1246) | receiver-type-required |
| `real_public_image_attachment` | `server.handle` | [1247](../../src/process_live_tests.rs#L1247) | receiver-type-required |
| `real_public_image_attachment` | `runtime.spawn` | [1248](../../src/process_live_tests.rs#L1248) | receiver-type-required |
| `real_public_image_attachment` | `server.serve` | [1248](../../src/process_live_tests.rs#L1248) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1249](../../src/process_live_tests.rs#L1249) | receiver-type-required |
| `real_public_image_attachment` | `fs::write` | [1249](../../src/process_live_tests.rs#L1249), [1281](../../src/process_live_tests.rs#L1281) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1249](../../src/process_live_tests.rs#L1249) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec` | [1249](../../src/process_live_tests.rs#L1249) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.start_periodic_sweep` | [1250](../../src/process_live_tests.rs#L1250) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::catch_unwind` | [1251](../../src/process_live_tests.rs#L1251) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::panic::AssertUnwindSafe` | [1251](../../src/process_live_tests.rs#L1251) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Instant::now` | [1252](../../src/process_live_tests.rs#L1252) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_secs` | [1252](../../src/process_live_tests.rs#L1252) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.join("client-image-receipt.json").exists` | [1254](../../src/process_live_tests.rs#L1254) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().count` | [1256](../../src/process_live_tests.rs#L1256) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines` | [1256](../../src/process_live_tests.rs#L1256), [1268](../../src/process_live_tests.rs#L1268) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap` | [1256](../../src/process_live_tests.rs#L1256), [1268](../../src/process_live_tests.rs#L1268) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string` | [1256](../../src/process_live_tests.rs#L1256), [1268](../../src/process_live_tests.rs#L1268) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.ensure_running(session).expect` | [1257](../../src/process_live_tests.rs#L1257) | receiver-type-required |
| `real_public_image_attachment` | `host.ensure_running` | [1257](../../src/process_live_tests.rs#L1257) | receiver-type-required |
| `real_public_image_attachment` | `std::thread::sleep` | [1261](../../src/process_live_tests.rs#L1261) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_millis` | [1261](../../src/process_live_tests.rs#L1261) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.shutdown` | [1264](../../src/process_live_tests.rs#L1264) | receiver-type-required |
| `real_public_image_attachment` | `handle.begin_drain` | [1265](../../src/process_live_tests.rs#L1265) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap().unwrap` | [1266](../../src/process_live_tests.rs#L1266) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap` | [1266](../../src/process_live_tests.rs#L1266) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::resume_unwind` | [1267](../../src/process_live_tests.rs#L1267) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1268](../../src/process_live_tests.rs#L1268) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map` | [1268](../../src/process_live_tests.rs#L1268) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str(line).unwrap` | [1268](../../src/process_live_tests.rs#L1268) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str` | [1268](../../src/process_live_tests.rs#L1268) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1269](../../src/process_live_tests.rs#L1269) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().filter` | [1269](../../src/process_live_tests.rs#L1269) | receiver-type-required |
| `real_public_image_attachment` | `events.iter` | [1269](../../src/process_live_tests.rs#L1269), [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find(&#124;e&#124; e["kind"]=="input").expect` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find(&#124;b&#124; b["type"]=="image").expect` | [1273](../../src/process_live_tests.rs#L1273) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find` | [1273](../../src/process_live_tests.rs#L1273) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter` | [1273](../../src/process_live_tests.rs#L1273) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap` | [1273](../../src/process_live_tests.rs#L1273) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array` | [1273](../../src/process_live_tests.rs#L1273) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAS0lEQVR42u3PQQkAAAgAsetfWiP4FgYrsKZeS0BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEDgsqnc8OJg6Ln3AAAAAElFTkSuQmCC").unwrap` | [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode` | [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str().unwrap` | [1278](../../src/process_live_tests.rs#L1278) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str` | [1278](../../src/process_live_tests.rs#L1278) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap()).unwrap` | [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap` | [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty` | [1281](../../src/process_live_tests.rs#L1281) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap()).unwrap` | [1288](../../src/process_live_tests.rs#L1288) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice` | [1288](../../src/process_live_tests.rs#L1288), [1294](../../src/process_live_tests.rs#L1294) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap` | [1288](../../src/process_live_tests.rs#L1288) | receiver-type-required |
| `real_public_flow_case` | `fs::read` | [1288](../../src/process_live_tests.rs#L1288), [1294](../../src/process_live_tests.rs#L1294), [1431](../../src/process_live_tests.rs#L1431), [1445](../../src/process_live_tests.rs#L1445) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_FLOW_CASE").unwrap` | [1288](../../src/process_live_tests.rs#L1288) | receiver-type-required |
| `real_public_flow_case` | `std::env::var` | [1288](../../src/process_live_tests.rs#L1288), [1289](../../src/process_live_tests.rs#L1289), [1294](../../src/process_live_tests.rs#L1294), [1335](../../src/process_live_tests.rs#L1335), [1336](../../src/process_live_tests.rs#L1336), [1378](../../src/process_live_tests.rs#L1378), [1379](../../src/process_live_tests.rs#L1379) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `PathBuf::from` | [1289](../../src/process_live_tests.rs#L1289) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1289](../../src/process_live_tests.rs#L1289) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap().join` | [1290](../../src/process_live_tests.rs#L1290) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap` | [1290](../../src/process_live_tests.rs#L1290) | receiver-type-required |
| `real_public_flow_case` | `root.parent` | [1290](../../src/process_live_tests.rs#L1290) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(&workspace).unwrap` | [1291](../../src/process_live_tests.rs#L1291) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all` | [1291](../../src/process_live_tests.rs#L1291), [1292](../../src/process_live_tests.rs#L1292), [1293](../../src/process_live_tests.rs#L1293), [1306](../../src/process_live_tests.rs#L1306), [1315](../../src/process_live_tests.rs#L1315), [1323](../../src/process_live_tests.rs#L1323), [1330](../../src/process_live_tests.rs#L1330), [1373](../../src/process_live_tests.rs#L1373) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::create_dir_all(root.join("config")).unwrap` | [1292](../../src/process_live_tests.rs#L1292) | receiver-type-required |
| `real_public_flow_case` | `root.join` | [1292](../../src/process_live_tests.rs#L1292), [1293](../../src/process_live_tests.rs#L1293), [1295](../../src/process_live_tests.rs#L1295), [1303](../../src/process_live_tests.rs#L1303), [1314](../../src/process_live_tests.rs#L1314), [1323](../../src/process_live_tests.rs#L1323), [1324](../../src/process_live_tests.rs#L1324), [1330](../../src/process_live_tests.rs#L1330), [1331](../../src/process_live_tests.rs#L1331), [1336](../../src/process_live_tests.rs#L1336), [1342](../../src/process_live_tests.rs#L1342), [1350](../../src/process_live_tests.rs#L1350), [1354](../../src/process_live_tests.rs#L1354), [1373](../../src/process_live_tests.rs#L1373), [1374](../../src/process_live_tests.rs#L1374), [1379](../../src/process_live_tests.rs#L1379), [1385](../../src/process_live_tests.rs#L1385), [1393](../../src/process_live_tests.rs#L1393), [1397](../../src/process_live_tests.rs#L1397), [1433](../../src/process_live_tests.rs#L1433), [1451](../../src/process_live_tests.rs#L1451), [1456](../../src/process_live_tests.rs#L1456) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1293](../../src/process_live_tests.rs#L1293) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1294](../../src/process_live_tests.rs#L1294) | receiver-type-required |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1294](../../src/process_live_tests.rs#L1294) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1294](../../src/process_live_tests.rs#L1294) | receiver-type-required |
| `real_public_flow_case` | `write_canonical_test_json` | [1295](../../src/process_live_tests.rs#L1295), [1303](../../src/process_live_tests.rs#L1303), [1319](../../src/process_live_tests.rs#L1319), [1329](../../src/process_live_tests.rs#L1329), [1372](../../src/process_live_tests.rs#L1372) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["max_wall_seconds"].as_u64` | [1300](../../src/process_live_tests.rs#L1300) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect("policy object").insert` | [1301](../../src/process_live_tests.rs#L1301) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect` | [1301](../../src/process_live_tests.rs#L1301) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut` | [1301](../../src/process_live_tests.rs#L1301) | receiver-type-required |
| `real_public_flow_case` | `"max_wall_seconds".to_owned` | [1301](../../src/process_live_tests.rs#L1301) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"].as_object().unwrap` | [1304](../../src/process_live_tests.rs#L1304) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"].as_object` | [1304](../../src/process_live_tests.rs#L1304) | receiver-type-required |
| `real_public_flow_case` | `workspace.join` | [1305](../../src/process_live_tests.rs#L1305) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(target.parent().unwrap()).unwrap` | [1306](../../src/process_live_tests.rs#L1306) | receiver-type-required |
| `real_public_flow_case` | `target.parent().unwrap` | [1306](../../src/process_live_tests.rs#L1306) | receiver-type-required |
| `real_public_flow_case` | `target.parent` | [1306](../../src/process_live_tests.rs#L1306) | receiver-type-required |
| `real_public_flow_case` | `fs::write(target, content.as_str().unwrap()).unwrap` | [1307](../../src/process_live_tests.rs#L1307) | receiver-type-required |
| `real_public_flow_case` | `fs::write` | [1307](../../src/process_live_tests.rs#L1307), [1324](../../src/process_live_tests.rs#L1324), [1331](../../src/process_live_tests.rs#L1331), [1350](../../src/process_live_tests.rs#L1350), [1374](../../src/process_live_tests.rs#L1374), [1393](../../src/process_live_tests.rs#L1393), [1433](../../src/process_live_tests.rs#L1433), [1451](../../src/process_live_tests.rs#L1451), [1456](../../src/process_live_tests.rs#L1456) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `content.as_str().unwrap` | [1307](../../src/process_live_tests.rs#L1307) | receiver-type-required |
| `real_public_flow_case` | `content.as_str` | [1307](../../src/process_live_tests.rs#L1307) | receiver-type-required |
| `real_public_flow_case` | `register_local_tools_server` | [1311](../../src/process_live_tests.rs#L1311) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1076) |
| `real_public_flow_case` | `root.join("threads").join` | [1314](../../src/process_live_tests.rs#L1314) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1315](../../src/process_live_tests.rs#L1315) | receiver-type-required |
| `real_public_flow_case` | `folder.join` | [1315](../../src/process_live_tests.rs#L1315), [1316](../../src/process_live_tests.rs#L1316), [1319](../../src/process_live_tests.rs#L1319), [1329](../../src/process_live_tests.rs#L1329), [1372](../../src/process_live_tests.rs#L1372) | receiver-type-required |
| `real_public_flow_case` | `write_test_genesis_with_identity` | [1317](../../src/process_live_tests.rs#L1317) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["identity_profile"].as_str` | [1317](../../src/process_live_tests.rs#L1317) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(root.join(".agent/commands")).unwrap` | [1323](../../src/process_live_tests.rs#L1323), [1330](../../src/process_live_tests.rs#L1330), [1373](../../src/process_live_tests.rs#L1373) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap` | [1324](../../src/process_live_tests.rs#L1324), [1331](../../src/process_live_tests.rs#L1331), [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str().unwrap` | [1324](../../src/process_live_tests.rs#L1324), [1331](../../src/process_live_tests.rs#L1331), [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str` | [1324](../../src/process_live_tests.rs#L1324), [1331](../../src/process_live_tests.rs#L1331), [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_flow_case` | `Arc::new` | [1334](../../src/process_live_tests.rs#L1334), [1342](../../src/process_live_tests.rs#L1342), [1377](../../src/process_live_tests.rs#L1377), [1385](../../src/process_live_tests.rs#L1385) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `provider::MemorySecretStore::new` | [1334](../../src/process_live_tests.rs#L1334), [1377](../../src/process_live_tests.rs#L1377) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_flow_case` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `secrets.publish` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str().unwrap` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379) | receiver-type-required |
| `real_public_flow_case` | `tokio::runtime::Runtime::new().unwrap` | [1337](../../src/process_live_tests.rs#L1337), [1380](../../src/process_live_tests.rs#L1380) | receiver-type-required |
| `real_public_flow_case` | `tokio::runtime::Runtime::new` | [1337](../../src/process_live_tests.rs#L1337), [1380](../../src/process_live_tests.rs#L1380) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1338](../../src/process_live_tests.rs#L1338), [1381](../../src/process_live_tests.rs#L1381) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on` | [1338](../../src/process_live_tests.rs#L1338), [1362](../../src/process_live_tests.rs#L1362), [1381](../../src/process_live_tests.rs#L1381), [1405](../../src/process_live_tests.rs#L1405) | receiver-type-required |
| `real_public_flow_case` | `tokio::net::TcpListener::bind` | [1338](../../src/process_live_tests.rs#L1338), [1381](../../src/process_live_tests.rs#L1381) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `listener.local_addr().unwrap` | [1339](../../src/process_live_tests.rs#L1339), [1382](../../src/process_live_tests.rs#L1382) | receiver-type-required |
| `real_public_flow_case` | `listener.local_addr` | [1339](../../src/process_live_tests.rs#L1339), [1382](../../src/process_live_tests.rs#L1382) | receiver-type-required |
| `real_public_flow_case` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1340](../../src/process_live_tests.rs#L1340), [1383](../../src/process_live_tests.rs#L1383) | receiver-type-required |
| `real_public_flow_case` | `crate::host_runtime::assemble_application_endpoint_host` | [1340](../../src/process_live_tests.rs#L1340), [1383](../../src/process_live_tests.rs#L1383) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L286) |
| `real_public_flow_case` | `"test".into` | [1341](../../src/process_live_tests.rs#L1341), [1384](../../src/process_live_tests.rs#L1384) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy().into_owned` | [1341](../../src/process_live_tests.rs#L1341), [1384](../../src/process_live_tests.rs#L1384) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy` | [1341](../../src/process_live_tests.rs#L1341), [1384](../../src/process_live_tests.rs#L1384) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy().into_owned` | [1341](../../src/process_live_tests.rs#L1341), [1384](../../src/process_live_tests.rs#L1384) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy` | [1341](../../src/process_live_tests.rs#L1341), [1384](../../src/process_live_tests.rs#L1384) | receiver-type-required |
| `real_public_flow_case` | `Ok` | [1342](../../src/process_live_tests.rs#L1342), [1385](../../src/process_live_tests.rs#L1385) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `"2026-09-04T10:00:00.000Z".into` | [1342](../../src/process_live_tests.rs#L1342), [1385](../../src/process_live_tests.rs#L1385) | receiver-type-required |
| `real_public_flow_case` | `host.clone` | [1342](../../src/process_live_tests.rs#L1342), [1343](../../src/process_live_tests.rs#L1343), [1385](../../src/process_live_tests.rs#L1385), [1386](../../src/process_live_tests.rs#L1386) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1343](../../src/process_live_tests.rs#L1343), [1386](../../src/process_live_tests.rs#L1386) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1343](../../src/process_live_tests.rs#L1343), [1386](../../src/process_live_tests.rs#L1386) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2617) |
| `real_public_flow_case` | `transport::TransportConfig::loopback` | [1344](../../src/process_live_tests.rs#L1344), [1387](../../src/process_live_tests.rs#L1387) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_flow_case` | `transport::BearerToken::new` | [1344](../../src/process_live_tests.rs#L1344), [1387](../../src/process_live_tests.rs#L1387) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_flow_case` | `host.attach_streams` | [1345](../../src/process_live_tests.rs#L1345), [1388](../../src/process_live_tests.rs#L1388) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams().clone` | [1345](../../src/process_live_tests.rs#L1345), [1388](../../src/process_live_tests.rs#L1388) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams` | [1345](../../src/process_live_tests.rs#L1345), [1388](../../src/process_live_tests.rs#L1388) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery().unwrap` | [1346](../../src/process_live_tests.rs#L1346), [1389](../../src/process_live_tests.rs#L1389) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery` | [1346](../../src/process_live_tests.rs#L1346), [1389](../../src/process_live_tests.rs#L1389) | receiver-type-required |
| `real_public_flow_case` | `assembly.into_server` | [1347](../../src/process_live_tests.rs#L1347), [1390](../../src/process_live_tests.rs#L1390) | receiver-type-required |
| `real_public_flow_case` | `server.handle` | [1348](../../src/process_live_tests.rs#L1348), [1391](../../src/process_live_tests.rs#L1391) | receiver-type-required |
| `real_public_flow_case` | `runtime.spawn` | [1349](../../src/process_live_tests.rs#L1349), [1392](../../src/process_live_tests.rs#L1392) | receiver-type-required |
| `real_public_flow_case` | `server.serve` | [1349](../../src/process_live_tests.rs#L1349), [1392](../../src/process_live_tests.rs#L1392) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1350](../../src/process_live_tests.rs#L1350), [1393](../../src/process_live_tests.rs#L1393) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1350](../../src/process_live_tests.rs#L1350), [1393](../../src/process_live_tests.rs#L1393) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec` | [1350](../../src/process_live_tests.rs#L1350), [1393](../../src/process_live_tests.rs#L1393) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.start_periodic_sweep` | [1351](../../src/process_live_tests.rs#L1351), [1394](../../src/process_live_tests.rs#L1394) | receiver-type-required |
| `real_public_flow_case` | `std::panic::catch_unwind` | [1352](../../src/process_live_tests.rs#L1352), [1395](../../src/process_live_tests.rs#L1395) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::panic::AssertUnwindSafe` | [1352](../../src/process_live_tests.rs#L1352), [1395](../../src/process_live_tests.rs#L1395) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Instant::now` | [1353](../../src/process_live_tests.rs#L1353), [1396](../../src/process_live_tests.rs#L1396) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_secs` | [1353](../../src/process_live_tests.rs#L1353), [1396](../../src/process_live_tests.rs#L1396) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64().unwrap_or` | [1353](../../src/process_live_tests.rs#L1353), [1396](../../src/process_live_tests.rs#L1396) | receiver-type-required |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64` | [1353](../../src/process_live_tests.rs#L1353), [1396](../../src/process_live_tests.rs#L1396) | receiver-type-required |
| `real_public_flow_case` | `root.join("client-flow-receipt.json").exists` | [1354](../../src/process_live_tests.rs#L1354), [1397](../../src/process_live_tests.rs#L1397) | receiver-type-required |
| `real_public_flow_case` | `std::thread::sleep` | [1357](../../src/process_live_tests.rs#L1357), [1400](../../src/process_live_tests.rs#L1400) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_millis` | [1357](../../src/process_live_tests.rs#L1357), [1400](../../src/process_live_tests.rs#L1400) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.shutdown` | [1360](../../src/process_live_tests.rs#L1360), [1403](../../src/process_live_tests.rs#L1403) | receiver-type-required |
| `real_public_flow_case` | `handle.begin_drain` | [1361](../../src/process_live_tests.rs#L1361), [1404](../../src/process_live_tests.rs#L1404) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap().unwrap` | [1362](../../src/process_live_tests.rs#L1362), [1405](../../src/process_live_tests.rs#L1405) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap` | [1362](../../src/process_live_tests.rs#L1362), [1405](../../src/process_live_tests.rs#L1405) | receiver-type-required |
| `real_public_flow_case` | `std::panic::resume_unwind` | [1363](../../src/process_live_tests.rs#L1363), [1406](../../src/process_live_tests.rs#L1406) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_str(line).unwrap` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_str` | [1364](../../src/process_live_tests.rs#L1364), [1407](../../src/process_live_tests.rs#L1407) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1365](../../src/process_live_tests.rs#L1365), [1408](../../src/process_live_tests.rs#L1408) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter` | [1365](../../src/process_live_tests.rs#L1365), [1408](../../src/process_live_tests.rs#L1408), [1419](../../src/process_live_tests.rs#L1419), [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `events.iter` | [1365](../../src/process_live_tests.rs#L1365), [1408](../../src/process_live_tests.rs#L1408), [1417](../../src/process_live_tests.rs#L1417), [1419](../../src/process_live_tests.rs#L1419), [1439](../../src/process_live_tests.rs#L1439), [1442](../../src/process_live_tests.rs#L1442), [1444](../../src/process_live_tests.rs#L1444), [1447](../../src/process_live_tests.rs#L1447), [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="compact" && e["origin_tuple"]["op"]=="commands/run").expect` | [1417](../../src/process_live_tests.rs#L1417), [1439](../../src/process_live_tests.rs#L1439) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find` | [1417](../../src/process_live_tests.rs#L1417), [1439](../../src/process_live_tests.rs#L1439), [1442](../../src/process_live_tests.rs#L1442), [1444](../../src/process_live_tests.rs#L1444), [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap().len` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="settle" && e["seq"].as_u64() < compact["seq"].as_u64()).count` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64` | [1419](../../src/process_live_tests.rs#L1419), [1442](../../src/process_live_tests.rs#L1442), [1444](../../src/process_live_tests.rs#L1444), [1447](../../src/process_live_tests.rs#L1447), [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `compact["seq"].as_u64` | [1419](../../src/process_live_tests.rs#L1419), [1440](../../src/process_live_tests.rs#L1440) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array().expect` | [1425](../../src/process_live_tests.rs#L1425) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array` | [1425](../../src/process_live_tests.rs#L1425) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map(&#124;r&#124; r["from"].as_u64().unwrap()..=r["to"].as_u64().unwrap()).collect` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64().unwrap` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64().unwrap` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str().map` | [1429](../../src/process_live_tests.rs#L1429) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str` | [1429](../../src/process_live_tests.rs#L1429) | receiver-type-required |
| `real_public_flow_case` | `summary.starts_with` | [1429](../../src/process_live_tests.rs#L1429) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger(&fs::read(&path).unwrap(), schema::READER_VERSION).unwrap` | [1431](../../src/process_live_tests.rs#L1431), [1445](../../src/process_live_tests.rs#L1445) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger` | [1431](../../src/process_live_tests.rs#L1431), [1445](../../src/process_live_tests.rs#L1445) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1053) |
| `real_public_flow_case` | `fs::read(&path).unwrap` | [1431](../../src/process_live_tests.rs#L1431), [1445](../../src/process_live_tests.rs#L1445) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compaction-summary.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap()).unwrap` | [1433](../../src/process_live_tests.rs#L1433) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap` | [1433](../../src/process_live_tests.rs#L1433) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty` | [1433](../../src/process_live_tests.rs#L1433), [1451](../../src/process_live_tests.rs#L1451), [1456](../../src/process_live_tests.rs#L1456) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `compact["seq"].as_u64().unwrap` | [1440](../../src/process_live_tests.rs#L1440) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap().to_owned` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="epoch" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1442](../../src/process_live_tests.rs#L1442) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64().unwrap` | [1442](../../src/process_live_tests.rs#L1442), [1444](../../src/process_live_tests.rs#L1444), [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64().unwrap` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt(&typed.events, &origin_key, &[post_input]).expect` | [1446](../../src/process_live_tests.rs#L1446) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt` | [1446](../../src/process_live_tests.rs#L1446) | [engine::compact_gate::first_post_compact_attempt](../../../engine/src/compact_gate.rs#L20) |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap().to_owned` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_flow_case` | `Some` | [1447](../../src/process_live_tests.rs#L1447) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map(&#124;e&#124; e["call"].as_str().map(str::to_owned)).collect` | [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map` | [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str().map` | [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str` | [1448](../../src/process_live_tests.rs#L1448) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compact-gate.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap()).unwrap` | [1451](../../src/process_live_tests.rs#L1451) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap` | [1451](../../src/process_live_tests.rs#L1451) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap()).unwrap` | [1456](../../src/process_live_tests.rs#L1456) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap` | [1456](../../src/process_live_tests.rs#L1456) | receiver-type-required |
| `real_public_mcp_task_continuation` | `PathBuf::from` | [1469](../../src/process_live_tests.rs#L1469) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1469](../../src/process_live_tests.rs#L1469) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var` | [1469](../../src/process_live_tests.rs#L1469), [1475](../../src/process_live_tests.rs#L1475), [1478](../../src/process_live_tests.rs#L1478), [1504](../../src/process_live_tests.rs#L1504), [1505](../../src/process_live_tests.rs#L1505) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&root).unwrap` | [1470](../../src/process_live_tests.rs#L1470) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all` | [1470](../../src/process_live_tests.rs#L1470), [1472](../../src/process_live_tests.rs#L1472), [1473](../../src/process_live_tests.rs#L1473), [1474](../../src/process_live_tests.rs#L1474), [1496](../../src/process_live_tests.rs#L1496) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `root.parent().unwrap().join` | [1471](../../src/process_live_tests.rs#L1471) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent().unwrap` | [1471](../../src/process_live_tests.rs#L1471) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent` | [1471](../../src/process_live_tests.rs#L1471) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&workspace).unwrap` | [1472](../../src/process_live_tests.rs#L1472) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(root.join("config")).unwrap` | [1473](../../src/process_live_tests.rs#L1473) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.join` | [1473](../../src/process_live_tests.rs#L1473), [1474](../../src/process_live_tests.rs#L1474), [1476](../../src/process_live_tests.rs#L1476), [1477](../../src/process_live_tests.rs#L1477), [1479](../../src/process_live_tests.rs#L1479), [1495](../../src/process_live_tests.rs#L1495), [1505](../../src/process_live_tests.rs#L1505), [1545](../../src/process_live_tests.rs#L1545) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1475](../../src/process_live_tests.rs#L1475) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice` | [1475](../../src/process_live_tests.rs#L1475) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1475](../../src/process_live_tests.rs#L1475) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read` | [1475](../../src/process_live_tests.rs#L1475) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1475](../../src/process_live_tests.rs#L1475) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_canonical_test_json` | [1476](../../src/process_live_tests.rs#L1476), [1477](../../src/process_live_tests.rs#L1477) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect` | [1478](../../src/process_live_tests.rs#L1478) | receiver-type-required |
| `real_public_mcp_task_continuation` | `mcp::McpRegistryStore::new` | [1479](../../src/process_live_tests.rs#L1479) | [mcp::management::McpRegistryStore::new](../../../mcp/src/management.rs#L391) |
| `real_public_mcp_task_continuation` | `registry.mutate_idempotent("save-fixture", &mcp::McpManagementMutation::Save {         server: mcp::McpServerConfig {             folder_id: None,             reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "fixture".to_owned() },             transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "task-augmented".to_owned()], cwd: None, environment: BTreeMap::new() },             enabled: true,             always_on: true,             protocol_mode: mcp::ProtocolMode::Legacy,             owner: None,             plugin_component: None,             project_trusted: true,         },         credential_fields: BTreeMap::new(),     }).expect` | [1480](../../src/process_live_tests.rs#L1480) | receiver-type-required |
| `real_public_mcp_task_continuation` | `registry.mutate_idempotent` | [1480](../../src/process_live_tests.rs#L1480) | receiver-type-required |
| `real_public_mcp_task_continuation` | `"ws".to_owned` | [1483](../../src/process_live_tests.rs#L1483) | receiver-type-required |
| `real_public_mcp_task_continuation` | `"fixture".to_owned` | [1483](../../src/process_live_tests.rs#L1483) | receiver-type-required |
| `real_public_mcp_task_continuation` | `BTreeMap::new` | [1484](../../src/process_live_tests.rs#L1484), [1492](../../src/process_live_tests.rs#L1492) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `root.join("threads").join` | [1495](../../src/process_live_tests.rs#L1495) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1496](../../src/process_live_tests.rs#L1496) | receiver-type-required |
| `real_public_mcp_task_continuation` | `folder.join` | [1496](../../src/process_live_tests.rs#L1496), [1497](../../src/process_live_tests.rs#L1497) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_test_genesis` | [1498](../../src/process_live_tests.rs#L1498) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `append_test_input` | [1499](../../src/process_live_tests.rs#L1499) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [1500](../../src/process_live_tests.rs#L1500), [1519](../../src/process_live_tests.rs#L1519) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map` | [1500](../../src/process_live_tests.rs#L1500), [1519](../../src/process_live_tests.rs#L1519) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines` | [1500](../../src/process_live_tests.rs#L1500), [1511](../../src/process_live_tests.rs#L1511), [1519](../../src/process_live_tests.rs#L1519) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap` | [1500](../../src/process_live_tests.rs#L1500), [1511](../../src/process_live_tests.rs#L1511), [1519](../../src/process_live_tests.rs#L1519) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string` | [1500](../../src/process_live_tests.rs#L1500), [1511](../../src/process_live_tests.rs#L1511), [1519](../../src/process_live_tests.rs#L1519) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).unwrap` | [1500](../../src/process_live_tests.rs#L1500), [1519](../../src/process_live_tests.rs#L1519) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str` | [1500](../../src/process_live_tests.rs#L1500), [1511](../../src/process_live_tests.rs#L1511), [1519](../../src/process_live_tests.rs#L1519) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `write_test_events` | [1502](../../src/process_live_tests.rs#L1502) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Arc::new` | [1503](../../src/process_live_tests.rs#L1503) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `provider::MemorySecretStore::new` | [1503](../../src/process_live_tests.rs#L1503) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_mcp_task_continuation` | `secrets.publish(provider["credential_key"].as_str().unwrap(), provider::SecretRecord::Active { generation: 1, material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap() }).unwrap` | [1504](../../src/process_live_tests.rs#L1504) | receiver-type-required |
| `real_public_mcp_task_continuation` | `secrets.publish` | [1504](../../src/process_live_tests.rs#L1504) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str().unwrap` | [1504](../../src/process_live_tests.rs#L1504) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str` | [1504](../../src/process_live_tests.rs#L1504) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1504](../../src/process_live_tests.rs#L1504) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store(&root, std::env::var("TEKES_TEST_REAL_WORKER").unwrap(), "live-mcp-task", root.join(".agent"), secrets).unwrap` | [1505](../../src/process_live_tests.rs#L1505) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store` | [1505](../../src/process_live_tests.rs#L1505) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1505](../../src/process_live_tests.rs#L1505) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::panic::catch_unwind` | [1506](../../src/process_live_tests.rs#L1506) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::panic::AssertUnwindSafe` | [1506](../../src/process_live_tests.rs#L1506) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap().expect` | [1507](../../src/process_live_tests.rs#L1507) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap` | [1507](../../src/process_live_tests.rs#L1507) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main` | [1507](../../src/process_live_tests.rs#L1507) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.start_periodic_sweep` | [1508](../../src/process_live_tests.rs#L1508) | receiver-type-required |
| `real_public_mcp_task_continuation` | `Instant::now` | [1509](../../src/process_live_tests.rs#L1509) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_secs` | [1509](../../src/process_live_tests.rs#L1509) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [1511](../../src/process_live_tests.rs#L1511) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [1511](../../src/process_live_tests.rs#L1511) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).ok` | [1511](../../src/process_live_tests.rs#L1511) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().any` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter` | [1512](../../src/process_live_tests.rs#L1512), [1520](../../src/process_live_tests.rs#L1520), [1522](../../src/process_live_tests.rs#L1522), [1525](../../src/process_live_tests.rs#L1525), [1532](../../src/process_live_tests.rs#L1532), [1536](../../src/process_live_tests.rs#L1536), [1542](../../src/process_live_tests.rs#L1542) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::thread::sleep` | [1514](../../src/process_live_tests.rs#L1514) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_millis` | [1514](../../src/process_live_tests.rs#L1514) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.shutdown` | [1517](../../src/process_live_tests.rs#L1517) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::panic::resume_unwind` | [1518](../../src/process_live_tests.rs#L1518) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `events.iter().find(&#124;e&#124; e["kind"] == "settle").unwrap` | [1520](../../src/process_live_tests.rs#L1520) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().find` | [1520](../../src/process_live_tests.rs#L1520) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "mcp__fixture__echo").collect` | [1522](../../src/process_live_tests.rs#L1522) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter` | [1522](../../src/process_live_tests.rs#L1522), [1525](../../src/process_live_tests.rs#L1525), [1532](../../src/process_live_tests.rs#L1532), [1536](../../src/process_live_tests.rs#L1536) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str().unwrap` | [1524](../../src/process_live_tests.rs#L1524) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str` | [1524](../../src/process_live_tests.rs#L1524) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "state" && e["subkind"] == "tool_continuation" && e["payload"]["call"] == call).collect` | [1525](../../src/process_live_tests.rs#L1525) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map(&#124;e&#124; e["payload"]["action"].as_str().unwrap()).collect` | [1526](../../src/process_live_tests.rs#L1526) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map` | [1526](../../src/process_live_tests.rs#L1526) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter` | [1526](../../src/process_live_tests.rs#L1526), [1529](../../src/process_live_tests.rs#L1529) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str().unwrap` | [1526](../../src/process_live_tests.rs#L1526) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str` | [1526](../../src/process_live_tests.rs#L1526) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find(&#124;e&#124; e["payload"]["action"] == "park").unwrap` | [1529](../../src/process_live_tests.rs#L1529) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find` | [1529](../../src/process_live_tests.rs#L1529) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "run_start").collect` | [1532](../../src/process_live_tests.rs#L1532) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64().unwrap` | [1534](../../src/process_live_tests.rs#L1534) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64` | [1534](../../src/process_live_tests.rs#L1534) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_result" && e["call"] == call).collect` | [1536](../../src/process_live_tests.rs#L1536) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str().unwrap` | [1539](../../src/process_live_tests.rs#L1539) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str` | [1539](../../src/process_live_tests.rs#L1539) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find(&#124;e&#124; e["kind"] == "output").unwrap` | [1542](../../src/process_live_tests.rs#L1542) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find` | [1542](../../src/process_live_tests.rs#L1542) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev` | [1542](../../src/process_live_tests.rs#L1542) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write(root.join("receipt.json"), serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap()).unwrap` | [1545](../../src/process_live_tests.rs#L1545) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write` | [1545](../../src/process_live_tests.rs#L1545) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap` | [1545](../../src/process_live_tests.rs#L1545) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty` | [1545](../../src/process_live_tests.rs#L1545) | external-constructor-callback-or-unresolved |
