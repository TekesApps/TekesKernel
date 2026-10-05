use provider::{
    PrepareInput, ToolChoice, epoch_profile, prepare_with_tool_choice, resolve_profile,
};
use schema::IJsonValue;
use serde_json::{Value, json};

#[test]
#[ignore = "CF credentials; scripts/run-live-anthropic-thinking.py --binding"]
fn cloudflare_preserved_thinking_prefix_binding() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let provider: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    let model = provider.models.iter().find(|m| m.enabled).unwrap();
    assert_eq!(model.id, "claude-fable-5-1");
    let resolved = resolve_profile(&provider, model).unwrap();
    let ij = |v: Value| IJsonValue::parse(&serde_json::to_vec(&v).unwrap()).unwrap();
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let system = format!(
        "Probe {nonce}. Answer arithmetic questions briefly. Reference background: {}",
        "A stable reference paragraph for a prefix caching compatibility experiment. ".repeat(180)
    );
    let mut items = vec![ij(
        json!({"role":"user","content":[{"type":"text","text":"Calculate the greatest common divisor of 1071 and 462. Give only the result."}]}),
    )];
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let runtime = provider::HttpRuntime::new().unwrap();
    let mut receipt = json!({"status":"running","model":model.id,"rounds":[]});
    for round in 0..3 {
        let head = if round == 2 {
            format!("{system}\nAnswer in English.")
        } else {
            system.clone()
        };
        let mut request = provider::prepare(&PrepareInput {
            attempt_id: format!("binding-{round}"),
            target: resolved.target.clone(),
            endpoint: provider.endpoint.clone(),
            epoch_profile: epoch_profile(&resolved, &head, None).unwrap(),
            continuation_id: None,
            stream: true,
            rendered_items: items.clone(),
            tool_catalog: ij(json!([])),
        })
        .unwrap();
        // Avoid mistaking a gateway response cache for an upstream prompt hit.
        request
            .headers_without_secret
            .insert("cf-aig-skip-cache".into(), "true".into());
        std::fs::write(
            directory.join(format!("binding-{round}.request.json")),
            &request.body,
        )
        .unwrap();
        let result = runtime.send(
            provider::AdapterId::Anthropic,
            &request,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );
        let terminal = match result {
            Ok(provider::ProviderCompletion::Terminal(t)) => t,
            other => panic!("{}", format!("{other:?}").replace(&key, "[REDACTED]")),
        };
        std::fs::write(
            directory.join(format!("binding-{round}.response.sse")),
            &terminal.raw_response,
        )
        .unwrap();
        receipt["rounds"].as_array_mut().unwrap().push(json!({"round":round,"finish_reason":terminal.finish_reason,"usage":terminal.usage,"input_transformations":terminal.input_transformations}));
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        assert!(terminal.is_final_answer(), "probe must finish successfully");
        let changes = terminal
            .input_transformations
            .as_ref()
            .and_then(Value::as_array)
            .expect("beta telemetry must survive the gateway");
        if round < 2 {
            assert!(
                changes.is_empty(),
                "an append-only prefix must keep its thinking"
            );
        } else {
            assert!(
                changes
                    .iter()
                    .any(|v| v["reason"] == "prefix_binding_mismatch"),
                "edited system must exercise the drop policy"
            );
        }
        assert!(
            terminal
                .sealed_fragments
                .as_array()
                .unwrap()
                .iter()
                .any(|b| b["type"] == "thinking"
                    && b["signature"].as_str().is_some_and(|s| !s.is_empty())),
            "must exercise real signed thinking"
        );
        items.push(ij(json!({"role":"sealed","adapter":"anthropic_messages_v1","fragments":terminal.sealed_fragments})));
        items.push(ij(json!({"role":"user","content":[{"type":"text","text":"Now multiply that result by two. Give only the result."}]})));
    }
    receipt["status"] = json!("passed");
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "CF credentials; scripts/run-live-anthropic-thinking.py"]
fn cloudflare_generation_thinking_and_optional_tool_replay() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let provider: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(provider.dialect, "anthropic_messages_v1");
    assert_eq!(provider.endpoint_owner, "cloudflare");
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
    let catalog = ij(
        json!([{"name":"lookup","description":"Look up the answer to a question.","strict":true,"parameters":{"type":"object","properties":{"question":{"type":"string"}},"required":["question"],"additionalProperties":false}}]),
    );
    let mut items = vec![ij(
        json!({"role":"user","content":[{"type":"text","text":"What is 21 * 2? Use the lookup tool."}]}),
    )];
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let mut receipt = json!({"status":"running","model":model.id,"rounds":[],"scope":"Generation-specific high reasoning through native CF Anthropic, required lookup then auto with sealed replay and retained tool declaration. Nonempty final or additional tool call matches legacy acceptance."});
    let mut previous = Value::Null;
    for round in 0..2 {
        let request = prepare_with_tool_choice(
            &PrepareInput {
                attempt_id: format!("anthropic-thinking-{round}"),
                target: resolved.target.clone(),
                endpoint: provider.endpoint.clone(),
                epoch_profile: epoch_profile(
                    &resolved,
                    "You answer via the lookup tool, then report its result verbatim.",
                    Some(&settings),
                )
                .unwrap(),
                continuation_id: None,
                stream: false,
                rendered_items: items.clone(),
                tool_catalog: catalog.clone(),
            },
            Some(if round == 0 {
                ToolChoice::Required
            } else {
                ToolChoice::Auto
            }),
        )
        .unwrap();
        assert_eq!(request.credential_header, "cf-aig-authorization");
        std::fs::write(
            directory.join(format!("round-{round}.request.json")),
            &request.body,
        )
        .unwrap();
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        if model.id.contains("fable") || model.id.contains("mythos") || model.id == "claude-opus-5"
        {
            assert_eq!(
                body["thinking"],
                json!({"display":"summarized","type":"adaptive","block_binding":{"prefix_mismatch_behavior":"drop_block"}})
            );
            assert_eq!(body["output_config"]["effort"], "high");
            assert!(body.get("cache_control").is_none());
            assert_eq!(
                body["messages"].as_array().unwrap().last().unwrap()["content"]
                    .as_array()
                    .unwrap()
                    .last()
                    .unwrap()["cache_control"],
                json!({"type":"ephemeral"})
            );
        } else {
            panic!("unreviewed generation; establish the legacy shape before running");
        }
        let forced = !model.id.starts_with("claude-fable-5-1");
        assert_eq!(
            body["tool_choice"]["type"],
            if round == 0 && forced { "any" } else { "auto" }
        );
        if round == 1 {
            assert!(
                body["messages"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|m| m["content"] == previous)
            );
        }
        capture.lock().unwrap().clear();
        let result = runtime.send(
            provider::AdapterId::Anthropic,
            &request,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );
        let raw = String::from_utf8_lossy(&capture.lock().unwrap()).replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("round-{round}.response.json")), &raw).unwrap();
        let terminal = match result {
            Ok(provider::ProviderCompletion::Terminal(t)) => t,
            other => panic!("{}", format!("{other:?}").replace(&key, "[REDACTED]")),
        };
        let accepted = terminal.http_status.is_none_or(|s| (200..300).contains(&s))
            && (terminal.is_final_answer() || !terminal.tool_calls.is_empty());
        receipt["rounds"].as_array_mut().unwrap().push(json!({"round":round,"request_digest":request.request_digest,"accepted":accepted,"finish_reason":terminal.finish_reason,"http_status":terminal.http_status,"tool_count":terminal.tool_calls.len()}));
        receipt["status"] = json!(if accepted { "running" } else { "failed" });
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        assert!(accepted, "provider response rejected; inspect receipt");
        if round == 1 || terminal.tool_calls.is_empty() {
            break;
        }
        let call = &terminal.tool_calls[0];
        assert_eq!(call.name, "lookup");
        previous = terminal.sealed_fragments.clone();
        items.push(ij(
            json!({"role":"sealed","adapter":"anthropic_messages_v1","fragments":previous}),
        ));
        items.push(ij(json!({"role":"tool","content":[{"type":"tool_result","call_id":call.call_id,"name":call.name,"result":"42"}]})));
        items.push(ij(json!({"role":"user","content":[{"type":"text","text":"Report the lookup result you already received. Do not call any more tools."}]})));
    }
    receipt["status"] = json!("passed");
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}
