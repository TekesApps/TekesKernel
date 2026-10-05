#[path = "helper/native_grep.rs"]
mod native_grep;

use std::collections::BTreeMap;
use std::ffi::{CString, OsStr, OsString};
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use thiserror::Error;

use store::FullSync;

use crate::sandbox::{
    ProbeStatus, SandboxApproval, SandboxPolicy, policy_digest, sandbox_command,
    validate_unsandboxed_approval,
};

pub const HELPER_PROTOCOL: &str = "tekes-exec-helper";
pub const HELPER_VERSION: u64 = 1;
pub const EX_PROTOCOL: i32 = 76;
const HARD_BYTES: usize = 16 * 1024 * 1024;
const HARD_READ_BYTES: usize = 32 * 1024 * 1024;
const HARD_TIMEOUT_MS: u64 = crate::runtime_backends::HARD_HELPER_TIMEOUT_MS;
const TERM_GRACE_MS: u64 = 250;
const HELPER_PROCESS_TIMEOUT_MS: u64 = HARD_TIMEOUT_MS + 1_000;
// One valid exec result may contain two 16 MiB binary streams encoded as
// base64 plus its JSON envelope.
const HELPER_PROTOCOL_STDOUT_BYTES: usize = 48 * 1024 * 1024;
const HELPER_PROTOCOL_STDERR_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ByteString {
    pub encoding: ByteEncoding,
    pub data: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ByteEncoding {
    Utf8,
    Base64,
}

impl ByteString {
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match std::str::from_utf8(bytes) {
            Ok(text) => Self {
                encoding: ByteEncoding::Utf8,
                data: text.to_owned(),
            },
            Err(_) => Self {
                encoding: ByteEncoding::Base64,
                data: BASE64.encode(bytes),
            },
        }
    }

    pub fn decode(&self) -> Result<Vec<u8>, HelperError> {
        match self.encoding {
            ByteEncoding::Utf8 => Ok(self.data.as_bytes().to_vec()),
            ByteEncoding::Base64 => BASE64.decode(&self.data).map_err(|_| {
                HelperError::new(
                    HelperErrorClass::InvalidRequest,
                    "invalid base64 byte string",
                )
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateMode {
    New,
    Replace,
    Upsert,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecRequest {
    pub argv: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<ByteString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<HelperPath>,
    pub env: BTreeMap<String, String>,
    pub stdout_bytes: u64,
    pub stderr_bytes: u64,
    /// Absent means the command runs to completion. A long build or test run is
    /// not a hung one, and only the caller knows which it is; the turn's own
    /// stop path remains the way to end it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelperPath {
    pub root: String,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelperOperation {
    Read {
        root: String,
        path: String,
        max_bytes: u64,
    },
    Write {
        root: String,
        path: String,
        content: ByteString,
        create: CreateMode,
    },
    Patch {
        root: String,
        path: String,
        expected_sha256: String,
        replacement: ByteString,
    },
    Glob {
        root: String,
        pattern: String,
        max_entries: u64,
    },
    Grep {
        root: String,
        path: String,
        pattern: String,
        glob: Option<String>,
        output_mode: String,
        case_insensitive: bool,
        context_lines: u64,
        stdout_bytes: u64,
        timeout_ms: u64,
    },
    Exec(ExecRequest),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HelperRequest {
    pub id: String,
    pub operation: HelperOperation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadValue {
    pub content: ByteString,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WriteValue {
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecValue {
    pub status: i32,
    pub stdout: ByteString,
    pub stderr: ByteString,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelperValue {
    Read(ReadValue),
    Write(WriteValue),
    Patch(WriteValue),
    Glob { paths: Vec<String> },
    Exec(ExecValue),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelperErrorClass {
    InvalidRequest,
    UnknownRoot,
    PathEscape,
    Symlink,
    NotRegular,
    Exists,
    Missing,
    DigestMismatch,
    Limit,
    Timeout,
    Denied,
    Io,
    Protocol,
    SandboxUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Error)]
#[error("{class:?}: {message}")]
#[serde(deny_unknown_fields)]
pub struct HelperError {
    pub class: HelperErrorClass,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Error)]
pub enum HelperClientError {
    #[error("helper invocation cancelled")]
    Cancelled,
    #[error(transparent)]
    Helper(#[from] HelperError),
}

impl HelperError {
    #[must_use]
    pub fn new(class: HelperErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
            retryable: false,
        }
    }

    fn io(error: std::io::Error) -> Self {
        let class = match error.kind() {
            std::io::ErrorKind::NotFound => HelperErrorClass::Missing,
            std::io::ErrorKind::AlreadyExists => HelperErrorClass::Exists,
            std::io::ErrorKind::PermissionDenied => HelperErrorClass::Denied,
            _ => HelperErrorClass::Io,
        };
        Self::new(class, error.to_string())
    }
}

impl From<std::io::Error> for HelperError {
    fn from(error: std::io::Error) -> Self {
        Self::io(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HelperResponse {
    Result { id: String, value: HelperValue },
    Error { id: String, error: HelperError },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Hello {
    proto: String,
    min: u64,
    max: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Selected {
    version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reject {
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultPayload {
    id: String,
    value: HelperValue,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ErrorPayload {
    id: String,
    #[serde(flatten)]
    error: HelperError,
}

pub fn encode_helper_line<T: Serialize>(key: &str, payload: &T) -> Result<Vec<u8>, HelperError> {
    let mut object = Map::new();
    object.insert(
        key.to_owned(),
        serde_json::to_value(payload)
            .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))?,
    );
    let mut bytes = serde_json_canonicalizer::to_vec(&Value::Object(object))
        .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn decode_helper_line(bytes: &[u8]) -> Result<(String, Value), HelperError> {
    let Some(body) = bytes.strip_suffix(b"\n") else {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "frame requires final LF",
        ));
    };
    if body.is_empty() || body.contains(&b'\n') || body.contains(&b'\r') {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "frame must be exactly one line",
        ));
    }
    let parsed = schema::IJsonValue::parse(body)
        .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))?;
    if parsed
        .canonical_bytes()
        .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))?
        != body
    {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "frame is not canonical",
        ));
    }
    let value: Value = serde_json::from_slice(body)
        .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))?;
    let object = value
        .as_object()
        .ok_or_else(|| HelperError::new(HelperErrorClass::Protocol, "frame must be an object"))?;
    if object.len() != 1 {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "frame must have one top-level key",
        ));
    }
    Ok(object
        .iter()
        .next()
        .map(|(key, value)| (key.clone(), value.clone()))
        .expect("one key"))
}

#[derive(Debug)]
pub struct RootBinding {
    name: String,
    path: PathBuf,
    directory: File,
}

impl RootBinding {
    pub fn open(name: impl Into<String>, path: impl AsRef<Path>) -> Result<Self, HelperError> {
        let name = name.into();
        if !valid_root_name(&name) {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "invalid root name",
            ));
        }
        let path = path.as_ref();
        if !path.is_absolute() || path.to_str().is_none() {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "root path must be absolute UTF-8",
            ));
        }
        if path.components().any(|component| {
            matches!(
                component,
                Component::CurDir | Component::ParentDir | Component::Prefix(_)
            )
        }) {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "root path must already be canonical",
            ));
        }
        let directory =
            open_directory_at(libc::AT_FDCWD, path.as_os_str()).map_err(|mut error| {
                error.message = format!("open root {}: {}", path.display(), error.message);
                error
            })?;
        Ok(Self {
            name,
            path: path.to_path_buf(),
            directory,
        })
    }
}

#[derive(Debug)]
pub struct HelperServer {
    roots: BTreeMap<String, RootBinding>,
}

impl HelperServer {
    pub fn new(roots: impl IntoIterator<Item = RootBinding>) -> Result<Self, HelperError> {
        let mut map = BTreeMap::new();
        for root in roots {
            if map.insert(root.name.clone(), root).is_some() {
                return Err(HelperError::new(
                    HelperErrorClass::InvalidRequest,
                    "duplicate root name",
                ));
            }
        }
        Ok(Self { roots: map })
    }

    pub fn execute(&self, request: &HelperRequest) -> HelperResponse {
        let result = self.execute_inner(request);
        match result {
            Ok(value) => HelperResponse::Result {
                id: request.id.clone(),
                value,
            },
            Err(error) => HelperResponse::Error {
                id: request.id.clone(),
                error,
            },
        }
    }

    fn execute_inner(&self, request: &HelperRequest) -> Result<HelperValue, HelperError> {
        if request.id.is_empty() {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "request id must be non-empty",
            ));
        }
        match &request.operation {
            HelperOperation::Read {
                root,
                path,
                max_bytes,
            } => {
                let cap = checked_cap_with(*max_bytes, HARD_READ_BYTES)?;
                let root = self.root(root)?;
                let bytes = read_regular(root.directory.as_raw_fd(), path, cap)?;
                Ok(HelperValue::Read(ReadValue {
                    content: ByteString::from_bytes(&bytes),
                    bytes: bytes.len() as u64,
                    sha256: sha256(&bytes),
                }))
            }
            HelperOperation::Write {
                root,
                path,
                content,
                create,
            } => {
                let root = self.root(root)?;
                let bytes = content.decode()?;
                check_content_cap(&bytes)?;
                atomic_replace(root.directory.as_raw_fd(), path, &bytes, *create)?;
                Ok(HelperValue::Write(WriteValue {
                    bytes: bytes.len() as u64,
                    sha256: sha256(&bytes),
                }))
            }
            HelperOperation::Patch {
                root,
                path,
                expected_sha256,
                replacement,
            } => {
                let root = self.root(root)?;
                let old = read_regular(root.directory.as_raw_fd(), path, HARD_BYTES)?;
                if &sha256(&old) != expected_sha256 {
                    return Err(HelperError::new(
                        HelperErrorClass::DigestMismatch,
                        "patch preimage digest mismatch",
                    ));
                }
                let bytes = replacement.decode()?;
                check_content_cap(&bytes)?;
                atomic_replace(
                    root.directory.as_raw_fd(),
                    path,
                    &bytes,
                    CreateMode::Replace,
                )?;
                Ok(HelperValue::Patch(WriteValue {
                    bytes: bytes.len() as u64,
                    sha256: sha256(&bytes),
                }))
            }
            HelperOperation::Glob {
                root,
                pattern,
                max_entries,
            } => {
                let cap = checked_cap(*max_entries)?;
                let root = self.root(root)?;
                let paths = glob_beneath(&root.path, pattern, cap)?;
                Ok(HelperValue::Glob { paths })
            }
            HelperOperation::Grep {
                root,
                path,
                pattern,
                glob,
                output_mode,
                case_insensitive,
                context_lines,
                stdout_bytes,
                timeout_ms,
            } => {
                let root = self.root(root)?;
                native_grep::search(
                    root,
                    path,
                    pattern,
                    glob.as_deref(),
                    output_mode,
                    *case_insensitive,
                    *context_lines,
                    *stdout_bytes,
                    *timeout_ms,
                )
                .map(HelperValue::Exec)
            }
            HelperOperation::Exec(request) => Ok(HelperValue::Exec(self.exec(request)?)),
        }
    }

    fn root(&self, name: &str) -> Result<&RootBinding, HelperError> {
        self.roots.get(name).ok_or_else(|| {
            HelperError::new(
                HelperErrorClass::UnknownRoot,
                format!("unknown root {name:?}"),
            )
        })
    }

    fn exec(&self, request: &ExecRequest) -> Result<ExecValue, HelperError> {
        if request.argv.first().is_none_or(String::is_empty) {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "exec argv must be non-empty",
            ));
        }
        let stdout_cap = checked_cap(request.stdout_bytes)?;
        let stderr_cap = checked_cap(request.stderr_bytes)?;
        if request.timeout_ms == Some(0) {
            return Err(HelperError::new(
                HelperErrorClass::Limit,
                "timeout is outside hard limits",
            ));
        }
        let mut command = Command::new(&request.argv[0]);
        command
            .args(&request.argv[1..])
            .env_clear()
            .envs(&request.env)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command.process_group(0);
        if let Some(cwd) = &request.cwd {
            let root = self.root(&cwd.root)?;
            // An empty relative cwd names the already-authorized root itself.
            // File operations still reject empty paths; only exec has this case.
            let relative = if cwd.path.is_empty() {
                PathBuf::new()
            } else {
                normalized_relative(&cwd.path)?
            };
            let components = relative
                .components()
                .map(|component| match component {
                    Component::Normal(name) => name.to_os_string(),
                    _ => unreachable!("cwd was normalized"),
                })
                .collect::<Vec<_>>();
            let directory = open_parent(root.directory.as_raw_fd(), &components)?;
            // SAFETY: the closure owns a live directory fd and calls only the
            // async-signal-safe fchdir before exec. O_NOFOLLOW was enforced on
            // every component. No ambient ancestor lookup or parent cwd change
            // is needed, and the fd remains alive until the spawn completes.
            unsafe {
                command.pre_exec(move || {
                    if libc::fchdir(directory.as_raw_fd()) == -1 {
                        Err(std::io::Error::last_os_error())
                    } else {
                        Ok(())
                    }
                });
            }
        }
        let mut child = command.spawn().map_err(|error| {
            let mut error = HelperError::io(error);
            error.message = format!("spawn exec program: {}", error.message);
            error
        })?;
        if let Some(input) = &request.stdin {
            let bytes = input.decode()?;
            check_content_cap(&bytes)?;
            child
                .stdin
                .take()
                .ok_or_else(|| HelperError::new(HelperErrorClass::Io, "child stdin missing"))?
                .write_all(&bytes)
                .map_err(HelperError::io)?;
        } else {
            drop(child.stdin.take());
        }
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| HelperError::new(HelperErrorClass::Io, "child stdout missing"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| HelperError::new(HelperErrorClass::Io, "child stderr missing"))?;
        let stdout_thread = thread::spawn(move || read_capped(stdout, stdout_cap));
        let stderr_thread = thread::spawn(move || read_capped(stderr, stderr_cap));
        let deadline = request
            .timeout_ms
            .map(|timeout| Instant::now() + Duration::from_millis(timeout));
        let status = loop {
            if let Some(status) = child.try_wait().map_err(HelperError::io)? {
                break status;
            }
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                terminate_then_kill(&mut child);
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                return Err(HelperError::new(
                    HelperErrorClass::Timeout,
                    "exec timed out",
                ));
            }
            thread::sleep(Duration::from_millis(5));
        };
        let (stdout, stdout_truncated) = stdout_thread
            .join()
            .map_err(|_| HelperError::new(HelperErrorClass::Io, "stdout reader panicked"))?
            .map_err(HelperError::io)?;
        let (stderr, stderr_truncated) = stderr_thread
            .join()
            .map_err(|_| HelperError::new(HelperErrorClass::Io, "stderr reader panicked"))?
            .map_err(HelperError::io)?;
        Ok(ExecValue {
            status: status.code().unwrap_or(-1),
            stdout: ByteString::from_bytes(&stdout),
            stderr: ByteString::from_bytes(&stderr),
            stdout_truncated,
            stderr_truncated,
        })
    }
}

#[derive(Clone, Debug)]
pub struct HelperClient {
    scratch_owner: Option<std::sync::Arc<tempfile::TempDir>>,
    executable: PathBuf,
    roots: Vec<(String, PathBuf)>,
    isolation: HelperIsolation,
}

#[derive(Clone, Debug)]
enum HelperIsolation {
    Sandboxed {
        policy: SandboxPolicy,
        probe: ProbeStatus,
    },
    ApprovedUnsandboxed,
}

impl HelperClient {
    /// Own a private scratch directory until every client clone has finished.
    pub fn sandboxed_with_scratch(
        executable: impl Into<PathBuf>,
        roots: Vec<(String, PathBuf)>,
        mut policy: SandboxPolicy,
        probe: ProbeStatus,
    ) -> Result<Self, HelperError> {
        if policy.scratch.is_some() {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "managed scratch requires an unset scratch path",
            ));
        }
        let scratch = tempfile::Builder::new()
            .prefix("tekes-tool-")
            .permissions(std::fs::Permissions::from_mode(0o700))
            .tempdir()?;
        policy.scratch = Some(
            std::fs::canonicalize(scratch.path())?
                .to_string_lossy()
                .into_owned(),
        );
        let mut client = Self::sandboxed(executable, roots, policy, probe)?;
        client.scratch_owner = Some(std::sync::Arc::new(scratch));
        Ok(client)
    }

    pub fn scratch_path(&self) -> Option<&str> {
        match &self.isolation {
            HelperIsolation::Sandboxed { policy, .. } => policy.scratch.as_deref(),
            HelperIsolation::ApprovedUnsandboxed => None,
        }
    }

    pub fn sandboxed(
        executable: impl Into<PathBuf>,
        roots: Vec<(String, PathBuf)>,
        policy: SandboxPolicy,
        probe: ProbeStatus,
    ) -> Result<Self, HelperError> {
        policy.validate().map_err(|error| {
            HelperError::new(HelperErrorClass::InvalidRequest, error.to_string())
        })?;
        if !matches!(probe, ProbeStatus::Available { .. }) {
            return Err(HelperError::new(
                HelperErrorClass::SandboxUnavailable,
                "sandbox probe is not available",
            ));
        }
        Ok(Self {
            executable: executable.into(),
            roots,
            isolation: HelperIsolation::Sandboxed { policy, probe },
            scratch_owner: None,
        })
    }

    pub fn approved_unsandboxed(
        executable: impl Into<PathBuf>,
        roots: Vec<(String, PathBuf)>,
        policy: &SandboxPolicy,
        approval: &SandboxApproval,
        run: &str,
        call: &str,
    ) -> Result<Self, HelperError> {
        let digest = policy_digest(policy).map_err(|error| {
            HelperError::new(HelperErrorClass::InvalidRequest, error.to_string())
        })?;
        validate_unsandboxed_approval(approval, run, call, &digest)
            .map_err(|error| HelperError::new(HelperErrorClass::Denied, error.to_string()))?;
        Ok(Self {
            executable: executable.into(),
            roots,
            isolation: HelperIsolation::ApprovedUnsandboxed,
            scratch_owner: None,
        })
    }

    pub fn execute(&self, request: &HelperRequest) -> Result<HelperResponse, HelperError> {
        self.execute_cancellable(
            request,
            Some(Duration::from_millis(HELPER_PROCESS_TIMEOUT_MS)),
            || false,
        )
        .map_err(|error| match error {
            HelperClientError::Helper(error) => error,
            HelperClientError::Cancelled => {
                HelperError::new(HelperErrorClass::Protocol, "unexpected helper cancellation")
            }
        })
    }

    /// `timeout` bounds the helper process itself. `None` means the request
    /// carries no deadline and the helper runs until it finishes or the caller
    /// cancels: a long command is not a hung one.
    pub fn execute_cancellable<F>(
        &self,
        request: &HelperRequest,
        timeout: Option<Duration>,
        cancelled: F,
    ) -> Result<HelperResponse, HelperClientError>
    where
        F: Fn() -> bool,
    {
        if timeout.is_some_and(|timeout| timeout.is_zero()) {
            return Err(HelperError::new(
                HelperErrorClass::InvalidRequest,
                "helper process timeout is outside hard limits",
            )
            .into());
        }
        let hello_line = encode_helper_line(
            "hello",
            &Hello {
                proto: HELPER_PROTOCOL.to_owned(),
                min: 1,
                max: 1,
            },
        )?;
        let request_line = encode_helper_line("request", request)?;
        let mut command = match &self.isolation {
            HelperIsolation::Sandboxed { policy, probe } => {
                sandbox_command(policy, probe, &self.executable).map_err(|error| {
                    HelperError::new(HelperErrorClass::SandboxUnavailable, error.to_string())
                })?
            }
            HelperIsolation::ApprovedUnsandboxed => Command::new(&self.executable),
        };
        for (name, path) in &self.roots {
            command
                .arg("--root")
                .arg(format!("{name}={}", path.display()));
        }
        command.process_group(0);
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(HelperError::io)?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| HelperError::new(HelperErrorClass::Protocol, "helper stdin missing"))?;
        if let Err(error) = stdin.write_all(&hello_line) {
            terminate_then_kill(&mut child);
            return Err(HelperError::io(error).into());
        }
        if let Err(error) = stdin.write_all(&request_line) {
            terminate_then_kill(&mut child);
            return Err(HelperError::io(error).into());
        }
        drop(stdin);
        let Some(stdout) = child.stdout.take() else {
            terminate_then_kill(&mut child);
            return Err(
                HelperError::new(HelperErrorClass::Protocol, "helper stdout missing").into(),
            );
        };
        let Some(stderr) = child.stderr.take() else {
            terminate_then_kill(&mut child);
            return Err(
                HelperError::new(HelperErrorClass::Protocol, "helper stderr missing").into(),
            );
        };
        let stdout_thread =
            thread::spawn(move || read_capped(stdout, HELPER_PROTOCOL_STDOUT_BYTES));
        let stderr_thread =
            thread::spawn(move || read_capped(stderr, HELPER_PROTOCOL_STDERR_BYTES));
        let deadline = timeout.map(|timeout| Instant::now() + timeout);
        let status = loop {
            if cancelled() {
                terminate_then_kill(&mut child);
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                return Err(HelperClientError::Cancelled);
            }
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                terminate_then_kill(&mut child);
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                return Err(HelperError::new(
                    HelperErrorClass::Timeout,
                    "helper process timed out",
                )
                .into());
            }
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {}
                Err(error) => {
                    terminate_then_kill(&mut child);
                    let _ = stdout_thread.join();
                    let _ = stderr_thread.join();
                    return Err(HelperError::io(error).into());
                }
            }
            thread::sleep(Duration::from_millis(5));
        };
        let (stdout, stdout_truncated) = stdout_thread
            .join()
            .map_err(|_| HelperError::new(HelperErrorClass::Io, "helper stdout reader panicked"))?
            .map_err(HelperError::io)?;
        let (stderr, stderr_truncated) = stderr_thread
            .join()
            .map_err(|_| HelperError::new(HelperErrorClass::Io, "helper stderr reader panicked"))?
            .map_err(HelperError::io)?;
        if stdout_truncated || stderr_truncated {
            return Err(HelperError::new(
                HelperErrorClass::Limit,
                "helper protocol output exceeded its cap",
            )
            .into());
        }
        if !status.success() {
            return Err(HelperError::new(
                HelperErrorClass::Protocol,
                format!(
                    "helper exited {}: {}",
                    status.code().unwrap_or(-1),
                    String::from_utf8_lossy(&stderr)
                ),
            )
            .into());
        }
        let mut lines = stdout.split_inclusive(|byte| *byte == b'\n');
        let selected = lines.next().ok_or_else(|| {
            HelperError::new(HelperErrorClass::Protocol, "missing selected response")
        })?;
        let (key, payload) = decode_helper_line(selected)?;
        if key != "selected" {
            return Err(
                HelperError::new(HelperErrorClass::Protocol, "expected selected response").into(),
            );
        }
        let selected: Selected = from_payload(payload)?;
        if selected.version != HELPER_VERSION {
            return Err(HelperError::new(
                HelperErrorClass::Protocol,
                "helper selected unsupported version",
            )
            .into());
        }
        let terminal = lines.next().ok_or_else(|| {
            HelperError::new(HelperErrorClass::Protocol, "missing terminal response")
        })?;
        if lines.next().is_some() {
            return Err(HelperError::new(HelperErrorClass::Protocol, "extra helper output").into());
        }
        Ok(decode_terminal(terminal)?)
    }
}

pub fn serve_stdio(server: &HelperServer) -> Result<(), HelperError> {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    let hello_line = read_line_limited(&mut input, 64 * 1024)?;
    let (key, payload) = decode_helper_line(&hello_line)?;
    if key != "hello" {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "expected hello",
        ));
    }
    let hello: Hello = from_payload(payload)?;
    if hello.proto != HELPER_PROTOCOL || hello.min > HELPER_VERSION || hello.max < HELPER_VERSION {
        output.write_all(&encode_helper_line(
            "reject",
            &Reject {
                reason: "no mutual protocol version".to_owned(),
            },
        )?)?;
        output.flush()?;
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "no mutual protocol version",
        ));
    }
    output.write_all(&encode_helper_line(
        "selected",
        &Selected {
            version: HELPER_VERSION,
        },
    )?)?;
    output.flush()?;
    let request_line = read_line_limited(&mut input, HARD_BYTES)?;
    let (key, payload) = decode_helper_line(&request_line)?;
    if key != "request" {
        return Err(HelperError::new(
            HelperErrorClass::Protocol,
            "expected request",
        ));
    }
    let request: HelperRequest = from_payload(payload)?;
    let response = server.execute(&request);
    match response {
        HelperResponse::Result { id, value } => {
            output.write_all(&encode_helper_line("result", &ResultPayload { id, value })?)?
        }
        HelperResponse::Error { id, error } => {
            output.write_all(&encode_helper_line("error", &ErrorPayload { id, error })?)?
        }
    }
    output.flush()?;
    Ok(())
}

fn decode_terminal(bytes: &[u8]) -> Result<HelperResponse, HelperError> {
    let (key, payload) = decode_helper_line(bytes)?;
    match key.as_str() {
        "result" => {
            let result: ResultPayload = from_payload(payload)?;
            Ok(HelperResponse::Result {
                id: result.id,
                value: result.value,
            })
        }
        "error" => {
            let error: ErrorPayload = from_payload(payload)?;
            Ok(HelperResponse::Error {
                id: error.id,
                error: error.error,
            })
        }
        _ => Err(HelperError::new(
            HelperErrorClass::Protocol,
            "expected result or error",
        )),
    }
}

fn from_payload<T: DeserializeOwned>(value: Value) -> Result<T, HelperError> {
    serde_json::from_value(value)
        .map_err(|error| HelperError::new(HelperErrorClass::Protocol, error.to_string()))
}

fn read_line_limited(reader: &mut impl Read, cap: usize) -> Result<Vec<u8>, HelperError> {
    let mut output = Vec::new();
    let mut byte = [0u8; 1];
    while output.len() <= cap {
        let count = reader.read(&mut byte).map_err(HelperError::io)?;
        if count == 0 {
            break;
        }
        output.push(byte[0]);
        if byte[0] == b'\n' {
            return Ok(output);
        }
    }
    Err(HelperError::new(
        HelperErrorClass::Limit,
        "frame exceeds cap or lacks LF",
    ))
}

fn valid_root_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|value| value.is_ascii_alphabetic())
        && name.len() <= 32
        && chars.all(|value| value.is_ascii_alphanumeric() || matches!(value, '_' | '-'))
}

fn normalized_relative(path: &str) -> Result<PathBuf, HelperError> {
    let path = Path::new(path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(HelperError::new(
            HelperErrorClass::PathEscape,
            "path must be non-empty and relative",
        ));
    }
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) if !value.as_bytes().contains(&0) => result.push(value),
            _ => {
                return Err(HelperError::new(
                    HelperErrorClass::PathEscape,
                    "path contains a forbidden component",
                ));
            }
        }
    }
    Ok(result)
}

fn split_parent(path: &str) -> Result<(Vec<OsString>, OsString), HelperError> {
    let normalized = normalized_relative(path)?;
    let mut components = normalized.components().collect::<Vec<_>>();
    let leaf = match components.pop() {
        Some(Component::Normal(value)) => value.to_os_string(),
        _ => {
            return Err(HelperError::new(
                HelperErrorClass::PathEscape,
                "path has no leaf",
            ));
        }
    };
    let parents = components
        .into_iter()
        .map(|component| match component {
            Component::Normal(value) => value.to_os_string(),
            _ => unreachable!("normalized components"),
        })
        .collect();
    Ok((parents, leaf))
}

fn open_parent(root: RawFd, components: &[OsString]) -> Result<File, HelperError> {
    let mut current = duplicate_fd(root)?;
    for component in components {
        current = open_directory_at(current.as_raw_fd(), component.as_os_str())?;
    }
    Ok(current)
}

fn create_parent(root: RawFd, components: &[OsString]) -> Result<File, HelperError> {
    let mut current = duplicate_fd(root)?;
    for component in components {
        current = match open_directory_at(current.as_raw_fd(), component.as_os_str()) {
            Ok(directory) => directory,
            Err(error) if error.class == HelperErrorClass::Missing => {
                let name = CString::new(component.as_bytes()).map_err(|_| {
                    HelperError::new(HelperErrorClass::PathEscape, "path contains NUL")
                })?;
                // SAFETY: current is an owned directory fd and name is NUL terminated.
                if unsafe { libc::mkdirat(current.as_raw_fd(), name.as_ptr(), 0o755) } != 0 {
                    let error = std::io::Error::last_os_error();
                    if error.raw_os_error() != Some(libc::EEXIST) {
                        return Err(classify_open(error));
                    }
                }
                // Reopen without following links, including a racing replacement.
                let directory = open_directory_at(current.as_raw_fd(), component.as_os_str())?;
                current.sync_all().map_err(HelperError::io)?;
                directory
            }
            Err(error) => return Err(error),
        };
    }
    Ok(current)
}

fn duplicate_fd(fd: RawFd) -> Result<File, HelperError> {
    // SAFETY: `fd` is live; F_DUPFD_CLOEXEC returns an independently owned fd.
    let copy = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if copy < 0 {
        return Err(HelperError::io(std::io::Error::last_os_error()));
    }
    // SAFETY: `copy` is a newly owned descriptor on success above.
    Ok(unsafe { File::from_raw_fd(copy) })
}

fn open_directory_at(parent: RawFd, name: &OsStr) -> Result<File, HelperError> {
    let name = CString::new(name.as_bytes())
        .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "path contains NUL"))?;
    // SAFETY: name is live and NUL-terminated; returned fd is owned on success.
    let fd = unsafe {
        libc::openat(
            parent,
            name.as_ptr(),
            libc::O_RDONLY
                | libc::O_DIRECTORY
                | libc::O_NOFOLLOW
                | libc::O_NONBLOCK
                | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(classify_open(std::io::Error::last_os_error()));
    }
    // SAFETY: `fd` is newly owned by this process.
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn open_regular_at(
    parent: RawFd,
    leaf: &OsStr,
    flags: i32,
    mode: u32,
) -> Result<File, HelperError> {
    let leaf = CString::new(leaf.as_bytes())
        .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "path contains NUL"))?;
    // SAFETY: leaf is live and NUL-terminated; returned fd is owned on success.
    let fd = unsafe {
        libc::openat(
            parent,
            leaf.as_ptr(),
            flags | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC,
            mode,
        )
    };
    if fd < 0 {
        return Err(classify_open(std::io::Error::last_os_error()));
    }
    // SAFETY: `fd` is newly owned by this process.
    let file = unsafe { File::from_raw_fd(fd) };
    let metadata = file.metadata().map_err(HelperError::io)?;
    if !metadata.file_type().is_file() {
        return Err(HelperError::new(
            HelperErrorClass::NotRegular,
            "opened descriptor is not a regular file",
        ));
    }
    Ok(file)
}

fn classify_open(error: std::io::Error) -> HelperError {
    match error.raw_os_error() {
        Some(libc::ELOOP) => HelperError::new(HelperErrorClass::Symlink, error.to_string()),
        Some(libc::ENOTDIR) => HelperError::new(HelperErrorClass::NotRegular, error.to_string()),
        _ => HelperError::io(error),
    }
}

fn read_regular(root: RawFd, path: &str, cap: usize) -> Result<Vec<u8>, HelperError> {
    let (parents, leaf) = split_parent(path)?;
    let parent = open_parent(root, &parents)?;
    let file = open_regular_at(parent.as_raw_fd(), &leaf, libc::O_RDONLY, 0)?;
    if file.metadata().map_err(HelperError::io)?.len() > cap as u64 {
        return Err(HelperError::new(
            HelperErrorClass::Limit,
            "file exceeds read cap",
        ));
    }
    let mut bytes = Vec::new();
    file.take((cap as u64) + 1)
        .read_to_end(&mut bytes)
        .map_err(HelperError::io)?;
    if bytes.len() > cap {
        return Err(HelperError::new(
            HelperErrorClass::Limit,
            "file exceeds read cap",
        ));
    }
    Ok(bytes)
}

fn atomic_replace(
    root: RawFd,
    path: &str,
    bytes: &[u8],
    create: CreateMode,
) -> Result<(), HelperError> {
    let (parents, leaf) = split_parent(path)?;
    let parent = if create == CreateMode::New {
        create_parent(root, &parents)?
    } else {
        open_parent(root, &parents)?
    };
    let existing = open_regular_at(parent.as_raw_fd(), &leaf, libc::O_RDONLY, 0);
    match (create, &existing) {
        (CreateMode::New, Ok(_)) => {
            return Err(HelperError::new(
                HelperErrorClass::Exists,
                "destination already exists",
            ));
        }
        (CreateMode::Replace, Err(error)) if error.class == HelperErrorClass::Missing => {
            return Err(error.clone());
        }
        (_, Err(error)) if error.class != HelperErrorClass::Missing => return Err(error.clone()),
        _ => {}
    }
    let temp_name = format!(".tekes-tmp-{}-{}", std::process::id(), monotonic_nonce());
    let temp_os = OsStr::new(&temp_name);
    let mut temp = open_regular_at(
        parent.as_raw_fd(),
        temp_os,
        libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
        0o600,
    )?;
    let result = (|| {
        temp.write_all(bytes).map_err(HelperError::io)?;
        FullSync::full_sync(&temp).map_err(HelperError::io)?;
        rename_at(
            parent.as_raw_fd(),
            temp_os,
            &leaf,
            create == CreateMode::New,
        )?;
        FullSync::full_sync(&parent).map_err(HelperError::io)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = unlink_at(parent.as_raw_fd(), temp_os);
    }
    result
}

fn rename_at(
    parent: RawFd,
    source: &OsStr,
    destination: &OsStr,
    exclusive: bool,
) -> Result<(), HelperError> {
    let source = CString::new(source.as_bytes())
        .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "temp contains NUL"))?;
    let destination = CString::new(destination.as_bytes())
        .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "destination contains NUL"))?;
    #[cfg(target_os = "macos")]
    let result = unsafe {
        // SAFETY: both names are live, NUL-terminated, and relative to live parent fds.
        libc::renameatx_np(
            parent,
            source.as_ptr(),
            parent,
            destination.as_ptr(),
            if exclusive { libc::RENAME_EXCL } else { 0 },
        )
    };
    #[cfg(not(target_os = "macos"))]
    let result = if exclusive {
        // SAFETY: names and descriptors satisfy linkat/unlinkat contracts.
        let linked =
            unsafe { libc::linkat(parent, source.as_ptr(), parent, destination.as_ptr(), 0) };
        if linked == 0 {
            // SAFETY: the temp name is still present in the same live directory.
            unsafe { libc::unlinkat(parent, source.as_ptr(), 0) }
        } else {
            linked
        }
    } else {
        // SAFETY: names are live and relative to live parent descriptors.
        unsafe { libc::renameat(parent, source.as_ptr(), parent, destination.as_ptr()) }
    };
    if result != 0 {
        return Err(HelperError::io(std::io::Error::last_os_error()));
    }
    Ok(())
}

fn unlink_at(parent: RawFd, name: &OsStr) -> Result<(), HelperError> {
    let name = CString::new(name.as_bytes())
        .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "name contains NUL"))?;
    // SAFETY: name is live and relative to a live directory descriptor.
    if unsafe { libc::unlinkat(parent, name.as_ptr(), 0) } != 0 {
        return Err(HelperError::io(std::io::Error::last_os_error()));
    }
    Ok(())
}

fn glob_beneath(root: &Path, pattern: &str, cap: usize) -> Result<Vec<String>, HelperError> {
    if pattern.is_empty() || Path::new(pattern).is_absolute() || pattern.contains("..") {
        return Err(HelperError::new(
            HelperErrorClass::PathEscape,
            "invalid glob pattern",
        ));
    }
    let mut stack = vec![root.to_path_buf()];
    let mut result = Vec::new();
    let mut scanned = 0usize;
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(&directory).map_err(HelperError::io)? {
            let entry = entry.map_err(HelperError::io)?;
            scanned = scanned.saturating_add(1);
            if scanned > 100_000 {
                return Err(HelperError::new(
                    HelperErrorClass::Limit,
                    "glob scanned-entry limit exceeded",
                ));
            }
            let file_type = entry.file_type().map_err(HelperError::io)?;
            if file_type.is_symlink() {
                continue;
            }
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|_| HelperError::new(HelperErrorClass::PathEscape, "glob escaped root"))?
                .to_string_lossy()
                .replace('\\', "/");
            if glob_match(pattern.as_bytes(), relative.as_bytes()) {
                let modified = entry
                    .metadata()
                    .and_then(|metadata| metadata.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                result.push((modified, relative.clone()));
            }
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }
    result.sort_by(|(left_time, left_path), (right_time, right_path)| {
        right_time
            .cmp(left_time)
            .then_with(|| left_path.as_bytes().cmp(right_path.as_bytes()))
    });
    result.truncate(cap);
    Ok(result.into_iter().map(|(_, path)| path).collect())
}

fn glob_match(pattern: &[u8], text: &[u8]) -> bool {
    fn matches(
        pattern: &[u8],
        text: &[u8],
        pattern_index: usize,
        text_index: usize,
        memo: &mut BTreeMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(result) = memo.get(&(pattern_index, text_index)) {
            return *result;
        }
        let result = if pattern_index == pattern.len() {
            text_index == text.len()
        } else if pattern[pattern_index..].starts_with(b"**/") {
            matches(pattern, text, pattern_index + 3, text_index, memo)
                || (text_index < text.len()
                    && matches(pattern, text, pattern_index, text_index + 1, memo))
        } else if pattern[pattern_index..].starts_with(b"**") {
            matches(pattern, text, pattern_index + 2, text_index, memo)
                || (text_index < text.len()
                    && matches(pattern, text, pattern_index, text_index + 1, memo))
        } else if pattern[pattern_index] == b'*' {
            matches(pattern, text, pattern_index + 1, text_index, memo)
                || (text_index < text.len()
                    && text[text_index] != b'/'
                    && matches(pattern, text, pattern_index, text_index + 1, memo))
        } else if text_index < text.len()
            && (pattern[pattern_index] == b'?' && text[text_index] != b'/'
                || pattern[pattern_index] == text[text_index])
        {
            matches(pattern, text, pattern_index + 1, text_index + 1, memo)
        } else {
            false
        };
        memo.insert((pattern_index, text_index), result);
        result
    }
    matches(pattern, text, 0, 0, &mut BTreeMap::new())
}

fn read_capped(mut reader: impl Read, cap: usize) -> std::io::Result<(Vec<u8>, bool)> {
    let mut output = Vec::with_capacity(cap.min(8192));
    let mut truncated = false;
    let mut buffer = [0u8; 8192];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let remaining = cap.saturating_sub(output.len());
        output.extend_from_slice(&buffer[..count.min(remaining)]);
        truncated |= count > remaining;
    }
    Ok((output, truncated))
}

fn terminate_then_kill(child: &mut std::process::Child) {
    let group = -(child.id() as i32);
    // SAFETY: the child created its own process group; kill does not retain pointers.
    unsafe {
        libc::kill(group, libc::SIGTERM);
    }
    let deadline = Instant::now() + Duration::from_millis(TERM_GRACE_MS);
    while Instant::now() < deadline {
        if child.try_wait().ok().flatten().is_some() {
            return;
        }
        thread::sleep(Duration::from_millis(5));
    }
    // SAFETY: the child process group remains identified by its negated pid.
    unsafe {
        libc::kill(group, libc::SIGKILL);
    }
    let _ = child.wait();
}

fn checked_cap(value: u64) -> Result<usize, HelperError> {
    checked_cap_with(value, HARD_BYTES)
}

fn checked_cap_with(value: u64, hard_cap: usize) -> Result<usize, HelperError> {
    let value = usize::try_from(value)
        .map_err(|_| HelperError::new(HelperErrorClass::Limit, "limit is not addressable"))?;
    if value == 0 || value > hard_cap {
        return Err(HelperError::new(
            HelperErrorClass::Limit,
            "limit is outside hard bounds",
        ));
    }
    Ok(value)
}

fn check_content_cap(bytes: &[u8]) -> Result<(), HelperError> {
    if bytes.len() > HARD_BYTES {
        return Err(HelperError::new(
            HelperErrorClass::Limit,
            "content exceeds hard cap",
        ));
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256-{:x}", Sha256::digest(bytes))
}

fn monotonic_nonce() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NONCE: AtomicU64 = AtomicU64::new(0);
    NONCE.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod nested_create_tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn create_builds_parents_and_preserves_no_replace() {
        let root = tempfile::tempdir().unwrap();
        let directory = File::open(root.path()).unwrap();
        atomic_replace(
            directory.as_raw_fd(),
            "docs/nested/file.md",
            b"LIVE_OK\n",
            CreateMode::New,
        )
        .unwrap();
        assert_eq!(
            std::fs::read(root.path().join("docs/nested/file.md")).unwrap(),
            b"LIVE_OK\n"
        );
        assert!(
            atomic_replace(
                directory.as_raw_fd(),
                "docs/nested/file.md",
                b"changed",
                CreateMode::New
            )
            .is_err()
        );
        assert_eq!(
            std::fs::read(root.path().join("docs/nested/file.md")).unwrap(),
            b"LIVE_OK\n"
        );
    }

    #[test]
    fn parent_creation_is_limited_to_new_and_validates_entire_path_first() {
        let root = tempfile::tempdir().unwrap();
        let directory = File::open(root.path()).unwrap();
        for mode in [CreateMode::Replace, CreateMode::Upsert] {
            assert!(atomic_replace(directory.as_raw_fd(), "missing/file.md", b"x", mode).is_err());
            assert!(!root.path().join("missing").exists());
        }
        assert!(
            atomic_replace(
                directory.as_raw_fd(),
                "new/../escape.md",
                b"x",
                CreateMode::New
            )
            .is_err()
        );
        assert!(
            !root.path().join("new").exists(),
            "invalid path must not partially create parents"
        );
        std::fs::write(root.path().join("regular"), b"unchanged").unwrap();
        assert!(
            atomic_replace(
                directory.as_raw_fd(),
                "regular/nested/file.md",
                b"x",
                CreateMode::New
            )
            .is_err()
        );
        assert_eq!(
            std::fs::read(root.path().join("regular")).unwrap(),
            b"unchanged"
        );
    }

    #[test]
    fn parent_creation_cannot_follow_symlinks_or_escape_root() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let directory = File::open(root.path()).unwrap();
        symlink(outside.path(), root.path().join("escape")).unwrap();
        for path in ["escape/nested/file.md", "../outside/file.md"] {
            assert!(
                atomic_replace(directory.as_raw_fd(), path, b"forbidden", CreateMode::New).is_err()
            );
        }
        assert!(!outside.path().join("nested").exists());
    }
}
