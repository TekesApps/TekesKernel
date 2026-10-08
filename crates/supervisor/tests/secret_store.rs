use std::sync::Arc;

use profile::ConfigSnapshot;
use provider::{MemorySecretStore, SecretRecord, SecretResolution, SecretStore, SecretStoreError};
use serde_json::json;
use tekes_supervisor::endpoint_host::{
    ProviderReadinessAuthority, RuntimeProviderFailure, RuntimeProviderStatus,
};
use tekes_supervisor::process_host::ProductionProcessHost;

fn config() -> ConfigSnapshot {
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
            "providers": [{
                "id":"provider-main",
                "adapter":"responses",
                "dialect":"openai_responses_v1",
                "endpoint_owner":"openai",
                "gateway_translation":"direct",
                "evidence_revision":"openai-2026-08-01",
                "endpoint":"https://api.openai.com/v1",
                "credential_key":"shared",
                "models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,"context_window_tokens":100,"compact_trigger_tokens":50}]
            }]
        },
                "settings": {"format":1,"revision":0},
        "revisions": {"workspace":1,"providers":1,"settings":0}
    }))
    .expect("config snapshot")
}

fn host(store: Arc<dyn SecretStore>) -> (tempfile::TempDir, Arc<ProductionProcessHost>) {
    let directory = tempfile::tempdir().expect("root");
    let host = ProductionProcessHost::open_with_secret_store(
        directory.path(),
        std::env::current_exe().expect("test executable"),
        env!("CARGO_PKG_VERSION"),
        directory.path().join(".agent"),
        store,
    )
    .expect("process host");
    (directory, host)
}

#[test]
fn provider_readiness_tracks_active_missing_and_revoked_secret_records() {
    let store = Arc::new(MemorySecretStore::new());
    let (_directory, host) = host(Arc::clone(&store) as Arc<dyn SecretStore>);

    let missing = host.readiness("session", &config()).expect("missing state");
    assert_eq!(
        missing[0].status,
        RuntimeProviderStatus::Failed {
            failure: RuntimeProviderFailure::InvalidCredential
        }
    );

    store
        .publish(
            "shared",
            SecretRecord::Active {
                generation: 1,
                material: "fixture-secret-never-log".to_owned(),
            },
        )
        .expect("active");
    let ready = host.readiness("session", &config()).expect("active state");
    assert_eq!(ready[0].status, RuntimeProviderStatus::Ready);
    assert_eq!(ready[0].models.len(), 1);

    store
        .publish("shared", SecretRecord::Revoked { generation: 2 })
        .expect("revoke");
    let revoked = host.readiness("session", &config()).expect("revoked state");
    assert_eq!(
        revoked[0].status,
        RuntimeProviderStatus::Failed {
            failure: RuntimeProviderFailure::InvalidCredential
        }
    );
    assert!(revoked[0].models.is_empty());

    let evidence = format!("{missing:?}{ready:?}{revoked:?}");
    assert!(!evidence.contains("fixture-secret-never-log"));
}

struct UnavailableStore;

impl SecretStore for UnavailableStore {
    fn resolve(&self, _credential_id: &str) -> Result<SecretResolution, SecretStoreError> {
        Err(SecretStoreError::Unavailable)
    }
}

#[test]
fn provider_readiness_distinguishes_store_unavailable() {
    let (_directory, host) = host(Arc::new(UnavailableStore));
    let readiness = host
        .readiness("session", &config())
        .expect("typed unavailable");
    assert_eq!(
        readiness[0].status,
        RuntimeProviderStatus::Failed {
            failure: RuntimeProviderFailure::Unavailable
        }
    );
}

fn with_provider(fields: serde_json::Value) -> ConfigSnapshot {
    let mut value = serde_json::to_value(config()).expect("config JSON");
    let provider = value
        .pointer_mut("/providers/providers/0")
        .and_then(serde_json::Value::as_object_mut)
        .expect("provider object");
    for (key, field) in fields.as_object().expect("fields") {
        provider.insert(key.clone(), field.clone());
    }
    serde_json::from_value(value).expect("provider config")
}

#[test]
fn provider_readiness_accepts_a_configured_gateway_the_kernel_has_never_seen() {
    let config = with_provider(json!({
        "dialect":"deepseek_responses_v1",
        "endpoint_owner":"example",
        "gateway_translation":"newapi-openai-responses",
        "evidence_revision":"example-1",
        "endpoint":"https://gateway.example/v1",
        "credential_key":null,
        "models":[{
            "id":"private-alias",
            "profile":"deepseek_responses_v1:private-alias",
            "enabled":true,
            "context_window_tokens":100,
            "compact_trigger_tokens":50
        }]
    }));
    let (_directory, host) = host(Arc::new(MemorySecretStore::new()));
    let readiness = host.readiness("session", &config).expect("readiness");
    assert_eq!(readiness[0].status, RuntimeProviderStatus::Ready);
    assert_eq!(readiness[0].models.len(), 1);
}

#[test]
fn provider_readiness_rejects_an_unknown_dialect_as_misconfigured() {
    let config = with_provider(json!({"dialect":"unknown_v1","credential_key":null}));
    let (_directory, host) = host(Arc::new(MemorySecretStore::new()));
    let readiness = host.readiness("session", &config).expect("readiness");
    assert_eq!(
        readiness[0].status,
        RuntimeProviderStatus::Failed {
            failure: RuntimeProviderFailure::Misconfigured
        }
    );
    assert!(readiness[0].models.is_empty());
}
