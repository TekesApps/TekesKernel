//! Local transport attachment, not local process management. A CLI or App
//! owns each socket and supplies a fresh stdio stream for each connection.
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::PathBuf;

use crate::mcp_runtime::{StdioConnector, StdioOpen};

pub struct UnixStdioChannels(pub BTreeMap<String, PathBuf>);

impl StdioConnector for UnixStdioChannels {
    fn open<'a>(&'a self, _session: &'a str, channel: &'a str) -> StdioOpen<'a> {
        Box::pin(async move {
            let path = self
                .0
                .get(channel)
                .ok_or_else(|| io::Error::other("unknown stdio channel"))?;
            let parent = path
                .parent()
                .ok_or_else(|| io::Error::other("invalid channel path"))?;
            let metadata = std::fs::symlink_metadata(path)?;
            let parent_metadata = std::fs::symlink_metadata(parent)?;
            // Both the private directory and socket must belong to this host user.
            // No pathname from a Session is interpreted as a socket address.
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
            let stream = tokio::net::UnixStream::connect(path).await?.into_std()?;
            Ok((
                File::from(std::os::fd::OwnedFd::from(stream.try_clone()?)),
                File::from(std::os::fd::OwnedFd::from(stream)),
            ))
        })
    }
}
