//! Versioned continuation request identity and the `tool_continuation` /
//! `tool_continuation_result` wire pair that carries it between a worker and
//! its supervisor once a tool-control result reported a pending continuation.
use crate::durable::ToolControl;
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DOMAIN: &[u8] = b"tekes-tool-continuation-v1\0";
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuationOperation {
    Query,
    Update { input_responses: IJsonValue },
    Cancel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolContinuationRequest {
    pub format: u64,
    pub request_id: String,
    pub original: ToolControl,
    pub continuation_id: String,
    pub step: u64,
    pub action: ContinuationOperation,
}

impl ToolContinuationRequest {
    pub fn new(
        original: ToolControl,
        continuation_id: String,
        step: u64,
        action: ContinuationOperation,
    ) -> Result<Self, String> {
        let mut request = Self {
            format: 1,
            request_id: String::new(),
            original,
            continuation_id,
            step,
            action,
        };
        request.validate_identity()?;
        request.request_id = request.derived_id()?;
        Ok(request)
    }
    fn validate_identity(&self) -> Result<(), String> {
        self.original
            .validate()
            .map_err(|error| error.to_string())?;
        if self.format != 1 || self.step == 0 || self.step > MAX_SAFE_INTEGER {
            return Err("invalid continuation format or step".into());
        }
        if self.continuation_id.len() != 64
            || !self
                .continuation_id
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err("invalid host continuation identity".into());
        }
        Ok(())
    }
    fn derived_id(&self) -> Result<String, String> {
        let preimage = serde_json_canonicalizer::to_vec(&(
            &self.original.request_id,
            &self.continuation_id,
            self.step,
        ))
        .map_err(|error| error.to_string())?;
        let mut digest = Sha256::new();
        digest.update(DOMAIN);
        digest.update(preimage);
        Ok(format!("{:x}", digest.finalize()))
    }
    pub fn validate(&self) -> Result<(), String> {
        self.validate_identity()?;
        if self.request_id != self.derived_id()? {
            return Err("continuation request identity mismatch".into());
        }
        Ok(())
    }
    /// Receipt identity is stable for a step. Changed actions or arguments must
    /// conflict with the original stored bytes, not create another operation.
    pub fn matches_receipt(&self, stored: &Self) -> Result<bool, String> {
        self.validate()?;
        stored.validate()?;
        Ok(self == stored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn original() -> ToolControl {
        ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn step_identity_is_stable_but_changed_action_is_not_an_exact_retry() {
        let first = ToolContinuationRequest::new(
            original(),
            "a".repeat(64),
            1,
            ContinuationOperation::Query,
        )
        .unwrap();
        let retry: ToolContinuationRequest =
            serde_json::from_slice(&serde_json::to_vec(&first).unwrap()).unwrap();
        assert!(retry.matches_receipt(&first).unwrap());
        assert_ne!(first.request_id, first.original.request_id);
        let changed = ToolContinuationRequest::new(
            original(),
            "a".repeat(64),
            1,
            ContinuationOperation::Cancel,
        )
        .unwrap();
        assert_eq!(changed.request_id, first.request_id);
        assert!(!changed.matches_receipt(&first).unwrap());
        let next = ToolContinuationRequest::new(
            original(),
            "a".repeat(64),
            2,
            ContinuationOperation::Query,
        )
        .unwrap();
        assert_ne!(first.request_id, next.request_id);
        let mut changed_arguments = first.clone();
        changed_arguments.original.arguments =
            IJsonValue::parse_str(r#"{"changed":true}"#).unwrap();
        assert!(!changed_arguments.matches_receipt(&first).unwrap());
        let mut forged = first.clone();
        forged.step = 2;
        assert!(forged.validate().is_err());
        for step in [0, MAX_SAFE_INTEGER + 1] {
            assert!(
                ToolContinuationRequest::new(
                    original(),
                    "a".repeat(64),
                    step,
                    ContinuationOperation::Query
                )
                .is_err()
            );
        }
    }
}

/// Host-owned outcome; pending state cannot be confused with tool result JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum ToolContinuationOutcome {
    Pending {
        continuation_id: String,
        next_step: u64,
        state: IJsonValue,
    },
    Completed {
        value: IJsonValue,
    },
    Failed {
        error: crate::durable::ToolControlError,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolContinuationResponse {
    pub request_id: String,
    pub call_id: String,
    pub result: ToolContinuationOutcome,
}

impl ToolContinuationResponse {
    pub fn validate_for(&self, request: &ToolContinuationRequest) -> Result<(), String> {
        request.validate()?;
        if self.request_id != request.request_id || self.call_id != request.original.call_id {
            return Err("continuation response binding mismatch".into());
        }
        if let ToolContinuationOutcome::Pending {
            continuation_id,
            next_step,
            ..
        } = &self.result
        {
            if continuation_id != &request.continuation_id
                || *next_step > MAX_SAFE_INTEGER
                || *next_step != request.step + 1
            {
                return Err("pending continuation identity or sequence mismatch".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod outcome_tests {
    use super::*;
    #[test]
    fn pending_outcomes_bind_identity_and_advance_exactly_one_step() {
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        let request =
            ToolContinuationRequest::new(original, "a".repeat(64), 1, ContinuationOperation::Query)
                .unwrap();
        let response = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: request.original.call_id.clone(),
            result: ToolContinuationOutcome::Pending {
                continuation_id: request.continuation_id.clone(),
                next_step: 2,
                state: IJsonValue::parse_str(r#"{"taskId":"remote-1","status":"working"}"#)
                    .unwrap(),
            },
        };
        response.validate_for(&request).unwrap();
        let bytes = serde_json::to_vec(&response).unwrap();
        let replay: ToolContinuationResponse = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(replay, response);
        for (id, step) in [
            ("b".repeat(64), 2),
            ("a".repeat(64), 1),
            ("a".repeat(64), 3),
        ] {
            let mut changed = response.clone();
            if let ToolContinuationOutcome::Pending {
                continuation_id,
                next_step,
                ..
            } = &mut changed.result
            {
                *continuation_id = id;
                *next_step = step;
            }
            assert!(changed.validate_for(&request).is_err());
        }
        let mut wrong_call = response.clone();
        wrong_call.call_id = "other".into();
        assert!(wrong_call.validate_for(&request).is_err());
        let mut forged: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        forged["result"]["value"] = serde_json::json!({"pretend":"completed"});
        assert!(serde_json::from_value::<ToolContinuationResponse>(forged).is_err());
    }
}

/// Wire key of the worker → supervisor continuation step request.
pub const TOOL_CONTINUATION_KEY: &str = "tool_continuation";
/// Wire key of the supervisor → worker continuation step response.
pub const TOOL_CONTINUATION_RESULT_KEY: &str = "tool_continuation_result";

pub fn encode_tool_continuation(
    request: &ToolContinuationRequest,
) -> Result<Vec<u8>, crate::ProtocolError> {
    request
        .validate()
        .map_err(crate::ProtocolError::InvalidDurableControl)?;
    crate::encode_line(TOOL_CONTINUATION_KEY, request)
}

pub fn decode_tool_continuation(
    line: &[u8],
) -> Result<ToolContinuationRequest, crate::ProtocolError> {
    let request: ToolContinuationRequest = crate::decode_named(line, TOOL_CONTINUATION_KEY)?;
    request
        .validate()
        .map_err(crate::ProtocolError::InvalidDurableControl)?;
    Ok(request)
}

pub fn encode_tool_continuation_result(
    response: &ToolContinuationResponse,
) -> Result<Vec<u8>, crate::ProtocolError> {
    crate::encode_line(TOOL_CONTINUATION_RESULT_KEY, response)
}

pub fn decode_tool_continuation_result(
    line: &[u8],
) -> Result<ToolContinuationResponse, crate::ProtocolError> {
    crate::decode_named(line, TOOL_CONTINUATION_RESULT_KEY)
}

#[cfg(test)]
mod wire_tests {
    use super::*;
    #[test]
    fn continuation_wire_pair_round_trips_and_rejects_foreign_keys() {
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        let request =
            ToolContinuationRequest::new(original, "a".repeat(64), 1, ContinuationOperation::Query)
                .unwrap();
        let line = encode_tool_continuation(&request).unwrap();
        assert!(line.starts_with(b"{\"tool_continuation\":"));
        assert_eq!(decode_tool_continuation(&line).unwrap(), request);
        assert!(
            decode_tool_continuation_result(&line).is_err(),
            "a request line is not a result line"
        );
        let response = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: "call-1".into(),
            result: ToolContinuationOutcome::Completed {
                value: IJsonValue::parse_str(r#"{"content":[]}"#).unwrap(),
            },
        };
        let line = encode_tool_continuation_result(&response).unwrap();
        assert_eq!(decode_tool_continuation_result(&line).unwrap(), response);
        assert!(decode_tool_continuation(&line).is_err());
    }
}
