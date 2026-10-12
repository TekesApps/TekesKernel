//! Builds and writes `thread-card.json` (G-BOOT-1 S2). Nothing reads the card yet: this slice
//! only produces it, from folds the supervisor has already done, so that the List and the thread
//! open paths can later skip decoding. See `store::card` for the invariants (C1, content
//! identity, no locks).
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use endpoint::EndpointJournal;
use schema::LedgerProjection;
use store::card::{
    CardInput, CardSummary, Hold, HoldResolution, JournalInput, LineInput, WriteOutcome,
};

use crate::endpoint_carrier::line_requests;

/// Default minimum interval between two card writes of one thread while it is busy.
pub(crate) const DEFAULT_DEBOUNCE: Duration = Duration::from_secs(1);

pub(crate) struct PreparedLine {
    file: String,
    prefix: Vec<u8>,
    last_seq: u64,
    line: String,
    parent_file: Option<String>,
    facts: schema::LifecycleFacts,
    holds: Vec<Hold>,
}

pub(crate) struct PreparedCard {
    session: String,
    folder: PathBuf,
    written: String,
    lines: Vec<PreparedLine>,
    summary: CardSummary,
    journal: Option<JournalInput>,
}

impl PreparedCard {
    fn fingerprint(&self) -> (u64, u64, u64) {
        let main = self.lines.first();
        (
            main.map_or(0, |line| line.last_seq),
            main.map_or(0, |line| line.prefix.len() as u64),
            self.journal.as_ref().map_or(0, |journal| journal.len),
        )
    }
}

fn holds_of(
    session: &str,
    source_line: Option<&str>,
    projection: &LedgerProjection,
) -> Option<Vec<Hold>> {
    let requests = line_requests(session, source_line, projection).ok()?;
    Some(
        requests
            .into_iter()
            .map(|(state, _)| Hold {
                hold_seq: state.request.causal_kernel_seq,
                kind: match state.request.frame_type {
                    endpoint::RequestFrameType::Approval => "approval",
                    endpoint::RequestFrameType::Question => "question",
                }
                .to_owned(),
                resolved: state.resolution.map(|(seq, outcome)| HoldResolution {
                    seq,
                    outcome: serde_json::to_value(outcome)
                        .ok()
                        .and_then(|value| value.as_str().map(str::to_owned))
                        .unwrap_or_default(),
                }),
            })
            .collect(),
    )
}

/// Builds the card for one thread from a **clean** fold of its main ledger (`bytes` is the file
/// content that was folded, `valid_bytes` its verified prefix) plus the open journal. Child
/// lines are read and folded here. Returns `None` when the thread cannot be described (no
/// summary, a hold that cannot be derived): a missing card is always safe.
pub(crate) fn prepare(
    session: &str,
    folder: PathBuf,
    mut bytes: Vec<u8>,
    valid_bytes: usize,
    projection: &LedgerProjection,
    journal: &EndpointJournal,
) -> Option<PreparedCard> {
    bytes.truncate(valid_bytes);
    let summary = endpoint::card_summary(projection).ok()?;
    let mut lines = vec![PreparedLine {
        file: "main.jsonl".to_owned(),
        prefix: bytes,
        last_seq: projection.last_seq,
        line: session.to_owned(),
        parent_file: None,
        facts: projection.lifecycle.clone(),
        holds: holds_of(session, None, projection)?,
    }];
    let mut children = Vec::new();
    for entry in std::fs::read_dir(&folder).ok()?.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".jsonl")
            && name != "main.jsonl"
            && name != endpoint::JOURNAL_FILE
            && entry.file_type().ok()?.is_file()
        {
            children.push(name);
        }
    }
    children.sort();
    for name in children {
        let child_bytes = std::fs::read(folder.join(&name)).ok()?;
        let scan = store::scan_valid_prefix(&child_bytes, 1);
        let valid = usize::try_from(scan.valid_bytes).ok()?;
        let Some(child) = scan.projection else {
            // An undescribable child leaves the card without that line (the reader then treats
            // the thread as Ahead for it and verifies it on first need).
            continue;
        };
        let genesis = child.events.first()?;
        let parent_file = serde_json::to_value(genesis.raw())
            .ok()?
            .get("parent")
            .and_then(|parent| parent.get("file"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);
        let mut prefix = child_bytes;
        prefix.truncate(valid);
        lines.push(PreparedLine {
            holds: holds_of(session, Some(&name), &child)?,
            line: genesis.string_field("thread")?.to_owned(),
            file: name,
            prefix,
            last_seq: child.last_seq,
            parent_file,
            facts: child.lifecycle,
        });
    }
    let tail = journal.tail_summary().ok()?;
    let len = std::fs::metadata(journal.path()).ok()?.len();
    Some(PreparedCard {
        session: session.to_owned(),
        written: crate::host_runtime::system_timestamp().ok()?,
        lines,
        summary,
        journal: Some(JournalInput {
            len,
            rows: tail.rows,
            last_endpoint_seq: tail
                .last_seq
                .map_or(-1, |seq| i64::try_from(seq).unwrap_or(i64::MAX)),
            last_kernel_seq: tail.last_kernel_seq,
        }),
        folder,
    })
}

fn write(prepared: &PreparedCard) -> WriteOutcome {
    let input = CardInput {
        thread: &prepared.session,
        kernel: env!("CARGO_PKG_VERSION"),
        written: prepared.written.clone(),
        lines: prepared
            .lines
            .iter()
            .map(|line| LineInput {
                file: &line.file,
                verified_prefix: &line.prefix,
                last_seq: line.last_seq,
                line: line.line.clone(),
                parent_file: line.parent_file.clone(),
                facts: line.facts.clone(),
                holds: line.holds.clone(),
            })
            .collect(),
        summary: prepared.summary.clone(),
        journal: prepared.journal.as_ref().map(|journal| JournalInput {
            len: journal.len,
            rows: journal.rows,
            last_endpoint_seq: journal.last_endpoint_seq,
            last_kernel_seq: journal.last_kernel_seq,
        }),
    };
    match store::card::write_card(&prepared.folder, &input) {
        Ok(outcome) => outcome,
        Err(error) => {
            eprintln!(
                "process-host-card-write-failed: session={} error={error}",
                prepared.session
            );
            WriteOutcome::Skipped(store::card::SkipReason::FileChanged)
        }
    }
}

enum Message {
    Card(Box<PreparedCard>),
    Shutdown,
}

/// Counters for tests and tracing.
#[derive(Default)]
pub(crate) struct CardStats {
    pub written: std::sync::atomic::AtomicU64,
    pub skipped: std::sync::atomic::AtomicU64,
}

/// A background thread that coalesces card jobs per thread and writes the newest one at most
/// once per debounce interval. The `F_FULLFSYNC` of the card protocol runs here, off the
/// doorbell path. Dropping or `shutdown()` flushes what is pending.
pub(crate) struct CardWriter {
    sender: Mutex<Option<Sender<Message>>>,
    handle: Mutex<Option<JoinHandle<()>>>,
    stats: Arc<CardStats>,
}

impl CardWriter {
    pub(crate) fn spawn(debounce: Duration) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let stats = Arc::new(CardStats::default());
        let thread_stats = Arc::clone(&stats);
        let handle = std::thread::Builder::new()
            .name("thread-card-writer".to_owned())
            .spawn(move || {
                let mut pending: HashMap<String, PreparedCard> = HashMap::new();
                let mut last: HashMap<String, (Instant, (u64, u64, u64))> = HashMap::new();
                let mut closing = false;
                loop {
                    let message = if pending.is_empty() {
                        receiver.recv().ok()
                    } else {
                        match receiver.recv_timeout(Duration::from_millis(100)) {
                            Ok(message) => Some(message),
                            Err(RecvTimeoutError::Timeout) => None,
                            Err(RecvTimeoutError::Disconnected) => {
                                closing = true;
                                None
                            }
                        }
                    };
                    match message {
                        Some(Message::Card(card)) => {
                            pending.insert(card.session.clone(), *card);
                        }
                        Some(Message::Shutdown) => closing = true,
                        None if pending.is_empty() => break,
                        None => {}
                    }
                    let due = pending
                        .keys()
                        .filter(|session| {
                            closing
                                || last
                                    .get(*session)
                                    .is_none_or(|(at, _)| at.elapsed() >= debounce)
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    for session in due {
                        let Some(card) = pending.remove(&session) else {
                            continue;
                        };
                        let fingerprint = card.fingerprint();
                        if last
                            .get(&session)
                            .is_some_and(|(_, prior)| *prior == fingerprint)
                        {
                            continue;
                        }
                        let outcome = write(&card);
                        let counter = if outcome == WriteOutcome::Written {
                            &thread_stats.written
                        } else {
                            &thread_stats.skipped
                        };
                        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if outcome == WriteOutcome::Written {
                            last.insert(session, (Instant::now(), fingerprint));
                        }
                    }
                    if closing && pending.is_empty() {
                        break;
                    }
                }
            })
            .ok();
        Self {
            sender: Mutex::new(Some(sender)),
            handle: Mutex::new(handle),
            stats,
        }
    }

    /// Debounce from `TEKES_KERNEL_CARD_DEBOUNCE_MS` (tests use 0), else one second.
    pub(crate) fn spawn_from_env() -> Self {
        let debounce = std::env::var("TEKES_KERNEL_CARD_DEBOUNCE_MS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map_or(DEFAULT_DEBOUNCE, Duration::from_millis);
        Self::spawn(debounce)
    }

    pub(crate) fn submit(&self, card: PreparedCard) {
        let sender = self
            .sender
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(sender) = sender.as_ref() {
            let _ = sender.send(Message::Card(Box::new(card)));
        }
    }

    /// Flushes pending cards and stops the thread. Idempotent.
    pub(crate) fn shutdown(&self) {
        if let Some(sender) = self
            .sender
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
        {
            let _ = sender.send(Message::Shutdown);
        }
        if let Some(handle) = self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
        {
            let _ = handle.join();
            if crate::boot_trace::per_item_enabled() {
                let (written, skipped) = self.counts();
                eprintln!("thread-card: written={written} skipped={skipped}");
            }
        }
    }

    /// `(written, skipped)` card counts so far.
    pub(crate) fn counts(&self) -> (u64, u64) {
        (
            self.stats
                .written
                .load(std::sync::atomic::Ordering::Relaxed),
            self.stats
                .skipped
                .load(std::sync::atomic::Ordering::Relaxed),
        )
    }
}

impl Drop for CardWriter {
    fn drop(&mut self) {
        self.shutdown();
    }
}
