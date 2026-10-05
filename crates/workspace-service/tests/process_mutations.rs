use serde_json::{Value, json};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn git(root: &Path, args: &[&str]) -> String {
    let result = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap().trim().to_owned()
}
fn initialize(root: &Path) {
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.name", "Kernel Acceptance"]);
    git(
        root,
        &["config", "user.email", "kernel-test@example.invalid"],
    );
    std::fs::write(root.join("selected"), "initial\n").unwrap();
    std::fs::write(root.join("unrelated"), "initial\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "initial"]);
}
fn call(root: &Path, authority: Option<&Path>, method: &str, request: Value) -> Value {
    let child = start(root, authority, method, request);
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn start(
    root: &Path,
    authority: Option<&Path>,
    method: &str,
    request: Value,
) -> std::process::Child {
    let mut command = Command::new(env!("CARGO_BIN_EXE_tekes-workspace-service"));
    command.arg("--root").arg(root);
    if let Some(path) = authority {
        command.arg("--authority-file").arg(path);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&json!({"method":method,"request":request})).unwrap())
        .unwrap();
    child
}
fn authority(state: &Path, root: &Path) -> std::path::PathBuf {
    let path = state.join("authority.json");
    std::fs::write(&path,serde_json::to_vec(&json!({"stateRoot":state,"allowedOperations":["commit","branch.create","branch.switch","push"],"allowedRepositoryRoots":[root]})).unwrap()).unwrap();
    path
}

#[test]
fn authorized_selected_commit_branch_and_bare_push() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let remote = tempfile::tempdir().unwrap();
    initialize(root.path());
    git(remote.path(), &["init", "--bare"]);
    let auth = authority(state.path(), root.path());
    std::fs::write(root.path().join("selected"), "selected change\n").unwrap();
    std::fs::write(root.path().join("unrelated"), "staged unrelated\n").unwrap();
    git(root.path(), &["add", "unrelated"]);
    let commit = call(
        root.path(),
        Some(&auth),
        "gitCommit",
        json!({"workspaceId":"w","paths":["selected"],"message":"selected only"}),
    );
    assert_eq!(commit["result"]["subject"], "selected only", "{commit}");
    assert_eq!(
        git(root.path(), &["show", "HEAD:selected"]),
        "selected change"
    );
    assert_eq!(git(root.path(), &["show", "HEAD:unrelated"]), "initial");
    assert_eq!(
        git(root.path(), &["show", ":unrelated"]),
        "staged unrelated"
    );
    let created = call(
        root.path(),
        Some(&auth),
        "gitChangeBranch",
        json!({"workspaceId":"w","operation":"create","name":"feature"}),
    );
    assert_eq!(created["result"]["created"], true, "{created}");
    git(
        root.path(),
        &["remote", "add", "origin", remote.path().to_str().unwrap()],
    );
    let pushed = call(
        root.path(),
        Some(&auth),
        "gitPush",
        json!({"workspaceId":"w","remote":"origin"}),
    );
    assert_eq!(pushed["result"]["branch"], "feature", "{pushed}");
    assert_eq!(
        git(remote.path(), &["rev-parse", "refs/heads/feature"]),
        commit["result"]["commit"].as_str().unwrap()
    );
    let switched = call(
        root.path(),
        Some(&auth),
        "gitChangeBranch",
        json!({"workspaceId":"w","operation":"switch","name":"main"}),
    );
    assert_eq!(switched["result"]["branch"], "main", "{switched}");
}

#[test]
fn default_and_wrong_root_denied_and_hook_failure_preserves_index() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let wrong = tempfile::tempdir().unwrap();
    initialize(root.path());
    let request = json!({"workspaceId":"w","operation":"create","name":"must-not-exist"});
    assert_eq!(
        call(root.path(), None, "gitChangeBranch", request.clone())["error"]["code"],
        "permission-denied"
    );
    let auth = authority(state.path(), wrong.path());
    assert_eq!(
        call(root.path(), Some(&auth), "gitChangeBranch", request)["error"]["code"],
        "permission-denied"
    );
    assert_eq!(
        git(root.path(), &["branch", "--list", "must-not-exist"]),
        ""
    );
    let auth = authority(state.path(), root.path());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let hook = root.path().join(".git/hooks/pre-commit");
        std::fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o700)).unwrap();
        std::fs::write(root.path().join("new-file"), "new\n").unwrap();
        std::fs::write(root.path().join("unrelated"), "keep staged\n").unwrap();
        git(root.path(), &["add", "unrelated"]);
        let before = git(root.path(), &["ls-files", "--stage"]);
        let result = call(
            root.path(),
            Some(&auth),
            "gitCommit",
            json!({"workspaceId":"w","paths":["new-file"],"message":"rejected"}),
        );
        assert_eq!(result["error"]["code"], "git-failed", "{result}");
        assert_eq!(git(root.path(), &["ls-files", "--stage"]), before);
        assert_eq!(
            std::fs::read_to_string(root.path().join("new-file")).unwrap(),
            "new\n"
        );
    }
}

#[cfg(unix)]
#[test]
fn concurrent_mutations_wait_for_commit_and_recover_after_hook_failure() {
    use std::os::unix::fs::PermissionsExt;
    use std::time::{Duration, Instant};
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    initialize(root.path());
    let auth = authority(state.path(), root.path());
    let hook = root.path().join(".git/hooks/pre-commit");
    // The hook establishes that the first real helper owns the repository lock.
    // Bound its wait so a failing assertion cannot leave a permanent child behind.
    std::fs::write(&hook, "#!/bin/sh\n[ \"$1\" = --warm ] && exit 0\ntouch .git/hook-entered\ni=0\nwhile [ ! -f .git/hook-release ]; do\n  i=$((i + 1))\n  [ \"$i\" -lt 100 ] || exit 1\n  sleep 0.05\ndone\nexit 1\n").unwrap();
    std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o700)).unwrap();
    // macOS assesses a new executable on its first exec, serially across the
    // machine; under a full parallel test run that alone could outlast the 4 s
    // bound below. Take the first exec here, untimed (git passes no arguments).
    assert!(
        Command::new(&hook)
            .arg("--warm")
            .status()
            .unwrap()
            .success()
    );
    std::fs::write(root.path().join("selected"), "pending change\n").unwrap();
    let head = git(root.path(), &["rev-parse", "HEAD"]);
    let first = start(
        root.path(),
        Some(&auth),
        "gitCommit",
        json!({"workspaceId":"w","paths":["selected"],"message":"will fail"}),
    );
    let deadline = Instant::now() + Duration::from_secs(4);
    while !root.path().join(".git/hook-entered").exists() {
        assert!(Instant::now() < deadline, "commit never entered hook");
        std::thread::sleep(Duration::from_millis(10));
    }
    let mut second = start(
        root.path(),
        Some(&auth),
        "gitChangeBranch",
        json!({"workspaceId":"w","operation":"create","name":"after-failure"}),
    );
    std::thread::sleep(Duration::from_millis(200));
    assert!(
        second.try_wait().unwrap().is_none(),
        "second mutation bypassed the lock"
    );
    assert_eq!(git(root.path(), &["branch", "--list", "after-failure"]), "");
    std::fs::write(root.path().join(".git/hook-release"), "release").unwrap();
    let failed: Value = serde_json::from_slice(&first.wait_with_output().unwrap().stdout).unwrap();
    assert_eq!(failed["error"]["code"], "git-failed", "{failed}");
    let recovered: Value =
        serde_json::from_slice(&second.wait_with_output().unwrap().stdout).unwrap();
    assert_eq!(recovered["result"]["created"], true, "{recovered}");
    assert_eq!(git(root.path(), &["rev-parse", "HEAD"]), head);
    assert_eq!(
        git(root.path(), &["branch", "--show-current"]),
        "after-failure"
    );
    assert_eq!(
        std::fs::read_to_string(root.path().join("selected")).unwrap(),
        "pending change\n"
    );
}

#[test]
fn selected_new_literal_paths_and_rename_commit_from_container() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path().join("repo");
    std::fs::create_dir(&root).unwrap();
    let state = tempfile::tempdir().unwrap();
    initialize(&root);
    let auth = authority(state.path(), &root);
    std::fs::write(root.join("unrelated"), "keep staged\n").unwrap();
    git(&root, &["add", "unrelated"]);
    git(&root, &["mv", "selected", "renamed"]);
    let names = [":(glob)*", "-new\nfile"];
    for name in names {
        std::fs::write(root.join(name), "new selected content\n").unwrap();
    }
    std::fs::write(root.join("unselected-new"), "must remain untracked\n").unwrap();
    let result = call(
        workspace.path(),
        Some(&auth),
        "gitCommit",
        json!({
            "workspaceId":"w", "paths":["renamed", names[0], names[1]], "message":"literal selections"
        }),
    );
    assert_eq!(
        result["result"]["subject"], "literal selections",
        "{result}"
    );
    for name in names {
        assert_eq!(
            git(&root, &["show", &format!("HEAD:{name}")]),
            "new selected content"
        );
    }
    assert_eq!(git(&root, &["show", "HEAD:renamed"]), "initial");
    assert_eq!(
        git(&root, &["ls-tree", "--name-only", "HEAD", "--", "selected"]),
        ""
    );
    assert_eq!(git(&root, &["show", "HEAD:unrelated"]), "initial");
    assert_eq!(git(&root, &["show", ":unrelated"]), "keep staged");
    assert_eq!(
        git(&root, &["ls-files", "--others", "--exclude-standard"]),
        "unselected-new"
    );
}
