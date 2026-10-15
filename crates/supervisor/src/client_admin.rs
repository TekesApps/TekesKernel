//! Production workspace and environment administration endpoint routes.
//! Provider configuration is owned by the launching application.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use endpoint::{DurableHandoffProof, EndpointHostCall, MethodClass, RpcDurableIdentity};
use profile::{ConfigRepository, WorkspacePolicy};
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{AtomicPublisher, NamedLock};

use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use crate::mcp_runtime::{MAX_REPORTED_TOOL_REFUSALS, McpRefusedTool, refused_tool_json};
use crate::process_host::{PolicyCandidateCause, PolicyCandidateError, ProductionProcessHost};

mod session_folders;
pub use session_folders::SESSION_FOLDER_METHODS;

pub const ENVIRONMENT_METHODS: [(&str, MethodClass); 2] = [
    ("environments.list", MethodClass::ReadOnly),
    ("environments.save", MethodClass::Mutation),
];

pub const WORKSPACE_POLICY_METHODS: [(&str, MethodClass); 2] = [
    ("workspace.policy.get", MethodClass::ReadOnly),
    ("workspace.policy.set", MethodClass::Mutation),
];

pub const WORKSPACE_FOLDER_METHODS: [(&str, MethodClass); 3] = [
    ("workspace.listFolders", MethodClass::ReadOnly),
    ("workspace.addFolder", MethodClass::Mutation),
    ("workspace.removeFolder", MethodClass::Mutation),
];

pub const FOLDER_METHODS: [(&str, MethodClass); 2] = [
    ("folders.list", MethodClass::ReadOnly),
    ("folders.relocate", MethodClass::Mutation),
];

pub struct ClientAdminRoutes {
    root: PathBuf,
    repository: ConfigRepository,
    process_host: Arc<ProductionProcessHost>,
    mutation_gate: Mutex<()>,
    journal: AdminMutationJournal,
}

impl ClientAdminRoutes {
    pub fn new(
        root: impl AsRef<std::path::Path>,
        process_host: Arc<ProductionProcessHost>,
    ) -> Result<Self, ProductionRouteFailure> {
        let root = root.as_ref();
        let routes = Self {
            root: root.to_path_buf(),
            repository: ConfigRepository::open(root).map_err(internal_profile)?,
            process_host,
            mutation_gate: Mutex::new(()),
            journal: AdminMutationJournal::open(root).map_err(internal_store)?,
        };
        routes.recover_pending_intents()?;
        Ok(routes)
    }

    /// Returns the atomic workspace and environment administration groups.
    #[must_use]
    pub fn routes(self: &Arc<Self>) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
        vec![
            Arc::new(AdminCapabilityRoutes::new(
                "sessionFolders.v1",
                &SESSION_FOLDER_METHODS,
                Arc::clone(self),
            )),
            Arc::new(AdminCapabilityRoutes::new(
                "folders.v1",
                &FOLDER_METHODS,
                Arc::clone(self),
            )),
            Arc::new(AdminCapabilityRoutes::new(
                "environments.v1",
                &ENVIRONMENT_METHODS,
                Arc::clone(self),
            )),
            Arc::new(AdminCapabilityRoutes::new(
                "workspacePolicy.v1",
                &WORKSPACE_POLICY_METHODS,
                Arc::clone(self),
            )),
            Arc::new(AdminCapabilityRoutes::new(
                "workspaceFolders.v1",
                &WORKSPACE_FOLDER_METHODS,
                Arc::clone(self),
            )),
        ]
    }

    fn recover_pending_intents(&self) -> Result<(), ProductionRouteFailure> {
        for record in self.journal.records()? {
            // Provider and settings edits from the removed providerAdmin.v1 are
            // abandoned: application launch configuration owns both files.
            if matches!(
                record.authority.as_str(),
                "config/providers.json" | "config/settings.json"
            ) {
                continue;
            }
            if record.phase == AdminPhase::Committed {
                continue;
            }
            if self.journal.authority_digest(&record.authority)?.as_deref()
                != Some(&record.desired_sha256)
            {
                match record.authority.as_str() {
                    "config/environments.json" => {
                        let desired = profile::EnvironmentsConfig::decode(&record.desired)
                            .map_err(internal_profile)?;
                        self.repository
                            .publish_environments(record.expected_revision, &desired)
                            .map_err(internal_profile)?;
                    }
                    authority
                        if authority.starts_with("workspaces/") && authority.ends_with(".json") =>
                    {
                        let desired = profile::WorkspaceConfig::decode(&record.desired)
                            .map_err(map_policy)?;
                        let previous = self
                            .repository
                            .resolve(&desired.id)
                            .map_err(internal_profile)?;
                        let policy = desired.policy.clone().unwrap_or_default();
                        let published = {
                            let _management = self.lock_management()?;
                            self.repository
                                .publish_workspace_policy(
                                    &desired.id,
                                    record.expected_revision,
                                    policy,
                                )
                                .map_err(map_policy)?
                        };
                        if published.canonical_bytes().map_err(map_policy)? != record.desired {
                            return Err(internal(
                                "recovered workspace publication differs from intent",
                            ));
                        }
                        self.process_host
                            .workspace_policy_published(&desired.id, &previous)
                            .map_err(internal_daemon)?;
                    }
                    _ => {
                        return Err(internal(
                            "config mutation intent names an unknown authority",
                        ));
                    }
                }
            }
            if record.authority.starts_with("workspaces/") {
                let desired =
                    profile::WorkspaceConfig::decode(&record.desired).map_err(map_policy)?;
                self.process_host.workspace_policy_recovered(&desired.id);
            }
            self.journal.commit_record(&record.rpc_id)?;
        }
        Ok(())
    }

    fn list_environments(&self) -> Result<IJsonValue, ProductionRouteFailure> {
        let config = self.repository.environments().map_err(internal_profile)?;
        let mut environments = config.environments.clone();
        if !environments
            .iter()
            .any(|environment| environment.id == schema::DEFAULT_ENVIRONMENT)
        {
            environments.push(profile::Environment::local());
        }
        environments.sort_by(|left, right| left.id.cmp(&right.id));
        to_ijson(&json!({"format":1,"revision":config.revision,
            "defaultEnvironment":schema::DEFAULT_ENVIRONMENT,
            "environments":environments.into_iter().map(EnvironmentView::from).collect::<Vec<_>>()}))
    }

    fn save_environment(
        &self,
        request: &EndpointHostCall,
        input: EnvironmentSaveRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let mut desired = self.repository.environments().map_err(internal_profile)?;
        if desired.revision != input.expected_revision {
            return Err(stale(input.expected_revision, desired.revision));
        }
        desired.revision = next_revision(desired.revision)?;
        let environment: profile::Environment = input.environment.into();
        desired.environments.retain(|old| old.id != environment.id);
        desired.environments.push(environment.clone());
        desired
            .environments
            .sort_by(|left, right| left.id.cmp(&right.id));
        // Validate before preparing any intent or touching authority bytes.
        let bytes = desired.canonical_bytes().map_err(map_environment)?;
        let result = to_ijson(&json!({"format":1,"revision":desired.revision,
            "environment":EnvironmentView::from(environment)}))?;
        self.journal.begin(
            request,
            "config/environments.json",
            input.expected_revision,
            desired.revision,
            &bytes,
            &result,
        )?;
        if let Err(error) = self
            .repository
            .publish_environments(input.expected_revision, &desired)
        {
            self.journal.abort(request)?;
            return Err(map_environment(error));
        }
        // Existing workers retain immutable descriptors. The environment is
        // a reference resolved afresh by the next ordinary launch.
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }

    fn get_policy(&self, input: PolicyGetRequest) -> Result<IJsonValue, ProductionRouteFailure> {
        let workspace = self
            .repository
            .workspace(&input.workspace_id)
            .map_err(map_workspace)?;
        to_ijson(&json!({"format":1,"revision":workspace.revision,
            "policy":workspace.policy.unwrap_or_default()}))
    }

    fn set_policy(
        &self,
        request: &EndpointHostCall,
        input: PolicySetRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        if let Some(result) = self.journal.recover(request)? {
            mark_handoff(request)?;
            return Ok(result);
        }
        let workspace = self
            .repository
            .workspace(&input.workspace_id)
            .map_err(map_workspace)?;
        if workspace.revision != input.expected_revision {
            return Err(stale(input.expected_revision, workspace.revision));
        }
        let unavailable = self
            .process_host
            .validate_workspace_policy_candidate(&input.workspace_id, &input.policy)
            .map_err(policy_candidate_failure)?;
        let previous = self
            .repository
            .resolve(&input.workspace_id)
            .map_err(internal_profile)?;
        let mut desired_workspace = workspace.clone();
        desired_workspace.revision = next_revision(desired_workspace.revision)?;
        desired_workspace.policy = Some(input.policy.clone());
        let desired = desired_workspace.canonical_bytes().map_err(map_policy)?;
        let result = policy_result(desired_workspace.revision, &input.policy, &unavailable)?;
        self.journal.begin(
            request,
            &format!("workspaces/{}/workspace.json", input.workspace_id),
            input.expected_revision,
            desired_workspace.revision,
            &desired,
            &result,
        )?;
        // The management lock covers only the publication: a management operation stages a
        // snapshot of this document under that lock and publishes it later under the same
        // lock, so a publication in between would leave the operation unable to complete (K1).
        // The expected-revision check is made again inside the publication; if an operation
        // moved the document while this call waited, the call fails as stale.
        let published = {
            let _management = self.lock_management()?;
            self.repository.publish_workspace_policy(
                &input.workspace_id,
                input.expected_revision,
                input.policy.clone(),
            )
        };
        let published = match published {
            Ok(value) => value,
            Err(error) => {
                self.journal.abort(request)?;
                return Err(map_policy(error));
            }
        };
        self.process_host
            .workspace_policy_published(&input.workspace_id, &previous)
            .map_err(internal_daemon)?;
        if published.revision != desired_workspace.revision {
            return Err(internal(
                "workspace policy publication revision disagrees with intent",
            ));
        }
        let result = self.journal.commit(request)?;
        mark_handoff(request)?;
        Ok(result)
    }
}

impl ClientAdminRoutes {
    fn folder_inventory(&self) -> Result<IJsonValue, ProductionRouteFailure> {
        use crate::endpoint_carrier::SupervisorSessionAuthority;
        let paths = self
            .management()?
            .folder_paths()
            .map_err(map_folder_management)?;
        let mut groups = std::collections::BTreeMap::<String, Vec<String>>::new();
        for path in paths.values() {
            groups.entry(path.clone()).or_default();
        }
        let mut archived = Vec::new();
        let sessions = self
            .process_host
            .inventory_items(true)
            .ok_or_else(|| internal("session inventory unavailable"))?
            .map_err(internal)?;
        for session in sessions {
            if session.archived {
                archived.push(session.session_id);
            } else if !session.ephemeral {
                groups
                    .entry(session.resolved_cwd(&paths))
                    .or_default()
                    .push(session.session_id);
            }
        }
        archived.sort();
        let items = groups
            .into_iter()
            .map(|(path, mut sessions)| {
                sessions.sort();
                json!({"path":path,"sessionIds":sessions})
            })
            .collect::<Vec<_>>();
        to_ijson(&json!({"items":items,"archivedSessionIds":archived}))
    }

    fn relocate_folder(
        &self,
        request: &EndpointHostCall,
        input: FolderRelocateRequest,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        let hash = management_request_hash(request)?;
        let management = self.management()?;
        let catalog = profile::FolderCatalog::new(&self.root);
        let logical = catalog
            .relocation(&request.rpc_id, &hash, &input.previous_path)
            .map_err(map_catalog_relocation)?;
        if logical.as_ref().is_some_and(|(_, done)| *done) {
            mark_catalog_handoff(request)?;
            return to_ijson(&json!({"relocated":true}));
        }
        let _folder_quiescence = logical
            .as_ref()
            .map(|(id, _)| {
                NamedLock::try_exclusive(crate::process_host::workspace_quiescence_lock_path(
                    &self.root,
                    &format!("folder:{id}"),
                ))
            })
            .transpose()
            .map_err(|error| match error {
                store::StoreError::Busy => failure(
                    "workspace-busy",
                    "Folder has a running session",
                    json!({"path":input.previous_path}),
                ),
                other => internal_store(other),
            })?;
        let workspace =
            match management.relocation_workspace(&request.rpc_id, &hash, &input.previous_path) {
                Ok(workspace) => workspace,
                Err(endpoint::ManagementError::InvalidPath(_)) if logical.is_some() => {
                    let canonical = Path::new(&input.path).canonicalize().map_err(internal_io)?;
                    if !canonical.is_dir()
                        || canonical.parent().is_none()
                        || tools::protected_root_containing(
                            &tools::kernel_protected_roots(&self.root),
                            &canonical,
                        )
                        .is_some()
                    {
                        return Err(failure(
                            "workspace-invalid-path",
                            "Workspace path is invalid",
                            json!({"path":input.path}),
                        ));
                    }
                    catalog
                        .relocate(
                            &request.rpc_id,
                            &hash,
                            &logical.expect("checked").0,
                            &input.previous_path,
                            &canonical.to_string_lossy(),
                        )
                        .map_err(map_catalog_relocation)?;
                    mark_catalog_handoff(request)?;
                    crate::endpoint_host::SessionDeliveryAuthority::workspace_metadata_changed(
                        self.process_host.as_ref(),
                        "",
                    );
                    return to_ijson(&json!({"relocated":true}));
                }
                Err(error) => return Err(map_folder_management(error)),
            };
        let _quiescence = NamedLock::try_exclusive(
            crate::process_host::workspace_quiescence_lock_path(&self.root, &workspace),
        )
        .map_err(|error| match error {
            store::StoreError::Busy => failure(
                "workspace-busy",
                "Workspace has a running session",
                json!({"workspaceId":workspace}),
            ),
            other => internal_store(other),
        })?;
        let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        management
            .relocate_workspace(
                &request.rpc_id,
                &hash,
                &workspace,
                &input.previous_path,
                &input.path,
                &timestamp,
            )
            .map_err(map_folder_management)?;
        mark_management_handoff(request)?;
        crate::endpoint_host::SessionDeliveryAuthority::workspace_metadata_changed(
            self.process_host.as_ref(),
            &workspace,
        );
        to_ijson(&json!({"relocated":true}))
    }

    fn list_folders(&self, input: FoldersGetRequest) -> Result<IJsonValue, ProductionRouteFailure> {
        let folders = self
            .management()?
            .workspace_folders(&input.workspace_id)
            .map_err(map_folder_management)?;
        to_ijson(&json!({
            "format":1,
            "workspaceId":folders.workspace_id,
            "revision":folders.revision,
            "folders":folders.folders,
        }))
    }

    /// Folder edits are management operations: the endpoint operation record
    /// is their durable carrier, so a retried rpc id resumes that record.
    /// The admin gate serializes them with `workspace.policy.set`, the other
    /// in-process writer of the same workspace document, and `set_policy`
    /// publishes under the management lock these operations hold throughout.
    fn change_folders(
        &self,
        request: &EndpointHostCall,
        input: FolderChangeRequest,
        add: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let _gate = self
            .mutation_gate
            .lock()
            .map_err(|_| internal("admin mutation lock poisoned"))?;
        self.recover_pending_intents()?;
        // Removal narrows every launch of the workspace, so like relocation
        // it requires a quiescent workspace. Workers hold this lock shared.
        let _quiescence = if add {
            None
        } else {
            Some(
                NamedLock::try_exclusive(crate::process_host::workspace_quiescence_lock_path(
                    &self.root,
                    &input.workspace_id,
                ))
                .map_err(|error| match error {
                    store::StoreError::Busy => failure(
                        "workspace-busy",
                        "Workspace has a running session",
                        json!({"workspaceId":input.workspace_id}),
                    ),
                    other => internal_store(other),
                })?,
            )
        };
        let hash = management_request_hash(request)?;
        let started_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let management = self.management()?;
        let (workspace, folders) = if add {
            management.add_workspace_folder(
                &request.rpc_id,
                &hash,
                &input.workspace_id,
                &input.path,
                &started_at,
            )
        } else {
            management.remove_workspace_folder(
                &request.rpc_id,
                &hash,
                &input.workspace_id,
                &input.path,
                &started_at,
            )
        }
        .map_err(map_folder_management)?;
        mark_management_handoff(request)?;
        to_ijson(&json!({
            "workspace":workspace,
            "folders":{
                "format":1,
                "workspaceId":folders.workspace_id,
                "revision":folders.revision,
                "folders":folders.folders,
            },
        }))
    }

    /// Management lock, then `config/.lock` inside the publication.
    fn lock_management(&self) -> Result<NamedLock, ProductionRouteFailure> {
        endpoint::ManagementStore::lock_exclusive(&self.root)
            .map_err(|error| internal(error.to_string()))
    }

    fn management(&self) -> Result<endpoint::ManagementStore, ProductionRouteFailure> {
        endpoint::ManagementStore::open_light(&self.root)
            .map_err(|error| internal(error.to_string()))
    }
}

struct AdminCapabilityRoutes {
    id: &'static str,
    methods: &'static [(&'static str, MethodClass)],
    authority: Arc<ClientAdminRoutes>,
}

impl AdminCapabilityRoutes {
    fn new(
        id: &'static str,
        methods: &'static [(&'static str, MethodClass)],
        authority: Arc<ClientAdminRoutes>,
    ) -> Self {
        Self {
            id,
            methods,
            authority,
        }
    }

    fn class(&self, method: &str) -> Option<MethodClass> {
        self.methods
            .iter()
            .find_map(|(name, class)| (*name == method).then_some(*class))
    }
}

impl ProductionEndpointRoutes for AdminCapabilityRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        self.methods
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        self.class(method)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        if self.class(operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Client administration capability is unavailable",
                json!({"capability":self.id,"operation":operation}),
            ));
        }
        self.authority
            .validate_extension_payload(operation, payload)
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        self.class(operation).is_some()
            && self
                .authority
                .extension_failure_is_exact(operation, failure)
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        if self.class(&request.operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Client administration capability is unavailable",
                json!({"capability":self.id,"operation":request.operation}),
            ));
        }
        self.authority.execute(request, payload, principal)
    }
}

impl ProductionEndpointRoutes for ClientAdminRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        WORKSPACE_POLICY_METHODS
            .into_iter()
            .chain(WORKSPACE_FOLDER_METHODS)
            .chain(ENVIRONMENT_METHODS)
            .chain(FOLDER_METHODS)
            .chain(SESSION_FOLDER_METHODS)
            .map(|(name, _)| name.to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        WORKSPACE_POLICY_METHODS
            .into_iter()
            .chain(WORKSPACE_FOLDER_METHODS)
            .chain(ENVIRONMENT_METHODS)
            .chain(FOLDER_METHODS)
            .chain(SESSION_FOLDER_METHODS)
            .find_map(|(name, class)| (name == method).then_some(class))
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match operation {
            "session.listFolders" => {
                parse::<session_folders::SessionFoldersGet>(payload).map(|_| ())
            }
            "session.addFolder" | "session.removeFolder" => {
                parse::<session_folders::SessionFolderChange>(payload).map(|_| ())
            }
            "folders.list" => parse::<EnvironmentsListRequest>(payload).map(|_| ()),
            "folders.relocate" => parse::<FolderRelocateRequest>(payload).map(|_| ()),
            "environments.list" => parse::<EnvironmentsListRequest>(payload).map(|_| ()),
            "environments.save" => parse_environment_save(payload).map(|_| ()),
            "workspace.policy.get" => parse::<PolicyGetRequest>(payload).map(|_| ()),
            "workspace.policy.set" => parse::<PolicySetRequest>(payload).map(|_| ()),
            "workspace.listFolders" => parse::<FoldersGetRequest>(payload).map(|_| ()),
            "workspace.addFolder" | "workspace.removeFolder" => {
                parse::<FolderChangeRequest>(payload).map(|_| ())
            }
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":operation}),
            )),
        }
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        if operation.starts_with("environments.") {
            if matches!(
                failure.code.as_str(),
                "environment-invalid" | "stale-revision"
            ) {
                let Ok(Value::Object(details)) = serde_json::to_value(&failure.details) else {
                    return false;
                };
                return operation == "environments.save"
                    && match failure.code.as_str() {
                        "environment-invalid" => {
                            failure.message == "Environment definition is invalid"
                                && details.len() == 1
                                && details.get("reason").is_some_and(Value::is_string)
                        }
                        "stale-revision" => {
                            failure.message == "Configuration revision is stale"
                                && details.len() == 2
                                && ["expected", "actual"]
                                    .iter()
                                    .all(|key| details.get(*key).is_some_and(Value::is_u64))
                        }
                        _ => false,
                    };
            }
            return matches!(
                failure.code.as_str(),
                "bad-request" | "unsupported-capability" | "internal"
            ) || (operation == "environments.save"
                && matches!(failure.code.as_str(), "idempotency-conflict"));
        }
        matches!(
            failure.code.as_str(),
            "bad-request"
                | "unsupported-capability"
                | "internal"
                | "idempotency-conflict"
                | "stale-revision"
                | "workspace-not-found"
                | "policy-invalid"
                | "policy-escalation"
                | "workspace-invalid-path"
                | "workspace-ambiguous"
                | "workspace-busy"
                | "workspace-last-folder"
                | "workspace-folder-in-use"
                | "workspace-legacy-session"
        )
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match request.operation.as_str() {
            "session.listFolders" => self.session_folders(parse(payload)?),
            "session.addFolder" => {
                self.change_session_folder(request, parse(payload)?, true, principal)
            }
            "session.removeFolder" => {
                self.change_session_folder(request, parse(payload)?, false, principal)
            }
            "folders.list" => {
                let _: EnvironmentsListRequest = parse(payload)?;
                self.folder_inventory()
            }
            "folders.relocate" => self.relocate_folder(request, parse(payload)?),
            "environments.list" => {
                let _: EnvironmentsListRequest = parse(payload)?;
                self.list_environments()
            }
            "environments.save" => self.save_environment(request, parse_environment_save(payload)?),
            "workspace.policy.get" => self.get_policy(parse(payload)?),
            "workspace.policy.set" => self.set_policy(request, parse(payload)?),
            "workspace.listFolders" => self.list_folders(parse(payload)?),
            "workspace.addFolder" => self.change_folders(request, parse(payload)?, true),
            "workspace.removeFolder" => self.change_folders(request, parse(payload)?, false),
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":request.operation}),
            )),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentsListRequest {}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EnvironmentView {
    id: String,
    network: bool,
    toolchain_roots: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_wall_seconds: Option<u64>,
}

impl From<profile::Environment> for EnvironmentView {
    fn from(value: profile::Environment) -> Self {
        Self {
            id: value.id,
            network: value.network,
            toolchain_roots: value.toolchain_roots,
            max_wall_seconds: value.max_wall_seconds,
        }
    }
}
impl From<EnvironmentView> for profile::Environment {
    fn from(value: EnvironmentView) -> Self {
        Self {
            id: value.id,
            network: value.network,
            toolchain_roots: value.toolchain_roots,
            max_wall_seconds: value.max_wall_seconds,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EnvironmentSaveRequest {
    expected_revision: u64,
    environment: EnvironmentView,
}

fn parse_environment_save(
    payload: &Value,
) -> Result<EnvironmentSaveRequest, ProductionRouteFailure> {
    if payload
        .get("environment")
        .and_then(|value| value.get("maxWallSeconds"))
        .is_some_and(Value::is_null)
    {
        return Err(failure(
            "bad-request",
            "Invalid client administration request",
            json!({"reason":"maxWallSeconds must be absent or a positive integer"}),
        ));
    }
    parse(payload)
}

fn map_environment(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::InvalidSchema { .. }
        | profile::ProfileError::InvalidReference { .. } => failure(
            "environment-invalid",
            "Environment definition is invalid",
            json!({"reason":error.to_string()}),
        ),
        other => internal_profile(other),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicyGetRequest {
    workspace_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FoldersGetRequest {
    workspace_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FolderRelocateRequest {
    previous_path: String,
    path: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FolderChangeRequest {
    workspace_id: String,
    path: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicySetRequest {
    workspace_id: String,
    expected_revision: u64,
    policy: WorkspacePolicy,
}

fn parse<T: for<'de> Deserialize<'de>>(payload: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(payload.clone()).map_err(|error| {
        failure(
            "bad-request",
            "Invalid client administration request",
            json!({"reason":error.to_string()}),
        )
    })
}
fn to_ijson(value: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(value).map_err(|error| internal(error.to_string()))?,
    )
    .map_err(|error| internal(error.to_string()))
}
fn failure(code: &str, message: &str, details: Value) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        code,
        message,
        to_ijson(&details).unwrap_or_else(|_| IJsonValue::parse_str("{}").expect("I-JSON")),
    )
}
fn internal(message: impl ToString) -> ProductionRouteFailure {
    failure(
        "internal",
        "Client administration failed",
        json!({"reason":message.to_string()}),
    )
}
fn internal_profile(error: profile::ProfileError) -> ProductionRouteFailure {
    internal(error)
}
fn internal_daemon(error: crate::host_runtime::DaemonError) -> ProductionRouteFailure {
    internal(error)
}
fn stale(expected: u64, actual: u64) -> ProductionRouteFailure {
    failure(
        "stale-revision",
        "Configuration revision is stale",
        json!({"expected":expected,"actual":actual}),
    )
}
fn next_revision(value: u64) -> Result<u64, ProductionRouteFailure> {
    value
        .checked_add(1)
        .filter(|value| *value <= 9_007_199_254_740_991)
        .ok_or_else(|| {
            failure(
                "bad-request",
                "Configuration revision is exhausted",
                json!({}),
            )
        })
}
fn map_workspace(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::Io(ref io) if io.kind() == std::io::ErrorKind::NotFound => {
            failure("workspace-not-found", "Workspace is absent", json!({}))
        }
        _ => internal_profile(error),
    }
}
fn map_policy(error: profile::ProfileError) -> ProductionRouteFailure {
    match error {
        profile::ProfileError::StaleRevision { expected, actual } => stale(expected, actual),
        profile::ProfileError::Io(ref io) if io.kind() == std::io::ErrorKind::NotFound => {
            failure("workspace-not-found", "Workspace is absent", json!({}))
        }
        profile::ProfileError::InvalidReference { .. } => failure(
            "policy-escalation",
            "Workspace policy exceeds an authority ceiling",
            json!({"reason":error.to_string()}),
        ),
        _ => failure(
            "policy-invalid",
            "Workspace policy is invalid",
            json!({"reason":error.to_string()}),
        ),
    }
}
/// Maps a refused candidate policy to its endpoint failure.
///
/// The documented codes stay. `policy-escalation` is the documented code for
/// every way a policy can exceed what is installed or granted (the instruction
/// ceiling, a tool no catalog has, an MCP server that is not available), so
/// those keep it and differ in `message` and in `details.cause`. A candidate
/// that could not be judged because the catalogs could not be assembled is not
/// an escalation and is reported as `internal`. `details.reason` always
/// carries the precise sentence.
fn policy_candidate_failure(error: PolicyCandidateError) -> ProductionRouteFailure {
    let details = json!({"reason":error.to_string(),"cause":error.cause.code()});
    match error.cause {
        PolicyCandidateCause::AuthorityCeiling => failure(
            "policy-escalation",
            "Workspace policy exceeds an authority ceiling",
            details,
        ),
        PolicyCandidateCause::ToolNotInstalled => failure(
            "policy-escalation",
            "Workspace policy names a tool that is not installed",
            details,
        ),
        PolicyCandidateCause::McpServerUnavailable => failure(
            "policy-escalation",
            "Workspace policy requires an MCP server that is unavailable",
            details,
        ),
        PolicyCandidateCause::CatalogUnavailable => failure(
            "internal",
            "Workspace policy could not be checked against the installed catalogs",
            details,
        ),
    }
}

/// `{format,revision,policy}` plus, only when the policy names tools that an
/// otherwise healthy MCP server refused to project, `unavailableTools`: at most
/// [`MAX_REPORTED_TOOL_REFUSALS`] entries `{server,tool,name,code,reason,
/// pointer?,keyword?}`, with `unavailableToolCount` when more exist. Those
/// tools are accepted in the policy, absent from the catalog and not callable.
fn policy_result(
    revision: u64,
    policy: &WorkspacePolicy,
    unavailable: &[McpRefusedTool],
) -> Result<IJsonValue, ProductionRouteFailure> {
    let mut result = json!({"format":1,"revision":revision,"policy":policy});
    if !unavailable.is_empty() {
        let object = result.as_object_mut().expect("object literal");
        object.insert(
            "unavailableTools".to_owned(),
            Value::Array(
                unavailable
                    .iter()
                    .take(MAX_REPORTED_TOOL_REFUSALS)
                    .map(refused_tool_json)
                    .collect(),
            ),
        );
        if unavailable.len() > MAX_REPORTED_TOOL_REFUSALS {
            object.insert("unavailableToolCount".to_owned(), json!(unavailable.len()));
        }
    }
    to_ijson(&result)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AdminPhase {
    Prepared,
    Committed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdminRecord {
    format: u64,
    rpc_id: String,
    operation: String,
    request_sha256: String,
    authority: String,
    expected_revision: u64,
    next_revision: u64,
    desired_sha256: String,
    desired: Vec<u8>,
    phase: AdminPhase,
    result: IJsonValue,
}

enum AdminBegin {
    Execute,
    Completed,
}

/// The durable config-admin rpc carrier directory under the authority root.
const CONFIG_ADMIN_DIR: &str = "config-admin";
/// The name it carried before the rename; retired in place on first open.
const LEGACY_CONFIG_ADMIN_DIR: &str = "config-admin-v1";

struct AdminMutationJournal {
    authority_root: PathBuf,
    root: PathBuf,
    lock: PathBuf,
}

impl AdminMutationJournal {
    fn open(authority_root: &Path) -> Result<Self, store::StoreError> {
        let admin_root =
            store::retire_legacy_name(authority_root, CONFIG_ADMIN_DIR, LEGACY_CONFIG_ADMIN_DIR)?;
        let root = admin_root.join("rpc");
        fs::create_dir_all(&root)?;
        let lock = admin_root.join("lock");
        if !lock.exists() {
            AtomicPublisher::replace(&lock, b"config-admin\n")?;
        }
        Ok(Self {
            authority_root: authority_root.to_path_buf(),
            root,
            lock,
        })
    }

    fn begin(
        &self,
        request: &EndpointHostCall,
        authority: &str,
        expected_revision: u64,
        next_revision: u64,
        desired_bytes: &[u8],
        result: &IJsonValue,
    ) -> Result<AdminBegin, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let request_sha256 = request_digest(request)?;
        let path = self.record_path(&request.rpc_id);
        if let Some(mut old) = read_record(&path)? {
            if old.rpc_id != request.rpc_id
                || old.operation != request.operation
                || old.request_sha256 != request_sha256
            {
                return Err(failure(
                    "idempotency-conflict",
                    "rpcId was already used for another config mutation",
                    json!({"rpcId":request.rpc_id}),
                ));
            }
            if old.authority != authority
                || old.expected_revision != expected_revision
                || old.next_revision != next_revision
                || old.desired_sha256 != sha256(desired_bytes)
                || old.desired != desired_bytes
                || old.result != *result
            {
                return Err(failure(
                    "idempotency-conflict",
                    "rpcId config intent disagrees with its durable record",
                    json!({"rpcId":request.rpc_id}),
                ));
            }
            if old.phase == AdminPhase::Prepared
                && self.authority_digest(&old.authority)?.as_deref() == Some(&old.desired_sha256)
            {
                old.phase = AdminPhase::Committed;
                publish_record(&path, &old)?;
            }
            return Ok(if old.phase == AdminPhase::Committed {
                AdminBegin::Completed
            } else {
                AdminBegin::Execute
            });
        }
        let record = AdminRecord {
            format: 1,
            rpc_id: request.rpc_id.clone(),
            operation: request.operation.clone(),
            request_sha256,
            authority: authority.to_owned(),
            expected_revision,
            next_revision,
            desired_sha256: sha256(desired_bytes),
            desired: desired_bytes.to_vec(),
            phase: AdminPhase::Prepared,
            result: result.clone(),
        };
        publish_record(&path, &record)?;
        Ok(AdminBegin::Execute)
    }

    fn recover(
        &self,
        request: &EndpointHostCall,
    ) -> Result<Option<IJsonValue>, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let Some(mut record) = read_record(&path)? else {
            return Ok(None);
        };
        if record.rpc_id != request.rpc_id
            || record.operation != request.operation
            || record.request_sha256 != request_digest(request)?
        {
            return Err(failure(
                "idempotency-conflict",
                "rpcId was already used for another config mutation",
                json!({"rpcId":request.rpc_id}),
            ));
        }
        if record.phase == AdminPhase::Prepared
            && self.authority_digest(&record.authority)?.as_deref() == Some(&record.desired_sha256)
        {
            record.phase = AdminPhase::Committed;
            publish_record(&path, &record)?;
        }
        Ok((record.phase == AdminPhase::Committed).then_some(record.result))
    }

    fn commit(&self, request: &EndpointHostCall) -> Result<IJsonValue, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let mut record =
            read_record(&path)?.ok_or_else(|| internal("config mutation intent is missing"))?;
        if record.request_sha256 != request_digest(request)?
            || record.operation != request.operation
        {
            return Err(failure(
                "idempotency-conflict",
                "rpcId config intent disagrees with request",
                json!({"rpcId":request.rpc_id}),
            ));
        }
        if self.authority_digest(&record.authority)?.as_deref() != Some(&record.desired_sha256) {
            return Err(internal(
                "published config bytes disagree with durable intent",
            ));
        }
        record.phase = AdminPhase::Committed;
        publish_record(&path, &record)?;
        Ok(record.result)
    }

    fn records(&self) -> Result<Vec<AdminRecord>, ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let mut entries = fs::read_dir(&self.root)
            .map_err(internal_io)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(internal_io)?;
        entries.sort_by_key(fs::DirEntry::file_name);
        entries
            .into_iter()
            .map(|entry| {
                read_record(&entry.path())?.ok_or_else(|| internal("config journal entry vanished"))
            })
            .collect()
    }

    fn commit_record(&self, rpc_id: &str) -> Result<(), ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(rpc_id);
        let mut record =
            read_record(&path)?.ok_or_else(|| internal("config mutation intent is missing"))?;
        if self.authority_digest(&record.authority)?.as_deref() != Some(&record.desired_sha256) {
            return Err(internal(
                "published config bytes disagree with durable intent",
            ));
        }
        record.phase = AdminPhase::Committed;
        publish_record(&path, &record)
    }

    fn abort(&self, request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
        let _lock = NamedLock::exclusive(&self.lock).map_err(internal_store)?;
        let path = self.record_path(&request.rpc_id);
        let Some(record) = read_record(&path)? else {
            return Ok(());
        };
        if record.phase == AdminPhase::Committed
            || self.authority_digest(&record.authority)?.as_deref() == Some(&record.desired_sha256)
        {
            return Err(internal("cannot abort a published config mutation"));
        }
        fs::remove_file(&path).map_err(internal_io)?;
        fs::File::open(&self.root)
            .and_then(|directory| directory.sync_all())
            .map_err(internal_io)
    }

    fn authority_digest(&self, authority: &str) -> Result<Option<String>, ProductionRouteFailure> {
        if authority.starts_with('/')
            || authority
                .split('/')
                .any(|part| matches!(part, "" | "." | ".."))
        {
            return Err(internal("config mutation authority path is invalid"));
        }
        let path = self.authority_root.join(authority);
        match fs::read(path) {
            Ok(bytes) => Ok(Some(sha256(&bytes))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(internal_io(error)),
        }
    }

    fn record_path(&self, rpc_id: &str) -> PathBuf {
        self.root
            .join(format!("{}.json", sha256(rpc_id.as_bytes())))
    }
}

fn request_digest(request: &EndpointHostCall) -> Result<String, ProductionRouteFailure> {
    let bytes = serde_json_canonicalizer::to_vec(
        &json!({"method":request.operation,"payload":request.payload}),
    )
    .map_err(|error| internal(error.to_string()))?;
    Ok(sha256(&bytes))
}
fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read_record(path: &Path) -> Result<Option<AdminRecord>, ProductionRouteFailure> {
    match fs::read(path) {
        Ok(bytes) => {
            if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n') {
                return Err(internal("config mutation record framing is invalid"));
            }
            let value: AdminRecord =
                serde_json::from_slice(&bytes).map_err(|error| internal(error.to_string()))?;
            let mut canonical = serde_json_canonicalizer::to_vec(&value)
                .map_err(|error| internal(error.to_string()))?;
            canonical.push(b'\n');
            if canonical != bytes {
                return Err(internal("config mutation record is noncanonical"));
            }
            Ok(Some(value))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(internal_io(error)),
    }
}
fn publish_record(path: &Path, record: &AdminRecord) -> Result<(), ProductionRouteFailure> {
    let mut bytes =
        serde_json_canonicalizer::to_vec(record).map_err(|error| internal(error.to_string()))?;
    bytes.push(b'\n');
    AtomicPublisher::replace(path, &bytes).map_err(internal_store)
}
fn internal_store(error: store::StoreError) -> ProductionRouteFailure {
    internal(error)
}
fn internal_io(error: std::io::Error) -> ProductionRouteFailure {
    internal(error)
}
fn mark_handoff(request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: request.rpc_id.clone(),
            durable_identity: Some(RpcDurableIdentity {
                kind: CONFIG_ADMIN_DIR.to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|error| internal(error.to_string()))
}

fn management_request_hash(request: &EndpointHostCall) -> Result<String, ProductionRouteFailure> {
    let payload: Value = serde_json::from_slice(
        &request
            .payload
            .canonical_bytes()
            .map_err(|error| internal(error.to_string()))?,
    )
    .map_err(|error| internal(error.to_string()))?;
    let bytes = serde_json_canonicalizer::to_vec(&json!({
        "type":"client-request",
        "rpcId":request.rpc_id,
        "method":request.operation,
        "payload":payload,
    }))
    .map_err(|error| internal(error.to_string()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

/// Same durable identity the base workspace mutations report: the endpoint
/// management operation record keyed by rpc id.
fn mark_management_handoff(request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: "management".to_owned(),
            durable_identity: Some(RpcDurableIdentity {
                kind: "management-operation".to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|error| internal(error.to_string()))
}

fn map_catalog_relocation(error: profile::ProfileError) -> ProductionRouteFailure {
    if let profile::ProfileError::InvalidReference { reason, .. } = &error
        && reason == "folder relocation idempotency conflict"
    {
        return failure(
            "idempotency-conflict",
            "RPC id was reused with different parameters",
            json!({}),
        );
    }
    internal_profile(error)
}

fn mark_catalog_handoff(request: &EndpointHostCall) -> Result<(), ProductionRouteFailure> {
    request
        .handoff
        .mark_handed_off(DurableHandoffProof {
            delivery: request.rpc_id.clone(),
            durable_identity: Some(RpcDurableIdentity {
                kind: "folder-relocation".to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|error| internal(error.to_string()))
}

fn map_folder_management(error: endpoint::ManagementError) -> ProductionRouteFailure {
    use endpoint::ManagementError as E;
    match error {
        E::WorkspaceNotFound(workspace_id) => failure(
            "workspace-not-found",
            "Workspace was not found",
            json!({"workspaceId":workspace_id}),
        ),
        E::InvalidPath(path) => failure(
            "workspace-invalid-path",
            "Workspace path is invalid",
            json!({"path":path}),
        ),
        E::WorkspaceAmbiguous(path) => failure(
            "workspace-ambiguous",
            "Path is already a workspace folder",
            json!({"path":path}),
        ),
        E::LastFolder(workspace_id) => failure(
            "workspace-last-folder",
            "A workspace keeps at least one folder",
            json!({"workspaceId":workspace_id}),
        ),
        E::FolderInUse {
            workspace_id,
            path,
            session_ids,
        } => failure(
            "workspace-folder-in-use",
            "Sessions are bound to this folder",
            json!({"workspaceId":workspace_id,"path":path,"sessionIds":session_ids}),
        ),
        E::LegacySessions {
            workspace_id,
            session_ids,
        } => failure(
            "workspace-legacy-session",
            "Workspace has sessions without a stable folder binding",
            json!({"workspaceId":workspace_id,"sessionIds":session_ids}),
        ),
        E::IdempotencyConflict { rpc_id, operation } => failure(
            "idempotency-conflict",
            "rpcId was already used for another request",
            json!({"rpcId":rpc_id,"operation":operation}),
        ),
        other => internal(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use endpoint::DurableHandoffSignal;
    use profile::WorkspaceConfig;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, Arc<ProductionProcessHost>, ClientAdminRoutes) {
        let root = TempDir::new().expect("temp root");
        let agent = root.path().join("agent");
        fs::create_dir_all(&agent).expect("agent root");
        let host = ProductionProcessHost::open(root.path(), "/usr/bin/true", "test-build", &agent)
            .expect("process host");
        let repository = ConfigRepository::open(root.path()).expect("config repository");
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-1".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: vec![root.path().to_string_lossy().into_owned()],
                    folders: Vec::new(),
                    policy: Some(WorkspacePolicy::default()),
                },
            )
            .expect("workspace");
        let routes = ClientAdminRoutes::new(root.path(), Arc::clone(&host)).expect("routes");
        (root, host, routes)
    }

    fn call(rpc_id: &str, operation: &str, payload: Value, recovering: bool) -> EndpointHostCall {
        EndpointHostCall {
            rpc_id: rpc_id.to_owned(),
            operation: operation.to_owned(),
            payload: to_ijson(&payload).expect("payload"),
            recovering,
            handoff: DurableHandoffSignal::new(),
        }
    }

    #[test]
    fn folder_inventory_groups_session_cwds_and_relocation_preserves_genesis_and_trust_boundary() {
        use crate::endpoint_carrier::SupervisorSessionAuthority;
        let (root, host, routes) = fixture();
        let projects = TempDir::new().unwrap();
        let first = projects.path().join("first");
        let second = projects.path().join("second");
        fs::create_dir(&first).unwrap();
        fs::create_dir(&second).unwrap();
        let first = fs::canonicalize(first)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let second = fs::canonicalize(second)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        routes
            .repository
            .publish_workspace(
                1,
                &WorkspaceConfig {
                    format: 1,
                    revision: 2,
                    id: "workspace-1".into(),
                    name: "Workspace".into(),
                    cwd: Vec::new(),
                    folders: vec![
                        profile::WorkspaceFolder {
                            id: "first".into(),
                            path: first.clone(),
                        },
                        profile::WorkspaceFolder {
                            id: "second".into(),
                            path: second.clone(),
                        },
                    ],
                    policy: Some(WorkspacePolicy::default()),
                },
            )
            .unwrap();
        let management = routes.management().unwrap();
        let mut ids = Vec::new();
        for (index, cwd) in [&first, &second].into_iter().enumerate() {
            let id = management
                .create_session(endpoint::SessionCreateOperation {
                    rpc_id: &format!("create-folder-{index}"),
                    request_sha256: &format!("{index}").repeat(64),
                    requested_session_id: None,
                    workspace_id: Some("workspace-1"),
                    cwd: Some(cwd),
                    environment: None,
                    identity_profile: None,
                    preset: None,
                    preset_mcp: None,
                    preset_models: None,
                    user_agent_dir: &root.path().join("agent"),
                    started_at: "2026-10-15T00:00:00.000Z",
                    principal: "uid:1",
                })
                .unwrap();
            host.reconcile_projection(&id).unwrap();
            ids.push(id);
        }
        let native = endpoint::NativeEndpoint::open(root.path()).unwrap();
        let scanned = native.list_sessions(&Default::default()).unwrap();
        assert_eq!(
            scanned,
            native.list_sessions_cached(&Default::default()).unwrap()
        );
        assert_eq!(
            scanned.iter().find(|s| s.session_id == ids[1]).unwrap().cwd,
            second
        );
        let groups = serde_json::to_value(routes.folder_inventory().unwrap()).unwrap();
        let members = |value: &Value, path: &str| {
            value["items"]
                .as_array()
                .unwrap()
                .iter()
                .find(|group| group["path"] == path)
                .unwrap()["sessionIds"]
                .clone()
        };
        assert_eq!(members(&groups, &first), json!([ids[0]]));
        assert_eq!(members(&groups, &second), json!([ids[1]]));
        let ledger_path = root.path().join("threads").join(&ids[1]).join("main.jsonl");
        let ledger = fs::read(&ledger_path).unwrap();
        let genesis = store::read_genesis(&ledger_path).unwrap().unwrap();
        let mut legacy = serde_json::to_value(genesis.raw()).unwrap();
        legacy.as_object_mut().unwrap().remove("cwd");
        let legacy = schema::Event::decode(&serde_json::to_vec(&legacy).unwrap()).unwrap();
        assert_eq!(
            endpoint::genesis_cwd(ledger_path.parent().unwrap(), &legacy),
            second
        );
        let missing_assets = TempDir::new().unwrap();
        assert_eq!(endpoint::genesis_cwd(missing_assets.path(), &legacy), "");
        assert!(!missing_assets.path().join("assets").exists());
        let digest = serde_json::to_value(legacy.raw()).unwrap()["config"]["digest"]
            .as_str()
            .unwrap()
            .to_owned();
        fs::create_dir(missing_assets.path().join("assets")).unwrap();
        fs::write(
            missing_assets
                .path()
                .join("assets")
                .join(format!("sha256-{digest}")),
            b"{}",
        )
        .unwrap();
        assert_eq!(endpoint::genesis_cwd(missing_assets.path(), &legacy), "");
        let trust = profile::FolderAuthorityStore::new(root.path());
        trust
            .request("trust-request", 0, Path::new(&second))
            .unwrap();
        let granted = trust
            .grant_from_host("trust-grant", 1, "trust-request")
            .unwrap();
        assert!(granted.folder.trusted);
        let moved = projects.path().join("moved");
        fs::rename(&second, &moved).unwrap();
        let moved = fs::canonicalize(moved)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let payload = json!({"previousPath":second,"path":moved});
        let request = call("move-second", "folders.relocate", payload.clone(), false);
        let busy = NamedLock::shared(crate::process_host::workspace_quiescence_lock_path(
            root.path(),
            "workspace-1",
        ))
        .unwrap();
        assert_eq!(
            routes
                .execute(&request, &payload, "uid:1")
                .unwrap_err()
                .code,
            "workspace-busy"
        );
        drop(busy);
        let result = routes.execute(&request, &payload, "uid:1").unwrap();
        assert!(request.handoff.is_durable());
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({"relocated":true})
        );
        assert_eq!(
            result,
            routes
                .execute(
                    &call("move-second", "folders.relocate", payload.clone(), true),
                    &payload,
                    "uid:1"
                )
                .unwrap()
        );
        let groups = serde_json::to_value(routes.folder_inventory().unwrap()).unwrap();
        assert_eq!(members(&groups, &first), json!([ids[0]]));
        assert_eq!(members(&groups, &moved), json!([ids[1]]));
        assert_eq!(fs::read(ledger_path).unwrap(), ledger);
        let current = trust.status(Path::new(&moved)).unwrap();
        assert_eq!(current.id, granted.folder.id);
        assert!(!current.trusted);
        let scope = profile::SessionScope::locate(root.path(), &ids[1]).unwrap();
        assert_eq!(
            scope.config(&routes.repository).unwrap().execution_cwd(),
            Some(moved.as_str())
        );
        let conflict = json!({"previousPath":second,"path":first});
        assert_eq!(
            routes
                .execute(
                    &call("move-second", "folders.relocate", conflict.clone(), true),
                    &conflict,
                    "uid:1"
                )
                .unwrap_err()
                .code,
            "idempotency-conflict"
        );
        host.shutdown();
    }

    #[test]
    fn folder_relocation_refuses_root_unknown_paths_and_ambiguous_owners() {
        let (root, host, routes) = fixture();
        let projects = TempDir::new().unwrap();
        let path = fs::canonicalize(projects.path())
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let mut config = routes.repository.workspace("workspace-1").unwrap();
        config.cwd = vec![path.clone()];
        config.revision += 1;
        routes.repository.publish_workspace(1, &config).unwrap();
        for (rpc, previous, target, error) in [
            ("root", path.as_str(), "/", "workspace-invalid-path"),
            (
                "unknown",
                "/missing",
                path.as_str(),
                "workspace-invalid-path",
            ),
        ] {
            let payload = json!({"previousPath":previous,"path":target});
            assert_eq!(
                routes
                    .execute(
                        &call(rpc, "folders.relocate", payload.clone(), false),
                        &payload,
                        "uid:1"
                    )
                    .unwrap_err()
                    .code,
                error
            );
        }
        config.id = "workspace-2".into();
        config.revision = 1;
        routes.repository.publish_workspace(0, &config).unwrap();
        let payload = json!({"previousPath":path,"path":root.path()});
        assert_eq!(
            routes
                .execute(
                    &call("ambiguous", "folders.relocate", payload.clone(), false),
                    &payload,
                    "uid:1"
                )
                .unwrap_err()
                .code,
            "workspace-ambiguous"
        );
        for payload in [
            json!({"path":path}),
            json!({"previousPath":path,"path":"/","trusted":true}),
        ] {
            assert!(
                routes
                    .validate_extension_payload("folders.relocate", &payload)
                    .is_err()
            );
        }
        assert!(
            routes
                .validate_extension_payload("folders.list", &json!({"workspaceId":"workspace-1"}))
                .is_err()
        );
        host.shutdown();
    }

    #[test]
    fn environment_reads_are_pure_and_invalid_saves_publish_nothing() {
        let (root, _host, routes) = fixture();
        let value = serde_json::to_value(routes.list_environments().unwrap()).unwrap();
        assert_eq!(
            value,
            json!({"format":1,"revision":0,"defaultEnvironment":"local",
            "environments":[{"id":"local","network":true,"toolchainRoots":[]}]})
        );
        let path = root.path().join("config/environments.json");
        assert!(!path.exists());
        for environment in [
            json!({"id":"","network":false,"toolchainRoots":[]}),
            json!({"id":"offline","network":false,"toolchainRoots":["relative/bin"]}),
            json!({"id":"offline","network":false,"toolchainRoots":[],"maxWallSeconds":0}),
            json!({"id":"e\u{301}","network":false,"toolchainRoots":[]}),
        ] {
            let payload = json!({"expectedRevision":0,"environment":environment});
            let request = call(
                "environment-invalid",
                "environments.save",
                payload.clone(),
                false,
            );
            let error = routes.execute(&request, &payload, "uid:1").unwrap_err();
            assert_eq!(error.code, "environment-invalid");
            assert!(!request.handoff.is_durable());
            assert!(!path.exists());
            assert!(routes.journal.records().unwrap().is_empty());
        }
        for environment in [
            json!({"id":"offline","network":false,"toolchainRoots":[],"projectTrusted":true}),
            json!({"id":"offline","network":false,"toolchainRoots":[],"writableRoots":["/"]}),
            json!({"id":"offline","network":false,"toolchainRoots":[],"maxWallSeconds":null}),
        ] {
            let payload = json!({"expectedRevision":0,"environment":environment});
            let request = call(
                "environment-malformed",
                "environments.save",
                payload.clone(),
                false,
            );
            assert_eq!(
                routes
                    .execute(&request, &payload, "uid:1")
                    .unwrap_err()
                    .code,
                "bad-request"
            );
            assert!(!path.exists());
            assert!(routes.journal.records().unwrap().is_empty());
        }
    }

    #[test]
    fn environment_save_has_cas_and_exact_retries_after_later_edits() {
        let (_root, _host, routes) = fixture();
        let payload = json!({"expectedRevision":0,"environment":{"id":"offline","network":false,"toolchainRoots":[],"maxWallSeconds":60}});
        let request = call("env-first", "environments.save", payload.clone(), false);
        let first = routes.execute(&request, &payload, "uid:1").unwrap();
        assert!(request.handoff.is_durable());
        let changed = json!({"expectedRevision":1,"environment":{"id":"local","network":false,"toolchainRoots":[]}});
        let second = call("env-second", "environments.save", changed.clone(), false);
        routes.execute(&second, &changed, "uid:1").unwrap();
        assert_eq!(routes.execute(&request, &payload, "uid:1").unwrap(), first);
        assert_eq!(routes.repository.environments().unwrap().revision, 2);
        assert!(
            routes
                .repository
                .environments()
                .unwrap()
                .environment("offline")
                .is_some()
        );
        let stale_request = call("env-stale", "environments.save", payload.clone(), false);
        assert_eq!(
            routes
                .execute(&stale_request, &payload, "uid:1")
                .unwrap_err()
                .code,
            "stale-revision"
        );
        let conflict = call("env-first", "environments.save", changed.clone(), false);
        assert_eq!(
            routes
                .execute(&conflict, &changed, "uid:1")
                .unwrap_err()
                .code,
            "idempotency-conflict"
        );
        assert_eq!(routes.repository.environments().unwrap().revision, 2);
    }

    #[test]
    fn environment_intents_recover_before_and_after_config_publication() {
        for published in [false, true] {
            let (root, host, routes) = fixture();
            let environment = profile::Environment {
                id: "offline".into(),
                network: false,
                toolchain_roots: Vec::new(),
                max_wall_seconds: Some(60),
            };
            let desired = profile::EnvironmentsConfig {
                format: 1,
                revision: 1,
                environments: vec![environment.clone()],
            };
            let payload = json!({"expectedRevision":0,"environment":EnvironmentView::from(environment.clone())});
            let request = call("env-crash", "environments.save", payload, false);
            let expected = to_ijson(
                &json!({"format":1,"revision":1,"environment":EnvironmentView::from(environment)}),
            )
            .unwrap();
            routes
                .journal
                .begin(
                    &request,
                    "config/environments.json",
                    0,
                    1,
                    &desired.canonical_bytes().unwrap(),
                    &expected,
                )
                .unwrap();
            if published {
                routes.repository.publish_environments(0, &desired).unwrap();
            }
            drop(routes);
            let recovered = ClientAdminRoutes::new(root.path(), host).unwrap();
            assert_eq!(
                fs::read(root.path().join("config/environments.json")).unwrap(),
                desired.canonical_bytes().unwrap()
            );
            let payload = serde_json::to_value(&request.payload).unwrap();
            assert_eq!(
                recovered.execute(&request, &payload, "uid:1").unwrap(),
                expected
            );
            assert!(request.handoff.is_durable());
            assert!(matches!(
                recovered.journal.records().unwrap()[0].phase,
                AdminPhase::Committed
            ));
            assert_eq!(recovered.repository.environments().unwrap().revision, 1);
        }
    }

    #[test]
    fn concurrent_environment_saves_have_one_cas_winner_and_no_loser_intent() {
        let (_root, _host, routes) = fixture();
        let routes = Arc::new(routes);
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let outcomes = std::thread::scope(|scope| {
            let tasks = ["first", "second"].map(|id| {
                let routes = Arc::clone(&routes);
                let barrier = Arc::clone(&barrier);
                scope.spawn(move || {
                    let payload = json!({"expectedRevision":0,"environment":{
                        "id":id,"network":false,"toolchainRoots":[]}});
                    let request = call(id, "environments.save", payload.clone(), false);
                    barrier.wait();
                    (id, routes.execute(&request, &payload, "uid:1"))
                })
            });
            barrier.wait();
            tasks.map(|task| task.join().unwrap())
        });
        let winners = outcomes
            .iter()
            .filter(|(_, outcome)| outcome.is_ok())
            .collect::<Vec<_>>();
        let losers = outcomes
            .iter()
            .filter(|(_, outcome)| outcome.is_err())
            .collect::<Vec<_>>();
        assert_eq!(winners.len(), 1);
        assert_eq!(losers.len(), 1);
        let error = losers[0].1.as_ref().unwrap_err();
        assert_eq!(error.code, "stale-revision");
        assert_eq!(
            serde_json::to_value(&error.details).unwrap(),
            json!({"expected":0,"actual":1})
        );
        let config = routes.repository.environments().unwrap();
        assert_eq!(config.revision, 1);
        assert_eq!(config.environments.len(), 1);
        assert_eq!(config.environments[0].id, winners[0].0);
        let records = routes.journal.records().unwrap();
        assert_eq!(records.len(), 1, "the CAS loser must not prepare an intent");
        assert_eq!(records[0].rpc_id, winners[0].0);
        assert!(matches!(records[0].phase, AdminPhase::Committed));
    }

    #[test]
    fn concurrent_named_environment_save_and_create_capture_one_complete_revision() {
        for iteration in 0..8 {
            let (root, _host, routes) = fixture();
            let project = tempfile::tempdir().unwrap();
            let project_path = project
                .path()
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let make_toolchain = |name: &str| {
                let path = project.path().join(name);
                fs::create_dir(&path).unwrap();
                path.canonicalize().unwrap().to_string_lossy().into_owned()
            };
            let old_toolchain = make_toolchain("old-toolchain");
            let new_toolchain = make_toolchain("new-toolchain");
            let mut workspace = routes.repository.workspace("workspace-1").unwrap();
            workspace.revision = 2;
            workspace.cwd = vec![project_path.clone()];
            workspace.policy = Some(WorkspacePolicy {
                network: true,
                writable_roots: vec![project_path],
                max_wall_seconds: Some(600),
                ..WorkspacePolicy::default()
            });
            routes.repository.publish_workspace(1, &workspace).unwrap();
            routes
                .repository
                .publish_environments(
                    0,
                    &profile::EnvironmentsConfig {
                        format: 1,
                        revision: 1,
                        environments: vec![profile::Environment {
                            id: "named".into(),
                            network: false,
                            toolchain_roots: vec![old_toolchain.clone()],
                            max_wall_seconds: Some(60),
                        }],
                    },
                )
                .unwrap();
            let management = endpoint::ManagementStore::open_light(root.path()).unwrap();
            let barrier = Arc::new(std::sync::Barrier::new(3));
            let payload = json!({"expectedRevision":1,"environment":{
                "id":"named","network":true,"toolchainRoots":[new_toolchain.clone()],"maxWallSeconds":120}});
            let request = call(
                &format!("update-race-{iteration}"),
                "environments.save",
                payload.clone(),
                false,
            );
            let create_rpc = format!("create-race-{iteration}");
            let agents = root.path().join("agent");
            let (saved, created) = std::thread::scope(|scope| {
                let save_gate = Arc::clone(&barrier);
                let create_gate = Arc::clone(&barrier);
                let routes_ref = &routes;
                let request_ref = &request;
                let payload_ref = &payload;
                let saved = scope.spawn(move || {
                    save_gate.wait();
                    routes_ref.execute(request_ref, payload_ref, "uid:1")
                });
                let management_ref = &management;
                let create_rpc_ref = &create_rpc;
                let agents_ref = &agents;
                let created = scope.spawn(move || {
                    create_gate.wait();
                    management_ref.create_session(endpoint::SessionCreateOperation {
                        rpc_id: create_rpc_ref,
                        request_sha256: &"a".repeat(64),
                        requested_session_id: None,
                        workspace_id: Some("workspace-1"),
                        cwd: None,
                        environment: Some("named"),
                        identity_profile: None,
                        preset: None,
                        preset_mcp: None,
                        preset_models: None,
                        user_agent_dir: agents_ref,
                        started_at: "2026-10-15T00:00:00.000Z",
                        principal: "uid:1",
                    })
                });
                barrier.wait();
                (saved.join().unwrap(), created.join().unwrap())
            });
            saved.expect("racing save commits revision 2");
            let id = created.expect("create resolves either complete environment revision");
            let folder = root.path().join("threads").join(&id);
            let ledger_path = folder.join("main.jsonl");
            let ledger = fs::read(&ledger_path).unwrap();
            let genesis = store::read_genesis(&ledger_path).unwrap().unwrap();
            assert_eq!(genesis.string_field("environment"), Some("named"));
            let raw = serde_json::to_value(genesis.raw()).unwrap();
            let assets = store::AssetStore::new(folder.join("assets")).unwrap();
            let snapshot = profile::ConfigSnapshot::decode(
                &assets
                    .read_verified(&format!(
                        "sha256-{}",
                        raw["config"]["digest"].as_str().unwrap()
                    ))
                    .unwrap(),
            )
            .unwrap();
            let tuple = (
                snapshot.revisions.environments.unwrap(),
                snapshot.workspace.policy.network,
                snapshot.workspace.policy.max_wall_seconds,
                snapshot.workspace.policy.toolchain_roots.clone(),
            );
            assert!(
                tuple == (1, false, Some(60), vec![old_toolchain])
                    || tuple == (2, true, Some(120), vec![new_toolchain.clone()]),
                "mixed revision: {tuple:?}"
            );
            let hash = sha256(create_rpc.as_bytes());
            let operation: Value = serde_json::from_slice(
                &fs::read(
                    root.path()
                        .join("endpoint-management/operations")
                        .join(&hash[..2])
                        .join(format!("{hash}.json")),
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(operation["intent"]["environment"], "named");
            let session_scope = profile::SessionScope::locate(root.path(), &id).unwrap();
            let next = profile::LaunchProfile::resolve_and_publish_for_scope(
                &routes.repository,
                &session_scope,
                &agents,
                &assets,
            )
            .unwrap();
            assert_eq!(next.config.revisions.environments, Some(2));
            assert!(next.config.workspace.policy.network);
            assert_eq!(next.config.workspace.policy.max_wall_seconds, Some(120));
            assert_eq!(
                next.config.workspace.policy.toolchain_roots,
                vec![new_toolchain]
            );
            assert_eq!(fs::read(ledger_path).unwrap(), ledger);
        }
    }

    #[test]
    fn policy_replacement_can_relax_local_default_below_ceiling() {
        let (_root, _host, routes) = fixture();
        let request = call(
            "rpc-policy",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                "network":false,"allowed_tools":["read"],"writable_roots":[]
            }}),
            false,
        );
        let payload = serde_json::to_value(&request.payload).expect("payload value");
        let result = routes
            .execute(&request, &payload, "uid:1")
            .expect("policy set");
        let result = serde_json::to_value(result).expect("result value");
        assert_eq!(result["revision"], 2);
        assert_eq!(result["policy"]["allowed_tools"], json!(["read"]));
        assert!(request.handoff.is_durable());
    }

    #[test]
    fn journal_rejects_wrong_bytes_for_the_same_rpc_id() {
        let (_root, _host, routes) = fixture();
        let request = call(
            "rpc-conflict",
            "workspace.policy.set",
            json!({"a":1}),
            false,
        );
        let desired = b"{\"format\":1,\"revision\":2}\n";
        let result = to_ijson(&json!({"format":1})).expect("result");
        assert!(matches!(
            routes.journal.begin(
                &request,
                "workspaces/workspace-1/workspace.json",
                1,
                2,
                desired,
                &result
            ),
            Ok(AdminBegin::Execute)
        ));
        let changed = call("rpc-conflict", "workspace.policy.set", json!({"a":2}), true);
        let error = routes
            .journal
            .recover(&changed)
            .expect_err("wrong bytes reject");
        assert_eq!(error.code, "idempotency-conflict");
    }

    #[test]
    fn crash_before_publish_is_completed_before_an_unrelated_rpc() {
        let (_root, _host, routes) = fixture();
        let mut desired = routes
            .repository
            .workspace("workspace-1")
            .expect("workspace");
        desired.revision = 2;
        desired.policy = Some(WorkspacePolicy {
            allowed_tools: vec!["read".to_owned()],
            ..WorkspacePolicy::default()
        });
        let interrupted = call(
            "rpc-interrupted-policy",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                "network":false,"allowed_tools":["read"],"writable_roots":[]
            }}),
            false,
        );
        let result =
            policy_result(2, desired.policy.as_ref().expect("policy"), &[]).expect("result");
        assert!(matches!(
            routes.journal.begin(
                &interrupted,
                "workspaces/workspace-1/workspace.json",
                1,
                2,
                &desired.canonical_bytes().expect("desired"),
                &result,
            ),
            Ok(AdminBegin::Execute)
        ));

        let unrelated = call(
            "rpc-unrelated-policy",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":2,"policy":{
                "network":false,"allowed_tools":[],"writable_roots":[]
            }}),
            false,
        );
        let payload = serde_json::to_value(&unrelated.payload).expect("payload value");
        let completed = routes
            .execute(&unrelated, &payload, "uid:1")
            .expect("unrelated succeeds after recovery");
        assert_eq!(
            serde_json::to_value(completed).expect("result")["revision"],
            3
        );
        let record = routes
            .journal
            .records()
            .expect("records")
            .into_iter()
            .find(|record| record.rpc_id == "rpc-interrupted-policy")
            .expect("interrupted record");
        assert_eq!(record.phase, AdminPhase::Committed);
    }

    #[test]
    fn startup_closes_policy_crashes_on_both_sides_of_the_side_effect() {
        for side_effect_ran in [false, true] {
            let (root, host, routes) = fixture();
            let request = call(
                if side_effect_ran {
                    "rpc-policy-after-side-effect"
                } else {
                    "rpc-policy-before-side-effect"
                },
                "workspace.policy.set",
                json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{
                    "network":false,"allowed_tools":["read"],"writable_roots":[]
                }}),
                false,
            );
            let mut desired = routes
                .repository
                .workspace("workspace-1")
                .expect("workspace");
            let previous = routes.repository.resolve("workspace-1").expect("previous");
            desired.revision = 2;
            desired.policy = Some(WorkspacePolicy {
                allowed_tools: vec!["read".to_owned()],
                ..WorkspacePolicy::default()
            });
            let result =
                policy_result(2, desired.policy.as_ref().expect("policy"), &[]).expect("result");
            routes
                .journal
                .begin(
                    &request,
                    "workspaces/workspace-1/workspace.json",
                    1,
                    2,
                    &desired.canonical_bytes().expect("desired"),
                    &result,
                )
                .expect("prepare");
            routes
                .repository
                .publish_workspace_policy("workspace-1", 1, desired.policy.clone().expect("policy"))
                .expect("publish");
            if side_effect_ran {
                host.workspace_policy_published("workspace-1", &previous)
                    .expect("side effect");
            }
            drop(routes);
            let recovered =
                ClientAdminRoutes::new(root.path(), Arc::clone(&host)).expect("startup recovery");
            let record = recovered
                .journal
                .records()
                .expect("records")
                .into_iter()
                .find(|record| record.rpc_id == request.rpc_id)
                .expect("policy record");
            assert_eq!(record.phase, AdminPhase::Committed);
            assert_eq!(
                recovered
                    .repository
                    .workspace("workspace-1")
                    .expect("workspace"),
                desired
            );
        }
    }

    /// K1. A workspace operation stages a snapshot of the workspace document under the
    /// management lock and publishes it later under the same lock, so `workspace.policy.set`
    /// (and the policy intent replay at startup) must publish only while holding it.
    mod k1_management_race {
        use super::*;
        use std::sync::mpsc::{RecvTimeoutError, channel};
        use std::time::Duration;

        fn folder_fixture() -> (
            TempDir,
            Arc<ProductionProcessHost>,
            ClientAdminRoutes,
            PathBuf,
        ) {
            let (root, host, routes) = fixture();
            let project = root.path().join("project");
            fs::create_dir(&project).expect("project");
            let folder = project.to_string_lossy().into_owned();
            routes
                .repository
                .publish_workspace(
                    1,
                    &WorkspaceConfig {
                        format: 1,
                        revision: 2,
                        id: "workspace-1".to_owned(),
                        name: "Workspace".to_owned(),
                        cwd: Vec::new(),
                        folders: vec![profile::WorkspaceFolder {
                            id: "folder-0001".to_owned(),
                            path: folder,
                        }],
                        policy: Some(WorkspacePolicy::default()),
                    },
                )
                .expect("workspace with a folder");
            (root, host, routes, project)
        }

        fn policy_call(rpc_id: &str, expected_revision: u64, tool: &str) -> EndpointHostCall {
            call(
                rpc_id,
                "workspace.policy.set",
                json!({"workspaceId":"workspace-1","expectedRevision":expected_revision,
                    "policy":{"network":false,"allowed_tools":[tool],"writable_roots":[]}}),
                false,
            )
        }

        fn revision(routes: &ClientAdminRoutes) -> u64 {
            routes
                .repository
                .workspace("workspace-1")
                .expect("workspace")
                .revision
        }

        /// A policy set that arrives while a management operation holds the lock must not
        /// publish until the lock is released.
        #[test]
        fn policy_set_publishes_only_under_the_management_lock() {
            let (root, _host, routes, _project) = folder_fixture();
            let before = revision(&routes);
            let held = endpoint::ManagementStore::lock_exclusive(root.path()).expect("lock");
            let (sender, receiver) = channel();
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    let request = policy_call("rpc-locked-policy", before, "read");
                    let payload = serde_json::to_value(&request.payload).expect("payload");
                    let result = routes.execute(&request, &payload, "uid:1");
                    sender.send(result.is_ok()).expect("result");
                });
                assert!(
                    matches!(
                        receiver.recv_timeout(Duration::from_millis(500)),
                        Err(RecvTimeoutError::Timeout)
                    ),
                    "the policy set must wait for the management lock"
                );
                assert_eq!(revision(&routes), before, "nothing was published yet");
                drop(held);
                assert!(
                    receiver
                        .recv_timeout(Duration::from_secs(30))
                        .expect("the policy set completes once the lock is free")
                );
            });
            assert_eq!(revision(&routes), before + 1);
        }

        /// The startup replay of a journaled policy intent publishes under the same lock.
        #[test]
        fn recovered_policy_intent_publishes_only_under_the_management_lock() {
            let (root, _host, routes, _project) = folder_fixture();
            let before = revision(&routes);
            let request = policy_call("rpc-replayed-policy", before, "read");
            let mut desired = routes
                .repository
                .workspace("workspace-1")
                .expect("workspace");
            desired.revision = before + 1;
            desired.policy = Some(WorkspacePolicy {
                allowed_tools: vec!["read".to_owned()],
                ..WorkspacePolicy::default()
            });
            let result = policy_result(
                desired.revision,
                desired.policy.as_ref().expect("policy"),
                &[],
            )
            .expect("result");
            routes
                .journal
                .begin(
                    &request,
                    "workspaces/workspace-1/workspace.json",
                    before,
                    desired.revision,
                    &desired.canonical_bytes().expect("desired"),
                    &result,
                )
                .expect("prepare");
            let held = endpoint::ManagementStore::lock_exclusive(root.path()).expect("lock");
            let (sender, receiver) = channel();
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    sender
                        .send(routes.recover_pending_intents().is_ok())
                        .expect("result");
                });
                assert!(
                    matches!(
                        receiver.recv_timeout(Duration::from_millis(500)),
                        Err(RecvTimeoutError::Timeout)
                    ),
                    "the replay must wait for the management lock"
                );
                assert_eq!(revision(&routes), before, "nothing was published yet");
                drop(held);
                assert!(
                    receiver
                        .recv_timeout(Duration::from_secs(30))
                        .expect("the replay completes once the lock is free")
                );
            });
            assert_eq!(revision(&routes), before + 1);
        }

        /// The 30 s management recovery tick against a stream of `workspace.policy.set` calls.
        #[test]
        fn policy_set_racing_the_recovery_tick_never_corrupts_management() {
            let (root, host, routes, _project) = folder_fixture();
            let done = std::sync::atomic::AtomicBool::new(false);
            let (applied, tick_failures) = std::thread::scope(|scope| {
                let ticker = scope.spawn(|| {
                    let mut validated = endpoint::CompleteOperations::default();
                    let mut failures = 0_u32;
                    while !done.load(std::sync::atomic::Ordering::Acquire) {
                        if crate::endpoint_host::recover_pending_management(
                            root.path(),
                            &*host,
                            &mut validated,
                        )
                        .is_err()
                        {
                            failures += 1;
                        }
                    }
                    failures
                });
                let mut applied = 0_u32;
                for index in 0..120_u32 {
                    let tool = if index % 2 == 0 { "read" } else { "grep" };
                    let request =
                        policy_call(&format!("rpc-race-policy-{index}"), revision(&routes), tool);
                    let payload = serde_json::to_value(&request.payload).expect("payload");
                    if routes.execute(&request, &payload, "uid:1").is_ok() {
                        applied += 1;
                    }
                }
                done.store(true, std::sync::atomic::Ordering::Release);
                (applied, ticker.join().expect("ticker"))
            });
            assert_eq!(applied, 120, "only this thread writes the policy");
            assert_eq!(tick_failures, 0, "no recovery tick failed");
            endpoint::ManagementStore::open_at_with_queue_driver(
                root.path(),
                "2026-10-13T06:00:06.000Z",
                None,
            )
            .expect("boot");
        }

        /// `workspace.relocate` against a stream of `workspace.policy.set` calls. A policy set
        /// that loses to a relocate is stale; nothing may fail any other way.
        #[test]
        fn policy_set_racing_relocate_never_corrupts_management() {
            let (root, _host, routes, project) = folder_fixture();
            // Relocation targets are folder entries, which may not lie inside the storage root.
            let outside = tempfile::tempdir().expect("outside storage");
            let first_dir = outside.path().join("project-first");
            let other = outside.path().join("project-moved");
            fs::create_dir(&first_dir).expect("first");
            fs::create_dir(&other).expect("other");
            let first = fs::canonicalize(&first_dir)
                .expect("first")
                .to_string_lossy()
                .into_owned();
            let second = fs::canonicalize(&other)
                .expect("second")
                .to_string_lossy()
                .into_owned();
            let done = std::sync::atomic::AtomicBool::new(false);
            let (relocate_failures, other_policy_failures) = std::thread::scope(|scope| {
                let writer = scope.spawn(|| {
                    let mut index = 0_u32;
                    let mut other_failures = 0_u32;
                    while !done.load(std::sync::atomic::Ordering::Acquire) {
                        let tool = if index.is_multiple_of(2) {
                            "read"
                        } else {
                            "grep"
                        };
                        let request = policy_call(
                            &format!("rpc-relocate-race-policy-{index}"),
                            revision(&routes),
                            tool,
                        );
                        let payload = serde_json::to_value(&request.payload).expect("payload");
                        if let Err(error) = routes.execute(&request, &payload, "uid:1")
                            && error.code != "stale-revision"
                        {
                            other_failures += 1;
                        }
                        index += 1;
                    }
                    other_failures
                });
                let mut from = project.to_string_lossy().into_owned();
                let mut to = second.clone();
                let mut failures = 0_u32;
                for index in 0..80_u32 {
                    let store = endpoint::ManagementStore::open_light(root.path()).expect("light");
                    match store.relocate_workspace(
                        &format!("rpc-race-relocate-{index}"),
                        &"0".repeat(64),
                        "workspace-1",
                        &from,
                        &to,
                        "2026-10-13T05:40:00.000Z",
                    ) {
                        Ok(_) => {
                            let moved_to = to.clone();
                            to = if moved_to == second {
                                first.clone()
                            } else {
                                second.clone()
                            };
                            from = moved_to;
                        }
                        Err(_) => failures += 1,
                    }
                }
                done.store(true, std::sync::atomic::Ordering::Release);
                (failures, writer.join().expect("writer"))
            });
            assert_eq!(relocate_failures, 0, "no relocate failed");
            assert_eq!(
                other_policy_failures, 0,
                "policy sets only ever lose as stale"
            );
            endpoint::ManagementStore::open_at_with_queue_driver(
                root.path(),
                "2026-10-13T06:00:06.000Z",
                None,
            )
            .expect("boot");
        }
    }
}
