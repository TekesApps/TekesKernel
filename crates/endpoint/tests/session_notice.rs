use endpoint::{NativeEndpoint, SESSION_NOTICE_OPERATION, SessionNotice, SessionNoticeSeverity};
use schema::{OriginTuple, ResumePolicy};

const SESSION: &str = "018f0000-0000-7000-8000-000000000121";

fn origin(operation: &str, key: &str) -> OriginTuple {
    OriginTuple {
        principal: "host".to_owned(),
        client: "tekes-supervisor".to_owned(),
        target: SESSION.to_owned(),
        op: operation.to_owned(),
        key: key.to_owned(),
    }
}

fn notice(message: &str) -> SessionNotice {
    SessionNotice {
        severity: SessionNoticeSeverity::Warning,
        classification: "mcp_registry".to_owned(),
        operation: "worker-launch".to_owned(),
        message: message.to_owned(),
    }
}

/// The same degraded launch on every worker restart is one fact, not one row
/// per restart: a notice identical to the newest one folds unless an input
/// arrived in between — a fresh attempt by the user whose outcome is shown.
#[test]
fn identical_notice_folds_until_the_next_input() {
    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    endpoint
        .create_session(
            SESSION,
            "workspace-1",
            "config-digest",
            ResumePolicy::Never,
            "2026-09-21T00:00:00.000Z",
            &origin("create", "create-1"),
        )
        .expect("create session");
    let record = |key: &str, message: &str| {
        endpoint.record_session_notice(
            SESSION,
            "2026-09-21T00:00:01.000Z",
            &origin(SESSION_NOTICE_OPERATION, key),
            &notice(message),
        )
    };
    let first = record("launch-1", "MCP servers were skipped")
        .expect("first notice")
        .expect("first notice is written");
    assert!(!first.deduplicated);
    assert!(
        record("launch-2", "MCP servers were skipped")
            .expect("repeat")
            .is_none(),
        "a repeat of the newest notice folds"
    );
    assert!(
        record("launch-3", "MCP server x is unavailable")
            .expect("different")
            .is_some(),
        "a different message is a new fact"
    );
    let retry = record("launch-3", "MCP server x is unavailable")
        .expect("exact retry")
        .expect("keyed retry returns the original receipt");
    assert!(retry.deduplicated);

    {
        // A user input, appended the way the endpoint's prompt route does once
        // the profile exists (no workspace profile in this fixture).
        let ledger = root.path().join("threads").join(SESSION).join("main.jsonl");
        let mut locked = store::LockedLedger::open(ledger, 1).expect("ledger");
        let seq = locked.next_seq();
        let value = serde_json::json!({
            "v": 1, "seq": seq, "kind": "input", "ts": "2026-09-21T00:00:02.000Z",
            "content": [{"type": "text", "text": "again"}],
            "origin_key": "prompt-1",
            "origin_tuple": {"principal": "user", "client": "test", "target": SESSION,
                "op": "session.prompt", "key": "prompt-1"}
        });
        let event =
            schema::Event::decode(&serde_json::to_vec(&value).expect("json")).expect("event");
        locked
            .append_contract(event, store::BarrierContext::default())
            .expect("append input");
    }
    assert!(
        record("launch-4", "MCP server x is unavailable")
            .expect("after input")
            .is_some(),
        "an input since the newest notice makes the repeat worth showing again"
    );
}
