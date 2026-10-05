//! Durable, append-only JSONL storage and content-addressed assets.

mod asset;
mod atomic;
mod folder;
mod management_root;
mod platform;
mod rewrite;
mod tail;

pub use asset::{AssetRef, AssetStore};
pub use atomic::AtomicPublisher;
pub use folder::{AppendOutcome, ConditionalAppendOutcome, CreateOutcome, ThreadStore};
pub use management_root::{
    ENDPOINT_MANAGEMENT_DIR, SESSION_SETTINGS_FILE, endpoint_management_root, retire_legacy_name,
    session_settings_path,
};
pub use platform::{
    DirectoryLock, FullSync, NamedLock, RootLock, SyncPolicy, SystemSync, probe_local_filesystem,
};
pub use rewrite::{
    ForkGenesisBinding, RedactionInventory, RewriteEventRef, RewriteFileMap, RewriteKind,
    RewriteOperation, RewritePhase, RewriteProgress,
};
pub use tail::{LockedLedger, TailScan, scan_valid_prefix};

use std::io;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BarrierContext {
    pub side_effectful_tool_call: bool,
}

#[must_use]
pub fn requires_barrier(event: &schema::Event, context: BarrierContext) -> bool {
    use schema::EventKind;

    match event.kind() {
        EventKind::Genesis
        | EventKind::RunStart
        | EventKind::Input
        | EventKind::QueueEdit
        | EventKind::Output
        | EventKind::EffectiveExecution
        | EventKind::ToolExecutionStarted
        | EventKind::ApprovalRequest
        | EventKind::ApprovalResponse
        | EventKind::Spawn
        | EventKind::Attempt
        | EventKind::AttemptDispatched
        | EventKind::AttemptRecovery
        | EventKind::Settle
        | EventKind::StopRequested => true,
        EventKind::Error => event.has_field("attempt"),
        EventKind::Compact => event.origin_key().is_some(),
        EventKind::Meta => event.origin_key().is_some() || event.has_field("upgrade"),
        EventKind::ToolCall => context.side_effectful_tool_call,
        EventKind::Checkpoint => true,
        EventKind::Epoch
        | EventKind::TurnOpen
        | EventKind::Reasoning
        | EventKind::ToolResult
        | EventKind::ChildResult
        | EventKind::State
        | EventKind::Extension(_) => false,
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("ledger is busy")]
    Busy,
    #[error(
        "ledger requires reader {required_reader} and writer {required_writer}; this writer supports reader {reader_version} and writer {writer_version}"
    )]
    VersionGate {
        required_reader: u64,
        required_writer: u64,
        reader_version: u64,
        writer_version: u64,
    },
    #[error("ledger inode changed while acquiring its lock")]
    ReplacedWhileLocking,
    #[error("ledger corruption: {0}")]
    Corruption(String),
    #[error("schema error: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("asset digest mismatch: expected {expected}, found {actual}")]
    AssetDigest { expected: String, actual: String },
    #[error("thread id already exists with a different genesis")]
    ThreadCollision,
    #[error("thread is archived")]
    Archived,
    #[error("thread is not ephemeral")]
    NotEphemeral,
    #[error("thread does not exist")]
    NotFound,
    #[error("thread is owned by a live rewrite operation")]
    RewriteInProgress,
    #[error("rewrite source prefix changed")]
    RewriteSourceChanged,
    #[error("rewrite destination already exists")]
    RewriteDestinationExists,
    #[error("rewrite source is not terminal: {0}")]
    RewriteSourceNotTerminal(String),
    #[error("rewrite encountered an unsupported visible fact: {0}")]
    RewriteUnsupportedVisibleFact(String),
    #[error("rewrite carrier is corrupt: {0}")]
    RewriteCarrierCorrupt(String),
    #[error("rewrite operation record is corrupt: {0}")]
    RewriteOperationCorrupt(String),
    #[error("kernel seq {0} is not a complete terminal fork anchor")]
    InvalidForkAnchor(u64),
    #[error("unsupported filesystem: {0}")]
    UnsupportedFilesystem(String),
}
