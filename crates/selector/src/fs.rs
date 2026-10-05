use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt, symlink};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

use crate::error::{IoContext, SelectorError};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(crate) struct FileLock {
    file: File,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExistingLockState {
    Available,
    Busy,
    Missing,
}

impl FileLock {
    pub(crate) fn try_exclusive(path: &Path) -> Result<Self, SelectorError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(path)
            .selector_io("lock-open")?;
        loop {
            // SAFETY: `file` owns a live descriptor and flock does not retain it.
            let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if result == 0 {
                break;
            }
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            if error
                .raw_os_error()
                .is_some_and(|code| code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                return Err(SelectorError::invalid_state("lock-busy"));
            }
            return Err(SelectorError::io("lock-acquire", error));
        }
        let descriptor = file.metadata().selector_io("lock-stat")?;
        let pathname = fs::metadata(path).selector_io("lock-revalidate")?;
        if descriptor.dev() != pathname.dev() || descriptor.ino() != pathname.ino() {
            return Err(SelectorError::corruption(path.display().to_string()));
        }
        Ok(Self { file })
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        // SAFETY: `file` remains live through this call.
        let _ = unsafe { libc::flock(self.file.as_raw_fd(), libc::LOCK_UN) };
    }
}

pub(crate) fn probe_existing_lock(path: &Path) -> Result<ExistingLockState, SelectorError> {
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(ExistingLockState::Missing);
        }
        Err(error) => return Err(SelectorError::io("root-lock-open", error)),
    };
    let descriptor = file.metadata().selector_io("root-lock-stat")?;
    let pathname = fs::metadata(path).selector_io("root-lock-revalidate")?;
    if !descriptor.is_file()
        || descriptor.dev() != pathname.dev()
        || descriptor.ino() != pathname.ino()
        || descriptor.uid() != unsafe { libc::geteuid() }
        || descriptor.permissions().mode() & 0o7777 != 0o600
    {
        return Err(SelectorError::corruption(path.display().to_string()));
    }
    loop {
        // SAFETY: file owns this live descriptor for the complete probe.
        let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        if result == 0 {
            // SAFETY: this process acquired the advisory lock immediately above.
            let _ = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
            return Ok(ExistingLockState::Available);
        }
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if error
            .raw_os_error()
            .is_some_and(|code| code == libc::EWOULDBLOCK || code == libc::EAGAIN)
        {
            return Ok(ExistingLockState::Busy);
        }
        return Err(SelectorError::io("root-lock-probe", error));
    }
}

pub(crate) fn canonical_line<T: Serialize>(value: &T) -> Result<Vec<u8>, SelectorError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|_| SelectorError::corruption("canonical-json"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn read_canonical<T: DeserializeOwned + Serialize>(
    path: &Path,
) -> Result<T, SelectorError> {
    let bytes = read_regular(path)?;
    let canonical = bytes
        .strip_suffix(b"\n")
        .filter(|body| !body.ends_with(b"\n"))
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let value: T = serde_json::from_slice(canonical)
        .map_err(|_| SelectorError::corruption(path.display().to_string()))?;
    let encoded = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| SelectorError::corruption(path.display().to_string()))?;
    if encoded != canonical {
        return Err(SelectorError::corruption(path.display().to_string()));
    }
    Ok(value)
}

pub(crate) fn read_regular(path: &Path) -> Result<Vec<u8>, SelectorError> {
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .selector_io("file-open")?;
    if !file.metadata().selector_io("file-stat")?.is_file() {
        return Err(SelectorError::corruption(path.display().to_string()));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).selector_io("file-read")?;
    Ok(bytes)
}

pub(crate) fn atomic_json<T: Serialize>(path: &Path, value: &T) -> Result<(), SelectorError> {
    atomic_bytes(path, &canonical_line(value)?, 0o600)
}

pub(crate) fn atomic_bytes(path: &Path, bytes: &[u8], mode: u32) -> Result<(), SelectorError> {
    let parent = path
        .parent()
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let temp = parent.join(format!(
        ".{name}.tmp-{}-{}",
        std::process::id(),
        TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(mode)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&temp)
            .selector_io("temp-create")?;
        file.write_all(bytes).selector_io("temp-write")?;
        full_sync(&file)?;
        drop(file);
        fs::rename(&temp, path).selector_io("atomic-rename")?;
        sync_directory(parent)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

pub(crate) fn atomic_symlink(path: &Path, target: &Path) -> Result<(), SelectorError> {
    let parent = path
        .parent()
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let temp = parent.join(format!(
        ".active.tmp-{}-{}",
        std::process::id(),
        TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    symlink(target, &temp).selector_io("symlink-create")?;
    if let Err(error) = fs::rename(&temp, path).selector_io("symlink-rename") {
        let _ = fs::remove_file(&temp);
        return Err(error);
    }
    sync_directory(parent)
}

pub(crate) fn sync_directory(path: &Path) -> Result<(), SelectorError> {
    let directory = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(path)
        .selector_io("directory-open")?;
    full_sync(&directory).map_err(|error| match error.code {
        crate::ErrorCode::Io => {
            SelectorError::io("directory-sync", io::Error::other(error.to_string()))
        }
        _ => error,
    })
}

pub(crate) fn full_sync(file: &File) -> Result<(), SelectorError> {
    #[cfg(target_os = "macos")]
    loop {
        // SAFETY: `file` owns a live descriptor and F_FULLFSYNC does not retain it.
        let result = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) };
        if result == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(SelectorError::io("file-full-sync", error));
        }
    }
    #[cfg(not(target_os = "macos"))]
    file.sync_all().selector_io("file-full-sync")
}

pub(crate) fn validate_absolute_lexical(path: &Path) -> Result<(), SelectorError> {
    let text = path
        .to_str()
        .ok_or_else(|| SelectorError::usage(path.display().to_string()))?;
    if !path.is_absolute() || text.nfc().collect::<String>() != text {
        return Err(SelectorError::usage(text));
    }
    if text != "/"
        && (text.ends_with('/')
            || text
                .strip_prefix('/')
                .is_none_or(|tail| tail.split('/').any(|component| component.is_empty())))
    {
        return Err(SelectorError::usage(text));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::CurDir | Component::ParentDir | Component::Prefix(_)
        )
    }) {
        return Err(SelectorError::usage(text));
    }
    Ok(())
}

pub(crate) fn validate_id(value: &str) -> Result<(), SelectorError> {
    if value.is_empty()
        || value.len() > 128
        || value.starts_with('.')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(SelectorError::usage(value));
    }
    Ok(())
}

pub(crate) fn validate_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn mode(path: &Path) -> Result<u32, SelectorError> {
    Ok(fs::symlink_metadata(path)
        .selector_io("metadata")?
        .permissions()
        .mode()
        & 0o7777)
}

pub(crate) fn create_private_dir(path: &Path) -> Result<(), SelectorError> {
    fs::DirBuilder::new()
        .recursive(false)
        .mode(0o700)
        .create(path)
        .selector_io("directory-create")
}

pub(crate) fn copy_regular(
    source: &Path,
    destination: &Path,
    mode: u32,
) -> Result<(), SelectorError> {
    let bytes = read_regular(source)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(destination)
        .selector_io("copy-create")?;
    output.write_all(&bytes).selector_io("copy-write")?;
    full_sync(&output)
}

pub(crate) fn relative_active_target(version: &str) -> PathBuf {
    PathBuf::from("../bundles").join(version)
}

pub(crate) fn read_active(path: &Path) -> Result<Option<PathBuf>, SelectorError> {
    match fs::read_link(path) {
        Ok(target) => Ok(Some(target)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
            Err(SelectorError::corruption(path.display().to_string()))
        }
        Err(error) => Err(SelectorError::io("active-read", error)),
    }
}

pub(crate) fn remove_dir_if_exists(path: &Path) -> Result<(), SelectorError> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(SelectorError::io("staging-remove", error)),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    use super::{ExistingLockState, probe_existing_lock};

    #[test]
    fn existing_root_lock_probe_distinguishes_busy_available_and_missing() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join(".root-lock");
        assert_eq!(
            probe_existing_lock(&path).expect("missing probe"),
            ExistingLockState::Missing
        );
        let owner = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .expect("root lock");
        // SAFETY: owner holds this live descriptor throughout the assertion.
        assert_eq!(
            unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        assert_eq!(
            probe_existing_lock(&path).expect("busy probe"),
            ExistingLockState::Busy
        );
        // SAFETY: the test acquired this lock above.
        assert_eq!(unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_UN) }, 0);
        assert_eq!(
            probe_existing_lock(&path).expect("available probe"),
            ExistingLockState::Available
        );
        assert_eq!(
            owner.metadata().expect("metadata").permissions().mode() & 0o7777,
            0o600
        );
    }
}
