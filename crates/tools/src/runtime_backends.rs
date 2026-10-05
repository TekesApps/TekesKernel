//! Production backend substrate for the Slice-8 tool dispatcher.
//!
//! This module deliberately does not author ledger events.  It returns typed
//! backend outcomes to the worker, which remains responsible for approval,
//! hooks, secret scanning, spilling, and the terminal `tool_result`.

use std::collections::{BTreeMap, VecDeque};
use std::fs::{self, File};
use std::io::Read;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use reqwest::header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use store::{AtomicPublisher, NamedLock};
use thiserror::Error;

use crate::helper::{
    HelperClient, HelperClientError, HelperErrorClass, HelperOperation, HelperRequest,
    HelperResponse,
};
use crate::sandbox::{ProbeStatus, SandboxPolicy, sandbox_command};

pub const HARD_HELPER_BYTES: u64 = 16 * 1024 * 1024;
pub const HARD_HELPER_READ_BYTES: u64 = 32 * 1024 * 1024;
pub const HARD_HELPER_TIMEOUT_MS: u64 = 600_000;
const HELPER_PROCESS_OVERHEAD_MS: u64 = 1_000;
pub const HARD_HTTP_BYTES: usize = 16 * 1024 * 1024;
pub const HARD_HTTP_TIMEOUT_MS: u64 = 120_000;
pub const HARD_JOB_TAIL_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_REDIRECTS: usize = 5;
pub const WEB_FETCH_ACCEPT: &str =
    "text/html,application/xhtml+xml,text/plain,application/json;q=0.9,*/*;q=0.8";
pub const WEB_FETCH_MAX_CHARACTERS: usize = 24_000;
pub const WEB_FETCH_MIN_MEANINGFUL_CHARACTERS: usize = 160;
pub const WEB_FETCH_USER_AGENT: &str = "TekesBot/1.0 (+https://tekes.local)";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendOutcome<T> {
    Completed(Result<T, BackendFailure>),
    Hold(BackendHold),
    Unavailable(BackendUnavailable),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendHold {
    pub scope: String,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendUnavailable {
    pub dependency: String,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendGate {
    Ready,
    Hold(BackendHold),
    Unavailable(BackendUnavailable),
}

/// How a non-success HTTP response fails. The server spoke HTTP correctly and
/// answered; only a malformed exchange is a protocol failure. Classifying every
/// status as `Protocol` reported a plain 404 as "backend protocol failed"
/// (2026-09-20) and made a retryable 503 look terminal.
fn web_fetch_status_failure(status: reqwest::StatusCode) -> BackendFailure {
    let code = status.as_u16();
    match code {
        404 | 410 => BackendFailure::NotFound(format!("HTTP status {code}")),
        401 | 403 => BackendFailure::Denied(format!("HTTP status {code}")),
        408 => BackendFailure::Timeout,
        429 => BackendFailure::Limit(format!("HTTP status {code}")),
        _ if status.is_client_error() => BackendFailure::Invalid(format!("HTTP status {code}")),
        // 5xx is the origin failing to serve a request it accepted: the same
        // request may well succeed later, so it travels as retryable.
        _ if status.is_server_error() => BackendFailure::Unavailable(format!("HTTP status {code}")),
        _ => BackendFailure::Protocol(format!("unexpected HTTP status {code}")),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum BackendFailure {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("operation denied: {0}")]
    Denied(String),
    #[error("operation cancelled")]
    Cancelled,
    #[error("operation timed out")]
    Timeout,
    #[error("byte or item limit exceeded: {0}")]
    Limit(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("operation is in conflict: {0}")]
    Conflict(String),
    #[error("backend unavailable: {0}")]
    Unavailable(String),
    #[error("backend terminally unavailable: {0}")]
    TerminalUnavailable(String),
    #[error("backend I/O failed: {0}")]
    Io(String),
    #[error("backend protocol failed: {0}")]
    Protocol(String),
    #[error("side effect has an unknown outcome: {0}")]
    Unknown(String),
}

impl<T> BackendOutcome<T> {
    fn enter(gate: BackendGate) -> Result<(), Self> {
        match gate {
            BackendGate::Ready => Ok(()),
            BackendGate::Hold(hold) => Err(Self::Hold(hold)),
            BackendGate::Unavailable(unavailable) => Err(Self::Unavailable(unavailable)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

pub trait HelperInvoker: Send + Sync {
    fn invoke(
        &self,
        request: &HelperRequest,
        cancellation: &CancellationToken,
    ) -> Result<HelperResponse, BackendFailure>;
}

impl HelperInvoker for HelperClient {
    fn invoke(
        &self,
        request: &HelperRequest,
        cancellation: &CancellationToken,
    ) -> Result<HelperResponse, BackendFailure> {
        // An exec without a timeout runs to completion: the parent must not
        // impose one of its own, or it would reintroduce the cap through the
        // back door. Cancellation still ends it.
        let timeout = match &request.operation {
            HelperOperation::Exec(exec) => exec.timeout_ms.map(|timeout| {
                Duration::from_millis(timeout.saturating_add(HELPER_PROCESS_OVERHEAD_MS))
            }),
            _ => Some(Duration::from_secs(30)),
        };
        self.execute_cancellable(request, timeout, || cancellation.is_cancelled())
            .map_err(|error| match error {
                HelperClientError::Cancelled => BackendFailure::Cancelled,
                HelperClientError::Helper(error) => match error.class {
                    HelperErrorClass::Timeout => BackendFailure::Timeout,
                    HelperErrorClass::Limit => BackendFailure::Limit(error.message),
                    HelperErrorClass::Denied => BackendFailure::Denied(error.message),
                    HelperErrorClass::Missing => BackendFailure::NotFound(error.message),
                    HelperErrorClass::SandboxUnavailable => {
                        BackendFailure::Unavailable(error.message)
                    }
                    HelperErrorClass::Protocol => BackendFailure::Protocol(error.message),
                    _ => BackendFailure::Io(error.to_string()),
                },
            })
    }
}

impl<T: HelperInvoker + ?Sized> HelperInvoker for Arc<T> {
    fn invoke(
        &self,
        request: &HelperRequest,
        cancellation: &CancellationToken,
    ) -> Result<HelperResponse, BackendFailure> {
        (**self).invoke(request, cancellation)
    }
}

pub struct BoundedHelper<I> {
    invoker: Option<I>,
}

impl<I: HelperInvoker> BoundedHelper<I> {
    #[must_use]
    pub const fn new(invoker: Option<I>) -> Self {
        Self { invoker }
    }

    pub fn invoke(
        &self,
        request: &HelperRequest,
        gate: BackendGate,
        cancellation: &CancellationToken,
    ) -> BackendOutcome<HelperResponse> {
        if let Err(outcome) = BackendOutcome::enter(gate) {
            return outcome;
        }
        if cancellation.is_cancelled() {
            return BackendOutcome::Completed(Err(BackendFailure::Cancelled));
        }
        if let Err(error) = validate_helper_limits(&request.operation) {
            return BackendOutcome::Completed(Err(error));
        }
        let Some(invoker) = &self.invoker else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "exec-helper".to_owned(),
                reason: "helper process is not configured".to_owned(),
            });
        };
        BackendOutcome::Completed(invoker.invoke(request, cancellation))
    }
}

fn validate_helper_limits(operation: &HelperOperation) -> Result<(), BackendFailure> {
    let bounded = |value: u64, ceiling: u64, name: &str| {
        if value == 0 || value > ceiling {
            Err(BackendFailure::Limit(format!(
                "{name} must be in 1..={ceiling}"
            )))
        } else {
            Ok(())
        }
    };
    match operation {
        HelperOperation::Read { max_bytes, .. } => {
            bounded(*max_bytes, HARD_HELPER_READ_BYTES, "max_bytes")
        }
        HelperOperation::Glob { max_entries, .. } => bounded(*max_entries, 100_000, "max_entries"),
        HelperOperation::Grep {
            stdout_bytes,
            timeout_ms,
            context_lines,
            ..
        } => {
            bounded(*stdout_bytes, HARD_HELPER_BYTES, "stdout_bytes")?;
            bounded(*timeout_ms, HARD_HELPER_TIMEOUT_MS, "timeout_ms")?;
            if *context_lines > 100_000 {
                return Err(BackendFailure::Limit("grep context limit exceeded".into()));
            }
            Ok(())
        }
        HelperOperation::Exec(exec) => {
            bounded(exec.stdout_bytes, HARD_HELPER_BYTES, "stdout_bytes")?;
            bounded(exec.stderr_bytes, HARD_HELPER_BYTES, "stderr_bytes")?;
            if exec.timeout_ms == Some(0) {
                return Err(BackendFailure::Limit(
                    "timeout_ms must be at least 1".into(),
                ));
            }
            Ok(())
        }
        HelperOperation::Write { content, .. } => {
            let bytes = content
                .decode()
                .map_err(|error| BackendFailure::Invalid(error.to_string()))?;
            if bytes.len() as u64 > HARD_HELPER_BYTES {
                Err(BackendFailure::Limit(
                    "content exceeds helper byte limit".into(),
                ))
            } else {
                Ok(())
            }
        }
        HelperOperation::Patch { replacement, .. } => {
            let bytes = replacement
                .decode()
                .map_err(|error| BackendFailure::Invalid(error.to_string()))?;
            if bytes.len() as u64 > HARD_HELPER_BYTES {
                Err(BackendFailure::Limit(
                    "replacement exceeds helper byte limit".into(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobSpec {
    pub program: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub writable_paths: Vec<String>,
    pub environment: BTreeMap<String, String>,
}

/// Immutable caller-supplied authority used to resolve one `job.start`.
///
/// `base_sandbox` contains fixed process/network/read authority and any fixed
/// write authority unrelated to the tool invocation. `permitted_write_roots`
/// is the ceiling against which every requested `JobSpec::writable_paths`
/// entry is revalidated. Requested roots are never copied into the effective
/// sandbox merely because the caller supplied them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JobLaunchPolicy {
    primary_workspace_cwd: String,
    base_sandbox: SandboxPolicy,
    permitted_write_roots: Vec<String>,
}

impl JobLaunchPolicy {
    pub fn new(
        primary_workspace_cwd: impl Into<String>,
        base_sandbox: SandboxPolicy,
        permitted_write_roots: Vec<String>,
    ) -> Result<Self, BackendFailure> {
        base_sandbox
            .validate()
            .map_err(|error| BackendFailure::Invalid(error.to_string()))?;
        let primary_workspace_cwd =
            require_canonical_directory(&primary_workspace_cwd.into(), "primary workspace cwd")?;
        let read_roots = base_sandbox
            .read_roots
            .iter()
            .map(|root| require_canonical_directory(root, "sandbox read root"))
            .collect::<Result<Vec<_>, _>>()?;
        for root in &base_sandbox.write_roots {
            require_canonical_directory(root, "sandbox write root")?;
        }
        if let Some(scratch) = &base_sandbox.scratch {
            require_canonical_directory(scratch, "sandbox scratch root")?;
        }
        if !covered_by(&primary_workspace_cwd, &read_roots) {
            return Err(BackendFailure::Denied(
                "primary workspace cwd is outside sandbox read roots".to_owned(),
            ));
        }
        let permitted_write_roots = normalize_policy_roots(
            permitted_write_roots,
            &read_roots,
            "permitted job write root",
        )?;
        Ok(Self {
            primary_workspace_cwd,
            base_sandbox,
            permitted_write_roots,
        })
    }

    #[must_use]
    pub fn primary_workspace_cwd(&self) -> &str {
        &self.primary_workspace_cwd
    }

    #[must_use]
    pub fn base_sandbox(&self) -> &SandboxPolicy {
        &self.base_sandbox
    }

    #[must_use]
    pub fn permitted_write_roots(&self) -> &[String] {
        &self.permitted_write_roots
    }

    fn resolve(&self, mut spec: JobSpec) -> Result<(JobSpec, SandboxPolicy), BackendFailure> {
        let working_directory = match spec.working_directory.as_deref() {
            Some(working_directory) => canonical_invocation_directory(
                working_directory,
                &self.primary_workspace_cwd,
                "job working directory",
            )?,
            None => self.primary_workspace_cwd.clone(),
        };
        if !covered_by(&working_directory, &self.base_sandbox.read_roots) {
            return Err(BackendFailure::Denied(
                "job working directory is outside sandbox read roots".to_owned(),
            ));
        }

        let mut requested = Vec::with_capacity(spec.writable_paths.len());
        for path in &spec.writable_paths {
            let path = canonical_invocation_directory(
                path,
                &self.primary_workspace_cwd,
                "job writable path",
            )?;
            if !covered_by(&path, &self.permitted_write_roots) {
                return Err(BackendFailure::Denied(format!(
                    "job writable path {path} is outside permitted policy roots"
                )));
            }
            if requested.iter().any(|existing| existing == &path) {
                return Err(BackendFailure::Invalid(
                    "job writable paths contain duplicate canonical roots".to_owned(),
                ));
            }
            requested.push(path);
        }
        requested.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));

        spec.working_directory = Some(working_directory);
        spec.writable_paths = requested.clone();
        let mut effective = self.base_sandbox.clone();
        effective.write_roots.extend(requested);
        effective
            .write_roots
            .sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        effective.write_roots.dedup();
        effective
            .validate()
            .map_err(|error| BackendFailure::Invalid(error.to_string()))?;
        Ok((spec, effective))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Starting,
    Running,
    Exited { status: i32 },
    Stopped,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobRecord {
    pub format: u64,
    pub id: String,
    pub spec: JobSpec,
    pub state: JobState,
    pub pid: Option<u32>,
    pub process_start: Option<String>,
    pub created_unix_ms: u64,
    pub updated_unix_ms: u64,
    pub stdout_tail: String,
    pub stderr_tail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JobRunnerRequest {
    format: u64,
    root: String,
    id: String,
    tail_bytes: usize,
    spec: JobSpec,
}

/// Constructs the `tekes-helper` runner command with the mandatory effective
/// platform sandbox already applied. The broker adds only its private
/// `--job-runner` request path; the target inherits the runner's sandbox.
/// The broker deliberately cannot construct an unsandboxed target command
/// from a `JobSpec`; executable assembly supplies this boundary.
pub trait SandboxedJobLauncher: Send + Sync {
    fn runner_command(
        &self,
        spec: &JobSpec,
        effective_policy: &SandboxPolicy,
    ) -> Result<Command, BackendFailure>;
}

/// Production launcher for the sole `tekes-helper --job-runner` binary.
/// Construction fails closed unless the platform sandbox probe succeeded, and
/// every command is compiled from the per-call effective sandbox policy.
#[derive(Clone, Debug)]
pub struct HelperJobLauncher {
    executable: PathBuf,
    probe: ProbeStatus,
}

impl HelperJobLauncher {
    pub fn new(executable: impl Into<PathBuf>, probe: ProbeStatus) -> Result<Self, BackendFailure> {
        let executable = executable.into();
        if !executable.is_absolute() || !executable.is_file() {
            return Err(BackendFailure::Unavailable(
                "tekes-helper executable is not an absolute regular file".to_owned(),
            ));
        }
        if !matches!(probe, ProbeStatus::Available { .. }) {
            return Err(BackendFailure::Unavailable(
                "job sandbox backend is unavailable".to_owned(),
            ));
        }
        Ok(Self { executable, probe })
    }

    /// Keeps the job broker present while refusing every job when a host has
    /// no usable per-job sandbox. MCP-only hosts use this instead of granting
    /// command execution through their broader application sandbox.
    pub fn disabled(executable: impl Into<PathBuf>) -> Result<Self, BackendFailure> {
        let executable = executable.into();
        if !executable.is_absolute() || !executable.is_file() {
            return Err(BackendFailure::Unavailable(
                "tekes-helper executable is not an absolute regular file".to_owned(),
            ));
        }
        Ok(Self {
            executable,
            probe: ProbeStatus::Unavailable {
                class: crate::ProbeFailure::Rejected,
                detail: "per-job sandbox unavailable in MCP-only App Sandbox host".to_owned(),
            },
        })
    }
}

impl SandboxedJobLauncher for HelperJobLauncher {
    fn runner_command(
        &self,
        _spec: &JobSpec,
        effective_policy: &SandboxPolicy,
    ) -> Result<Command, BackendFailure> {
        sandbox_command(effective_policy, &self.probe, &self.executable)
            .map_err(|error| BackendFailure::Unavailable(error.to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct JobBroker {
    root: PathBuf,
    tail_bytes: usize,
    stop_grace: Duration,
}

impl JobBroker {
    pub fn new(root: impl Into<PathBuf>, tail_bytes: usize) -> Result<Self, BackendFailure> {
        if tail_bytes == 0 || tail_bytes > HARD_JOB_TAIL_BYTES {
            return Err(BackendFailure::Limit(format!(
                "job tail must be in 1..={HARD_JOB_TAIL_BYTES}"
            )));
        }
        let root = root.into();
        fs::create_dir_all(&root).map_err(io_failure)?;
        if fs::symlink_metadata(&root)
            .map_err(io_failure)?
            .file_type()
            .is_symlink()
        {
            return Err(BackendFailure::Denied("job root is a symlink".to_owned()));
        }
        let root = fs::canonicalize(root).map_err(io_failure)?;
        Ok(Self {
            root,
            tail_bytes,
            stop_grace: Duration::from_millis(500),
        })
    }

    pub fn start<L: SandboxedJobLauncher + ?Sized + 'static>(
        &self,
        id: &str,
        spec: JobSpec,
        launch_policy: &JobLaunchPolicy,
        launcher: Arc<L>,
        gate: BackendGate,
        cancellation: &CancellationToken,
    ) -> BackendOutcome<JobRecord> {
        if let Err(outcome) = BackendOutcome::enter(gate) {
            return outcome;
        }
        if cancellation.is_cancelled() {
            return BackendOutcome::Completed(Err(BackendFailure::Cancelled));
        }
        if let Err(error) = validate_job_id(id) {
            return BackendOutcome::Completed(Err(error));
        }
        if spec.program.is_empty() || spec.program.as_bytes().contains(&0) {
            return BackendOutcome::Completed(Err(BackendFailure::Invalid(
                "job program is empty or contains NUL".to_owned(),
            )));
        }
        let (spec, effective_policy) = match launch_policy.resolve(spec) {
            Ok(resolved) => resolved,
            Err(error) => return BackendOutcome::Completed(Err(error)),
        };

        BackendOutcome::Completed(self.start_inner(
            id,
            spec,
            effective_policy,
            launcher,
            cancellation,
        ))
    }

    fn start_inner<L: SandboxedJobLauncher + ?Sized + 'static>(
        &self,
        id: &str,
        spec: JobSpec,
        mut effective_policy: SandboxPolicy,
        launcher: Arc<L>,
        cancellation: &CancellationToken,
    ) -> Result<JobRecord, BackendFailure> {
        let directory = self.job_directory(id);
        fs::create_dir(&directory).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                BackendFailure::Conflict(format!("job {id} already exists"))
            } else {
                io_failure(error)
            }
        })?;
        sync_directory(&self.root)?;
        let now = unix_millis()?;
        let starting = JobRecord {
            format: 1,
            id: id.to_owned(),
            spec: spec.clone(),
            state: JobState::Starting,
            pid: None,
            process_start: None,
            created_unix_ms: now,
            updated_unix_ms: now,
            stdout_tail: "stdout.tail".to_owned(),
            stderr_tail: "stderr.tail".to_owned(),
        };
        self.write_record(&starting)?;
        AtomicPublisher::replace(directory.join(&starting.stdout_tail), b"")
            .map_err(store_failure)?;
        AtomicPublisher::replace(directory.join(&starting.stderr_tail), b"")
            .map_err(store_failure)?;

        if cancellation.is_cancelled() {
            let mut cancelled = starting;
            cancelled.state = JobState::Unknown;
            cancelled.updated_unix_ms = unix_millis()?;
            self.write_record(&cancelled)?;
            return Err(BackendFailure::Cancelled);
        }

        let runner_request = JobRunnerRequest {
            format: 1,
            root: self.root.to_string_lossy().into_owned(),
            id: id.to_owned(),
            tail_bytes: self.tail_bytes,
            spec: spec.clone(),
        };
        let request_path = directory.join("runner.json");
        let request_bytes = serde_json_canonicalizer::to_vec(&runner_request)
            .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
        AtomicPublisher::replace(&request_path, &request_bytes).map_err(store_failure)?;

        let job_directory =
            require_canonical_directory(&directory.to_string_lossy(), "job runner directory")?;
        effective_policy.read_roots.push(job_directory.clone());
        effective_policy.write_roots.push(job_directory);
        effective_policy
            .read_roots
            .sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        effective_policy.read_roots.dedup();
        effective_policy
            .write_roots
            .sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        effective_policy.write_roots.dedup();
        effective_policy
            .validate()
            .map_err(|error| BackendFailure::Invalid(error.to_string()))?;

        let mut command = launcher.runner_command(&spec, &effective_policy)?;
        command
            .arg("--job-runner")
            .arg(&request_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // SAFETY: the closure performs only the async-signal-safe `setsid`
        // syscall in the child immediately before exec.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn().map_err(io_failure)?;
        let pid = child.id();
        let token = process_start_token(pid).ok_or_else(|| {
            terminate_group(&mut child, Duration::from_millis(100));
            BackendFailure::Unknown("could not obtain process start identity".to_owned())
        })?;
        let mut running = starting;
        running.state = JobState::Running;
        running.pid = Some(pid);
        running.process_start = Some(token);
        running.updated_unix_ms = unix_millis()?;
        if let Err(error) = self.write_record(&running) {
            terminate_group(&mut child, Duration::from_millis(100));
            return Err(error);
        }
        if cancellation.is_cancelled() {
            terminate_group(&mut child, Duration::from_millis(100));
            running.state = JobState::Unknown;
            running.updated_unix_ms = unix_millis()?;
            self.write_record(&running)?;
            return Err(BackendFailure::Cancelled);
        }
        AtomicPublisher::replace(directory.join("start.permit"), b"start\n")
            .map_err(store_failure)?;

        // Reaping is an optimization only. The detached runner owns the target,
        // tails and terminal record and remains correct if this process dies.
        thread::spawn(move || {
            let _ = child.wait();
        });
        Ok(running)
    }

    pub fn status(&self, id: &str) -> BackendOutcome<JobRecord> {
        BackendOutcome::Completed(self.status_inner(id))
    }

    fn status_inner(&self, id: &str) -> Result<JobRecord, BackendFailure> {
        validate_job_id(id)?;
        let mut record = self.read_record(id)?;
        if record.state == JobState::Starting {
            // `start` returns only after the running record is durable. A
            // visible starting record therefore proves a crashed launch
            // window, not a successful detached process.
            record.state = JobState::Unknown;
            record.updated_unix_ms = unix_millis()?;
            self.write_record(&record)?;
        } else if record.state == JobState::Running {
            let live =
                record
                    .pid
                    .zip(record.process_start.as_deref())
                    .is_some_and(|(pid, expected)| {
                        process_start_token(pid).as_deref() == Some(expected)
                    });
            if !live {
                record.state = JobState::Unknown;
                record.updated_unix_ms = unix_millis()?;
                self.write_record(&record)?;
            }
        }
        Ok(record)
    }

    pub fn list(&self) -> BackendOutcome<Vec<JobRecord>> {
        BackendOutcome::Completed(self.list_inner())
    }

    fn list_inner(&self) -> Result<Vec<JobRecord>, BackendFailure> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(&self.root).map_err(io_failure)? {
            let entry = entry.map_err(io_failure)?;
            if entry.file_type().map_err(io_failure)?.is_dir() {
                if let Some(id) = entry.file_name().to_str() {
                    if validate_job_id(id).is_ok() {
                        ids.push(id.to_owned());
                    }
                }
            }
        }
        ids.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        ids.into_iter().map(|id| self.status_inner(&id)).collect()
    }

    pub fn stop(
        &self,
        id: &str,
        gate: BackendGate,
        cancellation: &CancellationToken,
    ) -> BackendOutcome<JobRecord> {
        if let Err(outcome) = BackendOutcome::enter(gate) {
            return outcome;
        }
        if cancellation.is_cancelled() {
            return BackendOutcome::Completed(Err(BackendFailure::Cancelled));
        }
        BackendOutcome::Completed(self.stop_inner(id, cancellation))
    }

    fn stop_inner(
        &self,
        id: &str,
        cancellation: &CancellationToken,
    ) -> Result<JobRecord, BackendFailure> {
        let mut record = self.status_inner(id)?;
        if matches!(record.state, JobState::Exited { .. } | JobState::Stopped) {
            return Ok(record);
        }
        if record.state != JobState::Running {
            return Err(BackendFailure::Unknown(format!(
                "job {id} has no verified live process"
            )));
        }
        let pid = record.pid.expect("running job has pid");
        signal_group(pid, libc::SIGTERM)?;
        let deadline = Instant::now() + self.stop_grace;
        while Instant::now() < deadline {
            if cancellation.is_cancelled() {
                return Err(BackendFailure::Cancelled);
            }
            if process_start_token(pid).as_deref() != record.process_start.as_deref() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        if process_start_token(pid).as_deref() == record.process_start.as_deref() {
            signal_group(pid, libc::SIGKILL)?;
        }
        record.state = JobState::Stopped;
        record.updated_unix_ms = unix_millis()?;
        self.write_record(&record)?;
        Ok(record)
    }

    fn job_directory(&self, id: &str) -> PathBuf {
        self.root.join(id)
    }

    fn read_record(&self, id: &str) -> Result<JobRecord, BackendFailure> {
        let directory = self.job_directory(id);
        let _lock = NamedLock::shared(directory.join("job.lock")).map_err(store_failure)?;
        let bytes = fs::read(directory.join("job.json")).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                BackendFailure::NotFound(format!("job {id}"))
            } else {
                io_failure(error)
            }
        })?;
        serde_json::from_slice(&bytes).map_err(|error| BackendFailure::Protocol(error.to_string()))
    }

    fn write_record(&self, record: &JobRecord) -> Result<(), BackendFailure> {
        let directory = self.job_directory(&record.id);
        let _lock = NamedLock::exclusive(directory.join("job.lock")).map_err(store_failure)?;
        let bytes = serde_json_canonicalizer::to_vec(record)
            .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
        AtomicPublisher::replace(directory.join("job.json"), &bytes).map_err(store_failure)
    }
}

/// Entry point for `tekes-helper --job-runner`. The runner is already inside
/// the effective sandbox and owns the target, bounded tails, and terminal
/// metadata independently of the supervisor that launched it.
pub fn run_job_runner(request_path: impl AsRef<Path>) -> Result<(), BackendFailure> {
    let request_path = request_path.as_ref();
    if !request_path.is_absolute() {
        return Err(BackendFailure::Invalid(
            "job runner request path must be absolute".to_owned(),
        ));
    }
    let bytes = fs::read(request_path).map_err(io_failure)?;
    if bytes.len() > 1024 * 1024 {
        return Err(BackendFailure::Limit(
            "job runner request exceeds 1 MiB".to_owned(),
        ));
    }
    let request: JobRunnerRequest = serde_json::from_slice(&bytes)
        .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
    if request.format != 1 {
        return Err(BackendFailure::Protocol(
            "unsupported job runner request format".to_owned(),
        ));
    }
    validate_job_id(&request.id)?;
    let broker = JobBroker::new(&request.root, request.tail_bytes)?;
    let directory = broker.job_directory(&request.id);
    if request_path != directory.join("runner.json") {
        return Err(BackendFailure::Denied(
            "job runner request is outside its job directory".to_owned(),
        ));
    }
    let permit = directory.join("start.permit");
    let deadline = Instant::now() + Duration::from_secs(30);
    while !permit.is_file() {
        if Instant::now() >= deadline {
            return Err(BackendFailure::Timeout);
        }
        thread::sleep(Duration::from_millis(5));
    }
    let record = broker.read_record(&request.id)?;
    let runner_pid = std::process::id();
    if record.state != JobState::Running
        || record.pid != Some(runner_pid)
        || record.process_start.as_deref() != process_start_token(runner_pid).as_deref()
    {
        return Err(BackendFailure::Conflict(
            "job runner does not match the durable running record".to_owned(),
        ));
    }

    let working_directory = request.spec.working_directory.as_deref().ok_or_else(|| {
        BackendFailure::Protocol("job runner request has no resolved working directory".to_owned())
    })?;
    let mut command = Command::new(&request.spec.program);
    command
        .args(&request.spec.args)
        .current_dir(working_directory)
        .env_clear()
        .envs(&request.spec.environment)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(io_failure)?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| BackendFailure::Protocol("job stdout pipe missing".to_owned()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| BackendFailure::Protocol("job stderr pipe missing".to_owned()))?;
    let stdout_path = directory.join(&record.stdout_tail);
    let stderr_path = directory.join(&record.stderr_tail);
    let tail_bytes = request.tail_bytes;
    let stdout_thread = thread::spawn(move || drain_tail(stdout, &stdout_path, tail_bytes));
    let stderr_thread = thread::spawn(move || drain_tail(stderr, &stderr_path, tail_bytes));
    let status = child.wait().map_err(io_failure);
    let stdout_result = stdout_thread
        .join()
        .map_err(|_| BackendFailure::Unknown("job stdout reader panicked".to_owned()))?;
    let stderr_result = stderr_thread
        .join()
        .map_err(|_| BackendFailure::Unknown("job stderr reader panicked".to_owned()))?;
    stdout_result?;
    stderr_result?;

    let mut record = broker.read_record(&request.id)?;
    if record.state != JobState::Stopped {
        record.state = match status {
            Ok(status) => JobState::Exited {
                status: status.code().unwrap_or(-1),
            },
            Err(_) => JobState::Unknown,
        };
        record.updated_unix_ms = unix_millis()?;
        broker.write_record(&record)?;
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpLimits {
    pub timeout: Duration,
    pub max_bytes: usize,
    pub max_redirects: usize,
}

impl Default for HttpLimits {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_bytes: 4 * 1024 * 1024,
            max_redirects: MAX_REDIRECTS,
        }
    }
}

impl HttpLimits {
    fn validate(&self) -> Result<(), BackendFailure> {
        if self.timeout.is_zero() || self.timeout > Duration::from_millis(HARD_HTTP_TIMEOUT_MS) {
            return Err(BackendFailure::Limit(
                "HTTP timeout is outside hard bounds".to_owned(),
            ));
        }
        if self.max_bytes == 0 || self.max_bytes > HARD_HTTP_BYTES {
            return Err(BackendFailure::Limit(
                "HTTP byte cap is outside hard bounds".to_owned(),
            ));
        }
        if self.max_redirects > MAX_REDIRECTS {
            return Err(BackendFailure::Limit(
                "HTTP redirect cap is outside hard bounds".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpFetchResult {
    pub final_url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
    pub extraction: WebExtraction,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebExtractionKind {
    Html,
    Text,
    NonText,
}

/// Deterministic model-facing reduction of a bounded HTTP body. External
/// JavaScript readers are deliberately not part of v1; `under_rendered`
/// makes that retirement observable without sending the target URL to a
/// second service.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WebExtraction {
    pub kind: WebExtractionKind,
    pub text: String,
    pub under_rendered: bool,
    pub truncated: bool,
}

#[derive(Clone, Debug)]
pub struct BoundedHttpClient {
    limits: HttpLimits,
}

impl BoundedHttpClient {
    pub fn new(limits: HttpLimits) -> Result<Self, BackendFailure> {
        limits.validate()?;
        Ok(Self { limits })
    }

    pub fn fetch(
        &self,
        url: &str,
        gate: BackendGate,
        cancellation: &CancellationToken,
    ) -> BackendOutcome<HttpFetchResult> {
        if let Err(outcome) = BackendOutcome::enter(gate) {
            return outcome;
        }
        if cancellation.is_cancelled() {
            return BackendOutcome::Completed(Err(BackendFailure::Cancelled));
        }
        let url = url.to_owned();
        let limits = self.limits.clone();
        let cancellation = cancellation.clone();
        let result = thread::Builder::new()
            .name("tekes-http-backend".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(io_failure)?;
                runtime.block_on(fetch_async(url, limits, cancellation))
            })
            .map_err(io_failure)
            .and_then(|thread| {
                thread
                    .join()
                    .map_err(|_| BackendFailure::Unknown("HTTP worker panicked".to_owned()))?
            });
        BackendOutcome::Completed(result)
    }

    pub fn search<P: SearchProvider + ?Sized>(
        &self,
        request: &SearchRequest,
        provider: Option<&P>,
        gate: BackendGate,
        cancellation: &CancellationToken,
    ) -> BackendOutcome<Vec<SearchHit>> {
        if let Err(outcome) = BackendOutcome::enter(gate) {
            return outcome;
        }
        if request.query.trim().is_empty() || !(1..=10).contains(&request.max_results) {
            return BackendOutcome::Completed(Err(BackendFailure::Invalid(
                "search query must be nonempty and max_results in 1..=10".to_owned(),
            )));
        }
        if cancellation.is_cancelled() {
            return BackendOutcome::Completed(Err(BackendFailure::Cancelled));
        }
        let Some(provider) = provider else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "web-search-provider".to_owned(),
                reason: "no credential-backed search provider is configured".to_owned(),
            });
        };
        let result = provider
            .search(request, &self.limits, cancellation)
            .and_then(|mut hits| {
                let total_bytes = hits.iter().try_fold(0usize, |total, hit| {
                    total
                        .checked_add(hit.title.len())
                        .and_then(|value| value.checked_add(hit.url.len()))
                        .and_then(|value| value.checked_add(hit.snippet.len()))
                        .ok_or_else(|| {
                            BackendFailure::Limit("search result size overflow".to_owned())
                        })
                })?;
                if total_bytes > self.limits.max_bytes {
                    return Err(BackendFailure::Limit(
                        "search results exceed the HTTP byte cap".to_owned(),
                    ));
                }
                hits.truncate(request.max_results.into());
                Ok(hits)
            });
        BackendOutcome::Completed(result)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchTopic {
    General,
    News,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchRequest {
    /// Durable tool-call identity used for the credential broker attempt.
    pub call_id: String,
    pub query: String,
    pub max_results: u8,
    pub topic: SearchTopic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchHit {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// Provider-specific request/credential/response bytes remain outside the
/// fixed tool ABI. Implementations must use the supplied limits and token and
/// return no credential material in `SearchHit` or failures.
pub trait SearchProvider: Send + Sync {
    fn search(
        &self,
        request: &SearchRequest,
        limits: &HttpLimits,
        cancellation: &CancellationToken,
    ) -> Result<Vec<SearchHit>, BackendFailure>;
}

async fn fetch_async(
    mut url: String,
    limits: HttpLimits,
    cancellation: CancellationToken,
) -> Result<HttpFetchResult, BackendFailure> {
    let deadline = Instant::now() + limits.timeout;
    for hop in 0..=limits.max_redirects {
        if cancellation.is_cancelled() {
            return Err(BackendFailure::Cancelled);
        }
        let parsed = reqwest::Url::parse(&url)
            .map_err(|error| BackendFailure::Invalid(error.to_string()))?;
        let route = route_public_url(&parsed)?;
        let client = route
            .client_builder()?
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| BackendFailure::Unavailable(error.to_string()))?;
        let response = await_cancel_deadline(
            client
                .get(parsed.clone())
                .header(USER_AGENT, WEB_FETCH_USER_AGENT)
                .header(ACCEPT, WEB_FETCH_ACCEPT)
                .send(),
            &cancellation,
            deadline,
        )
        .await?
        .map_err(|error| BackendFailure::Io(error.to_string()))?;

        if response.status().is_redirection() {
            if hop == limits.max_redirects {
                return Err(BackendFailure::Limit("redirect limit exceeded".to_owned()));
            }
            let location = response
                .headers()
                .get(LOCATION)
                .ok_or_else(|| BackendFailure::Protocol("redirect has no Location".to_owned()))?
                .to_str()
                .map_err(|_| {
                    BackendFailure::Protocol("redirect Location is not UTF-8".to_owned())
                })?;
            url = resolve_web_redirect(parsed.as_str(), location)?;
            continue;
        }
        if !response.status().is_success() {
            return Err(web_fetch_status_failure(response.status()));
        }
        if response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<usize>().ok())
            .is_some_and(|length| length > limits.max_bytes)
        {
            return Err(BackendFailure::Limit(
                "response Content-Length exceeds cap".to_owned(),
            ));
        }
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let mut response = response;
        let mut body = Vec::new();
        while let Some(chunk) = await_cancel_deadline(response.chunk(), &cancellation, deadline)
            .await?
            .map_err(|error| BackendFailure::Io(error.to_string()))?
        {
            if body.len().saturating_add(chunk.len()) > limits.max_bytes {
                return Err(BackendFailure::Limit(
                    "response body exceeds cap".to_owned(),
                ));
            }
            body.extend_from_slice(&chunk);
        }
        let extraction = extract_web_content(content_type.as_deref(), &body);
        return Ok(HttpFetchResult {
            final_url: parsed.to_string(),
            status,
            content_type,
            body,
            extraction,
        });
    }
    Err(BackendFailure::Limit("redirect limit exceeded".to_owned()))
}

/// Resolve one redirect exactly as the production loop does. The returned URL
/// is deliberately not trusted: the next loop iteration re-enters
/// `route_public_url`, including DNS classification and address pinning.
pub fn resolve_web_redirect(current: &str, location: &str) -> Result<String, BackendFailure> {
    let current = reqwest::Url::parse(current)
        .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
    current
        .join(location)
        .map(|url| url.to_string())
        .map_err(|error| BackendFailure::Protocol(error.to_string()))
}

async fn await_cancel_deadline<F: std::future::Future>(
    future: F,
    cancellation: &CancellationToken,
    deadline: Instant,
) -> Result<F::Output, BackendFailure> {
    tokio::pin!(future);
    loop {
        if cancellation.is_cancelled() {
            return Err(BackendFailure::Cancelled);
        }
        let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
            return Err(BackendFailure::Timeout);
        };
        tokio::select! {
            output = &mut future => return Ok(output),
            () = tokio::time::sleep(remaining.min(Duration::from_millis(10))) => {}
        }
    }
}

/// How one public web request leaves the worker: pinned to a DNS answer this
/// process classified, or handed to the outbound proxy the environment names
/// for it. The proxy resolves the origin in its own network, so the local
/// answer is neither consulted nor pinned — under a fake-IP VPN it would be a
/// `198.18.0.0/15` placeholder that no public-address policy can accept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicRoute {
    /// Connect directly to `address`, the classified answer for `host`.
    Direct { host: String, address: SocketAddr },
    /// Tunnel through `proxy`; the origin hostname is sent to the proxy as is.
    Proxied { proxy: reqwest::Url },
}

impl PublicRoute {
    /// A client builder that follows exactly this route and never a second
    /// proxy decision: the system proxy reading reqwest would otherwise apply
    /// is disabled, so a pinned connection cannot be tunnelled and a tunnelled
    /// one cannot be pinned.
    pub fn client_builder(&self) -> Result<reqwest::ClientBuilder, BackendFailure> {
        let builder = reqwest::Client::builder().no_proxy();
        Ok(match self {
            Self::Direct { host, address } => builder.resolve(host, *address),
            Self::Proxied { proxy } => builder.proxy(
                reqwest::Proxy::all(proxy.clone())
                    .map_err(|error| BackendFailure::Unavailable(error.to_string()))?,
            ),
        })
    }
}

/// Validate a URL for a public web request and decide its route. Structural
/// checks (scheme, userinfo, local hostnames, non-public IP literals) apply on
/// both routes; DNS classification and pinning apply only to a direct one.
pub fn route_public_url(url: &reqwest::Url) -> Result<PublicRoute, BackendFailure> {
    route_public_url_with(url, |name| std::env::var(name).ok())
}

/// `route_public_url` over an explicit environment reader, for tests.
pub fn route_public_url_with(
    url: &reqwest::Url,
    env: impl Fn(&str) -> Option<String>,
) -> Result<PublicRoute, BackendFailure> {
    if !matches!(url.scheme(), "http" | "https") {
        return Err(BackendFailure::Denied(
            "only HTTP(S) URLs are allowed".to_owned(),
        ));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(BackendFailure::Denied(
            "URL userinfo is forbidden".to_owned(),
        ));
    }
    let host = url
        .host_str()
        .ok_or_else(|| BackendFailure::Invalid("URL has no host".to_owned()))?
        .trim_end_matches('.')
        .to_ascii_lowercase();
    if host == "localhost"
        || host.ends_with(".localhost")
        || host.ends_with(".local")
        || host.ends_with(".internal")
    {
        return Err(BackendFailure::Denied(
            "local hostnames are forbidden".to_owned(),
        ));
    }
    let port = url
        .port_or_known_default()
        .ok_or_else(|| BackendFailure::Invalid("URL has no usable port".to_owned()))?;
    let address_literal = host
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(&host);
    let literal = address_literal.parse::<IpAddr>().ok();
    if let Some(address) = literal {
        if !is_public_internet_address(address) {
            return Err(BackendFailure::Denied(
                "URL names a non-public address".to_owned(),
            ));
        }
    }
    if let Some(proxy) = outbound_proxy_for(url.scheme(), address_literal, literal, &env) {
        return Ok(PublicRoute::Proxied { proxy });
    }
    if let Some(address) = literal {
        return Ok(PublicRoute::Direct {
            host: address_literal.to_owned(),
            address: SocketAddr::new(address, port),
        });
    }
    let addresses: Vec<_> = (host.as_str(), port)
        .to_socket_addrs()
        .map_err(|error| BackendFailure::Unavailable(error.to_string()))?
        .collect();
    if addresses.is_empty() {
        return Err(BackendFailure::Unavailable(
            "DNS returned no address".to_owned(),
        ));
    }
    if addresses
        .iter()
        .any(|address| !is_public_internet_address(address.ip()))
    {
        return Err(BackendFailure::Denied(
            "DNS resolved to a non-public address".to_owned(),
        ));
    }
    Ok(PublicRoute::Direct {
        host,
        address: addresses[0],
    })
}

/// The outbound proxy the environment names for `scheme`, unless `no_proxy`
/// exempts the host. Precedence follows the convention every other HTTP client
/// in the worker (reqwest's system proxy) applies: the scheme's own variable,
/// then `ALL_PROXY`, uppercase before lowercase. Only `http://` and `https://`
/// proxies are usable by this build; anything else is treated as unset.
fn outbound_proxy_for(
    scheme: &str,
    host: &str,
    literal: Option<IpAddr>,
    env: &impl Fn(&str) -> Option<String>,
) -> Option<reqwest::Url> {
    let read = |name: &str| {
        env(name)
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
    };
    let names: [&str; 4] = if scheme == "https" {
        ["HTTPS_PROXY", "https_proxy", "ALL_PROXY", "all_proxy"]
    } else {
        ["HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"]
    };
    let proxy = names.iter().find_map(|name| read(name))?;
    let proxy = reqwest::Url::parse(&proxy).ok()?;
    if !matches!(proxy.scheme(), "http" | "https") || proxy.host_str().is_none() {
        return None;
    }
    let no_proxy = read("NO_PROXY")
        .or_else(|| read("no_proxy"))
        .unwrap_or_default();
    if no_proxy_exempts(&no_proxy, host, literal) {
        return None;
    }
    Some(proxy)
}

/// Whether a `NO_PROXY` list exempts `host`: `*` exempts everything; an entry
/// matches the host and every subdomain under it (a leading `.` or `*.` is
/// accepted, a trailing `:port` ignored); an IP entry or CIDR block matches an
/// IP-literal host.
fn no_proxy_exempts(no_proxy: &str, host: &str, literal: Option<IpAddr>) -> bool {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    for raw in no_proxy.split(|c: char| c == ',' || c.is_whitespace()) {
        let entry = raw.trim().to_ascii_lowercase();
        if entry.is_empty() {
            continue;
        }
        if entry == "*" {
            return true;
        }
        if let Some((network, bits)) = entry.split_once('/') {
            if let (Some(ip), Ok(network), Ok(bits)) =
                (literal, network.parse::<IpAddr>(), bits.parse::<u32>())
            {
                if cidr_contains(network, bits, ip) {
                    return true;
                }
            }
            continue;
        }
        let entry = entry.trim_start_matches("*.").trim_start_matches('.');
        let entry = match entry.rsplit_once(':') {
            Some((name, port))
                if !name.contains(':') && port.chars().all(|c| c.is_ascii_digit()) =>
            {
                name
            }
            _ => entry,
        };
        let entry = entry.trim_end_matches('.');
        if entry.is_empty() {
            continue;
        }
        if let (Some(ip), Ok(candidate)) = (
            literal,
            entry
                .trim_matches(|c| c == '[' || c == ']')
                .parse::<IpAddr>(),
        ) {
            if ip == candidate {
                return true;
            }
            continue;
        }
        if host == entry || host.ends_with(&format!(".{entry}")) {
            return true;
        }
    }
    false
}

fn cidr_contains(network: IpAddr, bits: u32, ip: IpAddr) -> bool {
    match (network, ip) {
        (IpAddr::V4(network), IpAddr::V4(ip)) if bits <= 32 => {
            let mask = if bits == 0 {
                0
            } else {
                u32::MAX << (32 - bits)
            };
            u32::from(network) & mask == u32::from(ip) & mask
        }
        (IpAddr::V6(network), IpAddr::V6(ip)) if bits <= 128 => {
            let mask = if bits == 0 {
                0
            } else {
                u128::MAX << (128 - bits)
            };
            u128::from(network) & mask == u128::from(ip) & mask
        }
        _ => false,
    }
}

/// Shared SSRF boundary for every configured or dynamically resolved network
/// target. IPv4-mapped IPv6 is classified as its IPv4 address so mapped
/// loopback/private/special-use ranges cannot bypass the policy.
#[must_use]
pub fn is_public_internet_address(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(ip) => {
            let [a, b, c, _] = ip.octets();
            !(a == 0
                || a == 10
                || a == 127
                || (a == 100 && (64..=127).contains(&b))
                || (a == 169 && b == 254)
                || (a == 172 && (16..=31).contains(&b))
                || (a == 192 && b == 0 && c == 0)
                || (a == 192 && b == 0 && c == 2)
                || (a == 192 && b == 168)
                || (a == 192 && b == 88 && c == 99)
                || (a == 198 && (b == 18 || b == 19))
                || (a == 198 && b == 51 && c == 100)
                || (a == 203 && b == 0 && c == 113)
                || a >= 224)
        }
        IpAddr::V6(ip) => {
            let segments = ip.segments();
            if let Some(mapped) = ip.to_ipv4_mapped() {
                return is_public_internet_address(IpAddr::V4(mapped));
            }
            let octets = ip.octets();
            if octets[..12].iter().all(|byte| *byte == 0) {
                return is_public_internet_address(IpAddr::V4(std::net::Ipv4Addr::new(
                    octets[12], octets[13], octets[14], octets[15],
                )));
            }
            if octets[..4] == [0x00, 0x64, 0xff, 0x9b]
                && octets[4..12].iter().all(|byte| *byte == 0)
            {
                return is_public_internet_address(IpAddr::V4(std::net::Ipv4Addr::new(
                    octets[12], octets[13], octets[14], octets[15],
                )));
            }
            // 64:ff9b:1::/48 is a local-use translation prefix. Unlike the
            // well-known /96 form above, its embedding position is deployment
            // selected, so the whole prefix is non-public here.
            !(ip.is_unspecified()
                || ip.is_loopback()
                || ip.is_multicast()
                || (segments[0] == 0x0064 && segments[1] == 0xff9b && segments[2] == 0x0001)
                || (segments[0] == 0x0100
                    && segments[1] == 0
                    && segments[2] == 0
                    && segments[3] == 0)
                || (segments[0] & 0xfe00) == 0xfc00
                || (segments[0] & 0xffc0) == 0xfe80
                || (segments[0] & 0xffc0) == 0xfec0
                || (segments[0] == 0x2001 && segments[1] == 0)
                || (segments[0] == 0x2001 && segments[1] == 2 && segments[2] == 0)
                || (segments[0] == 0x2001 && (segments[1] & 0xfff0) == 0x0010)
                || (segments[0] == 0x2001 && (segments[1] & 0xfff0) == 0x0020)
                || (segments[0] == 0x2001 && segments[1] == 0x0db8)
                || segments[0] == 0x2002
                || (segments[0] == 0x3fff && (segments[1] & 0xf000) == 0)
                || segments[0] == 0x5f00)
        }
    }
}

/// Production extractor used by `web_fetch` and by the Slice-14E oracle.
/// The implementation is intentionally dependency-light, but its byte-to-
/// text behavior is contract-owned rather than a best-effort caller choice.
#[must_use]
pub fn extract_web_content(content_type: Option<&str>, bytes: &[u8]) -> WebExtraction {
    let normalized_type = content_type.unwrap_or_default().to_ascii_lowercase();
    let textual = normalized_type.is_empty()
        || normalized_type.contains("html")
        || normalized_type.starts_with("text/")
        || normalized_type.contains("json")
        || normalized_type.contains("xml")
        || normalized_type.contains("javascript");
    if !textual {
        return WebExtraction {
            kind: WebExtractionKind::NonText,
            text: format!(
                "[non-text content: {}, {} bytes — not rendered]",
                if normalized_type.is_empty() {
                    "unknown"
                } else {
                    normalized_type.as_str()
                },
                bytes.len()
            ),
            under_rendered: false,
            truncated: false,
        };
    }
    let raw = decode_web_text(bytes);
    let is_html =
        normalized_type.contains("html") || (normalized_type.is_empty() && looks_like_html(&raw));
    let rendered = if is_html {
        extract_html_text(&raw)
    } else {
        raw.trim().to_owned()
    };
    let under_rendered = is_html && html_is_under_rendered(&rendered);
    let (text, truncated) = truncate_characters(&rendered, WEB_FETCH_MAX_CHARACTERS);
    WebExtraction {
        kind: if is_html {
            WebExtractionKind::Html
        } else {
            WebExtractionKind::Text
        },
        text,
        under_rendered,
        truncated,
    }
}

fn decode_web_text(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).map_or_else(
        |_| bytes.iter().map(|byte| char::from(*byte)).collect(),
        str::to_owned,
    )
}

fn looks_like_html(value: &str) -> bool {
    let head = value
        .chars()
        .take(512)
        .collect::<String>()
        .to_ascii_lowercase();
    head.contains("<!doctype html") || head.contains("<html") || head.contains("<body")
}

fn extract_html_text(html: &str) -> String {
    let mut output = String::with_capacity(html.len());
    let title = extract_html_title(html);
    let mut suppressed: Vec<String> = Vec::new();
    let bytes = html.as_bytes();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes[cursor] != b'<' {
            let next = bytes[cursor..]
                .iter()
                .position(|byte| *byte == b'<')
                .map_or(bytes.len(), |offset| cursor + offset);
            if suppressed.is_empty() {
                output.push_str(&html[cursor..next]);
            }
            cursor = next;
            continue;
        }
        if html[cursor..].starts_with("<!--") {
            cursor = html[cursor + 4..]
                .find("-->")
                .map_or(bytes.len(), |offset| cursor + 4 + offset + 3);
            continue;
        }
        let Some(relative_end) = html[cursor..].find('>') else {
            break;
        };
        let end = cursor + relative_end;
        let raw_tag = html[cursor + 1..end].trim();
        let closing = raw_tag.starts_with('/');
        let name = raw_tag
            .trim_start_matches('/')
            .split(|character: char| character.is_ascii_whitespace() || character == '/')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if closing {
            if suppressed.last().is_some_and(|value| value == &name) {
                suppressed.pop();
            }
            if suppressed.is_empty() && is_html_block(&name) {
                output.push_str("\n\n");
            }
        } else if matches!(name.as_str(), "script" | "style" | "head" | "noscript") {
            suppressed.push(name.clone());
        } else if suppressed.is_empty() {
            match name.as_str() {
                "br" | "hr" => output.push('\n'),
                "li" => output.push_str("\n- "),
                _ => {}
            }
        }
        cursor = end + 1;
    }
    let mut body = collapse_web_whitespace(&decode_html_entities(&output));
    let title = collapse_web_whitespace(&decode_html_entities(&title));
    if !title.is_empty() && !body.starts_with(&title) {
        body = format!("Title: {title}\n\n{body}");
    }
    body
}

fn extract_html_title(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let Some(open) = lower.find("<title") else {
        return String::new();
    };
    let Some(open_end) = lower[open..].find('>').map(|offset| open + offset + 1) else {
        return String::new();
    };
    let Some(close) = lower[open_end..]
        .find("</title>")
        .map(|offset| open_end + offset)
    else {
        return String::new();
    };
    html[open_end..close].to_owned()
}

fn is_html_block(name: &str) -> bool {
    matches!(
        name,
        "p" | "div"
            | "li"
            | "ul"
            | "ol"
            | "tr"
            | "table"
            | "section"
            | "article"
            | "header"
            | "footer"
            | "nav"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "blockquote"
            | "pre"
    )
}

fn decode_html_entities(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes[cursor] != b'&' {
            let character = value[cursor..].chars().next().expect("character boundary");
            output.push(character);
            cursor += character.len_utf8();
            continue;
        }
        let Some(relative_end) = value[cursor..].find(';') else {
            output.push('&');
            cursor += 1;
            continue;
        };
        let end = cursor + relative_end;
        let entity = &value[cursor..=end];
        let replacement = match entity {
            "&amp;" => Some("&".to_owned()),
            "&lt;" => Some("<".to_owned()),
            "&gt;" => Some(">".to_owned()),
            "&quot;" => Some("\"".to_owned()),
            "&apos;" | "&#39;" => Some("'".to_owned()),
            "&nbsp;" => Some(" ".to_owned()),
            "&mdash;" => Some("—".to_owned()),
            "&ndash;" => Some("–".to_owned()),
            "&hellip;" => Some("…".to_owned()),
            "&copy;" => Some("©".to_owned()),
            "&reg;" => Some("®".to_owned()),
            "&trade;" => Some("™".to_owned()),
            "&rsquo;" => Some("’".to_owned()),
            "&lsquo;" => Some("‘".to_owned()),
            "&ldquo;" => Some("“".to_owned()),
            "&rdquo;" => Some("”".to_owned()),
            "&middot;" => Some("·".to_owned()),
            _ => decode_numeric_entity(entity),
        };
        if let Some(replacement) = replacement {
            output.push_str(&replacement);
            cursor = end + 1;
        } else {
            output.push('&');
            cursor += 1;
        }
    }
    output
}

fn decode_numeric_entity(entity: &str) -> Option<String> {
    let body = entity.strip_prefix("&#")?.strip_suffix(';')?;
    let value = body
        .strip_prefix('x')
        .or_else(|| body.strip_prefix('X'))
        .map_or_else(|| body.parse::<u32>(), |hex| u32::from_str_radix(hex, 16))
        .ok()?;
    char::from_u32(value).map(|value| value.to_string())
}

fn collapse_web_whitespace(value: &str) -> String {
    let mut lines = Vec::new();
    let mut blank = false;
    for raw in value.lines() {
        let line = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            if !blank && !lines.is_empty() {
                lines.push(String::new());
            }
            blank = true;
        } else {
            lines.push(line);
            blank = false;
        }
    }
    while lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    lines.join("\n")
}

fn html_is_under_rendered(value: &str) -> bool {
    let meaningful = value
        .strip_prefix("Title:")
        .and_then(|value| value.split_once('\n').map(|(_, body)| body))
        .unwrap_or(value)
        .trim();
    let lower = meaningful.to_ascii_lowercase();
    meaningful.chars().count() < WEB_FETCH_MIN_MEANINGFUL_CHARACTERS
        || lower.contains("enable javascript")
        || lower.contains("requires javascript")
}

fn truncate_characters(value: &str, limit: usize) -> (String, bool) {
    let mut characters = value.chars();
    let prefix = characters.by_ref().take(limit).collect::<String>();
    if characters.next().is_none() {
        (prefix, false)
    } else {
        (
            format!("{prefix}\n\n[truncated at {limit} characters]"),
            true,
        )
    }
}

fn require_canonical_directory(value: &str, label: &str) -> Result<String, BackendFailure> {
    let path = Path::new(value);
    if value.is_empty()
        || !path.is_absolute()
        || value.contains('\0')
        || value.chars().any(char::is_control)
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(BackendFailure::Invalid(format!(
            "{label} must be an absolute canonical directory"
        )));
    }
    let canonical = fs::canonicalize(path)
        .map_err(|error| BackendFailure::Invalid(format!("{label} is unavailable: {error}")))?;
    if canonical != path || !canonical.is_dir() {
        return Err(BackendFailure::Invalid(format!(
            "{label} must be an absolute canonical directory"
        )));
    }
    canonical
        .to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| BackendFailure::Invalid(format!("{label} is not valid UTF-8")))
}

fn canonical_invocation_directory(
    value: &str,
    primary_workspace_cwd: &str,
    label: &str,
) -> Result<String, BackendFailure> {
    let path = Path::new(value);
    if value.is_empty()
        || value.contains('\0')
        || value.chars().any(char::is_control)
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(BackendFailure::Invalid(format!(
            "{label} is empty or contains unsafe components"
        )));
    }
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        Path::new(primary_workspace_cwd).join(path)
    };
    let canonical = fs::canonicalize(&resolved)
        .map_err(|error| BackendFailure::Invalid(format!("{label} is unavailable: {error}")))?;
    if !canonical.is_dir() {
        return Err(BackendFailure::Invalid(format!(
            "{label} is not a directory"
        )));
    }
    canonical
        .to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| BackendFailure::Invalid(format!("{label} is not valid UTF-8")))
}

fn normalize_policy_roots(
    values: Vec<String>,
    read_roots: &[String],
    label: &str,
) -> Result<Vec<String>, BackendFailure> {
    let mut roots = Vec::with_capacity(values.len());
    for value in values {
        let root = require_canonical_directory(&value, label)?;
        if !covered_by(&root, read_roots) {
            return Err(BackendFailure::Denied(format!(
                "{label} {root} is outside sandbox read roots"
            )));
        }
        if roots.iter().any(|existing| existing == &root) {
            return Err(BackendFailure::Invalid(format!(
                "{label} list contains duplicate canonical roots"
            )));
        }
        roots.push(root);
    }
    roots.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    Ok(roots)
}

fn covered_by(path: &str, roots: &[String]) -> bool {
    roots
        .iter()
        .any(|root| Path::new(path).starts_with(Path::new(root)))
}

fn validate_job_id(id: &str) -> Result<(), BackendFailure> {
    if id.is_empty()
        || id.len() > 128
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(BackendFailure::Invalid(
            "job id must match [A-Za-z0-9_-]{1,128}".to_owned(),
        ));
    }
    Ok(())
}

fn drain_tail(mut reader: impl Read, path: &Path, cap: usize) -> Result<(), BackendFailure> {
    let mut tail = VecDeque::with_capacity(cap.min(8192));
    let mut buffer = [0_u8; 8192];
    loop {
        let count = reader.read(&mut buffer).map_err(io_failure)?;
        if count == 0 {
            break;
        }
        tail.extend(&buffer[..count]);
        while tail.len() > cap {
            tail.pop_front();
        }
        let bytes: Vec<_> = tail.iter().copied().collect();
        AtomicPublisher::replace(path, &bytes).map_err(store_failure)?;
    }
    Ok(())
}

fn signal_group(pid: u32, signal: i32) -> Result<(), BackendFailure> {
    let pid = i32::try_from(pid).map_err(|_| BackendFailure::Invalid("pid overflow".to_owned()))?;
    // SAFETY: negative pid selects the process group created by setsid; kill
    // retains no pointer and errors are checked.
    let result = unsafe { libc::kill(-pid, signal) };
    if result == 0 {
        Ok(())
    } else {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            Ok(())
        } else {
            Err(io_failure(error))
        }
    }
}

fn terminate_group(child: &mut Child, grace: Duration) {
    let _ = signal_group(child.id(), libc::SIGTERM);
    let deadline = Instant::now() + grace;
    while Instant::now() < deadline {
        if child.try_wait().ok().flatten().is_some() {
            return;
        }
        thread::sleep(Duration::from_millis(5));
    }
    let _ = signal_group(child.id(), libc::SIGKILL);
    let _ = child.wait();
}

#[cfg(target_os = "macos")]
fn process_start_token(pid: u32) -> Option<String> {
    let mut info = std::mem::MaybeUninit::<libc::proc_bsdinfo>::zeroed();
    // SAFETY: proc_pidinfo writes at most the supplied initialized allocation;
    // the value is read only when the full structure was returned.
    let count = unsafe {
        libc::proc_pidinfo(
            i32::try_from(pid).ok()?,
            libc::PROC_PIDTBSDINFO,
            0,
            info.as_mut_ptr().cast(),
            i32::try_from(std::mem::size_of::<libc::proc_bsdinfo>()).ok()?,
        )
    };
    if usize::try_from(count).ok()? != std::mem::size_of::<libc::proc_bsdinfo>() {
        return None;
    }
    // SAFETY: the size check above proves the kernel initialized the structure.
    let info = unsafe { info.assume_init() };
    Some(format!(
        "{}:{}",
        info.pbi_start_tvsec, info.pbi_start_tvusec
    ))
}

#[cfg(target_os = "linux")]
fn process_start_token(pid: u32) -> Option<String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let end = stat.rfind(')')?;
    stat[end + 2..]
        .split_whitespace()
        .nth(19)
        .map(ToOwned::to_owned)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn process_start_token(_pid: u32) -> Option<String> {
    None
}

fn unix_millis() -> Result<u64, BackendFailure> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| BackendFailure::Io(error.to_string()))?
        .as_millis();
    u64::try_from(millis).map_err(|_| BackendFailure::Limit("clock overflow".to_owned()))
}

fn sync_directory(path: &Path) -> Result<(), BackendFailure> {
    File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(io_failure)
}

fn io_failure(error: std::io::Error) -> BackendFailure {
    BackendFailure::Io(error.to_string())
}

fn store_failure(error: store::StoreError) -> BackendFailure {
    BackendFailure::Io(error.to_string())
}

#[cfg(test)]
mod disabled_job_launcher_tests {
    use super::{BackendFailure, HelperJobLauncher, JobSpec, SandboxedJobLauncher};
    use crate::{NetworkPolicy, SandboxPolicy};
    use std::collections::BTreeMap;

    #[test]
    fn mcp_only_launcher_refuses_command_execution() {
        let launcher = HelperJobLauncher::disabled(std::env::current_exe().unwrap()).unwrap();
        let spec = JobSpec {
            program: "/bin/true".to_owned(),
            args: Vec::new(),
            working_directory: None,
            writable_paths: Vec::new(),
            environment: BTreeMap::new(),
        };
        let policy = SandboxPolicy {
            format: 1,
            read_roots: vec!["/tmp".to_owned()],
            write_roots: Vec::new(),
            network: NetworkPolicy::Deny,
            allow_process: false,
            scratch: None,
        };
        assert!(matches!(
            launcher.runner_command(&spec, &policy),
            Err(BackendFailure::Unavailable(_))
        ));
    }
}

#[cfg(test)]
mod web_fetch_status_tests {
    use super::{BackendFailure, web_fetch_status_failure};
    use reqwest::StatusCode;

    /// A server that answered is not a protocol failure. Reporting every status
    /// as `Protocol` surfaced a plain 404 as "backend protocol failed"
    /// (2026-09-20) and hid that a 5xx is worth retrying.
    #[test]
    fn non_success_statuses_classify_by_what_the_server_said() {
        assert!(matches!(
            web_fetch_status_failure(StatusCode::NOT_FOUND),
            BackendFailure::NotFound(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::GONE),
            BackendFailure::NotFound(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::FORBIDDEN),
            BackendFailure::Denied(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::UNAUTHORIZED),
            BackendFailure::Denied(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::REQUEST_TIMEOUT),
            BackendFailure::Timeout
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::TOO_MANY_REQUESTS),
            BackendFailure::Limit(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::BAD_REQUEST),
            BackendFailure::Invalid(_)
        ));
        // Retryable: the origin accepted the request and failed to serve it.
        assert!(matches!(
            web_fetch_status_failure(StatusCode::SERVICE_UNAVAILABLE),
            BackendFailure::Unavailable(_)
        ));
        assert!(matches!(
            web_fetch_status_failure(StatusCode::INTERNAL_SERVER_ERROR),
            BackendFailure::Unavailable(_)
        ));
        // 1xx/3xx here really is an exchange that should not have reached us.
        assert!(matches!(
            web_fetch_status_failure(StatusCode::CONTINUE),
            BackendFailure::Protocol(_)
        ));
        assert_eq!(
            web_fetch_status_failure(StatusCode::NOT_FOUND).to_string(),
            "resource not found: HTTP status 404"
        );
    }
}
