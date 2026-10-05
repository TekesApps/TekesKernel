use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::Duration;

fn authority(state: &Path) -> PathBuf {
    let file = state.join("authority.json");
    std::fs::write(
        &file,
        serde_json::to_vec(
            &json!({"stateRoot":state,"allowedOperations":[],"allowedRepositoryRoots":[]}),
        )
        .unwrap(),
    )
    .unwrap();
    file
}
async fn call(root: &Path, authority: &Path, method: &str, request: Value) -> Value {
    workspace_service::process::invoke_with_authority(
        Path::new(env!("CARGO_BIN_EXE_tekes-workspace-service")),
        root,
        Some(authority),
        method,
        request,
        Duration::from_secs(10),
    )
    .await
    .unwrap()
}
fn edit(root: &Path, session: &str, turn: &str, event: &str, before: Value, after: Value) -> Value {
    json!({"workspaceId":"workspace","sessionId":session,"turnId":turn,"edit":{"eventId":event,"path":root.canonicalize().unwrap().join("file.txt"),"before":before,"after":after}})
}
fn query(session: &str, turn: &str) -> Value {
    json!({"workspaceId":"workspace","sessionId":session,"turnId":turn})
}

#[tokio::test]
async fn requested_diff_uses_recorded_snapshots_and_preserves_missing_newline() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "s",
            "t",
            "e",
            json!("same\nold"),
            json!("same\nnew\n"),
        ),
    )
    .await;
    // Today's file must not alter historical turn evidence.
    std::fs::write(root.path().join("file.txt"), "external").unwrap();
    let plain = call(root.path(), &auth, "turnChanges", query("s", "t")).await;
    assert!(plain["result"]["files"][0].get("unifiedDiff").is_none());
    let mut request = query("s", "t");
    request["includeDiff"] = json!(true);
    let response = call(root.path(), &auth, "turnChanges", request.clone()).await;
    let patch = response["result"]["files"][0]["unifiedDiff"]
        .as_str()
        .unwrap();
    assert!(
        patch.contains(" same\n-old\n\\ No newline at end of file\n+new\n"),
        "{patch}"
    );
    assert_eq!(response["result"]["additions"], 1);
    assert_eq!(response["result"]["deletions"], 1);
    call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "s",
            "t",
            "e2",
            json!("unexpected"),
            json!("final"),
        ),
    )
    .await;
    let discontinuous = call(root.path(), &auth, "turnChanges", request).await;
    assert!(
        discontinuous["result"]["files"][0]
            .get("unifiedDiff")
            .is_none()
    );
}

#[tokio::test]
async fn turn_net_replay_dedup_and_revert_without_git() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    let event = edit(
        root.path(),
        "session",
        "1",
        "write",
        Value::Null,
        json!("alpha\nbeta\n"),
    );
    let first = call(root.path(), &auth, "recordTurnEdit", event.clone()).await;
    assert_eq!(first["result"]["additions"], 2, "{first}");
    // Each call boots a fresh process; reads and duplicate delivery survive restart.
    let recovered = call(root.path(), &auth, "turnChanges", query("session", "1")).await;
    assert_eq!(first, recovered);
    assert_eq!(
        first,
        call(root.path(), &auth, "recordTurnEdit", event).await
    );
    let second = call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "session",
            "1",
            "edit",
            json!("alpha\nbeta\n"),
            json!("alpha\ngamma\ndelta\n"),
        ),
    )
    .await;
    assert_eq!(second["result"]["additions"], 3);
    assert_eq!(second["result"]["deletions"], 0);
    assert_eq!(second["result"]["revision"], 4);
    let other_turn = call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "session",
            "2",
            "edit2",
            json!("alpha\ngamma\ndelta\n"),
            json!("alpha\nbeta\n"),
        ),
    )
    .await;
    assert_eq!(other_turn["result"]["additions"], 1);
    assert_eq!(other_turn["result"]["deletions"], 2);
    let reverted = call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "session",
            "1",
            "remove",
            json!("alpha\ngamma\ndelta\n"),
            Value::Null,
        ),
    )
    .await;
    assert_eq!(reverted["result"]["fileCount"], 0);
    assert_eq!(reverted["result"]["additions"], 0);
    assert!(!root.path().join(".git").exists());
}

#[tokio::test]
async fn sessions_are_isolated_and_discontinuity_is_not_exact_count() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    let first = edit(
        root.path(),
        "one",
        "1",
        "same-event",
        json!("a\n"),
        json!("b\n"),
    );
    let second = edit(
        root.path(),
        "two",
        "1",
        "same-event",
        json!("a\n"),
        json!("c\nd\n"),
    );
    let (one, two) = tokio::join!(
        call(root.path(), &auth, "recordTurnEdit", first.clone()),
        call(root.path(), &auth, "recordTurnEdit", second)
    );
    assert_eq!(one["result"]["additions"], 1);
    assert_eq!(two["result"]["additions"], 2);
    let conflict = call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "one",
            "1",
            "same-event",
            json!("a\n"),
            json!("other\n"),
        ),
    )
    .await;
    assert_eq!(conflict["error"]["code"], "event-conflict");
    let discontinuous = call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(
            root.path(),
            "one",
            "1",
            "next",
            json!("external\n"),
            json!("final\n"),
        ),
    )
    .await;
    assert_eq!(discontinuous["result"]["state"], "discontinuous");
    assert!(discontinuous["result"]["additions"].is_null());
    assert_eq!(
        call(root.path(), &auth, "turnChanges", query("two", "1")).await,
        two
    );
    let unobserved = call(root.path(), &auth, "turnChanges", query("none", "1")).await;
    assert_eq!(unobserved["result"]["state"], "unobserved");
    assert!(unobserved["result"]["additions"].is_null());
}

#[tokio::test]
async fn partial_durable_record_is_an_error() {
    use std::io::Write;
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    call(
        root.path(),
        &auth,
        "recordTurnEdit",
        edit(root.path(), "one", "1", "event", Value::Null, json!("a\n")),
    )
    .await;
    let ledger = std::fs::read_dir(state.path().join("turn-changes"))
        .unwrap()
        .map(Result::unwrap)
        .find(|e| e.path().extension().is_some_and(|e| e == "jsonl"))
        .unwrap()
        .path();
    std::fs::OpenOptions::new()
        .append(true)
        .open(ledger)
        .unwrap()
        .write_all(b"{partial")
        .unwrap();
    let response = call(root.path(), &auth, "turnChanges", query("one", "1")).await;
    assert_eq!(response["error"]["code"], "corrupt-ledger");
}

#[tokio::test]
async fn prepared_edit_survives_crash_as_incomplete_until_success_receipt() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    let event = edit(
        root.path(),
        "session",
        "1",
        "pending",
        Value::Null,
        json!("alpha\n"),
    );
    let pending = call(root.path(), &auth, "prepareTurnEdit", event.clone()).await;
    assert_eq!(pending["result"]["state"], "discontinuous", "{pending}");
    assert!(pending["result"]["additions"].is_null());
    // The write occurred, but a process exited before delivering its success receipt.
    std::fs::write(root.path().join("file.txt"), "alpha\n").unwrap();
    let recovered = call(root.path(), &auth, "turnChanges", query("session", "1")).await;
    assert_eq!(recovered, pending);
    let committed = call(root.path(), &auth, "recordTurnEdit", event.clone()).await;
    assert_eq!(committed["result"]["additions"], 1);
    assert!(
        committed["result"]["revision"].as_u64().unwrap()
            > pending["result"]["revision"].as_u64().unwrap()
    );
    assert_eq!(
        committed,
        call(root.path(), &auth, "recordTurnEdit", event).await
    );
}

#[tokio::test]
async fn definitive_failed_edit_aborts_intent_without_counting_content() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    let event = edit(
        root.path(),
        "session",
        "1",
        "denied",
        Value::Null,
        json!("not written\n"),
    );
    let pending = call(root.path(), &auth, "prepareTurnEdit", event.clone()).await;
    let aborted = call(root.path(), &auth, "abortTurnEdit", event.clone()).await;
    assert_eq!(aborted["result"]["fileCount"], 0);
    assert_eq!(aborted["result"]["additions"], 0);
    assert!(
        aborted["result"]["revision"].as_u64().unwrap()
            > pending["result"]["revision"].as_u64().unwrap()
    );
    assert_eq!(
        call(root.path(), &auth, "recordTurnEdit", event).await["error"]["code"],
        "event-conflict"
    );
}

#[tokio::test]
async fn registered_secondary_root_and_relocated_workspace_preserve_history() {
    let root = tempfile::tempdir().unwrap();
    let secondary = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    std::fs::write(&auth,serde_json::to_vec(&json!({"stateRoot":state.path(),"workspaceRoots":[secondary.path()],"allowedOperations":[],"allowedRepositoryRoots":[]})).unwrap()).unwrap();
    let event = edit(
        secondary.path(),
        "session",
        "1",
        "secondary",
        Value::Null,
        json!("other root\n"),
    );
    let recorded = call(root.path(), &auth, "recordTurnEdit", event).await;
    assert_eq!(recorded["result"]["additions"], 1, "{recorded}");
    let relocated = tempfile::tempdir().unwrap();
    assert_eq!(
        call(
            relocated.path(),
            &auth,
            "turnChanges",
            query("session", "1")
        )
        .await,
        recorded
    );
    let outside = tempfile::tempdir().unwrap();
    assert_eq!(
        call(
            root.path(),
            &auth,
            "recordTurnEdit",
            edit(
                outside.path(),
                "session",
                "1",
                "outside",
                Value::Null,
                json!("denied")
            )
        )
        .await["error"]["code"],
        "path-outside-workspace"
    );
}

#[tokio::test]
async fn omitted_large_content_persists_unknown_counts_and_conflict_detection() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let auth = authority(state.path());
    let mut event = edit(root.path(), "large", "1", "event", Value::Null, Value::Null);
    event["edit"]["omittedContentHashes"] = json!([null, "a".repeat(64)]);
    let prepared = call(root.path(), &auth, "prepareTurnEdit", event.clone()).await;
    assert!(prepared.get("error").is_none(), "{prepared}");
    let recorded = call(root.path(), &auth, "recordTurnEdit", event.clone()).await;
    assert!(recorded.get("error").is_none(), "{recorded}");
    let snapshot = call(root.path(), &auth, "turnChanges", query("large", "1")).await;
    assert_eq!(snapshot["result"]["fileCount"], 1, "{snapshot}");
    assert!(snapshot["result"]["additions"].is_null(), "{snapshot}");
    assert!(snapshot["result"]["deletions"].is_null(), "{snapshot}");
    assert_eq!(
        recorded,
        call(root.path(), &auth, "recordTurnEdit", event.clone()).await
    );
    event["edit"]["omittedContentHashes"] = json!([null, "b".repeat(64)]);
    assert!(
        call(root.path(), &auth, "recordTurnEdit", event)
            .await
            .get("error")
            .is_some()
    );
}
