use crate::McpCapabilities;
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// Correlates one dedicated modern subscription before exposing its changes.
/// Only notifications accepted by the first correlated acknowledgement pass.
pub struct McpSubscriptionFilter {
    id: u64,
    requested: Value,
    first_received: bool,
    accepted: BTreeSet<String>,
    resources: BTreeSet<String>,
}

impl McpSubscriptionFilter {
    pub fn new(id: u64, capabilities: &McpCapabilities, resource_uris: &[String]) -> Self {
        let mut requested = serde_json::Map::new();
        for (enabled, name) in [
            (
                capabilities.tools && capabilities.tools_list_changed,
                "toolsListChanged",
            ),
            (
                capabilities.prompts && capabilities.prompts_list_changed,
                "promptsListChanged",
            ),
            (
                capabilities.resources && capabilities.resources_list_changed,
                "resourcesListChanged",
            ),
        ] {
            if enabled {
                requested.insert(name.into(), json!(true));
            }
        }
        if capabilities.resources && capabilities.resources_subscribe && !resource_uris.is_empty() {
            let uris: BTreeSet<_> = resource_uris.iter().collect();
            requested.insert("resourceSubscriptions".into(), json!(uris));
        }
        Self {
            id,
            requested: Value::Object(requested),
            first_received: false,
            accepted: BTreeSet::new(),
            resources: BTreeSet::new(),
        }
    }
    pub fn request_params(&self) -> Value {
        json!({"notifications":self.requested})
    }
    pub fn is_empty(&self) -> bool {
        self.requested
            .as_object()
            .is_none_or(|value| value.is_empty())
    }
    pub fn accepts(&mut self, notification: &Value) -> bool {
        if self.id == 0
            || notification["jsonrpc"] != "2.0"
            || notification.get("id").is_some()
            || notification
                .pointer("/params/_meta/io.modelcontextprotocol~1subscriptionId")
                .and_then(Value::as_u64)
                != Some(self.id)
        {
            return false;
        }
        let method = notification["method"].as_str().unwrap_or_default();
        if !self.first_received {
            self.first_received = true;
            if method == "notifications/subscriptions/acknowledged" {
                let acknowledged = &notification["params"]["notifications"];
                for (key, wanted) in self.requested.as_object().unwrap() {
                    if key == "resourceSubscriptions" {
                        for uri in acknowledged[key]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(Value::as_str)
                        {
                            if wanted.as_array().is_some_and(|values| {
                                values.iter().any(|value| value.as_str() == Some(uri))
                            }) {
                                self.resources.insert(uri.into());
                            }
                        }
                    } else if acknowledged[key] == true {
                        self.accepted.insert(key.clone());
                    }
                }
            }
            return false;
        }
        match method {
            "notifications/tools/list_changed" => self.accepted.contains("toolsListChanged"),
            "notifications/prompts/list_changed" => self.accepted.contains("promptsListChanged"),
            "notifications/resources/list_changed" => {
                self.accepted.contains("resourcesListChanged")
            }
            "notifications/resources/updated" => notification["params"]["uri"]
                .as_str()
                .is_some_and(|uri| self.resources.contains(uri)),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event(id: Value, method: &str, params: Value) -> Value {
        let mut event = json!({"jsonrpc":"2.0","method":method,"params":params});
        event["params"]["_meta"] = json!({"io.modelcontextprotocol/subscriptionId":id});
        event
    }
    #[test]
    fn subscription_requires_exact_identity_acknowledgement_and_requested_scope() {
        let caps = McpCapabilities {
            tools: true,
            tools_list_changed: true,
            resources: true,
            resources_subscribe: true,
            ..Default::default()
        };
        let mut filter = McpSubscriptionFilter::new(7, &caps, &["file:///one".into()]);
        let ack = |id| {
            event(
                id,
                "notifications/subscriptions/acknowledged",
                json!({"notifications":{"toolsListChanged":true,"promptsListChanged":true,"resourceSubscriptions":["file:///one","file:///outside"]}}),
            )
        };
        assert!(!filter.accepts(&ack(json!("7"))));
        assert!(!filter.accepts(&ack(json!(8))));
        assert!(!filter.accepts(&ack(json!(7))));
        assert!(filter.accepts(&event(
            json!(7),
            "notifications/tools/list_changed",
            json!({})
        )));
        assert!(!filter.accepts(&event(
            json!(7),
            "notifications/prompts/list_changed",
            json!({})
        )));
        for (uri, expected) in [("file:///one", true), ("file:///outside", false)] {
            assert_eq!(
                filter.accepts(&event(
                    json!(7),
                    "notifications/resources/updated",
                    json!({"uri":uri})
                )),
                expected
            );
        }
        let mut poisoned = McpSubscriptionFilter::new(7, &caps, &[]);
        assert!(!poisoned.accepts(&event(
            json!(7),
            "notifications/tools/list_changed",
            json!({})
        )));
        assert!(!poisoned.accepts(&ack(json!(7))));
        assert!(!poisoned.accepts(&event(
            json!(7),
            "notifications/tools/list_changed",
            json!({})
        )));
    }
}
