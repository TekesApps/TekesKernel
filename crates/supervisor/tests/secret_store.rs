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

#[test]
fn provider_readiness_rejects_test_only_dialect_profile() {
    let mut value = serde_json::to_value(config()).expect("config JSON");
    let provider = value
        .pointer_mut("/providers/providers/0")
        .and_then(serde_json::Value::as_object_mut)
        .expect("provider object");
    provider.insert("adapter".to_owned(), json!("chat_completions"));
    provider.insert("dialect".to_owned(), json!("generic_chat_v1"));
    provider.insert("endpoint_owner".to_owned(), json!("fixture"));
    provider.insert("evidence_revision".to_owned(), json!("kernel-fixture-1"));
    provider.insert("endpoint".to_owned(), json!("https://fixture.invalid/v1"));
    provider.insert("credential_key".to_owned(), serde_json::Value::Null);
    provider.insert(
        "models".to_owned(),
        json!([{
            "id":"generic-chat-fixture-1",
            "profile":"generic_chat_v1:generic-chat-fixture-1",
            "enabled":true,
            "context_window_tokens":100,
            "compact_trigger_tokens":50
        }]),
    );
    let config: ConfigSnapshot = serde_json::from_value(value).expect("generic config");
    let (_directory, host) = host(Arc::new(MemorySecretStore::new()));
    let readiness = host
        .readiness("session", &config)
        .expect("typed dialect proof failure");
    assert_eq!(
        readiness[0].status,
        RuntimeProviderStatus::Failed {
            failure: RuntimeProviderFailure::DialectUnproved
        }
    );
    assert!(readiness[0].models.is_empty());
}
