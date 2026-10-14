//! Session presets: a base chosen by the workspace and ordered domain layers
//! chosen per session, rendered into the root system instructions
//! (spec session-endpoint §Session presets).
//!
//! The base is `general` (general identity line and general rules) or `blank`
//! (neither). A layer is a Kernel-published layer referenced by id, such as
//! `coding`, or an inline layer a client sends with its own identity line and
//! duty text. At most one inline layer declares an identity; it is the
//! session's primary role and wins over a Kernel layer's identity, and either
//! replaces the base's identity line but never the base rules. The `identityProfile` sugar maps to
//! the general base with `[coding]` or no layers, which renders byte-for-byte
//! like the profile text.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

use crate::builtin::BuiltinManifest;

use crate::guidance::{
    CODING_IDENTITY, CODING_RULES, GENERAL_IDENTITY, GENERAL_RULES, HARNESS_IDENTITY,
    IdentityProfile, WORKING_DIRECTORY,
};

/// Genesis format of a resolved preset.
pub const PRESET_FORMAT: u64 = 1;
/// Version of the render order below; published by `session.presets.describe`.
pub const PRESET_RENDER_VERSION: u64 = 1;

pub const MAX_PRESET_LAYERS: usize = 8;
pub const MAX_PRESET_ID_BYTES: usize = 64;
pub const MAX_PRESET_VERSION_BYTES: usize = 32;
pub const MAX_PRESET_IDENTITY_BYTES: usize = 512;
pub const MAX_PRESET_DUTY_BYTES: usize = 16 * 1024;
/// Bound on the preset-owned text: identity, base rules and every duty.
pub const MAX_PRESET_TEXT_BYTES: usize = 32 * 1024;
pub const MAX_PRESET_TOOLS: usize = 256;
pub const MAX_PRESET_MCP_SERVERS: usize = 32;
pub const MAX_PRESET_MCP_TOOLS: usize = 256;
pub const MAX_PRESET_SKILLS: usize = 256;

/// The workspace base under every session's layers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresetBase {
    #[default]
    General,
    Blank,
}

impl PresetBase {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Blank => "blank",
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "general" => Some(Self::General),
            "blank" => Some(Self::Blank),
            _ => None,
        }
    }

    const fn identity(self) -> Option<&'static str> {
        match self {
            Self::General => Some(GENERAL_IDENTITY),
            Self::Blank => None,
        }
    }

    const fn rules(self) -> Option<&'static str> {
        match self {
            Self::General => Some(GENERAL_RULES),
            Self::Blank => None,
        }
    }
}

/// A layer the Kernel publishes and clients reference by id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelLayer {
    pub id: &'static str,
    pub version: &'static str,
    pub title: &'static str,
    pub identity: &'static str,
    pub duty: &'static str,
}

pub const KERNEL_LAYERS: [KernelLayer; 1] = [KernelLayer {
    id: "coding",
    version: "1",
    title: "Coding",
    identity: CODING_IDENTITY,
    duty: CODING_RULES,
}];

fn kernel_layer(id: &str) -> Option<&'static KernelLayer> {
    KERNEL_LAYERS.iter().find(|layer| layer.id == id)
}

/// One resolved layer as frozen in genesis.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PresetLayer {
    Kernel {
        id: String,
        version: String,
    },
    Inline {
        id: String,
        version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identity: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duty: Option<String>,
    },
}

impl PresetLayer {
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Kernel { id, .. } | Self::Inline { id, .. } => id,
        }
    }

    #[must_use]
    pub fn version(&self) -> &str {
        match self {
            Self::Kernel { version, .. } | Self::Inline { version, .. } => version,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Kernel { .. } => "kernel",
            Self::Inline { .. } => "inline",
        }
    }

    fn identity(&self) -> Option<&str> {
        match self {
            Self::Kernel { id, .. } => kernel_layer(id).map(|layer| layer.identity),
            Self::Inline { identity, .. } => identity.as_deref(),
        }
    }

    fn duty(&self) -> Option<&str> {
        match self {
            Self::Kernel { id, .. } => kernel_layer(id).map(|layer| layer.duty),
            Self::Inline { duty, .. } => duty.as_deref(),
        }
    }
}

/// A preset resolved at session creation and frozen in genesis.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedPreset {
    pub format: u64,
    pub base: PresetBase,
    pub layers: Vec<PresetLayer>,
    /// The session's model-facing tool selection (fixed tools and projected
    /// `mcp__<server>__<tool>` names), sorted. Absent keeps the workspace
    /// selection. Every launch meets it with the user policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    /// Registered MCP servers the selection references, by name, with the
    /// definition digest seen at creation (audit only) and the raw tool names.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp: Vec<PresetMcpRef>,
    /// Skill packages the session may discover and load; absent keeps all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    pub digest: String,
}

/// One referenced MCP server. The definition stays in the MCP registry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresetMcpRef {
    pub server: String,
    pub digest: String,
    pub tools: Vec<String>,
}

/// What a preset may reference when it is resolved. The defaults describe an
/// environment with no ceiling, no MCP servers and no skills.
pub trait PresetEnvironment {
    /// The user/deployment tool ceiling; `None` when nothing narrows it.
    fn permitted_tools(&self) -> Option<&BTreeSet<String>> {
        None
    }
    /// Definition digest of the MCP server registered under `server`.
    fn mcp_server_digest(&self, _server: &str) -> Option<String> {
        None
    }
    /// Model-facing name of an MCP tool, `None` when it cannot project.
    fn mcp_tool_name(&self, _server: &str, _tool: &str) -> Option<String> {
        None
    }
    /// Whether the instruction snapshot carries a skill package of this name.
    fn skill_known(&self, _name: &str) -> bool {
        false
    }
}

/// The environment without a ceiling, MCP servers or skills.
pub struct NoPresetEnvironment;

impl PresetEnvironment for NoPresetEnvironment {}

/// Why a preset was refused (`preset-invalid {reason, layer?, field?}`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresetInvalidReason {
    MultipleIdentity,
    UnknownKernelLayer,
    DuplicateLayer,
    TooManyLayers,
    InvalidId,
    InvalidIdentity,
    TextTooLarge,
    SecretInPreset,
    UnsupportedField,
    UnknownTool,
    RoleTool,
    ToolNotPermitted,
    UnknownMcpServer,
    UnknownMcpTool,
    UnknownSkill,
    TooManyEntries,
}

impl PresetInvalidReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultipleIdentity => "multiple-identity",
            Self::UnknownKernelLayer => "unknown-kernel-layer",
            Self::DuplicateLayer => "duplicate-layer",
            Self::TooManyLayers => "too-many-layers",
            Self::InvalidId => "invalid-id",
            Self::InvalidIdentity => "invalid-identity",
            Self::TextTooLarge => "text-too-large",
            Self::SecretInPreset => "secret-in-preset",
            Self::UnsupportedField => "unsupported-field",
            Self::UnknownTool => "unknown-tool",
            Self::RoleTool => "role-tool",
            Self::ToolNotPermitted => "tool-not-permitted",
            Self::UnknownMcpServer => "unknown-mcp-server",
            Self::UnknownMcpTool => "unknown-mcp-tool",
            Self::UnknownSkill => "unknown-skill",
            Self::TooManyEntries => "too-many-entries",
        }
    }

    pub const ALL: [Self; 16] = [
        Self::MultipleIdentity,
        Self::UnknownKernelLayer,
        Self::DuplicateLayer,
        Self::TooManyLayers,
        Self::InvalidId,
        Self::InvalidIdentity,
        Self::TextTooLarge,
        Self::SecretInPreset,
        Self::UnsupportedField,
        Self::UnknownTool,
        Self::RoleTool,
        Self::ToolNotPermitted,
        Self::UnknownMcpServer,
        Self::UnknownMcpTool,
        Self::UnknownSkill,
        Self::TooManyEntries,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresetInvalid {
    pub reason: PresetInvalidReason,
    /// The layer's id, or `#<index>` when the id itself is unusable.
    pub layer: Option<String>,
    /// JSON path of the offending member, such as `layers[1].duty`.
    pub field: Option<String>,
}

impl PresetInvalid {
    fn new(reason: PresetInvalidReason, layer: Option<String>, field: impl Into<String>) -> Self {
        Self {
            reason,
            layer,
            field: Some(field.into()),
        }
    }

    /// Error detail on the wire.
    #[must_use]
    pub fn detail(&self) -> Value {
        let mut detail = Map::new();
        detail.insert("reason".to_owned(), json!(self.reason.as_str()));
        if let Some(layer) = &self.layer {
            detail.insert("layer".to_owned(), json!(layer));
        }
        if let Some(field) = &self.field {
            detail.insert("field".to_owned(), json!(field));
        }
        Value::Object(detail)
    }
}

/// A preset request that is not even a well-formed preset (`bad-request`), or
/// a well-formed one the Kernel refuses (`preset-invalid`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PresetError {
    BadRequest(String),
    Invalid(PresetInvalid),
}

impl From<PresetInvalid> for PresetError {
    fn from(value: PresetInvalid) -> Self {
        Self::Invalid(value)
    }
}

/// Member names that carry an MCP definition or a credential. A preset only
/// references registered servers by id, so any of them is refused as
/// `secret-in-preset` rather than as an unknown field.
const SECRET_KEYS: [&str; 16] = [
    "command",
    "args",
    "env",
    "environment",
    "headers",
    "url",
    "transport",
    "oauth",
    "token",
    "apiKey",
    "api_key",
    "password",
    "secret",
    "credential",
    "credentials",
    "authorization",
];

/// Prefixes of well-known credential formats, matched at a token start.
const SECRET_PREFIXES: [(&str, usize); 8] = [
    ("sk-", 20),
    ("sk_live_", 16),
    ("ghp_", 30),
    ("github_pat_", 30),
    ("xoxb-", 20),
    ("xoxp-", 20),
    ("AKIA", 16),
    ("AIza", 30),
];

fn looks_like_secret(text: &str) -> bool {
    if text.contains("-----BEGIN") && text.contains("PRIVATE KEY") {
        return true;
    }
    text.split(|ch: char| ch.is_whitespace() || matches!(ch, '"' | '\'' | '`' | '(' | ')' | '='))
        .any(|token| {
            SECRET_PREFIXES.iter().any(|(prefix, min_tail)| {
                token.strip_prefix(prefix).is_some_and(|tail| {
                    tail.len() >= *min_tail
                        && tail
                            .chars()
                            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
                })
            })
        })
}

fn find_secret(value: &Value, path: &str) -> Option<String> {
    match value {
        Value::Object(object) => object.iter().find_map(|(key, member)| {
            let field = if path.is_empty() {
                key.clone()
            } else {
                format!("{path}.{key}")
            };
            if SECRET_KEYS.contains(&key.as_str()) {
                Some(field)
            } else {
                find_secret(member, &field)
            }
        }),
        Value::Array(items) => items
            .iter()
            .enumerate()
            .find_map(|(index, item)| find_secret(item, &format!("{path}[{index}]"))),
        Value::String(text) => looks_like_secret(text).then(|| path.to_owned()),
        _ => None,
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_PRESET_ID_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && value.as_bytes()[0].is_ascii_alphanumeric()
}

fn valid_version(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_PRESET_VERSION_BYTES
        && value.bytes().all(|byte| byte.is_ascii_graphic())
}

fn object_members<'a>(
    value: &'a Value,
    path: &str,
    allowed: &[&str],
) -> Result<&'a Map<String, Value>, PresetError> {
    let object = value
        .as_object()
        .ok_or_else(|| PresetError::BadRequest(format!("{path} must be an object")))?;
    if let Some(unknown) = object.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(PresetError::BadRequest(format!(
            "{path} has unknown member {unknown}"
        )));
    }
    Ok(object)
}

fn string_member<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<Option<&'a str>, PresetError> {
    match object.get(key) {
        None => Ok(None),
        Some(Value::String(text)) => Ok(Some(text)),
        Some(_) => Err(PresetError::BadRequest(format!(
            "{path}.{key} must be a string"
        ))),
    }
}

/// Layer members reserved for model defaults; refused as `unsupported-field`
/// until the Kernel applies them.
const RESERVED_MEMBERS: [&str; 1] = ["defaults"];

/// Tools, MCP references and skills declared across the request and its layers.
#[derive(Default)]
struct Selection {
    builtins: Option<BTreeSet<String>>,
    /// Server name to raw tool names, plus the path of its first mention.
    mcp: BTreeMap<String, (BTreeSet<String>, String, Option<String>)>,
    skills: Option<BTreeMap<String, (String, Option<String>)>>,
}

fn string_array<'a>(value: &'a Value, path: &str) -> Result<Vec<&'a str>, PresetError> {
    value
        .as_array()
        .and_then(|items| items.iter().map(Value::as_str).collect::<Option<Vec<_>>>())
        .ok_or_else(|| PresetError::BadRequest(format!("{path} must be an array of strings")))
}

impl Selection {
    fn collect(
        &mut self,
        object: &Map<String, Value>,
        path: &str,
        layer: Option<&str>,
    ) -> Result<(), PresetError> {
        let label = || layer.map(str::to_owned);
        if let Some(tools) = object.get("tools") {
            let manifest = BuiltinManifest::compiled();
            let interactive = manifest.interactive_names();
            let fixed = manifest.fixed_names();
            let builtins = self.builtins.get_or_insert_with(BTreeSet::new);
            for (index, name) in string_array(tools, &format!("{path}.tools"))?
                .into_iter()
                .enumerate()
            {
                let field = format!("{path}.tools[{index}]");
                if interactive.iter().any(|known| known == name) {
                    builtins.insert(name.to_owned());
                } else if fixed.contains(name) {
                    return Err(PresetInvalid::new(PresetReason::RoleTool, label(), field).into());
                } else {
                    return Err(
                        PresetInvalid::new(PresetReason::UnknownTool, label(), field).into(),
                    );
                }
            }
        }
        if let Some(mcp) = object.get("mcp") {
            let items = mcp
                .as_array()
                .ok_or_else(|| PresetError::BadRequest(format!("{path}.mcp must be an array")))?;
            for (index, item) in items.iter().enumerate() {
                let item_path = format!("{path}.mcp[{index}]");
                let entry = object_members(item, &item_path, &["server", "tools"])?;
                let server = string_member(entry, "server", &item_path)?
                    .filter(|server| !server.is_empty())
                    .ok_or_else(|| {
                        PresetError::BadRequest(format!("{item_path}.server is required"))
                    })?;
                let tools = string_array(
                    entry.get("tools").unwrap_or(&Value::Null),
                    &format!("{item_path}.tools"),
                )?;
                if tools.is_empty() || tools.iter().any(|tool| tool.is_empty()) {
                    return Err(PresetError::BadRequest(format!(
                        "{item_path}.tools must name at least one tool"
                    )));
                }
                let slot = self
                    .mcp
                    .entry(server.to_owned())
                    .or_insert_with(|| (BTreeSet::new(), format!("{item_path}.server"), label()));
                slot.0.extend(tools.into_iter().map(str::to_owned));
            }
        }
        if let Some(skills) = object.get("skills") {
            let known = self.skills.get_or_insert_with(BTreeMap::new);
            for (index, name) in string_array(skills, &format!("{path}.skills"))?
                .into_iter()
                .enumerate()
            {
                known
                    .entry(name.to_owned())
                    .or_insert_with(|| (format!("{path}.skills[{index}]"), label()));
            }
        }
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn finish(
        self,
        environment: &dyn PresetEnvironment,
    ) -> Result<(Option<Vec<String>>, Vec<PresetMcpRef>, Option<Vec<String>>), PresetError> {
        let too_many = |field: &str| {
            PresetError::from(PresetInvalid::new(
                PresetReason::TooManyEntries,
                None,
                field.to_owned(),
            ))
        };
        if self
            .builtins
            .as_ref()
            .is_some_and(|tools| tools.len() > MAX_PRESET_TOOLS)
        {
            return Err(too_many("preset.tools"));
        }
        if self.mcp.len() > MAX_PRESET_MCP_SERVERS {
            return Err(too_many("preset.mcp"));
        }
        if self
            .skills
            .as_ref()
            .is_some_and(|skills| skills.len() > MAX_PRESET_SKILLS)
        {
            return Err(too_many("preset.skills"));
        }
        let permitted = environment.permitted_tools();
        let check_permitted = |name: &str, field: &str, layer: Option<String>| {
            if permitted.is_some_and(|permitted| !permitted.contains(name)) {
                Err(PresetError::from(PresetInvalid::new(
                    PresetReason::ToolNotPermitted,
                    layer,
                    field.to_owned(),
                )))
            } else {
                Ok(())
            }
        };
        let mut projected = BTreeSet::new();
        let mut references = Vec::with_capacity(self.mcp.len());
        for (server, (tools, field, layer)) in self.mcp {
            if tools.len() > MAX_PRESET_MCP_TOOLS {
                return Err(too_many(&field));
            }
            let digest = environment.mcp_server_digest(&server).ok_or_else(|| {
                PresetInvalid::new(PresetReason::UnknownMcpServer, layer.clone(), field.clone())
            })?;
            for tool in &tools {
                let name = environment.mcp_tool_name(&server, tool).ok_or_else(|| {
                    PresetInvalid::new(PresetReason::UnknownMcpTool, layer.clone(), field.clone())
                })?;
                check_permitted(&name, &field, layer.clone())?;
                projected.insert(name);
            }
            references.push(PresetMcpRef {
                server,
                digest,
                tools: tools.into_iter().collect(),
            });
        }
        let tools = if self.builtins.is_some() || !projected.is_empty() {
            let builtins = match self.builtins {
                Some(builtins) => {
                    for name in &builtins {
                        check_permitted(name, "preset.tools", None)?;
                    }
                    builtins
                }
                None => BuiltinManifest::compiled()
                    .interactive_names()
                    .into_iter()
                    .collect(),
            };
            Some(
                builtins
                    .into_iter()
                    .chain(projected)
                    .collect::<BTreeSet<_>>(),
            )
        } else {
            None
        };
        let skills = match self.skills {
            None => None,
            Some(skills) => {
                if let Some((_, (field, layer))) = skills
                    .iter()
                    .find(|(name, _)| !environment.skill_known(name))
                {
                    return Err(PresetInvalid::new(
                        PresetReason::UnknownSkill,
                        layer.clone(),
                        field.clone(),
                    )
                    .into());
                }
                Some(skills.into_keys().collect())
            }
        };
        Ok((
            tools.map(|tools| tools.into_iter().collect()),
            references,
            skills,
        ))
    }
}

impl ResolvedPreset {
    /// Resolves a `session.create {preset}` request under the workspace base,
    /// checking its references against `environment`.
    pub fn resolve_request(
        request: &Value,
        base: PresetBase,
        environment: &dyn PresetEnvironment,
    ) -> Result<Self, PresetError> {
        if let Some(field) = find_secret(request, "preset") {
            return Err(PresetInvalid::new(PresetReason::SecretInPreset, None, field).into());
        }
        let object = object_members(request, "preset", &["layers", "tools", "mcp", "skills"])?;
        let mut selection = Selection::default();
        selection.collect(object, "preset", None)?;
        let requested = match object.get("layers") {
            None => &[][..],
            Some(Value::Array(items)) => items.as_slice(),
            Some(_) => {
                return Err(PresetError::BadRequest(
                    "preset.layers must be an array".to_owned(),
                ));
            }
        };
        if requested.len() > MAX_PRESET_LAYERS {
            return Err(
                PresetInvalid::new(PresetReason::TooManyLayers, None, "preset.layers").into(),
            );
        }
        let mut layers = Vec::with_capacity(requested.len());
        for (index, layer) in requested.iter().enumerate() {
            let resolved = resolve_layer(layer, index)?;
            if let (PresetLayer::Inline { id, .. }, Some(object)) = (&resolved, layer.as_object()) {
                selection.collect(object, &format!("preset.layers[{index}]"), Some(id))?;
            }
            layers.push(resolved);
        }
        let mut preset = Self::from_parts(base, layers, None, Vec::new(), None)?;
        let (tools, mcp, skills) = selection.finish(environment)?;
        preset.tools = tools;
        preset.mcp = mcp;
        preset.skills = skills;
        preset.digest = preset.compute_digest();
        Ok(preset)
    }

    /// The sugar a session without an explicit preset resolves to.
    #[must_use]
    pub fn from_identity(identity: Option<IdentityProfile>, base: PresetBase) -> Self {
        let layers = match identity {
            Some(IdentityProfile::Coding) => vec![PresetLayer::Kernel {
                id: KERNEL_LAYERS[0].id.to_owned(),
                version: KERNEL_LAYERS[0].version.to_owned(),
            }],
            Some(IdentityProfile::General) | None => Vec::new(),
        };
        Self::from_parts(base, layers, None, Vec::new(), None).expect("sugar presets are valid")
    }

    fn from_parts(
        base: PresetBase,
        layers: Vec<PresetLayer>,
        tools: Option<Vec<String>>,
        mcp: Vec<PresetMcpRef>,
        skills: Option<Vec<String>>,
    ) -> Result<Self, PresetError> {
        let mut seen = std::collections::BTreeSet::new();
        let mut identities = 0;
        for (index, layer) in layers.iter().enumerate() {
            if !seen.insert(layer.id().to_owned()) {
                return Err(PresetInvalid::new(
                    PresetReason::DuplicateLayer,
                    Some(layer.id().to_owned()),
                    format!("preset.layers[{index}].id"),
                )
                .into());
            }
            if matches!(
                layer,
                PresetLayer::Inline {
                    identity: Some(_),
                    ..
                }
            ) {
                identities += 1;
                if identities > 1 {
                    return Err(PresetInvalid::new(
                        PresetReason::MultipleIdentity,
                        Some(layer.id().to_owned()),
                        format!("preset.layers[{index}]"),
                    )
                    .into());
                }
            }
        }
        let mut preset = Self {
            format: PRESET_FORMAT,
            base,
            layers,
            tools,
            mcp,
            skills,
            digest: String::new(),
        };
        if preset
            .owned_sections()
            .iter()
            .map(|text| text.len())
            .sum::<usize>()
            > MAX_PRESET_TEXT_BYTES
        {
            return Err(PresetInvalid::new(PresetReason::TextTooLarge, None, "preset").into());
        }
        preset.digest = preset.compute_digest();
        Ok(preset)
    }

    /// `sha256:` over the RFC 8785 bytes of the preset without `digest`.
    fn compute_digest(&self) -> String {
        let mut preimage = serde_json::to_value(self).expect("preset is JSON");
        preimage
            .as_object_mut()
            .expect("preset object")
            .remove("digest");
        let canonical = serde_json_canonicalizer::to_vec(&preimage).expect("preset is JSON");
        // `sha256:` rather than the asset prefix `sha256-`: ledger rewrites treat
        // every `sha256-<hex>` string as an asset reference.
        format!("sha256:{:x}", Sha256::digest(canonical))
    }

    /// Reads and checks a genesis `preset` member: known format and base,
    /// known Kernel layers, at most one identity, and a matching digest.
    pub fn from_genesis(value: &Value) -> Result<Self, String> {
        let preset: Self = serde_json::from_value(value.clone())
            .map_err(|error| format!("invalid genesis preset: {error}"))?;
        if preset.format != PRESET_FORMAT {
            return Err(format!("unsupported preset format {}", preset.format));
        }
        if let Some(layer) = preset.layers.iter().find(
            |layer| matches!(layer, PresetLayer::Kernel { id, .. } if kernel_layer(id).is_none()),
        ) {
            return Err(format!("unknown Kernel layer {}", layer.id()));
        }
        let check = Self::from_parts(
            preset.base,
            preset.layers.clone(),
            preset.tools.clone(),
            preset.mcp.clone(),
            preset.skills.clone(),
        )
        .map_err(|error| format!("invalid genesis preset: {error:?}"))?;
        if check.digest != preset.digest {
            return Err("genesis preset digest mismatch".to_owned());
        }
        Ok(preset)
    }

    /// The preset-owned sections, `{model}` unresolved: identity line, base
    /// rules, then each layer's duty.
    fn owned_sections(&self) -> Vec<&str> {
        // An inline identity is the session's primary role; a Kernel layer's
        // identity applies only when no inline layer declares one.
        let inline = self.layers.iter().find_map(|layer| match layer {
            PresetLayer::Inline { identity, .. } => identity.as_deref(),
            PresetLayer::Kernel { .. } => None,
        });
        let identity = inline
            .or_else(|| self.layers.iter().find_map(PresetLayer::identity))
            .or_else(|| self.base.identity());
        identity
            .into_iter()
            .chain(self.base.rules())
            .chain(self.layers.iter().filter_map(PresetLayer::duty))
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .collect()
    }

    /// Harness identity and the preset-owned sections with `{model}`
    /// unresolved; `session.presets.resolve` returns this text.
    #[must_use]
    pub fn preset_text(&self) -> String {
        std::iter::once(HARNESS_IDENTITY)
            .chain(self.owned_sections())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// The root system instructions: harness identity, the preset-owned
    /// sections with `{model}` replaced, the working directory when one is
    /// bound, then the instruction snapshot text when it is nonempty.
    #[must_use]
    pub fn render(&self, model: &str, cwd: Option<&str>, instruction_text: &str) -> String {
        let mut sections = vec![HARNESS_IDENTITY.to_owned()];
        sections.extend(
            self.owned_sections()
                .into_iter()
                .map(|text| text.replace("{model}", model)),
        );
        if let Some(cwd) = cwd {
            sections.push(WORKING_DIRECTORY.replace("{cwd}", cwd));
        }
        let instruction_text = instruction_text.trim();
        if !instruction_text.is_empty() {
            sections.push(instruction_text.to_owned());
        }
        sections.join("\n\n")
    }

    /// Inventory summary: `{base, layers:[{kind,id,version}], digest}`.
    #[must_use]
    pub fn summary(&self) -> Value {
        json!({
            "base": self.base,
            "layers": self.layers.iter().map(|layer| json!({
                "kind": layer.kind(), "id": layer.id(), "version": layer.version()
            })).collect::<Vec<_>>(),
            "digest": self.digest,
        })
    }
}

type PresetReason = PresetInvalidReason;

fn resolve_layer(layer: &Value, index: usize) -> Result<PresetLayer, PresetError> {
    let path = format!("preset.layers[{index}]");
    let kind = layer
        .as_object()
        .and_then(|object| object.get("kind"))
        .and_then(Value::as_str)
        .ok_or_else(|| PresetError::BadRequest(format!("{path}.kind must be a string")))?;
    match kind {
        "kernel" => {
            let object = object_members(layer, &path, &["kind", "id"])?;
            let id = string_member(object, "id", &path)?
                .ok_or_else(|| PresetError::BadRequest(format!("{path}.id is required")))?;
            let known = kernel_layer(id).ok_or_else(|| {
                PresetInvalid::new(
                    PresetReason::UnknownKernelLayer,
                    Some(id.to_owned()),
                    format!("{path}.id"),
                )
            })?;
            Ok(PresetLayer::Kernel {
                id: known.id.to_owned(),
                version: known.version.to_owned(),
            })
        }
        "inline" => {
            let object = object_members(
                layer,
                &path,
                &[
                    "kind", "id", "version", "identity", "duty", "tools", "mcp", "skills",
                    "defaults",
                ],
            )?;
            let id = string_member(object, "id", &path)?
                .ok_or_else(|| PresetError::BadRequest(format!("{path}.id is required")))?;
            let label = || {
                Some(if valid_id(id) {
                    id.to_owned()
                } else {
                    format!("#{index}")
                })
            };
            if !valid_id(id) {
                return Err(PresetInvalid::new(
                    PresetReason::InvalidId,
                    label(),
                    format!("{path}.id"),
                )
                .into());
            }
            let version = string_member(object, "version", &path)?
                .ok_or_else(|| PresetError::BadRequest(format!("{path}.version is required")))?;
            if !valid_version(version) {
                return Err(PresetInvalid::new(
                    PresetReason::InvalidId,
                    label(),
                    format!("{path}.version"),
                )
                .into());
            }
            if let Some(reserved) = RESERVED_MEMBERS
                .iter()
                .find(|key| object.contains_key(**key))
            {
                return Err(PresetInvalid::new(
                    PresetReason::UnsupportedField,
                    label(),
                    format!("{path}.{reserved}"),
                )
                .into());
            }
            let identity = string_member(object, "identity", &path)?;
            if let Some(identity) = identity {
                if identity.len() > MAX_PRESET_IDENTITY_BYTES {
                    return Err(PresetInvalid::new(
                        PresetReason::TextTooLarge,
                        label(),
                        format!("{path}.identity"),
                    )
                    .into());
                }
                if identity.trim().is_empty() || identity.trim().contains('\n') {
                    return Err(PresetInvalid::new(
                        PresetReason::InvalidIdentity,
                        label(),
                        format!("{path}.identity"),
                    )
                    .into());
                }
            }
            let duty = string_member(object, "duty", &path)?;
            if duty.is_some_and(|duty| duty.len() > MAX_PRESET_DUTY_BYTES) {
                return Err(PresetInvalid::new(
                    PresetReason::TextTooLarge,
                    label(),
                    format!("{path}.duty"),
                )
                .into());
            }
            Ok(PresetLayer::Inline {
                id: id.to_owned(),
                version: version.to_owned(),
                identity: identity.map(|text| text.trim().to_owned()),
                duty: duty
                    .map(|text| text.trim().to_owned())
                    .filter(|text| !text.is_empty()),
            })
        }
        other => Err(PresetError::BadRequest(format!(
            "{path}.kind {other} is not kernel or inline"
        ))),
    }
}

/// `session.presets.describe` result.
#[must_use]
pub fn describe_value() -> Value {
    json!({
        "format": PRESET_FORMAT,
        "renderVersion": PRESET_RENDER_VERSION,
        "bases": [
            {"id": "general", "identity": GENERAL_IDENTITY.trim(), "rules": GENERAL_RULES.trim()},
            {"id": "blank"}
        ],
        "kernelLayers": KERNEL_LAYERS.iter().map(|layer| json!({
            "kind": "kernel", "id": layer.id, "version": layer.version, "title": layer.title,
            "identity": layer.identity.trim(), "duty": layer.duty.trim()
        })).collect::<Vec<_>>(),
        "limits": {
            "maxLayers": MAX_PRESET_LAYERS,
            "maxIdBytes": MAX_PRESET_ID_BYTES,
            "maxVersionBytes": MAX_PRESET_VERSION_BYTES,
            "maxIdentityBytes": MAX_PRESET_IDENTITY_BYTES,
            "maxDutyBytes": MAX_PRESET_DUTY_BYTES,
            "maxTextBytes": MAX_PRESET_TEXT_BYTES,
            "maxTools": MAX_PRESET_TOOLS,
            "maxMcpServers": MAX_PRESET_MCP_SERVERS,
            "maxMcpToolsPerServer": MAX_PRESET_MCP_TOOLS,
            "maxSkills": MAX_PRESET_SKILLS
        },
        "invalidReasons": PresetInvalidReason::ALL.iter().map(|reason| reason.as_str()).collect::<Vec<_>>(),
        // Fixed tools a preset may select (`tools`); every other fixed name is a
        // role tool the worker grants by role.
        "selectableTools": BuiltinManifest::compiled().interactive_names(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guidance::root_system_instructions;

    fn resolve(request: Value) -> Result<ResolvedPreset, PresetError> {
        ResolvedPreset::resolve_request(&request, PresetBase::General, &NoPresetEnvironment)
    }

    struct Fixture {
        permitted: Option<BTreeSet<String>>,
    }

    impl PresetEnvironment for Fixture {
        fn permitted_tools(&self) -> Option<&BTreeSet<String>> {
            self.permitted.as_ref()
        }
        fn mcp_server_digest(&self, server: &str) -> Option<String> {
            (server == "docs").then(|| "sha256:docs".to_owned())
        }
        fn mcp_tool_name(&self, server: &str, tool: &str) -> Option<String> {
            (!tool.contains('/')).then(|| format!("mcp__{server}__{tool}"))
        }
        fn skill_known(&self, name: &str) -> bool {
            name == "style-guide"
        }
    }

    fn resolve_in(
        request: Value,
        permitted: Option<&[&str]>,
    ) -> Result<ResolvedPreset, PresetError> {
        ResolvedPreset::resolve_request(
            &request,
            PresetBase::General,
            &Fixture {
                permitted: permitted
                    .map(|names| names.iter().map(|name| (*name).to_owned()).collect()),
            },
        )
    }

    fn invalid(request: Value) -> PresetInvalid {
        match resolve(request) {
            Err(PresetError::Invalid(invalid)) => invalid,
            other => panic!("expected preset-invalid, got {other:?}"),
        }
    }

    #[test]
    fn sugar_renders_exactly_like_the_identity_profile() {
        for profile in [IdentityProfile::Coding, IdentityProfile::General] {
            for cwd in [None, Some("/work")] {
                let preset = ResolvedPreset::from_identity(Some(profile), PresetBase::General);
                assert_eq!(
                    preset.render("deepseek-flash", cwd, " rule \n"),
                    root_system_instructions(profile, "deepseek-flash", cwd, " rule \n")
                );
            }
        }
        let coding = resolve(json!({"layers":[{"kind":"kernel","id":"coding"}]})).unwrap();
        assert_eq!(
            coding,
            ResolvedPreset::from_identity(Some(IdentityProfile::Coding), PresetBase::General)
        );
    }

    #[test]
    fn blank_base_drops_identity_and_rules() {
        let empty = ResolvedPreset::from_identity(None, PresetBase::Blank);
        assert_eq!(
            empty.render("m", Some("/w"), ""),
            format!("{HARNESS_IDENTITY}\n\nYour working directory is /w.")
        );
        let coding =
            ResolvedPreset::from_identity(Some(IdentityProfile::Coding), PresetBase::Blank);
        assert_eq!(
            coding.render("m", None, ""),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a coding agent powered by the m model.\n\n{}",
                CODING_RULES.trim()
            )
        );
    }

    #[test]
    fn inline_identity_replaces_the_base_identity_but_not_its_rules() {
        let preset = resolve(json!({"layers":[
            {"kind":"kernel","id":"coding"},
            {"kind":"inline","id":"domain-expert","version":"3","duty":"Prefer the domain tools."}
        ]}))
        .unwrap();
        assert!(
            preset
                .render("m", None, "")
                .ends_with("Prefer the domain tools.")
        );
        let expert = resolve(json!({"layers":[{"kind":"inline","id":"domain-expert","version":"3",
            "identity":"You are a domain expert agent powered by the {model} model.","duty":"Prefer the domain tools."}]}))
        .unwrap();
        assert_eq!(
            expert.render("m", None, ""),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a domain expert agent powered by the m model.\n\n{}\n\nPrefer the domain tools.",
                GENERAL_RULES.trim()
            )
        );
        assert_eq!(
            expert.preset_text(),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a domain expert agent powered by the {{model}} model.\n\n{}\n\nPrefer the domain tools.",
                GENERAL_RULES.trim()
            )
        );
    }

    #[test]
    fn inline_identity_wins_over_a_kernel_layer_identity() {
        let preset = resolve(json!({"layers":[{"kind":"kernel","id":"coding"},
            {"kind":"inline","id":"domain-expert","version":"1",
             "identity":"You are a domain expert agent powered by the {model} model.","duty":"Prefer the domain tools."}]}))
        .unwrap();
        assert_eq!(
            preset.render("m", None, ""),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a domain expert agent powered by the m model.\n\n{}\n\n{}\n\nPrefer the domain tools.",
                GENERAL_RULES.trim(),
                CODING_RULES.trim()
            )
        );
    }

    #[test]
    fn refusals_name_reason_layer_and_field() {
        let two = invalid(json!({"layers":[
            {"kind":"inline","id":"expert","version":"1","identity":"You are a domain agent."},
            {"kind":"inline","id":"video","version":"1","identity":"You are a video agent."}]}));
        assert_eq!(two.reason, PresetReason::MultipleIdentity);
        assert_eq!(two.layer.as_deref(), Some("video"));
        let duplicate = invalid(
            json!({"layers":[{"kind":"kernel","id":"coding"},{"kind":"kernel","id":"coding"}]}),
        );
        assert_eq!(duplicate.reason, PresetReason::DuplicateLayer);
        assert_eq!(
            invalid(json!({"layers":[{"kind":"kernel","id":"video"}]})).reason,
            PresetReason::UnknownKernelLayer
        );
        let bad_id = invalid(json!({"layers":[{"kind":"inline","id":"Expert","version":"1"}]}));
        assert_eq!(
            (bad_id.reason, bad_id.layer.as_deref()),
            (PresetReason::InvalidId, Some("#0"))
        );
        assert_eq!(
            invalid(
                json!({"layers":[{"kind":"inline","id":"a","version":"1","identity":"two\nlines"}]})
            )
            .reason,
            PresetReason::InvalidIdentity
        );
        let many = (0..9)
            .map(|index| json!({"kind":"inline","id":format!("l{index}"),"version":"1"}))
            .collect::<Vec<_>>();
        assert_eq!(
            invalid(json!({"layers":many})).reason,
            PresetReason::TooManyLayers
        );
        let long = "x".repeat(MAX_PRESET_DUTY_BYTES + 1);
        assert_eq!(
            invalid(json!({"layers":[{"kind":"inline","id":"a","version":"1","duty":long}]})).field,
            Some("preset.layers[0].duty".to_owned())
        );
        let total = (0..3)
            .map(|index| json!({"kind":"inline","id":format!("l{index}"),"version":"1","duty":"y".repeat(MAX_PRESET_DUTY_BYTES)}))
            .collect::<Vec<_>>();
        assert_eq!(
            invalid(json!({"layers":total})).reason,
            PresetReason::TextTooLarge
        );
        assert_eq!(
            invalid(json!({"layers":[{"kind":"inline","id":"a","version":"1","defaults":{}}]}))
                .reason,
            PresetReason::UnsupportedField
        );
    }

    #[test]
    fn tools_mcp_and_skills_resolve_against_the_environment() {
        let preset = resolve_in(
            json!({"tools":["read"],"layers":[{"kind":"inline","id":"expert","version":"1",
                "tools":["grep"],"mcp":[{"server":"docs","tools":["open","draw"]}],
                "skills":["style-guide"]}]}),
            None,
        )
        .unwrap();
        assert_eq!(
            preset.tools,
            Some(vec![
                "grep".to_owned(),
                "mcp__docs__draw".to_owned(),
                "mcp__docs__open".to_owned(),
                "read".to_owned()
            ])
        );
        assert_eq!(
            preset.mcp,
            vec![PresetMcpRef {
                server: "docs".to_owned(),
                digest: "sha256:docs".to_owned(),
                tools: vec!["draw".to_owned(), "open".to_owned()],
            }]
        );
        assert_eq!(preset.skills, Some(vec!["style-guide".to_owned()]));
        let value = serde_json::to_value(&preset).unwrap();
        assert_eq!(ResolvedPreset::from_genesis(&value).unwrap(), preset);

        // MCP alone keeps the interactive fixed tools; no selection keeps the workspace's.
        let mcp_only =
            resolve_in(json!({"mcp":[{"server":"docs","tools":["open"]}]}), None).unwrap();
        let tools = mcp_only.tools.unwrap();
        assert!(
            tools.contains(&"shell".to_owned()) && tools.contains(&"mcp__docs__open".to_owned())
        );
        assert_eq!(resolve_in(json!({"layers":[]}), None).unwrap().tools, None);
        // A preset without selections keeps the digest it had before selections existed.
        assert_eq!(
            resolve(json!({"layers":[{"kind":"kernel","id":"coding"}]}))
                .unwrap()
                .digest,
            ResolvedPreset::from_identity(Some(IdentityProfile::Coding), PresetBase::General)
                .digest
        );

        let reason =
            |request: Value, permitted: Option<&[&str]>| match resolve_in(request, permitted) {
                Err(PresetError::Invalid(invalid)) => (invalid.reason, invalid.field),
                other => panic!("expected preset-invalid, got {other:?}"),
            };
        assert_eq!(
            reason(json!({"tools":["teleport"]}), None).0,
            PresetReason::UnknownTool
        );
        assert_eq!(
            reason(json!({"tools":["report"]}), None),
            (PresetReason::RoleTool, Some("preset.tools[0]".to_owned()))
        );
        assert_eq!(
            reason(json!({"tools":["read","shell"]}), Some(&["read"])).0,
            PresetReason::ToolNotPermitted
        );
        assert_eq!(
            reason(json!({"mcp":[{"server":"pdf","tools":["x"]}]}), None),
            (
                PresetReason::UnknownMcpServer,
                Some("preset.mcp[0].server".to_owned())
            )
        );
        assert_eq!(
            reason(json!({"mcp":[{"server":"docs","tools":["a/b"]}]}), None).0,
            PresetReason::UnknownMcpTool
        );
        assert_eq!(
            reason(
                json!({"mcp":[{"server":"docs","tools":["open"]}]}),
                Some(&["read"])
            )
            .0,
            PresetReason::ToolNotPermitted
        );
        assert_eq!(
            reason(json!({"skills":["nope"]}), None).0,
            PresetReason::UnknownSkill
        );
        let many = (0..=MAX_PRESET_MCP_SERVERS)
            .map(|index| json!({"server":format!("s{index}"),"tools":["t"]}))
            .collect::<Vec<_>>();
        assert_eq!(
            reason(json!({"mcp":many}), None).0,
            PresetReason::TooManyEntries
        );
        for request in [
            json!({"tools":"read"}),
            json!({"mcp":[{"server":"docs"}]}),
            json!({"mcp":[{"server":"docs","tools":[]}]}),
            json!({"layers":[{"kind":"kernel","id":"coding","tools":["read"]}]}),
        ] {
            assert!(
                matches!(
                    resolve_in(request.clone(), None),
                    Err(PresetError::BadRequest(_))
                ),
                "{request}"
            );
        }
    }

    #[test]
    fn secrets_and_definitions_are_refused_before_shape_checks() {
        let definition = invalid(json!({"mcp":[{"server":"pdf","command":["/bin/x"]}]}));
        assert_eq!(definition.reason, PresetReason::SecretInPreset);
        assert_eq!(definition.field.as_deref(), Some("preset.mcp[0].command"));
        let token = invalid(json!({"layers":[{"kind":"inline","id":"a","version":"1",
            "duty":"Use key sk-abcdefghijklmnopqrstuvwxyz0123 for calls."}]}));
        assert_eq!(token.reason, PresetReason::SecretInPreset);
        assert!(
            resolve(json!({"layers":[{"kind":"inline","id":"a","version":"1",
            "duty":"Ask the user for an sk- key when needed."}]}))
            .is_ok()
        );
    }

    #[test]
    fn shape_errors_are_bad_requests() {
        for request in [
            json!([]),
            json!({"layers":{}}),
            json!({"extra":true}),
            json!({"layers":[{"kind":"remote","id":"a"}]}),
            json!({"layers":[{"kind":"kernel","id":"coding","version":"1"}]}),
            json!({"layers":[{"kind":"inline","id":"a"}]}),
            json!({"layers":[{"kind":"inline","id":"a","version":"1","duty":7}]}),
        ] {
            assert!(
                matches!(resolve(request.clone()), Err(PresetError::BadRequest(_))),
                "{request}"
            );
        }
    }

    #[test]
    fn genesis_round_trip_checks_digest_and_layers() {
        let preset = resolve(json!({"layers":[{"kind":"kernel","id":"coding"},
            {"kind":"inline","id":"expert","version":"1","duty":"Domain."}]}))
        .unwrap();
        let value = serde_json::to_value(&preset).unwrap();
        assert_eq!(ResolvedPreset::from_genesis(&value).unwrap(), preset);
        let mut tampered = value.clone();
        tampered["layers"][1]["duty"] = json!("Other.");
        assert!(ResolvedPreset::from_genesis(&tampered).is_err());
        let mut future = value;
        future["format"] = json!(2);
        assert!(ResolvedPreset::from_genesis(&future).is_err());
        assert_eq!(
            preset.summary(),
            json!({"base":"general","layers":[
                {"kind":"kernel","id":"coding","version":"1"},
                {"kind":"inline","id":"expert","version":"1"}],"digest":preset.digest})
        );
    }
}
