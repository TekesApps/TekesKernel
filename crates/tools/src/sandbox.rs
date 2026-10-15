use std::collections::BTreeSet;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use thiserror::Error;

// Earlier ABIs cannot deny truncate(2), including on files outside write roots.
pub(crate) const LINUX_LANDLOCK_ABI_MIN: u32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkPolicy {
    Deny,
    Loopback,
    All,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SandboxPolicy {
    pub format: u64,
    pub read_roots: Vec<String>,
    pub write_roots: Vec<String>,
    pub network: NetworkPolicy,
    pub allow_process: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scratch: Option<String>,
    /// Directories no write may reach even beneath a write root, such as the
    /// Kernel storage root under a writable home folder. Only a write root
    /// declared strictly beneath a protected root (a Kernel-owned job
    /// directory) stays writable there; callers must never derive such a root
    /// from session input.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protected_roots: Vec<String>,
}

impl SandboxPolicy {
    pub fn validate(&self) -> Result<(), SandboxError> {
        if self.format != 1 {
            return Err(SandboxError::Invalid("format must equal 1"));
        }
        validate_roots(&self.read_roots)?;
        validate_roots(&self.write_roots)?;
        validate_roots(&self.protected_roots)?;
        if let Some(scratch) = &self.scratch {
            validate_absolute(scratch)?;
        }
        for root in &self.write_roots {
            if !self
                .read_roots
                .iter()
                .any(|read| Path::new(root).starts_with(read))
            {
                return Err(SandboxError::Invalid(
                    "every write root must be beneath a read root",
                ));
            }
        }
        Ok(())
    }

    /// True when `root` lies strictly beneath a protected root, which keeps
    /// a write root declared there writable.
    #[must_use]
    pub fn is_protected_exception(&self, root: &Path) -> bool {
        self.protected_roots
            .iter()
            .any(|protected| root != Path::new(protected) && root.starts_with(protected))
    }

    /// The first protected root that `root` would expose to writes: one at or
    /// beneath `root`, unless `root` is itself a declared exception.
    #[must_use]
    pub fn exposed_protected_root(&self, root: &Path) -> Option<&str> {
        if self.is_protected_exception(root) {
            return None;
        }
        self.protected_roots
            .iter()
            .find(|protected| Path::new(protected).starts_with(root))
            .map(String::as_str)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, SandboxError> {
        self.validate()?;
        serde_json_canonicalizer::to_vec(self)
            .map_err(|error| SandboxError::Encoding(error.to_string()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxBackend {
    DarwinSeatbeltV1,
    /// Prefer a verified bubblewrap backend, then a verified Landlock backend.
    LinuxAutoV1,
    LinuxBubblewrapSeccompV1,
    LinuxLandlockSeccompV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeStatus {
    Available { backend: String, version: String },
    Unavailable { class: ProbeFailure, detail: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeFailure {
    Missing,
    Rejected,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SandboxApproval {
    pub run: String,
    pub call: String,
    pub policy_digest: String,
    pub grant: bool,
}

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("invalid sandbox policy: {0}")]
    Invalid(&'static str),
    #[error("sandbox encoding failed: {0}")]
    Encoding(String),
    #[error("sandbox backend unavailable: {0}")]
    Unavailable(String),
    #[error("sandbox probe failed: {0}")]
    Probe(String),
    #[error("unsandboxed approval does not match run/call/policy")]
    ApprovalMismatch,
    #[error("sandbox I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Directories no session may write or use as a folder: the Kernel storage
/// root and the service user's platform credential store. Paths are
/// canonical; a credential store that does not exist is omitted.
#[must_use]
pub fn kernel_protected_roots(storage_root: &Path) -> Vec<String> {
    let mut roots = Vec::new();
    roots.push(canonical_text(storage_root));
    if let Some(home) = service_user_home() {
        #[cfg(target_os = "macos")]
        let stores = [home.join("Library/Keychains")];
        #[cfg(not(target_os = "macos"))]
        let stores = [home.join(".local/share/keyrings")];
        for store in stores {
            if let Ok(canonical) = store.canonicalize() {
                roots.push(canonical.to_string_lossy().into_owned());
            }
        }
    }
    roots.sort();
    roots.dedup();
    roots
}

/// The protected roots that lie at or beneath one of `write_roots`, which
/// a sandbox built from those write roots must deny explicitly. Roots outside
/// every write root are already unwritable, so the policy omits them.
#[must_use]
pub fn exposed_protected_roots(protected: &[String], write_roots: &[String]) -> Vec<String> {
    let writes = write_roots
        .iter()
        .flat_map(|root| {
            let canonical = canonical_text(Path::new(root));
            [root.clone(), canonical]
        })
        .collect::<Vec<_>>();
    let mut exposed = protected
        .iter()
        .filter(|root| {
            writes
                .iter()
                .any(|write| Path::new(root).starts_with(write))
        })
        .cloned()
        .collect::<Vec<_>>();
    exposed.sort();
    exposed.dedup();
    exposed
}

/// The protected root that `path` names or lies beneath, if any. `path` is
/// compared both as given and resolved through its nearest existing
/// ancestor, so a symlinked alias of a protected directory is caught too.
#[must_use]
pub fn protected_root_containing<'a>(protected: &'a [String], path: &Path) -> Option<&'a str> {
    let mut existing = path;
    let mut suffix = Vec::new();
    let resolved = loop {
        if let Ok(canonical) = existing.canonicalize() {
            break Some(
                suffix
                    .iter()
                    .rev()
                    .fold(canonical, |base, part| base.join(part)),
            );
        }
        match (existing.parent(), existing.file_name()) {
            (Some(parent), Some(name)) => {
                suffix.push(name.to_owned());
                existing = parent;
            }
            _ => break None,
        }
    };
    protected
        .iter()
        .find(|root| {
            path.starts_with(root)
                || resolved
                    .as_deref()
                    .is_some_and(|resolved| resolved.starts_with(root))
        })
        .map(String::as_str)
}

fn canonical_text(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

/// The home directory from the password database, not `HOME`: workers and
/// tool processes run with a cleared environment.
fn service_user_home() -> Option<std::path::PathBuf> {
    use std::ffi::CStr;
    use std::os::unix::ffi::OsStrExt;
    let mut buffer = vec![0_u8; 16 * 1024];
    // SAFETY: zeroed passwd is a valid out-parameter for getpwuid_r.
    let mut entry: libc::passwd = unsafe { std::mem::zeroed() };
    let mut result = std::ptr::null_mut();
    // SAFETY: every pointer is live for the call and the buffer length is
    // its real size; getpwuid_r writes only into `entry` and `buffer`.
    let status = unsafe {
        libc::getpwuid_r(
            libc::geteuid(),
            &mut entry,
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            &mut result,
        )
    };
    if status != 0 || result.is_null() || entry.pw_dir.is_null() {
        return None;
    }
    // SAFETY: on success pw_dir points to a NUL-terminated string in `buffer`.
    let home = unsafe { CStr::from_ptr(entry.pw_dir) };
    let home = Path::new(std::ffi::OsStr::from_bytes(home.to_bytes()));
    home.is_absolute().then(|| home.to_path_buf())
}

pub fn policy_digest(policy: &SandboxPolicy) -> Result<String, SandboxError> {
    Ok(format!(
        "sha256-{:x}",
        Sha256::digest(policy.canonical_bytes()?)
    ))
}

pub fn compile_darwin_profile(policy: &SandboxPolicy) -> Result<Vec<u8>, SandboxError> {
    policy.validate()?;
    let mut lines = vec![
        "(version 1)".to_owned(),
        "(deny default)".to_owned(),
        "(import \"system.sb\")".to_owned(),
    ];
    // Executables may resolve their installation path before loading libraries.
    // Permit metadata traversal of declared roots' ancestors without permitting
    // directory listings or reading files outside the declared roots.
    let ancestors = policy
        .read_roots
        .iter()
        .chain(&policy.write_roots)
        .chain(policy.scratch.iter())
        .flat_map(|root| Path::new(root).ancestors().skip(1))
        .filter_map(Path::to_str)
        .collect::<BTreeSet<_>>();
    for ancestor in ancestors {
        lines.push(format!(
            "(allow file-read-metadata (literal \"{}\"))",
            escape_sbpl(ancestor)?
        ));
    }
    // Developer-provided command-line interpreters are read-only runtime inputs.
    // Their scripts and outputs remain constrained by the workspace read/write roots.
    // Direct interpreter paths avoid xcode-select's cache writes in /var/folders.
    lines.push("(allow file-read-metadata (literal \"/Applications\"))".to_owned());
    lines.push("(allow file-read-metadata (literal \"/Library\"))".to_owned());
    for root in [
        "/System",
        "/usr",
        "/bin",
        "/sbin",
        "/private/var/select",
        "/Applications/Xcode.app",
        "/Library/Developer/CommandLineTools",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .chain(policy.read_roots.iter().cloned())
    {
        lines.push(format!(
            "(allow file-read* (subpath \"{}\"))",
            escape_sbpl(&root)?
        ));
    }
    for root in &policy.write_roots {
        lines.push(format!(
            "(allow file-write* (subpath \"{}\"))",
            escape_sbpl(root)?
        ));
    }
    if let Some(scratch) = &policy.scratch {
        lines.push(format!(
            "(allow file-read* file-write* (subpath \"{}\"))",
            escape_sbpl(scratch)?
        ));
    }
    // Seatbelt applies the last matching rule, so these denials override the
    // write grants above. Seatbelt matches paths, not objects: renaming a
    // writable ancestor would move a protected tree out from under its rule,
    // so each ancestor within a write root is itself denied (creating
    // siblings inside it stays allowed).
    let grants = policy
        .write_roots
        .iter()
        .chain(policy.scratch.iter())
        .collect::<Vec<_>>();
    let mut denied_ancestors = BTreeSet::new();
    for protected in &policy.protected_roots {
        lines.push(format!(
            "(deny file-write* (subpath \"{}\"))",
            escape_sbpl(protected)?
        ));
        for ancestor in Path::new(protected).ancestors().skip(1) {
            if grants.iter().any(|root| ancestor.starts_with(root)) {
                denied_ancestors.extend(ancestor.to_str());
            }
        }
    }
    for ancestor in denied_ancestors {
        lines.push(format!(
            "(deny file-write* (literal \"{}\"))",
            escape_sbpl(ancestor)?
        ));
    }
    for root in grants
        .iter()
        .filter(|root| policy.is_protected_exception(Path::new(root)))
    {
        lines.push(format!(
            "(allow file-write* (subpath \"{}\"))",
            escape_sbpl(root)?
        ));
    }
    if policy.allow_process {
        lines.push("(allow process-exec process-fork)".to_owned());
    }
    match policy.network {
        NetworkPolicy::Deny => {}
        NetworkPolicy::Loopback => {
            lines.push("(allow network* (local ip \"localhost:*\"))".to_owned());
            lines.push("(allow network* (remote ip \"localhost:*\"))".to_owned());
        }
        NetworkPolicy::All => lines.push("(allow network*)".to_owned()),
    }
    let mut bytes = lines.join("\n").into_bytes();
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn compile_linux_plan(policy: &SandboxPolicy) -> Result<Vec<u8>, SandboxError> {
    policy.validate()?;
    let mut plan = json!({
        "allow_process": policy.allow_process,
        "backend": "linux-landlock-seccomp",
        "landlock_abi_min": LINUX_LANDLOCK_ABI_MIN,
        "network": policy.network,
        "read_roots": policy.read_roots,
        "seccomp_classes": if policy.allow_process {
            vec!["basic", "filesystem", "process"]
        } else {
            vec!["basic", "filesystem"]
        },
        "write_roots": policy.write_roots,
    });
    if let Some(scratch) = &policy.scratch {
        plan.as_object_mut()
            .expect("plan object")
            .insert("scratch".to_owned(), json!(scratch));
    }
    if !policy.protected_roots.is_empty() {
        plan.as_object_mut()
            .expect("plan object")
            .insert("protected_roots".to_owned(), json!(policy.protected_roots));
    }
    let mut bytes = serde_json_canonicalizer::to_vec(&plan)
        .map_err(|error| SandboxError::Encoding(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Deterministic description of the bubblewrap launcher. Probe and launch must
/// still verify installed bubblewrap, user namespaces and seccomp at runtime.
pub fn compile_linux_bubblewrap_plan(policy: &SandboxPolicy) -> Result<Vec<u8>, SandboxError> {
    policy.validate()?;
    if policy.network == NetworkPolicy::Loopback || !policy.allow_process {
        return Err(SandboxError::Unavailable(
            "bubblewrap requires allow_process=true and network=deny or all".to_owned(),
        ));
    }
    let mut namespaces = vec!["ipc", "mount", "pid", "user", "uts"];
    if policy.network == NetworkPolicy::Deny {
        namespaces.push("network");
        namespaces.sort_unstable();
    }
    let mut plan = json!({
        "allow_process": policy.allow_process,
        "backend": "linux-bubblewrap-seccomp",
        "bubblewrap_version_min": "0.4.0",
        "capabilities": [],
        "namespaces": namespaces,
        "network": policy.network,
        "private_devices": true,
        "private_proc": true,
        "read_roots": policy.read_roots,
        "root_readonly": true,
        "seccomp_classes": ["basic", "filesystem", "process"],
        "write_roots": policy.write_roots,
    });
    if let Some(scratch) = &policy.scratch {
        plan.as_object_mut()
            .expect("plan object")
            .insert("scratch".to_owned(), json!(scratch));
    }
    if !policy.protected_roots.is_empty() {
        plan.as_object_mut()
            .expect("plan object")
            .insert("protected_roots".to_owned(), json!(policy.protected_roots));
    }
    let mut bytes = serde_json_canonicalizer::to_vec(&plan)
        .map_err(|error| SandboxError::Encoding(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn validate_unsandboxed_approval(
    approval: &SandboxApproval,
    run: &str,
    call: &str,
    digest: &str,
) -> Result<(), SandboxError> {
    if approval.grant
        && approval.run == run
        && approval.call == call
        && approval.policy_digest == digest
    {
        Ok(())
    } else {
        Err(SandboxError::ApprovalMismatch)
    }
}

pub fn probe_backend(backend: SandboxBackend) -> ProbeStatus {
    match backend {
        SandboxBackend::DarwinSeatbeltV1 => probe_darwin(),
        SandboxBackend::LinuxAutoV1 => select_linux_backend(probe_bubblewrap(), probe_linux),
        SandboxBackend::LinuxBubblewrapSeccompV1 => probe_bubblewrap(),
        SandboxBackend::LinuxLandlockSeccompV1 => probe_linux(),
    }
}

pub(crate) fn sandbox_command(
    policy: &SandboxPolicy,
    status: &ProbeStatus,
    executable: &Path,
) -> Result<Command, SandboxError> {
    let ProbeStatus::Available { backend, .. } = status else {
        return Err(SandboxError::Unavailable(
            "sandbox probe did not report available".to_owned(),
        ));
    };
    #[cfg(target_os = "macos")]
    {
        if backend != "darwin-seatbelt" {
            return Err(SandboxError::Unavailable(format!(
                "probe selected incompatible backend {backend}"
            )));
        }
        let profile = String::from_utf8(compile_darwin_profile(policy)?)
            .map_err(|_| SandboxError::Encoding("profile is not UTF-8".to_owned()))?;
        let mut command = Command::new("/usr/bin/sandbox-exec");
        command.arg("-p").arg(profile).arg(executable);
        Ok(command)
    }
    #[cfg(all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    {
        if backend == crate::bubblewrap_sandbox::BACKEND {
            return crate::bubblewrap_sandbox::command(policy, executable);
        }
        if backend != crate::linux_sandbox::BACKEND {
            return Err(SandboxError::Unavailable(format!(
                "probe selected incompatible backend {backend}"
            )));
        }
        let mut command = Command::new(executable);
        crate::linux_sandbox::confine(&mut command, policy, executable)?;
        Ok(command)
    }
    #[cfg(not(any(
        target_os = "macos",
        all(
            target_os = "linux",
            any(target_arch = "x86_64", target_arch = "aarch64")
        )
    )))]
    {
        let _ = (policy, backend, executable);
        Err(SandboxError::Unavailable(
            "no applied sandbox launcher on this platform".to_owned(),
        ))
    }
}

fn probe_darwin() -> ProbeStatus {
    #[cfg(not(target_os = "macos"))]
    {
        ProbeStatus::Unavailable {
            class: ProbeFailure::Unsupported,
            detail: "Seatbelt requested on a non-macOS host".to_owned(),
        }
    }
    #[cfg(target_os = "macos")]
    {
        if !Path::new("/usr/bin/sandbox-exec").is_file() {
            return ProbeStatus::Unavailable {
                class: ProbeFailure::Missing,
                detail: "sandbox-exec is missing".to_owned(),
            };
        }
        probe_confinement(ProbeStatus::Available {
            backend: "darwin-seatbelt".to_owned(),
            version: std::env::consts::OS.to_owned(),
        })
    }
}

fn select_linux_backend(
    bubblewrap: ProbeStatus,
    landlock: impl FnOnce() -> ProbeStatus,
) -> ProbeStatus {
    if matches!(bubblewrap, ProbeStatus::Available { .. }) {
        return bubblewrap;
    }
    let fallback = landlock();
    if matches!(fallback, ProbeStatus::Available { .. }) {
        return fallback;
    }
    let ProbeStatus::Unavailable { class, detail } = fallback else {
        unreachable!()
    };
    ProbeStatus::Unavailable {
        class,
        detail: format!("bubblewrap probe: {bubblewrap:?}; Landlock probe: {detail}"),
    }
}

fn probe_bubblewrap() -> ProbeStatus {
    #[cfg(not(all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )))]
    {
        ProbeStatus::Unavailable {
            class: ProbeFailure::Unsupported,
            detail: "bubblewrap/seccomp requires Linux on x86_64 or aarch64".to_owned(),
        }
    }
    #[cfg(all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    {
        let binary = match crate::bubblewrap_sandbox::binary() {
            Ok(binary) => binary,
            Err(error) => {
                return ProbeStatus::Unavailable {
                    class: ProbeFailure::Missing,
                    detail: error.to_string(),
                };
            }
        };
        let version = match probe_output(Command::new(binary).arg("--version")) {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_owned()
            }
            result => {
                return ProbeStatus::Unavailable {
                    class: ProbeFailure::Rejected,
                    detail: format!("bubblewrap version probe failed: {result:?}"),
                };
            }
        };
        probe_confinement(ProbeStatus::Available {
            backend: crate::bubblewrap_sandbox::BACKEND.to_owned(),
            version,
        })
    }
}

fn probe_linux() -> ProbeStatus {
    #[cfg(not(all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )))]
    {
        ProbeStatus::Unavailable {
            class: ProbeFailure::Unsupported,
            detail: "Landlock/seccomp requires Linux on x86_64 or aarch64".to_owned(),
        }
    }
    #[cfg(all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    {
        match crate::linux_sandbox::supported_abi() {
            Ok(abi) => probe_confinement(ProbeStatus::Available {
                backend: crate::linux_sandbox::BACKEND.to_owned(),
                version: format!("landlock-abi-{abi}"),
            }),
            Err((class, detail)) => ProbeStatus::Unavailable { class, detail },
        }
    }
}

/// Returns `candidate` only after a confined `cat` reads a file inside the
/// policy and a confined `touch` fails to create one outside it.
#[cfg(any(
    target_os = "macos",
    all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
))]
fn probe_confinement(candidate: ProbeStatus) -> ProbeStatus {
    let unresolved_root = std::env::temp_dir().join(format!(
        "tekes-sandbox-probe-{}-{}",
        std::process::id(),
        probe_nonce()
    ));
    let allowed_unresolved = unresolved_root.join("allowed");
    let outside_unresolved = unresolved_root.join("outside");
    let result = (|| -> Result<(), SandboxError> {
        fs::create_dir_all(&allowed_unresolved)?;
        fs::create_dir_all(&outside_unresolved)?;
        let root = fs::canonicalize(&unresolved_root)?;
        let allowed = root.join("allowed");
        let outside = root.join("outside");
        fs::write(allowed.join("read.txt"), b"ok")?;
        let policy = SandboxPolicy {
            format: 1,
            read_roots: vec![allowed.to_string_lossy().into_owned()],
            write_roots: vec![allowed.to_string_lossy().into_owned()],
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
            protected_roots: Vec::new(),
        };
        let read = probe_output(
            sandbox_command(&policy, &candidate, Path::new("/bin/cat"))?
                .current_dir(&allowed)
                .arg(allowed.join("read.txt")),
        )?;
        let write = probe_output(
            sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?
                .current_dir(&allowed)
                .args(["-c", "printf ok > write.txt"]),
        )?;
        let denied = probe_output(
            sandbox_command(&policy, &candidate, Path::new("/bin/sh"))?
                .current_dir(&allowed)
                .args(["-c", "printf denied > \"$1\"", "probe"])
                .arg(outside.join("denied.txt")),
        )?;
        if read.status.success()
            && read.stdout == b"ok"
            && write.status.success()
            && fs::read(allowed.join("write.txt"))? == b"ok"
            && !denied.status.success()
            && !outside.join("denied.txt").exists()
        {
            Ok(())
        } else {
            Err(SandboxError::Probe(format!(
                "read_status={:?} read_stderr={} denied_status={:?} denied_stderr={} outside_exists={}",
                read.status.code(),
                String::from_utf8_lossy(&read.stderr),
                denied.status.code(),
                String::from_utf8_lossy(&denied.stderr),
                outside.join("denied.txt").exists()
            )))
        }
    })();
    let _ = fs::remove_dir_all(&unresolved_root);
    match result {
        Ok(()) => candidate,
        Err(error) => ProbeStatus::Unavailable {
            class: ProbeFailure::Rejected,
            detail: error.to_string(),
        },
    }
}

/// Keep a broken launcher from hanging host startup. Probe commands have only
/// bounded output (version, cat and shell builtins) and a dedicated process group.
#[cfg(any(
    target_os = "macos",
    all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
))]
fn probe_output(command: &mut Command) -> Result<std::process::Output, SandboxError> {
    use std::os::unix::process::CommandExt;
    use std::process::Stdio;
    use std::time::{Duration, Instant};
    let mut child = command
        .process_group(0)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait()?.is_none() {
        if Instant::now() >= deadline {
            // SAFETY: the child was assigned its own process group above.
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            let _ = child.kill();
            let _ = child.wait();
            return Err(SandboxError::Probe("sandbox probe timed out".to_owned()));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(child.wait_with_output()?)
}

fn validate_roots(values: &[String]) -> Result<(), SandboxError> {
    let mut previous: Option<&str> = None;
    let mut unique = BTreeSet::new();
    for value in values {
        validate_absolute(value)?;
        if previous.is_some_and(|last| last.as_bytes() >= value.as_bytes())
            || !unique.insert(value.as_str())
        {
            return Err(SandboxError::Invalid(
                "roots must be unique and UTF-8 byte-sorted",
            ));
        }
        previous = Some(value);
    }
    Ok(())
}

fn validate_absolute(value: &str) -> Result<(), SandboxError> {
    let path = Path::new(value);
    if !path.is_absolute()
        || value.contains('\0')
        || value.chars().any(char::is_control)
        || path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err(SandboxError::Invalid(
            "sandbox roots must be absolute canonical text paths",
        ));
    }
    Ok(())
}

fn escape_sbpl(value: &str) -> Result<String, SandboxError> {
    if value.chars().any(char::is_control) {
        return Err(SandboxError::Invalid(
            "SBPL path contains control character",
        ));
    }
    Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(any(
    target_os = "macos",
    all(
        target_os = "linux",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
))]
fn probe_nonce() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NONCE: AtomicU64 = AtomicU64::new(0);
    NONCE.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn available(backend: &str) -> ProbeStatus {
        ProbeStatus::Available {
            backend: backend.to_owned(),
            version: "test".to_owned(),
        }
    }
    fn missing() -> ProbeStatus {
        ProbeStatus::Unavailable {
            class: ProbeFailure::Missing,
            detail: "missing".to_owned(),
        }
    }
    #[test]
    fn linux_auto_prefers_bubblewrap_without_probing_landlock() {
        assert_eq!(
            select_linux_backend(available("linux-bubblewrap-seccomp"), || panic!(
                "unnecessary fallback"
            )),
            available("linux-bubblewrap-seccomp")
        );
    }
    #[test]
    fn linux_auto_falls_back_only_to_available_landlock() {
        assert_eq!(
            select_linux_backend(missing(), || available("linux-landlock-seccomp")),
            available("linux-landlock-seccomp")
        );
        assert!(matches!(
            select_linux_backend(missing(), missing),
            ProbeStatus::Unavailable { .. }
        ));
    }
    #[test]
    fn bubblewrap_plan_matches_golden_bytes() {
        let policy: SandboxPolicy = serde_json::from_str(include_str!(
            "../../../fixtures/sandbox/policy.canonical.json"
        ))
        .unwrap();
        assert_eq!(
            compile_linux_bubblewrap_plan(&policy).unwrap(),
            include_bytes!("../../../fixtures/sandbox/linux-bubblewrap-plan.canonical.json")
        );
    }

    fn protected_policy(home: &str, protected: &str, writes: &[&str]) -> SandboxPolicy {
        SandboxPolicy {
            format: 1,
            read_roots: vec![home.to_owned()],
            write_roots: writes.iter().map(|root| (*root).to_owned()).collect(),
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
            protected_roots: vec![protected.to_owned()],
        }
    }

    #[test]
    fn protected_roots_are_omitted_from_serialization_when_empty() {
        let policy: SandboxPolicy = serde_json::from_str(include_str!(
            "../../../fixtures/sandbox/policy.canonical.json"
        ))
        .unwrap();
        assert!(policy.protected_roots.is_empty());
        assert_eq!(
            policy.canonical_bytes().unwrap(),
            include_str!("../../../fixtures/sandbox/policy.canonical.json")
                .trim_end()
                .as_bytes()
        );
    }

    #[test]
    fn darwin_profile_denies_protected_root_and_its_writable_ancestors() {
        let policy = protected_policy(
            "/Users/u",
            "/Users/u/Library/Kernel",
            &["/Users/u", "/Users/u/Library/Kernel/jobs/j1"],
        );
        let profile = String::from_utf8(compile_darwin_profile(&policy).unwrap()).unwrap();
        let lines = profile.lines().collect::<Vec<_>>();
        let position = |line: &str| {
            lines
                .iter()
                .position(|candidate| *candidate == line)
                .unwrap_or_else(|| panic!("missing {line}\n{profile}"))
        };
        let home_grant = position("(allow file-write* (subpath \"/Users/u\"))");
        let deny = position("(deny file-write* (subpath \"/Users/u/Library/Kernel\"))");
        let library = position("(deny file-write* (literal \"/Users/u/Library\"))");
        let home = position("(deny file-write* (literal \"/Users/u\"))");
        let exception = lines
            .iter()
            .rposition(|line| {
                *line == "(allow file-write* (subpath \"/Users/u/Library/Kernel/jobs/j1\"))"
            })
            .unwrap();
        assert!(home_grant < deny && deny < library.min(home));
        assert!(exception > library.max(home), "{profile}");
        assert!(
            !profile.contains("(deny file-write* (literal \"/Users\"))"),
            "{profile}"
        );
    }

    #[test]
    fn protected_exceptions_must_be_strictly_beneath() {
        let policy = protected_policy("/a", "/a/k", &["/a"]);
        assert!(!policy.is_protected_exception(Path::new("/a/k")));
        assert!(policy.is_protected_exception(Path::new("/a/k/jobs")));
        assert!(!policy.is_protected_exception(Path::new("/a/kx")));
        assert_eq!(policy.exposed_protected_root(Path::new("/a")), Some("/a/k"));
        assert_eq!(
            policy.exposed_protected_root(Path::new("/a/k")),
            Some("/a/k")
        );
        assert_eq!(policy.exposed_protected_root(Path::new("/a/k/jobs")), None);
        assert_eq!(policy.exposed_protected_root(Path::new("/b")), None);
    }

    #[test]
    fn unsorted_protected_roots_are_rejected() {
        let mut policy = protected_policy("/a", "/a/k", &["/a"]);
        policy.protected_roots = vec!["/a/z".to_owned(), "/a/k".to_owned()];
        assert!(policy.validate().is_err());
        policy.protected_roots = vec!["relative".to_owned()];
        assert!(policy.validate().is_err());
    }

    #[test]
    fn exposed_roots_keep_only_protected_roots_inside_write_roots() {
        let protected = vec!["/h/Library/Keychains".to_owned(), "/h/data".to_owned()];
        assert_eq!(
            exposed_protected_roots(&protected, &["/h".to_owned()]),
            protected
        );
        assert_eq!(
            exposed_protected_roots(&protected, &["/h/data".to_owned()]),
            vec!["/h/data".to_owned()]
        );
        assert!(exposed_protected_roots(&protected, &["/h/project".to_owned()]).is_empty());
    }

    #[test]
    fn protected_root_lookup_resolves_symlinked_aliases() {
        let root = tempfile::tempdir().unwrap();
        let base = root.path().canonicalize().unwrap();
        let storage = base.join("storage");
        fs::create_dir_all(storage.join("threads")).unwrap();
        std::os::unix::fs::symlink(&storage, base.join("alias")).unwrap();
        let protected = kernel_protected_roots(&storage);
        assert!(protected.contains(&storage.to_string_lossy().into_owned()));
        for path in [
            storage.clone(),
            storage.join("threads/new/file"),
            base.join("alias/threads"),
            base.join("alias/missing/deeper"),
        ] {
            assert!(
                protected_root_containing(&protected, &path).is_some(),
                "{}",
                path.display()
            );
        }
        assert!(protected_root_containing(&protected, &base.join("storage-sibling")).is_none());
        assert!(protected_root_containing(&protected, &base).is_none());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn seatbelt_keeps_protected_root_read_only_beneath_a_writable_home() {
        let root = tempfile::tempdir().unwrap();
        let home = root.path().canonicalize().unwrap();
        let storage = home.join("Library/Kernel");
        let job = storage.join("jobs/j1");
        fs::create_dir_all(&job).unwrap();
        fs::create_dir_all(home.join("project")).unwrap();
        fs::write(storage.join("ledger"), b"original").unwrap();
        let text = |path: &Path| path.to_string_lossy().into_owned();
        let mut writes = vec![text(&home), text(&job)];
        writes.sort();
        let policy = SandboxPolicy {
            format: 1,
            read_roots: vec![text(&home)],
            write_roots: writes,
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
            protected_roots: vec![text(&storage)],
        };
        let status = available("darwin-seatbelt");
        let run = |script: &str| {
            sandbox_command(&policy, &status, Path::new("/bin/sh"))
                .unwrap()
                .args(["-c", script])
                .current_dir(&home)
                .env_clear()
                .output()
                .unwrap()
                .status
                .success()
        };
        assert!(run("echo ok > project/file"));
        assert!(run("echo ok > Library/sibling"));
        assert!(run("echo ok > Library/Kernel/jobs/j1/tail"));
        assert!(!run("echo pwned > Library/Kernel/ledger"));
        assert!(!run("echo pwned > Library/Kernel/new"));
        assert!(!run("mv Library Moved"));
        assert!(!run("mv Library/Kernel Moved"));
        assert!(!run(
            "ln Library/Kernel/ledger project/link && echo pwned >> project/link"
        ));
        assert!(!run("rm Library/Kernel/ledger"));
        assert_eq!(fs::read(storage.join("ledger")).unwrap(), b"original");
        assert!(!home.join("Moved").exists());
    }
}
