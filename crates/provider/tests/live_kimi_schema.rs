use serde_json::{Value, json};

#[test]
#[ignore = "live Kimi credentials; scripts/run-live-kimi-schema.py"]
fn kimi_k3_nullable_schema_has_no_moonshot_warning() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    let model = configured.models.iter().find(|m| m.enabled).unwrap();
    assert_eq!(model.id, "kimi-k3");
    assert_eq!(
        reqwest::Url::parse(&configured.endpoint)
            .unwrap()
            .host_str(),
        Some("api.moonshot.cn")
    );
    let resolved = provider::resolve_profile(&configured, model).unwrap();
    let ij = |v: Value| {
        schema::IJsonValue::parse(&serde_json_canonicalizer::to_vec(&v).unwrap()).unwrap()
    };
    let prepared = provider::prepare_with_tool_choice(&provider::PrepareInput {
        attempt_id:"kimi-schema-warning".into(),target:resolved.target.clone(),endpoint:configured.endpoint.clone(),
        epoch_profile:provider::epoch_profile(&resolved,"Answer directly without calling tools.",None).unwrap(),continuation_id:None,stream:false,
        rendered_items:vec![ij(json!({"role":"user","content":[{"type":"text","text":"What is 21 * 2? Reply with just the number."}]}))],
        tool_catalog:ij(json!([{"name":"read","description":"Read a bounded window.","strict":true,"parameters":{"type":"object","properties":{"path":{"type":"string"},"offset":{"type":["integer","null"],"minimum":1}},"required":["path","offset"],"additionalProperties":false}}])),
    },Some(provider::ToolChoice::Auto)).unwrap();
    std::fs::write(directory.join("request.json"), &prepared.body).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let (status, warning, body) = runtime.block_on(async {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap();
        let mut request = client.post(&prepared.url).header(
            &prepared.credential_header,
            format!("{}{}", prepared.credential_prefix, key),
        );
        for (name, value) in &prepared.headers_without_secret {
            request = request.header(name, value);
        }
        let response = request.body(prepared.body.clone()).send().await.unwrap();
        let status = response.status().as_u16();
        let warning = response
            .headers()
            .get("msh-schema-warning")
            .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned());
        (status, warning, response.bytes().await.unwrap())
    });
    std::fs::write(
        directory.join("response.json"),
        String::from_utf8_lossy(&body).replace(&key, "[REDACTED]"),
    )
    .unwrap();
    let normalized = provider::normalize_dialect_response(provider::DialectId::KimiChatV1, &body);
    let nonempty = normalized
        .as_ref()
        .is_ok_and(|t| t.is_final_answer() || !t.tool_calls.is_empty());
    let passed = (200..300).contains(&status) && warning.is_none() && nonempty;
    std::fs::write(directory.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":if passed {"passed"}else{"failed"},"http_status":status,"schema_warning":warning.map(|s|s.replace(&key,"[REDACTED]")),"model":model.id,"url":prepared.url,"request_digest":prepared.request_digest,"nonempty":nonempty,"scope":"Production request encoder and response normalizer, real HTTP response header gate; standalone adapter test, not worker loop."})).unwrap()).unwrap();
    assert!(passed, "inspect retained schema-warning receipt");
}
