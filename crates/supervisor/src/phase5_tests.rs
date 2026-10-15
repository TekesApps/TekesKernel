//! Real stdio acceptance for owner sharing, admission barriers and cleanup.
use super::*;
use provider::MemorySecretStore;
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !condition() {
        assert!(Instant::now() < deadline, "timed out");
        std::thread::sleep(Duration::from_millis(10));
    }
}
fn server(project: &Path, script: &Path, scope: McpScope, workspace: &str) -> McpServerConfig {
    McpServerConfig {
        folder_id: None,
        reference: McpServerReference {
            workspace_id: workspace.into(),
            scope,
            name: "fixture".into(),
        },
        transport: McpTransportConfig::Stdio {
            command: vec![
                "/usr/bin/python3".into(),
                "-u".into(),
                script.display().to_string(),
            ],
            cwd: Some(project.display().to_string()),
            environment: BTreeMap::new(),
        },
        enabled: true,
        always_on: false,
        protocol_mode: ProtocolMode::Legacy,
        owner: None,
        plugin_component: None,
        project_trusted: true,
    }
}
fn script(project: &Path) -> std::path::PathBuf {
    let script = project.join("server.py");
    fs::write(&script,r#"import os,json,sys,time,subprocess
if os.path.exists('descendant'):
 child=subprocess.Popen(['/bin/sleep','30'])
 with open('child-pid','w') as f:f.write(str(child.pid))
with open('starts','a') as f:f.write(str(os.getpid())+'\n')
for line in sys.stdin:
 r=json.loads(line)
 if 'id' not in r:continue
 m=r['method']
 if m=='initialize':out={'protocolVersion':'2025-11-25','capabilities':{'tools':{},'resources':{}},'serverInfo':{'name':'fixture','version':'1'}}
 elif m=='tools/list':out={'tools':[{'name':'ping','description':'ping','annotations':{'readOnlyHint':True},'inputSchema':{'type':'object','properties':{},'required':[],'additionalProperties':False}}]}
 elif m=='tools/call':
  with open('calls','a') as f:f.write('call\n')
  if os.path.exists('block'):
   open('entered','w').close()
   while not os.path.exists('release'):time.sleep(.01)
  out={'content':[{'type':'text','text':'ok'}]}
 elif m=='resources/list':out={'resources':[{'uri':'fixture://one','name':'one'}]}
 elif m=='resources/read':out={'contents':[{'uri':'fixture://one','text':'ok'}]}
 else:raise RuntimeError(m)
 print(json.dumps({'jsonrpc':'2.0','id':r['id'],'result':out}),flush=True)
"#).unwrap();
    script
}
fn save(runtime: &McpRuntime, op: &str, server: McpServerConfig) {
    runtime
        .mutate(
            op,
            &McpManagementMutation::Save {
                server,
                credential_fields: BTreeMap::new(),
            },
        )
        .unwrap();
}
fn workspace(state: &Path, name: &str, paths: Vec<String>) {
    profile::ConfigRepository::open(state)
        .unwrap()
        .publish_workspace(
            0,
            &profile::WorkspaceConfig {
                format: 1,
                revision: 1,
                id: name.into(),
                name: name.into(),
                cwd: paths,
                folders: vec![],
                policy: Some(profile::WorkspacePolicy::default()),
            },
        )
        .unwrap();
}
#[test]
fn user_owner_reuses_one_real_process_across_scopes_and_detects_conflicts() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("state");
    let project = root.path().join("project");
    fs::create_dir(&project).unwrap();
    let project = project.canonicalize().unwrap();
    workspace(&state, "a", vec![project.display().to_string()]);
    workspace(&state, "b", vec![project.display().to_string()]);
    let script = script(&project);
    let runtime =
        McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap();
    let first = server(&project, &script, McpScope::User, "a");
    let mut alias = first.clone();
    alias.reference.workspace_id = "b".into();
    save(&runtime, "a", first.clone());
    save(&runtime, "b", alias.clone());
    assert_eq!(
        runtime.preset_server_digest("a", "fixture"),
        runtime.preset_server_digest("b", "fixture")
    );
    let a = runtime.prepare_workspace("a").unwrap();
    let b = runtime.prepare_workspace("b").unwrap();
    assert_eq!(a.catalog.tools.len(), 1);
    assert_eq!(b.catalog.tools.len(), 1);
    assert_eq!(
        fs::read_to_string(project.join("starts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "reuse must precede spawn"
    );
    let request = worker_control::ToolControl::new(
        "018f0000-0000-7000-8000-000000000001",
        "018f0000-0000-7000-8000-000000000001",
        1,
        "call",
        b.catalog.tools[0].name.clone(),
        schema::IJsonValue::parse(b"{}").unwrap(),
    )
    .unwrap();
    drop(a);
    let rebound = runtime.prepare_workspace("a").unwrap();
    drop(rebound);
    use crate::production_tool_control::DynamicSupervisorAuthority as _;
    assert!(
        b.authority.execute(&b.catalog.tools[0], &request).is_ok(),
        "one scope cannot close another's lease"
    );
    alias.always_on = true;
    save(&runtime, "conflict", alias);
    let conflicted = runtime.prepare_workspace("a").unwrap();
    assert!(conflicted.catalog.tools.is_empty());
    assert!(conflicted.registry_failure.is_some());
    assert_eq!(
        fs::read_to_string(project.join("starts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "conflicts cannot spawn"
    );
}

#[test]
fn project_stdio_requires_host_grant_and_old_queued_effects_cannot_survive_revoke() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("state");
    let project = root.path().join("project");
    fs::create_dir(&project).unwrap();
    let project = project.canonicalize().unwrap();
    workspace(&state, "a", vec![project.display().to_string()]);
    let script = script(&project);
    let authority = profile::FolderAuthorityStore::new(&state);
    let request = authority.request("request", 0, &project).unwrap();
    let mut config = server(&project, &script, McpScope::Project, "a");
    config.folder_id = Some(request.folder.id.clone());
    let runtime = Arc::new(
        McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap(),
    );
    save(&runtime, "server", config.clone());
    assert!(
        runtime
            .prepare_workspace("a")
            .unwrap()
            .catalog
            .tools
            .is_empty()
    );
    assert!(runtime.probe_server(&config.reference).is_err());
    assert!(runtime.list_resource_catalog("a").unwrap().1.is_empty());
    assert!(!project.join("starts").exists());
    authority.grant_from_host("grant", 1, "request").unwrap();
    fs::write(project.join("descendant"), b"").unwrap();
    let binding = runtime.prepare_workspace("a").unwrap();
    assert_eq!(binding.catalog.tools.len(), 1);
    let secrets = SecretAccess {
        store: Arc::new(MemorySecretStore::new()),
        mutation: None,
        registry: Some(McpRegistryStore::new(state.join("config"))),
        plugin_resolver: None,
    };
    let old_key = super::server_pool_key(&config, None, &secrets).unwrap().0;
    // Direct broker calls exercise admission after its mutex, rather than only
    // the supervisor's early route validation.
    let broker = runtime.broker.handle();
    fs::write(project.join("block"), b"").unwrap();
    let first = broker.clone();
    let first_key = old_key.clone();
    let running = std::thread::spawn(move || {
        first.call_tool(
            first_key,
            "ping",
            schema::IJsonValue::parse(b"{}").unwrap(),
            McpCancellationToken::default(),
        )
    });
    wait_until(|| project.join("entered").exists());
    let second = broker.clone();
    let queued_key = old_key.clone();
    let queued = std::thread::spawn(move || {
        second.call_tool(
            queued_key,
            "ping",
            schema::IJsonValue::parse(b"{}").unwrap(),
            McpCancellationToken::default(),
        )
    });
    authority.revoke("revoke", 2, &request.folder.id).unwrap();
    assert!(!authority.effects_drained(&request.folder.id).unwrap());
    fs::write(project.join("release"), b"").unwrap();
    assert!(running.join().unwrap().is_ok());
    assert!(queued.join().unwrap().is_err());
    assert_eq!(
        fs::read_to_string(project.join("calls"))
            .unwrap()
            .lines()
            .count(),
        1
    );
    wait_until(|| authority.effects_drained(&request.folder.id).unwrap());
    assert!(
        runtime
            .prepare_workspace("a")
            .unwrap()
            .catalog
            .tools
            .is_empty()
    );
    let pid = fs::read_to_string(project.join("starts"))
        .unwrap()
        .trim()
        .to_owned();
    wait_until(|| {
        !std::process::Command::new("/bin/kill")
            .args(["-0", &pid])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    });
    let child_pid = fs::read_to_string(project.join("child-pid")).unwrap();
    wait_until(|| {
        !std::process::Command::new("/bin/kill")
            .args(["-0", child_pid.trim()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap()
            .success()
    });
    authority.request("r2", 3, &project).unwrap();
    authority.grant_from_host("g2", 4, "r2").unwrap();
    let new_key = super::server_pool_key(&config, None, &secrets).unwrap().0;
    assert_ne!(old_key, new_key);
    assert!(
        broker
            .call_tool(
                old_key,
                "ping",
                schema::IJsonValue::parse(b"{}").unwrap(),
                McpCancellationToken::default()
            )
            .is_err()
    );
    fs::remove_file(project.join("descendant")).unwrap();
    let again = runtime.prepare_workspace("a").unwrap();
    assert_eq!(again.catalog.tools.len(), 1);
    assert_eq!(
        fs::read_to_string(project.join("starts"))
            .unwrap()
            .lines()
            .count(),
        2
    );
}

#[test]
fn project_aliases_share_owner_but_session_scope_never_imports_extra_folders() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("state");
    let one = root.path().join("one");
    let two = root.path().join("two");
    fs::create_dir(&one).unwrap();
    fs::create_dir(&two).unwrap();
    let one = one.canonicalize().unwrap();
    let two = two.canonicalize().unwrap();
    let script = script(&one);
    workspace(
        &state,
        "a",
        vec![one.display().to_string(), two.display().to_string()],
    );
    workspace(&state, "b", vec![two.display().to_string()]);
    let authority = profile::FolderAuthorityStore::new(&state);
    let mut folders = Vec::new();
    for (index, path) in [&one, &two].iter().enumerate() {
        let rev = authority.registry_revision().unwrap();
        let req = format!("r{index}");
        authority.request(&req, rev, path).unwrap();
        folders.push(
            authority
                .grant_from_host(&format!("g{index}"), rev + 1, &req)
                .unwrap()
                .folder,
        );
    }
    let runtime =
        McpRuntime::open(state.join("config"), Arc::new(MemorySecretStore::new())).unwrap();
    let mut first = server(&one, &script, McpScope::Project, "a");
    first.folder_id = Some(folders[0].id.clone());
    let mut alias = first.clone();
    alias.reference.workspace_id = "b".into();
    let mut second = server(&two, &script, McpScope::Project, "b");
    second.reference.name = "second".into();
    second.folder_id = Some(folders[1].id.clone());
    save(&runtime, "first", first.clone());
    save(&runtime, "alias", alias);
    save(&runtime, "second", second.clone());
    let mut config = profile::ConfigRepository::open(&state)
        .unwrap()
        .resolve("a")
        .unwrap();
    config.workspace.cwd = vec![one.display().to_string()];
    let narrow = runtime.prepare_for_config(&config).unwrap();
    assert_eq!(narrow.catalog.tools.len(), 1);
    assert!(!two.join("starts").exists());
    assert!(
        !runtime
            .server_visible("a", Some(&config.workspace.cwd), &second.reference)
            .unwrap()
    );
    assert_eq!(
        runtime
            .list_resource_catalog_in_scope("a", Some(&config.workspace.cwd))
            .unwrap()
            .1
            .len(),
        1
    );
    config.workspace.cwd.push(two.display().to_string());
    let broad = runtime.prepare_for_config(&config).unwrap();
    assert_eq!(broad.catalog.tools.len(), 2);
    assert_eq!(
        fs::read_to_string(one.join("starts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "resource reads reuse the existing owner generation"
    );
    second.reference.name = "fixture".into();
    save(&runtime, "collision", second);
    let collision = runtime.prepare_for_config(&config).unwrap();
    assert!(collision.registry_failure.is_some());
    assert!(collision.catalog.tools.is_empty());
}

#[test]
fn credential_generation_changes_pool_identity_without_reviving_the_old_peer() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("state");
    let project = root.path().join("project");
    fs::create_dir(&project).unwrap();
    let project = project.canonicalize().unwrap();
    workspace(&state, "a", vec![project.display().to_string()]);
    let script = script(&project);
    let secrets = Arc::new(MemorySecretStore::new());
    secrets
        .publish(
            "fixture-key",
            provider::SecretRecord::Active {
                generation: 1,
                material: "synthetic-one".into(),
            },
        )
        .unwrap();
    let runtime = McpRuntime::open(state.join("config"), secrets.clone()).unwrap();
    let config = server(&project, &script, McpScope::User, "a");
    runtime
        .mutate(
            "save-key",
            &McpManagementMutation::Save {
                server: config.clone(),
                credential_fields: [(
                    "environment.FIXTURE".into(),
                    McpCredentialFieldOperation::Replace {
                        credential_id: "fixture-key".into(),
                    },
                )]
                .into(),
            },
        )
        .unwrap();
    let configured = runtime.get_server(&config.reference).unwrap().unwrap();
    let access = runtime.secret_access();
    let old = server_pool_key(&configured, None, &access).unwrap().0;
    let a = runtime.prepare_workspace("a").unwrap();
    assert_eq!(a.catalog.tools.len(), 1);
    secrets
        .publish(
            "fixture-key",
            provider::SecretRecord::Active {
                generation: 2,
                material: "synthetic-two".into(),
            },
        )
        .unwrap();
    let new = server_pool_key(&configured, None, &access).unwrap().0;
    assert_ne!(old, new);
    assert!(
        runtime
            .broker
            .handle()
            .call_tool(
                old,
                "ping",
                schema::IJsonValue::parse(b"{}").unwrap(),
                McpCancellationToken::default()
            )
            .is_err()
    );
    let b = runtime.prepare_workspace("a").unwrap();
    assert_eq!(b.catalog.tools.len(), 1);
    assert_eq!(
        fs::read_to_string(project.join("starts"))
            .unwrap()
            .lines()
            .count(),
        2
    );
}
