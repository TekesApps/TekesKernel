use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Exact-retry state for the authenticated endpoint. No management registry.
pub const ENDPOINT_RPC_DIR: &str = "endpoint-rpc";
pub const SESSION_SETTINGS_FILE: &str = "session-settings.json";

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

pub fn endpoint_rpc_root(root: &Path) -> io::Result<PathBuf> {
    Ok(root.join(ENDPOINT_RPC_DIR))
}

pub fn session_settings_path(thread_folder: &Path) -> io::Result<PathBuf> {
    Ok(thread_folder.join(SESSION_SETTINGS_FILE))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retired_state_is_never_imported_or_modified() {
        let root = tempfile::tempdir().unwrap();
        let old = root.path().join("endpoint-management-v2");
        fs::create_dir(&old).unwrap();
        fs::write(root.path().join("session-settings-v1.json"), b"old").unwrap();
        assert_eq!(
            endpoint_rpc_root(root.path()).unwrap(),
            root.path().join("endpoint-rpc")
        );
        assert_eq!(
            session_settings_path(root.path()).unwrap(),
            root.path().join("session-settings.json")
        );
        assert!(old.is_dir());
        assert_eq!(
            fs::read(root.path().join("session-settings-v1.json")).unwrap(),
            b"old"
        );
        assert!(!root.path().join("session-settings.json").exists());
    }
}
