use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use provider::{
    AdapterId, BrokerDecision, BrokerError, CredentialBroker, CredentialDecoder, CredentialGet,
    CredentialMessage, CredentialScope, DialectId, FinishReason, HttpRuntime, HttpStatusClass,
    PrepareInput, ProviderCompletion, ProviderFailure, ProviderFrame, ProviderTarget,
    ProviderTerminal, RouteEvidence, SecretRecord, SseDecoder, classify_status,
    credential_request_id, decode_credential_frame, encode_credential_frame, normalize_response,
    normalize_sse_stream, prepare, provider_query_key, provider_request_digest,
};
use schema::IJsonValue;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use test_support::FixtureRoot;

#[derive(Deserialize)]
struct SourceLock {
    files: Vec<SourceFile>,
}

#[derive(Deserialize)]
struct SourceFile {
    path: String,
    sha256: String,
}

fn fixtures() -> FixtureRoot {
    FixtureRoot::discover().expect("fixture root")
}

fn read(relative: &str) -> Vec<u8> {
    fs::read(fixtures().join(relative)).expect("fixture bytes")
}

fn ijson(value: serde_json::Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(&value).expect("JSON")).expect("I-JSON")
}

fn terminal_semantics(terminal: &ProviderTerminal) -> serde_json::Value {
    json!({
        "content": terminal.content,
        "finish_reason": terminal.finish_reason,
        "tool_calls": terminal.tool_calls,
        "usage": terminal.usage,
    })
}

fn serve_once(response_parts: Vec<Vec<u8>>) -> (String, std::thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind fixture server");
    let address = listener.local_addr().expect("fixture address");
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept fixture request");
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .expect("read timeout");
        let mut request = Vec::new();
        let mut chunk = [0_u8; 4096];
        let mut expected = None;
        loop {
            let count = stream.read(&mut chunk).expect("read fixture request");
            if count == 0 {
                break;
            }
            request.extend_from_slice(&chunk[..count]);
            if expected.is_none() {
                if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end + 4]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            line.strip_prefix("content-length: ")
                                .or_else(|| line.strip_prefix("Content-Length: "))
                        })
                        .and_then(|value| value.parse::<usize>().ok())
                        .unwrap_or(0);
                    expected = Some(end + 4 + length);
                }
            }
            if expected.is_some_and(|length| request.len() >= length) {
                break;
            }
        }
        for part in response_parts {
            stream.write_all(&part).expect("write fixture response");
            stream.flush().expect("flush fixture response");
        }
        request
    });
    (format!("http://{address}"), handle)
}

fn serve_delayed(delay: std::time::Duration) -> (String, std::thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind delayed server");
    let address = listener.local_addr().expect("delayed address");
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept delayed request");
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request);
        std::thread::sleep(delay);
        let _ = stream.write_all(
            b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}",
        );
    });
    (format!("http://{address}"), handle)
}

fn local_prepared(adapter: AdapterId, endpoint: String, stream: bool) -> provider::PreparedRequest {
    let target = exact_target(adapter);
    let exact_endpoint = match adapter {
        AdapterId::Responses | AdapterId::ChatCompletion => "https://api.openai.com/v1",
        AdapterId::Anthropic => "https://api.anthropic.com/v1",
        AdapterId::GoogleGeneration | AdapterId::GoogleInteractions => {
            "https://generativelanguage.googleapis.com/v1beta"
        }
    };
    let mut prepared = prepare(&PrepareInput {
        attempt_id: "attempt-http".to_owned(),
        target: target.clone(),
        endpoint: exact_endpoint.to_owned(),
        epoch_profile: ijson(json!({
            "controls":if matches!(adapter, AdapterId::Anthropic | AdapterId::ChatCompletion) { json!({}) } else { json!({"reasoning_effort":"high"}) },
            "serializer_revision":provider::validate_target(&target).expect("exact target").serializer_revision,
            "system":"","target":target
        })),
        continuation_id: None,
        rendered_items: vec![ijson(json!({
            "role":"user",
            "content":[{"type":"text","text":"hello"}]
        }))],
        tool_catalog: ijson(json!([])),
        stream,
    })
    .expect("prepare local request");
    let suffix = prepared
        .url
        .strip_prefix(exact_endpoint)
        .expect("exact endpoint prefix");
    prepared.url = format!("{}{suffix}", endpoint.trim_end_matches('/'));
    prepared
}

fn exact_target(adapter: AdapterId) -> ProviderTarget {
    let (dialect, owner, sku, profile, revision) = match adapter {
        AdapterId::Responses => (
            DialectId::OpenaiResponsesV1,
            "openai",
            "gpt-5",
            "openai_responses_v1:gpt-5",
            "openai-2026-08-01",
        ),
        AdapterId::Anthropic => (
            DialectId::AnthropicMessagesV1,
            "anthropic",
            "claude-sonnet-4-20250514",
            "anthropic_messages_v1:claude-sonnet-4-20250514",
            "anthropic-2026-08-01",
        ),
        AdapterId::GoogleGeneration => (
            DialectId::GoogleGenerationV1,
            "google",
            "gemini-2.5-flash",
            "google_generation_v1:gemini-2.5-flash",
            "google-generation-2026-06-22",
        ),
        AdapterId::GoogleInteractions => (
            DialectId::GoogleInteractionsV1,
            "google",
            "gemini-2.5-flash",
            "google_interactions_v1:gemini-2.5-flash",
            "google-interactions-2026-06-22",
        ),
        AdapterId::ChatCompletion => (
            DialectId::OpenaiChatV1,
            "openai",
            "gpt-4.1",
            "openai_chat_v1:gpt-4.1",
            "openai-2026-08-01",
        ),
    };
    ProviderTarget {
        protocol_family: match adapter {
            AdapterId::ChatCompletion => "chat_completions",
            AdapterId::Anthropic => "anthropic_messages",
            _ => adapter.as_str(),
        }
        .to_owned(),
        dialect_id: dialect.as_str().to_owned(),
        model_profile_id: profile.to_owned(),
        route: RouteEvidence {
            endpoint_owner: owner.to_owned(),
            gateway_translation: "direct".to_owned(),
            exact_sku: sku.to_owned(),
            evidence_revision: revision.to_owned(),
        },
    }
}

#[test]
fn slice7_gate_53_provider_dialect_parity_oracle() {
    fixtures().verify_manifest().expect("manifest exact");
    let registry: serde_json::Value =
        serde_json::from_slice(&read("provider-runtime/cases.canonical.json"))
            .expect("case registry");
    let cases = registry
        .get("cases")
        .and_then(serde_json::Value::as_array)
        .expect("registry cases");
    assert_eq!(cases.len(), 5);
    for case in cases {
        for field in [
            "request",
            "response",
            "stream",
            "stream_expected",
            "expected",
        ] {
            let path = case
                .get(field)
                .and_then(serde_json::Value::as_str)
                .expect("case artifact path");
            assert!(
                fixtures().join("provider-runtime").join(path).is_file(),
                "missing declared artifact {path}"
            );
        }
    }

    let active_bytes = read("secret-store/active.canonical.json");
    let active = SecretRecord::decode(
        active_bytes
            .strip_suffix(b"\n")
            .expect("secret-store fixture LF"),
    )
    .expect("active SecretStore record");
    assert!(matches!(active, SecretRecord::Active { generation: 7, .. }));
    assert!(!format!("{active:?}").contains("fixture-secret-never-log"));
    let revoked_bytes = read("secret-store/revoked.canonical.json");
    assert_eq!(
        SecretRecord::decode(
            revoked_bytes
                .strip_suffix(b"\n")
                .expect("secret-store fixture LF")
        )
        .expect("revoked SecretStore record"),
        SecretRecord::Revoked { generation: 8 }
    );
    let negative: serde_json::Value =
        serde_json::from_slice(&read("provider-runtime/negative.canonical.json"))
            .expect("negative registry");
    let declared = negative["cases"]
        .as_array()
        .expect("negative cases")
        .iter()
        .map(|case| case["id"].as_str().expect("negative id"))
        .collect::<std::collections::BTreeSet<_>>();
    let required = registry["negative"]["required"]
        .as_array()
        .expect("required negatives")
        .iter()
        .map(|id| id.as_str().expect("required id"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(declared, required);
    let lock: SourceLock =
        serde_json::from_slice(&read("provider-runtime/source-lock.canonical.json"))
            .expect("source lock");
    assert_eq!(lock.files.len(), 30);
    for file in lock.files {
        let bytes = read(&format!("provider-runtime/{}", file.path));
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), file.sha256);
    }

    for (adapter, stem) in [
        (AdapterId::Responses, "openai_responses"),
        (AdapterId::Anthropic, "anthropic"),
        (AdapterId::GoogleGeneration, "google_generation"),
        (AdapterId::ChatCompletion, "chat_completion"),
    ] {
        let terminal = normalize_response(
            adapter,
            &read(&format!(
                "provider-runtime/predecessor/{stem}.response.json"
            )),
        )
        .expect("snapshot response normalizes");
        let expected_terminal = normalize_response(
            adapter,
            &read(&format!("provider-runtime/predecessor/{stem}.result.json")),
        )
        .expect("predecessor expected result normalizes");
        assert_eq!(
            terminal_semantics(&terminal),
            terminal_semantics(&expected_terminal),
            "{stem} response semantic parity"
        );
        if adapter.capabilities().server_managed {
            assert_eq!(
                terminal.response_identity, expected_terminal.response_identity,
                "{stem} server-managed response identity parity"
            );
        }
        assert!(!terminal.tool_calls.is_empty());
        assert_eq!(terminal.finish_reason, FinishReason::ToolCalls);
        let (_, stream_terminal) = normalize_sse_stream(
            adapter,
            &read(&format!("provider-runtime/predecessor/{stem}.stream.sse")),
        )
        .expect("snapshot stream normalizes");
        let expected_stream = normalize_response(
            adapter,
            &read(&format!(
                "provider-runtime/predecessor/{stem}.stream.result.json"
            )),
        )
        .expect("predecessor stream expected result normalizes");
        assert_eq!(
            terminal_semantics(&stream_terminal),
            terminal_semantics(&expected_stream),
            "{stem} stream semantic parity"
        );
        if adapter.capabilities().server_managed {
            assert_eq!(
                stream_terminal.response_identity, expected_stream.response_identity,
                "{stem} server-managed stream identity parity"
            );
        }
        if adapter != AdapterId::Anthropic {
            assert!(stream_terminal.response_identity.is_some());
        }
        assert!(stream_terminal.usage.is_some());
    }

    let interactions = normalize_response(
        AdapterId::GoogleInteractions,
        &read("provider-runtime/google-interactions.response.canonical.json"),
    )
    .expect("interactions response");
    assert_eq!(interactions.finish_reason, FinishReason::ToolCalls);
    let expected: serde_json::Value = serde_json::from_slice(&read(
        "provider-runtime/google-interactions.expected.canonical.json",
    ))
    .expect("interactions expected");
    assert_eq!(
        serde_json::to_value(&interactions).expect("terminal value")["tool_calls"],
        expected["tool_calls"]
    );
    assert_eq!(
        interactions
            .usage
            .as_ref()
            .and_then(|usage| usage.input_tokens.as_deref()),
        Some("11")
    );
    let (frames, interactions_stream) = normalize_sse_stream(
        AdapterId::GoogleInteractions,
        &read("provider-runtime/google-interactions.stream.sse"),
    )
    .expect("interactions stream");
    assert!(!frames.is_empty());
    assert_eq!(interactions_stream.finish_reason, FinishReason::ToolCalls);
    assert_eq!(interactions_stream.tool_calls[0].call_id, "call-stream");

    let official = AdapterId::Responses.capabilities();
    assert!(official.server_managed);
    assert!(!official.query_by_identity);
    let interactions = AdapterId::GoogleInteractions.capabilities();
    assert!(interactions.server_managed);
    assert!(!interactions.query_by_identity);
}

#[test]
fn slice7_gate_54_provider_stream_framing_bounds() {
    for (adapter, stem) in [
        (AdapterId::Responses, "openai_responses"),
        (AdapterId::Anthropic, "anthropic"),
        (AdapterId::GoogleGeneration, "google_generation"),
        (AdapterId::ChatCompletion, "chat_completion"),
    ] {
        let bytes = read(&format!("provider-runtime/predecessor/{stem}.stream.sse"));
        let expected = normalize_sse_stream(adapter, &bytes).expect("whole stream");
        for split in 0..=bytes.len() {
            let mut decoder = SseDecoder::new();
            let mut events = decoder.push(&bytes[..split]).expect("prefix");
            events.extend(decoder.push(&bytes[split..]).expect("suffix"));
            events.extend(decoder.finish_events().expect("complete stream"));
            assert!(!events.is_empty());
        }
        assert!(!expected.0.is_empty());
    }
    let mut decoder = SseDecoder::new();
    assert!(decoder.push(&vec![b'x'; 8 * 1024 * 1024 + 1]).is_err());

    let body = read("provider-runtime/google-interactions.stream.sse");
    let header = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    let midpoint = body.len() / 2;
    let (endpoint, server) = serve_once(vec![
        header,
        body[..midpoint].to_vec(),
        body[midpoint..].to_vec(),
    ]);
    let prepared = local_prepared(AdapterId::GoogleInteractions, endpoint, true);
    let mut frames = Vec::new();
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send_with_frames(
            AdapterId::GoogleInteractions,
            &prepared,
            "fixture-key",
            &Arc::new(AtomicBool::new(false)),
            |frame| {
                std::thread::sleep(std::time::Duration::from_millis(1));
                frames.push(frame);
                Ok(())
            },
        )
        .expect("stream send");
    assert!(
        matches!(completion, ProviderCompletion::Terminal(_)),
        "{completion:?}"
    );
    assert!(matches!(frames.first(), Some(ProviderFrame::Status(_))));
    assert!(!server.join().expect("join server").is_empty());
}

#[test]
fn slice7_gate_57_provider_terminal_normalization() {
    let duplicate = br#"{"choices":[{"message":{"tool_calls":[{"id":"c","function":{"name":"x","arguments":"{}"}},{"id":"c","function":{"name":"y","arguments":"{}"}}]},"finish_reason":"tool_calls"}]}"#;
    assert!(normalize_response(AdapterId::ChatCompletion, duplicate).is_err());
    let missing = br#"{"choices":[{"message":{"tool_calls":[{"function":{"name":"x","arguments":"{}"}}]},"finish_reason":"tool_calls"}]}"#;
    assert!(normalize_response(AdapterId::ChatCompletion, missing).is_err());
    let missing_google_id = br#"{"candidates":[{"content":{"parts":[{"functionCall":{"args":{},"name":"lookup"}}]},"finishReason":"STOP"}]}"#;
    assert!(normalize_response(AdapterId::GoogleGeneration, missing_google_id).is_err());
    let unknown_interaction = br#"{"id":"i","object":"interaction","status":"completed","steps":[{"type":"future_required"}]}"#;
    assert_eq!(
        normalize_response(AdapterId::GoogleInteractions, unknown_interaction)
            .expect("structurally valid future interaction step")
            .sealed_fragments,
        json!([{"type":"future_required"}])
    );
    let overflow = br#"{"error":{"code":"context_length_exceeded","message":"too long","type":"invalid_request_error"}}"#;
    assert_eq!(
        normalize_response(AdapterId::Responses, overflow)
            .expect("structured overflow")
            .finish_reason,
        FinishReason::ContextOverflow
    );
    // Model-emitted argument text that is not usable (a `$spill` collision, a
    // duplicate member) keeps the call and becomes the invalid-arguments
    // sentinel (provider-adapter rule 7); it is never a malformed response.
    let spill_collision = br#"{"choices":[{"message":{"tool_calls":[{"id":"c","function":{"name":"x","arguments":"{\"$spill\":{\"asset\":\"sha256-aa\",\"bytes\":1}}"}}]},"finish_reason":"tool_calls"}]}"#;
    let refused = normalize_response(AdapterId::ChatCompletion, spill_collision)
        .expect("refused call, not a malformed response");
    assert!(
        refused.tool_calls[0]
            .arguments
            .get(provider::INVALID_ARGUMENTS_KEY)
            .is_some(),
        "{:?}",
        refused.tool_calls[0].arguments
    );
    let duplicate_argument_key = br#"{"choices":[{"message":{"tool_calls":[{"id":"c","function":{"name":"x","arguments":"{\"x\":1,\"x\":2}"}}]},"finish_reason":"tool_calls"}]}"#;
    let refused = normalize_response(AdapterId::ChatCompletion, duplicate_argument_key)
        .expect("refused call, not a malformed response");
    assert!(
        provider::invalid_arguments_detail(&refused.tool_calls[0].arguments)
            .is_some_and(|detail| detail.contains("duplicate object member"))
    );
    let completed = b"data: {\"type\":\"response.output_text.delta\",\"delta\":\"ok\"}\n\ndata: {\"type\":\"response.completed\",\"response\":{\"id\":\"r-current\",\"status\":\"completed\",\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}}\n\n";
    let (_, completed) = normalize_sse_stream(AdapterId::Responses, completed)
        .expect("current Responses completion event");
    assert_eq!(completed.response_identity.as_deref(), Some("r-current"));
    assert_eq!(completed.finish_reason, FinishReason::Completed);
    let terminal = normalize_response(
        AdapterId::GoogleGeneration,
        &read("provider-runtime/predecessor/google_generation.error.response.json"),
    )
    .expect("blocked response");
    assert_eq!(terminal.finish_reason, FinishReason::ContentFilter);
}

#[test]
fn slice7_gate_55_provider_send_recovery_matrix() {
    let headers = BTreeMap::from([("content-type".to_owned(), "application/json".to_owned())]);
    let digest = provider_request_digest(
        AdapterId::Responses,
        "post",
        "https://fixture.invalid/v1/responses",
        "fixture-model",
        &headers,
        b"{}",
    )
    .expect("digest");
    assert_eq!(digest.len(), 64);
    assert_eq!(provider_query_key("a1"), provider_query_key("a1"));
    assert_ne!(provider_query_key("a1"), provider_query_key("a2"));

    use engine::{
        AdapterCapabilities, Continuation, QueryCapability, QueryResult, RecoveryDecision, RunMode,
        SentState, decide_recovery,
    };
    let stateless = AdapterCapabilities {
        continuation: Continuation::Stateless,
        query_by_identity: QueryCapability::None,
        dispatch_marker_required: true,
    };
    assert_eq!(
        decide_recovery(
            stateless,
            SentState::MaybeSent,
            QueryResult::Unsupported,
            RunMode::Ordinary
        ),
        RecoveryDecision::Resend
    );
    assert_eq!(
        decide_recovery(
            stateless,
            SentState::MaybeSent,
            QueryResult::Outcome,
            RunMode::Reconcile
        ),
        RecoveryDecision::Adopt
    );
}

#[test]
fn slice7_gate_56_provider_http_classification_and_secrets() {
    assert_eq!(classify_status(200), HttpStatusClass::Success);
    assert_eq!(classify_status(401), HttpStatusClass::AuthFailure);
    assert_eq!(classify_status(429), HttpStatusClass::RecoverableTransport);
    assert_eq!(
        classify_status(400),
        HttpStatusClass::ProviderTerminalCandidate
    );
    assert_eq!(ProviderFailure::Cancelled, ProviderFailure::Cancelled);

    let origin = "https://api.example:443";
    let request_id = credential_request_id("a1", "main", origin);
    let mut broker = CredentialBroker::new([CredentialScope {
        credential_id: "main".to_owned(),
        adapter: "responses".to_owned(),
        endpoint_origin: origin.to_owned(),
        purpose: "provider".to_owned(),
        generation: "g1".to_owned(),
        material: "fixture-secret-never-log".to_owned(),
    }]);
    let decision = broker
        .get(CredentialGet {
            request_id,
            attempt: "a1".to_owned(),
            credential_id: "main".to_owned(),
            purpose: "provider".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: origin.to_owned(),
        })
        .expect("credential decision");
    assert!(matches!(decision, BrokerDecision::Material(_)));
    let diagnostic = format!("{decision:?}");
    assert!(!diagnostic.contains("fixture-secret-never-log"));

    let terminal = br#"{"id":"r1","output":[{"content":[{"text":"ok","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":1,"output_tokens":1}}"#;
    let header = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        terminal.len()
    )
    .into_bytes();
    let (endpoint, server) = serve_once(vec![header, terminal.to_vec()]);
    let prepared = local_prepared(AdapterId::Responses, endpoint, false);
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &prepared,
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
        )
        .expect("HTTP send");
    assert!(
        matches!(completion, ProviderCompletion::Terminal(_)),
        "{completion:?}"
    );
    let request = String::from_utf8(server.join().expect("join server")).expect("request UTF-8");
    assert!(
        request
            .to_ascii_lowercase()
            .contains("authorization: bearer fixture-secret")
    );

    let redirect = b"HTTP/1.1 302 Found\r\nlocation: https://cross-origin.invalid/steal\r\ncontent-length: 0\r\nconnection: close\r\n\r\n".to_vec();
    let (endpoint, server) = serve_once(vec![redirect]);
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
        )
        .expect("redirect classification");
    assert!(matches!(
        completion,
        ProviderCompletion::Failure(ProviderFailure::Malformed { .. })
    ));
    server.join().expect("join redirect server");

    let leaked = br#"{"id":"r1","output":[{"content":[{"text":"fixture-secret","type":"output_text"}],"type":"message"}],"status":"completed"}"#;
    let header = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        leaked.len()
    )
    .into_bytes();
    let (endpoint, server) = serve_once(vec![header, leaked.to_vec()]);
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
        )
        .expect("secret scan");
    assert!(matches!(
        completion,
        ProviderCompletion::Failure(ProviderFailure::Malformed { .. })
    ));
    server.join().expect("join secret server");

    let auth_body = br#"{"error":"bad fixture-secret"}"#;
    let auth_header = format!(
        "HTTP/1.1 401 Unauthorized\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        auth_body.len()
    )
    .into_bytes();
    let (endpoint, server) = serve_once(vec![auth_header, auth_body.to_vec()]);
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
        )
        .expect("auth classification");
    match completion {
        ProviderCompletion::Failure(ProviderFailure::AuthFailure { status, detail }) => {
            assert_eq!(status, 401);
            let detail = detail.expect("auth failure detail");
            assert!(detail.contains("[redacted]"));
            assert!(!detail.contains("fixture-secret"));
            assert!(detail.len() <= 512);
        }
        other => panic!("expected auth failure, got {other:?}"),
    }
    server.join().expect("join auth server");

    let rate_body = br#"{"error":"capacity"}"#;
    let rate_header = format!(
        "HTTP/1.1 429 Too Many Requests\r\nretry-after: 7\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        rate_body.len()
    )
    .into_bytes();
    let (endpoint, server) = serve_once(vec![rate_header, rate_body.to_vec()]);
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "",
            &Arc::new(AtomicBool::new(false)),
        )
        .expect("rate limit classification");
    assert!(matches!(
        completion,
        ProviderCompletion::Failure(ProviderFailure::RateLimited {
            status: 429,
            retry_after_seconds: Some(7),
            ..
        })
    ));
    server.join().expect("join rate limit server");

    let registry: serde_json::Value =
        serde_json::from_slice(&read("credential-broker/cases.canonical.json"))
            .expect("credential registry");
    let cases = registry["cases"].as_array().expect("credential cases");
    assert_eq!(cases.len(), 10);
    for case in cases {
        for artifact in case["artifacts"].as_array().expect("case artifacts") {
            let path = artifact.as_str().expect("artifact path");
            assert!(
                fixtures().join("credential-broker").join(path).is_file(),
                "missing credential artifact {path}"
            );
        }
        let packets = case["artifacts"][0].as_str().expect("packet path");
        for line in String::from_utf8(read(&format!("credential-broker/{packets}")))
            .expect("packet UTF-8")
            .lines()
        {
            let value: serde_json::Value = serde_json::from_str(line).expect("packet JSON");
            let Some(key) = value.as_object().and_then(|object| object.keys().next()) else {
                panic!("packet must have one key");
            };
            if matches!(
                key.as_str(),
                "credential_get" | "credential" | "credential_error"
            ) {
                let message: CredentialMessage =
                    serde_json::from_value(value).expect("credential message shape");
                let frame = encode_credential_frame(&message).expect("credential frame");
                let decoded = decode_credential_frame(&frame).expect("credential decode");
                assert_eq!(decoded, message);
                assert!(!format!("{decoded:?}").contains("fixture-secret-never-log"));
            }
        }
    }

    let mut malformed = 8_u32.to_be_bytes().to_vec();
    malformed.extend_from_slice(b"not-json");
    assert!(decode_credential_frame(&malformed).is_err());
    let mut decoder = CredentialDecoder::new();
    assert!(decoder.push(&65_537_u32.to_be_bytes()).is_err());
    let mut decoder = CredentialDecoder::new();
    assert!(decoder.push(&[0, 0, 0]).expect("partial prefix").is_empty());
    assert!(decoder.finish().is_err());

    let conflict_id = credential_request_id("a1", "main", origin);
    let first = CredentialGet {
        request_id: conflict_id.clone(),
        attempt: "a1".to_owned(),
        credential_id: "main".to_owned(),
        purpose: "provider".to_owned(),
        adapter: "responses".to_owned(),
        endpoint_origin: origin.to_owned(),
    };
    broker.get(first.clone()).expect("first dedup request");
    let mut changed = first;
    changed.request_id = conflict_id;
    changed.adapter = "anthropic".to_owned();
    let conflict = broker.get(changed);
    assert_eq!(conflict, Err(BrokerError::RequestConflict));

    let (endpoint, server) = serve_delayed(std::time::Duration::from_secs(1));
    let started = std::time::Instant::now();
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send_with_frames_and_wall(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "",
            &Arc::new(AtomicBool::new(false)),
            Some(std::time::Duration::from_millis(50)),
            |_| Ok(()),
        )
        .expect("wall-bounded send");
    assert!(matches!(
        completion,
        ProviderCompletion::Failure(ProviderFailure::Transport { .. })
    ));
    assert!(started.elapsed() < std::time::Duration::from_millis(500));
    server.join().expect("join wall server");

    let (endpoint, server) = serve_delayed(std::time::Duration::from_secs(1));
    let cancelled = Arc::new(AtomicBool::new(false));
    let trigger = Arc::clone(&cancelled);
    let cancel_thread = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        trigger.store(true, std::sync::atomic::Ordering::Release);
    });
    let started = std::time::Instant::now();
    let completion = HttpRuntime::new()
        .expect("runtime")
        .send(
            AdapterId::Responses,
            &local_prepared(AdapterId::Responses, endpoint, false),
            "",
            &cancelled,
        )
        .expect("cancelled send");
    assert_eq!(
        completion,
        ProviderCompletion::Failure(ProviderFailure::Cancelled)
    );
    assert!(started.elapsed() < std::time::Duration::from_millis(500));
    cancel_thread.join().expect("cancel trigger");
    server.join().expect("join cancel server");
}

#[test]
fn slice7_gate_58_provider_overflow_compact_retry() {
    let input = PrepareInput {
        attempt_id: "a1".to_owned(),
        target: exact_target(AdapterId::Responses),
        endpoint: "https://api.openai.com/v1".to_owned(),
        epoch_profile: ijson(json!({
            "controls":{"reasoning_effort":"high"},
            "serializer_revision":"openai-responses-serializer-1",
            "system":"","target":exact_target(AdapterId::Responses)
        })),
        continuation_id: None,
        rendered_items: vec![ijson(json!({
            "role":"user",
            "content":[{"type":"text","text":"same-prefix"}]
        }))],
        tool_catalog: ijson(json!([])),
        stream: false,
    };
    let first = prepare(&input).expect("first prepare");
    let second = prepare(&input).expect("retry prepare");
    assert_eq!(first.candidate_bytes, first.body.len() as u64);
    assert_eq!(first.body, second.body);
    assert_eq!(first.request_digest, second.request_digest);
}

#[test]
fn diagnostic_capture_retains_malformed_body_but_withholds_credential_echo() {
    for (status, body, withheld) in [
        (200, b"data: {broken-json}\n\n".as_slice(), false),
        (
            200,
            b"data: {\"text\":\"fixture-secret\"}\n\n".as_slice(),
            true,
        ),
        (400, b"data: {broken-json}\n\n".as_slice(), false),
        (
            400,
            b"data: {\"text\":\"fixture-secret\"}\n\n".as_slice(),
            true,
        ),
    ] {
        let header = format!("HTTP/1.1 {status} Test\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).into_bytes();
        let (endpoint, server) = serve_once(vec![header, body.to_vec()]);
        let capture = Arc::new(std::sync::Mutex::new(b"stale prior response".to_vec()));
        let completion = HttpRuntime::new()
            .unwrap()
            .with_response_capture(Arc::clone(&capture))
            .send(
                AdapterId::Responses,
                &local_prepared(AdapterId::Responses, endpoint, false),
                "fixture-secret",
                &Arc::new(AtomicBool::new(false)),
            )
            .unwrap();
        assert!(matches!(
            completion,
            ProviderCompletion::Failure(ProviderFailure::Malformed { .. })
        ));
        let retained = capture.lock().unwrap();
        if withheld {
            assert!(retained.is_empty());
        } else {
            assert_eq!(retained.as_slice(), body);
        }
        server.join().unwrap();
    }
}

#[test]
fn deepseek_http_error_preserves_request_rejection_instead_of_requiring_terminal_status() {
    let body = br#"{"error":{"message":"No tool output found for tool call call-fixture.","type":"invalid_request_error","code":"invalid_request_error"}}"#;
    for dialect in [DialectId::DeepseekResponsesV1, DialectId::DeepseekChatV1] {
        if dialect == DialectId::DeepseekChatV1 {
            let terminal = provider::normalize_dialect_response(dialect, body).unwrap();
            assert_eq!(terminal.finish_reason, FinishReason::ProviderError);
            assert!(!terminal.is_final_answer());
            assert_eq!(
                terminal.provider_error.unwrap()["code"],
                "invalid_request_error"
            );
            continue;
        }
        let header = format!("HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).into_bytes();
        let (endpoint, server) = serve_once(vec![header, body.to_vec()]);
        let completion = HttpRuntime::new()
            .unwrap()
            .send_dialect_with_frames_and_wall_transport(
                dialect,
                &local_prepared(dialect.family(), endpoint, false),
                None,
                "fixture-secret",
                &Arc::new(AtomicBool::new(false)),
                None,
                |_| Ok(()),
            )
            .unwrap();
        let ProviderCompletion::Terminal(terminal) = completion else {
            panic!("request error was misclassified: {completion:?}");
        };
        assert_eq!(terminal.finish_reason, FinishReason::ProviderError);
        assert!(!terminal.is_final_answer());
        assert!(terminal.content.is_empty() && terminal.tool_calls.is_empty());
        assert_eq!(
            terminal.provider_error.unwrap()["message"],
            "No tool output found for tool call call-fixture."
        );
        assert_eq!(terminal.raw_response, body);
        server.join().unwrap();
    }
}

#[test]
fn request_error_envelope_redacts_credential_before_retaining_terminal() {
    let body = br#"{"error":{"message":"Rejected fixture-secret","code":"invalid_request_error"}}"#;
    let header = format!("HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).into_bytes();
    let (endpoint, server) = serve_once(vec![header, body.to_vec()]);
    let capture = Arc::new(std::sync::Mutex::new(Vec::new()));
    let completion = HttpRuntime::new()
        .unwrap()
        .with_response_capture(Arc::clone(&capture))
        .send_dialect_with_frames_and_wall_transport(
            DialectId::DeepseekResponsesV1,
            &local_prepared(AdapterId::Responses, endpoint, false),
            None,
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
            None,
            |_| Ok(()),
        )
        .unwrap();
    let ProviderCompletion::Terminal(terminal) = completion else {
        panic!("expected request-level provider error");
    };
    assert_eq!(terminal.finish_reason, FinishReason::ProviderError);
    assert!(!String::from_utf8_lossy(&terminal.raw_response).contains("fixture-secret"));
    assert_eq!(terminal.http_status, Some(400));
    assert_eq!(
        terminal.provider_error.unwrap()["message"],
        "Rejected [REDACTED]"
    );
    assert!(capture.lock().unwrap().is_empty());
    server.join().unwrap();
}

#[test]
fn non_success_http_cannot_authorize_a_success_shaped_final_answer() {
    let body = br#"{"id":"rejected","status":"completed","output":[{"type":"message","role":"assistant","phase":"final_answer","content":[{"type":"output_text","text":"must not settle successfully"}]}]}"#;
    let header = format!("HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).into_bytes();
    let (endpoint, server) = serve_once(vec![header, body.to_vec()]);
    let completion = HttpRuntime::new()
        .unwrap()
        .send_dialect_with_frames_and_wall_transport(
            DialectId::OpenaiResponsesV1,
            &local_prepared(AdapterId::Responses, endpoint, false),
            None,
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
            None,
            |_| Ok(()),
        )
        .unwrap();
    let ProviderCompletion::Terminal(terminal) = completion else {
        panic!("expected normalized terminal");
    };
    assert_eq!(terminal.http_status, Some(400));
    assert_eq!(terminal.finish_reason, FinishReason::ProviderError);
    assert!(!terminal.is_final_answer());
    server.join().unwrap();
}

#[test]
fn connection_failure_preserves_the_cause_beyond_the_request_url() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let completion = HttpRuntime::new()
        .unwrap()
        .send_dialect_with_frames_and_wall_transport(
            DialectId::GoogleGenerationV1,
            &local_prepared(AdapterId::GoogleGeneration, endpoint, false),
            None,
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
            Some(std::time::Duration::from_secs(5)),
            |_| Ok(()),
        )
        .unwrap();
    let ProviderCompletion::Failure(ProviderFailure::Transport { detail, .. }) = completion else {
        panic!("expected connection failure: {completion:?}");
    };
    assert!(detail.starts_with("connect: "), "{detail}");
    assert!(
        detail.contains("tcp connect error"),
        "underlying cause must survive: {detail}"
    );
    assert!(detail.len() <= 512 && !detail.contains("fixture-secret"));
}

#[test]
fn malformed_http_error_retains_bounded_redacted_provider_diagnostic() {
    let body = serde_json::to_vec(&serde_json::json!({"error": {
        "message":format!("tool_result ordering rejected fixture-secret {}", "x".repeat(2000))
    }}))
    .unwrap();
    let header = format!("HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).into_bytes();
    let (endpoint, server) = serve_once(vec![header, body]);
    let dialect = DialectId::AnthropicMessagesV1;
    let completion = HttpRuntime::new()
        .unwrap()
        .send_dialect_with_frames_and_wall_transport(
            dialect,
            &local_prepared(dialect.family(), endpoint, false),
            None,
            "fixture-secret",
            &Arc::new(AtomicBool::new(false)),
            None,
            |_| Ok(()),
        )
        .unwrap();
    let ProviderCompletion::Failure(ProviderFailure::Malformed { detail }) = completion else {
        panic!("expected malformed response: {completion:?}");
    };
    assert!(detail.contains("http_400") && detail.contains("tool_result ordering rejected"));
    assert!(detail.contains("[redacted]") && !detail.contains("fixture-secret"));
    assert!(detail.len() < 1100);
    server.join().unwrap();
}

#[test]
fn http_content_sink_runs_before_server_releases_terminal_bytes() {
    use std::time::Duration;
    for (adapter, path, prefix_events, required_frames) in [
        (
            AdapterId::Responses,
            "provider-runtime/predecessor/openai_responses.stream.sse",
            2,
            1,
        ),
        (
            AdapterId::Anthropic,
            "provider-runtime/predecessor/anthropic.stream.sse",
            3,
            1,
        ),
        (
            AdapterId::GoogleGeneration,
            "provider-runtime/predecessor/google_generation.stream.sse",
            1,
            1,
        ),
        (
            AdapterId::ChatCompletion,
            "provider-runtime/predecessor/chat_completion.stream.sse",
            2,
            1,
        ),
        (
            AdapterId::GoogleInteractions,
            "provider-runtime/google-interactions.stream.sse",
            3,
            1,
        ),
        (
            AdapterId::Responses,
            "provider-runtime/responses-completion-only-tools.stream.sse",
            4,
            2,
        ),
    ] {
        let body = read(path);
        let split = body
            .windows(2)
            .enumerate()
            .filter(|(_, b)| *b == b"\n\n")
            .nth(prefix_events - 1)
            .unwrap()
            .0
            + 2;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let (ack, gate) = std::sync::mpsc::channel();
        let server = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            loop {
                let mut byte = [0];
                socket.read_exact(&mut byte).unwrap();
                request.push(byte[0]);
                if request.ends_with(b"\r\n\r\n") {
                    break;
                }
            }
            let headers = String::from_utf8(request).unwrap();
            let length: usize = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse().unwrap())
                })
                .unwrap_or(0);
            socket.read_exact(&mut vec![0; length]).unwrap();
            write!(socket, "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).unwrap();
            socket.write_all(&body[..split]).unwrap();
            socket.flush().unwrap();
            gate.recv_timeout(Duration::from_secs(5))
                .expect("content sink must acknowledge before terminal bytes exist");
            socket.write_all(&body[split..]).unwrap();
        });
        let prepared = local_prepared(adapter, endpoint, true);
        let mut acknowledged = false;
        let mut content_frames = 0;
        let completion = HttpRuntime::new()
            .unwrap()
            .send_with_frames_and_wall(
                adapter,
                &prepared,
                "fixture-key",
                &Arc::new(AtomicBool::new(false)),
                Some(Duration::from_secs(10)),
                |frame| {
                    let counts = if required_frames == 2 {
                        matches!(frame, ProviderFrame::ToolCallReady(_))
                    } else {
                        matches!(
                            frame,
                            ProviderFrame::TextDelta(_)
                                | ProviderFrame::ReasoningDelta(_)
                                | ProviderFrame::ToolDelta { .. }
                        )
                    };
                    if !acknowledged && counts {
                        content_frames += 1;
                        if content_frames == required_frames {
                            ack.send(()).unwrap();
                            acknowledged = true;
                        }
                    }
                    Ok(())
                },
            )
            .unwrap();
        server.join().unwrap();
        assert!(acknowledged, "{adapter:?}");
        assert!(
            matches!(completion, ProviderCompletion::Terminal(_)),
            "{completion:?}"
        );
    }
}
