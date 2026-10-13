//! Exercise the public job launcher against real Linux kernel enforcement.
//! The subprocess is this test executable so no Python or shell fixture is
//! needed, and denied syscalls cannot accidentally confine the test runner.

#![cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]

use std::collections::BTreeMap;
use std::ffi::CString;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::os::unix::net::UnixDatagram;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use tools::{
    BackendFailure, BackendGate, BackendOutcome, CancellationToken, ExecRequest, HelperClient,
    HelperClientError, HelperErrorClass, HelperJobLauncher, HelperOperation, HelperPath,
    HelperRequest, HelperResponse, HelperValue, JobBroker, JobLaunchPolicy, JobSpec, JobState,
    NetworkPolicy, ProbeFailure, ProbeStatus, SandboxBackend, SandboxPolicy, SandboxedJobLauncher,
    probe_backend,
};

struct Fixture {
    _root: tempfile::TempDir,
    inside: PathBuf,
    outside: PathBuf,
    scratch: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("tekes sandbox ")
            .tempdir()
            .unwrap();
        let base = fs::canonicalize(root.path()).unwrap();
        let inside = base.join("inside");
        let outside = base.join("outside");
        let scratch = base.join("scratch");
        for directory in [&inside, &outside, &scratch] {
            fs::create_dir(directory).unwrap();
        }
        fs::write(inside.join("read.txt"), b"readable").unwrap();
        fs::write(outside.join("secret.txt"), b"private bytes").unwrap();
        symlink(&outside, inside.join("escape")).unwrap();
        Self {
            _root: root,
            inside,
            outside,
            scratch,
        }
    }

    fn policy(&self) -> SandboxPolicy {
        SandboxPolicy {
            format: 1,
            read_roots: vec![self.inside.to_str().unwrap().to_owned()],
            write_roots: vec![self.inside.to_str().unwrap().to_owned()],
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
        }
    }

    fn command(&self, scenario: &str, policy: &SandboxPolicy) -> Command {
        let executable = std::env::current_exe().unwrap();
        let launcher = HelperJobLauncher::new(&executable, available_probe()).unwrap();
        let mut command = launcher.runner_command(&job_spec(), policy).unwrap();
        command
            .args([
                "--exact",
                "confined_probe_process",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env_clear()
            .env("TEKES_SANDBOX_TEST_EFFECTIVE_BACKEND", effective_backend())
            .env(
                "TEKES_SANDBOX_TEST_HOST_PID",
                std::process::id().to_string(),
            )
            .env(
                "TEKES_SANDBOX_TEST_HOST_PID_NAMESPACE",
                fs::read_link("/proc/self/ns/pid").unwrap(),
            )
            .env("TEKES_SANDBOX_TEST_SCENARIO", scenario)
            .env("TEKES_SANDBOX_TEST_INSIDE", &self.inside)
            .env("TEKES_SANDBOX_TEST_OUTSIDE", &self.outside)
            .env("TEKES_SANDBOX_TEST_SCRATCH", &self.scratch)
            .current_dir(&self.inside);
        command
    }
}

fn available_probe() -> ProbeStatus {
    static PROBE: OnceLock<ProbeStatus> = OnceLock::new();
    let probe = PROBE.get_or_init(|| {
        let selection =
            std::env::var("TEKES_SANDBOX_TEST_BACKEND").unwrap_or_else(|_| "auto".to_owned());
        let (backend, expected) = match selection.as_str() {
            "auto" => (SandboxBackend::LinuxAutoV1, None),
            "bubblewrap" => (
                SandboxBackend::LinuxBubblewrapSeccompV1,
                Some("linux-bubblewrap-seccomp"),
            ),
            "landlock" => (
                SandboxBackend::LinuxLandlockSeccompV1,
                Some("linux-landlock-seccomp"),
            ),
            other => panic!("unknown TEKES_SANDBOX_TEST_BACKEND: {other}"),
        };
        let probe = probe_backend(backend);
        if let (Some(expected), ProbeStatus::Available { backend, .. }) = (expected, &probe) {
            assert_eq!(backend, expected, "explicit backend must never fall back");
        }
        eprintln!("Linux sandbox runtime selection={selection}, probe={probe:?}");
        probe
    });
    assert!(matches!(probe, ProbeStatus::Available { .. }), "{probe:?}");
    probe.clone()
}

fn effective_backend() -> String {
    match available_probe() {
        ProbeStatus::Available { backend, .. } => backend,
        other => panic!("sandbox unavailable: {other:?}"),
    }
}

fn child_uses_bubblewrap() -> bool {
    std::env::var("TEKES_SANDBOX_TEST_EFFECTIVE_BACKEND").unwrap() == "linux-bubblewrap-seccomp"
}

fn job_spec() -> JobSpec {
    JobSpec {
        program: "/bin/true".to_owned(),
        args: Vec::new(),
        working_directory: None,
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    }
}

fn assert_success(output: Output) {
    assert!(
        output.status.success(),
        "confined probe failed: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn assert_denied<T: std::fmt::Debug>(result: std::io::Result<T>) {
    let error = result.expect_err("operation unexpectedly escaped the sandbox");
    assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
}

fn assert_filesystem_denied<T: std::fmt::Debug>(result: std::io::Result<T>) {
    let error = result.expect_err("filesystem operation unexpectedly escaped the sandbox");
    if child_uses_bubblewrap() {
        // Undeclared paths are absent in the mount namespace; visible read
        // roots reject mutations with EROFS. Other errors must fail the test.
        assert!(
            matches!(
                error.raw_os_error(),
                Some(libc::EACCES | libc::EPERM | libc::ENOENT | libc::EROFS)
            ),
            "unexpected filesystem denial: {error}"
        );
    } else {
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    }
}

fn assert_filesystem_syscall_denied(result: libc::c_int) {
    assert_eq!(result, -1, "filesystem syscall unexpectedly succeeded");
    assert_filesystem_denied::<()>(Err(std::io::Error::last_os_error()));
}

#[test]
fn linux_job_launcher_enforces_roots_symlinks_and_truncation() {
    let fixture = Fixture::new();
    assert_success(
        fixture
            .command("filesystem", &fixture.policy())
            .output()
            .unwrap(),
    );
    assert_eq!(
        fs::read(fixture.inside.join("written.txt")).unwrap(),
        b"new"
    );
    assert_eq!(
        fs::read(fixture.outside.join("secret.txt")).unwrap(),
        b"private bytes"
    );
    assert!(!fixture.outside.join("created.txt").exists());
}

#[test]
fn linux_job_launcher_keeps_read_roots_read_only_and_grants_scratch() {
    let fixture = Fixture::new();
    let mut policy = fixture.policy();
    policy.write_roots.clear();
    policy.scratch = Some(fixture.scratch.to_str().unwrap().to_owned());
    assert_success(fixture.command("scratch", &policy).output().unwrap());
    assert_eq!(
        fs::read(fixture.inside.join("read.txt")).unwrap(),
        b"readable"
    );
    assert_eq!(
        fs::read(fixture.scratch.join("temporary.txt")).unwrap(),
        b"temporary"
    );
}

#[test]
fn linux_sandbox_refuses_metadata_writes_outside_declared_roots() {
    let fixture = Fixture::new();
    let path = fixture.outside.join("secret.txt");
    let before = fs::metadata(&path).unwrap();
    assert_success(
        fixture
            .command("metadata", &fixture.policy())
            .output()
            .unwrap(),
    );
    let after = fs::metadata(path).unwrap();
    assert_eq!(before.permissions().mode(), after.permissions().mode());
    assert_eq!(before.modified().unwrap(), after.modified().unwrap());
}

#[test]
fn linux_network_deny_blocks_tcp_udp_and_unix_sockets() {
    let fixture = Fixture::new();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    assert_success(
        fixture
            .command("network-denied", &fixture.policy())
            .env(
                "TEKES_SANDBOX_TEST_ADDRESS",
                listener.local_addr().unwrap().to_string(),
            )
            .output()
            .unwrap(),
    );
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert!(!fixture.inside.join("socket").exists());
}

#[test]
fn linux_reused_command_keeps_concurrent_children_confined() {
    let fixture = Fixture::new();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut command = fixture.command("network-denied", &fixture.policy());
    command
        .env(
            "TEKES_SANDBOX_TEST_ADDRESS",
            listener.local_addr().unwrap().to_string(),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Both children begin before either is reaped. The bubblewrap filter fd
    // must have a separate read offset for each spawn of this same Command.
    let first = command.spawn().unwrap();
    let second = command.spawn().unwrap();
    assert_success(first.wait_with_output().unwrap());
    assert_success(second.wait_with_output().unwrap());
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert!(!fixture.inside.join("socket").exists());
}

#[test]
fn linux_network_all_reaches_a_real_tcp_listener() {
    let fixture = Fixture::new();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut policy = fixture.policy();
    policy.network = NetworkPolicy::All;
    assert_success(
        fixture
            .command("network-allowed", &policy)
            .env(
                "TEKES_SANDBOX_TEST_ADDRESS",
                listener.local_addr().unwrap().to_string(),
            )
            .output()
            .unwrap(),
    );
    let (mut connection, _) = listener.accept().unwrap();
    connection
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut received = Vec::new();
    connection.read_to_end(&mut received).unwrap();
    assert_eq!(received, b"network allowed");
}

#[test]
fn linux_network_deny_prevents_socketpair_endpoint_escape() {
    let fixture = Fixture::new();
    let endpoint = fixture.outside.join("socket");
    let listener = UnixDatagram::bind(&endpoint).unwrap();
    listener.set_nonblocking(true).unwrap();
    // Prove the host permits the attempted socketpair escape without our
    // filter; a platform-level restriction must not produce a false pass.
    let (unconfined, _peer) = UnixDatagram::pair().unwrap();
    unconfined.send_to(b"baseline", &endpoint).unwrap();
    let mut received = [0u8; 64];
    assert_eq!(listener.recv(&mut received).unwrap(), b"baseline".len());

    assert_success(
        fixture
            .command("socketpair-escape", &fixture.policy())
            .output()
            .unwrap(),
    );
    assert_eq!(
        listener.recv(&mut received).unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn linux_sandbox_is_inherited_by_children_and_grandchildren() {
    let fixture = Fixture::new();
    assert_success(
        fixture
            .command("descendants", &fixture.policy())
            .output()
            .unwrap(),
    );
    assert_eq!(
        fs::read(fixture.inside.join("child.txt")).unwrap(),
        b"child\n"
    );
    assert!(!fixture.outside.join("child.txt").exists());
    assert_eq!(
        fs::read(fixture.outside.join("secret.txt")).unwrap(),
        b"private bytes"
    );
}

#[test]
fn linux_no_process_permission_is_enforced_or_refused() {
    let fixture = Fixture::new();
    let mut policy = fixture.policy();
    policy.allow_process = false;
    if effective_backend() == "linux-bubblewrap-seccomp" {
        let launcher =
            HelperJobLauncher::new(std::env::current_exe().unwrap(), available_probe()).unwrap();
        assert!(matches!(
            launcher.runner_command(&job_spec(), &policy),
            Err(BackendFailure::Unavailable(_))
        ));
        return;
    }
    assert_success(fixture.command("threads-only", &policy).output().unwrap());
}

#[test]
fn linux_launcher_refuses_unavailable_mismatched_and_loopback_policies() {
    let fixture = Fixture::new();
    let executable = std::env::current_exe().unwrap();
    for class in [
        ProbeFailure::Missing,
        ProbeFailure::Rejected,
        ProbeFailure::Unsupported,
    ] {
        assert!(matches!(
            HelperJobLauncher::new(
                &executable,
                ProbeStatus::Unavailable {
                    class,
                    detail: "backend could not be applied".to_owned(),
                }
            ),
            Err(BackendFailure::Unavailable(_))
        ));
    }
    let mismatched = HelperJobLauncher::new(
        &executable,
        ProbeStatus::Available {
            backend: "darwin-seatbelt".to_owned(),
            version: "test".to_owned(),
        },
    )
    .unwrap();
    assert!(
        mismatched
            .runner_command(&job_spec(), &fixture.policy())
            .is_err()
    );

    let launcher = HelperJobLauncher::new(executable, available_probe()).unwrap();
    let mut policy = fixture.policy();
    policy.network = NetworkPolicy::Loopback;
    assert!(launcher.runner_command(&job_spec(), &policy).is_err());
    policy.network = NetworkPolicy::Deny;
    policy.format = 2;
    assert!(launcher.runner_command(&job_spec(), &policy).is_err());
}

#[test]
fn linux_launcher_does_not_fall_back_after_executable_disappears() {
    let fixture = Fixture::new();
    let executable = fixture.inside.join("removed-program");
    fs::copy("/bin/true", &executable).unwrap();
    let launcher = HelperJobLauncher::new(&executable, available_probe()).unwrap();
    let mut command = launcher
        .runner_command(&job_spec(), &fixture.policy())
        .unwrap();
    fs::remove_file(&executable).unwrap();
    match command.output() {
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
        Ok(output) => {
            assert_eq!(effective_backend(), "linux-bubblewrap-seccomp");
            assert!(
                !output.status.success(),
                "missing executable must fail closed"
            );
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("removed-program"), "{stderr}");
        }
    }
}

#[test]
fn linux_nested_write_and_scratch_mounts_preserve_union_permissions() {
    let fixture = Fixture::new();
    let writable = fixture.inside.join("writable tree");
    let overlapping_read = writable.join("also readable");
    let scratch = fixture.inside.join("scratch tree");
    fs::create_dir_all(&overlapping_read).unwrap();
    fs::create_dir(&scratch).unwrap();
    let mut policy = fixture.policy();
    policy
        .read_roots
        .push(overlapping_read.to_string_lossy().into_owned());
    policy.read_roots.sort();
    policy.write_roots = vec![writable.to_string_lossy().into_owned()];
    policy.scratch = Some(scratch.to_string_lossy().into_owned());
    assert_success(
        fixture
            .command("nested-roots", &policy)
            .env("TEKES_SANDBOX_TEST_SCRATCH", &scratch)
            .output()
            .unwrap(),
    );
    assert_eq!(
        fs::read(fixture.inside.join("read.txt")).unwrap(),
        b"readable"
    );
    assert_eq!(
        fs::read(overlapping_read.join("created.txt")).unwrap(),
        b"nested write"
    );
    assert_eq!(
        fs::read(scratch.join("created.txt")).unwrap(),
        b"scratch write"
    );
}

#[test]
fn linux_seccomp_refuses_mount_namespaces_and_io_uring() {
    let fixture = Fixture::new();
    assert_success(
        fixture
            .command("kernel-interfaces", &fixture.policy())
            .output()
            .unwrap(),
    );
}

#[test]
fn linux_real_helper_exec_uses_cwd_and_managed_scratch_until_last_client_drops() {
    let fixture = Fixture::new();
    let cwd = fixture.inside.join("working directory");
    fs::create_dir(&cwd).unwrap();
    let executable = fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).unwrap();
    let client = HelperClient::sandboxed_with_scratch(
        &executable,
        vec![("workspace".to_owned(), fixture.inside.clone())],
        fixture.policy(),
        available_probe(),
    )
    .unwrap();
    let scratch = PathBuf::from(client.scratch_path().unwrap());
    assert_eq!(
        fs::metadata(&scratch).unwrap().permissions().mode() & 0o777,
        0o700
    );
    let request = HelperRequest {
        id: "real-helper-exec".to_owned(),
        operation: HelperOperation::Exec(ExecRequest {
            argv: vec![
                "/bin/sh".to_owned(),
                "-c".to_owned(),
                "set -eu; printf 'helper cwd' > relative.txt; printf 'helper scratch' > \"$1/temporary.txt\"; if cat \"$2/secret.txt\"; then exit 73; fi; pwd".to_owned(),
                "sandbox-helper".to_owned(),
                scratch.to_string_lossy().into_owned(),
                fixture.outside.to_string_lossy().into_owned(),
            ],
            stdin: None,
            cwd: Some(HelperPath { root: "workspace".to_owned(), path: "working directory".to_owned() }),
            env: BTreeMap::new(),
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            timeout_ms: Some(5_000),
        }),
    };
    let survivor = client.clone();
    drop(client);
    assert!(
        scratch.is_dir(),
        "client clones must retain scratch ownership"
    );
    let response = survivor.execute(&request).unwrap();
    match response {
        HelperResponse::Result {
            value: HelperValue::Exec(value),
            ..
        } => {
            assert_eq!(
                value.status,
                0,
                "helper stderr: {:?}",
                value.stderr.decode()
            );
            assert_eq!(
                value.stdout.decode().unwrap(),
                format!("{}\n", cwd.display()).as_bytes()
            );
        }
        other => panic!("helper exec failed: {other:?}"),
    }
    assert_eq!(fs::read(cwd.join("relative.txt")).unwrap(), b"helper cwd");
    assert_eq!(
        fs::read(scratch.join("temporary.txt")).unwrap(),
        b"helper scratch"
    );
    drop(survivor);
    assert!(
        !scratch.exists(),
        "the last client must remove private scratch"
    );
    assert_eq!(
        fs::read(fixture.outside.join("secret.txt")).unwrap(),
        b"private bytes"
    );
}

#[test]
fn linux_real_job_runner_reports_completion_and_confines_target() {
    let fixture = Fixture::new();
    let executable = fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).unwrap();
    let launcher = Arc::new(HelperJobLauncher::new(executable, available_probe()).unwrap());
    let broker_root = fixture._root.path().join("job records");
    let broker = JobBroker::new(&broker_root, 4096).unwrap();
    let mut policy = fixture.policy();
    policy.write_roots.clear();
    let launch_policy = JobLaunchPolicy::new(
        fixture.inside.to_string_lossy().into_owned(),
        policy,
        vec![fixture.inside.to_string_lossy().into_owned()],
    )
    .unwrap();
    let spec = JobSpec {
        program: "/bin/sh".to_owned(),
        args: vec![
            "-c".to_owned(),
            "set -eu; printf 'job cwd' > job-output.txt; if cat \"$1/secret.txt\"; then exit 74; fi; if printf escape > \"$1/job-escape.txt\"; then exit 75; fi; printf 'job complete'".to_owned(),
            "sandbox-job".to_owned(),
            fixture.outside.to_string_lossy().into_owned(),
        ],
        working_directory: None,
        writable_paths: vec![fixture.inside.to_string_lossy().into_owned()],
        environment: BTreeMap::new(),
    };
    let started = broker.start(
        "confined-job",
        spec,
        &launch_policy,
        launcher,
        BackendGate::Ready,
        &CancellationToken::default(),
    );
    assert!(
        matches!(started, BackendOutcome::Completed(Ok(_))),
        "{started:?}"
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match broker.status("confined-job") {
            BackendOutcome::Completed(Ok(record)) if record.state == JobState::Running => {
                assert!(
                    Instant::now() < deadline,
                    "confined job did not finish: {record:?}"
                );
                std::thread::sleep(Duration::from_millis(20));
            }
            BackendOutcome::Completed(Ok(record)) => {
                assert_eq!(record.state, JobState::Exited { status: 0 }, "{record:?}");
                break;
            }
            other => panic!("confined job failed: {other:?}"),
        }
    }
    assert_eq!(
        fs::read(fixture.inside.join("job-output.txt")).unwrap(),
        b"job cwd"
    );
    assert_eq!(
        fs::read(broker_root.join("confined-job/stdout.tail")).unwrap(),
        b"job complete"
    );
    assert_eq!(
        fs::read(fixture.outside.join("secret.txt")).unwrap(),
        b"private bytes"
    );
    assert!(!fixture.outside.join("job-escape.txt").exists());
}

// Lifecycle enforcement below specifically exercises bubblewrap's private PID
// namespace. Landlock provides no equivalent process-tree ownership, so its
// filesystem/seccomp suite must not report these guarantees as tested.
fn require_bubblewrap_lifecycle() {
    assert_eq!(effective_backend(), "linux-bubblewrap-seccomp");
}

const DELAYED_DESCENDANT_SCRIPT: &str = concat!(
    "/bin/sh -c '",
    "trap \"\" TERM; printf ready > \"$1/ready\"; count=0; ",
    "while [ ! -f \"$1/release\" ]; do count=$((count+1)); ",
    "[ \"$count\" -lt 100 ] || exit 81; sleep 0.05; done; ",
    "sleep 0.2; printf escaped > \"$1/escaped\"",
    "' descendant \"$1\" </dev/null >/dev/null 2>&1 & wait",
);

fn descendant_control(fixture: &Fixture) -> PathBuf {
    // Demonstrate the delayed write succeeds without termination. Otherwise
    // a broken shell fixture could make every teardown test falsely pass.
    let baseline = fixture.inside.join("baseline descendant");
    fs::create_dir(&baseline).unwrap();
    fs::write(baseline.join("release"), b"release").unwrap();
    assert_success(
        Command::new("/bin/sh")
            .args(["-c", DELAYED_DESCENDANT_SCRIPT, "baseline"])
            .arg(&baseline)
            .output()
            .unwrap(),
    );
    assert_eq!(fs::read(baseline.join("ready")).unwrap(), b"ready");
    assert_eq!(fs::read(baseline.join("escaped")).unwrap(), b"escaped");

    let control = fixture.inside.join("confined descendant");
    fs::create_dir(&control).unwrap();
    control
}

fn wait_for_file(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(4);
    while !path.is_file() {
        assert!(
            Instant::now() < deadline,
            "subprocess did not create {}",
            path.display()
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn assert_descendant_was_terminated(control: &Path) {
    assert_eq!(fs::read(control.join("ready")).unwrap(), b"ready");
    fs::write(control.join("release"), b"release").unwrap();
    let deadline = Instant::now() + Duration::from_secs(1);
    while Instant::now() < deadline {
        assert!(
            !control.join("escaped").exists(),
            "a terminated sandbox left a live descendant"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn lifecycle_client(fixture: &Fixture) -> HelperClient {
    HelperClient::sandboxed(
        fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).unwrap(),
        vec![("workspace".to_owned(), fixture.inside.clone())],
        fixture.policy(),
        available_probe(),
    )
    .unwrap()
}

fn descendant_request(control: &Path, timeout_ms: Option<u64>) -> HelperRequest {
    HelperRequest {
        id: "descendant-lifecycle".to_owned(),
        operation: HelperOperation::Exec(ExecRequest {
            argv: vec![
                "/bin/sh".to_owned(),
                "-c".to_owned(),
                DELAYED_DESCENDANT_SCRIPT.to_owned(),
                "confined".to_owned(),
                control.to_string_lossy().into_owned(),
            ],
            stdin: None,
            cwd: Some(HelperPath {
                root: "workspace".to_owned(),
                path: String::new(),
            }),
            env: BTreeMap::new(),
            stdout_bytes: 4096,
            stderr_bytes: 4096,
            timeout_ms,
        }),
    }
}

#[test]
#[ignore = "requires explicit bubblewrap lifecycle suite"]
fn bubblewrap_lifecycle_helper_cancellation_terminates_descendants() {
    require_bubblewrap_lifecycle();
    let fixture = Fixture::new();
    let control = descendant_control(&fixture);
    let client = lifecycle_client(&fixture);
    let result = client.execute_cancellable(
        &descendant_request(&control, None),
        Some(Duration::from_secs(5)),
        || control.join("ready").is_file(),
    );
    assert!(
        matches!(result, Err(HelperClientError::Cancelled)),
        "{result:?}"
    );
    assert_descendant_was_terminated(&control);
}

#[test]
#[ignore = "requires explicit bubblewrap lifecycle suite"]
fn bubblewrap_lifecycle_helper_timeout_terminates_descendants() {
    require_bubblewrap_lifecycle();
    let fixture = Fixture::new();
    let control = descendant_control(&fixture);
    let client = lifecycle_client(&fixture);
    let result = client.execute_cancellable(
        &descendant_request(&control, None),
        Some(Duration::from_secs(1)),
        || false,
    );
    assert!(
        matches!(&result, Err(HelperClientError::Helper(error)) if error.class == HelperErrorClass::Timeout),
        "{result:?}",
    );
    assert_descendant_was_terminated(&control);
}

#[test]
#[ignore = "requires explicit bubblewrap lifecycle suite"]
fn bubblewrap_lifecycle_exec_timeout_terminates_descendants() {
    require_bubblewrap_lifecycle();
    let fixture = Fixture::new();
    let control = descendant_control(&fixture);
    let client = lifecycle_client(&fixture);
    let response = client
        .execute(&descendant_request(&control, Some(500)))
        .unwrap();
    assert!(
        matches!(&response, HelperResponse::Error { error, .. } if error.class == HelperErrorClass::Timeout),
        "{response:?}",
    );
    assert_descendant_was_terminated(&control);
}

#[test]
#[ignore = "requires explicit bubblewrap lifecycle suite"]
fn bubblewrap_lifecycle_reopened_job_broker_stops_descendants() {
    require_bubblewrap_lifecycle();
    let fixture = Fixture::new();
    let control = descendant_control(&fixture);
    let broker_root = fixture._root.path().join("job records");
    let broker = JobBroker::new(&broker_root, 4096).unwrap();
    let spec = JobSpec {
        program: "/bin/sh".to_owned(),
        args: vec![
            "-c".to_owned(),
            DELAYED_DESCENDANT_SCRIPT.to_owned(),
            "job".to_owned(),
            control.to_string_lossy().into_owned(),
        ],
        working_directory: None,
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    };
    let launch_policy = JobLaunchPolicy::new(
        fixture.inside.to_string_lossy().into_owned(),
        fixture.policy(),
        Vec::new(),
    )
    .unwrap();
    let launcher = Arc::new(
        HelperJobLauncher::new(
            fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).unwrap(),
            available_probe(),
        )
        .unwrap(),
    );
    let record = match broker.start(
        "stop-tree",
        spec,
        &launch_policy,
        launcher,
        BackendGate::Ready,
        &CancellationToken::default(),
    ) {
        BackendOutcome::Completed(Ok(record)) => record,
        other => panic!("job start failed: {other:?}"),
    };
    wait_for_file(&control.join("ready"));
    drop(broker);
    let reopened = JobBroker::new(&broker_root, 4096).unwrap();
    assert!(
        matches!(reopened.status("stop-tree"), BackendOutcome::Completed(Ok(current)) if current.state == JobState::Running && current.pid == record.pid && current.process_start == record.process_start)
    );
    let stopped = reopened.stop(
        "stop-tree",
        BackendGate::Ready,
        &CancellationToken::default(),
    );
    assert!(
        matches!(&stopped, BackendOutcome::Completed(Ok(record)) if record.state == JobState::Stopped),
        "{stopped:?}"
    );
    assert_descendant_was_terminated(&control);
    assert!(
        matches!(reopened.status("stop-tree"), BackendOutcome::Completed(Ok(record)) if record.state == JobState::Stopped)
    );
    assert!(
        matches!(reopened.stop("stop-tree", BackendGate::Ready, &CancellationToken::default()), BackendOutcome::Completed(Ok(record)) if record.state == JobState::Stopped)
    );
}

#[test]
#[ignore = "requires explicit bubblewrap lifecycle suite"]
fn bubblewrap_lifecycle_detached_job_survives_launcher_exit() {
    require_bubblewrap_lifecycle();
    let fixture = Fixture::new();
    let broker_root = fixture._root.path().join("detached jobs");
    assert_success(
        Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "detached_job_launcher_process",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("TEKES_SANDBOX_TEST_DETACHED_WORKSPACE", &fixture.inside)
            .env("TEKES_SANDBOX_TEST_DETACHED_BROKER", &broker_root)
            .env("TEKES_SANDBOX_TEST_BACKEND", "bubblewrap")
            .output()
            .unwrap(),
    );
    // The launcher has now exited. Only after that boundary may the job
    // finish, proving it did not inherit foreground die-with-parent behavior.
    let broker = JobBroker::new(&broker_root, 4096).unwrap();
    let status = broker.status("survivor");
    assert!(
        matches!(&status, BackendOutcome::Completed(Ok(record)) if record.state == JobState::Running),
        "{status:?}"
    );
    assert!(fixture.inside.join("detached-ready").is_file());
    fs::write(fixture.inside.join("detached-release"), b"release").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match broker.status("survivor") {
            BackendOutcome::Completed(Ok(record)) if record.state == JobState::Running => {
                assert!(
                    Instant::now() < deadline,
                    "detached job never completed: {record:?}"
                );
                std::thread::sleep(Duration::from_millis(10));
            }
            BackendOutcome::Completed(Ok(record)) => {
                assert_eq!(record.state, JobState::Exited { status: 0 }, "{record:?}");
                break;
            }
            other => panic!("detached job failed: {other:?}"),
        }
    }
    assert_eq!(
        fs::read(fixture.inside.join("detached-done")).unwrap(),
        b"survived"
    );
    assert_eq!(
        fs::read(broker_root.join("survivor/stdout.tail")).unwrap(),
        b"survived"
    );
}

#[test]
#[ignore = "subprocess entry point for the detached bubblewrap lifecycle test"]
fn detached_job_launcher_process() {
    require_bubblewrap_lifecycle();
    let workspace =
        PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_DETACHED_WORKSPACE").unwrap());
    let broker_root =
        PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_DETACHED_BROKER").unwrap());
    let policy = SandboxPolicy {
        format: 1,
        read_roots: vec![workspace.to_string_lossy().into_owned()],
        write_roots: vec![workspace.to_string_lossy().into_owned()],
        network: NetworkPolicy::Deny,
        allow_process: true,
        scratch: None,
    };
    let launch_policy =
        JobLaunchPolicy::new(workspace.to_string_lossy().into_owned(), policy, Vec::new()).unwrap();
    let broker = JobBroker::new(broker_root, 4096).unwrap();
    let launcher = Arc::new(
        HelperJobLauncher::new(
            fs::canonicalize(env!("CARGO_BIN_EXE_tekes-helper")).unwrap(),
            available_probe(),
        )
        .unwrap(),
    );
    let spec = JobSpec {
        program: "/bin/sh".to_owned(),
        args: vec!["-c".to_owned(), "set -eu; printf ready > detached-ready; count=0; while [ ! -f detached-release ]; do count=$((count+1)); [ \"$count\" -lt 200 ] || exit 82; sleep 0.05; done; printf survived > detached-done; printf survived".to_owned()],
        working_directory: None,
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    };
    let started = broker.start(
        "survivor",
        spec,
        &launch_policy,
        launcher,
        BackendGate::Ready,
        &CancellationToken::default(),
    );
    assert!(
        matches!(started, BackendOutcome::Completed(Ok(_))),
        "{started:?}"
    );
    wait_for_file(&workspace.join("detached-ready"));
}

#[test]
#[ignore = "subprocess entry point; invoked only by Linux sandbox integration tests"]
fn confined_probe_process() {
    let inside = PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_INSIDE").unwrap());
    let outside = PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_OUTSIDE").unwrap());
    match std::env::var("TEKES_SANDBOX_TEST_SCENARIO")
        .unwrap()
        .as_str()
    {
        "filesystem" => {
            assert_eq!(fs::read(inside.join("read.txt")).unwrap(), b"readable");
            fs::write(inside.join("written.txt"), b"new").unwrap();
            assert_filesystem_denied(fs::read(outside.join("secret.txt")));
            assert_filesystem_denied(fs::read_dir(&outside));
            assert_filesystem_denied(fs::write(outside.join("created.txt"), b"escape"));
            assert_filesystem_denied(fs::read(inside.join("escape/secret.txt")));
            assert_filesystem_denied(fs::write(inside.join("escape/created.txt"), b"escape"));
            let path = CString::new(outside.join("secret.txt").as_os_str().as_bytes()).unwrap();
            // SAFETY: the C string is valid for the entire call; length 0
            // deliberately tests the pathname truncate right introduced in ABI 3.
            assert_filesystem_syscall_denied(unsafe { libc::truncate(path.as_ptr(), 0) });
        }
        "scratch" => {
            assert_eq!(fs::read(inside.join("read.txt")).unwrap(), b"readable");
            assert_filesystem_denied(fs::write(inside.join("read.txt"), b"overwrite"));
            assert_filesystem_denied(fs::write(inside.join("created.txt"), b"new"));
            let scratch = PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_SCRATCH").unwrap());
            fs::write(scratch.join("temporary.txt"), b"temporary").unwrap();
            assert_eq!(
                fs::read(scratch.join("temporary.txt")).unwrap(),
                b"temporary"
            );
        }
        "nested-roots" => {
            assert_eq!(std::env::current_dir().unwrap(), inside);
            assert_eq!(fs::read(inside.join("read.txt")).unwrap(), b"readable");
            assert_filesystem_denied(fs::write(inside.join("read.txt"), b"overwrite"));
            assert_filesystem_denied(fs::write(inside.join("created.txt"), b"escape"));
            fs::write(
                inside.join("writable tree/also readable/created.txt"),
                b"nested write",
            )
            .unwrap();
            let scratch = PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_SCRATCH").unwrap());
            fs::write(scratch.join("created.txt"), b"scratch write").unwrap();
        }
        "kernel-interfaces" => {
            if child_uses_bubblewrap() {
                let synthetic_parent = inside.parent().unwrap();
                assert!(fs::metadata(synthetic_parent).unwrap().is_dir());
                let error = fs::write(synthetic_parent.join("synthetic-root-write.txt"), b"escape")
                    .expect_err("synthetic policy ancestors must remain read-only");
                assert_eq!(error.raw_os_error(), Some(libc::EROFS));

                let status = fs::read_to_string("/proc/self/status").unwrap();
                let field = |name: &str| {
                    status
                        .lines()
                        .find_map(|line| {
                            let (key, value) = line.split_once(':')?;
                            (key == name).then(|| value.trim())
                        })
                        .unwrap_or_else(|| panic!("missing {name} in /proc/self/status"))
                };
                assert_eq!(field("NoNewPrivs"), "1");
                assert_eq!(field("Seccomp"), "2");
                assert_eq!(u64::from_str_radix(field("CapEff"), 16).unwrap(), 0);
                let host_namespace = PathBuf::from(
                    std::env::var_os("TEKES_SANDBOX_TEST_HOST_PID_NAMESPACE").unwrap(),
                );
                assert_ne!(fs::read_link("/proc/self/ns/pid").unwrap(), host_namespace);
                let host_pid = std::env::var("TEKES_SANDBOX_TEST_HOST_PID").unwrap();
                match fs::read_link(format!("/proc/{host_pid}/ns/pid")) {
                    Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
                    // A small host PID can coincide numerically with a task
                    // in the inner namespace. It must not expose the host task.
                    Ok(namespace) => assert_ne!(namespace, host_namespace),
                }
            }
            // SAFETY: PR_GET_SECCOMP has no pointer arguments.
            assert_eq!(unsafe { libc::prctl(libc::PR_GET_SECCOMP, 0, 0, 0, 0) }, 2);
            // SAFETY: unshare(0) changes no namespaces even if the filter
            // failed. A working seccomp filter refuses the syscall outright.
            assert_eq!(unsafe { libc::unshare(0) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EPERM)
            );
            // SAFETY: null paths make this harmless without the filter too.
            assert_eq!(
                unsafe {
                    libc::mount(
                        std::ptr::null(),
                        std::ptr::null(),
                        std::ptr::null(),
                        0,
                        std::ptr::null(),
                    )
                },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EPERM)
            );
            let mut parameters = [0u64; 32];
            // SAFETY: zeroed storage exceeds io_uring_params and remains live;
            // the filter must return ENOSYS before creating a ring descriptor.
            assert_eq!(
                unsafe { libc::syscall(libc::SYS_io_uring_setup, 1u32, parameters.as_mut_ptr()) },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ENOSYS)
            );
        }
        "metadata" => {
            let outside = outside.join("secret.txt");
            assert_filesystem_denied(fs::set_permissions(
                &outside,
                fs::Permissions::from_mode(0o777),
            ));
            let result =
                fs::set_permissions(inside.join("read.txt"), fs::Permissions::from_mode(0o640));
            if child_uses_bubblewrap() {
                // The mount namespace confines metadata writes to writable
                // mounts, allowing ordinary build-tool chmod operations.
                result.unwrap();
                assert_eq!(
                    fs::metadata(inside.join("read.txt"))
                        .unwrap()
                        .permissions()
                        .mode()
                        & 0o777,
                    0o640
                );
            } else {
                // Landlock does not selectively mediate metadata writes;
                // its seccomp profile deliberately refuses them everywhere.
                assert_denied(result);
            }
            let path = CString::new(outside.as_os_str().as_bytes()).unwrap();
            // SAFETY: path is NUL-terminated; the uid/gid queries take no pointers.
            assert_filesystem_syscall_denied(unsafe {
                libc::chown(path.as_ptr(), libc::geteuid(), libc::getegid())
            });
            // SAFETY: a null times argument requests the current time; path is live.
            assert_filesystem_syscall_denied(unsafe {
                libc::utimensat(libc::AT_FDCWD, path.as_ptr(), std::ptr::null(), 0)
            });
            let attribute = c"user.tekes-sandbox-test";
            let value = b"escape";
            // SAFETY: the path, attribute name, and value storage are live and sized.
            assert_filesystem_syscall_denied(unsafe {
                libc::setxattr(
                    path.as_ptr(),
                    attribute.as_ptr(),
                    value.as_ptr().cast(),
                    value.len(),
                    0,
                )
            });
        }
        "network-denied" => {
            assert_filesystem_denied(fs::read(outside.join("secret.txt")));
            let address = std::env::var("TEKES_SANDBOX_TEST_ADDRESS")
                .unwrap()
                .parse()
                .unwrap();
            assert_denied(TcpStream::connect_timeout(&address, Duration::from_secs(2)));
            assert_denied(UdpSocket::bind("127.0.0.1:0"));
            assert_denied(UnixDatagram::bind(inside.join("socket")));
        }
        "network-allowed" => {
            let address = std::env::var("TEKES_SANDBOX_TEST_ADDRESS")
                .unwrap()
                .parse()
                .unwrap();
            TcpStream::connect_timeout(&address, Duration::from_secs(2))
                .unwrap()
                .write_all(b"network allowed")
                .unwrap();
        }
        "socketpair-escape" => {
            let (socket, peer) = UnixDatagram::pair().unwrap();
            let payload = b"local IPC";
            // UnixDatagram::send uses sendto on Linux, which the network-deny
            // policy rejects. Anonymous IPC deliberately uses read/write.
            // SAFETY: the descriptor and payload stay live for this write.
            assert_eq!(
                unsafe { libc::write(socket.as_raw_fd(), payload.as_ptr().cast(), payload.len()) },
                payload.len() as isize
            );
            let mut local = [0; 16];
            assert_eq!(peer.recv(&mut local).unwrap(), payload.len());
            assert_eq!(&local[..payload.len()], payload);
            let endpoint = outside.join("socket");
            assert_denied(socket.send_to(b"escape", &endpoint));
            assert_denied(socket.connect(&endpoint));
            assert_message_send_denied(&socket, &endpoint);
        }
        "descendants" => {
            let output = Command::new("/bin/sh")
                .args(["-c", "exec /bin/sh -c 'printf \"child\\n\" > \"$1/child.txt\"; if cat \"$2/secret.txt\"; then exit 71; fi; if printf escape > \"$2/child.txt\"; then exit 72; fi' child \"$1\" \"$2\"", "child"])
                .arg(&inside).arg(&outside).output().unwrap();
            assert_success(output);
        }
        "threads-only" => {
            assert_eq!(std::thread::spawn(|| 42).join().unwrap(), 42);
            assert_denied(Command::new("/bin/true").status());
        }
        scenario => panic!("unknown confined scenario: {scenario}"),
    }
}

fn assert_message_send_denied(socket: &UnixDatagram, endpoint: &Path) {
    // SAFETY: zero is a valid initial representation for sockaddr_un.
    let mut address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    address.sun_family = libc::AF_UNIX as libc::sa_family_t;
    let path = endpoint.as_os_str().as_bytes();
    assert!(path.len() < address.sun_path.len());
    for (destination, byte) in address.sun_path.iter_mut().zip(path) {
        *destination = *byte as libc::c_char;
    }
    let mut payload = *b"escape";
    let mut vector = libc::iovec {
        iov_base: payload.as_mut_ptr().cast(),
        iov_len: payload.len(),
    };
    // SAFETY: zeroed fields mean no control messages; pointers below refer to
    // live local storage for both synchronous send syscalls.
    let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
    message.msg_name = (&mut address as *mut libc::sockaddr_un).cast();
    message.msg_namelen = std::mem::size_of_val(&address) as libc::socklen_t;
    message.msg_iov = &mut vector;
    message.msg_iovlen = 1;
    // SAFETY: the descriptor is borrowed and message/payload storage is live.
    assert_eq!(
        unsafe { libc::sendmsg(socket.as_raw_fd(), &message, 0) },
        -1
    );
    assert_eq!(
        std::io::Error::last_os_error().kind(),
        std::io::ErrorKind::PermissionDenied
    );
    let mut messages = libc::mmsghdr {
        msg_hdr: message,
        msg_len: 0,
    };
    // SAFETY: exactly one initialized message is passed with a live payload.
    assert_eq!(
        unsafe { libc::sendmmsg(socket.as_raw_fd(), &mut messages, 1, 0) },
        -1
    );
    assert_eq!(
        std::io::Error::last_os_error().kind(),
        std::io::ErrorKind::PermissionDenied
    );
}
