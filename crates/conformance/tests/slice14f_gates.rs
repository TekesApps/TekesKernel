//! 0.9 capability negotiation and Session lifecycle. Removed host management
//! is tested as unavailable; there is no hidden legacy assembly fixture.
use endpoint::{
    ClientRequest, EndpointDispatcher, EndpointHost, RpcRegistry, SessionHostDescription,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::sync::Arc;
use tekes_supervisor::endpoint_host::{ProductionEndpointHost, TEKES_UNARY_ROUTES};
use tekes_supervisor::process_host::ProductionProcessHost;

struct Fixture {
    root: tempfile::TempDir,
    host: ProductionEndpointHost,
    process: Arc<ProductionProcessHost>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.process.shutdown();
    }
}
fn fixture() -> Fixture {
    let root = tempfile::tempdir().unwrap();
    let agent = root.path().join("host-instructions");
    std::fs::create_dir(&agent).unwrap();
    let process = ProductionProcessHost::open(
        root.path(),
        std::env::current_exe().unwrap(),
        env!("CARGO_PKG_VERSION"),
        &agent,
    )
    .unwrap();
    let host = tekes_supervisor::host_runtime::assemble_application_endpoint_host(
        root.path(),
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").into(),
            cwd: root.path().display().to_string(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: root.path().display().to_string(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-10-16T00:00:00.000Z".into())),
        &agent,
        process.clone(),
    )
    .unwrap();
    Fixture {
        root,
        host,
        process,
    }
}
fn request(id: &str, method: &str, payload: Value) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".into(),
        rpc_id: id.into(),
        method: method.into(),
        payload: IJsonValue::parse(&serde_json::to_vec(&payload).unwrap()).unwrap(),
    }
}
async fn dispatch(f: &Fixture, id: &str, method: &str, payload: Value) -> endpoint::ServerResponse {
    EndpointDispatcher::new(RpcRegistry::open(f.root.path()).unwrap())
        .dispatch(&f.host, request(id, method, payload))
        .await
        .unwrap()
}
fn value(response: endpoint::ServerResponse) -> Value {
    assert!(response.result.ok, "{response:?}");
    serde_json::from_slice(&response.result.value.unwrap().canonical_bytes().unwrap()).unwrap()
}
#[test]
fn catalog_contains_exactly_the_production_session_capabilities() {
    let f = fixture();
    let catalog: Value = serde_json::from_slice(
        &std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../fixtures/client-extensions/catalog.canonical.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let methods = catalog["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|group| group["methods"].as_array().unwrap())
        .map(|method| method["name"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(methods, f.host.extension_capabilities());
    let base = catalog["base"]
        .as_array()
        .unwrap()
        .iter()
        .map(|method| method.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        base,
        TEKES_UNARY_ROUTES
            .iter()
            .map(|route| route.name.to_owned())
            .chain(["remote.mux".into()])
            .collect()
    );
}
#[tokio::test]
async fn retired_management_is_unavailable_and_cannot_change_storage() {
    let f = fixture();
    for (index, method) in [
        "workspace.create",
        "workspace.rename",
        "workspace.relocate",
        "environments.save",
        "folders.list",
        "folders.relocate",
        "session.folders.add",
        "folder.trust.grant",
        "mcp.list",
        "mcp.save",
        "mcp.oauth.start",
        "plugin/install",
        "schedule.save",
        "directory.create",
    ]
    .into_iter()
    .enumerate()
    {
        assert!(!f.host.capabilities().contains(method));
        let response = dispatch(&f, &format!("retired-{index}"), method, json!({})).await;
        assert!(!response.result.ok, "{method}");
        assert_eq!(
            response.result.error.unwrap().code,
            "unsupported-capability",
            "{method}"
        );
    }
    for path in [
        "workspaces",
        "endpoint-management",
        "plugins",
        "schedule",
        "config/folders.json",
        "config/mcp-servers.json",
        "config/environments.json",
    ] {
        assert!(!f.root.path().join(path).exists(), "{path}");
    }
}
#[tokio::test]
async fn execution_create_replay_archive_restore_and_discard_survive_recovery() {
    let f = fixture();
    let project = tempfile::tempdir().unwrap();
    let created = value(
        dispatch(
            &f,
            "create",
            "session.create",
            json!({"execution":{"cwd":project.path(),"allowed_tools":["read"]}}),
        )
        .await,
    );
    assert_eq!(
        created,
        value(
            dispatch(
                &f,
                "create",
                "session.create",
                json!({"execution":{"cwd":project.path(),"allowed_tools":["read"]}})
            )
            .await
        )
    );
    let id = created["sessionId"].as_str().unwrap();
    value(dispatch(&f, "archive", "session.archive", json!({"sessionId":id})).await);
    assert!(f.root.path().join("archive").join(id).is_dir());
    value(dispatch(&f, "restore", "session.unarchive", json!({"sessionId":id})).await);
    assert!(f.root.path().join("threads").join(id).is_dir());
    let ledger = f.root.path().join("threads").join(id).join("main.jsonl");
    let mut bytes = std::fs::read(&ledger).unwrap();
    for event in [
        json!({"v":1,"seq":2,"ts":"2026-10-17T00:00:00.000Z","kind":"input","content":[{"type":"text","text":"fork input"}],"origin_key":"fork-input","origin_tuple":{"principal":"test","client":"test","target":id,"op":"submit","key":"fork-input"}}),
        json!({"v":1,"seq":3,"ts":"2026-10-17T00:00:00.000Z","kind":"turn_open","turn":1,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":4,"ts":"2026-10-17T00:00:00.000Z","kind":"settle","turn":1,"outcome":"completed"}),
    ] {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).unwrap());
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, schema::READER_VERSION).unwrap();
    std::fs::write(ledger, bytes).unwrap();
    let fork = value(
        dispatch(
            &f,
            "scratch",
            "session.fork",
            json!({"sessionId":id,"ephemeral":true}),
        )
        .await,
    );
    let scratch = fork["sessionId"].as_str().unwrap();
    value(
        dispatch(
            &f,
            "discard",
            "session.discard",
            json!({"sessionId":scratch}),
        )
        .await,
    );
    endpoint::SessionOperations::open_sessions(f.root.path())
        .unwrap()
        .recover_sessions(None, None)
        .unwrap();
    assert!(!f.root.path().join("threads").join(scratch).exists());
}
#[tokio::test]
async fn old_creation_inputs_are_rejected_without_creating_sessions() {
    let f = fixture();
    let project = tempfile::tempdir().unwrap();
    for (index, payload) in [
        json!({"cwd":project.path()}),
        json!({"workspaceId":"old"}),
        json!({"execution":{"cwd":project.path()},"environment":"local"}),
        json!({"execution":{"cwd":project.path(),"trusted":true}}),
    ]
    .into_iter()
    .enumerate()
    {
        let result = f.host.validate_request(&request(
            &format!("invalid-{index}"),
            "session.create",
            payload,
        ));
        assert!(result.is_err());
    }
    assert!(
        std::fs::read_dir(f.root.path().join("threads"))
            .unwrap()
            .next()
            .is_none()
    );
}
