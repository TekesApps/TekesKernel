use provider::{
    DeferredToolDefinition, DeferredToolReference, NativeDeferredTools, PrepareInput, ToolChoice,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
fn ij(value: Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap()
}

#[test]
#[ignore = "configured CF credentials; scripts/run-live-native-deferred.py"]
fn anthropic_custom_tool_reference_round_trip() {
    run_native_round_trip(false);
}

#[test]
#[ignore = "configured CF credentials; scripts/run-live-native-deferred.py --provider openai"]
fn openai_client_tool_search_round_trip() {
    run_native_round_trip(true);
}

fn run_native_round_trip(openai: bool) {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let config: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(
        config.dialect,
        if openai {
            "openai_responses_v1"
        } else {
            "anthropic_messages_v1"
        }
    );
    assert_eq!(config.endpoint_owner, "cloudflare");
    let model = config.models.iter().find(|m| m.enabled).unwrap();
    let resolved = provider::resolve_profile(&config, model).unwrap();
    let search = json!({"name":"tool_search","description":"Search for a deferred function required by the current task.","parameters":{"type":"object","properties":{"query":{"type":"string"},"limit":{"type":"integer"}},"required":["query"],"additionalProperties":false}});
    let loaded = ij(
        json!({"name":"get_shipping_eta","description":"Return the shipping ETA for an order identifier.","parameters":{"type":"object","properties":{"order_id":{"type":"string"}},"required":["order_id"],"additionalProperties":false}}),
    );
    let digest = format!(
        "sha256-{:x}",
        Sha256::digest(loaded.canonical_bytes().unwrap())
    );
    let mut native = NativeDeferredTools {
        catalog_revision: "live-catalog-v1".into(),
        search_tool_name: "tool_search".into(),
        definitions: vec![DeferredToolDefinition {
            schema: loaded.clone(),
            schema_digest: digest.clone(),
        }],
        references: vec![],
    };
    let mut input = PrepareInput {
        attempt_id: "native-search-1".into(),
        target: resolved.target.clone(),
        endpoint: config.endpoint.clone(),
        epoch_profile: provider::epoch_profile(
            &resolved,
            if openai {
                "Use the client tool search, then call the returned shipping ETA function."
            } else {
                "Search for the deferred shipping ETA tool, then use it. Do not answer in prose."
            },
            None,
        )
        .unwrap(),
        continuation_id: None,
        rendered_items: vec![ij(
            json!({"role":"user","content":[{"type":"text","text":if openai {"Find the deferred shipping ETA tool for order LIVE-42. Do not answer in prose."} else {"Find the deferred shipping ETA tool for order LIVE-42."}}]}),
        )],
        tool_catalog: ij(json!([search, loaded])),
        stream: false,
    };
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let capture = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let runtime = provider::HttpRuntime::new()
        .unwrap()
        .with_response_capture(capture.clone());
    let mut receipts = Vec::new();
    for round in 0..2 {
        input.attempt_id = format!("native-search-{round}");
        let request = provider::prepare_with_native_deferred_tools(
            &input,
            Some(if openai {
                ToolChoice::Auto
            } else {
                ToolChoice::Required
            }),
            &native,
        )
        .unwrap();
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        if openai {
            assert_eq!(body["tools"][0]["type"], "tool_search");
            assert_eq!(body["store"], false);
            assert!(body.get("previous_response_id").is_none());
            if round == 1 {
                assert!(
                    body["input"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|i| i["type"] == "tool_search_output")
                );
            }
        } else {
            assert_eq!(body["tools"][1]["defer_loading"], true);
            if round == 1 {
                assert!(
                    request
                        .body
                        .windows(b"tool_reference".len())
                        .any(|w| w == b"tool_reference")
                );
            }
        }
        std::fs::write(
            directory.join(format!("round-{round}.request.json")),
            &request.body,
        )
        .unwrap();
        capture.lock().unwrap().clear();
        let result = runtime.send(
            resolved.dialect.family(),
            &request,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );
        let raw = String::from_utf8_lossy(&capture.lock().unwrap()).replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("round-{round}.response.json")), raw).unwrap();
        let terminal = match result {
            Ok(provider::ProviderCompletion::Terminal(t)) => t,
            other => panic!("{}", format!("{other:?}").replace(&key, "[REDACTED]")),
        };
        assert!(
            terminal.http_status.is_none_or(|s| (200..300).contains(&s)),
            "provider rejected native request"
        );
        let call = terminal
            .tool_calls
            .first()
            .expect("native round must call a tool");
        assert_eq!(
            call.name,
            if round == 0 {
                "tool_search"
            } else {
                "get_shipping_eta"
            }
        );
        assert!(!call.call_id.is_empty());
        if round == 1 {
            assert_eq!(call.arguments["order_id"], "LIVE-42");
            assert!(terminal.usage.is_some());
        }
        receipts.push(json!({"round":round,"call":call,"usage":terminal.usage,"request_digest":request.request_digest}));
        if round == 0 {
            native.references.push(DeferredToolReference {
                tool_name: "get_shipping_eta".into(),
                schema_digest: digest.clone(),
                catalog_revision: "live-catalog-v1".into(),
                source_search_call_id: call.call_id.clone(),
            });
            input.rendered_items.push(ij(json!({"role":"sealed","adapter":config.dialect,"fragments":terminal.sealed_fragments})));
            input.rendered_items.push(ij(json!({"role":"tool","content":[{"type":"tool_result","call_id":call.call_id,"name":"tool_search","result":"Loaded get_shipping_eta."}]})));
        }
    }
    std::fs::write(directory.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":"passed","model":model.id,"rounds":receipts,"dialect":config.dialect,"scope":"adapter-direct native deferred tools through configured Cloudflare; worker native mode not yet wired"})).unwrap()).unwrap();
}
