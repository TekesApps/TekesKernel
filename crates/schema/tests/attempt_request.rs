use schema::Event;
use serde_json::json;

/// `attempt.request` is the exact transmitted request body as a thread asset
/// (`{asset, bytes}`). Every attempt carries it, and its shape is closed.
#[test]
fn attempt_request_is_a_required_closed_asset_reference() {
    let mut raw = json!({"v":1,"seq":4,"turn":1,"kind":"attempt",
        "ts":"2026-09-06T00:00:00.000Z","attempt":"a1","epoch":"e1","wire_digest":"w1",
        "admits":[{"from":2,"to":2}]});
    assert!(
        Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err(),
        "an attempt without its request body must be rejected"
    );
    raw["request"] = json!({"asset": format!("sha256-{}", "ab".repeat(32)), "bytes": 4096});
    let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).expect("attempt with request");
    let value = serde_json::to_value(event.raw()).unwrap();
    assert_eq!(value["request"]["bytes"], json!(4096));
    for broken in [
        json!({"asset": format!("sha256-{}", "ab".repeat(32))}),
        json!({"asset": format!("sha256-{}", "ab".repeat(32)), "bytes": 0}),
        json!({"asset": format!("sha256-{}", "ab".repeat(32)), "bytes": "4096"}),
        json!({"asset": format!("sha256-{}", "ab".repeat(32)), "bytes": 4096, "mime": "application/json"}),
        json!("sha256-abc"),
    ] {
        raw["request"] = broken.clone();
        assert!(
            Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err(),
            "{broken} must be rejected"
        );
    }
}
