use futures_util::future::join_all;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const PARAGRAPH: &str = "You are a careful engineering assistant. Prefer direct answers. Cite file paths when relevant. Do not speculate about systems you cannot observe. When a tool result is ambiguous, say so rather than guessing. Keep responses short unless asked to elaborate. Never invent an API that you have not seen in the provided context.";

async fn sample(
    directory: &Path,
    config: &profile::Provider,
    key: &str,
    client: &reqwest::Client,
    salt: &str,
    name: &str,
) -> Value {
    let model = config.models.iter().find(|m| m.enabled).unwrap();
    let system = format!("STABLE CONTRACT {salt}\n{}", vec![PARAGRAPH; 60].join("\n"));
    let body = json!({"model":model.id,"instructions":system,"stream":false,"max_output_tokens":16,
        "input":[{"role":"user","content":[{"type":"input_text","text":"Reply with the single word: acknowledged."}]}]});
    let bytes = serde_json_canonicalizer::to_vec(&body).unwrap();
    std::fs::write(directory.join(format!("{name}.request.json")), &bytes).unwrap();
    let start = Instant::now();
    let response = client
        .post(format!(
            "{}/responses",
            config.endpoint.trim_end_matches('/')
        ))
        .bearer_auth(key)
        .header("content-type", "application/json")
        .body(bytes)
        .send()
        .await
        .unwrap();
    let status = response.status().as_u16();
    let raw = response.bytes().await.unwrap();
    std::fs::write(
        directory.join(format!("{name}.response.json")),
        String::from_utf8_lossy(&raw).replace(key, "[REDACTED]"),
    )
    .unwrap();
    assert!((200..300).contains(&status), "cache probe HTTP {status}");
    let terminal =
        provider::normalize_dialect_response(provider::DialectId::OpenaiResponsesV1, &raw).unwrap();
    let usage = terminal
        .usage
        .as_ref()
        .expect("cache probe must report input usage");
    let input = usage
        .input_tokens
        .as_ref()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap();
    assert!(input > 0, "zero input tokens is not a live measurement");
    let cached = usage
        .cache_read
        .as_ref()
        .and_then(|v| v.parse::<u64>().ok());
    assert!(
        cached.is_none_or(|value| value <= input),
        "OpenAI cached input must be a subset"
    );
    json!({"name":name,"input_tokens":input,"cached_tokens":cached,"seconds":start.elapsed().as_secs_f64(),"http_status":status,"is_final_answer":terminal.is_final_answer()})
}

fn run(fanout: bool) {
    let directory = PathBuf::from(std::env::var("TEKES_SCHEMA_OUTPUT").unwrap());
    let config: profile::Provider =
        serde_json::from_slice(&std::fs::read(directory.join("provider.json")).unwrap()).unwrap();
    assert_eq!(config.dialect, "openai_responses_v1");
    assert_eq!(config.endpoint_owner, "cloudflare");
    let key = std::env::var("TEKES_SCHEMA_KEY").unwrap();
    let n = std::env::var("TEKES_FANOUT")
        .ok()
        .map(|n| n.parse::<usize>().unwrap())
        .unwrap_or(4);
    assert!(n >= 2);
    let nonce = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(120))
        .build()
        .unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let receipt=runtime.block_on(async {
        if fanout {
            let mut arms=Vec::new();
            for warmed in [false,true] {
                let arm=if warmed {"single-flight"} else {"parallel"};
                let salt=format!("{arm}-{nonce}");
                let start=Instant::now();
                let mut samples=Vec::new();
                if warmed {samples.push(sample(&directory,&config,&key,&client,&salt,&format!("{arm}-0")).await);}
                let names=(usize::from(warmed)..n).map(|i|format!("{arm}-{i}")).collect::<Vec<_>>();
                samples.extend(join_all(names.iter().map(|name|sample(&directory,&config,&key,&client,&salt,name))).await);
                assert_eq!(samples.len(),n);
                let inputs=samples.iter().map(|s|s["input_tokens"].as_u64().unwrap()).sum::<u64>();
                let cached=samples.iter().filter_map(|s|s["cached_tokens"].as_u64()).sum::<u64>();
                let unreported=samples.iter().filter(|s|s["cached_tokens"].is_null()).count();
                arms.push(json!({"arm":arm,"salt":salt,"samples":samples,"input_total":inputs,"reported_cached_total":cached,"unreported":unreported,"seconds":start.elapsed().as_secs_f64()}));
            }
            json!({"status":"passed","measurement":"fanout","fanout":n,"arms":arms})
        } else {
            let salt=format!("warm-{nonce}");
            let cold=sample(&directory,&config,&key,&client,&salt,"cold").await;
            tokio::time::sleep(Duration::from_secs(3)).await;
            let warm=sample(&directory,&config,&key,&client,&salt,"warm").await;
            let rate=warm["cached_tokens"].as_u64().map(|cached|cached as f64/warm["input_tokens"].as_u64().unwrap() as f64);
            json!({"status":"passed","measurement":"warm-rate","salt":salt,"cold":cold,"warm":warm,"warm_cache_fraction":rate})
        }
    });
    let mut receipt = receipt;
    receipt["model"] = json!(config.models.iter().find(|m| m.enabled).unwrap().id);
    receipt["scope"] = json!(
        "Original salted 60-paragraph cache experiment through configured Cloudflare OpenAI Responses. Raw-layout measurement normalized by Kernel provider; no cache threshold or final-answer requirement. Other provider routes remain separate measurements."
    );
    std::fs::write(
        directory.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "real Cloudflare credentials; scripts/run-live-cache.py --scenario fanout"]
fn fan_out_warmup_payoff() {
    run(true);
}

#[test]
#[ignore = "real Cloudflare credentials; scripts/run-live-cache.py --scenario warm"]
fn per_model_warm_cache_rate() {
    run(false);
}
