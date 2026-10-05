use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::SseDecoder;
use crate::{AdapterId, DialectId};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ContentBlock {
    Text(String),
    Reasoning(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub call_id: String,
    pub name: String,
    pub arguments: Value,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: Option<String>,
    pub output_tokens: Option<String>,
    pub cache_read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_miss: Option<String>,
    /// Tokens written to the provider cache this request (Anthropic
    /// `cache_creation_input_tokens`), billed above the plain input rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_write: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Completed,
    ToolCalls,
    Length,
    ContextOverflow,
    ContentFilter,
    ProviderError,
    /// The provider paused a long-running turn (`pause_turn`); the response
    /// is complete as a message but the turn continues by resending it.
    Paused,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderTerminal {
    pub content: Vec<ContentBlock>,
    pub tool_calls: Vec<ToolCall>,
    pub sealed_fragments: Value,
    pub response_identity: Option<String>,
    pub usage: Option<Usage>,
    pub finish_reason: FinishReason,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_error: Option<Value>,
    /// HTTP transport evidence; absent for protocol-only normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    /// Provider detail behind a refusal or content filter (Anthropic
    /// `stop_details`: `{type, category, explanation}`), verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_details: Option<Value>,
    /// Provider-reported input changes; diagnostic only, never replay content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_transformations: Option<Value>,
    pub raw_response: Vec<u8>,
}

impl ProviderTerminal {
    /// A response terminal is not necessarily a turn terminal. Preserve explicit
    /// Responses phases and legacy finality; phase-less protocols use their
    /// native end reason only when an actual assistant answer was produced.
    #[must_use]
    pub fn is_final_answer(&self) -> bool {
        if self
            .http_status
            .is_some_and(|status| !(200..300).contains(&status))
            || self.finish_reason != FinishReason::Completed
            || !self.tool_calls.is_empty()
            || !self
                .content
                .iter()
                .any(|block| matches!(block, ContentBlock::Text(text) if !text.trim().is_empty()))
        {
            return false;
        }
        if let Some(explicit) = self
            .sealed_fragments
            .get("isFinalAnswer")
            .and_then(Value::as_bool)
        {
            return explicit;
        }
        if let Some(items) = self.sealed_fragments.as_array() {
            let messages = items
                .iter()
                .filter(|item| item.get("type").and_then(Value::as_str) == Some("message"))
                .collect::<Vec<_>>();
            if messages.iter().any(|item| item.get("phase").is_some()) {
                return messages.iter().any(|item| {
                    item.get("phase").and_then(Value::as_str) == Some("final_answer")
                        && item.get("status").and_then(Value::as_str) == Some("completed")
                        && item
                            .get("content")
                            .and_then(Value::as_array)
                            .is_some_and(|parts| {
                                parts.iter().any(|part| {
                                    part.get("text")
                                        .and_then(Value::as_str)
                                        .is_some_and(|text| !text.trim().is_empty())
                                })
                            })
                });
            }
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProviderFailure {
    AuthFailure {
        status: u16,
        detail: Option<String>,
    },
    RateLimited {
        status: u16,
        detail: Option<String>,
        retry_after_seconds: Option<u64>,
    },
    Transport {
        status: Option<u16>,
        detail: String,
        /// Native output items the stream had already completed when the
        /// transport failed, in output order and in the adapter's sealed
        /// fragment shape (a Responses `output` array, Anthropic content
        /// blocks). A retry that must replay an eagerly dispatched call
        /// carries these verbatim in front of the call's result, so a
        /// provider that binds a call to its preceding reasoning item
        /// (DeepSeek thinking mode) accepts the replay. Absent when the
        /// failure precedes the body or nothing had completed.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        partial_fragments: Option<Value>,
    },
    Malformed {
        detail: String,
    },
    Cancelled,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProviderCompletion {
    Terminal(ProviderTerminal),
    Failure(ProviderFailure),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProviderFrame {
    TextDelta(String),
    ReasoningDelta(String),
    ToolDelta {
        call_id: String,
        name: Option<String>,
        delta: String,
    },
    /// The call's identity and arguments are complete and validated. The worker
    /// may dispatch it eagerly on this frame; the response terminal must still
    /// name the call unchanged, or the whole response fails.
    ToolCallReady(ToolCall),
    Status(String),
    UsageDelta(Usage),
}

pub fn normalize_response(adapter: AdapterId, bytes: &[u8]) -> Result<ProviderTerminal, String> {
    let value = parse_ijson(bytes)?;
    if value.get("error").is_some_and(|error| !error.is_null()) {
        return Ok(ProviderTerminal {
            content: Vec::new(),
            tool_calls: Vec::new(),
            sealed_fragments: Value::Array(Vec::new()),
            response_identity: None,
            usage: usage_from_any(value.get("usage")),
            finish_reason: if is_context_overflow(&value) {
                FinishReason::ContextOverflow
            } else {
                FinishReason::ProviderError
            },
            incomplete_reason: None,
            provider_error: value.get("error").cloned(),
            http_status: None,
            stop_details: None,
            input_transformations: None,
            raw_response: bytes.to_vec(),
        });
    }
    if value.get("functionCalls").is_some() || value.get("isFinalAnswer").is_some() {
        return normalize_legacy_result(bytes, &value);
    }
    match adapter {
        AdapterId::Responses => normalize_responses(bytes, &value),
        AdapterId::GoogleInteractions => normalize_interactions(bytes, &value),
        AdapterId::Anthropic => normalize_anthropic(bytes, &value),
        AdapterId::ChatCompletion => normalize_chat(bytes, &value),
        AdapterId::GoogleGeneration => normalize_google(bytes, &value),
    }
}

pub fn normalize_dialect_response(
    dialect: DialectId,
    bytes: &[u8],
) -> Result<ProviderTerminal, String> {
    let mut terminal = if matches!(
        dialect,
        DialectId::DeepseekResponsesV1 | DialectId::DeepseekChatV1
    ) {
        let mut value = parse_ijson(bytes)?;
        // Request-level errors are not Responses/Chat success envelopes and do
        // not carry response status, output, or choices. Preserve their cause.
        if value.get("error").is_some_and(|error| !error.is_null()) {
            return normalize_response(dialect.family(), bytes);
        }
        if value
            .get("output")
            .and_then(Value::as_array)
            .is_some_and(|items| items.iter().any(|item| item["type"] == "tool_search_call"))
        {
            return Err("native client tool search requires OpenAI Responses".into());
        }
        repair_response_argument_strings(dialect, &mut value)?;
        if dialect == DialectId::DeepseekChatV1 {
            normalize_chat(bytes, &value)?
        } else {
            validate_responses_output_container(&value)?;
            let mut terminal = normalize_responses(bytes, &value)?;
            terminal.finish_reason = match value.get("status").and_then(Value::as_str) {
                Some("completed") => terminal.finish_reason,
                Some("incomplete") => {
                    let reason = value
                        .get("incomplete_details")
                        .and_then(|details| details.get("reason"))
                        .and_then(Value::as_str)
                        .ok_or("DeepSeek Responses incomplete terminal lacks reason")?;
                    terminal.incomplete_reason = Some(reason.to_owned());
                    match reason {
                        "max_output_tokens" => FinishReason::Length,
                        "content_filter" => FinishReason::ContentFilter,
                        other => {
                            return Err(format!(
                                "unknown DeepSeek Responses incomplete reason {other}"
                            ));
                        }
                    }
                }
                Some("failed") => {
                    let error = value
                        .get("error")
                        .filter(|error| error.is_object())
                        .cloned()
                        .ok_or("DeepSeek Responses failed terminal lacks structured error")?;
                    terminal.provider_error = Some(error);
                    FinishReason::ProviderError
                }
                Some(other) => return Err(format!("unknown DeepSeek Responses status {other}")),
                None => return Err("DeepSeek Responses terminal lacks status".to_owned()),
            };
            terminal
        }
    } else {
        if matches!(
            dialect,
            DialectId::AnthropicMessagesV1 | DialectId::DeepseekAnthropicV1
        ) {
            let value = parse_ijson(bytes)?;
            validate_exact_anthropic_payload(&value)?;
        }
        normalize_response(dialect.family(), bytes)?
    };
    repair_terminal_tool_arguments(dialect, &mut terminal)?;
    Ok(terminal)
}

fn validate_responses_output_container(value: &Value) -> Result<(), String> {
    if value.get("output").is_some_and(|output| !output.is_array()) {
        return Err("Responses output must be an array".to_owned());
    }
    Ok(())
}

fn validate_exact_anthropic_payload(value: &Value) -> Result<(), String> {
    let content = value
        .get("content")
        .and_then(Value::as_array)
        .ok_or("exact Anthropic response lacks content")?;
    for part in content {
        if !part.is_object() || part.get("type").and_then(Value::as_str).is_none() {
            return Err("Anthropic content block lacks type".to_owned());
        }
    }
    Ok(())
}

fn repair_response_argument_strings(dialect: DialectId, value: &mut Value) -> Result<(), String> {
    let pointers = if dialect == DialectId::DeepseekResponsesV1 {
        value
            .get_mut("output")
            .and_then(Value::as_array_mut)
            .into_iter()
            .flatten()
            .filter_map(|item| item.get_mut("arguments"))
            .collect::<Vec<_>>()
    } else {
        value
            .get_mut("choices")
            .and_then(Value::as_array_mut)
            .into_iter()
            .flatten()
            .filter_map(|choice| choice.pointer_mut("/message/tool_calls"))
            .filter_map(Value::as_array_mut)
            .flatten()
            .filter_map(|call| call.pointer_mut("/function/arguments"))
            .collect::<Vec<_>>()
    };
    for argument in pointers {
        let raw = argument
            .as_str()
            .ok_or_else(|| "DeepSeek tool arguments must be a string".to_owned())?;
        // Unrepairable text stays as emitted; resolution turns it into the
        // invalid-arguments sentinel rather than failing the whole response.
        let Ok(repaired) = repair_tool_arguments(dialect, raw.as_bytes()) else {
            continue;
        };
        *argument = Value::String(
            serde_json_canonicalizer::to_string(&repaired).map_err(|error| error.to_string())?,
        );
    }
    Ok(())
}

pub fn repair_tool_arguments(dialect: DialectId, bytes: &[u8]) -> Result<Value, String> {
    if bytes.len() > 65_536 {
        return Err("provider tool arguments exceed the 65536-byte repair bound".to_owned());
    }
    if !matches!(
        dialect,
        DialectId::DeepseekResponsesV1 | DialectId::DeepseekChatV1
    ) {
        return parse_foreign_value(bytes, "tool arguments");
    }
    let mut repaired = String::from_utf8(bytes.to_vec())
        .map_err(|_| "provider tool arguments are not UTF-8".to_owned())?;
    if parse_foreign_value(repaired.as_bytes(), "tool arguments").is_err() {
        repaired = repair_bare_object_values(&repaired);
    }
    let value = parse_foreign_value(repaired.as_bytes(), "tool arguments")?;
    if !value.is_object() {
        return Err("provider tool arguments must be an object".to_owned());
    }
    validate_foreign_value(&value, "repaired tool arguments")?;
    Ok(value)
}

fn repair_bare_object_values(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut index = 0_usize;
    let mut output = String::with_capacity(input.len() + 8);

    fn skip_whitespace(chars: &[char], index: &mut usize, output: &mut String) {
        while chars.get(*index).is_some_and(|value| value.is_whitespace()) {
            output.push(chars[*index]);
            *index += 1;
        }
    }

    fn copy_quoted(chars: &[char], index: &mut usize, output: &mut String) -> bool {
        if chars.get(*index) != Some(&'"') {
            return false;
        }
        output.push('"');
        *index += 1;
        while let Some(character) = chars.get(*index).copied() {
            output.push(character);
            *index += 1;
            if character == '\\' {
                let Some(escaped) = chars.get(*index).copied() else {
                    return false;
                };
                output.push(escaped);
                *index += 1;
            } else if character == '"' {
                return true;
            }
        }
        false
    }

    fn starts_next_field(chars: &[char], mut index: usize) -> bool {
        while chars.get(index).is_some_and(|value| value.is_whitespace()) {
            index += 1;
        }
        if chars.get(index) != Some(&'"') {
            return false;
        }
        index += 1;
        while let Some(character) = chars.get(index).copied() {
            if character == '\\' {
                index += 2;
                continue;
            }
            if character == '"' {
                index += 1;
                break;
            }
            index += 1;
        }
        while chars.get(index).is_some_and(|value| value.is_whitespace()) {
            index += 1;
        }
        chars.get(index) == Some(&':')
    }

    fn only_whitespace_after(chars: &[char], index: usize) -> bool {
        chars[index.saturating_add(1)..]
            .iter()
            .all(|value| value.is_whitespace())
    }

    fn scan_balanced(chars: &[char], index: &mut usize) -> Option<String> {
        let start = *index;
        if !matches!(chars.get(*index), Some('{' | '[')) {
            return None;
        }
        let mut depth = 0_usize;
        while let Some(character) = chars.get(*index).copied() {
            if character == '"' {
                *index += 1;
                while let Some(quoted) = chars.get(*index).copied() {
                    if quoted == '\\' {
                        *index += 2;
                        continue;
                    }
                    *index += 1;
                    if quoted == '"' {
                        break;
                    }
                }
                continue;
            }
            if matches!(character, '{' | '[') {
                depth += 1;
            } else if matches!(character, '}' | ']') {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    *index += 1;
                    return Some(chars[start..*index].iter().collect());
                }
            }
            *index += 1;
        }
        None
    }

    fn consume_value(chars: &[char], index: &mut usize, output: &mut String) -> bool {
        match chars.get(*index) {
            Some('"') => return copy_quoted(chars, index, output),
            Some('{' | '[') => {
                let Some(fragment) = scan_balanced(chars, index) else {
                    return false;
                };
                if serde_json::from_str::<Value>(&fragment).is_ok() {
                    output.push_str(&fragment);
                    return true;
                }
                let inner = fragment[1..fragment.len() - 1].trim();
                if fragment.starts_with('[')
                    && !inner.is_empty()
                    && !inner.contains(['"', '[', '{'])
                {
                    output.push('[');
                    output.push_str(&serde_json::to_string(inner).expect("string JSON"));
                    output.push(']');
                    return true;
                }
                output.push_str(&fragment);
                return true;
            }
            Some(_) => {}
            None => return false,
        }

        let start = *index;
        let mut end = None;
        while let Some(character) = chars.get(*index).copied() {
            if character == ',' && starts_next_field(chars, *index + 1) {
                end = Some(*index);
                break;
            }
            if character == '}' && only_whitespace_after(chars, *index) {
                end = Some(*index);
                break;
            }
            *index += 1;
        }
        let Some(end) = end else {
            return false;
        };
        let bare = chars[start..end]
            .iter()
            .collect::<String>()
            .trim()
            .to_owned();
        if bare.is_empty() {
            return false;
        }
        if matches!(bare.as_str(), "true" | "false" | "null") || bare.parse::<f64>().is_ok() {
            output.push_str(&bare);
        } else {
            output.push_str(&serde_json::to_string(&bare).expect("string JSON"));
        }
        true
    }

    skip_whitespace(&chars, &mut index, &mut output);
    if chars.get(index) != Some(&'{') {
        return input.to_owned();
    }
    output.push('{');
    index += 1;
    skip_whitespace(&chars, &mut index, &mut output);
    if chars.get(index) == Some(&'}') {
        output.extend(chars[index..].iter());
        return output;
    }
    loop {
        skip_whitespace(&chars, &mut index, &mut output);
        if !copy_quoted(&chars, &mut index, &mut output) {
            return input.to_owned();
        }
        skip_whitespace(&chars, &mut index, &mut output);
        if chars.get(index) != Some(&':') {
            return input.to_owned();
        }
        output.push(':');
        index += 1;
        skip_whitespace(&chars, &mut index, &mut output);
        if !consume_value(&chars, &mut index, &mut output) {
            return input.to_owned();
        }
        skip_whitespace(&chars, &mut index, &mut output);
        match chars.get(index) {
            Some(',') => {
                output.push(',');
                index += 1;
            }
            Some('}') => {
                output.extend(chars[index..].iter());
                return output;
            }
            _ => return input.to_owned(),
        }
    }
}

fn repair_terminal_tool_arguments(
    dialect: DialectId,
    terminal: &mut ProviderTerminal,
) -> Result<(), String> {
    if !matches!(
        dialect,
        DialectId::DeepseekResponsesV1 | DialectId::DeepseekChatV1
    ) {
        return Ok(());
    }
    for call in &mut terminal.tool_calls {
        if call.arguments.get(INVALID_ARGUMENTS_KEY).is_some() {
            continue;
        }
        let bytes =
            serde_json_canonicalizer::to_vec(&call.arguments).map_err(|error| error.to_string())?;
        call.arguments = repair_tool_arguments(dialect, &bytes)?;
    }
    Ok(())
}

fn is_context_overflow(value: &Value) -> bool {
    let error = value.get("error").unwrap_or(value);
    ["code", "type", "status"]
        .into_iter()
        .filter_map(|field| error.get(field).and_then(Value::as_str))
        .any(|value| {
            matches!(
                value,
                "context_length_exceeded"
                    | "context_window_exceeded"
                    | "max_context_length_exceeded"
                    | "CONTEXT_LENGTH_EXCEEDED"
            )
        })
}

pub fn normalize_sse_stream(
    adapter: AdapterId,
    bytes: &[u8],
) -> Result<(Vec<ProviderFrame>, ProviderTerminal), String> {
    let mut decoder = ProviderStreamDecoder::new(adapter);
    let mut frames = decoder.push(bytes)?;
    let (tail, terminal) = decoder.finish()?;
    frames.extend(tail);
    Ok((frames, terminal))
}

pub fn normalize_dialect_sse_stream(
    dialect: DialectId,
    bytes: &[u8],
) -> Result<(Vec<ProviderFrame>, ProviderTerminal), String> {
    let mut decoder = ProviderStreamDecoder::for_dialect(dialect);
    let mut frames = decoder.push(bytes)?;
    let (tail, terminal) = decoder.finish()?;
    frames.extend(tail);
    Ok((frames, terminal))
}

pub struct ProviderStreamDecoder {
    adapter: AdapterId,
    dialect: Option<DialectId>,
    sse: SseDecoder,
    accumulator: StreamAccumulator,
    raw: Vec<u8>,
    completed_events: u64,
}

impl ProviderStreamDecoder {
    #[must_use]
    pub fn new(adapter: AdapterId) -> Self {
        Self {
            adapter,
            dialect: None,
            sse: SseDecoder::new(),
            accumulator: StreamAccumulator::default(),
            raw: Vec::new(),
            completed_events: 0,
        }
    }

    #[must_use]
    pub fn for_dialect(dialect: DialectId) -> Self {
        Self {
            adapter: dialect.family(),
            dialect: Some(dialect),
            sse: SseDecoder::new(),
            accumulator: StreamAccumulator::default(),
            raw: Vec::new(),
            completed_events: 0,
        }
    }

    #[must_use]
    pub const fn completed_events(&self) -> u64 {
        self.completed_events
    }

    /// The native output items completed so far (see
    /// [`ProviderFailure::Transport::partial_fragments`]); `None` until one
    /// item is complete. An item is complete once the stream closed it
    /// (`response.output_item.done`, `content_block_stop`) or, for a tool
    /// call, once its arguments are complete: the same fact that made the
    /// call eligible for eager dispatch.
    #[must_use]
    pub fn completed_fragments(&self) -> Option<Value> {
        self.accumulator
            .completed_fragments(self.adapter, self.dialect)
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<ProviderFrame>, String> {
        self.raw.extend_from_slice(bytes);
        let events = self.sse.push(bytes).map_err(|error| error.to_string())?;
        self.consume(events)?;
        Ok(std::mem::take(&mut self.accumulator.frames))
    }

    pub fn finish(mut self) -> Result<(Vec<ProviderFrame>, ProviderTerminal), String> {
        let events = std::mem::take(&mut self.sse)
            .finish_events()
            .map_err(|error| error.to_string())?;
        self.consume(events)?;
        let frames = std::mem::take(&mut self.accumulator.frames);
        let (_, terminal) = self
            .accumulator
            .finish(self.adapter, self.dialect, &self.raw)?;
        Ok((frames, terminal))
    }

    fn consume(&mut self, events: Vec<crate::SseEvent>) -> Result<(), String> {
        for event in events {
            self.completed_events = self.completed_events.saturating_add(1);
            if event.data == "[DONE]" {
                self.accumulator.done = true;
                continue;
            }
            let value = parse_ijson(event.data.as_bytes())?;
            self.accumulator
                .consume(self.adapter, self.dialect, &value)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct StreamAccumulator {
    frames: Vec<ProviderFrame>,
    text: String,
    reasoning: String,
    calls: std::collections::BTreeMap<usize, PartialCall>,
    response_identity: Option<String>,
    usage: Option<Usage>,
    finish_reason: Option<FinishReason>,
    incomplete_reason: Option<String>,
    provider_error: Option<Value>,
    terminal_fragments: Option<Value>,
    stop_details: Option<Value>,
    input_transformations: Option<Value>,
    response_items: std::collections::BTreeMap<usize, Value>,
    anthropic_blocks: std::collections::BTreeMap<usize, Value>,
    interaction_steps: std::collections::BTreeMap<usize, Value>,
    chat_message: serde_json::Map<String, Value>,
    chat_tool_calls: std::collections::BTreeMap<usize, Value>,
    google_parts: Vec<Value>,
    /// Output indices whose native item the stream has closed.
    completed_items: std::collections::BTreeSet<usize>,
    done: bool,
}

#[derive(Default)]
struct PartialCall {
    id: String,
    name: String,
    arguments: String,
    value_arguments: Option<Value>,
    arguments_done: bool,
}

fn validate_completed_response_call(call: &PartialCall, item: &Value) -> Result<(), String> {
    if item["type"] != "function_call" || item["call_id"] != call.id || item["name"] != call.name {
        return Err("completed tool identity differs from response manifest".to_owned());
    }
    let arguments = required_string(item, "arguments")?;
    let same = match (
        parse_ijson(arguments.as_bytes()),
        parse_ijson(call.arguments.as_bytes()),
    ) {
        (Ok(manifest), Ok(streamed)) => manifest == streamed,
        // Unusable arguments carry no value to compare; the manifest must
        // repeat the streamed text exactly, and both resolve to one sentinel.
        (Err(_), Err(_)) => arguments == call.arguments,
        _ => false,
    };
    if !same {
        return Err("completed tool arguments differ from response manifest".to_owned());
    }
    Ok(())
}

/// Arguments the model emitted that are not usable JSON (duplicate members,
/// truncated text, a non-object) become this sentinel instead of ending the
/// response: the call keeps its identity, the worker refuses it with a durable
/// invalid-arguments `tool_result`, and the model may resend it. Deterministic
/// for one raw text, so readiness and the terminal manifest still agree.
pub const INVALID_ARGUMENTS_KEY: &str = "$tekes";

pub fn invalid_arguments_sentinel(raw: &str, error: &str) -> Value {
    let mut end = raw.len().min(2048);
    while !raw.is_char_boundary(end) {
        end -= 1;
    }
    serde_json::json!({INVALID_ARGUMENTS_KEY: {"invalid_arguments": {"error": error, "raw": &raw[..end]}}})
}

/// The message the worker returns to the model for a sentinel argument value;
/// `None` for ordinary arguments.
#[must_use]
pub fn invalid_arguments_detail(arguments: &Value) -> Option<String> {
    let invalid = arguments
        .get(INVALID_ARGUMENTS_KEY)?
        .get("invalid_arguments")?;
    Some(format!(
        "tool arguments were not valid JSON and the tool was not executed: {}. Resend the call with well-formed JSON arguments (no duplicate keys).",
        invalid
            .get("error")
            .and_then(Value::as_str)
            .unwrap_or("invalid JSON")
    ))
}

fn resolve_call_arguments_or_sentinel(call: &PartialCall, dialect: Option<DialectId>) -> Value {
    resolve_call_arguments(call, dialect)
        .unwrap_or_else(|error| invalid_arguments_sentinel(&call.arguments, &error))
}

/// The arguments a partial call resolves to, identically at readiness and at
/// the terminal: the announced call and the manifest call are one fact.
fn resolve_call_arguments(call: &PartialCall, dialect: Option<DialectId>) -> Result<Value, String> {
    let arguments = if let Some(value) = call.value_arguments.as_ref() {
        if call.arguments.is_empty() {
            value.clone()
        } else if let Some(dialect) = dialect {
            repair_tool_arguments(dialect, call.arguments.as_bytes())?
        } else {
            parse_ijson(call.arguments.as_bytes())?
        }
    } else if let Some(dialect) = dialect {
        repair_tool_arguments(dialect, call.arguments.as_bytes())?
    } else {
        parse_ijson(call.arguments.as_bytes())?
    };
    validate_foreign_value(&arguments, "tool arguments")?;
    Ok(arguments)
}

impl StreamAccumulator {
    /// Announces a call whose identity and arguments are complete on this
    /// stream, exactly once. Adapters without a per-call completion event
    /// (Chat Completions) announce at their finish signal; adapters that
    /// deliver whole calls (Google) announce as the call arrives.
    fn mark_call_ready(&mut self, index: usize, dialect: Option<DialectId>) -> Result<(), String> {
        let Some(call) = self.calls.get_mut(&index) else {
            return Ok(());
        };
        if call.arguments_done {
            return Ok(());
        }
        if call.id.is_empty() || call.name.is_empty() {
            return Err("completed tool arguments lack identity".to_owned());
        }
        let arguments = resolve_call_arguments_or_sentinel(call, dialect);
        call.arguments_done = true;
        let ready = ToolCall {
            call_id: call.id.clone(),
            name: call.name.clone(),
            arguments,
        };
        self.frames.push(ProviderFrame::ToolCallReady(ready));
        Ok(())
    }

    /// Completed native items in output order, or `None` when nothing has
    /// completed. Tool-call items carry the same resolved arguments the
    /// terminal would give them, so the replayed call matches the
    /// eagerly dispatched one. Only families whose sealed carrier is an
    /// ordered item array are supported; the rest report `None`.
    fn completed_fragments(&self, adapter: AdapterId, dialect: Option<DialectId>) -> Option<Value> {
        let items = match adapter {
            AdapterId::Responses => &self.response_items,
            AdapterId::Anthropic => &self.anthropic_blocks,
            AdapterId::GoogleInteractions
            | AdapterId::ChatCompletion
            | AdapterId::GoogleGeneration => {
                return None;
            }
        };
        let mut fragments = Vec::new();
        for (index, item) in items {
            let call = self.calls.get(index).filter(|call| call.arguments_done);
            if !self.completed_items.contains(index) && call.is_none() {
                continue;
            }
            let mut item = item.clone();
            if let Some(call) = call {
                let arguments = resolve_call_arguments_or_sentinel(call, dialect);
                let object = item.as_object_mut()?;
                if adapter == AdapterId::Responses {
                    object.insert(
                        "arguments".to_owned(),
                        Value::String(serde_json_canonicalizer::to_string(&arguments).ok()?),
                    );
                    object.insert("call_id".to_owned(), Value::String(call.id.clone()));
                    object.insert("name".to_owned(), Value::String(call.name.clone()));
                } else {
                    object.insert("input".to_owned(), arguments);
                }
            }
            fragments.push(item);
        }
        (!fragments.is_empty()).then_some(Value::Array(fragments))
    }

    fn consume(
        &mut self,
        adapter: AdapterId,
        dialect: Option<DialectId>,
        value: &Value,
    ) -> Result<(), String> {
        if self.done {
            return Err("provider data follows terminal".to_owned());
        }
        match adapter {
            AdapterId::Responses => self.responses(value, dialect),
            AdapterId::GoogleInteractions => self.interactions(value),
            AdapterId::Anthropic => self.anthropic(value, dialect),
            AdapterId::ChatCompletion => self.chat(value, dialect),
            AdapterId::GoogleGeneration => self.google(value),
        }?;
        // Report after this event's content: its watermark covers all content in
        // the same frame. Preserve partial reports instead of replaying old usage.
        let kind = value.get("type").and_then(Value::as_str);
        let raw_usage = match adapter {
            AdapterId::Responses
                if matches!(
                    kind,
                    Some(
                        "response.done"
                            | "response.completed"
                            | "response.incomplete"
                            | "response.failed"
                    )
                ) =>
            {
                value.get("response").and_then(|v| v.get("usage"))
            }
            AdapterId::GoogleInteractions
                if value
                    .get("event_type")
                    .or_else(|| value.get("type"))
                    .and_then(Value::as_str)
                    == Some("interaction.completed") =>
            {
                value.get("interaction").and_then(|v| v.get("usage"))
            }
            AdapterId::Anthropic if kind == Some("message_start") => {
                value.get("message").and_then(|v| v.get("usage"))
            }
            AdapterId::Anthropic if kind == Some("message_delta") => value.get("usage"),
            AdapterId::ChatCompletion => value.get("usage"),
            AdapterId::GoogleGeneration => value.get("usageMetadata"),
            _ => None,
        };
        if let Some(usage) = usage_from_any(raw_usage) {
            self.frames.push(ProviderFrame::UsageDelta(usage));
        }
        Ok(())
    }

    fn responses(&mut self, value: &Value, dialect: Option<DialectId>) -> Result<(), String> {
        match value.get("type").and_then(Value::as_str) {
            Some("response.output_text.delta") => {
                let delta = delta_string(value, "delta")?;
                append_response_part_text(&mut self.response_items, value, "content", &delta)?;
                self.push_text(delta);
            }
            Some("response.reasoning_summary_text.delta") => {
                let delta = delta_string(value, "delta")?;
                append_response_part_text(&mut self.response_items, value, "summary", &delta)?;
                self.push_reasoning(delta);
            }
            Some("response.reasoning_text.delta") => {
                let delta = delta_string(value, "delta")?;
                append_response_part_text(&mut self.response_items, value, "content", &delta)?;
                self.push_reasoning(delta);
            }
            Some("response.function_call_arguments.delta") => {
                let index = value
                    .get("output_index")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as usize;
                let call = self.calls.entry(index).or_default();
                if call.arguments_done {
                    return Err("tool arguments delta follows arguments.done".to_owned());
                }
                if let Some(id) = value.get("call_id").and_then(Value::as_str) {
                    if !call.id.is_empty() && call.id != id {
                        return Err("tool identity changed during arguments stream".to_owned());
                    }
                    call.id = id.to_owned();
                }
                if let Some(name) = value.get("name").and_then(Value::as_str) {
                    if !call.name.is_empty() && call.name != name {
                        return Err("tool name changed during arguments stream".to_owned());
                    }
                    call.name = name.to_owned();
                }
                let delta = delta_string(value, "delta")?;
                call.arguments.push_str(&delta);
                self.frames.push(ProviderFrame::ToolDelta {
                    call_id: call.id.clone(),
                    name: (!call.name.is_empty()).then(|| call.name.clone()),
                    delta,
                });
            }
            Some("response.output_item.added") => {
                let item = value.get("item").ok_or("output_item.added lacks item")?;
                if !item.is_object() || item.get("type").and_then(Value::as_str).is_none() {
                    return Err("output_item.added has an invalid item".to_owned());
                }
                let index = optional_usize(value, "output_index")?.unwrap_or(0);
                self.response_items.insert(index, item.clone());
                if item.get("type").and_then(Value::as_str) == Some("function_call") {
                    let call = self.calls.entry(index).or_default();
                    call.id = required_string(item, "call_id")?;
                    call.name = required_string(item, "name")?;
                } else if item["type"] == "tool_search_call" {
                    if dialect.is_some_and(|d| d != DialectId::OpenaiResponsesV1)
                        || item["execution"] != "client"
                    {
                        return Err("native client tool search requires OpenAI Responses".into());
                    }
                    let call = self.calls.entry(index).or_default();
                    let id = required_string(item, "call_id")?;
                    if id.is_empty() || (!call.id.is_empty() && call.id != id) {
                        return Err("native search call changed identity".into());
                    }
                    call.id = id;
                    call.name = "tool_search".into();
                }
            }
            Some("response.done")
            | Some("response.completed")
            | Some("response.incomplete")
            | Some("response.failed") => {
                if self.done {
                    return Err("duplicate provider terminal".to_owned());
                }
                let response = value
                    .get("response")
                    .ok_or("response terminal lacks response")?;
                if let Some(output) = response.get("output") {
                    if !output.is_array() {
                        return Err("Responses terminal output must be an array".to_owned());
                    }
                    for (index, call) in &self.calls {
                        if call.arguments_done {
                            let item = output
                                .as_array()
                                .expect("validated output")
                                .get(*index)
                                .ok_or("terminal output omitted a completed tool call")?;
                            validate_completed_response_call(call, item)?;
                        }
                    }
                    for (index, item) in output
                        .as_array()
                        .expect("validated output")
                        .iter()
                        .enumerate()
                    {
                        if item["type"] == "tool_search_call" {
                            self.record_native_search_call(index, item, dialect)?;
                        }
                    }
                    self.terminal_fragments = Some(output.clone());
                }
                self.response_identity = response
                    .get("id")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                self.usage = usage_from_any(response.get("usage"));
                let event = value
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if event != "response.done" {
                    let expected_status = match event {
                        "response.completed" => "completed",
                        "response.incomplete" => "incomplete",
                        "response.failed" => "failed",
                        _ => unreachable!("known Responses terminal event"),
                    };
                    if response.get("status").and_then(Value::as_str) != Some(expected_status) {
                        return Err(format!("Responses {event} terminal status mismatch"));
                    }
                }
                self.finish_reason = Some(match event {
                    "response.incomplete" => {
                        let reason = response
                            .get("incomplete_details")
                            .and_then(|details| details.get("reason"))
                            .and_then(Value::as_str)
                            .ok_or("DeepSeek Responses incomplete terminal lacks reason")?;
                        self.incomplete_reason = Some(reason.to_owned());
                        match reason {
                            "max_output_tokens" => FinishReason::Length,
                            "content_filter" => FinishReason::ContentFilter,
                            other => {
                                return Err(format!(
                                    "unknown DeepSeek Responses incomplete reason {other}"
                                ));
                            }
                        }
                    }
                    "response.failed" => {
                        self.provider_error = Some(
                            response
                                .get("error")
                                .filter(|error| error.is_object())
                                .cloned()
                                .ok_or(
                                    "DeepSeek Responses failed terminal lacks structured error",
                                )?,
                        );
                        FinishReason::ProviderError
                    }
                    _ => finish_from_reason(
                        response.get("status").and_then(Value::as_str),
                        !self.calls.is_empty(),
                    )?,
                });
                self.done = true;
            }
            Some("response.function_call_arguments.done") => {
                let index = value
                    .get("output_index")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as usize;
                let call = self.calls.entry(index).or_default();
                // Arguments-done may carry only the item index and arguments;
                // identity belongs to the earlier output_item.added frame.
                if let Some(name) = value.get("name").and_then(Value::as_str) {
                    if !call.name.is_empty() && call.name != name {
                        return Err("tool name changed during arguments stream".to_owned());
                    }
                    call.name = name.to_owned();
                }
                let arguments = required_string(value, "arguments")?;
                if !call.arguments.is_empty() && call.arguments != arguments {
                    return Err("arguments.done differs from streamed tool arguments".to_owned());
                }
                // Some Responses streams deliver arguments only at done. Publish
                // those bytes through the same live sink, without replaying deltas.
                if call.arguments.is_empty() && !arguments.is_empty() && !call.arguments_done {
                    self.frames.push(ProviderFrame::ToolDelta {
                        call_id: call.id.clone(),
                        name: (!call.name.is_empty()).then(|| call.name.clone()),
                        delta: arguments.clone(),
                    });
                }
                call.arguments = arguments;
                // Readiness and the terminal must materialize the same call.
                // In particular, DeepSeek's bounded repair cannot run only
                // at terminal time after eager dispatch rejected the raw text.
                self.mark_call_ready(index, dialect)?;
            }
            Some("response.created")
            | Some("response.in_progress")
            | Some("response.output_text.done") => {}
            Some("response.content_part.added") | Some("response.content_part.done") => {
                let part = value.get("part").ok_or("content_part event lacks part")?;
                if !part.is_object() || part.get("type").and_then(Value::as_str).is_none() {
                    return Err("content_part event has an invalid part".to_owned());
                }
                upsert_response_part(&mut self.response_items, value, part.clone())?;
            }
            Some("response.output_item.done") => {
                let item = value.get("item").ok_or("output_item.done lacks item")?;
                if !item.is_object() || item.get("type").and_then(Value::as_str).is_none() {
                    return Err("output_item.done has an invalid item".to_owned());
                }
                let index = optional_usize(value, "output_index")?.unwrap_or(0);
                if let Some(call) = self.calls.get(&index).filter(|call| call.arguments_done) {
                    validate_completed_response_call(call, item)?;
                }
                self.response_items.insert(index, item.clone());
                self.completed_items.insert(index);
                if item["type"] == "tool_search_call" {
                    self.record_native_search_call(index, item, dialect)?;
                }
            }
            Some("response.reasoning_summary_text.done" | "response.reasoning_text.done") => {}
            // Response event vocabularies are open-ended. Unknown, well-formed
            // events may describe provider-managed work or future metadata;
            // the terminal response remains the native replay authority.
            Some(_) => {}
            None => return Err("provider event lacks type".to_owned()),
        }
        Ok(())
    }

    fn interactions(&mut self, value: &Value) -> Result<(), String> {
        match value
            .get("event_type")
            .or_else(|| value.get("type"))
            .and_then(Value::as_str)
        {
            Some("interaction.created") => {
                let interaction = value
                    .get("interaction")
                    .ok_or("interaction.created lacks interaction")?;
                self.response_identity = Some(required_string(interaction, "id")?);
                self.frames.push(ProviderFrame::Status(
                    interaction
                        .get("status")
                        .and_then(Value::as_str)
                        .unwrap_or("in_progress")
                        .to_owned(),
                ));
            }
            Some("interaction.status_update") => {
                let status = required_string(value, "status")?;
                self.frames.push(ProviderFrame::Status(status));
            }
            Some("step.start") => {
                let index = required_index(value)?;
                let step = value.get("step").ok_or("step.start lacks step")?;
                if !step.is_object() || step.get("type").and_then(Value::as_str).is_none() {
                    return Err("step.start has an invalid step".to_owned());
                }
                self.interaction_steps.insert(index, step.clone());
                if step.get("type").and_then(Value::as_str) == Some("function_call") {
                    let call = self.calls.entry(index).or_default();
                    call.id = required_string(step, "id")?;
                    call.name = required_string(step, "name")?;
                    call.value_arguments = step.get("arguments").cloned();
                }
            }
            Some("step.delta") => {
                let index = required_index(value)?;
                let delta = value.get("delta").ok_or("step.delta lacks delta")?;
                match delta.get("type").and_then(Value::as_str) {
                    Some("text") => {
                        let text = delta_string(delta, "text")?;
                        append_interaction_content(
                            self.interaction_steps.entry(index).or_insert_with(
                                || serde_json::json!({"content":[],"type":"model_output"}),
                            ),
                            serde_json::json!({"text":text,"type":"text"}),
                        )?;
                        self.push_text(text);
                    }
                    Some("thought") | Some("thinking") => {
                        let text = delta_string(delta, "text")?;
                        append_interaction_content(
                            self.interaction_steps.entry(index).or_insert_with(
                                || serde_json::json!({"content":[],"type":"thought"}),
                            ),
                            serde_json::json!({"text":text,"type":"text"}),
                        )?;
                        self.push_reasoning(text);
                    }
                    Some("thought_summary") => {
                        let content = delta
                            .get("content")
                            .ok_or("thought_summary delta lacks content")?;
                        if content.get("type").and_then(Value::as_str) == Some("text") {
                            let text = required_string(content, "text")?;
                            append_interaction_content(
                                self.interaction_steps.entry(index).or_insert_with(
                                    || serde_json::json!({"summary":[],"type":"thought"}),
                                ),
                                content.clone(),
                            )?;
                            self.push_reasoning(text);
                        }
                    }
                    Some("thought_signature") => {
                        let signature = required_string(delta, "signature")?;
                        let step = self
                            .interaction_steps
                            .entry(index)
                            .or_insert_with(|| serde_json::json!({"type":"thought"}));
                        step.as_object_mut()
                            .ok_or("interaction step must be an object")?
                            .insert("signature".to_owned(), Value::String(signature));
                    }
                    Some("function_call") => {
                        let call = self.calls.entry(index).or_default();
                        if let Some(id) = delta.get("id").and_then(Value::as_str) {
                            call.id = id.to_owned();
                        }
                        let first_name = if call.name.is_empty() {
                            delta.get("name").and_then(Value::as_str).map(str::to_owned)
                        } else {
                            None
                        };
                        if let Some(name) = &first_name {
                            call.name = name.clone();
                        }
                        if let Some(arguments) = delta.get("arguments") {
                            match arguments {
                                Value::String(piece) => {
                                    call.arguments.push_str(piece);
                                    self.frames.push(ProviderFrame::ToolDelta {
                                        call_id: call.id.clone(),
                                        name: first_name,
                                        delta: piece.clone(),
                                    });
                                }
                                value => call.value_arguments = Some(value.clone()),
                            }
                        }
                    }
                    Some("arguments_delta") => {
                        let piece = required_string(delta, "arguments")?;
                        self.calls
                            .entry(index)
                            .or_default()
                            .arguments
                            .push_str(&piece);
                    }
                    Some(_) => merge_delta_fields(
                        self.interaction_steps
                            .entry(index)
                            .or_insert_with(|| serde_json::json!({"type":"unknown"})),
                        delta,
                    )?,
                    None => return Err("interactions delta lacks type".to_owned()),
                }
            }
            Some("step.stop") => {
                let index = required_index(value)?;
                self.mark_call_ready(index, None)?;
            }
            Some("interaction.completed") => {
                if self.done {
                    return Err("duplicate provider terminal".to_owned());
                }
                let interaction = value
                    .get("interaction")
                    .ok_or("interaction.completed lacks interaction")?;
                self.response_identity = interaction
                    .get("id")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .or(self.response_identity.take());
                if let Some(steps) = interaction.get("steps") {
                    if !steps.is_array() {
                        return Err("completed interaction steps must be an array".to_owned());
                    }
                    self.terminal_fragments = Some(steps.clone());
                }
                self.usage = usage_from_any(interaction.get("usage"));
                self.finish_reason = Some(finish_from_reason(
                    interaction.get("status").and_then(Value::as_str),
                    !self.calls.is_empty(),
                )?);
                self.done = true;
            }
            Some("error") => {
                self.finish_reason = Some(if is_context_overflow(value) {
                    FinishReason::ContextOverflow
                } else {
                    FinishReason::ProviderError
                });
                self.done = true;
            }
            Some(_) => {}
            None => return Err("provider event lacks event_type/type".to_owned()),
        }
        Ok(())
    }

    fn record_native_search_call(
        &mut self,
        index: usize,
        item: &Value,
        dialect: Option<DialectId>,
    ) -> Result<(), String> {
        if dialect.is_some_and(|dialect| dialect != DialectId::OpenaiResponsesV1) {
            return Err("native client tool search requires OpenAI Responses".into());
        }
        let native = native_search_call(item)?;
        let arguments =
            serde_json_canonicalizer::to_string(&native.arguments).map_err(|e| e.to_string())?;
        let call = self.calls.entry(index).or_default();
        if (!call.id.is_empty() && call.id != native.call_id)
            || (!call.name.is_empty() && call.name != native.name)
            || (!call.arguments.is_empty() && call.arguments != arguments)
        {
            return Err("native search call changed identity or arguments".into());
        }
        call.id = native.call_id;
        call.name = native.name;
        call.arguments = arguments;
        Ok(())
    }

    fn anthropic(&mut self, value: &Value, dialect: Option<DialectId>) -> Result<(), String> {
        let _ = dialect;
        match value.get("type").and_then(Value::as_str) {
            Some("message_start") => {
                let message = value.get("message").ok_or("message_start lacks message")?;
                self.response_identity =
                    message.get("id").and_then(Value::as_str).map(str::to_owned);
                self.usage = usage_from_any(message.get("usage"));
                self.input_transformations = message
                    .get("input_transformations")
                    .filter(|v| v.is_array())
                    .cloned();
            }
            Some("content_block_start") => {
                let index = required_index(value)?;
                let block = value.get("content_block").ok_or("content block missing")?;
                if !block.is_object() || block.get("type").and_then(Value::as_str).is_none() {
                    return Err("anthropic content block lacks type".to_owned());
                }
                self.anthropic_blocks.insert(index, block.clone());
                match block.get("type").and_then(Value::as_str) {
                    Some("text") => self.push_text(
                        block
                            .get("text")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned(),
                    ),
                    Some("thinking") => self.push_reasoning(
                        block
                            .get("thinking")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned(),
                    ),
                    Some("tool_use") => {
                        let call = self.calls.entry(index).or_default();
                        call.id = required_string(block, "id")?;
                        call.name = required_string(block, "name")?;
                        call.value_arguments = block.get("input").cloned();
                    }
                    Some(_) => {}
                    None => return Err("anthropic content block lacks type".to_owned()),
                }
            }
            Some("content_block_delta") => {
                let index = required_index(value)?;
                let delta = value.get("delta").ok_or("content delta missing")?;
                match delta.get("type").and_then(Value::as_str) {
                    Some("text_delta") => {
                        let text = delta_string(delta, "text")?;
                        append_string_field(self.anthropic_blocks.get_mut(&index), "text", &text)?;
                        self.push_text(text);
                    }
                    Some("thinking_delta") => {
                        let thinking = delta_string(delta, "thinking")?;
                        append_string_field(
                            self.anthropic_blocks.get_mut(&index),
                            "thinking",
                            &thinking,
                        )?;
                        self.push_reasoning(thinking);
                    }
                    Some("signature_delta") => {
                        let signature = required_string(delta, "signature")?;
                        append_string_field(
                            self.anthropic_blocks.get_mut(&index),
                            "signature",
                            &signature,
                        )?;
                    }
                    Some("input_json_delta") => {
                        let piece = delta_string(delta, "partial_json")?;
                        let call = self.calls.entry(index).or_default();
                        if call.arguments_done {
                            return Err("tool arguments changed after the block stopped".to_owned());
                        }
                        call.arguments.push_str(&piece);
                        self.frames.push(ProviderFrame::ToolDelta {
                            call_id: call.id.clone(),
                            name: (!call.name.is_empty()).then(|| call.name.clone()),
                            delta: piece,
                        });
                    }
                    Some(_) => merge_delta_fields(
                        self.anthropic_blocks
                            .get_mut(&index)
                            .ok_or("anthropic delta arrived before content block start")?,
                        delta,
                    )?,
                    None => return Err("anthropic delta lacks type".to_owned()),
                }
            }
            Some("content_block_stop") => {
                let index = required_index(value)?;
                self.mark_call_ready(index, dialect)?;
                self.completed_items.insert(index);
            }
            Some("message_stop") => {
                let indices = self.calls.keys().copied().collect::<Vec<_>>();
                for index in indices {
                    self.mark_call_ready(index, dialect)?;
                }
                self.done = true;
            }
            Some("message_delta") => {
                let delta = value.get("delta").ok_or("message delta missing")?;
                self.finish_reason = Some(finish_from_reason(
                    delta.get("stop_reason").and_then(Value::as_str),
                    !self.calls.is_empty(),
                )?);
                if let Some(details) = delta.get("stop_details").filter(|value| !value.is_null()) {
                    self.stop_details = Some(details.clone());
                }
                if let Some(changes) = delta
                    .get("input_transformations")
                    .or_else(|| value.get("input_transformations"))
                    .filter(|v| v.is_array())
                {
                    self.input_transformations = Some(changes.clone());
                }
                if let Some(usage) = usage_from_any(value.get("usage")) {
                    merge_usage(&mut self.usage, usage);
                }
            }
            Some("ping") => {}
            Some(_) => {}
            None => return Err("provider event lacks type".to_owned()),
        }
        Ok(())
    }

    fn chat(&mut self, value: &Value, dialect: Option<DialectId>) -> Result<(), String> {
        self.response_identity = self
            .response_identity
            .take()
            .or_else(|| value.get("id").and_then(Value::as_str).map(str::to_owned));
        if let Some(usage) = usage_from_any(value.get("usage")) {
            self.usage = Some(usage);
        }
        for choice in value
            .get("choices")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            // Kimi K3 reports stream usage on the terminal choice, rather than
            // the top-level usage-only chunk used by other Chat dialects.
            // Do not aggregate alternate choices or override top-level usage.
            if dialect == Some(DialectId::KimiChatV1)
                && choice.get("index").and_then(Value::as_u64) == Some(0)
                && usage_from_any(value.get("usage")).is_none()
            {
                if let Some(usage) = usage_from_any(choice.get("usage")) {
                    self.usage = Some(usage);
                }
            }
            if let Some(delta) = choice.get("delta") {
                let delta_object = delta
                    .as_object()
                    .ok_or("chat choice delta must be an object")?;
                if self.finish_reason.is_some()
                    && delta_object.values().any(|value| !value.is_null())
                {
                    return Err("chat content follows finish_reason".to_owned());
                }
                for (key, value) in delta_object {
                    if matches!(
                        key.as_str(),
                        "content" | "reasoning" | "reasoning_content" | "tool_calls"
                    ) {
                        continue;
                    }
                    self.chat_message.insert(key.clone(), value.clone());
                }
                if let Some(text) = delta.get("content").and_then(Value::as_str) {
                    append_map_string(&mut self.chat_message, "content", text)?;
                    self.push_text(text.to_owned());
                }
                let reasoning_field = if delta.get("reasoning_content").is_some() {
                    "reasoning_content"
                } else {
                    "reasoning"
                };
                if let Some(reasoning) = delta.get(reasoning_field).and_then(Value::as_str) {
                    append_map_string(&mut self.chat_message, reasoning_field, reasoning)?;
                    self.push_reasoning(reasoning.to_owned());
                }
                for item in delta
                    .get("tool_calls")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    let index = item.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                    merge_chat_tool_delta(
                        self.chat_tool_calls
                            .entry(index)
                            .or_insert_with(|| serde_json::json!({})),
                        item,
                    )?;
                    let call = self.calls.entry(index).or_default();
                    if let Some(id) = item.get("id").and_then(Value::as_str) {
                        call.id = id.to_owned();
                    }
                    if let Some(function) = item.get("function") {
                        let first_name = if call.name.is_empty() {
                            function
                                .get("name")
                                .and_then(Value::as_str)
                                .map(str::to_owned)
                        } else {
                            None
                        };
                        if let Some(name) = &first_name {
                            call.name = name.clone();
                        }
                        let piece = function
                            .get("arguments")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned();
                        call.arguments.push_str(&piece);
                        self.frames.push(ProviderFrame::ToolDelta {
                            call_id: call.id.clone(),
                            name: first_name,
                            delta: piece,
                        });
                    }
                }
            }
            if let Some(reason) = choice.get("finish_reason").and_then(Value::as_str) {
                self.finish_reason =
                    Some(finish_from_reason(Some(reason), !self.calls.is_empty())?);
                // Chat streams have no per-call completion event; the finish
                // signal is the first point every call is complete.
                let indices = self.calls.keys().copied().collect::<Vec<_>>();
                for index in indices {
                    self.mark_call_ready(index, dialect)?;
                }
            }
        }
        Ok(())
    }

    fn google(&mut self, value: &Value) -> Result<(), String> {
        self.response_identity = self.response_identity.take().or_else(|| {
            value
                .get("responseId")
                .and_then(Value::as_str)
                .map(str::to_owned)
        });
        if let Some(usage) = usage_from_any(value.get("usageMetadata")) {
            self.usage = Some(usage);
        }
        for candidate in value
            .get("candidates")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            for part in candidate
                .pointer("/content/parts")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                self.google_parts.push(part.clone());
                if let Some(text) = part.get("text").and_then(Value::as_str) {
                    if part
                        .get("thought")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        self.push_reasoning(text.to_owned());
                    } else {
                        self.push_text(text.to_owned());
                    }
                }
                if let Some(function) = part.get("functionCall") {
                    let index = self.calls.len();
                    let call = self.calls.entry(index).or_default();
                    call.id = required_string(function, "id")?;
                    call.name = required_string(function, "name")?;
                    call.value_arguments = function.get("args").cloned();
                    self.mark_call_ready(index, None)?;
                }
            }
            if let Some(reason) = candidate.get("finishReason").and_then(Value::as_str) {
                self.finish_reason =
                    Some(finish_from_reason(Some(reason), !self.calls.is_empty())?);
                self.done = true;
            }
        }
        Ok(())
    }

    fn push_text(&mut self, value: String) {
        self.text.push_str(&value);
        self.frames.push(ProviderFrame::TextDelta(value));
    }

    fn push_reasoning(&mut self, value: String) {
        self.reasoning.push_str(&value);
        self.frames.push(ProviderFrame::ReasoningDelta(value));
    }

    fn finish(
        mut self,
        adapter: AdapterId,
        dialect: Option<DialectId>,
        bytes: &[u8],
    ) -> Result<(Vec<ProviderFrame>, ProviderTerminal), String> {
        if !self.done {
            return Err("provider stream ended without terminal".to_owned());
        }
        let mut content = Vec::new();
        if !self.reasoning.is_empty() {
            content.push(ContentBlock::Reasoning(
                self.reasoning.trim_end().to_owned(),
            ));
        }
        if !self.text.is_empty() {
            content.push(ContentBlock::Text(self.text));
        }
        let mut tool_calls = Vec::new();
        for (index, call) in self.calls {
            if call.id.is_empty() || call.name.is_empty() {
                return Err("missing tool call identity".to_owned());
            }
            let arguments = resolve_call_arguments_or_sentinel(&call, dialect);
            if let Some(block) = self.anthropic_blocks.get_mut(&index) {
                block
                    .as_object_mut()
                    .ok_or("anthropic content block must be an object")?
                    .insert("input".to_owned(), arguments.clone());
            }
            if let Some(step) = self.interaction_steps.get_mut(&index) {
                step.as_object_mut()
                    .ok_or("interaction step must be an object")?
                    .insert("arguments".to_owned(), arguments.clone());
            }
            if let Some(item) = self.response_items.get_mut(&index) {
                let object = item
                    .as_object_mut()
                    .ok_or("Responses output item must be an object")?;
                object.insert(
                    "arguments".to_owned(),
                    Value::String(
                        serde_json_canonicalizer::to_string(&arguments)
                            .map_err(|error| error.to_string())?,
                    ),
                );
                object.insert("call_id".to_owned(), Value::String(call.id.clone()));
                object.insert("name".to_owned(), Value::String(call.name.clone()));
            }
            tool_calls.push(ToolCall {
                call_id: call.id,
                name: call.name,
                arguments,
            });
        }
        let reason = self.finish_reason.unwrap_or({
            if tool_calls.is_empty() {
                FinishReason::Completed
            } else {
                FinishReason::ToolCalls
            }
        });
        let mut ids = std::collections::BTreeSet::new();
        if tool_calls
            .iter()
            .any(|call| !ids.insert(call.call_id.clone()))
        {
            return Err("duplicate tool call identity".to_owned());
        }
        if !tool_calls.is_empty() && adapter == AdapterId::ChatCompletion {
            let mut native_calls = self.chat_tool_calls;
            for (native, call) in native_calls.values_mut().zip(tool_calls.iter()) {
                let object = native
                    .as_object_mut()
                    .ok_or("chat tool call must be an object")?;
                object.remove("index");
                object.insert("id".to_owned(), Value::String(call.call_id.clone()));
                object
                    .entry("type".to_owned())
                    .or_insert_with(|| Value::String("function".to_owned()));
                let function = object
                    .entry("function".to_owned())
                    .or_insert_with(|| serde_json::json!({}))
                    .as_object_mut()
                    .ok_or("chat tool call function must be an object")?;
                function.insert("name".to_owned(), Value::String(call.name.clone()));
                function.insert(
                    "arguments".to_owned(),
                    Value::String(
                        serde_json_canonicalizer::to_string(&call.arguments)
                            .map_err(|error| error.to_string())?,
                    ),
                );
            }
            self.chat_message.insert(
                "tool_calls".to_owned(),
                Value::Array(native_calls.into_values().collect()),
            );
        }
        if adapter == AdapterId::ChatCompletion && !self.chat_message.contains_key("role") {
            self.chat_message
                .insert("role".to_owned(), Value::String("assistant".to_owned()));
        }
        let sealed_fragments = self
            .terminal_fragments
            .take()
            .unwrap_or_else(|| match adapter {
                AdapterId::Responses if !self.response_items.is_empty() => {
                    Value::Array(self.response_items.into_values().collect())
                }
                AdapterId::Anthropic if !self.anthropic_blocks.is_empty() => {
                    Value::Array(self.anthropic_blocks.into_values().collect())
                }
                AdapterId::GoogleInteractions if !self.interaction_steps.is_empty() => {
                    Value::Array(self.interaction_steps.into_values().collect())
                }
                AdapterId::ChatCompletion if !self.chat_message.is_empty() => {
                    Value::Object(self.chat_message)
                }
                AdapterId::GoogleGeneration if !self.google_parts.is_empty() => serde_json::json!({
                    "parts": self.google_parts,
                    "role": "model"
                }),
                _ => sealed_from_normalized(adapter, dialect, &content, &tool_calls),
            });
        let sealed_fragments = if adapter == AdapterId::Responses {
            complete_responses_fragments(sealed_fragments, dialect, &content, &tool_calls)
        } else {
            sealed_fragments
        };
        // Some valid streams carry the text only in the terminal output items.
        // Do not discard that answer, or fabricate finality from deltas alone.
        if adapter == AdapterId::Responses
            && !content
                .iter()
                .any(|block| matches!(block, ContentBlock::Text(_)))
        {
            let terminal_output = normalize_responses(
                bytes,
                &serde_json::json!({
                    "output": sealed_fragments, "status": "completed"
                }),
            )?;
            content.extend(
                terminal_output
                    .content
                    .into_iter()
                    .filter(|block| matches!(block, ContentBlock::Text(_))),
            );
        }
        Ok((
            self.frames,
            ProviderTerminal {
                content,
                tool_calls,
                sealed_fragments,
                response_identity: self.response_identity,
                usage: self.usage,
                finish_reason: reason,
                incomplete_reason: self.incomplete_reason,
                provider_error: self.provider_error,
                http_status: None,
                stop_details: self.stop_details,
                input_transformations: self.input_transformations,
                raw_response: bytes.to_vec(),
            },
        ))
    }
}

fn sealed_from_normalized(
    adapter: AdapterId,
    dialect: Option<DialectId>,
    content: &[ContentBlock],
    calls: &[ToolCall],
) -> Value {
    let text_parts = content.iter().filter_map(|block| match block {
        ContentBlock::Text(text) => Some(text.as_str()),
        ContentBlock::Reasoning(_) => None,
    });
    let reasoning_parts = content.iter().filter_map(|block| match block {
        ContentBlock::Reasoning(text) => Some(text.as_str()),
        ContentBlock::Text(_) => None,
    });
    match adapter {
        AdapterId::Responses if dialect == Some(DialectId::DeepseekResponsesV1) => Value::Array(
            reasoning_parts
                .map(|text| serde_json::json!({"content":[{"text":text,"type":"reasoning_text"}],"type":"reasoning"}))
                .chain(text_parts.map(|text| serde_json::json!({"content":[{"text":text,"type":"output_text"}],"role":"assistant","type":"message"})))
                .chain(calls.iter().map(|call| serde_json::json!({"arguments":serde_json_canonicalizer::to_string(&call.arguments).expect("validated I-JSON"),"call_id":call.call_id,"name":call.name,"type":"function_call"})))
                .collect(),
        ),
        AdapterId::Responses => Value::Array(
            reasoning_parts
                .map(|text| serde_json::json!({"summary":[{"text":text,"type":"summary_text"}],"type":"reasoning"}))
                .chain(text_parts.map(|text| serde_json::json!({"content":[{"text":text,"type":"output_text"}],"role":"assistant","type":"message"})))
                .chain(calls.iter().map(|call| serde_json::json!({"arguments":serde_json_canonicalizer::to_string(&call.arguments).expect("validated I-JSON"),"call_id":call.call_id,"name":call.name,"type":"function_call"})))
                .collect(),
        ),
        AdapterId::Anthropic => Value::Array(
            reasoning_parts
                .map(|text| serde_json::json!({"thinking":text,"type":"thinking"}))
                .chain(text_parts.map(|text| serde_json::json!({"text":text,"type":"text"})))
                .chain(calls.iter().map(|call| serde_json::json!({"id":call.call_id,"input":call.arguments,"name":call.name,"type":"tool_use"})))
                .collect(),
        ),
        AdapterId::ChatCompletion => serde_json::json!({
            "content": text_parts.collect::<Vec<_>>().join(""),
            "reasoning_content": reasoning_parts.collect::<Vec<_>>().join(""),
            "role":"assistant",
            "tool_calls":calls.iter().map(|call| serde_json::json!({"function":{"arguments":serde_json_canonicalizer::to_string(&call.arguments).expect("validated I-JSON"),"name":call.name},"id":call.call_id,"type":"function"})).collect::<Vec<_>>()
        }),
        AdapterId::GoogleGeneration => serde_json::json!({
            "parts": reasoning_parts
                .map(|text| serde_json::json!({"text":text,"thought":true}))
                .chain(text_parts.map(|text| serde_json::json!({"text":text})))
                .chain(calls.iter().map(|call| serde_json::json!({"functionCall":{"args":call.arguments,"id":call.call_id,"name":call.name}})))
                .collect::<Vec<_>>(),
            "role":"model"
        }),
        AdapterId::GoogleInteractions => Value::Array(
            reasoning_parts
                .map(|text| serde_json::json!({"content":[{"text":text,"type":"thought"}],"type":"model_output"}))
                .chain(text_parts.map(|text| serde_json::json!({"content":[{"text":text,"type":"text"}],"type":"model_output"})))
                .chain(calls.iter().map(|call| serde_json::json!({"arguments":call.arguments,"id":call.call_id,"name":call.name,"type":"function_call"})))
                .collect(),
        ),
    }
}

fn complete_responses_fragments(
    fragments: Value,
    dialect: Option<DialectId>,
    content: &[ContentBlock],
    calls: &[ToolCall],
) -> Value {
    let Value::Array(mut fragments) = fragments else {
        return fragments;
    };
    let has_reasoning = fragments
        .iter()
        .any(|item| item.get("type").and_then(Value::as_str) == Some("reasoning"));
    let has_message = fragments
        .iter()
        .any(|item| item.get("type").and_then(Value::as_str) == Some("message"));
    let call_ids = fragments
        .iter()
        .filter(|item| {
            matches!(
                item.get("type").and_then(Value::as_str),
                Some("function_call" | "tool_search_call")
            )
        })
        .filter_map(|item| item.get("call_id").and_then(Value::as_str))
        .collect::<std::collections::BTreeSet<_>>();
    let Value::Array(normalized) =
        sealed_from_normalized(AdapterId::Responses, dialect, content, calls)
    else {
        unreachable!("Responses sealed fragments are an array")
    };
    let mut missing = Vec::new();
    for item in normalized {
        match item.get("type").and_then(Value::as_str) {
            Some("reasoning") if !has_reasoning => missing.push(item),
            Some("message") if !has_message => missing.push(item),
            Some("function_call")
                if item
                    .get("call_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| !call_ids.contains(id)) =>
            {
                missing.push(item);
            }
            _ => {}
        }
    }
    missing.append(&mut fragments);
    Value::Array(missing)
}

fn append_string_field(value: Option<&mut Value>, field: &str, piece: &str) -> Result<(), String> {
    let Some(value) = value else {
        return Ok(());
    };
    let object = value
        .as_object_mut()
        .ok_or_else(|| "provider native fragment must be an object".to_owned())?;
    append_map_string(object, field, piece)
}

fn append_map_string(
    object: &mut serde_json::Map<String, Value>,
    field: &str,
    piece: &str,
) -> Result<(), String> {
    match object.get_mut(field) {
        Some(Value::String(value)) => value.push_str(piece),
        Some(_) => {
            return Err(format!(
                "provider native fragment field {field} must be a string"
            ));
        }
        None => {
            object.insert(field.to_owned(), Value::String(piece.to_owned()));
        }
    }
    Ok(())
}

fn append_interaction_content(step: &mut Value, part: Value) -> Result<(), String> {
    let object = step
        .as_object_mut()
        .ok_or_else(|| "interaction step must be an object".to_owned())?;
    let field = if object.get("type").and_then(Value::as_str) == Some("thought") {
        "summary"
    } else {
        "content"
    };
    match object
        .entry(field.to_owned())
        .or_insert_with(|| Value::Array(Vec::new()))
    {
        Value::Array(parts) => parts.push(part),
        _ => return Err(format!("interaction step {field} must be an array")),
    }
    Ok(())
}

fn merge_delta_fields(target: &mut Value, delta: &Value) -> Result<(), String> {
    let target = target
        .as_object_mut()
        .ok_or_else(|| "provider native fragment must be an object".to_owned())?;
    let delta = delta
        .as_object()
        .ok_or_else(|| "provider delta must be an object".to_owned())?;
    for (key, value) in delta {
        if key == "type" {
            continue;
        }
        match (target.get_mut(key), value) {
            (Some(Value::String(existing)), Value::String(piece)) => existing.push_str(piece),
            (Some(Value::Object(existing)), Value::Object(fields)) => {
                merge_object_delta(existing, fields);
            }
            _ => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}

fn merge_object_delta(
    target: &mut serde_json::Map<String, Value>,
    delta: &serde_json::Map<String, Value>,
) {
    for (key, value) in delta {
        match (target.get_mut(key), value) {
            (Some(Value::String(existing)), Value::String(piece)) => existing.push_str(piece),
            (Some(Value::Object(existing)), Value::Object(fields)) => {
                merge_object_delta(existing, fields);
            }
            _ => {
                target.insert(key.clone(), value.clone());
            }
        }
    }
}

fn merge_chat_tool_delta(target: &mut Value, item: &Value) -> Result<(), String> {
    let target = target
        .as_object_mut()
        .ok_or_else(|| "chat tool call must be an object".to_owned())?;
    let item = item
        .as_object()
        .ok_or_else(|| "chat tool call delta must be an object".to_owned())?;
    for (key, value) in item {
        if key == "index" {
            continue;
        }
        if key == "function" {
            let function = value
                .as_object()
                .ok_or_else(|| "chat tool call function must be an object".to_owned())?;
            let native = target
                .entry(key.clone())
                .or_insert_with(|| serde_json::json!({}))
                .as_object_mut()
                .ok_or_else(|| "chat tool call function must be an object".to_owned())?;
            for (field, value) in function {
                if field == "arguments" {
                    let piece = value
                        .as_str()
                        .ok_or_else(|| "chat tool arguments delta must be a string".to_owned())?;
                    append_map_string(native, field, piece)?;
                } else {
                    native.insert(field.clone(), value.clone());
                }
            }
        } else {
            target.insert(key.clone(), value.clone());
        }
    }
    Ok(())
}

fn upsert_response_part(
    items: &mut std::collections::BTreeMap<usize, Value>,
    event: &Value,
    part: Value,
) -> Result<(), String> {
    let output_index = optional_usize(event, "output_index")?.unwrap_or(0);
    let content_index = optional_usize(event, "content_index")?.unwrap_or(0);
    let item = items
        .entry(output_index)
        .or_insert_with(|| serde_json::json!({"content":[],"role":"assistant","type":"message"}));
    let content = item
        .as_object_mut()
        .ok_or_else(|| "Responses output item must be an object".to_owned())?
        .entry("content".to_owned())
        .or_insert_with(|| Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| "Responses message content must be an array".to_owned())?;
    match content_index.cmp(&content.len()) {
        std::cmp::Ordering::Less => content[content_index] = part,
        std::cmp::Ordering::Equal => content.push(part),
        std::cmp::Ordering::Greater => {
            return Err("Responses content part index is not contiguous".to_owned());
        }
    }
    Ok(())
}

fn append_response_part_text(
    items: &mut std::collections::BTreeMap<usize, Value>,
    event: &Value,
    field: &str,
    piece: &str,
) -> Result<(), String> {
    let (Some(output_index), Some(content_index)) = (
        event.get("output_index").and_then(Value::as_u64),
        event.get("content_index").and_then(Value::as_u64),
    ) else {
        return Ok(());
    };
    let Some(item) = items.get_mut(&(output_index as usize)) else {
        return Ok(());
    };
    let Some(parts) = item.get_mut(field).and_then(Value::as_array_mut) else {
        return Ok(());
    };
    let Some(part) = parts.get_mut(content_index as usize) else {
        return Ok(());
    };
    append_string_field(Some(part), "text", piece)
}

fn required_usize(value: &Value, field: &str) -> Result<usize, String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| format!("provider event lacks valid {field}"))
}

fn optional_usize(value: &Value, field: &str) -> Result<Option<usize>, String> {
    match value.get(field) {
        Some(value) => value
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .map(Some)
            .ok_or_else(|| format!("provider event has invalid {field}")),
        None => Ok(None),
    }
}

fn required_index(value: &Value) -> Result<usize, String> {
    required_usize(value, "index")
}

fn merge_usage(target: &mut Option<Usage>, incoming: Usage) {
    let value = target.get_or_insert_with(Usage::default);
    if incoming.input_tokens.is_some() {
        value.input_tokens = incoming.input_tokens;
    }
    if incoming.output_tokens.is_some() {
        value.output_tokens = incoming.output_tokens;
    }
    if incoming.cache_write.is_some() {
        value.cache_write = incoming.cache_write;
    }
    if incoming.cache_read.is_some() {
        value.cache_read = incoming.cache_read;
    }
    if incoming.reasoning_tokens.is_some() {
        value.reasoning_tokens = incoming.reasoning_tokens;
    }
}

fn finish_from_reason(reason: Option<&str>, has_calls: bool) -> Result<FinishReason, String> {
    match reason.unwrap_or("completed").to_ascii_lowercase().as_str() {
        "completed" | "stop" | "end_turn" => Ok(if has_calls {
            FinishReason::ToolCalls
        } else {
            FinishReason::Completed
        }),
        "tool_calls" | "tool_use" | "requires_action" => Ok(FinishReason::ToolCalls),
        "pause_turn" => Ok(FinishReason::Paused),
        "length" | "max_tokens" => Ok(FinishReason::Length),
        "context_length_exceeded" | "context_window_exceeded" | "max_context_length_exceeded" => {
            Ok(FinishReason::ContextOverflow)
        }
        "content_filter" | "safety" | "recitation" | "refusal" => Ok(FinishReason::ContentFilter),
        "failed" | "error" | "incomplete" => Ok(FinishReason::ProviderError),
        other => Err(format!("unknown finish reason {other}")),
    }
}

fn normalize_legacy_result(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        return Ok(ProviderTerminal {
            content: Vec::new(),
            tool_calls: Vec::new(),
            sealed_fragments: value.clone(),
            response_identity: string_at(value, &["rawMetadata", "response_id"]),
            usage: usage_from_legacy(value.get("usage")),
            finish_reason: if error.contains("blocked") {
                FinishReason::ContentFilter
            } else {
                FinishReason::ProviderError
            },
            incomplete_reason: None,
            provider_error: Some(Value::String(error.to_owned())),
            http_status: None,
            stop_details: None,
            input_transformations: None,
            raw_response: bytes.to_vec(),
        });
    }
    let mut content = Vec::new();
    if let Some(reasoning) = value.get("reasoningData").and_then(Value::as_str) {
        if !reasoning.is_empty() {
            content.push(ContentBlock::Reasoning(reasoning.to_owned()));
        }
    }
    if let Some(text) = value.get("text").and_then(Value::as_str) {
        if !text.is_empty() {
            content.push(ContentBlock::Text(text.to_owned()));
        }
    }
    let tool_calls = value
        .get("functionCalls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|call| {
            let arguments = call
                .get("arguments")
                .and_then(Value::as_str)
                .ok_or_else(|| "tool arguments missing".to_owned())?;
            Ok(ToolCall {
                call_id: required_string(call, "id")?,
                name: required_string(call, "name")?,
                arguments: parse_foreign_value(arguments.as_bytes(), "tool arguments")?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let finish_reason = if tool_calls.is_empty() {
        FinishReason::Completed
    } else {
        FinishReason::ToolCalls
    };
    Ok(ProviderTerminal {
        content,
        tool_calls,
        sealed_fragments: value.clone(),
        response_identity: string_at(value, &["rawMetadata", "response_id"]),
        usage: usage_from_legacy(value.get("usage")),
        finish_reason,
        incomplete_reason: None,
        provider_error: None,
        http_status: None,
        stop_details: None,
        input_transformations: None,
        raw_response: bytes.to_vec(),
    })
}

fn native_search_call(item: &Value) -> Result<ToolCall, String> {
    if item["execution"] != "client" || item["status"] != "completed" {
        return Err("native search call must be a completed client execution".into());
    }
    if item.get("name").is_some_and(|name| name != "tool_search") {
        return Err("native search call has an unexpected name".into());
    }
    let call_id = required_string(item, "call_id")?;
    if call_id.is_empty() {
        return Err("native search call lacks call identity".into());
    }
    let arguments = item
        .get("arguments")
        .filter(|v| v.is_object())
        .ok_or("native search arguments must be an object")?
        .clone();
    Ok(ToolCall {
        call_id,
        name: "tool_search".into(),
        arguments,
    })
}

fn normalize_responses(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    let mut content = Vec::new();
    let mut tool_calls = Vec::new();
    for item in value
        .get("output")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        match item.get("type").and_then(Value::as_str) {
            Some("message") => {
                let parts = item
                    .get("content")
                    .and_then(Value::as_array)
                    .ok_or_else(|| "Responses message lacks content array".to_owned())?;
                for part in parts {
                    if !part.is_object() || part.get("type").and_then(Value::as_str).is_none() {
                        return Err("Responses message part lacks type".to_owned());
                    }
                    if let Some(text) = part.get("text").and_then(Value::as_str) {
                        content.push(ContentBlock::Text(text.to_owned()));
                    }
                }
            }
            Some("reasoning") => match item.get("summary").or_else(|| item.get("content")) {
                Some(Value::String(text)) => content.push(ContentBlock::Reasoning(text.clone())),
                Some(Value::Array(parts)) => {
                    for part in parts {
                        if !part.is_object() || part.get("type").and_then(Value::as_str).is_none() {
                            return Err("Responses reasoning part lacks type".to_owned());
                        }
                        if let Some(text) = part.get("text").and_then(Value::as_str) {
                            content.push(ContentBlock::Reasoning(text.to_owned()));
                        }
                    }
                }
                _ => {}
            },
            Some("tool_search_call") => tool_calls.push(native_search_call(item)?),
            Some("function_call") => tool_calls.push(ToolCall {
                call_id: required_string(item, "call_id")?,
                name: required_string(item, "name")?,
                arguments: parse_arguments(item.get("arguments"))?,
            }),
            Some(_) => {}
            None => return Err("Responses output item lacks type".to_owned()),
        }
    }
    terminal(
        bytes,
        content,
        tool_calls,
        value
            .get("output")
            .cloned()
            .unwrap_or(Value::Array(Vec::new())),
        value.get("id").and_then(Value::as_str),
        value.get("usage"),
        value.get("status").and_then(Value::as_str),
    )
}

fn normalize_interactions(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    if value.get("object").and_then(Value::as_str) != Some("interaction") {
        return Err("interactions response lacks object=interaction".to_owned());
    }
    let mut content = Vec::new();
    let mut calls = Vec::new();
    for step in value
        .get("steps")
        .and_then(Value::as_array)
        .ok_or_else(|| "interactions response lacks steps".to_owned())?
    {
        match step.get("type").and_then(Value::as_str) {
            Some("model_output") => {
                for part in step
                    .get("content")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    match part.get("type").and_then(Value::as_str) {
                        Some("text") => {
                            content.push(ContentBlock::Text(required_string(part, "text")?));
                        }
                        Some("thought") | Some("thinking") => {
                            content.push(ContentBlock::Reasoning(required_string(part, "text")?));
                        }
                        Some(_) => {}
                        None => return Err("interactions content lacks type".to_owned()),
                    }
                }
            }
            Some("function_call") => calls.push(ToolCall {
                call_id: required_string(step, "id")?,
                name: required_string(step, "name")?,
                arguments: parse_arguments(step.get("arguments"))?,
            }),
            Some(_) => {}
            None => return Err("interactions step lacks type".to_owned()),
        }
    }
    terminal(
        bytes,
        content,
        calls,
        value
            .get("steps")
            .cloned()
            .unwrap_or(Value::Array(Vec::new())),
        value.get("id").and_then(Value::as_str),
        value.get("usage"),
        value.get("status").and_then(Value::as_str),
    )
}

fn normalize_anthropic(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    let mut content = Vec::new();
    let mut calls = Vec::new();
    let parts = value
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(|| "Anthropic response lacks content array".to_owned())?;
    for part in parts {
        match part.get("type").and_then(Value::as_str) {
            Some("text") => content.push(ContentBlock::Text(required_string(part, "text")?)),
            Some("thinking") => {
                if let Some(thinking) = part.get("thinking").and_then(Value::as_str) {
                    if !thinking.is_empty() {
                        content.push(ContentBlock::Reasoning(thinking.to_owned()));
                    }
                }
            }
            Some("tool_use") => calls.push(ToolCall {
                call_id: required_string(part, "id")?,
                name: required_string(part, "name")?,
                arguments: parse_foreign_value(
                    &serde_json::to_vec(
                        part.get("input")
                            .unwrap_or(&Value::Object(Default::default())),
                    )
                    .map_err(|error| error.to_string())?,
                    "tool arguments",
                )?,
            }),
            Some(_) => {}
            None => return Err("Anthropic content block lacks type".to_owned()),
        }
    }
    let mut terminal = terminal(
        bytes,
        content,
        calls,
        value
            .get("content")
            .cloned()
            .unwrap_or(Value::Array(Vec::new())),
        value.get("id").and_then(Value::as_str),
        value.get("usage"),
        value.get("stop_reason").and_then(Value::as_str),
    )?;
    terminal.stop_details = value
        .get("stop_details")
        .filter(|value| !value.is_null())
        .cloned();
    terminal.input_transformations = value
        .get("input_transformations")
        .filter(|v| v.is_array())
        .cloned();
    Ok(terminal)
}

fn normalize_chat(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    let choice = value
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .ok_or_else(|| "chat response has no choice".to_owned())?;
    let message = choice
        .get("message")
        .ok_or_else(|| "chat choice has no message".to_owned())?;
    if !message.is_object() {
        return Err("chat choice message must be an object".to_owned());
    }
    let mut content = Vec::new();
    if let Some(reasoning) = message.get("reasoning_content").and_then(Value::as_str) {
        content.push(ContentBlock::Reasoning(reasoning.to_owned()));
    }
    if let Some(text) = message.get("content").and_then(Value::as_str) {
        if !text.is_empty() {
            content.push(ContentBlock::Text(text.to_owned()));
        }
    }
    let calls = message
        .get("tool_calls")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|call| {
            let function = call
                .get("function")
                .ok_or_else(|| "chat tool call has no function".to_owned())?;
            Ok(ToolCall {
                call_id: required_string(call, "id")?,
                name: required_string(function, "name")?,
                arguments: parse_arguments(function.get("arguments"))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    terminal(
        bytes,
        content,
        calls,
        message.clone(),
        value.get("id").and_then(Value::as_str),
        value.get("usage"),
        choice.get("finish_reason").and_then(Value::as_str),
    )
}

fn normalize_google(bytes: &[u8], value: &Value) -> Result<ProviderTerminal, String> {
    if value.get("promptFeedback").is_some() && value.get("candidates").is_none() {
        return terminal(
            bytes,
            Vec::new(),
            Vec::new(),
            Value::Array(Vec::new()),
            None,
            value.get("usageMetadata"),
            Some("SAFETY"),
        );
    }
    let candidate = value
        .get("candidates")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .ok_or_else(|| "google response has no candidate".to_owned())?;
    let mut content = Vec::new();
    let mut calls = Vec::new();
    for part in candidate
        .pointer("/content/parts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if let Some(text) = part.get("text").and_then(Value::as_str) {
            if part
                .get("thought")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                content.push(ContentBlock::Reasoning(text.to_owned()));
            } else {
                content.push(ContentBlock::Text(text.to_owned()));
            }
        }
        if let Some(call) = part.get("functionCall") {
            calls.push(ToolCall {
                call_id: required_string(call, "id")?,
                name: required_string(call, "name")?,
                arguments: parse_foreign_value(
                    &serde_json::to_vec(
                        call.get("args")
                            .unwrap_or(&Value::Object(Default::default())),
                    )
                    .map_err(|error| error.to_string())?,
                    "tool arguments",
                )?,
            });
        }
    }
    terminal(
        bytes,
        content,
        calls,
        candidate
            .get("content")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
        value.get("responseId").and_then(Value::as_str),
        value.get("usageMetadata"),
        candidate.get("finishReason").and_then(Value::as_str),
    )
}

fn terminal(
    bytes: &[u8],
    content: Vec<ContentBlock>,
    tool_calls: Vec<ToolCall>,
    sealed_fragments: Value,
    id: Option<&str>,
    usage: Option<&Value>,
    reason: Option<&str>,
) -> Result<ProviderTerminal, String> {
    let mut ids = std::collections::BTreeSet::new();
    if tool_calls.iter().any(|call| {
        call.call_id.is_empty() || call.name.is_empty() || !ids.insert(call.call_id.clone())
    }) {
        return Err("missing or duplicate tool call identity".to_owned());
    }
    for call in &tool_calls {
        validate_foreign_value(&call.arguments, "tool arguments")?;
    }
    let finish_reason = finish_from_reason(reason, !tool_calls.is_empty())?;
    Ok(ProviderTerminal {
        content,
        tool_calls,
        sealed_fragments,
        response_identity: id.map(str::to_owned),
        usage: usage_from_any(usage),
        finish_reason,
        incomplete_reason: None,
        provider_error: None,
        http_status: None,
        stop_details: None,
        input_transformations: None,
        raw_response: bytes.to_vec(),
    })
}

fn usage_from_legacy(value: Option<&Value>) -> Option<Usage> {
    value.filter(|value| !value.is_null()).map(|value| Usage {
        input_tokens: numeric_string(value.get("inputTokens")),
        output_tokens: numeric_string(value.get("outputTokens")),
        cache_read: numeric_string(value.get("cachedInputTokens")),
        cache_miss: None,
        cache_write: numeric_string(value.get("cacheCreationInputTokens")),
        reasoning_tokens: numeric_string(value.get("reasoningTokens")),
    })
}

fn usage_from_any(value: Option<&Value>) -> Option<Usage> {
    value.filter(|value| !value.is_null()).map(|value| Usage {
        input_tokens: first_numeric(
            value,
            &[
                "input_tokens",
                "inputTokens",
                "prompt_tokens",
                "promptTokenCount",
                "total_input_tokens",
            ],
        ),
        output_tokens: first_numeric(
            value,
            &[
                "output_tokens",
                "outputTokens",
                "completion_tokens",
                "candidatesTokenCount",
                "total_output_tokens",
            ],
        ),
        cache_read: first_numeric(
            value,
            &[
                "prompt_cache_hit_tokens",
                "cached_tokens",
                "cache_read_input_tokens",
                "cachedContentTokenCount",
                "total_cached_tokens",
            ],
        )
        .or_else(|| numeric_string(value.pointer("/input_tokens_details/cached_tokens")))
        .or_else(|| numeric_string(value.pointer("/prompt_tokens_details/cached_tokens"))),
        cache_miss: numeric_string(value.get("prompt_cache_miss_tokens")),
        cache_write: numeric_string(value.get("cache_creation_input_tokens")),
        reasoning_tokens: first_numeric(
            value,
            &["reasoningTokens", "thoughtsTokenCount", "reasoning_tokens"],
        )
        .or_else(|| numeric_string(value.pointer("/output_tokens_details/reasoning_tokens")))
        .or_else(|| numeric_string(value.pointer("/output_tokens_details/thinking_tokens")))
        .or_else(|| numeric_string(value.pointer("/completion_tokens_details/reasoning_tokens"))),
    })
}

fn first_numeric(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| numeric_string(value.get(*key)))
}

#[cfg(test)]
mod reasoning_usage_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn streaming_usage_follows_content_and_keeps_zero_and_partial_reports() {
        let mut decoder = ProviderStreamDecoder::new(AdapterId::ChatCompletion);
        for (text, usage, expected) in [
            ("hello", json!({"completion_tokens":3}), Some("3")),
            ("", json!({"completion_tokens":0}), Some("0")),
            ("next", json!({"prompt_tokens":10}), None),
        ] {
            let bytes = format!(
                "data: {}\n\n",
                json!({"choices":[{"delta":{"content":text}}],"usage":usage})
            );
            let frames = decoder.push(bytes.as_bytes()).unwrap();
            let ProviderFrame::UsageDelta(report) = frames.last().unwrap() else {
                panic!("missing report")
            };
            assert_eq!(report.output_tokens.as_deref(), expected);
            if !text.is_empty() {
                assert!(matches!(frames.first(), Some(ProviderFrame::TextDelta(_))));
            }
        }
    }

    #[test]
    fn reported_reasoning_survives_partial_usage_without_inventing_missing_values() {
        for body in [
            json!({"thoughtsTokenCount":0}),
            json!({"reasoningTokens":0}),
            json!({"output_tokens_details":{"reasoning_tokens":0}}),
            json!({"completion_tokens_details":{"reasoning_tokens":0}}),
        ] {
            let mut usage = usage_from_any(Some(&body));
            assert_eq!(
                usage.as_ref().unwrap().reasoning_tokens.as_deref(),
                Some("0")
            );
            merge_usage(
                &mut usage,
                usage_from_any(Some(&json!({"output_tokens":12}))).unwrap(),
            );
            assert_eq!(
                usage.as_ref().unwrap().reasoning_tokens.as_deref(),
                Some("0")
            );
            assert_eq!(usage.unwrap().output_tokens.as_deref(), Some("12"));
        }
        assert_eq!(
            usage_from_any(Some(&json!({"output_tokens":12})))
                .unwrap()
                .reasoning_tokens,
            None
        );
        assert_eq!(
            usage_from_legacy(Some(&json!({"reasoningTokens":41})))
                .unwrap()
                .reasoning_tokens
                .as_deref(),
            Some("41")
        );
    }
}

fn numeric_string(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

fn parse_arguments(value: Option<&Value>) -> Result<Value, String> {
    match value {
        // Model-emitted argument text that is not usable JSON keeps the call
        // and becomes the invalid-arguments sentinel (same rule as readiness).
        Some(Value::String(value)) => Ok(parse_foreign_value(value.as_bytes(), "tool arguments")
            .unwrap_or_else(|error| invalid_arguments_sentinel(value, &error))),
        Some(value) => parse_foreign_value(
            &serde_json::to_vec(value).map_err(|error| error.to_string())?,
            "tool arguments",
        ),
        None => Err("tool arguments missing".to_owned()),
    }
}

fn parse_ijson(bytes: &[u8]) -> Result<Value, String> {
    let value = IJsonValue::parse(bytes).map_err(|error| error.to_string())?;
    serde_json::to_value(value).map_err(|error| error.to_string())
}

fn parse_foreign_value(bytes: &[u8], label: &str) -> Result<Value, String> {
    let value = parse_ijson(bytes)?;
    validate_foreign_value(&value, label)?;
    Ok(value)
}

fn validate_foreign_value(value: &Value, label: &str) -> Result<(), String> {
    if value
        .as_object()
        .is_some_and(|object| object.len() == 1 && object.contains_key("$spill"))
    {
        return Err(format!(
            "{label} collides with reserved $spill discriminant"
        ));
    }
    Ok(())
}

// Empty streaming fragments are valid; identity fields still use required_string.
fn delta_string(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("missing or non-string {key}"))
}

fn required_string(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("missing {key}"))
}

fn string_at(value: &Value, path: &[&str]) -> Option<String> {
    path.iter()
        .try_fold(value, |value, key| value.get(*key))
        .and_then(Value::as_str)
        .map(str::to_owned)
}

#[cfg(test)]
mod deepseek_cache_usage_tests {
    use super::*;

    #[test]
    fn top_level_cache_hits_are_retained_and_take_precedence() {
        for hit in [0, 128] {
            let usage = usage_from_any(Some(&serde_json::json!({
                "prompt_tokens":256,"completion_tokens":8,
                "prompt_cache_hit_tokens":hit,"prompt_cache_miss_tokens":256-hit,
                "prompt_tokens_details":{"cached_tokens":64}
            })))
            .unwrap();
            assert_eq!(usage.input_tokens.as_deref(), Some("256"));
            assert_eq!(usage.cache_read, Some(hit.to_string()));
            assert_eq!(usage.cache_miss, Some((256 - hit).to_string()));
        }
    }

    #[test]
    fn nested_cache_fallback_does_not_invent_missing_hits() {
        let nested = usage_from_any(Some(&serde_json::json!({
            "input_tokens":256,"input_tokens_details":{"cached_tokens":64}
        })))
        .unwrap();
        assert_eq!(nested.cache_read.as_deref(), Some("64"));
        assert_eq!(
            nested.cache_miss, None,
            "do not infer provider cache misses"
        );
        let absent = usage_from_any(Some(&serde_json::json!({"input_tokens":256}))).unwrap();
        assert_eq!(absent.cache_read, None);
    }
}
