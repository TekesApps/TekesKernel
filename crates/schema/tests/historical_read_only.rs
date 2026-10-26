use schema::{Event, LedgerValidator, READER_VERSION, validate_historical_ledger, validate_ledger};
use serde_json::Value;

fn canonical_line(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).expect("canonical JSON")
}

fn legacy_fixture() -> Vec<u8> {
    let source = include_str!("../../../fixtures/invalid/_valid-baseline.jsonl");
    let mut result = Vec::new();
    for line in source.lines() {
        let mut value: Value = serde_json::from_str(line).expect("fixture JSON");
        match value["kind"].as_str().expect("kind") {
            "run_start" => value["binary"] = Value::String("dev-1e0ae8a671de-123456789abc".into()),
            "epoch" => {
                let digest = value["tools"]["digest"]
                    .as_str()
                    .expect("tools digest")
                    .to_owned();
                value.as_object_mut().expect("event object").remove("tools");
                value["tools_digest"] = Value::String(format!("sha256-{}", "a".repeat(64)));
                assert!(!digest.is_empty());
            }
            "output" | "error" => {
                value.as_object_mut().unwrap().remove("usage");
            }
            _ => {}
        }
        result.extend(canonical_line(&value));
        result.push(b'\n');
    }
    result
}

fn legacy_usage_fixture() -> Vec<u8> {
    let source = String::from_utf8(legacy_fixture()).unwrap();
    let mut result = Vec::new();
    let mut injected = false;
    for line in source.lines() {
        let mut value: Value = serde_json::from_str(line).expect("fixture JSON");
        match value["kind"].as_str().expect("kind") {
            "output" => {
                let dispatched = serde_json::json!({
                    "kind": "attempt_dispatched", "attempt": "a1", "turn": 1,
                    "seq": 7, "ts": "2026-08-26T09:00:00.000Z", "v": 1
                });
                result.extend(canonical_line(&dispatched));
                result.push(b'\n');
                let usage = serde_json::json!({
                    "kind": "usage", "attempt": "a1", "turn": 1,
                    "availability": "reported", "input_tokens": "4",
                    "output_tokens": "2", "seq": 8,
                    "ts": "2026-08-26T09:00:00.000Z", "v": 1
                });
                result.extend(canonical_line(&usage));
                result.push(b'\n');
                value.as_object_mut().unwrap().remove("usage");
                value["seq"] = Value::from(9);
                injected = true;
            }
            "settle" => value["seq"] = Value::from(10),
            _ => {}
        }
        result.extend(canonical_line(&value));
        result.push(b'\n');
    }
    assert!(injected);
    result
}

fn rewrite_event(bytes: &[u8], kind: &str, field: &str, value: Value) -> Vec<u8> {
    bytes
        .split_inclusive(|byte| *byte == b'\n')
        .flat_map(|line| {
            let mut event: Value =
                serde_json::from_slice(&line[..line.len() - 1]).expect("event JSON");
            if event["kind"] == kind {
                event[field] = value.clone();
            }
            let mut canonical = canonical_line(&event);
            canonical.push(b'\n');
            canonical
        })
        .collect()
}

#[test]
fn historical_ledger_is_preserved_and_cannot_enter_execution_fold() {
    let bytes = legacy_fixture();
    assert!(validate_ledger(&bytes, READER_VERSION).is_err());

    let historical =
        validate_historical_ledger(&bytes, READER_VERSION).expect("known legacy history");
    assert_eq!(historical.last_seq(), 8);
    assert_eq!(historical.events().len(), 8);
    let original_lines: Vec<_> = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .collect();
    for (event, original) in historical.events().iter().zip(original_lines) {
        assert!(event.is_historical_only());
        assert_eq!(event.canonical_bytes().expect("event bytes"), original);
    }

    let mut execution = LedgerValidator::new(READER_VERSION);
    let error = execution
        .push(historical.events()[0].clone())
        .expect_err("history-only marker");
    assert!(error.to_string().contains("historical-only"));

    let old_epoch = historical
        .events()
        .iter()
        .find(|event| event.kind().as_str() == "epoch")
        .unwrap();
    assert!(Event::decode_canonical(&old_epoch.canonical_bytes().unwrap()).is_err());
}

#[test]
fn historical_compatibility_is_bound_to_proven_writer_and_format() {
    let bytes = legacy_fixture();
    let unknown_writer = rewrite_event(
        &bytes,
        "run_start",
        "binary",
        Value::String("dev-deadbeefdead-123456789abc".into()),
    );
    assert!(validate_historical_ledger(&unknown_writer, READER_VERSION).is_err());

    let wrong_format = rewrite_event(&bytes, "genesis", "format", Value::from(2));
    assert!(validate_historical_ledger(&wrong_format, READER_VERSION).is_err());

    let bad_digest = rewrite_event(
        &bytes,
        "epoch",
        "tools_digest",
        Value::String("not-a-digest".into()),
    );
    assert!(validate_historical_ledger(&bad_digest, READER_VERSION).is_err());

    let null_digest = rewrite_event(&bytes, "epoch", "tools_digest", Value::Null);
    assert!(validate_historical_ledger(&null_digest, READER_VERSION).is_err());

    let missing_digest = bytes
        .split_inclusive(|byte| *byte == b'\n')
        .flat_map(|line| {
            let mut event: Value = serde_json::from_slice(&line[..line.len() - 1]).unwrap();
            if event["kind"] == "epoch" {
                event.as_object_mut().unwrap().remove("tools_digest");
            }
            let mut canonical = canonical_line(&event);
            canonical.push(b'\n');
            canonical
        })
        .collect::<Vec<_>>();
    assert!(validate_historical_ledger(&missing_digest, READER_VERSION).is_err());

    let wrong_min_reader = rewrite_event(&bytes, "genesis", "min_reader", Value::from(2));
    assert!(validate_historical_ledger(&wrong_min_reader, READER_VERSION).is_err());

    let empty_line = [bytes.as_slice(), b"\n"].concat();
    assert!(validate_historical_ledger(&empty_line, READER_VERSION).is_err());

    let unknown_extension = rewrite_event(
        &bytes,
        "output",
        "kind",
        Value::String("old_note_extension".into()),
    );
    assert!(validate_historical_ledger(&unknown_extension, READER_VERSION).is_err());
}

#[test]
fn historical_usage_keeps_its_old_schema_and_attempt_order_checks() {
    let bytes = legacy_usage_fixture();
    let historical =
        validate_historical_ledger(&bytes, READER_VERSION).expect("legacy usage history");
    assert_eq!(historical.last_seq(), 10);
    assert!(
        historical
            .events()
            .iter()
            .any(|event| event.kind().as_str() == "usage")
    );

    let orphan_usage = bytes
        .split_inclusive(|byte| *byte == b'\n')
        .flat_map(|line| {
            let mut event: Value = serde_json::from_slice(&line[..line.len() - 1]).unwrap();
            if event["kind"] == "usage" {
                event["attempt"] = Value::String("missing-attempt".into());
            }
            let mut canonical = canonical_line(&event);
            canonical.push(b'\n');
            canonical
        })
        .collect::<Vec<_>>();
    assert!(validate_historical_ledger(&orphan_usage, READER_VERSION).is_err());

    let malformed_usage = bytes
        .split_inclusive(|byte| *byte == b'\n')
        .flat_map(|line| {
            let mut event: Value = serde_json::from_slice(&line[..line.len() - 1]).unwrap();
            if event["kind"] == "usage" {
                event["cache_write"] = Value::from(4);
            }
            let mut canonical = canonical_line(&event);
            canonical.push(b'\n');
            canonical
        })
        .collect::<Vec<_>>();
    assert!(validate_historical_ledger(&malformed_usage, READER_VERSION).is_err());
}
