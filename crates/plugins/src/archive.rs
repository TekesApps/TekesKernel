use std::collections::BTreeSet;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rustix::fs::{Dir, FileType, Mode, OFlags, fchmod, fstat, mkdirat, open, openat};
use serde::{Deserialize, Serialize};
use store::FullSync;

use crate::{PLUGIN_ARCHIVE_EXTENSION, PluginError};

const MAX_ARCHIVE_BYTES: u64 = 384 * 1_024 * 1_024;
const MAX_EXPANDED_BYTES: usize = 256 * 1_024 * 1_024;
const MAX_ENTRY_BYTES: usize = 128 * 1_024 * 1_024;
const MAX_ENTRIES: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PluginSource {
    Directory(PathBuf),
    Archive(PathBuf),
}

impl PluginSource {
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self, PluginError> {
        let path = path.into();
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| PluginError::Storage(error.to_string()))?;
        if metadata.file_type().is_dir() {
            Ok(Self::Directory(path))
        } else if metadata.file_type().is_file()
            && path
                .extension()
                .is_some_and(|value| value.eq_ignore_ascii_case(PLUGIN_ARCHIVE_EXTENSION))
        {
            Ok(Self::Archive(path))
        } else {
            Err(PluginError::InvalidArchive(
                "source must be a directory or .tekesplugin file".to_owned(),
            ))
        }
    }

    pub(crate) fn stage(&self, destination: &Path) -> Result<(), PluginError> {
        match self {
            Self::Directory(path) => copy_tree(path, destination),
            Self::Archive(path) => PluginArchive::expand(path, destination),
        }
    }

    pub(crate) fn description(&self) -> String {
        match self {
            Self::Directory(path) | Self::Archive(path) => path.display().to_string(),
        }
    }
}

pub struct PluginArchive;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Envelope {
    archive_version: u32,
    entries: Vec<Entry>,
}

#[derive(Deserialize, Serialize)]
struct Entry {
    path: String,
    kind: EntryKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    executable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum EntryKind {
    Directory,
    File,
}

impl PluginArchive {
    pub fn inspect(path: impl AsRef<Path>) -> Result<Vec<(String, bool)>, PluginError> {
        let envelope = read_envelope(path.as_ref())?;
        validate_entries(&envelope)?;
        Ok(envelope
            .entries
            .into_iter()
            .map(|entry| (entry.path, matches!(entry.kind, EntryKind::File)))
            .collect())
    }

    pub fn expand(
        path: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> Result<(), PluginError> {
        expand_archive_with_hook(path.as_ref(), destination.as_ref(), &NoopStageHook)
    }
}

trait ArchiveExpandHook {
    fn before_open_directory(&self, relative: &Path);
}

fn expand_archive_with_hook(
    path: &Path,
    destination: &Path,
    hook: &dyn ArchiveExpandHook,
) -> Result<(), PluginError> {
    let envelope = read_envelope(path)?;
    validate_entries(&envelope)?;
    fs::create_dir(destination).map_err(|error| PluginError::Storage(error.to_string()))?;
    let destination_fd = File::from(
        open(
            destination,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|error| PluginError::Storage(error.to_string()))?,
    );
    fchmod(&destination_fd, Mode::RWXU).map_err(|error| PluginError::Storage(error.to_string()))?;
    full_sync(&destination_fd)?;
    if let Some(parent) = destination.parent() {
        full_sync_path(parent)?;
    }
    let mut entries = envelope.entries;
    entries.sort_by(|left, right| {
        left.path
            .split('/')
            .count()
            .cmp(&right.path.split('/').count())
            .then_with(|| {
                matches!(right.kind, EntryKind::Directory)
                    .cmp(&matches!(left.kind, EntryKind::Directory))
            })
            .then_with(|| left.path.cmp(&right.path))
    });
    for entry in entries {
        let components = archive_components(&entry.path)?;
        match entry.kind {
            EntryKind::Directory => {
                let directory = open_archive_directory_chain(&destination_fd, &components, hook)?
                    .ok_or_else(|| {
                    PluginError::InvalidArchive("empty archive directory path".to_owned())
                })?;
                full_sync(&directory)?;
            }
            EntryKind::File => {
                let data = STANDARD
                    .decode(entry.data.as_deref().unwrap_or_default())
                    .map_err(|error| PluginError::InvalidArchive(error.to_string()))?;
                let (name, parents) = components.split_last().ok_or_else(|| {
                    PluginError::InvalidArchive("empty archive file path".to_owned())
                })?;
                let parent = open_archive_directory_chain(&destination_fd, parents, hook)?;
                let parent_fd = parent.as_ref().unwrap_or(&destination_fd);
                let mode = if entry.executable == Some(true) {
                    Mode::RWXU
                } else {
                    Mode::RUSR | Mode::WUSR
                };
                let mut file = File::from(
                    openat(
                        parent_fd,
                        name,
                        OFlags::WRONLY
                            | OFlags::CREATE
                            | OFlags::EXCL
                            | OFlags::NOFOLLOW
                            | OFlags::CLOEXEC,
                        mode,
                    )
                    .map_err(|error| PluginError::Storage(error.to_string()))?,
                );
                fchmod(&file, mode).map_err(|error| PluginError::Storage(error.to_string()))?;
                file.write_all(&data)
                    .map_err(|error| PluginError::Storage(error.to_string()))?;
                full_sync(&file)?;
                full_sync(parent_fd)?;
            }
        }
    }
    full_sync(&destination_fd)?;
    if let Some(parent) = destination.parent() {
        full_sync_path(parent)?;
    }
    Ok(())
}

fn read_envelope(path: &Path) -> Result<Envelope, PluginError> {
    let descriptor = open(
        path,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
        Mode::empty(),
    )
    .map_err(|error| PluginError::InvalidArchive(format!("archive open failed: {error}")))?;
    let stat = fstat(&descriptor)
        .map_err(|error| PluginError::InvalidArchive(format!("archive stat failed: {error}")))?;
    let size = u64::try_from(stat.st_size).unwrap_or(u64::MAX);
    if FileType::from_raw_mode(stat.st_mode as _) != FileType::RegularFile
        || !(1..=MAX_ARCHIVE_BYTES).contains(&size)
    {
        return Err(PluginError::InvalidArchive(
            "archive size or file kind is invalid".to_owned(),
        ));
    }
    let mut bytes = Vec::with_capacity(usize::try_from(size).unwrap_or(0));
    File::from(descriptor)
        .take(MAX_ARCHIVE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| PluginError::Storage(error.to_string()))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) != size {
        return Err(PluginError::InvalidArchive(
            "archive changed while reading".to_owned(),
        ));
    }
    serde_json::from_slice(&bytes).map_err(|error| PluginError::InvalidArchive(error.to_string()))
}

fn archive_components(path: &str) -> Result<Vec<CString>, PluginError> {
    path.split('/')
        .map(|component| {
            CString::new(component.as_bytes()).map_err(|_| {
                PluginError::InvalidArchive(format!("archive entry contains NUL: {path}"))
            })
        })
        .collect()
}

fn open_archive_directory_chain(
    root: &File,
    components: &[CString],
    hook: &dyn ArchiveExpandHook,
) -> Result<Option<File>, PluginError> {
    let mut current = None::<File>;
    let mut relative = PathBuf::new();
    for component in components {
        let parent = current.as_ref().unwrap_or(root);
        match mkdirat(parent, component, Mode::RWXU) {
            Ok(()) => full_sync(parent)?,
            Err(error) if error == rustix::io::Errno::EXIST => {}
            Err(error) => return Err(PluginError::Storage(error.to_string())),
        }
        relative.push(std::ffi::OsStr::from_bytes(component.as_bytes()));
        hook.before_open_directory(&relative);
        let directory = File::from(
            openat(
                parent,
                component,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(|error| {
                PluginError::InvalidArchive(format!(
                    "archive destination directory changed or became a link: {error}"
                ))
            })?,
        );
        fchmod(&directory, Mode::RWXU).map_err(|error| PluginError::Storage(error.to_string()))?;
        full_sync(&directory)?;
        current = Some(directory);
    }
    Ok(current)
}

fn validate_entries(envelope: &Envelope) -> Result<(), PluginError> {
    if envelope.archive_version != 1
        || envelope.entries.is_empty()
        || envelope.entries.len() > MAX_ENTRIES
    {
        return Err(PluginError::InvalidArchive(
            "unsupported version or entry count".to_owned(),
        ));
    }
    let mut paths = BTreeSet::new();
    let mut expanded = 0usize;
    for entry in &envelope.entries {
        if !crate::model::safe_relative(&entry.path) || !paths.insert(entry.path.clone()) {
            return Err(PluginError::InvalidArchive(format!(
                "unsafe or duplicate entry {}",
                entry.path
            )));
        }
        match entry.kind {
            EntryKind::Directory if entry.data.is_some() || entry.executable.is_some() => {
                return Err(PluginError::InvalidArchive(format!(
                    "directory {} carries file metadata",
                    entry.path
                )));
            }
            EntryKind::File => {
                let data = entry.data.as_deref().ok_or_else(|| {
                    PluginError::InvalidArchive(format!("file {} has no data", entry.path))
                })?;
                let decoded = STANDARD
                    .decode(data)
                    .map_err(|error| PluginError::InvalidArchive(error.to_string()))?;
                if decoded.len() > MAX_ENTRY_BYTES {
                    return Err(PluginError::InvalidArchive(format!(
                        "entry {} is too large",
                        entry.path
                    )));
                }
                expanded = expanded.saturating_add(decoded.len());
                if expanded > MAX_EXPANDED_BYTES {
                    return Err(PluginError::InvalidArchive(
                        "expanded archive is too large".to_owned(),
                    ));
                }
            }
            EntryKind::Directory => {}
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), PluginError> {
    copy_tree_with_hook(source, destination, &NoopStageHook)
}

trait DirectoryStageHook {
    fn before_open(&self, relative: &Path);
}

impl<F: Fn(&Path)> DirectoryStageHook for F {
    fn before_open(&self, relative: &Path) {
        self(relative);
    }
}

impl<F: Fn(&Path)> ArchiveExpandHook for F {
    fn before_open_directory(&self, relative: &Path) {
        self(relative);
    }
}

struct NoopStageHook;

impl DirectoryStageHook for NoopStageHook {
    fn before_open(&self, _: &Path) {}
}

impl ArchiveExpandHook for NoopStageHook {
    fn before_open_directory(&self, _: &Path) {}
}

fn copy_tree_with_hook(
    source: &Path,
    destination: &Path,
    hook: &dyn DirectoryStageHook,
) -> Result<(), PluginError> {
    let source = File::from(
        open(
            source,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|error| {
            PluginError::InvalidArchive(format!(
                "package source is not a stable directory: {error}"
            ))
        })?,
    );
    fs::create_dir(destination).map_err(|error| PluginError::Storage(error.to_string()))?;
    let destination_fd = File::from(
        open(
            destination,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|error| PluginError::Storage(error.to_string()))?,
    );
    fchmod(&destination_fd, Mode::RWXU).map_err(|error| PluginError::Storage(error.to_string()))?;
    full_sync(&destination_fd)?;
    if let Some(parent) = destination.parent() {
        full_sync_path(parent)?;
    }
    let mut limits = StageLimits::default();
    copy_directory_fd(&source, &destination_fd, Path::new(""), hook, &mut limits)?;
    full_sync(&destination_fd)?;
    if let Some(parent) = destination.parent() {
        full_sync_path(parent)?;
    }
    Ok(())
}

#[derive(Default)]
struct StageLimits {
    entries: usize,
    bytes: usize,
}

fn copy_directory_fd(
    source_fd: &File,
    destination_fd: &File,
    relative_root: &Path,
    hook: &dyn DirectoryStageHook,
    limits: &mut StageLimits,
) -> Result<(), PluginError> {
    for name in directory_names(source_fd)? {
        limits.entries = limits.entries.saturating_add(1);
        if limits.entries > MAX_ENTRIES {
            return Err(PluginError::InvalidArchive(
                "directory package has too many entries".to_owned(),
            ));
        }
        let name_text = name.to_str().map_err(|_| {
            PluginError::InvalidArchive("directory entry name is not UTF-8".to_owned())
        })?;
        let relative = relative_root.join(name_text);
        hook.before_open(&relative);
        let source = openat_source(source_fd, &name)?;
        let stat = descriptor_stat(&source)?;
        match FileType::from_raw_mode(stat.st_mode as _) {
            FileType::Directory => {
                create_directory_at(destination_fd, &name)?;
                let destination = openat_destination_directory(destination_fd, &name)?;
                copy_directory_fd(&source, &destination, &relative, hook, limits)?;
                full_sync(&destination)?;
            }
            FileType::RegularFile => {
                let size = usize::try_from(stat.st_size).map_err(|_| {
                    PluginError::InvalidArchive(format!(
                        "directory package entry is too large: {}",
                        relative.display()
                    ))
                })?;
                if size > MAX_ENTRY_BYTES || limits.bytes.saturating_add(size) > MAX_EXPANDED_BYTES
                {
                    return Err(PluginError::InvalidArchive(format!(
                        "directory package entry is too large: {}",
                        relative.display()
                    )));
                }
                limits.bytes += size;
                let executable = stat.st_mode & 0o111 != 0;
                let mut destination = openat_destination_file(destination_fd, &name, executable)?;
                let mut bounded =
                    source.take(u64::try_from(MAX_ENTRY_BYTES).unwrap_or(u64::MAX) + 1);
                let copied = io::copy(&mut bounded, &mut destination)
                    .map_err(|error| PluginError::Storage(error.to_string()))?;
                if copied != u64::try_from(size).unwrap_or(u64::MAX) {
                    return Err(PluginError::InvalidArchive(format!(
                        "directory package entry changed while staging: {}",
                        relative.display()
                    )));
                }
                full_sync(&destination)?;
                full_sync(destination_fd)?;
            }
            _ => {
                return Err(PluginError::InvalidArchive(format!(
                    "unsupported package entry {}",
                    relative.display()
                )));
            }
        }
    }
    Ok(())
}

fn directory_names(directory_fd: &File) -> Result<Vec<CString>, PluginError> {
    let mut stream =
        Dir::read_from(directory_fd).map_err(|error| PluginError::Storage(error.to_string()))?;
    let mut names = Vec::new();
    for entry in &mut stream {
        let entry = entry.map_err(|error| PluginError::Storage(error.to_string()))?;
        let name = entry.file_name();
        if name.to_bytes() == b"." || name.to_bytes() == b".." {
            continue;
        }
        names.push(name.to_owned());
    }
    names.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    Ok(names)
}

fn openat_source(directory_fd: &File, name: &CString) -> Result<File, PluginError> {
    openat(
        directory_fd,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
        Mode::empty(),
    )
    .map(File::from)
    .map_err(|error| {
        PluginError::InvalidArchive(format!("directory entry changed or became a link: {error}"))
    })
}

fn openat_destination_directory(directory_fd: &File, name: &CString) -> Result<File, PluginError> {
    openat(
        directory_fd,
        name,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map(File::from)
    .map_err(|error| PluginError::Storage(error.to_string()))
}

fn openat_destination_file(
    directory_fd: &File,
    name: &CString,
    executable: bool,
) -> Result<fs::File, PluginError> {
    let mode = if executable {
        Mode::RWXU
    } else {
        Mode::RUSR | Mode::WUSR
    };
    let descriptor = openat(
        directory_fd,
        name,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        mode,
    )
    .map_err(|error| PluginError::Storage(error.to_string()))?;
    fchmod(&descriptor, mode).map_err(|error| PluginError::Storage(error.to_string()))?;
    Ok(fs::File::from(descriptor))
}

fn create_directory_at(directory_fd: &File, name: &CString) -> Result<(), PluginError> {
    mkdirat(directory_fd, name, Mode::RWXU)
        .map_err(|error| PluginError::Storage(error.to_string()))?;
    full_sync(directory_fd)
}

fn descriptor_stat(descriptor: &File) -> Result<rustix::fs::Stat, PluginError> {
    fstat(descriptor).map_err(|error| PluginError::InvalidArchive(error.to_string()))
}

fn full_sync(file: &File) -> Result<(), PluginError> {
    FullSync::full_sync(file).map_err(|error| PluginError::Storage(error.to_string()))
}

fn full_sync_path(path: &Path) -> Result<(), PluginError> {
    let file = File::open(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    full_sync(&file)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn directory_stage_race_never_follows_a_replaced_symlink() {
        let source = TempDir::new().expect("source");
        let destination_parent = TempDir::new().expect("destination parent");
        let outside = TempDir::new().expect("outside");
        fs::write(source.path().join("payload"), "inside\n").expect("inside file");
        fs::write(outside.path().join("secret"), "outside-secret\n").expect("outside file");
        let swapped = AtomicBool::new(false);
        let hook = |relative: &Path| {
            if relative == Path::new("payload") && !swapped.swap(true, Ordering::SeqCst) {
                fs::remove_file(source.path().join("payload")).expect("remove raced file");
                std::os::unix::fs::symlink(
                    outside.path().join("secret"),
                    source.path().join("payload"),
                )
                .expect("replace with symlink");
            }
        };
        let destination = destination_parent.path().join("stage");
        assert!(matches!(
            copy_tree_with_hook(source.path(), &destination, &hook),
            Err(PluginError::InvalidArchive(_))
        ));
        assert!(!destination.join("payload").exists());
        assert_eq!(
            fs::read_to_string(outside.path().join("secret")).expect("outside remains"),
            "outside-secret\n"
        );
    }

    #[test]
    fn archive_source_symlink_is_rejected_before_reading() {
        let root = TempDir::new().expect("archive root");
        let archive = root.path().join("source.tekesplugin");
        fs::write(
            &archive,
            serde_json::to_vec(&Envelope {
                archive_version: 1,
                entries: vec![Entry {
                    path: "payload".to_owned(),
                    kind: EntryKind::File,
                    executable: None,
                    data: Some(STANDARD.encode("payload\n")),
                }],
            })
            .expect("archive bytes"),
        )
        .expect("archive");
        let alias = root.path().join("alias.tekesplugin");
        std::os::unix::fs::symlink(&archive, &alias).expect("archive alias");
        assert!(matches!(
            PluginArchive::inspect(&alias),
            Err(PluginError::InvalidArchive(_))
        ));
    }

    #[test]
    fn archive_expansion_race_never_follows_a_destination_symlink() {
        let root = TempDir::new().expect("archive root");
        let destination_parent = TempDir::new().expect("destination parent");
        let outside = TempDir::new().expect("outside");
        let archive = root.path().join("source.tekesplugin");
        fs::write(
            &archive,
            serde_json::to_vec(&Envelope {
                archive_version: 1,
                entries: vec![
                    Entry {
                        path: "parent".to_owned(),
                        kind: EntryKind::Directory,
                        executable: None,
                        data: None,
                    },
                    Entry {
                        path: "parent/payload".to_owned(),
                        kind: EntryKind::File,
                        executable: None,
                        data: Some(STANDARD.encode("inside\n")),
                    },
                ],
            })
            .expect("archive bytes"),
        )
        .expect("archive");
        let destination = destination_parent.path().join("stage");
        let swapped = AtomicBool::new(false);
        let hook = |relative: &Path| {
            if relative == Path::new("parent") && !swapped.swap(true, Ordering::SeqCst) {
                fs::remove_dir(destination.join("parent")).expect("remove raced directory");
                std::os::unix::fs::symlink(outside.path(), destination.join("parent"))
                    .expect("replace directory with symlink");
            }
        };
        assert!(matches!(
            expand_archive_with_hook(&archive, &destination, &hook),
            Err(PluginError::InvalidArchive(_))
        ));
        assert!(!outside.path().join("payload").exists());
    }
}
