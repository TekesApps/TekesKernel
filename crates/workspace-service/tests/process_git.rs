use serde_json::{Value, json};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

async fn call(root: &Path, method: &str, request: Value) -> Value {
    workspace_service::process::invoke(
        Path::new(env!("CARGO_BIN_EXE_tekes-workspace-service")),
        root,
        method,
        request,
        Duration::from_secs(10),
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn committed_and_branch_diffs_resolve_revisions_without_options() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.name", "Kernel Acceptance"]);
    git(
        root,
        &["config", "user.email", "kernel-test@example.invalid"],
    );
    std::fs::write(root.join("tracked"), "initial\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "initial"]);
    let initial = git(root, &["rev-parse", "HEAD"]);
    std::fs::write(root.join("tracked"), "committed\n").unwrap();
    git(root, &["commit", "-am", "second"]);
    std::fs::write(root.join("tracked"), "working\n").unwrap();
    for (scope, revision, added, excluded) in [
        ("committed", "HEAD", "+committed", "+working"),
        ("committed", initial.as_str(), "+initial", "+committed"),
        ("branch", initial.as_str(), "+working", "+committed"),
    ] {
        let response = call(
            root,
            "gitDiff",
            json!({"workspaceId":"w", "scope":scope, "revision":revision}),
        )
        .await;
        let patch = response["result"]["unifiedDiff"]
            .as_str()
            .expect("diff response");
        assert!(
            patch.contains(added) && !patch.contains(excluded),
            "{patch}"
        );
    }
    let response = call(
        root,
        "gitDiff",
        json!({"workspaceId":"w", "scope":"committed", "revision":"--all"}),
    )
    .await;
    assert!(response["error"].is_object(), "{response}");
}

#[tokio::test]
async fn history_pages_are_pinned_across_new_commits() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.name", "Kernel Acceptance"]);
    git(
        root,
        &["config", "user.email", "kernel-test@example.invalid"],
    );
    let empty = call(root, "gitLog", json!({"workspaceId":"w"})).await;
    assert_eq!(empty["result"]["commits"], json!([]));
    for subject in ["first", "second"] {
        git(root, &["commit", "--allow-empty", "-m", subject]);
    }
    let first = call(root, "gitLog", json!({"workspaceId":"w", "limit":1})).await;
    assert_eq!(first["result"]["commits"][0]["subject"], "second");
    assert_eq!(first["result"]["nextOffset"], 1);
    git(root, &["commit", "--allow-empty", "-m", "third"]);
    let next = call(
        root,
        "gitLog",
        json!({"workspaceId":"w", "limit":1, "offset":1, "head":first["result"]["head"]}),
    )
    .await;
    assert_eq!(next["result"]["commits"][0]["subject"], "first");
    assert!(next["result"]["nextOffset"].is_null());
    assert_eq!(
        call(root, "gitLog", json!({"workspaceId":"w","head":"--all"})).await["error"]["code"],
        "invalid-request"
    );
}

#[tokio::test]
async fn repository_lookup_distinguishes_non_repo_and_parent_escape() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    assert_eq!(
        call(root, "gitRepository", json!({"workspaceId":"w"})).await["result"]["isRepository"],
        false
    );
    git(root, &["init", "-b", "main"]);
    let repository = call(root, "gitRepository", json!({"workspaceId":"w"})).await;
    assert_eq!(
        repository["result"]["repositoryRoot"],
        root.canonicalize().unwrap().to_string_lossy().as_ref()
    );
    std::fs::create_dir(root.join("nested")).unwrap();
    assert_eq!(
        call(
            &root.join("nested"),
            "gitRepository",
            json!({"workspaceId":"w"})
        )
        .await["error"]["code"],
        "repository-outside-workspace"
    );
}

#[tokio::test]
async fn status_preserves_rename_and_unusual_paths() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.name", "Kernel Acceptance"]);
    git(
        root,
        &["config", "user.email", "kernel-test@example.invalid"],
    );
    std::fs::write(root.join("old file\nname"), "initial\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "initial"]);
    git(root, &["mv", "old file\nname", "new file\nname"]);
    std::fs::write(root.join("untracked\nfile"), "new\n").unwrap();
    let state = call(root, "gitStatus", json!({"workspaceId":"w"})).await;
    let result = &state["result"];
    assert_eq!(result["branch"], "main");
    assert_eq!(result["lastCommitSubject"], "initial");
    let changes = result["changes"].as_array().unwrap();
    let rename = changes.iter().find(|c| c["kind"] == "renamed").unwrap();
    assert_eq!(rename["path"], "new file\nname");
    assert_eq!(rename["originalPath"], "old file\nname");
    assert!(
        changes
            .iter()
            .any(|c| c["path"] == "untracked\nfile" && c["kind"] == "untracked")
    );
}

#[tokio::test]
async fn diff_and_branches_match_scopes_without_pathspec_expansion() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.name", "Kernel Acceptance"]);
    git(
        root,
        &["config", "user.email", "kernel-test@example.invalid"],
    );
    std::fs::write(root.join("tracked"), "alpha\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "initial"]);
    git(root, &["branch", "feature"]);
    std::fs::write(root.join("tracked"), "beta\n").unwrap();
    git(root, &["add", "tracked"]);
    std::fs::write(root.join("tracked"), "gamma\n").unwrap();
    std::fs::write(root.join(":(glob)*"), "literal\n").unwrap();
    let branches = call(root, "gitBranches", json!({"workspaceId":"w"})).await;
    assert_eq!(branches["result"]["defaultBranch"], "main");
    assert_eq!(branches["result"]["branches"].as_array().unwrap().len(), 2);
    for (scope, added, deleted) in [
        ("staged", "+beta", "-alpha"),
        ("unstaged", "+gamma", "-beta"),
        ("all", "+gamma", "-alpha"),
    ] {
        let response = call(
            root,
            "gitDiff",
            json!({"workspaceId":"w","scope":scope,"paths":["tracked"]}),
        )
        .await;
        let patch = response["result"]["unifiedDiff"].as_str().unwrap();
        assert!(patch.contains(added) && patch.contains(deleted), "{patch}");
    }
    let literal = call(
        root,
        "gitDiff",
        json!({"workspaceId":"w","paths":[":(glob)*"]}),
    )
    .await;
    let patch = literal["result"]["unifiedDiff"].as_str().unwrap();
    assert!(patch.contains("+literal"));
    assert!(!patch.contains("+gamma"));
    assert_eq!(
        call(
            root,
            "gitDiff",
            json!({"workspaceId":"w","paths":["../escape"]})
        )
        .await["error"]["code"],
        "invalid-request"
    );
}

#[tokio::test]
async fn navigator_discovers_clean_children_and_outgoing_commit_files() {
    let workspace = tempfile::tempdir().unwrap();
    let remote = tempfile::tempdir().unwrap();
    git(remote.path(), &["init", "--bare"]);
    for name in ["first", "second"] {
        let root = workspace.path().join(name);
        std::fs::create_dir(&root).unwrap();
        git(&root, &["init", "-b", "main"]);
        git(&root, &["config", "user.name", "Kernel Acceptance"]);
        git(
            &root,
            &["config", "user.email", "kernel-test@example.invalid"],
        );
        std::fs::write(root.join("proof"), "first\n").unwrap();
        git(&root, &["add", "."]);
        git(&root, &["commit", "-m", "initial"]);
    }
    let root = workspace.path().join("first");
    git(
        &root,
        &["remote", "add", "origin", remote.path().to_str().unwrap()],
    );
    git(&root, &["push", "-u", "origin", "main"]);
    std::fs::write(root.join("proof"), "second\n").unwrap();
    git(&root, &["commit", "-am", "outgoing"]);
    std::fs::write(root.join("proof"), "third\n").unwrap();
    let result = call(
        workspace.path(),
        "gitChangesNavigator",
        json!({"workspaceId":"w","includeClean":true}),
    )
    .await;
    let repositories = result["result"]["repositories"].as_array().unwrap();
    assert_eq!(repositories.len(), 2);
    let first = &repositories[0];
    assert_eq!(first["name"], "first");
    assert_eq!(first["uncommitted"][0]["additions"], 1);
    assert_eq!(first["uncommitted"][0]["deletions"], 1);
    assert_eq!(first["unpushedCommits"][0]["subject"], "outgoing");
    assert_eq!(first["unpushedCommits"][0]["files"][0]["path"], "proof");
    assert_eq!(first["unpushedCommits"][0]["files"][0]["additions"], 1);
    let dirty = call(
        workspace.path(),
        "gitChangesNavigator",
        json!({"workspaceId":"w"}),
    )
    .await;
    assert_eq!(dirty["result"]["repositories"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn container_workspace_selects_only_child_but_requires_choice_for_multiple() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path();
    let first = root.join("first");
    std::fs::create_dir(&first).unwrap();
    git(&first, &["init", "-b", "main"]);
    assert_eq!(
        call(root, "gitRepository", json!({"workspaceId":"w"})).await["result"]["isRepository"],
        false
    );
    let selected = call(root, "gitStatus", json!({"workspaceId":"w"})).await;
    assert!(selected.get("result").is_some(), "{selected}");
    let explicit = call(
        root,
        "gitStatus",
        json!({"workspaceId":"w", "repositoryPath":""}),
    )
    .await;
    assert_eq!(explicit["error"]["code"], "not-repository", "{explicit}");
    let second = root.join("second");
    std::fs::create_dir(&second).unwrap();
    git(&second, &["init", "-b", "main"]);
    let ambiguous = call(root, "gitStatus", json!({"workspaceId":"w"})).await;
    assert_eq!(
        ambiguous["error"]["code"], "repository-selection-required",
        "{ambiguous}"
    );
    let explicit = call(
        root,
        "gitStatus",
        json!({"workspaceId":"w", "repositoryPath":"second"}),
    )
    .await;
    assert!(explicit.get("result").is_some(), "{explicit}");
}
