use std::collections::HashSet;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

use endpoint::{
    ClientRequest, DurableHandoffProof, EndpointHost, EndpointHostCall, ManagementStore,
    NativeEndpoint, QueueTransactionOperation, QueueTransactionState, RpcDurableIdentity,
    SessionHostDescription,
};
use schema::{IJsonValue, OriginTuple, ResumePolicy};
use serde_json::Value;
use sha2::Digest;
use tekes_supervisor::endpoint_host::{
    EndpointAssemblyError, EndpointRouteClass, ProductionEndpointAssembly, ProductionEndpointHost,
    ProductionEndpointRoutes, ProductionRouteFailure, ProviderReadinessAuthority,
    QueueTransactionAuthority, RuntimeProviderFailure, RuntimeProviderReadiness,
    RuntimeProviderStatus, SessionDeliveryAuthority, TEKES_UNARY_ROUTES,
};

const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
const TIMESTAMP: &str = "2026-08-28T09:00:00.000Z";
const CONFIG_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn block_on_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("production host future unexpectedly pending"),
    }
}

#[test]
fn readiness_rejects_more_than_256_active_session_folders() {
    let root = tempfile::tempdir().expect("root");
    let threads = root.path().join("threads");
    std::fs::create_dir_all(&threads).unwrap();
    for index in 0..257_u16 {
        std::fs::create_dir(threads.join(format!("018f0000-0000-7000-8000-{index:012x}"))).unwrap();
    }
    let opened = ProductionEndpointAssembly::open(root.path(), description());
    assert!(matches!(
        opened,
        Err(EndpointAssemblyError::Management(
            endpoint::ManagementError::ActiveSessionLimit
        ))
    ));
}

struct LockedLedgerDelivery {
    endpoint: NativeEndpoint,
}

impl SessionDeliveryAuthority for LockedLedgerDelivery {
    fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &endpoint::MaterializedPrompt,
        steer: bool,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        let content = IJsonValue::parse(
            &serde_json::to_vec(&prompt.blocks).expect("materialized prompt JSON"),
        )
        .expect("materialized prompt I-JSON");
        self.endpoint
            .prompt_for_endpoint(session_id, timestamp, origin, content, steer)
            .map_err(|error| route_internal(error.to_string()))
    }

    fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        self.endpoint
            .cancel_for_endpoint(session_id, timestamp, origin)
            .map_err(|error| route_internal(error.to_string()))
    }

    fn rename(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        self.endpoint
            .rename_session_for_endpoint(session_id, timestamp, origin, title)
            .map_err(|error| route_internal(error.to_string()))
    }
}

fn route_internal(_diagnostic: String) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        "internal",
        "Endpoint operation failed",
        IJsonValue::parse_str("{}").expect("details"),
    )
}

struct QueueAccept;

impl QueueTransactionAuthority for QueueAccept {
    fn execute(
        &self,
        session_id: &str,
        transaction: &worker_control::QueueTransaction,
    ) -> Result<worker_control::QueueTransactionResult, ProductionRouteFailure> {
        assert_eq!(session_id, SESSION);
        assert_eq!(transaction.target_seq, 2);
        assert_eq!(transaction.retract_origin.op, "session.updateQueue");
        assert_eq!(transaction.retract_origin.key, "rpc-queue-edit/retract");
        let replacement = transaction
            .replacement_origin()
            .expect("replacement origin");
        assert_eq!(replacement.key, "rpc-queue-edit/replacement");
        Ok(worker_control::QueueTransactionResult {
            delivery: transaction.delivery.clone(),
            outcome: worker_control::QueueTransactionOutcome::Committed {
                first_seq: 3,
                last_seq: 4,
                deduplicated: false,
            },
        })
    }
}

struct CountingQueue {
    calls: AtomicUsize,
}

struct DeliveryFailure {
    code: &'static str,
    message: &'static str,
    details: &'static str,
}

impl SessionDeliveryAuthority for DeliveryFailure {
    fn prompt(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _prompt: &endpoint::MaterializedPrompt,
        _steer: bool,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        Err(ProductionRouteFailure::new(
            self.code,
            self.message,
            IJsonValue::parse_str(self.details).unwrap(),
        ))
    }

    fn cancel(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        panic!("not used")
    }

    fn rename(
        &self,
        _session_id: &str,
        _timestamp: &str,
        _origin: &OriginTuple,
        _title: &str,
    ) -> Result<endpoint::MutationReceipt, ProductionRouteFailure> {
        panic!("not used")
    }
}

impl QueueTransactionAuthority for CountingQueue {
    fn execute(
        &self,
        _session_id: &str,
        transaction: &worker_control::QueueTransaction,
    ) -> Result<worker_control::QueueTransactionResult, ProductionRouteFailure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(worker_control::QueueTransactionResult {
            delivery: transaction.delivery.clone(),
            outcome: worker_control::QueueTransactionOutcome::Committed {
                first_seq: 2,
                last_seq: 2,
                deduplicated: true,
            },
        })
    }
}

fn description() -> SessionHostDescription {
    SessionHostDescription {
        version: "0.1.0".to_owned(),
        cwd: "/workspace".to_owned(),
        provider: Some("deepseek".to_owned()),
        model: Some("deepseek-chat".to_owned()),
        attached_sessions: 1,
        home: "/Users/example".to_owned(),
        can_open_path: true,
    }
}

fn request(rpc_id: &str, method: &str, payload: &str) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: method.to_owned(),
        payload: IJsonValue::parse_str(payload).expect("request payload"),
    }
}

fn request_hash(request: &ClientRequest) -> String {
    let payload: Value =
        serde_json::from_slice(&request.payload.canonical_bytes().unwrap()).unwrap();
    let bytes = serde_json_canonicalizer::to_vec(&serde_json::json!({
        "type":"client-request",
        "rpcId":request.rpc_id,
        "method":request.method,
        "payload":payload,
    }))
    .unwrap();
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn bootstrap(root: &std::path::Path) {
    NativeEndpoint::open(root)
        .expect("native endpoint")
        .create_session(
            SESSION,
            "workspace-1",
            CONFIG_DIGEST,
            ResumePolicy::Never,
            TIMESTAMP,
            &OriginTuple {
                principal: "user".to_owned(),
                client: "test".to_owned(),
                target: SESSION.to_owned(),
                op: "create".to_owned(),
                key: "create-1".to_owned(),
            },
        )
        .expect("create session");
}

fn value(response: &endpoint::ServerResponse) -> Value {
    let value = response.result.value.as_ref().expect("success value");
    serde_json::from_slice(&value.canonical_bytes().expect("canonical value")).expect("JSON value")
}

#[test]
fn route_registry_is_frozen_and_never_advertises_placeholders() {
    assert_eq!(TEKES_UNARY_ROUTES.len(), 16);
    assert_eq!(
        TEKES_UNARY_ROUTES
            .iter()
            .map(|route| route.name)
            .collect::<HashSet<_>>()
            .len(),
        16
    );
    assert!(TEKES_UNARY_ROUTES.iter().any(|route| {
        route.name == "workspace.create"
            && route.class == EndpointRouteClass::Mutation
            && route.implemented
    }));
    assert!(!TEKES_UNARY_ROUTES.iter().any(|route| matches!(
        route.name,
        "host.describe"
            | "workspace.list"
            | "session.list"
            | "session.history"
            | "events.mux"
            | "events.host"
            | "respond"
    )));
    assert_eq!(
        ProductionEndpointHost::implemented_capabilities(),
        [
            "session.attachment",
            "session.create",
            "session.discard",
            "session.fork",
            "workspace.archiveSession",
            "workspace.create",
            "workspace.relocate",
            "workspace.rename",
            "workspace.unarchiveSession",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
}

#[test]
fn workspace_relocate_preserves_session_binding_and_rejects_a_stale_source_path() {
    let root = tempfile::tempdir().expect("root");
    let original = root.path().join("project-original");
    let relocated = root.path().join("project-relocated");
    std::fs::create_dir(&original).expect("project directory");
    let assembly = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
    )
    .expect("assembly");
    let created = block_on_ready(assembly.dispatch(request(
        "rpc-relocate-create-workspace",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(original.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("create workspace");
    let workspace_id = value(&created)["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id")
        .to_owned();
    let previous_path = value(&created)["workspace"]["path"]
        .as_str()
        .expect("canonical workspace path")
        .to_owned();
    block_on_ready(assembly.dispatch(request(
        "rpc-relocate-create-session",
        "session.create",
        &format!(r#"{{"sessionId":"{SESSION}","workspaceId":"{workspace_id}"}}"#),
    )))
    .expect("create session");
    let ledger =
        std::fs::read_to_string(root.path().join("threads").join(SESSION).join("main.jsonl"))
            .expect("session ledger");
    let genesis: Value = serde_json::from_str(ledger.lines().next().unwrap()).expect("genesis");
    assert_eq!(genesis["identity_profile"], "auto");

    std::fs::rename(&original, &relocated).expect("move project");
    let relocate_payload = format!(
        r#"{{"path":{},"previousPath":{},"workspaceId":"{workspace_id}"}}"#,
        serde_json::to_string(relocated.to_str().unwrap()).unwrap(),
        serde_json::to_string(&previous_path).unwrap(),
    );
    let response = block_on_ready(assembly.dispatch(request(
        "rpc-relocate-workspace",
        "workspace.relocate",
        &relocate_payload,
    )))
    .expect("relocate workspace");
    assert!(response.result.ok, "{response:?}");
    let canonical_relocated = relocated.canonicalize().expect("canonical relocated path");
    assert_eq!(
        value(&response)["workspace"]["path"],
        canonical_relocated.display().to_string()
    );
    let snapshot = NativeEndpoint::open(root.path())
        .expect("native endpoint")
        .session_config_snapshot(SESSION)
        .expect("session snapshot");
    assert_eq!(
        snapshot.workspace.selected_cwd.as_deref(),
        canonical_relocated.to_str()
    );

    let stale = block_on_ready(assembly.dispatch(request(
        "rpc-relocate-stale-source",
        "workspace.relocate",
        &relocate_payload,
    )))
    .expect("stale relocation response");
    assert!(!stale.result.ok, "stale compare-and-swap must fail closed");
    assert_eq!(
        stale.result.error.expect("error").code,
        "workspace-invalid-path"
    );
}

#[test]
fn session_create_stages_snapshots_and_recovers_from_pre_carrier_record() {
    let root = tempfile::tempdir().expect("root");
    let debris_digest = "aa00000000000000000000000000000000000000000000000000000000000000";
    let debris = root
        .path()
        .join("endpoint-management/operations/aa")
        .join(debris_digest)
        .join("payload");
    std::fs::create_dir_all(&debris).expect("recordless payload");
    std::fs::write(debris.join("candidate"), b"private").expect("recordless candidate");
    let project = root.path().join("project");
    std::fs::create_dir(&project).expect("project directory");
    let assembly = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
    )
    .expect("assembly");
    assert!(!debris.exists(), "recordless payload must be startup GC");
    let created_workspace = block_on_ready(assembly.dispatch(request(
        "rpc-create-workspace-for-session",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(project.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("workspace create");
    let workspace_id = value(&created_workspace)["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id")
        .to_owned();
    let invalid_profile = block_on_ready(assembly.dispatch(request(
        "rpc-session-invalid-profile",
        "session.create",
        &format!(r#"{{"workspaceId":"{workspace_id}","identityProfile":"unknown"}}"#),
    )))
    .expect("invalid profile response");
    assert_eq!(invalid_profile.result.error.unwrap().code, "bad-request");
    let create = request(
        "rpc-session-create",
        "session.create",
        &format!(
            r#"{{"sessionId":"{SESSION}","workspaceId":"{workspace_id}","identityProfile":"general"}}"#
        ),
    );
    let first = block_on_ready(assembly.dispatch(create.clone())).expect("session create");
    assert_eq!(value(&first), serde_json::json!({"sessionId":SESSION}));
    assert!(root.path().join("threads").join(SESSION).is_dir());
    let ledger =
        std::fs::read_to_string(root.path().join("threads").join(SESSION).join("main.jsonl"))
            .expect("session ledger");
    let genesis: Value = serde_json::from_str(ledger.lines().next().unwrap()).expect("genesis");
    assert_eq!(genesis["identity_profile"], "general");
    let inventory = endpoint::NativeEndpoint::open(root.path())
        .expect("inventory endpoint")
        .list_sessions(&std::collections::HashSet::new())
        .expect("inventory");
    assert_eq!(
        inventory
            .iter()
            .find(|item| item.session_id == SESSION)
            .unwrap()
            .identity_profile
            .as_deref(),
        Some("general")
    );

    let assets = std::fs::read_dir(root.path().join("threads").join(SESSION).join("assets"))
        .expect("assets")
        .count();
    assert_eq!(assets, 2);
    let retry = block_on_ready(assembly.dispatch(create)).expect("exact retry");
    assert_eq!(first, retry);

    let rpc_digest = format!("{:x}", sha2::Sha256::digest(b"rpc-session-create"));
    let record = root
        .path()
        .join("endpoint-management/operations")
        .join(&rpc_digest[..2])
        .join(format!("{rpc_digest}.json"));
    let mut operation: Value = serde_json::from_slice(
        &std::fs::read(&record).expect("record")[..std::fs::read(&record).unwrap().len() - 1],
    )
    .expect("operation JSON");
    operation["phase"] = Value::String("prepared".to_owned());
    operation.as_object_mut().unwrap().remove("response");
    let mut bytes = serde_json_canonicalizer::to_vec(&operation).expect("canonical operation");
    bytes.push(b'\n');
    std::fs::write(&record, bytes).expect("downgrade record");
    std::fs::rename(
        root.path().join("threads").join(SESSION),
        root.path().join("detached-session"),
    )
    .expect("detach published folder");
    drop(assembly);
    ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok("2026-08-28T09:01:00.000Z".to_owned())),
    )
    .expect("prepared recovery");
    assert!(root.path().join("threads").join(SESSION).is_dir());
}

#[test]
fn secondary_folder_session_binding_survives_create_and_path_move() {
    let root = tempfile::tempdir().expect("root");
    let first = root.path().join("project-primary");
    let second = root.path().join("project-secondary");
    std::fs::create_dir(&first).expect("primary project");
    std::fs::create_dir(&second).expect("secondary project");
    let assembly = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
    )
    .expect("assembly");
    let created_workspace = block_on_ready(assembly.dispatch(request(
        "rpc-create-multi-folder-workspace",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(first.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("workspace create");
    let workspace_id = value(&created_workspace)["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id")
        .to_owned();
    let repository = profile::ConfigRepository::open(root.path()).expect("repository");
    let mut workspace = repository.workspace(&workspace_id).expect("workspace");
    workspace.revision = 2;
    workspace.cwd.clear();
    workspace.folders = vec![
        profile::WorkspaceFolder {
            id: "primary".to_owned(),
            path: first.display().to_string(),
        },
        profile::WorkspaceFolder {
            id: "secondary".to_owned(),
            path: second.display().to_string(),
        },
    ];
    repository
        .publish_workspace(1, &workspace)
        .expect("publish multi-folder workspace");

    let response = block_on_ready(assembly.dispatch(request(
        "rpc-create-secondary-session",
        "session.create",
        &format!(
            r#"{{"cwd":{},"sessionId":"{SESSION}","workspaceId":"{workspace_id}"}}"#,
            serde_json::to_string(second.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("secondary folder session create");
    assert!(response.result.ok, "{response:?}");
    assert_eq!(value(&response), serde_json::json!({"sessionId":SESSION}));
    let ledger = root.path().join("threads").join(SESSION).join("main.jsonl");
    let first_line = std::fs::read_to_string(&ledger)
        .expect("ledger")
        .lines()
        .next()
        .expect("genesis")
        .to_owned();
    let genesis: Value = serde_json::from_str(&first_line).expect("genesis JSON");
    assert_eq!(genesis["folder_binding"], "secondary");

    let moved = root.path().join("project-secondary-moved");
    std::fs::rename(&second, &moved).expect("move selected folder");
    workspace.revision = 3;
    workspace.folders[1].path = moved.display().to_string();
    repository
        .publish_workspace(2, &workspace)
        .expect("publish moved binding");
    let snapshot = NativeEndpoint::open(root.path())
        .expect("native endpoint")
        .session_config_snapshot(SESSION)
        .expect("session snapshot after folder move");
    assert_eq!(
        snapshot.workspace.folder_binding.as_deref(),
        Some("secondary")
    );
    let first = first.canonicalize().expect("canonical primary");
    let moved = moved.canonicalize().expect("canonical moved secondary");
    assert_eq!(snapshot.workspace.selected_cwd.as_deref(), moved.to_str());
    assert_eq!(
        snapshot.workspace.cwd,
        vec![first.display().to_string(), moved.display().to_string()],
        "all folders retain authored order while execution follows the binding"
    );
}

#[test]
fn workspace_mutations_are_journaled_and_exact_retry_is_stable() {
    let root = tempfile::tempdir().expect("root");
    let project = root.path().join("project");
    std::fs::create_dir(&project).expect("project directory");
    let assembly = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
    )
    .expect("assembly");
    let create_request = request(
        "rpc-workspace-create",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(project.to_str().unwrap()).unwrap()
        ),
    );
    let first = block_on_ready(assembly.dispatch(create_request.clone())).expect("create");
    let second = block_on_ready(assembly.dispatch(create_request)).expect("retry");
    assert_eq!(first, second);
    let created = value(&first);
    assert_eq!(created["created"], true);
    let workspace_id = created["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id");
    assert_eq!(workspace_id.as_bytes().get(14), Some(&b'7'));
    assert!(
        root.path()
            .join("endpoint-management/workspaces")
            .join(format!("{workspace_id}.json"))
            .is_file()
    );

    let renamed = block_on_ready(assembly.dispatch(request(
        "rpc-workspace-rename",
        "workspace.rename",
        &format!(r#"{{"title":"Renamed","workspaceId":"{workspace_id}"}}"#),
    )))
    .expect("rename");
    assert_eq!(value(&renamed)["workspace"]["title"], "Renamed");

    drop(assembly);
    let _reopened = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok("2026-08-28T09:01:00.000Z".to_owned())),
    )
    .expect("recovery open");
    let sessions = endpoint::NativeEndpoint::open(root.path())
        .expect("native endpoint")
        .list_sessions(&HashSet::new())
        .expect("session snapshot");
    let workspaces = endpoint::ManagementStore::open(root.path())
        .expect("management")
        .list_workspaces(&sessions)
        .expect("workspace snapshot");
    assert_eq!(workspaces.items[0].title, "Renamed");
}

#[test]
fn inventory_reports_the_durable_permission_mode() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let list = || {
        endpoint::NativeEndpoint::open(root.path())
            .expect("native endpoint")
            .list_sessions(&HashSet::new())
            .expect("session snapshot")
    };
    let item = list()
        .into_iter()
        .find(|item| item.session_id == SESSION)
        .expect("session");
    assert_eq!(
        item.permission_mode, "workspace-write",
        "absent record is the default mode"
    );
    engine::write_permission_mode(
        &root.path().join("threads").join(SESSION),
        engine::PermissionMode::ReadOnly,
    )
    .expect("write mode");
    let item = list()
        .into_iter()
        .find(|item| item.session_id == SESSION)
        .expect("session");
    assert_eq!(item.permission_mode, "read-only");
    std::fs::write(
        root.path()
            .join("threads")
            .join(SESSION)
            .join("permission-mode.json"),
        b"{not json",
    )
    .expect("corrupt record");
    let item = list()
        .into_iter()
        .find(|item| item.session_id == SESSION)
        .expect("session");
    assert_eq!(
        item.permission_mode, "workspace-write",
        "a corrupt record falls back"
    );
}

#[test]
fn archive_and_unarchive_are_independent_journaled_operations() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open(root.path(), description()).expect("assembly");
    assert!(root.path().join("threads").join(SESSION).is_dir());
    let archive = block_on_ready(assembly.dispatch(request(
        "rpc-archive",
        "workspace.archiveSession",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("archive");
    assert_eq!(
        value(&archive),
        serde_json::json!({"archivedSessionIds":[SESSION]})
    );
    assert!(root.path().join("archive").join(SESSION).is_dir());
    assert!(!root.path().join("threads").join(SESSION).exists());
    bootstrap(root.path());
    assert!(
        !root.path().join("threads").join(SESSION).exists(),
        "an archived create retry must not create an active placeholder"
    );
    let unarchive = block_on_ready(assembly.dispatch(request(
        "rpc-unarchive",
        "workspace.unarchiveSession",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("unarchive");
    assert_eq!(value(&unarchive), serde_json::json!({"sessionId":SESSION}));
    assert!(root.path().join("threads").join(SESSION).is_dir());
}

struct CompleteRoutes;

impl ProductionEndpointRoutes for CompleteRoutes {
    fn capabilities(&self) -> std::collections::BTreeSet<String> {
        TEKES_UNARY_ROUTES
            .iter()
            .filter(|route| !route.implemented)
            .map(|route| route.name.to_owned())
            .collect()
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        _payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        assert!(principal.starts_with("uid:"));
        if request.operation == "session.prompt" {
            request
                .handoff
                .mark_handed_off(DurableHandoffProof {
                    delivery: "test".to_owned(),
                    durable_identity: Some(RpcDurableIdentity {
                        kind: "event-origin".to_owned(),
                        id: request.rpc_id.clone(),
                        seq: Some(2),
                    }),
                })
                .expect("handoff proof");
            return Ok(IJsonValue::parse_str(r#"{"accepted":true}"#).unwrap());
        }
        Ok(IJsonValue::parse_str("{}").unwrap())
    }
}

#[test]
fn complete_authority_seam_advertises_every_registered_route_without_placeholders() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open_with_route_authority(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        std::sync::Arc::new(CompleteRoutes),
    )
    .expect("full assembly");
    assert_eq!(assembly.host().capabilities().len(), 16);
    let accepted = block_on_ready(assembly.dispatch(request(
        "rpc-prompt-seam",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"text":"Hello","type":"text"}}],"mode":"queue","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("extension route");
    assert_eq!(value(&accepted), serde_json::json!({"accepted":true}));

    let malformed = block_on_ready(assembly.dispatch(request(
        "rpc-prompt-seam-malformed",
        "session.prompt",
        &format!(r#"{{"content":[],"extra":true,"mode":"queue","sessionId":"{SESSION}"}}"#),
    )))
    .expect("typed malformed");
    assert_eq!(malformed.result.error.unwrap().code, "bad-request");
}

struct UnavailableProvider {
    config_refreshes: std::sync::Arc<AtomicUsize>,
}

impl ProviderReadinessAuthority for UnavailableProvider {
    fn readiness(
        &self,
        _session_id: &str,
        _config: &profile::ConfigSnapshot,
    ) -> Result<Vec<RuntimeProviderReadiness>, ProductionRouteFailure> {
        Ok(vec![RuntimeProviderReadiness {
            provider: "deepseek".to_owned(),
            status: RuntimeProviderStatus::Failed {
                failure: RuntimeProviderFailure::Unavailable,
            },
            models: Vec::new(),
        }])
    }

    fn config_mutation_succeeded(&self, _session_id: &str) -> Result<(), ProductionRouteFailure> {
        self.config_refreshes.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

#[test]
fn model_catalog_and_selection_keep_runtime_failure_display_only() {
    let root = tempfile::tempdir().expect("root");
    let project = root.path().join("project-models");
    std::fs::create_dir(&project).expect("project directory");
    profile::ConfigRepository::open(root.path())
        .expect("config repository")
        .publish_providers(
            0,
            &profile::ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![profile::Provider {
                    id: "deepseek".to_owned(),
                    name: Some("DeepSeek".to_owned()),
                    adapter: "chat_completions".to_owned(),
                    dialect: "deepseek_chat_v1".to_owned(),
                    endpoint_owner: "deepseek".to_owned(),
                    gateway_translation: "direct".to_owned(),
                    evidence_revision: "deepseek-direct-chat-v4-2026-07-31+chat-completions-v1"
                        .to_owned(),
                    endpoint: "https://api.deepseek.com".to_owned(),
                    credential_key: None,
                    models: vec![profile::Model {
                        id: "deepseek-v4-flash".to_owned(),
                        profile: "deepseek_chat_v1:deepseek-v4-flash".to_owned(),
                        enabled: true,
                        context_window_tokens: 16_384,
                        compact_trigger_tokens: 12_000,
                    }],
                }],
                web_search: None,
            },
        )
        .expect("provider config");
    let config_refreshes = std::sync::Arc::new(AtomicUsize::new(0));
    let assembly = ProductionEndpointAssembly::open_with_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        Some(std::sync::Arc::new(UnavailableProvider {
            config_refreshes: std::sync::Arc::clone(&config_refreshes),
        })),
    )
    .expect("assembly");
    assert!(assembly.host().capabilities().contains("session.models"));
    assert!(
        assembly
            .host()
            .capabilities()
            .contains("session.selectModel")
    );
    let workspace = block_on_ready(assembly.dispatch(request(
        "rpc-model-workspace",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(project.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("workspace create");
    let workspace_id = value(&workspace)["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id")
        .to_owned();
    block_on_ready(assembly.dispatch(request(
        "rpc-model-session",
        "session.create",
        &format!(r#"{{"sessionId":"{SESSION}","workspaceId":"{workspace_id}"}}"#),
    )))
    .expect("session create");
    let selected_before_first_turn = block_on_ready(assembly.dispatch(request(
        "rpc-model-select-before-first-turn",
        "session.selectModel",
        &format!(
            r#"{{"model":"deepseek-v4-flash","provider":"deepseek","reasoningEffort":"high","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("select model before first turn");
    assert!(
        selected_before_first_turn.result.ok,
        "a genesis-only session is quiescent and must accept its first model selection: {:?}",
        selected_before_first_turn.result.error
    );
    let ledger = root.path().join("threads").join(SESSION).join("main.jsonl");
    let mut bytes = std::fs::read(&ledger).expect("genesis ledger");
    for event in [
        serde_json::json!({"v":1,"seq":2,"ts":TIMESTAMP,"kind":"input","content":[{"type":"text","text":"configure"}],"origin_key":"model-input","origin_tuple":{"principal":"user","client":"session-endpoint","target":SESSION,"op":"session.prompt","key":"model-input"}}),
        serde_json::json!({"v":1,"seq":3,"ts":TIMESTAMP,"kind":"turn_open","turn":1,"trigger":{"inputs":[2]}}),
    ] {
        bytes
            .extend_from_slice(&serde_json_canonicalizer::to_vec(&event).expect("canonical event"));
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, 1).expect("running ledger");
    std::fs::write(&ledger, &bytes).expect("running ledger write");
    let selected_during_turn = block_on_ready(assembly.dispatch(request(
        "rpc-model-select-during-turn",
        "session.selectModel",
        &format!(
            r#"{{"model":"deepseek-v4-flash","provider":"deepseek","reasoningEffort":"high","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("reject model selection during turn");
    assert_eq!(
        selected_during_turn.result.error.unwrap().code,
        "session-running"
    );
    bytes.extend_from_slice(
        &serde_json_canonicalizer::to_vec(
            &serde_json::json!({"v":1,"seq":4,"ts":TIMESTAMP,"kind":"settle","turn":1,"outcome":"completed"}),
        )
        .expect("canonical settle"),
    );
    bytes.push(b'\n');
    schema::validate_ledger(&bytes, 1).expect("settled ledger");
    std::fs::write(&ledger, bytes).expect("settled ledger write");

    let before = block_on_ready(assembly.dispatch(request(
        "rpc-models-before",
        "session.models",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("models");
    assert_eq!(value(&before)["groups"][0]["name"], "DeepSeek");
    assert_eq!(
        value(&before)["groups"][0]["models"][0]["contextWindow"],
        16_384
    );
    assert_eq!(
        value(&before)["groups"][0]["models"][0]["name"],
        "deepseek-v4-flash"
    );
    assert_eq!(
        value(&before)["groups"][0]["models"][0]["reasoning"]["defaultEffort"],
        "high"
    );
    assert_eq!(
        value(&before)["groups"][0]["models"][0]["reasoning"]["efforts"],
        serde_json::json!([
            {"id":"low","name":"low"},
            {"id":"high","name":"high"},
            {"id":"max","name":"max"}
        ])
    );
    assert_eq!(value(&before)["failures"][0]["message"], "unavailable");
    let invalid_effort = block_on_ready(assembly.dispatch(request(
        "rpc-model-select-invalid-effort",
        "session.selectModel",
        &format!(
            r#"{{"model":"deepseek-v4-flash","provider":"deepseek","reasoningEffort":"medium","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("typed invalid effort");
    assert_eq!(
        invalid_effort.result.error.unwrap().code,
        "model-unavailable"
    );
    let selected = block_on_ready(assembly.dispatch(request(
        "rpc-model-select",
        "session.selectModel",
        &format!(
            r#"{{"model":"deepseek-v4-flash","provider":"deepseek","reasoningEffort":"high","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("select model");
    assert!(
        selected.result.ok,
        "select model failed: {:?}",
        selected.result.error
    );
    assert_eq!(
        value(&selected),
        serde_json::json!({"selected":{
            "provider":"deepseek","model":"deepseek-v4-flash","reasoningEffort":"high"
        }})
    );
    let settings = profile::ConfigRepository::open(root.path())
        .expect("config repository")
        .session_settings(root.path().join("threads").join(SESSION))
        .expect("session settings")
        .expect("published settings");
    assert_eq!(settings.revision, 2);
    assert_eq!(settings.reasoning_effort.as_deref(), Some("high"));
    assert_eq!(config_refreshes.load(Ordering::SeqCst), 2);
    let after = block_on_ready(assembly.dispatch(request(
        "rpc-models-after",
        "session.models",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("models after selection");
    assert_eq!(value(&after)["routable"], true);
    assert_eq!(value(&after)["failures"][0]["message"], "unavailable");
}

#[test]
fn cancel_and_rename_use_exact_endpoint_origins_and_durable_receipts() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    profile::ConfigRepository::open(root.path())
        .unwrap()
        .publish_workspace(
            0,
            &profile::WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "workspace-1".into(),
                name: "Project".into(),
                cwd: vec![root.path().display().to_string()],
                folders: vec![],
                policy: None,
            },
        )
        .unwrap();
    let delivery = std::sync::Arc::new(LockedLedgerDelivery {
        endpoint: NativeEndpoint::open(root.path()).expect("native endpoint"),
    });
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        Some(delivery),
        None,
    )
    .expect("assembly");
    assert!(assembly.host().capabilities().contains("session.cancel"));
    assert!(assembly.host().capabilities().contains("session.rename"));
    assert!(assembly.host().capabilities().contains("session.prompt"));
    assert!(
        assembly
            .host()
            .capabilities()
            .contains("session.attachment")
    );
    let prompted = block_on_ready(assembly.dispatch(request(
        "rpc-session-prompt",
        "session.prompt",
        &format!(
            r#"{{"clientTimeZone":"Asia/Shanghai","content":[{{"text":"Hello","type":"text"}}],"mode":"queue","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("prompt");
    assert_eq!(value(&prompted), serde_json::json!({"accepted":true}));
    let renamed = block_on_ready(assembly.dispatch(request(
        "rpc-session-rename",
        "session.rename",
        &format!(r#"{{"sessionId":"{SESSION}","title":"Renamed"}}"#),
    )))
    .expect("rename");
    assert_eq!(
        value(&renamed),
        serde_json::json!({"title":"Renamed","seq":3})
    );
    let cancelled = block_on_ready(assembly.dispatch(request(
        "rpc-session-cancel",
        "session.cancel",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("cancel");
    assert_eq!(value(&cancelled), serde_json::json!({"accepted":true}));
    let ledger =
        std::fs::read_to_string(root.path().join("threads").join(SESSION).join("main.jsonl"))
            .expect("ledger");
    assert!(ledger.contains(r#""op":"session.rename""#));
    assert!(ledger.contains(r#""op":"session.cancel""#));
    assert!(ledger.contains(r#""op":"session.prompt""#));
}

#[test]
fn queue_route_builds_the_closed_worker_transaction_and_waits_for_commit() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        None,
        Some(std::sync::Arc::new(QueueAccept)),
    )
    .expect("assembly");
    assert!(
        assembly
            .host()
            .capabilities()
            .contains("session.updateQueue")
    );
    let response = block_on_ready(assembly.dispatch(request(
        "rpc-queue-edit",
        "session.updateQueue",
        &format!(
            r#"{{"action":{{"content":[{{"text":"replacement","type":"text"}}],"kind":"edit"}},"itemId":"input:2","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("queue edit");
    assert!(
        response.result.ok,
        "queue failed: {:?}",
        response.result.error
    );
    assert_eq!(value(&response), serde_json::json!({"accepted":true}));
}

#[test]
fn queue_management_gate_recovers_before_readiness_and_exact_retry_does_not_redeliver() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let rpc = request(
        "rpc-queue-recovery",
        "session.updateQueue",
        &format!(r#"{{"action":{{"kind":"remove"}},"itemId":"input:2","sessionId":"{SESSION}"}}"#),
    );
    let hash = request_hash(&rpc);
    let principal = format!("uid:{}", unsafe { libc::geteuid() });
    let retract = OriginTuple {
        principal,
        client: "session-endpoint".to_owned(),
        target: SESSION.to_owned(),
        op: "session.updateQueue".to_owned(),
        key: "rpc-queue-recovery/retract".to_owned(),
    };
    let action = IJsonValue::parse_str(r#"{"kind":"remove"}"#).unwrap();
    let management = ManagementStore::open_at(root.path(), TIMESTAMP).expect("management");
    let state = management
        .prepare_queue_transaction(QueueTransactionOperation {
            rpc_id: "rpc-queue-recovery",
            request_sha256: &hash,
            session_id: SESSION,
            target_seq: 2,
            action: &action,
            retract_origin: &retract,
            replacement_origin: None,
            asset_digests: &[],
            started_at: TIMESTAMP,
        })
        .expect("prepared queue operation");
    assert!(matches!(state, QueueTransactionState::Pending(_)));
    assert!(
        management
            .has_incomplete_session_operation(SESSION)
            .expect("management gate")
    );
    drop(management);

    let queue = std::sync::Arc::new(CountingQueue {
        calls: AtomicUsize::new(0),
    });
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        None,
        Some(queue.clone()),
    )
    .expect("queue recovery before readiness");
    assert_eq!(queue.calls.load(Ordering::SeqCst), 1);
    assert!(
        !assembly
            .has_incomplete_management_operation(SESSION)
            .expect("released management gate")
    );
    let retry = block_on_ready(assembly.dispatch(rpc)).expect("exact retry");
    assert_eq!(value(&retry), serde_json::json!({"accepted":true}));
    assert_eq!(queue.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn archived_precedes_image_decode_without_a_duplicate_delivery_preflight() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let delivery = std::sync::Arc::new(DeliveryFailure {
        code: "steer-unavailable",
        message: "Current turn no longer accepts steering",
        details: r#"{"itemId":"pending-steer"}"#,
    });
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        Some(delivery),
        None,
    )
    .expect("assembly");
    let steer = block_on_ready(assembly.dispatch(request(
        "rpc-steer-preflight",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"data":"iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=","mediaType":"image/png","type":"image"}}],"mode":"steer","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("steer rejection");
    assert_eq!(steer.result.error.unwrap().code, "steer-unavailable");
    assert_eq!(
        std::fs::read_dir(root.path().join("threads").join(SESSION).join("assets"))
            .expect("session asset directory")
            .count(),
        1,
        "delivery owns eligibility, so materialization is not preceded by a duplicate preflight"
    );

    block_on_ready(assembly.dispatch(request(
        "rpc-archive-before-invalid-image",
        "workspace.archiveSession",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("archive");
    let archived = block_on_ready(assembly.dispatch(request(
        "rpc-invalid-image-on-archive",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"data":"not-base64","mediaType":"image/png","type":"image"}}],"mode":"queue","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("archived rejection");
    assert_eq!(archived.result.error.unwrap().code, "archived");
}

#[test]
fn injected_route_error_outside_closed_union_fails_internal() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        Some(std::sync::Arc::new(DeliveryFailure {
            code: "workspace-not-found",
            message: "Workspace was not found",
            details: r#"{"workspaceId":"workspace-a"}"#,
        })),
        None,
    )
    .expect("assembly");
    let response = block_on_ready(assembly.dispatch(request(
        "rpc-invalid-route-error",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"text":"hello","type":"text"}}],"mode":"queue","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("typed response");
    let error = response.result.error.expect("semantic error");
    assert_eq!(error.code, "internal");
    assert_eq!(error.message, "Endpoint operation failed");
}

#[test]
fn injected_route_error_with_allowed_code_and_wrong_message_fails_internal() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        Some(std::sync::Arc::new(DeliveryFailure {
            code: "steer-unavailable",
            message: "injected authority diagnostic",
            details: r#"{"itemId":"pending-steer"}"#,
        })),
        None,
    )
    .expect("assembly");
    let response = block_on_ready(assembly.dispatch(request(
        "rpc-invalid-route-message",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"text":"hello","type":"text"}}],"mode":"steer","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("typed response");
    let error = response.result.error.expect("semantic error");
    assert_eq!(error.code, "internal");
    assert_eq!(error.message, "Endpoint operation failed");
    assert_eq!(error.details, IJsonValue::parse_str("{}").unwrap());
}

#[test]
fn injected_route_error_with_extra_detail_fails_internal() {
    let root = tempfile::tempdir().expect("root");
    bootstrap(root.path());
    let assembly = ProductionEndpointAssembly::open_with_full_authorities(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
        None,
        None,
        Some(std::sync::Arc::new(DeliveryFailure {
            code: "steer-unavailable",
            message: "Current turn no longer accepts steering",
            details: r#"{"diagnostic":"must-not-leak","itemId":"pending-steer"}"#,
        })),
        None,
    )
    .expect("assembly");
    let response = block_on_ready(assembly.dispatch(request(
        "rpc-invalid-route-details",
        "session.prompt",
        &format!(
            r#"{{"content":[{{"text":"hello","type":"text"}}],"mode":"steer","sessionId":"{SESSION}"}}"#
        ),
    )))
    .expect("typed response");
    let error = response.result.error.expect("semantic error");
    assert_eq!(error.code, "internal");
    assert_eq!(error.message, "Endpoint operation failed");
    assert_eq!(error.details, IJsonValue::parse_str("{}").unwrap());
}

/// Appends one user turn to the session ledger: input, turn_open and, when
/// `settled`, the completing settle. Returns the next free seq.
fn append_turn(root: &std::path::Path, session: &str, first_seq: u64, settled: bool) -> u64 {
    let ledger = root.join("threads").join(session).join("main.jsonl");
    let mut bytes = std::fs::read(&ledger).expect("ledger");
    let turn = first_seq.div_ceil(3);
    let key = format!("input-{first_seq}");
    let mut events = vec![
        serde_json::json!({"v":1,"seq":first_seq,"ts":TIMESTAMP,"kind":"input","content":[{"type":"text","text":"hello"}],"origin_key":key,"origin_tuple":{"principal":"user","client":"session-endpoint","target":session,"op":"session.prompt","key":key}}),
        serde_json::json!({"v":1,"seq":first_seq + 1,"ts":TIMESTAMP,"kind":"turn_open","turn":turn,"trigger":{"inputs":[first_seq]}}),
    ];
    if settled {
        events.push(serde_json::json!({"v":1,"seq":first_seq + 2,"ts":TIMESTAMP,"kind":"settle","turn":turn,"outcome":"completed"}));
    }
    for event in &events {
        bytes.extend_from_slice(&serde_json_canonicalizer::to_vec(event).expect("canonical event"));
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, 1).expect("valid ledger");
    std::fs::write(&ledger, &bytes).expect("ledger write");
    first_seq + events.len() as u64
}

#[test]
fn ephemeral_fork_is_a_scratch_ledger_that_discard_and_startup_remove() {
    let root = tempfile::tempdir().expect("root");
    let project = root.path().join("project");
    std::fs::create_dir(&project).expect("project directory");
    let assembly = ProductionEndpointAssembly::open_with_clock(
        root.path(),
        description(),
        std::sync::Arc::new(|| Ok(TIMESTAMP.to_owned())),
    )
    .expect("assembly");
    let created_workspace = block_on_ready(assembly.dispatch(request(
        "rpc-ephemeral-workspace",
        "workspace.create",
        &format!(
            r#"{{"path":{}}}"#,
            serde_json::to_string(project.to_str().unwrap()).unwrap()
        ),
    )))
    .expect("workspace create");
    let workspace_id = value(&created_workspace)["workspace"]["workspaceId"]
        .as_str()
        .expect("workspace id")
        .to_owned();
    block_on_ready(assembly.dispatch(request(
        "rpc-ephemeral-source",
        "session.create",
        &format!(r#"{{"sessionId":"{SESSION}","workspaceId":"{workspace_id}","identityProfile":"general"}}"#),
    )))
    .expect("session create");
    let next = append_turn(root.path(), SESSION, 2, true);
    // The source is mid-turn: an unnamed fork position anchors to the settled turn.
    append_turn(root.path(), SESSION, next, false);
    let fork = block_on_ready(assembly.dispatch(request(
        "rpc-fork-ephemeral",
        "session.fork",
        &format!(r#"{{"ephemeral":true,"sessionId":"{SESSION}"}}"#),
    )))
    .expect("fork");
    if let Some(error) = fork.result.error.as_ref() {
        panic!("fork failed: {} {}", error.code, error.message);
    }
    let forked = value(&fork)["sessionId"]
        .as_str()
        .expect("session id")
        .to_owned();
    let folder = root.path().join("threads").join(&forked);
    assert!(folder.is_dir());
    let genesis = std::fs::read_to_string(folder.join("main.jsonl")).expect("ledger");
    let genesis: Value = serde_json::from_str(genesis.lines().next().expect("genesis")).unwrap();
    assert_eq!(genesis["ephemeral"], Value::Bool(true));
    let copied = std::fs::read(folder.join("main.jsonl")).expect("forked ledger");
    let projection = schema::validate_ledger(&copied, 1).expect("forked ledger validates");
    assert_eq!(
        projection.lifecycle.latest_turn,
        Some(1),
        "the open second turn is not copied"
    );
    assert!(projection.lifecycle.terminal_tail);

    let sessions = endpoint::NativeEndpoint::open(root.path())
        .expect("native endpoint")
        .list_sessions(&HashSet::new())
        .expect("session snapshot");
    let item = sessions
        .iter()
        .find(|item| item.session_id == forked)
        .expect("listed");
    assert!(item.ephemeral);
    assert!(
        !sessions
            .iter()
            .find(|item| item.session_id == SESSION)
            .unwrap()
            .ephemeral
    );
    let lineage = ManagementStore::open(root.path())
        .expect("management")
        .completed_fork_lineage()
        .expect("lineage");
    assert_eq!(lineage[&forked].source, SESSION);
    assert!(lineage[&forked].ephemeral);

    let archive = block_on_ready(assembly.dispatch(request(
        "rpc-archive-ephemeral",
        "workspace.archiveSession",
        &format!(r#"{{"sessionId":"{forked}"}}"#),
    )))
    .expect("archive response");
    assert_eq!(
        archive.result.error.expect("archive refused").code,
        "ephemeral"
    );
    let durable_discard = block_on_ready(assembly.dispatch(request(
        "rpc-discard-durable",
        "session.discard",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("discard response");
    assert_eq!(
        durable_discard.result.error.expect("discard refused").code,
        "not-ephemeral"
    );
    assert!(root.path().join("threads").join(SESSION).is_dir());

    let discard = block_on_ready(assembly.dispatch(request(
        "rpc-discard-ephemeral",
        "session.discard",
        &format!(r#"{{"sessionId":"{forked}"}}"#),
    )))
    .expect("discard");
    assert_eq!(value(&discard), serde_json::json!({"sessionId":forked}));
    assert!(!folder.exists());
    assert_eq!(
        std::fs::read_dir(root.path().join(".rewrite-trash"))
            .unwrap()
            .count(),
        0
    );
    let replay = block_on_ready(assembly.dispatch(request(
        "rpc-discard-ephemeral",
        "session.discard",
        &format!(r#"{{"sessionId":"{forked}"}}"#),
    )))
    .expect("replayed discard");
    assert_eq!(value(&replay), serde_json::json!({"sessionId":forked}));

    // A second ephemeral fork left behind is swept by the startup preflight.
    let fork = block_on_ready(assembly.dispatch(request(
        "rpc-fork-ephemeral-2",
        "session.fork",
        &format!(r#"{{"ephemeral":true,"sessionId":"{SESSION}"}}"#),
    )))
    .expect("fork");
    let leftover = value(&fork)["sessionId"].as_str().unwrap().to_owned();
    assert!(root.path().join("threads").join(&leftover).is_dir());
    let durable_fork = block_on_ready(assembly.dispatch(request(
        "rpc-fork-durable",
        "session.fork",
        &format!(r#"{{"sessionId":"{SESSION}"}}"#),
    )))
    .expect("fork");
    let durable = value(&durable_fork)["sessionId"]
        .as_str()
        .unwrap()
        .to_owned();
    let fork_ledger = std::fs::read_to_string(
        root.path()
            .join("threads")
            .join(&durable)
            .join("main.jsonl"),
    )
    .expect("fork ledger");
    let fork_genesis: Value =
        serde_json::from_str(fork_ledger.lines().next().unwrap()).expect("fork genesis");
    assert_eq!(fork_genesis["identity_profile"], "general");
    // `host_runtime::prepare_storage` runs this sweep before any endpoint exists.
    let swept = store::ThreadStore::open(root.path())
        .expect("store")
        .sweep_ephemeral()
        .expect("sweep");
    assert_eq!(swept, vec![leftover.clone()]);
    assert!(!root.path().join("threads").join(&leftover).exists());
    assert!(root.path().join("threads").join(&durable).is_dir());
    assert!(root.path().join("threads").join(SESSION).is_dir());
}
