//! External, fail-closed deployment selector for the Tekes macOS service.
//!
//! This crate intentionally has no dependency on supervisor or worker code.
//! It owns only installed bundle selection, deployment observation and the
//! signed installer's durable transaction substrate.

mod canary;
mod cli;
mod error;
mod fs;
mod installer;
mod model;
mod selector;
mod signature;

pub use cli::{Command, parse_args, run_command};
pub use error::{ErrorCode, SelectorError};
pub use installer::{
    INSTALLER_PHASES, InstallRequest, InstallerEffects, InstallerOperation, InstallerOperationType,
    InstallerStateMachine, durable_credential_file_for_test, random_credential,
};
pub use model::*;
pub use selector::{
    EMBEDDED_AUTHORITY_REGISTRY_SHA256, EMBEDDED_SELECTOR_CONFORMANCE_SHA256, FailureDisposition,
    Selector, SelectorPaths, automatic_rollback_sha256, cli_command_sha256, describe_conformance,
    reply_bytes,
};
pub use signature::{CodeSignature, CodeSignatureVerifier, MacOsCodeSignatureVerifier};
