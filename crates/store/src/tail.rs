use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use schema::{Event, EventKind, LedgerProjection, LedgerValidator};
use serde_json::json;

use crate::platform::{SyncPolicy, SystemSync, try_lock_exclusive, unlock};
use crate::{BarrierContext, StoreError, requires_barrier};

#[derive(Clone, Debug)]
pub struct TailScan {
    pub valid_bytes: u64,
    pub file_bytes: u64,
    pub first_invalid_offset: Option<u64>,
    pub projection: Option<LedgerProjection>,
    validator: Option<LedgerValidator>,
}

impl TailScan {
    #[must_use]
    pub const fn needs_repair(&self) -> bool {
        self.valid_bytes != self.file_bytes
    }

    #[must_use]
    pub fn last_seq(&self) -> u64 {
        self.projection.as_ref().map_or(0, |value| value.last_seq)
    }
}

/// Finds the authoritative LF-terminated prefix. The first malformed or
/// fold-invalid line terminates the ledger; later bytes are deliberately not
/// interpreted.
pub fn scan_valid_prefix(bytes: &[u8], reader_version: u64) -> TailScan {
    let mut validator = LedgerValidator::new(reader_version);
    let mut cursor = 0usize;
    let mut invalid = None;

    while cursor < bytes.len() {
        let rest = &bytes[cursor..];
        let Some(relative_lf) = rest.iter().position(|byte| *byte == b'\n') else {
            invalid = Some(cursor as u64);
            break;
        };
        let line_end = cursor + relative_lf;
        if line_end == cursor {
            invalid = Some(cursor as u64);
            break;
        }
        match Event::decode_canonical(&bytes[cursor..line_end])
            .and_then(|event| validator.push(event))
        {
            Ok(()) => cursor = line_end + 1,
            Err(_) => {
                invalid = Some(cursor as u64);
                break;
            }
        }
    }

    let projection = (cursor != 0)
        .then(|| validator.clone().finish().ok())
        .flatten();
    TailScan {
        valid_bytes: cursor as u64,
        file_bytes: bytes.len() as u64,
        first_invalid_offset: invalid,
        projection,
        validator: Some(validator),
    }
}

/// Decodes the complete canonical lines before the first structural failure.
fn frame_events(bytes: &[u8]) -> Vec<Event> {
    let mut events = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        let rest = &bytes[cursor..];
        let Some(relative_lf) = rest.iter().position(|byte| *byte == b'\n') else {
            break;
        };
        let line_end = cursor + relative_lf;
        if line_end == cursor {
            break;
        }
        match Event::decode_canonical(&bytes[cursor..line_end]) {
            Ok(event) => {
                events.push(event);
                cursor = line_end + 1;
            }
            Err(_) => break,
        }
    }
    events
}

pub struct LockedLedger {
    path: PathBuf,
    file: File,
    validator: LedgerValidator,
    projection: Option<LedgerProjection>,
    sync: Arc<dyn SyncPolicy>,
    poisoned: bool,
    reader_version: u64,
}

// Format-writing capability belongs to the implementation, not the caller's
// requested reader version. Raising this requires implementing the new format.
const WRITER_VERSION: u64 = 1;

pub(crate) fn check_ledger_write_versions(
    bytes: &[u8],
    reader_version: u64,
) -> Result<(), StoreError> {
    check_write_versions(&frame_events(bytes), reader_version)
}

pub(crate) fn check_write_versions<'a>(
    events: impl IntoIterator<Item = &'a Event>,
    reader_version: u64,
) -> Result<(), StoreError> {
    let mut required_reader = 1;
    let mut required_writer = 1;
    for event in events {
        required_reader = required_reader.max(event.min_reader().unwrap_or(1));
        if matches!(event.kind(), EventKind::Genesis) {
            required_writer = required_writer.max(event.integer_field("min_writer").unwrap_or(1));
        }
        if matches!(event.kind(), EventKind::Meta) {
            let value = serde_json::to_value(event.raw())
                .map_err(|error| StoreError::Corruption(error.to_string()))?;
            if let Some(upgrade) = value.get("upgrade") {
                required_reader = required_reader.max(upgrade["min_reader"].as_u64().unwrap_or(1));
                required_writer = required_writer.max(upgrade["min_writer"].as_u64().unwrap_or(1));
            }
        }
    }
    if required_reader > reader_version || required_writer > WRITER_VERSION {
        return Err(StoreError::VersionGate {
            required_reader,
            required_writer,
            reader_version,
            writer_version: WRITER_VERSION,
        });
    }
    Ok(())
}

impl LockedLedger {
    pub fn open(path: impl AsRef<Path>, reader_version: u64) -> Result<Self, StoreError> {
        Self::open_with_sync(path, reader_version, Arc::new(SystemSync))
    }

    pub fn open_with_sync(
        path: impl AsRef<Path>,
        reader_version: u64,
        sync: Arc<dyn SyncPolicy>,
    ) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&path)?;
        try_lock_exclusive(&file)?;
        revalidate_inode(&path, &file)?;

        file.seek(SeekFrom::Start(0))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        // Check envelopes before semantic replay or tail repair: a newer
        // format's valid suffix must never be truncated by an older writer.
        check_ledger_write_versions(&bytes, reader_version)?;
        let scan = scan_valid_prefix(&bytes, reader_version);
        if scan.needs_repair() {
            file.set_len(scan.valid_bytes)?;
            sync.full_sync(&file)?;
        }

        let validator = scan
            .validator
            .clone()
            .unwrap_or_else(|| LedgerValidator::new(reader_version));
        let projection = scan.projection;
        Ok(Self {
            path,
            file,
            validator,
            projection,
            sync,
            poisoned: false,
            reader_version,
        })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn projection(&self) -> Option<&LedgerProjection> {
        self.projection.as_ref()
    }

    #[must_use]
    pub fn next_seq(&self) -> u64 {
        self.projection
            .as_ref()
            .map_or(1, |value| value.last_seq + 1)
    }

    /// Make the accepted prefix durable before an external observer consumes it.
    /// This creates no semantic event and cannot revive a poisoned writer.
    pub fn sync_prefix(&mut self) -> Result<(), StoreError> {
        if self.poisoned {
            return Err(StoreError::Corruption("writer is poisoned".to_owned()));
        }
        if let Err(error) = self.sync.full_sync(&self.file) {
            self.poisoned = true;
            return Err(StoreError::Io(error));
        }
        Ok(())
    }

    /// Appends the `checkpoint` marker covering the current tail: the
    /// compaction boundary a later `compact` may cover. It is a barrier; a
    /// reader always folds from genesis.
    pub fn create_checkpoint(&mut self, timestamp: &str, summary: &str) -> Result<u64, StoreError> {
        if self.poisoned {
            return Err(StoreError::Corruption(
                "writer is poisoned by an earlier append/sync failure".to_owned(),
            ));
        }
        let seq = self.next_seq();
        let canonical = serde_json_canonicalizer::to_vec(&json!({
            "v": 1,
            "seq": seq,
            "kind": "checkpoint",
            "ts": timestamp,
            "covers": seq - 1,
            "summary": summary
        }))
        .map_err(|error| StoreError::Corruption(error.to_string()))?;
        let event = Event::decode_canonical(&canonical)?;
        self.append_contract(event, BarrierContext::default())?;
        Ok(seq)
    }

    /// Appends exactly one canonical event and LF. `barrier` is supplied by
    /// the semantic caller for conditionally durable kinds; a true value
    /// orders the complete prefix before success is returned.
    pub fn append(&mut self, event: Event, barrier: bool) -> Result<(), StoreError> {
        if self.poisoned {
            return Err(StoreError::Corruption(
                "writer is poisoned by an earlier append/sync failure".to_owned(),
            ));
        }
        check_write_versions(std::iter::once(&event), self.reader_version)?;
        let mut next = self.validator.clone();
        next.push(event.clone())?;
        let mut bytes = event.canonical_bytes()?;
        bytes.push(b'\n');

        if let Err(error) = self.file.write_all(&bytes) {
            self.poisoned = true;
            return Err(StoreError::Io(error));
        }
        if barrier {
            if let Err(error) = self.sync.full_sync(&self.file) {
                self.poisoned = true;
                return Err(StoreError::Io(error));
            }
        } else if let Err(error) = self.file.flush() {
            self.poisoned = true;
            return Err(StoreError::Io(error));
        }

        self.validator = next;
        self.projection = Some(self.validator.clone().finish()?);
        Ok(())
    }

    pub fn append_contract(
        &mut self,
        event: Event,
        context: BarrierContext,
    ) -> Result<(), StoreError> {
        let barrier = requires_barrier(&event, context);
        self.append(event, barrier)
    }
}

impl Drop for LockedLedger {
    fn drop(&mut self) {
        let _ = unlock(&self.file);
    }
}

fn revalidate_inode(path: &Path, file: &File) -> Result<(), StoreError> {
    let held = file.metadata()?;
    let current = path.metadata()?;
    if held.dev() != current.dev() || held.ino() != current.ino() {
        return Err(StoreError::ReplacedWhileLocking);
    }
    Ok(())
}

#[cfg(test)]
mod version_gate_tests {
    use super::*;
    use serde_json::Value;

    fn event(mut value: Value, seq: u64) -> Event {
        value["v"] = json!(1);
        value["seq"] = json!(seq);
        value["ts"] = json!("2026-09-04T00:00:00.000Z");
        Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap()
    }

    fn genesis(reader: u64, writer: u64) -> Event {
        event(
            json!({"kind":"genesis","format":1,"min_reader":reader,"min_writer":writer,
            "thread":"018f0000-0000-7000-8000-000000000003","workspace":"ws","resume":"never",
            "config":{"digest":"cfg"},"origin_key":"create",
            "origin_tuple":{"principal":"p","client":"c","target":"018f0000-0000-7000-8000-000000000003","op":"create","key":"create"}}),
            1,
        )
    }

    fn bytes(events: &[Event]) -> Vec<u8> {
        events
            .iter()
            .flat_map(|event| {
                let mut bytes = event.canonical_bytes().unwrap();
                bytes.push(b'\n');
                bytes
            })
            .collect()
    }

    #[test]
    fn downgrade_open_preserves_gated_prefix_and_unrecognized_suffix() {
        let cases = [
            vec![genesis(2, 1)],
            vec![genesis(1, 2)],
            vec![
                genesis(1, 1),
                event(
                    json!({"kind":"meta","upgrade":{"min_reader":2,"min_writer":2}}),
                    2,
                ),
            ],
            vec![
                genesis(1, 1),
                event(
                    json!({"kind":"meta","min_reader":2,"upgrade":{"min_reader":1,"min_writer":1}}),
                    2,
                ),
            ],
        ];
        for events in cases {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("main.jsonl");
            let mut original = bytes(&events);
            original.extend_from_slice(b"future-format-tail-without-LF");
            std::fs::write(&path, &original).unwrap();
            assert!(matches!(
                LockedLedger::open(&path, 1),
                Err(StoreError::VersionGate { .. })
            ));
            assert_eq!(std::fs::read(&path).unwrap(), original);
            assert!(
                !dir.path().join("assets").exists(),
                "gate must precede storage mutation"
            );
        }
    }

    #[test]
    fn reader_selection_cannot_claim_new_writer_support() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.jsonl");
        let original = bytes(&[genesis(1, 2)]);
        std::fs::write(&path, &original).unwrap();
        assert!(matches!(
            LockedLedger::open(&path, 2),
            Err(StoreError::VersionGate {
                required_writer: 2,
                writer_version: 1,
                ..
            })
        ));
        assert_eq!(std::fs::read(path).unwrap(), original);
    }

    #[test]
    fn creation_and_rewrite_cannot_bypass_writer_gate() {
        let source = "018f0000-0000-7000-8000-000000000003";
        let destination = "018f0000-0000-7000-8000-000000000004";
        for (reader, writer) in [(2, 1), (1, 2)] {
            let dir = tempfile::tempdir().unwrap();
            let store = crate::ThreadStore::open(dir.path()).unwrap();
            let newer_genesis = genesis(reader, writer);
            assert!(matches!(
                store.create_thread_with_assets(
                    source,
                    newer_genesis.clone(),
                    &[b"carrier".to_vec()]
                ),
                Err(StoreError::VersionGate { .. })
            ));
            assert!(!dir.path().join("threads").join(source).exists());
            assert_eq!(
                std::fs::read_dir(dir.path().join(".create-staging"))
                    .unwrap()
                    .count(),
                0
            );

            // Simulate a source produced by a newer binary. It remains readable,
            // but this implementation cannot publish a rewritten derivative.
            let folder = dir.path().join("threads").join(source);
            std::fs::create_dir(&folder).unwrap();
            let original = bytes(&[newer_genesis]);
            std::fs::write(folder.join("main.jsonl"), &original).unwrap();
            assert!(schema::validate_ledger(&original, 1).is_ok());
            assert!(matches!(
                store.create_thread_with_assets(
                    source,
                    genesis(1, 1),
                    &[b"retry carrier".to_vec()]
                ),
                Err(StoreError::VersionGate { .. })
            ));
            assert!(
                !folder.join("assets").exists(),
                "deduplicated creation must not publish into a gated target"
            );
            assert!(matches!(
                store.begin_fork(
                    "gated-fork",
                    source,
                    destination,
                    "2026-09-04T00:00:00.000Z"
                ),
                Err(StoreError::VersionGate { .. })
            ));
            assert!(matches!(
                store.begin_redact(
                    "gated-redact",
                    source,
                    destination,
                    "2026-09-04T00:00:00.000Z",
                    &[b"private".to_vec()]
                ),
                Err(StoreError::VersionGate { .. })
            ));
            assert_eq!(std::fs::read(folder.join("main.jsonl")).unwrap(), original);
            assert!(!dir.path().join("threads").join(destination).exists());
            assert_eq!(
                std::fs::read_dir(dir.path().join("staging"))
                    .unwrap()
                    .count(),
                0
            );
            assert_eq!(
                std::fs::read_dir(dir.path().join(".rewrite-trash"))
                    .unwrap()
                    .count(),
                0
            );
        }
    }

    #[test]
    fn append_rejects_unsupported_upgrade_without_changing_fold_or_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.jsonl");
        let original = bytes(&[genesis(1, 1)]);
        std::fs::write(&path, &original).unwrap();
        let mut ledger = LockedLedger::open(&path, 1).unwrap();
        for value in [
            json!({"kind":"meta","upgrade":{"min_reader":1,"min_writer":2}}),
            json!({"kind":"meta","min_reader":2,"upgrade":{"min_reader":1,"min_writer":1}}),
        ] {
            assert!(matches!(
                ledger.append(event(value, 2), true),
                Err(StoreError::VersionGate { .. })
            ));
            assert_eq!(ledger.next_seq(), 2);
            assert_eq!(std::fs::read(&path).unwrap(), original);
        }
        ledger
            .append(
                event(
                    json!({"kind":"meta","upgrade":{"min_reader":1,"min_writer":1}}),
                    2,
                ),
                false,
            )
            .unwrap();
        assert_eq!(ledger.next_seq(), 3);
        ledger
            .create_checkpoint("2026-09-04T00:00:00.000Z", "version gate regression")
            .unwrap();
        drop(ledger);
        let reopened = LockedLedger::open(&path, 1).unwrap();
        assert_eq!(reopened.next_seq(), 4);
    }
}
