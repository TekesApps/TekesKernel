use std::process::Command;

use deployment_tests::{SELECTOR_BIN_ENV, configured_binary, run_fixture_checker, workspace_root};

#[test]
fn deployment_fixture_oracle_is_closed_and_disk_complete() {
    let output = run_fixture_checker();
    assert!(
        output.status.success(),
        "fixture checker failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Gates 71-76"));
}

#[test]
fn selector_cli_seam_is_exact_when_configured() {
    let Some(selector) = configured_binary(SELECTOR_BIN_ENV) else {
        eprintln!(
            "{SELECTOR_BIN_ENV} is unset; selector library/CLI suites own the executable path"
        );
        return;
    };
    let output = Command::new(selector)
        .args(["--install-root", "/tmp/tekes-selector-probe", "--unknown"])
        .output()
        .expect("selector usage probe");
    assert_eq!(output.status.code(), Some(64));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"{\"error\":{\"code\":\"usage\",\"details\":{\"argument\":\"--unknown\"},\"message\":\"Command line is invalid\"}}\n"
    );
}

#[test]
fn packaging_scripts_reject_relative_install_paths() {
    let root = workspace_root();
    let output = Command::new("python3")
        .arg(root.join("packaging/macos/render-launch-agent.py"))
        .args(["--template"])
        .arg(root.join("fixtures/deployment/com.tekes.kernel.supervisor.plist"))
        .args([
            "--output",
            "/tmp/tekes-invalid.plist",
            "--install-root",
            "relative",
            "--storage-root",
            "/tmp/threads",
            "--selector",
            "/tmp/selector",
            "--stdout",
            "/tmp/stdout",
            "--stderr",
            "/tmp/stderr",
        ])
        .output()
        .expect("renderer rejection");
    assert!(!output.status.success());
    assert!(!std::path::Path::new("/tmp/tekes-invalid.plist").exists());
}

#[test]
fn production_build_scripts_reject_broad_existing_and_aliased_outputs() {
    let root = workspace_root();
    let source = root.join("packaging/macos/production-uat/main.swift");
    let conformance = root.join("fixtures/deployment/production-uat-contract.canonical.json");
    let invalid_outputs = [
        "/".to_owned(),
        root.display().to_string(),
        root.join("target").display().to_string(),
        root.join("target/../target/aliased-output")
            .display()
            .to_string(),
    ];
    for output in invalid_outputs {
        let runner = Command::new(root.join("packaging/macos/build-production-uat.sh"))
            .args([
                "--identity",
                "TEST",
                "--team-id",
                "TEKESAPP01",
                "--client-requirement",
                "anchor apple generic and identifier com.tekes.client",
                "--profile",
            ])
            .arg(&source)
            .args(["--output", &output])
            .output()
            .expect("production runner build rejection");
        assert_eq!(runner.status.code(), Some(64), "unsafe output {output}");

        let release = Command::new(root.join("packaging/macos/build-signed-release.sh"))
            .args([
                "--identity",
                "TEST",
                "--team-id",
                "TEKESAPP01",
                "--client-requirement",
                "anchor apple generic and identifier com.tekes.client",
                "--supervisor-profile",
            ])
            .arg(&source)
            .args(["--selector"])
            .arg(&source)
            .arg("--supervisor")
            .arg(&source)
            .arg("--worker")
            .arg(&source)
            .arg("--helper")
            .arg(&source)
            .arg("--conformance")
            .arg(&conformance)
            .args(["--output", &output])
            .output()
            .expect("signed release build rejection");
        assert_eq!(release.status.code(), Some(64), "unsafe output {output}");
    }
}

#[test]
fn production_build_scripts_reject_symlink_parent_and_dangling_output() {
    use std::os::unix::fs::symlink;

    let root = workspace_root();
    let target = root.join("target");
    std::fs::create_dir_all(&target).expect("target directory");
    let suffix = std::process::id();
    let outside = std::env::temp_dir().join(format!("tekes-uat-output-outside-{suffix}"));
    std::fs::create_dir_all(&outside).expect("outside directory");
    let parent_link = target.join(format!("uat-parent-link-{suffix}"));
    let dangling = target.join(format!("uat-dangling-output-{suffix}"));
    symlink(&outside, &parent_link).expect("parent symlink");
    symlink(outside.join("missing"), &dangling).expect("dangling output symlink");

    let script = root.join("packaging/macos/build-production-uat.sh");
    let release_script = root.join("packaging/macos/build-signed-release.sh");
    let source = root.join("packaging/macos/production-uat/main.swift");
    let conformance = root.join("fixtures/deployment/production-uat-contract.canonical.json");
    for output in [parent_link.join("artifact"), dangling.clone()] {
        let result = Command::new(&script)
            .args([
                "--identity",
                "TEST",
                "--team-id",
                "TEKESAPP01",
                "--client-requirement",
                "anchor apple generic and identifier com.tekes.client",
                "--profile",
            ])
            .arg(&source)
            .args(["--output"])
            .arg(&output)
            .output()
            .expect("symlink output rejection");
        assert_eq!(
            result.status.code(),
            Some(64),
            "unsafe output {}",
            output.display()
        );

        let release = Command::new(&release_script)
            .args([
                "--identity",
                "TEST",
                "--team-id",
                "TEKESAPP01",
                "--client-requirement",
                "anchor apple generic and identifier com.tekes.client",
                "--supervisor-profile",
            ])
            .arg(&source)
            .args(["--selector"])
            .arg(&source)
            .arg("--supervisor")
            .arg(&source)
            .arg("--worker")
            .arg(&source)
            .arg("--helper")
            .arg(&source)
            .arg("--conformance")
            .arg(&conformance)
            .arg("--output")
            .arg(&output)
            .output()
            .expect("signed release symlink output rejection");
        assert_eq!(
            release.status.code(),
            Some(64),
            "unsafe output {}",
            output.display()
        );
    }

    std::fs::remove_file(parent_link).expect("remove parent symlink");
    std::fs::remove_file(dangling).expect("remove dangling symlink");
    std::fs::remove_dir(outside).expect("remove outside directory");
}

#[test]
fn safe_target_publisher_is_atomic_and_non_overwriting() {
    let root = workspace_root();
    let suffix = std::process::id();
    let source = root
        .join("target")
        .join(format!("uat-publish-source-{suffix}"));
    let output = root
        .join("target")
        .join(format!("uat-publish-destination-{suffix}"));
    std::fs::write(&source, b"signed-artifact").expect("publisher source");
    let helper = root.join("packaging/macos/safe-target-output.py");
    let validated = Command::new("python3")
        .arg(&helper)
        .args(["validate", "--root"])
        .arg(&root)
        .arg("--output")
        .arg(&output)
        .output()
        .expect("validate output");
    assert!(
        validated.status.success(),
        "{}",
        String::from_utf8_lossy(&validated.stderr)
    );
    let published = Command::new("python3")
        .arg(&helper)
        .args(["publish", "--root"])
        .arg(&root)
        .arg("--output")
        .arg(&output)
        .arg("--source")
        .arg(&source)
        .output()
        .expect("publish output");
    assert!(
        published.status.success(),
        "{}",
        String::from_utf8_lossy(&published.stderr)
    );
    assert_eq!(
        std::fs::read(&output).expect("published bytes"),
        b"signed-artifact"
    );
    assert!(!source.exists());

    let duplicate = Command::new("python3")
        .arg(&helper)
        .args(["validate", "--root"])
        .arg(&root)
        .arg("--output")
        .arg(&output)
        .output()
        .expect("reject duplicate output");
    assert!(!duplicate.status.success());
    std::fs::remove_file(output).expect("remove published output");
}

#[test]
fn safe_target_publisher_accepts_only_descendants_of_an_explicit_artifact_root() {
    let root = workspace_root();
    let suffix = std::process::id();
    let temporary = std::fs::canonicalize(std::env::temp_dir()).expect("canonical temp root");
    let artifact_root = temporary.join(format!("tekes-artifact-root-{suffix}"));
    std::fs::create_dir(&artifact_root).expect("external artifact root");
    let source = temporary.join(format!("tekes-artifact-source-{suffix}"));
    let output = artifact_root.join("signed-release");
    std::fs::write(&source, b"signed-artifact").expect("publisher source");
    let helper = root.join("packaging/macos/safe-target-output.py");

    let published = Command::new("python3")
        .arg(&helper)
        .args(["publish", "--root"])
        .arg(&root)
        .arg("--artifact-root")
        .arg(&artifact_root)
        .arg("--output")
        .arg(&output)
        .arg("--source")
        .arg(&source)
        .output()
        .expect("external publish");
    assert!(
        published.status.success(),
        "{}",
        String::from_utf8_lossy(&published.stderr)
    );
    assert_eq!(
        std::fs::read(&output).expect("published bytes"),
        b"signed-artifact"
    );

    let escaped = Command::new("python3")
        .arg(&helper)
        .args(["validate", "--root"])
        .arg(&root)
        .arg("--artifact-root")
        .arg(&artifact_root)
        .arg("--output")
        .arg(temporary.join(format!("tekes-escaped-output-{suffix}")))
        .output()
        .expect("escaped output rejection");
    assert!(!escaped.status.success());

    std::fs::remove_file(output).expect("remove published output");
    std::fs::remove_dir(artifact_root).expect("remove artifact root");
}

#[test]
fn production_uat_phase_graph_is_closed_and_recoverable() {
    let root = workspace_root();
    let bytes =
        std::fs::read(root.join("fixtures/deployment/production-uat-contract.canonical.json"))
            .expect("production UAT fixture");
    let value: serde_json::Value = serde_json::from_slice(&bytes).expect("production UAT JSON");
    assert_eq!(
        value["operation_phases"]["prepare"],
        serde_json::json!(["preparing", "installing", "crash-proved", "prepared"])
    );
    assert_eq!(
        value["operation_phases"]["resume"],
        serde_json::json!([
            "resume-process-proved",
            "resume-client-ready",
            "resume-session-proved",
            "resume-canary-attested",
            "resume-provider-initial",
            "resume-provider-rotated",
            "resume-provider-revoked",
            "resumed"
        ])
    );
    assert_eq!(
        value["operation_phases"]["consume"],
        serde_json::json!([
            "consume-prepared",
            "consume-service-stopped",
            "consume-credentials-deleted",
            "consume-plist-removed",
            "consume-binaries-removed",
            "consume-resume-agent-removed",
            "consume-data-verified",
            "consumed"
        ])
    );
    assert_eq!(
        value["operation_argv"]["acknowledge"],
        serde_json::json!(["--mode", "acknowledge", "--operation", "UUID"])
    );
    assert!(
        value["client_argv"]
            .as_array()
            .expect("client argv")
            .windows(2)
            .any(|pair| pair == ["--operation", "UUID"]),
        "Client session side effects require the operation idempotency key"
    );

    let source = std::fs::read_to_string(root.join("packaging/macos/production-uat/main.swift"))
        .expect("production UAT source");
    let sections = [
        source
            .split("private func prepareProduction")
            .nth(1)
            .unwrap()
            .split("private func operationContext")
            .next()
            .unwrap(),
        source
            .split("private func resumeProduction")
            .nth(1)
            .unwrap()
            .split("private func verifiedEvidence")
            .next()
            .unwrap(),
        source
            .split("private func consumeProduction")
            .nth(1)
            .unwrap()
            .split("private func acknowledgeProduction")
            .next()
            .unwrap(),
    ];
    for (phases, section) in [
        value["operation_phases"]["prepare"].as_array().unwrap(),
        value["operation_phases"]["resume"].as_array().unwrap(),
        value["operation_phases"]["consume"].as_array().unwrap(),
    ]
    .into_iter()
    .zip(sections)
    {
        let mut cursor = 0;
        for phase in phases {
            let needle = format!("phase: \"{}\"", phase.as_str().unwrap());
            let offset = section[cursor..]
                .find(&needle)
                .unwrap_or_else(|| panic!("implementation omits phase {needle}"));
            cursor += offset + needle.len();
        }
    }
    assert!(source.contains("/\\(name).authenticated.canonical.json"));
    let consume = source
        .split("private func consumeProduction")
        .nth(1)
        .unwrap()
        .split("private func acknowledgeProduction")
        .next()
        .unwrap();
    assert!(
        !consume.contains("item.delete()"),
        "consume must retain its authentication key until ACK"
    );
    let acknowledge = source
        .split("private func acknowledgeProduction")
        .nth(1)
        .unwrap();
    assert!(acknowledge.contains("try item.delete()"));
}

#[test]
fn entitled_actor_builders_require_profiled_apps_and_runtime_admission() {
    let root = workspace_root();
    let release = std::fs::read_to_string(root.join("packaging/macos/build-signed-release.sh"))
        .expect("release builder");
    let production = std::fs::read_to_string(root.join("packaging/macos/build-production-uat.sh"))
        .expect("production UAT builder");
    for source in [&release, &production] {
        assert!(source.contains("verify-provisioning-profile.py"));
        assert!(source.contains("embedded.provisionprofile"));
        assert!(source.contains(".app"));
        assert!(source.contains("TEKES_SIGNED_ARTIFACT_ROOT"));
        assert!(source.contains("--artifact-root"));
    }
    assert!(release.contains("--describe-build"));
    assert!(release.contains("--release-version"));
    assert!(release.contains("--source-revision"));
    assert!(release.contains("\"source_revision\": source_revision"));
    assert!(release.contains("authority_registry_sha256"));
    assert!(release.contains("result.stdout != canonical"));
    assert!(release.matches("verify-release.py").count() >= 4);
    assert!(production.contains("--describe-contract"));
    assert!(production.matches("codesign --verify --strict").count() >= 2);
    let runner = std::fs::read_to_string(root.join("packaging/macos/production-uat/main.swift"))
        .expect("production UAT runner");
    assert!(runner.contains("verifyEmbeddedProvisioningProfile("));
    assert!(runner.contains("verifyClientAdmissionProbe("));
    assert!(runner.contains("exit(75)"));
    let oracle: serde_json::Value = serde_json::from_slice(
        &std::fs::read(root.join("fixtures/deployment/provisioning-profiles.canonical.json"))
            .expect("provisioning oracle"),
    )
    .expect("provisioning JSON");
    assert_eq!(oracle["container"], "application-bundle");
    assert_eq!(oracle["profiles"].as_array().expect("profiles").len(), 3);

    let ci =
        std::fs::read_to_string(root.join("scripts/ci-slice10.sh")).expect("Slice-10 coordinator");
    assert!(ci.contains("production_action\" == preflight"));
    assert!(ci.contains("signed preflight passed; external Gates 72/76 remain release-blocking"));
    let full_suite = ci
        .find("cargo test --workspace --all-targets --locked")
        .unwrap();
    let preflight_exit = ci
        .find("if [[ \"$production_action\" == preflight ]]")
        .unwrap();
    assert!(full_suite < preflight_exit);
}

#[test]
fn production_prepare_helper_requires_exit_75_and_exact_canonical_bytes() {
    let root = workspace_root();
    let helper = root.join("packaging/macos/check-production-prepare.py");
    let operation = "018f0000-0000-7000-8000-000000000001";
    let boot = "018f0000-0000-7000-8000-000000000002";
    let evidence = format!(
        "{{\"format\":1,\"gate\":72,\"operation\":\"{operation}\",\"phase\":\"reboot-required\",\"pre_boot_session\":\"{boot}\",\"resume_label\":\"com.tekes.kernel.production-uat.{operation}\"}}\n"
    );
    let run = |status: i32, bytes: &str| {
        Command::new("python3")
            .arg(&helper)
            .arg("72")
            .arg("python3")
            .arg("-c")
            .arg(format!(
                "import sys;sys.stdout.write({bytes:?});sys.exit({status})"
            ))
            .output()
            .expect("run production prepare helper")
    };
    let accepted = run(75, &evidence);
    assert!(accepted.status.success());
    assert!(accepted.stderr.is_empty());
    assert_eq!(accepted.stdout, evidence.as_bytes());

    let wrong_status = run(0, &evidence);
    assert!(!wrong_status.status.success());
    assert!(String::from_utf8_lossy(&wrong_status.stderr).contains("expected 75"));

    let invalid_bytes = run(75, "{}\n");
    assert!(!invalid_bytes.status.success());
    assert!(invalid_bytes.stdout.is_empty());

    let unexpected_stderr = Command::new("python3")
        .arg(&helper)
        .arg("72")
        .arg("python3")
        .arg("-c")
        .arg(format!(
            "import sys;sys.stdout.write({evidence:?});sys.stderr.write('noise');sys.exit(75)"
        ))
        .output()
        .expect("reject prepare stderr");
    assert_eq!(unexpected_stderr.status.code(), Some(1));
    assert!(unexpected_stderr.stdout.is_empty());
}

#[cfg(target_os = "macos")]
#[test]
fn provisioning_verifier_rejects_non_profile_bytes() {
    let root = workspace_root();
    let invalid = root.join("packaging/macos/production-uat/main.swift");
    let output = Command::new("python3")
        .arg(root.join("packaging/macos/verify-provisioning-profile.py"))
        .args(["--profile"])
        .arg(&invalid)
        .args([
            "--team-id",
            "TEKESAPP01",
            "--identifier",
            "com.tekes.kernel.installer",
            "--access-group",
            "TEKESAPP01.com.tekes.shared.endpoint",
        ])
        .output()
        .expect("profile rejection");
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}

#[cfg(target_os = "macos")]
#[test]
fn production_uat_runner_has_closed_byte_contract_and_usage_failure() {
    let root = workspace_root();
    let scratch = std::env::temp_dir().join(format!(
        "tekes-production-uat-contract-{}",
        std::process::id()
    ));
    std::fs::create_dir(&scratch).expect("contract scratch directory");
    let source = scratch.join("main.swift");
    let identity = scratch.join("BuildIdentity.swift");
    let output = scratch.join("tekes-production-uat");
    std::fs::copy(
        root.join("packaging/macos/production-uat/main.swift"),
        &source,
    )
    .expect("stable source copy");
    std::fs::copy(
        root.join("packaging/macos/production-uat/BuildIdentity.test.swift"),
        &identity,
    )
    .expect("stable identity copy");
    let compiled = Command::new("/usr/bin/swiftc")
        .args([
            "-Onone",
            "-framework",
            "Security",
            "-framework",
            "CryptoKit",
        ])
        .arg(&source)
        .arg(&identity)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("compile production UAT contract runner");
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );

    let described = Command::new(&output)
        .arg("--describe-contract")
        .output()
        .expect("describe production UAT contract");
    assert!(described.status.success());
    assert!(described.stderr.is_empty());
    assert_eq!(
        described.stdout,
        std::fs::read(root.join("fixtures/deployment/production-uat-contract.canonical.json"))
            .expect("production UAT contract fixture")
    );

    let invalid = Command::new(&output)
        .args(["--mode", "prepare", "--gate", "72"])
        .output()
        .expect("reject partial production argv");
    assert_eq!(invalid.status.code(), Some(64));
    assert!(invalid.stdout.is_empty());
    assert_eq!(
        invalid.stderr,
        b"{\"error\":{\"code\":\"usage\",\"message\":\"expected the closed production argv\"}}\n"
    );
    std::fs::remove_dir_all(scratch).expect("remove contract scratch");
}

#[test]
fn signed_release_requires_a_full_source_revision() {
    let root = workspace_root();
    let input = root.join("packaging/macos/production-uat/main.swift");
    let output = root.join("target/source-revision-rejection");
    for revision in [
        "",
        "main",
        "C871EC2DC5CB4BFBDF79BC6CE23E91FD63318A9C",
        "c871ec2",
    ] {
        let release = Command::new(root.join("packaging/macos/build-signed-release.sh"))
            .args(["--identity", "TEST", "--team-id", "TEKESAPP01"])
            .args(["--release-version", "1.0.0"])
            .args([
                "--client-requirement",
                "anchor apple generic and identifier com.tekes.client",
            ])
            .arg("--supervisor-profile")
            .arg(&input)
            .arg("--selector")
            .arg(&input)
            .arg("--supervisor")
            .arg(&input)
            .arg("--worker")
            .arg(&input)
            .arg("--helper")
            .arg(&input)
            .arg("--conformance")
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .arg("--workspace-service")
            .arg(&input)
            .args(["--source-revision", revision])
            .output()
            .expect("signed release revision rejection");
        assert_eq!(release.status.code(), Some(64), "revision {revision:?}");
        assert!(!output.exists());
    }
}
