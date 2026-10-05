use std::collections::{BTreeSet, VecDeque};

use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MAX_RPC_ID_BYTES: usize = 128;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientRequest {
    #[serde(rename = "type")]
    pub envelope_type: String,
    #[serde(rename = "rpcId")]
    pub rpc_id: String,
    pub method: String,
    pub payload: IJsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcError {
    pub code: String,
    pub message: String,
    pub details: IJsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<IJsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl RpcResult {
    pub fn validate(&self) -> Result<(), RequestError> {
        match (self.ok, self.value.is_some(), self.error.is_some()) {
            (true, true, false) | (false, false, true) => Ok(()),
            _ => Err(RequestError::Result),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerResponse {
    #[serde(rename = "type")]
    pub envelope_type: String,
    #[serde(rename = "rpcId")]
    pub rpc_id: String,
    pub result: RpcResult,
}

/// Carrier-neutral form of a server-originated stream/request envelope.
/// The transport is responsible only for encoding this value on its chosen
/// carrier; it must not reinterpret `payload`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerRequest {
    #[serde(rename = "type")]
    pub envelope_type: String,
    #[serde(rename = "rpcId")]
    pub rpc_id: String,
    pub method: String,
    pub payload: IJsonValue,
}

impl ServerRequest {
    pub fn validate(&self) -> Result<(), RequestError> {
        if self.envelope_type != "server-request" {
            return Err(RequestError::ServerEnvelope);
        }
        validate_rpc_id(&self.rpc_id)?;
        if self.method.is_empty() {
            return Err(RequestError::Method);
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, RequestError> {
        self.validate()?;
        serde_json_canonicalizer::to_vec(self)
            .map_err(|error| RequestError::Canonical(error.to_string()))
    }

    pub fn validate_stream(&self, channel: crate::StreamChannel) -> Result<(), RequestError> {
        self.validate()?;
        let payload = self
            .payload
            .canonical_bytes()
            .map_err(|error| RequestError::Canonical(error.to_string()))?;
        let value: serde_json::Value = serde_json::from_slice(&payload)
            .map_err(|error| RequestError::Canonical(error.to_string()))?;
        let frame_type = value
            .as_object()
            .and_then(|object| object.get("type"))
            .and_then(serde_json::Value::as_str)
            .ok_or(RequestError::MissingFrameType)?;
        if self.method != frame_type {
            return Err(RequestError::StreamMethod);
        }
        if !channel.frame_types().contains(&frame_type) {
            return Err(RequestError::UnknownRequiredFrame(frame_type.to_owned()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientResponse {
    #[serde(rename = "type")]
    pub envelope_type: String,
    #[serde(rename = "rpcId")]
    pub rpc_id: String,
    pub result: ClientResponseResult,
}

impl ClientResponse {
    pub fn validate(&self) -> Result<(), RequestError> {
        if self.envelope_type != "client-response" {
            return Err(RequestError::ClientResponseEnvelope);
        }
        validate_rpc_id(&self.rpc_id)?;
        self.result.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientResponseResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<IJsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl ClientResponseResult {
    pub fn validate(&self) -> Result<(), RequestError> {
        match (self.ok, self.value.is_some(), self.error.is_some()) {
            (true, true, false) | (false, false, true) => Ok(()),
            _ => Err(RequestError::Result),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RespondReceipt {
    pub accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

pub fn validate_response(
    response: &ServerResponse,
    expected_rpc_id: &str,
) -> Result<(), RequestError> {
    if response.envelope_type != "server-response" {
        return Err(RequestError::ResponseEnvelope);
    }
    if validate_rpc_id(&response.rpc_id).is_err() || response.rpc_id != expected_rpc_id {
        return Err(RequestError::ResponseRpcId);
    }
    response.result.validate()
}

pub fn validate_request(
    request: &ClientRequest,
    capabilities: &BTreeSet<&str>,
) -> Result<(), RequestError> {
    if request.envelope_type != "client-request" {
        return Err(RequestError::Envelope);
    }
    validate_rpc_id(&request.rpc_id)?;
    if !capabilities.contains(request.method.as_str()) {
        return Err(RequestError::Unsupported(request.method.clone()));
    }
    Ok(())
}

pub fn validate_rpc_id(rpc_id: &str) -> Result<(), RequestError> {
    if rpc_id.is_empty() || rpc_id.len() > MAX_RPC_ID_BYTES {
        return Err(RequestError::RpcId);
    }
    Ok(())
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RequestError {
    #[error("request envelope type must be client-request")]
    Envelope,
    #[error("response envelope type must be server-response")]
    ResponseEnvelope,
    #[error("response envelope type must be client-response")]
    ClientResponseEnvelope,
    #[error("server envelope type must be server-request")]
    ServerEnvelope,
    #[error("rpcId must contain 1..=128 UTF-8 bytes")]
    RpcId,
    #[error("method must not be empty")]
    Method,
    #[error("response rpcId is empty or does not match its request")]
    ResponseRpcId,
    #[error("unsupported capability {0}")]
    Unsupported(String),
    #[error("RPC result must contain exactly value or error according to ok")]
    Result,
    #[error("canonical JSON failed: {0}")]
    Canonical(String),
    #[error("stream envelope method differs from its channel")]
    StreamMethod,
    #[error("stream payload has no string type")]
    MissingFrameType,
    #[error("unknown required stream frame {0}")]
    UnknownRequiredFrame(String),
}

#[derive(Clone, Debug)]
pub struct MuxBuffer {
    frames: VecDeque<Vec<u8>>,
    bytes: usize,
    max_frames: usize,
    max_bytes: usize,
}

impl Default for MuxBuffer {
    fn default() -> Self {
        Self::new(4_096, 4 * 1024 * 1024)
    }
}

impl MuxBuffer {
    #[must_use]
    pub fn new(max_frames: usize, max_bytes: usize) -> Self {
        Self {
            frames: VecDeque::new(),
            bytes: 0,
            max_frames,
            max_bytes,
        }
    }

    pub fn push(&mut self, frame: Vec<u8>) -> Result<(), MuxBufferError> {
        if self.frames.len() == self.max_frames
            || self
                .bytes
                .checked_add(frame.len())
                .is_none_or(|bytes| bytes > self.max_bytes)
        {
            return Err(MuxBufferError::LiveGap);
        }
        self.bytes += frame.len();
        self.frames.push_back(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let frame = self.frames.pop_front()?;
        self.bytes -= frame.len();
        Some(frame)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum MuxBufferError {
    #[error("live-gap")]
    LiveGap,
}
