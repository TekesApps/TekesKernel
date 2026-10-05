//! Revision-checked saves, serialized across helper processes and path aliases.
use crate::{Failure, fail, git::MutationAuthority, locate, open_scoped};
use base64::Engine as _;
use rustix::fs::{AtFlags, FlockOperation, Mode, OFlags, flock, openat, renameat, unlinkat};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;

const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Request {
    pub workspace_id: String,
    pub path: String,
    pub content: String,
    pub expected_revision: String,
}
fn revision(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read(file: &File) -> Result<Vec<u8>, Failure> {
    let mut bytes = Vec::new();
    file.take(MAX_BYTES as u64 + 1).read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn execute(
    root: &Path,
    request: Request,
    authority: Option<&MutationAuthority>,
) -> Result<Value, Failure> {
    let authority = authority
        .filter(|a| a.allow_file_writes)
        .ok_or_else(|| fail("permission-denied", "File writes are not authorized"))?;
    if request.workspace_id.is_empty()
        || request.path.is_empty()
        || request.content.len() > MAX_BYTES.div_ceil(3) * 4
        || request.expected_revision.len() != 64
        || !request
            .expected_revision
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(fail(
            "invalid-request",
            "Expected bounded content and SHA-256 revision",
        ));
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&request.content)
        .map_err(|_| fail("invalid-request", "Expected canonical base64"))?;
    if bytes.len() > MAX_BYTES
        || base64::engine::general_purpose::STANDARD.encode(&bytes) != request.content
    {
        return Err(fail("invalid-request", "Expected bounded canonical base64"));
    }
    let (root, target) = locate(root, &request.path)?;
    let lock_directory = authority.state_root.join("file-locks");
    fs::create_dir_all(&lock_directory)?;
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
        .open(lock_directory.join(revision(target.as_os_str().as_encoded_bytes())))?;
    flock(&lock, FlockOperation::LockExclusive).map_err(std::io::Error::from)?;
    let parent = target
        .parent()
        .ok_or_else(|| fail("not-file", "Expected a file"))?;
    let directory = open_scoped(&root, parent, true)?;
    let name = target
        .file_name()
        .ok_or_else(|| fail("not-file", "Expected a file"))?;
    let open_target = || -> Result<File, Failure> {
        Ok(File::from(
            openat(
                &directory,
                name,
                OFlags::RDWR | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(std::io::Error::from)?,
        ))
    };
    let original = open_target()?;
    let before = original.metadata()?;
    if !before.is_file() {
        return Err(fail("not-file", "Expected a regular file"));
    }
    if before.len() > MAX_BYTES as u64 || revision(&read(&original)?) != request.expected_revision {
        return Err(fail(
            "revision-conflict",
            "File changed since it was opened",
        ));
    }
    // PID plus an exclusively-created suffix; the directory descriptor pins the
    // destination and prevents a later path symlink from redirecting the save.
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = format!(".tekes-save-{}-{nonce}", std::process::id());
    let mut output = File::from(
        openat(
            &directory,
            temporary.as_str(),
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_raw_mode((before.mode() & 0o777) as _),
        )
        .map_err(std::io::Error::from)?,
    );
    let result = (|| {
        output.write_all(&bytes)?;
        output.set_permissions(std::os::unix::fs::PermissionsExt::from_mode(
            before.mode() & 0o777,
        ))?;
        output.sync_all()?;
        let current = open_target()?;
        let metadata = current.metadata()?;
        if !metadata.is_file()
            || metadata.ino() != before.ino()
            || metadata.dev() != before.dev()
            || metadata.len() > MAX_BYTES as u64
            || revision(&read(&current)?) != request.expected_revision
            || locate(&root, &request.path)?.1 != target
        {
            return Err(fail("revision-conflict", "File changed during save"));
        }
        renameat(&directory, temporary.as_str(), &directory, name).map_err(std::io::Error::from)?;
        directory.sync_all().map_err(|_| {
            fail(
                "outcome-unknown",
                "Replacement completed but directory sync failed",
            )
        })?;
        Ok(
            json!({"workspaceId":request.workspace_id,"path":request.path,"revision":revision(&bytes),"size":bytes.len()}),
        )
    })();
    let _ = unlinkat(&directory, temporary.as_str(), AtFlags::empty());
    result
}
