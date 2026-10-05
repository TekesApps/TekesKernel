use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use endpoint::{DurableHandoffSignal, EndpointHostCall, MethodClass};
use schema::IJsonValue;
use serde_json::Value;
use tekes_supervisor::client_extensions::{
    APPROVAL_METHODS, ATTACHMENT_METHODS, ClientExtensionAuthority, FEEDBACK_METHODS, FILE_METHODS,
    GOAL_METHODS, HOST_FILE_METHODS, INITIAL_PRESET_METHODS, MCP_METHODS, PLUGIN_METHODS,
    RECOVERY_METHODS, RESOURCE_METHODS, SCHEDULE_METHODS, SETTINGS_METHODS, SUBAGENT_METHODS,
    THREAD_SEARCH_METHODS, TOOL_METHODS, USAGE_METHODS, common_capability_routes,
};
use tekes_supervisor::endpoint_host::{
    CompositeProductionEndpointRoutes, ProductionEndpointRoutes, ProductionRouteFailure,
};

#[derive(Default)]
struct RecordingAuthority {
    calls: Mutex<Vec<(String, String, String, bool)>>,
}

impl ClientExtensionAuthority for RecordingAuthority {
    fn execute(
        &self,
        operation: &str,
        _payload: &Value,
        principal: &str,
        rpc_id: &str,
        recovering: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        self.calls.lock().expect("calls").push((
            operation.to_owned(),
            principal.to_owned(),
            rpc_id.to_owned(),
            recovering,
        ));
        IJsonValue::parse_str(r#"{"ok":true}"#).map_err(|error| {
            ProductionRouteFailure::new(
                "internal",
                error.to_string(),
                IJsonValue::parse_str("{}").expect("details"),
            )
        })
    }
}

#[test]
fn common_groups_are_atomic_complete_and_exclude_parallel_config_groups() {
    let authority = Arc::new(RecordingAuthority::default());
    let routes = common_capability_routes(authority);
    assert_eq!(routes.len(), 16);
    let composed = CompositeProductionEndpointRoutes::compose(routes).expect("compose");
    let expected = [
        RECOVERY_METHODS.as_slice(),
        ATTACHMENT_METHODS.as_slice(),
        APPROVAL_METHODS.as_slice(),
        HOST_FILE_METHODS.as_slice(),
        FEEDBACK_METHODS.as_slice(),
        SETTINGS_METHODS.as_slice(),
        GOAL_METHODS.as_slice(),
        SUBAGENT_METHODS.as_slice(),
        FILE_METHODS.as_slice(),
        RESOURCE_METHODS.as_slice(),
        INITIAL_PRESET_METHODS.as_slice(),
        TOOL_METHODS.as_slice(),
        PLUGIN_METHODS.as_slice(),
        SCHEDULE_METHODS.as_slice(),
        THREAD_SEARCH_METHODS.as_slice(),
        USAGE_METHODS.as_slice(),
    ]
    .into_iter()
    .flatten()
    .map(|(name, _)| (*name).to_owned())
    .collect::<BTreeSet<_>>();
    assert_eq!(composed.capabilities(), expected);
    assert!(
        !composed.capabilities().iter().any(
            |method| method.starts_with("providers.") || method.starts_with("workspace.policy")
        )
    );
    for (method, class) in [
        RECOVERY_METHODS.as_slice(),
        ATTACHMENT_METHODS.as_slice(),
        APPROVAL_METHODS.as_slice(),
        HOST_FILE_METHODS.as_slice(),
        FEEDBACK_METHODS.as_slice(),
        SETTINGS_METHODS.as_slice(),
        GOAL_METHODS.as_slice(),
        SUBAGENT_METHODS.as_slice(),
        FILE_METHODS.as_slice(),
        RESOURCE_METHODS.as_slice(),
        INITIAL_PRESET_METHODS.as_slice(),
        TOOL_METHODS.as_slice(),
        PLUGIN_METHODS.as_slice(),
        SCHEDULE_METHODS.as_slice(),
        THREAD_SEARCH_METHODS.as_slice(),
        USAGE_METHODS.as_slice(),
    ]
    .into_iter()
    .flatten()
    {
        assert_eq!(composed.extension_method_class(method), Some(*class));
    }
    assert_eq!(
        MCP_METHODS.len(),
        7,
        "existing MCP route remains an exact atomic group"
    );
}

#[test]
fn mutation_forwards_rpc_identity_and_records_authority_handoff() {
    let authority = Arc::new(RecordingAuthority::default());
    let authority_dyn: Arc<dyn ClientExtensionAuthority> = authority.clone();
    let route = common_capability_routes(authority_dyn)
        .into_iter()
        .find(|route| route.capabilities().contains("schedule.delete"))
        .expect("schedule group");
    let handoff = DurableHandoffSignal::new();
    let request = EndpointHostCall {
        rpc_id: "rpc-delete-1".to_owned(),
        operation: "schedule.delete".to_owned(),
        payload: IJsonValue::parse_str(r#"{"taskId":"018f0000-0000-7000-8000-000000000011"}"#)
            .expect("payload"),
        recovering: true,
        handoff: handoff.clone(),
    };
    let payload: Value =
        serde_json::from_slice(&request.payload.canonical_bytes().expect("bytes")).expect("value");
    route
        .validate_extension_payload(&request.operation, &payload)
        .expect("closed payload");
    route
        .execute(&request, &payload, "client-a")
        .expect("execute");
    assert!(handoff.is_durable());
    assert_eq!(
        route.extension_method_class("schedule.delete"),
        Some(MethodClass::Mutation)
    );
    assert_eq!(
        authority.calls.lock().expect("calls").as_slice(),
        &[(
            "schedule.delete".to_owned(),
            "client-a".to_owned(),
            "rpc-delete-1".to_owned(),
            true
        )]
    );
}

#[test]
fn closed_payload_rejects_unknown_fields_before_dispatch() {
    let authority: Arc<dyn ClientExtensionAuthority> = Arc::new(RecordingAuthority::default());
    let route = common_capability_routes(authority)
        .into_iter()
        .find(|route| route.capabilities().contains("usage.summary"))
        .expect("usage group");
    let payload =
        serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000001","extra":true});
    let error = route
        .validate_extension_payload("usage.summary", &payload)
        .expect_err("unknown field");
    assert_eq!(error.code, "bad-request");
}

#[test]
fn registry_groups_match_the_canonical_extension_catalog() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/client-extensions/catalog.canonical.json");
    let value: Value =
        serde_json::from_slice(&fs::read(fixture).expect("catalog fixture")).expect("catalog JSON");
    let expected_groups = [
        ("resources.v1", RESOURCE_METHODS.as_slice()),
        ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
        ("tools.v1", TOOL_METHODS.as_slice()),
        ("plugins.v1", PLUGIN_METHODS.as_slice()),
        ("mcp.v1", MCP_METHODS.as_slice()),
        ("schedule.v1", SCHEDULE_METHODS.as_slice()),
        ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
        ("usage.v1", USAGE_METHODS.as_slice()),
    ];
    for (id, methods) in expected_groups {
        let fixture_group = value["capabilities"]
            .as_array()
            .expect("capabilities")
            .iter()
            .find(|group| group["id"] == id)
            .expect("group");
        let fixture_methods = fixture_group["methods"]
            .as_array()
            .expect("methods")
            .iter()
            .map(|method| {
                let class = match method["class"].as_str().expect("class") {
                    "read" => MethodClass::ReadOnly,
                    "mutation" => MethodClass::Mutation,
                    value => panic!("unknown fixture class {value}"),
                };
                (method["name"].as_str().expect("name"), class)
            })
            .collect::<Vec<_>>();
        assert_eq!(fixture_methods, methods);
    }
    assert!(!value.to_string().contains("computerUse/"));
}
