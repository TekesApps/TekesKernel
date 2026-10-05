//! Durable successful-edit facts, independent of Git and repository snapshots.
use crate::{Failure, fail};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Edit {
    pub event_id: String,
    pub path: String,
    pub before: Option<String>,
    pub after: Option<String>,
    /// Large edits retain identity hashes instead of content; counts stay unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omitted_content_hashes: Option<[Option<String>; 2]>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Request {
    pub workspace_id: String,
    pub session_id: String,
    pub turn_id: String,
    pub edit: Option<Edit>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub include_diff: bool,
}

// Render only durable, continuous snapshots. Bound both the comparison table
// and output; an unavailable patch remains absent rather than fabricated.
fn unified_diff(path: &str, before: Option<&str>, after: Option<&str>) -> Option<String> {
    let left: Vec<_> = before.unwrap_or("").split_inclusive('\n').collect();
    let right: Vec<_> = after.unwrap_or("").split_inclusive('\n').collect();
    let width = right.len() + 1;
    let cells = (left.len() + 1).checked_mul(width)?;
    if cells > 4_000_000 {
        return None;
    }
    let mut table = vec![0u32; cells];
    for i in (0..left.len()).rev() {
        for j in (0..right.len()).rev() {
            table[i * width + j] = if left[i] == right[j] {
                1 + table[(i + 1) * width + j + 1]
            } else {
                table[(i + 1) * width + j].max(table[i * width + j + 1])
            };
        }
    }
    let label = |exists: bool| {
        if exists {
            serde_json::to_string(path).unwrap()
        } else {
            "/dev/null".into()
        }
    };
    let mut patch = format!(
        "--- {}\n+++ {}\n@@ -{},{} +{},{} @@\n",
        label(before.is_some()),
        label(after.is_some()),
        usize::from(!left.is_empty()),
        left.len(),
        usize::from(!right.is_empty()),
        right.len()
    );
    let (mut i, mut j) = (0, 0);
    while i < left.len() || j < right.len() {
        let (prefix, line) = if i < left.len() && j < right.len() && left[i] == right[j] {
            let line = left[i];
            i += 1;
            j += 1;
            (' ', line)
        } else if i < left.len()
            && (j == right.len() || table[(i + 1) * width + j] >= table[i * width + j + 1])
        {
            let line = left[i];
            i += 1;
            ('-', line)
        } else {
            let line = right[j];
            j += 1;
            ('+', line)
        };
        patch.push(prefix);
        patch.push_str(line);
        if !line.ends_with('\n') {
            patch.push_str("\n\\ No newline at end of file\n");
        }
        if patch.len() > 1024 * 1024 {
            return None;
        }
    }
    Some(patch)
}

// Common equal prefixes/suffixes keep ordinary edits linear; bound pathological
// comparisons and report incomplete instead of inventing exact counts.
fn line_counts(before: &str, after: &str) -> Option<(usize, usize)> {
    let left = before.split_inclusive('\n').collect::<Vec<_>>();
    let right = after.split_inclusive('\n').collect::<Vec<_>>();
    let prefix = left.iter().zip(&right).take_while(|(a, b)| a == b).count();
    let left = &left[prefix..];
    let right = &right[prefix..];
    let suffix = left
        .iter()
        .rev()
        .zip(right.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let left = &left[..left.len() - suffix];
    let right = &right[..right.len() - suffix];
    if left.len().saturating_mul(right.len()) > 4_000_000 {
        return None;
    }
    let mut row = vec![0usize; right.len() + 1];
    for a in left {
        let mut diagonal = 0;
        for (j, b) in right.iter().enumerate() {
            let previous = row[j + 1];
            row[j + 1] = if a == b {
                diagonal + 1
            } else {
                row[j + 1].max(row[j])
            };
            diagonal = previous;
        }
    }
    let common = row[right.len()];
    Some((right.len() - common, left.len() - common))
}

pub fn execute(
    state: &Path,
    workspace: &Path,
    method: &str,
    request: Request,
) -> Result<Value, Failure> {
    execute_with_roots(state, workspace, &[], method, request)
}

pub fn execute_with_roots(
    state: &Path,
    workspace: &Path,
    additional_roots: &[std::path::PathBuf],
    method: &str,
    request: Request,
) -> Result<Value, Failure> {
    if [&request.workspace_id, &request.session_id, &request.turn_id]
        .iter()
        .any(|s| s.is_empty() || s.len() > 4096 || s.contains('\0'))
    {
        return Err(fail(
            "invalid-request",
            "Expected workspace, session and turn identities",
        ));
    }
    if ![
        "turnChanges",
        "recordTurnEdit",
        "prepareTurnEdit",
        "abortTurnEdit",
    ]
    .contains(&method)
    {
        return Err(fail("unsupported", "Unknown turn operation"));
    }
    if (method != "turnChanges") != request.edit.is_some()
        || (method != "turnChanges" && request.include_diff)
    {
        return Err(fail("invalid-request", "Unexpected edit payload"));
    }
    if let Some(edit) = &request.edit {
        if edit.event_id.is_empty()
            || edit.event_id.len() > 4096
            || edit.path.is_empty()
            || edit.path.contains('\0')
            || !Path::new(&edit.path).is_absolute()
        {
            return Err(fail("invalid-request", "Expected canonical edit identity"));
        }
        // The capture owner provides the canonical target even for files deleted by
        // the edit. Do not resolve it against today's filesystem during replay.
        let mut roots = vec![workspace.canonicalize()?];
        for root in additional_roots {
            roots.push(root.canonicalize()?);
        }
        if !roots
            .iter()
            .any(|root| Path::new(&edit.path).starts_with(root))
            || Path::new(&edit.path)
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(fail(
                "path-outside-workspace",
                "Edit target escapes workspace",
            ));
        }
    }
    let directory = state.join("turn-changes");
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    builder.mode(0o700);
    builder.create(&directory)?;
    let identity = serde_json::to_vec(&(&request.workspace_id, &request.session_id))
        .map_err(|e| fail("invalid-request", e.to_string()))?;
    let key = format!("{:x}", Sha256::digest(identity));
    let mut lock_options = std::fs::OpenOptions::new();
    #[cfg(unix)]
    lock_options.mode(0o600);
    let lock = lock_options
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(directory.join(format!("{key}.lock")))?;
    rustix::fs::flock(&lock, rustix::fs::FlockOperation::LockExclusive)
        .map_err(std::io::Error::from)?;
    let path = directory.join(format!("{key}.jsonl"));
    let mut bytes = Vec::new();
    match std::fs::File::open(&path) {
        Ok(file) => {
            file.take(64 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }
    if bytes.len() > 64 * 1024 * 1024 {
        return Err(fail("storage-limit", "Turn ledger exceeds read limit"));
    }
    if !bytes.is_empty() && !bytes.ends_with(b"\n") {
        return Err(fail("corrupt-ledger", "Incomplete edit record"));
    }
    let mut records = Vec::<Request>::new();
    for line in bytes.split(|b| *b == b'\n').filter(|line| !line.is_empty()) {
        let record: Request = serde_json::from_slice(line)
            .map_err(|_| fail("corrupt-ledger", "Invalid edit record"))?;
        if record.workspace_id != request.workspace_id
            || record.session_id != request.session_id
            || record.edit.is_none()
        {
            return Err(fail("corrupt-ledger", "Unexpected ledger identity"));
        }
        records.push(record);
    }
    let intents = directory.join(format!("{key}.intents"));
    if method == "prepareTurnEdit" || method == "abortTurnEdit" {
        builder.create(&intents)?;
        let edit = request.edit.as_ref().unwrap();
        let id = format!("{:x}", Sha256::digest(edit.event_id.as_bytes()));
        let intent = intents.join(format!("{id}.intent"));
        let aborted = intents.join(format!("{id}.aborted"));
        if aborted.exists() {
            return Err(fail("event-conflict", "Edit was already aborted"));
        }
        if intent.exists() {
            let previous: Request = serde_json::from_slice(&std::fs::read(&intent)?)
                .map_err(|_| fail("corrupt-ledger", "Invalid edit intent"))?;
            if previous != request {
                return Err(fail("event-conflict", "Edit intent identity conflict"));
            }
        } else if method == "prepareTurnEdit" {
            let mut pending = tempfile::NamedTempFile::new_in(&intents)?;
            pending.write_all(
                &serde_json::to_vec(&request)
                    .map_err(|e| fail("invalid-request", e.to_string()))?,
            )?;
            pending.as_file().sync_all()?;
            pending
                .persist(&intent)
                .map_err(|e| Failure::from(e.error))?;
        } else {
            return Err(fail("event-conflict", "Missing edit intent"));
        }
        if method == "abortTurnEdit" {
            std::fs::rename(intent, aborted)?;
        }
        std::fs::File::open(&intents)?.sync_all()?;
        std::fs::File::open(&directory)?.sync_all()?;
    }
    if let Some(edit) = request.edit.as_ref().filter(|_| method == "recordTurnEdit") {
        let id = format!("{:x}", Sha256::digest(edit.event_id.as_bytes()));
        if intents.join(format!("{id}.aborted")).exists() {
            return Err(fail("event-conflict", "Edit was aborted"));
        }
        let pending = intents.join(format!("{id}.intent"));
        if pending.exists() {
            let previous: Request = serde_json::from_slice(&std::fs::read(pending)?)
                .map_err(|_| fail("corrupt-ledger", "Invalid edit intent"))?;
            if previous != request {
                return Err(fail("event-conflict", "Committed edit differs from intent"));
            }
        }
        if let Some(previous) = records
            .iter()
            .find(|r| r.edit.as_ref().unwrap().event_id == edit.event_id)
        {
            if previous != &request {
                return Err(fail(
                    "event-conflict",
                    "Edit identity already has another value",
                ));
            }
        } else {
            let mut record =
                serde_json::to_vec(&request).map_err(|e| fail("invalid-request", e.to_string()))?;
            record.push(b'\n');
            if bytes.len() + record.len() > 64 * 1024 * 1024 {
                return Err(fail("storage-limit", "Turn ledger exceeds limit"));
            }
            let mut options = std::fs::OpenOptions::new();
            #[cfg(unix)]
            options.mode(0o600);
            let mut file = options.create(true).append(true).open(&path)?;
            file.write_all(&record)?;
            file.sync_all()?;
            std::fs::File::open(&directory)?.sync_all()?;
            records.push(request.clone());
        }
    }
    let mut files = BTreeMap::<String, (Option<String>, Option<String>, bool)>::new();
    let mut revision = 0;
    for record in records.iter().filter(|r| r.turn_id == request.turn_id) {
        revision += 2;
        let edit = record.edit.as_ref().unwrap();
        if let Some(file) = files.get_mut(&edit.path) {
            file.2 &= file.1 == edit.before && edit.omitted_content_hashes.is_none();
            file.1 = edit.after.clone();
        } else {
            files.insert(
                edit.path.clone(),
                (
                    edit.before.clone(),
                    edit.after.clone(),
                    edit.omitted_content_hashes.is_none(),
                ),
            );
        }
    }
    if intents.exists() {
        let mut visited = 0;
        for entry in std::fs::read_dir(&intents)? {
            let entry = entry?;
            if !entry
                .path()
                .extension()
                .is_some_and(|e| e == "intent" || e == "aborted")
            {
                continue;
            }
            visited += 1;
            if visited > 10000 {
                return Err(fail("storage-limit", "Too many edit intents"));
            }
            let mut bytes = Vec::new();
            std::fs::File::open(entry.path())?
                .take(1024 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            if bytes.len() > 1024 * 1024 {
                return Err(fail("storage-limit", "Edit intent too large"));
            }
            let pending: Request = serde_json::from_slice(&bytes)
                .map_err(|_| fail("corrupt-ledger", "Invalid edit intent"))?;
            if pending.workspace_id != request.workspace_id
                || pending.session_id != request.session_id
            {
                return Err(fail("corrupt-ledger", "Wrong edit intent identity"));
            }
            if pending.turn_id != request.turn_id {
                continue;
            }
            let edit = pending
                .edit
                .as_ref()
                .ok_or_else(|| fail("corrupt-ledger", "Missing intent edit"))?;
            let aborted = entry.path().extension().is_some_and(|e| e == "aborted");
            revision += if aborted { 2 } else { 1 };
            if aborted {
                continue;
            }
            if let Some(committed) = records
                .iter()
                .find(|r| r.edit.as_ref().unwrap().event_id == edit.event_id)
            {
                if committed != &pending {
                    return Err(fail("event-conflict", "Committed edit differs from intent"));
                }
                continue;
            }
            files
                .entry(edit.path.clone())
                .and_modify(|file| file.2 = false)
                .or_insert((edit.before.clone(), edit.after.clone(), false));
        }
    }
    let mut summaries = Vec::new();
    let mut additions = 0;
    let mut deletions = 0;
    let mut complete = true;
    for (path, (before, after, continuous)) in files {
        if continuous && before == after {
            continue;
        }
        let counts = continuous
            .then(|| {
                line_counts(
                    before.as_deref().unwrap_or(""),
                    after.as_deref().unwrap_or(""),
                )
            })
            .flatten();
        if let Some((added, deleted)) = counts {
            additions += added;
            deletions += deleted;
            let mut summary =
                json!({"path":path,"additions":added,"deletions":deleted,"state":"available"});
            if request.include_diff {
                if let Some(patch) = unified_diff(&path, before.as_deref(), after.as_deref()) {
                    summary["unifiedDiff"] = json!(patch);
                }
            }
            summaries.push(summary);
        } else {
            complete = false;
            summaries.push(json!({"path":path,"additions":null,"deletions":null,"state":if continuous {"incomplete"} else {"discontinuous"}}));
        }
    }
    Ok(
        json!({"sessionId":request.session_id,"turnId":request.turn_id,"metric":"net","coverage":"recorded-edits",
        "state":if revision==0 {"unobserved"} else if complete {"available"} else {"discontinuous"},"fileCount":summaries.len(),"files":summaries,
        "additions":(revision>0 && complete).then_some(additions),"deletions":(revision>0 && complete).then_some(deletions),"revision":revision}),
    )
}
