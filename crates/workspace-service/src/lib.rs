//! Child-process workspace operations. The parent supplies a registry-resolved root;
//! endpoint payloads must never be used as root authority.
pub mod git;
pub mod observation;
pub mod process;
pub mod turn;
pub mod write;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
}

fn fail(code: &'static str, message: impl Into<String>) -> Failure {
    Failure {
        code,
        message: message.into(),
    }
}

impl From<std::io::Error> for Failure {
    fn from(error: std::io::Error) -> Self {
        fail(
            match error.kind() {
                std::io::ErrorKind::NotFound => "path-missing",
                std::io::ErrorKind::PermissionDenied => "permission-denied",
                _ => "io-error",
            },
            error.to_string(),
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FilesRequest {
    pub workspace_id: String,
    #[serde(default)]
    pub path: String,
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub max_bytes: Option<usize>,
}

fn locate(root: &Path, relative: &str) -> Result<(PathBuf, PathBuf), Failure> {
    let path = Path::new(relative);
    if path.is_absolute()
        || relative.contains('\0')
        || path.components().any(|c| matches!(c, Component::ParentDir))
    {
        return Err(fail(
            "path-outside-workspace",
            "Expected a workspace-relative path",
        ));
    }
    let root = root.canonicalize()?;
    let target = root.join(path).canonicalize()?;
    if !target.starts_with(&root) {
        return Err(fail("path-outside-workspace", "Link escapes workspace"));
    }
    Ok((root, target))
}

fn bounded(value: Option<usize>, default: usize, maximum: usize) -> Result<usize, Failure> {
    let value = value.unwrap_or(default);
    if value == 0 || value > maximum {
        return Err(fail("invalid-request", "Invalid result limit"));
    }
    Ok(value)
}

/// Acquire a complete bounded snapshot under a registry-resolved workspace root.
/// The caller owns the returned bytes; later path changes cannot alter a lease.
pub fn file_snapshot(root: &Path, relative: &str, maximum: usize) -> Result<Vec<u8>, Failure> {
    if maximum == 0 || maximum > 64 * 1024 * 1024 {
        return Err(fail("invalid-request", "Invalid snapshot limit"));
    }
    let (root, target) = locate(root, relative)?;
    let file = open_scoped(&root, &target, false)?;
    let before = file.metadata()?;
    if !before.is_file() {
        return Err(fail("not-file", "Expected regular file"));
    }
    if before.len() > maximum as u64 {
        return Err(fail("output-limit", "File exceeds snapshot limit"));
    }
    let mut bytes = Vec::new();
    (&file).take(maximum as u64 + 1).read_to_end(&mut bytes)?;
    let after = file.metadata()?;
    if bytes.len() > maximum {
        return Err(fail("output-limit", "File exceeds snapshot limit"));
    }
    if before.len() != after.len()
        || after.len() != bytes.len() as u64
        || before.modified()? != after.modified()?
    {
        return Err(fail("snapshot-changed", "File changed while reading"));
    }
    Ok(bytes)
}

fn file_version(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::MetadataExt;
    let identity = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        metadata.dev(),
        metadata.ino(),
        metadata.len(),
        metadata.mtime(),
        metadata.mtime_nsec(),
        metadata.ctime(),
        metadata.ctime_nsec()
    );
    format!("{:x}", Sha256::digest(identity.as_bytes()))
}

/// Text offsets are one-based line numbers; response text joins logical lines
/// without inventing an extra line for a trailing newline.
pub fn file_text_page(
    root: &Path,
    relative: &str,
    offset: usize,
    limit: usize,
) -> Result<Value, Failure> {
    use std::io::{BufRead, BufReader};
    const MAX_PAGE: usize = 4 * 1024 * 1024;
    if offset == 0 || limit == 0 || limit > 10000 {
        return Err(fail("invalid-request", "Invalid text page range"));
    }
    let (root, target) = locate(root, relative)?;
    let file = open_scoped(&root, &target, false)?;
    let before = file.metadata()?;
    if !before.is_file() {
        return Err(fail("not-file", "Expected regular file"));
    }
    let version = file_version(&before);
    let mut reader = BufReader::new(file);
    let mut line_number = 1usize;
    let mut lines = Vec::new();
    let mut output_bytes = 0;
    let mut eof = false;
    loop {
        if lines.len() == limit {
            eof = reader.fill_buf()?.is_empty();
            break;
        }
        let mut line = Vec::new();
        let count = (&mut reader)
            .take(MAX_PAGE as u64 + 1)
            .read_until(b'\n', &mut line)?;
        if count == 0 {
            eof = true;
            break;
        }
        if count > MAX_PAGE {
            return Err(fail("output-limit", "Text line exceeds limit"));
        }
        if line_number < offset {
            line_number += 1;
            continue;
        }
        if line.last() == Some(&b'\n') {
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
        }
        let line = String::from_utf8_lossy(&line).into_owned();
        let added = line.len() + usize::from(!lines.is_empty());
        if output_bytes + added > MAX_PAGE {
            if lines.is_empty() {
                return Err(fail("output-limit", "Text line exceeds limit"));
            }
            break;
        }
        output_bytes += added;
        lines.push(line);
    }
    if version != file_version(&reader.get_ref().metadata()?) {
        return Err(fail("snapshot-changed", "File changed while reading"));
    }
    Ok(
        json!({"absolutePath":target,"version":version,"bytes":before.len(),"offset":offset,
        "text":lines.join("\n"),"lines":lines.len(),"eof":eof}),
    )
}

/// Byte offsets are zero-based. The opaque version covers inode identity and
/// high-resolution modification/change times, so pages can detect replacement.
pub fn file_byte_page(
    root: &Path,
    relative: &str,
    offset: u64,
    length: usize,
) -> Result<Value, Failure> {
    use std::io::{Seek, SeekFrom};
    if length == 0 || length > 4 * 1024 * 1024 {
        return Err(fail("invalid-request", "Invalid byte page length"));
    }
    let (root, target) = locate(root, relative)?;
    let mut file = open_scoped(&root, &target, false)?;
    let before = file.metadata()?;
    if !before.is_file() {
        return Err(fail("not-file", "Expected regular file"));
    }
    if offset > before.len() {
        return Err(fail("invalid-request", "Offset exceeds file size"));
    }
    let version = file_version(&before);
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = Vec::new();
    (&file).take(length as u64).read_to_end(&mut bytes)?;
    let after = file.metadata()?;
    if version != file_version(&after)
        || bytes.len() as u64 != (before.len() - offset).min(length as u64)
    {
        return Err(fail("snapshot-changed", "File changed while reading"));
    }
    Ok(json!({"absolutePath":target,"version":version,
        "bytes":before.len(),"offset":offset,"eof":offset + bytes.len() as u64 == before.len(),
        "data":base64::engine::general_purpose::STANDARD.encode(bytes)}))
}

/// Walk from a pinned directory descriptor. A path component replaced by a
/// symlink after canonicalization is rejected rather than followed.
fn open_scoped(root: &Path, target: &Path, directory: bool) -> Result<fs::File, Failure> {
    use rustix::fs::{Mode, OFlags, open, openat};
    let mut descriptor = open(
        root,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(std::io::Error::from)?;
    let relative = target
        .strip_prefix(root)
        .map_err(|_| fail("path-outside-workspace", "Invalid scoped path"))?;
    let components = relative.components().collect::<Vec<_>>();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err(fail("path-outside-workspace", "Invalid path component"));
        };
        let mut flags = OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK;
        if directory || index + 1 < components.len() {
            flags |= OFlags::DIRECTORY;
        }
        descriptor =
            openat(&descriptor, *name, flags, Mode::empty()).map_err(std::io::Error::from)?;
    }
    Ok(fs::File::from(descriptor))
}

fn directory_entries(root: &Path, target: &Path) -> Result<Vec<Value>, Failure> {
    use rustix::fs::{Dir, FileType};
    let file = open_scoped(root, target, true)?;
    let entries = Dir::read_from(&file).map_err(std::io::Error::from)?;
    let mut result = Vec::new();
    for child in entries {
        let child = child.map_err(std::io::Error::from)?;
        let name = child
            .file_name()
            .to_str()
            .map_err(|_| fail("invalid-output", "Filename is not UTF-8"))?;
        if name == "." || name == ".." {
            continue;
        }
        if result.len() >= 100000 {
            return Err(fail("discovery-limit", "Directory entry limit exceeded"));
        }
        let path = target
            .join(name)
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let kind = match child.file_type() {
            FileType::Directory => "directory",
            FileType::RegularFile => "file",
            FileType::Symlink => "symlink",
            _ => "other",
        };
        result.push(json!({"name":name,"path":path,"kind":kind,"hidden":path.split('/').any(|s|s.starts_with('.'))}));
    }
    Ok(result)
}

pub fn files(root: &Path, method: &str, request: FilesRequest) -> Result<Value, Failure> {
    if request.workspace_id.is_empty() {
        return Err(fail("invalid-request", "Missing workspace identity"));
    }
    let (root, target) = locate(root, &request.path)?;
    match method {
        "filesList" => {
            let limit = bounded(request.limit, 2000, 10000)?;
            let mut children = directory_entries(&root, &target)?;
            children.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
            let entries = children.iter().take(limit).cloned().collect::<Vec<_>>();
            Ok(
                json!({"workspaceId":request.workspace_id,"path":target.strip_prefix(&root).unwrap().to_string_lossy(),"entries":entries,"truncated":children.len()>limit}),
            )
        }
        "filesSearch" => {
            let limit = bounded(request.limit, 1000, 10000)?;
            let needle = request.query.as_deref().unwrap_or("").trim().to_lowercase();
            if needle.is_empty() || needle.chars().count() > 256 {
                return Err(fail("invalid-request", "Expected bounded path search"));
            }
            let mut pending = vec![root.clone()];
            let mut entries = Vec::new();
            let mut visited = 0;
            let mut truncated = false;
            'search: while let Some(directory) = pending.pop() {
                for value in directory_entries(&root, &directory)? {
                    visited += 1;
                    if visited > 50000 || entries.len() >= limit {
                        truncated = true;
                        break 'search;
                    }
                    let subdirectory = (value["kind"] == "directory")
                        .then(|| root.join(value["path"].as_str().unwrap()));
                    if value["path"]
                        .as_str()
                        .unwrap()
                        .to_lowercase()
                        .contains(&needle)
                    {
                        entries.push(value);
                    }
                    if let Some(path) = subdirectory {
                        pending.push(path);
                    }
                }
            }
            entries.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));
            Ok(
                json!({"workspaceId":request.workspace_id,"path":"","entries":entries,"truncated":truncated}),
            )
        }
        "filesRead" => {
            // Zero explicitly requests a complete file; existing preview callers stay bounded.
            let maximum = if request.max_bytes == Some(0) {
                None
            } else {
                Some(bounded(request.max_bytes, 1024 * 1024, 4 * 1024 * 1024)?)
            };
            let file = open_scoped(&root, &target, false)?;
            let before = file.metadata()?;
            if !before.is_file() {
                return Err(fail("not-file", "Expected regular file"));
            }
            let mut bytes = Vec::new();
            match maximum {
                Some(maximum) => {
                    (&file).take(maximum as u64 + 1).read_to_end(&mut bytes)?;
                }
                None => {
                    (&file).read_to_end(&mut bytes)?;
                }
            }
            let after = file.metadata()?;
            if before.len() != after.len() || before.modified()? != after.modified()? {
                return Err(fail("snapshot-changed", "File changed while reading"));
            }
            let truncated = maximum.is_some_and(|maximum| bytes.len() > maximum)
                || after.len() > bytes.len() as u64;
            if let Some(maximum) = maximum {
                bytes.truncate(maximum);
            }
            let revision = (!truncated).then(|| format!("{:x}", Sha256::digest(&bytes)));
            Ok(
                json!({"workspaceId":request.workspace_id,"path":request.path,"encoding":"base64",
                "content":base64::engine::general_purpose::STANDARD.encode(bytes),"size":after.len(),"truncated":truncated,"revision":revision}),
            )
        }
        _ => Err(fail("unsupported", "Unknown file operation")),
    }
}

#[cfg(all(test, unix))]
mod scoped_tests {
    use super::*;
    #[test]
    fn text_pages_preserve_empty_lines_unicode_and_eof() {
        let workspace = tempfile::tempdir().unwrap();
        fs::write(workspace.path().join("text"), "第一行\r\n\nlast\n").unwrap();
        let first = file_text_page(workspace.path(), "text", 1, 2).unwrap();
        assert_eq!(first["text"], "第一行\n");
        assert_eq!(first["lines"], 2);
        assert_eq!(first["eof"], false);
        let last = file_text_page(workspace.path(), "text", 3, 2).unwrap();
        assert_eq!(last["text"], "last");
        assert_eq!(last["lines"], 1);
        assert_eq!(last["eof"], true);
        assert_eq!(last["version"], first["version"]);
        let eof = file_text_page(workspace.path(), "text", 4, 2).unwrap();
        assert_eq!(eof["lines"], 0);
        assert_eq!(eof["eof"], true);
        assert!(file_text_page(workspace.path(), "text", 0, 2).is_err());
        fs::write(workspace.path().join("text"), "no newline").unwrap();
        assert_eq!(
            file_text_page(workspace.path(), "text", 1, 1).unwrap()["eof"],
            true
        );
        fs::write(
            workspace.path().join("text"),
            vec![b'x'; 4 * 1024 * 1024 + 1],
        )
        .unwrap();
        assert_eq!(
            file_text_page(workspace.path(), "text", 1, 1)
                .unwrap_err()
                .code,
            "output-limit"
        );
    }
    #[test]
    fn byte_pages_are_bounded_and_detect_same_size_replacement() {
        let workspace = tempfile::tempdir().unwrap();
        fs::write(workspace.path().join("file"), [0, 255, 2, 3, 4]).unwrap();
        let first = file_byte_page(workspace.path(), "file", 0, 2).unwrap();
        let last = file_byte_page(workspace.path(), "file", 2, 3).unwrap();
        assert_eq!(first["version"], last["version"]);
        assert_eq!(first["data"], "AP8=");
        assert_eq!(first["eof"], false);
        assert_eq!(last["data"], "AgME");
        assert_eq!(last["eof"], true);
        assert!(file_byte_page(workspace.path(), "file", 6, 1).is_err());
        assert!(file_byte_page(workspace.path(), "file", 0, 0).is_err());
        fs::write(workspace.path().join("replacement"), [5, 6, 7, 8, 9]).unwrap();
        fs::rename(
            workspace.path().join("replacement"),
            workspace.path().join("file"),
        )
        .unwrap();
        assert_ne!(
            first["version"],
            file_byte_page(workspace.path(), "file", 0, 2).unwrap()["version"]
        );
        assert_eq!(
            file_byte_page(workspace.path(), "file", 5, 2).unwrap()["eof"],
            true
        );
    }
    #[test]
    fn byte_page_reads_large_sparse_file_without_full_snapshot() {
        use std::io::{Seek, SeekFrom, Write};
        let workspace = tempfile::tempdir().unwrap();
        let mut file = fs::File::create(workspace.path().join("large")).unwrap();
        let size = 1024 * 1024 * 1024u64;
        file.set_len(size).unwrap();
        file.seek(SeekFrom::Start(size - 4)).unwrap();
        file.write_all(b"tail").unwrap();
        let page = file_byte_page(workspace.path(), "large", size - 4, 4).unwrap();
        assert_eq!(page["bytes"], size);
        assert_eq!(page["offset"], size - 4);
        assert_eq!(page["data"], "dGFpbA==");
        assert_eq!(page["eof"], true);
    }
    #[test]
    fn complete_snapshot_is_bounded_scoped_and_independent_of_later_writes() {
        let workspace = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::write(workspace.path().join("file"), b"original").unwrap();
        let snapshot = file_snapshot(workspace.path(), "file", 8).unwrap();
        fs::write(workspace.path().join("file"), b"changed!").unwrap();
        assert_eq!(snapshot, b"original");
        assert_eq!(
            file_snapshot(workspace.path(), "file", 7).unwrap_err().code,
            "output-limit"
        );
        assert!(file_snapshot(workspace.path(), "../file", 100).is_err());
        fs::write(outside.path().join("secret"), b"outside").unwrap();
        std::os::unix::fs::symlink(outside.path().join("secret"), workspace.path().join("link"))
            .unwrap();
        assert!(file_snapshot(workspace.path(), "link", 100).is_err());
        assert_eq!(
            file_snapshot(workspace.path(), "", 100).unwrap_err().code,
            "not-file"
        );
    }
    #[test]
    fn replacement_after_resolution_cannot_escape_descriptor_walk() {
        let workspace = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::create_dir(workspace.path().join("folder")).unwrap();
        fs::write(workspace.path().join("folder/data"), "allowed").unwrap();
        fs::write(outside.path().join("data"), "must not read").unwrap();
        let (root, target) = locate(workspace.path(), "folder/data").unwrap();
        fs::rename(
            workspace.path().join("folder"),
            workspace.path().join("original"),
        )
        .unwrap();
        std::os::unix::fs::symlink(outside.path(), workspace.path().join("folder")).unwrap();
        assert!(open_scoped(&root, &target, false).is_err());
        assert!(directory_entries(&root, &root.join("folder")).is_err());
    }
}
