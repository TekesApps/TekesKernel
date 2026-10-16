use std::future::Future;
use std::task::{Context, Poll, Waker};

use endpoint::{ClientRequest, SessionHostDescription};
use schema::IJsonValue;
use serde_json::{Value, json};
use tekes_supervisor::endpoint_host::ProductionEndpointAssembly;

const SESSION: &str = "018f0000-0000-7000-8000-000000000081";

fn ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("unexpected pending operation"),
    }
}

fn open(root: &std::path::Path) -> ProductionEndpointAssembly {
    ProductionEndpointAssembly::open(
        root,
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            cwd: root.to_string_lossy().into_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: root.to_string_lossy().into_owned(),
            can_open_path: false,
        },
    )
    .unwrap()
}

fn request(id: &str, payload: Value) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: id.to_owned(),
        method: "session.create".to_owned(),
        payload: IJsonValue::parse(&serde_json::to_vec(&payload).unwrap()).unwrap(),
    }
}

#[test]
fn create_replay_reopen_and_inventory_ignore_retired_registries() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    // These must not even be parsed by boot, create, inventory or launch scope.
    for relative in [
        "config/environments.json",
        "config/folders.json",
        "workspaces/broken/workspace.json",
        "endpoint-management/operations/broken.json",
    ] {
        let path = root.path().join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"not JSON").unwrap();
    }
    std::fs::write(project.path().join("AGENTS.md"), "CREATION_PROJECT_RULE").unwrap();
    std::fs::create_dir_all(project.path().join(".agent/hooks")).unwrap();
    std::fs::write(
        project.path().join(".agent/hooks/pre.json"),
        b"invalid executable hook must not be parsed",
    )
    .unwrap();
    let assembly = open(root.path());
    let create = request(
        "create",
        json!({"sessionId":SESSION,"execution":{"cwd":project.path(),"network":false}}),
    );
    let first = ready(assembly.dispatch(create.clone())).unwrap();
    assert!(first.result.ok, "{first:?}");
    let folder = root.path().join("threads").join(SESSION);
    let bytes = std::fs::read(folder.join("main.jsonl")).unwrap();
    let genesis: Value =
        serde_json::from_slice(bytes.split(|b| *b == b'\n').next().unwrap()).unwrap();
    assert_eq!(genesis["min_reader"], 5);
    let instruction_digest = genesis["instruction"]["digest"].as_str().unwrap();
    let snapshot = profile::InstructionSnapshot::decode(
        &std::fs::read(
            folder
                .join("assets")
                .join(format!("sha256-{instruction_digest}")),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(
        snapshot
            .sources
            .iter()
            .any(|s| s.content == "CREATION_PROJECT_RULE")
    );
    assert!(snapshot.effective.hooks.is_empty());

    for field in ["environment", "folders", "session_scope"] {
        assert!(genesis.get(field).is_none());
    }
    let repo = profile::ConfigRepository::open(root.path()).unwrap();
    let scope = profile::SessionScope::locate(root.path(), SESSION).unwrap();
    let config = scope.config(&repo).unwrap();
    assert!(!config.execution.policy.network);
    assert!(
        serde_json::to_value(&config).unwrap()["revisions"]
            .get("environments")
            .is_none()
    );
    assert!(scope.execution().mcp.is_empty());
    let inventory = endpoint::NativeEndpoint::open(root.path())
        .unwrap()
        .list_sessions(&Default::default())
        .unwrap();
    assert_eq!(inventory.len(), 1);
    assert_eq!(inventory[0].cwd, config.execution_cwd().unwrap());
    drop(assembly);
    // Exact retries use staged bytes even when the original cwd is gone.
    project.close().unwrap();
    let reopened = open(root.path());
    assert!(ready(reopened.dispatch(create)).unwrap().result.ok);
    assert_eq!(bytes, std::fs::read(folder.join("main.jsonl")).unwrap());
    assert!(
        scope.config(&repo).is_err(),
        "a new launch must reject the missing cwd"
    );
}

#[test]
fn management_inputs_and_invalid_execution_have_no_session_side_effects() {
    let root = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let assembly = open(root.path());
    for (index, payload) in [
        json!({"workspaceId":"legacy"}),
        json!({"cwd":project.path()}),
        json!({"execution":{"cwd":project.path()},"environment":"local"}),
        json!({"execution":{"cwd":"relative"}}),
        json!({"execution":{"cwd":project.path(),"trusted":true}}),
        json!({"execution":{"cwd":root.path()}}),
        json!({"execution":{"cwd":project.path(),"max_wall_seconds":0}}),
    ]
    .into_iter()
    .enumerate()
    {
        let response = ready(assembly.dispatch(request(&format!("invalid-{index}"), payload)));
        assert!(
            response.is_err() || !response.unwrap().result.ok,
            "invalid create accepted"
        );
    }
    assert!(
        std::fs::read_dir(root.path().join("threads"))
            .unwrap()
            .next()
            .is_none()
    );
    assert!(!root.path().join("config/folders.json").exists());
}
