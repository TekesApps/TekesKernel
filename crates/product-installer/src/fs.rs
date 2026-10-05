use crate::{Failure, Result, canonical, require};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    os::{
        fd::AsRawFd,
        unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    },
    path::{Path, PathBuf},
};

pub fn absolute(s: &str) -> Result<PathBuf> {
    require(
        s.starts_with('/')
            && !s.contains('\0')
            && (s == "/" || !s.ends_with('/'))
            && !s.contains("//")
            && !s.split('/').any(|c| c == "." || c == ".."),
        "unsafe-path",
    )?;
    Ok(s.into())
}
pub fn uid() -> u32 {
    unsafe { libc::geteuid() }
}
pub fn exists(p: &Path) -> Result<bool> {
    match fs::symlink_metadata(p) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(Failure("io")),
    }
}
pub fn no_symlink_ancestors(p: &Path) -> Result<()> {
    absolute(p.to_str().ok_or(Failure("unsafe-path"))?)?;
    for ancestor in p.ancestors() {
        if let Ok(m) = fs::symlink_metadata(ancestor) {
            require(!m.file_type().is_symlink(), "unsafe-path")?;
        }
    }
    Ok(())
}
pub fn no_symlink_tree(p: &Path) -> Result<()> {
    no_symlink_ancestors(p)?;
    let mut pending = vec![p.to_path_buf()];
    while let Some(p) = pending.pop() {
        let m = fs::symlink_metadata(&p)?;
        require(!m.file_type().is_symlink(), "invalid-artifact")?;
        if m.is_dir() {
            for e in fs::read_dir(p)? {
                pending.push(e?.path());
            }
        }
    }
    Ok(())
}
pub fn regular(p: &Path) -> Result<Vec<u8>> {
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(p)
        .map_err(|_| Failure("invalid-artifact"))?;
    require(f.metadata()?.is_file(), "invalid-artifact")?;
    use std::io::Read;
    let mut b = Vec::new();
    (&f).read_to_end(&mut b)?;
    Ok(b)
}
pub fn mode(p: &Path) -> Result<u32> {
    Ok(fs::symlink_metadata(p)?.mode() & 0o7777)
}
pub fn object(p: &Path) -> Result<(Value, Vec<u8>)> {
    let b = regular(p)?;
    let v: Value = serde_json::from_slice(&b).map_err(|_| Failure("invalid-artifact"))?;
    require(v.is_object() && canonical(&v)? == b, "invalid-artifact")?;
    Ok((v, b))
}
pub fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
pub fn digest(p: &Path, expected: &str) -> Result<()> {
    require(
        expected.len() == 64 && sha(&regular(p)?) == expected,
        "invalid-artifact",
    )
}
pub fn directory(p: &Path) -> Result<()> {
    no_symlink_ancestors(p)?;
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(p)?;
    let m = fs::symlink_metadata(p)?;
    require(m.is_dir() && m.uid() == uid(), "unsafe-path")?;
    fs::set_permissions(p, fs::Permissions::from_mode(0o700))?;
    Ok(())
}
fn full_sync(f: &File) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        require(
            unsafe { libc::fcntl(f.as_raw_fd(), libc::F_FULLFSYNC) } == 0,
            "io",
        )?;
    }
    #[cfg(not(target_os = "macos"))]
    f.sync_all()?;
    Ok(())
}
pub fn sync_dir(p: &Path) -> Result<()> {
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(p)?;
    full_sync(&f)
}
pub fn durable(p: &Path, b: &[u8], mode: u32) -> Result<()> {
    let parent = p.parent().ok_or(Failure("unsafe-path"))?;
    directory(parent)?;
    let mut tmp = tempfile::Builder::new()
        .prefix(".installer-tmp-")
        .tempfile_in(parent)?;
    tmp.as_file()
        .set_permissions(fs::Permissions::from_mode(mode))?;
    tmp.write_all(b)?;
    full_sync(tmp.as_file())?;
    tmp.persist(p).map_err(|_| Failure("io"))?;
    sync_dir(parent)
}
pub fn lock(p: &Path) -> Result<File> {
    directory(p.parent().ok_or(Failure("unsafe-path"))?)?;
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(p)?;
    require(
        unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0,
        "operation-busy",
    )?;
    Ok(f)
}
pub fn lock_available(p: &Path) -> Result<bool> {
    let f = match OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(p)
    {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(true),
        Err(_) => return Err(Failure("io")),
    };
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        return Ok(true);
    }
    if std::io::Error::last_os_error().raw_os_error() == Some(libc::EWOULDBLOCK) {
        Ok(false)
    } else {
        Err(Failure("io"))
    }
}
#[cfg(target_os = "macos")]
pub fn same_file(a: &Path, b: &Path) -> bool {
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(a), Ok(b)) => a.is_file() && b.is_file() && a.dev() == b.dev() && a.ino() == b.ino(),
        _ => false,
    }
}
