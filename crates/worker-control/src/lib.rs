use schema::{Block, IJsonValue, OriginTuple, ResumePolicy, SeqRange};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

pub mod continuation;
pub mod durable;

pub use durable::*;

pub const PROTOCOL_NAME: &str = "tekes-worker";
pub const PROTOCOL_VERSION: u64 = 2;
pub const EX_PROTOCOL: i32 = 76;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hello {
    pub proto: String,
    pub min: u64,
    pub max: u64,
}

impl Default for Hello {
    fn default() -> Self {
        Self {
            proto: PROTOCOL_NAME.to_owned(),
            min: PROTOCOL_VERSION,
            max: PROTOCOL_VERSION,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Selected {
    pub version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Reject {
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetRef {
    pub asset: String,
    pub mime: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub delivery: String,
    pub origin: OriginTuple,
    pub content: Vec<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission: Option<IJsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetRef>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub delivery: String,
    pub origin: OriginTuple,
    pub call: String,
    pub grant: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<IJsonValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueueEdit {
    pub delivery: String,
    pub origin: OriginTuple,
    pub supersedes: Vec<SeqRange>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub delivery: String,
    pub origin: OriginTuple,
    pub generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lease {
    pub attempt: String,
    pub granted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ping {
    pub id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Compact {
    pub delivery: String,
    pub origin: OriginTuple,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub delivery: String,
    pub origin: OriginTuple,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LaunchResult {
    pub child: String,
    pub spawn_id: String,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SupervisorMessage {
    Input(Input),
    ApprovalResponse(ApprovalResponse),
    QueueEdit(QueueEdit),
    Stop(Stop),
    Lease(Lease),
    Ping(Ping),
    Compact(Compact),
    Meta(Meta),
    LaunchResult(LaunchResult),
    QueueTransaction(durable::QueueTransaction),
    Unknown { key: String, payload: IJsonValue },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub delivery: String,
    pub seq: u64,
    pub deduplicated: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttemptSettled {
    pub attempt: String,
    pub outcome_seq: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LeaseRequest {
    pub attempt: String,
    pub class: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Appended {
    pub seq: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameChannel {
    Text,
    Reasoning,
    Tool,
    Usage,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    /// The provider completed this call's arguments; not an execution result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments_complete: Option<bool>,
    /// Ledger high-water (last durable seq) when the worker wrote this frame.
    /// A causal cut for the supervisor: durable events beyond it were appended
    /// after the frame left the worker, so a projection refresh triggered by
    /// this frame must not publish them ahead of it. Absent on legacy frames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger_seq: Option<u64>,
    pub attempt: String,
    pub channel: FrameChannel,
    pub block: u64,
    pub delta: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Frame {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        match self.channel {
            FrameChannel::Tool
                if self
                    .call_id
                    .as_deref()
                    .is_some_and(|value| !value.is_empty()) =>
            {
                Ok(())
            }
            FrameChannel::Tool => Err(ProtocolError::InvalidFrame(
                "tool frame requires nonempty call_id".to_owned(),
            )),
            FrameChannel::Text | FrameChannel::Reasoning | FrameChannel::Usage
                if self.call_id.is_none()
                    && self.name.is_none()
                    && self.arguments_complete.is_none() =>
            {
                Ok(())
            }
            FrameChannel::Text | FrameChannel::Reasoning | FrameChannel::Usage => {
                Err(ProtocolError::InvalidFrame(
                    "text/reasoning frame forbids call_id and name".to_owned(),
                ))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub phase: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pong {
    pub id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LaunchChild {
    pub child: String,
    pub spawn_id: String,
    pub resume: ResumePolicy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorkerMessage {
    Receipt(Receipt),
    AttemptSettled(AttemptSettled),
    LeaseRequest(LeaseRequest),
    Appended(Appended),
    Frame(Frame),
    State(State),
    Pong(Pong),
    LaunchChild(LaunchChild),
    QueueTransactionResult(durable::QueueTransactionResult),
    Unknown { key: String, payload: IJsonValue },
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("protocol line must contain exactly one top-level key")]
    TopLevelKeyCount,
    #[error("invalid {message} payload: {source}")]
    Payload {
        message: String,
        source: serde_json::Error,
    },
    #[error("protocol name mismatch: {0}")]
    ProtocolName(String),
    #[error("unexpected message {actual}; expected {expected}")]
    UnexpectedMessage { expected: String, actual: String },
    #[error("no mutual protocol version")]
    NoMutualVersion,
    #[error("invalid frame: {0}")]
    InvalidFrame(String),
    #[error("invalid durable control message: {0}")]
    InvalidDurableControl(String),
}

pub fn negotiate(
    hello: &Hello,
    supervisor_min: u64,
    supervisor_max: u64,
) -> Result<Selected, ProtocolError> {
    if hello.proto != PROTOCOL_NAME {
        return Err(ProtocolError::ProtocolName(hello.proto.clone()));
    }
    let lower = hello.min.max(supervisor_min);
    let upper = hello.max.min(supervisor_max);
    if lower > upper {
        return Err(ProtocolError::NoMutualVersion);
    }
    Ok(Selected { version: upper })
}

pub fn decode_hello(line: &[u8]) -> Result<Hello, ProtocolError> {
    decode_named(line, "hello")
}

pub fn decode_selected(line: &[u8]) -> Result<Selected, ProtocolError> {
    decode_named(line, "selected")
}

pub fn decode_reject(line: &[u8]) -> Result<Reject, ProtocolError> {
    decode_named(line, "reject")
}

pub fn decode_supervisor(line: &[u8]) -> Result<SupervisorMessage, ProtocolError> {
    let (key, payload) = split_line(line)?;
    Ok(match key.as_str() {
        "input" => SupervisorMessage::Input(from_payload(&key, payload)?),
        "approval_response" => SupervisorMessage::ApprovalResponse(from_payload(&key, payload)?),
        "queue_edit" => SupervisorMessage::QueueEdit(from_payload(&key, payload)?),
        "stop" => SupervisorMessage::Stop(from_payload(&key, payload)?),
        "lease" => SupervisorMessage::Lease(from_payload(&key, payload)?),
        "ping" => SupervisorMessage::Ping(from_payload(&key, payload)?),
        "compact" => SupervisorMessage::Compact(from_payload(&key, payload)?),
        "meta" => SupervisorMessage::Meta(from_payload(&key, payload)?),
        "launch_result" => SupervisorMessage::LaunchResult(from_payload(&key, payload)?),
        "queue_transaction" => {
            let message: durable::QueueTransaction = from_payload(&key, payload)?;
            message
                .validate()
                .map_err(|source| ProtocolError::InvalidDurableControl(source.to_string()))?;
            SupervisorMessage::QueueTransaction(message)
        }
        _ => SupervisorMessage::Unknown {
            key,
            payload: serde_json::from_value(payload)?,
        },
    })
}

pub fn decode_worker(line: &[u8]) -> Result<WorkerMessage, ProtocolError> {
    let (key, payload) = split_line(line)?;
    Ok(match key.as_str() {
        "receipt" => WorkerMessage::Receipt(from_payload(&key, payload)?),
        "attempt_settled" => WorkerMessage::AttemptSettled(from_payload(&key, payload)?),
        "lease_request" => WorkerMessage::LeaseRequest(from_payload(&key, payload)?),
        "appended" => WorkerMessage::Appended(from_payload(&key, payload)?),
        "frame" => {
            let frame: Frame = from_payload(&key, payload)?;
            frame.validate()?;
            WorkerMessage::Frame(frame)
        }
        "state" => WorkerMessage::State(from_payload(&key, payload)?),
        "pong" => WorkerMessage::Pong(from_payload(&key, payload)?),
        "launch_child" => WorkerMessage::LaunchChild(from_payload(&key, payload)?),
        "queue_transaction_result" => {
            let message: durable::QueueTransactionResult = from_payload(&key, payload)?;
            message
                .validate()
                .map_err(|source| ProtocolError::InvalidDurableControl(source.to_string()))?;
            WorkerMessage::QueueTransactionResult(message)
        }
        _ => WorkerMessage::Unknown {
            key,
            payload: serde_json::from_value(payload)?,
        },
    })
}

pub fn encode_line<T: Serialize>(key: &str, payload: &T) -> Result<Vec<u8>, ProtocolError> {
    let mut object = Map::new();
    object.insert(key.to_owned(), serde_json::to_value(payload)?);
    let mut bytes = serde_json_canonicalizer::to_vec(&Value::Object(object))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn decode_named<T: DeserializeOwned>(line: &[u8], expected: &str) -> Result<T, ProtocolError> {
    let (key, payload) = split_line(line)?;
    if key != expected {
        return Err(ProtocolError::UnexpectedMessage {
            expected: expected.to_owned(),
            actual: key,
        });
    }
    from_payload(expected, payload)
}

fn split_line(line: &[u8]) -> Result<(String, Value), ProtocolError> {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    let value: Value = serde_json::from_slice(line)?;
    let object = value
        .as_object()
        .cloned()
        .ok_or(ProtocolError::TopLevelKeyCount)?;
    if object.len() != 1 {
        return Err(ProtocolError::TopLevelKeyCount);
    }
    Ok(object.into_iter().next().expect("one key"))
}

fn from_payload<T: DeserializeOwned>(message: &str, payload: Value) -> Result<T, ProtocolError> {
    serde_json::from_value(payload).map_err(|source| ProtocolError::Payload {
        message: message.to_owned(),
        source,
    })
}
