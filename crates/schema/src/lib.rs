//! TekesKernel's language-owned event and canonical JSON contract.

mod event;
mod execution;
mod fold;
mod ijson;
pub mod metrics;
mod types;

pub use event::{Event, EventKind, Visibility};
pub use execution::{ExecutionConfig, McpConnection, McpConnectionTransport};
pub use fold::{
    HistoricalLedger, LedgerProjection, LedgerValidator, LifecycleFacts, RunRange,
    validate_historical_ledger, validate_ledger,
};
pub use ijson::IJsonValue;
pub use types::{Block, OriginTuple, ResumePolicy, SeqRange};

/// Ledger reader version this build understands. Version 2 adds the genesis
/// `preset` field: a genesis that carries one sets `min_reader` 2, so an older
/// reader opens the session read-only instead of rendering it with the wrong
/// prompt. Version 3 makes genesis `folders` narrow the session's folders: a
/// genesis that records them sets `min_reader` 3, so an older reader does not
/// widen the session to every workspace folder (spec event §Genesis).
pub const READER_VERSION: u64 = 5;

/// Session-owned execution values without Environment or Folder references.
pub const EXECUTION_READER: u64 = 5;

/// Session-owned creation seeds and subsequent per-session folder edits.
pub const SESSION_SCOPE_READER: u64 = 4;

/// The reader a genesis that records `folders` requires.
pub const FOLDERS_READER: u64 = 3;

/// Genesis fields a session started from another session's scope (a fork,
/// or a `context` start) copies verbatim: where it runs, with which
/// configuration and preset. Fork additionally carries `format` and remaps
/// `seed` and `parent`; every other field is set by the operation itself.
pub const GENESIS_INHERITED_FIELDS: &[&str] = &[
    "execution",
    "session_scope",
    "workspace",
    "folder_binding",
    "cwd",
    "environment",
    "folders",
    "resume",
    "config",
    "instruction",
    "identity_profile",
    "preset",
];

/// Fold only session scope edits over the original creation record. This is
/// shared by launch, inventory, fork and context-start; no workspace lookup.
pub fn effective_session_genesis(
    events: &[Event],
) -> Result<Option<serde_json::Value>, SchemaError> {
    let Some(first) = events.first() else {
        return Ok(None);
    };
    let mut value = serde_json::to_value(first.raw())?;
    for event in events.iter().skip(1) {
        if event.kind() != &EventKind::Meta {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        if let Some(change) = raw
            .get("scope_change")
            .and_then(serde_json::Value::as_object)
        {
            if value.get("execution").is_some() {
                return Err(SchemaError::event(
                    Some(event.seq()),
                    "execution sessions cannot carry legacy scope changes",
                ));
            }
            for field in [
                "session_scope",
                "folders",
                "folder_binding",
                "cwd",
                "environment",
            ] {
                if let Some(next) = change.get(field) {
                    value[field] = next.clone();
                }
            }
            value["min_reader"] = serde_json::json!(SESSION_SCOPE_READER);
        }
    }
    Ok(Some(value))
}

/// Genesis fields the creating operation always sets for the new ledger.
pub const GENESIS_OPERATION_FIELDS: &[&str] = &[
    "v",
    "seq",
    "ts",
    "kind",
    "thread",
    "format",
    "min_reader",
    "min_writer",
    "origin_key",
    "origin_tuple",
    "ephemeral",
    "seed",
    "parent",
];

/// The environment a session without a genesis `environment` runs in.
pub const DEFAULT_ENVIRONMENT: &str = "local";

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
