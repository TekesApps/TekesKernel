use std::fs;
use std::path::PathBuf;

use schema::IJsonValue;
use serde_json::json;
use worker_control::{
    DurableControlError, Hello, ToolControl, ToolControlBinding, ToolControlError,
    ToolControlErrorCode, ToolControlResult, decode_tool_control, decode_tool_control_result,
    derive_request_id, encode_tool_control, encode_tool_control_result,
};
use worker_control::{Selected, require_version};

const THREAD: &str = "018f0000-0000-7000-8000-000000000001";
const REQUEST_ID: &str = "c4c8d1996c06badd6ca85825db618b9b95d5ae3430232ed136a692c887497100";

fn arguments() -> IJsonValue {
    ijson(json!({
        "operation": "threads",
        "reason": "fixture audit",
        "thread_id": THREAD,
    }))
}

#[test]
fn durable_control_request_id_and_fixture_bytes() {
    assert_eq!(
        derive_request_id(THREAD, THREAD, 1, "call-1").unwrap(),
        REQUEST_ID
    );
    let fixture = fs::read(fixture("tool-control.jsonl")).unwrap();
    let mut lines = fixture.split_inclusive(|byte| *byte == b'\n');
    let request_line = lines.next().unwrap();
    let result_line = lines.next().unwrap();

    let request = decode_tool_control(request_line).unwrap();
    assert_eq!(request.request_id, REQUEST_ID);
    assert_eq!(encode_tool_control(&request).unwrap(), request_line);

    let result = decode_tool_control_result(result_line).unwrap();
    result.validate_for(&request).unwrap();
    assert_eq!(encode_tool_control_result(&result).unwrap(), result_line);
    assert!(lines.next().is_none());
}

#[test]
fn sibling_line_identity_cannot_alias_a_control_receipt() {
    let child_a = "018f0000-0000-7000-8000-000000000002";
    let child_b = "018f0000-0000-7000-8000-000000000003";
    let left = derive_request_id(THREAD, child_a, 1, "call-1").unwrap();
    let right = derive_request_id(THREAD, child_b, 1, "call-1").unwrap();
    assert_ne!(left, right);
}

#[test]
fn durable_control_validates_process_and_durable_binding() {
    let request = ToolControl::new(THREAD, THREAD, 1, "call-1", "context", arguments()).unwrap();
    request
        .validate_against(&ToolControlBinding {
            session: THREAD,
            thread: THREAD,
            turn: 1,
            call_id: "call-1",
            name: "context",
            arguments: &arguments(),
        })
        .unwrap();

    let other_arguments = ijson(json!({"operation": "other"}));
    let error = request
        .validate_against(&ToolControlBinding {
            session: THREAD,
            thread: THREAD,
            turn: 1,
            call_id: "call-1",
            name: "context",
            arguments: &other_arguments,
        })
        .unwrap_err();
    assert!(matches!(
        error,
        DurableControlError::BindingMismatch("arguments")
    ));
}

#[test]
fn durable_control_rejects_malformed_requests_and_results() {
    let uppercase = format!("{}A", &REQUEST_ID[..63]);
    let bad_id = format!(
        "{{\"tool_control\":{{\"arguments\":{{}},\"call_id\":\"call-1\",\"name\":\"context\",\"request_id\":\"{uppercase}\",\"session\":\"{THREAD}\",\"thread\":\"{THREAD}\",\"turn\":1}}}}\n"
    );
    assert!(matches!(
        decode_tool_control(bad_id.as_bytes()),
        Err(DurableControlError::RequestIdSyntax)
    ));

    let both = format!(
        "{{\"tool_control_result\":{{\"call_id\":\"call-1\",\"error\":{{\"code\":\"internal\",\"message\":\"x\",\"retryable\":false}},\"request_id\":\"{REQUEST_ID}\",\"value\":{{\"ok\":true}}}}}}\n"
    );
    assert!(matches!(
        decode_tool_control_result(both.as_bytes()),
        Err(DurableControlError::ResultUnion)
    ));

    let unknown = format!(
        "{{\"tool_control\":{{\"arguments\":{{}},\"call_id\":\"call-1\",\"extra\":true,\"name\":\"context\",\"request_id\":\"{REQUEST_ID}\",\"session\":\"{THREAD}\",\"thread\":\"{THREAD}\",\"turn\":1}}}}\n"
    );
    assert!(matches!(
        decode_tool_control(unknown.as_bytes()),
        Err(DurableControlError::Json(_))
    ));
}

#[test]
fn durable_control_result_union_and_negotiation_are_closed() {
    let success = ToolControlResult::success(REQUEST_ID, "call-1", IJsonValue::from(true));
    success.validate().unwrap();
    let failure = ToolControlResult::failure(
        REQUEST_ID,
        "call-1",
        ToolControlError {
            code: ToolControlErrorCode::Unavailable,
            message: "authority absent".to_owned(),
            retryable: true,
        },
    );
    failure.validate().unwrap();

    assert_eq!(Hello::default().min, worker_control::PROTOCOL_VERSION);
    assert_eq!(Hello::default().max, worker_control::PROTOCOL_VERSION);
    assert_eq!(worker_control::PROTOCOL_VERSION, 2);
    require_version(&Selected { version: 2 }).unwrap();
    assert!(matches!(
        require_version(&Selected { version: 1 }),
        Err(DurableControlError::ProtocolVersion(1))
    ));
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/wire/durable")
        .join(name)
}

fn ijson(value: serde_json::Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap()
}
