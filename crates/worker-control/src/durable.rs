//! Durable supervisor control: the endpoint queue transaction and the
//! request-id-keyed tool-control exchange (worker-control §Durable control).

use schema::{Block, IJsonValue, OriginTuple};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{AssetRef, PROTOCOL_VERSION, Selected};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const REQUEST_DOMAIN: &[u8] = b"tekes-tool-control-v2\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkerStartup {
    QueueTransaction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub version: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup: Option<WorkerStartup>,
}

impl Selection {
    pub fn validate(&self) -> Result<(), DurableControlError> {
        if self.startup.is_some() && self.version != PROTOCOL_VERSION {
            return Err(DurableControlError::ProtocolVersion(self.version));
        }
        Ok(())
    }

    #[must_use]
    pub const fn selected(self) -> Selected {
        Selected {
            version: self.version,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum QueueTransactionAction {
    Edit {
        replacement_origin: OriginTuple,
        content: Vec<Block>,
        steer: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assets: Option<Vec<AssetRef>>,
    },
    Remove,
    Steer {
        replacement_origin: OriginTuple,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueueTransaction {
    pub delivery: String,
    pub rpc_id: String,
    pub target_seq: u64,
    pub retract_origin: OriginTuple,
    pub action: QueueTransactionAction,
}

impl QueueTransaction {
    pub fn validate(&self) -> Result<(), DurableControlError> {
        validate_nonempty("delivery", &self.delivery)?;
        validate_nonempty("rpc_id", &self.rpc_id)?;
        validate_safe_positive("target_seq", self.target_seq)?;
        validate_origin(&self.retract_origin)?;
        if self.retract_origin.key != format!("{}/retract", self.rpc_id) {
            return Err(DurableControlError::QueueOriginKey("retract_origin"));
        }
        match &self.action {
            QueueTransactionAction::Edit {
                replacement_origin,
                content,
                ..
            } => {
                validate_replacement_origin(self, replacement_origin)?;
                if content.is_empty() {
                    return Err(DurableControlError::QueueContentEmpty);
                }
            }
            QueueTransactionAction::Remove => {}
            QueueTransactionAction::Steer { replacement_origin } => {
                validate_replacement_origin(self, replacement_origin)?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn replacement_origin(&self) -> Option<&OriginTuple> {
        match &self.action {
            QueueTransactionAction::Edit {
                replacement_origin, ..
            }
            | QueueTransactionAction::Steer { replacement_origin } => Some(replacement_origin),
            QueueTransactionAction::Remove => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QueueTransactionRejectCode {
    QueueItemNotFound,
    SteerUnavailable,
    AttachmentError,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum QueueTransactionOutcome {
    Committed {
        first_seq: u64,
        last_seq: u64,
        deduplicated: bool,
    },
    Rejected {
        code: QueueTransactionRejectCode,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueueTransactionResult {
    pub delivery: String,
    pub outcome: QueueTransactionOutcome,
}

impl QueueTransactionResult {
    pub fn validate(&self) -> Result<(), DurableControlError> {
        validate_nonempty("delivery", &self.delivery)?;
        match &self.outcome {
            QueueTransactionOutcome::Committed {
                first_seq,
                last_seq,
                ..
            } => {
                validate_safe_positive("first_seq", *first_seq)?;
                validate_safe_positive("last_seq", *last_seq)?;
                if first_seq > last_seq {
                    return Err(DurableControlError::QueueResultRange);
                }
            }
            QueueTransactionOutcome::Rejected { code, reason } => match (code, reason) {
                (QueueTransactionRejectCode::AttachmentError, Some(reason))
                    if !reason.is_empty() => {}
                (QueueTransactionRejectCode::AttachmentError, _) => {
                    return Err(DurableControlError::QueueAttachmentReason);
                }
                (_, None) => {}
                (_, Some(_)) => return Err(DurableControlError::QueueUnexpectedReason),
            },
        }
        Ok(())
    }

    pub fn validate_for(&self, request: &QueueTransaction) -> Result<(), DurableControlError> {
        self.validate()?;
        request.validate()?;
        if self.delivery != request.delivery {
            return Err(DurableControlError::QueueDeliveryMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolControl {
    pub request_id: String,
    pub session: String,
    pub thread: String,
    pub turn: u64,
    pub call_id: String,
    pub name: String,
    pub arguments: IJsonValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToolControlBinding<'a> {
    pub session: &'a str,
    pub thread: &'a str,
    pub turn: u64,
    pub call_id: &'a str,
    pub name: &'a str,
    pub arguments: &'a IJsonValue,
}

impl ToolControl {
    pub fn new(
        session: impl Into<String>,
        thread: impl Into<String>,
        turn: u64,
        call_id: impl Into<String>,
        name: impl Into<String>,
        arguments: IJsonValue,
    ) -> Result<Self, DurableControlError> {
        let session = session.into();
        let thread = thread.into();
        let call_id = call_id.into();
        let name = name.into();
        let request_id = derive_request_id(&session, &thread, turn, &call_id)?;
        let request = Self {
            request_id,
            session,
            thread,
            turn,
            call_id,
            name,
            arguments,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), DurableControlError> {
        validate_request_id_syntax(&self.request_id)?;
        validate_preimage(&self.session, &self.thread, self.turn, &self.call_id)?;
        if self.name.is_empty() {
            return Err(DurableControlError::EmptyName);
        }
        let expected = derive_request_id(&self.session, &self.thread, self.turn, &self.call_id)?;
        if self.request_id != expected {
            return Err(DurableControlError::RequestIdMismatch {
                expected,
                actual: self.request_id.clone(),
            });
        }
        Ok(())
    }

    /// Validates the process binding and the paired durable effective execution.
    pub fn validate_against(
        &self,
        binding: &ToolControlBinding<'_>,
    ) -> Result<(), DurableControlError> {
        self.validate()?;
        if self.session != binding.session {
            return Err(DurableControlError::BindingMismatch("session"));
        }
        if self.thread != binding.thread {
            return Err(DurableControlError::BindingMismatch("thread"));
        }
        if self.turn != binding.turn {
            return Err(DurableControlError::BindingMismatch("turn"));
        }
        if self.call_id != binding.call_id {
            return Err(DurableControlError::BindingMismatch("call_id"));
        }
        if self.name != binding.name {
            return Err(DurableControlError::BindingMismatch("name"));
        }
        if &self.arguments != binding.arguments {
            return Err(DurableControlError::BindingMismatch("arguments"));
        }
        Ok(())
    }

    /// Performs validation that is safe before looking up an existing receipt.
    /// The derived-id equality check is deliberately deferred so reuse of a valid
    /// request id with different tuple fields is classified as a conflict.
    pub fn validate_for_receipt_lookup(&self) -> Result<(), DurableControlError> {
        validate_request_id_syntax(&self.request_id)?;
        validate_preimage(&self.session, &self.thread, self.turn, &self.call_id)?;
        if self.name.is_empty() {
            return Err(DurableControlError::EmptyName);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolControlErrorCode {
    Unsupported,
    Denied,
    NotFound,
    Conflict,
    Unavailable,
    Timeout,
    /// The backend may have committed the business effect, but neither its
    /// receipt nor authoritative reconciliation can currently prove the state.
    EffectUnknown,
    /// The external authority reported incompatible state for the stable
    /// idempotency key. Automatic execution is forbidden.
    EffectConflicted,
    Internal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolControlError {
    pub code: ToolControlErrorCode,
    pub message: String,
    pub retryable: bool,
}

/// Host-owned pending outcome of a tool-control request: the supervisor bound
/// a remote continuation (an MCP task) instead of a terminal value. The worker
/// drives it to exactly one terminal through `tool_continuation` steps; an
/// ordinary `value` can never impersonate this state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingContinuation {
    pub continuation_id: String,
    /// The first continuation step the worker may request (always 1 for an
    /// initial pending result).
    pub next_step: u64,
    pub state: IJsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolControlResult {
    pub request_id: String,
    pub call_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<IJsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ToolControlError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<PendingContinuation>,
}

impl ToolControlResult {
    #[must_use]
    pub fn pending(
        request_id: impl Into<String>,
        call_id: impl Into<String>,
        continuation_id: impl Into<String>,
        state: IJsonValue,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            call_id: call_id.into(),
            value: None,
            error: None,
            pending: Some(PendingContinuation {
                continuation_id: continuation_id.into(),
                next_step: 1,
                state,
            }),
        }
    }

    #[must_use]
    pub fn success(
        request_id: impl Into<String>,
        call_id: impl Into<String>,
        value: IJsonValue,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            call_id: call_id.into(),
            value: Some(value),
            error: None,
            pending: None,
        }
    }

    #[must_use]
    pub fn failure(
        request_id: impl Into<String>,
        call_id: impl Into<String>,
        error: ToolControlError,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            call_id: call_id.into(),
            value: None,
            error: Some(error),
            pending: None,
        }
    }

    #[must_use]
    pub fn conflict(request: &ToolControl) -> Self {
        Self::failure(
            request.request_id.clone(),
            request.call_id.clone(),
            ToolControlError {
                code: ToolControlErrorCode::Conflict,
                message: "request_id was reused with different request fields".to_owned(),
                retryable: false,
            },
        )
    }

    pub fn validate(&self) -> Result<(), DurableControlError> {
        validate_request_id_syntax(&self.request_id)?;
        if self.call_id.is_empty() {
            return Err(DurableControlError::EmptyCallId);
        }
        match (
            self.value.is_some(),
            self.error.is_some(),
            self.pending.as_ref(),
        ) {
            (true, false, None) | (false, true, None) => Ok(()),
            (false, false, Some(pending)) => {
                if pending.next_step != 1
                    || pending.continuation_id.len() != 64
                    || !pending
                        .continuation_id
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(DurableControlError::ResultUnion);
                }
                Ok(())
            }
            _ => Err(DurableControlError::ResultUnion),
        }
    }

    pub fn validate_for(&self, request: &ToolControl) -> Result<(), DurableControlError> {
        self.validate()?;
        if self.request_id != request.request_id {
            return Err(DurableControlError::BindingMismatch("request_id"));
        }
        if self.call_id != request.call_id {
            return Err(DurableControlError::BindingMismatch("call_id"));
        }
        Ok(())
    }
}

/// Every durable control message is defined at the negotiated protocol
/// version; a peer that negotiated anything else must not send or accept one.
pub fn require_version(selected: &Selected) -> Result<(), DurableControlError> {
    if selected.version != PROTOCOL_VERSION {
        return Err(DurableControlError::ProtocolVersion(selected.version));
    }
    Ok(())
}

pub fn derive_request_id(
    session: &str,
    thread: &str,
    turn: u64,
    call_id: &str,
) -> Result<String, DurableControlError> {
    validate_preimage(session, thread, turn, call_id)?;
    let mut digest = Sha256::new();
    digest.update(REQUEST_DOMAIN);
    digest.update(session.as_bytes());
    digest.update([0]);
    digest.update(thread.as_bytes());
    digest.update([0]);
    digest.update(turn.to_string().as_bytes());
    digest.update([0]);
    digest.update(call_id.as_bytes());
    Ok(format!("{digest:x}", digest = digest.finalize()))
}

pub fn encode_tool_control(request: &ToolControl) -> Result<Vec<u8>, DurableControlError> {
    request.validate()?;
    encode_line("tool_control", request)
}

pub fn decode_tool_control(line: &[u8]) -> Result<ToolControl, DurableControlError> {
    let request: ToolControl = decode_named(line, "tool_control")?;
    request.validate()?;
    Ok(request)
}

pub fn encode_tool_control_result(
    result: &ToolControlResult,
) -> Result<Vec<u8>, DurableControlError> {
    result.validate()?;
    encode_line("tool_control_result", result)
}

pub fn decode_tool_control_result(line: &[u8]) -> Result<ToolControlResult, DurableControlError> {
    let result: ToolControlResult = decode_named(line, "tool_control_result")?;
    result.validate()?;
    Ok(result)
}

pub fn decode_selection(line: &[u8]) -> Result<Selection, DurableControlError> {
    let selection: Selection = decode_named(line, "selected")?;
    selection.validate()?;
    Ok(selection)
}

pub fn encode_queue_transaction(
    request: &QueueTransaction,
) -> Result<Vec<u8>, DurableControlError> {
    request.validate()?;
    encode_line("queue_transaction", request)
}

pub fn decode_queue_transaction(line: &[u8]) -> Result<QueueTransaction, DurableControlError> {
    let request: QueueTransaction = decode_named(line, "queue_transaction")?;
    request.validate()?;
    Ok(request)
}

pub fn encode_queue_transaction_result(
    result: &QueueTransactionResult,
) -> Result<Vec<u8>, DurableControlError> {
    result.validate()?;
    encode_line("queue_transaction_result", result)
}

pub fn decode_queue_transaction_result(
    line: &[u8],
) -> Result<QueueTransactionResult, DurableControlError> {
    let result: QueueTransactionResult = decode_named(line, "queue_transaction_result")?;
    result.validate()?;
    Ok(result)
}

fn validate_replacement_origin(
    request: &QueueTransaction,
    replacement: &OriginTuple,
) -> Result<(), DurableControlError> {
    validate_origin(replacement)?;
    if replacement.key != format!("{}/replacement", request.rpc_id) {
        return Err(DurableControlError::QueueOriginKey("replacement_origin"));
    }
    let retract = &request.retract_origin;
    if replacement.principal != retract.principal
        || replacement.client != retract.client
        || replacement.target != retract.target
        || replacement.op != retract.op
    {
        return Err(DurableControlError::QueueOriginBinding);
    }
    Ok(())
}

fn validate_origin(origin: &OriginTuple) -> Result<(), DurableControlError> {
    for (field, value) in [
        ("principal", &origin.principal),
        ("client", &origin.client),
        ("target", &origin.target),
        ("op", &origin.op),
        ("key", &origin.key),
    ] {
        validate_nonempty(field, value)?;
    }
    Ok(())
}

fn validate_nonempty(field: &'static str, value: &str) -> Result<(), DurableControlError> {
    if value.is_empty() {
        return Err(DurableControlError::EmptyField(field));
    }
    Ok(())
}

fn validate_safe_positive(field: &'static str, value: u64) -> Result<(), DurableControlError> {
    if value == 0 || value > MAX_SAFE_INTEGER {
        return Err(DurableControlError::SafePositiveInteger { field, value });
    }
    Ok(())
}

fn validate_preimage(
    session: &str,
    thread: &str,
    turn: u64,
    call_id: &str,
) -> Result<(), DurableControlError> {
    if !is_canonical_uuid(session) {
        return Err(DurableControlError::SessionUuid);
    }
    if !is_canonical_uuid(thread) {
        return Err(DurableControlError::ThreadUuid);
    }
    if turn == 0 || turn > MAX_SAFE_INTEGER {
        return Err(DurableControlError::Turn(turn));
    }
    if call_id.is_empty() {
        return Err(DurableControlError::EmptyCallId);
    }
    Ok(())
}

fn validate_request_id_syntax(request_id: &str) -> Result<(), DurableControlError> {
    if request_id.len() != 64
        || !request_id
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(DurableControlError::RequestIdSyntax);
    }
    Ok(())
}

fn is_canonical_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
        })
}

fn encode_line<T: Serialize>(key: &str, payload: &T) -> Result<Vec<u8>, DurableControlError> {
    let mut object = Map::new();
    object.insert(key.to_owned(), serde_json::to_value(payload)?);
    let mut bytes = serde_json_canonicalizer::to_vec(&Value::Object(object))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn decode_named<T: DeserializeOwned>(
    line: &[u8],
    expected: &str,
) -> Result<T, DurableControlError> {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    let value: Value = serde_json::from_slice(line)?;
    let object = value
        .as_object()
        .ok_or(DurableControlError::TopLevelKeyCount)?;
    if object.len() != 1 {
        return Err(DurableControlError::TopLevelKeyCount);
    }
    let (actual, payload) = object.iter().next().expect("one key");
    if actual != expected {
        return Err(DurableControlError::UnexpectedMessage {
            expected: expected.to_owned(),
            actual: actual.clone(),
        });
    }
    serde_json::from_value(payload.clone()).map_err(DurableControlError::Json)
}

#[derive(Debug, Error)]
pub enum DurableControlError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("protocol line must contain exactly one top-level key")]
    TopLevelKeyCount,
    #[error("unexpected message {actual}; expected {expected}")]
    UnexpectedMessage { expected: String, actual: String },
    #[error("request_id must be 64 lowercase hexadecimal characters")]
    RequestIdSyntax,
    #[error("thread must be a lowercase canonical UUID")]
    ThreadUuid,
    #[error("session must be a lowercase canonical UUID")]
    SessionUuid,
    #[error("turn must be a positive I-JSON safe integer, got {0}")]
    Turn(u64),
    #[error("call_id must not be empty")]
    EmptyCallId,
    #[error("tool name must not be empty")]
    EmptyName,
    #[error("request_id mismatch: expected {expected}, got {actual}")]
    RequestIdMismatch { expected: String, actual: String },
    #[error("tool-control {0} does not match its durable/process binding")]
    BindingMismatch(&'static str),
    #[error("tool_control_result must contain exactly one of value or error")]
    ResultUnion,
    #[error("durable control requires protocol version {PROTOCOL_VERSION}, got {0}")]
    ProtocolVersion(u64),
    #[error("{0} must not be empty")]
    EmptyField(&'static str),
    #[error("{field} must be a positive I-JSON safe integer, got {value}")]
    SafePositiveInteger { field: &'static str, value: u64 },
    #[error("{0} key does not match the transaction rpc_id")]
    QueueOriginKey(&'static str),
    #[error("replacement origin tuple must match the retraction tuple except for key")]
    QueueOriginBinding,
    #[error("queue edit replacement content must not be empty")]
    QueueContentEmpty,
    #[error("committed queue transaction has an invalid sequence range")]
    QueueResultRange,
    #[error("attachment-error requires a nonempty reason")]
    QueueAttachmentReason,
    #[error("only attachment-error may carry a reason")]
    QueueUnexpectedReason,
    #[error("queue_transaction_result delivery does not match its request")]
    QueueDeliveryMismatch,
}

#[cfg(test)]
mod queue_transaction_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn fixture(name: &str) -> Vec<u8> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("fixtures/wire/durable")
            .join(name);
        fs::read(root).expect("wire fixture")
    }

    #[test]
    fn canonical_queue_transcripts_round_trip_byte_for_byte() {
        for name in [
            "queue-transaction.jsonl",
            "queue-transaction-retry.jsonl",
            "queue-transaction-startup.jsonl",
            "queue-transaction-rejections.jsonl",
        ] {
            for line in fixture(name).split_inclusive(|byte| *byte == b'\n') {
                let value: Value = serde_json::from_slice(line).expect("fixture JSON");
                let key = value.as_object().unwrap().keys().next().unwrap().as_str();
                let encoded = match key {
                    "selected" => {
                        let selection = decode_selection(line).expect("selection");
                        encode_line("selected", &selection).expect("encode selection")
                    }
                    "queue_transaction" => {
                        let request = decode_queue_transaction(line).expect("transaction");
                        encode_queue_transaction(&request).expect("encode transaction")
                    }
                    "queue_transaction_result" => {
                        let result =
                            decode_queue_transaction_result(line).expect("transaction result");
                        encode_queue_transaction_result(&result).expect("encode result")
                    }
                    "hello" => continue,
                    other => panic!("unexpected queue fixture message {other}"),
                };
                assert_eq!(encoded, line, "{name}: {key}");
            }
        }
    }

    #[test]
    fn queue_result_union_is_closed() {
        let bad = br#"{"queue_transaction_result":{"delivery":"d","outcome":{"code":"queue-item-not-found","kind":"rejected","reason":"not allowed"}}}"#;
        assert!(matches!(
            decode_queue_transaction_result(bad),
            Err(DurableControlError::QueueUnexpectedReason)
        ));
    }

    #[test]
    fn queue_result_is_correlated_by_delivery() {
        let lines = fixture("queue-transaction.jsonl");
        let mut lines = lines.split_inclusive(|byte| *byte == b'\n');
        let request = decode_queue_transaction(lines.next().unwrap()).expect("request");
        let mut result = decode_queue_transaction_result(lines.next().unwrap()).expect("result");
        result.delivery = "other-delivery".to_owned();
        assert!(matches!(
            result.validate_for(&request),
            Err(DurableControlError::QueueDeliveryMismatch)
        ));
    }
}
