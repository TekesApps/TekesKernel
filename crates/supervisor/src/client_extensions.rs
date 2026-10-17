//! Slice-14F Client extension registry.
//!
//! The frozen Session Endpoint table remains in `endpoint_host`. Each value
//! below is one atomic capability group: construction installs every method
//! in the group or none of them, and dispatch is delegated to exactly one
//! durable authority backend.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use endpoint::{DurableHandoffProof, EndpointHostCall, MethodClass, RpcDurableIdentity};
use profile::ResourceCatalog;
use schema::{EventKind, IJsonValue};
use serde::Deserialize;
use serde_json::Value;
use sha2::Digest as _;
use thread_search::{ArchiveVisibility, SearchRequest, ThreadSearchAuthority};
use tools::{BuiltinManifest, fixed_schema};

use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};

use crate::process_host::ProductionProcessHost;
use crate::resource_capability::{
    ClientResourceService, CommandRunRequest, EndpointCommandInputAuthority,
    map_prompt_materialize_error, map_resource_failure,
};

pub const RESOURCE_METHODS: [(&str, MethodClass); 5] = [
    ("skills/list", MethodClass::ReadOnly),
    ("commands/list", MethodClass::ReadOnly),
    ("commands/run", MethodClass::Mutation),
    ("resources/list", MethodClass::ReadOnly),
    ("resources/read", MethodClass::ReadOnly),
];
pub const INITIAL_PRESET_METHODS: [(&str, MethodClass); 1] =
    [("session.initialPresets", MethodClass::ReadOnly)];
pub const SESSION_PRESET_METHODS: [(&str, MethodClass); 2] = [
    ("session.presets.describe", MethodClass::ReadOnly),
    ("session.presets.resolve", MethodClass::ReadOnly),
];
pub const RECOVERY_METHODS: [(&str, MethodClass); 1] =
    [("sessions.recover", MethodClass::Mutation)];
pub const ATTACHMENT_METHODS: [(&str, MethodClass); 3] = [
    ("attachments.policy", MethodClass::ReadOnly),
    ("session.uploadFile", MethodClass::Mutation),
    ("session.fileAttachment", MethodClass::ReadOnly),
];
pub const APPROVAL_METHODS: [(&str, MethodClass); 3] = [
    ("approvals.policy", MethodClass::ReadOnly),
    ("approvals.mode", MethodClass::ReadOnly),
    ("approvals.select", MethodClass::Mutation),
];
pub const FEEDBACK_METHODS: [(&str, MethodClass); 3] = [
    ("feedback.list", MethodClass::ReadOnly),
    ("feedback.put", MethodClass::Mutation),
    ("feedback.delete", MethodClass::Mutation),
];
pub const SETTINGS_METHODS: [(&str, MethodClass); 4] = [
    ("settings.describe", MethodClass::ReadOnly),
    ("settings.mutate", MethodClass::Mutation),
    ("settings.update", MethodClass::Mutation),
    ("settings.document", MethodClass::ReadOnly),
];
pub const GOAL_METHODS: [(&str, MethodClass); 5] = [
    ("goals.get", MethodClass::ReadOnly),
    ("goals.edit", MethodClass::Mutation),
    ("goals.clear", MethodClass::Mutation),
    ("goals.pause", MethodClass::Mutation),
    ("goals.resume", MethodClass::Mutation),
];
pub const SUBAGENT_METHODS: [(&str, MethodClass); 2] = [
    ("subagents.list", MethodClass::ReadOnly),
    ("subagents.history", MethodClass::ReadOnly),
];

pub const FILE_METHODS: [(&str, MethodClass); 3] = [
    ("session.files.stat", MethodClass::ReadOnly),
    ("session.files.read", MethodClass::ReadOnly),
    ("session.files.readBytes", MethodClass::ReadOnly),
];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FilePageRequest {
    session_id: String,
    path: String,
    offset: Option<u64>,
    limit: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AttachmentPolicyRequest {
    session_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ApprovalModeRequest {
    session_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ApprovalSelectRequest {
    session_id: String,
    mode: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UploadFileRequest {
    session_id: String,
    name: String,
    media_type: Option<String>,
    data: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FileAttachmentRequest {
    session_id: String,
    attachment_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RecoveryRequest {
    session_ids: Vec<String>,
}
pub const TOOL_METHODS: [(&str, MethodClass); 2] = [
    ("tools/list", MethodClass::ReadOnly),
    ("tools/resolve", MethodClass::ReadOnly),
];

pub const THREAD_SEARCH_METHODS: [(&str, MethodClass); 2] = [
    ("thread.search", MethodClass::ReadOnly),
    ("session.search", MethodClass::ReadOnly),
];
pub const USAGE_METHODS: [(&str, MethodClass); 2] = [
    ("usage.summary", MethodClass::ReadOnly),
    ("usage.cacheAttribution", MethodClass::ReadOnly),
];

/// Semantic authority behind the transport-neutral registry. Mutation
/// implementations receive the endpoint rpc id unchanged; dropping or
/// replacing it would break their durable idempotency namespace.
pub trait ClientExtensionAuthority: Send + Sync {
    fn execute(
        &self,
        operation: &str,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
        _recovering: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure>;
}

pub struct AtomicCapabilityRoutes {
    id: &'static str,
    methods: &'static [(&'static str, MethodClass)],
    authority: Arc<dyn ClientExtensionAuthority>,
}

impl AtomicCapabilityRoutes {
    #[must_use]
    pub fn new(
        id: &'static str,
        methods: &'static [(&'static str, MethodClass)],
        authority: Arc<dyn ClientExtensionAuthority>,
    ) -> Self {
        assert!(
            !methods.is_empty(),
            "an advertised capability cannot be empty"
        );
        Self {
            id,
            methods,
            authority,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.id
    }

    fn class(&self, operation: &str) -> Option<MethodClass> {
        self.methods
            .iter()
            .find_map(|(name, class)| (*name == operation).then_some(*class))
    }
}

impl ProductionEndpointRoutes for AtomicCapabilityRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        self.methods
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        self.class(method)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        if self.class(operation).is_none() || !payload.is_object() {
            return Err(failure(
                "bad-request",
                "Invalid Client extension request",
                serde_json::json!({"operation":operation}),
            ));
        }
        validate_payload(operation, payload)
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        // Every backend is responsible for emitting only the closed contract
        // vocabulary. This registry additionally blocks accidental raw Rust
        // and transport errors by requiring the public kebab-case shape.
        error_allowed(operation, self.class(operation), &failure.code)
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        if self.class(&request.operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Capability is unavailable",
                serde_json::json!({"operation":request.operation}),
            ));
        }
        let result = self.authority.execute(
            &request.operation,
            payload,
            principal,
            &request.rpc_id,
            request.recovering,
        )?;
        if self.class(&request.operation) == Some(MethodClass::Mutation) {
            request
                .handoff
                .mark_handed_off(DurableHandoffProof {
                    delivery: "extension-authority-barrier".to_owned(),
                    durable_identity: Some(RpcDurableIdentity {
                        kind: "extension-rpc".to_owned(),
                        id: request.rpc_id.clone(),
                        seq: None,
                    }),
                })
                .map_err(|_| {
                    failure(
                        "internal",
                        "Durable handoff proof failed",
                        serde_json::json!({}),
                    )
                })?;
        }
        Ok(result)
    }
}

#[must_use]
pub fn common_capability_routes(
    authority: Arc<dyn ClientExtensionAuthority>,
) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
    [
        ("resources.v1", RESOURCE_METHODS.as_slice()),
        ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
        ("sessionPresets.v1", SESSION_PRESET_METHODS.as_slice()),
        ("recovery.v1", RECOVERY_METHODS.as_slice()),
        ("attachments.v1", ATTACHMENT_METHODS.as_slice()),
        ("approvals.v1", APPROVAL_METHODS.as_slice()),
        ("feedback.v1", FEEDBACK_METHODS.as_slice()),
        ("settings.v1", SETTINGS_METHODS.as_slice()),
        ("goals.v1", GOAL_METHODS.as_slice()),
        ("subagents.v1", SUBAGENT_METHODS.as_slice()),
        ("sessionFiles.v1", FILE_METHODS.as_slice()),
        ("tools.v1", TOOL_METHODS.as_slice()),
        ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
        ("usage.v1", USAGE_METHODS.as_slice()),
    ]
    .into_iter()
    .map(|(id, methods)| {
        Arc::new(AtomicCapabilityRoutes::new(
            id,
            methods,
            Arc::clone(&authority),
        )) as Arc<dyn ProductionEndpointRoutes>
    })
    .collect()
}

/// Concrete binding to the Slice-11--14 authorities. There is deliberately no
/// Product-specific branch: plugin and MCP rows go through their generic stores.
pub struct ProductionClientExtensions {
    root: PathBuf,
    commands: ClientResourceService<EndpointCommandInputAuthority>,
    search: ThreadSearchAuthority,
    process: Arc<ProductionProcessHost>,
}

impl ProductionClientExtensions {
    pub fn open(
        root: impl AsRef<Path>,
        resources: ResourceCatalog,
        commands: EndpointCommandInputAuthority,
        process: Arc<ProductionProcessHost>,
    ) -> Result<Self, ProductionRouteFailure> {
        let root = root.as_ref().to_path_buf();
        let commands =
            commands.with_attachment_authority(endpoint::AttachmentAuthority::open(&root));
        Ok(Self {
            commands: ClientResourceService::new(resources, commands),
            search: ThreadSearchAuthority::open(&root).map_err(map_search_error)?,
            root,
            process,
        })
    }

    /// The same session-scoped attachment authority `session.prompt` and
    /// `session.attachment` use: one storage root, content-addressed assets.
    fn attachments(&self) -> endpoint::AttachmentAuthority {
        endpoint::AttachmentAuthority::open(&self.root)
    }

    pub fn routes(self: &Arc<Self>) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
        let authority: Arc<dyn ClientExtensionAuthority> = self.clone();
        let groups = vec![
            ("resources.v1", RESOURCE_METHODS.as_slice()),
            ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
            ("sessionPresets.v1", SESSION_PRESET_METHODS.as_slice()),
            ("recovery.v1", RECOVERY_METHODS.as_slice()),
            ("attachments.v1", ATTACHMENT_METHODS.as_slice()),
            ("approvals.v1", APPROVAL_METHODS.as_slice()),
            ("feedback.v1", FEEDBACK_METHODS.as_slice()),
            ("settings.v1", SETTINGS_METHODS.as_slice()),
            ("goals.v1", GOAL_METHODS.as_slice()),
            ("subagents.v1", SUBAGENT_METHODS.as_slice()),
            ("sessionFiles.v1", FILE_METHODS.as_slice()),
            ("tools.v1", TOOL_METHODS.as_slice()),
            ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
            ("usage.v1", USAGE_METHODS.as_slice()),
        ];
        groups
            .into_iter()
            .map(|(id, methods)| {
                Arc::new(AtomicCapabilityRoutes::new(
                    id,
                    methods,
                    Arc::clone(&authority),
                )) as Arc<dyn ProductionEndpointRoutes>
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ResourceListRequest {
    session_id: String,
    source: String,
    after: Option<String>,
    limit: Option<usize>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ResourceReadRequest {
    session_id: String,
    reference: Value,
    uri: String,
}
#[derive(Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
enum ResourceReference {
    #[serde(rename = "skill")]
    Skill {
        #[serde(rename = "sessionId")]
        session_id: String,
        name: String,
        #[serde(rename = "contentDigest")]
        content_digest: String,
    },
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolListRequest {
    session_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolResolveRequest {
    session_id: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SessionSearchRequest {
    query: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ThreadSearchRequest {
    session_id: Option<String>,
    query: String,
    limit: usize,
    visibility: ArchiveVisibility,
    after: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SessionIdRequest {
    session_id: String,
}

fn decode<T: for<'de> Deserialize<'de>>(value: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(value.clone()).map_err(|error| bad_request(&error.to_string()))
}
fn encode(value: &impl serde::Serialize) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|_| {
        failure(
            "internal",
            "Response serialization failed",
            serde_json::json!({}),
        )
    })?)
    .map_err(|_| failure("internal", "Response is not I-JSON", serde_json::json!({})))
}
fn ijson_value(value: &IJsonValue) -> Result<Value, ProductionRouteFailure> {
    serde_json::from_slice(&value.canonical_bytes().map_err(|_| {
        failure(
            "internal",
            "I-JSON serialization failed",
            serde_json::json!({}),
        )
    })?)
    .map_err(|_| failure("internal", "I-JSON decoding failed", serde_json::json!({})))
}
fn bad_request(reason: &str) -> ProductionRouteFailure {
    failure(
        "bad-request",
        "Invalid Client extension request",
        serde_json::json!({"reason":reason}),
    )
}

fn decode_resource_cursor(value: &str, generation: &str) -> Result<usize, ProductionRouteFailure> {
    let (found, offset) = value.rsplit_once(':').ok_or_else(|| {
        failure(
            "resource-stale",
            "Resource cursor is malformed",
            serde_json::json!({}),
        )
    })?;
    if found != generation {
        return Err(failure(
            "resource-stale",
            "Resource generation changed",
            serde_json::json!({}),
        ));
    }
    offset.parse().map_err(|_| {
        failure(
            "resource-stale",
            "Resource cursor is malformed",
            serde_json::json!({}),
        )
    })
}

fn numeric_token(value: Option<&Value>) -> Result<u64, ProductionRouteFailure> {
    match value.and_then(Value::as_str) {
        None => Ok(0),
        Some(value) => value.parse().map_err(|_| {
            failure(
                "source-corrupt",
                "Usage token value is invalid",
                serde_json::json!({}),
            )
        }),
    }
}

#[derive(Debug)]
struct CacheAttributionGroup {
    target: Value,
    attempts: BTreeSet<String>,
    input_tokens: String,
    output_tokens: String,
    cache_read_tokens: String,
    cost: Option<String>,
    cost_members: usize,
}

impl CacheAttributionGroup {
    fn new(target: Value) -> Self {
        Self {
            target,
            attempts: BTreeSet::new(),
            input_tokens: "0".to_owned(),
            output_tokens: "0".to_owned(),
            cache_read_tokens: "0".to_owned(),
            cost: None,
            cost_members: 0,
        }
    }

    fn add(
        &mut self,
        attempt: &str,
        input: &str,
        output: &str,
        cache_read: &str,
        cost: Option<&str>,
    ) -> Result<(), ProductionRouteFailure> {
        if !self.attempts.insert(attempt.to_owned()) {
            return Err(failure(
                "source-corrupt",
                "Effective usage repeats an attempt in one target group",
                serde_json::json!({}),
            ));
        }
        self.input_tokens = add_unsigned_decimal(&self.input_tokens, input)?;
        self.output_tokens = add_unsigned_decimal(&self.output_tokens, output)?;
        self.cache_read_tokens = add_unsigned_decimal(&self.cache_read_tokens, cache_read)?;
        match (&mut self.cost, cost) {
            (None, Some(value)) if self.cost_members == 0 => {
                self.cost = Some(canonical_cost(value)?);
            }
            (Some(total), Some(value)) => {
                *total = add_canonical_cost(total, value)?;
            }
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => {
                return Err(failure(
                    "source-corrupt",
                    "Usage cost availability changed within one exact target",
                    serde_json::json!({}),
                ));
            }
        }
        self.cost_members += 1;
        Ok(())
    }

    fn value(self) -> Result<Value, ProductionRouteFailure> {
        let attempts = u64::try_from(self.attempts.len()).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage attempt count overflowed",
                serde_json::json!({}),
            )
        })?;
        if attempts > 9_007_199_254_740_991 {
            return Err(failure(
                "source-corrupt",
                "Usage attempt count exceeds I-JSON",
                serde_json::json!({}),
            ));
        }
        let mut value = serde_json::json!({
            "target":self.target,
            "attempts":attempts,
            "inputTokens":self.input_tokens,
            "outputTokens":self.output_tokens,
            "cacheReadTokens":self.cache_read_tokens,
        });
        if let Some(cost) = self.cost {
            value["cost"] = Value::String(cost);
        }
        Ok(value)
    }
}

fn canonical_unsigned(value: &str) -> Result<&str, ProductionRouteFailure> {
    if value == "0" || (!value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        Ok(value)
    } else {
        Err(failure(
            "source-corrupt",
            "Usage token value is not canonical unsigned decimal",
            serde_json::json!({}),
        ))
    }
}

fn add_unsigned_decimal(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    let left = canonical_unsigned(left)?;
    let right = canonical_unsigned(right)?;
    add_decimal_digits(left, right)
}

fn add_decimal_digits(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    if !left.bytes().all(|byte| byte.is_ascii_digit())
        || !right.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(failure(
            "source-corrupt",
            "Usage decimal contains a non-digit",
            serde_json::json!({}),
        ));
    }
    let mut carry = 0u8;
    let mut result = Vec::with_capacity(left.len().max(right.len()) + 1);
    let mut left = left.bytes().rev();
    let mut right = right.bytes().rev();
    loop {
        let a = left.next().map(|byte| byte - b'0');
        let b = right.next().map(|byte| byte - b'0');
        if a.is_none() && b.is_none() && carry == 0 {
            break;
        }
        let sum = a.unwrap_or(0) + b.unwrap_or(0) + carry;
        result.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    result.reverse();
    let result = String::from_utf8(result).map_err(|_| {
        failure(
            "internal",
            "Usage decimal addition failed",
            serde_json::json!({}),
        )
    })?;
    let result = result.trim_start_matches('0');
    Ok(if result.is_empty() {
        "0".to_owned()
    } else {
        result.to_owned()
    })
}

fn canonical_cost(value: &str) -> Result<String, ProductionRouteFailure> {
    let (integer, fraction) = match value.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (value, None),
    };
    canonical_unsigned(integer)?;
    if let Some(fraction) = fraction
        && (fraction.is_empty()
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || fraction.ends_with('0'))
    {
        return Err(failure(
            "source-corrupt",
            "Usage cost is not canonical decimal",
            serde_json::json!({}),
        ));
    }
    if integer == "0" && fraction.is_some_and(|digits| digits.bytes().all(|byte| byte == b'0')) {
        return Err(failure(
            "source-corrupt",
            "Usage zero cost is not canonical",
            serde_json::json!({}),
        ));
    }
    Ok(value.to_owned())
}

fn add_canonical_cost(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    let left = canonical_cost(left)?;
    let right = canonical_cost(right)?;
    let left_scale = left.split_once('.').map_or(0, |(_, digits)| digits.len());
    let right_scale = right.split_once('.').map_or(0, |(_, digits)| digits.len());
    let scale = left_scale.max(right_scale);
    let left_digits = left.replace('.', "") + &"0".repeat(scale - left_scale);
    let right_digits = right.replace('.', "") + &"0".repeat(scale - right_scale);
    let mut sum = add_decimal_digits(&left_digits, &right_digits)?;
    if scale == 0 {
        return Ok(sum);
    }
    if sum.len() <= scale {
        sum = format!("{}{}", "0".repeat(scale + 1 - sum.len()), sum);
    }
    let point = sum.len() - scale;
    sum.insert(point, '.');
    while sum.ends_with('0') {
        sum.pop();
    }
    if sum.ends_with('.') {
        sum.pop();
    }
    Ok(sum)
}

fn usage_route_target(
    epoch: &Value,
    assets: &store::AssetStore,
) -> Result<Value, ProductionRouteFailure> {
    let asset = epoch
        .pointer("/system/asset")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile asset is absent",
                serde_json::json!({}),
            )
        })?;
    let digest = epoch
        .pointer("/system/digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile digest is absent",
                serde_json::json!({}),
            )
        })?;
    if asset != format!("sha256-{digest}") {
        return Err(failure(
            "source-corrupt",
            "Epoch profile asset and digest disagree",
            serde_json::json!({}),
        ));
    }
    let bytes = assets.read_verified(asset).map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile asset is corrupt",
            serde_json::json!({}),
        )
    })?;
    let profile = IJsonValue::parse(&bytes).map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile asset is not I-JSON",
            serde_json::json!({}),
        )
    })?;
    if profile.canonical_bytes().map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile is not canonical",
            serde_json::json!({}),
        )
    })? != bytes
    {
        return Err(failure(
            "source-corrupt",
            "Epoch profile asset is not canonical",
            serde_json::json!({}),
        ));
    }
    let profile = ijson_value(&profile)?;
    let target = profile
        .get("target")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile target is absent",
                serde_json::json!({}),
            )
        })?;
    let route = target
        .get("route")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch route evidence is absent",
                serde_json::json!({}),
            )
        })?;
    let field = |object: &serde_json::Map<String, Value>, name: &str| {
        object
            .get(name)
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Epoch target field is absent",
                    serde_json::json!({"field":name}),
                )
            })
    };
    let protocol_family = field(target, "protocol_family")?;
    let dialect_id = field(target, "dialect_id")?;
    let model_profile_id = field(target, "model_profile_id")?;
    let endpoint_owner = field(route, "endpoint_owner")?;
    let gateway_translation = field(route, "gateway_translation")?;
    let exact_sku = field(route, "exact_sku")?;
    let evidence_revision = field(route, "evidence_revision")?;
    if epoch.get("adapter").and_then(Value::as_str) != Some(dialect_id.as_str())
        || epoch.get("model").and_then(Value::as_str) != Some(exact_sku.as_str())
    {
        return Err(failure(
            "source-corrupt",
            "Epoch summary disagrees with its exact target",
            serde_json::json!({}),
        ));
    }
    Ok(serde_json::json!({
        "protocolFamily":protocol_family,
        "dialectId":dialect_id,
        "modelProfileId":model_profile_id,
        "endpointOwner":endpoint_owner,
        "gatewayTranslation":gateway_translation,
        "exactSku":exact_sku,
        "evidenceRevision":evidence_revision,
    }))
}

fn map_search_error(error: thread_search::SearchError) -> ProductionRouteFailure {
    use thread_search::SearchError::*;
    let code = match &error {
        InvalidSessionFilter => "invalid-session-filter",
        InvalidQuery => "invalid-query",
        InvalidLimit => "invalid-limit",
        MalformedCursor => "malformed-cursor",
        CursorScope => "cursor-scope",
        CursorStale => "cursor-stale",
        CursorPosition => "cursor-position",
        SourceIdentity => "source-identity",
        SourceCorrupt(_) | IndexCorrupt(_) | Canonical(_) | Store(_) | Io(_) => "source-corrupt",
    };
    failure(
        code,
        "Thread search failed",
        serde_json::json!({"reason":error.to_string()}),
    )
}

impl ProductionClientExtensions {
    /// `(folder, archived)` for a session that exists in either area.
    fn session_folder(&self, session_id: &str) -> Result<(PathBuf, bool), ProductionRouteFailure> {
        endpoint::validate_session_id(session_id)
            .map_err(|_| bad_request("invalid session identity"))?;
        [("threads", false), ("archive", true)]
            .into_iter()
            .map(|(area, archived)| (self.root.join(area).join(session_id), archived))
            .find(|(folder, _)| folder.is_dir())
            .ok_or_else(|| {
                failure(
                    "session-not-found",
                    "Session was not found",
                    serde_json::json!({"sessionId":session_id}),
                )
            })
    }

    fn session_controls(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let map = |error: session_controls::Failure| {
            failure(error.code(), &error.to_string(), error.details())
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default();
        let value = if operation == "subagents.list" {
            let (folder, archived) = self.session_folder(text("parentSessionId"))?;
            let bytes = std::fs::read(folder.join("main.jsonl")).map_err(|error| {
                failure(
                    "ledger-corrupt",
                    "Session ledger is unreadable",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
            session_controls::subagent_catalog(&bytes, text("parentSessionId"), archived)
                .map_err(map)?
        } else {
            self.session_folder(text("sessionId"))?;
            // Production never binds a host goal id; the record owns its identity.
            let value = session_controls::execute_goal(&self.root, operation, payload, None)
                .map_err(map)?;
            if operation == "goals.get" {
                serde_json::json!({"goal": value})
            } else {
                value
            }
        };
        encode(&value)
    }
}

impl ClientExtensionAuthority for ProductionClientExtensions {
    fn execute(
        &self,
        operation: &str,
        payload: &Value,
        principal: &str,
        _rpc_id: &str,
        _recovering: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match operation {
            "session.presets.describe" => encode(&tools::describe_presets()),
            "session.presets.resolve" => {
                let execution: schema::ExecutionConfig =
                    serde_json::from_value(payload["execution"].clone())
                        .map_err(|_| bad_request("execution is required"))?;
                let repository = profile::ConfigRepository::open(&self.root)
                    .map_err(|_| bad_request("configuration unavailable"))?;
                let execution = repository
                    .normalize_execution(&execution)
                    .map_err(|_| bad_request("invalid execution"))?;
                let snapshot = repository
                    .resolve_execution("preview", &execution, None)
                    .map_err(|_| bad_request("invalid execution"))?;
                let instruction = profile::InstructionResolver::new_scoped(
                    self.process.user_agent_dir(),
                    self.root.join("session-instructions/preview"),
                    snapshot.execution.cwd.iter().map(Path::new),
                )
                .capture()
                .map_err(|_| bad_request("instructions unavailable"))?;
                let environment =
                    endpoint::SessionPresetEnvironment::new(&instruction, &execution.mcp)
                        .with_models(&snapshot, Some(&crate::endpoint_host::ConfiguredModels));
                let base = tools::PresetBase::default();
                let preset =
                    tools::ResolvedPreset::resolve_request(&payload["preset"], base, &environment)
                        .map_err(|error| match error {
                            tools::PresetError::BadRequest(message) => bad_request(&message),
                            tools::PresetError::Invalid(invalid) => failure(
                                "preset-invalid",
                                "Session preset is invalid",
                                invalid.detail(),
                            ),
                        })?;
                let mut result = serde_json::json!({
                    "base": preset.base,
                    "layers": preset.summary()["layers"],
                    "presetText": preset.preset_text(),
                    "digest": preset.digest,
                });
                if let Some(tools) = &preset.tools {
                    result["tools"] = serde_json::json!(tools);
                }
                if !preset.mcp.is_empty() {
                    result["mcp"] = serde_json::json!(preset.mcp);
                }
                if let Some(skills) = &preset.skills {
                    result["skills"] = serde_json::json!(skills);
                }
                if let Some(defaults) = &preset.defaults {
                    let mut value =
                        serde_json::json!({"provider":defaults.provider,"model":defaults.model});
                    if let Some(effort) = &defaults.reasoning_effort {
                        value["reasoningEffort"] = serde_json::json!(effort);
                    }
                    result["defaults"] = value;
                }
                if !preset.mcp.is_empty() {
                    result["mcpReadiness"] = serde_json::json!([]);
                }
                encode(&result)
            }
            "session.initialPresets" => encode(&serde_json::json!({
                "presets": [
                    {"id":"coding", "title":"Coding", "configurationText":tools::IdentityProfile::Coding.profile_text()},
                    {"id":"general", "title":"General", "configurationText":tools::IdentityProfile::General.profile_text()}
                ],
                "defaultID": null
            })),
            "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
                let request: FilePageRequest = decode(payload)?;
                let text = operation == "session.files.read";
                let mut page = self
                    .process
                    .client_file_page(
                        &request.session_id,
                        &request.path,
                        text,
                        request.offset.unwrap_or(if text { 1 } else { 0 }),
                        request.limit.unwrap_or(if text {
                            200
                        } else if operation == "session.files.stat" {
                            1
                        } else {
                            256 * 1024
                        }),
                    )
                    .map_err(|error| {
                        failure(
                            "file-unavailable",
                            "Session file is unavailable",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                if operation == "session.files.stat" {
                    page.as_object_mut().unwrap().retain(|key, _| {
                        ["absolutePath", "version", "bytes"].contains(&key.as_str())
                    });
                }
                encode(&page)
            }
            "attachments.policy" => encode(&endpoint::AttachmentPolicy::default()),
            "session.uploadFile" => {
                let request: UploadFileRequest = decode(payload)?;
                let receipt = self
                    .attachments()
                    .upload_file(
                        &request.session_id,
                        &request.name,
                        request.media_type.as_deref(),
                        &request.data,
                    )
                    .map_err(|error| map_prompt_materialize_error(&request.session_id, error))?;
                encode(&receipt)
            }
            "session.fileAttachment" => {
                let request: FileAttachmentRequest = decode(payload)?;
                let attachment = self
                    .attachments()
                    .read_authorized_file(&request.session_id, &request.attachment_id)
                    .map_err(|error| map_attachment_read_error(&request.session_id, error))?;
                encode(&attachment)
            }
            "approvals.policy" => encode(&endpoint::ApprovalPolicy::default()),
            "approvals.mode" => {
                let request: ApprovalModeRequest = decode(payload)?;
                let (folder, _archived) = self.session_folder(&request.session_id)?;
                let read = engine::read_permission_mode(&folder);
                if let Some(diagnostic) = read.diagnostic {
                    eprintln!("tekes-supervisor: permission mode {diagnostic}");
                }
                encode(&serde_json::json!({"mode": read.mode.as_str()}))
            }
            "approvals.select" => {
                let request: ApprovalSelectRequest = decode(payload)?;
                let mode = engine::PermissionMode::parse(&request.mode)
                    .ok_or_else(|| bad_request("unknown permission mode"))?;
                let (folder, archived) = self.session_folder(&request.session_id)?;
                if archived {
                    return Err(failure(
                        "archived",
                        "Session is archived",
                        serde_json::json!({"sessionId": request.session_id}),
                    ));
                }
                engine::write_permission_mode(&folder, mode).map_err(|error| {
                    failure(
                        "internal",
                        "Permission mode could not be written",
                        serde_json::json!({"reason": error.to_string()}),
                    )
                })?;
                // The mode rides on the session's inventory summary: re-announce
                // the session so every subscriber's composer follows it.
                crate::endpoint_host::SessionDeliveryAuthority::session_metadata_changed(
                    self.process.as_ref(),
                    &request.session_id,
                );
                encode(&serde_json::json!({"mode": mode.as_str()}))
            }

            "subagents.history" => {
                let (folder, _) =
                    self.session_folder(payload["parentSessionId"].as_str().unwrap_or_default())?;
                encode(&crate::bounded_child_history::read(&folder, payload)?)
            }
            operation if session_controls::METHODS.contains(&operation) => {
                self.session_controls(operation, payload)
            }
            operation if user_documents::METHODS.contains(&operation) => {
                let value = user_documents::execute(&self.root, operation, payload)
                    .map_err(|error| failure(error.code, &error.message, error.details))?;
                encode(&value)
            }
            "sessions.recover" => {
                let request: RecoveryRequest = decode(payload)?;
                self.process
                    .recover_client_sessions(&request.session_ids)
                    .map_err(|error| {
                        failure(
                            "session-not-found",
                            "Session recovery failed",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                encode(&serde_json::json!({"recoveredSessionIds":request.session_ids}))
            }
            "skills/list" | "commands/list" => {
                if let Some(session_id) = payload.get("sessionId").and_then(Value::as_str) {
                    let catalog =
                        self.process
                            .client_resource_catalog(session_id)
                            .map_err(|error| {
                                failure(
                                    "resource-not-found",
                                    "Session resources are unavailable",
                                    serde_json::json!({"reason":error.to_string()}),
                                )
                            })?;
                    if operation == "skills/list" {
                        encode(
                            &serde_json::json!({"skills": catalog.skill_summaries().iter().map(|skill| serde_json::json!({
                            "name":skill.name, "description":skill.description, "modelInvocable":true
                        })).collect::<Vec<_>>()}),
                        )
                    } else {
                        encode(
                            &serde_json::json!({"commands":catalog.command_summaries().iter().map(|command| serde_json::json!({
                            "name":command.name, "description":command.description, "argumentHint":command.argument_hint, "supportsAttachments":true
                        })).collect::<Vec<_>>()}),
                        )
                    }
                } else if operation == "skills/list" {
                    encode(&self.commands.skills_list())
                } else {
                    encode(&self.commands.commands_list())
                }
            }
            "commands/run" => {
                let input: CommandRunRequest = decode(payload)?;
                self.commands
                    .validate_session(&input.session_id)
                    .map_err(map_resource_failure)?;
                let catalog = self
                    .process
                    .client_resource_catalog(&input.session_id)
                    .map_err(|error| {
                        failure(
                            "invalid-command",
                            "Session command catalog is unavailable",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                encode(
                    &self
                        .commands
                        .commands_run_with_catalog(&catalog, &input, principal)
                        .map_err(map_resource_failure)?,
                )
            }
            "resources/list" => self.resources_list(payload),
            "resources/read" => self.resources_read(payload),
            "tools/list" => {
                let request: ToolListRequest = decode(payload)?;
                self.tools(&request.session_id, None)
            }
            "tools/resolve" => {
                let request: ToolResolveRequest = decode(payload)?;
                self.tools(&request.session_id, Some(&request.name))
            }

            "thread.search" => self.thread_search(payload),
            "session.search" => {
                let request: SessionSearchRequest = decode(payload)?;
                encode(
                    &self
                        .search
                        .search_sessions(&request.query)
                        .map_err(map_search_error)?,
                )
            }
            "usage.summary" => self.usage(payload, false),
            "usage.cacheAttribution" => self.usage(payload, true),
            _ => Err(failure(
                "unsupported-capability",
                "Capability is unavailable",
                serde_json::json!({"operation":operation}),
            )),
        }
    }
}

impl ProductionClientExtensions {
    fn resources_list(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ResourceListRequest = decode(payload)?;
        let limit = request.limit.unwrap_or(100);
        if !(1..=100).contains(&limit) {
            return Err(bad_request("limit must be 1...100"));
        }
        match request.source.as_str() {
            "skill" => {
                let catalog = self
                    .process
                    .client_resource_catalog(&request.session_id)
                    .map_err(|_| {
                        failure(
                            "resource-not-found",
                            "Session resources are unavailable",
                            serde_json::json!({}),
                        )
                    })?;

                let mut items = Vec::new();
                for package in catalog.skill_packages() {
                    for resource in &package.resources {
                        items.push(serde_json::json!({
                            "reference":{"kind":"skill","sessionId":request.session_id,"name":package.summary.name,"contentDigest":package.summary.content_digest},
                            "uri":resource.path,
                            "name":resource.path,"mimeType":"text/markdown",
                            "description":package.summary.description
                        }));
                    }
                }
                items.sort_by(|left, right| left["uri"].as_str().cmp(&right["uri"].as_str()));
                let generation = format!(
                    "sha256-{:x}",
                    sha2::Sha256::digest(serde_json_canonicalizer::to_vec(&items).map_err(
                        |_| failure(
                            "internal",
                            "Skill resources cannot be canonicalized",
                            serde_json::json!({})
                        )
                    )?)
                );
                let start = match request.after {
                    None => 0,
                    Some(cursor) => decode_resource_cursor(&cursor, &generation)?,
                };
                if start > items.len() {
                    return Err(failure(
                        "resource-stale",
                        "Skill resource cursor is stale",
                        serde_json::json!({}),
                    ));
                }
                let end = start.saturating_add(limit).min(items.len());
                let next = (end < items.len()).then(|| format!("{generation}:{end}"));
                let mut value = serde_json::json!({"format":1,"items":items[start..end]});
                if let Some(next) = next {
                    value["next"] = Value::String(next);
                }
                encode(&value)
            }

            _ => Err(bad_request("source is not skill or mcp")),
        }
    }

    fn resources_read(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ResourceReadRequest = decode(payload)?;
        match serde_json::from_value::<ResourceReference>(request.reference)
            .map_err(|_| bad_request("reference is invalid"))?
        {
            ResourceReference::Skill {
                session_id,
                name,
                content_digest,
            } => {
                if session_id != request.session_id {
                    return Err(failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    ));
                }
                let catalog = self
                    .process
                    .client_resource_catalog(&request.session_id)
                    .map_err(|_| {
                        failure(
                            "resource-not-found",
                            "Session resources are unavailable",
                            serde_json::json!({}),
                        )
                    })?;

                let package = catalog.skill(&name).map_err(|_| {
                    failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    )
                })?;
                if package.summary.content_digest != content_digest {
                    return Err(failure(
                        "resource-stale",
                        "Skill package generation changed",
                        serde_json::json!({}),
                    ));
                }
                let text = catalog.skill_resource(&name, &request.uri).map_err(|_| {
                    failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    )
                })?;
                if text.len() > 4 * 1024 * 1024 {
                    return Err(failure(
                        "resource-too-large",
                        "Resource exceeds 4 MiB",
                        serde_json::json!({}),
                    ));
                }
                encode(
                    &serde_json::json!({"format":1,"content":{"text":text,"mimeType":"text/markdown"}}),
                )
            }
        }
    }

    fn tools(
        &self,
        session_id: &str,
        resolve: Option<&str>,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let dynamic = self.process.client_tool_catalog(session_id).map_err(|_| {
            failure(
                "internal",
                "Tool catalog is unavailable",
                serde_json::json!({}),
            )
        })?;
        let mut rows = Vec::new();
        let fixed = BuiltinManifest::compiled();
        for tool in &fixed.tools {
            let schema = fixed_schema(&tool.name).ok_or_else(|| {
                failure(
                    "internal",
                    "Compiled tool schema is absent",
                    serde_json::json!({}),
                )
            })?;
            rows.push(serde_json::json!({
                "name":tool.name,"description":schema.description,
                "arguments":schema.parameters.to_json_schema(),"availability":tool.availability,
                "backend":tool.backend,"effect":tool.effect,"source":{"kind":"builtin"}
            }));
        }
        for tool in dynamic.tools {
            let schema = ijson_value(&tool.schema)?;
            rows.push(serde_json::json!({
                "name":tool.name,"description":schema.get("description").and_then(Value::as_str).unwrap_or(""),
                "arguments":schema.get("parameters").cloned().unwrap_or_else(|| serde_json::json!({})),
                "availability":if tool.always_on {"always"} else {"deferred"},
                "backend":"supervisor_control","effect":tool.effect,"source":tool.source
            }));
        }
        rows.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
        let digest = format!(
            "sha256-{:x}",
            sha2::Sha256::digest(serde_json_canonicalizer::to_vec(&rows).map_err(|_| failure(
                "internal",
                "Tool catalog cannot be canonicalized",
                serde_json::json!({})
            ))?)
        );
        if let Some(name) = resolve {
            let tool = rows
                .into_iter()
                .find(|row| row["name"] == name)
                .ok_or_else(|| {
                    failure(
                        "tool-not-found",
                        "Tool was not found",
                        serde_json::json!({"name":name}),
                    )
                })?;
            encode(&serde_json::json!({"format":1,"catalogDigest":digest,"tool":tool}))
        } else {
            encode(&serde_json::json!({"format":1,"catalogDigest":digest,"tools":rows}))
        }
    }

    fn thread_search(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ThreadSearchRequest = decode(payload)?;
        let page = self
            .search
            .search(&SearchRequest {
                session_filter: request.session_id,
                query: request.query,
                limit: request.limit,
                visibility: request.visibility,
                after: request.after,
            })
            .map_err(map_search_error)?;
        let mut result = serde_json::json!({"format":1,"results":page.results,"reachedEnd":page.reached_end,"catalogDigest":page.catalog_digest});
        if let Some(next) = page.next_cursor {
            result["nextCursor"] = Value::String(next);
        }
        encode(&result)
    }

    fn usage(&self, payload: &Value, cache: bool) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: SessionIdRequest = decode(payload)?;
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(&request.session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| {
                failure(
                    "session-not-found",
                    "Session was not found",
                    serde_json::json!({}),
                )
            })?;
        let bytes = std::fs::read(folder.join("main.jsonl")).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage source is unreadable",
                serde_json::json!({}),
            )
        })?;
        let assets = store::AssetStore::new(folder.join("assets")).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage assets are corrupt",
                serde_json::json!({}),
            )
        })?;
        let scan = store::scan_valid_prefix(&bytes, schema::READER_VERSION);
        // A torn tail (a partial append, not yet repaired) is ignored: the valid prefix is the
        // ledger. A complete line that does not validate is damage, and the source is corrupt.
        let valid_len = usize::try_from(scan.valid_bytes).unwrap_or(usize::MAX);
        if scan.needs_repair()
            && bytes
                .get(valid_len..)
                .is_some_and(|tail| tail.contains(&b'\n'))
        {
            return Err(failure(
                "source-corrupt",
                "Usage source has an invalid record",
                serde_json::json!({}),
            ));
        }
        let projection = scan.projection.ok_or_else(|| {
            failure(
                "source-corrupt",
                "Usage source has no projection",
                serde_json::json!({}),
            )
        })?;
        let mut epoch_targets = BTreeMap::<String, Value>::new();
        let mut attempt_epochs = BTreeMap::<String, String>::new();
        if cache {
            for event in &projection.events {
                match event.kind() {
                    EventKind::Epoch => {
                        let raw = ijson_value(event.raw())?;
                        let id = event.string_field("id").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Epoch identity is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        let target = usage_route_target(&raw, &assets)?;
                        if epoch_targets.insert(id.to_owned(), target).is_some() {
                            return Err(failure(
                                "source-corrupt",
                                "Epoch identity is duplicated",
                                serde_json::json!({}),
                            ));
                        }
                    }
                    EventKind::Attempt => {
                        let attempt = event.string_field("attempt").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Attempt identity is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        let epoch = event.string_field("epoch").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Attempt epoch is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        if attempt_epochs
                            .insert(attempt.to_owned(), epoch.to_owned())
                            .is_some()
                        {
                            return Err(failure(
                                "source-corrupt",
                                "Attempt identity is duplicated",
                                serde_json::json!({}),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut input = 0u64;
        let mut output = 0u64;
        let mut groups = BTreeMap::<Vec<u8>, CacheAttributionGroup>::new();
        for event in &projection.events {
            // The settling output/error carries its attempt's usage.
            let settles_attempt = matches!(event.kind(), EventKind::Output)
                || matches!(event.kind(), EventKind::Error) && event.has_field("attempt");
            if !settles_attempt {
                continue;
            }
            let outcome = ijson_value(event.raw())?;
            let raw = outcome.get("usage").cloned().unwrap_or(Value::Null);
            let availability = raw
                .get("availability")
                .and_then(Value::as_str)
                .unwrap_or("unavailable");
            let parsed_input = numeric_token(raw.get("input_tokens"))?;
            let parsed_output = numeric_token(raw.get("output_tokens"))?;
            input = input.checked_add(parsed_input).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            output = output.checked_add(parsed_output).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            if cache && availability == "reported" {
                let (Some(input_tokens), Some(output_tokens), Some(cache_read_tokens)) = (
                    raw.get("input_tokens").and_then(Value::as_str),
                    raw.get("output_tokens").and_then(Value::as_str),
                    raw.get("cache_read").and_then(Value::as_str),
                ) else {
                    continue;
                };
                canonical_unsigned(input_tokens)?;
                canonical_unsigned(output_tokens)?;
                canonical_unsigned(cache_read_tokens)?;
                let attempt = event.string_field("attempt").ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage attempt is absent",
                        serde_json::json!({}),
                    )
                })?;
                let epoch = attempt_epochs.get(attempt).ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage attempt has no epoch binding",
                        serde_json::json!({}),
                    )
                })?;
                let target = epoch_targets.get(epoch).ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage epoch has no exact target",
                        serde_json::json!({}),
                    )
                })?;
                let key = serde_json_canonicalizer::to_vec(target).map_err(|_| {
                    failure(
                        "source-corrupt",
                        "Usage target cannot be canonicalized",
                        serde_json::json!({}),
                    )
                })?;
                groups
                    .entry(key)
                    .or_insert_with(|| CacheAttributionGroup::new(target.clone()))
                    .add(
                        attempt,
                        input_tokens,
                        output_tokens,
                        cache_read_tokens,
                        raw.get("cost").and_then(Value::as_str),
                    )?;
            }
        }
        if cache {
            let entries = groups
                .into_values()
                .map(CacheAttributionGroup::value)
                .collect::<Result<Vec<_>, _>>()?;
            encode(
                &serde_json::json!({"format":1,"sessionId":request.session_id,"asOfSeq":projection.last_seq,"entries":entries}),
            )
        } else {
            let total = input.checked_add(output).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            if [input, output, total]
                .into_iter()
                .any(|value| value > 9_007_199_254_740_991)
            {
                return Err(failure(
                    "source-corrupt",
                    "Usage total exceeds the I-JSON safe integer range",
                    serde_json::json!({}),
                ));
            }
            encode(
                &serde_json::json!({"format":1,"sessionId":request.session_id,"asOfSeq":projection.last_seq,"inputTokens":input,"outputTokens":output,"totalTokens":total}),
            )
        }
    }
}

/// Mirrors `session.attachment` in `endpoint_host`: lifecycle codes keep the
/// session identity, validation is the closed `attachment-error {reason}`,
/// and storage defects fold to `internal`.
fn map_attachment_read_error(
    session_id: &str,
    error: endpoint::AttachmentReadError,
) -> ProductionRouteFailure {
    match error {
        endpoint::AttachmentReadError::SessionNotFound(_) => failure(
            "session-not-found",
            "Session was not found",
            serde_json::json!({"sessionId": session_id}),
        ),
        endpoint::AttachmentReadError::Archived(_) => failure(
            "archived",
            "Session is archived",
            serde_json::json!({"sessionId": session_id}),
        ),
        endpoint::AttachmentReadError::Attachment(reason) => failure(
            "attachment-error",
            "Image attachment is unavailable",
            serde_json::json!({"reason": reason}),
        ),
        _ => failure(
            "internal",
            "Endpoint operation failed",
            serde_json::json!({}),
        ),
    }
}

fn error_allowed(operation: &str, class: Option<MethodClass>, code: &str) -> bool {
    if matches!(code, "bad-request" | "unsupported-capability" | "internal")
        || (class == Some(MethodClass::Mutation) && code == "idempotency-conflict")
    {
        return true;
    }
    match operation {
        "session.presets.resolve" => code == "preset-invalid",
        "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
            code == "file-unavailable"
        }
        "sessions.recover" => code == "session-not-found",
        "approvals.mode" | "approvals.select" => matches!(code, "session-not-found" | "archived"),
        "session.uploadFile" | "session.fileAttachment" => {
            matches!(code, "attachment-error" | "session-not-found" | "archived")
        }

        value if value.starts_with("feedback.") => matches!(code, "version-conflict" | "io"),
        value if value.starts_with("settings.") => matches!(
            code,
            "unknown-namespace" | "stale-revision" | "settings-invalid" | "io"
        ),
        value if value.starts_with("goals.") => matches!(
            code,
            "goal-stale"
                | "goal-not-found"
                | "goal-invalid-transition"
                | "session-not-found"
                | "ledger-corrupt"
        ),
        "subagents.list" | "subagents.history" => matches!(
            code,
            "session-not-found"
                | "ledger-corrupt"
                | "child-unavailable"
                | "history-stale"
                | "history-limit"
        ),
        "skills/list" | "commands/list" => code == "resource-not-found",
        "commands/run" => matches!(
            code,
            "invalid-command"
                | "invalid-arguments"
                | "command-not-found"
                | "session-not-found"
                | "session-archived"
                | "session-busy"
                | "delivery-timeout"
                | "delivery-rejected"
                | "attachment-error"
        ),
        "resources/list" | "resources/read" => matches!(
            code,
            "resource-not-found" | "resource-stale" | "resource-too-large"
        ),
        "tools/list" | "tools/resolve" => code == "tool-not-found",

        "thread.search" | "session.search" => matches!(
            code,
            "invalid-session-filter"
                | "invalid-query"
                | "invalid-limit"
                | "malformed-cursor"
                | "cursor-scope"
                | "cursor-stale"
                | "cursor-position"
                | "source-identity"
                | "source-corrupt"
        ),
        "usage.summary" | "usage.cacheAttribution" => {
            matches!(code, "session-not-found" | "source-corrupt")
        }
        _ => false,
    }
}

fn validate_payload(operation: &str, payload: &Value) -> Result<(), ProductionRouteFailure> {
    match operation {
        "session.presets.describe" => {
            if payload.as_object().is_some_and(serde_json::Map::is_empty) {
                Ok(())
            } else {
                Err(bad_request("request must be empty"))
            }
        }
        "session.presets.resolve" => {
            // The preset itself is checked by the resolver, which reports
            // malformed members as bad-request and refusals as preset-invalid.
            let exact = payload.as_object().is_some_and(|object| {
                object.len() == 2
                    && object.get("execution").is_some_and(|value| {
                        serde_json::from_value::<schema::ExecutionConfig>(value.clone())
                            .is_ok_and(|execution| execution.validate().is_ok())
                    })
                    && object.contains_key("preset")
            });
            if exact {
                Ok(())
            } else {
                Err(bad_request("request is {execution, preset}"))
            }
        }
        "session.initialPresets" => {
            if payload.as_object().is_some_and(serde_json::Map::is_empty) {
                Ok(())
            } else {
                Err(bad_request("request must be empty"))
            }
        }
        "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
            let request: FilePageRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.path.is_empty()
                || request.path.contains('\0')
                || request.limit == Some(0)
                || (operation == "session.files.read" && request.offset == Some(0))
                || (operation == "session.files.stat"
                    && (request.offset.is_some() || request.limit.is_some()))
                || request.limit.is_some_and(|limit| {
                    limit
                        > if operation == "session.files.read" {
                            10000
                        } else {
                            4 * 1024 * 1024
                        }
                })
            {
                return Err(bad_request("Invalid file range"));
            }
            Ok(())
        }
        "subagents.history" => {
            crate::bounded_child_history::validate(payload).map_err(|reason| bad_request(&reason))
        }
        operation if session_controls::METHODS.contains(&operation) => {
            session_controls::validate(operation, payload).map_err(|reason| bad_request(&reason))
        }
        operation if user_documents::METHODS.contains(&operation) => {
            user_documents::validate(operation, payload).map_err(|reason| bad_request(&reason))
        }

        "attachments.policy" | "approvals.policy" => {
            let request: AttachmentPolicyRequest = decode(payload)?;
            if let Some(id) = request.session_id {
                endpoint::validate_session_id(&id)
                    .map_err(|_| bad_request("Invalid session identity"))?;
            }
            Ok(())
        }
        "approvals.mode" => {
            let request: ApprovalModeRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            Ok(())
        }
        "approvals.select" => {
            let request: ApprovalSelectRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if engine::PermissionMode::parse(&request.mode).is_none() {
                return Err(bad_request("unknown permission mode"));
            }
            Ok(())
        }
        "session.uploadFile" => {
            let request: UploadFileRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.name.is_empty() || request.data.is_empty() {
                return Err(bad_request("Invalid upload request"));
            }
            Ok(())
        }
        "session.fileAttachment" => {
            let request: FileAttachmentRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.attachment_id.is_empty() {
                return Err(bad_request("Invalid attachment identity"));
            }
            Ok(())
        }
        "sessions.recover" => decode::<RecoveryRequest>(payload).map(|_| ()),
        "skills/list" | "commands/list" => {
            let object = payload
                .as_object()
                .ok_or_else(|| bad_request("object required"))?;
            if object.is_empty()
                || (object.len() == 1
                    && object
                        .get("sessionId")
                        .and_then(Value::as_str)
                        .is_some_and(|id| !id.is_empty()))
            {
                Ok(())
            } else {
                Err(bad_request("request must be empty"))
            }
        }
        "commands/run" => decode::<CommandRunRequest>(payload).map(|_| ()),
        "resources/list" => decode::<ResourceListRequest>(payload).map(|_| ()),
        "resources/read" => decode::<ResourceReadRequest>(payload).map(|_| ()),
        "tools/list" => decode::<ToolListRequest>(payload).map(|_| ()),
        "tools/resolve" => {
            let request = decode::<ToolResolveRequest>(payload)?;
            if !request.name.is_empty() {
                Ok(())
            } else {
                Err(bad_request("tools/resolve requires name"))
            }
        }

        "thread.search" => decode::<ThreadSearchRequest>(payload).map(|_| ()),
        "session.search" => decode::<SessionSearchRequest>(payload).map(|_| ()),
        "usage.summary" | "usage.cacheAttribution" => {
            decode::<SessionIdRequest>(payload).map(|_| ())
        }
        _ => Err(failure(
            "unsupported-capability",
            "Capability is unavailable",
            serde_json::json!({"operation":operation}),
        )),
    }
}

pub(crate) fn failure(code: &str, message: &str, details: Value) -> ProductionRouteFailure {
    let details = IJsonValue::parse(
        &serde_json::to_vec(&details).expect("extension failure details are JSON"),
    )
    .expect("extension failure details are I-JSON");
    ProductionRouteFailure::new(code, message, details)
}

pub const FOLDER_TRUST_METHODS: [(&str, MethodClass); 3] = [
    ("folders.trust.status", MethodClass::ReadOnly),
    ("folders.trust.request", MethodClass::Mutation),
    ("folders.trust.revoke", MethodClass::Mutation),
];

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::endpoint_host::{SessionDeliveryAuthority, SessionInputAdmissionAuthority};

    use super::*;

    fn production_extensions(
        root: &Path,
    ) -> (Arc<ProductionClientExtensions>, Arc<ProductionProcessHost>) {
        let agent = root.join(".agent");
        fs::create_dir_all(&agent).expect("agent root");
        let process = ProductionProcessHost::open(
            root,
            std::env::current_exe().expect("test executable"),
            "1.0.0",
            &agent,
        )
        .expect("process host");
        let admission = Arc::new(SessionInputAdmissionAuthority::new(root.to_path_buf()));
        let commands = EndpointCommandInputAuthority::new(
            Arc::clone(&process) as Arc<dyn SessionDeliveryAuthority>,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            admission,
        );
        let extensions = Arc::new(
            ProductionClientExtensions::open(
                root,
                ResourceCatalog::default(),
                commands,
                Arc::clone(&process),
            )
            .expect("extensions"),
        );
        (extensions, process)
    }

    #[test]
    fn initial_presets_disclose_profile_text_without_creating_a_session() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, _process) = production_extensions(root.path());
        let catalog = execute(
            &extensions,
            "session.initialPresets",
            &serde_json::json!({}),
            "presets",
        )
        .expect("catalog");
        assert_eq!(catalog["defaultID"], Value::Null);
        assert_eq!(catalog["presets"][0]["id"], "coding");
        assert_eq!(
            catalog["presets"][0]["configurationText"],
            tools::IdentityProfile::Coding.profile_text()
        );
        assert_eq!(
            catalog["presets"][1]["configurationText"],
            tools::IdentityProfile::General.profile_text()
        );
        assert!(
            catalog["presets"][0]["configurationText"]
                .as_str()
                .unwrap()
                .contains("{model}")
        );
        assert!(
            !root.path().join("threads").exists()
                || fs::read_dir(root.path().join("threads"))
                    .unwrap()
                    .next()
                    .is_none()
        );
        assert!(
            validate_payload(
                "session.initialPresets",
                &serde_json::json!({"sessionId":"x"})
            )
            .is_err()
        );
    }

    #[test]
    fn session_presets_describe_and_resolve_without_creating_a_session() {
        let root = tempfile::tempdir().unwrap();
        let project = tempfile::tempdir().unwrap();
        let (extensions, _process) = production_extensions(root.path());
        let described = execute(
            &extensions,
            "session.presets.describe",
            &serde_json::json!({}),
            "describe",
        )
        .unwrap();
        assert_eq!(described["kernelLayers"][0]["id"], "coding");
        let preview=execute(&extensions,"session.presets.resolve",&serde_json::json!({"execution":{"cwd":project.path(),"allowed_tools":["read"]},"preset":{"tools":["read"]}}),"preview").unwrap();
        assert_eq!(preview["tools"], serde_json::json!(["read"]));
        assert!(
            std::fs::read_dir(root.path().join("threads"))
                .unwrap()
                .next()
                .is_none()
        );
    }

    fn execute(
        extensions: &ProductionClientExtensions,
        operation: &str,
        payload: &Value,
        rpc_id: &str,
    ) -> Result<Value, ProductionRouteFailure> {
        let result = ClientExtensionAuthority::execute(
            extensions,
            operation,
            payload,
            "test-client",
            rpc_id,
            false,
        )?;
        ijson_value(&result)
    }

    /// `approvals.select` durably writes the per-session mode record,
    /// `approvals.mode` reads it back (absent = workspace-write), the option
    /// ids published by `approvals.policy` are exactly the engine's modes,
    /// an unknown mode is `bad-request`, and an archived session is refused.
    #[test]
    fn approvals_select_persists_the_session_mode_and_refuses_archived_sessions() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, _process) = production_extensions(root.path());
        let session = "018f0000-0000-7000-8000-000000000031";
        let archived = "018f0000-0000-7000-8000-000000000032";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(&folder).expect("thread folder");
        fs::create_dir_all(root.path().join("archive").join(archived)).expect("archive folder");

        let policy = execute(
            &extensions,
            "approvals.policy",
            &serde_json::json!({}),
            "policy",
        )
        .expect("policy");
        let published: Vec<&str> = policy["threadLevel"]
            .as_array()
            .expect("thread options")
            .iter()
            .map(|option| option["value"].as_str().expect("value"))
            .collect();
        assert_eq!(
            published,
            engine::PermissionMode::ALL.map(engine::PermissionMode::as_str)
        );
        assert_eq!(
            policy["serverLevel"],
            serde_json::json!({"options":[],"currentValue":null})
        );

        let absent = execute(
            &extensions,
            "approvals.mode",
            &serde_json::json!({"sessionId":session}),
            "mode-0",
        )
        .expect("mode");
        assert_eq!(absent, serde_json::json!({"mode":"workspace-write"}));

        let selected = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":session,"mode":"read-only"}),
            "select-1",
        )
        .expect("select");
        assert_eq!(selected, serde_json::json!({"mode":"read-only"}));
        assert_eq!(
            fs::read(folder.join("permission-mode.json")).expect("record"),
            b"{\"format\":1,\"mode\":\"read-only\"}\n"
        );
        let read = execute(
            &extensions,
            "approvals.mode",
            &serde_json::json!({"sessionId":session}),
            "mode-1",
        )
        .expect("mode");
        assert_eq!(read, serde_json::json!({"mode":"read-only"}));

        let unknown = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":session,"mode":"yolo"}),
            "select-2",
        )
        .expect_err("unknown mode");
        assert_eq!(unknown.code, "bad-request");
        let refused = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":archived,"mode":"read-only"}),
            "select-3",
        )
        .expect_err("archived");
        assert_eq!(refused.code, "archived");
        assert!(
            !root
                .path()
                .join("archive")
                .join(archived)
                .join("permission-mode.json")
                .exists()
        );
        let missing = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033","mode":"read-only"}),
            "select-4",
        )
        .expect_err("missing");
        assert_eq!(missing.code, "session-not-found");
        for (operation, class, code) in [
            ("approvals.select", MethodClass::Mutation, "archived"),
            ("approvals.mode", MethodClass::ReadOnly, "session-not-found"),
        ] {
            assert!(
                error_allowed(operation, Some(class), code),
                "{operation} {code}"
            );
        }
    }

    #[test]
    fn tools_list_is_closed_and_includes_every_fixed_tool() {
        assert!(
            validate_payload(
                "tools/list",
                &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033","name":"web_search"}),
            )
            .is_err(),
            "the list DTO must reject resolve-only fields"
        );
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-000000000033";
        endpoint::NativeEndpoint::open(root.path())
            .unwrap()
            .create_session(
                session,
                &test_support::execution_fixture(),
                "cfg",
                schema::ResumePolicy::Never,
                "2026-08-29T00:00:00.000Z",
                &schema::OriginTuple {
                    principal: "test".into(),
                    client: "test".into(),
                    target: session.into(),
                    op: "create".into(),
                    key: "create".into(),
                },
            )
            .unwrap();
        let (extensions, _) = production_extensions(root.path());
        let result = execute(
            &extensions,
            "tools/list",
            &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033"}),
            "rpc-tools-list",
        )
        .expect("tool list");
        let tools = result["tools"].as_array().expect("tools");
        let fixed = BuiltinManifest::compiled();
        for expected in &fixed.tools {
            assert!(tools.iter().any(|tool| tool["name"] == expected.name));
        }
        assert_eq!(fixed.tools.len(), 25);

        let resolved = execute(
            &extensions,
            "tools/resolve",
            &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033","name":"web_search"}),
            "rpc-tools-resolve",
        )
        .expect("conditional fixed tool resolves for introspection");
        assert_eq!(resolved["tool"]["name"], "web_search");
    }

    fn append_canonical(lines: &mut Vec<u8>, value: Value) {
        lines.extend(serde_json_canonicalizer::to_vec(&value).expect("canonical event"));
        lines.push(b'\n');
    }

    fn epoch_profile_asset(
        assets: &store::AssetStore,
        dialect: &str,
        profile: &str,
        model: &str,
        owner: &str,
    ) -> store::AssetRef {
        let value = serde_json::json!({
            "controls":{},
            "serializer_revision":format!("{dialect}-serializer-1"),
            "system":"system",
            "target":{
                "protocol_family":if dialect.contains("anthropic") {"anthropic_messages"} else {"responses"},
                "dialect_id":dialect,
                "model_profile_id":profile,
                "route":{
                    "endpoint_owner":owner,
                    "gateway_translation":"direct",
                    "exact_sku":model,
                    "evidence_revision":format!("{owner}-evidence-1")
                }
            }
        });
        assets
            .publish(&serde_json_canonicalizer::to_vec(&value).expect("profile bytes"))
            .expect("publish profile")
    }

    #[test]
    fn usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, process) = production_extensions(root.path());
        let session = "018f0000-0000-7000-8000-000000000003";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(&folder).expect("thread folder");
        let assets = store::AssetStore::new(folder.join("assets")).expect("assets");
        let anthropic = epoch_profile_asset(
            &assets,
            "anthropic_messages_v1",
            "anthropic_messages_v1:claude",
            "claude",
            "anthropic",
        );
        let openai = epoch_profile_asset(
            &assets,
            "openai_responses_v1",
            "openai_responses_v1:gpt-5",
            "gpt-5",
            "openai",
        );
        let mut ledger = Vec::new();
        append_canonical(
            &mut ledger,
            serde_json::json!({"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":session},"resume":"never","seq":1,"thread":session,"ts":"2026-08-29T00:00:00.000Z","v":1,"workspace":"ws"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"one","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":session},"seq":2,"ts":"2026-08-29T00:00:01.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"binary":"worker","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"2026-08-29T00:00:02.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":4,"trigger":{"inputs":[2]},"ts":"2026-08-29T00:00:03.000Z","turn":1,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"adapter":"anthropic_messages_v1","id":"e1","kind":"epoch","model":"claude","reason":"initial","renderer":1,"seq":5,"system":{"asset":anthropic.asset,"digest":anthropic.asset.strip_prefix("sha256-").expect("digest")},"tools":{"asset":"sha256-td1","digest":"td1"},"ts":"2026-08-29T00:00:04.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":2,"to":2}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":6,"ts":"2026-08-29T00:00:05.000Z","turn":1,"v":1,"wire_digest":"wd1"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a1","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"anthropic_messages_v1","fragments":"f1","version":1},"seq":7,"ts":"2026-08-29T00:00:07.000Z","turn":1,"v":1,"usage":{"availability":"reported","input_tokens":"10","output_tokens":"5","cache_read":"2","cost":"0.01"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":8,"ts":"2026-08-29T00:00:08.000Z","turn":1,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"two","type":"text"}],"kind":"input","origin_key":"i2","origin_tuple":{"client":"cli","key":"i2","op":"submit","principal":"p","target":session},"seq":9,"ts":"2026-08-29T00:00:09.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":10,"trigger":{"inputs":[9]},"ts":"2026-08-29T00:00:10.000Z","turn":2,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"adapter":"openai_responses_v1","id":"e2","kind":"epoch","model":"gpt-5","reason":"profile-change","renderer":1,"seq":11,"system":{"asset":openai.asset,"digest":openai.asset.strip_prefix("sha256-").expect("digest")},"tools":{"asset":"sha256-td2","digest":"td2"},"ts":"2026-08-29T00:00:11.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":9,"to":9}],"attempt":"a2","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":12,"ts":"2026-08-29T00:00:12.000Z","turn":2,"v":1,"wire_digest":"wd2"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a2","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f2","version":1},"seq":13,"ts":"2026-08-29T00:00:15.000Z","turn":2,"v":1,"usage":{"availability":"reported","input_tokens":"20","output_tokens":"7","cache_read":"3","cost":"0.02"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":14,"ts":"2026-08-29T00:00:16.000Z","turn":2,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"three","type":"text"}],"kind":"input","origin_key":"i3","origin_tuple":{"client":"cli","key":"i3","op":"submit","principal":"p","target":session},"seq":15,"ts":"2026-08-29T00:00:17.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":16,"trigger":{"inputs":[15]},"ts":"2026-08-29T00:00:18.000Z","turn":3,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":15,"to":15}],"attempt":"a3","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":17,"ts":"2026-08-29T00:00:19.000Z","turn":3,"v":1,"wire_digest":"wd3"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a3","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f3","version":1},"seq":18,"ts":"2026-08-29T00:00:21.000Z","turn":3,"v":1,"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1","cache_read":"4","cost":"0.03"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":19,"ts":"2026-08-29T00:00:22.000Z","turn":3,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"four","type":"text"}],"kind":"input","origin_key":"i4","origin_tuple":{"client":"cli","key":"i4","op":"submit","principal":"p","target":session},"seq":20,"ts":"2026-08-29T00:00:23.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":21,"trigger":{"inputs":[20]},"ts":"2026-08-29T00:00:24.000Z","turn":4,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":20,"to":20}],"attempt":"a4","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":22,"ts":"2026-08-29T00:00:25.000Z","turn":4,"v":1,"wire_digest":"wd4"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a4","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f4","version":1},"seq":23,"ts":"2026-08-29T00:00:27.000Z","turn":4,"v":1,"usage":{"availability":"reported","input_tokens":"2","output_tokens":"2"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":24,"ts":"2026-08-29T00:00:28.000Z","turn":4,"v":1}),
        );
        fs::write(folder.join("main.jsonl"), ledger).expect("ledger");

        let attribution = execute(
            &extensions,
            "usage.cacheAttribution",
            &serde_json::json!({"sessionId":session}),
            "usage-cache",
        )
        .expect("cache attribution");
        assert_eq!(attribution["entries"].as_array().expect("entries").len(), 2);
        assert_eq!(
            attribution["entries"][0]["target"]["dialectId"],
            "anthropic_messages_v1"
        );
        assert_eq!(attribution["entries"][0]["inputTokens"], "10");
        assert_eq!(
            attribution["entries"][1]["target"]["dialectId"],
            "openai_responses_v1"
        );
        assert_eq!(attribution["entries"][1]["attempts"], 2);
        assert_eq!(attribution["entries"][1]["inputTokens"], "21");
        assert_eq!(attribution["entries"][1]["outputTokens"], "8");
        assert_eq!(attribution["entries"][1]["cacheReadTokens"], "7");
        assert_eq!(attribution["entries"][1]["cost"], "0.05");
        let summary = execute(
            &extensions,
            "usage.summary",
            &serde_json::json!({"sessionId":session}),
            "usage-summary",
        )
        .expect("usage summary");
        assert_eq!(summary["inputTokens"], 33);
        assert_eq!(summary["outputTokens"], 15);
        process.shutdown();
    }
}
