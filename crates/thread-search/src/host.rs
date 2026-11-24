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
    /// Includes active and archived sessions across Sessions. Results are stable
    /// by session identity and capped at 50; no credentials or tool payloads are searched.
    pub fn search_sessions(&self, query: &str) -> Result<SessionSearchResults, SearchError> {
        validate_request(&SearchRequest {
            session_filter: None,
            query: query.into(),
            limit: MAX_PAGE_SIZE,
            visibility: ArchiveVisibility::All,
            after: None,
        })?;
        let query = normalize(query);
        let _search = NamedLock::exclusive(self.store.root().join(".thread-search.lock"))?;
        let _rewrite = NamedLock::shared(self.store.root().join(".rewrite.lock"))?;
        let _membership = NamedLock::shared(self.store.root().join(".thread-catalog.lock"))?;
        let sources = self.scan_sources_cached()?;
        let mut items = Vec::new();
        for source in sources {
            let mut snippet = source
                .entry
                .title
                .as_deref()
                .and_then(|text| matching_snippet(text, &query));
            if snippet.is_none() {
                snippet = source
                    .visible_texts_rev
                    .iter()
                    .find_map(|text| matching_snippet(text, &query));
            }
            if let Some(snippet) = snippet {
                if items.len() == MAX_PAGE_SIZE {
                    return Ok(SessionSearchResults {
                        items,
                        has_more: true,
                    });
                }
                items.push(SessionSearchHit {
                    session_id: source.entry.session_id,
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
