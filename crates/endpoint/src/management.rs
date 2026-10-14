//! Crash-recoverable Session Endpoint workspace management.
//!
//! This module owns only the endpoint-management config/metadata journal.
//! Semantic ledger authorship remains in the worker/store boundary.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use profile::{
    ConfigRepository, InstructionResolver, WorkspaceConfig, WorkspaceFolder, WorkspacePolicy,
};
use schema::{Event, IJsonValue, OriginTuple, ResumePolicy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{
    AssetStore, AtomicPublisher, DirectoryLock, NamedLock, ThreadStore, scan_valid_prefix,
};
use thiserror::Error;
use uuid::Uuid;

use crate::SessionInventoryItem;

const VERSION: u64 = 2;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceView {
    pub workspace_id: String,
    pub path: String,
    pub title: String,
    pub session_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// One authored workspace folder and its stable binding id.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceFolderView {
    pub folder_id: String,
    pub path: String,
}

/// The ordered folder list of one workspace at one config revision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceFolders {
    pub workspace_id: String,
    pub revision: u64,
    pub folders: Vec<WorkspaceFolderView>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkspaceList {
    pub items: Vec<WorkspaceView>,
    pub archived_session_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkspaceMetadata {
    v: u64,
    workspace_id: String,
    path: String,
    created_at: String,
    metadata_updated_at: String,
    config_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OperationPhase {
    Prepared,
    Carriers,
    Semantic,
    Inventory,
    Complete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkspaceIntent {
    kind: String,
    action: String,
    workspace_id: String,
    canonical_path: String,
    title: String,
    config_digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relocate_folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    previous_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    relocated_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    folder_path: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FolderMoveIntent {
    kind: String,
    action: String,
    session_id: String,
    source_path: String,
    dest_path: String,
    directory_identity: String,
    valid_prefix_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DiscardIntent {
    kind: String,
    session_id: String,
    directory_identity: String,
    valid_prefix_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionCreateIntent {
    kind: String,
    session_id: String,
    workspace_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    folder_binding: String,
    cwd: String,
    config_asset: String,
    instruction_asset: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    identity_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preset_digest: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectModelIntent {
    kind: String,
    session_id: String,
    expected_revision: u64,
    next_revision: u64,
    provider: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ForkIntent {
    kind: String,
    source: String,
    dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    at_endpoint_seq: Option<u64>,
    kernel_anchor: u64,
    rewrite_op_id: String,
    principal: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    ephemeral: bool,
    /// Preset resolved at fork time for the destination; frozen so a retry
    /// rebuilds the same genesis whatever the registry says later.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    preset: Option<tools::ResolvedPreset>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QueueTransactionIntent {
    kind: String,
    session_id: String,
    target_seq: u64,
    action: IJsonValue,
    retract_origin: OriginTuple,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_origin: Option<OriginTuple>,
    asset_digests: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QueueTailSnapshot {
    v: u64,
    session_id: String,
    target_seq: u64,
    last_seq: u64,
    valid_prefix_digest: String,
}
/// The origin-tuple `client` namespace stamped on every endpoint-authored event.
pub const ORIGIN_CLIENT: &str = "session-endpoint";
/// Namespace carried by ledgers and operation records written before the
/// rename; recovery accepts it as endpoint-authored, nothing writes it.
pub const LEGACY_ORIGIN_CLIENT: &str = "session-endpoint-v2";

#[must_use]
pub fn is_endpoint_origin_client(client: &str) -> bool {
    client == ORIGIN_CLIENT || client == LEGACY_ORIGIN_CLIENT
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OperationRecord {
    v: u64,
    rpc_id: String,
    operation: String,
    request_sha256: String,
    phase: OperationPhase,
    started_at: String,
    intent: IJsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<IJsonValue>,
}

struct WorkspaceOperationWrite<'a> {
    rpc_id: &'a str,
    request_sha256: &'a str,
    operation: &'a str,
    action: &'a str,
    config: &'a WorkspaceConfig,
    started_at: &'a str,
    created: Option<bool>,
    relocation: Option<(&'a str, &'a str, &'a str)>,
    /// `(folder id, folder path)` for `add-folder` and `remove-folder`.
    folder: Option<(&'a str, &'a str)>,
}

/// MCP registry lookups for resolving session presets; the supervisor owns
/// the registry and the projected tool naming (spec mcp-runtime).
pub trait PresetMcpSource {
    /// Canonical definition digest of the server registered under `server`
    /// for the workspace (a project entry shadows a user entry).
    fn server_digest(&self, workspace_id: &str, server: &str) -> Option<String>;
    /// Model-facing `mcp__<server>__<tool>` name, `None` when it cannot project.
    fn tool_name(&self, server: &str, tool: &str) -> Option<String>;
}

/// The environment a preset resolves in: the instruction snapshot's tool
/// ceiling and skills, and the workspace's MCP registry. `session.create` and
/// `session.presets.resolve` use the same one.
pub struct SessionPresetEnvironment<'a> {
    workspace_id: &'a str,
    permitted: Option<std::collections::BTreeSet<String>>,
    skills: &'a std::collections::BTreeMap<String, u64>,
    mcp: Option<&'a dyn PresetMcpSource>,
}

impl<'a> SessionPresetEnvironment<'a> {
    #[must_use]
    pub fn new(
        workspace_id: &'a str,
        instruction: &'a profile::InstructionSnapshot,
        mcp: Option<&'a dyn PresetMcpSource>,
    ) -> Self {
        Self {
            workspace_id,
            permitted: instruction
                .effective
                .policy
                .allowed_tools
                .as_ref()
                .map(|tools| tools.iter().cloned().collect()),
            skills: &instruction.effective.skills,
            mcp,
        }
    }
}

impl tools::PresetEnvironment for SessionPresetEnvironment<'_> {
    fn permitted_tools(&self) -> Option<&std::collections::BTreeSet<String>> {
        self.permitted.as_ref()
    }

    fn mcp_server_digest(&self, server: &str) -> Option<String> {
        self.mcp?.server_digest(self.workspace_id, server)
    }

    fn mcp_tool_name(&self, server: &str, tool: &str) -> Option<String> {
        self.mcp?.tool_name(server, tool)
    }

    fn skill_known(&self, name: &str) -> bool {
        self.skills.contains_key(name)
    }
}

pub struct SessionCreateOperation<'a> {
    pub rpc_id: &'a str,
    pub request_sha256: &'a str,
    pub requested_session_id: Option<&'a str>,
    pub workspace_id: Option<&'a str>,
    pub cwd: Option<&'a str>,
    pub identity_profile: Option<tools::IdentityProfile>,
    /// Raw `session.create {preset}` request, resolved under the workspace base.
    pub preset: Option<&'a Value>,
    /// MCP registry lookups a preset's `mcp` references resolve against.
    pub preset_mcp: Option<&'a dyn PresetMcpSource>,
    pub user_agent_dir: &'a Path,
    pub started_at: &'a str,
    pub principal: &'a str,
}

pub struct SelectModelOperation<'a> {
    pub rpc_id: &'a str,
    pub request_sha256: &'a str,
    pub session_id: &'a str,
    pub provider: &'a str,
    pub model: &'a str,
    pub reasoning_effort: Option<&'a str>,
    pub started_at: &'a str,
}

pub struct ForkSessionOperation<'a> {
    pub rpc_id: &'a str,
    pub request_sha256: &'a str,
    pub source_session_id: &'a str,
    pub at_endpoint_seq: Option<u64>,
    pub started_at: &'a str,
    pub principal: &'a str,
    /// Publish the destination as a scratch ledger (see `session.discard`).
    pub ephemeral: bool,
    /// Raw `session.fork {preset}`: replaces the source's preset in the
    /// destination, resolved under the source workspace.
    pub preset: Option<&'a Value>,
    pub preset_mcp: Option<&'a dyn PresetMcpSource>,
    pub user_agent_dir: &'a Path,
}

/// One completed `session.fork` as inventory sees it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForkLineage {
    pub source: String,
    pub ephemeral: bool,
}

pub struct QueueTransactionOperation<'a> {
    pub rpc_id: &'a str,
    pub request_sha256: &'a str,
    pub session_id: &'a str,
    pub target_seq: u64,
    pub action: &'a IJsonValue,
    pub retract_origin: &'a OriginTuple,
    pub replacement_origin: Option<&'a OriginTuple>,
    pub asset_digests: &'a [String],
    pub started_at: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingQueueTransaction {
    pub rpc_id: String,
    pub session_id: String,
    pub target_seq: u64,
    pub action: IJsonValue,
    pub retract_origin: OriginTuple,
    pub replacement_origin: Option<OriginTuple>,
    pub asset_digests: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueueTransactionDecision {
    Committed,
    Rejected {
        code: String,
        reason: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueueTransactionCompletion {
    Committed,
    Rejected {
        code: String,
        reason: Option<String>,
    },
}

// Pending is the transient arm; boxing it would change the public state type.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
pub enum QueueTransactionState {
    Pending(PendingQueueTransaction),
    Complete(QueueTransactionCompletion),
}

pub trait QueueRecoveryDriver {
    fn execute(
        &self,
        transaction: &PendingQueueTransaction,
    ) -> Result<QueueTransactionDecision, ManagementError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedModel {
    pub provider: String,
    pub model: String,
    pub reasoning_effort: Option<String>,
}

/// Operation records a recovering open already validated as complete, with the identity of the
/// file validated. The periodic recovery skips them while the file is unchanged.
#[derive(Debug, Default)]
pub struct CompleteOperations(
    std::collections::HashSet<(PathBuf, (u64, Option<std::time::SystemTime>))>,
);

fn file_identity(path: &Path) -> Option<(u64, Option<std::time::SystemTime>)> {
    let metadata = fs::metadata(path).ok()?;
    Some((metadata.len(), metadata.modified().ok()))
}

/// Endpoint-scoped durable workspace/config authority.
pub struct ManagementStore {
    storage_root: PathBuf,
    root: PathBuf,
    config: ConfigRepository,
}

impl ManagementStore {
    /// Recovering open for this crate's tests. Production code has exactly two ways in:
    /// `open_at_with_queue_driver` (boot, which recovers pending operations) and `open_light`
    /// (every other caller), so a per-request open can never silently turn into the heavy one.
    #[cfg(test)]
    pub(crate) fn open_at(
        storage_root: impl AsRef<Path>,
        import_started_at: &str,
    ) -> Result<Self, ManagementError> {
        Self::open_at_with_queue_driver(storage_root, import_started_at, None)
    }

    pub fn open_at_with_queue_driver(
        storage_root: impl AsRef<Path>,
        import_started_at: &str,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
    ) -> Result<Self, ManagementError> {
        Self::open_recovering(storage_root, import_started_at, queue_driver, None)
    }

    /// The recovering open for the periodic retry: operation records already validated as
    /// complete (same path, length and modification time) are not read again, so an idle root
    /// costs a directory listing and a `stat` per record rather than a decode of each.
    pub fn open_recovering_again(
        storage_root: impl AsRef<Path>,
        import_started_at: &str,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        validated: &mut CompleteOperations,
    ) -> Result<Self, ManagementError> {
        Self::open_recovering(
            storage_root,
            import_started_at,
            queue_driver,
            Some(validated),
        )
    }

    fn open_recovering(
        storage_root: impl AsRef<Path>,
        import_started_at: &str,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        validated: Option<&mut CompleteOperations>,
    ) -> Result<Self, ManagementError> {
        let storage_root = storage_root.as_ref().to_path_buf();
        ThreadStore::open(&storage_root)?;
        let root = store::endpoint_management_root(&storage_root)?;
        fs::create_dir_all(root.join("workspaces"))?;
        fs::create_dir_all(root.join("operations"))?;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(root.join("lock"))?;
        let store = Self {
            config: ConfigRepository::open(&storage_root)?,
            storage_root,
            root,
        };
        store.recover_skipping(queue_driver, validated)?;
        store.seed_missing_workspace_policies()?;
        store.reconcile_workspace_metadata(import_started_at)?;
        // The 256 active-session cap is enforced where a slot is reserved (create, fork,
        // unarchive) and at live-event attach; opening the store, recovery and the List never
        // fail on the number of sessions (D5, ADDENDUM-03 C5).
        Ok(store)
    }

    /// The per-request constructor (ADDENDUM-03 C3, -04 K2). It performs **no** operation
    /// recovery and no policy seeding (those run in the recovering constructor at boot and on the
    /// background tick), but keeps the cheap workspace-metadata heal: metadata is a digest of the
    /// whole workspace config, so any config writer (policy set, relocate, folders, rename)
    /// invalidates it, and the next reader must not fail until some later tick. The heal reads only
    /// workspace config and metadata files, never a thread file.
    pub fn open_light(storage_root: impl AsRef<Path>) -> Result<Self, ManagementError> {
        Self::open_light_at(storage_root, &current_timestamp()?)
    }

    pub fn open_light_at(
        storage_root: impl AsRef<Path>,
        started_at: &str,
    ) -> Result<Self, ManagementError> {
        let storage_root = storage_root.as_ref().to_path_buf();
        let root = store::endpoint_management_root(&storage_root)?;
        fs::create_dir_all(root.join("workspaces"))?;
        fs::create_dir_all(root.join("operations"))?;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(root.join("lock"))?;
        let store = Self {
            config: ConfigRepository::open(&storage_root)?,
            storage_root,
            root,
        };
        let _lock = NamedLock::exclusive(store.root.join("lock"))?;
        store.heal_workspace_metadata_locked(started_at)?;
        drop(_lock);
        Ok(store)
    }

    /// Publishes corrected endpoint metadata for every workspace whose metadata is missing or
    /// does not match its config, without an operation record and without the Inventory phase
    /// (which reads threads). The caller holds the management `lock`. This function holds
    /// `config/.lock` shared from the config read to the metadata write, so no config writer can
    /// publish in between. That is the only protection here: `config/.lock` is not held across
    /// an operation (`begin_workspace_operation` to the Carriers publication). Every writer of
    /// workspace config therefore takes the management `lock` first (management, then config);
    /// `client_admin` does so around `workspace.policy.set`.
    fn heal_workspace_metadata_locked(&self, started_at: &str) -> Result<(), ManagementError> {
        let _config = NamedLock::shared(self.storage_root.join("config/.lock"))?;
        for config in self.workspace_configs()?.into_values() {
            // A workspace with no folder cannot have metadata; it fails only its own rows.
            let Ok(primary) = primary_path(&config).map(str::to_owned) else {
                eprintln!(
                    "management-workspace-skipped: id={} cwd is empty",
                    config.id
                );
                continue;
            };
            let existing = self.read_metadata_optional(&config.id)?;
            if existing
                .as_ref()
                .is_some_and(|metadata| verify_metadata(&config, metadata).is_ok())
            {
                continue;
            }
            let metadata = WorkspaceMetadata {
                v: VERSION,
                workspace_id: config.id.clone(),
                path: primary,
                created_at: existing
                    .map_or_else(|| started_at.to_owned(), |metadata| metadata.created_at),
                metadata_updated_at: started_at.to_owned(),
                config_digest: hex_digest(&canonical_line(&config)?),
            };
            publish_canonical(&self.metadata_path(&config.id), &metadata)?;
        }
        Ok(())
    }

    /// The management lock, exclusive, for a workspace-config writer outside this crate.
    /// Every operation here stages a config snapshot under this lock and publishes it later in
    /// the same critical section, so a writer that does not hold it can publish in between and
    /// leave the operation unable to complete. Lock order is management, then `config/.lock`
    /// (which the writer takes itself); hold this only around the publication.
    pub fn lock_exclusive(storage_root: impl AsRef<Path>) -> Result<NamedLock, ManagementError> {
        let root = store::endpoint_management_root(storage_root.as_ref())?;
        fs::create_dir_all(&root)?;
        Ok(NamedLock::exclusive(root.join("lock"))?)
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Re-drive every incomplete config operation before endpoint readiness.
    pub fn recover(&self) -> Result<(), ManagementError> {
        self.recover_with_queue_driver(None)
    }

    pub fn recover_with_queue_driver(
        &self,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
    ) -> Result<(), ManagementError> {
        self.recover_skipping(queue_driver, None)
    }

    fn recover_skipping(
        &self,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        mut validated: Option<&mut CompleteOperations>,
    ) -> Result<(), ManagementError> {
        // The management lock is taken per record (and released around the queue driver, which
        // can start a worker for several seconds), never for the whole pass: a stuck record
        // cannot hold up baselines and requests that need the lock.
        let paths = {
            let _lock = NamedLock::exclusive(self.root.join("lock"))?;
            self.remove_recordless_payloads()?;
            self.operation_paths()?
        };
        // One failing record does not stop the pass: the others are still recovered, and the
        // first failure is returned afterwards so boot still fails on it.
        let mut first_error = None;
        for path in paths {
            if let Err(error) = self.recover_record(path, queue_driver, validated.as_deref_mut()) {
                first_error.get_or_insert(error);
            }
        }
        first_error.map_or(Ok(()), Err)
    }

    #[allow(unused_assignments)]
    fn recover_record(
        &self,
        path: PathBuf,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        validated: Option<&mut CompleteOperations>,
    ) -> Result<(), ManagementError> {
        let mut lock = Some(NamedLock::exclusive(self.root.join("lock"))?);
        if !path.exists() {
            return Ok(());
        }
        let identity = file_identity(&path);
        if let (Some(known), Some(identity)) = (validated.as_deref(), identity.as_ref())
            && known.0.contains(&(path.clone(), *identity))
        {
            return Ok(());
        }
        let record = read_canonical::<OperationRecord>(&path)?;
        let kind = intent_kind(&record)?;
        if kind == "workspace" && record.v != VERSION {
            // A newer Kernel's record after a downgrade is not corrupt and not ours to judge:
            // leave it, and its payload, for its owner.
            eprintln!(
                "management-operation-skipped: record={} reason=unsupported record version",
                path.file_stem()
                    .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned())
            );
            return Ok(());
        }
        if record.phase == OperationPhase::Complete {
            if let (Some(known), Some(identity)) = (validated, identity) {
                known.0.insert((path.clone(), identity));
            }
            match kind.as_str() {
                "workspace" => {
                    if let Err(error) = workspace_intent(&record) {
                        return self.quarantine_corrupt_workspace(&path, error);
                    }
                }
                "folder-move" => {
                    folder_move_intent(&record)?;
                }
                "session-create" => {
                    session_create_intent(&record)?;
                }
                "select-model" => {
                    select_model_intent(&record)?;
                }
                "fork" => {
                    fork_intent(&record)?;
                }
                "queue-transaction" => {
                    let intent = queue_transaction_intent(&record)?;
                    verify_queue_payload(&self.operation_payload_path(&record.rpc_id), &intent)?;
                    queue_completion_from_record(&record)?;
                    return Ok(());
                }
                other => {
                    return Err(ManagementError::CorruptOperation(format!(
                        "unknown operation intent kind: {other}"
                    )));
                }
            }
            if record.response.is_none() {
                let error = ManagementError::CorruptOperation(
                    "complete operation lacks response".to_owned(),
                );
                return if kind == "workspace" {
                    self.quarantine_corrupt_workspace(&path, error)
                } else {
                    Err(error)
                };
            }
            return Ok(());
        }
        let recovery = match kind.as_str() {
            "workspace" => workspace_intent(&record)
                .and_then(|_| self.drive_workspace_operation(&path, record))
                .map(|_| ()),
            "folder-move" => {
                folder_move_intent(&record)?;
                self.drive_folder_move(&path, record, None).map(|_| ())
            }
            "session-create" => {
                session_create_intent(&record)?;
                self.drive_session_create(&path, record).map(|_| ())
            }
            "select-model" => {
                select_model_intent(&record)?;
                self.drive_select_model(&path, record).map(|_| ())
            }
            "fork" => {
                if self.fork_was_quarantined(&record)? {
                    // The source and failed operation are preserved. This fork has
                    // no successful response, but must not block unrelated sessions.
                    return Ok(());
                }
                self.drive_fork(&path, record).map(|_| ())
            }
            "queue-transaction" => {
                let intent = queue_transaction_intent(&record)?;
                verify_queue_payload(&self.operation_payload_path(&record.rpc_id), &intent)?;
                let pending = pending_queue_transaction(&record)?;
                let driver = queue_driver.ok_or_else(|| {
                    ManagementError::QueueRecoveryRequired(pending.session_id.clone())
                })?;
                // The driver may start a worker (a handshake of up to 10 s): do not hold
                // the management lock meanwhile, and re-check the record after it.
                drop(lock.take());
                let decision = driver.execute(&pending)?;
                lock = Some(NamedLock::exclusive(self.root.join("lock"))?);
                let current = read_canonical::<OperationRecord>(&path)?;
                if current.phase == OperationPhase::Complete {
                    return Ok(());
                }
                self.drive_queue_completion(&path, record, decision)
                    .map(|_| ())
            }
            other => {
                return Err(ManagementError::CorruptOperation(format!(
                    "unknown operation intent kind: {other}"
                )));
            }
        };
        match recovery {
            Ok(()) => {}
            Err(ManagementError::InvalidPath(unavailable))
                if unavailable_absolute_path(&unavailable) =>
            {
                // A durable operation bound to a removed checkout or an unmounted volume is
                // still pending, not corrupt. Preserve it for a later recovery pass without
                // making one unavailable workspace prevent the endpoint from becoming ready.
            }
            // K1: an operation that can never complete must not keep the endpoint from
            // becoming ready. Its record and payload are preserved outside the recovery set.
            Err(error @ ManagementError::CorruptOperation(_)) if kind == "workspace" => {
                return self.quarantine_corrupt_workspace(&path, error);
            }
            Err(error) => return Err(error),
        }
        Ok(())
    }

    fn quarantine_corrupt_workspace(
        &self,
        path: &Path,
        error: ManagementError,
    ) -> Result<(), ManagementError> {
        match error {
            ManagementError::CorruptOperation(reason) => self.quarantine_operation(path, &reason),
            other => Err(other),
        }
    }

    /// Moves a workspace operation that can never complete, with its payload, to
    /// `quarantine/<digest>/` (`record.json`, `payload-root/`, `reason.txt`) so recovery and boot
    /// stop re-driving it. The payload is copied and made durable first and the record is renamed
    /// last. Before that rename the operation is still intact in the recovery set and a retry
    /// starts over; after it the leftover payload is recordless and removed. No step loses the
    /// payload or leaves a record that could be driven without it.
    fn quarantine_operation(&self, path: &Path, reason: &str) -> Result<(), ManagementError> {
        let digest = path
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| is_hex_digest(value))
            .ok_or_else(|| {
                ManagementError::CorruptOperation("operation record has no digest name".to_owned())
            })?
            .to_owned();
        let quarantine = self.root.join("quarantine");
        fs::create_dir_all(&quarantine)?;
        // An earlier quarantine of the same digest is evidence: never overwrite it.
        let mut target = quarantine.join(&digest);
        let mut attempt = 1_u32;
        while target.join("record.json").exists() {
            target = quarantine.join(format!("{digest}-{attempt}"));
            attempt += 1;
        }
        // A directory without `record.json` is an unfinished copy; the original is still here.
        if target.exists() {
            fs::remove_dir_all(&target)?;
        }
        fs::create_dir_all(&target)?;
        let payload_root = path.with_extension("");
        if payload_root.is_dir() {
            copy_payload_tree(&payload_root, &target.join("payload-root"))?;
        }
        AtomicPublisher::replace(target.join("reason.txt"), reason.as_bytes())?;
        File::open(&target)?.sync_all()?;
        File::open(&quarantine)?.sync_all()?;
        fs::rename(path, target.join("record.json"))?;
        File::open(&target)?.sync_all()?;
        let prefix = path.parent().ok_or_else(|| {
            ManagementError::CorruptOperation("operation record has no prefix".to_owned())
        })?;
        File::open(prefix)?.sync_all()?;
        if payload_root.is_dir() {
            fs::remove_dir_all(&payload_root)?;
            File::open(prefix)?.sync_all()?;
        }
        eprintln!("management-operation-quarantined: record={digest} reason={reason}");
        Ok(())
    }

    /// Workspaces created before `workspace.create` seeded a policy resolve to
    /// the empty allowlist and therefore no tool at all. Publish the same seed
    /// for every policy-less workspace once, at the next open, so an existing
    /// installation behaves like a fresh one. An authored (present) policy is
    /// never touched.
    fn seed_missing_workspace_policies(&self) -> Result<(), ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        for config in self.workspace_configs()?.into_values() {
            if config.policy.is_some() {
                continue;
            }
            let Ok(folder) = primary_path(&config).map(str::to_owned) else {
                continue;
            };
            self.config.publish_workspace_policy(
                &config.id,
                config.revision,
                default_workspace_policy(&folder),
            )?;
        }
        Ok(())
    }

    /// Workspace metadata is derived state (a digest of the workspace config), so the periodic
    /// reconcile heals it in place under the management lock. It must not stage an operation
    /// record: a record carries a config snapshot, and a config writer that publishes after the
    /// snapshot makes the record's Carriers check fail forever (K1).
    fn reconcile_workspace_metadata(&self, started_at: &str) -> Result<(), ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        self.heal_workspace_metadata_locked(started_at)
    }

    pub fn list_workspaces(
        &self,
        sessions: &[SessionInventoryItem],
    ) -> Result<WorkspaceList, ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let mut configs = self.workspace_configs()?;
        configs.retain(|id, config| {
            let usable = primary_path(config).is_ok();
            if !usable {
                eprintln!("management-workspace-skipped: id={id} cwd is empty");
            }
            usable
        });
        let mut active = BTreeMap::<String, Vec<&SessionInventoryItem>>::new();
        let mut archived_session_ids = Vec::new();
        for session in sessions {
            if session.archived {
                archived_session_ids.push(session.session_id.clone());
            } else {
                active
                    .entry(session.workspace_id.clone())
                    .or_default()
                    .push(session);
            }
        }
        archived_session_ids.sort();

        // A config writer may have published between `open_light`'s heal and this read; heal
        // once under the lock we already hold and re-read (E831 E2).
        let configs = if configs.values().all(|config| {
            self.read_metadata_optional(&config.id)
                .ok()
                .flatten()
                .is_some_and(|metadata| verify_metadata(config, &metadata).is_ok())
        }) {
            configs
        } else {
            self.heal_workspace_metadata_locked(&current_timestamp()?)?;
            let mut healed = self.workspace_configs()?;
            healed.retain(|_, config| primary_path(config).is_ok());
            healed
        };
        let mut items = Vec::with_capacity(configs.len());
        for config in configs.values() {
            let metadata = self.read_metadata(&config.id)?;
            verify_metadata(config, &metadata)?;
            items.push(workspace_view(
                config,
                metadata,
                active.get(&config.id).cloned().unwrap_or_default(),
            )?);
        }
        items.sort_by(|left, right| left.workspace_id.cmp(&right.workspace_id));
        Ok(WorkspaceList {
            items,
            archived_session_ids,
        })
    }

    /// Every workspace's primary path, read once. A caller that needs the path of many sessions
    /// (an inventory baseline) uses this instead of `workspace_path` per session, which reads
    /// every workspace configuration each time.
    pub fn workspace_paths(&self) -> Result<BTreeMap<String, String>, ManagementError> {
        let _lock = NamedLock::shared(self.root.join("lock"))?;
        // A workspace without a folder has no path: its sessions' rows fail, nobody else's.
        Ok(self
            .workspace_configs()?
            .into_iter()
            .filter_map(|(id, config)| primary_path(&config).ok().map(|path| (id, path.to_owned())))
            .collect())
    }

    pub fn workspace_path(&self, workspace_id: &str) -> Result<String, ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let config = self
            .workspace_configs()?
            .remove(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
        Ok(primary_path(&config)?.to_owned())
    }

    /// Creates one native Session Endpoint thread through the durable
    /// management transaction. Snapshot bytes are staged and synced before
    /// `prepared`; after that point recovery reads only operation-private
    /// bytes and never rereads mutable config or instruction sources.
    pub fn create_session(
        &self,
        request: SessionCreateOperation<'_>,
    ) -> Result<String, ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.create"
                || record.request_sha256 != request.request_sha256
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.create".to_owned(),
                });
            }
            return self
                .session_id_from_response(self.drive_session_create(&operation_path, record)?);
        }

        if active_session_count(&self.storage_root)? >= 256 {
            return Err(ManagementError::ActiveSessionLimit);
        }
        let configs = self.workspace_configs()?;
        let (workspace, cwd, folder_binding) =
            resolve_session_workspace(&configs, request.workspace_id, request.cwd)?;
        let session_id = match request.requested_session_id {
            Some(value) => {
                crate::validate_session_id(value)?;
                value.to_owned()
            }
            None => allocate_uuid_v7()?,
        };
        let active = self.storage_root.join("threads").join(&session_id);
        let archived = self.storage_root.join("archive").join(&session_id);
        if active.exists() || archived.exists() {
            return Err(ManagementError::SessionConflict {
                session_id,
                requested_cwd: cwd,
                existing_cwd: existing_session_cwd(&active, &archived, &configs)?,
            });
        }

        let config = self
            .config
            .resolve_for_binding(&workspace.id, &folder_binding)?;
        let instruction = InstructionResolver::new_scoped(
            request.user_agent_dir,
            request
                .user_agent_dir
                .join("workspaces")
                .join(&workspace.id),
            config.workspace.cwd.iter().map(Path::new),
        )
        .capture()?;
        instruction.validate_against_config(&config)?;
        let environment =
            SessionPresetEnvironment::new(&workspace.id, &instruction, request.preset_mcp);
        let preset = session_preset(
            request.preset,
            request.identity_profile,
            config.workspace.policy.base.unwrap_or_default(),
            &environment,
        )?;
        let config_bytes = config.canonical_bytes()?;
        let instruction_bytes = instruction.canonical_bytes()?;
        let config_digest = hex_digest(&config_bytes);
        let instruction_digest = hex_digest(&instruction_bytes);
        let config_asset = format!("sha256-{config_digest}");
        let instruction_asset = format!("sha256-{instruction_digest}");
        let origin = OriginTuple {
            principal: request.principal.to_owned(),
            client: ORIGIN_CLIENT.to_owned(),
            target: session_id.clone(),
            op: "session.create".to_owned(),
            key: request.rpc_id.to_owned(),
        };
        let genesis = create_genesis(
            &session_id,
            &workspace.id,
            &folder_binding,
            &config_digest,
            &instruction_digest,
            request.identity_profile,
            preset.as_ref(),
            request.started_at,
            &origin,
        )?;
        let genesis_bytes = canonical_event_line(&genesis)?;
        let payload = self.operation_payload_path(request.rpc_id);
        self.remove_recordless_payload(request.rpc_id)?;
        let assets = AssetStore::new(payload.join("assets"))?;
        let published_config = assets.publish(&config_bytes)?;
        let published_instruction = assets.publish(&instruction_bytes)?;
        if published_config.asset != config_asset
            || published_instruction.asset != instruction_asset
        {
            return Err(ManagementError::CorruptOperation(
                "session snapshot publication changed digest identity".to_owned(),
            ));
        }
        AtomicPublisher::replace(payload.join("genesis.json"), &genesis_bytes)?;
        sync_operation_payload(&payload)?;

        let record = OperationRecord {
            v: VERSION,
            rpc_id: request.rpc_id.to_owned(),
            operation: "session.create".to_owned(),
            request_sha256: request.request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: request.started_at.to_owned(),
            intent: to_ijson(&SessionCreateIntent {
                kind: "session-create".to_owned(),
                session_id: session_id.clone(),
                workspace_id: workspace.id.clone(),
                folder_binding,
                cwd,
                config_asset,
                instruction_asset,
                identity_profile: preset.is_none().then(|| {
                    request
                        .identity_profile
                        .map_or("auto", tools::IdentityProfile::as_str)
                        .to_owned()
                }),
                preset_digest: preset.as_ref().map(|preset| preset.digest.clone()),
            })?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        self.session_id_from_response(self.drive_session_create(&operation_path, record)?)
    }

    pub fn select_model(
        &self,
        request: SelectModelOperation<'_>,
    ) -> Result<SelectedModel, ManagementError> {
        crate::validate_session_id(request.session_id)?;
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.selectModel"
                || record.request_sha256 != request.request_sha256
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.selectModel".to_owned(),
                });
            }
            return selected_model_from_response(self.drive_select_model(&operation_path, record)?);
        }
        let folder = active_session_folder(&self.storage_root, request.session_id)?;
        if ThreadStore::open(&self.storage_root)?
            .session_has_live_line_holder(request.session_id)?
        {
            return Err(ManagementError::SessionRunning(
                request.session_id.to_owned(),
            ));
        }
        let projection = valid_folder_projection(&folder)?;
        // A freshly created root session has no turn to settle yet. It is still quiescent: the
        // client must be able to choose its model before admitting the first prompt. Requiring a
        // terminal tail here classified that genesis-only state as `session-running` even though
        // it had neither a worker nor pending work.
        let has_unsettled_turn =
            projection.lifecycle.latest_turn.is_some() && !projection.lifecycle.terminal_tail;
        if has_unsettled_turn
            || projection.lifecycle.stop_active
            || projection.lifecycle.open_hold
            || projection.lifecycle.answered_hold
            || !projection.lifecycle.live_inputs.is_empty()
            || projection.lifecycle.unresolved_work
        {
            return Err(ManagementError::SessionRunning(
                request.session_id.to_owned(),
            ));
        }
        let current = self.config.session_settings(&folder)?;
        let expected_revision = current.as_ref().map_or(0, |value| value.revision);
        let candidate = profile::SessionSettings {
            format: 1,
            revision: expected_revision + 1,
            provider: request.provider.to_owned(),
            model: request.model.to_owned(),
            reasoning_effort: request.reasoning_effort.map(str::to_owned),
        };
        let candidate_bytes = candidate.canonical_bytes()?;
        self.remove_recordless_payload(request.rpc_id)?;
        let payload = self.operation_payload_path(request.rpc_id);
        AtomicPublisher::replace(payload.join(store::SESSION_SETTINGS_FILE), &candidate_bytes)?;
        sync_operation_payload(&payload)?;
        let record = OperationRecord {
            v: VERSION,
            rpc_id: request.rpc_id.to_owned(),
            operation: "session.selectModel".to_owned(),
            request_sha256: request.request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: request.started_at.to_owned(),
            intent: to_ijson(&SelectModelIntent {
                kind: "select-model".to_owned(),
                session_id: request.session_id.to_owned(),
                expected_revision,
                next_revision: candidate.revision,
                provider: candidate.provider.clone(),
                model: candidate.model.clone(),
                reasoning_effort: candidate.reasoning_effort.clone(),
            })?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        selected_model_from_response(self.drive_select_model(&operation_path, record)?)
    }

    pub fn fork_session(
        &self,
        request: ForkSessionOperation<'_>,
    ) -> Result<String, ManagementError> {
        crate::validate_session_id(request.source_session_id)?;
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.fork"
                || record.request_sha256 != request.request_sha256
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.fork".to_owned(),
                });
            }
            return fork_id_from_response(self.drive_fork(&operation_path, record)?);
        }
        if active_session_count(&self.storage_root)? >= 256 {
            return Err(ManagementError::ActiveSessionLimit);
        }
        let source_folder = active_session_folder(&self.storage_root, request.source_session_id)?;
        crate::NativeEndpoint::open(&self.storage_root)?
            .reconcile_projection(request.source_session_id)?;
        let journal = crate::EndpointJournal::open(&source_folder)?;
        // An absent position means the last settled turn boundary, so a fork
        // (and every SideChat) works while the source is mid-turn; the open
        // tail is simply not part of the copy.
        let selected_endpoint_seq = match request.at_endpoint_seq {
            Some(value) => value,
            None => journal.last_settled_endpoint_seq()?.ok_or_else(|| {
                ManagementError::InvalidAtSeq {
                    session_id: request.source_session_id.to_owned(),
                    at_seq: None,
                }
            })?,
        };
        let kernel_anchor = journal
            .kernel_anchor_for_endpoint_seq(selected_endpoint_seq)
            .map_err(|error| match error {
                crate::JournalError::EndpointSeqNotFound(_)
                | crate::JournalError::IncompleteProjectionGroup(_) => {
                    ManagementError::InvalidAtSeq {
                        session_id: request.source_session_id.to_owned(),
                        at_seq: request.at_endpoint_seq,
                    }
                }
                other => ManagementError::Journal(other),
            })?;
        // EndpointJournal holds a shared folder lifecycle lock. Release it
        // before rewrite publication acquires the source exclusively.
        drop(journal);
        let preset = request
            .preset
            .map(|raw| {
                self.resolve_fork_preset(
                    &source_folder,
                    raw,
                    request.user_agent_dir,
                    request.preset_mcp,
                )
            })
            .transpose()?;
        let destination = allocate_uuid_v7()?;
        let rewrite_op_id = format!("endpoint-fork-{}", hex_digest(request.rpc_id.as_bytes()));
        let intent = ForkIntent {
            kind: "fork".to_owned(),
            source: request.source_session_id.to_owned(),
            dest: destination.clone(),
            at_endpoint_seq: request.at_endpoint_seq,
            kernel_anchor,
            rewrite_op_id,
            principal: request.principal.to_owned(),
            ephemeral: request.ephemeral,
            preset,
        };
        let record = OperationRecord {
            v: VERSION,
            rpc_id: request.rpc_id.to_owned(),
            operation: "session.fork".to_owned(),
            request_sha256: request.request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: request.started_at.to_owned(),
            intent: to_ijson(&intent)?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        let response = self.drive_fork(&operation_path, record)?;
        fork_id_from_response(response)
    }

    /// Resolves a `session.fork {preset}` under the source session's workspace
    /// base, instruction snapshot and MCP registry, exactly as `session.create`.
    fn resolve_fork_preset(
        &self,
        source_folder: &Path,
        raw: &Value,
        user_agent_dir: &Path,
        mcp: Option<&dyn PresetMcpSource>,
    ) -> Result<tools::ResolvedPreset, ManagementError> {
        let genesis = store::read_genesis(source_folder.join("main.jsonl"))?.ok_or_else(|| {
            ManagementError::CorruptOperation("fork source has no genesis".to_owned())
        })?;
        let workspace_id = genesis
            .string_field("workspace")
            .ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "fork source genesis lacks a workspace".to_owned(),
                )
            })?
            .to_owned();
        let config = match genesis.string_field("folder_binding") {
            Some(binding) => self.config.resolve_for_binding(&workspace_id, binding)?,
            None => self.config.resolve(&workspace_id)?,
        };
        let instruction = InstructionResolver::new_scoped(
            user_agent_dir,
            user_agent_dir.join("workspaces").join(&workspace_id),
            config.workspace.cwd.iter().map(Path::new),
        )
        .capture()?;
        let environment = SessionPresetEnvironment::new(&workspace_id, &instruction, mcp);
        session_preset(
            Some(raw),
            None,
            config.workspace.policy.base.unwrap_or_default(),
            &environment,
        )?
        .ok_or_else(|| ManagementError::CorruptOperation("fork preset did not resolve".to_owned()))
    }

    /// Publishes the durable queue-management gate before a worker delivery.
    /// Exact retry returns the persisted decision and never revalidates a
    /// rejected transaction against a later tail.
    pub fn prepare_queue_transaction(
        &self,
        request: QueueTransactionOperation<'_>,
    ) -> Result<QueueTransactionState, ManagementError> {
        crate::validate_session_id(request.session_id)?;
        if request.target_seq == 0
            || request.retract_origin.target != request.session_id
            || request.retract_origin.op != "session.updateQueue"
            || request.retract_origin.key != format!("{}/retract", request.rpc_id)
            || request.replacement_origin.is_some_and(|origin| {
                origin.target != request.session_id
                    || origin.op != "session.updateQueue"
                    || origin.key != format!("{}/replacement", request.rpc_id)
            })
        {
            return Err(ManagementError::CorruptOperation(
                "queue transaction binding is invalid".to_owned(),
            ));
        }
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.updateQueue"
                || record.request_sha256 != request.request_sha256
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.updateQueue".to_owned(),
                });
            }
            return if record.phase == OperationPhase::Complete {
                verify_queue_payload(
                    &self.operation_payload_path(request.rpc_id),
                    &queue_transaction_intent(&record)?,
                )?;
                Ok(QueueTransactionState::Complete(
                    queue_completion_from_record(&record)?,
                ))
            } else {
                Ok(QueueTransactionState::Pending(pending_queue_transaction(
                    &record,
                )?))
            };
        }

        let folder = active_session_folder(&self.storage_root, request.session_id)?;
        let bytes = fs::read(folder.join("main.jsonl"))?;
        let scan = scan_valid_prefix(&bytes, schema::READER_VERSION);
        let projection = scan.projection.ok_or_else(|| {
            ManagementError::CorruptOperation(
                "queue target has no valid semantic ledger prefix".to_owned(),
            )
        })?;
        let valid_bytes = usize::try_from(scan.valid_bytes).map_err(|_| {
            ManagementError::CorruptOperation("queue tail length exceeds this host".to_owned())
        })?;
        let payload = self.operation_payload_path(request.rpc_id);
        self.remove_recordless_payload(request.rpc_id)?;
        AtomicPublisher::replace(
            payload.join("queue-action.json"),
            &request.action.canonical_bytes()?,
        )?;
        AtomicPublisher::replace(
            payload.join("target-tail.json"),
            &canonical_line(&QueueTailSnapshot {
                v: VERSION,
                session_id: request.session_id.to_owned(),
                target_seq: request.target_seq,
                last_seq: projection.last_seq,
                valid_prefix_digest: format!("sha256-{}", hex_digest(&bytes[..valid_bytes])),
            })?,
        )?;
        sync_operation_payload(&payload)?;
        let intent = QueueTransactionIntent {
            kind: "queue-transaction".to_owned(),
            session_id: request.session_id.to_owned(),
            target_seq: request.target_seq,
            action: request.action.clone(),
            retract_origin: request.retract_origin.clone(),
            replacement_origin: request.replacement_origin.cloned(),
            asset_digests: request.asset_digests.to_vec(),
        };
        let record = OperationRecord {
            v: VERSION,
            rpc_id: request.rpc_id.to_owned(),
            operation: "session.updateQueue".to_owned(),
            request_sha256: request.request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: request.started_at.to_owned(),
            intent: to_ijson(&intent)?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        verify_queue_payload(&payload, &intent)?;
        Ok(QueueTransactionState::Pending(pending_queue_transaction(
            &record,
        )?))
    }

    pub fn complete_queue_transaction(
        &self,
        rpc_id: &str,
        decision: QueueTransactionDecision,
    ) -> Result<QueueTransactionCompletion, ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        let record = read_canonical::<OperationRecord>(&operation_path)?;
        if record.rpc_id != rpc_id || record.operation != "session.updateQueue" {
            return Err(ManagementError::CorruptOperation(
                "queue completion does not match its operation".to_owned(),
            ));
        }
        self.drive_queue_completion(&operation_path, record, decision)
    }

    pub fn has_incomplete_session_operation(
        &self,
        session_id: &str,
    ) -> Result<bool, ManagementError> {
        crate::validate_session_id(session_id)?;
        let _lock = NamedLock::shared(self.root.join("lock"))?;
        for path in self.operation_paths()? {
            let record = read_canonical::<OperationRecord>(&path)?;
            if record.phase != OperationPhase::Complete
                && operation_target_session(&record)?.as_deref() == Some(session_id)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn completed_fork_lineage(&self) -> Result<BTreeMap<String, ForkLineage>, ManagementError> {
        let _lock = NamedLock::shared(self.root.join("lock"))?;
        let mut lineage = BTreeMap::new();
        for path in self.operation_paths()? {
            let record = read_canonical::<OperationRecord>(&path)?;
            if record.phase == OperationPhase::Complete && intent_kind(&record)? == "fork" {
                let intent = fork_intent(&record)?;
                let entry = ForkLineage {
                    source: intent.source,
                    ephemeral: intent.ephemeral,
                };
                if lineage.insert(intent.dest, entry).is_some() {
                    return Err(ManagementError::CorruptOperation(
                        "fork destination has more than one completed lineage".to_owned(),
                    ));
                }
            }
        }
        Ok(lineage)
    }

    pub fn create_workspace(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        path: &str,
        started_at: &str,
    ) -> Result<(WorkspaceView, bool), ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let canonical = canonical_workspace_path(path)?;
        let canonical_string = canonical.to_string_lossy().into_owned();
        let configs = self.workspace_configs()?;
        let existing = configs
            .values()
            .find(|config| primary_path(config).ok() == Some(canonical_string.as_str()));
        let (config, created) = match existing {
            Some(config) => (config.clone(), false),
            // A non-primary folder of another workspace would make cwd-only
            // session selection ambiguous; it cannot also root a new workspace.
            None if configs
                .values()
                .any(|config| config.folder_paths().contains(&canonical_string.as_str())) =>
            {
                return Err(ManagementError::InvalidPath(path.to_owned()));
            }
            None => {
                let title = canonical
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| ManagementError::InvalidTitle(path.to_owned()))?;
                validate_title(title)?;
                ensure_unique_title(&configs, title, None)?;
                (
                    WorkspaceConfig {
                        format: 1,
                        revision: 1,
                        id: allocate_uuid_v7()?,
                        name: title.to_owned(),
                        cwd: Vec::new(),
                        policy: Some(default_workspace_policy(&canonical_string)),
                        folders: vec![WorkspaceFolder {
                            id: "folder-0001".to_owned(),
                            path: canonical_string,
                        }],
                    },
                    true,
                )
            }
        };
        let record = self.begin_workspace_operation(WorkspaceOperationWrite {
            rpc_id,
            request_sha256,
            operation: "workspace.create",
            action: "create",
            config: &config,
            started_at,
            created: Some(created),
            relocation: None,
            folder: None,
        })?;
        let response = self.drive_workspace_operation(&record.0, record.1)?;
        let workspace = response
            .get("workspace")
            .ok_or(ManagementError::CorruptOperation(
                "missing workspace response".to_owned(),
            ))?;
        let created = response
            .get("created")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                ManagementError::CorruptOperation("missing create decision".to_owned())
            })?;
        Ok((serde_json::from_value(workspace.clone())?, created))
    }

    pub fn rename_workspace(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        workspace_id: &str,
        title: &str,
        started_at: &str,
    ) -> Result<WorkspaceView, ManagementError> {
        validate_title(title)?;
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let configs = self.workspace_configs()?;
        ensure_unique_title(&configs, title, Some(workspace_id))?;
        let current = configs
            .get(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
        let config = WorkspaceConfig {
            format: current.format,
            revision: current.revision + 1,
            id: current.id.clone(),
            name: title.to_owned(),
            cwd: current.cwd.clone(),
            folders: current.folders.clone(),
            policy: current.policy.clone(),
        };
        let record = self.begin_workspace_operation(WorkspaceOperationWrite {
            rpc_id,
            request_sha256,
            operation: "workspace.rename",
            action: "rename",
            config: &config,
            started_at,
            created: None,
            relocation: None,
            folder: None,
        })?;
        let response = self.drive_workspace_operation(&record.0, record.1)?;
        serde_json::from_value(response.get("workspace").cloned().ok_or_else(|| {
            ManagementError::CorruptOperation("missing workspace response".to_owned())
        })?)
        .map_err(Into::into)
    }

    /// Rebinds one authored workspace folder to a new on-disk directory while preserving the
    /// workspace id and every session's stable `folder_binding`. `previous_path` is an exact
    /// compare-and-swap guard: a stale Client can never move a different folder after inventory
    /// has advanced.
    pub fn relocate_workspace(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        workspace_id: &str,
        previous_path: &str,
        path: &str,
        started_at: &str,
    ) -> Result<WorkspaceView, ManagementError> {
        let canonical = canonical_workspace_path(path)?;
        let canonical_string = canonical.to_string_lossy().into_owned();
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let configs = self.workspace_configs()?;
        let current = configs
            .get(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;

        let mut folders = current.folders.clone();
        let target = folders
            .iter_mut()
            .find(|folder| folder.path == previous_path)
            .ok_or_else(|| ManagementError::InvalidPath(previous_path.to_owned()))?;
        let relocate_folder_id = target.id.clone();
        if configs.values().any(|config| {
            config.folders.iter().any(|folder| {
                folder.path == canonical_string
                    && (config.id != workspace_id || folder.id != relocate_folder_id)
            })
        }) {
            return Err(ManagementError::WorkspaceAmbiguous(canonical_string));
        }
        target.path = canonical_string.clone();
        let cwd = current
            .cwd
            .iter()
            .map(|candidate| {
                if candidate == previous_path {
                    canonical_string.clone()
                } else {
                    candidate.clone()
                }
            })
            .collect();
        let config = WorkspaceConfig {
            format: current.format,
            revision: current.revision + 1,
            id: current.id.clone(),
            name: current.name.clone(),
            cwd,
            folders,
            policy: current
                .policy
                .clone()
                .map(|policy| relocate_policy_roots(policy, previous_path, &canonical_string)),
        };
        let record = self.begin_workspace_operation(WorkspaceOperationWrite {
            rpc_id,
            request_sha256,
            operation: "workspace.relocate",
            action: "relocate",
            config: &config,
            started_at,
            created: None,
            relocation: Some((&relocate_folder_id, previous_path, &canonical_string)),
            folder: None,
        })?;
        let response = self.drive_workspace_operation(&record.0, record.1)?;
        serde_json::from_value(response.get("workspace").cloned().ok_or_else(|| {
            ManagementError::CorruptOperation("missing workspace response".to_owned())
        })?)
        .map_err(Into::into)
    }

    /// Returns the authored folder list of one workspace. A legacy `cwd`
    /// workspace reports the same synthesized ids its sessions bind to.
    pub fn workspace_folders(
        &self,
        workspace_id: &str,
    ) -> Result<WorkspaceFolders, ManagementError> {
        let config = self
            .workspace_configs()?
            .remove(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
        Ok(folders_view(&config))
    }

    /// Appends one existing directory to a workspace as a new stable folder
    /// binding. Existing sessions keep their binding; a later worker launch
    /// resolves the longer folder list. A seeded or authored policy gains the
    /// folder as a writable root, matching `workspace.create`.
    pub fn add_workspace_folder(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        workspace_id: &str,
        path: &str,
        started_at: &str,
    ) -> Result<(WorkspaceView, WorkspaceFolders), ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        if let Some(response) =
            self.resume_folder_operation(rpc_id, request_sha256, "workspace.addFolder")?
        {
            return folder_response(response);
        }
        let canonical = canonical_workspace_path(path)?
            .to_string_lossy()
            .into_owned();
        let configs = self.workspace_configs()?;
        let current = configs
            .get(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
        if configs
            .values()
            .any(|config| config.folder_paths().contains(&canonical.as_str()))
        {
            return Err(ManagementError::WorkspaceAmbiguous(canonical));
        }
        // A session created before stable bindings resolves only while its
        // workspace has exactly one folder; a second folder would strand it.
        let legacy = self
            .workspace_session_bindings(workspace_id)?
            .into_iter()
            .filter(|(_, binding)| binding.is_none())
            .map(|(session_id, _)| session_id)
            .collect::<Vec<_>>();
        if !legacy.is_empty() {
            return Err(ManagementError::LegacySessions {
                workspace_id: workspace_id.to_owned(),
                session_ids: legacy,
            });
        }
        let folder_id = next_folder_id(&materialized_folders(current))?;
        let config = with_added_folder(current, &folder_id, &canonical);
        self.drive_folder_operation(WorkspaceOperationWrite {
            rpc_id,
            request_sha256,
            operation: "workspace.addFolder",
            action: "add-folder",
            config: &config,
            started_at,
            created: None,
            relocation: None,
            folder: Some((&folder_id, &canonical)),
        })
    }

    /// Removes one authored folder. `path` is compared exactly with the
    /// authored path, so a folder whose directory is gone can still be
    /// removed. The last folder and a folder any active, archived or
    /// in-flight session is bound to are refused. Writable roots at or below
    /// the folder are dropped unless another remaining folder still owns them.
    pub fn remove_workspace_folder(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        workspace_id: &str,
        path: &str,
        started_at: &str,
    ) -> Result<(WorkspaceView, WorkspaceFolders), ManagementError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        if let Some(response) =
            self.resume_folder_operation(rpc_id, request_sha256, "workspace.removeFolder")?
        {
            return folder_response(response);
        }
        let configs = self.workspace_configs()?;
        let current = configs
            .get(workspace_id)
            .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
        let folders = materialized_folders(current);
        let folder_id = folders
            .iter()
            .find(|folder| folder.path == path)
            .map(|folder| folder.id.clone())
            .ok_or_else(|| ManagementError::InvalidPath(path.to_owned()))?;
        if folders.len() == 1 {
            return Err(ManagementError::LastFolder(workspace_id.to_owned()));
        }
        let bound = self
            .workspace_session_bindings(workspace_id)?
            .into_iter()
            .filter(|(_, binding)| binding.as_deref() == Some(folder_id.as_str()))
            .map(|(session_id, _)| session_id)
            .collect::<Vec<_>>();
        if !bound.is_empty() {
            return Err(ManagementError::FolderInUse {
                workspace_id: workspace_id.to_owned(),
                path: path.to_owned(),
                session_ids: bound,
            });
        }
        let config = with_removed_folder(current, &folder_id, path).ok_or_else(|| {
            ManagementError::CorruptOperation("located folder vanished".to_owned())
        })?;
        self.drive_folder_operation(WorkspaceOperationWrite {
            rpc_id,
            request_sha256,
            operation: "workspace.removeFolder",
            action: "remove-folder",
            config: &config,
            started_at,
            created: None,
            relocation: None,
            folder: Some((&folder_id, path)),
        })
    }

    /// An rpc id that already owns a record is an exact retry: it skips the
    /// precondition checks, which the completed operation has since changed.
    fn resume_folder_operation(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        operation: &str,
    ) -> Result<Option<Value>, ManagementError> {
        let operation_path = self.operation_path(rpc_id);
        if !operation_path.exists() {
            return Ok(None);
        }
        let record = read_canonical::<OperationRecord>(&operation_path)?;
        if record.rpc_id != rpc_id
            || record.operation != operation
            || record.request_sha256 != request_sha256
        {
            return Err(ManagementError::IdempotencyConflict {
                rpc_id: rpc_id.to_owned(),
                operation: operation.to_owned(),
            });
        }
        self.drive_workspace_operation(&operation_path, record)
            .map(Some)
    }

    fn drive_folder_operation(
        &self,
        write: WorkspaceOperationWrite<'_>,
    ) -> Result<(WorkspaceView, WorkspaceFolders), ManagementError> {
        let record = self.begin_workspace_operation(write)?;
        folder_response(self.drive_workspace_operation(&record.0, record.1)?)
    }

    /// Every session of `workspace_id` with its stable folder binding:
    /// active and archived folders plus session creates not yet complete.
    fn workspace_session_bindings(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<(String, Option<String>)>, ManagementError> {
        let mut sessions = Vec::new();
        for area in ["threads", "archive"] {
            let entries = match fs::read_dir(self.storage_root.join(area)) {
                Ok(entries) => entries,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error.into()),
            };
            for entry in entries {
                let entry = entry?;
                let Some(session_id) = entry.file_name().to_str().map(str::to_owned) else {
                    continue;
                };
                if !entry.file_type()?.is_dir() || crate::validate_session_id(&session_id).is_err()
                {
                    continue;
                }
                // Genesis is the first line; archived ledgers can be large.
                let mut bytes = Vec::new();
                match fs::File::open(entry.path().join("main.jsonl")) {
                    Ok(file) => {
                        std::io::BufRead::read_until(
                            &mut std::io::BufReader::new(file),
                            b'\n',
                            &mut bytes,
                        )?;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                    Err(error) => return Err(error.into()),
                }
                let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                    .projection
                    .ok_or_else(|| {
                        ManagementError::CorruptOperation("session ledger lacks genesis".to_owned())
                    })?;
                let genesis = projection.events.first().ok_or_else(|| {
                    ManagementError::CorruptOperation("session ledger lacks genesis".to_owned())
                })?;
                if genesis.string_field("workspace") == Some(workspace_id) {
                    let binding = genesis.string_field("folder_binding").map(str::to_owned);
                    sessions.push((session_id, binding));
                }
            }
        }
        for path in self.operation_paths()? {
            let record = read_canonical::<OperationRecord>(&path)?;
            if record.operation != "session.create" || record.phase == OperationPhase::Complete {
                continue;
            }
            let intent = session_create_intent(&record)?;
            if intent.workspace_id == workspace_id {
                let binding = (!intent.folder_binding.is_empty()).then_some(intent.folder_binding);
                sessions.push((intent.session_id, binding));
            }
        }
        sessions.sort();
        sessions.dedup();
        Ok(sessions)
    }

    pub fn archive_session(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        session_id: &str,
        started_at: &str,
    ) -> Result<Vec<String>, ManagementError> {
        let value = self.folder_move(rpc_id, request_sha256, session_id, started_at, "archive")?;
        value
            .get("archivedSessionIds")
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .map(|value| {
                        value.as_str().map(str::to_owned).ok_or_else(|| {
                            ManagementError::CorruptOperation(
                                "archive response contains a non-string id".to_owned(),
                            )
                        })
                    })
                    .collect()
            })
            .ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "archive response lacks archivedSessionIds".to_owned(),
                )
            })?
    }

    pub fn unarchive_session(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        session_id: &str,
        started_at: &str,
    ) -> Result<String, ManagementError> {
        let value =
            self.folder_move(rpc_id, request_sha256, session_id, started_at, "unarchive")?;
        value
            .get("sessionId")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                ManagementError::CorruptOperation("unarchive response lacks sessionId".to_owned())
            })
    }

    /// Removes an ephemeral session folder under the same idempotent operation
    /// record discipline as archive. A durable session is `SessionNotEphemeral`;
    /// a live worker or pending work is `SessionRunning`.
    pub fn discard_session(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        session_id: &str,
        started_at: &str,
    ) -> Result<String, ManagementError> {
        crate::validate_session_id(session_id)?;
        let _management = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != rpc_id
                || record.request_sha256 != request_sha256
                || record.operation != "session.discard"
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: rpc_id.to_owned(),
                    operation: "session.discard".to_owned(),
                });
            }
            return self.drive_discard(&operation_path, record);
        }
        if self.storage_root.join("archive").join(session_id).is_dir() {
            return Err(ManagementError::SessionArchived(session_id.to_owned()));
        }
        let source = self.storage_root.join("threads").join(session_id);
        if !source.is_dir() {
            return Err(ManagementError::SessionNotFound(session_id.to_owned()));
        }
        if !folder_is_ephemeral(&source)? {
            return Err(ManagementError::SessionNotEphemeral(session_id.to_owned()));
        }
        let lifecycle = DirectoryLock::try_exclusive(&source).map_err(|error| match error {
            store::StoreError::Busy => ManagementError::SessionRunning(session_id.to_owned()),
            other => ManagementError::Store(other),
        })?;
        if ThreadStore::open(&self.storage_root)?.session_has_live_line_holder(session_id)? {
            return Err(ManagementError::SessionRunning(session_id.to_owned()));
        }
        let (directory_identity, valid_prefix_digest) = folder_reservation(&source)?;
        drop(lifecycle);
        let record = OperationRecord {
            v: VERSION,
            rpc_id: rpc_id.to_owned(),
            operation: "session.discard".to_owned(),
            request_sha256: request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: started_at.to_owned(),
            intent: to_ijson(&DiscardIntent {
                kind: "discard".to_owned(),
                session_id: session_id.to_owned(),
                directory_identity,
                valid_prefix_digest,
            })?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        self.drive_discard(&operation_path, record)
    }

    fn drive_discard(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<String, ManagementError> {
        let intent = discard_intent(&record)?;
        if record.v != VERSION || record.operation != "session.discard" {
            return Err(ManagementError::CorruptOperation(
                "unsupported discard operation".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Prepared {
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            let source = self.storage_root.join("threads").join(&intent.session_id);
            if source.is_dir() {
                let (identity, digest) = folder_reservation(&source)?;
                if identity != intent.directory_identity || digest != intent.valid_prefix_digest {
                    return Err(ManagementError::CorruptOperation(
                        "discard reservation no longer matches target".to_owned(),
                    ));
                }
                ThreadStore::open(&self.storage_root)?
                    .discard_ephemeral(&intent.session_id)
                    .map_err(|error| match error {
                        store::StoreError::Busy => {
                            ManagementError::SessionRunning(intent.session_id.clone())
                        }
                        store::StoreError::NotEphemeral => {
                            ManagementError::SessionNotEphemeral(intent.session_id.clone())
                        }
                        other => ManagementError::Store(other),
                    })?;
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            // No workspace membership to update: the folder was never a member.
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            record.phase = OperationPhase::Complete;
            record.response = Some(to_ijson(&json!({"sessionId":intent.session_id}))?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation("complete discard lacks response".to_owned())
        })?;
        fork_id_from_response(serde_json::from_slice(&response.canonical_bytes()?)?)
    }

    fn folder_move(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        session_id: &str,
        started_at: &str,
        action: &str,
    ) -> Result<Value, ManagementError> {
        crate::validate_session_id(session_id)?;
        let _management = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        let requested_operation = if action == "archive" {
            "workspace.archiveSession"
        } else {
            "workspace.unarchiveSession"
        };
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != rpc_id
                || record.request_sha256 != request_sha256
                || record.operation != requested_operation
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: rpc_id.to_owned(),
                    operation: requested_operation.to_owned(),
                });
            }
            return self.drive_folder_move(&operation_path, record, None);
        }
        if action == "unarchive" && active_session_count(&self.storage_root)? >= 256 {
            return Err(ManagementError::ActiveSessionLimit);
        }
        let (source_area, destination_area, operation) = if action == "archive" {
            ("threads", "archive", "workspace.archiveSession")
        } else {
            ("archive", "threads", "workspace.unarchiveSession")
        };
        let source = self.storage_root.join(source_area).join(session_id);
        if !source.is_dir() {
            return Err(ManagementError::SessionNotFound(session_id.to_owned()));
        }
        if action == "archive" && folder_is_ephemeral(&source)? {
            return Err(ManagementError::SessionEphemeral(session_id.to_owned()));
        }
        let lifecycle = DirectoryLock::try_exclusive(&source).map_err(|error| match error {
            store::StoreError::Busy => ManagementError::SessionRunning(session_id.to_owned()),
            other => ManagementError::Store(other),
        })?;
        let (directory_identity, valid_prefix_digest) = folder_reservation(&source)?;
        let record = OperationRecord {
            v: VERSION,
            rpc_id: rpc_id.to_owned(),
            operation: operation.to_owned(),
            request_sha256: request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: started_at.to_owned(),
            intent: to_ijson(&FolderMoveIntent {
                kind: "folder-move".to_owned(),
                action: action.to_owned(),
                session_id: session_id.to_owned(),
                source_path: source.to_string_lossy().into_owned(),
                dest_path: self
                    .storage_root
                    .join(destination_area)
                    .join(session_id)
                    .to_string_lossy()
                    .into_owned(),
                directory_identity,
                valid_prefix_digest,
            })?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        self.drive_folder_move(&operation_path, record, Some(lifecycle))
    }

    fn begin_workspace_operation(
        &self,
        write: WorkspaceOperationWrite<'_>,
    ) -> Result<(PathBuf, OperationRecord), ManagementError> {
        let operation_path = self.operation_path(write.rpc_id);
        if operation_path.exists() {
            let existing = read_canonical::<OperationRecord>(&operation_path)?;
            if existing.rpc_id != write.rpc_id
                || existing.operation != write.operation
                || existing.request_sha256 != write.request_sha256
            {
                return Err(ManagementError::IdempotencyConflict {
                    rpc_id: write.rpc_id.to_owned(),
                    operation: write.operation.to_owned(),
                });
            }
            return Ok((operation_path, existing));
        }
        let candidate = canonical_line(write.config)?;
        let config_digest = hex_digest(&candidate);
        self.remove_recordless_payload(write.rpc_id)?;
        let payload = self.operation_payload_path(write.rpc_id);
        let stage = payload.join("workspace.json");
        AtomicPublisher::replace(&stage, &candidate)?;
        if let Some(created) = write.created {
            publish_canonical(
                &payload.join("create-result.json"),
                &json!({"created":created}),
            )?;
        }
        sync_operation_payload(&payload)?;
        let record = OperationRecord {
            v: VERSION,
            rpc_id: write.rpc_id.to_owned(),
            operation: write.operation.to_owned(),
            request_sha256: write.request_sha256.to_owned(),
            phase: OperationPhase::Prepared,
            started_at: write.started_at.to_owned(),
            intent: to_ijson(&WorkspaceIntent {
                kind: "workspace".to_owned(),
                action: write.action.to_owned(),
                workspace_id: write.config.id.clone(),
                canonical_path: primary_path(write.config)?.to_owned(),
                title: write.config.name.clone(),
                config_digest,
                relocate_folder_id: write.relocation.map(|value| value.0.to_owned()),
                previous_path: write.relocation.map(|value| value.1.to_owned()),
                relocated_path: write.relocation.map(|value| value.2.to_owned()),
                folder_id: write.folder.map(|value| value.0.to_owned()),
                folder_path: write.folder.map(|value| value.1.to_owned()),
            })?,
            response: None,
        };
        publish_canonical(&operation_path, &record)?;
        Ok((operation_path, record))
    }

    fn drive_workspace_operation(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, ManagementError> {
        if record.v != VERSION {
            return Err(ManagementError::CorruptOperation(
                "unsupported management operation record".to_owned(),
            ));
        }
        let intent = workspace_intent(&record)?;
        let stage = self
            .operation_payload_path(&record.rpc_id)
            .join("workspace.json");
        let config = match fs::read(&stage) {
            Ok(bytes) => WorkspaceConfig::decode(&bytes)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let rebuilt = self.rebuild_candidate(&intent)?;
                AtomicPublisher::replace(&stage, &canonical_line(&rebuilt)?)?;
                rebuilt
            }
            Err(error) => return Err(error.into()),
        };
        if config.id != intent.workspace_id
            || config.name != intent.title
            || primary_path(&config)? != intent.canonical_path
            || hex_digest(&canonical_line(&config)?) != intent.config_digest
        {
            return Err(ManagementError::CorruptOperation(
                "staged workspace does not match immutable intent".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Prepared {
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            let configs = self.workspace_configs()?;
            match configs.get(&config.id) {
                Some(existing) if existing == &config => {}
                Some(existing) if config.revision == existing.revision + 1 => {
                    self.config.publish_workspace(existing.revision, &config)?;
                }
                None if config.revision == 1 => self.config.publish_workspace(0, &config)?,
                _ => {
                    return Err(ManagementError::CorruptOperation(
                        "workspace publication does not match prepared revision".to_owned(),
                    ));
                }
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            let created = self.operation_created(&record, &intent)?;
            let metadata = match self.read_metadata_optional(&config.id)? {
                Some(existing) if record.operation == "workspace.create" && !created => existing,
                Some(existing) => WorkspaceMetadata {
                    v: VERSION,
                    workspace_id: config.id.clone(),
                    path: primary_path(&config)?.to_owned(),
                    created_at: existing.created_at,
                    metadata_updated_at: record.started_at.clone(),
                    config_digest: intent.config_digest.clone(),
                },
                None => WorkspaceMetadata {
                    v: VERSION,
                    workspace_id: config.id.clone(),
                    path: primary_path(&config)?.to_owned(),
                    created_at: record.started_at.clone(),
                    metadata_updated_at: record.started_at.clone(),
                    config_digest: intent.config_digest.clone(),
                },
            };
            publish_canonical(&self.metadata_path(&config.id), &metadata)?;
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            let metadata = self.read_metadata(&config.id)?;
            let sessions = crate::NativeEndpoint::open(&self.storage_root)?
                .list_sessions(&std::collections::HashSet::new())?;
            let members = sessions
                .iter()
                .filter(|session| !session.archived && session.workspace_id == config.id)
                .collect::<Vec<_>>();
            let workspace = workspace_view(&config, metadata, members)?;
            let value = if record.operation == "workspace.create" {
                // Existing-path create stages the unchanged config. Its
                // initial record temporarily stored the boolean in response.
                let created = self.operation_created(&record, &intent)?;
                json!({"workspace":workspace,"created":created})
            } else if intent.folder_id.is_some() {
                json!({"workspace":workspace,"folders":folders_view(&config)})
            } else {
                json!({"workspace":workspace})
            };
            record.phase = OperationPhase::Complete;
            record.response = Some(IJsonValue::parse(&serde_json::to_vec(&value)?)?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation("complete operation lacks response".to_owned())
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn workspace_configs(&self) -> Result<BTreeMap<String, WorkspaceConfig>, ManagementError> {
        let mut values = BTreeMap::new();
        for config in self.config.workspaces()? {
            if values.insert(config.id.clone(), config).is_some() {
                return Err(ManagementError::CorruptOperation(
                    "duplicate workspace id".to_owned(),
                ));
            }
        }
        Ok(values)
    }

    fn operation_created(
        &self,
        record: &OperationRecord,
        intent: &WorkspaceIntent,
    ) -> Result<bool, ManagementError> {
        if record.operation != "workspace.create" {
            return Ok(false);
        }
        let path = self
            .operation_payload_path(&record.rpc_id)
            .join("create-result.json");
        let value = match fs::read(&path) {
            Ok(bytes) => decode_canonical::<Value>(&path, &bytes)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let existing = self.workspace_configs()?.get(&intent.workspace_id).cloned();
                let created = existing.is_none();
                let value = json!({"created":created});
                publish_canonical(&path, &value)?;
                value
            }
            Err(error) => return Err(error.into()),
        };
        value
            .as_object()
            .filter(|object| object.len() == 1)
            .and_then(|object| object.get("created"))
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "workspace create stage lacks exact created decision".to_owned(),
                )
            })
    }

    fn rebuild_candidate(
        &self,
        intent: &WorkspaceIntent,
    ) -> Result<WorkspaceConfig, ManagementError> {
        let config = if intent.action == "create" {
            WorkspaceConfig {
                format: 1,
                revision: 1,
                id: intent.workspace_id.clone(),
                name: intent.title.clone(),
                cwd: Vec::new(),
                folders: vec![WorkspaceFolder {
                    id: "folder-0001".to_owned(),
                    path: intent.canonical_path.clone(),
                }],
                policy: None,
            }
        } else if intent.action == "rename" {
            let current = self
                .workspace_configs()?
                .remove(&intent.workspace_id)
                .ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "rename recovery cannot find source workspace".to_owned(),
                    )
                })?;
            let already_published = current.name == intent.title
                && hex_digest(&canonical_line(&current)?) == intent.config_digest;
            if already_published {
                current
            } else {
                WorkspaceConfig {
                    format: current.format,
                    revision: current.revision + 1,
                    id: current.id,
                    name: intent.title.clone(),
                    cwd: current.cwd,
                    folders: current.folders,
                    policy: current.policy,
                }
            }
        } else if intent.action == "relocate" {
            let mut current = self
                .workspace_configs()?
                .remove(&intent.workspace_id)
                .ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "relocate recovery cannot find source workspace".to_owned(),
                    )
                })?;
            if hex_digest(&canonical_line(&current)?) == intent.config_digest {
                current
            } else {
                let folder_id = intent.relocate_folder_id.as_deref().ok_or_else(|| {
                    ManagementError::CorruptOperation("relocate intent lacks folder id".to_owned())
                })?;
                let previous_path = intent.previous_path.as_deref().ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "relocate intent lacks previous path".to_owned(),
                    )
                })?;
                let relocated_path = intent.relocated_path.as_deref().ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "relocate intent lacks destination path".to_owned(),
                    )
                })?;
                let folder = current
                    .folders
                    .iter_mut()
                    .find(|folder| folder.id == folder_id && folder.path == previous_path)
                    .ok_or_else(|| {
                        ManagementError::CorruptOperation(
                            "relocate recovery source no longer matches intent".to_owned(),
                        )
                    })?;
                folder.path = relocated_path.to_owned();
                for path in &mut current.cwd {
                    if path == previous_path {
                        *path = relocated_path.to_owned();
                    }
                }
                current.revision += 1;
                current
            }
        } else if intent.action == "add-folder" || intent.action == "remove-folder" {
            let current = self
                .workspace_configs()?
                .remove(&intent.workspace_id)
                .ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "folder recovery cannot find source workspace".to_owned(),
                    )
                })?;
            if hex_digest(&canonical_line(&current)?) == intent.config_digest {
                current
            } else {
                let (Some(folder_id), Some(folder_path)) = (&intent.folder_id, &intent.folder_path)
                else {
                    return Err(ManagementError::CorruptOperation(
                        "folder intent lacks its folder".to_owned(),
                    ));
                };
                if intent.action == "add-folder" {
                    with_added_folder(&current, folder_id, folder_path)
                } else {
                    with_removed_folder(&current, folder_id, folder_path).ok_or_else(|| {
                        ManagementError::CorruptOperation(
                            "folder removal recovery source no longer matches intent".to_owned(),
                        )
                    })?
                }
            }
        } else {
            return Err(ManagementError::CorruptOperation(
                "unknown workspace action".to_owned(),
            ));
        };
        if hex_digest(&canonical_line(&config)?) != intent.config_digest {
            return Err(ManagementError::CorruptOperation(
                "rebuilt candidate does not match prepared digest".to_owned(),
            ));
        }
        Ok(config)
    }

    fn drive_session_create(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, ManagementError> {
        let intent = session_create_intent(&record)?;
        if record.v != VERSION || record.operation != "session.create" {
            return Err(ManagementError::CorruptOperation(
                "unsupported session-create operation".to_owned(),
            ));
        }
        let payload = self.operation_payload_path(&record.rpc_id);
        let asset_store = AssetStore::new(payload.join("assets"))?;
        let config_bytes = asset_store.read_verified(&intent.config_asset)?;
        let instruction_bytes = asset_store.read_verified(&intent.instruction_asset)?;
        let config = profile::ConfigSnapshot::decode(&config_bytes)?;
        let instruction = profile::InstructionSnapshot::decode(&instruction_bytes)?;
        instruction.validate_against_config(&config)?;
        if config.workspace.id != intent.workspace_id
            || execution_path_from_snapshot(&config)? != intent.cwd
            || !folder_binding_matches_snapshot(
                config.workspace.folder_binding.as_deref(),
                &intent.folder_binding,
            )
            || intent.config_asset != format!("sha256-{}", config.digest()?)
            || intent.instruction_asset != format!("sha256-{}", instruction.digest()?)
        {
            return Err(ManagementError::CorruptOperation(
                "session-create snapshots disagree with immutable intent".to_owned(),
            ));
        }
        let genesis = read_canonical_event(&payload.join("genesis.json"))?;
        verify_session_create_genesis(&genesis, &record, &intent)?;

        if record.phase == OperationPhase::Prepared {
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            ThreadStore::open(&self.storage_root)?.create_thread_with_assets(
                &intent.session_id,
                genesis,
                &[config_bytes, instruction_bytes],
            )?;
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            let mut metadata = self.read_metadata(&intent.workspace_id)?;
            metadata.metadata_updated_at = record.started_at.clone();
            publish_canonical(&self.metadata_path(&intent.workspace_id), &metadata)?;
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            let response = json!({"sessionId":intent.session_id});
            record.phase = OperationPhase::Complete;
            record.response = Some(to_ijson(&response)?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation(
                "complete session-create operation lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn session_id_from_response(&self, response: Value) -> Result<String, ManagementError> {
        response
            .as_object()
            .filter(|object| object.len() == 1)
            .and_then(|object| object.get("sessionId"))
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "session-create response lacks exact sessionId".to_owned(),
                )
            })
    }

    fn drive_select_model(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, ManagementError> {
        let intent = select_model_intent(&record)?;
        if record.v != VERSION || record.operation != "session.selectModel" {
            return Err(ManagementError::CorruptOperation(
                "unsupported select-model operation".to_owned(),
            ));
        }
        let payload = self.operation_payload_path(&record.rpc_id);
        // An operation staged before the document was renamed keeps the old
        // name in its payload; retiring it here lets that operation recover.
        let candidate_path = store::session_settings_path(&payload)?;
        let candidate = profile::SessionSettings::decode(&fs::read(&candidate_path)?)?;
        if candidate.revision != intent.next_revision
            || candidate.provider != intent.provider
            || candidate.model != intent.model
            || candidate.reasoning_effort != intent.reasoning_effort
        {
            return Err(ManagementError::CorruptOperation(
                "select-model candidate disagrees with immutable intent".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Prepared {
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            let folder = active_session_folder(&self.storage_root, &intent.session_id)?;
            self.config
                .publish_session_settings(&folder, intent.expected_revision, &candidate)?;
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            let folder = active_session_folder(&self.storage_root, &intent.session_id)?;
            let workspace_id = folder_workspace(&folder)?;
            let mut metadata = self.read_metadata(&workspace_id)?;
            metadata.metadata_updated_at = record.started_at.clone();
            publish_canonical(&self.metadata_path(&workspace_id), &metadata)?;
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            let mut selected = json!({
                "provider":intent.provider,
                "model":intent.model,
            });
            if let Some(effort) = intent.reasoning_effort {
                selected["reasoningEffort"] = Value::String(effort);
            }
            let response = json!({"selected":selected});
            record.phase = OperationPhase::Complete;
            record.response = Some(to_ijson(&response)?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation(
                "complete select-model operation lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn fork_was_quarantined(&self, record: &OperationRecord) -> Result<bool, ManagementError> {
        let intent = fork_intent(record)?;
        if !matches!(
            record.phase,
            OperationPhase::Prepared | OperationPhase::Carriers
        ) {
            return Ok(false);
        }
        let path = self
            .storage_root
            .join(".rewrite-quarantine")
            .join(&intent.rewrite_op_id)
            .join("op.json");
        if !path.is_file() {
            return Ok(false);
        }
        let operation = store::RewriteOperation::decode_canonical(&fs::read(path)?)?;
        Ok(operation.id == intent.rewrite_op_id
            && operation.kind == store::RewriteKind::Fork
            && operation.phase < store::RewritePhase::Validated
            && operation.source == intent.source
            && operation.dest == intent.dest
            && !self
                .storage_root
                .join("threads")
                .join(&intent.dest)
                .exists())
    }

    fn drive_fork(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, ManagementError> {
        let intent = fork_intent(&record)?;
        if record.v != VERSION || record.operation != "session.fork" {
            return Err(ManagementError::CorruptOperation(
                "unsupported fork operation".to_owned(),
            ));
        }
        if self.fork_was_quarantined(&record)? {
            return Err(ManagementError::CorruptOperation(
                "fork could not recover its assets; operation preserved in .rewrite-quarantine"
                    .to_owned(),
            ));
        }
        let rewrite_stage = self
            .storage_root
            .join("staging")
            .join(&intent.rewrite_op_id);
        let destination = self.storage_root.join("threads").join(&intent.dest);
        if record.phase == OperationPhase::Prepared {
            if !rewrite_stage.is_dir() && !destination.is_dir() {
                let origin = OriginTuple {
                    principal: intent.principal.clone(),
                    client: ORIGIN_CLIENT.to_owned(),
                    target: intent.dest.clone(),
                    op: "session.fork".to_owned(),
                    key: record.rpc_id.clone(),
                };
                ThreadStore::open(&self.storage_root)?.begin_fork_at_kernel_anchor(
                    &intent.rewrite_op_id,
                    &intent.source,
                    intent.kernel_anchor,
                    store::ForkGenesisBinding {
                        thread: intent.dest.clone(),
                        origin,
                        ephemeral: intent.ephemeral,
                        preset: intent
                            .preset
                            .as_ref()
                            .map(serde_json::to_value)
                            .transpose()?,
                    },
                    &record.started_at,
                )?;
            }
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            ThreadStore::open(&self.storage_root)?.recover_rewrites()?;
            if !destination.is_dir() {
                return Err(ManagementError::CorruptOperation(
                    "fork rewrite did not publish its destination".to_owned(),
                ));
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            // A scratch ledger is not workspace membership: nothing durable
            // about the workspace changed.
            if !intent.ephemeral {
                let workspace_id = folder_workspace(&destination)?;
                let mut metadata = self.read_metadata(&workspace_id)?;
                metadata.metadata_updated_at = record.started_at.clone();
                publish_canonical(&self.metadata_path(&workspace_id), &metadata)?;
            }
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            record.phase = OperationPhase::Complete;
            record.response = Some(to_ijson(&json!({"sessionId":intent.dest}))?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation("complete fork operation lacks response".to_owned())
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn drive_queue_completion(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
        decision: QueueTransactionDecision,
    ) -> Result<QueueTransactionCompletion, ManagementError> {
        let intent = queue_transaction_intent(&record)?;
        if record.v != VERSION || record.operation != "session.updateQueue" {
            return Err(ManagementError::CorruptOperation(
                "unsupported queue transaction operation".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Complete {
            let completed = queue_completion_from_record(&record)?;
            if completed != completion_for_decision(&decision) {
                return Err(ManagementError::CorruptOperation(
                    "queue completion decision changed after durability".to_owned(),
                ));
            }
            return Ok(completed);
        }
        verify_queue_payload(&self.operation_payload_path(&record.rpc_id), &intent)?;
        match decision {
            QueueTransactionDecision::Rejected { code, reason } => {
                if record.phase != OperationPhase::Prepared {
                    return Err(ManagementError::CorruptOperation(
                        "rejected queue transaction advanced before its decision".to_owned(),
                    ));
                }
                let completion = QueueTransactionCompletion::Rejected { code, reason };
                record.phase = OperationPhase::Complete;
                record.response = Some(queue_completion_value(&intent, &completion)?);
                publish_canonical(operation_path, &record)?;
                Ok(completion)
            }
            QueueTransactionDecision::Committed => {
                if record.phase == OperationPhase::Prepared {
                    record.phase = OperationPhase::Carriers;
                    publish_canonical(operation_path, &record)?;
                }
                if record.phase == OperationPhase::Carriers {
                    record.phase = OperationPhase::Semantic;
                    publish_canonical(operation_path, &record)?;
                }
                if record.phase == OperationPhase::Semantic {
                    record.phase = OperationPhase::Inventory;
                    publish_canonical(operation_path, &record)?;
                }
                if record.phase == OperationPhase::Inventory {
                    let completion = QueueTransactionCompletion::Committed;
                    record.phase = OperationPhase::Complete;
                    record.response = Some(queue_completion_value(&intent, &completion)?);
                    publish_canonical(operation_path, &record)?;
                }
                queue_completion_from_record(&record)
            }
        }
    }

    fn remove_recordless_payloads(&self) -> Result<(), ManagementError> {
        let operations = self.root.join("operations");
        for prefix in fs::read_dir(&operations)? {
            let prefix = prefix?;
            if !prefix.file_type()?.is_dir() {
                continue;
            }
            for entry in fs::read_dir(prefix.path())? {
                let entry = entry?;
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                let digest = entry.file_name().to_string_lossy().into_owned();
                if !is_hex_digest(&digest) {
                    return Err(ManagementError::CorruptOperation(
                        "operation payload directory has invalid digest name".to_owned(),
                    ));
                }
                if !prefix.path().join(format!("{digest}.json")).is_file() {
                    fs::remove_dir_all(entry.path())?;
                    File::open(prefix.path())?.sync_all()?;
                }
            }
        }
        Ok(())
    }

    fn remove_recordless_payload(&self, rpc_id: &str) -> Result<(), ManagementError> {
        let operation_path = self.operation_path(rpc_id);
        let payload_root = self
            .operation_payload_path(rpc_id)
            .parent()
            .ok_or_else(|| {
                ManagementError::CorruptOperation("operation payload has no root".to_owned())
            })?
            .to_path_buf();
        if !operation_path.exists() && payload_root.exists() {
            fs::remove_dir_all(&payload_root)?;
            File::open(payload_root.parent().ok_or_else(|| {
                ManagementError::CorruptOperation("operation payload has no prefix".to_owned())
            })?)?
            .sync_all()?;
        }
        Ok(())
    }

    fn drive_folder_move(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
        held_lock: Option<DirectoryLock>,
    ) -> Result<Value, ManagementError> {
        let intent = folder_move_intent(&record)?;
        if record.v != VERSION || !matches!(intent.action.as_str(), "archive" | "unarchive") {
            return Err(ManagementError::CorruptOperation(
                "unsupported folder move operation".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Prepared {
            record.phase = OperationPhase::Carriers;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Carriers {
            let source = Path::new(&intent.source_path);
            let destination = Path::new(&intent.dest_path);
            if source.is_dir() {
                let lifecycle = match held_lock {
                    Some(lock) => lock,
                    None => DirectoryLock::try_exclusive(source)?,
                };
                verify_folder_reservation(source, &intent)?;
                if destination.exists() {
                    return Err(ManagementError::CorruptOperation(
                        "folder move destination already exists".to_owned(),
                    ));
                }
                // The only function allowed to move a thread folder between areas: it renames,
                // makes both parents durable and announces `Moved` (ADDENDUM-03 C1).
                store::move_thread_folder(source, destination)?;
                drop(lifecycle);
            } else if destination.is_dir() {
                verify_folder_reservation(destination, &intent)?;
            } else {
                return Err(ManagementError::CorruptOperation(
                    "folder move lost both source and destination".to_owned(),
                ));
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            let destination = Path::new(&intent.dest_path);
            let workspace_id = folder_workspace(destination)?;
            if let Some(mut metadata) = self.read_metadata_optional(&workspace_id)? {
                metadata.metadata_updated_at = record.started_at.clone();
                publish_canonical(&self.metadata_path(&workspace_id), &metadata)?;
            }
            record.phase = OperationPhase::Inventory;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Inventory {
            let response = if intent.action == "archive" {
                json!({"archivedSessionIds":[intent.session_id]})
            } else {
                json!({"sessionId":intent.session_id})
            };
            record.phase = OperationPhase::Complete;
            record.response = Some(IJsonValue::parse(&serde_json::to_vec(&response)?)?);
            publish_canonical(operation_path, &record)?;
        }
        let response = record.response.ok_or_else(|| {
            ManagementError::CorruptOperation("complete folder move lacks response".to_owned())
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn operation_paths(&self) -> Result<Vec<PathBuf>, ManagementError> {
        let mut paths = Vec::new();
        for prefix in fs::read_dir(self.root.join("operations"))? {
            let prefix = prefix?;
            if !prefix.file_type()?.is_dir() {
                continue;
            }
            for entry in fs::read_dir(prefix.path())? {
                let entry = entry?;
                if entry.file_type()?.is_file()
                    && entry.path().extension().and_then(|value| value.to_str()) == Some("json")
                {
                    paths.push(entry.path());
                }
            }
        }
        paths.sort();
        Ok(paths)
    }

    fn operation_path(&self, rpc_id: &str) -> PathBuf {
        let digest = hex_digest(rpc_id.as_bytes());
        self.root
            .join("operations")
            .join(&digest[..2])
            .join(format!("{digest}.json"))
    }

    fn operation_payload_path(&self, rpc_id: &str) -> PathBuf {
        let digest = hex_digest(rpc_id.as_bytes());
        self.root
            .join("operations")
            .join(&digest[..2])
            .join(digest)
            .join("payload")
    }

    fn metadata_path(&self, workspace_id: &str) -> PathBuf {
        self.root
            .join("workspaces")
            .join(format!("{workspace_id}.json"))
    }

    fn read_metadata(&self, workspace_id: &str) -> Result<WorkspaceMetadata, ManagementError> {
        self.read_metadata_optional(workspace_id)?.ok_or_else(|| {
            ManagementError::CorruptOperation(format!(
                "workspace {workspace_id} has no endpoint metadata"
            ))
        })
    }

    fn read_metadata_optional(
        &self,
        workspace_id: &str,
    ) -> Result<Option<WorkspaceMetadata>, ManagementError> {
        let path = self.metadata_path(workspace_id);
        match fs::read(&path) {
            Ok(bytes) => Ok(Some(decode_canonical(&path, &bytes)?)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

fn primary_path(config: &WorkspaceConfig) -> Result<&str, ManagementError> {
    config
        .folders
        .first()
        .map(|folder| folder.path.as_str())
        .or_else(|| config.cwd.first().map(String::as_str))
        .ok_or_else(|| ManagementError::CorruptOperation("workspace cwd is empty".to_owned()))
}

fn binding_for_path(config: &WorkspaceConfig, path: &str) -> Result<String, ManagementError> {
    config.binding_for_path(path).ok_or_else(|| {
        ManagementError::CorruptOperation("workspace path has no folder binding".to_owned())
    })
}

fn binding_for_canonical_path(
    config: &WorkspaceConfig,
    canonical: &str,
) -> Result<Option<String>, ManagementError> {
    for authored in config.folder_paths() {
        let resolved = canonical_workspace_path(authored)?
            .to_string_lossy()
            .into_owned();
        if resolved == canonical {
            return binding_for_path(config, authored).map(Some);
        }
    }
    Ok(None)
}

fn execution_path_from_snapshot(config: &profile::ConfigSnapshot) -> Result<&str, ManagementError> {
    config.execution_cwd().ok_or_else(|| {
        ManagementError::CorruptOperation("snapshot workspace cwd is empty".to_owned())
    })
}

fn folder_binding_matches_snapshot(snapshot: Option<&str>, intent: &str) -> bool {
    intent.is_empty() || snapshot == Some(intent)
}

fn resolve_session_workspace<'a>(
    configs: &'a BTreeMap<String, WorkspaceConfig>,
    workspace_id: Option<&str>,
    cwd: Option<&str>,
) -> Result<(&'a WorkspaceConfig, String, String), ManagementError> {
    match (workspace_id, cwd) {
        (Some(workspace_id), cwd) => {
            let workspace = configs
                .get(workspace_id)
                .ok_or_else(|| ManagementError::WorkspaceNotFound(workspace_id.to_owned()))?;
            let primary = primary_path(workspace)?;
            if let Some(cwd) = cwd {
                let canonical = canonical_workspace_path(cwd)?
                    .to_string_lossy()
                    .into_owned();
                let binding = binding_for_canonical_path(workspace, &canonical)?
                    .ok_or_else(|| ManagementError::InvalidPath(cwd.to_owned()))?;
                return Ok((workspace, canonical, binding));
            }
            let primary = canonical_workspace_path(primary)?
                .to_string_lossy()
                .into_owned();
            Ok((
                workspace,
                primary.clone(),
                binding_for_canonical_path(workspace, &primary)?.ok_or_else(|| {
                    ManagementError::CorruptOperation(
                        "primary workspace path has no folder binding".to_owned(),
                    )
                })?,
            ))
        }
        (None, Some(cwd)) => {
            let canonical = canonical_workspace_path(cwd)?
                .to_string_lossy()
                .into_owned();
            let mut matches = Vec::new();
            for workspace in configs.values() {
                if let Some(binding) = binding_for_canonical_path(workspace, &canonical)? {
                    matches.push((workspace, binding));
                }
            }
            match matches.as_slice() {
                [(workspace, binding)] => Ok((*workspace, canonical, binding.clone())),
                [] => Err(ManagementError::InvalidPath(cwd.to_owned())),
                _ => Err(ManagementError::WorkspaceAmbiguous(canonical)),
            }
        }
        (None, None) => match configs.values().collect::<Vec<_>>().as_slice() {
            [workspace] => {
                let authored = primary_path(workspace)?;
                let path = canonical_workspace_path(authored)?
                    .to_string_lossy()
                    .into_owned();
                let binding = binding_for_path(workspace, authored)?;
                Ok((*workspace, path, binding))
            }
            _ => Err(ManagementError::InvalidCreateSelection),
        },
    }
}

fn existing_session_cwd(
    active: &Path,
    archived: &Path,
    configs: &BTreeMap<String, WorkspaceConfig>,
) -> Result<Option<String>, ManagementError> {
    let folder = if active.is_dir() {
        active
    } else if archived.is_dir() {
        archived
    } else {
        return Ok(None);
    };
    let workspace_id = folder_workspace(folder)?;
    configs
        .get(&workspace_id)
        .map(primary_path)
        .transpose()
        .map(|value| value.map(str::to_owned))
}

/// The preset a new session freezes in genesis: an explicit request resolved
/// under the workspace base, or the identity sugar when the base is `blank`.
/// A `general` workspace without a request keeps the identity-profile genesis.
fn session_preset(
    request: Option<&Value>,
    identity_profile: Option<tools::IdentityProfile>,
    base: tools::PresetBase,
    environment: &dyn tools::PresetEnvironment,
) -> Result<Option<tools::ResolvedPreset>, ManagementError> {
    match request {
        Some(request) => tools::ResolvedPreset::resolve_request(request, base, environment)
            .map(Some)
            .map_err(|error| match error {
                tools::PresetError::BadRequest(message) => {
                    ManagementError::PresetBadRequest(message)
                }
                tools::PresetError::Invalid(invalid) => ManagementError::PresetInvalid(invalid),
            }),
        None if base == tools::PresetBase::Blank => Ok(Some(tools::ResolvedPreset::from_identity(
            identity_profile,
            base,
        ))),
        None => Ok(None),
    }
}

#[allow(clippy::too_many_arguments)]
fn create_genesis(
    session_id: &str,
    workspace_id: &str,
    folder_binding: &str,
    config_digest: &str,
    instruction_digest: &str,
    identity_profile: Option<tools::IdentityProfile>,
    preset: Option<&tools::ResolvedPreset>,
    timestamp: &str,
    origin: &OriginTuple,
) -> Result<Event, ManagementError> {
    let mut value = json!({
        "v":1,
        "seq":1,
        "ts":timestamp,
        "kind":"genesis",
        "thread":session_id,
        "workspace":workspace_id,
        "folder_binding":folder_binding,
        "format":1,
        "min_reader":1,
        "min_writer":1,
        "resume":ResumePolicy::Never,
        "config":{"digest":config_digest},
        "instruction":{"digest":instruction_digest},
        "identity_profile":identity_profile.map_or("auto", tools::IdentityProfile::as_str),
        "origin_key":origin.key,
        "origin_tuple":origin,
    });
    // A preset replaces the identity profile and needs reader 2 (spec event
    // §Genesis); a session without one keeps the reader-1 genesis bytes.
    if let Some(preset) = preset {
        let object = value.as_object_mut().expect("genesis object");
        object.remove("identity_profile");
        object.insert("min_reader".to_owned(), json!(2));
        object.insert("preset".to_owned(), serde_json::to_value(preset)?);
    }
    Event::from_value(IJsonValue::parse(&serde_json::to_vec(&value)?)?).map_err(Into::into)
}

fn canonical_event_line(event: &Event) -> Result<Vec<u8>, ManagementError> {
    let mut bytes = event.canonical_bytes()?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn read_canonical_event(path: &Path) -> Result<Event, ManagementError> {
    let bytes = fs::read(path)?;
    if !bytes.ends_with(b"\n") || bytes.len() == 1 {
        return Err(ManagementError::CorruptOperation(format!(
            "{} is not one canonical event line",
            path.display()
        )));
    }
    Event::decode_canonical(&bytes[..bytes.len() - 1]).map_err(Into::into)
}

fn verify_session_create_genesis(
    genesis: &Event,
    record: &OperationRecord,
    intent: &SessionCreateIntent,
) -> Result<(), ManagementError> {
    let origin = genesis.origin_tuple()?.ok_or_else(|| {
        ManagementError::CorruptOperation("session-create genesis lacks origin".to_owned())
    })?;
    let genesis_value: Value = serde_json::from_slice(&genesis.canonical_bytes()?)?;
    let object = genesis_value.as_object().ok_or_else(|| {
        ManagementError::CorruptOperation("session-create genesis is not an object".to_owned())
    })?;
    let config_digest = object
        .get("config")
        .and_then(Value::as_object)
        .and_then(|value| value.get("digest"))
        .and_then(Value::as_str);
    let instruction_digest = object
        .get("instruction")
        .and_then(Value::as_object)
        .and_then(|value| value.get("digest"))
        .and_then(Value::as_str);
    if genesis.seq() != 1
        || genesis.string_field("thread") != Some(intent.session_id.as_str())
        || genesis.string_field("workspace") != Some(intent.workspace_id.as_str())
        || (!intent.folder_binding.is_empty()
            && genesis.string_field("folder_binding") != Some(intent.folder_binding.as_str()))
        || genesis.string_field("ts") != Some(record.started_at.as_str())
        || config_digest != intent.config_asset.strip_prefix("sha256-")
        || instruction_digest != intent.instruction_asset.strip_prefix("sha256-")
        || intent.identity_profile.as_ref().is_some_and(|profile| {
            genesis.string_field("identity_profile") != Some(profile.as_str())
        })
        || intent.preset_digest.as_deref()
            != object
                .get("preset")
                .and_then(|preset| preset.get("digest"))
                .and_then(Value::as_str)
        || !is_endpoint_origin_client(&origin.client)
        || origin.target != intent.session_id
        || origin.op != "session.create"
        || origin.key != record.rpc_id
    {
        return Err(ManagementError::CorruptOperation(
            "session-create genesis disagrees with immutable intent".to_owned(),
        ));
    }
    Ok(())
}

/// Copies a payload directory (regular files and directories only) and syncs every file.
fn copy_payload_tree(source: &Path, target: &Path) -> Result<(), ManagementError> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination = target.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_payload_tree(&entry.path(), &destination)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), &destination)?;
            File::open(&destination)?.sync_all()?;
        } else {
            return Err(ManagementError::CorruptOperation(
                "operation payload holds a non-regular entry".to_owned(),
            ));
        }
    }
    File::open(target)?.sync_all()?;
    Ok(())
}

fn sync_operation_payload(payload: &Path) -> Result<(), ManagementError> {
    File::open(payload)?.sync_all()?;
    let rpc_root = payload.parent().ok_or_else(|| {
        ManagementError::CorruptOperation("operation payload has no rpc root".to_owned())
    })?;
    File::open(rpc_root)?.sync_all()?;
    let prefix = rpc_root.parent().ok_or_else(|| {
        ManagementError::CorruptOperation("operation payload has no prefix".to_owned())
    })?;
    File::open(prefix)?.sync_all()?;
    Ok(())
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

/// The policy an endpoint-created workspace starts with. A workspace document
/// without `policy` resolves to the empty allowlist, which leaves a session no
/// tool at all; a Client that creates a workspace through `workspace.create`
/// expects the fixed interactive catalog inside that folder, with the
/// per-session permission mode deciding what may actually run.
/// `workspace.policy.set` replaces this seed like any authored policy.
#[must_use]
pub fn default_workspace_policy(canonical_folder: &str) -> WorkspacePolicy {
    WorkspacePolicy {
        writable_roots: vec![canonical_folder.to_owned()],
        toolchain_roots: Vec::new(),
        network: true,
        allowed_tools: tools::BuiltinManifest::compiled().interactive_names(),
        provider: None,
        model: None,
        max_wall_seconds: None,
        base: None,
    }
}

/// Moves every writable root at or below the relocated folder to the new
/// directory, so a seeded or authored root keeps following its folder.
fn relocate_policy_roots(
    mut policy: WorkspacePolicy,
    previous_path: &str,
    new_path: &str,
) -> WorkspacePolicy {
    let previous = Path::new(previous_path);
    for root in &mut policy.writable_roots {
        if let Ok(suffix) = Path::new(root.as_str()).strip_prefix(previous) {
            *root = if suffix.as_os_str().is_empty() {
                new_path.to_owned()
            } else {
                Path::new(new_path)
                    .join(suffix)
                    .to_string_lossy()
                    .into_owned()
            };
        }
    }
    policy.writable_roots.sort();
    policy.writable_roots.dedup();
    policy
}

/// A legacy `cwd` workspace becomes explicit folders whose ids equal the
/// bindings `WorkspaceConfig::binding_for_path` already synthesizes for it.
fn materialized_folders(config: &WorkspaceConfig) -> Vec<WorkspaceFolder> {
    if config.folders.is_empty() {
        config
            .cwd
            .iter()
            .enumerate()
            .map(|(index, path)| WorkspaceFolder {
                id: format!("folder-{:04}", index + 1),
                path: path.clone(),
            })
            .collect()
    } else {
        config.folders.clone()
    }
}

fn next_folder_id(folders: &[WorkspaceFolder]) -> Result<String, ManagementError> {
    let next = folders
        .iter()
        .filter_map(|folder| folder.id.strip_prefix("folder-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| ManagementError::CorruptOperation("folder ids are exhausted".to_owned()))?;
    Ok(format!("folder-{next:04}"))
}

fn with_added_folder(current: &WorkspaceConfig, folder_id: &str, path: &str) -> WorkspaceConfig {
    let mut folders = materialized_folders(current);
    folders.push(WorkspaceFolder {
        id: folder_id.to_owned(),
        path: path.to_owned(),
    });
    let policy = current.policy.clone().map(|mut policy| {
        policy.writable_roots.push(path.to_owned());
        policy.writable_roots.sort();
        policy.writable_roots.dedup();
        policy
    });
    WorkspaceConfig {
        format: current.format,
        revision: current.revision + 1,
        id: current.id.clone(),
        name: current.name.clone(),
        cwd: Vec::new(),
        folders,
        policy,
    }
}

/// Returns `None` when `(folder_id, path)` is not an authored folder.
fn with_removed_folder(
    current: &WorkspaceConfig,
    folder_id: &str,
    path: &str,
) -> Option<WorkspaceConfig> {
    let mut folders = materialized_folders(current);
    let before = folders.len();
    folders.retain(|folder| folder.id != folder_id || folder.path != path);
    if folders.len() == before {
        return None;
    }
    let removed = Path::new(path);
    let policy = current.policy.clone().map(|mut policy| {
        policy.writable_roots.retain(|root| {
            let root = Path::new(root.as_str());
            !root.starts_with(removed)
                || folders
                    .iter()
                    .any(|folder| root.starts_with(Path::new(&folder.path)))
        });
        policy
    });
    Some(WorkspaceConfig {
        format: current.format,
        revision: current.revision + 1,
        id: current.id.clone(),
        name: current.name.clone(),
        cwd: Vec::new(),
        folders,
        policy,
    })
}

fn folders_view(config: &WorkspaceConfig) -> WorkspaceFolders {
    WorkspaceFolders {
        workspace_id: config.id.clone(),
        revision: config.revision,
        folders: materialized_folders(config)
            .into_iter()
            .map(|folder| WorkspaceFolderView {
                folder_id: folder.id,
                path: folder.path,
            })
            .collect(),
    }
}

fn folder_response(response: Value) -> Result<(WorkspaceView, WorkspaceFolders), ManagementError> {
    let field = |name: &str| {
        response.get(name).cloned().ok_or_else(|| {
            ManagementError::CorruptOperation(format!("folder operation response lacks {name}"))
        })
    };
    Ok((
        serde_json::from_value(field("workspace")?)?,
        serde_json::from_value(field("folders")?)?,
    ))
}

fn canonical_workspace_path(value: &str) -> Result<PathBuf, ManagementError> {
    let path = Path::new(value);
    if !path.is_absolute() {
        return Err(ManagementError::InvalidPath(value.to_owned()));
    }
    path.canonicalize()
        .map_err(|_| ManagementError::InvalidPath(value.to_owned()))
}

fn unavailable_absolute_path(value: &str) -> bool {
    let path = Path::new(value);
    path.is_absolute() && !path.exists()
}

fn allocate_uuid_v7() -> Result<String, ManagementError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ManagementError::Clock(error.to_string()))?
        .as_millis();
    if millis > u128::from((1_u64 << 48) - 1) {
        return Err(ManagementError::Clock(
            "system clock exceeds UUIDv7 timestamp width".to_owned(),
        ));
    }
    let mut bytes = [0_u8; 16];
    fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    let timestamp = (millis as u64).to_be_bytes();
    bytes[..6].copy_from_slice(&timestamp[2..]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(Uuid::from_bytes(bytes).hyphenated().to_string())
}

fn current_timestamp() -> Result<String, ManagementError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ManagementError::Clock(error.to_string()))?;
    timestamp_from_millis(u64::try_from(duration.as_millis()).map_err(|_| {
        ManagementError::Clock("system clock exceeds supported millisecond range".to_owned())
    })?)
}

fn validate_title(title: &str) -> Result<(), ManagementError> {
    let trimmed = title.trim_matches(char::is_whitespace);
    if trimmed != title
        || title.is_empty()
        || title.len() > 256
        || title.chars().any(char::is_control)
    {
        return Err(ManagementError::InvalidTitle(title.to_owned()));
    }
    Ok(())
}

fn ensure_unique_title(
    configs: &BTreeMap<String, WorkspaceConfig>,
    title: &str,
    excluding: Option<&str>,
) -> Result<(), ManagementError> {
    if configs
        .values()
        .any(|config| config.name == title && excluding != Some(config.id.as_str()))
    {
        return Err(ManagementError::NameConflict(title.to_owned()));
    }
    Ok(())
}

fn verify_metadata(
    config: &WorkspaceConfig,
    metadata: &WorkspaceMetadata,
) -> Result<(), ManagementError> {
    let digest = hex_digest(&canonical_line(config)?);
    if metadata.v != VERSION
        || metadata.workspace_id != config.id
        || metadata.path != primary_path(config)?
        || metadata.config_digest != digest
    {
        return Err(ManagementError::CorruptOperation(format!(
            "workspace {} metadata does not match config",
            config.id
        )));
    }
    Ok(())
}

fn workspace_view(
    config: &WorkspaceConfig,
    metadata: WorkspaceMetadata,
    members: Vec<&SessionInventoryItem>,
) -> Result<WorkspaceView, ManagementError> {
    let mut session_ids = members
        .iter()
        .map(|session| session.session_id.clone())
        .collect::<Vec<_>>();
    session_ids.sort();
    let updated_at = members
        .iter()
        .filter_map(|session| timestamp_from_millis(session.updated_at).ok())
        .chain(std::iter::once(metadata.metadata_updated_at.clone()))
        .max()
        .expect("metadata timestamp is always present");
    Ok(WorkspaceView {
        workspace_id: config.id.clone(),
        path: primary_path(config)?.to_owned(),
        title: config.name.clone(),
        session_ids,
        created_at: metadata.created_at,
        updated_at,
    })
}

fn timestamp_from_millis(value: u64) -> Result<String, ManagementError> {
    let seconds = value / 1_000;
    let millis = value % 1_000;
    let days = i64::try_from(seconds / 86_400)
        .map_err(|_| ManagementError::CorruptOperation("timestamp overflow".to_owned()))?;
    let day_seconds = seconds % 86_400;
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        day_seconds / 3_600,
        day_seconds % 3_600 / 60,
        day_seconds % 60
    ))
}

fn publish_canonical<T: Serialize>(path: &Path, value: &T) -> Result<(), ManagementError> {
    AtomicPublisher::replace(path, &canonical_line(value)?)?;
    Ok(())
}

fn read_canonical<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ManagementError> {
    decode_canonical(path, &fs::read(path)?)
}

fn decode_canonical<T: for<'de> Deserialize<'de>>(
    path: &Path,
    bytes: &[u8],
) -> Result<T, ManagementError> {
    if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].ends_with(b"\n") {
        return Err(ManagementError::CorruptOperation(format!(
            "{} is not one canonical JSON line",
            path.display()
        )));
    }
    let body = &bytes[..bytes.len() - 1];
    let value = IJsonValue::parse(body)?;
    if value.canonical_bytes()? != body {
        return Err(ManagementError::CorruptOperation(format!(
            "{} is not RFC-8785 canonical",
            path.display()
        )));
    }
    serde_json::from_slice(body).map_err(Into::into)
}

fn canonical_line<T: Serialize>(value: &T) -> Result<Vec<u8>, ManagementError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| ManagementError::Canonical(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn to_ijson<T: Serialize>(value: &T) -> Result<IJsonValue, ManagementError> {
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(value)
            .map_err(|error| ManagementError::Canonical(error.to_string()))?,
    )
    .map_err(Into::into)
}

fn intent_kind(record: &OperationRecord) -> Result<String, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let value: Value = serde_json::from_slice(&bytes)?;
    value
        .as_object()
        .and_then(|object| object.get("kind"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ManagementError::CorruptOperation("operation intent has no kind".to_owned()))
}

fn workspace_intent(record: &OperationRecord) -> Result<WorkspaceIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: WorkspaceIntent = serde_json::from_slice(&bytes)?;
    let relocation = [
        &intent.relocate_folder_id,
        &intent.previous_path,
        &intent.relocated_path,
    ];
    let folder = [&intent.folder_id, &intent.folder_path];
    let relocation_is_exact = match intent.action.as_str() {
        "create" | "rename" => {
            relocation.iter().all(|field| field.is_none())
                && folder.iter().all(|field| field.is_none())
        }
        "relocate" => {
            relocation.iter().all(|field| field.is_some())
                && folder.iter().all(|field| field.is_none())
        }
        "add-folder" | "remove-folder" => {
            relocation.iter().all(|field| field.is_none())
                && folder.iter().all(|field| field.is_some())
        }
        _ => false,
    };
    if intent.kind != "workspace" || !relocation_is_exact {
        return Err(ManagementError::CorruptOperation(
            "operation is not a workspace intent".to_owned(),
        ));
    }
    Ok(intent)
}

fn discard_intent(record: &OperationRecord) -> Result<DiscardIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: DiscardIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "discard" {
        return Err(ManagementError::CorruptOperation(
            "operation is not a discard intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn folder_is_ephemeral(folder: &Path) -> Result<bool, ManagementError> {
    Ok(store::read_genesis(folder.join("main.jsonl"))?
        .as_ref()
        .is_some_and(schema::Event::is_ephemeral_genesis))
}

fn folder_move_intent(record: &OperationRecord) -> Result<FolderMoveIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: FolderMoveIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "folder-move" || !matches!(intent.action.as_str(), "archive" | "unarchive") {
        return Err(ManagementError::CorruptOperation(
            "operation is not a folder-move intent".to_owned(),
        ));
    }
    Ok(intent)
}

fn session_create_intent(record: &OperationRecord) -> Result<SessionCreateIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: SessionCreateIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "session-create"
        || !is_hex_digest(
            intent
                .config_asset
                .strip_prefix("sha256-")
                .unwrap_or_default(),
        )
        || !is_hex_digest(
            intent
                .instruction_asset
                .strip_prefix("sha256-")
                .unwrap_or_default(),
        )
    {
        return Err(ManagementError::CorruptOperation(
            "operation is not a valid session-create intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn select_model_intent(record: &OperationRecord) -> Result<SelectModelIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: SelectModelIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "select-model"
        || intent.next_revision != intent.expected_revision + 1
        || intent.provider.is_empty()
        || intent.model.is_empty()
        || intent
            .reasoning_effort
            .as_deref()
            .is_some_and(str::is_empty)
    {
        return Err(ManagementError::CorruptOperation(
            "operation is not a valid select-model intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn fork_intent(record: &OperationRecord) -> Result<ForkIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: ForkIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "fork"
        || intent.kernel_anchor == 0
        || intent.rewrite_op_id.is_empty()
        || !is_uid_principal(&intent.principal)
    {
        return Err(ManagementError::CorruptOperation(
            "operation is not a valid fork intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.source)?;
    crate::validate_session_id(&intent.dest)?;
    Ok(intent)
}

fn queue_transaction_intent(
    record: &OperationRecord,
) -> Result<QueueTransactionIntent, ManagementError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: QueueTransactionIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "queue-transaction"
        || intent.target_seq == 0
        || intent.retract_origin.target != intent.session_id
        || intent.retract_origin.op != "session.updateQueue"
        || intent.retract_origin.key != format!("{}/retract", record.rpc_id)
        || intent.replacement_origin.as_ref().is_some_and(|origin| {
            origin.target != intent.session_id
                || origin.op != "session.updateQueue"
                || origin.key != format!("{}/replacement", record.rpc_id)
        })
        || intent
            .asset_digests
            .iter()
            .any(|asset| !asset.strip_prefix("sha256-").is_some_and(is_hex_digest))
    {
        return Err(ManagementError::CorruptOperation(
            "operation is not a valid queue-transaction intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn pending_queue_transaction(
    record: &OperationRecord,
) -> Result<PendingQueueTransaction, ManagementError> {
    let intent = queue_transaction_intent(record)?;
    Ok(PendingQueueTransaction {
        rpc_id: record.rpc_id.clone(),
        session_id: intent.session_id,
        target_seq: intent.target_seq,
        action: intent.action,
        retract_origin: intent.retract_origin,
        replacement_origin: intent.replacement_origin,
        asset_digests: intent.asset_digests,
    })
}

fn verify_queue_payload(
    payload: &Path,
    intent: &QueueTransactionIntent,
) -> Result<(), ManagementError> {
    let action_path = payload.join("queue-action.json");
    let action = fs::read(&action_path)?;
    if action != intent.action.canonical_bytes()? {
        return Err(ManagementError::CorruptOperation(
            "queue action staging bytes disagree with prepared intent".to_owned(),
        ));
    }
    let snapshot: QueueTailSnapshot = read_canonical(&payload.join("target-tail.json"))?;
    if snapshot.v != VERSION
        || snapshot.session_id != intent.session_id
        || snapshot.target_seq != intent.target_seq
        || snapshot.last_seq == 0
        || !snapshot
            .valid_prefix_digest
            .strip_prefix("sha256-")
            .is_some_and(is_hex_digest)
    {
        return Err(ManagementError::CorruptOperation(
            "queue tail snapshot disagrees with prepared intent".to_owned(),
        ));
    }
    for asset in &intent.asset_digests {
        if !payload.join("assets").join(asset).is_file() {
            return Err(ManagementError::CorruptOperation(format!(
                "queue replacement asset is absent from staging: {asset}"
            )));
        }
    }
    Ok(())
}

fn completion_for_decision(decision: &QueueTransactionDecision) -> QueueTransactionCompletion {
    match decision {
        QueueTransactionDecision::Committed => QueueTransactionCompletion::Committed,
        QueueTransactionDecision::Rejected { code, reason } => {
            QueueTransactionCompletion::Rejected {
                code: code.clone(),
                reason: reason.clone(),
            }
        }
    }
}

fn queue_completion_value(
    intent: &QueueTransactionIntent,
    completion: &QueueTransactionCompletion,
) -> Result<IJsonValue, ManagementError> {
    let value = match completion {
        QueueTransactionCompletion::Committed => json!({
            "ok":true,
            "value":{"accepted":true},
        }),
        QueueTransactionCompletion::Rejected { code, reason } => {
            let item_id = format!("input:{}", intent.target_seq);
            let (message, details) = match (code.as_str(), reason.as_deref()) {
                ("queue-item-not-found", None) => (
                    "Queued item is no longer pending",
                    json!({"itemId":item_id}),
                ),
                ("steer-unavailable", None) => (
                    "Current turn no longer accepts steering",
                    json!({"itemId":item_id}),
                ),
                ("attachment-error", Some(reason)) if !reason.is_empty() => {
                    ("Image attachment is unavailable", json!({"reason":reason}))
                }
                _ => {
                    return Err(ManagementError::CorruptOperation(
                        "queue rejection is outside the closed union".to_owned(),
                    ));
                }
            };
            json!({
                "ok":false,
                "error":{"code":code,"message":message,"details":details},
            })
        }
    };
    to_ijson(&value)
}

fn queue_completion_from_record(
    record: &OperationRecord,
) -> Result<QueueTransactionCompletion, ManagementError> {
    if record.phase != OperationPhase::Complete {
        return Err(ManagementError::CorruptOperation(
            "queue operation is not complete".to_owned(),
        ));
    }
    let intent = queue_transaction_intent(record)?;
    let response = record.response.as_ref().ok_or_else(|| {
        ManagementError::CorruptOperation("complete queue operation lacks response".to_owned())
    })?;
    let bytes = response.canonical_bytes()?;
    let value: Value = serde_json::from_slice(&bytes)?;
    let completion = if value.get("ok") == Some(&Value::Bool(true)) {
        QueueTransactionCompletion::Committed
    } else {
        let error = value
            .get("error")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "queue completion lacks its exact error".to_owned(),
                )
            })?;
        let code = error.get("code").and_then(Value::as_str).ok_or_else(|| {
            ManagementError::CorruptOperation("queue completion lacks error code".to_owned())
        })?;
        let reason = error
            .get("details")
            .and_then(Value::as_object)
            .and_then(|details| details.get("reason"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        QueueTransactionCompletion::Rejected {
            code: code.to_owned(),
            reason,
        }
    };
    if response != &queue_completion_value(&intent, &completion)? {
        return Err(ManagementError::CorruptOperation(
            "queue completion response is not exact".to_owned(),
        ));
    }
    Ok(completion)
}

fn operation_target_session(record: &OperationRecord) -> Result<Option<String>, ManagementError> {
    Ok(match intent_kind(record)?.as_str() {
        "session-create" => Some(session_create_intent(record)?.session_id),
        "select-model" => Some(select_model_intent(record)?.session_id),
        "folder-move" => Some(folder_move_intent(record)?.session_id),
        "fork" => Some(fork_intent(record)?.dest),
        "queue-transaction" => Some(queue_transaction_intent(record)?.session_id),
        "workspace" => None,
        other => {
            return Err(ManagementError::CorruptOperation(format!(
                "unknown operation intent kind: {other}"
            )));
        }
    })
}

fn is_uid_principal(value: &str) -> bool {
    value.strip_prefix("uid:").is_some_and(|digits| {
        !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn fork_id_from_response(response: Value) -> Result<String, ManagementError> {
    response
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("sessionId"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            ManagementError::CorruptOperation("fork response lacks exact sessionId".to_owned())
        })
}

fn selected_model_from_response(response: Value) -> Result<SelectedModel, ManagementError> {
    let selected = response
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("selected"))
        .and_then(Value::as_object)
        .ok_or_else(|| {
            ManagementError::CorruptOperation(
                "select-model response lacks exact selected value".to_owned(),
            )
        })?;
    let provider = selected
        .get("provider")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            ManagementError::CorruptOperation("selected provider is missing".to_owned())
        })?;
    let model = selected
        .get("model")
        .and_then(Value::as_str)
        .ok_or_else(|| ManagementError::CorruptOperation("selected model is missing".to_owned()))?;
    let reasoning_effort = selected
        .get("reasoningEffort")
        .map(|value| {
            value.as_str().map(str::to_owned).ok_or_else(|| {
                ManagementError::CorruptOperation(
                    "selected reasoning effort is not a string".to_owned(),
                )
            })
        })
        .transpose()?;
    Ok(SelectedModel {
        provider: provider.to_owned(),
        model: model.to_owned(),
        reasoning_effort,
    })
}

fn active_session_folder(root: &Path, session_id: &str) -> Result<PathBuf, ManagementError> {
    if root.join("archive").join(session_id).is_dir() {
        return Err(ManagementError::SessionArchived(session_id.to_owned()));
    }
    let folder = root.join("threads").join(session_id);
    if !folder.is_dir() {
        return Err(ManagementError::SessionNotFound(session_id.to_owned()));
    }
    Ok(folder)
}

fn valid_folder_projection(folder: &Path) -> Result<schema::LedgerProjection, ManagementError> {
    // Boot no longer repairs every tail: bring this thread's tail clean when nobody holds it.
    let (_, scan) = store::read_ledger_repaired(folder.join("main.jsonl"))?;
    if scan.valid_bytes != scan.file_bytes {
        return Err(ManagementError::CorruptOperation(
            "session ledger has a torn or invalid tail".to_owned(),
        ));
    }
    scan.projection.ok_or_else(|| {
        ManagementError::CorruptOperation("session ledger has no valid genesis".to_owned())
    })
}

fn folder_reservation(folder: &Path) -> Result<(String, String), ManagementError> {
    let metadata = fs::metadata(folder)?;
    let (bytes, scan) = store::read_ledger_repaired(folder.join("main.jsonl"))?;
    if scan.valid_bytes != scan.file_bytes || scan.projection.is_none() {
        return Err(ManagementError::CorruptOperation(
            "folder move source has no complete valid ledger prefix".to_owned(),
        ));
    }
    let valid = usize::try_from(scan.valid_bytes).map_err(|_| {
        ManagementError::CorruptOperation("valid prefix length exceeds usize".to_owned())
    })?;
    Ok((
        format!("dev:{}:ino:{}", metadata.dev(), metadata.ino()),
        format!("sha256-{}", hex_digest(&bytes[..valid])),
    ))
}

fn verify_folder_reservation(
    folder: &Path,
    intent: &FolderMoveIntent,
) -> Result<(), ManagementError> {
    let (identity, digest) = folder_reservation(folder)?;
    if identity != intent.directory_identity || digest != intent.valid_prefix_digest {
        return Err(ManagementError::CorruptOperation(
            "folder move reservation no longer matches target".to_owned(),
        ));
    }
    Ok(())
}

fn folder_workspace(folder: &Path) -> Result<String, ManagementError> {
    let genesis = store::read_genesis(folder.join("main.jsonl"))?.ok_or_else(|| {
        ManagementError::CorruptOperation("folder ledger lacks genesis".to_owned())
    })?;
    genesis
        .string_field("workspace")
        .map(str::to_owned)
        .ok_or_else(|| {
            ManagementError::CorruptOperation("folder genesis lacks workspace".to_owned())
        })
}

fn active_session_count(root: &Path) -> Result<usize, ManagementError> {
    let mut count = 0_usize;
    for entry in fs::read_dir(root.join("threads"))? {
        if entry?.file_type()?.is_dir() {
            count += 1;
        }
    }
    Ok(count)
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Debug, Error)]
pub enum ManagementError {
    #[error("workspace path is invalid: {0}")]
    InvalidPath(String),
    #[error("workspace title is invalid: {0}")]
    InvalidTitle(String),
    #[error("workspace title already exists: {0}")]
    NameConflict(String),
    #[error("workspace was not found: {0}")]
    WorkspaceNotFound(String),
    #[error("workspace path matches more than one workspace: {0}")]
    WorkspaceAmbiguous(String),
    #[error("workspace has only one folder: {0}")]
    LastFolder(String),
    #[error("workspace folder {path} is bound to sessions")]
    FolderInUse {
        workspace_id: String,
        path: String,
        session_ids: Vec<String>,
    },
    #[error("workspace {workspace_id} has sessions without a stable folder binding")]
    LegacySessions {
        workspace_id: String,
        session_ids: Vec<String>,
    },
    #[error("session create requires one unambiguous workspace")]
    InvalidCreateSelection,
    #[error("session preset is malformed: {0}")]
    PresetBadRequest(String),
    #[error("session preset was refused: {}", .0.reason.as_str())]
    PresetInvalid(tools::PresetInvalid),
    #[error("rpc id was reused for another management operation: {rpc_id} ({operation})")]
    IdempotencyConflict { rpc_id: String, operation: String },
    #[error("session was not found: {0}")]
    SessionNotFound(String),
    #[error("session is archived: {0}")]
    SessionArchived(String),
    #[error("session is ephemeral and cannot be archived: {0}")]
    SessionEphemeral(String),
    #[error("session is not ephemeral: {0}")]
    SessionNotEphemeral(String),
    #[error("session has a running worker or pending work: {0}")]
    SessionRunning(String),
    #[error("fork position is not a complete endpoint projection boundary")]
    InvalidAtSeq {
        session_id: String,
        at_seq: Option<u64>,
    },
    #[error("session identity conflicts with existing state: {session_id}")]
    SessionConflict {
        session_id: String,
        requested_cwd: String,
        existing_cwd: Option<String>,
    },
    #[error("active session limit is 256")]
    ActiveSessionLimit,
    #[error("queue transaction recovery requires its worker authority: {0}")]
    QueueRecoveryRequired(String),
    #[error("queue transaction recovery failed: {0}")]
    QueueRecoveryFailed(String),
    #[error("management operation is corrupt: {0}")]
    CorruptOperation(String),
    #[error("management canonicalization failed: {0}")]
    Canonical(String),
    #[error("management clock failed: {0}")]
    Clock(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Schema(#[from] schema::SchemaError),
    #[error(transparent)]
    Profile(#[from] profile::ProfileError),
    #[error(transparent)]
    Store(#[from] store::StoreError),
    #[error(transparent)]
    Endpoint(#[from] crate::NativeEndpointError),
    #[error(transparent)]
    Journal(#[from] crate::JournalError),
    #[error(transparent)]
    EndpointType(#[from] crate::types::EndpointTypeError),
}

#[cfg(test)]
mod tests {
    use super::{
        ForkIntent, ManagementError, ManagementStore, OperationPhase, OperationRecord, VERSION,
        WorkspaceMetadata, WorkspaceOperationWrite, WorkspaceView, canonical_event_line,
        create_genesis, default_workspace_policy, folder_binding_matches_snapshot,
        publish_canonical, read_canonical, session_preset, to_ijson,
    };
    use crate::SessionInventoryItem;
    use profile::{ConfigRepository, WorkspaceConfig, WorkspaceFolder};
    use serde_json::{Value, json};
    use std::fs;
    use std::path::Path;
    use store::ThreadStore;

    #[test]
    fn omitted_identity_is_auto_but_explicit_and_legacy_identity_stay_fixed() {
        let origin = schema::OriginTuple {
            principal: "test".to_owned(),
            client: "session-endpoint".to_owned(),
            target: "018f0000-0000-7000-8000-000000000003".to_owned(),
            op: "session.create".to_owned(),
            key: "create".to_owned(),
        };
        let genesis = |identity| {
            create_genesis(
                &origin.target,
                "ws",
                "folder-1",
                "cfg",
                "ins",
                identity,
                None,
                "2026-09-27T00:00:00.000Z",
                &origin,
            )
            .unwrap()
        };
        assert_eq!(genesis(None).string_field("identity_profile"), Some("auto"));
        assert_eq!(
            genesis(Some(tools::IdentityProfile::Coding)).string_field("identity_profile"),
            Some("coding")
        );
        assert_eq!(
            genesis(Some(tools::IdentityProfile::General)).string_field("identity_profile"),
            Some("general")
        );
    }

    #[test]
    fn preset_genesis_replaces_identity_and_needs_reader_two() {
        let origin = schema::OriginTuple {
            principal: "test".to_owned(),
            client: "session-endpoint".to_owned(),
            target: "018f0000-0000-7000-8000-000000000003".to_owned(),
            op: "session.create".to_owned(),
            key: "create".to_owned(),
        };
        let request = json!({"layers":[{"kind":"kernel","id":"coding"},
            {"kind":"inline","id":"cad-expert","version":"1","duty":"Prefer DXF tools."}]});
        let preset = session_preset(
            Some(&request),
            None,
            tools::PresetBase::General,
            &tools::NoPresetEnvironment,
        )
        .unwrap()
        .expect("explicit preset");
        let genesis = create_genesis(
            &origin.target,
            "ws",
            "folder-1",
            "cfg",
            "ins",
            None,
            Some(&preset),
            "2026-09-27T00:00:00.000Z",
            &origin,
        )
        .unwrap();
        assert_eq!(genesis.string_field("identity_profile"), None);
        assert_eq!(genesis.min_reader(), Some(2));
        let value: Value = serde_json::from_slice(&genesis.canonical_bytes().unwrap()).unwrap();
        assert_eq!(
            tools::ResolvedPreset::from_genesis(&value["preset"]).unwrap(),
            preset
        );

        // A general workspace without a request keeps the identity genesis; a
        // blank one freezes the sugar under the blank base, never classifying.
        assert_eq!(
            session_preset(
                None,
                None,
                tools::PresetBase::General,
                &tools::NoPresetEnvironment
            )
            .unwrap(),
            None
        );
        assert_eq!(
            session_preset(
                None,
                None,
                tools::PresetBase::Blank,
                &tools::NoPresetEnvironment
            )
            .unwrap(),
            Some(tools::ResolvedPreset::from_identity(
                None,
                tools::PresetBase::Blank
            ))
        );
        assert!(matches!(
            session_preset(
                Some(&json!({"layers":[{"kind":"kernel","id":"video"}]})),
                None,
                tools::PresetBase::General,
                &tools::NoPresetEnvironment
            ),
            Err(ManagementError::PresetInvalid(tools::PresetInvalid {
                reason: tools::PresetInvalidReason::UnknownKernelLayer,
                ..
            }))
        ));
        assert!(matches!(
            session_preset(
                Some(&json!({"layers":7})),
                None,
                tools::PresetBase::General,
                &tools::NoPresetEnvironment
            ),
            Err(ManagementError::PresetBadRequest(_))
        ));
    }

    #[test]
    fn legacy_session_create_without_folder_binding_remains_recoverable() {
        assert!(folder_binding_matches_snapshot(None, ""));
        assert!(folder_binding_matches_snapshot(Some("primary"), ""));
        assert!(folder_binding_matches_snapshot(
            Some("secondary"),
            "secondary"
        ));
        assert!(!folder_binding_matches_snapshot(None, "secondary"));
        assert!(!folder_binding_matches_snapshot(
            Some("primary"),
            "secondary"
        ));
    }

    #[test]
    fn quarantined_fork_does_not_block_recovery_or_claim_success() {
        let root = tempfile::tempdir().unwrap();
        let timestamp = "2026-10-04T00:00:00.000Z";
        let management = ManagementStore::open_at(root.path(), timestamp).unwrap();
        let store = ThreadStore::open(root.path()).unwrap();
        let source = "018f0000-0000-7000-8000-000000000003";
        let dest = "018f0000-0000-7000-8000-000000000004";
        let folder = root.path().join("threads").join(source);
        fs::create_dir(&folder).unwrap();
        let baseline = include_bytes!("../../../fixtures/invalid/_valid-baseline.jsonl");
        let mut events: Vec<Value> = baseline
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice(line).unwrap())
            .collect();
        events
            .iter_mut()
            .find(|event| event["kind"] == "run_start")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("launch_bindings_digest".into(), json!("a".repeat(64)));
        let mut ledger = Vec::new();
        for event in events {
            ledger.extend(serde_json_canonicalizer::to_vec(&event).unwrap());
            ledger.push(b'\n');
        }
        fs::write(folder.join("main.jsonl"), &ledger).unwrap();
        store
            .begin_fork("failed-fork", source, dest, timestamp)
            .unwrap();
        let record = OperationRecord {
            v: VERSION,
            rpc_id: "failed-fork-rpc".into(),
            operation: "session.fork".into(),
            request_sha256: "0".repeat(64),
            phase: OperationPhase::Carriers,
            started_at: timestamp.into(),
            response: None,
            intent: to_ijson(&ForkIntent {
                kind: "fork".into(),
                source: source.into(),
                dest: dest.into(),
                at_endpoint_seq: None,
                kernel_anchor: 8,
                rewrite_op_id: "failed-fork".into(),
                principal: "uid:501".into(),
                ephemeral: false,
                preset: None,
            })
            .unwrap(),
        };
        let path = management.operation_path(&record.rpc_id);
        publish_canonical(&path, &record).unwrap();
        store.recover_rewrites_for_startup().unwrap();
        management.recover().unwrap();
        ManagementStore::open_at(root.path(), timestamp).unwrap();
        assert!(matches!(
            management.drive_fork(&path, record),
            Err(ManagementError::CorruptOperation(_))
        ));
        assert!(
            read_canonical::<OperationRecord>(&path)
                .unwrap()
                .response
                .is_none()
        );
        assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), ledger);
        assert!(!root.path().join("threads").join(dest).exists());
    }

    /// E831 E2 / K2: a config writer that does not touch the endpoint metadata (policy set,
    /// rename, any direct publication) must not make the next baseline fail. `open_light`
    /// heals the metadata from config alone and never reads a thread file.
    #[test]
    fn open_light_heals_metadata_after_any_config_writer_without_reading_threads() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join("project");
        std::fs::create_dir(&project).expect("project");
        let config = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "ws-light".to_owned(),
            name: "Light".to_owned(),
            cwd: Vec::new(),
            folders: vec![WorkspaceFolder {
                id: "folder-0001".to_owned(),
                path: project.to_string_lossy().into_owned(),
            }],
            policy: None,
        };
        let repository = ConfigRepository::open(root.path()).expect("config");
        repository.publish_workspace(0, &config).expect("workspace");
        // boot: the recovering open creates the metadata (synthetic operation)
        let recovering = ManagementStore::open_at(root.path(), "2026-10-12T00:00:00.000Z")
            .expect("recovering open");
        let sessions: Vec<SessionInventoryItem> = Vec::new();
        recovering
            .list_workspaces(&sessions)
            .expect("baseline after boot");

        // thread folders that must never be read by the light path
        let trap = root
            .path()
            .join("threads/018f0000-0000-7000-8000-0000000000f1");
        std::fs::create_dir_all(&trap).expect("trap");
        std::fs::set_permissions(&trap, std::fs::Permissions::from_mode(0o000)).expect("chmod");

        // 1. policy set (bumps the config revision, never touches the metadata)
        let current = repository.workspace("ws-light").expect("current");
        repository
            .publish_workspace_policy(
                "ws-light",
                current.revision,
                default_workspace_policy(project.to_str().unwrap()),
            )
            .expect("policy set");
        let light = ManagementStore::open_light(root.path()).expect("light open");
        let listed = light
            .list_workspaces(&sessions)
            .expect("baseline right after policy.set");
        assert_eq!(listed.items.len(), 1);

        // 2. another writer: rename (revision bump, different digest)
        let mut renamed = repository.workspace("ws-light").expect("current");
        renamed.name = "Renamed".to_owned();
        renamed.revision += 1;
        repository
            .publish_workspace(renamed.revision - 1, &renamed)
            .expect("rename");
        let listed = ManagementStore::open_light(root.path())
            .expect("light open")
            .list_workspaces(&sessions)
            .expect("baseline right after rename");
        assert_eq!(listed.items[0].title, "Renamed");

        // 3. a writer that lands after the light open: list_workspaces itself retries once
        let mut third = repository.workspace("ws-light").expect("current");
        third.name = "Third".to_owned();
        third.revision += 1;
        let stale_view = ManagementStore::open_light(root.path()).expect("light open");
        repository
            .publish_workspace(third.revision - 1, &third)
            .expect("third writer");
        let listed = stale_view
            .list_workspaces(&sessions)
            .expect("list_workspaces heals once and succeeds");
        assert_eq!(listed.items[0].title, "Third");

        // 4. missing metadata is rebuilt; created_at survives when the file existed
        let metadata_path = light.metadata_path("ws-light");
        let before: WorkspaceMetadata = read_canonical(&metadata_path).expect("metadata");
        std::fs::remove_file(&metadata_path).expect("remove metadata");
        let healed = ManagementStore::open_light_at(root.path(), "2026-10-12T05:00:00.000Z")
            .expect("light open")
            .read_metadata("ws-light")
            .expect("rebuilt");
        assert_eq!(healed.created_at, "2026-10-12T05:00:00.000Z");
        assert_eq!(healed.path, before.path);
        std::fs::set_permissions(&trap, std::fs::Permissions::from_mode(0o700)).expect("restore");
    }

    #[test]
    fn unavailable_workspace_keeps_its_management_operation_pending() {
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join("removable-project");
        std::fs::create_dir(&project).expect("project");
        let store = ManagementStore::open_at(root.path(), "2026-09-01T00:00:00.000Z")
            .expect("management store");
        let config = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "workspace-offline".to_owned(),
            name: "Offline".to_owned(),
            cwd: Vec::new(),
            folders: vec![WorkspaceFolder {
                id: "folder-0001".to_owned(),
                path: project.to_string_lossy().into_owned(),
            }],
            policy: None,
        };
        store
            .begin_workspace_operation(WorkspaceOperationWrite {
                rpc_id: "workspace-offline-create",
                request_sha256: &"0".repeat(64),
                operation: "workspace.create",
                action: "create",
                config: &config,
                started_at: "2026-09-01T00:00:00.000Z",
                created: Some(true),
                relocation: None,
                folder: None,
            })
            .expect("prepared operation");
        std::fs::remove_dir(&project).expect("remove project");

        ManagementStore::open_at(root.path(), "2026-09-01T00:00:01.000Z")
            .expect("an unavailable workspace must not block endpoint recovery");
        assert!(store.operation_path("workspace-offline-create").is_file());
    }

    #[test]
    fn a_select_model_payload_staged_before_the_rename_is_read_on_recovery() {
        let root = tempfile::tempdir().expect("root");
        let store = ManagementStore::open_at(root.path(), "2026-09-01T00:00:00.000Z")
            .expect("management store");
        let rpc_id = "rpc-select-model-staged-before-rename";
        let payload = store.operation_payload_path(rpc_id);
        std::fs::create_dir_all(&payload).expect("payload");
        // Exactly what a crash during the rename release left behind: the
        // candidate document under the superseded name.
        let candidate = serde_json::json!({
            "format": 1, "revision": 2, "provider": "fixture", "model": "gpt-5"
        });
        let candidate_bytes = {
            let mut bytes = serde_json_canonicalizer::to_vec(&candidate).expect("canonical");
            bytes.push(b'\n');
            bytes
        };
        std::fs::write(payload.join("session-settings-v1.json"), &candidate_bytes)
            .expect("staged candidate");
        let record = serde_json::json!({
            "v": VERSION,
            "rpc_id": rpc_id,
            "operation": "session.selectModel",
            "request_sha256": "0".repeat(64),
            "phase": "prepared",
            "started_at": "2026-09-01T00:00:00.000Z",
            "intent": {
                "kind": "select-model",
                "session_id": "018f0000-0000-7000-8000-000000000001",
                "expected_revision": 1,
                "next_revision": 2,
                "provider": "fixture",
                "model": "gpt-5"
            }
        });
        let operation_path = store.operation_path(rpc_id);
        std::fs::create_dir_all(operation_path.parent().expect("prefix")).expect("prefix");
        let mut record_bytes = serde_json_canonicalizer::to_vec(&record).expect("canonical record");
        record_bytes.push(b'\n');
        std::fs::write(&operation_path, &record_bytes).expect("operation record");

        // Recovery runs on open. The session folder is absent, so the
        // operation cannot complete and stays pending -- but it must have read
        // its staged candidate, which means retiring the superseded name.
        let recovered = ManagementStore::open_at(root.path(), "2026-09-01T00:00:01.000Z");
        // The session folder is absent, so the operation cannot finish; the
        // failure must be about that, never about reading the candidate.
        let message = recovered
            .err()
            .map(|error| error.to_string())
            .unwrap_or_default();
        assert!(
            !message.contains("select-model") && !message.contains("No such file"),
            "recovery stopped before reading the staged candidate: {message}"
        );
        assert!(
            payload.join("session-settings.json").is_file(),
            "the staged candidate is retired under the current name on recovery"
        );
        assert!(!payload.join("session-settings-v1.json").exists());
        assert_eq!(
            std::fs::read(payload.join("session-settings.json")).expect("candidate"),
            candidate_bytes,
            "the staged bytes are unchanged"
        );
    }

    #[test]
    fn a_created_workspace_starts_with_the_interactive_fixed_catalog_writable_in_its_folder() {
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join("seeded-project");
        std::fs::create_dir(&project).expect("project");
        let store = ManagementStore::open_at(root.path(), "2026-09-14T00:00:00.000Z")
            .expect("management store");
        let (view, created) = store
            .create_workspace(
                "seeded-create",
                &"2".repeat(64),
                &project.to_string_lossy(),
                "2026-09-14T00:00:00.000Z",
            )
            .expect("workspace create");
        assert!(created);
        let config = store.workspace_configs().expect("configs")[&view.workspace_id].clone();
        let policy = config.policy.expect("seeded policy");
        assert_eq!(policy.writable_roots, vec![view.path.clone()]);
        assert!(policy.network);
        assert_eq!(
            policy.allowed_tools,
            tools::BuiltinManifest::compiled().interactive_names()
        );
        for role_tool in ["plan", "summary_artifact", "verify", "report"] {
            assert!(!policy.allowed_tools.iter().any(|name| name == role_tool));
        }
        for tool in ["read", "apply_patch", "shell", "grep", "glob"] {
            assert!(policy.allowed_tools.iter().any(|name| name == tool));
        }
        let resolved = profile::ConfigRepository::open(root.path())
            .expect("repository")
            .resolve(&view.workspace_id)
            .expect("resolved workspace");
        assert_eq!(
            resolved.workspace.policy.allowed_tools,
            policy.allowed_tools
        );
        assert_eq!(
            resolved.workspace.policy.writable_roots,
            policy.writable_roots
        );
    }

    #[test]
    fn opening_the_store_seeds_a_policy_for_a_policy_less_workspace_once() {
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join("legacy-project");
        std::fs::create_dir(&project).expect("project");
        let store = ManagementStore::open_at(root.path(), "2026-09-14T00:00:00.000Z")
            .expect("management store");
        let (view, _) = store
            .create_workspace(
                "legacy-create",
                &"5".repeat(64),
                &project.to_string_lossy(),
                "2026-09-14T00:00:00.000Z",
            )
            .expect("workspace create");
        // Rewrite the document the way a pre-seed Kernel left it: no policy.
        let path = root
            .path()
            .join("workspaces")
            .join(&view.workspace_id)
            .join("workspace.json");
        let mut document: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).expect("document")).expect("json");
        document.as_object_mut().expect("object").remove("policy");
        let mut bytes = serde_json_canonicalizer::to_vec(&document).expect("canonical");
        bytes.push(b'\n');
        std::fs::write(&path, bytes).expect("rewrite");
        drop(store);

        let reopened = ManagementStore::open_at(root.path(), "2026-09-14T00:00:01.000Z")
            .expect("reopen seeds the policy");
        let config = reopened.workspace_configs().expect("configs")[&view.workspace_id].clone();
        let policy = config.policy.expect("seeded on open");
        assert_eq!(policy.writable_roots, vec![view.path.clone()]);
        assert_eq!(
            policy.allowed_tools,
            tools::BuiltinManifest::compiled().interactive_names()
        );
        let revision_after_seed = config.revision;
        drop(reopened);

        // An authored policy (including the seed itself) is left alone on later opens.
        let again = ManagementStore::open_at(root.path(), "2026-09-14T00:00:02.000Z")
            .expect("reopen is idempotent");
        let config = again.workspace_configs().expect("configs")[&view.workspace_id].clone();
        assert_eq!(config.revision, revision_after_seed);
    }

    #[test]
    fn relocation_moves_writable_roots_with_their_folder() {
        let root = tempfile::tempdir().expect("root");
        let before = root.path().join("before");
        std::fs::create_dir(&before).expect("before");
        let store = ManagementStore::open_at(root.path(), "2026-09-14T00:00:00.000Z")
            .expect("management store");
        let (view, _) = store
            .create_workspace(
                "relocate-create",
                &"3".repeat(64),
                &before.to_string_lossy(),
                "2026-09-14T00:00:00.000Z",
            )
            .expect("workspace create");
        let after = root.path().join("after");
        std::fs::rename(&before, &after).expect("move folder");
        let moved = store
            .relocate_workspace(
                "relocate-move",
                &"4".repeat(64),
                &view.workspace_id,
                &view.path,
                &after.to_string_lossy(),
                "2026-09-14T00:00:01.000Z",
            )
            .expect("relocate");
        let config = store.workspace_configs().expect("configs")[&view.workspace_id].clone();
        let policy = config.policy.expect("policy survives relocation");
        assert_eq!(policy.writable_roots, vec![moved.path.clone()]);
        assert_eq!(
            policy.allowed_tools,
            tools::BuiltinManifest::compiled().interactive_names()
        );
    }

    fn folder_store(name: &str) -> (tempfile::TempDir, ManagementStore, WorkspaceView) {
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join(name);
        std::fs::create_dir(&project).expect("project");
        let store = ManagementStore::open_at(root.path(), "2026-10-08T00:00:00.000Z")
            .expect("management store");
        let (view, _) = store
            .create_workspace(
                &format!("{name}-create"),
                &"5".repeat(64),
                &project.to_string_lossy(),
                "2026-10-08T00:00:00.000Z",
            )
            .expect("workspace create");
        (root, store, view)
    }

    fn sibling(root: &tempfile::TempDir, name: &str) -> String {
        let path = root.path().join(name);
        std::fs::create_dir_all(&path).expect("sibling");
        path.canonicalize()
            .expect("canonical")
            .to_string_lossy()
            .into_owned()
    }

    /// Publishes a minimal active session folder whose genesis names
    /// `workspace_id` and, when given, a stable folder binding.
    fn write_session(root: &Path, session_id: &str, workspace_id: &str, binding: Option<&str>) {
        let origin = schema::OriginTuple {
            principal: "test".to_owned(),
            client: "session-endpoint".to_owned(),
            target: session_id.to_owned(),
            op: "session.create".to_owned(),
            key: session_id.to_owned(),
        };
        let genesis = create_genesis(
            session_id,
            workspace_id,
            binding.unwrap_or("folder-0001"),
            &format!("sha256-{}", "a".repeat(64)),
            &format!("sha256-{}", "b".repeat(64)),
            None,
            None,
            "2026-10-08T00:00:00.000Z",
            &origin,
        )
        .expect("genesis");
        let mut value: Value =
            serde_json::from_slice(&genesis.canonical_bytes().expect("bytes")).expect("json");
        if binding.is_none() {
            value
                .as_object_mut()
                .expect("object")
                .remove("folder_binding");
        }
        let event = schema::Event::from_value(
            schema::IJsonValue::parse(&serde_json::to_vec(&value).expect("json")).expect("ijson"),
        )
        .expect("event");
        let folder = root.join("threads").join(session_id);
        std::fs::create_dir_all(&folder).expect("session folder");
        std::fs::write(
            folder.join("main.jsonl"),
            canonical_event_line(&event).expect("line"),
        )
        .expect("ledger");
    }

    #[test]
    fn folders_are_added_listed_and_removed_with_their_writable_roots() {
        let (root, store, view) = folder_store("folders-primary");
        let second = sibling(&root, "folders-second");
        let (added, folders) = store
            .add_workspace_folder(
                "folders-add",
                &"6".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:01.000Z",
            )
            .expect("add folder");
        assert_eq!(added.path, view.path, "the primary folder is unchanged");
        assert_eq!(folders.revision, 2);
        assert_eq!(
            folders
                .folders
                .iter()
                .map(|folder| (folder.folder_id.as_str(), folder.path.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("folder-0001", view.path.as_str()),
                ("folder-0002", second.as_str())
            ]
        );
        assert_eq!(
            store.workspace_folders(&view.workspace_id).unwrap(),
            folders
        );
        let config = store.workspace_configs().unwrap()[&view.workspace_id].clone();
        let mut expected_roots = vec![view.path.clone(), second.clone()];
        expected_roots.sort();
        assert_eq!(config.policy.unwrap().writable_roots, expected_roots);

        // An exact retry replays the completed record, although the path is
        // now a folder and the precondition would fail on a fresh request.
        let (_, replayed) = store
            .add_workspace_folder(
                "folders-add",
                &"6".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:09.000Z",
            )
            .expect("exact retry");
        assert_eq!(replayed, folders);
        assert!(matches!(
            store.add_workspace_folder(
                "folders-add",
                &"7".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:09.000Z",
            ),
            Err(ManagementError::IdempotencyConflict { .. })
        ));
        assert!(matches!(
            store.add_workspace_folder(
                "folders-add-again",
                &"8".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:02.000Z",
            ),
            Err(ManagementError::WorkspaceAmbiguous(_))
        ));
        // A secondary folder cannot root a second workspace either.
        assert!(matches!(
            store.create_workspace(
                "folders-create-secondary",
                &"9".repeat(64),
                &second,
                "2026-10-08T00:00:02.000Z",
            ),
            Err(ManagementError::InvalidPath(_))
        ));

        // Removing the primary folder promotes the next one; its roots go.
        let (removed, remaining) = store
            .remove_workspace_folder(
                "folders-remove",
                &"a".repeat(64),
                &view.workspace_id,
                &view.path,
                "2026-10-08T00:00:03.000Z",
            )
            .expect("remove folder");
        assert_eq!(removed.path, second);
        assert_eq!(remaining.revision, 3);
        assert_eq!(remaining.folders.len(), 1);
        assert_eq!(remaining.folders[0].folder_id, "folder-0002");
        let config = store.workspace_configs().unwrap()[&view.workspace_id].clone();
        assert_eq!(config.policy.unwrap().writable_roots, vec![second.clone()]);
        assert!(matches!(
            store.remove_workspace_folder(
                "folders-remove-last",
                &"b".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:04.000Z",
            ),
            Err(ManagementError::LastFolder(_))
        ));

        // A reopened store recovers nothing and keeps the metadata current.
        drop(store);
        let reopened =
            ManagementStore::open_at(root.path(), "2026-10-08T00:01:00.000Z").expect("reopen");
        assert_eq!(
            reopened.workspace_folders(&view.workspace_id).unwrap(),
            remaining
        );
    }

    #[test]
    fn a_folder_bound_to_a_session_is_not_removed() {
        let (root, store, view) = folder_store("bound-primary");
        let second = sibling(&root, "bound-second");
        store
            .add_workspace_folder(
                "bound-add",
                &"6".repeat(64),
                &view.workspace_id,
                &second,
                "2026-10-08T00:00:01.000Z",
            )
            .expect("add folder");
        let session = "018f0000-0000-7000-8000-0000000000b1";
        write_session(
            root.path(),
            session,
            &view.workspace_id,
            Some("folder-0002"),
        );
        match store.remove_workspace_folder(
            "bound-remove",
            &"a".repeat(64),
            &view.workspace_id,
            &second,
            "2026-10-08T00:00:02.000Z",
        ) {
            Err(ManagementError::FolderInUse { session_ids, .. }) => {
                assert_eq!(session_ids, vec![session.to_owned()]);
            }
            other => panic!("expected folder-in-use, got {other:?}"),
        }
        assert!(
            !store.operation_path("bound-remove").exists(),
            "a refused removal authors no operation record"
        );
        assert_eq!(
            store
                .workspace_folders(&view.workspace_id)
                .unwrap()
                .folders
                .len(),
            2
        );
    }

    #[test]
    fn a_legacy_unbound_session_blocks_a_second_folder() {
        let (root, store, view) = folder_store("legacy-primary");
        let second = sibling(&root, "legacy-second");
        let session = "018f0000-0000-7000-8000-0000000000c1";
        write_session(root.path(), session, &view.workspace_id, None);
        match store.add_workspace_folder(
            "legacy-add",
            &"6".repeat(64),
            &view.workspace_id,
            &second,
            "2026-10-08T00:00:01.000Z",
        ) {
            Err(ManagementError::LegacySessions { session_ids, .. }) => {
                assert_eq!(session_ids, vec![session.to_owned()]);
            }
            other => panic!("expected legacy-session refusal, got {other:?}"),
        }
        assert_eq!(
            store
                .workspace_folders(&view.workspace_id)
                .unwrap()
                .folders
                .len(),
            1
        );
    }

    #[test]
    fn folder_edits_materialize_a_legacy_cwd_workspace_with_its_synthesized_ids() {
        let legacy = WorkspaceConfig {
            format: 1,
            revision: 4,
            id: "legacy".to_owned(),
            name: "legacy".to_owned(),
            cwd: vec!["/a".to_owned(), "/b".to_owned()],
            folders: Vec::new(),
            policy: None,
        };
        let added = super::with_added_folder(&legacy, "folder-0003", "/c");
        assert!(added.cwd.is_empty());
        assert_eq!(
            added
                .folders
                .iter()
                .map(|folder| (folder.id.as_str(), folder.path.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("folder-0001", "/a"),
                ("folder-0002", "/b"),
                ("folder-0003", "/c")
            ]
        );
        for path in ["/a", "/b"] {
            assert_eq!(legacy.binding_for_path(path), added.binding_for_path(path));
        }
        assert_eq!(added.revision, 5);
        assert!(super::with_removed_folder(&legacy, "folder-0001", "/b").is_none());
    }

    #[test]
    fn completed_workspace_operation_never_reopens_its_old_path() {
        let root = tempfile::tempdir().expect("root");
        let project = root.path().join("completed-project");
        std::fs::create_dir(&project).expect("project");
        let store = ManagementStore::open_at(root.path(), "2026-09-01T00:00:00.000Z")
            .expect("management store");
        store
            .create_workspace(
                "completed-workspace-create",
                &"1".repeat(64),
                &project.to_string_lossy(),
                "2026-09-01T00:00:00.000Z",
            )
            .expect("complete workspace create");
        std::fs::remove_dir(&project).expect("remove project");

        ManagementStore::open_at(root.path(), "2026-09-01T00:00:01.000Z")
            .expect("completed history must not require its old path");
    }
}

/// K1: the periodic recovery tick, `workspace.policy.set` and a workspace operation used to
/// race on one workspace document; the loser left an operation that could never complete and
/// every later boot failed on it.
#[cfg(test)]
mod k1_management_race {
    use super::*;
    use profile::{ConfigRepository, WorkspaceConfig, WorkspaceFolder};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    const BOOT: &str = "2026-10-13T05:38:06.787Z";
    const TICK: &str = "2026-10-13T05:38:27.837Z";
    const LATER: &str = "2026-10-13T06:00:06.000Z";
    const WORKSPACE: &str = "ws-k1";

    fn fixture(root: &Path) -> (ConfigRepository, PathBuf) {
        let project = root.join("project");
        fs::create_dir(&project).expect("project");
        let folder = project.to_string_lossy().into_owned();
        let repository = ConfigRepository::open(root).expect("config");
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: WORKSPACE.to_owned(),
                    name: "K1".to_owned(),
                    cwd: Vec::new(),
                    folders: vec![WorkspaceFolder {
                        id: "folder-0001".to_owned(),
                        path: folder.clone(),
                    }],
                    policy: Some(default_workspace_policy(&folder)),
                },
            )
            .expect("workspace rev 1");
        (repository, project)
    }

    fn policy_with(project: &Path, extra: &str) -> WorkspacePolicy {
        let mut policy = default_workspace_policy(&project.to_string_lossy());
        policy.allowed_tools.push(extra.to_owned());
        policy
    }

    fn set_policy(repository: &ConfigRepository, project: &Path, extra: &str) {
        let revision = repository.workspace(WORKSPACE).expect("workspace").revision;
        repository
            .publish_workspace_policy(WORKSPACE, revision, policy_with(project, extra))
            .expect("policy set");
    }

    fn quarantine_root(root: &Path) -> PathBuf {
        store::endpoint_management_root(root)
            .expect("management root")
            .join("quarantine")
    }

    fn stale_metadata(store: &ManagementStore) {
        let path = store.metadata_path(WORKSPACE);
        let mut metadata: WorkspaceMetadata = read_canonical(&path).expect("metadata");
        metadata.config_digest = "0".repeat(64);
        publish_canonical(&path, &metadata).expect("stale metadata");
    }

    fn metadata_matches(store: &ManagementStore, id: &str) -> bool {
        let config = store.workspace_configs().expect("configs")[id].clone();
        verify_metadata(&config, &store.read_metadata(id).expect("metadata")).is_ok()
    }

    /// Prepares a rename of the current config and drives it to Carriers, then lets the live
    /// config move on by two revisions: the Carriers check can never pass.
    fn stuck_carriers_record(
        store: &ManagementStore,
        repository: &ConfigRepository,
        project: &Path,
        rpc_id: &str,
    ) -> PathBuf {
        let mut config = store.workspace_configs().expect("configs")[WORKSPACE].clone();
        config.revision += 1;
        config.name = "K1 renamed".to_owned();
        let digest = hex_digest(&canonical_line(&config).expect("line"));
        let (path, mut record) = store
            .begin_workspace_operation(WorkspaceOperationWrite {
                rpc_id,
                request_sha256: &digest,
                operation: "workspace.rename",
                action: "rename",
                config: &config,
                started_at: TICK,
                created: None,
                relocation: None,
                folder: None,
            })
            .expect("prepared");
        record.phase = OperationPhase::Carriers;
        publish_canonical(&path, &record).expect("carriers");
        set_policy(repository, project, "mcp__a__a");
        set_policy(repository, project, "mcp__b__b");
        path
    }

    /// The tick used to stage a synthetic `workspace.rename` for stale metadata, which a policy
    /// set between its snapshot and its Carriers check broke. It now heals the metadata in place
    /// and leaves no operation behind.
    #[test]
    fn tick_heals_stale_metadata_without_staging_an_operation() {
        let root = tempfile::tempdir().expect("root");
        let (repository, project) = fixture(root.path());
        let store = ManagementStore::open_at(root.path(), BOOT).expect("boot");
        let before = store.operation_paths().expect("operations");
        set_policy(&repository, &project, "mcp__a__a");
        stale_metadata(&store);
        assert!(!metadata_matches(&store, WORKSPACE));

        let mut validated = CompleteOperations::default();
        let tick = ManagementStore::open_recovering_again(root.path(), TICK, None, &mut validated)
            .expect("tick");

        assert!(
            metadata_matches(&tick, WORKSPACE),
            "the tick healed the metadata"
        );
        assert_eq!(
            tick.operation_paths().expect("operations"),
            before,
            "no synthetic operation is staged"
        );
        assert!(!quarantine_root(root.path()).exists());
        set_policy(&repository, &project, "mcp__b__b");
        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");
    }

    /// The unmodified production functions on two threads: one loops `workspace.policy.set`
    /// publications, the other loops the recovering open the 30 s tick calls.
    #[test]
    fn tick_never_fails_while_policy_set_publishes() {
        let root = tempfile::tempdir().expect("root");
        let (_repository, project) = fixture(root.path());
        ManagementStore::open_at(root.path(), BOOT).expect("boot");
        let stop = Arc::new(AtomicBool::new(false));
        let writer = {
            let stop = Arc::clone(&stop);
            let root = root.path().to_path_buf();
            std::thread::spawn(move || {
                let repository = ConfigRepository::open(&root).expect("config");
                let mut writes = 0_u64;
                while !stop.load(Ordering::Acquire) {
                    let current = repository.workspace(WORKSPACE).expect("workspace");
                    let _ = repository.publish_workspace_policy(
                        WORKSPACE,
                        current.revision,
                        policy_with(&project, &format!("mcp__t__{writes}")),
                    );
                    writes += 1;
                }
                writes
            })
        };
        let mut validated = CompleteOperations::default();
        let mut failures = 0_u32;
        for _ in 0..100 {
            if ManagementStore::open_recovering_again(root.path(), TICK, None, &mut validated)
                .is_err()
            {
                failures += 1;
            }
        }
        stop.store(true, Ordering::Release);
        let writes = writer.join().expect("writer");
        assert_eq!(failures, 0, "no tick fails");
        assert!(writes > 20, "the writer ran concurrently");
        assert!(!quarantine_root(root.path()).exists());
        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");
    }

    /// The F148 parked shape, generated. The 0.4.4 tick staged a synthetic `workspace.rename` of
    /// the unchanged live config (no revision bump) for stale metadata; policy sets then moved
    /// the live document on before the Carriers check. Names are generated and every digest is
    /// computed by production code, so the state is self-consistent. Returns the record path, the
    /// record bytes and the staged payload bytes.
    fn parked_synthetic_rename(root: &Path) -> (PathBuf, Vec<u8>, Vec<u8>) {
        let (repository, project) = fixture(root);
        let store = ManagementStore::open_at(root, BOOT).expect("boot");
        // Size realism without real names: a generated tool list of the product's order of size.
        let mut policy = default_workspace_policy(&project.to_string_lossy());
        policy.allowed_tools = (0..232)
            .map(|index| format!("mcp__fixture_server__tool_{index:03}"))
            .collect();
        let revision = repository.workspace(WORKSPACE).expect("workspace").revision;
        repository
            .publish_workspace_policy(WORKSPACE, revision, policy)
            .expect("policy set");
        stale_metadata(&store);
        let config = store.workspace_configs().expect("configs")[WORKSPACE].clone();
        assert_eq!(config.revision, 2, "staged at the live revision, no bump");
        let digest = hex_digest(&canonical_line(&config).expect("line"));
        // The id shape the 0.4.4 tick used.
        let rpc_id = format!("synthetic-workspace-rename-{WORKSPACE}-{digest}");
        let (path, mut record) = store
            .begin_workspace_operation(WorkspaceOperationWrite {
                rpc_id: &rpc_id,
                request_sha256: &digest,
                operation: "workspace.rename",
                action: "rename",
                config: &config,
                started_at: TICK,
                created: None,
                relocation: None,
                folder: None,
            })
            .expect("prepared");
        record.phase = OperationPhase::Carriers;
        publish_canonical(&path, &record).expect("carriers");
        for extra in [
            "mcp__fixture_server__a",
            "mcp__fixture_server__b",
            "mcp__fixture_server__c",
        ] {
            set_policy(&repository, &project, extra);
        }
        let staged =
            fs::read(store.operation_payload_path(&rpc_id).join("workspace.json")).expect("staged");
        let bytes = fs::read(&path).expect("record bytes");
        (path, bytes, staged)
    }

    /// The parked F148 state (a synthetic `workspace.rename` stuck in `carriers`, staged at
    /// revision 2) against a live workspace that has since moved on to revision 5.
    #[test]
    fn boot_quarantines_the_parked_carriers_record_and_succeeds() {
        let root = tempfile::tempdir().expect("root");
        let (path, record, staged) = parked_synthetic_rename(root.path());
        let repository = ConfigRepository::open(root.path()).expect("config");
        assert_eq!(repository.workspace(WORKSPACE).expect("live").revision, 5);
        let digest = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();

        // Without the quarantine this is the exit-74 boot failure of the parked root.
        let boot = ManagementStore::open_at_with_queue_driver(root.path(), LATER, None)
            .expect("boot quarantines the record and continues");

        assert!(!path.exists(), "the record left the recovery set");
        let target = quarantine_root(root.path()).join(&digest);
        assert_eq!(
            fs::read(target.join("record.json")).expect("record"),
            record
        );
        assert_eq!(
            fs::read(target.join("payload-root/payload/workspace.json")).expect("payload"),
            staged
        );
        assert_eq!(
            fs::read_to_string(target.join("reason.txt")).expect("reason"),
            "workspace publication does not match prepared revision"
        );
        assert_eq!(
            boot.operation_paths().expect("operations"),
            Vec::<PathBuf>::new()
        );
        assert!(
            !path.with_extension("").exists(),
            "the original payload directory is gone"
        );
        assert!(
            metadata_matches(&boot, WORKSPACE),
            "the workspace metadata was healed"
        );
        // The live workspace is untouched and a second boot is clean.
        assert_eq!(repository.workspace(WORKSPACE).expect("live").revision, 5);
        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("second boot");
        assert_eq!(
            fs::read_dir(quarantine_root(root.path()))
                .expect("quarantine")
                .count(),
            1
        );
    }

    /// A record of another version (a newer Kernel's, after a downgrade) is not this Kernel's to
    /// judge: it is logged and left in place with its payload, never quarantined.
    #[test]
    fn record_of_another_version_is_left_in_place_not_quarantined() {
        let root = tempfile::tempdir().expect("root");
        let (path, _, staged) = parked_synthetic_rename(root.path());
        let mut record: OperationRecord = read_canonical(&path).expect("record");
        record.v = VERSION + 1;
        publish_canonical(&path, &record).expect("newer record");
        let before = fs::read(&path).expect("record bytes");

        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None)
            .expect("boot skips the record");

        assert_eq!(fs::read(&path).expect("record"), before);
        assert_eq!(
            fs::read(path.with_extension("").join("payload/workspace.json")).expect("payload"),
            staged
        );
        assert!(!quarantine_root(root.path()).exists());
    }

    /// The quarantine keeps the whole payload, the record is never driven again, and earlier
    /// evidence for the same digest is never overwritten.
    #[test]
    fn quarantine_keeps_the_payload_and_never_redrives_the_operation() {
        let root = tempfile::tempdir().expect("root");
        let (repository, project) = fixture(root.path());
        let store = ManagementStore::open_at(root.path(), BOOT).expect("boot");
        let path = stuck_carriers_record(&store, &repository, &project, "rpc-stuck");
        let digest = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();
        let original = fs::read(&path).expect("record bytes");
        let staged = fs::read(
            store
                .operation_payload_path("rpc-stuck")
                .join("workspace.json"),
        )
        .expect("staged");
        // An unfinished copy left by an interrupted quarantine must not leak into the new one.
        let partial = quarantine_root(root.path()).join(&digest);
        fs::create_dir_all(partial.join("payload-root/stale")).expect("partial");
        fs::write(partial.join("payload-root/stale/leftover"), b"x").expect("leftover");
        let revision = repository.workspace(WORKSPACE).expect("live").revision;

        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");

        let first = quarantine_root(root.path()).join(&digest);
        assert_eq!(
            fs::read(first.join("record.json")).expect("record"),
            original
        );
        assert_eq!(
            fs::read(first.join("payload-root/payload/workspace.json")).expect("payload"),
            staged
        );
        assert!(!first.join("payload-root/stale").exists());
        assert!(!path.exists());
        assert!(!store.operation_payload_path("rpc-stuck").exists());
        // Never re-driven: the live config was not touched by another publication.
        assert_eq!(
            repository.workspace(WORKSPACE).expect("live").revision,
            revision
        );
        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");
        assert_eq!(
            fs::read(first.join("record.json")).expect("record"),
            original
        );

        // The same rpc id stuck again lands beside the first, not on top of it.
        let again = stuck_carriers_record(&store, &repository, &project, "rpc-stuck");
        assert_eq!(again, path);
        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");
        let second = quarantine_root(root.path()).join(format!("{digest}-1"));
        assert!(second.join("record.json").is_file());
        assert!(second.join("payload-root/payload/workspace.json").is_file());
        assert_eq!(
            fs::read(first.join("record.json")).expect("record"),
            original
        );
    }

    /// A completed workspace record without its response can never serve a retry either.
    #[test]
    fn complete_workspace_record_without_response_is_quarantined() {
        let root = tempfile::tempdir().expect("root");
        let (_repository, _project) = fixture(root.path());
        let store = ManagementStore::open_at(root.path(), BOOT).expect("boot");
        let config = store.workspace_configs().expect("configs")[WORKSPACE].clone();
        let (path, mut record) = store
            .begin_workspace_operation(WorkspaceOperationWrite {
                rpc_id: "rpc-complete",
                request_sha256: &"1".repeat(64),
                operation: "workspace.rename",
                action: "rename",
                config: &config,
                started_at: TICK,
                created: None,
                relocation: None,
                folder: None,
            })
            .expect("prepared");
        record.phase = OperationPhase::Complete;
        publish_canonical(&path, &record).expect("complete without response");

        ManagementStore::open_at_with_queue_driver(root.path(), LATER, None).expect("boot");

        assert!(!path.exists());
        let digest = path.file_stem().expect("stem");
        assert!(
            quarantine_root(root.path())
                .join(digest)
                .join("record.json")
                .is_file()
        );
    }

    /// One failing record does not stop the pass: the others are recovered and the failure is
    /// still reported. A corrupt operation that is not a workspace operation is not quarantined.
    #[test]
    fn one_failing_record_does_not_stop_the_others_from_recovering() {
        let root = tempfile::tempdir().expect("root");
        let (_repository, _project) = fixture(root.path());
        let store = ManagementStore::open_at(root.path(), BOOT).expect("boot");
        let mut config = store.workspace_configs().expect("configs")[WORKSPACE].clone();
        config.revision += 1;
        config.name = "K1 recovered".to_owned();
        let digest = hex_digest(&canonical_line(&config).expect("line"));
        let (pending, _) = store
            .begin_workspace_operation(WorkspaceOperationWrite {
                rpc_id: "rpc-pending",
                request_sha256: &digest,
                operation: "workspace.rename",
                action: "rename",
                config: &config,
                started_at: TICK,
                created: None,
                relocation: None,
                folder: None,
            })
            .expect("pending rename");
        // A record of an unknown kind that sorts before the pending rename.
        let bogus = (0..)
            .map(|index| format!("rpc-bogus-{index}"))
            .find(|rpc_id| store.operation_path(rpc_id) < pending)
            .expect("bogus rpc id");
        let bogus_path = store.operation_path(&bogus);
        fs::create_dir_all(bogus_path.parent().expect("prefix")).expect("prefix");
        publish_canonical(
            &bogus_path,
            &OperationRecord {
                v: VERSION,
                rpc_id: bogus,
                operation: "bogus".to_owned(),
                request_sha256: "0".repeat(64),
                phase: OperationPhase::Prepared,
                started_at: TICK.to_owned(),
                intent: to_ijson(&json!({"kind":"bogus"})).expect("intent"),
                response: None,
            },
        )
        .expect("bogus record");

        let error = store
            .recover()
            .expect_err("the unknown kind still fails the pass");

        assert!(matches!(error, ManagementError::CorruptOperation(_)));
        assert!(
            bogus_path.is_file(),
            "a non-workspace record is not quarantined"
        );
        assert!(!quarantine_root(root.path()).exists());
        let recovered: OperationRecord = read_canonical(&pending).expect("pending record");
        assert_eq!(recovered.phase, OperationPhase::Complete);
        assert_eq!(
            store.workspace_configs().expect("configs")[WORKSPACE].name,
            "K1 recovered"
        );
    }
}
