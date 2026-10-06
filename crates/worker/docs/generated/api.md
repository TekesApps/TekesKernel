# tekes-worker — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [tekes-worker::child_agents::ChildSpawnExecution](../../src/child_agents.rs#L3) | `pub(crate)` | not a function |
| [tekes-worker::child_agents::DurableChildSpawn](../../src/child_agents.rs#L16) | `pub(crate)` | not a function |
| [tekes-worker::child_agents::ChildTerminal](../../src/child_agents.rs#L24) | `pub(crate)` | not a function |
| [tekes-worker::child_agents::execute_child_spawn_tool](../../src/child_agents.rs#L29) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::child_agents::ensure_durable_child](../../src/child_agents.rs#L213) | `pub(crate)` | [tekes-worker::child_agents::execute_child_spawn_tool](../../src/child_agents.rs#L29) |
| [tekes-worker::child_agents::ensure_delegation_state](../../src/child_agents.rs#L334) | `pub(crate)` | [tekes-worker::child_agents::ensure_durable_child](../../src/child_agents.rs#L213) |
| [tekes-worker::child_agents::TaskInputError](../../src/child_agents.rs#L381) | `pub(crate)` | not a function |
| [tekes-worker::child_agents::resolve_completed_task_inputs](../../src/child_agents.rs#L393) | `pub(crate)` | [tekes-worker::child_agents::ensure_delegation_state](../../src/child_agents.rs#L334) |
| [tekes-worker::child_agents::child_identity](../../src/child_agents.rs#L468) | `pub(crate)` | [tekes-worker::child_agents::ensure_durable_child](../../src/child_agents.rs#L213) |
| [tekes-worker::child_agents::publish_child_genesis](../../src/child_agents.rs#L509) | `pub(crate)` | [tekes-worker::child_agents::ensure_durable_child](../../src/child_agents.rs#L213) |
| [tekes-worker::child_agents::exchange_launch_child_runtime](../../src/child_agents.rs#L543) | `pub(crate)` | [tekes-worker::child_agents::execute_child_spawn_tool](../../src/child_agents.rs#L29) |
| [tekes-worker::child_agents::wait_for_child_terminal](../../src/child_agents.rs#L593) | `pub(crate)` | [tekes-worker::child_agents::execute_child_spawn_tool](../../src/child_agents.rs#L29) |
| [tekes-worker::child_agents::read_child_projection](../../src/child_agents.rs#L642) | `pub(crate)` | [tekes-worker::child_agents::ensure_durable_child](../../src/child_agents.rs#L213); [tekes-worker::child_agents::wait_for_child_terminal](../../src/child_agents.rs#L593) |
| [tekes-worker::child_agents::append_child_result_once](../../src/child_agents.rs#L702) | `pub(crate)` | [tekes-worker::child_agents::execute_child_spawn_tool](../../src/child_agents.rs#L29) |
| [tekes-worker::compaction::CONTEXT_RENDERER_VERSION](../../src/compaction.rs#L3) | `pub(crate)` | not a function |
| [tekes-worker::compaction::auto_compact_for_turn](../../src/compaction.rs#L5) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::compaction::build_compaction_event](../../src/compaction.rs#L25) | `pub(crate)` | [tekes-worker::compaction::auto_compact_for_turn](../../src/compaction.rs#L5) |
| [tekes-worker::compaction::run_compaction_summary](../../src/compaction.rs#L70) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::compaction::preflight_compaction_due](../../src/compaction.rs#L137) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::compaction::observed_prefix_token_bound](../../src/compaction.rs#L150) | `pub(crate)` | [tekes-worker::compaction::preflight_compaction_due](../../src/compaction.rs#L137) |
| [tekes-worker::compaction::prefix_preserving_token_bound](../../src/compaction.rs#L194) | `pub(crate)` | [tekes-worker::compaction::observed_prefix_token_bound](../../src/compaction.rs#L150) |
| [tekes-worker::context_projection::dynamic_catalog_revision](../../src/context_projection.rs#L4) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::native_search_references](../../src/context_projection.rs#L20) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104) | `pub(crate)` | [tekes-worker::context_projection::project_provider_context](../../src/context_projection.rs#L94) |
| [tekes-worker::context_projection::seed_provider_items](../../src/context_projection.rs#L451) | `pub(crate)` | [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104) |
| [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L595) | `pub(crate)` | [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104) |
| [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L680) | `pub(crate)` | [tekes-worker::context_projection::render_event_item](../../src/context_projection.rs#L595) |
| [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L731) | `pub(crate)` | [tekes-worker::context_projection::native_search_references](../../src/context_projection.rs#L20); [tekes-worker::context_projection::render_blocks](../../src/context_projection.rs#L680); [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L759); [tekes-worker::context_projection::append_tool_result_trim](../../src/context_projection.rs#L853) |
| [tekes-worker::context_projection::materialize_json_at](../../src/context_projection.rs#L738) | `pub(crate)` | [tekes-worker::context_projection::materialize_json](../../src/context_projection.rs#L731) |
| [tekes-worker::context_projection::materialize_string](../../src/context_projection.rs#L759) | `pub(crate)` | [tekes-worker::context_projection::project_provider_context_mode](../../src/context_projection.rs#L104); [tekes-worker::context_projection::render_sealed_carrier](../../src/context_projection.rs#L568) |
| [tekes-worker::context_projection::effective_system](../../src/context_projection.rs#L786) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::EpochAppend](../../src/context_projection.rs#L798) | `pub(crate)` | not a function |
| [tekes-worker::context_projection::epoch_open_reason](../../src/context_projection.rs#L813) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::append_tool_result_trim](../../src/context_projection.rs#L853) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::append_provider_epoch](../../src/context_projection.rs#L893) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::latest_compatible_epoch](../../src/context_projection.rs#L939) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::context_projection::ranges](../../src/context_projection.rs#L974) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::continuation::ContinuationFacts](../../src/continuation.rs#L5) | `pub(crate)` | not a function |
| [tekes-worker::continuation::continuation_facts](../../src/continuation.rs#L22) | `pub(crate)` | [tekes-worker::continuation::wait_tool_continuation](../../src/continuation.rs#L157) |
| [tekes-worker::continuation::CONTINUATION_INPUT_SCOPE](../../src/continuation.rs#L130) | `pub(crate)` | not a function |
| [tekes-worker::continuation::rfc3339_after](../../src/continuation.rs#L137) | `pub(crate)` | [tekes-worker::continuation::wait_tool_continuation](../../src/continuation.rs#L157) |
| [tekes-worker::continuation::ContinuationWait](../../src/continuation.rs#L142) | `pub(crate)` | not a function |
| [tekes-worker::continuation::wait_tool_continuation](../../src/continuation.rs#L157) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::CANCEL_STOP](../../src/control_channel.rs#L4) | `pub(crate)` | not a function |
| [tekes-worker::control_channel::CANCEL_SUPERVISOR_LOSS](../../src/control_channel.rs#L5) | `pub(crate)` | not a function |
| [tekes-worker::control_channel::RuntimeCancellation](../../src/control_channel.rs#L8) | `pub(crate)` | not a function |
| [tekes-worker::control_channel::RuntimeCancellation::cancel](../../src/control_channel.rs#L23) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::stop_requested](../../src/control_channel.rs#L31) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::supervisor_lost](../../src/control_channel.rs#L35) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::mark_protocol_failed](../../src/control_channel.rs#L39) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::protocol_failed](../../src/control_channel.rs#L43) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::defer](../../src/control_channel.rs#L47) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::park_delivery](../../src/control_channel.rs#L54) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::RuntimeCancellation::take_parked_deliveries](../../src/control_channel.rs#L61) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::ControlLines](../../src/control_channel.rs#L66) | `pub(crate)` | not a function |
| [tekes-worker::control_channel::ControlLines::drain_ready](../../src/control_channel.rs#L75) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::ProtocolFailure](../../src/control_channel.rs#L109) | `pub(crate)` | not a function |
| [tekes-worker::control_channel::spawn_control_reader](../../src/control_channel.rs#L119) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L137) | `pub(crate)` | [tekes-worker::control_channel::read_control_lines](../../src/control_channel.rs#L132) |
| [tekes-worker::control_channel::control_line_key](../../src/control_channel.rs#L162) | `pub(crate)` | [tekes-worker::control_channel::is_stop_control_line](../../src/control_channel.rs#L170) |
| [tekes-worker::control_channel::is_delivery_control_line](../../src/control_channel.rs#L176) | `pub(crate)` | [tekes-worker::control_channel::forward_control_lines](../../src/control_channel.rs#L137) |
| [tekes-worker::delivery::drain_ready_deliveries](../../src/delivery.rs#L10) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::delivery::drain_parked_deliveries](../../src/delivery.rs#L31) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::delivery::handle_delivery](../../src/delivery.rs#L52) | `pub(crate)` | [tekes-worker::delivery::drain_ready_deliveries](../../src/delivery.rs#L10); [tekes-worker::delivery::drain_parked_deliveries](../../src/delivery.rs#L31) |
| [tekes-worker::delivery::post_turn_exit](../../src/delivery.rs#L190) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::identity::selected](../../src/identity.rs#L7) | `pub(super)` | [tekes-worker::identity::resolve](../../src/identity.rs#L41) |
| [tekes-worker::identity::resolve](../../src/identity.rs#L41) | `pub(super)` | no resolved direct caller |
| [tekes-worker::ledger_events::append_run_start](../../src/ledger_events.rs#L3) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::open_ready_turn](../../src/ledger_events.rs#L42) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::open_goal_continuation](../../src/ledger_events.rs#L69) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::goal_store_location](../../src/ledger_events.rs#L122) | `pub(crate)` | [tekes-worker::ledger_events::open_goal_continuation](../../src/ledger_events.rs#L69) |
| [tekes-worker::ledger_events::append_turn_open](../../src/ledger_events.rs#L136) | `pub(crate)` | [tekes-worker::ledger_events::open_ready_turn](../../src/ledger_events.rs#L42) |
| [tekes-worker::ledger_events::origin_event](../../src/ledger_events.rs#L160) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::append_and_receipt](../../src/ledger_events.rs#L179) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::announce_appended](../../src/ledger_events.rs#L216) | `pub(crate)` | [tekes-worker::ledger_events::append_and_receipt](../../src/ledger_events.rs#L179) |
| [tekes-worker::ledger_events::event_at](../../src/ledger_events.rs#L228) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::event_json](../../src/ledger_events.rs#L235) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::ledger_events::make_event](../../src/ledger_events.rs#L239) | `pub(crate)` | [tekes-worker::ledger_events::append_turn_open](../../src/ledger_events.rs#L136); [tekes-worker::ledger_events::append_and_receipt](../../src/ledger_events.rs#L179); [tekes-worker::ledger_events::append_run_start](../../src/ledger_events.rs#L3); [tekes-worker::ledger_events::open_goal_continuation](../../src/ledger_events.rs#L69) |
| [tekes-worker::ledger_events::ijson](../../src/ledger_events.rs#L243) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks](../../src/lifecycle_hooks.rs#L9) | `pub(super)` | not a function |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::load](../../src/lifecycle_hooks.rs#L20) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::before_turn](../../src/lifecycle_hooks.rs#L150) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::prepare](../../src/lifecycle_hooks.rs#L170) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::before_compact](../../src/lifecycle_hooks.rs#L184) | `pub(super)` | no resolved direct caller |
| [tekes-worker::lifecycle_hooks::LifecycleHooks::observe](../../src/lifecycle_hooks.rs#L201) | `pub(super)` | no resolved direct caller |
| [tekes-worker::live_tests::prepare_live_choice](../../src/live_tests.rs#L557) | `pub(super)` | no resolved direct caller |
| [tekes-worker::main::exchange_tool_control](../../src/main.rs#L402) | `pub` | no resolved direct caller |
| [tekes-worker::provider_turn::ProviderOutcome](../../src/provider_turn.rs#L15) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::ProviderContext](../../src/provider_turn.rs#L26) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::run_provider_turn](../../src/provider_turn.rs#L33) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::provider_turn::ProviderRunContext](../../src/provider_turn.rs#L102) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::EagerDispatch](../../src/provider_turn.rs#L1182) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::EagerDispatch::contains](../../src/provider_turn.rs#L1192) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::provider_turn::local_provider_call_id](../../src/provider_turn.rs#L1200) | `pub(crate)` | [tekes-worker::provider_turn::localize_provider_frame](../../src/provider_turn.rs#L1284) |
| [tekes-worker::provider_turn::repair_provider_call_arguments](../../src/provider_turn.rs#L1268) | `pub(crate)` | [tekes-worker::provider_turn::run_provider_turn_inner](../../src/provider_turn.rs#L108) |
| [tekes-worker::provider_turn::localize_provider_frame](../../src/provider_turn.rs#L1284) | `pub(crate)` | [tekes-worker::provider_turn::run_provider_turn_inner](../../src/provider_turn.rs#L108) |
| [tekes-worker::provider_turn::EagerReadyCall](../../src/provider_turn.rs#L1305) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::eager_dispatch_ready_call](../../src/provider_turn.rs#L1321) | `pub(crate)` | [tekes-worker::provider_turn::run_provider_turn_inner](../../src/provider_turn.rs#L108) |
| [tekes-worker::provider_turn::append_provider_tool_call_with_wire_id](../../src/provider_turn.rs#L1414) | `pub(crate)` | [tekes-worker::provider_turn::eager_dispatch_ready_call](../../src/provider_turn.rs#L1321); [tekes-worker::provider_turn::append_provider_tool_call](../../src/provider_turn.rs#L1403) |
| [tekes-worker::provider_turn::SelectedProviderFailure](../../src/provider_turn.rs#L1474) | `pub(crate)` | not a function |
| [tekes-worker::provider_turn::selected_provider](../../src/provider_turn.rs#L1494) | `pub(crate)` | [tekes-worker::provider_turn::run_provider_turn_inner](../../src/provider_turn.rs#L108) |
| [tekes-worker::provider_turn::current_turn_inputs](../../src/provider_turn.rs#L1528) | `pub(crate)` | [tekes-worker::provider_turn::run_provider_turn_inner](../../src/provider_turn.rs#L108) |
| [tekes-worker::queue::execute_queue_transaction](../../src/queue.rs#L10) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::recovery::UnresolvedAttempt](../../src/recovery.rs#L3) | `pub(crate)` | not a function |
| [tekes-worker::recovery::PendingToolCall](../../src/recovery.rs#L17) | `pub(crate)` | not a function |
| [tekes-worker::recovery::RecoveryProgress](../../src/recovery.rs#L29) | `pub(crate)` | not a function |
| [tekes-worker::recovery::reconcile_provider_attempt](../../src/recovery.rs#L34) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::recovery::recover_unpaired_tool_calls](../../src/recovery.rs#L103) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::recovery::pending_tool_calls](../../src/recovery.rs#L224) | `pub(crate)` | [tekes-worker::recovery::recover_unpaired_tool_calls](../../src/recovery.rs#L103) |
| [tekes-worker::recovery::recover_provider_attempt_for_ordinary](../../src/recovery.rs#L315) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::recovery::unresolved_attempt](../../src/recovery.rs#L417) | `pub(crate)` | [tekes-worker::recovery::recover_provider_attempt_for_ordinary](../../src/recovery.rs#L315); [tekes-worker::recovery::reconcile_provider_attempt](../../src/recovery.rs#L34) |
| [tekes-worker::recovery::validate_adopted_calls](../../src/recovery.rs#L535) | `pub(crate)` | [tekes-worker::recovery::complete_adopt](../../src/recovery.rs#L565) |
| [tekes-worker::result_presentation::present](../../src/result_presentation.rs#L13) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_backends::AllowedDynamicBackend](../../src/tool_backends.rs#L3) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::AllowedToolBackend](../../src/tool_backends.rs#L33) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::DeferredChildBackend](../../src/tool_backends.rs#L62) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::ReplayedChildBackend](../../src/tool_backends.rs#L74) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::exchange_tool_control_runtime](../../src/tool_backends.rs#L118) | `pub(crate)` | [tekes-worker::tool_backends::assemble_tool_backends](../../src/tool_backends.rs#L250) |
| [tekes-worker::tool_backends::tool_catalog_context](../../src/tool_backends.rs#L186) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_backends::ToolBackendPlan](../../src/tool_backends.rs#L235) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::ToolBackendRuntime](../../src/tool_backends.rs#L244) | `pub(crate)` | not a function |
| [tekes-worker::tool_backends::assemble_tool_backends](../../src/tool_backends.rs#L250) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_backends::invocation_write_roots](../../src/tool_backends.rs#L495) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_backends::session_permission_policy](../../src/tool_backends.rs#L616) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_calls::append_tool_validation_error](../../src/tool_calls.rs#L194) | `pub(crate)` | [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) |
| [tekes-worker::tool_calls::frozen_hook_bindings](../../src/tool_calls.rs#L211) | `pub(crate)` | [tekes-worker::tool_calls::execute_provider_tool_calls](../../src/tool_calls.rs#L3) |
| [tekes-worker::tool_calls::frozen_skill_catalog](../../src/tool_calls.rs#L236) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::tool_calls::ToolCallBatch](../../src/tool_calls.rs#L290) | `pub(crate)` | not a function |
| [tekes-worker::turn_terminal::PROVIDER_RETRIES](../../src/turn_terminal.rs#L4) | `pub(crate)` | not a function |
| [tekes-worker::turn_terminal::PROVIDER_ADMISSION_SUBKIND](../../src/turn_terminal.rs#L7) | `pub(crate)` | not a function |
| [tekes-worker::turn_terminal::validate_terminal_tool_calls](../../src/turn_terminal.rs#L13) | `pub(crate)` | [tekes-worker::turn_terminal::append_terminal](../../src/turn_terminal.rs#L96) |
| [tekes-worker::turn_terminal::TerminalAppend](../../src/turn_terminal.rs#L86) | `pub(crate)` | not a function |
| [tekes-worker::turn_terminal::append_terminal](../../src/turn_terminal.rs#L96) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::append_provider_terminal_error](../../src/turn_terminal.rs#L194) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::append_context_overflow](../../src/turn_terminal.rs#L267) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::append_provider_failure](../../src/turn_terminal.rs#L293) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::ProviderAdmissionRetry](../../src/turn_terminal.rs#L444) | `pub(crate)` | not a function |
| [tekes-worker::turn_terminal::wait_provider_admission](../../src/turn_terminal.rs#L458) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::settle_internal_worker_failure](../../src/turn_terminal.rs#L498) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::bounded_ledger_detail](../../src/turn_terminal.rs#L530) | `pub(crate)` | [tekes-worker::turn_terminal::append_provider_terminal_error](../../src/turn_terminal.rs#L194); [tekes-worker::turn_terminal::append_provider_failure](../../src/turn_terminal.rs#L293) |
| [tekes-worker::turn_terminal::usage_object](../../src/turn_terminal.rs#L552) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::record_input_transformations](../../src/turn_terminal.rs#L574) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::sealed_fragments](../../src/turn_terminal.rs#L594) | `pub(crate)` | [tekes-worker::turn_terminal::partial_carrier_for_eager_calls](../../src/turn_terminal.rs#L407); [tekes-worker::turn_terminal::record_input_transformations](../../src/turn_terminal.rs#L574); [tekes-worker::turn_terminal::append_terminal](../../src/turn_terminal.rs#L96) |
| [tekes-worker::turn_terminal::spill_json](../../src/turn_terminal.rs#L608) | `pub(crate)` | no resolved direct caller |
| [tekes-worker::turn_terminal::append_settle](../../src/turn_terminal.rs#L621) | `pub(crate)` | [tekes-worker::turn_terminal::settle_internal_worker_failure](../../src/turn_terminal.rs#L498) |
| [tekes-worker::validation_runtime::JUDGE_SYSTEM](../../src/validation_runtime.rs#L6) | `pub(super)` | not a function |
| [tekes-worker::validation_runtime::validator_workspace_paths](../../src/validation_runtime.rs#L8) | `pub(super)` | [tekes-worker::validation_runtime::ensure_validator](../../src/validation_runtime.rs#L1108); [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) |
| [tekes-worker::validation_runtime::has_pending_validation](../../src/validation_runtime.rs#L107) | `pub(super)` | no resolved direct caller |
| [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) | `pub(super)` | no resolved direct caller |
| [tekes-worker::validation_runtime::isolate_validator_profile](../../src/validation_runtime.rs#L204) | `pub(super)` | [tekes-worker::validation_runtime::activate_validator_profile](../../src/validation_runtime.rs#L135) |
| [tekes-worker::validation_runtime::result_value](../../src/validation_runtime.rs#L349) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719); [tekes-worker::validation_runtime::judge](../../src/validation_runtime.rs#L998) |
| [tekes-worker::validation_runtime::snapshot](../../src/validation_runtime.rs#L477) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) |
| [tekes-worker::validation_runtime::artifact_bytes](../../src/validation_runtime.rs#L662) | `pub(super)` | [tekes-worker::validation_runtime::ensure_validator](../../src/validation_runtime.rs#L1108); [tekes-worker::validation_runtime::judge](../../src/validation_runtime.rs#L998) |
| [tekes-worker::validation_runtime::validator_mandate_exhausted](../../src/validation_runtime.rs#L705) | `pub(super)` | [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) |
| [tekes-worker::validation_runtime::advance](../../src/validation_runtime.rs#L719) | `pub(super)` | no resolved direct caller |
| [tekes-worker::web_search::TavilySearchProvider](../../src/web_search.rs#L3) | `pub(crate)` | not a function |
| [tekes-worker::web_search::TavilyTransport](../../src/web_search.rs#L9) | `pub(crate)` | not a function |
| [tekes-worker::web_search::ProductionTavilyTransport](../../src/web_search.rs#L20) | `pub(crate)` | not a function |
| [tekes-worker::web_search::classify_tavily_status](../../src/web_search.rs#L150) | `pub(crate)` | [tekes-worker::web_search::send_tavily_search](../../src/web_search.rs#L90) |
| [tekes-worker::web_search::tavily_request_bytes](../../src/web_search.rs#L168) | `pub(crate)` | [tekes-worker::web_search::send_tavily_search](../../src/web_search.rs#L90) |
| [tekes-worker::web_search::resolve_public_search_endpoint](../../src/web_search.rs#L190) | `pub(crate)` | [tekes-worker::web_search::send_tavily_search](../../src/web_search.rs#L90) |
| [tekes-worker::web_search::parse_tavily_response](../../src/web_search.rs#L210) | `pub(crate)` | [tekes-worker::web_search::send_tavily_search](../../src/web_search.rs#L90) |
| [tekes-worker::web_search::validate_public_result_url](../../src/web_search.rs#L250) | `pub(crate)` | [tekes-worker::web_search::parse_tavily_response](../../src/web_search.rs#L210) |
| [tekes-worker::workspace_edits::WorkspaceEditRecorder](../../src/workspace_edits.rs#L30) | `pub` | not a function |
