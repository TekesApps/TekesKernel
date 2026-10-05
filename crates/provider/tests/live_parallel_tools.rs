use provider::{
    PrepareInput, ProviderCompletion, ProviderFrame, ToolChoice, epoch_profile,
    prepare_with_tool_choice, resolve_profile,
};
use schema::IJsonValue;
use serde_json::{Value, json};

#[test]
#[ignore = "CF credentials; scripts/run-live-native-deferred.py --provider openai --parallel-tools"]
fn openai_two_strict_tools_complete_through_sink() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let config: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(config.dialect, "openai_responses_v1");
    assert_eq!(config.endpoint_owner, "cloudflare");
    let model = config.models.iter().find(|m| m.enabled).unwrap();
    let resolved = resolve_profile(&config, model).unwrap();
    let settings = profile::SessionSettings {
        format: 1,
        revision: 1,
        provider: config.id.clone(),
        model: model.id.clone(),
        reasoning_effort: Some("low".into()),
    };
    let ij = |v: Value| IJsonValue::parse(&serde_json::to_vec(&v).unwrap()).unwrap();
    let tools = ["record_left", "record_right"].map(|name| json!({"name":name,"description":format!("Record the {} value.",if name=="record_left" {"LEFT"} else {"RIGHT"}),"strict":true,"parameters":{"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}}));
    let request = prepare_with_tool_choice(&PrepareInput {attempt_id:"parallel-live".into(),target:resolved.target.clone(),endpoint:config.endpoint.clone(),epoch_profile:epoch_profile(&resolved,"You are a tool-calling assistant. Call tools when asked; never answer in prose.",Some(&settings)).unwrap(),continuation_id:None,stream:true,rendered_items:vec![ij(json!({"role":"user","content":[{"type":"text","text":"In ONE response, make BOTH of these tool calls together, in parallel — do not wait for the first result before making the second:\n  • call `record_left` with value \"A\"\n  • call `record_right` with value \"B\""}]}))],tool_catalog:ij(json!(tools))},Some(ToolChoice::Required)).unwrap();
    let body: Value = serde_json::from_slice(&request.body).unwrap();
    assert_eq!(body["reasoning"]["effort"], "low");
    assert_eq!(body["tool_choice"], "required");
    assert!(body.get("temperature").is_none() && body.get("top_p").is_none());
    assert!(
        body["tools"]
            .as_array()
            .unwrap()
            .iter()
            .all(|t| t["strict"] == true)
    );
    std::fs::write(directory.join("request.json"), &request.body).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let mut ready = Vec::new();
    let completion = provider::HttpRuntime::new()
        .unwrap()
        .send_dialect_with_frames_and_wall_transport(
            resolved.dialect,
            &request,
            None,
            &key,
            &std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            Some(std::time::Duration::from_secs(120)),
            |frame| {
                if let ProviderFrame::ToolCallReady(call) = frame {
                    ready.push(call);
                }
                Ok(())
            },
        )
        .unwrap();
    let ProviderCompletion::Terminal(terminal) = completion else {
        panic!("provider did not complete")
    };
    std::fs::write(directory.join("response.sse"), &terminal.raw_response).unwrap();
    std::fs::write(directory.join("receipt.json"),serde_json::to_vec_pretty(&json!({"model":model.id,"attempt":"parallel-live","ready":ready,"terminal_calls":terminal.tool_calls,"request_digest":request.request_digest})).unwrap()).unwrap();
    assert_eq!(ready.len(), 2);
    assert_eq!(ready, terminal.tool_calls);
    for (name, value) in [("record_left", "A"), ("record_right", "B")] {
        let call = ready.iter().find(|c| c.name == name).unwrap();
        assert_eq!(call.arguments, json!({"value":value}));
    }
    assert_ne!(ready[0].call_id, ready[1].call_id);
}
