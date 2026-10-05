use profile::ProvidersConfig;

#[test]
fn independent_web_search_config_round_trips_and_private_origins_reject() {
    let canonical = include_bytes!("../../../fixtures/config/providers-web-search.canonical.json");
    let decoded = ProvidersConfig::decode(canonical).expect("web-search config");
    let search = decoded.web_search.as_ref().expect("web_search");
    assert_eq!(search.adapter, "tavily_v1");
    assert_eq!(search.endpoint, "https://api.tavily.com");
    assert_eq!(
        decoded.canonical_bytes().expect("canonical bytes"),
        canonical
    );
    assert!(
        ProvidersConfig::decode(include_bytes!(
            "../../../fixtures/config/providers-web-search-private.invalid.json"
        ))
        .is_err()
    );
    assert!(
        ProvidersConfig::decode(include_bytes!(
            "../../../fixtures/config/providers-web-search-mapped-loopback.invalid.json"
        ))
        .is_err()
    );
}
