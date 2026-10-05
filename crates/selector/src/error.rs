use std::io;

use serde::Serialize;
use serde_json::{Value, json};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    Usage,
    InvalidBundle,
    InvalidState,
    Unavailable,
    Io,
    Corruption,
}

impl ErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::InvalidBundle => "invalid-bundle",
            Self::InvalidState => "invalid-state",
            Self::Unavailable => "unavailable",
            Self::Io => "io",
            Self::Corruption => "corruption",
        }
    }

    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::Usage => "Command line is invalid",
            Self::InvalidBundle => "Bundle is invalid",
            Self::InvalidState => "Selector state does not permit the operation",
            Self::Unavailable => "Selected build is unavailable",
            Self::Io => "Selector I/O failed",
            Self::Corruption => "Selector state is corrupt",
        }
    }

    #[must_use]
    pub const fn exit_code(self) -> i32 {
        match self {
            Self::Usage => 64,
            Self::InvalidBundle => 65,
            Self::InvalidState => 66,
            Self::Unavailable => 69,
            Self::Io => 74,
            Self::Corruption => 75,
        }
    }
}

#[derive(Debug, Error)]
#[error("{code:?}: {detail}")]
pub struct SelectorError {
    pub code: ErrorCode,
    pub details: Value,
    detail: String,
    #[source]
    source: Option<io::Error>,
}

impl SelectorError {
    #[must_use]
    pub fn new(code: ErrorCode, details: Value) -> Self {
        Self {
            code,
            detail: code.message().to_owned(),
            details,
            source: None,
        }
    }

    #[must_use]
    pub fn usage(argument: impl Into<String>) -> Self {
        Self::new(ErrorCode::Usage, json!({"argument": argument.into()}))
    }

    #[must_use]
    pub fn invalid_bundle(path: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidBundle, json!({"path": path.into()}))
    }

    #[must_use]
    pub fn invalid_state(state: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidState, json!({"state": state.into()}))
    }

    #[must_use]
    pub fn unavailable(code: impl Into<String>, version: impl Into<String>) -> Self {
        Self::new(
            ErrorCode::Unavailable,
            json!({"code": code.into(), "version": version.into()}),
        )
    }

    #[must_use]
    pub fn corruption(path: impl Into<String>) -> Self {
        Self::new(ErrorCode::Corruption, json!({"path": path.into()}))
    }

    #[must_use]
    pub fn io(operation: impl Into<String>, source: io::Error) -> Self {
        Self {
            code: ErrorCode::Io,
            detail: ErrorCode::Io.message().to_owned(),
            details: json!({"operation": operation.into()}),
            source: Some(source),
        }
    }

    pub fn envelope_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        #[derive(Serialize)]
        struct ErrorBody<'a> {
            code: &'a str,
            details: &'a Value,
            message: &'a str,
        }
        #[derive(Serialize)]
        struct Envelope<'a> {
            error: ErrorBody<'a>,
        }
        let mut bytes = serde_json_canonicalizer::to_vec(&Envelope {
            error: ErrorBody {
                code: self.code.as_str(),
                details: &self.details,
                message: self.code.message(),
            },
        })?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

pub(crate) trait IoContext<T> {
    fn selector_io(self, operation: &'static str) -> Result<T, SelectorError>;
}

impl<T> IoContext<T> for io::Result<T> {
    fn selector_io(self, operation: &'static str) -> Result<T, SelectorError> {
        self.map_err(|error| SelectorError::io(operation, error))
    }
}
