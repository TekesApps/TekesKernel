use serde_json::{Value, json};

#[test]
#[ignore = "live provider credentials and an existing compact ledger; TEKES_SCHEMA_OUTPUT/KEY/SOURCE"]
fn production_compactor_admits_frozen_history() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    let model = configured.models.iter().find(|m| m.enabled).unwrap();
    let resolved = provider::resolve_profile(&configured, model).unwrap();
    let ij = |v: Value| {
        schema::IJsonValue::parse(&serde_json_canonicalizer::to_vec(&v).unwrap()).unwrap()
    };
    let source = std::fs::read_to_string(std::env::var("TEKES_SCHEMA_SOURCE").unwrap()).unwrap();
    let rows: Vec<Value> = source
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let compact = rows.iter().find(|r| r["kind"] == "compact").unwrap();
    let covers: Vec<u64> = compact["covers"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|r| r["from"].as_u64().unwrap()..=r["to"].as_u64().unwrap())
        .collect();
    let events: Vec<schema::Event> = rows
        .iter()
        .map(|r| schema::Event::from_value(ij(r.clone())).unwrap())
        .collect();
    let bundle = engine::freeze_source_bundle(
        &events,
        &covers,
        engine::summary_request_bytes(model.context_window_tokens),
    )
    .unwrap();
    let catalog = ij(
        json!([{"name":"summary_artifact","description":"Produce one minimal sufficient continuation from the frozen accepted history; evidence_refs are the seq numbers of the history records it is drawn from.",
        "parameters":{"type":"object","properties":{"continuation":{"type":"string","minLength":1},
            "evidence_refs":{"type":"array","items":{"type":"integer","minimum":1},"minItems":1,"uniqueItems":true}},
            "required":["continuation","evidence_refs"],"additionalProperties":false}}]),
    );
    let request = provider::prepare_summary_request(
        &configured.endpoint,
        &resolved,
        engine::SUMMARY_SYSTEM,
        &bundle.rendered,
        catalog,
        "live-frozen-summary".into(),
    )
    .unwrap();
    std::fs::write(directory.join("request.json"), &request.body).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let completion = runtime.send_dialect_with_frames_and_wall_transport(
        resolved.dialect,
        &request,
        None,
        &key,
        &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        Some(std::time::Duration::from_secs(120)),
        |_| Ok(()),
    );
    std::fs::write(
        directory.join("response.json"),
        String::from_utf8_lossy(&capture.lock().unwrap()).replace(&key, "[REDACTED]"),
    )
    .unwrap();
    let (artifact, usage) = provider::summary_completion_artifact(completion);
    let admitted = artifact.and_then(|a| engine::admit_summary_artifact(&bundle, &a));
    let receipt = json!({"passed":admitted.is_ok(),"model":model.id,"usage":usage,
        "source_bundle":bundle.sha256,"covered_records":covers.len(),"request_digest":request.request_digest,
        "result":format!("{admitted:?}").replace(&key,"[REDACTED]"),
        "scope":"Production request builder, transport, normalizer and frozen-history admission; supplemental compactor proof, not a four-turn worker run."});
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    assert!(admitted.is_ok(), "inspect retained compactor receipt");
}
