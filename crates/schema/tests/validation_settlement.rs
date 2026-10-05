use schema::Event;
use serde_json::{Value, json};

#[test]
fn validation_settlement_requires_terminal_outcome_and_prior_ordered_references() {
    let valid = json!({"v":1,"seq":12,"turn":1,"kind":"settle",
        "ts":"2026-09-04T00:00:00.000Z","outcome":"completed",
        "validation":{"outcome":"pass","promoted_output_seq":7,"candidate_seq":8,"decision_seq":11}});
    let accepts = |value: &Value| Event::decode(&serde_json::to_vec(value).unwrap()).is_ok();
    assert!(accepts(&valid));
    for outcome in ["pass", "inconclusive", "not_required"] {
        let mut value = valid.clone();
        value["validation"]["outcome"] = json!(outcome);
        assert!(accepts(&value));
    }
    for (field, invalid) in [
        ("outcome", json!("fail")),
        ("promoted_output_seq", json!(0)),
        ("promoted_output_seq", json!(8)),
        ("candidate_seq", json!(11)),
        ("decision_seq", json!(12)),
        ("decision_seq", json!("11")),
    ] {
        let mut value = valid.clone();
        value["validation"][field] = invalid;
        assert!(!accepts(&value), "{field}");
    }
    for field in [
        "outcome",
        "promoted_output_seq",
        "candidate_seq",
        "decision_seq",
    ] {
        let mut value = valid.clone();
        value["validation"].as_object_mut().unwrap().remove(field);
        assert!(!accepts(&value), "missing {field}");
    }
    let mut failed = valid.clone();
    failed["outcome"] = json!("error");
    failed["classification"] = json!("internal");
    assert!(!accepts(&failed));
    let mut legacy = valid;
    legacy.as_object_mut().unwrap().remove("validation");
    assert!(accepts(&legacy), "legacy records stay readable");
    legacy["promoted_output_seq"] = json!(7);
    assert!(
        accepts(&legacy),
        "a final answer may settle without independent validation"
    );
    for invalid in [json!(0), json!(12), json!("7")] {
        let mut direct = legacy.clone();
        direct["promoted_output_seq"] = invalid;
        assert!(!accepts(&direct));
    }
    let mut invalid = legacy;
    invalid["outcome"] = json!("interrupted");
    invalid["reason"] = json!("user_stop");
    assert!(!accepts(&invalid));
}
