use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;
use tools::{
    BackendFailure, BackendGate, BackendOutcome, BoundedHttpClient, CancellationToken, HttpLimits,
    MAX_REDIRECTS, WEB_FETCH_ACCEPT, WEB_FETCH_MAX_CHARACTERS, WEB_FETCH_USER_AGENT,
    extract_web_content, is_public_internet_address, resolve_web_redirect,
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

#[test]
fn slice14e_gate_109_web_fetch_extraction_chain() {
    let fixture: Value = serde_json::from_slice(
        &std::fs::read(fixtures().join("web-tools/extraction.canonical.json")).expect("fixture"),
    )
    .expect("fixture JSON");
    for case in fixture["cases"].as_array().expect("cases") {
        let body = if let Some(value) = case.get("body").and_then(Value::as_str) {
            value.as_bytes().to_vec()
        } else {
            decode_hex(case["body_hex"].as_str().expect("body_hex"))
        };
        let content_type = case["content_type"].as_str();
        let actual = serde_json::to_value(extract_web_content(content_type, &body))
            .expect("serialize extraction");
        assert_eq!(actual, case["expected"], "case {}", case["id"]);
    }

    for boundary in fixture["generated_boundaries"]
        .as_array()
        .expect("boundaries")
    {
        let character = boundary["character"].as_str().expect("character");
        let count = boundary["input_characters"].as_u64().expect("count") as usize;
        let body = character.repeat(count);
        let actual = extract_web_content(Some("text/plain"), body.as_bytes());
        assert_eq!(actual.truncated, boundary["truncated"]);
        if let Some(suffix) = boundary.get("suffix").and_then(Value::as_str) {
            assert!(actual.text.ends_with(suffix));
            assert_eq!(
                actual.text.chars().take(WEB_FETCH_MAX_CHARACTERS).count(),
                24_000
            );
        } else {
            assert_eq!(actual.text, body);
        }
    }
}

#[test]
fn slice14e_gate_110_web_network_policy_and_bounds() {
    let fixture: Value = serde_json::from_slice(
        &std::fs::read(fixtures().join("web-tools/network-negative.canonical.json"))
            .expect("fixture"),
    )
    .expect("fixture JSON");
    for address in fixture["reject_ips"].as_array().expect("reject IPs") {
        let address: IpAddr = address.as_str().expect("IP string").parse().expect("IP");
        assert!(!is_public_internet_address(address), "accepted {address}");
    }
    assert!(is_public_internet_address("8.8.8.8".parse().unwrap()));
    assert!(is_public_internet_address(
        "2606:4700:4700::1111".parse().unwrap()
    ));

    assert_eq!(
        fixture["redirect_limit"].as_u64(),
        Some(MAX_REDIRECTS as u64)
    );
    assert_eq!(
        fixture["request_headers"]["user_agent"].as_str(),
        Some(WEB_FETCH_USER_AGENT)
    );
    assert_eq!(
        fixture["request_headers"]["accept"].as_str(),
        Some(WEB_FETCH_ACCEPT)
    );
    assert!(
        BoundedHttpClient::new(HttpLimits {
            timeout: Duration::ZERO,
            ..HttpLimits::default()
        })
        .is_err()
    );
    assert!(
        BoundedHttpClient::new(HttpLimits {
            max_bytes: 0,
            ..HttpLimits::default()
        })
        .is_err()
    );
    assert!(
        BoundedHttpClient::new(HttpLimits {
            max_redirects: MAX_REDIRECTS + 1,
            ..HttpLimits::default()
        })
        .is_err()
    );

    let client = BoundedHttpClient::new(HttpLimits::default()).expect("client");
    let cancellation = CancellationToken::default();
    cancellation.cancel();
    assert!(matches!(
        client.fetch("https://example.com", BackendGate::Ready, &cancellation),
        BackendOutcome::Completed(Err(BackendFailure::Cancelled))
    ));

    assert_eq!(
        resolve_web_redirect("https://example.com/a/b", "../next").unwrap(),
        "https://example.com/next"
    );
    assert_eq!(
        resolve_web_redirect("https://example.com/a", "http://127.0.0.1/private").unwrap(),
        "http://127.0.0.1/private"
    );
    assert!(resolve_web_redirect("https://example.com/a", "http://[").is_err());
    for host in fixture["reject_hosts"].as_array().expect("reject hosts") {
        let url = format!("https://{}/private", host.as_str().expect("host"));
        let outcome = client.fetch(&url, BackendGate::Ready, &CancellationToken::default());
        assert!(
            matches!(
                outcome,
                BackendOutcome::Completed(Err(BackendFailure::Denied(_)))
            ),
            "{url}: {outcome:?}"
        );
    }
    for url in [
        "http://127.0.0.1/private",
        "http://[::ffff:127.0.0.1]/private",
        "file:///etc/passwd",
    ] {
        let outcome = client.fetch(url, BackendGate::Ready, &CancellationToken::default());
        assert!(
            matches!(
                outcome,
                BackendOutcome::Completed(Err(BackendFailure::Denied(_)))
            ),
            "{url}: {outcome:?}"
        );
    }
}

fn decode_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("hex UTF-8");
            u8::from_str_radix(text, 16).expect("hex byte")
        })
        .collect()
}

/// A fake-IP VPN answers `198.18.0.0/15` for every name. Direct egress must
/// still refuse that answer; egress the environment routes through a proxy
/// never consults local DNS, so the placeholder answer is not in the path.
#[test]
fn web_egress_routes_through_the_environment_proxy_without_pinning() {
    use tools::{PublicRoute, route_public_url_with};
    let env = |vars: &'static [(&'static str, &'static str)]| {
        move |name: &str| {
            vars.iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        }
    };
    let url = reqwest::Url::parse("https://www.jalan.net/").unwrap();
    let proxied = env(&[("HTTPS_PROXY", "http://127.0.0.1:1082")]);
    assert_eq!(
        route_public_url_with(&url, proxied).unwrap(),
        PublicRoute::Proxied {
            proxy: reqwest::Url::parse("http://127.0.0.1:1082").unwrap()
        }
    );
    // The scheme's own variable wins over ALL_PROXY; lowercase is the fallback.
    let layered = env(&[
        ("all_proxy", "http://all.example:1"),
        ("https_proxy", "http://https.example:2"),
    ]);
    assert!(matches!(
        route_public_url_with(&url, layered).unwrap(),
        PublicRoute::Proxied { proxy } if proxy.host_str() == Some("https.example")
    ));
    let http_url = reqwest::Url::parse("http://www.jalan.net/").unwrap();
    let http_side = env(&[("https_proxy", "http://https.example:2")]);
    // No proxy for http: → direct, which needs DNS; a resolution failure is
    // acceptable here, a proxied route is not.
    assert!(!matches!(
        route_public_url_with(&http_url, http_side),
        Ok(PublicRoute::Proxied { .. })
    ));

    // NO_PROXY exemptions: suffix, wildcard-prefixed, CIDR against literals, `*`.
    let exempt = env(&[
        ("HTTPS_PROXY", "http://127.0.0.1:1082"),
        ("NO_PROXY", "*.jalan.net, 8.8.0.0/16"),
    ]);
    assert!(!matches!(
        route_public_url_with(&url, exempt),
        Ok(PublicRoute::Proxied { .. })
    ));
    let literal = reqwest::Url::parse("https://8.8.8.8/").unwrap();
    assert_eq!(
        route_public_url_with(&literal, exempt).unwrap(),
        PublicRoute::Direct {
            host: "8.8.8.8".to_owned(),
            address: "8.8.8.8:443".parse().unwrap()
        }
    );
    let other_literal = reqwest::Url::parse("https://1.1.1.1/").unwrap();
    assert!(matches!(
        route_public_url_with(&other_literal, exempt).unwrap(),
        PublicRoute::Proxied { .. }
    ));
    let star = env(&[("HTTPS_PROXY", "http://127.0.0.1:1082"), ("no_proxy", "*")]);
    assert!(matches!(
        route_public_url_with(&other_literal, star).unwrap(),
        PublicRoute::Direct { .. }
    ));

    // Structural refusals hold on the proxied route too: the proxy would
    // otherwise be a way to reach loopback and link-local services.
    for blocked in [
        "https://127.0.0.1/",
        "https://[::1]/",
        "https://169.254.169.254/latest/meta-data",
        "https://localhost/",
        "https://printer.local/",
        "https://user:pw@example.com/",
        "ftp://example.com/",
    ] {
        let blocked = reqwest::Url::parse(blocked).unwrap();
        assert!(
            route_public_url_with(&blocked, proxied).is_err(),
            "accepted {blocked}"
        );
    }

    // A proxy this build cannot use (SOCKS, PAC, garbage) is treated as unset.
    for unusable in [
        "socks5://127.0.0.1:1080",
        "pac+http://x/proxy.pac",
        "not a url",
    ] {
        let route = route_public_url_with(&literal, |name: &str| {
            (name == "HTTPS_PROXY").then(|| unusable.to_owned())
        });
        assert!(
            !matches!(route, Ok(PublicRoute::Proxied { .. })),
            "proxied via {unusable}"
        );
    }
}
