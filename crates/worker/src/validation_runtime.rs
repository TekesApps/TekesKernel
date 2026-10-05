//! Production bridge from provider final to writer-owned validation settlement.
use super::*;
use engine::{ValidationBinding, ValidationDecision};
use std::collections::BTreeMap;

pub(super) const JUDGE_SYSTEM: &str = "You are the independent validator for a frozen candidate. Judge every explicit user requirement against the exact candidate answer, artifact snapshot, and frozen execution_evidence supplied in your seed. Required actions such as task delegation are judged from recorded tool calls and their successful results or child joins, not from whether the final prose repeats a tool call. A call alone does not prove successful execution. Execution evidence covers only the root turn through the candidate output; do not infer unrecorded child actions or treat omitted evidence as proof that an action did not happen. A correct file does not excuse an incorrect candidate answer. If the user requires an exact final answer, compare the entire candidate text: extra prose, formatting, or a missing marker is a failure. For exact file contents, check bytes including trailing newlines; do not trim or normalize them. List concrete unmet requirements before choosing fail; only pass when all requested requirements are met. The frozen artifacts in the seed and the read-only snapshot are authoritative; the live source workspace may have changed. Your default workspace is private scratch for temporary checks. Do not repair the deliverable. Call verify as soon as the supplied evidence supports a verdict; use tools only for a concrete unresolved check. Pass the exact supplied covered_set, including [] when no artifacts are present. Report fail with concrete issues and repair guidance. If evidence is insufficient use inconclusive, never fabricate pass. Plain assistant final text does not fulfill the verify mandate.";

pub(super) fn validator_workspace_paths(
    ledger_path: &std::path::Path,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let folder = ledger_path
        .parent()
        .ok_or("validator folder missing")?
        .canonicalize()?;
    let child = ledger_path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or("validator identity missing")?;
    let root = folder.join("validator-workspaces").join(child);
    Ok((root.join("scratch"), root.join("snapshot")))
}

fn freeze_artifact_copy(
    snapshot_root: &std::path::Path,
    workspace_roots: &[String],
    path: &str,
    bytes: &[u8],
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let target = workspace_roots
        .iter()
        .enumerate()
        .find_map(|(index, root)| {
            std::path::Path::new(path)
                .strip_prefix(root)
                .ok()
                .map(|relative| {
                    snapshot_root
                        .join(format!("workspace-{index}"))
                        .join(relative)
                })
        });
    if let Some(target) = &target {
        fs::create_dir_all(target.parent().ok_or("snapshot file has no parent")?)?;
        if target.exists() {
            fs::remove_file(target)?;
        }
        fs::write(target, bytes)?;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(target, fs::Permissions::from_mode(0o400))?;
    }
    Ok(target)
}

fn value(event: &Event) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::to_value(event.raw())?)
}

/// Freeze root execution facts at the candidate boundary. Do not include
/// later repair actions, other turns, model prose, or host validator spawns.
fn execution_evidence(
    events: &[Event],
    turn: u64,
    output_seq: u64,
) -> Result<Value, Box<dyn std::error::Error>> {
    let calls: std::collections::BTreeSet<_> = events
        .iter()
        .filter(|e| {
            e.turn() == Some(turn) && e.seq() <= output_seq && e.kind() == &EventKind::ToolCall
        })
        .filter_map(|e| e.string_field("call"))
        .collect();
    let mut facts = Vec::new();
    let mut bytes = 0;
    let mut omitted = 0;
    for event in events.iter().filter(|e| {
        e.turn() == Some(turn)
            && e.seq() <= output_seq
            && matches!(
                e.kind(),
                EventKind::ToolCall
                    | EventKind::ToolResult
                    | EventKind::Spawn
                    | EventKind::ChildResult
            )
            && e.string_field("call")
                .is_some_and(|call| calls.contains(call))
    }) {
        let raw = value(event)?;
        let encoded = serde_json::to_vec(&raw)?;
        let scanned = SecretScanner::default().scan(&IJsonValue::parse(&encoded)?);
        if !matches!(scanned, SecretScan::Clean(_))
            || bytes + encoded.len() > 256 * 1024
            || facts.len() >= 512
        {
            omitted += 1;
            continue;
        }
        bytes += encoded.len();
        facts.push(raw);
    }
    Ok(
        json!({"scope":"root turn only", "turn":turn, "through_output_seq":output_seq,
        "complete":omitted == 0,"omitted_events":omitted,"events":facts}),
    )
}

pub(super) fn has_pending_validation(
    ledger: &LockedLedger,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(projection) = ledger.projection() else {
        return Ok(false);
    };
    if projection.terminal_tail
        || projection
            .events
            .first()
            .is_none_or(|e| e.has_field("parent"))
    {
        return Ok(false);
    }
    for event in &projection.events {
        if event.turn() != projection.latest_turn {
            continue;
        }
        if (event.kind() == &EventKind::State
            && event.string_field("subkind") == Some("validation.candidate"))
            || (event.kind() == &EventKind::Output && value(event)?["final_answer"] == true)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn activate_validator_profile(
    ledger: &LockedLedger,
    profile: &mut RuntimeProfile,
) -> Result<(), Box<dyn std::error::Error>> {
    if profile.validator {
        return Ok(());
    }
    let genesis = ledger
        .projection()
        .and_then(|p| p.events.first())
        .ok_or("missing genesis")?;
    let raw = value(genesis)?;
    let Some(binding) = raw.get("validator_for") else {
        return Ok(());
    };
    let parent = raw
        .pointer("/parent/file")
        .and_then(Value::as_str)
        .ok_or("validator has no parent")?;
    let parent_path = ledger
        .path()
        .parent()
        .ok_or("validator folder missing")?
        .join(parent);
    let projection = read_child_projection(&parent_path)?;
    let candidate_seq = binding
        .get("candidate_seq")
        .and_then(Value::as_u64)
        .ok_or("validator candidate reference missing")?;
    let candidate = projection
        .events
        .iter()
        .find(|event| event.seq() == candidate_seq)
        .ok_or("validator candidate missing")?;
    if candidate.kind() != &EventKind::State
        || candidate.string_field("subkind") != Some("validation.candidate")
        || value(candidate)?["payload"]["binding"] != binding["binding"]
    {
        return Err("validator lineage does not match host candidate".into());
    }
    let spawn_seq = raw
        .pointer("/parent/seq")
        .and_then(Value::as_u64)
        .ok_or("validator parent spawn reference missing")?;
    let spawn = projection
        .events
        .iter()
        .find(|event| event.seq() == spawn_seq)
        .ok_or("validator parent spawn missing")?;
    let child_file = ledger
        .path()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("validator child filename missing")?;
    if !validator_spawn_matches(&raw, &value(spawn)?, child_file) {
        return Err("validator lineage does not match host spawn".into());
    }
    let (scratch, snapshot) = validator_workspace_paths(ledger.path())?;
    if !scratch.is_dir()
        || !snapshot.is_dir()
        || scratch.canonicalize()? != scratch
        || snapshot.canonicalize()? != snapshot
    {
        return Err("validator private workspace is missing".into());
    }
    isolate_validator_profile(profile, &scratch, &snapshot);
    Ok(())
}

pub(super) fn isolate_validator_profile(
    profile: &mut RuntimeProfile,
    scratch: &std::path::Path,
    snapshot: &std::path::Path,
) {
    let scratch = scratch.to_string_lossy().into_owned();
    let snapshot = snapshot.to_string_lossy().into_owned();
    let original = std::mem::replace(
        &mut profile.config.workspace.cwd,
        vec![scratch.clone(), snapshot],
    );
    profile.config.workspace.cwd.extend(original);
    if profile.config.workspace.folder_binding.is_some() {
        profile.config.workspace.selected_cwd = Some(scratch.clone());
    }
    profile.config.workspace.policy.writable_roots = vec![scratch.clone()];
    profile.instruction.effective.policy.writable_roots = Some(vec![scratch]);
    profile.validator = true;
}

fn validator_spawn_matches(genesis: &Value, spawn: &Value, child_file: &str) -> bool {
    let binding = &genesis["validator_for"];
    let Some(candidate_seq) = binding["candidate_seq"].as_u64() else {
        return false;
    };
    let Some(spawn_id) = genesis["parent"]["spawn_id"].as_str() else {
        return false;
    };
    let Some(thread) = genesis["thread"].as_str() else {
        return false;
    };
    spawn["kind"] == "spawn"
        && spawn["seq"].as_u64().is_some_and(|seq| seq > candidate_seq)
        && spawn["seq"] == genesis["parent"]["seq"]
        && spawn["turn"] == binding["binding"]["turn"]
        && spawn["child"] == child_file
        && child_file == format!("{thread}.jsonl")
        && spawn["spawn_id"] == spawn_id
        && spawn["call"] == format!("validation-{candidate_seq}")
}

#[cfg(test)]
mod lineage_tests {
    use super::*;

    #[test]
    fn frozen_candidate_copy_is_read_only_and_detached_from_source() {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        let snapshot = root.path().join("snapshot");
        fs::create_dir(&source).unwrap();
        fs::create_dir(&snapshot).unwrap();
        let candidate = source.join("module.py");
        fs::write(&candidate, b"candidate").unwrap();
        let frozen = freeze_artifact_copy(
            &snapshot,
            &[source.to_string_lossy().into_owned()],
            candidate.to_str().unwrap(),
            b"candidate",
        )
        .unwrap()
        .unwrap();
        fs::write(&candidate, b"later revision").unwrap();
        assert_eq!(fs::read(&frozen).unwrap(), b"candidate");
        assert_eq!(fs::read(&candidate).unwrap(), b"later revision");
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&frozen).unwrap().permissions().mode() & 0o222,
            0
        );
        assert!(!frozen.starts_with(&source));
    }

    #[test]
    fn judge_execution_evidence_is_bound_to_candidate_turn_and_successful_results() {
        let call = |seq, turn, id: &str| {
            make_event(json!({"v":1,"seq":seq,
            "ts":"2026-09-04T00:00:00.000Z","kind":"tool_call","turn":turn,
            "call":id,"name":"task","attempt":"a","source":"provider","args":{"task":"required task"}})).unwrap()
        };
        let result = make_event(json!({"v":1,"seq":3,"ts":"2026-09-04T00:00:00.000Z",
            "kind":"tool_result","turn":2,"call":"required","outcome":"ok","content":[]}))
        .unwrap();
        let host_spawn = make_event(json!({"v":1,"seq":4,"ts":"2026-09-04T00:00:00.000Z",
            "kind":"spawn","turn":2,"call":"validation-7","child":"018f0000-0000-7000-8000-000000000003.jsonl",
            "spawn_id":"validator","resume":"never","seed":{"kinds":[]}})).unwrap();
        let events = vec![
            call(1, 1, "old"),
            call(2, 2, "required"),
            result,
            host_spawn,
            call(5, 2, "later-repair"),
        ];
        let evidence = execution_evidence(&events, 2, 4).unwrap();
        assert_eq!(evidence["complete"], true);
        assert_eq!(evidence["through_output_seq"], 4);
        let facts = evidence["events"].as_array().unwrap();
        assert_eq!(facts.len(), 2);
        assert_eq!(facts[0]["call"], "required");
        assert_eq!(facts[1]["outcome"], "ok");
        assert!(!evidence.to_string().contains("later-repair"));
        assert!(!evidence.to_string().contains("old"));
        assert!(!evidence.to_string().contains("validation-7"));
        let large: Vec<_> = (2..42)
            .map(|seq| {
                let mut raw = value(&call(seq, 2, &format!("large-{seq}"))).unwrap();
                raw["args"]["task"] = json!("x".repeat(8 * 1024));
                make_event(raw).unwrap()
            })
            .collect();
        let bounded = execution_evidence(&large, 2, 42).unwrap();
        assert_eq!(bounded["complete"], false);
        assert!(bounded["omitted_events"].as_u64().unwrap() > 0);
        assert!(bounded["events"].as_array().unwrap().len() < large.len());
    }

    #[test]
    fn validator_role_requires_the_exact_host_spawn_not_just_a_candidate_reference() {
        let genesis = json!({"thread":"judge", "parent":{"seq":12,"spawn_id":"host-spawn"},
            "validator_for":{"candidate_seq":10,"binding":{"turn":2}}});
        let spawn = json!({"kind":"spawn","seq":12,"turn":2,"child":"judge.jsonl",
            "spawn_id":"host-spawn","call":"validation-10"});
        assert!(validator_spawn_matches(&genesis, &spawn, "judge.jsonl"));
        for (field, forged) in [
            ("kind", json!("state")),
            ("seq", json!(9)),
            ("turn", json!(3)),
            ("child", json!("other.jsonl")),
            ("spawn_id", json!("other-spawn")),
            ("call", json!("ordinary-task")),
        ] {
            let mut altered = spawn.clone();
            altered[field] = forged;
            assert!(
                !validator_spawn_matches(&genesis, &altered, "judge.jsonl"),
                "{field}"
            );
        }
        assert!(!validator_spawn_matches(&genesis, &spawn, "other.jsonl"));
        let mut altered = genesis.clone();
        altered["thread"] = json!("other");
        assert!(!validator_spawn_matches(&altered, &spawn, "judge.jsonl"));
    }
}

pub(super) fn result_value(
    ledger: &LockedLedger,
    event: &Event,
) -> Result<Value, Box<dyn std::error::Error>> {
    result_value_at(ledger.path(), event)
}

fn result_value_at(
    path: &std::path::Path,
    event: &Event,
) -> Result<Value, Box<dyn std::error::Error>> {
    // A trim is a model-context replacement, not a new execution receipt.
    // Its head/tail text need not be JSON. Validation reads the original
    // durable receipt, including artifact identities pruned from the view.
    if event.has_field("supersedes") {
        let projection = read_child_projection(path)?;
        let original = projection
            .events
            .iter()
            .find(|candidate| {
                candidate.kind() == &EventKind::ToolResult
                    && candidate.string_field("call") == event.string_field("call")
                    && candidate.seq() < event.seq()
                    && !candidate.has_field("supersedes")
            })
            .ok_or("trimmed tool result has no original execution receipt")?;
        return result_value_at(path, original);
    }
    let raw = value(event)?;
    let content =
        materialize_json_at(path, raw.get("content").ok_or("tool result lacks content")?)?;
    let blocks = content
        .as_array()
        .ok_or("tool result content is not blocks")?;
    let mut text = String::new();
    for block in blocks {
        if block.get("type").and_then(Value::as_str) == Some("text") {
            let value = materialize_json_at(path, block.get("text").ok_or("result text missing")?)?;
            text.push_str(value.as_str().ok_or("result text is not a string")?);
        }
    }
    Ok(serde_json::from_str(&text)?)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ArtifactRevision {
    sha256: String,
    version: Option<u64>,
    source: String,
    sequence: u64,
}

#[derive(Default)]
struct ArtifactSnapshot(BTreeMap<String, ArtifactRevision>);

impl ArtifactSnapshot {
    fn observe(
        &mut self,
        path: String,
        revision: ArtifactRevision,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(previous) = self.0.get(&path) {
            match (previous.version, revision.version) {
                (Some(old), Some(new)) if new < old => return Ok(()),
                (Some(old), Some(new)) if new == old => {
                    if previous.sha256 != revision.sha256 {
                        return Err(format!(
                            "artifact {path} has conflicting content at version {new}"
                        )
                        .into());
                    }
                    return Ok(());
                }
                (Some(_), Some(_)) => {}
                _ if previous.source == revision.source => {
                    if revision.sequence <= previous.sequence {
                        return Ok(());
                    }
                }
                _ => {
                    return Err(format!(
                        "artifact {path} has cross-session history without comparable versions"
                    )
                    .into());
                }
            }
        }
        self.0.insert(path, revision);
        Ok(())
    }

    fn covered_set(self) -> BTreeMap<String, String> {
        self.0
            .into_iter()
            .map(|(path, revision)| (path, revision.sha256))
            .collect()
    }
}

fn artifact_identity(
    workspace: &std::path::Path,
    raw: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = PathBuf::from(raw);
    let path = if path.is_absolute() {
        path
    } else {
        workspace.join(path)
    };
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err("artifact identity contains parent traversal".into());
            }
            std::path::Component::CurDir => {}
            component => normalized.push(component.as_os_str()),
        }
    }
    if !normalized.is_absolute() {
        return Err("artifact workspace root is not absolute".into());
    }
    normalized
        .into_os_string()
        .into_string()
        .map_err(|_| "artifact identity is not UTF-8".into())
}

pub(super) fn snapshot(
    ledger: &LockedLedger,
    _turn: u64,
    profile: &RuntimeProfile,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let mut snapshot = ArtifactSnapshot::default();
    let workspace = profile
        .config
        .workspace
        .cwd
        .first()
        .ok_or("artifact workspace root missing")?;
    collect_artifacts(
        ledger.path(),
        ledger.projection().ok_or("missing projection")?,
        None,
        std::path::Path::new(workspace),
        &mut BTreeSet::new(),
        &mut snapshot,
    )?;
    Ok(snapshot.covered_set())
}

fn collect_artifacts(
    path: &std::path::Path,
    projection: &schema::LedgerProjection,
    turn: Option<u64>,
    workspace: &std::path::Path,
    visited: &mut BTreeSet<PathBuf>,
    snapshot: &mut ArtifactSnapshot,
) -> Result<(), Box<dyn std::error::Error>> {
    if visited.len() >= 1024 || !visited.insert(path.to_owned()) {
        return Err("artifact session graph is cyclic or exceeds the traversal bound".into());
    }
    let events = &projection.events;
    let names: BTreeMap<_, _> = events
        .iter()
        .filter(|e| e.kind() == &EventKind::ToolCall)
        .filter_map(|e| Some((e.string_field("call")?, e.string_field("name")?)))
        .collect();
    let source = events
        .first()
        .and_then(|event| event.string_field("thread"))
        .ok_or("artifact source missing")?;
    for event in events.iter().filter(|e| {
        turn.is_none_or(|turn| e.turn() == Some(turn))
            && e.kind() == &EventKind::ToolResult
            && e.string_field("outcome") == Some("ok")
            // Trims do not re-execute the tool or advance artifact versions.
            // Observe each original receipt at its original sequence only.
            && !e.has_field("supersedes")
    }) {
        let name = event
            .string_field("call")
            .and_then(|call| names.get(call))
            .copied();
        if !matches!(name, Some("apply_patch" | "edit" | "write" | "shell")) {
            continue;
        }
        let result = result_value_at(path, event)?;
        let artifacts = if matches!(name, Some("apply_patch" | "edit" | "write")) {
            vec![&result]
        } else {
            result
                .get("artifacts")
                .and_then(Value::as_array)
                .map(|a| a.iter().collect())
                .unwrap_or_default()
        };
        for artifact in artifacts {
            if let (Some(path), Some(sha)) = (
                artifact.get("path").and_then(Value::as_str),
                artifact.get("sha256").and_then(Value::as_str),
            ) {
                let path = artifact_identity(workspace, path)?;
                snapshot.observe(
                    path,
                    ArtifactRevision {
                        sha256: sha.to_owned(),
                        version: artifact.get("artifact_version").and_then(Value::as_u64),
                        source: source.to_owned(),
                        sequence: event.seq(),
                    },
                )?;
            }
        }
    }
    for spawn in events.iter().filter(|event| {
        event.kind() == &EventKind::Spawn && turn.is_none_or(|turn| event.turn() == Some(turn))
    }) {
        let child_file = spawn
            .string_field("child")
            .ok_or("artifact spawn has no child")?;
        if !matches!(
            std::path::Path::new(child_file)
                .components()
                .collect::<Vec<_>>()
                .as_slice(),
            [std::path::Component::Normal(_)]
        ) {
            return Err("artifact child is not a ledger basename".into());
        }
        let child_path = path
            .parent()
            .ok_or("artifact parent folder missing")?
            .join(child_file);
        let bytes = fs::read(&child_path)?;
        let scan = scan_valid_prefix(&bytes, 1);
        if scan.needs_repair() {
            return Err("artifact child ledger requires tail repair before validation".into());
        }
        let child = scan.projection.ok_or("artifact child projection missing")?;
        let genesis = child
            .events
            .first()
            .ok_or("artifact child genesis missing")?;
        let raw = value(genesis)?;
        if raw["parent"]["file"].as_str() != path.file_name().and_then(|name| name.to_str())
            || raw["parent"]["seq"].as_u64() != Some(spawn.seq())
            || raw["parent"]["spawn_id"].as_str() != spawn.string_field("spawn_id")
            || raw["workspace"].as_str() != events[0].string_field("workspace")
            || raw["thread"].as_str()
                != std::path::Path::new(child_file)
                    .file_stem()
                    .and_then(|name| name.to_str())
        {
            return Err("artifact child lineage does not match its durable spawn".into());
        }
        // Validator checks are not worker deliverables. Their scope is bound separately.
        if raw.get("validator_for").is_some() {
            if !validator_spawn_matches(&raw, &value(spawn)?, child_file) {
                return Err("artifact graph contains an unbound validator".into());
            }
            continue;
        }
        if !events.iter().any(|event| {
            event.kind() == &EventKind::ChildResult
                && event.string_field("child") == Some(child_file)
                && event.string_field("spawn_id") == spawn.string_field("spawn_id")
                && event.string_field("call") == spawn.string_field("call")
        }) {
            return Err(
                "artifact snapshot requires the spawned worker join to settle first".into(),
            );
        }
        collect_artifacts(&child_path, &child, None, workspace, visited, snapshot)?;
    }
    Ok(())
}

fn state(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    subkind: &str,
    payload: Value,
    model: bool,
) -> Result<u64, Box<dyn std::error::Error>> {
    let seq = ledger.next_seq();
    ledger.append(
        make_event(
            json!({"v":1,"seq":seq,"turn":turn,"kind":"state","ts":options.event_timestamp(),
        "subkind":subkind,"payload":payload,"visibility":if model {"model"} else {"runtime"}}),
        )?,
        true,
    )?;
    Ok(seq)
}

fn decision(
    ledger: &LockedLedger,
    seq: u64,
) -> Result<ValidationDecision, Box<dyn std::error::Error>> {
    let event = ledger
        .projection()
        .unwrap()
        .events
        .iter()
        .find(|e| e.seq() == seq)
        .ok_or("decision missing")?;
    Ok(serde_json::from_value(
        value(event)?["payload"]["decision"].clone(),
    )?)
}

pub(super) fn artifact_bytes(
    profile: &RuntimeProfile,
    path: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        path
    } else {
        PathBuf::from(
            profile
                .config
                .workspace
                .cwd
                .first()
                .ok_or("workspace has no root")?,
        )
        .join(path)
    };
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    const LIMIT: u64 = 16 * 1024 * 1024;
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC)
        .open(&path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > LIMIT {
        return Err("artifact is not a bounded regular file".into());
    }
    let mut bytes = Vec::new();
    file.take(LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > LIMIT {
        return Err("artifact grew beyond snapshot read limit".into());
    }
    Ok(bytes)
}

fn matches_snapshot(bytes: &[u8], expected: &str) -> bool {
    expected == format!("sha256-{:x}", Sha256::digest(bytes))
}

// Only an attempted verdict consumes the validator's verify mandate. Ordinary
// progress or inspection output is not a refused verdict.
pub(super) fn validator_mandate_exhausted(events: &[Event], turn: u64) -> bool {
    events
        .iter()
        .filter(|event| {
            event.turn() == Some(turn)
                && event.kind() == &EventKind::ToolCall
                && event.string_field("name") == Some("verify")
        })
        .count()
        >= 4
}

/// true means this execution is settled or awaiting external work; false means
/// the original session must continue (including durable repair feedback).
pub(super) fn advance(
    ledger: &mut LockedLedger,
    run: ProviderRunContext<'_>,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
) -> Result<bool, Box<dyn std::error::Error>> {
    let options = run.options;
    if cancellation.stop_requested() {
        return Ok(true);
    }
    let projection = ledger.projection().ok_or("missing validation projection")?;
    if projection.terminal_tail {
        return Ok(true);
    }
    let turn = projection.latest_turn.ok_or("missing validation turn")?;
    if run.profile.validator {
        let names: BTreeSet<_> = projection
            .events
            .iter()
            .filter(|e| {
                e.turn() == Some(turn)
                    && e.kind() == &EventKind::ToolCall
                    && e.string_field("name") == Some("verify")
            })
            .filter_map(|e| e.string_field("call").map(str::to_owned))
            .collect();
        let verify_results = projection
            .events
            .iter()
            .filter(|e| {
                e.turn() == Some(turn)
                    && e.kind() == &EventKind::ToolResult
                    && e.string_field("outcome") == Some("ok")
                    && e.string_field("call")
                        .is_some_and(|call| names.contains(call))
            })
            .cloned()
            .collect::<Vec<_>>();
        let expected: BTreeMap<String, String> = serde_json::from_value(
            value(&projection.events[0])?["validator_for"]["binding"]["snapshot"].clone(),
        )?;
        for event in &verify_results {
            let result = result_value(ledger, event)?;
            let covered = result["covered_set"]
                .as_array()
                .ok_or("verify has no coverage")?;
            let actual: BTreeMap<String, String> = covered
                .iter()
                .filter_map(|entry| {
                    Some((
                        entry["id"].as_str()?.to_owned(),
                        entry["dedup_key"].as_str()?.to_owned(),
                    ))
                })
                .collect();
            if actual == expected && actual.len() == covered.len() {
                append_settle(ledger, &options.event_timestamp(), turn, "completed", None)?;
                return Ok(true);
            }
        }
        if validator_mandate_exhausted(&projection.events, turn) {
            append_settle(
                ledger,
                &options.event_timestamp(),
                turn,
                "error",
                Some("validator_verify_attempts_exhausted"),
            )?;
            return Ok(true);
        }
        let outputs = projection
            .events
            .iter()
            .filter(|event| event.turn() == Some(turn) && event.kind() == &EventKind::Output)
            .count();
        let reminder_sent = projection.events.iter().any(|event| {
            event.turn() == Some(turn)
                && event.string_field("subkind") == Some("validation.verify_reminder")
        });
        if let Some(result) = verify_results.last() {
            let already_explained = projection.events.iter().any(|event| {
                event.string_field("subkind") == Some("validation.verify_rejected")
                    && value(event)
                        .ok()
                        .is_some_and(|v| v["payload"]["result_seq"].as_u64() == Some(result.seq()))
            });
            if !already_explained {
                state(
                    ledger,
                    options,
                    turn,
                    "validation.verify_rejected",
                    json!({"result_seq":result.seq(),
                    "instruction":"verify did not cover the exact frozen snapshot. Call verify again using the seed's complete covered_set without duplicates."}),
                    true,
                )?;
            }
        }
        if names.is_empty() && outputs >= 4 {
            append_settle(
                ledger,
                &options.event_timestamp(),
                turn,
                "error",
                Some("validator_no_verdict"),
            )?;
            return Ok(true);
        }
        if names.is_empty() && outputs >= 2 && !reminder_sent {
            state(
                ledger,
                options,
                turn,
                "validation.verify_reminder",
                json!({
                    "instruction":"Call verify now with the seed's exact covered_set. Use inconclusive if a remaining requirement lacks evidence. Do not produce a plain final answer."
                }),
                true,
            )?;
        }
        return Ok(false);
    }
    let Some(output) = projection
        .events
        .iter()
        .rev()
        .find(|e| e.turn() == Some(turn) && e.kind() == &EventKind::Output)
    else {
        return Ok(false);
    };
    if value(output)?.get("final_answer") != Some(&Value::Bool(true)) {
        return Ok(false);
    }
    if projection.events[0].has_field("parent") {
        append_settle(ledger, &options.event_timestamp(), turn, "completed", None)?;
        return Ok(true);
    }
    let output_seq = output.seq();
    // If this final was already returned to the worker as repair feedback,
    // recovery continues the original conversation instead of judging it again.
    if projection.events.iter().any(|e| {
        e.turn() == Some(turn)
            && e.string_field("subkind") == Some("validation.feedback")
            && value(e)
                .ok()
                .is_some_and(|v| v["payload"]["output_seq"].as_u64() == Some(output_seq))
    }) {
        return Ok(false);
    }
    // A new root final is the turn terminal. Only turns that already entered
    // the legacy independent-validation loop must finish that durable loop.
    let legacy_validation_active = projection.events.iter().any(|e| {
        e.turn() == Some(turn) && e.string_field("subkind") == Some("validation.candidate")
    });
    if !legacy_validation_active {
        let event = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"settle",
            "ts":options.event_timestamp(),"outcome":"completed",
            "promoted_output_seq":output_seq
        }))?;
        ledger.append_contract(event, BarrierContext::default())?;
        return Ok(true);
    }
    let thread = projection.events[0]
        .string_field("thread")
        .ok_or("thread missing")?
        .to_owned();
    let existing = projection
        .events
        .iter()
        .find(|e| {
            e.turn() == Some(turn)
                && e.string_field("subkind") == Some("validation.candidate")
                && value(e).ok().is_some_and(|v| {
                    v["payload"]["binding"]["output_seq"].as_u64() == Some(output_seq)
                })
        })
        .cloned();
    let binding = if let Some(existing) = existing {
        serde_json::from_value(value(&existing)?["payload"]["binding"].clone())?
    } else {
        ValidationBinding {
            thread: thread.clone(),
            worker: thread,
            turn,
            output_seq,
            snapshot: snapshot(ledger, turn, run.profile)?,
        }
    };
    let candidate_seq = engine::begin_validation(ledger, &options.event_timestamp(), &binding)?;
    let mut decision_seq = engine::commit_validation_decision(
        ledger,
        &options.event_timestamp(),
        candidate_seq,
        candidate_seq,
    )?;
    if matches!(
        decision(ledger, decision_seq)?,
        ValidationDecision::ForkValidator
    ) {
        let source = ledger
            .projection()
            .unwrap()
            .events
            .iter()
            .find(|e| {
                matches!(
                    e.string_field("subkind"),
                    Some("validation.verdict" | "validation.death")
                ) && value(e)
                    .ok()
                    .is_some_and(|v| v["payload"]["candidate_seq"].as_u64() == Some(candidate_seq))
            })
            .map(Event::seq);
        let source = match source {
            Some(seq) => seq,
            None => judge(
                ledger,
                run,
                candidate_seq,
                &binding,
                lines,
                stdout,
                cancellation,
            )?,
        };
        if cancellation.stop_requested() {
            return Ok(true);
        }
        decision_seq = engine::commit_validation_decision(
            ledger,
            &options.event_timestamp(),
            candidate_seq,
            source,
        )?;
    }
    match decision(ledger, decision_seq)? {
        ValidationDecision::Settle { .. } => {
            engine::materialize_validation_settlement(
                ledger,
                &options.event_timestamp(),
                decision_seq,
            )?;
            Ok(true)
        }
        ValidationDecision::Feedback { ordinal, reason } => {
            let verdict = ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .rev()
                .find(|e| {
                    e.turn() == Some(turn)
                        && e.string_field("subkind") == Some("validation.verdict")
                })
                .map(value)
                .transpose()?
                .unwrap_or(Value::Null);
            state(
                ledger,
                options,
                turn,
                "validation.feedback",
                json!({"decision_seq":decision_seq,"output_seq":output_seq,
                "repair_generation":format!("validation-{decision_seq}"),"ordinal":ordinal,"reason":reason,
                "failures":verdict["payload"]["failures"],"instruction":"Repair the requested deliverables using this feedback, in this same session, then produce a new final answer."}),
                true,
            )?;
            Ok(false)
        }
        ValidationDecision::StandDown => Ok(true),
        ValidationDecision::ForkValidator => {
            Err("validator did not produce a routing decision".into())
        }
    }
}

fn judge(
    ledger: &mut LockedLedger,
    run: ProviderRunContext<'_>,
    candidate_seq: u64,
    binding: &ValidationBinding,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
) -> Result<u64, Box<dyn std::error::Error>> {
    let spawn = ensure_validator(ledger, run, candidate_seq, binding)?;
    let launch = exchange_launch_child_runtime(&spawn, lines, stdout, cancellation)?;
    if !launch.ok {
        return state(
            ledger,
            run.options,
            binding.turn,
            "validation.death",
            json!({"candidate_seq":candidate_seq,"child":spawn.child,"reason":launch.error}),
            false,
        );
    }
    let terminal = wait_for_child_terminal(
        ledger,
        &spawn,
        cancellation,
        run.options,
        run.selected,
        stdout,
    )?;
    let call = format!("validation-{candidate_seq}");
    if !ledger.projection().unwrap().events.iter().any(|event| {
        event.kind() == &EventKind::ChildResult && event.string_field("call") == Some(call.as_str())
    }) {
        ledger.append_contract(make_event(json!({"v":1,"seq":ledger.next_seq(),"turn":binding.turn,
            "kind":"child_result","ts":run.options.event_timestamp(),"visibility":"runtime","child":spawn.child_file,
            "call":call,"spawn_id":spawn.spawn_id,"outcome":terminal.outcome}))?, BarrierContext::default())?;
    }
    let child_path = ledger.path().parent().unwrap().join(&spawn.child_file);
    let child = loop {
        match LockedLedger::open(&child_path, 1) {
            Ok(child) => break child,
            Err(StoreError::Busy)
                if !cancellation.stop_requested() && !cancellation.supervisor_lost() =>
            {
                std::thread::sleep(Duration::from_millis(25))
            }
            Err(error) => return Err(error.into()),
        }
    };
    let events = &child.projection().ok_or("validator ledger missing")?.events;
    let calls: BTreeSet<_> = events
        .iter()
        .filter(|e| e.kind() == &EventKind::ToolCall && e.string_field("name") == Some("verify"))
        .filter_map(|e| e.string_field("call").map(str::to_owned))
        .collect();
    if terminal.outcome == "completed" {
        for event in events.iter().filter(|e| {
            e.kind() == &EventKind::ToolResult
                && e.string_field("outcome") == Some("ok")
                && e.string_field("call")
                    .is_some_and(|call| calls.contains(call))
        }) {
            let result = result_value(&child, event)?;
            let covered: BTreeMap<String, String> = result["covered_set"]
                .as_array()
                .ok_or("verify lacks covered_set")?
                .iter()
                .map(|v| {
                    Ok((
                        v["id"].as_str().ok_or("verify id missing")?.to_owned(),
                        v["dedup_key"]
                            .as_str()
                            .ok_or("verify dedup_key missing")?
                            .to_owned(),
                    ))
                })
                .collect::<Result<_, Box<dyn std::error::Error>>>()?;
            if covered != binding.snapshot
                || result["covered_set"].as_array().unwrap().len() != covered.len()
            {
                continue;
            }
            let changed = binding.snapshot.iter().filter(|(path,sha)| artifact_bytes(run.profile,path).map(|bytes| !matches_snapshot(&bytes,sha)).unwrap_or(true))
                .map(|(path,_)| json!({"id":path,"issues":["Artifact no longer matches the frozen candidate snapshot"],"guidance":["Restore or repair the requested deliverable and emit a new candidate."]})).collect::<Vec<_>>();
            let (verdict, failures) = if changed.is_empty() {
                (result["verdict"].clone(), result["failures"].clone())
            } else {
                (json!("fail"), json!(changed))
            };
            return state(
                ledger,
                run.options,
                binding.turn,
                "validation.verdict",
                json!({"candidate_seq":candidate_seq,
                "child":spawn.child,"verify_seq":event.seq(),"verdict":verdict,"failures":failures}),
                false,
            );
        }
    }
    state(
        ledger,
        run.options,
        binding.turn,
        "validation.death",
        json!({"candidate_seq":candidate_seq,"child":spawn.child,"reason":"validator ended without a snapshot-bound verify"}),
        false,
    )
}

fn ensure_validator(
    ledger: &mut LockedLedger,
    run: ProviderRunContext<'_>,
    candidate_seq: u64,
    binding: &ValidationBinding,
) -> Result<DurableChildSpawn, Box<dyn std::error::Error>> {
    let call = format!("validation-{candidate_seq}");
    if let Some(event) =
        ledger.projection().unwrap().events.iter().find(|e| {
            e.kind() == &EventKind::Spawn && e.string_field("call") == Some(call.as_str())
        })
    {
        let child_file = event
            .string_field("child")
            .ok_or("validator spawn child missing")?
            .to_owned();
        return Ok(DurableChildSpawn {
            child: child_file.trim_end_matches(".jsonl").to_owned(),
            child_file,
            spawn_id: event
                .string_field("spawn_id")
                .ok_or("validator spawn id missing")?
                .to_owned(),
            resume: ResumePolicy::Bounded(3),
        });
    }
    let seq = ledger.next_seq();
    let (child, spawn_id) = child_identity(&binding.thread, &call, seq);
    let child_file = format!("{child}.jsonl");
    let folder = ledger.path().parent().ok_or("validator folder missing")?;
    let child_path = folder.join(&child_file);
    let (scratch, snapshot_root) = validator_workspace_paths(&child_path)?;
    fs::create_dir_all(&scratch)?;
    fs::create_dir_all(&snapshot_root)?;
    if scratch.canonicalize()? != scratch || snapshot_root.canonicalize()? != snapshot_root {
        return Err("validator private workspace resolves outside its directory".into());
    }
    let events = &ledger.projection().unwrap().events;
    let inputs = current_turn_inputs(ledger, binding.turn)?;
    let request = events
        .iter()
        .filter(|e| inputs.contains(&e.seq()))
        .map(|e| {
            let raw = value(e)?;
            materialize_json(
                ledger,
                raw.get("content").ok_or("validation input missing")?,
            )
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let output = events
        .iter()
        .find(|e| e.seq() == binding.output_seq)
        .ok_or("candidate output missing")?;
    let output = materialize_json(ledger, &value(output)?["content"])?;
    let covered_set: Vec<_> = binding
        .snapshot
        .iter()
        .map(|(id, dedup_key)| json!({"id":id,"dedup_key":dedup_key}))
        .collect();
    let assets = AssetStore::new(folder.join("assets"))?;
    let mut frozen_artifacts = Vec::new();
    for (path, sha) in &binding.snapshot {
        let frozen = match artifact_bytes(run.profile, path) {
            Ok(bytes) if matches_snapshot(&bytes, sha) => {
                let scan = SecretScanner::default().scan(&IJsonValue::from(
                    String::from_utf8_lossy(&bytes).into_owned(),
                ));
                if matches!(scan, SecretScan::Clean(_)) {
                    let asset = assets.publish(&bytes)?;
                    let snapshot_path = freeze_artifact_copy(
                        &snapshot_root,
                        &run.profile.config.workspace.cwd,
                        path,
                        &bytes,
                    )?;
                    json!({"id":path,"dedup_key":sha,"asset":asset.asset,
                        "snapshot_path":snapshot_path,"text":if bytes.len() <= 64*1024 { std::str::from_utf8(&bytes).ok() } else { None }})
                } else {
                    json!({"id":path,"dedup_key":sha,"unavailable":"snapshot content withheld by secret scanner; use inconclusive if evidence cannot be established"})
                }
            }
            _ => {
                json!({"id":path,"dedup_key":sha,"unavailable":"current file does not match candidate; do not pass without evidence"})
            }
        };
        frozen_artifacts.push(frozen);
    }
    let execution_evidence = execution_evidence(events, binding.turn, binding.output_seq)?;
    let seed_event = make_event(
        json!({"v":1,"seq":candidate_seq,"turn":binding.turn,"kind":"state","ts":run.options.event_timestamp(),
        "subkind":"validation.judge","visibility":"model","payload":{"request":request,"candidate":output,
        "covered_set":covered_set,"frozen_artifacts":frozen_artifacts,"execution_evidence":execution_evidence,"instruction":JUDGE_SYSTEM}}),
    )?;
    let mut seed_bytes = seed_event.canonical_bytes()?;
    seed_bytes.push(b'\n');
    let seed = AssetStore::new(folder.join("assets"))?.publish(&seed_bytes)?;
    let parent = ledger
        .path()
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("parent filename missing")?;
    let origin = format!("spawn:{spawn_id}");
    let genesis = make_event(
        json!({"v":1,"seq":1,"kind":"genesis","ts":run.options.event_timestamp(),
        "format":1,"min_reader":1,"min_writer":1,"thread":child,"workspace":run.profile.config.workspace.id,
        "origin_key":origin,"origin_tuple":{"principal":"kernel-worker","client":"worker","target":child,"op":"spawn","key":origin},
        "parent":{"file":parent,"seq":seq,"spawn_id":spawn_id},"resume":{"bounded":3},
        "config":{"digest":run.options.config_digest},
        "validator_for":{"candidate_seq":candidate_seq,"binding":binding},
        "seed":{"source":binding.thread,"kinds":["state"],"snapshot":{"asset":seed.asset,"digest":seed.asset.strip_prefix("sha256-").unwrap()}}}),
    )?;
    publish_child_genesis(folder, &child_file, genesis)?;
    ledger.append_contract(make_event(json!({"v":1,"seq":seq,"turn":binding.turn,"kind":"spawn","ts":run.options.event_timestamp(),
        "child":child_file,"call":call,"spawn_id":spawn_id,"resume":{"bounded":3},"seed":{"kinds":["state"]}}))?,BarrierContext::default())?;
    Ok(DurableChildSpawn {
        child,
        child_file,
        spawn_id,
        resume: ResumePolicy::Bounded(3),
    })
}

#[cfg(test)]
mod artifact_snapshot_tests {
    use super::*;

    fn revision(version: Option<u64>, source: &str, sequence: u64, sha: &str) -> ArtifactRevision {
        ArtifactRevision {
            version,
            source: source.into(),
            sequence,
            sha256: sha.into(),
        }
    }

    #[test]
    fn path_aliases_collapse_and_newest_version_wins_independent_of_branch_order() {
        let root = std::path::Path::new("/workspace");
        let relative = artifact_identity(root, "./note.txt").unwrap();
        let absolute = artifact_identity(root, "/workspace/note.txt").unwrap();
        assert_eq!(relative, absolute);
        for reversed in [false, true] {
            let mut snapshot = ArtifactSnapshot::default();
            let mut records = vec![
                revision(Some(2), "child-a", 100, "old"),
                revision(Some(3), "child-b", 5, "new"),
            ];
            if reversed {
                records.reverse();
            }
            for record in records {
                snapshot.observe(relative.clone(), record).unwrap();
            }
            assert_eq!(snapshot.0[&absolute].source, "child-b");
            assert_eq!(
                snapshot.covered_set(),
                BTreeMap::from([(absolute.clone(), "new".into())])
            );
        }
        assert!(artifact_identity(root, "../outside").is_err());
    }

    #[test]
    fn ambiguous_or_conflicting_branch_versions_cannot_be_silently_promoted() {
        let mut snapshot = ArtifactSnapshot::default();
        snapshot
            .observe("file".into(), revision(Some(2), "a", 10, "old"))
            .unwrap();
        assert!(
            snapshot
                .observe("file".into(), revision(Some(2), "b", 20, "different"))
                .is_err()
        );
        assert!(
            snapshot
                .observe("file".into(), revision(None, "b", 30, "different"))
                .is_err()
        );
        assert_eq!(snapshot.0["file"].sha256, "old");
    }

    #[test]
    fn legacy_local_unversioned_history_uses_its_own_ledger_order() {
        let mut snapshot = ArtifactSnapshot::default();
        snapshot
            .observe("file".into(), revision(None, "a", 20, "new"))
            .unwrap();
        snapshot
            .observe("file".into(), revision(None, "a", 10, "old"))
            .unwrap();
        assert_eq!(snapshot.0["file"].sha256, "new");
    }
}

#[cfg(test)]
mod artifact_tree_tests {
    use super::*;

    struct Branch {
        id: &'static str,
        revision: Option<u64>,
        children: Vec<Branch>,
    }

    fn write_branch(folder: &std::path::Path, branch: &Branch, parent: Option<Value>) -> PathBuf {
        let path = folder.join(format!("{}.jsonl", branch.id));
        let asset = AssetStore::new(folder.join("assets"))
            .unwrap()
            .publish(b"{}")
            .unwrap();
        let mut records = Vec::new();
        let mut push = |mut value: Value| {
            value["v"] = json!(1);
            value["seq"] = json!(records.len() + 1);
            value["ts"] = json!("2026-08-27T09:00:04.000Z");
            records.push(value);
            records.len() as u64
        };
        let mut genesis = json!({"kind":"genesis","format":1,"min_reader":1,"min_writer":1,
            "thread":branch.id,"workspace":"ws","origin_key":"fixture",
            "origin_tuple":{"principal":"test","client":"test","target":branch.id,"op":"create","key":"fixture"},
            "resume":"never","config":{"digest":"cfg"}});
        let spawned = parent.is_some();
        if let Some(parent) = parent {
            genesis["parent"] = parent;
        }
        push(genesis);
        if spawned {
            push(json!({"kind":"turn_open","turn":1,"trigger":"genesis"}));
        } else {
            let input = push(
                json!({"kind":"input","content":[{"type":"text","text":"deliver"}],
                "origin_key":"input","origin_tuple":{"client":"test","key":"input","op":"submit","principal":"test","target":branch.id}}),
            );
            push(json!({"kind":"turn_open","turn":1,"trigger":{"inputs":[input]}}));
        }
        push(
            json!({"kind":"epoch","id":"epoch","reason":"initial","adapter":"responses","model":"m",
            "system":{"asset":asset.asset,"digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
        );
        push(
            json!({"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"turn":1,"attempt":"attempt","epoch":"epoch","wire_digest":"wire","admits":[]}),
        );
        if branch.revision.is_some() {
            push(
                json!({"kind":"tool_call","turn":1,"attempt":"attempt","call":"write","name":"apply_patch",
                "args":{},"source":"provider"}),
            );
        }
        for child in &branch.children {
            push(
                json!({"kind":"tool_call","turn":1,"attempt":"attempt","call":child.id,"name":"task",
                "args":{},"source":"provider"}),
            );
        }
        push(
            json!({"kind":"output","turn":1,"attempt":"attempt","content":[],
            "usage":{"availability":"unavailable"},
            "sealed":{"version":1,"adapter":"responses","fragments":"[]"}}),
        );
        if let Some(version) = branch.revision {
            push(
                json!({"kind":"tool_result","turn":1,"call":"write","outcome":"ok",
                "content":[{"type":"text","text":json!({"path":"shared.txt","artifact_version":version,
                    "sha256":format!("version-{version}")}).to_string()}]}),
            );
        }
        for child in &branch.children {
            let child_file = format!("{}.jsonl", child.id);
            let spawn_id = format!("spawn-{}", child.id);
            let seq = push(
                json!({"kind":"spawn","turn":1,"child":child_file,"call":child.id,
                "spawn_id":spawn_id,"resume":"never","seed":{"kinds":[]}}),
            );
            write_branch(
                folder,
                child,
                Some(json!({"file":path.file_name().unwrap().to_str().unwrap(),
                "seq":seq,"spawn_id":spawn_id})),
            );
            push(
                json!({"kind":"child_result","turn":1,"child":child_file,"call":child.id,
                "spawn_id":spawn_id,"outcome":"completed"}),
            );
            push(
                json!({"kind":"tool_result","turn":1,"call":child.id,"outcome":"ok",
                "content":[{"type":"text","text":"joined"}]}),
            );
        }
        push(json!({"kind":"settle","turn":1,"outcome":"completed"}));
        let mut bytes = Vec::new();
        for record in records {
            bytes.extend(make_event(record).unwrap().canonical_bytes().unwrap());
            bytes.push(b'\n');
        }
        let scan = scan_valid_prefix(&bytes, 1);
        assert_eq!(
            scan.valid_bytes,
            bytes.len() as u64,
            "fixture {} rejected after seq {:?}",
            branch.id,
            scan.projection
                .as_ref()
                .map(|p| p.events.last().map(Event::seq))
        );
        fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn nested_and_competing_branches_choose_latest_shared_version() {
        for reversed in [false, true] {
            let folder = tempfile::tempdir().unwrap();
            let mut children = vec![
                Branch {
                    id: "018f0000-0000-7000-8000-000000000012",
                    revision: Some(4),
                    children: vec![Branch {
                        id: "018f0000-0000-7000-8000-000000000013",
                        revision: Some(7),
                        children: vec![],
                    }],
                },
                Branch {
                    id: "018f0000-0000-7000-8000-000000000014",
                    revision: Some(5),
                    children: vec![],
                },
            ];
            if reversed {
                children.reverse();
            }
            let root = Branch {
                id: "018f0000-0000-7000-8000-000000000011",
                revision: Some(6),
                children,
            };
            let path = write_branch(folder.path(), &root, None);
            let projection = read_child_projection(&path).unwrap();
            let mut snapshot = ArtifactSnapshot::default();
            collect_artifacts(
                &path,
                &projection,
                Some(1),
                folder.path(),
                &mut BTreeSet::new(),
                &mut snapshot,
            )
            .unwrap();
            let identity = folder
                .path()
                .join("shared.txt")
                .to_string_lossy()
                .into_owned();
            assert_eq!(snapshot.0[&identity].version, Some(7));
            assert_eq!(
                snapshot.0[&identity].source,
                "018f0000-0000-7000-8000-000000000013"
            );
            assert_eq!(
                snapshot.covered_set(),
                BTreeMap::from([(identity, "version-7".to_owned())])
            );
            let grandchild = folder
                .path()
                .join("018f0000-0000-7000-8000-000000000013.jsonl");
            let mut damaged = fs::read(&grandchild).unwrap();
            damaged.extend_from_slice(b"{\"kind\":");
            fs::write(&grandchild, damaged).unwrap();
            assert!(
                collect_artifacts(
                    &path,
                    &projection,
                    Some(1),
                    folder.path(),
                    &mut BTreeSet::new(),
                    &mut ArtifactSnapshot::default()
                )
                .is_err(),
                "a torn descendant cannot silently provide an older snapshot"
            );
            let mut other_turn = ArtifactSnapshot::default();
            collect_artifacts(
                &path,
                &projection,
                Some(2),
                folder.path(),
                &mut BTreeSet::new(),
                &mut other_turn,
            )
            .unwrap();
            assert!(other_turn.0.is_empty());
        }
    }
}
