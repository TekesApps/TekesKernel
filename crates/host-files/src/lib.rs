//! host-files: client extension authority for host directory browsing and
//! prompt mention discovery.
//!
//! Four client-request methods are served here (see [`METHODS`]):
//!
//! * `directory.list` / `directory.create` back the Tekes
//!   `SessionDirectoryPickerEndpoint` contract. They browse absolute host
//!   directories (directories only, never files) rooted at the caller-supplied
//!   home directory. This crate never reads `$HOME`; the supervisor passes it.
//! * `session.references.files` / `session.references.sessions` back the
//!   `SessionReferenceEndpoint` contract. File candidates are relative to the
//!   session's primary workspace root; session candidates come from an
//!   explicit slice the supervisor builds from its own session inventory.
//!
//! The crate performs payload validation, filesystem work and result shaping.
//! Mapping [`Failure`] onto the wire error envelope belongs to the supervisor.

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};

/// Client-request method names served by this crate.
pub const METHODS: &[&str] = &[
    "directory.list",
    "directory.create",
    "session.references.files",
    "session.references.sessions",
];

/// Maximum directory entries returned by `directory.list` before `truncated`.
pub const DIRECTORY_LIST_CAP: usize = 2000;
/// Maximum file reference candidates returned by `session.references.files`.
pub const FILE_REFERENCE_LIMIT: usize = 50;
/// Maximum filesystem entries visited while searching file references.
pub const FILE_REFERENCE_WALK_BUDGET: usize = 20_000;
/// Session candidates returned for an empty query.
pub const SESSION_REFERENCE_LIMIT: usize = 20;

/// Directory names never descended into during file reference search.
const SKIPPED_DIRECTORIES: &[&str] = &[".git", "node_modules", "target", ".build", "DerivedData"];

/// Stable failure surface; every arm carries a wire error code and message.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Failure {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DirectoryNotFound(String),
    #[error("{0}")]
    DirectoryExists(String),
    #[error("{0}")]
    DirectoryForbidden(String),
    #[error("{0}")]
    SessionNotFound(String),
    #[error("{0}")]
    Io(String),
}

impl Failure {
    /// Stable machine-readable error code for the wire envelope.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad-request",
            Self::DirectoryNotFound(_) => "directory-not-found",
            Self::DirectoryExists(_) => "directory-exists",
            Self::DirectoryForbidden(_) => "directory-forbidden",
            Self::SessionNotFound(_) => "session-not-found",
            Self::Io(_) => "io-error",
        }
    }

    /// Human-readable message for the wire envelope.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::BadRequest(m)
            | Self::DirectoryNotFound(m)
            | Self::DirectoryExists(m)
            | Self::DirectoryForbidden(m)
            | Self::SessionNotFound(m)
            | Self::Io(m) => m,
        }
    }
}

fn io_failure(path: &Path, error: &std::io::Error) -> Failure {
    let shown = path.display();
    match error.kind() {
        std::io::ErrorKind::NotFound => Failure::DirectoryNotFound(format!("{shown}: not found")),
        std::io::ErrorKind::PermissionDenied => {
            Failure::DirectoryForbidden(format!("{shown}: permission denied"))
        }
        std::io::ErrorKind::AlreadyExists => {
            Failure::DirectoryExists(format!("{shown}: already exists"))
        }
        _ => Failure::Io(format!("{shown}: {error}")),
    }
}

/// A session visible to the host, offered as a mention candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionCandidate {
    pub session_id: String,
    pub title: Option<String>,
    pub cwd: String,
    pub workspace_id: String,
    /// Milliseconds since the Unix epoch.
    pub created_at_ms: f64,
    pub archived: bool,
}

// ---------------------------------------------------------------------------
// Payload validation
// ---------------------------------------------------------------------------

fn object<'a>(
    payload: &'a Value,
    allowed: &[&str],
) -> Result<&'a serde_json::Map<String, Value>, String> {
    let map = payload.as_object().ok_or("payload must be an object")?;
    for key in map.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(map)
}

fn required_string(map: &serde_json::Map<String, Value>, key: &str) -> Result<(), String> {
    match map.get(key) {
        Some(Value::String(_)) => Ok(()),
        Some(_) => Err(format!("`{key}` must be a string")),
        None => Err(format!("missing `{key}`")),
    }
}

/// Validate the closed payload shape for one of [`METHODS`].
///
/// # Errors
/// Returns a message naming the first offending field, or an unknown method.
pub fn validate(method: &str, payload: &Value) -> Result<(), String> {
    match method {
        "directory.list" => {
            let map = object(payload, &["path"])?;
            match map.get("path") {
                None | Some(Value::String(_)) => Ok(()),
                Some(_) => Err("`path` must be a string".into()),
            }
        }
        "directory.create" => {
            let map = object(payload, &["path", "name"])?;
            required_string(map, "path")?;
            required_string(map, "name")
        }
        "session.references.files" | "session.references.sessions" => {
            let map = object(payload, &["sessionId", "query"])?;
            required_string(map, "sessionId")?;
            required_string(map, "query")
        }
        other => Err(format!("unknown method `{other}`")),
    }
}

// ---------------------------------------------------------------------------
// directory.list / directory.create
// ---------------------------------------------------------------------------

fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn check_absolute(path: &str) -> Result<PathBuf, Failure> {
    if path.is_empty() {
        return Err(Failure::BadRequest("path must not be empty".into()));
    }
    if path.contains('\0') {
        return Err(Failure::BadRequest("path must not contain NUL".into()));
    }
    let parsed = Path::new(path);
    if !parsed.is_absolute() {
        return Err(Failure::BadRequest("path must be absolute".into()));
    }
    if parsed
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(Failure::BadRequest("path must not contain `..`".into()));
    }
    // Normalize away `.` components and trailing separators; symlinks are
    // deliberately left unresolved so the listing reflects the typed path.
    let mut normalized = PathBuf::new();
    for component in parsed.components() {
        match component {
            Component::CurDir => {}
            other => normalized.push(other),
        }
    }
    Ok(normalized)
}

fn entry_json(name: &str, path: &Path) -> Value {
    json!({ "name": name, "path": path.to_string_lossy(), "hidden": is_hidden(name) })
}

fn crumbs(path: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component);
        let name = match component {
            Component::RootDir => "/".to_owned(),
            other => other.as_os_str().to_string_lossy().into_owned(),
        };
        out.push(entry_json(&name, &current));
    }
    out
}

/// List the directories directly under `path` (default: `home`).
///
/// # Errors
/// `bad-request` for malformed paths, `directory-not-found`,
/// `directory-forbidden` or `io-error` from the filesystem.
pub fn list_directory(home: &Path, path: Option<&str>) -> Result<Value, Failure> {
    let target = match path {
        Some(path) => check_absolute(path)?,
        None => check_absolute(&home.to_string_lossy())?,
    };
    let metadata = fs::metadata(&target).map_err(|error| io_failure(&target, &error))?;
    if !metadata.is_dir() {
        return Err(Failure::BadRequest(format!(
            "{}: not a directory",
            target.display()
        )));
    }
    let read = fs::read_dir(&target).map_err(|error| io_failure(&target, &error))?;
    let mut names: BTreeSet<Vec<u8>> = BTreeSet::new();
    for entry in read {
        let entry = entry.map_err(|error| io_failure(&target, &error))?;
        // `metadata` follows symlinks so a symlinked directory is listed, but
        // its target is never resolved into the reported path.
        let is_dir = fs::metadata(entry.path())
            .map(|m| m.is_dir())
            .unwrap_or(false);
        if is_dir {
            names.insert(entry.file_name().to_string_lossy().as_bytes().to_vec());
        }
    }
    let truncated = names.len() > DIRECTORY_LIST_CAP;
    let entries: Vec<Value> = names
        .iter()
        .take(DIRECTORY_LIST_CAP)
        .map(|bytes| {
            let name = String::from_utf8_lossy(bytes);
            entry_json(&name, &target.join(&*name))
        })
        .collect();
    Ok(json!({
        "path": target.to_string_lossy(),
        "home": home.to_string_lossy(),
        "crumbs": crumbs(&target),
        "entries": entries,
        "truncated": truncated,
    }))
}

/// Create `name` under `path` and return the new absolute path as a JSON string.
///
/// # Errors
/// `bad-request` for malformed inputs, `directory-exists` when the target
/// exists, `directory-not-found` / `directory-forbidden` / `io-error` otherwise.
pub fn create_directory(path: &str, name: &str) -> Result<Value, Failure> {
    let parent = check_absolute(path)?;
    if name.is_empty() || name == "." || name == ".." {
        return Err(Failure::BadRequest("invalid directory name".into()));
    }
    if name.contains('/') || name.contains('\0') {
        return Err(Failure::BadRequest(
            "directory name must not contain `/` or NUL".into(),
        ));
    }
    if !parent.is_dir() {
        return Err(Failure::DirectoryNotFound(format!(
            "{}: not a directory",
            parent.display()
        )));
    }
    let target = parent.join(name);
    if fs::symlink_metadata(&target).is_ok() {
        return Err(Failure::DirectoryExists(format!(
            "{}: already exists",
            target.display()
        )));
    }
    fs::create_dir(&target).map_err(|error| io_failure(&target, &error))?;
    Ok(Value::String(target.to_string_lossy().into_owned()))
}

// ---------------------------------------------------------------------------
// session.references.files
// ---------------------------------------------------------------------------

fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    needle.is_empty() || haystack.to_lowercase().contains(&needle.to_lowercase())
}

/// Search the session's primary workspace root for path candidates.
///
/// `roots[0]` is the primary root and the base of every relative path;
/// symlinks resolving outside any root are never followed. Results are the
/// first [`FILE_REFERENCE_LIMIT`] matches of a breadth-first walk bounded by
/// [`FILE_REFERENCE_WALK_BUDGET`] visited entries.
///
/// # Errors
/// `session-not-found` when `roots` is empty, `bad-request` for a query
/// containing NUL, `io-error` when the primary root cannot be read.
pub fn file_references(roots: &[PathBuf], query: &str) -> Result<Value, Failure> {
    let primary = roots
        .first()
        .ok_or_else(|| Failure::SessionNotFound("session has no workspace root".into()))?;
    if query.contains('\0') {
        return Err(Failure::BadRequest("query must not contain NUL".into()));
    }
    let query = query.trim();
    let include_hidden = query.starts_with('.');
    let canonical_roots: Vec<PathBuf> = roots
        .iter()
        .filter_map(|root| root.canonicalize().ok())
        .collect();
    let primary_canonical = primary
        .canonicalize()
        .map_err(|error| io_failure(primary, &error))?;

    let mut results = Vec::new();
    let mut visited = 0usize;
    let mut queue: VecDeque<PathBuf> = VecDeque::from([primary_canonical.clone()]);
    'walk: while let Some(directory) = queue.pop_front() {
        let Ok(read) = fs::read_dir(&directory) else {
            continue;
        };
        let mut children: Vec<(String, PathBuf, bool)> = Vec::new();
        for entry in read.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if is_hidden(&name) && !include_hidden {
                continue;
            }
            let path = entry.path();
            let Ok(link_meta) = fs::symlink_metadata(&path) else {
                continue;
            };
            let is_dir = if link_meta.file_type().is_symlink() {
                match path.canonicalize() {
                    Ok(resolved) if canonical_roots.iter().any(|r| resolved.starts_with(r)) => {
                        resolved.is_dir()
                    }
                    // Symlink escaping every root, or dangling: never followed.
                    _ => continue,
                }
            } else {
                link_meta.is_dir()
            };
            children.push((name, path, is_dir));
        }
        children.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
        for (name, path, is_dir) in children {
            visited += 1;
            if visited > FILE_REFERENCE_WALK_BUDGET {
                break 'walk;
            }
            let relative = path
                .strip_prefix(&primary_canonical)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            let matches = if query.is_empty() {
                directory == primary_canonical
            } else {
                contains_ignore_case(&relative, query)
            };
            if matches {
                results.push(json!({
                    "path": relative,
                    "kind": if is_dir { "directory" } else { "file" },
                }));
                if results.len() >= FILE_REFERENCE_LIMIT {
                    break 'walk;
                }
            }
            if is_dir && !query.is_empty() && !SKIPPED_DIRECTORIES.contains(&name.as_str()) {
                queue.push_back(path);
            }
        }
    }
    Ok(Value::Array(results))
}

// ---------------------------------------------------------------------------
// session.references.sessions
// ---------------------------------------------------------------------------

/// Rank mention candidates for the session identified by `requesting`.
///
/// `requesting_workspace` decides `sameWorkspace`. The requesting session and
/// archived sessions are excluded; the rest are matched by case-insensitive
/// substring on their title and ordered most recent first. An empty query
/// returns at most [`SESSION_REFERENCE_LIMIT`] rows.
#[must_use]
pub fn session_references(
    requesting: &str,
    requesting_workspace: &str,
    candidates: &[SessionCandidate],
    query: &str,
) -> Value {
    let query = query.trim();
    let mut rows: Vec<&SessionCandidate> = candidates
        .iter()
        .filter(|candidate| !candidate.archived && candidate.session_id != requesting)
        .filter(|candidate| {
            let label = candidate.title.as_deref().unwrap_or("");
            query.is_empty()
                || contains_ignore_case(label, query)
                || contains_ignore_case(&candidate.session_id, query)
        })
        .collect();
    rows.sort_by(|a, b| {
        b.created_at_ms
            .partial_cmp(&a.created_at_ms)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.session_id.cmp(&b.session_id))
    });
    if query.is_empty() {
        rows.truncate(SESSION_REFERENCE_LIMIT);
    }
    Value::Array(
        rows.into_iter()
            .map(|candidate| {
                let label = candidate
                    .title
                    .as_deref()
                    .filter(|title| !title.trim().is_empty())
                    .unwrap_or(&candidate.session_id);
                let mut row = json!({
                    "sessionId": candidate.session_id,
                    "label": label,
                    "sameWorkspace": candidate.workspace_id == requesting_workspace,
                    "createdAt": candidate.created_at_ms,
                    "mention": format!("@thread:{}", candidate.session_id),
                });
                if !candidate.cwd.is_empty() {
                    row["cwd"] = Value::String(candidate.cwd.clone());
                }
                row
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    fn tmp() -> tempfile::TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn validate_rejects_unknown_fields_and_wrong_types() {
        assert!(validate("directory.list", &json!({})).is_ok());
        assert!(validate("directory.list", &json!({"path": "/x"})).is_ok());
        assert!(validate("directory.list", &json!({"path": 1})).is_err());
        assert!(validate("directory.list", &json!({"extra": 1})).is_err());
        assert!(validate("directory.create", &json!({"path": "/x", "name": "y"})).is_ok());
        assert!(validate("directory.create", &json!({"path": "/x"})).is_err());
        assert!(
            validate(
                "session.references.files",
                &json!({"sessionId": "s", "query": ""})
            )
            .is_ok()
        );
        assert!(validate("session.references.files", &json!({"sessionId": "s"})).is_err());
        assert!(
            validate(
                "session.references.sessions",
                &json!({"sessionId": "s", "query": "", "x": 0})
            )
            .is_err()
        );
        assert!(validate("nope", &json!({})).is_err());
        assert!(validate("directory.list", &json!([])).is_err());
    }

    #[test]
    fn listing_shows_only_directories_with_crumbs_and_hidden_flags() {
        let dir = tmp();
        let home = dir.path().join("home");
        fs::create_dir_all(home.join("b")).unwrap();
        fs::create_dir_all(home.join("a")).unwrap();
        fs::create_dir_all(home.join(".hidden")).unwrap();
        fs::write(home.join("file.txt"), "x").unwrap();
        let listing = list_directory(&home, None).unwrap();
        assert_eq!(listing["path"], json!(home.to_string_lossy()));
        assert_eq!(listing["home"], json!(home.to_string_lossy()));
        assert_eq!(listing["truncated"], json!(false));
        let names: Vec<&str> = listing["entries"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec![".hidden", "a", "b"]);
        assert_eq!(listing["entries"][0]["hidden"], json!(true));
        assert_eq!(listing["entries"][1]["hidden"], json!(false));
        assert_eq!(
            listing["entries"][1]["path"],
            json!(home.join("a").to_string_lossy())
        );
        let crumbs = listing["crumbs"].as_array().unwrap();
        assert_eq!(crumbs[0]["name"], json!("/"));
        assert_eq!(crumbs[0]["path"], json!("/"));
        assert_eq!(
            crumbs.last().unwrap()["path"],
            json!(home.to_string_lossy())
        );
        assert_eq!(crumbs.last().unwrap()["name"], json!("home"));
        assert_eq!(crumbs.len(), home.components().count());
    }

    #[test]
    fn listing_explicit_path_and_symlinked_directory_not_resolved() {
        let dir = tmp();
        let home = dir.path().join("home");
        let elsewhere = dir.path().join("elsewhere");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&elsewhere).unwrap();
        symlink(&elsewhere, home.join("link")).unwrap();
        let listing = list_directory(dir.path(), Some(&home.to_string_lossy())).unwrap();
        assert_eq!(listing["home"], json!(dir.path().to_string_lossy()));
        assert_eq!(listing["entries"][0]["name"], json!("link"));
        assert_eq!(
            listing["entries"][0]["path"],
            json!(home.join("link").to_string_lossy())
        );
    }

    #[test]
    fn listing_truncates_at_cap() {
        let dir = tmp();
        for i in 0..(DIRECTORY_LIST_CAP + 5) {
            fs::create_dir(dir.path().join(format!("d{i:05}"))).unwrap();
        }
        let listing = list_directory(dir.path(), None).unwrap();
        assert_eq!(
            listing["entries"].as_array().unwrap().len(),
            DIRECTORY_LIST_CAP
        );
        assert_eq!(listing["truncated"], json!(true));
    }

    #[test]
    fn listing_rejects_bad_paths() {
        let home = Path::new("/");
        for bad in ["relative/path", "/a/../b", "/a\0b", ""] {
            let err = list_directory(home, Some(bad)).unwrap_err();
            assert_eq!(err.code(), "bad-request", "{bad:?}");
        }
        let missing = list_directory(home, Some("/definitely/not/here/xyz")).unwrap_err();
        assert_eq!(missing.code(), "directory-not-found");
        let dir = tmp();
        fs::write(dir.path().join("f"), "").unwrap();
        let file = list_directory(home, Some(&dir.path().join("f").to_string_lossy())).unwrap_err();
        assert_eq!(file.code(), "bad-request");
    }

    #[test]
    fn create_directory_succeeds_then_conflicts() {
        let dir = tmp();
        let base = dir.path().to_string_lossy().into_owned();
        let created = create_directory(&base, "new").unwrap();
        assert_eq!(created, json!(dir.path().join("new").to_string_lossy()));
        assert!(dir.path().join("new").is_dir());
        assert_eq!(
            create_directory(&base, "new").unwrap_err().code(),
            "directory-exists"
        );
        fs::write(dir.path().join("f"), "").unwrap();
        assert_eq!(
            create_directory(&base, "f").unwrap_err().code(),
            "directory-exists"
        );
    }

    #[test]
    fn create_directory_rejects_bad_names_and_parents() {
        let dir = tmp();
        let base = dir.path().to_string_lossy().into_owned();
        for bad in ["a/b", "a\0b", ".", "..", ""] {
            assert_eq!(
                create_directory(&base, bad).unwrap_err().code(),
                "bad-request",
                "{bad:?}"
            );
        }
        assert_eq!(
            create_directory("relative", "x").unwrap_err().code(),
            "bad-request"
        );
        assert_eq!(
            create_directory("/x/../y", "x").unwrap_err().code(),
            "bad-request"
        );
        let missing =
            create_directory(&dir.path().join("missing").to_string_lossy(), "x").unwrap_err();
        assert_eq!(missing.code(), "directory-not-found");
    }

    fn workspace() -> (tempfile::TempDir, PathBuf) {
        let dir = tmp();
        let root = dir.path().join("root");
        fs::create_dir_all(root.join("src/nested")).unwrap();
        fs::write(root.join("src/main.rs"), "").unwrap();
        fs::write(root.join("src/nested/Helper.swift"), "").unwrap();
        fs::write(root.join("README.md"), "").unwrap();
        fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        fs::write(root.join("node_modules/pkg/main.js"), "").unwrap();
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/HEAD"), "").unwrap();
        fs::write(root.join(".env"), "").unwrap();
        (dir, root)
    }

    fn paths(value: &Value) -> Vec<(String, String)> {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["path"].as_str().unwrap().to_owned(),
                    r["kind"].as_str().unwrap().to_owned(),
                )
            })
            .collect()
    }

    #[test]
    fn file_references_empty_query_lists_top_level_without_hidden() {
        let (_dir, root) = workspace();
        let rows = paths(&file_references(&[root], "").unwrap());
        assert_eq!(
            rows,
            vec![
                ("README.md".to_owned(), "file".to_owned()),
                ("node_modules".to_owned(), "directory".to_owned()),
                ("src".to_owned(), "directory".to_owned()),
            ]
        );
    }

    #[test]
    fn file_references_match_case_insensitively_and_skip_directories() {
        let (_dir, root) = workspace();
        let rows = paths(&file_references(&[root.clone()], "MAIN").unwrap());
        assert_eq!(rows, vec![("src/main.rs".to_owned(), "file".to_owned())]);
        let rows = paths(&file_references(&[root.clone()], "helper").unwrap());
        assert_eq!(
            rows,
            vec![("src/nested/Helper.swift".to_owned(), "file".to_owned())]
        );
        let rows = paths(&file_references(&[root.clone()], "nested").unwrap());
        assert_eq!(
            rows,
            vec![
                ("src/nested".to_owned(), "directory".to_owned()),
                ("src/nested/Helper.swift".to_owned(), "file".to_owned()),
            ]
        );
        assert!(paths(&file_references(&[root.clone()], "HEAD").unwrap()).is_empty());
        assert!(paths(&file_references(&[root.clone()], "env").unwrap()).is_empty());
        let rows = paths(&file_references(&[root], ".env").unwrap());
        assert_eq!(rows, vec![(".env".to_owned(), "file".to_owned())]);
    }

    #[test]
    fn file_references_never_follow_symlinks_outside_roots() {
        let (dir, root) = workspace();
        let outside = dir.path().join("outside");
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("secret.txt"), "").unwrap();
        symlink(&outside, root.join("escape")).unwrap();
        symlink(root.join("src"), root.join("inside")).unwrap();
        let rows = paths(&file_references(&[root.clone()], "secret").unwrap());
        assert!(rows.is_empty());
        let rows = paths(&file_references(&[root.clone()], "escape").unwrap());
        assert!(rows.is_empty());
        let rows = paths(&file_references(&[root.clone()], "inside").unwrap());
        assert_eq!(rows[0], ("inside".to_owned(), "directory".to_owned()));
        // A second root makes the escape followable.
        let rows = paths(&file_references(&[root, outside], "secret").unwrap());
        assert_eq!(
            rows,
            vec![("escape/secret.txt".to_owned(), "file".to_owned())]
        );
    }

    #[test]
    fn file_references_are_bounded() {
        let dir = tmp();
        for i in 0..(FILE_REFERENCE_LIMIT + 10) {
            fs::write(dir.path().join(format!("match{i}.txt")), "").unwrap();
        }
        let rows = file_references(&[dir.path().to_owned()], "match").unwrap();
        assert_eq!(rows.as_array().unwrap().len(), FILE_REFERENCE_LIMIT);
        assert_eq!(
            file_references(&[], "x").unwrap_err().code(),
            "session-not-found"
        );
        assert_eq!(
            file_references(&[dir.path().to_owned()], "a\0")
                .unwrap_err()
                .code(),
            "bad-request"
        );
    }

    fn candidate(
        id: &str,
        title: Option<&str>,
        ws: &str,
        at: f64,
        archived: bool,
    ) -> SessionCandidate {
        SessionCandidate {
            session_id: id.into(),
            title: title.map(Into::into),
            cwd: format!("/w/{ws}"),
            workspace_id: ws.into(),
            created_at_ms: at,
            archived,
        }
    }

    #[test]
    fn session_references_exclude_self_and_archived_and_rank_recent_first() {
        let candidates = vec![
            candidate("me", Some("Me"), "w1", 900.0, false),
            candidate("old", Some("Old plan"), "w1", 100.0, false),
            candidate("new", Some("New Plan"), "w2", 300.0, false),
            candidate("gone", Some("Plan archived"), "w1", 500.0, true),
            candidate("untitled", None, "w1", 200.0, false),
        ];
        let rows = session_references("me", "w1", &candidates, "");
        let ids: Vec<&str> = rows
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["sessionId"].as_str().unwrap())
            .collect();
        assert_eq!(ids, vec!["new", "untitled", "old"]);
        let first = &rows[0];
        assert_eq!(first["label"], json!("New Plan"));
        assert_eq!(first["cwd"], json!("/w/w2"));
        assert_eq!(first["sameWorkspace"], json!(false));
        assert_eq!(first["createdAt"], json!(300.0));
        assert_eq!(first["mention"], json!("@thread:new"));
        assert_eq!(rows[1]["label"], json!("untitled"));
        assert_eq!(rows[1]["sameWorkspace"], json!(true));

        let rows = session_references("me", "w1", &candidates, "plan");
        let ids: Vec<&str> = rows
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["sessionId"].as_str().unwrap())
            .collect();
        assert_eq!(ids, vec!["new", "old"]);
        assert!(
            session_references("me", "w1", &candidates, "zzz")
                .as_array()
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn session_references_empty_query_is_capped() {
        let candidates: Vec<SessionCandidate> = (0..30)
            .map(|i| candidate(&format!("s{i}"), Some("t"), "w", f64::from(i), false))
            .collect();
        let rows = session_references("x", "w", &candidates, "");
        assert_eq!(rows.as_array().unwrap().len(), SESSION_REFERENCE_LIMIT);
        assert_eq!(rows[0]["sessionId"], json!("s29"));
        let rows = session_references("x", "w", &candidates, "t");
        assert_eq!(rows.as_array().unwrap().len(), 30);
    }
}
