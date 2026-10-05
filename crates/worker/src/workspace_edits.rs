//! Worker-owned adapter from successful patch facts to the workspace child ledger.
use serde_json::json;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tools::{BackendFailure, ToolExecution};

fn edit_payload(
    event: &str,
    path: &std::path::Path,
    before: Option<&str>,
    after: Option<&str>,
) -> serde_json::Value {
    // JSON escaping can expand each byte sixfold. Stay below the 1 MiB child
    // protocol limit while retaining room for identities and envelope fields.
    if before
        .map_or(0, str::len)
        .saturating_add(after.map_or(0, str::len))
        > 128 * 1024
    {
        use sha2::{Digest, Sha256};
        let hashes = [before, after]
            .map(|content| content.map(|text| format!("{:x}", Sha256::digest(text.as_bytes()))));
        json!({"eventId":event,"path":path,"before":null,"after":null,"omittedContentHashes":hashes})
    } else {
        json!({"eventId":event,"path":path,"before":before,"after":after})
    }
}

pub struct WorkspaceEditRecorder {
    pub executable: PathBuf,
    pub workspace: PathBuf,
    pub workspace_roots: Vec<PathBuf>,
    pub state_root: PathBuf,
    pub workspace_id: String,
    pub session_id: String,
}

fn canonical_edit_target(target: &std::path::Path) -> io::Result<PathBuf> {
    let mut parent = target
        .parent()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    let mut missing = Vec::new();
    let canonical_parent = loop {
        match parent.canonicalize() {
            Ok(canonical) => break canonical,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let Some(component) = parent.file_name() else {
                    return Err(error);
                };
                missing.push(component.to_os_string());
                let Some(next) = parent.parent() else {
                    return Err(error);
                };
                parent = next;
            }
            Err(error) => return Err(error),
        }
    };
    let mut path = canonical_parent;
    for component in missing.into_iter().rev() {
        path.push(component);
    }
    path.push(
        target
            .file_name()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?,
    );
    Ok(path)
}

impl engine::CommittedEditRecorder for WorkspaceEditRecorder {
    fn abort(
        &self,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        self.invoke("abortTurnEdit", execution, path, before, after)
    }
    fn prepare(
        &self,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        self.invoke("prepareTurnEdit", execution, path, before, after)
    }
    fn record(
        &self,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        self.invoke("recordTurnEdit", execution, path, before, after)
    }
}

impl WorkspaceEditRecorder {
    fn invoke(
        &self,
        method: &'static str,
        execution: &ToolExecution,
        path: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) -> Result<(), BackendFailure> {
        let executable = self.executable.clone();
        let workspace = self.workspace.clone();
        let authority = json!({"stateRoot":self.state_root,"workspaceRoots":self.workspace_roots,"allowedOperations":[],"allowedRepositoryRoots":[]});
        let target = std::path::Path::new(path);
        let path = canonical_edit_target(target).map_err(|error| {
            BackendFailure::Unknown(format!("Canonical edit identity is unavailable: {error}"))
        })?;
        let request = json!({"workspaceId":self.workspace_id,"sessionId":self.session_id,"turnId":execution.turn.to_string(),
            "edit":edit_payload(&execution.call, &path, before, after)});
        // The backend is synchronous and may itself run inside a runtime. A short
        // owned thread avoids nested block_on while retaining process-group cleanup.
        let result = std::thread::spawn(move || -> Result<(), String> {
            let mut file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
            file.write_all(&serde_json::to_vec(&authority).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            file.flush().map_err(|e| e.to_string())?;
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())?;
            let response = runtime
                .block_on(workspace_service::process::invoke_with_authority(
                    &executable,
                    &workspace,
                    Some(file.path()),
                    method,
                    request,
                    std::time::Duration::from_secs(30),
                ))
                .map_err(|e| e.message)?;
            if let Some(error) = response.get("error") {
                return Err(error.to_string());
            }
            Ok(())
        })
        .join()
        .map_err(|_| BackendFailure::Unknown("file committed but edit recorder panicked".into()))?;
        result.map_err(|error| {
            BackendFailure::Unknown(format!(
                "{}: {error}",
                if method == "prepareTurnEdit" {
                    "Edit intent could not be persisted; file write was not started"
                } else {
                    "Edit settlement could not be persisted"
                }
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edit_identity_accepts_missing_nested_parent_under_canonical_workspace() {
        let workspace = tempfile::tempdir().unwrap();
        let path = workspace.path().join("new/nested/file.txt");
        assert_eq!(
            canonical_edit_target(&path).unwrap(),
            workspace
                .path()
                .canonicalize()
                .unwrap()
                .join("new/nested/file.txt")
        );
    }
    #[test]
    fn large_edit_capture_is_bounded_and_retains_content_identity() {
        let content = "\0".repeat(1024 * 1024);
        let first = edit_payload("event", std::path::Path::new("/file"), None, Some(&content));
        assert!(serde_json::to_vec(&first).unwrap().len() < 1024);
        assert!(first["after"].is_null());
        assert!(first["omittedContentHashes"][0].is_null());
        assert_eq!(first["omittedContentHashes"][1].as_str().unwrap().len(), 64);
        let changed = edit_payload(
            "event",
            std::path::Path::new("/file"),
            None,
            Some(&(content + "x")),
        );
        assert_ne!(
            first["omittedContentHashes"],
            changed["omittedContentHashes"]
        );
    }
    use engine::{DurableArtifactVersions, SystemToolBackend, SystemToolConfig, ToolBackend};
    use std::sync::Arc;
    use tools::{
        BackendTerminal, CancellationToken, HelperClient, HelperInvoker, NetworkPolicy,
        SandboxBackend, SandboxPolicy,
    };

    #[test]
    #[ignore = "requires current TEKES_WORKSPACE_SERVICE_BIN and TEKES_TOOL_HELPER_BIN"]
    fn actual_patch_helper_to_turn_ledger_round_trip() {
        let executable = PathBuf::from(
            std::env::var_os("TEKES_WORKSPACE_SERVICE_BIN").expect("workspace helper"),
        );
        let helper = PathBuf::from(std::env::var_os("TEKES_TOOL_HELPER_BIN").expect("tool helper"));
        let project = tempfile::tempdir().unwrap();
        let storage = tempfile::tempdir().unwrap();
        let root = project.path().canonicalize().unwrap();
        #[cfg(target_os = "macos")]
        let sandbox = SandboxBackend::DarwinSeatbeltV1;
        #[cfg(target_os = "linux")]
        let sandbox = SandboxBackend::LinuxLandlockSeccompV1;
        let client = HelperClient::sandboxed_with_scratch(
            helper,
            vec![("workspace".into(), root.clone())],
            SandboxPolicy {
                format: 1,
                read_roots: vec![root.to_string_lossy().into_owned()],
                write_roots: vec![root.to_string_lossy().into_owned()],
                network: NetworkPolicy::Deny,
                allow_process: true,
                scratch: None,
            },
            tools::probe_backend(sandbox),
        )
        .unwrap();
        let recorder = Arc::new(WorkspaceEditRecorder {
            executable: executable.clone(),
            workspace: root.clone(),
            workspace_roots: vec![root.clone()],
            state_root: storage.path().to_owned(),
            workspace_id: "workspace".into(),
            session_id: "session".into(),
        });
        let mut backend = SystemToolBackend::new(
            SystemToolConfig::workspace("workspace", &root),
            Some(Arc::new(client) as Arc<dyn HelperInvoker>),
            None,
            None,
            Some(Arc::new(
                DurableArtifactVersions::new(storage.path().join("versions")).unwrap(),
            )),
            CancellationToken::default(),
        )
        .with_edit_recorder(recorder);
        let large_content = "large file line\n".repeat(80_000);
        let large_diff = large_content
            .lines()
            .map(|line| format!("+{line}"))
            .collect::<Vec<_>>()
            .join("\n");
        for (turn, call, args) in [
            (
                1,
                "create",
                json!({"operation":"create_file","path":"file.txt","diff":"+alpha\n+beta","expected_artifact_version":0,"summary":"create"}),
            ),
            (
                2,
                "update",
                json!({"operation":"update_file","path":"file.txt","diff":"@@\n alpha\n-beta\n+gamma\n+delta","expected_artifact_version":1,"summary":"update"}),
            ),
            (
                3,
                "create-large",
                json!({"operation":"create_file","path":"large.txt","diff":large_diff,"expected_artifact_version":0,"summary":"create large file"}),
            ),
        ] {
            let invocation =
                schema::IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap();
            let execution = ToolExecution {
                thread: "session".into(),
                call: call.into(),
                name: "apply_patch".into(),
                attempt: "attempt".into(),
                invocation: invocation.clone(),
                side_effectful: true,
                turn,
                timestamp: "2026-09-08T00:00:00Z".into(),
            };
            let outcome = backend.execute(&execution, &invocation);
            match outcome {
                BackendTerminal::Completed(value) => assert!(
                    serde_json::to_value(value)
                        .unwrap()
                        .get("artifact_version")
                        .is_some()
                ),
                other => panic!("patch failed: {other:?}"),
            }
        }
        assert_eq!(
            std::fs::read_to_string(root.join("file.txt")).unwrap(),
            "alpha\ngamma\ndelta"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("large.txt")).unwrap(),
            large_content.trim_end_matches('\n')
        );
        let mut authority = tempfile::NamedTempFile::new().unwrap();
        authority.write_all(&serde_json::to_vec(&json!({"stateRoot":storage.path(),"allowedOperations":[],"allowedRepositoryRoots":[]})).unwrap()).unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        for (turn, additions, deletions) in [("1", 2, 0), ("2", 2, 1)] {
            let result = runtime
                .block_on(workspace_service::process::invoke_with_authority(
                    &executable,
                    &root,
                    Some(authority.path()),
                    "turnChanges",
                    json!({"workspaceId":"workspace","sessionId":"session","turnId":turn}),
                    std::time::Duration::from_secs(10),
                ))
                .unwrap();
            assert_eq!(result["result"]["additions"], additions, "{result}");
            assert_eq!(result["result"]["deletions"], deletions, "{result}");
        }
        let large = runtime
            .block_on(workspace_service::process::invoke_with_authority(
                &executable,
                &root,
                Some(authority.path()),
                "turnChanges",
                json!({"workspaceId":"workspace","sessionId":"session","turnId":"3"}),
                std::time::Duration::from_secs(10),
            ))
            .unwrap();
        assert_eq!(large["result"]["fileCount"], 1, "{large}");
        assert!(large["result"]["additions"].is_null(), "{large}");
        assert!(large["result"]["deletions"].is_null(), "{large}");
    }
}
