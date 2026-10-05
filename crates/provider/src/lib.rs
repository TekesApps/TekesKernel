//! Production provider request, streaming, and credential protocol primitives.

mod compaction_summary;
pub use compaction_summary::{
    prepare_summary_request, summary_artifact_arguments, summary_completion_artifact,
};
mod credential;
mod oauth;
pub use oauth::{
    OAUTH_GRANT_KIND, OAuthExchangeError, OAuthGrant, OAuthTokenExchange, mint_oauth_grant,
    revoke_oauth_grant, revoke_refresh_grant_remote, rotate_oauth_grant,
};
mod dialect;
mod http;
mod native_deferred;
mod normalize;
pub use native_deferred::{DeferredToolDefinition, DeferredToolReference, NativeDeferredTools};
mod environment_secrets;
mod request;
mod secret_store;
pub use environment_secrets::EnvironmentSecretStore;
mod sse;

pub use credential::{
    BrokerDecision, BrokerError, CredentialBroker, CredentialBrokerControl, CredentialClient,
    CredentialClientError, CredentialControlError, CredentialDecoder, CredentialError,
    CredentialFrameError, CredentialGet, CredentialMaterial, CredentialMessage, CredentialScope,
    RevokedCredentialScope, credential_request_id, decode_credential_frame,
    encode_credential_frame, serve_credential_channel, start_credential_channel,
};
pub use dialect::{
    AdvertisedDialectProof, DialectError, DialectId, NativeDeferredMode, ProviderTarget,
    ResolvedDialectProfile, RouteEvidence, ThinkingWire, advertised_dialect_proofs,
    configured_route_is_verified, epoch_profile, resolve_profile, validate_endpoint,
    validate_epoch_target, validate_target,
};
pub use http::{HttpRuntime, HttpRuntimeError, HttpStatusClass, classify_status};
pub use normalize::{
    ContentBlock, FinishReason, INVALID_ARGUMENTS_KEY, ProviderCompletion, ProviderFailure,
    ProviderFrame, ProviderStreamDecoder, ProviderTerminal, ToolCall, Usage,
    invalid_arguments_detail, invalid_arguments_sentinel, normalize_dialect_response,
    normalize_dialect_sse_stream, normalize_response, normalize_sse_stream, repair_tool_arguments,
};
pub use request::{
    AdapterId, PrepareError, PrepareInput, PreparedRequest, ProviderCapabilities, ToolChoice,
    endpoint_origin, prepare, prepare_with_native_deferred_tools, prepare_with_tool_choice,
    provider_query_key, provider_request_digest, provider_request_digest_for_dialect,
};
pub use secret_store::{
    CredentialAvailability, MemorySecretStore, ResolveBindingsError, ResolvedCredentialBindings,
    SecretMutationAuthority, SecretRecord, SecretResolution, SecretStore, SecretStoreError,
    resolve_config_credentials,
};
pub use sse::{SseDecoder, SseError, SseEvent};
