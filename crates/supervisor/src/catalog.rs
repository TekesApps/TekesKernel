//! The in-memory catalog of active threads (G-BOOT-1 S3, DESIGN §4, ADDENDUM-01 §1.4).
//!
//! Boot does not read it. After `ready`, a background task loads it (`ensure_loaded`): one
//! `read_dir(threads/)` and one small card read plus a content-identity check per thread, never a
//! decode. A reader that needs one thread calls `get(id)` (reads just that folder); a reader that
//! needs the List calls `ensure_loaded()` (single flight) and then `snapshot()`.
//!
//! An entry is a *view* of a thread, not its truth. `Fresh` means a full fold of the whole
//! ledger produced the view; `Ahead` that the view is the verified prefix a card describes
//! while newer bytes exist; `Cold` that nothing trustworthy is known; `Corrupt` that the thread is
//! quarantined.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, RwLock};

use schema::LifecycleFacts;
use store::card::{CardSummary, Freshness, Hold, JournalInput};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Trust {
    Fresh,
    Ahead,
    Cold,
    Corrupt(String),
}

#[derive(Clone, Debug)]
/// `file`, `line`, `parent_file`, `holds` are read by the S4 request index and baselines.
#[allow(dead_code)]
pub(crate) struct LineView {
    pub file: String,
    pub last_seq: u64,
    pub line: String,
    pub parent_file: Option<String>,
    pub facts: LifecycleFacts,
    pub holds: Vec<Hold>,
}

/// Everything the List and the sweeps need about a thread without opening its ledger.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct EntryView {
    pub summary: CardSummary,
    /// Main line first, then children by name.
    pub lines: Vec<LineView>,
    pub journal: Option<JournalInput>,
}

impl EntryView {
    pub(crate) fn main(&self) -> Option<&LineView> {
        self.lines.first()
    }

    pub(crate) fn stop_active(&self) -> bool {
        self.lines.iter().any(|line| line.facts.stop_active)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Entry {
    pub trust: Trust,
    pub view: Option<Arc<EntryView>>,
}

impl Entry {
    fn cold() -> Self {
        Self {
            trust: Trust::Cold,
            view: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Load {
    NotStarted,
    Loading,
    Loaded,
}

pub(crate) struct Catalog {
    root: PathBuf,
    entries: RwLock<BTreeMap<String, Entry>>,
    load: Mutex<Load>,
    loaded: Condvar,
}

impl Catalog {
    pub(crate) fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            entries: RwLock::new(BTreeMap::new()),
            load: Mutex::new(Load::NotStarted),
            loaded: Condvar::new(),
        }
    }

    fn folder(&self, id: &str) -> PathBuf {
        self.root.join("threads").join(id)
    }

    /// Reads one folder's card and classifies it. `None` when the folder does not exist.
    fn read_entry(&self, id: &str) -> Option<Entry> {
        let folder = self.folder(id);
        if !folder.is_dir() {
            return None;
        }
        let Some(card) = store::card::read_card(&folder) else {
            return Some(Entry::cold());
        };
        let Ok(classified) = store::card::classify(&folder, &card) else {
            return Some(Entry::cold());
        };
        let trust = match classified.freshness() {
            Freshness::Fresh => Trust::Fresh,
            Freshness::Ahead => Trust::Ahead,
            Freshness::Cold => return Some(Entry::cold()),
        };
        Some(Entry {
            trust,
            view: Some(Arc::new(view_of_card(&card))),
        })
    }

    /// The entry for one thread, reading just its folder when it has not been seen. This never
    /// triggers the full load.
    pub(crate) fn get(&self, id: &str) -> Option<Entry> {
        if let Some(entry) = self
            .entries
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(id)
        {
            return Some(entry.clone());
        }
        let entry = self.read_entry(id)?;
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        Some(entries.entry(id.to_owned()).or_insert(entry).clone())
    }

    /// Loads every thread once (single flight). A concurrent caller waits for the loader.
    pub(crate) fn ensure_loaded(&self) {
        let mut state = self
            .load
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            match *state {
                Load::Loaded => return,
                Load::Loading => {
                    state = self
                        .loaded
                        .wait(state)
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                }
                Load::NotStarted => {
                    *state = Load::Loading;
                    break;
                }
            }
        }
        drop(state);
        let started = std::time::Instant::now();
        let mut ids = Vec::new();
        if let Ok(entries) = std::fs::read_dir(self.root.join("threads")) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if endpoint::validate_session_id(&name).is_ok()
                    && entry.file_type().is_ok_and(|kind| kind.is_dir())
                {
                    ids.push(name);
                }
            }
        }
        ids.sort();
        for id in &ids {
            let _ = self.get(id);
        }
        if crate::boot_trace::per_item_enabled() {
            let entries = self
                .entries
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let count = |trust: &dyn Fn(&Trust) -> bool| {
                entries.values().filter(|entry| trust(&entry.trust)).count()
            };
            eprintln!(
                "catalog-ready: threads={} fresh={} ahead={} cold={} corrupt={} ms={}",
                entries.len(),
                count(&|t| *t == Trust::Fresh),
                count(&|t| *t == Trust::Ahead),
                count(&|t| *t == Trust::Cold),
                count(&|t| matches!(t, Trust::Corrupt(_))),
                started.elapsed().as_millis()
            );
        }
        *self
            .load
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Load::Loaded;
        self.loaded.notify_all();
    }

    pub(crate) fn is_loaded(&self) -> bool {
        *self
            .load
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            == Load::Loaded
    }

    /// What is known right now, without loading (used by sweeps before the load finished).
    pub(crate) fn entries_now(&self) -> Vec<(String, Entry)> {
        self.entries
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .map(|(id, entry)| (id.clone(), entry.clone()))
            .collect()
    }

    /// Records a fold of the main ledger. Child-line views are kept from the previous entry
    /// (they are folded by the card writer, not on the doorbell path); a corrupt entry stays
    /// corrupt until `forget` (a quarantined thread is not revived by a doorbell).
    pub(crate) fn set_fresh_main(&self, id: &str, mut view: EntryView) {
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(previous) = entries.get(id) {
            if matches!(previous.trust, Trust::Corrupt(_)) {
                return;
            }
            if let Some(previous) = &previous.view {
                view.lines.extend(previous.lines.iter().skip(1).cloned());
            }
        }
        entries.insert(
            id.to_owned(),
            Entry {
                trust: Trust::Fresh,
                view: Some(Arc::new(view)),
            },
        );
    }

    pub(crate) fn mark_corrupt(&self, id: &str, reason: &str) {
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let view = entries.get(id).and_then(|entry| entry.view.clone());
        entries.insert(
            id.to_owned(),
            Entry {
                trust: Trust::Corrupt(reason.to_owned()),
                view,
            },
        );
    }

    /// A folder was published (or moved) into `threads/`; its entry is read on first use.
    pub(crate) fn forget(&self, id: &str) {
        self.entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(id);
    }

    /// Ids of unverified entries, newest ledger first (the hydrator's work list).
    pub(crate) fn unverified(&self) -> Vec<String> {
        let mut ids = self
            .entries_now()
            .into_iter()
            .filter(|(_, entry)| matches!(entry.trust, Trust::Cold | Trust::Ahead))
            .map(|(id, _)| id)
            .collect::<Vec<_>>();
        ids.sort_by_key(|id| {
            std::cmp::Reverse(
                std::fs::metadata(self.folder(id).join("main.jsonl"))
                    .and_then(|metadata| metadata.modified())
                    .ok(),
            )
        });
        ids
    }
}

fn view_of_card(card: &store::card::ThreadCard) -> EntryView {
    EntryView {
        summary: card.summary.clone(),
        lines: card
            .lines
            .iter()
            .map(|line| LineView {
                file: line.file.clone(),
                last_seq: line.last_seq,
                line: line.line.clone(),
                parent_file: line.parent_file.clone(),
                facts: line.facts.clone(),
                holds: line.holds.clone(),
            })
            .collect(),
        journal: card.journal.as_ref().map(|journal| JournalInput {
            len: journal.len,
            rows: journal.rows,
            last_endpoint_seq: journal.last_endpoint_seq,
            last_kernel_seq: journal.last_kernel_seq,
        }),
    }
}
