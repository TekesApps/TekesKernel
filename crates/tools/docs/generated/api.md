# tools — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [tools::builtin::BuiltinManifest](../../src/builtin.rs#L9) | `pub` | not a function |
| [tools::builtin::BuiltinTool](../../src/builtin.rs#L16) | `pub` | not a function |
| [tools::builtin::CatalogRole](../../src/builtin.rs#L285) | `pub` | not a function |
| [tools::builtin::CatalogContext](../../src/builtin.rs#L295) | `pub` | not a function |
| [tools::builtin::BuiltinManifest::compiled](../../src/builtin.rs#L315) | `pub` | [endpoint::management::default_workspace_policy](../../../endpoint/src/management.rs#L2792); [engine::tests::slice8_dispatcher::generic_policy_runs_after_effective_argument_validation](../../../engine/tests/slice8_dispatcher.rs#L126); [engine::tests::slice8_dispatcher::production_policy_allows_reads_delegates_workflow_and_holds_effects](../../../engine/tests/slice8_dispatcher.rs#L164); [engine::tests::slice8_dispatcher::production_policy_durably_parks_workspace_edits_before_backend_execution](../../../engine/tests/slice8_dispatcher.rs#L201); [engine::tests::slice8_dispatcher::provider_catalog_omits_visible_tools_without_an_executable_backend](../../../engine/tests/slice8_dispatcher.rs#L20); [engine::tests::slice8_dispatcher::granted_approval_resumes_durable_effective_invocation_exactly_once](../../../engine/tests/slice8_dispatcher.rs#L233); [engine::tests::slice8_dispatcher::pending_and_denied_approvals_never_execute_the_backend](../../../engine/tests/slice8_dispatcher.rs#L316); [engine::tests::slice8_dispatcher::skill_consumption_requires_a_prior_durable_offer_in_the_same_turn](../../../engine/tests/slice8_dispatcher.rs#L383); [engine::tests::slice8_dispatcher::dispatcher_derives_catalog_schema_and_backend_from_fixed_entry](../../../engine/tests/slice8_dispatcher.rs#L51); [engine::tests::slice8_dispatcher::dispatcher_rejects_unadvertised_invalid_and_non_durable_calls](../../../engine/tests/slice8_dispatcher.rs#L84); [profile::launch::LaunchBindings::validate_against](../../../profile/src/launch.rs#L235); [profile::launch::validate_dynamic_catalog](../../../profile/src/launch.rs#L290); [tekes-supervisor::client_extensions::ProductionClientExtensions::tools](../../../supervisor/src/client_extensions.rs#L2296); [tekes-supervisor::process_host::ProductionProcessHost::validate_workspace_policy_candidate](../../../supervisor/src/process_host.rs#L882); [tekes-supervisor::process_live_tests::real_public_image_attachment](../../../supervisor/src/process_live_tests.rs#L1118); [tekes-worker::main::run](../../../worker/src/main.rs#L181) |
| [tools::builtin::BuiltinManifest::decode_canonical](../../src/builtin.rs#L335) | `pub` | [tools::tests::slice4_gates::slice4_gate_43_builtin_tool_manifest_parity](../../tests/slice4_gates.rs#L179); [tools::tests::slice8_tool_runtime_oracle::slice8_gate_59_tool_dispatcher_catalog_coverage](../../tests/slice8_tool_runtime_oracle.rs#L82) |
| [tools::builtin::BuiltinManifest::validate](../../src/builtin.rs#L358) | `pub` | no resolved direct caller |
| [tools::builtin::BuiltinManifest::fixed_names](../../src/builtin.rs#L401) | `pub` | [tools::builtin::BuiltinManifest::reject_dynamic_collisions](../../src/builtin.rs#L405) |
| [tools::builtin::BuiltinManifest::reject_dynamic_collisions](../../src/builtin.rs#L405) | `pub` | no resolved direct caller |
| [tools::builtin::BuiltinManifest::projection](../../src/builtin.rs#L420) | `pub` | no resolved direct caller |
| [tools::builtin::BuiltinManifest::interactive_names](../../src/builtin.rs#L445) | `pub` | no resolved direct caller |
| [tools::builtin::BuiltinManifest::catalog_digest](../../src/builtin.rs#L465) | `pub` | no resolved direct caller |
| [tools::builtin::BuiltinError](../../src/builtin.rs#L504) | `pub` | not a function |
| [tools::builtin::index_by_name](../../src/builtin.rs#L513) | `pub` | no resolved direct caller |
| [tools::guidance::HARNESS_IDENTITY](../../src/guidance.rs#L18) | `pub` | not a function |
| [tools::guidance::CODING_PROFILE](../../src/guidance.rs#L22) | `pub` | not a function |
| [tools::guidance::GENERAL_PROFILE](../../src/guidance.rs#L25) | `pub` | not a function |
| [tools::guidance::IdentityProfile](../../src/guidance.rs#L31) | `pub` | not a function |
| [tools::guidance::IdentityProfile::as_str](../../src/guidance.rs#L39) | `pub` | no resolved direct caller |
| [tools::guidance::IdentityProfile::parse](../../src/guidance.rs#L47) | `pub` | [tekes-supervisor::endpoint_host::ProductionEndpointHost::create_session](../../../supervisor/src/endpoint_host.rs#L966) |
| [tools::guidance::WORKING_DIRECTORY](../../src/guidance.rs#L57) | `pub` | not a function |
| [tools::guidance::root_system_instructions](../../src/guidance.rs#L63) | `pub` | [tekes-worker::identity::tests::selection_is_durable_and_runtime_only](../../../worker/src/identity.rs#L275); [tekes-worker::provider_turn::run_provider_turn_inner](../../../worker/src/provider_turn.rs#L108) |
| [tools::guidance::guidance_oracle_value](../../src/guidance.rs#L89) | `pub` | [tools::guidance::canonical_guidance_oracle_bytes](../../src/guidance.rs#L109) |
| [tools::guidance::canonical_guidance_oracle_bytes](../../src/guidance.rs#L109) | `pub` | no resolved direct caller |
| [tools::helper::native_grep::search](../../src/helper/native_grep.rs#L8) | `pub(super)` | [tools::helper::HelperServer::execute_inner](../../src/helper.rs#L441) |
| [tools::helper::HELPER_PROTOCOL](../../src/helper.rs#L32) | `pub` | not a function |
| [tools::helper::HELPER_VERSION](../../src/helper.rs#L33) | `pub` | not a function |
| [tools::helper::EX_PROTOCOL](../../src/helper.rs#L34) | `pub` | not a function |
| [tools::helper::ByteString](../../src/helper.rs#L47) | `pub` | not a function |
| [tools::helper::ByteEncoding](../../src/helper.rs#L54) | `pub` | not a function |
| [tools::helper::ByteString::from_bytes](../../src/helper.rs#L61) | `pub` | [engine::system_tools::SystemToolBackend::write_content](../../../engine/src/system_tools.rs#L1210); [engine::system_tools::SystemToolBackend::apply_patch](../../../engine/src/system_tools.rs#L953); [engine::tests::slice8_system_tools::MemoryHelper::invoke](../../../engine/tests/slice8_system_tools.rs#L31); [tools::helper::HelperServer::execute_inner](../../src/helper.rs#L441); [tools::helper::HelperServer::exec](../../src/helper.rs#L553); [tools::tests::slice4_gates::slice4_gate_44_exec_helper_process_protocol](../../tests/slice4_gates.rs#L400); [tools::tests::slice4_gates::slice4_gate_10_descriptor_first_file_open](../../tests/slice4_gates.rs#L884) |
| [tools::helper::ByteString::decode](../../src/helper.rs#L74) | `pub` | no resolved direct caller |
| [tools::helper::CreateMode](../../src/helper.rs#L89) | `pub` | not a function |
| [tools::helper::ExecRequest](../../src/helper.rs#L97) | `pub` | not a function |
| [tools::helper::HelperPath](../../src/helper.rs#L115) | `pub` | not a function |
| [tools::helper::HelperOperation](../../src/helper.rs#L122) | `pub` | not a function |
| [tools::helper::HelperRequest](../../src/helper.rs#L161) | `pub` | not a function |
| [tools::helper::ReadValue](../../src/helper.rs#L168) | `pub` | not a function |
| [tools::helper::WriteValue](../../src/helper.rs#L176) | `pub` | not a function |
| [tools::helper::ExecValue](../../src/helper.rs#L183) | `pub` | not a function |
| [tools::helper::HelperValue](../../src/helper.rs#L193) | `pub` | not a function |
| [tools::helper::HelperErrorClass](../../src/helper.rs#L203) | `pub` | not a function |
| [tools::helper::HelperError](../../src/helper.rs#L223) | `pub` | not a function |
| [tools::helper::HelperClientError](../../src/helper.rs#L230) | `pub` | not a function |
| [tools::helper::HelperError::new](../../src/helper.rs#L239) | `pub` | [engine::tests::slice8_system_tools::MemoryHelper::invoke](../../../engine/tests/slice8_system_tools.rs#L31); [tools::helper::from_payload](../../src/helper.rs#L1020); [tools::helper::read_line_limited](../../src/helper.rs#L1025); [tools::helper::normalized_relative](../../src/helper.rs#L1053); [tools::helper::split_parent](../../src/helper.rs#L1076); [tools::helper::create_parent](../../src/helper.rs#L1106); [tools::helper::open_directory_at](../../src/helper.rs#L1143); [tools::helper::open_regular_at](../../src/helper.rs#L1165); [tools::helper::classify_open](../../src/helper.rs#L1197); [tools::helper::read_regular](../../src/helper.rs#L1205); [tools::helper::atomic_replace](../../src/helper.rs#L1228); [tools::helper::rename_at](../../src/helper.rs#L1280); [tools::helper::unlink_at](../../src/helper.rs#L1322); [tools::helper::glob_beneath](../../src/helper.rs#L1332); [tools::helper::checked_cap_with](../../src/helper.rs#L1463); [tools::helper::check_content_cap](../../src/helper.rs#L1475); [tools::helper::HelperError::io](../../src/helper.rs#L247); [tools::helper::encode_helper_line](../../src/helper.rs#L305); [tools::helper::decode_helper_line](../../src/helper.rs#L318); [tools::helper::RootBinding::open](../../src/helper.rs#L369); [tools::helper::HelperServer::new](../../src/helper.rs#L414); [tools::helper::HelperServer::execute_inner](../../src/helper.rs#L441); [tools::helper::HelperServer::root](../../src/helper.rs#L544); [tools::helper::HelperServer::exec](../../src/helper.rs#L553); [tools::helper::HelperClient::sandboxed_with_scratch](../../src/helper.rs#L690); [tools::helper::HelperClient::sandboxed](../../src/helper.rs#L723); [tools::helper::HelperClient::approved_unsandboxed](../../src/helper.rs#L746); [tools::helper::ByteString::decode](../../src/helper.rs#L74); [tools::helper::HelperClient::execute](../../src/helper.rs#L767); [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784); [tools::helper::serve_stdio](../../src/helper.rs#L942); [tools::helper::decode_terminal](../../src/helper.rs#L996) |
| [tools::helper::HelperResponse](../../src/helper.rs#L265) | `pub` | not a function |
| [tools::helper::encode_helper_line](../../src/helper.rs#L305) | `pub` | [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784); [tools::helper::serve_stdio](../../src/helper.rs#L942) |
| [tools::helper::decode_helper_line](../../src/helper.rs#L318) | `pub` | [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784); [tools::helper::serve_stdio](../../src/helper.rs#L942); [tools::helper::decode_terminal](../../src/helper.rs#L996); [tools::tests::slice4_gates::tool_contract_oracles](../../tests/slice4_gates.rs#L104) |
| [tools::helper::RootBinding](../../src/helper.rs#L362) | `pub` | not a function |
| [tools::helper::RootBinding::open](../../src/helper.rs#L369) | `pub` | [engine::tests::slice8_system_tools::shell_kills_a_sleeping_command_group_at_the_deadline_and_marks_the_step](../../../engine/tests/slice8_system_tools.rs#L280); [engine::tests::slice8_system_tools::shell_stops_before_a_step_when_earlier_steps_used_the_whole_budget](../../../engine/tests/slice8_system_tools.rs#L346); [tools::bin::tekes-helper::run](../../src/bin/tekes-helper.rs#L17); [tools::tests::slice4_gates::slice4_gate_10_descriptor_first_file_open](../../tests/slice4_gates.rs#L884) |
| [tools::helper::HelperServer](../../src/helper.rs#L409) | `pub` | not a function |
| [tools::helper::HelperServer::new](../../src/helper.rs#L414) | `pub` | [engine::tests::slice8_system_tools::shell_kills_a_sleeping_command_group_at_the_deadline_and_marks_the_step](../../../engine/tests/slice8_system_tools.rs#L280); [engine::tests::slice8_system_tools::shell_stops_before_a_step_when_earlier_steps_used_the_whole_budget](../../../engine/tests/slice8_system_tools.rs#L346); [tools::bin::tekes-helper::run](../../src/bin/tekes-helper.rs#L17); [tools::tests::slice4_gates::slice4_gate_10_descriptor_first_file_open](../../tests/slice4_gates.rs#L884) |
| [tools::helper::HelperServer::execute](../../src/helper.rs#L427) | `pub` | no resolved direct caller |
| [tools::helper::HelperClient](../../src/helper.rs#L672) | `pub` | not a function |
| [tools::helper::HelperClient::sandboxed_with_scratch](../../src/helper.rs#L690) | `pub` | [tools::tests::slice8_runtime_backends::managed_helper_scratch_survives_clones_and_is_removed_on_last_drop](../../tests/slice8_runtime_backends.rs#L555); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205) |
| [tools::helper::HelperClient::scratch_path](../../src/helper.rs#L716) | `pub` | no resolved direct caller |
| [tools::helper::HelperClient::sandboxed](../../src/helper.rs#L723) | `pub` | [tools::helper::HelperClient::sandboxed_with_scratch](../../src/helper.rs#L690); [tools::tests::slice4_gates::slice4_gate_44_exec_helper_process_protocol](../../tests/slice4_gates.rs#L400) |
| [tools::helper::HelperClient::approved_unsandboxed](../../src/helper.rs#L746) | `pub` | [tools::tests::slice8_runtime_backends::helper_client_cancellation_kills_the_process_group](../../tests/slice8_runtime_backends.rs#L32) |
| [tools::helper::HelperClient::execute](../../src/helper.rs#L767) | `pub` | no resolved direct caller |
| [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784) | `pub` | [tools::helper::HelperClient::execute](../../src/helper.rs#L767) |
| [tools::helper::serve_stdio](../../src/helper.rs#L942) | `pub` | [tools::bin::tekes-helper::run](../../src/bin/tekes-helper.rs#L17) |
| [tools::hook::HookPhase](../../src/hook.rs#L18) | `pub` | not a function |
| [tools::hook::HookResultView](../../src/hook.rs#L25) | `pub` | not a function |
| [tools::hook::HookRequest](../../src/hook.rs#L35) | `pub` | not a function |
| [tools::hook::HookRequest::validate](../../src/hook.rs#L47) | `pub` | no resolved direct caller |
| [tools::hook::PreVerdict](../../src/hook.rs#L66) | `pub` | not a function |
| [tools::hook::PostVerdict](../../src/hook.rs#L84) | `pub` | not a function |
| [tools::hook::HookResponse](../../src/hook.rs#L97) | `pub` | not a function |
| [tools::hook::HookFailureMode](../../src/hook.rs#L123) | `pub` | not a function |
| [tools::hook::HookBinding](../../src/hook.rs#L130) | `pub` | not a function |
| [tools::hook::HookBinding::validate](../../src/hook.rs#L144) | `pub` | no resolved direct caller |
| [tools::hook::decode_hook_binding](../../src/hook.rs#L165) | `pub` | [tools::lifecycle_hook::validate_instruction_hook](../../src/lifecycle_hook.rs#L76) |
| [tools::hook::HookError](../../src/hook.rs#L177) | `pub` | not a function |
| [tools::hook::encode_hook_line](../../src/hook.rs#L198) | `pub` | [tools::hook::ProcessHook::run](../../src/hook.rs#L270); [tools::lifecycle_hook::run_lifecycle_hook_with_cancel](../../src/lifecycle_hook.rs#L115); [tekes-worker::lifecycle_hooks::LifecycleHooks::invoke_one](../../../worker/src/lifecycle_hooks.rs#L96); [tekes-worker::provider_context_tests::install_lifecycle_test_hooks](../../../worker/src/provider_context_tests.rs#L267); [tekes-worker::provider_context_tests::lifecycle_observer_retries_unacknowledged_event_with_stable_identity](../../../worker/src/provider_context_tests.rs#L373) |
| [tools::hook::decode_hook_request](../../src/hook.rs#L204) | `pub` | [tools::tests::slice4_gates::tool_contract_oracles](../../tests/slice4_gates.rs#L104) |
| [tools::hook::decode_hook_response](../../src/hook.rs#L210) | `pub` | [tools::hook::ProcessHook::run](../../src/hook.rs#L270); [tools::tests::slice4_gates::tool_contract_oracles](../../tests/slice4_gates.rs#L104) |
| [tools::hook::decode_one_line](../../src/hook.rs#L248) | `pub(crate)` | [tools::hook::decode_hook_binding](../../src/hook.rs#L165); [tools::hook::decode_hook_request](../../src/hook.rs#L204); [tools::hook::decode_hook_response](../../src/hook.rs#L210); [tools::lifecycle_hook::run_lifecycle_hook_with_cancel](../../src/lifecycle_hook.rs#L115); [tools::lifecycle_hook::decode_lifecycle_hook_binding](../../src/lifecycle_hook.rs#L59); [tools::lifecycle_hook::validate_instruction_hook](../../src/lifecycle_hook.rs#L76) |
| [tools::hook::ProcessHook](../../src/hook.rs#L267) | `pub` | not a function |
| [tools::hook::ProcessHook::run](../../src/hook.rs#L270) | `pub` | [tools::pipeline::ToolPipeline::execute_terminal_with_gate_and_resume](../../src/pipeline.rs#L391); [tools::pipeline::ToolPipeline::complete_terminal](../../src/pipeline.rs#L506) |
| [tools::hook::run_hook_process](../../src/hook.rs#L283) | `pub(crate)` | [tools::hook::ProcessHook::run](../../src/hook.rs#L270); [tools::lifecycle_hook::run_lifecycle_hook_with_cancel](../../src/lifecycle_hook.rs#L115) |
| [tools::lifecycle_hook::LifecycleEvent](../../src/lifecycle_hook.rs#L9) | `pub` | not a function |
| [tools::lifecycle_hook::LifecycleHookBinding](../../src/lifecycle_hook.rs#L24) | `pub` | not a function |
| [tools::lifecycle_hook::decode_lifecycle_hook_binding](../../src/lifecycle_hook.rs#L59) | `pub` | [tools::lifecycle_hook::validate_instruction_hook](../../src/lifecycle_hook.rs#L76); [tekes-worker::lifecycle_hooks::LifecycleHooks::load](../../../worker/src/lifecycle_hooks.rs#L20) |
| [tools::lifecycle_hook::validate_instruction_hook](../../src/lifecycle_hook.rs#L76) | `pub` | [profile::instruction::validate_snapshot](../../../profile/src/instruction.rs#L850) |
| [tools::lifecycle_hook::LifecycleHookRequest](../../src/lifecycle_hook.rs#L87) | `pub` | not a function |
| [tools::lifecycle_hook::LifecycleHookResponse](../../src/lifecycle_hook.rs#L100) | `pub` | not a function |
| [tools::lifecycle_hook::run_lifecycle_hook](../../src/lifecycle_hook.rs#L108) | `pub` | no resolved direct caller |
| [tools::lifecycle_hook::run_lifecycle_hook_with_cancel](../../src/lifecycle_hook.rs#L115) | `pub` | [tools::lifecycle_hook::run_lifecycle_hook](../../src/lifecycle_hook.rs#L108); [tekes-worker::lifecycle_hooks::LifecycleHooks::invoke_one](../../../worker/src/lifecycle_hooks.rs#L96) |
| [tools::linux_sandbox::BACKEND](../../src/linux_sandbox.rs#L19) | `pub(crate)` | not a function |
| [tools::linux_sandbox::supported_abi](../../src/linux_sandbox.rs#L77) | `pub(crate)` | [tools::linux_sandbox::confine](../../src/linux_sandbox.rs#L107); [tools::sandbox::probe_linux](../../src/sandbox.rs#L317) |
| [tools::linux_sandbox::confine](../../src/linux_sandbox.rs#L107) | `pub(crate)` | [tools::sandbox::sandbox_command](../../src/sandbox.rs#L242) |
| [tools::pipeline::ToolExecution](../../src/pipeline.rs#L14) | `pub` | not a function |
| [tools::pipeline::BackendTerminal](../../src/pipeline.rs#L26) | `pub` | not a function |
| [tools::pipeline::ApprovalGate](../../src/pipeline.rs#L53) | `pub` | not a function |
| [tools::pipeline::DurableApprovalResponse](../../src/pipeline.rs#L64) | `pub` | not a function |
| [tools::pipeline::PipelineDecision](../../src/pipeline.rs#L72) | `pub` | not a function |
| [tools::pipeline::TOOL_CONTINUATION_SUBKIND](../../src/pipeline.rs#L91) | `pub` | not a function |
| [tools::pipeline::SecretScan](../../src/pipeline.rs#L94) | `pub` | not a function |
| [tools::pipeline::SecretScanner](../../src/pipeline.rs#L101) | `pub` | not a function |
| [tools::pipeline::SecretScanner::failing](../../src/pipeline.rs#L107) | `pub` | [tekes-supervisor::production_tool_control::continuation_policy_tests::pending_results_and_continuation_responses_pass_the_secret_policy](../../../supervisor/src/production_tool_control.rs#L1533); [tools::tests::slice4_gates::slice4_gate_07_mutated_tool_write_ahead](../../tests/slice4_gates.rs#L554) |
| [tools::pipeline::SecretScanner::scan](../../src/pipeline.rs#L113) | `pub` | no resolved direct caller |
| [tools::pipeline::ToolPipeline](../../src/pipeline.rs#L129) | `pub` | not a function |
| [tools::pipeline::ToolPipeline::new](../../src/pipeline.rs#L137) | `pub` | [engine::tests::slice8_dispatcher::generic_policy_runs_after_effective_argument_validation](../../../engine/tests/slice8_dispatcher.rs#L126); [engine::tests::slice8_dispatcher::production_policy_durably_parks_workspace_edits_before_backend_execution](../../../engine/tests/slice8_dispatcher.rs#L201); [engine::tests::slice8_dispatcher::granted_approval_resumes_durable_effective_invocation_exactly_once](../../../engine/tests/slice8_dispatcher.rs#L233); [engine::tests::slice8_dispatcher::pending_and_denied_approvals_never_execute_the_backend](../../../engine/tests/slice8_dispatcher.rs#L316); [engine::tests::slice8_dispatcher::skill_consumption_requires_a_prior_durable_offer_in_the_same_turn](../../../engine/tests/slice8_dispatcher.rs#L383); [engine::tests::slice8_dispatcher::dispatcher_derives_catalog_schema_and_backend_from_fixed_entry](../../../engine/tests/slice8_dispatcher.rs#L51); [engine::tests::slice8_dispatcher::dispatcher_rejects_unadvertised_invalid_and_non_durable_calls](../../../engine/tests/slice8_dispatcher.rs#L84); [engine::tests::slice8_dynamic_catalog::mcp_destructive_hint_reaches_the_engine_destructive_approval_gate](../../../engine/tests/slice8_dynamic_catalog.rs#L128); [engine::tests::slice8_dynamic_catalog::deferred_provider_projection_requires_a_visible_search_offer_and_backend](../../../engine/tests/slice8_dynamic_catalog.rs#L187); [engine::tests::slice8_dynamic_catalog::compaction_over_the_search_result_retires_the_offer](../../../engine/tests/slice8_dynamic_catalog.rs#L240); [engine::tests::slice8_dynamic_catalog::deferred_dispatch_requires_a_durable_visible_tool_search_offer](../../../engine/tests/slice8_dynamic_catalog.rs#L321); [engine::tests::slice8_dynamic_catalog::exact_dynamic_schema_and_dependency_are_checked_before_effect](../../../engine/tests/slice8_dynamic_catalog.rs#L365); [engine::tests::slice8_dynamic_catalog::dynamic_dispatch_uses_exact_metadata_and_the_common_policy_pipeline](../../../engine/tests/slice8_dynamic_catalog.rs#L82); [tools::tests::slice4_gates::execution_start_is_synced_before_backend_and_absent_while_approval_is_held](../../tests/slice4_gates.rs#L29); [tools::tests::slice4_gates::slice4_gate_07_mutated_tool_write_ahead](../../tests/slice4_gates.rs#L554); [tools::tests::slice4_gates::ordered_hooks_chain_mutations_and_compose_post_decisions](../../tests/slice4_gates.rs#L636); [tools::tests::slice4_gates::ordered_hooks_apply_open_and_closed_failure_modes_without_shortening_post_chain](../../tests/slice4_gates.rs#L705) |
| [tools::pipeline::ToolPipeline::execute](../../src/pipeline.rs#L145) | `pub` | no resolved direct caller |
| [tools::pipeline::ToolPipeline::verify_durable_execution](../../src/pipeline.rs#L166) | `pub` | no resolved direct caller |
| [tools::pipeline::ToolPipeline::verify_causal_offer](../../src/pipeline.rs#L229) | `pub` | no resolved direct caller |
| [tools::pipeline::ToolPipeline::has_causal_offer](../../src/pipeline.rs#L267) | `pub` | [tools::pipeline::ToolPipeline::verify_causal_offer](../../src/pipeline.rs#L229) |
| [tools::pipeline::ToolPipeline::execute_terminal](../../src/pipeline.rs#L353) | `pub` | [tools::pipeline::ToolPipeline::execute](../../src/pipeline.rs#L145) |
| [tools::pipeline::ToolPipeline::execute_terminal_with_gate](../../src/pipeline.rs#L365) | `pub` | [tools::pipeline::ToolPipeline::execute_terminal](../../src/pipeline.rs#L353) |
| [tools::pipeline::ToolPipeline::execute_terminal_with_gate_and_resume](../../src/pipeline.rs#L391) | `pub` | [tools::pipeline::ToolPipeline::execute_terminal_with_gate](../../src/pipeline.rs#L365) |
| [tools::pipeline::ToolPipeline::complete_continuation](../../src/pipeline.rs#L774) | `pub` | no resolved direct caller |
| [tools::pipeline::ToolPipeline::append_continuation_step](../../src/pipeline.rs#L787) | `pub` | [tools::pipeline::ToolPipeline::complete_terminal](../../src/pipeline.rs#L506) |
| [tools::pipeline::ToolPipelineError](../../src/pipeline.rs#L947) | `pub` | not a function |
| [tools::runtime_backends::HARD_HELPER_BYTES](../../src/runtime_backends.rs#L30) | `pub` | not a function |
| [tools::runtime_backends::HARD_HELPER_READ_BYTES](../../src/runtime_backends.rs#L31) | `pub` | not a function |
| [tools::runtime_backends::HARD_HELPER_TIMEOUT_MS](../../src/runtime_backends.rs#L32) | `pub` | not a function |
| [tools::runtime_backends::HARD_HTTP_BYTES](../../src/runtime_backends.rs#L34) | `pub` | not a function |
| [tools::runtime_backends::HARD_HTTP_TIMEOUT_MS](../../src/runtime_backends.rs#L35) | `pub` | not a function |
| [tools::runtime_backends::HARD_JOB_TAIL_BYTES](../../src/runtime_backends.rs#L36) | `pub` | not a function |
| [tools::runtime_backends::MAX_REDIRECTS](../../src/runtime_backends.rs#L37) | `pub` | not a function |
| [tools::runtime_backends::WEB_FETCH_ACCEPT](../../src/runtime_backends.rs#L38) | `pub` | not a function |
| [tools::runtime_backends::WEB_FETCH_MAX_CHARACTERS](../../src/runtime_backends.rs#L40) | `pub` | not a function |
| [tools::runtime_backends::WEB_FETCH_MIN_MEANINGFUL_CHARACTERS](../../src/runtime_backends.rs#L41) | `pub` | not a function |
| [tools::runtime_backends::WEB_FETCH_USER_AGENT](../../src/runtime_backends.rs#L42) | `pub` | not a function |
| [tools::runtime_backends::BackendOutcome](../../src/runtime_backends.rs#L45) | `pub` | not a function |
| [tools::runtime_backends::BackendHold](../../src/runtime_backends.rs#L52) | `pub` | not a function |
| [tools::runtime_backends::BackendUnavailable](../../src/runtime_backends.rs#L58) | `pub` | not a function |
| [tools::runtime_backends::BackendGate](../../src/runtime_backends.rs#L64) | `pub` | not a function |
| [tools::runtime_backends::BackendFailure](../../src/runtime_backends.rs#L90) | `pub` | not a function |
| [tools::runtime_backends::CancellationToken](../../src/runtime_backends.rs#L128) | `pub` | not a function |
| [tools::runtime_backends::CancellationToken::cancel](../../src/runtime_backends.rs#L131) | `pub` | no resolved direct caller |
| [tools::runtime_backends::CancellationToken::is_cancelled](../../src/runtime_backends.rs#L136) | `pub` | no resolved direct caller |
| [tools::runtime_backends::HelperInvoker](../../src/runtime_backends.rs#L141) | `pub` | not a function |
| [tools::runtime_backends::BoundedHelper](../../src/runtime_backends.rs#L192) | `pub` | not a function |
| [tools::runtime_backends::BoundedHelper::new](../../src/runtime_backends.rs#L198) | `pub` | [engine::system_tools::SystemToolBackend::new](../../../engine/src/system_tools.rs#L477); [tools::tests::slice8_runtime_backends::helper_adapter_preserves_gate_and_limits](../../tests/slice8_runtime_backends.rs#L124) |
| [tools::runtime_backends::BoundedHelper::invoke](../../src/runtime_backends.rs#L202) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobSpec](../../src/runtime_backends.rs#L294) | `pub` | not a function |
| [tools::runtime_backends::JobLaunchPolicy](../../src/runtime_backends.rs#L312) | `pub` | not a function |
| [tools::runtime_backends::JobLaunchPolicy::new](../../src/runtime_backends.rs#L319) | `pub` | [tekes-supervisor::process_host::frozen_tool_launch_policy](../../../supervisor/src/process_host.rs#L3873); [tekes-supervisor::process_host::private_validator_launch_policy](../../../supervisor/src/process_host.rs#L3940); [tools::tests::slice8_runtime_backends::test_job_policy](../../tests/slice8_runtime_backends.rs#L184) |
| [tools::runtime_backends::JobLaunchPolicy::primary_workspace_cwd](../../src/runtime_backends.rs#L358) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobLaunchPolicy::base_sandbox](../../src/runtime_backends.rs#L363) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobLaunchPolicy::permitted_write_roots](../../src/runtime_backends.rs#L368) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobState](../../src/runtime_backends.rs#L425) | `pub` | not a function |
| [tools::runtime_backends::JobRecord](../../src/runtime_backends.rs#L435) | `pub` | not a function |
| [tools::runtime_backends::SandboxedJobLauncher](../../src/runtime_backends.rs#L463) | `pub` | not a function |
| [tools::runtime_backends::HelperJobLauncher](../../src/runtime_backends.rs#L475) | `pub` | not a function |
| [tools::runtime_backends::HelperJobLauncher::new](../../src/runtime_backends.rs#L481) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::preflight_mandatory_authorities](../../../supervisor/src/process_host.rs#L2247); [tekes-supervisor::process_host::ProductionProcessHost::freeze_tool_authority](../../../supervisor/src/process_host.rs#L2286) |
| [tools::runtime_backends::HelperJobLauncher::disabled](../../src/runtime_backends.rs#L499) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::preflight_mandatory_authorities](../../../supervisor/src/process_host.rs#L2247); [tekes-supervisor::process_host::ProductionProcessHost::freeze_tool_authority](../../../supervisor/src/process_host.rs#L2286); [tools::runtime_backends::disabled_job_launcher_tests::mcp_only_launcher_refuses_command_execution](../../src/runtime_backends.rs#L2049) |
| [tools::runtime_backends::JobBroker](../../src/runtime_backends.rs#L528) | `pub` | not a function |
| [tools::runtime_backends::JobBroker::new](../../src/runtime_backends.rs#L535) | `pub` | [tekes-supervisor::production_tool_control::JobBrokerSupervisorAuthority::new](../../../supervisor/src/production_tool_control.rs#L188); [tools::runtime_backends::run_job_runner](../../src/runtime_backends.rs#L847); [tools::tests::slice8_runtime_backends::job_launch_policy_is_per_call_bounded_and_defaults_to_primary_workspace](../../tests/slice8_runtime_backends.rs#L203); [tools::tests::slice8_runtime_backends::job_survives_launching_supervisor_process_exit](../../tests/slice8_runtime_backends.rs#L285); [tools::tests::slice8_runtime_backends::job_supervisor_child](../../tests/slice8_runtime_backends.rs#L316); [tools::tests::slice8_runtime_backends::ownerless_job_is_durable_queryable_and_stoppable](../../tests/slice8_runtime_backends.rs#L346); [tools::tests::slice8_runtime_backends::job_output_tail_is_bounded](../../tests/slice8_runtime_backends.rs#L386) |
| [tools::runtime_backends::JobBroker::start](../../src/runtime_backends.rs#L558) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobBroker::status](../../src/runtime_backends.rs#L717) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobBroker::list](../../src/runtime_backends.rs#L748) | `pub` | no resolved direct caller |
| [tools::runtime_backends::JobBroker::stop](../../src/runtime_backends.rs#L768) | `pub` | no resolved direct caller |
| [tools::runtime_backends::run_job_runner](../../src/runtime_backends.rs#L847) | `pub` | [tools::bin::tekes-helper::run](../../src/bin/tekes-helper.rs#L17) |
| [tools::runtime_backends::HttpLimits](../../src/runtime_backends.rs#L945) | `pub` | not a function |
| [tools::runtime_backends::HttpFetchResult](../../src/runtime_backends.rs#L983) | `pub` | not a function |
| [tools::runtime_backends::WebExtractionKind](../../src/runtime_backends.rs#L993) | `pub` | not a function |
| [tools::runtime_backends::WebExtraction](../../src/runtime_backends.rs#L1004) | `pub` | not a function |
| [tools::runtime_backends::BoundedHttpClient](../../src/runtime_backends.rs#L1012) | `pub` | not a function |
| [tools::runtime_backends::BoundedHttpClient::new](../../src/runtime_backends.rs#L1017) | `pub` | [engine::tests::slice8_system_tools::slice8_gate_64_tool_policy_secret_and_network](../../../engine/tests/slice8_system_tools.rs#L682); [tools::tests::slice14e_web::slice14e_gate_110_web_network_policy_and_bounds](../../tests/slice14e_web.rs#L56); [tools::tests::slice8_runtime_backends::search_is_credential_conditional_and_bounded](../../tests/slice8_runtime_backends.rs#L446); [tools::tests::slice8_runtime_backends::cancellation_is_terminal_and_does_not_become_success](../../tests/slice8_runtime_backends.rs#L475) |
| [tools::runtime_backends::BoundedHttpClient::fetch](../../src/runtime_backends.rs#L1022) | `pub` | no resolved direct caller |
| [tools::runtime_backends::BoundedHttpClient::search](../../src/runtime_backends.rs#L1055) | `pub` | no resolved direct caller |
| [tools::runtime_backends::SearchTopic](../../src/runtime_backends.rs#L1104) | `pub` | not a function |
| [tools::runtime_backends::SearchRequest](../../src/runtime_backends.rs#L1110) | `pub` | not a function |
| [tools::runtime_backends::SearchHit](../../src/runtime_backends.rs#L1119) | `pub` | not a function |
| [tools::runtime_backends::SearchProvider](../../src/runtime_backends.rs#L1128) | `pub` | not a function |
| [tools::runtime_backends::resolve_web_redirect](../../src/runtime_backends.rs#L1230) | `pub` | [tools::runtime_backends::fetch_async](../../src/runtime_backends.rs#L1137) |
| [tools::runtime_backends::PublicRoute](../../src/runtime_backends.rs#L1265) | `pub` | not a function |
| [tools::runtime_backends::PublicRoute::client_builder](../../src/runtime_backends.rs#L1277) | `pub` | no resolved direct caller |
| [tools::runtime_backends::route_public_url](../../src/runtime_backends.rs#L1292) | `pub` | [tools::runtime_backends::fetch_async](../../src/runtime_backends.rs#L1137) |
| [tools::runtime_backends::route_public_url_with](../../src/runtime_backends.rs#L1297) | `pub` | [tools::runtime_backends::route_public_url](../../src/runtime_backends.rs#L1292) |
| [tools::runtime_backends::is_public_internet_address](../../src/runtime_backends.rs#L1488) | `pub` | [profile::config::validate_web_search_origin](../../../profile/src/config.rs#L1400); [tools::runtime_backends::route_public_url_with](../../src/runtime_backends.rs#L1297); [tools::runtime_backends::is_public_internet_address](../../src/runtime_backends.rs#L1488) |
| [tools::runtime_backends::extract_web_content](../../src/runtime_backends.rs#L1555) | `pub` | [tools::runtime_backends::fetch_async](../../src/runtime_backends.rs#L1137); [tools::tests::slice14e_web::slice14e_gate_109_web_fetch_extraction_chain](../../tests/slice14e_web.rs#L17) |
| [tools::sandbox::NetworkPolicy](../../src/sandbox.rs#L14) | `pub` | not a function |
| [tools::sandbox::SandboxPolicy](../../src/sandbox.rs#L22) | `pub` | not a function |
| [tools::sandbox::SandboxPolicy::validate](../../src/sandbox.rs#L33) | `pub` | [tools::sandbox::SandboxPolicy::canonical_bytes](../../src/sandbox.rs#L56) |
| [tools::sandbox::SandboxPolicy::canonical_bytes](../../src/sandbox.rs#L56) | `pub` | no resolved direct caller |
| [tools::sandbox::SandboxBackend](../../src/sandbox.rs#L65) | `pub` | not a function |
| [tools::sandbox::ProbeStatus](../../src/sandbox.rs#L72) | `pub` | not a function |
| [tools::sandbox::ProbeFailure](../../src/sandbox.rs#L79) | `pub` | not a function |
| [tools::sandbox::SandboxApproval](../../src/sandbox.rs#L87) | `pub` | not a function |
| [tools::sandbox::SandboxError](../../src/sandbox.rs#L95) | `pub` | not a function |
| [tools::sandbox::policy_digest](../../src/sandbox.rs#L110) | `pub` | [tools::helper::HelperClient::approved_unsandboxed](../../src/helper.rs#L746); [tools::tests::slice4_gates::slice4_gate_45_sandbox_probe_and_approval_binding](../../tests/slice4_gates.rs#L477); [tools::tests::slice8_runtime_backends::helper_client_cancellation_kills_the_process_group](../../tests/slice8_runtime_backends.rs#L32) |
| [tools::sandbox::compile_darwin_profile](../../src/sandbox.rs#L117) | `pub` | [tools::sandbox::sandbox_command](../../src/sandbox.rs#L242); [tools::tests::slice8_runtime_backends::declared_toolchain_python_runs_inside_sandbox](../../tests/slice8_runtime_backends.rs#L488) |
| [tools::sandbox::compile_linux_plan](../../src/sandbox.rs#L192) | `pub` | no resolved direct caller |
| [tools::sandbox::validate_unsandboxed_approval](../../src/sandbox.rs#L218) | `pub` | [tools::helper::HelperClient::approved_unsandboxed](../../src/helper.rs#L746); [tools::tests::slice4_gates::slice4_gate_45_sandbox_probe_and_approval_binding](../../tests/slice4_gates.rs#L477) |
| [tools::sandbox::probe_backend](../../src/sandbox.rs#L235) | `pub` | [tekes-supervisor::process_host::ProductionProcessHost::preflight_mandatory_authorities](../../../supervisor/src/process_host.rs#L2247); [tekes-supervisor::process_host::ProductionProcessHost::freeze_tool_authority](../../../supervisor/src/process_host.rs#L2286); [tools::linux_sandbox::tests::run](../../src/linux_sandbox.rs#L545); [tools::linux_sandbox::tests::loopback_only_network_is_refused](../../src/linux_sandbox.rs#L640); [tools::tests::slice4_gates::slice4_gate_44_exec_helper_process_protocol](../../tests/slice4_gates.rs#L400); [tools::tests::slice4_gates::slice4_gate_11_sandbox_backend_probe](../../tests/slice4_gates.rs#L994); [tekes-worker::workspace_edits::tests::actual_patch_helper_to_turn_ledger_round_trip](../../../worker/src/workspace_edits.rs#L205) |
| [tools::sandbox::sandbox_command](../../src/sandbox.rs#L242) | `pub(crate)` | [tools::helper::HelperClient::execute_cancellable](../../src/helper.rs#L784); [tools::linux_sandbox::tests::run](../../src/linux_sandbox.rs#L545); [tools::runtime_backends::HelperJobLauncher::runner_command](../../src/runtime_backends.rs#L517); [tools::sandbox::probe_confinement](../../src/sandbox.rs#L352) |
| [tools::schema_registry::FIXED_SCHEMA_REVISION](../../src/schema_registry.rs#L15) | `pub` | not a function |
| [tools::schema_registry::FixedToolSchema](../../src/schema_registry.rs#L18) | `pub` | not a function |
| [tools::schema_registry::ObjectSchema](../../src/schema_registry.rs#L26) | `pub` | not a function |
| [tools::schema_registry::PropertySchema](../../src/schema_registry.rs#L33) | `pub` | not a function |
| [tools::schema_registry::ValueSchema](../../src/schema_registry.rs#L39) | `pub` | not a function |
| [tools::schema_registry::ObjectRule](../../src/schema_registry.rs#L63) | `pub` | not a function |
| [tools::schema_registry::SchemaValidationError](../../src/schema_registry.rs#L96) | `pub` | not a function |
| [tools::schema_registry::FixedToolSchema::model_schema](../../src/schema_registry.rs#L105) | `pub` | [tools::schema_registry::FixedToolSchema::canonical_schema_bytes](../../src/schema_registry.rs#L114) |
| [tools::schema_registry::FixedToolSchema::canonical_schema_bytes](../../src/schema_registry.rs#L114) | `pub` | no resolved direct caller |
| [tools::schema_registry::FixedToolSchema::ordered_model_schema_bytes](../../src/schema_registry.rs#L123) | `pub` | no resolved direct caller |
| [tools::schema_registry::FixedToolSchema::schema_digest](../../src/schema_registry.rs#L129) | `pub` | no resolved direct caller |
| [tools::schema_registry::FixedToolSchema::validate](../../src/schema_registry.rs#L133) | `pub` | [tools::schema_registry::FixedToolSchema::repair_quoted_null_exclusive](../../src/schema_registry.rs#L141) |
| [tools::schema_registry::FixedToolSchema::repair_quoted_null_exclusive](../../src/schema_registry.rs#L141) | `pub` | no resolved direct caller |
| [tools::schema_registry::ObjectSchema::argument_order](../../src/schema_registry.rs#L280) | `pub` | no resolved direct caller |
| [tools::schema_registry::ObjectSchema::to_json_schema](../../src/schema_registry.rs#L288) | `pub` | no resolved direct caller |
| [tools::schema_registry::fixed_schema_registry](../../src/schema_registry.rs#L713) | `pub` | [tools::schema_registry::fixed_schema](../../src/schema_registry.rs#L1225); [tools::tests::slice8_tool_runtime_oracle::slice8_gate_59_tool_dispatcher_catalog_coverage](../../tests/slice8_tool_runtime_oracle.rs#L82) |
| [tools::schema_registry::schema_oracle_value](../../src/schema_registry.rs#L1002) | `pub` | [tools::schema_registry::canonical_schema_oracle_bytes](../../src/schema_registry.rs#L1017) |
| [tools::schema_registry::canonical_schema_oracle_bytes](../../src/schema_registry.rs#L1017) | `pub` | no resolved direct caller |
| [tools::schema_registry::fixed_schema](../../src/schema_registry.rs#L1225) | `pub` | [engine::dispatcher::ToolDispatcher::provider_catalog](../../../engine/src/dispatcher.rs#L176); [tekes-supervisor::client_extensions::ProductionClientExtensions::tools](../../../supervisor/src/client_extensions.rs#L2296); [tekes-supervisor::process_host::ProductionProcessHost::summary_for_manual_compaction](../../../supervisor/src/process_host.rs#L2989); [tools::schema_registry::validate_fixed_arguments](../../src/schema_registry.rs#L1231); [tekes-worker::compaction::run_compaction_summary](../../../worker/src/compaction.rs#L70); [tekes-worker::provider_context_tests::quoted_null_ready_terminal_and_reopen_keep_one_normalized_call](../../../worker/src/provider_context_tests.rs#L4161); [tekes-worker::provider_turn::repair_provider_call_arguments](../../../worker/src/provider_turn.rs#L1268) |
| [tools::schema_registry::validate_fixed_arguments](../../src/schema_registry.rs#L1231) | `pub` | [engine::dispatcher::ToolDispatcher::dispatch](../../../engine/src/dispatcher.rs#L193); [engine::system_tools::SystemToolBackend::execute](../../../engine/src/system_tools.rs#L1442); [tekes-supervisor::production_tool_control::ProductionToolControlHandler::execute_inner](../../../supervisor/src/production_tool_control.rs#L411); [tools::tests::slice8_tool_runtime_oracle::every_tool_case_is_canonical_schema_positive_and_exactly_classified](../../tests/slice8_tool_runtime_oracle.rs#L249); [tools::tests::slice8_tool_runtime_oracle::slice8_gate_59_tool_dispatcher_catalog_coverage](../../tests/slice8_tool_runtime_oracle.rs#L82) |
