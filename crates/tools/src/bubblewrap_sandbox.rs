//! Linux filesystem isolation for kernels without Landlock. Bubblewrap builds
//! a private mount tree from explicit roots and applies the seccomp program
//! after namespace/mount setup, immediately before running the target.
//!
//! The command line deliberately uses only options present in bubblewrap
//! 0.4.0. Nothing retries a failed launch without isolation.

use std::collections::BTreeMap;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::linux_sandbox::{SYSTEM_READ_ROOTS, elf_interpreter, seccomp_filter};
use crate::sandbox::{NetworkPolicy, SandboxError, SandboxPolicy};

pub(crate) const BACKEND: &str = "linux-bubblewrap-seccomp";

/// Do not search PATH: an executable in a workspace is not a sandbox launcher.
/// Check the resolved file and its ancestors so an unprivileged user cannot
/// replace either the binary or one of its parent directories.
pub(crate) fn binary() -> Result<PathBuf, SandboxError> {
    for candidate in ["/usr/bin/bwrap", "/bin/bwrap"] {
        let path = match fs::canonicalize(candidate) {
            Ok(path) => path,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        };
        let metadata = fs::metadata(&path)?;
        if !metadata.is_file() || metadata.mode() & 0o111 == 0 {
            continue;
        }
        for ancestor in path.ancestors() {
            let metadata = fs::metadata(ancestor)?;
            if metadata.uid() != 0 || metadata.mode() & 0o022 != 0 {
                return Err(SandboxError::Unavailable(format!(
                    "bubblewrap path is not root-owned and protected from replacement: {}",
                    ancestor.display()
                )));
            }
        }
        return Ok(path);
    }
    Err(SandboxError::Unavailable(
        "bubblewrap is not installed at /usr/bin/bwrap or /bin/bwrap".to_owned(),
    ))
}

/// The caller can append target arguments and set `current_dir` as usual.
/// Bubblewrap 0.4.0 obtains the inherited cwd after exec and restores that path
/// in its new root if it was mounted. Callers that require a particular cwd
/// must choose one within the policy roots.
pub(crate) fn command(policy: &SandboxPolicy, executable: &Path) -> Result<Command, SandboxError> {
    command_with_lifecycle(policy, executable, true)
}

/// Durable jobs retain their existing lifetime independently of the client
/// that launched them. Their broker owns cancellation and process identity.
pub(crate) fn detached_command(
    policy: &SandboxPolicy,
    executable: &Path,
) -> Result<Command, SandboxError> {
    command_with_lifecycle(policy, executable, false)
}

fn command_with_lifecycle(
    policy: &SandboxPolicy,
    executable: &Path,
    die_with_parent: bool,
) -> Result<Command, SandboxError> {
    policy.validate()?;
    if policy.network == NetworkPolicy::Loopback {
        return Err(SandboxError::Unavailable(
            "loopback-only network cannot be enforced by the bubblewrap backend".to_owned(),
        ));
    }
    if !policy.allow_process {
        return Err(SandboxError::Unavailable(
            "bubblewrap cannot enforce allow_process=false for executable replacement".to_owned(),
        ));
    }
    if !executable.is_absolute() {
        return Err(SandboxError::Invalid(
            "sandbox executable must be an absolute path",
        ));
    }
    let plan = mount_plan(policy, executable)?;
    let filter = sealed_filter(policy)?;
    let descriptor_path = CString::new(format!("/proc/self/fd/{}", filter.as_raw_fd()))
        .expect("formatted descriptor path contains no NUL");
    let mut command = Command::new(binary()?);
    command.args([
        "--unshare-user",
        "--unshare-pid",
        "--unshare-ipc",
        "--unshare-uts",
        "--cap-drop",
        "ALL",
    ]);
    if die_with_parent {
        command.arg("--die-with-parent");
    }
    if policy.network == NetworkPolicy::Deny {
        command.arg("--unshare-net");
    }
    for mount in plan.mounts {
        command
            .arg(if mount.writable {
                "--bind"
            } else {
                "--ro-bind"
            })
            .arg(mount.source)
            .arg(mount.destination);
    }
    for alias in plan.aliases {
        command
            .arg("--symlink")
            .arg(alias.source)
            .arg(alias.destination);
    }
    // Never expose the host's process tree or device tree, even when an
    // explicitly declared read root contains them. These mounts come last.
    command.args(["--proc", "/proc", "--dev", "/dev", "--remount-ro", "/"]);
    command
        .arg("--seccomp")
        .arg(filter.as_raw_fd().to_string())
        .arg("--")
        .arg(executable);
    // SAFETY: the fork child only makes raw open/dup2/close calls against a
    // descriptor and path prepared by the parent. It allocates and locks
    // nothing. Reopening creates an independent offset for each spawn, so
    // overlapping spawns using one Command cannot consume each other's BPF.
    // dup2 clears CLOEXEC only in this child. Applying the filter here would
    // break bubblewrap's own namespace setup.
    unsafe {
        command.pre_exec(move || {
            let fd = filter.as_raw_fd();
            let reader = libc::open(descriptor_path.as_ptr(), libc::O_RDONLY | libc::O_CLOEXEC);
            if reader < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::dup2(reader, fd) < 0 {
                let error = io::Error::last_os_error();
                libc::close(reader);
                return Err(error);
            }
            libc::close(reader);
            Ok(())
        });
    }
    Ok(command)
}

#[derive(Debug)]
struct Mount {
    source: PathBuf,
    destination: PathBuf,
    writable: bool,
}

#[derive(Debug)]
struct MountPlan {
    mounts: Vec<Mount>,
    aliases: Vec<Mount>,
}

fn mount_plan(policy: &SandboxPolicy, executable: &Path) -> Result<MountPlan, SandboxError> {
    let mut writable = Vec::new();
    for root in policy.write_roots.iter().chain(policy.scratch.iter()) {
        // Check both names: a /dev alias must not smuggle host devices into
        // the sandbox, and a writable ancestor must not replace private mounts.
        reject_protected_write(Path::new(root))?;
        if let Some(canonical) = canonical_optional(Path::new(root))? {
            reject_protected_write(&canonical)?;
            // An exception beneath a protected root must be declared by its
            // canonical name: a symlinked alias would otherwise turn a session
            // path into a writable bind inside Kernel storage.
            if policy.is_protected_exception(&canonical) && canonical != Path::new(root) {
                return Err(SandboxError::Invalid(
                    "write roots beneath a protected root must be canonical paths",
                ));
            }
            writable.push(canonical);
        }
    }
    let writable = Writable {
        roots: writable,
        policy,
    };
    let mut mounts = BTreeMap::new();
    let mut aliases = BTreeMap::new();
    for root in SYSTEM_READ_ROOTS {
        add_mount(&mut mounts, &mut aliases, Path::new(root), &writable, false)?;
    }
    for root in policy
        .read_roots
        .iter()
        .chain(&policy.write_roots)
        .chain(policy.scratch.iter())
    {
        add_mount(&mut mounts, &mut aliases, Path::new(root), &writable, false)?;
    }
    // A protected root inside a writable bind becomes its own read-only bind;
    // deeper mounts sort later, so a declared exception beneath it stays
    // writable. Bind mounts follow the directory, so renaming an ancestor
    // cannot move the protected tree out of its read-only view.
    for root in &policy.protected_roots {
        if writable
            .roots
            .iter()
            .any(|write| Path::new(root).starts_with(write))
        {
            if canonical_optional(Path::new(root))?.is_none() {
                return Err(SandboxError::Unavailable(format!(
                    "protected root {root} must exist before exposing its writable ancestor"
                )));
            }
            add_mount(&mut mounts, &mut aliases, Path::new(root), &writable, false)?;
        }
    }
    add_mount(&mut mounts, &mut aliases, executable, &writable, true)?;
    if let Some(interpreter) = elf_interpreter(executable)? {
        add_mount(&mut mounts, &mut aliases, &interpreter, &writable, true)?;
    }
    // Bubblewrap 0.4 builds /newroot before its final pivot. An absolute
    // symlink inside a previously bound directory therefore does not resolve
    // relative to the final sandbox yet. Never pass such a path as a later
    // mount or symlink destination. Its original symlink will work after the
    // pivot because its canonical target is mounted separately.
    let mut needed_aliases = Vec::new();
    for (destination, alias) in &aliases {
        let covered = destination
            .ancestors()
            .skip(1)
            .any(|ancestor| mounts.contains_key(ancestor) || aliases.contains_key(ancestor));
        if !covered {
            needed_aliases.push(Mount {
                source: alias.source.clone(),
                destination: alias.destination.clone(),
                writable: false,
            });
        }
    }
    let mut mounts = mounts.into_values().collect::<Vec<_>>();
    // A writable child overrides a readonly parent. Read roots nested inside
    // a writable root inherit write permission, matching the union of grants
    // in the Landlock backend rather than revoking a previously granted right.
    mounts.sort_by(|a, b| {
        a.destination
            .components()
            .count()
            .cmp(&b.destination.components().count())
            .then_with(|| a.destination.cmp(&b.destination))
    });
    Ok(MountPlan {
        mounts,
        aliases: needed_aliases,
    })
}

struct Writable<'a> {
    roots: Vec<PathBuf>,
    policy: &'a SandboxPolicy,
}

impl Writable<'_> {
    /// Writable when some write root covers `source` and no protected root
    /// at or above `source` overrides it, unless that write root is itself
    /// a declared exception strictly beneath the protected root.
    fn covers(&self, source: &Path) -> bool {
        self.roots.iter().any(|root| {
            source.starts_with(root)
                && self.policy.protected_roots.iter().all(|protected| {
                    let protected = Path::new(protected);
                    !source.starts_with(protected)
                        || (root != protected && root.starts_with(protected))
                })
        })
    }
}

fn canonical_optional(path: &Path) -> Result<Option<PathBuf>, SandboxError> {
    match fs::canonicalize(path) {
        Ok(path) => Ok(Some(path)),
        Err(error)
            if matches!(
                error.raw_os_error(),
                Some(libc::ENOENT) | Some(libc::ENOTDIR)
            ) =>
        {
            Ok(None)
        }
        Err(error) => Err(error.into()),
    }
}

fn private_tree(path: &Path) -> bool {
    path.starts_with("/proc") || path.starts_with("/dev")
}

fn reject_protected_write(path: &Path) -> Result<(), SandboxError> {
    if private_tree(path)
        || Path::new("/proc").starts_with(path)
        || Path::new("/dev").starts_with(path)
    {
        return Err(SandboxError::Invalid(
            "bubblewrap write roots must not overlap the private /proc or /dev trees",
        ));
    }
    Ok(())
}

fn add_mount(
    mounts: &mut BTreeMap<PathBuf, Mount>,
    aliases: &mut BTreeMap<PathBuf, Mount>,
    path: &Path,
    writable: &Writable<'_>,
    required: bool,
) -> Result<(), SandboxError> {
    if path == Path::new("/") {
        return Err(SandboxError::Invalid(
            "bubblewrap roots must name specific paths rather than the host filesystem root",
        ));
    }
    if private_tree(path) {
        if required {
            return Err(SandboxError::Invalid(
                "sandbox executable must not be inside /proc or /dev",
            ));
        }
        return Ok(());
    }
    let Some(source) = canonical_optional(path)? else {
        if required {
            return Err(SandboxError::Unavailable(format!(
                "sandbox executable does not exist: {}",
                path.display()
            )));
        }
        return Ok(());
    };
    if private_tree(&source) {
        return Err(SandboxError::Invalid(
            "bubblewrap roots must not alias the host /proc or /dev trees",
        ));
    }
    if source == Path::new("/") {
        return Err(SandboxError::Invalid(
            "bubblewrap roots must not alias the host filesystem root",
        ));
    }
    let write = writable.covers(&source);
    // Mount only canonical destinations. Synthetic symlinks preserve aliases
    // without creating two independent mount views whose child write grants
    // could disagree. Existing symlinks within a parent bind remain intact.
    let mut candidates = vec![(mounts, source.clone())];
    if path != source {
        candidates.push((aliases, path.to_path_buf()));
    }
    for (entries, destination) in candidates {
        match entries.get_mut(&destination) {
            Some(existing) => {
                if existing.source != source {
                    return Err(SandboxError::Invalid(
                        "sandbox roots resolve to conflicting mount destinations",
                    ));
                }
                existing.writable |= write;
            }
            None => {
                entries.insert(
                    destination.clone(),
                    Mount {
                        source: source.clone(),
                        destination,
                        writable: write,
                    },
                );
            }
        }
    }
    Ok(())
}

fn sealed_filter(policy: &SandboxPolicy) -> Result<File, SandboxError> {
    let filter = seccomp_filter(policy, false)?;
    // SAFETY: the name is a valid static C string and memfd_create returns a
    // fresh descriptor. Seals make the filter immutable before it is inherited.
    let fd = unsafe {
        libc::memfd_create(
            c"tekes-sandbox-seccomp".as_ptr(),
            libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    // SAFETY: fd was just returned by memfd_create and has no other owner.
    let mut file = unsafe { File::from_raw_fd(fd) };
    for instruction in &filter {
        file.write_all(&instruction.code.to_ne_bytes())?;
        file.write_all(&[instruction.jt, instruction.jf])?;
        file.write_all(&instruction.k.to_ne_bytes())?;
    }
    file.seek(SeekFrom::Start(0))?;
    // SAFETY: fcntl operates on the live memfd with documented seal flags.
    if unsafe {
        libc::fcntl(
            file.as_raw_fd(),
            libc::F_ADD_SEALS,
            libc::F_SEAL_SEAL | libc::F_SEAL_SHRINK | libc::F_SEAL_GROW | libc::F_SEAL_WRITE,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    fn policy(read_roots: Vec<String>, write_roots: Vec<String>) -> SandboxPolicy {
        SandboxPolicy {
            format: 1,
            read_roots,
            write_roots,
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
            protected_roots: Vec::new(),
        }
    }

    #[test]
    fn absent_protected_root_under_writable_home_is_not_silently_skipped() {
        let root = tempfile::tempdir().unwrap();
        let home = fs::canonicalize(root.path()).unwrap();
        let text = home.display().to_string();
        let mut policy = policy(vec![text.clone()], vec![text]);
        policy.protected_roots = vec![home.join("missing/.agents").display().to_string()];
        assert!(matches!(
            mount_plan(&policy, Path::new("/bin/true")),
            Err(SandboxError::Unavailable(message)) if message.contains("must exist")
        ));
        assert!(!home.join("missing").exists());
    }

    #[test]
    fn protected_root_inside_a_writable_root_is_bound_read_only() {
        let root = tempfile::tempdir().unwrap();
        let home = fs::canonicalize(root.path()).unwrap();
        let storage = home.join("storage");
        let job = storage.join("jobs").join("j1");
        fs::create_dir_all(&job).unwrap();
        let text = |path: &Path| path.display().to_string();
        let mut writes = vec![text(&home), text(&job)];
        writes.sort();
        let mut policy = policy(vec![text(&home)], writes);
        policy.protected_roots = vec![text(&storage)];
        let mounts = mount_plan(&policy, Path::new("/bin/true")).unwrap().mounts;
        let find = |path: &Path| {
            mounts
                .iter()
                .position(|mount| mount.destination == path)
                .unwrap_or_else(|| panic!("no mount for {}: {mounts:?}", path.display()))
        };
        let (home_at, storage_at, job_at) = (find(&home), find(&storage), find(&job));
        assert!(mounts[home_at].writable);
        assert!(!mounts[storage_at].writable);
        assert!(mounts[job_at].writable);
        assert!(home_at < storage_at && storage_at < job_at);
    }

    #[test]
    fn symlinked_exception_beneath_a_protected_root_is_rejected() {
        let root = tempfile::tempdir().unwrap();
        let home = fs::canonicalize(root.path()).unwrap();
        let storage = home.join("storage");
        fs::create_dir_all(storage.join("threads")).unwrap();
        symlink(storage.join("threads"), home.join("alias")).unwrap();
        let mut policy = policy(
            vec![home.display().to_string()],
            vec![home.join("alias").display().to_string()],
        );
        policy.protected_roots = vec![storage.display().to_string()];
        assert!(mount_plan(&policy, Path::new("/bin/true")).is_err());
    }

    #[test]
    fn read_subroot_does_not_revoke_ancestor_write_grant() {
        let root = tempfile::tempdir().unwrap();
        let parent = fs::canonicalize(root.path()).unwrap();
        let child = parent.join("child");
        fs::create_dir(&child).unwrap();
        let mut reads = vec![parent.display().to_string(), child.display().to_string()];
        reads.sort();
        let policy = policy(reads, vec![parent.display().to_string()]);
        let mounts = mount_plan(&policy, Path::new("/bin/true")).unwrap().mounts;
        assert!(
            mounts
                .iter()
                .find(|mount| mount.destination == child)
                .unwrap()
                .writable
        );
    }

    #[test]
    fn read_parent_keeps_readonly_siblings_of_write_subroot() {
        let root = tempfile::tempdir().unwrap();
        let parent = fs::canonicalize(root.path()).unwrap();
        let child = parent.join("child");
        fs::create_dir(&child).unwrap();
        let policy = policy(
            vec![parent.display().to_string()],
            vec![child.display().to_string()],
        );
        let mounts = mount_plan(&policy, Path::new("/bin/true")).unwrap().mounts;
        let parent_mount = mounts
            .iter()
            .position(|mount| mount.destination == parent)
            .unwrap();
        let child_mount = mounts
            .iter()
            .position(|mount| mount.destination == child)
            .unwrap();
        assert!(!mounts[parent_mount].writable);
        assert!(mounts[child_mount].writable);
        assert!(parent_mount < child_mount);
    }

    #[test]
    fn root_alias_grants_the_same_canonical_object() {
        let root = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(root.path()).unwrap();
        let actual = root.join("actual");
        let alias = root.join("alias");
        fs::create_dir(&actual).unwrap();
        symlink(&actual, &alias).unwrap();
        let policy = policy(
            vec![alias.display().to_string()],
            vec![alias.display().to_string()],
        );
        let plan = mount_plan(&policy, Path::new("/bin/true")).unwrap();
        let mount = plan
            .mounts
            .iter()
            .find(|mount| mount.destination == actual)
            .unwrap();
        assert_eq!(mount.source, actual);
        assert!(mount.writable);
        let alias = plan
            .aliases
            .iter()
            .find(|mount| mount.destination == alias)
            .unwrap();
        assert_eq!(alias.source, actual);
    }

    #[test]
    fn symlink_inside_bound_parent_is_never_used_as_setup_destination() {
        let root = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(root.path()).unwrap();
        let parent = root.join("parent");
        let target = root.join("target");
        fs::create_dir(&parent).unwrap();
        fs::create_dir(&target).unwrap();
        let alias = parent.join("alias");
        symlink(&target, &alias).unwrap();
        // This path only becomes dangerous if setup follows it against
        // bubblewrap's temporary root containing the host at /oldroot.
        let malicious = parent.join("malicious");
        symlink("/oldroot/tmp/tekes-not-created", &malicious).unwrap();
        let mut reads = vec![parent.display().to_string(), alias.display().to_string()];
        reads.sort();
        let policy = policy(reads, vec![alias.display().to_string()]);
        let plan = mount_plan(&policy, Path::new("/bin/true")).unwrap();
        assert!(plan.mounts.iter().any(|mount| {
            mount.destination == target && mount.source == target && mount.writable
        }));
        assert!(
            plan.mounts
                .iter()
                .chain(&plan.aliases)
                .all(|mount| mount.destination != alias && mount.destination != malicious)
        );
        assert_eq!(
            fs::read_link(malicious).unwrap(),
            Path::new("/oldroot/tmp/tekes-not-created")
        );
    }

    #[test]
    fn whole_host_read_root_is_rejected_even_through_an_alias() {
        let root_policy = policy(vec!["/".to_owned()], Vec::new());
        assert!(mount_plan(&root_policy, Path::new("/bin/true")).is_err());
        let root = tempfile::tempdir().unwrap();
        let alias = root.path().join("host");
        symlink("/", &alias).unwrap();
        let policy = policy(vec![alias.display().to_string()], Vec::new());
        assert!(mount_plan(&policy, Path::new("/bin/true")).is_err());
    }

    #[test]
    fn cannot_replace_private_kernel_views_with_writable_host_trees() {
        for path in ["/", "/proc", "/proc/sys", "/dev", "/dev/shm"] {
            assert!(reject_protected_write(Path::new(path)).is_err(), "{path}");
        }
        for path in ["/tmp", "/home", "/proc-logs", "/development"] {
            assert!(reject_protected_write(Path::new(path)).is_ok(), "{path}");
        }
    }

    #[test]
    fn seccomp_descriptor_is_sealed_and_cloexec_in_parent() {
        let policy = policy(Vec::new(), Vec::new());
        let mut file = sealed_filter(&policy).unwrap();
        assert!(file.write_all(&[0; 8]).is_err());
        // SAFETY: the descriptor is owned by the live file and F_GETFD takes
        // no pointer arguments or mutable state.
        let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) };
        assert!(flags >= 0 && flags & libc::FD_CLOEXEC != 0);
    }
}
