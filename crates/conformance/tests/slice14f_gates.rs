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

#[tokio::test]
async fn bench_165_create_and_preset_multi_root() {
    let f = fixture();
    let workspace = tempfile::tempdir().unwrap();
    let cwd = workspace.path().join("cwd");
    let writable_in_cwd = cwd.join("writable-a");
    let read1 = workspace.path().join("read-1");
    let writable_in_read1 = read1.join("writable-b");
    let read2 = workspace.path().join("read-2");
    let read3 = workspace.path().join("read-3");
    let read4 = workspace.path().join("read-4");
    let tc1 = workspace.path().join("toolchain-1");
    let tc2 = workspace.path().join("toolchain-2");
    for dir in [
        &cwd,
        &writable_in_cwd,
        &read1,
        &writable_in_read1,
        &read2,
        &read3,
        &read4,
        &tc1,
        &tc2,
    ] {
        std::fs::create_dir_all(dir).unwrap();
    }
    let exec = json!({
        "cwd": cwd,
        "read_roots": [read1, read2, read3, read4],
        "writable_roots": [writable_in_cwd, writable_in_read1],
        "toolchain_roots": [tc1, tc2],
        "allowed_tools": ["read", "edit"],
    });

    profile::reset_execution_metrics();
    tekes_supervisor::endpoint_host::reset_endpoint_execution_parses();
    let create_payload = json!({"execution": exec.clone()});
    f.host
        .validate_request(&request(
            "bench-create",
            "session.create",
            create_payload.clone(),
        ))
        .unwrap();
    let created = value(dispatch(&f, "bench-create", "session.create", create_payload).await);
    assert!(created["sessionId"].as_str().is_some());
    let (create_norm, create_canon) = profile::execution_metrics();
    let create_parses = tekes_supervisor::endpoint_host::endpoint_execution_parses();
    println!(
        "bench_165 session.create (9 paths): normalize_calls={create_norm}, canonicalize_calls={create_canon}, execution_parses={create_parses}"
    );

    profile::reset_execution_metrics();
    tekes_supervisor::client_extensions::reset_preset_execution_parses();
    let resolve_payload = json!({
        "execution": exec,
        "preset": {"tools": ["read"]}
    });
    f.host
        .validate_request(&request(
            "bench-resolve",
            "session.presets.resolve",
            resolve_payload.clone(),
        ))
        .unwrap();
    let resolved = value(
        dispatch(
            &f,
            "bench-resolve",
            "session.presets.resolve",
            resolve_payload,
        )
        .await,
    );
    assert_eq!(resolved["tools"], json!(["read"]));
    let (resolve_norm, resolve_canon) = profile::execution_metrics();
    let resolve_parses = tekes_supervisor::client_extensions::preset_execution_parses();
    println!(
        "bench_165 session.presets.resolve (9 paths): normalize_calls={resolve_norm}, canonicalize_calls={resolve_canon}, preset_execution_parses={resolve_parses}"
    );
}

#[tokio::test]
async fn single_pass_execution_preserves_canonicalization_validation_idempotency_and_reopen_checks()
{
    let f = fixture();
    let workspace = tempfile::tempdir().unwrap();
    let real_cwd = workspace.path().join("real-cwd");
    let real_read = workspace.path().join("real-read");
    let real_writable = real_read.join("sub-write");
    let real_tc = workspace.path().join("real-tc");
    for d in [&real_cwd, &real_read, &real_writable, &real_tc] {
        std::fs::create_dir_all(d).unwrap();
    }
    let link_cwd = workspace.path().join("link-cwd");
    let link_read = workspace.path().join("link-read");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&real_cwd, &link_cwd).unwrap();
        std::os::unix::fs::symlink(&real_read, &link_read).unwrap();
    }
    let canon_cwd = real_cwd.canonicalize().unwrap();
    let canon_read = real_read.canonicalize().unwrap();
    let canon_writable = real_writable.canonicalize().unwrap();
    let canon_tc = real_tc.canonicalize().unwrap();

    // 4 paths supplied through symlinks.
    let exec = json!({
        "cwd": if cfg!(unix) { link_cwd.as_path() } else { real_cwd.as_path() },
        "read_roots": [if cfg!(unix) { link_read.as_path() } else { real_read.as_path() }],
        "writable_roots": [real_writable],
        "toolchain_roots": [real_tc],
        "allowed_tools": ["edit", "read", "read"],
    });

    profile::reset_execution_metrics();
    tekes_supervisor::endpoint_host::reset_endpoint_execution_parses();
    let create_payload = json!({"execution": exec.clone()});
    f.host
        .validate_request(&request(
            "sp-create-1",
            "session.create",
            create_payload.clone(),
        ))
        .unwrap();
    let created1 =
        value(dispatch(&f, "sp-create-1", "session.create", create_payload.clone()).await);
    assert_eq!(profile::execution_metrics(), (1, 4));
    assert_eq!(
        tekes_supervisor::endpoint_host::endpoint_execution_parses(),
        1
    );

    // Exact rpcId idempotency returns same sessionId.
    f.host
        .validate_request(&request(
            "sp-create-1",
            "session.create",
            create_payload.clone(),
        ))
        .unwrap();
    let created2 = value(dispatch(&f, "sp-create-1", "session.create", create_payload).await);
    assert_eq!(created1["sessionId"], created2["sessionId"]);

    let sid = created1["sessionId"].as_str().unwrap();
    let ledger = f.root.path().join("threads").join(sid).join("main.jsonl");
    let genesis_line = std::fs::read_to_string(&ledger)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_owned();
    let genesis: Value = serde_json::from_str(&genesis_line).unwrap();
    assert_eq!(
        genesis["execution"]["cwd"],
        canon_cwd.to_string_lossy().as_ref()
    );
    assert_eq!(
        genesis["execution"]["read_roots"],
        json!([canon_read.to_string_lossy()])
    );
    assert_eq!(
        genesis["execution"]["writable_roots"],
        json!([canon_writable.to_string_lossy()])
    );
    assert_eq!(
        genesis["execution"]["toolchain_roots"],
        json!([canon_tc.to_string_lossy()])
    );
    assert_eq!(
        genesis["execution"]["allowed_tools"],
        json!(["edit", "read"])
    );

    // Preset resolve runs single parse/normalize/canonicalize pass.
    profile::reset_execution_metrics();
    tekes_supervisor::client_extensions::reset_preset_execution_parses();
    let resolve_payload = json!({"execution": exec, "preset": {"tools": ["read"]}});
    f.host
        .validate_request(&request(
            "sp-res-1",
            "session.presets.resolve",
            resolve_payload.clone(),
        ))
        .unwrap();
    let resolved =
        value(dispatch(&f, "sp-res-1", "session.presets.resolve", resolve_payload).await);
    assert_eq!(resolved["tools"], json!(["read"]));
    assert_eq!(profile::execution_metrics(), (1, 4));
    assert_eq!(
        tekes_supervisor::client_extensions::preset_execution_parses(),
        1
    );

    // Protected storage root, nonexistent root, and writable outside read roots fail with exact codes.
    let outside = tempfile::tempdir().unwrap();
    for (idx, bad_exec) in [
        json!({"cwd": f.root.path()}),
        json!({"cwd": workspace.path().join("does-not-exist")}),
        json!({"cwd": canon_cwd, "writable_roots": [outside.path()]}),
    ]
    .into_iter()
    .enumerate()
    {
        let p = json!({"execution": bad_exec.clone()});
        f.host
            .validate_request(&request(
                &format!("bad-c-{idx}"),
                "session.create",
                p.clone(),
            ))
            .unwrap();
        let resp = dispatch(&f, &format!("bad-c-{idx}"), "session.create", p).await;
        assert!(!resp.result.ok);
        assert_eq!(resp.result.error.unwrap().code, "bad-request");

        let rp = json!({"execution": bad_exec, "preset": {}});
        f.host
            .validate_request(&request(
                &format!("bad-r-{idx}"),
                "session.presets.resolve",
                rp.clone(),
            ))
            .unwrap();
        let rresp = dispatch(&f, &format!("bad-r-{idx}"), "session.presets.resolve", rp).await;
        assert!(!rresp.result.ok);
        assert_eq!(rresp.result.error.unwrap().code, "bad-request");
    }

    // Reopen check (`resolve_execution`) STILL re-verifies canonicality on disk!
    #[cfg(unix)]
    {
        let scope = profile::SessionScope::locate(f.root.path(), sid).unwrap();
        let repo = profile::ConfigRepository::open(f.root.path()).unwrap();
        assert!(
            repo.resolve_execution(sid, scope.execution(), Some(scope.session_folder()))
                .is_ok()
        );

        let moved_target = workspace.path().join("moved-cwd");
        std::fs::create_dir_all(&moved_target).unwrap();
        std::fs::remove_dir_all(&canon_cwd).unwrap();
        std::os::unix::fs::symlink(&moved_target, &canon_cwd).unwrap();
        assert!(
            repo.resolve_execution(sid, scope.execution(), Some(scope.session_folder()))
                .is_err(),
            "reopen resolve_execution must fail when a recorded canonical root becomes a symlink"
        );
    }
}
