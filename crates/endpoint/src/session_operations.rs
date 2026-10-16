//! Crash-recoverable Session lifecycle journal. Semantic ledger authorship
//! remains in the worker/store boundary. No directory registry is maintained.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use profile::ConfigRepository;
use schema::{Event, IJsonValue, OriginTuple, ResumePolicy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{
    AssetStore, AtomicPublisher, DirectoryLock, NamedLock, ThreadStore, scan_valid_prefix,
};
use thiserror::Error;
use uuid::Uuid;

const VERSION: u64 = 3;

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
    execution_digest: String,
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

#[must_use]
pub fn is_endpoint_origin_client(client: &str) -> bool {
    client == ORIGIN_CLIENT
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

/// Model availability for preset `defaults`; the supervisor owns the provider
/// profiles that list each model's reasoning efforts.
pub trait PresetModelSource {
    /// Whether `provider`/`model` is configured and enabled in `config` and,
    /// when given, `effort` is one of its reasoning efforts.
    fn model_available(
        &self,
        config: &profile::ConfigSnapshot,
        provider: &str,
        model: &str,
        effort: Option<&str>,
    ) -> bool;
}

/// The environment a preset resolves in: the instruction snapshot's tool
/// ceiling and skills, and the workspace's MCP registry. `session.create` and
/// `session.presets.resolve` use the same one.
pub struct SessionPresetEnvironment<'a> {
    permitted: Option<std::collections::BTreeSet<String>>,
    /// Skill package names (`ResourceCatalog`), the names the worker's skill
    /// catalog uses; not the snapshot's per-file resource keys.
    skills: std::collections::BTreeSet<String>,
    connections: &'a [schema::McpConnection],
    models: Option<(&'a profile::ConfigSnapshot, &'a dyn PresetModelSource)>,
}

impl<'a> SessionPresetEnvironment<'a> {
    #[must_use]
    pub fn new(
        instruction: &'a profile::InstructionSnapshot,
        connections: &'a [schema::McpConnection],
    ) -> Self {
        Self {
            permitted: instruction
                .effective
                .policy
                .allowed_tools
                .as_ref()
                .map(|tools| tools.iter().cloned().collect()),
            skills: profile::ResourceCatalog::skill_package_names(instruction),
            connections,
            models: None,
        }
    }

    /// Lets the preset's `defaults` name the models `config` provides.
    #[must_use]
    pub fn with_models(
        mut self,
        config: &'a profile::ConfigSnapshot,
        models: Option<&'a dyn PresetModelSource>,
    ) -> Self {
        self.models = models.map(|models| (config, models));
        self
    }
}

impl tools::PresetEnvironment for SessionPresetEnvironment<'_> {
    fn permitted_tools(&self) -> Option<&std::collections::BTreeSet<String>> {
        self.permitted.as_ref()
    }

    fn mcp_server_digest(&self, server: &str) -> Option<String> {
        self.connections
            .iter()
            .find(|connection| connection.name == server)
            .and_then(|connection| serde_json_canonicalizer::to_vec(connection).ok())
            .map(|bytes| format!("sha256-{}", hex_digest(&bytes)))
    }

    fn mcp_tool_name(&self, server: &str, tool: &str) -> Option<String> {
        self.connections
            .iter()
            .find(|connection| connection.name == server)
            .and_then(|_| mcp::project_name(server, tool).ok())
    }

    fn skill_known(&self, name: &str) -> bool {
        self.skills.contains(name)
    }

    fn model_available(&self, provider: &str, model: &str, effort: Option<&str>) -> bool {
        self.models
            .is_some_and(|(config, models)| models.model_available(config, provider, model, effort))
    }
}

/// Inputs shared by the session execution endpoint. No management identities.
pub struct ExecutionCreateOperation<'a> {
    pub rpc_id: &'a str,
    pub request_sha256: &'a str,
    pub requested_session_id: Option<&'a str>,
    pub identity_profile: Option<tools::IdentityProfile>,
    pub preset: Option<&'a Value>,
    pub preset_models: Option<&'a dyn PresetModelSource>,
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
    pub preset_models: Option<&'a dyn PresetModelSource>,
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
    ) -> Result<QueueTransactionDecision, SessionOperationError>;
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
pub struct SessionOperations {
    storage_root: PathBuf,
    root: PathBuf,
    config: ConfigRepository,
}

impl SessionOperations {
    /// Open only the session operation journal. Retired management journals,
    /// workspace metadata and global Folder registries are not inspected.
    pub fn open_sessions(storage_root: impl AsRef<Path>) -> Result<Self, SessionOperationError> {
        let storage_root = storage_root.as_ref().to_path_buf();
        ThreadStore::open(&storage_root)?;
        let root = storage_root.join("session-operations");
        fs::create_dir_all(root.join("operations"))?;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(root.join("lock"))?;
        Ok(Self {
            config: ConfigRepository::open(&storage_root)?,
            storage_root,
            root,
        })
    }

    pub fn recover_sessions(
        &self,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        validated: Option<&mut CompleteOperations>,
    ) -> Result<(), SessionOperationError> {
        self.recover_skipping(queue_driver, validated)
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Re-drive every incomplete config operation before endpoint readiness.
    pub fn recover(&self) -> Result<(), SessionOperationError> {
        self.recover_with_queue_driver(None)
    }

    pub fn recover_with_queue_driver(
        &self,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
    ) -> Result<(), SessionOperationError> {
        self.recover_skipping(queue_driver, None)
    }

    fn recover_skipping(
        &self,
        queue_driver: Option<&dyn QueueRecoveryDriver>,
        mut validated: Option<&mut CompleteOperations>,
    ) -> Result<(), SessionOperationError> {
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
    ) -> Result<(), SessionOperationError> {
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
        if record.v != VERSION {
            return Err(SessionOperationError::CorruptOperation(
                "unsupported Session journal version".into(),
            ));
        }
        let kind = intent_kind(&record)?;
        if record.phase == OperationPhase::Complete {
            if let (Some(known), Some(identity)) = (validated, identity) {
                known.0.insert((path.clone(), identity));
            }
            match kind.as_str() {
                "discard" => {
                    discard_intent(&record)?;
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
                    return Err(SessionOperationError::CorruptOperation(format!(
                        "unknown operation intent kind: {other}"
                    )));
                }
            }
            if record.response.is_none() {
                let error = SessionOperationError::CorruptOperation(
                    "complete operation lacks response".to_owned(),
                );
                return Err(error);
            }
            return Ok(());
        }
        let recovery = match kind.as_str() {
            "discard" => self.drive_discard(&path, record).map(|_| ()),
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
                    SessionOperationError::QueueRecoveryRequired(pending.session_id.clone())
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
                return Err(SessionOperationError::CorruptOperation(format!(
                    "unknown operation intent kind: {other}"
                )));
            }
        };
        recovery?;
        Ok(())
    }

    /// Session execution creation. Only effective values are durable; no named
    /// workspace, Environment or Folder catalog participates in this path.
    pub fn create_execution_session(
        &self,
        request: ExecutionCreateOperation<'_>,
        execution: &profile::ExecutionConfig,
    ) -> Result<String, SessionOperationError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.create"
                || record.request_sha256 != request.request_sha256
            {
                return Err(SessionOperationError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.create".to_owned(),
                });
            }
            return self
                .session_id_from_response(self.drive_session_create(&operation_path, record)?);
        }
        if active_session_count(&self.storage_root)? >= 256 {
            return Err(SessionOperationError::ActiveSessionLimit);
        }
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
            return Err(SessionOperationError::SessionConflict {
                session_id,
                requested_cwd: execution.cwd.clone(),
                existing_cwd: None,
            });
        }
        let execution = self
            .config
            .normalize_execution(execution)
            .map_err(|error| SessionOperationError::InvalidExecution(error.to_string()))?;
        // The launch snapshot is addressed by this Session alone.
        let address = session_id.clone();
        let execution_digest = hex_digest(
            &serde_json_canonicalizer::to_vec(&execution)
                .map_err(|e| SessionOperationError::Canonical(e.to_string()))?,
        );
        let config = self.config.resolve_execution(&address, &execution, None)?;
        let instruction = profile::InstructionResolver::new_scoped(
            request.user_agent_dir,
            self.storage_root
                .join("session-instructions")
                .join(&session_id),
            config.execution.cwd.iter().map(Path::new),
        )
        .capture()?;
        instruction.validate_against_config(&config)?;
        let environment = SessionPresetEnvironment::new(&instruction, &execution.mcp)
            .with_models(&config, request.preset_models);
        let preset = session_preset(
            request.preset,
            request.identity_profile,
            tools::PresetBase::default(),
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
        let mut raw = json!({"v":1,"seq":1,"ts":request.started_at,"kind":"genesis",
            "thread":session_id,"cwd":execution.cwd,"execution":execution,
            "format":1,"min_reader":schema::EXECUTION_READER,"min_writer":1,
            "resume":ResumePolicy::Never,"config":{"digest":config_digest},
            "instruction":{"digest":instruction_digest},"origin_key":origin.key,"origin_tuple":origin});
        if let Some(preset) = &preset {
            raw["preset"] = serde_json::to_value(preset)?;
        } else {
            raw["identity_profile"] = json!(
                request
                    .identity_profile
                    .map_or("auto", tools::IdentityProfile::as_str)
            );
        }
        let genesis = Event::from_value(to_ijson(&raw)?)?;
        let genesis_bytes = canonical_event_line(&genesis)?;
        let payload = self.operation_payload_path(request.rpc_id);
        self.remove_recordless_payload(request.rpc_id)?;
        let assets = AssetStore::new(payload.join("assets"))?;
        assets.publish(&config_bytes)?;
        assets.publish(&instruction_bytes)?;
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
                execution_digest,
                cwd: execution.cwd,
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
    ) -> Result<SelectedModel, SessionOperationError> {
        crate::validate_session_id(request.session_id)?;
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.selectModel"
                || record.request_sha256 != request.request_sha256
            {
                return Err(SessionOperationError::IdempotencyConflict {
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
            return Err(SessionOperationError::SessionRunning(
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
            return Err(SessionOperationError::SessionRunning(
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
    ) -> Result<String, SessionOperationError> {
        crate::validate_session_id(request.source_session_id)?;
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(request.rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != request.rpc_id
                || record.operation != "session.fork"
                || record.request_sha256 != request.request_sha256
            {
                return Err(SessionOperationError::IdempotencyConflict {
                    rpc_id: request.rpc_id.to_owned(),
                    operation: "session.fork".to_owned(),
                });
            }
            return fork_id_from_response(self.drive_fork(&operation_path, record)?);
        }
        if active_session_count(&self.storage_root)? >= 256 {
            return Err(SessionOperationError::ActiveSessionLimit);
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
                SessionOperationError::InvalidAtSeq {
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
                    SessionOperationError::InvalidAtSeq {
                        session_id: request.source_session_id.to_owned(),
                        at_seq: request.at_endpoint_seq,
                    }
                }
                other => SessionOperationError::Journal(other),
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
                    request.preset_models,
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

    /// Writes a preset's starting model as the session's settings, as
    /// `session.selectModel` would. A retry that finds them written is done.
    fn apply_preset_defaults(
        &self,
        folder: &Path,
        defaults: &tools::PresetDefaults,
    ) -> Result<(), SessionOperationError> {
        let current = self.config.session_settings(folder)?;
        if current.as_ref().is_some_and(|current| {
            current.provider == defaults.provider
                && current.model == defaults.model
                && current.reasoning_effort == defaults.reasoning_effort
        }) {
            return Ok(());
        }
        let revision = current.as_ref().map_or(0, |current| current.revision);
        self.config.publish_session_settings(
            folder,
            revision,
            &profile::SessionSettings {
                format: 1,
                revision: revision + 1,
                provider: defaults.provider.clone(),
                model: defaults.model.clone(),
                reasoning_effort: defaults.reasoning_effort.clone(),
            },
        )?;
        Ok(())
    }

    /// Resolves a `session.fork {preset}` under the source session's workspace
    /// base, instruction snapshot and MCP registry, exactly as `session.create`.
    fn resolve_fork_preset(
        &self,
        source_folder: &Path,
        raw: &Value,
        user_agent_dir: &Path,
        models: Option<&dyn PresetModelSource>,
    ) -> Result<tools::ResolvedPreset, SessionOperationError> {
        let scope = profile::SessionScope::for_session_folder(source_folder)?;
        let config = scope.base_config(&self.config)?;
        let instruction = scope
            .instructions(&self.config, user_agent_dir, &config)?
            .capture()?;
        let environment = SessionPresetEnvironment::new(&instruction, &scope.execution().mcp)
            .with_models(&config, models);
        session_preset(Some(raw), None, tools::PresetBase::default(), &environment)?.ok_or_else(
            || SessionOperationError::CorruptOperation("fork preset did not resolve".to_owned()),
        )
    }

    /// Publishes the durable queue-management gate before a worker delivery.
    /// Exact retry returns the persisted decision and never revalidates a
    /// rejected transaction against a later tail.
    pub fn prepare_queue_transaction(
        &self,
        request: QueueTransactionOperation<'_>,
    ) -> Result<QueueTransactionState, SessionOperationError> {
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
            return Err(SessionOperationError::CorruptOperation(
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
                return Err(SessionOperationError::IdempotencyConflict {
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
            SessionOperationError::CorruptOperation(
                "queue target has no valid semantic ledger prefix".to_owned(),
            )
        })?;
        let valid_bytes = usize::try_from(scan.valid_bytes).map_err(|_| {
            SessionOperationError::CorruptOperation(
                "queue tail length exceeds this host".to_owned(),
            )
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
    ) -> Result<QueueTransactionCompletion, SessionOperationError> {
        let _lock = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        let record = read_canonical::<OperationRecord>(&operation_path)?;
        if record.rpc_id != rpc_id || record.operation != "session.updateQueue" {
            return Err(SessionOperationError::CorruptOperation(
                "queue completion does not match its operation".to_owned(),
            ));
        }
        self.drive_queue_completion(&operation_path, record, decision)
    }

    pub fn has_incomplete_session_operation(
        &self,
        session_id: &str,
    ) -> Result<bool, SessionOperationError> {
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

    pub fn completed_fork_lineage(
        &self,
    ) -> Result<BTreeMap<String, ForkLineage>, SessionOperationError> {
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
                    return Err(SessionOperationError::CorruptOperation(
                        "fork destination has more than one completed lineage".to_owned(),
                    ));
                }
            }
        }
        Ok(lineage)
    }

    pub fn archive_session(
        &self,
        rpc_id: &str,
        request_sha256: &str,
        session_id: &str,
        started_at: &str,
    ) -> Result<Vec<String>, SessionOperationError> {
        let value = self.folder_move(rpc_id, request_sha256, session_id, started_at, "archive")?;
        value
            .get("archivedSessionIds")
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .map(|value| {
                        value.as_str().map(str::to_owned).ok_or_else(|| {
                            SessionOperationError::CorruptOperation(
                                "archive response contains a non-string id".to_owned(),
                            )
                        })
                    })
                    .collect()
            })
            .ok_or_else(|| {
                SessionOperationError::CorruptOperation(
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
    ) -> Result<String, SessionOperationError> {
        let value =
            self.folder_move(rpc_id, request_sha256, session_id, started_at, "unarchive")?;
        value
            .get("sessionId")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                SessionOperationError::CorruptOperation(
                    "unarchive response lacks sessionId".to_owned(),
                )
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
    ) -> Result<String, SessionOperationError> {
        crate::validate_session_id(session_id)?;
        let _management = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != rpc_id
                || record.request_sha256 != request_sha256
                || record.operation != "session.discard"
            {
                return Err(SessionOperationError::IdempotencyConflict {
                    rpc_id: rpc_id.to_owned(),
                    operation: "session.discard".to_owned(),
                });
            }
            return self.drive_discard(&operation_path, record);
        }
        if self.storage_root.join("archive").join(session_id).is_dir() {
            return Err(SessionOperationError::SessionArchived(
                session_id.to_owned(),
            ));
        }
        let source = self.storage_root.join("threads").join(session_id);
        if !source.is_dir() {
            return Err(SessionOperationError::SessionNotFound(
                session_id.to_owned(),
            ));
        }
        if !folder_is_ephemeral(&source)? {
            return Err(SessionOperationError::SessionNotEphemeral(
                session_id.to_owned(),
            ));
        }
        let lifecycle = DirectoryLock::try_exclusive(&source).map_err(|error| match error {
            store::StoreError::Busy => SessionOperationError::SessionRunning(session_id.to_owned()),
            other => SessionOperationError::Store(other),
        })?;
        if ThreadStore::open(&self.storage_root)?.session_has_live_line_holder(session_id)? {
            return Err(SessionOperationError::SessionRunning(session_id.to_owned()));
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
    ) -> Result<String, SessionOperationError> {
        let intent = discard_intent(&record)?;
        if record.v != VERSION || record.operation != "session.discard" {
            return Err(SessionOperationError::CorruptOperation(
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
                    return Err(SessionOperationError::CorruptOperation(
                        "discard reservation no longer matches target".to_owned(),
                    ));
                }
                ThreadStore::open(&self.storage_root)?
                    .discard_ephemeral(&intent.session_id)
                    .map_err(|error| match error {
                        store::StoreError::Busy => {
                            SessionOperationError::SessionRunning(intent.session_id.clone())
                        }
                        store::StoreError::NotEphemeral => {
                            SessionOperationError::SessionNotEphemeral(intent.session_id.clone())
                        }
                        other => SessionOperationError::Store(other),
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
            SessionOperationError::CorruptOperation("complete discard lacks response".to_owned())
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
    ) -> Result<Value, SessionOperationError> {
        crate::validate_session_id(session_id)?;
        let _management = NamedLock::exclusive(self.root.join("lock"))?;
        let operation_path = self.operation_path(rpc_id);
        let requested_operation = if action == "archive" {
            "session.archive"
        } else {
            "session.unarchive"
        };
        if operation_path.exists() {
            let record = read_canonical::<OperationRecord>(&operation_path)?;
            if record.rpc_id != rpc_id
                || record.request_sha256 != request_sha256
                || record.operation != requested_operation
            {
                return Err(SessionOperationError::IdempotencyConflict {
                    rpc_id: rpc_id.to_owned(),
                    operation: requested_operation.to_owned(),
                });
            }
            return self.drive_folder_move(&operation_path, record, None);
        }
        if action == "unarchive" && active_session_count(&self.storage_root)? >= 256 {
            return Err(SessionOperationError::ActiveSessionLimit);
        }
        let (source_area, destination_area, operation) = if action == "archive" {
            ("threads", "archive", "session.archive")
        } else {
            ("archive", "threads", "session.unarchive")
        };
        let source = self.storage_root.join(source_area).join(session_id);
        if !source.is_dir() {
            return Err(SessionOperationError::SessionNotFound(
                session_id.to_owned(),
            ));
        }
        if action == "archive" && folder_is_ephemeral(&source)? {
            return Err(SessionOperationError::SessionEphemeral(
                session_id.to_owned(),
            ));
        }
        let lifecycle = DirectoryLock::try_exclusive(&source).map_err(|error| match error {
            store::StoreError::Busy => SessionOperationError::SessionRunning(session_id.to_owned()),
            other => SessionOperationError::Store(other),
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

    fn drive_session_create(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, SessionOperationError> {
        let intent = session_create_intent(&record)?;
        if record.v != VERSION || record.operation != "session.create" {
            return Err(SessionOperationError::CorruptOperation(
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
        if config.execution.session_id != intent.session_id
            || execution_path_from_snapshot(&config)? != intent.cwd
            || intent.config_asset != format!("sha256-{}", config.digest()?)
            || intent.instruction_asset != format!("sha256-{}", instruction.digest()?)
        {
            return Err(SessionOperationError::CorruptOperation(
                "session-create snapshots disagree with immutable intent".to_owned(),
            ));
        }
        let genesis = read_canonical_event(&payload.join("genesis.json"))?;
        if let Some(execution) = serde_json::to_value(genesis.raw())?.get("execution") {
            let execution: profile::ExecutionConfig = serde_json::from_value(execution.clone())?;
            let roots = std::iter::once(&execution.cwd)
                .chain(&execution.read_roots)
                .cloned()
                .collect::<Vec<_>>();
            if roots != config.execution.cwd
                || execution.writable_roots != config.execution.policy.writable_roots
                || execution.toolchain_roots != config.execution.policy.toolchain_roots
                || execution.network != config.execution.policy.network
                || execution.allowed_tools != config.execution.policy.allowed_tools
                || execution.max_wall_seconds != config.execution.policy.max_wall_seconds
            {
                return Err(SessionOperationError::CorruptOperation(
                    "execution differs from staged config asset".to_owned(),
                ));
            }
        }
        verify_session_create_genesis(&genesis, &record, &intent)?;
        let defaults = genesis_preset_defaults(&genesis)?;

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
            if let Some(defaults) = &defaults {
                self.apply_preset_defaults(
                    &self.storage_root.join("threads").join(&intent.session_id),
                    defaults,
                )?;
            }
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
            SessionOperationError::CorruptOperation(
                "complete session-create operation lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn session_id_from_response(&self, response: Value) -> Result<String, SessionOperationError> {
        response
            .as_object()
            .filter(|object| object.len() == 1)
            .and_then(|object| object.get("sessionId"))
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                SessionOperationError::CorruptOperation(
                    "session-create response lacks exact sessionId".to_owned(),
                )
            })
    }

    fn drive_select_model(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
    ) -> Result<Value, SessionOperationError> {
        let intent = select_model_intent(&record)?;
        if record.v != VERSION || record.operation != "session.selectModel" {
            return Err(SessionOperationError::CorruptOperation(
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
            return Err(SessionOperationError::CorruptOperation(
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
            SessionOperationError::CorruptOperation(
                "complete select-model operation lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn fork_was_quarantined(
        &self,
        record: &OperationRecord,
    ) -> Result<bool, SessionOperationError> {
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
    ) -> Result<Value, SessionOperationError> {
        let intent = fork_intent(&record)?;
        if record.v != VERSION || record.operation != "session.fork" {
            return Err(SessionOperationError::CorruptOperation(
                "unsupported fork operation".to_owned(),
            ));
        }
        if self.fork_was_quarantined(&record)? {
            return Err(SessionOperationError::CorruptOperation(
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
                return Err(SessionOperationError::CorruptOperation(
                    "fork rewrite did not publish its destination".to_owned(),
                ));
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
            if let Some(defaults) = intent
                .preset
                .as_ref()
                .and_then(|preset| preset.defaults.as_ref())
            {
                self.apply_preset_defaults(&destination, defaults)?;
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
            SessionOperationError::CorruptOperation(
                "complete fork operation lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn drive_queue_completion(
        &self,
        operation_path: &Path,
        mut record: OperationRecord,
        decision: QueueTransactionDecision,
    ) -> Result<QueueTransactionCompletion, SessionOperationError> {
        let intent = queue_transaction_intent(&record)?;
        if record.v != VERSION || record.operation != "session.updateQueue" {
            return Err(SessionOperationError::CorruptOperation(
                "unsupported queue transaction operation".to_owned(),
            ));
        }
        if record.phase == OperationPhase::Complete {
            let completed = queue_completion_from_record(&record)?;
            if completed != completion_for_decision(&decision) {
                return Err(SessionOperationError::CorruptOperation(
                    "queue completion decision changed after durability".to_owned(),
                ));
            }
            return Ok(completed);
        }
        verify_queue_payload(&self.operation_payload_path(&record.rpc_id), &intent)?;
        match decision {
            QueueTransactionDecision::Rejected { code, reason } => {
                if record.phase != OperationPhase::Prepared {
                    return Err(SessionOperationError::CorruptOperation(
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

    fn remove_recordless_payloads(&self) -> Result<(), SessionOperationError> {
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
                    return Err(SessionOperationError::CorruptOperation(
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

    fn remove_recordless_payload(&self, rpc_id: &str) -> Result<(), SessionOperationError> {
        let operation_path = self.operation_path(rpc_id);
        let payload_root = self
            .operation_payload_path(rpc_id)
            .parent()
            .ok_or_else(|| {
                SessionOperationError::CorruptOperation("operation payload has no root".to_owned())
            })?
            .to_path_buf();
        if !operation_path.exists() && payload_root.exists() {
            fs::remove_dir_all(&payload_root)?;
            File::open(payload_root.parent().ok_or_else(|| {
                SessionOperationError::CorruptOperation(
                    "operation payload has no prefix".to_owned(),
                )
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
    ) -> Result<Value, SessionOperationError> {
        let intent = folder_move_intent(&record)?;
        if record.v != VERSION || !matches!(intent.action.as_str(), "archive" | "unarchive") {
            return Err(SessionOperationError::CorruptOperation(
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
                    return Err(SessionOperationError::CorruptOperation(
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
                return Err(SessionOperationError::CorruptOperation(
                    "folder move lost both source and destination".to_owned(),
                ));
            }
            record.phase = OperationPhase::Semantic;
            publish_canonical(operation_path, &record)?;
        }
        if record.phase == OperationPhase::Semantic {
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
            SessionOperationError::CorruptOperation(
                "complete folder move lacks response".to_owned(),
            )
        })?;
        serde_json::from_slice(&response.canonical_bytes()?).map_err(Into::into)
    }

    fn operation_paths(&self) -> Result<Vec<PathBuf>, SessionOperationError> {
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
}

fn execution_path_from_snapshot(
    config: &profile::ConfigSnapshot,
) -> Result<&str, SessionOperationError> {
    config.execution_cwd().ok_or_else(|| {
        SessionOperationError::CorruptOperation("snapshot workspace cwd is empty".to_owned())
    })
}

/// The preset a new session freezes in genesis: an explicit request resolved
/// under the workspace base, or the identity sugar when the base is `blank`.
/// A `general` workspace without a request keeps the identity-profile genesis.
fn session_preset(
    request: Option<&Value>,
    identity_profile: Option<tools::IdentityProfile>,
    base: tools::PresetBase,
    environment: &dyn tools::PresetEnvironment,
) -> Result<Option<tools::ResolvedPreset>, SessionOperationError> {
    match request {
        Some(request) => tools::ResolvedPreset::resolve_request(request, base, environment)
            .map(Some)
            .map_err(|error| match error {
                tools::PresetError::BadRequest(message) => {
                    SessionOperationError::PresetBadRequest(message)
                }
                tools::PresetError::Invalid(invalid) => {
                    SessionOperationError::PresetInvalid(invalid)
                }
            }),
        None if base == tools::PresetBase::Blank => Ok(Some(tools::ResolvedPreset::from_identity(
            identity_profile,
            base,
        ))),
        None => Ok(None),
    }
}

/// The `defaults` of a genesis preset, when the session has one.
fn genesis_preset_defaults(
    genesis: &Event,
) -> Result<Option<tools::PresetDefaults>, SessionOperationError> {
    let value: Value = serde_json::from_slice(&genesis.canonical_bytes()?)?;
    let Some(preset) = value.get("preset") else {
        return Ok(None);
    };
    tools::ResolvedPreset::from_genesis(preset)
        .map(|preset| preset.defaults)
        .map_err(SessionOperationError::CorruptOperation)
}

fn canonical_event_line(event: &Event) -> Result<Vec<u8>, SessionOperationError> {
    let mut bytes = event.canonical_bytes()?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn read_canonical_event(path: &Path) -> Result<Event, SessionOperationError> {
    let bytes = fs::read(path)?;
    if !bytes.ends_with(b"\n") || bytes.len() == 1 {
        return Err(SessionOperationError::CorruptOperation(format!(
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
) -> Result<(), SessionOperationError> {
    let origin = genesis.origin_tuple()?.ok_or_else(|| {
        SessionOperationError::CorruptOperation("session-create genesis lacks origin".to_owned())
    })?;
    let genesis_value: Value = serde_json::from_slice(&genesis.canonical_bytes()?)?;
    let object = genesis_value.as_object().ok_or_else(|| {
        SessionOperationError::CorruptOperation(
            "session-create genesis is not an object".to_owned(),
        )
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
        || genesis.string_field("cwd") != Some(intent.cwd.as_str())
        || object
            .get("execution")
            .map(|value| serde_json_canonicalizer::to_vec(value).map(|bytes| hex_digest(&bytes)))
            .transpose()
            .map_err(|e| SessionOperationError::Canonical(e.to_string()))?
            .as_deref()
            != Some(intent.execution_digest.as_str())
        || [
            "workspace",
            "environment",
            "folders",
            "folder_binding",
            "session_scope",
        ]
        .iter()
        .any(|field| object.contains_key(*field))
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
        return Err(SessionOperationError::CorruptOperation(
            "session-create genesis disagrees with immutable intent".to_owned(),
        ));
    }
    Ok(())
}

fn sync_operation_payload(payload: &Path) -> Result<(), SessionOperationError> {
    File::open(payload)?.sync_all()?;
    let rpc_root = payload.parent().ok_or_else(|| {
        SessionOperationError::CorruptOperation("operation payload has no rpc root".to_owned())
    })?;
    File::open(rpc_root)?.sync_all()?;
    let prefix = rpc_root.parent().ok_or_else(|| {
        SessionOperationError::CorruptOperation("operation payload has no prefix".to_owned())
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

fn allocate_uuid_v7() -> Result<String, SessionOperationError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| SessionOperationError::Clock(error.to_string()))?
        .as_millis();
    if millis > u128::from((1_u64 << 48) - 1) {
        return Err(SessionOperationError::Clock(
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

fn publish_canonical<T: Serialize>(path: &Path, value: &T) -> Result<(), SessionOperationError> {
    AtomicPublisher::replace(path, &canonical_line(value)?)?;
    Ok(())
}

fn read_canonical<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, SessionOperationError> {
    decode_canonical(path, &fs::read(path)?)
}

fn decode_canonical<T: for<'de> Deserialize<'de>>(
    path: &Path,
    bytes: &[u8],
) -> Result<T, SessionOperationError> {
    if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].ends_with(b"\n") {
        return Err(SessionOperationError::CorruptOperation(format!(
            "{} is not one canonical JSON line",
            path.display()
        )));
    }
    let body = &bytes[..bytes.len() - 1];
    let value = IJsonValue::parse(body)?;
    if value.canonical_bytes()? != body {
        return Err(SessionOperationError::CorruptOperation(format!(
            "{} is not RFC-8785 canonical",
            path.display()
        )));
    }
    serde_json::from_slice(body).map_err(Into::into)
}

fn canonical_line<T: Serialize>(value: &T) -> Result<Vec<u8>, SessionOperationError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| SessionOperationError::Canonical(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn to_ijson<T: Serialize>(value: &T) -> Result<IJsonValue, SessionOperationError> {
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(value)
            .map_err(|error| SessionOperationError::Canonical(error.to_string()))?,
    )
    .map_err(Into::into)
}

fn intent_kind(record: &OperationRecord) -> Result<String, SessionOperationError> {
    let bytes = record.intent.canonical_bytes()?;
    let value: Value = serde_json::from_slice(&bytes)?;
    value
        .as_object()
        .and_then(|object| object.get("kind"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            SessionOperationError::CorruptOperation("operation intent has no kind".to_owned())
        })
}

fn discard_intent(record: &OperationRecord) -> Result<DiscardIntent, SessionOperationError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: DiscardIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "discard" {
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a discard intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn folder_is_ephemeral(folder: &Path) -> Result<bool, SessionOperationError> {
    Ok(store::read_genesis(folder.join("main.jsonl"))?
        .as_ref()
        .is_some_and(schema::Event::is_ephemeral_genesis))
}

fn folder_move_intent(record: &OperationRecord) -> Result<FolderMoveIntent, SessionOperationError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: FolderMoveIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "folder-move" || !matches!(intent.action.as_str(), "archive" | "unarchive") {
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a folder-move intent".to_owned(),
        ));
    }
    Ok(intent)
}

fn session_create_intent(
    record: &OperationRecord,
) -> Result<SessionCreateIntent, SessionOperationError> {
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
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a valid session-create intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn select_model_intent(
    record: &OperationRecord,
) -> Result<SelectModelIntent, SessionOperationError> {
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
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a valid select-model intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn fork_intent(record: &OperationRecord) -> Result<ForkIntent, SessionOperationError> {
    let bytes = record.intent.canonical_bytes()?;
    let intent: ForkIntent = serde_json::from_slice(&bytes)?;
    if intent.kind != "fork"
        || intent.kernel_anchor == 0
        || intent.rewrite_op_id.is_empty()
        || !is_uid_principal(&intent.principal)
    {
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a valid fork intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.source)?;
    crate::validate_session_id(&intent.dest)?;
    Ok(intent)
}

fn queue_transaction_intent(
    record: &OperationRecord,
) -> Result<QueueTransactionIntent, SessionOperationError> {
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
        return Err(SessionOperationError::CorruptOperation(
            "operation is not a valid queue-transaction intent".to_owned(),
        ));
    }
    crate::validate_session_id(&intent.session_id)?;
    Ok(intent)
}

fn pending_queue_transaction(
    record: &OperationRecord,
) -> Result<PendingQueueTransaction, SessionOperationError> {
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
) -> Result<(), SessionOperationError> {
    let action_path = payload.join("queue-action.json");
    let action = fs::read(&action_path)?;
    if action != intent.action.canonical_bytes()? {
        return Err(SessionOperationError::CorruptOperation(
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
        return Err(SessionOperationError::CorruptOperation(
            "queue tail snapshot disagrees with prepared intent".to_owned(),
        ));
    }
    for asset in &intent.asset_digests {
        if !payload.join("assets").join(asset).is_file() {
            return Err(SessionOperationError::CorruptOperation(format!(
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
) -> Result<IJsonValue, SessionOperationError> {
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
                    return Err(SessionOperationError::CorruptOperation(
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
) -> Result<QueueTransactionCompletion, SessionOperationError> {
    if record.phase != OperationPhase::Complete {
        return Err(SessionOperationError::CorruptOperation(
            "queue operation is not complete".to_owned(),
        ));
    }
    let intent = queue_transaction_intent(record)?;
    let response = record.response.as_ref().ok_or_else(|| {
        SessionOperationError::CorruptOperation(
            "complete queue operation lacks response".to_owned(),
        )
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
                SessionOperationError::CorruptOperation(
                    "queue completion lacks its exact error".to_owned(),
                )
            })?;
        let code = error.get("code").and_then(Value::as_str).ok_or_else(|| {
            SessionOperationError::CorruptOperation("queue completion lacks error code".to_owned())
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
        return Err(SessionOperationError::CorruptOperation(
            "queue completion response is not exact".to_owned(),
        ));
    }
    Ok(completion)
}

fn operation_target_session(
    record: &OperationRecord,
) -> Result<Option<String>, SessionOperationError> {
    Ok(match intent_kind(record)?.as_str() {
        "session-create" => Some(session_create_intent(record)?.session_id),
        "select-model" => Some(select_model_intent(record)?.session_id),
        "folder-move" => Some(folder_move_intent(record)?.session_id),
        "discard" => Some(discard_intent(record)?.session_id),
        "fork" => Some(fork_intent(record)?.dest),
        "queue-transaction" => Some(queue_transaction_intent(record)?.session_id),
        other => {
            return Err(SessionOperationError::CorruptOperation(format!(
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

fn fork_id_from_response(response: Value) -> Result<String, SessionOperationError> {
    response
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("sessionId"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            SessionOperationError::CorruptOperation(
                "fork response lacks exact sessionId".to_owned(),
            )
        })
}

fn selected_model_from_response(response: Value) -> Result<SelectedModel, SessionOperationError> {
    let selected = response
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("selected"))
        .and_then(Value::as_object)
        .ok_or_else(|| {
            SessionOperationError::CorruptOperation(
                "select-model response lacks exact selected value".to_owned(),
            )
        })?;
    let provider = selected
        .get("provider")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            SessionOperationError::CorruptOperation("selected provider is missing".to_owned())
        })?;
    let model = selected
        .get("model")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            SessionOperationError::CorruptOperation("selected model is missing".to_owned())
        })?;
    let reasoning_effort = selected
        .get("reasoningEffort")
        .map(|value| {
            value.as_str().map(str::to_owned).ok_or_else(|| {
                SessionOperationError::CorruptOperation(
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

fn active_session_folder(root: &Path, session_id: &str) -> Result<PathBuf, SessionOperationError> {
    if root.join("archive").join(session_id).is_dir() {
        return Err(SessionOperationError::SessionArchived(
            session_id.to_owned(),
        ));
    }
    let folder = root.join("threads").join(session_id);
    if !folder.is_dir() {
        return Err(SessionOperationError::SessionNotFound(
            session_id.to_owned(),
        ));
    }
    Ok(folder)
}

fn valid_folder_projection(
    folder: &Path,
) -> Result<schema::LedgerProjection, SessionOperationError> {
    // Boot no longer repairs every tail: bring this thread's tail clean when nobody holds it.
    let (_, scan) = store::read_ledger_repaired(folder.join("main.jsonl"))?;
    if scan.valid_bytes != scan.file_bytes {
        return Err(SessionOperationError::CorruptOperation(
            "session ledger has a torn or invalid tail".to_owned(),
        ));
    }
    scan.projection.ok_or_else(|| {
        SessionOperationError::CorruptOperation("session ledger has no valid genesis".to_owned())
    })
}

fn folder_reservation(folder: &Path) -> Result<(String, String), SessionOperationError> {
    let metadata = fs::metadata(folder)?;
    let (bytes, scan) = store::read_ledger_repaired(folder.join("main.jsonl"))?;
    if scan.valid_bytes != scan.file_bytes || scan.projection.is_none() {
        return Err(SessionOperationError::CorruptOperation(
            "folder move source has no complete valid ledger prefix".to_owned(),
        ));
    }
    let valid = usize::try_from(scan.valid_bytes).map_err(|_| {
        SessionOperationError::CorruptOperation("valid prefix length exceeds usize".to_owned())
    })?;
    Ok((
        format!("dev:{}:ino:{}", metadata.dev(), metadata.ino()),
        format!("sha256-{}", hex_digest(&bytes[..valid])),
    ))
}

fn verify_folder_reservation(
    folder: &Path,
    intent: &FolderMoveIntent,
) -> Result<(), SessionOperationError> {
    let (identity, digest) = folder_reservation(folder)?;
    if identity != intent.directory_identity || digest != intent.valid_prefix_digest {
        return Err(SessionOperationError::CorruptOperation(
            "folder move reservation no longer matches target".to_owned(),
        ));
    }
    Ok(())
}

fn active_session_count(root: &Path) -> Result<usize, SessionOperationError> {
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
pub enum SessionOperationError {
    #[error("invalid execution configuration: {0}")]
    InvalidExecution(String),

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
