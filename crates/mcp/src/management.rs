use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use plugins::PluginComponentReference;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use store::{FullSync, NamedLock};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

use crate::ProtocolMode;

const FORMAT: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpScope {
    User,
    Project,
    Plugin,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpServerReference {
    pub workspace_id: String,
    pub scope: McpScope,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CredentialValue {
    Literal { literal: String },
    Credential { credential: String },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ClosedCredentialValue {
    Literal(CredentialLiteral),
    Credential(CredentialReferenceValue),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CredentialLiteral {
    literal: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CredentialReferenceValue {
    credential: String,
}

impl<'de> Deserialize<'de> for CredentialValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match ClosedCredentialValue::deserialize(deserializer)? {
            ClosedCredentialValue::Literal(value) => Ok(Self::Literal {
                literal: value.literal,
            }),
            ClosedCredentialValue::Credential(value) => Ok(Self::Credential {
                credential: value.credential,
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum McpTransportConfig {
    Stdio {
        command: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<String>,
        environment: BTreeMap<String, CredentialValue>,
    },
    Http {
        url: String,
        headers: BTreeMap<String, CredentialValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpServerConfig {
    pub reference: McpServerReference,
    pub transport: McpTransportConfig,
    pub enabled: bool,
    pub always_on: bool,
    pub protocol_mode: ProtocolMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_component: Option<PluginComponentReference>,
    #[serde(default)]
    pub project_trusted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpRegistry {
    pub format: u64,
    pub servers: Vec<McpServerConfig>,
}

impl Default for McpRegistry {
    fn default() -> Self {
        Self {
            format: FORMAT,
            servers: Vec::new(),
        }
    }
}

impl McpRegistry {
    pub fn validate(&self) -> Result<(), McpRegistryError> {
        if self.format != FORMAT {
            return Err(McpRegistryError::Invalid("format must be 1".to_owned()));
        }
        let mut seen = BTreeSet::new();
        let mut previous = None;
        for server in &self.servers {
            validate_server(server)?;
            if !seen.insert(server.reference.clone()) {
                return Err(McpRegistryError::Invalid(
                    "duplicate MCP server reference".to_owned(),
                ));
            }
            if previous
                .as_ref()
                .is_some_and(|value| value >= &server.reference)
            {
                return Err(McpRegistryError::Invalid(
                    "MCP servers must be strictly reference-sorted".to_owned(),
                ));
            }
            previous = Some(server.reference.clone());
        }
        Ok(())
    }

    pub fn resolve(&self, workspace: &str) -> Result<Vec<&McpServerConfig>, McpRegistryError> {
        let mut resolved: BTreeMap<&str, &McpServerConfig> = BTreeMap::new();
        for server in &self.servers {
            if server.reference.workspace_id != workspace || !server.enabled {
                continue;
            }
            if server.reference.scope == McpScope::Project && !server.project_trusted {
                continue;
            }
            match resolved.get(server.reference.name.as_str()) {
                Some(existing)
                    if existing.reference.scope == McpScope::Plugin
                        || server.reference.scope == McpScope::Plugin =>
                {
                    return Err(McpRegistryError::Conflict(format!(
                        "plugin MCP server {} collides with another scope",
                        server.reference.name
                    )));
                }
                Some(existing)
                    if scope_rank(existing.reference.scope)
                        >= scope_rank(server.reference.scope) => {}
                _ => {
                    resolved.insert(&server.reference.name, server);
                }
            }
        }
        Ok(resolved.into_values().collect())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McpCredentialReferenceState {
    Pending,
    Bound,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpCredentialReference {
    pub credential_id: String,
    pub server: McpServerReference,
    pub field: String,
    pub state: McpCredentialReferenceState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CredentialRegistry {
    format: u64,
    references: Vec<McpCredentialReference>,
}

impl Default for CredentialRegistry {
    fn default() -> Self {
        Self {
            format: FORMAT,
            references: Vec::new(),
        }
    }
}

impl CredentialRegistry {
    fn from_registry(
        registry: &McpRegistry,
        prior: &Self,
        pending: Option<(&McpServerReference, &str)>,
    ) -> Self {
        let mut references = registry
            .servers
            .iter()
            .flat_map(|server| {
                credential_fields(server)
                    .into_iter()
                    .map(|(field, credential_id)| {
                        let state = if pending.is_some_and(|(reference, credential)| {
                            reference == &server.reference && credential == credential_id
                        }) {
                            McpCredentialReferenceState::Pending
                        } else {
                            prior
                                .references
                                .iter()
                                .find(|item| {
                                    item.server == server.reference
                                        && item.field == field
                                        && item.credential_id == credential_id
                                })
                                .map_or(McpCredentialReferenceState::Bound, |item| {
                                    item.state.clone()
                                })
                        };
                        McpCredentialReference {
                            credential_id,
                            server: server.reference.clone(),
                            field,
                            state,
                        }
                    })
            })
            .collect::<Vec<_>>();
        references.sort();
        Self {
            format: FORMAT,
            references,
        }
    }

    fn validate(&self, registry: &McpRegistry) -> Result<(), McpRegistryError> {
        if self.format != FORMAT {
            return Err(McpRegistryError::Invalid(
                "credential reference format must be 1".to_owned(),
            ));
        }
        let mut previous = None;
        let mut fields = BTreeSet::new();
        for reference in &self.references {
            validate_token(&reference.credential_id, "credential id")?;
            if previous.as_ref().is_some_and(|value| *value >= reference) {
                return Err(McpRegistryError::Invalid(
                    "credential references must be strictly sorted".to_owned(),
                ));
            }
            if !fields.insert((reference.server.clone(), reference.field.clone())) {
                return Err(McpRegistryError::Invalid(
                    "MCP server has duplicate credential field references".to_owned(),
                ));
            }
            let server = registry
                .servers
                .iter()
                .find(|server| server.reference == reference.server)
                .ok_or_else(|| {
                    McpRegistryError::Invalid("credential reference has no MCP server".to_owned())
                })?;
            if credential_fields(server).get(&reference.field) != Some(&reference.credential_id) {
                return Err(McpRegistryError::Invalid(
                    "credential reference differs from MCP field binding".to_owned(),
                ));
            }
            previous = Some(reference);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum McpManagementMutation {
    Save {
        server: McpServerConfig,
        credential_fields: BTreeMap<String, McpCredentialFieldOperation>,
    },
    Remove {
        reference: McpServerReference,
    },
    OauthStart {
        reference: McpServerReference,
        credential_id: String,
        authorization_url: String,
    },
    OauthRemove {
        reference: McpServerReference,
    },
    /// Trusted completion callback after the platform OAuth authority has
    /// durably installed the token. This arm never carries token bytes and is
    /// intentionally not exposed as an MCP management endpoint method.
    OauthBind {
        reference: McpServerReference,
        credential_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum McpCredentialFieldOperation {
    Preserve,
    Replace { credential_id: String },
    Remove,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum McpManagementResult {
    Saved {
        server: McpServerConfig,
    },
    Removed {
        reference: McpServerReference,
    },
    OauthStarted {
        credential_id: String,
        authorization_url: String,
    },
    OauthRemoved {
        reference: McpServerReference,
    },
    OauthBound {
        reference: McpServerReference,
        credential_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpMutationReceipt {
    pub format: u64,
    pub rpc_id: String,
    pub request_digest: String,
    pub registry_digest: String,
    pub credential_digest: String,
    pub result: McpManagementResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpManagementFault {
    IntentDurable,
    RegistryStaged,
    CredentialsStaged,
    RegistryPublished,
    CredentialsPublished,
    BeforeReceipt,
}

#[derive(Clone)]
pub struct McpRegistryStore {
    root: PathBuf,
    fault: Option<McpManagementFault>,
}

impl McpRegistryStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            fault: None,
        }
    }

    #[must_use]
    pub fn injecting_fault(mut self, fault: McpManagementFault) -> Self {
        self.fault = Some(fault);
        self
    }

    pub fn load(&self) -> Result<McpRegistry, McpRegistryError> {
        self.with_lock(|store| store.load_registry_locked())
    }

    pub fn publish(&self, registry: &McpRegistry) -> Result<(), McpRegistryError> {
        registry.validate()?;
        self.with_lock(|store| {
            replace_sync(&store.root.join("mcp-servers.json"), &canonical(registry)?)?;
            sync_directory(&store.root)
        })
    }

    pub fn list(&self, workspace_id: &str) -> Result<Vec<McpServerConfig>, McpRegistryError> {
        Ok(self
            .load()?
            .servers
            .into_iter()
            .filter(|server| server.reference.workspace_id == workspace_id)
            .collect())
    }

    pub fn get(
        &self,
        reference: &McpServerReference,
    ) -> Result<Option<McpServerConfig>, McpRegistryError> {
        Ok(self
            .load()?
            .servers
            .into_iter()
            .find(|server| &server.reference == reference))
    }

    pub fn credential_reference_state(
        &self,
        server: &McpServerReference,
        field: &str,
        credential_id: &str,
    ) -> Result<Option<McpCredentialReferenceState>, McpRegistryError> {
        self.with_lock(|store| {
            Ok(store
                .load_credentials_locked()?
                .references
                .into_iter()
                .find(|reference| {
                    &reference.server == server
                        && reference.field == field
                        && reference.credential_id == credential_id
                })
                .map(|reference| reference.state))
        })
    }

    /// Returns an already-committed exact receipt without applying a missing
    /// mutation. This lets a trusted external precondition be required for the
    /// first commit while preserving rpc-idempotent acknowledgements later.
    pub fn committed_receipt(
        &self,
        rpc_id: &str,
        mutation: &McpManagementMutation,
    ) -> Result<Option<McpMutationReceipt>, McpRegistryError> {
        validate_token(rpc_id, "rpc id")?;
        let request_digest = hex_digest(&canonical(mutation)?);
        self.with_lock(|store| {
            let path = store.receipt_path(rpc_id);
            if !path.exists() {
                return Ok(None);
            }
            let receipt: McpMutationReceipt = decode_canonical(&fs::read(path)?)?;
            if receipt.rpc_id == rpc_id && receipt.request_digest == request_digest {
                return Ok(Some(receipt));
            }
            Err(McpRegistryError::Conflict(
                "rpc id was already committed for a different management request".to_owned(),
            ))
        })
    }

    /// Applies one closed mutation under the endpoint rpc id. The digest is
    /// computed internally from the canonical request; callers cannot supply it.
    pub fn mutate_idempotent(
        &self,
        rpc_id: &str,
        mutation: &McpManagementMutation,
    ) -> Result<McpMutationReceipt, McpRegistryError> {
        validate_token(rpc_id, "rpc id")?;
        let request_digest = hex_digest(&canonical(mutation)?);
        self.with_lock(|store| {
            let receipt_path = store.receipt_path(rpc_id);
            if receipt_path.exists() {
                let receipt: McpMutationReceipt = decode_canonical(&fs::read(receipt_path)?)?;
                validate_token(&receipt.rpc_id, "receipt rpc id")?;
                validate_digest(&receipt.request_digest, "receipt request digest")?;
                validate_digest(&receipt.registry_digest, "receipt registry digest")?;
                validate_digest(&receipt.credential_digest, "receipt credential digest")?;
                if receipt.rpc_id == rpc_id && receipt.request_digest == request_digest {
                    return Ok(receipt);
                }
                return Err(McpRegistryError::Conflict(
                    "rpc id was already committed for a different management request".to_owned(),
                ));
            }
            let (registry, credentials, result) = store.apply_locked(mutation)?;
            let registry_bytes = canonical(&registry)?;
            let credential_bytes = canonical(&credentials)?;
            let receipt = McpMutationReceipt {
                format: FORMAT,
                rpc_id: rpc_id.to_owned(),
                request_digest,
                registry_digest: hex_digest(&registry_bytes),
                credential_digest: hex_digest(&credential_bytes),
                result,
            };
            let operation = DurableOperation {
                format: FORMAT,
                rpc_id: rpc_id.to_owned(),
                request_digest: receipt.request_digest.clone(),
                mutation: mutation.clone(),
                phase: OperationPhase::Intent,
                registry,
                credentials,
                receipt: receipt.clone(),
            };
            store.write_operation(&operation)?;
            store.crash(McpManagementFault::IntentDurable)?;
            store.complete_operation(operation)?;
            Ok(receipt)
        })
    }

    pub fn recover(&self) -> Result<(), McpRegistryError> {
        self.with_lock(|_| Ok(()))
    }

    fn with_lock<T>(
        &self,
        action: impl FnOnce(&Self) -> Result<T, McpRegistryError>,
    ) -> Result<T, McpRegistryError> {
        fs::create_dir_all(&self.root)?;
        let _lock = NamedLock::exclusive(self.root.join(".mcp.lock"))?;
        self.recover_locked()?;
        action(self)
    }

    fn load_registry_locked(&self) -> Result<McpRegistry, McpRegistryError> {
        let path = self.root.join("mcp-servers.json");
        if path.exists() {
            decode_registry_file(&path, &fs::read(&path)?)
        } else {
            Ok(McpRegistry::default())
        }
    }

    fn load_credentials_locked(&self) -> Result<CredentialRegistry, McpRegistryError> {
        let path = self.root.join("mcp-credential-references.json");
        if path.exists() {
            let credentials: CredentialRegistry = decode_canonical(&fs::read(path)?)?;
            credentials.validate(&self.load_registry_locked()?)?;
            Ok(credentials)
        } else {
            Ok(CredentialRegistry::default())
        }
    }

    fn apply_locked(
        &self,
        mutation: &McpManagementMutation,
    ) -> Result<(McpRegistry, CredentialRegistry, McpManagementResult), McpRegistryError> {
        let mut registry = self.load_registry_locked()?;
        let mut credentials = self.load_credentials_locked()?;
        let mut pending = None;
        let result = match mutation {
            McpManagementMutation::Save {
                server,
                credential_fields,
            } => {
                let existing = registry
                    .servers
                    .iter()
                    .find(|item| item.reference == server.reference);
                let server =
                    apply_credential_field_operations(server, existing, credential_fields)?;
                validate_server(&server)?;
                if server.reference.scope == McpScope::Plugin {
                    return Err(McpRegistryError::ReadOnlyPlugin);
                }
                registry
                    .servers
                    .retain(|item| item.reference != server.reference);
                registry.servers.push(server.clone());
                registry
                    .servers
                    .sort_by(|left, right| left.reference.cmp(&right.reference));
                McpManagementResult::Saved { server }
            }
            McpManagementMutation::Remove { reference } => {
                if reference.scope == McpScope::Plugin {
                    return Err(McpRegistryError::ReadOnlyPlugin);
                }
                if !registry
                    .servers
                    .iter()
                    .any(|item| &item.reference == reference)
                {
                    return Err(McpRegistryError::NotFound);
                }
                registry.servers.retain(|item| &item.reference != reference);
                McpManagementResult::Removed {
                    reference: reference.clone(),
                }
            }
            McpManagementMutation::OauthStart {
                reference,
                credential_id,
                authorization_url,
            } => {
                validate_token(credential_id, "credential id")?;
                let parsed = url::Url::parse(authorization_url)
                    .map_err(|error| McpRegistryError::Invalid(error.to_string()))?;
                if parsed.scheme() != "https"
                    || parsed.username() != ""
                    || parsed.password().is_some()
                    || parsed.query_pairs().any(|(name, _)| {
                        matches!(
                            name.as_ref().to_ascii_lowercase().as_str(),
                            "access_token" | "refresh_token" | "token" | "code" | "client_secret"
                        )
                    })
                {
                    return Err(McpRegistryError::Invalid(
                        "OAuth authorization URL must be credential-free HTTPS".to_owned(),
                    ));
                }
                if reference.scope == McpScope::Plugin {
                    return Err(McpRegistryError::ReadOnlyPlugin);
                }
                let server = registry
                    .servers
                    .iter_mut()
                    .find(|item| &item.reference == reference)
                    .ok_or(McpRegistryError::NotFound)?;
                let McpTransportConfig::Http { oauth, .. } = &mut server.transport else {
                    return Err(McpRegistryError::Invalid(
                        "OAuth requires an HTTP MCP server".to_owned(),
                    ));
                };
                *oauth = Some(credential_id.clone());
                pending = Some((reference, credential_id.as_str()));
                McpManagementResult::OauthStarted {
                    credential_id: credential_id.clone(),
                    authorization_url: authorization_url.clone(),
                }
            }
            McpManagementMutation::OauthRemove { reference } => {
                if reference.scope == McpScope::Plugin {
                    return Err(McpRegistryError::ReadOnlyPlugin);
                }
                let server = registry
                    .servers
                    .iter_mut()
                    .find(|item| &item.reference == reference)
                    .ok_or(McpRegistryError::NotFound)?;
                let McpTransportConfig::Http { oauth, .. } = &mut server.transport else {
                    return Err(McpRegistryError::Invalid(
                        "OAuth requires an HTTP MCP server".to_owned(),
                    ));
                };
                *oauth = None;
                McpManagementResult::OauthRemoved {
                    reference: reference.clone(),
                }
            }
            McpManagementMutation::OauthBind {
                reference,
                credential_id,
            } => {
                validate_token(credential_id, "credential id")?;
                if reference.scope == McpScope::Plugin {
                    return Err(McpRegistryError::ReadOnlyPlugin);
                }
                let server = registry
                    .servers
                    .iter()
                    .find(|item| &item.reference == reference)
                    .ok_or(McpRegistryError::NotFound)?;
                let McpTransportConfig::Http { oauth, .. } = &server.transport else {
                    return Err(McpRegistryError::Invalid(
                        "OAuth requires an HTTP MCP server".to_owned(),
                    ));
                };
                if oauth.as_deref() != Some(credential_id) {
                    return Err(McpRegistryError::Invalid(
                        "OAuth completion does not own the pending credential reference".to_owned(),
                    ));
                }
                let Some(pending_reference) = credentials.references.iter_mut().find(|item| {
                    item.server == *reference
                        && item.field == "oauth"
                        && item.credential_id == *credential_id
                }) else {
                    return Err(McpRegistryError::Invalid(
                        "OAuth completion has no pending credential reference".to_owned(),
                    ));
                };
                if pending_reference.state != McpCredentialReferenceState::Pending {
                    return Err(McpRegistryError::Invalid(
                        "OAuth credential reference is not pending".to_owned(),
                    ));
                }
                pending_reference.state = McpCredentialReferenceState::Bound;
                McpManagementResult::OauthBound {
                    reference: reference.clone(),
                    credential_id: credential_id.clone(),
                }
            }
        };
        registry.validate()?;
        if !matches!(mutation, McpManagementMutation::OauthBind { .. }) {
            credentials = CredentialRegistry::from_registry(&registry, &credentials, pending);
        }
        credentials.validate(&registry)?;
        Ok((registry, credentials, result))
    }

    fn recover_locked(&self) -> Result<(), McpRegistryError> {
        let path = self.root.join("mcp-operation.json");
        if !path.exists() {
            return Ok(());
        }
        let operation: DurableOperation = decode_canonical(&fs::read(path)?)?;
        operation.validate()?;
        self.complete_operation(operation)
    }

    fn complete_operation(&self, mut operation: DurableOperation) -> Result<(), McpRegistryError> {
        let registry_bytes = canonical(&operation.registry)?;
        let credential_bytes = canonical(&operation.credentials)?;
        stage_sync(&self.root.join("mcp-servers.json.staged"), &registry_bytes)?;
        self.crash(McpManagementFault::RegistryStaged)?;
        stage_sync(
            &self.root.join("mcp-credential-references.json.staged"),
            &credential_bytes,
        )?;
        self.crash(McpManagementFault::CredentialsStaged)?;
        operation.phase = OperationPhase::Staged;
        self.write_operation(&operation)?;
        publish_staged(&self.root, "mcp-servers.json", &registry_bytes)?;
        self.crash(McpManagementFault::RegistryPublished)?;
        publish_staged(
            &self.root,
            "mcp-credential-references.json",
            &credential_bytes,
        )?;
        self.crash(McpManagementFault::CredentialsPublished)?;
        operation.phase = OperationPhase::Published;
        self.write_operation(&operation)?;
        self.crash(McpManagementFault::BeforeReceipt)?;
        let receipts = self.root.join("receipts");
        fs::create_dir_all(&receipts)?;
        replace_sync(
            &self.receipt_path(&operation.rpc_id),
            &canonical(&operation.receipt)?,
        )?;
        sync_directory(&receipts)?;
        fs::remove_file(self.root.join("mcp-operation.json"))?;
        sync_directory(&self.root)
    }

    fn write_operation(&self, operation: &DurableOperation) -> Result<(), McpRegistryError> {
        replace_sync(
            &self.root.join("mcp-operation.json"),
            &canonical(operation)?,
        )?;
        sync_directory(&self.root)
    }

    fn receipt_path(&self, rpc_id: &str) -> PathBuf {
        self.root.join("receipts").join(format!("{rpc_id}.json"))
    }

    fn crash(&self, point: McpManagementFault) -> Result<(), McpRegistryError> {
        if self.fault == Some(point) {
            Err(McpRegistryError::InjectedFault(point))
        } else {
            Ok(())
        }
    }
}

fn apply_credential_field_operations(
    proposed: &McpServerConfig,
    existing: Option<&McpServerConfig>,
    operations: &BTreeMap<String, McpCredentialFieldOperation>,
) -> Result<McpServerConfig, McpRegistryError> {
    let mut result = proposed.clone();
    let proposed_credentials = credential_fields(&result);
    if !proposed_credentials.is_empty() {
        return Err(McpRegistryError::Invalid(
            "save transport embeds credential ids; use credentialFields replace operations"
                .to_owned(),
        ));
    }
    let existing_credentials = existing.map(credential_fields).unwrap_or_default();
    let mut keys = existing_credentials
        .keys()
        .chain(operations.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for key in std::mem::take(&mut keys) {
        let value = match operations.get(&key) {
            Some(McpCredentialFieldOperation::Preserve) | None => {
                existing_credentials.get(&key).cloned()
            }
            Some(McpCredentialFieldOperation::Replace { credential_id }) => {
                validate_token(credential_id, "credential id")?;
                Some(credential_id.clone())
            }
            Some(McpCredentialFieldOperation::Remove) => None,
        };
        set_credential_field(&mut result, &key, value)?;
    }
    Ok(result)
}

fn credential_fields(server: &McpServerConfig) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    match &server.transport {
        McpTransportConfig::Stdio { environment, .. } => {
            for (name, value) in environment {
                if let CredentialValue::Credential { credential } = value {
                    fields.insert(format!("environment.{name}"), credential.clone());
                }
            }
        }
        McpTransportConfig::Http { headers, oauth, .. } => {
            for (name, value) in headers {
                if let CredentialValue::Credential { credential } = value {
                    fields.insert(format!("headers.{name}"), credential.clone());
                }
            }
            if let Some(credential) = oauth {
                fields.insert("oauth".to_owned(), credential.clone());
            }
        }
    }
    fields
}

fn set_credential_field(
    server: &mut McpServerConfig,
    key: &str,
    value: Option<String>,
) -> Result<(), McpRegistryError> {
    match (&mut server.transport, key.split_once('.')) {
        (McpTransportConfig::Stdio { environment, .. }, Some(("environment", name)))
            if !name.is_empty() =>
        {
            match value {
                Some(credential) => {
                    environment.insert(name.to_owned(), CredentialValue::Credential { credential });
                }
                None => {
                    environment.remove(name);
                }
            }
        }
        (McpTransportConfig::Http { headers, .. }, Some(("headers", name))) if !name.is_empty() => {
            match value {
                Some(credential) => {
                    headers.insert(name.to_owned(), CredentialValue::Credential { credential });
                }
                None => {
                    headers.remove(name);
                }
            }
        }
        (McpTransportConfig::Http { oauth, .. }, None) if key == "oauth" => {
            *oauth = value;
        }
        _ => {
            return Err(McpRegistryError::Invalid(format!(
                "credential field {key} does not match transport"
            )));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum OperationPhase {
    Intent,
    Staged,
    Published,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableOperation {
    format: u64,
    rpc_id: String,
    request_digest: String,
    mutation: McpManagementMutation,
    phase: OperationPhase,
    registry: McpRegistry,
    credentials: CredentialRegistry,
    receipt: McpMutationReceipt,
}

impl DurableOperation {
    fn validate(&self) -> Result<(), McpRegistryError> {
        self.registry.validate()?;
        self.credentials.validate(&self.registry)?;
        validate_token(&self.rpc_id, "rpc id")?;
        let request_digest = hex_digest(&canonical(&self.mutation)?);
        let registry_digest = hex_digest(&canonical(&self.registry)?);
        let credential_digest = hex_digest(&canonical(&self.credentials)?);
        if self.format != FORMAT
            || self.receipt.format != FORMAT
            || self.rpc_id != self.receipt.rpc_id
            || self.request_digest != request_digest
            || self.receipt.request_digest != request_digest
            || self.receipt.registry_digest != registry_digest
            || self.receipt.credential_digest != credential_digest
        {
            return Err(McpRegistryError::Invalid(
                "durable MCP operation and receipt digests differ".to_owned(),
            ));
        }
        Ok(())
    }
}

fn validate_server(server: &McpServerConfig) -> Result<(), McpRegistryError> {
    let reference = &server.reference;
    if reference.workspace_id.is_empty()
        || reference.name.is_empty()
        || reference.name.len() > 128
        || reference.name != reference.name.nfc().collect::<String>()
    {
        return Err(McpRegistryError::Invalid(
            "invalid MCP workspace/name identity".to_owned(),
        ));
    }
    match reference.scope {
        McpScope::Plugin => {
            let owner = server
                .owner
                .as_deref()
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    McpRegistryError::Invalid("plugin MCP server requires owner".to_owned())
                })?;
            let component = server.plugin_component.as_ref().ok_or_else(|| {
                McpRegistryError::Invalid(
                    "plugin MCP server requires component reference".to_owned(),
                )
            })?;
            if component.plugin_id != owner {
                return Err(McpRegistryError::Invalid(
                    "plugin owner and component reference differ".to_owned(),
                ));
            }
            if !matches!(server.transport, McpTransportConfig::Stdio { .. }) {
                return Err(McpRegistryError::Invalid(
                    "plugin MCP server must use stdio".to_owned(),
                ));
            }
        }
        McpScope::User | McpScope::Project
            if server.owner.is_some() || server.plugin_component.is_some() =>
        {
            return Err(McpRegistryError::Invalid(
                "standalone MCP server cannot claim plugin ownership".to_owned(),
            ));
        }
        McpScope::User | McpScope::Project => {}
    }
    match &server.transport {
        McpTransportConfig::Stdio { command, cwd, .. } => {
            if reference.scope != McpScope::Plugin
                && (command.first().is_none_or(String::is_empty)
                    || cwd
                        .as_ref()
                        .is_some_and(|path| !Path::new(path).is_absolute()))
            {
                return Err(McpRegistryError::Invalid(
                    "invalid stdio MCP transport".to_owned(),
                ));
            }
        }
        McpTransportConfig::Http { url, .. } => {
            let url = url::Url::parse(url)
                .map_err(|error| McpRegistryError::Invalid(error.to_string()))?;
            if url.scheme() != "https" {
                return Err(McpRegistryError::Invalid(
                    "production HTTP MCP URL must use https".to_owned(),
                ));
            }
        }
    }
    Ok(())
}

fn scope_rank(scope: McpScope) -> u8 {
    match scope {
        McpScope::User => 0,
        McpScope::Project => 1,
        McpScope::Plugin => 2,
    }
}

fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, McpRegistryError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| McpRegistryError::Invalid(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Reads the on-disk registry. The file is also written by hand and by tools
/// outside the Kernel, which commonly drop the final newline of an otherwise
/// canonical record or append a server that is already registered under the
/// same `{workspace_id, scope, name}`; both are repaired before the strict
/// validation so the whole MCP catalog does not vanish over a missing byte or
/// a repeated entry. A repeated reference keeps its last occurrence (the
/// newest write) and is logged as a warning; the next mutation publishes the
/// repaired, reference-sorted registry. Every other deviation still fails,
/// and the failure names the file.
fn decode_registry_file(path: &Path, bytes: &[u8]) -> Result<McpRegistry, McpRegistryError> {
    let mut repaired;
    let bytes = if bytes.is_empty() || bytes.ends_with(b"\n") {
        bytes
    } else {
        repaired = bytes.to_vec();
        repaired.push(b'\n');
        &repaired
    };
    let name_file = |error| match error {
        McpRegistryError::Invalid(message) => {
            McpRegistryError::Invalid(format!("{}: {message}", path.display()))
        }
        other => other,
    };
    let mut registry: McpRegistry = decode_canonical(bytes).map_err(name_file)?;
    let duplicates = dedupe_registry_last_wins(&mut registry);
    for reference in &duplicates {
        eprintln!(
            "mcp-registry-duplicate-reference: file={} workspace={} scope={:?} name={} resolution=last-entry-wins",
            path.display(),
            reference.workspace_id,
            reference.scope,
            reference.name
        );
    }
    registry.validate().map_err(name_file)?;
    Ok(registry)
}

/// Keeps the last entry for every repeated reference, restores the sorted
/// order, and returns the references that were repeated (in registry order).
/// A registry without repeats or disorder is returned untouched.
fn dedupe_registry_last_wins(registry: &mut McpRegistry) -> Vec<McpServerReference> {
    let mut last_index: BTreeMap<McpServerReference, usize> = BTreeMap::new();
    let mut duplicates = Vec::new();
    for (index, server) in registry.servers.iter().enumerate() {
        if last_index.insert(server.reference.clone(), index).is_some()
            && !duplicates.contains(&server.reference)
        {
            duplicates.push(server.reference.clone());
        }
    }
    let sorted = registry
        .servers
        .windows(2)
        .all(|pair| pair[0].reference < pair[1].reference);
    if duplicates.is_empty() && sorted {
        return duplicates;
    }
    let servers = std::mem::take(&mut registry.servers);
    registry.servers = servers
        .into_iter()
        .enumerate()
        .filter(|(index, server)| last_index[&server.reference] == *index)
        .map(|(_, server)| server)
        .collect();
    registry
        .servers
        .sort_by(|left, right| left.reference.cmp(&right.reference));
    duplicates
}

fn decode_canonical<T>(bytes: &[u8]) -> Result<T, McpRegistryError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    if !bytes.ends_with(b"\n") || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n') {
        return Err(McpRegistryError::Invalid(
            "record must be one canonical JSON line".to_owned(),
        ));
    }
    let value: T = serde_json::from_slice(&bytes[..bytes.len() - 1])
        .map_err(|error| McpRegistryError::Invalid(error.to_string()))?;
    if canonical(&value)? != bytes {
        return Err(McpRegistryError::Invalid(
            "record bytes are not canonical".to_owned(),
        ));
    }
    Ok(value)
}

fn validate_token(value: &str, label: &str) -> Result<(), McpRegistryError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(McpRegistryError::Invalid(format!("invalid {label}")));
    }
    Ok(())
}

fn validate_digest(value: &str, label: &str) -> Result<(), McpRegistryError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(McpRegistryError::Invalid(format!("invalid {label}")));
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn stage_sync(path: &Path, bytes: &[u8]) -> Result<(), McpRegistryError> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    write_sync(path, bytes)
}

fn publish_staged(root: &Path, destination: &str, expected: &[u8]) -> Result<(), McpRegistryError> {
    let staged = root.join(format!("{destination}.staged"));
    let destination = root.join(destination);
    if staged.exists() {
        fs::rename(&staged, &destination)?;
        sync_directory(root)?;
    }
    if !destination.exists() || fs::read(&destination)? != expected {
        return Err(McpRegistryError::Invalid(
            "published management bytes differ from durable operation".to_owned(),
        ));
    }
    Ok(())
}

fn replace_sync(path: &Path, bytes: &[u8]) -> Result<(), McpRegistryError> {
    let temporary = path.with_extension("replacement");
    if temporary.exists() {
        fs::remove_file(&temporary)?;
    }
    write_sync(&temporary, bytes)?;
    fs::rename(temporary, path)?;
    if let Some(parent) = path.parent() {
        sync_directory(parent)?;
    }
    Ok(())
}

fn write_sync(path: &Path, bytes: &[u8]) -> Result<(), McpRegistryError> {
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(bytes)?;
    FullSync::full_sync(&file)?;
    Ok(())
}

fn sync_directory(path: &Path) -> Result<(), McpRegistryError> {
    FullSync::full_sync(&File::open(path)?)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum McpRegistryError {
    #[error("invalid MCP registry: {0}")]
    Invalid(String),
    #[error("MCP registry I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("MCP registry lock failed: {0}")]
    Store(#[from] store::StoreError),
    #[error("MCP registry idempotency conflict: {0}")]
    Conflict(String),
    #[error("plugin-owned MCP rows are read-only")]
    ReadOnlyPlugin,
    #[error("MCP server was not found")]
    NotFound,
    #[error("injected MCP management fault: {0:?}")]
    InjectedFault(McpManagementFault),
}
