use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tools::{
    Availability, Backend, BuiltinManifest, CatalogContext, CatalogRole, Effect,
    FIXED_SCHEMA_REVISION, fixed_schema_registry, validate_fixed_arguments,
};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn decode(path: &Path) -> Value {
    serde_json::from_slice(
        &fs::read(path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("failed to decode {}: {error}", path.display()))
}

fn canonical_case(path: &Path) -> Value {
    let bytes = fs::read(path).unwrap();
    let body = bytes
        .strip_suffix(b"\n")
        .unwrap_or_else(|| panic!("{} lacks its one framing LF", path.display()));
    assert!(
        !body.contains(&b'\n'),
        "{} is not one JSON line",
        path.display()
    );
    assert!(!body.contains(&b'\r'), "{} contains CR", path.display());
    let value: Value = serde_json::from_slice(body).unwrap();
    assert_eq!(
        serde_json_canonicalizer::to_vec(&value).unwrap(),
        body,
        "{} is not RFC-8785 canonical",
        path.display()
    );
    value
}

fn relative_files(root: &Path, directory: &Path, output: &mut BTreeSet<String>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            relative_files(root, &path, output);
        } else {
            output.insert(
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
}

fn projection_names(manifest: &BuiltinManifest, context: &CatalogContext) -> BTreeSet<String> {
    manifest
        .projection(context)
        .into_iter()
        .map(|tool| tool.name.clone())
        .collect()
}

fn independently_visible(availability: Availability, name: &str, context: &CatalogContext) -> bool {
    match availability {
        Availability::Default => true,
        Availability::SelectionControlled => context.selected_tools.contains(name),
        Availability::PlanMode => context.role == CatalogRole::Plan,
        Availability::RoleCompactor => context.role == CatalogRole::Compactor,
        Availability::RoleSubagent => context.role == CatalogRole::Subagent,
        Availability::RoleValidator => context.role == CatalogRole::Validator,
        Availability::ConditionalCredential => context.has_web_credential,
        Availability::ConditionalDeferredCatalog => context.has_deferred_catalog,
    }
}

#[test]
fn slice8_gate_59_tool_dispatcher_catalog_coverage() {
    let fixture_bytes = fs::read(fixtures().join("tools/builtin-tools.canonical.json")).unwrap();
    let manifest = BuiltinManifest::decode_canonical(&fixture_bytes).unwrap();
    assert_eq!(manifest, BuiltinManifest::compiled());
    assert_eq!(manifest.tools.len(), 25);

    let runtime_registry = canonical_case(&fixtures().join("tool-runtime/cases.canonical.json"));
    let manifest_names = manifest
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect::<BTreeSet<_>>();
    let runtime_names = runtime_registry["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let schema_names = fixed_schema_registry()
        .into_iter()
        .map(|schema| schema.name.to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(manifest_names.len(), 25);
    assert_eq!(runtime_names, manifest_names);
    assert_eq!(schema_names, manifest_names);

    let mut availability_profiles = BTreeSet::new();
    let selected_tools = manifest
        .tools
        .iter()
        .filter(|tool| tool.availability == Availability::SelectionControlled)
        .map(|tool| tool.name.clone())
        .collect::<BTreeSet<_>>();
    for tool in &manifest.tools {
        availability_profiles.insert(format!("{:?}", tool.availability));
        let case = canonical_case(&fixtures().join(format!(
            "tool-runtime/tools/{}.case.canonical.json",
            tool.name
        )));
        validate_fixed_arguments(&tool.name, &case["input"]["arguments"]).unwrap();
        assert_eq!(
            case["expected_result"]["backend"],
            serde_json::to_value(tool.backend).unwrap()
        );
        assert_eq!(
            case["expected_result"]["effect"],
            serde_json::to_value(tool.effect).unwrap()
        );
    }
    assert_eq!(availability_profiles.len(), 8);

    let contexts = [
        CatalogContext::default(),
        CatalogContext {
            selected_tools: selected_tools.clone(),
            ..CatalogContext::default()
        },
        CatalogContext {
            role: CatalogRole::Plan,
            ..CatalogContext::default()
        },
        CatalogContext {
            role: CatalogRole::Compactor,
            ..CatalogContext::default()
        },
        CatalogContext {
            role: CatalogRole::Subagent,
            ..CatalogContext::default()
        },
        CatalogContext {
            role: CatalogRole::Validator,
            ..CatalogContext::default()
        },
        CatalogContext {
            has_web_credential: true,
            ..CatalogContext::default()
        },
        CatalogContext {
            has_deferred_catalog: true,
            ..CatalogContext::default()
        },
        CatalogContext {
            role: CatalogRole::Benchmark,
            has_web_credential: true,
            has_deferred_catalog: true,
            selected_tools,
        },
    ];
    for context in contexts {
        let expected = manifest
            .tools
            .iter()
            .filter(|tool| independently_visible(tool.availability, &tool.name, &context))
            .filter(|tool| {
                context.role != CatalogRole::Benchmark
                    || (tool.backend == Backend::InProcess && tool.effect != Effect::ChildSpawn)
            })
            .map(|tool| tool.name.clone())
            .collect::<BTreeSet<_>>();
        assert_eq!(projection_names(&manifest, &context), expected);
    }

    assert!(!projection_names(&manifest, &CatalogContext::default()).contains("web_search"));
    assert!(!projection_names(&manifest, &CatalogContext::default()).contains("tool_search"));
    assert!(
        manifest
            .reject_dynamic_collisions(["dynamic_unique"])
            .is_ok()
    );
    assert!(manifest.reject_dynamic_collisions(["read"]).is_err());
    assert!(
        manifest
            .reject_dynamic_collisions(["dynamic_duplicate", "dynamic_duplicate"])
            .is_err()
    );
}

#[test]
fn tool_runtime_registry_disk_and_global_manifest_are_bidirectional() {
    let root = fixtures().join("tool-runtime");
    let registry = canonical_case(&root.join("cases.canonical.json"));
    let global_manifest = decode(&fixtures().join("manifest.json"));
    let builtin = canonical_case(&fixtures().join("tools/builtin-tools.canonical.json"));

    let registry_tools = registry["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    let builtin_tools = builtin["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(registry_tools, builtin_tools);

    let mut expected_cases = registry_tools
        .iter()
        .map(|name| format!("tools/{name}.case.canonical.json"))
        .collect::<BTreeSet<_>>();
    for row in registry["matrix_cases"].as_array().unwrap() {
        expected_cases.insert(format!(
            "matrix/{}--{}--{}.case.canonical.json",
            row["backend"].as_str().unwrap(),
            row["effect"].as_str().unwrap(),
            row["failure"].as_str().unwrap()
        ));
    }
    let mut disk_cases = BTreeSet::new();
    relative_files(&root, &root, &mut disk_cases);
    disk_cases.retain(|name| name.ends_with(".case.canonical.json"));
    assert_eq!(disk_cases, expected_cases);

    let mut all_disk = BTreeSet::new();
    relative_files(&root, &root, &mut all_disk);
    let declared = global_manifest["corpora"]["tool-runtime"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(all_disk, declared);
}

#[test]
fn every_tool_case_is_canonical_schema_positive_and_exactly_classified() {
    let root = fixtures().join("tool-runtime");
    let registry = canonical_case(&root.join("cases.canonical.json"));
    let builtin = canonical_case(&fixtures().join("tools/builtin-tools.canonical.json"));
    let required_fields = registry["artifact_contract"]["required_case_fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<BTreeSet<_>>();

    for descriptor in builtin["tools"].as_array().unwrap() {
        let name = descriptor["name"].as_str().unwrap();
        let case = canonical_case(&root.join(format!("tools/{name}.case.canonical.json")));
        assert_eq!(
            case.as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            required_fields
        );
        assert_eq!(case["input"]["name"], name);
        validate_fixed_arguments(name, &case["input"]["arguments"])
            .unwrap_or_else(|error| panic!("{name} fixture violates its schema: {error}"));
        assert_eq!(
            case["preconditions"]["availability"],
            descriptor["availability"]
        );
        assert_eq!(case["preconditions"]["backend_dependency"], "ready");
        assert_eq!(
            case["preconditions"]["schema_revision"],
            FIXED_SCHEMA_REVISION
        );
        assert_eq!(case["expected_result"]["backend"], descriptor["backend"]);
        assert_eq!(case["expected_result"]["effect"], descriptor["effect"]);
        assert_eq!(case["expected_result"]["validation"], "accepted");
        assert_eq!(case["crash_points"], Value::Array(vec![]));
        assert_eq!(case["expected_events"], Value::Array(vec![]));
    }
}

#[test]
fn matrix_is_the_smallest_complete_real_pair_axis_cover() {
    let root = fixtures().join("tool-runtime");
    let registry = canonical_case(&root.join("cases.canonical.json"));
    let builtin = canonical_case(&fixtures().join("tools/builtin-tools.canonical.json"));
    assert_eq!(
        registry["artifact_contract"]["matrix_semantics"],
        "minimum-axis-cover"
    );

    let declared = |key: &str| {
        registry[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<BTreeSet<_>>()
    };
    let declared_backends = declared("backends");
    let declared_effects = declared("effects");
    let declared_failures = declared("required_failures");
    let real_pairs = builtin["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| {
            (
                tool["backend"].as_str().unwrap().to_owned(),
                tool["effect"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();

    let rows = registry["matrix_cases"].as_array().unwrap();
    let minimum = declared_backends
        .len()
        .max(declared_effects.len())
        .max(declared_failures.len());
    assert_eq!(rows.len(), minimum);
    let mut rows_seen = BTreeSet::new();
    let mut covered_backends = BTreeSet::new();
    let mut covered_effects = BTreeSet::new();
    let mut covered_failures = BTreeSet::new();
    let required_fields = registry["artifact_contract"]["required_case_fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<BTreeSet<_>>();

    for row in rows {
        let backend = row["backend"].as_str().unwrap();
        let effect = row["effect"].as_str().unwrap();
        let failure = row["failure"].as_str().unwrap();
        assert!(real_pairs.contains(&(backend.to_owned(), effect.to_owned())));
        assert!(rows_seen.insert((backend, effect, failure)));
        covered_backends.insert(backend.to_owned());
        covered_effects.insert(effect.to_owned());
        covered_failures.insert(failure.to_owned());

        let path = root.join(format!(
            "matrix/{backend}--{effect}--{failure}.case.canonical.json"
        ));
        let case = canonical_case(&path);
        assert_eq!(
            case.as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            required_fields
        );
        assert_eq!(&case["input"], row);
        assert_eq!(case["expected_result"]["backend"], backend);
        assert_eq!(case["expected_result"]["effect"], effect);
        assert_eq!(case["expected_result"]["failure"], failure);
        assert_eq!(case["expected_result"]["repeat_effect"], false);
        let events = case["expected_events"].as_array().unwrap();
        assert_eq!(events.first().unwrap()["kind"], "tool_call");
        assert_eq!(events.first().unwrap()["barrier"], true);
        assert_eq!(events.last().unwrap()["terminal"], true);
    }

    assert_eq!(covered_backends, declared_backends);
    assert_eq!(covered_effects, declared_effects);
    assert_eq!(covered_failures, declared_failures);
}
