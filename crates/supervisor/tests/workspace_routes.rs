use endpoint::{DurableHandoffSignal, EndpointHostCall, ManagementStore};
use schema::IJsonValue;
use serde_json::{Value, json};
use tekes_supervisor::endpoint_host::ProductionEndpointRoutes;
use tekes_supervisor::workspace_routes::WorkspaceRoutes;

fn request(method: &str, payload: Value, recovering: bool) -> EndpointHostCall {
    EndpointHostCall {
        rpc_id: format!("workspace-test-{method}"),
        operation: format!("tekesWorkspace.{method}"),
        payload: IJsonValue::parse(&serde_json::to_vec(&payload).unwrap()).unwrap(),
        recovering,
        handoff: DurableHandoffSignal::new(),
    }
}
fn invoke(routes: &WorkspaceRoutes, method: &str, payload: Value) -> Value {
    let request = request(method, payload.clone(), false);
    serde_json::to_value(
        routes
            .execute(&request, &payload, "test-principal")
            .unwrap(),
    )
    .unwrap()
}

#[test]
#[ignore = "requires TEKES_WORKSPACE_SERVICE_BIN built from the current workspace"]
fn production_routes_invoke_real_helper_and_enforce_workspace_authority() {
    let helper = std::path::PathBuf::from(
        std::env::var_os("TEKES_WORKSPACE_SERVICE_BIN").expect("current helper binary"),
    );
    let storage = tempfile::tempdir().unwrap();
    let project = tempfile::tempdir().unwrap();
    let management = ManagementStore::open(storage.path()).unwrap();
    let (workspace, _) = management
        .create_workspace(
            "create",
            &"a".repeat(64),
            project.path().to_str().unwrap(),
            "2026-09-08T00:00:00.000Z",
        )
        .unwrap();
    std::fs::write(project.path().join("proof.txt"), "hello\n").unwrap();
    let routes = WorkspaceRoutes::open(storage.path(), helper).unwrap();
    let capabilities = invoke(&routes, "capabilities", json!({}));
    assert_eq!(capabilities["mutations"], false);
    let methods = capabilities["methods"].as_array().unwrap();
    assert!(methods.contains(&json!("filesRead")));
    assert!(!methods.contains(&json!("gitCommit")));
    assert!(!methods.contains(&json!("gitPush")));
    assert!(!methods.contains(&json!("gitChangeBranch")));
    let files = invoke(
        &routes,
        "filesList",
        json!({"workspaceId":workspace.workspace_id}),
    );
    assert_eq!(files["entries"][0]["name"], "proof.txt");
    let read = invoke(
        &routes,
        "filesRead",
        json!({"workspaceId":workspace.workspace_id,"path":"proof.txt"}),
    );
    assert_eq!(read["content"], "aGVsbG8K");
    let unknown = json!({"workspaceId":"missing"});
    assert_eq!(
        routes
            .execute(
                &request("filesList", unknown.clone(), false),
                &unknown,
                "test"
            )
            .unwrap_err()
            .code,
        "workspace-missing"
    );
    let injected = json!({"workspaceId":workspace.workspace_id,"root":"/","shell":"pwd"});
    assert!(
        routes
            .validate_extension_payload("tekesWorkspace.filesList", &injected)
            .is_err()
    );
    assert!(
        !routes
            .capabilities()
            .contains("tekesWorkspace.recordTurnEdit")
    );
    let session = json!({"workspaceId":workspace.workspace_id,"sessionId":"other","turnId":"1"});
    assert_eq!(
        routes
            .execute(
                &request("turnChanges", session.clone(), false),
                &session,
                "test"
            )
            .unwrap_err()
            .code,
        "session-outside-workspace"
    );
    let mutation =
        json!({"workspaceId":workspace.workspace_id,"operation":"create","name":"feature"});
    assert!(
        std::process::Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(project.path())
            .output()
            .unwrap()
            .status
            .success()
    );
    assert_eq!(
        routes
            .execute(
                &request("gitChangeBranch", mutation.clone(), false),
                &mutation,
                "test"
            )
            .unwrap_err()
            .code,
        "permission-denied"
    );
    let policy = storage.path().join("workspace-service/git-authority.json");
    std::fs::create_dir_all(policy.parent().unwrap()).unwrap();
    std::fs::write(policy,serde_json::to_vec(&json!({"allowedOperations":["branch.create"],"allowedRepositoryRoots":[project.path()]})).unwrap()).unwrap();
    let branch = invoke(&routes, "gitChangeBranch", mutation.clone());
    assert_eq!(branch["branch"], "feature");
    assert_eq!(
        routes
            .execute(
                &request("gitChangeBranch", mutation.clone(), true),
                &mutation,
                "test"
            )
            .unwrap_err()
            .code,
        "outcome-unknown"
    );
}
