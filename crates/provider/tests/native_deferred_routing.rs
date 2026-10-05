//! Native deferred-tool mode is a property of the exact route declared in the
//! reviewed capability catalog, never of the adapter family.
use profile::{Model, Provider};
use provider::NativeDeferredMode;

fn provider(
    dialect: &str,
    adapter: &str,
    owner: &str,
    translation: &str,
    endpoint: &str,
    sku: &str,
) -> (Provider, Model) {
    let model = Model {
        id: sku.to_owned(),
        profile: format!("{dialect}:{sku}"),
        enabled: true,
        context_window_tokens: 200_000,
        compact_trigger_tokens: 180_000,
    };
    (
        Provider {
            id: format!("{owner}-{dialect}"),
            name: None,
            adapter: adapter.to_owned(),
            dialect: dialect.to_owned(),
            endpoint_owner: owner.to_owned(),
            gateway_translation: translation.to_owned(),
            evidence_revision: "legacy-example-v2".to_owned(),
            endpoint: endpoint.to_owned(),
            credential_key: Some("key".to_owned()),
            models: vec![model.clone()],
        },
        model,
    )
}

#[test]
fn native_mode_is_declared_per_exact_route_only() {
    let cases = [
        (
            "openai_responses_v1",
            "responses",
            "cloudflare",
            "router",
            "https://api.cloudflare.com/client/v4/accounts/x/ai/v1",
            "openai/gpt-5.6-luna",
            Some(NativeDeferredMode::OpenaiClientToolSearch),
        ),
        (
            "openai_responses_v1",
            "responses",
            "cloudflare",
            "router",
            "https://api.cloudflare.com/client/v4/accounts/x/ai/v1",
            "openai/gpt-5.6-sol",
            None,
        ),
        (
            "openai_responses_v1",
            "responses",
            "openai",
            "direct",
            "https://api.openai.com/v1",
            "openai/gpt-5.6-luna",
            None,
        ),
        (
            "openai_responses_v1",
            "responses",
            "openai",
            "direct",
            "https://api.openai.com/v1",
            "gpt-5",
            None,
        ),
        (
            "anthropic_messages_v1",
            "anthropic_messages",
            "cloudflare",
            "cloudflare-native-anthropic",
            "https://gateway.ai.cloudflare.com/v1/x/router/anthropic",
            "claude-fable-5",
            Some(NativeDeferredMode::AnthropicCustomToolReference),
        ),
        (
            "anthropic_messages_v1",
            "anthropic_messages",
            "anthropic",
            "direct",
            "https://api.anthropic.com",
            "claude-fable-5",
            None,
        ),
        (
            "anthropic_messages_v1",
            "anthropic_messages",
            "cloudflare",
            "cloudflare-native-anthropic",
            "https://gateway.ai.cloudflare.com/v1/x/router/anthropic",
            "claude-opus-5",
            None,
        ),
    ];
    for (dialect, adapter, owner, translation, endpoint, sku, expected) in cases {
        let (provider, model) = provider(dialect, adapter, owner, translation, endpoint, sku);
        let resolved =
            provider::resolve_profile(&provider, &model).expect("configured route resolves");
        assert_eq!(
            resolved.native_deferred_tools(),
            expected,
            "{dialect} {owner}/{translation} {sku}"
        );
    }
    assert_eq!(
        NativeDeferredMode::OpenaiClientToolSearch.dialect(),
        provider::DialectId::OpenaiResponsesV1
    );
    assert_eq!(
        NativeDeferredMode::AnthropicCustomToolReference.dialect(),
        provider::DialectId::AnthropicMessagesV1
    );
}
