//! The only feed of thread-folder membership changes (G-BOOT-1, ADDENDUM-03 C1 / -04 K1).
//!
//! Every way a thread folder appears in `threads/`, moves between `threads/` and `archive/`, or
//! leaves `threads/` passes through one of a handful of `store` primitives, each of which emits
//! exactly one event here after the change is durable. Consumers (the admitted set, the
//! catalog, the request index) subscribe instead of enumerating callers, so a future creator of
//! folders cannot be forgotten. Events carry the storage root: the observer registry is
//! process-wide and tests run several hosts in one process.
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Area {
    Threads,
    Archive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoveReason {
    /// An ephemeral thread was discarded or swept (`threads/<id>` -> `.rewrite-trash/discard-<id>`).
    Discarded,
    /// A redact retired its source thread.
    RedactRetired,
    /// A retired folder's tombstone was deleted (informational: it had already left the catalog).
    TombstoneDeleted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderEvent {
    /// A folder was published into `threads/` (create, staged-create completion, fork or redact
    /// destination).
    Published { root: PathBuf, id: String },
    /// A folder moved between areas (archive / unarchive).
    Moved {
        root: PathBuf,
        id: String,
        from: Area,
        to: Area,
    },
    /// A folder left `threads/` without arriving in `archive/`.
    Removed {
        root: PathBuf,
        id: String,
        reason: RemoveReason,
    },
}

impl FolderEvent {
    #[must_use]
    pub fn root(&self) -> &Path {
        match self {
            Self::Published { root, .. }
            | Self::Moved { root, .. }
            | Self::Removed { root, .. } => root,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Published { id, .. } | Self::Moved { id, .. } | Self::Removed { id, .. } => id,
        }
    }
}

/// The spelling of a storage root events are keyed by: the canonical path (symlinks and `..`
/// resolved), or the path as given when it cannot be resolved. Every component that opens the
/// same root under a different spelling must still see the same admission set.
#[must_use]
pub fn canonical_root(root: &Path) -> PathBuf {
    // Resolved once per spelling: `is_admitted` runs per session per sweep tick, and a storage
    // root does not move while the process runs.
    static MEMO: OnceLock<Mutex<HashMap<PathBuf, PathBuf>>> = OnceLock::new();
    let memo = MEMO.get_or_init(Mutex::default);
    if let Some(known) = memo
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(root)
    {
        return known.clone();
    }
    match std::fs::canonicalize(root) {
        Ok(resolved) => {
            memo.lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(root.to_path_buf(), resolved.clone());
            resolved
        }
        // A root that cannot be resolved yet (it does not exist) keeps its spelling and is
        // looked up again next time: a failure is never remembered.
        Err(_) => root.to_path_buf(),
    }
}

impl FolderEvent {
    fn with_root(self, new_root: PathBuf) -> Self {
        match self {
            Self::Published { id, .. } => Self::Published { root: new_root, id },
            Self::Moved { id, from, to, .. } => Self::Moved {
                root: new_root,
                id,
                from,
                to,
            },
            Self::Removed { id, reason, .. } => Self::Removed {
                root: new_root,
                id,
                reason,
            },
        }
    }
}

/// An observer returns `false` to unsubscribe (typically because its owner was dropped).
pub type Observer = Arc<dyn Fn(&FolderEvent) -> bool + Send + Sync>;

#[derive(Default)]
struct Registry {
    observers: Vec<Observer>,
    /// Roots whose admission recording is armed, with the ids admitted since.
    admission: HashMap<PathBuf, HashSet<String>>,
}

fn registry() -> &'static Mutex<Registry> {
    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::default()))
}

fn lock() -> std::sync::MutexGuard<'static, Registry> {
    registry()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Registers an observer for all roots; it must filter on `FolderEvent::root`.
pub fn subscribe(observer: Observer) {
    lock().observers.push(observer);
}

/// Starts recording admissions for `root`. Folders published before this call (startup recovery
/// of pending operations) are deliberately **not** admitted: they are pre-existing sessions.
pub fn arm_admission(root: &Path) {
    lock().admission.entry(canonical_root(root)).or_default();
}

/// True for an id published, moved into `threads/`, or created by this process since
/// `arm_admission(root)`.
#[must_use]
pub fn is_admitted(root: &Path, id: &str) -> bool {
    let root = canonical_root(root);
    lock()
        .admission
        .get(&root)
        .is_some_and(|admitted| admitted.contains(id))
}

/// The ids admitted for `root` since `arm_admission`.
#[must_use]
pub fn admitted_ids(root: &Path) -> Vec<String> {
    let root = canonical_root(root);
    lock()
        .admission
        .get(&root)
        .map(|admitted| admitted.iter().cloned().collect())
        .unwrap_or_default()
}

pub(crate) fn emit(event: FolderEvent) {
    let event = {
        let root = canonical_root(event.root());
        event.with_root(root)
    };
    let observers = {
        let mut registry = lock();
        if let Some(admitted) = registry.admission.get_mut(event.root()) {
            match &event {
                FolderEvent::Published { id, .. }
                | FolderEvent::Moved {
                    id,
                    to: Area::Threads,
                    ..
                } => {
                    admitted.insert(id.clone());
                }
                // A removed id is no longer an active session of this process; reuse of the
                // id is a new publication and is admitted again.
                FolderEvent::Removed { id, .. } => {
                    admitted.remove(id);
                }
                FolderEvent::Moved {
                    to: Area::Archive, ..
                } => {}
            }
        }
        registry.observers.clone()
    };
    // Observers run outside the registry lock, so they may call `is_admitted`.
    let gone = observers
        .iter()
        .filter(|observer| !observer(&event))
        .cloned()
        .collect::<Vec<_>>();
    if !gone.is_empty() {
        lock()
            .observers
            .retain(|observer| !gone.iter().any(|dead| Arc::ptr_eq(dead, observer)));
    }
}

/// Test helper: a shared record of events for one root.
#[must_use]
pub fn record_events(root: &Path) -> Arc<Mutex<Vec<FolderEvent>>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let root = canonical_root(root);
    subscribe(Arc::new(move |event| {
        if event.root() == root {
            sink.lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(event.clone());
        }
        Arc::strong_count(&sink) > 1
    }));
    seen
}

#[cfg(test)]
mod tests;
