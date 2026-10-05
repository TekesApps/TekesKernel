//! Additive Host workspace routes. Workspace roots and mutation grants are server-owned.
use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use endpoint::{EndpointHostCall, ManagementStore, MethodClass, NativeEndpoint};
use schema::IJsonValue;
use serde_json::{Value, json};
use std::collections::{BTreeSet, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};

const READS: &[&str] = &[
    "capabilities",
    "filesList",
    "filesSearch",
    "filesRead",
    "gitRepository",
    "gitStatus",
    "gitBranches",
    "gitLog",
    "gitDiff",
    "gitChangesNavigator",
    "turnChanges",
];
const WRITES: &[&str] = &["gitCommit", "gitPush", "gitChangeBranch", "filesWrite"];
fn failure(code: &str, message: impl Into<String>) -> ProductionRouteFailure {
    ProductionRouteFailure::new(code, message.into(), IJsonValue::parse_str("{}").unwrap())
}
fn wire(value: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|e| failure("internal", e.to_string()))?)
        .map_err(|e| failure("internal", e.to_string()))
}
fn method(operation: &str) -> Option<&str> {
    let name = operation.strip_prefix("tekesWorkspace.")?;
    (READS.contains(&name) || WRITES.contains(&name)).then_some(name)
}

pub struct WorkspaceRoutes {
    root: PathBuf,
    executable: PathBuf,
    management: ManagementStore,
    endpoint: NativeEndpoint,
}
impl WorkspaceRoutes {
    pub fn open(root: &Path, executable: PathBuf) -> Result<Self, ProductionRouteFailure> {
        if !executable.is_absolute() || !executable.is_file() {
            return Err(failure("unavailable", "Workspace helper is not installed"));
        }
        Ok(Self {
            root: root.to_owned(),
            executable,
            management: ManagementStore::open(root)
                .map_err(|e| failure("internal", e.to_string()))?,
            endpoint: NativeEndpoint::open(root).map_err(|e| failure("internal", e.to_string()))?,
        })
    }
    fn authority(&self) -> Result<Value, ProductionRouteFailure> {
        // This file is an operator-managed capability grant. It is not writable
        // through the public service. Missing configuration is deliberately deny-all.
        let path = self.root.join("workspace-service/git-authority.json");
        let mut authority = match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice::<Value>(&bytes)
                .map_err(|e| failure("invalid-authority", e.to_string()))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                json!({"allowedOperations":[],"allowedRepositoryRoots":[]})
            }
            Err(e) => return Err(failure("invalid-authority", e.to_string())),
        };
        if !authority.is_object() {
            return Err(failure("invalid-authority", "Expected authority object"));
        }
        authority["stateRoot"] = json!(self.root.join("workspace-service"));
        serde_json::from_value::<workspace_service::git::MutationAuthority>(authority.clone())
            .map_err(|e| failure("invalid-authority", e.to_string()))?;
        Ok(authority)
    }
}
impl ProductionEndpointRoutes for WorkspaceRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        READS
            .iter()
            .chain(WRITES)
            .map(|name| format!("tekesWorkspace.{name}"))
            .collect()
    }
    fn extension_method_class(&self, name: &str) -> Option<MethodClass> {
        method(name).map(|m| {
            if WRITES.contains(&m) {
                MethodClass::Mutation
            } else {
                MethodClass::ReadOnly
            }
        })
    }
    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        let name = method(operation)
            .ok_or_else(|| failure("unsupported-capability", "Unknown workspace method"))?;
        let object = payload
            .as_object()
            .ok_or_else(|| failure("invalid-request", "Expected request object"))?;
        let fields: &[&str] = match name {
            "capabilities" => &[],
            "filesList" => &["path", "limit"],
            "filesSearch" => &["query", "limit"],
            "filesRead" => &["path", "maxBytes"],
            "filesWrite" => &["path", "content", "expectedRevision"],
            "gitRepository" | "gitStatus" | "gitBranches" => &["repositoryPath"],
            "gitLog" => &["repositoryPath", "limit", "offset", "head"],
            "gitDiff" => &["repositoryPath", "scope", "paths", "revision"],
            "gitChangesNavigator" => &["includeClean"],
            "gitCommit" => &["repositoryPath", "message", "paths"],
            "gitPush" => &["repositoryPath", "remote"],
            "gitChangeBranch" => &["repositoryPath", "operation", "name"],
            "turnChanges" => &["sessionId", "turnId", "includeDiff"],
            _ => unreachable!(),
        };
        if object.keys().any(|key| {
            (name == "capabilities" || key != "workspaceId") && !fields.contains(&key.as_str())
        }) {
            return Err(failure("invalid-request", "Unknown request field"));
        }
        if name != "capabilities" && payload["workspaceId"].as_str().is_none_or(str::is_empty) {
            return Err(failure("invalid-request", "Missing workspace identity"));
        }
        let required: &[&str] = match name {
            "filesRead" => &["path"],
            "filesWrite" => &["path", "expectedRevision"],
            "filesSearch" => &["query"],
            "gitCommit" => &["message"],
            "gitPush" => &["remote"],
            "gitChangeBranch" => &["operation", "name"],
            "turnChanges" => &["sessionId", "turnId"],
            _ => &[],
        };
        if required
            .iter()
            .any(|key| payload[key].as_str().is_none_or(str::is_empty))
        {
            return Err(failure("invalid-request", "Missing required request field"));
        }
        if name == "filesWrite" {
            serde_json::from_value::<workspace_service::write::Request>(payload.clone())
                .map_err(|e| failure("invalid-request", e.to_string()))?;
        } else if name.starts_with("files") {
            serde_json::from_value::<workspace_service::FilesRequest>(payload.clone())
                .map_err(|e| failure("invalid-request", e.to_string()))?;
        }
        if name.starts_with("git") {
            serde_json::from_value::<workspace_service::git::Query>(payload.clone())
                .map_err(|e| failure("invalid-request", e.to_string()))?;
        }
        if name == "turnChanges" {
            serde_json::from_value::<workspace_service::turn::Request>(payload.clone())
                .map_err(|e| failure("invalid-request", e.to_string()))?;
        }
        Ok(())
    }
    fn extension_failure_is_exact(&self, operation: &str, error: &ProductionRouteFailure) -> bool {
        method(operation).is_some()
            && [
                "invalid-request",
                "unavailable",
                "workspace-missing",
                "session-outside-workspace",
                "invalid-authority",
                "permission-denied",
                "outcome-unknown",
                "path-missing",
                "path-outside-workspace",
                "repository-outside-workspace",
                "not-file",
                "snapshot-changed",
                "revision-conflict",
                "not-repository",
                "repository-selection-required",
                "invalid-output",
                "git-failed",
                "invalid-remote",
                "no-branch",
                "upstream-mismatch",
                "conflicts",
                "stale-selection",
                "busy",
                "timeout",
                "output-limit",
                "process-failed",
                "invalid-response",
                "io-error",
                "discovery-limit",
                "storage-limit",
                "corrupt-ledger",
                "internal",
            ]
            .contains(&error.code.as_str())
    }
    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        self.validate_extension_payload(&request.operation, payload)?;
        let name = method(&request.operation).unwrap();
        let authority = self.authority()?;
        if name == "capabilities" {
            // Advertise callable features under the current service grants, matching
            // the DSH implementation. Route registration remains independent of grants.
            let operations = authority["allowedOperations"].as_array().unwrap();
            let allowed =
                |operation: &str| operations.iter().any(|v| v.as_str() == Some(operation));
            let mut methods = READS
                .iter()
                .copied()
                .filter(|m| *m != "capabilities")
                .collect::<Vec<_>>();
            if allowed("commit") {
                methods.push("gitCommit");
            }
            if allowed("push") {
                methods.push("gitPush");
            }
            if operations
                .iter()
                .any(|v| v.as_str().is_some_and(|s| s.starts_with("branch.")))
            {
                methods.push("gitChangeBranch");
            }
            let file_writes = authority["allowFileWrites"].as_bool().unwrap_or(false);
            if file_writes {
                methods.push("filesWrite");
            }
            return wire(
                &json!({"version":1,"methods":methods,"mutations":file_writes || !operations.is_empty(),"allowedGitOperations":authority["allowedOperations"],"turnChanges":true}),
            );
        }
        if request.recovering && WRITES.contains(&name) {
            return Err(failure(
                "outcome-unknown",
                "A previous Git mutation may have completed; refresh repository state before issuing a new operation",
            ));
        }
        let workspace_id = payload["workspaceId"].as_str().unwrap();
        let workspace = PathBuf::from(
            self.management
                .workspace_path(workspace_id)
                .map_err(|e| failure("workspace-missing", e.to_string()))?,
        );
        if name == "turnChanges" {
            let session = payload["sessionId"].as_str().unwrap();
            let sessions = self
                .endpoint
                .list_sessions(&HashSet::new())
                .map_err(|e| failure("internal", e.to_string()))?;
            if !sessions
                .iter()
                .any(|s| s.session_id == session && s.workspace_id == workspace_id)
            {
                return Err(failure(
                    "session-outside-workspace",
                    "Session does not belong to workspace",
                ));
            }
        }
        let mut authority_file =
            tempfile::NamedTempFile::new().map_err(|e| failure("internal", e.to_string()))?;
        authority_file
            .write_all(
                &serde_json::to_vec(&authority).map_err(|e| failure("internal", e.to_string()))?,
            )
            .map_err(|e| failure("internal", e.to_string()))?;
        authority_file
            .flush()
            .map_err(|e| failure("internal", e.to_string()))?;
        let executable = self.executable.clone();
        let name = name.to_owned();
        let payload = payload.clone();
        let response = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| failure("internal", e.to_string()))?;
            runtime
                .block_on(workspace_service::process::invoke_with_authority(
                    &executable,
                    &workspace,
                    Some(authority_file.path()),
                    &name,
                    payload,
                    std::time::Duration::from_secs(if name == "gitPush" { 135 } else { 30 }),
                ))
                .map_err(|e| failure(e.code, e.message))
        })
        .join()
        .map_err(|_| failure("internal", "Workspace invocation panicked"))??;
        if let Some(error) = response.get("error") {
            return Err(failure(
                error["code"].as_str().unwrap_or("internal"),
                error["message"]
                    .as_str()
                    .unwrap_or("Workspace service failed"),
            ));
        }
        wire(&response["result"])
    }
}
