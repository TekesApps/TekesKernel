use std::collections::BTreeMap;
use std::io::Write as _;
use std::os::unix::process::CommandExt as _;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use schema::IJsonValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const HARD_TIMEOUT_MS: u64 = 300_000;
const HARD_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookPhase {
    Pre,
    Post,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookResultView {
    pub outcome: IJsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IJsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<IJsonValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookRequest {
    pub format: u64,
    pub hook_id: String,
    pub phase: HookPhase,
    pub call: String,
    pub name: String,
    pub invocation: IJsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HookResultView>,
}

impl HookRequest {
    pub fn validate(&self) -> Result<(), HookError> {
        if self.format != 1 {
            return Err(HookError::Invalid("format must equal 1"));
        }
        if self.hook_id.is_empty() || self.call.is_empty() || self.name.is_empty() {
            return Err(HookError::Invalid(
                "hook_id, call, and name must be non-empty",
            ));
        }
        match (self.phase, self.result.is_some()) {
            (HookPhase::Pre, false) | (HookPhase::Post, true) => Ok(()),
            (HookPhase::Pre, true) => Err(HookError::Invalid("pre request forbids result")),
            (HookPhase::Post, false) => Err(HookError::Invalid("post request requires result")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreVerdict {
    Allow {},
    Deny {
        reason: String,
    },
    Mutate {
        invocation: IJsonValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    },
    Ask {
        scope: String,
        question: IJsonValue,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostVerdict {
    Allow {
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<IJsonValue>,
    },
    Deny {
        reason: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<IJsonValue>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum HookResponse {
    Pre {
        format: u64,
        hook_id: String,
        call: String,
        verdict: PreVerdict,
    },
    Post {
        format: u64,
        hook_id: String,
        call: String,
        verdict: PostVerdict,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawResponse<T> {
    format: u64,
    hook_id: String,
    call: String,
    verdict: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookFailureMode {
    Open,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookBinding {
    pub format: u64,
    pub id: String,
    pub argv: Vec<String>,
    pub phase: HookPhase,
    pub failure: HookFailureMode,
    pub timeout_ms: u64,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

impl HookBinding {
    pub fn validate(&self) -> Result<(), HookError> {
        if self.format != 1 {
            return Err(HookError::Invalid("hook binding format must equal 1"));
        }
        if self.id.is_empty() || self.argv.first().is_none_or(String::is_empty) {
            return Err(HookError::Invalid("hook id and argv must be non-empty"));
        }
        if self.timeout_ms == 0 || self.timeout_ms > HARD_TIMEOUT_MS {
            return Err(HookError::Invalid("hook timeout is outside hard limits"));
        }
        if self.stdout_bytes == 0
            || self.stderr_bytes == 0
            || self.stdout_bytes > HARD_OUTPUT_BYTES
            || self.stderr_bytes > HARD_OUTPUT_BYTES
        {
            return Err(HookError::Invalid("hook output cap is outside hard limits"));
        }
        Ok(())
    }
}

pub fn decode_hook_binding(bytes: &[u8], logical_id: &str) -> Result<HookBinding, HookError> {
    let binding: HookBinding = decode_one_line(bytes)?;
    binding.validate()?;
    if binding.id != logical_id {
        return Err(HookError::Invalid(
            "hook binding id must equal its logical instruction path",
        ));
    }
    Ok(binding)
}

#[derive(Debug, Error)]
pub enum HookError {
    #[error("invalid hook contract: {0}")]
    Invalid(&'static str),
    #[error("hook JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("hook process I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("hook cancelled")]
    Cancelled,
    #[error("hook timed out")]
    Timeout,
    #[error("hook stdout exceeded its cap")]
    OutputLimit,
    #[error("hook exited with status {0}")]
    Exit(i32),
    #[error("hook response correlation mismatch")]
    Correlation,
    #[error("hook response has unexpected phase")]
    Phase,
}

pub fn encode_hook_line<T: Serialize>(value: &T) -> Result<Vec<u8>, HookError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn decode_hook_request(bytes: &[u8]) -> Result<HookRequest, HookError> {
    let request: HookRequest = decode_one_line(bytes)?;
    request.validate()?;
    Ok(request)
}

pub fn decode_hook_response(
    bytes: &[u8],
    request: &HookRequest,
) -> Result<HookResponse, HookError> {
    match request.phase {
        HookPhase::Pre => {
            let raw: RawResponse<PreVerdict> = decode_one_line(bytes)?;
            validate_response(&raw, request)?;
            Ok(HookResponse::Pre {
                format: raw.format,
                hook_id: raw.hook_id,
                call: raw.call,
                verdict: raw.verdict,
            })
        }
        HookPhase::Post => {
            let raw: RawResponse<PostVerdict> = decode_one_line(bytes)?;
            validate_response(&raw, request)?;
            Ok(HookResponse::Post {
                format: raw.format,
                hook_id: raw.hook_id,
                call: raw.call,
                verdict: raw.verdict,
            })
        }
    }
}

fn validate_response<T>(raw: &RawResponse<T>, request: &HookRequest) -> Result<(), HookError> {
    if raw.format != 1 {
        return Err(HookError::Invalid("response format must equal 1"));
    }
    if raw.hook_id != request.hook_id || raw.call != request.call {
        return Err(HookError::Correlation);
    }
    Ok(())
}

pub(crate) fn decode_one_line<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, HookError> {
    let Some(body) = bytes.strip_suffix(b"\n") else {
        return Err(HookError::Invalid("message requires final LF"));
    };
    if body.is_empty() || body.contains(&b'\n') || body.contains(&b'\r') {
        return Err(HookError::Invalid("message must be exactly one line"));
    }
    let parsed = IJsonValue::parse(body).map_err(|_| HookError::Invalid("invalid I-JSON"))?;
    if parsed
        .canonical_bytes()
        .map_err(|_| HookError::Invalid("invalid canonical JSON"))?
        != body
    {
        return Err(HookError::Invalid("message is not canonical"));
    }
    Ok(serde_json::from_slice(body)?)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessHook;

impl ProcessHook {
    pub fn run(binding: &HookBinding, request: &HookRequest) -> Result<HookResponse, HookError> {
        binding.validate()?;
        request.validate()?;
        if binding.phase != request.phase || binding.id != request.hook_id {
            return Err(HookError::Phase);
        }
        let stdout = run_hook_process(binding, encode_hook_line(request)?, &|| false)?;
        decode_hook_response(&stdout, request)
    }
}

/// Shared bounded process carrier. All pipes are drained concurrently, and the
/// deadline includes stdin writes and descendants retaining stdout/stderr.
pub(crate) fn run_hook_process(
    binding: &HookBinding,
    input: Vec<u8>,
    cancelled: &dyn Fn() -> bool,
) -> Result<Vec<u8>, HookError> {
    binding.validate()?;
    if cancelled() {
        return Err(HookError::Cancelled);
    }
    let mut command = Command::new(&binding.argv[0]);
    command
        .args(&binding.argv[1..])
        .env_clear()
        .envs(&binding.env)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command.process_group(0);
    let mut child = command.spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or(HookError::Invalid("hook stdin unavailable"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or(HookError::Invalid("hook stdout unavailable"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or(HookError::Invalid("hook stderr unavailable"))?;
    let (tx, rx) = std::sync::mpsc::channel();
    let output_tx = tx.clone();
    let error_tx = tx.clone();
    let stdout_cap = binding.stdout_bytes;
    let stderr_cap = binding.stderr_bytes;
    thread::spawn(move || {
        let _ = tx.send((0, stdin.write_all(&input).map(|()| (Vec::new(), false))));
    });
    thread::spawn(move || {
        let _ = output_tx.send((1, read_capped(stdout, stdout_cap)));
    });
    thread::spawn(move || {
        let _ = error_tx.send((2, read_capped(stderr, stderr_cap)));
    });
    let deadline = Instant::now() + Duration::from_millis(binding.timeout_ms);
    let mut status = None;
    let mut results = Vec::new();
    while status.is_none() || results.len() < 3 {
        if status.is_none() {
            status = child.try_wait()?;
        }
        while let Ok(result) = rx.try_recv() {
            results.push(result);
        }
        if status.is_some() && results.len() == 3 {
            break;
        }
        if cancelled() {
            terminate_then_kill(&mut child);
            return Err(HookError::Cancelled);
        }
        if Instant::now() >= deadline {
            terminate_then_kill(&mut child);
            return Err(HookError::Timeout);
        }
        thread::sleep(Duration::from_millis(5));
    }
    let status = status.expect("child reaped");
    if !status.success() {
        return Err(HookError::Exit(status.code().unwrap_or(-1)));
    }
    let mut output = Vec::new();
    for (pipe, result) in results {
        let (bytes, truncated) = result?;
        if truncated {
            return Err(HookError::OutputLimit);
        }
        if pipe == 1 {
            output = bytes;
        }
    }
    Ok(output)
}

fn terminate_then_kill(child: &mut std::process::Child) {
    let group = -(child.id() as i32);
    // SAFETY: the hook child was placed in a fresh process group.
    unsafe {
        libc::kill(group, libc::SIGTERM);
    }
    let deadline = Instant::now() + Duration::from_millis(100);
    while Instant::now() < deadline {
        let _ = child.try_wait();
        thread::sleep(Duration::from_millis(5));
    }
    // SAFETY: the process group remains identified by the child's negated pid.
    unsafe {
        libc::kill(group, libc::SIGKILL);
    }
    let _ = child.wait();
}

fn read_capped(
    mut reader: impl std::io::Read,
    cap: usize,
) -> Result<(Vec<u8>, bool), std::io::Error> {
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
