# tekes-supervisor::process_live_tests

[Package atlas](index.md) · [Source](../../src/process_live_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::process_live_tests::real_worker_repeated_context_releases_queued_body](../../src/process_live_tests.rs#L4) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_process_promotes_frozen_candidate](../../src/process_live_tests.rs#L181) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_feedback_repairs_same_worker](../../src/process_live_tests.rs#L188) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L193) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::delivery_fallback_requires_confirmed_child_exit](../../src/process_live_tests.rs#L435) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::approval_closed_stdin_waits_for_actual_exit_before_fallback](../../src/process_live_tests.rs#L461) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::cancel_stage_failure_names_the_stage_and_keeps_typed_failures](../../src/process_live_tests.rs#L533) | function_item | `private` | test;  |
| [tekes-supervisor::process_live_tests::cancel_after_the_worker_closed_stdin_returns_the_durable_receipt](../../src/process_live_tests.rs#L560) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_validator_exhaustion_releases_queued_input](../../src/process_live_tests.rs#L640) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_legacy_simple_task](../../src/process_live_tests.rs#L830) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_provider_400_releases_queue_over_public_transport](../../src/process_live_tests.rs#L983) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1066) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_deferred_tool_search](../../src/process_live_tests.rs#L1073) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_image_attachment](../../src/process_live_tests.rs#L1187) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_flow_case](../../src/process_live_tests.rs#L1259) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::real_public_mcp_task_continuation](../../src/process_live_tests.rs#L1439) | function_item | `private` | test; #[cfg(target_os = "macos")] |
| [tekes-supervisor::process_live_tests::stage_live_stdio](../../src/process_live_tests.rs#L1504) | function_item | `private` | test; #[cfg(target_os="macos")] |
| [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) | function_item | `private` | test; #[cfg(target_os="macos")] |

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
| `real_worker_repeated_context_releases_queued_body` | `std::env::var` | [5](../../src/process_live_tests.rs#L5), [11](../../src/process_live_tests.rs#L11), [43](../../src/process_live_tests.rs#L43), [49](../../src/process_live_tests.rs#L49) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(&root).unwrap` | [6](../../src/process_live_tests.rs#L6) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all` | [6](../../src/process_live_tests.rs#L6), [8](../../src/process_live_tests.rs#L8), [9](../../src/process_live_tests.rs#L9), [26](../../src/process_live_tests.rs#L26) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `root.parent().unwrap().join` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.parent().unwrap` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.parent` | [7](../../src/process_live_tests.rs#L7) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(&workspace).unwrap` | [8](../../src/process_live_tests.rs#L8) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(root.join("config")).unwrap` | [9](../../src/process_live_tests.rs#L9) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `root.join` | [9](../../src/process_live_tests.rs#L9), [15](../../src/process_live_tests.rs#L15), [25](../../src/process_live_tests.rs#L25), [51](../../src/process_live_tests.rs#L51), [175](../../src/process_live_tests.rs#L175) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [10](../../src/process_live_tests.rs#L10) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_slice` | [10](../../src/process_live_tests.rs#L10) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [11](../../src/process_live_tests.rs#L11) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read` | [11](../../src/process_live_tests.rs#L11) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [11](../../src/process_live_tests.rs#L11) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `write_canonical_test_json` | [14](../../src/process_live_tests.rs#L14) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `write_test_execution_config` | [18](../../src/process_live_tests.rs#L18) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `root.join("threads").join` | [25](../../src/process_live_tests.rs#L25) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::create_dir_all(folder.join("assets")).unwrap` | [26](../../src/process_live_tests.rs#L26) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `folder.join` | [26](../../src/process_live_tests.rs#L26), [27](../../src/process_live_tests.rs#L27) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `write_test_genesis` | [28](../../src/process_live_tests.rs#L28) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `append_test_input` | [29](../../src/process_live_tests.rs#L29) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [30](../../src/process_live_tests.rs#L30), [107](../../src/process_live_tests.rs#L107) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map` | [30](../../src/process_live_tests.rs#L30), [107](../../src/process_live_tests.rs#L107) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap()         .lines` | [30](../../src/process_live_tests.rs#L30), [107](../../src/process_live_tests.rs#L107) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)         .unwrap` | [30](../../src/process_live_tests.rs#L30), [107](../../src/process_live_tests.rs#L107) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string` | [30](../../src/process_live_tests.rs#L30), [63](../../src/process_live_tests.rs#L63), [107](../../src/process_live_tests.rs#L107) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str(s).unwrap` | [33](../../src/process_live_tests.rs#L33), [110](../../src/process_live_tests.rs#L110) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str` | [33](../../src/process_live_tests.rs#L33), [66](../../src/process_live_tests.rs#L66), [110](../../src/process_live_tests.rs#L110) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `write_test_events` | [36](../../src/process_live_tests.rs#L36) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Arc::new` | [37](../../src/process_live_tests.rs#L37) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `provider::MemorySecretStore::new` | [37](../../src/process_live_tests.rs#L37) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_worker_repeated_context_releases_queued_body` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [38](../../src/process_live_tests.rs#L38) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `secrets         .publish` | [38](../../src/process_live_tests.rs#L38) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `provider["credential_key"].as_str().unwrap` | [40](../../src/process_live_tests.rs#L40) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `provider["credential_key"].as_str` | [40](../../src/process_live_tests.rs#L40) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [43](../../src/process_live_tests.rs#L43) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-process",         root.join(".agent"),         secrets,     ).inspect(&#124;host&#124; { attach_live_stdio(host,&root); })     .unwrap` | [47](../../src/process_live_tests.rs#L47) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-process",         root.join(".agent"),         secrets,     ).inspect` | [47](../../src/process_live_tests.rs#L47) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `ProductionProcessHost::open_with_secret_store` | [47](../../src/process_live_tests.rs#L47) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [49](../../src/process_live_tests.rs#L49) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `attach_live_stdio` | [53](../../src/process_live_tests.rs#L53) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::catch_unwind` | [55](../../src/process_live_tests.rs#L55) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::AssertUnwindSafe` | [55](../../src/process_live_tests.rs#L55) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main(session)             .unwrap()             .expect` | [56](../../src/process_live_tests.rs#L56) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main(session)             .unwrap` | [56](../../src/process_live_tests.rs#L56) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.schedule_main` | [56](../../src/process_live_tests.rs#L56) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.start_periodic_sweep` | [59](../../src/process_live_tests.rs#L59) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `Instant::now` | [60](../../src/process_live_tests.rs#L60) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Duration::from_secs` | [60](../../src/process_live_tests.rs#L60) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map(&#124;s&#124; serde_json::from_str(s).ok())                 .collect` | [63](../../src/process_live_tests.rs#L63) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map` | [63](../../src/process_live_tests.rs#L63) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap()                 .lines` | [63](../../src/process_live_tests.rs#L63) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::read_to_string(&path)                 .unwrap` | [63](../../src/process_live_tests.rs#L63) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::from_str(s).ok` | [66](../../src/process_live_tests.rs#L66) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().any` | [68](../../src/process_live_tests.rs#L68) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter` | [68](../../src/process_live_tests.rs#L68), [92](../../src/process_live_tests.rs#L92) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"test".into` | [70](../../src/process_live_tests.rs#L70) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"live-process".into` | [71](../../src/process_live_tests.rs#L71) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `session.into` | [72](../../src/process_live_tests.rs#L72) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"session.prompt".into` | [73](../../src/process_live_tests.rs#L73) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `"queued-body".into` | [74](../../src/process_live_tests.rs#L74) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.prompt(                     session,                     "2026-09-04T10:00:00.000Z",                     &origin,                     &MaterializedPrompt {                         blocks: vec![Block::Text {                             text: "重试".into(),                         }],                         attachments: vec![],                         files: Vec::new(),                     },                     false,                 )                 .unwrap` | [76](../../src/process_live_tests.rs#L76) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `host.prompt` | [76](../../src/process_live_tests.rs#L76) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `Vec::new` | [85](../../src/process_live_tests.rs#L85) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().filter(&#124;e&#124; e["kind"] == "settle").count` | [92](../../src/process_live_tests.rs#L92) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events.iter().filter` | [92](../../src/process_live_tests.rs#L92) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::thread::sleep` | [100](../../src/process_live_tests.rs#L100) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `Duration::from_millis` | [100](../../src/process_live_tests.rs#L100) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `host.shutdown` | [103](../../src/process_live_tests.rs#L103) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `std::panic::resume_unwind` | [105](../../src/process_live_tests.rs#L105) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 1)         .unwrap` | [113](../../src/process_live_tests.rs#L113) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find` | [113](../../src/process_live_tests.rs#L113), [117](../../src/process_live_tests.rs#L117), [131](../../src/process_live_tests.rs#L131), [135](../../src/process_live_tests.rs#L135) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter` | [113](../../src/process_live_tests.rs#L113), [117](../../src/process_live_tests.rs#L117), [131](../../src/process_live_tests.rs#L131), [135](../../src/process_live_tests.rs#L135), [142](../../src/process_live_tests.rs#L142) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 2)         .unwrap` | [117](../../src/process_live_tests.rs#L117) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find(&#124;e&#124; e["seq"] == settle["promoted_output_seq"])             .unwrap` | [124](../../src/process_live_tests.rs#L124) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find` | [124](../../src/process_live_tests.rs#L124), [153](../../src/process_live_tests.rs#L153) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter` | [124](../../src/process_live_tests.rs#L124), [153](../../src/process_live_tests.rs#L153) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "turn_open" && e["turn"] == 2)         .unwrap` | [131](../../src/process_live_tests.rs#L131) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .find(&#124;e&#124; e["kind"] == "input" && e["content"][0]["text"] == "重试")         .unwrap` | [135](../../src/process_live_tests.rs#L135) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .filter(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "context_get")         .collect` | [142](../../src/process_live_tests.rs#L142) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events         .iter()         .filter` | [142](../../src/process_live_tests.rs#L142) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.iter().enumerate` | [152](../../src/process_live_tests.rs#L152) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.iter` | [152](../../src/process_live_tests.rs#L152) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `events             .iter()             .find(&#124;e&#124; e["kind"] == "tool_result" && e["call"] == call["call"])             .unwrap` | [153](../../src/process_live_tests.rs#L153) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls             .get(index + 1)             .map_or` | [159](../../src/process_live_tests.rs#L159) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls             .get` | [159](../../src/process_live_tests.rs#L159) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `first["seq"].as_u64().unwrap` | [161](../../src/process_live_tests.rs#L161) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `first["seq"].as_u64` | [161](../../src/process_live_tests.rs#L161) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `next["seq"].as_u64().unwrap` | [162](../../src/process_live_tests.rs#L162) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `next["seq"].as_u64` | [162](../../src/process_live_tests.rs#L162) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `calls.get` | [168](../../src/process_live_tests.rs#L168) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"Real worker process and supervisor context_get, four model responses then queued ordinary input; no artifact validator or external client transport.","first_settle":first["seq"],"second_settle":second["seq"]})).unwrap()).unwrap` | [175](../../src/process_live_tests.rs#L175) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `fs::write` | [175](../../src/process_live_tests.rs#L175) | external-constructor-callback-or-unresolved |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::to_vec_pretty(&json!({"status":"passed","scope":"Real worker process and supervisor context_get, four model responses then queued ordinary input; no artifact validator or external client transport.","first_settle":first["seq"],"second_settle":second["seq"]})).unwrap` | [175](../../src/process_live_tests.rs#L175) | receiver-type-required |
| `real_worker_repeated_context_releases_queued_body` | `serde_json::to_vec_pretty` | [175](../../src/process_live_tests.rs#L175) | external-constructor-callback-or-unresolved |
| `real_validator_process_promotes_frozen_candidate` | `run_real_validator_process` | [182](../../src/process_live_tests.rs#L182) | [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L193) |
| `real_validator_feedback_repairs_same_worker` | `run_real_validator_process` | [189](../../src/process_live_tests.rs#L189) | [tekes-supervisor::process_live_tests::run_real_validator_process](../../src/process_live_tests.rs#L193) |
| `run_real_validator_process` | `PathBuf::from` | [194](../../src/process_live_tests.rs#L194), [318](../../src/process_live_tests.rs#L318) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [194](../../src/process_live_tests.rs#L194) | receiver-type-required |
| `run_real_validator_process` | `std::env::var` | [194](../../src/process_live_tests.rs#L194), [200](../../src/process_live_tests.rs#L200), [272](../../src/process_live_tests.rs#L272), [278](../../src/process_live_tests.rs#L278) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::create_dir_all(&root).unwrap` | [195](../../src/process_live_tests.rs#L195) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all` | [195](../../src/process_live_tests.rs#L195), [197](../../src/process_live_tests.rs#L197), [198](../../src/process_live_tests.rs#L198), [217](../../src/process_live_tests.rs#L217) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `root.parent().unwrap().join` | [196](../../src/process_live_tests.rs#L196) | receiver-type-required |
| `run_real_validator_process` | `root.parent().unwrap` | [196](../../src/process_live_tests.rs#L196) | receiver-type-required |
| `run_real_validator_process` | `root.parent` | [196](../../src/process_live_tests.rs#L196) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(&workspace).unwrap` | [197](../../src/process_live_tests.rs#L197) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(root.join("config")).unwrap` | [198](../../src/process_live_tests.rs#L198) | receiver-type-required |
| `run_real_validator_process` | `root.join` | [198](../../src/process_live_tests.rs#L198), [204](../../src/process_live_tests.rs#L204), [216](../../src/process_live_tests.rs#L216), [280](../../src/process_live_tests.rs#L280), [430](../../src/process_live_tests.rs#L430) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [199](../../src/process_live_tests.rs#L199) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_slice` | [199](../../src/process_live_tests.rs#L199) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [200](../../src/process_live_tests.rs#L200) | receiver-type-required |
| `run_real_validator_process` | `fs::read` | [200](../../src/process_live_tests.rs#L200) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [200](../../src/process_live_tests.rs#L200) | receiver-type-required |
| `run_real_validator_process` | `write_canonical_test_json` | [203](../../src/process_live_tests.rs#L203) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `write_test_execution_config` | [207](../../src/process_live_tests.rs#L207) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `root.join("threads").join` | [216](../../src/process_live_tests.rs#L216) | receiver-type-required |
| `run_real_validator_process` | `fs::create_dir_all(folder.join("assets")).unwrap` | [217](../../src/process_live_tests.rs#L217) | receiver-type-required |
| `run_real_validator_process` | `folder.join` | [217](../../src/process_live_tests.rs#L217), [218](../../src/process_live_tests.rs#L218), [234](../../src/process_live_tests.rs#L234), [351](../../src/process_live_tests.rs#L351), [414](../../src/process_live_tests.rs#L414) | receiver-type-required |
| `run_real_validator_process` | `write_test_genesis` | [219](../../src/process_live_tests.rs#L219) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `append_test_input` | [220](../../src/process_live_tests.rs#L220) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [221](../../src/process_live_tests.rs#L221), [383](../../src/process_live_tests.rs#L383) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines()         .map` | [221](../../src/process_live_tests.rs#L221), [383](../../src/process_live_tests.rs#L383) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap()         .lines` | [221](../../src/process_live_tests.rs#L221), [383](../../src/process_live_tests.rs#L383) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)         .unwrap` | [221](../../src/process_live_tests.rs#L221), [383](../../src/process_live_tests.rs#L383) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string` | [221](../../src/process_live_tests.rs#L221), [298](../../src/process_live_tests.rs#L298), [351](../../src/process_live_tests.rs#L351), [383](../../src/process_live_tests.rs#L383), [415](../../src/process_live_tests.rs#L415) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `serde_json::from_str(s).unwrap` | [224](../../src/process_live_tests.rs#L224), [386](../../src/process_live_tests.rs#L386), [418](../../src/process_live_tests.rs#L418) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str` | [224](../../src/process_live_tests.rs#L224), [301](../../src/process_live_tests.rs#L301), [354](../../src/process_live_tests.rs#L354), [386](../../src/process_live_tests.rs#L386), [418](../../src/process_live_tests.rs#L418) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `workspace.join` | [226](../../src/process_live_tests.rs#L226), [322](../../src/process_live_tests.rs#L322) | receiver-type-required |
| `run_real_validator_process` | `b"WRONG".as_slice` | [228](../../src/process_live_tests.rs#L228) | receiver-type-required |
| `run_real_validator_process` | `b"KERNEL_VALIDATOR_OK".as_slice` | [230](../../src/process_live_tests.rs#L230) | receiver-type-required |
| `run_real_validator_process` | `fs::write(&artifact, initial).unwrap` | [232](../../src/process_live_tests.rs#L232) | receiver-type-required |
| `run_real_validator_process` | `fs::write` | [232](../../src/process_live_tests.rs#L232), [430](../../src/process_live_tests.rs#L430) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap()         .publish(b"{}")         .unwrap` | [234](../../src/process_live_tests.rs#L234) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap()         .publish` | [234](../../src/process_live_tests.rs#L234) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new(folder.join("assets"))         .unwrap` | [234](../../src/process_live_tests.rs#L234) | receiver-type-required |
| `run_real_validator_process` | `AssetStore::new` | [234](../../src/process_live_tests.rs#L234) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `events.extend` | [238](../../src/process_live_tests.rs#L238) | receiver-type-required |
| `run_real_validator_process` | `events.iter_mut().enumerate` | [245](../../src/process_live_tests.rs#L245) | receiver-type-required |
| `run_real_validator_process` | `events.iter_mut` | [245](../../src/process_live_tests.rs#L245) | receiver-type-required |
| `run_real_validator_process` | `write_test_events` | [250](../../src/process_live_tests.rs#L250) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `store::LockedLedger::open(&path, schema::READER_VERSION).unwrap` | [251](../../src/process_live_tests.rs#L251) | receiver-type-required |
| `run_real_validator_process` | `store::LockedLedger::open` | [251](../../src/process_live_tests.rs#L251) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L186) |
| `run_real_validator_process` | `session.into` | [253](../../src/process_live_tests.rs#L253), [254](../../src/process_live_tests.rs#L254), [333](../../src/process_live_tests.rs#L333) | receiver-type-required |
| `run_real_validator_process` | `[(             artifact.to_string_lossy().into_owned(),             format!("sha256-{:x}", sha2::Sha256::digest(initial)),         )]         .into_iter()         .collect` | [257](../../src/process_live_tests.rs#L257) | receiver-type-required |
| `run_real_validator_process` | `[(             artifact.to_string_lossy().into_owned(),             format!("sha256-{:x}", sha2::Sha256::digest(initial)),         )]         .into_iter` | [257](../../src/process_live_tests.rs#L257) | receiver-type-required |
| `run_real_validator_process` | `artifact.to_string_lossy().into_owned` | [258](../../src/process_live_tests.rs#L258) | receiver-type-required |
| `run_real_validator_process` | `artifact.to_string_lossy` | [258](../../src/process_live_tests.rs#L258) | receiver-type-required |
| `run_real_validator_process` | `engine::begin_validation(&mut ledger, "2026-09-04T00:00:01.000Z", &binding).unwrap` | [264](../../src/process_live_tests.rs#L264) | receiver-type-required |
| `run_real_validator_process` | `engine::begin_validation` | [264](../../src/process_live_tests.rs#L264) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `run_real_validator_process` | `drop` | [265](../../src/process_live_tests.rs#L265) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Arc::new` | [266](../../src/process_live_tests.rs#L266), [286](../../src/process_live_tests.rs#L286) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `provider::MemorySecretStore::new` | [266](../../src/process_live_tests.rs#L266) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `run_real_validator_process` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [267](../../src/process_live_tests.rs#L267) | receiver-type-required |
| `run_real_validator_process` | `secrets         .publish` | [267](../../src/process_live_tests.rs#L267) | receiver-type-required |
| `run_real_validator_process` | `provider["credential_key"].as_str().unwrap` | [269](../../src/process_live_tests.rs#L269) | receiver-type-required |
| `run_real_validator_process` | `provider["credential_key"].as_str` | [269](../../src/process_live_tests.rs#L269) | receiver-type-required |
| `run_real_validator_process` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [272](../../src/process_live_tests.rs#L272) | receiver-type-required |
| `run_real_validator_process` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     ).inspect(&#124;host&#124; { attach_live_stdio(host,&root); })     .unwrap` | [276](../../src/process_live_tests.rs#L276) | receiver-type-required |
| `run_real_validator_process` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     ).inspect` | [276](../../src/process_live_tests.rs#L276) | receiver-type-required |
| `run_real_validator_process` | `ProductionProcessHost::open_with_secret_store` | [276](../../src/process_live_tests.rs#L276) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [278](../../src/process_live_tests.rs#L278) | receiver-type-required |
| `run_real_validator_process` | `attach_live_stdio` | [282](../../src/process_live_tests.rs#L282) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `run_real_validator_process` | `crate::endpoint_carrier::ProductionRespondAuthority::open(         &root,         Arc::new(crate::endpoint_host::SessionAdmissionGates::default()),         host.clone(),     )     .unwrap` | [284](../../src/process_live_tests.rs#L284) | receiver-type-required |
| `run_real_validator_process` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [284](../../src/process_live_tests.rs#L284) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L176) |
| `run_real_validator_process` | `crate::endpoint_host::SessionAdmissionGates::default` | [286](../../src/process_live_tests.rs#L286) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.clone` | [287](../../src/process_live_tests.rs#L287) | receiver-type-required |
| `run_real_validator_process` | `Vec::new` | [290](../../src/process_live_tests.rs#L290) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::panic::catch_unwind` | [291](../../src/process_live_tests.rs#L291) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `std::panic::AssertUnwindSafe` | [291](../../src/process_live_tests.rs#L291) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.schedule_main(session)             .unwrap()             .expect` | [292](../../src/process_live_tests.rs#L292) | receiver-type-required |
| `run_real_validator_process` | `host.schedule_main(session)             .unwrap` | [292](../../src/process_live_tests.rs#L292) | receiver-type-required |
| `run_real_validator_process` | `host.schedule_main` | [292](../../src/process_live_tests.rs#L292) | receiver-type-required |
| `run_real_validator_process` | `host.start_periodic_sweep` | [295](../../src/process_live_tests.rs#L295) | receiver-type-required |
| `run_real_validator_process` | `Instant::now` | [296](../../src/process_live_tests.rs#L296) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Duration::from_secs` | [296](../../src/process_live_tests.rs#L296) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map(&#124;s&#124; serde_json::from_str(s).ok())                 .collect` | [298](../../src/process_live_tests.rs#L298) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines()                 .filter_map` | [298](../../src/process_live_tests.rs#L298) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap()                 .lines` | [298](../../src/process_live_tests.rs#L298) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&path)                 .unwrap` | [298](../../src/process_live_tests.rs#L298) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str(s).ok` | [301](../../src/process_live_tests.rs#L301) | receiver-type-required |
| `run_real_validator_process` | `events.iter().filter` | [303](../../src/process_live_tests.rs#L303) | receiver-type-required |
| `run_real_validator_process` | `events.iter` | [303](../../src/process_live_tests.rs#L303), [347](../../src/process_live_tests.rs#L347), [388](../../src/process_live_tests.rs#L388), [413](../../src/process_live_tests.rs#L413) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .any` | [304](../../src/process_live_tests.rs#L304) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter` | [304](../../src/process_live_tests.rs#L304), [310](../../src/process_live_tests.rs#L310) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .find(&#124;e&#124; e["kind"] == "tool_call" && e["call"] == held["call"])                     .unwrap` | [310](../../src/process_live_tests.rs#L310) | receiver-type-required |
| `run_real_validator_process` | `events                     .iter()                     .find` | [310](../../src/process_live_tests.rs#L310) | receiver-type-required |
| `run_real_validator_process` | `call["args"]["path"].as_str().unwrap` | [318](../../src/process_live_tests.rs#L318) | receiver-type-required |
| `run_real_validator_process` | `call["args"]["path"].as_str` | [318](../../src/process_live_tests.rs#L318) | receiver-type-required |
| `run_real_validator_process` | `requested.is_absolute` | [319](../../src/process_live_tests.rs#L319) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap().canonicalize().unwrap` | [324](../../src/process_live_tests.rs#L324) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap().canonicalize` | [324](../../src/process_live_tests.rs#L324) | receiver-type-required |
| `run_real_validator_process` | `requested.parent().unwrap` | [324](../../src/process_live_tests.rs#L324) | receiver-type-required |
| `run_real_validator_process` | `requested.parent` | [324](../../src/process_live_tests.rs#L324) | receiver-type-required |
| `run_real_validator_process` | `key.clone` | [331](../../src/process_live_tests.rs#L331) | receiver-type-required |
| `run_real_validator_process` | `"live-disposable-approval".into` | [332](../../src/process_live_tests.rs#L332) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str().unwrap().into` | [334](../../src/process_live_tests.rs#L334) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str().unwrap` | [334](../../src/process_live_tests.rs#L334) | receiver-type-required |
| `run_real_validator_process` | `call["call"].as_str` | [334](../../src/process_live_tests.rs#L334) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author(&responder, authorization.clone())                     .expect` | [340](../../src/process_live_tests.rs#L340) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author` | [340](../../src/process_live_tests.rs#L340), [342](../../src/process_live_tests.rs#L342) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `authorization.clone` | [340](../../src/process_live_tests.rs#L340) | receiver-type-required |
| `run_real_validator_process` | `endpoint::RespondAuthority::author(&responder, authorization)                     .expect` | [342](../../src/process_live_tests.rs#L342) | receiver-type-required |
| `run_real_validator_process` | `approval_deliveries.push` | [345](../../src/process_live_tests.rs#L345) | receiver-type-required |
| `run_real_validator_process` | `events.iter().find` | [347](../../src/process_live_tests.rs#L347), [388](../../src/process_live_tests.rs#L388) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines()                     .filter_map(&#124;line&#124; serde_json::from_str(line).ok())                     .collect` | [351](../../src/process_live_tests.rs#L351) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines()                     .filter_map` | [351](../../src/process_live_tests.rs#L351) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default()                     .lines` | [351](../../src/process_live_tests.rs#L351) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(folder.join("endpoint.jsonl"))                     .unwrap_or_default` | [351](../../src/process_live_tests.rs#L351) | receiver-type-required |
| `run_real_validator_process` | `serde_json::from_str(line).ok` | [354](../../src/process_live_tests.rs#L354) | receiver-type-required |
| `run_real_validator_process` | `journal.iter().find` | [356](../../src/process_live_tests.rs#L356), [360](../../src/process_live_tests.rs#L360) | receiver-type-required |
| `run_real_validator_process` | `journal.iter` | [356](../../src/process_live_tests.rs#L356), [360](../../src/process_live_tests.rs#L360) | receiver-type-required |
| `run_real_validator_process` | `journal.iter().find(&#124;e&#124;                         e["event"]["type"] == "assistant/message"                             && &e["event"]["data"]["message"]["id"] == promoted                     ).expect` | [360](../../src/process_live_tests.rs#L360) | receiver-type-required |
| `run_real_validator_process` | `std::thread::sleep` | [376](../../src/process_live_tests.rs#L376) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `Duration::from_millis` | [376](../../src/process_live_tests.rs#L376) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `host.shutdown` | [379](../../src/process_live_tests.rs#L379) | receiver-type-required |
| `run_real_validator_process` | `std::panic::resume_unwind` | [381](../../src/process_live_tests.rs#L381) | external-constructor-callback-or-unresolved |
| `run_real_validator_process` | `events.iter().find(&#124;e&#124; e["kind"] == "settle").unwrap` | [388](../../src/process_live_tests.rs#L388) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev().find(&#124;e&#124; e["kind"] == "spawn").unwrap` | [413](../../src/process_live_tests.rs#L413) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev().find` | [413](../../src/process_live_tests.rs#L413) | receiver-type-required |
| `run_real_validator_process` | `events.iter().rev` | [413](../../src/process_live_tests.rs#L413) | receiver-type-required |
| `run_real_validator_process` | `spawn["child"].as_str().unwrap` | [414](../../src/process_live_tests.rs#L414) | receiver-type-required |
| `run_real_validator_process` | `spawn["child"].as_str` | [414](../../src/process_live_tests.rs#L414) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines()         .map(&#124;s&#124; serde_json::from_str(s).unwrap())         .collect` | [415](../../src/process_live_tests.rs#L415) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines()         .map` | [415](../../src/process_live_tests.rs#L415) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap()         .lines` | [415](../../src/process_live_tests.rs#L415) | receiver-type-required |
| `run_real_validator_process` | `fs::read_to_string(&child_path)         .unwrap` | [415](../../src/process_live_tests.rs#L415) | receiver-type-required |
| `run_real_validator_process` | `child         .iter()         .find(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "verify")         .unwrap` | [422](../../src/process_live_tests.rs#L422) | receiver-type-required |
| `run_real_validator_process` | `child         .iter()         .find` | [422](../../src/process_live_tests.rs#L422) | receiver-type-required |
| `run_real_validator_process` | `child         .iter` | [422](../../src/process_live_tests.rs#L422) | receiver-type-required |
| `run_real_validator_process` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","repair":repair,"approval_deliveries":approval_deliveries,"scope":"Recovered root candidate and real independently spawned validator process with live model; frozen artifact pass and exact promotion. Root generation is seeded, not a live mutation.","settle_seq":settle["seq"],"child":child_path})).unwrap()).unwrap` | [430](../../src/process_live_tests.rs#L430) | receiver-type-required |
| `run_real_validator_process` | `serde_json::to_vec_pretty(&json!({"status":"passed","repair":repair,"approval_deliveries":approval_deliveries,"scope":"Recovered root candidate and real independently spawned validator process with live model; frozen artifact pass and exact promotion. Root generation is seeded, not a live mutation.","settle_seq":settle["seq"],"child":child_path})).unwrap` | [430](../../src/process_live_tests.rs#L430) | receiver-type-required |
| `run_real_validator_process` | `serde_json::to_vec_pretty` | [430](../../src/process_live_tests.rs#L430) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `Mutex::new` | [436](../../src/process_live_tests.rs#L436), [443](../../src/process_live_tests.rs#L443) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args(["-c", "sleep 0.05; exit 0"])             .spawn()             .unwrap` | [437](../../src/process_live_tests.rs#L437) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args(["-c", "sleep 0.05; exit 0"])             .spawn` | [437](../../src/process_live_tests.rs#L437) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sh")             .args` | [437](../../src/process_live_tests.rs#L437) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new` | [437](../../src/process_live_tests.rs#L437), [444](../../src/process_live_tests.rs#L444) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg("5")             .spawn()             .unwrap` | [444](../../src/process_live_tests.rs#L444) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg("5")             .spawn` | [444](../../src/process_live_tests.rs#L444) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `std::process::Command::new("/bin/sleep")             .arg` | [444](../../src/process_live_tests.rs#L444) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child_exited_within` | [449](../../src/process_live_tests.rs#L449) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `Duration::from_millis` | [449](../../src/process_live_tests.rs#L449) | external-constructor-callback-or-unresolved |
| `delivery_fallback_requires_confirmed_child_exit` | `live.lock().unwrap` | [450](../../src/process_live_tests.rs#L450) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `live.lock` | [450](../../src/process_live_tests.rs#L450) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.kill().unwrap` | [451](../../src/process_live_tests.rs#L451) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.kill` | [451](../../src/process_live_tests.rs#L451) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.wait().unwrap` | [452](../../src/process_live_tests.rs#L452) | receiver-type-required |
| `delivery_fallback_requires_confirmed_child_exit` | `child.wait` | [452](../../src/process_live_tests.rs#L452) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `tempfile::tempdir().unwrap` | [462](../../src/process_live_tests.rs#L462), [463](../../src/process_live_tests.rs#L463) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `tempfile::tempdir` | [462](../../src/process_live_tests.rs#L462), [463](../../src/process_live_tests.rs#L463) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `project_root.path().to_path_buf` | [464](../../src/process_live_tests.rs#L464) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `project_root.path` | [464](../../src/process_live_tests.rs#L464) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all(&workspace).unwrap` | [465](../../src/process_live_tests.rs#L465) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all` | [465](../../src/process_live_tests.rs#L465), [489](../../src/process_live_tests.rs#L489) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `write_test_execution_config` | [466](../../src/process_live_tests.rs#L466) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path` | [466](../../src/process_live_tests.rs#L466), [469](../../src/process_live_tests.rs#L469), [484](../../src/process_live_tests.rs#L484), [488](../../src/process_live_tests.rs#L488), [493](../../src/process_live_tests.rs#L493), [497](../../src/process_live_tests.rs#L497), [499](../../src/process_live_tests.rs#L499) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join` | [469](../../src/process_live_tests.rs#L469), [484](../../src/process_live_tests.rs#L484), [488](../../src/process_live_tests.rs#L488), [493](../../src/process_live_tests.rs#L493), [497](../../src/process_live_tests.rs#L497), [499](../../src/process_live_tests.rs#L499) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(         &bin,         br##"#!/bin/sh printf '%s\n' '{"hello":{"max":2,"min":2,"proto":"tekes-worker"}}' IFS= read -r selected while [ ! -f "$0.exit" ]; do sleep 0.01; done exec 0<&- : > "$0.closed" sleep 0.2 exit 0 "##,     )     .unwrap` | [470](../../src/process_live_tests.rs#L470) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write` | [470](../../src/process_live_tests.rs#L470), [485](../../src/process_live_tests.rs#L485), [497](../../src/process_live_tests.rs#L497) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap` | [483](../../src/process_live_tests.rs#L483) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions` | [483](../../src/process_live_tests.rs#L483), [486](../../src/process_live_tests.rs#L486) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::Permissions::from_mode` | [483](../../src/process_live_tests.rs#L483), [486](../../src/process_live_tests.rs#L486) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap` | [485](../../src/process_live_tests.rs#L485) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap` | [486](../../src/process_live_tests.rs#L486) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join("threads").join` | [488](../../src/process_live_tests.rs#L488) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::create_dir_all(folder.join("assets")).unwrap` | [489](../../src/process_live_tests.rs#L489) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `folder.join` | [489](../../src/process_live_tests.rs#L489), [490](../../src/process_live_tests.rs#L490), [491](../../src/process_live_tests.rs#L491) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `write_test_genesis` | [490](../../src/process_live_tests.rs#L490) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `append_test_input` | [491](../../src/process_live_tests.rs#L491) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent")).unwrap` | [493](../../src/process_live_tests.rs#L493) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `ProductionProcessHost::open` | [493](../../src/process_live_tests.rs#L493) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::catch_unwind` | [494](../../src/process_live_tests.rs#L494) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::AssertUnwindSafe` | [494](../../src/process_live_tests.rs#L494) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main(session).unwrap().unwrap` | [495](../../src/process_live_tests.rs#L495) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main(session).unwrap` | [495](../../src/process_live_tests.rs#L495) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.schedule_main` | [495](../../src/process_live_tests.rs#L495) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.live_worker(session).unwrap` | [496](../../src/process_live_tests.rs#L496) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.live_worker` | [496](../../src/process_live_tests.rs#L496) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `fs::write(root.path().join("fake-worker.exit"), b"exit").unwrap` | [497](../../src/process_live_tests.rs#L497) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Instant::now` | [498](../../src/process_live_tests.rs#L498) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Duration::from_secs` | [498](../../src/process_live_tests.rs#L498) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `root.path().join("fake-worker.closed").exists` | [499](../../src/process_live_tests.rs#L499) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::thread::sleep` | [501](../../src/process_live_tests.rs#L501) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `Duration::from_millis` | [501](../../src/process_live_tests.rs#L501) | external-constructor-callback-or-unresolved |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"approval".into` | [508](../../src/process_live_tests.rs#L508), [514](../../src/process_live_tests.rs#L514) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"test".into` | [510](../../src/process_live_tests.rs#L510), [511](../../src/process_live_tests.rs#L511) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `session.into` | [512](../../src/process_live_tests.rs#L512) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"respond".into` | [513](../../src/process_live_tests.rs#L513) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `"call".into` | [516](../../src/process_live_tests.rs#L516) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `host.shutdown` | [526](../../src/process_live_tests.rs#L526) | receiver-type-required |
| `approval_closed_stdin_waits_for_actual_exit_before_fallback` | `std::panic::resume_unwind` | [528](../../src/process_live_tests.rs#L528) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `Instant::now` | [534](../../src/process_live_tests.rs#L534) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `cancel_stage_failure` | [535](../../src/process_live_tests.rs#L535), [548](../../src/process_live_tests.rs#L548) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `internal` | [538](../../src/process_live_tests.rs#L538) | external-constructor-callback-or-unresolved |
| `cancel_stage_failure_names_the_stage_and_keeps_typed_failures` | `failure` | [551](../../src/process_live_tests.rs#L551) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `tempfile::tempdir().unwrap` | [561](../../src/process_live_tests.rs#L561), [562](../../src/process_live_tests.rs#L562) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `tempfile::tempdir` | [561](../../src/process_live_tests.rs#L561), [562](../../src/process_live_tests.rs#L562) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `project_root.path().to_path_buf` | [563](../../src/process_live_tests.rs#L563) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `project_root.path` | [563](../../src/process_live_tests.rs#L563) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all(&workspace).unwrap` | [564](../../src/process_live_tests.rs#L564) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all` | [564](../../src/process_live_tests.rs#L564), [588](../../src/process_live_tests.rs#L588) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `write_test_execution_config` | [565](../../src/process_live_tests.rs#L565) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path` | [565](../../src/process_live_tests.rs#L565), [568](../../src/process_live_tests.rs#L568), [583](../../src/process_live_tests.rs#L583), [587](../../src/process_live_tests.rs#L587), [592](../../src/process_live_tests.rs#L592), [596](../../src/process_live_tests.rs#L596), [598](../../src/process_live_tests.rs#L598) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join` | [568](../../src/process_live_tests.rs#L568), [583](../../src/process_live_tests.rs#L583), [587](../../src/process_live_tests.rs#L587), [592](../../src/process_live_tests.rs#L592), [596](../../src/process_live_tests.rs#L596), [598](../../src/process_live_tests.rs#L598) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(         &bin,         br##"#!/bin/sh printf '%s\n' '{"hello":{"max":2,"min":2,"proto":"tekes-worker"}}' IFS= read -r selected while [ ! -f "$0.exit" ]; do sleep 0.01; done exec 0<&- : > "$0.closed" sleep 0.2 exit 0 "##,     )     .unwrap` | [569](../../src/process_live_tests.rs#L569) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write` | [569](../../src/process_live_tests.rs#L569), [584](../../src/process_live_tests.rs#L584), [596](../../src/process_live_tests.rs#L596) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap` | [582](../../src/process_live_tests.rs#L582) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions` | [582](../../src/process_live_tests.rs#L582), [585](../../src/process_live_tests.rs#L585) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::Permissions::from_mode` | [582](../../src/process_live_tests.rs#L582), [585](../../src/process_live_tests.rs#L585) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap` | [584](../../src/process_live_tests.rs#L584) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap` | [585](../../src/process_live_tests.rs#L585) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join("threads").join` | [587](../../src/process_live_tests.rs#L587) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::create_dir_all(folder.join("assets")).unwrap` | [588](../../src/process_live_tests.rs#L588) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `folder.join` | [588](../../src/process_live_tests.rs#L588), [589](../../src/process_live_tests.rs#L589), [590](../../src/process_live_tests.rs#L590), [620](../../src/process_live_tests.rs#L620) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `write_test_genesis` | [589](../../src/process_live_tests.rs#L589) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `append_test_input` | [590](../../src/process_live_tests.rs#L590) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent")).unwrap` | [592](../../src/process_live_tests.rs#L592) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `ProductionProcessHost::open` | [592](../../src/process_live_tests.rs#L592) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::catch_unwind` | [593](../../src/process_live_tests.rs#L593) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::AssertUnwindSafe` | [593](../../src/process_live_tests.rs#L593) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main(session).unwrap().unwrap` | [594](../../src/process_live_tests.rs#L594) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main(session).unwrap` | [594](../../src/process_live_tests.rs#L594) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.schedule_main` | [594](../../src/process_live_tests.rs#L594) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.live_worker(session).unwrap` | [595](../../src/process_live_tests.rs#L595) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.live_worker` | [595](../../src/process_live_tests.rs#L595) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::write(root.path().join("fake-worker.exit"), b"exit").unwrap` | [596](../../src/process_live_tests.rs#L596) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Instant::now` | [597](../../src/process_live_tests.rs#L597) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Duration::from_secs` | [597](../../src/process_live_tests.rs#L597) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `root.path().join("fake-worker.closed").exists` | [598](../../src/process_live_tests.rs#L598) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::thread::sleep` | [600](../../src/process_live_tests.rs#L600) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `Duration::from_millis` | [600](../../src/process_live_tests.rs#L600) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"test".into` | [607](../../src/process_live_tests.rs#L607), [608](../../src/process_live_tests.rs#L608) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `session.into` | [609](../../src/process_live_tests.rs#L609) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"session.cancel".into` | [610](../../src/process_live_tests.rs#L610) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `"stop-after-exit".into` | [611](../../src/process_live_tests.rs#L611) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host             .cancel(session, "2026-08-28T00:00:05.000Z", &origin)             .expect` | [613](../../src/process_live_tests.rs#L613) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host             .cancel` | [613](../../src/process_live_tests.rs#L613) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::read(folder.join("main.jsonl")).unwrap` | [620](../../src/process_live_tests.rs#L620) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `fs::read` | [620](../../src/process_live_tests.rs#L620) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `scan_valid_prefix(&bytes, schema::READER_VERSION).projection.unwrap` | [621](../../src/process_live_tests.rs#L621) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `scan_valid_prefix` | [621](../../src/process_live_tests.rs#L621) | external-constructor-callback-or-unresolved |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter()             .filter(&#124;event&#124; event.kind() == &EventKind::StopRequested)             .collect::<Vec<_>>` | [622](../../src/process_live_tests.rs#L622) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter()             .filter` | [622](../../src/process_live_tests.rs#L622) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `projection             .events             .iter` | [622](../../src/process_live_tests.rs#L622) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `event.kind` | [625](../../src/process_live_tests.rs#L625) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `host.shutdown` | [631](../../src/process_live_tests.rs#L631) | receiver-type-required |
| `cancel_after_the_worker_closed_stdin_returns_the_durable_receipt` | `std::panic::resume_unwind` | [633](../../src/process_live_tests.rs#L633) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `PathBuf::from` | [641](../../src/process_live_tests.rs#L641), [740](../../src/process_live_tests.rs#L740) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [641](../../src/process_live_tests.rs#L641) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var` | [641](../../src/process_live_tests.rs#L641), [652](../../src/process_live_tests.rs#L652), [683](../../src/process_live_tests.rs#L683), [689](../../src/process_live_tests.rs#L689), [716](../../src/process_live_tests.rs#L716) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&root).unwrap` | [642](../../src/process_live_tests.rs#L642) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all` | [642](../../src/process_live_tests.rs#L642), [644](../../src/process_live_tests.rs#L644), [645](../../src/process_live_tests.rs#L645), [647](../../src/process_live_tests.rs#L647), [668](../../src/process_live_tests.rs#L668) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap().join` | [643](../../src/process_live_tests.rs#L643) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent().unwrap` | [643](../../src/process_live_tests.rs#L643) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.parent` | [643](../../src/process_live_tests.rs#L643) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(&workspace).unwrap` | [644](../../src/process_live_tests.rs#L644) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(workspace.join(".agent/skills")).unwrap` | [645](../../src/process_live_tests.rs#L645) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.join` | [645](../../src/process_live_tests.rs#L645), [646](../../src/process_live_tests.rs#L646), [675](../../src/process_live_tests.rs#L675), [744](../../src/process_live_tests.rs#L744) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(workspace.join(".agent/skills/live-proof.md"), "Live proof skill\nSKILL_LIVE_BODY_OK\n").unwrap` | [646](../../src/process_live_tests.rs#L646) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write` | [646](../../src/process_live_tests.rs#L646), [708](../../src/process_live_tests.rs#L708), [823](../../src/process_live_tests.rs#L823) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(root.join("config")).unwrap` | [647](../../src/process_live_tests.rs#L647) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join` | [647](../../src/process_live_tests.rs#L647), [648](../../src/process_live_tests.rs#L648), [656](../../src/process_live_tests.rs#L656), [667](../../src/process_live_tests.rs#L667), [691](../../src/process_live_tests.rs#L691), [700](../../src/process_live_tests.rs#L700), [708](../../src/process_live_tests.rs#L708), [710](../../src/process_live_tests.rs#L710), [782](../../src/process_live_tests.rs#L782), [823](../../src/process_live_tests.rs#L823) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_canonical_test_json` | [648](../../src/process_live_tests.rs#L648), [655](../../src/process_live_tests.rs#L655) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice(         &fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap(),     )     .unwrap` | [651](../../src/process_live_tests.rs#L651) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_slice` | [651](../../src/process_live_tests.rs#L651) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [652](../../src/process_live_tests.rs#L652) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read` | [652](../../src/process_live_tests.rs#L652) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [652](../../src/process_live_tests.rs#L652) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_test_execution_config` | [659](../../src/process_live_tests.rs#L659) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.join("threads").join` | [667](../../src/process_live_tests.rs#L667) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::create_dir_all(folder.join("assets")).unwrap` | [668](../../src/process_live_tests.rs#L668) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `folder.join` | [668](../../src/process_live_tests.rs#L668), [669](../../src/process_live_tests.rs#L669), [775](../../src/process_live_tests.rs#L775), [814](../../src/process_live_tests.rs#L814), [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `write_test_genesis` | [670](../../src/process_live_tests.rs#L670) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `append_test_input` | [671](../../src/process_live_tests.rs#L671) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map` | [672](../../src/process_live_tests.rs#L672), [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines` | [672](../../src/process_live_tests.rs#L672), [724](../../src/process_live_tests.rs#L724), [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap` | [672](../../src/process_live_tests.rs#L672), [724](../../src/process_live_tests.rs#L724), [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string` | [672](../../src/process_live_tests.rs#L672), [724](../../src/process_live_tests.rs#L724), [775](../../src/process_live_tests.rs#L775), [793](../../src/process_live_tests.rs#L793), [814](../../src/process_live_tests.rs#L814), [820](../../src/process_live_tests.rs#L820) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).unwrap` | [672](../../src/process_live_tests.rs#L672) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str` | [672](../../src/process_live_tests.rs#L672), [724](../../src/process_live_tests.rs#L724), [775](../../src/process_live_tests.rs#L775), [793](../../src/process_live_tests.rs#L793), [814](../../src/process_live_tests.rs#L814), [820](../../src/process_live_tests.rs#L820) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `write_test_events` | [674](../../src/process_live_tests.rs#L674) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Arc::new` | [677](../../src/process_live_tests.rs#L677), [700](../../src/process_live_tests.rs#L700), [715](../../src/process_live_tests.rs#L715) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `provider::MemorySecretStore::new` | [677](../../src/process_live_tests.rs#L677) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish(             provider["credential_key"].as_str().unwrap(),             provider::SecretRecord::Active {                 generation: 1,                 material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap(),             },         )         .unwrap` | [678](../../src/process_live_tests.rs#L678) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `secrets         .publish` | [678](../../src/process_live_tests.rs#L678) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str().unwrap` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `provider["credential_key"].as_str` | [680](../../src/process_live_tests.rs#L680) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [683](../../src/process_live_tests.rs#L683) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     ).inspect(&#124;host&#124; { attach_live_stdio(host,&root); })     .unwrap` | [687](../../src/process_live_tests.rs#L687) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store(         &root,         std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),         "live-validator-process",         root.join(".agent"),         secrets,     ).inspect` | [687](../../src/process_live_tests.rs#L687) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ProductionProcessHost::open_with_secret_store` | [687](../../src/process_live_tests.rs#L687) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [689](../../src/process_live_tests.rs#L689) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `attach_live_stdio` | [693](../../src/process_live_tests.rs#L693) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new().unwrap` | [695](../../src/process_live_tests.rs#L695) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::runtime::Runtime::new` | [695](../../src/process_live_tests.rs#L695) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [696](../../src/process_live_tests.rs#L696) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on` | [696](../../src/process_live_tests.rs#L696), [791](../../src/process_live_tests.rs#L791) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `tokio::net::TcpListener::bind` | [696](../../src/process_live_tests.rs#L696) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr().unwrap` | [697](../../src/process_live_tests.rs#L697) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `listener.local_addr` | [697](../../src/process_live_tests.rs#L697) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [698](../../src/process_live_tests.rs#L698) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::host_runtime::assemble_application_endpoint_host` | [698](../../src/process_live_tests.rs#L698) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_validator_exhaustion_releases_queued_input` | `"test".into` | [699](../../src/process_live_tests.rs#L699), [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy().into_owned` | [699](../../src/process_live_tests.rs#L699) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `workspace.to_string_lossy` | [699](../../src/process_live_tests.rs#L699) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy().into_owned` | [699](../../src/process_live_tests.rs#L699) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.to_string_lossy` | [699](../../src/process_live_tests.rs#L699) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Ok` | [700](../../src/process_live_tests.rs#L700), [716](../../src/process_live_tests.rs#L716) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `"2026-09-04T10:00:00.000Z".into` | [700](../../src/process_live_tests.rs#L700) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.clone` | [700](../../src/process_live_tests.rs#L700), [701](../../src/process_live_tests.rs#L701), [715](../../src/process_live_tests.rs#L715) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),         transport::TransportConfig::loopback(address, transport::BearerToken::new([42; 32]))).unwrap` | [701](../../src/process_live_tests.rs#L701) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [701](../../src/process_live_tests.rs#L701) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_validator_exhaustion_releases_queued_input` | `transport::TransportConfig::loopback` | [702](../../src/process_live_tests.rs#L702) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_validator_exhaustion_releases_queued_input` | `transport::BearerToken::new` | [702](../../src/process_live_tests.rs#L702) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_validator_exhaustion_releases_queued_input` | `host.attach_streams` | [703](../../src/process_live_tests.rs#L703) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams().clone` | [703](../../src/process_live_tests.rs#L703) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.streams` | [703](../../src/process_live_tests.rs#L703) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery().unwrap` | [704](../../src/process_live_tests.rs#L704) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.finish_recovery` | [704](../../src/process_live_tests.rs#L704) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `assembly.into_server` | [705](../../src/process_live_tests.rs#L705) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.handle` | [706](../../src/process_live_tests.rs#L706) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.spawn` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `server.serve` | [707](../../src/process_live_tests.rs#L707) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [708](../../src/process_live_tests.rs#L708) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [708](../../src/process_live_tests.rs#L708) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec` | [708](../../src/process_live_tests.rs#L708) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Instant::now` | [709](../../src/process_live_tests.rs#L709), [722](../../src/process_live_tests.rs#L722) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_secs` | [709](../../src/process_live_tests.rs#L709), [722](../../src/process_live_tests.rs#L722) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-ready").exists` | [710](../../src/process_live_tests.rs#L710) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::thread::sleep` | [712](../../src/process_live_tests.rs#L712), [786](../../src/process_live_tests.rs#L786) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `Duration::from_millis` | [712](../../src/process_live_tests.rs#L712), [786](../../src/process_live_tests.rs#L786) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open(         &root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [714](../../src/process_live_tests.rs#L714) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [714](../../src/process_live_tests.rs#L714) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L176) |
| `real_validator_exhaustion_releases_queued_input` | `crate::endpoint_host::SessionAdmissionGates::default` | [715](../../src/process_live_tests.rs#L715) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [716](../../src/process_live_tests.rs#L716) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `Vec::new` | [717](../../src/process_live_tests.rs#L717) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::catch_unwind` | [718](../../src/process_live_tests.rs#L718) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::AssertUnwindSafe` | [718](../../src/process_live_tests.rs#L718) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap().expect` | [719](../../src/process_live_tests.rs#L719) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main(session).unwrap` | [719](../../src/process_live_tests.rs#L719) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.schedule_main` | [719](../../src/process_live_tests.rs#L719) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.start_periodic_sweep` | [720](../../src/process_live_tests.rs#L720) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [724](../../src/process_live_tests.rs#L724) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [724](../../src/process_live_tests.rs#L724) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).ok` | [724](../../src/process_live_tests.rs#L724) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter` | [725](../../src/process_live_tests.rs#L725), [774](../../src/process_live_tests.rs#L774), [778](../../src/process_live_tests.rs#L778), [798](../../src/process_live_tests.rs#L798), [818](../../src/process_live_tests.rs#L818) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter` | [725](../../src/process_live_tests.rs#L725), [769](../../src/process_live_tests.rs#L769), [774](../../src/process_live_tests.rs#L774), [778](../../src/process_live_tests.rs#L778), [798](../../src/process_live_tests.rs#L798), [805](../../src/process_live_tests.rs#L805), [806](../../src/process_live_tests.rs#L806), [809](../../src/process_live_tests.rs#L809), [811](../../src/process_live_tests.rs#L811), [813](../../src/process_live_tests.rs#L813), [818](../../src/process_live_tests.rs#L818) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .any` | [726](../../src/process_live_tests.rs#L726) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter` | [726](../../src/process_live_tests.rs#L726), [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find(&#124;e&#124; e["kind"] == "tool_call" && e["call"] == held["call"])                     .unwrap` | [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events                     .iter()                     .find` | [732](../../src/process_live_tests.rs#L732) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str().unwrap` | [740](../../src/process_live_tests.rs#L740) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["args"]["path"].as_str` | [740](../../src/process_live_tests.rs#L740) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.is_absolute` | [741](../../src/process_live_tests.rs#L741) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize().unwrap` | [746](../../src/process_live_tests.rs#L746) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap().canonicalize` | [746](../../src/process_live_tests.rs#L746) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent().unwrap` | [746](../../src/process_live_tests.rs#L746) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `requested.parent` | [746](../../src/process_live_tests.rs#L746) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `key.clone` | [753](../../src/process_live_tests.rs#L753) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"live-disposable-approval".into` | [754](../../src/process_live_tests.rs#L754) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `session.into` | [755](../../src/process_live_tests.rs#L755), [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap().into` | [756](../../src/process_live_tests.rs#L756) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str().unwrap` | [756](../../src/process_live_tests.rs#L756) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `call["call"].as_str` | [756](../../src/process_live_tests.rs#L756) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization.clone())                     .expect` | [762](../../src/process_live_tests.rs#L762) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author` | [762](../../src/process_live_tests.rs#L762), [764](../../src/process_live_tests.rs#L764) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `authorization.clone` | [762](../../src/process_live_tests.rs#L762) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `endpoint::RespondAuthority::author(&responder, authorization)                     .expect` | [764](../../src/process_live_tests.rs#L764) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `approval_deliveries.push` | [767](../../src/process_live_tests.rs#L767) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().any` | [769](../../src/process_live_tests.rs#L769) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"exhaustion-process".into` | [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"session.prompt".into` | [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `"queued-body".into` | [770](../../src/process_live_tests.rs#L770) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt(session,"2026-09-04T10:00:00.000Z",&origin,&MaterializedPrompt {blocks:vec![Block::Text {text:"重试".into()}],attachments:vec![],files:vec![]},false).unwrap` | [771](../../src/process_live_tests.rs#L771) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.prompt` | [771](../../src/process_live_tests.rs#L771) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "settle").count` | [774](../../src/process_live_tests.rs#L774) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines().filter_map` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default().lines` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join("endpoint.jsonl")).unwrap_or_default` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(line).ok` | [775](../../src/process_live_tests.rs#L775) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter(&#124;e&#124; e["event"]["type"] == "turn/end").collect` | [776](../../src/process_live_tests.rs#L776) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter().filter` | [776](../../src/process_live_tests.rs#L776) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `journal.iter` | [776](../../src/process_live_tests.rs#L776) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.len` | [777](../../src/process_live_tests.rs#L777) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter().zip` | [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `ends.iter` | [778](../../src/process_live_tests.rs#L778) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `root.join("client-receipt.json").exists` | [782](../../src/process_live_tests.rs#L782) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `host.shutdown` | [789](../../src/process_live_tests.rs#L789) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_handle.begin_drain` | [790](../../src/process_live_tests.rs#L790) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap().unwrap` | [791](../../src/process_live_tests.rs#L791) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `transport_runtime.block_on(serve).unwrap` | [791](../../src/process_live_tests.rs#L791) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `std::panic::resume_unwind` | [792](../../src/process_live_tests.rs#L792) | external-constructor-callback-or-unresolved |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [793](../../src/process_live_tests.rs#L793) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::from_str(s).unwrap` | [793](../../src/process_live_tests.rs#L793), [814](../../src/process_live_tests.rs#L814), [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "approval_response").collect` | [798](../../src/process_live_tests.rs#L798) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 1).unwrap` | [805](../../src/process_live_tests.rs#L805) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find` | [805](../../src/process_live_tests.rs#L805), [806](../../src/process_live_tests.rs#L806), [809](../../src/process_live_tests.rs#L809), [811](../../src/process_live_tests.rs#L811), [813](../../src/process_live_tests.rs#L813) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "settle" && e["turn"] == 2).unwrap` | [806](../../src/process_live_tests.rs#L806) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "turn_open" && e["turn"] == 2).unwrap` | [809](../../src/process_live_tests.rs#L809) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "input" && e["content"][0]["text"] == "重试").unwrap` | [811](../../src/process_live_tests.rs#L811) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().find(&#124;e&#124; e["kind"] == "spawn").unwrap` | [813](../../src/process_live_tests.rs#L813) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines().map` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap().lines` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str().unwrap` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawn["child"].as_str` | [814](../../src/process_live_tests.rs#L814) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `events.iter().filter(&#124;e&#124; e["kind"] == "spawn").collect` | [818](../../src/process_live_tests.rs#L818) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines().map` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap().lines` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::read_to_string(folder.join(spawns[1]["child"].as_str().unwrap())).unwrap` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str().unwrap` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `spawns[1]["child"].as_str` | [820](../../src/process_live_tests.rs#L820) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "verify").expect` | [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter().find` | [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `second_child.iter` | [821](../../src/process_live_tests.rs#L821) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap()).unwrap` | [823](../../src/process_live_tests.rs#L823) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty(&json!({"status":"passed","scope":"real worker and validator processes with scripted provider; real approved helper write; four mandate refusals, inconclusive settlement, queued input admission and second validator pass over retained artifact history","first":first,"second":second,"queued_seq":input["seq"],"admission_seq":admitted["seq"]})).unwrap` | [823](../../src/process_live_tests.rs#L823) | receiver-type-required |
| `real_validator_exhaustion_releases_queued_input` | `serde_json::to_vec_pretty` | [823](../../src/process_live_tests.rs#L823) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `PathBuf::from` | [831](../../src/process_live_tests.rs#L831), [922](../../src/process_live_tests.rs#L922) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [831](../../src/process_live_tests.rs#L831) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var` | [831](../../src/process_live_tests.rs#L831), [835](../../src/process_live_tests.rs#L835), [846](../../src/process_live_tests.rs#L846), [881](../../src/process_live_tests.rs#L881), [882](../../src/process_live_tests.rs#L882), [883](../../src/process_live_tests.rs#L883) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.parent().unwrap().join` | [832](../../src/process_live_tests.rs#L832) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent().unwrap` | [832](../../src/process_live_tests.rs#L832) | receiver-type-required |
| `real_legacy_simple_task` | `root.parent` | [832](../../src/process_live_tests.rs#L832) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("docs")).unwrap` | [833](../../src/process_live_tests.rs#L833) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all` | [833](../../src/process_live_tests.rs#L833), [834](../../src/process_live_tests.rs#L834), [841](../../src/process_live_tests.rs#L841), [850](../../src/process_live_tests.rs#L850), [853](../../src/process_live_tests.rs#L853) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `workspace.join` | [833](../../src/process_live_tests.rs#L833), [850](../../src/process_live_tests.rs#L850), [853](../../src/process_live_tests.rs#L853), [855](../../src/process_live_tests.rs#L855), [856](../../src/process_live_tests.rs#L856), [857](../../src/process_live_tests.rs#L857), [861](../../src/process_live_tests.rs#L861), [862](../../src/process_live_tests.rs#L862), [863](../../src/process_live_tests.rs#L863), [923](../../src/process_live_tests.rs#L923), [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(root.join("config")).unwrap` | [834](../../src/process_live_tests.rs#L834) | receiver-type-required |
| `real_legacy_simple_task` | `root.join` | [834](../../src/process_live_tests.rs#L834), [836](../../src/process_live_tests.rs#L836), [837](../../src/process_live_tests.rs#L837), [840](../../src/process_live_tests.rs#L840), [882](../../src/process_live_tests.rs#L882), [890](../../src/process_live_tests.rs#L890), [898](../../src/process_live_tests.rs#L898), [900](../../src/process_live_tests.rs#L900), [931](../../src/process_live_tests.rs#L931), [977](../../src/process_live_tests.rs#L977) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [835](../../src/process_live_tests.rs#L835) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_slice` | [835](../../src/process_live_tests.rs#L835) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [835](../../src/process_live_tests.rs#L835) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read` | [835](../../src/process_live_tests.rs#L835) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [835](../../src/process_live_tests.rs#L835) | receiver-type-required |
| `real_legacy_simple_task` | `write_canonical_test_json` | [836](../../src/process_live_tests.rs#L836), [837](../../src/process_live_tests.rs#L837) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `write_test_execution_config` | [838](../../src/process_live_tests.rs#L838) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("threads").join` | [840](../../src/process_live_tests.rs#L840) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(folder.join("assets")).unwrap` | [841](../../src/process_live_tests.rs#L841) | receiver-type-required |
| `real_legacy_simple_task` | `folder.join` | [841](../../src/process_live_tests.rs#L841), [842](../../src/process_live_tests.rs#L842) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_genesis` | [843](../../src/process_live_tests.rs#L843) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `append_test_input` | [844](../../src/process_live_tests.rs#L844) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124; serde_json::from_str(line).unwrap()).collect` | [845](../../src/process_live_tests.rs#L845) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map` | [845](../../src/process_live_tests.rs#L845), [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines` | [845](../../src/process_live_tests.rs#L845), [912](../../src/process_live_tests.rs#L912), [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap` | [845](../../src/process_live_tests.rs#L845), [912](../../src/process_live_tests.rs#L912), [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string` | [845](../../src/process_live_tests.rs#L845), [912](../../src/process_live_tests.rs#L912), [916](../../src/process_live_tests.rs#L916), [942](../../src/process_live_tests.rs#L942), [951](../../src/process_live_tests.rs#L951) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `serde_json::from_str(line).unwrap` | [845](../../src/process_live_tests.rs#L845), [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str` | [845](../../src/process_live_tests.rs#L845), [912](../../src/process_live_tests.rs#L912), [916](../../src/process_live_tests.rs#L916), [942](../../src/process_live_tests.rs#L942) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_LEGACY_TASK_SCENARIO").unwrap_or_default` | [846](../../src/process_live_tests.rs#L846) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("src")).unwrap` | [850](../../src/process_live_tests.rs#L850) | receiver-type-required |
| `real_legacy_simple_task` | `fs::create_dir_all(workspace.join("markdown")).unwrap` | [853](../../src/process_live_tests.rs#L853) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/1.md"), source).unwrap` | [855](../../src/process_live_tests.rs#L855) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write` | [855](../../src/process_live_tests.rs#L855), [856](../../src/process_live_tests.rs#L856), [857](../../src/process_live_tests.rs#L857), [861](../../src/process_live_tests.rs#L861), [862](../../src/process_live_tests.rs#L862), [863](../../src/process_live_tests.rs#L863), [898](../../src/process_live_tests.rs#L898), [977](../../src/process_live_tests.rs#L977) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/2.md"), "# Population\nBeijing has 21,000,000 people, based on markdown/1.md.\n").unwrap` | [856](../../src/process_live_tests.rs#L856) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("markdown/3.md"), "# People aged 60+\nBeijing has 4,200,000 people aged 60+, assuming 20% of the population in markdown/2.md.\n").unwrap` | [857](../../src/process_live_tests.rs#L857) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/1.py"), python_source).unwrap` | [861](../../src/process_live_tests.rs#L861) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/2.py"), "import importlib\ndef make_capital_sentence():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    return 'Beijing is the capital of China.'\n").unwrap` | [862](../../src/process_live_tests.rs#L862) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(workspace.join("src/3.py"), "import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('src.1').get_country_and_capital()\n    sentence = importlib.import_module('src.2').make_capital_sentence()\n    return 'China / Beijing: ' + sentence\n").unwrap` | [863](../../src/process_live_tests.rs#L863) | receiver-type-required |
| `real_legacy_simple_task` | `write_test_events` | [879](../../src/process_live_tests.rs#L879) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Arc::new` | [880](../../src/process_live_tests.rs#L880), [890](../../src/process_live_tests.rs#L890), [906](../../src/process_live_tests.rs#L906) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `provider::MemorySecretStore::new` | [880](../../src/process_live_tests.rs#L880) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_legacy_simple_task` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [881](../../src/process_live_tests.rs#L881) | receiver-type-required |
| `real_legacy_simple_task` | `secrets.publish` | [881](../../src/process_live_tests.rs#L881) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str().unwrap` | [881](../../src/process_live_tests.rs#L881) | receiver-type-required |
| `real_legacy_simple_task` | `provider["credential_key"].as_str` | [881](../../src/process_live_tests.rs#L881) | receiver-type-required |
| `real_legacy_simple_task` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [881](../../src/process_live_tests.rs#L881) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"live-task",root.join(".agent"),secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [882](../../src/process_live_tests.rs#L882) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"live-task",root.join(".agent"),secrets).inspect` | [882](../../src/process_live_tests.rs#L882) | receiver-type-required |
| `real_legacy_simple_task` | `ProductionProcessHost::open_with_secret_store` | [882](../../src/process_live_tests.rs#L882) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [882](../../src/process_live_tests.rs#L882) | receiver-type-required |
| `real_legacy_simple_task` | `attach_live_stdio` | [882](../../src/process_live_tests.rs#L882) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_legacy_simple_task` | `std::env::var("TEKES_PUBLIC_APPROVAL").as_deref` | [883](../../src/process_live_tests.rs#L883) | receiver-type-required |
| `real_legacy_simple_task` | `Ok` | [883](../../src/process_live_tests.rs#L883), [890](../../src/process_live_tests.rs#L890) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new().unwrap` | [885](../../src/process_live_tests.rs#L885) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::runtime::Runtime::new` | [885](../../src/process_live_tests.rs#L885) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [886](../../src/process_live_tests.rs#L886) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on` | [886](../../src/process_live_tests.rs#L886), [939](../../src/process_live_tests.rs#L939) | receiver-type-required |
| `real_legacy_simple_task` | `tokio::net::TcpListener::bind` | [886](../../src/process_live_tests.rs#L886) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `listener.local_addr().unwrap` | [887](../../src/process_live_tests.rs#L887) | receiver-type-required |
| `real_legacy_simple_task` | `listener.local_addr` | [887](../../src/process_live_tests.rs#L887) | receiver-type-required |
| `real_legacy_simple_task` | `crate::host_runtime::assemble_application_endpoint_host(&root,             endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},             Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"), host.clone()).unwrap` | [888](../../src/process_live_tests.rs#L888) | receiver-type-required |
| `real_legacy_simple_task` | `crate::host_runtime::assemble_application_endpoint_host` | [888](../../src/process_live_tests.rs#L888) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_legacy_simple_task` | `"test".into` | [889](../../src/process_live_tests.rs#L889) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy().into_owned` | [889](../../src/process_live_tests.rs#L889) | receiver-type-required |
| `real_legacy_simple_task` | `workspace.to_string_lossy` | [889](../../src/process_live_tests.rs#L889) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy().into_owned` | [889](../../src/process_live_tests.rs#L889) | receiver-type-required |
| `real_legacy_simple_task` | `root.to_string_lossy` | [889](../../src/process_live_tests.rs#L889) | receiver-type-required |
| `real_legacy_simple_task` | `"2026-09-04T10:00:00.000Z".into` | [890](../../src/process_live_tests.rs#L890) | receiver-type-required |
| `real_legacy_simple_task` | `host.clone` | [890](../../src/process_live_tests.rs#L890), [891](../../src/process_live_tests.rs#L891), [906](../../src/process_live_tests.rs#L906) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root, unary, host.clone(),             transport::TransportConfig::loopback(address, transport::BearerToken::new([42;32]))).unwrap` | [891](../../src/process_live_tests.rs#L891) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [891](../../src/process_live_tests.rs#L891) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_legacy_simple_task` | `transport::TransportConfig::loopback` | [892](../../src/process_live_tests.rs#L892) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_legacy_simple_task` | `transport::BearerToken::new` | [892](../../src/process_live_tests.rs#L892) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_legacy_simple_task` | `host.attach_streams` | [893](../../src/process_live_tests.rs#L893) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams().clone` | [893](../../src/process_live_tests.rs#L893) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.streams` | [893](../../src/process_live_tests.rs#L893) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery().unwrap` | [894](../../src/process_live_tests.rs#L894) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.finish_recovery` | [894](../../src/process_live_tests.rs#L894) | receiver-type-required |
| `real_legacy_simple_task` | `assembly.into_server` | [895](../../src/process_live_tests.rs#L895) | receiver-type-required |
| `real_legacy_simple_task` | `server.handle` | [896](../../src/process_live_tests.rs#L896) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.spawn` | [897](../../src/process_live_tests.rs#L897) | receiver-type-required |
| `real_legacy_simple_task` | `server.serve` | [897](../../src/process_live_tests.rs#L897) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [898](../../src/process_live_tests.rs#L898) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec` | [898](../../src/process_live_tests.rs#L898) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Instant::now` | [899](../../src/process_live_tests.rs#L899), [910](../../src/process_live_tests.rs#L910) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_secs` | [899](../../src/process_live_tests.rs#L899), [910](../../src/process_live_tests.rs#L910) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `root.join("client-ready").exists` | [900](../../src/process_live_tests.rs#L900) | receiver-type-required |
| `real_legacy_simple_task` | `std::thread::sleep` | [902](../../src/process_live_tests.rs#L902), [933](../../src/process_live_tests.rs#L933) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Duration::from_millis` | [902](../../src/process_live_tests.rs#L902), [933](../../src/process_live_tests.rs#L933) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `Some` | [904](../../src/process_live_tests.rs#L904), [915](../../src/process_live_tests.rs#L915) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open(&root, Arc::new(crate::endpoint_host::SessionAdmissionGates::default()), host.clone()).unwrap` | [906](../../src/process_live_tests.rs#L906) | receiver-type-required |
| `real_legacy_simple_task` | `crate::endpoint_carrier::ProductionRespondAuthority::open` | [906](../../src/process_live_tests.rs#L906) | [tekes-supervisor::endpoint_carrier::ProductionRespondAuthority::open](../../src/endpoint_carrier.rs#L176) |
| `real_legacy_simple_task` | `crate::endpoint_host::SessionAdmissionGates::default` | [906](../../src/process_live_tests.rs#L906) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::catch_unwind` | [907](../../src/process_live_tests.rs#L907) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `std::panic::AssertUnwindSafe` | [907](../../src/process_live_tests.rs#L907) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap().expect` | [908](../../src/process_live_tests.rs#L908) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main(session).unwrap` | [908](../../src/process_live_tests.rs#L908) | receiver-type-required |
| `real_legacy_simple_task` | `host.schedule_main` | [908](../../src/process_live_tests.rs#L908) | receiver-type-required |
| `real_legacy_simple_task` | `host.start_periodic_sweep` | [909](../../src/process_live_tests.rs#L909) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124; serde_json::from_str(line).ok()).collect` | [912](../../src/process_live_tests.rs#L912) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [912](../../src/process_live_tests.rs#L912) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::from_str(line).ok` | [912](../../src/process_live_tests.rs#L912), [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir(&folder).unwrap` | [913](../../src/process_live_tests.rs#L913) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_dir` | [913](../../src/process_live_tests.rs#L913) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `entry.unwrap().path` | [914](../../src/process_live_tests.rs#L914) | receiver-type-required |
| `real_legacy_simple_task` | `entry.unwrap` | [914](../../src/process_live_tests.rs#L914) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension().and_then` | [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_legacy_simple_task` | `file.extension` | [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_legacy_simple_task` | `v.to_str` | [915](../../src/process_live_tests.rs#L915) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name().unwrap` | [915](../../src/process_live_tests.rs#L915), [926](../../src/process_live_tests.rs#L926) | receiver-type-required |
| `real_legacy_simple_task` | `file.file_name` | [915](../../src/process_live_tests.rs#L915), [926](../../src/process_live_tests.rs#L926) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines().filter_map` | [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap().lines` | [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_legacy_simple_task` | `fs::read_to_string(&file).unwrap` | [916](../../src/process_live_tests.rs#L916) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().filter` | [917](../../src/process_live_tests.rs#L917) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter` | [917](../../src/process_live_tests.rs#L917), [918](../../src/process_live_tests.rs#L918), [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().any` | [918](../../src/process_live_tests.rs#L918) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["call"]==held["call"]).unwrap` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_legacy_simple_task` | `facts.iter().find` | [919](../../src/process_live_tests.rs#L919) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str().unwrap` | [922](../../src/process_live_tests.rs#L922) | receiver-type-required |
| `real_legacy_simple_task` | `call["args"]["path"].as_str` | [922](../../src/process_live_tests.rs#L922) | receiver-type-required |
| `real_legacy_simple_task` | `raw.is_absolute` | [923](../../src/process_live_tests.rs#L923) | receiver-type-required |
| `real_legacy_simple_task` | `session.to_owned` | [926](../../src/process_live_tests.rs#L926) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author(&responder,endpoint::RespondAuthorization {rpc_id:key.clone(),request_sha256:"live-task-approval".into(),session_id:target,call:held["call"].as_str().unwrap().into(),grant:true,answer:None,origin_key:key,resolution:endpoint::ResolutionOutcome::AllowedOnce}).expect` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `endpoint::RespondAuthority::author` | [928](../../src/process_live_tests.rs#L928) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `key.clone` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `"live-task-approval".into` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap().into` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str().unwrap` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `held["call"].as_str` | [928](../../src/process_live_tests.rs#L928) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter().any` | [931](../../src/process_live_tests.rs#L931) | receiver-type-required |
| `real_legacy_simple_task` | `events.iter` | [931](../../src/process_live_tests.rs#L931) | receiver-type-required |
| `real_legacy_simple_task` | `root.join("client-task-receipt.json").exists` | [931](../../src/process_live_tests.rs#L931) | receiver-type-required |
| `real_legacy_simple_task` | `host.shutdown` | [936](../../src/process_live_tests.rs#L936) | receiver-type-required |
| `real_legacy_simple_task` | `handle.begin_drain` | [938](../../src/process_live_tests.rs#L938) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap().unwrap` | [939](../../src/process_live_tests.rs#L939) | receiver-type-required |
| `real_legacy_simple_task` | `runtime.block_on(serve).unwrap` | [939](../../src/process_live_tests.rs#L939) | receiver-type-required |
| `real_legacy_simple_task` | `std::panic::resume_unwind` | [941](../../src/process_live_tests.rs#L941) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [942](../../src/process_live_tests.rs#L942) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::Map::new` | [949](../../src/process_live_tests.rs#L949) | external-constructor-callback-or-unresolved |
| `real_legacy_simple_task` | `fs::read_to_string(workspace.join(relative)).expect` | [951](../../src/process_live_tests.rs#L951) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase().replace` | [961](../../src/process_live_tests.rs#L961) | receiver-type-required |
| `real_legacy_simple_task` | `body.to_lowercase` | [961](../../src/process_live_tests.rs#L961) | receiver-type-required |
| `real_legacy_simple_task` | `artifacts.insert` | [971](../../src/process_live_tests.rs#L971) | receiver-type-required |
| `real_legacy_simple_task` | `(*relative).into` | [971](../../src/process_live_tests.rs#L971) | receiver-type-required |
| `real_legacy_simple_task` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap()).unwrap` | [977](../../src/process_live_tests.rs#L977) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty(&json!({"status":"passed","artifacts":artifacts})).unwrap` | [977](../../src/process_live_tests.rs#L977) | receiver-type-required |
| `real_legacy_simple_task` | `serde_json::to_vec_pretty` | [977](../../src/process_live_tests.rs#L977) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PUBLIC_QUESTION").is_ok_and` | [984](../../src/process_live_tests.rs#L984) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var` | [984](../../src/process_live_tests.rs#L984), [985](../../src/process_live_tests.rs#L985), [989](../../src/process_live_tests.rs#L989), [998](../../src/process_live_tests.rs#L998), [999](../../src/process_live_tests.rs#L999) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `PathBuf::from` | [985](../../src/process_live_tests.rs#L985) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [985](../../src/process_live_tests.rs#L985) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap().join` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent().unwrap` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.parent` | [986](../../src/process_live_tests.rs#L986) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(&workspace).unwrap` | [987](../../src/process_live_tests.rs#L987) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all` | [987](../../src/process_live_tests.rs#L987), [988](../../src/process_live_tests.rs#L988), [994](../../src/process_live_tests.rs#L994) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(root.join("config")).unwrap` | [988](../../src/process_live_tests.rs#L988) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join` | [988](../../src/process_live_tests.rs#L988), [990](../../src/process_live_tests.rs#L990), [993](../../src/process_live_tests.rs#L993), [999](../../src/process_live_tests.rs#L999), [1005](../../src/process_live_tests.rs#L1005), [1013](../../src/process_live_tests.rs#L1013), [1020](../../src/process_live_tests.rs#L1020), [1022](../../src/process_live_tests.rs#L1022), [1059](../../src/process_live_tests.rs#L1059) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [989](../../src/process_live_tests.rs#L989) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_slice` | [989](../../src/process_live_tests.rs#L989) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [989](../../src/process_live_tests.rs#L989) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read` | [989](../../src/process_live_tests.rs#L989) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [989](../../src/process_live_tests.rs#L989) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_canonical_test_json` | [990](../../src/process_live_tests.rs#L990) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `write_test_execution_config` | [991](../../src/process_live_tests.rs#L991) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("threads").join` | [993](../../src/process_live_tests.rs#L993) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::create_dir_all(folder.join("assets")).unwrap` | [994](../../src/process_live_tests.rs#L994) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `folder.join` | [994](../../src/process_live_tests.rs#L994), [995](../../src/process_live_tests.rs#L995) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `write_test_genesis` | [996](../../src/process_live_tests.rs#L996) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Arc::new` | [997](../../src/process_live_tests.rs#L997), [1005](../../src/process_live_tests.rs#L1005) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `provider::MemorySecretStore::new` | [997](../../src/process_live_tests.rs#L997) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `secrets.publish` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str().unwrap` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `provider["credential_key"].as_str` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [998](../../src/process_live_tests.rs#L998) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).inspect` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `ProductionProcessHost::open_with_secret_store` | [999](../../src/process_live_tests.rs#L999) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [999](../../src/process_live_tests.rs#L999) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `attach_live_stdio` | [999](../../src/process_live_tests.rs#L999) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new().unwrap` | [1000](../../src/process_live_tests.rs#L1000) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::runtime::Runtime::new` | [1000](../../src/process_live_tests.rs#L1000) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1001](../../src/process_live_tests.rs#L1001) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on` | [1001](../../src/process_live_tests.rs#L1001), [1029](../../src/process_live_tests.rs#L1029) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `tokio::net::TcpListener::bind` | [1001](../../src/process_live_tests.rs#L1001) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr().unwrap` | [1002](../../src/process_live_tests.rs#L1002) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `listener.local_addr` | [1002](../../src/process_live_tests.rs#L1002) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1003](../../src/process_live_tests.rs#L1003) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::host_runtime::assemble_application_endpoint_host` | [1003](../../src/process_live_tests.rs#L1003) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_provider_400_releases_queue_over_public_transport` | `"test".into` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy().into_owned` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `workspace.to_string_lossy` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy().into_owned` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.to_string_lossy` | [1004](../../src/process_live_tests.rs#L1004) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `Ok` | [1005](../../src/process_live_tests.rs#L1005) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `"2026-09-04T10:00:00.000Z".into` | [1005](../../src/process_live_tests.rs#L1005) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `host.clone` | [1005](../../src/process_live_tests.rs#L1005), [1006](../../src/process_live_tests.rs#L1006) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1006](../../src/process_live_tests.rs#L1006) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1006](../../src/process_live_tests.rs#L1006) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::TransportConfig::loopback` | [1007](../../src/process_live_tests.rs#L1007) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_provider_400_releases_queue_over_public_transport` | `transport::BearerToken::new` | [1007](../../src/process_live_tests.rs#L1007) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_provider_400_releases_queue_over_public_transport` | `host.attach_streams` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams().clone` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.streams` | [1008](../../src/process_live_tests.rs#L1008) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery().unwrap` | [1009](../../src/process_live_tests.rs#L1009) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.finish_recovery` | [1009](../../src/process_live_tests.rs#L1009) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `assembly.into_server` | [1010](../../src/process_live_tests.rs#L1010) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.handle` | [1011](../../src/process_live_tests.rs#L1011) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.spawn` | [1012](../../src/process_live_tests.rs#L1012) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `server.serve` | [1012](../../src/process_live_tests.rs#L1012) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1013](../../src/process_live_tests.rs#L1013) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write` | [1013](../../src/process_live_tests.rs#L1013), [1020](../../src/process_live_tests.rs#L1020), [1059](../../src/process_live_tests.rs#L1059) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1013](../../src/process_live_tests.rs#L1013) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec` | [1013](../../src/process_live_tests.rs#L1013) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.start_periodic_sweep` | [1014](../../src/process_live_tests.rs#L1014) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::catch_unwind` | [1015](../../src/process_live_tests.rs#L1015) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::AssertUnwindSafe` | [1015](../../src/process_live_tests.rs#L1015) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Instant::now` | [1016](../../src/process_live_tests.rs#L1016) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_secs` | [1016](../../src/process_live_tests.rs#L1016) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;line&#124;serde_json::from_str(line).ok()).collect` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines` | [1018](../../src/process_live_tests.rs#L1018), [1031](../../src/process_live_tests.rs#L1031) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap` | [1018](../../src/process_live_tests.rs#L1018), [1031](../../src/process_live_tests.rs#L1031) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string` | [1018](../../src/process_live_tests.rs#L1018), [1031](../../src/process_live_tests.rs#L1031) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).ok` | [1018](../../src/process_live_tests.rs#L1018) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str` | [1018](../../src/process_live_tests.rs#L1018), [1031](../../src/process_live_tests.rs#L1031) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").count` | [1019](../../src/process_live_tests.rs#L1019) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter` | [1019](../../src/process_live_tests.rs#L1019), [1032](../../src/process_live_tests.rs#L1032), [1036](../../src/process_live_tests.rs#L1036), [1039](../../src/process_live_tests.rs#L1039), [1051](../../src/process_live_tests.rs#L1051) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter` | [1019](../../src/process_live_tests.rs#L1019), [1032](../../src/process_live_tests.rs#L1032), [1036](../../src/process_live_tests.rs#L1036), [1039](../../src/process_live_tests.rs#L1039), [1046](../../src/process_live_tests.rs#L1046), [1050](../../src/process_live_tests.rs#L1050), [1051](../../src/process_live_tests.rs#L1051), [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("queued-input-observed"),"durable").unwrap` | [1020](../../src/process_live_tests.rs#L1020) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `root.join("client-error-queue-receipt.json").exists` | [1022](../../src/process_live_tests.rs#L1022) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::thread::sleep` | [1024](../../src/process_live_tests.rs#L1024) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `Duration::from_millis` | [1024](../../src/process_live_tests.rs#L1024) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `host.shutdown` | [1027](../../src/process_live_tests.rs#L1027) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `handle.begin_drain` | [1028](../../src/process_live_tests.rs#L1028) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap().unwrap` | [1029](../../src/process_live_tests.rs#L1029) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `runtime.block_on(serve).unwrap` | [1029](../../src/process_live_tests.rs#L1029) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `std::panic::resume_unwind` | [1030](../../src/process_live_tests.rs#L1030) | external-constructor-callback-or-unresolved |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1031](../../src/process_live_tests.rs#L1031) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::read_to_string(&path).unwrap().lines().map` | [1031](../../src/process_live_tests.rs#L1031) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::from_str(line).unwrap` | [1031](../../src/process_live_tests.rs#L1031) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1032](../../src/process_live_tests.rs#L1032) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="input").collect` | [1036](../../src/process_live_tests.rs#L1036) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="turn_open").collect` | [1039](../../src/process_live_tests.rs#L1039) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="think").unwrap` | [1046](../../src/process_live_tests.rs#L1046) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find` | [1046](../../src/process_live_tests.rs#L1046), [1050](../../src/process_live_tests.rs#L1050), [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="ask_user_questions").unwrap` | [1050](../../src/process_live_tests.rs#L1050) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().filter(&#124;e&#124;e["kind"]=="approval_response" && e["call"]==question_call["call"]).collect` | [1051](../../src/process_live_tests.rs#L1051) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `events.iter().find(&#124;e&#124;e["kind"]=="output" && e["final_answer"]==true).unwrap` | [1056](../../src/process_live_tests.rs#L1056) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap()).unwrap` | [1059](../../src/process_live_tests.rs#L1059) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"queued_seq":inputs[1]["seq"],"second_open_seq":opened[1]["seq"],"think_seq":think["seq"],"answer_seq":answer["seq"]})).unwrap` | [1059](../../src/process_live_tests.rs#L1059) | receiver-type-required |
| `real_provider_400_releases_queue_over_public_transport` | `serde_json::to_vec_pretty` | [1059](../../src/process_live_tests.rs#L1059) | external-constructor-callback-or-unresolved |
| `register_local_tools_server` | `stage_live_stdio` | [1067](../../src/process_live_tests.rs#L1067) | [tekes-supervisor::process_live_tests::stage_live_stdio](../../src/process_live_tests.rs#L1504) |
| `real_public_deferred_tool_search` | `PathBuf::from` | [1074](../../src/process_live_tests.rs#L1074) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1074](../../src/process_live_tests.rs#L1074) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var` | [1074](../../src/process_live_tests.rs#L1074), [1078](../../src/process_live_tests.rs#L1078), [1083](../../src/process_live_tests.rs#L1083), [1103](../../src/process_live_tests.rs#L1103), [1104](../../src/process_live_tests.rs#L1104) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.parent().unwrap().join` | [1075](../../src/process_live_tests.rs#L1075) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent().unwrap` | [1075](../../src/process_live_tests.rs#L1075) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.parent` | [1075](../../src/process_live_tests.rs#L1075) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(&workspace).unwrap` | [1076](../../src/process_live_tests.rs#L1076) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all` | [1076](../../src/process_live_tests.rs#L1076), [1077](../../src/process_live_tests.rs#L1077), [1099](../../src/process_live_tests.rs#L1099) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::create_dir_all(root.join("config")).unwrap` | [1077](../../src/process_live_tests.rs#L1077) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.join` | [1077](../../src/process_live_tests.rs#L1077), [1079](../../src/process_live_tests.rs#L1079), [1098](../../src/process_live_tests.rs#L1098), [1104](../../src/process_live_tests.rs#L1104), [1110](../../src/process_live_tests.rs#L1110), [1118](../../src/process_live_tests.rs#L1118), [1123](../../src/process_live_tests.rs#L1123), [1181](../../src/process_live_tests.rs#L1181) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_slice` | [1078](../../src/process_live_tests.rs#L1078) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read` | [1078](../../src/process_live_tests.rs#L1078) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1078](../../src/process_live_tests.rs#L1078) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_canonical_test_json` | [1079](../../src/process_live_tests.rs#L1079) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TOOL_SEARCH_SCENARIO").is_ok_and` | [1083](../../src/process_live_tests.rs#L1083) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_test_execution_config` | [1095](../../src/process_live_tests.rs#L1095) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `register_local_tools_server` | [1096](../../src/process_live_tests.rs#L1096) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1066) |
| `real_public_deferred_tool_search` | `root.join("threads").join` | [1098](../../src/process_live_tests.rs#L1098) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1099](../../src/process_live_tests.rs#L1099) | receiver-type-required |
| `real_public_deferred_tool_search` | `folder.join` | [1099](../../src/process_live_tests.rs#L1099), [1100](../../src/process_live_tests.rs#L1100) | receiver-type-required |
| `real_public_deferred_tool_search` | `write_test_genesis` | [1101](../../src/process_live_tests.rs#L1101) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Arc::new` | [1102](../../src/process_live_tests.rs#L1102), [1110](../../src/process_live_tests.rs#L1110) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider::MemorySecretStore::new` | [1102](../../src/process_live_tests.rs#L1102) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_deferred_tool_search` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `secrets.publish` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str().unwrap` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider["credential_key"].as_str` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1103](../../src/process_live_tests.rs#L1103) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-deferred-tool",root.join(".agent"),secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-deferred-tool",root.join(".agent"),secrets).inspect` | [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `ProductionProcessHost::open_with_secret_store` | [1104](../../src/process_live_tests.rs#L1104) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1104](../../src/process_live_tests.rs#L1104) | receiver-type-required |
| `real_public_deferred_tool_search` | `attach_live_stdio` | [1104](../../src/process_live_tests.rs#L1104) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new().unwrap` | [1105](../../src/process_live_tests.rs#L1105) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::runtime::Runtime::new` | [1105](../../src/process_live_tests.rs#L1105) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1106](../../src/process_live_tests.rs#L1106) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on` | [1106](../../src/process_live_tests.rs#L1106), [1135](../../src/process_live_tests.rs#L1135) | receiver-type-required |
| `real_public_deferred_tool_search` | `tokio::net::TcpListener::bind` | [1106](../../src/process_live_tests.rs#L1106) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `listener.local_addr().unwrap` | [1107](../../src/process_live_tests.rs#L1107) | receiver-type-required |
| `real_public_deferred_tool_search` | `listener.local_addr` | [1107](../../src/process_live_tests.rs#L1107) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1108](../../src/process_live_tests.rs#L1108) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::host_runtime::assemble_application_endpoint_host` | [1108](../../src/process_live_tests.rs#L1108) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_public_deferred_tool_search` | `"test".into` | [1109](../../src/process_live_tests.rs#L1109) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy().into_owned` | [1109](../../src/process_live_tests.rs#L1109) | receiver-type-required |
| `real_public_deferred_tool_search` | `workspace.to_string_lossy` | [1109](../../src/process_live_tests.rs#L1109) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy().into_owned` | [1109](../../src/process_live_tests.rs#L1109) | receiver-type-required |
| `real_public_deferred_tool_search` | `root.to_string_lossy` | [1109](../../src/process_live_tests.rs#L1109) | receiver-type-required |
| `real_public_deferred_tool_search` | `Ok` | [1110](../../src/process_live_tests.rs#L1110) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `"2026-09-04T10:00:00.000Z".into` | [1110](../../src/process_live_tests.rs#L1110) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.clone` | [1110](../../src/process_live_tests.rs#L1110), [1111](../../src/process_live_tests.rs#L1111) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1111](../../src/process_live_tests.rs#L1111) | receiver-type-required |
| `real_public_deferred_tool_search` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1111](../../src/process_live_tests.rs#L1111) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_public_deferred_tool_search` | `transport::TransportConfig::loopback` | [1112](../../src/process_live_tests.rs#L1112) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_public_deferred_tool_search` | `transport::BearerToken::new` | [1112](../../src/process_live_tests.rs#L1112) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_deferred_tool_search` | `host.attach_streams` | [1113](../../src/process_live_tests.rs#L1113) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams().clone` | [1113](../../src/process_live_tests.rs#L1113) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.streams` | [1113](../../src/process_live_tests.rs#L1113) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery().unwrap` | [1114](../../src/process_live_tests.rs#L1114) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.finish_recovery` | [1114](../../src/process_live_tests.rs#L1114) | receiver-type-required |
| `real_public_deferred_tool_search` | `assembly.into_server` | [1115](../../src/process_live_tests.rs#L1115) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.handle` | [1116](../../src/process_live_tests.rs#L1116) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.spawn` | [1117](../../src/process_live_tests.rs#L1117) | receiver-type-required |
| `real_public_deferred_tool_search` | `server.serve` | [1117](../../src/process_live_tests.rs#L1117) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1118](../../src/process_live_tests.rs#L1118) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write` | [1118](../../src/process_live_tests.rs#L1118), [1181](../../src/process_live_tests.rs#L1181) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1118](../../src/process_live_tests.rs#L1118) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec` | [1118](../../src/process_live_tests.rs#L1118) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.start_periodic_sweep` | [1119](../../src/process_live_tests.rs#L1119) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::catch_unwind` | [1120](../../src/process_live_tests.rs#L1120) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `std::panic::AssertUnwindSafe` | [1120](../../src/process_live_tests.rs#L1120) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Instant::now` | [1121](../../src/process_live_tests.rs#L1121) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_secs` | [1121](../../src/process_live_tests.rs#L1121) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `root.join("client-tool-search-receipt.json").exists` | [1123](../../src/process_live_tests.rs#L1123) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().count` | [1125](../../src/process_live_tests.rs#L1125) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines` | [1125](../../src/process_live_tests.rs#L1125), [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap` | [1125](../../src/process_live_tests.rs#L1125), [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string` | [1125](../../src/process_live_tests.rs#L1125), [1137](../../src/process_live_tests.rs#L1137) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.ensure_running(session).expect` | [1126](../../src/process_live_tests.rs#L1126) | receiver-type-required |
| `real_public_deferred_tool_search` | `host.ensure_running` | [1126](../../src/process_live_tests.rs#L1126) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::thread::sleep` | [1130](../../src/process_live_tests.rs#L1130) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `Duration::from_millis` | [1130](../../src/process_live_tests.rs#L1130) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `host.shutdown` | [1133](../../src/process_live_tests.rs#L1133) | receiver-type-required |
| `real_public_deferred_tool_search` | `handle.begin_drain` | [1134](../../src/process_live_tests.rs#L1134) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap().unwrap` | [1135](../../src/process_live_tests.rs#L1135) | receiver-type-required |
| `real_public_deferred_tool_search` | `runtime.block_on(serve).unwrap` | [1135](../../src/process_live_tests.rs#L1135) | receiver-type-required |
| `real_public_deferred_tool_search` | `std::panic::resume_unwind` | [1136](../../src/process_live_tests.rs#L1136) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::read_to_string(&path).unwrap().lines().map` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(line).unwrap` | [1137](../../src/process_live_tests.rs#L1137) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str` | [1137](../../src/process_live_tests.rs#L1137), [1151](../../src/process_live_tests.rs#L1151) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1138](../../src/process_live_tests.rs#L1138) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter` | [1138](../../src/process_live_tests.rs#L1138), [1156](../../src/process_live_tests.rs#L1156), [1174](../../src/process_live_tests.rs#L1174) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter` | [1138](../../src/process_live_tests.rs#L1138), [1143](../../src/process_live_tests.rs#L1143), [1144](../../src/process_live_tests.rs#L1144), [1145](../../src/process_live_tests.rs#L1145), [1148](../../src/process_live_tests.rs#L1148), [1156](../../src/process_live_tests.rs#L1156), [1174](../../src/process_live_tests.rs#L1174) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from(turn).unwrap` | [1141](../../src/process_live_tests.rs#L1141) | receiver-type-required |
| `real_public_deferred_tool_search` | `usize::try_from` | [1141](../../src/process_live_tests.rs#L1141) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]=="tool_search" && e["turn"]==turn).expect` | [1143](../../src/process_live_tests.rs#L1143) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find` | [1143](../../src/process_live_tests.rs#L1143), [1144](../../src/process_live_tests.rs#L1144), [1145](../../src/process_live_tests.rs#L1145), [1148](../../src/process_live_tests.rs#L1148) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==search["call"] && e["outcome"]=="ok").unwrap` | [1144](../../src/process_live_tests.rs#L1144) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_call" && e["name"]==deferred_tool && e["turn"]==turn).expect` | [1145](../../src/process_live_tests.rs#L1145) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().find(&#124;e&#124;e["kind"]=="tool_result" && e["call"]==call["call"] && e["outcome"]=="ok").unwrap` | [1148](../../src/process_live_tests.rs#L1148) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_str(output["content"][0]["text"].as_str().unwrap()).unwrap` | [1151](../../src/process_live_tests.rs#L1151) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str().unwrap` | [1151](../../src/process_live_tests.rs#L1151) | receiver-type-required |
| `real_public_deferred_tool_search` | `output["content"][0]["text"].as_str` | [1151](../../src/process_live_tests.rs#L1151) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str().expect` | [1153](../../src/process_live_tests.rs#L1153) | receiver-type-required |
| `real_public_deferred_tool_search` | `envelope["content"][0]["text"].as_str` | [1153](../../src/process_live_tests.rs#L1153) | receiver-type-required |
| `real_public_deferred_tool_search` | `events.iter().filter(&#124;e&#124;e["kind"]=="epoch").collect` | [1156](../../src/process_live_tests.rs#L1156) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value(provider.clone()).unwrap` | [1163](../../src/process_live_tests.rs#L1163) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::from_value` | [1163](../../src/process_live_tests.rs#L1163) | external-constructor-callback-or-unresolved |
| `real_public_deferred_tool_search` | `provider.clone` | [1163](../../src/process_live_tests.rs#L1163) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools().is_some` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap().native_deferred_tools` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile(&configured, &configured.models[0]).unwrap` | [1164](../../src/process_live_tests.rs#L1164) | receiver-type-required |
| `real_public_deferred_tool_search` | `provider::resolve_profile` | [1164](../../src/process_live_tests.rs#L1164) | [provider::dialect::resolve_profile](../../../provider/src/dialect.rs#L692) |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap().to_owned` | [1165](../../src/process_live_tests.rs#L1165) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str().unwrap` | [1165](../../src/process_live_tests.rs#L1165) | receiver-type-required |
| `real_public_deferred_tool_search` | `epoch["tools"]["digest"].as_str` | [1165](../../src/process_live_tests.rs#L1165) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find(&#124;e&#124;e["id"]==attempt["epoch"]).unwrap` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter().find` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_deferred_tool_search` | `epochs.iter` | [1175](../../src/process_live_tests.rs#L1175) | receiver-type-required |
| `real_public_deferred_tool_search` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap()).unwrap` | [1181](../../src/process_live_tests.rs#L1181) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty(&json!({"status":"passed","native_deferred_routing":native,"settles":settles,"epochs":epochs})).unwrap` | [1181](../../src/process_live_tests.rs#L1181) | receiver-type-required |
| `real_public_deferred_tool_search` | `serde_json::to_vec_pretty` | [1181](../../src/process_live_tests.rs#L1181) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `PathBuf::from` | [1188](../../src/process_live_tests.rs#L1188) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1188](../../src/process_live_tests.rs#L1188) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var` | [1188](../../src/process_live_tests.rs#L1188), [1192](../../src/process_live_tests.rs#L1192), [1206](../../src/process_live_tests.rs#L1206), [1207](../../src/process_live_tests.rs#L1207) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.parent().unwrap().join` | [1189](../../src/process_live_tests.rs#L1189) | receiver-type-required |
| `real_public_image_attachment` | `root.parent().unwrap` | [1189](../../src/process_live_tests.rs#L1189) | receiver-type-required |
| `real_public_image_attachment` | `root.parent` | [1189](../../src/process_live_tests.rs#L1189) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(&workspace).unwrap` | [1190](../../src/process_live_tests.rs#L1190) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all` | [1190](../../src/process_live_tests.rs#L1190), [1191](../../src/process_live_tests.rs#L1191), [1202](../../src/process_live_tests.rs#L1202) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::create_dir_all(root.join("config")).unwrap` | [1191](../../src/process_live_tests.rs#L1191) | receiver-type-required |
| `real_public_image_attachment` | `root.join` | [1191](../../src/process_live_tests.rs#L1191), [1193](../../src/process_live_tests.rs#L1193), [1201](../../src/process_live_tests.rs#L1201), [1207](../../src/process_live_tests.rs#L1207), [1213](../../src/process_live_tests.rs#L1213), [1221](../../src/process_live_tests.rs#L1221), [1226](../../src/process_live_tests.rs#L1226), [1253](../../src/process_live_tests.rs#L1253) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_slice` | [1192](../../src/process_live_tests.rs#L1192) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_image_attachment` | `fs::read` | [1192](../../src/process_live_tests.rs#L1192) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1192](../../src/process_live_tests.rs#L1192) | receiver-type-required |
| `real_public_image_attachment` | `write_canonical_test_json` | [1193](../../src/process_live_tests.rs#L1193) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map(&#124;tool&#124; tool.name).collect` | [1196](../../src/process_live_tests.rs#L1196) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter(&#124;tool&#124; !matches!(tool.name.as_str(), "plan" &#124; "summary_artifact" &#124; "report" &#124; "verify"))         .map` | [1196](../../src/process_live_tests.rs#L1196) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter()         .filter` | [1196](../../src/process_live_tests.rs#L1196) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled().tools.into_iter` | [1196](../../src/process_live_tests.rs#L1196) | receiver-type-required |
| `real_public_image_attachment` | `tools::BuiltinManifest::compiled` | [1196](../../src/process_live_tests.rs#L1196) | [tools::builtin::BuiltinManifest::compiled](../../../tools/src/builtin.rs#L315) |
| `real_public_image_attachment` | `write_test_execution_config` | [1199](../../src/process_live_tests.rs#L1199) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.join("threads").join` | [1201](../../src/process_live_tests.rs#L1201) | receiver-type-required |
| `real_public_image_attachment` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1202](../../src/process_live_tests.rs#L1202) | receiver-type-required |
| `real_public_image_attachment` | `folder.join` | [1202](../../src/process_live_tests.rs#L1202), [1203](../../src/process_live_tests.rs#L1203) | receiver-type-required |
| `real_public_image_attachment` | `write_test_genesis` | [1204](../../src/process_live_tests.rs#L1204) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Arc::new` | [1205](../../src/process_live_tests.rs#L1205), [1213](../../src/process_live_tests.rs#L1213) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `provider::MemorySecretStore::new` | [1205](../../src/process_live_tests.rs#L1205) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_image_attachment` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1206](../../src/process_live_tests.rs#L1206) | receiver-type-required |
| `real_public_image_attachment` | `secrets.publish` | [1206](../../src/process_live_tests.rs#L1206) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str().unwrap` | [1206](../../src/process_live_tests.rs#L1206) | receiver-type-required |
| `real_public_image_attachment` | `provider["credential_key"].as_str` | [1206](../../src/process_live_tests.rs#L1206) | receiver-type-required |
| `real_public_image_attachment` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1206](../../src/process_live_tests.rs#L1206) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-image",root.join(".agent"),secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [1207](../../src/process_live_tests.rs#L1207) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-image",root.join(".agent"),secrets).inspect` | [1207](../../src/process_live_tests.rs#L1207) | receiver-type-required |
| `real_public_image_attachment` | `ProductionProcessHost::open_with_secret_store` | [1207](../../src/process_live_tests.rs#L1207) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1207](../../src/process_live_tests.rs#L1207) | receiver-type-required |
| `real_public_image_attachment` | `attach_live_stdio` | [1207](../../src/process_live_tests.rs#L1207) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new().unwrap` | [1208](../../src/process_live_tests.rs#L1208) | receiver-type-required |
| `real_public_image_attachment` | `tokio::runtime::Runtime::new` | [1208](../../src/process_live_tests.rs#L1208) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1209](../../src/process_live_tests.rs#L1209) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on` | [1209](../../src/process_live_tests.rs#L1209), [1238](../../src/process_live_tests.rs#L1238) | receiver-type-required |
| `real_public_image_attachment` | `tokio::net::TcpListener::bind` | [1209](../../src/process_live_tests.rs#L1209) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `listener.local_addr().unwrap` | [1210](../../src/process_live_tests.rs#L1210) | receiver-type-required |
| `real_public_image_attachment` | `listener.local_addr` | [1210](../../src/process_live_tests.rs#L1210) | receiver-type-required |
| `real_public_image_attachment` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1211](../../src/process_live_tests.rs#L1211) | receiver-type-required |
| `real_public_image_attachment` | `crate::host_runtime::assemble_application_endpoint_host` | [1211](../../src/process_live_tests.rs#L1211) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_public_image_attachment` | `"test".into` | [1212](../../src/process_live_tests.rs#L1212) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy().into_owned` | [1212](../../src/process_live_tests.rs#L1212) | receiver-type-required |
| `real_public_image_attachment` | `workspace.to_string_lossy` | [1212](../../src/process_live_tests.rs#L1212) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy().into_owned` | [1212](../../src/process_live_tests.rs#L1212) | receiver-type-required |
| `real_public_image_attachment` | `root.to_string_lossy` | [1212](../../src/process_live_tests.rs#L1212) | receiver-type-required |
| `real_public_image_attachment` | `Ok` | [1213](../../src/process_live_tests.rs#L1213) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `"2026-09-04T10:00:00.000Z".into` | [1213](../../src/process_live_tests.rs#L1213) | receiver-type-required |
| `real_public_image_attachment` | `host.clone` | [1213](../../src/process_live_tests.rs#L1213), [1214](../../src/process_live_tests.rs#L1214) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1214](../../src/process_live_tests.rs#L1214) | receiver-type-required |
| `real_public_image_attachment` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1214](../../src/process_live_tests.rs#L1214) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_public_image_attachment` | `transport::TransportConfig::loopback` | [1215](../../src/process_live_tests.rs#L1215) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_public_image_attachment` | `transport::BearerToken::new` | [1215](../../src/process_live_tests.rs#L1215) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_image_attachment` | `host.attach_streams` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams().clone` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `assembly.streams` | [1216](../../src/process_live_tests.rs#L1216) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery().unwrap` | [1217](../../src/process_live_tests.rs#L1217) | receiver-type-required |
| `real_public_image_attachment` | `assembly.finish_recovery` | [1217](../../src/process_live_tests.rs#L1217) | receiver-type-required |
| `real_public_image_attachment` | `assembly.into_server` | [1218](../../src/process_live_tests.rs#L1218) | receiver-type-required |
| `real_public_image_attachment` | `server.handle` | [1219](../../src/process_live_tests.rs#L1219) | receiver-type-required |
| `real_public_image_attachment` | `runtime.spawn` | [1220](../../src/process_live_tests.rs#L1220) | receiver-type-required |
| `real_public_image_attachment` | `server.serve` | [1220](../../src/process_live_tests.rs#L1220) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1221](../../src/process_live_tests.rs#L1221) | receiver-type-required |
| `real_public_image_attachment` | `fs::write` | [1221](../../src/process_live_tests.rs#L1221), [1253](../../src/process_live_tests.rs#L1253) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1221](../../src/process_live_tests.rs#L1221) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec` | [1221](../../src/process_live_tests.rs#L1221) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.start_periodic_sweep` | [1222](../../src/process_live_tests.rs#L1222) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::catch_unwind` | [1223](../../src/process_live_tests.rs#L1223) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `std::panic::AssertUnwindSafe` | [1223](../../src/process_live_tests.rs#L1223) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Instant::now` | [1224](../../src/process_live_tests.rs#L1224) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_secs` | [1224](../../src/process_live_tests.rs#L1224) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `root.join("client-image-receipt.json").exists` | [1226](../../src/process_live_tests.rs#L1226) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().count` | [1228](../../src/process_live_tests.rs#L1228) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines` | [1228](../../src/process_live_tests.rs#L1228), [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap` | [1228](../../src/process_live_tests.rs#L1228), [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string` | [1228](../../src/process_live_tests.rs#L1228), [1240](../../src/process_live_tests.rs#L1240) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.ensure_running(session).expect` | [1229](../../src/process_live_tests.rs#L1229) | receiver-type-required |
| `real_public_image_attachment` | `host.ensure_running` | [1229](../../src/process_live_tests.rs#L1229) | receiver-type-required |
| `real_public_image_attachment` | `std::thread::sleep` | [1233](../../src/process_live_tests.rs#L1233) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `Duration::from_millis` | [1233](../../src/process_live_tests.rs#L1233) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `host.shutdown` | [1236](../../src/process_live_tests.rs#L1236) | receiver-type-required |
| `real_public_image_attachment` | `handle.begin_drain` | [1237](../../src/process_live_tests.rs#L1237) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap().unwrap` | [1238](../../src/process_live_tests.rs#L1238) | receiver-type-required |
| `real_public_image_attachment` | `runtime.block_on(serve).unwrap` | [1238](../../src/process_live_tests.rs#L1238) | receiver-type-required |
| `real_public_image_attachment` | `std::panic::resume_unwind` | [1239](../../src/process_live_tests.rs#L1239) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `fs::read_to_string(&path).unwrap().lines().map` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str(line).unwrap` | [1240](../../src/process_live_tests.rs#L1240) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::from_str` | [1240](../../src/process_live_tests.rs#L1240) | external-constructor-callback-or-unresolved |
| `real_public_image_attachment` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1241](../../src/process_live_tests.rs#L1241) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().filter` | [1241](../../src/process_live_tests.rs#L1241) | receiver-type-required |
| `real_public_image_attachment` | `events.iter` | [1241](../../src/process_live_tests.rs#L1241), [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find(&#124;e&#124; e["kind"]=="input").expect` | [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `events.iter().find` | [1244](../../src/process_live_tests.rs#L1244) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find(&#124;b&#124; b["type"]=="image").expect` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter().find` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap().iter` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array().unwrap` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `input["content"].as_array` | [1245](../../src/process_live_tests.rs#L1245) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAS0lEQVR42u3PQQkAAAgAsetfWiP4FgYrsKZeS0BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEDgsqnc8OJg6Ln3AAAAAElFTkSuQmCC").unwrap` | [1249](../../src/process_live_tests.rs#L1249) | receiver-type-required |
| `real_public_image_attachment` | `base64::engine::general_purpose::STANDARD.decode` | [1249](../../src/process_live_tests.rs#L1249) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str().unwrap` | [1250](../../src/process_live_tests.rs#L1250) | receiver-type-required |
| `real_public_image_attachment` | `image["asset"].as_str` | [1250](../../src/process_live_tests.rs#L1250) | receiver-type-required |
| `real_public_image_attachment` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap()).unwrap` | [1253](../../src/process_live_tests.rs#L1253) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles,"asset":asset,"bytes":expected.len(),"provider":provider["id"],"model":provider["models"][0]["id"]})).unwrap` | [1253](../../src/process_live_tests.rs#L1253) | receiver-type-required |
| `real_public_image_attachment` | `serde_json::to_vec_pretty` | [1253](../../src/process_live_tests.rs#L1253) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap()).unwrap` | [1260](../../src/process_live_tests.rs#L1260) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice` | [1260](../../src/process_live_tests.rs#L1260), [1265](../../src/process_live_tests.rs#L1265) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_FLOW_CASE").unwrap()).unwrap` | [1260](../../src/process_live_tests.rs#L1260) | receiver-type-required |
| `real_public_flow_case` | `fs::read` | [1260](../../src/process_live_tests.rs#L1260), [1265](../../src/process_live_tests.rs#L1265), [1402](../../src/process_live_tests.rs#L1402), [1416](../../src/process_live_tests.rs#L1416) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_FLOW_CASE").unwrap` | [1260](../../src/process_live_tests.rs#L1260) | receiver-type-required |
| `real_public_flow_case` | `std::env::var` | [1260](../../src/process_live_tests.rs#L1260), [1261](../../src/process_live_tests.rs#L1261), [1265](../../src/process_live_tests.rs#L1265), [1306](../../src/process_live_tests.rs#L1306), [1307](../../src/process_live_tests.rs#L1307), [1349](../../src/process_live_tests.rs#L1349), [1350](../../src/process_live_tests.rs#L1350) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `PathBuf::from` | [1261](../../src/process_live_tests.rs#L1261) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1261](../../src/process_live_tests.rs#L1261) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap().join` | [1262](../../src/process_live_tests.rs#L1262) | receiver-type-required |
| `real_public_flow_case` | `root.parent().unwrap` | [1262](../../src/process_live_tests.rs#L1262) | receiver-type-required |
| `real_public_flow_case` | `root.parent` | [1262](../../src/process_live_tests.rs#L1262) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(&workspace).unwrap` | [1263](../../src/process_live_tests.rs#L1263) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all` | [1263](../../src/process_live_tests.rs#L1263), [1264](../../src/process_live_tests.rs#L1264), [1277](../../src/process_live_tests.rs#L1277), [1286](../../src/process_live_tests.rs#L1286), [1294](../../src/process_live_tests.rs#L1294), [1301](../../src/process_live_tests.rs#L1301), [1344](../../src/process_live_tests.rs#L1344) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::create_dir_all(root.join("config")).unwrap` | [1264](../../src/process_live_tests.rs#L1264) | receiver-type-required |
| `real_public_flow_case` | `root.join` | [1264](../../src/process_live_tests.rs#L1264), [1266](../../src/process_live_tests.rs#L1266), [1285](../../src/process_live_tests.rs#L1285), [1294](../../src/process_live_tests.rs#L1294), [1295](../../src/process_live_tests.rs#L1295), [1301](../../src/process_live_tests.rs#L1301), [1302](../../src/process_live_tests.rs#L1302), [1307](../../src/process_live_tests.rs#L1307), [1313](../../src/process_live_tests.rs#L1313), [1321](../../src/process_live_tests.rs#L1321), [1325](../../src/process_live_tests.rs#L1325), [1344](../../src/process_live_tests.rs#L1344), [1345](../../src/process_live_tests.rs#L1345), [1350](../../src/process_live_tests.rs#L1350), [1356](../../src/process_live_tests.rs#L1356), [1364](../../src/process_live_tests.rs#L1364), [1368](../../src/process_live_tests.rs#L1368), [1404](../../src/process_live_tests.rs#L1404), [1422](../../src/process_live_tests.rs#L1422), [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1265](../../src/process_live_tests.rs#L1265) | receiver-type-required |
| `real_public_flow_case` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1265](../../src/process_live_tests.rs#L1265) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1265](../../src/process_live_tests.rs#L1265) | receiver-type-required |
| `real_public_flow_case` | `write_canonical_test_json` | [1266](../../src/process_live_tests.rs#L1266), [1290](../../src/process_live_tests.rs#L1290), [1300](../../src/process_live_tests.rs#L1300), [1343](../../src/process_live_tests.rs#L1343) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["max_wall_seconds"].as_u64` | [1271](../../src/process_live_tests.rs#L1271) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect("policy object").insert` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut().expect` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_flow_case` | `policy.as_object_mut` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_flow_case` | `"max_wall_seconds".to_owned` | [1272](../../src/process_live_tests.rs#L1272) | receiver-type-required |
| `real_public_flow_case` | `write_test_execution_config` | [1274](../../src/process_live_tests.rs#L1274) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["seed_files"].as_object().unwrap` | [1275](../../src/process_live_tests.rs#L1275) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"].as_object` | [1275](../../src/process_live_tests.rs#L1275) | receiver-type-required |
| `real_public_flow_case` | `workspace.join` | [1276](../../src/process_live_tests.rs#L1276) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(target.parent().unwrap()).unwrap` | [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `target.parent().unwrap` | [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `target.parent` | [1277](../../src/process_live_tests.rs#L1277) | receiver-type-required |
| `real_public_flow_case` | `fs::write(target, content.as_str().unwrap()).unwrap` | [1278](../../src/process_live_tests.rs#L1278) | receiver-type-required |
| `real_public_flow_case` | `fs::write` | [1278](../../src/process_live_tests.rs#L1278), [1295](../../src/process_live_tests.rs#L1295), [1302](../../src/process_live_tests.rs#L1302), [1321](../../src/process_live_tests.rs#L1321), [1345](../../src/process_live_tests.rs#L1345), [1364](../../src/process_live_tests.rs#L1364), [1404](../../src/process_live_tests.rs#L1404), [1422](../../src/process_live_tests.rs#L1422), [1427](../../src/process_live_tests.rs#L1427) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `content.as_str().unwrap` | [1278](../../src/process_live_tests.rs#L1278) | receiver-type-required |
| `real_public_flow_case` | `content.as_str` | [1278](../../src/process_live_tests.rs#L1278) | receiver-type-required |
| `real_public_flow_case` | `register_local_tools_server` | [1282](../../src/process_live_tests.rs#L1282) | [tekes-supervisor::process_live_tests::register_local_tools_server](../../src/process_live_tests.rs#L1066) |
| `real_public_flow_case` | `root.join("threads").join` | [1285](../../src/process_live_tests.rs#L1285) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1286](../../src/process_live_tests.rs#L1286) | receiver-type-required |
| `real_public_flow_case` | `folder.join` | [1286](../../src/process_live_tests.rs#L1286), [1287](../../src/process_live_tests.rs#L1287), [1290](../../src/process_live_tests.rs#L1290), [1300](../../src/process_live_tests.rs#L1300), [1343](../../src/process_live_tests.rs#L1343) | receiver-type-required |
| `real_public_flow_case` | `write_test_genesis_with_identity` | [1288](../../src/process_live_tests.rs#L1288) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["identity_profile"].as_str` | [1288](../../src/process_live_tests.rs#L1288) | receiver-type-required |
| `real_public_flow_case` | `fs::create_dir_all(root.join(".agent/commands")).unwrap` | [1294](../../src/process_live_tests.rs#L1294), [1301](../../src/process_live_tests.rs#L1301), [1344](../../src/process_live_tests.rs#L1344) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join(".agent/commands/compact.md"), case["seed_files"][".agent/commands/compact.md"].as_str().unwrap()).unwrap` | [1295](../../src/process_live_tests.rs#L1295), [1302](../../src/process_live_tests.rs#L1302), [1345](../../src/process_live_tests.rs#L1345) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str().unwrap` | [1295](../../src/process_live_tests.rs#L1295), [1302](../../src/process_live_tests.rs#L1302), [1345](../../src/process_live_tests.rs#L1345) | receiver-type-required |
| `real_public_flow_case` | `case["seed_files"][".agent/commands/compact.md"].as_str` | [1295](../../src/process_live_tests.rs#L1295), [1302](../../src/process_live_tests.rs#L1302), [1345](../../src/process_live_tests.rs#L1345) | receiver-type-required |
| `real_public_flow_case` | `Arc::new` | [1305](../../src/process_live_tests.rs#L1305), [1313](../../src/process_live_tests.rs#L1313), [1348](../../src/process_live_tests.rs#L1348), [1356](../../src/process_live_tests.rs#L1356) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `provider::MemorySecretStore::new` | [1305](../../src/process_live_tests.rs#L1305), [1348](../../src/process_live_tests.rs#L1348) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_flow_case` | `secrets.publish(provider["credential_key"].as_str().unwrap(),provider::SecretRecord::Active {generation:1,material:std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap()}).unwrap` | [1306](../../src/process_live_tests.rs#L1306), [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `secrets.publish` | [1306](../../src/process_live_tests.rs#L1306), [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str().unwrap` | [1306](../../src/process_live_tests.rs#L1306), [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `provider["credential_key"].as_str` | [1306](../../src/process_live_tests.rs#L1306), [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1306](../../src/process_live_tests.rs#L1306), [1349](../../src/process_live_tests.rs#L1349) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [1307](../../src/process_live_tests.rs#L1307), [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store(&root,std::env::var("TEKES_TEST_REAL_WORKER").unwrap(),"public-error-queue",root.join(".agent"),secrets).inspect` | [1307](../../src/process_live_tests.rs#L1307), [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `ProductionProcessHost::open_with_secret_store` | [1307](../../src/process_live_tests.rs#L1307), [1350](../../src/process_live_tests.rs#L1350) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1307](../../src/process_live_tests.rs#L1307), [1350](../../src/process_live_tests.rs#L1350) | receiver-type-required |
| `real_public_flow_case` | `attach_live_stdio` | [1307](../../src/process_live_tests.rs#L1307), [1350](../../src/process_live_tests.rs#L1350) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_public_flow_case` | `tokio::runtime::Runtime::new().unwrap` | [1308](../../src/process_live_tests.rs#L1308), [1351](../../src/process_live_tests.rs#L1351) | receiver-type-required |
| `real_public_flow_case` | `tokio::runtime::Runtime::new` | [1308](../../src/process_live_tests.rs#L1308), [1351](../../src/process_live_tests.rs#L1351) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `runtime.block_on(tokio::net::TcpListener::bind("127.0.0.1:0")).unwrap` | [1309](../../src/process_live_tests.rs#L1309), [1352](../../src/process_live_tests.rs#L1352) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on` | [1309](../../src/process_live_tests.rs#L1309), [1333](../../src/process_live_tests.rs#L1333), [1352](../../src/process_live_tests.rs#L1352), [1376](../../src/process_live_tests.rs#L1376) | receiver-type-required |
| `real_public_flow_case` | `tokio::net::TcpListener::bind` | [1309](../../src/process_live_tests.rs#L1309), [1352](../../src/process_live_tests.rs#L1352) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `listener.local_addr().unwrap` | [1310](../../src/process_live_tests.rs#L1310), [1353](../../src/process_live_tests.rs#L1353) | receiver-type-required |
| `real_public_flow_case` | `listener.local_addr` | [1310](../../src/process_live_tests.rs#L1310), [1353](../../src/process_live_tests.rs#L1353) | receiver-type-required |
| `real_public_flow_case` | `crate::host_runtime::assemble_application_endpoint_host(&root,         endpoint::SessionHostDescription {version:"test".into(),cwd:workspace.to_string_lossy().into_owned(),provider:None,model:None,attached_sessions:0,home:root.to_string_lossy().into_owned(),can_open_path:false},         Arc::new(&#124;&#124; Ok("2026-09-04T10:00:00.000Z".into())), &root.join(".agent"),host.clone()).unwrap` | [1311](../../src/process_live_tests.rs#L1311), [1354](../../src/process_live_tests.rs#L1354) | receiver-type-required |
| `real_public_flow_case` | `crate::host_runtime::assemble_application_endpoint_host` | [1311](../../src/process_live_tests.rs#L1311), [1354](../../src/process_live_tests.rs#L1354) | [tekes-supervisor::host_runtime::assemble_application_endpoint_host](../../src/host_runtime.rs#L283) |
| `real_public_flow_case` | `"test".into` | [1312](../../src/process_live_tests.rs#L1312), [1355](../../src/process_live_tests.rs#L1355) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy().into_owned` | [1312](../../src/process_live_tests.rs#L1312), [1355](../../src/process_live_tests.rs#L1355) | receiver-type-required |
| `real_public_flow_case` | `workspace.to_string_lossy` | [1312](../../src/process_live_tests.rs#L1312), [1355](../../src/process_live_tests.rs#L1355) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy().into_owned` | [1312](../../src/process_live_tests.rs#L1312), [1355](../../src/process_live_tests.rs#L1355) | receiver-type-required |
| `real_public_flow_case` | `root.to_string_lossy` | [1312](../../src/process_live_tests.rs#L1312), [1355](../../src/process_live_tests.rs#L1355) | receiver-type-required |
| `real_public_flow_case` | `Ok` | [1313](../../src/process_live_tests.rs#L1313), [1356](../../src/process_live_tests.rs#L1356) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `"2026-09-04T10:00:00.000Z".into` | [1313](../../src/process_live_tests.rs#L1313), [1356](../../src/process_live_tests.rs#L1356) | receiver-type-required |
| `real_public_flow_case` | `host.clone` | [1313](../../src/process_live_tests.rs#L1313), [1314](../../src/process_live_tests.rs#L1314), [1356](../../src/process_live_tests.rs#L1356), [1357](../../src/process_live_tests.rs#L1357) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble(&root,unary,host.clone(),         transport::TransportConfig::loopback(address,transport::BearerToken::new([42;32]))).unwrap` | [1314](../../src/process_live_tests.rs#L1314), [1357](../../src/process_live_tests.rs#L1357) | receiver-type-required |
| `real_public_flow_case` | `crate::endpoint_carrier::ProductionCarrierAssembly::assemble` | [1314](../../src/process_live_tests.rs#L1314), [1357](../../src/process_live_tests.rs#L1357) | [tekes-supervisor::endpoint_carrier::ProductionCarrierAssembly::assemble](../../src/endpoint_carrier.rs#L2789) |
| `real_public_flow_case` | `transport::TransportConfig::loopback` | [1315](../../src/process_live_tests.rs#L1315), [1358](../../src/process_live_tests.rs#L1358) | [transport::server::TransportConfig::loopback](../../../transport/src/server.rs#L108) |
| `real_public_flow_case` | `transport::BearerToken::new` | [1315](../../src/process_live_tests.rs#L1315), [1358](../../src/process_live_tests.rs#L1358) | [transport::auth::BearerToken::new](../../../transport/src/auth.rs#L36) |
| `real_public_flow_case` | `host.attach_streams` | [1316](../../src/process_live_tests.rs#L1316), [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams().clone` | [1316](../../src/process_live_tests.rs#L1316), [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_flow_case` | `assembly.streams` | [1316](../../src/process_live_tests.rs#L1316), [1359](../../src/process_live_tests.rs#L1359) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery().unwrap` | [1317](../../src/process_live_tests.rs#L1317), [1360](../../src/process_live_tests.rs#L1360) | receiver-type-required |
| `real_public_flow_case` | `assembly.finish_recovery` | [1317](../../src/process_live_tests.rs#L1317), [1360](../../src/process_live_tests.rs#L1360) | receiver-type-required |
| `real_public_flow_case` | `assembly.into_server` | [1318](../../src/process_live_tests.rs#L1318), [1361](../../src/process_live_tests.rs#L1361) | receiver-type-required |
| `real_public_flow_case` | `server.handle` | [1319](../../src/process_live_tests.rs#L1319), [1362](../../src/process_live_tests.rs#L1362) | receiver-type-required |
| `real_public_flow_case` | `runtime.spawn` | [1320](../../src/process_live_tests.rs#L1320), [1363](../../src/process_live_tests.rs#L1363) | receiver-type-required |
| `real_public_flow_case` | `server.serve` | [1320](../../src/process_live_tests.rs#L1320), [1363](../../src/process_live_tests.rs#L1363) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("client-endpoint.json"),serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap()).unwrap` | [1321](../../src/process_live_tests.rs#L1321), [1364](../../src/process_live_tests.rs#L1364) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec(&json!({"address":address.to_string(),"session":session})).unwrap` | [1321](../../src/process_live_tests.rs#L1321), [1364](../../src/process_live_tests.rs#L1364) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec` | [1321](../../src/process_live_tests.rs#L1321), [1364](../../src/process_live_tests.rs#L1364) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.start_periodic_sweep` | [1322](../../src/process_live_tests.rs#L1322), [1365](../../src/process_live_tests.rs#L1365) | receiver-type-required |
| `real_public_flow_case` | `std::panic::catch_unwind` | [1323](../../src/process_live_tests.rs#L1323), [1366](../../src/process_live_tests.rs#L1366) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `std::panic::AssertUnwindSafe` | [1323](../../src/process_live_tests.rs#L1323), [1366](../../src/process_live_tests.rs#L1366) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Instant::now` | [1324](../../src/process_live_tests.rs#L1324), [1367](../../src/process_live_tests.rs#L1367) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_secs` | [1324](../../src/process_live_tests.rs#L1324), [1367](../../src/process_live_tests.rs#L1367) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64().unwrap_or` | [1324](../../src/process_live_tests.rs#L1324), [1367](../../src/process_live_tests.rs#L1367) | receiver-type-required |
| `real_public_flow_case` | `case["timeout_seconds"].as_u64` | [1324](../../src/process_live_tests.rs#L1324), [1367](../../src/process_live_tests.rs#L1367) | receiver-type-required |
| `real_public_flow_case` | `root.join("client-flow-receipt.json").exists` | [1325](../../src/process_live_tests.rs#L1325), [1368](../../src/process_live_tests.rs#L1368) | receiver-type-required |
| `real_public_flow_case` | `std::thread::sleep` | [1328](../../src/process_live_tests.rs#L1328), [1371](../../src/process_live_tests.rs#L1371) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `Duration::from_millis` | [1328](../../src/process_live_tests.rs#L1328), [1371](../../src/process_live_tests.rs#L1371) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `host.shutdown` | [1331](../../src/process_live_tests.rs#L1331), [1374](../../src/process_live_tests.rs#L1374) | receiver-type-required |
| `real_public_flow_case` | `handle.begin_drain` | [1332](../../src/process_live_tests.rs#L1332), [1375](../../src/process_live_tests.rs#L1375) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap().unwrap` | [1333](../../src/process_live_tests.rs#L1333), [1376](../../src/process_live_tests.rs#L1376) | receiver-type-required |
| `real_public_flow_case` | `runtime.block_on(serve).unwrap` | [1333](../../src/process_live_tests.rs#L1333), [1376](../../src/process_live_tests.rs#L1376) | receiver-type-required |
| `real_public_flow_case` | `std::panic::resume_unwind` | [1334](../../src/process_live_tests.rs#L1334), [1377](../../src/process_live_tests.rs#L1377) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map(&#124;line&#124;serde_json::from_str(line).unwrap()).collect` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines().map` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap().lines` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string(&path).unwrap` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `fs::read_to_string` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `serde_json::from_str(line).unwrap` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | receiver-type-required |
| `real_public_flow_case` | `serde_json::from_str` | [1335](../../src/process_live_tests.rs#L1335), [1378](../../src/process_live_tests.rs#L1378) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124;e["kind"]=="settle").collect` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379), [1390](../../src/process_live_tests.rs#L1390), [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `events.iter` | [1336](../../src/process_live_tests.rs#L1336), [1379](../../src/process_live_tests.rs#L1379), [1388](../../src/process_live_tests.rs#L1388), [1390](../../src/process_live_tests.rs#L1390), [1410](../../src/process_live_tests.rs#L1410), [1413](../../src/process_live_tests.rs#L1413), [1415](../../src/process_live_tests.rs#L1415), [1418](../../src/process_live_tests.rs#L1418), [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="compact" && e["origin_tuple"]["op"]=="commands/run").expect` | [1388](../../src/process_live_tests.rs#L1388), [1410](../../src/process_live_tests.rs#L1410) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find` | [1388](../../src/process_live_tests.rs#L1388), [1410](../../src/process_live_tests.rs#L1410), [1413](../../src/process_live_tests.rs#L1413), [1415](../../src/process_live_tests.rs#L1415), [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap().len` | [1389](../../src/process_live_tests.rs#L1389) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array().unwrap` | [1389](../../src/process_live_tests.rs#L1389) | receiver-type-required |
| `real_public_flow_case` | `case["seed_prompts"].as_array` | [1389](../../src/process_live_tests.rs#L1389) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="settle" && e["seq"].as_u64() < compact["seq"].as_u64()).count` | [1390](../../src/process_live_tests.rs#L1390) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64` | [1390](../../src/process_live_tests.rs#L1390), [1413](../../src/process_live_tests.rs#L1413), [1415](../../src/process_live_tests.rs#L1415), [1418](../../src/process_live_tests.rs#L1418), [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `compact["seq"].as_u64` | [1390](../../src/process_live_tests.rs#L1390), [1411](../../src/process_live_tests.rs#L1411) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array().expect` | [1396](../../src/process_live_tests.rs#L1396) | receiver-type-required |
| `real_public_flow_case` | `record["evidence_refs"].as_array` | [1396](../../src/process_live_tests.rs#L1396) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map(&#124;r&#124; r["from"].as_u64().unwrap()..=r["to"].as_u64().unwrap()).collect` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter().flat_map` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap().iter` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array().unwrap` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `compact["covers"].as_array` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64().unwrap` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `r["from"].as_u64` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64().unwrap` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `r["to"].as_u64` | [1398](../../src/process_live_tests.rs#L1398) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str().map` | [1400](../../src/process_live_tests.rs#L1400) | receiver-type-required |
| `real_public_flow_case` | `compact["summary"].as_str` | [1400](../../src/process_live_tests.rs#L1400) | receiver-type-required |
| `real_public_flow_case` | `summary.starts_with` | [1400](../../src/process_live_tests.rs#L1400) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger(&fs::read(&path).unwrap(), schema::READER_VERSION).unwrap` | [1402](../../src/process_live_tests.rs#L1402), [1416](../../src/process_live_tests.rs#L1416) | receiver-type-required |
| `real_public_flow_case` | `schema::validate_ledger` | [1402](../../src/process_live_tests.rs#L1402), [1416](../../src/process_live_tests.rs#L1416) | [schema::fold::validate_ledger](../../../schema/src/fold.rs#L1053) |
| `real_public_flow_case` | `fs::read(&path).unwrap` | [1402](../../src/process_live_tests.rs#L1402), [1416](../../src/process_live_tests.rs#L1416) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compaction-summary.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap()).unwrap` | [1404](../../src/process_live_tests.rs#L1404) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact["seq"],"covers":compact["covers"],"summary_request":record,"summary_is_model":summary_is_model,"summary_bytes":compact["summary"].as_str().map(str::len)})).unwrap` | [1404](../../src/process_live_tests.rs#L1404) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty` | [1404](../../src/process_live_tests.rs#L1404), [1422](../../src/process_live_tests.rs#L1422), [1427](../../src/process_live_tests.rs#L1427) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `compact["seq"].as_u64().unwrap` | [1411](../../src/process_live_tests.rs#L1411) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap().to_owned` | [1412](../../src/process_live_tests.rs#L1412) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str().unwrap` | [1412](../../src/process_live_tests.rs#L1412) | receiver-type-required |
| `real_public_flow_case` | `compact["origin_key"].as_str` | [1412](../../src/process_live_tests.rs#L1412) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="epoch" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1413](../../src/process_live_tests.rs#L1413) | receiver-type-required |
| `real_public_flow_case` | `e["seq"].as_u64().unwrap` | [1413](../../src/process_live_tests.rs#L1413), [1415](../../src/process_live_tests.rs#L1415), [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64().unwrap` | [1415](../../src/process_live_tests.rs#L1415) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect("post-compact input")["seq"].as_u64` | [1415](../../src/process_live_tests.rs#L1415) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["kind"]=="input" && e["seq"].as_u64().unwrap() > compact_seq).expect` | [1415](../../src/process_live_tests.rs#L1415) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt(&typed.events, &origin_key, &[post_input]).expect` | [1417](../../src/process_live_tests.rs#L1417) | receiver-type-required |
| `real_public_flow_case` | `engine::first_post_compact_attempt` | [1417](../../src/process_live_tests.rs#L1417) | [engine::compact_gate::first_post_compact_attempt](../../../engine/src/compact_gate.rs#L20) |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap().to_owned` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str().unwrap` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap()["attempt"].as_str` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `events.iter().find(&#124;e&#124; e["seq"].as_u64()==Some(gate)).unwrap` | [1418](../../src/process_live_tests.rs#L1418) | receiver-type-required |
| `real_public_flow_case` | `Some` | [1418](../../src/process_live_tests.rs#L1418) | external-constructor-callback-or-unresolved |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map(&#124;e&#124; e["call"].as_str().map(str::to_owned)).collect` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `events.iter().filter(&#124;e&#124; e["kind"]=="tool_call" && e["seq"].as_u64().unwrap() < compact_seq).filter_map` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str().map` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `e["call"].as_str` | [1419](../../src/process_live_tests.rs#L1419) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("compact-gate.json"), serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap()).unwrap` | [1422](../../src/process_live_tests.rs#L1422) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"compact_seq":compact_seq,"origin_key":origin_key,"covers":compact["covers"],"epoch_seq":epoch["seq"],"post_input_seq":post_input,"first_post_compact_attempt_seq":gate,"first_post_compact_attempt":attempt,"pre_compact_call_ids":pre_calls})).unwrap` | [1422](../../src/process_live_tests.rs#L1422) | receiver-type-required |
| `real_public_flow_case` | `fs::write(root.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap()).unwrap` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_flow_case` | `serde_json::to_vec_pretty(&json!({"status":"passed","settles":settles})).unwrap` | [1427](../../src/process_live_tests.rs#L1427) | receiver-type-required |
| `real_public_mcp_task_continuation` | `PathBuf::from` | [1440](../../src/process_live_tests.rs#L1440) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_PROCESS_ARTIFACT").unwrap` | [1440](../../src/process_live_tests.rs#L1440) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var` | [1440](../../src/process_live_tests.rs#L1440), [1445](../../src/process_live_tests.rs#L1445), [1459](../../src/process_live_tests.rs#L1459), [1460](../../src/process_live_tests.rs#L1460) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&root).unwrap` | [1441](../../src/process_live_tests.rs#L1441) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all` | [1441](../../src/process_live_tests.rs#L1441), [1443](../../src/process_live_tests.rs#L1443), [1444](../../src/process_live_tests.rs#L1444), [1451](../../src/process_live_tests.rs#L1451) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `root.parent().unwrap().join` | [1442](../../src/process_live_tests.rs#L1442) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent().unwrap` | [1442](../../src/process_live_tests.rs#L1442) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.parent` | [1442](../../src/process_live_tests.rs#L1442) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(&workspace).unwrap` | [1443](../../src/process_live_tests.rs#L1443) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(root.join("config")).unwrap` | [1444](../../src/process_live_tests.rs#L1444) | receiver-type-required |
| `real_public_mcp_task_continuation` | `root.join` | [1444](../../src/process_live_tests.rs#L1444), [1446](../../src/process_live_tests.rs#L1446), [1450](../../src/process_live_tests.rs#L1450), [1460](../../src/process_live_tests.rs#L1460), [1500](../../src/process_live_tests.rs#L1500) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice(&fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap()).unwrap` | [1445](../../src/process_live_tests.rs#L1445) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_slice` | [1445](../../src/process_live_tests.rs#L1445) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read(std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap()).unwrap` | [1445](../../src/process_live_tests.rs#L1445) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read` | [1445](../../src/process_live_tests.rs#L1445) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").unwrap` | [1445](../../src/process_live_tests.rs#L1445) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_canonical_test_json` | [1446](../../src/process_live_tests.rs#L1446) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `write_test_execution_config` | [1447](../../src/process_live_tests.rs#L1447) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `stage_live_stdio` | [1448](../../src/process_live_tests.rs#L1448) | [tekes-supervisor::process_live_tests::stage_live_stdio](../../src/process_live_tests.rs#L1504) |
| `real_public_mcp_task_continuation` | `root.join("threads").join` | [1450](../../src/process_live_tests.rs#L1450) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::create_dir_all(folder.join("assets")).unwrap` | [1451](../../src/process_live_tests.rs#L1451) | receiver-type-required |
| `real_public_mcp_task_continuation` | `folder.join` | [1451](../../src/process_live_tests.rs#L1451), [1452](../../src/process_live_tests.rs#L1452) | receiver-type-required |
| `real_public_mcp_task_continuation` | `write_test_genesis` | [1453](../../src/process_live_tests.rs#L1453) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `append_test_input` | [1454](../../src/process_live_tests.rs#L1454) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map(&#124;s&#124; serde_json::from_str(s).unwrap()).collect` | [1455](../../src/process_live_tests.rs#L1455), [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().map` | [1455](../../src/process_live_tests.rs#L1455), [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines` | [1455](../../src/process_live_tests.rs#L1455), [1466](../../src/process_live_tests.rs#L1466), [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap` | [1455](../../src/process_live_tests.rs#L1455), [1466](../../src/process_live_tests.rs#L1466), [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string` | [1455](../../src/process_live_tests.rs#L1455), [1466](../../src/process_live_tests.rs#L1466), [1474](../../src/process_live_tests.rs#L1474) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).unwrap` | [1455](../../src/process_live_tests.rs#L1455), [1474](../../src/process_live_tests.rs#L1474) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str` | [1455](../../src/process_live_tests.rs#L1455), [1466](../../src/process_live_tests.rs#L1466), [1474](../../src/process_live_tests.rs#L1474) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `write_test_events` | [1457](../../src/process_live_tests.rs#L1457) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Arc::new` | [1458](../../src/process_live_tests.rs#L1458) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `provider::MemorySecretStore::new` | [1458](../../src/process_live_tests.rs#L1458) | [provider::secret_store::MemorySecretStore::new](../../../provider/src/secret_store.rs#L179) |
| `real_public_mcp_task_continuation` | `secrets.publish(provider["credential_key"].as_str().unwrap(), provider::SecretRecord::Active { generation: 1, material: std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap() }).unwrap` | [1459](../../src/process_live_tests.rs#L1459) | receiver-type-required |
| `real_public_mcp_task_continuation` | `secrets.publish` | [1459](../../src/process_live_tests.rs#L1459) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str().unwrap` | [1459](../../src/process_live_tests.rs#L1459) | receiver-type-required |
| `real_public_mcp_task_continuation` | `provider["credential_key"].as_str` | [1459](../../src/process_live_tests.rs#L1459) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_KERNEL_LIVE_KEY").unwrap` | [1459](../../src/process_live_tests.rs#L1459) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store(&root, std::env::var("TEKES_TEST_REAL_WORKER").unwrap(), "live-mcp-task", root.join(".agent"), secrets).inspect(&#124;host&#124; { attach_live_stdio(host,&root); }).unwrap` | [1460](../../src/process_live_tests.rs#L1460) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store(&root, std::env::var("TEKES_TEST_REAL_WORKER").unwrap(), "live-mcp-task", root.join(".agent"), secrets).inspect` | [1460](../../src/process_live_tests.rs#L1460) | receiver-type-required |
| `real_public_mcp_task_continuation` | `ProductionProcessHost::open_with_secret_store` | [1460](../../src/process_live_tests.rs#L1460) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::env::var("TEKES_TEST_REAL_WORKER").unwrap` | [1460](../../src/process_live_tests.rs#L1460) | receiver-type-required |
| `real_public_mcp_task_continuation` | `attach_live_stdio` | [1460](../../src/process_live_tests.rs#L1460) | [tekes-supervisor::process_live_tests::attach_live_stdio](../../src/process_live_tests.rs#L1509) |
| `real_public_mcp_task_continuation` | `std::panic::catch_unwind` | [1461](../../src/process_live_tests.rs#L1461) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `std::panic::AssertUnwindSafe` | [1461](../../src/process_live_tests.rs#L1461) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap().expect` | [1462](../../src/process_live_tests.rs#L1462) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main(session).unwrap` | [1462](../../src/process_live_tests.rs#L1462) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.schedule_main` | [1462](../../src/process_live_tests.rs#L1462) | receiver-type-required |
| `real_public_mcp_task_continuation` | `host.start_periodic_sweep` | [1463](../../src/process_live_tests.rs#L1463) | receiver-type-required |
| `real_public_mcp_task_continuation` | `Instant::now` | [1464](../../src/process_live_tests.rs#L1464) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_secs` | [1464](../../src/process_live_tests.rs#L1464) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map(&#124;s&#124; serde_json::from_str(s).ok()).collect` | [1466](../../src/process_live_tests.rs#L1466) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::read_to_string(&path).unwrap().lines().filter_map` | [1466](../../src/process_live_tests.rs#L1466) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::from_str(s).ok` | [1466](../../src/process_live_tests.rs#L1466) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().any` | [1467](../../src/process_live_tests.rs#L1467) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter` | [1467](../../src/process_live_tests.rs#L1467), [1475](../../src/process_live_tests.rs#L1475), [1477](../../src/process_live_tests.rs#L1477), [1480](../../src/process_live_tests.rs#L1480), [1487](../../src/process_live_tests.rs#L1487), [1491](../../src/process_live_tests.rs#L1491), [1497](../../src/process_live_tests.rs#L1497) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::thread::sleep` | [1469](../../src/process_live_tests.rs#L1469) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `Duration::from_millis` | [1469](../../src/process_live_tests.rs#L1469) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `host.shutdown` | [1472](../../src/process_live_tests.rs#L1472) | receiver-type-required |
| `real_public_mcp_task_continuation` | `std::panic::resume_unwind` | [1473](../../src/process_live_tests.rs#L1473) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `events.iter().find(&#124;e&#124; e["kind"] == "settle").unwrap` | [1475](../../src/process_live_tests.rs#L1475) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().find` | [1475](../../src/process_live_tests.rs#L1475) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_call" && e["name"] == "mcp__fixture__echo").collect` | [1477](../../src/process_live_tests.rs#L1477) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter` | [1477](../../src/process_live_tests.rs#L1477), [1480](../../src/process_live_tests.rs#L1480), [1487](../../src/process_live_tests.rs#L1487), [1491](../../src/process_live_tests.rs#L1491) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str().unwrap` | [1479](../../src/process_live_tests.rs#L1479) | receiver-type-required |
| `real_public_mcp_task_continuation` | `calls[0]["call"].as_str` | [1479](../../src/process_live_tests.rs#L1479) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "state" && e["subkind"] == "tool_continuation" && e["payload"]["call"] == call).collect` | [1480](../../src/process_live_tests.rs#L1480) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map(&#124;e&#124; e["payload"]["action"].as_str().unwrap()).collect` | [1481](../../src/process_live_tests.rs#L1481) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().map` | [1481](../../src/process_live_tests.rs#L1481) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter` | [1481](../../src/process_live_tests.rs#L1481), [1484](../../src/process_live_tests.rs#L1484) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str().unwrap` | [1481](../../src/process_live_tests.rs#L1481) | receiver-type-required |
| `real_public_mcp_task_continuation` | `e["payload"]["action"].as_str` | [1481](../../src/process_live_tests.rs#L1481) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find(&#124;e&#124; e["payload"]["action"] == "park").unwrap` | [1484](../../src/process_live_tests.rs#L1484) | receiver-type-required |
| `real_public_mcp_task_continuation` | `steps.iter().find` | [1484](../../src/process_live_tests.rs#L1484) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "run_start").collect` | [1487](../../src/process_live_tests.rs#L1487) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64().unwrap` | [1489](../../src/process_live_tests.rs#L1489) | receiver-type-required |
| `real_public_mcp_task_continuation` | `park["seq"].as_u64` | [1489](../../src/process_live_tests.rs#L1489) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().filter(&#124;e&#124; e["kind"] == "tool_result" && e["call"] == call).collect` | [1491](../../src/process_live_tests.rs#L1491) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str().unwrap` | [1494](../../src/process_live_tests.rs#L1494) | receiver-type-required |
| `real_public_mcp_task_continuation` | `results[0]["content"][0]["text"].as_str` | [1494](../../src/process_live_tests.rs#L1494) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find(&#124;e&#124; e["kind"] == "output").unwrap` | [1497](../../src/process_live_tests.rs#L1497) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev().find` | [1497](../../src/process_live_tests.rs#L1497) | receiver-type-required |
| `real_public_mcp_task_continuation` | `events.iter().rev` | [1497](../../src/process_live_tests.rs#L1497) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write(root.join("receipt.json"), serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap()).unwrap` | [1500](../../src/process_live_tests.rs#L1500) | receiver-type-required |
| `real_public_mcp_task_continuation` | `fs::write` | [1500](../../src/process_live_tests.rs#L1500) | external-constructor-callback-or-unresolved |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty(&json!({"status":"passed","call":call,"continuation_actions":actions,"runs":runs.len(),"result_seq":results[0]["seq"]})).unwrap` | [1500](../../src/process_live_tests.rs#L1500) | receiver-type-required |
| `real_public_mcp_task_continuation` | `serde_json::to_vec_pretty` | [1500](../../src/process_live_tests.rs#L1500) | external-constructor-callback-or-unresolved |
| `stage_live_stdio` | `std::env::var("TEKES_TEST_MCP_STDIO_SOCKET").expect` | [1505](../../src/process_live_tests.rs#L1505) | receiver-type-required |
| `stage_live_stdio` | `std::env::var` | [1505](../../src/process_live_tests.rs#L1505) | external-constructor-callback-or-unresolved |
| `stage_live_stdio` | `write_canonical_test_json` | [1506](../../src/process_live_tests.rs#L1506) | external-constructor-callback-or-unresolved |
| `stage_live_stdio` | `root.join` | [1506](../../src/process_live_tests.rs#L1506) | receiver-type-required |
| `attach_live_stdio` | `fs::read` | [1510](../../src/process_live_tests.rs#L1510) | external-constructor-callback-or-unresolved |
| `attach_live_stdio` | `root.join` | [1510](../../src/process_live_tests.rs#L1510) | receiver-type-required |
| `attach_live_stdio` | `serde_json::from_slice(&bytes).unwrap` | [1511](../../src/process_live_tests.rs#L1511) | receiver-type-required |
| `attach_live_stdio` | `serde_json::from_slice` | [1511](../../src/process_live_tests.rs#L1511) | external-constructor-callback-or-unresolved |
| `attach_live_stdio` | `host.mcp_runtime().set_stdio_connector` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `attach_live_stdio` | `host.mcp_runtime` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `attach_live_stdio` | `Arc::new` | [1512](../../src/process_live_tests.rs#L1512) | external-constructor-callback-or-unresolved |
| `attach_live_stdio` | `crate::stdio_channels::UnixStdioChannels` | [1512](../../src/process_live_tests.rs#L1512) | external-constructor-callback-or-unresolved |
| `attach_live_stdio` | `[("test".into(),PathBuf::from(value["socket"].as_str().unwrap()))].into` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `attach_live_stdio` | `"test".into` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `attach_live_stdio` | `PathBuf::from` | [1512](../../src/process_live_tests.rs#L1512) | external-constructor-callback-or-unresolved |
| `attach_live_stdio` | `value["socket"].as_str().unwrap` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
| `attach_live_stdio` | `value["socket"].as_str` | [1512](../../src/process_live_tests.rs#L1512) | receiver-type-required |
