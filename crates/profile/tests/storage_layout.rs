use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::{PermissionsExt, symlink};

use profile::{ConfigRepository, InstructionResolver, ResourceCatalog};

fn canonical_line(value: &serde_json::Value) -> Vec<u8> {
    let mut bytes = serde_json_canonicalizer::to_vec(value).expect("canonical workspace");
    bytes.push(b'\n');
    bytes
}

#[test]
fn retired_workspace_directory_is_not_opened() {
    // The pre-release flat `workspaces/<id>.json` layout is no longer migrated in place.
    // A file where a workspace directory belongs fails the open with a typed path error
    // instead of being silently rewritten.
    let root = tempfile::tempdir().expect("root");
    let workspaces = root.path().join("workspaces");
    fs::create_dir(&workspaces).expect("workspaces");
    let stray = workspaces.join("workspace-1.json");
    fs::write(
        &stray,
        canonical_line(&serde_json::json!({
            "cwd":[root.path().display().to_string()],
            "format":1,
            "id":"workspace-1",
            "name":"Workspace",
            "revision":1
        })),
    )
    .expect("stray workspace file");

    ConfigRepository::open(root.path()).expect("retired directory is not read");
    assert!(stray.is_file());
    assert!(!root.path().join("workspaces/workspace-1").exists());
}

#[test]
fn repository_rejects_symlinked_root_and_authority_directories() {
    let parent = tempfile::tempdir().expect("parent");
    let real_root = parent.path().join("real-root");
    fs::create_dir(&real_root).expect("real root");
    fs::set_permissions(&real_root, fs::Permissions::from_mode(0o700)).expect("root mode");
    let linked_root = parent.path().join("linked-root");
    symlink(&real_root, &linked_root).expect("root symlink");
    let error = ConfigRepository::open(&linked_root).expect_err("symlinked root");
    assert!(error.to_string().contains("symlink"));

    let root = tempfile::tempdir().expect("root");
    let outside = tempfile::tempdir().expect("outside");
    symlink(outside.path(), root.path().join("config")).expect("config symlink");
    let error = ConfigRepository::open(root.path()).expect_err("symlinked config");
    assert!(error.to_string().contains("not a real directory"));
    assert!(!root.path().join("workspaces").exists());
}

#[test]
fn repository_rejects_dangerously_writable_private_directories_before_hardening() {
    let root = tempfile::tempdir().expect("root");
    let config = root.path().join("config");
    fs::create_dir(&config).expect("config");
    fs::set_permissions(&config, fs::Permissions::from_mode(0o777)).expect("unsafe mode");
    let error = ConfigRepository::open(root.path()).expect_err("dangerous mode");
    assert!(error.to_string().contains("group- or world-writable"));
    assert_eq!(
        fs::metadata(&config).expect("config metadata").mode() & 0o777,
        0o777
    );
}

#[test]
fn repository_hardens_safe_existing_directories_and_reopens_idempotently() {
    let root = tempfile::tempdir().expect("root");
    for path in [root.path().to_path_buf(), root.path().join("config")] {
        if !path.exists() {
            fs::create_dir(&path).expect("private directory");
        }
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("safe broad mode");
    }
    ConfigRepository::open(root.path()).expect("first open");
    ConfigRepository::open(root.path()).expect("repeat open");
    for path in [root.path().to_path_buf(), root.path().join("config")] {
        assert_eq!(fs::metadata(path).expect("metadata").mode() & 0o777, 0o700);
    }
}

#[test]
fn repository_securely_creates_a_missing_root_leaf() {
    let parent = tempfile::tempdir().expect("parent");
    let root = parent.path().join("authority");
    let repository = ConfigRepository::open(&root).expect("create repository leaf");
    assert_eq!(repository.root(), root);
    for path in [root.clone(), root.join("config")] {
        let metadata = fs::symlink_metadata(path).expect("private directory");
        assert!(metadata.is_dir());
        assert!(!metadata.file_type().is_symlink());
        assert_eq!(metadata.mode() & 0o777, 0o700);
    }
    ConfigRepository::open(&root).expect("repeat open");
}

#[test]
fn repository_rejects_an_empty_or_root_only_authority_path() {
    for path in [std::path::Path::new(""), std::path::Path::new("/")] {
        let error = ConfigRepository::open(path).expect_err("root without private leaf");
        assert!(error.to_string().contains("must name a directory"));
    }
}

#[test]
fn user_workspace_and_project_skills_keep_sources_and_collisions_fail() {
    let root = tempfile::tempdir().expect("root");
    let user = root.path().join("user");
    let workspace = root.path().join("workspace-data");
    let project = root.path().join("project");
    for (base, name) in [(&user, "user-skill"), (&workspace, "workspace-skill")] {
        let skill = base.join("skills").join(name);
        fs::create_dir_all(&skill).expect("skill directory");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: test\n---\nbody\n"),
        )
        .expect("skill");
    }
    let project_skill = project.join(".agents/skills/project-skill");
    fs::create_dir_all(&project_skill).expect("project skill");
    fs::write(
        project_skill.join("SKILL.md"),
        "---\nname: project-skill\ndescription: >-\n  shared skill\n  with multiple lines\nmetadata:\n  tools: [read, write]\n---\nbody\n",
    )
    .expect("project skill bytes");
    fs::create_dir_all(project_skill.join("assets")).expect("asset directory");
    fs::write(
        project_skill.join("assets/image.png"),
        [0x89, b'P', b'N', b'G', 0xff],
    )
    .expect("binary skill asset");

    let snapshot = InstructionResolver::new_scoped(&user, &workspace, [&project])
        .capture()
        .expect("three-tier snapshot");
    let catalog = ResourceCatalog::from_snapshot(&snapshot).expect("catalog");
    let sources = catalog
        .skill_summaries()
        .into_iter()
        .map(|skill| (skill.name, skill.source))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(sources.get("user-skill").map(String::as_str), Some("user"));
    assert_eq!(
        sources.get("workspace-skill").map(String::as_str),
        Some("workspace")
    );
    assert!(!sources.contains_key("project-skill"));

    let duplicate = workspace.join("skills/user-skill");
    fs::create_dir_all(&duplicate).expect("duplicate directory");
    fs::write(
        duplicate.join("SKILL.md"),
        "---\nname: user-skill\ndescription: duplicate\n---\nbody\n",
    )
    .expect("duplicate skill");
    let snapshot = InstructionResolver::new_scoped(&user, &workspace, [&project])
        .capture()
        .expect("duplicate snapshot keeps both sources");
    let error = ResourceCatalog::from_snapshot(&snapshot).expect_err("ambiguous skill");
    assert!(
        error
            .to_string()
            .contains("defined by both user and workspace")
    );
}

#[test]
fn project_folders_never_enable_ambient_skills() {
    let root = tempfile::tempdir().expect("root");
    let user = root.path().join("user");
    let workspace = root.path().join("workspace-data");
    fs::create_dir_all(&user).expect("user root");
    fs::create_dir_all(&workspace).expect("workspace root");
    let first = root.path().join("first");
    let second = root.path().join("second");
    for (project, body) in [(&first, "first"), (&second, "second")] {
        let skill = project.join(".agents/skills/shared");
        fs::create_dir_all(&skill).expect("skill directory");
        fs::write(
            skill.join("SKILL.md"),
            format!("---\nname: shared\ndescription: {body}\n---\n{body}\n"),
        )
        .expect("skill bytes");
    }
    let snapshot = InstructionResolver::new_scoped(&user, &workspace, [&first, &second])
        .capture()
        .expect("multi-folder snapshot");
    assert!(
        ResourceCatalog::from_snapshot(&snapshot)
            .unwrap()
            .skill_summaries()
            .is_empty()
    );
}

#[test]
fn agents_directory_is_primary_for_all_project_resources() {
    let root = tempfile::tempdir().expect("root");
    let user = root.path().join("home/.agents");
    let project = root.path().join("project");
    for (path, name, body) in [
        (user.join("skills/user-shared"), "user-shared", "user body"),
        (
            project.join(".agents/skills/review"),
            "review",
            "shared body",
        ),
        (
            project.join(".tekes/skills/review"),
            "review",
            "obsolete body",
        ),
    ] {
        fs::create_dir_all(&path).expect("skill directory");
        fs::write(
            path.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: test\n---\n{body}\n"),
        )
        .expect("skill");
    }
    fs::create_dir_all(project.join(".agents/rules")).expect("rules");
    fs::write(project.join(".agents/rules/review.md"), "shared rules").expect("rule");
    fs::write(project.join("AGENTS.md"), "project instructions").expect("AGENTS");
    let snapshot = InstructionResolver::new_scoped(&user, project.join(".agents"), [&project])
        .capture()
        .expect("shared snapshot");
    let catalog = ResourceCatalog::from_snapshot(&snapshot).expect("catalog");
    assert_eq!(catalog.skill_summaries().len(), 2);
    assert!(
        catalog
            .skill("review")
            .expect("review")
            .body
            .contains("shared body")
    );
    assert!(
        snapshot
            .sources
            .iter()
            .any(|source| source.path == "rules/review.md")
    );
    assert!(
        snapshot
            .sources
            .iter()
            .any(|source| source.path == "AGENTS.md")
    );
}

#[test]
fn preset_skills_name_packages_not_resource_files() {
    let root = tempfile::tempdir().expect("root");
    let user = root.path().join("agent");
    let project = root.path().join("project");
    fs::create_dir_all(user.join("skills/review/scripts")).expect("package");
    fs::create_dir_all(&project).expect("project");
    fs::write(
        user.join("skills/review/SKILL.md"),
        "---\nname: review\ndescription: Review a change\n---\nBody\n",
    )
    .expect("manifest");
    fs::write(user.join("skills/review/scripts/check.sh"), "true\n").expect("companion");
    fs::write(user.join("skills/notes.md"), "Loose notes\n").expect("flat file");
    let snapshot = InstructionResolver::new(&user, [&project])
        .capture()
        .expect("snapshot");
    // The snapshot keys every resource file; the package catalog names packages.
    assert!(snapshot.effective.skills.contains_key("review/SKILL.md"));
    assert_eq!(
        ResourceCatalog::skill_package_names(&snapshot),
        ["review".to_owned()].into_iter().collect()
    );
}
