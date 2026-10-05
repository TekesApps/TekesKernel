use schema::IJsonValue;
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    ClientResponse, PendingRequest, RequestError, RequestFrameType, RequestState,
    ResolutionOutcome, RespondReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RespondRejectionReason {
    UnknownRpcId,
    AlreadyResolved,
    ResponseTypeMismatch,
    SessionMismatch,
    Archived,
    StopActive,
}

impl RespondRejectionReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownRpcId => "unknown-rpc-id",
            Self::AlreadyResolved => "already-resolved",
            Self::ResponseTypeMismatch => "response-type-mismatch",
            Self::SessionMismatch => "session-mismatch",
            Self::Archived => "archived",
            Self::StopActive => "stop-active",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RespondPrepareContext {
    pub archived: bool,
    pub stop_active: bool,
    /// Required for question requests because the public requested frame does
    /// not carry the causal held call id.
    pub held_call: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RespondAuthorization {
    pub rpc_id: String,
    pub request_sha256: String,
    pub session_id: String,
    pub call: String,
    pub grant: bool,
    pub answer: Option<IJsonValue>,
    pub origin_key: String,
    pub resolution: ResolutionOutcome,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RespondDecision {
    Reject(RespondReceipt),
    CachedSuccess(RespondReceipt),
    Authorize(RespondAuthorization),
}

pub struct RespondLifecycle;

impl RespondLifecycle {
    /// `state` is the request the caller located for `response.rpc_id`
    /// (derived from the semantic ledger); `None` is the unknown-rpc-id arm.
    pub fn prepare(
        state: Option<&RequestState>,
        response: &ClientResponse,
        context: &RespondPrepareContext,
    ) -> Result<RespondDecision, RespondLifecycleError> {
        response.validate()?;
        if !response.result.ok || response.result.error.is_some() {
            return Err(RespondLifecycleError::Envelope);
        }
        let Some(state) = state else {
            return Ok(reject(RespondRejectionReason::UnknownRpcId));
        };
        if state.rpc_id_mismatch(&response.rpc_id) {
            return Err(RespondLifecycleError::StaleAuthorization);
        }
        if state.resolution.is_some() {
            return Ok(reject(RespondRejectionReason::AlreadyResolved));
        }
        let value = response
            .result
            .value
            .as_ref()
            .ok_or(RespondLifecycleError::Envelope)?;
        let value: Value = serde_json::from_slice(&value.canonical_bytes()?)?;
        let object = value.as_object().ok_or(RespondLifecycleError::Envelope)?;
        let session_id = object
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or(RespondLifecycleError::Envelope)?;
        if session_id != state.request.session_id {
            return Ok(reject(RespondRejectionReason::SessionMismatch));
        }

        let parsed = match state.request.frame_type {
            RequestFrameType::Approval => parse_approval(&state.request, object),
            RequestFrameType::Question => {
                parse_question(&state.request, object, context.held_call.as_deref())
            }
        };
        let (call, grant, answer, resolution) = match parsed {
            Ok(value) => value,
            Err(RespondLifecycleError::ResponseTypeMismatch) => {
                return Ok(reject(RespondRejectionReason::ResponseTypeMismatch));
            }
            Err(error) => return Err(error),
        };
        if context.archived {
            return Ok(reject(RespondRejectionReason::Archived));
        }
        if context.stop_active {
            return Ok(reject(RespondRejectionReason::StopActive));
        }
        let canonical = serde_json_canonicalizer::to_vec(response)
            .map_err(|error| RespondLifecycleError::Canonical(error.to_string()))?;
        Ok(RespondDecision::Authorize(RespondAuthorization {
            rpc_id: response.rpc_id.clone(),
            request_sha256: format!("{:x}", Sha256::digest(canonical)),
            session_id: session_id.to_owned(),
            call,
            grant,
            answer,
            origin_key: format!("{}/response", response.rpc_id),
            resolution,
        }))
    }
}

impl RequestState {
    fn rpc_id_mismatch(&self, rpc_id: &str) -> bool {
        self.request.rpc_id != rpc_id
    }
}

type ParsedResponse = (String, bool, Option<IJsonValue>, ResolutionOutcome);

fn parse_approval(
    request: &PendingRequest,
    value: &serde_json::Map<String, Value>,
) -> Result<ParsedResponse, RespondLifecycleError> {
    if value
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>()
        != ["approvalId", "outcome", "sessionId"].into_iter().collect()
    {
        return Err(RespondLifecycleError::ResponseTypeMismatch);
    }
    let payload: Value = serde_json::from_slice(&request.envelope.payload.canonical_bytes()?)?;
    let expected_approval = payload.get("approvalId").and_then(Value::as_str);
    let approval = value.get("approvalId").and_then(Value::as_str);
    let call = payload.get("callId").and_then(Value::as_str);
    let outcome = value.get("outcome").and_then(Value::as_str);
    if approval != expected_approval || call.is_none() {
        return Err(RespondLifecycleError::ResponseTypeMismatch);
    }
    match outcome {
        Some("allowed-once") => Ok((
            call.expect("checked").to_owned(),
            true,
            None,
            ResolutionOutcome::AllowedOnce,
        )),
        Some("rejected") => Ok((
            call.expect("checked").to_owned(),
            false,
            None,
            ResolutionOutcome::Rejected,
        )),
        _ => Err(RespondLifecycleError::ResponseTypeMismatch),
    }
}

/// A question response is either the answer arm `{sessionId, answer:{answers:[...]}}`
/// or the cancellation arm `{sessionId, cancelQuestion:true}`
/// (`SessionQuestionCancellationEndpoint`). Cancellation authors the keyed
/// `approval_response {grant:false}` for the question's causal held call
/// without an `answer`; the worker closes that call with a denied
/// `tool_result` and the turn continues, so the session is not stopped. The
/// projection reports it as `question/resolved {outcome:"cancelled"}`.
fn parse_question(
    _request: &PendingRequest,
    value: &serde_json::Map<String, Value>,
    held_call: Option<&str>,
) -> Result<ParsedResponse, RespondLifecycleError> {
    let keys = value
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    if keys == ["cancelQuestion", "sessionId"].into_iter().collect() {
        if value.get("cancelQuestion") != Some(&Value::Bool(true)) {
            return Err(RespondLifecycleError::ResponseTypeMismatch);
        }
        let held_call = held_call.ok_or(RespondLifecycleError::MissingHoldBinding)?;
        return Ok((
            held_call.to_owned(),
            false,
            None,
            ResolutionOutcome::Cancelled,
        ));
    }
    if keys != ["answer", "sessionId"].into_iter().collect() {
        return Err(RespondLifecycleError::ResponseTypeMismatch);
    }
    let answer = value
        .get("answer")
        .and_then(Value::as_object)
        .filter(|answer| answer.len() == 1 && answer.get("answers").is_some_and(Value::is_array))
        .ok_or(RespondLifecycleError::ResponseTypeMismatch)?;
    let held_call = held_call.ok_or(RespondLifecycleError::MissingHoldBinding)?;
    Ok((
        held_call.to_owned(),
        true,
        Some(IJsonValue::parse(&serde_json::to_vec(answer)?)?),
        ResolutionOutcome::Answered,
    ))
}

fn reject(reason: RespondRejectionReason) -> RespondDecision {
    RespondDecision::Reject(RespondReceipt {
        accepted: false,
        reason: Some(reason.as_str().to_owned()),
    })
}

#[derive(Debug, Error)]
pub enum RespondLifecycleError {
    #[error("respond envelope must be the closed successful client-response arm")]
    Envelope,
    #[error("respond request does not match the requested frame type")]
    ResponseTypeMismatch,
    #[error("question response lacks a causal held-call binding")]
    MissingHoldBinding,
    #[error("respond authorization became stale before its semantic barrier")]
    StaleAuthorization,
    #[error("respond request validation failed: {0}")]
    Request(#[from] crate::rpc::RequestError),
    #[error("respond request derivation failed: {0}")]
    Derivation(#[from] RequestError),
    #[error("respond JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("respond schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("respond canonicalization failed: {0}")]
    Canonical(String),
}

#[cfg(test)]
mod prepare_tests {
    use super::*;

    #[test]
    fn prepare_rejects_resolved_unknown_and_mismatched_requests() {
        let session = "018f0000-0000-7000-8000-000000000003";
        let request = PendingRequest::derive(session, None, RequestFrameType::Approval, 10,
            IJsonValue::parse_str(&format!(r#"{{"type":"approval/requested","sessionId":"{session}","approvalId":"approval:a","callId":"a","toolName":"apply_patch"}}"#)).unwrap()).unwrap();
        let response = ClientResponse {
            envelope_type: "client-response".into(), rpc_id: request.rpc_id.clone(),
            result: crate::ClientResponseResult { ok: true, error: None, value: Some(
                IJsonValue::parse_str(&format!(r#"{{"approvalId":"approval:a","outcome":"allowed-once","sessionId":"{session}"}}"#)).unwrap()) },
        };
        let context = RespondPrepareContext {
            archived: false,
            stop_active: false,
            held_call: None,
        };
        let pending = RequestState {
            request: request.clone(),
            resolution: None,
        };
        let RespondDecision::Authorize(authorization) =
            RespondLifecycle::prepare(Some(&pending), &response, &context).unwrap()
        else {
            panic!("authorize")
        };
        assert_eq!(authorization.call, "a");
        assert_eq!(authorization.resolution, ResolutionOutcome::AllowedOnce);
        assert_eq!(
            authorization.origin_key,
            format!("{}/response", request.rpc_id)
        );
        let resolved = RequestState::resolved(request, 11, ResolutionOutcome::AllowedOnce).unwrap();
        assert!(
            matches!(RespondLifecycle::prepare(Some(&resolved), &response, &context).unwrap(), RespondDecision::Reject(receipt) if receipt.reason.as_deref() == Some("already-resolved"))
        );
        assert!(
            matches!(RespondLifecycle::prepare(None, &response, &context).unwrap(), RespondDecision::Reject(receipt) if receipt.reason.as_deref() == Some("unknown-rpc-id"))
        );
        let archived = RespondPrepareContext {
            archived: true,
            ..context.clone()
        };
        assert!(
            matches!(RespondLifecycle::prepare(Some(&pending), &response, &archived).unwrap(), RespondDecision::Reject(receipt) if receipt.reason.as_deref() == Some("archived"))
        );
    }

    fn question_fixture(session: &str) -> (RequestState, RespondPrepareContext) {
        let request = PendingRequest::derive(session, None, RequestFrameType::Question, 10,
            IJsonValue::parse_str(&format!(r#"{{"type":"question/requested","sessionId":"{session}","questions":[{{"id":"q1","question":"Continue?"}}]}}"#)).unwrap()).unwrap();
        let context = RespondPrepareContext {
            archived: false,
            stop_active: false,
            held_call: Some("call-hold".into()),
        };
        (
            RequestState {
                request,
                resolution: None,
            },
            context,
        )
    }

    fn question_response(rpc_id: &str, value: &str) -> ClientResponse {
        ClientResponse {
            envelope_type: "client-response".into(),
            rpc_id: rpc_id.to_owned(),
            result: crate::ClientResponseResult {
                ok: true,
                error: None,
                value: Some(IJsonValue::parse_str(value).unwrap()),
            },
        }
    }

    #[test]
    fn question_cancellation_authors_denied_response_without_answer() {
        let session = "018f0000-0000-7000-8000-000000000004";
        let (pending, context) = question_fixture(session);
        let rpc_id = pending.request.rpc_id.clone();
        let cancel = question_response(
            &rpc_id,
            &format!(r#"{{"cancelQuestion":true,"sessionId":"{session}"}}"#),
        );
        let RespondDecision::Authorize(authorization) =
            RespondLifecycle::prepare(Some(&pending), &cancel, &context).unwrap()
        else {
            panic!("authorize")
        };
        assert_eq!(authorization.call, "call-hold");
        assert!(!authorization.grant);
        assert!(authorization.answer.is_none());
        assert_eq!(authorization.resolution, ResolutionOutcome::Cancelled);
        assert_eq!(authorization.origin_key, format!("{rpc_id}/response"));
        assert!(
            RequestState::resolved(pending.request.clone(), 11, ResolutionOutcome::Cancelled)
                .is_ok()
        );

        // Only the literal `true` cancels; anything else is a type mismatch, not an answer.
        for wrong in [
            format!(r#"{{"cancelQuestion":false,"sessionId":"{session}"}}"#),
            format!(
                r#"{{"cancelQuestion":true,"answer":{{"answers":[]}},"sessionId":"{session}"}}"#
            ),
        ] {
            assert!(
                matches!(RespondLifecycle::prepare(Some(&pending), &question_response(&rpc_id, &wrong), &context).unwrap(), RespondDecision::Reject(receipt) if receipt.reason.as_deref() == Some("response-type-mismatch"))
            );
        }
        // Answers still authorize a granted response.
        let answer = question_response(
            &rpc_id,
            &format!(r#"{{"answer":{{"answers":["yes"]}},"sessionId":"{session}"}}"#),
        );
        let RespondDecision::Authorize(answered) =
            RespondLifecycle::prepare(Some(&pending), &answer, &context).unwrap()
        else {
            panic!("authorize")
        };
        assert!(answered.grant && answered.answer.is_some());
        assert_eq!(answered.resolution, ResolutionOutcome::Answered);
        // Without the hold binding cancellation fails closed.
        let unbound = RespondPrepareContext {
            held_call: None,
            ..context
        };
        assert!(matches!(
            RespondLifecycle::prepare(Some(&pending), &cancel, &unbound),
            Err(RespondLifecycleError::MissingHoldBinding)
        ));
    }
}
