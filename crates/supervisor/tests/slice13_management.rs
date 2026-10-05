use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

use endpoint::{
    ClientRequest, EndpointDispatcher, EndpointHost, RpcRegistry, SessionHostDescription,
};
use mcp::{
    CredentialValue, McpCredentialReferenceState, McpManagementMutation, McpRegistryStore,
    McpScope, McpServerConfig, McpServerReference, McpTransportConfig, ProtocolMode,
};
use plugins::{
    HostEnvironment, InstallOptions, MacOsNativeHelperVerifier, OperatingSystem,
    PluginComponentReference, PluginSource, PluginStore, PluginVersion, SignaturePolicy,
};
use provider::{MemorySecretStore, SecretRecord};
use schema::IJsonValue;
use tekes_supervisor::client_admin::{PROVIDER_ADMIN_METHODS, WORKSPACE_POLICY_METHODS};
use tekes_supervisor::client_extensions::{
    APPROVAL_METHODS, ATTACHMENT_METHODS, FEEDBACK_METHODS, FILE_METHODS, GOAL_METHODS,
    HOST_FILE_METHODS, INITIAL_PRESET_METHODS, PLUGIN_METHODS, RECOVERY_METHODS, RESOURCE_METHODS,
    SCHEDULE_METHODS, SETTINGS_METHODS, SUBAGENT_METHODS, THREAD_SEARCH_METHODS, TOOL_METHODS,
    USAGE_METHODS,
};
use tekes_supervisor::daemon::assemble_production_endpoint_host;
use tekes_supervisor::endpoint_host::TEKES_UNARY_ROUTES;
use tekes_supervisor::mcp_runtime::{
    MCP_MANAGEMENT_METHODS, McpRuntime, PluginMcpResolver, PluginStoreMcpResolver,
};
use tekes_supervisor::process_host::ProductionProcessHost;
use tekes_supervisor::production_tool_control::DynamicSupervisorAuthority;
use tekes_supervisor::resource_capability::RESOURCE_CAPABILITIES;
use worker_control::ToolControl;

fn request(rpc_id: &str, method: &str, payload: &str) -> ClientRequest {
    ClientRequest {
        envelope_type: "client-request".to_owned(),
        rpc_id: rpc_id.to_owned(),
        method: method.to_owned(),
        payload: IJsonValue::parse_str(payload).expect("I-JSON payload"),
    }
}

#[tokio::test]
async fn slice13_gate_91_production_assembly_mounts_and_executes_management() {
    let root = tempfile::tempdir().expect("production root");
    let agent = root.path().join(".agent");
    std::fs::create_dir(&agent).expect("agent root");
    let process = ProductionProcessHost::open(
        root.path(),
        std::env::current_exe().expect("test executable"),
        env!("CARGO_PKG_VERSION"),
        &agent,
    )
    .expect("production process host");
    let host = assemble_production_endpoint_host(
        root.path(),
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            cwd: "/".to_owned(),
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
    .expect("production endpoint assembly");
    let advertised = host.capabilities();
    let expected = MCP_MANAGEMENT_METHODS
        .into_iter()
        .chain(RESOURCE_CAPABILITIES)
        .chain(
            [
                RESOURCE_METHODS.as_slice(),
                INITIAL_PRESET_METHODS.as_slice(),
                RECOVERY_METHODS.as_slice(),
                ATTACHMENT_METHODS.as_slice(),
                APPROVAL_METHODS.as_slice(),
                HOST_FILE_METHODS.as_slice(),
                FEEDBACK_METHODS.as_slice(),
                SETTINGS_METHODS.as_slice(),
                GOAL_METHODS.as_slice(),
                SUBAGENT_METHODS.as_slice(),
                FILE_METHODS.as_slice(),
                TOOL_METHODS.as_slice(),
                PLUGIN_METHODS.as_slice(),
                SCHEDULE_METHODS.as_slice(),
                THREAD_SEARCH_METHODS.as_slice(),
                PROVIDER_ADMIN_METHODS.as_slice(),
                WORKSPACE_POLICY_METHODS.as_slice(),
                USAGE_METHODS.as_slice(),
            ]
            .into_iter()
            .flatten()
            .map(|(method, _)| *method),
        )
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        TEKES_UNARY_ROUTES.len(),
        16,
        "Session Endpoint V3 unary registry must remain closed"
    );
    assert_eq!(host.extension_capabilities(), expected);
    assert!(host.extension_capabilities().is_subset(&advertised));

    let dispatcher = EndpointDispatcher::new(RpcRegistry::open(root.path()).expect("rpc registry"));
    let saved = dispatcher
        .dispatch(
            &host,
            request(
                "mcp-save-1",
                "mcp.save",
                r#"{"credentialFields":{},"server":{"always_on":false,"enabled":true,"project_trusted":true,"protocol_mode":"legacy","reference":{"name":"fixture","scope":"project","workspace_id":"ws"},"transport":{"command":["/usr/bin/true"],"environment":{},"kind":"stdio"}}}"#,
            ),
        )
        .await
        .expect("save dispatch");
    assert!(saved.result.ok, "{saved:?}");
    let listed = dispatcher
        .dispatch(
            &host,
            request("mcp-list-1", "mcp.list", r#"{"workspaceId":"ws"}"#),
        )
        .await
        .expect("list dispatch");
    assert!(listed.result.ok, "{listed:?}");
    let value = listed.result.value.expect("list value");
    let value: serde_json::Value =
        serde_json::from_slice(&value.canonical_bytes().expect("canonical list response"))
            .expect("list JSON");
    assert_eq!(value["servers"][0]["editable"], true);
    assert_eq!(
        value["servers"][0]["server"]["reference"]["name"],
        "fixture"
    );

    let malformed = dispatcher
        .dispatch(
            &host,
            request(
                "mcp-list-bad",
                "mcp.list",
                r#"{"workspaceId":"ws","unknown":true}"#,
            ),
        )
        .await
        .expect("malformed dispatch response");
    assert!(!malformed.result.ok);
    process.shutdown();
}

#[test]
fn production_assembly_keeps_unavailable_workspace_in_inventory_authority() {
    let root = tempfile::tempdir().expect("production root");
    let agent = root.path().join(".agent");
    fs::create_dir(&agent).expect("agent root");
    let missing = root.path().join("unmounted-workspace");
    let repository = profile::ConfigRepository::open(root.path()).expect("config repository");
    repository
        .publish_workspace(
            0,
            &profile::WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "workspace-unavailable".to_owned(),
                name: "Unavailable Workspace".to_owned(),
                cwd: vec![missing.display().to_string()],
                folders: Vec::new(),
                policy: Some(profile::WorkspacePolicy::default()),
            },
        )
        .expect("workspace authority");
    assert!(!missing.exists());

    let process = ProductionProcessHost::open(
        root.path(),
        std::env::current_exe().expect("test executable"),
        env!("CARGO_PKG_VERSION"),
        &agent,
    )
    .expect("production process host");
    let _host = assemble_production_endpoint_host(
        root.path(),
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            cwd: "/".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: root.path().to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-09-01T00:00:00.000000000Z".to_owned())),
        &agent,
        Arc::clone(&process),
    )
    .expect("an unavailable workspace must not prevent endpoint assembly");

    process.shutdown();
}

#[test]
fn slice13_gate_90_plugin_bridge_resolves_immutable_slice12_generation() {
    let root = tempfile::tempdir().expect("root");
    let package = root.path().join("package-source");
    fs::create_dir(&package).expect("package source");
    let executable = package.join("mcp-server");
    fs::write(
        &executable,
        b"#!/bin/sh\nIFS= read -r first\nprintf '%s\\n' '{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":{\"capabilities\":{\"tools\":{}},\"protocolVersion\":\"2025-11-25\",\"serverInfo\":{\"name\":\"plugin-fixture\",\"version\":\"1\"}}}'\nIFS= read -r initialized\nIFS= read -r list\nprintf '%s\\n' '{\"id\":2,\"jsonrpc\":\"2.0\",\"result\":{\"tools\":[{\"name\":\"echo\",\"description\":\"Echo\",\"inputSchema\":{\"additionalProperties\":false,\"properties\":{},\"required\":[],\"type\":\"object\"}}]}}'\nwhile IFS= read -r line; do printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"method\":\"notifications/tools/list_changed\"}'; printf '%s\\n' '{\"id\":3,\"jsonrpc\":\"2.0\",\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"ok\"}],\"isError\":false}}'; done\n",
    )
    .expect("server script");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).expect("executable mode");
    let operating_system = if cfg!(target_os = "macos") {
        OperatingSystem::MacOs
    } else if cfg!(target_os = "windows") {
        OperatingSystem::Windows
    } else {
        OperatingSystem::Linux
    };
    let os_name = match operating_system {
        OperatingSystem::MacOs => "macos",
        OperatingSystem::Linux => "linux",
        OperatingSystem::Windows => "windows",
    };
    fs::write(
        package.join("tekes-plugin.json"),
        serde_json::to_vec(&serde_json::json!({
            "manifestVersion":1,
            "id":"com.example.mcp",
            "version":"1.0.0",
            "displayName":"Fixture MCP",
            "platforms":[{"os":os_name,"architectures":[std::env::consts::ARCH]}],
            "capabilities":[{"id":"network.access"}],
            "components":[{"id":"server","type":"mcp-server","path":"mcp-server","capabilities":["network.access"]}]
        }))
        .expect("manifest JSON"),
    )
    .expect("manifest");
    let plugin_root = root.path().join("plugins");
    let host = HostEnvironment {
        host_version: "1.0.0".parse::<PluginVersion>().expect("host version"),
        operating_system,
        operating_system_version: "1.0".to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
    };
    let policy = SignaturePolicy {
        allow_unsigned_local: true,
        trusted_publishers: BTreeMap::new(),
    };
    let mut plugins = PluginStore::open(
        &plugin_root,
        host.clone(),
        policy.clone(),
        MacOsNativeHelperVerifier,
    )
    .expect("plugin store");
    let receipt = plugins
        .install(
            &PluginSource::Directory(package.clone()),
            InstallOptions {
                grants: BTreeSet::from(["network.access".to_owned()]),
                enable: true,
                ..InstallOptions::default()
            },
        )
        .expect("install plugin package");
    let resolver: Arc<dyn PluginMcpResolver> = Arc::new(PluginStoreMcpResolver::new(plugins));
    let config_root = root.path().join("config");
    let runtime = McpRuntime::open_with_plugin_resolver(
        &config_root,
        Arc::new(MemorySecretStore::new()),
        Some(resolver),
    )
    .expect("MCP runtime");
    let listed = runtime
        .list_servers("ws")
        .expect("automatic plugin projection");
    assert_eq!(listed.len(), 1);
    let server = &listed[0];
    assert_eq!(server.reference.scope, McpScope::Plugin);
    assert_eq!(server.owner.as_deref(), Some("com.example.mcp"));
    assert_eq!(
        server.plugin_component,
        Some(PluginComponentReference {
            plugin_id: "com.example.mcp".to_owned(),
            component_id: "server".to_owned(),
        })
    );
    let binding = runtime.prepare_workspace("ws").expect("plugin MCP binding");
    assert_eq!(binding.catalog.tools[0].name, "mcp__server__echo");
    let request = ToolControl::new(
        "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222",
        1,
        "call",
        "mcp__server__echo",
        IJsonValue::parse_str("{}").expect("arguments"),
    )
    .expect("tool control");
    binding
        .authority
        .execute(&binding.catalog.tools[0], &request)
        .expect("plugin tool call");
    let catalog_refreshed = runtime
        .prepare_workspace("ws")
        .expect("list_changed invalidates binding cache");
    assert!(!Arc::ptr_eq(
        &binding.authority,
        &catalog_refreshed.authority
    ));
    assert!(
        runtime
            .mutate(
                "plugin-overwrite",
                &McpManagementMutation::Remove {
                    reference: server.reference.clone(),
                },
            )
            .is_err(),
        "standalone management cannot mutate plugin-owned rows"
    );
    assert_eq!(receipt.plugin_id, "com.example.mcp");
    assert!(receipt.package_digest.len() >= 64);

    let mut lifecycle = PluginStore::open(
        &plugin_root,
        host.clone(),
        policy.clone(),
        MacOsNativeHelperVerifier,
    )
    .expect("second production plugin authority");
    fs::OpenOptions::new()
        .append(true)
        .open(&executable)
        .expect("open plugin executable for update")
        .write_all(b"\n")
        .expect("change package digest");
    lifecycle
        .install(
            &PluginSource::Directory(package),
            InstallOptions {
                grants: BTreeSet::from(["network.access".to_owned()]),
                enable: true,
                allow_same_version_replacement: true,
                ..InstallOptions::default()
            },
        )
        .expect("update plugin package generation");
    assert_eq!(runtime.reconcile_authority().expect("reconcile update"), 1);
    let updated = runtime
        .prepare_workspace("ws")
        .expect("updated plugin generation");
    assert!(!Arc::ptr_eq(
        &catalog_refreshed.authority,
        &updated.authority
    ));
    lifecycle
        .set_enabled("com.example.mcp", false)
        .expect("disable plugin");
    assert!(
        updated
            .authority
            .execute(&updated.catalog.tools[0], &request)
            .is_err(),
        "without a production mutation hook yet, every effect still fail-closes against the current plugin lifecycle authority"
    );
    assert_eq!(runtime.reconcile_authority().expect("reconcile disable"), 1);
    assert!(
        runtime
            .list_servers("ws")
            .expect("projection after disable")
            .is_empty()
    );
    lifecycle
        .set_enabled("com.example.mcp", true)
        .expect("re-enable after disable");
    let after_disable = runtime
        .prepare_workspace("ws")
        .expect("generation after re-enable");
    assert!(!Arc::ptr_eq(&updated.authority, &after_disable.authority));
    lifecycle
        .set_grants("com.example.mcp", BTreeSet::new())
        .expect("revoke plugin grant");
    assert_eq!(
        runtime
            .reconcile_authority()
            .expect("reconcile grant revoke"),
        1
    );
    assert!(
        runtime
            .list_servers("ws")
            .expect("projection after revoke")
            .is_empty()
    );

    lifecycle
        .set_grants(
            "com.example.mcp",
            BTreeSet::from(["network.access".to_owned()]),
        )
        .expect("restore plugin grant");
    lifecycle
        .set_enabled("com.example.mcp", true)
        .expect("re-enable plugin");
    let rebound = runtime
        .prepare_workspace("ws")
        .expect("fresh plugin generation");
    assert!(!Arc::ptr_eq(&after_disable.authority, &rebound.authority));
    assert!(
        lifecycle
            .remove("com.example.mcp")
            .expect("uninstall plugin")
    );
    assert_eq!(
        runtime.reconcile_authority().expect("reconcile uninstall"),
        1
    );
    assert!(
        runtime
            .list_servers("ws")
            .expect("projection after uninstall")
            .is_empty()
    );
}

#[test]
fn slice13_gate_89_pending_oauth_requires_exact_bound_ownership() {
    let root = tempfile::tempdir().expect("root");
    let secrets = Arc::new(MemorySecretStore::new());
    secrets
        .publish(
            "oauth-ref",
            SecretRecord::Active {
                generation: 1,
                material: "same-name-token-must-not-authorize-pending".to_owned(),
            },
        )
        .expect("active same-name secret");
    let runtime = McpRuntime::open(root.path(), secrets.clone()).expect("runtime");
    let reference = McpServerReference {
        workspace_id: "ws".to_owned(),
        scope: McpScope::User,
        name: "oauth".to_owned(),
    };
    runtime
        .mutate(
            "oauth-save",
            &McpManagementMutation::Save {
                server: McpServerConfig {
                    reference: reference.clone(),
                    transport: McpTransportConfig::Http {
                        url: "https://127.0.0.1:9/mcp".to_owned(),
                        headers: BTreeMap::new(),
                        oauth: None,
                    },
                    enabled: true,
                    always_on: false,
                    protocol_mode: ProtocolMode::Legacy,
                    owner: None,
                    plugin_component: None,
                    project_trusted: false,
                },
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save OAuth server");
    runtime
        .mutate(
            "oauth-start",
            &McpManagementMutation::OauthStart {
                reference: reference.clone(),
                credential_id: "oauth-ref".to_owned(),
                authorization_url: "https://identity.example.invalid/authorize".to_owned(),
            },
        )
        .expect("persist pending OAuth owner");
    let store = McpRegistryStore::new(root.path());
    assert_eq!(
        store
            .credential_reference_state(&reference, "oauth", "oauth-ref")
            .expect("pending state"),
        Some(McpCredentialReferenceState::Pending)
    );
    let degraded = runtime
        .prepare_workspace("ws")
        .expect("pending owner degrades only its server");
    assert!(degraded.catalog.tools.is_empty());
    assert_eq!(degraded.failures.len(), 1);
    assert_eq!(degraded.failures[0].server, reference);
    assert_eq!(degraded.failures[0].code, "credential-unavailable");
    let bound = runtime
        .bind_oauth("oauth-bind", &reference, "oauth-ref")
        .expect("trusted OAuth completion");
    assert_eq!(
        store
            .credential_reference_state(&reference, "oauth", "oauth-ref")
            .expect("bound state"),
        Some(McpCredentialReferenceState::Bound)
    );
    assert!(
        runtime
            .mutate(
                "oauth-bind-bypass",
                &McpManagementMutation::OauthBind {
                    reference: reference.clone(),
                    credential_id: "oauth-ref".to_owned(),
                },
            )
            .is_err(),
        "ordinary management cannot bypass the trusted completion gate"
    );
    secrets
        .publish("oauth-ref", SecretRecord::Revoked { generation: 2 })
        .expect("revoke after completion");
    assert_eq!(
        runtime
            .bind_oauth("oauth-bind", &reference, "oauth-ref")
            .expect("exact rpc retry re-acks after later revocation"),
        bound
    );
    assert!(
        runtime
            .bind_oauth("oauth-bind-wrong", &reference, "different-ref")
            .is_err(),
        "completion cannot claim a different credential identity"
    );
}

#[test]
fn slice13_gate_91_nested_credential_values_fail_closed() {
    assert!(serde_json::from_str::<CredentialValue>(r#"{"literal":"ok","extra":true}"#).is_err());
    assert!(
        serde_json::from_str::<CredentialValue>(
            r#"{"literal":"ambiguous","credential":"must-fail"}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<McpServerConfig>(
            r#"{"reference":{"workspace_id":"ws","scope":"user","name":"closed"},"transport":{"kind":"http","url":"https://example.invalid/mcp","headers":{"X-Test":{"credential":"ref","unknown":true}},"oauth":null},"enabled":true,"always_on":false,"protocol_mode":"legacy","project_trusted":false}"#
        )
        .is_err()
    );
}

#[tokio::test]
async fn slice13_gate_88_standalone_mutations_close_the_bound_generation() {
    let root = tempfile::tempdir().expect("root");
    let executable = root.path().join("standalone-mcp");
    fs::write(
        &executable,
        b"#!/bin/sh\nIFS= read -r first\nprintf '%s\\n' '{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":{\"capabilities\":{\"tools\":{}},\"protocolVersion\":\"2025-11-25\",\"serverInfo\":{\"name\":\"standalone\",\"version\":\"1\"}}}'\nIFS= read -r initialized\nIFS= read -r list\nprintf '%s\\n' '{\"id\":2,\"jsonrpc\":\"2.0\",\"result\":{\"tools\":[{\"name\":\"echo\",\"description\":\"Echo\",\"inputSchema\":{\"additionalProperties\":false,\"properties\":{},\"required\":[],\"type\":\"object\"}}]}}'\nwhile IFS= read -r line; do printf '%s\\n' '{\"id\":3,\"jsonrpc\":\"2.0\",\"result\":{\"content\":[{\"type\":\"text\",\"text\":\"ok\"}],\"isError\":false}}'; done\n",
    )
    .expect("server script");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).expect("executable mode");
    let runtime = McpRuntime::open(
        root.path().join("config"),
        Arc::new(MemorySecretStore::new()),
    )
    .expect("runtime");
    let reference = McpServerReference {
        workspace_id: "ws".to_owned(),
        scope: McpScope::User,
        name: "standalone".to_owned(),
    };
    let config = |enabled| McpServerConfig {
        reference: reference.clone(),
        transport: McpTransportConfig::Stdio {
            command: vec![executable.to_string_lossy().into_owned()],
            cwd: None,
            environment: BTreeMap::new(),
        },
        enabled,
        always_on: false,
        protocol_mode: ProtocolMode::Legacy,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    };
    runtime
        .mutate(
            "save-enabled",
            &McpManagementMutation::Save {
                server: config(true),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save enabled server");
    let mut unaffected = config(true);
    unaffected.reference.name = "unaffected".to_owned();
    runtime
        .mutate(
            "save-unaffected",
            &McpManagementMutation::Save {
                server: unaffected,
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save unaffected server");
    let mut unavailable = config(true);
    unavailable.reference.name = "offline".to_owned();
    unavailable.transport = McpTransportConfig::Stdio {
        command: vec![
            root.path()
                .join("missing-mcp")
                .to_string_lossy()
                .into_owned(),
        ],
        cwd: None,
        environment: BTreeMap::new(),
    };
    runtime
        .mutate(
            "save-offline",
            &McpManagementMutation::Save {
                server: unavailable,
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save offline server");
    let binding = runtime.prepare_workspace("ws").expect("binding");
    assert_eq!(binding.catalog.tools.len(), 2);
    assert_eq!(binding.failures.len(), 1);
    assert_eq!(binding.failures[0].server.name, "offline");
    assert_eq!(binding.failures[0].code, "refresh-failed");
    let request = ToolControl::new(
        "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222",
        1,
        "call-disable",
        "mcp__standalone__echo",
        IJsonValue::parse_str("{}").expect("arguments"),
    )
    .expect("tool control");
    let mut changed = config(true);
    if let McpTransportConfig::Stdio { environment, .. } = &mut changed.transport {
        environment.insert(
            "MODE".to_owned(),
            CredentialValue::Literal {
                literal: "changed".to_owned(),
            },
        );
    }
    runtime
        .mutate(
            "save-config-generation",
            &McpManagementMutation::Save {
                server: changed,
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("change configuration generation");
    assert!(
        binding
            .authority
            .execute(&binding.catalog.tools[0], &request)
            .is_err(),
        "frozen authority cannot execute after a config generation change"
    );
    let unaffected_tool = binding
        .catalog
        .tools
        .iter()
        .find(|tool| tool.name == "mcp__unaffected__echo")
        .expect("unaffected route");
    let unaffected_request = ToolControl::new(
        "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222",
        1,
        "call-unaffected",
        "mcp__unaffected__echo",
        IJsonValue::parse_str("{}").expect("arguments"),
    )
    .expect("unaffected tool control");
    binding
        .authority
        .execute(unaffected_tool, &unaffected_request)
        .expect("unrelated server generation remains live");
    let changed_binding = runtime.prepare_workspace("ws").expect("changed binding");
    runtime
        .mutate(
            "save-disabled",
            &McpManagementMutation::Save {
                server: config(false),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("disable server");
    assert!(
        changed_binding
            .authority
            .execute(&changed_binding.catalog.tools[0], &request)
            .is_err(),
        "frozen authority cannot execute after disable"
    );
    let disabled = runtime.prepare_workspace("ws").expect("disabled workspace");
    assert_eq!(disabled.catalog.tools.len(), 1);
    assert_eq!(disabled.catalog.tools[0].name, "mcp__unaffected__echo");

    runtime
        .mutate(
            "save-again",
            &McpManagementMutation::Save {
                server: config(true),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("re-enable server");
    let rebound = runtime.prepare_workspace("ws").expect("rebound");
    runtime
        .mutate(
            "remove-server",
            &McpManagementMutation::Remove {
                reference: reference.clone(),
            },
        )
        .expect("remove server");
    assert!(
        rebound
            .authority
            .execute(&rebound.catalog.tools[0], &request)
            .is_err(),
        "frozen authority cannot execute after remove"
    );
}
