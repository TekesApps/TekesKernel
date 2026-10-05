use std::cell::Cell;

use engine::{SupervisorControlBackend, ToolBackend};
use schema::IJsonValue;
use tools::{BackendTerminal, ToolExecution};
use worker_control::{ToolControl, ToolControlResult};

#[test]
fn supervisor_backend_derives_and_validates_correlated_request() {
    let calls = Cell::new(0_u32);
    let mut backend = SupervisorControlBackend::new(
        "018f0000-0000-7000-8000-000000000001",
        |request: &ToolControl| {
            calls.set(calls.get() + 1);
            assert_eq!(request.name, "context_get");
            assert_eq!(request.thread, "018f0000-0000-7000-8000-000000000003");
            Ok(ToolControlResult::success(
                request.request_id.clone(),
                request.call_id.clone(),
                IJsonValue::from("history"),
            ))
        },
    );
    let terminal = backend.execute(&execution("context_get"), &IJsonValue::from("args"));
    assert_eq!(
        terminal,
        BackendTerminal::Completed(IJsonValue::from("history"))
    );
    assert_eq!(calls.get(), 1);
    assert!(backend.supports("task"));
    assert!(backend.supports("job"));
    assert!(!backend.supports("read"));
}

#[test]
fn supervisor_backend_never_converts_a_typed_failure_to_success() {
    let mut backend = SupervisorControlBackend::new(
        "018f0000-0000-7000-8000-000000000001",
        |request: &ToolControl| {
            Ok(ToolControlResult::failure(
                request.request_id.clone(),
                request.call_id.clone(),
                worker_control::ToolControlError {
                    code: worker_control::ToolControlErrorCode::Unavailable,
                    message: "authority offline".to_owned(),
                    retryable: true,
                },
            ))
        },
    );
    assert_eq!(
        backend.execute(&execution("task"), &IJsonValue::from("args")),
        BackendTerminal::Unavailable {
            code: "unavailable".to_owned(),
            message: "authority offline".to_owned(),
            retryable: true,
        }
    );
}

fn execution(name: &str) -> ToolExecution {
    ToolExecution {
        thread: "018f0000-0000-7000-8000-000000000003".to_owned(),
        turn: 1,
        attempt: "a1".to_owned(),
        call: "c1".to_owned(),
        name: name.to_owned(),
        invocation: IJsonValue::from("args"),
        side_effectful: true,
        timestamp: "2026-08-27T09:00:00.000Z".to_owned(),
    }
}
