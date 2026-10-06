//! Runtime shared by the application-owned launch (`--built-in`): endpoint
//! token decoding, the storage root lock and preflight, endpoint host
//! assembly, the launcher-lifetime watcher and the host error type.

#[cfg(target_os = "macos")]
use std::ffi::CString;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
#[cfg(target_os = "macos")]
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use endpoint::SessionHostDescription;
use profile::{ConfigRepository, InstructionResolver, ResourceCatalog};
use store::{LockedLedger, StoreError, ThreadStore, scan_valid_prefix};
use thiserror::Error;

use crate::client_admin::ClientAdminRoutes;
use crate::client_extensions::ProductionClientExtensions;
use crate::endpoint_carrier::ProductionCarrierError;
use crate::endpoint_host::{
    CompositeProductionEndpointRoutes, EndpointClock, ProductionEndpointHost,
    ProviderReadinessAuthority, QueueTransactionAuthority, SessionDeliveryAuthority,
    SessionInputAdmissionAuthority,
};
use crate::process_host::ProductionProcessHost;
use crate::resource_capability::EndpointCommandInputAuthority;

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

static TERMINATE_REQUESTED: AtomicBool = AtomicBool::new(false);

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

pub fn prepare_storage(storage_root: &Path) -> Result<ThreadStore, DaemonError> {
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

/// Runs the storage preflight without keeping the store open: the same
/// no-symlink ownership checks, local-filesystem probe, recovery and semantic
/// sweep as startup. It does not acquire the root lock.
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

/// Other platforms rely on the local-filesystem probe in `prepare_storage`.
#[cfg(not(target_os = "macos"))]
fn require_production_apfs(_root: &Path) -> Result<(), DaemonError> {
    Ok(())
}

#[cfg_attr(not(any(target_os = "macos", test)), allow(dead_code))]
fn require_production_filesystem_name(name: &str) -> Result<(), DaemonError> {
    if name == "apfs" {
        Ok(())
    } else {
        Err(DaemonError::store(StoreError::UnsupportedFilesystem(
            format!("storage on macOS requires local APFS; found {name}"),
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

/// Builds the unary host for application-owned launch. Resource methods are
/// captured once from user instructions. Workspace/project resources are
/// captured independently per workspace, so unrelated workspaces cannot make
/// startup fail merely because they use the same resource name.
pub fn assemble_application_endpoint_host(
    authority_root: &Path,
    description: SessionHostDescription,
    clock: Arc<EndpointClock>,
    user_agent_dir: &Path,
    process_host: Arc<ProductionProcessHost>,
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
            // checkout, or another path that is not mounted at startup.
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
        ClientAdminRoutes::new(authority_root, Arc::clone(&process_host))
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

#[derive(Debug, Error)]
#[error("{message}")]
pub struct DaemonError {
    code: &'static str,
    message: String,
}

impl DaemonError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub(crate) fn protocol(message: impl Into<String>) -> Self {
        Self::new("protocol-mismatch", message)
    }

    pub(crate) fn io(error: io::Error) -> Self {
        Self::new("io", error.to_string())
    }

    pub(crate) fn store(error: StoreError) -> Self {
        match error {
            StoreError::Busy => Self::new("already-running", error.to_string()),
            StoreError::VersionGate { .. } => Self::new("version-gate", error.to_string()),
            StoreError::UnsupportedFilesystem(_) => {
                Self::new("unsupported-filesystem", error.to_string())
            }
            StoreError::Corruption(_)
            | StoreError::Schema(_)
            | StoreError::AssetDigest { .. }
            | StoreError::RewriteCarrierCorrupt(_)
            | StoreError::RewriteOperationCorrupt(_) => {
                Self::new("corrupt-ledger", error.to_string())
            }
            StoreError::Io(error) => Self::io(error),
            other => Self::new("corrupt-ledger", other.to_string()),
        }
    }

    fn already_running(root: &Path) -> Self {
        Self::new(
            "already-running",
            format!("storage root is already owned: {}", root.display()),
        )
    }

    pub(crate) fn invalid_install(path: PathBuf, error: io::Error) -> Self {
        Self::new("invalid-install", format!("{}: {error}", path.display()))
    }

    pub(crate) fn invalid_install_reason(path: &Path, reason: &str) -> Self {
        Self::new("invalid-install", format!("{}: {reason}", path.display()))
    }

    pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
        Self::new("invalid-config", message)
    }

    pub(crate) fn corrupt(message: impl Into<String>) -> Self {
        Self::new("corrupt-ledger", message)
    }

    pub(crate) fn required_broker(message: impl Into<String>) -> Self {
        Self::new("required-broker-unavailable", message)
    }

    fn credential(message: impl Into<String>) -> Self {
        Self::new("endpoint-credential-unavailable", message)
    }

    #[must_use]
    pub const fn bootstrap_code(&self) -> &'static str {
        self.code
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
    use std::os::unix::fs::symlink;

    use super::{
        reject_cloud_managed_storage, require_production_filesystem_name, system_timestamp,
    };

    #[test]
    fn macos_storage_accepts_apfs_and_rejects_hfs() {
        require_production_filesystem_name("apfs").expect("APFS production volume");
        let error = require_production_filesystem_name("hfs")
            .expect_err("HFS is not accepted for application storage on macOS");
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
}
