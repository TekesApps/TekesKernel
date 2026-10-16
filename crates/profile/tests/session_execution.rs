use profile::{ConfigRepository, ExecutionConfig};

fn input(cwd: &std::path::Path) -> ExecutionConfig {
    serde_json::from_value(serde_json::json!({"cwd":cwd})).unwrap()
}

#[test]
fn execution_has_no_management_registry_dependency_and_keeps_its_ceiling() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let repo = ConfigRepository::open(root.path()).unwrap();
    for path in [
        "config/environments.json",
        "workspaces/broken/workspace.json",
        "config/folders.json",
    ] {
        let path = root.path().join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"corrupt management input").unwrap();
    }
    let execution = repo.normalize_execution(&input(project.path())).unwrap();
    let first = repo.resolve_execution("session", &execution, None).unwrap();
    assert_eq!(first.execution_cwd(), Some(execution.cwd.as_str()));
    assert!(!first.execution.policy.network);
    assert!(first.execution.policy.writable_roots.is_empty());
    assert!(first.execution.policy.allowed_tools.is_empty());
    assert!(
        serde_json::to_value(&first).unwrap()["revisions"]
            .get("environments")
            .is_none()
    );
    assert_eq!(
        first,
        repo.resolve_execution("session", &execution, None).unwrap()
    );
}

#[test]
fn execution_rejects_protected_roots_and_writes_outside_read_authority() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let repo = ConfigRepository::open(root.path()).unwrap();
    assert!(repo.normalize_execution(&input(root.path())).is_err());
    assert!(
        repo.normalize_execution(&input(std::path::Path::new("/")))
            .is_err()
    );
    let mut execution = input(project.path());
    execution.writable_roots = vec![other.path().to_str().unwrap().to_owned()];
    assert!(repo.normalize_execution(&execution).is_err());
    execution.read_roots = execution.writable_roots.clone();
    assert!(repo.normalize_execution(&execution).is_ok());
}

#[test]
fn recorded_roots_fail_closed_if_replaced_by_an_alias() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let first = project.path().join("first");
    let second = project.path().join("second");
    std::fs::create_dir(&first).unwrap();
    std::fs::create_dir(&second).unwrap();
    let repo = ConfigRepository::open(root.path()).unwrap();
    let execution = repo.normalize_execution(&input(&first)).unwrap();
    std::fs::remove_dir(&first).unwrap();
    std::os::unix::fs::symlink(&second, &first).unwrap();
    assert!(repo.resolve_execution("session", &execution, None).is_err());
}

#[test]
fn session_instructions_read_explicit_roots_without_executing_project_code() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let additional = tempfile::tempdir().unwrap();
    let repo = ConfigRepository::open(root.path()).unwrap();
    std::fs::write(project.path().join("AGENTS.md"), "PRIMARY_PROJECT_RULE").unwrap();
    std::fs::write(
        additional.path().join("AGENTS.md"),
        "EXPLICIT_READ_ROOT_RULE",
    )
    .unwrap();
    std::fs::create_dir_all(project.path().join(".agent/skills")).unwrap();
    std::fs::write(
        project.path().join(".agent/skills/ambient.md"),
        "AMBIENT_CODE",
    )
    .unwrap();
    let mut execution = input(project.path());
    execution.read_roots = vec![additional.path().to_str().unwrap().into()];
    let execution = repo.normalize_execution(&execution).unwrap();
    let scope = profile::SessionScope::from_genesis_value(
        root.path().join("threads/session"),
        &serde_json::json!({"thread":"session","execution":execution}),
    )
    .unwrap();
    let config = scope.base_config(&repo).unwrap();
    let snapshot = scope
        .instructions(&repo, &root.path().join("host-agents"), &config)
        .unwrap()
        .capture()
        .unwrap();
    assert!(
        snapshot
            .sources
            .iter()
            .any(|s| s.content == "PRIMARY_PROJECT_RULE")
    );
    assert!(
        snapshot
            .sources
            .iter()
            .any(|s| s.content == "EXPLICIT_READ_ROOT_RULE")
    );
    assert!(snapshot.effective.skills.is_empty());
    assert!(snapshot.effective.hooks.is_empty());
}
