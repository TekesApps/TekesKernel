//! Versioned user configuration and immutable spawn-profile snapshots.

mod config;
mod instruction;
mod launch;
mod resources;

pub use config::{
    ConfigRepository, ConfigSnapshot, Limits, Model, Provider, ProvidersConfig, ResolvedWorkspace,
    RevisionVector, SessionSettings, SettingsConfig, WebSearch, WorkspaceConfig, WorkspaceFolder,
    WorkspacePolicy,
};
pub use instruction::{
    EffectiveInstructions, EffectivePolicy, InstructionKind, InstructionOrigin, InstructionPolicy,
    InstructionResolver, InstructionSettings, InstructionSnapshot, InstructionSource,
    LaunchProfile,
};
pub use launch::{
    DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
    ExternalEffectBinding, ExternalEffectProtocol, LaunchBindings,
};
pub use resources::{
    CommandCatalogEntry, CommandExpansion, CommandSummary, ResourceCatalog, ResourceError,
    SkillPackage, SkillResource, SkillSummary,
};

use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("invalid canonical bytes in {path}: {reason}")]
    InvalidBytes { path: PathBuf, reason: String },
    #[error("invalid schema at {path}: {reason}")]
    InvalidSchema { path: PathBuf, reason: String },
    #[error("invalid path {path}: {reason}")]
    InvalidPath { path: PathBuf, reason: String },
    #[error("invalid reference at {path}: {reason}")]
    InvalidReference { path: PathBuf, reason: String },
    #[error("unsupported format {format} at {path}")]
    UnsupportedFormat { path: PathBuf, format: u64 },
    #[error("stale revision: expected {expected}, actual {actual}")]
    StaleRevision { expected: u64, actual: u64 },
    #[error("instruction source is unstable after {attempts} scans")]
    UnstableSource { attempts: usize },
    #[error("instruction limit exceeded: {0}")]
    LimitExceeded(&'static str),
    #[error("instruction symlink is forbidden: {0}")]
    Symlink(PathBuf),
    #[error("instruction source is not a regular file: {0}")]
    NotRegular(PathBuf),
    #[error("snapshot digest mismatch: expected {expected}, found {actual}")]
    DigestMismatch { expected: String, actual: String },
    #[error(transparent)]
    Store(#[from] store::StoreError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub(crate) fn parse_canonical<T: DeserializeOwned>(
    path: impl Into<PathBuf>,
    bytes: &[u8],
) -> Result<T, ProfileError> {
    let path = path.into();
    if !bytes.ends_with(b"\n") || bytes.len() < 2 || bytes[..bytes.len() - 1].ends_with(b"\n") {
        return Err(ProfileError::InvalidBytes {
            path,
            reason: "expected one canonical JSON object followed by exactly one LF".to_owned(),
        });
    }
    let body = &bytes[..bytes.len() - 1];
    let value = schema::IJsonValue::parse(body).map_err(|error| ProfileError::InvalidBytes {
        path: path.clone(),
        reason: error.to_string(),
    })?;
    let canonical = value
        .canonical_bytes()
        .map_err(|error| ProfileError::InvalidBytes {
            path: path.clone(),
            reason: error.to_string(),
        })?;
    if canonical != body {
        return Err(ProfileError::InvalidBytes {
            path,
            reason: "bytes are not RFC-8785 canonical".to_owned(),
        });
    }
    let json: serde_json::Value =
        serde_json::from_slice(body).map_err(|error| ProfileError::InvalidBytes {
            path: path.clone(),
            reason: error.to_string(),
        })?;
    if contains_null(&json) {
        return Err(ProfileError::InvalidSchema {
            path,
            reason: "JSON null is not a substitute for an absent optional field".to_owned(),
        });
    }
    serde_json::from_value(json).map_err(|error| ProfileError::InvalidSchema {
        path,
        reason: error.to_string(),
    })
}

pub(crate) fn canonical_line<T: Serialize>(
    path: impl Into<PathBuf>,
    value: &T,
) -> Result<Vec<u8>, ProfileError> {
    let path = path.into();
    let mut bytes =
        serde_json_canonicalizer::to_vec(value).map_err(|error| ProfileError::InvalidSchema {
            path,
            reason: error.to_string(),
        })?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn contains_null(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::Array(values) => values.iter().any(contains_null),
        serde_json::Value::Object(values) => values.values().any(contains_null),
        serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => false,
    }
}
