//! user-documents: Client-facing durable metadata that Kernel owns on behalf of the
//! user but which is not ledger state.
//!
//! Two document families live here:
//!
//! * Message feedback (`feedback.*`): one file per session under `root/feedback/`,
//!   compare-and-swap versioned with a per-session monotonic counter.
//! * User instruction settings (`settings.*`): the user-level `root/settings.json`
//!   (closed schema from `spec/instruction-snapshot.md`) with a revision sidecar
//!   `root/settings.revision` that also detects out-of-band edits.
//!
//! Every file is RFC 8785 canonical JSON followed by one LF and written atomically
//! (temp file + rename). Functions take the Kernel state root explicitly.

use std::path::{Path, PathBuf};

pub mod feedback;
pub mod settings;

/// Client-extension methods served by this crate.
pub const METHODS: &[&str] = &[
    "feedback.list",
    "feedback.put",
    "feedback.delete",
    "settings.describe",
    "settings.mutate",
    "settings.update",
    "settings.document",
];

/// A request the crate refuses. `code` is the wire error code.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
    pub details: serde_json::Value,
}

impl Failure {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: serde_json::Value::Object(Default::default()),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("bad-request", message)
    }

    pub fn io(context: &str, error: &std::io::Error) -> Self {
        Self::new("io", format!("{context}: {error}"))
    }
}

/// Validate a payload's closed shape without touching disk.
pub fn validate(method: &str, payload: &serde_json::Value) -> Result<(), String> {
    match method {
        "feedback.list" | "feedback.put" | "feedback.delete" => feedback::validate(method, payload),
        "settings.describe" | "settings.mutate" | "settings.update" | "settings.document" => {
            settings::validate(method, payload)
        }
        other => Err(format!("unknown method {other}")),
    }
}

/// Validate and execute one method against the Kernel state root.
pub fn execute(
    root: &Path,
    method: &str,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, Failure> {
    validate(method, payload).map_err(Failure::bad_request)?;
    match method {
        "feedback.list" => feedback::list(root, payload),
        "feedback.put" => feedback::put(root, payload),
        "feedback.delete" => feedback::delete(root, payload),
        "settings.describe" => settings::describe(root),
        "settings.mutate" => settings::mutate(root, payload),
        "settings.update" => settings::update(root, payload),
        "settings.document" => settings::document(root),
        other => Err(Failure::bad_request(format!("unknown method {other}"))),
    }
}

pub(crate) fn canonical_bytes(value: &serde_json::Value) -> Result<Vec<u8>, Failure> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| Failure::new("io", format!("canonical JSON failed: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Write `bytes` to `path` via a sibling temp file and rename.
pub(crate) fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), Failure> {
    let parent = path
        .parent()
        .ok_or_else(|| Failure::new("io", "path has no parent"))?;
    std::fs::create_dir_all(parent).map_err(|error| Failure::io("create directory", &error))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("document");
    let temp: PathBuf = parent.join(format!(".{name}.{}.tmp", uuid::Uuid::new_v4()));
    let result = (|| {
        std::fs::write(&temp, bytes)?;
        std::fs::rename(&temp, path)
    })();
    if let Err(error) = result {
        let _ = std::fs::remove_file(&temp);
        return Err(Failure::io("write document", &error));
    }
    Ok(())
}

/// Reject any object key outside `allowed`.
pub(crate) fn closed_object<'a>(
    payload: &'a serde_json::Value,
    allowed: &[&str],
) -> Result<&'a serde_json::Map<String, serde_json::Value>, String> {
    let object = payload.as_object().ok_or("payload must be an object")?;
    for key in object.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field {key}"));
        }
    }
    Ok(object)
}

pub(crate) fn required_str<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<&'a str, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{key} is required"))?
        .as_str()
        .ok_or_else(|| format!("{key} must be a string"))
}

pub(crate) fn now_milliseconds() -> u64 {
    u64::try_from(chrono::Utc::now().timestamp_millis()).unwrap_or(0)
}
