use std::collections::BTreeMap;

use profile::{DynamicTool, DynamicToolCatalog, DynamicToolEffect};
use schema::IJsonValue;
use serde_json::Value;
use thiserror::Error;
use tools::{
    Availability, Backend, BackendTerminal, BuiltinTool, Effect, HookBinding, PipelineDecision,
    ToolExecution, ToolPipeline, ToolPipelineError,
};
use worker_control::{ToolControl, ToolControlErrorCode, ToolControlResult};

use crate::{
    ApprovalClass, CatalogEntry, PolicyDecision, ToolInvocation, ToolPolicy, WorkflowBackend,
};

/// Backend seam for a declared dynamic tool. The complete immutable entry is
/// supplied on both capability probing and execution so routing cannot be
/// reconstructed from a name or mutable registry.
pub trait DynamicBackend {
    fn supports(&self, tool: &DynamicTool) -> bool;

    fn execute(
        &mut self,
        tool: &DynamicTool,
        execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> tools::BackendTerminal;
}

/// Correlated dynamic route over the same worker-control durable request/result
/// pair used by fixed supervisor-owned tools. Catalog membership, not a mutable
/// daemon registry, decides which names can reach this adapter.
pub struct DynamicSupervisorBackend<F> {
    session: String,
    exchange: F,
}

impl<F> DynamicSupervisorBackend<F> {
    #[must_use]
    pub fn new(session: impl Into<String>, exchange: F) -> Self {
        Self {
            session: session.into(),
            exchange,
        }
    }
}

impl<F> DynamicBackend for DynamicSupervisorBackend<F>
where
    F: FnMut(&ToolControl) -> Result<ToolControlResult, String>,
{
    fn supports(&self, _tool: &DynamicTool) -> bool {
        true
    }

    fn execute(
        &mut self,
        tool: &DynamicTool,
        execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> BackendTerminal {
        if !self.supports(tool) {
            return BackendTerminal::Unavailable {
                code: "dynamic_supervisor_unavailable".to_owned(),
                message: format!("supervisor route is unavailable for {}", tool.name),
                retryable: false,
            };
        }
        let request = match ToolControl::new(
            self.session.clone(),
            execution.thread.clone(),
            execution.turn,
            execution.call.clone(),
            tool.name.clone(),
            invocation.clone(),
        ) {
            Ok(request) => request,
            Err(error) => {
                return BackendTerminal::Unavailable {
                    code: "protocol".to_owned(),
                    message: error.to_string(),
                    retryable: false,
                };
            }
        };
        let result = match (self.exchange)(&request) {
            Ok(result) => result,
            Err(message) => {
                return BackendTerminal::Unavailable {
                    code: "transport".to_owned(),
                    message,
                    retryable: true,
                };
            }
        };
        if let Err(error) = result.validate_for(&request) {
            return BackendTerminal::Unavailable {
                code: "protocol".to_owned(),
                message: error.to_string(),
                retryable: false,
            };
        }
        match (result.value, result.error, result.pending) {
            (Some(value), None, None) => BackendTerminal::Completed(value),
            (None, Some(error), None) => BackendTerminal::Unavailable {
                code: dynamic_control_error_code(error.code).to_owned(),
                message: error.message,
                retryable: error.retryable,
            },
            (None, None, Some(pending)) => BackendTerminal::Pending {
                continuation_id: pending.continuation_id,
                state: pending.state,
            },
            _ => BackendTerminal::Unavailable {
                code: "protocol".to_owned(),
                message: "invalid tool-control result union".to_owned(),
                retryable: false,
            },
        }
    }
}

const fn dynamic_control_error_code(code: ToolControlErrorCode) -> &'static str {
    match code {
        ToolControlErrorCode::Unsupported => "unsupported",
        ToolControlErrorCode::Denied => "denied",
        ToolControlErrorCode::NotFound => "not_found",
        ToolControlErrorCode::Conflict => "conflict",
        ToolControlErrorCode::Unavailable => "unavailable",
        ToolControlErrorCode::Timeout => "timeout",
        ToolControlErrorCode::EffectUnknown => "effect_unknown",
        ToolControlErrorCode::EffectConflicted => "effect_conflicted",
        ToolControlErrorCode::Internal => "internal",
    }
}

/// Shared validator used on both sides of the dynamic supervisor route. The
/// worker validates before sending; the supervisor repeats it before effect.
pub fn validate_dynamic_invocation(
    tool: &DynamicTool,
    arguments: &IJsonValue,
) -> Result<(), DynamicDispatchError> {
    validate_dynamic_arguments(tool, arguments)
}

pub struct DynamicToolDispatcher<'a> {
    catalog: &'a DynamicToolCatalog,
    by_name: BTreeMap<&'a str, &'a DynamicTool>,
}

impl<'a> DynamicToolDispatcher<'a> {
    #[must_use]
    pub fn new(catalog: &'a DynamicToolCatalog) -> Self {
        Self {
            catalog,
            by_name: catalog
                .tools
                .iter()
                .map(|tool| (tool.name.as_str(), tool))
                .collect(),
        }
    }

    /// Resident dynamic schemas, preserving the catalog's `(source,name)`
    /// order and omitting any missing backend dependency.
    #[must_use]
    pub fn provider_catalog<B: DynamicBackend>(&self, backend: &B) -> Vec<IJsonValue> {
        self.catalog
            .tools
            .iter()
            .filter(|tool| tool.always_on && backend.supports(tool))
            .map(|tool| tool.schema.clone())
            .collect()
    }

    /// The schemas a request before `before_seq` declares: every always-on
    /// tool the backend supports plus every deferred tool with a
    /// model-visible `tool_search` offer. The offer outlives its turn, so the
    /// declared catalog only grows between compactions and the provider's
    /// cached prefix survives a turn boundary.
    pub fn provider_catalog_visible<B: DynamicBackend>(
        &self,
        backend: &B,
        pipeline: &ToolPipeline<'_>,
        before_seq: u64,
    ) -> Result<Vec<IJsonValue>, ToolPipelineError> {
        let mut schemas = Vec::new();
        for tool in &self.catalog.tools {
            if backend.supports(tool)
                && (tool.always_on
                    || pipeline.has_causal_offer(before_seq, "tool_search", &tool.name)?)
            {
                schemas.push(tool.schema.clone());
            }
        }
        Ok(schemas)
    }

    /// Complete client-facing catalog. This does not make deferred schemas
    /// model-visible and therefore cannot substitute for a causal offer.
    #[must_use]
    pub fn complete_catalog<B: DynamicBackend>(&self, backend: &B) -> Vec<IJsonValue> {
        self.catalog
            .tools
            .iter()
            .filter(|tool| backend.supports(tool))
            .map(|tool| tool.schema.clone())
            .collect()
    }

    /// SearchFrame input for the fixed `tool_search` backend. The schema body
    /// remains in the immutable launch catalog; only bounded descriptors are
    /// handed to discovery.
    pub fn deferred_search_entries<B: DynamicBackend>(
        &self,
        backend: &B,
    ) -> Result<Vec<CatalogEntry>, DynamicDispatchError> {
        self.catalog
            .tools
            .iter()
            .filter(|tool| !tool.always_on && backend.supports(tool))
            .map(|tool| {
                Ok(CatalogEntry {
                    name: tool.name.clone(),
                    summary: schema_description(tool)?,
                    aliases: tool.aliases.clone(),
                    schema_digest: tool.schema_digest.clone(),
                    content: tool.schema.clone(),
                })
            })
            .collect()
    }

    pub fn dispatch<B: DynamicBackend, P: ToolPolicy>(
        &self,
        pipeline: &mut ToolPipeline<'_>,
        invocation: &ToolInvocation,
        hooks: &[HookBinding],
        policy: &mut P,
        backend: &mut B,
    ) -> Result<PipelineDecision, DynamicDispatchError> {
        let tool = self
            .by_name
            .get(invocation.name.as_str())
            .copied()
            .ok_or_else(|| DynamicDispatchError::UnknownTool(invocation.name.clone()))?;
        if !backend.supports(tool) {
            return Err(DynamicDispatchError::UnavailableTool(tool.name.clone()));
        }
        validate_dynamic_arguments(tool, &invocation.arguments)?;
        let execution = ToolExecution {
            thread: invocation.thread.clone(),
            call: invocation.call.clone(),
            name: invocation.name.clone(),
            attempt: invocation.attempt.clone(),
            invocation: invocation.arguments.clone(),
            side_effectful: dynamic_side_effectful(tool.effect),
            turn: invocation.turn,
            timestamp: invocation.timestamp.clone(),
        };
        pipeline.verify_durable_execution(&execution)?;
        if !tool.always_on {
            pipeline.verify_causal_offer(&execution, "tool_search", &tool.name)?;
        }

        let policy_tool = policy_descriptor(tool);
        pipeline
            .execute_terminal_with_gate(
                &execution,
                hooks,
                |execution, effective| {
                    if let Err(error) = validate_dynamic_arguments(tool, effective) {
                        return tools::ApprovalGate::Deny(format!(
                            "effective dynamic invocation failed validation: {error}"
                        ));
                    }
                    match policy.decide(
                        dynamic_approval_class(tool.effect),
                        &policy_tool,
                        execution,
                        effective,
                    ) {
                        PolicyDecision::Allow => tools::ApprovalGate::Allow,
                        PolicyDecision::Deny(reason) => tools::ApprovalGate::Deny(reason),
                        PolicyDecision::Hold { question } => tools::ApprovalGate::Hold {
                            scope: dynamic_approval_scope(tool.effect).to_owned(),
                            question,
                        },
                    }
                },
                |execution, effective| backend.execute(tool, execution, effective),
            )
            .map_err(DynamicDispatchError::Pipeline)
    }
}

/// Constructs the workflow discovery backend from the exact declared catalog;
/// useful to worker assembly without exposing a second registry.
pub fn dynamic_workflow_backend(
    root: impl Into<std::path::PathBuf>,
    workspace: impl Into<String>,
    goal_id: Option<String>,
    skills: Vec<CatalogEntry>,
    catalog: &DynamicToolDispatcher<'_>,
    backend: &impl DynamicBackend,
) -> Result<WorkflowBackend, DynamicDispatchError> {
    WorkflowBackend::new(
        root,
        workspace,
        goal_id,
        skills,
        catalog.deferred_search_entries(backend)?,
    )
    .map_err(DynamicDispatchError::Assembly)
}

fn policy_descriptor(tool: &DynamicTool) -> BuiltinTool {
    BuiltinTool {
        arguments: parameter_names(tool),
        availability: Availability::SelectionControlled,
        backend: Backend::SupervisorControl,
        effect: match tool.effect {
            DynamicToolEffect::ReadOnly => Effect::ReadOnly,
            DynamicToolEffect::WorkspaceWrite => Effect::WorkspaceWrite,
            DynamicToolEffect::NetworkRead => Effect::NetworkRead,
            DynamicToolEffect::ExternalProcess
            | DynamicToolEffect::Destructive
            | DynamicToolEffect::ComputerControl
            | DynamicToolEffect::SystemPermission => Effect::ExternalProcess,
        },
        name: tool.name.clone(),
    }
}

fn parameter_names(tool: &DynamicTool) -> Vec<String> {
    schema_value(tool)
        .pointer("/parameters/properties")
        .and_then(Value::as_object)
        .map(|properties| properties.keys().cloned().collect())
        .unwrap_or_default()
}

fn schema_description(tool: &DynamicTool) -> Result<String, DynamicDispatchError> {
    schema_value(tool)
        .get("description")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| DynamicDispatchError::InvalidSchema {
            name: tool.name.clone(),
            reason: "description is missing".to_owned(),
        })
}

fn schema_value(tool: &DynamicTool) -> Value {
    serde_json::to_value(&tool.schema).expect("IJsonValue is serializable")
}

fn validate_dynamic_arguments(
    tool: &DynamicTool,
    arguments: &IJsonValue,
) -> Result<(), DynamicDispatchError> {
    let schema = schema_value(tool);
    let parameters =
        schema
            .get("parameters")
            .ok_or_else(|| DynamicDispatchError::InvalidSchema {
                name: tool.name.clone(),
                reason: "parameters are missing".to_owned(),
            })?;
    let arguments = serde_json::to_value(arguments).map_err(DynamicDispatchError::Json)?;
    validate_schema_value(parameters, &arguments, "$").map_err(|reason| {
        DynamicDispatchError::InvalidArguments {
            name: tool.name.clone(),
            reason,
        }
    })
}

fn validate_schema_value(schema: &Value, value: &Value, path: &str) -> Result<(), String> {
    let object = schema
        .as_object()
        .ok_or_else(|| format!("{path}: schema node is not an object"))?;

    if let Some(constant) = object.get("const") {
        if value != constant {
            return Err(format!("{path}: value does not match const"));
        }
    }
    if let Some(values) = object.get("enum").and_then(Value::as_array) {
        if !values.contains(value) {
            return Err(format!("{path}: value is outside enum"));
        }
    }
    if let Some(branches) = object.get("allOf").and_then(Value::as_array) {
        for branch in branches {
            validate_schema_value(branch, value, path)?;
        }
    }
    if let Some(branches) = object.get("anyOf").and_then(Value::as_array) {
        if !branches
            .iter()
            .any(|branch| validate_schema_value(branch, value, path).is_ok())
        {
            return Err(format!("{path}: value matches no anyOf branch"));
        }
    }
    if let Some(branches) = object.get("oneOf").and_then(Value::as_array) {
        if branches
            .iter()
            .filter(|branch| validate_schema_value(branch, value, path).is_ok())
            .count()
            != 1
        {
            return Err(format!(
                "{path}: value does not match exactly one oneOf branch"
            ));
        }
    }

    if let Some(kind) = object.get("type").and_then(Value::as_str) {
        let matches = match kind {
            "object" => value.is_object(),
            "array" => value.is_array(),
            "string" => value.is_string(),
            "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
            "number" => value.is_number(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            other => return Err(format!("{path}: unsupported schema type {other}")),
        };
        if !matches {
            return Err(format!("{path}: expected {kind}"));
        }
    }

    if let Some(map) = value.as_object() {
        let properties = object
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        if let Some(required) = object.get("required").and_then(Value::as_array) {
            for name in required.iter().filter_map(Value::as_str) {
                if !map.contains_key(name) {
                    return Err(format!("{path}: missing required property {name}"));
                }
            }
        }
        if object.get("additionalProperties").and_then(Value::as_bool) == Some(false) {
            for name in map.keys() {
                if !properties.contains_key(name) {
                    return Err(format!("{path}: additional property {name}"));
                }
            }
        }
        for (name, property_schema) in properties {
            if let Some(property) = map.get(&name) {
                validate_schema_value(&property_schema, property, &format!("{path}.{name}"))?;
            }
        }
    }
    if let Some(items) = value.as_array() {
        if let Some(minimum) = object.get("minItems").and_then(Value::as_u64) {
            if items.len() < minimum as usize {
                return Err(format!("{path}: fewer than minItems"));
            }
        }
        if let Some(maximum) = object.get("maxItems").and_then(Value::as_u64) {
            if items.len() > maximum as usize {
                return Err(format!("{path}: more than maxItems"));
            }
        }
        if let Some(item_schema) = object.get("items") {
            for (index, item) in items.iter().enumerate() {
                validate_schema_value(item_schema, item, &format!("{path}[{index}]"))?;
            }
        }
    }
    if let Some(text) = value.as_str() {
        if let Some(minimum) = object.get("minLength").and_then(Value::as_u64) {
            if text.chars().count() < minimum as usize {
                return Err(format!("{path}: shorter than minLength"));
            }
        }
        if let Some(maximum) = object.get("maxLength").and_then(Value::as_u64) {
            if text.chars().count() > maximum as usize {
                return Err(format!("{path}: longer than maxLength"));
            }
        }
    }
    if let Some(number) = value.as_f64() {
        if let Some(minimum) = object.get("minimum").and_then(Value::as_f64) {
            if number < minimum {
                return Err(format!("{path}: below minimum"));
            }
        }
        if let Some(maximum) = object.get("maximum").and_then(Value::as_f64) {
            if number > maximum {
                return Err(format!("{path}: above maximum"));
            }
        }
    }
    Ok(())
}

#[must_use]
pub const fn dynamic_side_effectful(effect: DynamicToolEffect) -> bool {
    !matches!(effect, DynamicToolEffect::ReadOnly)
}

#[must_use]
pub const fn dynamic_approval_class(effect: DynamicToolEffect) -> ApprovalClass {
    match effect {
        DynamicToolEffect::ReadOnly => ApprovalClass::ReadOnly,
        DynamicToolEffect::WorkspaceWrite => ApprovalClass::Edit,
        DynamicToolEffect::ExternalProcess | DynamicToolEffect::NetworkRead => {
            ApprovalClass::Execute
        }
        DynamicToolEffect::Destructive
        | DynamicToolEffect::ComputerControl
        | DynamicToolEffect::SystemPermission => ApprovalClass::Destructive,
    }
}

const fn dynamic_approval_scope(effect: DynamicToolEffect) -> &'static str {
    match dynamic_approval_class(effect) {
        ApprovalClass::ReadOnly => "read_only",
        ApprovalClass::Edit => "edit",
        ApprovalClass::Execute => "execute",
        ApprovalClass::Destructive => "destructive",
        ApprovalClass::WorkflowHold => "workflow",
    }
}

#[derive(Debug, Error)]
pub enum DynamicDispatchError {
    #[error("unknown dynamic tool: {0}")]
    UnknownTool(String),
    #[error("dynamic tool dependency is unavailable: {0}")]
    UnavailableTool(String),
    #[error("invalid dynamic schema for {name}: {reason}")]
    InvalidSchema { name: String, reason: String },
    #[error("invalid dynamic arguments for {name}: {reason}")]
    InvalidArguments { name: String, reason: String },
    #[error("dynamic backend assembly failed: {0}")]
    Assembly(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Pipeline(#[from] ToolPipelineError),
}

#[cfg(test)]
mod mcp_schema_defaults_tests {
    use super::*;
    use profile::{DynamicToolSource, DynamicToolSourceKind};
    use serde_json::json;

    #[test]
    fn remote_open_and_closed_objects_keep_their_argument_semantics() {
        let ijson = |value: Value| IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap();
        for closed in [false, true] {
            let mut parameters = json!({"$schema":"https://json-schema.org/draft/2020-12/schema",
                "type":"object","properties":{"query":{"type":"string"}},"required":["query"]});
            if closed {
                parameters["additionalProperties"] = json!(false);
            }
            let tool = DynamicTool::declared("mcp__remote__search", DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp, id:"remote".into(),
            }, DynamicToolEffect::ReadOnly,
                false, vec![], ijson(json!({"name":"mcp__remote__search","description":"Search","parameters":parameters}))).unwrap();
            assert!(validate_dynamic_arguments(&tool, &ijson(json!({"query":"docs"}))).is_ok());
            assert!(validate_dynamic_arguments(&tool, &ijson(json!({}))).is_err());
            assert!(validate_dynamic_arguments(&tool, &ijson(json!({"query":42}))).is_err());
            assert_eq!(
                validate_dynamic_arguments(&tool, &ijson(json!({"query":"docs","extra":true})))
                    .is_ok(),
                !closed
            );
        }
        let tool = DynamicTool::declared("mcp__remote__guide", DynamicToolSource {
            kind: DynamicToolSourceKind::Mcp, id:"remote".into(),
        }, DynamicToolEffect::ReadOnly, false, vec![],
            ijson(json!({"name":"mcp__remote__guide","description":"Guide","parameters":{"type":"object","properties":{}}}))).unwrap();
        assert!(validate_dynamic_arguments(&tool, &ijson(json!({}))).is_ok());
        assert!(validate_dynamic_arguments(&tool, &ijson(json!({"extra":"retained"}))).is_ok());
    }
}
