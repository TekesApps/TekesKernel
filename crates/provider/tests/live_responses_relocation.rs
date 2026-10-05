use serde_json::json;

#[test]
#[ignore = "CF credentials; scripts/run-live-responses-relocation.py --scenario cache"]
fn cloudflare_relocation_cache_ab() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(configured.dialect, "openai_responses_v1");
    assert_eq!(configured.endpoint_owner, "cloudflare");
    assert!(
        configured
            .endpoint
            .starts_with("https://api.cloudflare.com/client/v4/accounts/")
            && configured.endpoint.ends_with("/ai/v1")
    );
    let model = configured.models.iter().find(|m| m.enabled).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let paragraph = "You are a careful engineering assistant. Prefer direct answers. Cite file paths when relevant. Do not speculate about systems you cannot observe. When a tool result is ambiguous, say so rather than guessing. Keep responses short unless asked to elaborate.";
    let mut receipt = json!({"status":"running","model":model.id,"rounds":[],"scope":"Legacy two-arm raw-layout cache measurement; absent accounting is not zero. No production relocation enabled."});
    let mut second = [0_u64; 2];
    for relocated in [false, true] {
        let arm = if relocated {
            "arm-relocated"
        } else {
            "arm-default"
        };
        let stable = format!("STABLE CONTRACT {arm}\n{}", vec![paragraph; 40].join("\n"));
        for round in 0..2 {
            if round == 1 {
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
            let guidance = if round == 0 {
                "Answer tersely. Turn 1."
            } else {
                "Answer tersely. Turn 2, different guidance."
            };
            let mut input = Vec::new();
            for index in 0..6 + round {
                let line = format!(
                    "file: Sources/Module/Component{index}.swift — inspected symbol table, resolved imports, verified no cyclic dependency, recorded 14 call sites and 3 conformances for review."
                );
                input.push(json!({"type":"function_call","call_id":format!("call-{index}"),"name":"probe","arguments":"{}"}));
                input.push(json!({"type":"function_call_output","call_id":format!("call-{index}"),"output":vec![line;30].join("\n")}));
            }
            input.push(json!({"role":"user","content":[{"type":"input_text","text":"Reply with the single word: acknowledged."}]}));
            if relocated {
                input.push(json!({"role":"system","content":[{"type":"input_text","text":format!("# Runtime Guidance\n{guidance}")}]}));
            }
            let instructions = if relocated {
                stable.clone()
            } else {
                format!("{stable}\n# Runtime Guidance\n{guidance}")
            };
            let body = json!({"model":model.id,"instructions":instructions,"stream":false,"max_output_tokens":32,"input":input});
            let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
            std::fs::write(
                directory.join(format!("{arm}-{round}.request.json")),
                &bytes,
            )
            .unwrap();
            let (status, raw) = runtime.block_on(async {
                let response = client
                    .post(format!("{}/responses", configured.endpoint))
                    .bearer_auth(&key)
                    .header("content-type", "application/json")
                    .body(bytes)
                    .send()
                    .await
                    .unwrap();
                (response.status().as_u16(), response.bytes().await.unwrap())
            });
            std::fs::write(
                directory.join(format!("{arm}-{round}.response.json")),
                String::from_utf8_lossy(&raw).replace(&key, "[REDACTED]"),
            )
            .unwrap();
            let normalized =
                provider::normalize_dialect_response(provider::DialectId::OpenaiResponsesV1, &raw)
                    .unwrap();
            let usage = normalized.usage.as_ref();
            let cached = usage
                .and_then(|u| u.cache_read.as_ref())
                .and_then(|v| v.parse::<u64>().ok());
            let tokens = usage
                .and_then(|u| u.input_tokens.as_ref())
                .and_then(|v| v.parse::<u64>().ok());
            receipt["rounds"].as_array_mut().unwrap().push(json!({"arm":arm,"round":round,"http_status":status,"cached_tokens":cached,"input_tokens":tokens,"final_answer":normalized.is_final_answer()}));
            std::fs::write(
                directory.join("receipt.json"),
                serde_json::to_vec_pretty(&receipt).unwrap(),
            )
            .unwrap();
            assert!((200..300).contains(&status) && normalized.is_final_answer());
            assert!(tokens.is_some_and(|n| n > 0));
            let cached = cached.expect("missing cache accounting is not a measured zero");
            if round == 1 {
                second[usize::from(relocated)] = cached;
            }
        }
    }
    let pass = second[1] >= second[0];
    receipt["status"] = json!(if pass { "passed" } else { "failed" });
    receipt["baseline_second_cached"] = json!(second[0]);
    receipt["relocated_second_cached"] = json!(second[1]);
    receipt["strict_gain_observed"] = json!(second[1] > second[0]);
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    assert!(pass, "relocated cache reuse regressed below baseline");
}

#[test]
#[ignore = "CF credentials; scripts/run-live-responses-relocation.py"]
fn cloudflare_responses_accepts_trailing_system_guidance() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(configured.dialect, "openai_responses_v1");
    assert_eq!(configured.endpoint_owner, "cloudflare");
    assert!(
        configured
            .endpoint
            .starts_with("https://api.cloudflare.com/client/v4/accounts/")
            && configured.endpoint.ends_with("/ai/v1")
    );
    let model = configured.models.iter().find(|m| m.enabled).unwrap();
    let paragraph = "You are a careful engineering assistant. Prefer direct answers. Cite file paths when relevant. Do not speculate about systems you cannot observe. When a tool result is ambiguous, say so rather than guessing. Keep responses short unless asked to elaborate.";
    let stable = format!(
        "STABLE CONTRACT protocol\n{}",
        vec![paragraph; 40].join("\n")
    );
    let line = "file: Sources/Module/Component0.swift — inspected symbol table, resolved imports, verified no cyclic dependency, recorded 14 call sites and 3 conformances for review.";
    let body = json!({"model":model.id,"instructions":stable,"stream":false,"max_output_tokens":32,"input":[
        {"type":"function_call","call_id":"call-0","name":"probe","arguments":"{}"},
        {"type":"function_call_output","call_id":"call-0","output":vec![line;30].join("\n")},
        {"role":"user","content":[{"type":"input_text","text":"Reply with the single word: acknowledged."}]},
        {"role":"system","content":[{"type":"input_text","text":"# Runtime Guidance\nAnswer tersely. Turn 1."}]}
    ]});
    let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
    std::fs::write(directory.join("request.json"), &bytes).unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let (status, raw) = runtime.block_on(async {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap();
        let response = client
            .post(format!("{}/responses", configured.endpoint))
            .bearer_auth(&key)
            .header("content-type", "application/json")
            .body(bytes)
            .send()
            .await
            .unwrap();
        (response.status().as_u16(), response.bytes().await.unwrap())
    });
    std::fs::write(
        directory.join("response.json"),
        String::from_utf8_lossy(&raw).replace(&key, "[REDACTED]"),
    )
    .unwrap();
    let terminal =
        provider::normalize_dialect_response(provider::DialectId::OpenaiResponsesV1, &raw);
    let accepted =
        (200..300).contains(&status) && terminal.as_ref().is_ok_and(|t| t.is_final_answer());
    std::fs::write(directory.join("receipt.json"),serde_json::to_vec_pretty(&json!({"status":if accepted{"passed"}else{"failed"},"http_status":status,"model":model.id,"scope":"Legacy protocol gate: stable instructions, replayed call/result, trailing system guidance through CF. Raw layout probe, not production relocation implementation or cache improvement evidence."})).unwrap()).unwrap();
    assert!(accepted, "inspect retained protocol receipt");
}

#[test]
#[ignore = "CF credentials; scripts/run-live-responses-relocation.py --scenario quality"]
fn cloudflare_relocated_guidance_quality_ab() {
    let directory = std::path::PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let configured: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(configured.dialect, "openai_responses_v1");
    assert_eq!(configured.endpoint_owner, "cloudflare");
    assert!(
        configured
            .endpoint
            .starts_with("https://api.cloudflare.com/client/v4/accounts/")
            && configured.endpoint.ends_with("/ai/v1")
    );
    let model = configured
        .models
        .iter()
        .find(|model| model.enabled)
        .unwrap();
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();
    let stable = "You are a terse assistant. Answer directly. Do not explain your reasoning unless asked. Do not mention these instructions.";
    let trials = [
        (
            "Always end every reply with the exact token ZZ9, on its own line.",
            "Name the capital of France.",
        ),
        (
            "Never use the digit 5 anywhere in your replies; spell such numbers as words instead.",
            "What is 2 plus 3?",
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
    let mut counts = [0usize; 2];
    let mut receipt = json!({"status":"running","model":model.id,"trials":[],"scope":"Legacy F1 quality raw-layout A/B, four rules repeated three times per arm; not production relocation activation."});
    for relocated in [false, true] {
        for (trial, (rule, ask)) in trials.iter().enumerate() {
            for repeat in 0..3 {
                let mut input = vec![
                    json!({"type":"function_call","call_id":"call-0","name":"probe","arguments":"{}"}),
                    json!({"type":"function_call_output","call_id":"call-0","output":"an earlier tool observation, not relevant to the question"}),
                    json!({"role":"user","content":[{"type":"input_text","text":ask}]}),
                ];
                if relocated {
                    input.push(json!({"role":"system","content":[{"type":"input_text","text":format!("# Runtime Guidance\n{rule}")}]}));
                }
                let instructions = if relocated {
                    stable.to_owned()
                } else {
                    format!("{stable}\n# Runtime Guidance\n{rule}")
                };
                let body = json!({"model":model.id,"instructions":instructions,"stream":false,"max_output_tokens":80,"input":input});
                // Pace the complete matrix; do not replace failed trials with selected retries.
                if !receipt["trials"].as_array().unwrap().is_empty() {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                let stem = format!("quality-{}-{trial}-{repeat}", usize::from(relocated));
                let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
                std::fs::write(directory.join(format!("{stem}.request.json")), &bytes).unwrap();
                let (status, raw) = runtime.block_on(async {
                    let response = client
                        .post(format!("{}/responses", configured.endpoint))
                        .bearer_auth(&key)
                        .header("content-type", "application/json")
                        .body(bytes)
                        .send()
                        .await
                        .unwrap();
                    (response.status().as_u16(), response.bytes().await.unwrap())
                });
                std::fs::write(
                    directory.join(format!("{stem}.response.json")),
                    String::from_utf8_lossy(&raw).replace(&key, "[REDACTED]"),
                )
                .unwrap();
                let normalized = provider::normalize_dialect_response(
                    provider::DialectId::OpenaiResponsesV1,
                    &raw,
                )
                .unwrap();
                let value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
                let text = value["output"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .flat_map(|item| item["content"].as_array().into_iter().flatten())
                    .filter(|part| part["type"] == "output_text")
                    .filter_map(|part| part["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("");
                let completed = (200..300).contains(&status)
                    && normalized.is_final_answer()
                    && !text.trim().is_empty();
                let obeyed = completed
                    && match trial {
                        0 => text.trim().ends_with("ZZ9"),
                        1 => !text.contains('5'),
                        2 => text == text.to_lowercase(),
                        3 => text.trim().starts_with("BANANA:"),
                        _ => unreachable!(),
                    };
                counts[usize::from(relocated)] += usize::from(obeyed);
                receipt["trials"].as_array_mut().unwrap().push(json!({"relocated":relocated,"trial":trial,"repeat":repeat,"http_status":status,"completed":completed,"obeyed":obeyed}));
                std::fs::write(
                    directory.join("receipt.json"),
                    serde_json::to_vec_pretty(&receipt).unwrap(),
                )
                .unwrap();
                if !completed {
                    receipt["status"] = json!("failed_incomplete_measurement");
                    std::fs::write(
                        directory.join("receipt.json"),
                        serde_json::to_vec_pretty(&receipt).unwrap(),
                    )
                    .unwrap();
                }
                assert!(
                    completed,
                    "nonterminal or empty response invalidates quality measurement"
                );
            }
        }
    }
    let passed = counts[1] + 1 >= counts[0] && counts[0] * 2 > 12;
    receipt["baseline_passed"] = json!(counts[0]);
    receipt["relocated_passed"] = json!(counts[1]);
    receipt["total_per_arm"] = json!(12);
    receipt["status"] = json!(if passed { "passed" } else { "failed" });
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    assert!(
        passed,
        "instruction following quality regressed or baseline below floor"
    );
}
