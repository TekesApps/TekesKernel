# session-controls — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [session-controls::METHODS](../../src/lib.rs#L41) | `pub` | not a function |
| [session-controls::MAX_GOAL_ROUNDS](../../src/lib.rs#L52) | `pub` | not a function |
| [session-controls::PHASE_ACTIVE](../../src/lib.rs#L54) | `pub` | not a function |
| [session-controls::PHASE_PAUSED](../../src/lib.rs#L55) | `pub` | not a function |
| [session-controls::PHASE_BLOCKED](../../src/lib.rs#L56) | `pub` | not a function |
| [session-controls::PHASE_COMPLETE](../../src/lib.rs#L57) | `pub` | not a function |
| [session-controls::Failure](../../src/lib.rs#L61) | `pub` | not a function |
| [session-controls::Failure::code](../../src/lib.rs#L81) | `pub` | no resolved direct caller |
| [session-controls::Failure::details](../../src/lib.rs#L95) | `pub` | no resolved direct caller |
| [session-controls::GoalRecord](../../src/lib.rs#L112) | `pub` | not a function |
| [session-controls::activation_of](../../src/lib.rs#L132) | `pub` | no resolved direct caller |
| [session-controls::activation_value](../../src/lib.rs#L143) | `pub` | no resolved direct caller |
| [session-controls::goal_view](../../src/lib.rs#L161) | `pub` | [session-controls::execute_goal](../../src/lib.rs#L245) |
| [session-controls::validate](../../src/lib.rs#L180) | `pub` | [session-controls::execute_goal](../../src/lib.rs#L245); [tekes-supervisor::client_extensions::validate_payload](../../../supervisor/src/client_extensions.rs#L2834) |
| [session-controls::execute_goal](../../src/lib.rs#L245) | `pub` | [session-controls::tests::complete_state_and_turn_count_survive_a_history_summary](../../src/lib.rs#L586); [session-controls::tests::goals_get_folds_the_model_log_without_a_bound_id](../../src/lib.rs#L611); [session-controls::tests::bound_goal_id_follows_the_record_phase](../../src/lib.rs#L642); [tekes-supervisor::client_extensions::ProductionClientExtensions::session_controls](../../../supervisor/src/client_extensions.rs#L1617); [tekes-worker::live_tests::live_kernel_turn](../../../worker/src/live_tests.rs#L38) |
| [session-controls::bound_goal_id](../../src/lib.rs#L352) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::spawn_worker_at_with_handshake_timeout](../../../supervisor/src/process_host.rs#L1998) |
| [session-controls::read_goal](../../src/lib.rs#L366) | `pub` | [tekes-supervisor::process_host::goal_continuation_due](../../../supervisor/src/process_host.rs#L5156); [tekes-worker::ledger_events::open_goal_continuation](../../../worker/src/ledger_events.rs#L69); [tekes-worker::live_tests::live_kernel_turn](../../../worker/src/live_tests.rs#L38); [tekes-worker::provider_turn::run_provider_turn_inner](../../../worker/src/provider_turn.rs#L108) |
| [session-controls::subagent_catalog](../../src/lib.rs#L516) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::session_controls](../../../supervisor/src/client_extensions.rs#L1617) |
