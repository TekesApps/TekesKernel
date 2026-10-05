use std::net::SocketAddr;

use axum::body::Body;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, HOST, ORIGIN, REFERRER_POLICY};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;

#[cfg(test)]
use sha2::{Digest, Sha256};

pub(crate) const INDEX_HTML: &[u8] = include_bytes!("../web/index.html");
pub(crate) const APP_CSS: &[u8] = include_bytes!("../web/app.css");
pub(crate) const APP_JS: &[u8] = include_bytes!("../web/app.js");

// Updated by `scripts/check-web-client-assets.py`, which also verifies the exact
// source-byte digest before packaging. Runtime tests independently recompute it.
pub const WEB_CLIENT_SHA256: &str =
    "0bde7b59311dab426b91cd65049e218fc7705a915eeb0271aca483b14f978a9b";

pub(crate) struct BrowserAccess {
    authority: String,
    origin: String,
}

impl BrowserAccess {
    pub(crate) fn new(bind: SocketAddr) -> Self {
        let authority = bind.to_string();
        Self {
            origin: format!("http://{authority}"),
            authority,
        }
    }

    pub(crate) fn permits_origin(&self, headers: &HeaderMap) -> bool {
        header(headers, ORIGIN).is_some_and(|value| value == self.origin)
    }

    pub(crate) fn permits_host(&self, headers: &HeaderMap) -> bool {
        let mut values = headers.get_all(HOST).iter();
        let Some(value) = values.next() else {
            return false;
        };
        values.next().is_none() && value.to_str().is_ok_and(|value| value == self.authority)
    }
}

#[cfg(test)]
pub(crate) fn asset_digest() -> String {
    let mut digest = Sha256::new();
    for (name, bytes) in [
        ("app.css", APP_CSS),
        ("app.js", APP_JS),
        ("index.html", INDEX_HTML),
    ] {
        digest.update(name.as_bytes());
        digest.update([0]);
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    format!("{:x}", digest.finalize())
}

pub(crate) fn static_response(content_type: &'static str, bytes: &'static [u8]) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .header("content-length", bytes.len().to_string())
        .header("x-content-type-options", "nosniff")
        .body(Body::from(bytes))
        .expect("embedded web asset response")
}

pub(crate) fn index_response() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/html; charset=utf-8")
        .header(CACHE_CONTROL, "no-store")
        .header(REFERRER_POLICY, "no-referrer")
        .header(
            "content-security-policy",
            "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data: blob:; connect-src 'self' ws:; font-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
        )
        .header("x-content-type-options", "nosniff")
        .header("x-frame-options", "DENY")
        .header("content-length", INDEX_HTML.len().to_string())
        .body(Body::from(INDEX_HTML))
        .expect("embedded web index response")
}

pub(crate) fn index_unauthorized() -> Response {
    const MESSAGE: &[u8] = b"Tekes Web Client rejected this Host.\n";
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(CACHE_CONTROL, "no-store")
        .header("content-length", MESSAGE.len().to_string())
        .body(Body::from(MESSAGE))
        .expect("browser Host rejection")
}

fn header(headers: &HeaderMap, name: axum::http::header::HeaderName) -> Option<&str> {
    let mut values = headers.get_all(name).iter();
    let value = values.next()?.to_str().ok()?;
    values.next().is_none().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::{WEB_CLIENT_SHA256, asset_digest};

    #[test]
    fn embedded_asset_digest_matches_generated_constant() {
        assert_eq!(asset_digest(), WEB_CLIENT_SHA256);
    }
}
