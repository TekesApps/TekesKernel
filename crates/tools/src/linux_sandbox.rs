//! Linux backend of the sandbox profile contract: Landlock confines the
//! filesystem and seccomp confines network sockets, process creation and a
//! fixed set of kernel interfaces. Both halves are prepared in the parent and
//! applied in the forked child before `execve`, so the requested executable
//! and its whole child tree start confined. Failing to apply either half fails
//! the spawn; nothing falls back to unconfined execution.

use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::sandbox::{NetworkPolicy, ProbeFailure, SandboxError, SandboxPolicy};

pub(crate) const BACKEND: &str = "linux-landlock-seccomp";

const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1;
const LANDLOCK_RULE_PATH_BENEATH: libc::c_int = 1;

const ACCESS_EXECUTE: u64 = 1 << 0;
const ACCESS_WRITE_FILE: u64 = 1 << 1;
const ACCESS_READ_FILE: u64 = 1 << 2;
const ACCESS_READ_DIR: u64 = 1 << 3;
/// Rights ABI 1 defines: execute through make-symlink.
const ACCESS_ABI_1: u64 = (1 << 13) - 1;
const ACCESS_REFER: u64 = 1 << 13;
const ACCESS_TRUNCATE: u64 = 1 << 14;
/// Rights a rule on a non-directory may carry.
const ACCESS_FILE: u64 = ACCESS_EXECUTE | ACCESS_WRITE_FILE | ACCESS_READ_FILE | ACCESS_TRUNCATE;

/// Read-only runtime roots every confined program needs: binaries, shared
/// libraries, the dynamic loader cache and system configuration, and the
/// process and CPU views runtimes consult. Missing roots are skipped.
const SYSTEM_READ_ROOTS: &[&str] = &[
    "/bin",
    "/etc",
    "/lib",
    "/lib32",
    "/lib64",
    "/libx32",
    "/proc",
    "/run/systemd/resolve",
    "/sbin",
    "/sys/devices/system/cpu",
    "/sys/fs/cgroup",
    "/usr",
];

/// Character devices programs open by name; readable and writable like the
/// equivalent entries of the macOS system baseline.
const SYSTEM_DEVICES: &[&str] = &[
    "/dev/full",
    "/dev/null",
    "/dev/random",
    "/dev/tty",
    "/dev/urandom",
    "/dev/zero",
];

#[repr(C)]
struct RulesetAttr {
    handled_access_fs: u64,
}

#[repr(C, packed)]
struct PathBeneathAttr {
    allowed_access: u64,
    parent_fd: i32,
}

/// The Landlock ABI the running kernel implements, or why Landlock or
/// seccomp is unusable.
pub(crate) fn supported_abi() -> Result<u32, (ProbeFailure, String)> {
    // SAFETY: the version query takes no attribute pointer and returns either
    // a non-negative ABI number or -1 with errno set.
    let abi = unsafe {
        libc::syscall(
            libc::SYS_landlock_create_ruleset,
            std::ptr::null::<RulesetAttr>(),
            0usize,
            LANDLOCK_CREATE_RULESET_VERSION,
        )
    };
    if abi < 1 {
        let error = io::Error::last_os_error();
        return Err((
            ProbeFailure::Missing,
            format!("Landlock is not available in this kernel: {error}"),
        ));
    }
    // SAFETY: PR_GET_SECCOMP takes no pointers; -1 means seccomp is not built.
    if unsafe { libc::prctl(libc::PR_GET_SECCOMP, 0, 0, 0, 0) } < 0 {
        let error = io::Error::last_os_error();
        return Err((
            ProbeFailure::Missing,
            format!("seccomp is not available in this kernel: {error}"),
        ));
    }
    u32::try_from(abi).map_err(|_| (ProbeFailure::Rejected, "invalid Landlock ABI".to_owned()))
}

/// Configures `command`, which must run `executable`, to start under `policy`.
pub(crate) fn confine(
    command: &mut Command,
    policy: &SandboxPolicy,
    executable: &Path,
) -> Result<(), SandboxError> {
    policy.validate()?;
    if policy.network == NetworkPolicy::Loopback {
        return Err(SandboxError::Unavailable(
            "loopback-only network cannot be enforced by the Linux backend".to_owned(),
        ));
    }
    let abi = supported_abi().map_err(|(_, detail)| SandboxError::Unavailable(detail))?;
    let ruleset = landlock_ruleset(policy, executable, abi)?;
    let filter = seccomp_filter(policy)?;
    // SAFETY: the closure runs in the forked child before exec. It performs
    // only raw syscalls on memory the parent prepared (the ruleset descriptor
    // and the filter instructions it owns), allocates nothing and takes no
    // locks, so it is async-signal-safe.
    unsafe {
        command.pre_exec(move || {
            if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::syscall(libc::SYS_landlock_restrict_self, ruleset.as_raw_fd(), 0u32) != 0 {
                return Err(io::Error::last_os_error());
            }
            let program = libc::sock_fprog {
                len: filter.len() as libc::c_ushort,
                filter: filter.as_ptr().cast_mut(),
            };
            if libc::syscall(
                libc::SYS_seccomp,
                libc::SECCOMP_SET_MODE_FILTER,
                0u32,
                &program as *const libc::sock_fprog,
            ) != 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
    Ok(())
}

fn landlock_ruleset(
    policy: &SandboxPolicy,
    executable: &Path,
    abi: u32,
) -> Result<OwnedFd, SandboxError> {
    let mut handled = ACCESS_ABI_1;
    if abi >= 2 {
        handled |= ACCESS_REFER;
    }
    if abi >= 3 {
        handled |= ACCESS_TRUNCATE;
    }
    let attr = RulesetAttr {
        handled_access_fs: handled,
    };
    // SAFETY: `attr` is a live, correctly sized ruleset attribute; ABI 1
    // kernels accept the leading `handled_access_fs` field alone.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_landlock_create_ruleset,
            &attr as *const RulesetAttr,
            std::mem::size_of::<RulesetAttr>(),
            0u32,
        )
    };
    if fd < 0 {
        return Err(SandboxError::Unavailable(format!(
            "Landlock ruleset creation failed: {}",
            io::Error::last_os_error()
        )));
    }
    // SAFETY: the kernel returned a new descriptor that nothing else owns.
    let ruleset = unsafe { OwnedFd::from_raw_fd(fd as libc::c_int) };

    let execute = if policy.allow_process {
        ACCESS_EXECUTE
    } else {
        0
    };
    let read = ACCESS_READ_FILE | ACCESS_READ_DIR | execute;
    for root in SYSTEM_READ_ROOTS {
        add_rule(&ruleset, Path::new(root), read & handled, false)?;
    }
    // Shell redirections open devices with O_TRUNC, which ABI 3 mediates.
    let device = (ACCESS_READ_FILE | ACCESS_WRITE_FILE | ACCESS_TRUNCATE) & handled;
    for path in SYSTEM_DEVICES {
        add_rule(&ruleset, Path::new(path), device, false)?;
    }
    for root in &policy.read_roots {
        add_rule(&ruleset, Path::new(root), read & handled, false)?;
    }
    for root in policy.write_roots.iter().chain(policy.scratch.iter()) {
        let rights = if policy.allow_process {
            handled
        } else {
            handled & !ACCESS_EXECUTE
        };
        add_rule(&ruleset, Path::new(root), rights, false)?;
    }
    // The requested program and its ELF interpreter must be executable even
    // when the policy grants no other execution.
    add_rule(
        &ruleset,
        executable,
        ACCESS_READ_FILE | ACCESS_EXECUTE,
        true,
    )?;
    if let Some(interpreter) = elf_interpreter(executable)? {
        add_rule(
            &ruleset,
            &interpreter,
            ACCESS_READ_FILE | ACCESS_EXECUTE,
            true,
        )?;
    }
    Ok(ruleset)
}

/// Grants `rights` beneath `path`. A missing optional path grants nothing; a
/// missing `required` path fails, because exec would fail later anyway.
fn add_rule(
    ruleset: &OwnedFd,
    path: &Path,
    rights: u64,
    required: bool,
) -> Result<(), SandboxError> {
    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| SandboxError::Invalid("sandbox path contains NUL"))?;
    // SAFETY: `c_path` is a valid NUL-terminated path; O_PATH opens no data.
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
    if fd < 0 {
        let error = io::Error::last_os_error();
        if !required
            && matches!(
                error.raw_os_error(),
                Some(libc::ENOENT) | Some(libc::ENOTDIR)
            )
        {
            return Ok(());
        }
        return Err(SandboxError::Unavailable(format!(
            "cannot open sandbox path {}: {error}",
            path.display()
        )));
    }
    // SAFETY: `open` returned a new descriptor that nothing else owns.
    let parent = unsafe { OwnedFd::from_raw_fd(fd) };
    let is_dir = File::from(parent.try_clone()?)
        .metadata()
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false);
    let allowed = if is_dir { rights } else { rights & ACCESS_FILE };
    if allowed == 0 {
        return Ok(());
    }
    let attr = PathBeneathAttr {
        allowed_access: allowed,
        parent_fd: parent.as_raw_fd(),
    };
    // SAFETY: `attr` is a live packed path-beneath attribute whose descriptor
    // stays open for the duration of the call.
    let status = unsafe {
        libc::syscall(
            libc::SYS_landlock_add_rule,
            ruleset.as_raw_fd(),
            LANDLOCK_RULE_PATH_BENEATH,
            &attr as *const PathBeneathAttr,
            0u32,
        )
    };
    if status != 0 {
        return Err(SandboxError::Unavailable(format!(
            "Landlock rule for {} rejected: {}",
            path.display(),
            io::Error::last_os_error()
        )));
    }
    Ok(())
}

/// The `PT_INTERP` path of a 64-bit little-endian ELF executable, if any.
fn elf_interpreter(executable: &Path) -> Result<Option<PathBuf>, SandboxError> {
    let mut header = Vec::new();
    File::open(executable)?
        .take(64 * 1024)
        .read_to_end(&mut header)?;
    if header.len() < 64 || &header[..4] != b"\x7fELF" || header[4] != 2 || header[5] != 1 {
        return Ok(None);
    }
    let u16_at = |at: usize| {
        header
            .get(at..at + 2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
    };
    let u32_at = |at: usize| {
        header
            .get(at..at + 4)
            .map(|b| u32::from_le_bytes(b.try_into().expect("four bytes")))
    };
    let u64_at = |at: usize| {
        header
            .get(at..at + 8)
            .map(|b| u64::from_le_bytes(b.try_into().expect("eight bytes")))
    };
    let (Some(phoff), Some(phentsize), Some(phnum)) = (u64_at(0x20), u16_at(0x36), u16_at(0x38))
    else {
        return Ok(None);
    };
    for index in 0..usize::from(phnum) {
        let entry = usize::try_from(phoff)
            .unwrap_or(usize::MAX)
            .saturating_add(index.saturating_mul(usize::from(phentsize)));
        const PT_INTERP: u32 = 3;
        if u32_at(entry) != Some(PT_INTERP) {
            continue;
        }
        let (Some(offset), Some(size)) = (u64_at(entry + 8), u64_at(entry + 32)) else {
            return Ok(None);
        };
        let start = usize::try_from(offset).unwrap_or(usize::MAX);
        let end = start.saturating_add(usize::try_from(size).unwrap_or(usize::MAX));
        let Some(bytes) = header.get(start..end) else {
            return Ok(None);
        };
        let bytes = bytes.strip_suffix(b"\0").unwrap_or(bytes);
        return Ok(Some(PathBuf::from(std::ffi::OsStr::from_bytes(bytes))));
    }
    Ok(None)
}

#[cfg(target_arch = "x86_64")]
const AUDIT_ARCH: u32 = 0xc000_003e;
#[cfg(target_arch = "aarch64")]
const AUDIT_ARCH: u32 = 0xc000_00b7;

const SECCOMP_RET_KILL_PROCESS: u32 = 0x8000_0000;
const SECCOMP_RET_ERRNO: u32 = 0x0005_0000;
const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;

const BPF_LD_W_ABS: u16 = 0x20;
const BPF_JMP_JEQ_K: u16 = 0x15;
const BPF_JMP_JGE_K: u16 = 0x35;
const BPF_JMP_JSET_K: u16 = 0x45;
const BPF_RET_K: u16 = 0x06;

const DATA_NR: u32 = 0;
const DATA_ARCH: u32 = 4;
/// Low 32 bits of syscall argument `index` on a little-endian target.
const fn data_arg(index: u32) -> u32 {
    16 + 8 * index
}

const CLONE_THREAD: u32 = 0x0001_0000;
const CLONE_NAMESPACES: u32 = 0x0000_0080 // CLONE_NEWTIME
    | 0x0002_0000 // CLONE_NEWNS
    | 0x0200_0000 // CLONE_NEWCGROUP
    | 0x0400_0000 // CLONE_NEWUTS
    | 0x0800_0000 // CLONE_NEWIPC
    | 0x1000_0000 // CLONE_NEWUSER
    | 0x2000_0000 // CLONE_NEWPID
    | 0x4000_0000; // CLONE_NEWNET
const TIOCSTI: u32 = 0x5412;
const TIOCLINUX: u32 = 0x541c;

/// Syscalls no confined program may make: kernel-extension, mount and
/// namespace interfaces, the kernel keyring, and io_uring, whose operations
/// would bypass the socket rules below.
const DENIED: &[(libc::c_long, i32)] = &[
    (libc::SYS_add_key, libc::EPERM),
    (libc::SYS_bpf, libc::EPERM),
    (libc::SYS_delete_module, libc::EPERM),
    (libc::SYS_finit_module, libc::EPERM),
    (libc::SYS_fsconfig, libc::EPERM),
    (libc::SYS_fsmount, libc::EPERM),
    (libc::SYS_fsopen, libc::EPERM),
    (libc::SYS_init_module, libc::EPERM),
    (libc::SYS_io_uring_enter, libc::ENOSYS),
    (libc::SYS_io_uring_register, libc::ENOSYS),
    (libc::SYS_io_uring_setup, libc::ENOSYS),
    (libc::SYS_kexec_file_load, libc::EPERM),
    (libc::SYS_kexec_load, libc::EPERM),
    (libc::SYS_keyctl, libc::EPERM),
    (libc::SYS_mount, libc::EPERM),
    (libc::SYS_move_mount, libc::EPERM),
    (libc::SYS_open_tree, libc::EPERM),
    (libc::SYS_perf_event_open, libc::EPERM),
    (libc::SYS_pivot_root, libc::EPERM),
    (libc::SYS_reboot, libc::EPERM),
    (libc::SYS_request_key, libc::EPERM),
    (libc::SYS_setns, libc::EPERM),
    (libc::SYS_swapoff, libc::EPERM),
    (libc::SYS_swapon, libc::EPERM),
    (libc::SYS_umount2, libc::EPERM),
    (libc::SYS_unshare, libc::EPERM),
    (libc::SYS_userfaultfd, libc::EPERM),
    // clone3 passes its flags in memory a filter cannot inspect; ENOSYS makes
    // libc fall back to clone, whose flags are checked below.
    (libc::SYS_clone3, libc::ENOSYS),
];

#[derive(Default)]
struct Program(Vec<libc::sock_filter>);

impl Program {
    fn push(&mut self, code: u16, jt: u8, jf: u8, k: u32) {
        self.0.push(libc::sock_filter { code, jt, jf, k });
    }

    fn load(&mut self, offset: u32) {
        self.push(BPF_LD_W_ABS, 0, 0, offset);
    }

    fn ret(&mut self, action: u32) {
        self.push(BPF_RET_K, 0, 0, action);
    }

    fn deny(&mut self, nr: libc::c_long, errno: i32) {
        self.load(DATA_NR);
        self.push(BPF_JMP_JEQ_K, 0, 1, nr as u32);
        self.ret(SECCOMP_RET_ERRNO | errno as u32);
    }

    /// Denies `nr` when argument `arg` has any bit of `mask` set.
    fn deny_if_any(&mut self, nr: libc::c_long, arg: u32, mask: u32, errno: i32) {
        self.load(DATA_NR);
        self.push(BPF_JMP_JEQ_K, 0, 3, nr as u32);
        self.load(data_arg(arg));
        self.push(BPF_JMP_JSET_K, 0, 1, mask);
        self.ret(SECCOMP_RET_ERRNO | errno as u32);
    }

    /// Denies `nr` when argument `arg` has no bit of `mask` set.
    fn deny_unless_any(&mut self, nr: libc::c_long, arg: u32, mask: u32, errno: i32) {
        self.load(DATA_NR);
        self.push(BPF_JMP_JEQ_K, 0, 3, nr as u32);
        self.load(data_arg(arg));
        self.push(BPF_JMP_JSET_K, 1, 0, mask);
        self.ret(SECCOMP_RET_ERRNO | errno as u32);
    }

    /// Denies `nr` when argument `arg` equals `value`.
    fn deny_if_equal(&mut self, nr: libc::c_long, arg: u32, value: u32, errno: i32) {
        self.load(DATA_NR);
        self.push(BPF_JMP_JEQ_K, 0, 3, nr as u32);
        self.load(data_arg(arg));
        self.push(BPF_JMP_JEQ_K, 0, 1, value);
        self.ret(SECCOMP_RET_ERRNO | errno as u32);
    }
}

fn seccomp_filter(policy: &SandboxPolicy) -> Result<Box<[libc::sock_filter]>, SandboxError> {
    let mut program = Program::default();
    // A foreign syscall ABI would bypass every number-based rule.
    program.load(DATA_ARCH);
    program.push(BPF_JMP_JEQ_K, 1, 0, AUDIT_ARCH);
    program.ret(SECCOMP_RET_KILL_PROCESS);
    #[cfg(target_arch = "x86_64")]
    {
        const X32_SYSCALL_BIT: u32 = 0x4000_0000;
        program.load(DATA_NR);
        program.push(BPF_JMP_JGE_K, 0, 1, X32_SYSCALL_BIT);
        program.ret(SECCOMP_RET_ERRNO | libc::ENOSYS as u32);
    }
    for &(nr, errno) in DENIED {
        program.deny(nr, errno);
    }
    program.deny_if_any(libc::SYS_clone, 0, CLONE_NAMESPACES, libc::EPERM);
    program.deny_if_equal(libc::SYS_ioctl, 1, TIOCSTI, libc::EPERM);
    program.deny_if_equal(libc::SYS_ioctl, 1, TIOCLINUX, libc::EPERM);
    if policy.network == NetworkPolicy::Deny {
        // socketpair stays available: it creates no endpoint outside the
        // process tree.
        program.deny(libc::SYS_socket, libc::EACCES);
    }
    if !policy.allow_process {
        program.deny_unless_any(libc::SYS_clone, 0, CLONE_THREAD, libc::EPERM);
        #[cfg(target_arch = "x86_64")]
        {
            program.deny(libc::SYS_fork, libc::EPERM);
            program.deny(libc::SYS_vfork, libc::EPERM);
        }
    }
    program.ret(SECCOMP_RET_ALLOW);
    if program.0.len() > usize::from(u16::MAX) {
        return Err(SandboxError::Encoding("seccomp filter too long".to_owned()));
    }
    Ok(program.0.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Output;

    use crate::sandbox::{
        NetworkPolicy, ProbeStatus, SandboxBackend, SandboxPolicy, probe_backend, sandbox_command,
    };

    struct Fixture {
        _root: tempfile::TempDir,
        inside: PathBuf,
        outside: PathBuf,
    }

    fn fixture() -> Fixture {
        let root = tempfile::tempdir().unwrap();
        let base = fs::canonicalize(root.path()).unwrap();
        let inside = base.join("inside");
        let outside = base.join("outside");
        fs::create_dir(&inside).unwrap();
        fs::create_dir(&outside).unwrap();
        fs::write(inside.join("visible.txt"), b"visible").unwrap();
        fs::write(outside.join("secret.txt"), b"secret").unwrap();
        Fixture {
            _root: root,
            inside,
            outside,
        }
    }

    fn policy(fixture: &Fixture, network: NetworkPolicy, allow_process: bool) -> SandboxPolicy {
        let inside = fixture.inside.to_string_lossy().into_owned();
        SandboxPolicy {
            format: 1,
            read_roots: vec![inside.clone()],
            write_roots: vec![inside],
            network,
            allow_process,
            scratch: None,
        }
    }

    fn run(policy: &SandboxPolicy, executable: &str, args: &[&str]) -> Output {
        let probe = probe_backend(SandboxBackend::LinuxLandlockSeccompV1);
        assert!(matches!(probe, ProbeStatus::Available { .. }), "{probe:?}");
        sandbox_command(policy, &probe, Path::new(executable))
            .unwrap()
            .args(args)
            .output()
            .unwrap()
    }

    fn sh(policy: &SandboxPolicy, script: &str) -> Output {
        run(policy, "/bin/sh", &["-c", script])
    }

    fn stderr(output: &Output) -> String {
        String::from_utf8_lossy(&output.stderr).into_owned()
    }

    #[test]
    fn filesystem_access_is_limited_to_policy_roots() {
        let fixture = fixture();
        let policy = policy(&fixture, NetworkPolicy::Deny, true);
        let inside = fixture.inside.display();
        let outside = fixture.outside.display();

        let read = sh(&policy, &format!("cat {inside}/visible.txt"));
        assert!(read.status.success(), "{}", stderr(&read));
        assert_eq!(read.stdout, b"visible");

        let write = sh(
            &policy,
            &format!("echo new > {inside}/new.txt && mv {inside}/new.txt {inside}/moved.txt"),
        );
        assert!(write.status.success(), "{}", stderr(&write));
        assert_eq!(
            fs::read(fixture.inside.join("moved.txt")).unwrap(),
            b"new\n"
        );

        // Redirection opens the device with O_TRUNC.
        let devnull = sh(&policy, "echo discarded > /dev/null");
        assert!(devnull.status.success(), "{}", stderr(&devnull));

        // Child processes inherit the confinement.
        let secret = sh(&policy, &format!("cat {outside}/secret.txt"));
        assert!(!secret.status.success());
        assert!(secret.stdout.is_empty());

        let escape = sh(&policy, &format!("touch {outside}/created.txt"));
        assert!(!escape.status.success());
        assert!(!fixture.outside.join("created.txt").exists());
    }

    #[test]
    fn read_roots_are_not_writable() {
        let fixture = fixture();
        let mut policy = policy(&fixture, NetworkPolicy::Deny, true);
        policy.write_roots.clear();
        let output = sh(
            &policy,
            &format!("touch {}/new.txt", fixture.inside.display()),
        );
        assert!(!output.status.success());
        assert!(!fixture.inside.join("new.txt").exists());
    }

    #[test]
    fn denied_network_refuses_sockets_and_all_permits_them() {
        let fixture = fixture();
        let connect = "exec 3<>/dev/tcp/127.0.0.1/9";
        let denied = run(
            &policy(&fixture, NetworkPolicy::Deny, true),
            "/bin/bash",
            &["-c", connect],
        );
        assert!(!denied.status.success());
        assert!(
            stderr(&denied).contains("Permission denied"),
            "{}",
            stderr(&denied)
        );

        let allowed = run(
            &policy(&fixture, NetworkPolicy::All, true),
            "/bin/bash",
            &["-c", connect],
        );
        assert!(
            !stderr(&allowed).contains("Permission denied"),
            "{}",
            stderr(&allowed)
        );
    }

    #[test]
    fn loopback_only_network_is_refused() {
        let fixture = fixture();
        let probe = probe_backend(SandboxBackend::LinuxLandlockSeccompV1);
        assert!(
            sandbox_command(
                &policy(&fixture, NetworkPolicy::Loopback, true),
                &probe,
                Path::new("/bin/true"),
            )
            .is_err()
        );
    }

    #[test]
    fn without_process_permission_only_the_requested_program_runs() {
        let fixture = fixture();
        let policy = policy(&fixture, NetworkPolicy::Deny, false);
        let builtin = sh(&policy, "echo ok");
        assert!(builtin.status.success(), "{}", stderr(&builtin));
        assert_eq!(builtin.stdout, b"ok\n");

        let forked = sh(&policy, "/bin/true");
        assert!(!forked.status.success());
        let replaced = sh(&policy, "exec /bin/true");
        assert!(!replaced.status.success());
    }

    #[test]
    fn namespaces_and_outside_processes_are_out_of_reach() {
        let fixture = fixture();
        let policy = policy(&fixture, NetworkPolicy::Deny, true);
        let unshare = run(&policy, "/usr/bin/unshare", &["--user", "/bin/true"]);
        assert!(!unshare.status.success());

        let own = sh(&policy, "cat /proc/self/environ > /dev/null");
        assert!(own.status.success(), "{}", stderr(&own));
        let parent = sh(
            &policy,
            &format!("cat /proc/{}/environ", std::process::id()),
        );
        assert!(!parent.status.success());
        assert!(parent.stdout.is_empty());
    }
}
