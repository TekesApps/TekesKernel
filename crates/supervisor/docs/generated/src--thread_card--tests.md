# tekes-supervisor::thread_card::tests

[Package atlas](index.md) · [Source](../../src/thread_card/tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-supervisor::thread_card::tests::SESSION](../../src/thread_card/tests.rs#L4) | const_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::ledger_lines](../../src/thread_card/tests.rs#L6) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::fold](../../src/thread_card/tests.rs#L30) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::is_due_claims_one_window_per_thread](../../src/thread_card/tests.rs#L55) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::doorbells_inside_the_window_converge_to_the_newest_state_without_preparing](../../src/thread_card/tests.rs#L70) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::writes_are_paced_to_the_global_cap](../../src/thread_card/tests.rs#L104) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk](../../src/thread_card/tests.rs#L139) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::children_are_folded_on_the_writer_thread_not_by_prepare](../../src/thread_card/tests.rs#L170) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::a_torn_refold_is_retried_until_the_tail_is_whole](../../src/thread_card/tests.rs#L196) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::one_governor_counts_fsyncs_for_the_writer_and_the_hydrator](../../src/thread_card/tests.rs#L225) | function_item | `private` | test;  |
| [tekes-supervisor::thread_card::tests::card_view_cost_against_fold_cost](../../src/thread_card/tests.rs#L248) | function_item | `private` | test;  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `*` | `super::*` | `private` |
| `AtomicUsize` | `std::sync::atomic::AtomicUsize` | `private` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
| `ledger_lines` | `lines.push` | [11](../../src/thread_card/tests.rs#L11) | receiver-type-required |
| `folder_with` | `tempfile::tempdir().unwrap` | [19](../../src/thread_card/tests.rs#L19) | receiver-type-required |
| `folder_with` | `tempfile::tempdir` | [19](../../src/thread_card/tests.rs#L19) | external-constructor-callback-or-unresolved |
| `folder_with` | `root.path().join` | [20](../../src/thread_card/tests.rs#L20) | receiver-type-required |
| `folder_with` | `root.path` | [20](../../src/thread_card/tests.rs#L20) | receiver-type-required |
| `folder_with` | `std::fs::create_dir(&folder).unwrap` | [21](../../src/thread_card/tests.rs#L21) | receiver-type-required |
| `folder_with` | `std::fs::create_dir` | [21](../../src/thread_card/tests.rs#L21) | external-constructor-callback-or-unresolved |
| `folder_with` | `std::fs::write(         folder.join("main.jsonl"),         ledger_lines(events).join("\n") + "\n",     )     .unwrap` | [22](../../src/thread_card/tests.rs#L22) | receiver-type-required |
| `folder_with` | `std::fs::write` | [22](../../src/thread_card/tests.rs#L22) | external-constructor-callback-or-unresolved |
| `folder_with` | `folder.join` | [23](../../src/thread_card/tests.rs#L23) | receiver-type-required |
| `folder_with` | `ledger_lines(events).join` | [24](../../src/thread_card/tests.rs#L24) | receiver-type-required |
| `folder_with` | `ledger_lines` | [24](../../src/thread_card/tests.rs#L24) | [tekes-supervisor::thread_card::tests::ledger_lines](../../src/thread_card/tests.rs#L6) |
| `fold` | `std::fs::read(folder.join("main.jsonl")).unwrap` | [31](../../src/thread_card/tests.rs#L31) | receiver-type-required |
| `fold` | `std::fs::read` | [31](../../src/thread_card/tests.rs#L31) | external-constructor-callback-or-unresolved |
| `fold` | `folder.join` | [31](../../src/thread_card/tests.rs#L31) | receiver-type-required |
| `fold` | `store::scan_valid_prefix` | [32](../../src/thread_card/tests.rs#L32) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `fold` | `scan.projection.unwrap` | [34](../../src/thread_card/tests.rs#L34) | receiver-type-required |
| `fold` | `prepare(         session,         folder.to_path_buf(),         bytes,         valid,         &projection,         None,     )     .unwrap` | [35](../../src/thread_card/tests.rs#L35) | receiver-type-required |
| `fold` | `prepare` | [35](../../src/thread_card/tests.rs#L35) | external-constructor-callback-or-unresolved |
| `fold` | `folder.to_path_buf` | [37](../../src/thread_card/tests.rs#L37) | receiver-type-required |
| `tuning` | `Duration::from_millis` | [48](../../src/thread_card/tests.rs#L48), [50](../../src/thread_card/tests.rs#L50) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `CardWriter::spawn_with` | [56](../../src/thread_card/tests.rs#L56) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `tuning` | [57](../../src/thread_card/tests.rs#L57) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `is_due_claims_one_window_per_thread` | `Arc::new` | [58](../../src/thread_card/tests.rs#L58), [59](../../src/thread_card/tests.rs#L59) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `SyncGovernor::new` | [58](../../src/thread_card/tests.rs#L58) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `std::thread::sleep` | [64](../../src/thread_card/tests.rs#L64) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `Duration::from_millis` | [64](../../src/thread_card/tests.rs#L64) | external-constructor-callback-or-unresolved |
| `is_due_claims_one_window_per_thread` | `writer.shutdown` | [66](../../src/thread_card/tests.rs#L66) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `folder_with` | [71](../../src/thread_card/tests.rs#L71) | [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `CardWriter::spawn` | [72](../../src/thread_card/tests.rs#L72) | external-constructor-callback-or-unresolved |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `tuning` | [72](../../src/thread_card/tests.rs#L72) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `writer.submit` | [74](../../src/thread_card/tests.rs#L74) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `fold` | [74](../../src/thread_card/tests.rs#L74) | [tekes-supervisor::thread_card::tests::fold](../../src/thread_card/tests.rs#L30) |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `std::fs::OpenOptions::new()             .append(true)             .open(folder.join("main.jsonl"))             .unwrap` | [78](../../src/thread_card/tests.rs#L78) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `std::fs::OpenOptions::new()             .append(true)             .open` | [78](../../src/thread_card/tests.rs#L78) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `std::fs::OpenOptions::new()             .append` | [78](../../src/thread_card/tests.rs#L78) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `std::fs::OpenOptions::new` | [78](../../src/thread_card/tests.rs#L78) | external-constructor-callback-or-unresolved |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `folder.join` | [80](../../src/thread_card/tests.rs#L80) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `file.write_all((line + "\n").as_bytes()).unwrap` | [86](../../src/thread_card/tests.rs#L86) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `file.write_all` | [86](../../src/thread_card/tests.rs#L86) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `(line + "\n").as_bytes` | [86](../../src/thread_card/tests.rs#L86) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `writer.mark_dirty` | [88](../../src/thread_card/tests.rs#L88) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `folder.clone` | [88](../../src/thread_card/tests.rs#L88) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `writer.shutdown` | [91](../../src/thread_card/tests.rs#L91) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `store::card::read_card(&folder).expect` | [92](../../src/thread_card/tests.rs#L92) | receiver-type-required |
| `doorbells_inside_the_window_converge_to_the_newest_state_without_preparing` | `store::card::read_card` | [92](../../src/thread_card/tests.rs#L92) | [store::card::read_card](../../../store/src/card.rs#L172) |
| `writes_are_paced_to_the_global_cap` | `Arc::new` | [105](../../src/thread_card/tests.rs#L105), [110](../../src/thread_card/tests.rs#L110), [111](../../src/thread_card/tests.rs#L111) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `Mutex::new` | [105](../../src/thread_card/tests.rs#L105) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `Vec::<Instant>::new` | [105](../../src/thread_card/tests.rs#L105) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `Arc::clone` | [106](../../src/thread_card/tests.rs#L106) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `CardWriter::spawn_with` | [108](../../src/thread_card/tests.rs#L108) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `tuning` | [109](../../src/thread_card/tests.rs#L109) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `writes_are_paced_to_the_global_cap` | `SyncGovernor::new` | [110](../../src/thread_card/tests.rs#L110) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `seen.lock().unwrap().push` | [112](../../src/thread_card/tests.rs#L112) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `seen.lock().unwrap` | [112](../../src/thread_card/tests.rs#L112) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `seen.lock` | [112](../../src/thread_card/tests.rs#L112) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `Instant::now` | [112](../../src/thread_card/tests.rs#L112), [122](../../src/thread_card/tests.rs#L122) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `folder_with` | [116](../../src/thread_card/tests.rs#L116) | [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) |
| `writes_are_paced_to_the_global_cap` | `fold` | [118](../../src/thread_card/tests.rs#L118) | [tekes-supervisor::thread_card::tests::fold](../../src/thread_card/tests.rs#L30) |
| `writes_are_paced_to_the_global_cap` | `writer.submit` | [120](../../src/thread_card/tests.rs#L120) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `calls.lock().unwrap().len` | [123](../../src/thread_card/tests.rs#L123) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `calls.lock().unwrap` | [123](../../src/thread_card/tests.rs#L123), [127](../../src/thread_card/tests.rs#L127) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `calls.lock` | [123](../../src/thread_card/tests.rs#L123), [127](../../src/thread_card/tests.rs#L127) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `started.elapsed` | [123](../../src/thread_card/tests.rs#L123) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `Duration::from_secs` | [123](../../src/thread_card/tests.rs#L123) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `std::thread::sleep` | [124](../../src/thread_card/tests.rs#L124) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `Duration::from_millis` | [124](../../src/thread_card/tests.rs#L124) | external-constructor-callback-or-unresolved |
| `writes_are_paced_to_the_global_cap` | `writer.shutdown` | [126](../../src/thread_card/tests.rs#L126) | receiver-type-required |
| `writes_are_paced_to_the_global_cap` | `calls.windows` | [129](../../src/thread_card/tests.rs#L129) | receiver-type-required |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `Arc::new` | [140](../../src/thread_card/tests.rs#L140), [144](../../src/thread_card/tests.rs#L144), [145](../../src/thread_card/tests.rs#L145) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `AtomicUsize::new` | [140](../../src/thread_card/tests.rs#L140) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `Arc::clone` | [141](../../src/thread_card/tests.rs#L141) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `CardWriter::spawn_with` | [142](../../src/thread_card/tests.rs#L142) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `tuning` | [143](../../src/thread_card/tests.rs#L143) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `SyncGovernor::new` | [144](../../src/thread_card/tests.rs#L144) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `std::thread::sleep` | [146](../../src/thread_card/tests.rs#L146) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `Duration::from_millis` | [146](../../src/thread_card/tests.rs#L146) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `counter.fetch_add` | [147](../../src/thread_card/tests.rs#L147) | receiver-type-required |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `folder_with` | [151](../../src/thread_card/tests.rs#L151) | [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `fold` | [153](../../src/thread_card/tests.rs#L153) | [tekes-supervisor::thread_card::tests::fold](../../src/thread_card/tests.rs#L30) |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `writer.submit` | [156](../../src/thread_card/tests.rs#L156) | receiver-type-required |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `Instant::now` | [158](../../src/thread_card/tests.rs#L158) | external-constructor-callback-or-unresolved |
| `shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk` | `writer.shutdown` | [159](../../src/thread_card/tests.rs#L159) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `folder_with` | [171](../../src/thread_card/tests.rs#L171) | [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `std::fs::write(         folder.join(format!("{child}.jsonl")),         format!(             r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"c","origin_tuple":{{"client":"cli","key":"c","op":"create","principal":"p","target":"{child}"}},"resume":"never","seq":1,"thread":"{child}","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}}"#         ) + "\n",     )     .unwrap` | [173](../../src/thread_card/tests.rs#L173) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `std::fs::write` | [173](../../src/thread_card/tests.rs#L173) | external-constructor-callback-or-unresolved |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `folder.join` | [174](../../src/thread_card/tests.rs#L174) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `fold` | [180](../../src/thread_card/tests.rs#L180) | [tekes-supervisor::thread_card::tests::fold](../../src/thread_card/tests.rs#L30) |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `CardWriter::spawn` | [182](../../src/thread_card/tests.rs#L182) | external-constructor-callback-or-unresolved |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `tuning` | [182](../../src/thread_card/tests.rs#L182) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `writer.submit` | [183](../../src/thread_card/tests.rs#L183) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `writer.shutdown` | [184](../../src/thread_card/tests.rs#L184) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `store::card::read_card(&folder).expect` | [185](../../src/thread_card/tests.rs#L185) | receiver-type-required |
| `children_are_folded_on_the_writer_thread_not_by_prepare` | `store::card::read_card` | [185](../../src/thread_card/tests.rs#L185) | [store::card::read_card](../../../store/src/card.rs#L172) |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `folder_with` | [197](../../src/thread_card/tests.rs#L197) | [tekes-supervisor::thread_card::tests::folder_with](../../src/thread_card/tests.rs#L18) |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `folder.join` | [198](../../src/thread_card/tests.rs#L198) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::fs::read(&ledger).unwrap` | [199](../../src/thread_card/tests.rs#L199) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::fs::read` | [199](../../src/thread_card/tests.rs#L199) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `whole.clone` | [200](../../src/thread_card/tests.rs#L200) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `torn.extend_from_slice` | [201](../../src/thread_card/tests.rs#L201) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::fs::write(&ledger, &torn).unwrap` | [202](../../src/thread_card/tests.rs#L202) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::fs::write` | [202](../../src/thread_card/tests.rs#L202), [213](../../src/thread_card/tests.rs#L213) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `CardWriter::spawn` | [203](../../src/thread_card/tests.rs#L203) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `tuning` | [203](../../src/thread_card/tests.rs#L203) | [tekes-supervisor::thread_card::tests::tuning](../../src/thread_card/tests.rs#L46) |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `writer.mark_dirty` | [204](../../src/thread_card/tests.rs#L204) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `folder.clone` | [204](../../src/thread_card/tests.rs#L204) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::thread::sleep` | [205](../../src/thread_card/tests.rs#L205), [216](../../src/thread_card/tests.rs#L216) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `Duration::from_millis` | [205](../../src/thread_card/tests.rs#L205), [216](../../src/thread_card/tests.rs#L216) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `done.extend_from_slice` | [209](../../src/thread_card/tests.rs#L209) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `done.push` | [212](../../src/thread_card/tests.rs#L212) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `std::fs::write(&ledger, &done).unwrap` | [213](../../src/thread_card/tests.rs#L213) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `Instant::now` | [214](../../src/thread_card/tests.rs#L214) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `writer.counts` | [215](../../src/thread_card/tests.rs#L215) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `started.elapsed` | [215](../../src/thread_card/tests.rs#L215) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `Duration::from_secs` | [215](../../src/thread_card/tests.rs#L215) | external-constructor-callback-or-unresolved |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `writer.shutdown` | [218](../../src/thread_card/tests.rs#L218) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `store::card::read_card(&folder).expect` | [219](../../src/thread_card/tests.rs#L219) | receiver-type-required |
| `a_torn_refold_is_retried_until_the_tail_is_whole` | `store::card::read_card` | [219](../../src/thread_card/tests.rs#L219) | [store::card::read_card](../../../store/src/card.rs#L172) |
| `one_governor_counts_fsyncs_for_the_writer_and_the_hydrator` | `SyncGovernor::new` | [226](../../src/thread_card/tests.rs#L226), [238](../../src/thread_card/tests.rs#L238) | external-constructor-callback-or-unresolved |
| `one_governor_counts_fsyncs_for_the_writer_and_the_hydrator` | `governor.charge` | [228](../../src/thread_card/tests.rs#L228), [231](../../src/thread_card/tests.rs#L231), [235](../../src/thread_card/tests.rs#L235) | receiver-type-required |
| `one_governor_counts_fsyncs_for_the_writer_and_the_hydrator` | `governor.delay` | [229](../../src/thread_card/tests.rs#L229) | receiver-type-required |
| `one_governor_counts_fsyncs_for_the_writer_and_the_hydrator` | `Instant::now` | [234](../../src/thread_card/tests.rs#L234) | external-constructor-callback-or-unresolved |
| `one_governor_counts_fsyncs_for_the_writer_and_the_hydrator` | `unlimited.charge` | [239](../../src/thread_card/tests.rs#L239) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `std::env::var` | [249](../../src/thread_card/tests.rs#L249) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `String::new` | [255](../../src/thread_card/tests.rs#L255) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `std::fs::read_dir(std::path::Path::new(&root).join("threads"))         .unwrap()         .flatten` | [256](../../src/thread_card/tests.rs#L256) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `std::fs::read_dir(std::path::Path::new(&root).join("threads"))         .unwrap` | [256](../../src/thread_card/tests.rs#L256) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `std::fs::read_dir` | [256](../../src/thread_card/tests.rs#L256) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `std::path::Path::new(&root).join` | [256](../../src/thread_card/tests.rs#L256) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `std::path::Path::new` | [256](../../src/thread_card/tests.rs#L256) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `entry.path` | [260](../../src/thread_card/tests.rs#L260) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `std::fs::read` | [261](../../src/thread_card/tests.rs#L261) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `folder.join` | [261](../../src/thread_card/tests.rs#L261) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `Instant::now` | [264](../../src/thread_card/tests.rs#L264), [272](../../src/thread_card/tests.rs#L272) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `store::scan_valid_prefix` | [265](../../src/thread_card/tests.rs#L265) | [store::tail::scan_valid_prefix](../../../store/src/tail.rs#L37) |
| `card_view_cost_against_fold_cost` | `started.elapsed` | [266](../../src/thread_card/tests.rs#L266), [274](../../src/thread_card/tests.rs#L274) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `usize::try_from(scan.valid_bytes).unwrap` | [267](../../src/thread_card/tests.rs#L267) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `usize::try_from` | [267](../../src/thread_card/tests.rs#L267) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `entry.file_name().to_string_lossy().into_owned` | [271](../../src/thread_card/tests.rs#L271) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `entry.file_name().to_string_lossy` | [271](../../src/thread_card/tests.rs#L271) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `entry.file_name` | [271](../../src/thread_card/tests.rs#L271) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `prepare` | [273](../../src/thread_card/tests.rs#L273) | external-constructor-callback-or-unresolved |
| `card_view_cost_against_fold_cost` | `folder.clone` | [273](../../src/thread_card/tests.rs#L273) | receiver-type-required |
| `card_view_cost_against_fold_cost` | `projection.events.len` | [278](../../src/thread_card/tests.rs#L278), [280](../../src/thread_card/tests.rs#L280) | receiver-type-required |
