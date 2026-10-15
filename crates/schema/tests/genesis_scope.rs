use schema::Event;
use serde_json::{Value, json};

fn genesis() -> Value {
    json!({"v":1,"seq":1,"kind":"genesis","ts":"2026-10-15T00:00:00.000Z",
        "thread":"018f0000-0000-7000-8000-000000000001","workspace":"ws",
        "format":1,"min_reader":1,"min_writer":1,"resume":"never",
        "config":{"digest":"cfg"},"origin_key":"create",
        "origin_tuple":{"principal":"p","client":"c","target":"018f0000-0000-7000-8000-000000000001","op":"create","key":"create"},
        "folder_binding":"folder-0002","environment":"local","cwd":"/fixture/second",
        "folders":["folder-0001","folder-0002"]})
}

fn decodes(raw: &Value) -> bool {
    Event::decode(&serde_json::to_vec(raw).unwrap()).is_ok()
}

/// A genesis records its environment and folder ids; the bound folder is one
/// of them, and a genesis without either still decodes (it reads as `local`
/// with every workspace folder).
#[test]
fn genesis_environment_and_folders_are_validated_together() {
    let raw = genesis();
    assert!(decodes(&raw));

    let mut legacy = raw.clone();
    for field in ["environment", "folders"] {
        legacy.as_object_mut().unwrap().remove(field);
    }
    assert!(decodes(&legacy));

    for (field, value) in [
        ("cwd", json!("relative/path")),
        ("cwd", json!(null)),
        ("cwd", json!(7)),
        ("cwd", json!("/bad\0path")),
        ("environment", json!("")),
        ("environment", json!(7)),
        ("folders", json!([])),
        ("folders", json!(["folder-0002", "folder-0002"])),
        ("folders", json!(["folder-0001", ""])),
        ("folders", json!([{"id": "folder-0002"}])),
        ("folders", json!("folder-0002")),
        // The binding must name a recorded folder.
        ("folders", json!(["folder-0001"])),
    ] {
        let mut invalid = raw.clone();
        invalid[field] = value.clone();
        assert!(!decodes(&invalid), "{field} = {value}");
    }
}

/// Every field a genesis may carry is either inherited by derived sessions or
/// set by the operation that creates one; the two lists never overlap.
#[test]
fn genesis_field_dispositions_are_disjoint() {
    for field in schema::GENESIS_INHERITED_FIELDS {
        assert!(
            !schema::GENESIS_OPERATION_FIELDS.contains(field),
            "{field} is both inherited and operation-set"
        );
    }
    let raw = genesis();
    for field in raw.as_object().unwrap().keys() {
        assert!(
            schema::GENESIS_INHERITED_FIELDS.contains(&field.as_str())
                || schema::GENESIS_OPERATION_FIELDS.contains(&field.as_str()),
            "{field} has no disposition"
        );
    }
}
