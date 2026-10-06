//! Production workspacePolicy.v1 endpoint routes. Provider configuration is
//! owned by the launching application.

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
use crate::process_host::ProductionProcessHost;

pub const WORKSPACE_POLICY_METHODS: [(&str, MethodClass); 2] = [
    ("workspace.policy.get", MethodClass::ReadOnly),
    ("workspace.policy.set", MethodClass::Mutation),
];

pub struct ClientAdminRoutes {
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
            repository: ConfigRepository::open(root).map_err(internal_profile)?,
            process_host,
            mutation_gate: Mutex::new(()),
            journal: AdminMutationJournal::open(root).map_err(internal_store)?,
        };
        routes.recover_pending_intents()?;
        Ok(routes)
    }

    /// Returns the workspace policy capability group.
    #[must_use]
    pub fn routes(self: &Arc<Self>) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
        vec![Arc::new(AdminCapabilityRoutes::new(
            "workspacePolicy.v1",
            &WORKSPACE_POLICY_METHODS,
            Arc::clone(self),
        ))]
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
                        let published = self
                            .repository
                            .publish_workspace_policy(&desired.id, record.expected_revision, policy)
                            .map_err(map_policy)?;
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
        self.process_host
            .validate_workspace_policy_candidate(&input.workspace_id, &input.policy)
            .map_err(|error| {
                failure(
                    "policy-escalation",
                    "Workspace policy exceeds an authority ceiling",
                    json!({"reason":error.to_string()}),
                )
            })?;
        let previous = self
            .repository
            .resolve(&input.workspace_id)
            .map_err(internal_profile)?;
        let mut desired_workspace = workspace.clone();
        desired_workspace.revision = next_revision(desired_workspace.revision)?;
        desired_workspace.policy = Some(input.policy.clone());
        let desired = desired_workspace.canonical_bytes().map_err(map_policy)?;
        let result = policy_result(desired_workspace.revision, &input.policy)?;
        self.journal.begin(
            request,
            &format!("workspaces/{}/workspace.json", input.workspace_id),
            input.expected_revision,
            desired_workspace.revision,
            &desired,
            &result,
        )?;
        let published = match self.repository.publish_workspace_policy(
            &input.workspace_id,
            input.expected_revision,
            input.policy.clone(),
        ) {
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
            .map(|(name, _)| name.to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        Some(match method {
            "workspace.policy.get" => MethodClass::ReadOnly,
            "workspace.policy.set" => MethodClass::Mutation,
            _ => return None,
        })
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match operation {
            "workspace.policy.get" => parse::<PolicyGetRequest>(payload).map(|_| ()),
            "workspace.policy.set" => parse::<PolicySetRequest>(payload).map(|_| ()),
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":operation}),
            )),
        }
    }

    fn extension_failure_is_exact(
        &self,
        _operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
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
        )
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match request.operation.as_str() {
            "workspace.policy.get" => self.get_policy(parse(payload)?),
            "workspace.policy.set" => self.set_policy(request, parse(payload)?),
            _ => Err(failure(
                "unsupported-capability",
                "Client administration method is unavailable",
                json!({"operation":request.operation}),
            )),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PolicyGetRequest {
    workspace_id: String,
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
fn policy_result(
    revision: u64,
    policy: &WorkspacePolicy,
) -> Result<IJsonValue, ProductionRouteFailure> {
    to_ijson(&json!({"format":1,"revision":revision,"policy":policy}))
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
        let result = policy_result(2, desired.policy.as_ref().expect("policy")).expect("result");
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
                policy_result(2, desired.policy.as_ref().expect("policy")).expect("result");
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
}
