//! Deterministic lifecycle, admission, delivery, and recovery decisions.
mod brief_contract;
mod compaction_summary;
pub use compaction_summary::{
    CompactionSummary, MAX_CONTINUATION_BYTES, SUMMARY_SYSTEM, SourceBundle, SummaryArtifact,
    admit_summary_artifact, freeze_source_bundle, seq_ranges, summary_record,
    summary_request_bytes, summary_unavailable,
};
mod compact_gate;
mod context;
pub use brief_contract::{BriefAtom, BriefAtomKind, BriefContract, read_brief};
pub use compact_gate::first_post_compact_attempt;
pub use context::{
    ContextCompactionPlan, TRIM_THRESHOLD_BYTES, ToolResultTrim, compaction_anchor_sequences,
    plan_context_compaction, plan_tool_result_trim, trimmed_tool_result_content,
};

mod admission;
mod delivery;
mod dispatcher;
mod dynamic_catalog;
mod lifecycle;
mod permission_mode;
mod provider;
mod seams;
mod stop;
mod supervisor;
mod supervisor_tools;
mod system_tools;
mod tool;
mod transactions;
mod validation;
mod validation_writer;
mod workflow_tools;

pub use admission::{AdmissionLease, AdmissionPool, LeaseRelease};
pub use delivery::{CreateIndex, CreateResolution, DeliveryIndex, DeliveryResolution};
pub use dispatcher::{
    AllowAllPolicy, ApprovalClass, DispatchError, PermissionModePolicy, PolicyDecision,
    ProductionToolPolicy, ToolDispatcher, ToolInvocation, ToolPolicy, approval_class,
    approval_scope, side_effectful,
};
pub use dynamic_catalog::{
    DynamicBackend, DynamicDispatchError, DynamicSupervisorBackend, DynamicToolDispatcher,
    dynamic_approval_class, dynamic_side_effectful, dynamic_workflow_backend,
    validate_dynamic_invocation,
};
pub use lifecycle::{
    ArchiveAction, DeliveryAction, EnsureAction, LockFacts, RunDecision, RunMode, TailState,
    archive_action, classify, delivery_action, ensure_action, ensure_action_at, run_decision,
};
pub use permission_mode::{
    PERMISSION_MODE_FILE, PermissionMode, PermissionModeRead, permission_mode_path,
    read_permission_mode, write_permission_mode,
};
pub use provider::{
    AdapterCapabilities, Continuation, QueryCapability, QueryResult, RecoveryDecision, SentState,
    decide_recovery, sent_state,
};
pub use seams::{
    FakeProcessHost, FakeProvider, FakeToolBackend, ProcessHost, ProcessState, ProviderAdapter,
    ProviderTerminal, ToolBackend, ToolBackendRouter,
};
pub use stop::{StopNode, StopTree};
pub use supervisor::{DrainAction, DrainTracker, RECOVERY_ORDER, RecoveryStage};
pub use supervisor_tools::SupervisorControlBackend;
pub use system_tools::{
    ArtifactVersionAuthority, ArtifactVersionPermit, CommittedEditRecorder,
    DEFAULT_SHELL_DURATION_MS, DurableArtifactVersions, MAX_SHELL_DURATION_MS, RootMount,
    SystemToolBackend, SystemToolConfig, web_fetch_result_value,
};
pub use tool::execute_tool;
pub use tools::{
    DurableApprovalResponse, HookBinding, PipelineDecision, SecretScanner, ToolExecution,
    ToolPipeline, ToolPipelineError,
};
pub use transactions::{
    AttemptFlow, AttemptPhase, DeliveryCommit, DeliveryPhase, OutcomeCommit, OutcomePhase,
    TransactionError,
};
pub use validation::{
    ValidationBinding, ValidationDecision, ValidationNegative, ValidationOutcome, ValidationSignal,
    ValidationVerdict, validation_decision,
};
pub use validation_writer::{
    begin_validation, commit_validation_decision, materialize_validation_settlement,
};
pub use workflow_tools::{CatalogEntry, WorkflowBackend};
