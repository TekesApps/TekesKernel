use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::redirect::Policy;
use thiserror::Error;

use crate::{
    AdapterId, DialectId, PreparedRequest, ProviderCompletion, ProviderFailure, ProviderFrame,
    ProviderStreamDecoder, normalize_dialect_response, normalize_response,
};

const MAX_RESPONSE: usize = 64 * 1024 * 1024;
const RESPONSE_IDLE: Duration = Duration::from_secs(120);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpStatusClass {
    Success,
    AuthFailure,
    RecoverableTransport,
    ProviderTerminalCandidate,
}

#[must_use]
/// Cloudflare AI Gateway wholesale admission limit: HTTP 402 carrying the
/// gateway's "Wholesale rate limit exceeded" message. It is an admission
/// limit on the gateway line, not a provider verdict on the request, so it is
/// retried under the rate-limit budget instead of settling the turn.
pub fn wholesale_limited(status: u16, body: &str) -> bool {
    status == 402 && body.to_ascii_lowercase().contains("wholesale rate limit")
}

pub fn classify_status(status: u16) -> HttpStatusClass {
    match status {
        200..=299 => HttpStatusClass::Success,
        401 | 403 => HttpStatusClass::AuthFailure,
        408 | 425 | 429 | 500..=599 => HttpStatusClass::RecoverableTransport,
        _ => HttpStatusClass::ProviderTerminalCandidate,
    }
}

#[derive(Debug, Error)]
pub enum HttpRuntimeError {
    #[error("provider request could not be built: {0}")]
    Request(String),
    #[error("provider response exceeded 64 MiB")]
    ResponseTooLarge,
    #[error("provider stream is malformed: {0}")]
    Malformed(String),
}

pub struct HttpRuntime {
    client: Client,
    runtime: tokio::runtime::Runtime,
    response_capture: Option<Arc<std::sync::Mutex<Vec<u8>>>>,
}

impl HttpRuntime {
    pub fn new() -> Result<Self, HttpRuntimeError> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .redirect(Policy::none())
            .build()
            .map_err(|error| HttpRuntimeError::Request(error.to_string()))?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .map_err(|error| HttpRuntimeError::Request(error.to_string()))?;
        Ok(Self {
            client,
            runtime,
            response_capture: None,
        })
    }

    /// Opt-in diagnostic body capture, after credential-echo checks and before
    /// decoding. No request headers or credentials are copied to this buffer.
    #[doc(hidden)]
    pub fn with_response_capture(mut self, capture: Arc<std::sync::Mutex<Vec<u8>>>) -> Self {
        self.response_capture = Some(capture);
        self
    }

    pub fn send(
        &self,
        adapter: AdapterId,
        prepared: &PreparedRequest,
        credential: &str,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<ProviderCompletion, HttpRuntimeError> {
        self.send_with_frames(adapter, prepared, credential, cancelled, |_| Ok(()))
    }

    pub fn send_with_frames(
        &self,
        adapter: AdapterId,
        prepared: &PreparedRequest,
        credential: &str,
        cancelled: &Arc<AtomicBool>,
        mut on_frame: impl FnMut(ProviderFrame) -> Result<(), HttpRuntimeError>,
    ) -> Result<ProviderCompletion, HttpRuntimeError> {
        self.send_with_frames_and_wall(
            adapter,
            prepared,
            credential,
            cancelled,
            None,
            &mut on_frame,
        )
    }

    pub fn send_with_frames_and_wall(
        &self,
        adapter: AdapterId,
        prepared: &PreparedRequest,
        credential: &str,
        cancelled: &Arc<AtomicBool>,
        wall: Option<Duration>,
        mut on_frame: impl FnMut(ProviderFrame) -> Result<(), HttpRuntimeError>,
    ) -> Result<ProviderCompletion, HttpRuntimeError> {
        self.runtime.block_on(self.send_async(
            adapter,
            None,
            prepared,
            None,
            credential,
            cancelled,
            wall,
            &mut on_frame,
        ))
    }

    /// Hermetic connector seam. The prepared request, digest identity, HTTP
    /// origin-form path and Host header remain those of the proved route; only
    /// the socket destination is replaced by a loopback HTTP endpoint.
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn send_dialect_with_frames_and_wall_transport(
        &self,
        dialect: DialectId,
        prepared: &PreparedRequest,
        transport_endpoint: Option<&str>,
        credential: &str,
        cancelled: &Arc<AtomicBool>,
        wall: Option<Duration>,
        mut on_frame: impl FnMut(ProviderFrame) -> Result<(), HttpRuntimeError>,
    ) -> Result<ProviderCompletion, HttpRuntimeError> {
        self.runtime.block_on(self.send_async(
            dialect.family(),
            Some(dialect),
            prepared,
            transport_endpoint,
            credential,
            cancelled,
            wall,
            &mut on_frame,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_async(
        &self,
        adapter: AdapterId,
        dialect: Option<DialectId>,
        prepared: &PreparedRequest,
        transport_endpoint: Option<&str>,
        credential: &str,
        cancelled: &Arc<AtomicBool>,
        wall: Option<Duration>,
        on_frame: &mut impl FnMut(ProviderFrame) -> Result<(), HttpRuntimeError>,
    ) -> Result<ProviderCompletion, HttpRuntimeError> {
        if let Some(capture) = &self.response_capture {
            capture
                .lock()
                .map_err(|_| HttpRuntimeError::Request("response capture lock poisoned".into()))?
                .clear();
        }
        if cancelled.load(Ordering::Acquire) {
            return Ok(ProviderCompletion::Failure(ProviderFailure::Cancelled));
        }
        let wall_deadline = wall.map(|duration| tokio::time::Instant::now() + duration);
        let method = prepared
            .method
            .parse()
            .map_err(|error| HttpRuntimeError::Request(format!("invalid method: {error}")))?;
        let (request_url, logical_host) = transport_endpoint.map_or_else(
            || Ok((prepared.url.clone(), None)),
            |endpoint| transport_request_url(&prepared.url, endpoint),
        )?;
        let mut request = self.client.request(method, request_url);
        if let Some(host) = logical_host {
            request = request.header(reqwest::header::HOST, host);
        }
        for (name, value) in &prepared.headers_without_secret {
            request = request.header(name, value);
        }
        if !credential.is_empty() {
            request = request.header(
                &prepared.credential_header,
                format!("{}{credential}", prepared.credential_prefix),
            );
        }
        request = request.body(prepared.body.clone());
        let header_wait = wall
            .map(|duration| duration.min(RESPONSE_IDLE))
            .unwrap_or(RESPONSE_IDLE);
        let response = match wait_with_cancellation(request.send(), cancelled, header_wait).await {
            WaitResult::Value(Ok(response)) => response,
            WaitResult::Cancelled => {
                return Ok(ProviderCompletion::Failure(ProviderFailure::Cancelled));
            }
            WaitResult::TimedOut => {
                return Ok(ProviderCompletion::Failure(ProviderFailure::Transport {
                    status: None,
                    detail: "provider response header timed out".to_owned(),
                    partial_fragments: None,
                }));
            }
            WaitResult::Value(Err(error)) => {
                let failure = if cancelled.load(Ordering::Acquire) {
                    ProviderFailure::Cancelled
                } else if error.is_builder() {
                    ProviderFailure::Malformed {
                        detail: "provider request could not be constructed".to_owned(),
                    }
                } else {
                    ProviderFailure::Transport {
                        status: error.status().map(|status| status.as_u16()),
                        detail: transport_detail(&error, credential),
                        partial_fragments: None,
                    }
                };
                return Ok(ProviderCompletion::Failure(failure));
            }
        };
        let status = response.status();
        if !status.is_success() {
            let retry_after_seconds = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok());
            const MAX_FAILURE_BODY: usize = 16 * 1024;
            let mut failure_body = Vec::new();
            let mut body = response.bytes_stream();
            while failure_body.len() < MAX_FAILURE_BODY {
                let wait = wall
                    .map(|duration| duration.min(RESPONSE_IDLE))
                    .unwrap_or(RESPONSE_IDLE);
                match wait_with_cancellation(body.next(), cancelled, wait).await {
                    WaitResult::Value(Some(Ok(chunk))) => {
                        let remaining = MAX_FAILURE_BODY - failure_body.len();
                        failure_body.extend_from_slice(&chunk[..chunk.len().min(remaining)]);
                    }
                    WaitResult::Value(None)
                    | WaitResult::Value(Some(Err(_)))
                    | WaitResult::TimedOut => break,
                    WaitResult::Cancelled => {
                        return Ok(ProviderCompletion::Failure(ProviderFailure::Cancelled));
                    }
                }
            }
            if let Some(capture) = &self.response_capture {
                let mut captured = capture.lock().map_err(|_| {
                    HttpRuntimeError::Request("response capture lock poisoned".into())
                })?;
                if credential.is_empty()
                    || !failure_body
                        .windows(credential.len())
                        .any(|bytes| bytes == credential.as_bytes())
                {
                    captured.extend_from_slice(&failure_body);
                }
            }
            let body = String::from_utf8_lossy(&failure_body).into_owned();
            let detail = bounded_detail(&body, credential);
            return Ok(ProviderCompletion::Failure(
                match classify_status(status.as_u16()) {
                    HttpStatusClass::AuthFailure => ProviderFailure::AuthFailure {
                        status: status.as_u16(),
                        detail: (!detail.is_empty()).then_some(detail),
                    },
                    HttpStatusClass::RecoverableTransport
                        if status == StatusCode::TOO_MANY_REQUESTS =>
                    {
                        ProviderFailure::RateLimited {
                            status: status.as_u16(),
                            detail: (!detail.is_empty()).then_some(detail),
                            retry_after_seconds,
                        }
                    }
                    HttpStatusClass::ProviderTerminalCandidate
                        if wholesale_limited(status.as_u16(), &body) =>
                    {
                        ProviderFailure::RateLimited {
                            status: status.as_u16(),
                            detail: (!detail.is_empty()).then_some(detail),
                            retry_after_seconds,
                        }
                    }
                    HttpStatusClass::ProviderTerminalCandidate if status.is_redirection() => {
                        ProviderFailure::Malformed {
                            detail: format!(
                                "provider returned disallowed redirect HTTP {}",
                                status.as_u16()
                            ),
                        }
                    }
                    HttpStatusClass::ProviderTerminalCandidate => {
                        let redacted;
                        let normalization_body =
                            if !credential.is_empty() && body.contains(credential) {
                                redacted = body.replace(credential, "[REDACTED]").into_bytes();
                                redacted.as_slice()
                            } else {
                                failure_body.as_slice()
                            };
                        let normalized = dialect.map_or_else(
                            || normalize_response(adapter, normalization_body),
                            |dialect| normalize_dialect_response(dialect, normalization_body),
                        );
                        return Ok(match normalized {
                            Ok(mut terminal) => {
                                terminal.http_status = Some(status.as_u16());
                                if !matches!(
                                    terminal.finish_reason,
                                    crate::FinishReason::ContextOverflow
                                        | crate::FinishReason::ContentFilter
                                        | crate::FinishReason::ProviderError
                                ) {
                                    terminal.finish_reason = crate::FinishReason::ProviderError;
                                }
                                ProviderCompletion::Terminal(terminal)
                            }
                            Err(error) => ProviderCompletion::Failure(ProviderFailure::Malformed {
                                detail: format!(
                                    "http_{}: {}; body: {}",
                                    status.as_u16(),
                                    bounded_detail(&error, credential),
                                    detail
                                ),
                            }),
                        });
                    }
                    HttpStatusClass::RecoverableTransport => ProviderFailure::Transport {
                        status: Some(status.as_u16()),
                        detail: if detail.is_empty() {
                            format!("provider returned HTTP {}", status.as_u16())
                        } else {
                            detail
                        },
                        partial_fragments: None,
                    },
                    HttpStatusClass::Success => {
                        unreachable!("non-success response classified success")
                    }
                },
            ));
        }
        let is_sse = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.split(';').next() == Some("text/event-stream"));
        let mut stream = is_sse.then(|| {
            dialect.map_or_else(
                || ProviderStreamDecoder::new(adapter),
                ProviderStreamDecoder::for_dialect,
            )
        });
        let mut bytes = Vec::new();
        let mut since_frame = Instant::now();
        let secret = credential.as_bytes();
        let mut secret_tail = Vec::new();
        let mut body = response.bytes_stream();
        loop {
            if cancelled.load(Ordering::Acquire) {
                return Ok(ProviderCompletion::Failure(ProviderFailure::Cancelled));
            }
            let idle_remaining = RESPONSE_IDLE.saturating_sub(since_frame.elapsed());
            let wall_remaining = wall_deadline
                .map(|deadline| deadline.saturating_duration_since(tokio::time::Instant::now()));
            let wait = wall_remaining
                .map(|remaining| remaining.min(idle_remaining))
                .unwrap_or(idle_remaining);
            // A transport loss mid-body keeps the items the stream had already
            // completed: the worker may have dispatched a call from them and
            // the retry must replay that call behind its native predecessors.
            let partial_fragments = |stream: &Option<ProviderStreamDecoder>| {
                stream
                    .as_ref()
                    .and_then(ProviderStreamDecoder::completed_fragments)
            };
            if wait.is_zero() {
                return Ok(ProviderCompletion::Failure(ProviderFailure::Transport {
                    status: None,
                    detail: "provider response exceeded its idle or wall deadline".to_owned(),
                    partial_fragments: partial_fragments(&stream),
                }));
            }
            let chunk = match wait_with_cancellation(body.next(), cancelled, wait).await {
                WaitResult::Value(Some(Ok(chunk))) => chunk,
                WaitResult::Value(None) => break,
                WaitResult::Value(Some(Err(error))) => {
                    return Ok(ProviderCompletion::Failure(ProviderFailure::Transport {
                        status: None,
                        detail: transport_detail(&error, credential),
                        partial_fragments: partial_fragments(&stream),
                    }));
                }
                WaitResult::TimedOut => {
                    return Ok(ProviderCompletion::Failure(ProviderFailure::Transport {
                        status: None,
                        detail: "provider response stream timed out".to_owned(),
                        partial_fragments: partial_fragments(&stream),
                    }));
                }
                WaitResult::Cancelled => {
                    return Ok(ProviderCompletion::Failure(ProviderFailure::Cancelled));
                }
            };
            if bytes.len().saturating_add(chunk.len()) > MAX_RESPONSE {
                return Ok(ProviderCompletion::Failure(ProviderFailure::Malformed {
                    detail: "provider response exceeded the byte limit".to_owned(),
                }));
            }
            if !secret.is_empty() && contains_secret(&mut secret_tail, &chunk, secret) {
                if let Some(capture) = &self.response_capture {
                    capture
                        .lock()
                        .map_err(|_| {
                            HttpRuntimeError::Request("response capture lock poisoned".into())
                        })?
                        .clear();
                }
                return Ok(ProviderCompletion::Failure(ProviderFailure::Malformed {
                    detail: "provider response contained credential material".to_owned(),
                }));
            }
            bytes.extend_from_slice(&chunk);
            if let Some(capture) = &self.response_capture {
                capture
                    .lock()
                    .map_err(|_| {
                        HttpRuntimeError::Request("response capture lock poisoned".into())
                    })?
                    .extend_from_slice(&chunk);
            }
            if let Some(decoder) = &mut stream {
                let before = decoder.completed_events();
                let frames = match decoder.push(&chunk) {
                    Ok(frames) => frames,
                    Err(error) => {
                        return Ok(ProviderCompletion::Failure(ProviderFailure::Malformed {
                            detail: bounded_detail(
                                &format!("provider event stream was malformed: {error}"),
                                credential,
                            ),
                        }));
                    }
                };
                if decoder.completed_events() != before {
                    since_frame = Instant::now();
                }
                for frame in frames {
                    on_frame(frame)?;
                }
            }
        }
        if let Some(decoder) = stream {
            let (frames, terminal) = match decoder.finish() {
                Ok(terminal) => terminal,
                Err(error) => {
                    return Ok(ProviderCompletion::Failure(ProviderFailure::Malformed {
                        detail: bounded_detail(
                            &format!(
                                "provider event stream ended with an incomplete frame: {error}"
                            ),
                            credential,
                        ),
                    }));
                }
            };
            for frame in frames {
                on_frame(frame)?;
            }
            return Ok(ProviderCompletion::Terminal(terminal));
        }
        let normalized = dialect.map_or_else(
            || normalize_response(adapter, &bytes),
            |dialect| normalize_dialect_response(dialect, &bytes),
        );
        match normalized {
            Ok(terminal) => Ok(ProviderCompletion::Terminal(terminal)),
            Err(error) => Ok(ProviderCompletion::Failure(ProviderFailure::Malformed {
                detail: bounded_detail(&error, credential),
            })),
        }
    }
}

fn transport_detail(error: &reqwest::Error, secret: &str) -> String {
    use std::error::Error;
    // reqwest's Display often contains only the request URL. Retain the
    // connection/TLS/IO cause so a live transport failure can be diagnosed.
    let mut causes = Vec::new();
    let mut source = error.source();
    while let Some(cause) = source {
        causes.push(cause.to_string());
        if causes.len() == 8 {
            break;
        }
        source = cause.source();
    }
    let kind = if error.is_timeout() {
        "timeout"
    } else if error.is_connect() {
        "connect"
    } else {
        "transport"
    };
    bounded_detail(&format!("{kind}: {}; {error}", causes.join(": ")), secret)
}

fn bounded_detail(value: &str, secret: &str) -> String {
    let mut sanitized = value
        .replace(['\r', '\n', '\t'], " ")
        .replace(char::is_control, "");
    if !secret.is_empty() {
        sanitized = sanitized.replace(secret, "[redacted]");
    }
    let mut end = sanitized.len().min(512);
    while !sanitized.is_char_boundary(end) {
        end -= 1;
    }
    sanitized[..end].to_owned()
}

fn transport_request_url(
    logical_url: &str,
    endpoint: &str,
) -> Result<(String, Option<String>), HttpRuntimeError> {
    let logical = reqwest::Url::parse(logical_url)
        .map_err(|error| HttpRuntimeError::Request(format!("invalid logical URL: {error}")))?;
    let mut transport = reqwest::Url::parse(endpoint).map_err(|error| {
        HttpRuntimeError::Request(format!("invalid transport endpoint: {error}"))
    })?;
    if transport.scheme() != "http" || transport.host_str() != Some("127.0.0.1") {
        return Err(HttpRuntimeError::Request(
            "transport override must be an http://127.0.0.1 endpoint".to_owned(),
        ));
    }
    transport.set_path(logical.path());
    transport.set_query(logical.query());
    transport.set_fragment(None);
    let host = logical
        .host_str()
        .ok_or_else(|| HttpRuntimeError::Request("logical URL lacks host".to_owned()))?;
    let authority = match logical.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_owned(),
    };
    Ok((transport.to_string(), Some(authority)))
}

enum WaitResult<T> {
    Value(T),
    Cancelled,
    TimedOut,
}

async fn wait_with_cancellation<T>(
    future: impl std::future::Future<Output = T>,
    cancelled: &AtomicBool,
    timeout: Duration,
) -> WaitResult<T> {
    tokio::pin!(future);
    let deadline = tokio::time::sleep(timeout);
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            value = &mut future => return WaitResult::Value(value),
            () = &mut deadline => return WaitResult::TimedOut,
            () = tokio::time::sleep(Duration::from_millis(25)) => {
                if cancelled.load(Ordering::Acquire) {
                    return WaitResult::Cancelled;
                }
            }
        }
    }
}

fn contains_secret(tail: &mut Vec<u8>, chunk: &[u8], secret: &[u8]) -> bool {
    if secret.is_empty() {
        return false;
    }
    let mut candidate = Vec::with_capacity(tail.len() + chunk.len());
    candidate.extend_from_slice(tail);
    candidate.extend_from_slice(chunk);
    let found = candidate
        .windows(secret.len())
        .any(|window| window == secret);
    let keep = secret.len().saturating_sub(1).min(candidate.len());
    tail.clear();
    tail.extend_from_slice(&candidate[candidate.len() - keep..]);
    found
}

impl Default for HttpRuntime {
    fn default() -> Self {
        Self::new().expect("static reqwest client configuration is valid")
    }
}

#[cfg(test)]
mod wholesale_tests {
    use super::{HttpStatusClass, classify_status, wholesale_limited};

    #[test]
    fn cloudflare_wholesale_402_is_an_admission_limit_not_a_terminal_verdict() {
        let body = r#"{"error":{"code":"invalid_prompt","message":"Wholesale rate limit exceeded for this gateway. Please reduce request rate or use BYOK."}}"#;
        assert_eq!(
            classify_status(402),
            HttpStatusClass::ProviderTerminalCandidate
        );
        assert!(wholesale_limited(402, body));
        assert!(!wholesale_limited(
            402,
            r#"{"error":{"message":"payment required"}}"#
        ));
        assert!(!wholesale_limited(400, body));
        assert!(!wholesale_limited(429, body));
    }
}
