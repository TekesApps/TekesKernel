# tekes-worker::validation_tests

[Package atlas](index.md) · [Source](../../src/validation_tests.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_writer_rejects_nonfinal_and_foreign_candidates_without_writes](../../src/validation_tests.rs#L34) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_candidate_and_decision_survive_reopen_without_second_writes](../../src/validation_tests.rs#L48) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_decision_rejects_model_output_as_control_signal](../../src/validation_tests.rs#L114) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_obligation_survives_resume_never_without_reopening_settled_turn](../../src/validation_tests.rs#L128) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::long_provider_reasoning_and_answer_survive_spill_and_reopen](../../src/validation_tests.rs#L146) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap](../../src/validation_tests.rs#L170) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::durable_final_settles_without_requiring_a_still_available_provider](../../src/validation_tests.rs#L229) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::direct_final_recovery_settles_once_without_validator](../../src/validation_tests.rs#L251) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::settlement_recovery_rejects_a_foreign_candidate_binding_without_writes](../../src/validation_tests.rs#L289) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::failed_decision_recovery_delivers_feedback_once_to_the_same_worker](../../src/validation_tests.rs#L308) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::stop_during_validator_launch_does_not_settle_as_internal_failure](../../src/validation_tests.rs#L346) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink](../../src/validation_tests.rs#L381) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validator_profile_writes_only_to_private_scratch](../../src/validation_tests.rs#L397) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::queued_retry_body_waits_for_validation_settlement_then_opens_once](../../src/validation_tests.rs#L417) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::first_root_input_opens_after_run_start_without_prior_settlement](../../src/validation_tests.rs#L449) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::queued_input_replays_after_previous_tool_result_not_at_storage_position](../../src/validation_tests.rs#L465) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::deepseek_request_error_is_visible_and_never_becomes_a_final_candidate](../../src/validation_tests.rs#L494) | function_item | `private` | test;  |
| [tekes-worker::validation_tests::validator_mandate_counts_verify_attempts_not_progress_outputs](../../src/validation_tests.rs#L513) | function_item | `private` | test;  |

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
| `validation_test_candidate` | `append_test_attempt` | [6](../../src/validation_tests.rs#L6) | external-constructor-callback-or-unresolved |
| `validation_test_candidate` | `ledger.next_seq` | [7](../../src/validation_tests.rs#L7) | receiver-type-required |
| `validation_test_candidate` | `ledger         .append_contract(             make_event(json!({                 "v":1,"seq":seq,"turn":2,"kind":"output","ts":"2026-08-27T09:00:02.000Z",                 "attempt":attempt,"content":[{"type":"text","text":"candidate"}],                 "final_answer":final_answer,"usage":usage_object(None),                 "sealed":{"version":1,"adapter":"responses","fragments":"[]"}             }))             .unwrap(),             BarrierContext::default(),         )         .unwrap` | [8](../../src/validation_tests.rs#L8) | receiver-type-required |
| `validation_test_candidate` | `ledger         .append_contract` | [8](../../src/validation_tests.rs#L8) | receiver-type-required |
| `validation_test_candidate` | `make_event(json!({                 "v":1,"seq":seq,"turn":2,"kind":"output","ts":"2026-08-27T09:00:02.000Z",                 "attempt":attempt,"content":[{"type":"text","text":"candidate"}],                 "final_answer":final_answer,"usage":usage_object(None),                 "sealed":{"version":1,"adapter":"responses","fragments":"[]"}             }))             .unwrap` | [10](../../src/validation_tests.rs#L10) | receiver-type-required |
| `validation_test_candidate` | `make_event` | [10](../../src/validation_tests.rs#L10) | external-constructor-callback-or-unresolved |
| `validation_test_candidate` | `BarrierContext::default` | [17](../../src/validation_tests.rs#L17) | external-constructor-callback-or-unresolved |
| `validation_test_candidate` | `ledger.projection().unwrap().events[0]         .string_field("thread")         .unwrap()         .to_owned` | [20](../../src/validation_tests.rs#L20) | receiver-type-required |
| `validation_test_candidate` | `ledger.projection().unwrap().events[0]         .string_field("thread")         .unwrap` | [20](../../src/validation_tests.rs#L20) | receiver-type-required |
| `validation_test_candidate` | `ledger.projection().unwrap().events[0]         .string_field` | [20](../../src/validation_tests.rs#L20) | receiver-type-required |
| `validation_test_candidate` | `ledger.projection().unwrap` | [20](../../src/validation_tests.rs#L20) | receiver-type-required |
| `validation_test_candidate` | `ledger.projection` | [20](../../src/validation_tests.rs#L20) | receiver-type-required |
| `validation_test_candidate` | `thread.clone` | [25](../../src/validation_tests.rs#L25) | receiver-type-required |
| `validation_test_candidate` | `Default::default` | [29](../../src/validation_tests.rs#L29) | external-constructor-callback-or-unresolved |
| `validation_writer_rejects_nonfinal_and_foreign_candidates_without_writes` | `context_ledger` | [35](../../src/validation_tests.rs#L35) | external-constructor-callback-or-unresolved |
| `validation_writer_rejects_nonfinal_and_foreign_candidates_without_writes` | `validation_test_candidate` | [36](../../src/validation_tests.rs#L36), [40](../../src/validation_tests.rs#L40) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validation_writer_rejects_nonfinal_and_foreign_candidates_without_writes` | `ledger.next_seq` | [37](../../src/validation_tests.rs#L37), [42](../../src/validation_tests.rs#L42) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `context_ledger` | [49](../../src/validation_tests.rs#L49) | external-constructor-callback-or-unresolved |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `validation_test_candidate` | [50](../../src/validation_tests.rs#L50) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [52](../../src/validation_tests.rs#L52) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::begin_validation` | [52](../../src/validation_tests.rs#L52) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [54](../../src/validation_tests.rs#L54) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::commit_validation_decision` | [54](../../src/validation_tests.rs#L54) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `ledger.next_seq` | [55](../../src/validation_tests.rs#L55) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `ledger.path().to_owned` | [56](../../src/validation_tests.rs#L56) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `ledger.path` | [56](../../src/validation_tests.rs#L56) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `drop` | [57](../../src/validation_tests.rs#L57) | external-constructor-callback-or-unresolved |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `LockedLedger::open(&path, 1).unwrap` | [58](../../src/validation_tests.rs#L58) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `LockedLedger::open` | [58](../../src/validation_tests.rs#L58) | external-constructor-callback-or-unresolved |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `serde_json::to_value(reopened.projection().unwrap().events.last().unwrap().raw()).unwrap` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `serde_json::to_value` | [69](../../src/validation_tests.rs#L69) | external-constructor-callback-or-unresolved |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.projection().unwrap().events.last().unwrap().raw` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.projection().unwrap().events.last().unwrap` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.projection().unwrap().events.last` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.projection().unwrap` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.projection` | [69](../../src/validation_tests.rs#L69) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `binding.clone` | [75](../../src/validation_tests.rs#L75) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `changed         .snapshot         .insert` | [76](../../src/validation_tests.rs#L76) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `"artifact".into` | [78](../../src/validation_tests.rs#L78) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `"different".into` | [78](../../src/validation_tests.rs#L78) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap()         .events         .iter()         .find(&#124;event&#124; event.seq() == binding.output_seq)         .unwrap()         .canonical_bytes()         .unwrap` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap()         .events         .iter()         .find(&#124;event&#124; event.seq() == binding.output_seq)         .unwrap()         .canonical_bytes` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap()         .events         .iter()         .find(&#124;event&#124; event.seq() == binding.output_seq)         .unwrap` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap()         .events         .iter()         .find` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap()         .events         .iter` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection()         .unwrap` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened         .projection` | [81](../../src/validation_tests.rs#L81) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `event.seq` | [86](../../src/validation_tests.rs#L86) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::materialize_validation_settlement(&mut reopened, timestamp, decision).unwrap` | [91](../../src/validation_tests.rs#L91) | receiver-type-required |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `engine::materialize_validation_settlement` | [91](../../src/validation_tests.rs#L91) | [engine::validation_writer::materialize_validation_settlement](../../../engine/src/validation_writer.rs#L248) |
| `validation_candidate_and_decision_survive_reopen_without_second_writes` | `reopened.next_seq` | [93](../../src/validation_tests.rs#L93) | receiver-type-required |
| `validation_decision_rejects_model_output_as_control_signal` | `context_ledger` | [115](../../src/validation_tests.rs#L115) | external-constructor-callback-or-unresolved |
| `validation_decision_rejects_model_output_as_control_signal` | `validation_test_candidate` | [116](../../src/validation_tests.rs#L116) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validation_decision_rejects_model_output_as_control_signal` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [118](../../src/validation_tests.rs#L118) | receiver-type-required |
| `validation_decision_rejects_model_output_as_control_signal` | `engine::begin_validation` | [118](../../src/validation_tests.rs#L118) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `validation_decision_rejects_model_output_as_control_signal` | `ledger.next_seq` | [119](../../src/validation_tests.rs#L119) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `context_ledger` | [129](../../src/validation_tests.rs#L129) | external-constructor-callback-or-unresolved |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `validation_test_candidate` | [131](../../src/validation_tests.rs#L131) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `ledger.path().to_owned` | [134](../../src/validation_tests.rs#L134) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `ledger.path` | [134](../../src/validation_tests.rs#L134) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `drop` | [135](../../src/validation_tests.rs#L135) | external-constructor-callback-or-unresolved |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `LockedLedger::open(&path, 1).unwrap` | [136](../../src/validation_tests.rs#L136) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `LockedLedger::open` | [136](../../src/validation_tests.rs#L136) | external-constructor-callback-or-unresolved |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [139](../../src/validation_tests.rs#L139) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::begin_validation` | [139](../../src/validation_tests.rs#L139) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [140](../../src/validation_tests.rs#L140) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::commit_validation_decision` | [140](../../src/validation_tests.rs#L140) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::materialize_validation_settlement(&mut ledger, timestamp, decision).unwrap` | [141](../../src/validation_tests.rs#L141) | receiver-type-required |
| `validation_obligation_survives_resume_never_without_reopening_settled_turn` | `engine::materialize_validation_settlement` | [141](../../src/validation_tests.rs#L141) | [engine::validation_writer::materialize_validation_settlement](../../../engine/src/validation_writer.rs#L248) |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `context_ledger` | [147](../../src/validation_tests.rs#L147) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `test_options` | [148](../../src/validation_tests.rs#L148) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `append_test_attempt` | [149](../../src/validation_tests.rs#L149) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `"long model text ".repeat` | [150](../../src/validation_tests.rs#L150) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `provider::normalize_response(provider::AdapterId::Responses,         &serde_json::to_vec(&json!({"text":text,"reasoningData":text,"isFinalAnswer":true,"functionCalls":[]})).unwrap()).unwrap` | [151](../../src/validation_tests.rs#L151) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `provider::normalize_response` | [151](../../src/validation_tests.rs#L151) | [provider::normalize::normalize_response](../../../provider/src/normalize.rs#L182) |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `serde_json::to_vec(&json!({"text":text,"reasoningData":text,"isFinalAnswer":true,"functionCalls":[]})).unwrap` | [152](../../src/validation_tests.rs#L152) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `serde_json::to_vec` | [152](../../src/validation_tests.rs#L152) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `append_terminal(&mut ledger, TerminalAppend { options:&options, manifest:&BuiltinManifest::compiled(),         eager:&EagerDispatch::default(), dialect:DialectId::OpenaiResponsesV1,         server_managed:false, turn:2, attempt:"long-response" }, terminal).unwrap` | [153](../../src/validation_tests.rs#L153) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `append_terminal` | [153](../../src/validation_tests.rs#L153) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `BuiltinManifest::compiled` | [153](../../src/validation_tests.rs#L153) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `EagerDispatch::default` | [154](../../src/validation_tests.rs#L154) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `ledger.path().to_owned` | [156](../../src/validation_tests.rs#L156) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `ledger.path` | [156](../../src/validation_tests.rs#L156) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `drop` | [157](../../src/validation_tests.rs#L157) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `LockedLedger::open(path,1).unwrap` | [158](../../src/validation_tests.rs#L158) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `LockedLedger::open` | [158](../../src/validation_tests.rs#L158) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `ledger.projection().unwrap` | [159](../../src/validation_tests.rs#L159) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `ledger.projection` | [159](../../src/validation_tests.rs#L159) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.iter().rev().find(&#124;e&#124; e.kind()==&EventKind::Reasoning).unwrap` | [160](../../src/validation_tests.rs#L160) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.iter().rev().find` | [160](../../src/validation_tests.rs#L160) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.iter().rev` | [160](../../src/validation_tests.rs#L160) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.iter` | [160](../../src/validation_tests.rs#L160) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `e.kind` | [160](../../src/validation_tests.rs#L160) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `serde_json::to_value(reasoning.raw()).unwrap` | [161](../../src/validation_tests.rs#L161) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `serde_json::to_value` | [161](../../src/validation_tests.rs#L161), [164](../../src/validation_tests.rs#L164) | external-constructor-callback-or-unresolved |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `reasoning.raw` | [161](../../src/validation_tests.rs#L161) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `serde_json::to_value(events.last().unwrap().raw()).unwrap` | [164](../../src/validation_tests.rs#L164) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.last().unwrap().raw` | [164](../../src/validation_tests.rs#L164) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.last().unwrap` | [164](../../src/validation_tests.rs#L164) | receiver-type-required |
| `long_provider_reasoning_and_answer_survive_spill_and_reopen` | `events.last` | [164](../../src/validation_tests.rs#L164) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `context_ledger` | [171](../../src/validation_tests.rs#L171) | external-constructor-callback-or-unresolved |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `validation_test_candidate` | [173](../../src/validation_tests.rs#L173), [201](../../src/validation_tests.rs#L201) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `binding         .snapshot         .insert` | [174](../../src/validation_tests.rs#L174) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `"proof.txt".into` | [176](../../src/validation_tests.rs#L176) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `"sha256-draft".into` | [176](../../src/validation_tests.rs#L176) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [177](../../src/validation_tests.rs#L177) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::begin_validation` | [177](../../src/validation_tests.rs#L177), [203](../../src/validation_tests.rs#L203) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [179](../../src/validation_tests.rs#L179) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::commit_validation_decision` | [179](../../src/validation_tests.rs#L179), [187](../../src/validation_tests.rs#L187), [204](../../src/validation_tests.rs#L204), [213](../../src/validation_tests.rs#L213) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.next_seq` | [181](../../src/validation_tests.rs#L181), [195](../../src/validation_tests.rs#L195), [208](../../src/validation_tests.rs#L208) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.append(make_event(json!({"v":1,"seq":verdict_seq,"turn":2,"kind":"state",         "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",         "payload":{"candidate_seq":candidate,"verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong content"],"guidance":"fix content"}]}     })).unwrap(), true).unwrap` | [182](../../src/validation_tests.rs#L182) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.append` | [182](../../src/validation_tests.rs#L182), [209](../../src/validation_tests.rs#L209) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `make_event(json!({"v":1,"seq":verdict_seq,"turn":2,"kind":"state",         "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",         "payload":{"candidate_seq":candidate,"verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong content"],"guidance":"fix content"}]}     })).unwrap` | [182](../../src/validation_tests.rs#L182) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `make_event` | [182](../../src/validation_tests.rs#L182), [209](../../src/validation_tests.rs#L209) | external-constructor-callback-or-unresolved |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, verdict_seq).unwrap` | [187](../../src/validation_tests.rs#L187) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw()).unwrap` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `serde_json::to_value` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | external-constructor-callback-or-unresolved |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.projection().unwrap().events.last().unwrap().raw` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.projection().unwrap().events.last().unwrap` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.projection().unwrap().events.last` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.projection().unwrap` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.projection` | [189](../../src/validation_tests.rs#L189), [205](../../src/validation_tests.rs#L205), [215](../../src/validation_tests.rs#L215), [223](../../src/validation_tests.rs#L223) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.path().to_owned` | [192](../../src/validation_tests.rs#L192) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.path` | [192](../../src/validation_tests.rs#L192) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `drop` | [193](../../src/validation_tests.rs#L193) | external-constructor-callback-or-unresolved |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `LockedLedger::open(path, 1).unwrap` | [194](../../src/validation_tests.rs#L194) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `LockedLedger::open` | [194](../../src/validation_tests.rs#L194) | external-constructor-callback-or-unresolved |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::begin_validation(&mut ledger, timestamp, &later).unwrap` | [203](../../src/validation_tests.rs#L203) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate2, candidate2).unwrap` | [204](../../src/validation_tests.rs#L204) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `ledger.append(make_event(json!({"v":1,"seq":second_verdict,"turn":2,"kind":"state",         "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",         "payload":{"candidate_seq":candidate2,"verdict":"fail","failures":[]}     })).unwrap(), true).unwrap` | [209](../../src/validation_tests.rs#L209) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `make_event(json!({"v":1,"seq":second_verdict,"turn":2,"kind":"state",         "ts":timestamp,"visibility":"runtime","subkind":"validation.verdict",         "payload":{"candidate_seq":candidate2,"verdict":"fail","failures":[]}     })).unwrap` | [209](../../src/validation_tests.rs#L209) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate2, second_verdict).unwrap` | [213](../../src/validation_tests.rs#L213) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::materialize_validation_settlement(&mut ledger, timestamp, terminal).unwrap` | [221](../../src/validation_tests.rs#L221) | receiver-type-required |
| `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap` | `engine::materialize_validation_settlement` | [221](../../src/validation_tests.rs#L221) | [engine::validation_writer::materialize_validation_settlement](../../../engine/src/validation_writer.rs#L248) |
| `durable_final_settles_without_requiring_a_still_available_provider` | `context_ledger` | [230](../../src/validation_tests.rs#L230) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `validation_test_candidate` | [231](../../src/validation_tests.rs#L231) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `durable_final_settles_without_requiring_a_still_available_provider` | `tool_profile` | [232](../../src/validation_tests.rs#L232) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `directory.path` | [232](../../src/validation_tests.rs#L232) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `test_options` | [234](../../src/validation_tests.rs#L234) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `std::iter::empty` | [236](../../src/validation_tests.rs#L236) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `Vec::new` | [237](../../src/validation_tests.rs#L237) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `run_provider_turn(&mut ledger, &options, &profile, &Selected { version: 2 },         &mut credential, &mut lines, &mut output, &RuntimeCancellation::default()).unwrap` | [238](../../src/validation_tests.rs#L238) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `run_provider_turn` | [238](../../src/validation_tests.rs#L238) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `RuntimeCancellation::default` | [239](../../src/validation_tests.rs#L239) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `ledger.projection().unwrap` | [240](../../src/validation_tests.rs#L240) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `ledger.projection` | [240](../../src/validation_tests.rs#L240) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `serde_json::to_value(events.last().unwrap().raw()).unwrap` | [241](../../src/validation_tests.rs#L241) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `serde_json::to_value` | [241](../../src/validation_tests.rs#L241) | external-constructor-callback-or-unresolved |
| `durable_final_settles_without_requiring_a_still_available_provider` | `events.last().unwrap().raw` | [241](../../src/validation_tests.rs#L241) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `events.last().unwrap` | [241](../../src/validation_tests.rs#L241) | receiver-type-required |
| `durable_final_settles_without_requiring_a_still_available_provider` | `events.last` | [241](../../src/validation_tests.rs#L241) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `context_ledger` | [252](../../src/validation_tests.rs#L252) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `validation_test_candidate` | [253](../../src/validation_tests.rs#L253) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `direct_final_recovery_settles_once_without_validator` | `ledger.next_seq` | [255](../../src/validation_tests.rs#L255), [277](../../src/validation_tests.rs#L277) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `ledger.append_contract(make_event(json!({"v":1,"seq":input_seq,"kind":"input",         "ts":timestamp,"content":[{"type":"text","text":"next"}],         "origin_key":"direct-final-queued","origin_tuple":{"principal":"test",         "client":"test","target":binding.thread,"op":"submit",         "key":"direct-final-queued"}})).unwrap(), BarrierContext::default()).unwrap` | [256](../../src/validation_tests.rs#L256) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `ledger.append_contract` | [256](../../src/validation_tests.rs#L256) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `make_event(json!({"v":1,"seq":input_seq,"kind":"input",         "ts":timestamp,"content":[{"type":"text","text":"next"}],         "origin_key":"direct-final-queued","origin_tuple":{"principal":"test",         "client":"test","target":binding.thread,"op":"submit",         "key":"direct-final-queued"}})).unwrap` | [256](../../src/validation_tests.rs#L256) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `make_event` | [256](../../src/validation_tests.rs#L256) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `BarrierContext::default` | [260](../../src/validation_tests.rs#L260) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `ledger.path().to_owned` | [262](../../src/validation_tests.rs#L262) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `ledger.path` | [262](../../src/validation_tests.rs#L262) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `drop` | [263](../../src/validation_tests.rs#L263), [278](../../src/validation_tests.rs#L278), [282](../../src/validation_tests.rs#L282) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `LockedLedger::open(&path, 1).unwrap` | [264](../../src/validation_tests.rs#L264), [279](../../src/validation_tests.rs#L279), [283](../../src/validation_tests.rs#L283) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `LockedLedger::open` | [264](../../src/validation_tests.rs#L264), [279](../../src/validation_tests.rs#L279), [283](../../src/validation_tests.rs#L283) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `tool_profile` | [265](../../src/validation_tests.rs#L265) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `directory.path` | [265](../../src/validation_tests.rs#L265) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `test_options` | [266](../../src/validation_tests.rs#L266) | external-constructor-callback-or-unresolved |
| `direct_final_recovery_settles_once_without_validator` | `ledger.projection().unwrap` | [271](../../src/validation_tests.rs#L271) | receiver-type-required |
| `direct_final_recovery_settles_once_without_validator` | `ledger.projection` | [271](../../src/validation_tests.rs#L271) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `context_ledger` | [290](../../src/validation_tests.rs#L290) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `validation_test_candidate` | [291](../../src/validation_tests.rs#L291) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [293](../../src/validation_tests.rs#L293) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `engine::begin_validation` | [293](../../src/validation_tests.rs#L293) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [294](../../src/validation_tests.rs#L294) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `engine::commit_validation_decision` | [294](../../src/validation_tests.rs#L294) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `ledger.path().to_owned` | [295](../../src/validation_tests.rs#L295) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `ledger.path` | [295](../../src/validation_tests.rs#L295) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `drop` | [296](../../src/validation_tests.rs#L296) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `fs::read_to_string(&path).unwrap` | [297](../../src/validation_tests.rs#L297) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `fs::read_to_string` | [297](../../src/validation_tests.rs#L297) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `text.lines().map(&#124;line&#124; serde_json::from_str::<Value>(line).unwrap()).collect::<Vec<_>>` | [298](../../src/validation_tests.rs#L298) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `text.lines().map` | [298](../../src/validation_tests.rs#L298) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `text.lines` | [298](../../src/validation_tests.rs#L298) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `serde_json::from_str::<Value>(line).unwrap` | [298](../../src/validation_tests.rs#L298) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `serde_json::from_str::<Value>` | [298](../../src/validation_tests.rs#L298) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `records.iter_mut().find(&#124;e&#124; e["seq"] == candidate).unwrap` | [299](../../src/validation_tests.rs#L299) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `records.iter_mut().find` | [299](../../src/validation_tests.rs#L299) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `records.iter_mut` | [299](../../src/validation_tests.rs#L299) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `records.iter().fold` | [300](../../src/validation_tests.rs#L300) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `records.iter` | [300](../../src/validation_tests.rs#L300) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `String::new` | [300](../../src/validation_tests.rs#L300) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `acc.push_str` | [300](../../src/validation_tests.rs#L300) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `serde_json_canonicalizer::to_string(e).unwrap` | [300](../../src/validation_tests.rs#L300) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `serde_json_canonicalizer::to_string` | [300](../../src/validation_tests.rs#L300) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `acc.push` | [300](../../src/validation_tests.rs#L300) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `fs::write(&path, &changed).unwrap` | [301](../../src/validation_tests.rs#L301) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `fs::write` | [301](../../src/validation_tests.rs#L301) | external-constructor-callback-or-unresolved |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `LockedLedger::open(&path, 1).unwrap` | [302](../../src/validation_tests.rs#L302) | receiver-type-required |
| `settlement_recovery_rejects_a_foreign_candidate_binding_without_writes` | `LockedLedger::open` | [302](../../src/validation_tests.rs#L302) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `context_ledger` | [309](../../src/validation_tests.rs#L309) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `validation_test_candidate` | [310](../../src/validation_tests.rs#L310) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `binding.snapshot.insert` | [311](../../src/validation_tests.rs#L311) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `"proof.txt".into` | [311](../../src/validation_tests.rs#L311) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `"sha256-fixture".into` | [311](../../src/validation_tests.rs#L311) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [313](../../src/validation_tests.rs#L313) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `engine::begin_validation` | [313](../../src/validation_tests.rs#L313) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [314](../../src/validation_tests.rs#L314) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `engine::commit_validation_decision` | [314](../../src/validation_tests.rs#L314), [319](../../src/validation_tests.rs#L319) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.next_seq` | [315](../../src/validation_tests.rs#L315) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.append(make_event(json!({"v":1,"seq":source,"turn":2,"kind":"state","ts":timestamp,         "subkind":"validation.verdict","visibility":"runtime","payload":{"candidate_seq":candidate,         "verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong contents"],"guidance":["repair contents"]}]}})).unwrap(),true).unwrap` | [316](../../src/validation_tests.rs#L316) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.append` | [316](../../src/validation_tests.rs#L316) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `make_event(json!({"v":1,"seq":source,"turn":2,"kind":"state","ts":timestamp,         "subkind":"validation.verdict","visibility":"runtime","payload":{"candidate_seq":candidate,         "verdict":"fail","failures":[{"id":"proof.txt","issues":["wrong contents"],"guidance":["repair contents"]}]}})).unwrap` | [316](../../src/validation_tests.rs#L316) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `make_event` | [316](../../src/validation_tests.rs#L316) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `engine::commit_validation_decision(&mut ledger,timestamp,candidate,source).unwrap` | [319](../../src/validation_tests.rs#L319) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.path().to_owned` | [320](../../src/validation_tests.rs#L320) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.path` | [320](../../src/validation_tests.rs#L320) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `drop` | [321](../../src/validation_tests.rs#L321), [329](../../src/validation_tests.rs#L329) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `LockedLedger::open(&path,1).unwrap` | [322](../../src/validation_tests.rs#L322), [330](../../src/validation_tests.rs#L330) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `LockedLedger::open` | [322](../../src/validation_tests.rs#L322), [330](../../src/validation_tests.rs#L330) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `tool_profile` | [323](../../src/validation_tests.rs#L323) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `directory.path` | [323](../../src/validation_tests.rs#L323) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `test_options` | [324](../../src/validation_tests.rs#L324) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.projection().unwrap` | [332](../../src/validation_tests.rs#L332) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `ledger.projection` | [332](../../src/validation_tests.rs#L332) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `events.iter().filter(&#124;e&#124; e.string_field("subkind")==Some("validation.feedback")).collect::<Vec<_>>` | [333](../../src/validation_tests.rs#L333) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `events.iter().filter` | [333](../../src/validation_tests.rs#L333) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `events.iter` | [333](../../src/validation_tests.rs#L333) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `e.string_field` | [333](../../src/validation_tests.rs#L333) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `Some` | [333](../../src/validation_tests.rs#L333) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `serde_json::to_value(feedback[0].raw()).unwrap` | [335](../../src/validation_tests.rs#L335) | receiver-type-required |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `serde_json::to_value` | [335](../../src/validation_tests.rs#L335) | external-constructor-callback-or-unresolved |
| `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` | `feedback[0].raw` | [335](../../src/validation_tests.rs#L335) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `context_ledger` | [347](../../src/validation_tests.rs#L347) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `validation_test_candidate` | [348](../../src/validation_tests.rs#L348) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `directory.path().join` | [349](../../src/validation_tests.rs#L349) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `directory.path` | [349](../../src/validation_tests.rs#L349), [353](../../src/validation_tests.rs#L353) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `fs::write(&artifact,b"draft").unwrap` | [350](../../src/validation_tests.rs#L350) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `fs::write` | [350](../../src/validation_tests.rs#L350) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `binding.snapshot.insert` | [351](../../src/validation_tests.rs#L351) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `artifact.to_string_lossy().into_owned` | [351](../../src/validation_tests.rs#L351) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `artifact.to_string_lossy` | [351](../../src/validation_tests.rs#L351) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `engine::begin_validation(&mut ledger,"2026-09-04T00:00:00.000Z",&binding).unwrap` | [352](../../src/validation_tests.rs#L352) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `engine::begin_validation` | [352](../../src/validation_tests.rs#L352) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `tool_profile` | [353](../../src/validation_tests.rs#L353) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `test_options` | [354](../../src/validation_tests.rs#L354) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `RuntimeCancellation::default` | [355](../../src/validation_tests.rs#L355) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `std::iter::once` | [357](../../src/validation_tests.rs#L357) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `Ok` | [357](../../src/validation_tests.rs#L357) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `stop.to_owned` | [357](../../src/validation_tests.rs#L357) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `run_provider_turn(&mut ledger,&options,&profile,&Selected{version:2},&mut None,         &mut lines,&mut Vec::new(),&cancellation).unwrap` | [358](../../src/validation_tests.rs#L358) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `run_provider_turn` | [358](../../src/validation_tests.rs#L358) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `Vec::new` | [359](../../src/validation_tests.rs#L359), [367](../../src/validation_tests.rs#L367) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `sync_channel` | [364](../../src/validation_tests.rs#L364) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `drop` | [365](../../src/validation_tests.rs#L365) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `cancellation.clone` | [366](../../src/validation_tests.rs#L366) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `drain_ready_deliveries(&mut ledger,&options,&Selected{version:2},&mut control,&mut output).unwrap` | [368](../../src/validation_tests.rs#L368) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `drain_ready_deliveries` | [368](../../src/validation_tests.rs#L368) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `post_turn_exit(&mut ledger,&options,&cancellation,&mut output).unwrap` | [369](../../src/validation_tests.rs#L369) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `post_turn_exit` | [369](../../src/validation_tests.rs#L369) | external-constructor-callback-or-unresolved |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `ledger.projection().unwrap` | [371](../../src/validation_tests.rs#L371) | receiver-type-required |
| `stop_during_validator_launch_does_not_settle_as_internal_failure` | `ledger.projection` | [371](../../src/validation_tests.rs#L371) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `tempfile::tempdir().unwrap` | [382](../../src/validation_tests.rs#L382) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `tempfile::tempdir` | [382](../../src/validation_tests.rs#L382) | external-constructor-callback-or-unresolved |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `tool_profile` | [383](../../src/validation_tests.rs#L383) | external-constructor-callback-or-unresolved |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `directory.path` | [383](../../src/validation_tests.rs#L383), [384](../../src/validation_tests.rs#L384), [387](../../src/validation_tests.rs#L387), [391](../../src/validation_tests.rs#L391) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `directory.path().join` | [384](../../src/validation_tests.rs#L384), [387](../../src/validation_tests.rs#L387), [391](../../src/validation_tests.rs#L391) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `fs::write(&file,b"frozen contents").unwrap` | [385](../../src/validation_tests.rs#L385) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `fs::write` | [385](../../src/validation_tests.rs#L385) | external-constructor-callback-or-unresolved |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `std::os::unix::fs::symlink(&file,&link).unwrap` | [388](../../src/validation_tests.rs#L388) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `std::os::unix::fs::symlink` | [388](../../src/validation_tests.rs#L388) | external-constructor-callback-or-unresolved |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `fs::File::create(directory.path().join("large")).unwrap` | [391](../../src/validation_tests.rs#L391) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `fs::File::create` | [391](../../src/validation_tests.rs#L391) | external-constructor-callback-or-unresolved |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `large.set_len(16 * 1024 * 1024 + 1).unwrap` | [392](../../src/validation_tests.rs#L392) | receiver-type-required |
| `validation_snapshot_read_uses_bounded_regular_file_and_rejects_symlink` | `large.set_len` | [392](../../src/validation_tests.rs#L392) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `tempfile::tempdir().unwrap` | [398](../../src/validation_tests.rs#L398) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `tempfile::tempdir` | [398](../../src/validation_tests.rs#L398) | external-constructor-callback-or-unresolved |
| `validator_profile_writes_only_to_private_scratch` | `directory.path().join` | [399](../../src/validation_tests.rs#L399), [400](../../src/validation_tests.rs#L400), [401](../../src/validation_tests.rs#L401) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `directory.path` | [399](../../src/validation_tests.rs#L399), [400](../../src/validation_tests.rs#L400), [401](../../src/validation_tests.rs#L401) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `fs::create_dir_all(&source).unwrap` | [402](../../src/validation_tests.rs#L402) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `fs::create_dir_all` | [402](../../src/validation_tests.rs#L402), [403](../../src/validation_tests.rs#L403), [404](../../src/validation_tests.rs#L404) | external-constructor-callback-or-unresolved |
| `validator_profile_writes_only_to_private_scratch` | `fs::create_dir_all(&scratch).unwrap` | [403](../../src/validation_tests.rs#L403) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `fs::create_dir_all(&snapshot).unwrap` | [404](../../src/validation_tests.rs#L404) | receiver-type-required |
| `validator_profile_writes_only_to_private_scratch` | `tool_profile` | [405](../../src/validation_tests.rs#L405) | external-constructor-callback-or-unresolved |
| `validator_profile_writes_only_to_private_scratch` | `validation_runtime::isolate_validator_profile` | [407](../../src/validation_tests.rs#L407) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `context_ledger` | [418](../../src/validation_tests.rs#L418) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `validation_test_candidate` | [420](../../src/validation_tests.rs#L420) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.next_seq` | [421](../../src/validation_tests.rs#L421), [426](../../src/validation_tests.rs#L426), [441](../../src/validation_tests.rs#L441) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.append_contract(make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,         "content":[{"type":"text","text":"重试"}],"origin_key":"queued-retry-body",         "origin_tuple":{"principal":"test","client":"test","target":binding.thread,             "op":"submit","key":"queued-retry-body"}})).unwrap(), BarrierContext::default()).unwrap` | [422](../../src/validation_tests.rs#L422) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.append_contract` | [422](../../src/validation_tests.rs#L422) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,         "content":[{"type":"text","text":"重试"}],"origin_key":"queued-retry-body",         "origin_tuple":{"principal":"test","client":"test","target":binding.thread,             "op":"submit","key":"queued-retry-body"}})).unwrap` | [422](../../src/validation_tests.rs#L422) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `make_event` | [422](../../src/validation_tests.rs#L422) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `BarrierContext::default` | [425](../../src/validation_tests.rs#L425) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::begin_validation(&mut ledger, timestamp, &binding).unwrap` | [429](../../src/validation_tests.rs#L429) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::begin_validation` | [429](../../src/validation_tests.rs#L429) | [engine::validation_writer::begin_validation](../../../engine/src/validation_writer.rs#L25) |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::commit_validation_decision(&mut ledger, timestamp, candidate, candidate).unwrap` | [430](../../src/validation_tests.rs#L430) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::commit_validation_decision` | [430](../../src/validation_tests.rs#L430) | [engine::validation_writer::commit_validation_decision](../../../engine/src/validation_writer.rs#L92) |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.path().to_owned` | [433](../../src/validation_tests.rs#L433) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.path` | [433](../../src/validation_tests.rs#L433) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `drop` | [434](../../src/validation_tests.rs#L434) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `LockedLedger::open(path, 1).unwrap` | [435](../../src/validation_tests.rs#L435) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `LockedLedger::open` | [435](../../src/validation_tests.rs#L435) | external-constructor-callback-or-unresolved |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::materialize_validation_settlement(&mut ledger, timestamp, decision).unwrap` | [437](../../src/validation_tests.rs#L437) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `engine::materialize_validation_settlement` | [437](../../src/validation_tests.rs#L437) | [engine::validation_writer::materialize_validation_settlement](../../../engine/src/validation_writer.rs#L248) |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.projection().unwrap` | [444](../../src/validation_tests.rs#L444) | receiver-type-required |
| `queued_retry_body_waits_for_validation_settlement_then_opens_once` | `ledger.projection` | [444](../../src/validation_tests.rs#L444) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `context_ledger` | [450](../../src/validation_tests.rs#L450) | external-constructor-callback-or-unresolved |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `ledger.path().to_owned` | [451](../../src/validation_tests.rs#L451) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `ledger.path` | [451](../../src/validation_tests.rs#L451) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `drop` | [452](../../src/validation_tests.rs#L452) | external-constructor-callback-or-unresolved |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::read_to_string(&path).unwrap().lines().take(3)         .fold` | [453](../../src/validation_tests.rs#L453) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::read_to_string(&path).unwrap().lines().take` | [453](../../src/validation_tests.rs#L453) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::read_to_string(&path).unwrap().lines` | [453](../../src/validation_tests.rs#L453) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::read_to_string(&path).unwrap` | [453](../../src/validation_tests.rs#L453) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::read_to_string` | [453](../../src/validation_tests.rs#L453) | external-constructor-callback-or-unresolved |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `String::new` | [454](../../src/validation_tests.rs#L454) | external-constructor-callback-or-unresolved |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `acc.push_str` | [454](../../src/validation_tests.rs#L454) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::write(&path, prefix).unwrap` | [455](../../src/validation_tests.rs#L455) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `fs::write` | [455](../../src/validation_tests.rs#L455) | external-constructor-callback-or-unresolved |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `LockedLedger::open(&path, 1).unwrap` | [456](../../src/validation_tests.rs#L456) | receiver-type-required |
| `first_root_input_opens_after_run_start_without_prior_settlement` | `LockedLedger::open` | [456](../../src/validation_tests.rs#L456) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `context_ledger` | [466](../../src/validation_tests.rs#L466) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `append_test_attempt` | [468](../../src/validation_tests.rs#L468) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `append_test_call` | [469](../../src/validation_tests.rs#L469) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `ledger.next_seq` | [470](../../src/validation_tests.rs#L470), [474](../../src/validation_tests.rs#L474), [478](../../src/validation_tests.rs#L478) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `ledger.append_contract(make_event(json!({"v":1,"seq":seq,"kind":"output","turn":2,"ts":timestamp,         "attempt":"queued-replay","content":[],"final_answer":false,"usage":usage_object(None),         "sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[{\"type\":\"function_call\",\"call_id\":\"queued-read\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}]"}})).unwrap(), BarrierContext::default()).unwrap` | [471](../../src/validation_tests.rs#L471) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `ledger.append_contract` | [471](../../src/validation_tests.rs#L471), [475](../../src/validation_tests.rs#L475), [479](../../src/validation_tests.rs#L479) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `make_event(json!({"v":1,"seq":seq,"kind":"output","turn":2,"ts":timestamp,         "attempt":"queued-replay","content":[],"final_answer":false,"usage":usage_object(None),         "sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[{\"type\":\"function_call\",\"call_id\":\"queued-read\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}]"}})).unwrap` | [471](../../src/validation_tests.rs#L471) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `make_event` | [471](../../src/validation_tests.rs#L471), [475](../../src/validation_tests.rs#L475), [479](../../src/validation_tests.rs#L479) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `BarrierContext::default` | [473](../../src/validation_tests.rs#L473), [477](../../src/validation_tests.rs#L477), [480](../../src/validation_tests.rs#L480) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `ledger.append_contract(make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,         "content":[{"type":"text","text":"QUEUED_BODY_BOUNDARY"}],"origin_key":"queued-replay",         "origin_tuple":{"principal":"test","client":"test","target":"test","op":"submit","key":"queued-replay"}})).unwrap(), BarrierContext::default()).unwrap` | [475](../../src/validation_tests.rs#L475) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `make_event(json!({"v":1,"seq":input,"kind":"input","ts":timestamp,         "content":[{"type":"text","text":"QUEUED_BODY_BOUNDARY"}],"origin_key":"queued-replay",         "origin_tuple":{"principal":"test","client":"test","target":"test","op":"submit","key":"queued-replay"}})).unwrap` | [475](../../src/validation_tests.rs#L475) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `ledger.append_contract(make_event(json!({"v":1,"seq":result,"kind":"tool_result","turn":2,"ts":timestamp,         "call":"queued-read","outcome":"ok","content":[{"type":"text","text":"RESULT_BOUNDARY"}]})).unwrap(), BarrierContext::default()).unwrap` | [479](../../src/validation_tests.rs#L479) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `make_event(json!({"v":1,"seq":result,"kind":"tool_result","turn":2,"ts":timestamp,         "call":"queued-read","outcome":"ok","content":[{"type":"text","text":"RESULT_BOUNDARY"}]})).unwrap` | [479](../../src/validation_tests.rs#L479) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 2).unwrap` | [482](../../src/validation_tests.rs#L482) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `project_provider_context` | [482](../../src/validation_tests.rs#L482), [486](../../src/validation_tests.rs#L486) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `append_settle(&mut ledger, timestamp, 2, "completed", None).unwrap` | [484](../../src/validation_tests.rs#L484) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `append_settle` | [484](../../src/validation_tests.rs#L484) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 3).unwrap` | [486](../../src/validation_tests.rs#L486) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `context.items.iter().map(&#124;item&#124; serde_json::to_string(item).unwrap()).collect::<Vec<_>>` | [487](../../src/validation_tests.rs#L487) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `context.items.iter().map` | [487](../../src/validation_tests.rs#L487) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `context.items.iter` | [487](../../src/validation_tests.rs#L487) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `serde_json::to_string(item).unwrap` | [487](../../src/validation_tests.rs#L487) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `serde_json::to_string` | [487](../../src/validation_tests.rs#L487) | external-constructor-callback-or-unresolved |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `rendered.iter().position(&#124;item&#124; item.contains("RESULT_BOUNDARY")).unwrap` | [488](../../src/validation_tests.rs#L488) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `rendered.iter().position` | [488](../../src/validation_tests.rs#L488), [489](../../src/validation_tests.rs#L489) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `rendered.iter` | [488](../../src/validation_tests.rs#L488), [489](../../src/validation_tests.rs#L489) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `item.contains` | [488](../../src/validation_tests.rs#L488), [489](../../src/validation_tests.rs#L489) | receiver-type-required |
| `queued_input_replays_after_previous_tool_result_not_at_storage_position` | `rendered.iter().position(&#124;item&#124; item.contains("QUEUED_BODY_BOUNDARY")).unwrap` | [489](../../src/validation_tests.rs#L489) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `context_ledger` | [495](../../src/validation_tests.rs#L495) | external-constructor-callback-or-unresolved |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `test_options` | [496](../../src/validation_tests.rs#L496) | external-constructor-callback-or-unresolved |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `append_test_attempt` | [497](../../src/validation_tests.rs#L497) | external-constructor-callback-or-unresolved |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.next_seq` | [498](../../src/validation_tests.rs#L498) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `provider::normalize_dialect_response(DialectId::DeepseekResponsesV1,         br#"{"error":{"code":"invalid_request_error","message":"No tool output found for tool call fixture."}}"#).unwrap` | [499](../../src/validation_tests.rs#L499) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `provider::normalize_dialect_response` | [499](../../src/validation_tests.rs#L499) | [provider::normalize::normalize_dialect_response](../../../provider/src/normalize.rs#L216) |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `append_provider_terminal_error(&mut ledger, &options, 2, "request-rejection", &terminal).unwrap` | [501](../../src/validation_tests.rs#L501) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `append_provider_terminal_error` | [501](../../src/validation_tests.rs#L501) | external-constructor-callback-or-unresolved |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.projection().unwrap().events.iter().filter(&#124;event&#124; event.seq() >= before).collect::<Vec<_>>` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.projection().unwrap().events.iter().filter` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.projection().unwrap().events.iter` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.projection().unwrap` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `ledger.projection` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `event.seq` | [504](../../src/validation_tests.rs#L504) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `new.iter().find(&#124;event&#124; event.kind() == &EventKind::Error).unwrap` | [506](../../src/validation_tests.rs#L506) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `new.iter().find` | [506](../../src/validation_tests.rs#L506) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `new.iter` | [506](../../src/validation_tests.rs#L506) | receiver-type-required |
| `deepseek_request_error_is_visible_and_never_becomes_a_final_candidate` | `event.kind` | [506](../../src/validation_tests.rs#L506) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `context_ledger` | [514](../../src/validation_tests.rs#L514) | external-constructor-callback-or-unresolved |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `validation_test_candidate` | [517](../../src/validation_tests.rs#L517) | [tekes-worker::validation_tests::validation_test_candidate](../../src/validation_tests.rs#L1) |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.path().to_owned` | [518](../../src/validation_tests.rs#L518) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.path` | [518](../../src/validation_tests.rs#L518) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `drop` | [519](../../src/validation_tests.rs#L519) | external-constructor-callback-or-unresolved |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `LockedLedger::open(&path, 1).unwrap` | [520](../../src/validation_tests.rs#L520) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `LockedLedger::open` | [520](../../src/validation_tests.rs#L520) | external-constructor-callback-or-unresolved |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.projection().unwrap` | [521](../../src/validation_tests.rs#L521) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.projection` | [521](../../src/validation_tests.rs#L521) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.next_seq` | [526](../../src/validation_tests.rs#L526) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.append(make_event(json!({"v":1,"seq":seq,"turn":2,"kind":"tool_call",             "ts":"2026-08-27T09:00:02.000Z","call":format!("verify-{count}"),             "name":"verify","attempt":"validation-test","source":"provider",             "args":{"covered_set":[],"verdict":"inconclusive","failures":[]}})).unwrap(), true).unwrap` | [527](../../src/validation_tests.rs#L527) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `ledger.append` | [527](../../src/validation_tests.rs#L527) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `make_event(json!({"v":1,"seq":seq,"turn":2,"kind":"tool_call",             "ts":"2026-08-27T09:00:02.000Z","call":format!("verify-{count}"),             "name":"verify","attempt":"validation-test","source":"provider",             "args":{"covered_set":[],"verdict":"inconclusive","failures":[]}})).unwrap` | [527](../../src/validation_tests.rs#L527) | receiver-type-required |
| `validator_mandate_counts_verify_attempts_not_progress_outputs` | `make_event` | [527](../../src/validation_tests.rs#L527) | external-constructor-callback-or-unresolved |
