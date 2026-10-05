//! Host-wide search over visible conversation text, without provider/tool internals.
use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSearchHit {
    pub session_id: String,
    pub snippet: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSearchResults {
    pub items: Vec<SessionSearchHit>,
    pub has_more: bool,
}

impl ThreadSearchAuthority {
    /// Includes active and archived sessions across workspaces. Results are stable
    /// by session identity and capped at 50; no credentials or tool payloads are searched.
    pub fn search_sessions(&self, query: &str) -> Result<SessionSearchResults, SearchError> {
        validate_request(&SearchRequest {
            workspace_id: "host".into(),
            query: query.into(),
            limit: MAX_PAGE_SIZE,
            visibility: ArchiveVisibility::All,
            after: None,
        })?;
        let query = normalize(query);
        let _search = NamedLock::exclusive(self.store.root().join(".thread-search.lock"))?;
        let _rewrite = NamedLock::shared(self.store.root().join(".rewrite.lock"))?;
        let _membership = NamedLock::shared(self.store.root().join(".thread-catalog.lock"))?;
        let sources = scan_sources(self.store.root())?;
        let mut items = Vec::new();
        for source in sources {
            let mut snippet = source
                .title
                .as_deref()
                .and_then(|text| matching_snippet(text, &query));
            if snippet.is_none() {
                let bytes = fs::read(source.folder.join("main.jsonl"))?;
                let projection = scan_valid_prefix(&bytes, 1).projection.ok_or_else(|| {
                    SearchError::SourceCorrupt("Conversation has no valid prefix".into())
                })?;
                // Appends may continue during a search. Use the catalog's captured sequence.
                let events: Vec<_> = projection
                    .events
                    .into_iter()
                    .filter(|event| event.seq() <= source.as_of_seq)
                    .collect();
                let superseded = superseded_sequences(&events)?;
                for event in events.iter().rev() {
                    if superseded.contains(&event.seq())
                        || !matches!(event.kind(), EventKind::Input | EventKind::Output)
                    {
                        continue;
                    }
                    let raw: serde_json::Value = serde_json::from_slice(
                        &event
                            .raw()
                            .canonical_bytes()
                            .map_err(|error| SearchError::SourceCorrupt(error.to_string()))?,
                    )
                    .map_err(|error| SearchError::SourceCorrupt(error.to_string()))?;
                    let Some(blocks) = raw.get("content").and_then(serde_json::Value::as_array)
                    else {
                        continue;
                    };
                    for block in blocks {
                        if block.get("type").and_then(serde_json::Value::as_str) != Some("text") {
                            continue;
                        }
                        if let Some(text) = block.get("text").and_then(serde_json::Value::as_str) {
                            snippet = matching_snippet(text, &query);
                            if snippet.is_some() {
                                break;
                            }
                        }
                    }
                    if snippet.is_some() {
                        break;
                    }
                }
            }
            if let Some(snippet) = snippet {
                if items.len() == MAX_PAGE_SIZE {
                    return Ok(SessionSearchResults {
                        items,
                        has_more: true,
                    });
                }
                items.push(SessionSearchHit {
                    session_id: source.session_id,
                    snippet,
                });
            }
        }
        Ok(SessionSearchResults {
            items,
            has_more: false,
        })
    }
}

fn matching_snippet(text: &str, query: &str) -> Option<String> {
    // Normalization can change scalar counts; derive the excerpt in normalized
    // text so a late match is always present and UTF-8 boundaries remain valid.
    let text = normalize(text);
    let position = text.find(query)?;
    let before = text[..position].chars().count();
    let start = before.saturating_sub(60);
    let count = query.chars().count().saturating_add(120);
    let mut result: String = text.chars().skip(start).take(count).collect();
    if start > 0 {
        result.insert(0, '…');
    }
    if text.chars().count() > start + count {
        result.push('…');
    }
    Some(result)
}
