use std::collections::BTreeSet;

use serde_json::{Map, Value};
use uuid::Uuid;

use crate::{IJsonValue, OriginTuple, SchemaError, SeqRange};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const SPILL_THRESHOLD: usize = 16_384;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EventKind {
    Genesis,
    RunStart,
    Epoch,
    Input,
    TurnOpen,
    QueueEdit,
    Output,
    Reasoning,
    ToolCall,
    EffectiveExecution,
    ToolExecutionStarted,
    ToolResult,
    ApprovalRequest,
    ApprovalResponse,
    Spawn,
    ChildResult,
    Attempt,
    AttemptDispatched,
    AttemptRecovery,
    Error,
    Settle,
    Checkpoint,
    Compact,
    StopRequested,
    Meta,
    State,
    Extension(String),
}

impl EventKind {
    #[must_use]
    pub fn parse(value: &str) -> Self {
        match value {
            "genesis" => Self::Genesis,
            "run_start" => Self::RunStart,
            "epoch" => Self::Epoch,
            "input" => Self::Input,
            "turn_open" => Self::TurnOpen,
            "queue_edit" => Self::QueueEdit,
            "output" => Self::Output,
            "reasoning" => Self::Reasoning,
            "tool_call" => Self::ToolCall,
            "effective_execution" => Self::EffectiveExecution,
            "tool_execution_started" => Self::ToolExecutionStarted,
            "tool_result" => Self::ToolResult,
            "approval_request" => Self::ApprovalRequest,
            "approval_response" => Self::ApprovalResponse,
            "spawn" => Self::Spawn,
            "child_result" => Self::ChildResult,
            "attempt" => Self::Attempt,
            "attempt_dispatched" => Self::AttemptDispatched,
            "attempt_recovery" => Self::AttemptRecovery,
            "error" => Self::Error,
            "settle" => Self::Settle,
            "checkpoint" => Self::Checkpoint,
            "compact" => Self::Compact,
            "stop_requested" => Self::StopRequested,
            "meta" => Self::Meta,
            "state" => Self::State,
            other => Self::Extension(other.to_owned()),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Genesis => "genesis",
            Self::RunStart => "run_start",
            Self::Epoch => "epoch",
            Self::Input => "input",
            Self::TurnOpen => "turn_open",
            Self::QueueEdit => "queue_edit",
            Self::Output => "output",
            Self::Reasoning => "reasoning",
            Self::ToolCall => "tool_call",
            Self::EffectiveExecution => "effective_execution",
            Self::ToolExecutionStarted => "tool_execution_started",
            Self::ToolResult => "tool_result",
            Self::ApprovalRequest => "approval_request",
            Self::ApprovalResponse => "approval_response",
            Self::Spawn => "spawn",
            Self::ChildResult => "child_result",
            Self::Attempt => "attempt",
            Self::AttemptDispatched => "attempt_dispatched",
            Self::AttemptRecovery => "attempt_recovery",
            Self::Error => "error",
            Self::Settle => "settle",
            Self::Checkpoint => "checkpoint",
            Self::Compact => "compact",
            Self::StopRequested => "stop_requested",
            Self::Meta => "meta",
            Self::State => "state",
            Self::Extension(value) => value,
        }
    }

    #[must_use]
    pub const fn is_turn_bound(&self) -> Option<bool> {
        match self {
            Self::TurnOpen
            | Self::Output
            | Self::Reasoning
            | Self::ToolCall
            | Self::ToolResult
            | Self::ChildResult
            | Self::State
            | Self::ApprovalRequest
            | Self::ApprovalResponse
            | Self::EffectiveExecution
            | Self::ToolExecutionStarted
            | Self::Spawn
            | Self::Attempt
            | Self::AttemptDispatched
            | Self::AttemptRecovery
            | Self::Settle
            | Self::Error => Some(true),
            Self::Genesis
            | Self::RunStart
            | Self::Epoch
            | Self::Input
            | Self::Checkpoint
            | Self::Compact
            | Self::Meta
            | Self::StopRequested
            | Self::QueueEdit => Some(false),
            Self::Extension(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    Model,
    Runtime,
    Never,
    DialectDependent,
}

#[derive(Clone, Debug)]
pub struct Event {
    raw: IJsonValue,
    seq: u64,
    kind: EventKind,
    turn: Option<u64>,
    min_reader: Option<u64>,
    visibility: Option<Visibility>,
    origin_key: Option<String>,
}

impl Event {
    pub fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        let raw = IJsonValue::parse(bytes)?;
        Self::from_value(raw)
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, SchemaError> {
        let event = Self::decode(bytes)?;
        if event.canonical_bytes()? != bytes {
            return Err(SchemaError::Canonical(
                "event bytes are not RFC 8785 canonical".to_owned(),
            ));
        }
        Ok(event)
    }

    pub fn from_value(raw: IJsonValue) -> Result<Self, SchemaError> {
        let object = raw
            .value()
            .as_object()
            .ok_or_else(|| SchemaError::event(None, "event must be a JSON object"))?;
        let seq = required_u64(object, "seq", None)?;
        let version = required_u64(object, "v", Some(seq))?;
        if version != 1 {
            return Err(SchemaError::event(Some(seq), "v must equal 1"));
        }
        let kind_text = required_str(object, "kind", Some(seq))?;
        if kind_text.is_empty() {
            return Err(SchemaError::event(Some(seq), "kind must not be empty"));
        }
        let kind = EventKind::parse(kind_text);
        validate_timestamp(required_str(object, "ts", Some(seq))?, seq)?;
        let turn = optional_u64(object, "turn", seq)?;
        match kind.is_turn_bound() {
            Some(true) if turn.is_none() => {
                return Err(SchemaError::event(
                    Some(seq),
                    "turn-bound kind requires turn",
                ));
            }
            Some(false) if turn.is_some() => {
                return Err(SchemaError::event(
                    Some(seq),
                    "file-scoped kind forbids turn",
                ));
            }
            _ => {}
        }
        let min_reader = optional_u64(object, "min_reader", seq)?;
        let visibility = optional_visibility(object, seq)?;
        optional_bool(object, "anchor", seq)?;
        validate_ranges_optional(object, "supersedes", seq)?;
        let origin_key = optional_str(object, "origin_key", seq)?.map(ToOwned::to_owned);
        validate_payload(&kind, object, seq)?;
        validate_origin_equality(object, origin_key.as_deref(), seq)?;
        Ok(Self {
            raw,
            seq,
            kind,
            turn,
            min_reader,
            visibility,
            origin_key,
        })
    }

    #[must_use]
    pub const fn seq(&self) -> u64 {
        self.seq
    }

    #[must_use]
    pub const fn turn(&self) -> Option<u64> {
        self.turn
    }

    #[must_use]
    pub const fn kind(&self) -> &EventKind {
        &self.kind
    }

    #[must_use]
    pub const fn min_reader(&self) -> Option<u64> {
        self.min_reader
    }

    #[must_use]
    pub fn origin_key(&self) -> Option<&str> {
        self.origin_key.as_deref()
    }

    #[must_use]
    pub fn effective_visibility(&self) -> Visibility {
        if let Some(override_value) = self.visibility {
            return match (self.default_visibility(), override_value) {
                (Visibility::Never, _) => Visibility::Never,
                (_, value) => value,
            };
        }
        self.default_visibility()
    }

    #[must_use]
    pub const fn default_visibility(&self) -> Visibility {
        match self.kind {
            EventKind::Input
            | EventKind::Output
            | EventKind::ToolCall
            | EventKind::ToolResult
            | EventKind::ChildResult
            | EventKind::State => Visibility::Model,
            EventKind::Reasoning => Visibility::DialectDependent,
            EventKind::ApprovalRequest
            | EventKind::ApprovalResponse
            | EventKind::EffectiveExecution
            | EventKind::ToolExecutionStarted
            | EventKind::Error
            | EventKind::QueueEdit
            | EventKind::Extension(_) => Visibility::Runtime,
            _ => Visibility::Never,
        }
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, SchemaError> {
        self.raw.canonical_bytes()
    }

    #[must_use]
    pub const fn raw(&self) -> &IJsonValue {
        &self.raw
    }

    #[must_use]
    pub fn string_field(&self, field: &str) -> Option<&str> {
        self.object().get(field).and_then(Value::as_str)
    }

    #[must_use]
    pub fn integer_field(&self, field: &str) -> Option<u64> {
        self.object().get(field).and_then(Value::as_u64)
    }

    #[must_use]
    pub fn has_field(&self, field: &str) -> bool {
        self.object().contains_key(field)
    }

    /// The genesis `ephemeral` flag; false when absent or on non-genesis events.
    #[must_use]
    pub fn is_ephemeral_genesis(&self) -> bool {
        matches!(self.kind(), EventKind::Genesis)
            && self.object().get("ephemeral").and_then(Value::as_bool) == Some(true)
    }

    /// The `usage` object carried by an `output`/`error` that settles an
    /// attempt (`{availability, figures...}`); absent on every other event.
    pub fn usage(&self) -> Option<&Map<String, Value>> {
        self.object().get("usage").and_then(Value::as_object)
    }

    pub fn origin_tuple(&self) -> Result<Option<OriginTuple>, SchemaError> {
        self.object()
            .get("origin_tuple")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(SchemaError::Json)
    }

    /// Returns the validated envelope retraction ranges. Rebuildable
    /// projections use this accessor instead of interpreting raw append order.
    pub fn supersedes(&self) -> Result<Vec<SeqRange>, SchemaError> {
        if !self.object().contains_key("supersedes") {
            return Ok(Vec::new());
        }
        ranges(self.object(), "supersedes", self.seq).map(|values| {
            values
                .into_iter()
                .map(|(from, to)| SeqRange { from, to })
                .collect()
        })
    }

    pub(crate) fn object(&self) -> &Map<String, Value> {
        self.raw
            .value()
            .as_object()
            .expect("validated event is an object")
    }
}

fn validate_payload(
    kind: &EventKind,
    object: &Map<String, Value>,
    seq: u64,
) -> Result<(), SchemaError> {
    match kind {
        EventKind::Genesis => validate_genesis(object, seq),
        EventKind::RunStart => {
            required_id(object, "run", seq)?;
            required_enum(object, "mode", &["ordinary", "reconcile"], seq)?;
            required_u64(object, "recovery_ordinal", Some(seq))?;
            required_str(object, "binary", Some(seq))?;
            required_str(object, "config_digest", Some(seq))?;
            required_str(object, "instruction_digest", Some(seq))?;
            if let Some(digest) = optional_str(object, "launch_bindings_digest", seq)? {
                validate_lower_sha256(digest, seq, "run_start.launch_bindings_digest")?;
            }
            required_str(object, "policy", Some(seq))?;
            Ok(())
        }
        EventKind::Epoch => validate_epoch(object, seq),
        EventKind::Input => validate_input(object, seq),
        EventKind::TurnOpen => validate_turn_open(object, seq),
        EventKind::QueueEdit => {
            required_object(object, "origin_tuple", seq)
                .and_then(|v| validate_origin_tuple(v, seq))?;
            let ranges = validate_ranges_required_from_envelope(object, "supersedes", seq)?;
            if ranges.is_empty() {
                return Err(SchemaError::event(
                    Some(seq),
                    "queue_edit supersedes must not be empty",
                ));
            }
            Ok(())
        }
        EventKind::Output => validate_output(object, seq),
        EventKind::Reasoning => {
            required_id(object, "attempt", seq)?;
            validate_string_or_spill(required(object, "content", seq)?, seq)
        }
        EventKind::ToolCall => validate_tool_call(object, seq),
        EventKind::ToolExecutionStarted => required_id(object, "call", seq).map(|_| ()),
        EventKind::EffectiveExecution => {
            required_id(object, "call", seq)?;
            validate_json_or_spill(required(object, "invocation", seq)?, seq)
        }
        EventKind::ToolResult => validate_tool_result(object, seq),
        EventKind::ApprovalRequest => {
            required_id(object, "call", seq)?;
            required_str(object, "scope", Some(seq))?;
            optional_json_value(object, "question")?;
            Ok(())
        }
        EventKind::ApprovalResponse => {
            required_id(object, "call", seq)?;
            required_bool(object, "grant", seq)?;
            optional_str(object, "scope", seq)?;
            optional_json_value(object, "answer")?;
            validate_origin_tuple(required_object(object, "origin_tuple", seq)?, seq)
        }
        EventKind::Spawn => validate_spawn(object, seq),
        EventKind::ChildResult => validate_child_result(object, seq),
        EventKind::Attempt => validate_attempt(object, seq),
        EventKind::AttemptDispatched => required_id(object, "attempt", seq).map(|_| ()),
        EventKind::AttemptRecovery => validate_attempt_recovery(object, seq),
        EventKind::Error => validate_error(object, seq),
        EventKind::Settle => validate_settle(object, seq),
        EventKind::Checkpoint => validate_checkpoint(object, seq),
        EventKind::Compact => validate_compact(object, seq),
        EventKind::StopRequested => {
            required_u64(object, "generation", Some(seq))?;
            validate_origin_tuple(required_object(object, "origin_tuple", seq)?, seq)
        }
        EventKind::Meta => validate_meta(object, seq),
        EventKind::State => {
            required_str(object, "subkind", Some(seq))?;
            required(object, "payload", seq)?;
            Ok(())
        }
        EventKind::Extension(_) => Ok(()),
    }
}

fn validate_lower_sha256(value: &str, seq: u64, label: &str) -> Result<(), SchemaError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(SchemaError::event(
            Some(seq),
            format!("{label} must be 64 lowercase hexadecimal digits"),
        ))
    }
}

fn validate_genesis(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    if seq != 1 {
        return Err(SchemaError::event(Some(seq), "genesis must be seq 1"));
    }
    required_u64(object, "format", Some(seq))?;
    required_u64(object, "min_reader", Some(seq))?;
    required_u64(object, "min_writer", Some(seq))?;
    let thread = required_str(object, "thread", Some(seq))?;
    let parsed = Uuid::parse_str(thread)
        .map_err(|_| SchemaError::event(Some(seq), "thread must be a canonical UUID"))?;
    if parsed.hyphenated().to_string() != thread || thread != thread.to_ascii_lowercase() {
        return Err(SchemaError::event(
            Some(seq),
            "thread must be a lower-case canonical UUID",
        ));
    }
    required_str(object, "workspace", Some(seq))?;
    if object.contains_key("folder_binding") {
        required_id(object, "folder_binding", seq)?;
    }
    if let Some(identity) = object.get("identity_profile") {
        if !matches!(identity.as_str(), Some("auto" | "coding" | "general")) {
            return Err(SchemaError::event(Some(seq), "invalid identity_profile"));
        }
    }
    // An ephemeral session is a scratch ledger: it is swept at startup and
    // discarded when its client closes it. Readers that predate the flag see
    // an ordinary session, so the ledger format is not gated on it.
    optional_bool(object, "ephemeral", seq)?;
    required_str(object, "origin_key", Some(seq))?;
    validate_origin_tuple(required_object(object, "origin_tuple", seq)?, seq)?;
    validate_resume(required(object, "resume", seq)?, seq)?;
    if let Some(parent) = optional_object(object, "parent", seq)? {
        required_str(parent, "file", Some(seq))?;
        required_u64(parent, "seq", Some(seq))?;
        required_id(parent, "spawn_id", seq)?;
    }
    if let Some(seed) = optional_object(object, "seed", seq)? {
        required_str(seed, "source", Some(seq))?;
        let kinds = required_array(seed, "kinds", seq)?;
        let mut previous: Option<&str> = None;
        let mut seen = BTreeSet::new();
        for value in kinds {
            let kind = value
                .as_str()
                .ok_or_else(|| SchemaError::event(Some(seq), "seed.kinds must be strings"))?;
            if previous.is_some_and(|old| old >= kind) || !seen.insert(kind) {
                return Err(SchemaError::event(
                    Some(seq),
                    "seed.kinds must be sorted and unique",
                ));
            }
            previous = Some(kind);
        }
        let snapshot = required_object(seed, "snapshot", seq)?;
        required_str(snapshot, "asset", Some(seq))?;
        required_str(snapshot, "digest", Some(seq))?;
    }
    let config = required_object(object, "config", seq)?;
    required_str(config, "digest", Some(seq))?;
    if let Some(instruction) = optional_object(object, "instruction", seq)? {
        required_str(instruction, "digest", Some(seq))?;
    }
    Ok(())
}

fn validate_epoch(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_id(object, "id", seq)?;
    required_str(object, "reason", Some(seq))?;
    required_str(object, "adapter", Some(seq))?;
    required_str(object, "model", Some(seq))?;
    let system = required_object(object, "system", seq)?;
    required_str(system, "asset", Some(seq))?;
    required_str(system, "digest", Some(seq))?;
    // The declared tool catalog, stored like the system envelope: a digest
    // alone cannot rebuild a request, and an MCP catalog is not derivable
    // from config plus code because it came from a live server.
    let tools = required_object(object, "tools", seq)?;
    required_str(tools, "asset", Some(seq))?;
    required_str(tools, "digest", Some(seq))?;
    required_u64(object, "renderer", Some(seq))?;
    if let Some(pending) = optional_object(object, "pending", seq)? {
        validate_u64_array(
            required_array(pending, "eligible", seq)?,
            seq,
            "pending.eligible",
        )?;
        validate_u64_array(
            required_array(pending, "withheld", seq)?,
            seq,
            "pending.withheld",
        )?;
    }
    Ok(())
}

fn validate_input(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    let content = required_array(object, "content", seq)?;
    if content.is_empty() {
        return Err(SchemaError::event(
            Some(seq),
            "input.content must not be empty",
        ));
    }
    validate_blocks(content, seq)?;
    optional_bool(object, "steer", seq)?;
    validate_origin_tuple(required_object(object, "origin_tuple", seq)?, seq)?;
    if let Some(source) = object.get("source") {
        match source {
            Value::String(value) if matches!(value.as_str(), "user" | "cross_thread") => {}
            Value::Object(value) => {
                required_str(value, "thread", Some(seq))?;
            }
            _ => return Err(SchemaError::event(Some(seq), "invalid input.source")),
        }
    }
    Ok(())
}

fn validate_turn_open(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    match required(object, "trigger", seq)? {
        Value::String(value) if value == "genesis" => Ok(()),
        Value::Object(value) => {
            if value.contains_key("goal") {
                if value.len() != 1 {
                    return Err(SchemaError::event(
                        Some(seq),
                        "goal turn trigger has extra fields",
                    ));
                }
                required_id(value, "goal", seq)?;
                return Ok(());
            }
            let inputs = required_array(value, "inputs", seq)?;
            if inputs.is_empty() {
                return Err(SchemaError::event(
                    Some(seq),
                    "turn_open inputs must not be empty",
                ));
            }
            validate_strictly_increasing_u64(inputs, seq, "turn_open inputs")
        }
        _ => Err(SchemaError::event(Some(seq), "invalid turn_open.trigger")),
    }
}

fn validate_output(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_id(object, "attempt", seq)?;
    validate_usage(required_object(object, "usage", seq)?, seq)?;
    optional_bool(object, "final_answer", seq)?;
    validate_blocks(required_array(object, "content", seq)?, seq)?;
    let sealed = required_object(object, "sealed", seq)?;
    required_u64(sealed, "version", Some(seq))?;
    required_str(sealed, "adapter", Some(seq))?;
    validate_string_or_spill(required(sealed, "fragments", seq)?, seq)?;
    if let Some(continuation) = optional_object(object, "continuation", seq)? {
        required_id(continuation, "id", seq)?;
    }
    Ok(())
}

fn validate_tool_call(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    optional_bool(object, "execution_tracked", seq)?;
    if object.contains_key("provider_call") {
        required_id(object, "provider_call", seq)?;
    }
    required_id(object, "call", seq)?;
    required_str(object, "name", Some(seq))?;
    validate_json_or_spill(required(object, "args", seq)?, seq)?;
    required_enum(object, "source", &["provider"], seq)?;
    required_id(object, "attempt", seq)?;
    Ok(())
}

fn validate_tool_result(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_id(object, "call", seq)?;
    let outcome = required(object, "outcome", seq)?;
    match outcome {
        Value::String(value) if matches!(value.as_str(), "ok" | "error") => {}
        Value::Object(value) if value.len() == 1 && value.contains_key("aborted") => {
            required_enum(value, "aborted", &["crash", "recovered"], seq)?;
        }
        Value::Object(value) if value.len() == 1 && value.contains_key("denied") => {
            required_str(value, "denied", Some(seq))?;
        }
        Value::Object(value) if value.len() == 1 && value.contains_key("withheld") => {
            required_enum(value, "withheld", &["scan_failed", "quarantined"], seq)?;
        }
        _ => return Err(SchemaError::event(Some(seq), "invalid tool_result.outcome")),
    }
    if let Some(content) = object.get("content") {
        validate_blocks_or_spill(content, seq)?;
    }
    optional_json_value(object, "meta")?;
    Ok(())
}

fn validate_spawn(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_str(object, "child", Some(seq))?;
    required_id(object, "call", seq)?;
    required_id(object, "spawn_id", seq)?;
    validate_resume(required(object, "resume", seq)?, seq)?;
    let seed = required_object(object, "seed", seq)?;
    for kind in required_array(seed, "kinds", seq)? {
        if kind.as_str().is_none() {
            return Err(SchemaError::event(
                Some(seq),
                "spawn.seed.kinds must be strings",
            ));
        }
    }
    Ok(())
}

fn validate_child_result(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_str(object, "child", Some(seq))?;
    required_id(object, "call", seq)?;
    required_id(object, "spawn_id", seq)?;
    required_enum(
        object,
        "outcome",
        &["completed", "interrupted", "error", "failed"],
        seq,
    )?;
    optional_str(object, "summary", seq)?;
    if let Some(artifacts) = optional_array(object, "artifacts", seq)? {
        for artifact in artifacts {
            let artifact = artifact.as_object().ok_or_else(|| {
                SchemaError::event(Some(seq), "child_result artifact must be object")
            })?;
            required_str(artifact, "asset", Some(seq))?;
            required_str(artifact, "mime", Some(seq))?;
        }
    }
    Ok(())
}

fn validate_attempt(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_id(object, "attempt", seq)?;
    required_id(object, "epoch", seq)?;
    required_str(object, "wire_digest", Some(seq))?;
    // The exact transmitted request body as a thread asset. Every attempt
    // carries it: an attempt without its request body is not replayable
    // evidence of what was sent.
    let request = required_object(object, "request", seq)?;
    if request.len() != 2 {
        return Err(SchemaError::event(
            Some(seq),
            "attempt request carries exactly asset and bytes",
        ));
    }
    required_str(request, "asset", Some(seq))?;
    if required_u64(request, "bytes", Some(seq))? == 0 {
        return Err(SchemaError::event(
            Some(seq),
            "attempt request bytes must be positive",
        ));
    }
    validate_ranges_required_from_payload(object, "admits", seq)?;
    Ok(())
}

fn validate_attempt_recovery(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_id(object, "attempt", seq)?;
    let decision = required_enum(
        object,
        "decision",
        &[
            "adopt",
            "recovery_epoch",
            "resend",
            "not_dispatched",
            "unresolved",
        ],
        seq,
    )?;
    if decision == "adopt" {
        let inventory = required_array(object, "inventory", seq)?;
        for call in inventory {
            if call.as_str().is_none() {
                return Err(SchemaError::event(
                    Some(seq),
                    "adopt inventory must be strings",
                ));
            }
        }
        let response = required_object(object, "response", seq)?;
        required_str(response, "asset", Some(seq))?;
    } else if object.contains_key("inventory") || object.contains_key("response") {
        return Err(SchemaError::event(
            Some(seq),
            "only adopt carries inventory/response",
        ));
    }
    Ok(())
}

/// The `usage` object carried by the `output`/`error` that settles an
/// attempt: `{availability, figures...}` with figures as decimal strings.
fn validate_usage(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    let availability = required_enum(object, "availability", &["reported", "unavailable"], seq)?;
    let figures = [
        "input_tokens",
        "output_tokens",
        "cache_read",
        "cache_miss",
        "cache_write",
        "reasoning_tokens",
        "cost",
    ];
    if availability == "reported" {
        for field in figures {
            optional_str(object, field, seq)?;
        }
    } else if figures.into_iter().any(|field| object.contains_key(field)) {
        return Err(SchemaError::event(
            Some(seq),
            "unavailable usage cannot carry figures",
        ));
    }
    Ok(())
}

fn validate_error(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_bool(object, "recoverable", seq)?;
    required_enum(
        object,
        "classification",
        &[
            "provider_terminal",
            "unresolved_dispatch",
            "transport",
            "rate_limit",
            "tool",
            "internal",
        ],
        seq,
    )?;
    optional_str(object, "attempt", seq)?;
    if object.contains_key("attempt") {
        validate_usage(required_object(object, "usage", seq)?, seq)?;
    } else if object.contains_key("usage") {
        return Err(SchemaError::event(
            Some(seq),
            "usage belongs to the error that settles an attempt",
        ));
    }
    optional_str(object, "detail", seq)?;
    // A recoverable transport error may seal the native items the response
    // had completed before the loss, so a retry replays an eagerly
    // dispatched call behind its native predecessors (spec/event.md).
    if let Some(sealed) = optional_object(object, "sealed", seq)? {
        if !object.contains_key("attempt")
            || object.get("recoverable") != Some(&Value::Bool(true))
            || object.get("classification").and_then(Value::as_str) != Some("transport")
        {
            return Err(SchemaError::event(
                Some(seq),
                "sealed partial carrier belongs to a recoverable transport error that settles an attempt",
            ));
        }
        required_u64(sealed, "version", Some(seq))?;
        required_str(sealed, "adapter", Some(seq))?;
        validate_string_or_spill(required(sealed, "fragments", seq)?, seq)?;
    }
    Ok(())
}

fn validate_settle(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    if let Some(output) = optional_u64(object, "promoted_output_seq", seq)? {
        if object.get("outcome").and_then(Value::as_str) != Some("completed")
            || object.contains_key("validation")
            || output == 0
            || output >= seq
        {
            return Err(SchemaError::event(
                Some(seq),
                "direct final settlement has invalid output reference",
            ));
        }
    }
    if let Some(validation) = optional_object(object, "validation", seq)? {
        if object.get("outcome").and_then(Value::as_str) != Some("completed") {
            return Err(SchemaError::event(
                Some(seq),
                "validation settlement must be completed",
            ));
        }
        required_enum(
            validation,
            "outcome",
            &["pass", "inconclusive", "not_required"],
            seq,
        )?;
        let output = required_u64(validation, "promoted_output_seq", Some(seq))?;
        let candidate = required_u64(validation, "candidate_seq", Some(seq))?;
        let decision = required_u64(validation, "decision_seq", Some(seq))?;
        if !(0 < output && output < candidate && candidate < decision && decision < seq) {
            return Err(SchemaError::event(
                Some(seq),
                "validation references must precede settlement in causal order",
            ));
        }
    }
    match required_enum(
        object,
        "outcome",
        &["completed", "interrupted", "error"],
        seq,
    )? {
        "completed" => {
            if object.contains_key("reason") || object.contains_key("classification") {
                return Err(SchemaError::event(
                    Some(seq),
                    "completed settle has no reason/classification",
                ));
            }
        }
        "interrupted" => {
            required_enum(
                object,
                "reason",
                &["user_stop", "budget_tokens", "budget_wall", "recovered"],
                seq,
            )?;
            if object.contains_key("classification") {
                return Err(SchemaError::event(
                    Some(seq),
                    "interrupted settle has no classification",
                ));
            }
        }
        "error" => {
            required_enum(
                object,
                "classification",
                &["provider_terminal", "transport", "rate_limit", "internal"],
                seq,
            )?;
            if object.contains_key("reason") {
                return Err(SchemaError::event(Some(seq), "error settle has no reason"));
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn validate_checkpoint(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    required_u64(object, "covers", Some(seq))?;
    validate_string_or_spill(required(object, "summary", seq)?, seq)?;
    Ok(())
}

fn validate_compact(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    validate_ranges_required_from_payload(object, "covers", seq)?;
    validate_string_or_spill(required(object, "summary", seq)?, seq)?;
    if let Some(origin) = optional_object(object, "origin_tuple", seq)? {
        validate_origin_tuple(origin, seq)?;
    }
    if let Some(request) = optional_object(object, "summary_request", seq)? {
        // The summary request's telemetry: the frozen bundle and the admission
        // verdict are the contract; the artifact fields are open.
        let bundle = required_object(request, "bundle", seq)?;
        validate_ranges_required_from_payload(bundle, "covers", seq)?;
        if required_str(bundle, "sha256", Some(seq))?.is_empty() {
            return Err(SchemaError::event(
                Some(seq),
                "compact.summary_request.bundle.sha256 must not be empty",
            ));
        }
        required_bool(request, "accepted", seq)?;
    }
    Ok(())
}

fn validate_meta(object: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    optional_str(object, "title", seq)?;
    if let Some(labels) = optional_array(object, "labels", seq)? {
        for label in labels {
            if label.as_str().is_none() {
                return Err(SchemaError::event(Some(seq), "meta.labels must be strings"));
            }
        }
    }
    optional_u64(object, "ownership", seq)?;
    if let Some(upgrade) = optional_object(object, "upgrade", seq)? {
        required_u64(upgrade, "min_reader", Some(seq))?;
        required_u64(upgrade, "min_writer", Some(seq))?;
    }
    if let Some(origin) = optional_object(object, "origin_tuple", seq)? {
        validate_origin_tuple(origin, seq)?;
        if object.contains_key("ownership") || object.contains_key("upgrade") {
            return Err(SchemaError::event(
                Some(seq),
                "client meta cannot set ownership/upgrade",
            ));
        }
    }
    if let Some(notice) = optional_object(object, "notice", seq)? {
        validate_session_notice(notice, seq)?;
    }
    if !object.keys().any(|key| {
        matches!(
            key.as_str(),
            "title" | "labels" | "ownership" | "upgrade" | "notice"
        )
    }) {
        return Err(SchemaError::event(Some(seq), "meta must carry a mutation"));
    }
    Ok(())
}

/// `meta.notice`: a host-authored, turn-independent runtime fact the
/// transcript shows (a launch that ran degraded, an input that will not run).
/// `severity` is the closed two-level grade — `warning` when the session goes
/// on, `error` when the input it follows is stranded — so the UI never infers
/// the grade from the message.
fn validate_session_notice(notice: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    let severity = required_str(notice, "severity", Some(seq))?;
    if !matches!(severity, "warning" | "error") {
        return Err(SchemaError::event(
            Some(seq),
            "meta.notice.severity must be warning or error",
        ));
    }
    for field in ["classification", "operation", "message"] {
        if required_str(notice, field, Some(seq))?.is_empty() {
            return Err(SchemaError::event(
                Some(seq),
                format!("meta.notice.{field} must be nonempty"),
            ));
        }
    }
    Ok(())
}

fn validate_blocks(values: &[Value], seq: u64) -> Result<(), SchemaError> {
    for value in values {
        let object = value
            .as_object()
            .ok_or_else(|| SchemaError::event(Some(seq), "Block must be an object"))?;
        match required_enum(
            object,
            "type",
            &[
                "text",
                "reasoning",
                "image",
                "file",
                "tool-call",
                "tool-result",
            ],
            seq,
        )? {
            "text" | "reasoning" => {
                required_str(object, "text", Some(seq))?;
            }
            "image" => {
                required_str(object, "asset", Some(seq))?;
                required_str(object, "mime", Some(seq))?;
                if let Some(name) = optional_str(object, "name", seq)? {
                    if name.is_empty() || name.len() > 255 || name.chars().any(char::is_control) {
                        return Err(SchemaError::event(
                            Some(seq),
                            "image name must be 1..255 UTF-8 bytes without control scalars",
                        ));
                    }
                }
            }
            "file" => {
                required_str(object, "asset", Some(seq))?;
                required_str(object, "mime", Some(seq))?;
                let name = required_str(object, "name", Some(seq))?;
                if name.is_empty()
                    || name.len() > 255
                    || name.chars().any(char::is_control)
                    || name.contains('/')
                {
                    return Err(SchemaError::event(
                        Some(seq),
                        "file name must be 1..255 UTF-8 bytes without control scalars or '/'",
                    ));
                }
                if required_u64(object, "bytes", Some(seq))? == 0 {
                    return Err(SchemaError::event(Some(seq), "file bytes must be >= 1"));
                }
            }
            "tool-call" => {
                required_id(object, "call", seq)?;
                required_str(object, "name", Some(seq))?;
                required(object, "args", seq)?;
            }
            "tool-result" => {
                required_id(object, "call", seq)?;
                validate_blocks(required_array(object, "content", seq)?, seq)?;
                optional_bool(object, "error", seq)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn validate_blocks_or_spill(value: &Value, seq: u64) -> Result<(), SchemaError> {
    if is_spill(value) {
        validate_spill(value, seq)
    } else {
        let values = value
            .as_array()
            .ok_or_else(|| SchemaError::event(Some(seq), "content must be blocks or spill"))?;
        validate_inline_size(value, seq)?;
        validate_blocks(values, seq)
    }
}

fn validate_string_or_spill(value: &Value, seq: u64) -> Result<(), SchemaError> {
    if is_spill(value) {
        validate_spill(value, seq)
    } else {
        if value.as_str().is_none() {
            return Err(SchemaError::event(
                Some(seq),
                "value must be string or spill",
            ));
        }
        validate_inline_size(value, seq)
    }
}

fn validate_json_or_spill(value: &Value, seq: u64) -> Result<(), SchemaError> {
    if is_spill(value) {
        validate_spill(value, seq)
    } else {
        validate_inline_size(value, seq)
    }
}

fn validate_inline_size(value: &Value, seq: u64) -> Result<(), SchemaError> {
    let bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| SchemaError::event(Some(seq), error.to_string()))?;
    if bytes.len() > SPILL_THRESHOLD {
        return Err(SchemaError::event(
            Some(seq),
            "inline spill position exceeds 16384 bytes",
        ));
    }
    Ok(())
}

fn is_spill(value: &Value) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.len() == 1 && object.contains_key("$spill"))
}

fn validate_spill(value: &Value, seq: u64) -> Result<(), SchemaError> {
    let spill = value
        .as_object()
        .and_then(|object| object.get("$spill"))
        .and_then(Value::as_object)
        .ok_or_else(|| SchemaError::event(Some(seq), "invalid $spill reference"))?;
    required_str(spill, "asset", Some(seq))?;
    let bytes = required_u64(spill, "bytes", Some(seq))?;
    if bytes <= SPILL_THRESHOLD as u64 {
        return Err(SchemaError::event(
            Some(seq),
            "$spill bytes must exceed 16384",
        ));
    }
    Ok(())
}

fn validate_resume(value: &Value, seq: u64) -> Result<(), SchemaError> {
    match value {
        Value::String(value) if value == "never" => Ok(()),
        Value::Object(value) if value.len() == 1 => {
            required_u64(value, "bounded", Some(seq))?;
            Ok(())
        }
        _ => Err(SchemaError::event(
            Some(seq),
            "resume must be never or bounded",
        )),
    }
}

fn validate_origin_equality(
    object: &Map<String, Value>,
    origin_key: Option<&str>,
    seq: u64,
) -> Result<(), SchemaError> {
    let Some(origin) = object.get("origin_tuple") else {
        return Ok(());
    };
    let origin = origin
        .as_object()
        .ok_or_else(|| SchemaError::event(Some(seq), "origin_tuple must be object"))?;
    let key = required_str(origin, "key", Some(seq))?;
    if origin_key != Some(key) {
        return Err(SchemaError::constraint(
            6,
            seq,
            "top-level origin_tuple requires matching origin_key",
        ));
    }
    Ok(())
}

fn validate_origin_tuple(value: &Map<String, Value>, seq: u64) -> Result<(), SchemaError> {
    for field in ["principal", "client", "target", "op", "key"] {
        if required_str(value, field, Some(seq))?.is_empty() {
            return Err(SchemaError::event(
                Some(seq),
                format!("origin_tuple.{field} must not be empty"),
            ));
        }
    }
    Ok(())
}

fn validate_timestamp(value: &str, seq: u64) -> Result<(), SchemaError> {
    let bytes = value.as_bytes();
    let valid = bytes.len() == 24
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'.'
        && bytes[23] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19 | 23) || byte.is_ascii_digit()
        });
    let calendar_valid = valid
        && parse_decimal(&bytes[5..7]).is_some_and(|month| (1..=12).contains(&month))
        && parse_decimal(&bytes[8..10]).is_some_and(|day| {
            let year = parse_decimal(&bytes[0..4]).expect("shape checked");
            let month = parse_decimal(&bytes[5..7]).expect("shape checked");
            (1..=days_in_month(year, month)).contains(&day)
        })
        && parse_decimal(&bytes[11..13]).is_some_and(|hour| hour <= 23)
        && parse_decimal(&bytes[14..16]).is_some_and(|minute| minute <= 59)
        && parse_decimal(&bytes[17..19]).is_some_and(|second| second <= 59);
    if !calendar_valid {
        return Err(SchemaError::event(
            Some(seq),
            "ts must be UTC ISO8601 with milliseconds",
        ));
    }
    Ok(())
}

fn parse_decimal(bytes: &[u8]) -> Option<u32> {
    bytes.iter().try_fold(0_u32, |value, byte| {
        byte.is_ascii_digit()
            .then_some(value * 10 + u32::from(*byte - b'0'))
    })
}

const fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn is_leap_year(year: u32) -> bool {
    divisible_by(year, 4) && (!divisible_by(year, 100) || divisible_by(year, 400))
}

const fn divisible_by(value: u32, divisor: u32) -> bool {
    value / divisor * divisor == value
}

fn required<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a Value, SchemaError> {
    object
        .get(field)
        .filter(|value| !value.is_null())
        .ok_or_else(|| {
            SchemaError::event(Some(seq), format!("required field {field} missing or null"))
        })
}

fn required_str<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: Option<u64>,
) -> Result<&'a str, SchemaError> {
    object
        .get(field)
        .filter(|value| !value.is_null())
        .and_then(Value::as_str)
        .ok_or_else(|| SchemaError::event(seq, format!("{field} must be a string")))
}

fn required_id<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a str, SchemaError> {
    let value = required_str(object, field, Some(seq))?;
    if value.is_empty() {
        return Err(SchemaError::event(
            Some(seq),
            format!("{field} must not be empty"),
        ));
    }
    Ok(value)
}

fn optional_str<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Option<&'a str>, SchemaError> {
    object
        .get(field)
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be a string")))
        })
        .transpose()
}

fn required_u64(
    object: &Map<String, Value>,
    field: &str,
    seq: Option<u64>,
) -> Result<u64, SchemaError> {
    let value = object
        .get(field)
        .filter(|value| !value.is_null())
        .and_then(Value::as_u64)
        .filter(|value| *value <= MAX_SAFE_INTEGER)
        .ok_or_else(|| {
            SchemaError::event(seq, format!("{field} must be a nonnegative safe integer"))
        })?;
    Ok(value)
}

fn optional_u64(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Option<u64>, SchemaError> {
    object
        .get(field)
        .map(|value| {
            value
                .as_u64()
                .filter(|value| *value <= MAX_SAFE_INTEGER)
                .ok_or_else(|| {
                    SchemaError::event(
                        Some(seq),
                        format!("{field} must be a nonnegative safe integer"),
                    )
                })
        })
        .transpose()
}

fn required_bool(object: &Map<String, Value>, field: &str, seq: u64) -> Result<bool, SchemaError> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be boolean")))
}

fn optional_bool(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Option<bool>, SchemaError> {
    object
        .get(field)
        .map(|value| {
            value
                .as_bool()
                .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be boolean")))
        })
        .transpose()
}

fn required_object<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a Map<String, Value>, SchemaError> {
    required(object, field, seq)?
        .as_object()
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be an object")))
}

fn optional_object<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Option<&'a Map<String, Value>>, SchemaError> {
    object
        .get(field)
        .map(|value| {
            value
                .as_object()
                .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be an object")))
        })
        .transpose()
}

fn required_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<&'a [Value], SchemaError> {
    required(object, field, seq)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be an array")))
}

fn optional_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Option<&'a [Value]>, SchemaError> {
    object
        .get(field)
        .map(|value| {
            value
                .as_array()
                .map(Vec::as_slice)
                .ok_or_else(|| SchemaError::event(Some(seq), format!("{field} must be an array")))
        })
        .transpose()
}

fn required_enum<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    allowed: &[&str],
    seq: u64,
) -> Result<&'a str, SchemaError> {
    let value = required_str(object, field, Some(seq))?;
    if !allowed.contains(&value) {
        return Err(SchemaError::event(
            Some(seq),
            format!("invalid {field}: {value}"),
        ));
    }
    Ok(value)
}

fn optional_json_value(object: &Map<String, Value>, field: &str) -> Result<(), SchemaError> {
    if let Some(value) = object.get(field) {
        serde_json_canonicalizer::to_vec(value)
            .map_err(|error| SchemaError::Canonical(error.to_string()))?;
    }
    Ok(())
}

fn optional_visibility(
    object: &Map<String, Value>,
    seq: u64,
) -> Result<Option<Visibility>, SchemaError> {
    optional_str(object, "visibility", seq)?
        .map(|value| match value {
            "model" => Ok(Visibility::Model),
            "runtime" => Ok(Visibility::Runtime),
            _ => Err(SchemaError::event(
                Some(seq),
                "visibility must be model or runtime",
            )),
        })
        .transpose()
}

pub(crate) fn ranges(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Vec<(u64, u64)>, SchemaError> {
    let values = required_array(object, field, seq)?;
    values
        .iter()
        .map(|value| {
            let value = value.as_object().ok_or_else(|| {
                SchemaError::event(Some(seq), format!("{field} range must be object"))
            })?;
            let from = required_u64(value, "from", Some(seq))?;
            let to = required_u64(value, "to", Some(seq))?;
            if from > to {
                return Err(SchemaError::event(
                    Some(seq),
                    format!("{field} range is reversed"),
                ));
            }
            Ok((from, to))
        })
        .collect()
}

fn validate_ranges_optional(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<(), SchemaError> {
    if object.contains_key(field) {
        ranges(object, field, seq)?;
    }
    Ok(())
}

fn validate_ranges_required_from_envelope(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Vec<(u64, u64)>, SchemaError> {
    ranges(object, field, seq)
}

fn validate_ranges_required_from_payload(
    object: &Map<String, Value>,
    field: &str,
    seq: u64,
) -> Result<Vec<(u64, u64)>, SchemaError> {
    ranges(object, field, seq)
}

fn validate_u64_array(values: &[Value], seq: u64, name: &str) -> Result<(), SchemaError> {
    for value in values {
        if value
            .as_u64()
            .filter(|value| *value <= MAX_SAFE_INTEGER)
            .is_none()
        {
            return Err(SchemaError::event(
                Some(seq),
                format!("{name} must contain safe integers"),
            ));
        }
    }
    Ok(())
}

fn validate_strictly_increasing_u64(
    values: &[Value],
    seq: u64,
    name: &str,
) -> Result<(), SchemaError> {
    let mut previous = None;
    for value in values {
        let value = value
            .as_u64()
            .filter(|value| *value <= MAX_SAFE_INTEGER)
            .ok_or_else(|| {
                SchemaError::event(Some(seq), format!("{name} must contain safe integers"))
            })?;
        if previous.is_some_and(|old| old >= value) {
            return Err(SchemaError::event(
                Some(seq),
                format!("{name} must be strictly increasing"),
            ));
        }
        previous = Some(value);
    }
    Ok(())
}
