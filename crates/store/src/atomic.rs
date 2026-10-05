use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::StoreError;
use crate::platform::{SyncPolicy, SystemSync};

static TEMP_ORDINAL: AtomicU64 = AtomicU64::new(0);

pub struct AtomicPublisher;

impl AtomicPublisher {
    pub fn replace(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), StoreError> {
        Self::replace_with_sync(path, bytes, &SystemSync)
    }

    pub fn replace_with_sync(
        path: impl AsRef<Path>,
        bytes: &[u8],
        sync: &dyn SyncPolicy,
    ) -> Result<(), StoreError> {
        let path = path.as_ref();
        let parent = path.parent().ok_or_else(|| {
            StoreError::Corruption("atomic publication path has no parent".to_owned())
        })?;
        fs::create_dir_all(parent)?;
        let file_name = path
            .file_name()
            .unwrap_or_else(|| OsStr::new("publication"));
        let ordinal = TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let temp = parent.join(format!(
            ".{}.{}.{ordinal}.tmp",
            file_name.to_string_lossy(),
            std::process::id()
        ));
        let result = (|| {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
                .open(&temp)?;
            file.write_all(bytes)?;
            sync.full_sync(&file)?;
            drop(file);
            fs::rename(&temp, path)?;
            File::open(parent)?.sync_all()?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result
    }
}
