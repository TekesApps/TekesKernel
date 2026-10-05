//! Presentation projection of the actual prepared request. Byte bounds are the Kernel's
//! conservative preflight policy, never exact token measurements or cumulative billing totals.
use std::fs;
use std::path::Path;

use schema::{Event, EventKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
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
