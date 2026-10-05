//! Bounded in-process search for installations without the rg executable.
use super::*;
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use regex::bytes::RegexBuilder;

#[allow(clippy::too_many_arguments)]
pub(super) fn search(
    root: &RootBinding,
    path: &str,
    pattern: &str,
    glob: Option<&str>,
    mode: &str,
    insensitive: bool,
    context: u64,
    output_cap: u64,
    timeout_ms: u64,
) -> Result<ExecValue, HelperError> {
    if !matches!(mode, "content" | "files_with_matches" | "count") {
        return Err(HelperError::new(
            HelperErrorClass::InvalidRequest,
            "invalid grep output mode",
        ));
    }
    if timeout_ms == 0 || timeout_ms > HARD_TIMEOUT_MS || context > 100_000 {
        return Err(HelperError::new(
            HelperErrorClass::Limit,
            "grep limit out of bounds",
        ));
    }
    let cap = checked_cap(output_cap)?;
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(insensitive)
        .size_limit(4 * 1024 * 1024)
        .dfa_size_limit(4 * 1024 * 1024)
        .build()
        .map_err(|e| HelperError::new(HelperErrorClass::InvalidRequest, e.to_string()))?;
    let relative = if path.is_empty() {
        PathBuf::new()
    } else {
        normalized_relative(path)?
    };
    if !relative.as_os_str().is_empty() {
        let (parents, leaf) = split_parent(path)?;
        let parent = open_parent(root.directory.as_raw_fd(), &parents)?;
        match open_directory_at(parent.as_raw_fd(), &leaf) {
            Ok(_) => {}
            Err(error) if error.class == HelperErrorClass::NotRegular => {
                open_regular_at(parent.as_raw_fd(), &leaf, libc::O_RDONLY, 0)?;
            }
            Err(error) => return Err(error),
        }
    }
    let base = root.path.join(&relative);
    let mut walker = WalkBuilder::new(&base);
    if let Some(glob) = glob {
        let mut overrides = OverrideBuilder::new(&base);
        overrides
            .add(glob)
            .map_err(|e| HelperError::new(HelperErrorClass::InvalidRequest, e.to_string()))?;
        walker.overrides(
            overrides
                .build()
                .map_err(|e| HelperError::new(HelperErrorClass::InvalidRequest, e.to_string()))?,
        );
    }
    walker
        .follow_links(false)
        .max_filesize(Some(HARD_BYTES as u64))
        .git_global(false)
        .git_exclude(false)
        .parents(false)
        .sort_by_file_path(|a, b| a.cmp(b));
    let started = Instant::now();
    let mut stdout = Vec::new();
    let mut matched = false;
    let mut truncated = false;
    let mut total = 0usize;
    let mut matches = 0usize;
    for (index, entry) in walker.build().enumerate() {
        if index >= 100_000 || started.elapsed() > Duration::from_millis(timeout_ms) {
            return Err(HelperError::new(
                HelperErrorClass::Limit,
                "grep scan/time limit exceeded",
            ));
        }
        let entry = entry.map_err(|e| HelperError::new(HelperErrorClass::Io, e.to_string()))?;
        if !entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&root.path)
            .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "grep escaped root"))?;
        let display = entry
            .path()
            .strip_prefix(&base)
            .ok()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(relative);
        // Reopen every component beneath the pinned root without following links.
        // Traversal metadata never authorizes a subsequent file read.
        let bytes = read_regular(
            root.directory.as_raw_fd(),
            &relative.to_string_lossy(),
            HARD_BYTES,
        )?;
        total = total.saturating_add(bytes.len());
        if total > 32 * 1024 * 1024 {
            return Err(HelperError::new(
                HelperErrorClass::Limit,
                "grep total-byte limit exceeded",
            ));
        }
        if bytes.contains(&0) {
            continue;
        }
        let mut lines: Vec<_> = bytes.split(|b| *b == b'\n').collect();
        if lines.last().is_some_and(|line| line.is_empty()) {
            lines.pop();
        }
        let mut hits = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if started.elapsed() > Duration::from_millis(timeout_ms) {
                return Err(HelperError::new(
                    HelperErrorClass::Timeout,
                    "grep timed out",
                ));
            }
            if regex.is_match(line) {
                if matches.saturating_add(hits.len()) >= 100_000 {
                    return Err(HelperError::new(
                        HelperErrorClass::Limit,
                        "grep match limit exceeded",
                    ));
                }
                hits.push(i);
            }
        }
        if hits.is_empty() {
            continue;
        }
        matched = true;
        matches = matches.saturating_add(hits.len());
        if matches > 100_000 {
            return Err(HelperError::new(
                HelperErrorClass::Limit,
                "grep match limit exceeded",
            ));
        }
        let name = display.to_string_lossy();
        let mut emit = |value: &[u8]| {
            let remaining = cap.saturating_sub(stdout.len());
            stdout.extend_from_slice(&value[..value.len().min(remaining)]);
            truncated |= value.len() > remaining;
        };
        match mode {
            "files_with_matches" => emit(format!("{name}\n").as_bytes()),
            "count" => emit(format!("{name}:{}\n", hits.len()).as_bytes()),
            _ => {
                let mut selected = std::collections::BTreeSet::new();
                for hit in &hits {
                    selected.extend(
                        hit.saturating_sub(context as usize)
                            ..=hit.saturating_add(context as usize).min(lines.len() - 1),
                    );
                }
                let mut previous = None;
                for i in selected {
                    if previous.is_some_and(|p| i > p + 1) {
                        emit(b"--\n");
                    }
                    previous = Some(i);
                    let separator = if hits.binary_search(&i).is_ok() {
                        ":"
                    } else {
                        "-"
                    };
                    emit(format!("{name}{separator}{}{separator}", i + 1).as_bytes());
                    let line = lines[i];
                    if line.len() > 2000 {
                        emit(b"[line exceeds 2000-byte display limit]");
                    } else {
                        emit(line);
                    }
                    emit(b"\n");
                }
            }
        }
        if truncated {
            break;
        }
    }
    Ok(ExecValue {
        status: if matched { 0 } else { 1 },
        stdout: ByteString::from_bytes(&stdout),
        stderr: ByteString::from_bytes(&[]),
        stdout_truncated: truncated,
        stderr_truncated: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_regex_modes_bounds_and_root_confinement() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("docs")).unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        std::fs::write(
            dir.path().join("docs/a.md"),
            b"before\nNeedle 42\nafter\nNeedle 7\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("ignored.md"), b"Needle 9").unwrap();
        std::fs::write(dir.path().join(".gitignore"), b"ignored.md\n").unwrap();
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("secret"), b"Needle SECRET").unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join("escape")).unwrap();
        let root = RootBinding::open("workspace", dir.path()).unwrap();
        let run = |mode, context, cap| {
            search(
                &root,
                "",
                r"needle \d+",
                None,
                mode,
                true,
                context,
                cap,
                1000,
            )
            .unwrap()
        };
        let explicit = search(
            &root,
            "",
            "Needle",
            Some("ignored.md"),
            "files_with_matches",
            false,
            0,
            4096,
            1000,
        )
        .unwrap();
        assert_eq!(
            explicit.stdout.decode().unwrap(),
            b"ignored.md\n",
            "explicit glob overrides ignore rules"
        );
        let files = run("files_with_matches", 0, 4096);
        assert_eq!(files.stdout.decode().unwrap(), b"docs/a.md\n");
        assert_eq!(
            run("count", 0, 4096).stdout.decode().unwrap(),
            b"docs/a.md:2\n"
        );
        let content = String::from_utf8(run("content", 1, 4096).stdout.decode().unwrap()).unwrap();
        assert!(content.contains("docs/a.md:2:Needle 42"));
        assert!(content.contains("docs/a.md-1-before"));
        assert_eq!(content.matches("docs/a.md-3-after").count(), 1);
        assert!(run("content", 0, 4).stdout_truncated);
        assert!(
            search(
                &root,
                "../escape",
                "x",
                None,
                "content",
                false,
                0,
                100,
                1000
            )
            .is_err()
        );
        assert!(
            search(
                &root,
                "escape/secret",
                "Needle",
                None,
                "content",
                false,
                0,
                100,
                1000
            )
            .is_err()
        );
        assert!(search(&root, "", "[", None, "content", false, 0, 100, 1000).is_err());
        assert_eq!(
            search(&root, "", "absent", None, "count", false, 0, 100, 1000)
                .unwrap()
                .status,
            1
        );
    }
}
