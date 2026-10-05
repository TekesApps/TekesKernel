use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::session_event_registry_generated::{
    is_surface_event_type, session_event_registry_entry,
};

pub const MAX_SAFE_SEQUENCE: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum SurfaceOperation {
    Append(String),
    Replace { op: String, start: u64, end: u64 },
}

impl SurfaceOperation {
    pub fn validate(&self) -> Result<(), EndpointTypeError> {
        match self {
            Self::Append(value) if value == "append" => Ok(()),
            Self::Append(value) => Err(EndpointTypeError::SurfaceOperation(value.clone())),
            Self::Replace { op, start, end } if op == "replace" && start <= end => Ok(()),
            Self::Replace { op, .. } => Err(EndpointTypeError::SurfaceOperation(op.clone())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub seq: u64,
    pub time: f64,
    pub data: IJsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignorable: Option<bool>,
    #[serde(rename = "sourceEventSeqs", skip_serializing_if = "Option::is_none")]
    pub source_event_seqs: Option<Vec<u64>>,
    #[serde(rename = "surfaceOp", skip_serializing_if = "Option::is_none")]
    pub surface_op: Option<SurfaceOperation>,
}

impl SessionEvent {
    pub fn validate(&self) -> Result<(), EndpointTypeError> {
        if self.seq > MAX_SAFE_SEQUENCE {
            return Err(EndpointTypeError::Sequence(self.seq));
        }
        if !self.time.is_finite()
            || self.time < 0.0
            || self.time.fract() != 0.0
            || self.time > MAX_SAFE_SEQUENCE as f64
        {
            return Err(EndpointTypeError::Time);
        }
        if self.ignorable == Some(false) {
            return Err(EndpointTypeError::IgnorableFalse);
        }
        if session_event_registry_entry(&self.event_type).is_none() && self.ignorable != Some(true)
        {
            return Err(EndpointTypeError::UnknownRequired(self.event_type.clone()));
        }
        let surface = is_surface_event_type(&self.event_type);
        if !surface && (self.source_event_seqs.is_some() || self.surface_op.is_some()) {
            return Err(EndpointTypeError::SurfaceMetadata(self.event_type.clone()));
        }
        if let Some(operation) = &self.surface_op {
            operation.validate()?;
            if matches!(operation, SurfaceOperation::Replace { end, .. } if *end >= self.seq) {
                return Err(EndpointTypeError::SurfaceRange(self.seq));
            }
        }
        let mut previous = None;
        for source in self.source_event_seqs.as_deref().unwrap_or_default() {
            if *source >= self.seq || previous.is_some_and(|value| value >= *source) {
                return Err(EndpointTypeError::SourceSequence(*source));
            }
            previous = Some(*source);
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EndpointTypeError> {
        serde_json_canonicalizer::to_vec(self)
            .map_err(|error| EndpointTypeError::Canonical(error.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionToolEventView {
    #[serde(rename = "for")]
    pub kind: String,
    pub view: IJsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionHistoryEntry {
    pub event: SessionEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<SessionToolEventView>,
}

#[derive(Debug, Error)]
pub enum EndpointTypeError {
    #[error("invalid SessionEvent sequence {0}")]
    Sequence(u64),
    #[error("invalid SessionEvent time")]
    Time,
    #[error("ignorable may only be present as true")]
    IgnorableFalse,
    #[error("surface metadata is forbidden on {0}")]
    SurfaceMetadata(String),
    #[error("invalid surface operation {0}")]
    SurfaceOperation(String),
    #[error("invalid source event sequence {0}")]
    SourceSequence(u64),
    #[error("surface replacement must end before event seq {0}")]
    SurfaceRange(u64),
    #[error("canonical JSON failed: {0}")]
    Canonical(String),
    #[error("unknown required SessionEvent {0}")]
    UnknownRequired(String),
    #[error("native session id is not a canonical UUID")]
    SessionId,
}

pub fn validate_session_id(value: &str) -> Result<(), EndpointTypeError> {
    let uuid = uuid::Uuid::parse_str(value).map_err(|_| EndpointTypeError::SessionId)?;
    if uuid.hyphenated().to_string() != value || value != value.to_ascii_lowercase() {
        return Err(EndpointTypeError::SessionId);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsh_state_only_events_are_required_contract_members() {
        for (seq, event_type) in [
            "model/selection",
            "session-log-deepseek/delivery-accepted",
            "subagent/model-selection-policy",
        ]
        .into_iter()
        .enumerate()
        {
            SessionEvent {
                event_type: event_type.to_owned(),
                seq: seq as u64,
                time: 1.0,
                data: IJsonValue::parse_str("{}").expect("data"),
                ignorable: None,
                source_event_seqs: None,
                surface_op: None,
            }
            .validate()
            .expect("registered DSH state-only event");
        }
    }
}
