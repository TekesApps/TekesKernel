use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use store::{DirectoryLock, FullSync, NamedLock};
use thiserror::Error;

use crate::types::{EndpointTypeError, MAX_SAFE_SEQUENCE, SessionEvent};

/// Record version of the durable projection. Bumped whenever the deterministic
/// projection of an existing ledger changes shape; a journal written under an
/// earlier version is retired on open and rebuilt from the ledger (endpoint
/// seqs are reassigned, clients re-baseline), never merged.
pub const JOURNAL_RECORD_VERSION: u64 = 5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JournalRecord {
    pub v: u64,
    pub event: SessionEvent,
    pub kernel_seqs: Vec<u64>,
    pub slot: String,
}

impl JournalRecord {
    pub fn validate(&self, expected_seq: u64) -> Result<(), JournalError> {
        if self.v != JOURNAL_RECORD_VERSION || self.event.seq != expected_seq {
            return Err(JournalError::Corruption(format!(
                "record at endpoint seq {expected_seq} has v={} seq={}",
                self.v, self.event.seq
            )));
        }
        self.event.validate()?;
        if self.kernel_seqs.iter().any(|seq| *seq > MAX_SAFE_SEQUENCE)
            || self.kernel_seqs.windows(2).any(|pair| pair[0] >= pair[1])
        {
            return Err(JournalError::Corruption(
                "kernel_seqs must be sorted and unique".to_owned(),
            ));
        }
        if self.slot.is_empty() || self.kernel_seqs.is_empty() {
            return Err(JournalError::Corruption(
                "record must name its kernel slot and causal seqs".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, JournalError> {
        serde_json_canonicalizer::to_vec(self)
            .map_err(|error| JournalError::Canonical(error.to_string()))
    }
}

/// The endpoint journal: the deterministic projection of the semantic ledger
/// into public `SessionEvent` rows, one file per thread folder.
pub const JOURNAL_FILE: &str = "endpoint.jsonl";
pub const LOCK_FILE: &str = "endpoint.lock";

/// Journals written under the earlier filename also persisted provider stream
/// chunks, which are transient now, and the earlier request journal cached
/// holds the ledger already carries. Neither can be read as a ledger
/// projection, so both are retired on open; the projector rebuilds
/// `endpoint.jsonl` from the semantic ledger (endpoint seqs are reassigned,
/// clients re-baseline) and requests are derived from the ledger.
/// A journal whose first record carries an earlier `v` was projected by an
/// older deterministic projection; its rows can no longer be reproduced
/// byte-for-byte from the ledger, so the file (and its lock) are retired and
/// the projector rebuilds `endpoint.jsonl` on the next publish.
fn retire_outdated_journal(folder: &Path, journal: &Path, lock: &Path) -> Result<(), JournalError> {
    if !journal.exists() {
        return Ok(());
    }
    let bytes = fs::read(journal)?;
    let Some(first) = bytes
        .split(|byte| *byte == b'\n')
        .find(|line| !line.is_empty())
    else {
        return Ok(());
    };
    let version = serde_json::from_slice::<serde_json::Value>(first)
        .ok()
        .and_then(|value| value.get("v").and_then(serde_json::Value::as_u64));
    if version == Some(JOURNAL_RECORD_VERSION) {
        return Ok(());
    }
    fs::remove_file(journal)?;
    if lock.exists() {
        fs::remove_file(lock)?;
    }
    fs::File::open(folder)?.sync_all()?;
    Ok(())
}

fn retire_legacy_journal(folder: &Path, journal: &Path) -> Result<(), JournalError> {
    if journal.exists() {
        return Ok(());
    }
    let mut removed = false;
    for name in [
        "endpoint-v2.jsonl",
        "endpoint-v2.lock",
        "endpoint-requests-v2.jsonl",
        "endpoint-requests-v2.lock",
    ] {
        let legacy = folder.join(name);
        if legacy.exists() {
            fs::remove_file(&legacy)?;
            removed = true;
        }
    }
    if removed {
        fs::File::open(folder)?.sync_all()?;
    }
    Ok(())
}

pub struct EndpointJournal {
    path: PathBuf,
    lock_path: PathBuf,
    _lifecycle: DirectoryLock,
    cache: Mutex<Option<JournalCache>>,
}

struct JournalCache {
    file_len: u64,
    records: Vec<JournalRecord>,
    identities: HashMap<String, usize>,
}

impl JournalCache {
    fn new(file_len: u64, records: Vec<JournalRecord>) -> Result<Self, JournalError> {
        let mut identities = HashMap::with_capacity(records.len());
        for (index, record) in records.iter().enumerate() {
            let identity = identity(record)?;
            if identities.insert(identity, index).is_some() {
                return Err(JournalError::Corruption(
                    "duplicate projection identity".to_owned(),
                ));
            }
        }
        Ok(Self {
            file_len,
            records,
            identities,
        })
    }
}

impl EndpointJournal {
    pub fn open(thread_folder: impl AsRef<Path>) -> Result<Self, JournalError> {
        let folder = thread_folder.as_ref();
        if !folder.is_dir() {
            return Err(JournalError::Corruption(format!(
                "thread folder does not exist: {}",
                folder.display()
            )));
        }
        let lifecycle = DirectoryLock::shared(folder)?;
        let path = folder.join(JOURNAL_FILE);
        let lock_path = folder.join(LOCK_FILE);
        retire_legacy_journal(folder, &path)?;
        retire_outdated_journal(folder, &path, &lock_path)?;
        let journal_was_missing = !path.exists();
        let lock_was_missing = !lock_path.exists();
        OpenOptions::new().create(true).append(true).open(&path)?;
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&lock_path)?;
        if journal_was_missing || lock_was_missing {
            std::fs::File::open(folder)?.sync_all()?;
        }
        let journal = Self {
            path,
            lock_path,
            _lifecycle: lifecycle,
            cache: Mutex::new(None),
        };
        journal.repair_tail()?;
        Ok(journal)
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn thread_folder(&self) -> &Path {
        self.path
            .parent()
            .expect("endpoint journal always has a thread-folder parent")
    }

    pub fn records(&self) -> Result<Vec<JournalRecord>, JournalError> {
        let _lock = NamedLock::shared(&self.lock_path)?;
        let mut cache = self
            .cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.refresh_cache_unlocked(&mut cache)?;
        Ok(cache.as_ref().expect("cache was refreshed").records.clone())
    }

    /// Returns one durable endpoint event without cloning the complete journal. Long-lived
    /// writers keep this lookup O(1) after append; a second journal instance still refreshes from
    /// disk when another process has extended the file.
    pub fn event(&self, seq: u64) -> Result<Option<SessionEvent>, JournalError> {
        let _lock = NamedLock::shared(&self.lock_path)?;
        let mut cache = self
            .cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.refresh_cache_unlocked(&mut cache)?;
        let Ok(index) = usize::try_from(seq) else {
            return Ok(None);
        };
        Ok(cache
            .as_ref()
            .expect("cache was refreshed")
            .records
            .get(index)
            .map(|record| record.event.clone()))
    }

    pub fn last_seq(&self) -> Result<Option<u64>, JournalError> {
        Ok(self.records()?.last().map(|record| record.event.seq))
    }

    /// Resolves a public endpoint sequence to the durable Kernel prefix it
    /// completely represents. Fork anchors are legal only at the last slot of
    /// one projection group; accepting an earlier 1:N slot would materialize
    /// only part of one Kernel fact.
    /// The endpoint seq a fork anchors to when the caller names none: the
    /// last record when no turn is open, otherwise the final slot of the
    /// projection group that ended the last settled turn. `None` when the
    /// journal is empty or its only turn is still open.
    pub fn last_settled_endpoint_seq(&self) -> Result<Option<u64>, JournalError> {
        let records = self.records()?;
        let Some(last) = records.last() else {
            return Ok(None);
        };
        let last_start = records
            .iter()
            .rposition(|record| record.event.event_type == "turn/start");
        let last_end = records
            .iter()
            .rposition(|record| record.event.event_type == "turn/end");
        let open_turn = match (last_start, last_end) {
            (Some(start), Some(end)) => start > end,
            (Some(_), None) => true,
            (None, _) => false,
        };
        if !open_turn {
            return Ok(Some(last.event.seq));
        }
        let Some(end) = last_end else {
            return Ok(None);
        };
        let anchor = records[end].kernel_seqs.last().copied();
        let mut index = end;
        while records
            .get(index + 1)
            .is_some_and(|next| next.kernel_seqs.last().copied() == anchor)
        {
            index += 1;
        }
        Ok(Some(records[index].event.seq))
    }

    pub fn kernel_anchor_for_endpoint_seq(&self, endpoint_seq: u64) -> Result<u64, JournalError> {
        let records = self.records()?;
        let index = usize::try_from(endpoint_seq)
            .map_err(|_| JournalError::EndpointSeqNotFound(endpoint_seq))?;
        let record = records
            .get(index)
            .ok_or(JournalError::EndpointSeqNotFound(endpoint_seq))?;
        let anchor = record.kernel_seqs.last().copied().ok_or_else(|| {
            JournalError::Corruption("kernel projection lacks causal anchor".to_owned())
        })?;
        if records
            .get(index + 1)
            .is_some_and(|next| next.kernel_seqs.last().copied() == Some(anchor))
        {
            return Err(JournalError::IncompleteProjectionGroup(endpoint_seq));
        }
        Ok(anchor)
    }

    pub fn append_kernel(
        &self,
        kernel_seqs: Vec<u64>,
        slot: impl Into<String>,
        event: SessionEvent,
    ) -> Result<SessionEvent, JournalError> {
        self.append_kernel_batch(vec![(kernel_seqs, slot.into(), event)])
            .map(|mut events| events.remove(0))
    }

    /// Appends all slots produced by one Kernel event under one journal lock
    /// and one durability barrier. Existing prefix slots are reused after a
    /// crash; only missing slots are appended, in the supplied order.
    pub fn append_kernel_batch(
        &self,
        projections: Vec<(Vec<u64>, String, SessionEvent)>,
    ) -> Result<Vec<SessionEvent>, JournalError> {
        if projections.is_empty() {
            return Ok(Vec::new());
        }
        let mut anchors = projections
            .iter()
            .map(|(kernel_seqs, _, _)| kernel_seqs.last().copied());
        let anchor = anchors.next().flatten();
        if anchor.is_none() || anchors.any(|candidate| candidate != anchor) {
            return Err(JournalError::Corruption(
                "kernel projection batch must share one causal anchor".to_owned(),
            ));
        }
        let candidates = projections
            .into_iter()
            .map(|(kernel_seqs, slot, event)| JournalRecord {
                v: JOURNAL_RECORD_VERSION,
                event,
                kernel_seqs,
                slot,
            })
            .collect();
        self.append_batch(candidates)
    }

    fn append_batch(
        &self,
        mut candidates: Vec<JournalRecord>,
    ) -> Result<Vec<SessionEvent>, JournalError> {
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        let mut cache_slot = self
            .cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.refresh_cache_unlocked(&mut cache_slot)?;
        let cache = cache_slot.as_mut().expect("cache was refreshed");
        let mut results = Vec::with_capacity(candidates.len());
        let mut bytes = Vec::new();
        let mut appended_missing = false;
        for mut candidate in candidates.drain(..) {
            let candidate_identity = identity(&candidate)?;
            if let Some(existing_index) = cache.identities.get(&candidate_identity).copied() {
                let existing = &cache.records[existing_index];
                if appended_missing {
                    return Err(JournalError::Corruption(
                        "projection batch has an existing slot after a missing slot".to_owned(),
                    ));
                }
                candidate.event.seq = existing.event.seq;
                if candidate.canonical_bytes()? != existing.canonical_bytes()? {
                    return Err(JournalError::IdentityConflict(existing.event.seq));
                }
                results.push(existing.event.clone());
                continue;
            }
            appended_missing = true;
            candidate.event.seq = u64::try_from(cache.records.len())
                .map_err(|_| JournalError::Corruption("journal length overflow".to_owned()))?;
            candidate.validate(candidate.event.seq)?;
            let mut line = candidate.canonical_bytes()?;
            line.push(b'\n');
            bytes.extend_from_slice(&line);
            results.push(candidate.event.clone());
            cache
                .identities
                .insert(candidate_identity, cache.records.len());
            cache.records.push(candidate);
        }
        if !bytes.is_empty() {
            let mut file = OpenOptions::new().append(true).open(&self.path)?;
            file.write_all(&bytes)?;
            FullSync::full_sync(&file)?;
            cache.file_len = cache.file_len.saturating_add(bytes.len() as u64);
        }
        Ok(results)
    }

    fn repair_tail(&self) -> Result<(), JournalError> {
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        let bytes = fs::read(&self.path)?;
        let (_, valid_len) = parse_records(&bytes, true)?;
        if valid_len != bytes.len() {
            let file = OpenOptions::new().write(true).open(&self.path)?;
            file.set_len(valid_len as u64)?;
            FullSync::full_sync(&file)?;
        }
        *self
            .cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        Ok(())
    }

    fn refresh_cache_unlocked(&self, cache: &mut Option<JournalCache>) -> Result<(), JournalError> {
        let file_len = fs::metadata(&self.path)?.len();
        if cache
            .as_ref()
            .is_some_and(|cached| cached.file_len == file_len)
        {
            return Ok(());
        }
        let bytes = fs::read(&self.path)?;
        let (records, valid_len) = parse_records(&bytes, false)?;
        if valid_len != bytes.len() {
            return Err(JournalError::Corruption("partial journal tail".to_owned()));
        }
        *cache = Some(JournalCache::new(file_len, records)?);
        Ok(())
    }
}

fn parse_records(
    bytes: &[u8],
    allow_partial_tail: bool,
) -> Result<(Vec<JournalRecord>, usize), JournalError> {
    let mut records = Vec::new();
    let mut offset = 0;
    let mut identities = HashSet::new();
    while offset < bytes.len() {
        let Some(relative_end) = bytes[offset..].iter().position(|byte| *byte == b'\n') else {
            if allow_partial_tail {
                return Ok((records, offset));
            }
            return Err(JournalError::Corruption("partial journal tail".to_owned()));
        };
        let end = offset + relative_end;
        if end == offset {
            return Err(JournalError::Corruption("blank journal line".to_owned()));
        }
        let line = &bytes[offset..end];
        let record: JournalRecord = serde_json::from_slice(line)?;
        if record.canonical_bytes()? != line {
            return Err(JournalError::Corruption(
                "journal line is not canonical JSON".to_owned(),
            ));
        }
        record.validate(records.len() as u64)?;
        let identity = identity(&record)?;
        if !identities.insert(identity) {
            return Err(JournalError::Corruption(
                "duplicate projection identity".to_owned(),
            ));
        }
        records.push(record);
        offset = end + 1;
    }
    Ok((records, offset))
}

fn identity(record: &JournalRecord) -> Result<String, JournalError> {
    let anchor = record.kernel_seqs.last().ok_or_else(|| {
        JournalError::Corruption("kernel projection lacks causal anchor".to_owned())
    })?;
    Ok(format!("kernel:{anchor}:{}", record.slot))
}

#[derive(Debug, Error)]
pub enum JournalError {
    #[error("endpoint journal IO failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("endpoint journal JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("endpoint event is invalid: {0}")]
    Type(#[from] EndpointTypeError),
    #[error("endpoint journal store failed: {0}")]
    Store(#[from] store::StoreError),
    #[error("endpoint journal canonicalization failed: {0}")]
    Canonical(String),
    #[error("endpoint journal corruption: {0}")]
    Corruption(String),
    #[error("projection identity at endpoint seq {0} changed bytes")]
    IdentityConflict(u64),
    #[error("endpoint seq {0} does not exist")]
    EndpointSeqNotFound(u64),
    #[error("endpoint seq {0} is not the last slot of its Kernel projection group")]
    IncompleteProjectionGroup(u64),
}

#[cfg(test)]
mod tests {
    use schema::IJsonValue;

    use super::*;

    fn event(event_type: &str) -> SessionEvent {
        SessionEvent {
            event_type: event_type.to_owned(),
            seq: 99,
            time: 0.0,
            data: IJsonValue::parse_str("{}").expect("data"),
            ignorable: None,
            source_event_seqs: None,
            surface_op: None,
        }
    }

    #[test]
    fn append_is_durable_and_idempotent() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        let first = journal
            .append_kernel(vec![3], "turn-start", event("turn/start"))
            .expect("append");
        assert_eq!(first.seq, 0);
        let retry = journal
            .append_kernel(vec![3], "turn-start", event("turn/start"))
            .expect("retry");
        assert_eq!(retry.seq, 0);
        assert_eq!(journal.records().expect("records").len(), 1);
    }

    #[test]
    fn an_outdated_record_version_retires_the_journal_on_open() {
        let folder = tempfile::tempdir().expect("tempdir");
        fs::write(
            folder.path().join(JOURNAL_FILE),
            b"{\"event\":{\"data\":{},\"eventType\":\"turn/start\",\"seq\":0,\"time\":1.0},\"kernel_seqs\":[1],\"slot\":\"turn-start\",\"v\":2}\n",
        )
        .expect("outdated journal");
        fs::write(folder.path().join(LOCK_FILE), b"").expect("lock");
        let journal = EndpointJournal::open(folder.path()).expect("open");
        assert!(
            journal.records().expect("records").is_empty(),
            "v2 rows are not carried over"
        );
        drop(journal);
        let reopened = EndpointJournal::open(folder.path()).expect("reopen");
        assert!(reopened.records().expect("records").is_empty());
    }

    #[test]
    fn a_legacy_journal_is_retired_on_open_and_rebuilt_from_the_ledger() {
        let folder = tempfile::tempdir().expect("folder");
        fs::write(folder.path().join("endpoint-v2.jsonl"), b"{\"v\":2}\n").expect("legacy");
        fs::write(folder.path().join("endpoint-v2.lock"), b"").expect("legacy lock");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        assert!(journal.records().expect("records").is_empty());
        assert!(!folder.path().join("endpoint-v2.jsonl").exists());
        assert!(!folder.path().join("endpoint-v2.lock").exists());
        assert!(folder.path().join(JOURNAL_FILE).is_file());
    }

    #[test]
    fn partial_tail_is_truncated() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        journal
            .append_kernel(vec![3], "turn-start", event("turn/start"))
            .expect("append");
        let mut file = OpenOptions::new()
            .append(true)
            .open(journal.path())
            .expect("open");
        file.write_all(b"{\"v\":2").expect("tear");
        drop(file);
        let reopened = EndpointJournal::open(folder.path()).expect("repair");
        assert_eq!(reopened.records().expect("records").len(), 1);
    }

    #[test]
    fn fork_anchor_requires_the_last_projection_slot() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        journal
            .append_kernel_batch(vec![
                (vec![3, 4], "turn-start".to_owned(), event("turn/start")),
                (vec![3, 4], "user-message".to_owned(), event("user/message")),
            ])
            .expect("projection group");

        assert!(matches!(
            journal.kernel_anchor_for_endpoint_seq(0),
            Err(JournalError::IncompleteProjectionGroup(0))
        ));
        assert_eq!(journal.kernel_anchor_for_endpoint_seq(1).unwrap(), 4);
        assert!(matches!(
            journal.kernel_anchor_for_endpoint_seq(2),
            Err(JournalError::EndpointSeqNotFound(2))
        ));
    }
}
