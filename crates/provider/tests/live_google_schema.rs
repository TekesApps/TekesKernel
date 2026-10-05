use provider::{
    PrepareInput, ToolChoice, epoch_profile, prepare_with_tool_choice, resolve_profile,
};
use schema::IJsonValue;
use serde_json::{Value, json};

#[test]
#[ignore = "real Google credentials; scripts/run-live-google-schema.py --scenario thinking"]
fn google_thinking_tool_result_preserves_signature() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let provider: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(provider.dialect, "google_generation_v1");
    let model = provider.models.iter().find(|m| m.enabled).unwrap();
    let resolved = resolve_profile(&provider, model).unwrap();
    let settings = profile::SessionSettings {
        format: 1,
        revision: 1,
        provider: provider.id.clone(),
        model: model.id.clone(),
        reasoning_effort: Some("high".into()),
    };
    let ij = |v: Value| IJsonValue::parse(&serde_json_canonicalizer::to_vec(&v).unwrap()).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let mut items = vec![ij(
        json!({"role":"user","content":[{"type":"text","text":"Use lookup to calculate 21 * 2."}]}),
    )];
    let catalog = ij(
        json!([{"name":"lookup","description":"Look up the exact answer to a question.","strict":true,"parameters":{"type":"object","properties":{"question":{"type":"string"}},"required":["question"],"additionalProperties":false}}]),
    );
    let mut receipt = json!({"status":"running","model":model.id,"rounds":[],"scope":"Production Google high-thinking, required lookup then sealed signature replay and final answer; raw reasoning usage retained."});
    let mut sealed = Value::Null;
    for round in 0..2 {
        let system = if round == 0 {
            "Call lookup exactly once. Do not answer in prose before the tool result."
        } else {
            "Return the lookup result as the final answer. Do not call another tool."
        };
        let request = prepare_with_tool_choice(
            &PrepareInput {
                attempt_id: format!("google-thinking-{round}"),
                target: resolved.target.clone(),
                endpoint: provider.endpoint.clone(),
                epoch_profile: epoch_profile(&resolved, system, Some(&settings)).unwrap(),
                continuation_id: None,
                stream: false,
                rendered_items: items.clone(),
                tool_catalog: if round == 0 {
                    catalog.clone()
                } else {
                    ij(json!([]))
                },
            },
            if round == 0 {
                Some(ToolChoice::Required)
            } else {
                None
            },
        )
        .unwrap();
        std::fs::write(
            directory.join(format!("round-{round}.request.json")),
            &request.body,
        )
        .unwrap();
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body["generationConfig"]["thinkingConfig"]["thinkingBudget"],
            4096
        );
        if round == 1 {
            assert!(
                body["contents"].as_array().unwrap().contains(&sealed),
                "signature carrier changed during replay"
            );
        }
        capture.lock().unwrap().clear();
        let completion = runtime.send(
            provider::AdapterId::GoogleGeneration,
            &request,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );
        let raw = String::from_utf8_lossy(&capture.lock().unwrap()).replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("round-{round}.response.json")), &raw).unwrap();
        let terminal = match completion {
            Ok(provider::ProviderCompletion::Terminal(t)) => t,
            other => panic!(
                "provider failed: {}",
                format!("{other:?}").replace(&key, "[REDACTED]")
            ),
        };
        assert!(terminal.http_status.is_none_or(|s| (200..300).contains(&s)));
        let raw_value: Value = serde_json::from_str(&raw).unwrap();
        receipt["rounds"].as_array_mut().unwrap().push(json!({"round":round,"request_digest":request.request_digest,"tool_count":terminal.tool_calls.len(),"final_answer":terminal.is_final_answer(),"usage":raw_value["usageMetadata"]}));
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        if round == 0 {
            assert!(!terminal.is_final_answer());
            assert_eq!(terminal.tool_calls.len(), 1);
            let call = &terminal.tool_calls[0];
            assert_eq!(call.name, "lookup");
            sealed = terminal.sealed_fragments.clone();
            assert!(sealed["parts"].as_array().unwrap().iter().any(|p| {
                p["thoughtSignature"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty())
            }));
            assert!(
                raw_value["usageMetadata"]["thoughtsTokenCount"]
                    .as_u64()
                    .unwrap_or(0)
                    > 0
            );
            assert_eq!(
                terminal.usage.as_ref().unwrap().reasoning_tokens.as_deref(),
                Some(
                    raw_value["usageMetadata"]["thoughtsTokenCount"]
                        .to_string()
                        .as_str()
                )
            );
            items.push(ij(
                json!({"role":"sealed","adapter":"google_generation_v1","fragments":sealed}),
            ));
            items.push(ij(json!({"role":"tool","content":[{"type":"tool_result","call_id":call.call_id,"name":call.name,"result":"42"}]})));
            items.push(ij(json!({"role":"user","content":[{"type":"text","text":"Give the final numeric answer."}]})));
        } else {
            assert!(terminal.tool_calls.is_empty());
            assert!(terminal.is_final_answer());
            assert!(
                terminal
                    .content
                    .iter()
                    .any(|c| matches!(c,provider::ContentBlock::Text(t) if t.contains("42")))
            );
        }
    }
    receipt["status"] = json!("passed");
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "real Google credentials; scripts/run-live-google-schema.py"]
fn google_accepts_legacy_nullable_union_tool_schema() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let provider: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(provider.dialect, "google_generation_v1");
    assert_eq!(provider.endpoint_owner, "google");
    let model = provider.models.iter().find(|model| model.enabled).unwrap();
    let resolved = resolve_profile(&provider, model).unwrap();
    let ij = |v: Value| IJsonValue::parse(&serde_json_canonicalizer::to_vec(&v).unwrap()).unwrap();
    let prepared = prepare_with_tool_choice(&PrepareInput {
        attempt_id: "legacy-google-union".into(), target: resolved.target.clone(),
        endpoint: provider.endpoint.clone(),
        epoch_profile: epoch_profile(&resolved, "Answer directly and concisely.", None).unwrap(),
        continuation_id: None, stream: false,
        rendered_items: vec![ij(json!({"role":"user","content":[{"type":"text","text":"What is 21 * 2? Reply with just the number."}]}))],
        tool_catalog: ij(json!([{"name":"ask_user_questions","description":"Ask the user a preference question, optionally with preset options.","strict":true,
            "parameters":{"type":"object","properties":{"question":{"type":"string"},"options":{"type":["array","null"],"items":{"type":"string"}},"progress":{"type":"string"}},"required":["question","options","progress"],"additionalProperties":false}}])),
    }, Some(ToolChoice::Auto)).unwrap();
    std::fs::write(directory.join("request.json"), &prepared.body).unwrap();
    let body: Value = serde_json::from_slice(&prepared.body).unwrap();
    assert_eq!(
        body["tools"][0]["functionDeclarations"][0]["parametersJsonSchema"]["properties"]["options"]
            ["type"],
        json!(["array", "null"])
    );
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let result = runtime.send(
        provider::AdapterId::GoogleGeneration,
        &prepared,
        &key,
        &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
    );
    let bytes = capture.lock().unwrap().clone();
    let raw = String::from_utf8_lossy(&bytes).replace(&key, "[REDACTED]");
    std::fs::write(directory.join("response.json"), raw).unwrap();
    let passed = matches!(&result, Ok(provider::ProviderCompletion::Terminal(t))
        if t.http_status.is_none_or(|status| (200..300).contains(&status))
            && (t.is_final_answer() || !t.tool_calls.is_empty()));
    let summary = match &result {
        Ok(provider::ProviderCompletion::Terminal(t)) => {
            json!({"finish_reason":format!("{:?}",t.finish_reason),"tool_count":t.tool_calls.len(),"final_answer":t.is_final_answer(),"http_status":t.http_status})
        }
        other => json!({"error":format!("{other:?}").replace(&key,"[REDACTED]")}),
    };
    let receipt = json!({"status":if passed {"passed"} else {"failed"},"model":model.id,
        "url":prepared.url,"request_digest":prepared.request_digest,
        "scope":"Legacy nullable union declaration acceptance and nonempty response, through production prepare and HTTP normalization. Kernel uses parametersJsonSchema rather than legacy scalar parameters sanitizer.",
        "result":summary});
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    assert!(passed, "inspect retained receipt and response");
}
