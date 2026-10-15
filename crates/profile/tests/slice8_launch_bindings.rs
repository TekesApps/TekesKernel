use profile::{
    ConfigSnapshot, DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource,
    DynamicToolSourceKind, LaunchBindings, ProvidersConfig, ResolvedWorkspace, RevisionVector,
    SchemaRefusal, SettingsConfig, WorkspacePolicy,
};
use schema::IJsonValue;
use serde_json::json;

const DYNAMIC_CATALOG_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/launch-bindings/dynamic-catalog.canonical.json");

#[test]
fn declared_dynamic_catalog_fixture_is_the_canonical_byte_authority() {
    let catalog = DynamicToolCatalog::decode(DYNAMIC_CATALOG_FIXTURE).expect("fixture decode");
    assert_eq!(
        catalog.canonical_bytes().expect("fixture encode"),
        DYNAMIC_CATALOG_FIXTURE
    );
    assert_eq!(catalog.tools[0].name, "acme.status");
    assert_eq!(catalog.tools[0].source.kind, DynamicToolSourceKind::Plugin);
}

#[test]
fn host_goal_and_dynamic_catalog_are_exact_snapshot_bound_bytes() {
    let directory = tempfile::tempdir().expect("workspace");
    let config = config(directory.path(), vec!["camera.snap".to_owned()]);
    let tool = dynamic_tool(
        "camera.snap",
        DynamicToolSourceKind::Plugin,
        "com.tekes.camera",
        false,
    );
    let bindings = LaunchBindings::bind(
        &config,
        Some("goal-host-1".to_owned()),
        DynamicToolCatalog::resolve(vec![tool]).expect("catalog"),
    )
    .expect("bindings");
    let bytes = bindings.canonical_bytes().expect("canonical bindings");
    let digest = bindings.digest().expect("digest");
    let decoded = LaunchBindings::decode_verified(&bytes, &digest, &config).expect("decode");
    assert_eq!(decoded, bindings);
    assert_eq!(decoded.goal_id.as_deref(), Some("goal-host-1"));

    let mut wrong_snapshot = config.clone();
    wrong_snapshot.workspace.name = "different".to_owned();
    assert!(LaunchBindings::decode(&bytes, &wrong_snapshot).is_err());
    assert!(LaunchBindings::decode_verified(&bytes, &"0".repeat(64), &config).is_err());
}

#[test]
fn goal_identity_and_allowed_catalog_references_fail_closed() {
    let directory = tempfile::tempdir().expect("workspace");
    let config = config(directory.path(), vec!["missing.dynamic".to_owned()]);
    let empty = DynamicToolCatalog::resolve(vec![]).expect("empty catalog");
    assert!(LaunchBindings::bind(&config, None, empty.clone()).is_err());

    let mut no_allowed = config.clone();
    no_allowed.workspace.policy.allowed_tools.clear();
    assert!(LaunchBindings::bind(&no_allowed, Some(String::new()), empty).is_err());
}

#[test]
fn dynamic_catalog_rejects_fixed_and_dynamic_collisions_and_has_stable_order() {
    let fixed_collision = dynamic_tool("read", DynamicToolSourceKind::Mcp, "project-tools", true);
    assert!(DynamicToolCatalog::resolve(vec![fixed_collision]).is_err());

    let first = dynamic_tool("zeta", DynamicToolSourceKind::Plugin, "a.plugin", false);
    let duplicate = DynamicTool {
        source: DynamicToolSource {
            kind: DynamicToolSourceKind::Mcp,
            id: "remote".to_owned(),
        },
        ..first.clone()
    };
    assert!(DynamicToolCatalog::resolve(vec![first.clone(), duplicate]).is_err());

    let second = dynamic_tool("beta", DynamicToolSourceKind::Mcp, "project-tools", true);
    let third = dynamic_tool("alpha", DynamicToolSourceKind::Plugin, "a.plugin", false);
    let catalog = DynamicToolCatalog::resolve(vec![first, third, second]).expect("sorted catalog");
    assert_eq!(
        catalog
            .tools
            .iter()
            .map(|tool| (
                tool.source.kind,
                tool.source.id.as_str(),
                tool.name.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            (DynamicToolSourceKind::Plugin, "a.plugin", "alpha"),
            (DynamicToolSourceKind::Plugin, "a.plugin", "zeta"),
            (DynamicToolSourceKind::Mcp, "project-tools", "beta"),
        ]
    );
}

#[test]
fn schema_name_and_digest_are_validated() {
    let mut tool = dynamic_tool(
        "camera.snap",
        DynamicToolSourceKind::Plugin,
        "camera",
        false,
    );
    tool.schema_digest = "sha256-deadbeef".to_owned();
    assert!(DynamicToolCatalog::resolve(vec![tool]).is_err());

    let schema = ijson(json!({
        "name":"different",
        "description":"Capture a frame",
        "parameters":{"type":"object","properties":{},"required":[],"additionalProperties":false}
    }));
    assert!(
        DynamicTool::declared(
            "camera.snap",
            DynamicToolSource {
                kind: DynamicToolSourceKind::Plugin,
                id: "camera".to_owned(),
            },
            DynamicToolEffect::ComputerControl,
            false,
            vec![],
            schema,
        )
        .is_err()
    );
}

fn config(path: &std::path::Path, allowed_tools: Vec<String>) -> ConfigSnapshot {
    ConfigSnapshot {
        format: 1,
        workspace: ResolvedWorkspace {
            environment: None,
            folders: None,
            format: 1,
            revision: 1,
            id: "ws".to_owned(),
            name: "Workspace".to_owned(),
            folder_binding: None,
            selected_cwd: None,
            cwd: vec![
                path.canonicalize()
                    .expect("canonical workspace")
                    .to_string_lossy()
                    .into_owned(),
            ],
            policy: WorkspacePolicy {
                allowed_tools,
                ..WorkspacePolicy::default()
            },
        },
        providers: ProvidersConfig::default(),
        legacy_integrations: (),
        settings: SettingsConfig::default(),
        session_settings: None,
        revisions: RevisionVector {
            environments: None,
            workspace: 1,
            providers: 0,
            legacy_integrations: (),
            settings: 0,
            session_settings: None,
        },
    }
}

fn dynamic_tool(
    name: &str,
    kind: DynamicToolSourceKind,
    source: &str,
    always_on: bool,
) -> DynamicTool {
    DynamicTool::declared(
        name,
        DynamicToolSource {
            kind,
            id: source.to_owned(),
        },
        DynamicToolEffect::ComputerControl,
        always_on,
        vec!["camera".to_owned()],
        ijson(json!({
            "name":name,
            "description":"Capture a frame",
            "parameters":{
                "type":"object",
                "properties":{"quality":{"type":"integer","minimum":1,"maximum":10}},
                "required":["quality"],
                "additionalProperties":false
            }
        })),
    )
    .expect("dynamic tool")
}

fn ijson(value: serde_json::Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(&value).expect("canonical JSON"))
        .expect("I-JSON")
}

#[test]
fn dynamic_description_preserves_multiline_text_and_rejects_non_text_controls() {
    for (description, valid) in [
        (
            "Search documentation.\n\tUse keywords.\r\nReturn results.",
            true,
        ),
        ("bad\u{0000}text", false),
        ("bad\u{001b}text", false),
        (" \n\t", false),
    ] {
        let schema = ijson(json!({"name":"remote.search","description":description,
            "parameters":{"type":"object","properties":{},"required":[],"additionalProperties":false}}));
        let result = DynamicTool::declared(
            "remote.search",
            DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp,
                id: "remote".into(),
            },
            DynamicToolEffect::ReadOnly,
            false,
            vec![],
            schema.clone(),
        );
        assert_eq!(result.is_ok(), valid, "{description:?}");
        if valid {
            assert_eq!(result.unwrap().schema, schema);
        }
    }
}

#[test]
fn mcp_schema_defaults_preserve_remote_bytes_without_relaxing_plugin_contract() {
    for parameters in [
        json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{}}),
        json!({"type":"object","properties":{"query":{"type":"string"}},"required":["query"],"additionalProperties":true}),
        json!({"type":"object","properties":{},"additionalProperties":{"type":"string"}}),
    ] {
        for (kind, valid) in [
            (DynamicToolSourceKind::Mcp, true),
            (DynamicToolSourceKind::Plugin, false),
        ] {
            let schema = ijson(
                json!({"name":"remote.search","description":"Search","parameters":parameters}),
            );
            let result = DynamicTool::declared(
                "remote.search",
                DynamicToolSource {
                    kind,
                    id: "remote".into(),
                },
                DynamicToolEffect::ReadOnly,
                false,
                vec![],
                schema.clone(),
            );
            assert_eq!(result.is_ok(), valid);
            if valid {
                assert_eq!(result.unwrap().schema, schema);
            }
        }
    }
    for parameters in [
        json!({"type":"object","properties":{},"required":false}),
        json!({"type":"object","properties":{},"additionalProperties":"yes"}),
        json!({"type":"object","properties":{},"$schema":"https://example.invalid/schema"}),
    ] {
        assert!(
            DynamicTool::declared(
                "remote.search",
                DynamicToolSource {
                    kind: DynamicToolSourceKind::Mcp,
                    id: "remote".into()
                },
                DynamicToolEffect::ReadOnly,
                false,
                vec![],
                ijson(
                    json!({"name":"remote.search","description":"Search","parameters":parameters})
                )
            )
            .is_err()
        );
    }
}

/// The keyword contract for generator-emitted schemas (pydantic, zod,
/// OpenAPI): each accepted keyword projects byte-for-byte, and anything
/// outside the contract is refused by keyword name and JSON pointer.
#[test]
fn dynamic_schema_keywords_accept_generator_output_and_name_the_refused_node() {
    let declare = |kind, parameters: serde_json::Value| {
        DynamicTool::declared(
            "remote.search",
            DynamicToolSource {
                kind,
                id: "remote".into(),
            },
            DynamicToolEffect::ReadOnly,
            false,
            vec![],
            ijson(json!({"name":"remote.search","description":"Search","parameters":parameters})),
        )
    };
    let accepted = [
        // pydantic: title everywhere, Optional as anyOf/null, $defs + local $ref, dict args
        json!({"$defs":{"Point":{"properties":{"x":{"title":"X","type":"number"}},"required":["x"],"title":"Point","type":"object"}},
               "properties":{"layout":{"anyOf":[{"type":"string"},{"type":"null"}],"default":null,"title":"Layout"},
                             "start":{"$ref":"#/$defs/Point"},
                             "extra":{"title":"Extra","type":"object","additionalProperties":{"type":"string"}},
                             "raw":{"title":"Raw","type":"object"}},
               "required":["start"],"title":"moveArguments","type":"object"}),
        // zod / OpenAPI flavoured bounds and annotations
        json!({"type":"object","properties":{
                 "tags":{"type":"array","items":{"type":"string","pattern":"^[a-z]+$","format":"slug"},"uniqueItems":true,"examples":[["a"]]},
                 "count":{"type":"integer","exclusiveMinimum":0,"exclusiveMaximum":10,"nullable":true,"deprecated":false},
                 "shape":{"allOf":[{"$ref":"#/definitions/Base"}],"$comment":"legacy"}},
               "definitions":{"Base":{"type":"object","properties":{}}}}),
    ];
    for parameters in accepted {
        let schema =
            ijson(json!({"name":"remote.search","description":"Search","parameters":parameters}));
        let tool =
            declare(DynamicToolSourceKind::Mcp, parameters).expect("generator keywords project");
        assert_eq!(tool.schema, schema);
    }
    let refused = [
        (
            json!({"type":"object","properties":{"layout":{"type":"string","not":{"const":"model"}}}}),
            "/parameters/properties/layout: uses unsupported keyword \"not\"",
        ),
        (
            json!({"type":"object","properties":{"p":{"$ref":"#/$defs/Missing"}}}),
            "/parameters/properties/p: $ref \"#/$defs/Missing\" does not name a root $defs/definitions entry",
        ),
        (
            json!({"type":"object","properties":{"p":{"$ref":"https://example.invalid/schema.json"}}}),
            "does not name a root $defs/definitions entry",
        ),
        (
            json!({"type":"object","properties":{"p":{"type":"object","properties":{},"$defs":{}}}}),
            "/parameters/properties/p: keyword \"$defs\" is only accepted at the root",
        ),
        (
            json!({"type":"object","properties":{"p":{"type":"string","format":7}}}),
            "/parameters/properties/p: format must be a string",
        ),
        (
            json!({"type":"object","properties":{"p":{"type":"array","items":{"type":"string"},"uniqueItems":"yes"}}}),
            "/parameters/properties/p: uniqueItems must be a boolean",
        ),
        (
            json!({"type":"object","properties":{"p":{"anyOf":[{"type":"string"},{"type":"string","if":{}}]}}}),
            "/parameters/properties/p/anyOf/1: uses unsupported keyword \"if\"",
        ),
    ];
    for (parameters, expected) in refused {
        let error = declare(DynamicToolSourceKind::Mcp, parameters)
            .expect_err(expected)
            .to_string();
        assert!(error.contains(expected), "{error}\n  expected: {expected}");
    }
    // Plugin schemas stay closed: no $defs, no schema-valued additionalProperties,
    // no property-less objects.
    for parameters in [
        json!({"type":"object","properties":{},"required":[],"additionalProperties":false,"$defs":{}}),
        json!({"type":"object","properties":{"p":{"type":"object","properties":{},"required":[],"additionalProperties":{"type":"string"}}},"required":[],"additionalProperties":false}),
        json!({"type":"object","properties":{"p":{"type":"object"}},"required":[],"additionalProperties":false}),
    ] {
        assert!(declare(DynamicToolSourceKind::Plugin, parameters).is_err());
    }
}

#[test]
fn toolchain_selection_changes_require_respawn() {
    let directory = tempfile::tempdir().unwrap();
    let previous = config(directory.path(), vec![]);
    let mut current = previous.clone();
    current.workspace.policy.toolchain_roots =
        vec![directory.path().to_string_lossy().into_owned()];
    assert!(current.requires_respawn_from(&previous));
    assert!(previous.requires_respawn_from(&current));
    assert!(!current.requires_respawn_from(&current));
}

#[test]
fn retired_memory_selectors_do_not_break_historical_workspace_launches() {
    let directory = tempfile::tempdir().unwrap();
    let snapshot = config(
        directory.path(),
        vec!["note".into(), "read".into(), "recall".into()],
    );
    let bindings = LaunchBindings::bind(
        &snapshot,
        None,
        DynamicToolCatalog::resolve(vec![]).unwrap(),
    )
    .unwrap();
    let bytes = bindings.canonical_bytes().unwrap();
    LaunchBindings::decode(&bytes, &snapshot).unwrap();
}

/// `SchemaRefusal` splits the walk's `<pointer>: <message>` text; the walk and
/// the split are tested together so a changed message cannot silently drop the
/// pointer or keyword from a per-tool diagnostic.
#[test]
fn schema_refusal_splits_pointer_keyword_and_message_from_real_refusals() {
    for (parameters, pointer, keyword, message) in [
        (
            json!({"type":"object","properties":{"p":{"type":"object","maxProperties":2,"properties":{}}}}),
            Some("/parameters/properties/p"),
            Some("maxProperties"),
            "uses unsupported keyword \"maxProperties\"",
        ),
        (
            json!({"type":"object","properties":{"p":{"type":"object","properties":{},"$defs":{}}}}),
            Some("/parameters/properties/p"),
            Some("$defs"),
            "keyword \"$defs\" is only accepted at the root of an MCP parameters schema",
        ),
        (
            json!({"type":"object","properties":{"p":{"type":"string","format":7}}}),
            Some("/parameters/properties/p"),
            None,
            "format must be a string",
        ),
    ] {
        let error = DynamicTool::declared(
            "mcp__probe__tool",
            DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp,
                id: "probe".to_owned(),
            },
            DynamicToolEffect::ReadOnly,
            false,
            Vec::new(),
            ijson(json!({"name":"mcp__probe__tool","description":"Probe","parameters":parameters})),
        )
        .expect_err("refused schema");
        let refusal = SchemaRefusal::from_error(&error);
        assert_eq!(refusal.pointer.as_deref(), pointer, "{error}");
        assert_eq!(refusal.keyword.as_deref(), keyword, "{error}");
        assert_eq!(refusal.message, message);
    }
    // A refusal without a pointer keeps the whole sentence.
    let error = DynamicTool::declared(
        "mcp__probe__tool",
        DynamicToolSource {
            kind: DynamicToolSourceKind::Mcp,
            id: "probe".to_owned(),
        },
        DynamicToolEffect::ReadOnly,
        false,
        Vec::new(),
        ijson(json!({"name":"mcp__probe__tool","description":"","parameters":{"type":"object","properties":{}}})),
    )
    .expect_err("blank description");
    let refusal = SchemaRefusal::from_error(&error);
    assert_eq!(refusal.pointer, None);
    assert!(
        refusal.message.contains("description"),
        "{}",
        refusal.message
    );
}

/// `unavailable_tools` excuses a policy reference to a refused tool, and only
/// that: it never excuses an unknown name, never lists an allowed-and-bound
/// tool, and is absent (so bytes are unchanged) when empty.
#[test]
fn unavailable_tools_excuse_only_allowed_unbound_names_and_keep_empty_bytes() {
    let directory = tempfile::tempdir().expect("workspace");
    let tool = dynamic_tool(
        "mcp__probe__alpha",
        DynamicToolSourceKind::Mcp,
        "probe",
        false,
    );
    let catalog = DynamicToolCatalog::resolve(vec![tool]).expect("catalog");
    let config = config(
        directory.path(),
        vec![
            "mcp__probe__alpha".to_owned(),
            "mcp__probe__beta".to_owned(),
        ],
    );

    // Unknown to the catalog and not excused: the launch is still refused.
    assert!(LaunchBindings::bind(&config, None, catalog.clone()).is_err());
    // Excused by an exact refused name.
    let bindings = LaunchBindings::bind_with_unavailable(
        &config,
        None,
        catalog.clone(),
        vec!["mcp__probe__beta".to_owned()],
    )
    .expect("refused tool is excused");
    let bytes = bindings.canonical_bytes().expect("canonical");
    let digest = bindings.digest().expect("digest");
    let decoded = LaunchBindings::decode_verified(&bytes, &digest, &config).expect("decode");
    assert_eq!(
        decoded.unavailable_tools,
        vec!["mcp__probe__beta".to_owned()]
    );
    assert!(
        std::str::from_utf8(&bytes)
            .unwrap()
            .contains("\"unavailable_tools\":[\"mcp__probe__beta\"]")
    );

    // Another unknown name is not excused by it.
    let mut other = config.clone();
    other.workspace.policy.allowed_tools = vec![
        "mcp__probe__alpha".to_owned(),
        "mcp__probe__beta".to_owned(),
        "mcp__probe__gamma".to_owned(),
    ];
    assert!(
        LaunchBindings::bind_with_unavailable(
            &other,
            None,
            catalog.clone(),
            vec!["mcp__probe__beta".to_owned()],
        )
        .is_err()
    );
    // A bound (executable) tool cannot be listed as unavailable, and a name
    // the policy does not allow cannot be listed at all.
    assert!(
        LaunchBindings::bind_with_unavailable(
            &config,
            None,
            catalog.clone(),
            vec![
                "mcp__probe__alpha".to_owned(),
                "mcp__probe__beta".to_owned()
            ],
        )
        .is_err()
    );
    let mut narrower = config.clone();
    narrower.workspace.policy.allowed_tools = vec!["mcp__probe__alpha".to_owned()];
    assert!(
        LaunchBindings::bind_with_unavailable(
            &narrower,
            None,
            catalog.clone(),
            vec!["mcp__probe__beta".to_owned()],
        )
        .is_err()
    );
    // Empty: no field, same bytes as before this contract existed.
    let mut plain = config;
    plain.workspace.policy.allowed_tools = vec!["mcp__probe__alpha".to_owned()];
    let plain = LaunchBindings::bind(&plain, None, catalog).expect("plain");
    assert!(
        !std::str::from_utf8(&plain.canonical_bytes().unwrap())
            .unwrap()
            .contains("unavailable_tools")
    );
}

/// Like `goal_id: null`, an explicit empty `unavailable_tools` is a second byte
/// form (and digest) of the same value; the host never emits it, so decode
/// refuses it.
#[test]
fn explicit_empty_unavailable_tools_is_rejected_like_a_null_goal() {
    let directory = tempfile::tempdir().expect("workspace");
    let config = config(directory.path(), vec!["camera.snap".to_owned()]);
    let tool = dynamic_tool(
        "camera.snap",
        DynamicToolSourceKind::Plugin,
        "com.tekes.camera",
        false,
    );
    let bindings = LaunchBindings::bind(
        &config,
        None,
        DynamicToolCatalog::resolve(vec![tool]).expect("catalog"),
    )
    .expect("bindings");
    let mut value: serde_json::Value =
        serde_json::from_slice(&bindings.canonical_bytes().expect("bytes")).expect("json");
    value["unavailable_tools"] = json!([]);
    let mut bytes = serde_json_canonicalizer::to_vec(&value).expect("canonical");
    bytes.push(b'\n');
    let error = LaunchBindings::decode(&bytes, &config).expect_err("explicit empty list");
    assert!(error.to_string().contains("unavailable_tools"), "{error}");
    // The omitted form still decodes.
    LaunchBindings::decode(&bindings.canonical_bytes().unwrap(), &config).expect("omitted");
}

#[test]
fn launch_tool_name_rule_matches_the_binding_validation() {
    assert!(profile::is_dynamic_tool_name("mcp__probe__alpha"));
    assert!(profile::is_dynamic_tool_name(&"x".repeat(128)));
    assert!(!profile::is_dynamic_tool_name(&"x".repeat(129)));
    assert!(!profile::is_dynamic_tool_name(""));
    assert!(!profile::is_dynamic_tool_name("a\nb"));
}
