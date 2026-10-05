use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
#[ignore = "live route credentials; run scripts/run-live-chat-layout.py"]
fn legacy_chat_layout_acceptance_probe() {
    let endpoint = std::env::var("TEKES_LAYOUT_ENDPOINT").unwrap();
    let model = std::env::var("TEKES_LAYOUT_MODEL").unwrap();
    let key = std::env::var("TEKES_LAYOUT_KEY").unwrap();
    let directory = std::path::PathBuf::from(std::env::var("TEKES_LAYOUT_OUTPUT").unwrap());
    assert!(endpoint.starts_with("https://"));
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let mut receipt = json!({"endpoint":endpoint,"model":model,"scope":"Protocol layout acceptance only; does not authorize production reordering.","results":[]});
    let mut passed = true;
    for case in [
        "current_layout_is_accepted",
        "f2_candidate_layout_frames_before_the_live_turn",
        "f2_causal_variant_keeps_a_base_user_message_first",
    ] {
        let mut messages = vec![json!({"role":"system","content":"You are terse."})];
        if case == "current_layout_is_accepted" {
            messages.push(json!({"role":"user","content":"Reply with: acknowledged."}));
        } else if case == "f2_causal_variant_keeps_a_base_user_message_first" {
            messages.push(json!({"role":"user","content":"Begin the task."}));
        }
        messages.push(json!({"role":"assistant","content":null,"tool_calls":[{"id":"call-0","type":"function","function":{"name":"probe","arguments":"{}"}}]}));
        messages.push(json!({"role":"tool","tool_call_id":"call-0","content":"sealed result"}));
        if case != "current_layout_is_accepted" {
            messages.push(json!({"role":"user","content":"Reply with: acknowledged."}));
        }
        let body = json!({"model":model,"messages":messages,"max_tokens":32,"stream":false,
            "tools":[{"type":"function","function":{"name":"probe","description":"a probe","parameters":{"type":"object","properties":{}}}}]});
        let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
        std::fs::write(directory.join(format!("{case}.request.json")), &bytes).unwrap();
        let response = runtime.block_on(async {
            let response = client
                .post(format!(
                    "{}/chat/completions",
                    endpoint.trim_end_matches('/')
                ))
                .header("content-type", "application/json")
                .bearer_auth(&key)
                .body(bytes.clone())
                .send()
                .await?;
            let status = response.status().as_u16();
            Ok::<_, reqwest::Error>((status, response.text().await?))
        });
        let (status, text) = match response {
            Ok((status, text)) => (Some(status), text),
            Err(error) => (None, error.to_string()),
        };
        let text = text.replace(&key, "[REDACTED]");
        std::fs::write(directory.join(format!("{case}.response.txt")), &text).unwrap();
        let case_passed =
            status == Some(200) || (case != "current_layout_is_accepted" && status == Some(400));
        passed &= case_passed;
        receipt["results"].as_array_mut().unwrap().push(json!({"case":case,"http_status":status,"passed":case_passed,
            "layout_accepted":status == Some(200),"request_sha256":format!("{:x}",Sha256::digest(&bytes)),
            "response_excerpt":text.chars().take(2000).collect::<String>()}));
        receipt["status"] = json!(if passed { "passed" } else { "failed" });
        std::fs::write(
            directory.join("receipt.json"),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
        eprintln!("{case}: HTTP {status:?}");
    }
    assert!(
        passed,
        "layout probe failed its original status requirements; inspect receipt"
    );
}
