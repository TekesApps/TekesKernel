use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
#[ignore = "exact-route live credentials; run scripts/run-live-anthropic-schema.py"]
fn anthropic_gateway_accepts_canonical_nonstrict_schema() {
    let endpoint = std::env::var("TEKES_SCHEMA_ENDPOINT").unwrap();
    let model = std::env::var("TEKES_SCHEMA_MODEL").unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    assert!(
        endpoint.starts_with("https://gateway.ai.cloudflare.com/")
            && endpoint.ends_with("/anthropic")
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let schema = json!({"type":"object","properties":{"path":{"type":"string","pattern":"^[./]"},
        "timeout":{"type":["integer","null"],"minimum":1,"maximum":600},
        "mode":{"type":"string","enum":["read","write"]}},"required":["path"],"additionalProperties":false});
    let mut receipt = json!({"schema":"tekes.kernel.anthropic-schema-evidence.v1","route":endpoint,"model":model,
        "scope":"Only this exact route and model; strict=true is recorded but is not the acceptance gate.","results":[]});
    let mut accepted = false;
    for strict in [None, Some(true)] {
        let case = if strict.is_none() {
            "canonical_nonstrict"
        } else {
            "canonical_strict_true"
        };
        let mut tool = json!({"name":"probe_tool","description":"schema acceptance probe; do not call","input_schema":schema});
        if let Some(strict) = strict {
            tool["strict"] = json!(strict);
        }
        let body = json!({"model":model,"max_tokens":16,"thinking":{"type":"disabled"},"tools":[tool],
            "messages":[{"role":"user","content":"Reply with the single word ok. Do not use tools."}]});
        let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
        std::fs::write(directory.join(format!("{case}.request.json")), &bytes).unwrap();
        let response = runtime.block_on(async {
            let response = client
                .post(format!("{endpoint}/v1/messages"))
                .header("content-type", "application/json")
                .header("anthropic-version", "2023-06-01")
                .header("cf-aig-authorization", format!("Bearer {key}"))
                .body(bytes.clone())
                .send()
                .await?;
            let status = response.status().as_u16();
            let text = response.text().await?;
            Ok::<_, reqwest::Error>((status, text))
        });
        let (status, text) = match response {
            Ok((status, text)) => (Some(status), text),
            Err(error) => (None, error.to_string()),
        };
        let text = text.replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("{case}.response.txt")), &text).unwrap();
        receipt["results"].as_array_mut().unwrap().push(json!({"case":case,"http_status":status,
            "request_sha256":format!("{:x}",Sha256::digest(&bytes)),"response_excerpt":text.chars().take(2000).collect::<String>()}));
        if strict.is_none() {
            accepted = status == Some(200);
        }
        receipt["status"] = json!(if accepted { "passed" } else { "failed" });
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        eprintln!("{case}: HTTP {status:?}");
        if !accepted {
            break;
        }
    }
    assert!(
        accepted,
        "canonical nonstrict schema rejected; inspect retained receipt"
    );
}
