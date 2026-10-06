use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tekes_selector::{
    BundleFile, BundleManifest, CodeSignature, CodeSignatureVerifier, Compatibility,
    InstallIdentity, Observation, ObservationState, OperationActor, OperationType, PreviousFile,
    Selection, SelectionFile, Selector, SelectorError, SelectorFile, SelectorManifest,
    SelectorOperation, Signing, cli_command_sha256,
};
use tempfile::TempDir;

const TEAM: &str = "TEKESAPP01";
const REQUIREMENT: &str = "anchor apple generic and identifier com.tekes.kernel";
const REGISTRY: &str = "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469";

#[derive(Clone, Copy)]
struct TestSignatures;

impl CodeSignatureVerifier for TestSignatures {
    fn verify(&self, executable: &Path, requirement: &str) -> Result<CodeSignature, SelectorError> {
        let expected = match executable.file_name().and_then(|name| name.to_str()) {
            Some("tekes-worker") => "anchor apple generic and identifier com.tekes.kernel.worker",
            Some("tekes-helper") => "anchor apple generic and identifier com.tekes.kernel.helper",
            _ => REQUIREMENT,
        };
        if requirement != expected {
            return Err(SelectorError::invalid_bundle(
                executable.display().to_string(),
            ));
        }
        Ok(CodeSignature {
            architectures: vec!["aarch64".to_owned()],
            team_id: TEAM.to_owned(),
        })
    }

    fn verify_provisioned_app(
        &self,
        app_bundle: &Path,
        requirement: &str,
        team_id: &str,
        bundle_identifier: &str,
        required_access_groups: &[String],
    ) -> Result<(), SelectorError> {
        let expected_groups = [
            format!("{TEAM}.com.tekes.shared.endpoint"),
            format!("{TEAM}.com.tekes.kernel.provider-secrets"),
        ];
        if requirement != REQUIREMENT
            || team_id != TEAM
            || bundle_identifier != "com.tekes.kernel.supervisor"
            || required_access_groups != expected_groups
            || fs::read(app_bundle.join("Contents/embedded.provisionprofile")).map_or(
                true,
                |bytes| {
                    bytes == b"invalid-profile"
                        || bytes == b"reject-in-staging"
                            && app_bundle
                                .ancestors()
                                .any(|path| path.to_string_lossy().ends_with(".staging"))
                },
            )
        {
            return Err(SelectorError::invalid_bundle(
                app_bundle.display().to_string(),
            ));
        }
        Ok(())
    }
}

#[test]
fn slice10_gate_73_install_upgrade_publication_crash_matrix() {
    for phase in [
        "prepared",
        "link-published",
        "previous-published",
        "current-published",
    ] {
        let fixture = Fixture::new();
        let (v1, v2) = fixture.stage_upgrade_pair();
        let generation = 2;
        if phase != "prepared" {
            fs::remove_file(&fixture.selector.paths().active).expect("remove active");
            symlink(
                Path::new("../bundles/2.0.0"),
                &fixture.selector.paths().active,
            )
            .expect("publish link");
        }
        if matches!(phase, "previous-published" | "current-published") {
            write_canonical(
                &fixture.selector.paths().previous,
                &PreviousFile {
                    format: 1,
                    generation,
                    selection: Some(v1.clone()),
                },
            );
        }
        if phase == "current-published" {
            write_canonical(
                &fixture.selector.paths().current,
                &SelectionFile {
                    format: 1,
                    generation,
                    selection: v2.clone(),
                },
            );
        }
        let argv = vec![
            "--install-root".to_owned(),
            fixture.kernel.display().to_string(),
            "activate".to_owned(),
            "--version".to_owned(),
            "2.0.0".to_owned(),
        ];
        write_canonical(
            &fixture.selector.paths().operation,
            &SelectorOperation {
                actor: OperationActor::Cli,
                command_sha256: cli_command_sha256(&argv).expect("command digest"),
                format: 1,
                from: Some(v1),
                generation,
                launch_id: None,
                op_id: "activate-2".to_owned(),
                operation_type: OperationType::Activate,
                phase: phase.to_owned(),
                reason: None,
                response: None,
                to: v2.clone(),
            },
        );

        let recovered = fixture.selector.recover().expect("recover activation");
        assert!(recovered.recovered, "phase {phase} must be recovered");
        let status = fixture.selector.status().expect("status after recovery");
        assert_eq!(status.selection.expect("current").selection, v2);
        assert_eq!(status.previous.expect("previous").generation, 2);
    }
}

#[test]
fn first_activation_recovers_every_publication_phase() {
    for phase in [
        "prepared",
        "link-published",
        "previous-published",
        "current-published",
    ] {
        let fixture = Fixture::new();
        let source = fixture.make_bundle("1.0.0", b'1');
        let selection = fixture
            .selector
            .stage(
                &source,
                "1.0.0",
                cli_command_sha256(&stage_argv(&fixture.kernel, &source, "1.0.0")).expect("hash"),
            )
            .expect("stage")
            .selection;
        if phase != "prepared" {
            symlink(
                Path::new("../bundles/1.0.0"),
                &fixture.selector.paths().active,
            )
            .expect("publish first link");
        }
        if matches!(phase, "previous-published" | "current-published") {
            write_canonical(
                &fixture.selector.paths().previous,
                &PreviousFile {
                    format: 1,
                    generation: 1,
                    selection: None,
                },
            );
        }
        if phase == "current-published" {
            write_canonical(
                &fixture.selector.paths().current,
                &SelectionFile {
                    format: 1,
                    generation: 1,
                    selection: selection.clone(),
                },
            );
        }
        write_canonical(
            &fixture.selector.paths().operation,
            &SelectorOperation {
                actor: OperationActor::Cli,
                command_sha256: "c".repeat(64),
                format: 1,
                from: None,
                generation: 1,
                launch_id: None,
                op_id: "activate-1".to_owned(),
                operation_type: OperationType::Activate,
                phase: phase.to_owned(),
                reason: None,
                response: None,
                to: selection.clone(),
            },
        );
        assert!(fixture.selector.recover().expect("recover").recovered);
        let logs = fs::read_to_string(&fixture.selector.paths().operational_log)
            .expect("selector recovery log");
        assert!(logs.lines().any(|line| {
            line.contains("\"code\":\"selector-recovered\"")
                && line.contains("\"phase\":\"closed\"")
        }));
        let status = fixture.selector.status().expect("status");
        assert_eq!(status.selection.expect("current").selection, selection);
        assert_eq!(status.previous.expect("previous").selection, None);
    }
}

#[test]
fn stage_prepared_partial_copy_is_removed_without_publication() {
    let fixture = Fixture::new();
    let source = fixture.make_bundle("1.0.0", b'1');
    let manifest_bytes = fs::read(source.join("manifest.canonical.json")).expect("manifest");
    let selection = Selection {
        manifest_sha256: format!("{:x}", Sha256::digest(manifest_bytes)),
        version: "1.0.0".to_owned(),
    };
    let op_id = "stage-partial";
    let staging = fixture
        .selector
        .paths()
        .bundles
        .join(format!(".{op_id}.staging"));
    fs::create_dir(&staging).expect("partial staging");
    fs::write(staging.join("partial"), b"torn").expect("partial bytes");
    write_canonical(
        &fixture.selector.paths().operation,
        &SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256: "d".repeat(64),
            format: 1,
            from: None,
            generation: 0,
            launch_id: None,
            op_id: op_id.to_owned(),
            operation_type: OperationType::Stage,
            phase: "prepared".to_owned(),
            reason: None,
            response: None,
            to: selection,
        },
    );
    let reply = fixture.selector.recover().expect("recover prepared stage");
    assert!(reply.recovered);
    assert!(!staging.exists());
    assert!(!fixture.selector.paths().operation.exists());
    assert!(!fixture.selector.paths().bundles.join("1.0.0").exists());
}

#[test]
fn slice10_gate_74_crash_loop_external_rollback() {
    let fixture = Fixture::new();
    let v1 = fixture.stage_bundle("1.0.0", supervisor_script("invalid-config"));
    fixture.activate("1.0.0");
    let v2 = fixture.stage_bundle("2.0.0", supervisor_script("crash"));
    fixture.activate("2.0.0");

    let mut resident = spawn_serve(&fixture);
    wait_until("serve-authored rollback", || {
        read_canonical_if_present::<SelectionFile>(&fixture.selector.paths().current)
            .is_some_and(|current| current.generation == 3 && current.selection == v1)
    });
    stop_serve(&mut resident);

    let status = fixture.selector.status().expect("status after rollback");
    let current = status.selection.expect("current selection");
    assert_eq!(current.generation, 3);
    assert_eq!(current.selection, v1);
    assert_eq!(status.previous.expect("previous").selection, Some(v2));
    let operation: SelectorOperation = read_canonical(&fixture.selector.paths().operation);
    assert_eq!(operation.actor, OperationActor::Serve);
    assert_eq!(operation.phase, "closed");
    assert!(
        operation.response.is_none(),
        "serve rollback has no CLI response"
    );
    let logs = fs::read_to_string(&fixture.selector.paths().operational_log)
        .expect("selector operational log");
    let launch_ids = selector_failure_launch_ids(&logs, "unexpected-child-exit", 2);
    assert_eq!(
        launch_ids.len(),
        3,
        "three distinct process launches must fail"
    );
    assert!(
        launch_ids
            .iter()
            .any(|id| operation.launch_id.as_ref() == Some(id))
    );
    for code in [
        "unexpected-child-exit",
        "readiness-crash-loop",
        "rollback-complete",
    ] {
        assert!(
            logs.contains(&format!("\"code\":\"{code}\"")),
            "missing {code}"
        );
    }
    assert!(!logs.contains("secret"));

    for code in [
        "invalid-config",
        "required-broker-unavailable",
        "listener-unavailable",
    ] {
        let control = Fixture::new();
        control.stage_bundle("1.0.0", supervisor_script("invalid-config"));
        control.activate("1.0.0");
        control.stage_bundle("2.0.0", supervisor_script(code));
        control.activate("2.0.0");
        let mut resident = spawn_serve(&control);
        wait_until(code, || {
            read_canonical_if_present::<Observation>(
                &control.selector.paths().observations.join("2.0.0.json"),
            )
            .is_some_and(|observation| {
                observation.last_code == code && observation.consecutive_failures == 0
            })
        });
        stop_serve(&mut resident);
        let status = control.selector.status().expect("environment status");
        assert_eq!(status.selection.expect("selection").generation, 2);
        assert_eq!(
            status
                .observation
                .expect("observation")
                .consecutive_failures,
            0
        );
    }

    let pre_boundary = Fixture::new();
    pre_boundary.stage_bundle("1.0.0", supervisor_script("invalid-config"));
    pre_boundary.activate("1.0.0");
    let selected = pre_boundary.stage_bundle("2.0.0", supervisor_script("crash"));
    pre_boundary.activate("2.0.0");
    write_ready_observation(&pre_boundary, &selected, "2999-08-28T00:00:01.000000000Z");
    let mut resident = spawn_serve(&pre_boundary);
    wait_until("pre-boundary attributable failure", || {
        read_canonical_if_present::<Observation>(
            &pre_boundary
                .selector
                .paths()
                .observations
                .join("2.0.0.json"),
        )
        .is_some_and(|observation| observation.consecutive_failures >= 1)
    });
    stop_serve(&mut resident);
    let observation = pre_boundary
        .selector
        .status()
        .expect("pre-boundary status")
        .observation
        .expect("pre-boundary observation");
    assert!(observation.rollback_eligible);
    assert_eq!(observation.consecutive_failures, 1);

    let promoted = Fixture::new();
    promoted.stage_bundle("1.0.0", supervisor_script("invalid-config"));
    promoted.activate("1.0.0");
    let selected = promoted.stage_bundle("2.0.0", supervisor_script("controlled"));
    promoted.activate("2.0.0");
    write_ready_observation(&promoted, &selected, "2026-08-28T00:00:01.000000000Z");
    fs::write(promoted.storage_root().join("selector-test-mode"), b"ready").expect("ready control");
    let mut resident = spawn_serve(&promoted);
    wait_until("promotion", || {
        read_canonical_if_present::<Observation>(
            &promoted.selector.paths().observations.join("2.0.0.json"),
        )
        .is_some_and(|observation| observation.state == ObservationState::Promoted)
    });
    stop_serve(&mut resident);
    fs::write(promoted.storage_root().join("selector-test-mode"), b"crash").expect("crash control");
    let mut resident = spawn_serve(&promoted);
    wait_until("post-promotion crash loop", || {
        read_canonical_if_present::<Observation>(
            &promoted.selector.paths().observations.join("2.0.0.json"),
        )
        .is_some_and(|observation| observation.consecutive_failures == 3)
    });
    stop_serve(&mut resident);
    let status = promoted.selector.status().expect("post-promotion status");
    assert_eq!(status.selection.expect("selection").generation, 2);
    let observation = status.observation.expect("observation");
    assert_eq!(observation.state, ObservationState::Failed);
    assert_eq!(observation.consecutive_failures, 3);
    assert!(!observation.rollback_eligible);
}

#[test]
fn selector_serve_process_harness_child() {
    let Ok(install_root) = std::env::var("TEKES_SELECTOR_HARNESS_INSTALL_ROOT") else {
        return;
    };
    let storage_root =
        std::env::var("TEKES_SELECTOR_HARNESS_STORAGE_ROOT").expect("harness storage root");
    let listen = std::env::var("TEKES_SELECTOR_HARNESS_LISTEN").expect("harness listen endpoint");
    Selector::new(install_root, TestSignatures)
        .serve_test_loopback(Path::new(&storage_root), &listen)
        .expect("resident selector serve");
}

fn spawn_serve(fixture: &Fixture) -> Child {
    let listen = isolated_test_listen();
    Command::new(std::env::current_exe().expect("test executable"))
        .args([
            "--exact",
            "selector_serve_process_harness_child",
            "--nocapture",
        ])
        .env("TEKES_SELECTOR_HARNESS_INSTALL_ROOT", &fixture.kernel)
        .env(
            "TEKES_SELECTOR_HARNESS_STORAGE_ROOT",
            fixture.storage_root(),
        )
        .env("TEKES_SELECTOR_HARNESS_LISTEN", listen)
        .spawn()
        .expect("spawn resident selector harness")
}

fn isolated_test_listen() -> String {
    loop {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("allocate test listener");
        let address = listener.local_addr().expect("allocated listener address");
        if address.port() != 7347 {
            return address.to_string();
        }
    }
}

#[test]
fn selector_harness_uses_an_os_assigned_nonproduction_endpoint() {
    for _ in 0..4 {
        let listen = isolated_test_listen();
        let address: SocketAddr = listen.parse().expect("parse allocated endpoint");
        assert!(address.ip().is_loopback());
        assert_ne!(address.port(), 0);
        assert_ne!(address.port(), 7347);
    }

    let fixture = Fixture::new();
    let error = fixture
        .selector
        .serve_test_loopback(&fixture.storage_root(), "127.0.0.1:7347")
        .expect_err("test seam must never use the production endpoint");
    assert_eq!(error.code, tekes_selector::ErrorCode::Usage);
}

fn stop_serve(child: &mut Child) {
    if child.try_wait().expect("poll resident").is_none() {
        // SAFETY: the PID belongs to the live child owned by this test.
        assert_eq!(
            unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) },
            0
        );
        let deadline = Instant::now() + Duration::from_secs(5);
        while child.try_wait().expect("wait resident").is_none() {
            assert!(Instant::now() < deadline, "resident selector did not drain");
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

fn wait_until(label: &str, mut predicate: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(15);
    while !predicate() {
        assert!(Instant::now() < deadline, "timed out waiting for {label}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn selector_failure_launch_ids(log: &str, code: &str, generation: u64) -> Vec<String> {
    let mut ids = log
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|record| {
            record["code"] == code && record["correlation"]["generation"] == generation
        })
        .filter_map(|record| {
            record["correlation"]["launch_id"]
                .as_str()
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    ids
}

fn read_canonical_if_present<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(bytes.strip_suffix(b"\n")?).ok()
}

fn write_ready_observation(fixture: &Fixture, selected: &Selection, window_closes_at: &str) {
    write_canonical(
        &fixture.selector.paths().observations.join("2.0.0.json"),
        &Observation {
            attempt: 1,
            canary_deadline_at: None,
            canary_required: false,
            canary_run: Some("canary-run".to_owned()),
            canary_session: Some("123e4567-e89b-12d3-a456-426614174000".to_owned()),
            consecutive_failures: 0,
            deadline_at: "2026-08-28T00:00:30.000000000Z".to_owned(),
            format: 1,
            generation: 2,
            last_code: "ready".to_owned(),
            launch_id: "2-1-0123456789abcdef0123456789abcdef".to_owned(),
            manifest_sha256: selected.manifest_sha256.clone(),
            rollback_eligible: true,
            started_at: "2026-08-28T00:00:00.000000000Z".to_owned(),
            state: ObservationState::Ready,
            version: selected.version.clone(),
            window_closes_at: Some(window_closes_at.to_owned()),
        },
    );
}

fn supervisor_script(mode: &str) -> Vec<u8> {
    format!(
        r#"#!/usr/bin/env python3
import json, os, socket, sys, time
args = dict(zip(sys.argv[1::2], sys.argv[2::2]))
mode = {mode:?}
if mode == "controlled":
    with open(os.path.join(args["--storage-root"], "selector-test-mode"), "r", encoding="utf-8") as stream:
        mode = stream.read().strip()
if mode == "crash":
    os._exit(70)
selection = {{"manifest_sha256": args["--manifest-sha256"], "version": args["--selected-version"]}}
if mode != "ready":
    row = {{"code": mode, "format": 1, "launch_id": args["--launch-id"], "selection": selection, "state": "failed"}}
    os.write(3, (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode())
    time.sleep(60)
    raise SystemExit(70)
row = {{"format": 1, "launch_id": args["--launch-id"], "selection": selection, "state": "listener-bound"}}
os.write(3, (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode())
listener = socket.socket()
listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
host, port = args["--listen"].rsplit(":", 1)
listener.bind((host, int(port)))
listener.listen(4)
body = json.dumps({{"build": args["--selected-version"], "generation": int(args["--selector-generation"]), "ready": True}}, sort_keys=True, separators=(",", ":")).encode()
while True:
    client, _ = listener.accept()
    client.recv(4096)
    client.sendall(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n" + body)
    client.close()
"#
    )
    .into_bytes()
}

#[test]
fn offline_mutation_requires_proof_that_the_old_child_group_released_root_lock() {
    let fixture = Fixture::new();
    let source = fixture.make_bundle("1.0.0", b'1');
    fixture
        .selector
        .stage(
            &source,
            "1.0.0",
            cli_command_sha256(&stage_argv(&fixture.kernel, &source, "1.0.0")).expect("hash"),
        )
        .expect("stage");
    let owner = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(fixture.selector.paths().storage_root.join(".root-lock"))
        .expect("open root lock");
    // SAFETY: owner remains live through the assertion.
    assert_eq!(
        unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0
    );
    let error = fixture
        .selector
        .activate("1.0.0", "a".repeat(64))
        .expect_err("old child group ownership must block mutation");
    assert_eq!(error.code, tekes_selector::ErrorCode::InvalidState);
    assert!(!fixture.selector.paths().active.exists());
    drop(owner);
    fixture
        .selector
        .activate("1.0.0", "a".repeat(64))
        .expect("activation after group exit proof");
}

#[test]
fn stage_rejects_an_existing_version_with_different_durable_bytes() {
    let fixture = Fixture::new();
    let first = fixture.make_bundle("1.0.0", b'1');
    let first_reply = fixture
        .selector
        .stage(
            &first,
            "1.0.0",
            cli_command_sha256(&stage_argv(&fixture.kernel, &first, "1.0.0")).expect("hash"),
        )
        .expect("first stage");
    let operation_before = fs::read(&fixture.selector.paths().operation).expect("closed operation");

    let conflicting = fixture.make_bundle("1.0.0", b'9');
    let error = fixture
        .selector
        .stage(
            &conflicting,
            "1.0.0",
            cli_command_sha256(&stage_argv(&fixture.kernel, &conflicting, "1.0.0")).expect("hash"),
        )
        .expect_err("immutable version collision must fail");
    assert_eq!(error.code, tekes_selector::ErrorCode::InvalidState);
    assert_eq!(
        error.details,
        serde_json::json!({"state": "version-already-published"})
    );
    assert_eq!(
        fs::read(&fixture.selector.paths().operation).expect("operation after rejection"),
        operation_before,
        "rejection must not mutate the retained closed request"
    );
    let manifest = fs::read(
        fixture
            .selector
            .paths()
            .bundles
            .join("1.0.0/manifest.canonical.json"),
    )
    .expect("published manifest");
    assert_eq!(
        format!("{:x}", Sha256::digest(manifest)),
        first_reply.selection.manifest_sha256
    );
}

#[test]
fn prune_removes_unselected_versions_so_a_version_can_be_staged_again() {
    // https://github.com/TekesApps/TekesKernel/issues/4
    let fixture = Fixture::new();
    let stage = |version: &str, byte: u8| {
        let source = fixture.make_bundle(version, byte);
        fixture
            .selector
            .stage(
                &source,
                version,
                cli_command_sha256(&stage_argv(&fixture.kernel, &source, version)).expect("hash"),
            )
            .expect("stage")
            .selection
    };
    stage("1.0.0", b'1');
    fixture.activate("1.0.0");
    stage("2.0.0", b'2');
    let unselected = stage("3.0.0", b'3');
    fixture.activate("2.0.0");
    let paths = fixture.selector.paths();
    write_canonical(
        &paths.observations.join("3.0.0.json"),
        &Observation {
            attempt: 1,
            canary_deadline_at: None,
            canary_required: false,
            canary_run: None,
            canary_session: None,
            consecutive_failures: 2,
            deadline_at: "2026-08-28T00:00:30.000000000Z".to_owned(),
            format: 1,
            generation: 9,
            last_code: "readiness-crash-loop".to_owned(),
            launch_id: "9-1-0123456789abcdef0123456789abcdef".to_owned(),
            manifest_sha256: unselected.manifest_sha256.clone(),
            rollback_eligible: false,
            started_at: "2026-08-28T00:00:00.000000000Z".to_owned(),
            state: ObservationState::Failed,
            version: "3.0.0".to_owned(),
            window_closes_at: None,
        },
    );
    let interrupted = paths.bundles.join(".9.0.0.prune");
    fs::create_dir_all(interrupted.join("bin")).expect("interrupted prune");

    let reply = fixture.selector.prune().expect("prune");
    assert_eq!(reply.removed, ["3.0.0"]);
    assert!(!paths.bundles.join("3.0.0").exists());
    assert!(!paths.observations.join("3.0.0.json").exists());
    assert!(!interrupted.exists(), "an interrupted prune is finished");
    assert!(
        paths.bundles.join("1.0.0").is_dir(),
        "previous selection stays"
    );
    assert!(
        paths.bundles.join("2.0.0").is_dir(),
        "current selection stays"
    );
    fixture.selector.status().expect("status after prune");

    let restaged = stage("3.0.0", b'7');
    assert_ne!(
        restaged, unselected,
        "a new build reuses the pruned version"
    );
    let reply = fixture.selector.prune().expect("second prune");
    assert!(
        reply.removed.is_empty(),
        "the closed stage record keeps its bundle"
    );
    fixture
        .selector
        .recover()
        .expect("closed stage record still validates");
}

#[test]
fn recovery_fails_closed_on_conflicting_derived_state_and_extra_operation_files() {
    let fixture = Fixture::new();
    let (v1, v2) = fixture.stage_upgrade_pair();
    write_canonical(
        &fixture.selector.paths().operation,
        &SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256: "a".repeat(64),
            format: 1,
            from: Some(v1),
            generation: 2,
            launch_id: None,
            op_id: "activate-2".to_owned(),
            operation_type: OperationType::Activate,
            phase: "prepared".to_owned(),
            reason: None,
            response: None,
            to: v2.clone(),
        },
    );
    write_canonical(
        &fixture.selector.paths().current,
        &SelectionFile {
            format: 1,
            generation: 99,
            selection: v2,
        },
    );
    let bytes_before = fs::read(&fixture.selector.paths().current).expect("conflicting current");
    let error = fixture
        .selector
        .recover()
        .expect_err("recovery must not overwrite unexplained durable bytes");
    assert_eq!(error.code, tekes_selector::ErrorCode::Corruption);
    assert_eq!(
        fs::read(&fixture.selector.paths().current).expect("current after failure"),
        bytes_before
    );

    fs::write(
        fixture.selector.paths().operations.join("stray.json"),
        b"{}\n",
    )
    .expect("inject stray operation file");
    let error = fixture
        .selector
        .recover()
        .expect_err("operations directory is closed");
    assert_eq!(error.code, tekes_selector::ErrorCode::Corruption);
}

#[test]
fn ordinary_restart_preserves_accepted_canary_attribution_and_window() {
    let fixture = Fixture::new();
    let (_, v2) = fixture.stage_upgrade_pair();
    fixture
        .selector
        .activate("2.0.0", "b".repeat(64))
        .expect("activate candidate");
    let session = "123e4567-e89b-12d3-a456-426614174000";
    write_canonical(
        &fixture.selector.paths().observations.join("2.0.0.json"),
        &Observation {
            attempt: 1,
            canary_deadline_at: Some("2026-08-28T00:02:00.000000000Z".to_owned()),
            canary_required: true,
            canary_run: Some("canary-run".to_owned()),
            canary_session: Some(session.to_owned()),
            consecutive_failures: 0,
            deadline_at: "2026-08-28T00:00:30.000000000Z".to_owned(),
            format: 1,
            generation: 2,
            last_code: "ready".to_owned(),
            launch_id: "2-1-0123456789abcdef0123456789abcdef".to_owned(),
            manifest_sha256: v2.manifest_sha256.clone(),
            rollback_eligible: true,
            started_at: "2026-08-28T00:00:00.000000000Z".to_owned(),
            state: ObservationState::Ready,
            version: v2.version.clone(),
            window_closes_at: Some("2026-08-28T00:07:00.000000000Z".to_owned()),
        },
    );
    fixture
        .selector
        .begin_observation(Observation {
            attempt: 2,
            canary_deadline_at: None,
            canary_required: false,
            canary_run: None,
            canary_session: None,
            consecutive_failures: 0,
            deadline_at: "2026-08-28T00:01:30.000000000Z".to_owned(),
            format: 1,
            generation: 2,
            last_code: "launching".to_owned(),
            launch_id: "2-2-fedcba9876543210fedcba9876543210".to_owned(),
            manifest_sha256: v2.manifest_sha256,
            rollback_eligible: true,
            started_at: "2026-08-28T00:01:00.000000000Z".to_owned(),
            state: ObservationState::Pending,
            version: v2.version,
            window_closes_at: None,
        })
        .expect("begin ordinary restart");
    let observation = fixture
        .selector
        .status()
        .expect("status")
        .observation
        .expect("observation");
    assert_eq!(observation.canary_session.as_deref(), Some(session));
    assert_eq!(observation.canary_run.as_deref(), Some("canary-run"));
    assert_eq!(
        observation.window_closes_at.as_deref(),
        Some("2026-08-28T00:07:00.000000000Z")
    );
    assert!(!observation.canary_required);
    assert!(observation.canary_deadline_at.is_none());
}

#[test]
fn consumer_install_health_admission_requires_exact_live_build_and_generation() {
    let fixture = Fixture::new();
    let (_, v2) = fixture.stage_upgrade_pair();
    fixture.activate("2.0.0");
    write_canonical(
        &fixture.selector.paths().observations.join("2.0.0.json"),
        &Observation {
            attempt: 1,
            canary_deadline_at: Some("2099-08-31T00:02:00.000000000Z".to_owned()),
            canary_required: true,
            canary_run: None,
            canary_session: None,
            consecutive_failures: 0,
            deadline_at: "2099-08-31T00:00:30.000000000Z".to_owned(),
            format: 1,
            generation: 2,
            last_code: "listener-bound".to_owned(),
            launch_id: "2-1-0123456789abcdef0123456789abcdef".to_owned(),
            manifest_sha256: v2.manifest_sha256,
            rollback_eligible: true,
            started_at: "2099-08-31T00:00:00.000000000Z".to_owned(),
            state: ObservationState::Pending,
            version: v2.version,
            window_closes_at: None,
        },
    );

    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("health listener");
    let listen = listener.local_addr().expect("health address").to_string();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("health request");
        let mut request = [0_u8; 512];
        let count = stream.read(&mut request).expect("read health request");
        assert!(String::from_utf8_lossy(&request[..count]).starts_with("GET /health/ready "));
        let body = br#"{"build":"2.0.0","generation":2,"ready":true}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .expect("health headers");
        stream.write_all(body).expect("health body");
    });

    let reply = fixture
        .selector
        .attest_install_health(
            "2.0.0",
            &listen,
            "2099-08-31T00:01:00.000000000Z",
            "2099-08-31T00:06:00.000000000Z",
        )
        .expect("exact health admission");
    server.join().expect("health server");
    assert_eq!(reply.operation, "attest-install-health");
    let observation = fixture
        .selector
        .status()
        .expect("status")
        .observation
        .expect("observation");
    assert_eq!(observation.state, ObservationState::Ready);
    assert!(!observation.canary_required);
    assert!(observation.canary_deadline_at.is_none());
    assert_eq!(
        observation.window_closes_at.as_deref(),
        Some("2099-08-31T00:06:00.000000000Z")
    );
}

#[test]
fn selector_update_checks_mode_and_install_identity_requirement_before_publish() {
    let fixture = Fixture::new();
    let artifact = fixture._temp.path().join("selector-candidate");
    let candidate_digest = "d".repeat(64);
    assert_ne!(
        candidate_digest,
        "e".repeat(64),
        "new selector evidence may evolve"
    );
    let candidate = format!(
        "#!/bin/sh\n[ \"$1\" = describe-conformance ] || exit 64\nprintf '%s\\n' '{{\"architecture\":\"aarch64\",\"conformance_sha256\":\"{candidate_digest}\",\"format\":1,\"operation\":\"describe-conformance\",\"version\":\"1.1.0\"}}'\n"
    );
    fs::write(&artifact, candidate.as_bytes()).expect("artifact");
    fs::set_permissions(&artifact, fs::Permissions::from_mode(0o755)).expect("artifact mode");
    let manifest_path = fixture._temp.path().join("selector-manifest.json");
    let manifest = SelectorManifest {
        architecture: "aarch64".to_owned(),
        conformance_sha256: candidate_digest,
        file: SelectorFile {
            bytes: candidate.len() as u64,
            mode: "0755".to_owned(),
            path: "selector/bin/tekes-selector".to_owned(),
            sha256: format!("{:x}", Sha256::digest(candidate.as_bytes())),
        },
        format: 1,
        minimum_os: "15.0".to_owned(),
        signing: Signing {
            requirement: REQUIREMENT.to_owned(),
            team_id: TEAM.to_owned(),
        },
        version: "1.1.0".to_owned(),
    };
    write_canonical(&manifest_path, &manifest);
    let reply = fixture
        .selector
        .update_selector(&artifact, &manifest_path)
        .expect("valid atomic selector update");
    assert_eq!(reply.sha256, manifest.file.sha256);
    let logs =
        fs::read_to_string(&fixture.selector.paths().operational_log).expect("selector update log");
    assert!(logs.lines().any(|line| {
        line.contains("\"code\":\"selector-update-complete\"")
            && line.contains(&format!("\"sha256\":\"{}\"", reply.sha256))
    }));
    let stable = fixture.selector.paths().selector.join("bin/tekes-selector");
    let stable_before = fs::read(&stable).expect("stable selector");

    fs::set_permissions(&artifact, fs::Permissions::from_mode(0o644)).expect("wrong mode");
    let error = fixture
        .selector
        .update_selector(&artifact, &manifest_path)
        .expect_err("wrong artifact mode must fail");
    assert_eq!(error.code, tekes_selector::ErrorCode::InvalidBundle);
    assert_eq!(
        fs::read(&stable).expect("stable after rejection"),
        stable_before
    );

    fs::set_permissions(&artifact, fs::Permissions::from_mode(0o755)).expect("restore mode");
    let mut wrong_evidence = manifest.clone();
    wrong_evidence.conformance_sha256 = "e".repeat(64);
    write_canonical(&manifest_path, &wrong_evidence);
    let error = fixture
        .selector
        .update_selector(&artifact, &manifest_path)
        .expect_err("manifest must bind the candidate's own embedded evidence");
    assert_eq!(error.code, tekes_selector::ErrorCode::InvalidBundle);
    assert_eq!(
        fs::read(&stable).expect("stable after evidence mismatch"),
        stable_before
    );

    let mut wrong_requirement = manifest;
    wrong_requirement.signing.requirement = "wrong requirement".to_owned();
    write_canonical(&manifest_path, &wrong_requirement);
    assert!(
        fixture
            .selector
            .update_selector(&artifact, &manifest_path)
            .is_err()
    );
    assert_eq!(
        fs::read(&stable).expect("stable remains complete"),
        stable_before
    );
}

#[test]
fn stage_revalidates_supervisor_app_profile_and_entitlements_before_publication() {
    let fixture = Fixture::new();
    let source = fixture.make_bundle("profile-rejected", b'1');
    let profile = source.join("apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile");
    fs::write(&profile, b"reject-in-staging").expect("replace profile fixture");
    let manifest_path = source.join("manifest.canonical.json");
    let mut manifest: BundleManifest = read_canonical(&manifest_path);
    let row = manifest
        .files
        .iter_mut()
        .find(|row| row.path.ends_with("embedded.provisionprofile"))
        .expect("profile row");
    row.bytes = b"reject-in-staging".len() as u64;
    row.sha256 = format!("{:x}", Sha256::digest(b"reject-in-staging"));
    write_canonical(&manifest_path, &manifest);

    let error = fixture
        .selector
        .stage(
            &source,
            "profile-rejected",
            cli_command_sha256(&stage_argv(&fixture.kernel, &source, "profile-rejected"))
                .expect("command hash"),
        )
        .expect_err("staging app verification must reject the copied profile");

    assert_eq!(error.code, tekes_selector::ErrorCode::InvalidBundle);
    assert!(
        !fixture
            .selector
            .paths()
            .bundles
            .join("profile-rejected")
            .exists()
    );
}

struct Fixture {
    _temp: TempDir,
    kernel: PathBuf,
    selector: Selector<TestSignatures>,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().expect("tempdir");
        let tekes = temp.path().join("Library/Application Support/Tekes");
        let kernel = tekes.join("Kernel");
        fs::create_dir_all(&tekes).expect("Tekes root");
        let selector = Selector::new(&kernel, TestSignatures);
        let storage = selector.paths().storage_root.clone();
        fs::create_dir_all(&storage).expect("storage root");
        fs::set_permissions(&storage, fs::Permissions::from_mode(0o700)).expect("storage mode");
        fs::write(storage.join(".root-lock"), b"").expect("root lock");
        fs::set_permissions(
            storage.join(".root-lock"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("root lock mode");
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        let installer = tekes.join("Installer");
        fs::create_dir(&installer).expect("installer directory");
        fs::set_permissions(&installer, fs::Permissions::from_mode(0o700)).expect("installer mode");
        write_canonical(
            &selector.paths().install_identity,
            &InstallIdentity {
                access_group: "TEKESAPP01.com.tekes.shared.endpoint".to_owned(),
                client_requirement: "client".to_owned(),
                format: 1,
                installer_requirement: "installer".to_owned(),
                selector_requirement: REQUIREMENT.to_owned(),
                supervisor_requirement: REQUIREMENT.to_owned(),
                team_id: TEAM.to_owned(),
            },
        );
        let log_parent = selector
            .paths()
            .operational_log
            .parent()
            .expect("log parent");
        fs::create_dir_all(log_parent).expect("selector log directory");
        fs::set_permissions(log_parent, fs::Permissions::from_mode(0o700))
            .expect("selector log mode");
        Self {
            _temp: temp,
            kernel,
            selector,
        }
    }

    fn stage_upgrade_pair(&self) -> (Selection, Selection) {
        let source1 = self.make_bundle("1.0.0", b'1');
        let argv1 = stage_argv(&self.kernel, &source1, "1.0.0");
        let v1 = self
            .selector
            .stage(&source1, "1.0.0", cli_command_sha256(&argv1).expect("hash"))
            .expect("stage v1")
            .selection;
        let activate1 = vec![
            "--install-root".to_owned(),
            self.kernel.display().to_string(),
            "activate".to_owned(),
            "--version".to_owned(),
            "1.0.0".to_owned(),
        ];
        self.selector
            .activate("1.0.0", cli_command_sha256(&activate1).expect("hash"))
            .expect("activate v1");
        let source2 = self.make_bundle("2.0.0", b'2');
        let argv2 = stage_argv(&self.kernel, &source2, "2.0.0");
        let v2 = self
            .selector
            .stage(&source2, "2.0.0", cli_command_sha256(&argv2).expect("hash"))
            .expect("stage v2")
            .selection;
        (v1, v2)
    }

    fn storage_root(&self) -> PathBuf {
        self.selector.paths().storage_root.clone()
    }

    fn activate(&self, version: &str) {
        let argv = vec![
            "--install-root".to_owned(),
            self.kernel.display().to_string(),
            "activate".to_owned(),
            "--version".to_owned(),
            version.to_owned(),
        ];
        self.selector
            .activate(version, cli_command_sha256(&argv).expect("activate hash"))
            .expect("activate bundle");
    }

    fn stage_bundle(&self, version: &str, supervisor: Vec<u8>) -> Selection {
        let source = self.make_bundle_with_supervisor(version, "process", supervisor);
        let argv = stage_argv(&self.kernel, &source, version);
        self.selector
            .stage(
                &source,
                version,
                cli_command_sha256(&argv).expect("stage hash"),
            )
            .expect("stage process bundle")
            .selection
    }

    fn make_bundle(&self, version: &str, byte: u8) -> PathBuf {
        self.make_bundle_with_supervisor(version, &byte.to_string(), vec![byte + 1; 17])
    }

    fn make_bundle_with_supervisor(
        &self,
        version: &str,
        suffix: &str,
        supervisor: Vec<u8>,
    ) -> PathBuf {
        let root = self._temp.path().join(format!("source-{version}-{suffix}"));
        fs::create_dir(&root).expect("bundle root");
        fs::create_dir(root.join("bin")).expect("bundle bin");
        fs::create_dir(root.join("apps")).expect("bundle apps");
        fs::create_dir(root.join("apps/TekesKernelSupervisor.app")).expect("supervisor app");
        fs::create_dir(root.join("apps/TekesKernelSupervisor.app/Contents"))
            .expect("supervisor contents");
        fs::create_dir(root.join("apps/TekesKernelSupervisor.app/Contents/MacOS"))
            .expect("supervisor macos");
        fs::create_dir(root.join("apps/TekesKernelSupervisor.app/Contents/_CodeSignature"))
            .expect("supervisor code signature");
        fs::create_dir(root.join("apps/TekesKernelSupervisor.app/Contents/Resources"))
            .expect("supervisor resources");
        let mut files = Vec::new();
        for (path, bytes, mode) in [
            (
                "apps/TekesKernelSupervisor.app/Contents/Info.plist",
                vec![b'i'; 19],
                0o644,
            ),
            (
                "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
                supervisor,
                0o755,
            ),
            (
                "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
                vec![b'v'; 22],
                0o644,
            ),
            (
                "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
                vec![b'c'; 20],
                0o644,
            ),
            (
                "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
                vec![b'p'; 21],
                0o644,
            ),
            ("bin/tekes-helper", vec![b'h'; 16], 0o755),
            ("bin/tekes-worker", vec![b'w'; 18], 0o755),
        ] {
            fs::write(root.join(path), &bytes).expect("executable");
            fs::set_permissions(root.join(path), fs::Permissions::from_mode(mode)).expect("mode");
            files.push(BundleFile {
                bytes: bytes.len() as u64,
                mode: format!("{mode:04o}"),
                path: path.to_owned(),
                sha256: format!("{:x}", Sha256::digest(&bytes)),
            });
        }
        write_canonical(
            &root.join("manifest.canonical.json"),
            &BundleManifest {
                architectures: vec!["aarch64".to_owned()],
                compatibility: Compatibility {
                    authority_registry_sha256: REGISTRY.to_owned(),
                    reader_profile: "v1".to_owned(),
                    writer_profile: "v1".to_owned(),
                },
                files,
                format: 1,
                minimum_os: "15.0".to_owned(),
                signing: Signing {
                    requirement: REQUIREMENT.to_owned(),
                    team_id: TEAM.to_owned(),
                },
                version: version.to_owned(),
            },
        );
        root
    }
}

fn stage_argv(kernel: &Path, bundle: &Path, version: &str) -> Vec<String> {
    vec![
        "--install-root".to_owned(),
        kernel.display().to_string(),
        "stage".to_owned(),
        "--bundle".to_owned(),
        bundle.display().to_string(),
        "--version".to_owned(),
        version.to_owned(),
    ]
}

fn write_canonical(path: &Path, value: &impl Serialize) {
    let mut bytes = serde_json_canonicalizer::to_vec(value).expect("canonical JSON");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("write canonical file");
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).expect("canonical mode");
}

fn read_canonical<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let bytes = fs::read(path).expect("read canonical");
    serde_json::from_slice(bytes.strip_suffix(b"\n").expect("LF")).expect("decode canonical")
}
