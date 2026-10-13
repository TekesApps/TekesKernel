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
use std::process::{Command, Output};
use std::sync::OnceLock;
use std::time::Duration;

use tools::{
    BackendFailure, HelperJobLauncher, JobSpec, NetworkPolicy, ProbeFailure, ProbeStatus,
    SandboxBackend, SandboxPolicy, SandboxedJobLauncher, probe_backend,
};

struct Fixture {
    _root: tempfile::TempDir,
    inside: PathBuf,
    outside: PathBuf,
    scratch: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
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
    let probe = PROBE.get_or_init(|| probe_backend(SandboxBackend::LinuxLandlockSeccompV1));
    assert!(matches!(probe, ProbeStatus::Available { .. }), "{probe:?}");
    probe.clone()
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
fn linux_sandbox_refuses_metadata_writes_outside_landlock_rights() {
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
fn linux_no_process_permission_allows_threads_but_refuses_children() {
    let fixture = Fixture::new();
    let mut policy = fixture.policy();
    policy.allow_process = false;
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
    assert_eq!(
        command.spawn().unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
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
            assert_denied(fs::read(outside.join("secret.txt")));
            assert_denied(fs::read_dir(&outside));
            assert_denied(fs::write(outside.join("created.txt"), b"escape"));
            assert_denied(fs::read(inside.join("escape/secret.txt")));
            assert_denied(fs::write(inside.join("escape/created.txt"), b"escape"));
            let path = CString::new(outside.join("secret.txt").as_os_str().as_bytes()).unwrap();
            // SAFETY: the C string is valid for the entire call; length 0
            // deliberately tests the pathname truncate right introduced in ABI 3.
            assert_eq!(unsafe { libc::truncate(path.as_ptr(), 0) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().kind(),
                std::io::ErrorKind::PermissionDenied
            );
        }
        "scratch" => {
            assert_eq!(fs::read(inside.join("read.txt")).unwrap(), b"readable");
            assert_denied(fs::write(inside.join("read.txt"), b"overwrite"));
            assert_denied(fs::write(inside.join("created.txt"), b"new"));
            let scratch = PathBuf::from(std::env::var_os("TEKES_SANDBOX_TEST_SCRATCH").unwrap());
            fs::write(scratch.join("temporary.txt"), b"temporary").unwrap();
            assert_eq!(
                fs::read(scratch.join("temporary.txt")).unwrap(),
                b"temporary"
            );
        }
        "metadata" => {
            let outside = outside.join("secret.txt");
            assert_denied(fs::set_permissions(
                &outside,
                fs::Permissions::from_mode(0o777),
            ));
            // Metadata mutation is intentionally refused even on writable
            // roots because Landlock cannot selectively mediate these calls.
            assert_denied(fs::set_permissions(
                inside.join("read.txt"),
                fs::Permissions::from_mode(0o777),
            ));
            let path = CString::new(outside.as_os_str().as_bytes()).unwrap();
            // SAFETY: path is NUL-terminated; the uid/gid queries take no pointers.
            assert_eq!(
                unsafe { libc::chown(path.as_ptr(), libc::geteuid(), libc::getegid()) },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().kind(),
                std::io::ErrorKind::PermissionDenied
            );
            // SAFETY: a null times argument requests the current time; path is live.
            assert_eq!(
                unsafe { libc::utimensat(libc::AT_FDCWD, path.as_ptr(), std::ptr::null(), 0) },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().kind(),
                std::io::ErrorKind::PermissionDenied
            );
            let attribute = c"user.tekes-sandbox-test";
            let value = b"escape";
            // SAFETY: the path, attribute name, and value storage are live and sized.
            assert_eq!(
                unsafe {
                    libc::setxattr(
                        path.as_ptr(),
                        attribute.as_ptr(),
                        value.as_ptr().cast(),
                        value.len(),
                        0,
                    )
                },
                -1
            );
            assert_eq!(
                std::io::Error::last_os_error().kind(),
                std::io::ErrorKind::PermissionDenied
            );
        }
        "network-denied" => {
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
            socket.send(b"local IPC").unwrap();
            let mut local = [0; 16];
            assert_eq!(peer.recv(&mut local).unwrap(), b"local IPC".len());
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
