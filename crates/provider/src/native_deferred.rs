//! Explicit native discovery references, separate from ordinary function exposure.
use crate::{DialectId, PrepareError};
use schema::IJsonValue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct DeferredToolDefinition {
    pub schema: IJsonValue,
    pub schema_digest: String,
}

#[derive(Clone, Debug)]
pub struct DeferredToolReference {
    pub tool_name: String,
    pub schema_digest: String,
    pub catalog_revision: String,
    pub source_search_call_id: String,
}

/// Caller supplies references only after validating its durable search result.
/// The adapter additionally verifies catalog identity and replay call/result binding.
#[derive(Clone, Debug)]
pub struct NativeDeferredTools {
    pub catalog_revision: String,
    pub search_tool_name: String,
    pub definitions: Vec<DeferredToolDefinition>,
    pub references: Vec<DeferredToolReference>,
}

fn invalid(message: &str) -> PrepareError {
    PrepareError::InvalidJson(format!("native deferred tools: {message}"))
}

pub(crate) fn apply(
    dialect: DialectId,
    catalog: &Value,
    body: &mut Value,
    native: &NativeDeferredTools,
) -> Result<(), PrepareError> {
    if !matches!(
        dialect,
        DialectId::AnthropicMessagesV1 | DialectId::OpenaiResponsesV1
    ) {
        return Err(invalid(
            "native references require OpenAI Responses or Anthropic Messages",
        ));
    }
    if native.catalog_revision.is_empty() || native.search_tool_name.is_empty() {
        return Err(invalid(
            "catalog revision and search tool name are required",
        ));
    }
    let schemas = catalog
        .as_array()
        .ok_or_else(|| invalid("catalog is not an array"))?;
    let mut catalog_names = BTreeSet::new();
    for schema in schemas {
        let name = schema["name"]
            .as_str()
            .ok_or_else(|| invalid("catalog tool has no name"))?;
        if !catalog_names.insert(name) {
            return Err(invalid("duplicate catalog tool name"));
        }
    }
    let mut definitions = BTreeMap::new();
    for definition in &native.definitions {
        let schema: Value =
            serde_json::to_value(&definition.schema).map_err(|_| invalid("invalid schema"))?;
        let name = schema["name"]
            .as_str()
            .ok_or_else(|| invalid("schema has no name"))?
            .to_owned();
        let digest = format!(
            "sha256-{:x}",
            Sha256::digest(
                definition
                    .schema
                    .canonical_bytes()
                    .map_err(|_| invalid("invalid schema bytes"))?
            )
        );
        if digest != definition.schema_digest
            || name == native.search_tool_name
            || schemas
                .iter()
                .filter(|candidate| **candidate == schema)
                .count()
                != 1
            || definitions.insert(name, digest).is_some()
        {
            return Err(invalid(
                "deferred schema is duplicated, stale, absent, or is the search tool",
            ));
        }
    }
    if schemas
        .iter()
        .filter(|s| s["name"] == native.search_tool_name)
        .count()
        != 1
    {
        return Err(invalid("search tool must be declared exactly once"));
    }
    for tool in body["tools"]
        .as_array_mut()
        .ok_or_else(|| invalid("rendered tools are absent"))?
    {
        if tool["name"]
            .as_str()
            .is_some_and(|name| definitions.contains_key(name))
        {
            tool["defer_loading"] = Value::Bool(true);
        }
    }
    let mut grouped: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for reference in &native.references {
        if reference.catalog_revision != native.catalog_revision
            || definitions.get(&reference.tool_name) != Some(&reference.schema_digest)
            || reference.source_search_call_id.is_empty()
            || !seen.insert((&reference.source_search_call_id, &reference.tool_name))
        {
            return Err(invalid(
                "reference is stale, duplicated or outside the catalog",
            ));
        }
        grouped
            .entry(&reference.source_search_call_id)
            .or_default()
            .push(&reference.tool_name);
    }
    if dialect == DialectId::OpenaiResponsesV1 {
        return apply_openai(body, native, &definitions, grouped);
    }
    let messages = body["messages"]
        .as_array_mut()
        .ok_or_else(|| invalid("messages are absent"))?;
    for (call_id, names) in grouped {
        let mut calls = Vec::new();
        let mut results = Vec::new();
        for (mi, message) in messages.iter().enumerate() {
            for (bi, block) in message["content"]
                .as_array()
                .into_iter()
                .flatten()
                .enumerate()
            {
                if block["type"] == "tool_use" && block["id"] == call_id {
                    if message["role"] != "assistant" || block["name"] != native.search_tool_name {
                        return Err(invalid("reference source is not the declared search call"));
                    }
                    calls.push((mi, bi));
                }
                if block["type"] == "tool_result" && block["tool_use_id"] == call_id {
                    if message["role"] != "user" || block["is_error"] == true {
                        return Err(invalid(
                            "reference result is not a successful user tool result",
                        ));
                    }
                    results.push((mi, bi));
                }
            }
        }
        if calls.len() != 1 || results.len() != 1 || calls[0].0 >= results[0].0 {
            return Err(invalid(
                "reference requires one preceding search call and one result",
            ));
        }
        let (mi, bi) = results[0];
        let result = &mut messages[mi]["content"][bi];
        if !result["content"].is_string() {
            return Err(invalid("unexpected result representation"));
        }
        // Anthropic treats definitions as a separate content family: native
        // references cannot be mixed with ordinary text in this tool result.
        result["content"] = Value::Array(
            names
                .into_iter()
                .map(|name| json!({"type":"tool_reference","tool_name":name}))
                .collect(),
        );
    }
    Ok(())
}

fn apply_openai(
    body: &mut Value,
    native: &NativeDeferredTools,
    definitions: &BTreeMap<String, String>,
    grouped: BTreeMap<&str, Vec<&str>>,
) -> Result<(), PrepareError> {
    if native.search_tool_name != "tool_search" || body.get("previous_response_id").is_some() {
        return Err(invalid(
            "OpenAI native search requires tool_search and client-managed replay",
        ));
    }
    body["store"] = json!(false);
    let tools = body["tools"]
        .as_array_mut()
        .ok_or_else(|| invalid("tools are absent"))?;
    let mut loaded = BTreeMap::new();
    for tool in tools.iter() {
        if tool["name"]
            .as_str()
            .is_some_and(|name| definitions.contains_key(name))
        {
            loaded.insert(tool["name"].as_str().unwrap().to_owned(), tool.clone());
        }
    }
    tools.retain(|tool| {
        !tool["name"]
            .as_str()
            .is_some_and(|name| definitions.contains_key(name))
    });
    let search = tools
        .iter_mut()
        .find(|tool| tool["name"] == native.search_tool_name)
        .ok_or_else(|| invalid("search tool absent"))?;
    *search = json!({"type":"tool_search","execution":"client","description":search["description"],"parameters":search["parameters"]});
    let items = body["input"]
        .as_array_mut()
        .ok_or_else(|| invalid("input is absent"))?;
    for (call_id, names) in grouped {
        let calls = items
            .iter()
            .enumerate()
            .filter(|(_, item)| item["type"] == "tool_search_call" && item["call_id"] == call_id)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let results = items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item["type"] == "function_call_output" && item["call_id"] == call_id
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if calls.len() != 1 || results.len() != 1 || calls[0] >= results[0] {
            return Err(invalid(
                "native output requires one preceding search call and one result",
            ));
        }
        let call = &items[calls[0]];
        if call["execution"] != "client"
            || call["status"] != "completed"
            || !call["arguments"].is_object()
        {
            return Err(invalid("invalid native search source"));
        }
        items[results[0]] = json!({"type":"tool_search_output","execution":"client","call_id":call_id,"status":"completed","tools":names.iter().map(|name|loaded[*name].clone()).collect::<Vec<_>>()});
    }
    // A native search that bound nothing (no matches, or a failed host search)
    // still replays natively: an empty loaded set, never an ordinary function
    // output that the API would reject for a client tool_search_call.
    let searched = items
        .iter()
        .filter(|item| item["type"] == "tool_search_call")
        .map(|item| item["call_id"].clone())
        .collect::<Vec<_>>();
    for item in items
        .iter_mut()
        .filter(|item| item["type"] == "function_call_output")
    {
        if searched.contains(&item["call_id"]) {
            *item = json!({"type":"tool_search_output","execution":"client","call_id":item["call_id"].clone(),"status":"completed","tools":[]});
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Value, NativeDeferredTools) {
        let schema=IJsonValue::parse_str(r#"{"description":"Shipping ETA","name":"get_shipping_eta","parameters":{"additionalProperties":false,"properties":{"order_id":{"type":"string"}},"required":["order_id"],"type":"object"}}"#).unwrap();
        let digest = format!(
            "sha256-{:x}",
            Sha256::digest(schema.canonical_bytes().unwrap())
        );
        let native = NativeDeferredTools {
            catalog_revision: "catalog-v1".into(),
            search_tool_name: "tool_search".into(),
            definitions: vec![DeferredToolDefinition {
                schema: schema.clone(),
                schema_digest: digest.clone(),
            }],
            references: vec![DeferredToolReference {
                tool_name: "get_shipping_eta".into(),
                schema_digest: digest,
                catalog_revision: "catalog-v1".into(),
                source_search_call_id: "search-1".into(),
            }],
        };
        let catalog = json!([{"name":"tool_search"},schema]);
        let body = json!({"tools":[{"name":"tool_search"},{"name":"get_shipping_eta"}],"messages":[{"role":"assistant","content":[{"type":"tool_use","name":"tool_search","id":"search-1","input":{"query":"shipping"}}]},{"role":"user","content":[{"type":"tool_result","tool_use_id":"search-1","content":"Loaded shipping ETA","is_error":false}]}]});
        (catalog, body, native)
    }
    #[test]
    fn custom_reference_preserves_call_identity_and_binds_declared_schema() {
        let (catalog, mut body, native) = fixture();
        apply(DialectId::AnthropicMessagesV1, &catalog, &mut body, &native).unwrap();
        assert!(body["tools"][0].get("defer_loading").is_none());
        assert_eq!(body["tools"][1]["defer_loading"], true);
        assert_eq!(
            body["messages"][1]["content"][0]["content"],
            json!([{"type":"tool_reference","tool_name":"get_shipping_eta"}])
        );
    }
    #[test]
    fn references_reject_stale_catalog_digest_and_call_binding() {
        for mutation in [
            "digest",
            "revision",
            "call",
            "tool",
            "duplicate",
            "error",
            "wrong_source",
            "missing_call",
            "out_of_order",
        ] {
            let (catalog, mut body, mut native) = fixture();
            match mutation {
                "digest" => native.references[0].schema_digest = "stale".into(),
                "revision" => native.references[0].catalog_revision = "stale".into(),
                "call" => native.references[0].source_search_call_id = "other".into(),
                "tool" => native.references[0].tool_name = "other".into(),
                "duplicate" => native.references.push(native.references[0].clone()),
                "error" => body["messages"][1]["content"][0]["is_error"] = json!(true),
                "wrong_source" => body["messages"][0]["content"][0]["name"] = json!("other"),
                "missing_call" => body["messages"][0]["content"] = json!([]),
                "out_of_order" => body["messages"].as_array_mut().unwrap().swap(0, 1),
                _ => unreachable!(),
            }
            assert!(
                apply(DialectId::AnthropicMessagesV1, &catalog, &mut body, &native).is_err(),
                "{mutation}"
            );
        }
    }
    #[test]
    fn openai_client_search_uses_native_output_and_rejects_unbound_replay() {
        let (catalog, _, native) = fixture();
        let original = json!({"tools":[{"name":"tool_search","description":"Search","parameters":{"type":"object"}},{"type":"function","name":"get_shipping_eta","parameters":{"type":"object"}}],"input":[{"type":"message","role":"user","content":[{"type":"input_text","text":"LIVE-42"}]},{"type":"tool_search_call","execution":"client","status":"completed","call_id":"search-1","arguments":{"query":"shipping"}},{"type":"function_call_output","call_id":"search-1","output":"Loaded"}]});
        let mut body = original.clone();
        apply(DialectId::OpenaiResponsesV1, &catalog, &mut body, &native).unwrap();
        assert_eq!(body["store"], false);
        assert_eq!(body["tools"].as_array().unwrap().len(), 1);
        assert_eq!(body["tools"][0]["type"], "tool_search");
        assert_eq!(body["tools"][0]["execution"], "client");
        assert_eq!(body["input"][0], original["input"][0]);
        assert_eq!(body["input"][2]["type"], "tool_search_output");
        assert_eq!(body["input"][2]["tools"][0]["name"], "get_shipping_eta");
        assert_eq!(body["input"][2]["tools"][0]["defer_loading"], true);
        for mutation in [
            "missing_source",
            "server_source",
            "wrong_result",
            "server_chain",
        ] {
            let mut body = original.clone();
            match mutation {
                "missing_source" => body["input"][1]["call_id"] = json!("other"),
                "server_source" => body["input"][1]["execution"] = json!("server"),
                "wrong_result" => body["input"][2]["call_id"] = json!("other"),
                "server_chain" => body["previous_response_id"] = json!("old"),
                _ => unreachable!(),
            }
            assert!(
                apply(DialectId::OpenaiResponsesV1, &catalog, &mut body, &native).is_err(),
                "{mutation}"
            );
        }
        // A search that bound nothing replays as an empty native output.
        let mut body = original.clone();
        let mut unbound = native.clone();
        unbound.references.clear();
        apply(DialectId::OpenaiResponsesV1, &catalog, &mut body, &unbound).unwrap();
        assert_eq!(
            body["input"][2],
            json!({"type":"tool_search_output","execution":"client","call_id":"search-1","status":"completed","tools":[]})
        );
    }

    #[test]
    fn initial_catalog_defers_without_fabricating_references() {
        let (catalog, mut body, mut native) = fixture();
        native.references.clear();
        body["messages"] = json!([]);
        apply(DialectId::AnthropicMessagesV1, &catalog, &mut body, &native).unwrap();
        assert_eq!(body["messages"], json!([]));
        assert!(apply(DialectId::DeepseekResponsesV1, &catalog, &mut body, &native).is_err());
    }
}
