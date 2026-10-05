use std::cell::Cell;
use std::fs;

use schema::IJsonValue;
use serde_json::json;
use tekes_supervisor::tool_control::{ToolControlReceiptStore, ToolControlResolution};
use worker_control::{
    ToolControl, ToolControlError, ToolControlErrorCode, ToolControlResult,
    decode_tool_control_result,
};

const THREAD: &str = "018f0000-0000-7000-8000-000000000001";

#[test]
fn slice8_supervisor_receipt_publishes_then_replays_exact_bytes() {
    let root = tempfile::tempdir().unwrap();
    let thread_folder = root.path().join("threads").join(THREAD);
    fs::create_dir_all(&thread_folder).unwrap();
    let request = request();
    let store = ToolControlReceiptStore::new(&thread_folder, THREAD);
    let backend_calls = Cell::new(0);
    let policy_calls = Cell::new(0);

    let published = store
        .publish_or_replay(
            &request,
            |_| {
                backend_calls.set(backend_calls.get() + 1);
                ToolControlResult::success(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    ijson(json!({"ok": true})),
                )
            },
            |result| {
                policy_calls.set(policy_calls.get() + 1);
                result
            },
        )
        .unwrap();
    let ToolControlResolution::Published {
        response,
        receipt_path,
    } = published
    else {
        panic!("first execution must publish");
    };
    assert_eq!(backend_calls.get(), 1);
    assert_eq!(policy_calls.get(), 1);
    assert!(receipt_path.is_file());
    assert_eq!(receipt_path, store.record_path(&request.request_id));
    assert_eq!(
        fs::read(&receipt_path).unwrap(),
        include_bytes!("../../../fixtures/wire/durable/control-receipt.canonical.json")
            .strip_suffix(b"\n")
            .expect("fixture LF frames canonical JSON")
    );
    assert_eq!(
        response,
        concat!(
            "{\"tool_control_result\":{\"call_id\":\"call-1\",",
            "\"request_id\":\"c4c8d1996c06badd6ca85825db618b9b95d5ae3430232ed136a692c887497100\",",
            "\"value\":{\"ok\":true}}}\n"
        )
        .as_bytes()
    );

    let restarted = ToolControlReceiptStore::new(&thread_folder, THREAD);
    let replayed = restarted
        .publish_or_replay(
            &request,
            |_| panic!("replay must not invoke backend"),
            |_| panic!("replay must not re-run policy"),
        )
        .unwrap();
    assert!(replayed.is_replay());
    assert_eq!(replayed.response(), response);
}

#[test]
fn slice8_supervisor_receipt_conflict_never_executes() {
    let root = tempfile::tempdir().unwrap();
    let thread_folder = root.path().join(THREAD);
    fs::create_dir_all(&thread_folder).unwrap();
    let request = request();
    let store = ToolControlReceiptStore::new(&thread_folder, THREAD);
    store
        .publish_or_replay(
            &request,
            |_| {
                ToolControlResult::success(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    IJsonValue::from(true),
                )
            },
            |result| result,
        )
        .unwrap();

    let mut conflict = request.clone();
    conflict.name = "report".to_owned();
    let resolution = store
        .publish_or_replay(
            &conflict,
            |_| panic!("conflicting tuple must not execute"),
            |_| panic!("conflicting tuple must not enter policy"),
        )
        .unwrap();
    let ToolControlResolution::Conflict { response } = resolution else {
        panic!("changed tuple must conflict");
    };
    let result = decode_tool_control_result(&response).unwrap();
    assert_eq!(result.error.unwrap().code, ToolControlErrorCode::Conflict);
}

#[test]
fn slice8_supervisor_receipt_persists_only_policy_output() {
    let root = tempfile::tempdir().unwrap();
    let thread_folder = root.path().join(THREAD);
    fs::create_dir_all(&thread_folder).unwrap();
    let request = request();
    let store = ToolControlReceiptStore::new(&thread_folder, THREAD);
    let resolution = store
        .publish_or_replay(
            &request,
            |_| {
                ToolControlResult::success(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    IJsonValue::from("secret-value"),
                )
            },
            |_| {
                ToolControlResult::failure(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    ToolControlError {
                        code: ToolControlErrorCode::Denied,
                        message: "withheld by policy".to_owned(),
                        retryable: false,
                    },
                )
            },
        )
        .unwrap();
    assert!(!String::from_utf8_lossy(resolution.response()).contains("secret-value"));
    let receipt = fs::read(store.record_path(&request.request_id)).unwrap();
    assert!(!String::from_utf8_lossy(&receipt).contains("secret-value"));
}

fn request() -> ToolControl {
    ToolControl::new(
        THREAD,
        THREAD,
        1,
        "call-1",
        "context",
        ijson(json!({
            "operation": "threads",
            "reason": "fixture audit",
            "thread_id": THREAD,
        })),
    )
    .unwrap()
}

fn ijson(value: serde_json::Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap()
}

#[test]
fn a_pre_merge_receipt_layout_is_folded_into_the_control_directory_on_first_use() {
    let root = tempfile::tempdir().unwrap();
    let thread_folder = root.path().join("threads").join(THREAD);
    let request = request();
    let prefix = &request.request_id[..2];
    let legacy_receipt = thread_folder
        .join("control-receipts")
        .join(prefix)
        .join(format!("{}.json", request.request_id));
    fs::create_dir_all(legacy_receipt.parent().unwrap()).unwrap();
    fs::write(
        &legacy_receipt,
        include_bytes!("../../../fixtures/wire/durable/control-receipt.canonical.json")
            .strip_suffix(b"\n")
            .unwrap(),
    )
    .unwrap();
    // A stale intent for the same id loses to the receipt.
    let legacy_intent = thread_folder
        .join("control-intents")
        .join(prefix)
        .join(format!("{}.json", request.request_id));
    fs::create_dir_all(legacy_intent.parent().unwrap()).unwrap();
    fs::write(
        &legacy_intent,
        serde_json_canonicalizer::to_vec(&json!({"format":1,"request":request})).unwrap(),
    )
    .unwrap();

    let store = ToolControlReceiptStore::new(&thread_folder, THREAD);
    let resolution = store
        .publish_or_replay(
            &request,
            |_| panic!("a settled record is replayed, never re-executed"),
            |result| result,
        )
        .unwrap();
    assert!(resolution.is_replay());
    assert!(!thread_folder.join("control-receipts").exists());
    assert!(!thread_folder.join("control-intents").exists());
    let record = store.read_record(&request.request_id).unwrap().unwrap();
    assert_eq!(record.request, request);
    assert!(record.response.is_some());
    assert_eq!(
        fs::read(store.record_path(&request.request_id)).unwrap(),
        fs::read(
            thread_folder
                .join("control")
                .join(prefix)
                .join(format!("{}.json", request.request_id))
        )
        .unwrap()
    );
}
