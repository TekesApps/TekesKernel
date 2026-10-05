//! session-controls: client extension authority for the Swift contract
//! families `SessionGoalControlEndpoint` / `SessionGoalActivationEndpoint`
//! (`goals.*`) and `SessionSubagentControlEndpoint` (`subagents.list`).
//!
//! # Goals
//!
//! Kernel keeps one durable per-session goal record at
//! `root/goals/sessions/<sessionId>.json` (canonical JSON + LF, atomically
//! replaced). The record is the client-authored view: `objective`, `phase`,
//! and a CAS `revision`. Its `id` is the session's host-bound goal id
//! (`LaunchBindings.goal_id`) when one exists, else a fresh UUID. Production
//! `ProcessHost` binds the ID of an active, blocked, or paused record at the
//! next worker launch. Model-authored `goals/log.jsonl` records (written by
//! the `new_goal` / `set_goal_state` workflow tools) contribute when their
//! goal ID matches that binding; then the log
//! records for that goal id are folded into the view on every read:
//! each `new_goal` counts as one round started, and the latest
//! `set_goal_state.state` maps to `phase` (`blocked` carrying its `reason`
//! as `blockedReason`) unless a later client mutation supersedes it.
//!
//! `activation` is derived, not stored: Kernel has no separate activation
//! state, so it is `"active"` exactly when `phase == "active"` and
//! `"inactive"` otherwise. `maxGoalRounds` bounds automatic goal turns.
//!
//! # Subagents
//!
//! Kernel `task`/`subagent` children are bounded delegations: child lines
//! inside the parent session (`state{subkind:"delegation"}` -> `spawn` ->
//! `child_result`). They are not user-continuable and have no parent-owned
//! prompt/cancel control; `subagents.list` only reads the parent's semantic
//! ledger. The function takes the ledger bytes so the crate stays pure.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

pub const METHODS: &[&str] = &[
    "goals.get",
    "goals.edit",
    "goals.clear",
    "goals.pause",
    "goals.resume",
    "subagents.list",
];

const RECORD_FORMAT: u64 = 1;
/// Maximum model turns admitted for one active goal, including its first turn.
pub const MAX_GOAL_ROUNDS: u64 = 16;

pub const PHASE_ACTIVE: &str = "active";
pub const PHASE_PAUSED: &str = "paused";
pub const PHASE_BLOCKED: &str = "blocked";
pub const PHASE_COMPLETE: &str = "complete";
const PHASES: &[&str] = &[PHASE_ACTIVE, PHASE_PAUSED, PHASE_BLOCKED, PHASE_COMPLETE];

#[derive(Debug, thiserror::Error)]
pub enum Failure {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("goal revision is stale; current is {current}")]
    GoalStale { current: u64 },
    #[error("no goal record for session {0}")]
    GoalNotFound(String),
    #[error("goal transition {from} -> {to} is invalid")]
    GoalInvalidTransition { from: String, to: String },
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("ledger corrupt: {0}")]
    LedgerCorrupt(String),
    #[error("io failure: {0}")]
    Io(String),
}

impl Failure {
    /// Stable public error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad-request",
            Self::GoalStale { .. } => "goal-stale",
            Self::GoalNotFound(_) => "goal-not-found",
            Self::GoalInvalidTransition { .. } => "goal-invalid-transition",
            Self::SessionNotFound(_) => "session-not-found",
            Self::LedgerCorrupt(_) => "ledger-corrupt",
            Self::Io(_) => "internal",
        }
    }

    /// I-JSON details carried next to the code.
    #[must_use]
    pub fn details(&self) -> Value {
        match self {
            Self::GoalStale { current } => json!({"current": current}),
            Self::GoalInvalidTransition { from, to } => json!({"from": from, "to": to}),
            _ => json!({}),
        }
    }
}

impl From<std::io::Error> for Failure {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

/// Durable per-session goal record (`root/goals/sessions/<sessionId>.json`).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct GoalRecord {
    pub format: u64,
    pub id: String,
    pub revision: u64,
    pub objective: String,
    pub phase: String,
    #[serde(rename = "maxGoalRounds")]
    pub max_goal_rounds: u64,
    #[serde(rename = "roundsStarted")]
    pub rounds_started: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "blockedReason", skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
}

/// `activation` is derived from `phase`; Kernel stores no activation state.
#[must_use]
pub fn activation_of(phase: &str) -> &'static str {
    if phase == PHASE_ACTIVE {
        "active"
    } else {
        "inactive"
    }
}

/// Pure `{id, revision, activation}` projection of a record
/// (`SessionGoalActivationValue`).
#[must_use]
pub fn activation_value(record: &GoalRecord) -> Value {
    json!({
        "id": record.id,
        "revision": record.revision,
        "activation": activation_of(&record.phase),
    })
}

/// RFC 3339 record timestamp -> Unix epoch milliseconds (the Swift
/// `SessionGoalView.createdAt/updatedAt` are `Double` milliseconds).
fn epoch_millis(timestamp: &str) -> f64 {
    chrono::DateTime::parse_from_rfc3339(timestamp)
        .map_or(0.0, |parsed| parsed.timestamp_millis() as f64)
}

/// `GoalView` wire shape. The durable record keeps RFC 3339 strings; the
/// view carries epoch milliseconds to match the Swift contract.
#[must_use]
pub fn goal_view(record: &GoalRecord) -> Value {
    let mut view = json!({
        "id": record.id,
        "revision": record.revision,
        "objective": record.objective,
        "phase": record.phase,
        "maxGoalRounds": record.max_goal_rounds,
        "roundsStarted": record.rounds_started,
        "createdAt": epoch_millis(&record.created_at),
        "updatedAt": epoch_millis(&record.updated_at),
        "activation": activation_of(&record.phase),
    });
    if let Some(reason) = &record.blocked_reason {
        view["blockedReason"] = Value::String(reason.clone());
    }
    view
}

/// Closed-shape validation of one method payload.
pub fn validate(method: &str, payload: &Value) -> Result<(), String> {
    let object = payload
        .as_object()
        .ok_or_else(|| "payload must be an object".to_owned())?;
    let allowed: &[&str] = match method {
        "goals.get" => &["sessionId"],
        "goals.edit" => &["sessionId", "ref", "objective"],
        "goals.clear" | "goals.pause" | "goals.resume" => &["sessionId", "ref"],
        "subagents.list" => &["parentSessionId"],
        _ => return Err(format!("unknown method {method}")),
    };
    for key in object.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unexpected field {key}"));
        }
    }
    let session_key = if method == "subagents.list" {
        "parentSessionId"
    } else {
        "sessionId"
    };
    require_id(object, session_key)?;
    if allowed.contains(&"ref") {
        let reference = object
            .get("ref")
            .and_then(Value::as_object)
            .ok_or_else(|| "ref must be an object".to_owned())?;
        if reference.len() != 2 {
            return Err("ref must be exactly {id, revision}".to_owned());
        }
        require_id(reference, "id")?;
        if !reference.get("revision").is_some_and(Value::is_u64) {
            return Err("ref.revision must be a non-negative integer".to_owned());
        }
    }
    if method == "goals.edit" {
        let objective = object
            .get("objective")
            .and_then(Value::as_str)
            .ok_or_else(|| "objective must be a string".to_owned())?;
        if objective.trim().is_empty() || objective.len() > 16_384 {
            return Err("objective must be non-empty and at most 16384 bytes".to_owned());
        }
    }
    Ok(())
}

fn require_id(object: &Map<String, Value>, key: &str) -> Result<(), String> {
    let value = object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{key} must be a string"))?;
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(format!("{key} must be a non-empty identifier"));
    }
    Ok(())
}

/// Execute one `goals.*` method against the state root. `bound_goal_id` is
/// the session's host-bound goal id when one exists.
pub fn execute_goal(
    root: &Path,
    method: &str,
    payload: &Value,
    bound_goal_id: Option<&str>,
) -> Result<Value, Failure> {
    validate(method, payload).map_err(Failure::BadRequest)?;
    let session_id = payload["sessionId"].as_str().expect("validated");
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let store = GoalStore { root, session_id };
    match method {
        "goals.get" => Ok(store
            .read(bound_goal_id)?
            .as_ref()
            .map_or(Value::Null, goal_view)),
        "goals.edit" => {
            let objective = payload["objective"].as_str().expect("validated");
            let reference = reference_of(payload);
            let record = match store.read(bound_goal_id)? {
                None if reference.1 == 0 => GoalRecord {
                    format: RECORD_FORMAT,
                    id: bound_goal_id
                        .map_or_else(|| uuid::Uuid::new_v4().to_string(), str::to_owned),
                    revision: 1,
                    objective: objective.to_owned(),
                    phase: PHASE_ACTIVE.to_owned(),
                    max_goal_rounds: MAX_GOAL_ROUNDS,
                    rounds_started: 0,
                    created_at: now.clone(),
                    updated_at: now,
                    blocked_reason: None,
                },
                None => return Err(Failure::GoalNotFound(session_id.to_owned())),
                Some(mut current) => {
                    check_reference(&current, reference)?;
                    current.objective = objective.to_owned();
                    current.revision += 1;
                    current.updated_at = now;
                    current
                }
            };
            store.write(&record)?;
            Ok(goal_view(&record))
        }
        "goals.clear" => {
            let current = store
                .read(bound_goal_id)?
                .ok_or_else(|| Failure::GoalNotFound(session_id.to_owned()))?;
            check_reference(&current, reference_of(payload))?;
            store.remove()?;
            Ok(json!({"id": current.id, "revision": current.revision}))
        }
        "goals.pause" | "goals.resume" => {
            let mut current = store
                .read(bound_goal_id)?
                .ok_or_else(|| Failure::GoalNotFound(session_id.to_owned()))?;
            check_reference(&current, reference_of(payload))?;
            let to = if method == "goals.pause" {
                PHASE_PAUSED
            } else {
                PHASE_ACTIVE
            };
            let allowed = matches!(
                (current.phase.as_str(), to),
                (PHASE_ACTIVE, PHASE_PAUSED) | (PHASE_PAUSED | PHASE_BLOCKED, PHASE_ACTIVE)
            );
            if !allowed {
                return Err(Failure::GoalInvalidTransition {
                    from: current.phase,
                    to: to.to_owned(),
                });
            }
            current.phase = to.to_owned();
            current.blocked_reason = None;
            current.revision += 1;
            current.updated_at = now;
            store.write(&current)?;
            Ok(goal_view(&current))
        }
        _ => Err(Failure::BadRequest(format!(
            "{method} is not a goals method"
        ))),
    }
}

fn reference_of(payload: &Value) -> (&str, u64) {
    (
        payload["ref"]["id"].as_str().expect("validated"),
        payload["ref"]["revision"].as_u64().expect("validated"),
    )
}

fn check_reference(current: &GoalRecord, reference: (&str, u64)) -> Result<(), Failure> {
    if current.id != reference.0 || current.revision != reference.1 {
        return Err(Failure::GoalStale {
            current: current.revision,
        });
    }
    Ok(())
}

/// The goal id a worker launched for `session_id` binds as its host goal:
/// the session's durable record while it is `active`, `blocked`, or `paused`, so the
/// model-side `new_goal`/`set_goal_state` tools enter the catalog and act on
/// this record. A complete goal (or no record) binds nothing.
/// Launch bindings are immutable, so a goal edited mid-session binds at the
/// session's next launch.
pub fn bound_goal_id(root: &Path, session_id: &str) -> Result<Option<String>, Failure> {
    let store = GoalStore { root, session_id };
    Ok(store
        .read(None)?
        .filter(|record| {
            record.phase == PHASE_ACTIVE
                || record.phase == PHASE_BLOCKED
                || record.phase == PHASE_PAUSED
        })
        .map(|record| record.id))
}

/// Read the authoritative goal text and current model-authored phase. Callers
/// must compare the id with their immutable launch binding before using it.
pub fn read_goal(root: &Path, session_id: &str) -> Result<Option<GoalRecord>, Failure> {
    GoalStore { root, session_id }.read(None)
}

struct GoalStore<'a> {
    root: &'a Path,
    session_id: &'a str,
}

impl GoalStore<'_> {
    fn path(&self) -> PathBuf {
        self.root
            .join("goals")
            .join("sessions")
            .join(format!("{}.json", self.session_id))
    }

    /// The stored record with the model-authored `goals/log.jsonl` folded
    /// in. The fold is read-only: the durable file keeps the client view.
    fn read(&self, bound_goal_id: Option<&str>) -> Result<Option<GoalRecord>, Failure> {
        let bytes = match fs::read(self.path()) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let mut record: GoalRecord = serde_json::from_slice(&bytes)
            .map_err(|error| Failure::Io(format!("goal record unreadable: {error}")))?;
        if record.format != RECORD_FORMAT {
            return Err(Failure::Io(format!(
                "goal record format {} is unsupported",
                record.format
            )));
        }
        // V1 reported 1 as a placeholder before automatic goal turns existed.
        if record.max_goal_rounds == 1 {
            record.max_goal_rounds = MAX_GOAL_ROUNDS;
        }
        // The log is keyed by goal id, so folding on an id match is always
        // sound; a caller that knows the bound id may still pass it as a
        // guard, but the endpoint's `goals.get` has no launch context and
        // must see the model's `set_goal_state`/`new_goal` progress too.
        if bound_goal_id.is_none_or(|bound| bound == record.id) {
            fold_goal_log(&mut record, &self.root.join("goals").join("log.jsonl"))?;
        }
        fold_goal_rounds(
            &mut record,
            &self
                .root
                .join("threads")
                .join(self.session_id)
                .join("main.jsonl"),
        )?;
        Ok(Some(record))
    }

    fn write(&self, record: &GoalRecord) -> Result<(), Failure> {
        let path = self.path();
        let parent = path.parent().expect("record has a parent");
        fs::create_dir_all(parent)?;
        let mut bytes = serde_json_canonicalizer::to_vec(record)
            .map_err(|error| Failure::Io(error.to_string()))?;
        bytes.push(b'\n');
        let temporary = parent.join(format!(".{}.{}.tmp", self.session_id, uuid::Uuid::new_v4()));
        {
            let mut file = fs::File::create(&temporary)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        fs::rename(&temporary, &path)?;
        Ok(())
    }

    fn remove(&self) -> Result<(), Failure> {
        match fs::remove_file(self.path()) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
}

/// Fold the workflow-tool records for `record.id` into the view: every
/// `new_goal` is a round started; the latest `set_goal_state` after the
/// client's `updatedAt` decides `phase` (and `blockedReason`).
fn fold_goal_log(record: &mut GoalRecord, log: &Path) -> Result<(), Failure> {
    let bytes = match fs::read(log) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    let mut rounds = 0_u64;
    let mut latest_state: Option<(String, String, Option<String>)> = None;
    for line in bytes.split(|byte| *byte == b'\n') {
        if line.is_empty() {
            continue;
        }
        let Ok(entry) = serde_json::from_slice::<Value>(line) else {
            continue;
        };
        if entry["goal_id"].as_str() != Some(record.id.as_str()) {
            continue;
        }
        match entry["op"].as_str() {
            Some("new_goal") => rounds += 1,
            Some("set_goal_state") => {
                if let Some(state) = entry["state"].as_str() {
                    let ts = entry["ts"].as_str().unwrap_or_default().to_owned();
                    let reason = entry["reason"].as_str().map(str::to_owned);
                    latest_state = Some((ts, state.to_owned(), reason));
                }
            }
            _ => {}
        }
    }
    record.rounds_started = record.rounds_started.max(rounds);
    if let Some((ts, state, reason)) = latest_state {
        // RFC 3339 UTC timestamps with fixed precision compare lexically.
        if PHASES.contains(&state.as_str()) && ts.as_str() > record.updated_at.as_str() {
            record.blocked_reason = if state == PHASE_BLOCKED { reason } else { None };
            record.phase = state;
        }
    }
    Ok(())
}

/// The thread ledger, rather than a repeated goal message, accounts for the
/// turns spent on a goal. A replacement goal starts a fresh count at createdAt.
fn fold_goal_rounds(record: &mut GoalRecord, ledger: &Path) -> Result<(), Failure> {
    let bytes = match fs::read(ledger) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    let rounds = bytes
        .split(|byte| *byte == b'\n')
        .filter_map(|line| serde_json::from_slice::<Value>(line).ok())
        .filter(|event| {
            event["kind"] == "turn_open"
                && event["ts"]
                    .as_str()
                    .is_some_and(|ts| ts >= record.created_at.as_str())
        })
        .count() as u64;
    record.rounds_started = record.rounds_started.max(rounds);
    Ok(())
}

/// `subagents.list`: the parent session's `SessionSubagentCatalog` derived
/// from its semantic ledger bytes (`sessions/<id>/main.jsonl`). `archived`
/// is the caller's archive fact for the session.
pub fn subagent_catalog(
    ledger_bytes: &[u8],
    session_id: &str,
    archived: bool,
) -> Result<Value, Failure> {
    let projection = schema::validate_ledger(ledger_bytes, 1)
        .map_err(|error| Failure::LedgerCorrupt(error.to_string()))?;
    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    let mut children: Vec<(String, String)> = Vec::new();
    let mut settled: Vec<String> = Vec::new();
    for event in &projection.events {
        let raw: Value = serde_json::to_value(event.raw())
            .map_err(|error| Failure::LedgerCorrupt(error.to_string()))?;
        match event.kind() {
            schema::EventKind::State if event.string_field("subkind") == Some("delegation") => {
                if let (Some(call), Some(name)) = (
                    raw["payload"]["call"].as_str(),
                    raw["payload"]["name"].as_str(),
                ) {
                    labels.insert(call.to_owned(), name.to_owned());
                }
            }
            schema::EventKind::Spawn => {
                let child = event
                    .string_field("child")
                    .ok_or_else(|| Failure::LedgerCorrupt("spawn lacks child".to_owned()))?;
                let call = event
                    .string_field("call")
                    .ok_or_else(|| Failure::LedgerCorrupt("spawn lacks call".to_owned()))?;
                children.push((child_line_id(child), call.to_owned()));
            }
            schema::EventKind::ChildResult => {
                if let Some(child) = event.string_field("child") {
                    settled.push(child_line_id(child));
                }
            }
            _ => {}
        }
    }
    let mut entries = vec![json!({
        "kind": "parent",
        "id": session_id,
        "activity": if archived { "settled" } else { "running" },
        "hasChildren": !children.is_empty(),
    })];
    for (child, call) in children {
        let mut entry = json!({
            "kind": "child",
            "id": child,
            "mode": "bounded",
            "activity": if settled.contains(&child) { "settled" } else { "running" },
            "hasChildren": false,
        });
        if let Some(label) = labels.get(&call) {
            entry["label"] = Value::String(label.clone());
        }
        entries.push(entry);
    }
    Ok(json!({"entries": entries, "parentAvailable": !archived}))
}

/// Spawn records name the child ledger file (`<line>.jsonl`); the public id
/// is the line id.
fn child_line_id(child: &str) -> String {
    child.strip_suffix(".jsonl").unwrap_or(child).to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn complete_state_and_turn_count_survive_a_history_summary() {
        let root = tempfile::tempdir().unwrap();
        let sid = "018f0000-0000-7000-8000-0000000000c4";
        let created = super::execute_goal(root.path(), "goals.edit",
            &serde_json::json!({"sessionId":sid,"ref":{"id":"new","revision":0},"objective":"full original objective"}), None).unwrap();
        let id = created["id"].as_str().unwrap();
        let folder = root.path().join("threads").join(sid);
        std::fs::create_dir_all(&folder).unwrap();
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        std::fs::write(folder.join("main.jsonl"), format!("{{\"kind\":\"turn_open\",\"ts\":\"{now}\",\"trigger\":{{\"inputs\":[2]}}}}\n{{\"kind\":\"compact\",\"summary\":\"history only\"}}\n")).unwrap();
        let later = (chrono::Utc::now() + chrono::Duration::seconds(2))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        std::fs::write(root.path().join("goals/log.jsonl"), format!("{{\"op\":\"set_goal_state\",\"goal_id\":\"{id}\",\"state\":\"complete\",\"ts\":\"{later}\"}}\n")).unwrap();
        let view = super::execute_goal(
            root.path(),
            "goals.get",
            &serde_json::json!({"sessionId":sid}),
            None,
        )
        .unwrap();
        assert_eq!(view["objective"], "full original objective");
        assert_eq!(view["phase"], "complete");
        assert_eq!(view["roundsStarted"], 1);
    }
    #[test]
    fn goals_get_folds_the_model_log_without_a_bound_id() {
        let root = tempfile::tempdir().unwrap();
        let sid = "018f0000-0000-7000-8000-0000000000b2";
        let created = super::execute_goal(
            root.path(),
            "goals.edit",
            &serde_json::json!({"sessionId": sid, "ref": {"id": "new", "revision": 0}, "objective": "ship"}),
            None,
        )
        .unwrap();
        let id = created["id"].as_str().unwrap();
        std::fs::write(
            root.path().join("goals").join("log.jsonl"),
            format!(
                "{{\"format\":1,\"op\":\"new_goal\",\"goal_id\":\"{id}\",\"thread\":\"{sid}\",\"turn\":1,\"call\":\"c1\",\"ts\":\"2099-01-01T00:00:01.000Z\",\"goal\":\"g\",\"completion_criteria\":\"d\",\"reason\":\"r\"}}\n{{\"format\":1,\"op\":\"set_goal_state\",\"goal_id\":\"{id}\",\"thread\":\"{sid}\",\"turn\":2,\"call\":\"c2\",\"ts\":\"2099-01-01T00:00:02.000Z\",\"state\":\"blocked\",\"progress\":\"1\",\"reason\":\"waiting\",\"user_action\":null}}\n"
            ),
        )
        .unwrap();
        let view = super::execute_goal(
            root.path(),
            "goals.get",
            &serde_json::json!({"sessionId": sid}),
            None,
        )
        .unwrap();
        assert_eq!(view["roundsStarted"], 1);
        assert_eq!(view["phase"], "blocked");
        assert_eq!(view["blockedReason"], "waiting");
    }

    #[test]
    fn bound_goal_id_follows_the_record_phase() {
        let root = tempfile::tempdir().unwrap();
        let sid = "018f0000-0000-7000-8000-0000000000b1";
        assert_eq!(super::bound_goal_id(root.path(), sid).unwrap(), None);
        let created = super::execute_goal(
            root.path(),
            "goals.edit",
            &serde_json::json!({"sessionId": sid, "ref": {"id": "new", "revision": 0}, "objective": "ship"}),
            None,
        )
        .unwrap();
        let id = created["id"].as_str().unwrap().to_owned();
        assert_eq!(
            super::bound_goal_id(root.path(), sid).unwrap().as_deref(),
            Some(id.as_str())
        );
        super::execute_goal(
            root.path(),
            "goals.pause",
            &serde_json::json!({"sessionId": sid, "ref": {"id": id, "revision": 1}}),
            None,
        )
        .unwrap();
        assert_eq!(
            super::bound_goal_id(root.path(), sid).unwrap().as_deref(),
            Some(id.as_str())
        );
    }

    use super::*;

    fn session() -> &'static str {
        "018f0000-0000-7000-8000-00000000c0de"
    }

    fn edit(
        root: &Path,
        revision: u64,
        objective: &str,
        bound: Option<&str>,
    ) -> Result<Value, Failure> {
        execute_goal(
            root,
            "goals.edit",
            &json!({"sessionId": session(), "ref": {"id": "goal-x", "revision": revision}, "objective": objective}),
            bound,
        )
    }

    fn with_ref(view: &Value) -> Value {
        json!({"sessionId": session(), "ref": {"id": view["id"], "revision": view["revision"]}})
    }

    #[test]
    fn validate_closes_shapes() {
        assert!(validate("goals.get", &json!({"sessionId": session()})).is_ok());
        assert!(validate("goals.get", &json!({"sessionId": session(), "extra": 1})).is_err());
        assert!(validate("goals.get", &json!({"sessionId": ""})).is_err());
        assert!(validate("goals.edit", &json!({"sessionId": session(), "ref": {"id": "g", "revision": 0}, "objective": "ship"})).is_ok());
        assert!(validate("goals.edit", &json!({"sessionId": session(), "ref": {"id": "g", "revision": -1}, "objective": "ship"})).is_err());
        assert!(validate("goals.edit", &json!({"sessionId": session(), "ref": {"id": "g", "revision": 0}, "objective": "  "})).is_err());
        assert!(
            validate(
                "goals.clear",
                &json!({"sessionId": session(), "ref": {"id": "g", "revision": 1, "x": 1}})
            )
            .is_err()
        );
        assert!(validate("subagents.list", &json!({"parentSessionId": session()})).is_ok());
        assert!(validate("subagents.list", &json!({"sessionId": session()})).is_err());
        assert!(validate("nope", &json!({})).is_err());
    }

    #[test]
    fn goal_lifecycle_with_cas() {
        let root = tempfile::tempdir().unwrap();
        let get = || {
            execute_goal(
                root.path(),
                "goals.get",
                &json!({"sessionId": session()}),
                None,
            )
            .unwrap()
        };
        assert_eq!(get(), Value::Null);
        assert!(matches!(
            edit(root.path(), 1, "ship", None),
            Err(Failure::GoalNotFound(_))
        ));
        let created = edit(root.path(), 0, "ship v1", Some("goal-x")).unwrap();
        assert_eq!(created["id"], "goal-x");
        assert_eq!(created["revision"], 1);
        assert_eq!(created["phase"], "active");
        assert_eq!(created["activation"], "active");
        assert_eq!(created["maxGoalRounds"], MAX_GOAL_ROUNDS);
        assert_eq!(created["roundsStarted"], 0);
        assert!(created.get("blockedReason").is_none());
        let stored = fs::read(
            root.path()
                .join("goals/sessions")
                .join(format!("{}.json", session())),
        )
        .unwrap();
        assert!(stored.ends_with(b"\n"));
        assert_eq!(get(), created);
        assert!(
            created["createdAt"].as_f64().is_some_and(|ms| ms > 1.7e12),
            "epoch milliseconds"
        );
        assert_eq!(created["createdAt"], created["updatedAt"]);

        // Stale revision fails closed with the current revision.
        let stale = edit(root.path(), 0, "again", Some("goal-x")).unwrap_err();
        assert!(matches!(stale, Failure::GoalStale { current: 1 }));
        assert_eq!(stale.code(), "goal-stale");
        assert_eq!(stale.details(), json!({"current": 1}));

        let edited = edit(root.path(), 1, "ship v2", Some("goal-x")).unwrap();
        assert_eq!(edited["revision"], 2);
        assert_eq!(edited["objective"], "ship v2");
        assert_eq!(edited["createdAt"], created["createdAt"]);

        // active -> paused -> active; resume from active is invalid.
        let paused = execute_goal(root.path(), "goals.pause", &with_ref(&edited), None).unwrap();
        assert_eq!(paused["phase"], "paused");
        assert_eq!(paused["activation"], "inactive");
        assert_eq!(paused["revision"], 3);
        let again = execute_goal(root.path(), "goals.pause", &with_ref(&paused), None).unwrap_err();
        assert_eq!(again.code(), "goal-invalid-transition");
        assert_eq!(again.details(), json!({"from": "paused", "to": "paused"}));
        let resumed = execute_goal(root.path(), "goals.resume", &with_ref(&paused), None).unwrap();
        assert_eq!(resumed["phase"], "active");
        assert_eq!(resumed["revision"], 4);
        let record: GoalRecord = serde_json::from_value(json!({
            "format": 1, "id": "goal-x", "revision": 4, "objective": "o", "phase": "active",
            "maxGoalRounds": 1, "roundsStarted": 0, "createdAt": "t", "updatedAt": "t"
        }))
        .unwrap();
        assert_eq!(
            activation_value(&record),
            json!({"id": "goal-x", "revision": 4, "activation": "active"})
        );

        let cleared = execute_goal(root.path(), "goals.clear", &with_ref(&resumed), None).unwrap();
        assert_eq!(cleared, json!({"id": "goal-x", "revision": 4}));
        assert_eq!(get(), Value::Null);
        assert!(matches!(
            execute_goal(root.path(), "goals.clear", &with_ref(&resumed), None),
            Err(Failure::GoalNotFound(_))
        ));
    }

    #[test]
    fn unbound_goal_gets_uuid_and_bound_goal_folds_log() {
        let root = tempfile::tempdir().unwrap();
        let unbound = edit(root.path(), 0, "free", None).unwrap();
        assert!(uuid::Uuid::parse_str(unbound["id"].as_str().unwrap()).is_ok());
        execute_goal(root.path(), "goals.clear", &with_ref(&unbound), None).unwrap();

        let created = edit(root.path(), 0, "bound", Some("goal-x")).unwrap();
        // Age the client record so model records authored later than the
        // client's last edit are visible in the fold.
        let record_path = root
            .path()
            .join("goals/sessions")
            .join(format!("{}.json", session()));
        let mut aged: Value = serde_json::from_slice(&fs::read(&record_path).unwrap()).unwrap();
        aged["updatedAt"] = json!("2000-01-01T00:00:00.000Z");
        fs::write(&record_path, serde_json::to_vec(&aged).unwrap()).unwrap();
        let log = root.path().join("goals/log.jsonl");
        let later = "2001-01-01T00:00:00.000Z";
        let lines = [
            json!({"format":1,"op":"new_goal","goal_id":"goal-x","thread":"m","turn":1,"call":"c1","ts":later,"goal":"g","completion_criteria":"c","reason":"r"}),
            json!({"format":1,"op":"new_goal","goal_id":"other","thread":"m","turn":1,"call":"c2","ts":later,"goal":"g","completion_criteria":"c","reason":"r"}),
            json!({"format":1,"op":"set_goal_state","goal_id":"goal-x","thread":"m","turn":2,"call":"c3","ts":later,"state":"blocked","progress":"p","reason":"needs key","user_action":null}),
        ];
        let mut bytes = Vec::new();
        for line in lines {
            bytes.extend(serde_json::to_vec(&line).unwrap());
            bytes.push(b'\n');
        }
        fs::write(&log, bytes).unwrap();
        let view = execute_goal(
            root.path(),
            "goals.get",
            &json!({"sessionId": session()}),
            Some("goal-x"),
        )
        .unwrap();
        assert_eq!(view["roundsStarted"], 1);
        assert_eq!(view["phase"], "blocked");
        assert_eq!(view["blockedReason"], "needs key");
        assert_eq!(view["activation"], "inactive");
        assert_eq!(view["revision"], created["revision"], "fold is read-only");
        // Resume from the folded blocked phase is a later client mutation, so it supersedes the model state.
        let resumed = execute_goal(
            root.path(),
            "goals.resume",
            &with_ref(&view),
            Some("goal-x"),
        )
        .unwrap();
        assert_eq!(resumed["phase"], "active");
        assert!(resumed.get("blockedReason").is_none());
        let reread = execute_goal(
            root.path(),
            "goals.get",
            &json!({"sessionId": session()}),
            Some("goal-x"),
        )
        .unwrap();
        assert_eq!(reread["phase"], "active");
        assert_eq!(reread["roundsStarted"], 1);
        // Without the binding the log is ignored.
        let plain = execute_goal(
            root.path(),
            "goals.get",
            &json!({"sessionId": session()}),
            None,
        )
        .unwrap();
        assert_eq!(plain["phase"], "active");
    }

    fn ledger(events: &[Value]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (index, event) in events.iter().enumerate() {
            let mut event = event.clone();
            event["v"] = json!(1);
            event["seq"] = json!(index as u64 + 1);
            event["ts"] = json!("2026-09-13T00:00:00.000Z");
            bytes.extend(serde_json_canonicalizer::to_vec(&event).unwrap());
            bytes.push(b'\n');
        }
        bytes
    }

    fn genesis() -> Value {
        json!({"kind":"genesis","format":1,"min_reader":1,"min_writer":1,"thread":session(),"workspace":"ws","origin_key":"genesis","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"create","key":"genesis"},"resume":"never","config":{"digest":"d"}})
    }

    #[test]
    fn subagent_catalog_reads_spawns_and_child_results() {
        let base = ledger(&[genesis()]);
        let only_parent = subagent_catalog(&base, session(), false).unwrap();
        assert_eq!(only_parent["parentAvailable"], true);
        assert_eq!(only_parent["entries"].as_array().unwrap().len(), 1);
        assert_eq!(only_parent["entries"][0]["kind"], "parent");
        assert_eq!(only_parent["entries"][0]["hasChildren"], false);

        let full = ledger(&[
            genesis(),
            json!({"kind":"input","content":[{"type":"text","text":"go"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"input","key":"i1"}}),
            json!({"kind":"turn_open","turn":1,"trigger":{"inputs":[2]}}),
            json!({"kind":"state","turn":1,"subkind":"delegation","payload":{"call":"t1","name":"research","invocation":{},"resolved_inputs":{}}}),
            json!({"kind":"spawn","turn":1,"child":"child-a.jsonl","call":"t1","spawn_id":"s1","resume":{"bounded":3},"seed":{"kinds":["state"]}}),
            json!({"kind":"state","turn":1,"subkind":"delegation","payload":{"call":"t2","name":"write","invocation":{},"resolved_inputs":{}}}),
            json!({"kind":"spawn","turn":1,"child":"child-b.jsonl","call":"t2","spawn_id":"s2","resume":{"bounded":3},"seed":{"kinds":["state"]}}),
            json!({"kind":"child_result","turn":1,"child":"child-a.jsonl","call":"t1","spawn_id":"s1","outcome":"completed","summary":"done"}),
        ]);
        let catalog = subagent_catalog(&full, session(), false).unwrap();
        let entries = catalog["entries"].as_array().unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0]["hasChildren"], true);
        assert_eq!(
            entries[1],
            json!({"kind":"child","id":"child-a","mode":"bounded","activity":"settled","hasChildren":false,"label":"research"})
        );
        assert_eq!(
            entries[2],
            json!({"kind":"child","id":"child-b","mode":"bounded","activity":"running","hasChildren":false,"label":"write"})
        );
        let archived = subagent_catalog(&full, session(), true).unwrap();
        assert_eq!(archived["parentAvailable"], false);
        assert_eq!(archived["entries"][0]["activity"], "settled");

        let corrupt = subagent_catalog(b"{not json\n", session(), false).unwrap_err();
        assert_eq!(corrupt.code(), "ledger-corrupt");
    }
}
