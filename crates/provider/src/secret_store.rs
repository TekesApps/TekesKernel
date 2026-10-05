use std::collections::BTreeMap;
use std::sync::Mutex;

use profile::ConfigSnapshot;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zeroize::Zeroize;

use crate::{CredentialScope, RevokedCredentialScope, endpoint_origin};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RecordState {
    Active,
    Revoked,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    format: u64,
    generation: u64,
    state: RecordState,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<String>,
}

#[derive(Eq, PartialEq)]
pub enum SecretRecord {
    Active { generation: u64, material: String },
    Revoked { generation: u64 },
}

impl Clone for SecretRecord {
    fn clone(&self) -> Self {
        match self {
            Self::Active {
                generation,
                material,
            } => Self::Active {
                generation: *generation,
                material: material.clone(),
            },
            Self::Revoked { generation } => Self::Revoked {
                generation: *generation,
            },
        }
    }
}

impl std::fmt::Debug for SecretRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active { generation, .. } => formatter
                .debug_struct("Active")
                .field("generation", generation)
                .field("material", &"<redacted>")
                .finish(),
            Self::Revoked { generation } => formatter
                .debug_struct("Revoked")
                .field("generation", generation)
                .finish(),
        }
    }
}

impl Drop for SecretRecord {
    fn drop(&mut self) {
        if let Self::Active { material, .. } = self {
            material.zeroize();
        }
    }
}

impl SecretRecord {
    pub fn decode(bytes: &[u8]) -> Result<Self, SecretStoreError> {
        let wire: WireRecord =
            serde_json::from_slice(bytes).map_err(|_| SecretStoreError::InvalidRecord)?;
        let canonical =
            serde_json_canonicalizer::to_vec(&wire).map_err(|_| SecretStoreError::InvalidRecord)?;
        if canonical != bytes
            || wire.format != 1
            || wire.generation == 0
            || wire.generation > MAX_SAFE_INTEGER
        {
            return Err(SecretStoreError::InvalidRecord);
        }
        match (wire.state, wire.material) {
            (RecordState::Active, Some(material)) if valid_material(&material) => {
                Ok(Self::Active {
                    generation: wire.generation,
                    material,
                })
            }
            (RecordState::Revoked, None) => Ok(Self::Revoked {
                generation: wire.generation,
            }),
            _ => Err(SecretStoreError::InvalidRecord),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, SecretStoreError> {
        let wire = match self {
            Self::Active {
                generation,
                material,
            } if *generation > 0 && *generation <= MAX_SAFE_INTEGER && valid_material(material) => {
                WireRecord {
                    format: 1,
                    generation: *generation,
                    state: RecordState::Active,
                    material: Some(material.clone()),
                }
            }
            Self::Revoked { generation } if *generation > 0 && *generation <= MAX_SAFE_INTEGER => {
                WireRecord {
                    format: 1,
                    generation: *generation,
                    state: RecordState::Revoked,
                    material: None,
                }
            }
            _ => return Err(SecretStoreError::InvalidRecord),
        };
        serde_json_canonicalizer::to_vec(&wire).map_err(|_| SecretStoreError::InvalidRecord)
    }

    #[must_use]
    pub fn generation(&self) -> u64 {
        match self {
            Self::Active { generation, .. } | Self::Revoked { generation } => *generation,
        }
    }
}

fn valid_material(material: &str) -> bool {
    !material.is_empty()
        && !material
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecretResolution {
    Active(SecretRecord),
    Revoked { generation: u64 },
    NotFound,
}

pub trait SecretStore: Send + Sync {
    fn resolve(&self, credential_id: &str) -> Result<SecretResolution, SecretStoreError>;
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SecretStoreError {
    #[error("credential id is invalid")]
    InvalidId,
    #[error("secret record is invalid")]
    InvalidRecord,
    #[error("secret generation did not advance")]
    GenerationRollback,
    #[error("secret generation conflicts with durable authority")]
    GenerationConflict,
    #[error("secret store is unavailable")]
    Unavailable,
    #[error("secret store backend returned OSStatus {0}")]
    BackendStatus(i32),
}

#[derive(Default)]
pub struct MemorySecretStore {
    records: Mutex<BTreeMap<String, SecretRecord>>,
}

impl MemorySecretStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(
        &self,
        credential_id: &str,
        record: SecretRecord,
    ) -> Result<(), SecretStoreError> {
        validate_credential_id(credential_id)?;
        // Encoding validates the material and generation without exposing it.
        drop(record.encode()?);
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if records
            .get(credential_id)
            .is_some_and(|prior| record.generation() <= prior.generation())
        {
            return Err(SecretStoreError::GenerationRollback);
        }
        records.insert(credential_id.to_owned(), record);
        Ok(())
    }

    pub fn remove(&self, credential_id: &str) {
        self.records
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(credential_id);
    }
}

impl SecretStore for MemorySecretStore {
    fn resolve(&self, credential_id: &str) -> Result<SecretResolution, SecretStoreError> {
        validate_credential_id(credential_id)?;
        Ok(
            match self
                .records
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(credential_id)
                .cloned()
            {
                Some(record @ SecretRecord::Active { .. }) => SecretResolution::Active(record),
                Some(SecretRecord::Revoked { generation }) => {
                    SecretResolution::Revoked { generation }
                }
                None => SecretResolution::NotFound,
            },
        )
    }
}

/// secret-store §OAuth secret mutation: the narrow, versioned write
/// authority the Kernel holds over records it minted itself. `publish_record`
/// is one atomic item replacement whose generation is strictly greater than
/// the current one; `expected_prior` pins the current generation (`None`: the
/// item must not exist) so concurrent writers cannot interleave.
pub trait SecretMutationAuthority: Send + Sync {
    fn publish_record(
        &self,
        credential_id: &str,
        expected_prior: Option<u64>,
        record: SecretRecord,
    ) -> Result<(), SecretStoreError>;
}

impl SecretMutationAuthority for MemorySecretStore {
    fn publish_record(
        &self,
        credential_id: &str,
        expected_prior: Option<u64>,
        record: SecretRecord,
    ) -> Result<(), SecretStoreError> {
        validate_credential_id(credential_id)?;
        drop(record.encode()?);
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = records.get(credential_id).map(SecretRecord::generation);
        if current != expected_prior {
            return Err(SecretStoreError::GenerationConflict);
        }
        if current.is_some_and(|prior| record.generation() <= prior) {
            return Err(SecretStoreError::GenerationRollback);
        }
        records.insert(credential_id.to_owned(), record);
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialAvailability {
    Active { generation: u64 },
    Revoked { generation: u64 },
    NotFound,
    Unavailable,
}

#[derive(Clone, Debug, Default)]
pub struct ResolvedCredentialBindings {
    pub active: Vec<CredentialScope>,
    pub revoked: Vec<RevokedCredentialScope>,
    pub availability: BTreeMap<String, CredentialAvailability>,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ResolveBindingsError {
    #[error("configured provider endpoint is invalid")]
    InvalidEndpoint,
}

pub fn resolve_config_credentials(
    config: &ConfigSnapshot,
    store: &dyn SecretStore,
) -> Result<ResolvedCredentialBindings, ResolveBindingsError> {
    let mut resolutions = BTreeMap::new();
    for provider in &config.providers.providers {
        if let Some(credential_id) = &provider.credential_key {
            resolutions
                .entry(credential_id.clone())
                .or_insert_with(|| store.resolve(credential_id));
        }
    }
    if let Some(search) = &config.providers.web_search {
        resolutions
            .entry(search.credential_key.clone())
            .or_insert_with(|| store.resolve(&search.credential_key));
    }

    let mut result = ResolvedCredentialBindings::default();
    for (credential_id, resolution) in &resolutions {
        let availability = match resolution {
            Ok(SecretResolution::Active(SecretRecord::Active { generation, .. })) => {
                CredentialAvailability::Active {
                    generation: *generation,
                }
            }
            Ok(SecretResolution::Active(SecretRecord::Revoked { generation }))
            | Ok(SecretResolution::Revoked { generation }) => CredentialAvailability::Revoked {
                generation: *generation,
            },
            Ok(SecretResolution::NotFound) => CredentialAvailability::NotFound,
            Err(_) => CredentialAvailability::Unavailable,
        };
        result
            .availability
            .insert(credential_id.clone(), availability);
    }

    for configured in &config.providers.providers {
        let Some(credential_id) = &configured.credential_key else {
            continue;
        };
        let origin = endpoint_origin(&configured.endpoint)
            .map_err(|_| ResolveBindingsError::InvalidEndpoint)?;
        push_resolved_scope(
            &mut result,
            resolutions.get(credential_id),
            credential_id,
            &configured.adapter,
            &origin,
            "provider",
        );
    }
    if let Some(search) = &config.providers.web_search {
        let origin =
            endpoint_origin(&search.endpoint).map_err(|_| ResolveBindingsError::InvalidEndpoint)?;
        push_resolved_scope(
            &mut result,
            resolutions.get(&search.credential_key),
            &search.credential_key,
            &search.adapter,
            &origin,
            "web_search",
        );
    }
    result.active.sort_by(scope_order);
    result.active.dedup();
    result.revoked.sort_by(revoked_scope_order);
    result.revoked.dedup();
    Ok(result)
}

fn push_resolved_scope(
    result: &mut ResolvedCredentialBindings,
    resolution: Option<&Result<SecretResolution, SecretStoreError>>,
    credential_id: &str,
    adapter: &str,
    origin: &str,
    purpose: &str,
) {
    match resolution {
        Some(Ok(SecretResolution::Active(SecretRecord::Active {
            generation,
            material,
        }))) => result.active.push(CredentialScope {
            credential_id: credential_id.to_owned(),
            adapter: adapter.to_owned(),
            endpoint_origin: origin.to_owned(),
            purpose: purpose.to_owned(),
            generation: generation.to_string(),
            material: material.clone(),
        }),
        Some(Ok(
            SecretResolution::Active(SecretRecord::Revoked { generation })
            | SecretResolution::Revoked { generation },
        )) => result.revoked.push(RevokedCredentialScope {
            credential_id: credential_id.to_owned(),
            adapter: adapter.to_owned(),
            endpoint_origin: origin.to_owned(),
            purpose: purpose.to_owned(),
            generation: generation.to_string(),
        }),
        Some(Ok(SecretResolution::NotFound) | Err(_)) | None => {}
    }
}

fn scope_order(left: &CredentialScope, right: &CredentialScope) -> std::cmp::Ordering {
    (
        &left.credential_id,
        &left.adapter,
        &left.endpoint_origin,
        &left.purpose,
    )
        .cmp(&(
            &right.credential_id,
            &right.adapter,
            &right.endpoint_origin,
            &right.purpose,
        ))
}

fn revoked_scope_order(
    left: &RevokedCredentialScope,
    right: &RevokedCredentialScope,
) -> std::cmp::Ordering {
    (
        &left.credential_id,
        &left.adapter,
        &left.endpoint_origin,
        &left.purpose,
    )
        .cmp(&(
            &right.credential_id,
            &right.adapter,
            &right.endpoint_origin,
            &right.purpose,
        ))
}

pub(crate) fn validate_credential_id(value: &str) -> Result<(), SecretStoreError> {
    if value.is_empty()
        || value.len() > 256
        || value
            .chars()
            .any(|character| character.is_control() || character == '\0')
    {
        Err(SecretStoreError::InvalidId)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn record_bytes_are_closed_canonical_and_redacted() {
        let active = SecretRecord::Active {
            generation: 7,
            material: "fixture-secret-never-log".to_owned(),
        };
        let bytes = active.encode().expect("active bytes");
        assert_eq!(
            bytes,
            br#"{"format":1,"generation":7,"material":"fixture-secret-never-log","state":"active"}"#
        );
        assert_eq!(SecretRecord::decode(&bytes).expect("decode"), active);
        let fixture = include_bytes!("../../../fixtures/secret-store/active.canonical.json");
        assert_eq!(fixture.strip_suffix(b"\n").expect("fixture LF"), bytes);
        assert!(!format!("{active:?}").contains("fixture-secret"));
        assert!(
            SecretRecord::decode(
                br#"{"format":1,"generation":7,"material":"x","state":"active","unknown":true}"#
            )
            .is_err()
        );
        assert!(
            SecretRecord::decode(br#"{"state":"active","material":"x","generation":7,"format":1}"#)
                .is_err()
        );
    }

    #[test]
    fn memory_store_enforces_monotonic_rotation_and_revocation() {
        let store = MemorySecretStore::new();
        store
            .publish(
                "provider-main",
                SecretRecord::Active {
                    generation: 1,
                    material: "first".to_owned(),
                },
            )
            .expect("publish");
        assert!(matches!(
            store.resolve("provider-main").expect("resolve"),
            SecretResolution::Active(SecretRecord::Active { generation: 1, .. })
        ));
        assert_eq!(
            store.publish("provider-main", SecretRecord::Revoked { generation: 1 }),
            Err(SecretStoreError::GenerationRollback)
        );
        store
            .publish("provider-main", SecretRecord::Revoked { generation: 2 })
            .expect("revoke");
        assert_eq!(
            store.resolve("provider-main").expect("resolve revoked"),
            SecretResolution::Revoked { generation: 2 }
        );
    }

    fn config_with_shared_key() -> ConfigSnapshot {
        serde_json::from_value(json!({
            "format": 1,
            "workspace": {
                "format": 1,
                "revision": 1,
                "id": "workspace",
                "name": "Workspace",
                "cwd": ["/tmp"],
                "policy": {"network": true}
            },
            "providers": {
                "format": 1,
                "revision": 1,
                "providers": [
                    {"id":"one","adapter":"responses","dialect":"openai_responses_v1","endpoint_owner":"openai","gateway_translation":"direct","evidence_revision":"openai-2026-08-01","endpoint":"https://one.example/v1","credential_key":"shared","models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,"context_window_tokens":100,"compact_trigger_tokens":50}]},
                    {"id":"two","adapter":"anthropic_messages","dialect":"anthropic_messages_v1","endpoint_owner":"anthropic","gateway_translation":"direct","evidence_revision":"anthropic-2026-08-01","endpoint":"https://two.example/v1","credential_key":"shared","models":[{"id":"claude-sonnet-4-20250514","profile":"anthropic_messages_v1:claude-sonnet-4-20250514","enabled":true,"context_window_tokens":100,"compact_trigger_tokens":50}]}
                ],
                "web_search": {"adapter":"tavily_v1","endpoint":"https://search.example","credential_key":"shared"}
            },
                        "settings": {"format":1,"revision":0},
            "revisions": {"workspace":1,"providers":1,"settings":0}
        }))
        .expect("config snapshot")
    }

    #[test]
    fn frozen_config_derives_model_and_independent_web_search_scopes() {
        let store = MemorySecretStore::new();
        store
            .publish(
                "shared",
                SecretRecord::Active {
                    generation: 9,
                    material: "fixture-secret-never-log".to_owned(),
                },
            )
            .expect("secret");
        let bindings =
            resolve_config_credentials(&config_with_shared_key(), &store).expect("bindings");
        assert_eq!(bindings.active.len(), 3);
        assert!(bindings.revoked.is_empty());
        assert_eq!(
            bindings.availability["shared"],
            CredentialAvailability::Active { generation: 9 }
        );
        assert!(bindings.active.iter().any(|scope| {
            scope.adapter == "tavily_v1"
                && scope.endpoint_origin == "https://search.example:443"
                && scope.purpose == "web_search"
        }));
        assert!(
            !bindings
                .active
                .iter()
                .any(|scope| { scope.adapter == "anthropic" && scope.purpose == "web_search" })
        );
        assert!(!format!("{bindings:?}").contains("fixture-secret-never-log"));
    }
}
