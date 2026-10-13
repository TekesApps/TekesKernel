//! `rpc_id -> (session, line)` for every request hold (pending or resolved) the Kernel has seen
//! (G-BOOT-1 S4, ADDENDUM-02 M2/M3, ADDENDUM-03 C2).
//!
//! A request id is a pure hash of `(session, line, kind, hold seq)`, so the index can be filled
//! from a thread card's holds without reading the ledger. Resolved holds are indexed too: a retry
//! of an applied respond must find its request to replay the cached receipt. The index is a
//! lookup accelerator, never truth: a hit is always re-validated against the thread's ledger by
//! the caller, and a stale hit only costs one thread read.
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use endpoint::{RequestFrameType, derive_line_request_rpc_id};

/// Garbage ids remembered so a client loop cannot turn each one into a scan.
const NEGATIVE_CAPACITY: usize = 1_024;
const NEGATIVE_TTL: Duration = Duration::from_secs(60);

/// The byte pattern every `approval_request` line contains (JCS: no whitespace, member `kind`).
const HOLD_PATTERN: &[u8] = br#""kind":"approval_request""#;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IndexedRequest {
    pub session: String,
    pub line: Option<String>,
    pub archived: bool,
}

#[derive(Default)]
struct State {
    by_rpc: HashMap<String, IndexedRequest>,
    by_session: HashMap<String, Vec<String>>,
    negative: VecDeque<(String, Instant)>,
    negative_ids: HashSet<String>,
    /// Card-less folders (unverified threads, archives) already examined -- prefiltered and, if
    /// they hold requests, folded and indexed -- with the signature of the bytes examined.
    examined: HashMap<String, u64>,
    /// The byte prefilter of a thread's child lines, with the signature of the child files it
    /// was computed for: an unchanged signature answers it without reading the files again.
    children_prefilter: HashMap<String, (u64, bool)>,
}

#[derive(Default)]
pub(crate) struct RequestIndex {
    state: Mutex<State>,
    /// Test observer: how many times the child files were actually read by the prefilter.
    #[cfg(test)]
    children_scans: std::sync::atomic::AtomicU64,
}

impl RequestIndex {
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn insert(
        &self,
        session: &str,
        line: Option<&str>,
        kind: RequestFrameType,
        hold_seq: u64,
        archived: bool,
    ) {
        let rpc_id = derive_line_request_rpc_id(session, line, kind, hold_seq);
        let mut state = self.state();
        state.negative_ids.remove(&rpc_id);
        let entry = IndexedRequest {
            session: session.to_owned(),
            line: line.map(str::to_owned),
            archived,
        };
        if state.by_rpc.insert(rpc_id.clone(), entry).is_none() {
            state
                .by_session
                .entry(session.to_owned())
                .or_default()
                .push(rpc_id);
        }
    }

    pub(crate) fn lookup(&self, rpc_id: &str) -> Option<IndexedRequest> {
        self.state().by_rpc.get(rpc_id).cloned()
    }

    /// Drops a hit that turned out stale (the thread no longer holds the request).
    pub(crate) fn remove(&self, rpc_id: &str) {
        let mut state = self.state();
        if let Some(entry) = state.by_rpc.remove(rpc_id)
            && let Some(ids) = state.by_session.get_mut(&entry.session)
        {
            ids.retain(|id| id != rpc_id);
        }
    }

    /// Everything remembered about one session: a folder was published, moved or removed, and
    /// an id can be reused by a different thread.
    pub(crate) fn forget_session(&self, session: &str) {
        let mut state = self.state();
        for rpc_id in state.by_session.remove(session).unwrap_or_default() {
            state.by_rpc.remove(&rpc_id);
        }
        state.examined.remove(session);
        state.examined.remove(&format!("{session}#children"));
        state.children_prefilter.remove(session);
    }

    pub(crate) fn is_known_missing(&self, rpc_id: &str) -> bool {
        let mut state = self.state();
        let now = Instant::now();
        while state
            .negative
            .front()
            .is_some_and(|(_, at)| now.duration_since(*at) > NEGATIVE_TTL)
        {
            if let Some((id, _)) = state.negative.pop_front() {
                state.negative_ids.remove(&id);
            }
        }
        state.negative_ids.contains(rpc_id)
    }

    pub(crate) fn note_missing(&self, rpc_id: &str) {
        let mut state = self.state();
        if !state.negative_ids.insert(rpc_id.to_owned()) {
            return;
        }
        state
            .negative
            .push_back((rpc_id.to_owned(), Instant::now()));
        while state.negative.len() > NEGATIVE_CAPACITY {
            if let Some((id, _)) = state.negative.pop_front() {
                state.negative_ids.remove(&id);
            }
        }
    }

    /// True when this folder was already examined at exactly these bytes.
    pub(crate) fn examined(&self, session: &str, signature: u64) -> bool {
        self.state().examined.get(session) == Some(&signature)
    }

    pub(crate) fn note_examined(&self, session: &str, signature: u64) {
        self.state().examined.insert(session.to_owned(), signature);
    }

    /// `children_may_hold_requests` for the actionables baseline, which asks about every thread
    /// whose child lines the catalog cannot vouch for on every recompute. Child ledgers are
    /// append-only, so an unchanged signature of the child files (names and lengths) means
    /// unchanged bytes and the earlier answer stands; only a grown child file is read again.
    pub(crate) fn children_may_hold(&self, session: &str, folder: &Path) -> bool {
        let Some(signature) = children_signature(folder) else {
            // The folder cannot be listed: let the fold report it, as the scan does.
            return true;
        };
        if let Some((known, answer)) = self.state().children_prefilter.get(session)
            && *known == signature
        {
            return *answer;
        }
        #[cfg(test)]
        self.children_scans
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // The scan runs outside the lock. A file that grew after the signature was taken is
        // read at its newer length, and the changed signature makes the next call read again.
        let answer = children_may_hold_requests(folder);
        self.state()
            .children_prefilter
            .insert(session.to_owned(), (signature, answer));
        answer
    }

    /// Test observer: child-file prefilter reads so far.
    #[cfg(test)]
    pub(crate) fn children_scans(&self) -> u64 {
        self.children_scans
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.state().by_rpc.len()
    }
}

/// Cheap identity of a folder's ledger bytes: the lengths of every `*.jsonl` file except the
/// endpoint journal. Ledgers are append-only and rewrite/redact publish a new folder, so an
/// unchanged signature means nothing new could have been appended.
pub(crate) fn ledger_signature(folder: &Path) -> Option<u64> {
    signature_of(folder, false)
}

/// `ledger_signature` over the child line files only (everything but `main.jsonl`): the main
/// ledger grows with every turn and must not invalidate what is known about the children.
fn children_signature(folder: &Path) -> Option<u64> {
    signature_of(folder, true)
}

fn signature_of(folder: &Path, children_only: bool) -> Option<u64> {
    let mut signature = 0_u64;
    for entry in fs::read_dir(folder).ok()?.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.ends_with(".jsonl")
            || name == endpoint::JOURNAL_FILE
            || (children_only && name == "main.jsonl")
        {
            continue;
        }
        let length = entry.metadata().ok()?.len();
        // Order independent and sensitive to which file grew.
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for byte in name.bytes().chain(length.to_le_bytes()) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        signature = signature.wrapping_add(hash);
    }
    Some(signature)
}

/// `may_hold_requests` restricted to the child line files (everything but `main.jsonl`): for a
/// thread whose main line is folded and whose child lines are not.
pub(crate) fn children_may_hold_requests(folder: &Path) -> bool {
    scan_for_requests(folder, true)
}

/// M3: true when any ledger line file of the thread may hold an `approval_request`. A necessary
/// condition only (the valid prefix of a folded ledger decides): no false negatives, because
/// every such event is a canonical JSON line containing the exact pattern. An unreadable file
/// counts as a possible hit so the caller's fold reports it per thread.
pub(crate) fn may_hold_requests(folder: &Path) -> bool {
    scan_for_requests(folder, false)
}

fn scan_for_requests(folder: &Path, children_only: bool) -> bool {
    let Ok(entries) = fs::read_dir(folder) else {
        return true;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.ends_with(".jsonl")
            || name == endpoint::JOURNAL_FILE
            || (children_only && name == "main.jsonl")
        {
            continue;
        }
        match fs::read(entry.path()) {
            Ok(bytes) => {
                let valid = bytes
                    .iter()
                    .rposition(|byte| *byte == b'\n')
                    .map_or(&bytes[..0], |last| &bytes[..=last]);
                if valid
                    .windows(HOLD_PATTERN.len())
                    .any(|window| window == HOLD_PATTERN)
                {
                    return true;
                }
            }
            Err(_) => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_lookup_forget_and_negative_cache() {
        let index = RequestIndex::default();
        let session = "018f0000-0000-7000-8000-000000000001";
        index.insert(session, None, RequestFrameType::Approval, 8, false);
        index.insert(
            session,
            Some("018f0000-0000-7000-8000-000000000002.jsonl"),
            RequestFrameType::Question,
            4,
            false,
        );
        let main = derive_line_request_rpc_id(session, None, RequestFrameType::Approval, 8);
        assert_eq!(index.lookup(&main).expect("hit").session, session);
        assert_eq!(index.len(), 2);
        index.note_missing("request-garbage");
        assert!(index.is_known_missing("request-garbage"));
        index.forget_session(session);
        assert!(index.lookup(&main).is_none());
        assert_eq!(index.len(), 0);
    }

    /// O-final n2: the child-file prefilter of the actionables baseline reads the child files
    /// once per change of the child files, not once per recompute, and a grown child is read
    /// again. The main ledger growing does not invalidate it.
    #[test]
    fn the_child_prefilter_reads_the_child_files_only_when_they_change() {
        let folder = tempfile::tempdir().expect("folder");
        let session = "018f0000-0000-7000-8000-000000000001";
        let child = folder
            .path()
            .join("018f0000-0000-7000-8000-000000000002.jsonl");
        fs::write(&child, b"{\"kind\":\"turn_open\"}\n").expect("child");
        fs::write(folder.path().join("main.jsonl"), b"{}\n").expect("main");
        let index = RequestIndex::default();
        assert!(!index.children_may_hold(session, folder.path()));
        assert!(!index.children_may_hold(session, folder.path()));
        assert_eq!(
            index.children_scans(),
            1,
            "the second recompute reads nothing"
        );
        // main grows (every turn does): the children are unchanged
        fs::write(folder.path().join("main.jsonl"), b"{}\n{}\n").expect("main grows");
        assert!(!index.children_may_hold(session, folder.path()));
        assert_eq!(
            index.children_scans(),
            1,
            "main.jsonl is not part of the signature"
        );
        // a child grows with a request: read again, and the answer changes
        let mut text = fs::read(&child).expect("child");
        text.extend_from_slice(b"{\"kind\":\"approval_request\",\"seq\":2}\n");
        fs::write(&child, text).expect("child grows");
        assert!(index.children_may_hold(session, folder.path()));
        assert_eq!(index.children_scans(), 2);
        assert!(index.children_may_hold(session, folder.path()));
        assert_eq!(index.children_scans(), 2);
        // a forgotten session (id reuse) is read afresh
        index.forget_session(session);
        assert!(index.children_may_hold(session, folder.path()));
        assert_eq!(index.children_scans(), 3);
    }

    #[test]
    fn inserting_a_request_clears_its_negative_entry() {
        let index = RequestIndex::default();
        let session = "018f0000-0000-7000-8000-000000000001";
        let id = derive_line_request_rpc_id(session, None, RequestFrameType::Approval, 8);
        index.note_missing(&id);
        assert!(index.is_known_missing(&id));
        index.insert(session, None, RequestFrameType::Approval, 8, false);
        assert!(!index.is_known_missing(&id));
    }

    #[test]
    fn negative_cache_is_bounded() {
        let index = RequestIndex::default();
        for n in 0..(NEGATIVE_CAPACITY + 50) {
            index.note_missing(&format!("request-{n}"));
        }
        assert!(!index.is_known_missing("request-0"));
        assert!(index.is_known_missing(&format!("request-{}", NEGATIVE_CAPACITY + 49)));
    }

    #[test]
    fn prefilter_has_no_false_negative_and_ignores_the_torn_suffix() {
        let folder = tempfile::tempdir().expect("folder");
        let pattern = r#"{"kind":"approval_request","seq":8}"#;
        fs::write(folder.path().join("main.jsonl"), "{\"kind\":\"input\"}\n").expect("main");
        assert!(!may_hold_requests(folder.path()));
        // the pattern only in a child line file
        fs::write(
            folder
                .path()
                .join("018f0000-0000-7000-8000-000000000002.jsonl"),
            format!("{{\"kind\":\"input\"}}\n{pattern}\n"),
        )
        .expect("child");
        assert!(may_hold_requests(folder.path()));
        // the pattern only in a torn (not LF-terminated) suffix: not a hit
        fs::remove_file(
            folder
                .path()
                .join("018f0000-0000-7000-8000-000000000002.jsonl"),
        )
        .expect("remove");
        fs::write(
            folder.path().join("main.jsonl"),
            format!("{{\"kind\":\"input\"}}\n{pattern}"),
        )
        .expect("torn");
        assert!(!may_hold_requests(folder.path()));
        // the endpoint journal is never scanned
        fs::write(
            folder.path().join(endpoint::JOURNAL_FILE),
            format!("{pattern}\n"),
        )
        .expect("journal");
        assert!(!may_hold_requests(folder.path()));
    }

    #[test]
    fn ledger_signature_changes_when_a_file_grows_and_ignores_the_journal() {
        let folder = tempfile::tempdir().expect("folder");
        fs::write(folder.path().join("main.jsonl"), "a\n").expect("main");
        let first = ledger_signature(folder.path()).expect("signature");
        fs::write(folder.path().join(endpoint::JOURNAL_FILE), "journal\n").expect("journal");
        assert_eq!(ledger_signature(folder.path()), Some(first));
        fs::write(folder.path().join("main.jsonl"), "a\nb\n").expect("grown");
        assert_ne!(ledger_signature(folder.path()), Some(first));
    }
}
