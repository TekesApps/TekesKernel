use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn fixture(name: &str) -> Value {
    let path = root().join("fixtures/deployment").join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

fn check_oracle() {
    let status = Command::new("python3")
        .arg(root().join("scripts/check-deployment-fixtures.py"))
        .current_dir(root())
        .status()
        .expect("run deployment fixture checker");
    assert!(status.success(), "deployment fixture checker failed");
}

fn scratch(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "tekes-slice10-{label}-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("create scratch directory");
    path
}

fn optional_selector_usage_probe() {
    let Ok(selector) = std::env::var("TEKES_SLICE10_SELECTOR_BIN") else {
        eprintln!(
            "Slice-10 selector executable seam: set TEKES_SLICE10_SELECTOR_BIN to enable exact CLI probing"
        );
        return;
    };
    let output = Command::new(selector)
        .args(["--install-root", "/tmp/tekes-selector-probe", "--unknown"])
        .output()
        .expect("run selector usage probe");
    assert_eq!(output.status.code(), Some(64));
    assert!(output.stdout.is_empty());
    let selector_oracle = fixture("selector.canonical.json");
    let row = selector_oracle["error_rows"]
        .as_array()
        .expect("error rows")
        .iter()
        .find(|row| row["code"] == "usage")
        .expect("usage row");
    let expected = serde_json_canonicalizer::to_vec(&serde_json::json!({"error": row}))
        .expect("canonical usage error");
    assert_eq!(output.stderr, [expected, b"\n".to_vec()].concat());
}

#[test]
fn slice10_portable_oracle_71_deployment_filesystem_and_ownership() {
    check_oracle();
    let cloud = fixture("cases/cloud-managed-storage.expected.canonical.json");
    assert_eq!(cloud["readiness"]["code"], "unsupported-filesystem");
    let work = scratch("gate71");
    let output = work.join("LaunchAgents/com.tekes.kernel.supervisor.plist");
    let install = "/Users/test/Library/Application Support/Tekes/Kernel";
    let storage = "/Users/test/.agents/threads";
    let selector =
        "/Users/test/Library/Application Support/Tekes/Kernel/selector/bin/tekes-selector";
    let status = Command::new("python3")
        .arg(root().join("packaging/macos/render-launch-agent.py"))
        .args(["--template"])
        .arg(root().join("fixtures/deployment/com.tekes.kernel.supervisor.plist"))
        .args(["--output"])
        .arg(&output)
        .args([
            "--install-root",
            install,
            "--storage-root",
            storage,
            "--selector",
            selector,
            "--stdout",
            "/Users/test/.agents/logs/kernel/stdout.log",
            "--stderr",
            "/Users/test/.agents/logs/kernel/stderr.log",
        ])
        .status()
        .expect("render LaunchAgent");
    assert!(status.success());
    assert_eq!(
        fs::metadata(&output)
            .expect("plist metadata")
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
    let plist = fs::read_to_string(&output).expect("rendered plist");
    for value in [install, storage, selector, "127.0.0.1:7347"] {
        assert!(plist.contains(value), "rendered plist omits {value}");
    }
    assert!(!plist.contains("threads/threads"));
    assert!(!plist.contains("EnvironmentVariables"));
    fs::remove_dir_all(work).expect("remove scratch directory");
}

#[test]
fn slice10_portable_oracle_72_launchd_crash_recovery() {
    check_oracle();
    let bootstrap = fixture("bootstrap-status.canonical.json");
    let codes = bootstrap["failures"]
        .as_array()
        .expect("bootstrap rows")
        .iter()
        .map(|row| row["code"].as_str().expect("bootstrap code"))
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        [
            "already-running",
            "invalid-install",
            "unsupported-filesystem",
            "invalid-config",
            "corrupt-ledger",
            "protocol-mismatch",
            "required-broker-unavailable",
            "selector-mismatch",
            "listener-unavailable",
            "endpoint-credential-unavailable",
            "io",
        ]
    );
    let cases = [
        "reboot-start",
        "reboot-target-user-login",
        "selector-sigkill-recovery",
        "live-drain",
    ];
    for case in cases {
        let expected = fixture(&format!("cases/{case}.expected.canonical.json"));
        assert_eq!(expected["gate"], 72);
        assert_eq!(expected["processes"]["orphans"], 0);
    }
}

#[test]
fn slice10_portable_oracle_73_install_upgrade_publication_crash_matrix() {
    check_oracle();
    let machine = fixture("publication-state-machine.canonical.json");
    assert_eq!(
        machine["stage"],
        serde_json::json!([
            "prepared",
            "copied",
            "verified",
            "bundle-published",
            "closed"
        ])
    );
    assert_eq!(
        machine["decision_of_record"],
        "active-link-after-link-published"
    );
    assert_eq!(machine["corruption_exit"], 75);
    let installer = fixture("installer-state-machine.canonical.json");
    assert_eq!(
        installer["closed_retry"]["response"]["operation"],
        "uninstall"
    );
    optional_selector_usage_probe();
}

#[test]
fn slice10_portable_oracle_74_crash_loop_external_rollback() {
    check_oracle();
    let oracle = fixture("failure-attribution.canonical.json");
    assert_eq!(oracle["threshold"], 3);
    assert_eq!(oracle["retry_seconds"], serde_json::json!([1, 2, 4, 8, 30]));
    assert_eq!(
        oracle["promotion"]["comparator"],
        "failure-commit-now-less-than-window-close"
    );
    let automatic =
        &fixture("publication-state-machine.canonical.json")["operation_shapes"]["rollback_auto"];
    assert_eq!(automatic["actor"], "serve");
    assert!(automatic.get("response").is_none());
}

#[test]
fn slice10_portable_oracle_75_production_observability_redaction() {
    check_oracle();
    let contract = fixture("observability.canonical.json");
    let support = root().join("fixtures/deployment/support-bundle");
    let joined = fs::read_dir(&support)
        .expect("support bundle")
        .flat_map(|entry| fs::read(entry.expect("support entry").path()).expect("support file"))
        .collect::<Vec<_>>();
    for canary in contract["forbidden_canaries"].as_array().expect("canaries") {
        let needle = canary.as_str().expect("canary").as_bytes();
        assert!(!joined.windows(needle.len()).any(|window| window == needle));
    }
    assert_eq!(
        contract["metric_names"].as_array().expect("metrics").len(),
        17
    );
}

#[test]
fn slice10_portable_oracle_76_clean_install_client_uninstall_e2e() {
    check_oracle();
    for case in [
        "clean-install",
        "archive-preservation",
        "uninstall-retains-data",
        "installer-uninstall-recovery",
    ] {
        let steps = fixture(&format!("cases/{case}.steps.canonical.json"));
        let expected = fixture(&format!("cases/{case}.expected.canonical.json"));
        assert_eq!(steps["expected_selection"], expected["selection"]);
        assert_eq!(steps["expected_readiness"], expected["readiness"]);
        assert_eq!(expected["processes"]["orphans"], 0);
    }
    eprintln!(
        "Gate 76 portable checks validate only the closed fixture oracle; signed installer/Client, launchd, Keychain, reboot and APFS execution remain mandatory production evidence"
    );
}
