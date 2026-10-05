use serde_json::{Value, json};

fn rows() -> Vec<Value> {
    include_str!("../../../fixtures/live/flow/inline-approval-batch-prefix.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn facts(rows: &[Value]) -> schema::LifecycleFacts {
    let mut bytes = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let mut row = row.clone();
        row["seq"] = json!(index + 1);
        bytes.extend(serde_json_canonicalizer::to_vec(&row).unwrap());
        bytes.push(b'\n');
    }
    schema::validate_ledger(&bytes, 1).unwrap().lifecycle
}

#[test]
fn an_unstarted_sibling_waits_for_its_same_batch_approval() {
    let original = rows();
    let current = facts(&original);
    assert!(current.open_hold);
    assert!(
        !current.unresolved_work,
        "tracked later sibling must not cause recovery to revoke the pending hold"
    );

    let mut legacy = original.clone();
    let sibling = legacy
        .iter_mut()
        .find(|r| r["kind"] == "tool_call" && r["call"] == "flow-sibling-2")
        .unwrap();
    sibling.as_object_mut().unwrap().remove("execution_tracked");
    assert!(
        facts(&legacy).unresolved_work,
        "legacy absence of an execution marker proves nothing"
    );

    let mut started = original.clone();
    started.push(json!({"v":1,"seq":18,"kind":"tool_execution_started","ts":"2026-09-07T00:00:00.000Z","turn":1,"call":"flow-sibling-2"}));
    assert!(
        facts(&started).unresolved_work,
        "an already-started sibling must not be hidden by a hold"
    );

    let mut other_batch = original.clone();
    other_batch
        .iter_mut()
        .find(|r| r["kind"] == "tool_call" && r["call"] == "flow-sibling-2")
        .unwrap()["attempt"] = json!("another-batch");
    assert!(
        facts(&other_batch).unresolved_work,
        "a hold cannot hide another batch's pending call"
    );
}
