//! Real adapters for the fixed filesystem, process, job, and web tool family.
//!
//! Admission, approval, hooks, and durable event ordering are owned by the
//! dispatcher. This module only parses the already-effective invocation and
//! executes the selected bounded backend.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use schema::IJsonValue;
use serde_json::{Value, json};
use store::{FullSync, NamedLock};
use tools::{
    BackendFailure, BackendGate, BackendHold, BackendOutcome, BackendTerminal, BackendUnavailable,
    BoundedHelper, BoundedHttpClient, ByteString, CancellationToken, CreateMode, ExecRequest,
    ExecValue, HelperInvoker, HelperOperation, HelperPath, HelperRequest, HelperResponse,
    HelperValue, HttpFetchResult, SearchProvider, SearchRequest, SearchTopic, ToolExecution,
    WriteValue, validate_fixed_arguments,
};

use crate::ToolBackend;

const READ_SCAN_BYTES: u64 = 32 * 1024 * 1024;
const DEFAULT_READ_LINES: usize = 2_000;
const MAX_LINE_BYTES: usize = 2_000;
const GLOB_RESULTS: u64 = 1_000;
const EXEC_STDOUT_BYTES: u64 = 4 * 1024 * 1024;
const EXEC_STDERR_BYTES: u64 = 1024 * 1024;
const GREP_MS: u64 = 30_000;
/// Budget applied to a `shell` call that omits `max_duration_ms`. A 3000-trial
/// property check ran unbounded at 100% CPU for many minutes on 2026-09-26
/// because the omitted field meant "no limit"; the only bound was the
/// harness's process limit. Five minutes covers a build or a test run and
/// still ends a hung one.
pub const DEFAULT_SHELL_DURATION_MS: u64 = 300_000;
/// Ceiling on a model-supplied `shell` budget; a larger request is clamped,
/// not rejected. Longer work belongs in a detached `job`.
pub const MAX_SHELL_DURATION_MS: u64 = 600_000;
const MAX_SHELL_STEPS: usize = 31;
const MAX_ARTIFACTS: usize = 64;

#[derive(Clone, Copy)]
struct ExecLimits {
    /// `None` runs the command to completion. `shell` always sets it: the
    /// configured default when the model omits `max_duration_ms`, the request
    /// clamped to the configured cap otherwise.
    timeout_ms: Option<u64>,
    stdout_bytes: u64,
    stderr_bytes: u64,
}

macro_rules! outcome_try {
    ($expression:expr) => {
        match $expression {
            Ok(value) => value,
            Err(error) => return BackendOutcome::Completed(Err(error)),
        }
    };
}

#[derive(Clone, Debug)]
pub struct RootMount {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct SystemToolConfig {
    pub workspace: RootMount,
    pub extra_roots: Vec<RootMount>,
    pub shell_program: String,
    pub grep_program: String,
    pub environment: BTreeMap<String, String>,
    /// Budget for a `shell` call that omits `max_duration_ms`.
    pub shell_default_duration_ms: u64,
    /// Largest budget a `shell` call may request; larger values are clamped.
    pub shell_max_duration_ms: u64,
}

impl SystemToolConfig {
    #[must_use]
    pub fn workspace(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            workspace: RootMount {
                name: name.into(),
                path: path.into(),
            },
            extra_roots: Vec::new(),
            shell_program: "/bin/sh".to_owned(),
            grep_program: "rg".to_owned(),
            environment: BTreeMap::new(),
            shell_default_duration_ms: DEFAULT_SHELL_DURATION_MS,
            shell_max_duration_ms: MAX_SHELL_DURATION_MS,
        }
    }

    /// The budget a `shell` call runs under, and how it was chosen.
    fn shell_budget(&self, requested: Option<u64>) -> (u64, &'static str) {
        let cap = self.shell_max_duration_ms.max(1);
        match requested {
            None => (self.shell_default_duration_ms.clamp(1, cap), "default"),
            Some(value) if value > cap => (cap, "clamped"),
            Some(value) => (value.max(1), "requested"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactVersionPermit {
    path: String,
    call: String,
    version: u64,
    preimage_sha256: String,
}

pub trait ArtifactVersionAuthority: Send + Sync {
    fn observe(&self, path: &str, sha256: &str) -> Result<u64, BackendFailure>;
    fn reserve(
        &self,
        path: &str,
        expected_version: u64,
        preimage_sha256: &str,
        call: &str,
    ) -> Result<ArtifactVersionPermit, BackendFailure>;
    fn commit(
        &self,
        permit: &ArtifactVersionPermit,
        resulting_sha256: &str,
    ) -> Result<u64, BackendFailure>;
}

/// Internal successful-edit facts; preimages never enter the model tool result.
pub trait CommittedEditRecorder: Send + Sync {
    fn prepare(
        &self,
        _execution: &ToolExecution,
        _path: &str,
        _before: Option<&str>,
        _after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        Ok(())
    }
    fn abort(
        &self,
        _execution: &ToolExecution,
        _path: &str,
        _before: Option<&str>,
        _after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        Ok(())
    }
    fn record(
        &self,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure>;
}

#[derive(Clone, Debug)]
pub struct DurableArtifactVersions {
    root: PathBuf,
}

#[derive(Clone, Debug)]
struct VersionState {
    version: u64,
    sha256: String,
    pending_call: Option<String>,
}

impl DurableArtifactVersions {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, BackendFailure> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(backend_io)?;
        if fs::symlink_metadata(&root)
            .map_err(backend_io)?
            .file_type()
            .is_symlink()
        {
            return Err(BackendFailure::Denied(
                "artifact version root is a symlink".to_owned(),
            ));
        }
        Ok(Self { root })
    }

    // Old relative records lack mount provenance. Never silently adopt them into
    // an arbitrary workspace or create a second version history beside them.
    fn reject_ambiguous_legacy_path(
        states: &BTreeMap<String, VersionState>,
        path: &str,
    ) -> Result<(), BackendFailure> {
        let target = Path::new(path);
        if target.is_absolute() {
            for legacy in states.keys() {
                let old = Path::new(legacy);
                if !old.is_absolute() && target.ends_with(old) {
                    return Err(BackendFailure::Conflict(format!(
                        "artifact version record {legacy} has no workspace identity; explicit migration is required for {path}"
                    )));
                }
            }
        }
        Ok(())
    }

    fn locked_state(
        &self,
    ) -> Result<(NamedLock, File, BTreeMap<String, VersionState>, u64), BackendFailure> {
        let lock = NamedLock::exclusive(self.root.join("artifact-versions.lock"))
            .map_err(backend_store)?;
        let log_path = self.root.join("artifact-versions.jsonl");
        let log_existed = log_path.exists();
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(log_path)
            .map_err(backend_io)?;
        if !log_existed {
            FullSync::full_sync(&File::open(&self.root).map_err(backend_io)?)
                .map_err(backend_io)?;
        }
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(backend_io)?;
        let valid_end = bytes
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(0, |index| index + 1);
        if valid_end != bytes.len() {
            file.set_len(valid_end as u64).map_err(backend_io)?;
            FullSync::full_sync(&file).map_err(backend_io)?;
            bytes.truncate(valid_end);
        }
        let mut states = BTreeMap::new();
        let mut sequence = 0u64;
        for raw in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let record = IJsonValue::parse(raw)
                .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
            if record
                .canonical_bytes()
                .map_err(|error| BackendFailure::Protocol(error.to_string()))?
                != raw
            {
                return Err(BackendFailure::Protocol(
                    "artifact version log is not canonical JSONL".to_owned(),
                ));
            }
            let value: Value = serde_json::from_slice(raw)
                .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
            let object = value.as_object().ok_or_else(|| {
                BackendFailure::Protocol("artifact version record is not an object".to_owned())
            })?;
            let seq = object.get("seq").and_then(Value::as_u64).ok_or_else(|| {
                BackendFailure::Protocol("artifact version record lacks seq".to_owned())
            })?;
            if seq != sequence + 1 {
                return Err(BackendFailure::Protocol(
                    "artifact version log sequence is discontinuous".to_owned(),
                ));
            }
            sequence = seq;
            let path = object.get("path").and_then(Value::as_str).ok_or_else(|| {
                BackendFailure::Protocol("artifact version record lacks path".to_owned())
            })?;
            let version = object
                .get("version")
                .and_then(Value::as_u64)
                .ok_or_else(|| {
                    BackendFailure::Protocol("artifact version record lacks version".to_owned())
                })?;
            let sha256 = object
                .get("sha256")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    BackendFailure::Protocol("artifact version record lacks sha256".to_owned())
                })?;
            let phase = object.get("phase").and_then(Value::as_str).ok_or_else(|| {
                BackendFailure::Protocol("artifact version record lacks phase".to_owned())
            })?;
            let call = object
                .get("call")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned);
            let pending_call = match phase {
                "reserved" => Some(call.ok_or_else(|| {
                    BackendFailure::Protocol("reserved version record lacks call".to_owned())
                })?),
                "observed" | "committed" => None,
                _ => {
                    return Err(BackendFailure::Protocol(format!(
                        "unknown artifact version phase {phase}"
                    )));
                }
            };
            states.insert(
                path.to_owned(),
                VersionState {
                    version,
                    sha256: sha256.to_owned(),
                    pending_call,
                },
            );
        }
        Ok((lock, file, states, sequence))
    }

    fn append(
        file: &mut File,
        sequence: u64,
        path: &str,
        state: &VersionState,
        phase: &str,
        call: Option<&str>,
    ) -> Result<(), BackendFailure> {
        let mut value = json!({
            "format": 1,
            "path": path,
            "phase": phase,
            "seq": sequence,
            "sha256": state.sha256,
            "version": state.version,
        });
        if let Some(call) = call {
            value
                .as_object_mut()
                .expect("record is an object")
                .insert("call".to_owned(), json!(call));
        }
        let mut bytes = serde_json_canonicalizer::to_vec(&value)
            .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
        bytes.push(b'\n');
        file.write_all(&bytes).map_err(backend_io)?;
        FullSync::full_sync(file).map_err(backend_io)
    }
}

impl ArtifactVersionAuthority for DurableArtifactVersions {
    fn observe(&self, path: &str, sha256: &str) -> Result<u64, BackendFailure> {
        let (_lock, mut file, mut states, sequence) = self.locked_state()?;
        Self::reject_ambiguous_legacy_path(&states, path)?;
        let Some(previous) = states.remove(path) else {
            let state = VersionState {
                version: 0,
                sha256: sha256.to_owned(),
                pending_call: None,
            };
            Self::append(&mut file, sequence + 1, path, &state, "observed", None)?;
            return Ok(0);
        };
        if previous.sha256 == sha256 {
            return Ok(previous.version);
        }
        let version = if previous.pending_call.is_some() {
            previous.version
        } else {
            previous.version.saturating_add(1)
        };
        let state = VersionState {
            version,
            sha256: sha256.to_owned(),
            pending_call: None,
        };
        Self::append(&mut file, sequence + 1, path, &state, "observed", None)?;
        Ok(version)
    }

    fn reserve(
        &self,
        path: &str,
        expected_version: u64,
        preimage_sha256: &str,
        call: &str,
    ) -> Result<ArtifactVersionPermit, BackendFailure> {
        let (_lock, mut file, states, sequence) = self.locked_state()?;
        Self::reject_ambiguous_legacy_path(&states, path)?;
        let current = states.get(path).cloned().unwrap_or_else(|| VersionState {
            version: 0,
            sha256: preimage_sha256.to_owned(),
            pending_call: None,
        });
        if current.sha256 != preimage_sha256 {
            return Err(BackendFailure::Conflict(format!(
                "artifact content changed before version reservation for {path}"
            )));
        }
        if current.version != expected_version {
            return Err(BackendFailure::Conflict(format!(
                "artifact version conflict for {path}: expected {expected_version}, actual {}",
                current.version
            )));
        }
        let version = current.version.saturating_add(1);
        let state = VersionState {
            version,
            sha256: preimage_sha256.to_owned(),
            pending_call: Some(call.to_owned()),
        };
        Self::append(
            &mut file,
            sequence + 1,
            path,
            &state,
            "reserved",
            Some(call),
        )?;
        Ok(ArtifactVersionPermit {
            path: path.to_owned(),
            call: call.to_owned(),
            version,
            preimage_sha256: preimage_sha256.to_owned(),
        })
    }

    fn commit(
        &self,
        permit: &ArtifactVersionPermit,
        resulting_sha256: &str,
    ) -> Result<u64, BackendFailure> {
        let (_lock, mut file, states, sequence) = self.locked_state()?;
        let current = states.get(&permit.path).ok_or_else(|| {
            BackendFailure::Protocol("artifact reservation disappeared".to_owned())
        })?;
        if current.version != permit.version
            || current.sha256 != permit.preimage_sha256
            || current.pending_call.as_deref() != Some(permit.call.as_str())
        {
            return Err(BackendFailure::Conflict(
                "artifact reservation no longer belongs to this call".to_owned(),
            ));
        }
        let state = VersionState {
            version: permit.version,
            sha256: resulting_sha256.to_owned(),
            pending_call: None,
        };
        Self::append(
            &mut file,
            sequence + 1,
            &permit.path,
            &state,
            "committed",
            Some(&permit.call),
        )?;
        Ok(permit.version)
    }
}

/// Synchronous dispatcher boundary. HTTP may use an internal runtime, but it
/// cannot append events or otherwise become ordering authority.
pub struct SystemToolBackend {
    config: SystemToolConfig,
    helper: BoundedHelper<Arc<dyn HelperInvoker>>,
    helper_available: bool,
    http: Option<BoundedHttpClient>,
    search: Option<Arc<dyn SearchProvider>>,
    artifact_versions: Option<Arc<dyn ArtifactVersionAuthority>>,
    edit_recorder: Option<Arc<dyn CommittedEditRecorder>>,
    cancellation: CancellationToken,
}

impl SystemToolBackend {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        config: SystemToolConfig,
        helper: Option<Arc<dyn HelperInvoker>>,
        http: Option<BoundedHttpClient>,
        search: Option<Arc<dyn SearchProvider>>,
        artifact_versions: Option<Arc<dyn ArtifactVersionAuthority>>,
        cancellation: CancellationToken,
    ) -> Self {
        let helper_available = helper.is_some();
        Self {
            config,
            helper: BoundedHelper::new(helper),
            helper_available,
            http,
            search,
            artifact_versions,
            edit_recorder: None,
            cancellation,
        }
    }

    #[must_use]
    pub fn with_edit_recorder(mut self, recorder: Arc<dyn CommittedEditRecorder>) -> Self {
        self.edit_recorder = Some(recorder);
        self
    }

    fn execute_inner(&mut self, execution: &ToolExecution, arguments: &Value) -> BackendTerminal {
        let outcome = match execution.name.as_str() {
            "apply_patch" => self.apply_patch(execution, arguments),
            "edit" => self.edit(execution, arguments),
            "write" => self.write(execution, arguments),
            "read" => self.read(execution, arguments),
            "glob" => self.glob(execution, arguments),
            "grep" => self.grep(execution, arguments),
            "shell" => self.shell(execution, arguments),
            "web_fetch" => self.web_fetch(arguments),
            "web_search" => self.web_search(execution, arguments),
            name => {
                return unavailable(
                    "unsupported_system_tool",
                    format!("system backend does not implement {name}"),
                    false,
                );
            }
        };
        terminal(outcome)
    }

    fn invoke_helper(&self, call: &str, operation: HelperOperation) -> BackendOutcome<HelperValue> {
        match self.helper.invoke(
            &HelperRequest {
                id: call.to_owned(),
                operation,
            },
            BackendGate::Ready,
            &self.cancellation,
        ) {
            BackendOutcome::Completed(Ok(HelperResponse::Result { id, value })) if id == call => {
                BackendOutcome::Completed(Ok(value))
            }
            BackendOutcome::Completed(Ok(HelperResponse::Error { id, error })) if id == call => {
                BackendOutcome::Completed(Err(helper_failure(error)))
            }
            BackendOutcome::Completed(Ok(_)) => BackendOutcome::Completed(Err(
                BackendFailure::Protocol("helper response id did not match call id".to_owned()),
            )),
            BackendOutcome::Completed(Err(error)) => BackendOutcome::Completed(Err(error)),
            BackendOutcome::Hold(hold) => BackendOutcome::Hold(hold),
            BackendOutcome::Unavailable(unavailable) => BackendOutcome::Unavailable(unavailable),
        }
    }

    fn read(&mut self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let path = outcome_try!(required_str(args, "path"));
        let offset = outcome_try!(optional_u64(args, "offset")).unwrap_or(1);
        let limit = outcome_try!(optional_u64(args, "limit")).unwrap_or(DEFAULT_READ_LINES as u64);
        let (root, relative) = outcome_try!(self.resolve_path(path, false));
        let identity = outcome_try!(self.artifact_identity(&root, &relative));
        let read = match self.invoke_helper(
            &execution.call,
            HelperOperation::Read {
                root,
                path: relative,
                max_bytes: READ_SCAN_BYTES,
            },
        ) {
            BackendOutcome::Completed(Ok(HelperValue::Read(value))) => value,
            other => return retype(other, "read helper returned the wrong value"),
        };
        let bytes = outcome_try!(read.content.decode().map_err(helper_failure));
        let text = outcome_try!(
            std::str::from_utf8(&bytes)
                .map_err(|_| BackendFailure::Invalid("read target is not UTF-8".to_owned()))
        );
        let Some(authority) = &self.artifact_versions else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "artifact-version-authority".to_owned(),
                reason: "durable artifact version authority is not configured".to_owned(),
            });
        };
        let version = outcome_try!(authority.observe(&identity, &read.sha256));
        let start = outcome_try!(
            usize::try_from(offset - 1)
                .map_err(|_| BackendFailure::Limit("read offset is too large".to_owned()))
        );
        let count = outcome_try!(
            usize::try_from(limit)
                .map_err(|_| BackendFailure::Limit("read limit is too large".to_owned()))
        );
        let all: Vec<&str> = text.lines().collect();
        let lines = all
            .iter()
            .enumerate()
            .skip(start)
            .take(count)
            .map(|(index, line)| {
                let (text, truncated) = truncate_utf8(line, MAX_LINE_BYTES);
                json!({"line": index + 1, "text": text, "truncated": truncated})
            })
            .collect::<Vec<_>>();
        BackendOutcome::Completed(Ok(json!({
            "path": path,
            "offset": offset,
            "limit": limit,
            "lines": lines,
            "total_lines": all.len(),
            "bytes": read.bytes,
            "sha256": read.sha256,
            "artifact_version": version,
            "truncated": start.saturating_add(count) < all.len(),
        })))
    }

    fn glob(&self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let pattern = outcome_try!(required_str(args, "pattern"));
        let base = outcome_try!(optional_str(args, "path")).unwrap_or("");
        let (root, relative) = outcome_try!(self.resolve_path(base, false));
        let pattern = if relative.is_empty() {
            pattern.to_owned()
        } else {
            format!("{relative}/{pattern}")
        };
        match self.invoke_helper(
            &execution.call,
            HelperOperation::Glob {
                root,
                pattern,
                max_entries: GLOB_RESULTS,
            },
        ) {
            BackendOutcome::Completed(Ok(HelperValue::Glob { paths })) => {
                BackendOutcome::Completed(Ok(json!({"paths": paths})))
            }
            other => retype(other, "glob helper returned the wrong value"),
        }
    }

    fn grep(&self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let pattern = outcome_try!(required_str(args, "pattern"));
        let path = outcome_try!(optional_str(args, "path")).unwrap_or("");
        let output_mode = outcome_try!(optional_str(args, "output_mode")).unwrap_or("content");
        let case_insensitive =
            outcome_try!(optional_bool(args, "case_insensitive")).unwrap_or(false);
        let context = outcome_try!(optional_u64(args, "context_lines")).unwrap_or(0);
        let glob = outcome_try!(optional_str(args, "glob"));
        let (root, relative) = outcome_try!(self.resolve_path(path, false));
        let mut argv = vec![
            self.config.grep_program.clone(),
            "--color=never".to_owned(),
            "--no-messages".to_owned(),
            "--max-columns=2000".to_owned(),
            "--max-filesize=16M".to_owned(),
        ];
        match output_mode {
            "content" => argv.extend(["--line-number".to_owned(), "--no-heading".to_owned()]),
            "files_with_matches" => argv.push("--files-with-matches".to_owned()),
            "count" => argv.push("--count".to_owned()),
            _ => return invalid_outcome("invalid grep output_mode"),
        }
        if case_insensitive {
            argv.push("--ignore-case".to_owned());
        }
        if context > 0 {
            argv.extend(["--context".to_owned(), context.to_string()]);
        }
        if let Some(glob) = glob {
            argv.extend(["--glob".to_owned(), glob.to_owned()]);
        }
        // Keep the command's cwd at the mount root and pass the selected path
        // to rg. A file is a valid search target; using it as cwd fails with
        // ENOTDIR before rg can inspect it.
        let target = if relative.is_empty() {
            ".".to_owned()
        } else {
            relative.clone()
        };
        argv.extend(["--".to_owned(), pattern.to_owned(), target]);
        let exec = self.exec_helper(
            execution,
            argv,
            root,
            String::new(),
            ExecLimits {
                timeout_ms: Some(GREP_MS),
                stdout_bytes: EXEC_STDOUT_BYTES,
                stderr_bytes: EXEC_STDERR_BYTES,
            },
        );
        let exec = match exec {
            BackendOutcome::Completed(Err(BackendFailure::NotFound(_))) => {
                let (root, relative) = outcome_try!(self.resolve_path(path, false));
                match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Grep {
                        root,
                        path: relative,
                        pattern: pattern.to_owned(),
                        glob: glob.map(str::to_owned),
                        output_mode: output_mode.to_owned(),
                        case_insensitive,
                        context_lines: context,
                        stdout_bytes: EXEC_STDOUT_BYTES,
                        timeout_ms: GREP_MS,
                    },
                ) {
                    BackendOutcome::Completed(Ok(HelperValue::Exec(value))) => {
                        BackendOutcome::Completed(Ok(value))
                    }
                    other => retype(other, "native grep returned the wrong value"),
                }
            }
            other => other,
        };
        match exec {
            BackendOutcome::Completed(Ok(value)) if value.status == 0 || value.status == 1 => {
                let matched = value.status == 0;
                BackendOutcome::Completed(exec_json(value, true).map(|mut result| {
                    result
                        .as_object_mut()
                        .expect("exec result is an object")
                        .insert("matched".to_owned(), json!(matched));
                    result
                }))
            }
            BackendOutcome::Completed(Ok(value)) => BackendOutcome::Completed(Err(
                BackendFailure::Io(format!("ripgrep exited with status {}", value.status)),
            )),
            other => retype(other, "grep helper returned the wrong value"),
        }
    }

    fn shell(&self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let artifacts = outcome_try!(parse_artifacts(args));
        if !artifacts.is_empty() && self.artifact_versions.is_none() {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "artifact-version-authority".to_owned(),
                reason: "declared shell artifacts require durable version tracking".to_owned(),
            });
        }
        let working = outcome_try!(optional_str(args, "working_directory")).unwrap_or("");
        let (root, relative) = outcome_try!(self.resolve_path(working, false));
        let max_ms = outcome_try!(optional_u64(args, "max_duration_ms"));
        let output_tokens = outcome_try!(optional_u64(args, "max_output_tokens"));
        let stdout_bytes = output_tokens
            .map(|tokens| tokens.saturating_mul(4).clamp(1, EXEC_STDOUT_BYTES))
            .unwrap_or(EXEC_STDOUT_BYTES);
        let stderr_bytes = stdout_bytes.min(EXEC_STDERR_BYTES);
        let failure_policy =
            outcome_try!(optional_str(args, "failure_policy")).unwrap_or("stop_on_error");
        if !matches!(failure_policy, "stop_on_error" | "continue") {
            return invalid_outcome("failure_policy must be stop_on_error or continue");
        }
        let mut commands = Vec::new();
        if let Some(command) = outcome_try!(optional_str(args, "command")) {
            commands.push(vec![
                self.config.shell_program.clone(),
                "-c".to_owned(),
                command.to_owned(),
            ]);
        } else if let Some(program) = outcome_try!(optional_str(args, "program")) {
            let mut argv = vec![program.to_owned()];
            argv.extend(outcome_try!(string_array(args, "args")).unwrap_or_default());
            commands.push(argv);
        } else {
            return invalid_outcome("shell requires command or program");
        }
        commands.extend(outcome_try!(parse_steps(args)));
        if commands.len() > MAX_SHELL_STEPS + 1 {
            return invalid_outcome("shell may contain at most 31 verification steps");
        }
        let (budget, budget_source) = self.config.shell_budget(max_ms);
        let cap = self.config.shell_max_duration_ms;
        let started = Instant::now();
        let mut failed = false;
        let mut results = Vec::with_capacity(commands.len());
        for (index, argv) in commands.into_iter().enumerate() {
            if failed && failure_policy == "stop_on_error" {
                results.push(json!({
                    "index": index,
                    "program": argv[0],
                    "status": "not_executed",
                }));
                continue;
            }
            let elapsed = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
            let Some(remaining) = budget.checked_sub(elapsed).filter(|value| *value > 0) else {
                // Earlier steps used up the whole budget: nothing to kill, but
                // the call has failed and the model needs to know why.
                failed = true;
                results.push(json!({
                    "index": index,
                    "program": argv[0],
                    "status": "not_executed",
                    "budget_exhausted": true,
                    "budget_ms": budget,
                }));
                continue;
            };
            let exec = match self.exec_helper(
                execution,
                argv.clone(),
                root.clone(),
                relative.clone(),
                ExecLimits {
                    timeout_ms: Some(remaining),
                    stdout_bytes,
                    stderr_bytes,
                },
            ) {
                BackendOutcome::Completed(Ok(value)) => value,
                BackendOutcome::Completed(Err(BackendFailure::Timeout)) => {
                    // The helper has already sent SIGTERM then SIGKILL to the
                    // child's process group, so `sh -c` and everything it
                    // started are gone. Report that as a failed step rather
                    // than a retryable backend error: the model can then
                    // choose a larger budget or a detached job.
                    failed = true;
                    results.push(json!({
                        "index": index,
                        "program": argv[0],
                        "status": "timed_out",
                        "killed_after_ms": remaining,
                        "budget_ms": budget,
                        "budget_source": budget_source,
                        "max_duration_ms": cap,
                    }));
                    continue;
                }
                other => return retype(other, "shell helper returned the wrong value"),
            };
            let step_succeeded = exec.status == 0;
            failed |= !step_succeeded;
            let mut value = exec_json(exec, step_succeeded)
                .unwrap_or_else(|error| json!({"serialization_error": error.to_string()}));
            if let Some(object) = value.as_object_mut() {
                object.insert("index".to_owned(), json!(index));
                object.insert("program".to_owned(), json!(argv[0]));
            }
            results.push(value);
        }
        let verified = if failed {
            artifacts
                .iter()
                .map(|path| json!({"path": path, "status": "not_checked"}))
                .collect()
        } else {
            let mut output = Vec::with_capacity(artifacts.len());
            for path in artifacts {
                let (root, relative) = outcome_try!(self.resolve_path(&path, true));
                let identity = outcome_try!(self.artifact_identity(&root, &relative));
                match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Read {
                        root: self.config.workspace.name.clone(),
                        path: relative,
                        max_bytes: READ_SCAN_BYTES,
                    },
                ) {
                    BackendOutcome::Completed(Ok(HelperValue::Read(value))) => {
                        let authority = self
                            .artifact_versions
                            .as_ref()
                            .expect("artifact authority checked before shell");
                        let version = outcome_try!(authority.observe(&identity, &value.sha256));
                        output.push(json!({
                            "path": path,
                            "status": "verified",
                            "bytes": value.bytes,
                            "sha256": value.sha256,
                            "artifact_version": version,
                        }));
                    }
                    BackendOutcome::Completed(Err(BackendFailure::NotFound(_))) => {
                        return BackendOutcome::Completed(Err(BackendFailure::NotFound(format!(
                            "declared artifact {path}"
                        ))));
                    }
                    other => return retype(other, "artifact helper returned the wrong value"),
                }
            }
            output
        };
        BackendOutcome::Completed(Ok(json!({
            "status": if failed { "failed" } else { "completed" },
            "is_error": failed,
            "steps": results,
            "artifacts": verified,
        })))
    }

    fn web_fetch(&self, args: &Value) -> BackendOutcome<Value> {
        let Some(http) = &self.http else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "http-client".to_owned(),
                reason: "bounded HTTP client is not configured".to_owned(),
            });
        };
        match http.fetch(
            outcome_try!(required_str(args, "url")),
            BackendGate::Ready,
            &self.cancellation,
        ) {
            BackendOutcome::Completed(Ok(result)) => {
                BackendOutcome::Completed(Ok(web_fetch_result_value(result)))
            }
            BackendOutcome::Completed(Err(error)) => BackendOutcome::Completed(Err(error)),
            BackendOutcome::Hold(hold) => BackendOutcome::Hold(hold),
            BackendOutcome::Unavailable(unavailable) => BackendOutcome::Unavailable(unavailable),
        }
    }

    fn web_search(&self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let Some(http) = &self.http else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "http-client".to_owned(),
                reason: "bounded HTTP client is not configured".to_owned(),
            });
        };
        let topic = match outcome_try!(optional_str(args, "topic")).unwrap_or("general") {
            "general" => SearchTopic::General,
            "news" => SearchTopic::News,
            _ => return invalid_outcome("invalid web_search topic"),
        };
        let max_results = outcome_try!(optional_u64(args, "max_results")).unwrap_or(5);
        let max_results = outcome_try!(
            u8::try_from(max_results)
                .map_err(|_| BackendFailure::Invalid("max_results is too large".to_owned()))
        );
        let request = SearchRequest {
            call_id: execution.call.clone(),
            query: outcome_try!(required_str(args, "query")).to_owned(),
            max_results,
            topic,
        };
        match http.search(
            &request,
            self.search.as_deref(),
            BackendGate::Ready,
            &self.cancellation,
        ) {
            BackendOutcome::Completed(Ok(hits)) => BackendOutcome::Completed(Ok(json!({
                "query": request.query,
                "topic": match request.topic { SearchTopic::General => "general", SearchTopic::News => "news" },
                "results": hits.into_iter().map(|hit| json!({
                    "title": hit.title,
                    "url": hit.url,
                    "snippet": hit.snippet,
                })).collect::<Vec<_>>()
            }))),
            BackendOutcome::Completed(Err(error)) => BackendOutcome::Completed(Err(error)),
            BackendOutcome::Hold(hold) => BackendOutcome::Hold(hold),
            BackendOutcome::Unavailable(unavailable) => BackendOutcome::Unavailable(unavailable),
        }
    }

    fn apply_patch(&mut self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let path = outcome_try!(required_str(args, "path"));
        let diff = outcome_try!(required_str(args, "diff"));
        let summary = args
            .get("summary")
            .and_then(Value::as_str)
            .unwrap_or("patch");
        let expected = args
            .get("expected_artifact_version")
            .and_then(Value::as_u64);
        let (root, relative) = outcome_try!(self.resolve_path(path, false));
        let identity = outcome_try!(self.artifact_identity(&root, &relative));
        let Some(authority) = &self.artifact_versions else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "artifact-version-authority".to_owned(),
                reason: "durable artifact version authority is not configured".to_owned(),
            });
        };
        let inferred_operation;
        let operation = if let Some(operation) = args.get("operation").and_then(Value::as_str) {
            operation
        } else {
            inferred_operation = match self.invoke_helper(
                &execution.call,
                HelperOperation::Read {
                    root: root.clone(),
                    path: relative.clone(),
                    max_bytes: 1,
                },
            ) {
                BackendOutcome::Completed(Err(BackendFailure::NotFound(_))) => "create_file",
                BackendOutcome::Completed(Err(BackendFailure::Limit(_)))
                | BackendOutcome::Completed(Ok(HelperValue::Read(_))) => "update_file",
                other => return retype(other, "patch preflight returned the wrong value"),
            };
            inferred_operation
        };
        match operation {
            "create_file" => {
                match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Read {
                        root: root.clone(),
                        path: relative.clone(),
                        max_bytes: 1,
                    },
                ) {
                    BackendOutcome::Completed(Err(BackendFailure::NotFound(_))) => {}
                    BackendOutcome::Completed(Err(BackendFailure::Limit(_)))
                    | BackendOutcome::Completed(Ok(HelperValue::Read(_))) => {
                        return BackendOutcome::Completed(Err(BackendFailure::Conflict(format!(
                            "file already exists: {path}"
                        ))));
                    }
                    other => return retype(other, "create preflight returned the wrong value"),
                }
                let actual = outcome_try!(authority.observe(&identity, "missing"));
                let patch = outcome_try!(PatchResult::create(diff));
                let permit = outcome_try!(authority.reserve(
                    &identity,
                    expected.unwrap_or(actual),
                    "missing",
                    &execution.call,
                ));
                debug_assert_eq!(permit.version, actual.saturating_add(1));
                if let Some(recorder) = &self.edit_recorder {
                    outcome_try!(recorder.prepare(
                        execution,
                        &identity,
                        None,
                        Some(&patch.content)
                    ));
                }
                match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Write {
                        root,
                        path: relative,
                        content: ByteString::from_bytes(patch.content.as_bytes()),
                        create: CreateMode::New,
                    },
                ) {
                    BackendOutcome::Completed(Ok(HelperValue::Write(value))) => {
                        if let Some(recorder) = &self.edit_recorder {
                            outcome_try!(recorder.record(
                                execution,
                                &identity,
                                None,
                                Some(&patch.content)
                            ));
                        }
                        let version =
                            outcome_try!(authority.commit(&permit, &value.sha256).map_err(
                                |error| BackendFailure::Unknown(format!(
                                    "file committed but artifact version commit failed: {error}"
                                ))
                            ));
                        BackendOutcome::Completed(Ok(patch_result_json(
                            path, summary, version, &value, &patch,
                        )))
                    }
                    other => {
                        if matches!(
                            &other,
                            BackendOutcome::Completed(Err(BackendFailure::Conflict(_)
                                | BackendFailure::Denied(_)
                                | BackendFailure::Invalid(_)
                                | BackendFailure::NotFound(_)))
                        ) {
                            if let Some(recorder) = &self.edit_recorder {
                                outcome_try!(recorder.abort(
                                    execution,
                                    &identity,
                                    None,
                                    Some(&patch.content)
                                ));
                            }
                        }
                        retype(other, "write helper returned the wrong value")
                    }
                }
            }
            "update_file" => {
                let read = match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Read {
                        root: root.clone(),
                        path: relative.clone(),
                        max_bytes: 16 * 1024 * 1024,
                    },
                ) {
                    BackendOutcome::Completed(Ok(HelperValue::Read(value))) => value,
                    other => return retype(other, "patch read returned the wrong value"),
                };
                let actual = outcome_try!(authority.observe(&identity, &read.sha256));
                let original = outcome_try!(read.content.decode().map_err(helper_failure));
                let original = outcome_try!(std::str::from_utf8(&original).map_err(|_| {
                    BackendFailure::Invalid("patch target is not UTF-8".to_owned())
                }));
                let patch = outcome_try!(PatchResult::update(original, diff));
                let permit = outcome_try!(authority.reserve(
                    &identity,
                    expected.unwrap_or(actual),
                    &read.sha256,
                    &execution.call,
                ));
                if let Some(recorder) = &self.edit_recorder {
                    outcome_try!(recorder.prepare(
                        execution,
                        &identity,
                        Some(original),
                        Some(&patch.content)
                    ));
                }
                match self.invoke_helper(
                    &execution.call,
                    HelperOperation::Patch {
                        root,
                        path: relative,
                        expected_sha256: read.sha256,
                        replacement: ByteString::from_bytes(patch.content.as_bytes()),
                    },
                ) {
                    BackendOutcome::Completed(Ok(HelperValue::Patch(value))) => {
                        if let Some(recorder) = &self.edit_recorder {
                            outcome_try!(recorder.record(
                                execution,
                                &identity,
                                Some(original),
                                Some(&patch.content)
                            ));
                        }
                        debug_assert_eq!(permit.version, actual.saturating_add(1));
                        let version =
                            outcome_try!(authority.commit(&permit, &value.sha256).map_err(
                                |error| BackendFailure::Unknown(format!(
                                    "patch committed but artifact version commit failed: {error}"
                                ))
                            ));
                        BackendOutcome::Completed(Ok(patch_result_json(
                            path, summary, version, &value, &patch,
                        )))
                    }
                    other => {
                        if matches!(
                            &other,
                            BackendOutcome::Completed(Err(BackendFailure::Conflict(_)
                                | BackendFailure::Denied(_)
                                | BackendFailure::Invalid(_)
                                | BackendFailure::NotFound(_)))
                        ) {
                            if let Some(recorder) = &self.edit_recorder {
                                outcome_try!(recorder.abort(
                                    execution,
                                    &identity,
                                    Some(original),
                                    Some(&patch.content)
                                ));
                            }
                        }
                        retype(other, "patch helper returned the wrong value")
                    }
                }
            }
            _ => invalid_outcome("invalid apply_patch operation"),
        }
    }

    fn write(&mut self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let path = outcome_try!(required_str(args, "path"));
        let content = outcome_try!(
            args.get("content")
                .and_then(Value::as_str)
                .ok_or_else(|| BackendFailure::Invalid("content must be a string".to_owned()))
        );
        self.write_content(execution, path, content, None)
    }

    fn edit(&mut self, execution: &ToolExecution, args: &Value) -> BackendOutcome<Value> {
        let path = outcome_try!(required_str(args, "path"));
        let old = outcome_try!(required_str(args, "old_text"));
        let new = outcome_try!(
            args.get("new_text")
                .and_then(Value::as_str)
                .ok_or_else(|| BackendFailure::Invalid("new_text must be a string".to_owned()))
        );
        let (root, relative) = outcome_try!(self.resolve_path(path, false));
        let read = match self.invoke_helper(
            &execution.call,
            HelperOperation::Read {
                root,
                path: relative,
                max_bytes: 16 * 1024 * 1024,
            },
        ) {
            BackendOutcome::Completed(Ok(HelperValue::Read(value))) => value,
            other => return retype(other, "edit read returned the wrong value"),
        };
        let bytes = outcome_try!(read.content.decode().map_err(helper_failure));
        let original = outcome_try!(
            std::str::from_utf8(&bytes)
                .map_err(|_| BackendFailure::Invalid("edit target is not UTF-8".to_owned()))
        );
        let mut matches = original.match_indices(old);
        let Some((index, _)) = matches.next() else {
            return invalid_outcome("old_text was not found in the file");
        };
        if matches.next().is_some() {
            return invalid_outcome("old_text occurs more than once; supply a longer unique span");
        }
        let mut replacement = String::with_capacity(original.len() - old.len() + new.len());
        replacement.push_str(&original[..index]);
        replacement.push_str(new);
        replacement.push_str(&original[index + old.len()..]);
        self.write_content(execution, path, &replacement, Some(&read.sha256))
    }

    fn write_content(
        &mut self,
        execution: &ToolExecution,
        path: &str,
        content: &str,
        expected_sha: Option<&str>,
    ) -> BackendOutcome<Value> {
        let (root, relative) = outcome_try!(self.resolve_path(path, false));
        let identity = outcome_try!(self.artifact_identity(&root, &relative));
        let Some(authority) = &self.artifact_versions else {
            return BackendOutcome::Unavailable(BackendUnavailable {
                dependency: "artifact-version-authority".to_owned(),
                reason: "durable artifact version authority is not configured".to_owned(),
            });
        };
        let existing = match self.invoke_helper(
            &execution.call,
            HelperOperation::Read {
                root: root.clone(),
                path: relative.clone(),
                max_bytes: 16 * 1024 * 1024,
            },
        ) {
            BackendOutcome::Completed(Ok(HelperValue::Read(value))) => Some(value),
            BackendOutcome::Completed(Err(BackendFailure::NotFound(_))) => None,
            other => return retype(other, "write preflight returned the wrong value"),
        };
        let prior_sha = existing
            .as_ref()
            .map_or("missing", |value| value.sha256.as_str());
        if expected_sha.is_some_and(|expected| expected != prior_sha) {
            return BackendOutcome::Completed(Err(BackendFailure::Conflict(
                "file changed since edit read".to_owned(),
            )));
        }
        let before_bytes = existing
            .as_ref()
            .map(|value| value.content.decode().map_err(helper_failure))
            .transpose();
        let before_bytes = outcome_try!(before_bytes);
        let before = before_bytes
            .as_deref()
            .and_then(|bytes| std::str::from_utf8(bytes).ok());
        let actual = outcome_try!(authority.observe(&identity, prior_sha));
        let permit = outcome_try!(authority.reserve(&identity, actual, prior_sha, &execution.call));
        if let Some(recorder) = &self.edit_recorder {
            outcome_try!(recorder.prepare(execution, &identity, before, Some(content)));
        }
        let operation = match existing {
            Some(value) => HelperOperation::Patch {
                root,
                path: relative,
                expected_sha256: value.sha256,
                replacement: ByteString::from_bytes(content.as_bytes()),
            },
            None => HelperOperation::Write {
                root,
                path: relative,
                content: ByteString::from_bytes(content.as_bytes()),
                create: CreateMode::New,
            },
        };
        match self.invoke_helper(&execution.call, operation) {
            BackendOutcome::Completed(Ok(
                HelperValue::Write(value) | HelperValue::Patch(value),
            )) => {
                if let Some(recorder) = &self.edit_recorder {
                    outcome_try!(recorder.record(execution, &identity, before, Some(content)));
                }
                let version =
                    outcome_try!(authority.commit(&permit, &value.sha256).map_err(|error| {
                        BackendFailure::Unknown(format!(
                            "file committed but artifact version commit failed: {error}"
                        ))
                    }));
                BackendOutcome::Completed(Ok(json!({
                    "path": path,
                    "bytes": value.bytes,
                    "sha256": value.sha256,
                    "artifact_version": version,
                })))
            }
            other => {
                if matches!(
                    &other,
                    BackendOutcome::Completed(Err(BackendFailure::Conflict(_)
                        | BackendFailure::Denied(_)
                        | BackendFailure::Invalid(_)
                        | BackendFailure::NotFound(_)))
                ) {
                    if let Some(recorder) = &self.edit_recorder {
                        outcome_try!(recorder.abort(execution, &identity, before, Some(content)));
                    }
                }
                retype(other, "write helper returned the wrong value")
            }
        }
    }

    fn exec_helper(
        &self,
        execution: &ToolExecution,
        argv: Vec<String>,
        root: String,
        relative: String,
        limits: ExecLimits,
    ) -> BackendOutcome<ExecValue> {
        match self.invoke_helper(
            &execution.call,
            HelperOperation::Exec(ExecRequest {
                argv,
                stdin: None,
                cwd: Some(HelperPath {
                    root,
                    path: relative,
                }),
                env: self.config.environment.clone(),
                stdout_bytes: limits.stdout_bytes,
                stderr_bytes: limits.stderr_bytes,
                timeout_ms: limits.timeout_ms,
            }),
        ) {
            BackendOutcome::Completed(Ok(HelperValue::Exec(value))) => {
                BackendOutcome::Completed(Ok(value))
            }
            other => retype(other, "exec helper returned the wrong value"),
        }
    }

    fn artifact_identity(&self, root: &str, relative: &str) -> Result<String, BackendFailure> {
        let mount = std::iter::once(&self.config.workspace)
            .chain(self.config.extra_roots.iter())
            .find(|mount| mount.name == root)
            .ok_or_else(|| {
                BackendFailure::Protocol("resolved artifact mount disappeared".to_owned())
            })?;
        if !mount.path.is_absolute() {
            return Err(BackendFailure::Invalid(
                "artifact mount must be absolute".to_owned(),
            ));
        }
        mount
            .path
            .join(relative)
            .to_str()
            .map(ToOwned::to_owned)
            .ok_or_else(|| BackendFailure::Invalid("artifact path is not UTF-8".to_owned()))
    }

    /// The roots a path may name, as a human list for a refusal the model can act
    /// on ("/a" / "/a or /b" / "/a, /b or /c").
    fn allowed_roots_sentence(&self) -> String {
        let roots = std::iter::once(&self.config.workspace)
            .chain(self.config.extra_roots.iter())
            .map(|mount| mount.path.display().to_string())
            .collect::<Vec<_>>();
        match roots.split_last() {
            None => "(no configured root)".to_owned(),
            Some((last, [])) => last.clone(),
            Some((last, rest)) => format!("{} or {last}", rest.join(", ")),
        }
    }

    fn resolve_path(
        &self,
        raw: &str,
        workspace_only: bool,
    ) -> Result<(String, String), BackendFailure> {
        let path = Path::new(if raw.is_empty() { "." } else { raw });
        if path.is_absolute() {
            if workspace_only && !path.starts_with(&self.config.workspace.path) {
                return Err(BackendFailure::Denied(format!(
                    "path is outside the workspace root; use a path under {}",
                    self.config.workspace.path.display()
                )));
            }
            let mounts =
                std::iter::once(&self.config.workspace).chain(self.config.extra_roots.iter());
            for mount in mounts {
                if let Ok(relative) = path.strip_prefix(&mount.path) {
                    return Ok((mount.name.clone(), normalized_relative(relative)?));
                }
            }
            // Name the roots that would have worked. "outside configured helper
            // roots" told the model only that it had lost the call, so it kept
            // retrying the same absolute path (2026-09-20, a shell run in `/`).
            return Err(BackendFailure::Denied(format!(
                "path is outside the allowed roots; use a path under {}",
                self.allowed_roots_sentence()
            )));
        }
        Ok((
            self.config.workspace.name.clone(),
            normalized_relative(path)?,
        ))
    }
}

/// Exact durable value returned by the production `web_fetch` backend. The
/// raw response body is intentionally consumed only to report its byte count.
#[must_use]
pub fn web_fetch_result_value(result: HttpFetchResult) -> Value {
    json!({
        "final_url": result.final_url,
        "status": result.status,
        "content_type": result.content_type,
        "bytes": result.body.len(),
        "text": result.extraction.text,
        "extraction": {
            "kind": result.extraction.kind,
            "under_rendered": result.extraction.under_rendered,
            "truncated": result.extraction.truncated,
        },
    })
}

impl ToolBackend for SystemToolBackend {
    fn supports(&self, name: &str) -> bool {
        match name {
            "apply_patch" | "edit" | "write" | "read" => {
                self.helper_available && self.artifact_versions.is_some()
            }
            "glob" | "grep" | "shell" => self.helper_available,
            // `background_exec` is supervisor-owned and reaches JobBroker over
            // correlated worker-control, never from this worker backend.
            "job" => false,
            "web_fetch" => self.http.is_some(),
            "web_search" => self.http.is_some() && self.search.is_some(),
            _ => false,
        }
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal {
        let arguments = match serde_json::to_value(invocation) {
            Ok(value) => value,
            Err(error) => return unavailable("invalid_arguments", error.to_string(), false),
        };
        if let Err(error) = validate_fixed_arguments(&execution.name, &arguments) {
            return unavailable("invalid_arguments", error.to_string(), false);
        }
        self.execute_inner(execution, &arguments)
    }
}

fn terminal(outcome: BackendOutcome<Value>) -> BackendTerminal {
    match outcome {
        BackendOutcome::Completed(Ok(value)) => match json_to_ijson(&value) {
            Ok(value) => BackendTerminal::Completed(value),
            Err(error) => unavailable("result_encoding", error, false),
        },
        BackendOutcome::Completed(Err(error)) => {
            let retryable = matches!(
                error,
                BackendFailure::Timeout | BackendFailure::Unavailable(_)
            );
            unavailable(failure_code(&error), error.to_string(), retryable)
        }
        BackendOutcome::Hold(BackendHold { scope, reason }) => BackendTerminal::Hold {
            scope,
            question: json_to_ijson(&json!({"reason": reason})).expect("static I-JSON"),
        },
        BackendOutcome::Unavailable(BackendUnavailable { dependency, reason }) => {
            unavailable(&format!("backend_unavailable.{dependency}"), reason, true)
        }
    }
}

fn unavailable(code: &str, message: impl Into<String>, retryable: bool) -> BackendTerminal {
    BackendTerminal::Unavailable {
        code: code.to_owned(),
        message: message.into(),
        retryable,
    }
}

fn failure_code(error: &BackendFailure) -> &'static str {
    match error {
        BackendFailure::Invalid(_) => "invalid_arguments",
        BackendFailure::Denied(_) => "denied",
        BackendFailure::Cancelled => "cancelled",
        BackendFailure::Timeout => "timeout",
        BackendFailure::Limit(_) => "limit_exceeded",
        BackendFailure::NotFound(_) => "not_found",
        BackendFailure::Conflict(_) => "conflict",
        BackendFailure::Unavailable(_) => "backend_unavailable",
        BackendFailure::TerminalUnavailable(_) => "backend_unavailable",
        BackendFailure::Io(_) => "backend_io",
        BackendFailure::Protocol(_) => "backend_protocol",
        BackendFailure::Unknown(_) => "unknown_effect",
    }
}

fn helper_failure(error: tools::HelperError) -> BackendFailure {
    use tools::HelperErrorClass as Class;
    match error.class {
        Class::InvalidRequest => BackendFailure::Invalid(error.message),
        Class::UnknownRoot | Class::SandboxUnavailable => {
            BackendFailure::Unavailable(error.message)
        }
        Class::PathEscape | Class::Symlink | Class::Denied => BackendFailure::Denied(error.message),
        Class::NotRegular => BackendFailure::Invalid(error.message),
        Class::Exists | Class::DigestMismatch => BackendFailure::Conflict(error.message),
        Class::Missing => BackendFailure::NotFound(error.message),
        Class::Limit => BackendFailure::Limit(error.message),
        Class::Timeout => BackendFailure::Timeout,
        Class::Io => BackendFailure::Io(error.message),
        Class::Protocol => BackendFailure::Protocol(error.message),
    }
}

fn backend_io(error: std::io::Error) -> BackendFailure {
    BackendFailure::Io(error.to_string())
}

fn backend_store(error: store::StoreError) -> BackendFailure {
    BackendFailure::Io(error.to_string())
}

fn json_to_ijson(value: &Value) -> Result<IJsonValue, String> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn required_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, BackendFailure> {
    args.get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| BackendFailure::Invalid(format!("{key} must be a nonempty string")))
}

fn optional_str<'a>(args: &'a Value, key: &str) -> Result<Option<&'a str>, BackendFailure> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value)),
        Some(_) => Err(BackendFailure::Invalid(format!("{key} must be a string"))),
    }
}

fn optional_bool(args: &Value, key: &str) -> Result<Option<bool>, BackendFailure> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(value)) => Ok(Some(*value)),
        Some(_) => Err(BackendFailure::Invalid(format!("{key} must be a boolean"))),
    }
}

fn optional_u64(args: &Value, key: &str) -> Result<Option<u64>, BackendFailure> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(value)) => value
            .as_u64()
            .map(Some)
            .ok_or_else(|| BackendFailure::Invalid(format!("{key} must be a nonnegative integer"))),
        Some(_) => Err(BackendFailure::Invalid(format!("{key} must be an integer"))),
    }
}

fn string_array(args: &Value, key: &str) -> Result<Option<Vec<String>>, BackendFailure> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| {
                item.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                    BackendFailure::Invalid(format!("{key} must contain only strings"))
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Some),
        Some(_) => Err(BackendFailure::Invalid(format!("{key} must be an array"))),
    }
}

/// `null` and an absent property read as "no steps", the same as `[]`: a strict
/// function schema makes the model send every property, and it writes `null` for
/// the ones it has nothing to say about.
fn parse_steps(args: &Value) -> Result<Vec<Vec<String>>, BackendFailure> {
    let steps = match args.get("steps") {
        None | Some(Value::Null) => return Ok(Vec::new()),
        Some(value) => value
            .as_array()
            .ok_or_else(|| BackendFailure::Invalid("steps must be an array".to_owned()))?,
    };
    steps
        .iter()
        .map(|step| {
            let object = step
                .as_object()
                .ok_or_else(|| BackendFailure::Invalid("each step must be an object".to_owned()))?;
            if object
                .keys()
                .any(|key| !matches!(key.as_str(), "program" | "args"))
            {
                return Err(BackendFailure::Invalid(
                    "step contains an unknown property".to_owned(),
                ));
            }
            let program = object
                .get("program")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    BackendFailure::Invalid("step program must be nonempty".to_owned())
                })?;
            let items = object
                .get("args")
                .and_then(Value::as_array)
                .ok_or_else(|| BackendFailure::Invalid("step args must be an array".to_owned()))?;
            let mut argv = vec![program.to_owned()];
            for item in items {
                argv.push(
                    item.as_str()
                        .ok_or_else(|| {
                            BackendFailure::Invalid("step args must be strings".to_owned())
                        })?
                        .to_owned(),
                );
            }
            Ok(argv)
        })
        .collect()
}

/// `null` reads as "no artifact outputs", the same as `[]` and an absent
/// property; see `parse_steps`.
fn parse_artifacts(args: &Value) -> Result<Vec<String>, BackendFailure> {
    let items = match args.get("artifact_outputs") {
        None | Some(Value::Null) => return Ok(Vec::new()),
        Some(value) => value.as_array().ok_or_else(|| {
            BackendFailure::Invalid("artifact_outputs must be an array".to_owned())
        })?,
    };
    if items.len() > MAX_ARTIFACTS {
        return Err(BackendFailure::Limit(
            "at most 64 artifact outputs are allowed".to_owned(),
        ));
    }
    let mut seen = BTreeSet::new();
    for item in items {
        let object = item.as_object().ok_or_else(|| {
            BackendFailure::Invalid("artifact output must be an object".to_owned())
        })?;
        if object.len() != 1 || !object.contains_key("path") {
            return Err(BackendFailure::Invalid(
                "artifact output must contain only path".to_owned(),
            ));
        }
        let path = object["path"]
            .as_str()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| BackendFailure::Invalid("artifact path must be nonempty".to_owned()))?;
        if Path::new(path).is_absolute()
            || path.split('/').any(|part| part == "..")
            || path.to_ascii_lowercase().starts_with(".agents/runtime/")
        {
            return Err(BackendFailure::Denied(format!(
                "invalid artifact path {path}"
            )));
        }
        if !seen.insert(path.to_owned()) {
            return Err(BackendFailure::Invalid(format!(
                "duplicate artifact path {path}"
            )));
        }
    }
    Ok(seen.into_iter().collect())
}

fn normalized_relative(path: &Path) -> Result<String, BackendFailure> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_string_lossy().into_owned()),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(BackendFailure::Denied(
                    "path escapes its helper root".to_owned(),
                ));
            }
        }
    }
    Ok(parts.join("/"))
}

fn exec_json(value: ExecValue, success: bool) -> Result<Value, BackendFailure> {
    let stdout = value.stdout.decode().map_err(helper_failure)?;
    let stderr = value.stderr.decode().map_err(helper_failure)?;
    Ok(json!({
        "status": if success { "completed" } else { "failed" },
        "exit_code": value.status,
        "stdout": ByteString::from_bytes(&stdout),
        "stderr": ByteString::from_bytes(&stderr),
        "stdout_truncated": value.stdout_truncated,
        "stderr_truncated": value.stderr_truncated,
    }))
}

fn retype<T, U>(outcome: BackendOutcome<T>, message: &str) -> BackendOutcome<U> {
    match outcome {
        BackendOutcome::Completed(Ok(_)) => {
            BackendOutcome::Completed(Err(BackendFailure::Protocol(message.to_owned())))
        }
        BackendOutcome::Completed(Err(error)) => BackendOutcome::Completed(Err(error)),
        BackendOutcome::Hold(hold) => BackendOutcome::Hold(hold),
        BackendOutcome::Unavailable(unavailable) => BackendOutcome::Unavailable(unavailable),
    }
}

fn invalid_outcome<T>(message: &str) -> BackendOutcome<T> {
    BackendOutcome::Completed(Err(BackendFailure::Invalid(message.to_owned())))
}

fn truncate_utf8(value: &str, bytes: usize) -> (&str, bool) {
    if value.len() <= bytes {
        return (value, false);
    }
    let mut end = bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    (&value[..end], true)
}

#[derive(Clone, Debug)]
struct PatchResult {
    content: String,
    added: usize,
    removed: usize,
    fuzz: usize,
}

impl PatchResult {
    fn create(diff: &str) -> Result<Self, BackendFailure> {
        let newline = detected_newline(diff);
        let mut lines = normalized_lines(diff);
        let mut output = Vec::with_capacity(lines.len());
        for line in lines.drain(..) {
            let Some(content) = line.strip_prefix('+') else {
                return Err(BackendFailure::Invalid(
                    "create_file expects only content lines prefixed with '+' (for example +hello); omit ---/+++ file headers and @@ hunks".to_owned(),
                ));
            };
            output.push(content.to_owned());
        }
        let content = output.join(newline);
        Ok(Self {
            added: logical_lines(&content),
            content,
            removed: 0,
            fuzz: 0,
        })
    }

    fn update(input: &str, diff: &str) -> Result<Self, BackendFailure> {
        let newline = if input.contains('\n') {
            detected_newline(input)
        } else {
            detected_newline(diff)
        };
        let normalized = input.replace("\r\n", "\n");
        let input_lines: Vec<String> = if normalized.is_empty() {
            Vec::new()
        } else {
            normalized.split('\n').map(ToOwned::to_owned).collect()
        };
        let lines = normalized_lines(diff);
        if lines.is_empty() {
            return Err(BackendFailure::Invalid(
                "update_file diff must contain a hunk".to_owned(),
            ));
        }
        let mut replacements = Vec::new();
        let mut search_from = 0usize;
        let mut index = 0usize;
        let mut added = 0usize;
        let mut removed = 0usize;
        let mut fuzz = 0usize;
        while index < lines.len() {
            if forbidden_boundary(&lines[index]) {
                return Err(BackendFailure::Invalid(
                    "diff must be V4A patch body only".to_owned(),
                ));
            }
            if let Some(anchor) = lines[index].strip_prefix("@@ ") {
                let (position, used_fuzz) = find_anchor(&input_lines, anchor, search_from)?;
                search_from = position + 1;
                fuzz += used_fuzz;
                index += 1;
            } else if lines[index] == "@@" {
                index += 1;
            } else if index != 0 {
                return Err(BackendFailure::Invalid(
                    "each patch section after the first must start with '@@'".to_owned(),
                ));
            }
            let start = index;
            let mut old = Vec::new();
            let mut new = Vec::new();
            let mut eof = false;
            while index < lines.len() && !lines[index].starts_with("@@") {
                if lines[index] == "*** End of File" {
                    eof = true;
                    index += 1;
                    break;
                }
                if forbidden_boundary(&lines[index]) || lines[index].starts_with("***") {
                    return Err(BackendFailure::Invalid(
                        "invalid V4A patch boundary".to_owned(),
                    ));
                }
                let raw = if lines[index].is_empty() {
                    " "
                } else {
                    &lines[index]
                };
                let (prefix, content) = raw.split_at(1);
                match prefix {
                    " " => {
                        old.push(content.to_owned());
                        new.push(content.to_owned());
                    }
                    "-" => {
                        old.push(content.to_owned());
                        removed += 1;
                    }
                    "+" => {
                        new.push(content.to_owned());
                        added += 1;
                    }
                    _ => {
                        return Err(BackendFailure::Invalid(
                            "patch lines must start with space, '+', or '-'".to_owned(),
                        ));
                    }
                }
                index += 1;
            }
            if index == start {
                return Err(BackendFailure::Invalid("update hunk is empty".to_owned()));
            }
            let (position, used_fuzz) = find_context(&input_lines, &old, search_from, eof)
                .ok_or_else(|| {
                    BackendFailure::Conflict("patch context did not match current file".to_owned())
                })?;
            fuzz += used_fuzz;
            search_from = position + old.len();
            replacements.push((position, old.len(), new));
            if eof && index != lines.len() {
                return Err(BackendFailure::Invalid(
                    "End of File must terminate diff".to_owned(),
                ));
            }
        }
        if added == 0 && removed == 0 {
            return Err(BackendFailure::Invalid(
                "patch contains no additions or deletions".to_owned(),
            ));
        }
        let mut destination = Vec::new();
        let mut input_cursor = 0usize;
        for (position, old_count, new) in replacements {
            if position < input_cursor || position.saturating_add(old_count) > input_lines.len() {
                return Err(BackendFailure::Invalid(
                    "patch contains overlapping or out-of-range hunks".to_owned(),
                ));
            }
            destination.extend_from_slice(&input_lines[input_cursor..position]);
            destination.extend(new);
            input_cursor = position + old_count;
        }
        destination.extend_from_slice(&input_lines[input_cursor..]);
        Ok(Self {
            content: destination.join(newline),
            added,
            removed,
            fuzz,
        })
    }
}

fn find_anchor(
    lines: &[String],
    anchor: &str,
    start: usize,
) -> Result<(usize, usize), BackendFailure> {
    if let Some(index) = lines
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, line)| (line == anchor).then_some(index))
    {
        return Ok((index, 0));
    }
    let trimmed = anchor.trim_matches([' ', '\t']);
    lines
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, line)| (line.trim_matches([' ', '\t']) == trimmed).then_some((index, 1)))
        .ok_or_else(|| BackendFailure::Conflict(format!("patch anchor did not match: {anchor}")))
}

fn find_context(
    lines: &[String],
    context: &[String],
    start: usize,
    eof: bool,
) -> Option<(usize, usize)> {
    if context.is_empty() {
        return Some((
            if eof {
                lines.len()
            } else {
                start.min(lines.len())
            },
            0,
        ));
    }
    let first = if eof {
        lines.len().saturating_sub(context.len())
    } else {
        start
    };
    for (fuzz, mode) in [(0, 0), (1, 1), (100, 2)] {
        for index in first..=lines.len().saturating_sub(context.len()) {
            let equal =
                lines[index..index + context.len()]
                    .iter()
                    .zip(context)
                    .all(|(left, right)| match mode {
                        0 => left == right,
                        1 => {
                            left.trim_end_matches([' ', '\t'])
                                == right.trim_end_matches([' ', '\t'])
                        }
                        _ => left.trim_matches([' ', '\t']) == right.trim_matches([' ', '\t']),
                    });
            if equal {
                return Some((index, fuzz));
            }
        }
    }
    if eof {
        find_context(lines, context, start, false).map(|(index, fuzz)| (index, fuzz + 10_000))
    } else {
        None
    }
}

fn normalized_lines(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line).to_owned())
        .collect();
    if lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    lines
}

fn detected_newline(text: &str) -> &'static str {
    if text.as_bytes().windows(2).any(|pair| pair == b"\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}
fn forbidden_boundary(line: &str) -> bool {
    matches!(line, "*** Begin Patch" | "*** End Patch")
        || line.starts_with("*** Update File:")
        || line.starts_with("*** Add File:")
        || line.starts_with("*** Delete File:")
}
fn logical_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.bytes().filter(|byte| *byte == b'\n').count() + usize::from(!text.ends_with('\n'))
    }
}

fn patch_result_json(
    path: &str,
    summary: &str,
    version: u64,
    value: &WriteValue,
    patch: &PatchResult,
) -> Value {
    json!({
        "path": path,
        "summary": summary,
        "artifact_version": version,
        "bytes": value.bytes,
        "sha256": value.sha256,
        "added_lines": patch.added,
        "removed_lines": patch.removed,
        "fuzz": patch.fuzz,
    })
}
