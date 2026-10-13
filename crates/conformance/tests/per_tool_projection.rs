//! mcp-runtime §Catalogs and client-extensions §Workspace administration: a tool
//! outside the dynamic-schema contract is refused by itself. Its server stays
//! healthy, the policy naming it is accepted and reports it, and everything that
//! was already refused keeps being refused. Names below are generated; no real
//! tool surface appears in this file.

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

use endpoint::{ClientRequest, EndpointDispatcher, RpcRegistry, SessionHostDescription};
use profile::{ConfigRepository, WorkspaceConfig, WorkspacePolicy};
use schema::IJsonValue;
use serde_json::{Value, json};
use tekes_supervisor::endpoint_host::ProductionEndpointHost;
use tekes_supervisor::host_runtime::assemble_application_endpoint_host;
use tekes_supervisor::process_host::ProductionProcessHost;
use tempfile::TempDir;

struct Fixture {
    root: TempDir,
    host: ProductionEndpointHost,
    process: Arc<ProductionProcessHost>,
    next: std::cell::Cell<u32>,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.process.shutdown();
    }
}

fn fixture() -> Fixture {
    let root = TempDir::new().expect("root");
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
    Fixture {
        root,
        host,
        process,
        next: std::cell::Cell::new(0),
    }
}

impl Fixture {
    async fn call(&self, method: &str, payload: Value) -> endpoint::ServerResponse {
        let id = self.next.get();
        self.next.set(id + 1);
        let request = ClientRequest {
            envelope_type: "client-request".to_owned(),
            rpc_id: format!("rpc-{id}"),
            method: method.to_owned(),
            payload: IJsonValue::parse(&serde_json::to_vec(&payload).expect("request JSON"))
                .expect("request I-JSON"),
        };
        EndpointDispatcher::new(RpcRegistry::open(self.root.path()).expect("rpc registry"))
            .dispatch(&self.host, request)
            .await
            .expect("endpoint response")
    }

    async fn ok(&self, method: &str, payload: Value) -> Value {
        let response = self.call(method, payload).await;
        assert!(response.result.ok, "{method}: {response:?}");
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

    /// The failure code and `details` of a refused call.
    async fn refused(&self, method: &str, payload: Value) -> (String, String, Value) {
        let response = self.call(method, payload).await;
        assert!(!response.result.ok, "{method} unexpectedly succeeded");
        let error = response.result.error.expect("error");
        let details =
            serde_json::from_slice(&error.details.canonical_bytes().expect("canonical details"))
                .expect("details JSON");
        (error.code, error.message, details)
    }

    fn script(&self, name: &str, body: &str) -> String {
        let path = self.root.path().join(name);
        fs::write(&path, body).expect("server script");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("script mode");
        path.to_string_lossy().into_owned()
    }

    async fn save_server(&self, name: &str, command: &str) {
        self.ok(
            "mcp.save",
            json!({"credentialFields":{},"server":{
                "always_on":false,"enabled":true,"project_trusted":true,"protocol_mode":"legacy",
                "reference":{"name":name,"scope":"user","workspace_id":"workspace-1"},
                "transport":{"command":[command],"environment":{},"kind":"stdio"}}}),
        )
        .await;
    }

    async fn policy(&self, revision: u64, tools: &[&str]) -> (u64, Value) {
        (
            revision,
            json!({"workspaceId":"workspace-1","expectedRevision":revision,
                "policy":{"allowed_tools":tools,"network":false,"writable_roots":[]}}),
        )
    }
}

fn catalog_server(tools_json: &str) -> String {
    format!(
        "#!/bin/sh\nwhile IFS= read -r line; do\ncase \"$line\" in\n*'\"method\":\"initialize\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"protocolVersion\":\"{}\",\"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"catalog\",\"version\":\"1\"}}}}}}' ;;\n*'\"method\":\"tools/list\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{{\"tools\":{tools_json}}}}}' ;;\nesac\ndone\n",
        mcp::LEGACY_PROTOCOL_VERSION
    )
}

const GOOD_TOOL: &str = r#"{"name":"alpha","description":"Good tool","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}}"#;
const BAD_TOOL: &str = r#"{"name":"beta","description":"Bad tool","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"operand":{"type":"object","minProperties":1,"maxProperties":2,"properties":{"left":{"type":"string"}},"additionalProperties":false}},"required":["operand"],"additionalProperties":false}}"#;

fn tool_names(listed: &Value) -> Vec<String> {
    listed["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .filter_map(|row| row["name"].as_str())
        .filter(|name| name.starts_with("mcp__"))
        .map(str::to_owned)
        .collect()
}

#[tokio::test]
async fn one_refused_tool_leaves_the_server_and_the_policy_usable() {
    let fixture = fixture();
    let script = fixture.script(
        "probe-server",
        &catalog_server(&format!("[{GOOD_TOOL},{BAD_TOOL}]")),
    );
    fixture.save_server("probe", &script).await;

    // The policy names the good and the refused tool: accepted, and the refused
    // one is reported with its server, pointer, keyword and reason.
    let (_, request) = fixture
        .policy(1, &["mcp__probe__alpha", "mcp__probe__beta"])
        .await;
    let set = fixture.ok("workspace.policy.set", request).await;
    assert_eq!(set["revision"], 2);
    let unavailable = set["unavailableTools"]
        .as_array()
        .expect("unavailable tools");
    assert_eq!(unavailable.len(), 1, "{set}");
    let refused = &unavailable[0];
    assert_eq!(refused["server"], "probe");
    assert_eq!(refused["tool"], "beta");
    assert_eq!(refused["name"], "mcp__probe__beta");
    assert_eq!(refused["code"], "schema-refused");
    assert_eq!(refused["pointer"], "/parameters/properties/operand");
    assert_eq!(refused["keyword"], "maxProperties");
    assert!(
        refused["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("unsupported keyword")),
        "{refused}"
    );
    assert!(set.get("unavailableToolCount").is_none());
    // The stored policy is what was asked for.
    let stored = fixture
        .ok("workspace.policy.get", json!({"workspaceId":"workspace-1"}))
        .await;
    assert_eq!(
        stored["policy"]["allowed_tools"],
        json!(["mcp__probe__alpha", "mcp__probe__beta"])
    );

    // The good tool is in the client catalog; the refused one is not, and
    // resolving it is `tool-not-found`: not callable.
    let listed = fixture
        .ok("tools/list", json!({"workspaceId":"workspace-1"}))
        .await;
    assert_eq!(tool_names(&listed), vec!["mcp__probe__alpha".to_owned()]);
    let (code, _, _) = fixture
        .refused(
            "tools/resolve",
            json!({"workspaceId":"workspace-1","name":"mcp__probe__beta"}),
        )
        .await;
    assert_eq!(code, "tool-not-found");

    // The server is ready, not failed, and its status lists the refusal.
    let servers = fixture
        .ok("mcp.list", json!({"workspaceId":"workspace-1"}))
        .await;
    let row = servers["servers"]
        .as_array()
        .and_then(|rows| {
            rows.iter()
                .find(|row| row["server"]["reference"]["name"] == "probe")
        })
        .expect("probe row");
    assert_eq!(row["readiness"]["state"], "ready", "{row}");
    assert!(row["readiness"].get("code").is_none());
    assert_eq!(row["readiness"]["refusedToolCount"], 1);
    assert_eq!(row["readiness"]["refusedTools"][0]["tool"], "beta");
    assert_eq!(
        row["readiness"]["refusedTools"][0]["keyword"],
        "maxProperties"
    );

    // A policy that only names the good tool reports nothing unavailable.
    let (_, request) = fixture.policy(2, &["mcp__probe__alpha"]).await;
    let set = fixture.ok("workspace.policy.set", request).await;
    assert!(set.get("unavailableTools").is_none(), "{set}");
}

#[tokio::test]
async fn unknown_names_and_unavailable_servers_are_still_refused_with_their_own_cause() {
    let fixture = fixture();
    let script = fixture.script(
        "probe-server",
        &catalog_server(&format!("[{GOOD_TOOL},{BAD_TOOL}]")),
    );
    fixture.save_server("probe", &script).await;
    let offline = fixture.script("offline-server", "#!/bin/sh\nexit 1\n");
    fixture.save_server("offline", &offline).await;

    // A name no catalog has is not excused by a neighbouring refusal.
    let (_, request) = fixture
        .policy(1, &["mcp__probe__beta", "mcp__probe__gamma"])
        .await;
    let (code, message, details) = fixture.refused("workspace.policy.set", request).await;
    assert_eq!(code, "policy-escalation");
    assert_eq!(
        message,
        "Workspace policy names a tool that is not installed"
    );
    assert_eq!(details["cause"], "tool-not-installed");
    let reason = details["reason"].as_str().expect("reason");
    assert!(reason.contains("mcp__probe__gamma"), "{reason}");
    assert!(!reason.contains("mcp__probe__beta"), "{reason}");

    // The refused name on a different server is likewise unknown.
    let (_, request) = fixture.policy(1, &["mcp__other__beta"]).await;
    let (code, _, details) = fixture.refused("workspace.policy.set", request).await;
    assert_eq!(code, "policy-escalation");
    assert_eq!(details["cause"], "tool-not-installed");

    // A server that fails as a whole and is named by the policy still fails
    // the policy, with a precise label rather than "authority ceiling".
    let (_, request) = fixture
        .policy(1, &["mcp__probe__alpha", "mcp__offline__anything"])
        .await;
    let (code, message, details) = fixture.refused("workspace.policy.set", request).await;
    assert_eq!(code, "policy-escalation");
    assert_eq!(
        message,
        "Workspace policy requires an MCP server that is unavailable"
    );
    assert_eq!(details["cause"], "mcp-server-unavailable");
    let reason = details["reason"].as_str().expect("reason");
    assert!(reason.contains("offline (refresh-failed)"), "{reason}");

    // The same offline server does not block a policy that does not name it,
    // and its refusal-free sibling keeps working.
    let (_, request) = fixture.policy(1, &["mcp__probe__alpha"]).await;
    fixture.ok("workspace.policy.set", request).await;
    let servers = fixture
        .ok("mcp.list", json!({"workspaceId":"workspace-1"}))
        .await;
    let states = servers["servers"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| {
            (
                row["server"]["reference"]["name"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                row["readiness"]["state"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(states["probe"], "ready");
    assert_eq!(states["offline"], "refresh-failed");
}

#[tokio::test]
async fn a_policy_beyond_a_ceiling_keeps_the_escalation_code_and_label() {
    let fixture = fixture();
    // The instruction ceiling check is unchanged: network and a root outside
    // the workspace are an escalation with the precise cause in details.
    let (_, request) = fixture.policy(1, &["shell"]).await;
    let mut escalated = request;
    escalated["policy"]["network"] = json!(true);
    escalated["policy"]["writable_roots"] = json!(["/"]);
    let (code, _, details) = fixture.refused("workspace.policy.set", escalated).await;
    assert_eq!(code, "policy-escalation");
    assert!(details["reason"].is_string(), "{details}");
    assert_eq!(details["cause"], "authority-ceiling");
    assert!(
        details["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("writable root")),
        "{details}"
    );
}

fn tool_json(name: &str, refused: bool) -> String {
    let parameters = if refused {
        r#"{"type":"object","properties":{"o":{"type":"object","maxProperties":2,"properties":{}}},"required":[],"additionalProperties":false}"#
    } else {
        r#"{"type":"object","properties":{},"required":[],"additionalProperties":false}"#
    };
    format!(
        r#"{{"name":"{name}","description":"Fixture","annotations":{{"readOnlyHint":true}},"inputSchema":{parameters}}}"#
    )
}

/// A refused tool whose projected name is longer than a launch tool name (128
/// bytes) can never be an executable or an "unavailable" tool: a policy naming
/// it is an unknown tool, so `set` refuses it and the workspace keeps working.
/// (Before, `set` accepted it and every `tools/list` and launch then failed.)
#[tokio::test]
async fn a_refused_tool_with_an_overlong_projected_name_is_unknown_not_unavailable() {
    let fixture = fixture();
    let long = "x".repeat(120);
    let script = fixture.script(
        "probe-server",
        &catalog_server(&format!("[{GOOD_TOOL},{}]", tool_json(&long, true))),
    );
    fixture.save_server("probe", &script).await;
    let long_name = format!("mcp__probe__{long}");
    let (_, request) = fixture
        .policy(1, &["mcp__probe__alpha", long_name.as_str()])
        .await;
    let (code, _, details) = fixture.refused("workspace.policy.set", request).await;
    assert_eq!(code, "policy-escalation");
    assert_eq!(details["cause"], "tool-not-installed");

    // The refusal is still reported by the server status, with a bounded diagnostic.
    let servers = fixture
        .ok("mcp.list", json!({"workspaceId":"workspace-1"}))
        .await;
    let refused = &servers["servers"][0]["readiness"]["refusedTools"][0];
    assert_eq!(refused["code"], "schema-refused", "{servers}");
    assert!(refused.get("name").is_none(), "{refused}");
    assert!(
        refused["tool"]
            .as_str()
            .is_some_and(|tool| tool.len() <= 256)
    );

    let (_, request) = fixture.policy(1, &["mcp__probe__alpha"]).await;
    fixture.ok("workspace.policy.set", request).await;
    let listed = fixture
        .ok("tools/list", json!({"workspaceId":"workspace-1"}))
        .await;
    assert_eq!(tool_names(&listed), vec!["mcp__probe__alpha".to_owned()]);
}

/// `a`/`b__c` and `a__b`/`c` escape to the same projected name. The healthy
/// one runs; the refused one is not "unavailable" because the name is
/// executable. `set` and `tools/list` agree.
#[tokio::test]
async fn a_refused_tool_sharing_its_projected_name_with_a_healthy_tool_is_not_unavailable() {
    let fixture = fixture();
    let one = fixture.script(
        "one-server",
        &catalog_server(&format!("[{}]", tool_json("b__c", true))),
    );
    let two = fixture.script(
        "two-server",
        &catalog_server(&format!("[{}]", tool_json("c", false))),
    );
    fixture.save_server("a", &one).await;
    fixture.save_server("a__b", &two).await;
    let (_, request) = fixture.policy(1, &["mcp__a__b__c"]).await;
    let set = fixture.ok("workspace.policy.set", request).await;
    assert!(set.get("unavailableTools").is_none(), "{set}");
    let listed = fixture
        .ok("tools/list", json!({"workspaceId":"workspace-1"}))
        .await;
    assert_eq!(tool_names(&listed), vec!["mcp__a__b__c".to_owned()]);
}

/// Whatever `workspace.policy.set` accepts, the workspace's catalog still
/// lists: a policy never turns acceptance into a dead workspace.
#[tokio::test]
async fn every_policy_set_accepts_leaves_tools_list_working() {
    let long = "x".repeat(120);
    let long_name = format!("mcp__probe__{long}");
    type Scenario = (Vec<(&'static str, Vec<String>)>, Vec<String>);
    let scenarios: Vec<Scenario> = vec![
        (
            vec![(
                "probe",
                vec![tool_json("alpha", false), tool_json("beta", true)],
            )],
            vec!["mcp__probe__alpha".into(), "mcp__probe__beta".into()],
        ),
        (
            vec![(
                "probe",
                vec![tool_json("alpha", false), tool_json(&long, true)],
            )],
            vec!["mcp__probe__alpha".into(), long_name.clone()],
        ),
        (
            vec![
                ("a", vec![tool_json("b__c", true)]),
                ("a__b", vec![tool_json("c", false)]),
            ],
            vec!["mcp__a__b__c".into()],
        ),
        (
            vec![("probe", vec![tool_json("alpha", false)])],
            vec!["mcp__probe__alpha".into(), "mcp__probe__gamma".into()],
        ),
        (
            vec![("probe", vec![tool_json("beta", true)])],
            vec!["mcp__probe__beta".into(), "read".into()],
        ),
    ];
    let mut accepted = 0;
    for (servers, tools) in scenarios {
        let fixture = fixture();
        for (name, listed) in &servers {
            let script = fixture.script(
                &format!("{name}-server"),
                &catalog_server(&format!("[{}]", listed.join(","))),
            );
            fixture.save_server(name, &script).await;
        }
        let names = tools.iter().map(String::as_str).collect::<Vec<_>>();
        let (_, request) = fixture.policy(1, &names).await;
        let response = fixture.call("workspace.policy.set", request).await;
        if response.result.ok {
            accepted += 1;
            let listed = fixture
                .call("tools/list", json!({"workspaceId":"workspace-1"}))
                .await;
            assert!(
                listed.result.ok,
                "set accepted {tools:?} but tools/list failed: {:?}",
                listed.result.error
            );
        }
    }
    assert_eq!(accepted, 3, "the accepting scenarios changed");
}
