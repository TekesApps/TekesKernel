# engine

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `CompactionSummary` | `compaction_summary::CompactionSummary` | `pub` |
| `MAX_CONTINUATION_BYTES` | `compaction_summary::MAX_CONTINUATION_BYTES` | `pub` |
| `SUMMARY_SYSTEM` | `compaction_summary::SUMMARY_SYSTEM` | `pub` |
| `SourceBundle` | `compaction_summary::SourceBundle` | `pub` |
| `SummaryArtifact` | `compaction_summary::SummaryArtifact` | `pub` |
| `admit_summary_artifact` | `compaction_summary::admit_summary_artifact` | `pub` |
| `freeze_source_bundle` | `compaction_summary::freeze_source_bundle` | `pub` |
| `seq_ranges` | `compaction_summary::seq_ranges` | `pub` |
| `summary_record` | `compaction_summary::summary_record` | `pub` |
| `summary_request_bytes` | `compaction_summary::summary_request_bytes` | `pub` |
| `summary_unavailable` | `compaction_summary::summary_unavailable` | `pub` |
| `BriefAtom` | `brief_contract::BriefAtom` | `pub` |
| `BriefAtomKind` | `brief_contract::BriefAtomKind` | `pub` |
| `BriefContract` | `brief_contract::BriefContract` | `pub` |
| `read_brief` | `brief_contract::read_brief` | `pub` |
| `first_post_compact_attempt` | `compact_gate::first_post_compact_attempt` | `pub` |
| `ContextCompactionPlan` | `context::ContextCompactionPlan` | `pub` |
| `TRIM_THRESHOLD_BYTES` | `context::TRIM_THRESHOLD_BYTES` | `pub` |
| `ToolResultTrim` | `context::ToolResultTrim` | `pub` |
| `compaction_anchor_sequences` | `context::compaction_anchor_sequences` | `pub` |
| `plan_context_compaction` | `context::plan_context_compaction` | `pub` |
| `plan_tool_result_trim` | `context::plan_tool_result_trim` | `pub` |
| `trimmed_tool_result_content` | `context::trimmed_tool_result_content` | `pub` |
| `AdmissionLease` | `admission::AdmissionLease` | `pub` |
| `AdmissionPool` | `admission::AdmissionPool` | `pub` |
| `LeaseRelease` | `admission::LeaseRelease` | `pub` |
| `CreateIndex` | `delivery::CreateIndex` | `pub` |
| `CreateResolution` | `delivery::CreateResolution` | `pub` |
| `DeliveryIndex` | `delivery::DeliveryIndex` | `pub` |
| `DeliveryResolution` | `delivery::DeliveryResolution` | `pub` |
| `AllowAllPolicy` | `dispatcher::AllowAllPolicy` | `pub` |
| `ApprovalClass` | `dispatcher::ApprovalClass` | `pub` |
| `DispatchError` | `dispatcher::DispatchError` | `pub` |
| `PermissionModePolicy` | `dispatcher::PermissionModePolicy` | `pub` |
| `PolicyDecision` | `dispatcher::PolicyDecision` | `pub` |
| `ProductionToolPolicy` | `dispatcher::ProductionToolPolicy` | `pub` |
| `ToolDispatcher` | `dispatcher::ToolDispatcher` | `pub` |
| `ToolInvocation` | `dispatcher::ToolInvocation` | `pub` |
| `ToolPolicy` | `dispatcher::ToolPolicy` | `pub` |
| `approval_class` | `dispatcher::approval_class` | `pub` |
| `approval_scope` | `dispatcher::approval_scope` | `pub` |
| `side_effectful` | `dispatcher::side_effectful` | `pub` |
| `DynamicBackend` | `dynamic_catalog::DynamicBackend` | `pub` |
| `DynamicDispatchError` | `dynamic_catalog::DynamicDispatchError` | `pub` |
| `DynamicSupervisorBackend` | `dynamic_catalog::DynamicSupervisorBackend` | `pub` |
| `DynamicToolDispatcher` | `dynamic_catalog::DynamicToolDispatcher` | `pub` |
| `dynamic_approval_class` | `dynamic_catalog::dynamic_approval_class` | `pub` |
| `dynamic_side_effectful` | `dynamic_catalog::dynamic_side_effectful` | `pub` |
| `dynamic_workflow_backend` | `dynamic_catalog::dynamic_workflow_backend` | `pub` |
| `validate_dynamic_invocation` | `dynamic_catalog::validate_dynamic_invocation` | `pub` |
| `ArchiveAction` | `lifecycle::ArchiveAction` | `pub` |
| `DeliveryAction` | `lifecycle::DeliveryAction` | `pub` |
| `EnsureAction` | `lifecycle::EnsureAction` | `pub` |
| `LockFacts` | `lifecycle::LockFacts` | `pub` |
| `RunDecision` | `lifecycle::RunDecision` | `pub` |
| `RunMode` | `lifecycle::RunMode` | `pub` |
| `TailState` | `lifecycle::TailState` | `pub` |
| `archive_action` | `lifecycle::archive_action` | `pub` |
| `classify` | `lifecycle::classify` | `pub` |
| `delivery_action` | `lifecycle::delivery_action` | `pub` |
| `ensure_action` | `lifecycle::ensure_action` | `pub` |
| `ensure_action_at` | `lifecycle::ensure_action_at` | `pub` |
| `run_decision` | `lifecycle::run_decision` | `pub` |
| `PERMISSION_MODE_FILE` | `permission_mode::PERMISSION_MODE_FILE` | `pub` |
| `PermissionMode` | `permission_mode::PermissionMode` | `pub` |
| `PermissionModeRead` | `permission_mode::PermissionModeRead` | `pub` |
| `permission_mode_path` | `permission_mode::permission_mode_path` | `pub` |
| `read_permission_mode` | `permission_mode::read_permission_mode` | `pub` |
| `write_permission_mode` | `permission_mode::write_permission_mode` | `pub` |
| `AdapterCapabilities` | `provider::AdapterCapabilities` | `pub` |
| `Continuation` | `provider::Continuation` | `pub` |
| `QueryCapability` | `provider::QueryCapability` | `pub` |
| `QueryResult` | `provider::QueryResult` | `pub` |
| `RecoveryDecision` | `provider::RecoveryDecision` | `pub` |
| `SentState` | `provider::SentState` | `pub` |
| `decide_recovery` | `provider::decide_recovery` | `pub` |
| `sent_state` | `provider::sent_state` | `pub` |
| `FakeProcessHost` | `seams::FakeProcessHost` | `pub` |
| `FakeProvider` | `seams::FakeProvider` | `pub` |
| `FakeToolBackend` | `seams::FakeToolBackend` | `pub` |
| `ProcessHost` | `seams::ProcessHost` | `pub` |
| `ProcessState` | `seams::ProcessState` | `pub` |
| `ProviderAdapter` | `seams::ProviderAdapter` | `pub` |
| `ProviderTerminal` | `seams::ProviderTerminal` | `pub` |
| `ToolBackend` | `seams::ToolBackend` | `pub` |
| `ToolBackendRouter` | `seams::ToolBackendRouter` | `pub` |
| `StopNode` | `stop::StopNode` | `pub` |
| `StopTree` | `stop::StopTree` | `pub` |
| `DrainAction` | `supervisor::DrainAction` | `pub` |
| `DrainTracker` | `supervisor::DrainTracker` | `pub` |
| `RECOVERY_ORDER` | `supervisor::RECOVERY_ORDER` | `pub` |
| `RecoveryStage` | `supervisor::RecoveryStage` | `pub` |
| `SupervisorControlBackend` | `supervisor_tools::SupervisorControlBackend` | `pub` |
| `ArtifactVersionAuthority` | `system_tools::ArtifactVersionAuthority` | `pub` |
| `ArtifactVersionPermit` | `system_tools::ArtifactVersionPermit` | `pub` |
| `CommittedEditRecorder` | `system_tools::CommittedEditRecorder` | `pub` |
| `DEFAULT_SHELL_DURATION_MS` | `system_tools::DEFAULT_SHELL_DURATION_MS` | `pub` |
| `DurableArtifactVersions` | `system_tools::DurableArtifactVersions` | `pub` |
| `MAX_SHELL_DURATION_MS` | `system_tools::MAX_SHELL_DURATION_MS` | `pub` |
| `RootMount` | `system_tools::RootMount` | `pub` |
| `SystemToolBackend` | `system_tools::SystemToolBackend` | `pub` |
| `SystemToolConfig` | `system_tools::SystemToolConfig` | `pub` |
| `web_fetch_result_value` | `system_tools::web_fetch_result_value` | `pub` |
| `execute_tool` | `tool::execute_tool` | `pub` |
| `DurableApprovalResponse` | `tools::DurableApprovalResponse` | `pub` |
| `HookBinding` | `tools::HookBinding` | `pub` |
| `PipelineDecision` | `tools::PipelineDecision` | `pub` |
| `SecretScanner` | `tools::SecretScanner` | `pub` |
| `ToolExecution` | `tools::ToolExecution` | `pub` |
| `ToolPipeline` | `tools::ToolPipeline` | `pub` |
| `ToolPipelineError` | `tools::ToolPipelineError` | `pub` |
| `AttemptFlow` | `transactions::AttemptFlow` | `pub` |
| `AttemptPhase` | `transactions::AttemptPhase` | `pub` |
| `DeliveryCommit` | `transactions::DeliveryCommit` | `pub` |
| `DeliveryPhase` | `transactions::DeliveryPhase` | `pub` |
| `OutcomeCommit` | `transactions::OutcomeCommit` | `pub` |
| `OutcomePhase` | `transactions::OutcomePhase` | `pub` |
| `TransactionError` | `transactions::TransactionError` | `pub` |
| `ValidationBinding` | `validation::ValidationBinding` | `pub` |
| `ValidationDecision` | `validation::ValidationDecision` | `pub` |
| `ValidationNegative` | `validation::ValidationNegative` | `pub` |
| `ValidationOutcome` | `validation::ValidationOutcome` | `pub` |
| `ValidationSignal` | `validation::ValidationSignal` | `pub` |
| `ValidationVerdict` | `validation::ValidationVerdict` | `pub` |
| `validation_decision` | `validation::validation_decision` | `pub` |
| `begin_validation` | `validation_writer::begin_validation` | `pub` |
| `commit_validation_decision` | `validation_writer::commit_validation_decision` | `pub` |
| `materialize_validation_settlement` | `validation_writer::materialize_validation_settlement` | `pub` |
| `CatalogEntry` | `workflow_tools::CatalogEntry` | `pub` |
| `WorkflowBackend` | `workflow_tools::WorkflowBackend` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `engine::brief_contract` | `private` |  |
| `engine::compaction_summary` | `private` |  |
| `engine::compact_gate` | `private` |  |
| `engine::context` | `private` |  |
| `engine::admission` | `private` |  |
| `engine::delivery` | `private` |  |
| `engine::dispatcher` | `private` |  |
| `engine::dynamic_catalog` | `private` |  |
| `engine::lifecycle` | `private` |  |
| `engine::permission_mode` | `private` |  |
| `engine::provider` | `private` |  |
| `engine::seams` | `private` |  |
| `engine::stop` | `private` |  |
| `engine::supervisor` | `private` |  |
| `engine::supervisor_tools` | `private` |  |
| `engine::system_tools` | `private` |  |
| `engine::tool` | `private` |  |
| `engine::transactions` | `private` |  |
| `engine::validation` | `private` |  |
| `engine::validation_writer` | `private` |  |
| `engine::workflow_tools` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
