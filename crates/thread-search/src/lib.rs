//! Rebuildable, title-only search over authoritative Kernel thread ledgers.
//!
//! This crate deliberately exposes no Client route. Slice 14F may bind the
//! collision-safe authority seam below to a separately versioned capability.

mod host;
pub use host::{SessionSearchHit, SessionSearchResults};

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use schema::EventKind;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use store::{AtomicPublisher, NamedLock, ThreadStore, scan_valid_prefix};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

const FORMAT: u64 = 1;
const INDEX_NAME: &str = ".thread-search.json";
/// The cache name before version suffixes were dropped; removed on sight.
const LEGACY_INDEX_NAME: &str = ".thread-search-v1.json";
const MAX_QUERY_BYTES: usize = 512;
const MAX_WORKSPACE_BYTES: usize = 128;
const MAX_PAGE_SIZE: usize = 50;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveVisibility {
    Active,
    Archived,
    All,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchRequest {
    pub workspace_id: String,
    pub query: String,
    pub limit: usize,
    pub visibility: ArchiveVisibility,
    pub after: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchClass {
    Exact,
    Prefix,
    Tokens,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResult {
    pub session_id: String,
    pub workspace_id: String,
    pub title: String,
    pub updated_at: String,
    pub archived: bool,
    pub as_of_seq: u64,
    pub score: u16,
    pub match_class: MatchClass,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexUse {
    pub hits: usize,
    pub missing: usize,
    pub stale: usize,
    pub corrupt: usize,
    pub write_failed: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchPage {
    pub results: Vec<SearchResult>,
    pub next_cursor: Option<String>,
    pub reached_end: bool,
    pub catalog_digest: String,
    pub index_use: IndexUse,
}

#[derive(Clone, Debug)]
pub struct ThreadSearchAuthority {
    store: ThreadStore,
}

impl ThreadSearchAuthority {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, SearchError> {
        Ok(Self {
            store: ThreadStore::open(root)?,
        })
    }

    /// Searches one authoritative catalog snapshot. The search lock prevents
    /// two cache repairers from publishing over one another; the rewrite and
    /// membership locks make the active/archive folder set collision-free.
    pub fn search(&self, request: &SearchRequest) -> Result<SearchPage, SearchError> {
        validate_request(request)?;
        let normalized_query = normalize(&request.query);
        let _search = NamedLock::exclusive(self.store.root().join(".thread-search.lock"))?;
        let _rewrite = NamedLock::shared(self.store.root().join(".rewrite.lock"))?;
        let _membership = NamedLock::shared(self.store.root().join(".thread-catalog.lock"))?;
        let (entries, index_use) = self.scan_and_repair_indices()?;
        let scoped = entries
            .into_iter()
            .filter(|entry| entry.workspace_id == request.workspace_id)
            .filter(|entry| visibility_includes(request.visibility, entry.archived))
            .collect::<Vec<_>>();
        let catalog_digest = digest_entries(&scoped)?;

        let mut ranked = scoped
            .into_iter()
            .filter_map(|entry| rank(entry, &normalized_query))
            .collect::<Vec<_>>();
        ranked.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.session_id.as_bytes().cmp(right.session_id.as_bytes()))
        });

        let start = if let Some(cursor) = &request.after {
            let cursor = decode_cursor(cursor)?;
            validate_cursor_scope(&cursor, request, &normalized_query, &catalog_digest)?;
            let position = ranked
                .iter()
                .position(|candidate| {
                    candidate.session_id == cursor.session_id && candidate.score == cursor.score
                })
                .ok_or(SearchError::CursorPosition)?;
            position + 1
        } else {
            0
        };
        let end = start.saturating_add(request.limit).min(ranked.len());
        let results = ranked[start..end].to_vec();
        let reached_end = end == ranked.len();
        let next_cursor = (!reached_end)
            .then(|| {
                let last = results.last().ok_or(SearchError::CursorPosition)?;
                encode_cursor(&CursorBody {
                    v: FORMAT,
                    workspace_id: request.workspace_id.clone(),
                    visibility: request.visibility,
                    normalized_query: normalized_query.clone(),
                    catalog_digest: catalog_digest.clone(),
                    score: last.score,
                    session_id: last.session_id.clone(),
                })
            })
            .transpose()?;
        Ok(SearchPage {
            results,
            next_cursor,
            reached_end,
            catalog_digest,
            index_use,
        })
    }

    /// Rebuilds every per-thread cache from the semantic ledgers. The returned
    /// digest covers the complete active+archived search source, not cache
    /// bytes. A failed cache write is an error here (unlike query fallback).
    pub fn rebuild_index(&self) -> Result<String, SearchError> {
        let _search = NamedLock::exclusive(self.store.root().join(".thread-search.lock"))?;
        let _rewrite = NamedLock::shared(self.store.root().join(".rewrite.lock"))?;
        let _membership = NamedLock::shared(self.store.root().join(".thread-catalog.lock"))?;
        let sources = scan_sources(self.store.root())?;
        for source in &sources {
            publish_index(source)?;
        }
        digest_entries(&sources)
    }

    fn scan_and_repair_indices(&self) -> Result<(Vec<IndexEntry>, IndexUse), SearchError> {
        let sources = scan_sources(self.store.root())?;
        let mut use_counts = IndexUse::default();
        let mut usable = Vec::with_capacity(sources.len());
        for source in sources {
            let _ = std::fs::remove_file(source.folder.join(LEGACY_INDEX_NAME));
            let path = source.folder.join(INDEX_NAME);
            match fs::read(&path) {
                Ok(bytes) => match decode_index(&bytes) {
                    Ok(mut index) if same_projection(&index.entry, &source) => {
                        use_counts.hits += 1;
                        index.entry.folder = source.folder;
                        usable.push(index.entry);
                        continue;
                    }
                    Ok(_) => {
                        use_counts.stale += 1;
                    }
                    Err(_) => {
                        use_counts.corrupt += 1;
                    }
                },
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    use_counts.missing += 1;
                }
                Err(_) => {
                    use_counts.corrupt += 1;
                }
            }
            if publish_index(&source).is_err() {
                use_counts.write_failed += 1;
            }
            usable.push(source);
        }
        Ok((usable, use_counts))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct IndexFile {
    format: u64,
    source_digest: String,
    entry: IndexEntry,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct IndexEntry {
    session_id: String,
    workspace_id: String,
    title: Option<String>,
    normalized_title: Option<String>,
    updated_at: String,
    archived: bool,
    as_of_seq: u64,
    #[serde(skip)]
    folder: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CursorBody {
    v: u64,
    workspace_id: String,
    visibility: ArchiveVisibility,
    normalized_query: String,
    catalog_digest: String,
    score: u16,
    session_id: String,
}

fn validate_request(request: &SearchRequest) -> Result<(), SearchError> {
    if !valid_workspace_id(&request.workspace_id) {
        return Err(SearchError::InvalidWorkspace);
    }
    if request.query.len() > MAX_QUERY_BYTES
        || request
            .query
            .chars()
            .any(|value| value.is_control() && !is_search_whitespace(value))
        || normalize(&request.query).is_empty()
    {
        return Err(SearchError::InvalidQuery);
    }
    if !(1..=MAX_PAGE_SIZE).contains(&request.limit) {
        return Err(SearchError::InvalidLimit);
    }
    Ok(())
}

fn valid_workspace_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_WORKSPACE_BYTES
        && !value.starts_with('.')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn visibility_includes(visibility: ArchiveVisibility, archived: bool) -> bool {
    matches!(visibility, ArchiveVisibility::All)
        || matches!(visibility, ArchiveVisibility::Archived) == archived
}

fn rank(entry: IndexEntry, query: &str) -> Option<SearchResult> {
    let normalized = entry.normalized_title.as_deref()?;
    let (score, match_class) = if normalized == query {
        (3_000, MatchClass::Exact)
    } else if normalized.starts_with(query) {
        (2_000, MatchClass::Prefix)
    } else if query.split(' ').all(|token| normalized.contains(token)) {
        (1_000, MatchClass::Tokens)
    } else {
        return None;
    };
    Some(SearchResult {
        session_id: entry.session_id,
        workspace_id: entry.workspace_id,
        title: entry.title.expect("normalized title requires source title"),
        updated_at: entry.updated_at,
        archived: entry.archived,
        as_of_seq: entry.as_of_seq,
        score,
        match_class,
    })
}

fn scan_sources(root: &Path) -> Result<Vec<IndexEntry>, SearchError> {
    let mut entries = Vec::new();
    let mut identities = HashSet::new();
    for (area, archived) in [("threads", false), ("archive", true)] {
        let mut folders = fs::read_dir(root.join(area))?.collect::<Result<Vec<_>, _>>()?;
        folders.sort_by_key(std::fs::DirEntry::file_name);
        for folder in folders {
            if !folder.file_type()?.is_dir() {
                continue;
            }
            let session_id = folder.file_name().to_string_lossy().into_owned();
            validate_session_id(&session_id)?;
            if !identities.insert(session_id.clone()) {
                return Err(SearchError::SourceCorrupt(format!(
                    "session {session_id} exists in active and archive catalogs"
                )));
            }
            let bytes = fs::read(folder.path().join("main.jsonl"))?;
            let scan = scan_valid_prefix(&bytes, 1);
            let valid_len = usize::try_from(scan.valid_bytes)
                .map_err(|_| SearchError::SourceCorrupt("valid prefix is too large".to_owned()))?;
            if bytes
                .get(valid_len..)
                .is_some_and(|tail| tail.contains(&b'\n'))
            {
                return Err(SearchError::SourceCorrupt(format!(
                    "session {session_id} has an invalid complete ledger line"
                )));
            }
            let projection = scan.projection.ok_or_else(|| {
                SearchError::SourceCorrupt(format!(
                    "session {session_id} has no valid genesis prefix"
                ))
            })?;
            let genesis = projection.events.first().ok_or_else(|| {
                SearchError::SourceCorrupt(format!("session {session_id} has no genesis"))
            })?;
            if !matches!(genesis.kind(), EventKind::Genesis)
                || genesis.string_field("thread") != Some(session_id.as_str())
            {
                return Err(SearchError::SourceCorrupt(format!(
                    "session {session_id} folder/genesis identity mismatch"
                )));
            }
            let workspace_id = genesis
                .string_field("workspace")
                .ok_or_else(|| {
                    SearchError::SourceCorrupt(format!(
                        "session {session_id} genesis lacks workspace"
                    ))
                })?
                .to_owned();
            if !valid_workspace_id(&workspace_id) {
                return Err(SearchError::SourceCorrupt(format!(
                    "session {session_id} has an invalid workspace identity"
                )));
            }
            let superseded = superseded_sequences(&projection.events)?;
            let title = projection
                .events
                .iter()
                .rev()
                .find(|event| {
                    matches!(event.kind(), EventKind::Meta)
                        && event.string_field("title").is_some()
                        && !superseded.contains(&event.seq())
                })
                .and_then(|event| event.string_field("title"))
                .map(str::to_owned);
            let updated_at = projection
                .events
                .last()
                .and_then(|event| event.string_field("ts"))
                .ok_or_else(|| {
                    SearchError::SourceCorrupt(format!("session {session_id} tail lacks timestamp"))
                })?
                .to_owned();
            entries.push(IndexEntry {
                session_id,
                workspace_id,
                normalized_title: title.as_deref().map(normalize),
                title,
                updated_at,
                archived,
                as_of_seq: projection.last_seq,
                folder: folder.path(),
            });
        }
    }
    entries.sort_by(|left, right| left.session_id.as_bytes().cmp(right.session_id.as_bytes()));
    Ok(entries)
}

fn superseded_sequences(events: &[schema::Event]) -> Result<BTreeSet<u64>, SearchError> {
    let mut superseded = BTreeSet::new();
    for event in events {
        for range in event.supersedes().map_err(|error| {
            SearchError::SourceCorrupt(format!("seq {} supersedes: {error}", event.seq()))
        })? {
            superseded.extend(range.from..=range.to);
        }
    }
    Ok(superseded)
}

fn validate_session_id(value: &str) -> Result<(), SearchError> {
    let parsed = Uuid::parse_str(value).map_err(|_| SearchError::SourceIdentity)?;
    if parsed.hyphenated().to_string() != value || value != value.to_ascii_lowercase() {
        return Err(SearchError::SourceIdentity);
    }
    Ok(())
}

fn publish_index(entry: &IndexEntry) -> Result<(), SearchError> {
    let value = IndexFile {
        format: FORMAT,
        source_digest: digest_entries(std::slice::from_ref(entry))?,
        entry: entry.clone(),
    };
    let mut bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|error| SearchError::Canonical(error.to_string()))?;
    bytes.push(b'\n');
    AtomicPublisher::replace(entry.folder.join(INDEX_NAME), &bytes)?;
    let _ = std::fs::remove_file(entry.folder.join(LEGACY_INDEX_NAME));
    Ok(())
}

fn same_projection(left: &IndexEntry, right: &IndexEntry) -> bool {
    left.session_id == right.session_id
        && left.workspace_id == right.workspace_id
        && left.title == right.title
        && left.normalized_title == right.normalized_title
        && left.updated_at == right.updated_at
        && left.archived == right.archived
        && left.as_of_seq == right.as_of_seq
}

fn decode_index(bytes: &[u8]) -> Result<IndexFile, SearchError> {
    let line = bytes
        .strip_suffix(b"\n")
        .ok_or_else(|| SearchError::IndexCorrupt("index lacks final LF".to_owned()))?;
    if line.contains(&b'\n') {
        return Err(SearchError::IndexCorrupt(
            "index contains multiple lines".to_owned(),
        ));
    }
    let index: IndexFile = serde_json::from_slice(line)
        .map_err(|error| SearchError::IndexCorrupt(error.to_string()))?;
    if index.format != FORMAT {
        return Err(SearchError::IndexCorrupt(
            "unsupported index format".to_owned(),
        ));
    }
    validate_session_id(&index.entry.session_id)?;
    if !valid_workspace_id(&index.entry.workspace_id)
        || index.entry.as_of_seq == 0
        || index.entry.normalized_title != index.entry.title.as_deref().map(normalize)
        || index.source_digest != digest_entries(std::slice::from_ref(&index.entry))?
    {
        return Err(SearchError::IndexCorrupt(
            "index source projection mismatch".to_owned(),
        ));
    }
    let canonical = serde_json_canonicalizer::to_vec(&index)
        .map_err(|error| SearchError::Canonical(error.to_string()))?;
    if canonical != line {
        return Err(SearchError::IndexCorrupt(
            "index is not canonical JSON".to_owned(),
        ));
    }
    Ok(index)
}

fn digest_entries(entries: &[IndexEntry]) -> Result<String, SearchError> {
    let bytes = serde_json_canonicalizer::to_vec(&entries)
        .map_err(|error| SearchError::Canonical(error.to_string()))?;
    Ok(format!("sha256-{:x}", Sha256::digest(bytes)))
}

fn encode_cursor(cursor: &CursorBody) -> Result<String, SearchError> {
    let bytes = serde_json_canonicalizer::to_vec(cursor)
        .map_err(|error| SearchError::Canonical(error.to_string()))?;
    Ok(format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(&bytes),
        hex_digest(&bytes)
    ))
}

fn decode_cursor(value: &str) -> Result<CursorBody, SearchError> {
    let mut fields = value.split('.');
    let encoded = fields.next().ok_or(SearchError::MalformedCursor)?;
    let digest = fields.next().ok_or(SearchError::MalformedCursor)?;
    if fields.next().is_some()
        || digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(SearchError::MalformedCursor);
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| SearchError::MalformedCursor)?;
    if URL_SAFE_NO_PAD.encode(&bytes) != encoded || hex_digest(&bytes) != digest {
        return Err(SearchError::MalformedCursor);
    }
    let cursor: CursorBody =
        serde_json::from_slice(&bytes).map_err(|_| SearchError::MalformedCursor)?;
    let canonical =
        serde_json_canonicalizer::to_vec(&cursor).map_err(|_| SearchError::MalformedCursor)?;
    if canonical != bytes
        || cursor.v != FORMAT
        || !matches!(cursor.score, 1_000 | 2_000 | 3_000)
        || validate_session_id(&cursor.session_id).is_err()
        || !valid_workspace_id(&cursor.workspace_id)
        || cursor.normalized_query.is_empty()
        || normalize(&cursor.normalized_query) != cursor.normalized_query
        || !valid_sha256_label(&cursor.catalog_digest)
    {
        return Err(SearchError::MalformedCursor);
    }
    Ok(cursor)
}

fn valid_sha256_label(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256-")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_cursor_scope(
    cursor: &CursorBody,
    request: &SearchRequest,
    normalized_query: &str,
    catalog_digest: &str,
) -> Result<(), SearchError> {
    if cursor.workspace_id != request.workspace_id
        || cursor.visibility != request.visibility
        || cursor.normalized_query != normalized_query
    {
        return Err(SearchError::CursorScope);
    }
    if cursor.catalog_digest != catalog_digest {
        return Err(SearchError::CursorStale);
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn normalize(value: &str) -> String {
    let mut output = String::new();
    let mut whitespace = false;
    for scalar in value.nfkc() {
        if is_search_whitespace(scalar) {
            whitespace = !output.is_empty();
            continue;
        }
        if whitespace {
            output.push(' ');
            whitespace = false;
        }
        output.push(if scalar.is_ascii_uppercase() {
            scalar.to_ascii_lowercase()
        } else {
            scalar
        });
    }
    output
}

fn is_search_whitespace(value: char) -> bool {
    matches!(
        value,
        '\u{0009}'..='\u{000d}'
            | '\u{0020}'
            | '\u{0085}'
            | '\u{00a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
    )
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("invalid workspace scope")]
    InvalidWorkspace,
    #[error("invalid search query")]
    InvalidQuery,
    #[error("invalid search page limit")]
    InvalidLimit,
    #[error("malformed search cursor")]
    MalformedCursor,
    #[error("search cursor does not match request scope")]
    CursorScope,
    #[error("search cursor catalog snapshot is stale")]
    CursorStale,
    #[error("search cursor does not name a ranked result")]
    CursorPosition,
    #[error("thread folder identity is not a canonical UUID")]
    SourceIdentity,
    #[error("search source is corrupt: {0}")]
    SourceCorrupt(String),
    #[error("search index is corrupt: {0}")]
    IndexCorrupt(String),
    #[error("canonical JSON failed: {0}")]
    Canonical(String),
    #[error("store error: {0}")]
    Store(#[from] store::StoreError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
