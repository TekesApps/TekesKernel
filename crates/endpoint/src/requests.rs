//! Answerable requests (approval/question holds) derived from the semantic
//! ledger. There is no separate request journal: a pending request is an
//! `approval_request` hold without a later resolution on its line, its
//! identity is derived from the hold's causal seq, and its resolution is the
//! ledger's own `approval_response` (or the aborting `tool_result`).
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{ServerRequest, StreamChannel, validate_session_id};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum RequestFrameType {
    #[serde(rename = "approval/requested")]
    Approval,
    #[serde(rename = "question/requested")]
    Question,
}

impl RequestFrameType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approval => "approval/requested",
            Self::Question => "question/requested",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResolutionOutcome {
    AllowedOnce,
    Rejected,
    Cancelled,
    Answered,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingRequest {
    pub rpc_id: String,
    pub session_id: String,
    pub source_line: Option<String>,
    pub frame_type: RequestFrameType,
    pub causal_kernel_seq: u64,
    pub envelope: ServerRequest,
}

/// One request and, when the ledger already carries it, the semantic seq and
/// outcome that resolved it.
#[derive(Clone, Debug, PartialEq)]
pub struct RequestState {
    pub request: PendingRequest,
    pub resolution: Option<(u64, ResolutionOutcome)>,
}

impl PendingRequest {
    /// Builds the request for one hold: the rpc id is derived from the
    /// session, source line, frame type and the hold's causal seq, and the
    /// envelope is validated against that identity.
    pub fn derive(
        session_id: &str,
        source_line: Option<&str>,
        frame_type: RequestFrameType,
        causal_kernel_seq: u64,
        payload: IJsonValue,
    ) -> Result<Self, RequestError> {
        validate_session_id(session_id)?;
        validate_source_line(source_line)?;
        if causal_kernel_seq == 0 {
            return Err(RequestError::CausalSequence);
        }
        let rpc_id =
            derive_line_request_rpc_id(session_id, source_line, frame_type, causal_kernel_seq);
        let envelope = ServerRequest {
            envelope_type: "server-request".to_owned(),
            rpc_id: rpc_id.clone(),
            method: frame_type.as_str().to_owned(),
            payload,
        };
        validate_request_envelope(session_id, frame_type, &envelope)?;
        Ok(Self {
            rpc_id,
            session_id: session_id.to_owned(),
            source_line: source_line.map(str::to_owned),
            frame_type,
            causal_kernel_seq,
            envelope,
        })
    }
}

impl RequestState {
    /// Attaches the ledger's resolution to a request, checking that the
    /// outcome is legal for the frame type and later than the hold.
    pub fn resolved(
        request: PendingRequest,
        causal_kernel_seq: u64,
        outcome: ResolutionOutcome,
    ) -> Result<Self, RequestError> {
        validate_resolution(&request, causal_kernel_seq, outcome)?;
        Ok(Self {
            request,
            resolution: Some((causal_kernel_seq, outcome)),
        })
    }
}

#[must_use]
pub fn derive_request_rpc_id(
    session_id: &str,
    frame_type: RequestFrameType,
    causal_kernel_seq: u64,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(session_id.as_bytes());
    hasher.update([0]);
    hasher.update(frame_type.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(causal_kernel_seq.to_string().as_bytes());
    format!("request-{:x}", hasher.finalize())
}

/// A child-line request hashes the NUL-delimited components `child-request`,
/// root session, source line, frame type and decimal causal seq.
#[must_use]
pub fn derive_line_request_rpc_id(
    session: &str,
    line: Option<&str>,
    kind: RequestFrameType,
    seq: u64,
) -> String {
    let Some(line) = line else {
        return derive_request_rpc_id(session, kind, seq);
    };
    let mut hasher = Sha256::new();
    for component in [
        "child-request",
        session,
        line,
        kind.as_str(),
        &seq.to_string(),
    ] {
        hasher.update(component.as_bytes());
        hasher.update([0]);
    }
    format!("request-{:x}", hasher.finalize())
}

fn validate_source_line(line: Option<&str>) -> Result<(), RequestError> {
    if let Some(line) = line {
        let id = line
            .strip_suffix(".jsonl")
            .ok_or_else(|| RequestError::Corruption("invalid child ledger name".into()))?;
        validate_session_id(id)?;
    }
    Ok(())
}

fn validate_request_envelope(
    session_id: &str,
    frame_type: RequestFrameType,
    envelope: &ServerRequest,
) -> Result<(), RequestError> {
    envelope.validate_stream(StreamChannel::Mux)?;
    if envelope.method != frame_type.as_str() {
        return Err(RequestError::Corruption(
            "request envelope method/frame mismatch".to_owned(),
        ));
    }
    let payload: serde_json::Value = serde_json::from_slice(&envelope.payload.canonical_bytes()?)?;
    if payload.get("type").and_then(serde_json::Value::as_str) != Some(frame_type.as_str())
        || payload.get("sessionId").and_then(serde_json::Value::as_str) != Some(session_id)
    {
        return Err(RequestError::Corruption(
            "request envelope payload identity mismatch".to_owned(),
        ));
    }
    Ok(())
}

fn validate_resolution(
    request: &PendingRequest,
    causal_kernel_seq: u64,
    outcome: ResolutionOutcome,
) -> Result<(), RequestError> {
    if causal_kernel_seq <= request.causal_kernel_seq {
        return Err(RequestError::CausalSequence);
    }
    let allowed = matches!(
        (request.frame_type, outcome),
        (
            RequestFrameType::Approval,
            ResolutionOutcome::AllowedOnce
                | ResolutionOutcome::Rejected
                | ResolutionOutcome::Cancelled
        ) | (
            RequestFrameType::Question,
            ResolutionOutcome::Answered | ResolutionOutcome::Cancelled
        )
    );
    if !allowed {
        return Err(RequestError::OutcomeMismatch);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("request JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("request schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("request envelope is invalid: {0}")]
    Request(#[from] crate::rpc::RequestError),
    #[error("request identity is invalid: {0}")]
    Type(#[from] crate::types::EndpointTypeError),
    #[error("request causal sequence is invalid")]
    CausalSequence,
    #[error("request outcome does not match its frame type")]
    OutcomeMismatch,
    #[error("request derivation failed: {0}")]
    Corruption(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const SESSION: &str = "018f0000-0000-7000-8000-000000000001";

    fn payload() -> IJsonValue {
        IJsonValue::parse_str(&format!(
            r#"{{"approvalId":"approval:c","callId":"c","sessionId":"{SESSION}","toolName":"shell","type":"approval/requested"}}"#
        ))
        .unwrap()
    }

    #[test]
    fn request_identity_is_derived_from_the_hold_and_its_line() {
        let root = PendingRequest::derive(SESSION, None, RequestFrameType::Approval, 9, payload())
            .unwrap();
        assert_eq!(
            root.rpc_id,
            derive_request_rpc_id(SESSION, RequestFrameType::Approval, 9)
        );
        let child_line = "018f0000-0000-7000-8000-000000000002.jsonl";
        let child = PendingRequest::derive(
            SESSION,
            Some(child_line),
            RequestFrameType::Approval,
            9,
            payload(),
        )
        .unwrap();
        assert_ne!(child.rpc_id, root.rpc_id);
        assert_eq!(child.source_line.as_deref(), Some(child_line));
        for line in ["../main.jsonl", "main.jsonl", "bad.jsonl"] {
            assert!(
                PendingRequest::derive(
                    SESSION,
                    Some(line),
                    RequestFrameType::Approval,
                    9,
                    payload()
                )
                .is_err()
            );
        }
        assert!(
            PendingRequest::derive(SESSION, None, RequestFrameType::Approval, 0, payload())
                .is_err()
        );
        assert!(
            PendingRequest::derive(SESSION, None, RequestFrameType::Question, 9, payload())
                .is_err()
        );
    }

    #[test]
    fn resolution_must_follow_the_hold_and_match_the_frame_type() {
        let request =
            PendingRequest::derive(SESSION, None, RequestFrameType::Approval, 9, payload())
                .unwrap();
        assert!(
            RequestState::resolved(request.clone(), 9, ResolutionOutcome::AllowedOnce).is_err()
        );
        assert!(RequestState::resolved(request.clone(), 10, ResolutionOutcome::Answered).is_err());
        let state = RequestState::resolved(request, 10, ResolutionOutcome::Rejected).unwrap();
        assert_eq!(state.resolution, Some((10, ResolutionOutcome::Rejected)));
    }
}
