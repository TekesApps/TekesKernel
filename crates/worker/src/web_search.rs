use super::*;

pub(crate) struct TavilySearchProvider {
    pub(crate) config: WebSearch,
    pub(crate) credential: Arc<Mutex<CredentialClient>>,
    pub(crate) transport: Arc<dyn TavilyTransport>,
}

pub(crate) trait TavilyTransport: Send + Sync {
    fn send(
        &self,
        endpoint: &str,
        credential: &str,
        request: &SearchRequest,
        limits: &HttpLimits,
        cancellation: &CancellationToken,
    ) -> Result<Vec<SearchHit>, BackendFailure>;
}

pub(crate) struct ProductionTavilyTransport;

impl TavilyTransport for ProductionTavilyTransport {
    fn send(
        &self,
        endpoint: &str,
        credential: &str,
        request: &SearchRequest,
        limits: &HttpLimits,
        cancellation: &CancellationToken,
    ) -> Result<Vec<SearchHit>, BackendFailure> {
        send_tavily_search(endpoint, credential, request, limits, cancellation)
    }
}

impl SearchProvider for TavilySearchProvider {
    fn search(
        &self,
        request: &SearchRequest,
        limits: &HttpLimits,
        cancellation: &CancellationToken,
    ) -> Result<Vec<SearchHit>, BackendFailure> {
        if self.config.adapter != "tavily_v1" {
            return Err(BackendFailure::TerminalUnavailable(
                "unsupported web-search adapter".to_owned(),
            ));
        }
        let origin = endpoint_origin(&self.config.endpoint)
            .map_err(|_| BackendFailure::Protocol("invalid web-search endpoint".to_owned()))?;
        let attempt = format!("tool:{}", request.call_id);
        let get = CredentialGet {
            request_id: credential_request_id(&attempt, &self.config.credential_key, &origin),
            attempt,
            credential_id: self.config.credential_key.clone(),
            purpose: "web_search".to_owned(),
            adapter: "tavily_v1".to_owned(),
            endpoint_origin: origin,
        };
        // Material is resolved per call and lives only for this send; it is
        // zeroed when `material` drops at the end of the call.
        let material = self
            .credential
            .lock()
            .map_err(|_| BackendFailure::Unavailable("credential client lock poisoned".to_owned()))?
            .get(get)
            .map_err(credential_backend_failure)?;
        self.transport.send(
            &self.config.endpoint,
            &material.material,
            request,
            limits,
            cancellation,
        )
    }
}

fn credential_backend_failure(error: provider::CredentialClientError) -> BackendFailure {
    match error {
        provider::CredentialClientError::Rejected(code)
            if matches!(code.as_str(), "not_found" | "revoked" | "unavailable") =>
        {
            BackendFailure::Unavailable(format!("web-search credential {code}"))
        }
        provider::CredentialClientError::Rejected(code) => {
            BackendFailure::Protocol(format!("web-search credential broker rejected: {code}"))
        }
        other => BackendFailure::Unavailable(format!("web-search credential broker: {other}")),
    }
}

fn send_tavily_search(
    endpoint: &str,
    credential: &str,
    request: &SearchRequest,
    limits: &HttpLimits,
    cancellation: &CancellationToken,
) -> Result<Vec<SearchHit>, BackendFailure> {
    let (url, route) = resolve_public_search_endpoint(endpoint)?;
    let client = route
        .client_builder()?
        .connect_timeout(limits.timeout.min(Duration::from_secs(30)))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|error| BackendFailure::Unavailable(error.to_string()))?;
    let body = tavily_request_bytes(request)?;
    let cancellation = cancellation.clone();
    let timeout = limits.timeout;
    let max_bytes = limits.max_bytes;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| BackendFailure::Unavailable(error.to_string()))?;
    runtime.block_on(async move {
        let operation = async {
            let response = tokio::select! {
                response = client
                    .post(url)
                    .header("accept", "application/json")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {credential}"))
                    .body(body)
                    .send() => response.map_err(|error| BackendFailure::Unavailable(error.to_string()))?,
                () = wait_search_cancelled(&cancellation) => return Err(BackendFailure::Cancelled),
            };
            let status = response.status().as_u16();
            classify_tavily_status(status)?;
            if response.content_length().is_some_and(|length| length > max_bytes as u64) {
                return Err(BackendFailure::Limit("web-search response exceeds byte cap".to_owned()));
            }
            let mut response = response;
            let mut bytes = Vec::new();
            loop {
                let chunk = tokio::select! {
                    chunk = response.chunk() => chunk.map_err(|error| BackendFailure::Unavailable(error.to_string()))?,
                    () = wait_search_cancelled(&cancellation) => return Err(BackendFailure::Cancelled),
                };
                let Some(chunk) = chunk else { break };
                if bytes.len().saturating_add(chunk.len()) > max_bytes {
                    return Err(BackendFailure::Limit("web-search response exceeds byte cap".to_owned()));
                }
                bytes.extend_from_slice(&chunk);
            }
            parse_tavily_response(&bytes, request.max_results)
        };
        tokio::time::timeout(timeout, operation)
            .await
            .map_err(|_| BackendFailure::Timeout)?
    })
}

pub(crate) fn classify_tavily_status(status: u16) -> Result<(), BackendFailure> {
    match status {
        200..=299 => Ok(()),
        400 => Err(BackendFailure::Invalid(
            "web-search provider rejected request".to_owned(),
        )),
        401 | 403 => Err(BackendFailure::Denied(
            "web-search authentication failed".to_owned(),
        )),
        429 | 500..=599 => Err(BackendFailure::Unavailable(format!(
            "web-search HTTP {status}"
        ))),
        _ => Err(BackendFailure::TerminalUnavailable(format!(
            "web-search HTTP {status}"
        ))),
    }
}

pub(crate) fn tavily_request_bytes(request: &SearchRequest) -> Result<Vec<u8>, BackendFailure> {
    serde_json_canonicalizer::to_vec(&json!({
        "query": request.query,
        "search_depth": "basic",
        "max_results": request.max_results,
        "topic": match request.topic { SearchTopic::General => "general", SearchTopic::News => "news" },
        "include_answer": false,
        "include_raw_content": false,
        "include_images": false
    }))
    .map_err(|error| BackendFailure::Protocol(error.to_string()))
}

async fn wait_search_cancelled(cancellation: &CancellationToken) {
    while !cancellation.is_cancelled() {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// The search endpoint shares `web_fetch`'s egress boundary: pinned to a
/// classified DNS answer, or tunnelled through the outbound proxy the
/// environment names, never resolved twice.
pub(crate) fn resolve_public_search_endpoint(
    endpoint: &str,
) -> Result<(reqwest::Url, PublicRoute), BackendFailure> {
    let mut url = reqwest::Url::parse(endpoint)
        .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
    url.set_path("/search");
    if url.host_str().is_none() {
        return Err(BackendFailure::Protocol(
            "web-search endpoint lacks host".to_owned(),
        ));
    }
    let route = route_public_url(&url).map_err(|failure| match failure {
        BackendFailure::Denied(_) => BackendFailure::Denied(
            "web-search endpoint did not resolve exclusively to public addresses".to_owned(),
        ),
        other => other,
    })?;
    Ok((url, route))
}

pub(crate) fn parse_tavily_response(
    bytes: &[u8],
    max_results: u8,
) -> Result<Vec<SearchHit>, BackendFailure> {
    let checked = IJsonValue::parse(bytes)
        .map_err(|error| BackendFailure::Protocol(format!("invalid web-search JSON: {error}")))?;
    let value = serde_json::to_value(checked)
        .map_err(|error| BackendFailure::Protocol(error.to_string()))?;
    let results = value
        .as_object()
        .and_then(|object| object.get("results"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            BackendFailure::Protocol("web-search response lacks results array".to_owned())
        })?;
    let mut hits = results
        .iter()
        .map(|entry| {
            let object = entry.as_object().ok_or_else(|| {
                BackendFailure::Protocol("web-search result is not an object".to_owned())
            })?;
            let field = |name| {
                object.get(name).and_then(Value::as_str).ok_or_else(|| {
                    BackendFailure::Protocol(format!("web-search result lacks string {name}"))
                })
            };
            let title = field("title")?.to_owned();
            let url = field("url")?.to_owned();
            validate_public_result_url(&url)?;
            Ok(SearchHit {
                title,
                url,
                snippet: field("content")?.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    hits.truncate(usize::from(max_results));
    Ok(hits)
}

pub(crate) fn validate_public_result_url(value: &str) -> Result<(), BackendFailure> {
    let url = reqwest::Url::parse(value)
        .map_err(|error| BackendFailure::Protocol(format!("invalid result URL: {error}")))?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(BackendFailure::Protocol(
            "web-search result URL is not a public HTTP URL".to_owned(),
        ));
    }
    let host = url
        .host_str()
        .ok_or_else(|| BackendFailure::Protocol("web-search result URL lacks host".to_owned()))?;
    let address_literal = host
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(host);
    let lower_host = host.to_ascii_lowercase();
    if lower_host == "localhost"
        || lower_host.ends_with(".localhost")
        || lower_host.ends_with(".local")
        || lower_host.ends_with(".internal")
        || address_literal
            .parse::<IpAddr>()
            .is_ok_and(|address| !is_public_internet_address(address))
    {
        return Err(BackendFailure::Protocol(
            "web-search result URL is not public".to_owned(),
        ));
    }
    Ok(())
}
