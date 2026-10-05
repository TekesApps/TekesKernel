//! OAuth refresh-grant records the Kernel mints itself (secret-store §OAuth
//! secret mutation) and the refresh-token exchange behind an `oauth` MCP
//! binding (mcp-runtime §oauth): exchange at the grant's token endpoint,
//! rotate the stored refresh token when the response carries a new one, and on
//! HTTP 400 (`invalid_grant`) publish the revoked record and fail closed.
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::{
    SecretMutationAuthority, SecretRecord, SecretResolution, SecretStore, SecretStoreError,
};

/// Material discriminator of a Kernel-minted OAuth record. Any other active
/// material behind an `oauth` binding is a platform-installed access token and
/// is injected as-is.
pub const OAUTH_GRANT_KIND: &str = "tekes-oauth-refresh-grant-v1";

/// The closed in-memory record carries everything the refresh
/// exchange needs. Encoded as the canonical JSON material of an active record.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OAuthGrant {
    pub kind: String,
    pub token_endpoint: String,
    pub client_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub refresh_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl std::fmt::Debug for OAuthGrant {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OAuthGrant")
            .field("token_endpoint", &self.token_endpoint)
            .field("client_id", &self.client_id)
            .field(
                "client_secret",
                &self.client_secret.as_ref().map(|_| "<redacted>"),
            )
            .field("refresh_token", &"<redacted>")
            .field("resource", &self.resource)
            .field("scope", &self.scope)
            .finish()
    }
}

impl Drop for OAuthGrant {
    fn drop(&mut self) {
        self.refresh_token.zeroize();
        if let Some(secret) = &mut self.client_secret {
            secret.zeroize();
        }
    }
}

impl OAuthGrant {
    pub fn new(
        token_endpoint: impl Into<String>,
        client_id: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        Self {
            kind: OAUTH_GRANT_KIND.to_owned(),
            token_endpoint: token_endpoint.into(),
            client_id: client_id.into(),
            client_secret: None,
            refresh_token: refresh_token.into(),
            resource: None,
            scope: None,
        }
    }

    pub fn encode_material(&self) -> Result<String, SecretStoreError> {
        if self.kind != OAUTH_GRANT_KIND
            || self.refresh_token.is_empty()
            || self.client_id.is_empty()
            || !(self.token_endpoint.starts_with("https://")
                || self.token_endpoint.starts_with("http://127.0.0.1"))
        {
            return Err(SecretStoreError::InvalidRecord);
        }
        let bytes =
            serde_json_canonicalizer::to_vec(self).map_err(|_| SecretStoreError::InvalidRecord)?;
        String::from_utf8(bytes).map_err(|_| SecretStoreError::InvalidRecord)
    }

    /// `None` when the material is not a Kernel-minted grant.
    #[must_use]
    pub fn decode_material(material: &str) -> Option<Self> {
        if !material.starts_with('{') {
            return None;
        }
        let grant: Self = serde_json::from_str(material).ok()?;
        (grant.kind == OAUTH_GRANT_KIND && !grant.refresh_token.is_empty()).then_some(grant)
    }
}

fn current_grant(
    store: &dyn SecretStore,
    credential_id: &str,
) -> Result<(OAuthGrant, u64), SecretStoreError> {
    match &store.resolve(credential_id)? {
        SecretResolution::Active(SecretRecord::Active {
            generation,
            material,
        }) => OAuthGrant::decode_material(material)
            .map(|grant| (grant, *generation))
            .ok_or(SecretStoreError::InvalidRecord),
        _ => Err(SecretStoreError::InvalidRecord),
    }
}

/// Mint generation 1 of a Kernel-owned grant; the id must be unused.
pub fn mint_oauth_grant(
    store: &dyn SecretStore,
    authority: &dyn SecretMutationAuthority,
    credential_id: &str,
    grant: &OAuthGrant,
) -> Result<u64, SecretStoreError> {
    if !matches!(store.resolve(credential_id)?, SecretResolution::NotFound) {
        return Err(SecretStoreError::GenerationConflict);
    }
    authority.publish_record(
        credential_id,
        None,
        SecretRecord::Active {
            generation: 1,
            material: grant.encode_material()?,
        },
    )?;
    Ok(1)
}

/// Replace the refresh token of a Kernel-minted grant in place (generation
/// +1). A record the Kernel did not mint is refused (`InvalidRecord`).
pub fn rotate_oauth_grant(
    store: &dyn SecretStore,
    authority: &dyn SecretMutationAuthority,
    credential_id: &str,
    refresh_token: &str,
) -> Result<u64, SecretStoreError> {
    let (mut grant, generation) = current_grant(store, credential_id)?;
    grant.refresh_token = refresh_token.to_owned();
    let next = generation
        .checked_add(1)
        .ok_or(SecretStoreError::InvalidRecord)?;
    authority.publish_record(
        credential_id,
        Some(generation),
        SecretRecord::Active {
            generation: next,
            material: grant.encode_material()?,
        },
    )?;
    Ok(next)
}

/// Publish the closed revoked record over a Kernel-minted grant.
pub fn revoke_oauth_grant(
    store: &dyn SecretStore,
    authority: &dyn SecretMutationAuthority,
    credential_id: &str,
) -> Result<u64, SecretStoreError> {
    let (_grant, generation) = current_grant(store, credential_id)?;
    let next = generation
        .checked_add(1)
        .ok_or(SecretStoreError::InvalidRecord)?;
    authority.publish_record(
        credential_id,
        Some(generation),
        SecretRecord::Revoked { generation: next },
    )?;
    Ok(next)
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum OAuthExchangeError {
    /// The store has no active Kernel-minted grant (missing, revoked, foreign).
    #[error("OAuth credential {0} is unavailable")]
    Unavailable(String),
    /// The token endpoint refused the refresh grant (legacy exact text).
    #[error("token endpoint returned HTTP 400")]
    Revoked,
    #[error("token endpoint returned HTTP {0}")]
    Status(u16),
    #[error("token endpoint: {0}")]
    Transport(String),
}

struct CachedAccess {
    token: String,
    generation: u64,
    expires_at: Option<Instant>,
}

impl Drop for CachedAccess {
    fn drop(&mut self) {
        self.token.zeroize();
    }
}

/// Access-token authority for one `oauth` binding: the identity is stable
/// across rotations (`oauth:<credential>:<client_id>`), the access token is
/// cached per stored generation until it expires, and every exchange that
/// returns a new refresh token rotates the stored grant.
pub struct OAuthTokenExchange {
    credential_id: String,
    store: Arc<dyn SecretStore>,
    authority: Arc<dyn SecretMutationAuthority>,
    cache: Mutex<Option<CachedAccess>>,
    /// Set once this connector saw the token endpoint refuse the grant: every
    /// later request on the same connector (the MCP client retries a failed
    /// request once) reports the same exact failure, not "unavailable".
    revoked: std::sync::atomic::AtomicBool,
    timeout: Duration,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    refresh_token: Option<String>,
}

impl OAuthTokenExchange {
    pub fn new(
        credential_id: impl Into<String>,
        store: Arc<dyn SecretStore>,
        authority: Arc<dyn SecretMutationAuthority>,
    ) -> Self {
        Self {
            credential_id: credential_id.into(),
            store,
            authority,
            cache: Mutex::new(None),
            revoked: std::sync::atomic::AtomicBool::new(false),
            timeout: Duration::from_secs(30),
        }
    }

    /// The generation the cached access token was issued under, when any.
    #[must_use]
    pub fn cached_generation(&self) -> Option<u64> {
        self.cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|cached| cached.generation)
    }

    /// `(identity, access_token, generation)` for the next request.
    pub fn bearer(&self) -> Result<(String, String, u64), OAuthExchangeError> {
        if self.revoked.load(std::sync::atomic::Ordering::Acquire) {
            return Err(OAuthExchangeError::Revoked);
        }
        let (grant, generation) = current_grant(self.store.as_ref(), &self.credential_id)
            .map_err(|_| OAuthExchangeError::Unavailable(self.credential_id.clone()))?;
        let identity = format!("oauth:{}:{}", self.credential_id, grant.client_id);
        {
            let cache = self
                .cache
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(cached) = cache.as_ref() {
                let fresh = cached.expires_at.is_none_or(|at| Instant::now() < at);
                if cached.generation == generation && fresh {
                    return Ok((identity, cached.token.clone(), generation));
                }
            }
        }
        let response = exchange_refresh_grant(&grant, self.timeout);
        let response = match response {
            Ok(response) => response,
            Err(OAuthExchangeError::Revoked) => {
                // invalid_grant: the peer generation is closed durably; a later
                // connector sees the revoked record, never a retry.
                let _ = revoke_oauth_grant(
                    self.store.as_ref(),
                    self.authority.as_ref(),
                    &self.credential_id,
                );
                self.revoked
                    .store(true, std::sync::atomic::Ordering::Release);
                return Err(OAuthExchangeError::Revoked);
            }
            Err(error) => return Err(error),
        };
        let mut generation = generation;
        if let Some(rotated) = response
            .refresh_token
            .as_deref()
            .filter(|token| *token != grant.refresh_token)
        {
            generation = rotate_oauth_grant(
                self.store.as_ref(),
                self.authority.as_ref(),
                &self.credential_id,
                rotated,
            )
            .map_err(|error| {
                OAuthExchangeError::Transport(format!(
                    "refresh token rotation was not persisted: {error}"
                ))
            })?;
        }
        let expires_at = response
            .expires_in
            .map(|seconds| Instant::now() + Duration::from_secs(seconds.saturating_sub(30).max(1)));
        let token = response.access_token.clone();
        *self
            .cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(CachedAccess {
            token: response.access_token.clone(),
            generation,
            expires_at,
        });
        Ok((identity, token, generation))
    }
}

/// One refresh-grant POST on its own thread and runtime (callers may sit on
/// an async runtime); no redirects, bounded time, JSON body only.
fn exchange_refresh_grant(
    grant: &OAuthGrant,
    timeout: Duration,
) -> Result<TokenResponse, OAuthExchangeError> {
    let mut form = vec![
        ("grant_type".to_owned(), "refresh_token".to_owned()),
        ("refresh_token".to_owned(), grant.refresh_token.clone()),
        ("client_id".to_owned(), grant.client_id.clone()),
    ];
    if let Some(secret) = &grant.client_secret {
        form.push(("client_secret".to_owned(), secret.clone()));
    }
    if let Some(resource) = &grant.resource {
        form.push(("resource".to_owned(), resource.clone()));
    }
    // Authorization servers that rotate refresh tokens answer a refresh that
    // overlaps a just-completed one with 429 `temporarily_unavailable` and a
    // Retry-After (Cloudflare: "Token refresh is already in progress"). That
    // is a transient, not a revocation: honour the hint a bounded number of
    // times before failing the connect with the status.
    let mut attempts = 0;
    loop {
        attempts += 1;
        let (status, retry_after, mut body) = post_form(&grant.token_endpoint, &form, timeout)?;
        let parsed = match status {
            200 => serde_json::from_slice::<TokenResponse>(&body).map_err(|_| {
                OAuthExchangeError::Transport(
                    "token endpoint returned a malformed token response".to_owned(),
                )
            }),
            400 => Err(OAuthExchangeError::Revoked),
            429 if attempts < 4 => {
                body.zeroize();
                std::thread::sleep(Duration::from_secs(retry_after.unwrap_or(5).clamp(1, 30)));
                continue;
            }
            other => Err(OAuthExchangeError::Status(other)),
        };
        body.zeroize();
        return parsed;
    }
}

/// One form POST on its own thread and runtime; returns status, Retry-After
/// seconds when present, and the body.
fn post_form(
    endpoint: &str,
    form: &[(String, String)],
    timeout: Duration,
) -> Result<(u16, Option<u64>, Vec<u8>), OAuthExchangeError> {
    let endpoint = endpoint.to_owned();
    let form = form.to_vec();
    std::thread::scope(|scope| {
        scope
            .spawn(
                move || -> Result<(u16, Option<u64>, Vec<u8>), OAuthExchangeError> {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .map_err(|error| OAuthExchangeError::Transport(error.to_string()))?;
                    runtime.block_on(async move {
                        let client = reqwest::Client::builder()
                            .connect_timeout(Duration::from_secs(30))
                            .timeout(timeout)
                            .redirect(reqwest::redirect::Policy::none())
                            .build()
                            .map_err(|error| OAuthExchangeError::Transport(error.to_string()))?;
                        let response = client
                            .post(&endpoint)
                            .header("accept", "application/json")
                            .form(&form)
                            .send()
                            .await
                            .map_err(|error| {
                                OAuthExchangeError::Transport(redact(&error.to_string()))
                            })?;
                        let status = response.status().as_u16();
                        let retry_after = response
                            .headers()
                            .get("retry-after")
                            .and_then(|value| value.to_str().ok())
                            .and_then(|value| value.trim().parse::<u64>().ok());
                        let body = response.bytes().await.map_err(|error| {
                            OAuthExchangeError::Transport(redact(&error.to_string()))
                        })?;
                        Ok((status, retry_after, body.to_vec()))
                    })
                },
            )
            .join()
            .unwrap_or_else(|_| {
                Err(OAuthExchangeError::Transport(
                    "token exchange thread panicked".to_owned(),
                ))
            })
    })
}

/// RFC 7009 revocation of the grant's current refresh token at
/// `revocation_endpoint`; returns the HTTP status. Used by the live gate to
/// revoke remotely between connectors; never retried.
#[doc(hidden)]
pub fn revoke_refresh_grant_remote(
    grant: &OAuthGrant,
    revocation_endpoint: &str,
    timeout: Duration,
) -> Result<u16, OAuthExchangeError> {
    let mut form = vec![
        ("token".to_owned(), grant.refresh_token.clone()),
        ("token_type_hint".to_owned(), "refresh_token".to_owned()),
        ("client_id".to_owned(), grant.client_id.clone()),
    ];
    if let Some(secret) = &grant.client_secret {
        form.push(("client_secret".to_owned(), secret.clone()));
    }
    let endpoint = revocation_endpoint.to_owned();
    std::thread::scope(|scope| {
        scope
            .spawn(move || -> Result<u16, OAuthExchangeError> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|error| OAuthExchangeError::Transport(error.to_string()))?;
                runtime.block_on(async move {
                    let client = reqwest::Client::builder()
                        .connect_timeout(Duration::from_secs(30))
                        .timeout(timeout)
                        .redirect(reqwest::redirect::Policy::none())
                        .build()
                        .map_err(|error| OAuthExchangeError::Transport(error.to_string()))?;
                    let response =
                        client
                            .post(&endpoint)
                            .form(&form)
                            .send()
                            .await
                            .map_err(|error| {
                                OAuthExchangeError::Transport(redact(&error.to_string()))
                            })?;
                    Ok(response.status().as_u16())
                })
            })
            .join()
            .unwrap_or_else(|_| {
                Err(OAuthExchangeError::Transport(
                    "revocation thread panicked".to_owned(),
                ))
            })
    })
}

fn redact(message: &str) -> String {
    // reqwest errors carry the URL, never the form body; keep only the class.
    message
        .split(':')
        .next()
        .unwrap_or("request failed")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemorySecretStore;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    fn token_endpoint(
        responses: Vec<(u16, String)>,
    ) -> (String, std::thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/token", listener.local_addr().unwrap());
        let handle = std::thread::spawn(move || {
            let mut bodies = Vec::new();
            for (status, body) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = Vec::new();
                let mut chunk = [0_u8; 4096];
                loop {
                    let count = stream.read(&mut chunk).unwrap();
                    request.extend_from_slice(&chunk[..count]);
                    if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                        let headers =
                            String::from_utf8_lossy(&request[..end + 4]).to_ascii_lowercase();
                        let length = headers
                            .lines()
                            .find_map(|l| l.strip_prefix("content-length: "))
                            .and_then(|v| v.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                        if request.len() >= end + 4 + length {
                            bodies.push(String::from_utf8_lossy(&request[end + 4..]).into_owned());
                            break;
                        }
                    }
                    if count == 0 {
                        break;
                    }
                }
                let retry = if status == 429 {
                    "retry-after: 1\r\n"
                } else {
                    ""
                };
                write!(stream, "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\n{retry}content-length: {}\r\nconnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
            bodies
        });
        (endpoint, handle)
    }

    #[test]
    fn grant_material_is_closed_and_only_kernel_minted_records_rotate() {
        let store = Arc::new(MemorySecretStore::new());
        let grant = OAuthGrant::new("https://issuer.example/token", "client-1", "refresh-1");
        let material = grant.encode_material().unwrap();
        assert!(
            material.starts_with("{\"client_id\":\"client-1\""),
            "{material}"
        );
        assert_eq!(
            OAuthGrant::decode_material(&material)
                .unwrap()
                .refresh_token,
            "refresh-1"
        );
        assert!(OAuthGrant::decode_material("plain-access-token").is_none());
        assert!(OAuthGrant::decode_material("{\"kind\":\"other\",\"token_endpoint\":\"x\",\"client_id\":\"c\",\"refresh_token\":\"r\"}").is_none());
        assert!(
            format!("{grant:?}").contains("<redacted>")
                && !format!("{grant:?}").contains("refresh-1")
        );
        assert_eq!(
            mint_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf", &grant).unwrap(),
            1
        );
        assert_eq!(
            mint_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf", &grant).unwrap_err(),
            SecretStoreError::GenerationConflict
        );
        assert_eq!(
            rotate_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf", "refresh-2").unwrap(),
            2
        );
        match &store.resolve("oauth-cf").unwrap() {
            SecretResolution::Active(SecretRecord::Active {
                generation,
                material,
            }) => {
                assert_eq!(*generation, 2);
                assert_eq!(
                    OAuthGrant::decode_material(material).unwrap().refresh_token,
                    "refresh-2"
                );
            }
            other => panic!("{other:?}"),
        }
        store
            .publish(
                "platform-token",
                SecretRecord::Active {
                    generation: 1,
                    material: "installed-by-the-app".to_owned(),
                },
            )
            .unwrap();
        assert_eq!(
            rotate_oauth_grant(store.as_ref(), store.as_ref(), "platform-token", "x").unwrap_err(),
            SecretStoreError::InvalidRecord,
            "platform-minted records stay read-only"
        );
        assert_eq!(
            revoke_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf").unwrap(),
            3
        );
        assert!(matches!(
            store.resolve("oauth-cf").unwrap(),
            SecretResolution::Revoked { generation: 3 }
        ));
        assert_eq!(
            rotate_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf", "x").unwrap_err(),
            SecretStoreError::InvalidRecord
        );
    }

    #[test]
    fn exchange_honours_retry_after_on_429_before_failing() {
        let (endpoint, server) = token_endpoint(vec![
            (429, r#"{"error":"temporarily_unavailable","error_description":"Token refresh is already in progress; retry shortly"}"#.to_owned()),
            (200, r#"{"access_token":"access-late","token_type":"bearer","expires_in":60}"#.to_owned()),
        ]);
        let store = Arc::new(MemorySecretStore::new());
        mint_oauth_grant(
            store.as_ref(),
            store.as_ref(),
            "oauth-cf",
            &OAuthGrant::new(endpoint, "client-1", "refresh-1"),
        )
        .unwrap();
        let exchange = OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let started = Instant::now();
        assert_eq!(exchange.bearer().unwrap().1, "access-late");
        assert!(
            started.elapsed() >= Duration::from_secs(1),
            "the retry waited for the hint (default when absent: 5 s, floor 1 s)"
        );
        assert_eq!(server.join().unwrap().len(), 2);
    }

    #[test]
    fn exchange_caches_rotates_and_revokes_on_invalid_grant() {
        let (endpoint, server) = token_endpoint(vec![
            (200, r#"{"access_token":"access-1","token_type":"bearer","expires_in":3600,"refresh_token":"refresh-2"}"#.to_owned()),
            (200, r#"{"access_token":"access-2","token_type":"bearer","expires_in":3600}"#.to_owned()),
            (400, r#"{"error":"invalid_grant","error_description":"revoked"}"#.to_owned()),
        ]);
        let store = Arc::new(MemorySecretStore::new());
        let mut grant = OAuthGrant::new(endpoint, "client-1", "refresh-1");
        grant.resource = Some("https://mcp.example/mcp".to_owned());
        mint_oauth_grant(store.as_ref(), store.as_ref(), "oauth-cf", &grant).unwrap();
        let exchange = OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let (identity, token, generation) = exchange.bearer().unwrap();
        assert_eq!(
            (identity.as_str(), token.as_str(), generation),
            ("oauth:oauth-cf:client-1", "access-1", 2),
            "rotation persisted as generation 2"
        );
        assert_eq!(
            exchange.bearer().unwrap().1,
            "access-1",
            "cached until expiry, no second exchange"
        );
        // A fresh connector (restart) exchanges the rotated refresh token.
        let restarted = OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let (identity, token, generation) = restarted.bearer().unwrap();
        assert_eq!(
            (identity.as_str(), token.as_str(), generation),
            ("oauth:oauth-cf:client-1", "access-2", 2),
            "no new refresh token: generation stays"
        );
        // Remote revocation: the next fresh connector fails with the exact legacy text and closes the record.
        let revoked = OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let error = revoked.bearer().unwrap_err();
        assert_eq!(error, OAuthExchangeError::Revoked);
        assert_eq!(error.to_string(), "token endpoint returned HTTP 400");
        assert!(matches!(
            store.resolve("oauth-cf").unwrap(),
            SecretResolution::Revoked { generation: 3 }
        ));
        assert_eq!(
            revoked.bearer().unwrap_err(),
            OAuthExchangeError::Revoked,
            "the same connector keeps the exact failure (the MCP client retries once)"
        );
        let later = OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        assert_eq!(
            later.bearer().unwrap_err(),
            OAuthExchangeError::Unavailable("oauth-cf".to_owned()),
            "a later connector sees the closed record: no exchange, no retry"
        );
        let bodies = server.join().unwrap();
        assert!(
            bodies[0].contains("grant_type=refresh_token")
                && bodies[0].contains("refresh_token=refresh-1")
                && bodies[0].contains("resource=https%3A%2F%2Fmcp.example%2Fmcp"),
            "{}",
            bodies[0]
        );
        assert!(
            bodies[1].contains("refresh_token=refresh-2"),
            "the restart used the rotated token: {}",
            bodies[1]
        );
    }
}
