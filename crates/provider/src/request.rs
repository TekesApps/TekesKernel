use std::collections::BTreeMap;
use std::str::FromStr;

use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{DialectId, ProviderTarget, validate_target};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterId {
    Responses,
    Anthropic,
    GoogleGeneration,
    GoogleInteractions,
    ChatCompletion,
}

impl AdapterId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Responses => "responses",
            Self::Anthropic => "anthropic",
            Self::GoogleGeneration => "google_generation",
            Self::GoogleInteractions => "google_interactions",
            Self::ChatCompletion => "chat_completion",
        }
    }

    #[must_use]
    pub const fn capabilities(self) -> ProviderCapabilities {
        match self {
            Self::Responses | Self::GoogleInteractions => ProviderCapabilities {
                server_managed: true,
                query_by_identity: false,
                dispatch_marker_required: true,
            },
            Self::Anthropic | Self::GoogleGeneration | Self::ChatCompletion => {
                ProviderCapabilities {
                    server_managed: false,
                    query_by_identity: false,
                    dispatch_marker_required: true,
                }
            }
        }
    }
}

impl FromStr for AdapterId {
    type Err = PrepareError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "responses" => Ok(Self::Responses),
            "anthropic" => Ok(Self::Anthropic),
            "google_generation" => Ok(Self::GoogleGeneration),
            "google_interactions" => Ok(Self::GoogleInteractions),
            "chat_completion" => Ok(Self::ChatCompletion),
            other => Err(PrepareError::UnknownAdapter(other.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub server_managed: bool,
    pub query_by_identity: bool,
    pub dispatch_marker_required: bool,
}

#[derive(Clone, Debug)]
pub struct PrepareInput {
    pub attempt_id: String,
    pub target: ProviderTarget,
    pub endpoint: String,
    pub epoch_profile: IJsonValue,
    pub continuation_id: Option<String>,
    pub rendered_items: Vec<IJsonValue>,
    pub tool_catalog: IJsonValue,
    pub stream: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedRequest {
    pub method: String,
    pub url: String,
    pub headers_without_secret: BTreeMap<String, String>,
    pub credential_header: String,
    pub credential_prefix: String,
    pub body: Vec<u8>,
    pub candidate_bytes: u64,
    pub dispatch_marker_required: bool,
    pub request_digest: String,
    pub query_key: Option<String>,
}

#[derive(Debug, Error)]
pub enum PrepareError {
    #[error("unknown provider adapter {0}")]
    UnknownAdapter(String),
    #[error("provider endpoint is invalid: {0}")]
    InvalidEndpoint(String),
    #[error("provider request is not I-JSON: {0}")]
    InvalidJson(String),
    #[error("provider target is unavailable: {0}")]
    InvalidTarget(String),
}

pub fn endpoint_origin(endpoint: &str) -> Result<String, PrepareError> {
    let url = reqwest::Url::parse(endpoint)
        .map_err(|error| PrepareError::InvalidEndpoint(error.to_string()))?;
    let host = url
        .host_str()
        .ok_or_else(|| PrepareError::InvalidEndpoint("endpoint has no host".to_owned()))?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| PrepareError::InvalidEndpoint("endpoint has no known port".to_owned()))?;
    Ok(format!("{}://{host}:{port}", url.scheme()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
}

pub fn prepare(input: &PrepareInput) -> Result<PreparedRequest, PrepareError> {
    prepare_with_tool_choice(input, None)
}

/// Explicit per-response policy, serialized before computing request identity.
pub fn prepare_with_tool_choice(
    input: &PrepareInput,
    choice: Option<ToolChoice>,
) -> Result<PreparedRequest, PrepareError> {
    prepare_inner(input, choice, None)
}

pub fn prepare_with_native_deferred_tools(
    input: &PrepareInput,
    choice: Option<ToolChoice>,
    native: &crate::NativeDeferredTools,
) -> Result<PreparedRequest, PrepareError> {
    prepare_inner(input, choice, Some(native))
}

fn prepare_inner(
    input: &PrepareInput,
    choice: Option<ToolChoice>,
    native: Option<&crate::NativeDeferredTools>,
) -> Result<PreparedRequest, PrepareError> {
    let configured_base = input.endpoint.trim_end_matches('/');
    if !(configured_base.starts_with("https://")
        || configured_base.starts_with("http://127.0.0.1:"))
    {
        return Err(PrepareError::InvalidEndpoint(input.endpoint.clone()));
    }
    let resolved = validate_target(&input.target)
        .map_err(|error| PrepareError::InvalidTarget(error.to_string()))?;
    let dialect = resolved.dialect;
    let adapter = dialect.family();
    let model = resolved.wire_model();
    let profile = value(&input.epoch_profile);
    if profile.get("target")
        != Some(
            &serde_json::to_value(&input.target)
                .map_err(|error| PrepareError::InvalidJson(error.to_string()))?,
        )
    {
        return Err(PrepareError::InvalidTarget(
            "epoch target does not match request target".to_owned(),
        ));
    }
    if profile.get("serializer_revision").and_then(Value::as_str)
        != Some(resolved.serializer_revision)
    {
        return Err(PrepareError::InvalidTarget(
            "epoch serializer revision does not match request target".to_owned(),
        ));
    }
    let system = profile
        .get("system")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut controls = profile
        .get("controls")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let items = values(&input.rendered_items);
    let tools = value(&input.tool_catalog);
    let reasoning_effort = controls.remove("reasoning_effort");
    let reasoning_disabled = match controls.remove("reasoning_disabled") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(_) => return Err(invalid("reasoning_disabled must be a boolean")),
    };
    // Title-only: the automatic thread-title request's output ceiling. Not a
    // session or product control; sessions never set an output cap.
    let title_output_cap = controls
        .remove("title_max_output_tokens")
        .map(|value| {
            value
                .as_u64()
                .filter(|cap| *cap > 0)
                .ok_or_else(|| invalid("title_max_output_tokens must be a positive integer"))
        })
        .transpose()?;
    apply_dialect_controls(
        &resolved,
        &mut controls,
        reasoning_effort,
        reasoning_disabled,
    )?;
    let anthropic_max_tokens = Value::from(anthropic_max_tokens(&resolved, input.stream));
    if input.continuation_id.is_some() && !dialect.server_managed() {
        return Err(PrepareError::InvalidJson(
            "stateless adapter cannot receive continuation_id".to_owned(),
        ));
    }
    let (suffix, mut body) = match adapter {
        AdapterId::Responses => (
            "/responses".to_owned(),
            json!({"input": render_responses(&items, dialect)?, "instructions": system, "model": model, "stream": input.stream, "tools": render_tools(dialect, &tools)?}),
        ),
        AdapterId::Anthropic => (
            "/messages".to_owned(),
            json!({"max_tokens": anthropic_max_tokens, "messages": render_anthropic(&items, dialect)?, "model": model, "stream": input.stream, "system": system, "tools": render_tools(dialect, &tools)?}),
        ),
        AdapterId::GoogleGeneration => {
            let suffix = if input.stream {
                format!("/models/{model}:streamGenerateContent?alt=sse")
            } else {
                format!("/models/{model}:generateContent")
            };
            (
                suffix,
                json!({"contents": render_google(&items, dialect)?, "systemInstruction":{"parts":[{"text":system}],"role":"user"}, "tools": render_tools(dialect, &tools)?}),
            )
        }
        AdapterId::GoogleInteractions => (
            "/interactions".to_owned(),
            json!({"input": render_interactions(&items, dialect, input.continuation_id.is_some())?, "model": model, "store": true, "stream": input.stream, "system_instruction": system, "tools": render_tools(dialect, &tools)?}),
        ),
        AdapterId::ChatCompletion => (
            "/chat/completions".to_owned(),
            json!({"messages": render_chat(&items, system, dialect)?, "model": model, "stream": input.stream, "tools": render_tools(dialect, &tools)?}),
        ),
    };
    if adapter == AdapterId::ChatCompletion {
        if let Some(ceiling) = resolved.max_output_tokens() {
            body.as_object_mut()
                .expect("chat body is an object")
                .insert("max_tokens".to_owned(), Value::from(ceiling));
        }
    }
    if dialect == DialectId::OllamaChatV1 && input.stream {
        body.as_object_mut()
            .expect("chat body is an object")
            .insert("stream_options".to_owned(), json!({"include_usage":true}));
    }
    if dialect == DialectId::OpenaiResponsesV1 {
        body.as_object_mut()
            .expect("Responses body is an object")
            .insert("store".to_owned(), Value::Bool(true));
    }
    if dialect == DialectId::AnthropicMessagesV1 {
        attach_anthropic_breakpoints(&mut body)?;
        let object = body.as_object_mut().expect("Messages body is an object");
        if resolved.refusal_fallback() {
            object.insert("fallbacks".to_owned(), Value::String("default".to_owned()));
        }
    }
    if let Some(continuation) = &input.continuation_id {
        let field = match adapter {
            AdapterId::Responses => "previous_response_id",
            AdapterId::GoogleInteractions => "previous_interaction_id",
            AdapterId::Anthropic | AdapterId::GoogleGeneration | AdapterId::ChatCompletion => {
                unreachable!("stateless continuation rejected above")
            }
        };
        body.as_object_mut()
            .expect("adapter request bodies are objects")
            .insert(field.to_owned(), Value::String(continuation.clone()));
    }
    merge_controls(&mut body, controls)?;
    if let Some(cap) = title_output_cap {
        apply_title_output_cap(dialect, &mut body, cap)?;
    }
    // Prefix edits (compaction, result trimming, system/tool changes) are
    // intentional Kernel operations. Let Anthropic drop only invalid thinking
    // for this request; never erase the durable native carrier on a model switch.
    let thinking_binding = dialect == DialectId::AnthropicMessagesV1
        && matches!(
            body.pointer("/thinking/type").and_then(Value::as_str),
            Some("adaptive" | "enabled")
        );
    if thinking_binding {
        body["thinking"]["block_binding"] = json!({"prefix_mismatch_behavior":"drop_block"});
    }
    if let Some(choice) = choice {
        if choice == ToolChoice::Required && tools.as_array().is_none_or(Vec::is_empty) {
            return Err(invalid("required tool choice needs a nonempty catalog"));
        }
        match adapter {
            AdapterId::Responses | AdapterId::ChatCompletion => {
                body["tool_choice"] = json!(match choice {
                    ToolChoice::Auto => "auto",
                    ToolChoice::Required => "required",
                    ToolChoice::None => "none",
                });
            }
            AdapterId::Anthropic => {
                // Generations without forced tool use (Fable 5.1) reject `any`;
                // the catalog keeps strict schemas and the prompt carries the
                // instruction, so the closest accepted policy is `auto`.
                let required = if resolved.forced_tool_choice() {
                    "any"
                } else {
                    "auto"
                };
                body["tool_choice"] = json!({"type":match choice {
                    ToolChoice::Auto => "auto", ToolChoice::Required => required, ToolChoice::None => "none",
                }});
            }
            AdapterId::GoogleGeneration => {
                body["toolConfig"] = json!({"functionCallingConfig":{"mode":match choice {
                    ToolChoice::Auto => "AUTO", ToolChoice::Required => "ANY", ToolChoice::None => "NONE",
                }}});
            }
            AdapterId::GoogleInteractions => {
                return Err(invalid(
                    "explicit tool choice is not proved for Google Interactions",
                ));
            }
        }
    }
    if let Some(native) = native {
        for item in &items {
            for block in item
                .get("content")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if block["type"] == "tool_result"
                    && block["error"] == true
                    && native
                        .references
                        .iter()
                        .any(|reference| block["call_id"] == reference.source_search_call_id)
                {
                    return Err(invalid(
                        "native reference cannot bind a failed search result",
                    ));
                }
            }
        }
        crate::native_deferred::apply(dialect, &tools, &mut body, native)?;
    }
    let body = encode_request_body(dialect, &body)?;
    let mut headers_without_secret = BTreeMap::from([
        (
            "accept".to_owned(),
            if input.stream {
                "text/event-stream"
            } else {
                "application/json"
            }
            .to_owned(),
        ),
        ("content-type".to_owned(), "application/json".to_owned()),
    ]);
    if adapter == AdapterId::Anthropic {
        headers_without_secret.insert("anthropic-version".to_owned(), "2023-06-01".to_owned());
        if dialect == DialectId::AnthropicMessagesV1 && resolved.refusal_fallback() {
            headers_without_secret.insert(
                "anthropic-beta".to_owned(),
                "server-side-fallback-2026-07-01".to_owned(),
            );
        }
    }
    let url = format!("{configured_base}{suffix}");
    if thinking_binding {
        let beta = headers_without_secret
            .entry("anthropic-beta".to_owned())
            .or_default();
        if !beta.is_empty() {
            beta.push(',');
        }
        beta.push_str("thinking-binding-controls-2026-08-01");
    }
    let request_digest = provider_request_digest_for_dialect(
        dialect.as_str(),
        "POST",
        &url,
        model,
        &headers_without_secret,
        &body,
    )?;
    let capabilities = adapter.capabilities();
    let query_key = capabilities
        .query_by_identity
        .then(|| provider_query_key(&input.attempt_id));
    Ok(PreparedRequest {
        method: "POST".to_owned(),
        url,
        headers_without_secret,
        credential_header: resolved.credential_header,
        credential_prefix: resolved.credential_prefix,
        candidate_bytes: u64::try_from(body.len()).unwrap_or(u64::MAX),
        body,
        dispatch_marker_required: capabilities.dispatch_marker_required,
        request_digest,
        query_key,
    })
}

fn encode_request_body(dialect: DialectId, body: &Value) -> Result<Vec<u8>, PrepareError> {
    if !matches!(
        dialect,
        DialectId::DeepseekChatV1 | DialectId::KimiChatV1 | DialectId::GlmChatV1
    ) {
        return serde_json_canonicalizer::to_vec(body)
            .map_err(|error| PrepareError::InvalidJson(error.to_string()));
    }
    let object = body
        .as_object()
        .ok_or_else(|| invalid("provider request body must be an object"))?;
    let order = [
        "model",
        "tools",
        "reasoning_effort",
        "thinking",
        "max_tokens",
        "temperature",
        "top_p",
        "tool_choice",
        "stream",
        "messages",
    ];
    if object.keys().any(|key| !order.contains(&key.as_str())) {
        return Err(invalid(format!(
            "cache-ordered {} request contains an unregistered field",
            dialect.as_str()
        )));
    }
    let mut bytes = vec![b'{'];
    let mut first = true;
    for key in order {
        let Some(value) = object.get(key) else {
            continue;
        };
        if !first {
            bytes.push(b',');
        }
        first = false;
        bytes.extend_from_slice(
            &serde_json::to_vec(key)
                .map_err(|error| PrepareError::InvalidJson(error.to_string()))?,
        );
        bytes.push(b':');
        bytes.extend_from_slice(
            &serde_json_canonicalizer::to_vec(value)
                .map_err(|error| PrepareError::InvalidJson(error.to_string()))?,
        );
    }
    bytes.push(b'}');
    Ok(bytes)
}

pub fn provider_request_digest(
    adapter: AdapterId,
    method: &str,
    final_url: &str,
    model: &str,
    headers: &BTreeMap<String, String>,
    body: &[u8],
) -> Result<String, PrepareError> {
    provider_request_digest_for_dialect(adapter.as_str(), method, final_url, model, headers, body)
}

pub fn provider_request_digest_for_dialect(
    dialect_id: &str,
    method: &str,
    final_url: &str,
    model: &str,
    headers: &BTreeMap<String, String>,
    body: &[u8],
) -> Result<String, PrepareError> {
    if headers
        .keys()
        .any(|name| name != &name.to_ascii_lowercase())
    {
        return Err(PrepareError::InvalidJson(
            "header names must be lowercase".to_owned(),
        ));
    }
    let header_bytes = serde_json_canonicalizer::to_vec(headers)
        .map_err(|error| PrepareError::InvalidJson(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-provider-request-v1\0");
    hasher.update(dialect_id.as_bytes());
    hasher.update([0]);
    hasher.update(method.to_ascii_uppercase().as_bytes());
    hasher.update([0]);
    hasher.update(final_url.as_bytes());
    hasher.update([0]);
    hasher.update(model.as_bytes());
    hasher.update([0]);
    hasher.update(header_bytes);
    hasher.update([0]);
    hasher.update(body);
    Ok(format!("{:x}", hasher.finalize()))
}

#[must_use]
pub fn provider_query_key(attempt_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-provider-query-v1\0");
    hasher.update(attempt_id.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn value(value: &IJsonValue) -> Value {
    serde_json::to_value(value).expect("IJsonValue always serializes")
}

fn values(values: &[IJsonValue]) -> Vec<Value> {
    values.iter().map(value).collect()
}

/// `max_tokens` for the Messages adapter: the catalog ceiling when declared,
/// otherwise room for a full answer (the cap truncates tool arguments
/// mid-JSON, so it is not a cost control). Streaming lifts the transport
/// timeout concern, so it gets the larger default. The DeepSeek Anthropic
/// facade keeps its documented 8192 ceiling.
fn anthropic_max_tokens(profile: &crate::ResolvedDialectProfile, stream: bool) -> u64 {
    if let Some(ceiling) = profile.max_output_tokens() {
        return ceiling;
    }
    match profile.dialect {
        DialectId::AnthropicMessagesV1 if stream => 64_000,
        DialectId::AnthropicMessagesV1 => 16_000,
        _ => 8192,
    }
}

fn apply_dialect_controls(
    profile: &crate::ResolvedDialectProfile,
    controls: &mut serde_json::Map<String, Value>,
    reasoning_effort: Option<Value>,
    reasoning_disabled: bool,
) -> Result<(), PrepareError> {
    let dialect = profile.dialect;
    // v1 exposes only reasoning_effort through the product settings contract.
    // Keep the epoch surface closed until another control has a typed
    // session/endpoint path and an end-to-end proof.
    if !controls.is_empty() {
        return unsupported_controls(dialect, controls);
    }
    if reasoning_disabled {
        return apply_reasoning_disabled(dialect, controls, reasoning_effort.is_some());
    }
    let effort = reasoning_effort
        .as_ref()
        .map(|value| {
            let value = value
                .as_str()
                .ok_or_else(|| invalid("reasoning_effort must be a string"))?;
            profile
                .reasoning_efforts()
                .iter()
                .any(|item| item == value)
                .then_some(value)
                .ok_or_else(|| {
                    invalid(format!(
                        "reasoning_effort {value} is unsupported by exact dialect {}",
                        dialect.as_str()
                    ))
                })
        })
        .transpose()?;
    match dialect {
        DialectId::OpenaiResponsesV1 | DialectId::DeepseekResponsesV1 if effort.is_some() => {
            controls.insert("reasoning".to_owned(), json!({"effort":effort}));
        }
        DialectId::DeepseekChatV1 if effort.is_some() => {
            controls.insert("reasoning_effort".to_owned(), json!(effort));
            controls.insert("thinking".to_owned(), json!({"type":"enabled"}));
        }
        DialectId::KimiChatV1 if effort.is_some() => {
            controls.insert("reasoning_effort".to_owned(), json!(effort));
        }
        DialectId::GlmChatV1 if effort.is_some() => {
            controls.insert("reasoning_effort".to_owned(), json!(effort));
            controls.insert(
                "thinking".to_owned(),
                json!({"clear_thinking":false,"type":"enabled"}),
            );
        }
        DialectId::AnthropicMessagesV1 | DialectId::DeepseekAnthropicV1 => {
            match profile.thinking_wire() {
                // Thinking is on by default on these generations; the explicit
                // shape only turns the summarized display on so the reasoning
                // rows have text. Effort rides in output_config, never in a
                // token budget (rejected with 400 on 4.7+).
                Some(crate::ThinkingWire::Adaptive) => {
                    controls.insert(
                        "thinking".to_owned(),
                        json!({"display":"summarized","type":"adaptive"}),
                    );
                    if let Some(effort) = effort {
                        controls.insert("output_config".to_owned(), json!({"effort":effort}));
                    }
                }
                Some(crate::ThinkingWire::Budget) if effort.is_some() => {
                    controls.insert(
                        "thinking".to_owned(),
                        json!({"budget_tokens":4096,"type":"enabled"}),
                    );
                }
                Some(crate::ThinkingWire::Budget) | None => {}
            }
        }
        DialectId::GoogleGenerationV1 if effort.is_some() => {
            // Only the three efforts with a budget mapping are renderable. A
            // catalog entry that declares another level passes control
            // validation and would otherwise reach an unreachable arm, so this
            // fails closed instead of panicking on a configuration mistake.
            let budget = match effort {
                Some("low") => 1_024,
                Some("medium") => 2_048,
                Some("high") => 4_096,
                other => {
                    return Err(invalid(format!(
                        "google_generation_v1 has no thinking budget for reasoning_effort {}",
                        other.unwrap_or("<none>")
                    )));
                }
            };
            controls.insert(
                "generationConfig".to_owned(),
                json!({"thinkingConfig":{"thinkingBudget":budget}}),
            );
        }
        DialectId::GoogleInteractionsV1 if effort.is_some() => {
            controls.insert(
                "generation_config".to_owned(),
                json!({"thinking_level":effort}),
            );
        }
        _ => {}
    }
    if profile.pro_reasoning() {
        controls.entry("reasoning").or_insert_with(|| json!({}))["mode"] = json!("pro");
    }
    Ok(())
}

/// Runtime-internal requests (automatic titles) must not reason, whatever the
/// model would do by default. Omitting the effort is not enough: deepseek-flash
/// thinks by default and, given a long coding prompt, reasoned for 25k tokens
/// and began implementing it. Only dialects with a verified off switch accept
/// the control; the rest fail closed so the caller sends nothing.
fn apply_reasoning_disabled(
    dialect: DialectId,
    controls: &mut serde_json::Map<String, Value>,
    has_effort: bool,
) -> Result<(), PrepareError> {
    if has_effort {
        return Err(invalid(
            "reasoning_disabled cannot be combined with reasoning_effort",
        ));
    }
    match dialect {
        DialectId::DeepseekResponsesV1 => {
            controls.insert("reasoning".to_owned(), json!({"effort":"none"}));
        }
        DialectId::DeepseekChatV1 => {
            controls.insert("thinking".to_owned(), json!({"type":"disabled"}));
        }
        _ => {
            return Err(invalid(format!(
                "reasoning cannot be disabled for exact dialect {}",
                dialect.as_str()
            )));
        }
    }
    Ok(())
}

/// Output ceiling for the automatic thread-title request only (control
/// `title_max_output_tokens`), so a model that ignores the title instruction
/// cannot generate thousands of tokens. Rendered as each dialect's wire field
/// (`max_output_tokens` on Responses, `max_tokens` on chat). Like
/// `reasoning_disabled`, only verified DeepSeek dialects accept it.
fn apply_title_output_cap(
    dialect: DialectId,
    body: &mut Value,
    cap: u64,
) -> Result<(), PrepareError> {
    let object = body
        .as_object_mut()
        .ok_or_else(|| invalid("provider request body must be an object"))?;
    match dialect {
        DialectId::DeepseekResponsesV1 => {
            object.insert("max_output_tokens".to_owned(), Value::from(cap));
        }
        DialectId::DeepseekChatV1 => {
            let ceiling = object
                .get("max_tokens")
                .and_then(Value::as_u64)
                .map_or(cap, |c| c.min(cap));
            object.insert("max_tokens".to_owned(), Value::from(ceiling));
        }
        _ => {
            return Err(invalid(format!(
                "title_max_output_tokens is not supported for exact dialect {}",
                dialect.as_str()
            )));
        }
    }
    Ok(())
}

fn unsupported_controls(
    dialect: DialectId,
    controls: &serde_json::Map<String, Value>,
) -> Result<(), PrepareError> {
    Err(invalid(format!(
        "unsupported controls for exact dialect {}: {}",
        dialect.as_str(),
        controls.keys().cloned().collect::<Vec<_>>().join(",")
    )))
}

/// Prompt-cache breakpoints for the Messages API. A root-level
/// `cache_control` is honoured, but its unit is the whole body: a session
/// changes the body every turn, so it wrote the full prefix on every attempt
/// and never read it back (measured on 2026-09-06: every request wrote the
/// whole prefix and none was a cache read). The server checks for
/// hits at the block boundaries before an explicit breakpoint, so one marker
/// on the last content block of the last message caches tools, system and
/// every earlier message, and the next request reads all of it back; a second
/// marker on the last tool keeps the tool catalog reusable by requests whose
/// messages differ. Both positions are deterministic, so the body stays a pure
/// function of the ledger. Only block types the API accepts a marker on are
/// eligible; a trailing thinking block is left alone.
fn attach_anthropic_breakpoints(body: &mut Value) -> Result<(), PrepareError> {
    let marker = json!({"type":"ephemeral"});
    if let Some(tool) = body
        .get_mut("tools")
        .and_then(Value::as_array_mut)
        .and_then(|tools| tools.last_mut())
    {
        tool.as_object_mut()
            .ok_or_else(|| invalid("tool declaration must be an object"))?
            .insert("cache_control".to_owned(), marker.clone());
    }
    let block = body
        .get_mut("messages")
        .and_then(Value::as_array_mut)
        .and_then(|messages| messages.last_mut())
        .and_then(|message| message.get_mut("content"))
        .and_then(Value::as_array_mut)
        .and_then(|content| content.last_mut());
    if let Some(block) = block {
        let eligible = matches!(
            block.get("type").and_then(Value::as_str),
            Some("text" | "image" | "document" | "tool_use" | "tool_result")
        );
        if eligible {
            block
                .as_object_mut()
                .ok_or_else(|| invalid("content block must be an object"))?
                .insert("cache_control".to_owned(), marker);
        }
    }
    Ok(())
}

fn merge_controls(
    body: &mut Value,
    controls: serde_json::Map<String, Value>,
) -> Result<(), PrepareError> {
    let object = body
        .as_object_mut()
        .ok_or_else(|| PrepareError::InvalidJson("request body is not an object".to_owned()))?;
    for (key, value) in controls {
        if object.contains_key(&key) {
            return Err(PrepareError::InvalidJson(format!(
                "epoch control overrides reserved request field {key}"
            )));
        }
        object.insert(key, value);
    }
    Ok(())
}

fn render_responses(items: &[Value], dialect: DialectId) -> Result<Vec<Value>, PrepareError> {
    let mut output = Vec::new();
    for item in items {
        if let Some(fragments) = sealed_fragments(item, dialect)? {
            output.extend(fragments);
            continue;
        }
        let role = item_role(item)?;
        let content = item_content(item)?;
        let mut blocks = Vec::new();
        for block in content {
            match block_type(block)? {
                "text" => blocks.push(json!({
                    "text": required_text(block, "text")?,
                    "type": if role == "assistant" { "output_text" } else { "input_text" }
                })),
                "tool_call" => output.push(json!({
                    "arguments": canonical_argument_string(block.get("arguments"))?,
                    "call_id": required_text(block, "call_id")?,
                    "name": required_text(block, "name")?,
                    "type": "function_call"
                })),
                "tool_result" => output.push(json!({
                    "call_id": required_text(block, "call_id")?,
                    "output": tool_result_text(block)?,
                    "type": "function_call_output"
                })),
                "reasoning" => blocks.push(json!({
                    "text": required_text(block, "text")?,
                    "type": if role == "assistant" { "output_text" } else { "input_text" }
                })),
                "file"
                    if required_text(block, "mime")?.starts_with("image/")
                        && dialect.input_blocks().contains(&"image") =>
                {
                    blocks.push(json!({
                        "image_url": data_url(block)?,
                        "type":"input_image"
                    }))
                }
                "file" if dialect.input_blocks().contains(&"file") => blocks.push(json!({
                    "file_data": data_url(block)?,
                    "filename": required_text(block, "name")
                        .unwrap_or_else(|_| "attachment".to_owned()),
                    "type":"input_file"
                })),
                "file" => blocks.push(json!({
                    "text": degraded_file_text(block, dialect)?,
                    "type": "input_text"
                })),
                other => return Err(invalid(format!("unsupported Responses block {other}"))),
            }
        }
        if !blocks.is_empty() {
            output.push(json!({"content": blocks, "role": role, "type": "message"}));
        }
    }
    Ok(output)
}

fn render_anthropic(items: &[Value], dialect: DialectId) -> Result<Vec<Value>, PrepareError> {
    let mut messages = Vec::new();
    for item in items {
        if let Some(fragments) = sealed_fragments(item, dialect)? {
            messages.push(json!({"content":fragments,"role":"assistant"}));
            continue;
        }
        let role = item_role(item)?;
        let mut blocks = Vec::new();
        for block in item_content(item)? {
            match block_type(block)? {
                "text" => {
                    blocks.push(json!({"text": required_text(block, "text")?, "type": "text"}))
                }
                "reasoning" if dialect.supports_reasoning_blocks() => blocks.push(json!({
                    "text":required_text(block, "text")?,"type":"text"
                })),
                "reasoning" => {
                    return Err(invalid(format!(
                        "{} rejects reasoning blocks",
                        dialect.as_str()
                    )));
                }
                "file" => {
                    let mime = required_text(block, "mime")?;
                    if mime.starts_with("image/") {
                        if !dialect.input_blocks().contains(&"image") {
                            return Err(invalid(format!(
                                "{} rejects the requested file block",
                                dialect.as_str()
                            )));
                        }
                        blocks.push(json!({
                            "source":{"data":required_text(block, "data")?,"media_type":mime,"type":"base64"},
                            "type":"image"
                        }));
                    } else if mime == "application/pdf" && dialect.input_blocks().contains(&"file")
                    {
                        blocks.push(json!({
                            "source":{"data":required_text(block, "data")?,"media_type":mime,"type":"base64"},
                            "type":"document"
                        }));
                    } else {
                        blocks.push(json!({
                            "text": degraded_file_text(block, dialect)?,
                            "type": "text"
                        }));
                    }
                }
                "tool_call" => blocks.push(json!({
                    "id": required_text(block, "call_id")?,
                    "input": required_value(block, "arguments")?,
                    "name": required_text(block, "name")?,
                    "type": "tool_use"
                })),
                "tool_result" => blocks.push(json!({
                    "content": tool_result_text(block)?,
                    "is_error": block.get("error").and_then(Value::as_bool).unwrap_or(false),
                    "tool_use_id": required_text(block, "call_id")?,
                    "type": "tool_result"
                })),
                other => return Err(invalid(format!("unsupported Anthropic block {other}"))),
            }
        }
        let role = if role == "tool" { "user" } else { role };
        messages.push(json!({"content": blocks, "role": role}));
    }
    // Host state and tool results can be separate canonical items. Anthropic
    // requires results in the immediate user message after tool_use, before
    // any text. Merge only adjacent user items, preserving result/text order
    // within their respective groups and every assistant boundary.
    let mut merged: Vec<Value> = Vec::new();
    for message in messages {
        if message["role"] == "user" && merged.last().is_some_and(|last| last["role"] == "user") {
            merged.last_mut().unwrap()["content"]
                .as_array_mut()
                .unwrap()
                .extend(message["content"].as_array().unwrap().iter().cloned());
        } else {
            merged.push(message);
        }
    }
    for message in &mut merged {
        if message["role"] == "user" {
            message["content"]
                .as_array_mut()
                .unwrap()
                .sort_by_key(|block| block["type"] != "tool_result");
        }
    }
    Ok(merged)
}

fn render_chat(
    items: &[Value],
    system: &str,
    dialect: DialectId,
) -> Result<Vec<Value>, PrepareError> {
    let mut messages = Vec::new();
    if !system.is_empty() {
        messages.push(json!({"content": system, "role": "system"}));
    }
    for item in items {
        if let Some(fragments) = sealed_fragment_value(item, dialect)? {
            messages.push(fragments);
            continue;
        }
        let role = item_role(item)?;
        let content = item_content(item)?;
        if content
            .iter()
            .all(|block| matches!(block_type(block), Ok("text" | "reasoning")))
        {
            let text = content
                .iter()
                .filter(|block| block_type(block).ok() == Some("text"))
                .map(|block| required_text(block, "text"))
                .collect::<Result<Vec<_>, _>>()?
                .join("");
            let reasoning = content
                .iter()
                .filter(|block| block_type(block).ok() == Some("reasoning"))
                .map(|block| required_text(block, "text"))
                .collect::<Result<Vec<_>, _>>()?
                .join("");
            let mut message = json!({"content": text, "role": role});
            if !reasoning.is_empty() {
                if !dialect.supports_reasoning_blocks() {
                    return Err(invalid(format!(
                        "{} rejects reasoning blocks",
                        dialect.as_str()
                    )));
                }
                if role != "assistant" {
                    return Err(invalid("chat reasoning content must have assistant role"));
                }
                message
                    .as_object_mut()
                    .expect("chat message object")
                    .insert("reasoning_content".to_owned(), Value::String(reasoning));
            }
            messages.push(message);
            continue;
        }
        for block in content {
            match block_type(block)? {
                "text" => {
                    messages.push(json!({"content": required_text(block, "text")?, "role": role}))
                }
                "reasoning" if dialect.supports_reasoning_blocks() => messages.push(json!({
                    "content":required_text(block, "text")?,"role":role
                })),
                "reasoning" => return Err(invalid(format!(
                    "{} rejects reasoning blocks",
                    dialect.as_str()
                ))),
                "file" if required_text(block, "mime")?.starts_with("image/")
                    && dialect.input_blocks().contains(&"image") => messages.push(json!({
                    "content":[{"image_url":{"url":data_url(block)?},"type":"image_url"}],"role":role
                })),
                "file" => messages.push(json!({
                    "content": degraded_file_text(block, dialect)?, "role": role
                })),
                "tool_call" => messages.push(json!({
                    "content": Value::Null,
                    "role": "assistant",
                    "tool_calls": [{
                        "function": {
                            "arguments": canonical_argument_string(block.get("arguments"))?,
                            "name": required_text(block, "name")?
                        },
                        "id": required_text(block, "call_id")?,
                        "type": "function"
                    }]
                })),
                "tool_result" => messages.push(json!({
                    "content": tool_result_text(block)?,
                    "role": "tool",
                    "tool_call_id": required_text(block, "call_id")?
                })),
                other => return Err(invalid(format!("unsupported Chat block {other}"))),
            }
        }
    }
    Ok(messages)
}

fn render_google(items: &[Value], dialect: DialectId) -> Result<Vec<Value>, PrepareError> {
    let mut contents = Vec::new();
    for item in items {
        if let Some(fragment) = sealed_fragment_value(item, dialect)? {
            contents.push(fragment);
            continue;
        }
        let role = item_role(item)?;
        let mut parts = Vec::new();
        for block in item_content(item)? {
            match block_type(block)? {
                "text" => {
                    parts.push(json!({"text": required_text(block, "text")?}));
                }
                "reasoning" if dialect.supports_reasoning_blocks() => {
                    parts.push(json!({"text":required_text(block, "text")?,"thought":true}))
                }
                "reasoning" => {
                    return Err(invalid(format!(
                        "{} rejects reasoning blocks",
                        dialect.as_str()
                    )));
                }
                "file" => {
                    let mime = required_text(block, "mime")?;
                    let supported = if mime.starts_with("image/") {
                        dialect.input_blocks().contains(&"image")
                    } else {
                        dialect.input_blocks().contains(&"file")
                    };
                    if !supported {
                        parts.push(json!({"text": degraded_file_text(block, dialect)?}));
                    } else if let Some(data) = block.get("data").and_then(Value::as_str) {
                        parts.push(json!({"inlineData":{"data":data,"mimeType":mime}}));
                    } else {
                        parts.push(json!({"file_data":{
                            "file_uri":required_text(block, "uri")?,"mime_type":mime
                        }}));
                    }
                }
                "tool_call" => parts.push(json!({"functionCall": {
                    "args": required_value(block, "arguments")?,
                    "id": required_text(block, "call_id")?,
                    "name": required_text(block, "name")?
                }})),
                "tool_result" => parts.push(json!({"functionResponse": {
                    "id": required_text(block, "call_id")?,
                    "name": required_text(block, "name")?,
                    "response": google_function_response(required_value(block, "result")?)
                }})),
                other => return Err(invalid(format!("unsupported Google block {other}"))),
            }
        }
        contents.push(
            json!({"parts": parts, "role": if role == "assistant" { "model" } else { "user" }}),
        );
    }
    Ok(contents)
}

fn google_function_response(result: Value) -> Value {
    if result.is_object() {
        result
    } else {
        json!({"result": result})
    }
}

/// `continued` says whether this request carries a `previous_interaction_id`.
/// The Interactions API accepts a text-only conversation replayed in full, but
/// rejects one that carries a completed tool round unless the server already
/// holds the conversation: a `function_call`/`function_result` pair in `input`
/// without a continuation answers `400 Request contains an invalid argument`
/// (measured 2026-09-07 against gemini-3.8-flash, with both real and synthetic
/// steps). At a recovery boundary, preserve completed tool work as quoted
/// history instead. Apply this to the rendered steps so sealed provider calls
/// and projected tool results take the same fallback path.
fn render_interactions(
    items: &[Value],
    dialect: DialectId,
    continued: bool,
) -> Result<Vec<Value>, PrepareError> {
    let mut steps = Vec::new();
    for item in items {
        if let Some(fragments) = sealed_fragments(item, dialect)? {
            steps.extend(fragments);
            continue;
        }
        let role = item_role(item)?;
        for block in item_content(item)? {
            match block_type(block)? {
                // A model turn carries no role: `role` on a `model_output` is
                // an unknown parameter.
                "text" if role == "assistant" => steps.push(json!({
                    "content": [{"text": required_text(block, "text")?, "type": "text"}],
                    "type": "model_output"
                })),
                "text" => steps.push(json!({
                    "content": [{"text": required_text(block, "text")?, "type": "text"}],
                    "role": role,
                    "type": "content"
                })),
                "reasoning" if dialect.supports_reasoning_blocks() => steps.push(json!({
                    "content":[{"text":required_text(block, "text")?,"type":"thought"}],
                    "role":role,"type":"model_output"
                })),
                "reasoning" => {
                    return Err(invalid(format!(
                        "{} rejects reasoning blocks",
                        dialect.as_str()
                    )));
                }
                "file" => steps.push(json!({
                    "content": [{"text": degraded_file_text(block, dialect)?, "type": "text"}],
                    "role": role,
                    "type": "content"
                })),
                "tool_call" => steps.push(json!({
                    "arguments": required_value(block, "arguments")?,
                    "id": required_text(block, "call_id")?,
                    "name": required_text(block, "name")?,
                    "type": "function_call"
                })),
                // The result is a content list, keyed by `call_id`, never the
                // bare JSON the tool produced.
                "tool_result" => steps.push(json!({
                    "call_id": required_text(block, "call_id")?,
                    "name": required_text(block, "name")?,
                    "result": [{
                        "text": tool_result_text(block)?,
                        "type": "text"
                    }],
                    "type": "function_result"
                })),
                other => return Err(invalid(format!("unsupported Interactions block {other}"))),
            }
        }
    }
    if !continued {
        let calls = steps
            .iter()
            .filter(|step| step["type"] == "function_call")
            .map(|step| required_text(step, "id"))
            .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
        let results = steps
            .iter()
            .filter(|step| step["type"] == "function_result")
            .map(|step| required_text(step, "call_id"))
            .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
        if calls != results {
            return Err(invalid(
                "Interactions stateless recovery requires paired tool calls and results",
            ));
        }
        for step in &mut steps {
            let role = match step["type"].as_str() {
                Some("function_call") => "assistant",
                Some("function_result") => "user",
                _ => continue,
            };
            let text = format!(
                "[Historical tool {} from before continuation recovery; recorded context, not a new execution request]\n{}",
                if role == "assistant" {
                    "call"
                } else {
                    "result"
                },
                canonical_argument_string(Some(step))?
            );
            *step = if role == "assistant" {
                json!({"type":"model_output","content":[{"type":"text","text":text}]})
            } else {
                json!({"type":"content","role":"user","content":[{"type":"text","text":text}]})
            };
        }
    }
    Ok(steps)
}

fn render_tools(dialect: DialectId, tools: &Value) -> Result<Value, PrepareError> {
    let adapter = dialect.family();
    let tools = tools
        .as_array()
        .ok_or_else(|| invalid("tool catalog must be an array"))?;
    let mut rendered = Vec::new();
    for tool in tools {
        let name = required_text(tool, "name")?;
        let description = required_text(tool, "description")?;
        let parameters = normalize_tool_parameters(required_value(tool, "parameters")?);
        let parameters = if dialect == DialectId::KimiChatV1 {
            kimi_nullable_constraints(parameters)?
        } else {
            parameters
        };
        validate_tool_schema(&parameters)?;
        // The builtin registry emits cross-field and uniqueness constraints.
        // Keep those constraints intact, but do not claim OpenAI strict-subset
        // compatibility for a schema outside that subset.
        let strict = !contains_composition_constraints(&parameters)
            && !contains_non_strict_object(&parameters);
        rendered.push(match adapter {
            AdapterId::Responses if dialect == DialectId::DeepseekResponsesV1 => json!({"description": description, "name": name, "parameters": parameters, "type": "function"}),
            AdapterId::Responses => json!({"description": description, "name": name, "parameters": parameters, "strict": strict, "type": "function"}),
            AdapterId::Anthropic
                if dialect == DialectId::AnthropicMessagesV1
                    && strict
                    && anthropic_strict_subset(&parameters) =>
            {
                json!({"description": description, "input_schema": anthropic_root_conditional(parameters), "name": name, "strict": true})
            }
            AdapterId::Anthropic => json!({"description": description, "input_schema": anthropic_root_conditional(parameters), "name": name}),
            AdapterId::GoogleGeneration => json!({"description": description, "name": name, "parametersJsonSchema": parameters}),
            AdapterId::GoogleInteractions => json!({"description": description, "name": name, "parameters": parameters, "type": "function"}),
            AdapterId::ChatCompletion if dialect != DialectId::OpenaiChatV1 => json!({"function": {"description": description, "name": name, "parameters": parameters}, "type": "function"}),
            AdapterId::ChatCompletion => json!({"function": {"description": description, "name": name, "parameters": parameters, "strict": strict}, "type": "function"}),
        });
    }
    Ok(
        if adapter == AdapterId::GoogleGeneration && !rendered.is_empty() {
            json!([{"functionDeclarations": rendered}])
        } else {
            Value::Array(rendered)
        },
    )
}

// MCP inputSchema permits an omitted empty `required` and schema-document
// annotations. Provider tool declarations require an explicit array and do
// not accept these top-level annotations. Keep all validation constraints.
fn normalize_tool_parameters(parameters: Value) -> Value {
    let Value::Object(mut schema) = parameters else {
        return parameters;
    };
    schema.remove("$schema");
    schema.remove("title");
    schema.entry("properties").or_insert_with(|| json!({}));
    schema.entry("required").or_insert_with(|| json!([]));
    Value::Object(schema)
}

/// Keywords Anthropic's strict custom-tool validation rejects anywhere in an
/// input schema, including inside a nested `anyOf`. Measured against the live
/// API: `minLength`, `maxLength`, `pattern`, `format`, `minItems`, `default`
/// and `enum` are accepted, these are not. A schema that uses one keeps every
/// constraint and is sent without the strict claim, because dropping a
/// constraint to buy strictness would weaken the contract the registry wrote.
const ANTHROPIC_STRICT_UNSUPPORTED: [&str; 9] = [
    "exclusiveMaximum",
    "exclusiveMinimum",
    "maxItems",
    "maxProperties",
    "maximum",
    "minProperties",
    "minimum",
    "multipleOf",
    "uniqueItems",
];

fn anthropic_strict_subset(schema: &Value) -> bool {
    match schema {
        Value::Array(items) => items.iter().all(anthropic_strict_subset),
        Value::Object(object) => {
            if object
                .keys()
                .any(|key| ANTHROPIC_STRICT_UNSUPPORTED.contains(&key.as_str()))
            {
                return false;
            }
            object.iter().all(|(key, value)| match key.as_str() {
                // Members of these maps are names authored by the tool, not
                // schema keywords: only their values are schemas.
                "properties" | "patternProperties" | "$defs" | "definitions" => value
                    .as_object()
                    .is_none_or(|members| members.values().all(anthropic_strict_subset)),
                _ => anthropic_strict_subset(value),
            })
        }
        _ => true,
    }
}

// A single conditional conjunct can be moved intact to its enclosing schema.
// Never flatten arbitrary branches: property closure and overlapping keywords
// can change their meaning. Other root combinators remain intact under an
// unconditional conditional, avoiding Anthropic's top-level union restriction.
fn anthropic_root_conditional(mut schema: Value) -> Value {
    let Some(root) = schema.as_object_mut() else {
        return schema;
    };
    let conditional = root
        .get("allOf")
        .and_then(Value::as_array)
        .filter(|branches| branches.len() == 1)
        .and_then(|branches| branches[0].as_object())
        .filter(|branch| {
            branch.contains_key("if")
                && branch.contains_key("then")
                && branch
                    .keys()
                    .all(|key| matches!(key.as_str(), "if" | "then" | "else"))
                && ["if", "then", "else"]
                    .iter()
                    .all(|key| !root.contains_key(*key))
        })
        .cloned();
    if let Some(conditional) = conditional {
        root.remove("allOf");
        root.extend(conditional);
    } else if ["allOf", "oneOf", "anyOf"]
        .iter()
        .any(|key| root.contains_key(*key))
    {
        let mut constraints = serde_json::Map::new();
        for key in ["allOf", "oneOf", "anyOf", "if", "then", "else"] {
            if let Some(value) = root.remove(key) {
                constraints.insert(key.to_owned(), value);
            }
        }
        root.insert("if".to_owned(), json!({}));
        root.insert("then".to_owned(), Value::Object(constraints));
    }
    schema
}

fn kimi_nullable_constraints(mut value: Value) -> Result<Value, PrepareError> {
    match &mut value {
        Value::Array(items) => {
            for item in items {
                *item = kimi_nullable_constraints(item.take())?;
            }
        }
        Value::Object(object) => {
            for child in object.values_mut() {
                *child = kimi_nullable_constraints(child.take())?;
            }
            let concrete = object
                .get("type")
                .and_then(Value::as_array)
                .filter(|types| types.len() == 2 && types.iter().any(|t| t == "null"))
                .and_then(|types| types.iter().find(|t| *t != "null"))
                .and_then(Value::as_str)
                .map(str::to_owned);
            let keys: &[&str] = match concrete.as_deref() {
                Some("integer" | "number") => &[
                    "minimum",
                    "maximum",
                    "exclusiveMinimum",
                    "exclusiveMaximum",
                    "multipleOf",
                ],
                Some("string") => &["minLength", "maxLength", "pattern", "format"],
                Some("array") => &[
                    "items",
                    "prefixItems",
                    "contains",
                    "minContains",
                    "maxContains",
                    "minItems",
                    "maxItems",
                    "uniqueItems",
                ],
                Some("object") => &[
                    "properties",
                    "patternProperties",
                    "additionalProperties",
                    "required",
                    "dependentRequired",
                    "minProperties",
                    "maxProperties",
                ],
                _ => &[],
            };
            if keys.iter().any(|key| object.contains_key(*key)) {
                if object.contains_key("anyOf") {
                    return Err(invalid(
                        "Kimi nullable constraints conflict with existing anyOf",
                    ));
                }
                let mut branch = serde_json::Map::new();
                branch.insert("type".into(), json!(concrete.unwrap()));
                for key in keys {
                    if let Some(v) = object.remove(*key) {
                        branch.insert((*key).into(), v);
                    }
                }
                object.remove("type");
                object.insert("anyOf".into(), json!([branch,{"type":"null"}]));
            }
        }
        _ => {}
    }
    Ok(value)
}

#[cfg(test)]
mod anthropic_conditional_tests {
    use super::*;

    #[test]
    fn kimi_nullable_bounds_preserve_null_enum_and_reject_composition_collision() {
        let input = json!({"type":"object","properties":{"offset":{"type":["integer","null"],"minimum":1,"enum":[null,1,2]}}});
        let lowered = kimi_nullable_constraints(input).unwrap();
        assert_eq!(
            lowered["properties"]["offset"],
            json!({"enum":[null,1,2],"anyOf":[{"type":"integer","minimum":1},{"type":"null"}]})
        );
        assert!(
            kimi_nullable_constraints(json!({"type":["integer","null"],"minimum":1,"anyOf":[{}]}))
                .is_err()
        );
        let unconstrained = json!({"type":["string","null"],"description":"keep"});
        assert_eq!(
            kimi_nullable_constraints(unconstrained.clone()).unwrap(),
            unconstrained
        );
    }

    #[test]
    fn only_isolated_single_conditional_is_hoisted_without_losing_constraints() {
        let rule = json!({"if":{"properties":{"verdict":{"const":"fail"}},"required":["verdict"]},"then":{"properties":{"failures":{"minItems":1}}},"else":{"properties":{"failures":{"maxItems":0}}}});
        let schema = json!({"type":"object","additionalProperties":false,"properties":{"verdict":{"type":"string"},"failures":{"type":"array"}},"required":["verdict","failures"],"allOf":[rule]});
        let lowered = anthropic_root_conditional(schema.clone());
        assert!(lowered.get("allOf").is_none());
        for key in ["if", "then", "else"] {
            assert_eq!(lowered[key], rule[key]);
        }
        for key in ["type", "additionalProperties", "properties", "required"] {
            assert_eq!(lowered[key], schema[key]);
        }
        for unsafe_schema in [
            json!({"allOf":[rule.clone(),rule.clone()]}),
            json!({"if":{},"allOf":[rule.clone()]}),
            json!({"allOf":[{"type":"object","additionalProperties":false}]}),
            json!({"allOf":[{"if":{},"then":{},"else":{},"properties":{}}]}),
        ] {
            let wrapped = anthropic_root_conditional(unsafe_schema.clone());
            assert_eq!(wrapped, json!({"if":{},"then":unsafe_schema}));
        }
    }
}

fn contains_composition_constraints(value: &Value) -> bool {
    match value {
        Value::Object(object) => object.iter().any(|(key, value)| {
            matches!(
                key.as_str(),
                "allOf" | "oneOf" | "not" | "if" | "then" | "else" | "uniqueItems"
            ) || contains_composition_constraints(value)
        }),
        Value::Array(values) => values.iter().any(contains_composition_constraints),
        _ => false,
    }
}

// Preserve optional fields rather than pretending the source schema makes them
// mandatory. Strict function schemas require every object property to be required.
fn contains_non_strict_object(value: &Value) -> bool {
    match value {
        Value::Object(object) => {
            let is_object = object.get("type").and_then(Value::as_str) == Some("object")
                || object.contains_key("properties");
            if is_object {
                let properties = object.get("properties").and_then(Value::as_object);
                let required = object.get("required").and_then(Value::as_array);
                if object.get("additionalProperties") != Some(&Value::Bool(false))
                    || properties
                        .zip(required)
                        .is_none_or(|(properties, required)| {
                            properties.len() != required.len()
                                || properties.keys().any(|key| {
                                    !required
                                        .iter()
                                        .any(|value| value.as_str() == Some(key.as_str()))
                                })
                        })
                {
                    return true;
                }
            }
            object.values().any(contains_non_strict_object)
        }
        Value::Array(values) => values.iter().any(contains_non_strict_object),
        _ => false,
    }
}

#[cfg(test)]
mod strict_optional_tests {
    use super::*;

    #[test]
    fn mcp_schema_metadata_and_omitted_empty_required_render_without_losing_constraints() {
        let schema = json!({
            "$schema":"https://json-schema.org/draft/2020-12/schema",
            "title":"get_capabilitiesArguments", "type":"object",
            "properties":{"limit":{"type":"integer","minimum":1}},
            "additionalProperties":false
        });
        let rendered = render_tools(
            DialectId::OpenaiResponsesV1,
            &json!([{"name":"mcp__fixture__get_capabilities","description":"Inspect",
                "parameters":schema}]),
        )
        .expect("render MCP tool");
        let parameters = &rendered[0]["parameters"];
        assert_eq!(parameters["required"], json!([]));
        assert_eq!(parameters["properties"]["limit"]["minimum"], 1);
        assert!(parameters.get("$schema").is_none());
        assert!(parameters.get("title").is_none());
    }

    #[test]
    fn unique_items_in_nullable_array_preserves_constraints_without_strict_mode() {
        let parameters = json!({"type":"object","properties":{"options":{"anyOf":[
            {"type":"array","items":{"type":"string"},"uniqueItems":true},{"type":"null"}]}},
            "required":["options"],"additionalProperties":false});
        for dialect in [DialectId::OpenaiResponsesV1, DialectId::OpenaiChatV1] {
            let rendered = render_tools(
                dialect,
                &json!([{"name":"ask_user_questions","description":"ask","parameters":parameters}]),
            )
            .unwrap();
            let tool = rendered[0].get("function").unwrap_or(&rendered[0]);
            assert_eq!(tool["strict"], false);
            assert_eq!(tool["parameters"], parameters);
        }
    }

    #[test]
    fn optional_fields_keep_their_schema_and_disable_strict_mode() {
        let optional = json!({"type":"object","properties":{"record_ids":{"type":"array","items":{"type":"integer"}},
            "cursor":{"type":"string"}},"required":["record_ids"],"additionalProperties":false});
        assert!(contains_non_strict_object(&optional));
        for dialect in [DialectId::OpenaiResponsesV1, DialectId::OpenaiChatV1] {
            let rendered = render_tools(
                dialect,
                &json!([{"name":"context_get","description":"read", "parameters":optional}]),
            )
            .unwrap();
            let tool = rendered[0].get("function").unwrap_or(&rendered[0]);
            assert_eq!(tool["strict"], false);
            assert_eq!(
                tool["parameters"], optional,
                "optional fields must not be rewritten as mandatory"
            );
        }
        let mut required = optional.clone();
        required["required"] = json!(["record_ids", "cursor"]);
        assert!(!contains_non_strict_object(&required));
        let nested = json!({"type":"object","properties":{"query":optional},"required":["query"],"additionalProperties":false});
        assert!(contains_non_strict_object(&nested));
    }
}

fn validate_tool_schema(schema: &Value) -> Result<(), PrepareError> {
    let object = schema
        .as_object()
        .ok_or_else(|| invalid("tool parameters must be an object schema"))?;
    if object.get("type").and_then(Value::as_str) != Some("object") {
        return Err(invalid("tool parameters must have type object"));
    }
    let properties = object
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid("tool object schema lacks properties"))?;
    let required = object
        .get("required")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("tool object schema lacks required"))?;
    if required.iter().any(|name| {
        name.as_str()
            .is_none_or(|name| name.is_empty() || !properties.contains_key(name))
    }) {
        return Err(invalid(
            "tool required fields must name declared properties",
        ));
    }
    if object.keys().any(|key| {
        !matches!(
            key.as_str(),
            "type" | "properties" | "required" | "additionalProperties" | "description" | "allOf"
        )
    }) {
        return Err(invalid("tool object schema contains an unproved keyword"));
    }
    if let Some(rules) = object.get("allOf") {
        if !rules
            .as_array()
            .is_some_and(|rules| !rules.is_empty() && rules.iter().all(Value::is_object))
        {
            return Err(invalid(
                "tool allOf must be a nonempty array of object schemas",
            ));
        }
    }
    Ok(())
}

fn item_role(item: &Value) -> Result<&str, PrepareError> {
    item.get("role")
        .and_then(Value::as_str)
        .filter(|role| matches!(*role, "user" | "assistant" | "tool"))
        .ok_or_else(|| invalid("render item has invalid role"))
}

fn sealed_fragments(item: &Value, dialect: DialectId) -> Result<Option<Vec<Value>>, PrepareError> {
    let Some(value) = sealed_fragment_value(item, dialect)? else {
        return Ok(None);
    };
    value
        .as_array()
        .cloned()
        .map(Some)
        .ok_or_else(|| invalid("sealed fragments must be an array for this adapter"))
}

fn sealed_fragment_value(item: &Value, dialect: DialectId) -> Result<Option<Value>, PrepareError> {
    if item.get("role").and_then(Value::as_str) != Some("sealed") {
        return Ok(None);
    }
    if item.get("adapter").and_then(Value::as_str) != Some(dialect.as_str()) {
        return Err(invalid("sealed item adapter mismatch"));
    }
    item.get("fragments")
        .cloned()
        .map(Some)
        .ok_or_else(|| invalid("sealed item lacks fragments"))
}

/// Largest decoded byte length whose UTF-8 content is delivered inline when
/// a dialect cannot receive the file natively (spec §File attachments).
const DEGRADED_FILE_TEXT_LIMIT: usize = 262_144;

/// Deterministic text degradation of a non-image `file` block for a dialect
/// that does not accept its media type. Valid UTF-8 content of at most
/// [`DEGRADED_FILE_TEXT_LIMIT`] bytes is quoted in a fenced block; anything
/// else becomes a fixed stub. An `image/*` file is never degraded here: a
/// dialect without image input still rejects it.
fn degraded_file_text(block: &Value, dialect: DialectId) -> Result<String, PrepareError> {
    use base64::Engine as _;
    let mime = required_text(block, "mime")?;
    if mime.starts_with("image/") {
        return Err(invalid(format!(
            "{} rejects the requested file block",
            dialect.as_str()
        )));
    }
    let name = required_text(block, "name").unwrap_or_else(|_| "attachment".to_owned());
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(required_text(block, "data")?)
        .map_err(|error| invalid(format!("file block data is not base64: {error}")))?;
    let bytes = block
        .get("bytes")
        .and_then(Value::as_u64)
        .unwrap_or(decoded.len() as u64);
    if decoded.len() <= DEGRADED_FILE_TEXT_LIMIT {
        if let Ok(content) = std::str::from_utf8(&decoded) {
            return Ok(format!(
                "Attached file {name} ({mime}, {bytes} bytes):\n```\n{content}\n```"
            ));
        }
    }
    Ok(format!(
        "Attached file {name} ({mime}, {bytes} bytes) could not be delivered to this model."
    ))
}

fn data_url(block: &Value) -> Result<String, PrepareError> {
    Ok(format!(
        "data:{};base64,{}",
        required_text(block, "mime")?,
        required_text(block, "data")?
    ))
}

fn item_content(item: &Value) -> Result<&[Value], PrepareError> {
    item.get("content")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| invalid("render item content must be an array"))
}

fn block_type(block: &Value) -> Result<&str, PrepareError> {
    block
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("render block lacks type"))
}

fn required_text(value: &Value, field: &str) -> Result<String, PrepareError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| invalid(format!("missing {field}")))
}

fn required_value(value: &Value, field: &str) -> Result<Value, PrepareError> {
    value
        .get(field)
        .cloned()
        .ok_or_else(|| invalid(format!("missing {field}")))
}

/// The string a text-carrying dialect sends for a tool result. A result that
/// is exactly one text block is sent as that text; anything else (several
/// blocks, files, a bare outcome object) is sent as canonical JSON.
fn tool_result_text(block: &Value) -> Result<String, PrepareError> {
    let result = block.get("result");
    if let Some([only]) = result.and_then(Value::as_array).map(Vec::as_slice) {
        if only.get("type").and_then(Value::as_str) == Some("text") {
            if let Some(text) = only.get("text").and_then(Value::as_str) {
                return Ok(text.to_owned());
            }
        }
    }
    canonical_argument_string(result)
}

fn canonical_argument_string(value: Option<&Value>) -> Result<String, PrepareError> {
    let value = value.ok_or_else(|| invalid("missing argument value"))?;
    serde_json_canonicalizer::to_string(value)
        .map_err(|error| PrepareError::InvalidJson(error.to_string()))
}

fn invalid(message: impl Into<String>) -> PrepareError {
    PrepareError::InvalidJson(message.into())
}

#[cfg(test)]
mod file_degradation_tests {
    use super::*;
    use base64::Engine as _;

    fn user(block: Value) -> Vec<Value> {
        vec![json!({"role":"user","content":[block]})]
    }

    fn file(mime: &str, name: &str, bytes: &[u8]) -> Value {
        json!({
            "type":"file","mime":mime,"name":name,"bytes":bytes.len(),
            "data":base64::engine::general_purpose::STANDARD.encode(bytes)
        })
    }

    #[test]
    fn pdf_on_anthropic_is_still_a_document() {
        let messages = render_anthropic(
            &user(file("application/pdf", "x.pdf", b"%PDF")),
            DialectId::AnthropicMessagesV1,
        )
        .unwrap();
        assert_eq!(messages[0]["content"][0]["type"], "document");
        assert_eq!(
            messages[0]["content"][0]["source"]["media_type"],
            "application/pdf"
        );
    }

    #[test]
    fn text_file_on_anthropic_degrades_to_fenced_text() {
        let messages = render_anthropic(
            &user(file("text/plain", "notes.txt", b"line one\nline two")),
            DialectId::AnthropicMessagesV1,
        )
        .unwrap();
        assert_eq!(
            messages[0]["content"][0],
            json!({"text":"Attached file notes.txt (text/plain, 17 bytes):\n```\nline one\nline two\n```","type":"text"})
        );
    }

    #[test]
    fn binary_file_on_chat_completions_degrades_to_the_stub() {
        let messages = render_chat(
            &user(file(
                "application/octet-stream",
                "blob.bin",
                &[0xff, 0xfe, 0x00],
            )),
            "",
            DialectId::OpenaiChatV1,
        )
        .unwrap();
        assert_eq!(
            messages[0],
            json!({"content":"Attached file blob.bin (application/octet-stream, 3 bytes) could not be delivered to this model.","role":"user"})
        );
    }

    #[test]
    fn oversized_utf8_file_degrades_to_the_stub() {
        let big = vec![b'a'; DEGRADED_FILE_TEXT_LIMIT + 1];
        let messages = render_chat(
            &user(file("text/plain", "big.txt", &big)),
            "",
            DialectId::DeepseekChatV1,
        )
        .unwrap();
        assert_eq!(
            messages[0]["content"],
            "Attached file big.txt (text/plain, 262145 bytes) could not be delivered to this model."
        );
        let exact = vec![b'a'; DEGRADED_FILE_TEXT_LIMIT];
        let messages = render_chat(
            &user(file("text/plain", "max.txt", &exact)),
            "",
            DialectId::DeepseekChatV1,
        )
        .unwrap();
        assert!(
            messages[0]["content"]
                .as_str()
                .unwrap()
                .starts_with("Attached file max.txt (text/plain, 262144 bytes):\n```\n")
        );
    }

    #[test]
    fn text_file_on_responses_is_delivered_as_input_file() {
        let output = render_responses(
            &user(file("text/plain", "notes.txt", b"hi")),
            DialectId::OpenaiResponsesV1,
        )
        .unwrap();
        assert_eq!(output[0]["content"][0]["type"], "input_file");
        assert_eq!(output[0]["content"][0]["filename"], "notes.txt");
        assert_eq!(
            output[0]["content"][0]["file_data"],
            "data:text/plain;base64,aGk="
        );
        // A Responses dialect without file input degrades instead of failing.
        let output = render_responses(
            &user(file("text/plain", "notes.txt", b"hi")),
            DialectId::DeepseekResponsesV1,
        )
        .unwrap();
        assert_eq!(
            output[0]["content"][0],
            json!({"text":"Attached file notes.txt (text/plain, 2 bytes):\n```\nhi\n```","type":"input_text"})
        );
    }

    #[test]
    fn google_and_interactions_degrade_in_their_native_text_forms() {
        let contents = render_google(
            &user(file("text/csv", "t.csv", b"a,b")),
            DialectId::GoogleGenerationV1,
        )
        .unwrap();
        assert_eq!(
            contents[0]["parts"][0]["inlineData"]["mimeType"],
            "text/csv"
        );
        let steps = render_interactions(
            &user(file("text/csv", "t.csv", b"a,b")),
            DialectId::GoogleInteractionsV1,
            true,
        )
        .unwrap();
        assert_eq!(
            steps[0],
            json!({"content":[{"text":"Attached file t.csv (text/csv, 3 bytes):\n```\na,b\n```","type":"text"}],"role":"user","type":"content"})
        );
    }

    #[test]
    fn image_files_are_never_degraded() {
        let error = render_chat(
            &user(file("image/png", "p.png", b"\x89PNG")),
            "",
            DialectId::DeepseekChatV1,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("rejects the requested file block"),
            "{error}"
        );
    }
}

#[cfg(test)]
mod anthropic_tool_adjacency_tests {
    use super::*;

    /// Anthropic rejects a strict custom tool whose schema carries a numeric,
    /// array-size or object-size bound. The tool must still be offered, with
    /// its constraints intact and no strict claim.
    #[test]
    fn a_bounded_schema_is_offered_to_anthropic_without_the_strict_claim() {
        let bounded = json!({"type":"object","additionalProperties":false,
            "properties":{"n":{"type":"integer","minimum":0}},"required":["n"]});
        let plain = json!({"type":"object","additionalProperties":false,
            "properties":{"s":{"type":"string","minLength":1,"pattern":"^a+$"}},"required":["s"]});
        assert!(!anthropic_strict_subset(&bounded));
        assert!(anthropic_strict_subset(&plain));
        // Nested inside a union, and inside an array's items, it still counts.
        assert!(!anthropic_strict_subset(
            &json!({"properties":{"n":{"anyOf":[{"type":"integer","minimum":1},{"type":"null"}]}}})
        ));
        assert!(!anthropic_strict_subset(
            &json!({"properties":{"a":{"type":"array","items":{"type":"string"},"uniqueItems":true}}})
        ));
        // A property literally named `minimum` is a name, not a keyword.
        assert!(anthropic_strict_subset(
            &json!({"type":"object","properties":{"minimum":{"type":"string"}}})
        ));

        let rendered = render_tools(
            DialectId::AnthropicMessagesV1,
            &json!([
                {"name":"apply_patch","description":"d","parameters":bounded},
                {"name":"think","description":"d","parameters":plain}
            ]),
        )
        .expect("anthropic tools");
        assert_eq!(
            rendered[0]["input_schema"]["properties"]["n"]["minimum"],
            json!(0)
        );
        assert!(
            rendered[0].get("strict").is_none(),
            "a bounded schema never claims strict"
        );
        assert_eq!(rendered[1]["strict"], json!(true));
    }

    #[test]
    fn host_state_between_call_and_result_does_not_break_anthropic_adjacency() {
        let messages = render_anthropic(&[
            json!({"role":"assistant","content":[{"type":"tool_call","call_id":"task-1","name":"task","arguments":{}}]}),
            json!({"role":"user","content":[{"type":"text","text":"delegation state"}]}),
            json!({"role":"tool","content":[{"type":"tool_result","call_id":"task-1","result":{"outcome":"completed"}}]}),
            json!({"role":"assistant","content":[{"type":"text","text":"done"}]}),
            json!({"role":"user","content":[{"type":"text","text":"next turn"}]}),
        ], DialectId::AnthropicMessagesV1).unwrap();
        assert_eq!(messages.len(), 4);
        assert_eq!(messages[1]["content"][0]["type"], "tool_result");
        assert_eq!(messages[1]["content"][0]["tool_use_id"], "task-1");
        assert_eq!(messages[1]["content"][1]["text"], "delegation state");
        assert_eq!(messages[2]["role"], "assistant");
        assert_eq!(messages[3]["content"][0]["text"], "next turn");
    }
}
