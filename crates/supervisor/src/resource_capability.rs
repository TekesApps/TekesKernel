//! Versioned Client resource capabilities for user/project skills and commands.
//!
//! These methods are deliberately separate from the frozen Session Endpoint v3
//! base registrations. `commands/run` ends at the same keyed durable
//! input authority as `session.prompt`; it has no alternate transcript path.

use std::collections::BTreeSet;
use std::sync::Arc;

use endpoint::{
    AttachmentAuthority, DurableHandoffProof, EndpointHostCall, MaterializedPrompt, MethodClass,
    MutationReceipt, PromptMaterializeError, PromptPart, RpcDurableIdentity,
};
use profile::{CommandSummary, ResourceCatalog, ResourceError, SkillSummary};
use schema::{Block, IJsonValue, OriginTuple};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use sha2::Digest as _;
use thiserror::Error;

use crate::endpoint_host::{
    ProductionEndpointRoutes, ProductionRouteFailure, SessionDeliveryAuthority,
    SessionInputAdmissionAuthority, production_failure_is_exact_for,
};

pub const SKILLS_LIST: &str = "skills/list";
pub const COMMANDS_LIST: &str = "commands/list";
pub const COMMANDS_RUN: &str = "commands/run";
pub const RESOURCE_CAPABILITY_FORMAT: u64 = 1;
pub const RESOURCE_CAPABILITIES: [&str; 3] = [SKILLS_LIST, COMMANDS_LIST, COMMANDS_RUN];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillsListResult {
    pub format: u64,
    pub skills: Vec<SkillSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandsListResult {
    pub format: u64,
    pub commands: Vec<CommandSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandRunRequest {
    pub session_id: String,
    pub name: String,
    pub arguments: String,
    pub key: String,
    /// Optional prompt attachments (image parts or file receipts). They are
    /// materialized like `session.prompt` content and follow the expanded
    /// command text in request order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<PromptPart>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandRunResult {
    #[serde(rename = "commandId")]
    pub command_id: String,
    pub format: u64,
    pub accepted: bool,
    pub command: String,
    pub content_digest: String,
    pub seq: u64,
    pub deduplicated: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyedCommandInput {
    pub principal: String,
    pub session_id: String,
    pub key: String,
    pub command: String,
    pub content_digest: String,
    pub text: String,
}

/// The reserved command verb: an expanded body that is exactly this text is
/// the manual compaction request for the session, not an input.
pub const COMPACT_COMMAND_VERB: &str = "compact";

/// The reserved permission verb: an expanded body that is exactly
/// `permission <mode>` selects the session's durable permission mode, never
/// authors model input, and needs no catalog entry named `permission`.
pub const PERMISSION_COMMAND_VERB: &str = "permission";

/// `Some(mode)` when `text` is exactly the reserved permission verb followed
/// by one whitespace-separated mode token.
#[must_use]
pub fn permission_verb_mode(text: &str) -> Option<&str> {
    let mut tokens = text.split_whitespace();
    let verb = tokens.next()?;
    let mode = tokens.next()?;
    (verb == PERMISSION_COMMAND_VERB && tokens.next().is_none()).then_some(mode)
}

pub trait CommandInputAuthority: Send + Sync {
    fn submit(&self, input: &KeyedCommandInput)
    -> Result<MutationReceipt, ResourceCapabilityError>;

    /// Deliver the keyed command input followed by materialized prompt
    /// attachments. The default authority accepts only the attachment-free
    /// shape and reports the closed `unsupported-capability` delivery
    /// failure otherwise.
    fn submit_with_attachments(
        &self,
        input: &KeyedCommandInput,
        attachments: &[PromptPart],
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        if attachments.is_empty() {
            return self.submit(input);
        }
        Err(ResourceCapabilityError::Delivery(unsupported(
            "commands/run attachments",
        )))
    }

    /// Deliver the keyed manual compaction request (same origin shape as
    /// `submit`; the text is the verb and is not appended anywhere). An
    /// authority without compaction reports the closed
    /// `unsupported-capability` delivery failure.
    fn compact(
        &self,
        input: &KeyedCommandInput,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        let _ = input;
        Err(ResourceCapabilityError::Delivery(unsupported("compact")))
    }

    /// Durably select the session's permission mode (the `permission <mode>`
    /// verb). `mode` is already a validated wire id. An authority without a
    /// session store reports the closed `unsupported-capability` failure.
    fn select_permission(
        &self,
        input: &KeyedCommandInput,
        mode: engine::PermissionMode,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        let _ = (input, mode);
        Err(ResourceCapabilityError::Delivery(unsupported("permission")))
    }
}

pub struct ClientResourceService<A> {
    catalog: ResourceCatalog,
    input: A,
}

impl<A> ClientResourceService<A>
where
    A: CommandInputAuthority,
{
    #[must_use]
    pub const fn new(catalog: ResourceCatalog, input: A) -> Self {
        Self { catalog, input }
    }

    #[must_use]
    pub fn capabilities() -> BTreeSet<String> {
        RESOURCE_CAPABILITIES
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[must_use]
    pub fn skills_list(&self) -> SkillsListResult {
        SkillsListResult {
            format: RESOURCE_CAPABILITY_FORMAT,
            skills: self.catalog.skill_summaries(),
        }
    }

    #[must_use]
    pub fn commands_list(&self) -> CommandsListResult {
        CommandsListResult {
            format: RESOURCE_CAPABILITY_FORMAT,
            commands: self.catalog.command_summaries(),
        }
    }

    pub fn commands_run(
        &self,
        request: &CommandRunRequest,
        principal: &str,
    ) -> Result<CommandRunResult, ResourceCapabilityError> {
        self.commands_run_with_catalog(&self.catalog, request, principal)
    }

    pub fn commands_run_with_catalog(
        &self,
        catalog: &ResourceCatalog,
        request: &CommandRunRequest,
        principal: &str,
    ) -> Result<CommandRunResult, ResourceCapabilityError> {
        validate_run_request(request)?;
        // The reserved `permission` verb needs no catalog entry: the request
        // name is the verb and `arguments` is the mode. A user command that
        // happens to be named `permission` still takes precedence.
        let expanded =
            if request.name == PERMISSION_COMMAND_VERB && catalog.command(&request.name).is_err() {
                let mode = request.arguments.trim();
                if mode.is_empty() {
                    return Err(ResourceCapabilityError::Resource(
                        ResourceError::InvalidArguments("permission needs a mode".to_owned()),
                    ));
                }
                let text = format!("{PERMISSION_COMMAND_VERB} {mode}");
                profile::CommandExpansion {
                    name: request.name.clone(),
                    content_digest: format!("{:x}", sha2::Sha256::digest(text.as_bytes())),
                    text,
                }
            } else {
                catalog.expand_command(&request.name, &request.arguments)?
            };
        let keyed = KeyedCommandInput {
            principal: principal.to_owned(),
            session_id: request.session_id.clone(),
            key: request.key.clone(),
            command: expanded.name.clone(),
            content_digest: expanded.content_digest.clone(),
            text: expanded.text,
        };
        // A command whose whole expanded body is the reserved verb is the
        // legacy `/compact` control request: it never becomes model input.
        let receipt = if keyed.text.trim() == COMPACT_COMMAND_VERB {
            if !request.attachments.is_empty() {
                return Err(ResourceCapabilityError::Delivery(
                    ProductionRouteFailure::new(
                        "invalid-arguments",
                        "Command arguments are invalid",
                        empty_details(),
                    ),
                ));
            }
            self.input.compact(&keyed)?
        } else if let Some(mode) = permission_verb_mode(&keyed.text) {
            // `permission <mode>` selects the durable per-session mode; it
            // never becomes model input and an unknown mode is an argument
            // error, never a prompt.
            let Some(mode) = engine::PermissionMode::parse(mode) else {
                return Err(ResourceCapabilityError::Resource(
                    ResourceError::InvalidArguments(format!("unknown permission mode {mode:?}")),
                ));
            };
            if !request.attachments.is_empty() {
                return Err(ResourceCapabilityError::Delivery(
                    ProductionRouteFailure::new(
                        "invalid-arguments",
                        "Command arguments are invalid",
                        empty_details(),
                    ),
                ));
            }
            self.input.select_permission(&keyed, mode)?
        } else if request.attachments.is_empty() {
            self.input.submit(&keyed)?
        } else {
            self.input
                .submit_with_attachments(&keyed, &request.attachments)?
        };
        Ok(CommandRunResult {
            command_id: request.key.clone(),
            format: RESOURCE_CAPABILITY_FORMAT,
            accepted: true,
            command: expanded.name,
            content_digest: expanded.content_digest,
            seq: receipt.seq,
            deduplicated: receipt.deduplicated,
        })
    }
}

impl<A> ProductionEndpointRoutes for ClientResourceService<A>
where
    A: CommandInputAuthority,
{
    fn capabilities(&self) -> BTreeSet<String> {
        Self::capabilities()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        match method {
            SKILLS_LIST | COMMANDS_LIST => Some(MethodClass::ReadOnly),
            COMMANDS_RUN => Some(MethodClass::Mutation),
            _ => None,
        }
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match operation {
            SKILLS_LIST | COMMANDS_LIST
                if payload.as_object().is_some_and(|object| object.is_empty()) =>
            {
                Ok(())
            }
            COMMANDS_RUN => {
                let request = decode_payload::<CommandRunRequest>(payload)?;
                validate_run_request(&request).map_err(map_resource_failure)
            }
            _ => Err(bad_request()),
        }
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        if operation != COMMANDS_RUN {
            return base_failure_is_exact(failure);
        }
        base_failure_is_exact(failure)
            || resource_failure_is_exact(failure)
            || production_failure_is_exact_for("session.prompt", failure)
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match request.operation.as_str() {
            SKILLS_LIST => encode_result(&self.skills_list()),
            COMMANDS_LIST => encode_result(&self.commands_list()),
            COMMANDS_RUN => {
                let run = decode_payload::<CommandRunRequest>(payload)?;
                let result = self
                    .commands_run(&run, principal)
                    .map_err(map_resource_failure)?;
                request
                    .handoff
                    .mark_handed_off(DurableHandoffProof {
                        delivery: "locked-append".to_owned(),
                        durable_identity: Some(RpcDurableIdentity {
                            kind: "event-origin".to_owned(),
                            id: request.rpc_id.clone(),
                            seq: Some(result.seq),
                        }),
                    })
                    .map_err(|_| internal_failure())?;
                encode_result(&result)
            }
            _ => Err(unsupported(&request.operation)),
        }
    }
}

pub type ResourceClock = dyn Fn() -> Result<String, String> + Send + Sync;

impl ClientResourceService<EndpointCommandInputAuthority> {
    pub fn validate_session(&self, session_id: &str) -> Result<(), ResourceCapabilityError> {
        self.input.admission.with_active_session(
            session_id,
            ResourceCapabilityError::Delivery,
            || Ok(()),
        )
    }
}

/// Production adapter to the same session delivery authority used by
/// `session.prompt`. The command capability never appends an event itself.
pub struct EndpointCommandInputAuthority {
    delivery: Arc<dyn SessionDeliveryAuthority>,
    clock: Arc<ResourceClock>,
    admission: Arc<SessionInputAdmissionAuthority>,
    /// Same attachment authority as `session.prompt`; absent authorities
    /// accept only attachment-free commands.
    attachments: Option<AttachmentAuthority>,
}

impl EndpointCommandInputAuthority {
    #[must_use]
    pub fn new(
        delivery: Arc<dyn SessionDeliveryAuthority>,
        clock: Arc<ResourceClock>,
        admission: Arc<SessionInputAdmissionAuthority>,
    ) -> Self {
        Self {
            delivery,
            clock,
            admission,
            attachments: None,
        }
    }

    /// Installs the session attachment authority so `commands/run` can carry
    /// image parts and file receipts exactly like `session.prompt`.
    #[must_use]
    pub fn with_attachment_authority(mut self, attachments: AttachmentAuthority) -> Self {
        self.attachments = Some(attachments);
        self
    }

    fn deliver(
        &self,
        input: &KeyedCommandInput,
        attachments: &[PromptPart],
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.admission.with_active_session(
            &input.session_id,
            ResourceCapabilityError::Delivery,
            || {
                let mut blocks = vec![Block::Text {
                    text: input.text.clone(),
                }];
                let mut references = Vec::new();
                if !attachments.is_empty() {
                    let authority = self.attachments.as_ref().ok_or_else(|| {
                        ResourceCapabilityError::Delivery(unsupported("commands/run attachments"))
                    })?;
                    let materialized = authority
                        .materialize_prompt_parts(&input.session_id, attachments)
                        .map_err(|error| {
                            ResourceCapabilityError::Delivery(map_prompt_materialize_error(
                                &input.session_id,
                                error,
                            ))
                        })?;
                    blocks.extend(materialized.blocks);
                    references = materialized.attachments;
                }
                let timestamp = (self.clock)().map_err(ResourceCapabilityError::Clock)?;
                let origin = OriginTuple {
                    principal: input.principal.clone(),
                    client: "tekes-client-resource".to_owned(),
                    target: input.session_id.clone(),
                    // Expansion enters the same durable prompt admission as
                    // typed user input. The dedicated client plus command key
                    // retain command identity without violating prompt authority.
                    op: "session.prompt".to_owned(),
                    key: input.key.clone(),
                };
                self.delivery
                    .prompt(
                        &input.session_id,
                        &timestamp,
                        &origin,
                        &MaterializedPrompt {
                            blocks,
                            attachments: references,
                            files: Vec::new(),
                        },
                        false,
                    )
                    .map_err(ResourceCapabilityError::Delivery)
            },
        )
    }
}

/// Mirrors the `session.prompt` mapping in `endpoint_host`: attachment
/// validation is the closed `attachment-error {reason}`; lifecycle failures
/// keep their session codes; everything else is folded to `internal`.
pub(crate) fn map_prompt_materialize_error(
    session_id: &str,
    error: PromptMaterializeError,
) -> ProductionRouteFailure {
    match error {
        PromptMaterializeError::EmptyContent | PromptMaterializeError::InvalidName => bad_request(),
        PromptMaterializeError::Attachment(reason) => ProductionRouteFailure::new(
            "attachment-error",
            "Image attachment is unavailable",
            json_details(&json!({"reason": reason})),
        ),
        PromptMaterializeError::SessionNotFound(_) => ProductionRouteFailure::new(
            "session-not-found",
            "Session was not found",
            json_details(&json!({"sessionId": session_id})),
        ),
        PromptMaterializeError::Archived(_) => ProductionRouteFailure::new(
            "archived",
            "Session is archived",
            json_details(&json!({"sessionId": session_id})),
        ),
        _ => internal_failure(),
    }
}

impl CommandInputAuthority for EndpointCommandInputAuthority {
    fn submit(
        &self,
        input: &KeyedCommandInput,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.deliver(input, &[])
    }

    fn submit_with_attachments(
        &self,
        input: &KeyedCommandInput,
        attachments: &[PromptPart],
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.deliver(input, attachments)
    }

    fn compact(
        &self,
        input: &KeyedCommandInput,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.admission.with_active_session(
            &input.session_id,
            ResourceCapabilityError::Delivery,
            || {
                let timestamp = (self.clock)().map_err(ResourceCapabilityError::Clock)?;
                let origin = OriginTuple {
                    principal: input.principal.clone(),
                    client: "tekes-client-resource".to_owned(),
                    target: input.session_id.clone(),
                    op: COMMANDS_RUN.to_owned(),
                    key: input.key.clone(),
                };
                self.delivery
                    .compact(&input.session_id, &timestamp, &origin)
                    .map_err(ResourceCapabilityError::Delivery)
            },
        )
    }

    fn select_permission(
        &self,
        input: &KeyedCommandInput,
        mode: engine::PermissionMode,
    ) -> Result<MutationReceipt, ResourceCapabilityError> {
        self.admission.with_active_session(
            &input.session_id,
            ResourceCapabilityError::Delivery,
            || {
                let folder = self
                    .admission
                    .root()
                    .join("threads")
                    .join(&input.session_id);
                engine::write_permission_mode(&folder, mode)
                    .map_err(|_| ResourceCapabilityError::Delivery(internal_failure()))?;
                // The mode is part of the session's inventory summary; tell
                // subscribers now so every Client's composer follows it.
                self.delivery.session_metadata_changed(&input.session_id);
                // The receipt carries no ledger seq: the mode is session
                // metadata, never an event.
                Ok(MutationReceipt {
                    seq: 0,
                    deduplicated: false,
                })
            },
        )
    }
}

#[derive(Debug, Error)]
pub enum ResourceCapabilityError {
    #[error(transparent)]
    Resource(#[from] ResourceError),
    #[error("invalid commands/run request: {0}")]
    InvalidRequest(String),
    #[error("command input delivery failed: {0:?}")]
    Delivery(ProductionRouteFailure),
    #[error("command input clock failed: {0}")]
    Clock(String),
}

fn validate_run_request(request: &CommandRunRequest) -> Result<(), ResourceCapabilityError> {
    if request.session_id.is_empty() {
        return Err(ResourceCapabilityError::InvalidRequest(
            "session_id must be nonempty".to_owned(),
        ));
    }
    if request.key.is_empty()
        || request.key.len() > 128
        || request.key.starts_with("request-")
        || request.key.contains('\0')
    {
        return Err(ResourceCapabilityError::InvalidRequest(
            "key must be 1..128 bytes, contain no NUL, and not use request-".to_owned(),
        ));
    }
    Ok(())
}

fn decode_payload<T: DeserializeOwned>(payload: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(payload.clone()).map_err(|_| bad_request())
}

fn encode_result(value: &impl Serialize) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|_| internal_failure())?)
        .map_err(|_| internal_failure())
}

pub(crate) fn map_resource_failure(error: ResourceCapabilityError) -> ProductionRouteFailure {
    match error {
        ResourceCapabilityError::Resource(ResourceError::InvalidCommand(_)) => {
            ProductionRouteFailure::new("invalid-command", "Command is invalid", empty_details())
        }
        ResourceCapabilityError::Resource(ResourceError::InvalidArguments(_)) => {
            ProductionRouteFailure::new(
                "invalid-arguments",
                "Command arguments are invalid",
                empty_details(),
            )
        }
        ResourceCapabilityError::Resource(ResourceError::CommandNotFound(name)) => {
            ProductionRouteFailure::new(
                "command-not-found",
                "Command was not found",
                json_details(&json!({"name":name})),
            )
        }
        ResourceCapabilityError::Resource(_) | ResourceCapabilityError::Clock(_) => {
            internal_failure()
        }
        ResourceCapabilityError::InvalidRequest(_) => bad_request(),
        ResourceCapabilityError::Delivery(failure) if failure.code == "archived" => {
            ProductionRouteFailure::new("session-archived", failure.message, failure.details)
        }
        ResourceCapabilityError::Delivery(failure) => failure,
    }
}

fn resource_failure_is_exact(failure: &ProductionRouteFailure) -> bool {
    match failure.code.as_str() {
        "invalid-command" => {
            failure.message == "Command is invalid" && failure.details == empty_details()
        }
        "invalid-arguments" => {
            failure.message == "Command arguments are invalid" && failure.details == empty_details()
        }
        "command-not-found" => {
            failure.message == "Command was not found"
                && serde_json::from_slice::<Value>(
                    &failure.details.canonical_bytes().unwrap_or_default(),
                )
                .ok()
                .and_then(|value| value.as_object().cloned())
                .is_some_and(|details| {
                    details.len() == 1 && details.get("name").is_some_and(Value::is_string)
                })
        }
        _ => false,
    }
}

fn base_failure_is_exact(failure: &ProductionRouteFailure) -> bool {
    match failure.code.as_str() {
        "bad-request" => {
            failure.message == "Request payload is invalid" && failure.details == empty_details()
        }
        "internal" => {
            failure.message == "Endpoint operation failed" && failure.details == empty_details()
        }
        _ => false,
    }
}

fn empty_details() -> IJsonValue {
    IJsonValue::parse_str("{}").expect("empty object is I-JSON")
}

fn json_details(value: &Value) -> IJsonValue {
    IJsonValue::parse(&serde_json::to_vec(value).expect("JSON details"))
        .expect("JSON details are I-JSON")
}

fn bad_request() -> ProductionRouteFailure {
    ProductionRouteFailure::new("bad-request", "Request payload is invalid", empty_details())
}

fn internal_failure() -> ProductionRouteFailure {
    ProductionRouteFailure::new("internal", "Endpoint operation failed", empty_details())
}

fn unsupported(operation: &str) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        "unsupported-capability",
        "Capability is unavailable",
        IJsonValue::parse(&serde_json::to_vec(&json!({"operation":operation})).expect("JSON"))
            .expect("operation object is I-JSON"),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use profile::{
        EffectiveInstructions, InstructionKind, InstructionOrigin, InstructionSnapshot,
        InstructionSource,
    };
    use sha2::{Digest, Sha256};

    use super::*;

    #[derive(Default)]
    struct RecordingInput {
        calls: Mutex<Vec<KeyedCommandInput>>,
        compacts: Mutex<Vec<KeyedCommandInput>>,
        permissions: Mutex<Vec<(KeyedCommandInput, engine::PermissionMode)>>,
    }

    impl CommandInputAuthority for RecordingInput {
        fn submit(
            &self,
            input: &KeyedCommandInput,
        ) -> Result<MutationReceipt, ResourceCapabilityError> {
            self.calls.lock().expect("calls").push(input.clone());
            Ok(MutationReceipt {
                seq: 9,
                deduplicated: false,
            })
        }

        fn compact(
            &self,
            input: &KeyedCommandInput,
        ) -> Result<MutationReceipt, ResourceCapabilityError> {
            self.compacts.lock().expect("compacts").push(input.clone());
            Ok(MutationReceipt {
                seq: 11,
                deduplicated: false,
            })
        }

        fn select_permission(
            &self,
            input: &KeyedCommandInput,
            mode: engine::PermissionMode,
        ) -> Result<MutationReceipt, ResourceCapabilityError> {
            self.permissions
                .lock()
                .expect("permissions")
                .push((input.clone(), mode));
            Ok(MutationReceipt {
                seq: 0,
                deduplicated: false,
            })
        }
    }

    fn catalog() -> ResourceCatalog {
        let skill = "---\nname: review\ndescription: Review code\n---\nRead rules.\n";
        let command = "---\ndescription: Review one path\nargument-hint: <path>\n---\nReview $1. Raw: $ARGUMENTS\n";
        let sources = vec![
            source("skills/review/SKILL.md", InstructionKind::Skill, skill),
            source(
                "skills/review/references/rules.md",
                InstructionKind::Skill,
                "rules\n",
            ),
            source("commands/review.md", InstructionKind::Command, command),
        ];
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("review.md".to_owned(), 2);
        ResourceCatalog::from_snapshot(&InstructionSnapshot {
            format: 1,
            sources,
            effective,
        })
        .expect("catalog")
    }

    fn source(path: &str, kind: InstructionKind, content: &str) -> InstructionSource {
        InstructionSource {
            origin: InstructionOrigin::User,
            path: path.to_owned(),
            kind,
            content: content.to_owned(),
            content_sha256: format!("{:x}", Sha256::digest(content.as_bytes())),
        }
    }

    #[test]
    fn session_catalog_controls_command_expansion() {
        let service =
            ClientResourceService::new(ResourceCatalog::default(), RecordingInput::default());
        let request = CommandRunRequest {
            session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),
            name: "review".to_owned(),
            arguments: "src/main.rs".to_owned(),
            key: "session-command".to_owned(),
            attachments: Vec::new(),
        };
        assert!(service.commands_run(&request, "uid:501").is_err());
        service
            .commands_run_with_catalog(&catalog(), &request, "uid:501")
            .expect("project command");
        let calls = service.input.calls.lock().expect("calls");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].text, "Review src/main.rs. Raw: src/main.rs");
    }

    #[test]
    fn capabilities_are_separate_and_run_submits_one_keyed_input() {
        let service = ClientResourceService::new(catalog(), RecordingInput::default());
        assert_eq!(
            ClientResourceService::<RecordingInput>::capabilities(),
            RESOURCE_CAPABILITIES
                .into_iter()
                .map(str::to_owned)
                .collect()
        );
        assert_eq!(service.skills_list().skills[0].name, "review");
        assert_eq!(service.commands_list().commands[0].name, "review");
        let result = service
            .commands_run(
                &CommandRunRequest {
                    session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),
                    name: "review".to_owned(),
                    arguments: "'src/lib.rs'".to_owned(),
                    key: "command-1".to_owned(),
                    attachments: Vec::new(),
                },
                "uid:501",
            )
            .expect("run");
        assert_eq!(result.seq, 9);
        let calls = service.input.calls.lock().expect("calls");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].key, "command-1");
        assert_eq!(calls[0].text, "Review src/lib.rs. Raw: 'src/lib.rs'");
    }

    /// The reserved `permission` verb: the request name is the verb and
    /// `arguments` is the mode. It needs no catalog entry, never becomes
    /// model input, and an unknown mode is an argument error.
    #[test]
    fn permission_verb_selects_the_mode_without_a_catalog_entry_or_an_input() {
        let service = ClientResourceService::new(catalog(), RecordingInput::default());
        assert!(
            service
                .commands_list()
                .commands
                .iter()
                .all(|command| command.name != "permission")
        );
        let run = |name: &str, arguments: &str, key: &str| {
            service.commands_run(
                &CommandRunRequest {
                    session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),
                    name: name.to_owned(),
                    arguments: arguments.to_owned(),
                    key: key.to_owned(),
                    attachments: Vec::new(),
                },
                "uid:501",
            )
        };
        let result = run("permission", "read-only", "perm-1").expect("select");
        assert!(result.accepted);
        assert_eq!(result.command, "permission");
        assert_eq!(result.command_id, "perm-1");
        assert_eq!(result.seq, 0);
        let rejected = run("permission", "yolo", "perm-2").expect_err("unknown mode");
        assert_eq!(map_resource_failure(rejected).code, "invalid-arguments");
        let missing = run("permission", "", "perm-3").expect_err("missing mode");
        assert_eq!(map_resource_failure(missing).code, "invalid-arguments");
        let permissions = service.input.permissions.lock().expect("permissions");
        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].0.key, "perm-1");
        assert_eq!(permissions[0].0.text, "permission read-only");
        assert_eq!(permissions[0].1, engine::PermissionMode::ReadOnly);
        assert!(
            service.input.calls.lock().expect("calls").is_empty(),
            "never model input"
        );
        assert!(service.input.compacts.lock().expect("compacts").is_empty());
    }

    /// A catalog command whose expanded body is exactly `permission <mode>`
    /// is the same control request; a body that merely mentions the verb is
    /// ordinary command text.
    #[test]
    fn permission_verb_in_an_expanded_catalog_body_is_the_same_control_request() {
        let verb =
            "---\ndescription: Lock the session\nargument-hint: <mode>\n---\npermission $1\n";
        let prose = "---\ndescription: Talk about it\n---\npermission read-only please\n";
        let sources = vec![
            source("commands/lock.md", InstructionKind::Command, verb),
            source("commands/lock-talk.md", InstructionKind::Command, prose),
        ];
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("lock.md".to_owned(), 0);
        effective.commands.insert("lock-talk.md".to_owned(), 1);
        let service = ClientResourceService::new(
            ResourceCatalog::from_snapshot(&InstructionSnapshot {
                format: 1,
                sources,
                effective,
            })
            .expect("catalog"),
            RecordingInput::default(),
        );
        let run = |name: &str, arguments: &str, key: &str| {
            service
                .commands_run(
                    &CommandRunRequest {
                        session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),
                        name: name.to_owned(),
                        arguments: arguments.to_owned(),
                        key: key.to_owned(),
                        attachments: Vec::new(),
                    },
                    "uid:501",
                )
                .expect("run")
        };
        assert_eq!(run("lock", "danger-full-access", "lock-1").seq, 0);
        assert_eq!(run("lock-talk", "", "lock-2").seq, 9);
        let permissions = service.input.permissions.lock().expect("permissions");
        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0].1, engine::PermissionMode::DangerFullAccess);
        let calls = service.input.calls.lock().expect("calls");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].text, "permission read-only please");
    }

    /// The production authority writes the durable mode record under the
    /// session folder through the same admission gate as prompts: an
    /// archived session is refused as `session-archived` and nothing is
    /// delivered to the ledger.
    #[test]
    fn production_permission_selection_persists_the_mode_under_the_session_folder() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-000000000072";
        let archived = "018f0000-0000-7000-8000-000000000073";
        std::fs::create_dir_all(root.path().join("threads").join(session)).expect("session");
        std::fs::create_dir_all(root.path().join("archive").join(archived)).expect("archived");
        let delivery = Arc::new(RecordingDelivery(Mutex::new(None), Mutex::new(Vec::new())));
        let authority = EndpointCommandInputAuthority::new(
            Arc::clone(&delivery) as Arc<dyn SessionDeliveryAuthority>,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(SessionInputAdmissionAuthority::new(
                root.path().to_path_buf(),
            )),
        );
        let service = ClientResourceService::new(catalog(), authority);
        let run = |session_id: &str, mode: &str| {
            service.commands_run(
                &CommandRunRequest {
                    session_id: session_id.to_owned(),
                    name: "permission".to_owned(),
                    arguments: mode.to_owned(),
                    key: format!("perm-{mode}"),
                    attachments: Vec::new(),
                },
                "uid:501",
            )
        };
        assert_eq!(
            engine::read_permission_mode(&root.path().join("threads").join(session)).mode,
            engine::PermissionMode::WorkspaceWrite,
            "absent record is workspace-write"
        );
        let result = run(session, "read-only").expect("select");
        assert!(result.accepted && result.seq == 0);
        let record = std::fs::read(
            root.path()
                .join("threads")
                .join(session)
                .join("permission-mode.json"),
        )
        .expect("record");
        assert_eq!(record, b"{\"format\":1,\"mode\":\"read-only\"}\n");
        assert_eq!(
            engine::read_permission_mode(&root.path().join("threads").join(session)).mode,
            engine::PermissionMode::ReadOnly
        );
        let refused = run(archived, "read-only").expect_err("archived");
        assert_eq!(map_resource_failure(refused).code, "session-archived");
        assert!(
            !root
                .path()
                .join("archive")
                .join(archived)
                .join("permission-mode.json")
                .exists()
        );
        let missing =
            run("018f0000-0000-7000-8000-000000000074", "read-only").expect_err("missing");
        assert_eq!(map_resource_failure(missing).code, "session-not-found");
        assert!(
            delivery.0.lock().expect("delivery").is_none(),
            "no prompt is delivered"
        );
        // The mode is inventory metadata: the one accepted selection re-announced its session.
        assert_eq!(
            *delivery.1.lock().expect("metadata"),
            vec![session.to_owned()]
        );
    }

    struct RecordingDelivery(
        Mutex<Option<(String, OriginTuple, MaterializedPrompt, bool)>>,
        Mutex<Vec<String>>,
    );

    impl SessionDeliveryAuthority for RecordingDelivery {
        fn session_metadata_changed(&self, session_id: &str) {
            self.1.lock().expect("metadata").push(session_id.to_owned());
        }

        fn prompt(
            &self,
            session_id: &str,
            _timestamp: &str,
            origin: &OriginTuple,
            prompt: &MaterializedPrompt,
            steer: bool,
        ) -> Result<MutationReceipt, ProductionRouteFailure> {
            *self.0.lock().expect("prompt") =
                Some((session_id.to_owned(), origin.clone(), prompt.clone(), steer));
            Ok(MutationReceipt {
                seq: 21,
                deduplicated: false,
            })
        }

        fn cancel(
            &self,
            _session_id: &str,
            _timestamp: &str,
            _origin: &OriginTuple,
        ) -> Result<MutationReceipt, ProductionRouteFailure> {
            unreachable!("command capability never cancels")
        }

        fn rename(
            &self,
            _session_id: &str,
            _timestamp: &str,
            _origin: &OriginTuple,
            _title: &str,
        ) -> Result<MutationReceipt, ProductionRouteFailure> {
            unreachable!("command capability never renames")
        }
    }

    /// `commands/run` attachments: the expanded command text is the leading
    /// text block and the materialized image and file blocks follow in
    /// request order, through the same attachment authority as
    /// `session.prompt`.
    #[test]
    fn command_attachments_follow_the_expanded_text_in_request_order() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-000000000071";
        let folder = root.path().join("threads").join(session);
        std::fs::create_dir_all(folder.join("assets")).expect("session folder");
        let genesis = json!({"config":{"digest":"cfg-1"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,
            "origin_key":"create","origin_tuple":{"client":"client","key":"create","op":"create","principal":"principal","target":session},
            "resume":"never","seq":1,"thread":session,"ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"workspace"});
        let mut ledger = serde_json_canonicalizer::to_vec(&genesis).expect("genesis");
        ledger.push(b'\n');
        std::fs::write(folder.join("main.jsonl"), ledger).expect("ledger");
        let attachments = AttachmentAuthority::open(root.path());
        let receipt = attachments
            .upload_file(session, "notes.txt", Some("text/plain"), "aGVsbG8gZmlsZQ==")
            .expect("upload");
        let delivery = Arc::new(RecordingDelivery(Mutex::new(None), Mutex::new(Vec::new())));
        let authority = EndpointCommandInputAuthority::new(
            Arc::clone(&delivery) as Arc<dyn SessionDeliveryAuthority>,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(SessionInputAdmissionAuthority::new(
                root.path().to_path_buf(),
            )),
        )
        .with_attachment_authority(attachments);
        let service = ClientResourceService::new(catalog(), authority);
        let png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";
        let request = CommandRunRequest {
            session_id: session.to_owned(),
            name: "review".to_owned(),
            arguments: "src/lib.rs".to_owned(),
            key: "command-attachments-1".to_owned(),
            attachments: vec![
                PromptPart::Image {
                    media_type: endpoint::ImageMediaType::Png,
                    data: png.to_owned(),
                    name: Some("dot.png".to_owned()),
                },
                PromptPart::File {
                    receipt_id: receipt.receipt_id.clone(),
                },
            ],
        };
        let result = service.commands_run(&request, "uid:501").expect("run");
        assert_eq!(result.seq, 21);
        let recorded = delivery.0.lock().expect("prompt");
        let (recorded_session, origin, prompt, steer) = recorded.as_ref().expect("prompt");
        assert_eq!(recorded_session, session);
        assert_eq!(origin.key, "command-attachments-1");
        assert!(!steer);
        assert_eq!(prompt.blocks.len(), 3);
        assert_eq!(
            prompt.blocks[0],
            Block::Text {
                text: "Review src/lib.rs. Raw: src/lib.rs".to_owned()
            }
        );
        let image = serde_json::to_value(&prompt.blocks[1]).expect("image block");
        assert_eq!(image["type"], "image");
        assert_eq!(image["name"], "dot.png");
        let file = serde_json::to_value(&prompt.blocks[2]).expect("file block");
        assert_eq!(file["type"], "file");
        assert_eq!(file["name"], "notes.txt");
        assert_eq!(file["mime"], "text/plain");
        assert_eq!(file["bytes"], 10);
        assert_eq!(
            file["asset"],
            format!(
                "sha256-{}",
                receipt.file.attachment_id.trim_start_matches("sha256:")
            )
        );
        assert_eq!(prompt.attachments.len(), 1);
        assert_eq!(prompt.attachments[0].name.as_deref(), Some("dot.png"));

        // `/compact` is a control request and never carries attachments.
        let compact = "---\ndescription: Force an effective compaction\n---\ncompact\n";
        let sources = vec![source(
            "commands/compact.md",
            InstructionKind::Command,
            compact,
        )];
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("compact.md".to_owned(), 0);
        let compact_catalog = ResourceCatalog::from_snapshot(&InstructionSnapshot {
            format: 1,
            sources,
            effective,
        })
        .expect("catalog");
        let rejected = service
            .commands_run_with_catalog(
                &compact_catalog,
                &CommandRunRequest {
                    name: "compact".to_owned(),
                    arguments: String::new(),
                    key: "command-compact-attachments".to_owned(),
                    ..request.clone()
                },
                "uid:501",
            )
            .expect_err("compact rejects attachments");
        assert_eq!(map_resource_failure(rejected).code, "invalid-arguments");
    }

    /// The legacy `/compact` command: a body that is exactly the reserved verb
    /// is the manual compaction request (same key, never an input), while a
    /// body that merely contains the word is ordinary command text.
    #[test]
    fn compact_verb_maps_to_the_manual_compaction_request_not_an_input() {
        let compact = "---\ndescription: Force an effective compaction\n---\ncompact\n";
        let prose = "---\ndescription: Talk about it\n---\ncompact now please\n";
        let sources = vec![
            source("commands/compact.md", InstructionKind::Command, compact),
            source("commands/compact-now.md", InstructionKind::Command, prose),
        ];
        let mut effective = EffectiveInstructions::default();
        effective.commands.insert("compact.md".to_owned(), 0);
        effective.commands.insert("compact-now.md".to_owned(), 1);
        let snapshot = InstructionSnapshot {
            format: 1,
            sources,
            effective,
        };
        let service = ClientResourceService::new(
            ResourceCatalog::from_snapshot(&snapshot).expect("catalog"),
            RecordingInput::default(),
        );
        let run = |name: &str, key: &str| {
            service
                .commands_run(
                    &CommandRunRequest {
                        session_id: "018f0000-0000-7000-8000-000000000011".to_owned(),
                        name: name.to_owned(),
                        arguments: String::new(),
                        key: key.to_owned(),
                        attachments: Vec::new(),
                    },
                    "uid:501",
                )
                .expect("run")
        };
        let result = run("compact", "compact-1");
        assert_eq!(result.seq, 11);
        assert_eq!(result.command, "compact");
        let prose_result = run("compact-now", "compact-2");
        assert_eq!(prose_result.seq, 9);
        let compacts = service.input.compacts.lock().expect("compacts");
        let calls = service.input.calls.lock().expect("calls");
        assert_eq!(compacts.len(), 1);
        assert_eq!(compacts[0].key, "compact-1");
        assert_eq!(compacts[0].text.trim(), COMPACT_COMMAND_VERB);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].text, "compact now please");
    }
}
