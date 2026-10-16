use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};

use profile::{
    ConfigRepository, InstructionResolver, InstructionSettings, InstructionSnapshot, ProfileError,
    ProvidersConfig, SettingsConfig,
};
use store::AssetStore;
use test_support::{FixtureRoot, read};

#[test]
fn slice2_gate_35_config_contract_fixtures() {
    let fixtures = FixtureRoot::discover().unwrap();
    for name in [
        "providers.canonical.json",
        "providers-web-search.canonical.json",
    ] {
        let bytes = read(&fixtures.join("config").join(name)).unwrap();
        assert_eq!(
            ProvidersConfig::decode(&bytes)
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            bytes
        );
    }
    for name in [
        "providers-secret.invalid.json",
        "providers-web-search-private.invalid.json",
        "providers-web-search-mapped-loopback.invalid.json",
    ] {
        assert!(
            ProvidersConfig::decode(&read(&fixtures.join("config").join(name)).unwrap()).is_err()
        );
    }
    let settings = read(&fixtures.join("config/settings.canonical.json")).unwrap();
    assert_eq!(
        SettingsConfig::decode(&settings)
            .unwrap()
            .canonical_bytes()
            .unwrap(),
        settings
    );
    for value in [
        serde_json::json!({"cwd":"relative"}),
        serde_json::json!({"cwd":"/tmp","max_wall_seconds":0}),
        serde_json::json!({"cwd":"/tmp","trusted":true}),
        serde_json::json!({"cwd":"/tmp","environment":"local"}),
    ] {
        assert!(
            !serde_json::from_value::<schema::ExecutionConfig>(value)
                .map(|value| value.validate().is_ok())
                .unwrap_or(false)
        );
    }
}

#[test]
fn slice2_gate_36_config_atomic_publish_and_revocation() {
    let fixtures = FixtureRoot::discover().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let repository = Arc::new(ConfigRepository::open(directory.path()).unwrap());
    let mut providers =
        ProvidersConfig::decode(&read(&fixtures.join("config/providers.canonical.json")).unwrap())
            .unwrap();
    providers.revision = 1;
    repository.publish_providers(0, &providers).unwrap();
    let mut settings =
        SettingsConfig::decode(&read(&fixtures.join("config/settings.canonical.json")).unwrap())
            .unwrap();
    settings.revision = 1;
    repository.publish_settings(0, &settings).unwrap();
    let execution=repository.normalize_execution(&serde_json::from_value(serde_json::json!({"cwd":project.path(),"network":true,"writable_roots":[project.path()],"allowed_tools":["read"]})).unwrap()).unwrap();
    let first = repository
        .resolve_execution("session", &execution, None)
        .unwrap();
    let assets = AssetStore::new(directory.path().join("assets")).unwrap();
    let (first_digest, asset) = first.publish(&assets).unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let results = std::thread::scope(|scope| {
        [2, 3]
            .into_iter()
            .map(|workers| {
                let repository = repository.clone();
                let barrier = barrier.clone();
                let mut candidate = settings.clone();
                candidate.revision = 2;
                candidate.limits = Some(profile::Limits {
                    max_workers: Some(workers),
                    max_provider_leases: None,
                });
                scope.spawn(move || {
                    barrier.wait();
                    repository.publish_settings(1, &candidate)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|value| value.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|value| matches!(value, Err(ProfileError::StaleRevision { .. })))
            .count(),
        1
    );
    let second = repository
        .resolve_execution("session", &execution, None)
        .unwrap();
    assert_eq!(
        first.execution, second.execution,
        "host setting revisions cannot widen frozen execution"
    );
    let mut reduced = execution.clone();
    reduced.network = false;
    reduced.writable_roots.clear();
    assert!(
        repository
            .resolve_execution("session", &reduced, None)
            .unwrap()
            .requires_respawn_from(&first)
    );
    assert_eq!(
        fs::read(assets.root().join(asset.asset)).unwrap(),
        first.canonical_bytes().unwrap()
    );
    assert_ne!(first_digest, second.digest().unwrap());
    let corrupt = directory.path().join("config/providers.json");
    let bytes = b"{\"format\":1,\"providers\":[],\"revision\":1,\"unknown\":true}\n";
    fs::write(&corrupt, bytes).unwrap();
    providers.revision = 2;
    assert!(repository.publish_providers(1, &providers).is_err());
    assert_eq!(fs::read(corrupt).unwrap(), bytes);
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
    let changed = directory.path().join("user/skills/shared.md");
    let snapshot = resolver
        .capture_with_hook(|scan| {
            scans.store(scan, Ordering::SeqCst);
            if scan == 1 {
                fs::write(&changed, b"Host changed skill.\n").expect("race edit");
            }
        })
        .expect("stable capture after mutation");
    assert_eq!(scans.load(Ordering::SeqCst), 3);
    let shared = snapshot.effective.skills["shared.md"] as usize;
    assert_eq!(snapshot.sources[shared].content, "Host changed skill.\n");
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
        directory.path().join("user/skills/shared.md"),
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
        second.sources[second.effective.skills["shared.md"] as usize].content,
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
    // Historical canonical bytes remain decodeable. Runtime provenance is host-local.
    let oracle = read(&fixtures.join("instructions/snapshot.canonical.json")).expect("oracle");
    assert!(snapshot.sources.iter().all(|source| !matches!(
        (&source.origin, &source.kind),
        (
            profile::InstructionOrigin::Project { .. },
            profile::InstructionKind::Skill | profile::InstructionKind::Hook
        )
    )));
    assert_eq!(snapshot.effective.skills.len(), 1);
    assert!(snapshot.effective.hooks.is_empty());
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
