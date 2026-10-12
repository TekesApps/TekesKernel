//! Builds and writes `thread-card.json` (G-BOOT-1 S2). Nothing reads the card yet: this slice
//! only produces it, from folds the supervisor has already done, so that the List and the thread
//! open paths can later skip decoding. See `store::card` for the invariants (C1, content
//! identity, no locks).
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender, SyncSender};
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

/// A card ready to write. `lines` starts with the main line; child lines are added by the
/// writer thread (`complete_children`), never on the doorbell path.
pub(crate) struct PreparedCard {
    session: String,
    folder: PathBuf,
    written: String,
    lines: Vec<PreparedLine>,
    summary: CardSummary,
    journal: Option<JournalInput>,
}

impl PreparedCard {
    /// The catalog view of this fold (no ledger bytes).
    pub(crate) fn view(&self) -> crate::catalog::EntryView {
        crate::catalog::EntryView {
            summary: self.summary.clone(),
            lines: self
                .lines
                .iter()
                .map(|line| crate::catalog::LineView {
                    file: line.file.clone(),
                    last_seq: line.last_seq,
                    line: line.line.clone(),
                    parent_file: line.parent_file.clone(),
                    facts: line.facts.clone(),
                    holds: line.holds.clone(),
                })
                .collect(),
            journal: self.journal,
        }
    }

    /// Adds the journal block once the journal has been reconciled with the folded ledger.
    pub(crate) fn with_journal(mut self, journal: Option<JournalInput>) -> Self {
        self.journal = journal;
        self
    }

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
                    // kebab-case (`allowed-once`, `rejected`, `cancelled`, `answered`)
                    outcome: serde_json::to_value(outcome)
                        .ok()
                        .and_then(|value| value.as_str().map(str::to_owned))
                        .unwrap_or_default(),
                }),
            })
            .collect(),
    )
}

/// The journal block of a card from one cache snapshot (`len`, rows and last seqs agree).
pub(crate) fn journal_input(journal: &EndpointJournal) -> Option<JournalInput> {
    let tail = journal.tail_summary().ok()?;
    Some(JournalInput {
        len: tail.len,
        rows: tail.rows,
        last_endpoint_seq: tail
            .last_seq
            .map_or(-1, |seq| i64::try_from(seq).unwrap_or(i64::MAX)),
        last_kernel_seq: tail.last_kernel_seq,
    })
}

fn main_line(
    session: &str,
    mut bytes: Vec<u8>,
    valid_bytes: usize,
    projection: &LedgerProjection,
) -> Option<(PreparedLine, CardSummary)> {
    bytes.truncate(valid_bytes);
    let summary = endpoint::card_summary(projection).ok()?;
    Some((
        PreparedLine {
            file: "main.jsonl".to_owned(),
            prefix: bytes,
            last_seq: projection.last_seq,
            line: session.to_owned(),
            parent_file: None,
            facts: projection.lifecycle.clone(),
            holds: holds_of(session, None, projection)?,
        },
        summary,
    ))
}

/// Builds the card for one thread from a **clean** fold of its main ledger that the caller has
/// already done (`bytes` is the folded file content, `valid_bytes` its verified prefix). Child
/// lines are *not* read here. Returns `None` when the thread cannot be described: a missing
/// card is always safe.
pub(crate) fn prepare(
    session: &str,
    folder: PathBuf,
    bytes: Vec<u8>,
    valid_bytes: usize,
    projection: &LedgerProjection,
    journal: Option<JournalInput>,
) -> Option<PreparedCard> {
    let (line, summary) = main_line(session, bytes, valid_bytes, projection)?;
    Some(PreparedCard {
        session: session.to_owned(),
        written: crate::host_runtime::system_timestamp().ok()?,
        lines: vec![line],
        summary,
        journal,
        folder,
    })
}

/// Reads and folds every child ledger of the thread folder (writer thread only). A child that
/// cannot be described is left out; the reader then treats the thread as Ahead for that line.
fn complete_children(card: &mut PreparedCard) {
    let Ok(entries) = std::fs::read_dir(&card.folder) else {
        return;
    };
    let mut children = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            (store::card::valid_line_name(&name)
                && name != "main.jsonl"
                && entry.file_type().is_ok_and(|kind| kind.is_file()))
            .then_some(name)
        })
        .collect::<Vec<_>>();
    children.sort();
    for name in children {
        let Ok(mut prefix) = std::fs::read(card.folder.join(&name)) else {
            continue;
        };
        let scan = store::scan_valid_prefix(&prefix, 1);
        let Ok(valid) = usize::try_from(scan.valid_bytes) else {
            continue;
        };
        let Some(child) = scan.projection else {
            continue;
        };
        let Some(genesis) = child.events.first() else {
            continue;
        };
        let parent_file = serde_json::to_value(genesis.raw()).ok().and_then(|value| {
            value
                .get("parent")?
                .get("file")?
                .as_str()
                .map(str::to_owned)
        });
        let (Some(line), Some(holds)) = (
            genesis.string_field("thread").map(str::to_owned),
            holds_of(&card.session, Some(&name), &child),
        ) else {
            continue;
        };
        prefix.truncate(valid);
        card.lines.push(PreparedLine {
            file: name,
            prefix,
            last_seq: child.last_seq,
            line,
            parent_file,
            facts: child.lifecycle,
            holds,
        });
    }
}

/// Re-folds the main ledger on the writer thread for a thread that was marked dirty while its
/// debounce window was open. `None` when the file has a torn tail or cannot be described.
fn refold(session: &str, folder: PathBuf, journal: Option<JournalInput>) -> Option<PreparedCard> {
    let bytes = std::fs::read(folder.join("main.jsonl")).ok()?;
    let scan = store::scan_valid_prefix(&bytes, 1);
    if scan.needs_repair() {
        return None;
    }
    let valid = usize::try_from(scan.valid_bytes).ok()?;
    let projection = scan.projection?;
    prepare(session, folder, bytes, valid, &projection, journal)
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

/// The one limiter for everything that fsyncs on behalf of cards (E831 E4): the card writer
/// charges one unit per ledger `F_FULLFSYNC` it performs, the hydrator charges the tail repairs
/// it performs. It only ever delays the *writer thread* (`delay`); callers on request paths
/// (readiness, the first List, a thread open) only `charge` and never wait.
pub(crate) struct SyncGovernor {
    gap: Duration,
    next_allowed: Mutex<Instant>,
}

impl SyncGovernor {
    pub(crate) fn new(max_syncs_per_sec: u32) -> Self {
        Self {
            gap: if max_syncs_per_sec == 0 {
                Duration::ZERO
            } else {
                Duration::from_secs(1) / max_syncs_per_sec
            },
            next_allowed: Mutex::new(Instant::now()),
        }
    }

    /// Records `syncs` fsyncs just performed.
    pub(crate) fn charge(&self, syncs: u32) {
        let mut next = self
            .next_allowed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let base = (*next).max(Instant::now());
        *next = base + self.gap * syncs;
    }

    /// How long the writer thread must wait before its next fsync-bearing write.
    pub(crate) fn delay(&self) -> Duration {
        self.next_allowed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .saturating_duration_since(Instant::now())
    }
}

enum Message {
    Card(Box<PreparedCard>),
    /// A doorbell arrived inside the debounce window; the writer re-folds when it is due.
    Dirty {
        session: String,
        folder: PathBuf,
        journal: Option<JournalInput>,
    },
    Shutdown(Instant),
}

/// How many times a re-fold that found a torn tail (a worker mid-append) is retried.
const REFOLD_RETRIES: u8 = 5;
const REFOLD_RETRY_DELAY: Duration = Duration::from_millis(200);

enum Pending {
    Card(Box<PreparedCard>),
    Dirty {
        folder: PathBuf,
        journal: Option<JournalInput>,
        /// Re-fold attempts already spent on a torn read, and the earliest next attempt.
        attempts: u8,
        not_before: Option<Instant>,
    },
}

type WriteFn = Arc<dyn Fn(&PreparedCard) -> WriteOutcome + Send + Sync>;

/// Writer tuning. `max_per_sec == 0` means unpaced.
#[derive(Clone, Copy)]
pub(crate) struct Tuning {
    pub debounce: Duration,
    pub max_per_sec: u32,
    pub shutdown_budget: Duration,
}

impl Tuning {
    pub(crate) fn from_env() -> Self {
        let number = |name: &str| {
            std::env::var(name)
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
        };
        Self {
            debounce: number("TEKES_KERNEL_CARD_DEBOUNCE_MS")
                .map_or(DEFAULT_DEBOUNCE, Duration::from_millis),
            max_per_sec: number("TEKES_KERNEL_CARD_MAX_PER_SEC")
                .map_or(DEFAULT_MAX_PER_SEC, |value| {
                    u32::try_from(value).unwrap_or(u32::MAX)
                }),
            shutdown_budget: DEFAULT_SHUTDOWN_BUDGET,
        }
    }
}

pub(crate) const DEFAULT_MAX_PER_SEC: u32 = 8;
pub(crate) const DEFAULT_SHUTDOWN_BUDGET: Duration = Duration::from_secs(2);

/// Counters for tests and tracing.
#[derive(Default)]
struct CardStats {
    written: AtomicU64,
    skipped: AtomicU64,
    dropped_at_shutdown: AtomicU64,
}

/// A background thread that coalesces card jobs per thread and writes the newest one at most
/// once per debounce interval and at most `max_per_sec` cards per second overall, so the
/// `F_FULLFSYNC` of the card protocol never competes with workers for the device. The doorbell
/// path only asks `is_due` (one lock, one map lookup); the shutdown flush has a time budget.
pub(crate) struct CardWriter {
    governor: Arc<SyncGovernor>,
    sender: Mutex<Option<Sender<Message>>>,
    handle: Mutex<Option<JoinHandle<()>>>,
    done: Mutex<Option<mpsc::Receiver<()>>>,
    claimed: Mutex<HashMap<String, Instant>>,
    stats: Arc<CardStats>,
    tuning: Tuning,
}

impl CardWriter {
    pub(crate) fn spawn(tuning: Tuning) -> Self {
        Self::spawn_with(
            tuning,
            Arc::new(SyncGovernor::new(tuning.max_per_sec)),
            Arc::new(write),
        )
    }

    pub(crate) fn spawn_from_env() -> Self {
        Self::spawn(Tuning::from_env())
    }

    /// The limiter shared with the hydrator.
    pub(crate) fn governor(&self) -> Arc<SyncGovernor> {
        Arc::clone(&self.governor)
    }

    fn spawn_with(tuning: Tuning, governor: Arc<SyncGovernor>, write_card: WriteFn) -> Self {
        let (sender, receiver) = mpsc::channel::<Message>();
        let (done_sender, done) = mpsc::sync_channel::<()>(1);
        let stats = Arc::new(CardStats::default());
        let thread_stats = Arc::clone(&stats);
        let thread_governor = Arc::clone(&governor);
        let handle = std::thread::Builder::new()
            .name("thread-card-writer".to_owned())
            .spawn(move || {
                run(
                    &receiver,
                    tuning,
                    &thread_governor,
                    &write_card,
                    &thread_stats,
                    &done_sender,
                );
            })
            .ok();
        Self {
            governor,
            sender: Mutex::new(Some(sender)),
            handle: Mutex::new(handle),
            done: Mutex::new(Some(done)),
            claimed: Mutex::new(HashMap::new()),
            stats,
            tuning,
        }
    }

    /// Doorbell-path gate: true (and the window restarts) when no card for this thread was
    /// requested within the debounce interval. Cheap by design: nothing is prepared unless
    /// this returns true (K6).
    pub(crate) fn is_due(&self, session: &str) -> bool {
        let mut claimed = self
            .claimed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let now = Instant::now();
        match claimed.get(session) {
            Some(at) if now.duration_since(*at) < self.tuning.debounce => false,
            _ => {
                claimed.insert(session.to_owned(), now);
                true
            }
        }
    }

    /// Opens a new debounce window for a thread (the hydrator uses it so the card it is about to
    /// produce is written promptly instead of waiting out a window).
    pub(crate) fn reset_window(&self, session: &str) {
        self.claimed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(session);
    }

    pub(crate) fn submit(&self, card: PreparedCard) {
        self.send(Message::Card(Box::new(card)));
    }

    pub(crate) fn mark_dirty(&self, session: &str, folder: PathBuf, journal: Option<JournalInput>) {
        self.send(Message::Dirty {
            session: session.to_owned(),
            folder,
            journal,
        });
    }

    fn send(&self, message: Message) {
        let sender = self
            .sender
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(sender) = sender.as_ref() {
            let _ = sender.send(message);
        }
    }

    /// Flushes pending cards for at most the shutdown budget and stops the thread; whatever is
    /// left is dropped (a missing or older card is always safe). Idempotent. A writer stuck in
    /// a slow sync is abandoned rather than awaited past the budget.
    pub(crate) fn shutdown(&self) {
        let Some(sender) = self
            .sender
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
        else {
            return;
        };
        let budget = self.tuning.shutdown_budget;
        let _ = sender.send(Message::Shutdown(Instant::now() + budget));
        let done = self
            .done
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        let finished = done.is_none_or(|done| {
            done.recv_timeout(budget + Duration::from_millis(500))
                .is_ok()
        });
        let handle = self
            .handle
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        if finished {
            if let Some(handle) = handle {
                let _ = handle.join();
            }
        } else {
            eprintln!("thread-card: shutdown budget exceeded; abandoning the writer");
        }
        if crate::boot_trace::per_item_enabled() {
            let (written, skipped) = self.counts();
            eprintln!(
                "thread-card: written={written} skipped={skipped} dropped_at_shutdown={}",
                self.stats.dropped_at_shutdown.load(Ordering::Relaxed)
            );
        }
    }

    /// `(written, skipped)` card counts so far.
    pub(crate) fn counts(&self) -> (u64, u64) {
        (
            self.stats.written.load(Ordering::Relaxed),
            self.stats.skipped.load(Ordering::Relaxed),
        )
    }
}

impl Drop for CardWriter {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn absorb(
    message: Message,
    pending: &mut HashMap<String, Pending>,
    deadline: &mut Option<Instant>,
) {
    match message {
        Message::Card(card) => {
            pending.insert(card.session.clone(), Pending::Card(card));
        }
        Message::Dirty {
            session,
            folder,
            journal,
        } => {
            pending.insert(
                session,
                Pending::Dirty {
                    folder,
                    journal,
                    attempts: 0,
                    not_before: None,
                },
            );
        }
        Message::Shutdown(by) => *deadline = Some(by),
    }
}

fn is_idle(pending: &HashMap<String, Pending>, deadline: Option<Instant>) -> bool {
    pending.is_empty() && deadline.is_none()
}

fn run(
    receiver: &mpsc::Receiver<Message>,
    tuning: Tuning,
    governor: &SyncGovernor,
    write_card: &WriteFn,
    stats: &CardStats,
    done: &SyncSender<()>,
) {
    let mut pending: HashMap<String, Pending> = HashMap::new();
    let mut last_written: HashMap<String, (Instant, (u64, u64, u64))> = HashMap::new();
    let mut deadline: Option<Instant> = None;
    loop {
        let message = if pending.is_empty() && deadline.is_none() {
            receiver.recv().ok()
        } else {
            match receiver.recv_timeout(Duration::from_millis(50)) {
                Ok(message) => Some(message),
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => {
                    deadline.get_or_insert_with(Instant::now);
                    None
                }
            }
        };
        if let Some(message) = message {
            absorb(message, &mut pending, &mut deadline);
        }
        if is_idle(&pending, deadline) {
            break;
        }
        let closing = deadline.is_some();
        let now = Instant::now();
        if closing && deadline.is_some_and(|by| now >= by) {
            stats
                .dropped_at_shutdown
                .fetch_add(pending.len() as u64, Ordering::Relaxed);
            break;
        }
        let mut due = pending
            .iter()
            .filter(|(session, job)| {
                let retry_ready = match job {
                    Pending::Dirty {
                        not_before: Some(at),
                        ..
                    } => closing || now >= *at,
                    _ => true,
                };
                retry_ready
                    && (closing
                        || last_written
                            .get(*session)
                            .is_none_or(|(at, _)| at.elapsed() >= tuning.debounce))
            })
            .map(|(session, _)| session.clone())
            .collect::<Vec<_>>();
        due.sort();
        for session in due {
            // A long run of writes must still notice a shutdown request promptly.
            while let Ok(message) = receiver.try_recv() {
                absorb(message, &mut pending, &mut deadline);
            }
            let closing = deadline.is_some();
            if closing && deadline.is_some_and(|by| Instant::now() >= by) {
                break;
            }
            // The shared limiter paces fsync-bearing writes; a shutdown flush runs at device
            // speed up to its budget.
            if !closing && !governor.delay().is_zero() {
                break;
            }
            let Some(job) = pending.remove(&session) else {
                continue;
            };
            let card = match job {
                Pending::Card(card) => Some(*card),
                Pending::Dirty {
                    folder,
                    journal,
                    attempts,
                    ..
                } => {
                    let folded = refold(&session, folder.clone(), journal);
                    if folded.is_none() {
                        // A torn read (a worker mid-append) must not lose the final state:
                        // retry a bounded number of times instead of dropping the dirty mark.
                        if attempts < REFOLD_RETRIES {
                            pending.entry(session.clone()).or_insert(Pending::Dirty {
                                folder,
                                journal,
                                attempts: attempts + 1,
                                not_before: Some(Instant::now() + REFOLD_RETRY_DELAY),
                            });
                        } else {
                            stats.skipped.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    folded
                }
            };
            let Some(mut card) = card else {
                continue;
            };
            let fingerprint = card.fingerprint();
            if last_written
                .get(&session)
                .is_some_and(|(_, prior)| *prior == fingerprint)
            {
                continue;
            }
            complete_children(&mut card);
            let syncs = u32::try_from(card.lines.len()).unwrap_or(u32::MAX);
            let outcome = write_card(&card);
            governor.charge(syncs);
            let counter = if outcome == WriteOutcome::Written {
                &stats.written
            } else {
                &stats.skipped
            };
            counter.fetch_add(1, Ordering::Relaxed);
            if outcome == WriteOutcome::Written {
                last_written.insert(session, (Instant::now(), fingerprint));
            }
        }
        if closing && pending.is_empty() {
            break;
        }
    }
    let _ = done.send(());
}

#[cfg(test)]
mod tests;
