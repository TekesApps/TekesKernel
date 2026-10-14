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

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

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
    pub digest: String,
}

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
        }
    }

    pub const ALL: [Self; 9] = [
        Self::MultipleIdentity,
        Self::UnknownKernelLayer,
        Self::DuplicateLayer,
        Self::TooManyLayers,
        Self::InvalidId,
        Self::InvalidIdentity,
        Self::TextTooLarge,
        Self::SecretInPreset,
        Self::UnsupportedField,
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

/// Members reserved for session-level tools, MCP references, skills and model
/// defaults; they are refused as `unsupported-field` until the Kernel applies
/// them.
const RESERVED_MEMBERS: [&str; 4] = ["tools", "mcp", "skills", "defaults"];

impl ResolvedPreset {
    /// Resolves a `session.create {preset}` request under the workspace base.
    pub fn resolve_request(request: &Value, base: PresetBase) -> Result<Self, PresetError> {
        if let Some(field) = find_secret(request, "preset") {
            return Err(PresetInvalid::new(PresetReason::SecretInPreset, None, field).into());
        }
        let object = object_members(request, "preset", &["layers", "tools", "mcp", "skills"])?;
        if let Some(reserved) = RESERVED_MEMBERS
            .iter()
            .find(|key| object.contains_key(**key))
        {
            return Err(PresetInvalid::new(
                PresetReason::UnsupportedField,
                None,
                format!("preset.{reserved}"),
            )
            .into());
        }
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
            layers.push(resolve_layer(layer, index)?);
        }
        Self::from_layers(base, layers)
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
        Self::from_layers(base, layers).expect("sugar presets are valid")
    }

    fn from_layers(base: PresetBase, layers: Vec<PresetLayer>) -> Result<Self, PresetError> {
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

    fn compute_digest(&self) -> String {
        let preimage = json!({"format":self.format,"base":self.base,"layers":self.layers});
        let canonical = serde_json_canonicalizer::to_vec(&preimage).expect("preset is JSON");
        format!("sha256-{:x}", Sha256::digest(canonical))
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
        let check = Self::from_layers(preset.base, preset.layers.clone())
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
            "maxTextBytes": MAX_PRESET_TEXT_BYTES
        },
        "invalidReasons": PresetInvalidReason::ALL.iter().map(|reason| reason.as_str()).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guidance::root_system_instructions;

    fn resolve(request: Value) -> Result<ResolvedPreset, PresetError> {
        ResolvedPreset::resolve_request(&request, PresetBase::General)
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
            {"kind":"inline","id":"cad-expert","version":"3","duty":"Prefer DXF tools."}
        ]}))
        .unwrap();
        assert!(preset.render("m", None, "").ends_with("Prefer DXF tools."));
        let cad = resolve(json!({"layers":[{"kind":"inline","id":"cad-expert","version":"3",
            "identity":"You are a CAD expert agent powered by the {model} model.","duty":"Prefer DXF tools."}]}))
        .unwrap();
        assert_eq!(
            cad.render("m", None, ""),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a CAD expert agent powered by the m model.\n\n{}\n\nPrefer DXF tools.",
                GENERAL_RULES.trim()
            )
        );
        assert_eq!(
            cad.preset_text(),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a CAD expert agent powered by the {{model}} model.\n\n{}\n\nPrefer DXF tools.",
                GENERAL_RULES.trim()
            )
        );
    }

    #[test]
    fn inline_identity_wins_over_a_kernel_layer_identity() {
        let preset = resolve(json!({"layers":[{"kind":"kernel","id":"coding"},
            {"kind":"inline","id":"cad-expert","version":"1",
             "identity":"You are a CAD expert agent powered by the {model} model.","duty":"Prefer DXF tools."}]}))
        .unwrap();
        assert_eq!(
            preset.render("m", None, ""),
            format!(
                "{HARNESS_IDENTITY}\n\nYou are a CAD expert agent powered by the m model.\n\n{}\n\n{}\n\nPrefer DXF tools.",
                GENERAL_RULES.trim(),
                CODING_RULES.trim()
            )
        );
    }

    #[test]
    fn refusals_name_reason_layer_and_field() {
        let two = invalid(json!({"layers":[
            {"kind":"inline","id":"cad","version":"1","identity":"You are a CAD agent."},
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
        let bad_id = invalid(json!({"layers":[{"kind":"inline","id":"CAD","version":"1"}]}));
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
        for reserved in ["tools", "mcp", "skills"] {
            let refused = invalid(json!({reserved: []}));
            assert_eq!(refused.reason, PresetReason::UnsupportedField);
            assert_eq!(refused.field, Some(format!("preset.{reserved}")));
        }
        assert_eq!(
            invalid(json!({"layers":[{"kind":"inline","id":"a","version":"1","defaults":{}}]}))
                .reason,
            PresetReason::UnsupportedField
        );
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
            {"kind":"inline","id":"cad","version":"1","duty":"DXF."}]}))
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
                {"kind":"inline","id":"cad","version":"1"}],"digest":preset.digest})
        );
    }
}
