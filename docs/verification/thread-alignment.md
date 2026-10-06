# Thread semantics, documentation and code alignment

[Verification entry point](README.md) · [Current thread definition](../concepts/thread.md)

This record preserves the preceding A-alignment work and its 106 test results. Old chapter numbers in the tables below trace the changes made at that time;
the bodies were subsequently split and moved. See the [reorganization record](document-reorganization.md) for current locations.
Moving these documents does not claim those runtime tests were rerun.

[Layered reading entry point](../README.md) · [Thread definition A](../concepts/thread.md#thread-fundamentals-a) · [Execution flow](../flows/turn.md#turn-execution-and-validation)

## Baseline and conclusions

The baseline is A, as supplemented, corrected and explicitly reconfirmed in the discussion, then written into chapter 01. Review date: 2026-09-04.
The scope covers thread identity, data/files, execution contexts and roles, turns, the sink, candidates,
validation and repair, and settlement. It is not a renewed acceptance review of all product protocols, features or release qualifications.

The discrepancies confirmed in this task were in explanatory documentation: simplified terminology, the old validation topology, the old judge seed description,
confusion between UI settlement and liveness, and historical findings mistaken for current status. The corrections are listed below.
The corresponding code paths and existing tests support A. No Rust behavior discrepancy requiring a change was found in this review,
so conforming implementations were not rewritten for alignment. General tool progress remains optional and incompletely implemented.

## Rules → contracts → implementation → evidence

The A-numbers here are tracking labels for this review, not new protocol fields. Search the linked source files for the test names.

| Rule | Contract / definition | Implementation | Evidence from this run |
|---|---|---|---|
| A-01: a thread has a stable identity, main/child JSONL logs and assets; auxiliary protocol records and caches are distinct | [Thread layout / rules](../data/storage.md#layout), [event](../../spec/event.md) | `create_thread_with_assets` in [store/folder](../../crates/store/src/folder.rs); [store/asset](../../crates/store/src/asset.rs); `ensure_validator` in [worker validation](../../crates/worker/src/validation_runtime.rs) | Source review confirmed directory creation, the main log, flat child logs and seeds; Slice-1 create-retry / torn-tail tests passed |
| A-02: files are persistent execution logs; contexts are recoverable, not one file per process | [Thread terminology](../concepts/thread.md#terminology-and-boundaries), [tail lifecycle](../../spec/tail-lifecycle.md) | [store/tail](../../crates/store/src/tail.rs), [worker](../../crates/worker/src/main.rs), [schema/fold](../../crates/schema/src/fold.rs) | Worker tests `validation_candidate_and_decision_survive_reopen_without_second_writes` and `validation_obligation_survives_resume_never_without_reopening_settled_turn` passed |
| A-03: executor and validator are different roles supported by the same worker program | [Thread definition](../concepts/thread.md#terminology-and-boundaries), [Turn flow](../flows/turn.md#root-executor-and-independent-validator) | `activate_validator_profile`, `ensure_validator` and `advance` in [validation runtime](../../crates/worker/src/validation_runtime.rs) | `validator_role_requires_the_exact_host_spawn_not_just_a_candidate_reference` passed; main, ordinary child and validator branches were inspected |
| A-04: a thread may have multiple turns; repair stays in the original turn and queued input cannot bypass settlement | [event turn_open](../../spec/event.md), [provider runtime](../../spec/provider-runtime.md) | `open_ready_turn` in [worker](../../crates/worker/src/ledger_events.rs); input admission and settled-turn checks in [schema/fold](../../crates/schema/src/fold.rs) | `queued_retry_body_waits_for_validation_settlement_then_opens_once`, `first_root_input_opens_after_run_start_without_prior_settlement`, and `queued_input_replays_after_previous_tool_result_not_at_storage_position` passed |
| A-05: the sink records tool calls/results and nonfinal messages; status is projected and progress is optional | [tool runtime](../../spec/tool-runtime.md), [Context projection](../interfaces/client.md#process-output-and-final-delivery) | [engine/tool](../../crates/engine/src/tool.rs), [session-endpoint §Journal projection](../../crates/endpoint/src/projection.rs), `Inbound::Notification` in [MCP transport](../../crates/mcp/src/transport.rs) | Worker tool/nonfinal-message loop tests and endpoint projection tests passed; progress was inspected in source only, without claiming complete delivery support |
| A-06: final_answer marks a candidate, not turn completion | [provider runtime](../../spec/provider-runtime.md), [session-endpoint §Journal projection](../../spec/session-endpoint.md#journal-projection) | [provider normalize](../../crates/provider/src/normalize.rs), `run_provider_turn_inner` in [worker](../../crates/worker/src/provider_turn.rs), [validation advance](../../crates/worker/src/validation_runtime.rs) | Six provider finality tests, worker `production_turn_continues_through_tools_commentary_and_reasoning_until_final`, and endpoint `worker_final_and_validation_feedback_do_not_end_the_client_turn` passed |
| A-07: settlement is required even without artifact validation; an independent validator runs when needed | [provider runtime](../../spec/provider-runtime.md), [Turn flow](../flows/turn.md#turn-execution-and-validation) | `validation_decision` in [engine validation](../../crates/engine/src/validation.rs); [writer](../../crates/engine/src/validation_writer.rs) | `candidate_requires_validation_even_when_no_judge_is_needed` and `durable_final_settles_without_requiring_a_still_available_provider` passed |
| A-08: fail feeds back to the original context for one repair; a second negative result or validator death yields inconclusive | [provider runtime](../../spec/provider-runtime.md) | [engine decision](../../crates/engine/src/validation.rs), [worker feedback](../../crates/worker/src/validation_runtime.rs) | `fail_repairs_original_worker_once_and_never_becomes_pass_at_cap`, `direct_inconclusive_has_no_repair_round_but_death_does`, and `failed_decision_recovery_delivers_feedback_once_to_the_same_worker` passed |
| A-09: validation binds an exact candidate and snapshot; a new answer cannot reuse an old verdict | [provider runtime](../../spec/provider-runtime.md), [event](../../spec/event.md) | [ValidationBinding](../../crates/engine/src/validation.rs), [validation writer](../../crates/engine/src/validation_writer.rs), [judge](../../crates/worker/src/validation_runtime.rs) | `validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap`, rejection of nonfinal/foreign candidates, and snapshot path/version tests passed |
| A-10: each turn has exactly one settlement; completed does not mean pass | [event settle](../../spec/event.md#settle-barrier-exactly-one-per-turn) | [schema event](../../crates/schema/src/event.rs), [schema fold](../../crates/schema/src/fold.rs), [materialize_validation_settlement](../../crates/engine/src/validation_writer.rs) | Schema validation-settlement tests and worker tests for settlement idempotency after reopening and foreign-binding rejection passed |
| A-11: durable final state, process exit and parked state are distinct facts | [tail lifecycle](../../spec/tail-lifecycle.md), [Event settlement](../data/durability.md#settlement) | `classify` in [engine lifecycle](../../crates/engine/src/lifecycle.rs); [worker](../../crates/worker/src/main.rs) | Slice-1 completed-turn, run-mode, slow-hold, busy-worker and stop/recovery tests passed; worker stop and error branch tests passed |
| A-12: only the candidate referenced by root settlement is final delivery; child completion does not end the root turn | [session-endpoint §Journal projection](../../spec/session-endpoint.md#journal-projection), [Turn flow](../flows/turn.md#turn-execution-and-validation) | `Output` / `Settle` branches in [session-endpoint §Journal projection](../../crates/endpoint/src/projection.rs); [validation advance](../../crates/worker/src/validation_runtime.rs) | Four endpoint projection tests passed, covering candidates/repairs without premature turn completion and exact answer references; no real Client UI was run |

## Documentation discrepancies corrected

| Location | Original problem | Resolution |
|---|---|---|
| README, docs home | Entry points jumped straight into code, with a duplicate sequential list of over 20 documents; root README retained old V2 chapter descriptions | Organized chapters 00–21 into five layers; root README links to that single reading entry point |
| 00, 01, D-5 explanation | Thread/session/run were conflated and lacked a unified A definition | Chapter 01 defines terminology and all aspects; 00 uses execution logs and contexts; D-5 retains its original wording with an added explanation |
| 02 | Output was called "settled model output" without explaining candidates; UI was broadly described as depending on process-gone | Distinguished provider response, candidate, turn final state and liveness; clarified that Client follows turn/end |
| 07, 09 | Validation was described as an optional spawn/wait pattern with an outer parent launching an executor each time | Documented the root executor's built-in validation path, independent validator, repair in the original context, attempt cap and root settlement |
| 05 | Default validator was described as a generic selector copying input/output/tool results | Matched `ensure_validator`: a host-authored judge seed containing the request, candidate, frozen artifacts and exact binding |
| 05, endpoint spec | Boundaries between progress messages, candidate attributes and final delivery were unclear | Clarified channel reuse, sessionFinal as a candidate attribute, and promotion through the settle reference; wire names remain unchanged |
| 02, 07 | Every spawn was described as caused by a model tool_call | Separated model delegation from host-owned validation correlation without fabricating model tool calls |
| 14, historical audit | Original gates or early migration gaps could be mistaken for a complete current status report | Added scope and links to current alignment; retained early findings as history without upgrading live acceptance claims |
| Spec navigation | Topic contracts lacked a central entry point | Added an index covering 35 contracts, distinguishing current V3 from reused V2 internal protocols |

The remaining chapters, 03/04/06/08/11/12/13/15/16/17/18/19/20/21, retained their topic responsibilities with added reading layers and
parent entry points. Current explanations of A point back to 01/07/05. Historical chapters and original reviews were not mechanically rewritten as current rules.
Source indexes remain in the implementation layer for reference as needed, without displacing the introductory reading path.

## Validation performed

The following commands ran against the uncommitted worktree at that time; no real provider was called and no product UI was installed or operated.

| Command | Result |
|---|---|
| `cargo test --locked -p engine validation --lib` | 6 passed |
| `cargo test --locked -p tekes-worker --bin tekes-worker -- --skip live_kernel_turn` | 70 passed; one real-provider test explicitly excluded |
| `cargo test --locked -p provider --test turn_finality` | 6 passed |
| `cargo test --locked -p schema --test validation_settlement` | 1 passed |
| `cargo test --locked -p endpoint --lib projection::tests` | 4 passed |
| `cargo test --locked -p conformance --test slice1_gates` | 19 passed |

A total of 106 tests passed. Existing tests covered A's main boundaries; this task added no tests that merely mirror the implementation.
Directories, roles and message routing were also traced in source; the conclusions do not extend beyond the rules listed on this page.

## Documentation and index validation

| Check | Result |
|---|---|
| Reading entry coverage | All 22 numbered documents assigned to the five-layer reading path or background; all 35 existing specs classified in the topic index |
| New navigation links | 121 new local links/anchors passed checks |
| All current docs file links | `scripts/code-architecture.py --links` passed, including source line ranges; historical review paths were excluded from current-link requirements |
| Mermaid | Mermaid 11.12.0 parsed 264 diagrams without syntax errors, including the new root-turn validation flowchart |
| Source index | `scripts/code-architecture.py --check` passed after refreshing against the worktree |
| Whitespace / patch checks | `git diff --check -- README.md docs spec/provider-runtime.md spec/session-endpoint.md#journal-projection` passed |

Refreshed index input fingerprint: `ae0ece106aaa4e1cf0225c5556a20bde1dcc7d86029778d2ae0d44b314d20f06`.
The index includes existing worktree changes; its fingerprint is not a commit identity containing only this task's code.
This alignment task made no Git commit and did not mark previously incomplete real-provider, product-installation or Client UI acceptance as passed.

## Rules for future changes

When a discrepancy appears, first identify whether it is a conflicting definition, outdated explanation, implementation bug or missing evidence.
Confirmed domain rules must not be rewritten to accommodate code; passing tests cannot replace missing real-runtime acceptance.
When changing a rule within A's scope, update its authoritative contract, related explanations, implementation and necessary tests, and record the result here.
Historical audits need not be rewritten as the latest results, and existing protocol fields need not change merely to unify names.
