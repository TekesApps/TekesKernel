use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::Path;

use schema::{Event, EventKind, IJsonValue, OriginTuple, ResumePolicy, SeqRange};
use serde_json::{Value, json};
use store::{AppendOutcome, ConditionalAppendOutcome, ThreadStore, scan_valid_prefix};
use thiserror::Error;

use crate::projection::timestamp_millis;
use crate::{EndpointJournal, HistoryPage, Projector, history_page, validate_session_id};

pub const AUTOMATIC_TITLE_SEED_OPERATION: &str = "automatic-title.seed";
pub const AUTOMATIC_TITLE_REFINE_OPERATION: &str = "automatic-title.refine";
pub const SESSION_NOTICE_OPERATION: &str = "session-notice.record";

/// A host-authored, turn-independent fact about the session's runtime that the
/// transcript must show: a launch that ran degraded (`warning`) or an input that
/// will not run at all (`error`). It is a `meta` record so it needs no open
/// turn, and it is keyed so a repeated identical notice folds into one line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionNotice {
    pub severity: SessionNoticeSeverity,
    pub classification: String,
    pub operation: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionNoticeSeverity {
    Warning,
    Error,
}

impl SessionNoticeSeverity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MutationReceipt {
    pub seq: u64,
    pub deduplicated: bool,
}

impl From<AppendOutcome> for MutationReceipt {
    fn from(value: AppendOutcome) -> Self {
        Self {
            seq: value.seq,
            deduplicated: value.deduplicated,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveResult {
    pub archived_session_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnarchiveResult {
    pub session_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionInventoryItem {
    pub session_id: String,
    pub workspace_id: String,
    pub title: Option<String>,
    pub updated_at: u64,
    pub archived: bool,
    /// Lock evidence: a live line holder or a worker the caller registered.
    /// Combined with `lifecycle` by the carrier into the classified tail state.
    pub running: bool,
    pub blank: bool,
    pub as_of_seq: u64,
    pub lifecycle: schema::LifecycleFacts,
    /// The session's durable permission mode (`threads/<id>/permission-mode.json`);
    /// the default mode when the record is absent or unreadable.
    pub permission_mode: String,
    pub identity_profile: Option<String>,
    /// Genesis `ephemeral: true`: a scratch ledger that never joins a
    /// workspace, cannot be archived, and disappears on discard or restart.
    pub ephemeral: bool,
}

/// File name of the durable per-session permission mode record (owned by the
/// engine crate; mirrored here so inventory needs no engine dependency).
const PERMISSION_MODE_FILE: &str = "permission-mode.json";
const DEFAULT_PERMISSION_MODE: &str = "workspace-write";

/// The mode a session folder declares, or the default when the record is
/// absent or malformed (the worker applies the same fallback).
fn permission_mode_of(folder: &Path) -> String {
    let Ok(bytes) = fs::read(folder.join(PERMISSION_MODE_FILE)) else {
        return DEFAULT_PERMISSION_MODE.to_owned();
    };
    serde_json::from_slice::<serde_json::Value>(&bytes)
        .ok()
        .and_then(|value| value.get("mode")?.as_str().map(str::to_owned))
        .filter(|mode| {
            matches!(
                mode.as_str(),
                "read-only" | "workspace-write" | "danger-full-access"
            )
        })
        .unwrap_or_else(|| DEFAULT_PERMISSION_MODE.to_owned())
}

pub struct NativeEndpoint {
    store: ThreadStore,
}

struct CreateSessionWrite<'a> {
    session_id: &'a str,
    workspace: &'a str,
    config_digest: &'a str,
    resume: ResumePolicy,
    timestamp: &'a str,
    origin: &'a OriginTuple,
    operation: &'a str,
}

impl NativeEndpoint {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, NativeEndpointError> {
        Ok(Self {
            store: ThreadStore::open(root)?,
        })
    }

    #[must_use]
    pub fn capabilities() -> BTreeSet<&'static str> {
        [
            "session.models",
            "session.create",
            "session.prompt",
            "session.updateQueue",
            "session.cancel",
            "session.rename",
            "session.fork",
            "session.discard",
            "workspace.archiveSession",
            "workspace.unarchiveSession",
        ]
        .into_iter()
        .collect()
    }

    pub fn list_sessions(
        &self,
        running: &HashSet<String>,
    ) -> Result<Vec<SessionInventoryItem>, NativeEndpointError> {
        let mut items = Vec::new();
        for (area, archived) in [("threads", false), ("archive", true)] {
            for entry in fs::read_dir(self.store.root().join(area))? {
                let entry = entry?;
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                let session_id = entry.file_name().to_string_lossy().into_owned();
                validate_session_id(&session_id)?;
                let bytes = fs::read(entry.path().join("main.jsonl"))?;
                let scan = scan_valid_prefix(&bytes, 1);
                let projection = scan.projection.ok_or_else(|| {
                    NativeEndpointError::Store(store::StoreError::Corruption(format!(
                        "session {session_id} has no valid genesis prefix"
                    )))
                })?;
                let genesis = projection.events.first().ok_or_else(|| {
                    NativeEndpointError::Store(store::StoreError::Corruption(format!(
                        "session {session_id} has no genesis"
                    )))
                })?;
                let title = projection
                    .events
                    .iter()
                    .rev()
                    .find_map(|event| event.string_field("title"))
                    .map(str::to_owned);
                let updated_at = projection
                    .events
                    .last()
                    .and_then(|event| event.string_field("ts"))
                    .ok_or(NativeEndpointError::InventoryTimestamp)
                    .and_then(|value| timestamp_millis(value).map_err(NativeEndpointError::from))?
                    as u64;
                let is_running = !archived
                    && (running.contains(&session_id)
                        || self.store.session_has_live_line_holder(&session_id)?);
                items.push(SessionInventoryItem {
                    permission_mode: permission_mode_of(&entry.path()),
                    identity_profile: match genesis.string_field("identity_profile") {
                        None => Some("coding".to_owned()),
                        Some("coding" | "general") => {
                            genesis.string_field("identity_profile").map(str::to_owned)
                        }
                        _ => projection.events.iter().find_map(|event| {
                            if event.kind() != &EventKind::State
                                || event.string_field("subkind") != Some("identity.selected")
                            {
                                return None;
                            }
                            serde_json::to_value(event.raw())
                                .ok()?
                                .get("payload")?
                                .get("profile")?
                                .as_str()
                                .map(str::to_owned)
                        }),
                    },
                    ephemeral: genesis.is_ephemeral_genesis(),
                    workspace_id: genesis
                        .string_field("workspace")
                        .ok_or(NativeEndpointError::InventoryWorkspace)?
                        .to_owned(),
                    title,
                    updated_at,
                    running: is_running,
                    blank: projection.lifecycle.latest_turn.is_none(),
                    as_of_seq: projection.last_seq,
                    lifecycle: projection.lifecycle,
                    session_id,
                    archived,
                });
            }
        }
        items.sort_by(|left, right| left.session_id.as_bytes().cmp(right.session_id.as_bytes()));
        Ok(items)
    }

    pub fn models(
        snapshot: &profile::ConfigSnapshot,
        failures: IJsonValue,
    ) -> Result<IJsonValue, NativeEndpointError> {
        let selected_provider = snapshot
            .session_settings
            .as_ref()
            .map(|settings| &settings.provider)
            .or(snapshot.workspace.policy.provider.as_ref())
            .or(snapshot.settings.default_provider.as_ref());
        let selected_model = snapshot
            .session_settings
            .as_ref()
            .map(|settings| &settings.model)
            .or(snapshot.workspace.policy.model.as_ref())
            .or(snapshot.settings.default_model.as_ref());
        let groups = snapshot
            .providers
            .providers
            .iter()
            .map(|provider| {
                json!({
                    "id": provider.id,
                    "name": provider.id,
                    "models": provider.models.iter().filter(|model| model.enabled).map(|model| {
                        json!({"id": model.id, "name": model.id,
                            "contextWindow": model.context_window_tokens})
                    }).collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>();
        let routable = selected_provider
            .zip(selected_model)
            .is_some_and(|(provider, model)| {
                snapshot.providers.providers.iter().any(|candidate| {
                    candidate.id == *provider
                        && candidate
                            .models
                            .iter()
                            .any(|candidate| candidate.id == *model && candidate.enabled)
                })
            });
        let mut current = json!({
            "provider": selected_provider.cloned().unwrap_or_default(),
            "model": selected_model.cloned().unwrap_or_default(),
        });
        if let Some(effort) = snapshot
            .session_settings
            .as_ref()
            .and_then(|settings| settings.reasoning_effort.as_ref())
        {
            current["reasoningEffort"] = Value::String(effort.clone());
        }
        let value = json!({
            "current": current,
            "routable": routable,
            "groups": groups,
            "failures": failures
        });
        Ok(IJsonValue::parse(&serde_json::to_vec(&value)?)?)
    }

    /// Resolve the immutable configuration view which the next worker spawn
    /// for `session_id` would capture. Session-local model selection is read
    /// from the thread folder and therefore precedes workspace/global
    /// defaults exactly as it does at launch.
    pub fn session_config_snapshot(
        &self,
        session_id: &str,
    ) -> Result<profile::ConfigSnapshot, NativeEndpointError> {
        validate_session_id(session_id)?;
        let folder = self.active_folder(session_id)?;
        let bytes = fs::read(folder.join("main.jsonl"))?;
        let scan = scan_valid_prefix(&bytes, 1);
        let projection = scan.projection.ok_or_else(|| {
            NativeEndpointError::Store(store::StoreError::Corruption(
                "session ledger has no valid genesis prefix".to_owned(),
            ))
        })?;
        let genesis = projection
            .events
            .first()
            .ok_or(NativeEndpointError::InventoryWorkspace)?;
        let workspace = genesis
            .string_field("workspace")
            .ok_or(NativeEndpointError::InventoryWorkspace)?;
        let repository = profile::ConfigRepository::open(self.store.root())?;
        Ok(match genesis.string_field("folder_binding") {
            Some(binding) => repository.resolve_for_session_binding(workspace, &folder, binding)?,
            None => repository.resolve_for_session(workspace, &folder)?,
        })
    }

    /// Freeze user-selected execution settings at input admission, before worker startup or
    /// provider I/O. The immutable config asset retains the rest of the submission context.
    pub fn submission_snapshot(&self, session_id: &str) -> Result<IJsonValue, NativeEndpointError> {
        let config = self.session_config_snapshot(session_id)?;
        let assets = store::AssetStore::new(self.active_folder(session_id)?.join("assets"))?;
        let (digest, _) = config.publish(&assets)?;
        Self::submission_metadata(&config, &digest)
    }

    pub fn submission_metadata(
        config: &profile::ConfigSnapshot,
        digest: &str,
    ) -> Result<IJsonValue, NativeEndpointError> {
        let provider = config
            .session_settings
            .as_ref()
            .map(|s| s.provider.as_str())
            .or(config.workspace.policy.provider.as_deref())
            .or(config.settings.default_provider.as_deref());
        let model = config
            .session_settings
            .as_ref()
            .map(|s| s.model.as_str())
            .or(config.workspace.policy.model.as_deref())
            .or(config.settings.default_model.as_deref());
        Ok(IJsonValue::parse(&serde_json::to_vec(&json!({
            "configDigest": digest, "provider": provider, "model": model,
            "reasoningEffort": config.session_settings.as_ref().and_then(|s| s.reasoning_effort.as_deref()),
        }))?)?)
    }

    /// Older input records predate submission snapshots. Their turn's immutable run config
    /// still records the selection. Expose that recorded fact without changing journal identity
    /// or consulting today's mutable session settings. Missing historical assets do not hide history.
    pub fn hydrate_historical_submissions(
        &self,
        session_id: &str,
        entries: &mut [crate::SessionHistoryEntry],
    ) {
        let Ok(folder) = self.active_folder(session_id) else {
            return;
        };
        let Ok(bytes) = fs::read(folder.join("main.jsonl")) else {
            return;
        };
        let Some(projection) = scan_valid_prefix(&bytes, 1).projection else {
            return;
        };
        let Ok(assets) = store::AssetStore::new(folder.join("assets")) else {
            return;
        };
        let mut digest = None;
        let mut inputs = std::collections::BTreeMap::new();
        let mut submissions = std::collections::BTreeMap::new();
        for event in &projection.events {
            let Ok(value) = serde_json::to_value(event.raw()) else {
                continue;
            };
            match event.kind() {
                EventKind::Genesis => {
                    digest = value
                        .get("config")
                        .and_then(|c| c.get("digest"))
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                }
                EventKind::RunStart => {
                    digest = value
                        .get("config_digest")
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                }
                EventKind::Input => {
                    inputs.insert(event.seq(), value);
                }
                EventKind::TurnOpen => {
                    let Some(seqs) = value
                        .get("trigger")
                        .and_then(|t| t.get("inputs"))
                        .and_then(Value::as_array)
                    else {
                        continue;
                    };
                    let historical = digest.as_deref().and_then(|digest| {
                        let bytes = assets.read_verified(&format!("sha256-{digest}")).ok()?;
                        let config = profile::ConfigSnapshot::decode(&bytes).ok()?;
                        Self::submission_metadata(&config, digest).ok()
                    });
                    for seq in seqs.iter().filter_map(Value::as_u64) {
                        let submission = inputs
                            .get(&seq)
                            .and_then(|input| input.get("submission"))
                            .and_then(|s| IJsonValue::parse(&serde_json::to_vec(s).ok()?).ok())
                            .or_else(|| historical.clone());
                        if let Some(submission) = submission {
                            submissions.insert(format!("input-{seq}"), submission);
                        }
                    }
                }
                _ => {}
            }
        }
        for entry in entries {
            if entry.event.event_type != "user/message" {
                continue;
            }
            let Ok(mut data) = serde_json::to_value(&entry.event.data) else {
                continue;
            };
            if data.get("submission").is_some() {
                continue;
            }
            let Some(submission) = data
                .get("id")
                .and_then(Value::as_str)
                .and_then(|id| submissions.get(id))
            else {
                continue;
            };
            let Ok(value) = serde_json::to_value(submission) else {
                continue;
            };
            data["submission"] = value;
            if let Ok(bytes) = serde_json::to_vec(&data) {
                if let Ok(value) = IJsonValue::parse(&bytes) {
                    entry.event.data = value;
                }
            }
        }
    }

    pub fn create_session(
        &self,
        session_id: &str,
        workspace: &str,
        config_digest: &str,
        resume: ResumePolicy,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<bool, NativeEndpointError> {
        self.create_session_with_operation(CreateSessionWrite {
            session_id,
            workspace,
            config_digest,
            resume,
            timestamp,
            origin,
            operation: "create",
        })
    }

    /// Session Endpoint v2 creation uses the registration name as the
    /// durable origin operation; the generic Slice-6 API above retains its
    /// original operation for existing callers.
    pub fn create_session_for_endpoint(
        &self,
        session_id: &str,
        workspace: &str,
        config_digest: &str,
        resume: ResumePolicy,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<bool, NativeEndpointError> {
        self.create_session_with_operation(CreateSessionWrite {
            session_id,
            workspace,
            config_digest,
            resume,
            timestamp,
            origin,
            operation: "session.create",
        })
    }

    fn create_session_with_operation(
        &self,
        write: CreateSessionWrite<'_>,
    ) -> Result<bool, NativeEndpointError> {
        validate_session_id(write.session_id)?;
        if write.origin.target != write.session_id || write.origin.op != write.operation {
            return Err(NativeEndpointError::Origin);
        }
        let event = event(json!({
            "v": 1,
            "seq": 1,
            "ts": write.timestamp,
            "kind": "genesis",
            "thread": write.session_id,
            "workspace": write.workspace,
            "format": 1,
            "min_reader": 1,
            "min_writer": 1,
            "resume": write.resume,
            "config": {"digest": write.config_digest},
            "origin_key": write.origin.key,
            "origin_tuple": write.origin,
        }))?;
        Ok(self
            .store
            .create_thread(write.session_id, event)
            .map_err(map_store_error)?
            .created)
    }

    pub fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        content: IJsonValue,
        steer: bool,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.prompt_with_operation(session_id, timestamp, origin, content, steer, "submit")
    }

    pub fn prompt_for_endpoint(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        content: IJsonValue,
        steer: bool,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.prompt_with_operation(
            session_id,
            timestamp,
            origin,
            content,
            steer,
            "session.prompt",
        )
    }

    fn prompt_with_operation(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        content: IJsonValue,
        steer: bool,
        operation: &str,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.keyed(session_id, origin, operation, |seq| {
            let submission = self.submission_snapshot(session_id)?;
            let mut value = json!({
                "v": 1, "seq": seq, "ts": timestamp, "kind": "input",
                "content": content, "origin_key": origin.key, "origin_tuple": origin,
                "submission": submission,
            });
            if steer {
                value["steer"] = Value::Bool(true);
            }
            event(value)
        })
    }

    pub fn update_queue(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        supersedes: &[SeqRange],
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.keyed(session_id, origin, "queue_edit", |seq| {
            event(json!({
                "v": 1, "seq": seq, "ts": timestamp, "kind": "queue_edit",
                "supersedes": supersedes, "origin_key": origin.key, "origin_tuple": origin,
            }))
        })
    }

    pub fn rename_session(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.rename_session_with_operation(session_id, timestamp, origin, title, "rename")
    }

    pub fn rename_session_for_endpoint(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.rename_session_with_operation(session_id, timestamp, origin, title, "session.rename")
    }

    fn rename_session_with_operation(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
        operation: &str,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.keyed(session_id, origin, operation, |seq| {
            event(json!({
                "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,
                "origin_key": origin.key, "origin_tuple": origin,
            }))
        })
    }

    /// Persists the deterministic first-input title only when no title has
    /// ever been written. The decision and append share the ledger lock, so
    /// concurrent dispatchers can start at most one model refinement.
    pub fn seed_automatic_title_if_missing(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<Option<MutationReceipt>, NativeEndpointError> {
        self.validate_origin(session_id, origin, AUTOMATIC_TITLE_SEED_OPERATION)?;
        let outcome = self
            .store
            .append_keyed_with_projection_if(session_id, origin, |seq, projection| {
                if projection.events.iter().any(|item| {
                    *item.kind() == EventKind::Meta && item.string_field("title").is_some()
                }) {
                    return Ok(None);
                }
                event(json!({
                    "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,
                    "origin_key": origin.key, "origin_tuple": origin,
                }))
                .map(Some)
                .map_err(store_error)
            })
            .map_err(map_store_error)?;
        Ok(match outcome {
            ConditionalAppendOutcome::Appended(outcome) => Some(outcome.into()),
            ConditionalAppendOutcome::Skipped => None,
        })
    }

    /// Appends a session notice as a keyed `meta` record. The caller validates
    /// nothing about the running worker: the store lock serializes the append
    /// against the worker's own writes, exactly as the automatic title does.
    pub fn record_session_notice(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        notice: &SessionNotice,
    ) -> Result<Option<MutationReceipt>, NativeEndpointError> {
        self.validate_origin(session_id, origin, SESSION_NOTICE_OPERATION)?;
        let payload = json!({
            "severity": notice.severity.as_str(),
            "classification": notice.classification,
            "operation": notice.operation,
            "message": notice.message,
        });
        let outcome =
            self.store
                .append_keyed_with_projection_if(session_id, origin, |seq, projection| {
                    // A notice that repeats the newest one, with no input in between,
                    // says nothing new (the same degraded launch on every worker
                    // restart); fold it. An input after it is a fresh attempt the
                    // user made, and its outcome must be shown again.
                    let latest_notice = projection.events.iter().rposition(|item| {
                        *item.kind() == EventKind::Meta && item.has_field("notice")
                    });
                    if let Some(index) = latest_notice {
                        let repeats = serde_json::to_value(projection.events[index].raw())
                            .ok()
                            .and_then(|value| value.get("notice").cloned())
                            .is_some_and(|previous| previous == payload);
                        let input_since = projection.events[index + 1..]
                            .iter()
                            .any(|item| *item.kind() == EventKind::Input);
                        if repeats && !input_since {
                            return Ok(None);
                        }
                    }
                    event(json!({
                        "v": 1, "seq": seq, "ts": timestamp, "kind": "meta",
                        "notice": payload,
                        "origin_key": origin.key, "origin_tuple": origin,
                    }))
                    .map(Some)
                    .map_err(store_error)
                })
                .map_err(map_store_error)?;
        Ok(match outcome {
            ConditionalAppendOutcome::Appended(outcome) => Some(outcome.into()),
            ConditionalAppendOutcome::Skipped => None,
        })
    }

    /// Replaces an automatic fallback only while it is still authoritative.
    /// Any manual or otherwise-owned title written before this compare-and-
    /// append wins permanently.
    pub fn refine_automatic_title(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<Option<MutationReceipt>, NativeEndpointError> {
        self.validate_origin(session_id, origin, AUTOMATIC_TITLE_REFINE_OPERATION)?;
        let outcome = self
            .store
            .append_keyed_with_projection_if(session_id, origin, |seq, projection| {
                let latest_title = projection.events.iter().rev().find(|item| {
                    *item.kind() == EventKind::Meta && item.string_field("title").is_some()
                });
                let remains_automatic_seed = latest_title
                    .map(Event::origin_tuple)
                    .transpose()
                    .map_err(store::StoreError::from)?
                    .flatten()
                    .is_some_and(|tuple| tuple.op == AUTOMATIC_TITLE_SEED_OPERATION);
                if !remains_automatic_seed {
                    return Ok(None);
                }
                event(json!({
                    "v": 1, "seq": seq, "ts": timestamp, "kind": "meta", "title": title,
                    "origin_key": origin.key, "origin_tuple": origin,
                }))
                .map(Some)
                .map_err(store_error)
            })
            .map_err(map_store_error)?;
        Ok(match outcome {
            ConditionalAppendOutcome::Appended(outcome) => Some(outcome.into()),
            ConditionalAppendOutcome::Skipped => None,
        })
    }

    /// Keyed append whose builder owns the locked main ledger (checkpoint and
    /// asset publication allowed before the event). `None` when the builder
    /// declined; a repeated origin key returns the original seq as
    /// deduplicated. The caller validates the origin operation.
    pub fn author_keyed_with_ledger<F>(
        &self,
        session_id: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<Option<MutationReceipt>, NativeEndpointError>
    where
        F: FnOnce(&mut store::LockedLedger) -> Result<Option<Event>, store::StoreError>,
    {
        validate_session_id(session_id)?;
        if origin.target != session_id {
            return Err(NativeEndpointError::Origin);
        }
        let outcome = self
            .store
            .append_keyed_with_ledger_if(session_id, origin, build)
            .map_err(map_store_error)?;
        Ok(match outcome {
            ConditionalAppendOutcome::Appended(outcome) => Some(outcome.into()),
            ConditionalAppendOutcome::Skipped => None,
        })
    }

    pub fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.cancel_with_operation(session_id, timestamp, origin, "stop")
    }

    pub fn cancel_for_endpoint(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        self.cancel_with_operation(session_id, timestamp, origin, "session.cancel")
    }

    fn cancel_with_operation(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        operation: &str,
    ) -> Result<MutationReceipt, NativeEndpointError> {
        validate_session_id(session_id)?;
        self.validate_origin(session_id, origin, operation)?;
        let outcome = self
            .store
            .append_keyed_with_projection(session_id, origin, |seq, projection| {
                let generation = projection
                    .events
                    .iter()
                    .filter(|item| matches!(item.kind(), schema::EventKind::StopRequested))
                    .filter_map(|item| item.integer_field("generation"))
                    .max()
                    .unwrap_or(0)
                    + 1;
                event(json!({
                "v": 1, "seq": seq, "ts": timestamp, "kind": "stop_requested",
                "generation": generation, "origin_key": origin.key, "origin_tuple": origin,
                }))
                .map_err(store_error)
            })
            .map_err(map_store_error)?;
        Ok(outcome.into())
    }

    pub fn fork_session(
        &self,
        operation_id: &str,
        source: &str,
        destination: &str,
        timestamp: &str,
    ) -> Result<String, NativeEndpointError> {
        validate_session_id(source)?;
        validate_session_id(destination)?;
        self.store
            .begin_fork(operation_id, source, destination, timestamp)
            .map_err(map_store_error)?;
        self.store.recover_rewrites().map_err(map_store_error)?;
        Ok(destination.to_owned())
    }

    pub fn archive_session(&self, session_id: &str) -> Result<ArchiveResult, NativeEndpointError> {
        validate_session_id(session_id)?;
        self.store.archive(session_id).map_err(map_store_error)?;
        Ok(ArchiveResult {
            archived_session_ids: vec![session_id.to_owned()],
        })
    }

    pub fn unarchive_session(
        &self,
        session_id: &str,
    ) -> Result<UnarchiveResult, NativeEndpointError> {
        validate_session_id(session_id)?;
        self.store.unarchive(session_id).map_err(map_store_error)?;
        Ok(UnarchiveResult {
            session_id: session_id.to_owned(),
        })
    }

    pub fn reconcile_projection(&self, session_id: &str) -> Result<i64, NativeEndpointError> {
        let journal = self.reconciled_journal(session_id)?;
        match journal.last_seq()? {
            Some(seq) => i64::try_from(seq).map_err(|_| NativeEndpointError::SequenceOverflow(seq)),
            None => Ok(-1),
        }
    }

    fn reconciled_journal(&self, session_id: &str) -> Result<EndpointJournal, NativeEndpointError> {
        let folder = self.active_folder(session_id)?;
        let bytes = fs::read(folder.join("main.jsonl"))?;
        let scan = scan_valid_prefix(&bytes, 1);
        let ledger = scan.projection.ok_or_else(|| {
            NativeEndpointError::Store(store::StoreError::Corruption(
                "thread ledger has no valid genesis prefix".to_owned(),
            ))
        })?;
        let journal = EndpointJournal::open(&folder)?;
        Projector::default().reconcile(&ledger.events, &journal)?;
        Ok(journal)
    }

    pub fn history(
        &self,
        session_id: &str,
        before_seq: Option<u64>,
        max_messages: Option<usize>,
    ) -> Result<HistoryPage, NativeEndpointError> {
        let journal = self.reconciled_journal(session_id)?;
        Ok(history_page(&journal.records()?, before_seq, max_messages)?)
    }

    fn keyed<F>(
        &self,
        session_id: &str,
        origin: &OriginTuple,
        expected_operation: &str,
        build: F,
    ) -> Result<MutationReceipt, NativeEndpointError>
    where
        F: FnOnce(u64) -> Result<Event, NativeEndpointError>,
    {
        self.validate_origin(session_id, origin, expected_operation)?;
        let outcome = self
            .store
            .append_keyed(session_id, origin, |seq| build(seq).map_err(store_error))
            .map_err(map_store_error)?;
        Ok(outcome.into())
    }

    fn validate_origin(
        &self,
        session_id: &str,
        origin: &OriginTuple,
        expected_operation: &str,
    ) -> Result<(), NativeEndpointError> {
        validate_session_id(session_id)?;
        if origin.target != session_id || origin.op != expected_operation {
            return Err(NativeEndpointError::Origin);
        }
        Ok(())
    }

    fn active_folder(&self, session_id: &str) -> Result<std::path::PathBuf, NativeEndpointError> {
        validate_session_id(session_id)?;
        if self.store.root().join("archive").join(session_id).is_dir() {
            return Err(NativeEndpointError::Archived);
        }
        let folder = self.store.root().join("threads").join(session_id);
        if !folder.is_dir() {
            return Err(NativeEndpointError::NotFound);
        }
        Ok(folder)
    }
}

fn event(value: Value) -> Result<Event, NativeEndpointError> {
    let bytes = serde_json::to_vec(&value)?;
    Ok(Event::from_value(IJsonValue::parse(&bytes)?)?)
}

fn store_error(error: NativeEndpointError) -> store::StoreError {
    store::StoreError::Corruption(error.to_string())
}

fn map_store_error(error: store::StoreError) -> NativeEndpointError {
    match error {
        store::StoreError::Archived => NativeEndpointError::Archived,
        store::StoreError::NotFound => NativeEndpointError::NotFound,
        other => NativeEndpointError::Store(other),
    }
}

#[derive(Debug, Error)]
pub enum NativeEndpointError {
    #[error("endpoint IO failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("endpoint JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("endpoint schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("endpoint store failed: {0}")]
    Store(#[from] store::StoreError),
    #[error("endpoint journal failed: {0}")]
    Journal(#[from] crate::JournalError),
    #[error("endpoint projection failed: {0}")]
    Projection(#[from] crate::ProjectionError),
    #[error("endpoint profile failed: {0}")]
    Profile(#[from] profile::ProfileError),
    #[error("endpoint history failed: {0}")]
    History(#[from] crate::history::HistoryError),
    #[error("endpoint type failed: {0}")]
    Type(#[from] crate::types::EndpointTypeError),
    #[error("origin tuple does not target this session/operation")]
    Origin,
    #[error("session is archived")]
    Archived,
    #[error("session was not found")]
    NotFound,
    #[error("endpoint sequence {0} cannot be represented on the v2 wire")]
    SequenceOverflow(u64),
    #[error("inventory ledger lacks a timestamp")]
    InventoryTimestamp,
    #[error("inventory genesis lacks a workspace id")]
    InventoryWorkspace,
}
