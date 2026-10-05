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
| [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../src/process_live_tests.rs#L540) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../src/process_live_tests.rs#L732) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../src/process_live_tests.rs#L897) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L980) | function_item | `private` | test;  |
| [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../src/process_live_tests.rs#L1002) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_image_attachment](../../src/process_live_tests.rs#L1117) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_flow_case](../../src/process_live_tests.rs#L1190) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_mcp_task_continuation](../../src/process_live_tests.rs#L1371) | function_item | `private` | test; #[cfg(target_os = "macos")] |

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
| `run_real_validator_process` | `store::LockedLedger::open(&path, 1).unwrap` | [255](../../src/process_live_tests.rs#L255) | receiver-type-required |
| `run_real_validator_process` | `store::LockedLedger::open` | [255](../../src/process_live_tests.rs#L255) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L153) |
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
| `run_real_validator_process` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [288](../../src/process_live_tests.rs#L288) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L99) |
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
| `real_validator_exhaustion_releases_queued_input` | `PathBuf::from` | [541](../../src/process_live_tests.rs#L541), [642](../../src/process_live_tests.rs#L642) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [541](../../src/process_live_tests.rs#L541) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var` | [541](../../src/process_live_tests.rs#L541), [553](../../src/process_live_tests.rs#L553), [585](../../src/process_live_tests.rs#L585), [591](../../src/process_live_tests.rs#L591), [618](../../src/process_live_tests.rs#L618) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&root).unwrap` | [542](../../src/process_live_tests.rs#L542) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all` | [542](../../src/process_live_tests.rs#L542), [544](../../src/process_live_tests.rs#L544), [545](../../src/process_live_tests.rs#L545), [547](../../src/process_live_tests.rs#L547), [548](../../src/process_live_tests.rs#L548), [570](../../src/process_live_tests.rs#L570) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap().join` | [543](../../src/process_live_tests.rs#L543) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap` | [543](../../src/process_live_tests.rs#L543) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent` | [543](../../src/process_live_tests.rs#L543) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&workspace).unwrap` | [544](../../src/process_live_tests.rs#L544) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(workspace.join(".agent/skills")).unwrap` | [545](../../src/process_live_tests.rs#L545) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.join` | [545](../../src/process_live_tests.rs#L545), [546](../../src/process_live_tests.rs#L546), [577](../../src/process_live_tests.rs#L577), [646](../../src/process_live_tests.rs#L646) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(workspace.join(".agent/skills/live-proof.md"), "Live proof skill\nSKILL_LIVE_BODY_OK\n").unwrap` | [546](../../src/process_live_tests.rs#L546) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write` | [546](../../src/process_live_tests.rs#L546), [610](../../src/process_live_tests.rs#L610), [725](../../src/process_live_tests.rs#L725) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(root.join("config")).unwrap` | [547](../../src/process_live_tests.rs#L547) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join` | [547](../../src/process_live_tests.rs#L547), [548](../../src/process_live_tests.rs#L548), [549](../../src/process_live_tests.rs#L549), [557](../../src/process_live_tests.rs#L557), [561](../../src/process_live_tests.rs#L561), [569](../../src/process_live_tests.rs#L569), [593](../../src/process_live_tests.rs#L593), [602](../../src/process_live_tests.rs#L602), [610](../../src/process_live_tests.rs#L610), [612](../../src/process_live_tests.rs#L612), [684](../../src/process_live_tests.rs#L684), [725](../../src/process_live_tests.rs#L725) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [548](../../src/process_live_tests.rs#L548) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_canonical_test_json` | [549](../../src/process_live_tests.rs#L549), [556](../../src/process_live_tests.rs#L556), [560](../../src/process_live_tests.rs#L560) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [552](../../src/process_live_tests.rs#L552) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice` | [552](../../src/process_live_tests.rs#L552) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [553](../../src/process_live_tests.rs#L553) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read` | [553](../../src/process_live_tests.rs#L553) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [553](../../src/process_live_tests.rs#L553) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join("threads").join` | [569](../../src/process_live_tests.rs#L569) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(folder.join("assets")).unwrap` | [570](../../src/process_live_tests.rs#L570) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `folder.join` | [570](../../src/process_live_tests.rs#L570), [571](../../src/process_live_tests.rs#L571), [677](../../src/process_live_tests.rs#L677), [716](../../src/process_live_tests.rs#L716), [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_test_genesis` | [572](../../src/process_live_tests.rs#L572) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `append_test_input` | [573](../../src/process_live_tests.rs#L573) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [574](../../src/process_live_tests.rs#L574) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map` | [574](../../src/process_live_tests.rs#L574), [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines` | [574](../../src/process_live_tests.rs#L574), [626](../../src/process_live_tests.rs#L626), [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap` | [574](../../src/process_live_tests.rs#L574), [626](../../src/process_live_tests.rs#L626), [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string` | [574](../../src/process_live_tests.rs#L574), [626](../../src/process_live_tests.rs#L626), [677](../../src/process_live_tests.rs#L677), [695](../../src/process_live_tests.rs#L695), [716](../../src/process_live_tests.rs#L716), [722](../../src/process_live_tests.rs#L722) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).unwrap` | [574](../../src/process_live_tests.rs#L574) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str` | [574](../../src/process_live_tests.rs#L574), [626](../../src/process_live_tests.rs#L626), [677](../../src/process_live_tests.rs#L677), [695](../../src/process_live_tests.rs#L695), [716](../../src/process_live_tests.rs#L716), [722](../../src/process_live_tests.rs#L722) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `write_test_events` | [576](../../src/process_live_tests.rs#L576) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Arc::new` | [579](../../src/process_live_tests.rs#L579), [602](../../src/process_live_tests.rs#L602), [617](../../src/process_live_tests.rs#L617) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `provider::MemorySecretStore::new` | [579](../../src/process_live_tests.rs#L579) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [580](../../src/process_live_tests.rs#L580) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish` | [580](../../src/process_live_tests.rs#L580) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str().unwrap` | [582](../../src/process_live_tests.rs#L582) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str` | [582](../../src/process_live_tests.rs#L582) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [585](../../src/process_live_tests.rs#L585) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     )     .unwrap` | [589](../../src/process_live_tests.rs#L589) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store` | [589](../../src/process_live_tests.rs#L589) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [591](../../src/process_live_tests.rs#L591) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new().unwrap` | [597](../../src/process_live_tests.rs#L597) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new` | [597](../../src/process_live_tests.rs#L597) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [598](../../src/process_live_tests.rs#L598) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on` | [598](../../src/process_live_tests.rs#L598), [693](../../src/process_live_tests.rs#L693) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::net::TcpListener::bind` | [598](../../src/process_live_tests.rs#L598) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr().unwrap` | [599](../../src/process_live_tests.rs#L599) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr` | [599](../../src/process_live_tests.rs#L599) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::daemon::assemble_production_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [600](../../src/process_live_tests.rs#L600) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::daemon::assemble_production_endpoint_host` | [600](../../src/process_live_tests.rs#L600) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_validator_exhaustion_releases_queued_input` | `"test".into` | [601](../../src/process_live_tests.rs#L601), [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy().into_owned` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy().into_owned` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy` | [601](../../src/process_live_tests.rs#L601) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Ok` | [602](../../src/process_live_tests.rs#L602), [618](../../src/process_live_tests.rs#L618) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `"2026-09-04T10:00:00.000Z".into` | [602](../../src/process_live_tests.rs#L602) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.clone` | [602](../../src/process_live_tests.rs#L602), [603](../../src/process_live_tests.rs#L603), [617](../../src/process_live_tests.rs#L617) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),         transport::TransportConfig::loopback(address, transport::BearerToken::new([42; 32]))).unwrap` | [603](../../src/process_live_tests.rs#L603) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [603](../../src/process_live_tests.rs#L603) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_validator_exhaustion_releases_queued_input` | `transport::TransportConfig::loopback` | [604](../../src/process_live_tests.rs#L604) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_validator_exhaustion_releases_queued_input` | `transport::BearerToken::new` | [604](../../src/process_live_tests.rs#L604) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_validator_exhaustion_releases_queued_input` | `host.attach_streams` | [605](../../src/process_live_tests.rs#L605) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams().clone` | [605](../../src/process_live_tests.rs#L605) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams` | [605](../../src/process_live_tests.rs#L605) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery().unwrap` | [606](../../src/process_live_tests.rs#L606) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery` | [606](../../src/process_live_tests.rs#L606) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.into_server` | [607](../../src/process_live_tests.rs#L607) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.handle` | [608](../../src/process_live_tests.rs#L608) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.spawn` | [609](../../src/process_live_tests.rs#L609) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.serve` | [609](../../src/process_live_tests.rs#L609) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [610](../../src/process_live_tests.rs#L610) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [610](../../src/process_live_tests.rs#L610) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec` | [610](../../src/process_live_tests.rs#L610) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Instant::now` | [611](../../src/process_live_tests.rs#L611), [624](../../src/process_live_tests.rs#L624) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_secs` | [611](../../src/process_live_tests.rs#L611), [624](../../src/process_live_tests.rs#L624) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-ready").exists` | [612](../../src/process_live_tests.rs#L612) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::thread::sleep` | [614](../../src/process_live_tests.rs#L614), [688](../../src/process_live_tests.rs#L688) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_millis` | [614](../../src/process_live_tests.rs#L614), [688](../../src/process_live_tests.rs#L688) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open(         &root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [616](../../src/process_live_tests.rs#L616) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [616](../../src/process_live_tests.rs#L616) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L99) |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_host::SessionAdmissionGates::default` | [617](../../src/process_live_tests.rs#L617) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [618](../../src/process_live_tests.rs#L618) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Vec::new` | [619](../../src/process_live_tests.rs#L619) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::catch_unwind` | [620](../../src/process_live_tests.rs#L620) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::AssertUnwindSafe` | [620](../../src/process_live_tests.rs#L620) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap().expect` | [621](../../src/process_live_tests.rs#L621) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap` | [621](../../src/process_live_tests.rs#L621) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main` | [621](../../src/process_live_tests.rs#L621) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.start_periodic_sweep` | [622](../../src/process_live_tests.rs#L622) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [626](../../src/process_live_tests.rs#L626) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [626](../../src/process_live_tests.rs#L626) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).ok` | [626](../../src/process_live_tests.rs#L626) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter` | [627](../../src/process_live_tests.rs#L627), [676](../../src/process_live_tests.rs#L676), [680](../../src/process_live_tests.rs#L680), [700](../../src/process_live_tests.rs#L700), [720](../../src/process_live_tests.rs#L720) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter` | [627](../../src/process_live_tests.rs#L627), [671](../../src/process_live_tests.rs#L671), [676](../../src/process_live_tests.rs#L676), [680](../../src/process_live_tests.rs#L680), [700](../../src/process_live_tests.rs#L700), [707](../../src/process_live_tests.rs#L707), [708](../../src/process_live_tests.rs#L708), [711](../../src/process_live_tests.rs#L711), [713](../../src/process_live_tests.rs#L713), [715](../../src/process_live_tests.rs#L715), [720](../../src/process_live_tests.rs#L720) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .any` | [628](../../src/process_live_tests.rs#L628) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter` | [628](../../src/process_live_tests.rs#L628), [634](../../src/process_live_tests.rs#L634) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find(&#124;e&#124; e["kind"] == "tool_call" && e["call"] == held["call"])                     .unwrap` | [634](../../src/process_live_tests.rs#L634) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find` | [634](../../src/process_live_tests.rs#L634) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str().unwrap` | [642](../../src/process_live_tests.rs#L642) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str` | [642](../../src/process_live_tests.rs#L642) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.is_absolute` | [643](../../src/process_live_tests.rs#L643) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize().unwrap` | [648](../../src/process_live_tests.rs#L648) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize` | [648](../../src/process_live_tests.rs#L648) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap` | [648](../../src/process_live_tests.rs#L648) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent` | [648](../../src/process_live_tests.rs#L648) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `key.clone` | [655](../../src/process_live_tests.rs#L655) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"live-disposable-approval".into` | [656](../../src/process_live_tests.rs#L656) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `session.into` | [657](../../src/process_live_tests.rs#L657), [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap().into` | [658](../../src/process_live_tests.rs#L658) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap` | [658](../../src/process_live_tests.rs#L658) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str` | [658](../../src/process_live_tests.rs#L658) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization.clone())                     .expect` | [664](../../src/process_live_tests.rs#L664) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author` | [664](../../src/process_live_tests.rs#L664), [666](../../src/process_live_tests.rs#L666) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `authorization.clone` | [664](../../src/process_live_tests.rs#L664) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization)                     .expect` | [666](../../src/process_live_tests.rs#L666) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `approval_deliveries.push` | [669](../../src/process_live_tests.rs#L669) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().any` | [671](../../src/process_live_tests.rs#L671) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"exhaustion-process".into` | [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"session.prompt".into` | [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"queued-body".into` | [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt(session,"2026-09-04T10:00:00.000Z",&origin,&MaterializedPrompt {blocks:vec![Block::Text {text:"重试".into()}],attachments:vec![],files:vec![]},false).unwrap` | [673](../../src/process_live_tests.rs#L673) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt` | [673](../../src/process_live_tests.rs#L673) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "settle").count` | [676](../../src/process_live_tests.rs#L676) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [677](../../src/process_live_tests.rs#L677) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map` | [677](../../src/process_live_tests.rs#L677) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines` | [677](../../src/process_live_tests.rs#L677) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default` | [677](../../src/process_live_tests.rs#L677) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).ok` | [677](../../src/process_live_tests.rs#L677) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter(&#124;e&#124; e["event"]["type"] == "turn/end").collect` | [678](../../src/process_live_tests.rs#L678) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter` | [678](../../src/process_live_tests.rs#L678) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter` | [678](../../src/process_live_tests.rs#L678) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.len` | [679](../../src/process_live_tests.rs#L679) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter().zip` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-receipt.json").exists` | [684](../../src/process_live_tests.rs#L684) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.shutdown` | [691](../../src/process_live_tests.rs#L691) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_handle.begin_drain` | [692](../../src/process_live_tests.rs#L692) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap().unwrap` | [693](../../src/process_live_tests.rs#L693) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap` | [693](../../src/process_live_tests.rs#L693) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::resume_unwind` | [694](../../src/process_live_tests.rs#L694) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).unwrap` | [695](../../src/process_live_tests.rs#L695), [716](../../src/process_live_tests.rs#L716), [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "approval_response").collect` | [700](../../src/process_live_tests.rs#L700) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 1).unwrap` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find` | [707](../../src/process_live_tests.rs#L707), [708](../../src/process_live_tests.rs#L708), [711](../../src/process_live_tests.rs#L711), [713](../../src/process_live_tests.rs#L713), [715](../../src/process_live_tests.rs#L715) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 2).unwrap` | [708](../../src/process_live_tests.rs#L708) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "turn_open" && e["turn"] == 2).unwrap` | [711](../../src/process_live_tests.rs#L711) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "input" && e["content"][0]["text"] == "重试").unwrap` | [713](../../src/process_live_tests.rs#L713) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "spawn").unwrap` | [715](../../src/process_live_tests.rs#L715) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str().unwrap` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "spawn").collect` | [720](../../src/process_live_tests.rs#L720) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str().unwrap` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str` | [722](../../src/process_live_tests.rs#L722) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "verify").expect` | [723](../../src/process_live_tests.rs#L723) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find` | [723](../../src/process_live_tests.rs#L723) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter` | [723](../../src/process_live_tests.rs#L723) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap()).unwrap` | [725](../../src/process_live_tests.rs#L725) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap` | [725](../../src/process_live_tests.rs#L725) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty` | [725](../../src/process_live_tests.rs#L725) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `PathBuf::from` | [733](../../src/process_live_tests.rs#L733), [834](../../src/process_live_tests.rs#L834) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [733](../../src/process_live_tests.rs#L733) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var` | [733](../../src/process_live_tests.rs#L733), [738](../../src/process_live_tests.rs#L738), [742](../../src/process_live_tests.rs#L742), [756](../../src/process_live_tests.rs#L756), [793](../../src/process_live_tests.rs#L793), [794](../../src/process_live_tests.rs#L794), [795](../../src/process_live_tests.rs#L795) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.parent().unwrap().join` | [734](../../src/process_live_tests.rs#L734) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent().unwrap` | [734](../../src/process_live_tests.rs#L734) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent` | [734](../../src/process_live_tests.rs#L734) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("docs")).unwrap` | [735](../../src/process_live_tests.rs#L735) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all` | [735](../../src/process_live_tests.rs#L735), [736](../../src/process_live_tests.rs#L736), [737](../../src/process_live_tests.rs#L737), [744](../../src/process_live_tests.rs#L744), [745](../../src/process_live_tests.rs#L745), [751](../../src/process_live_tests.rs#L751), [760](../../src/process_live_tests.rs#L760), [763](../../src/process_live_tests.rs#L763) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `workspace.join` | [735](../../src/process_live_tests.rs#L735), [744](../../src/process_live_tests.rs#L744), [745](../../src/process_live_tests.rs#L745), [746](../../src/process_live_tests.rs#L746), [760](../../src/process_live_tests.rs#L760), [763](../../src/process_live_tests.rs#L763), [765](../../src/process_live_tests.rs#L765), [766](../../src/process_live_tests.rs#L766), [767](../../src/process_live_tests.rs#L767), [771](../../src/process_live_tests.rs#L771), [772](../../src/process_live_tests.rs#L772), [773](../../src/process_live_tests.rs#L773), [835](../../src/process_live_tests.rs#L835), [863](../../src/process_live_tests.rs#L863) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(root.join("config")).unwrap` | [736](../../src/process_live_tests.rs#L736) | receiver-type-required |
| `real_legacy_simple_task` | `root.join` | [736](../../src/process_live_tests.rs#L736), [737](../../src/process_live_tests.rs#L737), [739](../../src/process_live_tests.rs#L739), [740](../../src/process_live_tests.rs#L740), [741](../../src/process_live_tests.rs#L741), [747](../../src/process_live_tests.rs#L747), [750](../../src/process_live_tests.rs#L750), [794](../../src/process_live_tests.rs#L794), [802](../../src/process_live_tests.rs#L802), [810](../../src/process_live_tests.rs#L810), [812](../../src/process_live_tests.rs#L812), [843](../../src/process_live_tests.rs#L843), [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [737](../../src/process_live_tests.rs#L737) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [738](../../src/process_live_tests.rs#L738) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice` | [738](../../src/process_live_tests.rs#L738) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [738](../../src/process_live_tests.rs#L738) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read` | [738](../../src/process_live_tests.rs#L738) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [738](../../src/process_live_tests.rs#L738) | receiver-type-required |
| `real_legacy_simple_task` | `write_canonical_test_json` | [739](../../src/process_live_tests.rs#L739), [740](../../src/process_live_tests.rs#L740), [741](../../src/process_live_tests.rs#L741), [747](../../src/process_live_tests.rs#L747) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_LEGACY_TASK_SCENARIO").as_deref` | [742](../../src/process_live_tests.rs#L742) | receiver-type-required |
| `real_legacy_simple_task` | `Ok` | [742](../../src/process_live_tests.rs#L742), [795](../../src/process_live_tests.rs#L795), [802](../../src/process_live_tests.rs#L802) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("docs/drama")).unwrap` | [744](../../src/process_live_tests.rs#L744) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join(".agent/skills/chinese-short-drama")).unwrap` | [745](../../src/process_live_tests.rs#L745) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join(".agent/skills/chinese-short-drama/SKILL.md"),include_str!("../../../fixtures/live/drama/kernel-skill.md")).unwrap` | [746](../../src/process_live_tests.rs#L746) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write` | [746](../../src/process_live_tests.rs#L746), [765](../../src/process_live_tests.rs#L765), [766](../../src/process_live_tests.rs#L766), [767](../../src/process_live_tests.rs#L767), [771](../../src/process_live_tests.rs#L771), [772](../../src/process_live_tests.rs#L772), [773](../../src/process_live_tests.rs#L773), [810](../../src/process_live_tests.rs#L810), [891](../../src/process_live_tests.rs#L891) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("threads").join` | [750](../../src/process_live_tests.rs#L750) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(folder.join("assets")).unwrap` | [751](../../src/process_live_tests.rs#L751) | receiver-type-required |
| `real_legacy_simple_task` | `folder.join` | [751](../../src/process_live_tests.rs#L751), [752](../../src/process_live_tests.rs#L752) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_genesis` | [753](../../src/process_live_tests.rs#L753) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `append_test_input` | [754](../../src/process_live_tests.rs#L754) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [755](../../src/process_live_tests.rs#L755) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map` | [755](../../src/process_live_tests.rs#L755), [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines` | [755](../../src/process_live_tests.rs#L755), [824](../../src/process_live_tests.rs#L824), [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap` | [755](../../src/process_live_tests.rs#L755), [824](../../src/process_live_tests.rs#L824), [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string` | [755](../../src/process_live_tests.rs#L755), [824](../../src/process_live_tests.rs#L824), [828](../../src/process_live_tests.rs#L828), [854](../../src/process_live_tests.rs#L854), [863](../../src/process_live_tests.rs#L863) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `serde_json::from_str(line).unwrap` | [755](../../src/process_live_tests.rs#L755), [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str` | [755](../../src/process_live_tests.rs#L755), [824](../../src/process_live_tests.rs#L824), [828](../../src/process_live_tests.rs#L828), [854](../../src/process_live_tests.rs#L854) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_LEGACY_TASK_SCENARIO").unwrap_or_default` | [756](../../src/process_live_tests.rs#L756) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("src")).unwrap` | [760](../../src/process_live_tests.rs#L760) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("markdown")).unwrap` | [763](../../src/process_live_tests.rs#L763) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/1.md"), source).unwrap` | [765](../../src/process_live_tests.rs#L765) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/2.md"), "# Population\nBeijing has 21,000,000 people, based on markdown/1.md.\n").unwrap` | [766](../../src/process_live_tests.rs#L766) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/3.md"), "# People aged 60+\nBeijing has 4,200,000 people aged 60+, assuming 20% of the population in markdown/2.md.\n").unwrap` | [767](../../src/process_live_tests.rs#L767) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/1.py"), python_source).unwrap` | [771](../../src/process_live_tests.rs#L771) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/2.py"), "import importlib\ndef make_capital_sentence():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    return 'Beijing is the capital of China.'\n").unwrap` | [772](../../src/process_live_tests.rs#L772) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/3.py"), "import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    sentence = importlib.import_module('src.2').make_capital_sentence()\n    return 'China / Beijing: ' + sentence\n").unwrap` | [773](../../src/process_live_tests.rs#L773) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_events` | [791](../../src/process_live_tests.rs#L791) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Arc::new` | [792](../../src/process_live_tests.rs#L792), [802](../../src/process_live_tests.rs#L802), [818](../../src/process_live_tests.rs#L818) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `provider::MemorySecretStore::new` | [792](../../src/process_live_tests.rs#L792) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_legacy_simple_task` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_legacy_simple_task` | `secrets.publish` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str().unwrap` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"live-task",root.join(".agent"),secrets).unwrap` | [794](../../src/process_live_tests.rs#L794) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store` | [794](../../src/process_live_tests.rs#L794) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [794](../../src/process_live_tests.rs#L794) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [795](../../src/process_live_tests.rs#L795) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new().unwrap` | [797](../../src/process_live_tests.rs#L797) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new` | [797](../../src/process_live_tests.rs#L797) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [798](../../src/process_live_tests.rs#L798) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on` | [798](../../src/process_live_tests.rs#L798), [851](../../src/process_live_tests.rs#L851) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::net::TcpListener::bind` | [798](../../src/process_live_tests.rs#L798) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `listener.local_addr().unwrap` | [799](../../src/process_live_tests.rs#L799) | receiver-type-required |
| `real_legacy_simple_task` | `listener.local_addr` | [799](../../src/process_live_tests.rs#L799) | receiver-type-required |
| `real_legacy_simple_task` | `crate::daemon::assemble_production_endpoint_host(&root,             endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},             Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [800](../../src/process_live_tests.rs#L800) | receiver-type-required |
| `real_legacy_simple_task` | `crate::daemon::assemble_production_endpoint_host` | [800](../../src/process_live_tests.rs#L800) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_legacy_simple_task` | `"test".into` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy().into_owned` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy().into_owned` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy` | [801](../../src/process_live_tests.rs#L801) | receiver-type-required |
| `real_legacy_simple_task` | `"2026-09-04T10:00:00.000Z".into` | [802](../../src/process_live_tests.rs#L802) | receiver-type-required |
| `real_legacy_simple_task` | `host.clone` | [802](../../src/process_live_tests.rs#L802), [803](../../src/process_live_tests.rs#L803), [818](../../src/process_live_tests.rs#L818) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),             transport::TransportConfig::loopback(address, transport::BearerToken::new([42;32]))).unwrap` | [803](../../src/process_live_tests.rs#L803) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [803](../../src/process_live_tests.rs#L803) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_legacy_simple_task` | `transport::TransportConfig::loopback` | [804](../../src/process_live_tests.rs#L804) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_legacy_simple_task` | `transport::BearerToken::new` | [804](../../src/process_live_tests.rs#L804) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_legacy_simple_task` | `host.attach_streams` | [805](../../src/process_live_tests.rs#L805) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams().clone` | [805](../../src/process_live_tests.rs#L805) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams` | [805](../../src/process_live_tests.rs#L805) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery().unwrap` | [806](../../src/process_live_tests.rs#L806) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery` | [806](../../src/process_live_tests.rs#L806) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.into_server` | [807](../../src/process_live_tests.rs#L807) | receiver-type-required |
| `real_legacy_simple_task` | `server.handle` | [808](../../src/process_live_tests.rs#L808) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.spawn` | [809](../../src/process_live_tests.rs#L809) | receiver-type-required |
| `real_legacy_simple_task` | `server.serve` | [809](../../src/process_live_tests.rs#L809) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [810](../../src/process_live_tests.rs#L810) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [810](../../src/process_live_tests.rs#L810) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec` | [810](../../src/process_live_tests.rs#L810) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Instant::now` | [811](../../src/process_live_tests.rs#L811), [822](../../src/process_live_tests.rs#L822) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_secs` | [811](../../src/process_live_tests.rs#L811), [822](../../src/process_live_tests.rs#L822) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("client-ready").exists` | [812](../../src/process_live_tests.rs#L812) | receiver-type-required |
| `real_legacy_simple_task` | `std::thread::sleep` | [814](../../src/process_live_tests.rs#L814), [845](../../src/process_live_tests.rs#L845) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_millis` | [814](../../src/process_live_tests.rs#L814), [845](../../src/process_live_tests.rs#L845) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Some` | [816](../../src/process_live_tests.rs#L816), [827](../../src/process_live_tests.rs#L827) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open(&root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [818](../../src/process_live_tests.rs#L818) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [818](../../src/process_live_tests.rs#L818) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L99) |
| `real_legacy_simple_task` | `crate::endpoint_host::SessionAdmissionGates::default` | [818](../../src/process_live_tests.rs#L818) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::catch_unwind` | [819](../../src/process_live_tests.rs#L819) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::AssertUnwindSafe` | [819](../../src/process_live_tests.rs#L819) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap().expect` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_legacy_simple_task` | `host.start_periodic_sweep` | [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [824](../../src/process_live_tests.rs#L824) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [824](../../src/process_live_tests.rs#L824) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str(line).ok` | [824](../../src/process_live_tests.rs#L824), [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir(&folder).unwrap` | [825](../../src/process_live_tests.rs#L825) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir` | [825](../../src/process_live_tests.rs#L825) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `entry.unwrap().path` | [826](../../src/process_live_tests.rs#L826) | receiver-type-required |
| `real_legacy_simple_task` | `entry.unwrap` | [826](../../src/process_live_tests.rs#L826) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension().and_then` | [827](../../src/process_live_tests.rs#L827) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension` | [827](../../src/process_live_tests.rs#L827) | receiver-type-required |
| `real_legacy_simple_task` | `v.to_str` | [827](../../src/process_live_tests.rs#L827) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name().unwrap` | [827](../../src/process_live_tests.rs#L827), [838](../../src/process_live_tests.rs#L838) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name` | [827](../../src/process_live_tests.rs#L827), [838](../../src/process_live_tests.rs#L838) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap` | [828](../../src/process_live_tests.rs#L828) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().filter` | [829](../../src/process_live_tests.rs#L829) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter` | [829](../../src/process_live_tests.rs#L829), [830](../../src/process_live_tests.rs#L830), [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().any` | [830](../../src/process_live_tests.rs#L830) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["call"]==held["call"]).unwrap` | [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find` | [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str().unwrap` | [834](../../src/process_live_tests.rs#L834) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str` | [834](../../src/process_live_tests.rs#L834) | receiver-type-required |
| `real_legacy_simple_task` | `raw.is_absolute` | [835](../../src/process_live_tests.rs#L835) | receiver-type-required |
| `real_legacy_simple_task` | `session.to_owned` | [838](../../src/process_live_tests.rs#L838) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author(&responder,endpoint::RespondAuthorization {rpc_id:key.clone(),request_sha256:"live-task-approval".into(),session_id:target,call:held["call"].as_str().unwrap().into(),grant:true,answer:None,origin_key:key,resolution:endpoint::ResolutionOutcome::AllowedOnce}).expect` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author` | [840](../../src/process_live_tests.rs#L840) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `key.clone` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `"live-task-approval".into` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap().into` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter().any` | [843](../../src/process_live_tests.rs#L843) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter` | [843](../../src/process_live_tests.rs#L843) | receiver-type-required |
| `real_legacy_simple_task` | `root.join("client-task-receipt.json").exists` | [843](../../src/process_live_tests.rs#L843) | receiver-type-required |
| `real_legacy_simple_task` | `host.shutdown` | [848](../../src/process_live_tests.rs#L848) | receiver-type-required |
| `real_legacy_simple_task` | `handle.begin_drain` | [850](../../src/process_live_tests.rs#L850) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap().unwrap` | [851](../../src/process_live_tests.rs#L851) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap` | [851](../../src/process_live_tests.rs#L851) | receiver-type-required |
| `real_legacy_simple_task` | `std::panic::resume_unwind` | [853](../../src/process_live_tests.rs#L853) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [854](../../src/process_live_tests.rs#L854) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::Map::new` | [861](../../src/process_live_tests.rs#L861) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(workspace.join(relative)).expect` | [863](../../src/process_live_tests.rs#L863) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase().replace` | [875](../../src/process_live_tests.rs#L875) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase` | [875](../../src/process_live_tests.rs#L875) | receiver-type-required |
| `real_legacy_simple_task` | `artifacts.insert` | [885](../../src/process_live_tests.rs#L885) | receiver-type-required |
| `real_legacy_simple_task` | `(*relative).into` | [885](../../src/process_live_tests.rs#L885) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap()).unwrap` | [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap` | [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty` | [891](../../src/process_live_tests.rs#L891) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PUBLIC_QUESTION").is_ok_and` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var` | [898](../../src/process_live_tests.rs#L898), [899](../../src/process_live_tests.rs#L899), [904](../../src/process_live_tests.rs#L904), [913](../../src/process_live_tests.rs#L913), [914](../../src/process_live_tests.rs#L914) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `PathBuf::from` | [899](../../src/process_live_tests.rs#L899) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [899](../../src/process_live_tests.rs#L899) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap().join` | [900](../../src/process_live_tests.rs#L900) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap` | [900](../../src/process_live_tests.rs#L900) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent` | [900](../../src/process_live_tests.rs#L900) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(&workspace).unwrap` | [901](../../src/process_live_tests.rs#L901) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all` | [901](../../src/process_live_tests.rs#L901), [902](../../src/process_live_tests.rs#L902), [903](../../src/process_live_tests.rs#L903), [909](../../src/process_live_tests.rs#L909) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(root.join("config")).unwrap` | [902](../../src/process_live_tests.rs#L902) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join` | [902](../../src/process_live_tests.rs#L902), [903](../../src/process_live_tests.rs#L903), [905](../../src/process_live_tests.rs#L905), [906](../../src/process_live_tests.rs#L906), [908](../../src/process_live_tests.rs#L908), [914](../../src/process_live_tests.rs#L914), [920](../../src/process_live_tests.rs#L920), [928](../../src/process_live_tests.rs#L928), [935](../../src/process_live_tests.rs#L935), [937](../../src/process_live_tests.rs#L937), [974](../../src/process_live_tests.rs#L974) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [903](../../src/process_live_tests.rs#L903) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [904](../../src/process_live_tests.rs#L904) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice` | [904](../../src/process_live_tests.rs#L904) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [904](../../src/process_live_tests.rs#L904) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read` | [904](../../src/process_live_tests.rs#L904) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [904](../../src/process_live_tests.rs#L904) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_canonical_test_json` | [905](../../src/process_live_tests.rs#L905), [906](../../src/process_live_tests.rs#L906) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("threads").join` | [908](../../src/process_live_tests.rs#L908) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(folder.join("assets")).unwrap` | [909](../../src/process_live_tests.rs#L909) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `folder.join` | [909](../../src/process_live_tests.rs#L909), [910](../../src/process_live_tests.rs#L910) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_test_genesis` | [911](../../src/process_live_tests.rs#L911) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Arc::new` | [912](../../src/process_live_tests.rs#L912), [920](../../src/process_live_tests.rs#L920) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `provider::MemorySecretStore::new` | [912](../../src/process_live_tests.rs#L912) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str().unwrap` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap` | [914](../../src/process_live_tests.rs#L914) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store` | [914](../../src/process_live_tests.rs#L914) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [914](../../src/process_live_tests.rs#L914) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new().unwrap` | [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new` | [915](../../src/process_live_tests.rs#L915) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on` | [916](../../src/process_live_tests.rs#L916), [944](../../src/process_live_tests.rs#L944) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::net::TcpListener::bind` | [916](../../src/process_live_tests.rs#L916) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr().unwrap` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::daemon::assemble_production_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [918](../../src/process_live_tests.rs#L918) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::daemon::assemble_production_endpoint_host` | [918](../../src/process_live_tests.rs#L918) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_provider_400_releases_queue_over_public_transport` | `"test".into` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy().into_owned` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy().into_owned` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `Ok` | [920](../../src/process_live_tests.rs#L920) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `"2026-09-04T10:00:00.000Z".into` | [920](../../src/process_live_tests.rs#L920) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `host.clone` | [920](../../src/process_live_tests.rs#L920), [921](../../src/process_live_tests.rs#L921) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [921](../../src/process_live_tests.rs#L921) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [921](../../src/process_live_tests.rs#L921) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::TransportConfig::loopback` | [922](../../src/process_live_tests.rs#L922) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::BearerToken::new` | [922](../../src/process_live_tests.rs#L922) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_provider_400_releases_queue_over_public_transport` | `host.attach_streams` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams().clone` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery().unwrap` | [924](../../src/process_live_tests.rs#L924) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery` | [924](../../src/process_live_tests.rs#L924) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.into_server` | [925](../../src/process_live_tests.rs#L925) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.handle` | [926](../../src/process_live_tests.rs#L926) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.spawn` | [927](../../src/process_live_tests.rs#L927) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.serve` | [927](../../src/process_live_tests.rs#L927) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write` | [928](../../src/process_live_tests.rs#L928), [935](../../src/process_live_tests.rs#L935), [974](../../src/process_live_tests.rs#L974) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec` | [928](../../src/process_live_tests.rs#L928) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.start_periodic_sweep` | [929](../../src/process_live_tests.rs#L929) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::catch_unwind` | [930](../../src/process_live_tests.rs#L930) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::AssertUnwindSafe` | [930](../../src/process_live_tests.rs#L930) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Instant::now` | [931](../../src/process_live_tests.rs#L931) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_secs` | [931](../../src/process_live_tests.rs#L931) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [933](../../src/process_live_tests.rs#L933) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [933](../../src/process_live_tests.rs#L933) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines` | [933](../../src/process_live_tests.rs#L933), [946](../../src/process_live_tests.rs#L946) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap` | [933](../../src/process_live_tests.rs#L933), [946](../../src/process_live_tests.rs#L946) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string` | [933](../../src/process_live_tests.rs#L933), [946](../../src/process_live_tests.rs#L946) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).ok` | [933](../../src/process_live_tests.rs#L933) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str` | [933](../../src/process_live_tests.rs#L933), [946](../../src/process_live_tests.rs#L946) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").count` | [934](../../src/process_live_tests.rs#L934) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter` | [934](../../src/process_live_tests.rs#L934), [947](../../src/process_live_tests.rs#L947), [951](../../src/process_live_tests.rs#L951), [954](../../src/process_live_tests.rs#L954), [966](../../src/process_live_tests.rs#L966) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter` | [934](../../src/process_live_tests.rs#L934), [947](../../src/process_live_tests.rs#L947), [951](../../src/process_live_tests.rs#L951), [954](../../src/process_live_tests.rs#L954), [961](../../src/process_live_tests.rs#L961), [965](../../src/process_live_tests.rs#L965), [966](../../src/process_live_tests.rs#L966), [971](../../src/process_live_tests.rs#L971) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("queued-input-observed"),"durable").unwrap` | [935](../../src/process_live_tests.rs#L935) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("client-error-queue-receipt.json").exists` | [937](../../src/process_live_tests.rs#L937) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::thread::sleep` | [939](../../src/process_live_tests.rs#L939) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_millis` | [939](../../src/process_live_tests.rs#L939) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.shutdown` | [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `handle.begin_drain` | [943](../../src/process_live_tests.rs#L943) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap().unwrap` | [944](../../src/process_live_tests.rs#L944) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap` | [944](../../src/process_live_tests.rs#L944) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::resume_unwind` | [945](../../src/process_live_tests.rs#L945) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [946](../../src/process_live_tests.rs#L946) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map` | [946](../../src/process_live_tests.rs#L946) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).unwrap` | [946](../../src/process_live_tests.rs#L946) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [947](../../src/process_live_tests.rs#L947) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").collect` | [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="turn_open").collect` | [954](../../src/process_live_tests.rs#L954) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="think").unwrap` | [961](../../src/process_live_tests.rs#L961) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find` | [961](../../src/process_live_tests.rs#L961), [965](../../src/process_live_tests.rs#L965), [971](../../src/process_live_tests.rs#L971) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="ask_user_questions").unwrap` | [965](../../src/process_live_tests.rs#L965) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="approval_response" && e["call"]==question_call["call"]).collect` | [966](../../src/process_live_tests.rs#L966) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="output" && e["final_answer"]==true).unwrap` | [971](../../src/process_live_tests.rs#L971) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap()).unwrap` | [974](../../src/process_live_tests.rs#L974) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap` | [974](../../src/process_live_tests.rs#L974) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty` | [974](../../src/process_live_tests.rs#L974) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect` | [981](../../src/process_live_tests.rs#L981) | receiver-type-required |
| `register_local_tools_server` | `std::env::var` | [981](../../src/process_live_tests.rs#L981) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `fs::create_dir_all(root.join("config")).unwrap` | [982](../../src/process_live_tests.rs#L982) | receiver-type-required |
| `register_local_tools_server` | `fs::create_dir_all` | [982](../../src/process_live_tests.rs#L982) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `root.join` | [982](../../src/process_live_tests.rs#L982), [983](../../src/process_live_tests.rs#L983) | receiver-type-required |
| `register_local_tools_server` | `mcp::McpRegistryStore::new` | [983](../../src/process_live_tests.rs#L983) | [mcp::management::McpRegistryStore::new](../../../mcp/src/management.rs#L383) |
| `register_local_tools_server` | `registry.mutate_idempotent("save-local-tools", &mcp::McpManagementMutation::Save {         server: mcp::McpServerConfig {             reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "local".to_owned() },             transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "local-tools".to_owned()], cwd: None, environment: BTreeMap::new() },             enabled: true,             always_on,             protocol_mode: mcp::ProtocolMode::Legacy,             owner: None,             plugin_component: None,             project_trusted: true,         },         credential_fields: BTreeMap::new(),     }).expect` | [984](../../src/process_live_tests.rs#L984) | receiver-type-required |
| `register_local_tools_server` | `registry.mutate_idempotent` | [984](../../src/process_live_tests.rs#L984) | receiver-type-required |
| `register_local_tools_server` | `"ws".to_owned` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `register_local_tools_server` | `"local".to_owned` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `register_local_tools_server` | `BTreeMap::new` | [987](../../src/process_live_tests.rs#L987), [995](../../src/process_live_tests.rs#L995) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `PathBuf::from` | [1003](../../src/process_live_tests.rs#L1003) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1003](../../src/process_live_tests.rs#L1003) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var` | [1003](../../src/process_live_tests.rs#L1003), [1008](../../src/process_live_tests.rs#L1008), [1013](../../src/process_live_tests.rs#L1013), [1033](../../src/process_live_tests.rs#L1033), [1034](../../src/process_live_tests.rs#L1034) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.parent().unwrap().join` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent().unwrap` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(&workspace).unwrap` | [1005](../../src/process_live_tests.rs#L1005) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all` | [1005](../../src/process_live_tests.rs#L1005), [1006](../../src/process_live_tests.rs#L1006), [1007](../../src/process_live_tests.rs#L1007), [1029](../../src/process_live_tests.rs#L1029) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::create_dir_all(root.join("config")).unwrap` | [1006](../../src/process_live_tests.rs#L1006) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.join` | [1006](../../src/process_live_tests.rs#L1006), [1007](../../src/process_live_tests.rs#L1007), [1009](../../src/process_live_tests.rs#L1009), [1025](../../src/process_live_tests.rs#L1025), [1028](../../src/process_live_tests.rs#L1028), [1034](../../src/process_live_tests.rs#L1034), [1040](../../src/process_live_tests.rs#L1040), [1048](../../src/process_live_tests.rs#L1048), [1053](../../src/process_live_tests.rs#L1053), [1111](../../src/process_live_tests.rs#L1111) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1007](../../src/process_live_tests.rs#L1007) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice` | [1008](../../src/process_live_tests.rs#L1008) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read` | [1008](../../src/process_live_tests.rs#L1008) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_canonical_test_json` | [1009](../../src/process_live_tests.rs#L1009), [1025](../../src/process_live_tests.rs#L1025) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TOOL_SEARCH_SCENARIO").is_ok_and` | [1013](../../src/process_live_tests.rs#L1013) | receiver-type-required |
| `real_public_deferred_tool_search` | `register_local_tools_server` | [1026](../../src/process_live_tests.rs#L1026) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L980) |
| `real_public_deferred_tool_search` | `root.join("threads").join` | [1028](../../src/process_live_tests.rs#L1028) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1029](../../src/process_live_tests.rs#L1029) | receiver-type-required |
| `real_public_deferred_tool_search` | `folder.join` | [1029](../../src/process_live_tests.rs#L1029), [1030](../../src/process_live_tests.rs#L1030) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_test_genesis` | [1031](../../src/process_live_tests.rs#L1031) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Arc::new` | [1032](../../src/process_live_tests.rs#L1032), [1040](../../src/process_live_tests.rs#L1040) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider::MemorySecretStore::new` | [1032](../../src/process_live_tests.rs#L1032) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_deferred_tool_search` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1033](../../src/process_live_tests.rs#L1033) | receiver-type-required |
| `real_public_deferred_tool_search` | `secrets.publish` | [1033](../../src/process_live_tests.rs#L1033) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str().unwrap` | [1033](../../src/process_live_tests.rs#L1033) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str` | [1033](../../src/process_live_tests.rs#L1033) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1033](../../src/process_live_tests.rs#L1033) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-deferred-tool",root.join(".agent"),secrets).unwrap` | [1034](../../src/process_live_tests.rs#L1034) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store` | [1034](../../src/process_live_tests.rs#L1034) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1034](../../src/process_live_tests.rs#L1034) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new().unwrap` | [1035](../../src/process_live_tests.rs#L1035) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new` | [1035](../../src/process_live_tests.rs#L1035) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1036](../../src/process_live_tests.rs#L1036) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on` | [1036](../../src/process_live_tests.rs#L1036), [1065](../../src/process_live_tests.rs#L1065) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::net::TcpListener::bind` | [1036](../../src/process_live_tests.rs#L1036) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `listener.local_addr().unwrap` | [1037](../../src/process_live_tests.rs#L1037) | receiver-type-required |
| `real_public_deferred_tool_search` | `listener.local_addr` | [1037](../../src/process_live_tests.rs#L1037) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::daemon::assemble_production_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1038](../../src/process_live_tests.rs#L1038) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::daemon::assemble_production_endpoint_host` | [1038](../../src/process_live_tests.rs#L1038) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_public_deferred_tool_search` | `"test".into` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy().into_owned` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy().into_owned` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_public_deferred_tool_search` | `Ok` | [1040](../../src/process_live_tests.rs#L1040) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `"2026-09-04T10:00:00.000Z".into` | [1040](../../src/process_live_tests.rs#L1040) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.clone` | [1040](../../src/process_live_tests.rs#L1040), [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1041](../../src/process_live_tests.rs#L1041) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1041](../../src/process_live_tests.rs#L1041) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_public_deferred_tool_search` | `transport::TransportConfig::loopback` | [1042](../../src/process_live_tests.rs#L1042) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_deferred_tool_search` | `transport::BearerToken::new` | [1042](../../src/process_live_tests.rs#L1042) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_deferred_tool_search` | `host.attach_streams` | [1043](../../src/process_live_tests.rs#L1043) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams().clone` | [1043](../../src/process_live_tests.rs#L1043) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams` | [1043](../../src/process_live_tests.rs#L1043) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery().unwrap` | [1044](../../src/process_live_tests.rs#L1044) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery` | [1044](../../src/process_live_tests.rs#L1044) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.into_server` | [1045](../../src/process_live_tests.rs#L1045) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.handle` | [1046](../../src/process_live_tests.rs#L1046) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.spawn` | [1047](../../src/process_live_tests.rs#L1047) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.serve` | [1047](../../src/process_live_tests.rs#L1047) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1048](../../src/process_live_tests.rs#L1048) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write` | [1048](../../src/process_live_tests.rs#L1048), [1111](../../src/process_live_tests.rs#L1111) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1048](../../src/process_live_tests.rs#L1048) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec` | [1048](../../src/process_live_tests.rs#L1048) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.start_periodic_sweep` | [1049](../../src/process_live_tests.rs#L1049) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::catch_unwind` | [1050](../../src/process_live_tests.rs#L1050) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::panic::AssertUnwindSafe` | [1050](../../src/process_live_tests.rs#L1050) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Instant::now` | [1051](../../src/process_live_tests.rs#L1051) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_secs` | [1051](../../src/process_live_tests.rs#L1051) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.join("client-tool-search-receipt.json").exists` | [1053](../../src/process_live_tests.rs#L1053) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().count` | [1055](../../src/process_live_tests.rs#L1055) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines` | [1055](../../src/process_live_tests.rs#L1055), [1067](../../src/process_live_tests.rs#L1067) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap` | [1055](../../src/process_live_tests.rs#L1055), [1067](../../src/process_live_tests.rs#L1067) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string` | [1055](../../src/process_live_tests.rs#L1055), [1067](../../src/process_live_tests.rs#L1067) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.ensure_running(session).expect` | [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.ensure_running` | [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::thread::sleep` | [1060](../../src/process_live_tests.rs#L1060) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_millis` | [1060](../../src/process_live_tests.rs#L1060) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.shutdown` | [1063](../../src/process_live_tests.rs#L1063) | receiver-type-required |
| `real_public_deferred_tool_search` | `handle.begin_drain` | [1064](../../src/process_live_tests.rs#L1064) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap().unwrap` | [1065](../../src/process_live_tests.rs#L1065) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap` | [1065](../../src/process_live_tests.rs#L1065) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::resume_unwind` | [1066](../../src/process_live_tests.rs#L1066) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1067](../../src/process_live_tests.rs#L1067) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map` | [1067](../../src/process_live_tests.rs#L1067) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(line).unwrap` | [1067](../../src/process_live_tests.rs#L1067) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str` | [1067](../../src/process_live_tests.rs#L1067), [1081](../../src/process_live_tests.rs#L1081) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1068](../../src/process_live_tests.rs#L1068) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter` | [1068](../../src/process_live_tests.rs#L1068), [1086](../../src/process_live_tests.rs#L1086), [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter` | [1068](../../src/process_live_tests.rs#L1068), [1073](../../src/process_live_tests.rs#L1073), [1074](../../src/process_live_tests.rs#L1074), [1075](../../src/process_live_tests.rs#L1075), [1078](../../src/process_live_tests.rs#L1078), [1086](../../src/process_live_tests.rs#L1086), [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from(turn).unwrap` | [1071](../../src/process_live_tests.rs#L1071) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from` | [1071](../../src/process_live_tests.rs#L1071) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="tool_search" && e["turn"]==turn).expect` | [1073](../../src/process_live_tests.rs#L1073) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find` | [1073](../../src/process_live_tests.rs#L1073), [1074](../../src/process_live_tests.rs#L1074), [1075](../../src/process_live_tests.rs#L1075), [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==search["call"] && e["outcome"]=="ok").unwrap` | [1074](../../src/process_live_tests.rs#L1074) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]==deferred_tool && e["turn"]==turn).expect` | [1075](../../src/process_live_tests.rs#L1075) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==call["call"] && e["outcome"]=="ok").unwrap` | [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(output["content"][0]["text"].as_str().unwrap()).unwrap` | [1081](../../src/process_live_tests.rs#L1081) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str().unwrap` | [1081](../../src/process_live_tests.rs#L1081) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str` | [1081](../../src/process_live_tests.rs#L1081) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str().expect` | [1083](../../src/process_live_tests.rs#L1083) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str` | [1083](../../src/process_live_tests.rs#L1083) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="epoch").collect` | [1086](../../src/process_live_tests.rs#L1086) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value(provider.clone()).unwrap` | [1093](../../src/process_live_tests.rs#L1093) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value` | [1093](../../src/process_live_tests.rs#L1093) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider.clone` | [1093](../../src/process_live_tests.rs#L1093) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools().is_some` | [1094](../../src/process_live_tests.rs#L1094) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools` | [1094](../../src/process_live_tests.rs#L1094) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap` | [1094](../../src/process_live_tests.rs#L1094) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile` | [1094](../../src/process_live_tests.rs#L1094) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L888) |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap().to_owned` | [1095](../../src/process_live_tests.rs#L1095) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap` | [1095](../../src/process_live_tests.rs#L1095) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str` | [1095](../../src/process_live_tests.rs#L1095) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find(&#124;e&#124;e["id"]==attempt["epoch"]).unwrap` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap()).unwrap` | [1111](../../src/process_live_tests.rs#L1111) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap` | [1111](../../src/process_live_tests.rs#L1111) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty` | [1111](../../src/process_live_tests.rs#L1111) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `PathBuf::from` | [1118](../../src/process_live_tests.rs#L1118) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1118](../../src/process_live_tests.rs#L1118) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var` | [1118](../../src/process_live_tests.rs#L1118), [1123](../../src/process_live_tests.rs#L1123), [1137](../../src/process_live_tests.rs#L1137), [1138](../../src/process_live_tests.rs#L1138) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.parent().unwrap().join` | [1119](../../src/process_live_tests.rs#L1119) | receiver-type-required |
| `real_public_image_attachment` | `root.parent().unwrap` | [1119](../../src/process_live_tests.rs#L1119) | receiver-type-required |
| `real_public_image_attachment` | `root.parent` | [1119](../../src/process_live_tests.rs#L1119) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(&workspace).unwrap` | [1120](../../src/process_live_tests.rs#L1120) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all` | [1120](../../src/process_live_tests.rs#L1120), [1121](../../src/process_live_tests.rs#L1121), [1122](../../src/process_live_tests.rs#L1122), [1133](../../src/process_live_tests.rs#L1133) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::create_dir_all(root.join("config")).unwrap` | [1121](../../src/process_live_tests.rs#L1121) | receiver-type-required |
| `real_public_image_attachment` | `root.join` | [1121](../../src/process_live_tests.rs#L1121), [1122](../../src/process_live_tests.rs#L1122), [1124](../../src/process_live_tests.rs#L1124), [1130](../../src/process_live_tests.rs#L1130), [1132](../../src/process_live_tests.rs#L1132), [1138](../../src/process_live_tests.rs#L1138), [1144](../../src/process_live_tests.rs#L1144), [1152](../../src/process_live_tests.rs#L1152), [1157](../../src/process_live_tests.rs#L1157), [1184](../../src/process_live_tests.rs#L1184) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1122](../../src/process_live_tests.rs#L1122) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1123](../../src/process_live_tests.rs#L1123) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice` | [1123](../../src/process_live_tests.rs#L1123) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1123](../../src/process_live_tests.rs#L1123) | receiver-type-required |
| `real_public_image_attachment` | `fs::read` | [1123](../../src/process_live_tests.rs#L1123) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1123](../../src/process_live_tests.rs#L1123) | receiver-type-required |
| `real_public_image_attachment` | `write_canonical_test_json` | [1124](../../src/process_live_tests.rs#L1124), [1130](../../src/process_live_tests.rs#L1130) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map(&#124;tool&#124; tool.name).collect` | [1127](../../src/process_live_tests.rs#L1127) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map` | [1127](../../src/process_live_tests.rs#L1127) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter` | [1127](../../src/process_live_tests.rs#L1127) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter` | [1127](../../src/process_live_tests.rs#L1127) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled` | [1127](../../src/process_live_tests.rs#L1127) | [tools::builtin::BuiltinManifest::compiled](../../../tools/src/builtin.rs#L315) |
| `real_public_image_attachment` | `root.join("threads").join` | [1132](../../src/process_live_tests.rs#L1132) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1133](../../src/process_live_tests.rs#L1133) | receiver-type-required |
| `real_public_image_attachment` | `folder.join` | [1133](../../src/process_live_tests.rs#L1133), [1134](../../src/process_live_tests.rs#L1134) | receiver-type-required |
| `real_public_image_attachment` | `write_test_genesis` | [1135](../../src/process_live_tests.rs#L1135) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Arc::new` | [1136](../../src/process_live_tests.rs#L1136), [1144](../../src/process_live_tests.rs#L1144) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `provider::MemorySecretStore::new` | [1136](../../src/process_live_tests.rs#L1136) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_image_attachment` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_image_attachment` | `secrets.publish` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str().unwrap` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-image",root.join(".agent"),secrets).unwrap` | [1138](../../src/process_live_tests.rs#L1138) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store` | [1138](../../src/process_live_tests.rs#L1138) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1138](../../src/process_live_tests.rs#L1138) | receiver-type-required |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new().unwrap` | [1139](../../src/process_live_tests.rs#L1139) | receiver-type-required |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new` | [1139](../../src/process_live_tests.rs#L1139) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1140](../../src/process_live_tests.rs#L1140) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on` | [1140](../../src/process_live_tests.rs#L1140), [1169](../../src/process_live_tests.rs#L1169) | receiver-type-required |
| `real_public_image_attachment` | `tokio::net::TcpListener::bind` | [1140](../../src/process_live_tests.rs#L1140) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `listener.local_addr().unwrap` | [1141](../../src/process_live_tests.rs#L1141) | receiver-type-required |
| `real_public_image_attachment` | `listener.local_addr` | [1141](../../src/process_live_tests.rs#L1141) | receiver-type-required |
| `real_public_image_attachment` | `crate::daemon::assemble_production_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1142](../../src/process_live_tests.rs#L1142) | receiver-type-required |
| `real_public_image_attachment` | `crate::daemon::assemble_production_endpoint_host` | [1142](../../src/process_live_tests.rs#L1142) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_public_image_attachment` | `"test".into` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy().into_owned` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy().into_owned` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_image_attachment` | `Ok` | [1144](../../src/process_live_tests.rs#L1144) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `"2026-09-04T10:00:00.000Z".into` | [1144](../../src/process_live_tests.rs#L1144) | receiver-type-required |
| `real_public_image_attachment` | `host.clone` | [1144](../../src/process_live_tests.rs#L1144), [1145](../../src/process_live_tests.rs#L1145) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1145](../../src/process_live_tests.rs#L1145) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1145](../../src/process_live_tests.rs#L1145) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_public_image_attachment` | `transport::TransportConfig::loopback` | [1146](../../src/process_live_tests.rs#L1146) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_image_attachment` | `transport::BearerToken::new` | [1146](../../src/process_live_tests.rs#L1146) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_image_attachment` | `host.attach_streams` | [1147](../../src/process_live_tests.rs#L1147) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams().clone` | [1147](../../src/process_live_tests.rs#L1147) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams` | [1147](../../src/process_live_tests.rs#L1147) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery().unwrap` | [1148](../../src/process_live_tests.rs#L1148) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery` | [1148](../../src/process_live_tests.rs#L1148) | receiver-type-required |
| `real_public_image_attachment` | `assembly.into_server` | [1149](../../src/process_live_tests.rs#L1149) | receiver-type-required |
| `real_public_image_attachment` | `server.handle` | [1150](../../src/process_live_tests.rs#L1150) | receiver-type-required |
| `real_public_image_attachment` | `runtime.spawn` | [1151](../../src/process_live_tests.rs#L1151) | receiver-type-required |
| `real_public_image_attachment` | `server.serve` | [1151](../../src/process_live_tests.rs#L1151) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1152](../../src/process_live_tests.rs#L1152) | receiver-type-required |
| `real_public_image_attachment` | `fs::write` | [1152](../../src/process_live_tests.rs#L1152), [1184](../../src/process_live_tests.rs#L1184) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1152](../../src/process_live_tests.rs#L1152) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec` | [1152](../../src/process_live_tests.rs#L1152) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.start_periodic_sweep` | [1153](../../src/process_live_tests.rs#L1153) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::catch_unwind` | [1154](../../src/process_live_tests.rs#L1154) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::panic::AssertUnwindSafe` | [1154](../../src/process_live_tests.rs#L1154) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Instant::now` | [1155](../../src/process_live_tests.rs#L1155) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_secs` | [1155](../../src/process_live_tests.rs#L1155) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.join("client-image-receipt.json").exists` | [1157](../../src/process_live_tests.rs#L1157) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().count` | [1159](../../src/process_live_tests.rs#L1159) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines` | [1159](../../src/process_live_tests.rs#L1159), [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap` | [1159](../../src/process_live_tests.rs#L1159), [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string` | [1159](../../src/process_live_tests.rs#L1159), [1171](../../src/process_live_tests.rs#L1171) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.ensure_running(session).expect` | [1160](../../src/process_live_tests.rs#L1160) | receiver-type-required |
| `real_public_image_attachment` | `host.ensure_running` | [1160](../../src/process_live_tests.rs#L1160) | receiver-type-required |
| `real_public_image_attachment` | `std::thread::sleep` | [1164](../../src/process_live_tests.rs#L1164) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_millis` | [1164](../../src/process_live_tests.rs#L1164) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.shutdown` | [1167](../../src/process_live_tests.rs#L1167) | receiver-type-required |
| `real_public_image_attachment` | `handle.begin_drain` | [1168](../../src/process_live_tests.rs#L1168) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap().unwrap` | [1169](../../src/process_live_tests.rs#L1169) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap` | [1169](../../src/process_live_tests.rs#L1169) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::resume_unwind` | [1170](../../src/process_live_tests.rs#L1170) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map` | [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str(line).unwrap` | [1171](../../src/process_live_tests.rs#L1171) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str` | [1171](../../src/process_live_tests.rs#L1171) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1172](../../src/process_live_tests.rs#L1172) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().filter` | [1172](../../src/process_live_tests.rs#L1172) | receiver-type-required |
| `real_public_image_attachment` | `events.iter` | [1172](../../src/process_live_tests.rs#L1172), [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find(&#124;e&#124; e["kind"]=="input").expect` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find(&#124;b&#124; b["type"]=="image").expect` | [1176](../../src/process_live_tests.rs#L1176) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find` | [1176](../../src/process_live_tests.rs#L1176) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter` | [1176](../../src/process_live_tests.rs#L1176) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap` | [1176](../../src/process_live_tests.rs#L1176) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array` | [1176](../../src/process_live_tests.rs#L1176) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAS0lEQVR42u3PQQkAAAgAsetfWiP4FgYrsKZeS0BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEDgsqnc8OJg6Ln3AAAAAElFTkSuQmCC").unwrap` | [1180](../../src/process_live_tests.rs#L1180) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode` | [1180](../../src/process_live_tests.rs#L1180) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str().unwrap` | [1181](../../src/process_live_tests.rs#L1181) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str` | [1181](../../src/process_live_tests.rs#L1181) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap()).unwrap` | [1184](../../src/process_live_tests.rs#L1184) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap` | [1184](../../src/process_live_tests.rs#L1184) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty` | [1184](../../src/process_live_tests.rs#L1184) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap()).unwrap` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice` | [1191](../../src/process_live_tests.rs#L1191), [1197](../../src/process_live_tests.rs#L1197) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_flow_case` | `fs::read` | [1191](../../src/process_live_tests.rs#L1191), [1197](../../src/process_live_tests.rs#L1197), [1334](../../src/process_live_tests.rs#L1334), [1348](../../src/process_live_tests.rs#L1348) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_FLOW_CASE").unwrap` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_flow_case` | `std::env::var` | [1191](../../src/process_live_tests.rs#L1191), [1192](../../src/process_live_tests.rs#L1192), [1197](../../src/process_live_tests.rs#L1197), [1238](../../src/process_live_tests.rs#L1238), [1239](../../src/process_live_tests.rs#L1239), [1281](../../src/process_live_tests.rs#L1281), [1282](../../src/process_live_tests.rs#L1282) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `PathBuf::from` | [1192](../../src/process_live_tests.rs#L1192) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap().join` | [1193](../../src/process_live_tests.rs#L1193) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap` | [1193](../../src/process_live_tests.rs#L1193) | receiver-type-required |
| `real_public_flow_case` | `root.parent` | [1193](../../src/process_live_tests.rs#L1193) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(&workspace).unwrap` | [1194](../../src/process_live_tests.rs#L1194) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all` | [1194](../../src/process_live_tests.rs#L1194), [1195](../../src/process_live_tests.rs#L1195), [1196](../../src/process_live_tests.rs#L1196), [1209](../../src/process_live_tests.rs#L1209), [1218](../../src/process_live_tests.rs#L1218), [1226](../../src/process_live_tests.rs#L1226), [1233](../../src/process_live_tests.rs#L1233), [1276](../../src/process_live_tests.rs#L1276) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::create_dir_all(root.join("config")).unwrap` | [1195](../../src/process_live_tests.rs#L1195) | receiver-type-required |
| `real_public_flow_case` | `root.join` | [1195](../../src/process_live_tests.rs#L1195), [1196](../../src/process_live_tests.rs#L1196), [1198](../../src/process_live_tests.rs#L1198), [1206](../../src/process_live_tests.rs#L1206), [1217](../../src/process_live_tests.rs#L1217), [1226](../../src/process_live_tests.rs#L1226), [1227](../../src/process_live_tests.rs#L1227), [1233](../../src/process_live_tests.rs#L1233), [1234](../../src/process_live_tests.rs#L1234), [1239](../../src/process_live_tests.rs#L1239), [1245](../../src/process_live_tests.rs#L1245), [1253](../../src/process_live_tests.rs#L1253), [1257](../../src/process_live_tests.rs#L1257), [1276](../../src/process_live_tests.rs#L1276), [1277](../../src/process_live_tests.rs#L1277), [1282](../../src/process_live_tests.rs#L1282), [1288](../../src/process_live_tests.rs#L1288), [1296](../../src/process_live_tests.rs#L1296), [1300](../../src/process_live_tests.rs#L1300), [1336](../../src/process_live_tests.rs#L1336), [1354](../../src/process_live_tests.rs#L1354), [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1196](../../src/process_live_tests.rs#L1196) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1197](../../src/process_live_tests.rs#L1197) | receiver-type-required |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1197](../../src/process_live_tests.rs#L1197) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1197](../../src/process_live_tests.rs#L1197) | receiver-type-required |
| `real_public_flow_case` | `write_canonical_test_json` | [1198](../../src/process_live_tests.rs#L1198), [1206](../../src/process_live_tests.rs#L1206), [1222](../../src/process_live_tests.rs#L1222), [1232](../../src/process_live_tests.rs#L1232), [1275](../../src/process_live_tests.rs#L1275) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["max_wall_seconds"].as_u64` | [1203](../../src/process_live_tests.rs#L1203) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect("policy object").insert` | [1204](../../src/process_live_tests.rs#L1204) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect` | [1204](../../src/process_live_tests.rs#L1204) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut` | [1204](../../src/process_live_tests.rs#L1204) | receiver-type-required |
| `real_public_flow_case` | `"max_wall_seconds".to_owned` | [1204](../../src/process_live_tests.rs#L1204) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"].as_object().unwrap` | [1207](../../src/process_live_tests.rs#L1207) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"].as_object` | [1207](../../src/process_live_tests.rs#L1207) | receiver-type-required |
| `real_public_flow_case` | `workspace.join` | [1208](../../src/process_live_tests.rs#L1208) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(target.parent().unwrap()).unwrap` | [1209](../../src/process_live_tests.rs#L1209) | receiver-type-required |
| `real_public_flow_case` | `target.parent().unwrap` | [1209](../../src/process_live_tests.rs#L1209) | receiver-type-required |
| `real_public_flow_case` | `target.parent` | [1209](../../src/process_live_tests.rs#L1209) | receiver-type-required |
| `real_public_flow_case` | `fs::write(target, content.as_str().unwrap()).unwrap` | [1210](../../src/process_live_tests.rs#L1210) | receiver-type-required |
| `real_public_flow_case` | `fs::write` | [1210](../../src/process_live_tests.rs#L1210), [1227](../../src/process_live_tests.rs#L1227), [1234](../../src/process_live_tests.rs#L1234), [1253](../../src/process_live_tests.rs#L1253), [1277](../../src/process_live_tests.rs#L1277), [1296](../../src/process_live_tests.rs#L1296), [1336](../../src/process_live_tests.rs#L1336), [1354](../../src/process_live_tests.rs#L1354), [1359](../../src/process_live_tests.rs#L1359) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `content.as_str().unwrap` | [1210](../../src/process_live_tests.rs#L1210) | receiver-type-required |
| `real_public_flow_case` | `content.as_str` | [1210](../../src/process_live_tests.rs#L1210) | receiver-type-required |
| `real_public_flow_case` | `register_local_tools_server` | [1214](../../src/process_live_tests.rs#L1214) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L980) |
| `real_public_flow_case` | `root.join("threads").join` | [1217](../../src/process_live_tests.rs#L1217) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1218](../../src/process_live_tests.rs#L1218) | receiver-type-required |
| `real_public_flow_case` | `folder.join` | [1218](../../src/process_live_tests.rs#L1218), [1219](../../src/process_live_tests.rs#L1219), [1222](../../src/process_live_tests.rs#L1222), [1232](../../src/process_live_tests.rs#L1232), [1275](../../src/process_live_tests.rs#L1275) | receiver-type-required |
| `real_public_flow_case` | `write_test_genesis` | [1220](../../src/process_live_tests.rs#L1220) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::create_dir_all(root.join(".agent/commands")).unwrap` | [1226](../../src/process_live_tests.rs#L1226), [1233](../../src/process_live_tests.rs#L1233), [1276](../../src/process_live_tests.rs#L1276) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap` | [1227](../../src/process_live_tests.rs#L1227), [1234](../../src/process_live_tests.rs#L1234), [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str().unwrap` | [1227](../../src/process_live_tests.rs#L1227), [1234](../../src/process_live_tests.rs#L1234), [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str` | [1227](../../src/process_live_tests.rs#L1227), [1234](../../src/process_live_tests.rs#L1234), [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `Arc::new` | [1237](../../src/process_live_tests.rs#L1237), [1245](../../src/process_live_tests.rs#L1245), [1280](../../src/process_live_tests.rs#L1280), [1288](../../src/process_live_tests.rs#L1288) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `provider::MemorySecretStore::new` | [1237](../../src/process_live_tests.rs#L1237), [1280](../../src/process_live_tests.rs#L1280) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_flow_case` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1238](../../src/process_live_tests.rs#L1238), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_flow_case` | `secrets.publish` | [1238](../../src/process_live_tests.rs#L1238), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str().unwrap` | [1238](../../src/process_live_tests.rs#L1238), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str` | [1238](../../src/process_live_tests.rs#L1238), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1238](../../src/process_live_tests.rs#L1238), [1281](../../src/process_live_tests.rs#L1281) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).unwrap` | [1239](../../src/process_live_tests.rs#L1239), [1282](../../src/process_live_tests.rs#L1282) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store` | [1239](../../src/process_live_tests.rs#L1239), [1282](../../src/process_live_tests.rs#L1282) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1239](../../src/process_live_tests.rs#L1239), [1282](../../src/process_live_tests.rs#L1282) | receiver-type-required |
| `real_public_flow_case` | `tokio::runtime::Runtime::new().unwrap` | [1240](../../src/process_live_tests.rs#L1240), [1283](../../src/process_live_tests.rs#L1283) | receiver-type-required |
| `real_public_flow_case` | `tokio::runtime::Runtime::new` | [1240](../../src/process_live_tests.rs#L1240), [1283](../../src/process_live_tests.rs#L1283) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1241](../../src/process_live_tests.rs#L1241), [1284](../../src/process_live_tests.rs#L1284) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on` | [1241](../../src/process_live_tests.rs#L1241), [1265](../../src/process_live_tests.rs#L1265), [1284](../../src/process_live_tests.rs#L1284), [1308](../../src/process_live_tests.rs#L1308) | receiver-type-required |
| `real_public_flow_case` | `tokio::net::TcpListener::bind` | [1241](../../src/process_live_tests.rs#L1241), [1284](../../src/process_live_tests.rs#L1284) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `listener.local_addr().unwrap` | [1242](../../src/process_live_tests.rs#L1242), [1285](../../src/process_live_tests.rs#L1285) | receiver-type-required |
| `real_public_flow_case` | `listener.local_addr` | [1242](../../src/process_live_tests.rs#L1242), [1285](../../src/process_live_tests.rs#L1285) | receiver-type-required |
| `real_public_flow_case` | `crate::daemon::assemble_production_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1243](../../src/process_live_tests.rs#L1243), [1286](../../src/process_live_tests.rs#L1286) | receiver-type-required |
| `real_public_flow_case` | `crate::daemon::assemble_production_endpoint_host` | [1243](../../src/process_live_tests.rs#L1243), [1286](../../src/process_live_tests.rs#L1286) | [tekes-supervisor::daemon::assemble_production_endpoint_host](../../src/daemon.rs#L1023) |
| `real_public_flow_case` | `"test".into` | [1244](../../src/process_live_tests.rs#L1244), [1287](../../src/process_live_tests.rs#L1287) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy().into_owned` | [1244](../../src/process_live_tests.rs#L1244), [1287](../../src/process_live_tests.rs#L1287) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy` | [1244](../../src/process_live_tests.rs#L1244), [1287](../../src/process_live_tests.rs#L1287) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy().into_owned` | [1244](../../src/process_live_tests.rs#L1244), [1287](../../src/process_live_tests.rs#L1287) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy` | [1244](../../src/process_live_tests.rs#L1244), [1287](../../src/process_live_tests.rs#L1287) | receiver-type-required |
| `real_public_flow_case` | `Ok` | [1245](../../src/process_live_tests.rs#L1245), [1288](../../src/process_live_tests.rs#L1288) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `"2026-09-04T10:00:00.000Z".into` | [1245](../../src/process_live_tests.rs#L1245), [1288](../../src/process_live_tests.rs#L1288) | receiver-type-required |
| `real_public_flow_case` | `host.clone` | [1245](../../src/process_live_tests.rs#L1245), [1246](../../src/process_live_tests.rs#L1246), [1288](../../src/process_live_tests.rs#L1288), [1289](../../src/process_live_tests.rs#L1289) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1246](../../src/process_live_tests.rs#L1246), [1289](../../src/process_live_tests.rs#L1289) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1246](../../src/process_live_tests.rs#L1246), [1289](../../src/process_live_tests.rs#L1289) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L1670) |
| `real_public_flow_case` | `transport::TransportConfig::loopback` | [1247](../../src/process_live_tests.rs#L1247), [1290](../../src/process_live_tests.rs#L1290) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L111) |
| `real_public_flow_case` | `transport::BearerToken::new` | [1247](../../src/process_live_tests.rs#L1247), [1290](../../src/process_live_tests.rs#L1290) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_flow_case` | `host.attach_streams` | [1248](../../src/process_live_tests.rs#L1248), [1291](../../src/process_live_tests.rs#L1291) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams().clone` | [1248](../../src/process_live_tests.rs#L1248), [1291](../../src/process_live_tests.rs#L1291) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams` | [1248](../../src/process_live_tests.rs#L1248), [1291](../../src/process_live_tests.rs#L1291) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery().unwrap` | [1249](../../src/process_live_tests.rs#L1249), [1292](../../src/process_live_tests.rs#L1292) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery` | [1249](../../src/process_live_tests.rs#L1249), [1292](../../src/process_live_tests.rs#L1292) | receiver-type-required |
| `real_public_flow_case` | `assembly.into_server` | [1250](../../src/process_live_tests.rs#L1250), [1293](../../src/process_live_tests.rs#L1293) | receiver-type-required |
| `real_public_flow_case` | `server.handle` | [1251](../../src/process_live_tests.rs#L1251), [1294](../../src/process_live_tests.rs#L1294) | receiver-type-required |
| `real_public_flow_case` | `runtime.spawn` | [1252](../../src/process_live_tests.rs#L1252), [1295](../../src/process_live_tests.rs#L1295) | receiver-type-required |
| `real_public_flow_case` | `server.serve` | [1252](../../src/process_live_tests.rs#L1252), [1295](../../src/process_live_tests.rs#L1295) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1253](../../src/process_live_tests.rs#L1253), [1296](../../src/process_live_tests.rs#L1296) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1253](../../src/process_live_tests.rs#L1253), [1296](../../src/process_live_tests.rs#L1296) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec` | [1253](../../src/process_live_tests.rs#L1253), [1296](../../src/process_live_tests.rs#L1296) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.start_periodic_sweep` | [1254](../../src/process_live_tests.rs#L1254), [1297](../../src/process_live_tests.rs#L1297) | receiver-type-required |
| `real_public_flow_case` | `std::panic::catch_unwind` | [1255](../../src/process_live_tests.rs#L1255), [1298](../../src/process_live_tests.rs#L1298) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::panic::AssertUnwindSafe` | [1255](../../src/process_live_tests.rs#L1255), [1298](../../src/process_live_tests.rs#L1298) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Instant::now` | [1256](../../src/process_live_tests.rs#L1256), [1299](../../src/process_live_tests.rs#L1299) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_secs` | [1256](../../src/process_live_tests.rs#L1256), [1299](../../src/process_live_tests.rs#L1299) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64().unwrap_or` | [1256](../../src/process_live_tests.rs#L1256), [1299](../../src/process_live_tests.rs#L1299) | receiver-type-required |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64` | [1256](../../src/process_live_tests.rs#L1256), [1299](../../src/process_live_tests.rs#L1299) | receiver-type-required |
| `real_public_flow_case` | `root.join("client-flow-receipt.json").exists` | [1257](../../src/process_live_tests.rs#L1257), [1300](../../src/process_live_tests.rs#L1300) | receiver-type-required |
| `real_public_flow_case` | `std::thread::sleep` | [1260](../../src/process_live_tests.rs#L1260), [1303](../../src/process_live_tests.rs#L1303) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_millis` | [1260](../../src/process_live_tests.rs#L1260), [1303](../../src/process_live_tests.rs#L1303) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.shutdown` | [1263](../../src/process_live_tests.rs#L1263), [1306](../../src/process_live_tests.rs#L1306) | receiver-type-required |
| `real_public_flow_case` | `handle.begin_drain` | [1264](../../src/process_live_tests.rs#L1264), [1307](../../src/process_live_tests.rs#L1307) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap().unwrap` | [1265](../../src/process_live_tests.rs#L1265), [1308](../../src/process_live_tests.rs#L1308) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap` | [1265](../../src/process_live_tests.rs#L1265), [1308](../../src/process_live_tests.rs#L1308) | receiver-type-required |
| `real_public_flow_case` | `std::panic::resume_unwind` | [1266](../../src/process_live_tests.rs#L1266), [1309](../../src/process_live_tests.rs#L1309) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_str(line).unwrap` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_str` | [1267](../../src/process_live_tests.rs#L1267), [1310](../../src/process_live_tests.rs#L1310) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1268](../../src/process_live_tests.rs#L1268), [1311](../../src/process_live_tests.rs#L1311) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter` | [1268](../../src/process_live_tests.rs#L1268), [1311](../../src/process_live_tests.rs#L1311), [1322](../../src/process_live_tests.rs#L1322), [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `events.iter` | [1268](../../src/process_live_tests.rs#L1268), [1311](../../src/process_live_tests.rs#L1311), [1320](../../src/process_live_tests.rs#L1320), [1322](../../src/process_live_tests.rs#L1322), [1342](../../src/process_live_tests.rs#L1342), [1345](../../src/process_live_tests.rs#L1345), [1347](../../src/process_live_tests.rs#L1347), [1350](../../src/process_live_tests.rs#L1350), [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="compact" && e["origin_tuple"]["op"]=="commands/run").expect` | [1320](../../src/process_live_tests.rs#L1320), [1342](../../src/process_live_tests.rs#L1342) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find` | [1320](../../src/process_live_tests.rs#L1320), [1342](../../src/process_live_tests.rs#L1342), [1345](../../src/process_live_tests.rs#L1345), [1347](../../src/process_live_tests.rs#L1347), [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap().len` | [1321](../../src/process_live_tests.rs#L1321) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap` | [1321](../../src/process_live_tests.rs#L1321) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array` | [1321](../../src/process_live_tests.rs#L1321) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="settle" && e["seq"].as_u64() < compact["seq"].as_u64()).count` | [1322](../../src/process_live_tests.rs#L1322) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64` | [1322](../../src/process_live_tests.rs#L1322), [1345](../../src/process_live_tests.rs#L1345), [1347](../../src/process_live_tests.rs#L1347), [1350](../../src/process_live_tests.rs#L1350), [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `compact["seq"].as_u64` | [1322](../../src/process_live_tests.rs#L1322), [1343](../../src/process_live_tests.rs#L1343) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array().expect` | [1328](../../src/process_live_tests.rs#L1328) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array` | [1328](../../src/process_live_tests.rs#L1328) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map(&#124;r&#124; r["from"].as_u64().unwrap()..=r["to"].as_u64().unwrap()).collect` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64().unwrap` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64().unwrap` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64` | [1330](../../src/process_live_tests.rs#L1330) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str().map` | [1332](../../src/process_live_tests.rs#L1332) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str` | [1332](../../src/process_live_tests.rs#L1332) | receiver-type-required |
| `real_public_flow_case` | `summary.starts_with` | [1332](../../src/process_live_tests.rs#L1332) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger(&fs::read(&path).unwrap(), 1).unwrap` | [1334](../../src/process_live_tests.rs#L1334), [1348](../../src/process_live_tests.rs#L1348) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger` | [1334](../../src/process_live_tests.rs#L1334), [1348](../../src/process_live_tests.rs#L1348) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1054) |
| `real_public_flow_case` | `fs::read(&path).unwrap` | [1334](../../src/process_live_tests.rs#L1334), [1348](../../src/process_live_tests.rs#L1348) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compaction-summary.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap()).unwrap` | [1336](../../src/process_live_tests.rs#L1336) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap` | [1336](../../src/process_live_tests.rs#L1336) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty` | [1336](../../src/process_live_tests.rs#L1336), [1354](../../src/process_live_tests.rs#L1354), [1359](../../src/process_live_tests.rs#L1359) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `compact["seq"].as_u64().unwrap` | [1343](../../src/process_live_tests.rs#L1343) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap().to_owned` | [1344](../../src/process_live_tests.rs#L1344) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap` | [1344](../../src/process_live_tests.rs#L1344) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str` | [1344](../../src/process_live_tests.rs#L1344) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="epoch" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1345](../../src/process_live_tests.rs#L1345) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64().unwrap` | [1345](../../src/process_live_tests.rs#L1345), [1347](../../src/process_live_tests.rs#L1347), [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64().unwrap` | [1347](../../src/process_live_tests.rs#L1347) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64` | [1347](../../src/process_live_tests.rs#L1347) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1347](../../src/process_live_tests.rs#L1347) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt(&typed.events, &origin_key, &[post_input]).expect` | [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt` | [1349](../../src/process_live_tests.rs#L1349) | [engine::compact_gate::first_post_compact_attempt](../../../engine/src/compact_gate.rs#L20) |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap().to_owned` | [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap` | [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str` | [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap` | [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `Some` | [1350](../../src/process_live_tests.rs#L1350) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map(&#124;e&#124; e["call"].as_str().map(str::to_owned)).collect` | [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map` | [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str().map` | [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str` | [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compact-gate.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap()).unwrap` | [1354](../../src/process_live_tests.rs#L1354) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap` | [1354](../../src/process_live_tests.rs#L1354) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap()).unwrap` | [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap` | [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_mcp_task_continuation` | `PathBuf::from` | [1372](../../src/process_live_tests.rs#L1372) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1372](../../src/process_live_tests.rs#L1372) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var` | [1372](../../src/process_live_tests.rs#L1372), [1378](../../src/process_live_tests.rs#L1378), [1381](../../src/process_live_tests.rs#L1381), [1406](../../src/process_live_tests.rs#L1406), [1407](../../src/process_live_tests.rs#L1407) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&root).unwrap` | [1373](../../src/process_live_tests.rs#L1373) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all` | [1373](../../src/process_live_tests.rs#L1373), [1375](../../src/process_live_tests.rs#L1375), [1376](../../src/process_live_tests.rs#L1376), [1377](../../src/process_live_tests.rs#L1377), [1398](../../src/process_live_tests.rs#L1398) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `root.parent().unwrap().join` | [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent().unwrap` | [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent` | [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&workspace).unwrap` | [1375](../../src/process_live_tests.rs#L1375) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(root.join("config")).unwrap` | [1376](../../src/process_live_tests.rs#L1376) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.join` | [1376](../../src/process_live_tests.rs#L1376), [1377](../../src/process_live_tests.rs#L1377), [1379](../../src/process_live_tests.rs#L1379), [1380](../../src/process_live_tests.rs#L1380), [1382](../../src/process_live_tests.rs#L1382), [1397](../../src/process_live_tests.rs#L1397), [1407](../../src/process_live_tests.rs#L1407), [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(root.join("workspaces/ws")).unwrap` | [1377](../../src/process_live_tests.rs#L1377) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice` | [1378](../../src/process_live_tests.rs#L1378) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read` | [1378](../../src/process_live_tests.rs#L1378) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_canonical_test_json` | [1379](../../src/process_live_tests.rs#L1379), [1380](../../src/process_live_tests.rs#L1380) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_TEST_MCP_FIXTURE_SERVER").expect` | [1381](../../src/process_live_tests.rs#L1381) | receiver-type-required |
| `real_public_mcp_task_continuation` | `mcp::McpRegistryStore::new` | [1382](../../src/process_live_tests.rs#L1382) | [mcp::management::McpRegistryStore::new](../../../mcp/src/management.rs#L383) |
| `real_public_mcp_task_continuation` | `registry.mutate_idempotent("save-fixture", &mcp::McpManagementMutation::Save {         server: mcp::McpServerConfig {             reference: mcp::McpServerReference { workspace_id: "ws".to_owned(), scope: mcp::McpScope::User, name: "fixture".to_owned() },             transport: mcp::McpTransportConfig::Stdio { command: vec![fixture, "task-augmented".to_owned()], cwd: None, environment: BTreeMap::new() },             enabled: true,             always_on: true,             protocol_mode: mcp::ProtocolMode::Legacy,             owner: None,             plugin_component: None,             project_trusted: true,         },         credential_fields: BTreeMap::new(),     }).expect` | [1383](../../src/process_live_tests.rs#L1383) | receiver-type-required |
| `real_public_mcp_task_continuation` | `registry.mutate_idempotent` | [1383](../../src/process_live_tests.rs#L1383) | receiver-type-required |
| `real_public_mcp_task_continuation` | `"ws".to_owned` | [1385](../../src/process_live_tests.rs#L1385) | receiver-type-required |
| `real_public_mcp_task_continuation` | `"fixture".to_owned` | [1385](../../src/process_live_tests.rs#L1385) | receiver-type-required |
| `real_public_mcp_task_continuation` | `BTreeMap::new` | [1386](../../src/process_live_tests.rs#L1386), [1394](../../src/process_live_tests.rs#L1394) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `root.join("threads").join` | [1397](../../src/process_live_tests.rs#L1397) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_mcp_task_continuation` | `folder.join` | [1398](../../src/process_live_tests.rs#L1398), [1399](../../src/process_live_tests.rs#L1399) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_test_genesis` | [1400](../../src/process_live_tests.rs#L1400) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `append_test_input` | [1401](../../src/process_live_tests.rs#L1401) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [1402](../../src/process_live_tests.rs#L1402), [1421](../../src/process_live_tests.rs#L1421) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map` | [1402](../../src/process_live_tests.rs#L1402), [1421](../../src/process_live_tests.rs#L1421) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines` | [1402](../../src/process_live_tests.rs#L1402), [1413](../../src/process_live_tests.rs#L1413), [1421](../../src/process_live_tests.rs#L1421) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap` | [1402](../../src/process_live_tests.rs#L1402), [1413](../../src/process_live_tests.rs#L1413), [1421](../../src/process_live_tests.rs#L1421) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string` | [1402](../../src/process_live_tests.rs#L1402), [1413](../../src/process_live_tests.rs#L1413), [1421](../../src/process_live_tests.rs#L1421) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).unwrap` | [1402](../../src/process_live_tests.rs#L1402), [1421](../../src/process_live_tests.rs#L1421) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str` | [1402](../../src/process_live_tests.rs#L1402), [1413](../../src/process_live_tests.rs#L1413), [1421](../../src/process_live_tests.rs#L1421) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `write_test_events` | [1404](../../src/process_live_tests.rs#L1404) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Arc::new` | [1405](../../src/process_live_tests.rs#L1405) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `provider::MemorySecretStore::new` | [1405](../../src/process_live_tests.rs#L1405) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_mcp_task_continuation` | `secrets.publish(provider["credential_key"].as_str().unwrap(), provider::SecretRecord::Active { generation: 1, material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap() }).unwrap` | [1406](../../src/process_live_tests.rs#L1406) | receiver-type-required |
| `real_public_mcp_task_continuation` | `secrets.publish` | [1406](../../src/process_live_tests.rs#L1406) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str().unwrap` | [1406](../../src/process_live_tests.rs#L1406) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str` | [1406](../../src/process_live_tests.rs#L1406) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1406](../../src/process_live_tests.rs#L1406) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store(&root, std::env::var("TEKES_TEST_REAL_WORKER").unwrap(), "live-mcp-task", root.join(".agent"), secrets).unwrap` | [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store` | [1407](../../src/process_live_tests.rs#L1407) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1407](../../src/process_live_tests.rs#L1407) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::panic::catch_unwind` | [1408](../../src/process_live_tests.rs#L1408) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::panic::AssertUnwindSafe` | [1408](../../src/process_live_tests.rs#L1408) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap().expect` | [1409](../../src/process_live_tests.rs#L1409) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap` | [1409](../../src/process_live_tests.rs#L1409) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main` | [1409](../../src/process_live_tests.rs#L1409) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.start_periodic_sweep` | [1410](../../src/process_live_tests.rs#L1410) | receiver-type-required |
| `real_public_mcp_task_continuation` | `Instant::now` | [1411](../../src/process_live_tests.rs#L1411) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_secs` | [1411](../../src/process_live_tests.rs#L1411) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [1413](../../src/process_live_tests.rs#L1413) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [1413](../../src/process_live_tests.rs#L1413) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).ok` | [1413](../../src/process_live_tests.rs#L1413) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().any` | [1414](../../src/process_live_tests.rs#L1414) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter` | [1414](../../src/process_live_tests.rs#L1414), [1422](../../src/process_live_tests.rs#L1422), [1424](../../src/process_live_tests.rs#L1424), [1427](../../src/process_live_tests.rs#L1427), [1434](../../src/process_live_tests.rs#L1434), [1438](../../src/process_live_tests.rs#L1438), [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::thread::sleep` | [1416](../../src/process_live_tests.rs#L1416) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_millis` | [1416](../../src/process_live_tests.rs#L1416) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.shutdown` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::panic::resume_unwind` | [1420](../../src/process_live_tests.rs#L1420) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `events.iter().find(&#124;e&#124; e["kind"] == "settle").unwrap` | [1422](../../src/process_live_tests.rs#L1422) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().find` | [1422](../../src/process_live_tests.rs#L1422) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "mcp__fixture__echo").collect` | [1424](../../src/process_live_tests.rs#L1424) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter` | [1424](../../src/process_live_tests.rs#L1424), [1427](../../src/process_live_tests.rs#L1427), [1434](../../src/process_live_tests.rs#L1434), [1438](../../src/process_live_tests.rs#L1438) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str().unwrap` | [1426](../../src/process_live_tests.rs#L1426) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str` | [1426](../../src/process_live_tests.rs#L1426) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "state" && e["subkind"] == "tool_continuation" && e["payload"]["call"] == call).collect` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map(&#124;e&#124; e["payload"]["action"].as_str().unwrap()).collect` | [1428](../../src/process_live_tests.rs#L1428) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map` | [1428](../../src/process_live_tests.rs#L1428) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter` | [1428](../../src/process_live_tests.rs#L1428), [1431](../../src/process_live_tests.rs#L1431) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str().unwrap` | [1428](../../src/process_live_tests.rs#L1428) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str` | [1428](../../src/process_live_tests.rs#L1428) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find(&#124;e&#124; e["payload"]["action"] == "park").unwrap` | [1431](../../src/process_live_tests.rs#L1431) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find` | [1431](../../src/process_live_tests.rs#L1431) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "run_start").collect` | [1434](../../src/process_live_tests.rs#L1434) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64().unwrap` | [1436](../../src/process_live_tests.rs#L1436) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64` | [1436](../../src/process_live_tests.rs#L1436) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_result" && e["call"] == call).collect` | [1438](../../src/process_live_tests.rs#L1438) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str().unwrap` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find(&#124;e&#124; e["kind"] == "output").unwrap` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write(root.join("receipt.json"), serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap()).unwrap` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write` | [1447](../../src/process_live_tests.rs#L1447) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap` | [1447](../../src/process_live_tests.rs#L1447) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty` | [1447](../../src/process_live_tests.rs#L1447) | external-constructor-callback-or-unresolved |
