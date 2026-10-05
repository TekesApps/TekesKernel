use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Dynamic tool descriptions (MCP, plugin) may be up to this many bytes.
pub const MAX_DYNAMIC_DESCRIPTION_BYTES: usize = 4096;

use schema::IJsonValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use store::{AssetRef, AssetStore};
use tools::BuiltinManifest;

use crate::{ConfigSnapshot, ProfileError, canonical_line};

const FORMAT: u64 = 1;

/// A host-resolved dynamic tool source. `id` names the configured manifest,
/// plugin, or MCP integration; it is not executable process state.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DynamicToolSource {
    pub kind: DynamicToolSourceKind,
    pub id: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicToolSourceKind {
    Plugin,
    Mcp,
}

/// Dynamic effects retain extension-specific authority while still mapping to
/// the Kernel's common approval classes at dispatch time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicToolEffect {
    ReadOnly,
    WorkspaceWrite,
    ExternalProcess,
    NetworkRead,
    Destructive,
    ComputerControl,
    SystemPermission,
}

/// A versioned promise made by the external tool authority for one durable
/// business operation. The Kernel supplies its stable request id as the
/// idempotency key and may call the named read-only operation after a crash or
/// ambiguous transport result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalEffectBinding {
    pub protocol: ExternalEffectProtocol,
    pub reconcile_tool: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalEffectProtocol {
    IdempotencyReconcileV1,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DynamicTool {
    pub name: String,
    pub source: DynamicToolSource,
    pub effect: DynamicToolEffect,
    pub always_on: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    /// Absent means the external authority cannot prove an idempotent,
    /// queryable business effect. Production execution then fails closed for
    /// effectful tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_effect: Option<ExternalEffectBinding>,
    /// Exact provider-facing `{name,description,parameters}` object.
    pub schema: IJsonValue,
    pub schema_digest: String,
}

impl DynamicTool {
    #[allow(clippy::too_many_arguments)]
    pub fn declared(
        name: impl Into<String>,
        source: DynamicToolSource,
        effect: DynamicToolEffect,
        always_on: bool,
        aliases: Vec<String>,
        schema: IJsonValue,
    ) -> Result<Self, ProfileError> {
        let mut value = Self {
            name: name.into(),
            source,
            effect,
            always_on,
            aliases,
            external_effect: None,
            schema,
            schema_digest: String::new(),
        };
        value.schema_digest = schema_digest(&value.schema)?;
        validate_dynamic_tool(&value, Path::new("<dynamic-tool>"))?;
        Ok(value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DynamicToolCatalog {
    pub format: u64,
    pub tools: Vec<DynamicTool>,
}

impl Default for DynamicToolCatalog {
    fn default() -> Self {
        Self {
            format: FORMAT,
            tools: Vec::new(),
        }
    }
}

impl DynamicToolCatalog {
    pub fn resolve(mut tools: Vec<DynamicTool>) -> Result<Self, ProfileError> {
        tools.sort_by(compare_dynamic_tools);
        let value = Self {
            format: FORMAT,
            tools,
        };
        validate_dynamic_catalog(&value, Path::new("<dynamic-tool-catalog>"))?;
        Ok(value)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProfileError> {
        let (value, _): (Self, Value) = parse_launch_canonical("<dynamic-tool-catalog>", bytes)?;
        validate_dynamic_catalog(&value, Path::new("<dynamic-tool-catalog>"))?;
        Ok(value)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
        validate_dynamic_catalog(self, Path::new("<dynamic-tool-catalog>"))?;
        canonical_line("<dynamic-tool-catalog>", self)
    }

    pub fn digest(&self) -> Result<String, ProfileError> {
        Ok(format!("{:x}", Sha256::digest(self.canonical_bytes()?)))
    }
}

/// Immutable, host-supplied values that are specific to one worker launch and
/// therefore do not belong in user configuration. The workspace and config
/// digest prevent a valid binding from being replayed against another
/// ConfigSnapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchBindings {
    pub format: u64,
    pub workspace: String,
    pub config_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_id: Option<String>,
    pub dynamic_catalog: DynamicToolCatalog,
}

impl LaunchBindings {
    pub fn bind(
        config: &ConfigSnapshot,
        goal_id: Option<String>,
        dynamic_catalog: DynamicToolCatalog,
    ) -> Result<Self, ProfileError> {
        let value = Self {
            format: FORMAT,
            workspace: config.workspace.id.clone(),
            config_digest: config.digest()?,
            goal_id,
            dynamic_catalog,
        };
        value.validate_against(config)?;
        Ok(value)
    }

    pub fn decode(bytes: &[u8], config: &ConfigSnapshot) -> Result<Self, ProfileError> {
        let (value, raw): (Self, Value) = parse_launch_canonical("<launch-bindings>", bytes)?;
        if raw.get("goal_id").is_some_and(Value::is_null) {
            return invalid(
                Path::new("<launch-bindings>"),
                "goal_id must be absent rather than null",
            );
        }
        value.validate_against(config)?;
        Ok(value)
    }

    pub fn decode_verified(
        bytes: &[u8],
        expected_digest: &str,
        config: &ConfigSnapshot,
    ) -> Result<Self, ProfileError> {
        let actual = format!("{:x}", Sha256::digest(bytes));
        if actual != expected_digest {
            return Err(ProfileError::DigestMismatch {
                expected: expected_digest.to_owned(),
                actual,
            });
        }
        Self::decode(bytes, config)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
        validate_launch_bindings(self, Path::new("<launch-bindings>"))?;
        canonical_line("<launch-bindings>", self)
    }

    pub fn digest(&self) -> Result<String, ProfileError> {
        Ok(format!("{:x}", Sha256::digest(self.canonical_bytes()?)))
    }

    pub fn publish(&self, assets: &AssetStore) -> Result<(String, AssetRef), ProfileError> {
        let bytes = self.canonical_bytes()?;
        let digest = format!("{:x}", Sha256::digest(&bytes));
        let reference = assets.publish(&bytes)?;
        if reference.asset != format!("sha256-{digest}") {
            return Err(ProfileError::DigestMismatch {
                expected: digest,
                actual: reference.asset,
            });
        }
        Ok((digest, reference))
    }

    pub fn validate_against(&self, config: &ConfigSnapshot) -> Result<(), ProfileError> {
        let path = Path::new("<launch-bindings>");
        validate_launch_bindings(self, path)?;
        if self.workspace != config.workspace.id {
            return invalid(path, "launch workspace does not match ConfigSnapshot");
        }
        if self.config_digest != config.digest()? {
            return invalid(path, "launch config_digest does not match ConfigSnapshot");
        }

        let effective_names = BuiltinManifest::compiled()
            .tools
            .into_iter()
            .map(|tool| tool.name)
            .chain(
                self.dynamic_catalog
                    .tools
                    .iter()
                    .map(|tool| tool.name.clone()),
            )
            .collect::<BTreeSet<_>>();
        for allowed in &config.workspace.policy.allowed_tools {
            // Retired builtins may remain in historical workspace policies.
            // Accept the old selector without restoring an executable capability.
            if matches!(allowed.as_str(), "note" | "recall") {
                continue;
            }
            if !effective_names.contains(allowed) {
                return Err(ProfileError::InvalidReference {
                    path: path.to_path_buf(),
                    reason: format!(
                        "allowed tool {allowed:?} is absent from the effective catalog"
                    ),
                });
            }
        }
        Ok(())
    }
}

fn validate_launch_bindings(value: &LaunchBindings, path: &Path) -> Result<(), ProfileError> {
    if value.format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format: value.format,
        });
    }
    validate_identifier(&value.workspace, "workspace", 128, path)?;
    validate_sha256(&value.config_digest, "config_digest", path)?;
    if let Some(goal_id) = &value.goal_id {
        validate_identifier(goal_id, "goal_id", 128, path)?;
    }
    validate_dynamic_catalog(&value.dynamic_catalog, path)
}

fn validate_dynamic_catalog(value: &DynamicToolCatalog, path: &Path) -> Result<(), ProfileError> {
    if value.format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format: value.format,
        });
    }
    let fixed = BuiltinManifest::compiled()
        .tools
        .into_iter()
        .map(|tool| tool.name)
        .collect::<BTreeSet<_>>();
    let mut names = BTreeSet::new();
    for tool in &value.tools {
        validate_dynamic_tool(tool, path)?;
        if fixed.contains(&tool.name) {
            return invalid(path, "dynamic tool collides with a fixed name");
        }
        if !names.insert(tool.name.as_str()) {
            return invalid(path, "dynamic tool names must be unique");
        }
    }
    if value
        .tools
        .windows(2)
        .any(|pair| compare_dynamic_tools(&pair[0], &pair[1]).is_gt())
    {
        return invalid(path, "dynamic tools are not sorted by (source, name)");
    }
    Ok(())
}

/// The JSON Schema keywords a dynamic tool schema may use. The set is the
/// intersection of what providers accept as function parameters and what the
/// generators behind real MCP servers emit: pydantic/FastMCP (`title`,
/// `anyOf` with `null`, `$defs` + local `$ref`, `additionalProperties`,
/// `format`, `examples`), zod (`exclusiveMinimum`, `uniqueItems`, `pattern`)
/// and OpenAPI-flavoured hand-written schemas (`nullable`, `definitions`).
/// Anything outside it is refused by keyword and JSON pointer so a server
/// author can see exactly which node to change.
const DYNAMIC_SCHEMA_KEYWORDS: [&str; 33] = [
    "$defs",
    "$ref",
    "additionalProperties",
    "allOf",
    "anyOf",
    "const",
    "default",
    "definitions",
    "description",
    "enum",
    "examples",
    "exclusiveMaximum",
    "exclusiveMinimum",
    "format",
    "items",
    "maxItems",
    "maxLength",
    "maximum",
    "minItems",
    "minLength",
    "minimum",
    "nullable",
    "oneOf",
    "pattern",
    "properties",
    "required",
    "title",
    "type",
    "uniqueItems",
    // Reserved for the root node only; listed so the keyword error names the
    // placement rather than calling the keyword unsupported.
    "$schema",
    "$id",
    "$comment",
    "deprecated",
];

/// Where a node sits inside the tool schema, for error messages
/// (`/parameters/properties/layout/anyOf/1`).
struct SchemaWalk<'a> {
    path: &'a Path,
    mcp: bool,
    /// Root `$defs`/`definitions`: the only targets a local `$ref` may name.
    definitions: BTreeSet<String>,
}

impl SchemaWalk<'_> {
    fn invalid(&self, pointer: &str, reason: &str) -> Result<(), ProfileError> {
        invalid(self.path, &format!("{pointer}: {reason}"))
    }
}

fn validate_schema_node(schema: &Value, path: &Path, mcp: bool) -> Result<(), ProfileError> {
    let root = schema
        .as_object()
        .ok_or_else(|| schema_error(path, "/parameters: dynamic schema node must be an object"))?;
    let mut definitions = BTreeSet::new();
    for container in ["$defs", "definitions"] {
        if let Some(entries) = root.get(container) {
            let Some(entries) = entries.as_object() else {
                return invalid(
                    path,
                    &format!("/parameters/{container}: must be an object of schemas"),
                );
            };
            definitions.extend(entries.keys().map(|name| format!("#/{container}/{name}")));
        }
    }
    let walk = SchemaWalk {
        path,
        mcp,
        definitions,
    };
    walk.node(schema, "/parameters", true)?;
    for container in ["$defs", "definitions"] {
        if let Some(entries) = root.get(container).and_then(Value::as_object) {
            for (name, entry) in entries {
                walk.node(entry, &format!("/parameters/{container}/{name}"), false)?;
            }
        }
    }
    Ok(())
}

impl SchemaWalk<'_> {
    fn node(&self, schema: &Value, pointer: &str, root: bool) -> Result<(), ProfileError> {
        let object = schema.as_object().ok_or_else(|| {
            schema_error(
                self.path,
                &format!("{pointer}: dynamic schema node must be an object"),
            )
        })?;
        for key in object.keys() {
            if !DYNAMIC_SCHEMA_KEYWORDS.contains(&key.as_str()) {
                return self.invalid(pointer, &format!("uses unsupported keyword {key:?}"));
            }
            if matches!(key.as_str(), "$schema" | "$id" | "$defs" | "definitions")
                && (!root || !self.mcp)
            {
                return self.invalid(
                    pointer,
                    &format!(
                        "keyword {key:?} is only accepted at the root of an MCP parameters schema"
                    ),
                );
            }
        }
        if object.get("$schema").is_some_and(|dialect| {
            dialect.as_str() != Some("https://json-schema.org/draft/2020-12/schema")
        }) {
            return self.invalid(pointer, "unsupported MCP schema dialect");
        }
        if let Some(reference) = object.get("$ref") {
            let Some(reference) = reference.as_str() else {
                return self.invalid(pointer, "$ref must be a string");
            };
            if !self.definitions.contains(reference) {
                return self.invalid(
                    pointer,
                    &format!("$ref {reference:?} does not name a root $defs/definitions entry"),
                );
            }
        }
        if let Some(kind) = object.get("type") {
            let Some(kind) = kind.as_str() else {
                return self.invalid(pointer, "type must be a string");
            };
            if !matches!(
                kind,
                "object" | "array" | "string" | "integer" | "number" | "boolean" | "null"
            ) {
                return self.invalid(pointer, &format!("type {kind:?} is unsupported"));
            }
            if kind == "object" {
                self.object_node(object, pointer, root)?;
            }
            if kind == "array" {
                let Some(items) = object.get("items") else {
                    return self.invalid(pointer, "array schema requires items");
                };
                self.node(items, &format!("{pointer}/items"), false)?;
            }
        }
        for keyword in ["allOf", "anyOf", "oneOf"] {
            if let Some(branches) = object.get(keyword) {
                let Some(branches) = branches.as_array() else {
                    return self.invalid(pointer, &format!("{keyword} must be an array"));
                };
                if branches.is_empty() {
                    return self.invalid(pointer, &format!("{keyword} must be nonempty"));
                }
                for (index, branch) in branches.iter().enumerate() {
                    self.node(branch, &format!("{pointer}/{keyword}/{index}"), false)?;
                }
            }
        }
        if object
            .get("enum")
            .is_some_and(|value| value.as_array().is_none_or(Vec::is_empty))
        {
            return self.invalid(pointer, "enum must be a nonempty array");
        }
        if object
            .get("examples")
            .is_some_and(|value| !value.is_array())
        {
            return self.invalid(pointer, "examples must be an array");
        }
        for keyword in ["format", "pattern"] {
            if object.get(keyword).is_some_and(|value| !value.is_string()) {
                return self.invalid(pointer, &format!("{keyword} must be a string"));
            }
        }
        for keyword in ["nullable", "uniqueItems", "deprecated"] {
            if object.get(keyword).is_some_and(|value| !value.is_boolean()) {
                return self.invalid(pointer, &format!("{keyword} must be a boolean"));
            }
        }
        for keyword in [
            "minItems",
            "maxItems",
            "minLength",
            "maxLength",
            "minimum",
            "maximum",
            "exclusiveMinimum",
            "exclusiveMaximum",
        ] {
            if object.get(keyword).is_some_and(|value| !value.is_number()) {
                return self.invalid(pointer, &format!("{keyword} must be a number"));
            }
        }
        Ok(())
    }

    fn object_node(
        &self,
        object: &serde_json::Map<String, Value>,
        pointer: &str,
        root: bool,
    ) -> Result<(), ProfileError> {
        // Plugin schemas are closed objects with explicit properties. MCP
        // schemas keep remote semantics: a nested object may omit properties
        // (a pydantic `dict[str, Any]` argument) and additionalProperties may
        // be absent, boolean or a schema for the values.
        let empty_properties = serde_json::Map::new();
        let properties = match object.get("properties") {
            Some(Value::Object(properties)) => properties,
            None if self.mcp && !root => &empty_properties,
            _ => return self.invalid(pointer, "object schema requires properties"),
        };
        let empty_required = Vec::new();
        let required = match object.get("required") {
            None if self.mcp => &empty_required,
            Some(Value::Array(required)) => required,
            _ => return self.invalid(pointer, "object schema requires a required array"),
        };
        match object.get("additionalProperties") {
            Some(Value::Bool(false)) => {}
            Some(Value::Bool(true)) | None if self.mcp => {}
            Some(Value::Object(_)) if self.mcp => {
                self.node(
                    &object["additionalProperties"],
                    &format!("{pointer}/additionalProperties"),
                    false,
                )?;
            }
            _ => return self.invalid(pointer, "object schema must close additionalProperties"),
        }
        let mut names = BTreeSet::new();
        for item in required {
            let Some(name) = item.as_str() else {
                return self.invalid(pointer, "required items must be strings");
            };
            if !properties.contains_key(name) || !names.insert(name) {
                return self.invalid(pointer, "required must contain unique property names");
            }
        }
        for (name, child) in properties {
            self.node(child, &format!("{pointer}/properties/{name}"), false)?;
        }
        Ok(())
    }
}

fn validate_dynamic_tool(tool: &DynamicTool, path: &Path) -> Result<(), ProfileError> {
    validate_tool_name(&tool.name, path)?;
    validate_identifier(&tool.source.id, "dynamic source id", 256, path)?;
    let mut aliases = BTreeSet::new();
    for alias in &tool.aliases {
        validate_identifier(alias, "dynamic alias", 128, path)?;
        if !aliases.insert(alias.as_str()) {
            return invalid(path, "dynamic aliases must be unique");
        }
    }
    if let Some(binding) = &tool.external_effect {
        validate_identifier(
            &binding.reconcile_tool,
            "external effect reconcile_tool",
            256,
            path,
        )?;
        if matches!(
            tool.effect,
            DynamicToolEffect::ReadOnly | DynamicToolEffect::NetworkRead
        ) {
            return invalid(
                path,
                "read-only dynamic tools cannot declare an external effect protocol",
            );
        }
    }
    let actual_digest = schema_digest(&tool.schema)?;
    if tool.schema_digest != actual_digest {
        return invalid(path, "dynamic schema_digest does not match schema bytes");
    }
    validate_model_schema(
        &tool.name,
        &tool.schema,
        path,
        tool.source.kind == DynamicToolSourceKind::Mcp,
    )
}

fn validate_model_schema(
    name: &str,
    schema: &IJsonValue,
    path: &Path,
    mcp: bool,
) -> Result<(), ProfileError> {
    let value = serde_json::to_value(schema).map_err(|error| ProfileError::InvalidSchema {
        path: path.to_path_buf(),
        reason: error.to_string(),
    })?;
    let object = value
        .as_object()
        .ok_or_else(|| schema_error(path, "dynamic schema must be an object"))?;
    let keys = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    if keys != BTreeSet::from(["description", "name", "parameters"]) {
        return invalid(
            path,
            "dynamic schema must contain exactly name, description, and parameters",
        );
    }
    if object.get("name").and_then(Value::as_str) != Some(name) {
        return invalid(path, "dynamic schema name does not match catalog name");
    }
    let description = object
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default();
    // 4096 bytes (mcp-runtime): real servers ship multi-paragraph tool
    // descriptions (Cloudflare `search`: 1760 B); providers with tighter
    // function-description limits reject at request time, not here.
    if description.trim().is_empty()
        || description.len() > MAX_DYNAMIC_DESCRIPTION_BYTES
        || description
            .bytes()
            .any(|byte| byte.is_ascii_control() && !matches!(byte, b'\n' | b'\r' | b'\t'))
    {
        return invalid(
            path,
            "dynamic schema description is empty, too long, or contains non-text ASCII control",
        );
    }
    let parameters = object
        .get("parameters")
        .and_then(Value::as_object)
        .ok_or_else(|| schema_error(path, "dynamic parameters must be an object schema"))?;
    if parameters.get("type").and_then(Value::as_str) != Some("object")
        || parameters
            .get("properties")
            .and_then(Value::as_object)
            .is_none()
    {
        return invalid(path, "dynamic parameters require object/properties");
    }
    validate_schema_node(&Value::Object(parameters.clone()), path, mcp)
}

fn compare_dynamic_tools(left: &DynamicTool, right: &DynamicTool) -> std::cmp::Ordering {
    left.source
        .cmp(&right.source)
        .then_with(|| left.name.as_bytes().cmp(right.name.as_bytes()))
}

fn schema_digest(schema: &IJsonValue) -> Result<String, ProfileError> {
    let bytes = schema
        .canonical_bytes()
        .map_err(|error| schema_error(Path::new("<dynamic-tool>"), &error.to_string()))?;
    Ok(format!("sha256-{:x}", Sha256::digest(bytes)))
}

fn validate_sha256(value: &str, label: &str, path: &Path) -> Result<(), ProfileError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        invalid(
            path,
            &format!("{label} must be 64 lowercase hexadecimal digits"),
        )
    }
}

fn validate_tool_name(value: &str, path: &Path) -> Result<(), ProfileError> {
    if !value.is_empty()
        && value.len() <= 128
        && !value.chars().any(|character| character.is_ascii_control())
    {
        Ok(())
    } else {
        invalid(
            path,
            "dynamic tool name is empty, too long, or contains ASCII control",
        )
    }
}

fn validate_identifier(
    value: &str,
    label: &str,
    maximum_bytes: usize,
    path: &Path,
) -> Result<(), ProfileError> {
    if !value.is_empty()
        && value.len() <= maximum_bytes
        && !value.chars().any(|character| character.is_ascii_control())
    {
        Ok(())
    } else {
        invalid(
            path,
            &format!("{label} is empty, too long, or contains ASCII control"),
        )
    }
}

fn schema_error(path: &Path, reason: &str) -> ProfileError {
    ProfileError::InvalidSchema {
        path: path.to_path_buf(),
        reason: reason.to_owned(),
    }
}

fn invalid<T>(path: &Path, reason: &str) -> Result<T, ProfileError> {
    Err(schema_error(path, reason))
}

/// Launch documents embed JSON Schema, where JSON null is a legitimate schema
/// value. Config's absent-not-null parser is therefore intentionally not used.
fn parse_launch_canonical<T: DeserializeOwned>(
    path: impl Into<PathBuf>,
    bytes: &[u8],
) -> Result<(T, Value), ProfileError> {
    let path = path.into();
    if !bytes.ends_with(b"\n") || bytes.len() < 2 || bytes[..bytes.len() - 1].ends_with(b"\n") {
        return Err(ProfileError::InvalidBytes {
            path,
            reason: "expected one canonical JSON object followed by exactly one LF".to_owned(),
        });
    }
    let body = &bytes[..bytes.len() - 1];
    let ijson = IJsonValue::parse(body).map_err(|error| ProfileError::InvalidBytes {
        path: path.clone(),
        reason: error.to_string(),
    })?;
    if ijson
        .canonical_bytes()
        .map_err(|error| ProfileError::InvalidBytes {
            path: path.clone(),
            reason: error.to_string(),
        })?
        != body
    {
        return Err(ProfileError::InvalidBytes {
            path,
            reason: "bytes are not RFC-8785 canonical".to_owned(),
        });
    }
    let raw: Value = serde_json::from_slice(body).map_err(|error| ProfileError::InvalidBytes {
        path: path.clone(),
        reason: error.to_string(),
    })?;
    let value =
        serde_json::from_value(raw.clone()).map_err(|error| ProfileError::InvalidSchema {
            path,
            reason: error.to_string(),
        })?;
    Ok((value, raw))
}
