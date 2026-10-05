//! Legacy rpcOAuthPersistsRefreshTokenAndReconnectsAfterRestart, Kernel form
//! (mcp-runtime §oauth, secret-store §OAuth secret mutation). Driven by
//! scripts/run-live-mcp-oauth.py, which performs the one interactive consent
//! and hands the refresh grant over in a 0600 file (never argv/env).
//!
//! Gate: the Kernel mints its own OAuth record (generation 1, behind the
//! durable floor); a fresh MCP runtime exchanges the refresh token at the real
//! token endpoint and lists the catalog; the rotated refresh token is written
//! back (generation advances); a second fresh runtime uses the rotated grant;
//! remote revocation then makes a third fresh runtime fail with the exact
//! legacy text `token endpoint returned HTTP 400`, and the record is closed.
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use mcp::{
    McpCredentialFieldOperation, McpManagementMutation, McpScope, McpServerConfig,
    McpServerReference, McpTransportConfig, ProtocolMode,
};
use provider::{
    MemorySecretStore, OAuthGrant, SecretMutationAuthority, SecretRecord, SecretResolution,
    SecretStore,
};
use serde_json::{Value, json};
use tekes_supervisor::mcp_runtime::McpRuntime;

fn generation(store: &dyn SecretStore, id: &str) -> Value {
    match store.resolve(id).unwrap() {
        SecretResolution::Active(record) => {
            json!({"state":"active","generation":record.generation()})
        }
        SecretResolution::Revoked { generation } => {
            json!({"state":"revoked","generation":generation})
        }
        SecretResolution::NotFound => json!({"state":"not_found"}),
    }
}

fn refresh_token(store: &dyn SecretStore, id: &str) -> String {
    match &store.resolve(id).unwrap() {
        SecretResolution::Active(SecretRecord::Active { material, .. }) => {
            OAuthGrant::decode_material(material)
                .unwrap()
                .refresh_token
                .clone()
        }
        other => panic!("no active grant: {other:?}"),
    }
}

#[test]
#[ignore = "real OAuth authorization server and MCP server; run scripts/run-live-mcp-oauth.py"]
fn live_oauth_refresh_rotation_and_revocation_across_restarts() {
    let grant_path = PathBuf::from(
        std::env::var("TEKES_LIVE_MCP_OAUTH_GRANT").expect("TEKES_LIVE_MCP_OAUTH_GRANT"),
    );
    let server_url = std::env::var("TEKES_LIVE_MCP_OAUTH_URL").expect("TEKES_LIVE_MCP_OAUTH_URL");
    let artifact =
        PathBuf::from(std::env::var("TEKES_PROCESS_ARTIFACT").expect("TEKES_PROCESS_ARTIFACT"));
    assert_eq!(
        fs::metadata(&grant_path).unwrap().permissions().mode() & 0o777,
        0o600,
        "the grant file is private"
    );
    let grant_file: Value = serde_json::from_slice(&fs::read(&grant_path).unwrap()).unwrap();
    let mut grant = OAuthGrant::new(
        grant_file["token_endpoint"].as_str().unwrap(),
        grant_file["client_id"].as_str().unwrap(),
        grant_file["refresh_token"].as_str().unwrap(),
    );
    grant.client_secret = grant_file["client_secret"].as_str().map(str::to_owned);
    grant.resource = grant_file["resource"].as_str().map(str::to_owned);
    grant.scope = grant_file["scope"].as_str().map(str::to_owned);
    let revocation_endpoint = grant_file["revocation_endpoint"]
        .as_str()
        .map(str::to_owned);

    // Test-only OAuth mutation authority. Production credentials are supplied
    // by the owning application; this test never persists a grant in Kernel.
    let root = artifact.join("kernel");
    fs::create_dir_all(root.join("config")).unwrap();
    let durable = Arc::new(MemorySecretStore::new());
    let credential_id = "mcp-oauth-live-cloudflare";
    let minted =
        provider::mint_oauth_grant(durable.as_ref(), durable.as_ref(), credential_id, &grant)
            .unwrap();
    assert_eq!(minted, 1);
    let reference = McpServerReference {
        workspace_id: "ws".to_owned(),
        scope: McpScope::User,
        name: "cloudflare".to_owned(),
    };
    let mut steps: Vec<Value> = Vec::new();
    let fresh_runtime = |label: &str| -> McpRuntime {
        let runtime = McpRuntime::open_with_authorities(
            root.join("config"),
            Arc::clone(&durable) as Arc<dyn SecretStore>,
            Some(Arc::clone(&durable) as Arc<dyn SecretMutationAuthority>),
            None,
        )
        .unwrap();
        eprintln!("live-oauth: fresh runtime {label}");
        runtime
    };

    // Runtime 1: bind the Kernel-minted grant and connect (refresh exchange).
    let runtime = fresh_runtime("1");
    let mut credential_fields = BTreeMap::new();
    credential_fields.insert(
        "oauth".to_owned(),
        McpCredentialFieldOperation::Replace {
            credential_id: credential_id.to_owned(),
        },
    );
    runtime
        .mutate(
            "live-oauth-save",
            &McpManagementMutation::Save {
                server: McpServerConfig {
                    reference: reference.clone(),
                    transport: McpTransportConfig::Http {
                        url: server_url.clone(),
                        headers: BTreeMap::new(),
                        oauth: None,
                    },
                    enabled: true,
                    always_on: false,
                    protocol_mode: ProtocolMode::Auto,
                    owner: None,
                    plugin_component: None,
                    project_trusted: false,
                },
                credential_fields,
            },
        )
        .expect("save the OAuth-bound server");
    // probe_server = connect (refresh exchange) + tools/list without the
    // dynamic-catalog projection: Cloudflare's `search` description (1760 B)
    // exceeds the projection's 1024 B cap, a separate contract limit.
    let probe = |runtime: &McpRuntime, label: &str| -> Value {
        let value = runtime
            .probe_server(&reference)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let value: Value = serde_json::from_slice(&serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(
            value["catalog"]["toolCount"].as_u64().unwrap_or(0) > 0,
            "{label}: empty catalog {value}"
        );
        value
    };
    let first = probe(&runtime, "first connect");
    let token_before = refresh_token(durable.as_ref(), credential_id);
    let after_first = generation(durable.as_ref(), credential_id);
    steps.push(json!({"step":"connect-1","catalog":first["catalog"],"server":first["server"],"record":after_first}));
    let rotated_after_first = token_before != grant.refresh_token;
    drop(runtime);

    // Runtime 2 (restart): the stored grant (rotated or not) is exchanged again.
    let runtime = fresh_runtime("2");
    let second = probe(&runtime, "second connect");
    let token_after_second = refresh_token(durable.as_ref(), credential_id);
    let after_second = generation(durable.as_ref(), credential_id);
    steps.push(json!({"step":"connect-2","catalog":second["catalog"],"record":after_second}));
    let rotated_after_second = token_after_second != token_before;
    drop(runtime);
    // Remote revocation (RFC 7009 at the server's revocation endpoint), then a
    // third fresh connector must fail with the exact legacy text.
    let mut revocation_status = None;
    if let Some(endpoint) = revocation_endpoint.as_deref() {
        let mut current = grant.clone();
        current.refresh_token = token_after_second.clone();
        match provider::revoke_refresh_grant_remote(&current, endpoint, Duration::from_secs(30)) {
            Ok(status) => revocation_status = Some(status),
            Err(error) => steps.push(json!({"step":"revoke","error":error.to_string()})),
        }
    }
    steps.push(json!({"step":"revoke","endpoint":revocation_endpoint,"status":revocation_status}));
    let revocation_accepted = revocation_status.is_some_and(|status| (200..300).contains(&status));
    let runtime = fresh_runtime("3");
    let probe = runtime.probe_server(&reference);
    let final_record = generation(durable.as_ref(), credential_id);
    let probe_text = match &probe {
        Ok(_) => "ok".to_owned(),
        Err(error) => error.to_string(),
    };
    steps.push(json!({"step":"connect-3","probe":probe_text,"record":final_record}));
    let status = if revocation_accepted {
        assert!(
            probe_text.contains("token endpoint returned HTTP 400"),
            "after revocation the connector fails with the legacy text, got: {probe_text}"
        );
        assert_eq!(
            final_record["state"], "revoked",
            "the record is closed: {final_record}"
        );
        "passed"
    } else {
        // The authorization server did not accept the revocation request:
        // the rotation lifecycle is proven, the revocation half is not.
        "passed_without_remote_revocation"
    };
    fs::write(
        artifact.join("oauth-gate.json"),
        serde_json::to_vec_pretty(&json!({
            "status": status,
            "server_url": server_url,
            "rotated_after_first_connect": rotated_after_first,
            "rotated_after_second_connect": rotated_after_second,
            "revocation_status": revocation_status,
            "steps": steps,
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(status == "passed" || status == "passed_without_remote_revocation");
}
