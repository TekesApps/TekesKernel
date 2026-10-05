use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use mcp::{
    McpManagementMutation, McpScope, McpServerConfig, McpServerReference, McpTransportConfig,
    ProtocolMode,
};
use profile::DynamicToolEffect;
use provider::MemorySecretStore;
use schema::IJsonValue;
use tekes_supervisor::mcp_runtime::{McpRuntime, McpWorkspaceBinding};
use tekes_supervisor::production_tool_control::{
    DynamicSupervisorAuthority, SupervisorOperationError,
};
use tempfile::TempDir;
use worker_control::ToolControl;

const SERVER_NAME: &str = "computer-use";
const PROJECTED_PREFIX: &str = "mcp__computer_2duse__";
const PRODUCTION_EXECUTABLE_ENV: &str = "TEKES_SLICE14A_REFERENCE_EXECUTABLE";
const DENIED_EXECUTABLE_ENV: &str = "TEKES_SLICE14A_DENIED_REFERENCE_EXECUTABLE";
const REFERENCE_REVISION_ENV: &str = "TEKES_SLICE14A_REFERENCE_REVISION";
/// The production designated requirement of the signed reference executable.
/// The committed source lock carries only a placeholder identity.
const REFERENCE_REQUIREMENT_ENV: &str = "TEKES_SLICE14A_REFERENCE_REQUIREMENT";
const TOOL_NAMES: [&str; 12] = [
    "click",
    "drag",
    "get_app_state",
    "list_apps",
    "perform_secondary_action",
    "permission_status",
    "press_key",
    "request_permissions",
    "scroll",
    "select_text",
    "set_value",
    "type_text",
];

struct Harness {
    _root: TempDir,
    data_path: PathBuf,
    executable: PathBuf,
    runtime: McpRuntime,
}

#[test]
fn slice14a_gate_117_generic_local_mcp_configuration() {
    let harness = harness();
    let rows = harness
        .runtime
        .list_servers("qualification")
        .expect("list configured server");
    assert_eq!(
        rows,
        [server_config(&harness.executable, &harness.data_path, true)]
    );
    assert_eq!(rows[0].reference.scope, McpScope::User);
    assert!(rows[0].owner.is_none());
    assert!(rows[0].plugin_component.is_none());

    let missing = TempDir::new().expect("missing root");
    let runtime = McpRuntime::open(
        missing.path().join("mcp"),
        Arc::new(MemorySecretStore::new()),
    )
    .expect("runtime");
    runtime
        .mutate(
            "save-missing",
            &McpManagementMutation::Save {
                server: server_config(
                    &missing.path().join("missing-executable"),
                    missing.path(),
                    true,
                ),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("config durability is separate from readiness");
    let degraded = runtime
        .prepare_workspace("qualification")
        .expect("one unavailable server degrades its own catalog");
    assert!(degraded.catalog.tools.is_empty());
    assert_eq!(degraded.failures.len(), 1);
    assert_eq!(degraded.failures[0].server.name, "computer-use");
    assert_eq!(degraded.failures[0].code, "refresh-failed");
}

#[test]
fn slice14a_gate_118_twelve_tool_discovery_and_call() {
    let harness = harness();
    write_tcc_state(&harness.data_path, true);
    let binding = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("binding");
    assert_eq!(remote_names(&binding), TOOL_NAMES);
    assert_catalog_matches_migration_contract(&binding);
    for (index, name) in TOOL_NAMES.iter().enumerate() {
        if *name == "request_permissions" {
            assert_tool_failure(&binding, name, index, "hermetic-tcc-prompt-forbidden");
            continue;
        }
        let text = canonical_text(&call(&binding, name, index));
        match *name {
            "permission_status" => {
                assert!(text.contains("\\\"accessibility\\\":true"), "{text}");
                assert!(text.contains("\\\"screen_capture\\\":true"), "{text}");
            }
            "list_apps" => assert!(text.contains("[]"), "{text}"),
            _ => assert!(text.contains(&format!("fixture-ok:{name}")), "{text}"),
        }
    }
    let read_only = binding
        .catalog
        .tools
        .iter()
        .filter(|tool| tool.effect == DynamicToolEffect::ReadOnly)
        .map(|tool| remote_name(&tool.name))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        read_only,
        BTreeSet::from(["get_app_state", "list_apps", "permission_status"])
    );
}

#[test]
fn slice14a_gate_119_hermetic_tcc_denial_grant_and_prompt_suppression() {
    let harness = harness();
    write_tcc_state(&harness.data_path, false);
    let binding = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("binding");
    let status = canonical_text(&call(&binding, "permission_status", 1));
    assert!(status.contains("\\\"accessibility\\\":false"), "{status}");
    assert!(status.contains("\\\"screen_capture\\\":false"), "{status}");
    assert_tool_failure(&binding, "click", 2, "tcc-denied");
    assert_tool_failure(
        &binding,
        "request_permissions",
        3,
        "hermetic-tcc-prompt-forbidden",
    );
    write_tcc_state(&harness.data_path, true);
    assert!(canonical_text(&call(&binding, "click", 4)).contains("fixture-ok:click"));
}

#[test]
fn slice14a_gate_120_stop_restart_crash_recovery_and_no_special_cases() {
    let harness = harness();
    write_tcc_state(&harness.data_path, true);
    let first = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("first binding");
    let first_pid = wait_for_pid(&harness.data_path);
    drop(first);
    wait_for_exit(first_pid);

    let restarted = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("restarted binding");
    let restarted_pid = wait_for_different_pid(&harness.data_path, first_pid);
    fs::write(harness.data_path.join("crash-next-call"), b"crash\n").expect("crash marker");
    assert!(call_result(&restarted, "list_apps", 1).is_err());
    let recovered = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("recover after peer loss");
    let recovered_pid = wait_for_different_pid(&harness.data_path, restarted_pid);
    assert!(canonical_text(&call(&recovered, "list_apps", 2)).contains("[]"));

    mutate_server(&harness, "disable-reference", false);
    assert!(call_result(&recovered, "list_apps", 3).is_err());
    assert!(
        harness
            .runtime
            .prepare_workspace("qualification")
            .expect("disabled workspace")
            .catalog
            .tools
            .is_empty()
    );

    mutate_server(&harness, "reenable-reference", true);
    let rebound = harness
        .runtime
        .prepare_workspace("qualification")
        .expect("rebound");
    let rebound_pid = wait_for_different_pid(&harness.data_path, recovered_pid);
    assert_eq!(remote_names(&rebound), TOOL_NAMES);
    harness
        .runtime
        .mutate(
            "remove-reference",
            &McpManagementMutation::Remove {
                reference: reference(),
            },
        )
        .expect("remove ordinary MCP server");
    assert!(
        harness
            .runtime
            .list_servers("qualification")
            .expect("list removed")
            .is_empty()
    );
    assert!(call_result(&rebound, "list_apps", 4).is_err());
    wait_for_exit(rebound_pid);
}

#[test]
#[ignore = "requires explicit signed macOS MCP executables; never requests TCC consent"]
fn slice14a_real_reference_mcp_safe_preflight() {
    assert!(cfg!(target_os = "macos"));
    assert_eq!(
        std::env::var(REFERENCE_REVISION_ENV)
            .unwrap_or_else(|_| panic!("set {REFERENCE_REVISION_ENV}")),
        source_lock_string("/source/revision")
    );
    let production = PathBuf::from(
        std::env::var(PRODUCTION_EXECUTABLE_ENV)
            .unwrap_or_else(|_| panic!("set {PRODUCTION_EXECUTABLE_ENV}")),
    );
    let denied = PathBuf::from(
        std::env::var(DENIED_EXECUTABLE_ENV)
            .unwrap_or_else(|_| panic!("set {DENIED_EXECUTABLE_ENV}")),
    );
    let production_identity = codesign_identity(&production);
    let denied_identity = codesign_identity(&denied);
    assert_eq!(
        production_identity.0,
        source_lock_string("/executable/production_signing/identifier")
    );
    assert_eq!(
        production_identity.1,
        std::env::var(REFERENCE_REQUIREMENT_ENV)
            .unwrap_or_else(|_| panic!("set {REFERENCE_REQUIREMENT_ENV}"))
    );
    assert_ne!(production_identity, denied_identity);

    let root = TempDir::new().expect("real root");
    let data_path = root.path().join("data");
    fs::create_dir_all(&data_path).expect("real data path");
    let runtime = McpRuntime::open(root.path().join("mcp"), Arc::new(MemorySecretStore::new()))
        .expect("real runtime");
    runtime
        .mutate(
            "configure-real-reference",
            &McpManagementMutation::Save {
                server: server_config(&production, &data_path, true),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("configure real executable");
    let binding = runtime
        .prepare_workspace("qualification")
        .expect("real binding");
    assert_eq!(remote_names(&binding), TOOL_NAMES);
    assert_eq!(
        binding.catalog.digest().expect("catalog digest"),
        source_lock_string("/projected_catalog/sha256")
    );
    drop(binding);
}

fn harness() -> Harness {
    let root = TempDir::new().expect("qualification root");
    let data_path = root.path().join("data");
    fs::create_dir_all(&data_path).expect("data path");
    let executable = root.path().join("reference-mcp");
    fs::copy(fixture_executable(), &executable).expect("copy reference executable");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).expect("executable mode");
    let runtime = McpRuntime::open(root.path().join("mcp"), Arc::new(MemorySecretStore::new()))
        .expect("runtime");
    let harness = Harness {
        _root: root,
        data_path,
        executable,
        runtime,
    };
    mutate_server(&harness, "configure-reference", true);
    harness
}

fn mutate_server(harness: &Harness, rpc_id: &str, enabled: bool) {
    harness
        .runtime
        .mutate(
            rpc_id,
            &McpManagementMutation::Save {
                server: server_config(&harness.executable, &harness.data_path, enabled),
                credential_fields: BTreeMap::new(),
            },
        )
        .expect("save ordinary local MCP server");
}

fn fixture_executable() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/mcp-reference-qualification/reference-mcp")
}

fn reference() -> McpServerReference {
    McpServerReference {
        workspace_id: "qualification".to_owned(),
        scope: McpScope::User,
        name: SERVER_NAME.to_owned(),
    }
}

fn server_config(executable: &Path, data_path: &Path, enabled: bool) -> McpServerConfig {
    McpServerConfig {
        reference: reference(),
        transport: McpTransportConfig::Stdio {
            command: vec![executable.to_string_lossy().into_owned()],
            cwd: Some(data_path.to_string_lossy().into_owned()),
            environment: BTreeMap::new(),
        },
        enabled,
        always_on: false,
        protocol_mode: ProtocolMode::Auto,
        owner: None,
        plugin_component: None,
        project_trusted: false,
    }
}

fn remote_names(binding: &McpWorkspaceBinding) -> [&str; 12] {
    binding
        .catalog
        .tools
        .iter()
        .map(|tool| remote_name(&tool.name))
        .collect::<Vec<_>>()
        .try_into()
        .expect("exactly twelve tools")
}

fn remote_name(projected: &str) -> &str {
    projected
        .strip_prefix(PROJECTED_PREFIX)
        .expect("generic projected prefix")
}

fn assert_catalog_matches_migration_contract(binding: &McpWorkspaceBinding) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/tools/first-party-tools.canonical.json"
    ))
    .expect("first-party tool contract");
    let expected = fixture["plugins"][0]["tools"]
        .as_array()
        .expect("tool rows")
        .iter()
        .map(|row| {
            let name = row["name"].as_str().expect("tool name").to_owned();
            let arguments = row["arguments"]
                .as_array()
                .expect("arguments")
                .iter()
                .map(|value| value.as_str().expect("argument").to_owned())
                .collect::<BTreeSet<_>>();
            let effect = match (
                row["operation"].as_str().expect("operation"),
                row["effect"].as_str().expect("effect"),
            ) {
                ("read_only", "read_only") => DynamicToolEffect::ReadOnly,
                ("destructive", "system_permission") => DynamicToolEffect::Destructive,
                ("execute", "computer_control") => DynamicToolEffect::ExternalProcess,
                other => panic!("unmapped operation/effect {other:?}"),
            };
            (name, (arguments, effect))
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(expected.len(), binding.catalog.tools.len());
    for tool in &binding.catalog.tools {
        let name = remote_name(&tool.name);
        let (arguments, effect) = expected.get(name).expect("frozen tool name");
        let schema: serde_json::Value =
            serde_json::from_slice(&tool.schema.canonical_bytes().expect("schema bytes"))
                .expect("schema JSON");
        let actual = schema["parameters"]["properties"]
            .as_object()
            .expect("properties")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(&actual, arguments, "argument drift for {name}");
        assert_eq!(&tool.effect, effect, "effect drift for {name}");
    }
}

fn assert_tool_failure(binding: &McpWorkspaceBinding, name: &str, index: usize, reason: &str) {
    match call_result(binding, name, index) {
        Err(SupervisorOperationError::ToolFailed(message)) => {
            assert!(message.contains(reason), "{message}")
        }
        other => panic!("expected definitive tool failure {reason}, got {other:?}"),
    }
}

fn call(binding: &McpWorkspaceBinding, name: &str, index: usize) -> IJsonValue {
    call_result(binding, name, index).expect("reference tool call")
}

fn call_result(
    binding: &McpWorkspaceBinding,
    name: &str,
    index: usize,
) -> Result<IJsonValue, SupervisorOperationError> {
    let projected = format!("{PROJECTED_PREFIX}{name}");
    let tool = binding
        .catalog
        .tools
        .iter()
        .find(|tool| tool.name == projected)
        .expect("projected tool");
    let request = ToolControl::new(
        "11111111-1111-4111-8111-111111111111",
        "22222222-2222-4222-8222-222222222222",
        1,
        format!("reference-call-{index}"),
        projected,
        IJsonValue::parse_str("{}").expect("empty arguments"),
    )
    .expect("tool control");
    binding.authority.execute(tool, &request)
}

fn canonical_text(value: &IJsonValue) -> String {
    String::from_utf8(value.canonical_bytes().expect("canonical result")).expect("UTF-8")
}

fn write_tcc_state(data_path: &Path, granted: bool) {
    fs::write(
        data_path.join("tcc-state.canonical.json"),
        format!("{{\"accessibility\":{granted},\"screen_capture\":{granted}}}\n"),
    )
    .expect("TCC state");
}

fn wait_for_pid(data_path: &Path) -> u32 {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Ok(value) = fs::read_to_string(data_path.join("server.pid")) {
            if let Ok(pid) = value.trim().parse() {
                return pid;
            }
        }
        assert!(Instant::now() < deadline, "reference process did not start");
        thread::sleep(Duration::from_millis(20));
    }
}

fn wait_for_different_pid(data_path: &Path, previous: u32) -> u32 {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let pid = wait_for_pid(data_path);
        if pid != previous {
            return pid;
        }
        assert!(
            Instant::now() < deadline,
            "reference process did not restart"
        );
        thread::sleep(Duration::from_millis(20));
    }
}

fn wait_for_exit(pid: u32) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while process_exists(pid) {
        assert!(Instant::now() < deadline, "process {pid} did not exit");
        thread::sleep(Duration::from_millis(20));
    }
}

fn process_exists(pid: u32) -> bool {
    Command::new("/bin/kill")
        .args(["-0", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn codesign_identity(executable: &Path) -> (String, String) {
    let verify = Command::new("/usr/bin/codesign")
        .args(["--verify", "--strict"])
        .arg(executable)
        .output()
        .expect("codesign verify");
    assert!(verify.status.success(), "codesign verification failed");
    let details = Command::new("/usr/bin/codesign")
        .args(["-d", "-r-", "--verbose=4"])
        .arg(executable)
        .output()
        .expect("codesign details");
    assert!(details.status.success(), "codesign details failed");
    let text = String::from_utf8(details.stderr).expect("codesign UTF-8");
    let identifier = text
        .lines()
        .find_map(|line| line.strip_prefix("Identifier="))
        .expect("Identifier")
        .to_owned();
    let requirement = text
        .lines()
        .find_map(|line| line.strip_prefix("designated => "))
        .expect("designated requirement")
        .to_owned();
    (identifier, requirement)
}

fn source_lock_string(pointer: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/mcp-reference-qualification/source-lock.canonical.json"
    ))
    .expect("source lock");
    value
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("source-lock string {pointer}"))
        .to_owned()
}
