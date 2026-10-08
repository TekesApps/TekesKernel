use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use endpoint::{
    ClientRequest, EndpointDispatcher, EndpointHost, MethodClass, RpcRegistry,
    SessionHostDescription,
};
use profile::{ConfigRepository, WorkspaceConfig, WorkspacePolicy};
use schema::IJsonValue;
use serde_json::{Value, json};
use tekes_supervisor::client_admin::{
    ClientAdminRoutes, WORKSPACE_FOLDER_METHODS, WORKSPACE_POLICY_METHODS,
};
use tekes_supervisor::client_extensions::{
    APPROVAL_METHODS, ATTACHMENT_METHODS, FEEDBACK_METHODS, FILE_METHODS, GOAL_METHODS,
    HOST_FILE_METHODS, INITIAL_PRESET_METHODS, MCP_METHODS, PLUGIN_METHODS, RECOVERY_METHODS,
    RESOURCE_METHODS, SCHEDULE_METHODS, SETTINGS_METHODS, SUBAGENT_METHODS, THREAD_SEARCH_METHODS,
    TOOL_METHODS, USAGE_METHODS,
};
use tekes_supervisor::endpoint_host::{
    CompositeProductionEndpointRoutes, ProductionEndpointHost, ProductionEndpointRoutes,
    TEKES_UNARY_ROUTES,
};
use tekes_supervisor::host_runtime::assemble_application_endpoint_host;
use tekes_supervisor::process_host::ProductionProcessHost;
use tempfile::TempDir;

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn fixture(name: &str) -> Value {
    let path = repository_root()
        .join("fixtures/client-extensions")
        .join(name);
    serde_json::from_slice(&fs::read(path).expect("fixture bytes")).expect("fixture JSON")
}

fn request(rpc_id: &str, method: &str, payload: &Value) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: method.to_owned(),
        payload: IJsonValue::parse(&serde_json::to_vec(payload).expect("request JSON"))
            .expect("request I-JSON"),
    }
}

struct ProductionFixture {
    root: TempDir,
    host: ProductionEndpointHost,
    process: Arc<ProductionProcessHost>,
}

impl Drop for ProductionFixture {
    fn drop(&mut self) {
        self.process.shutdown();
    }
}

fn production_fixture(with_workspace: bool) -> ProductionFixture {
    let root = TempDir::new().expect("production root");
    let agent = root.path().join(".agent");
    fs::create_dir_all(&agent).expect("agent root");
    let worker = root.path().join("fixture-worker");
    fs::write(
        &worker,
        b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":1,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
    )
    .expect("fixture worker");
    fs::set_permissions(&worker, fs::Permissions::from_mode(0o700)).expect("worker mode");
    let helper = root.path().join("tekes-helper");
    fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fixture helper");
    fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");
    if with_workspace {
        let workspace_root = root.path().join("workspace");
        fs::create_dir(&workspace_root).expect("workspace root");
        let workspace_root = fs::canonicalize(workspace_root).expect("canonical workspace root");
        ConfigRepository::open(root.path())
            .expect("config repository")
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-1".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: vec![workspace_root.to_string_lossy().into_owned()],
                    folders: Vec::new(),
                    policy: Some(WorkspacePolicy::default()),
                },
            )
            .expect("workspace");
    }
    let process =
        ProductionProcessHost::open(root.path(), worker, env!("CARGO_PKG_VERSION"), &agent)
            .expect("process host");
    let host = assemble_application_endpoint_host(
        root.path(),
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            cwd: root.path().to_string_lossy().into_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: root.path().to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-08-29T00:00:00.000000000Z".to_owned())),
        &agent,
        Arc::clone(&process),
    )
    .expect("production endpoint host");
    ProductionFixture {
        root,
        host,
        process,
    }
}

async fn dispatch(
    fixture: &ProductionFixture,
    rpc_id: &str,
    method: &str,
    payload: Value,
) -> endpoint::ServerResponse {
    EndpointDispatcher::new(RpcRegistry::open(fixture.root.path()).expect("rpc registry"))
        .dispatch(&fixture.host, request(rpc_id, method, &payload))
        .await
        .expect("endpoint response")
}

fn success_value(response: &endpoint::ServerResponse) -> Value {
    assert!(response.result.ok, "{response:?}");
    serde_json::from_slice(
        &response
            .result
            .value
            .as_ref()
            .expect("success value")
            .canonical_bytes()
            .expect("canonical response"),
    )
    .expect("response JSON")
}

fn write_plugin_package(root: &Path) -> PathBuf {
    let package = root.join("fixture-plugin");
    fs::create_dir(&package).expect("plugin package");
    let executable = package.join("mcp-server");
    fs::write(
        &executable,
        b"#!/bin/sh\nIFS= read -r line\nprintf '%s\\n' '{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":{\"capabilities\":{},\"protocolVersion\":\"2025-11-25\",\"serverInfo\":{\"name\":\"gate116\",\"version\":\"1\"}}}'\nIFS= read -r line\nwhile IFS= read -r line; do printf '%s\\n' '{\"id\":2,\"jsonrpc\":\"2.0\",\"result\":{}}'; done\n",
    )
    .expect("MCP executable");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755))
        .expect("MCP executable mode");
    let operating_system = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    };
    let architectures = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        // Slice 14A's package vocabulary is arm64. Keep the Rust spelling in
        // this transition fixture too so the same package qualifies both the
        // current host and the normalized production host after integration.
        vec!["arm64", "aarch64"]
    } else {
        vec![std::env::consts::ARCH]
    };
    assert!(
        !cfg!(all(target_os = "macos", target_arch = "aarch64"))
            || architectures.contains(&"arm64")
    );
    fs::write(
        package.join("tekes-plugin.json"),
        serde_json::to_vec(&json!({
            "manifestVersion":1,
            "id":"com.example.gate116",
            "version":"1.0.0",
            "displayName":"Gate 116",
            "platforms":[{"os":operating_system,"architectures":architectures}],
            "capabilities":[],
            "components":[{"id":"server","type":"mcp-server","path":"mcp-server","capabilities":[]}]
        }))
        .expect("plugin manifest"),
    )
    .expect("plugin manifest file");
    package
}

fn write_archived_ledger(root: &Path) -> String {
    let session = "018f0000-0000-7000-8000-000000000099";
    let folder = root.join("archive").join(session);
    fs::create_dir_all(folder.join("assets")).expect("archive assets");
    let events = [
        json!({"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":session},"resume":"never","seq":1,"thread":session,"ts":"2026-08-29T00:00:00.000Z","v":1,"workspace":"workspace-1"}),
        json!({"content":[{"text":"review archived","type":"text"}],"kind":"input","origin_key":"input","origin_tuple":{"client":"cli","key":"input","op":"submit","principal":"p","target":session},"seq":2,"ts":"2026-08-29T00:00:01.000Z","v":1}),
        json!({"kind":"turn_open","seq":3,"trigger":{"inputs":[2]},"ts":"2026-08-29T00:00:02.000Z","turn":1,"v":1}),
        json!({"kind":"settle","outcome":"completed","seq":4,"ts":"2026-08-29T00:00:03.000Z","turn":1,"v":1}),
        json!({"kind":"meta","seq":5,"title":"review archived","ts":"2026-08-29T00:00:04.000Z","v":1}),
    ];
    let mut ledger = Vec::new();
    for event in events {
        ledger.extend(serde_json_canonicalizer::to_vec(&event).expect("event canonical"));
        ledger.push(b'\n');
    }
    fs::write(folder.join("main.jsonl"), ledger).expect("archive ledger");
    session.to_owned()
}

fn catalog_groups() -> BTreeMap<String, BTreeMap<String, MethodClass>> {
    fixture("catalog.canonical.json")["capabilities"]
        .as_array()
        .expect("capabilities")
        .iter()
        .map(|group| {
            let id = group["id"].as_str().expect("capability id").to_owned();
            let methods = group["methods"]
                .as_array()
                .expect("methods")
                .iter()
                .map(|method| {
                    let class = match method["class"].as_str().expect("method class") {
                        "read" => MethodClass::ReadOnly,
                        "mutation" => MethodClass::Mutation,
                        other => panic!("unknown method class {other}"),
                    };
                    (
                        method["name"].as_str().expect("method name").to_owned(),
                        class,
                    )
                })
                .collect();
            (id, methods)
        })
        .collect()
}

fn production_group_constants() -> BTreeMap<String, BTreeMap<String, MethodClass>> {
    [
        ("resources.v1", RESOURCE_METHODS.as_slice()),
        ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
        ("recovery.v1", RECOVERY_METHODS.as_slice()),
        ("attachments.v1", ATTACHMENT_METHODS.as_slice()),
        ("approvals.v1", APPROVAL_METHODS.as_slice()),
        ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),
        ("feedback.v1", FEEDBACK_METHODS.as_slice()),
        ("settings.v1", SETTINGS_METHODS.as_slice()),
        ("goals.v1", GOAL_METHODS.as_slice()),
        ("subagents.v1", SUBAGENT_METHODS.as_slice()),
        ("sessionFiles.v1", FILE_METHODS.as_slice()),
        ("tools.v1", TOOL_METHODS.as_slice()),
        ("plugins.v1", PLUGIN_METHODS.as_slice()),
        ("mcp.v1", MCP_METHODS.as_slice()),
        ("schedule.v1", SCHEDULE_METHODS.as_slice()),
        ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
        ("workspacePolicy.v1", WORKSPACE_POLICY_METHODS.as_slice()),
        ("workspaceFolders.v1", WORKSPACE_FOLDER_METHODS.as_slice()),
        ("usage.v1", USAGE_METHODS.as_slice()),
    ]
    .into_iter()
    .map(|(id, methods)| {
        (
            id.to_owned(),
            methods
                .iter()
                .map(|(name, class)| ((*name).to_owned(), *class))
                .collect(),
        )
    })
    .collect()
}

#[test]
fn slice14f_gate_113_catalog_negotiation_and_base_isolation() {
    let catalog = fixture("catalog.canonical.json");
    let base = catalog["base"]
        .as_array()
        .expect("base")
        .iter()
        .map(|name| name.as_str().expect("base name"))
        .collect::<Vec<_>>();
    let mut v3_base = TEKES_UNARY_ROUTES
        .iter()
        .map(|route| route.name)
        .collect::<Vec<_>>();
    v3_base.push("remote.mux");
    assert_eq!(base, v3_base);

    let expected_groups = catalog_groups();
    assert_eq!(expected_groups, production_group_constants());
    let expected_methods = expected_groups
        .values()
        .flat_map(|methods| methods.keys().cloned())
        .collect::<BTreeSet<_>>();
    assert_eq!(expected_methods.len(), 63);

    let production = production_fixture(false);
    assert_eq!(production.host.extension_capabilities(), expected_methods);
    assert!(
        base.iter()
            .all(|name| !production.host.extension_capabilities().contains(*name))
    );

    // The workspace policy and folder groups are real production route
    // owners, and composing one twice is a duplicate-ownership assembly failure.
    let admin_root = TempDir::new().expect("admin root");
    let agent = admin_root.path().join(".agent");
    fs::create_dir_all(&agent).expect("agent");
    let process =
        ProductionProcessHost::open(admin_root.path(), "/usr/bin/true", "test-build", &agent)
            .expect("process host");
    let admin =
        Arc::new(ClientAdminRoutes::new(admin_root.path(), process).expect("admin authority"));
    let routes = admin.routes();
    assert_eq!(routes.len(), 2);
    let composed = CompositeProductionEndpointRoutes::compose(routes.clone()).expect("compose");
    assert_eq!(
        composed.capabilities(),
        WORKSPACE_POLICY_METHODS
            .iter()
            .chain(WORKSPACE_FOLDER_METHODS.iter())
            .map(|(name, _)| (*name).to_owned())
            .collect::<BTreeSet<_>>()
    );
    assert!(
        CompositeProductionEndpointRoutes::compose([
            Arc::clone(&routes[0]),
            Arc::clone(&routes[0])
        ])
        .is_err(),
        "duplicate production ownership must fail assembly"
    );
}

#[tokio::test]
async fn slice14f_gate_114_closed_dtos_authorities_and_idempotency() {
    let production = production_fixture(true);
    let cases = fixture("method-cases.canonical.json")["cases"]
        .as_array()
        .expect("method cases")
        .clone();
    assert_eq!(cases.len(), 63);
    for (index, case) in cases.iter().enumerate() {
        let method = case["method"].as_str().expect("method");
        let payload = &case["request"];
        production
            .host
            .validate_request(&request(&format!("valid-{index}"), method, payload))
            .unwrap_or_else(|error| panic!("{method} canonical request rejected: {error:?}"));
        let mut unknown = payload.clone();
        unknown
            .as_object_mut()
            .expect("closed request object")
            .insert("__unknown".to_owned(), Value::Bool(true));
        assert!(
            production
                .host
                .validate_request(&request(&format!("invalid-{index}"), method, &unknown))
                .is_err(),
            "{method} accepted an unknown request field"
        );
        let expected_class = catalog_groups()
            .values()
            .find_map(|methods| methods.get(method))
            .copied()
            .expect("catalog method");
        assert_eq!(
            production.host.method_class(method),
            expected_class,
            "{method}"
        );
    }

    let mcp_save = json!({"credentialFields":{},"server":{"always_on":false,"enabled":true,"project_trusted":true,"protocol_mode":"legacy","reference":{"name":"fixture","scope":"project","workspace_id":"workspace-1"},"transport":{"command":["/usr/bin/true"],"environment":{},"kind":"stdio"}}});
    let mcp_first = dispatch(
        &production,
        "mcp-save-idempotent",
        "mcp.save",
        mcp_save.clone(),
    )
    .await;
    let mcp_repeated = dispatch(
        &production,
        "mcp-save-idempotent",
        "mcp.save",
        mcp_save.clone(),
    )
    .await;
    assert!(mcp_first.result.ok, "{mcp_first:?}");
    assert_eq!(mcp_first.result, mcp_repeated.result);
    let mut mcp_changed = mcp_save;
    mcp_changed["server"]["enabled"] = Value::Bool(false);
    let mcp_conflict = dispatch(&production, "mcp-save-idempotent", "mcp.save", mcp_changed).await;
    assert_eq!(
        mcp_conflict
            .result
            .error
            .expect("MCP idempotency conflict")
            .code,
        "idempotency-conflict"
    );
}

#[tokio::test]
async fn slice14f_gate_115_predecessor_disposition_and_no_special_cases() {
    let production = production_fixture(false);
    let disposition = fixture("dispositions.canonical.json");
    let production_methods = production.host.extension_capabilities();
    let base = TEKES_UNARY_ROUTES
        .iter()
        .map(|route| route.name.to_owned())
        .chain(["remote.mux".to_owned()])
        .collect::<BTreeSet<_>>();
    let allowed = base
        .union(&production_methods)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut referenced = BTreeSet::new();
    let mut retired = Vec::new();
    for row in disposition["rows"].as_array().expect("disposition rows") {
        let status = row["status"].as_str().expect("status");
        for replacement in row["replacement"].as_array().expect("replacements") {
            let replacement = replacement.as_str().expect("replacement");
            assert!(
                allowed.contains(replacement),
                "unknown replacement {replacement}"
            );
            referenced.insert(replacement.to_owned());
        }
        if status == "retired" {
            retired.extend(
                row["legacy"]
                    .as_array()
                    .expect("legacy")
                    .iter()
                    .map(|name| name.as_str().expect("legacy name").to_owned()),
            );
        }
    }
    assert!(production_methods.is_subset(&referenced));
    assert!(production_methods.iter().all(|method| {
        !method.contains("computerUse")
            && !method.starts_with("git/")
            && !method.contains("marketplace")
    }));

    let dispatcher =
        EndpointDispatcher::new(RpcRegistry::open(production.root.path()).expect("rpc registry"));
    for (index, method) in retired.iter().enumerate() {
        assert!(!production.host.capabilities().contains(method));
        let response = dispatcher
            .dispatch(
                &production.host,
                request(&format!("retired-{index}"), method, &json!({})),
            )
            .await
            .expect("typed unsupported response");
        assert!(
            !response.result.ok,
            "retired route {method} returned success"
        );
        assert_eq!(
            response.result.error.expect("typed error").code,
            "unsupported-capability",
            "{method}"
        );
    }
}

#[tokio::test]
async fn slice14f_gate_116_cross_capability_lifecycle() {
    let production = production_fixture(true);
    assert_eq!(production.host.extension_capabilities().len(), 63);

    let package = write_plugin_package(production.root.path());
    let inspected = success_value(
        &dispatch(
            &production,
            "plugin-inspect",
            "plugin/inspect",
            json!({"archivePath":package}),
        )
        .await,
    );
    let installed = success_value(
        &dispatch(
            &production,
            "plugin-install",
            "plugin/install",
            json!({"archivePath":package,"packageDigest":inspected["packageDigest"],"enable":true,"grants":[],"allowDowngrade":false,"allowSameVersionReplacement":false}),
        )
        .await,
    );
    assert_eq!(installed["readiness"]["state"], "ready");
    let mcp_enabled = success_value(
        &dispatch(
            &production,
            "mcp-list-enabled",
            "mcp.list",
            json!({"workspaceId":"workspace-1"}),
        )
        .await,
    );
    assert!(mcp_enabled["servers"].as_array().is_some_and(|rows| {
        rows.iter()
            .any(|row| row["server"]["owner"] == "com.example.gate116")
    }));
    success_value(
        &dispatch(
            &production,
            "plugin-disable",
            "plugin/setEnabled",
            json!({"pluginId":"com.example.gate116","enabled":false}),
        )
        .await,
    );
    let mcp_disabled = success_value(
        &dispatch(
            &production,
            "mcp-list-disabled",
            "mcp.list",
            json!({"workspaceId":"workspace-1"}),
        )
        .await,
    );
    assert!(!mcp_disabled["servers"].as_array().is_some_and(|rows| {
        rows.iter()
            .any(|row| row["server"]["owner"] == "com.example.gate116")
    }));
    let stale_plugin_server = dispatch(
        &production,
        "mcp-stale-plugin-generation",
        "mcp.get",
        json!({"reference":{"name":"server","scope":"plugin","workspace_id":"workspace-1"}}),
    )
    .await;
    assert_eq!(
        stale_plugin_server
            .result
            .error
            .expect("stale plugin server rejection")
            .code,
        "mcp-not-found"
    );

    let invalid_schedule = dispatch(
        &production,
        "schedule-invalid",
        "schedule.save",
        json!({"definition":{"cron":"0 9 * * 1-5","enabled":false,"id":"018f0000-0000-7000-8000-000000000011","missed_policy":"skip_and_record","model_id":"unproved","name":"Daily","permission_mode":"inherit","prompt":"review","time_zone":"Asia/Shanghai","workspace_id":"workspace-1"}}),
    )
    .await;
    assert_eq!(
        invalid_schedule.result.error.expect("schedule error").code,
        "schedule-model"
    );
    success_value(
        &dispatch(
            &production,
            "schedule-save",
            "schedule.save",
            json!({"definition":{"cron":"0 9 * * 1-5","enabled":false,"id":"018f0000-0000-7000-8000-000000000011","missed_policy":"skip_and_record","name":"Daily","permission_mode":"inherit","prompt":"review","time_zone":"Asia/Shanghai","workspace_id":"workspace-1"}}),
        )
        .await,
    );
    let schedules = success_value(
        &dispatch(
            &production,
            "schedule-list",
            "schedule.list",
            json!({"workspaceId":"workspace-1"}),
        )
        .await,
    );
    assert_eq!(schedules["schedules"][0]["definition"]["prompt"], "review");
    let run_now_response = dispatch(
        &production,
        "schedule-run-now",
        "schedule.runNow",
        json!({"taskId":"018f0000-0000-7000-8000-000000000011"}),
    )
    .await;
    assert!(
        run_now_response.result.ok,
        "schedule failure: {:?}; restart: {:?}; response: {run_now_response:?}",
        production.process.schedule_failure(),
        production.process.start_schedule_timer()
    );
    let run_now = success_value(&run_now_response);
    assert_eq!(run_now["claim"]["definition"]["prompt"], "review");

    let archived_session = write_archived_ledger(production.root.path());
    let search = success_value(
        &dispatch(
            &production,
            "search-archived",
            "thread.search",
            json!({"workspaceId":"workspace-1","query":"review","visibility":"archived","limit":20}),
        )
        .await,
    );
    assert_eq!(
        search["results"][0]["session_id"], archived_session,
        "{search}"
    );
    let usage = success_value(
        &dispatch(
            &production,
            "usage-archived",
            "usage.summary",
            json!({"sessionId":archived_session}),
        )
        .await,
    );
    assert_eq!(usage["totalTokens"], 0);

    let relaxed = success_value(
        &dispatch(
            &production,
            "policy-relax",
            "workspace.policy.set",
            json!({"workspaceId":"workspace-1","expectedRevision":1,"policy":{"allowed_tools":[],"network":false,"writable_roots":[]}}),
        )
        .await,
    );
    assert_eq!(relaxed["revision"], 2);
    let escalated = dispatch(
        &production,
        "policy-escalate",
        "workspace.policy.set",
        json!({"workspaceId":"workspace-1","expectedRevision":2,"policy":{"allowed_tools":["shell"],"network":true,"writable_roots":["/"]}}),
    )
    .await;
    assert_eq!(
        escalated.result.error.expect("policy error").code,
        "policy-escalation"
    );
}

#[tokio::test]
async fn workspace_folders_round_trip_through_the_production_host() {
    let production = production_fixture(true);
    let primary = fs::canonicalize(production.root.path().join("workspace"))
        .expect("primary")
        .to_string_lossy()
        .into_owned();
    let second = production.root.path().join("second");
    fs::create_dir(&second).expect("second folder");
    let second = fs::canonicalize(second)
        .expect("canonical second")
        .to_string_lossy()
        .into_owned();

    let listed = success_value(
        &dispatch(
            &production,
            "folders-list",
            "workspace.listFolders",
            json!({"workspaceId":"workspace-1"}),
        )
        .await,
    );
    assert_eq!(
        listed,
        json!({"format":1,"workspaceId":"workspace-1","revision":1,
            "folders":[{"folderId":"folder-0001","path":primary}]})
    );

    let add = json!({"workspaceId":"workspace-1","path":second});
    let added = dispatch(
        &production,
        "folders-add",
        "workspace.addFolder",
        add.clone(),
    )
    .await;
    let added_value = success_value(&added);
    assert_eq!(added_value["workspace"]["path"], json!(primary));
    assert_eq!(
        added_value["folders"]["folders"],
        json!([{"folderId":"folder-0001","path":primary},{"folderId":"folder-0002","path":second}])
    );
    // The transport replays an exact retry; a reused rpc id with another
    // payload is an idempotency conflict, never a second mutation.
    let replayed = dispatch(&production, "folders-add", "workspace.addFolder", add).await;
    assert_eq!(added.result, replayed.result);
    let conflict = dispatch(
        &production,
        "folders-add",
        "workspace.addFolder",
        json!({"workspaceId":"workspace-1","path":primary}),
    )
    .await;
    assert_eq!(
        conflict.result.error.expect("conflict").code,
        "idempotency-conflict"
    );
    let duplicate = dispatch(
        &production,
        "folders-add-again",
        "workspace.addFolder",
        json!({"workspaceId":"workspace-1","path":second}),
    )
    .await;
    assert_eq!(
        duplicate.result.error.expect("duplicate").code,
        "workspace-ambiguous"
    );

    // A live worker holds its workspace quiescence lock shared; removal
    // narrows every later launch and therefore waits for quiescence.
    let quiescence = {
        use sha2::Digest;
        let digest = sha2::Sha256::digest(b"workspace-1");
        production
            .root
            .path()
            .join(format!(".workspace-quiescence-{digest:x}.lock"))
    };
    let worker = store::NamedLock::shared(&quiescence).expect("worker quiescence lock");
    let busy = dispatch(
        &production,
        "folders-remove-busy",
        "workspace.removeFolder",
        json!({"workspaceId":"workspace-1","path":second}),
    )
    .await;
    assert_eq!(busy.result.error.expect("busy").code, "workspace-busy");
    drop(worker);

    let removed = success_value(
        &dispatch(
            &production,
            "folders-remove",
            "workspace.removeFolder",
            json!({"workspaceId":"workspace-1","path":second}),
        )
        .await,
    );
    assert_eq!(
        removed["folders"],
        json!({"format":1,"workspaceId":"workspace-1","revision":3,
            "folders":[{"folderId":"folder-0001","path":primary}]})
    );
    let last = dispatch(
        &production,
        "folders-remove-last",
        "workspace.removeFolder",
        json!({"workspaceId":"workspace-1","path":primary}),
    )
    .await;
    assert_eq!(
        last.result.error.expect("last folder").code,
        "workspace-last-folder"
    );
    let missing = dispatch(
        &production,
        "folders-missing",
        "workspace.listFolders",
        json!({"workspaceId":"workspace-absent"}),
    )
    .await;
    assert_eq!(
        missing.result.error.expect("absent").code,
        "workspace-not-found"
    );
}
