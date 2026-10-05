use provider::{AdapterId, normalize_response, normalize_sse_stream};
use serde_json::{Value, json};

#[test]
fn ollama_usage_only_frame_after_finish_is_preserved() {
    let wire = concat!(
        "data: {\"id\":\"local-1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"done\"},\"finish_reason\":null}]}\n\n",
        "data: {\"id\":\"local-1\",\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}]}\n\n",
        "data: {\"id\":\"local-1\",\"choices\":[],\"usage\":{\"prompt_tokens\":37,\"completion_tokens\":2,\"total_tokens\":39}}\n\n",
        "data: [DONE]\n\n"
    );
    let (_, terminal) =
        provider::normalize_dialect_sse_stream(provider::DialectId::OllamaChatV1, wire.as_bytes())
            .unwrap();
    assert!(terminal.is_final_answer());
    let usage = terminal
        .usage
        .expect("trailing usage must survive final text");
    assert_eq!(usage.input_tokens.as_deref(), Some("37"));
    assert_eq!(usage.output_tokens.as_deref(), Some("2"));
    assert_eq!(usage.cache_read, None, "unreported cache use is not zero");
}

fn response(items: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({"id":"r1","status":"completed","output":items})).unwrap()
}

#[test]
fn response_completion_requires_final_answer_evidence() {
    for (items, expected) in [
        (json!([]), false),
        (
            json!([{"type":"reasoning","summary":[{"type":"summary_text","text":"thinking"}]}]),
            false,
        ),
        (
            json!([{"type":"message","phase":"commentary","status":"completed","content":[{"type":"output_text","text":"I will continue"}]}]),
            false,
        ),
        (
            json!([{"type":"message","phase":"final_answer","status":"in_progress","content":[{"type":"output_text","text":"partial"}]}]),
            false,
        ),
        (
            json!([{"type":"message","phase":"final_answer","status":"completed","content":[{"type":"output_text","text":"done"}]}]),
            true,
        ),
        (
            json!([{"type":"message","content":[{"type":"output_text","text":"done"}]}]),
            true,
        ),
        (
            json!([{"type":"message","content":[{"type":"output_text","text":"   "}]}]),
            false,
        ),
        (
            json!([{"type":"message","phase":"final_answer","status":"completed","content":[{"type":"output_text","text":"done"}]},{"type":"function_call","call_id":"c1","name":"think","arguments":"{}"}]),
            false,
        ),
    ] {
        let terminal = normalize_response(AdapterId::Responses, &response(items.clone())).unwrap();
        assert_eq!(terminal.is_final_answer(), expected, "{items}");
    }
}

#[test]
fn legacy_explicit_nonfinal_text_is_not_promoted() {
    for finality in [false, true] {
        let bytes = serde_json::to_vec(
            &json!({"text":"message", "isFinalAnswer":finality, "functionCalls":[]}),
        )
        .unwrap();
        assert_eq!(
            normalize_response(AdapterId::Responses, &bytes)
                .unwrap()
                .is_final_answer(),
            finality
        );
    }
}

#[test]
fn anthropic_refusal_is_a_terminal_provider_filter_not_malformed_or_final() {
    let bytes =
        serde_json::to_vec(&json!({"id":"refused","content":[],"stop_reason":"refusal"})).unwrap();
    let terminal = normalize_response(AdapterId::Anthropic, &bytes).unwrap();
    assert_eq!(
        terminal.finish_reason,
        provider::FinishReason::ContentFilter
    );
    assert!(!terminal.is_final_answer());
}

#[test]
fn streamed_commentary_and_final_phase_remain_distinct() {
    for phase in ["commentary", "final_answer"] {
        let item = json!({"id":"m1","type":"message","phase":phase,"status":"completed","content":[{"type":"output_text","text":"message"}]});
        let events = [
            json!({"type":"response.output_item.added","output_index":0,"item":item}),
            json!({"type":"response.output_text.delta","output_index":0,"delta":"message"}),
            json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[item]}}),
        ];
        let wire = events.iter().fold(String::new(), |mut acc, event| {
            acc.push_str(&format!("data: {event}\n\n"));
            acc
        });
        let (_, terminal) = normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).unwrap();
        assert_eq!(terminal.is_final_answer(), phase == "final_answer");
    }
}

#[test]
fn responses_arguments_done_reuses_identity_from_added_frame() {
    let events = [
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","call_id":"c1","name":"think","arguments":""}}),
        json!({"type":"response.function_call_arguments.delta","output_index":0,"delta":""}),
        json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":"{\"thought\":\"check\"}"}),
        json!({"type":"response.completed","response":{"id":"r1","status":"completed"}}),
    ];
    let wire = events.iter().fold(String::new(), |mut acc, event| {
        acc.push_str(&format!("data: {event}\n\n"));
        acc
    });
    let (frames, terminal) = normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).unwrap();
    assert_eq!(frames.iter().filter(|frame| matches!(frame, provider::ProviderFrame::ToolDelta { delta, .. } if !delta.is_empty())).count(), 1);
    assert!(frames.iter().any(|frame| matches!(frame, provider::ProviderFrame::ToolDelta { call_id, name, delta }
        if call_id == "c1" && name.as_deref() == Some("think") && delta == "{\"thought\":\"check\"}")));
    assert_eq!(
        frames
            .iter()
            .filter(|f| matches!(f, provider::ProviderFrame::ToolCallReady(_)))
            .count(),
        1
    );
    assert_eq!(terminal.tool_calls[0].name, "think");
    assert_eq!(terminal.tool_calls[0].arguments, json!({"thought":"check"}));
    assert!(!terminal.is_final_answer());
}

#[test]
fn terminal_only_stream_retains_final_answer() {
    let event = json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[{"type":"message","phase":"final_answer","status":"completed","content":[{"type":"output_text","text":"done"}]}]}});
    let wire = format!("data: {event}\n\n");
    let (_, terminal) = normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).unwrap();
    assert!(terminal.is_final_answer());
}

#[test]
fn completion_markers_in_commentary_do_not_supply_finality() {
    for text in [
        "SKILL_POST_COMPACT_OK",
        "Final Answer",
        "final_answer=true",
        "重试",
    ] {
        let item = json!({"type":"message","phase":"commentary","status":"completed","content":[{"type":"output_text","text":text}]});
        let wire = response(json!([item]));
        let direct = normalize_response(AdapterId::Responses, &wire).unwrap();
        assert!(!direct.is_final_answer(), "body marker promoted: {text}");
        let event = json!({"type":"response.completed","response":serde_json::from_slice::<Value>(&wire).unwrap()});
        let (_, streamed) = normalize_sse_stream(
            AdapterId::Responses,
            format!("data: {event}\n\n").as_bytes(),
        )
        .unwrap();
        assert!(
            !streamed.is_final_answer(),
            "stream body marker promoted: {text}"
        );
    }
}

#[test]
fn native_client_search_decodes_object_arguments_and_preserves_native_replay() {
    let item = json!({"type":"tool_search_call","execution":"client","status":"completed","call_id":"search-1","arguments":{"query":"shipping ETA","limit":5}});
    let bytes = response(json!([item.clone()]));
    let terminal = normalize_response(AdapterId::Responses, &bytes).unwrap();
    assert_eq!(terminal.tool_calls.len(), 1);
    assert_eq!(terminal.tool_calls[0].name, "tool_search");
    assert_eq!(terminal.tool_calls[0].arguments, item["arguments"]);
    assert!(!terminal.is_final_answer());
    for frames in [
        vec![
            json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[item.clone()]}}),
        ],
        vec![
            json!({"type":"response.output_item.added","output_index":0,"item":{"type":"tool_search_call","execution":"client","status":"in_progress","call_id":"search-1","arguments":{}}}),
            json!({"type":"response.output_item.done","output_index":0,"item":item.clone()}),
            json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[item.clone()]}}),
        ],
    ] {
        let wire = frames.iter().fold(String::new(), |mut acc, event| {
            acc.push_str(&format!("data: {event}\n\n"));
            acc
        });
        let (_, streamed) = normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).unwrap();
        assert_eq!(streamed.tool_calls, terminal.tool_calls);
        assert_eq!(
            streamed.sealed_fragments,
            json!([item.clone()]),
            "native calls must not acquire duplicate synthetic function calls"
        );
        assert!(!streamed.is_final_answer());
    }
    assert!(
        provider::normalize_dialect_response(provider::DialectId::DeepseekResponsesV1, &bytes)
            .is_err()
    );
}

#[test]
fn native_search_rejects_nonclient_incomplete_or_unbound_calls() {
    let base = json!({"type":"tool_search_call","execution":"client","status":"completed","call_id":"search-1","arguments":{"query":"shipping"}});
    for (key, value) in [
        ("execution", json!("server")),
        ("status", json!("in_progress")),
        ("call_id", json!("")),
        ("arguments", json!("{}")),
        ("name", json!("shell")),
    ] {
        let mut item = base.clone();
        item[key] = value;
        assert!(
            normalize_response(AdapterId::Responses, &response(json!([item]))).is_err(),
            "{key}"
        );
    }
    let mut changed = base.clone();
    changed["arguments"] = json!({"query":"changed"});
    let events = [
        json!({"type":"response.output_item.done","output_index":0,"item":base}),
        json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[changed]}}),
    ];
    let wire = events.iter().fold(String::new(), |mut acc, event| {
        acc.push_str(&format!("data: {event}\n\n"));
        acc
    });
    assert!(normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).is_err());
}

#[test]
fn native_search_added_identity_cannot_change_at_completion() {
    let events = [
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"tool_search_call","execution":"client","status":"in_progress","call_id":"original","arguments":{}}}),
        json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[{"type":"tool_search_call","execution":"client","status":"completed","call_id":"changed","arguments":{"query":"shipping"}}]}}),
    ];
    let wire = events.iter().fold(String::new(), |mut acc, event| {
        acc.push_str(&format!("data: {event}\n\n"));
        acc
    });
    assert!(normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).is_err());
}

#[test]
fn kimi_terminal_choice_usage_is_preserved_without_reinterpreting_other_dialects() {
    // Shape and counters observed on the real Kimi K3 live-write terminal frame.
    let chunk = json!({"id":"kimi-live-shape","choices":[{"index":0,
        "delta":{"content":"LIVE_OK_DONE"},"finish_reason":"stop",
        "usage":{"prompt_tokens":1322,"completion_tokens":56,"total_tokens":1378,
            "cached_tokens":768,"completion_tokens_details":{"reasoning_tokens":37},
            "prompt_tokens_details":{"cached_tokens":768}}}]});
    let wire = format!("data: {chunk}\n\ndata: [DONE]\n\n");
    let (_, terminal) =
        provider::normalize_dialect_sse_stream(provider::DialectId::KimiChatV1, wire.as_bytes())
            .unwrap();
    assert!(terminal.is_final_answer());
    let usage = terminal.usage.unwrap();
    assert_eq!(usage.input_tokens.as_deref(), Some("1322"));
    assert_eq!(usage.output_tokens.as_deref(), Some("56"));
    assert_eq!(usage.cache_read.as_deref(), Some("768"));
    let (_, other) =
        provider::normalize_dialect_sse_stream(provider::DialectId::OllamaChatV1, wire.as_bytes())
            .unwrap();
    assert!(other.usage.is_none());
    let mut alternate = chunk.clone();
    alternate["choices"][0]["index"] = json!(1);
    let wire = format!("data: {alternate}\n\ndata: [DONE]\n\n");
    let (_, terminal) =
        provider::normalize_dialect_sse_stream(provider::DialectId::KimiChatV1, wire.as_bytes())
            .unwrap();
    assert!(
        terminal.usage.is_none(),
        "alternate choices must not be summed as request usage"
    );
    let mut top_level = chunk;
    top_level["usage"] = json!({"prompt_tokens":1500,"completion_tokens":60});
    let wire = format!("data: {top_level}\n\ndata: [DONE]\n\n");
    let (_, terminal) =
        provider::normalize_dialect_sse_stream(provider::DialectId::KimiChatV1, wire.as_bytes())
            .unwrap();
    assert_eq!(
        terminal.usage.unwrap().input_tokens.as_deref(),
        Some("1500")
    );
}

#[test]
fn chat_finish_reason_closes_content_but_allows_usage_tail() {
    for delta in [
        json!({"content":"late text"}),
        json!({"reasoning_content":"late reasoning"}),
        json!({"tool_calls":[{"index":0,"function":{"arguments":"{}"}}]}),
    ] {
        let wire = format!(
            "data: {}\n\ndata: {}\n\ndata: [DONE]\n\n",
            json!({"choices":[{"index":0,"delta":{"content":"done"},"finish_reason":"stop"}]}),
            json!({"choices":[{"index":0,"delta":delta,"finish_reason":null}]})
        );
        let error = normalize_sse_stream(AdapterId::ChatCompletion, wire.as_bytes()).unwrap_err();
        assert!(error.contains("content follows finish_reason"), "{error}");
    }
    let wire = concat!(
        "data: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"done\"},\"finish_reason\":\"stop\"}]}\n\n",
        "data: {\"choices\":[],\"usage\":{\"prompt_tokens\":7,\"completion_tokens\":2}}\n\n",
        "data: [DONE]\n\n"
    );
    let (_, terminal) = normalize_sse_stream(AdapterId::ChatCompletion, wire.as_bytes()).unwrap();
    assert!(terminal.is_final_answer());
    assert_eq!(terminal.usage.unwrap().input_tokens.as_deref(), Some("7"));
}

#[test]
fn responses_tool_arguments_cannot_change_after_streaming() {
    let added = json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","call_id":"c1","name":"record_left","arguments":""}});
    let delta = json!({"type":"response.function_call_arguments.delta","output_index":0,"delta":"{\"value\":\"A\"}"});
    let done = json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":"{\"value\":\"A\"}"});
    let terminal = json!({"type":"response.completed","response":{"id":"r1","status":"completed"}});
    for (middle, valid) in [
        (vec![delta.clone(), done.clone()], true),
        (
            vec![
                delta.clone(),
                json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":"{\"value\":\"B\"}"}),
            ],
            false,
        ),
        (vec![delta.clone(), done.clone(), delta.clone()], false),
        (
            vec![
                json!({"type":"response.function_call_arguments.delta","output_index":0,"call_id":"other","delta":"{}"}),
            ],
            false,
        ),
        (
            vec![
                json!({"type":"response.function_call_arguments.delta","output_index":0,"name":"record_right","delta":"{}"}),
            ],
            false,
        ),
    ] {
        let events = std::iter::once(added.clone())
            .chain(middle)
            .chain(std::iter::once(terminal.clone()));
        let wire = events.fold(String::new(), |mut acc, e| {
            acc.push_str(&format!("data: {e}\n\n"));
            acc
        });
        assert_eq!(
            normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).is_ok(),
            valid,
            "{wire}"
        );
    }
}

#[test]
fn completed_tool_manifest_cannot_rewrite_or_drop_sink_calls() {
    let original = json!({"type":"function_call","call_id":"c1","name":"record_left","arguments":"{\"value\":\"A\"}"});
    let mut name = original.clone();
    name["name"] = json!("record_right");
    let mut id = original.clone();
    id["call_id"] = json!("c2");
    let mut args = original.clone();
    args["arguments"] = json!("{\"value\":\"B\"}");
    let mut whitespace = original.clone();
    whitespace["arguments"] = json!("{ \"value\" : \"A\" }");
    for (item, valid) in [
        (original.clone(), true),
        (whitespace, true),
        (name, false),
        (id, false),
        (args, false),
        (Value::Null, false),
    ] {
        for item_done in [false, true] {
            let mut events = vec![
                json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","call_id":"c1","name":"record_left","arguments":""}}),
                json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":original["arguments"]}),
            ];
            if item_done {
                events
                    .push(json!({"type":"response.output_item.done","output_index":0,"item":item}));
            }
            events.push(json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":if item.is_null() {json!([])} else {json!([item])}}}));
            let wire = events.iter().fold(String::new(), |mut acc, e| {
                acc.push_str(&format!("data: {e}\n\n"));
                acc
            });
            assert_eq!(
                normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).is_ok(),
                valid,
                "{wire}"
            );
        }
    }
}

fn fixture(path: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/../../fixtures/{path}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("fixture")
}

/// Every streaming adapter announces a call exactly once when its identity and
/// arguments are complete, with the same arguments the terminal manifest
/// carries; the announcement precedes the terminal.
#[test]
fn every_streaming_adapter_announces_each_call_once_with_terminal_arguments() {
    let google = concat!(
        "data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[{\"functionCall\":{\"id\":\"g-1\",\"name\":\"lookup\",\"args\":{\"query\":\"gen\"}}}]}}]}\n\n",
        "data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[]},\"finishReason\":\"STOP\"}],\"usageMetadata\":{\"promptTokenCount\":3,\"candidatesTokenCount\":2,\"totalTokenCount\":5}}\n\n",
    ).as_bytes().to_vec();
    for (adapter, wire, id, arguments) in [
        (
            AdapterId::Anthropic,
            fixture("provider-runtime/predecessor/anthropic.stream.sse"),
            "toolu_snap",
            json!({"q":"x"}),
        ),
        (
            AdapterId::ChatCompletion,
            fixture("provider-runtime/predecessor/chat_completion.stream.sse"),
            "call_stream",
            json!({"query":"stream"}),
        ),
        (
            AdapterId::GoogleInteractions,
            fixture("provider-runtime/google-interactions.stream.sse"),
            "call-stream",
            json!({"query":"contract"}),
        ),
        (
            AdapterId::GoogleGeneration,
            google,
            "g-1",
            json!({"query":"gen"}),
        ),
    ] {
        let (frames, terminal) = normalize_sse_stream(adapter, &wire)
            .unwrap_or_else(|error| panic!("{adapter:?}: {error}"));
        let ready = frames
            .iter()
            .filter_map(|frame| match frame {
                provider::ProviderFrame::ToolCallReady(call) => Some(call.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(ready.len(), 1, "{adapter:?} announces once");
        assert_eq!(ready[0].call_id, id, "{adapter:?}");
        assert_eq!(ready[0].name, "lookup", "{adapter:?}");
        assert_eq!(ready[0].arguments, arguments, "{adapter:?}");
        assert_eq!(terminal.tool_calls.len(), 1, "{adapter:?}");
        assert_eq!(
            terminal.tool_calls[0], ready[0],
            "{adapter:?}: announced call equals the terminal call"
        );
        let last_delta = frames
            .iter()
            .rposition(|frame| matches!(frame, provider::ProviderFrame::ToolDelta { .. }));
        let ready_at = frames
            .iter()
            .position(|frame| matches!(frame, provider::ProviderFrame::ToolCallReady(_)))
            .unwrap();
        assert!(
            last_delta.is_none_or(|delta| delta < ready_at),
            "{adapter:?}: readiness follows the last argument delta"
        );
    }
}

#[test]
fn anthropic_rejects_argument_deltas_after_the_block_stopped() {
    let wire = concat!(
        "data: {\"type\":\"message_start\",\"message\":{\"usage\":{\"input_tokens\":1}}}\n\n",
        "data: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"tool_use\",\"id\":\"t1\",\"name\":\"lookup\",\"input\":{}}}\n\n",
        "data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"input_json_delta\",\"partial_json\":\"{\\\"q\\\":1}\"}}\n\n",
        "data: {\"type\":\"content_block_stop\",\"index\":0}\n\n",
        "data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"input_json_delta\",\"partial_json\":\"x\"}}\n\n",
        "data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"tool_use\"},\"usage\":{\"output_tokens\":1}}\n\n",
        "data: {\"type\":\"message_stop\"}\n\n",
    );
    assert!(normalize_sse_stream(AdapterId::Anthropic, wire.as_bytes()).is_err());
}

/// Model output that is not usable JSON (here a duplicate member, as
/// gpt-5.6-luna emitted on SWE-bench) keeps the call's identity: readiness
/// and the terminal both carry the invalid-arguments sentinel, the response
/// stays a normal terminal (no provider_terminal error), and the worker
/// refuses the call with a durable error result the model can answer.
#[test]
fn unusable_tool_arguments_become_a_refused_call_not_a_malformed_stream() {
    let raw = "{\"command\":\"ls\",\"artifact_outputs\":[],\"artifact_outputs\":[]}";
    let item = json!({"type":"function_call","call_id":"c-dup","name":"shell","arguments":raw,"status":"completed"});
    let events = [
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","call_id":"c-dup","name":"shell","arguments":""}}),
        json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":raw}),
        json!({"type":"response.output_item.done","output_index":0,"item":item}),
        json!({"type":"response.completed","response":{"id":"r-dup","status":"completed","output":[item]}}),
    ];
    let wire = events.iter().fold(String::new(), |mut acc, event| {
        acc.push_str(&format!("data: {event}\n\n"));
        acc
    });
    let (frames, terminal) =
        normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).expect("a usable terminal");
    let ready = frames
        .iter()
        .find_map(|frame| match frame {
            provider::ProviderFrame::ToolCallReady(call) => Some(call),
            _ => None,
        })
        .expect("ready frame");
    assert_eq!(ready.call_id, "c-dup");
    assert!(
        ready
            .arguments
            .get(provider::INVALID_ARGUMENTS_KEY)
            .is_some(),
        "{:?}",
        ready.arguments
    );
    assert_eq!(terminal.tool_calls.len(), 1);
    assert_eq!(
        terminal.tool_calls[0].arguments, ready.arguments,
        "readiness and terminal resolve to one sentinel"
    );
    let detail = provider::invalid_arguments_detail(&terminal.tool_calls[0].arguments)
        .expect("refusal detail");
    assert!(detail.contains("duplicate object member"), "{detail}");
    assert!(provider::invalid_arguments_detail(&json!({"command":"ls"})).is_none());
    // A manifest that repeats different unusable text is still a contract violation.
    let other = json!({"type":"function_call","call_id":"c-dup","name":"shell","arguments":"{\"a\":1,\"a\":2}","status":"completed"});
    let events = [
        json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","call_id":"c-dup","name":"shell","arguments":""}}),
        json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":raw}),
        json!({"type":"response.output_item.done","output_index":0,"item":other}),
        json!({"type":"response.completed","response":{"id":"r-dup","status":"completed","output":[other]}}),
    ];
    let wire = events.iter().fold(String::new(), |mut acc, event| {
        acc.push_str(&format!("data: {event}\n\n"));
        acc
    });
    assert!(normalize_sse_stream(AdapterId::Responses, wire.as_bytes()).is_err());
}
