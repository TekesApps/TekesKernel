//! event §compact `summary_request`: the summary request's telemetry is an
//! optional, validated object on the compact event; its frozen bundle and
//! verdict are the contract, the artifact fields are open.
use schema::{Event, IJsonValue};
use serde_json::{Value, json};

fn compact(request: Option<Value>) -> Result<Event, schema::SchemaError> {
    let mut object = json!({"v":1,"seq":9,"kind":"compact","ts":"2026-09-05T00:00:00.000Z","covers":[{"from":2,"to":5}],"summary":"[compacted history]\nx"});
    if let Some(request) = request {
        object["summary_request"] = request;
    }
    Event::from_value(IJsonValue::parse(&serde_json::to_vec(&object).unwrap()).unwrap())
}

#[test]
fn compact_summary_request_is_optional_and_validated() {
    compact(None).expect("a compact without a summary request stays valid");
    let valid = json!({"bundle":{"covers":[{"from":2,"to":5}],"sha256":"sha256-ab","bytes":10,"truncated":false},"accepted":true,"evidence_refs":[2,3],"model":"m","usage":{"input_tokens":"1"}});
    compact(Some(valid.clone())).expect("a full record validates");
    compact(Some(json!({"bundle":{"covers":[{"from":2,"to":2}],"sha256":"sha256-ab"},"accepted":false,"reason":"provider failure"}))).expect("a rejected record validates");
    for (label, mutate) in [
        (
            "missing bundle",
            Box::new(|s: &mut Value| {
                s.as_object_mut().unwrap().remove("bundle");
            }) as Box<dyn Fn(&mut Value)>,
        ),
        (
            "empty digest",
            Box::new(|s: &mut Value| s["bundle"]["sha256"] = json!("")),
        ),
        (
            "covers not ranges",
            Box::new(|s: &mut Value| s["bundle"]["covers"] = json!([2, 3])),
        ),
        (
            "accepted not bool",
            Box::new(|s: &mut Value| s["accepted"] = json!("yes")),
        ),
        (
            "not an object",
            Box::new(|s: &mut Value| *s = json!("summary")),
        ),
    ] {
        let mut request = valid.clone();
        mutate(&mut request);
        assert!(compact(Some(request)).is_err(), "{label} must be rejected");
    }
}
