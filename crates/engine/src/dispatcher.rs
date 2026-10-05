use schema::IJsonValue;
use serde_json::Value;
use thiserror::Error;
use tools::{
    ApprovalGate, BuiltinManifest, BuiltinTool, CatalogContext, DurableApprovalResponse, Effect,
    HookBinding, PipelineDecision, ToolExecution, ToolPipeline, ToolPipelineError, fixed_schema,
    validate_fixed_arguments,
};

use crate::ToolBackend;
use crate::permission_mode::{PermissionMode, read_permission_mode};

#[derive(Clone, Debug, PartialEq)]
pub struct ToolInvocation {
    pub thread: String,
    pub turn: u64,
    pub attempt: String,
    pub call: String,
    pub name: String,
    pub arguments: IJsonValue,
    pub timestamp: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalClass {
    ReadOnly,
    Edit,
    Execute,
    Destructive,
    WorkflowHold,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
    Hold { question: IJsonValue },
}

pub trait ToolPolicy {
    fn decide(
        &mut self,
        class: ApprovalClass,
        tool: &BuiltinTool,
        execution: &ToolExecution,
        effective_arguments: &IJsonValue,
    ) -> PolicyDecision;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AllowAllPolicy;

impl ToolPolicy for AllowAllPolicy {
    fn decide(
        &mut self,
        _class: ApprovalClass,
        _tool: &BuiltinTool,
        _execution: &ToolExecution,
        _effective_arguments: &IJsonValue,
    ) -> PolicyDecision {
        PolicyDecision::Allow
    }
}

/// The per-call-hold baseline: read-only tools run directly and workflow
/// holds are delegated to their backend. Every edit, execute, or destructive
/// call parks behind a durable per-call approval request. Production runs
/// [`PermissionModePolicy`]; this policy remains the hold-everything harness
/// the tests exercise.
#[derive(Clone, Copy, Debug, Default)]
pub struct ProductionToolPolicy;

impl ToolPolicy for ProductionToolPolicy {
    fn decide(
        &mut self,
        class: ApprovalClass,
        tool: &BuiltinTool,
        execution: &ToolExecution,
        _effective_arguments: &IJsonValue,
    ) -> PolicyDecision {
        match class {
            ApprovalClass::ReadOnly | ApprovalClass::WorkflowHold => PolicyDecision::Allow,
            ApprovalClass::Edit | ApprovalClass::Execute | ApprovalClass::Destructive => {
                per_call_hold(class, tool, execution)
            }
        }
    }
}

/// The durable per-call approval question shared by every holding policy.
fn per_call_hold(
    class: ApprovalClass,
    tool: &BuiltinTool,
    execution: &ToolExecution,
) -> PolicyDecision {
    let question = serde_json::json!({
        "call": execution.call,
        "tool": tool.name,
        "class": approval_scope(class),
    });
    let bytes =
        serde_json::to_vec(&question).expect("production approval question is serializable");
    PolicyDecision::Hold {
        question: IJsonValue::parse(&bytes).expect("production approval question is I-JSON"),
    }
}

/// The production policy: one durable per-session [`PermissionMode`] maps
/// each approval class to allow, deny, or a durable per-call hold. The class
/// taxonomy itself is never downgraded; only the decision per class changes.
///
/// - `read-only`: read-only and workflow holds run; edit, execute, and
///   destructive calls are denied.
/// - `workspace-write`: read-only, workflow, edit, and execute calls run
///   (execution is already confined by the immutable sandbox policy);
///   destructive calls park behind a per-call approval.
/// - `danger-full-access`: everything runs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PermissionModePolicy {
    pub mode: PermissionMode,
}

impl PermissionModePolicy {
    #[must_use]
    pub const fn new(mode: PermissionMode) -> Self {
        Self { mode }
    }

    /// Reads the session folder's durable mode. A corrupt record yields the
    /// default mode plus the diagnostic the caller should report.
    #[must_use]
    pub fn for_session_folder(session_folder: &std::path::Path) -> (Self, Option<String>) {
        let read = read_permission_mode(session_folder);
        (Self { mode: read.mode }, read.diagnostic)
    }
}

impl ToolPolicy for PermissionModePolicy {
    fn decide(
        &mut self,
        class: ApprovalClass,
        tool: &BuiltinTool,
        execution: &ToolExecution,
        _effective_arguments: &IJsonValue,
    ) -> PolicyDecision {
        match (self.mode, class) {
            (_, ApprovalClass::ReadOnly | ApprovalClass::WorkflowHold) => PolicyDecision::Allow,
            (PermissionMode::DangerFullAccess, _) => PolicyDecision::Allow,
            (PermissionMode::ReadOnly, _) => {
                PolicyDecision::Deny("read-only permission mode".to_owned())
            }
            (PermissionMode::WorkspaceWrite, ApprovalClass::Edit | ApprovalClass::Execute) => {
                PolicyDecision::Allow
            }
            (PermissionMode::WorkspaceWrite, ApprovalClass::Destructive) => {
                per_call_hold(class, tool, execution)
            }
        }
    }
}

pub struct ToolDispatcher<'a> {
    manifest: &'a BuiltinManifest,
    context: &'a CatalogContext,
}

impl<'a> ToolDispatcher<'a> {
    #[must_use]
    pub const fn new(manifest: &'a BuiltinManifest, context: &'a CatalogContext) -> Self {
        Self { manifest, context }
    }

    /// Produces the provider-facing fixed catalog for this launch. A tool is
    /// omitted unless both its catalog condition and its assembled backend are
    /// available, so an advertised schema is always executable.
    pub fn provider_catalog<B: ToolBackend>(
        &self,
        backend: &B,
    ) -> Result<Vec<IJsonValue>, DispatchError> {
        self.manifest
            .projection(self.context)
            .into_iter()
            .filter(|tool| backend.supports(&tool.name))
            .map(|tool| {
                let schema = fixed_schema(&tool.name)
                    .ok_or_else(|| DispatchError::UnknownTool(tool.name.clone()))?;
                let bytes = serde_json::to_vec(&schema.model_schema())?;
                Ok(IJsonValue::parse(&bytes)?)
            })
            .collect()
    }

    pub fn dispatch<B: ToolBackend, P: ToolPolicy>(
        &self,
        pipeline: &mut ToolPipeline<'_>,
        invocation: &ToolInvocation,
        hooks: &[HookBinding],
        policy: &mut P,
        backend: &mut B,
    ) -> Result<PipelineDecision, DispatchError> {
        let tool = self
            .manifest
            .tools
            .iter()
            .find(|tool| tool.name == invocation.name)
            .ok_or_else(|| DispatchError::UnknownTool(invocation.name.clone()))?;
        if !self
            .manifest
            .projection(self.context)
            .iter()
            .any(|visible| visible.name == tool.name)
        {
            return Err(DispatchError::UnavailableTool(invocation.name.clone()));
        }
        if !backend.supports(&tool.name) {
            return Err(DispatchError::UnavailableTool(invocation.name.clone()));
        }
        let arguments = serde_json::to_value(&invocation.arguments)?;
        validate_fixed_arguments(&tool.name, &arguments)?;
        let execution = ToolExecution {
            thread: invocation.thread.clone(),
            call: invocation.call.clone(),
            name: invocation.name.clone(),
            attempt: invocation.attempt.clone(),
            invocation: invocation.arguments.clone(),
            side_effectful: side_effectful(tool.effect),
            turn: invocation.turn,
            timestamp: invocation.timestamp.clone(),
        };
        pipeline.verify_durable_execution(&execution)?;
        if tool.name == "skill" {
            let skill = arguments
                .get("skill")
                .and_then(Value::as_str)
                .ok_or_else(|| DispatchError::UnavailableTool(tool.name.clone()))?;
            pipeline.verify_causal_offer(&execution, "skill_explorer", skill)?;
        }
        pipeline
            .execute_terminal_with_gate_and_resume(
                &execution,
                hooks,
                |execution, effective| {
                    let effective_value = match serde_json::to_value(effective) {
                        Ok(value) => value,
                        Err(error) => {
                            return ApprovalGate::Deny(format!(
                                "effective invocation is not JSON: {error}"
                            ));
                        }
                    };
                    if let Err(error) = validate_fixed_arguments(&tool.name, &effective_value) {
                        return ApprovalGate::Deny(format!(
                            "effective invocation failed validation: {error}"
                        ));
                    }
                    let effective_class = approval_class(tool, &effective_value);
                    match policy.decide(effective_class, tool, execution, effective) {
                        PolicyDecision::Allow => ApprovalGate::Allow,
                        PolicyDecision::Deny(reason) => ApprovalGate::Deny(reason),
                        PolicyDecision::Hold { question } => ApprovalGate::Hold {
                            scope: approval_scope(effective_class).to_owned(),
                            question,
                        },
                    }
                },
                |execution, effective, approval: Option<&DurableApprovalResponse>| match approval {
                    Some(approval) => backend.resume_after_approval(execution, effective, approval),
                    None => backend.execute(execution, effective),
                },
            )
            .map_err(DispatchError::Pipeline)
    }
}

#[must_use]
pub const fn side_effectful(effect: Effect) -> bool {
    !matches!(
        effect,
        Effect::None | Effect::ReadOnly | Effect::UserInteraction
    )
}

#[must_use]
pub fn approval_class(tool: &BuiltinTool, arguments: &Value) -> ApprovalClass {
    match tool.name.as_str() {
        "ask_user_questions" | "plan" => ApprovalClass::WorkflowHold,
        "apply_patch" => ApprovalClass::Edit,
        "shell" | "web_fetch" => ApprovalClass::Execute,
        "job" => match arguments.get("action").and_then(Value::as_str) {
            Some("list" | "status") => ApprovalClass::ReadOnly,
            Some("stop") => ApprovalClass::Destructive,
            _ => ApprovalClass::Execute,
        },
        "context" => match arguments.get("operation").and_then(Value::as_str) {
            Some("search" | "records" | "threads" | "read") => ApprovalClass::ReadOnly,
            _ => ApprovalClass::Destructive,
        },
        _ => match tool.effect {
            Effect::None | Effect::ReadOnly | Effect::NetworkRead => ApprovalClass::ReadOnly,
            Effect::WorkspaceWrite => ApprovalClass::Edit,
            Effect::ExternalProcess => ApprovalClass::Execute,
            Effect::UserInteraction => ApprovalClass::WorkflowHold,
            Effect::GoalState | Effect::ThreadControl | Effect::ChildSpawn => {
                ApprovalClass::Destructive
            }
        },
    }
}

pub const fn approval_scope(class: ApprovalClass) -> &'static str {
    match class {
        ApprovalClass::ReadOnly => "read_only",
        ApprovalClass::Edit => "edit",
        ApprovalClass::Execute => "execute",
        ApprovalClass::Destructive => "destructive",
        ApprovalClass::WorkflowHold => "workflow",
    }
}

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("unknown fixed tool: {0}")]
    UnknownTool(String),
    #[error("fixed tool is not available in this launch profile: {0}")]
    UnavailableTool(String),
    #[error(transparent)]
    Schema(#[from] tools::SchemaValidationError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    IJson(#[from] schema::SchemaError),
    #[error(transparent)]
    Pipeline(#[from] ToolPipelineError),
}

#[cfg(test)]
mod permission_mode_tests {
    use super::*;

    fn tool(name: &str) -> BuiltinTool {
        BuiltinManifest::compiled()
            .tools
            .into_iter()
            .find(|tool| tool.name == name)
            .expect("fixed tool")
    }

    fn execution(name: &str) -> ToolExecution {
        ToolExecution {
            thread: "t".to_owned(),
            call: "call-1".to_owned(),
            name: name.to_owned(),
            attempt: "a".to_owned(),
            invocation: IJsonValue::parse_str("{}").expect("i-json"),
            side_effectful: true,
            turn: 1,
            timestamp: "2026-09-14T00:00:00.000Z".to_owned(),
        }
    }

    fn decide(mode: PermissionMode, class: ApprovalClass) -> PolicyDecision {
        let tool = tool("shell");
        let arguments = IJsonValue::parse_str("{}").expect("i-json");
        PermissionModePolicy::new(mode).decide(class, &tool, &execution("shell"), &arguments)
    }

    const EVERY_CLASS: [ApprovalClass; 5] = [
        ApprovalClass::ReadOnly,
        ApprovalClass::WorkflowHold,
        ApprovalClass::Edit,
        ApprovalClass::Execute,
        ApprovalClass::Destructive,
    ];

    #[test]
    fn read_only_allows_reads_and_workflow_holds_and_denies_every_effect() {
        for class in [ApprovalClass::ReadOnly, ApprovalClass::WorkflowHold] {
            assert_eq!(
                decide(PermissionMode::ReadOnly, class),
                PolicyDecision::Allow
            );
        }
        for class in [
            ApprovalClass::Edit,
            ApprovalClass::Execute,
            ApprovalClass::Destructive,
        ] {
            assert_eq!(
                decide(PermissionMode::ReadOnly, class),
                PolicyDecision::Deny("read-only permission mode".to_owned()),
                "{class:?}"
            );
        }
    }

    #[test]
    fn workspace_write_allows_edit_and_execute_and_holds_destructive() {
        for class in [
            ApprovalClass::ReadOnly,
            ApprovalClass::WorkflowHold,
            ApprovalClass::Edit,
            ApprovalClass::Execute,
        ] {
            assert_eq!(
                decide(PermissionMode::WorkspaceWrite, class),
                PolicyDecision::Allow,
                "{class:?}"
            );
        }
        let PolicyDecision::Hold { question } =
            decide(PermissionMode::WorkspaceWrite, ApprovalClass::Destructive)
        else {
            panic!("destructive calls park behind a per-call approval");
        };
        let question: Value = serde_json::to_value(&question).expect("json");
        assert_eq!(
            question,
            serde_json::json!({"call":"call-1","tool":"shell","class":"destructive"})
        );
        // Same question shape as the per-call-hold baseline policy.
        let PolicyDecision::Hold { question: baseline } = ProductionToolPolicy.decide(
            ApprovalClass::Destructive,
            &tool("shell"),
            &execution("shell"),
            &IJsonValue::parse_str("{}").expect("i-json"),
        ) else {
            panic!("baseline holds");
        };
        let baseline: Value = serde_json::to_value(&baseline).expect("json");
        assert_eq!(question, baseline);
    }

    #[test]
    fn danger_full_access_allows_everything() {
        for class in EVERY_CLASS {
            assert_eq!(
                decide(PermissionMode::DangerFullAccess, class),
                PolicyDecision::Allow,
                "{class:?}"
            );
        }
    }

    #[test]
    fn default_mode_is_workspace_write_and_folder_read_reports_corruption() {
        assert_eq!(
            PermissionModePolicy::default().mode,
            PermissionMode::WorkspaceWrite
        );
        let folder = tempfile::tempdir().expect("folder");
        assert_eq!(
            PermissionModePolicy::for_session_folder(folder.path()),
            (
                PermissionModePolicy::new(PermissionMode::WorkspaceWrite),
                None
            )
        );
        crate::write_permission_mode(folder.path(), PermissionMode::ReadOnly).expect("write");
        assert_eq!(
            PermissionModePolicy::for_session_folder(folder.path())
                .0
                .mode,
            PermissionMode::ReadOnly
        );
        std::fs::write(crate::permission_mode_path(folder.path()), b"{").expect("corrupt");
        let (policy, diagnostic) = PermissionModePolicy::for_session_folder(folder.path());
        assert_eq!(policy.mode, PermissionMode::WorkspaceWrite);
        assert!(diagnostic.is_some());
    }
}
