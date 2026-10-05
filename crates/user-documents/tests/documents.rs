use std::path::Path;

use serde_json::{Value, json};
use user_documents::{Failure, METHODS, execute, validate};

const SESSION: &str = "0b7e3d2a-4c5e-4f6a-8b9c-0d1e2f3a4b5c";

fn run(root: &Path, method: &str, payload: Value) -> Result<Value, Failure> {
    execute(root, method, &payload)
}

fn put(
    root: &Path,
    message: &str,
    rating: &str,
    if_version: Value,
    note: Option<Value>,
) -> Result<Value, Failure> {
    let mut payload = json!({ "sessionId": SESSION, "messageId": message, "rating": rating, "ifVersion": if_version });
    if let Some(note) = note {
        payload["note"] = note;
    }
    run(root, "feedback.put", payload)
}

#[test]
fn method_list_matches_validate() {
    assert_eq!(METHODS.len(), 7);
    for method in METHODS {
        assert!(
            validate(method, &json!({ "unknown": 1 })).is_err(),
            "{method} accepted an unknown field"
        );
    }
    assert!(validate("nope", &json!({})).is_err());
}

#[test]
fn feedback_create_requires_null_version() {
    let dir = tempfile::tempdir().unwrap();
    let error = put(dir.path(), "m1", "positive", json!("1"), None).unwrap_err();
    assert_eq!(error.code, "version-conflict");
    assert_eq!(error.details, json!({ "current": null }));

    let created = put(dir.path(), "m1", "positive", Value::Null, None).unwrap();
    assert_eq!(created["messageId"], "m1");
    assert_eq!(created["rating"], "positive");
    assert_eq!(created["version"], "1");
    assert!(created.get("note").is_none());
    assert!(created["updatedAtMilliseconds"].as_u64().unwrap() > 0);

    let again = put(dir.path(), "m1", "negative", Value::Null, None).unwrap_err();
    assert_eq!(again.code, "version-conflict");
    assert_eq!(again.details["current"], created);
}

#[test]
fn feedback_replace_conflict_delete_and_monotonic_counter() {
    let dir = tempfile::tempdir().unwrap();
    let first = put(dir.path(), "m1", "positive", Value::Null, None).unwrap();
    let second = put(dir.path(), "m1", "negative", json!("1"), None).unwrap();
    assert_eq!(second["version"], "2");
    assert_eq!(second["rating"], "negative");

    let stale = put(dir.path(), "m1", "positive", json!("1"), None).unwrap_err();
    assert_eq!(stale.code, "version-conflict");
    assert_eq!(stale.details["current"], second);
    let _ = first;

    let listed = run(dir.path(), "feedback.list", json!({ "sessionId": SESSION })).unwrap();
    assert_eq!(listed["items"].as_array().unwrap().len(), 1);

    let wrong = run(
        dir.path(),
        "feedback.delete",
        json!({ "sessionId": SESSION, "messageId": "m1", "ifVersion": "1" }),
    )
    .unwrap_err();
    assert_eq!(wrong.code, "version-conflict");
    let deleted = run(
        dir.path(),
        "feedback.delete",
        json!({ "sessionId": SESSION, "messageId": "m1", "ifVersion": "2" }),
    )
    .unwrap();
    assert_eq!(deleted, json!({ "deleted": true }));
    let missing = run(
        dir.path(),
        "feedback.delete",
        json!({ "sessionId": SESSION, "messageId": "m1", "ifVersion": "2" }),
    )
    .unwrap_err();
    assert_eq!(missing.details, json!({ "current": null }));
    assert_eq!(
        run(dir.path(), "feedback.list", json!({ "sessionId": SESSION })).unwrap(),
        json!({ "items": [] })
    );

    let recreated = put(dir.path(), "m1", "positive", Value::Null, None).unwrap();
    assert_eq!(
        recreated["version"], "3",
        "versions are never reused after delete"
    );
}

#[test]
fn feedback_note_preserve_replace_clear() {
    let dir = tempfile::tempdir().unwrap();
    let created = put(
        dir.path(),
        "m1",
        "positive",
        Value::Null,
        Some(json!("first note")),
    )
    .unwrap();
    assert_eq!(created["note"], "first note");
    let preserved = put(dir.path(), "m1", "negative", json!("1"), None).unwrap();
    assert_eq!(preserved["note"], "first note");
    let replaced = put(
        dir.path(),
        "m1",
        "negative",
        json!("2"),
        Some(json!("second")),
    )
    .unwrap();
    assert_eq!(replaced["note"], "second");
    let cleared = put(dir.path(), "m1", "negative", json!("3"), Some(Value::Null)).unwrap();
    assert!(cleared.get("note").is_none());
    let listed = run(dir.path(), "feedback.list", json!({ "sessionId": SESSION })).unwrap();
    assert!(listed["items"][0].get("note").is_none());
}

#[test]
fn feedback_canonical_bytes_and_validation() {
    let dir = tempfile::tempdir().unwrap();
    put(dir.path(), "zeta", "positive", Value::Null, None).unwrap();
    put(
        dir.path(),
        "alpha",
        "negative",
        Value::Null,
        Some(json!("n")),
    )
    .unwrap();
    let path = dir.path().join("feedback").join(format!("{SESSION}.json"));
    let bytes = std::fs::read(&path).unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    assert!(text.ends_with('\n') && !text[..text.len() - 1].contains('\n'));
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    let mut canonical = serde_json_canonicalizer::to_vec(&value).unwrap();
    canonical.push(b'\n');
    assert_eq!(bytes, canonical);
    assert_eq!(value["format"], 1);
    assert_eq!(value["counter"], 2);
    assert_eq!(value["items"][0]["messageId"], "alpha");
    assert!(
        !std::fs::read_dir(dir.path().join("feedback"))
            .unwrap()
            .any(|entry| entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .ends_with(".tmp"))
    );

    assert!(validate("feedback.list", &json!({ "sessionId": "not-a-uuid" })).is_err());
    assert!(
        validate(
            "feedback.list",
            &json!({ "sessionId": SESSION.to_uppercase() })
        )
        .is_err()
    );
    assert!(validate("feedback.put", &json!({ "sessionId": SESSION, "messageId": "", "rating": "positive", "ifVersion": null })).is_err());
    assert!(validate("feedback.put", &json!({ "sessionId": SESSION, "messageId": "x".repeat(201), "rating": "positive", "ifVersion": null })).is_err());
    assert!(
        validate(
            "feedback.put",
            &json!({ "sessionId": SESSION, "messageId": "m", "rating": "meh", "ifVersion": null })
        )
        .is_err()
    );
    assert!(
        validate(
            "feedback.put",
            &json!({ "sessionId": SESSION, "messageId": "m", "rating": "positive" })
        )
        .is_err(),
        "ifVersion is required"
    );
    assert!(validate("feedback.put", &json!({ "sessionId": SESSION, "messageId": "m", "rating": "positive", "ifVersion": null, "note": "x".repeat(4097) })).is_err());
    assert!(
        validate(
            "feedback.delete",
            &json!({ "sessionId": SESSION, "messageId": "m", "ifVersion": null })
        )
        .is_err()
    );
    let error = run(dir.path(), "feedback.put", json!({ "sessionId": SESSION })).unwrap_err();
    assert_eq!(error.code, "bad-request");
}

fn policy(root: &Path) -> (Value, u64) {
    let described = run(root, "settings.describe", json!({})).unwrap();
    let ns = &described["namespaces"][0];
    (ns["value"].clone(), ns["revision"].as_u64().unwrap())
}

#[test]
fn settings_describe_empty_root() {
    let dir = tempfile::tempdir().unwrap();
    let described = run(dir.path(), "settings.describe", json!({})).unwrap();
    assert_eq!(described["writable"], true);
    assert_eq!(described["hasDocument"], true);
    let ns = &described["namespaces"][0];
    assert_eq!(described["namespaces"].as_array().unwrap().len(), 1);
    assert_eq!(ns["ns"], "policy");
    assert_eq!(ns["value"], json!({}));
    assert_eq!(ns["user"], json!({}));
    assert_eq!(ns["base"], Value::Null);
    assert_eq!(ns["applies"], "next-launch");
    assert_eq!(ns["secrets"], json!([]));
    assert_eq!(ns["revision"], 0);
    assert_eq!(ns["schema"]["properties"]["max_wall_seconds"]["minimum"], 1);
    assert!(
        !dir.path().join("settings.json").exists(),
        "describe does not create the document"
    );
}

#[test]
fn settings_mutate_update_unset() {
    let dir = tempfile::tempdir().unwrap();
    let ns = run(
        dir.path(),
        "settings.mutate",
        json!({
            "ns": "policy",
            "operations": [
                { "op": "set", "path": ["network"], "value": false },
                { "op": "set", "path": ["writable_roots"], "value": ["/tmp/a"] },
                { "op": "set", "path": ["max_wall_seconds"], "value": 30 }
            ],
            "expectedRevision": 0
        }),
    )
    .unwrap();
    assert_eq!(ns["revision"], 1);
    assert_eq!(
        ns["value"],
        json!({ "network": false, "writable_roots": ["/tmp/a"], "max_wall_seconds": 30 })
    );

    let bytes = std::fs::read(dir.path().join("settings.json")).unwrap();
    assert_eq!(bytes, b"{\"format\":1,\"policy\":{\"max_wall_seconds\":30,\"network\":false,\"writable_roots\":[\"/tmp/a\"]}}\n");
    assert!(
        std::fs::read_to_string(dir.path().join("settings.revision"))
            .unwrap()
            .starts_with("1 ")
    );

    let ns = run(dir.path(), "settings.update", json!({
        "ns": "policy", "patch": { "allowed_tools": ["read"], "network": null }, "expectedRevision": 1
    })).unwrap();
    assert_eq!(ns["revision"], 2);
    assert_eq!(
        ns["value"],
        json!({ "writable_roots": ["/tmp/a"], "max_wall_seconds": 30, "allowed_tools": ["read"] })
    );

    let ns = run(
        dir.path(),
        "settings.mutate",
        json!({
            "ns": "policy",
            "operations": [
                { "op": "unset", "path": ["writable_roots"] },
                { "op": "unset", "path": ["max_wall_seconds"] },
                { "op": "unset", "path": ["allowed_tools"] }
            ]
        }),
    )
    .unwrap();
    assert_eq!(ns["value"], json!({}));
    assert_eq!(
        std::fs::read(dir.path().join("settings.json")).unwrap(),
        b"{\"format\":1}\n"
    );
    assert_eq!(policy(dir.path()).1, 3);
}

#[test]
fn settings_rejections() {
    let dir = tempfile::tempdir().unwrap();
    let cases = [
        (
            json!([{ "op": "set", "path": ["network"], "value": "yes" }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["max_wall_seconds"], "value": 0 }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["max_wall_seconds"], "value": 1.5 }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["allowed_tools"], "value": [1] }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["writable_roots"], "value": ["relative/path"] }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["mystery"], "value": 1 }]),
            "settings-invalid",
        ),
        (
            json!([{ "op": "set", "path": ["network", "deep"], "value": true }]),
            "bad-request",
        ),
        (
            json!([{ "op": "set", "path": [], "value": true }]),
            "bad-request",
        ),
        (
            json!([{ "op": "flip", "path": ["network"] }]),
            "bad-request",
        ),
        (
            json!([{ "op": "unset", "path": ["network"], "value": 1 }]),
            "bad-request",
        ),
    ];
    for (operations, code) in cases {
        let error = run(
            dir.path(),
            "settings.mutate",
            json!({ "ns": "policy", "operations": operations }),
        )
        .unwrap_err();
        assert_eq!(error.code, code, "{operations}");
    }
    let error = run(
        dir.path(),
        "settings.update",
        json!({ "ns": "policy", "patch": { "mystery": 1 } }),
    )
    .unwrap_err();
    assert_eq!(error.code, "settings-invalid");
    let error = run(
        dir.path(),
        "settings.update",
        json!({ "ns": "policy", "patch": { "writable_roots": ["x"] } }),
    )
    .unwrap_err();
    assert_eq!(error.code, "settings-invalid");
    let error = run(
        dir.path(),
        "settings.update",
        json!({ "ns": "other", "patch": {} }),
    )
    .unwrap_err();
    assert_eq!(error.code, "unknown-namespace");
    let error = run(
        dir.path(),
        "settings.mutate",
        json!({ "ns": "other", "operations": [] }),
    )
    .unwrap_err();
    assert_eq!(error.code, "unknown-namespace");
    assert!(
        !dir.path().join("settings.json").exists(),
        "rejected writes leave no document"
    );
    assert_eq!(
        policy(dir.path()).1,
        0,
        "rejected writes do not bump the revision"
    );
}

#[test]
fn settings_stale_revision_and_out_of_band_edit() {
    let dir = tempfile::tempdir().unwrap();
    let ns = run(
        dir.path(),
        "settings.update",
        json!({ "ns": "policy", "patch": { "network": true }, "expectedRevision": 0 }),
    )
    .unwrap();
    assert_eq!(ns["revision"], 1);

    let error = run(
        dir.path(),
        "settings.update",
        json!({ "ns": "policy", "patch": { "network": false }, "expectedRevision": 0 }),
    )
    .unwrap_err();
    assert_eq!(error.code, "stale-revision");
    assert_eq!(error.details, json!({ "current": 1 }));

    std::fs::write(
        dir.path().join("settings.json"),
        "{\"format\":1,\"policy\":{\"network\":false}}\n",
    )
    .unwrap();
    let (value, revision) = policy(dir.path());
    assert_eq!(value, json!({ "network": false }));
    assert_eq!(revision, 2, "out-of-band edit bumps the revision");
    assert_eq!(policy(dir.path()).1, 2, "unchanged file keeps the revision");
    let error = run(
        dir.path(),
        "settings.mutate",
        json!({ "ns": "policy", "operations": [], "expectedRevision": 1 }),
    )
    .unwrap_err();
    assert_eq!(error.code, "stale-revision");
    assert_eq!(error.details["current"], 2);
    let ns = run(
        dir.path(),
        "settings.mutate",
        json!({ "ns": "policy", "operations": [], "expectedRevision": 2 }),
    )
    .unwrap();
    assert_eq!(ns["revision"], 3);

    std::fs::write(
        dir.path().join("settings.json"),
        "{\"format\":1,\"policy\":{\"network\":\"nope\"}}\n",
    )
    .unwrap();
    let error = run(dir.path(), "settings.describe", json!({})).unwrap_err();
    assert_eq!(error.code, "settings-invalid");
    std::fs::write(
        dir.path().join("settings.json"),
        "{\"format\":1,\"extra\":true}\n",
    )
    .unwrap();
    assert_eq!(
        run(dir.path(), "settings.describe", json!({}))
            .unwrap_err()
            .code,
        "settings-invalid"
    );
}

#[test]
fn settings_document_creates_file() {
    let dir = tempfile::tempdir().unwrap();
    let opened = run(dir.path(), "settings.document", json!({})).unwrap();
    let path = dir.path().join("settings.json");
    assert_eq!(opened, json!({ "path": path.to_string_lossy() }));
    assert_eq!(std::fs::read(&path).unwrap(), b"{\"format\":1}\n");
    assert_eq!(
        policy(dir.path()).1,
        0,
        "creating the document is not an edit"
    );
    let again = run(dir.path(), "settings.document", json!({})).unwrap();
    assert_eq!(again, opened);
    assert!(run(dir.path(), "settings.document", json!({ "x": 1 })).is_err());
}
