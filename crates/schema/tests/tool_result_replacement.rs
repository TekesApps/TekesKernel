use schema::validate_ledger;
use serde_json::{Value, json};

const TS: &str = "2026-09-06T00:00:00.000Z";
const THREAD: &str = "018f0000-0000-7000-8000-000000000003";

fn ledger(rows: Vec<Value>) -> Vec<u8> {
    let mut events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"go"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
        json!({"v":1,"seq":3,"kind":"turn_open","ts":TS,"turn":1,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"shell","args":{},"source":"provider"}),
        json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"whole output"}]}),
    ];
    events.extend(rows);
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical"));
        bytes.push(b'\n');
    }
    bytes
}

/// Constraint 3 keeps one live result per call. A trim is expressed as a
/// replacement that retracts the result it replaces, so the pairing stays 1:1
/// while the whole output stays on disk; an unexplained second result is
/// still a corrupt ledger.
#[test]
fn a_second_tool_result_is_legal_only_as_an_explicit_replacement() {
    let replacement = json!({"v":1,"seq":6,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok",
        "content":[{"type":"text","text":"whole [...] output"}],"supersedes":[{"from":5,"to":5}]});
    validate_ledger(&ledger(vec![replacement.clone()]), 1)
        .expect("a retracting replacement is legal");

    let mut duplicate = replacement.clone();
    duplicate.as_object_mut().unwrap().remove("supersedes");
    let error =
        validate_ledger(&ledger(vec![duplicate]), 1).expect_err("a bare duplicate is rejected");
    assert!(error.to_string().contains("supersede"), "{error}");

    let mut elsewhere = replacement.clone();
    elsewhere["supersedes"] = json!([{"from":4,"to":4}]);
    let error = validate_ledger(&ledger(vec![elsewhere]), 1)
        .expect_err("retracting something other than the result it replaces is rejected");
    assert!(error.to_string().contains("supersede"), "{error}");

    // A third result must retract the second, not the original.
    let chained = vec![
        replacement,
        json!({"v":1,"seq":7,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok",
            "content":[{"type":"text","text":"shorter"}],"supersedes":[{"from":6,"to":6}]}),
    ];
    validate_ledger(&ledger(chained), 1).expect("a chained replacement retracts the live result");
}
