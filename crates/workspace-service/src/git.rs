//! Git arguments are constructed here, never supplied as a shell string.
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;
use tokio::process::Command;

use crate::{Failure, bounded, fail, locate};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Query {
    pub workspace_id: String,
    pub repository_path: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub head: Option<String>,
    pub scope: Option<String>,
    pub revision: Option<String>,
    pub paths: Option<Vec<String>>,
    pub include_clean: Option<bool>,
    pub operation: Option<String>,
    pub name: Option<String>,
    pub message: Option<String>,
    pub remote: Option<String>,
}

struct Output {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

async fn read(mut reader: impl tokio::io::AsyncRead + Unpin) -> Result<Vec<u8>, Failure> {
    let mut bytes = Vec::new();
    (&mut reader)
        .take(4 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .await?;
    if bytes.len() > 4 * 1024 * 1024 {
        return Err(fail("output-limit", "Git output exceeded limit"));
    }
    Ok(bytes)
}

async fn run(root: &Path, args: &[&str]) -> Result<Output, Failure> {
    run_deadline(root, args, Duration::from_secs(15)).await
}

async fn run_deadline(root: &Path, args: &[&str], deadline: Duration) -> Result<Output, Failure> {
    let mut command = Command::new("git");
    command
        .arg("--no-pager")
        .args(args)
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    command
        .env("LC_ALL", "C")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_PAGER", "cat");
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let result = tokio::time::timeout(deadline, async {
        tokio::try_join!(read(stdout), read(stderr), async {
            child.wait().await.map_err(Failure::from)
        })
    })
    .await;
    match result {
        Ok(Ok((stdout, stderr, status))) => Ok(Output {
            code: status.code(),
            stdout,
            stderr,
        }),
        error => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            Err(match error {
                Ok(Err(error)) => error,
                _ => fail("timeout", "Git timed out"),
            })
        }
    }
}

fn checked(output: Output) -> Result<String, Failure> {
    if output.code != Some(0) {
        return Err(fail("git-failed", String::from_utf8_lossy(&output.stderr)));
    }
    String::from_utf8(output.stdout).map_err(|_| fail("invalid-output", "Git output is not UTF-8"))
}

async fn repository(root: &Path, request: &Query) -> Result<Option<PathBuf>, Failure> {
    let (root, target) = locate(root, request.repository_path.as_deref().unwrap_or(""))?;
    let output = run(&target, &["rev-parse", "--show-toplevel"]).await?;
    if output.code != Some(0)
        && String::from_utf8_lossy(&output.stderr).contains("not a git repository")
    {
        return Ok(None);
    }
    let path = PathBuf::from(checked(output)?.trim_end_matches('\n')).canonicalize()?;
    if !path.starts_with(root) {
        return Err(fail(
            "repository-outside-workspace",
            "Git root escapes workspace",
        ));
    }
    Ok(Some(path))
}

async fn require_repository(root: &Path, request: &Query) -> Result<PathBuf, Failure> {
    if let Some(path) = repository(root, request).await? {
        return Ok(path);
    }
    if request.repository_path.is_none() {
        let root = root.canonicalize()?;
        let candidates = repository_paths(&root)?;
        if candidates.len() > 1 {
            return Err(fail(
                "repository-selection-required",
                "Select a repository in this workspace",
            ));
        }
        if let Some(path) = candidates.first() {
            let selected: Query = serde_json::from_value(json!({
                "workspaceId": request.workspace_id,
                "repositoryPath": path.strip_prefix(&root).unwrap().to_string_lossy()
            }))
            .unwrap();
            if let Some(path) = repository(&root, &selected).await? {
                return Ok(path);
            }
        }
    }
    Err(fail("not-repository", "Workspace is not a Git repository"))
}

pub async fn query(root: &Path, method: &str, request: Query) -> Result<Value, Failure> {
    if request.workspace_id.is_empty() {
        return Err(fail("invalid-request", "Missing workspace identity"));
    }
    if method == "gitChangesNavigator" {
        return navigator(root, request.include_clean.unwrap_or(false)).await;
    }
    if method == "gitRepository" {
        let repository = repository(root, &request).await?;
        return Ok(
            json!({"isRepository":repository.is_some(),"repositoryRoot":repository,"contractVersion":"1"}),
        );
    }
    let root = require_repository(root, &request).await?;
    match method {
        "gitStatus" => status(&root).await,
        "gitBranches" => branches(&root).await,
        "gitDiff" => diff(&root, &request).await,
        "gitLog" => {
            let limit = bounded(request.limit, 50, 100)?;
            let offset = request.offset.unwrap_or(0);
            if offset > 1_000_000
                || request.head.as_ref().is_some_and(|head| {
                    ![40, 64].contains(&head.len())
                        || !head
                            .bytes()
                            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
                })
            {
                return Err(fail("invalid-request", "Invalid history page"));
            }
            let head = match request.head {
                Some(head) => Some(head),
                None => {
                    let result = run(&root, &["rev-parse", "--verify", "--quiet", "HEAD"]).await?;
                    if result.code == Some(1) {
                        None
                    } else {
                        Some(checked(result)?.trim().to_owned())
                    }
                }
            };
            let Some(head) = head else {
                return Ok(json!({"head":null,"commits":[],"nextOffset":null}));
            };
            let raw = checked(
                run(
                    &root,
                    &[
                        "log",
                        "-z",
                        "--format=%H%x00%an%x00%aI%x00%s",
                        &format!("--max-count={}", limit + 1),
                        &format!("--skip={offset}"),
                        &head,
                        "--",
                    ],
                )
                .await?,
            )?;
            let fields = raw
                .strip_suffix('\0')
                .unwrap_or(&raw)
                .split('\0')
                .collect::<Vec<_>>();
            let mut commits = Vec::new();
            if !raw.is_empty() {
                if fields.len() % 4 != 0 {
                    return Err(fail("invalid-output", "Malformed Git history"));
                }
                for row in fields.chunks_exact(4) {
                    commits
                        .push(json!({"id":row[0],"author":row[1],"date":row[2],"subject":row[3]}));
                }
            }
            let next = (commits.len() > limit).then_some(offset + limit);
            commits.truncate(limit);
            Ok(json!({"head":head,"commits":commits,"nextOffset":next}))
        }
        _ => Err(fail("unsupported", "Unknown Git query")),
    }
}

async fn status(root: &Path) -> Result<Value, Failure> {
    let raw = checked(
        run(
            root,
            &[
                "status",
                "--porcelain=v2",
                "--branch",
                "-z",
                "--untracked-files=all",
            ],
        )
        .await?,
    )?;
    let mut value = json!({"repositoryRoot":root,"branch":null,"head":null,"detached":false,
        "upstream":null,"ahead":0,"behind":0,"remotes":[],"changes":[],"contractVersion":"1",
        "lastCommitShort":null,"lastCommitSubject":null});
    let mut records = raw.split('\0');
    let mut changes = Vec::new();
    while let Some(row) = records.next() {
        if row.is_empty() {
            continue;
        }
        if let Some(head) = row.strip_prefix("# branch.oid ") {
            if head != "(initial)" {
                value["head"] = json!(head);
            }
        } else if let Some(branch) = row.strip_prefix("# branch.head ") {
            value["detached"] = json!(branch == "(detached)");
            if branch != "(detached)" {
                value["branch"] = json!(branch);
            }
        } else if let Some(upstream) = row.strip_prefix("# branch.upstream ") {
            value["upstream"] = json!(upstream);
        } else if let Some(ab) = row.strip_prefix("# branch.ab ") {
            let parts = ab.split(' ').collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(fail("invalid-output", "Malformed branch tracking"));
            }
            value["ahead"] = json!(
                parts[0]
                    .strip_prefix('+')
                    .and_then(|n| n.parse::<u64>().ok())
                    .ok_or_else(|| fail("invalid-output", "Malformed ahead count"))?
            );
            value["behind"] = json!(
                parts[1]
                    .strip_prefix('-')
                    .and_then(|n| n.parse::<u64>().ok())
                    .ok_or_else(|| fail("invalid-output", "Malformed behind count"))?
            );
        } else if let Some(path) = row.strip_prefix("? ") {
            changes.push(json!({"path":path,"kind":"untracked","indexStatus":"?","workTreeStatus":"?","isConflict":false}));
        } else if row.starts_with("1 ") || row.starts_with("2 ") || row.starts_with("u ") {
            let kind = row.as_bytes()[0];
            let count = match kind {
                b'1' => 8,
                b'2' => 9,
                _ => 10,
            };
            let fields = row.splitn(count + 1, ' ').collect::<Vec<_>>();
            if fields.len() != count + 1 || fields[1].len() != 2 || !fields[1].is_ascii() {
                return Err(fail("invalid-output", "Malformed status record"));
            }
            let xy = fields[1];
            let mut change = json!({"path":fields[count],"kind":match kind {b'u'=>"unmerged",b'2'=>if xy.contains('C') {"copied"} else {"renamed"},_=>"ordinary"},
                "indexStatus":&xy[..1],"workTreeStatus":&xy[1..],"isConflict":kind==b'u'});
            if kind == b'2' {
                let source = records
                    .next()
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| fail("invalid-output", "Missing rename source"))?;
                change["originalPath"] = json!(source);
            }
            changes.push(change);
        } else if !row.starts_with('#') && !row.starts_with("! ") {
            return Err(fail("invalid-output", "Unknown Git status record"));
        }
    }
    value["changes"] = json!(changes);
    value["remotes"] = json!(
        checked(run(root, &["remote"]).await?)?
            .lines()
            .collect::<Vec<_>>()
    );
    if let Some(head) = value["head"].as_str() {
        let metadata = checked(run(root, &["show", "-s", "--format=%h%x00%s", head]).await?)?;
        let (short, subject) = metadata
            .trim_end_matches('\n')
            .split_once('\0')
            .ok_or_else(|| fail("invalid-output", "Missing commit metadata"))?;
        value["lastCommitShort"] = json!(short);
        value["lastCommitSubject"] = json!(subject);
    }
    Ok(value)
}

async fn branches(root: &Path) -> Result<Value, Failure> {
    let raw = checked(run(root, &["for-each-ref", "--format=%(refname:short)%00%(HEAD)%00%(upstream:short)%00%(objectname)%00%(upstream:track)%00%(worktreepath)%00", "refs/heads/"]).await?)?;
    let fields = raw.split('\0').collect::<Vec<_>>();
    let mut branches = Vec::new();
    for row in fields.chunks_exact(6) {
        let name = row[0].strip_prefix('\n').unwrap_or(row[0]);
        let tracking = row[4];
        let count = |key: &str| -> u64 {
            tracking
                .split_once(key)
                .and_then(|(_, tail)| tail.split(|c: char| !c.is_ascii_digit()).next())
                .and_then(|n| n.parse().ok())
                .unwrap_or(0)
        };
        let tracked = !row[2].is_empty() && tracking != "[gone]";
        branches.push(json!({"name":name,"isCurrent":row[1]=="*","upstream":(!row[2].is_empty()).then_some(row[2]),"head":row[3],
            "isMerged":null,"aheadOfUpstream":tracked.then(||count("ahead ")),"behindUpstream":tracked.then(||count("behind ")),
            "isUpstreamGone":tracking=="[gone]","worktreePath":(!row[5].is_empty()).then_some(row[5])}));
    }
    let remote = run(
        root,
        &[
            "symbolic-ref",
            "--quiet",
            "--short",
            "refs/remotes/origin/HEAD",
        ],
    )
    .await?;
    let default = if remote.code == Some(0) {
        Some(checked(remote)?.trim().to_owned())
    } else if remote.code == Some(1) {
        ["main", "master"]
            .iter()
            .find(|name| branches.iter().any(|b| b["name"] == **name))
            .map(|s| s.to_string())
    } else {
        return Err(fail("git-failed", String::from_utf8_lossy(&remote.stderr)));
    };
    if let Some(default) = &default {
        let merged = checked(
            run(
                root,
                &[
                    "for-each-ref",
                    &format!("--merged={default}"),
                    "--format=%(refname:short)",
                    "refs/heads/",
                ],
            )
            .await?,
        )?;
        for branch in &mut branches {
            branch["isMerged"] = json!(merged.lines().any(|name| branch["name"] == name));
        }
    }
    Ok(json!({"branches":branches,"defaultBranch":default,"contractVersion":"1"}))
}

async fn diff(root: &Path, request: &Query) -> Result<Value, Failure> {
    let paths = request.paths.as_deref().unwrap_or(&[]);
    for path in paths {
        if path.is_empty()
            || path.contains('\0')
            || Path::new(path).is_absolute()
            || path.split(['/', '\\']).any(|part| part == "..")
        {
            return Err(fail(
                "invalid-request",
                "Expected repository-relative paths",
            ));
        }
    }
    let scope = request.scope.as_deref().unwrap_or("all");
    let resolved_revision;
    let ancestor;
    let mut args = vec![
        "--literal-pathspecs",
        "diff",
        "--no-ext-diff",
        "--no-textconv",
        "--no-color",
    ];
    match scope {
        "committed" | "branch" => {
            let revision = request
                .revision
                .as_deref()
                .unwrap_or(if scope == "committed" {
                    "HEAD"
                } else {
                    "@{upstream}"
                });
            if revision.is_empty() || revision.contains('\0') || revision.len() > 4096 {
                return Err(fail("invalid-request", "Invalid revision"));
            }
            resolved_revision = checked(
                run(
                    root,
                    &[
                        "rev-parse",
                        "--verify",
                        "--end-of-options",
                        &format!("{revision}^{{commit}}"),
                    ],
                )
                .await?,
            )?;
            if scope == "committed" {
                args = vec![
                    "--literal-pathspecs",
                    "show",
                    "--format=",
                    "--root",
                    "--first-parent",
                    "--no-ext-diff",
                    "--no-textconv",
                    "--no-color",
                    resolved_revision.trim(),
                ];
            } else {
                ancestor =
                    checked(run(root, &["merge-base", resolved_revision.trim(), "HEAD"]).await?)?;
                args.push(ancestor.trim());
            }
        }
        "staged" => args.push("--cached"),
        "unstaged" => (),
        "all" => {
            let head = run(root, &["rev-parse", "--verify", "--quiet", "HEAD"]).await?;
            if head.code == Some(0) {
                args.push("HEAD");
            } else if head.code == Some(1) {
                args.push("--cached");
            } else {
                checked(head)?;
            }
        }
        _ => return Err(fail("invalid-request", "Unknown diff scope")),
    }
    args.push("--");
    args.extend(paths.iter().map(String::as_str));
    let mut unified = checked(run(root, &args).await?)?;
    if !paths.is_empty() && matches!(scope, "all" | "unstaged") {
        let state = status(root).await?;
        let mut seen = std::collections::BTreeSet::new();
        for path in paths {
            if !seen.insert(path)
                || !state["changes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|c| c["kind"] == "untracked" && c["path"] == *path)
            {
                continue;
            }
            let (_, target) = locate(root, path)?;
            if !target.metadata()?.is_file() {
                return Err(fail("not-file", "Expected file diff"));
            }
            let mut result = run(
                root,
                &[
                    "diff",
                    "--no-index",
                    "--no-ext-diff",
                    "--no-textconv",
                    "--no-color",
                    "--",
                    "/dev/null",
                    path,
                ],
            )
            .await?;
            // --no-index reports 1 for an ordinary difference.
            if result.code == Some(1) {
                result.code = Some(0);
            }
            unified.push_str(&checked(result)?);
            if unified.len() > 4 * 1024 * 1024 {
                return Err(fail("output-limit", "Combined diff exceeds limit"));
            }
        }
    }
    Ok(
        json!({"repositoryRoot":root,"isBinary":unified.lines().any(|line|line.starts_with("Binary files ") && line.ends_with(" differ")),
        "unifiedDiff":unified,"truncated":false,"contractVersion":"1"}),
    )
}

fn counts(raw: &str) -> Result<std::collections::BTreeMap<String, Value>, Failure> {
    let mut records = raw.split('\0');
    let mut result = std::collections::BTreeMap::new();
    while let Some(row) = records.next() {
        if row.is_empty() {
            continue;
        }
        let fields = row.splitn(3, '\t').collect::<Vec<_>>();
        if fields.len() != 3 {
            return Err(fail("invalid-output", "Malformed numstat"));
        }
        let parse = |s: &str| -> Result<Option<u64>, Failure> {
            if s == "-" {
                Ok(None)
            } else {
                s.parse()
                    .map(Some)
                    .map_err(|_| fail("invalid-output", "Malformed numstat count"))
            }
        };
        let additions = parse(fields[0])?;
        let deletions = parse(fields[1])?;
        let path = if fields[2].is_empty() {
            records
                .next()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| fail("invalid-output", "Missing rename source"))?;
            records
                .next()
                .filter(|s| !s.is_empty())
                .ok_or_else(|| fail("invalid-output", "Missing rename target"))?
        } else {
            fields[2]
        };
        result.insert(path.to_owned(),json!({"additions":additions,"deletions":deletions,"isBinary":additions.is_none() || deletions.is_none()}));
    }
    Ok(result)
}

fn navigator_file(
    path: &str,
    original: Option<&str>,
    status: &str,
    counts: &std::collections::BTreeMap<String, Value>,
) -> Value {
    let mut file = json!({"path":path,"originalPath":original,"status":status,"additions":null,"deletions":null,"isBinary":null,"isConflict":status=="U"});
    if let Some(count) = counts.get(path) {
        for key in ["additions", "deletions", "isBinary"] {
            file[key] = count[key].clone();
        }
    }
    file
}

fn repository_paths(root: &Path) -> Result<Vec<PathBuf>, Failure> {
    let mut paths = Vec::new();
    let marker = |path: &Path| -> Result<bool, Failure> {
        match std::fs::symlink_metadata(path.join(".git")) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    };
    if marker(root)? {
        paths.push(root.to_path_buf());
    }
    let mut children = 0;
    for child in std::fs::read_dir(root)? {
        let child = child?;
        if child.file_type()?.is_dir() && !child.file_name().to_string_lossy().starts_with('.') {
            children += 1;
            if children > 512 {
                return Err(fail("discovery-limit", "Too many workspace directories"));
            }
            if marker(&child.path())? {
                paths.push(child.path());
            }
        }
    }
    paths.sort();
    Ok(paths)
}

async fn navigator(workspace: &Path, include_clean: bool) -> Result<Value, Failure> {
    let root = workspace.canonicalize()?;
    let paths = repository_paths(&root)?;
    let mut repositories = Vec::new();
    for path in paths {
        // Re-check Git's actual root (including worktree .git files) against workspace authority.
        let request: Query = serde_json::from_value(json!({"workspaceId":"internal", "repositoryPath":path.strip_prefix(&root).unwrap().to_string_lossy()})).unwrap();
        let path = repository(&root, &request)
            .await?
            .ok_or_else(|| fail("not-repository", "Repository disappeared"))?;
        let state = status(&path).await?;
        let base = if state["head"].is_null() {
            "--cached"
        } else {
            "HEAD"
        };
        let counts = counts(&checked(
            run(
                &path,
                &[
                    "diff",
                    "--numstat",
                    "-z",
                    "--no-ext-diff",
                    "--no-textconv",
                    "-M",
                    base,
                ],
            )
            .await?,
        )?)?;
        let uncommitted = state["changes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|change| {
                let status = if change["isConflict"] == true {
                    "U"
                } else if change["kind"] == "untracked" {
                    "?"
                } else if change["workTreeStatus"] != "." {
                    change["workTreeStatus"].as_str().unwrap()
                } else {
                    change["indexStatus"].as_str().unwrap()
                };
                navigator_file(
                    change["path"].as_str().unwrap(),
                    change["originalPath"].as_str(),
                    status,
                    &counts,
                )
            })
            .collect::<Vec<_>>();
        let mut outgoing = Vec::new();
        if state["upstream"].is_string() && state["ahead"].as_u64().unwrap_or(0) > 0 {
            let upstream = checked(run(&path, &["rev-parse", "--verify", "@{upstream}"]).await?)?;
            let range = format!("{}..{}", upstream.trim(), state["head"].as_str().unwrap());
            let commits = checked(run(&path, &["rev-list", "--max-count=20", &range]).await?)?;
            for commit in commits.lines() {
                let header =
                    checked(run(&path, &["show", "-s", "--format=%h%x00%s", commit]).await?)?;
                let (short, subject) = header
                    .trim_end_matches('\n')
                    .split_once('\0')
                    .ok_or_else(|| fail("invalid-output", "Missing commit header"))?;
                let parents =
                    checked(run(&path, &["rev-list", "--parents", "-n", "1", commit]).await?)?;
                let mut args = vec![
                    "diff-tree",
                    "--root",
                    "--no-commit-id",
                    "-r",
                    "--no-ext-diff",
                    "--no-textconv",
                    "-M",
                ];
                if let Some(parent) = parents.split_whitespace().nth(1) {
                    args.push(parent);
                }
                args.push(commit);
                let mut count_args = args.clone();
                count_args.extend(["--numstat", "-z"]);
                let commit_counts = self::counts(&checked(run(&path, &count_args).await?)?)?;
                args.extend(["--name-status", "-z"]);
                let raw = checked(run(&path, &args).await?)?;
                let mut records = raw.split('\0');
                let mut files = Vec::new();
                while let Some(status) = records.next() {
                    if status.is_empty() {
                        continue;
                    }
                    let source = records
                        .next()
                        .filter(|s| !s.is_empty())
                        .ok_or_else(|| fail("invalid-output", "Missing changed path"))?;
                    let renamed = status.starts_with('R') || status.starts_with('C');
                    let target = if renamed {
                        records
                            .next()
                            .filter(|s| !s.is_empty())
                            .ok_or_else(|| fail("invalid-output", "Missing changed target"))?
                    } else {
                        source
                    };
                    files.push(navigator_file(
                        target,
                        renamed.then_some(source),
                        &status[..1],
                        &commit_counts,
                    ));
                }
                outgoing.push(
                    json!({"commit":commit,"shortCommit":short,"subject":subject,"files":files}),
                );
            }
        }
        if include_clean || !uncommitted.is_empty() || !outgoing.is_empty() {
            repositories.push(json!({"repositoryRoot":path,"name":path.file_name().unwrap_or_default().to_string_lossy(),"branch":state["branch"],"head":state["head"],
                "detached":state["detached"],"upstream":state["upstream"],"uncommitted":uncommitted,"unpushedCommits":outgoing,"unpushedCommitsTruncated":state["ahead"].as_u64().unwrap_or(0)>20}));
        }
    }
    Ok(json!({"repositories":repositories,"contractVersion":"1"}))
}

/// Loaded only from a parent-selected authority file, never from endpoint JSON.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutationAuthority {
    pub state_root: PathBuf,
    #[serde(default)]
    pub allow_file_writes: bool,
    #[serde(default)]
    pub workspace_roots: Vec<PathBuf>,
    pub allowed_operations: Vec<String>,
    pub allowed_repository_roots: Vec<PathBuf>,
}

async fn mutation_lock(
    authority: &MutationAuthority,
    root: &Path,
    operation: &str,
) -> Result<std::fs::File, Failure> {
    if !authority
        .allowed_operations
        .iter()
        .any(|allowed| allowed == operation)
    {
        return Err(fail("permission-denied", "Git operation is not authorized"));
    }
    let mut allowed = false;
    for candidate in &authority.allowed_repository_roots {
        if candidate.is_absolute() && candidate.canonicalize()? == root {
            allowed = true;
        }
    }
    if !allowed {
        return Err(fail("permission-denied", "Repository is not authorized"));
    }
    if !authority.state_root.is_absolute() {
        return Err(fail(
            "invalid-authority",
            "Expected absolute service state root",
        ));
    }
    use sha2::Digest;
    let directory = authority.state_root.join("git-locks");
    std::fs::create_dir_all(&directory)?;
    let key = format!(
        "{:x}",
        sha2::Sha256::digest(root.as_os_str().as_encoded_bytes())
    );
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(directory.join(key))?;
    let start = std::time::Instant::now();
    loop {
        match rustix::fs::flock(&file, rustix::fs::FlockOperation::NonBlockingLockExclusive) {
            Ok(()) => return Ok(file),
            Err(error) if error == rustix::io::Errno::WOULDBLOCK => {
                if start.elapsed() > Duration::from_secs(15) {
                    return Err(fail("busy", "Repository mutation is busy"));
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
            Err(error) => return Err(std::io::Error::from(error).into()),
        }
    }
}

pub async fn mutate(
    root: &Path,
    method: &str,
    request: Query,
    authority: Option<&MutationAuthority>,
) -> Result<Value, Failure> {
    let authority =
        authority.ok_or_else(|| fail("permission-denied", "Git mutations are disabled"))?;
    if request.workspace_id.is_empty() {
        return Err(fail("invalid-request", "Missing workspace identity"));
    }
    let root = require_repository(root, &request).await?;
    let operation = match method {
        "gitCommit" => "commit",
        "gitPush" => "push",
        "gitChangeBranch" => match request.operation.as_deref() {
            Some("create") => "branch.create",
            Some("switch") => "branch.switch",
            _ => return Err(fail("invalid-request", "Unknown branch operation")),
        },
        _ => return Err(fail("unsupported", "Unknown Git mutation")),
    };
    let _lock = mutation_lock(authority, &root, operation).await?;
    match method {
        "gitChangeBranch" => {
            let name = request
                .name
                .as_deref()
                .filter(|name| !name.is_empty() && !name.starts_with('-') && !name.contains('\0'))
                .ok_or_else(|| fail("invalid-request", "Expected branch name"))?;
            checked(run(&root, &["check-ref-format", "--branch", name]).await?)?;
            let create = operation == "branch.create";
            let args = if create {
                vec!["switch", "-c", name]
            } else {
                vec!["switch", name]
            };
            checked(run(&root, &args).await?)?;
            Ok(json!({"branch":name,"created":create,"contractVersion":"1"}))
        }
        "gitCommit" => {
            let message = request
                .message
                .as_deref()
                .filter(|m| !m.trim().is_empty() && m.len() <= 10000 && !m.contains('\0'))
                .ok_or_else(|| fail("invalid-request", "Expected commit message"))?;
            let paths = request
                .paths
                .as_deref()
                .filter(|p| !p.is_empty())
                .ok_or_else(|| fail("invalid-request", "Expected selected files"))?;
            for path in paths {
                if path.is_empty()
                    || path.contains('\0')
                    || Path::new(path).is_absolute()
                    || path.split(['/', '\\']).any(|p| p == "..")
                {
                    return Err(fail("invalid-request", "Invalid selected path"));
                }
            }
            let state = status(&root).await?;
            let changes = state["changes"].as_array().unwrap();
            if changes.iter().any(|c| c["isConflict"] == true) {
                return Err(fail("conflicts", "Resolve conflicts before committing"));
            }
            let mut effective = std::collections::BTreeSet::new();
            let mut untracked = Vec::new();
            for path in paths {
                let change = changes
                    .iter()
                    .find(|c| c["path"] == *path)
                    .ok_or_else(|| fail("stale-selection", "Selected file is no longer changed"))?;
                effective.insert(path.as_str());
                if let Some(original) = change["originalPath"].as_str() {
                    effective.insert(original);
                }
                if change["kind"] == "untracked" {
                    untracked.push(path.as_str());
                }
            }
            if !untracked.is_empty() {
                let mut args = vec!["--literal-pathspecs", "add", "--intent-to-add", "--"];
                args.extend(untracked.iter().copied());
                checked(run(&root, &args).await?)?;
            }
            let mut args = vec![
                "--literal-pathspecs",
                "commit",
                "--only",
                "--message",
                message,
                "--",
            ];
            args.extend(effective);
            // Uncertain cancellation does not roll back an index that may belong to a completed commit.
            let result = run(&root, &args).await?;
            if result.code != Some(0) && !untracked.is_empty() {
                let mut cleanup = vec![
                    "--literal-pathspecs",
                    "update-index",
                    "--force-remove",
                    "--",
                ];
                cleanup.extend(untracked);
                checked(run(&root, &cleanup).await?)?;
            }
            checked(result)?;
            let commit = checked(run(&root, &["rev-parse", "HEAD"]).await?)?
                .trim()
                .to_owned();
            let metadata =
                checked(run(&root, &["show", "-s", "--format=%h%x00%s", &commit]).await?)?;
            let (short, subject) = metadata
                .trim_end_matches('\n')
                .split_once('\0')
                .ok_or_else(|| fail("invalid-output", "Missing commit metadata"))?;
            Ok(
                json!({"commit":commit,"shortCommit":short,"subject":subject,"committedPaths":paths.iter().collect::<std::collections::BTreeSet<_>>(),"contractVersion":"1"}),
            )
        }
        "gitPush" => {
            let remote = request
                .remote
                .as_deref()
                .filter(|r| !r.is_empty() && !r.starts_with('-') && !r.contains('\0'))
                .ok_or_else(|| fail("invalid-request", "Expected configured remote"))?;
            let state = status(&root).await?;
            if !state["remotes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r == remote)
            {
                return Err(fail("invalid-remote", "Remote is not configured"));
            }
            let branch = state["branch"]
                .as_str()
                .ok_or_else(|| fail("no-branch", "Push needs a branch"))?;
            let head = state["head"]
                .as_str()
                .ok_or_else(|| fail("no-branch", "Push needs a committed branch"))?;
            if state["changes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["isConflict"] == true)
            {
                return Err(fail("conflicts", "Resolve conflicts before pushing"));
            }
            let upstream = format!("{remote}/{branch}");
            if state["upstream"].as_str().is_some_and(|s| s != upstream) {
                return Err(fail(
                    "upstream-mismatch",
                    "Configured upstream differs from target",
                ));
            }
            checked(
                run_deadline(
                    &root,
                    &[
                        "-c",
                        &format!("remote.{remote}.mirror=false"),
                        "push",
                        "--porcelain",
                        "--no-force",
                        "--no-mirror",
                        "--no-follow-tags",
                        "--recurse-submodules=no",
                        remote,
                        &format!("{head}:refs/heads/{branch}"),
                    ],
                    Duration::from_secs(120),
                )
                .await?,
            )?;
            let created = state["upstream"].is_null();
            if created {
                checked(
                    run(
                        &root,
                        &["config", &format!("branch.{branch}.remote"), remote],
                    )
                    .await?,
                )?;
                checked(
                    run(
                        &root,
                        &[
                            "config",
                            &format!("branch.{branch}.merge"),
                            &format!("refs/heads/{branch}"),
                        ],
                    )
                    .await?,
                )?;
            }
            Ok(
                json!({"remote":remote,"branch":branch,"upstream":upstream,"head":head,"createdUpstream":created,"contractVersion":"1"}),
            )
        }
        _ => unreachable!(),
    }
}
