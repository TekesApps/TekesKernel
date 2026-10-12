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
    /// The child-line views were folded from the child files when this entry was verified (or
    /// come from a Fresh card, which lists every child). A doorbell fold of the main ledger
    /// carries the previous child views forward and clears this: a child may have changed since.
    pub children_exact: bool,
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
    /// The folder is known to exist but its card has not been read yet (it arrived through a
    /// folder event, which must not do I/O under the store's locks). `Catalog::get` reads it.
    pub unread: bool,
}

impl Entry {
    fn cold() -> Self {
        Self {
            trust: Trust::Cold,
            view: None,
            unread: false,
        }
    }

    fn unread() -> Self {
        Self {
            trust: Trust::Cold,
            view: None,
            unread: true,
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
            unread: false,
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
            .filter(|entry| !entry.unread)
        {
            return Some(entry.clone());
        }
        let entry = self.read_entry(id)?;
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match entries.get(id) {
            // Someone resolved it meanwhile.
            Some(existing) if !existing.unread => Some(existing.clone()),
            _ => {
                // The card was read without a lock: the folder may have been archived or removed
                // since, and its folder event may already have run. Insert only if it is still
                // here (the event and this check both take the write lock; O-final m2).
                if !self.folder(id).is_dir() {
                    entries.remove(id);
                    return None;
                }
                entries.insert(id.to_owned(), entry.clone());
                Some(entry)
            }
        }
    }

    /// A folder arrived in `threads/` (a folder event): remember that it exists without reading
    /// anything. The card is read by the first `get`, outside the store's locks. A folder that
    /// was seen before is replaced (an id can be reused by a different thread).
    pub(crate) fn note_present(&self, id: &str) {
        self.entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(id.to_owned(), Entry::unread());
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
    /// corrupt until `forget` (a quarantined thread is not revived by a doorbell). A fold older
    /// than the one already recorded (two folds raced and the older finished last) is refused.
    /// Returns whether the view was recorded.
    pub(crate) fn set_fresh_main(&self, id: &str, mut view: EntryView) -> bool {
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(previous) = entries.get(id) {
            if matches!(previous.trust, Trust::Corrupt(_)) {
                return false;
            }
            if let Some(previous_view) = &previous.view {
                let recorded = previous_view.main().map_or(0, |main| main.last_seq);
                let incoming = view.main().map_or(0, |main| main.last_seq);
                if previous.trust == Trust::Fresh && recorded > incoming {
                    return false;
                }
                view.lines
                    .extend(previous_view.lines.iter().skip(1).cloned());
                // The previous child views are carried, not refolded: they may be stale.
                view.children_exact = false;
            }
        }
        entries.insert(
            id.to_owned(),
            Entry {
                trust: Trust::Fresh,
                view: Some(Arc::new(view)),
                unread: false,
            },
        );
        true
    }

    /// Brings a Fresh entry up to a newer fold without rebuilding its card view (the cost the
    /// doorbell path must not pay on every fold, K6): the lifecycle facts, `last_seq`, `updatedAt`
    /// and `blank` come straight from the fold; the title, identity profile and request holds
    /// are carried over because none of the records added since can have changed them.
    /// Returns false when the entry cannot be advanced that way (no Fresh view yet, or a new
    /// record may change a carried-over field); the caller then builds the full view.
    pub(crate) fn refresh_light(&self, id: &str, projection: &schema::LedgerProjection) -> bool {
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(entry) = entries.get_mut(id) else {
            return false;
        };
        let (Trust::Fresh, Some(view)) = (&entry.trust, &entry.view) else {
            return false;
        };
        let Some(main) = view.main() else {
            return false;
        };
        if projection.last_seq < main.last_seq {
            // A fold older than the recorded one: nothing to add, and nothing to rebuild.
            return true;
        }
        let first_new = usize::try_from(main.last_seq).unwrap_or(usize::MAX);
        let Some(new_events) = projection.events.get(first_new..) else {
            return false;
        };
        if new_events.iter().any(|event| {
            event.string_field("title").is_some()
                || matches!(
                    event.kind(),
                    schema::EventKind::State
                        | schema::EventKind::ApprovalRequest
                        | schema::EventKind::ApprovalResponse
                        | schema::EventKind::ToolResult
                )
        }) {
            return false;
        }
        let Ok(updated_ms) = endpoint::card_updated_ms(projection) else {
            return false;
        };
        let mut next = (**view).clone();
        next.children_exact = false;
        next.summary.updated_ms = updated_ms;
        next.summary.blank = projection.lifecycle.latest_turn.is_none();
        if let Some(line) = next.lines.first_mut() {
            line.last_seq = projection.last_seq;
            line.facts = projection.lifecycle.clone();
        }
        entry.view = Some(Arc::new(next));
        true
    }

    /// Records the child lines folded from the files (verification). Returns false when the
    /// entry is not a Fresh view any more.
    pub(crate) fn set_children(&self, id: &str, children: Vec<LineView>) -> bool {
        let mut entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(entry) = entries.get_mut(id) else {
            return false;
        };
        let (Trust::Fresh, Some(view)) = (&entry.trust, &entry.view) else {
            return false;
        };
        let mut next = (**view).clone();
        next.lines.truncate(1);
        next.lines.extend(children);
        next.children_exact = true;
        entry.view = Some(Arc::new(next));
        true
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
                unread: false,
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
        children_exact: true,
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
