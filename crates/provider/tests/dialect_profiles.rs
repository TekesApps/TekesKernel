use std::collections::BTreeSet;
use std::fs;

use engine::{
    AdapterCapabilities, Continuation, QueryCapability, QueryResult, RecoveryDecision, RunMode,
    SentState, decide_recovery,
};
use profile::{Model, Provider, SessionSettings};
use provider::{
    DialectId, PrepareInput, ProviderTarget, epoch_profile, normalize_dialect_response,
    normalize_dialect_sse_stream, prepare, repair_tool_arguments, resolve_profile, validate_target,
};
use schema::IJsonValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use test_support::FixtureRoot;

fn corpus() -> Value {
    let root = FixtureRoot::discover().expect("fixture root");
    serde_json::from_slice(
        &fs::read(root.join("provider-dialects/profiles.canonical.json"))
            .expect("provider dialect fixture"),
    )
    .expect("provider dialect JSON")
}

fn invalid_corpus() -> Value {
    let root = FixtureRoot::discover().expect("fixture root");
    serde_json::from_slice(
        &fs::read(root.join("provider-dialects/invalid.canonical.json"))
            .expect("provider dialect invalid fixture"),
    )
    .expect("provider dialect invalid JSON")
}

fn forward_compatible_corpus() -> Value {
    let root = FixtureRoot::discover().expect("fixture root");
    let read_canonical = |name: &str| {
        let bytes = fs::read(root.join(format!("provider-dialects/{name}")))
            .expect("provider dialect forward-compatible fixture");
        let value: Value =
            serde_json::from_slice(&bytes).expect("provider dialect forward-compatible JSON");
        let mut canonical =
            serde_json_canonicalizer::to_vec(&value).expect("canonical forward-compatible JSON");
        canonical.push(b'\n');
        assert_eq!(bytes, canonical, "{name} is not RFC 8785 canonical JSON");
        value
    };
    let mut primary = read_canonical("forward-compatible.canonical.json");
    let additional = read_canonical("forward-compatible-additional.canonical.json");
    primary["cases"]
        .as_array_mut()
        .expect("primary cases")
        .extend(
            additional["cases"]
                .as_array()
                .expect("additional cases")
                .iter()
                .cloned(),
        );
    primary
}

fn ijson(value: &Value) -> IJsonValue {
    IJsonValue::parse(&serde_json_canonicalizer::to_vec(value).expect("canonical fixture value"))
        .expect("I-JSON fixture value")
}

fn endpoint(profile: &Value) -> &str {
    profile["endpoint"].as_str().expect("fixture endpoint")
}

fn text(role: &str, value: &str) -> IJsonValue {
    ijson(&json!({"content":[{"text":value,"type":"text"}],"role":role}))
}

fn chat_assistant() -> IJsonValue {
    ijson(&json!({"content":[
        {"text":"world","type":"text"},
        {"text":"reason","type":"reasoning"}
    ],"role":"assistant"}))
}

fn tool_catalog() -> IJsonValue {
    ijson(&json!([{
        "description":"fixture tool","name":"lookup",
        "parameters":{"properties":{"q":{"type":"string"}},"required":["q"],"type":"object"}
    }]))
}

#[test]
fn model_capability_catalog_preserves_deepseek_effort_order() {
    let fixture = corpus();
    let profile = fixture["profiles"]
        .as_array()
        .expect("profiles")
        .iter()
        .find(|profile| {
            profile["target"]["model_profile_id"] == "deepseek_responses_v1:deepseek-v4-flash"
        })
        .expect("DeepSeek V4 Flash profile");
    let target: ProviderTarget =
        serde_json::from_value(profile["target"].clone()).expect("DeepSeek target");
    let resolved = validate_target(&target).expect("configured DeepSeek profile");

    assert_eq!(resolved.reasoning_efforts(), ["low", "high", "max"]);
    assert_eq!(resolved.default_reasoning_effort(), Some("high"));
}

#[test]
fn compactor_tool_choice_preserves_reasoning_and_route_contract() {
    let fixture = corpus();
    for (model, choice) in [
        ("deepseek_responses_v1:deepseek-v4-flash", Some("auto")),
        ("google_interactions_v1:gemini-2.5-flash", None),
        ("openai_responses_v1:gpt-5", Some("required")),
    ] {
        let profile = fixture["profiles"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["target"]["model_profile_id"] == model)
            .unwrap();
        let target: ProviderTarget = serde_json::from_value(profile["target"].clone()).unwrap();
        let resolved = validate_target(&target).unwrap();
        let request = provider::prepare_summary_request(
            profile["endpoint"].as_str().unwrap(),
            &resolved,
            engine::SUMMARY_SYSTEM,
            "seq 2 input {\"text\":\"Keep AZURE-17\"}",
            ijson(
                &json!([{"name":"summary_artifact","description":"Summarize accepted history",
                "parameters":{"type":"object","properties":{"continuation":{"type":"string"},
                    "evidence_refs":{"type":"array","items":{"type":"integer"}}},
                    "required":["continuation","evidence_refs"],"additionalProperties":false}}]),
            ),
            "summary-route-contract".into(),
        )
        .expect("compactor request must prepare on this route");
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body.get("tool_choice").and_then(Value::as_str),
            choice,
            "{model}"
        );
        if resolved.dialect == DialectId::DeepseekResponsesV1 {
            assert_eq!(body["reasoning"]["effort"], "high");
            assert_eq!(body["stream"], false);
        }
    }
}

#[test]
fn configured_cloudflare_routes_resolve_with_their_dialect_credentials() {
    let cases = [
        (
            Provider {
                id: "cf-openai".to_owned(),
                name: Some("OpenAI".to_owned()),
                adapter: "responses".to_owned(),
                dialect: "openai_responses_v1".to_owned(),
                endpoint_owner: "cloudflare".to_owned(),
                gateway_translation: "router".to_owned(),
                evidence_revision: "legacy-example-v2".to_owned(),
                endpoint: "https://api.cloudflare.com/client/v4/accounts/example/ai/v1".to_owned(),
                credential_key: None,
                models: Vec::new(),
            },
            Model {
                id: "openai/gpt-5.6-luna".to_owned(),
                profile: "openai_responses_v1:openai/gpt-5.6-luna".to_owned(),
                enabled: true,
                context_window_tokens: 200_000,
                compact_trigger_tokens: 180_000,
            },
            "authorization",
            "Bearer ",
        ),
        (
            Provider {
                id: "cf-anthropic".to_owned(),
                name: Some("Anthropic".to_owned()),
                adapter: "anthropic_messages".to_owned(),
                dialect: "anthropic_messages_v1".to_owned(),
                endpoint_owner: "cloudflare".to_owned(),
                gateway_translation: "cloudflare-native-anthropic".to_owned(),
                evidence_revision: "legacy-example-v2".to_owned(),
                endpoint: "https://gateway.ai.cloudflare.com/v1/example/tekesrouter/anthropic"
                    .to_owned(),
                credential_key: None,
                models: Vec::new(),
            },
            Model {
                id: "claude-opus-5".to_owned(),
                profile: "anthropic_messages_v1:claude-opus-5".to_owned(),
                enabled: true,
                context_window_tokens: 200_000,
                compact_trigger_tokens: 180_000,
            },
            "cf-aig-authorization",
            "Bearer ",
        ),
    ];

    for (provider, model, credential_header, credential_prefix) in cases {
        provider::validate_provider(&provider).expect("configured dialect");
        let resolved = resolve_profile(&provider, &model).expect("configured target");
        assert_eq!(resolved.credential_header, credential_header);
        assert_eq!(resolved.credential_prefix, credential_prefix);
    }
}

fn block_item(block: Value, role: &str) -> IJsonValue {
    ijson(&json!({"content":[block],"role":role}))
}

fn prepare_error(
    _profile: &Value,
    target: ProviderTarget,
    endpoint_value: &str,
    epoch: &Value,
    rendered_items: Vec<IJsonValue>,
    tools: IJsonValue,
) -> String {
    prepare(&PrepareInput {
        attempt_id: "negative".to_owned(),
        target,
        endpoint: endpoint_value.to_owned(),
        epoch_profile: ijson(epoch),
        continuation_id: None,
        rendered_items,
        tool_catalog: tools,
        stream: true,
    })
    .expect_err("negative case unexpectedly prepared")
    .to_string()
}

fn terminal_summary(terminal: &provider::ProviderTerminal) -> Value {
    let mut usage = serde_json::Map::new();
    if let Some(reported) = &terminal.usage {
        if let Some(value) = &reported.input_tokens {
            usage.insert("input_tokens".to_owned(), json!(value));
        }
        if let Some(value) = &reported.output_tokens {
            usage.insert("output_tokens".to_owned(), json!(value));
        }
        if let Some(value) = &reported.cache_read {
            usage.insert("cache_read".to_owned(), json!(value));
        }
    }
    json!({
        "content":terminal.content,
        "finish_reason":terminal.finish_reason,
        "usage":usage,
    })
}

fn tool_terminal_summary(terminal: &provider::ProviderTerminal) -> Value {
    let usage = terminal
        .usage
        .as_ref()
        .map(|usage| {
            json!({
                "input_tokens":usage.input_tokens,
                "output_tokens":usage.output_tokens,
            })
        })
        .unwrap_or_else(|| json!({}));
    json!({
        "finish_reason":terminal.finish_reason,
        "tool_calls":terminal.tool_calls,
        "usage":usage,
    })
}

fn assert_reasoning_wire(dialect: DialectId, effort: &str, body: &[u8]) {
    let body: Value = serde_json::from_slice(body).expect("reasoning request JSON");
    match dialect {
        DialectId::OpenaiResponsesV1 | DialectId::DeepseekResponsesV1 => {
            assert_eq!(body.pointer("/reasoning/effort"), Some(&json!(effort)));
        }
        DialectId::DeepseekChatV1 => {
            assert_eq!(body.get("reasoning_effort"), Some(&json!(effort)));
            assert_eq!(body.pointer("/thinking/type"), Some(&json!("enabled")));
        }
        DialectId::KimiChatV1 => {
            assert_eq!(body.get("reasoning_effort"), Some(&json!(effort)));
            assert!(body.get("thinking").is_none());
        }
        DialectId::GlmChatV1 => {
            assert_eq!(body.get("reasoning_effort"), Some(&json!(effort)));
            assert_eq!(
                body.pointer("/thinking/clear_thinking"),
                Some(&json!(false))
            );
        }
        DialectId::GoogleGenerationV1 => {
            let budget = match effort {
                "low" => 1_024,
                "medium" => 2_048,
                "high" => 4_096,
                other => panic!("unproved Google effort {other}"),
            };
            assert_eq!(
                body.pointer("/generationConfig/thinkingConfig/thinkingBudget"),
                Some(&json!(budget))
            );
        }
        DialectId::GoogleInteractionsV1 => {
            assert_eq!(
                body.pointer("/generation_config/thinking_level"),
                Some(&json!(effort))
            );
        }
        DialectId::GenericChatV1
        | DialectId::OpenaiChatV1
        | DialectId::OllamaChatV1
        | DialectId::AnthropicMessagesV1
        | DialectId::DeepseekAnthropicV1 => {
            panic!("reasoning control reached an unsupported dialect")
        }
    }
}

fn prepare_fixture(
    profile: &Value,
    target: ProviderTarget,
    epoch_profile: IJsonValue,
    rendered_items: Vec<IJsonValue>,
    tools: IJsonValue,
) -> Result<provider::PreparedRequest, provider::PrepareError> {
    prepare(&PrepareInput {
        attempt_id: "fixture-attempt".to_owned(),
        target,
        endpoint: endpoint(profile).to_owned(),
        epoch_profile,
        continuation_id: None,
        rendered_items,
        tool_catalog: tools,
        stream: true,
    })
}

/// Exercise the native continuation shape here. Stateless Interactions
/// recovery has separate coverage for its quoted tool-history fallback.
fn prepare_fixture_as_sent(
    profile: &Value,
    target: ProviderTarget,
    epoch_profile: IJsonValue,
    rendered_items: Vec<IJsonValue>,
    tools: IJsonValue,
) -> Result<provider::PreparedRequest, provider::PrepareError> {
    let continuation_id = target
        .dialect()
        .ok()
        .filter(|dialect| dialect.server_managed())
        .map(|_| "fixture-continuation".to_owned());
    prepare(&PrepareInput {
        attempt_id: "fixture-attempt".to_owned(),
        target,
        endpoint: endpoint(profile).to_owned(),
        epoch_profile,
        continuation_id,
        rendered_items,
        tool_catalog: tools,
        stream: true,
    })
}

fn prepared(profile: &Value, multiturn: bool) -> provider::PreparedRequest {
    let target: ProviderTarget =
        serde_json::from_value(profile["target"].clone()).expect("fixture target");
    let dialect = target.dialect().expect("known fixture dialect");
    let (continuation_id, rendered_items) = if multiturn && dialect.server_managed() {
        let identity = if dialect == DialectId::GoogleInteractionsV1 {
            "int_1"
        } else {
            "resp_1"
        };
        (Some(identity.to_owned()), vec![text("user", "again")])
    } else if multiturn {
        let assistant = if matches!(
            dialect,
            DialectId::DeepseekChatV1 | DialectId::KimiChatV1 | DialectId::GlmChatV1
        ) {
            chat_assistant()
        } else {
            text("assistant", "world")
        };
        (
            None,
            vec![text("user", "hello"), assistant, text("user", "again")],
        )
    } else {
        (None, vec![text("user", "hello")])
    };
    prepare(&PrepareInput {
        attempt_id: "fixture-attempt".to_owned(),
        target,
        endpoint: endpoint(profile).to_owned(),
        epoch_profile: ijson(&profile["control"]["epoch_profile"]),
        continuation_id,
        rendered_items,
        tool_catalog: ijson(&json!([])),
        stream: true,
    })
    .expect("production projector accepts proved tuple")
}

#[test]
fn interactions_recovery_quotes_sealed_calls_and_results_then_resumes_native_tools() {
    let fixtures = corpus();
    let profile = fixtures["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|profile| {
            serde_json::from_value::<ProviderTarget>(profile["target"].clone())
                .unwrap()
                .dialect()
                .unwrap()
                == DialectId::GoogleInteractionsV1
        })
        .unwrap();
    let call = json!({"type":"function_call","id":"c1","name":"lookup","arguments":{"query":"saved work"}});
    let result = ijson(
        &json!({"role":"tool","content":[{"type":"tool_result","call_id":"c1","name":"lookup","result":{"answer":"already done"}}]}),
    );
    let mut input = PrepareInput {
        attempt_id: "recover".into(),
        target: serde_json::from_value(profile["target"].clone()).unwrap(),
        endpoint: endpoint(profile).into(),
        epoch_profile: ijson(&profile["control"]["epoch_profile"]),
        continuation_id: None,
        rendered_items: vec![
            text("user", "continue my work"),
            ijson(&json!({"role":"sealed","adapter":"google_interactions_v1","fragments":[call]})),
            result.clone(),
        ],
        tool_catalog: tool_catalog(),
        stream: true,
    };
    let recovered: Value = serde_json::from_slice(&prepare(&input).unwrap().body).unwrap();
    assert!(recovered.get("previous_interaction_id").is_none());
    let steps = recovered["input"].as_array().unwrap();
    assert!(steps.iter().all(|step| !matches!(
        step["type"].as_str(),
        Some("function_call" | "function_result")
    )));
    assert!(
        steps[1]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("saved work")
    );
    assert!(
        steps[2]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("already done")
    );
    assert!(steps[1].get("role").is_none());
    // A newly established server chain sends native results, not more quoted history.
    input.continuation_id = Some("new-chain".into());
    input.rendered_items = vec![result];
    let continued: Value = serde_json::from_slice(&prepare(&input).unwrap().body).unwrap();
    assert_eq!(continued["input"][0]["type"], "function_result");
    input.continuation_id = None;
    assert!(
        prepare(&input).is_err(),
        "orphan results must not be presented as completed history"
    );
}

#[test]
fn provider_dialect_gate_93_registry_closure() {
    let value = corpus();
    let required = value["required_proof_arms"]
        .as_array()
        .expect("required arms");
    let profiles = value["profiles"].as_array().expect("profiles");
    assert_eq!(profiles.len(), 14);
    assert_eq!(
        profiles
            .iter()
            .filter(|profile| profile["advertised"] == true)
            .count(),
        13,
        "the generic fixture tuple is test-only"
    );
    let mut identities = BTreeSet::new();
    for profile in profiles {
        let target = &profile["target"];
        let identity = serde_json_canonicalizer::to_string(target).expect("target JCS");
        assert!(identities.insert(identity), "duplicate target identity");
        if profile["advertised"] == true {
            for arm in required {
                assert!(
                    profile.get(arm.as_str().expect("arm string")).is_some(),
                    "advertised profile lacks {arm}"
                );
            }
        }
        if profile["target"]["dialect_id"] == "generic_chat_v1" {
            let target: ProviderTarget =
                serde_json::from_value(target.clone()).expect("generic target");
            let provider = Provider {
                id: "generic".to_owned(),
                name: None,
                adapter: target.protocol_family.clone(),
                dialect: target.dialect_id.clone(),
                endpoint_owner: target.route.endpoint_owner.clone(),
                gateway_translation: target.route.gateway_translation.clone(),
                evidence_revision: target.route.evidence_revision.clone(),
                endpoint: endpoint(profile).to_owned(),
                credential_key: None,
                models: Vec::new(),
            };
            let model = Model {
                id: target.route.exact_sku.clone(),
                profile: target.model_profile_id.clone(),
                enabled: true,
                context_window_tokens: 100_000,
                compact_trigger_tokens: 80_000,
            };
            resolve_profile(&provider, &model).expect("configured generic dialect");
        }
        let mut mismatch: ProviderTarget =
            serde_json::from_value(target.clone()).expect("fixture target");
        mismatch.dialect_id = profile["mismatch"]["other_dialect"]
            .as_str()
            .expect("mismatch dialect")
            .to_owned();
        let rejected = prepare(&PrepareInput {
            attempt_id: "mismatch".to_owned(),
            target: mismatch,
            endpoint: endpoint(profile).to_owned(),
            epoch_profile: ijson(&profile["control"]["epoch_profile"]),
            continuation_id: None,
            rendered_items: vec![text("user", "hello")],
            tool_catalog: ijson(&json!([])),
            stream: true,
        });
        assert!(rejected.is_err(), "route/dialect mismatch must fail closed");

        let exact: ProviderTarget = serde_json::from_value(target.clone()).expect("exact target");
        let mut family_mismatch = exact.clone();
        family_mismatch.protocol_family.push_str("-mismatch");
        assert!(
            validate_target(&family_mismatch).is_err(),
            "protocol family and dialect disagreement must still fail"
        );
        let mutations: [fn(&mut ProviderTarget); 5] = [
            |target: &mut ProviderTarget| target.model_profile_id.push_str("-mismatch"),
            |target: &mut ProviderTarget| target.route.endpoint_owner.push_str("-mismatch"),
            |target: &mut ProviderTarget| target.route.gateway_translation.push_str("-mismatch"),
            |target: &mut ProviderTarget| target.route.exact_sku.push_str("-mismatch"),
            |target: &mut ProviderTarget| target.route.evidence_revision.push_str("-mismatch"),
        ];
        for mutate in mutations {
            let mut candidate = exact.clone();
            mutate(&mut candidate);
            let resolved = validate_target(&candidate).expect("configured target");
            let epoch = epoch_profile(&resolved, "system", None).expect("unverified epoch");
            prepare(&PrepareInput {
                attempt_id: "unverified".to_owned(),
                target: candidate,
                endpoint: endpoint(profile).to_owned(),
                epoch_profile: epoch,
                continuation_id: None,
                rendered_items: vec![text("user", "hello")],
                tool_catalog: ijson(&json!([])),
                stream: true,
            })
            .expect("unverified configured target remains executable");
        }
        let wrong_endpoint = format!("{}/wrong-route", endpoint(profile));
        prepare(&PrepareInput {
            attempt_id: "unverified-endpoint".to_owned(),
            target: exact.clone(),
            endpoint: wrong_endpoint,
            epoch_profile: ijson(&profile["control"]["epoch_profile"]),
            continuation_id: None,
            rendered_items: vec![text("user", "hello")],
            tool_catalog: ijson(&json!([])),
            stream: true,
        })
        .expect("configured endpoint mismatch remains executable but unverified");
        let mut unknown_epoch = profile["control"]["epoch_profile"].clone();
        unknown_epoch["controls"]["unknown"] = json!(true);
        assert!(
            prepare_fixture(
                profile,
                exact.clone(),
                ijson(&unknown_epoch),
                vec![text("user", "hello")],
                ijson(&json!([])),
            )
            .is_err(),
            "unknown control must fail before send"
        );
        assert!(
            prepare_fixture(
                profile,
                exact.clone(),
                ijson(&profile["control"]["epoch_profile"]),
                vec![ijson(
                    &json!({"role":"sealed","adapter":"foreign","fragments":[]})
                )],
                ijson(&json!([])),
            )
            .is_err(),
            "foreign sealed carrier must fail before send"
        );
        assert!(
            prepare_fixture(
                profile,
                exact.clone(),
                ijson(&profile["control"]["epoch_profile"]),
                vec![ijson(
                    &json!({"role":"user","content":[{"type":"unknown"}]})
                )],
                ijson(&json!([])),
            )
            .is_err(),
            "unsupported input must fail before send"
        );
        let weak_schema = ijson(&json!([{
            "description":"weak","name":"lookup","parameters":{"type":"string"}
        }]));
        assert!(
            prepare_fixture(
                profile,
                exact.clone(),
                ijson(&profile["control"]["epoch_profile"]),
                vec![text("user", "hello")],
                weak_schema,
            )
            .is_err(),
            "schema weakening must fail before send"
        );

        for control in [
            "temperature",
            "top_p",
            "tool_choice",
            "response_format",
            "prompt_cache_key",
            "max_tokens",
        ] {
            let mut epoch = profile["control"]["epoch_profile"].clone();
            epoch["controls"][control] = match control {
                "tool_choice" => json!("required"),
                "response_format" => json!({"type":"json_object"}),
                "prompt_cache_key" => json!("session-fixture"),
                "max_tokens" => json!(512),
                _ => json!(0.5),
            };
            let error = prepare_error(
                profile,
                exact.clone(),
                endpoint(profile),
                &epoch,
                vec![text("user", "hello")],
                ijson(&json!([])),
            );
            assert!(!error.is_empty(), "{control} did not fail closed");
        }

        let dialect = exact.dialect().expect("dialect");
        assert_eq!(
            profile["capabilities"]["input_blocks"],
            json!(dialect.input_blocks())
        );
        assert_eq!(profile["capabilities"]["cache"], dialect.cache_policy());
        assert_eq!(
            profile["capabilities"]["sampling"],
            dialect.sampling_policy()
        );
        assert_eq!(profile["capabilities"]["schema"], dialect.schema_policy());
        assert_eq!(
            profile["capabilities"]["tool_choice"]["modes"],
            json!(dialect.tool_choice_modes())
        );
        assert_eq!(
            profile["capabilities"]["tool_choice"]["wire"],
            dialect.tool_choice_wire()
        );
        assert_eq!(profile["capabilities"]["repair"]["id"], dialect.repair_id());

        let image = block_item(
            json!({"data":"AA==","mime":"image/png","type":"file"}),
            "user",
        );
        let image_result = prepare_fixture(
            profile,
            exact.clone(),
            ijson(&profile["control"]["epoch_profile"]),
            vec![image],
            ijson(&json!([])),
        );
        assert_eq!(
            image_result.is_ok(),
            dialect.input_blocks().contains(&"image"),
            "image capability differs from production projection"
        );
        let file = block_item(
            json!({"data":"AA==","mime":"application/pdf","name":"x.pdf","type":"file"}),
            "user",
        );
        let file_result = prepare_fixture(
            profile,
            exact.clone(),
            ijson(&profile["control"]["epoch_profile"]),
            vec![file],
            ijson(&json!([])),
        );
        // A non-image file never fails a turn: a dialect that accepts files
        // receives the bytes, any other dialect receives the deterministic
        // text degradation (spec "File attachments").
        let file_body = String::from_utf8(
            file_result
                .unwrap_or_else(|error| {
                    panic!("{} rejected a file block: {error}", dialect.as_str())
                })
                .body,
        )
        .expect("utf-8 body");
        assert_eq!(
            !file_body.contains("Attached file x.pdf (application/pdf, 1 bytes)"),
            dialect.input_blocks().contains(&"file"),
            "{} file capability differs from production projection: {file_body}",
            dialect.as_str(),
        );
        for (name, item) in [
            ("text", text("user", "hello")),
            (
                "tool_call",
                block_item(
                    json!({"arguments":{"q":"x"},"call_id":"call-1","name":"lookup","type":"tool_call"}),
                    "assistant",
                ),
            ),
            (
                "tool_result",
                block_item(
                    json!({"call_id":"call-1","name":"lookup","result":{"ok":true},"type":"tool_result"}),
                    "tool",
                ),
            ),
        ] {
            assert!(
                prepare_fixture_as_sent(
                    profile,
                    exact.clone(),
                    ijson(&profile["control"]["epoch_profile"]),
                    vec![item],
                    ijson(&json!([])),
                )
                .is_ok(),
                "advertised {name} input is not projectable for {}",
                dialect.as_str()
            );
        }
        let reasoning = block_item(json!({"text":"r","type":"reasoning"}), "assistant");
        assert_eq!(
            prepare_fixture(
                profile,
                exact,
                ijson(&profile["control"]["epoch_profile"]),
                vec![reasoning],
                ijson(&json!([])),
            )
            .is_ok(),
            dialect.supports_reasoning_blocks(),
            "reasoning block capability differs from projection"
        );
    }
}

#[test]
fn provider_dialect_gate_93_invalid_oracle_executes() {
    let registry = corpus();
    let invalid = invalid_corpus();
    let deepseek = registry["profiles"]
        .as_array()
        .expect("profiles")
        .iter()
        .find(|profile| profile["target"]["dialect_id"] == "deepseek_responses_v1")
        .expect("DeepSeek Responses profile");
    let openai = registry["profiles"]
        .as_array()
        .expect("profiles")
        .iter()
        .find(|profile| profile["target"]["dialect_id"] == "openai_responses_v1")
        .expect("OpenAI Responses profile");
    for case in invalid["cases"].as_array().expect("invalid cases") {
        let id = case["id"].as_str().expect("case id");
        let mutation = &case["mutation"];
        match id {
            "missing-route-evidence" => {
                let mut target = deepseek["target"].clone();
                target["route"]
                    .as_object_mut()
                    .expect("route")
                    .remove(mutation["field"].as_str().expect("field"));
                assert!(serde_json::from_value::<ProviderTarget>(target).is_err());
            }
            "unknown-dialect" | "wrong-exact-sku" | "wrong-gateway" => {
                let mut target: ProviderTarget =
                    serde_json::from_value(deepseek["target"].clone()).expect("target");
                match mutation["field"].as_str().expect("field") {
                    "dialect_id" => {
                        target.dialect_id = mutation["value"].as_str().expect("value").to_owned()
                    }
                    "exact_sku" => {
                        target.route.exact_sku =
                            mutation["value"].as_str().expect("value").to_owned()
                    }
                    "gateway_translation" => {
                        target.route.gateway_translation =
                            mutation["value"].as_str().expect("value").to_owned()
                    }
                    other => panic!("unknown target mutation {other}"),
                }
                assert!(
                    prepare_error(
                        deepseek,
                        target,
                        endpoint(deepseek),
                        &deepseek["control"]["epoch_profile"],
                        vec![text("user", "hello")],
                        ijson(&json!([])),
                    )
                    .contains("target")
                );
            }
            "deepseek-responses-previous-response" => {
                let target: ProviderTarget =
                    serde_json::from_value(deepseek["target"].clone()).expect("target");
                let error = prepare(&PrepareInput {
                    attempt_id: "invalid-continuation".to_owned(),
                    target,
                    endpoint: endpoint(deepseek).to_owned(),
                    epoch_profile: ijson(&deepseek["control"]["epoch_profile"]),
                    continuation_id: Some(
                        mutation["value"].as_str().expect("continuation").to_owned(),
                    ),
                    rendered_items: vec![text("user", "hello")],
                    tool_catalog: ijson(&json!([])),
                    stream: true,
                })
                .expect_err("stateless continuation accepted")
                .to_string();
                assert!(error.contains("stateless adapter"));
            }
            "deepseek-responses-store" | "silently-ignored-control" => {
                let mut epoch = deepseek["control"]["epoch_profile"].clone();
                epoch["controls"][mutation["field"].as_str().expect("field")] =
                    mutation["value"].clone();
                let target = serde_json::from_value(deepseek["target"].clone()).expect("target");
                assert!(
                    prepare_error(
                        deepseek,
                        target,
                        endpoint(deepseek),
                        &epoch,
                        vec![text("user", "hello")],
                        ijson(&json!([])),
                    )
                    .contains("unsupported controls")
                );
            }
            "deepseek-responses-image" => {
                let target = serde_json::from_value(deepseek["target"].clone()).expect("target");
                assert!(
                    prepare_error(
                        deepseek,
                        target,
                        endpoint(deepseek),
                        &deepseek["control"]["epoch_profile"],
                        vec![block_item(mutation["block"].clone(), "user")],
                        ijson(&json!([])),
                    )
                    .contains("rejects")
                );
            }
            "sealed-carrier-cross-dialect" => {
                let target = serde_json::from_value(deepseek["target"].clone()).expect("target");
                assert!(
                    prepare_error(
                        deepseek,
                        target,
                        endpoint(deepseek),
                        &deepseek["control"]["epoch_profile"],
                        vec![ijson(&json!({
                            "adapter":mutation["adapter"],"fragments":[],"role":"sealed"
                        }))],
                        ijson(&json!([])),
                    )
                    .contains("adapter mismatch")
                );
            }
            "missing-terminal" => {
                assert!(
                    normalize_dialect_sse_stream(
                        DialectId::OpenaiResponsesV1,
                        mutation["bytes"].as_str().expect("stream bytes").as_bytes(),
                    )
                    .is_err()
                );
            }
            "deepseek-incomplete-missing-reason" | "deepseek-failed-missing-error" => {
                assert!(
                    normalize_dialect_sse_stream(
                        DialectId::DeepseekResponsesV1,
                        mutation["bytes"].as_str().expect("stream bytes").as_bytes(),
                    )
                    .is_err()
                );
            }
            "controls-stored-not-sent" => {
                let mut mutant: Value =
                    serde_json::from_str(openai["request_bytes"].as_str().expect("request bytes"))
                        .expect("request JSON");
                mutant
                    .as_object_mut()
                    .expect("request object")
                    .remove(mutation["remove"].as_str().expect("remove field"));
                assert_ne!(
                    serde_json_canonicalizer::to_vec(&mutant).expect("mutant bytes"),
                    prepared(openai, false).body,
                    "stored-only control mutant matched the production projector"
                );
            }
            other => panic!("unhandled invalid provider-dialect case {other}"),
        }
    }
}

#[test]
fn provider_dialect_gate_94_exact_dialect_bytes() {
    let value = corpus();
    for profile in value["profiles"].as_array().expect("profiles") {
        let first = prepared(profile, false);
        assert_eq!(
            first.body,
            profile["request_bytes"]
                .as_str()
                .expect("request bytes")
                .as_bytes(),
            "{} production request bytes",
            profile["target"]["dialect_id"]
        );
        assert_eq!(
            first.request_digest,
            profile["control"]["request_digest"]
                .as_str()
                .expect("request digest")
        );
        let next = prepared(profile, true);
        assert_eq!(
            next.body,
            profile["multiturn_request_bytes"]
                .as_str()
                .expect("multiturn bytes")
                .as_bytes(),
            "{} production multiturn bytes",
            profile["target"]["dialect_id"]
        );
        let target: ProviderTarget =
            serde_json::from_value(profile["target"].clone()).expect("target");
        let dialect = target.dialect().expect("known dialect");
        let with_tool = prepare_fixture(
            profile,
            target,
            ijson(&profile["control"]["epoch_profile"]),
            vec![text("user", "hello")],
            tool_catalog(),
        )
        .expect("proved schema/tool projection");
        assert_eq!(
            with_tool.body,
            profile["tool_request_bytes"]
                .as_str()
                .expect("tool request bytes")
                .as_bytes(),
            "{} production tool/schema bytes",
            profile["target"]["dialect_id"]
        );
        let tool_body: Value = serde_json::from_slice(&with_tool.body).expect("tool request JSON");
        assert!(tool_body.get("tool_choice").is_none());
        assert!(tool_body.get("temperature").is_none());
        assert!(tool_body.get("top_p").is_none());
        match dialect.cache_policy() {
            "server_managed" => assert_eq!(tool_body.get("store"), Some(&Value::Bool(true))),
            "implicit_prefix" => {
                let body = std::str::from_utf8(&with_tool.body).expect("request UTF-8");
                assert!(
                    body.find("\"tools\"") < body.find("\"messages\""),
                    "cache-stable tools must precede growing messages"
                );
            }
            "none" => assert!(tool_body.get("store").is_none()),
            "explicit_breakpoint" => {
                assert!(
                    tool_body.get("cache_control").is_none(),
                    "a root-level breakpoint caches the whole body and never reads back"
                );
                let last_tool = tool_body["tools"].as_array().and_then(|tools| tools.last());
                assert_eq!(
                    last_tool.and_then(|tool| tool.get("cache_control")),
                    Some(&json!({"type":"ephemeral"})),
                    "static breakpoint on the last tool"
                );
                let last_block = tool_body["messages"]
                    .as_array()
                    .and_then(|messages| messages.last())
                    .and_then(|message| message["content"].as_array())
                    .and_then(|content| content.last());
                assert_eq!(
                    last_block.and_then(|block| block.get("cache_control")),
                    Some(&json!({"type":"ephemeral"})),
                    "moving breakpoint on the last block of the last message"
                );
            }
            other => panic!("unknown cache policy {other}"),
        }

        for case in profile["capability_cases"]
            .as_array()
            .expect("capability cases")
        {
            let mut epoch = profile["control"]["epoch_profile"].clone();
            for (name, value) in case["controls"].as_object().expect("case controls") {
                epoch["controls"][name] = value.clone();
            }
            let result = prepare_fixture(
                profile,
                serde_json::from_value(profile["target"].clone()).expect("target"),
                ijson(&epoch),
                vec![text("user", "hello")],
                tool_catalog(),
            );
            if let Some(fragment) = case.get("reject_contains").and_then(Value::as_str) {
                let error = result.expect_err("capability rejection case prepared");
                assert!(
                    error.to_string().contains(fragment),
                    "{} {} rejected for the wrong reason: {error}",
                    profile["target"]["dialect_id"],
                    case["id"]
                );
                continue;
            }
            let body: Value = serde_json::from_slice(
                &result
                    .unwrap_or_else(|error| {
                        panic!(
                            "{} {} failed production projection: {error}",
                            profile["target"]["dialect_id"], case["id"]
                        )
                    })
                    .body,
            )
            .expect("capability request JSON");
            if let Some(expected) = case.get("expect").and_then(Value::as_object) {
                for (name, value) in expected {
                    assert_eq!(
                        body.get(name),
                        Some(value),
                        "{} {} field {name}",
                        profile["target"]["dialect_id"],
                        case["id"]
                    );
                }
            }
            if let Some(absent) = case.get("expect_absent").and_then(Value::as_array) {
                for name in absent {
                    assert!(
                        body.get(name.as_str().expect("field name")).is_none(),
                        "{} {} unexpectedly emitted {name}",
                        profile["target"]["dialect_id"],
                        case["id"]
                    );
                }
            }
        }

        let raw = profile["stream_bytes"].as_str().expect("stream bytes");
        assert!(
            !raw.contains("\\n"),
            "SSE oracle contains literal newline escapes"
        );
        let (_, terminal) =
            normalize_dialect_sse_stream(dialect, raw.as_bytes()).unwrap_or_else(|error| {
                panic!(
                    "{} production decoder failed exact stream: {error}",
                    profile["target"]["dialect_id"]
                )
            });
        if profile["terminal"].get("cases").is_none() {
            assert_eq!(
                terminal_summary(&terminal),
                profile["terminal"],
                "{} normalized terminal",
                profile["target"]["dialect_id"]
            );
        }
        if let Some(variants) = profile.get("stream_variants").and_then(Value::as_object) {
            for (name, value) in variants {
                let raw = value.as_str().expect("stream variant bytes");
                assert!(
                    !raw.contains("\\n"),
                    "{name} contains literal newline escapes"
                );
                let (_, terminal) = normalize_dialect_sse_stream(dialect, raw.as_bytes())
                    .unwrap_or_else(|error| panic!("{name} production decode failed: {error}"));
                let expected = profile["terminal"]["cases"]
                    .as_array()
                    .and_then(|cases| {
                        cases
                            .iter()
                            .find(|case| case["event"] == format!("response.{name}"))
                    })
                    .expect("variant terminal expectation");
                assert_eq!(
                    serde_json::to_value(terminal.finish_reason).expect("finish reason"),
                    expected["finish_reason"],
                    "{name} finish reason"
                );
                assert_eq!(
                    terminal.incomplete_reason.as_deref(),
                    expected.get("reason").and_then(Value::as_str),
                    "{name} structured incomplete reason"
                );
                assert_eq!(
                    terminal.provider_error.as_ref(),
                    expected.get("error"),
                    "{name} structured provider error"
                );
            }
        }
        if dialect == DialectId::DeepseekResponsesV1 {
            let raw = profile["content_filter_stream_bytes"]
                .as_str()
                .expect("content-filter stream");
            let (_, terminal) = normalize_dialect_sse_stream(dialect, raw.as_bytes())
                .expect("content-filter production decode");
            let expected = &profile["terminal"]["content_filter"];
            assert_eq!(
                terminal.finish_reason,
                provider::FinishReason::ContentFilter
            );
            assert_eq!(
                terminal.incomplete_reason.as_deref(),
                expected["reason"].as_str()
            );
            assert!(terminal.provider_error.is_none());
        }
        let tool_stream = profile["tool_stream_bytes"]
            .as_str()
            .expect("tool stream bytes");
        let (_, tool_terminal) = normalize_dialect_sse_stream(dialect, tool_stream.as_bytes())
            .unwrap_or_else(|error| {
                panic!(
                    "{} production decoder failed tool stream: {error}",
                    dialect.as_str()
                )
            });
        assert_eq!(
            tool_terminal_summary(&tool_terminal),
            profile["tool_terminal"],
            "{} tool terminal normalization",
            dialect.as_str()
        );
        if dialect == DialectId::DeepseekResponsesV1 {
            let target: ProviderTarget =
                serde_json::from_value(profile["target"].clone()).expect("target");
            let reasoning_sealed = ijson(&json!({
                "adapter":dialect.as_str(),
                "fragments":terminal.sealed_fragments,
                "role":"sealed"
            }));
            let reasoning_roundtrip = prepare_fixture(
                profile,
                target.clone(),
                ijson(&profile["control"]["epoch_profile"]),
                vec![reasoning_sealed, text("user", "again")],
                ijson(&json!([])),
            )
            .expect("DeepSeek reasoning carrier roundtrip");
            assert_eq!(
                reasoning_roundtrip.body,
                profile["sealed_reasoning_request_bytes"]
                    .as_str()
                    .expect("reasoning roundtrip bytes")
                    .as_bytes()
            );

            let tool_sealed = ijson(&json!({
                "adapter":dialect.as_str(),
                "fragments":tool_terminal.sealed_fragments,
                "role":"sealed"
            }));
            let tool_result = block_item(
                json!({"call_id":"call-1","name":"lookup","result":{"ok":true},"type":"tool_result"}),
                "tool",
            );
            let tool_roundtrip = prepare_fixture(
                profile,
                target,
                ijson(&profile["control"]["epoch_profile"]),
                vec![tool_sealed, tool_result, text("user", "again")],
                tool_catalog(),
            )
            .expect("DeepSeek tool carrier roundtrip");
            assert_eq!(
                tool_roundtrip.body,
                profile["sealed_tool_request_bytes"]
                    .as_str()
                    .expect("tool roundtrip bytes")
                    .as_bytes()
            );
        }
        let repair = &profile["capabilities"]["repair"];
        if repair["id"] == "deepseek_v4_arguments_v1" {
            for case in repair["cases"].as_array().expect("repair cases") {
                let result = repair_tool_arguments(
                    dialect,
                    case["input"].as_str().expect("repair input").as_bytes(),
                );
                if case["reject"] == true {
                    assert!(result.is_err(), "repair negative unexpectedly succeeded");
                } else {
                    assert_eq!(result.expect("bounded provider repair"), case["output"]);
                }
            }
            assert!(repair_tool_arguments(dialect, &vec![b'x'; 65_537]).is_err());
        } else {
            assert_eq!(repair["id"], "none");
        }
    }
}

#[test]
fn provider_dialect_gate_95_recovery_and_mismatch() {
    let value = corpus();
    for profile in value["profiles"].as_array().expect("profiles") {
        assert_eq!(profile["mismatch"]["result"], "dialect_mismatch");
        let continuation = profile["target"]["dialect_id"]
            .as_str()
            .is_some_and(|id| matches!(id, "openai_responses_v1" | "google_interactions_v1"));
        let expected = if continuation {
            "close_old_attempt_then_continue"
        } else {
            "close_old_attempt_then_replay"
        };
        assert_eq!(profile["recovery"]["after_dispatch"], expected);
        let target: ProviderTarget =
            serde_json::from_value(profile["target"].clone()).expect("target");
        let dialect = target.dialect().expect("dialect");
        let capabilities = AdapterCapabilities {
            continuation: if dialect.server_managed() {
                Continuation::ServerManaged
            } else {
                Continuation::Stateless
            },
            query_by_identity: QueryCapability::None,
            dispatch_marker_required: true,
        };
        assert_eq!(
            decide_recovery(
                capabilities,
                SentState::NotDispatched,
                QueryResult::Unsupported,
                RunMode::Ordinary,
            ),
            RecoveryDecision::NotDispatched
        );
        assert_eq!(
            decide_recovery(
                capabilities,
                SentState::MaybeSent,
                QueryResult::Outcome,
                RunMode::Ordinary,
            ),
            RecoveryDecision::Adopt
        );
        assert_eq!(
            decide_recovery(
                capabilities,
                SentState::MaybeSent,
                QueryResult::Unsupported,
                RunMode::Ordinary,
            ),
            if dialect.server_managed() {
                RecoveryDecision::RecoveryEpoch
            } else {
                RecoveryDecision::Resend
            }
        );
        assert_eq!(
            decide_recovery(
                capabilities,
                SentState::MaybeSent,
                QueryResult::TransientExhausted,
                RunMode::Reconcile,
            ),
            RecoveryDecision::Unresolved
        );
        if !dialect.server_managed() {
            let attempted = prepare(&PrepareInput {
                attempt_id: "illegal-continuation".to_owned(),
                target,
                endpoint: endpoint(profile).to_owned(),
                epoch_profile: ijson(&profile["control"]["epoch_profile"]),
                continuation_id: Some("foreign".to_owned()),
                rendered_items: vec![text("user", "again")],
                tool_catalog: ijson(&json!([])),
                stream: true,
            });
            assert!(
                attempted.is_err(),
                "stateless dialect accepted continuation identity"
            );
        }
    }
    let deepseek = value["profiles"]
        .as_array()
        .expect("profiles")
        .iter()
        .find(|profile| profile["target"]["dialect_id"] == "deepseek_responses_v1")
        .expect("DeepSeek Responses");
    for field in ["request_bytes", "multiturn_request_bytes"] {
        let body: Value =
            serde_json::from_str(deepseek[field].as_str().expect("body")).expect("request JSON");
        assert!(body.get("previous_response_id").is_none());
        assert!(body.get("conversation").is_none());
        assert!(body.get("store").is_none());
    }

    for stream in [
        concat!(
            "data: {\"response\":{\"id\":\"r\",\"status\":\"completed\"},\"type\":\"response.done\"}\n\n"
        ),
        concat!(
            "data: {\"type\":\"response.reasoning_summary_text.done\"}\n\n",
            "data: {\"response\":{\"id\":\"r\",\"status\":\"completed\"},\"type\":\"response.completed\"}\n\n"
        ),
    ] {
        assert!(
            normalize_dialect_sse_stream(DialectId::DeepseekResponsesV1, stream.as_bytes()).is_ok(),
            "DeepSeek decoder rejected a structurally valid Responses stream"
        );
    }
    assert!(
        normalize_dialect_sse_stream(
            DialectId::DeepseekResponsesV1,
            b"data: {\"response\":{\"error\":{\"code\":\"x\"},\"id\":\"r\",\"status\":\"completed\"},\"type\":\"response.failed\"}\n\n",
        )
        .is_err(),
        "DeepSeek decoder accepted an inconsistent terminal status"
    );
    let terminal = normalize_dialect_response(
        DialectId::DeepseekResponsesV1,
        br#"{"id":"r","output":[{"summary":[{"text":"reason","type":"summary_text"}],"type":"reasoning"}],"status":"completed"}"#,
    )
    .expect("well-formed provider-native reasoning item");
    assert_eq!(
        terminal.sealed_fragments,
        json!([{"summary":[{"text":"reason","type":"summary_text"}],"type":"reasoning"}])
    );
    for block_type in ["thinking", "image", "document"] {
        let response = serde_json_canonicalizer::to_vec(&json!({
            "content":[{"text":"unproved","type":block_type}],
            "id":"m","stop_reason":"end_turn"
        }))
        .expect("Anthropic negative response");
        let terminal = normalize_dialect_response(DialectId::AnthropicMessagesV1, &response)
            .expect("well-formed provider-native Anthropic block");
        assert_eq!(
            terminal.sealed_fragments,
            json!([{"text":"unproved","type":block_type}]),
            "Anthropic decoder did not preserve {block_type}"
        );
    }
}

#[test]
fn provider_dialect_forward_compatible_events_preserve_native_replay() {
    let registry = corpus();
    let forward = forward_compatible_corpus();
    for case in forward["cases"]
        .as_array()
        .expect("forward-compatible cases")
    {
        let dialect: DialectId =
            serde_json::from_value(case["dialect"].clone()).expect("known dialect");
        let (_, terminal) = normalize_dialect_sse_stream(
            dialect,
            case["stream"].as_str().expect("stream bytes").as_bytes(),
        )
        .unwrap_or_else(|error| panic!("{} rejected: {error}", case["id"]));
        assert_eq!(
            terminal.sealed_fragments, case["sealed"],
            "{} did not retain its provider-native output",
            case["id"]
        );

        let profile = registry["profiles"]
            .as_array()
            .expect("profiles")
            .iter()
            .find(|profile| profile["target"]["dialect_id"] == case["dialect"])
            .expect("fixture profile");
        let target: ProviderTarget =
            serde_json::from_value(profile["target"].clone()).expect("target");
        let sealed = ijson(&json!({
            "adapter": dialect.as_str(),
            "fragments": terminal.sealed_fragments,
            "role": "sealed"
        }));
        let prepared = prepare_fixture(
            profile,
            target,
            ijson(&profile["control"]["epoch_profile"]),
            vec![sealed, text("user", "again")],
            ijson(&json!([])),
        )
        .unwrap_or_else(|error| panic!("{} replay rejected: {error}", case["id"]));
        let body: Value = serde_json::from_slice(&prepared.body).expect("request JSON");
        match dialect {
            DialectId::DeepseekResponsesV1 | DialectId::GoogleInteractionsV1 => {
                let input = body["input"].as_array().expect("native input array");
                let expected = case["sealed"].as_array().expect("sealed array");
                assert_eq!(&input[..expected.len()], expected.as_slice());
            }
            DialectId::AnthropicMessagesV1 => {
                assert_eq!(body.pointer("/messages/0/content"), Some(&case["sealed"]));
            }
            DialectId::GenericChatV1 => {
                assert!(
                    body["messages"]
                        .as_array()
                        .expect("chat messages")
                        .contains(&case["sealed"]),
                    "chat request did not replay its native assistant message"
                );
            }
            DialectId::GoogleGenerationV1 => {
                assert_eq!(body.pointer("/contents/0"), Some(&case["sealed"]));
            }
            other => panic!("unexpected forward-compatible fixture {other:?}"),
        }
    }
    assert!(
        normalize_dialect_sse_stream(
            DialectId::OpenaiResponsesV1,
            b"data: {\"future\":true}\n\ndata: {\"response\":{\"id\":\"r\",\"status\":\"completed\"},\"type\":\"response.completed\"}\n\n",
        )
        .is_err(),
        "an event without a discriminator is not structurally valid"
    );
    assert!(
        normalize_dialect_response(
            DialectId::DeepseekResponsesV1,
            br#"{"id":"r","output":[{"future":true}],"status":"completed"}"#,
        )
        .is_err(),
        "an output item without a discriminator is not structurally valid"
    );
    assert!(
        normalize_dialect_response(
            DialectId::AnthropicMessagesV1,
            br#"{"content":[{"future":true}],"id":"m","stop_reason":"end_turn"}"#,
        )
        .is_err(),
        "an Anthropic block without a discriminator is not structurally valid"
    );
}

#[test]
fn provider_dialect_gate_96_session_controls_to_wire() {
    let value = corpus();
    for fixture in value["profiles"].as_array().expect("profiles") {
        let target: ProviderTarget =
            serde_json::from_value(fixture["target"].clone()).expect("target");
        let provider = Provider {
            id: "fixture".to_owned(),
            name: None,
            adapter: target.protocol_family.clone(),
            dialect: target.dialect_id.clone(),
            endpoint_owner: target.route.endpoint_owner.clone(),
            gateway_translation: target.route.gateway_translation.clone(),
            evidence_revision: target.route.evidence_revision.clone(),
            endpoint: endpoint(fixture).to_owned(),
            credential_key: None,
            models: Vec::new(),
        };
        let model = Model {
            id: target.route.exact_sku.clone(),
            profile: target.model_profile_id.clone(),
            enabled: true,
            context_window_tokens: 100_000,
            compact_trigger_tokens: 80_000,
        };
        let resolved = resolve_profile(&provider, &model).expect("configured target");
        let dialect = resolved.dialect;
        let requested = fixture["control"]["session"]["reasoning_effort"]
            .as_str()
            .map(str::to_owned);
        let settings = SessionSettings {
            format: 1,
            revision: 1,
            provider: provider.id.clone(),
            model: model.id.clone(),
            reasoning_effort: requested,
        };
        let epoch = epoch_profile(&resolved, "system", Some(&settings))
            .expect("session control validated into epoch");
        assert_eq!(
            epoch.canonical_bytes().expect("epoch bytes"),
            serde_json_canonicalizer::to_vec(&fixture["control"]["epoch_profile"])
                .expect("fixture epoch bytes")
        );
        assert_eq!(
            fixture["control"]["profile_digest"]
                .as_str()
                .expect("profile digest"),
            format!(
                "{:x}",
                Sha256::digest(epoch.canonical_bytes().expect("epoch bytes"))
            )
        );
        let prepared = prepare(&PrepareInput {
            attempt_id: "session-control".to_owned(),
            target,
            endpoint: provider.endpoint.clone(),
            epoch_profile: epoch,
            continuation_id: None,
            rendered_items: vec![text("user", "hello")],
            tool_catalog: ijson(&json!([])),
            stream: true,
        })
        .expect("real provider projector accepts session-derived epoch");
        assert_eq!(
            prepared.request_digest,
            fixture["control"]["request_digest"]
                .as_str()
                .expect("request digest")
        );

        let levels = fixture["capabilities"]["reasoning"]["levels"]
            .as_array()
            .expect("reasoning levels");
        for level in levels {
            let effort = level.as_str().expect("effort string");
            let settings = SessionSettings {
                format: 1,
                revision: 2,
                provider: provider.id.clone(),
                model: model.id.clone(),
                reasoning_effort: Some(effort.to_owned()),
            };
            let epoch = epoch_profile(&resolved, "system", Some(&settings))
                .expect("proved reasoning level enters epoch");
            let request = prepare(&PrepareInput {
                attempt_id: format!("control-{effort}"),
                target: resolved.target.clone(),
                endpoint: provider.endpoint.clone(),
                epoch_profile: epoch,
                continuation_id: None,
                rendered_items: vec![text("user", "hello")],
                tool_catalog: ijson(&json!([])),
                stream: true,
            })
            .expect("proved reasoning level reaches projector");
            assert_reasoning_wire(dialect, effort, &request.body);
        }
        let unsupported = SessionSettings {
            format: 1,
            revision: 3,
            provider: provider.id.clone(),
            model: model.id.clone(),
            reasoning_effort: Some("unproved".to_owned()),
        };
        assert!(
            epoch_profile(&resolved, "system", Some(&unsupported)).is_err(),
            "unproved reasoning level entered an epoch"
        );
    }
}

#[test]
fn explicit_tool_choice_is_dialect_encoded_and_digest_bound() {
    use provider::{AdapterId, ToolChoice, prepare_with_tool_choice};
    for profile in corpus()["profiles"].as_array().unwrap() {
        let target: ProviderTarget = serde_json::from_value(profile["target"].clone()).unwrap();
        let adapter = target.dialect().unwrap().family();
        let mut input = PrepareInput {
            attempt_id: "tool-choice".into(),
            target,
            endpoint: endpoint(profile).into(),
            epoch_profile: ijson(&profile["control"]["epoch_profile"]),
            continuation_id: None,
            rendered_items: vec![text("user", "Use lookup")],
            tool_catalog: tool_catalog(),
            stream: true,
        };
        let ordinary = prepare(&input).unwrap();
        assert_eq!(ordinary, prepare_with_tool_choice(&input, None).unwrap());
        let mut digests = BTreeSet::new();
        for choice in [ToolChoice::Auto, ToolChoice::Required, ToolChoice::None] {
            let result = prepare_with_tool_choice(&input, Some(choice));
            if adapter == AdapterId::GoogleInteractions {
                assert!(result.is_err());
                continue;
            }
            let request = result.unwrap();
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            let label = match choice {
                ToolChoice::Auto => "auto",
                ToolChoice::Required => "required",
                ToolChoice::None => "none",
            };
            match adapter {
                AdapterId::Responses | AdapterId::ChatCompletion => {
                    assert_eq!(body["tool_choice"], label)
                }
                AdapterId::Anthropic => assert_eq!(
                    body["tool_choice"]["type"],
                    if choice == ToolChoice::Required {
                        "any"
                    } else {
                        label
                    }
                ),
                AdapterId::GoogleGeneration => assert_eq!(
                    body["toolConfig"]["functionCallingConfig"]["mode"],
                    if choice == ToolChoice::Required {
                        "ANY".to_owned()
                    } else {
                        label.to_uppercase()
                    }
                ),
                _ => unreachable!(),
            }
            assert_ne!(request.request_digest, ordinary.request_digest);
            digests.insert(request.request_digest);
        }
        if adapter != AdapterId::GoogleInteractions {
            assert_eq!(digests.len(), 3);
        }
        input.tool_catalog = ijson(&json!([]));
        assert!(prepare_with_tool_choice(&input, Some(ToolChoice::Required)).is_err());
    }
}

#[test]
fn deepseek_responses_ready_and_terminal_share_argument_repair() {
    for raw in [
        r#"{"failure_policy": continue, "program":"bash", "command":null}"#,
        r#"{"program":"bash","program":"sh"}"#,
    ] {
        let item = json!({"type":"function_call","id":"item-1","call_id":"call-1","name":"shell","arguments":raw});
        let prefix = [
            json!({"type":"response.output_item.added","output_index":0,"item":{"type":"function_call","id":"item-1","call_id":"call-1","name":"shell","arguments":""}}),
            json!({"type":"response.function_call_arguments.done","output_index":0,"arguments":raw}),
        ].into_iter().fold(String::new(), |mut sse, event| { sse.push_str(&format!("data: {event}\n\n")); sse });
        let mut decoder =
            provider::ProviderStreamDecoder::for_dialect(DialectId::DeepseekResponsesV1);
        let frames = decoder.push(prefix.as_bytes()).unwrap();
        let ready = frames
            .into_iter()
            .find_map(|frame| match frame {
                provider::ProviderFrame::ToolCallReady(call) => Some(call),
                _ => None,
            })
            .expect("call must be ready before terminal exists");
        let terminal_event = json!({"type":"response.completed","response":{"id":"r1","status":"completed","output":[item]}});
        decoder
            .push(format!("data: {terminal_event}\n\n").as_bytes())
            .unwrap();
        let (_, terminal) = decoder.finish().unwrap();
        assert_eq!(
            ready, terminal.tool_calls[0],
            "readiness and terminal must use the same dialect repair: {raw}"
        );
        if raw.contains("continue") {
            assert_eq!(ready.arguments["failure_policy"], "continue");
        } else {
            assert!(provider::invalid_arguments_detail(&ready.arguments).is_some());
        }
    }
}

/// A transport loss mid-stream keeps the native items the decoder completed:
/// the closed `reasoning` item (with its encrypted content) and the call whose
/// arguments completed, carrying the same resolved arguments the terminal
/// would, in output order. Nothing is reported before an item completes.
#[test]
fn deepseek_stream_decoder_reports_completed_items_for_a_lost_response() {
    let mut decoder = provider::ProviderStreamDecoder::for_dialect(DialectId::DeepseekResponsesV1);
    let sse = |event: Value| format!("data: {event}\n\n");
    decoder.push(sse(json!({"type":"response.output_item.added","output_index":0,"item":{"type":"reasoning","id":"rs-1","summary":[],"content":[],"status":"in_progress"}})).as_bytes()).unwrap();
    decoder.push(sse(json!({"type":"response.reasoning_text.delta","output_index":0,"content_index":0,"delta":"think"})).as_bytes()).unwrap();
    assert_eq!(
        decoder.completed_fragments(),
        None,
        "an open item is not a completed fragment"
    );
    let reasoning = json!({"type":"reasoning","id":"rs-1","summary":[],"content":[{"type":"reasoning_text","text":"think"}],"encrypted_content":"enc-1","status":"completed"});
    decoder
        .push(
            sse(json!({"type":"response.output_item.done","output_index":0,"item":reasoning}))
                .as_bytes(),
        )
        .unwrap();
    assert_eq!(decoder.completed_fragments(), Some(json!([reasoning])));
    decoder.push(sse(json!({"type":"response.output_item.added","output_index":1,"item":{"type":"function_call","id":"fc-1","call_id":"call-1","name":"shell","arguments":""}})).as_bytes()).unwrap();
    decoder.push(sse(json!({"type":"response.function_call_arguments.delta","output_index":1,"delta":"{\"command\":"})).as_bytes()).unwrap();
    assert_eq!(
        decoder.completed_fragments(),
        Some(json!([reasoning])),
        "a call with open arguments is not replayable"
    );
    decoder.push(sse(json!({"type":"response.function_call_arguments.delta","output_index":1,"delta":"\"ls\"}"})).as_bytes()).unwrap();
    let frames = decoder.push(sse(json!({"type":"response.function_call_arguments.done","output_index":1,"arguments":"{\"command\":\"ls\"}"})).as_bytes()).unwrap();
    let ready = frames
        .into_iter()
        .find_map(|frame| match frame {
            provider::ProviderFrame::ToolCallReady(call) => Some(call),
            _ => None,
        })
        .expect("the call is ready for eager dispatch");
    let partial = decoder.completed_fragments().expect("completed items");
    let items = partial.as_array().unwrap();
    assert_eq!(items[0], reasoning);
    assert_eq!(items[1]["type"], "function_call");
    assert_eq!(items[1]["call_id"], ready.call_id);
    assert_eq!(items[1]["name"], "shell");
    assert_eq!(
        items[1]["arguments"],
        serde_json_canonicalizer::to_string(&ready.arguments).unwrap()
    );
    assert_eq!(items.len(), 2);
}

#[test]
fn glm_exact_output_ceiling_preserves_max_reasoning_and_append_prefix() {
    let fixtures = corpus();
    let mut profile = fixtures["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["target"]["dialect_id"] == "glm_chat_v1")
        .unwrap()
        .clone();
    profile["target"]["model_profile_id"] = json!("glm_chat_v1:glm-5.3");
    profile["target"]["route"]["exact_sku"] = json!("glm-5.3");
    profile["control"]["epoch_profile"]["target"] = profile["target"].clone();
    profile["control"]["epoch_profile"]["controls"]["reasoning_effort"] = json!("max");
    let first: Value = serde_json::from_slice(&prepared(&profile, false).body).unwrap();
    let next: Value = serde_json::from_slice(&prepared(&profile, true).body).unwrap();
    for body in [&first, &next] {
        assert_eq!(body["max_tokens"], 131072);
        assert_eq!(body["reasoning_effort"], "max");
        assert_eq!(body["model"], "glm-5.3");
    }
    let prefix = first["messages"].as_array().unwrap();
    assert!(next["messages"].as_array().unwrap().starts_with(prefix));
}

#[test]
fn sol_pro_local_variant_renders_base_model_and_independent_reasoning_mode() {
    let fixtures = corpus();
    let template = fixtures["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["target"]["dialect_id"] == "openai_responses_v1")
        .unwrap();
    for (sku, pro) in [
        ("openai/gpt-5.6-sol", false),
        ("openai/gpt-5.6-sol@cloudflare-gpt-5.6-sol-pro", true),
    ] {
        for effort in [None, Some("medium"), Some("max")] {
            let mut profile = template.clone();
            profile["target"]["model_profile_id"] = json!("openai_responses_v1:openai/gpt-5.6-sol");
            profile["target"]["route"]["exact_sku"] = json!(sku);
            profile["control"]["epoch_profile"]["target"] = profile["target"].clone();
            let controls = profile["control"]["epoch_profile"]["controls"]
                .as_object_mut()
                .unwrap();
            controls.remove("reasoning_effort");
            if let Some(effort) = effort {
                controls.insert("reasoning_effort".into(), json!(effort));
            }
            for continued in [false, true] {
                let body: Value =
                    serde_json::from_slice(&prepared(&profile, continued).body).unwrap();
                assert_eq!(body["model"], "openai/gpt-5.6-sol");
                assert_eq!(
                    body["reasoning"]["mode"],
                    if pro { json!("pro") } else { Value::Null }
                );
                assert_eq!(body["reasoning"]["effort"], json!(effort));
            }
        }
    }
}

#[test]
fn reasoning_disabled_has_a_verified_off_switch_or_fails_closed() {
    let mut covered = BTreeSet::new();
    for profile in corpus()["profiles"].as_array().expect("profiles") {
        let target: ProviderTarget =
            serde_json::from_value(profile["target"].clone()).expect("target");
        let resolved = validate_target(&target).expect("fixture target");
        let request = |controls: Value| {
            prepare(&PrepareInput {
                attempt_id: "reasoning-disabled".to_owned(),
                target: target.clone(),
                endpoint: endpoint(profile).to_owned(),
                epoch_profile: ijson(&json!({
                    "controls": controls,
                    "serializer_revision": resolved.serializer_revision,
                    "system": "system",
                    "target": resolved.target,
                })),
                continuation_id: None,
                rendered_items: vec![text("user", "hello")],
                tool_catalog: ijson(&json!([])),
                stream: false,
            })
        };
        let disabled = request(json!({"reasoning_disabled": true}));
        assert!(
            request(json!({"reasoning_disabled": true, "reasoning_effort": "high"})).is_err(),
            "disabled reasoning combined with an effort must be rejected"
        );
        assert!(request(json!({"reasoning_disabled": "yes"})).is_err());
        match resolved.dialect {
            DialectId::DeepseekResponsesV1 => {
                let body: Value =
                    serde_json::from_slice(&disabled.expect("prepared").body).unwrap();
                assert_eq!(body["reasoning"], json!({"effort": "none"}));
                covered.insert(resolved.dialect);
            }
            DialectId::DeepseekChatV1 => {
                let body: Value =
                    serde_json::from_slice(&disabled.expect("prepared").body).unwrap();
                assert_eq!(body["thinking"], json!({"type": "disabled"}));
                assert!(body.get("reasoning_effort").is_none());
                covered.insert(resolved.dialect);
            }
            _ => assert!(
                disabled.is_err(),
                "{} has no verified off switch",
                resolved.dialect.as_str()
            ),
        }
    }
    assert_eq!(covered.len(), 2, "both DeepSeek dialects are in the corpus");
}

#[test]
fn title_output_cap_renders_on_deepseek_dialects_and_fails_closed_elsewhere() {
    let mut covered = BTreeSet::new();
    for profile in corpus()["profiles"].as_array().expect("profiles") {
        let target: ProviderTarget =
            serde_json::from_value(profile["target"].clone()).expect("target");
        let resolved = validate_target(&target).expect("fixture target");
        let request = |controls: Value| {
            prepare(&PrepareInput {
                attempt_id: "title-output-cap".to_owned(),
                target: target.clone(),
                endpoint: endpoint(profile).to_owned(),
                epoch_profile: ijson(&json!({
                    "controls": controls,
                    "serializer_revision": resolved.serializer_revision,
                    "system": "system",
                    "target": resolved.target,
                })),
                continuation_id: None,
                rendered_items: vec![text("user", "hello")],
                tool_catalog: ijson(&json!([])),
                stream: false,
            })
        };
        assert!(request(json!({"title_max_output_tokens": 0})).is_err());
        assert!(request(json!({"title_max_output_tokens": "64"})).is_err());
        let capped = request(json!({"title_max_output_tokens": 64, "reasoning_disabled": true}));
        match resolved.dialect {
            DialectId::DeepseekResponsesV1 => {
                let body: Value = serde_json::from_slice(&capped.expect("prepared").body).unwrap();
                assert_eq!(body["max_output_tokens"], json!(64));
                covered.insert(resolved.dialect);
            }
            DialectId::DeepseekChatV1 => {
                let body: Value = serde_json::from_slice(&capped.expect("prepared").body).unwrap();
                assert_eq!(body["max_tokens"], json!(64));
                covered.insert(resolved.dialect);
            }
            _ => assert!(
                capped.is_err(),
                "{} accepted an unverified title output cap",
                resolved.dialect.as_str()
            ),
        }
    }
    assert_eq!(covered.len(), 2, "both DeepSeek dialects are in the corpus");
}
