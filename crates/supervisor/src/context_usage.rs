//! Presentation projection of the actual prepared request. Byte bounds are the Kernel's
//! conservative preflight policy, never exact token measurements or cumulative billing totals.
use std::fs;
use std::path::Path;

use schema::{Event, EventKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{AssetStore, AtomicPublisher};

const MAX_SAFE: u64 = 9_007_199_254_740_991;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedProjection {
    format: u64,
    sequence: u64,
    value: Value,
}

/// Caller serializes publication for its one supervisor. Persist before exposing a sequence,
/// so a restart cannot replay an older context over the client's newer unknown/compacted value.
pub(crate) fn publish(
    folder: &Path,
    value: Value,
    minimum_sequence: u64,
) -> Result<(u64, bool), String> {
    let path = folder.join("context-usage.projection.json");
    let previous = match fs::read(&path) {
        Ok(bytes) => {
            let saved: SavedProjection =
                serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
            if saved.format != 1 || saved.sequence > MAX_SAFE {
                return Err("invalid context projection version or sequence".to_owned());
            }
            Some(saved)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.to_string()),
    };
    if let Some(previous) = &previous {
        if previous.value == value && previous.sequence >= minimum_sequence {
            return Ok((previous.sequence, false));
        }
    }
    let sequence = match previous {
        Some(previous) => previous
            .sequence
            .checked_add(1)
            .filter(|seq| *seq <= MAX_SAFE)
            .ok_or("context projection sequence exhausted")?,
        None => 0,
    }
    .max(minimum_sequence);
    if sequence > MAX_SAFE {
        return Err("context projection sequence exhausted".to_owned());
    }
    let bytes = serde_json::to_vec(&SavedProjection {
        format: 1,
        sequence,
        value,
    })
    .map_err(|error| error.to_string())?;
    AtomicPublisher::replace(path, &bytes).map_err(|error| error.to_string())?;
    Ok((sequence, true))
}

pub(crate) fn derive(
    folder: &Path,
    events: &[Event],
    config: Option<&profile::ConfigSnapshot>,
) -> Value {
    let selected = config.and_then(route);
    let mut value = json!({"usedTokens":null,"contextWindow":null,"basis":"unknown"});
    if let Some((provider, model)) = selected {
        value["provider"] = json!(provider.id);
        value["model"] = json!(model.id);
        value["contextWindow"] = json!(model.context_window_tokens);
    }
    let Some(config) = config else { return value };
    let Some((_, model)) = selected else {
        return value;
    };
    let Some(index) = events
        .iter()
        .rposition(|event| *event.kind() == EventKind::Attempt)
    else {
        return value;
    };
    let Some(run) = events[..index]
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::RunStart)
    else {
        return value;
    };
    // A new model, prompt policy or tool configuration makes the prior prepared request stale.
    if config.digest().ok().as_deref() != run.string_field("config_digest") {
        return value;
    }
    let attempt = &events[index];
    let Some(epoch) = events[..index].iter().rev().find(|event| {
        *event.kind() == EventKind::Epoch
            && event.string_field("id") == attempt.string_field("epoch")
    }) else {
        return value;
    };
    if epoch.string_field("model") != Some(model.id.as_str()) {
        return value;
    }
    let Ok(raw) = serde_json::to_value(attempt.raw()) else {
        return value;
    };
    let Some(asset) = raw.pointer("/request/asset").and_then(Value::as_str) else {
        return value;
    };
    let Ok(assets) = AssetStore::new(folder.join("assets")) else {
        return value;
    };
    let Ok(body) = assets.read_verified(asset) else {
        return value;
    };
    // Remote media and server-held continuation state have no token bound in the wire bytes.
    // Expose unknown rather than presenting their small URL/identifier as full context usage.
    let Ok(request) = serde_json::from_slice::<Value>(&body) else {
        return value;
    };
    if has_opaque_context(&request) {
        return value;
    }
    let mut bound = body.len() as u64;
    for event in &events[index + 1..] {
        // A replacement invalidates the old sample. Only a later prepared request can reprice it.
        if matches!(
            event.kind(),
            EventKind::Compact | EventKind::Epoch | EventKind::RunStart | EventKind::Extension(_)
        ) {
            return value;
        }
        let Ok(raw) = serde_json::to_value(event.raw()) else {
            return value;
        };
        if raw.get("supersedes").is_some()
            || raw.get("retracts").is_some()
            || has_opaque_context(&raw)
        {
            return value;
        }
        let Some(bytes) = expanded_bytes(&raw, &assets) else {
            return value;
        };
        bound = bound.saturating_add(bytes);
        // Native usage can exceed the text-byte bound for non-text modalities. Retain the
        // larger provider observation without assuming uniform cache accounting across dialects.
        if let Some(usage) = raw
            .get("usage")
            .filter(|usage| usage["availability"] == "reported")
        {
            let reported = ["input_tokens", "output_tokens", "cache_read", "cache_write"]
                .iter()
                .filter_map(|key| count(&usage[*key]))
                .fold(0_u64, u64::saturating_add);
            bound = bound.max(reported);
        }
    }
    if bound > MAX_SAFE {
        return value;
    }
    value["usedTokens"] = json!(bound);
    value["basis"] = json!("upperBound");
    value
}

/// One replacement sample: details inherit the exact legacy usage validity decision.
/// Only safe metadata leaves this boundary; request text and schemas stay in AssetStore.
pub(crate) fn sample(
    folder: &Path,
    events: &[Event],
    config: Option<&profile::ConfigSnapshot>,
) -> Value {
    let mut usage = derive(folder, events, config);
    let mut evidence = json!({"origin":"harness", "scope":"preparedRequest",
        "sourceKey":"native-prepared-request", "clock":"native-context-sample-v1"});
    for key in ["provider", "model"] {
        if let Some(value) = usage.get(key) {
            evidence[key] = value.clone();
        }
    }
    // Never bind stale request identity to the currently selected model on invalidation.
    let request = (usage["basis"] != "unknown")
        .then(|| {
            let attempt = events
                .iter()
                .rev()
                .find(|e| *e.kind() == EventKind::Attempt)?;
            let raw = serde_json::to_value(attempt.raw()).ok()?;
            let asset = raw.pointer("/request/asset")?.as_str()?;
            let assets = AssetStore::new(folder.join("assets")).ok()?;
            let request =
                serde_json::from_slice::<Value>(&assets.read_verified(asset).ok()?).ok()?;
            for (output, input) in [("requestID", "attempt"), ("epochID", "epoch")] {
                if let Some(value) = attempt.string_field(input) {
                    evidence[output] = json!(value);
                }
            }
            // Asset identity covers request bytes without returning a raw request.
            evidence["sourceRevision"] = json!(asset);
            Some(request)
        })
        .flatten();
    if request.is_none() {
        usage["usedTokens"] = Value::Null;
        usage["basis"] = json!("unknown");
    }
    let mut rows = Vec::new();
    if let Some(request) = request.as_ref() {
        rows = request_rows(request);
    }
    let mut details = json!({"schemaVersion":1,"evidence":evidence,"usage":usage,
    "coverage": if request.is_some() { "partial" } else { "unknown" },
    "relation": if request.is_some() { "independentEstimate" } else { "unknown" },
    "rows":rows,
    "unavailableReason": if request.is_some() {
        "Serialized request byte upper bounds only; provider framing and later retained events are not apportioned. Tool origin/server, Skills, Memory files, MCP instructions and reserve have no request-level manifest. Deferred definition flags do not prove activation state."
    } else {
        "No valid prepared request bound: absent or unreadable request, changed config/model/epoch, replacement/compaction or opaque context."
    }});
    let revision = format!(
        "native-context-v1:{:x}",
        Sha256::digest(serde_json::to_vec(&details).expect("JSON sample"))
    );
    details["revision"] = json!(revision);
    json!({"contextUsage":usage,"contextDetails":details})
}

fn byte_tokens(value: &Value) -> Value {
    json!({"value":serde_json::to_vec(value).expect("JSON component").len(),
        "basis":"upperBound", "accountingID":"native-serialized-json-bytes-v1"})
}

fn row(id: &str, category: &str, title: &str, role: &str, content: &Value) -> Value {
    json!({"id":id,"category":category,"title":title,"role":role,
        "tokens":byte_tokens(content),"children":[],"childrenCoverage":"complete"})
}

fn request_rows(request: &Value) -> Vec<Value> {
    let mut rows = Vec::new();
    let mut system = Vec::new();
    for key in [
        "instructions",
        "system",
        "system_instruction",
        "systemInstruction",
    ] {
        if let Some(value) = request.get(key) {
            system.push(value.clone());
        }
    }
    for key in ["input", "messages", "contents"] {
        if let Some(items) = request.get(key).and_then(Value::as_array) {
            let mut messages = Vec::new();
            for item in items {
                if matches!(item["role"].as_str(), Some("system" | "developer")) {
                    system.push(item.clone());
                } else {
                    messages.push(item.clone());
                }
            }
            rows.push(row(
                "messages",
                "messages",
                "Messages",
                "occupied",
                &json!(messages),
            ));
        }
    }
    if !system.is_empty() {
        rows.push(row(
            "system",
            "systemPrompt",
            "System prompt",
            "occupied",
            &json!(system),
        ));
    }
    if let Some(tools) = request.get("tools").and_then(Value::as_array) {
        let mut active = Vec::new();
        let mut deferred = Vec::new();
        for tool in tools {
            if tool["defer_loading"] == true {
                deferred.push(tool.clone());
            } else {
                active.push(tool.clone());
            }
        }
        rows.push(tool_row(
            "tools",
            "Tools (origin unavailable)",
            "occupied",
            &active,
        ));
        if !deferred.is_empty() {
            rows.push(tool_row(
                "tools-deferred",
                "Tools (deferred declarations)",
                "deferred",
                &deferred,
            ));
        }
    }
    rows
}

fn tool_row(id: &str, title: &str, role: &str, tools: &[Value]) -> Value {
    let mut parent = row(id, "tools", title, role, &json!(tools));
    let mut children = std::collections::BTreeMap::new();
    for tool in tools {
        let definitions = tool
            .get("functionDeclarations")
            .and_then(Value::as_array)
            .map(|items| items.iter().collect::<Vec<_>>())
            .unwrap_or_else(|| vec![tool]);
        for definition in definitions {
            let name = definition
                .get("name")
                .or_else(|| definition.pointer("/function/name"))
                .and_then(Value::as_str);
            // Restrict titles to identifier metadata; never echo descriptions or arbitrary data.
            let name = name
                .filter(|name| {
                    !name.is_empty()
                        && name.len() <= 128
                        && name
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"_-.:/".contains(&b))
                })
                .unwrap_or("Unnamed tool");
            let identity = if name == "Unnamed tool" {
                serde_json::to_vec(definition).expect("tool JSON")
            } else {
                name.as_bytes().to_vec()
            };
            let digest = format!("{:x}", Sha256::digest(identity));
            let child_id = format!("{id}:{digest}");
            children
                .entry(child_id.clone())
                .or_insert_with(|| row(&child_id, "tool", name, role, definition));
        }
    }
    parent["itemCount"] = json!({"value":children.len(),"coverage":"complete"});
    parent["children"] = json!(children.into_values().collect::<Vec<_>>());
    parent
}

fn has_opaque_context(value: &Value) -> bool {
    match value {
        Value::Object(fields) => fields.iter().any(|(key, value)| {
            matches!(
                key.as_str(),
                "previous_response_id"
                    | "conversation_id"
                    | "cachedContent"
                    | "image_url"
                    | "image"
                    | "audio"
                    | "video"
                    | "inlineData"
                    | "fileData"
                    | "file_id"
            ) || (key == "type"
                && value.as_str().is_some_and(|kind| {
                    kind.contains("image")
                        || kind.contains("audio")
                        || kind.contains("video")
                        || kind == "document"
                        || kind == "input_file"
                }))
                || has_opaque_context(value)
        }),
        Value::Array(values) => values.iter().any(has_opaque_context),
        _ => false,
    }
}

fn count(value: &Value) -> Option<u64> {
    value.as_u64().or_else(|| value.as_str()?.parse().ok())
}

fn expanded_bytes(value: &Value, assets: &AssetStore) -> Option<u64> {
    let base = serde_json::to_vec(value).ok()?.len() as u64;
    fn references(value: &Value, assets: &AssetStore) -> Option<u64> {
        match value {
            Value::Object(fields) => {
                let mut bytes = 0_u64;
                if let Some(asset) = fields.get("asset").and_then(Value::as_str) {
                    bytes = assets.read_verified(asset).ok()?.len() as u64;
                }
                for child in fields.values() {
                    bytes = bytes.checked_add(references(child, assets)?)?;
                }
                Some(bytes)
            }
            Value::Array(values) => values.iter().try_fold(0_u64, |sum, child| {
                sum.checked_add(references(child, assets)?)
            }),
            _ => Some(0),
        }
    }
    base.checked_add(references(value, assets)?)
}

fn route(config: &profile::ConfigSnapshot) -> Option<(&profile::Provider, &profile::Model)> {
    let provider = config
        .session_settings
        .as_ref()
        .map(|settings| settings.provider.as_str())
        .or(config.workspace.policy.provider.as_deref())
        .or(config.settings.default_provider.as_deref());
    let model = config
        .session_settings
        .as_ref()
        .map(|settings| settings.model.as_str())
        .or(config.workspace.policy.model.as_deref())
        .or(config.settings.default_model.as_deref());
    let provider = match provider {
        Some(id) => config
            .providers
            .providers
            .iter()
            .find(|provider| provider.id == id)?,
        None => config
            .providers
            .providers
            .iter()
            .find(|provider| provider.models.iter().any(|model| model.enabled))?,
    };
    let model = match model {
        Some(id) => provider
            .models
            .iter()
            .find(|model| model.enabled && model.id == id)?,
        None => provider.models.iter().find(|model| model.enabled)?,
    };
    Some((provider, model))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> profile::ConfigSnapshot {
        serde_json::from_value(json!({
            "format":1,
            "workspace":{"format":1,"revision":1,"id":"w","name":"W","cwd":[std::fs::canonicalize(std::env::temp_dir()).unwrap()],"policy":{"network":true}},
            "providers":{"format":1,"revision":1,"providers":[{
                "id":"p","adapter":"responses","dialect":"openai_responses_v1","endpoint_owner":"openai",
                "gateway_translation":"direct","evidence_revision":"openai-2026-08-01","endpoint":"https://example.com/v1",
                "models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,
                    "context_window_tokens":100000,"compact_trigger_tokens":90000}]
            }]},
            "settings":{"format":1,"revision":1,"default_provider":"p","default_model":"gpt-5"},
            "revisions":{"workspace":1,"providers":1,"settings":1}
        })).unwrap()
    }

    fn event(seq: u64, mut value: Value) -> Event {
        value["v"] = json!(1);
        value["seq"] = json!(seq);
        value["ts"] = json!("2026-10-05T00:00:00.000Z");
        Event::from_value(schema::IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap())
            .unwrap()
    }

    fn prepared(folder: &Path, config: &profile::ConfigSnapshot) -> Vec<Event> {
        let assets = AssetStore::new(folder.join("assets")).unwrap();
        let system = assets.publish(b"system").unwrap();
        let tools = assets.publish(b"[]").unwrap();
        let request = assets
            .publish(br#"{"model":"gpt-5","input":[{"role":"user","content":"hello"}]}"#)
            .unwrap();
        vec![
            event(
                1,
                json!({"kind":"run_start","run":"r1","mode":"ordinary","binary":"worker",
                "config_digest":config.digest().unwrap(),"instruction_digest":"i","policy":"p","recovery_ordinal":0}),
            ),
            event(
                2,
                json!({"kind":"epoch","id":"e1","reason":"initial","adapter":"openai_responses_v1","model":"gpt-5",
                "system":{"asset":system.asset,"digest":"s"},"tools":{"asset":tools.asset,"digest":"t"},"renderer":1}),
            ),
            event(
                3,
                json!({"kind":"attempt","turn":1,"attempt":"a1","epoch":"e1","wire_digest":"d",
                "request":{"asset":request.asset,"bytes":request.bytes},"admits":[]}),
            ),
        ]
    }

    fn replace_request(folder: &Path, events: &mut [Event], request: Value) {
        let asset = AssetStore::new(folder.join("assets"))
            .unwrap()
            .publish(&serde_json::to_vec(&request).unwrap())
            .unwrap();
        let mut raw = serde_json::to_value(events[2].raw()).unwrap();
        raw["request"] = json!({"asset":asset.asset,"bytes":asset.bytes});
        events[2] = event(3, raw);
    }

    #[test]
    fn atomic_sample_partitions_system_and_deduplicates_safe_tool_inventory() {
        let folder = tempfile::tempdir().unwrap();
        let config = config();
        let mut events = prepared(folder.path(), &config);
        replace_request(
            folder.path(),
            &mut events,
            json!({"model":"gpt-5",
            "messages":[{"role":"system","content":"private system text"},
                {"role":"user","content":"private user text"},
                {"role":"tool","content":"private tool result"}],
            "tools":[{"type":"function","function":{"name":"search","description":"secret schema"}},
                {"type":"function","function":{"name":"search","description":"secret schema"}},
                {"name":"late","defer_loading":true}]}),
        );
        let sample = sample(folder.path(), &events, Some(&config));
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../fixtures/context-details/native-v1.json"
        ))
        .unwrap();
        assert_eq!(fixture, json!({"asOfSeq":42,"values":sample}));
        println!("SYNTHETIC_WIRE={}", json!({"asOfSeq":42,"values":sample}));
        let details = &sample["contextDetails"];
        assert_eq!(details["usage"], sample["contextUsage"]);
        assert_eq!(details["coverage"], "partial");
        assert_eq!(details["relation"], "independentEstimate");
        assert_eq!(details["evidence"]["requestID"], "a1");
        assert_eq!(details["evidence"]["epochID"], "e1");
        let rows = details["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        let messages = json!([{"role":"user","content":"private user text"},
            {"role":"tool","content":"private tool result"}]);
        assert_eq!(rows[0]["tokens"], byte_tokens(&messages));
        assert_eq!(rows[1]["category"], "systemPrompt");
        assert_eq!(rows[2]["itemCount"]["value"], 1);
        assert_eq!(rows[3]["role"], "deferred");
        assert_eq!(rows[3]["children"][0]["role"], "deferred");
        assert!(details.get("reserveOutsideUsed").is_none());
        let output = serde_json::to_string(details).unwrap();
        for secret in [
            "private system text",
            "private user text",
            "private tool result",
            "secret schema",
        ] {
            assert!(!output.contains(secret));
        }
        assert_eq!(sample, super::sample(folder.path(), &events, Some(&config)));
    }

    #[test]
    fn dialect_shapes_are_classified_without_catalog_inference() {
        for (system_key, message_key) in [
            ("instructions", "input"),
            ("system", "messages"),
            ("systemInstruction", "contents"),
            ("system_instruction", "input"),
        ] {
            let mut request =
                json!({"tools":[{"functionDeclarations":[{"name":"one"},{"name":"two"}]}]});
            request[system_key] = json!("system text");
            request[message_key] = json!([{"role":"user","content":"text"}]);
            let rows = request_rows(&request);
            assert_eq!(
                rows.iter()
                    .filter(|r| r["category"] == "systemPrompt")
                    .count(),
                1
            );
            assert_eq!(rows[2]["itemCount"]["value"], 2);
            assert_eq!(rows[2]["category"], "tools"); // No invented System/MCP origin.
            let mut ids = std::collections::BTreeSet::new();
            for row in &rows {
                assert!(ids.insert(row["id"].as_str().unwrap()));
                for child in row["children"].as_array().unwrap() {
                    assert!(ids.insert(child["id"].as_str().unwrap()));
                }
            }
        }
        let rows = request_rows(&json!({"tools":[{"name":"credential text\nnot an identifier"}]}));
        assert_eq!(rows[0]["children"][0]["title"], "Unnamed tool");
    }

    #[test]
    fn invalidation_replaces_both_total_and_details_and_changes_clock() {
        let folder = tempfile::tempdir().unwrap();
        let config = config();
        let events = prepared(folder.path(), &config);
        let initial = sample(folder.path(), &events, Some(&config));
        assert_eq!(
            publish(folder.path(), initial.clone(), 4).unwrap(),
            (4, true)
        );
        for (index, replacement) in [
            json!({"kind":"compact","covers":[{"from":3,"to":3}],"summary":"small"}),
            json!({"kind":"output","turn":1,"attempt":"a1","content":[],
                "sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},
                "usage":{"availability":"unavailable"},"supersedes":[{"from":3,"to":3}]}),
            serde_json::to_value(events[0].raw()).unwrap(),
            serde_json::to_value(events[1].raw()).unwrap(),
            json!({"kind":"extension-context-replacement"}),
            json!({"kind":"output","turn":1,"attempt":"a1","content":[],
                "sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},
                "usage":{"availability":"unavailable"},"retracts":[{"from":3,"to":3}]}),
        ]
        .into_iter()
        .enumerate()
        {
            let mut changed = events.clone();
            changed.push(event(4, replacement));
            let unknown = sample(folder.path(), &changed, Some(&config));
            if index == 0 {
                let fixture: Value = serde_json::from_str(include_str!(
                    "../../../fixtures/context-details/native-unknown-v1.json"
                ))
                .unwrap();
                assert_eq!(fixture, json!({"asOfSeq":43,"values":unknown}));
                println!(
                    "SYNTHETIC_UNKNOWN_WIRE={}",
                    json!({"asOfSeq":43,"values":unknown})
                );
            }
            assert_eq!(unknown["contextUsage"]["basis"], "unknown");
            assert_eq!(unknown["contextDetails"]["usage"], unknown["contextUsage"]);
            assert!(
                unknown["contextDetails"]["rows"]
                    .as_array()
                    .unwrap()
                    .is_empty()
            );
            assert_ne!(
                unknown["contextDetails"]["revision"],
                initial["contextDetails"]["revision"]
            );
            assert!(
                unknown["contextDetails"]["evidence"]
                    .get("requestID")
                    .is_none()
            );
            assert_eq!(
                publish(folder.path(), unknown.clone(), 4).unwrap(),
                (5, index == 0)
            );
            assert_eq!(publish(folder.path(), unknown, 0).unwrap(), (5, false));
        }
        let mut changed = config.clone();
        changed.providers.providers[0].models[0].id = "other".into();
        changed.settings.default_model = Some("other".into());
        let unknown = sample(folder.path(), &events, Some(&changed));
        assert_eq!(unknown["contextDetails"]["evidence"]["model"], "other");
        assert_eq!(unknown["contextDetails"]["usage"]["basis"], "unknown");
        let mut opaque = events.clone();
        replace_request(
            folder.path(),
            &mut opaque,
            json!({"previous_response_id":"remote"}),
        );
        assert_eq!(
            sample(folder.path(), &opaque, Some(&config))["contextDetails"]["coverage"],
            "unknown"
        );
        std::fs::remove_dir_all(folder.path().join("assets")).unwrap();
        assert_eq!(
            sample(folder.path(), &events, Some(&config))["contextDetails"]["usage"]["basis"],
            "unknown"
        );
    }

    #[test]
    fn paired_clock_migrates_legacy_and_rejects_corruption() {
        let folder = tempfile::tempdir().unwrap();
        let config = config();
        let events = prepared(folder.path(), &config);
        assert_eq!(
            publish(
                folder.path(),
                derive(folder.path(), &events, Some(&config)),
                8
            )
            .unwrap(),
            (8, true)
        );
        let paired = sample(folder.path(), &events, Some(&config));
        assert_eq!(
            publish(folder.path(), paired.clone(), 8).unwrap(),
            (9, true)
        );
        assert_eq!(
            publish(folder.path(), paired.clone(), 0).unwrap(),
            (9, false)
        );
        std::fs::write(
            folder.path().join("context-usage.projection.json"),
            b"bad JSON",
        )
        .unwrap();
        assert!(publish(folder.path(), paired, 0).is_err());
    }

    #[test]
    fn request_bounds_are_not_billing_totals_and_replacements_invalidate() {
        let folder = tempfile::tempdir().unwrap();
        let config = config();
        let mut events = prepared(folder.path(), &config);
        let first = derive(folder.path(), &events, Some(&config));
        assert_eq!(first["basis"], "upperBound");
        assert_eq!(first["contextWindow"], 100000);
        events.push(event(4, json!({"kind":"output","turn":1,"attempt":"a1","content":[{"type":"text","text":"answer"}],
            "sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},
            "usage":{"availability":"reported","input_tokens":"2000","output_tokens":"100"}})));
        let reported = derive(folder.path(), &events, Some(&config));
        assert_eq!(reported["usedTokens"], 2100);
        events.push(event(
            5,
            json!({"kind":"compact","covers":[{"from":3,"to":4}],"summary":"small"}),
        ));
        let unknown = derive(folder.path(), &events, Some(&config));
        assert_eq!(unknown["basis"], "unknown");
        assert!(unknown["usedTokens"].is_null());
        let mut changed = config.clone();
        changed.providers.providers[0].models[0].context_window_tokens = 200000;
        let changed = derive(folder.path(), &events[..3], Some(&changed));
        assert_eq!(changed["contextWindow"], 200000);
        assert!(changed["usedTokens"].is_null());
    }

    #[test]
    fn opaque_media_and_continuations_are_not_priced_by_identifier_bytes() {
        assert!(has_opaque_context(
            &json!({"previous_response_id":"response-1"})
        ));
        assert!(has_opaque_context(
            &json!({"messages":[{"content":[{"type":"image_url","image_url":{"url":"https://example.com/image"}}]}]})
        ));
        assert!(!has_opaque_context(
            &json!({"input":[{"role":"user","content":"hello"}]})
        ));
    }

    #[test]
    fn durable_clock_survives_recreation_and_does_not_reuse_invalidated_sequence() {
        let folder = tempfile::tempdir().unwrap();
        let full = json!({"basis":"upperBound","usedTokens":10000});
        assert_eq!(publish(folder.path(), full.clone(), 7).unwrap(), (7, true));
        assert_eq!(publish(folder.path(), full.clone(), 7).unwrap(), (7, false));
        let unknown = json!({"basis":"unknown","usedTokens":null});
        assert_eq!(
            publish(folder.path(), unknown.clone(), 7).unwrap(),
            (8, true)
        );
        assert_eq!(publish(folder.path(), unknown, 1).unwrap(), (8, false));
        assert_eq!(publish(folder.path(), full, 20).unwrap(), (20, true));
    }
}
