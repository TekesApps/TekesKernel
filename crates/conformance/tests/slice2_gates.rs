use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};

use profile::{
    ConfigRepository, InstructionResolver, InstructionSettings, InstructionSnapshot, ProfileError,
    ProvidersConfig, SettingsConfig, WorkspaceConfig,
};
use store::AssetStore;
use test_support::{FixtureRoot, read};

#[test]
fn slice2_gate_35_config_contract_fixtures() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let workspace = read(&fixtures.join("config/workspace.canonical.json")).expect("workspace");
    assert_eq!(
        WorkspaceConfig::decode(&workspace)
            .expect("workspace decode")
            .canonical_bytes()
            .expect("workspace encode"),
        workspace
    );
    let providers = read(&fixtures.join("config/providers.canonical.json")).expect("providers");
    assert_eq!(
        ProvidersConfig::decode(&providers)
            .expect("providers decode")
            .canonical_bytes()
            .expect("providers encode"),
        providers
    );
    let web_search = read(&fixtures.join("config/providers-web-search.canonical.json"))
        .expect("web search providers");
    assert_eq!(
        ProvidersConfig::decode(&web_search)
            .expect("web search providers decode")
            .canonical_bytes()
            .expect("web search providers encode"),
        web_search
    );
    assert!(
        ProvidersConfig::decode(
            &read(&fixtures.join("config/providers-web-search-private.invalid.json"))
                .expect("invalid web search endpoint")
        )
        .is_err()
    );
    assert!(
        ProvidersConfig::decode(
            &read(&fixtures.join("config/providers-web-search-mapped-loopback.invalid.json"))
                .expect("invalid mapped-loopback web search endpoint")
        )
        .is_err()
    );
    let settings = read(&fixtures.join("config/settings.canonical.json")).expect("settings");
    assert_eq!(
        SettingsConfig::decode(&settings)
            .expect("settings decode")
            .canonical_bytes()
            .expect("settings encode"),
        settings
    );
    assert!(
        WorkspaceConfig::decode(
            &read(&fixtures.join("config/workspace-relative.invalid.json")).expect("invalid")
        )
        .is_err()
    );
    assert!(
        WorkspaceConfig::decode(
            &read(&fixtures.join("config/workspace-unknown.invalid.json")).expect("invalid")
        )
        .is_err()
    );
    assert!(
        ProvidersConfig::decode(
            &read(&fixtures.join("config/providers-secret.invalid.json")).expect("invalid")
        )
        .is_err()
    );
    assert!(
        WorkspaceConfig::decode(
            &read(&fixtures.join("config/workspace-unsafe-integer.invalid.json")).expect("invalid")
        )
        .is_err()
    );
}

#[test]
fn slice2_gate_36_config_atomic_publish_and_revocation() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("config root");
    let cwd0 = directory.path().join("alpha");
    let cwd1 = directory.path().join("beta");
    fs::create_dir(&cwd0).expect("cwd0");
    fs::create_dir(&cwd1).expect("cwd1");
    let repository = ConfigRepository::open(directory.path()).expect("repository");

    let mut workspace = WorkspaceConfig::decode(
        &read(&fixtures.join("config/workspace.canonical.json")).expect("workspace"),
    )
    .expect("workspace decode");
    workspace.revision = 1;
    workspace.cwd = vec![path_text(&cwd0), path_text(&cwd1)];
    let policy = workspace.policy.as_mut().expect("policy");
    policy.writable_roots = workspace.cwd.clone();
    repository
        .publish_workspace(0, &workspace)
        .expect("workspace publish");

    let mut providers = ProvidersConfig::decode(
        &read(&fixtures.join("config/providers.canonical.json")).expect("providers"),
    )
    .expect("providers decode");
    providers.revision = 1;
    repository
        .publish_providers(0, &providers)
        .expect("providers publish");
    let mut settings = SettingsConfig::decode(
        &read(&fixtures.join("config/settings.canonical.json")).expect("settings"),
    )
    .expect("settings decode");
    settings.revision = 1;
    repository
        .publish_settings(0, &settings)
        .expect("settings publish");

    let first = repository.resolve("ws-main").expect("first snapshot");
    assert_eq!(first.revisions.workspace, 1);
    assert_eq!(first.revisions.providers, 1);
    let assets = AssetStore::new(directory.path().join("thread/assets")).expect("assets");
    let (first_digest, first_asset) = first.publish(&assets).expect("snapshot asset");
    assert_eq!(first_asset.asset, format!("sha256-{first_digest}"));

    let mut reduced_a = workspace.clone();
    reduced_a.revision = 2;
    reduced_a.name = "Reduced A".to_owned();
    let policy = reduced_a.policy.as_mut().expect("policy");
    policy.network = false;
    policy.writable_roots = vec![path_text(&cwd0)];
    let mut reduced_b = reduced_a.clone();
    reduced_b.name = "Reduced B".to_owned();
    let repository = Arc::new(repository);
    let barrier = Arc::new(Barrier::new(2));
    let results = std::thread::scope(|scope| {
        [reduced_a, reduced_b]
            .into_iter()
            .map(|candidate| {
                let repository = repository.clone();
                let barrier = barrier.clone();
                scope.spawn(move || {
                    barrier.wait();
                    repository.publish_workspace(1, &candidate)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| thread.join().expect("publisher"))
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Err(ProfileError::StaleRevision { .. })))
            .count(),
        1
    );
    let second = repository.resolve("ws-main").expect("second snapshot");
    assert!(second.requires_respawn_from(&first));
    assert_ne!(second.digest().expect("new digest"), first_digest);
    assert_eq!(
        fs::read(assets.root().join(&first_asset.asset)).expect("old snapshot remains"),
        first.canonical_bytes().expect("old bytes")
    );

    let corrupt_root = tempfile::tempdir().expect("corrupt config root");
    let corrupt_repository = ConfigRepository::open(corrupt_root.path()).expect("repository");
    let corrupt_path = corrupt_root.path().join("config/providers.json");
    let corrupt_bytes = b"{\"format\":1,\"providers\":[],\"revision\":1,\"unknown\":true}\n";
    fs::write(&corrupt_path, corrupt_bytes).expect("corrupt authority");
    let replacement = ProvidersConfig {
        revision: 2,
        ..ProvidersConfig::default()
    };
    assert!(
        corrupt_repository
            .publish_providers(1, &replacement)
            .is_err(),
        "a writer must validate the current authority before replacing it"
    );
    assert_eq!(
        fs::read(corrupt_path).expect("authority remains"),
        corrupt_bytes,
        "invalid current authority is fail-stop, never silently overwritten"
    );
}

#[test]
fn slice2_gate_20_instruction_snapshot_race() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("instruction root");
    copy_tree(&fixtures.join("instruction-tree"), directory.path());
    let resolver = InstructionResolver::new(
        directory.path().join("user"),
        [
            directory.path().join("project0"),
            directory.path().join("project1"),
        ],
    );
    let scans = AtomicUsize::new(0);
    let changed = directory.path().join("project0/.agent/skills/shared.md");
    let snapshot = resolver
        .capture_with_hook(|scan| {
            scans.store(scan, Ordering::SeqCst);
            if scan == 1 {
                fs::write(&changed, b"Project zero changed skill.\n").expect("race edit");
            }
        })
        .expect("stable capture after mutation");
    assert_eq!(scans.load(Ordering::SeqCst), 3);
    let shared = snapshot.effective.skills["shared.md"] as usize;
    assert_eq!(
        snapshot.sources[shared].content,
        "Project zero changed skill.\n"
    );
    assert_eq!(snapshot.effective.policy.network, Some(false));
    assert_eq!(
        snapshot.effective.policy.allowed_tools,
        Some(vec!["read".to_owned()])
    );
    assert_eq!(snapshot.effective.policy.max_wall_seconds, Some(600));
    let assets = AssetStore::new(directory.path().join("assets")).expect("assets");
    let (digest, reference) = snapshot.publish(&assets).expect("snapshot publish");
    assert_eq!(reference.asset, format!("sha256-{digest}"));
}

#[test]
fn slice2_gate_21_instruction_next_launch_only() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let directory = tempfile::tempdir().expect("instruction root");
    copy_tree(&fixtures.join("instruction-tree"), directory.path());
    let resolver = InstructionResolver::new(
        directory.path().join("user"),
        [
            directory.path().join("project0"),
            directory.path().join("project1"),
        ],
    );
    let first = resolver.capture().expect("first launch snapshot");
    let first_bytes = first.canonical_bytes().expect("first bytes");
    let first_digest = first.digest().expect("first digest");
    fs::write(
        directory.path().join("project1/.agent/skills/final.md"),
        b"Edited for next launch.\n",
    )
    .expect("ordinary project edit");
    assert_eq!(
        first
            .canonical_bytes()
            .expect("immutable in-memory snapshot"),
        first_bytes
    );
    assert_eq!(first.digest().expect("old digest remains"), first_digest);
    let second = resolver.capture().expect("next launch snapshot");
    assert_ne!(second.digest().expect("second digest"), first_digest);
    assert_eq!(
        second.sources[second.effective.skills["final.md"] as usize].content,
        "Edited for next launch.\n"
    );
}

#[test]
fn slice2_gate_37_instruction_oracle_and_rejections() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    let resolver = InstructionResolver::new(
        fixtures.join("instruction-tree/user"),
        [
            fixtures.join("instruction-tree/project0"),
            fixtures.join("instruction-tree/project1"),
        ],
    );
    let snapshot = resolver.capture().expect("fixture snapshot");
    let oracle = read(&fixtures.join("instructions/snapshot.canonical.json")).expect("oracle");
    assert_eq!(snapshot.canonical_bytes().expect("snapshot bytes"), oracle);
    assert_eq!(
        InstructionSnapshot::decode(&oracle)
            .expect("oracle decode")
            .canonical_bytes()
            .expect("oracle encode"),
        oracle
    );
    assert!(
        InstructionSettings::decode(
            &read(&fixtures.join("instructions/settings-unknown.invalid.json")).expect("invalid")
        )
        .is_err()
    );
    assert!(
        InstructionSnapshot::decode(
            &read(&fixtures.join("instructions/snapshot-bad-index.invalid.json")).expect("invalid")
        )
        .is_err()
    );
    for case in [
        "instructions/snapshot-wrong-effective.invalid.json",
        "instructions/snapshot-wrong-order.invalid.json",
    ] {
        assert!(
            InstructionSnapshot::decode(&read(&fixtures.join(case)).expect("invalid")).is_err(),
            "{case} must reject"
        );
    }

    let directory = tempfile::tempdir().expect("symlink root");
    let user = directory.path().join("user");
    fs::create_dir_all(user.join("skills")).expect("skills");
    fs::write(directory.path().join("outside.md"), b"outside\n").expect("outside");
    std::os::unix::fs::symlink(
        directory.path().join("outside.md"),
        user.join("skills/link.md"),
    )
    .expect("symlink");
    assert!(matches!(
        InstructionResolver::new(&user, std::iter::empty::<&Path>()).capture(),
        Err(ProfileError::Symlink(_))
    ));
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("copy destination");
    for entry in fs::read_dir(source).expect("source directory") {
        let entry = entry.expect("source entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy fixture file");
        }
    }
}

fn path_text(path: &Path) -> String {
    path.to_str().expect("UTF-8 temp path").to_owned()
}
