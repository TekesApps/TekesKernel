//! The compaction summary request (event §compact `summary_request`): one
//! non-streaming compactor-role request whose only tool is `summary_artifact`,
//! and the artifact read back from its terminal.
use schema::IJsonValue;
use serde_json::{Value, json};

use crate::{
    ContentBlock, DialectId, PrepareInput, PreparedRequest, ProviderTerminal,
    ResolvedDialectProfile, ToolChoice, prepare_with_tool_choice,
};

/// `tool_catalog` is the projected `summary_artifact` schema (the compactor
/// role's catalog); `rendered_history` is the frozen bundle rendering.
pub fn prepare_summary_request(
    endpoint: &str,
    resolved: &ResolvedDialectProfile,
    system: &str,
    rendered_history: &str,
    tool_catalog: IJsonValue,
    attempt_id: String,
) -> Result<PreparedRequest, String> {
    let canonical = |value: &Value, what: &str| -> Result<IJsonValue, String> {
        IJsonValue::parse(
            &serde_json_canonicalizer::to_vec(value).map_err(|error| format!("{what}: {error}"))?,
        )
        .map_err(|error| format!("{what}: {error}"))
    };
    let epoch_profile = crate::epoch_profile(resolved, system, None)
        .map_err(|error| format!("summary request profile: {error}"))?;
    let rendered = canonical(
        &json!({"content": [{"text": rendered_history, "type": "text"}], "role": "user"}),
        "summary request input",
    )?;
    // DeepSeek rejects forced tool choice with thinking enabled. Preserve the
    // model's reasoning configuration and admit the returned artifact normally.
    // Interactions has no explicit tool-choice wire contract; its single-tool
    // catalog and the compactor instruction carry the request instead.
    let choice = match resolved.dialect {
        DialectId::DeepseekResponsesV1 => Some(ToolChoice::Auto),
        DialectId::GoogleInteractionsV1 => None,
        _ => Some(ToolChoice::Required),
    };
    prepare_with_tool_choice(
        &PrepareInput {
            attempt_id,
            target: resolved.target.clone(),
            endpoint: endpoint.to_owned(),
            epoch_profile,
            continuation_id: None,
            rendered_items: vec![rendered],
            tool_catalog,
            stream: false,
        },
        choice,
    )
    .map_err(|error| format!("prepare: {error}"))
}

/// The artifact arguments of one compactor completion (the caller admits them
/// against its frozen bundle), with the request's usage when reported; `Err`
/// is the rejection reason when the completion carries no artifact.
pub fn summary_completion_artifact(
    completion: Result<crate::ProviderCompletion, crate::HttpRuntimeError>,
) -> (Result<Value, String>, Option<Value>) {
    match completion {
        Ok(crate::ProviderCompletion::Terminal(terminal)) => {
            let usage = terminal
                .usage
                .as_ref()
                .and_then(|usage| serde_json::to_value(usage).ok());
            let artifact = summary_artifact_arguments(&terminal).ok_or_else(|| {
                format!(
                    "the compactor terminal carries no summary_artifact ({:?})",
                    terminal.finish_reason
                )
            });
            (artifact, usage)
        }
        Ok(crate::ProviderCompletion::Failure(failure)) => {
            (Err(format!("provider failure: {failure:?}")), None)
        }
        Err(error) => (Err(format!("transport: {error}")), None),
    }
}

/// The `summary_artifact` arguments of a completed compactor terminal: the
/// tool call when the model made one, else a JSON object of the same shape in
/// its text (some routes answer in text despite a required tool). `None`
/// when the terminal carries no artifact.
pub fn summary_artifact_arguments(terminal: &ProviderTerminal) -> Option<Value> {
    if let Some(call) = terminal
        .tool_calls
        .iter()
        .find(|call| call.name == "summary_artifact")
    {
        return Some(call.arguments.clone());
    }
    let text = terminal
        .content
        .iter()
        .filter_map(|block| match block {
            ContentBlock::Text(text) => Some(text.as_str()),
            ContentBlock::Reasoning(_) => None,
        })
        .collect::<Vec<_>>()
        .join("");
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    if end < start {
        return None;
    }
    serde_json::from_str::<Value>(&text[start..=end])
        .ok()
        .filter(|value| value.get("continuation").is_some() && value.get("evidence_refs").is_some())
}
