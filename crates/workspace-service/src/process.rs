//! Bounded, cancellable invocation of the installed workspace helper.
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

use crate::{Failure, fail};

async fn read_bounded(reader: impl AsyncRead + Unpin, maximum: u64) -> Result<Vec<u8>, Failure> {
    let mut bytes = Vec::new();
    reader.take(maximum + 1).read_to_end(&mut bytes).await?;
    if bytes.len() as u64 > maximum {
        return Err(fail(
            "output-limit",
            "Workspace service output exceeded limit",
        ));
    }
    Ok(bytes)
}

/// `executable` and `registered_root` are parent-owned installation/registry values.
/// JSON payloads cannot select an executable or inject command-line arguments.
pub async fn invoke(
    executable: &Path,
    registered_root: &Path,
    method: &str,
    request: Value,
    deadline: Duration,
) -> Result<Value, Failure> {
    invoke_with_authority(executable, registered_root, None, method, request, deadline).await
}

pub async fn invoke_with_authority(
    executable: &Path,
    registered_root: &Path,
    authority_file: Option<&Path>,
    method: &str,
    request: Value,
    deadline: Duration,
) -> Result<Value, Failure> {
    let complete_file = method == "filesRead" && request["maxBytes"].as_u64() == Some(0);
    let input = serde_json::to_vec(&serde_json::json!({"method":method,"request":request}))
        .map_err(|error| fail("invalid-request", error.to_string()))?;
    let maximum = if method == "filesWrite" {
        8 * 1024 * 1024
    } else {
        1024 * 1024
    };
    if input.len() > maximum {
        return Err(fail("invalid-request", "Request too large"));
    }
    let mut command = Command::new(executable);
    command.arg("--root").arg(registered_root);
    if let Some(authority_file) = authority_file {
        command.arg("--authority-file").arg(authority_file);
    }
    #[cfg(unix)]
    command.process_group(0);
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    // The helper and Git descendants share this owned process group. Dropping
    // the request future must terminate all of them, not only the helper PID.
    #[cfg(unix)]
    let _group = ProcessGroup(
        child
            .id()
            .and_then(|id| rustix::process::Pid::from_raw(id as i32)),
    );
    let mut stdin = child.stdin.take().expect("piped stdin");
    let stdout = child.stdout.take().expect("piped stdout");
    let stderr = child.stderr.take().expect("piped stderr");
    let result = tokio::time::timeout(deadline, async {
        tokio::try_join!(
            async move {
                stdin.write_all(&input).await?;
                stdin.shutdown().await?;
                drop(stdin);
                Ok::<_, Failure>(())
            },
            async move {
                if complete_file {
                    let mut stdout = stdout;
                    let mut bytes = Vec::new();
                    stdout.read_to_end(&mut bytes).await?;
                    Ok::<_, Failure>(bytes)
                } else {
                    read_bounded(stdout, 8 * 1024 * 1024).await
                }
            },
            read_bounded(stderr, 64 * 1024),
            async { child.wait().await.map_err(Failure::from) },
        )
    })
    .await;
    let (_, output, stderr, exit) = match result {
        Ok(Ok(result)) => result,
        failure => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            return Err(match failure {
                Ok(Err(error)) => error,
                _ => fail("timeout", "Workspace service timed out"),
            });
        }
    };
    if !exit.success() {
        return Err(fail(
            "process-failed",
            format!(
                "Workspace service exited {exit}: {}",
                String::from_utf8_lossy(&stderr)
            ),
        ));
    }
    let envelope: Value = serde_json::from_slice(&output).map_err(|_| {
        fail(
            "invalid-response",
            "Workspace service returned invalid JSON",
        )
    })?;
    if !envelope.is_object()
        || (envelope.get("result").is_some() == envelope.get("error").is_some())
    {
        return Err(fail(
            "invalid-response",
            "Expected exactly one result or error",
        ));
    }
    Ok(envelope)
}

#[cfg(unix)]
struct ProcessGroup(Option<rustix::process::Pid>);

#[cfg(unix)]
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        if let Some(pid) = self.0 {
            let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
        }
    }
}
