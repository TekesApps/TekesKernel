use serde_json::{Value, json};

#[test]
#[ignore = "live DeepSeek credentials; scripts/run-live-deepseek-continuation.py"]
fn deepseek_lookup_continuation_is_bounded() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(configured.dialect, "deepseek_responses_v1");
    let model = configured.models.iter().find(|m| m.enabled).unwrap();
    let resolved = provider::resolve_profile(&configured, model).unwrap();
    let settings = profile::SessionSettings {
        format: 1,
        revision: 1,
        provider: configured.id.clone(),
        model: model.id.clone(),
        reasoning_effort: Some("high".into()),
    };
    let ij = |v: Value| {
        schema::IJsonValue::parse(&serde_json_canonicalizer::to_vec(&v).unwrap()).unwrap()
    };
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let mut items = vec![ij(
        json!({"role":"user","content":[{"type":"text","text":"Read docs/known.md."}]}),
    )];
    let started = std::time::Instant::now();
    let mut receipt = json!({"status":"running","model":model.id,"rounds":[],"scope":"Legacy lookup_file argument and two-response high-reasoning contract; production encoder and dialect transport. No requested/effective metadata API equivalence claimed."});
    for round in 0..2 {
        let system = if round == 0 {
            "Call lookup_file exactly once. Use path docs/known.md and a short plain reason."
        } else {
            "Report the received lookup result. Do not call another tool."
        };
        let catalog = if round == 0 {
            json!([{"name":"lookup_file","description":"Read one known file path.","parameters":{"type":"object","properties":{"path":{"type":"string"},"reason":{"type":"string"}},"required":["path","reason"],"additionalProperties":false}}])
        } else {
            json!([])
        };
        let request = provider::prepare(&provider::PrepareInput {
            attempt_id: format!("deepseek-lookup-{round}"),
            target: resolved.target.clone(),
            endpoint: configured.endpoint.clone(),
            epoch_profile: provider::epoch_profile(&resolved, system, Some(&settings)).unwrap(),
            continuation_id: None,
            stream: false,
            rendered_items: items.clone(),
            tool_catalog: ij(catalog),
        })
        .unwrap();
        std::fs::write(
            directory.join(format!("round-{round}.request.json")),
            &request.body,
        )
        .unwrap();
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["reasoning"]["effort"], "high");
        assert!(body.get("tool_choice").is_none());
        let remaining = std::time::Duration::from_secs(120)
            .checked_sub(started.elapsed())
            .expect("two-minute bound exceeded");
        let result = runtime.send_dialect_with_frames_and_wall_transport(
            provider::DialectId::DeepseekResponsesV1,
            &request,
            None,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            Some(remaining),
            |_| Ok(()),
        );
        let raw = String::from_utf8_lossy(&capture.lock().unwrap()).replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("round-{round}.response.json")), raw).unwrap();
        let t = match result {
            Ok(provider::ProviderCompletion::Terminal(t)) => t,
            other => panic!("{}", format!("{other:?}").replace(&key, "[REDACTED]")),
        };
        assert!(t.http_status.is_none_or(|s| (200..300).contains(&s)));
        receipt["rounds"].as_array_mut().unwrap().push(json!({"round":round,"request_digest":request.request_digest,"final_answer":t.is_final_answer(),"tool_count":t.tool_calls.len()}));
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        if round == 0 {
            assert_eq!(t.tool_calls.len(), 1);
            assert!(!t.is_final_answer());
            let call = &t.tool_calls[0];
            assert_eq!(call.name, "lookup_file");
            assert!(!call.call_id.is_empty());
            let args = call.arguments.as_object().unwrap();
            assert_eq!(args.len(), 2);
            assert_eq!(args["path"], "docs/known.md");
            assert!(args["reason"].as_str().unwrap().chars().count() < 512);
            items.push(ij(json!({"role":"sealed","adapter":"deepseek_responses_v1","fragments":t.sealed_fragments})));
            items.push(ij(json!({"role":"tool","content":[{"type":"tool_result","call_id":call.call_id,"name":call.name,"result":"KNOWN_CONTENT"}]})));
            items.push(ij(json!({"role":"user","content":[{"type":"text","text":"Finish from the one lookup result."}]})));
        } else {
            assert!(t.tool_calls.is_empty());
            assert!(t.is_final_answer());
            assert!(t.content.iter().any(
                |c| matches!(c,provider::ContentBlock::Text(s) if s.contains("KNOWN_CONTENT"))
            ));
        }
    }
    assert!(started.elapsed() < std::time::Duration::from_secs(120));
    receipt["status"] = json!("passed");
    receipt["elapsed_seconds"] = json!(started.elapsed().as_secs_f64());
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}
