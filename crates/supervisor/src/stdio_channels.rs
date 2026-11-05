//! Local transport attachment, not local process management. A CLI or App
//! owns each socket and supplies a fresh stdio stream for each connection.
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::mcp_runtime::{StdioConnector, StdioOpen};

pub struct UnixStdioChannels(RwLock<BTreeMap<String, PathBuf>>);

impl UnixStdioChannels {
    pub fn new(channels: BTreeMap<String, PathBuf>) -> Self {
        Self(RwLock::new(channels))
    }
}

impl transport::HostStdioChannelAuthority for UnixStdioChannels {
    fn register(
        &self,
        channel: String,
        path: PathBuf,
    ) -> Result<(), transport::HostStdioChannelError> {
        use transport::HostStdioChannelError;
        if channel.is_empty()
            || channel.len() > 128
            || !channel
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(HostStdioChannelError::Invalid);
        }
        validate_socket(&path).map_err(|_| HostStdioChannelError::Invalid)?;
        // Check and insertion share one lock: two conflicting registrations cannot
        // both acknowledge success. Opening an existing channel only clones its path.
        let mut channels = self.0.write().expect("stdio channel lock poisoned");
        if let Some(existing) = channels.get(&channel) {
            return if existing == &path {
                Ok(())
            } else {
                Err(HostStdioChannelError::Conflict)
            };
        }
        channels.insert(channel, path);
        Ok(())
    }
}

fn validate_socket(path: &Path) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("invalid channel path"))?;
    let metadata = std::fs::symlink_metadata(path)?;
    let parent_metadata = std::fs::symlink_metadata(parent)?;
    // Both the private directory and socket must belong to this host user.
    // SAFETY: geteuid has no preconditions.
    let uid = unsafe { libc::geteuid() };
    if !path.is_absolute()
        || parent.canonicalize()? != parent
        || !parent_metadata.is_dir()
        || parent_metadata.uid() != uid
        || parent_metadata.mode() & 0o077 != 0
        || !metadata.file_type().is_socket()
        || metadata.uid() != uid
        || metadata.mode() & 0o077 != 0
    {
        return Err(io::Error::other("insecure stdio channel"));
    }
    Ok(())
}

impl StdioConnector for UnixStdioChannels {
    fn open<'a>(&'a self, _session: &'a str, channel: &'a str) -> StdioOpen<'a> {
        Box::pin(async move {
            let path = self
                .0
                .read()
                .expect("stdio channel lock poisoned")
                .get(channel)
                .cloned()
                .ok_or_else(|| io::Error::other("unknown stdio channel"))?;
            // The host can remove or replace a socket after registration. Check the
            // filesystem boundary again before every connection; never trust Session paths.
            validate_socket(&path)?;
            let stream = tokio::net::UnixStream::connect(&path).await?.into_std()?;
            Ok((
                File::from(std::os::fd::OwnedFd::from(stream.try_clone()?)),
                File::from(std::os::fd::OwnedFd::from(stream)),
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::{Arc, Barrier};
    use transport::{HostStdioChannelAuthority, HostStdioChannelError};

    fn socket(root: &Path, name: &str) -> (PathBuf, std::os::unix::net::UnixListener) {
        let path = root.join(name);
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        (path, listener)
    }

    #[tokio::test]
    async fn append_preserves_existing_connections_and_rejects_rebinding() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        let (a, _a) = socket(&root, "a.sock");
        let (b, _b) = socket(&root, "b.sock");
        let channels = UnixStdioChannels::new([("a".into(), a.clone())].into());
        let original = channels.open("session-a", "a").await.unwrap();
        assert!(channels.open("session-b", "b").await.is_err());
        channels.register("b".into(), b.clone()).unwrap();
        channels.register("b".into(), b.clone()).unwrap();
        assert_eq!(
            channels.register("a".into(), b),
            Err(HostStdioChannelError::Conflict)
        );
        assert_eq!(channels.0.read().unwrap().get("a"), Some(&a));
        let _new = channels.open("session-b", "b").await.unwrap();
        let _reconnected = channels.open("session-a", "a").await.unwrap();
        assert!(original.0.metadata().is_ok());
        // A post-registration filesystem permission change is not trusted.
        std::fs::set_permissions(&a, std::fs::Permissions::from_mode(0o666)).unwrap();
        assert!(channels.open("session-a", "a").await.is_err());
    }

    #[test]
    fn invalid_attachment_does_not_change_the_map() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        let (valid, _listener) = socket(&root, "valid.sock");
        let channels = UnixStdioChannels::new(BTreeMap::new());
        for name in ["", "../bad", "with space"] {
            assert_eq!(
                channels.register(name.into(), valid.clone()),
                Err(HostStdioChannelError::Invalid)
            );
        }
        let regular = root.join("regular");
        std::fs::write(&regular, b"not a socket").unwrap();
        let link = root.join("link");
        std::os::unix::fs::symlink(&valid, &link).unwrap();
        for path in [
            root.join("absent"),
            regular,
            link,
            PathBuf::from("relative.sock"),
        ] {
            assert_eq!(
                channels.register("bad".into(), path),
                Err(HostStdioChannelError::Invalid)
            );
        }
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(
            channels.register("bad".into(), valid),
            Err(HostStdioChannelError::Invalid)
        );
        assert!(channels.0.read().unwrap().is_empty());
    }

    #[test]
    fn concurrent_conflicting_registrations_have_one_winner() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
        let (a, _a) = socket(&root, "a.sock");
        let (b, _b) = socket(&root, "b.sock");
        let channels = Arc::new(UnixStdioChannels::new(BTreeMap::new()));
        let barrier = Arc::new(Barrier::new(2));
        let handles: Vec<_> = [a, b]
            .into_iter()
            .map(|path| {
                let channels = channels.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    channels.register("same".into(), path)
                })
            })
            .collect();
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|r| **r == Err(HostStdioChannelError::Conflict))
                .count(),
            1
        );
        assert_eq!(channels.0.read().unwrap().len(), 1);
    }
}
