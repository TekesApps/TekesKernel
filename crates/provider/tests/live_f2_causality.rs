use serde_json::json;

#[test]
#[ignore = "live credentials; run scripts/run-live-f2-causality.py"]
fn user_constraint_after_tool_results_measurement() {
    let endpoint = std::env::var("TEKES_LAYOUT_ENDPOINT").unwrap();
    let model = std::env::var("TEKES_LAYOUT_MODEL").unwrap();
    let key = std::env::var("TEKES_LAYOUT_KEY").unwrap();
    let directory = std::path::PathBuf::from(std::env::var("TEKES_LAYOUT_OUTPUT").unwrap());
    assert_eq!(endpoint, "https://api.deepseek.com/v1");
    assert_eq!(model, "deepseek-chat");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let mut receipt = json!({"endpoint":endpoint,"model":model,"results":[],
        "scope":"Five trials per arm, one read-only frame, no tools array. Accuracy is measured without a threshold. Does not authorize production reordering."});
    for reordered in [false, true] {
        let arm = if reordered { "reordered" } else { "baseline" };
        for trial in 0..5 {
            let user = json!({"role":"user","content":"From the check results, report ONLY the check that FAILED. Name it exactly."});
            let mut messages = vec![
                json!({"role":"system","content":"You are a terse assistant. Answer in one short sentence."}),
            ];
            if !reordered {
                messages.push(user.clone());
            }
            messages.push(json!({"role":"assistant","tool_calls":[{"id":"c0","type":"function","function":{"name":"run_checks","arguments":"{}"}}]}));
            messages.push(json!({"role":"tool","tool_call_id":"c0","content":"check_alpha: PASSED\ncheck_bravo: FAILED\ncheck_charlie: PASSED"}));
            if reordered {
                messages.push(user);
            }
            let body = json!({"model":model,"messages":messages,"stream":false,"max_tokens":64});
            std::fs::write(
                directory.join(format!("{arm}-{trial}.request.json")),
                serde_json::to_vec_pretty(&body).unwrap(),
            )
            .unwrap();
            let (status, text) = runtime.block_on(async {
                let response = client
                    .post(format!("{endpoint}/chat/completions"))
                    .bearer_auth(&key)
                    .header("content-type", "application/json")
                    .body(serde_json::to_vec(&body).unwrap())
                    .send()
                    .await
                    .unwrap();
                (response.status().as_u16(), response.text().await.unwrap())
            });
            let text = text.replace(&key, "[REDACTED]");
            std::fs::write(
                directory.join(format!("{arm}-{trial}.response.json")),
                &text,
            )
            .unwrap();
            assert_eq!(status, 200, "inspect retained response");
            let response: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert!(
                response["choices"]
                    .as_array()
                    .is_some_and(|choices| !choices.is_empty())
            );
            let answer = response["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("");
            let lower = answer.to_lowercase();
            let correct =
                lower.contains("bravo") && !lower.contains("alpha") && !lower.contains("charlie");
            receipt["results"].as_array_mut().unwrap().push(json!({"arm":arm,"trial":trial,"correct":correct,"answer":answer.chars().take(60).collect::<String>().replace('\n'," "),"finish_reason":response["choices"][0]["finish_reason"]}));
            std::fs::write(
                directory.join("receipt.json"),
                serde_json::to_vec_pretty(&receipt).unwrap(),
            )
            .unwrap();
            eprintln!("{arm} trial {trial}: correct={correct}");
        }
    }
    assert_eq!(receipt["results"].as_array().unwrap().len(), 10);
    receipt["status"] = json!("completed_measurement");
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "legacy DeepSeek quality A/B; runner --scenario quality"]
fn live_turn_last_quality_ab() {
    let endpoint = std::env::var("TEKES_LAYOUT_ENDPOINT").unwrap();
    let model = std::env::var("TEKES_LAYOUT_MODEL").unwrap();
    let key = std::env::var("TEKES_LAYOUT_KEY").unwrap();
    assert_eq!(endpoint, "https://api.deepseek.com/v1");
    assert_eq!(model, "deepseek-chat");
    let directory = std::path::PathBuf::from(std::env::var("TEKES_LAYOUT_OUTPUT").unwrap());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let trials = [
        (
            "Always end every reply with the exact token ZZ9, on its own line.",
            "Name the capital of France.",
        ),
        (
            "Reply only in lowercase, with no capital letters at all.",
            "Describe the Pacific Ocean in one short sentence.",
        ),
        (
            "Begin every reply with the word BANANA followed by a colon.",
            "How many days are in a leap year?",
        ),
    ];
    let mut scores = [0usize; 2];
    let mut receipt = json!({"status":"running","endpoint":endpoint,"model":model,"trials":[],"scope":"Exact legacy F2 quality rules and thresholds; raw Chat layout probe, not production reordering activation."});
    for last in [false, true] {
        for (trial, (rule, ask)) in trials.iter().enumerate() {
            for repeat in 0..3 {
                let user = json!({"role":"user","content":ask});
                let mut messages = vec![
                    json!({"role":"system","content":format!("You are a terse assistant. Answer directly.\n{rule}")}),
                ];
                if !last {
                    messages.push(user.clone());
                }
                messages.push(json!({"role":"assistant","tool_calls":[{"id":"call-0","type":"function","function":{"name":"probe","arguments":"{}"}}]}));
                messages.push(json!({"role":"tool","tool_call_id":"call-0","content":"an earlier tool observation, not relevant to the question"}));
                if last {
                    messages.push(user);
                }
                let body =
                    json!({"model":model,"messages":messages,"stream":false,"max_tokens":80});
                let stem = format!("quality-{}-{trial}-{repeat}", usize::from(last));
                let bytes = serde_json::to_vec_pretty(&body).unwrap();
                std::fs::write(directory.join(format!("{stem}.request.json")), &bytes).unwrap();
                let (status, raw) = runtime.block_on(async {
                    let response = client
                        .post(format!("{endpoint}/chat/completions"))
                        .bearer_auth(&key)
                        .header("content-type", "application/json")
                        .body(bytes)
                        .send()
                        .await
                        .unwrap();
                    (response.status().as_u16(), response.text().await.unwrap())
                });
                let raw = raw.replace(&key, "[REDACTED]");
                std::fs::write(directory.join(format!("{stem}.response.json")), &raw).unwrap();
                let response: serde_json::Value = serde_json::from_str(&raw).unwrap();
                let answer = response["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("");
                let complete = status == 200
                    && response["choices"][0]["finish_reason"] == "stop"
                    && !answer.trim().is_empty();
                let obeyed = complete
                    && match trial {
                        0 => answer.trim().ends_with("ZZ9"),
                        1 => answer == answer.to_lowercase(),
                        2 => answer.trim().starts_with("BANANA:"),
                        _ => unreachable!(),
                    };
                scores[usize::from(last)] += usize::from(obeyed);
                receipt["trials"].as_array_mut().unwrap().push(json!({"last":last,"trial":trial,"repeat":repeat,"completed":complete,"obeyed":obeyed,"http_status":status}));
                if !complete {
                    receipt["status"] = json!("failed_incomplete_measurement");
                }
                std::fs::write(
                    directory.join("receipt.json"),
                    serde_json::to_vec_pretty(&receipt).unwrap(),
                )
                .unwrap();
                assert!(complete, "nonterminal response invalidates measurement");
            }
        }
    }
    let pass = scores[1] + 1 >= scores[0] && scores[0] * 2 > 9;
    receipt["status"] = json!(if pass { "passed" } else { "failed" });
    receipt["baseline_passed"] = json!(scores[0]);
    receipt["reordered_passed"] = json!(scores[1]);
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    assert!(pass, "quality regression or baseline below floor");
}
