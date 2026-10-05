//! Production Slice-10 supervisor service assembly.

#[cfg(target_os = "macos")]
use std::ffi::CString;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
#[cfg(target_os = "macos")]
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use endpoint::SessionHostDescription;
use profile::{ConfigRepository, InstructionResolver, ResourceCatalog};
use serde::{Deserialize, Serialize};
use store::{LockedLedger, StoreError, ThreadStore, scan_valid_prefix};
use thiserror::Error;
use tokio::net::TcpListener;
use transport::{BearerToken, TransportConfig, WebClientConfig};

use crate::client_admin::ClientAdminRoutes;
use crate::client_extensions::ProductionClientExtensions;
use crate::endpoint_carrier::{ProductionCarrierAssembly, ProductionCarrierError};
use crate::endpoint_host::{
    CompositeProductionEndpointRoutes, EndpointAssemblyError, EndpointClock,
    ProductionEndpointHost, ProviderReadinessAuthority, QueueTransactionAuthority,
    SessionDeliveryAuthority, SessionInputAdmissionAuthority,
};
use crate::observability::{
    Correlation, FrozenAttribution, LogRecord, OperationalMetrics, ProductionAccessLog,
    RotatingJsonlLog, Severity, operational_code_count, publish_metric_snapshot,
};
use crate::process_host::ProductionProcessHost;
use crate::resource_capability::EndpointCommandInputAuthority;

pub const AUTHORITY_REGISTRY_SHA256: &str =
    "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469";
/// Git commit a release build came from, set at compile time with `TEKES_SOURCE_REVISION`.
/// Development builds leave it unset. Any other value than 40 lowercase hex digits fails
/// the build.
pub const SOURCE_REVISION: Option<&str> = match option_env!("TEKES_SOURCE_REVISION") {
    Some(revision) => {
        assert!(
            is_source_revision(revision),
            "TEKES_SOURCE_REVISION must be 40 lowercase hex digits"
        );
        Some(revision)
    }
    None => None,
};

const fn is_source_revision(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 40 {
        return false;
    }
    let mut index = 0;
    while index < bytes.len() {
        if !matches!(bytes[index], b'0'..=b'9' | b'a'..=b'f') {
            return false;
        }
        index += 1;
    }
    true
}

pub const PRODUCTION_WEB_LISTEN: &str = "127.0.0.1:7357";
pub const BOOTSTRAP_STATUS_FD: RawFd = 3;
pub const LAUNCHER_LIFETIME_FD: RawFd = 4;
pub const DRAIN_DEADLINE_SECONDS: u64 = 30;

static TERMINATE_REQUESTED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Selection {
    pub version: String,
    pub manifest_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaemonArgs {
    pub install_root: PathBuf,
    pub storage_root: PathBuf,
    pub listen: SocketAddr,
    pub selected_version: String,
    pub selector_generation: u64,
    pub launch_id: String,
    pub manifest_sha256: String,
    pub bootstrap_status_fd: RawFd,
    pub authority_registry_sha256: String,
    pub launcher_lifetime_fd: RawFd,
    pub web_listen: Option<SocketAddr>,
}

impl DaemonArgs {
    pub fn parse(arguments: impl IntoIterator<Item = OsString>) -> Result<Self, DaemonError> {
        let arguments = arguments.into_iter().collect::<Vec<_>>();
        if arguments.len() != 20 && arguments.len() != 22 {
            return Err(DaemonError::usage(
                "production supervisor requires ten option/value pairs and an optional Web Client listen pair",
            ));
        }
        require_flag(&arguments, 0, "--install-root")?;
        require_flag(&arguments, 2, "--storage-root")?;
        require_flag(&arguments, 4, "--listen")?;
        require_flag(&arguments, 6, "--selected-version")?;
        require_flag(&arguments, 8, "--selector-generation")?;
        require_flag(&arguments, 10, "--launch-id")?;
        require_flag(&arguments, 12, "--manifest-sha256")?;
        require_flag(&arguments, 14, "--bootstrap-status-fd")?;
        require_flag(&arguments, 16, "--authority-registry-sha256")?;
        require_flag(&arguments, 18, "--launcher-lifetime-fd")?;

        let install_root = PathBuf::from(&arguments[1]);
        let storage_root = PathBuf::from(&arguments[3]);
        if !install_root.is_absolute() {
            return Err(DaemonError::usage("install root must be absolute"));
        }
        if !storage_root.is_absolute() {
            return Err(DaemonError::usage("storage root must be absolute"));
        }
        let listen = utf8(&arguments[5], "listen")?
            .parse::<SocketAddr>()
            .map_err(|_| DaemonError::usage("listen address is invalid"))?;
        if listen != SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7347) {
            return Err(DaemonError::usage(
                "Slice 10 listen address must be 127.0.0.1:7347",
            ));
        }
        let selected_version = utf8(&arguments[7], "selected version")?.to_owned();
        validate_id(&selected_version)?;
        let selector_generation = parse_positive_u64(&arguments[9], "selector generation")?;
        let launch_id = utf8(&arguments[11], "launch id")?.to_owned();
        validate_launch_id(&launch_id, selector_generation)?;
        let manifest_sha256 = utf8(&arguments[13], "manifest digest")?.to_owned();
        validate_hex(&manifest_sha256)?;
        let bootstrap_status_fd = parse_fd(&arguments[15], "bootstrap status fd")?;
        let authority_registry_sha256 =
            utf8(&arguments[17], "authority registry digest")?.to_owned();
        validate_hex(&authority_registry_sha256)?;
        let launcher_lifetime_fd = parse_fd(&arguments[19], "launcher lifetime fd")?;
        let web_listen = if arguments.len() == 22 {
            require_flag(&arguments, 20, "--web-listen")?;
            let value = utf8(&arguments[21], "Web Client listen")?;
            if value != PRODUCTION_WEB_LISTEN {
                return Err(DaemonError::usage(
                    "Web Client listen address must be 127.0.0.1:7357",
                ));
            }
            Some(
                value
                    .parse::<SocketAddr>()
                    .map_err(|_| DaemonError::usage("Web Client listen address is invalid"))?,
            )
        } else {
            None
        };
        if bootstrap_status_fd != BOOTSTRAP_STATUS_FD
            || launcher_lifetime_fd != LAUNCHER_LIFETIME_FD
        {
            return Err(DaemonError::protocol(
                "production handoff requires bootstrap fd 3 and launcher lifetime fd 4",
            ));
        }
        Ok(Self {
            install_root,
            storage_root,
            listen,
            selected_version,
            selector_generation,
            launch_id,
            manifest_sha256,
            bootstrap_status_fd,
            authority_registry_sha256,
            launcher_lifetime_fd,
            web_listen,
        })
    }

    #[must_use]
    pub fn selection(&self) -> Selection {
        Selection {
            version: self.selected_version.clone(),
            manifest_sha256: self.manifest_sha256.clone(),
        }
    }
}

fn require_flag(arguments: &[OsString], index: usize, expected: &str) -> Result<(), DaemonError> {
    if arguments[index] == OsStr::new(expected) {
        Ok(())
    } else {
        Err(DaemonError::usage(
            "production supervisor arguments are out of order",
        ))
    }
}

fn utf8<'a>(value: &'a OsStr, field: &'static str) -> Result<&'a str, DaemonError> {
    value
        .to_str()
        .ok_or_else(|| DaemonError::usage(format!("{field} must be UTF-8")))
}

fn parse_positive_u64(value: &OsStr, field: &'static str) -> Result<u64, DaemonError> {
    let value = utf8(value, field)?
        .parse::<u64>()
        .map_err(|_| DaemonError::usage(format!("{field} is invalid")))?;
    if value == 0 || value > 9_007_199_254_740_991 {
        Err(DaemonError::usage(format!(
            "{field} must be a positive safe integer"
        )))
    } else {
        Ok(value)
    }
}

fn parse_fd(value: &OsStr, field: &'static str) -> Result<RawFd, DaemonError> {
    utf8(value, field)?
        .parse::<RawFd>()
        .map_err(|_| DaemonError::usage(format!("{field} is invalid")))
}

fn validate_id(value: &str) -> Result<(), DaemonError> {
    if (1..=128).contains(&value.len())
        && !value.starts_with('.')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        Ok(())
    } else {
        Err(DaemonError::usage("selected version is invalid"))
    }
}

fn validate_hex(value: &str) -> Result<(), DaemonError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(DaemonError::protocol("digest is not lowercase SHA-256"))
    }
}

fn validate_launch_id(value: &str, generation: u64) -> Result<(), DaemonError> {
    let mut parts = value.split('-');
    let parsed_generation = parts.next().and_then(|part| part.parse::<u64>().ok());
    let attempt = parts.next().and_then(|part| part.parse::<u64>().ok());
    let random = parts.next();
    if parts.next().is_none()
        && parsed_generation == Some(generation)
        && attempt.is_some_and(|attempt| attempt >= 1)
        && random.is_some_and(|random| {
            random.len() == 32
                && random
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
    {
        Ok(())
    } else {
        Err(DaemonError::protocol(
            "launch id does not match selector generation/attempt/nonce",
        ))
    }
}

#[derive(Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
enum BootstrapState<'a> {
    ListenerBound,
    Failed { code: &'a str },
}

#[derive(Serialize)]
struct BootstrapStatus<'a> {
    format: u8,
    launch_id: &'a str,
    selection: &'a Selection,
    #[serde(flatten)]
    state: BootstrapState<'a>,
}

pub struct BootstrapReporter {
    file: Option<File>,
    emitted: bool,
}

impl BootstrapReporter {
    pub fn from_fd(fd: RawFd) -> Result<Self, DaemonError> {
        let duplicated = duplicate_fd(fd).map_err(DaemonError::io)?;
        // SAFETY: `duplicated` is a new owned descriptor returned by dup.
        let file = unsafe { File::from_raw_fd(duplicated) };
        Ok(Self {
            file: Some(file),
            emitted: false,
        })
    }

    fn from_inherited_fd(fd: RawFd) -> Result<Self, DaemonError> {
        let owned = take_inherited_fd(fd).map_err(DaemonError::io)?;
        // SAFETY: `owned` is a new descriptor and the inherited original was closed.
        let file = unsafe { File::from_raw_fd(owned) };
        Ok(Self {
            file: Some(file),
            emitted: false,
        })
    }

    pub fn listener_bound(
        &mut self,
        launch_id: &str,
        selection: &Selection,
    ) -> Result<(), DaemonError> {
        self.emit(BootstrapStatus {
            format: 1,
            launch_id,
            selection,
            state: BootstrapState::ListenerBound,
        })
    }

    pub fn failed(
        &mut self,
        launch_id: &str,
        selection: &Selection,
        code: &'static str,
    ) -> Result<(), DaemonError> {
        self.emit(BootstrapStatus {
            format: 1,
            launch_id,
            selection,
            state: BootstrapState::Failed { code },
        })
    }

    fn emit(&mut self, status: BootstrapStatus<'_>) -> Result<(), DaemonError> {
        if self.emitted {
            return Err(DaemonError::protocol(
                "bootstrap status was already emitted",
            ));
        }
        let mut bytes = serde_json_canonicalizer::to_vec(&status)
            .map_err(|error| DaemonError::protocol(error.to_string()))?;
        bytes.push(b'\n');
        let mut file = self
            .file
            .take()
            .ok_or_else(|| DaemonError::protocol("bootstrap descriptor is closed"))?;
        file.write_all(&bytes).map_err(DaemonError::io)?;
        file.flush().map_err(DaemonError::io)?;
        drop(file);
        self.emitted = true;
        Ok(())
    }
}

pub trait BearerCredentialSource: Send + Sync {
    fn load(&self, access_group: &str) -> Result<[u8; 32], DaemonError>;
}

pub struct EnvironmentBearer;

impl BearerCredentialSource for EnvironmentBearer {
    fn load(&self, _access_group: &str) -> Result<[u8; 32], DaemonError> {
        endpoint_token_from_environment()
    }
}

pub(crate) fn endpoint_token_from_environment() -> Result<[u8; 32], DaemonError> {
    let value = zeroize::Zeroizing::new(
        std::env::var("TEKES_KERNEL_ENDPOINT_TOKEN")
            .map_err(|_| DaemonError::credential("missing-endpoint-environment-token"))?,
    );
    decode_endpoint_token(&value)
}

fn decode_endpoint_token(value: &str) -> Result<[u8; 32], DaemonError> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(DaemonError::credential(
            "invalid-endpoint-environment-token",
        ));
    }
    let mut token = [0u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        token[index] = u8::from_str_radix(
            std::str::from_utf8(pair)
                .map_err(|_| DaemonError::credential("invalid-endpoint-environment-token"))?,
            16,
        )
        .map_err(|_| DaemonError::credential("invalid-endpoint-environment-token"))?;
    }
    Ok(token)
}

pub struct ProductionRootLock {
    file: File,
    path: PathBuf,
}

impl ProductionRootLock {
    pub fn acquire(storage_root: &Path) -> Result<Self, DaemonError> {
        validate_directory(storage_root, 0o700)?;
        let path = storage_root.join(".root-lock");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&path)
            .map_err(|error| DaemonError::invalid_install(path.clone(), error))?;
        validate_file_metadata(&file, &path, 0o600)?;
        loop {
            // SAFETY: file owns a live descriptor; flock changes advisory state only.
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                break;
            }
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            if error
                .raw_os_error()
                .is_some_and(|code| code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                return Err(DaemonError::already_running(storage_root));
            }
            return Err(DaemonError::io(error));
        }
        validate_file_metadata(&file, &path, 0o600)?;
        Ok(Self { file, path })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for ProductionRootLock {
    fn drop(&mut self) {
        // SAFETY: file owns a live descriptor; unlocking in Drop is best effort.
        let _ = unsafe { libc::flock(self.file.as_raw_fd(), libc::LOCK_UN) };
    }
}

fn validate_directory(path: &Path, mode: u32) -> Result<(), DaemonError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| DaemonError::invalid_install(path.to_path_buf(), error))?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_dir()
        || metadata.uid() != effective_uid()
        || metadata.mode() & 0o777 != mode
    {
        return Err(DaemonError::invalid_install_reason(
            path,
            "ownership, mode or type mismatch",
        ));
    }
    Ok(())
}

fn validate_file_metadata(file: &File, path: &Path, mode: u32) -> Result<(), DaemonError> {
    let descriptor = file.metadata().map_err(DaemonError::io)?;
    let pathname = fs::symlink_metadata(path)
        .map_err(|error| DaemonError::invalid_install(path.to_path_buf(), error))?;
    if pathname.file_type().is_symlink()
        || !pathname.file_type().is_file()
        || descriptor.dev() != pathname.dev()
        || descriptor.ino() != pathname.ino()
        || descriptor.uid() != effective_uid()
        || descriptor.mode() & 0o777 != mode
    {
        return Err(DaemonError::invalid_install_reason(
            path,
            "ownership, mode or inode mismatch",
        ));
    }
    Ok(())
}

fn effective_uid() -> u32 {
    // SAFETY: geteuid has no preconditions.
    unsafe { libc::geteuid() }
}

fn authority_root(storage_root: &Path) -> Result<&Path, DaemonError> {
    storage_root.parent().ok_or_else(|| {
        DaemonError::invalid_install_reason(storage_root, "storage root has no parent")
    })
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct InstallIdentity {
    format: u64,
    team_id: String,
    access_group: String,
    installer_requirement: String,
    client_requirement: String,
    selector_requirement: String,
    supervisor_requirement: String,
}

fn load_install_identity(install_root: &Path) -> Result<InstallIdentity, DaemonError> {
    validate_directory(install_root, 0o700)?;
    let installer = install_root
        .parent()
        .ok_or_else(|| {
            DaemonError::invalid_install_reason(install_root, "install root has no parent")
        })?
        .join("Installer");
    validate_directory(&installer, 0o700)?;
    let path = installer.join("install-identity.json");
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(&path)
        .map_err(|error| DaemonError::invalid_install(path.clone(), error))?;
    validate_file_metadata(&file, &path, 0o600)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(DaemonError::io)?;
    if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n') {
        return Err(DaemonError::invalid_install_reason(
            &path,
            "install identity is not canonical-plus-LF",
        ));
    }
    let identity: InstallIdentity = serde_json::from_slice(&bytes)
        .map_err(|error| DaemonError::invalid_install_reason(&path, &error.to_string()))?;
    let mut canonical = serde_json_canonicalizer::to_vec(&identity)
        .map_err(|error| DaemonError::invalid_install_reason(&path, &error.to_string()))?;
    canonical.push(b'\n');
    if canonical != bytes
        || identity.format != 1
        || identity.team_id.len() != 10
        || !identity
            .team_id
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        || identity.access_group != format!("{}.com.tekes.shared.endpoint", identity.team_id)
        || [
            &identity.installer_requirement,
            &identity.client_requirement,
            &identity.selector_requirement,
            &identity.supervisor_requirement,
        ]
        .iter()
        .any(|value| value.is_empty())
    {
        return Err(DaemonError::invalid_install_reason(
            &path,
            "install identity is invalid",
        ));
    }
    Ok(identity)
}

fn prepare_storage(storage_root: &Path) -> Result<ThreadStore, DaemonError> {
    validate_directory(storage_root, 0o700)?;
    let root = authority_root(storage_root)?;
    validate_directory(root, 0o700)?;
    reject_cloud_managed_storage(storage_root)?;
    require_production_apfs(root)?;
    for name in [
        "archive",
        "staging",
        ".create-staging",
        ".rewrite-trash",
        "workspaces",
        "memory",
        "goals",
        "jobs",
        "tool-state",
        "credential-state",
        "config",
        "cache",
        store::ENDPOINT_MANAGEMENT_DIR,
        "plugins",
    ] {
        let path = root.join(name);
        match fs::symlink_metadata(&path) {
            Ok(_) => validate_directory(&path, 0o700)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let mut builder = fs::DirBuilder::new();
                builder.mode(0o700);
                builder.create(&path).map_err(DaemonError::io)?;
                validate_directory(&path, 0o700)?;
            }
            Err(error) => return Err(DaemonError::io(error)),
        }
    }
    store::probe_local_filesystem(root).map_err(DaemonError::store)?;
    let store = ThreadStore::open(root).map_err(DaemonError::store)?;
    store
        .recover_rewrites_for_startup()
        .map_err(DaemonError::store)?;
    store.gc_rewrite_debris().map_err(DaemonError::store)?;
    // Scratch ledgers (`session.fork{ephemeral}`) do not survive a restart:
    // nothing holds a line lock yet, so every one of them is deleted here.
    store.sweep_ephemeral().map_err(DaemonError::store)?;
    sweep_semantic_ledgers(root)?;
    profile::ConfigRepository::open(root)
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
    Ok(store)
}

/// Deployment-test seam for the exact production storage preflight. It uses
/// the same no-symlink ownership checks, local-filesystem probe, recovery and
/// semantic sweep as daemon startup and does not acquire the root lock.
pub fn preflight_storage(storage_root: &Path) -> Result<(), DaemonError> {
    prepare_storage(storage_root).map(|_| ())
}

#[cfg(target_os = "macos")]
fn require_production_apfs(root: &Path) -> Result<(), DaemonError> {
    let path = CString::new(root.as_os_str().as_bytes()).map_err(|_| {
        DaemonError::store(StoreError::UnsupportedFilesystem(
            "storage path contains NUL".to_owned(),
        ))
    })?;
    // SAFETY: `stat` is initialized by statfs on success and `path` is a
    // NUL-terminated pathname valid for the duration of the call.
    let mut stat = unsafe { std::mem::zeroed::<libc::statfs>() };
    // SAFETY: pointers refer to the live values described above.
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return Err(DaemonError::io(io::Error::last_os_error()));
    }
    let name = stat
        .f_fstypename
        .iter()
        .map(|value| *value as u8)
        .take_while(|value| *value != 0)
        .collect::<Vec<_>>();
    require_production_filesystem_name(&String::from_utf8_lossy(&name).to_ascii_lowercase())
}

#[cfg(not(target_os = "macos"))]
fn require_production_apfs(_root: &Path) -> Result<(), DaemonError> {
    require_production_filesystem_name("non-darwin")
}

fn require_production_filesystem_name(name: &str) -> Result<(), DaemonError> {
    if name == "apfs" {
        Ok(())
    } else {
        Err(DaemonError::store(StoreError::UnsupportedFilesystem(
            format!("production daemon requires local APFS; found {name}"),
        )))
    }
}

fn reject_cloud_managed_storage(storage_root: &Path) -> Result<(), DaemonError> {
    let canonical = fs::canonicalize(storage_root).map_err(DaemonError::io)?;
    if is_icloud_mobile_documents_path(&canonical) {
        return Err(DaemonError::store(StoreError::UnsupportedFilesystem(
            "production storage is inside Library/Mobile Documents".to_owned(),
        )));
    }
    Ok(())
}

fn is_icloud_mobile_documents_path(path: &Path) -> bool {
    let mut prior_was_library = false;
    for component in path.components() {
        let std::path::Component::Normal(component) = component else {
            prior_was_library = false;
            continue;
        };
        if prior_was_library && component == OsStr::new("Mobile Documents") {
            return true;
        }
        prior_was_library = component == OsStr::new("Library");
    }
    false
}

fn sweep_semantic_ledgers(root: &Path) -> Result<(), DaemonError> {
    for collection in ["threads", "archive"] {
        let mut folders = fs::read_dir(root.join(collection))
            .map_err(DaemonError::io)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DaemonError::io)?;
        folders.sort_by_key(fs::DirEntry::file_name);
        for folder in folders {
            if !folder.file_type().map_err(DaemonError::io)?.is_dir() {
                if collection == "threads" && folder.file_name() == OsStr::new(".root-lock") {
                    continue;
                }
                return Err(DaemonError::corrupt(format!(
                    "{} contains a non-directory entry",
                    root.join(collection).display()
                )));
            }
            let mut ledgers = fs::read_dir(folder.path())
                .map_err(DaemonError::io)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(DaemonError::io)?;
            ledgers.sort_by_key(fs::DirEntry::file_name);
            for ledger in ledgers {
                let path = ledger.path();
                if !ledger.file_type().map_err(DaemonError::io)?.is_file()
                    || path.extension() != Some(OsStr::new("jsonl"))
                    || matches!(
                        path.file_name().and_then(OsStr::to_str),
                        Some("endpoint.jsonl")
                    )
                {
                    continue;
                }
                let bytes = fs::read(&path).map_err(DaemonError::io)?;
                let scan = scan_valid_prefix(&bytes, 1);
                if scan.projection.is_none() {
                    return Err(DaemonError::corrupt(format!(
                        "{} has no valid semantic prefix",
                        path.display()
                    )));
                }
                if scan.needs_repair() {
                    let suffix = &bytes[scan.valid_bytes as usize..];
                    if suffix.contains(&b'\n') {
                        return Err(DaemonError::corrupt(format!(
                            "{} has a complete invalid record",
                            path.display()
                        )));
                    }
                    drop(LockedLedger::open(&path, 1).map_err(DaemonError::store)?);
                }
            }
        }
    }
    Ok(())
}

pub async fn run_daemon(args: DaemonArgs) -> Result<(), DaemonError> {
    run_daemon_with_credential(args, Arc::new(EnvironmentBearer)).await
}

pub async fn run_daemon_with_credential(
    args: DaemonArgs,
    credential_source: Arc<dyn BearerCredentialSource>,
) -> Result<(), DaemonError> {
    let mut reporter = BootstrapReporter::from_inherited_fd(args.bootstrap_status_fd)?;
    let launcher_lifetime =
        take_inherited_fd(args.launcher_lifetime_fd).map_err(DaemonError::io)?;
    let selection = args.selection();
    let launch_id = args.launch_id.clone();
    match run_daemon_inner(
        &args,
        &launch_id,
        credential_source,
        &mut reporter,
        launcher_lifetime,
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(error) => {
            if !reporter.emitted {
                let _ = reporter.failed(&launch_id, &selection, error.bootstrap_code());
            }
            Err(error)
        }
    }
}

async fn run_daemon_inner(
    args: &DaemonArgs,
    launch_id: &str,
    credential_source: Arc<dyn BearerCredentialSource>,
    reporter: &mut BootstrapReporter,
    launcher_lifetime: RawFd,
) -> Result<(), DaemonError> {
    validate_handoff(args)?;
    let identity = load_install_identity(&args.install_root)?;
    validate_executable(&identity)?;
    let _root_lock = ProductionRootLock::acquire(&args.storage_root)?;
    let _store = prepare_storage(&args.storage_root)?;
    let authority_root = authority_root(&args.storage_root)?;
    let credential = credential_source.load(&identity.access_group)?;
    let home = user_home()?;
    let log_root = home.join(".agents").join("logs").join("kernel");
    validate_directory(&log_root, 0o700)?;
    let log = Arc::new(RotatingJsonlLog::new(log_root.join("supervisor.jsonl")));
    let attribution = FrozenAttribution {
        build: args.selected_version.clone(),
        launch_id: launch_id.to_owned(),
        attempt: launch_attempt(launch_id),
        generation: args.selector_generation,
        manifest_sha256: args.manifest_sha256.clone(),
    };
    let worker_binary =
        production_worker_binary(&std::env::current_exe().map_err(DaemonError::io)?)?;
    // The owning application supplies an explicit credential-id -> environment-name map.
    // No model credential is read from or written to the login Keychain.
    let bindings = match std::env::var("TEKES_KERNEL_CREDENTIAL_BINDINGS") {
        Ok(value) => serde_json::from_str::<std::collections::BTreeMap<String, String>>(&value)
            .map_err(|_| DaemonError::credential("provider-environment-bindings"))?,
        Err(std::env::VarError::NotPresent) => std::collections::BTreeMap::new(),
        Err(_) => return Err(DaemonError::credential("provider-environment-bindings")),
    };
    let provider_secrets = Arc::new(
        provider::EnvironmentSecretStore::capture(&bindings)
            .map_err(|_| DaemonError::credential("provider-environment-credentials"))?,
    );
    let process_host = ProductionProcessHost::open_with_secret_authorities(
        authority_root,
        worker_binary,
        &args.selected_version,
        authority_root,
        provider_secrets as Arc<dyn provider::SecretStore>,
        None,
    )?;
    process_host.preflight_mandatory_authorities()?;
    let unary = assemble_production_endpoint_host(
        authority_root,
        SessionHostDescription {
            version: args.selected_version.clone(),
            cwd: "/".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: home.to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| system_timestamp().map_err(EndpointAssemblyError::Clock)),
        authority_root,
        Arc::clone(&process_host),
    )?;
    let metrics = Arc::new(OperationalMetrics::default());
    if let Ok(rollbacks) = operational_code_count(&log_root, "selector.jsonl", "rollback-complete")
    {
        metrics.set("rollbacks_total", rollbacks as f64);
    }
    process_host.attach_metrics(Arc::clone(&metrics));
    let access_log = Arc::new(ProductionAccessLog::new(
        Arc::clone(&metrics),
        Arc::clone(&log),
        attribution.clone(),
    ));
    process_host.attach_observability(Arc::clone(&access_log));
    let config = TransportConfig::loopback(args.listen, BearerToken::new(credential))
        .with_readiness_identity(&args.selected_version, args.selector_generation)
        .with_access_log(Arc::clone(&access_log) as Arc<dyn transport::AccessLogSink>);
    let live_respond: Arc<dyn crate::endpoint_carrier::LiveRespondAuthority> =
        Arc::clone(&process_host) as Arc<dyn crate::endpoint_carrier::LiveRespondAuthority>;
    let assembly =
        ProductionCarrierAssembly::assemble(authority_root, unary, live_respond, config)?;
    let readiness_host = assembly.host();
    let readiness_process_host = Arc::clone(&process_host);
    access_log.install_health_hook(Arc::new(move |healthy| {
        let readiness = if healthy && !readiness_process_host.is_draining() {
            endpoint::HostReadiness::Ready
        } else {
            endpoint::HostReadiness::NotReady
        };
        readiness_host.set_readiness(readiness);
    }));
    process_host.attach_streams(assembly.streams().clone());
    process_host.boot_sweep()?;
    assembly.finish_recovery()?;
    process_host.start_periodic_sweep();
    process_host.start_schedule_timer()?;
    let listener = TcpListener::bind(args.listen).await.map_err(|error| {
        if error.kind() == io::ErrorKind::AddrInUse {
            DaemonError::listener_unavailable(args.listen)
        } else {
            DaemonError::io(error)
        }
    })?;
    let web_listener = if let Some(web_listen) = args.web_listen {
        Some(TcpListener::bind(web_listen).await.map_err(|error| {
            if error.kind() == io::ErrorKind::AddrInUse {
                DaemonError::listener_unavailable(web_listen)
            } else {
                DaemonError::io(error)
            }
        })?)
    } else {
        None
    };
    reporter.listener_bound(launch_id, &args.selection())?;

    emit_log(
        &log,
        LogRecord {
            v: 1,
            ts: system_timestamp().map_err(DaemonError::protocol)?,
            severity: Severity::Info,
            component: "supervisor".to_owned(),
            build: attribution.build.clone(),
            code: "boot-recovery-complete".to_owned(),
            message: "Supervisor boot recovery completed".to_owned(),
            correlation: Correlation::from(&attribution),
            fields: [
                ("generation".to_owned(), args.selector_generation.into()),
                ("version".to_owned(), args.selected_version.clone().into()),
            ]
            .into_iter()
            .collect(),
        },
    )?;
    publish_metric_snapshot(
        &log_root.join("metrics.canonical.json"),
        &metrics.snapshot(system_timestamp().map_err(DaemonError::protocol)?, true),
    )
    .map_err(|error| DaemonError::io(io::Error::other(error.to_string())))?;

    let periodic_metrics = Arc::clone(&metrics);
    let periodic_access_log = Arc::clone(&access_log);
    let periodic_metric_path = log_root.join("metrics.canonical.json");
    let metric_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let Ok(ts) = system_timestamp() else {
                continue;
            };
            let _ = publish_metric_snapshot(
                &periodic_metric_path,
                &periodic_metrics.snapshot(ts, !periodic_access_log.is_faulted()),
            );
        }
    });

    let host = assembly.host();
    let server = assembly.into_server();
    let handle = server.handle();
    let web_service = args
        .web_listen
        .map(|bind| server.web_client(WebClientConfig::loopback(bind)))
        .transpose()
        .map_err(DaemonError::from)?;
    let web_enabled = web_service.is_some();
    install_termination_handler()?;
    let mut lifetime_task = tokio::task::spawn_blocking(move || watch_shutdown(launcher_lifetime));
    let serve = server.serve(listener);
    tokio::pin!(serve);
    let web_serve = async move {
        match (web_service, web_listener) {
            (Some(service), Some(listener)) => service.serve(listener).await,
            (None, None) => std::future::pending().await,
            _ => unreachable!("Web Client service and listener are paired"),
        }
    };
    tokio::pin!(web_serve);

    enum CompletedServe {
        Native(Result<(), transport::TransportConfigError>),
        Web(Result<(), transport::TransportConfigError>),
        Lifetime,
    }
    let completed_serve = tokio::select! {
        result = &mut serve => {
            lifetime_task.abort();
            CompletedServe::Native(result)
        }
        result = &mut web_serve => {
            lifetime_task.abort();
            CompletedServe::Web(result)
        }
        result = &mut lifetime_task => {
            result.map_err(|error| DaemonError::io(io::Error::other(error.to_string())))??;
            CompletedServe::Lifetime
        }
    };

    metric_task.abort();
    let _ = metric_task.await;
    host.set_readiness(endpoint::HostReadiness::NotReady);
    handle.begin_drain();
    process_host.shutdown();
    emit_log(
        &log,
        LogRecord {
            v: 1,
            ts: system_timestamp().map_err(DaemonError::protocol)?,
            severity: Severity::Info,
            component: "supervisor".to_owned(),
            build: attribution.build.clone(),
            code: "server-draining".to_owned(),
            message: "Endpoint is draining".to_owned(),
            correlation: Correlation::from(&attribution),
            fields: [("deadline_seconds".to_owned(), DRAIN_DEADLINE_SECONDS.into())]
                .into_iter()
                .collect(),
        },
    )?;
    publish_metric_snapshot(
        &log_root.join("metrics.canonical.json"),
        &metrics.snapshot(system_timestamp().map_err(DaemonError::protocol)?, false),
    )
    .map_err(|error| DaemonError::io(io::Error::other(error.to_string())))?;
    match completed_serve {
        CompletedServe::Native(result) => {
            result.map_err(DaemonError::from)?;
            if web_enabled {
                web_serve.await.map_err(DaemonError::from)
            } else {
                Ok(())
            }
        }
        CompletedServe::Web(result) => {
            result.map_err(DaemonError::from)?;
            serve.await.map_err(DaemonError::from)
        }
        CompletedServe::Lifetime => {
            if web_enabled {
                let (native, web) = tokio::join!(serve, web_serve);
                native.map_err(DaemonError::from)?;
                web.map_err(DaemonError::from)
            } else {
                serve.await.map_err(DaemonError::from)
            }
        }
    }
}

fn production_worker_binary(supervisor: &Path) -> Result<PathBuf, DaemonError> {
    let macos = supervisor
        .parent()
        .filter(|path| path.ends_with("Contents/MacOS"));
    let app = macos
        .and_then(Path::parent)
        .and_then(Path::parent)
        .filter(|path| {
            path.file_name()
                .is_some_and(|name| name == "TekesKernelSupervisor.app")
        });
    let bundle = app
        .and_then(Path::parent)
        .filter(|path| path.file_name().is_some_and(|name| name == "apps"))
        .and_then(Path::parent)
        .ok_or_else(|| {
            DaemonError::invalid_install_reason(supervisor, "supervisor bundle layout is invalid")
        })?;
    Ok(bundle.join("bin/tekes-worker"))
}

/// Builds the unary host used by the production daemon. Resource methods are
/// captured once from user instructions. Workspace/project resources are
/// captured independently per workspace, so unrelated workspaces cannot make
/// daemon startup fail merely because they use the same resource name.
pub fn assemble_production_endpoint_host(
    authority_root: &Path,
    description: SessionHostDescription,
    clock: Arc<EndpointClock>,
    user_agent_dir: &Path,
    process_host: Arc<ProductionProcessHost>,
) -> Result<ProductionEndpointHost, DaemonError> {
    assemble_endpoint_host(
        authority_root,
        description,
        clock,
        user_agent_dir,
        process_host,
        false,
    )
}

pub fn assemble_application_endpoint_host(
    authority_root: &Path,
    description: SessionHostDescription,
    clock: Arc<EndpointClock>,
    user_agent_dir: &Path,
    process_host: Arc<ProductionProcessHost>,
) -> Result<ProductionEndpointHost, DaemonError> {
    assemble_endpoint_host(
        authority_root,
        description,
        clock,
        user_agent_dir,
        process_host,
        true,
    )
}

fn assemble_endpoint_host(
    authority_root: &Path,
    description: SessionHostDescription,
    clock: Arc<EndpointClock>,
    user_agent_dir: &Path,
    process_host: Arc<ProductionProcessHost>,
    application_owned: bool,
) -> Result<ProductionEndpointHost, DaemonError> {
    let repository = ConfigRepository::open(authority_root)
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
    let instruction = InstructionResolver::new(user_agent_dir, std::iter::empty::<&Path>())
        .capture()
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
    let catalog = ResourceCatalog::from_snapshot(&instruction)
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
    let workspace_resources = repository
        .workspaces()
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?
        .into_iter()
        .map(|workspace| {
            let workspace_id = workspace.id.clone();
            let authored_roots = workspace
                .folder_paths()
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>();

            // A workspace may refer to a removable volume, a deleted temporary
            // checkout, or another path that is not mounted at daemon startup.
            // It remains inventory authority, but it cannot contribute project
            // resources until its roots are available again. Do not let that
            // workspace prevent the host from binding its endpoint.
            if authored_roots.iter().any(|root| !root.is_dir()) {
                return Ok((workspace_id, ResourceCatalog::default()));
            }

            let resolved = match repository.resolve(&workspace_id) {
                Ok(snapshot) => snapshot.workspace.cwd,
                Err(_) if authored_roots.iter().any(|root| !root.is_dir()) => {
                    return Ok((workspace_id, ResourceCatalog::default()));
                }
                Err(error) => return Err(DaemonError::invalid_config(error.to_string())),
            };
            let snapshot = match InstructionResolver::new_scoped(
                user_agent_dir,
                authority_root.join("workspaces").join(&workspace_id),
                &resolved,
            )
            .capture()
            {
                Ok(snapshot) => snapshot,
                Err(_) if authored_roots.iter().any(|root| !root.is_dir()) => {
                    return Ok((workspace_id, ResourceCatalog::default()));
                }
                Err(error) => return Err(DaemonError::invalid_config(error.to_string())),
            };
            let catalog = ResourceCatalog::from_snapshot(&snapshot)
                .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
            Ok((workspace_id, catalog))
        })
        .collect::<Result<std::collections::BTreeMap<_, _>, DaemonError>>()?;
    let command_clock = Arc::clone(&clock);
    let input_admission = Arc::new(SessionInputAdmissionAuthority::new(
        authority_root.to_path_buf(),
    ));
    let extensions = Arc::new(
        ProductionClientExtensions::open(
            authority_root,
            catalog,
            workspace_resources,
            EndpointCommandInputAuthority::new(
                Arc::clone(&process_host) as Arc<dyn SessionDeliveryAuthority>,
                Arc::new(move || command_clock().map_err(|error| error.to_string())),
                Arc::clone(&input_admission),
            ),
            Arc::clone(&process_host),
        )
        .map_err(|error| DaemonError::corrupt(format!("{}: {}", error.code, error.message)))?,
    );
    let admin_routes = Arc::new(
        (if application_owned {
            ClientAdminRoutes::for_application(authority_root, Arc::clone(&process_host))
        } else {
            ClientAdminRoutes::new(authority_root, Arc::clone(&process_host))
        })
        .map_err(|error| DaemonError::invalid_config(error.code))?,
    );
    let mut extension_routes = extensions.routes();
    extension_routes.extend(admin_routes.routes());
    let workspace_helper = process_host.workspace_service_binary();
    if workspace_helper.is_file() {
        extension_routes.push(Arc::new(
            crate::workspace_routes::WorkspaceRoutes::open(authority_root, workspace_helper)
                .map_err(|error| DaemonError::invalid_config(error.message))?,
        ));
    }
    let routes = CompositeProductionEndpointRoutes::compose(extension_routes)
        .map_err(|error| DaemonError::corrupt(error.to_string()))?;
    ProductionEndpointHost::open_with_full_authorities_and_session_admission(
        authority_root,
        description,
        clock,
        Some(Arc::new(routes)),
        Some(Arc::clone(&process_host) as Arc<dyn ProviderReadinessAuthority>),
        Some(Arc::clone(&process_host) as Arc<dyn SessionDeliveryAuthority>),
        Some(process_host as Arc<dyn QueueTransactionAuthority>),
        input_admission,
    )
    .map_err(|error| DaemonError::corrupt(error.to_string()))
}

fn validate_handoff(args: &DaemonArgs) -> Result<(), DaemonError> {
    let embedded_build = option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
    if args.selected_version != embedded_build {
        return Err(DaemonError::selector_mismatch(
            embedded_build,
            &args.selected_version,
        ));
    }
    if args.authority_registry_sha256 != AUTHORITY_REGISTRY_SHA256 {
        return Err(DaemonError::selector_mismatch(
            AUTHORITY_REGISTRY_SHA256,
            &args.authority_registry_sha256,
        ));
    }
    Ok(())
}

fn validate_executable(identity: &InstallIdentity) -> Result<(), DaemonError> {
    let path = std::env::current_exe().map_err(DaemonError::io)?;
    let metadata = fs::symlink_metadata(&path)
        .map_err(|error| DaemonError::invalid_install(path.clone(), error))?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file()
        || metadata.uid() != effective_uid()
        || metadata.mode() & 0o022 != 0
        || metadata.mode() & 0o111 == 0
    {
        return Err(DaemonError::invalid_install_reason(
            &path,
            "executable ownership, mode or type mismatch",
        ));
    }
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    {
        let verification = std::process::Command::new("/usr/bin/codesign")
            .args(["--verify", "--strict"])
            .arg(&path)
            .status()
            .map_err(DaemonError::io)?;
        let requirement_verification = std::process::Command::new("/usr/bin/codesign")
            .args(["--verify", "--strict", "-R"])
            .arg(format!("={}", identity.supervisor_requirement))
            .arg(&path)
            .status()
            .map_err(DaemonError::io)?;
        let evidence = std::process::Command::new("/usr/bin/codesign")
            .args(["-d", "-r-", "-vv"])
            .arg(&path)
            .output()
            .map_err(DaemonError::io)?;
        let evidence = String::from_utf8_lossy(&evidence.stderr);
        let entitlements = std::process::Command::new("/usr/bin/codesign")
            .args(["-d", "--entitlements", ":-"])
            .arg(&path)
            .output()
            .map_err(DaemonError::io)?;
        let entitlement_bytes = if entitlements.stdout.is_empty() {
            &entitlements.stderr
        } else {
            &entitlements.stdout
        };
        let entitlements = String::from_utf8_lossy(entitlement_bytes);
        let provider_secret_group =
            format!("{}.com.tekes.kernel.provider-secrets", identity.team_id);
        if !verification.success()
            || !requirement_verification.success()
            || !evidence.contains(&format!("TeamIdentifier={}", identity.team_id))
            || !entitlements.contains(&format!("<string>{}</string>", identity.access_group))
            || !entitlements.contains(&format!("<string>{provider_secret_group}</string>"))
        {
            return Err(DaemonError::invalid_install_reason(
                &path,
                "code signature identity verification failed",
            ));
        }
    }
    #[cfg(any(not(target_os = "macos"), debug_assertions))]
    let _ = identity;
    Ok(())
}

fn watch_shutdown(fd: RawFd) -> Result<(), DaemonError> {
    // SAFETY: fd is a private duplicate owned by this blocking task.
    let mut file = unsafe { File::from_raw_fd(fd) };
    let mut poll_descriptor = libc::pollfd {
        fd: file.as_raw_fd(),
        events: libc::POLLIN | libc::POLLHUP,
        revents: 0,
    };
    let mut buffer = [0_u8; 64];
    loop {
        if TERMINATE_REQUESTED.load(Ordering::Acquire) {
            return Ok(());
        }
        // SAFETY: poll_descriptor points to one initialized live descriptor.
        let polled = unsafe { libc::poll(&mut poll_descriptor, 1, 100) };
        if polled == 0 {
            continue;
        }
        if polled < 0 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(DaemonError::io(error));
        }
        match file.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(_) => continue,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(DaemonError::io(error)),
        }
    }
}

/// Test/deployment-harness seam for the exact launcher-lifetime EOF contract.
/// The caller retains ownership of `fd`; this function watches a CLOEXEC
/// duplicate and returns only after EOF or SIGTERM.
pub fn wait_for_launcher_shutdown(fd: RawFd) -> Result<(), DaemonError> {
    let duplicated = duplicate_fd(fd).map_err(DaemonError::io)?;
    watch_shutdown(duplicated)
}

extern "C" fn request_termination(_signal: libc::c_int) {
    TERMINATE_REQUESTED.store(true, Ordering::Release);
}

pub(crate) fn install_termination_handler() -> Result<(), DaemonError> {
    TERMINATE_REQUESTED.store(false, Ordering::Release);
    // SAFETY: zero initialization is valid for sigaction and sigemptyset
    // initializes its mask before installation. The handler only stores to a
    // lock-free process-global flag observed by the shutdown watcher.
    unsafe {
        let mut action = std::mem::zeroed::<libc::sigaction>();
        action.sa_sigaction = request_termination as usize;
        action.sa_flags = 0;
        libc::sigemptyset(&mut action.sa_mask);
        if libc::sigaction(libc::SIGTERM, &action, std::ptr::null_mut()) != 0 {
            return Err(DaemonError::io(io::Error::last_os_error()));
        }
    }
    Ok(())
}

fn duplicate_fd(fd: RawFd) -> io::Result<RawFd> {
    loop {
        // SAFETY: dup copies a caller-supplied live descriptor and owns no pointers.
        let duplicated = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 5) };
        if duplicated >= 0 {
            return Ok(duplicated);
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}

fn take_inherited_fd(fd: RawFd) -> io::Result<RawFd> {
    let duplicated = duplicate_fd(fd)?;
    // SAFETY: this production boundary consumes the inherited descriptor;
    // the CLOEXEC duplicate is now its sole owner.
    if unsafe { libc::close(fd) } == 0 {
        Ok(duplicated)
    } else {
        let error = io::Error::last_os_error();
        // SAFETY: close the duplicate on the error path to avoid leaking it.
        let _ = unsafe { libc::close(duplicated) };
        Err(error)
    }
}

fn launch_attempt(launch_id: &str) -> u64 {
    launch_id
        .split('-')
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(1)
}

fn user_home() -> Result<PathBuf, DaemonError> {
    // getpwuid_r needs a caller-owned buffer and copies the stable result there.
    let uid = effective_uid();
    let mut pwd = unsafe { std::mem::zeroed::<libc::passwd>() };
    let mut result = std::ptr::null_mut();
    let mut buffer = vec![0_u8; 16 * 1024];
    // SAFETY: pointers reference live writable storage for the duration of the call.
    let status = unsafe {
        libc::getpwuid_r(
            uid,
            &mut pwd,
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            &mut result,
        )
    };
    if status != 0 || result.is_null() || pwd.pw_dir.is_null() {
        return Err(DaemonError::invalid_install_reason(
            Path::new("/etc/passwd"),
            "target user home is unavailable",
        ));
    }
    // SAFETY: successful getpwuid_r returns a NUL-terminated string in buffer.
    let bytes = unsafe { std::ffi::CStr::from_ptr(pwd.pw_dir) }.to_bytes();
    use std::os::unix::ffi::OsStrExt as _;
    let home = PathBuf::from(OsStr::from_bytes(bytes));
    if !home.is_absolute() {
        return Err(DaemonError::invalid_install_reason(
            &home,
            "target user home is not absolute",
        ));
    }
    Ok(home)
}

pub fn system_timestamp() -> Result<String, String> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    let seconds = duration.as_secs();
    let days = i64::try_from(seconds / 86_400).map_err(|error| error.to_string())? + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    let day_seconds = seconds % 86_400;
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        day_seconds / 3_600,
        day_seconds % 3_600 / 60,
        day_seconds % 60,
        duration.subsec_millis(),
    ))
}

fn emit_log(log: &RotatingJsonlLog, record: LogRecord) -> Result<(), DaemonError> {
    log.append(&record)
        .map_err(|error| DaemonError::io(io::Error::other(error.to_string())))?;
    if let Ok(bytes) = record.canonical_line() {
        let _ = io::stderr().write_all(&bytes);
    }
    Ok(())
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct DaemonError {
    code: &'static str,
    message: String,
    exit: u8,
}

impl DaemonError {
    fn new(code: &'static str, message: impl Into<String>, exit: u8) -> Self {
        Self {
            code,
            message: message.into(),
            exit,
        }
    }

    fn usage(message: impl Into<String>) -> Self {
        Self::new("protocol-mismatch", message, 66)
    }

    pub(crate) fn protocol(message: impl Into<String>) -> Self {
        Self::new("protocol-mismatch", message, 66)
    }

    pub(crate) fn io(error: io::Error) -> Self {
        Self::new("io", error.to_string(), 74)
    }

    pub(crate) fn store(error: StoreError) -> Self {
        match error {
            StoreError::Busy => Self::new("already-running", error.to_string(), 66),
            StoreError::VersionGate { .. } => Self::new("version-gate", error.to_string(), 76),
            StoreError::UnsupportedFilesystem(_) => {
                Self::new("unsupported-filesystem", error.to_string(), 66)
            }
            StoreError::Corruption(_)
            | StoreError::Schema(_)
            | StoreError::AssetDigest { .. }
            | StoreError::RewriteCarrierCorrupt(_)
            | StoreError::RewriteOperationCorrupt(_) => {
                Self::new("corrupt-ledger", error.to_string(), 75)
            }
            StoreError::Io(error) => Self::io(error),
            other => Self::new("corrupt-ledger", other.to_string(), 75),
        }
    }

    fn already_running(root: &Path) -> Self {
        Self::new(
            "already-running",
            format!("storage root is already owned: {}", root.display()),
            66,
        )
    }

    pub(crate) fn invalid_install(path: PathBuf, error: io::Error) -> Self {
        Self::new(
            "invalid-install",
            format!("{}: {error}", path.display()),
            66,
        )
    }

    pub(crate) fn invalid_install_reason(path: &Path, reason: &str) -> Self {
        Self::new(
            "invalid-install",
            format!("{}: {reason}", path.display()),
            66,
        )
    }

    pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
        Self::new("invalid-config", message, 66)
    }

    pub(crate) fn corrupt(message: impl Into<String>) -> Self {
        Self::new("corrupt-ledger", message, 75)
    }

    pub(crate) fn required_broker(message: impl Into<String>) -> Self {
        Self::new("required-broker-unavailable", message, 66)
    }

    fn credential(message: impl Into<String>) -> Self {
        Self::new("endpoint-credential-unavailable", message, 66)
    }

    fn listener_unavailable(address: SocketAddr) -> Self {
        Self::new(
            "listener-unavailable",
            format!("listener {address} is unavailable"),
            66,
        )
    }

    fn selector_mismatch(expected: &str, actual: &str) -> Self {
        Self::new(
            "selector-mismatch",
            format!("selector handoff mismatch: expected {expected}, actual {actual}"),
            66,
        )
    }

    #[must_use]
    pub const fn bootstrap_code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        self.exit
    }
}

impl From<ProductionCarrierError> for DaemonError {
    fn from(error: ProductionCarrierError) -> Self {
        match error {
            ProductionCarrierError::Io(error) => Self::io(error),
            ProductionCarrierError::Store(error) => Self::store(error),
            other => Self::corrupt(other.to_string()),
        }
    }
}

impl From<transport::TransportConfigError> for DaemonError {
    fn from(error: transport::TransportConfigError) -> Self {
        match error {
            transport::TransportConfigError::Io(error) => Self::io(error),
            other => Self::protocol(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
    use std::os::unix::fs::symlink;
    use std::process::Command;

    use super::{
        BootstrapReporter, production_worker_binary, reject_cloud_managed_storage,
        require_production_filesystem_name, system_timestamp,
    };

    #[test]
    fn production_worker_path_is_derived_from_the_frozen_bundle_layout() {
        let supervisor = std::path::Path::new(
            "/install/bundles/v/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
        );
        assert_eq!(
            production_worker_binary(supervisor).expect("frozen deployment layout"),
            std::path::Path::new("/install/bundles/v/bin/tekes-worker")
        );
        assert!(production_worker_binary(std::path::Path::new("/tmp/tekes-supervisor")).is_err());
    }

    #[test]
    fn production_daemon_accepts_apfs_and_rejects_hfs_seam() {
        require_production_filesystem_name("apfs").expect("APFS production volume");
        let error = require_production_filesystem_name("hfs")
            .expect_err("HFS must remain a generic-store capability, not production deployment");
        assert_eq!(error.bootstrap_code(), "unsupported-filesystem");
        assert!(error.to_string().contains("requires local APFS"));
    }

    #[test]
    fn production_storage_rejects_mobile_documents_and_an_ancestor_symlink_alias() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let cloud = directory
            .path()
            .join("Library")
            .join("Mobile Documents")
            .join("Tekes")
            .join("threads");
        fs::create_dir_all(&cloud).expect("cloud path fixture");
        let direct = reject_cloud_managed_storage(&cloud)
            .expect_err("Mobile Documents path must be rejected before a write probe");
        assert_eq!(direct.bootstrap_code(), "unsupported-filesystem");

        let alias = directory.path().join("storage-alias");
        symlink(
            directory.path().join("Library").join("Mobile Documents"),
            &alias,
        )
        .expect("ancestor alias");
        let aliased_storage = alias.join("Tekes").join("threads");
        let aliased = reject_cloud_managed_storage(&aliased_storage)
            .expect_err("realpath must expose a cloud-managed ancestor alias");
        assert_eq!(aliased.bootstrap_code(), "unsupported-filesystem");
    }

    #[test]
    fn production_timestamp_is_event_millisecond_utc() {
        let timestamp = system_timestamp().expect("system timestamp");
        schema::Event::decode(
            &serde_json::to_vec(&serde_json::json!({
                "v":1,"seq":1,"kind":"stop_requested","ts":timestamp,
                "generation":1,"origin_key":"clock-test",
                "origin_tuple":{"principal":"test","client":"test","target":"test",
                    "op":"cancel","key":"clock-test"}
            }))
            .expect("timestamp fixture JSON"),
        )
        .expect("production timestamp must satisfy event");
    }

    #[test]
    fn inherited_bootstrap_fd_is_consumed_and_never_survives_exec() {
        let mut descriptors = [-1; 2];
        // SAFETY: descriptors points to two writable integers.
        assert_eq!(unsafe { libc::pipe(descriptors.as_mut_ptr()) }, 0);
        // SAFETY: successful pipe returned two new owned descriptors.
        let read = unsafe { std::os::fd::OwnedFd::from_raw_fd(descriptors[0]) };
        // SAFETY: successful pipe returned two new owned descriptors.
        let write = unsafe { std::os::fd::OwnedFd::from_raw_fd(descriptors[1]) };
        let inherited = write.into_raw_fd();
        let reporter = BootstrapReporter::from_inherited_fd(inherited).expect("take inherited fd");
        assert_eq!(unsafe { libc::fcntl(inherited, libc::F_GETFD) }, -1);
        let owned = reporter.file.as_ref().expect("open reporter").as_raw_fd();
        assert_ne!(
            unsafe { libc::fcntl(owned, libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        let status = Command::new("/bin/sh")
            .arg("-c")
            .arg("test ! -e \"/dev/fd/$CHECK_FD\"")
            .env("CHECK_FD", owned.to_string())
            .status()
            .expect("exec descriptor probe");
        assert!(status.success(), "CLOEXEC descriptor leaked through exec");
        drop(reporter);
        drop(read);
    }
}
