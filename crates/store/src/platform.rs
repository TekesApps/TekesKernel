use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static PROBE_ORDINAL: AtomicU64 = AtomicU64::new(0);

use crate::StoreError;

pub trait SyncPolicy: Send + Sync {
    fn full_sync(&self, file: &File) -> io::Result<()>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemSync;

impl SyncPolicy for SystemSync {
    fn full_sync(&self, file: &File) -> io::Result<()> {
        FullSync::full_sync(file)
    }
}

pub struct FullSync;

impl FullSync {
    pub fn full_sync(file: &File) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        {
            loop {
                // SAFETY: `file` owns a live descriptor for the duration of
                // this call; F_FULLFSYNC neither retains nor closes it.
                let result = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) };
                if result == 0 {
                    return Ok(());
                }
                let error = io::Error::last_os_error();
                if error.kind() != io::ErrorKind::Interrupted {
                    return Err(error);
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        file.sync_all()
    }
}

pub struct DirectoryLock {
    file: File,
    path: PathBuf,
}

impl DirectoryLock {
    pub fn shared(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_SH, false)
    }

    pub fn exclusive(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_EX, false)
    }

    pub fn try_exclusive(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_EX, true)
    }

    fn acquire(
        path: impl AsRef<Path>,
        operation: libc::c_int,
        nonblocking: bool,
    ) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        lock_operation(&file, operation, nonblocking)?;
        revalidate_inode(&file, &path)?;
        Ok(Self { file, path })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub struct RootLock {
    file: File,
    path: PathBuf,
}

pub struct NamedLock {
    file: File,
    path: PathBuf,
}

impl NamedLock {
    pub fn shared(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_SH, false)
    }

    pub fn exclusive(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_EX, false)
    }

    pub fn try_exclusive(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        Self::acquire(path, libc::LOCK_EX, true)
    }

    fn acquire(
        path: impl AsRef<Path>,
        operation: libc::c_int,
        nonblocking: bool,
    ) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&path)?;
        lock_operation(&file, operation, nonblocking)?;
        revalidate_inode(&file, &path)?;
        Ok(Self { file, path })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for NamedLock {
    fn drop(&mut self) {
        let _ = unlock(&self.file);
    }
}

impl RootLock {
    pub fn acquire(root: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = root.as_ref().join(".supervisor.lock");
        let file = open_exclusive_create(&path, true)?;
        Ok(Self { file, path })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for RootLock {
    fn drop(&mut self) {
        let _ = unlock(&self.file);
    }
}

impl Drop for DirectoryLock {
    fn drop(&mut self) {
        let _ = unlock(&self.file);
    }
}

pub(crate) fn try_lock_exclusive(file: &File) -> Result<(), StoreError> {
    lock_operation(file, libc::LOCK_EX, true)
}

pub(crate) fn open_exclusive_create(path: &Path, nonblocking: bool) -> Result<File, StoreError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    lock_operation(&file, libc::LOCK_EX, nonblocking)?;
    revalidate_inode(&file, path)?;
    Ok(file)
}

fn lock_operation(
    file: &File,
    operation: libc::c_int,
    nonblocking: bool,
) -> Result<(), StoreError> {
    loop {
        // SAFETY: `file` owns a live descriptor and flock only changes the
        // advisory lock associated with its open-file description.
        let flags = if nonblocking {
            operation | libc::LOCK_NB
        } else {
            operation
        };
        let result = unsafe { libc::flock(file.as_raw_fd(), flags) };
        if result == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        match error.raw_os_error() {
            Some(code) if nonblocking && (code == libc::EWOULDBLOCK || code == libc::EAGAIN) => {
                return Err(StoreError::Busy);
            }
            _ if error.kind() == io::ErrorKind::Interrupted => continue,
            _ => return Err(StoreError::Io(error)),
        }
    }
}

fn revalidate_inode(file: &File, path: &Path) -> Result<(), StoreError> {
    let descriptor = file.metadata()?;
    let pathname = std::fs::metadata(path)?;
    if descriptor.dev() != pathname.dev() || descriptor.ino() != pathname.ino() {
        return Err(StoreError::ReplacedWhileLocking);
    }
    Ok(())
}

pub fn probe_local_filesystem(root: impl AsRef<Path>) -> Result<(), StoreError> {
    let root = root.as_ref();
    if root
        .to_string_lossy()
        .contains("/Library/Mobile Documents/")
    {
        return Err(StoreError::UnsupportedFilesystem(
            "iCloud-synchronized storage is not supported".to_owned(),
        ));
    }
    std::fs::create_dir_all(root)?;
    reject_known_remote_filesystem(root)?;
    let path = root.join(format!(
        ".kernel-fs-probe-{}-{}",
        std::process::id(),
        PROBE_ORDINAL.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        let mut first = OpenOptions::new()
            .read(true)
            .append(true)
            .create_new(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&path)?;
        use std::io::Write as _;
        first.write_all(b"probe\n")?;
        FullSync::full_sync(&first)?;
        try_lock_exclusive(&first)?;
        let second = OpenOptions::new()
            .read(true)
            .append(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&path)?;
        if !matches!(try_lock_exclusive(&second), Err(StoreError::Busy)) {
            return Err(StoreError::UnsupportedFilesystem(
                "flock contention probe did not report busy".to_owned(),
            ));
        }
        Ok(())
    })();
    let _ = std::fs::remove_file(&path);
    result
}

#[cfg(target_os = "macos")]
fn reject_known_remote_filesystem(root: &Path) -> Result<(), StoreError> {
    let path = CString::new(root.as_os_str().as_bytes())
        .map_err(|_| StoreError::UnsupportedFilesystem("storage path contains NUL".to_owned()))?;
    // SAFETY: `stat` is initialized by statfs on success and `path` is a
    // NUL-terminated pathname valid for the duration of the call.
    let mut stat = unsafe { std::mem::zeroed::<libc::statfs>() };
    // SAFETY: pointers refer to live values described above.
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return Err(StoreError::Io(io::Error::last_os_error()));
    }
    let bytes = stat
        .f_fstypename
        .iter()
        .map(|value| *value as u8)
        .take_while(|value| *value != 0)
        .collect::<Vec<_>>();
    let name = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
    if !matches!(name.as_str(), "apfs" | "hfs") {
        return Err(StoreError::UnsupportedFilesystem(format!(
            "filesystem {name} is not in the local Darwin allowlist"
        )));
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn reject_known_remote_filesystem(_root: &Path) -> Result<(), StoreError> {
    let _ = CString::new(Vec::<u8>::new());
    Ok(())
}

pub(crate) fn unlock(file: &File) -> io::Result<()> {
    loop {
        // SAFETY: `file` owns a live descriptor and flock only changes its
        // advisory lock state.
        let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
        if result == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}

#[cfg(test)]
mod probe_tests {
    #[test]
    fn concurrent_open_probes_do_not_remove_each_others_files() {
        let root = tempfile::tempdir().expect("root");
        let barrier = std::sync::Barrier::new(8);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let root = root.path();
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    super::probe_local_filesystem(root).expect("independent probe");
                });
            }
        });
        assert_eq!(std::fs::read_dir(root.path()).expect("root").count(), 0);
    }
}
