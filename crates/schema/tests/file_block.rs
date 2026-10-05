use schema::{Block, validate_ledger};
use serde_json::{Value, json};

const TS: &str = "2026-09-13T00:00:00.000Z";
const THREAD: &str = "018f0000-0000-7000-8000-000000000004";
const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn ledger_with_input(content: Value) -> Vec<u8> {
    let events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":content,"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
    ];
    let mut bytes = Vec::new();
    for event in events {
        bytes.extend(serde_json_canonicalizer::to_vec(&event).expect("canonical"));
        bytes.push(b'\n');
    }
    bytes
}

fn file_block() -> Value {
    json!({"type":"file","asset":format!("sha256-{DIGEST}"),"mime":"application/pdf","name":"spec.pdf","bytes":1234})
}

#[test]
fn file_block_round_trips_through_the_typed_block_and_the_validator() {
    let block: Block = serde_json::from_value(file_block()).expect("typed file block");
    assert_eq!(
        block,
        Block::File {
            asset: format!("sha256-{DIGEST}"),
            mime: "application/pdf".to_owned(),
            name: "spec.pdf".to_owned(),
            bytes: 1234,
        }
    );
    assert_eq!(
        serde_json::to_value(&block).expect("serialize"),
        file_block()
    );
    validate_ledger(&ledger_with_input(json!([file_block()])), 1)
        .expect("file block is a legal input block");
    validate_ledger(
        &ledger_with_input(json!([{"type":"tool-result","call":"c1","content":[file_block()]}])),
        1,
    )
    .expect("file block nests inside tool-result content");
}

#[test]
fn file_block_rejects_missing_or_malformed_fields() {
    let cases: Vec<(&str, Value)> = vec![
        (
            "missing asset",
            json!({"type":"file","mime":"text/plain","name":"a.txt","bytes":1}),
        ),
        (
            "missing mime",
            json!({"type":"file","asset":"sha256-x","name":"a.txt","bytes":1}),
        ),
        (
            "missing name",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","bytes":1}),
        ),
        (
            "missing bytes",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a.txt"}),
        ),
        (
            "zero bytes",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a.txt","bytes":0}),
        ),
        (
            "negative bytes",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a.txt","bytes":-1}),
        ),
        (
            "fractional bytes",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a.txt","bytes":1.5}),
        ),
        (
            "unsafe bytes",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a.txt","bytes":9007199254740992_u64}),
        ),
        (
            "empty name",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"","bytes":1}),
        ),
        (
            "long name",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"x".repeat(256),"bytes":1}),
        ),
        (
            "control name",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"a\u{0}b","bytes":1}),
        ),
        (
            "slash name",
            json!({"type":"file","asset":"sha256-x","mime":"text/plain","name":"dir/a.txt","bytes":1}),
        ),
    ];
    for (label, block) in cases {
        validate_ledger(&ledger_with_input(json!([block])), 1)
            .expect_err(&format!("{label} must be rejected"));
    }
    validate_ledger(
        &ledger_with_input(json!([{"type":"file","asset":"sha256-x","mime":"text/plain","name":"x".repeat(255),"bytes":1}])),
        1,
    )
    .expect("255-byte name is the inclusive maximum");
}
