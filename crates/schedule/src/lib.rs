//! Durable, host-owned schedule authority for Slice 14C.
//!
//! This crate intentionally exposes an internal seam only. Slice 14F owns any
//! Client capability and transport binding.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::Write as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use store::{FullSync, NamedLock, StoreError};
use thiserror::Error;
use uuid::Uuid;

const FORMAT: u64 = 1;
const LOG_NAME: &str = "schedules.jsonl";
const LOCK_NAME: &str = ".schedules.lock";
/// Names this authority used before the rename. The log is moved on open; the
/// lock file carries no state and is simply removed.
const LEGACY_LOG_NAME: &str = "schedules-v1.jsonl";
const LEGACY_LOCK_NAME: &str = ".schedules-v1.lock";
const MAX_TEXT_BYTES: usize = 16 * 1024;
const TEN_YEARS_MINUTES: i64 = 366 * 10 * 24 * 60;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTuple {
    pub client_id: String,
    pub key: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissedPolicy {
    SkipAndRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleDefinition {
    pub id: String,
    pub name: String,
    pub workspace_id: String,
    pub cron: String,
    pub time_zone: String,
    pub prompt: String,
    pub permission_mode: String,
    #[serde(default)]
    pub model_id: Option<String>,
    pub enabled: bool,
    pub missed_policy: MissedPolicy,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Idle,
    Claimed,
    Running,
    Parked,
    Completed,
    Failed,
    Interrupted,
}

impl RunStatus {
    #[must_use]
    pub fn is_active(self) -> bool {
        matches!(self, Self::Claimed | Self::Running | Self::Parked)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimReason {
    Scheduled,
    Manual,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchClaim {
    pub task_id: String,
    pub claim_id: String,
    pub scheduled_for: String,
    pub reason: ClaimReason,
    pub definition: ScheduleDefinition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleView {
    pub definition: ScheduleDefinition,
    pub next_run_at: Option<String>,
    pub missed_at: Option<String>,
    pub last_scheduled_at: Option<String>,
    pub last_started_at: Option<String>,
    pub last_finished_at: Option<String>,
    pub last_status: RunStatus,
    pub last_error: Option<String>,
    pub last_session_id: Option<String>,
    pub last_input_seq: Option<u64>,
    pub active_claim_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum MutationResult {
    Saved { task: ScheduleView },
    Deleted { task_id: String, deleted: bool },
    RunNow { claim: LaunchClaim },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum LogOperation {
    Save {
        format: u64,
        seq: u64,
        at: String,
        origin: OriginTuple,
        fingerprint: String,
        definition: ScheduleDefinition,
        next_run_at: Option<String>,
    },
    Delete {
        format: u64,
        seq: u64,
        at: String,
        origin: OriginTuple,
        fingerprint: String,
        task_id: String,
        deleted: bool,
    },
    Claim {
        format: u64,
        seq: u64,
        at: String,
        origin: Option<OriginTuple>,
        fingerprint: Option<String>,
        task_id: String,
        claim_id: String,
        scheduled_for: String,
        next_run_at: Option<String>,
        reason: ClaimReason,
    },
    Bind {
        format: u64,
        seq: u64,
        at: String,
        task_id: String,
        claim_id: String,
        session_id: String,
        input_seq: u64,
    },
    Status {
        format: u64,
        seq: u64,
        at: String,
        task_id: String,
        claim_id: String,
        status: RunStatus,
        error: Option<String>,
    },
    Missed {
        format: u64,
        seq: u64,
        at: String,
        task_id: String,
        missed_at: String,
        next_run_at: String,
    },
}

impl LogOperation {
    fn seq(&self) -> u64 {
        match self {
            Self::Save { seq, .. }
            | Self::Delete { seq, .. }
            | Self::Claim { seq, .. }
            | Self::Bind { seq, .. }
            | Self::Status { seq, .. }
            | Self::Missed { seq, .. } => *seq,
        }
    }
}

#[derive(Clone, Debug)]
struct OriginRecord {
    fingerprint: String,
    result: MutationResult,
}

#[derive(Default)]
struct State {
    seq: u64,
    tasks: BTreeMap<String, ScheduleView>,
    origins: BTreeMap<String, OriginRecord>,
}

#[derive(Clone, Debug)]
pub struct ScheduleAuthority {
    root: PathBuf,
}

impl ScheduleAuthority {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, ScheduleError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        store::retire_legacy_name(&root, LOG_NAME, LEGACY_LOG_NAME)?;
        let legacy_lock = root.join(LEGACY_LOCK_NAME);
        if legacy_lock.exists() {
            fs::remove_file(&legacy_lock)?;
        }
        Ok(Self { root })
    }

    pub fn list(&self, workspace_id: Option<&str>) -> Result<Vec<ScheduleView>, ScheduleError> {
        if let Some(workspace_id) = workspace_id {
            validate_workspace(workspace_id)?;
        }
        let _lock = NamedLock::shared(self.root.join(LOCK_NAME))?;
        let (state, _) = load_state(&self.root.join(LOG_NAME))?;
        Ok(state
            .tasks
            .into_values()
            .filter(|task| workspace_id.is_none_or(|id| task.definition.workspace_id == id))
            .collect())
    }

    pub fn save(
        &self,
        origin: OriginTuple,
        definition: ScheduleDefinition,
        now: DateTime<Utc>,
    ) -> Result<ScheduleView, ScheduleError> {
        validate_origin(&origin)?;
        validate_definition(&definition)?;
        let fingerprint = fingerprint(&serde_json::json!({"save": definition}))?;
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        if let Some(record) = state.origins.get(&origin_id(&origin)) {
            if record.fingerprint != fingerprint {
                return Err(ScheduleError::OriginCollision);
            }
            let MutationResult::Saved { task } = &record.result else {
                return Err(ScheduleError::OriginCollision);
            };
            return Ok(task.clone());
        }
        if state
            .tasks
            .get(&definition.id)
            .is_some_and(|task| task.last_status.is_active())
        {
            return Err(ScheduleError::Active(definition.id));
        }
        let next_run_at = if definition.enabled {
            Some(CronSchedule::parse(&definition.cron)?.next_after(now, &definition.time_zone)?)
        } else {
            None
        };
        let op = LogOperation::Save {
            format: FORMAT,
            seq: state.seq + 1,
            at: timestamp(now),
            origin,
            fingerprint,
            definition,
            next_run_at: next_run_at.map(timestamp),
        };
        append(&path, &op)?;
        apply(&mut state, &op)?;
        let id = match &op {
            LogOperation::Save { definition, .. } => &definition.id,
            _ => unreachable!(),
        };
        state.tasks.get(id).cloned().ok_or(ScheduleError::Corrupt(
            "saved task missing after replay".to_owned(),
        ))
    }

    pub fn delete(
        &self,
        origin: OriginTuple,
        task_id: &str,
        now: DateTime<Utc>,
    ) -> Result<bool, ScheduleError> {
        validate_origin(&origin)?;
        validate_uuid(task_id, "task id")?;
        let fingerprint = fingerprint(&serde_json::json!({"delete": task_id}))?;
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        if let Some(record) = state.origins.get(&origin_id(&origin)) {
            if record.fingerprint != fingerprint {
                return Err(ScheduleError::OriginCollision);
            }
            let MutationResult::Deleted { deleted, .. } = record.result else {
                return Err(ScheduleError::OriginCollision);
            };
            return Ok(deleted);
        }
        if state
            .tasks
            .get(task_id)
            .is_some_and(|task| task.last_status.is_active())
        {
            return Err(ScheduleError::Active(task_id.to_owned()));
        }
        let deleted = state.tasks.contains_key(task_id);
        let op = LogOperation::Delete {
            format: FORMAT,
            seq: state.seq + 1,
            at: timestamp(now),
            origin,
            fingerprint,
            task_id: task_id.to_owned(),
            deleted,
        };
        append(&path, &op)?;
        apply(&mut state, &op)?;
        Ok(deleted)
    }

    pub fn run_now(
        &self,
        origin: OriginTuple,
        task_id: &str,
        now: DateTime<Utc>,
    ) -> Result<LaunchClaim, ScheduleError> {
        validate_origin(&origin)?;
        validate_uuid(task_id, "task id")?;
        let fingerprint = fingerprint(&serde_json::json!({"run_now": task_id}))?;
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        if let Some(record) = state.origins.get(&origin_id(&origin)) {
            if record.fingerprint != fingerprint {
                return Err(ScheduleError::OriginCollision);
            }
            let MutationResult::RunNow { claim } = &record.result else {
                return Err(ScheduleError::OriginCollision);
            };
            return Ok(claim.clone());
        }
        let task = state
            .tasks
            .get(task_id)
            .ok_or_else(|| ScheduleError::NotFound(task_id.to_owned()))?;
        if task.last_status.is_active() {
            return Err(ScheduleError::Active(task_id.to_owned()));
        }
        let claim_id = claim_id(task_id, &timestamp(now), ClaimReason::Manual, Some(&origin));
        let op = LogOperation::Claim {
            format: FORMAT,
            seq: state.seq + 1,
            at: timestamp(now),
            origin: Some(origin),
            fingerprint: Some(fingerprint),
            task_id: task_id.to_owned(),
            claim_id: claim_id.clone(),
            scheduled_for: timestamp(now),
            next_run_at: task.next_run_at.clone(),
            reason: ClaimReason::Manual,
        };
        append(&path, &op)?;
        apply(&mut state, &op)?;
        claim_from_state(&state, task_id, &claim_id)
    }

    /// Claims at most one due occurrence per task. The durable claim precedes
    /// the returned launch work; a crash therefore redrives the same claim.
    pub fn poll_due(&self, now: DateTime<Utc>) -> Result<Vec<LaunchClaim>, ScheduleError> {
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        let mut claims = pending_unbound(&state)?;
        let task_ids = state.tasks.keys().cloned().collect::<Vec<_>>();
        for task_id in task_ids {
            let task = state.tasks.get(&task_id).cloned().expect("known task");
            let Some(raw_due) = &task.next_run_at else {
                continue;
            };
            let due = parse_timestamp(raw_due)?;
            if due > now {
                continue;
            }
            let next = CronSchedule::parse(&task.definition.cron)?
                .next_after(now, &task.definition.time_zone)?;
            if task.last_status.is_active() {
                let op = LogOperation::Missed {
                    format: FORMAT,
                    seq: state.seq + 1,
                    at: timestamp(now),
                    task_id: task_id.clone(),
                    missed_at: timestamp(due),
                    next_run_at: timestamp(next),
                };
                append(&path, &op)?;
                apply(&mut state, &op)?;
                continue;
            }
            let scheduled_for = timestamp(due);
            let id = claim_id(&task_id, &scheduled_for, ClaimReason::Scheduled, None);
            let op = LogOperation::Claim {
                format: FORMAT,
                seq: state.seq + 1,
                at: timestamp(now),
                origin: None,
                fingerprint: None,
                task_id: task_id.clone(),
                claim_id: id.clone(),
                scheduled_for,
                next_run_at: Some(timestamp(next)),
                reason: ClaimReason::Scheduled,
            };
            append(&path, &op)?;
            apply(&mut state, &op)?;
            claims.push(claim_from_state(&state, &task_id, &id)?);
        }
        claims.sort_by(|left, right| left.claim_id.as_bytes().cmp(right.claim_id.as_bytes()));
        claims.dedup_by(|left, right| left.claim_id == right.claim_id);
        Ok(claims)
    }

    /// Startup skips downtime occurrences (recording the earliest due one),
    /// while returning every unbound claim for keyed redrive.
    pub fn recover(&self, now: DateTime<Utc>) -> Result<Vec<LaunchClaim>, ScheduleError> {
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        let claims = pending_unbound(&state)?;
        let task_ids = state.tasks.keys().cloned().collect::<Vec<_>>();
        for task_id in task_ids {
            let task = state.tasks.get(&task_id).cloned().expect("known task");
            let Some(raw_due) = &task.next_run_at else {
                continue;
            };
            let due = parse_timestamp(raw_due)?;
            if due > now || task.last_status.is_active() {
                continue;
            }
            let next = CronSchedule::parse(&task.definition.cron)?
                .next_after(now, &task.definition.time_zone)?;
            let op = LogOperation::Missed {
                format: FORMAT,
                seq: state.seq + 1,
                at: timestamp(now),
                task_id: task_id.clone(),
                missed_at: timestamp(due),
                next_run_at: timestamp(next),
            };
            append(&path, &op)?;
            apply(&mut state, &op)?;
        }
        Ok(claims)
    }

    pub fn bind_launch(
        &self,
        task_id: &str,
        claim_id: &str,
        session_id: &str,
        input_seq: u64,
        now: DateTime<Utc>,
    ) -> Result<ScheduleView, ScheduleError> {
        validate_uuid(session_id, "session id")?;
        if input_seq == 0 {
            return Err(ScheduleError::Invalid("input seq"));
        }
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        let task = state
            .tasks
            .get(task_id)
            .ok_or_else(|| ScheduleError::NotFound(task_id.to_owned()))?;
        if task.active_claim_id.as_deref() != Some(claim_id) {
            return Err(ScheduleError::ClaimMismatch);
        }
        if let (Some(existing_session), Some(existing_input)) =
            (&task.last_session_id, task.last_input_seq)
        {
            if existing_session == session_id && existing_input == input_seq {
                return Ok(task.clone());
            }
            return Err(ScheduleError::ClaimMismatch);
        }
        let op = LogOperation::Bind {
            format: FORMAT,
            seq: state.seq + 1,
            at: timestamp(now),
            task_id: task_id.to_owned(),
            claim_id: claim_id.to_owned(),
            session_id: session_id.to_owned(),
            input_seq,
        };
        append(&path, &op)?;
        apply(&mut state, &op)?;
        Ok(state.tasks.get(task_id).expect("bound task").clone())
    }

    pub fn record_status(
        &self,
        task_id: &str,
        claim_id: &str,
        status: RunStatus,
        error: Option<String>,
        now: DateTime<Utc>,
    ) -> Result<ScheduleView, ScheduleError> {
        if matches!(status, RunStatus::Idle | RunStatus::Claimed) {
            return Err(ScheduleError::Invalid("observed run status"));
        }
        if error
            .as_ref()
            .is_some_and(|value| value.len() > MAX_TEXT_BYTES)
        {
            return Err(ScheduleError::Invalid("run error"));
        }
        let _lock = NamedLock::exclusive(self.root.join(LOCK_NAME))?;
        let path = self.root.join(LOG_NAME);
        let (mut state, valid_len) = load_state(&path)?;
        repair_tail(&path, valid_len)?;
        sync_existing(&path)?;
        let task = state
            .tasks
            .get(task_id)
            .ok_or_else(|| ScheduleError::NotFound(task_id.to_owned()))?;
        if task.active_claim_id.as_deref() != Some(claim_id) {
            return Err(ScheduleError::ClaimMismatch);
        }
        if task.last_status == status && task.last_error == error {
            return Ok(task.clone());
        }
        if task.last_status == RunStatus::Claimed {
            return Err(ScheduleError::ClaimMismatch);
        }
        if !task.last_status.is_active() {
            return Err(ScheduleError::ClaimMismatch);
        }
        let op = LogOperation::Status {
            format: FORMAT,
            seq: state.seq + 1,
            at: timestamp(now),
            task_id: task_id.to_owned(),
            claim_id: claim_id.to_owned(),
            status,
            error,
        };
        append(&path, &op)?;
        apply(&mut state, &op)?;
        Ok(state.tasks.get(task_id).expect("status task").clone())
    }

    #[must_use]
    pub fn log_path(&self) -> PathBuf {
        self.root.join(LOG_NAME)
    }
}

fn validate_definition(definition: &ScheduleDefinition) -> Result<(), ScheduleError> {
    validate_uuid(&definition.id, "task id")?;
    validate_workspace(&definition.workspace_id)?;
    for (value, name) in [
        (&definition.name, "name"),
        (&definition.prompt, "prompt"),
        (&definition.permission_mode, "permission mode"),
    ] {
        if value.trim().is_empty() || value.len() > MAX_TEXT_BYTES {
            return Err(ScheduleError::Invalid(name));
        }
    }
    if definition
        .model_id
        .as_ref()
        .is_some_and(|value| value.is_empty() || value.len() > 512)
    {
        return Err(ScheduleError::Invalid("model id"));
    }
    let _: Tz = definition
        .time_zone
        .parse()
        .map_err(|_| ScheduleError::InvalidTimeZone(definition.time_zone.clone()))?;
    CronSchedule::parse(&definition.cron)?;
    Ok(())
}

fn validate_origin(origin: &OriginTuple) -> Result<(), ScheduleError> {
    for (value, name) in [
        (&origin.client_id, "origin client"),
        (&origin.key, "origin key"),
    ] {
        if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
            return Err(ScheduleError::Invalid(name));
        }
    }
    Ok(())
}

fn validate_workspace(value: &str) -> Result<(), ScheduleError> {
    if value.is_empty()
        || value.len() > 128
        || value.starts_with('.')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(ScheduleError::Invalid("workspace id"));
    }
    Ok(())
}

fn validate_uuid(value: &str, name: &'static str) -> Result<(), ScheduleError> {
    let parsed = Uuid::parse_str(value).map_err(|_| ScheduleError::Invalid(name))?;
    if parsed.hyphenated().to_string() != value
        || value.bytes().any(|byte| byte.is_ascii_uppercase())
    {
        return Err(ScheduleError::Invalid(name));
    }
    Ok(())
}

fn claim_from_state(
    state: &State,
    task_id: &str,
    claim_id: &str,
) -> Result<LaunchClaim, ScheduleError> {
    let task = state
        .tasks
        .get(task_id)
        .ok_or_else(|| ScheduleError::NotFound(task_id.to_owned()))?;
    if task.active_claim_id.as_deref() != Some(claim_id) {
        return Err(ScheduleError::ClaimMismatch);
    }
    let operations = task.last_scheduled_at.clone().ok_or_else(|| {
        ScheduleError::Corrupt("active claim has no scheduled timestamp".to_owned())
    })?;
    Ok(LaunchClaim {
        task_id: task_id.to_owned(),
        claim_id: claim_id.to_owned(),
        scheduled_for: operations,
        reason: if claim_id.starts_with("scheduled-") {
            ClaimReason::Scheduled
        } else {
            ClaimReason::Manual
        },
        definition: task.definition.clone(),
    })
}

fn pending_unbound(state: &State) -> Result<Vec<LaunchClaim>, ScheduleError> {
    state
        .tasks
        .values()
        .filter(|task| {
            task.last_status == RunStatus::Claimed
                && task.active_claim_id.is_some()
                && task.last_session_id.is_none()
        })
        .map(|task| {
            claim_from_state(
                state,
                &task.definition.id,
                task.active_claim_id.as_deref().expect("filtered claim"),
            )
        })
        .collect()
}

fn apply(state: &mut State, operation: &LogOperation) -> Result<(), ScheduleError> {
    if operation.seq() != state.seq + 1 {
        return Err(ScheduleError::Corrupt(
            "non-contiguous schedule sequence".to_owned(),
        ));
    }
    match operation {
        LogOperation::Save {
            format,
            at,
            origin,
            fingerprint,
            definition,
            next_run_at,
            ..
        } => {
            check_format(*format)?;
            validate_origin(origin)?;
            validate_definition(definition)?;
            validate_optional_timestamp(next_run_at)?;
            let existing = state.tasks.get(&definition.id);
            if existing.is_some_and(|task| task.last_status.is_active()) {
                return Err(ScheduleError::Corrupt(
                    "save replaced active task".to_owned(),
                ));
            }
            let created_at = existing.map_or_else(|| at.clone(), |task| task.created_at.clone());
            let mut view = existing.cloned().unwrap_or_else(|| ScheduleView {
                definition: definition.clone(),
                next_run_at: None,
                missed_at: None,
                last_scheduled_at: None,
                last_started_at: None,
                last_finished_at: None,
                last_status: RunStatus::Idle,
                last_error: None,
                last_session_id: None,
                last_input_seq: None,
                active_claim_id: None,
                created_at: created_at.clone(),
                updated_at: at.clone(),
            });
            view.definition = definition.clone();
            view.next_run_at = next_run_at.clone();
            view.updated_at = at.clone();
            state.tasks.insert(definition.id.clone(), view.clone());
            insert_origin(
                state,
                origin,
                fingerprint,
                MutationResult::Saved { task: view },
            )?;
        }
        LogOperation::Delete {
            format,
            origin,
            fingerprint,
            task_id,
            deleted,
            ..
        } => {
            check_format(*format)?;
            validate_origin(origin)?;
            if *deleted != state.tasks.contains_key(task_id) {
                return Err(ScheduleError::Corrupt("delete result mismatch".to_owned()));
            }
            if state
                .tasks
                .get(task_id)
                .is_some_and(|task| task.last_status.is_active())
            {
                return Err(ScheduleError::Corrupt("deleted active task".to_owned()));
            }
            state.tasks.remove(task_id);
            insert_origin(
                state,
                origin,
                fingerprint,
                MutationResult::Deleted {
                    task_id: task_id.clone(),
                    deleted: *deleted,
                },
            )?;
        }
        LogOperation::Claim {
            format,
            at,
            origin,
            fingerprint,
            task_id,
            claim_id,
            scheduled_for,
            next_run_at,
            reason,
            ..
        } => {
            check_format(*format)?;
            parse_timestamp(scheduled_for)?;
            validate_optional_timestamp(next_run_at)?;
            let task = state.tasks.get_mut(task_id).ok_or_else(|| {
                ScheduleError::Corrupt("claim references missing task".to_owned())
            })?;
            if task.last_status.is_active() {
                return Err(ScheduleError::Corrupt(
                    "claim overlaps active run".to_owned(),
                ));
            }
            task.next_run_at = next_run_at.clone();
            task.last_scheduled_at = Some(scheduled_for.clone());
            task.last_started_at = None;
            task.last_finished_at = None;
            task.last_status = RunStatus::Claimed;
            task.last_error = None;
            task.last_session_id = None;
            task.last_input_seq = None;
            task.active_claim_id = Some(claim_id.clone());
            task.updated_at = at.clone();
            let launch_claim = LaunchClaim {
                task_id: task_id.clone(),
                claim_id: claim_id.clone(),
                scheduled_for: scheduled_for.clone(),
                reason: *reason,
                definition: task.definition.clone(),
            };
            match (reason, origin, fingerprint) {
                (ClaimReason::Manual, Some(origin), Some(fingerprint)) => insert_origin(
                    state,
                    origin,
                    fingerprint,
                    MutationResult::RunNow {
                        claim: launch_claim,
                    },
                )?,
                (ClaimReason::Scheduled, None, None) => {}
                _ => {
                    return Err(ScheduleError::Corrupt(
                        "claim origin does not match reason".to_owned(),
                    ));
                }
            }
        }
        LogOperation::Bind {
            format,
            at,
            task_id,
            claim_id,
            session_id,
            input_seq,
            ..
        } => {
            check_format(*format)?;
            validate_uuid(session_id, "session id")?;
            let task = active_task(state, task_id, claim_id)?;
            if task.last_session_id.is_some() || task.last_input_seq.is_some() {
                return Err(ScheduleError::Corrupt("claim bound twice".to_owned()));
            }
            task.last_session_id = Some(session_id.clone());
            task.last_input_seq = Some(*input_seq);
            task.last_started_at = Some(at.clone());
            task.last_status = RunStatus::Running;
            task.updated_at = at.clone();
        }
        LogOperation::Status {
            format,
            at,
            task_id,
            claim_id,
            status,
            error,
            ..
        } => {
            check_format(*format)?;
            if matches!(status, RunStatus::Idle | RunStatus::Claimed) {
                return Err(ScheduleError::Corrupt("invalid observed status".to_owned()));
            }
            let task = active_task(state, task_id, claim_id)?;
            if task.last_status == RunStatus::Claimed {
                return Err(ScheduleError::Corrupt(
                    "status observed before launch binding".to_owned(),
                ));
            }
            task.last_status = *status;
            task.last_error = error.clone();
            task.updated_at = at.clone();
            if !status.is_active() {
                task.last_finished_at = Some(at.clone());
                task.active_claim_id = None;
            }
        }
        LogOperation::Missed {
            format,
            at,
            task_id,
            missed_at,
            next_run_at,
            ..
        } => {
            check_format(*format)?;
            parse_timestamp(missed_at)?;
            parse_timestamp(next_run_at)?;
            let task = state.tasks.get_mut(task_id).ok_or_else(|| {
                ScheduleError::Corrupt("missed references missing task".to_owned())
            })?;
            task.missed_at = Some(missed_at.clone());
            task.next_run_at = Some(next_run_at.clone());
            task.updated_at = at.clone();
        }
    }
    state.seq = operation.seq();
    Ok(())
}

fn active_task<'a>(
    state: &'a mut State,
    task_id: &str,
    claim_id: &str,
) -> Result<&'a mut ScheduleView, ScheduleError> {
    let task = state
        .tasks
        .get_mut(task_id)
        .ok_or_else(|| ScheduleError::Corrupt("run references missing task".to_owned()))?;
    if !task.last_status.is_active() || task.active_claim_id.as_deref() != Some(claim_id) {
        return Err(ScheduleError::Corrupt("run claim mismatch".to_owned()));
    }
    Ok(task)
}

fn insert_origin(
    state: &mut State,
    origin: &OriginTuple,
    fingerprint: &str,
    result: MutationResult,
) -> Result<(), ScheduleError> {
    let key = origin_id(origin);
    if state.origins.contains_key(&key) {
        return Err(ScheduleError::Corrupt(
            "origin tuple appears twice".to_owned(),
        ));
    }
    state.origins.insert(
        key,
        OriginRecord {
            fingerprint: fingerprint.to_owned(),
            result,
        },
    );
    Ok(())
}

fn load_state(path: &Path) -> Result<(State, u64), ScheduleError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok((State::default(), 0));
        }
        Err(error) => return Err(error.into()),
    };
    let valid_len = bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    let mut state = State::default();
    for raw in bytes[..valid_len].split(|byte| *byte == b'\n') {
        if raw.is_empty() {
            continue;
        }
        let operation: LogOperation = serde_json::from_slice(raw)
            .map_err(|error| ScheduleError::Corrupt(format!("invalid complete line: {error}")))?;
        if serde_json_canonicalizer::to_vec(&operation)? != raw {
            return Err(ScheduleError::Corrupt(
                "noncanonical complete line".to_owned(),
            ));
        }
        apply(&mut state, &operation)?;
    }
    Ok((state, valid_len as u64))
}

fn repair_tail(path: &Path, valid_len: u64) -> Result<(), ScheduleError> {
    let Ok(metadata) = fs::metadata(path) else {
        return Ok(());
    };
    if metadata.len() != valid_len {
        let file = OpenOptions::new().write(true).open(path)?;
        file.set_len(valid_len)?;
        FullSync::full_sync(&file)?;
    }
    Ok(())
}

fn append(path: &Path, operation: &LogOperation) -> Result<(), ScheduleError> {
    let mut bytes = serde_json_canonicalizer::to_vec(operation)?;
    bytes.push(b'\n');
    let existed = path.exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    file.write_all(&bytes)?;
    FullSync::full_sync(&file)?;
    if !existed {
        File::open(
            path.parent()
                .ok_or(ScheduleError::Invalid("schedule root"))?,
        )?
        .sync_all()?;
    }
    Ok(())
}

fn sync_existing(path: &Path) -> Result<(), ScheduleError> {
    match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => {
            FullSync::full_sync(&file)?;
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn check_format(format: u64) -> Result<(), ScheduleError> {
    if format == FORMAT {
        Ok(())
    } else {
        Err(ScheduleError::Corrupt(
            "unsupported schedule format".to_owned(),
        ))
    }
}

fn origin_id(origin: &OriginTuple) -> String {
    format!("{}\0{}", origin.client_id, origin.key)
}

fn fingerprint(value: &Value) -> Result<String, ScheduleError> {
    Ok(hex_digest(&serde_json_canonicalizer::to_vec(value)?))
}

fn claim_id(
    task_id: &str,
    scheduled_for: &str,
    reason: ClaimReason,
    origin: Option<&OriginTuple>,
) -> String {
    let value = serde_json::json!({
        "origin": origin,
        "reason": reason,
        "scheduled_for": scheduled_for,
        "task_id": task_id,
    });
    let bytes = serde_json_canonicalizer::to_vec(&value).expect("claim identity is serializable");
    let prefix = if reason == ClaimReason::Scheduled {
        "scheduled"
    } else {
        "manual"
    };
    format!("{prefix}-{}", hex_digest(&bytes))
}

fn hex_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().fold(
        String::with_capacity(digest.len() * 2),
        |mut output, byte| {
            write!(output, "{byte:02x}").expect("writing to String cannot fail");
            output
        },
    )
}

fn timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, ScheduleError> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map_err(|_| ScheduleError::Corrupt("invalid timestamp".to_owned()))?
        .with_timezone(&Utc);
    if timestamp(parsed) != value {
        return Err(ScheduleError::Corrupt("noncanonical timestamp".to_owned()));
    }
    Ok(parsed)
}

fn validate_optional_timestamp(value: &Option<String>) -> Result<(), ScheduleError> {
    if let Some(value) = value {
        parse_timestamp(value)?;
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CronSchedule {
    minute: Field,
    hour: Field,
    day_of_month: Field,
    month: Field,
    weekday: Field,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field {
    values: BTreeSet<u32>,
    wildcard: bool,
}

impl CronSchedule {
    pub fn parse(expression: &str) -> Result<Self, ScheduleError> {
        if !expression.is_ascii() || expression.trim() != expression {
            return Err(ScheduleError::InvalidCron);
        }
        let fields = expression.split(' ').collect::<Vec<_>>();
        if fields.len() != 5 || fields.iter().any(|field| field.is_empty()) {
            return Err(ScheduleError::InvalidCron);
        }
        let weekday = parse_field(fields[4], 0, 7)?;
        Ok(Self {
            minute: parse_field(fields[0], 0, 59)?,
            hour: parse_field(fields[1], 0, 23)?,
            day_of_month: parse_field(fields[2], 1, 31)?,
            month: parse_field(fields[3], 1, 12)?,
            weekday: Field {
                values: weekday.values.into_iter().map(|value| value % 7).collect(),
                wildcard: weekday.wildcard,
            },
        })
    }

    pub fn next_after(
        &self,
        after: DateTime<Utc>,
        time_zone: &str,
    ) -> Result<DateTime<Utc>, ScheduleError> {
        let zone: Tz = time_zone
            .parse()
            .map_err(|_| ScheduleError::InvalidTimeZone(time_zone.to_owned()))?;
        let base = after
            .with_second(0)
            .and_then(|value| value.with_nanosecond(0))
            .ok_or(ScheduleError::NoOccurrence)?;
        let mut candidate = base + Duration::minutes(1);
        for _ in 0..TEN_YEARS_MINUTES {
            let local = candidate.with_timezone(&zone);
            let day_matches = self.day_of_month.values.contains(&local.day());
            let weekday_matches = self
                .weekday
                .values
                .contains(&local.weekday().num_days_from_sunday());
            let calendar_day_matches = match (self.day_of_month.wildcard, self.weekday.wildcard) {
                (true, true) => true,
                (true, false) => weekday_matches,
                (false, true) => day_matches,
                (false, false) => day_matches || weekday_matches,
            };
            if self.minute.values.contains(&local.minute())
                && self.hour.values.contains(&local.hour())
                && self.month.values.contains(&local.month())
                && calendar_day_matches
            {
                return Ok(candidate);
            }
            candidate += Duration::minutes(1);
        }
        Err(ScheduleError::NoOccurrence)
    }
}

fn parse_field(raw: &str, lower: u32, upper: u32) -> Result<Field, ScheduleError> {
    let mut values = BTreeSet::new();
    for component in raw.split(',') {
        if component.is_empty() {
            return Err(ScheduleError::InvalidCron);
        }
        let mut pieces = component.split('/');
        let base = pieces.next().ok_or(ScheduleError::InvalidCron)?;
        let step = pieces
            .next()
            .map(|value| value.parse::<u32>().ok())
            .unwrap_or(Some(1))
            .filter(|value| *value > 0)
            .ok_or(ScheduleError::InvalidCron)?;
        if pieces.next().is_some() {
            return Err(ScheduleError::InvalidCron);
        }
        let (start, end) = if base == "*" {
            (lower, upper)
        } else if let Some((start, end)) = base.split_once('-') {
            (
                start
                    .parse::<u32>()
                    .map_err(|_| ScheduleError::InvalidCron)?,
                end.parse::<u32>().map_err(|_| ScheduleError::InvalidCron)?,
            )
        } else {
            let start = base
                .parse::<u32>()
                .map_err(|_| ScheduleError::InvalidCron)?;
            (
                start,
                if component.contains('/') {
                    upper
                } else {
                    start
                },
            )
        };
        if start < lower || end > upper || start > end {
            return Err(ScheduleError::InvalidCron);
        }
        let mut value = start;
        while value <= end {
            values.insert(value);
            let Some(next) = value.checked_add(step) else {
                break;
            };
            if next <= value {
                break;
            }
            value = next;
        }
    }
    if values.is_empty() {
        return Err(ScheduleError::InvalidCron);
    }
    let full_domain = values.len() == usize::try_from(upper - lower + 1).expect("small cron field");
    Ok(Field {
        values,
        // DOM/DOW matching is based on whether a field restricts the
        // domain, not on which equivalent spelling produced the full set.
        wildcard: full_domain,
    })
}

#[derive(Debug, Error)]
pub enum ScheduleError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid {0}")]
    Invalid(&'static str),
    #[error("invalid cron expression")]
    InvalidCron,
    #[error("unknown time zone {0}")]
    InvalidTimeZone(String),
    #[error("cron has no occurrence in ten years")]
    NoOccurrence,
    #[error("scheduled task not found: {0}")]
    NotFound(String),
    #[error("scheduled task has an active run: {0}")]
    Active(String),
    #[error("origin tuple was reused for different work")]
    OriginCollision,
    #[error("schedule claim does not match active run")]
    ClaimMismatch,
    #[error("schedule ledger corruption: {0}")]
    Corrupt(String),
}
