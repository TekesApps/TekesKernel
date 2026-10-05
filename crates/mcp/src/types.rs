use schema::IJsonValue;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct McpCancellationToken(Arc<AtomicBool>);

impl McpCancellationToken {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub async fn cancelled(&self) {
        while !self.is_cancelled() {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolMode {
    Auto,
    Legacy,
    Modern,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Stdio,
    Http,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<IJsonValue>,
}

impl JsonRpcRequest {
    pub fn call(id: u64, method: impl Into<String>, params: Option<IJsonValue>) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: Some(id),
            method: method.into(),
            params,
        }
    }

    pub fn notification(method: impl Into<String>, params: Option<IJsonValue>) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: None,
            method: method.into(),
            params,
        }
    }

    pub fn canonical_line(&self) -> Result<Vec<u8>, McpError> {
        if self.jsonrpc != "2.0" || self.method.is_empty() || self.id == Some(0) {
            return Err(McpError::Protocol("invalid JSON-RPC request".to_owned()));
        }
        let mut bytes = serde_json_canonicalizer::to_vec(self)
            .map_err(|error| McpError::Protocol(error.to_string()))?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IJsonValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<IJsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn validate_for(&self, id: u64) -> Result<(), McpError> {
        if self.jsonrpc != "2.0" || self.id != id {
            return Err(McpError::Protocol(
                "response id/version mismatch".to_owned(),
            ));
        }
        match (self.result.is_some(), self.error.is_some()) {
            (true, false) | (false, true) => Ok(()),
            _ => Err(McpError::Protocol(
                "response must contain exactly result or error".to_owned(),
            )),
        }
    }

    pub fn into_result(self) -> Result<IJsonValue, McpError> {
        if let Some(error) = self.error {
            return Err(McpError::Remote {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }
        self.result
            .ok_or_else(|| McpError::Protocol("response result is absent".to_owned()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct McpImplementation {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<McpIcon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "websiteUrl", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct McpIcon {
    pub src: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<McpIconTheme>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpIconTheme {
    Light,
    Dark,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMcpImplementation {
    name: String,
    version: String,
    title: Option<String>,
    icons: Option<Vec<McpIcon>>,
    description: Option<String>,
    #[serde(rename = "websiteUrl")]
    website_url: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMcpIcon {
    src: String,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
    sizes: Option<Vec<String>>,
    theme: Option<McpIconTheme>,
}

impl<'de> Deserialize<'de> for McpImplementation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawMcpImplementation::deserialize(deserializer)?;
        validate_metadata_text(&raw.name, 128, "implementation name")
            .map_err(serde::de::Error::custom)?;
        validate_metadata_text(&raw.version, 128, "implementation version")
            .map_err(serde::de::Error::custom)?;
        if let Some(title) = &raw.title {
            validate_metadata_text(title, 256, "implementation title")
                .map_err(serde::de::Error::custom)?;
        }
        if let Some(description) = &raw.description {
            validate_metadata_text(description, 2_048, "implementation description")
                .map_err(serde::de::Error::custom)?;
        }
        if raw.icons.as_ref().is_some_and(|icons| icons.len() > 16) {
            return Err(serde::de::Error::custom(
                "implementation has more than 16 icons",
            ));
        }
        if let Some(website_url) = &raw.website_url {
            validate_http_url(website_url, 2_048, "implementation websiteUrl")
                .map_err(serde::de::Error::custom)?;
        }
        Ok(Self {
            name: raw.name,
            version: raw.version,
            title: raw.title,
            icons: raw.icons,
            description: raw.description,
            website_url: raw.website_url,
        })
    }
}

impl<'de> Deserialize<'de> for McpIcon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawMcpIcon::deserialize(deserializer)?;
        if raw.src.len() > 1024 * 1024 {
            return Err(serde::de::Error::custom("icon src exceeds 1 MiB"));
        }
        let parsed = url::Url::parse(&raw.src).map_err(serde::de::Error::custom)?;
        if !matches!(parsed.scheme(), "data" | "http" | "https") {
            return Err(serde::de::Error::custom(
                "icon src must use data, http, or https",
            ));
        }
        if let Some(mime_type) = &raw.mime_type {
            let Some((kind, subtype)) = mime_type.split_once('/') else {
                return Err(serde::de::Error::custom("icon mimeType is malformed"));
            };
            if mime_type.len() > 128
                || kind.is_empty()
                || subtype.is_empty()
                || mime_type.chars().any(char::is_whitespace)
            {
                return Err(serde::de::Error::custom("icon mimeType is malformed"));
            }
        }
        if let Some(sizes) = &raw.sizes {
            if sizes.len() > 16 || !sizes.iter().all(|size| valid_icon_size(size)) {
                return Err(serde::de::Error::custom("icon sizes are malformed"));
            }
        }
        Ok(Self {
            src: raw.src,
            mime_type: raw.mime_type,
            sizes: raw.sizes,
            theme: raw.theme,
        })
    }
}

fn validate_metadata_text(value: &str, maximum_bytes: usize, label: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > maximum_bytes || value.chars().any(char::is_control) {
        return Err(format!("{label} is empty, oversized, or contains controls"));
    }
    Ok(())
}

fn validate_http_url(value: &str, maximum_bytes: usize, label: &str) -> Result<(), String> {
    if value.len() > maximum_bytes {
        return Err(format!("{label} is oversized"));
    }
    let parsed = url::Url::parse(value).map_err(|_| format!("{label} is malformed"))?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err(format!("{label} must be an absolute HTTP(S) URL"));
    }
    Ok(())
}

fn valid_icon_size(value: &str) -> bool {
    if value == "any" {
        return true;
    }
    let Some((width, height)) = value.split_once('x') else {
        return false;
    };
    [width, height]
        .iter()
        .all(|dimension| dimension.parse::<u32>().is_ok_and(|value| value > 0))
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct McpCapabilities {
    pub tools: bool,
    pub prompts: bool,
    pub resources: bool,
    pub tasks: bool,
    pub subscriptions: bool,
    pub extensions: BTreeMap<String, serde_json::Value>,
    pub tools_list_changed: bool,
    pub prompts_list_changed: bool,
    pub resources_list_changed: bool,
    pub resources_subscribe: bool,
}

impl McpCapabilities {
    pub fn supports_tasks(&self) -> bool {
        self.tasks
            || self
                .extensions
                .get("io.modelcontextprotocol/tasks")
                .is_some_and(serde_json::Value::is_object)
    }
}

impl Serialize for McpCapabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let count = [
            self.tools,
            self.prompts,
            self.resources,
            self.tasks,
            self.subscriptions,
        ]
        .into_iter()
        .filter(|enabled| *enabled)
        .count();
        let mut map =
            serializer.serialize_map(Some(count + usize::from(!self.extensions.is_empty())))?;
        let empty = BTreeMap::<String, IJsonValue>::new();
        if self.tools {
            let mut detail = BTreeMap::new();
            if self.tools_list_changed {
                detail.insert("listChanged", true);
            }
            map.serialize_entry("tools", &detail)?;
        }
        if self.prompts {
            let mut detail = BTreeMap::new();
            if self.prompts_list_changed {
                detail.insert("listChanged", true);
            }
            map.serialize_entry("prompts", &detail)?;
        }
        if self.resources {
            let mut detail = BTreeMap::new();
            if self.resources_list_changed {
                detail.insert("listChanged", true);
            }
            if self.resources_subscribe {
                detail.insert("subscribe", true);
            }
            map.serialize_entry("resources", &detail)?;
        }
        if self.tasks {
            map.serialize_entry("tasks", &empty)?;
        }
        if self.subscriptions {
            map.serialize_entry("subscriptions", &empty)?;
        }
        if !self.extensions.is_empty() {
            map.serialize_entry("extensions", &self.extensions)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for McpCapabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireCapabilities {
            // Extension capabilities are advertised but never enabled by this
            // client unless an explicit supported field below is present.
            #[serde(default, rename = "experimental")]
            _experimental: Option<BTreeMap<String, IJsonValue>>,
            #[serde(default, rename = "logging")]
            _logging: Option<BTreeMap<String, IJsonValue>>,
            #[serde(default, rename = "completions")]
            _completions: Option<BTreeMap<String, IJsonValue>>,
            tools: Option<BTreeMap<String, IJsonValue>>,
            prompts: Option<BTreeMap<String, IJsonValue>>,
            resources: Option<BTreeMap<String, IJsonValue>>,
            tasks: Option<BTreeMap<String, IJsonValue>>,
            subscriptions: Option<BTreeMap<String, IJsonValue>>,
            #[serde(default)]
            extensions: BTreeMap<String, serde_json::Value>,
        }

        let wire = WireCapabilities::deserialize(deserializer)?;
        let flag =
            |object: &Option<BTreeMap<String, IJsonValue>>, key: &str| -> Result<bool, D::Error> {
                match object.as_ref().and_then(|object| object.get(key)) {
                    None => Ok(false),
                    Some(value) => serde_json::from_value::<bool>(
                        serde_json::to_value(value).map_err(serde::de::Error::custom)?,
                    )
                    .map_err(serde::de::Error::custom),
                }
            };

        Ok(Self {
            tools: wire.tools.is_some(),
            prompts: wire.prompts.is_some(),
            resources: wire.resources.is_some(),
            tasks: wire.tasks.is_some(),
            subscriptions: wire.subscriptions.is_some(),
            extensions: wire.extensions,
            tools_list_changed: flag(&wire.tools, "listChanged")?,
            prompts_list_changed: flag(&wire.prompts, "listChanged")?,
            resources_list_changed: flag(&wire.resources, "listChanged")?,
            resources_subscribe: flag(&wire.resources, "subscribe")?,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct McpToolAnnotations {
    #[serde(rename = "readOnlyHint", default)]
    pub read_only_hint: bool,
    #[serde(rename = "destructiveHint", default)]
    pub destructive_hint: bool,
}

/// The tasks extension's per-tool `execution.taskSupport` declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpTaskSupport {
    Forbidden,
    Optional,
    Required,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpToolExecution {
    #[serde(rename = "taskSupport")]
    pub task_support: McpTaskSupport,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: IJsonValue,
    #[serde(default)]
    pub annotations: McpToolAnnotations,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IJsonValue>,
    /// Task-augmentation declaration; absent when the server declared none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<McpToolExecution>,
}

impl McpTool {
    /// The server demands task-augmented execution for this tool.
    #[must_use]
    pub fn requires_task(&self) -> bool {
        self.execution
            .as_ref()
            .is_some_and(|execution| execution.task_support == McpTaskSupport::Required)
    }
}

impl<'de> Deserialize<'de> for McpTool {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WireTool {
            name: String,
            #[serde(default)]
            description: String,
            #[serde(rename = "inputSchema")]
            input_schema: IJsonValue,
            #[serde(default)]
            annotations: McpToolAnnotations,
            #[serde(rename = "_meta")]
            metadata: Option<IJsonValue>,
            #[serde(default, rename = "outputSchema")]
            _output_schema: Option<IJsonValue>,
            #[serde(default, rename = "title")]
            _title: Option<String>,
            #[serde(default, rename = "icons")]
            _icons: Option<Vec<McpIcon>>,
            #[serde(default)]
            execution: Option<WireExecution>,
        }
        // The execution object may grow other fields; only taskSupport is read.
        #[derive(Deserialize)]
        struct WireExecution {
            #[serde(rename = "taskSupport")]
            task_support: Option<McpTaskSupport>,
        }
        let wire = WireTool::deserialize(deserializer)?;
        Ok(Self {
            name: wire.name,
            description: wire.description,
            input_schema: wire.input_schema,
            annotations: wire.annotations,
            metadata: wire.metadata,
            execution: wire
                .execution
                .and_then(|execution| execution.task_support)
                .map(|task_support| McpToolExecution { task_support }),
        })
    }
}

/// Host-owned metadata for an effectful tool call. It is never model input:
/// the Kernel derives it from the durable call identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpToolCallContext {
    pub idempotency_key: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpPrompt {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpTask {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub status: String,
}

impl McpTask {
    /// Validate a task poll before treating it as a terminal tool result.
    pub fn validate_result(value: &IJsonValue, expected_id: &str) -> Result<Self, McpError> {
        let value =
            serde_json::to_value(value).map_err(|error| McpError::Protocol(error.to_string()))?;
        let invalid = || McpError::Protocol("invalid MCP task state, identity or result".into());
        let id = value["taskId"]
            .as_str()
            .filter(|id| !id.is_empty() && *id == expected_id)
            .ok_or_else(invalid)?;
        let status = value["status"]
            .as_str()
            .filter(|status| {
                matches!(
                    *status,
                    "working" | "input_required" | "completed" | "cancelled" | "failed"
                )
            })
            .ok_or_else(invalid)?;
        for field in ["createdAt", "lastUpdatedAt"] {
            let timestamp = value[field].as_str().ok_or_else(invalid)?;
            chrono::DateTime::parse_from_rfc3339(timestamp).map_err(|_| invalid())?;
        }
        for field in ["pollIntervalMs", "ttlMs"] {
            if let Some(number) = value.get(field) {
                if field == "ttlMs" && number.is_null() {
                    continue;
                }
                if number.as_u64().is_none() {
                    return Err(invalid());
                }
            }
        }
        if value
            .get("statusMessage")
            .is_some_and(|message| !message.is_string())
        {
            return Err(invalid());
        }
        if status == "input_required" {
            if value["inputRequests"]
                .as_object()
                .is_none_or(|requests| requests.is_empty())
            {
                return Err(invalid());
            }
        } else if value.get("inputRequests").is_some() {
            return Err(invalid());
        }
        if status == "completed" && !value["result"].is_object() {
            return Err(invalid());
        }
        if status == "completed"
            && value["result"]
                .get("isError")
                .is_some_and(|flag| !flag.is_boolean())
        {
            return Err(invalid());
        }
        if status == "failed" && value.get("error").is_none_or(serde_json::Value::is_null) {
            return Err(invalid());
        }
        Ok(Self {
            task_id: id.into(),
            status: status.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct McpToolContinuation {
    pub request_state: IJsonValue,
    pub input_responses: IJsonValue,
    pub round: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum McpContent {
    Text {
        text: String,
    },
    Image {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    Resource {
        uri: String,
        value: IJsonValue,
    },
    Json {
        value: IJsonValue,
    },
}

#[derive(Clone, Debug, Error)]
pub enum McpError {
    #[error("MCP transport failed: {0}")]
    Transport(String),
    #[error("MCP timed out during {0}")]
    Timeout(String),
    #[error("MCP protocol violation: {0}")]
    Protocol(String),
    #[error("MCP remote error {code}: {message}")]
    Remote {
        code: i64,
        message: String,
        data: Option<IJsonValue>,
    },
    #[error("MCP capability is unavailable: {0}")]
    Unsupported(String),
    #[error("MCP has no mutual protocol version; supported: {0:?}")]
    NoMutualProtocol(Vec<String>),
    #[error("MCP catalog exceeds v1 bounds")]
    CatalogLimit,
    #[error("MCP operation was cancelled")]
    Cancelled,
    #[error("MCP effect is unknown after transport loss")]
    UnknownEffect,
    #[error("MCP pool conflict: {0}")]
    Conflict(String),
}
