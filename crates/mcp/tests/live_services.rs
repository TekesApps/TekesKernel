use mcp::{HttpTransport, McpCancellationToken, McpClient, McpPeer, ProtocolMode};
use serde_json::json;
use std::collections::BTreeMap;

async fn public_round_trip(url: &str, mode: ProtocolMode, tool: &str, args: serde_json::Value) {
    let transport =
        HttpTransport::new(url.parse().unwrap(), &BTreeMap::new(), None, false).unwrap();
    let mut client = McpClient::new("live-public", transport);
    client.connect(mode).await.expect("real MCP handshake");
    assert!(
        client
            .supported_versions()
            .iter()
            .any(|version| version == client.protocol_version())
    );
    let catalog = client.list_tools().await.expect("real tools/list");
    assert!(
        catalog.iter().any(|entry| entry.name == tool),
        "live catalog lacks expected tool"
    );
    let arguments = schema::IJsonValue::parse(&serde_json::to_vec(&args).unwrap()).unwrap();
    let result = client
        .call_tool(tool, arguments, McpCancellationToken::default())
        .await
        .expect("real tools/call");
    let value = serde_json::to_value(result).unwrap();
    assert_ne!(value["isError"], true);
    assert!(
        value["content"]
            .as_array()
            .is_some_and(|parts| !parts.is_empty())
    );
    client.close().await.expect("close real MCP peer");
}

#[tokio::test]
#[ignore = "calls the public Cloudflare MCP service"]
async fn live_cloudflare_docs_discovery_list_and_call() {
    public_round_trip(
        "https://docs.mcp.cloudflare.com/mcp",
        ProtocolMode::Modern,
        "search_cloudflare_documentation",
        json!({"query":"MCP stateless server/discover"}),
    )
    .await;
}

#[tokio::test]
#[ignore = "calls the public DeepWiki MCP service"]
async fn live_deepwiki_list_and_call() {
    public_round_trip(
        "https://mcp.deepwiki.com/mcp",
        ProtocolMode::Auto,
        "read_wiki_structure",
        json!({"repoName":"modelcontextprotocol/specification"}),
    )
    .await;
}

#[tokio::test]
#[ignore = "requires TEKES_LIVE_CLOUDFLARE_API_TOKEN; read-only remote call"]
async fn live_cloudflare_bearer_catalog_and_search() {
    let token =
        std::env::var("TEKES_LIVE_CLOUDFLARE_API_TOKEN").expect("missing credential is not a pass");
    let transport = HttpTransport::new(
        "https://mcp.cloudflare.com/mcp".parse().unwrap(),
        &BTreeMap::new(),
        Some(&token),
        false,
    )
    .unwrap();
    let mut client = McpClient::new("live-cloudflare-account", transport);
    client
        .connect(ProtocolMode::Modern)
        .await
        .expect("authenticated handshake");
    assert_eq!(client.protocol_version(), mcp::MODERN_PROTOCOL_VERSION);
    assert_eq!(
        client.supported_versions(),
        &[mcp::MODERN_PROTOCOL_VERSION.to_owned()]
    );
    let tools = client.list_tools().await.expect("authenticated catalog");
    assert!(tools.iter().any(|tool| tool.name == "search"));
    assert!(tools.iter().any(|tool| tool.name == "execute"));
    let result = client.call_tool("search", schema::IJsonValue::parse(br#"{"code":"async () => Object.entries(spec.paths).filter(([path]) => path === \"/accounts\").map(([path, methods]) => ({ path, methods: Object.keys(methods) }))"}"#).unwrap(), McpCancellationToken::default()).await.expect("read-only search");
    let result = serde_json::to_value(result).unwrap();
    assert_ne!(result["isError"], true);
    assert!(result["content"].as_array().is_some_and(|blocks| {
        blocks.iter().any(|block| {
            block["type"] == "text" && block["text"].as_str().is_some_and(|text| !text.is_empty())
        })
    }));
    client.close().await.unwrap();
}

#[tokio::test]
#[ignore = "calls public Cloudflare with automatic HTTP discovery"]
async fn live_cloudflare_docs_auto_discovery() {
    public_round_trip(
        "https://docs.mcp.cloudflare.com/mcp",
        ProtocolMode::Auto,
        "search_cloudflare_documentation",
        json!({"query":"MCP stateless server/discover"}),
    )
    .await;
}

#[tokio::test]
#[ignore = "explicit legacy pin diagnostic; does not replace the Auto parity gate"]
async fn live_deepwiki_explicit_legacy_pin() {
    public_round_trip(
        "https://mcp.deepwiki.com/mcp",
        ProtocolMode::Legacy,
        "read_wiki_structure",
        json!({"repoName":"modelcontextprotocol/specification"}),
    )
    .await;
}

#[tokio::test]
#[ignore = "read-only public CF catalog diagnostic"]
async fn live_cloudflare_docs_parameter_contract() {
    let transport = HttpTransport::new(
        "https://docs.mcp.cloudflare.com/mcp".parse().unwrap(),
        &BTreeMap::new(),
        None,
        false,
    )
    .unwrap();
    let mut client = McpClient::new("cfdocs", transport);
    client.connect(ProtocolMode::Auto).await.unwrap();
    let catalog = client.list_tools().await.unwrap();
    assert!(!catalog.is_empty());
    for tool in catalog {
        println!(
            "{} {}",
            tool.name,
            String::from_utf8(tool.input_schema.canonical_bytes().unwrap()).unwrap()
        );
    }
    client.close().await.unwrap();
}
