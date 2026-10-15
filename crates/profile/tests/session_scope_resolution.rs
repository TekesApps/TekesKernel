use std::path::{Path, PathBuf};

use profile::{
    ConfigRepository, Environment, EnvironmentsConfig, ScopeSelection, WorkspaceConfig,
    WorkspaceFolder, WorkspacePolicy,
};

struct Fixture {
    _root: tempfile::TempDir,
    repository: ConfigRepository,
    first: String,
    second: String,
    toolchain: String,
}

fn canonical(path: &Path) -> String {
    path.canonicalize().unwrap().to_string_lossy().into_owned()
}

/// A workspace with two folders, both writable, network on, a 600 s cap.
fn fixture() -> Fixture {
    let root = tempfile::tempdir().unwrap();
    let make = |name: &str| {
        let path: PathBuf = root.path().join(name);
        std::fs::create_dir_all(&path).unwrap();
        canonical(&path)
    };
    let (first, second, toolchain) = (make("first"), make("second"), make("toolchain"));
    let repository = ConfigRepository::open(root.path().join("kernel")).unwrap();
    repository
        .publish_workspace(
            0,
            &WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "workspace-1".to_owned(),
                name: "Workspace".to_owned(),
                cwd: Vec::new(),
                folders: vec![
                    WorkspaceFolder {
                        id: "folder-0001".to_owned(),
                        path: first.clone(),
                    },
                    WorkspaceFolder {
                        id: "folder-0002".to_owned(),
                        path: second.clone(),
                    },
                ],
                policy: Some(WorkspacePolicy {
                    writable_roots: vec![first.clone(), second.clone()],
                    network: true,
                    max_wall_seconds: Some(600),
                    ..WorkspacePolicy::default()
                }),
            },
        )
        .unwrap();
    Fixture {
        _root: root,
        repository,
        first,
        second,
        toolchain,
    }
}

fn local<'a>(folders: Option<&'a [String]>) -> ScopeSelection<'a> {
    ScopeSelection {
        environment: "local",
        folders,
    }
}

/// Recorded folder ids narrow `cwd` and writable roots to the session's own
/// folders, so a folder the workspace gains later does not reach it.
#[test]
fn recorded_folders_narrow_cwd_and_writable_roots() {
    let fixture = fixture();
    let ids = vec!["folder-0001".to_owned()];
    let snapshot = fixture
        .repository
        .resolve_scoped("workspace-1", None, Some("folder-0001"), local(Some(&ids)))
        .unwrap();
    assert_eq!(snapshot.workspace.cwd, vec![fixture.first.clone()]);
    assert_eq!(
        snapshot.workspace.policy.writable_roots,
        vec![fixture.first.clone()]
    );
    assert_eq!(snapshot.workspace.folders, Some(ids));
    assert_eq!(snapshot.workspace.environment.as_deref(), Some("local"));
    snapshot
        .canonical_bytes()
        .expect("narrowed snapshot stays valid");

    // Without recorded folders every workspace folder stays.
    let wide = fixture
        .repository
        .resolve_scoped("workspace-1", None, Some("folder-0001"), local(None))
        .unwrap();
    assert_eq!(
        wide.workspace.cwd,
        vec![fixture.first.clone(), fixture.second.clone()]
    );
    assert_eq!(wide.workspace.folders, None);
}

#[test]
fn a_binding_outside_the_recorded_folders_or_an_unknown_folder_is_refused() {
    let fixture = fixture();
    let ids = vec!["folder-0001".to_owned()];
    assert!(
        fixture
            .repository
            .resolve_scoped("workspace-1", None, Some("folder-0002"), local(Some(&ids)))
            .is_err()
    );
    let unknown = vec!["folder-0009".to_owned()];
    assert!(
        fixture
            .repository
            .resolve_scoped("workspace-1", None, None, local(Some(&unknown)))
            .is_err()
    );
}

/// The built-in `local` leaves the workspace policy as is; an authored
/// environment can switch the network off, lower the wall cap and add a
/// read-only toolchain, but never widen writes. An unknown one fails closed.
#[test]
fn environments_meet_the_workspace_policy() {
    let fixture = fixture();
    let local_snapshot = fixture
        .repository
        .resolve_scoped("workspace-1", None, Some("folder-0001"), local(None))
        .unwrap();
    assert!(local_snapshot.workspace.policy.network);
    assert_eq!(local_snapshot.workspace.policy.max_wall_seconds, Some(600));
    assert!(local_snapshot.workspace.policy.toolchain_roots.is_empty());
    assert_eq!(local_snapshot.revisions.environments, Some(0));

    fixture
        .repository
        .publish_environments(
            0,
            &EnvironmentsConfig {
                format: 1,
                revision: 1,
                environments: vec![Environment {
                    id: "offline".to_owned(),
                    network: false,
                    toolchain_roots: vec![fixture.toolchain.clone()],
                    max_wall_seconds: Some(60),
                }],
            },
        )
        .unwrap();
    let offline = fixture
        .repository
        .resolve_scoped(
            "workspace-1",
            None,
            Some("folder-0001"),
            ScopeSelection {
                environment: "offline",
                folders: None,
            },
        )
        .unwrap();
    let policy = &offline.workspace.policy;
    assert!(!policy.network);
    assert_eq!(policy.max_wall_seconds, Some(60));
    assert_eq!(policy.toolchain_roots, vec![fixture.toolchain.clone()]);
    assert_eq!(
        policy.writable_roots,
        local_snapshot.workspace.policy.writable_roots
    );
    assert_eq!(offline.revisions.environments, Some(1));

    // `local` still exists beside authored entries.
    assert!(
        fixture
            .repository
            .resolve_scoped("workspace-1", None, Some("folder-0001"), local(None))
            .is_ok()
    );
    assert!(
        fixture
            .repository
            .resolve_scoped(
                "workspace-1",
                None,
                Some("folder-0001"),
                ScopeSelection {
                    environment: "missing",
                    folders: None,
                },
            )
            .is_err()
    );
}

#[test]
fn environment_documents_are_validated() {
    let fixture = fixture();
    let environment = |id: &str| Environment {
        id: id.to_owned(),
        network: true,
        toolchain_roots: Vec::new(),
        max_wall_seconds: None,
    };
    let publish = |environments: Vec<Environment>| {
        fixture.repository.publish_environments(
            0,
            &EnvironmentsConfig {
                format: 1,
                revision: 1,
                environments,
            },
        )
    };
    assert!(publish(vec![environment("a"), environment("a")]).is_err());
    assert!(publish(vec![environment("")]).is_err());
    let mut relative = environment("a");
    relative.toolchain_roots = vec!["relative/bin".to_owned()];
    assert!(publish(vec![relative]).is_err());
    let mut zero = environment("a");
    zero.max_wall_seconds = Some(0);
    assert!(publish(vec![zero]).is_err());
    assert_eq!(fixture.repository.environments().unwrap().revision, 0);
}
