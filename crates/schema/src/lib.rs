//! TekesKernel's language-owned event and canonical JSON contract.

mod event;
mod fold;
mod ijson;
mod types;

pub use event::{Event, EventKind, Visibility};
pub use fold::{LedgerProjection, LedgerValidator, LifecycleFacts, RunRange, validate_ledger};
pub use ijson::IJsonValue;
pub use types::{Block, OriginTuple, ResumePolicy, SeqRange};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid canonical JSON: {0}")]
    Canonical(String),
    #[error("invalid event at seq {seq:?}: {message}")]
    Event { seq: Option<u64>, message: String },
    #[error("constraint {rule} rejected seq {seq}: {message}")]
    Constraint { rule: u8, seq: u64, message: String },
}

impl SchemaError {
    pub(crate) fn event(seq: Option<u64>, message: impl Into<String>) -> Self {
        Self::Event {
            seq,
            message: message.into(),
        }
    }

    pub(crate) fn constraint(rule: u8, seq: u64, message: impl Into<String>) -> Self {
        Self::Constraint {
            rule,
            seq,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn rule(&self) -> Option<u8> {
        match self {
            Self::Constraint { rule, .. } => Some(*rule),
            Self::Json(_) | Self::Canonical(_) | Self::Event { .. } => None,
        }
    }
}
