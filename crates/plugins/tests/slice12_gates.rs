use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use plugins::{
    ComponentKind, FaultPoint, HostEnvironment, InstallOptions, IntegrityStatus,
    MacOsNativeHelperVerifier, ManagementRequest, ManagementResult, NativeHelperIdentity,
    NativeHelperVerifier, OperatingSystem, PluginArchive, PluginComponentReference, PluginError,
    PluginSource, PluginStore, PluginVersion, SignaturePolicy,
};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde_json::json;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

const COMPUTER_USE_MANIFEST: &str =
    include_str!("../../../fixtures/plugins/tekes-computer-use-manifest.canonical.json");

#[derive(Clone, Copy)]
struct AcceptSignatures;

impl NativeHelperVerifier for AcceptSignatures {
    fn verify(
        &self,
        _: &Path,
        component_id: &str,
        relative_path: &str,
    ) -> Result<NativeHelperIdentity, PluginError> {
        Ok(NativeHelperIdentity {
            component_id: component_id.to_owned(),
            relative_path: relative_path.to_owned(),
            designated_requirement:
                "anchor apple generic and identifier com.tekes.computer-use.helper".to_owned(),
            signing_identifier: "com.tekes.computer-use.helper".to_owned(),
            team_identifier: Some("TEKESAPP01".to_owned()),
        })
    }
}

#[derive(Clone, Copy)]
struct RejectSignatures;

impl NativeHelperVerifier for RejectSignatures {
    fn verify(
        &self,
        _: &Path,
        component_id: &str,
        _: &str,
    ) -> Result<NativeHelperIdentity, PluginError> {
        Err(PluginError::Signature(format!(
            "rejected native helper {component_id}"
        )))
    }
}

#[test]
fn slice12_gate_81_hermetic_package_carrier_compatibility() {
    let source = TempDir::new().expect("source");
    write_computer_use_package(source.path());
    let archive_dir = TempDir::new().expect("archive");
    let archive = archive_dir
        .path()
        .join("TekesComputerUse-0.1.5.tekesplugin");
    write_archive(source.path(), &archive);
    let entries = PluginArchive::inspect(&archive).expect("inspect archive");
    assert!(entries.iter().any(|entry| entry.0 == "tekes-plugin.json"));

    let directory_root = TempDir::new().expect("directory store");
    let directory_receipt = open(directory_root.path(), AcceptSignatures)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        )
        .expect("install existing directory format");
    assert_eq!(directory_receipt.plugin_id, "com.tekes.computer-use");

    let root = TempDir::new().expect("store");
    let mut store = open(root.path(), AcceptSignatures);
    let receipt = store
        .install(
            &PluginSource::from_path(&archive).expect("archive source"),
            InstallOptions {
                grants: computer_use_grants(),
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("install existing package format");
    assert_eq!(receipt.plugin_id, "com.tekes.computer-use");
    assert_eq!(receipt.version.to_string(), "0.1.5");
    let reduced_grants = BTreeSet::from([
        "accessibility.control".to_owned(),
        "native-helper.execute".to_owned(),
    ]);
    let disabled = store
        .set_grants("com.tekes.computer-use", reduced_grants)
        .expect("removing a required grant disables the plugin");
    assert!(!disabled.enabled);
    assert!(store.components().expect("disabled projection").is_empty());
    store
        .set_grants("com.tekes.computer-use", computer_use_grants())
        .expect("restore grants");
    store
        .set_enabled("com.tekes.computer-use", true)
        .expect("explicitly re-enable");
    let components = store.components().expect("project components");
    assert_eq!(components.len(), 3);
    assert!(components.iter().all(|component| !component.launchable));
    assert_eq!(
        components
            .iter()
            .find(|component| component.component_id == "computer-use")
            .map(|component| component.component_type),
        Some(ComponentKind::NativeHelper)
    );
}

#[test]
fn slice12_gate_82_hermetic_signatures_trust_and_grants_fail_closed() {
    let source = TempDir::new().expect("source");
    write_computer_use_package(source.path());
    fs::create_dir(source.path().join(".codex-plugin")).expect("attestation parent");

    let rejected_root = TempDir::new().expect("rejected store");
    let rejected = open(rejected_root.path(), RejectSignatures).install(
        &PluginSource::Directory(source.path().to_owned()),
        InstallOptions::default(),
    );
    assert!(matches!(rejected, Err(PluginError::Signature(_))));

    let digest_root = TempDir::new().expect("digest store");
    let digest = open(digest_root.path(), AcceptSignatures)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        )
        .expect("derive compatible package digest")
        .package_digest;
    let random = SystemRandom::new();
    let encoded = Ed25519KeyPair::generate_pkcs8(&random).expect("generate publisher key");
    let pair = Ed25519KeyPair::from_pkcs8(encoded.as_ref()).expect("decode publisher key");
    let attestation = json!({
        "publisherID": "com.tekes.first-party",
        "publicKey": STANDARD.encode(pair.public_key().as_ref()),
        "signature": STANDARD.encode(pair.sign(digest.as_bytes()).as_ref()),
        "contentDigest": digest,
    });
    fs::write(
        source.path().join(".codex-plugin/publisher.sig.json"),
        serde_json::to_vec(&attestation).expect("attestation bytes"),
    )
    .expect("write attestation");

    let trusted_root = TempDir::new().expect("trusted store");
    let publisher_fingerprint = Sha256::digest(pair.public_key().as_ref()).iter().fold(
        String::with_capacity(64),
        |mut output, byte| {
            write!(output, "{byte:02x}").expect("write fingerprint");
            output
        },
    );
    let untrusted_root = TempDir::new().expect("untrusted store");
    let untrusted = PluginStore::open(
        untrusted_root.path(),
        host(),
        SignaturePolicy {
            allow_unsigned_local: false,
            trusted_publishers: BTreeMap::from([(
                "com.tekes.first-party".to_owned(),
                "0".repeat(64),
            )]),
        },
        AcceptSignatures,
    )
    .expect("untrusted store")
    .install(
        &PluginSource::Directory(source.path().to_owned()),
        InstallOptions::default(),
    );
    assert!(matches!(
        untrusted,
        Err(PluginError::PublisherTrustRequired(_))
    ));
    let trusted_policy = SignaturePolicy {
        allow_unsigned_local: false,
        trusted_publishers: BTreeMap::from([(
            "com.tekes.first-party".to_owned(),
            publisher_fingerprint,
        )]),
    };
    let trusted = PluginStore::open(
        trusted_root.path(),
        host(),
        trusted_policy.clone(),
        AcceptSignatures,
    )
    .expect("trusted store")
    .install(
        &PluginSource::Directory(source.path().to_owned()),
        InstallOptions::default(),
    )
    .expect("verified publisher install");
    assert_eq!(trusted.integrity, IntegrityStatus::PublisherVerified);

    let grant_root = TempDir::new().expect("grant store");
    let enable_without_grants = PluginStore::open(
        grant_root.path(),
        host(),
        trusted_policy.clone(),
        AcceptSignatures,
    )
    .expect("grant store")
    .install(
        &PluginSource::Directory(source.path().to_owned()),
        InstallOptions {
            enable: true,
            ..InstallOptions::default()
        },
    );
    assert!(matches!(
        enable_without_grants,
        Err(PluginError::MissingGrant(_))
    ));

    let undeclared_root = TempDir::new().expect("undeclared grant store");
    let undeclared = PluginStore::open(
        undeclared_root.path(),
        host(),
        trusted_policy.clone(),
        AcceptSignatures,
    )
    .expect("undeclared grant store")
    .install(
        &PluginSource::Directory(source.path().to_owned()),
        InstallOptions {
            grants: BTreeSet::from(["filesystem.everywhere".to_owned()]),
            ..InstallOptions::default()
        },
    );
    assert_eq!(
        undeclared,
        Err(PluginError::UndeclaredGrant(
            "filesystem.everywhere".to_owned()
        ))
    );

    assert_signed_registry_tamper_rejected(source.path(), &trusted_policy, |receipt| {
        receipt["displayName"] = json!("Forged Display Name");
    });
    assert_signed_registry_tamper_rejected(source.path(), &trusted_policy, |receipt| {
        receipt["requestedCapabilities"] = json!([{"id": "filesystem.everywhere"}]);
        receipt["grantedCapabilities"] = json!(["filesystem.everywhere"]);
        receipt["enabled"] = json!(true);
    });
}

#[test]
fn slice12_gate_83_install_update_remove_enable_and_collision_ownership() {
    let root = TempDir::new().expect("store");
    let first = TempDir::new().expect("first");
    write_package(first.path(), "com.example.first", "1.0.0", "shared");
    let mut store = open(root.path(), AcceptSignatures);
    store
        .install(
            &PluginSource::Directory(first.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("install first");

    let update = TempDir::new().expect("update");
    write_package(update.path(), "com.example.first", "2.0.0", "shared");
    let updated = store
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("update");
    assert_eq!(updated.version.to_string(), "2.0.0");

    let downgrade = store.install(
        &PluginSource::Directory(first.path().to_owned()),
        InstallOptions::default(),
    );
    assert!(matches!(
        downgrade,
        Err(PluginError::DowngradeRejected { .. })
    ));
    let downgraded = store
        .install(
            &PluginSource::Directory(first.path().to_owned()),
            InstallOptions {
                enable: true,
                allow_downgrade: true,
                ..InstallOptions::default()
            },
        )
        .expect("authorized downgrade");
    assert_eq!(downgraded.version.to_string(), "1.0.0");
    store
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("restore update");
    fs::write(update.path().join("changed-content"), "changed\n")
        .expect("same-version content change");
    assert!(matches!(
        store.install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions::default(),
        ),
        Err(PluginError::SameVersionChanged(_))
    ));
    store
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions {
                enable: true,
                allow_same_version_replacement: true,
                ..InstallOptions::default()
            },
        )
        .expect("authorized same-version replacement");

    let build_first = TempDir::new().expect("first build metadata source");
    write_package(
        build_first.path(),
        "com.example.build-metadata",
        "3.0.0+darwin-arm64",
        "component",
    );
    store
        .install(
            &PluginSource::Directory(build_first.path().to_owned()),
            InstallOptions::default(),
        )
        .expect("install first build identity");
    let build_second = TempDir::new().expect("second build metadata source");
    write_package(
        build_second.path(),
        "com.example.build-metadata",
        "3.0.0+darwin-x86-64",
        "component",
    );
    assert!(matches!(
        store.install(
            &PluginSource::Directory(build_second.path().to_owned()),
            InstallOptions::default(),
        ),
        Err(PluginError::SameVersionChanged(_))
    ));
    store
        .install(
            &PluginSource::Directory(build_second.path().to_owned()),
            InstallOptions {
                allow_same_version_replacement: true,
                ..InstallOptions::default()
            },
        )
        .expect("authorize equal-precedence build replacement");

    let second = TempDir::new().expect("second");
    write_package(second.path(), "com.example.second", "1.0.0", "shared");
    store
        .install(
            &PluginSource::Directory(second.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("install second");
    assert!(matches!(
        store.components(),
        Err(PluginError::ComponentCollision { .. })
    ));

    assert!(store.remove("com.example.second").expect("remove second"));
    assert_eq!(store.components().expect("components").len(), 1);
    assert!(
        !store
            .remove("com.example.second")
            .expect("idempotent remove")
    );
}

#[test]
fn slice12_gate_84_transaction_recovery_is_commit_or_rollback() {
    let source = TempDir::new().expect("source");
    write_package(source.path(), "com.example.recovery", "1.0.0", "component");

    let rollback_root = TempDir::new().expect("rollback store");
    let unjournaled = open(rollback_root.path(), AcceptSignatures)
        .injecting_fault(FaultPoint::PackageDurableBeforeJournal)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        unjournaled,
        Err(PluginError::InjectedCrash(
            FaultPoint::PackageDurableBeforeJournal
        ))
    );
    assert_unjournaled_disk_state(&rollback_root, None, 1, true);
    assert!(
        open(rollback_root.path(), AcceptSignatures)
            .list()
            .expect("recover unjournaled publication")
            .is_empty()
    );
    assert_recovered_disk_state(&rollback_root, None, 0, false);

    let rollback = open(rollback_root.path(), AcceptSignatures)
        .injecting_fault(FaultPoint::PackagePublished)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        rollback,
        Err(PluginError::InjectedCrash(FaultPoint::PackagePublished))
    );
    assert_fault_disk_state(&rollback_root, "package-published", None, 1, true);
    assert!(
        open(rollback_root.path(), AcceptSignatures)
            .list()
            .expect("recover rollback")
            .is_empty()
    );
    assert_recovered_disk_state(&rollback_root, None, 0, false);

    let commit_root = TempDir::new().expect("commit store");
    let committed = open(commit_root.path(), AcceptSignatures)
        .injecting_fault(FaultPoint::RegistryPublished)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        committed,
        Err(PluginError::InjectedCrash(FaultPoint::RegistryPublished))
    );
    assert_fault_disk_state(&commit_root, "package-published", Some("1.0.0"), 1, true);
    let mut recovered = open(commit_root.path(), AcceptSignatures);
    assert_eq!(recovered.list().expect("recover commit").len(), 1);
    assert_recovered_disk_state(&commit_root, Some("1.0.0"), 1, true);

    let update = TempDir::new().expect("update source");
    write_package(update.path(), "com.example.recovery", "2.0.0", "component");
    let update_unjournaled = recovered
        .injecting_fault(FaultPoint::PackageDurableBeforeJournal)
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        update_unjournaled,
        Err(PluginError::InjectedCrash(
            FaultPoint::PackageDurableBeforeJournal
        ))
    );
    assert_unjournaled_disk_state(&commit_root, Some("1.0.0"), 2, true);
    let recovered = open(commit_root.path(), AcceptSignatures);
    assert_recovered_disk_state(&commit_root, Some("1.0.0"), 1, true);

    let update_rollback = recovered
        .injecting_fault(FaultPoint::PackagePublished)
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        update_rollback,
        Err(PluginError::InjectedCrash(FaultPoint::PackagePublished))
    );
    assert_fault_disk_state(&commit_root, "package-published", Some("1.0.0"), 2, true);
    let mut recovered = open(commit_root.path(), AcceptSignatures);
    assert_eq!(
        recovered.list().expect("recover update rollback")[0]
            .version
            .to_string(),
        "1.0.0"
    );
    assert_recovered_disk_state(&commit_root, Some("1.0.0"), 1, true);

    let update_commit = recovered
        .injecting_fault(FaultPoint::RegistryPublished)
        .install(
            &PluginSource::Directory(update.path().to_owned()),
            InstallOptions::default(),
        );
    assert_eq!(
        update_commit,
        Err(PluginError::InjectedCrash(FaultPoint::RegistryPublished))
    );
    assert_fault_disk_state(&commit_root, "package-published", Some("2.0.0"), 2, true);
    let mut recovered = open(commit_root.path(), AcceptSignatures);
    assert_eq!(
        recovered.list().expect("recover update commit")[0]
            .version
            .to_string(),
        "2.0.0"
    );
    assert_recovered_disk_state(&commit_root, Some("2.0.0"), 1, true);

    let remove_rollback = recovered
        .injecting_fault(FaultPoint::PackagePublished)
        .remove("com.example.recovery");
    assert_eq!(
        remove_rollback,
        Err(PluginError::InjectedCrash(FaultPoint::PackagePublished))
    );
    assert_fault_disk_state(&commit_root, "package-published", Some("2.0.0"), 1, true);
    let mut recovered = open(commit_root.path(), AcceptSignatures);
    assert_eq!(recovered.list().expect("recover remove rollback").len(), 1);
    assert_recovered_disk_state(&commit_root, Some("2.0.0"), 1, true);

    let removed = recovered
        .injecting_fault(FaultPoint::RegistryPublished)
        .remove("com.example.recovery");
    assert_eq!(
        removed,
        Err(PluginError::InjectedCrash(FaultPoint::RegistryPublished))
    );
    assert_fault_disk_state(&commit_root, "package-published", None, 1, true);
    assert!(
        open(commit_root.path(), AcceptSignatures)
            .list()
            .expect("recover removal")
            .is_empty()
    );
    assert_recovered_disk_state(&commit_root, None, 0, false);

    exercise_enable_and_grants_fault_windows();
    exercise_recovery_cleanup_boundaries();

    let corrupt_root = TempDir::new().expect("corrupt store");
    let mut corrupt_store = open(corrupt_root.path(), AcceptSignatures);
    corrupt_store
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        )
        .expect("seed corrupt-operation store");
    let registry_path = corrupt_root.path().join("registry.canonical.json");
    let registry: serde_json::Value =
        serde_json::from_slice(&fs::read(&registry_path).expect("registry bytes"))
            .expect("registry JSON");
    let mut previous = registry["plugins"][0].clone();
    previous["packageRelativePath"] = json!("victim");
    write_canonical_value(&registry_path, &json!({"format": 1, "plugins": []}));
    let victim = corrupt_root.path().join("victim");
    fs::create_dir(&victim).expect("victim directory");
    fs::write(victim.join("sentinel"), "preserve\n").expect("victim sentinel");
    write_canonical_value(
        &corrupt_root
            .path()
            .join("operations/current.canonical.json"),
        &json!({
            "format": 1,
            "id": "corrupt-operation",
            "kind": "remove",
            "phase": "registry-published",
            "pluginId": "com.example.recovery",
            "previous": previous,
            "next": null,
        }),
    );
    assert!(matches!(
        PluginStore::open(
            corrupt_root.path(),
            host(),
            SignaturePolicy::default(),
            AcceptSignatures,
        ),
        Err(PluginError::CorruptRegistry(_))
    ));
    assert!(victim.join("sentinel").is_file());
}

#[test]
fn slice12_gate_85_typed_management_is_inert_and_deterministic() {
    let source = TempDir::new().expect("source");
    write_package(
        source.path(),
        "com.example.management",
        "1.0.0",
        "component",
    );
    let root = TempDir::new().expect("store");
    let mut store = open(root.path(), AcceptSignatures);
    let installed = store
        .manage(ManagementRequest::Install {
            source: PluginSource::Directory(source.path().to_owned()),
            options: InstallOptions::default(),
        })
        .expect("typed install");
    assert!(matches!(installed, ManagementResult::Receipt(_)));
    let enabled = store
        .manage(ManagementRequest::SetEnabled {
            plugin_id: "com.example.management".to_owned(),
            enabled: true,
        })
        .expect("typed enable");
    assert!(matches!(enabled, ManagementResult::Receipt(_)));
    let listed = store.manage(ManagementRequest::List).expect("typed list");
    assert!(matches!(listed, ManagementResult::Receipts(receipts) if receipts.len() == 1));
    let grants = store
        .manage(ManagementRequest::SetGrants {
            plugin_id: "com.example.management".to_owned(),
            grants: BTreeSet::new(),
        })
        .expect("typed grants");
    assert!(matches!(grants, ManagementResult::Receipt(_)));
    assert_eq!(
        store
            .manage(ManagementRequest::Recover)
            .expect("typed recover"),
        ManagementResult::Recovered
    );
    let projected = store
        .manage(ManagementRequest::Components)
        .expect("typed projection");
    let ManagementResult::Components(components) = projected else {
        panic!("wrong typed result");
    };
    assert_eq!(components.len(), 1);
    assert!(!components[0].launchable);

    let executable_source = TempDir::new().expect("generic executable source");
    write_executable_package(executable_source.path(), "1.0.0", "first\n");
    store
        .install(
            &PluginSource::Directory(executable_source.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("install generic executable plugin");
    let reference = PluginComponentReference {
        plugin_id: "com.example.executable".to_owned(),
        component_id: "ordinary-peer".to_owned(),
    };
    let first_resolution = store
        .resolve_executable_component(&reference)
        .expect("resolve first executable generation")
        .expect("enabled executable relation");
    assert_eq!(first_resolution.reference, reference);
    assert_eq!(first_resolution.component_type, ComponentKind::McpServer);
    assert!(first_resolution.executable_path.is_file());
    assert!(first_resolution.data_path.is_dir());
    assert_ne!(first_resolution.plugin_generation, "");
    assert_eq!(
        serde_json::to_value(&reference).expect("component reference JSON"),
        json!({"pluginId":"com.example.executable","componentId":"ordinary-peer"})
    );

    fs::remove_dir(&first_resolution.data_path).expect("remove empty plugin data directory");
    let recreated_resolution = store
        .resolve_executable_component(&reference)
        .expect("recreate missing plugin data directory")
        .expect("enabled executable relation after data recreation");
    assert_eq!(recreated_resolution.data_path, first_resolution.data_path);
    assert!(recreated_resolution.data_path.is_dir());

    let executable_update = TempDir::new().expect("generic executable update");
    write_executable_package(executable_update.path(), "2.0.0", "second\n");
    store
        .install(
            &PluginSource::Directory(executable_update.path().to_owned()),
            InstallOptions {
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("update generic executable plugin");
    let second_resolution = store
        .resolve_executable_component(&reference)
        .expect("resolve second executable generation")
        .expect("updated executable relation");
    assert_ne!(
        first_resolution.plugin_generation,
        second_resolution.plugin_generation
    );
    assert_ne!(
        first_resolution.executable_path,
        second_resolution.executable_path
    );
    assert!(!first_resolution.executable_path.exists());
    store
        .set_enabled("com.example.executable", false)
        .expect("disable generic executable plugin");
    assert_eq!(
        store
            .resolve_executable_component(&reference)
            .expect("disabled executable relation"),
        None
    );
    let registry = fs::read(root.path().join("registry.canonical.json")).expect("registry");
    assert_eq!(registry.last(), Some(&b'\n'));
    let value: serde_json::Value = serde_json::from_slice(&registry).expect("registry JSON");
    let mut canonical = serde_json_canonicalizer::to_vec(&value).expect("canonical registry");
    canonical.push(b'\n');
    assert_eq!(registry, canonical);
    assert_eq!(
        store
            .manage(ManagementRequest::Remove {
                plugin_id: "com.example.management".to_owned(),
            })
            .expect("typed remove"),
        ManagementResult::Removed(true)
    );

    fs::remove_dir(&second_resolution.data_path).expect("remove empty plugin data directory");
    let outside = TempDir::new().expect("outside plugin data");
    std::os::unix::fs::symlink(outside.path(), &second_resolution.data_path)
        .expect("replace plugin data with symlink");
    assert!(matches!(
        store.resolve_executable_component(&reference),
        Err(PluginError::Storage(message))
            if message.contains("managed path is not a directory")
                || message.contains("plugin data path is not a stable directory")
    ));
}

#[test]
#[ignore = "requires an explicit signed Computer Use archive and macOS codesign"]
fn slice12_reference_qualification_external_archive_without_launch() {
    let archive = PathBuf::from(
        std::env::var("TEKES_COMPUTER_USE_ARCHIVE").expect("set TEKES_COMPUTER_USE_ARCHIVE"),
    );
    let root = TempDir::new().expect("store");
    let mut store = PluginStore::open(
        root.path(),
        host(),
        local_signature_policy(),
        MacOsNativeHelperVerifier,
    )
    .expect("store");
    let receipt = store
        .install(
            &PluginSource::from_path(archive).expect("real archive"),
            InstallOptions {
                grants: computer_use_grants(),
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("recognize and validate existing package");
    assert_eq!(receipt.plugin_id, "com.tekes.computer-use");
    let resolved = store
        .resolve_executable_component(&PluginComponentReference {
            plugin_id: receipt.plugin_id.clone(),
            component_id: "computer-use".to_owned(),
        })
        .expect("resolve generic reference relation")
        .expect("enabled executable relation");
    assert_eq!(resolved.plugin_generation, receipt.package_digest);
    assert!(resolved.executable_path.is_file());
}

fn open<V: NativeHelperVerifier>(root: &Path, verifier: V) -> PluginStore<V> {
    PluginStore::open(root, host(), local_signature_policy(), verifier).expect("plugin store")
}

fn local_signature_policy() -> SignaturePolicy {
    SignaturePolicy {
        allow_unsigned_local: true,
        trusted_publishers: BTreeMap::new(),
    }
}

fn host() -> HostEnvironment {
    HostEnvironment {
        host_version: "1.0.0".parse::<PluginVersion>().expect("host version"),
        operating_system: OperatingSystem::MacOs,
        operating_system_version: "15.0".to_owned(),
        architecture: "arm64".to_owned(),
    }
}

fn computer_use_grants() -> BTreeSet<String> {
    BTreeSet::from([
        "accessibility.control".to_owned(),
        "native-helper.execute".to_owned(),
        "screen.capture".to_owned(),
    ])
}

fn write_computer_use_package(root: &Path) {
    fs::create_dir_all(root.join("bin/TekesComputerUseHelper.app/Contents/MacOS"))
        .expect("helper directory");
    fs::create_dir_all(root.join("skills/computer-use")).expect("skill directory");
    fs::create_dir_all(root.join("assets")).expect("assets directory");
    fs::write(root.join("tekes-plugin.json"), COMPUTER_USE_MANIFEST).expect("manifest");
    fs::write(
        root.join("bin/TekesComputerUseHelper.app/Contents/MacOS/TekesComputerUseHelper"),
        b"signed-helper-fixture",
    )
    .expect("helper");
    let helper = root.join("bin/TekesComputerUseHelper.app/Contents/MacOS/TekesComputerUseHelper");
    fs::set_permissions(&helper, fs::Permissions::from_mode(0o755)).expect("helper mode");
    fs::write(
        root.join("skills/computer-use/SKILL.md"),
        "# Computer Use\n",
    )
    .expect("skill manifest");
    fs::write(root.join("assets/icon.txt"), "asset\n").expect("asset");
}

fn write_package(root: &Path, id: &str, version: &str, component: &str) {
    fs::create_dir_all(root.join("skills/basic")).expect("skill directory");
    fs::write(root.join("skills/basic/SKILL.md"), "# Basic\n").expect("skill");
    let manifest = json!({
        "manifestVersion": 1,
        "id": id,
        "version": version,
        "displayName": id,
        "platforms": [{"os": "macos", "minimumVersion": "15.0", "architectures": ["arm64"]}],
        "capabilities": [],
        "components": [{"id": component, "type": "skill", "path": "skills/basic", "capabilities": []}],
    });
    fs::write(
        root.join("tekes-plugin.json"),
        serde_json::to_vec(&manifest).expect("manifest bytes"),
    )
    .expect("manifest");
}

fn write_archive(source: &Path, archive: &Path) {
    let mut entries = Vec::new();
    collect_archive_entries(source, source, &mut entries);
    entries.sort_by(|left, right| {
        left.get("path")
            .and_then(serde_json::Value::as_str)
            .cmp(&right.get("path").and_then(serde_json::Value::as_str))
    });
    fs::write(
        archive,
        serde_json::to_vec(&json!({"archiveVersion": 1, "entries": entries}))
            .expect("archive bytes"),
    )
    .expect("archive");
}

fn collect_archive_entries(root: &Path, current: &Path, output: &mut Vec<serde_json::Value>) {
    for entry in fs::read_dir(current).expect("read package") {
        let entry = entry.expect("entry");
        let path = entry.path();
        let relative = path.strip_prefix(root).expect("relative").to_string_lossy();
        let metadata = entry.metadata().expect("metadata");
        if metadata.is_dir() {
            output.push(json!({"path": relative, "kind": "directory"}));
            collect_archive_entries(root, &path, output);
        } else {
            output.push(json!({
                "path": relative,
                "kind": "file",
                "executable": metadata.permissions().mode() & 0o111 != 0,
                "data": STANDARD.encode(fs::read(path).expect("file bytes")),
            }));
        }
    }
}

fn assert_fault_disk_state(
    root: &TempDir,
    phase: &str,
    registry_version: Option<&str>,
    package_count: usize,
    data_exists: bool,
) {
    let operation: serde_json::Value = serde_json::from_slice(
        &fs::read(root.path().join("operations/current.canonical.json"))
            .expect("fault operation bytes"),
    )
    .expect("fault operation JSON");
    assert_eq!(operation["phase"], phase);
    assert_eq!(
        registry_version_at(root.path()).as_deref(),
        registry_version
    );
    assert_eq!(package_leaf_count(root.path()), package_count);
    assert_eq!(
        root.path().join("data/com.example.recovery").is_dir(),
        data_exists
    );
}

fn assert_unjournaled_disk_state(
    root: &TempDir,
    registry_version: Option<&str>,
    package_count: usize,
    data_exists: bool,
) {
    assert!(
        !root
            .path()
            .join("operations/current.canonical.json")
            .exists()
    );
    assert_eq!(
        registry_version_at(root.path()).as_deref(),
        registry_version
    );
    assert_eq!(package_leaf_count(root.path()), package_count);
    assert_eq!(
        root.path().join("data/com.example.recovery").is_dir(),
        data_exists
    );
    assert!(directory_is_empty(&root.path().join(".staging")));
}

fn assert_recovered_disk_state(
    root: &TempDir,
    registry_version: Option<&str>,
    package_count: usize,
    data_exists: bool,
) {
    assert_eq!(
        registry_version_at(root.path()).as_deref(),
        registry_version
    );
    assert_eq!(package_leaf_count(root.path()), package_count);
    assert_eq!(
        root.path().join("data/com.example.recovery").is_dir(),
        data_exists
    );
    assert!(
        !root
            .path()
            .join("operations/current.canonical.json")
            .exists()
    );
    assert!(directory_is_empty(&root.path().join(".staging")));
    assert!(!has_publication_temporary(root.path()));
    assert!(!has_publication_temporary(&root.path().join("operations")));
}

fn registry_version_at(root: &Path) -> Option<String> {
    let path = root.join("registry.canonical.json");
    if !path.is_file() {
        return None;
    }
    let registry: serde_json::Value =
        serde_json::from_slice(&fs::read(path).expect("registry bytes")).expect("registry JSON");
    registry["plugins"]
        .as_array()
        .and_then(|plugins| plugins.first())
        .and_then(|receipt| receipt["version"].as_str())
        .map(str::to_owned)
}

fn package_leaf_count(root: &Path) -> usize {
    let packages = root.join("packages");
    if !packages.is_dir() {
        return 0;
    }
    fs::read_dir(packages)
        .expect("packages")
        .flat_map(|plugin| fs::read_dir(plugin.expect("plugin").path()).expect("versions"))
        .flat_map(|version| fs::read_dir(version.expect("version").path()).expect("digests"))
        .count()
}

fn directory_is_empty(path: &Path) -> bool {
    fs::read_dir(path)
        .expect("managed directory")
        .next()
        .is_none()
}

fn has_publication_temporary(path: &Path) -> bool {
    fs::read_dir(path)
        .expect("publication directory")
        .any(|entry| {
            entry
                .expect("publication entry")
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with('.') && name.ends_with(".tmp"))
        })
}

fn exercise_enable_and_grants_fault_windows() {
    let source = TempDir::new().expect("capability source");
    write_capability_package(source.path());
    let root = TempDir::new().expect("capability store");
    let grants = BTreeSet::from(["network.access".to_owned()]);
    open(root.path(), AcceptSignatures)
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions {
                grants: grants.clone(),
                ..InstallOptions::default()
            },
        )
        .expect("seed capability package");

    let enable_rollback = open(root.path(), AcceptSignatures)
        .injecting_fault(FaultPoint::PackagePublished)
        .set_enabled("com.example.capability", true);
    assert_eq!(
        enable_rollback,
        Err(PluginError::InjectedCrash(FaultPoint::PackagePublished))
    );
    assert_operation_and_receipt_state(root.path(), "package-published", false, &grants);
    let mut recovered = open(root.path(), AcceptSignatures);
    assert!(!recovered.list().expect("enable rollback")[0].enabled);

    let enable_commit = recovered
        .injecting_fault(FaultPoint::RegistryPublished)
        .set_enabled("com.example.capability", true);
    assert_eq!(
        enable_commit,
        Err(PluginError::InjectedCrash(FaultPoint::RegistryPublished))
    );
    assert_operation_and_receipt_state(root.path(), "package-published", true, &grants);
    let mut recovered = open(root.path(), AcceptSignatures);
    assert!(recovered.list().expect("enable commit")[0].enabled);

    let grants_rollback = recovered
        .injecting_fault(FaultPoint::PackagePublished)
        .set_grants("com.example.capability", BTreeSet::new());
    assert_eq!(
        grants_rollback,
        Err(PluginError::InjectedCrash(FaultPoint::PackagePublished))
    );
    assert_operation_and_receipt_state(root.path(), "package-published", true, &grants);
    let mut recovered = open(root.path(), AcceptSignatures);
    let receipt = &recovered.list().expect("grants rollback")[0];
    assert!(receipt.enabled);
    assert_eq!(receipt.granted_capabilities, vec!["network.access"]);

    let grants_commit = recovered
        .injecting_fault(FaultPoint::RegistryPublished)
        .set_grants("com.example.capability", BTreeSet::new());
    assert_eq!(
        grants_commit,
        Err(PluginError::InjectedCrash(FaultPoint::RegistryPublished))
    );
    assert_operation_and_receipt_state(root.path(), "package-published", false, &BTreeSet::new());
    let mut recovered = open(root.path(), AcceptSignatures);
    let receipt = &recovered.list().expect("grants commit")[0];
    assert!(!receipt.enabled);
    assert!(receipt.granted_capabilities.is_empty());
    assert!(
        !root
            .path()
            .join("operations/current.canonical.json")
            .exists()
    );
    assert_eq!(package_leaf_count(root.path()), 1);
    assert!(root.path().join("data/com.example.capability").is_dir());
}

fn assert_operation_and_receipt_state(
    root: &Path,
    phase: &str,
    enabled: bool,
    grants: &BTreeSet<String>,
) {
    let operation: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("operations/current.canonical.json")).expect("operation bytes"),
    )
    .expect("operation JSON");
    assert_eq!(operation["phase"], phase);
    let registry: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("registry.canonical.json")).expect("registry bytes"),
    )
    .expect("registry JSON");
    let receipt = &registry["plugins"][0];
    assert_eq!(receipt["enabled"], enabled);
    assert_eq!(
        receipt["grantedCapabilities"],
        json!(grants.iter().collect::<Vec<_>>())
    );
}

fn exercise_recovery_cleanup_boundaries() {
    let source = TempDir::new().expect("cleanup source");
    write_package(source.path(), "com.example.cleanup", "1.0.0", "component");
    let root = TempDir::new().expect("cleanup store");
    let mut store = open(root.path(), AcceptSignatures);
    store
        .install(
            &PluginSource::Directory(source.path().to_owned()),
            InstallOptions::default(),
        )
        .expect("cleanup seed");
    fs::write(
        root.path().join(".registry.canonical.json.crash.tmp"),
        "partial",
    )
    .expect("registry temporary");
    fs::write(
        root.path()
            .join("operations/.current.canonical.json.crash.tmp"),
        "partial",
    )
    .expect("operation temporary");
    fs::create_dir(root.path().join(".staging/abandoned")).expect("abandoned stage");
    fs::write(root.path().join(".staging/abandoned/file"), "partial")
        .expect("abandoned stage file");
    store.recover().expect("cleanup regular debris");
    assert!(directory_is_empty(&root.path().join(".staging")));
    assert!(!has_publication_temporary(root.path()));
    assert!(!has_publication_temporary(&root.path().join("operations")));

    let outside = TempDir::new().expect("cleanup outside");
    fs::write(outside.path().join("sentinel"), "preserve\n").expect("outside sentinel");
    std::os::unix::fs::symlink(outside.path(), root.path().join(".staging/symlink-race"))
        .expect("staging symlink");
    PluginStore::open(
        root.path(),
        host(),
        local_signature_policy(),
        AcceptSignatures,
    )
    .expect("remove the controlled staging symlink without following it");
    assert!(directory_is_empty(&root.path().join(".staging")));
    assert!(!root.path().join(".staging/symlink-race").exists());
    assert_eq!(
        fs::read_to_string(outside.path().join("sentinel")).expect("outside preserved"),
        "preserve\n"
    );
}

fn write_capability_package(root: &Path) {
    fs::create_dir_all(root.join("skills/basic")).expect("skill directory");
    fs::write(root.join("skills/basic/SKILL.md"), "# Basic\n").expect("skill");
    fs::write(
        root.join("tekes-plugin.json"),
        serde_json::to_vec(&json!({
            "manifestVersion": 1,
            "id": "com.example.capability",
            "version": "1.0.0",
            "displayName": "Capability",
            "platforms": [{"os": "macos", "minimumVersion": "15.0", "architectures": ["arm64"]}],
            "capabilities": [{"id": "network.access"}],
            "components": [{
                "id": "component",
                "type": "skill",
                "path": "skills/basic",
                "capabilities": ["network.access"]
            }],
        }))
        .expect("capability manifest"),
    )
    .expect("capability manifest bytes");
}

fn write_executable_package(root: &Path, version: &str, payload: &str) {
    fs::create_dir_all(root.join("bin")).expect("executable directory");
    let executable = root.join("bin/server");
    fs::write(&executable, payload).expect("executable bytes");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).expect("executable mode");
    fs::write(
        root.join("tekes-plugin.json"),
        serde_json::to_vec(&json!({
            "manifestVersion": 1,
            "id": "com.example.executable",
            "version": version,
            "displayName": "Generic Executable",
            "platforms": [{"os": "macos", "minimumVersion": "15.0", "architectures": ["arm64"]}],
            "capabilities": [],
            "components": [{
                "id": "ordinary-peer",
                "type": "mcp-server",
                "path": "bin/server",
                "capabilities": []
            }],
        }))
        .expect("executable manifest"),
    )
    .expect("executable manifest bytes");
}

fn write_canonical_value(path: &Path, value: &serde_json::Value) {
    let mut bytes = serde_json_canonicalizer::to_vec(value).expect("canonical JSON");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("canonical fixture state");
}

fn assert_signed_registry_tamper_rejected(
    source: &Path,
    policy: &SignaturePolicy,
    mutate: impl FnOnce(&mut serde_json::Value),
) {
    let root = TempDir::new().expect("tamper store");
    PluginStore::open(root.path(), host(), policy.clone(), AcceptSignatures)
        .expect("signed tamper store")
        .install(
            &PluginSource::Directory(source.to_owned()),
            InstallOptions::default(),
        )
        .expect("seed signed tamper store");
    let registry_path = root.path().join("registry.canonical.json");
    let mut registry: serde_json::Value =
        serde_json::from_slice(&fs::read(&registry_path).expect("registry bytes"))
            .expect("registry JSON");
    mutate(&mut registry["plugins"][0]);
    write_canonical_value(&registry_path, &registry);
    assert!(matches!(
        PluginStore::open(root.path(), host(), policy.clone(), AcceptSignatures,),
        Err(PluginError::CorruptRegistry(_))
    ));
}
