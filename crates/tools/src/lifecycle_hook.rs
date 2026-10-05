//! Host lifecycle extension protocol, independent of model tool calls.
use crate::hook::{HookError, decode_one_line, run_hook_process};
use crate::{HookBinding, HookFailureMode, HookPhase, encode_hook_line};
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LifecycleEvent {
    #[serde(rename = "turn.before")]
    TurnBefore,
    #[serde(rename = "context.prepare")]
    ContextPrepare,
    #[serde(rename = "tool.completed")]
    ToolCompleted,
    #[serde(rename = "context.before_compact")]
    ContextBeforeCompact,
    #[serde(rename = "turn.settled")]
    TurnSettled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleHookBinding {
    pub format: u64,
    pub id: String,
    pub event: LifecycleEvent,
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    pub argv: Vec<String>,
    pub timeout_ms: u64,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

fn enabled_by_default() -> bool {
    true
}

impl LifecycleHookBinding {
    fn process(&self) -> HookBinding {
        HookBinding {
            format: 1,
            id: self.id.clone(),
            argv: self.argv.clone(),
            phase: HookPhase::Pre,
            failure: HookFailureMode::Open,
            timeout_ms: self.timeout_ms,
            stdout_bytes: self.stdout_bytes,
            stderr_bytes: self.stderr_bytes,
            env: self.env.clone(),
        }
    }
}

/// Formats are disjoint; existing format-1 tool hooks retain their exact schema.
pub fn decode_lifecycle_hook_binding(
    bytes: &[u8],
    id: &str,
) -> Result<LifecycleHookBinding, HookError> {
    let binding: LifecycleHookBinding = decode_one_line(bytes)?;
    if binding.format != 2 || binding.id != id {
        return Err(HookError::Invalid(
            "lifecycle binding requires format 2 and its logical path id",
        ));
    }
    binding.process().validate()?;
    if binding.timeout_ms > 5_000 {
        return Err(HookError::Invalid("lifecycle hook timeout exceeds 5000ms"));
    }
    Ok(binding)
}

pub fn validate_instruction_hook(bytes: &[u8], id: &str) -> Result<(), HookError> {
    let value: serde_json::Value = decode_one_line(bytes)?;
    match value.get("format").and_then(serde_json::Value::as_u64) {
        Some(1) => crate::decode_hook_binding(bytes, id).map(|_| ()),
        Some(2) => decode_lifecycle_hook_binding(bytes, id).map(|_| ()),
        _ => Err(HookError::Invalid("unsupported hook binding format")),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleHookRequest {
    pub format: u64,
    pub hook_id: String,
    pub event_id: String,
    pub event: LifecycleEvent,
    pub workspace_id: String,
    pub thread_id: String,
    pub turn_id: u64,
    pub data: IJsonValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleHookResponse {
    pub format: u64,
    pub hook_id: String,
    pub event_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<String>,
}

pub fn run_lifecycle_hook(
    binding: &LifecycleHookBinding,
    request: &LifecycleHookRequest,
) -> Result<LifecycleHookResponse, HookError> {
    run_lifecycle_hook_with_cancel(binding, request, &|| false)
}

pub fn run_lifecycle_hook_with_cancel(
    binding: &LifecycleHookBinding,
    request: &LifecycleHookRequest,
    cancelled: &dyn Fn() -> bool,
) -> Result<LifecycleHookResponse, HookError> {
    if binding.format != 2
        || !binding.enabled
        || binding.timeout_ms > 5_000
        || request.format != 2
        || request.event_id.is_empty()
        || request.workspace_id.is_empty()
        || request.thread_id.is_empty()
        || request.hook_id != binding.id
        || request.event != binding.event
    {
        return Err(HookError::Correlation);
    }
    let bytes = run_hook_process(&binding.process(), encode_hook_line(request)?, cancelled)?;
    let response: LifecycleHookResponse = decode_one_line(&bytes)?;
    if response.format != 2
        || response.hook_id != request.hook_id
        || response.event_id != request.event_id
    {
        return Err(HookError::Correlation);
    }
    if request.event != LifecycleEvent::ContextPrepare && !response.context.is_empty() {
        return Err(HookError::Invalid(
            "only context.prepare may return context",
        ));
    }
    if response.context.len() > 32
        || response.context.iter().map(String::len).sum::<usize>() > 32_768
    {
        return Err(HookError::OutputLimit);
    }
    Ok(response)
}
