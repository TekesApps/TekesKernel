use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::OnceLock;

use profile::{Model, Provider, SessionSettings};
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::AdapterId;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialectId {
    OpenaiResponsesV1,
    DeepseekResponsesV1,
    GenericChatV1,
    OpenaiChatV1,
    DeepseekChatV1,
    KimiChatV1,
    GlmChatV1,
    OllamaChatV1,
    AnthropicMessagesV1,
    DeepseekAnthropicV1,
    GoogleGenerationV1,
    GoogleInteractionsV1,
}

impl DialectId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenaiResponsesV1 => "openai_responses_v1",
            Self::DeepseekResponsesV1 => "deepseek_responses_v1",
            Self::GenericChatV1 => "generic_chat_v1",
            Self::OpenaiChatV1 => "openai_chat_v1",
            Self::DeepseekChatV1 => "deepseek_chat_v1",
            Self::KimiChatV1 => "kimi_chat_v1",
            Self::GlmChatV1 => "glm_chat_v1",
            Self::OllamaChatV1 => "ollama_chat_v1",
            Self::AnthropicMessagesV1 => "anthropic_messages_v1",
            Self::DeepseekAnthropicV1 => "deepseek_anthropic_v1",
            Self::GoogleGenerationV1 => "google_generation_v1",
            Self::GoogleInteractionsV1 => "google_interactions_v1",
        }
    }

    #[must_use]
    pub const fn family(self) -> AdapterId {
        match self {
            Self::OpenaiResponsesV1 | Self::DeepseekResponsesV1 => AdapterId::Responses,
            Self::GenericChatV1
            | Self::OpenaiChatV1
            | Self::DeepseekChatV1
            | Self::KimiChatV1
            | Self::GlmChatV1
            | Self::OllamaChatV1 => AdapterId::ChatCompletion,
            Self::AnthropicMessagesV1 | Self::DeepseekAnthropicV1 => AdapterId::Anthropic,
            Self::GoogleGenerationV1 => AdapterId::GoogleGeneration,
            Self::GoogleInteractionsV1 => AdapterId::GoogleInteractions,
        }
    }

    #[must_use]
    pub const fn server_managed(self) -> bool {
        matches!(self, Self::OpenaiResponsesV1 | Self::GoogleInteractionsV1)
    }

    #[must_use]
    pub const fn input_blocks(self) -> &'static [&'static str] {
        match self {
            Self::OpenaiResponsesV1 | Self::GoogleGenerationV1 => {
                &["text", "image", "file", "tool_call", "tool_result"]
            }
            Self::AnthropicMessagesV1 => &["text", "image", "file", "tool_call", "tool_result"],
            Self::GenericChatV1 | Self::OpenaiChatV1 => {
                &["text", "image", "tool_call", "tool_result"]
            }
            Self::DeepseekResponsesV1
            | Self::DeepseekChatV1
            | Self::KimiChatV1
            | Self::GlmChatV1
            | Self::OllamaChatV1
            | Self::DeepseekAnthropicV1
            | Self::GoogleInteractionsV1 => &["text", "tool_call", "tool_result"],
        }
    }

    #[must_use]
    pub const fn supports_reasoning_blocks(self) -> bool {
        !matches!(
            self,
            Self::GenericChatV1
                | Self::OpenaiChatV1
                | Self::OllamaChatV1
                | Self::AnthropicMessagesV1
                | Self::DeepseekAnthropicV1
        )
    }

    #[must_use]
    pub const fn cache_policy(self) -> &'static str {
        match self {
            Self::OpenaiResponsesV1 | Self::GoogleInteractionsV1 => "server_managed",
            Self::DeepseekChatV1 | Self::KimiChatV1 | Self::GlmChatV1 => "implicit_prefix",
            Self::AnthropicMessagesV1 => "explicit_breakpoint",
            _ => "none",
        }
    }

    #[must_use]
    pub const fn sampling_policy(self) -> &'static str {
        "omitted"
    }

    #[must_use]
    pub const fn schema_policy(self) -> &'static str {
        if matches!(self, Self::OpenaiResponsesV1 | Self::OpenaiChatV1) {
            "strict_object"
        } else {
            "validated_object"
        }
    }

    #[must_use]
    pub const fn tool_choice_modes(self) -> &'static [&'static str] {
        &["auto"]
    }

    #[must_use]
    pub const fn tool_choice_wire(self) -> &'static str {
        "implicit"
    }

    #[must_use]
    pub const fn repair_id(self) -> &'static str {
        if matches!(self, Self::DeepseekResponsesV1 | Self::DeepseekChatV1) {
            "deepseek_v4_arguments_v1"
        } else {
            "none"
        }
    }
}

impl FromStr for DialectId {
    type Err = DialectError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "openai_responses_v1" => Ok(Self::OpenaiResponsesV1),
            "deepseek_responses_v1" => Ok(Self::DeepseekResponsesV1),
            "generic_chat_v1" => Ok(Self::GenericChatV1),
            "openai_chat_v1" => Ok(Self::OpenaiChatV1),
            "deepseek_chat_v1" => Ok(Self::DeepseekChatV1),
            "kimi_chat_v1" => Ok(Self::KimiChatV1),
            "glm_chat_v1" => Ok(Self::GlmChatV1),
            "ollama_chat_v1" => Ok(Self::OllamaChatV1),
            "anthropic_messages_v1" => Ok(Self::AnthropicMessagesV1),
            "deepseek_anthropic_v1" => Ok(Self::DeepseekAnthropicV1),
            "google_generation_v1" => Ok(Self::GoogleGenerationV1),
            "google_interactions_v1" => Ok(Self::GoogleInteractionsV1),
            other => Err(DialectError::UnknownDialect(other.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteEvidence {
    pub endpoint_owner: String,
    pub gateway_translation: String,
    pub exact_sku: String,
    pub evidence_revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderTarget {
    pub protocol_family: String,
    pub dialect_id: String,
    pub model_profile_id: String,
    pub route: RouteEvidence,
}

impl ProviderTarget {
    pub fn dialect(&self) -> Result<DialectId, DialectError> {
        DialectId::from_str(&self.dialect_id)
    }

    pub fn family(&self) -> Result<AdapterId, DialectError> {
        let dialect = self.dialect()?;
        let declared = match self.protocol_family.as_str() {
            "chat_completions" => AdapterId::ChatCompletion,
            "anthropic_messages" => AdapterId::Anthropic,
            other => AdapterId::from_str(other)
                .map_err(|_| DialectError::UnknownFamily(self.protocol_family.clone()))?,
        };
        if declared != dialect.family() {
            return Err(DialectError::FamilyMismatch);
        }
        Ok(declared)
    }
}

struct Definition {
    dialect: DialectId,
    family: &'static str,
    serializer_revision: &'static str,
}

/// Secret-free, executable proof-registry row exposed to management surfaces.
/// Every field is copied from the independently reviewed proof oracle; callers
/// must not infer route identity from an endpoint or display/model id.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvertisedDialectProof {
    pub proof_id: String,
    pub protocol_family: String,
    pub dialect_id: String,
    pub model_profile_id: String,
    pub endpoint_owner: String,
    pub gateway_translation: String,
    pub endpoint: String,
    pub exact_sku: String,
    pub evidence_revision: String,
    pub credential_header: String,
    pub credential_prefix: String,
}

const PROOF_ORACLE: &[u8] =
    include_bytes!("../../../fixtures/provider-dialects/profiles.canonical.json");
// Updated only with an independently reviewed canonical oracle change.
const PROOF_ORACLE_SHA256: &str =
    "2c42a8b0a07acb6915fb9c25a5fd113f81a78af01ece3c93d6684f9ea5a53912";

const MODEL_CAPABILITY_CATALOG: &[u8] =
    include_bytes!("../../../fixtures/provider-dialects/model-capabilities.canonical.json");
const MODEL_CAPABILITY_CATALOG_SHA256: &str =
    "c5f8926749325403cca2c0f1fa7e10a0a6cb93095550df78bad0a84b6b7bd1c0";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelReasoningCapability {
    levels: Vec<String>,
    default: Option<String>,
    /// Wire shape the thinking control takes on this exact model. Absent means
    /// the model exposes no thinking control (levels must then be empty).
    #[serde(default)]
    thinking: Option<ThinkingWire>,
}

/// How a model's thinking control is spelled on the wire. Declared per exact
/// SKU in the reviewed capability catalog; never inferred from the SKU string.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingWire {
    /// Anthropic 4.6+ generations: `{"type":"adaptive","display":"summarized"}`
    /// plus `output_config.effort`; `budget_tokens` is rejected.
    Adaptive,
    /// Pre-4.6 Anthropic generations: `{"type":"enabled","budget_tokens":N}`.
    Budget,
}

/// Native deferred-tool protocol a model speaks on an exact route. Declared in
/// the reviewed capability catalog only after a live adapter-direct gate passed
/// on that provider/model/API path; never inferred from the adapter family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeDeferredMode {
    /// OpenAI Responses client-executed `tool_search` with `tool_search_output`
    /// replay carrying loaded schemas (client-managed replay, `store=false`).
    OpenaiClientToolSearch,
    /// Anthropic Messages `defer_loading` declarations and custom `tool_search`
    /// whose bound result replays as `tool_reference` blocks.
    AnthropicCustomToolReference,
}

impl NativeDeferredMode {
    /// The only dialect a mode is proved on.
    #[must_use]
    pub const fn dialect(self) -> DialectId {
        match self {
            Self::OpenaiClientToolSearch => DialectId::OpenaiResponsesV1,
            Self::AnthropicCustomToolReference => DialectId::AnthropicMessagesV1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeDeferredRoute {
    endpoint_owner: String,
    gateway_translation: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeDeferredCapability {
    mode: NativeDeferredMode,
    routes: Vec<NativeDeferredRoute>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelCapabilityProfile {
    dialect_id: String,
    model_profile_id: String,
    exact_sku: String,
    /// Explicit wire target for a local named variant; never infer by splitting IDs.
    #[serde(default)]
    wire_model: Option<String>,
    #[serde(default)]
    pro_reasoning: bool,
    reasoning: ModelReasoningCapability,
    evidence_url: Option<String>,
    #[serde(default)]
    native_deferred_tools: Option<NativeDeferredCapability>,
    /// False when the provider rejects `tool_choice` `any`/`tool` for this
    /// model (Anthropic Fable 5.1); a required choice then degrades to `auto`.
    #[serde(default = "default_true")]
    forced_tool_choice: bool,
    /// True when the model may answer `stop_reason: refusal` and the provider
    /// offers server-side fallbacks for it (`fallbacks: "default"`).
    #[serde(default)]
    refusal_fallback: bool,
    /// Output ceiling sent as `max_tokens` when set; otherwise the adapter's
    /// streaming-aware default applies.
    #[serde(default)]
    max_output_tokens: Option<u64>,
}

const fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelCapabilityCatalog {
    format: u64,
    profiles: Vec<ModelCapabilityProfile>,
}

static MODEL_CAPABILITIES: OnceLock<Result<Vec<ModelCapabilityProfile>, String>> = OnceLock::new();

struct ProofRegistry {
    all: Vec<AdvertisedDialectProof>,
    advertised: Vec<AdvertisedDialectProof>,
}

static PROOF_REGISTRY: OnceLock<Result<ProofRegistry, String>> = OnceLock::new();

const DEFINITIONS: &[Definition] = &[
    Definition {
        dialect: DialectId::OpenaiResponsesV1,
        family: "responses",
        serializer_revision: "openai-responses-serializer-1",
    },
    Definition {
        dialect: DialectId::DeepseekResponsesV1,
        family: "responses",
        serializer_revision: "deepseek-responses-serializer-1",
    },
    Definition {
        dialect: DialectId::GenericChatV1,
        family: "chat_completions",
        serializer_revision: "generic-chat-serializer-1",
    },
    Definition {
        dialect: DialectId::OpenaiChatV1,
        family: "chat_completions",
        serializer_revision: "openai-chat-serializer-1",
    },
    Definition {
        dialect: DialectId::DeepseekChatV1,
        family: "chat_completions",
        serializer_revision: "deepseek-chat-serializer-1",
    },
    Definition {
        dialect: DialectId::KimiChatV1,
        family: "chat_completions",
        serializer_revision: "kimi-chat-serializer-1",
    },
    Definition {
        dialect: DialectId::GlmChatV1,
        family: "chat_completions",
        serializer_revision: "glm-chat-serializer-1",
    },
    Definition {
        dialect: DialectId::OllamaChatV1,
        family: "chat_completions",
        serializer_revision: "ollama-chat-serializer-2",
    },
    Definition {
        dialect: DialectId::AnthropicMessagesV1,
        family: "anthropic_messages",
        serializer_revision: "anthropic-messages-serializer-2",
    },
    Definition {
        dialect: DialectId::DeepseekAnthropicV1,
        family: "anthropic_messages",
        serializer_revision: "deepseek-anthropic-serializer-1",
    },
    Definition {
        dialect: DialectId::GoogleGenerationV1,
        family: "google_generation",
        serializer_revision: "google-generation-serializer-1",
    },
    Definition {
        dialect: DialectId::GoogleInteractionsV1,
        family: "google_interactions",
        serializer_revision: "google-interactions-serializer-1",
    },
];

#[derive(Clone, Debug)]
pub struct ResolvedDialectProfile {
    pub target: ProviderTarget,
    pub dialect: DialectId,
    pub serializer_revision: &'static str,
    pub credential_header: String,
    pub credential_prefix: String,
    /// True only when the exact target and configured endpoint match an advertised proof.
    /// False still represents an executable, configured dialect route.
    pub proof_verified: bool,
    /// Canonical endpoint carried by an exact target proof, when one exists. Request preparation
    /// may use its missing path prefix when the configured endpoint has the same origin.
    pub proved_endpoint: Option<String>,
    reasoning_efforts: Vec<String>,
    default_reasoning_effort: Option<String>,
    native_deferred_tools: Option<NativeDeferredMode>,
    thinking_wire: Option<ThinkingWire>,
    forced_tool_choice: bool,
    refusal_fallback: bool,
    max_output_tokens: Option<u64>,
    wire_model: Option<String>,
    pro_reasoning: bool,
}

impl ResolvedDialectProfile {
    pub fn wire_model(&self) -> &str {
        self.wire_model
            .as_deref()
            .unwrap_or(&self.target.route.exact_sku)
    }

    pub fn pro_reasoning(&self) -> bool {
        self.pro_reasoning
    }
    /// Native deferred-tool mode qualified by the exact dialect, model profile,
    /// SKU, endpoint owner and gateway translation of this route; `None` keeps
    /// host-side schema activation.
    #[must_use]
    pub fn native_deferred_tools(&self) -> Option<NativeDeferredMode> {
        self.native_deferred_tools
    }

    #[must_use]
    pub fn reasoning_efforts(&self) -> &[String] {
        &self.reasoning_efforts
    }

    #[must_use]
    pub fn default_reasoning_effort(&self) -> Option<&str> {
        self.default_reasoning_effort.as_deref()
    }

    /// Wire shape of the thinking control, when the exact model has one.
    #[must_use]
    pub fn thinking_wire(&self) -> Option<ThinkingWire> {
        self.thinking_wire
    }

    /// Whether `tool_choice` `any`/`tool` is accepted for this exact model.
    #[must_use]
    pub fn forced_tool_choice(&self) -> bool {
        self.forced_tool_choice
    }

    /// Whether server-side refusal fallbacks are requested for this model.
    #[must_use]
    pub fn refusal_fallback(&self) -> bool {
        self.refusal_fallback
    }

    /// Declared output ceiling, when the catalog pins one.
    #[must_use]
    pub fn max_output_tokens(&self) -> Option<u64> {
        self.max_output_tokens
    }
}

#[derive(Debug, Error)]
pub enum DialectError {
    #[error("unknown provider dialect {0}")]
    UnknownDialect(String),
    #[error("unknown provider protocol family {0}")]
    UnknownFamily(String),
    #[error("provider protocol family and dialect disagree")]
    FamilyMismatch,
    #[error("provider route evidence does not match a proved profile")]
    RouteMismatch,
    #[error("provider dialect proof is unavailable: {0}")]
    UnprovedProfile(String),
    #[error("provider control {0} is unsupported by the exact profile")]
    UnsupportedControl(String),
    #[error("provider epoch target does not match the selected exact profile")]
    EpochTargetMismatch,
    #[error("provider epoch profile is not I-JSON: {0}")]
    InvalidEpoch(String),
}

fn definition_for(dialect: DialectId) -> Result<&'static Definition, DialectError> {
    DEFINITIONS
        .iter()
        .find(|definition| definition.dialect == dialect)
        .ok_or_else(|| DialectError::UnknownDialect(dialect.as_str().to_owned()))
}

fn target_matches_definition(target: &ProviderTarget, definition: &Definition) -> bool {
    target.protocol_family == definition.family
}

fn load_proof_registry() -> Result<ProofRegistry, String> {
    let actual = format!("{:x}", Sha256::digest(PROOF_ORACLE));
    if actual != PROOF_ORACLE_SHA256 {
        return Err(format!(
            "provider proof oracle digest mismatch: expected {PROOF_ORACLE_SHA256}, got {actual}"
        ));
    }
    let root: Value = serde_json::from_slice(PROOF_ORACLE)
        .map_err(|error| format!("provider proof oracle is not JSON: {error}"))?;
    let required = root
        .get("required_proof_arms")
        .and_then(Value::as_array)
        .ok_or("provider proof oracle lacks required_proof_arms")?;
    let profiles = root
        .get("profiles")
        .and_then(Value::as_array)
        .ok_or("provider proof oracle lacks profiles")?;
    let mut all = Vec::new();
    let mut advertised = Vec::new();
    let mut proof_ids = BTreeSet::new();
    for profile in profiles {
        let is_advertised = profile.get("advertised").and_then(Value::as_bool) == Some(true);
        if is_advertised {
            for arm in required {
                let name = arm
                    .as_str()
                    .ok_or("provider proof arm name is not a string")?;
                if profile.get(name).is_none_or(Value::is_null) {
                    return Err(format!("advertised provider proof lacks {name}"));
                }
            }
        }
        let target: ProviderTarget = serde_json::from_value(
            profile
                .get("target")
                .cloned()
                .ok_or("provider proof lacks target")?,
        )
        .map_err(|error| format!("provider proof target is invalid: {error}"))?;
        let dialect = target
            .dialect()
            .map_err(|error| format!("provider proof dialect is invalid: {error}"))?;
        let definition = definition_for(dialect).map_err(|error| error.to_string())?;
        if !target_matches_definition(&target, definition) {
            return Err(format!(
                "provider proof target mismatch for {}",
                dialect.as_str()
            ));
        }
        let proof_id = profile
            .get("proof_id")
            .and_then(Value::as_str)
            .ok_or("provider proof lacks proof_id")?;
        if proof_id.is_empty() || !proof_ids.insert(proof_id.to_owned()) {
            return Err(format!("duplicate or empty provider proof id {proof_id}"));
        }
        let endpoint = profile
            .get("endpoint")
            .and_then(Value::as_str)
            .ok_or("provider proof lacks endpoint")?;
        if endpoint.is_empty() {
            return Err(format!("provider proof endpoint is empty for {proof_id}"));
        }
        let credential_header = profile
            .get("credential_header")
            .and_then(Value::as_str)
            .ok_or("provider proof lacks credential_header")?;
        let credential_prefix = profile
            .get("credential_prefix")
            .and_then(Value::as_str)
            .ok_or("provider proof lacks credential_prefix")?;
        if credential_header.is_empty() {
            return Err(format!(
                "provider proof credential header is empty for {proof_id}"
            ));
        }
        if profile.get("serializer_revision").and_then(Value::as_str)
            != Some(definition.serializer_revision)
        {
            return Err(format!(
                "provider serializer revision mismatch for {}",
                dialect.as_str()
            ));
        }
        let expected_continuation = if dialect.server_managed() {
            "server_managed"
        } else {
            "stateless_full_history"
        };
        if profile.get("continuation").and_then(Value::as_str) != Some(expected_continuation) {
            return Err(format!(
                "provider continuation policy mismatch for {}",
                dialect.as_str()
            ));
        }
        let proof = AdvertisedDialectProof {
            proof_id: proof_id.to_owned(),
            protocol_family: target.protocol_family,
            dialect_id: target.dialect_id,
            model_profile_id: target.model_profile_id,
            endpoint_owner: target.route.endpoint_owner,
            gateway_translation: target.route.gateway_translation,
            endpoint: endpoint.to_owned(),
            exact_sku: target.route.exact_sku,
            evidence_revision: target.route.evidence_revision,
            credential_header: credential_header.to_owned(),
            credential_prefix: credential_prefix.to_owned(),
        };
        all.push(proof.clone());
        if is_advertised {
            advertised.push(proof);
        }
    }
    Ok(ProofRegistry { all, advertised })
}

fn load_model_capabilities() -> Result<Vec<ModelCapabilityProfile>, String> {
    let actual = format!("{:x}", Sha256::digest(MODEL_CAPABILITY_CATALOG));
    if actual != MODEL_CAPABILITY_CATALOG_SHA256 {
        return Err(format!(
            "model capability catalog digest mismatch: expected {MODEL_CAPABILITY_CATALOG_SHA256}, got {actual}"
        ));
    }
    let catalog: ModelCapabilityCatalog = serde_json::from_slice(MODEL_CAPABILITY_CATALOG)
        .map_err(|error| format!("model capability catalog is not JSON: {error}"))?;
    if catalog.format != 1 {
        return Err(format!(
            "unsupported model capability catalog format {}",
            catalog.format
        ));
    }
    let mut identities = BTreeSet::new();
    for profile in &catalog.profiles {
        let identity = (
            profile.dialect_id.as_str(),
            profile.model_profile_id.as_str(),
            profile.exact_sku.as_str(),
        );
        if !identities.insert(identity) {
            return Err(format!(
                "duplicate model capability profile {}/{}/{}",
                profile.dialect_id, profile.model_profile_id, profile.exact_sku
            ));
        }
        DialectId::from_str(&profile.dialect_id)
            .map_err(|error| format!("model capability dialect is invalid: {error}"))?;
        if profile.model_profile_id.is_empty() || profile.exact_sku.is_empty() {
            return Err("model capability identity must be nonempty".to_owned());
        }
        let mut levels = BTreeSet::new();
        for level in &profile.reasoning.levels {
            if level.is_empty()
                || !level.as_bytes().iter().all(u8::is_ascii)
                || !levels.insert(level.as_str())
            {
                return Err(format!(
                    "invalid or duplicate reasoning level in {}",
                    profile.model_profile_id
                ));
            }
        }
        let dialect = DialectId::from_str(&profile.dialect_id).expect("validated above");
        if profile
            .wire_model
            .as_ref()
            .is_some_and(|model| model.is_empty())
            || profile.pro_reasoning
                && (dialect != DialectId::OpenaiResponsesV1 || profile.wire_model.is_none())
        {
            return Err("invalid model wire variant capability".to_owned());
        }
        if dialect.family() == AdapterId::Anthropic
            && !profile.reasoning.levels.is_empty()
            && profile.reasoning.thinking.is_none()
        {
            return Err(format!(
                "Anthropic capability {} declares reasoning levels without a thinking wire",
                profile.model_profile_id
            ));
        }
        if profile
            .reasoning
            .default
            .as_ref()
            .is_some_and(|value| !levels.contains(value.as_str()))
        {
            return Err(format!(
                "default reasoning level is absent from {}",
                profile.model_profile_id
            ));
        }
        if profile
            .evidence_url
            .as_ref()
            .is_some_and(|url| !url.starts_with("https://"))
        {
            return Err(format!(
                "model capability evidence URL is invalid for {}",
                profile.model_profile_id
            ));
        }
    }
    Ok(catalog.profiles)
}

fn model_capability_for(
    target: &ProviderTarget,
) -> Result<Option<ModelCapabilityProfile>, DialectError> {
    let catalog = MODEL_CAPABILITIES.get_or_init(load_model_capabilities);
    let catalog = catalog
        .as_ref()
        .map_err(|error| DialectError::UnprovedProfile(error.clone()))?;
    if let Some(exact) = catalog.iter().find(|profile| {
        profile.dialect_id == target.dialect_id
            && profile.model_profile_id == target.model_profile_id
            && profile.exact_sku == target.route.exact_sku
    }) {
        return Ok(Some(exact.clone()));
    }
    Ok(dialect_uniform_capability(
        catalog,
        &target.dialect_id,
        &target.route.exact_sku,
    ))
}

/// A configured sku the catalog does not list still runs on its dialect; when
/// every listed profile of that dialect agrees on the reasoning control and
/// tool-choice behaviour, that shared capability applies to the unlisted sku.
/// Model-specific facts (wire aliases, output ceilings, native deferred tools,
/// pro reasoning, refusal fallbacks) are never inherited.
fn dialect_uniform_capability(
    catalog: &[ModelCapabilityProfile],
    dialect_id: &str,
    exact_sku: &str,
) -> Option<ModelCapabilityProfile> {
    let mut family = catalog
        .iter()
        .filter(|profile| profile.dialect_id == dialect_id);
    let first = family.next()?;
    if family.any(|profile| {
        profile.reasoning != first.reasoning
            || profile.forced_tool_choice != first.forced_tool_choice
    }) {
        return None;
    }
    Some(ModelCapabilityProfile {
        dialect_id: dialect_id.to_owned(),
        model_profile_id: format!("{dialect_id}:{exact_sku}"),
        exact_sku: exact_sku.to_owned(),
        wire_model: None,
        pro_reasoning: false,
        reasoning: first.reasoning.clone(),
        evidence_url: None,
        native_deferred_tools: None,
        forced_tool_choice: first.forced_tool_choice,
        refusal_fallback: false,
        max_output_tokens: None,
    })
}

fn proof_matches_target(proof: &AdvertisedDialectProof, target: &ProviderTarget) -> bool {
    proof.protocol_family == target.protocol_family
        && proof.dialect_id == target.dialect_id
        && proof.model_profile_id == target.model_profile_id
        && proof.endpoint_owner == target.route.endpoint_owner
        && proof.gateway_translation == target.route.gateway_translation
        && proof.exact_sku == target.route.exact_sku
        && proof.evidence_revision == target.route.evidence_revision
}

fn endpoint_matches(pattern: &str, endpoint: &str) -> bool {
    let pattern = pattern.trim_end_matches('/');
    let endpoint = endpoint.trim_end_matches('/');
    let pattern_parts = pattern.split('/').collect::<Vec<_>>();
    let endpoint_parts = endpoint.split('/').collect::<Vec<_>>();
    pattern_parts.len() == endpoint_parts.len()
        && pattern_parts
            .iter()
            .zip(endpoint_parts)
            .all(|(expected, actual)| {
                if expected.starts_with('{') && expected.ends_with('}') {
                    expected.len() > 2
                        && !actual.is_empty()
                        && actual != "."
                        && actual != ".."
                        && !actual.contains(['?', '#', '{', '}'])
                } else {
                    *expected == actual
                }
            })
}

fn proof_matches_endpoint(
    proof: &AdvertisedDialectProof,
    target: &ProviderTarget,
    endpoint: &str,
) -> bool {
    proof_matches_target(proof, target) && endpoint_matches(&proof.endpoint, endpoint)
}

fn proof_matches_provider_route(proof: &AdvertisedDialectProof, provider: &Provider) -> bool {
    proof.protocol_family == provider.adapter
        && proof.dialect_id == provider.dialect
        && proof.endpoint_owner == provider.endpoint_owner
        && proof.gateway_translation == provider.gateway_translation
        && proof.evidence_revision == provider.evidence_revision
        && endpoint_matches(&proof.endpoint, &provider.endpoint)
}

pub fn validate_endpoint(target: &ProviderTarget, endpoint: &str) -> Result<(), DialectError> {
    let registry = PROOF_REGISTRY.get_or_init(load_proof_registry);
    match registry {
        Ok(registry)
            if registry
                .all
                .iter()
                .any(|proof| proof_matches_endpoint(proof, target, endpoint)) =>
        {
            Ok(())
        }
        Ok(_) => Err(DialectError::RouteMismatch),
        Err(error) => Err(DialectError::UnprovedProfile(error.clone())),
    }
}

/// Validates a configured connection without selecting a model. Route identity
/// and endpoint matching come only from the embedded proof catalog.
pub fn configured_route_is_verified(provider: &Provider) -> Result<bool, DialectError> {
    let dialect = DialectId::from_str(&provider.dialect)?;
    let definition = definition_for(dialect)?;
    if provider.adapter != definition.family {
        return Err(DialectError::RouteMismatch);
    }
    let registry = PROOF_REGISTRY.get_or_init(load_proof_registry);
    let registry = registry
        .as_ref()
        .map_err(|error| DialectError::UnprovedProfile(error.clone()))?;
    if !registry
        .advertised
        .iter()
        .any(|proof| proof.dialect_id == dialect.as_str())
    {
        return Err(DialectError::UnprovedProfile(dialect.as_str().to_owned()));
    }
    authentication_policy(registry, dialect)?;
    Ok(registry
        .advertised
        .iter()
        .any(|proof| proof_matches_provider_route(proof, provider)))
}

/// Returns the installed advertised proof registry in stable catalog order.
/// Loading is fail-closed: an invalid embedded oracle never produces a partial
/// catalog.
pub fn advertised_dialect_proofs() -> Result<Vec<AdvertisedDialectProof>, DialectError> {
    let registry = PROOF_REGISTRY.get_or_init(load_proof_registry);
    let registry = registry
        .as_ref()
        .map_err(|error| DialectError::UnprovedProfile(error.clone()))?;
    Ok(registry.advertised.clone())
}

pub fn resolve_profile(
    provider: &Provider,
    model: &Model,
) -> Result<ResolvedDialectProfile, DialectError> {
    let dialect = DialectId::from_str(&provider.dialect)?;
    let definition = definition_for(dialect)?;
    if provider.adapter != definition.family {
        return Err(DialectError::RouteMismatch);
    }
    let target = ProviderTarget {
        protocol_family: provider.adapter.clone(),
        dialect_id: provider.dialect.clone(),
        model_profile_id: model.profile.clone(),
        route: RouteEvidence {
            endpoint_owner: provider.endpoint_owner.clone(),
            gateway_translation: provider.gateway_translation.clone(),
            exact_sku: model.id.clone(),
            evidence_revision: provider.evidence_revision.clone(),
        },
    };
    resolve_configured_target(target, dialect, definition, Some(&provider.endpoint))
}

pub fn validate_target(target: &ProviderTarget) -> Result<ResolvedDialectProfile, DialectError> {
    let dialect = target.dialect()?;
    let definition = definition_for(dialect)?;
    if !target_matches_definition(target, definition) {
        return Err(DialectError::RouteMismatch);
    }
    resolve_configured_target(target.clone(), dialect, definition, None)
}

fn resolve_configured_target(
    target: ProviderTarget,
    dialect: DialectId,
    definition: &'static Definition,
    configured_endpoint: Option<&str>,
) -> Result<ResolvedDialectProfile, DialectError> {
    let registry = PROOF_REGISTRY.get_or_init(load_proof_registry);
    let registry = registry
        .as_ref()
        .map_err(|error| DialectError::UnprovedProfile(error.clone()))?;
    let exact = registry
        .advertised
        .iter()
        .find(|proof| proof_matches_target(proof, &target));
    let (mut credential_header, mut credential_prefix) = match exact {
        Some(proof) => (
            proof.credential_header.clone(),
            proof.credential_prefix.clone(),
        ),
        None => authentication_policy(registry, dialect)?,
    };
    if dialect == DialectId::AnthropicMessagesV1
        && target.route.endpoint_owner == "cloudflare"
        && target.route.gateway_translation == "cloudflare-native-anthropic"
    {
        credential_header = "cf-aig-authorization".to_owned();
        credential_prefix = "Bearer ".to_owned();
    }
    let proof_verified = exact.is_some_and(|proof| {
        configured_endpoint.is_none_or(|endpoint| endpoint_matches(&proof.endpoint, endpoint))
    });
    let capability = model_capability_for(&target)?;
    let native_deferred_tools = capability
        .as_ref()
        .and_then(|value| value.native_deferred_tools.as_ref())
        .filter(|native| {
            native.mode.dialect() == dialect
                && native.routes.iter().any(|route| {
                    route.endpoint_owner == target.route.endpoint_owner
                        && route.gateway_translation == target.route.gateway_translation
                })
        })
        .map(|native| native.mode);
    Ok(ResolvedDialectProfile {
        target,
        dialect,
        serializer_revision: definition.serializer_revision,
        credential_header,
        credential_prefix,
        proof_verified,
        proved_endpoint: exact.map(|proof| proof.endpoint.clone()),
        reasoning_efforts: capability
            .as_ref()
            .map(|value| value.reasoning.levels.clone())
            .unwrap_or_default(),
        default_reasoning_effort: capability
            .as_ref()
            .and_then(|value| value.reasoning.default.clone()),
        thinking_wire: capability
            .as_ref()
            .and_then(|value| value.reasoning.thinking),
        forced_tool_choice: capability
            .as_ref()
            .is_none_or(|value| value.forced_tool_choice),
        refusal_fallback: capability
            .as_ref()
            .is_some_and(|value| value.refusal_fallback),
        max_output_tokens: capability
            .as_ref()
            .and_then(|value| value.max_output_tokens),
        wire_model: capability
            .as_ref()
            .and_then(|value| value.wire_model.clone()),
        pro_reasoning: capability.as_ref().is_some_and(|value| value.pro_reasoning),
        native_deferred_tools,
    })
}

/// Missing exact route evidence does not make a configured dialect unusable. Best-effort
/// execution can reuse the dialect's authentication shape only when every embedded row agrees;
/// an ambiguous authentication policy remains a real configuration failure.
fn authentication_policy(
    registry: &ProofRegistry,
    dialect: DialectId,
) -> Result<(String, String), DialectError> {
    let policies = registry
        .all
        .iter()
        .filter(|proof| proof.dialect_id == dialect.as_str())
        .map(|proof| {
            (
                proof.credential_header.clone(),
                proof.credential_prefix.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if policies.len() == 1 {
        return Ok(policies
            .into_iter()
            .next()
            .expect("one authentication policy"));
    }
    Err(DialectError::UnprovedProfile(format!(
        "dialect {} has no unambiguous authentication policy",
        dialect.as_str()
    )))
}

pub fn epoch_profile(
    profile: &ResolvedDialectProfile,
    system: &str,
    settings: Option<&SessionSettings>,
) -> Result<IJsonValue, DialectError> {
    let requested = settings.and_then(|settings| settings.reasoning_effort.as_deref());
    let effort = requested.or_else(|| profile.default_reasoning_effort());
    if effort.is_some_and(|value| !profile.reasoning_efforts().iter().any(|item| item == value)) {
        return Err(DialectError::UnsupportedControl(
            "reasoning_effort".to_owned(),
        ));
    }
    let mut controls = serde_json::Map::new();
    if let Some(value) = effort {
        controls.insert(
            "reasoning_effort".to_owned(),
            Value::String(value.to_owned()),
        );
    }
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(&json!({
            "controls": controls,
            "serializer_revision": profile.serializer_revision,
            "system": system,
            "target": profile.target,
        }))
        .map_err(|error| DialectError::InvalidEpoch(error.to_string()))?,
    )
    .map_err(|error| DialectError::InvalidEpoch(error.to_string()))
}

pub fn validate_epoch_target(
    profile: &ResolvedDialectProfile,
    epoch: &IJsonValue,
) -> Result<(), DialectError> {
    let value = serde_json::to_value(epoch)
        .map_err(|error| DialectError::InvalidEpoch(error.to_string()))?;
    let target = value
        .get("target")
        .ok_or(DialectError::EpochTargetMismatch)?;
    let expected = serde_json::to_value(&profile.target)
        .map_err(|error| DialectError::InvalidEpoch(error.to_string()))?;
    if target != &expected {
        return Err(DialectError::EpochTargetMismatch);
    }
    if value.get("serializer_revision").and_then(Value::as_str) != Some(profile.serializer_revision)
    {
        return Err(DialectError::EpochTargetMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::endpoint_matches;

    #[test]
    fn unlisted_sku_inherits_a_dialect_uniform_reasoning_capability() {
        let catalog = super::MODEL_CAPABILITIES
            .get_or_init(super::load_model_capabilities)
            .as_ref()
            .expect("catalog");
        let inherited =
            super::dialect_uniform_capability(catalog, "deepseek_chat_v1", "deepseek-flash")
                .expect("deepseek chat profiles agree on reasoning");
        assert_eq!(inherited.exact_sku, "deepseek-flash");
        assert_eq!(
            inherited.model_profile_id,
            "deepseek_chat_v1:deepseek-flash"
        );
        assert_eq!(inherited.reasoning.levels, ["low", "high", "max"]);
        assert_eq!(inherited.reasoning.default.as_deref(), Some("high"));
        assert!(inherited.wire_model.is_none() && !inherited.pro_reasoning);
        let responses =
            super::dialect_uniform_capability(catalog, "deepseek_responses_v1", "unlisted")
                .expect("deepseek responses profiles agree on reasoning");
        assert_eq!(responses.reasoning.levels, ["low", "high", "max"]);
        assert!(super::dialect_uniform_capability(catalog, "glm_chat_v1", "unlisted").is_none());
        assert!(super::dialect_uniform_capability(catalog, "no_such_dialect", "x").is_none());
    }

    #[test]
    fn endpoint_patterns_match_opaque_configured_path_segments_only() {
        let pattern = "https://gateway.example/v1/{tenant}/{route}";
        assert!(endpoint_matches(
            pattern,
            "https://gateway.example/v1/tenant-42/primary"
        ));
        assert!(endpoint_matches(
            pattern,
            "https://gateway.example/v1/tenant-42/primary/"
        ));
        assert!(!endpoint_matches(
            pattern,
            "https://gateway.example/v1/tenant-42"
        ));
        assert!(!endpoint_matches(
            pattern,
            "https://other.example/v1/tenant-42/primary"
        ));
        assert!(!endpoint_matches(
            pattern,
            "https://gateway.example/v1/../primary"
        ));
    }
}
