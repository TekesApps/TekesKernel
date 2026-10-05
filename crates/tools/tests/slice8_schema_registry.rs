#[allow(dead_code)]
#[path = "../src/schema_registry.rs"]
mod schema_registry;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use schema_registry::{
    FIXED_SCHEMA_REVISION, canonical_schema_oracle_bytes, fixed_schema_registry,
    schema_oracle_value, validate_fixed_arguments,
};

#[test]
fn quoted_null_repair_requires_one_unique_fully_valid_candidate() {
    let registry = fixed_schema_registry();
    let shell = registry.iter().find(|s| s.name == "shell").unwrap();
    let valid = json!({"working_directory":".","max_duration_ms":1000,"command":"pwd","program":null,
        "args":null,"steps":[],"failure_policy":"stop_on_error","writable_paths":[],"artifact_outputs":[]});
    assert!(shell.repair_quoted_null_exclusive(&valid).is_none());
    let mut broken = valid.clone();
    broken["program"] = json!("null");
    assert!(shell.validate(&broken).is_err());
    assert_eq!(
        shell.repair_quoted_null_exclusive(&broken),
        Some(valid.clone())
    );
    broken["command"] = json!("null");
    assert!(
        shell.repair_quoted_null_exclusive(&broken).is_none(),
        "two valid interpretations are ambiguous"
    );
    broken["program"] = Value::Null;
    assert!(
        shell.repair_quoted_null_exclusive(&broken).is_none(),
        "valid literal command null stays literal"
    );
    broken = valid.clone();
    broken["command"] = json!("null");
    broken["program"] = json!("/bin/pwd");
    let mut repaired = broken.clone();
    repaired["command"] = Value::Null;
    assert_eq!(shell.repair_quoted_null_exclusive(&broken), Some(repaired));
    for other in [json!(""), json!("[]"), json!("python3"), json!(" null ")] {
        broken = valid.clone();
        broken["program"] = other;
        assert!(shell.repair_quoted_null_exclusive(&broken).is_none());
    }
    broken = valid.clone();
    broken["program"] = json!("null");
    broken["max_duration_ms"] = json!("invalid");
    assert!(
        shell.repair_quoted_null_exclusive(&broken).is_none(),
        "other validation errors stay rejected"
    );
}
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(path)
}

fn valid_arguments() -> BTreeMap<&'static str, Value> {
    BTreeMap::from([
        (
            "apply_patch",
            json!({"operation":"update_file","path":"a.txt","diff":"@@\n-old\n+new","expected_artifact_version":0,"summary":"update"}),
        ),
        (
            "ask_user_questions",
            json!({"question":"Continue?","options":["yes","no"]}),
        ),
        (
            "context",
            json!({"operation":"threads","reason":"inspect related work"}),
        ),
        (
            "context_get",
            json!({"record_ids":[1],"reason":"read exact evidence"}),
        ),
        (
            "edit",
            json!({"path":"a.txt","old_text":"before","new_text":"after"}),
        ),
        ("glob", json!({"pattern":"**/*.rs","path":null})),
        (
            "grep",
            json!({"pattern":"needle","path":null,"glob":null,"output_mode":null,"case_insensitive":null,"context_lines":null}),
        ),
        ("job", json!({"action":"list"})),
        (
            "new_goal",
            json!({"goal":"finish","completion_criteria":null,"reason":"requested"}),
        ),
        ("plan", json!({"plan":"1. Verify"})),
        (
            "read",
            json!({"path":"src/lib.rs","offset":null,"limit":null}),
        ),
        ("report", json!({"result":"ok"})),
        (
            "set_goal_state",
            json!({"state":"active","progress":null,"reason":null,"user_action":null}),
        ),
        ("shell", json!({"command":"pwd"})),
        ("write", json!({"path":"a.txt","content":"hello\n"})),
        ("skill", json!({"skill":"rust"})),
        ("skill_explorer", json!({"query":"rust","limit":5})),
        (
            "subagent",
            json!({"brief":"audit the files","report_back_session_id":""}),
        ),
        (
            "summary_artifact",
            json!({"continuation":"continue from evidence","evidence_refs":[7,9]}),
        ),
        (
            "task",
            json!({"task_name":"audit","goal":"find defects","instructions":"inspect inputs","input_sources":[],"output":{"mode":"inline","format":"text","name":"result","path":"","contract":"findings"}}),
        ),
        ("think", json!({"thought":"check invariant"})),
        ("tool_search", json!({"query":"database","limit":5})),
        (
            "verify",
            json!({"covered_set":[{"id":"artifact-1","dedup_key":"v1"}],"verdict":"pass","failures":[]}),
        ),
        ("web_fetch", json!({"url":"https://example.com"})),
        (
            "web_search",
            json!({"query":"Tekes","max_results":5,"topic":"general"}),
        ),
    ])
}

/// Auxiliary shell fields are optional. Older or strict-profile callers may
/// still send null for unused lists; null and [] remain accepted.
#[test]
fn shell_list_properties_accept_null_as_the_empty_list() {
    let registry = fixed_schema_registry();
    let shell = registry.iter().find(|s| s.name == "shell").unwrap();
    let base = json!({"working_directory":".","max_duration_ms":1000,"command":"pwd",
        "program":null,"args":null,"steps":null,"failure_policy":"stop_on_error",
        "writable_paths":null,"artifact_outputs":null});
    shell.validate(&base).expect("null lists are accepted");
    for key in ["steps", "writable_paths", "artifact_outputs"] {
        let mut empty = base.clone();
        empty[key] = json!([]);
        shell.validate(&empty).expect("the empty list stays valid");
        let mut wrong = base.clone();
        wrong[key] = json!("nope");
        assert!(
            shell.validate(&wrong).is_err(),
            "{key} must still reject a non-list"
        );
    }
}

#[test]
fn registry_matches_manifest_names_and_argument_order_exactly() {
    let manifest: Value = serde_json::from_slice(
        &fs::read(fixture("tools/builtin-tools.canonical.json")).expect("builtin manifest"),
    )
    .expect("manifest JSON");
    let manifest_tools = manifest["tools"].as_array().expect("tools array");
    let registry = fixed_schema_registry();

    assert_eq!(registry.len(), 25);
    assert_eq!(registry.len(), manifest_tools.len());
    for (schema, entry) in registry.iter().zip(manifest_tools) {
        assert_eq!(schema.name, entry["name"].as_str().unwrap());
        let arguments = entry["arguments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(schema.parameters.argument_order(), arguments);
        let ordered = String::from_utf8(schema.ordered_model_schema_bytes()).unwrap();
        let properties_start = ordered.find("\"properties\":{").unwrap();
        let mut cursor = properties_start;
        for argument in &arguments {
            let position = ordered[cursor..]
                .find(&format!("\"{argument}\":"))
                .unwrap_or_else(|| panic!("{} omitted ordered property {argument}", schema.name));
            cursor += position + argument.len() + 3;
        }
        assert!(!schema.description.is_empty());
        assert_eq!(schema.revision, FIXED_SCHEMA_REVISION);
        assert_eq!(schema.schema_digest().len(), "sha256-".len() + 64);
        assert_eq!(
            schema.model_schema()["parameters"]["additionalProperties"],
            false
        );
    }
}

#[test]
fn every_fixed_schema_accepts_a_minimal_positive_and_rejects_unknown_fields() {
    let examples = valid_arguments();
    assert_eq!(examples.len(), 25);
    for (name, arguments) in examples {
        validate_fixed_arguments(name, &arguments)
            .unwrap_or_else(|error| panic!("{name} positive example failed: {error}"));
        let mut with_unknown = arguments.as_object().unwrap().clone();
        with_unknown.insert("unexpected".to_owned(), Value::Bool(true));
        assert!(
            validate_fixed_arguments(name, &Value::Object(with_unknown)).is_err(),
            "{name} accepted an unknown top-level property"
        );
    }
}

#[test]
fn explicit_string_enum_and_bounds_constraints_are_enforced() {
    assert!(validate_fixed_arguments("apply_patch", &json!({"operation":"delete_file","path":"a","diff":"x","expected_artifact_version":0,"summary":"x"})).is_err());
    assert!(validate_fixed_arguments("apply_patch", &json!({"operation":"create_file","path":"a","diff":"x","expected_artifact_version":-1,"summary":"x"})).is_err());
    assert!(
        validate_fixed_arguments("context_get", &json!({"record_ids":[],"reason":"audit"}))
            .is_err()
    );
    assert!(
        validate_fixed_arguments(
            "context_get",
            &json!({"record_ids":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17],"reason":"audit"})
        )
        .is_err()
    );
    assert!(
        validate_fixed_arguments("context_get", &json!({"record_ids":[0],"reason":"audit"}))
            .is_err()
    );
    assert!(
        validate_fixed_arguments(
            "context_get",
            &json!({"record_ids":[1],"reason":"audit","page_bytes":16385})
        )
        .is_err()
    );
    assert!(validate_fixed_arguments("skill_explorer", &json!({"query":"x","limit":0})).is_err());
    assert!(validate_fixed_arguments("tool_search", &json!({"query":"x","limit":11})).is_err());
    assert!(
        validate_fixed_arguments(
            "web_search",
            &json!({"query":"x","max_results":5,"topic":"images"})
        )
        .is_err()
    );
    assert!(validate_fixed_arguments("web_fetch", &json!({"url":"file:///etc/passwd"})).is_err());
}

#[test]
fn explicit_uniqueness_and_conditional_constraints_are_enforced() {
    assert!(validate_fixed_arguments("ask_user_questions", &json!({"question":""})).is_err());
    assert!(
        validate_fixed_arguments(
            "ask_user_questions",
            &json!({"question":"x","options":["same","same"]})
        )
        .is_err()
    );
    assert!(
        validate_fixed_arguments("context", &json!({"operation":"threads","reason":""})).is_err()
    );
    assert!(validate_fixed_arguments("job", &json!({"action":"start"})).is_err());
    assert!(validate_fixed_arguments("job", &json!({"action":"stop"})).is_err());
    assert!(
        validate_fixed_arguments(
            "set_goal_state",
            &json!({"state":"blocked","progress":null,"reason":"why","user_action":null})
        )
        .is_err()
    );
    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert("program".to_owned(), json!("/bin/pwd"));
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_err());
    assert!(
        validate_fixed_arguments(
            "verify",
            &json!({
                "covered_set":[],"verdict":"inconclusive","failures":[]
            })
        )
        .is_ok()
    );
    let mut quoted_null = valid_arguments()["shell"].as_object().unwrap().clone();
    quoted_null.insert("program".to_owned(), json!("null"));
    let error = validate_fixed_arguments("shell", &Value::Object(quoted_null.clone()))
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("provide exactly one of command, program"),
        "{error}"
    );
    quoted_null.insert("program".to_owned(), Value::Null);
    assert!(validate_fixed_arguments("shell", &Value::Object(quoted_null)).is_ok());
    // The schema puts no ceiling on the shell budget (the runtime clamps a
    // large request to its cap) and null means "use the default"; only a zero
    // or negative budget is meaningless.
    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert("max_duration_ms".to_owned(), json!(3_600_000));
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_ok());
    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert("max_duration_ms".to_owned(), Value::Null);
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_ok());
    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert("max_duration_ms".to_owned(), json!(0));
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_err());
    assert!(
        validate_fixed_arguments(
            "verify",
            &json!({"covered_set":[{"id":"a","dedup_key":"v"}],"verdict":"fail","failures":[]})
        )
        .is_err()
    );
    assert!(
        validate_fixed_arguments(
            "verify",
            &json!({"covered_set":[{"id":"a","dedup_key":"v"}],"verdict":"pass","failures":[{"id":"a","issues":["bad"],"guidance":["fix"]}]})
        )
        .is_err()
    );
}

#[test]
fn explicit_nested_task_output_is_closed_and_file_path_is_project_relative() {
    let base =
        json!({"task_name":"build","goal":"build","instructions":"do work","input_sources":[]});
    let mut arguments = base.as_object().unwrap().clone();
    arguments.insert(
        "output".to_owned(),
        json!({"mode":"file","format":"json","name":"report","path":"out/report.json","contract":"report-v1"}),
    );
    validate_fixed_arguments("task", &Value::Object(arguments.clone())).unwrap();

    arguments.insert(
        "output".to_owned(),
        json!({"mode":"file","format":"json","name":"report","contract":"report-v1"}),
    );
    assert!(validate_fixed_arguments("task", &Value::Object(arguments.clone())).is_err());
    arguments.insert(
        "output".to_owned(),
        json!({"mode":"file","format":"json","name":"report","path":"../outside","contract":"report-v1"}),
    );
    assert!(validate_fixed_arguments("task", &Value::Object(arguments.clone())).is_err());
    arguments.insert("output".to_owned(), json!({"mode":"inline","format":"text","name":"result","path":"","contract":"result","extra":true}));
    assert!(validate_fixed_arguments("task", &Value::Object(arguments)).is_err());
}

#[test]
fn schema_digests_are_deterministic_and_unique() {
    let first = fixed_schema_registry()
        .into_iter()
        .map(|schema| (schema.name, schema.schema_digest()))
        .collect::<BTreeMap<_, _>>();
    let second = fixed_schema_registry()
        .into_iter()
        .map(|schema| (schema.name, schema.schema_digest()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(first, second);
    assert_eq!(first.values().collect::<BTreeSet<_>>().len(), 25);
    assert_eq!(
        schema_oracle_value()["schemas"].as_array().unwrap().len(),
        25
    );
}

#[test]
fn canonical_guidance_oracle_matches_registry_bytes() {
    let fixture_bytes = fs::read(fixture("tools/builtin-tool-guidance.canonical.json"))
        .expect("guidance oracle fixture");
    assert_eq!(fixture_bytes, tools::canonical_guidance_oracle_bytes());
    let decoded: Value = serde_json::from_slice(&fixture_bytes).unwrap();
    assert_eq!(decoded["format"], 4);
    assert!(
        decoded.get("guidance").is_none(),
        "format 4 carries no per-tool paragraphs"
    );
    let preimage = json!({
        "harness_identity": decoded["harness_identity"],
        "coding_profile": decoded["coding_profile"],
        "general_profile": decoded["general_profile"],
        "working_directory": decoded["working_directory"],
    });
    let canonical = serde_json_canonicalizer::to_vec(&preimage).unwrap();
    assert_eq!(
        decoded["digest"],
        format!("sha256-{:x}", Sha256::digest(canonical))
    );
    assert_eq!(decoded["harness_identity"], tools::HARNESS_IDENTITY);
    assert_eq!(decoded["coding_profile"], tools::CODING_PROFILE);
    assert_eq!(decoded["general_profile"], tools::GENERAL_PROFILE);
    assert_eq!(decoded["working_directory"], tools::WORKING_DIRECTORY);
}

#[test]
fn canonical_all_schema_oracle_matches_registry_bytes_and_digests() {
    let fixture_bytes = fs::read(fixture("tools/builtin-tool-schemas.canonical.json"))
        .expect("schema oracle fixture");
    assert_eq!(fixture_bytes, canonical_schema_oracle_bytes());
    let decoded: Value = serde_json::from_slice(&fixture_bytes).unwrap();
    assert_eq!(decoded["revision"], FIXED_SCHEMA_REVISION);
    for entry in decoded["schemas"].as_array().unwrap() {
        let name = entry["name"].as_str().unwrap();
        let compiled = fixed_schema_registry()
            .into_iter()
            .find(|schema| schema.name == name)
            .unwrap();
        assert_eq!(entry["digest"], compiled.schema_digest());
        assert_eq!(entry["description"], compiled.description);
        assert_eq!(entry["parameters"], compiled.model_schema()["parameters"]);
        let preimage = json!({
            "description": entry["description"],
            "name": entry["name"],
            "parameters": entry["parameters"],
        });
        let canonical = serde_json_canonicalizer::to_vec(&preimage).unwrap();
        assert_eq!(
            entry["digest"],
            format!("sha256-{:x}", Sha256::digest(canonical))
        );
    }
}

#[test]
fn every_declared_parameter_object_is_closed_recursively() {
    fn visit(schema: &Value, path: &str) {
        if schema.get("type") == Some(&Value::String("object".to_owned())) {
            assert_eq!(
                schema.get("additionalProperties"),
                Some(&Value::Bool(false)),
                "open object schema at {path}"
            );
        }
        if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
            for (name, property) in properties {
                visit(property, &format!("{path}.properties.{name}"));
            }
        }
        if let Some(items) = schema.get("items") {
            visit(items, &format!("{path}.items"));
        }
        for keyword in ["anyOf", "oneOf"] {
            if let Some(options) = schema.get(keyword).and_then(Value::as_array) {
                for (index, option) in options.iter().enumerate() {
                    visit(option, &format!("{path}.{keyword}[{index}]"));
                }
            }
        }
    }

    for schema in fixed_schema_registry() {
        visit(
            &schema.model_schema()["parameters"],
            &format!("$.{}", schema.name),
        );
    }
}

#[test]
fn context_memory_and_shell_nested_rules_are_executable() {
    assert!(
        validate_fixed_arguments("context", &json!({"operation":"search","reason":"audit"}))
            .is_err()
    );
    assert!(
        validate_fixed_arguments(
            "context",
            &json!({"operation":"search","reason":"audit","query":"x","scope":"thread"})
        )
        .is_err()
    );
    assert!(
        validate_fixed_arguments(
            "context",
            &json!({"operation":"send","reason":"coordinate","thread_id":"thread"})
        )
        .is_err()
    );

    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert(
        "steps".to_owned(),
        json!([{"program":"/bin/true","args":[],"extra":true}]),
    );
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_err());
    let mut shell = valid_arguments()["shell"].as_object().unwrap().clone();
    shell.insert("artifact_outputs".to_owned(), json!([{"path":"../escape"}]));
    assert!(validate_fixed_arguments("shell", &Value::Object(shell)).is_err());
}
