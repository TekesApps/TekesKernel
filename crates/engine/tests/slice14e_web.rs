use engine::web_fetch_result_value;
use serde_json::json;
use tools::{HttpFetchResult, WebExtraction, WebExtractionKind};

#[test]
fn slice14e_gate_109_web_fetch_result_is_closed_and_drops_raw_body() {
    let result = web_fetch_result_value(HttpFetchResult {
        final_url: "https://example.com/docs".to_owned(),
        status: 200,
        content_type: Some("text/html".to_owned()),
        body: b"raw-secret-that-must-not-be-durable".to_vec(),
        extraction: WebExtraction {
            kind: WebExtractionKind::Html,
            text: "Readable docs".to_owned(),
            under_rendered: false,
            truncated: false,
        },
    });
    assert_eq!(
        result,
        json!({
            "final_url":"https://example.com/docs",
            "status":200,
            "content_type":"text/html",
            "bytes":35,
            "text":"Readable docs",
            "extraction":{"kind":"html","under_rendered":false,"truncated":false}
        })
    );
    assert!(!result.to_string().contains("raw-secret"));
}
