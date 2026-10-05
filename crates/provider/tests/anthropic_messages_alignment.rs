//! Anthropic Messages alignment: the thinking control, forced tool choice,
//! refusal fallbacks and output ceiling are read from the reviewed capability
//! catalog per exact SKU; the wire carries a cache breakpoint, strict tool
//! declarations, PDF documents, `pause_turn`, `stop_details` and cache writes.
use profile::{Model, Provider, SessionSettings};
use provider::{
    DialectId, FinishReason, PrepareInput, ToolChoice, epoch_profile, normalize_dialect_response,
    normalize_dialect_sse_stream, prepare_with_tool_choice, resolve_profile,
};
use schema::IJsonValue;
use serde_json::{Value, json};

fn ijson(value: &Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(value).expect("canonical")).expect("I-JSON")
}

fn route(
    dialect: &str,
    owner: &str,
    translation: &str,
    endpoint: &str,
    sku: &str,
) -> (Provider, Model) {
    let model = Model {
        id: sku.to_owned(),
        profile: format!("{dialect}:{sku}"),
        enabled: true,
        context_window_tokens: 200_000,
        compact_trigger_tokens: 180_000,
    };
    let provider = Provider {
        id: format!("{owner}-{dialect}"),
        name: None,
        adapter: "anthropic_messages".to_owned(),
        dialect: dialect.to_owned(),
        endpoint_owner: owner.to_owned(),
        gateway_translation: translation.to_owned(),
        evidence_revision: "legacy-example-v2".to_owned(),
        endpoint: endpoint.to_owned(),
        credential_key: Some("key".to_owned()),
        models: vec![model.clone()],
    };
    (provider, model)
}

fn anthropic(sku: &str) -> (Provider, Model) {
    route(
        "anthropic_messages_v1",
        "anthropic",
        "direct",
        "https://api.anthropic.com/v1",
        sku,
    )
}

fn deepseek_anthropic() -> (Provider, Model) {
    route(
        "deepseek_anthropic_v1",
        "deepseek",
        "direct",
        "https://api.deepseek.com/anthropic/v1",
        "deepseek-chat",
    )
}

#[derive(Debug)]
struct Prepared {
    body: Value,
    headers: std::collections::BTreeMap<String, String>,
}

fn prepare(
    (provider, model): &(Provider, Model),
    effort: Option<&str>,
    stream: bool,
    items: Vec<Value>,
    tools: Value,
    choice: Option<ToolChoice>,
) -> Result<Prepared, String> {
    let resolved = resolve_profile(provider, model).expect("configured route resolves");
    let settings = effort.map(|effort| SessionSettings {
        format: 1,
        revision: 1,
        provider: provider.id.clone(),
        model: model.id.clone(),
        reasoning_effort: Some(effort.to_owned()),
    });
    let epoch =
        epoch_profile(&resolved, "system", settings.as_ref()).map_err(|error| error.to_string())?;
    let request = prepare_with_tool_choice(
        &PrepareInput {
            attempt_id: "alignment".to_owned(),
            target: resolved.target.clone(),
            endpoint: provider.endpoint.clone(),
            epoch_profile: epoch,
            continuation_id: None,
            stream,
            rendered_items: items.iter().map(ijson).collect(),
            tool_catalog: ijson(&tools),
        },
        choice,
    )
    .map_err(|error| error.to_string())?;
    Ok(Prepared {
        body: serde_json::from_slice(&request.body).expect("request JSON"),
        headers: request.headers_without_secret,
    })
}

fn hello() -> Vec<Value> {
    vec![json!({"role":"user","content":[{"type":"text","text":"hello"}]})]
}

#[test]
fn preserved_thinking_replays_opaque_blocks_and_reports_transformations() {
    let blocks = json!([
        {"type":"thinking","thinking":"","signature":"opaque-original"},
        {"type":"redacted_thinking","data":"opaque-redacted"},
        {"type":"text","text":"checking"},
        {"type":"tool_use","id":"c1","name":"lookup","input":{"q":"x"}}
    ]);
    let changes = json!([{"type":"thinking_dropped","path":"messages.1.content.0","reason":"prefix_binding_mismatch"}]);
    let response = json!({"id":"m1","content":blocks,"stop_reason":"tool_use","input_transformations":changes});
    let terminal = normalize_dialect_response(
        DialectId::AnthropicMessagesV1,
        &serde_json::to_vec(&response).unwrap(),
    )
    .unwrap();
    assert_eq!(terminal.input_transformations, Some(changes.clone()));
    assert_eq!(terminal.sealed_fragments, blocks);
    let mut items = hello();
    items.push(json!({"role":"sealed","adapter":"anthropic_messages_v1","fragments":terminal.sealed_fragments}));
    items.push(
        json!({"role":"tool","content":[{"type":"tool_result","call_id":"c1","result":"42"}]}),
    );
    let request = prepare(
        &anthropic("claude-fable-5-1"),
        None,
        true,
        items,
        strict_tool(),
        None,
    )
    .unwrap();
    assert_eq!(request.body["messages"][1]["content"], blocks);
    assert!(request.body.get("input_transformations").is_none());
    let events = [
        json!({"type":"message_start","message":{"id":"m2","input_transformations":changes}}),
        json!({"type":"content_block_start","index":0,"content_block":{"type":"thinking","thinking":"","signature":""}}),
        json!({"type":"content_block_delta","index":0,"delta":{"type":"signature_delta","signature":"opaque-original"}}),
        json!({"type":"content_block_stop","index":0}),
        json!({"type":"content_block_start","index":1,"content_block":blocks[1]}),
        json!({"type":"content_block_stop","index":1}),
        json!({"type":"message_delta","delta":{"stop_reason":"end_turn"}}),
        json!({"type":"message_stop"}),
    ];
    let sse = events.iter().fold(String::new(), |mut sse, v| {
        sse.push_str(&format!("data: {v}\n\n"));
        sse
    });
    let (_, streamed) =
        normalize_dialect_sse_stream(DialectId::AnthropicMessagesV1, sse.as_bytes()).unwrap();
    assert_eq!(streamed.input_transformations, Some(changes));
    assert_eq!(streamed.sealed_fragments, json!([blocks[0], blocks[1]]));
    // A fallback may replace the initial report at message_delta, including
    // unknown future entries; preserve it without changing replay content.
    let fallback = sse.replace("\"stop_reason\":\"end_turn\"", "\"stop_reason\":\"end_turn\",\"input_transformations\":[{\"type\":\"future\",\"reason\":\"future\"}]");
    let (_, streamed) =
        normalize_dialect_sse_stream(DialectId::AnthropicMessagesV1, fallback.as_bytes()).unwrap();
    assert_eq!(
        streamed.input_transformations,
        Some(json!([{"type":"future","reason":"future"}]))
    );
}

fn strict_tool() -> Value {
    json!([{"name":"lookup","description":"Look something up.","parameters":{
        "type":"object","properties":{"q":{"type":"string"}},"required":["q"],"additionalProperties":false}}])
}

fn loose_tool() -> Value {
    json!([{"name":"lookup","description":"Look something up.","parameters":{
        "type":"object","properties":{"q":{"type":"string"}},"required":["q"]}}])
}

#[test]
fn fable_5_1_sends_adaptive_display_effort_fallbacks_and_never_forces_a_tool() {
    let target = anthropic("claude-fable-5-1");
    let prepared = prepare(
        &target,
        Some("xhigh"),
        true,
        hello(),
        strict_tool(),
        Some(ToolChoice::Required),
    )
    .expect("Fable 5.1 accepts xhigh");
    let body = &prepared.body;
    assert_eq!(
        body["thinking"],
        json!({"display":"summarized","type":"adaptive","block_binding":{"prefix_mismatch_behavior":"drop_block"}})
    );
    assert_eq!(body["output_config"], json!({"effort":"xhigh"}));
    assert!(
        body.pointer("/thinking/budget_tokens").is_none(),
        "budget_tokens is a 400 on this generation"
    );
    assert_eq!(body["fallbacks"], json!("default"));
    assert_eq!(
        prepared.headers.get("anthropic-beta").map(String::as_str),
        Some("server-side-fallback-2026-07-01,thinking-binding-controls-2026-08-01")
    );
    assert_eq!(
        body["tool_choice"],
        json!({"type":"auto"}),
        "forced tool use is rejected on Fable 5.1"
    );
    assert_eq!(body["tools"][0]["strict"], json!(true));
    assert!(
        body.get("cache_control").is_none(),
        "no root-level breakpoint"
    );
    assert_eq!(body["max_tokens"], json!(64_000));

    let none = prepare(
        &target,
        None,
        true,
        hello(),
        json!([]),
        Some(ToolChoice::None),
    )
    .expect("none choice");
    assert_eq!(
        none.body["tool_choice"],
        json!({"type":"none"}),
        "`none` is unaffected"
    );
    assert_eq!(
        none.body["thinking"],
        json!({"display":"summarized","type":"adaptive","block_binding":{"prefix_mismatch_behavior":"drop_block"}}),
        "display rides without effort"
    );
    assert!(none.body.get("output_config").is_none());
}

#[test]
fn opus_5_keeps_forced_tool_choice_with_fallbacks_and_all_five_efforts() {
    let target = anthropic("claude-opus-5");
    for effort in ["low", "medium", "high", "xhigh", "max"] {
        let prepared = prepare(
            &target,
            Some(effort),
            false,
            hello(),
            strict_tool(),
            Some(ToolChoice::Required),
        )
        .unwrap_or_else(|error| panic!("{effort}: {error}"));
        assert_eq!(prepared.body["output_config"]["effort"], json!(effort));
        assert_eq!(prepared.body["thinking"]["type"], json!("adaptive"));
        assert_eq!(prepared.body["tool_choice"], json!({"type":"any"}));
        assert_eq!(prepared.body["fallbacks"], json!("default"));
        assert_eq!(
            prepared.body["max_tokens"],
            json!(16_000),
            "non-streaming ceiling"
        );
    }
    assert!(
        prepare(&target, Some("ultra"), false, hello(), json!([]), None).is_err(),
        "unknown effort fails closed"
    );
}

#[test]
fn fable_5_has_no_fallbacks_and_still_forces_tools() {
    let target = anthropic("claude-fable-5");
    let prepared = prepare(
        &target,
        Some("max"),
        true,
        hello(),
        strict_tool(),
        Some(ToolChoice::Required),
    )
    .expect("max");
    assert!(prepared.body.get("fallbacks").is_none());
    assert_eq!(
        prepared.headers.get("anthropic-beta").map(String::as_str),
        Some("thinking-binding-controls-2026-08-01")
    );
    assert_eq!(prepared.body["tool_choice"], json!({"type":"any"}));
    assert_eq!(
        prepared.body["thinking"],
        json!({"display":"summarized","type":"adaptive","block_binding":{"prefix_mismatch_behavior":"drop_block"}})
    );
}

#[test]
fn generations_without_a_thinking_wire_send_no_thinking_but_keep_the_breakpoints() {
    let target = anthropic("claude-sonnet-4-20250514");
    let streamed = prepare(&target, None, true, hello(), loose_tool(), None).expect("sonnet 4");
    assert!(streamed.body.get("thinking").is_none());
    assert!(streamed.body.get("output_config").is_none());
    assert!(streamed.body.get("fallbacks").is_none());
    assert!(streamed.body.get("cache_control").is_none());
    assert_eq!(
        streamed.body["tools"][0]["cache_control"],
        json!({"type":"ephemeral"})
    );
    assert_eq!(
        streamed.body["messages"][0]["content"][0]["cache_control"],
        json!({"type":"ephemeral"})
    );
    assert_eq!(streamed.body["max_tokens"], json!(64_000));
    assert!(
        streamed.body["tools"][0].get("strict").is_none(),
        "open schema is never declared strict"
    );
    let buffered =
        prepare(&target, None, false, hello(), json!([]), None).expect("sonnet 4 buffered");
    assert_eq!(buffered.body["max_tokens"], json!(16_000));
    assert!(
        prepare(&target, Some("high"), true, hello(), json!([]), None).is_err(),
        "no levels, no effort"
    );
}

/// The breakpoints ride the last tool and the last content block of the last
/// message and move forward with the conversation: the server reads every
/// block boundary before the marker, so request N+1 reads what request N
/// wrote. A root-level marker would key the cache on the whole body instead.
#[test]
fn breakpoints_move_with_the_last_message_and_never_sit_on_the_root() {
    let target = anthropic("claude-sonnet-4-20250514");
    let first = prepare(&target, None, true, hello(), loose_tool(), None).expect("turn 1");
    assert!(first.body.get("cache_control").is_none());
    assert_eq!(
        first.body["tools"][0]["cache_control"],
        json!({"type":"ephemeral"})
    );
    assert_eq!(
        first.body["messages"][0]["content"][0]["cache_control"],
        json!({"type":"ephemeral"})
    );

    let mut items = hello();
    items.push(json!({"role":"assistant","content":[{"type":"text","text":"world"}]}));
    items.push(json!({"role":"user","content":[{"type":"text","text":"again"},{"type":"text","text":"and again"}]}));
    let next = prepare(&target, None, true, items, loose_tool(), None).expect("turn 2");
    let messages = next.body["messages"].as_array().expect("messages");
    assert_eq!(messages.len(), 3);
    assert!(
        messages[0]["content"][0].get("cache_control").is_none(),
        "the marker moved off turn 1"
    );
    assert!(
        messages[2]["content"][0].get("cache_control").is_none(),
        "only the last block carries it"
    );
    assert_eq!(
        messages[2]["content"][1]["cache_control"],
        json!({"type":"ephemeral"})
    );
    // Everything before the marker in `next` is byte-identical to `first`
    // minus its own marker: the server finds turn 1's write at that boundary.
    let mut first_block = first.body["messages"][0]["content"][0].clone();
    first_block.as_object_mut().unwrap().remove("cache_control");
    assert_eq!(messages[0]["content"][0], first_block);

    let bare = prepare(&target, None, true, hello(), json!([]), None).expect("no tools");
    assert!(bare.body.get("cache_control").is_none());
    assert_eq!(
        bare.body["messages"][0]["content"][0]["cache_control"],
        json!({"type":"ephemeral"})
    );
}

#[test]
fn deepseek_anthropic_facade_keeps_its_legacy_shape() {
    let target = deepseek_anthropic();
    let prepared = prepare(
        &target,
        None,
        true,
        hello(),
        strict_tool(),
        Some(ToolChoice::Required),
    )
    .expect("deepseek");
    assert!(prepared.body.get("cache_control").is_none());
    assert!(prepared.body.get("fallbacks").is_none());
    assert!(prepared.body.get("thinking").is_none());
    assert_eq!(prepared.body["max_tokens"], json!(8192));
    assert!(
        prepared.body["tools"][0].get("strict").is_none(),
        "strict is an Anthropic-only field"
    );
    assert_eq!(prepared.body["tool_choice"], json!({"type":"any"}));
}

#[test]
fn pdf_files_become_document_blocks_and_other_mimes_degrade_to_text() {
    let pdf = vec![json!({"role":"user","content":[
        {"type":"file","mime":"application/pdf","name":"x.pdf","data":"AA=="},
        {"type":"text","text":"summarize"}]})];
    let prepared = prepare(
        &anthropic("claude-opus-5"),
        None,
        true,
        pdf.clone(),
        json!([]),
        None,
    )
    .expect("pdf accepted");
    assert_eq!(
        prepared.body["messages"][0]["content"][0],
        json!({"source":{"data":"AA==","media_type":"application/pdf","type":"base64"},"type":"document"})
    );
    let image =
        vec![json!({"role":"user","content":[{"type":"file","mime":"image/png","data":"AA=="}]})];
    let prepared = prepare(
        &anthropic("claude-opus-5"),
        None,
        true,
        image,
        json!([]),
        None,
    )
    .expect("image accepted");
    assert_eq!(
        prepared.body["messages"][0]["content"][0]["type"],
        json!("image")
    );
    let text = vec![
        json!({"role":"user","content":[{"type":"file","mime":"text/plain","name":"a.txt","data":"aGk=","bytes":2}]}),
    ];
    let prepared = prepare(
        &anthropic("claude-opus-5"),
        None,
        true,
        text,
        json!([]),
        None,
    )
    .expect("text/plain degrades");
    assert_eq!(prepared.body["messages"][0]["content"][0]["type"], "text");
    assert_eq!(
        prepared.body["messages"][0]["content"][0]["text"],
        "Attached file a.txt (text/plain, 2 bytes):\n```\nhi\n```"
    );
    let prepared = prepare(&deepseek_anthropic(), None, true, pdf, json!([]), None)
        .expect("facade degrades files");
    assert_eq!(prepared.body["messages"][0]["content"][0]["type"], "text");
    assert_eq!(
        prepared.body["messages"][0]["content"][0]["text"],
        "Attached file x.pdf (application/pdf, 1 bytes):\n```\n\u{0}\n```"
    );
    let image =
        vec![json!({"role":"user","content":[{"type":"file","mime":"image/png","data":"AA=="}]})];
    let error = prepare(&deepseek_anthropic(), None, true, image, json!([]), None)
        .expect_err("image files are not degraded");
    assert!(
        error.contains("rejects the requested file block"),
        "{error}"
    );
}

#[test]
fn pause_turn_is_a_continuation_not_a_final_answer() {
    let response = serde_json_canonicalizer::to_vec(&json!({
        "content":[{"text":"partial","type":"text"}],"id":"m","stop_reason":"pause_turn",
        "usage":{"input_tokens":3,"output_tokens":1}
    }))
    .unwrap();
    let terminal = normalize_dialect_response(DialectId::AnthropicMessagesV1, &response)
        .expect("pause_turn decodes");
    assert_eq!(terminal.finish_reason, FinishReason::Paused);
    assert!(
        !terminal.is_final_answer(),
        "a paused turn must be resent, not settled"
    );
    assert!(terminal.stop_details.is_none());

    let stream = concat!(
        "data: {\"message\":{\"id\":\"msg_1\",\"usage\":{\"input_tokens\":2}},\"type\":\"message_start\"}\n\n",
        "data: {\"content_block\":{\"text\":\"\",\"type\":\"text\"},\"index\":0,\"type\":\"content_block_start\"}\n\n",
        "data: {\"delta\":{\"text\":\"partial\",\"type\":\"text_delta\"},\"index\":0,\"type\":\"content_block_delta\"}\n\n",
        "data: {\"index\":0,\"type\":\"content_block_stop\"}\n\n",
        "data: {\"delta\":{\"stop_reason\":\"pause_turn\"},\"type\":\"message_delta\",\"usage\":{\"output_tokens\":1}}\n\n",
        "data: {\"type\":\"message_stop\"}\n\n",
    );
    let (_, terminal) =
        normalize_dialect_sse_stream(DialectId::AnthropicMessagesV1, stream.as_bytes())
            .expect("stream decodes");
    assert_eq!(terminal.finish_reason, FinishReason::Paused);
    assert_eq!(
        terminal.sealed_fragments,
        json!([{"text":"partial","type":"text"}]),
        "paused output replays verbatim"
    );
}

#[test]
fn refusal_keeps_stop_details_beside_the_content_filter_verdict() {
    let response = serde_json_canonicalizer::to_vec(&json!({
        "content":[],"id":"m","stop_reason":"refusal",
        "stop_details":{"category":"cyber","explanation":"declined","type":"refusal"},
        "usage":{"input_tokens":3,"output_tokens":0}
    }))
    .unwrap();
    let terminal = normalize_dialect_response(DialectId::AnthropicMessagesV1, &response)
        .expect("refusal decodes");
    assert_eq!(terminal.finish_reason, FinishReason::ContentFilter);
    assert_eq!(
        terminal
            .stop_details
            .as_ref()
            .and_then(|d| d.get("category")),
        Some(&json!("cyber"))
    );

    let stream = concat!(
        "data: {\"message\":{\"id\":\"msg_1\",\"usage\":{\"input_tokens\":2}},\"type\":\"message_start\"}\n\n",
        "data: {\"delta\":{\"stop_reason\":\"refusal\",\"stop_details\":{\"category\":\"bio\",\"type\":\"refusal\"}},\"type\":\"message_delta\",\"usage\":{\"output_tokens\":0}}\n\n",
        "data: {\"type\":\"message_stop\"}\n\n",
    );
    let (_, terminal) =
        normalize_dialect_sse_stream(DialectId::AnthropicMessagesV1, stream.as_bytes())
            .expect("stream decodes");
    assert_eq!(terminal.finish_reason, FinishReason::ContentFilter);
    assert_eq!(
        terminal.stop_details,
        Some(json!({"category":"bio","type":"refusal"}))
    );

    let plain = serde_json_canonicalizer::to_vec(&json!({
        "content":[{"text":"ok","type":"text"}],"id":"m","stop_reason":"end_turn","stop_details":null
    })).unwrap();
    let terminal = normalize_dialect_response(DialectId::AnthropicMessagesV1, &plain)
        .expect("end_turn decodes");
    assert!(
        terminal.stop_details.is_none(),
        "null stop_details is absence"
    );
    let serialized = serde_json::to_value(&terminal).unwrap();
    assert!(
        serialized.get("stop_details").is_none(),
        "absent details do not widen the terminal record"
    );
}

#[test]
fn cache_writes_are_reported_beside_cache_reads() {
    let response = serde_json_canonicalizer::to_vec(&json!({
        "content":[{"text":"ok","type":"text"}],"id":"m","stop_reason":"end_turn",
        "usage":{"input_tokens":10,"output_tokens":1,"cache_read_input_tokens":700,"cache_creation_input_tokens":300}
    })).unwrap();
    let terminal =
        normalize_dialect_response(DialectId::AnthropicMessagesV1, &response).expect("decodes");
    let usage = terminal.usage.expect("usage");
    assert_eq!(usage.cache_read.as_deref(), Some("700"));
    assert_eq!(usage.cache_write.as_deref(), Some("300"));
    assert_eq!(usage.input_tokens.as_deref(), Some("10"));

    // Live Fable 5 shape (2026-09-06): thinking spend sits under output_tokens_details.
    let response = serde_json_canonicalizer::to_vec(&json!({
        "content":[{"text":"ok","type":"text"}],"id":"m","stop_reason":"end_turn",
        "usage":{"input_tokens":501,"output_tokens":15,"output_tokens_details":{"thinking_tokens":12},
                 "cache_creation_input_tokens":0,"cache_read_input_tokens":0}
    })).unwrap();
    let usage = normalize_dialect_response(DialectId::AnthropicMessagesV1, &response)
        .expect("decodes")
        .usage
        .expect("usage");
    assert_eq!(usage.reasoning_tokens.as_deref(), Some("12"));
    assert_eq!(usage.cache_write.as_deref(), Some("0"));

    let stream = concat!(
        "data: {\"message\":{\"id\":\"msg_1\",\"usage\":{\"input_tokens\":2,\"cache_creation_input_tokens\":40,\"cache_read_input_tokens\":0}},\"type\":\"message_start\"}\n\n",
        "data: {\"content_block\":{\"text\":\"\",\"type\":\"text\"},\"index\":0,\"type\":\"content_block_start\"}\n\n",
        "data: {\"delta\":{\"text\":\"ok\",\"type\":\"text_delta\"},\"index\":0,\"type\":\"content_block_delta\"}\n\n",
        "data: {\"index\":0,\"type\":\"content_block_stop\"}\n\n",
        "data: {\"delta\":{\"stop_reason\":\"end_turn\"},\"type\":\"message_delta\",\"usage\":{\"output_tokens\":1}}\n\n",
        "data: {\"type\":\"message_stop\"}\n\n",
    );
    let (_, terminal) =
        normalize_dialect_sse_stream(DialectId::AnthropicMessagesV1, stream.as_bytes())
            .expect("stream decodes");
    let usage = terminal.usage.expect("usage");
    assert_eq!(
        usage.cache_write.as_deref(),
        Some("40"),
        "message_start figures survive the message_delta merge"
    );
    assert_eq!(usage.output_tokens.as_deref(), Some("1"));
    let sonnet = serde_json_canonicalizer::to_vec(&json!({
        "content":[{"text":"ok","type":"text"}],"id":"m","stop_reason":"end_turn","usage":{"input_tokens":1,"output_tokens":1}
    })).unwrap();
    let terminal =
        normalize_dialect_response(DialectId::AnthropicMessagesV1, &sonnet).expect("decodes");
    assert!(
        serde_json::to_value(terminal.usage.unwrap())
            .unwrap()
            .get("cache_write")
            .is_none(),
        "unreported writes stay absent"
    );
}

#[test]
fn unknown_stop_reasons_still_fail_closed() {
    let response = serde_json_canonicalizer::to_vec(&json!({
        "content":[],"id":"m","stop_reason":"model_context_window_exceeded_v9"
    }))
    .unwrap();
    assert!(normalize_dialect_response(DialectId::AnthropicMessagesV1, &response).is_err());
}
