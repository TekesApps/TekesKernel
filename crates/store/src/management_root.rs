use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The root-scoped endpoint management directory (rpc exact-retry carriers,
/// workspace sidecars, crash-recoverable operations). A directory created
/// under the earlier `endpoint-management-v2` name is renamed in place the
/// first time the root is opened; both layouts populated at once is corruption.
pub const ENDPOINT_MANAGEMENT_DIR: &str = "endpoint-management";
const LEGACY_ENDPOINT_MANAGEMENT_DIR: &str = "endpoint-management-v2";

/// The per-thread session settings document.
pub const SESSION_SETTINGS_FILE: &str = "session-settings.json";
const LEGACY_SESSION_SETTINGS_FILE: &str = "session-settings-v1.json";

/// Renames a durable file or directory published under a superseded name to
/// its current name the first time its parent is opened, and returns the
/// current path. A caller may hold the parent's lock but never needs to: the
/// rename is atomic and idempotent. Two populated layouts fail closed rather
/// than silently choosing one authority.
pub fn retire_legacy_name(parent: &Path, current: &str, legacy: &str) -> io::Result<PathBuf> {
    let current_path = parent.join(current);
    let legacy_path = parent.join(legacy);
    if fs::symlink_metadata(&legacy_path).is_ok() {
        match fs::symlink_metadata(&current_path) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fs::rename(&legacy_path, &current_path)?;
                fs::File::open(parent)?.sync_all()?;
            }
            Ok(metadata) if metadata.is_dir() && fs::read_dir(&current_path)?.next().is_none() => {
                fs::remove_dir(&current_path)?;
                fs::rename(&legacy_path, &current_path)?;
                fs::File::open(parent)?.sync_all()?;
            }
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("both {current} and {legacy} are populated"),
                ));
            }
            Err(error) => return Err(error),
        }
    }
    Ok(current_path)
}

pub fn endpoint_management_root(root: &Path) -> io::Result<PathBuf> {
    retire_legacy_name(
        root,
        ENDPOINT_MANAGEMENT_DIR,
        LEGACY_ENDPOINT_MANAGEMENT_DIR,
    )
}

/// The thread's session settings path, retiring a document published under the
/// earlier `session-settings-v1.json` name.
pub fn session_settings_path(thread_folder: &Path) -> io::Result<PathBuf> {
    retire_legacy_name(
        thread_folder,
        SESSION_SETTINGS_FILE,
        LEGACY_SESSION_SETTINGS_FILE,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_directory_is_renamed_once() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(LEGACY_ENDPOINT_MANAGEMENT_DIR).join("rpc")).unwrap();
        let current = endpoint_management_root(root.path()).unwrap();
        assert_eq!(current, root.path().join(ENDPOINT_MANAGEMENT_DIR));
        assert!(current.join("rpc").is_dir());
        assert!(!root.path().join(LEGACY_ENDPOINT_MANAGEMENT_DIR).exists());
        assert_eq!(endpoint_management_root(root.path()).unwrap(), current);
        fs::create_dir(root.path().join(LEGACY_ENDPOINT_MANAGEMENT_DIR)).unwrap();
        assert!(
            endpoint_management_root(root.path()).is_err(),
            "two populated layouts fail closed"
        );
    }

    #[test]
    fn a_legacy_session_settings_document_is_renamed_on_first_use() {
        let folder = tempfile::tempdir().unwrap();
        fs::write(
            folder.path().join(LEGACY_SESSION_SETTINGS_FILE),
            b"{\"format\":1}\n",
        )
        .unwrap();
        let path = session_settings_path(folder.path()).unwrap();
        assert_eq!(path, folder.path().join(SESSION_SETTINGS_FILE));
        assert_eq!(fs::read(&path).unwrap(), b"{\"format\":1}\n");
        assert!(!folder.path().join(LEGACY_SESSION_SETTINGS_FILE).exists());
        assert_eq!(session_settings_path(folder.path()).unwrap(), path);

        // Two populated documents are corruption, not a silent winner.
        fs::write(folder.path().join(LEGACY_SESSION_SETTINGS_FILE), b"{}\n").unwrap();
        assert!(session_settings_path(folder.path()).is_err());
    }
}
