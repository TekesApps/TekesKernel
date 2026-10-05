use schema::Event;
use serde_json::json;

/// The `usage` object rides on the output that settles its attempt: an explicit
/// `cache_miss` is retained losslessly, `unavailable` usage carries no figures,
/// figures are strings, and an output without `usage` is not an outcome.
#[test]
fn explicit_cache_miss_is_lossless_and_cannot_accompany_unavailable_usage() {
    let mut raw = json!({"v":1,"seq":2,"turn":1,"kind":"output",
        "ts":"2026-09-04T00:00:00.000Z","attempt":"a1","content":[],
        "sealed":{"version":1,"adapter":"fake","fragments":"[]"},
        "usage":{"availability":"reported","cache_read":"0","cache_miss":"9007199254740993"}});
    let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
    let usage = event.usage().unwrap();
    assert_eq!(
        usage.get("cache_miss").and_then(|value| value.as_str()),
        Some("9007199254740993")
    );
    let round_trip = Event::decode(&event.canonical_bytes().unwrap()).unwrap();
    assert_eq!(round_trip.usage(), event.usage());
    raw["usage"]["availability"] = json!("unavailable");
    raw["usage"].as_object_mut().unwrap().remove("cache_read");
    assert!(Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err());
    raw["usage"]["availability"] = json!("reported");
    raw["usage"]["cache_miss"] = json!(12);
    assert!(Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err());
    raw.as_object_mut().unwrap().remove("usage");
    assert!(
        Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err(),
        "an outcome carries its usage"
    );
}

/// `cache_write` (tokens written to the provider cache, billed above the input
/// rate) is a first-class figure next to `cache_read`; it stays a string and is
/// rejected on `unavailable` usage like every other figure.
#[test]
fn cache_write_is_a_reported_figure() {
    let mut raw = json!({"v":1,"seq":2,"turn":1,"kind":"output",
        "ts":"2026-09-04T00:00:00.000Z","attempt":"a1","content":[],
        "sealed":{"version":1,"adapter":"fake","fragments":"[]"},
        "usage":{"availability":"reported","cache_read":"700","cache_write":"300"}});
    let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
    assert_eq!(
        event
            .usage()
            .unwrap()
            .get("cache_write")
            .and_then(|value| value.as_str()),
        Some("300")
    );
    raw["usage"]["cache_write"] = json!(300);
    assert!(
        Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err(),
        "figures are decimal strings"
    );
    raw["usage"] = json!({"availability":"unavailable","cache_write":"300"});
    assert!(Event::decode(&serde_json::to_vec(&raw).unwrap()).is_err());
}
