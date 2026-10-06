use std::collections::BTreeSet;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use thiserror::Error;

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
}

impl SandboxPolicy {
    pub fn validate(&self) -> Result<(), SandboxError> {
        if self.format != 1 {
            return Err(SandboxError::Invalid("format must equal 1"));
        }
        validate_roots(&self.read_roots)?;
        validate_roots(&self.write_roots)?;
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
        "landlock_abi_min": 1,
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
        };
        let read = sandbox_command(&policy, &candidate, Path::new("/bin/cat"))?
            .arg(allowed.join("read.txt"))
            .output()?;
        let denied = sandbox_command(&policy, &candidate, Path::new("/usr/bin/touch"))?
            .arg(outside.join("denied.txt"))
            .output()?;
        if read.status.success()
            && read.stdout == b"ok"
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
