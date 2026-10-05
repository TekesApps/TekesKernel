//! Per-session permission mode.
//!
//! A session has one durable permission mode at
//! `threads/<sessionId>/permission-mode.json` = `{format:1, mode}` (canonical
//! JSON + LF, atomic replace, mode 0600). An absent file means
//! `workspace-write`. A corrupt file is `workspace-write` plus a diagnostic,
//! never a crash. The worker re-reads the file at the start of every tool
//! batch so a change made through the endpoint applies to the next tool call
//! of a running session without a restart.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// File name of the durable mode record inside a session folder.
pub const PERMISSION_MODE_FILE: &str = "permission-mode.json";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum PermissionMode {
    #[serde(rename = "read-only")]
    ReadOnly,
    #[default]
    #[serde(rename = "workspace-write")]
    WorkspaceWrite,
    #[serde(rename = "danger-full-access")]
    DangerFullAccess,
}

impl PermissionMode {
    /// Every mode in the order a Client menu presents them.
    pub const ALL: [PermissionMode; 3] = [
        PermissionMode::ReadOnly,
        PermissionMode::WorkspaceWrite,
        PermissionMode::DangerFullAccess,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            PermissionMode::ReadOnly => "read-only",
            PermissionMode::WorkspaceWrite => "workspace-write",
            PermissionMode::DangerFullAccess => "danger-full-access",
        }
    }

    /// Exact wire id; anything else (case, whitespace) is unknown.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.as_str() == value)
    }
}

impl fmt::Display for PermissionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PermissionModeRecord {
    format: u32,
    mode: PermissionMode,
}

/// The mode a session folder currently declares. `diagnostic` is set when
/// the file exists but is unreadable or malformed; the mode then falls back
/// to the default.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermissionModeRead {
    pub mode: PermissionMode,
    pub diagnostic: Option<String>,
}

#[must_use]
pub fn permission_mode_path(session_folder: &Path) -> PathBuf {
    session_folder.join(PERMISSION_MODE_FILE)
}

/// Read the durable mode. Absent = default; corrupt = default + diagnostic.
#[must_use]
pub fn read_permission_mode(session_folder: &Path) -> PermissionModeRead {
    let path = permission_mode_path(session_folder);
    let fallback = |reason: String| PermissionModeRead {
        mode: PermissionMode::default(),
        diagnostic: Some(format!(
            "{}: {reason}; using {}",
            path.display(),
            PermissionMode::default()
        )),
    };
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return PermissionModeRead {
                mode: PermissionMode::default(),
                diagnostic: None,
            };
        }
        Err(error) => return fallback(error.to_string()),
    };
    match serde_json::from_slice::<PermissionModeRecord>(&bytes) {
        Ok(record) if record.format == 1 => PermissionModeRead {
            mode: record.mode,
            diagnostic: None,
        },
        Ok(record) => fallback(format!("unsupported format {}", record.format)),
        Err(error) => fallback(error.to_string()),
    }
}

/// Durably replace the mode: canonical JSON + LF written to a sibling temp
/// file with mode 0600, then renamed over the record.
pub fn write_permission_mode(session_folder: &Path, mode: PermissionMode) -> io::Result<()> {
    let path = permission_mode_path(session_folder);
    let record = PermissionModeRecord { format: 1, mode };
    let mut bytes = serde_json_canonicalizer::to_vec(&record)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    bytes.push(b'\n');
    let temp = session_folder.join(format!(
        ".{PERMISSION_MODE_FILE}.{}.tmp",
        std::process::id()
    ));
    let result = (|| {
        let mut options = fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut file = options.open(&temp)?;
        io::Write::write_all(&mut file, &bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_file_is_workspace_write_without_diagnostic() {
        let folder = tempfile::tempdir().expect("folder");
        assert_eq!(
            read_permission_mode(folder.path()),
            PermissionModeRead {
                mode: PermissionMode::WorkspaceWrite,
                diagnostic: None
            }
        );
    }

    #[test]
    fn write_then_read_round_trips_every_mode_with_canonical_bytes() {
        let folder = tempfile::tempdir().expect("folder");
        for mode in PermissionMode::ALL {
            write_permission_mode(folder.path(), mode).expect("write");
            let bytes = fs::read(permission_mode_path(folder.path())).expect("bytes");
            assert_eq!(
                bytes,
                format!("{{\"format\":1,\"mode\":\"{}\"}}\n", mode.as_str()).into_bytes()
            );
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                let permissions = fs::metadata(permission_mode_path(folder.path()))
                    .expect("metadata")
                    .permissions();
                assert_eq!(permissions.mode() & 0o777, 0o600);
            }
            assert_eq!(read_permission_mode(folder.path()).mode, mode);
        }
        assert_eq!(
            fs::read_dir(folder.path()).expect("dir").count(),
            1,
            "no temp file is left behind"
        );
    }

    #[test]
    fn corrupt_file_falls_back_with_a_diagnostic() {
        let folder = tempfile::tempdir().expect("folder");
        for bytes in [
            "not json",
            "{\"format\":2,\"mode\":\"read-only\"}\n",
            "{\"format\":1,\"mode\":\"yolo\"}\n",
        ] {
            fs::write(permission_mode_path(folder.path()), bytes).expect("write");
            let read = read_permission_mode(folder.path());
            assert_eq!(read.mode, PermissionMode::WorkspaceWrite);
            assert!(
                read.diagnostic
                    .as_deref()
                    .is_some_and(|text| text.contains("permission-mode.json"))
            );
        }
    }

    #[test]
    fn parse_is_exact() {
        assert_eq!(
            PermissionMode::parse("read-only"),
            Some(PermissionMode::ReadOnly)
        );
        assert_eq!(PermissionMode::parse("Read-Only"), None);
        assert_eq!(PermissionMode::parse(" read-only"), None);
        assert_eq!(
            PermissionMode::ALL.map(PermissionMode::as_str),
            ["read-only", "workspace-write", "danger-full-access"]
        );
    }
}
