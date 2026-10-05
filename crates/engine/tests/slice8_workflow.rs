use engine::{CatalogEntry, ToolBackend, WorkflowBackend};
use schema::IJsonValue;
use serde_json::{Value, json};
use tools::{BackendTerminal, DurableApprovalResponse, ToolExecution};

const THREAD: &str = "018f0000-0000-7000-8000-000000000003";

#[test]
fn workflow_holds_are_typed_and_do_not_fabricate_success() {
    let root = tempfile::tempdir().unwrap();
    let mut backend = WorkflowBackend::new(root.path(), "ws", None, vec![], vec![]).unwrap();
    assert!(matches!(
        backend.execute(
            &execution("ask_user_questions", "ask1"),
            &ijson(json!({"question":"Which?","options":null}))
        ),
        BackendTerminal::Hold { ref scope, .. } if scope == "answer"
    ));
    assert!(matches!(
        backend.execute(
            &execution("plan", "plan1"),
            &ijson(json!({"plan":"1. verify"}))
        ),
        BackendTerminal::Hold { ref scope, .. } if scope == "plan"
    ));

    let answer = ijson(json!({"selected":["A"]}));
    let resumed = backend.resume_after_approval(
        &execution("ask_user_questions", "ask1"),
        &ijson(json!({"question":"Which?","options":null})),
        &DurableApprovalResponse {
            request_seq: 10,
            response_seq: 11,
            scope: "answer".to_owned(),
            answer: Some(answer.clone()),
        },
    );
    assert_eq!(resumed, BackendTerminal::Completed(answer));

    let missing = backend.resume_after_approval(
        &execution("ask_user_questions", "ask2"),
        &ijson(json!({"question":"Which?","options":null})),
        &DurableApprovalResponse {
            request_seq: 12,
            response_seq: 13,
            scope: "answer".to_owned(),
            answer: None,
        },
    );
    assert!(matches!(
        missing,
        BackendTerminal::Unavailable { ref code, .. } if code == "missing_answer"
    ));
}

#[test]
fn discovery_and_skill_content_survive_backend_restart_without_side_authority() {
    let root = tempfile::tempdir().unwrap();
    let skill = CatalogEntry {
        name: "rust-review".to_owned(),
        summary: "Review Rust code".to_owned(),
        aliases: vec!["rust".to_owned()],
        schema_digest: "sha256-aa".to_owned(),
        content: ijson(json!({"skill":"full body"})),
    };
    let mut first =
        WorkflowBackend::new(root.path(), "ws", None, vec![skill.clone()], vec![]).unwrap();
    assert!(matches!(
        first.execute(
            &execution("skill_explorer", "offer1"),
            &ijson(json!({"query":"rust","limit":5}))
        ),
        BackendTerminal::Completed(_)
    ));
    assert!(!root.path().join("offers.jsonl").exists());
    drop(first);
    let mut second = WorkflowBackend::new(root.path(), "ws", None, vec![skill], vec![]).unwrap();
    let loaded = second.execute(
        &execution("skill", "load1"),
        &ijson(json!({"skill":"rust-review"})),
    );
    assert_eq!(
        loaded,
        BackendTerminal::Completed(ijson(json!({"skill":"full body"})))
    );
    assert!(!root.path().join("offers.jsonl").exists());
}

#[test]
fn goal_transactions_are_durable_not_process_memory() {
    let root = tempfile::tempdir().unwrap();
    let mut first = WorkflowBackend::new(
        root.path(),
        "ws",
        Some("goal-host-1".to_owned()),
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        first.execute(
            &execution("new_goal", "g1"),
            &ijson(json!({"goal":"finish","completion_criteria":"green","reason":"user"}))
        ),
        BackendTerminal::Completed(_)
    ));
    drop(first);

    let mut second = WorkflowBackend::new(
        root.path(),
        "ws",
        Some("goal-host-1".to_owned()),
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        second.execute(
            &execution("set_goal_state", "g2"),
            &ijson(
                json!({"state":"completed","progress":"all green","reason":null,"user_action":null})
            )
        ),
        BackendTerminal::Completed(_)
    ));
}

fn execution(name: &str, call: &str) -> ToolExecution {
    ToolExecution {
        thread: THREAD.to_owned(),
        call: call.to_owned(),
        name: name.to_owned(),
        attempt: "a1".to_owned(),
        invocation: ijson(json!({})),
        side_effectful: false,
        turn: 1,
        timestamp: "2026-08-26T09:00:00.000Z".to_owned(),
    }
}

fn ijson(value: Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap()
}

#[test]
fn retired_memory_tools_cannot_read_write_or_repair_the_legacy_archive() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("memory/log.jsonl");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    // Even a torn historical suffix must remain untouched.
    let archive = b"{\"op\":\"stage\"}\nunfinished";
    std::fs::write(&path, archive).unwrap();
    let mut backend = WorkflowBackend::new(root.path(), "ws", None, vec![], vec![]).unwrap();
    for name in ["note", "recall", "memorize"] {
        assert!(!backend.supports(name));
        assert!(
            matches!(backend.execute(&execution(name, name), &ijson(json!({}))),
            BackendTerminal::Unavailable { ref code, .. } if code == "unsupported")
        );
    }
    assert_eq!(std::fs::read(&path).unwrap(), archive);
    assert!(!path.with_extension("lock").exists());
}
