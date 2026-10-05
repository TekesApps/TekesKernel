//! User-level instruction settings: `root/settings.json` with the closed schema from
//! `spec/instruction-snapshot.md`, exposed to the Client as one settings namespace
//! `policy`. A sidecar `root/settings.revision` (`<n> <sha256>` + LF) carries the
//! revision the Client uses for `expectedRevision`; when the JSON's digest no longer
//! matches the sidecar the file was edited out of band and the revision is bumped.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

use crate::{Failure, closed_object};

pub const NAMESPACE: &str = "policy";
const FIELDS: &[&str] = &[
    "network",
    "allowed_tools",
    "writable_roots",
    "max_wall_seconds",
];

pub fn document_path(root: &Path) -> PathBuf {
    root.join("settings.json")
}

pub fn revision_path(root: &Path) -> PathBuf {
    root.join("settings.revision")
}

pub fn schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "network": { "type": "boolean" },
            "allowed_tools": { "type": "array", "items": { "type": "string" } },
            "writable_roots": { "type": "array", "items": { "type": "string", "format": "absolute-path" } },
            "max_wall_seconds": { "type": "integer", "minimum": 1 }
        }
    })
}

// ---- payload shape --------------------------------------------------------------------

fn validate_namespace(object: &Map<String, Value>) -> Result<(), String> {
    match object.get("ns") {
        Some(Value::String(_)) => Ok(()),
        _ => Err("ns must be a string".to_owned()),
    }
}

fn validate_expected_revision(object: &Map<String, Value>) -> Result<(), String> {
    match object.get("expectedRevision") {
        None | Some(Value::Null) => Ok(()),
        Some(value) if value.as_u64().is_some() => Ok(()),
        Some(_) => Err("expectedRevision must be a non-negative integer".to_owned()),
    }
}

fn validate_path(value: Option<&Value>) -> Result<(), String> {
    let path = value
        .and_then(Value::as_array)
        .ok_or("path must be an array of strings")?;
    if path.is_empty() || !path.iter().all(Value::is_string) {
        return Err("path must be a non-empty array of strings".to_owned());
    }
    Ok(())
}

pub fn validate(method: &str, payload: &Value) -> Result<(), String> {
    match method {
        "settings.describe" | "settings.document" => closed_object(payload, &[]).map(|_| ()),
        "settings.mutate" => {
            let object = closed_object(payload, &["ns", "operations", "expectedRevision"])?;
            validate_namespace(object)?;
            validate_expected_revision(object)?;
            let operations = object
                .get("operations")
                .and_then(Value::as_array)
                .ok_or("operations must be an array")?;
            for operation in operations {
                let op = closed_object(operation, &["op", "path", "value"])?;
                match op.get("op").and_then(Value::as_str) {
                    Some("set") => {
                        validate_path(op.get("path"))?;
                        if !op.contains_key("value") {
                            return Err("set requires value".to_owned());
                        }
                    }
                    Some("unset") => {
                        validate_path(op.get("path"))?;
                        if op.contains_key("value") {
                            return Err("unset takes no value".to_owned());
                        }
                    }
                    _ => return Err("op must be \"set\" or \"unset\"".to_owned()),
                }
            }
            Ok(())
        }
        "settings.update" => {
            let object = closed_object(payload, &["ns", "patch", "expectedRevision"])?;
            validate_namespace(object)?;
            validate_expected_revision(object)?;
            if !object.get("patch").is_some_and(Value::is_object) {
                return Err("patch must be an object".to_owned());
            }
            Ok(())
        }
        other => Err(format!("unknown method {other}")),
    }
}

// ---- policy schema --------------------------------------------------------------------

fn invalid(message: impl Into<String>) -> Failure {
    Failure::new("settings-invalid", message)
}

fn validate_field(key: &str, value: &Value) -> Result<(), Failure> {
    match key {
        "network" => value
            .is_boolean()
            .then_some(())
            .ok_or_else(|| invalid("network must be a boolean")),
        "allowed_tools" => match value.as_array() {
            Some(items) if items.iter().all(Value::is_string) => Ok(()),
            _ => Err(invalid("allowed_tools must be an array of strings")),
        },
        "writable_roots" => match value.as_array() {
            Some(items) if items.iter().all(Value::is_string) => {
                for item in items {
                    let root = item.as_str().unwrap_or_default();
                    if !root.starts_with('/') {
                        return Err(invalid(format!(
                            "writable_roots entry is not absolute: {root}"
                        )));
                    }
                }
                Ok(())
            }
            _ => Err(invalid(
                "writable_roots must be an array of absolute path strings",
            )),
        },
        "max_wall_seconds" => match value.as_u64() {
            Some(seconds) if seconds >= 1 => Ok(()),
            _ => Err(invalid("max_wall_seconds must be an integer >= 1")),
        },
        other => Err(invalid(format!("unknown policy key {other}"))),
    }
}

fn validate_policy(policy: &Map<String, Value>) -> Result<(), Failure> {
    for (key, value) in policy {
        validate_field(key, value)?;
    }
    Ok(())
}

fn validate_settings(settings: &Map<String, Value>) -> Result<(), Failure> {
    for key in settings.keys() {
        if key != "format" && key != "policy" {
            return Err(invalid(format!("unknown settings key {key}")));
        }
    }
    if settings.get("format").and_then(Value::as_u64) != Some(1) {
        return Err(invalid("format must be 1"));
    }
    match settings.get("policy") {
        None => Ok(()),
        Some(Value::Object(policy)) => validate_policy(policy),
        Some(_) => Err(invalid("policy must be an object")),
    }
}

// ---- storage ---------------------------------------------------------------------------

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

struct State {
    settings: Map<String, Value>,
    bytes: Vec<u8>,
    revision: u64,
}

fn read_document(root: &Path) -> Result<(Map<String, Value>, Vec<u8>), Failure> {
    match std::fs::read(document_path(root)) {
        Ok(bytes) => {
            let value: Value = serde_json::from_slice(&bytes)
                .map_err(|error| invalid(format!("settings.json is not valid JSON: {error}")))?;
            let settings = match value {
                Value::Object(settings) => settings,
                _ => return Err(invalid("settings.json must be an object")),
            };
            validate_settings(&settings)?;
            Ok((settings, bytes))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut settings = Map::new();
            settings.insert("format".to_owned(), json!(1));
            let bytes = crate::canonical_bytes(&Value::Object(settings.clone()))?;
            Ok((settings, bytes))
        }
        Err(error) => Err(Failure::io("read settings.json", &error)),
    }
}

fn read_sidecar(root: &Path) -> Result<Option<(u64, String)>, Failure> {
    match std::fs::read_to_string(revision_path(root)) {
        Ok(text) => {
            let mut parts = text.split_whitespace();
            let revision = parts.next().and_then(|part| part.parse::<u64>().ok());
            let sha = parts.next().map(str::to_owned);
            Ok(revision.zip(sha))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(Failure::io("read settings.revision", &error)),
    }
}

fn write_sidecar(root: &Path, revision: u64, bytes: &[u8]) -> Result<(), Failure> {
    crate::write_atomic(
        &revision_path(root),
        format!("{revision} {}\n", digest(bytes)).as_bytes(),
    )
}

/// Load settings and reconcile the revision sidecar with the document's digest.
fn load(root: &Path) -> Result<State, Failure> {
    let (settings, bytes) = read_document(root)?;
    let sha = digest(&bytes);
    let revision = match read_sidecar(root)? {
        Some((revision, recorded)) if recorded == sha => revision,
        Some((revision, _)) => {
            write_sidecar(root, revision + 1, &bytes)?;
            revision + 1
        }
        None => {
            write_sidecar(root, 0, &bytes)?;
            0
        }
    };
    Ok(State {
        settings,
        bytes,
        revision,
    })
}

fn store(root: &Path, state: &State, settings: Map<String, Value>) -> Result<State, Failure> {
    validate_settings(&settings)?;
    let bytes = crate::canonical_bytes(&Value::Object(settings.clone()))?;
    crate::write_atomic(&document_path(root), &bytes)?;
    let revision = state.revision + 1;
    write_sidecar(root, revision, &bytes)?;
    Ok(State {
        settings,
        bytes,
        revision,
    })
}

fn policy_of(settings: &Map<String, Value>) -> Map<String, Value> {
    settings
        .get("policy")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default()
}

fn with_policy(mut settings: Map<String, Value>, policy: Map<String, Value>) -> Map<String, Value> {
    if policy.is_empty() {
        settings.remove("policy");
    } else {
        settings.insert("policy".to_owned(), Value::Object(policy));
    }
    settings
}

fn namespace(state: &State) -> Value {
    let value = Value::Object(policy_of(&state.settings));
    json!({
        "ns": NAMESPACE,
        "schema": schema(),
        "value": value,
        "base": Value::Null,
        "user": value,
        "applies": "next-launch",
        "secrets": [],
        "revision": state.revision,
    })
}

fn check_namespace(payload: &Value) -> Result<(), Failure> {
    let ns = payload["ns"].as_str().unwrap_or_default();
    if ns != NAMESPACE {
        return Err(Failure::new(
            "unknown-namespace",
            format!("unknown settings namespace {ns}"),
        )
        .with_details(json!({ "ns": ns })));
    }
    Ok(())
}

fn check_revision(payload: &Value, state: &State) -> Result<(), Failure> {
    if let Some(expected) = payload.get("expectedRevision").and_then(Value::as_u64) {
        if expected != state.revision {
            return Err(Failure::new("stale-revision", "settings revision is stale")
                .with_details(json!({ "current": state.revision })));
        }
    }
    Ok(())
}

fn field_of(path: &Value) -> Result<String, Failure> {
    let path = path.as_array().map(Vec::as_slice).unwrap_or_default();
    match path {
        [Value::String(key)] if FIELDS.contains(&key.as_str()) => Ok(key.clone()),
        [Value::String(key)] => Err(invalid(format!("unknown policy key {key}"))),
        _ => Err(Failure::bad_request(
            "path must name exactly one policy field",
        )),
    }
}

// ---- methods ---------------------------------------------------------------------------

pub fn describe(root: &Path) -> Result<Value, Failure> {
    let state = load(root)?;
    Ok(json!({ "writable": true, "hasDocument": true, "namespaces": [namespace(&state)] }))
}

pub fn mutate(root: &Path, payload: &Value) -> Result<Value, Failure> {
    check_namespace(payload)?;
    let state = load(root)?;
    check_revision(payload, &state)?;
    let mut policy = policy_of(&state.settings);
    for operation in payload["operations"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
    {
        let key = field_of(&operation["path"])?;
        if operation["op"] == "set" {
            let value = operation["value"].clone();
            validate_field(&key, &value)?;
            policy.insert(key, value);
        } else {
            policy.remove(&key);
        }
    }
    let state = store(root, &state, with_policy(state.settings.clone(), policy))?;
    Ok(namespace(&state))
}

pub fn update(root: &Path, payload: &Value) -> Result<Value, Failure> {
    check_namespace(payload)?;
    let state = load(root)?;
    check_revision(payload, &state)?;
    let mut policy = policy_of(&state.settings);
    for (key, value) in payload["patch"].as_object().cloned().unwrap_or_default() {
        if !FIELDS.contains(&key.as_str()) {
            return Err(invalid(format!("unknown policy key {key}")));
        }
        if value.is_null() {
            policy.remove(&key);
        } else {
            validate_field(&key, &value)?;
            policy.insert(key, value);
        }
    }
    let state = store(root, &state, with_policy(state.settings.clone(), policy))?;
    Ok(namespace(&state))
}

pub fn document(root: &Path) -> Result<Value, Failure> {
    let state = load(root)?;
    let path = document_path(root);
    if !path.exists() {
        crate::write_atomic(&path, &state.bytes)?;
        write_sidecar(root, state.revision, &state.bytes)?;
    }
    Ok(json!({ "path": path.to_string_lossy() }))
}
