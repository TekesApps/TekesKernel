# engine — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [engine::admission::AdmissionLease](../../src/admission.rs#L4) | `pub` | not a function |
| [engine::admission::LeaseRelease](../../src/admission.rs#L10) | `pub` | not a function |
| [engine::admission::AdmissionPool](../../src/admission.rs#L16) | `pub` | not a function |
| [engine::admission::AdmissionPool::new](../../src/admission.rs#L24) | `pub` | [conformance::tests::slice1_gates::slice1_gate_04_attempt_lease_commit](../../../conformance/tests/slice1_gates.rs#L461); [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities](../../../supervisor/src/process_host.rs#L782) |
| [engine::admission::AdmissionPool::request](../../src/admission.rs#L32) | `pub` | no resolved direct caller |
| [engine::admission::AdmissionPool::settle](../../src/admission.rs#L47) | `pub` | no resolved direct caller |
| [engine::admission::AdmissionPool::reap](../../src/admission.rs#L55) | `pub` | no resolved direct caller |
| [engine::admission::AdmissionPool::is_held](../../src/admission.rs#L63) | `pub` | no resolved direct caller |
| [engine::admission::AdmissionPool::held_count](../../src/admission.rs#L68) | `pub` | no resolved direct caller |
| [engine::brief_contract::BriefAtomKind](../../src/brief_contract.rs#L12) | `pub` | not a function |
| [engine::brief_contract::BriefAtom](../../src/brief_contract.rs#L19) | `pub` | not a function |
| [engine::brief_contract::BriefContract](../../src/brief_contract.rs#L27) | `pub` | not a function |
| [engine::brief_contract::read_brief](../../src/brief_contract.rs#L48) | `pub` | no resolved direct caller |
| [engine::compact_gate::first_post_compact_attempt](../../src/compact_gate.rs#L20) | `pub` | [tekes-supervisor::process_live_tests::real_public_flow_case](../../../supervisor/src/process_live_tests.rs#L1191) |
| [engine::compaction_summary::MAX_CONTINUATION_BYTES](../../src/compaction_summary.rs#L14) | `pub` | not a function |
| [engine::compaction_summary::summary_request_bytes](../../src/compaction_summary.rs#L18) | `pub` | [provider::tests::live_compaction_summary::production_compactor_admits_frozen_history](../../../provider/tests/live_compaction_summary.rs#L5); [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L3141); [tekes-worker::main::run_compaction_summary](../../../worker/src/main.rs#L6524) |
| [engine::compaction_summary::SourceBundle](../../src/compaction_summary.rs#L31) | `pub` | not a function |
| [engine::compaction_summary::freeze_source_bundle](../../src/compaction_summary.rs#L39) | `pub` | [provider::tests::live_compaction_summary::production_compactor_admits_frozen_history](../../../provider/tests/live_compaction_summary.rs#L5); [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L3141); [tekes-worker::main::run_compaction_summary](../../../worker/src/main.rs#L6524) |
| [engine::compaction_summary::SUMMARY_SYSTEM](../../src/compaction_summary.rs#L92) | `pub` | not a function |
| [engine::compaction_summary::SummaryArtifact](../../src/compaction_summary.rs#L95) | `pub` | not a function |
| [engine::compaction_summary::admit_summary_artifact](../../src/compaction_summary.rs#L103) | `pub` | [provider::tests::live_compaction_summary::production_compactor_admits_frozen_history](../../../provider/tests/live_compaction_summary.rs#L5); [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L3141); [tekes-worker::main::run_compaction_summary](../../../worker/src/main.rs#L6524) |
| [engine::compaction_summary::summary_record](../../src/compaction_summary.rs#L153) | `pub` | [engine::compaction_summary::summary_unavailable](../../src/compaction_summary.rs#L176); [engine::compaction_summary::CompactionSummary::from_outcome](../../src/compaction_summary.rs#L192) |
| [engine::compaction_summary::summary_unavailable](../../src/compaction_summary.rs#L176) | `pub` | [engine::compaction_summary::CompactionSummary::apply](../../src/compaction_summary.rs#L213) |
| [engine::compaction_summary::CompactionSummary](../../src/compaction_summary.rs#L182) | `pub` | not a function |
| [engine::compaction_summary::CompactionSummary::from_outcome](../../src/compaction_summary.rs#L192) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L3141); [tekes-worker::main::run_compaction_summary](../../../worker/src/main.rs#L6524) |
| [engine::compaction_summary::CompactionSummary::apply](../../src/compaction_summary.rs#L213) | `pub` | no resolved direct caller |
| [engine::compaction_summary::seq_ranges](../../src/compaction_summary.rs#L232) | `pub` | no resolved direct caller |
| [engine::context::ContextCompactionPlan](../../src/context.rs#L5) | `pub` | not a function |
| [engine::context::compaction_anchor_sequences](../../src/context.rs#L13) | `pub` | [engine::context::plan_context_compaction](../../src/context.rs#L106); [tekes-worker::main::project_provider_context_mode](../../../worker/src/main.rs#L6791) |
| [engine::context::plan_context_compaction](../../src/context.rs#L106) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::locked_compact](../../../supervisor/src/process_host.rs#L2802); [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L3141); [tekes-worker::main::provider_context_tests::overflow_with_summary](../../../worker/src/main.rs#L12373); [tekes-worker::main::provider_context_tests::preflight_compaction_precedes_the_send_when_the_candidate_exceeds_the_trigger](../../../worker/src/main.rs#L13441); [tekes-worker::main::run_provider_turn_inner](../../../worker/src/main.rs#L5012); [tekes-worker::main::build_compaction_event](../../../worker/src/main.rs#L6479); [tekes-worker::main::run_compaction_summary](../../../worker/src/main.rs#L6524) |
| [engine::context::ToolResultTrim](../../src/context.rs#L411) | `pub` | not a function |
| [engine::context::TRIM_THRESHOLD_BYTES](../../src/context.rs#L419) | `pub` | not a function |
| [engine::context::TRIM_HEAD_BYTES](../../src/context.rs#L420) | `pub` | not a function |
| [engine::context::TRIM_TAIL_BYTES](../../src/context.rs#L421) | `pub` | not a function |
| [engine::context::plan_tool_result_trim](../../src/context.rs#L431) | `pub` | [tekes-worker::main::provider_context_tests::a_trim_written_in_a_later_turn_is_stamped_with_the_open_turn](../../../worker/src/main.rs#L13174); [tekes-worker::main::run_provider_turn_inner](../../../worker/src/main.rs#L5012) |
| [engine::context::trimmed_tool_result_content](../../src/context.rs#L502) | `pub` | [tekes-worker::main::append_tool_result_trim](../../../worker/src/main.rs#L7540) |
| [engine::delivery::DeliveryResolution](../../src/delivery.rs#L27) | `pub` | not a function |
| [engine::delivery::DeliveryIndex](../../src/delivery.rs#L33) | `pub` | not a function |
| [engine::delivery::DeliveryIndex::resolve](../../src/delivery.rs#L39) | `pub` | no resolved direct caller |
| [engine::delivery::DeliveryIndex::commit](../../src/delivery.rs#L47) | `pub` | no resolved direct caller |
| [engine::delivery::CreateResolution](../../src/delivery.rs#L55) | `pub` | not a function |
| [engine::delivery::CreateIndex](../../src/delivery.rs#L61) | `pub` | not a function |
| [engine::delivery::CreateIndex::resolve](../../src/delivery.rs#L66) | `pub` | no resolved direct caller |
| [engine::dispatcher::ToolInvocation](../../src/dispatcher.rs#L14) | `pub` | not a function |
| [engine::dispatcher::ApprovalClass](../../src/dispatcher.rs#L25) | `pub` | not a function |
| [engine::dispatcher::PolicyDecision](../../src/dispatcher.rs#L34) | `pub` | not a function |
| [engine::dispatcher::ToolPolicy](../../src/dispatcher.rs#L40) | `pub` | not a function |
| [engine::dispatcher::AllowAllPolicy](../../src/dispatcher.rs#L51) | `pub` | not a function |
| [engine::dispatcher::ProductionToolPolicy](../../src/dispatcher.rs#L71) | `pub` | not a function |
| [engine::dispatcher::PermissionModePolicy](../../src/dispatcher.rs#L119) | `pub` | not a function |
| [engine::dispatcher::PermissionModePolicy::new](../../src/dispatcher.rs#L125) | `pub` | no resolved direct caller |
| [engine::dispatcher::PermissionModePolicy::for_session_folder](../../src/dispatcher.rs#L132) | `pub` | [tekes-worker::main::session_permission_policy](../../../worker/src/main.rs#L3341) |
| [engine::dispatcher::ToolDispatcher](../../src/dispatcher.rs#L162) | `pub` | not a function |
| [engine::dispatcher::ToolDispatcher::new](../../src/dispatcher.rs#L169) | `pub` | [engine::tests::slice8_dispatcher::generic_policy_runs_after_effective_argument_validation](../../tests/slice8_dispatcher.rs#L126); [engine::tests::slice8_dispatcher::production_policy_durably_parks_workspace_edits_before_backend_execution](../../tests/slice8_dispatcher.rs#L201); [engine::tests::slice8_dispatcher::provider_catalog_omits_visible_tools_without_an_executable_backend](../../tests/slice8_dispatcher.rs#L20); [engine::tests::slice8_dispatcher::granted_approval_resumes_durable_effective_invocation_exactly_once](../../tests/slice8_dispatcher.rs#L233); [engine::tests::slice8_dispatcher::pending_and_denied_approvals_never_execute_the_backend](../../tests/slice8_dispatcher.rs#L316); [engine::tests::slice8_dispatcher::skill_consumption_requires_a_prior_durable_offer_in_the_same_turn](../../tests/slice8_dispatcher.rs#L383); [engine::tests::slice8_dispatcher::dispatcher_derives_catalog_schema_and_backend_from_fixed_entry](../../tests/slice8_dispatcher.rs#L51); [engine::tests::slice8_dispatcher::dispatcher_rejects_unadvertised_invalid_and_non_durable_calls](../../tests/slice8_dispatcher.rs#L84); [tekes-worker::main::execute_provider_tool_calls](../../../worker/src/main.rs#L3349); [tekes-worker::main::execute_child_spawn_tool](../../../worker/src/main.rs#L4227); [tekes-worker::main::run_provider_turn_inner](../../../worker/src/main.rs#L5012) |
| [engine::dispatcher::ToolDispatcher::provider_catalog](../../src/dispatcher.rs#L176) | `pub` | no resolved direct caller |
| [engine::dispatcher::ToolDispatcher::dispatch](../../src/dispatcher.rs#L193) | `pub` | no resolved direct caller |
| [engine::dispatcher::side_effectful](../../src/dispatcher.rs#L276) | `pub` | [engine::dispatcher::ToolDispatcher::dispatch](../../src/dispatcher.rs#L193); [tekes-worker::main::recover_unpaired_tool_calls](../../../worker/src/main.rs#L1427); [tekes-worker::main::append_provider_tool_call_with_wire_id](../../../worker/src/main.rs#L6318); [tekes-worker::main::provider_context_tests::append_test_call](../../../worker/src/main.rs#L9444) |
| [engine::dispatcher::approval_class](../../src/dispatcher.rs#L284) | `pub` | [engine::dispatcher::ToolDispatcher::dispatch](../../src/dispatcher.rs#L193) |
| [engine::dispatcher::approval_scope](../../src/dispatcher.rs#L310) | `pub` | [engine::dispatcher::ToolDispatcher::dispatch](../../src/dispatcher.rs#L193) |
| [engine::dispatcher::DispatchError](../../src/dispatcher.rs#L321) | `pub` | not a function |
| [engine::dynamic_catalog::DynamicBackend](../../src/dynamic_catalog.rs#L20) | `pub` | not a function |
| [engine::dynamic_catalog::DynamicSupervisorBackend](../../src/dynamic_catalog.rs#L34) | `pub` | not a function |
| [engine::dynamic_catalog::DynamicSupervisorBackend::new](../../src/dynamic_catalog.rs#L41) | `pub` | [engine::tests::slice8_dynamic_catalog::supervisor_dynamic_route_uses_the_same_correlated_control_tuple](../../tests/slice8_dynamic_catalog.rs#L409); [engine::tests::slice8_dynamic_catalog::supervisor_dynamic_route_maps_control_eof_to_retryable_unavailable](../../tests/slice8_dynamic_catalog.rs#L452); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975) |
| [engine::dynamic_catalog::validate_dynamic_invocation](../../src/dynamic_catalog.rs#L140) | `pub` | [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_dynamic](../../../supervisor/src/production_tool_control.rs#L448) |
| [engine::dynamic_catalog::DynamicToolDispatcher](../../src/dynamic_catalog.rs#L147) | `pub` | not a function |
| [engine::dynamic_catalog::DynamicToolDispatcher::new](../../src/dynamic_catalog.rs#L154) | `pub` | [engine::tests::slice8_dynamic_catalog::mcp_destructive_hint_reaches_the_engine_destructive_approval_gate](../../tests/slice8_dynamic_catalog.rs#L128); [engine::tests::slice8_dynamic_catalog::deferred_provider_projection_requires_a_visible_search_offer_and_backend](../../tests/slice8_dynamic_catalog.rs#L187); [engine::tests::slice8_dynamic_catalog::compaction_over_the_search_result_retires_the_offer](../../tests/slice8_dynamic_catalog.rs#L240); [engine::tests::slice8_dynamic_catalog::deferred_dispatch_requires_a_durable_visible_tool_search_offer](../../tests/slice8_dynamic_catalog.rs#L321); [engine::tests::slice8_dynamic_catalog::exact_dynamic_schema_and_dependency_are_checked_before_effect](../../tests/slice8_dynamic_catalog.rs#L365); [engine::tests::slice8_dynamic_catalog::resident_projection_is_exact_stable_and_dependency_gated](../../tests/slice8_dynamic_catalog.rs#L51); [engine::tests::slice8_dynamic_catalog::dynamic_dispatch_uses_exact_metadata_and_the_common_policy_pipeline](../../tests/slice8_dynamic_catalog.rs#L82); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975); [tekes-worker::main::execute_provider_tool_calls](../../../worker/src/main.rs#L3349); [tekes-worker::main::run_provider_turn_inner](../../../worker/src/main.rs#L5012) |
| [engine::dynamic_catalog::DynamicToolDispatcher::provider_catalog](../../src/dynamic_catalog.rs#L168) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::DynamicToolDispatcher::provider_catalog_visible](../../src/dynamic_catalog.rs#L182) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::DynamicToolDispatcher::complete_catalog](../../src/dynamic_catalog.rs#L203) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::DynamicToolDispatcher::deferred_search_entries](../../src/dynamic_catalog.rs#L215) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::DynamicToolDispatcher::dispatch](../../src/dynamic_catalog.rs#L235) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::dynamic_workflow_backend](../../src/dynamic_catalog.rs#L300) | `pub` | no resolved direct caller |
| [engine::dynamic_catalog::dynamic_side_effectful](../../src/dynamic_catalog.rs#L508) | `pub` | [engine::dynamic_catalog::DynamicToolDispatcher::dispatch](../../src/dynamic_catalog.rs#L235); [tekes-worker::main::wait_tool_continuation](../../../worker/src/main.rs#L3700) |
| [engine::dynamic_catalog::dynamic_approval_class](../../src/dynamic_catalog.rs#L513) | `pub` | [engine::dynamic_catalog::DynamicToolDispatcher::dispatch](../../src/dynamic_catalog.rs#L235); [engine::dynamic_catalog::dynamic_approval_scope](../../src/dynamic_catalog.rs#L526) |
| [engine::dynamic_catalog::DynamicDispatchError](../../src/dynamic_catalog.rs#L537) | `pub` | not a function |
| [engine::lifecycle::LockFacts](../../src/lifecycle.rs#L4) | `pub` | not a function |
| [engine::lifecycle::FREE](../../src/lifecycle.rs#L10) | `pub` | not a function |
| [engine::lifecycle::CALLER](../../src/lifecycle.rs#L14) | `pub` | not a function |
| [engine::lifecycle::OTHER](../../src/lifecycle.rs#L18) | `pub` | not a function |
| [engine::lifecycle::TailState](../../src/lifecycle.rs#L25) | `pub` | not a function |
| [engine::lifecycle::TailState::as_str](../../src/lifecycle.rs#L38) | `pub` | no resolved direct caller |
| [engine::lifecycle::RunMode](../../src/lifecycle.rs#L52) | `pub` | not a function |
| [engine::lifecycle::RunDecision](../../src/lifecycle.rs#L58) | `pub` | not a function |
| [engine::lifecycle::EnsureAction](../../src/lifecycle.rs#L64) | `pub` | not a function |
| [engine::lifecycle::DeliveryAction](../../src/lifecycle.rs#L71) | `pub` | not a function |
| [engine::lifecycle::ArchiveAction](../../src/lifecycle.rs#L78) | `pub` | not a function |
| [engine::lifecycle::classify](../../src/lifecycle.rs#L84) | `pub` | [conformance::tests::slice1_gates::slice1_gate_06_completed_turn_usage_exit](../../../conformance/tests/slice1_gates.rs#L639); [conformance::tests::slice1_gates::slice1_gate_33b_admission_wait_is_a_durable_obligation_of_the_open_turn](../../../conformance/tests/slice1_gates.rs#L687); [conformance::tests::slice1_gates::slice1_gate_33_run_mode_arbitration](../../../conformance/tests/slice1_gates.rs#L731); [tekes-supervisor::endpoint_carrier::ProductionCarrierStreams::inventory_baseline](../../../supervisor/src/endpoint_carrier.rs#L920); [tekes-supervisor::process_host::ProductionProcessHost::reconcile_schedule_statuses](../../../supervisor/src/process_host.rs#L1432); [tekes-supervisor::process_host::ProductionProcessHost::sweep_once](../../../supervisor/src/process_host.rs#L1892); [tekes-supervisor::process_host::ProductionProcessHost::launch_child](../../../supervisor/src/process_host.rs#L3631); [tekes-supervisor::process_host::ProductionProcessHost::reconcile_after_exit](../../../supervisor/src/process_host.rs#L3897); [tekes-supervisor::process_host::ledger_needs_worker](../../../supervisor/src/process_host.rs#L5445); [tekes-worker::main::run](../../../worker/src/main.rs#L786) |
| [engine::lifecycle::ensure_action](../../src/lifecycle.rs#L107) | `pub` | no resolved direct caller |
| [engine::lifecycle::ensure_action_at](../../src/lifecycle.rs#L116) | `pub` | [engine::lifecycle::ensure_action](../../src/lifecycle.rs#L107); [tekes-supervisor::process_host::ProductionProcessHost::sweep_once](../../../supervisor/src/process_host.rs#L1892); [tekes-supervisor::process_host::ProductionProcessHost::launch_child](../../../supervisor/src/process_host.rs#L3631); [tekes-supervisor::process_host::ProductionProcessHost::reconcile_after_exit](../../../supervisor/src/process_host.rs#L3897); [tekes-supervisor::process_host::ledger_needs_worker](../../../supervisor/src/process_host.rs#L5445) |
| [engine::lifecycle::delivery_action](../../src/lifecycle.rs#L140) | `pub` | no resolved direct caller |
| [engine::lifecycle::run_decision](../../src/lifecycle.rs#L159) | `pub` | [tekes-worker::main::run](../../../worker/src/main.rs#L786) |
| [engine::lifecycle::archive_action](../../src/lifecycle.rs#L184) | `pub` | no resolved direct caller |
| [engine::permission_mode::PERMISSION_MODE_FILE](../../src/permission_mode.rs#L19) | `pub` | not a function |
| [engine::permission_mode::PermissionMode](../../src/permission_mode.rs#L22) | `pub` | not a function |
| [engine::permission_mode::ALL](../../src/permission_mode.rs#L34) | `pub` | not a function |
| [engine::permission_mode::PermissionMode::as_str](../../src/permission_mode.rs#L41) | `pub` | no resolved direct caller |
| [engine::permission_mode::PermissionMode::parse](../../src/permission_mode.rs#L51) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../../supervisor/src/client_extensions.rs#L1739); [tekes-supervisor::client_extensions::validate_payload](../../../supervisor/src/client_extensions.rs#L2834); [tekes-supervisor::resource_capability::ClientResourceService::commands_run_with_catalog](../../../supervisor/src/resource_capability.rs#L194) |
| [engine::permission_mode::PermissionModeRead](../../src/permission_mode.rs#L73) | `pub` | not a function |
| [engine::permission_mode::permission_mode_path](../../src/permission_mode.rs#L79) | `pub` | [engine::dispatcher::permission_mode_tests::default_mode_is_workspace_write_and_folder_read_reports_corruption](../../src/dispatcher.rs#L445); [engine::permission_mode::write_permission_mode](../../src/permission_mode.rs#L117); [engine::permission_mode::read_permission_mode](../../src/permission_mode.rs#L85); [tekes-worker::main::provider_context_tests::permission_mode_file_change_applies_to_the_next_tool_batch](../../../worker/src/main.rs#L12871) |
| [engine::permission_mode::read_permission_mode](../../src/permission_mode.rs#L85) | `pub` | [engine::dispatcher::PermissionModePolicy::for_session_folder](../../src/dispatcher.rs#L132); [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../../supervisor/src/client_extensions.rs#L1739) |
| [engine::permission_mode::write_permission_mode](../../src/permission_mode.rs#L117) | `pub` | [engine::dispatcher::permission_mode_tests::default_mode_is_workspace_write_and_folder_read_reports_corruption](../../src/dispatcher.rs#L445); [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../../supervisor/src/client_extensions.rs#L1739); [tekes-supervisor::resource_capability::EndpointCommandInputAuthority::select_permission](../../../supervisor/src/resource_capability.rs#L529); [tekes-supervisor::tests::slice9_endpoint_host::inventory_reports_the_durable_permission_mode](../../../supervisor/tests/slice9_endpoint_host.rs#L640); [tekes-worker::main::provider_context_tests::permission_mode_file_change_applies_to_the_next_tool_batch](../../../worker/src/main.rs#L12871) |
| [engine::provider::Continuation](../../src/provider.rs#L4) | `pub` | not a function |
| [engine::provider::QueryCapability](../../src/provider.rs#L10) | `pub` | not a function |
| [engine::provider::AdapterCapabilities](../../src/provider.rs#L16) | `pub` | not a function |
| [engine::provider::SentState](../../src/provider.rs#L23) | `pub` | not a function |
| [engine::provider::QueryResult](../../src/provider.rs#L29) | `pub` | not a function |
| [engine::provider::RecoveryDecision](../../src/provider.rs#L40) | `pub` | not a function |
| [engine::provider::sent_state](../../src/provider.rs#L49) | `pub` | [tekes-worker::main::recover_provider_attempt_for_ordinary](../../../worker/src/main.rs#L1639) |
| [engine::provider::decide_recovery](../../src/provider.rs#L58) | `pub` | [tekes-worker::main::recover_provider_attempt_for_ordinary](../../../worker/src/main.rs#L1639) |
| [engine::seams::ProviderTerminal](../../src/seams.rs#L9) | `pub` | not a function |
| [engine::seams::ProviderAdapter](../../src/seams.rs#L16) | `pub` | not a function |
| [engine::seams::FakeProvider](../../src/seams.rs#L24) | `pub` | not a function |
| [engine::seams::FakeProvider::new](../../src/seams.rs#L32) | `pub` | [conformance::tests::slice1_gates::slice1_gate_04_attempt_lease_commit](../../../conformance/tests/slice1_gates.rs#L461) |
| [engine::seams::FakeProvider::push_query_result](../../src/seams.rs#L40) | `pub` | no resolved direct caller |
| [engine::seams::FakeProvider::insert_response](../../src/seams.rs#L44) | `pub` | no resolved direct caller |
| [engine::seams::ToolBackend](../../src/seams.rs#L69) | `pub` | not a function |
| [engine::seams::ToolBackendRouter](../../src/seams.rs#L93) | `pub` | not a function |
| [engine::seams::ToolBackendRouter::new](../../src/seams.rs#L99) | `pub` | no resolved direct caller |
| [engine::seams::ToolBackendRouter::push](../../src/seams.rs#L103) | `pub` | no resolved direct caller |
| [engine::seams::FakeToolBackend](../../src/seams.rs#L150) | `pub` | not a function |
| [engine::seams::FakeToolBackend::set_outcome](../../src/seams.rs#L156) | `pub` | no resolved direct caller |
| [engine::seams::FakeToolBackend::set_terminal](../../src/seams.rs#L170) | `pub` | no resolved direct caller |
| [engine::seams::FakeToolBackend::execution_count](../../src/seams.rs#L175) | `pub` | no resolved direct caller |
| [engine::seams::ProcessState](../../src/seams.rs#L195) | `pub` | not a function |
| [engine::seams::ProcessHost](../../src/seams.rs#L201) | `pub` | not a function |
| [engine::seams::FakeProcessHost](../../src/seams.rs#L209) | `pub` | not a function |
| [engine::seams::FakeProcessHost::exit](../../src/seams.rs#L214) | `pub` | no resolved direct caller |
| [engine::stop::StopNode](../../src/stop.rs#L4) | `pub` | not a function |
| [engine::stop::StopTree](../../src/stop.rs#L13) | `pub` | not a function |
| [engine::stop::StopTree::add_root](../../src/stop.rs#L19) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::add_child](../../src/stop.rs#L31) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::mark_stop_durable](../../src/stop.rs#L47) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::mark_signaled](../../src/stop.rs#L66) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::mark_settled](../../src/stop.rs#L77) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::node](../../src/stop.rs#L96) | `pub` | no resolved direct caller |
| [engine::stop::StopTree::root_settled](../../src/stop.rs#L101) | `pub` | no resolved direct caller |
| [engine::supervisor::DrainAction](../../src/supervisor.rs#L4) | `pub` | not a function |
| [engine::supervisor::RecoveryStage](../../src/supervisor.rs#L12) | `pub` | not a function |
| [engine::supervisor::RECOVERY_ORDER](../../src/supervisor.rs#L19) | `pub` | not a function |
| [engine::supervisor::DrainTracker](../../src/supervisor.rs#L27) | `pub` | not a function |
| [engine::supervisor::DrainTracker::new](../../src/supervisor.rs#L34) | `pub` | [conformance::tests::slice1_gates::slice1_gate_12_supervisor_total_failure](../../../conformance/tests/slice1_gates.rs#L1086) |
| [engine::supervisor::DrainTracker::observe](../../src/supervisor.rs#L41) | `pub` | no resolved direct caller |
| [engine::supervisor_tools::SupervisorControlBackend](../../src/supervisor_tools.rs#L9) | `pub` | not a function |
| [engine::supervisor_tools::SupervisorControlBackend::new](../../src/supervisor_tools.rs#L16) | `pub` | [engine::tests::slice8_supervisor_backend::supervisor_backend_never_converts_a_typed_failure_to_success](../../tests/slice8_supervisor_backend.rs#L36); [engine::tests::slice8_supervisor_backend::supervisor_backend_derives_and_validates_correlated_request](../../tests/slice8_supervisor_backend.rs#L9); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975) |
| [engine::system_tools::DEFAULT_SHELL_DURATION_MS](../../src/system_tools.rs#L40) | `pub` | not a function |
| [engine::system_tools::MAX_SHELL_DURATION_MS](../../src/system_tools.rs#L43) | `pub` | not a function |
| [engine::system_tools::RootMount](../../src/system_tools.rs#L67) | `pub` | not a function |
| [engine::system_tools::SystemToolConfig](../../src/system_tools.rs#L73) | `pub` | not a function |
| [engine::system_tools::SystemToolConfig::workspace](../../src/system_tools.rs#L87) | `pub` | [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205) |
| [engine::system_tools::ArtifactVersionPermit](../../src/system_tools.rs#L114) | `pub` | not a function |
| [engine::system_tools::ArtifactVersionAuthority](../../src/system_tools.rs#L121) | `pub` | not a function |
| [engine::system_tools::CommittedEditRecorder](../../src/system_tools.rs#L138) | `pub` | not a function |
| [engine::system_tools::DurableArtifactVersions](../../src/system_tools.rs#L167) | `pub` | not a function |
| [engine::system_tools::DurableArtifactVersions::new](../../src/system_tools.rs#L179) | `pub` | [engine::tests::slice8_system_tools::backend_with](../../tests/slice8_system_tools.rs#L161); [engine::tests::slice8_system_tools::artifact_identity_preserves_absolute_history_and_separates_workspaces](../../tests/slice8_system_tools.rs#L514); [engine::tests::slice8_system_tools::ambiguous_legacy_relative_history_is_preserved_and_rejected](../../tests/slice8_system_tools.rs#L530); [engine::tests::slice8_system_tools::reserved_version_survives_authority_restart_and_fences_old_token](../../tests/slice8_system_tools.rs#L575); [engine::tests::slice8_system_tools::only_one_concurrent_reservation_wins](../../tests/slice8_system_tools.rs#L596); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205) |
| [engine::system_tools::SystemToolBackend](../../src/system_tools.rs#L463) | `pub` | not a function |
| [engine::system_tools::SystemToolBackend::new](../../src/system_tools.rs#L477) | `pub` | [engine::tests::slice8_system_tools::backend_with](../../tests/slice8_system_tools.rs#L161); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205) |
| [engine::system_tools::SystemToolBackend::with_edit_recorder](../../src/system_tools.rs#L499) | `pub` | no resolved direct caller |
| [engine::system_tools::web_fetch_result_value](../../src/system_tools.rs#L1411) | `pub` | [engine::system_tools::SystemToolBackend::web_fetch](../../src/system_tools.rs#L888); [engine::tests::slice14e_web::slice14e_gate_109_web_fetch_result_is_closed_and_drops_raw_body](../../tests/slice14e_web.rs#L6) |
| [engine::tool::execute_tool](../../src/tool.rs#L5) | `pub` | no resolved direct caller |
| [engine::transactions::DeliveryPhase](../../src/transactions.rs#L4) | `pub` | not a function |
| [engine::transactions::DeliveryCommit](../../src/transactions.rs#L12) | `pub` | not a function |
| [engine::transactions::DeliveryCommit::new](../../src/transactions.rs#L19) | `pub` | [conformance::tests::slice1_gates::slice1_gate_02_dead_target_keyed_delivery](../../../conformance/tests/slice1_gates.rs#L922) |
| [engine::transactions::DeliveryCommit::appended](../../src/transactions.rs#L26) | `pub` | no resolved direct caller |
| [engine::transactions::DeliveryCommit::durable](../../src/transactions.rs#L34) | `pub` | no resolved direct caller |
| [engine::transactions::DeliveryCommit::acknowledge](../../src/transactions.rs#L42) | `pub` | no resolved direct caller |
| [engine::transactions::OutcomePhase](../../src/transactions.rs#L52) | `pub` | not a function |
| [engine::transactions::AttemptPhase](../../src/transactions.rs#L60) | `pub` | not a function |
| [engine::transactions::AttemptFlow](../../src/transactions.rs#L70) | `pub` | not a function |
| [engine::transactions::AttemptFlow::new](../../src/transactions.rs#L78) | `pub` | [conformance::tests::slice1_gates::slice1_gate_04_attempt_lease_commit](../../../conformance/tests/slice1_gates.rs#L461) |
| [engine::transactions::AttemptFlow::lease_granted](../../src/transactions.rs#L86) | `pub` | no resolved direct caller |
| [engine::transactions::AttemptFlow::attempt_durable](../../src/transactions.rs#L94) | `pub` | no resolved direct caller |
| [engine::transactions::AttemptFlow::dispatch_durable](../../src/transactions.rs#L102) | `pub` | no resolved direct caller |
| [engine::transactions::AttemptFlow::begin_http](../../src/transactions.rs#L113) | `pub` | no resolved direct caller |
| [engine::transactions::AttemptFlow::terminal_received](../../src/transactions.rs#L126) | `pub` | no resolved direct caller |
| [engine::transactions::OutcomeCommit](../../src/transactions.rs#L136) | `pub` | not a function |
| [engine::transactions::OutcomeCommit::new](../../src/transactions.rs#L143) | `pub` | [conformance::tests::slice1_gates::slice1_gate_05_outcome_usage_crash_window](../../../conformance/tests/slice1_gates.rs#L548) |
| [engine::transactions::OutcomeCommit::outcome_appended](../../src/transactions.rs#L152) | `pub` | no resolved direct caller |
| [engine::transactions::OutcomeCommit::durable](../../src/transactions.rs#L160) | `pub` | no resolved direct caller |
| [engine::transactions::OutcomeCommit::release_lease](../../src/transactions.rs#L168) | `pub` | no resolved direct caller |
| [engine::transactions::TransactionError](../../src/transactions.rs#L178) | `pub` | not a function |
| [engine::validation::ValidationBinding](../../src/validation.rs#L14) | `pub` | not a function |
| [engine::validation::ValidationBinding::covered_by](../../src/validation.rs#L26) | `pub` | no resolved direct caller |
| [engine::validation::ValidationBinding::admits_verdict](../../src/validation.rs#L37) | `pub` | no resolved direct caller |
| [engine::validation::ValidationVerdict](../../src/validation.rs#L44) | `pub` | not a function |
| [engine::validation::ValidationOutcome](../../src/validation.rs#L52) | `pub` | not a function |
| [engine::validation::ValidationSignal](../../src/validation.rs#L59) | `pub` | not a function |
| [engine::validation::ValidationNegative](../../src/validation.rs#L70) | `pub` | not a function |
| [engine::validation::ValidationDecision](../../src/validation.rs#L78) | `pub` | not a function |
| [engine::validation::validation_decision](../../src/validation.rs#L95) | `pub` | [engine::validation_writer::commit_validation_decision](../../src/validation_writer.rs#L92) |
| [engine::validation_writer::begin_validation](../../src/validation_writer.rs#L25) | `pub` | [tekes-supervisor::process_host::tests::real_worker_recovers_final_before_settlement_with_resume_never](../../../supervisor/src/process_host.rs#L8944); [tekes-supervisor::process_live_tests::run_real_validator_process](../../../supervisor/src/process_live_tests.rs#L195); [tekes-worker::validation_runtime::advance](../../../worker/src/validation_runtime.rs#L719); [tekes-worker::validation_tests::validation_decision_rejects_model_output_as_control_signal](../../../worker/src/validation_tests.rs#L114); [tekes-worker::validation_tests::validation_obligation_survives_resume_never_without_reopening_settled_turn](../../../worker/src/validation_tests.rs#L128); [tekes-worker::validation_tests::validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap](../../../worker/src/validation_tests.rs#L170); [tekes-worker::validation_tests::settlement_recovery_rejects_a_foreign_candidate_binding_without_writes](../../../worker/src/validation_tests.rs#L289); [tekes-worker::validation_tests::failed_decision_recovery_delivers_feedback_once_to_the_same_worker](../../../worker/src/validation_tests.rs#L308); [tekes-worker::validation_tests::stop_during_validator_launch_does_not_settle_as_internal_failure](../../../worker/src/validation_tests.rs#L346); [tekes-worker::validation_tests::queued_retry_body_waits_for_validation_settlement_then_opens_once](../../../worker/src/validation_tests.rs#L417); [tekes-worker::validation_tests::validation_candidate_and_decision_survive_reopen_without_second_writes](../../../worker/src/validation_tests.rs#L48) |
| [engine::validation_writer::commit_validation_decision](../../src/validation_writer.rs#L92) | `pub` | [tekes-supervisor::process_host::tests::real_worker_recovers_final_before_settlement_with_resume_never](../../../supervisor/src/process_host.rs#L8944); [tekes-worker::validation_runtime::advance](../../../worker/src/validation_runtime.rs#L719); [tekes-worker::validation_tests::validation_obligation_survives_resume_never_without_reopening_settled_turn](../../../worker/src/validation_tests.rs#L128); [tekes-worker::validation_tests::validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap](../../../worker/src/validation_tests.rs#L170); [tekes-worker::validation_tests::settlement_recovery_rejects_a_foreign_candidate_binding_without_writes](../../../worker/src/validation_tests.rs#L289); [tekes-worker::validation_tests::failed_decision_recovery_delivers_feedback_once_to_the_same_worker](../../../worker/src/validation_tests.rs#L308); [tekes-worker::validation_tests::queued_retry_body_waits_for_validation_settlement_then_opens_once](../../../worker/src/validation_tests.rs#L417); [tekes-worker::validation_tests::validation_candidate_and_decision_survive_reopen_without_second_writes](../../../worker/src/validation_tests.rs#L48) |
| [engine::validation_writer::materialize_validation_settlement](../../src/validation_writer.rs#L248) | `pub` | [tekes-worker::validation_runtime::advance](../../../worker/src/validation_runtime.rs#L719); [tekes-worker::validation_tests::validation_obligation_survives_resume_never_without_reopening_settled_turn](../../../worker/src/validation_tests.rs#L128); [tekes-worker::validation_tests::validation_changed_answer_rejudges_same_files_and_second_failure_hits_cap](../../../worker/src/validation_tests.rs#L170); [tekes-worker::validation_tests::queued_retry_body_waits_for_validation_settlement_then_opens_once](../../../worker/src/validation_tests.rs#L417); [tekes-worker::validation_tests::validation_candidate_and_decision_survive_reopen_without_second_writes](../../../worker/src/validation_tests.rs#L48) |
| [engine::workflow_tools::CatalogEntry](../../src/workflow_tools.rs#L43) | `pub` | not a function |
| [engine::workflow_tools::WorkflowBackend](../../src/workflow_tools.rs#L52) | `pub` | not a function |
| [engine::workflow_tools::WorkflowBackend::new](../../src/workflow_tools.rs#L60) | `pub` | [engine::dynamic_catalog::dynamic_workflow_backend](../../src/dynamic_catalog.rs#L300); [engine::tests::slice8_dispatcher::skill_consumption_requires_a_prior_durable_offer_in_the_same_turn](../../tests/slice8_dispatcher.rs#L383); [engine::tests::slice8_workflow::retired_memory_tools_cannot_read_write_or_repair_the_legacy_archive](../../tests/slice8_workflow.rs#L146); [engine::tests::slice8_workflow::discovery_and_skill_content_survive_backend_restart_without_side_authority](../../tests/slice8_workflow.rs#L57); [engine::tests::slice8_workflow::goal_transactions_are_durable_not_process_memory](../../tests/slice8_workflow.rs#L90); [engine::tests::slice8_workflow::workflow_holds_are_typed_and_do_not_fabricate_success](../../tests/slice8_workflow.rs#L9); [tekes-worker::main::assemble_tool_backends](../../../worker/src/main.rs#L2975) |
