# tekes-worker::live_tests

[Package atlas](index.md) · [Source](../../src/live_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::live_tests::LiveValueHook](../../src/live_tests.rs#L4) | type_item | `private` | test;  |
| [tekes-worker::live_tests::LiveBytesHook](../../src/live_tests.rs#L5) | type_item | `private` | test;  |
| [tekes-worker::live_tests::LiveControl](../../src/live_tests.rs#L7) | struct_item | `private` | test;  |
| [tekes-worker::live_tests::LiveControl::write](../../src/live_tests.rs#L10) | function_item | `private` | test;  |
| [tekes-worker::live_tests::LiveControl::flush](../../src/live_tests.rs#L14) | function_item | `private` | test;  |
| [tekes-worker::live_tests::Item](../../src/live_tests.rs#L18) | type_item | `private` | test;  |
| [tekes-worker::live_tests::LiveControl::next](../../src/live_tests.rs#L19) | function_item | `private` | test;  |
| [tekes-worker::live_tests::live_kernel_turn](../../src/live_tests.rs#L38) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects](../../src/live_tests.rs#L540) | struct_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::ensure_running](../../src/live_tests.rs#L542) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::deliver_input](../../src/live_tests.rs#L543) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::interrupt](../../src/live_tests.rs#L544) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::ensure_child](../../src/live_tests.rs#L545) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::deliver_report](../../src/live_tests.rs#L546) | function_item | `private` | test;  |
| [tekes-worker::live_tests::UnavailableLiveEffects::execute](../../src/live_tests.rs#L549) | function_item | `private` | test;  |
| [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) | function_item | `private` | test;  |
| [tekes-worker::live_tests::prepare_live_choice](../../src/live_tests.rs#L557) | function_item | `pub(super)` | test;  |

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
| `write` | `self.0.borrow_mut().extend_from_slice` | [11](../../src/live_tests.rs#L11) | receiver-type-required |
| `write` | `self.0.borrow_mut` | [11](../../src/live_tests.rs#L11) | receiver-type-required |
| `write` | `Ok` | [12](../../src/live_tests.rs#L12) | external-constructor-callback-or-unresolved |
| `write` | `bytes.len` | [12](../../src/live_tests.rs#L12) | receiver-type-required |
| `flush` | `Ok` | [14](../../src/live_tests.rs#L14) | external-constructor-callback-or-unresolved |
| `next` | `self.0.borrow().clone` | [20](../../src/live_tests.rs#L20) | receiver-type-required |
| `next` | `self.0.borrow` | [20](../../src/live_tests.rs#L20) | receiver-type-required |
| `next` | `bytes.split(&#124;byte&#124; *byte == b'\n').rev().find` | [21](../../src/live_tests.rs#L21) | receiver-type-required |
| `next` | `bytes.split(&#124;byte&#124; *byte == b'\n').rev` | [21](../../src/live_tests.rs#L21) | receiver-type-required |
| `next` | `bytes.split` | [21](../../src/live_tests.rs#L21) | receiver-type-required |
| `next` | `line.is_empty` | [21](../../src/live_tests.rs#L21) | receiver-type-required |
| `next` | `serde_json::from_slice(last).ok` | [22](../../src/live_tests.rs#L22) | receiver-type-required |
| `next` | `serde_json::from_slice` | [22](../../src/live_tests.rs#L22) | external-constructor-callback-or-unresolved |
| `next` | `message.get` | [23](../../src/live_tests.rs#L23), [28](../../src/live_tests.rs#L28), [31](../../src/live_tests.rs#L31) | receiver-type-required |
| `next` | `self.1.as_ref` | [24](../../src/live_tests.rs#L24) | receiver-type-required |
| `next` | `callback` | [25](../../src/live_tests.rs#L25) | external-constructor-callback-or-unresolved |
| `next` | `Some` | [25](../../src/live_tests.rs#L25), [26](../../src/live_tests.rs#L26), [29](../../src/live_tests.rs#L29), [32](../../src/live_tests.rs#L32) | external-constructor-callback-or-unresolved |
| `next` | `Err` | [25](../../src/live_tests.rs#L25) | external-constructor-callback-or-unresolved |
| `next` | `Ok` | [26](../../src/live_tests.rs#L26), [32](../../src/live_tests.rs#L32) | external-constructor-callback-or-unresolved |
| `next` | `serde_json::json!({"launch_result":{"child":launch["child"],"spawn_id":launch["spawn_id"],"ok":true}}).to_string` | [26](../../src/live_tests.rs#L26) | receiver-type-required |
| `next` | `message.get("tool_control").is_some` | [28](../../src/live_tests.rs#L28) | receiver-type-required |
| `next` | `self.2.as_ref()?` | [29](../../src/live_tests.rs#L29) | external-constructor-callback-or-unresolved |
| `next` | `self.2.as_ref` | [29](../../src/live_tests.rs#L29) | receiver-type-required |
| `next` | `message.get("lease_request")?.get("attempt")?.as_str` | [31](../../src/live_tests.rs#L31) | receiver-type-required |
| `next` | `message.get("lease_request")?.get` | [31](../../src/live_tests.rs#L31) | receiver-type-required |
| `next` | `serde_json::json!({"lease":{"attempt":attempt,"granted":true}}).to_string` | [32](../../src/live_tests.rs#L32) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_PROVIDER").expect` | [39](../../src/live_tests.rs#L39) | receiver-type-required |
| `live_kernel_turn` | `std::env::var` | [39](../../src/live_tests.rs#L39), [41](../../src/live_tests.rs#L41), [42](../../src/live_tests.rs#L42), [43](../../src/live_tests.rs#L43), [44](../../src/live_tests.rs#L44), [45](../../src/live_tests.rs#L45), [76](../../src/live_tests.rs#L76), [87](../../src/live_tests.rs#L87), [92](../../src/live_tests.rs#L92), [106](../../src/live_tests.rs#L106), [109](../../src/live_tests.rs#L109), [137](../../src/live_tests.rs#L137) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `serde_json::from_slice(&std::fs::read(config_path).unwrap()).unwrap` | [40](../../src/live_tests.rs#L40) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_slice` | [40](../../src/live_tests.rs#L40), [327](../../src/live_tests.rs#L327) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::fs::read(config_path).unwrap` | [40](../../src/live_tests.rs#L40) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read` | [40](../../src/live_tests.rs#L40), [408](../../src/live_tests.rs#L408) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_MODEL").unwrap_or_else` | [41](../../src/live_tests.rs#L41) | receiver-type-required |
| `live_kernel_turn` | `selected_provider.models[0].id.clone` | [41](../../src/live_tests.rs#L41) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_PROMPT").expect` | [42](../../src/live_tests.rs#L42) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_EXPECT").expect` | [43](../../src/live_tests.rs#L43) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_TOOLS").unwrap_or_default` | [44](../../src/live_tests.rs#L44) | receiver-type-required |
| `live_kernel_turn` | `std::path::PathBuf::from` | [45](../../src/live_tests.rs#L45) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_ARTIFACT").expect` | [45](../../src/live_tests.rs#L45) | receiver-type-required |
| `live_kernel_turn` | `std::fs::create_dir_all(&artifact).unwrap` | [46](../../src/live_tests.rs#L46) | receiver-type-required |
| `live_kernel_turn` | `std::fs::create_dir_all` | [46](../../src/live_tests.rs#L46), [55](../../src/live_tests.rs#L55), [78](../../src/live_tests.rs#L78), [241](../../src/live_tests.rs#L241), [469](../../src/live_tests.rs#L469) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `tempfile::tempdir().unwrap` | [47](../../src/live_tests.rs#L47) | receiver-type-required |
| `live_kernel_turn` | `tempfile::tempdir` | [47](../../src/live_tests.rs#L47) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `context_ledger` | [48](../../src/live_tests.rs#L48) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `ledger.path().to_owned` | [49](../../src/live_tests.rs#L49) | receiver-type-required |
| `live_kernel_turn` | `ledger.path` | [49](../../src/live_tests.rs#L49), [157](../../src/live_tests.rs#L157) | receiver-type-required |
| `live_kernel_turn` | `drop` | [50](../../src/live_tests.rs#L50), [536](../../src/live_tests.rs#L536) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `artifact.join` | [53](../../src/live_tests.rs#L53), [57](../../src/live_tests.rs#L57), [238](../../src/live_tests.rs#L238), [240](../../src/live_tests.rs#L240), [251](../../src/live_tests.rs#L251), [254](../../src/live_tests.rs#L254), [267](../../src/live_tests.rs#L267), [277](../../src/live_tests.rs#L277), [327](../../src/live_tests.rs#L327), [406](../../src/live_tests.rs#L406), [468](../../src/live_tests.rs#L468), [505](../../src/live_tests.rs#L505), [514](../../src/live_tests.rs#L514), [515](../../src/live_tests.rs#L515), [530](../../src/live_tests.rs#L530), [531](../../src/live_tests.rs#L531) | receiver-type-required |
| `live_kernel_turn` | `storage_root.join("threads").join` | [54](../../src/live_tests.rs#L54) | receiver-type-required |
| `live_kernel_turn` | `storage_root.join` | [54](../../src/live_tests.rs#L54) | receiver-type-required |
| `live_kernel_turn` | `std::fs::create_dir_all(runtime_folder.join("assets")).unwrap` | [55](../../src/live_tests.rs#L55) | receiver-type-required |
| `live_kernel_turn` | `runtime_folder.join` | [55](../../src/live_tests.rs#L55), [58](../../src/live_tests.rs#L58), [63](../../src/live_tests.rs#L63), [407](../../src/live_tests.rs#L407) | receiver-type-required |
| `live_kernel_turn` | `std::os::unix::fs::symlink(&runtime_folder, artifact.join("runtime-thread")).unwrap` | [57](../../src/live_tests.rs#L57) | receiver-type-required |
| `live_kernel_turn` | `std::os::unix::fs::symlink` | [57](../../src/live_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::fs::copy(&source, &path).unwrap` | [59](../../src/live_tests.rs#L59) | receiver-type-required |
| `live_kernel_turn` | `std::fs::copy` | [59](../../src/live_tests.rs#L59), [63](../../src/live_tests.rs#L63), [238](../../src/live_tests.rs#L238), [245](../../src/live_tests.rs#L245), [251](../../src/live_tests.rs#L251), [514](../../src/live_tests.rs#L514) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::fs::read_dir(source.parent().unwrap().join("assets")).unwrap` | [60](../../src/live_tests.rs#L60) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_dir` | [60](../../src/live_tests.rs#L60), [242](../../src/live_tests.rs#L242), [248](../../src/live_tests.rs#L248), [478](../../src/live_tests.rs#L478) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `source.parent().unwrap().join` | [60](../../src/live_tests.rs#L60) | receiver-type-required |
| `live_kernel_turn` | `source.parent().unwrap` | [60](../../src/live_tests.rs#L60) | receiver-type-required |
| `live_kernel_turn` | `source.parent` | [60](../../src/live_tests.rs#L60) | receiver-type-required |
| `live_kernel_turn` | `entry.unwrap` | [61](../../src/live_tests.rs#L61), [243](../../src/live_tests.rs#L243), [249](../../src/live_tests.rs#L249), [479](../../src/live_tests.rs#L479) | receiver-type-required |
| `live_kernel_turn` | `entry.file_type().unwrap().is_file` | [62](../../src/live_tests.rs#L62), [244](../../src/live_tests.rs#L244) | receiver-type-required |
| `live_kernel_turn` | `entry.file_type().unwrap` | [62](../../src/live_tests.rs#L62), [244](../../src/live_tests.rs#L244) | receiver-type-required |
| `live_kernel_turn` | `entry.file_type` | [62](../../src/live_tests.rs#L62), [244](../../src/live_tests.rs#L244) | receiver-type-required |
| `live_kernel_turn` | `std::fs::copy(entry.path(), runtime_folder.join("assets").join(entry.file_name())).unwrap` | [63](../../src/live_tests.rs#L63) | receiver-type-required |
| `live_kernel_turn` | `entry.path` | [63](../../src/live_tests.rs#L63), [245](../../src/live_tests.rs#L245), [250](../../src/live_tests.rs#L250), [251](../../src/live_tests.rs#L251) | receiver-type-required |
| `live_kernel_turn` | `runtime_folder.join("assets").join` | [63](../../src/live_tests.rs#L63) | receiver-type-required |
| `live_kernel_turn` | `entry.file_name` | [63](../../src/live_tests.rs#L63), [245](../../src/live_tests.rs#L245), [251](../../src/live_tests.rs#L251) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(&path).unwrap` | [66](../../src/live_tests.rs#L66) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string` | [66](../../src/live_tests.rs#L66), [275](../../src/live_tests.rs#L275), [467](../../src/live_tests.rs#L467), [481](../../src/live_tests.rs#L481), [505](../../src/live_tests.rs#L505) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `original.lines().take(4).map(&#124;line&#124; serde_json::from_str::<serde_json::Value>(line).unwrap()).collect::<Vec<_>>` | [67](../../src/live_tests.rs#L67) | receiver-type-required |
| `live_kernel_turn` | `original.lines().take(4).map` | [67](../../src/live_tests.rs#L67) | receiver-type-required |
| `live_kernel_turn` | `original.lines().take` | [67](../../src/live_tests.rs#L67) | receiver-type-required |
| `live_kernel_turn` | `original.lines` | [67](../../src/live_tests.rs#L67) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str::<serde_json::Value>(line).unwrap` | [67](../../src/live_tests.rs#L67) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str::<serde_json::Value>` | [67](../../src/live_tests.rs#L67) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `prefix.iter().fold` | [70](../../src/live_tests.rs#L70) | receiver-type-required |
| `live_kernel_turn` | `prefix.iter` | [70](../../src/live_tests.rs#L70) | receiver-type-required |
| `live_kernel_turn` | `String::new` | [70](../../src/live_tests.rs#L70) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `acc.push_str` | [70](../../src/live_tests.rs#L70) | receiver-type-required |
| `live_kernel_turn` | `serde_json_canonicalizer::to_string(event).unwrap` | [70](../../src/live_tests.rs#L70) | receiver-type-required |
| `live_kernel_turn` | `serde_json_canonicalizer::to_string` | [70](../../src/live_tests.rs#L70) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `acc.push` | [70](../../src/live_tests.rs#L70) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(&path, bytes).unwrap` | [71](../../src/live_tests.rs#L71) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write` | [71](../../src/live_tests.rs#L71), [74](../../src/live_tests.rs#L74), [84](../../src/live_tests.rs#L84), [254](../../src/live_tests.rs#L254), [267](../../src/live_tests.rs#L267), [277](../../src/live_tests.rs#L277), [406](../../src/live_tests.rs#L406), [470](../../src/live_tests.rs#L470), [515](../../src/live_tests.rs#L515), [530](../../src/live_tests.rs#L530), [531](../../src/live_tests.rs#L531) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `store::LockedLedger::open(&path, 1).unwrap` | [72](../../src/live_tests.rs#L72) | receiver-type-required |
| `live_kernel_turn` | `store::LockedLedger::open` | [72](../../src/live_tests.rs#L72) | [store::tail::LockedLedger::open](../../../store/src/tail.rs#L153) |
| `live_kernel_turn` | `prompt.contains` | [73](../../src/live_tests.rs#L73), [113](../../src/live_tests.rs#L113), [125](../../src/live_tests.rs#L125), [204](../../src/live_tests.rs#L204), [272](../../src/live_tests.rs#L272), [299](../../src/live_tests.rs#L299), [316](../../src/live_tests.rs#L316), [351](../../src/live_tests.rs#L351), [352](../../src/live_tests.rs#L352), [401](../../src/live_tests.rs#L401), [414](../../src/live_tests.rs#L414), [425](../../src/live_tests.rs#L425), [437](../../src/live_tests.rs#L437), [455](../../src/live_tests.rs#L455), [463](../../src/live_tests.rs#L463), [465](../../src/live_tests.rs#L465), [497](../../src/live_tests.rs#L497) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(workspace.path().join("repeated.txt"), "KERNEL_REPEATED_OK").unwrap` | [74](../../src/live_tests.rs#L74) | receiver-type-required |
| `live_kernel_turn` | `workspace.path().join` | [74](../../src/live_tests.rs#L74), [78](../../src/live_tests.rs#L78), [84](../../src/live_tests.rs#L84), [274](../../src/live_tests.rs#L274), [467](../../src/live_tests.rs#L467) | receiver-type-required |
| `live_kernel_turn` | `workspace.path` | [74](../../src/live_tests.rs#L74), [78](../../src/live_tests.rs#L78), [84](../../src/live_tests.rs#L84), [90](../../src/live_tests.rs#L90), [274](../../src/live_tests.rs#L274), [467](../../src/live_tests.rs#L467) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_SKILL_FIXTURE").as_deref` | [76](../../src/live_tests.rs#L76), [92](../../src/live_tests.rs#L92) | receiver-type-required |
| `live_kernel_turn` | `Ok` | [76](../../src/live_tests.rs#L76), [87](../../src/live_tests.rs#L87), [92](../../src/live_tests.rs#L92), [178](../../src/live_tests.rs#L178) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::fs::create_dir_all(workspace.path().join("src")).unwrap` | [78](../../src/live_tests.rs#L78) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(workspace.path().join("src").join(name), body).unwrap` | [84](../../src/live_tests.rs#L84) | receiver-type-required |
| `live_kernel_turn` | `workspace.path().join("src").join` | [84](../../src/live_tests.rs#L84) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_GOAL").as_deref` | [87](../../src/live_tests.rs#L87) | receiver-type-required |
| `live_kernel_turn` | `required_tools.split(',').filter(&#124;name&#124; !name.is_empty()).collect::<Vec<_>>` | [88](../../src/live_tests.rs#L88) | receiver-type-required |
| `live_kernel_turn` | `required_tools.split(',').filter` | [88](../../src/live_tests.rs#L88), [450](../../src/live_tests.rs#L450) | receiver-type-required |
| `live_kernel_turn` | `required_tools.split` | [88](../../src/live_tests.rs#L88), [450](../../src/live_tests.rs#L450) | receiver-type-required |
| `live_kernel_turn` | `name.is_empty` | [88](../../src/live_tests.rs#L88), [450](../../src/live_tests.rs#L450) | receiver-type-required |
| `live_kernel_turn` | `selected_tools.push` | [89](../../src/live_tests.rs#L89) | receiver-type-required |
| `live_kernel_turn` | `tool_profile` | [90](../../src/live_tests.rs#L90) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `selected_tools.contains` | [91](../../src/live_tests.rs#L91), [304](../../src/live_tests.rs#L304), [463](../../src/live_tests.rs#L463) | receiver-type-required |
| `live_kernel_turn` | `profile.instruction.sources.push` | [98](../../src/live_tests.rs#L98) | receiver-type-required |
| `live_kernel_turn` | `body.into` | [100](../../src/live_tests.rs#L100) | receiver-type-required |
| `live_kernel_turn` | `profile.instruction.effective.skills.insert` | [103](../../src/live_tests.rs#L103) | receiver-type-required |
| `live_kernel_turn` | `Some` | [106](../../src/live_tests.rs#L106), [111](../../src/live_tests.rs#L111), [112](../../src/live_tests.rs#L112), [114](../../src/live_tests.rs#L114), [116](../../src/live_tests.rs#L116), [118](../../src/live_tests.rs#L118), [133](../../src/live_tests.rs#L133), [146](../../src/live_tests.rs#L146), [153](../../src/live_tests.rs#L153), [161](../../src/live_tests.rs#L161), [189](../../src/live_tests.rs#L189), [232](../../src/live_tests.rs#L232), [250](../../src/live_tests.rs#L250), [264](../../src/live_tests.rs#L264), [280](../../src/live_tests.rs#L280), [283](../../src/live_tests.rs#L283), [318](../../src/live_tests.rs#L318), [354](../../src/live_tests.rs#L354), [426](../../src/live_tests.rs#L426), [431](../../src/live_tests.rs#L431), [434](../../src/live_tests.rs#L434), [443](../../src/live_tests.rs#L443), [447](../../src/live_tests.rs#L447), [452](../../src/live_tests.rs#L452), [456](../../src/live_tests.rs#L456), [480](../../src/live_tests.rs#L480), [500](../../src/live_tests.rs#L500) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_MAX_WALL_SECONDS").map(&#124;value&#124; value.parse::<u64>().expect("positive live wall timeout")).unwrap_or` | [106](../../src/live_tests.rs#L106) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_MAX_WALL_SECONDS").map` | [106](../../src/live_tests.rs#L106) | receiver-type-required |
| `live_kernel_turn` | `value.parse::<u64>().expect` | [106](../../src/live_tests.rs#L106) | receiver-type-required |
| `live_kernel_turn` | `value.parse::<u64>` | [106](../../src/live_tests.rs#L106) | receiver-type-required |
| `live_kernel_turn` | `profile.config.workspace.cwd.clone` | [107](../../src/live_tests.rs#L107) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str(         &std::env::var("TEKES_KERNEL_LIVE_TOOLCHAIN_ROOTS").unwrap_or_else(&#124;_&#124; "[]".to_owned())     ).expect` | [108](../../src/live_tests.rs#L108) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str` | [108](../../src/live_tests.rs#L108), [482](../../src/live_tests.rs#L482) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_TOOLCHAIN_ROOTS").unwrap_or_else` | [109](../../src/live_tests.rs#L109) | receiver-type-required |
| `live_kernel_turn` | `"[]".to_owned` | [109](../../src/live_tests.rs#L109) | receiver-type-required |
| `live_kernel_turn` | `selected_provider.id.clone` | [111](../../src/live_tests.rs#L111), [115](../../src/live_tests.rs#L115) | receiver-type-required |
| `live_kernel_turn` | `model.clone` | [112](../../src/live_tests.rs#L112) | receiver-type-required |
| `live_kernel_turn` | `"high".to_owned` | [116](../../src/live_tests.rs#L116) | receiver-type-required |
| `live_kernel_turn` | `session_controls::execute_goal(&storage_root, "goals.edit",             &json!({"sessionId":session,"ref":{"id":"new","revision":0},"objective":objective}), None).unwrap` | [130](../../src/live_tests.rs#L130) | receiver-type-required |
| `live_kernel_turn` | `session_controls::execute_goal` | [130](../../src/live_tests.rs#L130) | [session-controls::execute_goal](../../../session-controls/src/lib.rs#L245) |
| `live_kernel_turn` | `profile::LaunchBindings::bind(&profile.config,             Some(created["id"].as_str().unwrap().to_owned()), Default::default()).unwrap` | [132](../../src/live_tests.rs#L132) | receiver-type-required |
| `live_kernel_turn` | `profile::LaunchBindings::bind` | [132](../../src/live_tests.rs#L132), [135](../../src/live_tests.rs#L135) | [profile::launch::LaunchBindings::bind](../../../profile/src/launch.rs#L170) |
| `live_kernel_turn` | `created["id"].as_str().unwrap().to_owned` | [133](../../src/live_tests.rs#L133) | receiver-type-required |
| `live_kernel_turn` | `created["id"].as_str().unwrap` | [133](../../src/live_tests.rs#L133) | receiver-type-required |
| `live_kernel_turn` | `created["id"].as_str` | [133](../../src/live_tests.rs#L133) | receiver-type-required |
| `live_kernel_turn` | `Default::default` | [133](../../src/live_tests.rs#L133), [135](../../src/live_tests.rs#L135), [171](../../src/live_tests.rs#L171) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `profile::LaunchBindings::bind(&profile.config, None, Default::default()).unwrap` | [135](../../src/live_tests.rs#L135) | receiver-type-required |
| `live_kernel_turn` | `std::env::var("TEKES_KERNEL_LIVE_KEY").expect` | [137](../../src/live_tests.rs#L137) | receiver-type-required |
| `live_kernel_turn` | `std::os::unix::net::UnixStream::pair().unwrap` | [139](../../src/live_tests.rs#L139) | receiver-type-required |
| `live_kernel_turn` | `std::os::unix::net::UnixStream::pair` | [139](../../src/live_tests.rs#L139) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `provider::start_credential_channel` | [140](../../src/live_tests.rs#L140) | [provider::credential::start_credential_channel](../../../provider/src/credential.rs#L519) |
| `live_kernel_turn` | `provider::CredentialBroker::new` | [140](../../src/live_tests.rs#L140) | [provider::credential::CredentialBroker::new](../../../provider/src/credential.rs#L355) |
| `live_kernel_turn` | `selected_provider.credential_key.clone().unwrap` | [141](../../src/live_tests.rs#L141) | receiver-type-required |
| `live_kernel_turn` | `selected_provider.credential_key.clone` | [141](../../src/live_tests.rs#L141) | receiver-type-required |
| `live_kernel_turn` | `selected_provider.adapter.clone` | [142](../../src/live_tests.rs#L142) | receiver-type-required |
| `live_kernel_turn` | `provider::endpoint_origin(&selected_provider.endpoint).unwrap` | [143](../../src/live_tests.rs#L143) | receiver-type-required |
| `live_kernel_turn` | `provider::endpoint_origin` | [143](../../src/live_tests.rs#L143) | [provider::request::endpoint_origin](../../../provider/src/request.rs#L113) |
| `live_kernel_turn` | `"provider".to_owned` | [144](../../src/live_tests.rs#L144) | receiver-type-required |
| `live_kernel_turn` | `"1".to_owned` | [144](../../src/live_tests.rs#L144) | receiver-type-required |
| `live_kernel_turn` | `std::sync::Arc::new` | [146](../../src/live_tests.rs#L146) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::sync::Mutex::new` | [146](../../src/live_tests.rs#L146) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `provider::CredentialClient::new` | [146](../../src/live_tests.rs#L146) | [provider::credential::CredentialClient::new](../../../provider/src/credential.rs#L271) |
| `live_kernel_turn` | `LiveControl::default` | [147](../../src/live_tests.rs#L147) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `tekes_supervisor::production_tool_control::ProductionToolControlHandler::new` | [148](../../src/live_tests.rs#L148) | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::new](../../../supervisor/src/production_tool_control.rs#L273) |
| `live_kernel_turn` | `tekes_supervisor::production_tool_control::ProductionToolControlPolicy::new` | [150](../../src/live_tests.rs#L150) | [tekes-supervisor::production_tool_control::ProductionToolControlPolicy::new](../../../supervisor/src/production_tool_control.rs#L873) |
| `live_kernel_turn` | `SecretScanner::default` | [150](../../src/live_tests.rs#L150) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::rc::Rc::new` | [151](../../src/live_tests.rs#L151), [153](../../src/live_tests.rs#L153), [161](../../src/live_tests.rs#L161) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::cell::RefCell::new` | [151](../../src/live_tests.rs#L151) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `tekes_supervisor::tool_control::ToolControlSession::new` | [151](../../src/live_tests.rs#L151) | [tekes-supervisor::tool_control::ToolControlSession::new](../../../supervisor/src/tool_control.rs#L125) |
| `live_kernel_turn` | `session.borrow_mut().handle_line(line).map_err` | [154](../../src/live_tests.rs#L154) | receiver-type-required |
| `live_kernel_turn` | `session.borrow_mut().handle_line` | [154](../../src/live_tests.rs#L154) | receiver-type-required |
| `live_kernel_turn` | `session.borrow_mut` | [154](../../src/live_tests.rs#L154) | receiver-type-required |
| `live_kernel_turn` | `std::io::Error::other` | [154](../../src/live_tests.rs#L154), [194](../../src/live_tests.rs#L194) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `error.to_string` | [154](../../src/live_tests.rs#L154), [194](../../src/live_tests.rs#L194) | receiver-type-required |
| `live_kernel_turn` | `String::from_utf8(response).map_err` | [155](../../src/live_tests.rs#L155) | receiver-type-required |
| `live_kernel_turn` | `String::from_utf8` | [155](../../src/live_tests.rs#L155) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `ledger.path().parent().unwrap().to_owned` | [157](../../src/live_tests.rs#L157) | receiver-type-required |
| `live_kernel_turn` | `ledger.path().parent().unwrap` | [157](../../src/live_tests.rs#L157) | receiver-type-required |
| `live_kernel_turn` | `ledger.path().parent` | [157](../../src/live_tests.rs#L157) | receiver-type-required |
| `live_kernel_turn` | `profile.clone` | [158](../../src/live_tests.rs#L158) | receiver-type-required |
| `live_kernel_turn` | `credential.clone` | [159](../../src/live_tests.rs#L159) | receiver-type-required |
| `live_kernel_turn` | `output.2.clone` | [160](../../src/live_tests.rs#L160) | receiver-type-required |
| `live_kernel_turn` | `launch["child"].as_str().ok_or` | [163](../../src/live_tests.rs#L163) | receiver-type-required |
| `live_kernel_turn` | `launch["child"].as_str` | [163](../../src/live_tests.rs#L163) | receiver-type-required |
| `live_kernel_turn` | `LockedLedger::open` | [164](../../src/live_tests.rs#L164) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `child_folder.join` | [164](../../src/live_tests.rs#L164) | receiver-type-required |
| `live_kernel_turn` | `test_options` | [165](../../src/live_tests.rs#L165), [197](../../src/live_tests.rs#L197) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `child.projection().unwrap` | [167](../../src/live_tests.rs#L167), [178](../../src/live_tests.rs#L178) | receiver-type-required |
| `live_kernel_turn` | `child.projection` | [167](../../src/live_tests.rs#L167), [178](../../src/live_tests.rs#L178) | receiver-type-required |
| `live_kernel_turn` | `append_run_start` | [168](../../src/live_tests.rs#L168) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `append_turn_open` | [169](../../src/live_tests.rs#L169) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `child_control.clone` | [171](../../src/live_tests.rs#L171) | receiver-type-required |
| `live_kernel_turn` | `child_output.clone` | [172](../../src/live_tests.rs#L172) | receiver-type-required |
| `live_kernel_turn` | `child_credential.clone` | [173](../../src/live_tests.rs#L173) | receiver-type-required |
| `live_kernel_turn` | `RuntimeCancellation::default` | [174](../../src/live_tests.rs#L174), [203](../../src/live_tests.rs#L203) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `run_provider_turn` | [176](../../src/live_tests.rs#L176), [212](../../src/live_tests.rs#L212), [216](../../src/live_tests.rs#L216), [234](../../src/live_tests.rs#L234), [512](../../src/live_tests.rs#L512) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `pending_tool_calls` | [179](../../src/live_tests.rs#L179), [223](../../src/live_tests.rs#L223) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `pending.iter().filter(&#124;call&#124; call.approval_requested && call.approval_response.is_none()).collect::<Vec<_>>` | [180](../../src/live_tests.rs#L180), [224](../../src/live_tests.rs#L224) | receiver-type-required |
| `live_kernel_turn` | `pending.iter().filter` | [180](../../src/live_tests.rs#L180), [224](../../src/live_tests.rs#L224) | receiver-type-required |
| `live_kernel_turn` | `pending.iter` | [180](../../src/live_tests.rs#L180), [224](../../src/live_tests.rs#L224) | receiver-type-required |
| `live_kernel_turn` | `call.approval_response.is_none` | [180](../../src/live_tests.rs#L180), [224](../../src/live_tests.rs#L224) | receiver-type-required |
| `live_kernel_turn` | `held.is_empty` | [181](../../src/live_tests.rs#L181), [225](../../src/live_tests.rs#L225) | receiver-type-required |
| `live_kernel_turn` | `Err` | [181](../../src/live_tests.rs#L181), [183](../../src/live_tests.rs#L183), [192](../../src/live_tests.rs#L192) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `"live validator stopped without settlement or approval".into` | [181](../../src/live_tests.rs#L181) | receiver-type-required |
| `live_kernel_turn` | `effective_allowed_tools(&child_profile).contains` | [183](../../src/live_tests.rs#L183) | receiver-type-required |
| `live_kernel_turn` | `effective_allowed_tools` | [183](../../src/live_tests.rs#L183) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `"validator approval outside live scenario".into` | [183](../../src/live_tests.rs#L183) | receiver-type-required |
| `live_kernel_turn` | `child.append_contract` | [185](../../src/live_tests.rs#L185) | receiver-type-required |
| `live_kernel_turn` | `make_event` | [185](../../src/live_tests.rs#L185), [229](../../src/live_tests.rs#L229) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `BarrierContext::default` | [187](../../src/live_tests.rs#L187) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `recover_unpaired_tool_calls` | [189](../../src/live_tests.rs#L189), [232](../../src/live_tests.rs#L232) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `"validator exceeded live approval bound".into` | [192](../../src/live_tests.rs#L192) | receiver-type-required |
| `live_kernel_turn` | `execute().map_err` | [194](../../src/live_tests.rs#L194) | receiver-type-required |
| `live_kernel_turn` | `execute` | [194](../../src/live_tests.rs#L194) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `output.clone` | [196](../../src/live_tests.rs#L196) | receiver-type-required |
| `live_kernel_turn` | `(chrono::Utc::now() + chrono::Duration::seconds(1))             .to_rfc3339_opts` | [199](../../src/live_tests.rs#L199) | receiver-type-required |
| `live_kernel_turn` | `chrono::Utc::now` | [199](../../src/live_tests.rs#L199) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `chrono::Duration::seconds` | [199](../../src/live_tests.rs#L199) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `std::time::Instant::now` | [202](../../src/live_tests.rs#L202) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `cancellation.park_delivery` | [205](../../src/live_tests.rs#L205) | receiver-type-required |
| `live_kernel_turn` | `json!({"input":{"assets":null,             "content":[{"type":"text","text":"Do not call tools. Reply exactly KERNEL_QUEUE_OK."}],             "delivery":"live-queued-second","origin":{"client":"live-harness","key":"queued-second",                 "op":"submit","principal":"live-test","target":"test"},"steer":null}}).to_string` | [205](../../src/live_tests.rs#L205) | receiver-type-required |
| `live_kernel_turn` | `drain_parked_deliveries(&mut ledger, &options, &Selected {version:2}, &cancellation, &mut output).unwrap` | [209](../../src/live_tests.rs#L209) | receiver-type-required |
| `live_kernel_turn` | `drain_parked_deliveries` | [209](../../src/live_tests.rs#L209) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `result.is_err` | [215](../../src/live_tests.rs#L215), [222](../../src/live_tests.rs#L222) | receiver-type-required |
| `live_kernel_turn` | `open_goal_continuation(&mut ledger, &options, &profile, &cancellation).unwrap` | [215](../../src/live_tests.rs#L215) | receiver-type-required |
| `live_kernel_turn` | `open_goal_continuation` | [215](../../src/live_tests.rs#L215) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `started.elapsed().as_secs` | [222](../../src/live_tests.rs#L222) | receiver-type-required |
| `live_kernel_turn` | `started.elapsed` | [222](../../src/live_tests.rs#L222) | receiver-type-required |
| `live_kernel_turn` | `pending_tool_calls(&ledger).unwrap` | [223](../../src/live_tests.rs#L223) | receiver-type-required |
| `live_kernel_turn` | `make_event(serde_json::json!({"v":1,"seq":ledger.next_seq(),"turn":call.turn,"kind":"approval_response","ts":options.timestamp,"call":call.call,"grant":true,"answer":"approved disposable live-test operation","origin_key":key,"origin_tuple":{"principal":"live-test","client":"live-harness","target":"test","op":"approve","key":key}})).unwrap` | [229](../../src/live_tests.rs#L229) | receiver-type-required |
| `live_kernel_turn` | `ledger.append_contract(event, store::BarrierContext::default()).unwrap` | [230](../../src/live_tests.rs#L230) | receiver-type-required |
| `live_kernel_turn` | `ledger.append_contract` | [230](../../src/live_tests.rs#L230) | receiver-type-required |
| `live_kernel_turn` | `store::BarrierContext::default` | [230](../../src/live_tests.rs#L230) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `recover_unpaired_tool_calls(&mut ledger, &options, Some(&profile), Selected { version: 2 }, false, &mut replies, &mut output, &cancellation).map` | [232](../../src/live_tests.rs#L232) | receiver-type-required |
| `live_kernel_turn` | `result.is_ok` | [233](../../src/live_tests.rs#L233) | receiver-type-required |
| `live_kernel_turn` | `std::fs::copy(&path, artifact.join("main.jsonl")).unwrap` | [238](../../src/live_tests.rs#L238), [514](../../src/live_tests.rs#L514) | receiver-type-required |
| `live_kernel_turn` | `path.parent().unwrap().join` | [239](../../src/live_tests.rs#L239) | receiver-type-required |
| `live_kernel_turn` | `path.parent().unwrap` | [239](../../src/live_tests.rs#L239), [248](../../src/live_tests.rs#L248) | receiver-type-required |
| `live_kernel_turn` | `path.parent` | [239](../../src/live_tests.rs#L239), [248](../../src/live_tests.rs#L248) | receiver-type-required |
| `live_kernel_turn` | `std::fs::create_dir_all(&asset_destination).unwrap` | [241](../../src/live_tests.rs#L241) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_dir(&asset_source).unwrap` | [242](../../src/live_tests.rs#L242) | receiver-type-required |
| `live_kernel_turn` | `std::fs::copy(entry.path(),asset_destination.join(entry.file_name())).unwrap` | [245](../../src/live_tests.rs#L245) | receiver-type-required |
| `live_kernel_turn` | `asset_destination.join` | [245](../../src/live_tests.rs#L245) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_dir(path.parent().unwrap()).unwrap` | [248](../../src/live_tests.rs#L248) | receiver-type-required |
| `live_kernel_turn` | `entry.path().extension().and_then` | [250](../../src/live_tests.rs#L250) | receiver-type-required |
| `live_kernel_turn` | `entry.path().extension` | [250](../../src/live_tests.rs#L250) | receiver-type-required |
| `live_kernel_turn` | `s.to_str` | [250](../../src/live_tests.rs#L250), [480](../../src/live_tests.rs#L480) | receiver-type-required |
| `live_kernel_turn` | `std::fs::copy(entry.path(), artifact.join(entry.file_name())).unwrap` | [251](../../src/live_tests.rs#L251) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(artifact.join("control.jsonl"), &*output.0.borrow()).unwrap` | [254](../../src/live_tests.rs#L254), [515](../../src/live_tests.rs#L515) | receiver-type-required |
| `live_kernel_turn` | `output.0.borrow` | [254](../../src/live_tests.rs#L254), [515](../../src/live_tests.rs#L515) | receiver-type-required |
| `live_kernel_turn` | `ledger.projection().unwrap` | [255](../../src/live_tests.rs#L255), [509](../../src/live_tests.rs#L509), [517](../../src/live_tests.rs#L517) | receiver-type-required |
| `live_kernel_turn` | `ledger.projection` | [255](../../src/live_tests.rs#L255), [509](../../src/live_tests.rs#L509), [517](../../src/live_tests.rs#L517) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::AttemptDispatched).count` | [256](../../src/live_tests.rs#L256) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter` | [256](../../src/live_tests.rs#L256), [257](../../src/live_tests.rs#L257), [258](../../src/live_tests.rs#L258), [280](../../src/live_tests.rs#L280), [283](../../src/live_tests.rs#L283), [317](../../src/live_tests.rs#L317), [323](../../src/live_tests.rs#L323), [353](../../src/live_tests.rs#L353), [416](../../src/live_tests.rs#L416), [426](../../src/live_tests.rs#L426), [443](../../src/live_tests.rs#L443), [445](../../src/live_tests.rs#L445), [447](../../src/live_tests.rs#L447), [452](../../src/live_tests.rs#L452), [456](../../src/live_tests.rs#L456), [503](../../src/live_tests.rs#L503) | receiver-type-required |
| `live_kernel_turn` | `events.iter` | [256](../../src/live_tests.rs#L256), [257](../../src/live_tests.rs#L257), [258](../../src/live_tests.rs#L258), [280](../../src/live_tests.rs#L280), [283](../../src/live_tests.rs#L283), [305](../../src/live_tests.rs#L305), [317](../../src/live_tests.rs#L317), [323](../../src/live_tests.rs#L323), [343](../../src/live_tests.rs#L343), [353](../../src/live_tests.rs#L353), [364](../../src/live_tests.rs#L364), [416](../../src/live_tests.rs#L416), [426](../../src/live_tests.rs#L426), [434](../../src/live_tests.rs#L434), [443](../../src/live_tests.rs#L443), [445](../../src/live_tests.rs#L445), [447](../../src/live_tests.rs#L447), [452](../../src/live_tests.rs#L452), [456](../../src/live_tests.rs#L456), [499](../../src/live_tests.rs#L499), [503](../../src/live_tests.rs#L503), [523](../../src/live_tests.rs#L523) | receiver-type-required |
| `live_kernel_turn` | `event.kind` | [256](../../src/live_tests.rs#L256), [257](../../src/live_tests.rs#L257), [258](../../src/live_tests.rs#L258), [264](../../src/live_tests.rs#L264), [317](../../src/live_tests.rs#L317), [323](../../src/live_tests.rs#L323), [343](../../src/live_tests.rs#L343), [353](../../src/live_tests.rs#L353), [364](../../src/live_tests.rs#L364), [452](../../src/live_tests.rs#L452), [499](../../src/live_tests.rs#L499), [503](../../src/live_tests.rs#L503) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::ToolCall).filter_map(&#124;event&#124; event.string_field("name")).collect::<Vec<_>>` | [257](../../src/live_tests.rs#L257) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::ToolCall).filter_map` | [257](../../src/live_tests.rs#L257) | receiver-type-required |
| `live_kernel_turn` | `event.string_field` | [257](../../src/live_tests.rs#L257), [264](../../src/live_tests.rs#L264), [318](../../src/live_tests.rs#L318), [344](../../src/live_tests.rs#L344), [354](../../src/live_tests.rs#L354), [361](../../src/live_tests.rs#L361), [365](../../src/live_tests.rs#L365), [417](../../src/live_tests.rs#L417), [418](../../src/live_tests.rs#L418), [452](../../src/live_tests.rs#L452), [500](../../src/live_tests.rs#L500) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::Output).map(&#124;event&#124; serde_json::to_value(event.raw()).unwrap()).collect::<Vec<_>>` | [258](../../src/live_tests.rs#L258) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::Output).map` | [258](../../src/live_tests.rs#L258) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value(event.raw()).unwrap` | [258](../../src/live_tests.rs#L258), [259](../../src/live_tests.rs#L259), [285](../../src/live_tests.rs#L285), [305](../../src/live_tests.rs#L305), [357](../../src/live_tests.rs#L357) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value` | [258](../../src/live_tests.rs#L258), [259](../../src/live_tests.rs#L259), [285](../../src/live_tests.rs#L285), [305](../../src/live_tests.rs#L305), [340](../../src/live_tests.rs#L340), [357](../../src/live_tests.rs#L357), [369](../../src/live_tests.rs#L369), [393](../../src/live_tests.rs#L393), [427](../../src/live_tests.rs#L427), [518](../../src/live_tests.rs#L518), [524](../../src/live_tests.rs#L524) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `event.raw` | [258](../../src/live_tests.rs#L258), [259](../../src/live_tests.rs#L259), [285](../../src/live_tests.rs#L285), [305](../../src/live_tests.rs#L305), [357](../../src/live_tests.rs#L357) | receiver-type-required |
| `live_kernel_turn` | `events.last().map(&#124;event&#124; serde_json::to_value(event.raw()).unwrap()).unwrap_or` | [259](../../src/live_tests.rs#L259) | receiver-type-required |
| `live_kernel_turn` | `events.last().map` | [259](../../src/live_tests.rs#L259) | receiver-type-required |
| `live_kernel_turn` | `events.last` | [259](../../src/live_tests.rs#L259), [264](../../src/live_tests.rs#L264), [498](../../src/live_tests.rs#L498), [518](../../src/live_tests.rs#L518) | receiver-type-required |
| `live_kernel_turn` | `settlement.get("validation").cloned().unwrap_or` | [260](../../src/live_tests.rs#L260) | receiver-type-required |
| `live_kernel_turn` | `settlement.get("validation").cloned` | [260](../../src/live_tests.rs#L260) | receiver-type-required |
| `live_kernel_turn` | `settlement.get` | [260](../../src/live_tests.rs#L260) | receiver-type-required |
| `live_kernel_turn` | `settlement["promoted_output_seq"].as_u64().or_else` | [261](../../src/live_tests.rs#L261) | receiver-type-required |
| `live_kernel_turn` | `settlement["promoted_output_seq"].as_u64` | [261](../../src/live_tests.rs#L261) | receiver-type-required |
| `live_kernel_turn` | `validation["promoted_output_seq"].as_u64` | [261](../../src/live_tests.rs#L261) | receiver-type-required |
| `live_kernel_turn` | `answers.iter().find` | [262](../../src/live_tests.rs#L262), [431](../../src/live_tests.rs#L431) | receiver-type-required |
| `live_kernel_turn` | `answers.iter` | [262](../../src/live_tests.rs#L262), [431](../../src/live_tests.rs#L431) | receiver-type-required |
| `live_kernel_turn` | `event["seq"].as_u64` | [262](../../src/live_tests.rs#L262) | receiver-type-required |
| `live_kernel_turn` | `final_output.map(&#124;event&#124; event["content"].as_array().unwrap().iter().filter_map(&#124;block&#124; block["text"].as_str()).collect::<String>()).unwrap_or_default` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `final_output.map` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `event["content"].as_array().unwrap().iter().filter_map(&#124;block&#124; block["text"].as_str()).collect::<String>` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `event["content"].as_array().unwrap().iter().filter_map` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `event["content"].as_array().unwrap().iter` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `event["content"].as_array().unwrap` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `event["content"].as_array` | [263](../../src/live_tests.rs#L263) | receiver-type-required |
| `live_kernel_turn` | `block["text"].as_str` | [263](../../src/live_tests.rs#L263), [287](../../src/live_tests.rs#L287), [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `events.last().is_some_and` | [264](../../src/live_tests.rs#L264) | receiver-type-required |
| `live_kernel_turn` | `session_controls::read_goal(&storage_root, "018f0000-0000-7000-8000-000000000003").unwrap().map` | [265](../../src/live_tests.rs#L265) | receiver-type-required |
| `live_kernel_turn` | `session_controls::read_goal(&storage_root, "018f0000-0000-7000-8000-000000000003").unwrap` | [265](../../src/live_tests.rs#L265) | receiver-type-required |
| `live_kernel_turn` | `session_controls::read_goal` | [265](../../src/live_tests.rs#L265) | [session-controls::read_goal](../../../session-controls/src/lib.rs#L366) |
| `live_kernel_turn` | `std::fs::write(artifact.join("result.json"), serde_json::to_vec_pretty(&report).unwrap()).unwrap` | [267](../../src/live_tests.rs#L267) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_vec_pretty(&report).unwrap` | [267](../../src/live_tests.rs#L267) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_vec_pretty` | [267](../../src/live_tests.rs#L267), [406](../../src/live_tests.rs#L406), [530](../../src/live_tests.rs#L530), [531](../../src/live_tests.rs#L531) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `result.expect` | [268](../../src/live_tests.rs#L268) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(&file).expect` | [275](../../src/live_tests.rs#L275) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(artifact.join("task-deliverable.md"), body).unwrap` | [277](../../src/live_tests.rs#L277) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("shell"))             .filter_map(&#124;e&#124; e.string_field("call")).collect::<std::collections::HashSet<_>>` | [280](../../src/live_tests.rs#L280) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("shell"))             .filter_map` | [280](../../src/live_tests.rs#L280), [456](../../src/live_tests.rs#L456) | receiver-type-required |
| `live_kernel_turn` | `e.kind` | [280](../../src/live_tests.rs#L280), [283](../../src/live_tests.rs#L283), [416](../../src/live_tests.rs#L416), [445](../../src/live_tests.rs#L445), [447](../../src/live_tests.rs#L447), [456](../../src/live_tests.rs#L456) | receiver-type-required |
| `live_kernel_turn` | `e.string_field` | [280](../../src/live_tests.rs#L280), [281](../../src/live_tests.rs#L281), [283](../../src/live_tests.rs#L283), [284](../../src/live_tests.rs#L284), [321](../../src/live_tests.rs#L321), [426](../../src/live_tests.rs#L426), [434](../../src/live_tests.rs#L434), [443](../../src/live_tests.rs#L443), [447](../../src/live_tests.rs#L447), [456](../../src/live_tests.rs#L456), [457](../../src/live_tests.rs#L457) | receiver-type-required |
| `live_kernel_turn` | `std::collections::HashSet::new` | [282](../../src/live_tests.rs#L282) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `e.string_field("call").is_some_and` | [284](../../src/live_tests.rs#L284) | receiver-type-required |
| `live_kernel_turn` | `shell_ids.contains` | [284](../../src/live_tests.rs#L284) | receiver-type-required |
| `live_kernel_turn` | `raw["content"].as_array().unwrap` | [286](../../src/live_tests.rs#L286), [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `raw["content"].as_array` | [286](../../src/live_tests.rs#L286), [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str::<Value>` | [287](../../src/live_tests.rs#L287) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `block["text"].as_str().unwrap_or_default` | [287](../../src/live_tests.rs#L287) | receiver-type-required |
| `live_kernel_turn` | `result["steps"].as_array().into_iter().flatten` | [289](../../src/live_tests.rs#L289) | receiver-type-required |
| `live_kernel_turn` | `result["steps"].as_array().into_iter` | [289](../../src/live_tests.rs#L289) | receiver-type-required |
| `live_kernel_turn` | `result["steps"].as_array` | [289](../../src/live_tests.rs#L289) | receiver-type-required |
| `live_kernel_turn` | `step["stdout"]["data"].as_str().unwrap_or_default().lines` | [291](../../src/live_tests.rs#L291) | receiver-type-required |
| `live_kernel_turn` | `step["stdout"]["data"].as_str().unwrap_or_default` | [291](../../src/live_tests.rs#L291) | receiver-type-required |
| `live_kernel_turn` | `step["stdout"]["data"].as_str` | [291](../../src/live_tests.rs#L291) | receiver-type-required |
| `live_kernel_turn` | `lines.insert` | [291](../../src/live_tests.rs#L291) | receiver-type-required |
| `live_kernel_turn` | `line.to_owned` | [291](../../src/live_tests.rs#L291) | receiver-type-required |
| `live_kernel_turn` | `events.iter().map(&#124;event&#124; serde_json::to_value(event.raw()).unwrap()).collect::<Vec<_>>` | [305](../../src/live_tests.rs#L305) | receiver-type-required |
| `live_kernel_turn` | `events.iter().map` | [305](../../src/live_tests.rs#L305) | receiver-type-required |
| `live_kernel_turn` | `raw.iter().find(&#124;event&#124; event["kind"]=="tool_call" && event["name"]==name).expect` | [308](../../src/live_tests.rs#L308) | receiver-type-required |
| `live_kernel_turn` | `raw.iter().find` | [308](../../src/live_tests.rs#L308), [309](../../src/live_tests.rs#L309) | receiver-type-required |
| `live_kernel_turn` | `raw.iter` | [308](../../src/live_tests.rs#L308), [309](../../src/live_tests.rs#L309) | receiver-type-required |
| `live_kernel_turn` | `raw.iter().find(&#124;event&#124; event["kind"]=="tool_result" && event["call"]==call["call"]).expect` | [309](../../src/live_tests.rs#L309) | receiver-type-required |
| `live_kernel_turn` | `result["seq"].as_u64().unwrap` | [312](../../src/live_tests.rs#L312) | receiver-type-required |
| `live_kernel_turn` | `result["seq"].as_u64` | [312](../../src/live_tests.rs#L312) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; event.kind() == &EventKind::ToolCall             && event.string_field("name") == Some("think")).collect::<Vec<_>>` | [317](../../src/live_tests.rs#L317) | receiver-type-required |
| `live_kernel_turn` | `calls.iter().map(&#124;e&#124; e.string_field("attempt").unwrap()).collect::<std::collections::BTreeSet<_>>` | [321](../../src/live_tests.rs#L321) | receiver-type-required |
| `live_kernel_turn` | `calls.iter().map` | [321](../../src/live_tests.rs#L321) | receiver-type-required |
| `live_kernel_turn` | `calls.iter` | [321](../../src/live_tests.rs#L321), [339](../../src/live_tests.rs#L339) | receiver-type-required |
| `live_kernel_turn` | `e.string_field("attempt").unwrap` | [321](../../src/live_tests.rs#L321) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; event.kind() == &EventKind::Attempt).collect::<Vec<_>>` | [323](../../src/live_tests.rs#L323) | receiver-type-required |
| `live_kernel_turn` | `requests.iter().enumerate` | [325](../../src/live_tests.rs#L325) | receiver-type-required |
| `live_kernel_turn` | `requests.iter` | [325](../../src/live_tests.rs#L325) | receiver-type-required |
| `live_kernel_turn` | `attempt.string_field("attempt").unwrap` | [326](../../src/live_tests.rs#L326), [504](../../src/live_tests.rs#L504) | receiver-type-required |
| `live_kernel_turn` | `attempt.string_field` | [326](../../src/live_tests.rs#L326), [504](../../src/live_tests.rs#L504) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_slice(&fs::read(artifact.join(format!("{id}.request.json"))).unwrap()).unwrap` | [327](../../src/live_tests.rs#L327) | receiver-type-required |
| `live_kernel_turn` | `fs::read(artifact.join(format!("{id}.request.json"))).unwrap` | [327](../../src/live_tests.rs#L327) | receiver-type-required |
| `live_kernel_turn` | `fs::read` | [327](../../src/live_tests.rs#L327) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `body.get("tool_choice").and_then(&#124;v&#124; v.as_str().or_else(&#124;&#124; v.get("type").and_then(Value::as_str)))                 .or_else(&#124;&#124; body.pointer("/toolConfig/functionCallingConfig/mode").and_then(Value::as_str)).unwrap_or` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `body.get("tool_choice").and_then(&#124;v&#124; v.as_str().or_else(&#124;&#124; v.get("type").and_then(Value::as_str)))                 .or_else` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `body.get("tool_choice").and_then` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `body.get` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `v.as_str().or_else` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `v.as_str` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `v.get("type").and_then` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `v.get` | [328](../../src/live_tests.rs#L328) | receiver-type-required |
| `live_kernel_turn` | `body.pointer("/toolConfig/functionCallingConfig/mode").and_then` | [329](../../src/live_tests.rs#L329) | receiver-type-required |
| `live_kernel_turn` | `body.pointer` | [329](../../src/live_tests.rs#L329) | receiver-type-required |
| `live_kernel_turn` | `calls.iter().enumerate` | [339](../../src/live_tests.rs#L339) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value(call.raw()).unwrap` | [340](../../src/live_tests.rs#L340) | receiver-type-required |
| `live_kernel_turn` | `call.raw` | [340](../../src/live_tests.rs#L340) | receiver-type-required |
| `live_kernel_turn` | `materialize_json(&ledger, &raw["args"]).unwrap` | [341](../../src/live_tests.rs#L341), [358](../../src/live_tests.rs#L358) | receiver-type-required |
| `live_kernel_turn` | `materialize_json` | [341](../../src/live_tests.rs#L341), [358](../../src/live_tests.rs#L358) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `events.iter().find(&#124;event&#124; event.kind() == &EventKind::ToolResult                 && event.string_field("call") == call.string_field("call")).unwrap` | [343](../../src/live_tests.rs#L343) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find` | [343](../../src/live_tests.rs#L343), [364](../../src/live_tests.rs#L364), [434](../../src/live_tests.rs#L434), [499](../../src/live_tests.rs#L499), [523](../../src/live_tests.rs#L523) | receiver-type-required |
| `live_kernel_turn` | `call.string_field` | [344](../../src/live_tests.rs#L344), [365](../../src/live_tests.rs#L365) | receiver-type-required |
| `live_kernel_turn` | `calls.get` | [347](../../src/live_tests.rs#L347) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; event.kind() == &EventKind::ToolCall             && event.string_field("name") == Some(name)).collect::<Vec<_>>` | [353](../../src/live_tests.rs#L353) | receiver-type-required |
| `live_kernel_turn` | `repeated.iter().map(&#124;event&#124; {             let raw = serde_json::to_value(event.raw()).unwrap();             materialize_json(&ledger, &raw["args"]).unwrap()         }).collect::<Vec<_>>` | [356](../../src/live_tests.rs#L356) | receiver-type-required |
| `live_kernel_turn` | `repeated.iter().map` | [356](../../src/live_tests.rs#L356), [361](../../src/live_tests.rs#L361) | receiver-type-required |
| `live_kernel_turn` | `repeated.iter` | [356](../../src/live_tests.rs#L356), [361](../../src/live_tests.rs#L361), [363](../../src/live_tests.rs#L363) | receiver-type-required |
| `live_kernel_turn` | `repeated.iter().map(&#124;event&#124; event.string_field("attempt").unwrap()).collect::<std::collections::BTreeSet<_>>` | [361](../../src/live_tests.rs#L361) | receiver-type-required |
| `live_kernel_turn` | `event.string_field("attempt").unwrap` | [361](../../src/live_tests.rs#L361), [417](../../src/live_tests.rs#L417) | receiver-type-required |
| `live_kernel_turn` | `repeated.iter().enumerate` | [363](../../src/live_tests.rs#L363) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find(&#124;event&#124; event.kind() == &EventKind::ToolResult                 && event.string_field("call") == call.string_field("call")).expect` | [364](../../src/live_tests.rs#L364) | receiver-type-required |
| `live_kernel_turn` | `validation_runtime::result_value(&ledger, result).unwrap` | [368](../../src/live_tests.rs#L368) | receiver-type-required |
| `live_kernel_turn` | `validation_runtime::result_value` | [368](../../src/live_tests.rs#L368) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `serde_json::to_value(events[0].raw()).unwrap` | [369](../../src/live_tests.rs#L369) | receiver-type-required |
| `live_kernel_turn` | `events[0].raw` | [369](../../src/live_tests.rs#L369) | receiver-type-required |
| `live_kernel_turn` | `repeated.get` | [377](../../src/live_tests.rs#L377) | receiver-type-required |
| `live_kernel_turn` | `endpoint::EndpointJournal::open(&runtime_folder).unwrap` | [384](../../src/live_tests.rs#L384), [409](../../src/live_tests.rs#L409) | receiver-type-required |
| `live_kernel_turn` | `endpoint::EndpointJournal::open` | [384](../../src/live_tests.rs#L384), [409](../../src/live_tests.rs#L409) | [endpoint::journal::EndpointJournal::open](../../../endpoint/src/journal.rs#L154) |
| `live_kernel_turn` | `endpoint::Projector::default` | [385](../../src/live_tests.rs#L385), [410](../../src/live_tests.rs#L410) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `Vec::new` | [386](../../src/live_tests.rs#L386) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `projector.reconcile(std::slice::from_ref(event), &journal).unwrap` | [388](../../src/live_tests.rs#L388) | receiver-type-required |
| `live_kernel_turn` | `projector.reconcile` | [388](../../src/live_tests.rs#L388), [528](../../src/live_tests.rs#L528) | receiver-type-required |
| `live_kernel_turn` | `std::slice::from_ref` | [388](../../src/live_tests.rs#L388) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `serde_json::to_value(&wire.data).unwrap` | [393](../../src/live_tests.rs#L393) | receiver-type-required |
| `live_kernel_turn` | `wire_events.extend` | [398](../../src/live_tests.rs#L398) | receiver-type-required |
| `live_kernel_turn` | `wire_events.iter().filter(&#124;event&#124; event.event_type == "turn/end").count` | [400](../../src/live_tests.rs#L400) | receiver-type-required |
| `live_kernel_turn` | `wire_events.iter().filter` | [400](../../src/live_tests.rs#L400) | receiver-type-required |
| `live_kernel_turn` | `wire_events.iter` | [400](../../src/live_tests.rs#L400) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(artifact.join("endpoint-events.json"), serde_json::to_vec_pretty(&wire_events).unwrap()).unwrap` | [406](../../src/live_tests.rs#L406) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_vec_pretty(&wire_events).unwrap` | [406](../../src/live_tests.rs#L406) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read(&journal_path).unwrap` | [408](../../src/live_tests.rs#L408) | receiver-type-required |
| `live_kernel_turn` | `endpoint::Projector::default().reconcile(events, &reopened).unwrap` | [410](../../src/live_tests.rs#L410) | receiver-type-required |
| `live_kernel_turn` | `endpoint::Projector::default().reconcile` | [410](../../src/live_tests.rs#L410) | receiver-type-required |
| `live_kernel_turn` | `std::collections::BTreeMap::<String, Vec<String>>::new` | [415](../../src/live_tests.rs#L415) | external-constructor-callback-or-unresolved |
| `live_kernel_turn` | `batches.entry(event.string_field("attempt").unwrap().to_owned()).or_default()                 .push` | [417](../../src/live_tests.rs#L417) | receiver-type-required |
| `live_kernel_turn` | `batches.entry(event.string_field("attempt").unwrap().to_owned()).or_default` | [417](../../src/live_tests.rs#L417) | receiver-type-required |
| `live_kernel_turn` | `batches.entry` | [417](../../src/live_tests.rs#L417) | receiver-type-required |
| `live_kernel_turn` | `event.string_field("attempt").unwrap().to_owned` | [417](../../src/live_tests.rs#L417) | receiver-type-required |
| `live_kernel_turn` | `event.string_field("call").unwrap().to_owned` | [418](../../src/live_tests.rs#L418) | receiver-type-required |
| `live_kernel_turn` | `event.string_field("call").unwrap` | [418](../../src/live_tests.rs#L418) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.string_field("subkind") == Some("validation.candidate"))             .map(&#124;e&#124; serde_json::to_value(e.raw()).unwrap()).collect::<Vec<_>>` | [426](../../src/live_tests.rs#L426) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.string_field("subkind") == Some("validation.candidate"))             .map` | [426](../../src/live_tests.rs#L426) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value(e.raw()).unwrap` | [427](../../src/live_tests.rs#L427) | receiver-type-required |
| `live_kernel_turn` | `e.raw` | [427](../../src/live_tests.rs#L427) | receiver-type-required |
| `live_kernel_turn` | `candidates[0]["payload"]["binding"]["output_seq"].as_u64().unwrap` | [430](../../src/live_tests.rs#L430) | receiver-type-required |
| `live_kernel_turn` | `candidates[0]["payload"]["binding"]["output_seq"].as_u64` | [430](../../src/live_tests.rs#L430) | receiver-type-required |
| `live_kernel_turn` | `answers.iter().find(&#124;e&#124; e["seq"].as_u64() == Some(first_seq)).unwrap` | [431](../../src/live_tests.rs#L431) | receiver-type-required |
| `live_kernel_turn` | `e["seq"].as_u64` | [431](../../src/live_tests.rs#L431) | receiver-type-required |
| `live_kernel_turn` | `first["content"].as_array().unwrap().iter().filter_map(&#124;b&#124; b["text"].as_str()).collect::<String>` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `first["content"].as_array().unwrap().iter().filter_map` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `first["content"].as_array().unwrap().iter` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `first["content"].as_array().unwrap` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `first["content"].as_array` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `b["text"].as_str` | [432](../../src/live_tests.rs#L432) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find(&#124;e&#124; e.string_field("subkind") == Some("validation.feedback")).unwrap` | [434](../../src/live_tests.rs#L434) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.string_field("subkind") == Some("validation.feedback")).count` | [443](../../src/live_tests.rs#L443) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.kind() == &EventKind::Spawn).count` | [445](../../src/live_tests.rs#L445) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("apply_patch")).count` | [447](../../src/live_tests.rs#L447) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::ToolCall && event.string_field("name") == Some(tool)).filter_map(&#124;event&#124; event.string_field("call")).collect::<Vec<_>>` | [452](../../src/live_tests.rs#L452) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;event&#124; *event.kind() == schema::EventKind::ToolCall && event.string_field("name") == Some(tool)).filter_map` | [452](../../src/live_tests.rs#L452) | receiver-type-required |
| `live_kernel_turn` | `events.iter().filter(&#124;e&#124; e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("shell"))             .filter_map(&#124;e&#124; e.string_field("call")).collect::<Vec<_>>` | [456](../../src/live_tests.rs#L456) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(workspace.path().join(relative)).expect` | [467](../../src/live_tests.rs#L467) | receiver-type-required |
| `live_kernel_turn` | `std::fs::create_dir_all(destination.parent().unwrap()).unwrap` | [469](../../src/live_tests.rs#L469) | receiver-type-required |
| `live_kernel_turn` | `destination.parent().unwrap` | [469](../../src/live_tests.rs#L469) | receiver-type-required |
| `live_kernel_turn` | `destination.parent` | [469](../../src/live_tests.rs#L469) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(destination, &file).unwrap` | [470](../../src/live_tests.rs#L470) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_dir(&artifact).unwrap` | [478](../../src/live_tests.rs#L478) | receiver-type-required |
| `live_kernel_turn` | `entry.unwrap().path` | [479](../../src/live_tests.rs#L479) | receiver-type-required |
| `live_kernel_turn` | `path.extension().and_then` | [480](../../src/live_tests.rs#L480) | receiver-type-required |
| `live_kernel_turn` | `path.extension` | [480](../../src/live_tests.rs#L480) | receiver-type-required |
| `live_kernel_turn` | `path.file_name().and_then` | [480](../../src/live_tests.rs#L480) | receiver-type-required |
| `live_kernel_turn` | `path.file_name` | [480](../../src/live_tests.rs#L480) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(path).unwrap().lines` | [481](../../src/live_tests.rs#L481) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(path).unwrap` | [481](../../src/live_tests.rs#L481) | receiver-type-required |
| `live_kernel_turn` | `serde_json::from_str(line).unwrap` | [482](../../src/live_tests.rs#L482) | receiver-type-required |
| `live_kernel_turn` | `event.get("usage").is_none` | [483](../../src/live_tests.rs#L483) | receiver-type-required |
| `live_kernel_turn` | `event.get` | [483](../../src/live_tests.rs#L483) | receiver-type-required |
| `live_kernel_turn` | `events.last().unwrap().seq` | [498](../../src/live_tests.rs#L498) | receiver-type-required |
| `live_kernel_turn` | `events.last().unwrap` | [498](../../src/live_tests.rs#L498), [518](../../src/live_tests.rs#L518) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find(&#124;event&#124; event.kind() == &EventKind::Input             && event.string_field("origin_key") == Some("queued-second")).expect("second input was queued while busy").seq` | [499](../../src/live_tests.rs#L499) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find(&#124;event&#124; event.kind() == &EventKind::Input             && event.string_field("origin_key") == Some("queued-second")).expect` | [499](../../src/live_tests.rs#L499) | receiver-type-required |
| `live_kernel_turn` | `events.len` | [502](../../src/live_tests.rs#L502) | receiver-type-required |
| `live_kernel_turn` | `std::fs::read_to_string(artifact.join(format!("{name}.request.json"))).unwrap` | [505](../../src/live_tests.rs#L505) | receiver-type-required |
| `live_kernel_turn` | `ledger.projection().unwrap().events.last().unwrap().seq` | [509](../../src/live_tests.rs#L509) | receiver-type-required |
| `live_kernel_turn` | `ledger.projection().unwrap().events.last().unwrap` | [509](../../src/live_tests.rs#L509) | receiver-type-required |
| `live_kernel_turn` | `ledger.projection().unwrap().events.last` | [509](../../src/live_tests.rs#L509) | receiver-type-required |
| `live_kernel_turn` | `second.expect` | [516](../../src/live_tests.rs#L516) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value(events.last().unwrap().raw()).unwrap` | [518](../../src/live_tests.rs#L518) | receiver-type-required |
| `live_kernel_turn` | `events.last().unwrap().raw` | [518](../../src/live_tests.rs#L518) | receiver-type-required |
| `live_kernel_turn` | `terminal["validation"]["promoted_output_seq"].as_u64().unwrap` | [522](../../src/live_tests.rs#L522) | receiver-type-required |
| `live_kernel_turn` | `terminal["validation"]["promoted_output_seq"].as_u64` | [522](../../src/live_tests.rs#L522) | receiver-type-required |
| `live_kernel_turn` | `events.iter().find(&#124;event&#124; event.seq() == promoted).unwrap` | [523](../../src/live_tests.rs#L523) | receiver-type-required |
| `live_kernel_turn` | `event.seq` | [523](../../src/live_tests.rs#L523) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_value(answer.raw()).unwrap` | [524](../../src/live_tests.rs#L524) | receiver-type-required |
| `live_kernel_turn` | `answer.raw` | [524](../../src/live_tests.rs#L524) | receiver-type-required |
| `live_kernel_turn` | `raw["content"].as_array().unwrap().iter().filter_map(&#124;block&#124; block["text"].as_str()).collect::<String>` | [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `raw["content"].as_array().unwrap().iter().filter_map` | [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `raw["content"].as_array().unwrap().iter` | [525](../../src/live_tests.rs#L525) | receiver-type-required |
| `live_kernel_turn` | `projector.reconcile(&events[first_count..], &journal).unwrap` | [528](../../src/live_tests.rs#L528) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(artifact.join("queued-endpoint-events.json"), serde_json::to_vec_pretty(&second_wire).unwrap()).unwrap` | [530](../../src/live_tests.rs#L530) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_vec_pretty(&second_wire).unwrap` | [530](../../src/live_tests.rs#L530) | receiver-type-required |
| `live_kernel_turn` | `std::fs::write(artifact.join("queue-result.json"), serde_json::to_vec_pretty(&json!({             "queued_input_seq":queued,"first_settle_seq":first_settle,"second_open_seq":opened,             "second_settle_seq":terminal["seq"],"second_answer":text,"completed":true})).unwrap()).unwrap` | [531](../../src/live_tests.rs#L531) | receiver-type-required |
| `live_kernel_turn` | `serde_json::to_vec_pretty(&json!({             "queued_input_seq":queued,"first_settle_seq":first_settle,"second_open_seq":opened,             "second_settle_seq":terminal["seq"],"second_answer":text,"completed":true})).unwrap` | [531](../../src/live_tests.rs#L531) | receiver-type-required |
| `live_kernel_turn` | `service.join().unwrap().unwrap` | [536](../../src/live_tests.rs#L536) | receiver-type-required |
| `live_kernel_turn` | `service.join().unwrap` | [536](../../src/live_tests.rs#L536) | receiver-type-required |
| `live_kernel_turn` | `service.join` | [536](../../src/live_tests.rs#L536) | receiver-type-required |
| `ensure_running` | `Err` | [542](../../src/live_tests.rs#L542) | external-constructor-callback-or-unresolved |
| `ensure_running` | `live_effect_unavailable` | [542](../../src/live_tests.rs#L542) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `deliver_input` | `Err` | [543](../../src/live_tests.rs#L543) | external-constructor-callback-or-unresolved |
| `deliver_input` | `live_effect_unavailable` | [543](../../src/live_tests.rs#L543) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `interrupt` | `Err` | [544](../../src/live_tests.rs#L544) | external-constructor-callback-or-unresolved |
| `interrupt` | `live_effect_unavailable` | [544](../../src/live_tests.rs#L544) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `ensure_child` | `Err` | [545](../../src/live_tests.rs#L545) | external-constructor-callback-or-unresolved |
| `ensure_child` | `live_effect_unavailable` | [545](../../src/live_tests.rs#L545) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `deliver_report` | `Err` | [546](../../src/live_tests.rs#L546) | external-constructor-callback-or-unresolved |
| `deliver_report` | `live_effect_unavailable` | [546](../../src/live_tests.rs#L546) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `execute` | `Err` | [549](../../src/live_tests.rs#L549) | external-constructor-callback-or-unresolved |
| `execute` | `live_effect_unavailable` | [549](../../src/live_tests.rs#L549) | [tekes-worker::live_tests::live_effect_unavailable](../../src/live_tests.rs#L551) |
| `live_effect_unavailable` | `tekes_supervisor::production_tool_control::SupervisorOperationError::Unavailable` | [552](../../src/live_tests.rs#L552) | external-constructor-callback-or-unresolved |
| `live_effect_unavailable` | `"this live harness has no process mutation authority".into` | [552](../../src/live_tests.rs#L552) | receiver-type-required |
| `prepare_live_choice` | `std::env::var("TEKES_KERNEL_LIVE_PROMPT").ok().is_none_or` | [562](../../src/live_tests.rs#L562) | receiver-type-required |
| `prepare_live_choice` | `std::env::var("TEKES_KERNEL_LIVE_PROMPT").ok` | [562](../../src/live_tests.rs#L562) | receiver-type-required |
| `prepare_live_choice` | `std::env::var` | [562](../../src/live_tests.rs#L562) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `p.contains` | [562](../../src/live_tests.rs#L562) | receiver-type-required |
| `prepare_live_choice` | `prepare` | [563](../../src/live_tests.rs#L563), [585](../../src/live_tests.rs#L585) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `ledger.projection().unwrap` | [566](../../src/live_tests.rs#L566) | receiver-type-required |
| `prepare_live_choice` | `ledger.projection` | [566](../../src/live_tests.rs#L566) | receiver-type-required |
| `prepare_live_choice` | `events.iter().filter` | [568](../../src/live_tests.rs#L568) | receiver-type-required |
| `prepare_live_choice` | `events.iter` | [568](../../src/live_tests.rs#L568), [570](../../src/live_tests.rs#L570) | receiver-type-required |
| `prepare_live_choice` | `event.turn` | [568](../../src/live_tests.rs#L568) | receiver-type-required |
| `prepare_live_choice` | `Some` | [568](../../src/live_tests.rs#L568), [569](../../src/live_tests.rs#L569), [571](../../src/live_tests.rs#L571), [587](../../src/live_tests.rs#L587) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `event.kind` | [569](../../src/live_tests.rs#L569) | receiver-type-required |
| `prepare_live_choice` | `event.string_field` | [569](../../src/live_tests.rs#L569) | receiver-type-required |
| `prepare_live_choice` | `events.iter().find` | [570](../../src/live_tests.rs#L570) | receiver-type-required |
| `prepare_live_choice` | `call.kind` | [570](../../src/live_tests.rs#L570) | receiver-type-required |
| `prepare_live_choice` | `call.string_field` | [571](../../src/live_tests.rs#L571) | receiver-type-required |
| `prepare_live_choice` | `result.string_field` | [571](../../src/live_tests.rs#L571) | receiver-type-required |
| `prepare_live_choice` | `prefixes.len` | [572](../../src/live_tests.rs#L572), [578](../../src/live_tests.rs#L578) | receiver-type-required |
| `prepare_live_choice` | `serde_json::to_value(call.raw()).unwrap` | [573](../../src/live_tests.rs#L573) | receiver-type-required |
| `prepare_live_choice` | `serde_json::to_value` | [573](../../src/live_tests.rs#L573) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `call.raw` | [573](../../src/live_tests.rs#L573) | receiver-type-required |
| `prepare_live_choice` | `materialize_json(ledger, &raw["args"]).unwrap` | [574](../../src/live_tests.rs#L574) | receiver-type-required |
| `prepare_live_choice` | `materialize_json` | [574](../../src/live_tests.rs#L574) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `args["thought"].as_str().is_some_and` | [575](../../src/live_tests.rs#L575) | receiver-type-required |
| `prepare_live_choice` | `args["thought"].as_str` | [575](../../src/live_tests.rs#L575) | receiver-type-required |
| `prepare_live_choice` | `thought.starts_with` | [575](../../src/live_tests.rs#L575) | receiver-type-required |
| `prepare_live_choice` | `input.rendered_items.push` | [579](../../src/live_tests.rs#L579) | receiver-type-required |
| `prepare_live_choice` | `IJsonValue::parse(&serde_json::to_vec(&json!({"role":"user","content":[{"type":"text","text":format!("This request must call think with a new thought beginning exactly {}. Do not repeat any earlier thought.", prefixes[accepted])}]})).unwrap()).unwrap` | [579](../../src/live_tests.rs#L579) | receiver-type-required |
| `prepare_live_choice` | `IJsonValue::parse` | [579](../../src/live_tests.rs#L579) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `serde_json::to_vec(&json!({"role":"user","content":[{"type":"text","text":format!("This request must call think with a new thought beginning exactly {}. Do not repeat any earlier thought.", prefixes[accepted])}]})).unwrap` | [579](../../src/live_tests.rs#L579) | receiver-type-required |
| `prepare_live_choice` | `serde_json::to_vec` | [579](../../src/live_tests.rs#L579) | external-constructor-callback-or-unresolved |
| `prepare_live_choice` | `provider::prepare_with_tool_choice` | [587](../../src/live_tests.rs#L587) | [provider::request::prepare_with_tool_choice](../../../provider/src/request.rs#L137) |
