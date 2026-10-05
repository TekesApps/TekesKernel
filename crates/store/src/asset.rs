use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use crate::StoreError;
use crate::platform::{SyncPolicy, SystemSync};

static TEMP_ORDINAL: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetRef {
    pub asset: String,
    pub bytes: u64,
}

#[derive(Clone, Debug)]
pub struct AssetStore {
    root: PathBuf,
}

impl AssetStore {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, StoreError> {
        fs::create_dir_all(root.as_ref())?;
        Ok(Self {
            root: root.as_ref().to_path_buf(),
        })
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn publish(&self, content: &[u8]) -> Result<AssetRef, StoreError> {
        self.publish_with_sync(content, &SystemSync)
    }

    pub fn publish_with_sync(
        &self,
        content: &[u8],
        sync: &dyn SyncPolicy,
    ) -> Result<AssetRef, StoreError> {
        let digest = hex_digest(content);
        let name = format!("sha256-{digest}");
        let destination = self.root.join(&name);
        if destination.exists() {
            self.verify_named(&name)?;
            let file = File::open(&destination)?;
            sync.full_sync(&file)?;
            File::open(&self.root)?.sync_all()?;
            return Ok(AssetRef {
                asset: name,
                bytes: content.len() as u64,
            });
        }

        let ordinal = TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let temp_name = format!(".{name}.{}.{ordinal}.tmp", std::process::id());
        let temp = self.root.join(temp_name);
        let result = (|| {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&temp)?;
            file.write_all(content)?;
            sync.full_sync(&file)?;
            drop(file);
            match rename_exclusive(&temp, &destination) {
                Ok(()) => {}
                Err(_error) if destination.exists() => {
                    let _ = fs::remove_file(&temp);
                    self.verify_named(&name)?;
                    return Ok(AssetRef {
                        asset: name.clone(),
                        bytes: content.len() as u64,
                    });
                }
                Err(error) => return Err(StoreError::Io(error)),
            }
            let directory = File::open(&self.root)?;
            directory.sync_all()?;
            Ok(AssetRef {
                asset: name.clone(),
                bytes: content.len() as u64,
            })
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result
    }

    pub fn verify_named(&self, name: &str) -> Result<u64, StoreError> {
        let expected = name.strip_prefix("sha256-").ok_or_else(|| {
            StoreError::Corruption(format!("asset name {name} lacks sha256- prefix"))
        })?;
        let content = fs::read(self.root.join(name))?;
        let actual = hex_digest(&content);
        if actual != expected {
            return Err(StoreError::AssetDigest {
                expected: expected.to_owned(),
                actual,
            });
        }
        Ok(content.len() as u64)
    }

    pub fn read_verified(&self, name: &str) -> Result<Vec<u8>, StoreError> {
        self.verify_named(name)?;
        Ok(fs::read(self.root.join(name))?)
    }
}

#[cfg(target_os = "macos")]
fn rename_exclusive(source: &Path, destination: &Path) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt as _;

    let source = CString::new(source.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "source path contains NUL"))?;
    let destination = CString::new(destination.as_os_str().as_bytes()).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "destination path contains NUL")
    })?;
    loop {
        // SAFETY: both C strings remain live for the call and name valid paths.
        let result = unsafe {
            libc::renameatx_np(
                libc::AT_FDCWD,
                source.as_ptr(),
                libc::AT_FDCWD,
                destination.as_ptr(),
                libc::RENAME_EXCL,
            )
        };
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
fn rename_exclusive(source: &Path, destination: &Path) -> io::Result<()> {
    fs::hard_link(source, destination)?;
    fs::remove_file(source)
}

pub(crate) fn hex_digest(content: &[u8]) -> String {
    let digest = Sha256::digest(content);
    digest.iter().fold(
        String::with_capacity(digest.len() * 2),
        |mut result, byte| {
            write!(result, "{byte:02x}").expect("writing to String is infallible");
            result
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_identical_publication_never_replaces_content() {
        let directory = tempfile::tempdir().expect("asset tempdir");
        let store = AssetStore::new(directory.path()).expect("asset store");
        let references = std::thread::scope(|scope| {
            (0..8)
                .map(|_| {
                    let store = store.clone();
                    scope.spawn(move || store.publish(b"same immutable bytes"))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|thread| thread.join().expect("publisher").expect("publication"))
                .collect::<Vec<_>>()
        });
        assert!(references.windows(2).all(|pair| pair[0] == pair[1]));
        let reference = &references[0];
        assert_eq!(store.verify_named(&reference.asset).expect("digest"), 20);
        assert_eq!(
            fs::read(store.root().join(&reference.asset)).expect("asset bytes"),
            b"same immutable bytes"
        );
        assert_eq!(
            fs::read_dir(store.root()).expect("asset directory").count(),
            1
        );
    }
}
