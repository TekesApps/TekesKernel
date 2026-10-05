//! Supervisor-owned worker launch boundary.

pub mod builtin;
pub mod client_admin;
pub mod client_extensions;
mod context_usage;
pub mod continuation_journal;
pub mod daemon;
pub mod dynamic_bindings;
pub mod endpoint_carrier;
pub mod endpoint_host;
pub mod file_leases;
pub mod file_observation;
pub mod mcp_continuation;
pub mod mcp_runtime;
pub mod observability;
pub mod process_host;
pub mod production_tool_control;
pub mod resource_capability;
pub mod tool_control;
pub mod workspace_routes;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use profile::{
    ConfigRepository, ConfigSnapshot, DynamicToolCatalog, InstructionSnapshot, LaunchBindings,
    LaunchProfile, ProfileError,
};
use provider::{
    CredentialBroker, CredentialBrokerControl, CredentialScope, ResolvedCredentialBindings,
    RevokedCredentialScope, SecretStore, resolve_config_credentials, start_credential_channel,
};
use worker_control::{LaunchChild, LaunchResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildLaunchState {
    Running,
    Settled,
    Parked,
}

/// Idempotent `(child, spawn_id)` launch classification used by the process
/// host after it has inspected the durable child genesis and tail state.
#[derive(Clone, Debug, Default)]
pub struct ChildLaunchRegistry {
    children: BTreeMap<String, (String, ChildLaunchState)>,
}

impl ChildLaunchRegistry {
    pub fn record(
        &mut self,
        child: impl Into<String>,
        spawn_id: impl Into<String>,
        state: ChildLaunchState,
    ) {
        self.children.insert(child.into(), (spawn_id.into(), state));
    }

    #[must_use]
    pub fn resolve(&self, request: &LaunchChild) -> LaunchResult {
        match self.children.get(&request.child) {
            None => launch_failure(request, "missing"),
            Some((spawn_id, _)) if spawn_id != &request.spawn_id => {
                launch_failure(request, "spawn_mismatch")
            }
            Some(_) => LaunchResult {
                child: request.child.clone(),
                spawn_id: request.spawn_id.clone(),
                ok: true,
                error: None,
            },
        }
    }
}

fn launch_failure(request: &LaunchChild, error: &str) -> LaunchResult {
    LaunchResult {
        child: request.child.clone(),
        spawn_id: request.spawn_id.clone(),
        ok: false,
        error: Some(error.to_owned()),
    }
}

#[derive(Clone, Debug)]
pub struct WorkerLaunchSpec {
    pub binary: PathBuf,
    pub ledger: PathBuf,
    pub timestamp: String,
    pub run_id: String,
    pub binary_attribution: String,
    pub assets: PathBuf,
    pub config_digest: String,
    pub instruction_digest: String,
}

#[derive(Clone, Debug)]
pub struct ProfiledWorkerLaunchSpec {
    pub binary: PathBuf,
    pub ledger: PathBuf,
    pub timestamp: String,
    pub run_id: String,
    pub binary_attribution: String,
    pub workspace_id: String,
    /// Stable binding copied from the session genesis. Legacy sessions may
    /// leave this absent and retain primary-folder launch behavior.
    pub folder_binding: Option<String>,
    pub user_agent_dir: PathBuf,
}

pub struct WorkerLaunch {
    pub child: Child,
    pub config_snapshot: ConfigSnapshot,
    pub instruction_snapshot: InstructionSnapshot,
    pub config_digest: String,
    pub instruction_digest: String,
    pub launch_bindings_digest: String,
    pub credential_bindings: Option<ResolvedCredentialBindings>,
    pub credential_control: Option<CredentialBrokerControl>,
    pub credential_broker: Option<std::thread::JoinHandle<Result<(), String>>>,
}

#[derive(Clone, Debug)]
pub struct WorkerLaunchBindings {
    pub goal_id: Option<String>,
    pub dynamic_catalog: DynamicToolCatalog,
}

type WorkerLaunchBindingResolver<'a> =
    dyn FnMut(&ConfigSnapshot, &InstructionSnapshot) -> Result<WorkerLaunchBindings, String> + 'a;

impl Default for WorkerLaunchBindings {
    fn default() -> Self {
        Self {
            goal_id: None,
            dynamic_catalog: DynamicToolCatalog {
                format: 1,
                tools: Vec::new(),
            },
        }
    }
}

pub fn launch_profiled_worker_with_bindings(
    repository: &ConfigRepository,
    spec: &ProfiledWorkerLaunchSpec,
    bindings: WorkerLaunchBindings,
) -> Result<WorkerLaunch, LaunchError> {
    launch_profiled_worker_inner(repository, spec, None, None, Some(bindings), None, None)
}

pub fn launch_profiled_worker_with_credentials(
    repository: &ConfigRepository,
    spec: &ProfiledWorkerLaunchSpec,
    scopes: Vec<CredentialScope>,
) -> Result<WorkerLaunch, LaunchError> {
    launch_profiled_worker_inner(repository, spec, Some(scopes), None, None, None, None)
}

/// Hermetic debug-only transport seam. Exact target resolution, request bytes,
/// and request digest still use the proved route; only the already-prepared
/// HTTP destination is redirected to a loopback test server.
#[doc(hidden)]
pub fn launch_profiled_worker_with_credentials_and_provider_test_redirect(
    repository: &ConfigRepository,
    spec: &ProfiledWorkerLaunchSpec,
    scopes: Vec<CredentialScope>,
    redirect: &str,
) -> Result<WorkerLaunch, LaunchError> {
    launch_profiled_worker_inner(
        repository,
        spec,
        Some(scopes),
        None,
        None,
        None,
        Some(redirect),
    )
}

/// Resolves launch bindings from the exact immutable config and instruction
/// snapshots that are published for this worker. The callback must not reopen
/// mutable config files or manufacture a goal id without a host goal
/// authority.
pub fn launch_profiled_worker_with_secret_store_and_binding_resolver<F>(
    repository: &ConfigRepository,
    spec: &ProfiledWorkerLaunchSpec,
    secret_store: &dyn SecretStore,
    mut resolver: F,
) -> Result<WorkerLaunch, LaunchError>
where
    F: FnMut(&ConfigSnapshot, &InstructionSnapshot) -> Result<WorkerLaunchBindings, String>,
{
    launch_profiled_worker_inner(
        repository,
        spec,
        None,
        Some(secret_store),
        None,
        Some(&mut resolver),
        None,
    )
}

fn launch_profiled_worker_inner(
    repository: &ConfigRepository,
    spec: &ProfiledWorkerLaunchSpec,
    scopes: Option<Vec<CredentialScope>>,
    secret_store: Option<&dyn SecretStore>,
    bindings: Option<WorkerLaunchBindings>,
    binding_resolver: Option<&mut WorkerLaunchBindingResolver<'_>>,
    provider_test_redirect: Option<&str>,
) -> Result<WorkerLaunch, LaunchError> {
    let folder = spec
        .ledger
        .parent()
        .ok_or(LaunchError::LedgerWithoutFolder)?;
    let assets = store::AssetStore::new(folder.join("assets"))?;
    let profile = LaunchProfile::resolve_and_publish_for_binding(
        repository,
        &spec.workspace_id,
        &spec.user_agent_dir,
        &assets,
        spec.folder_binding.as_deref(),
    )?;
    let bindings = match (bindings, binding_resolver) {
        (Some(bindings), None) => bindings,
        (None, Some(resolver)) => resolver(&profile.config, &profile.instruction)
            .map_err(LaunchError::BindingResolution)?,
        (None, None) => WorkerLaunchBindings::default(),
        (Some(_), Some(_)) => {
            return Err(LaunchError::BindingResolution(
                "launch bindings and a binding resolver are mutually exclusive".into(),
            ));
        }
    };
    let launch_bindings =
        LaunchBindings::bind(&profile.config, bindings.goal_id, bindings.dynamic_catalog)?;
    let launch = WorkerLaunchSpec {
        binary: spec.binary.clone(),
        ledger: spec.ledger.clone(),
        timestamp: spec.timestamp.clone(),
        run_id: spec.run_id.clone(),
        binary_attribution: spec.binary_attribution.clone(),
        assets: assets.root().to_path_buf(),
        config_digest: profile.config_digest,
        instruction_digest: profile.instruction_digest,
    };
    let mut resolved_credential_bindings = None;
    let credentials = if let Some(store) = secret_store {
        let bindings = resolve_config_credentials(&profile.config, store)
            .map_err(|_| LaunchError::CredentialBindings)?;
        let credential = if bindings.active.is_empty() && bindings.revoked.is_empty() {
            None
        } else {
            Some((bindings.active.clone(), bindings.revoked.clone()))
        };
        resolved_credential_bindings = Some(bindings);
        credential
    } else {
        scopes.map(|scopes| (scopes, Vec::new()))
    };
    let credential = credentials
        .map(|(scopes, revoked)| {
            let (supervisor, worker) = UnixStream::pair()?;
            Ok::<_, io::Error>((supervisor, worker, scopes, revoked))
        })
        .transpose()?;
    let mut launched = launch_worker_inner(
        &launch,
        credential,
        Some(&launch_bindings),
        provider_test_redirect,
    )?;
    launched.credential_bindings = resolved_credential_bindings;
    Ok(launched)
}

fn launch_worker_inner(
    spec: &WorkerLaunchSpec,
    credential: Option<(
        UnixStream,
        UnixStream,
        Vec<CredentialScope>,
        Vec<RevokedCredentialScope>,
    )>,
    supplied_bindings: Option<&LaunchBindings>,
    provider_test_redirect: Option<&str>,
) -> Result<WorkerLaunch, LaunchError> {
    let config_path = snapshot_path(&spec.assets, &spec.config_digest)?;
    let instruction_path = snapshot_path(&spec.assets, &spec.instruction_digest)?;
    // Validate through independent opens. Reading `/dev/fd/N` here may share
    // the inherited open-file description's offset on Darwin, which would
    // make the worker observe EOF instead of the immutable snapshot.
    let config_bytes = std::fs::read(&config_path)?;
    let instruction_bytes = std::fs::read(&instruction_path)?;
    let config = ConfigSnapshot::decode(&config_bytes)?;
    let instruction = InstructionSnapshot::decode(&instruction_bytes)?;
    instruction.validate_against_config(&config)?;
    if config.digest()? != spec.config_digest {
        return Err(LaunchError::DigestMismatch("config"));
    }
    if instruction.digest()? != spec.instruction_digest {
        return Err(LaunchError::DigestMismatch("instruction"));
    }
    let launch_bindings = match supplied_bindings {
        Some(bindings) => {
            bindings.validate_against(&config)?;
            bindings.clone()
        }
        None => LaunchBindings::bind(
            &config,
            None,
            DynamicToolCatalog {
                format: 1,
                tools: Vec::new(),
            },
        )?,
    };
    let launch_assets = store::AssetStore::new(&spec.assets)?;
    let (launch_bindings_digest, _) = launch_bindings.publish(&launch_assets)?;
    let launch_bindings_path = snapshot_path(&spec.assets, &launch_bindings_digest)?;
    let launch_bindings_bytes = std::fs::read(&launch_bindings_path)?;
    LaunchBindings::decode_verified(&launch_bindings_bytes, &launch_bindings_digest, &config)?;

    let config_file = open_snapshot(&config_path)?;
    let instruction_file = open_snapshot(&instruction_path)?;
    let launch_bindings_file = open_snapshot(&launch_bindings_path)?;

    let config_fd = config_file.as_raw_fd();
    let instruction_fd = instruction_file.as_raw_fd();
    let launch_bindings_fd = launch_bindings_file.as_raw_fd();
    let credential_fd = credential
        .as_ref()
        .map(|(_, worker, _, _)| worker.as_raw_fd());
    let web_search_ready = config.providers.web_search.as_ref().is_some_and(|search| {
        let Ok(origin) = provider::endpoint_origin(&search.endpoint) else {
            return false;
        };
        credential.as_ref().is_some_and(|(_, _, active, _)| {
            active.iter().any(|scope| {
                scope.credential_id == search.credential_key
                    && scope.adapter == search.adapter
                    && scope.endpoint_origin == origin
                    && scope.purpose == "web_search"
            })
        })
    });
    let mut command = closed_command(&spec.binary);
    command
        .arg(&spec.ledger)
        .args(["--timestamp", &spec.timestamp])
        .args(["--run-id", &spec.run_id])
        .args(["--binary", &spec.binary_attribution])
        .args(["--config-fd", &config_fd.to_string()])
        .args(["--instruction-fd", &instruction_fd.to_string()])
        .args(["--launch-bindings-fd", &launch_bindings_fd.to_string()])
        .args(["--launch-bindings-digest", &launch_bindings_digest])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(fd) = credential_fd {
        command.args(["--credential-fd", &fd.to_string()]);
    }
    if web_search_ready {
        command.args(["--web-search-ready", "true"]);
    }
    if let Some(redirect) = provider_test_redirect {
        command.args(["--provider-test-redirect", redirect]);
    }
    for (name, value) in forwarded_worker_environment() {
        command.env(name, value);
    }
    // SAFETY: the closure executes after fork and before exec in the child.
    // It only clears CLOEXEC on the two read-only snapshot descriptors kept
    // live by this stack frame until `spawn` returns. No allocation or lock is
    // performed in the closure.
    unsafe {
        command.pre_exec(move || {
            clear_cloexec(config_fd)?;
            clear_cloexec(instruction_fd)?;
            clear_cloexec(launch_bindings_fd)?;
            if let Some(fd) = credential_fd {
                clear_cloexec(fd)?;
            }
            Ok(())
        });
    }
    let child = command.spawn()?;
    let (credential_control, credential_broker) =
        credential.map_or((None, None), |(supervisor, worker, scopes, revoked)| {
            drop(worker);
            let (control, service) = start_credential_channel(
                supervisor,
                CredentialBroker::with_revoked(scopes, revoked),
            );
            let join = std::thread::spawn(move || {
                service
                    .join()
                    .map_err(|_| "credential broker service panicked".to_owned())?
                    .map_err(|error| error.to_string())
            });
            (Some(control), Some(join))
        });
    Ok(WorkerLaunch {
        child,
        config_snapshot: config,
        instruction_snapshot: instruction,
        config_digest: spec.config_digest.clone(),
        instruction_digest: spec.instruction_digest.clone(),
        launch_bindings_digest,
        credential_bindings: None,
        credential_control,
        credential_broker,
    })
}

fn closed_command(program: &Path) -> Command {
    let mut command = Command::new(program);
    command.env_clear();
    command
}

/// Outbound proxy variables the worker's HTTP clients (provider, web tools)
/// read, in both casings so a value reaches the child exactly as it was set.
const FORWARDED_PROXY_VARIABLES: [&str; 8] = [
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "ALL_PROXY",
    "all_proxy",
    "NO_PROXY",
    "no_proxy",
];

/// The complete environment a worker inherits from the supervisor: the child
/// starts from a cleared environment (no HOME, login keychain, XDG or PATH
/// leakage) and receives only the live evidence capture directory when the
/// supervisor itself was started with one, plus the outbound proxy policy the
/// host launched the supervisor with — transport policy, which the worker's
/// provider and web-tool clients must share with every other program on the
/// machine. Provider credentials never travel this way; they cross the
/// credential broker descriptor.
pub fn forwarded_worker_environment() -> Vec<(String, String)> {
    forwarded_environment_from(|name| std::env::var(name).ok())
}

/// The forwarding rule over an explicit variable lookup, so tests exercise it
/// without mutating the process environment (unsynchronized `set_var` races
/// with every other test thread and with spawned children).
fn forwarded_environment_from(lookup: impl Fn(&str) -> Option<String>) -> Vec<(String, String)> {
    std::iter::once("TEKES_KERNEL_LIVE_ARTIFACT")
        .chain(FORWARDED_PROXY_VARIABLES)
        .filter_map(|name| lookup(name).map(|value| (name.to_owned(), value)))
        .collect()
}

fn snapshot_path(assets: &Path, digest: &str) -> Result<PathBuf, LaunchError> {
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(LaunchError::InvalidDigest);
    }
    Ok(assets.join(format!("sha256-{digest}")))
}

fn open_snapshot(path: &Path) -> Result<File, LaunchError> {
    Ok(OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?)
}

fn clear_cloexec(fd: libc::c_int) -> io::Result<()> {
    // SAFETY: fd names an inherited live descriptor. F_SETFD changes only
    // descriptor flags and does not retain pointers.
    if unsafe { libc::fcntl(fd, libc::F_SETFD, 0) } == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("worker ledger path has no thread folder")]
    LedgerWithoutFolder,
    #[error("snapshot digest is not 64 lowercase hex characters")]
    InvalidDigest,
    #[error("{0} snapshot bytes do not match the requested digest")]
    DigestMismatch(&'static str),
    #[error("credential bindings could not be derived from the frozen config")]
    CredentialBindings,
    #[error("launch bindings could not be derived from the frozen profile: {0}")]
    BindingResolution(String),
    #[error(transparent)]
    Profile(#[from] ProfileError),
    #[error(transparent)]
    Store(#[from] store::StoreError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use super::{closed_command, forwarded_environment_from};

    /// The legacy hermetic/keychain-home environment contract, on the Kernel
    /// side: nothing from the login environment reaches a worker except the
    /// explicit evidence-capture directory and the outbound proxy policy.
    #[test]
    fn worker_environment_forwards_only_the_evidence_capture_directory() {
        assert!(forwarded_environment_from(|_| None).is_empty());
        let login = std::collections::BTreeMap::from([
            ("TEKES_KERNEL_LIVE_ARTIFACT", "/tmp/evidence"),
            ("HTTPS_PROXY", "http://127.0.0.1:1082"),
            ("no_proxy", "localhost,.local"),
            ("HOME", "/Users/someone"),
            ("PATH", "/usr/bin"),
            ("OPENAI_API_KEY", "never-forwarded"),
        ]);
        assert_eq!(
            forwarded_environment_from(|name| login.get(name).map(|value| (*value).to_owned())),
            vec![
                (
                    "TEKES_KERNEL_LIVE_ARTIFACT".to_owned(),
                    "/tmp/evidence".to_owned()
                ),
                ("HTTPS_PROXY".to_owned(), "http://127.0.0.1:1082".to_owned()),
                ("no_proxy".to_owned(), "localhost,.local".to_owned()),
            ]
        );
        let mut command = closed_command(std::path::Path::new("/usr/bin/env"));
        command.env("TEKES_KERNEL_LIVE_ARTIFACT", "/tmp/evidence");
        let output = command.output().expect("run environment probe");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "TEKES_KERNEL_LIVE_ARTIFACT=/tmp/evidence"
        );
    }

    #[test]
    fn worker_command_drops_the_complete_ambient_environment() {
        let output = closed_command(std::path::Path::new("/usr/bin/env"))
            .output()
            .expect("run environment probe");
        assert!(output.status.success());
        assert!(
            output.stdout.is_empty(),
            "closed command inherited environment"
        );
        assert!(output.stderr.is_empty());
    }
}
