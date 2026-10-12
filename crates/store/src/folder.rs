use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use schema::{Event, EventKind, OriginTuple};

use crate::folder_events::{Area, FolderEvent, RemoveReason};
use crate::platform::{DirectoryLock, NamedLock, open_exclusive_create};
use crate::{AssetStore, BarrierContext, LockedLedger, StoreError, probe_local_filesystem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateOutcome {
    pub thread_id: String,
    pub created: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppendOutcome {
    pub seq: u64,
    pub deduplicated: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionalAppendOutcome {
    Appended(AppendOutcome),
    Skipped,
}

#[derive(Clone, Debug)]
pub struct ThreadStore {
    root: PathBuf,
}

impl ThreadStore {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, StoreError> {
        let root = root.as_ref().to_path_buf();
        probe_local_filesystem(&root)?;
        for name in [
            "threads",
            "archive",
            "staging",
            ".create-staging",
            ".rewrite-trash",
        ] {
            let path = root.join(name);
            match fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.file_type().is_dir() => {}
                Ok(_) => {
                    return Err(StoreError::Corruption(format!(
                        "ledger directory is not a real directory: {}",
                        path.display()
                    )));
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    fs::create_dir_all(&path)?;
                }
                Err(error) => return Err(StoreError::Io(error)),
            }
        }
        Ok(Self { root })
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// True when any top-level semantic JSONL line in the active session
    /// folder has a live advisory lock holder. The probe never blocks and the
    /// process table is deliberately not consulted.
    pub fn session_has_live_line_holder(&self, thread_id: &str) -> Result<bool, StoreError> {
        let folder = self.root.join("threads").join(thread_id);
        if !folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            if !entry.file_type()?.is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("jsonl")
                // The endpoint journal is a projection with its own lock (`endpoint.lock`); no
                // worker holds a line lock on it, and a List probes every thread.
                || !crate::card::valid_line_name(&entry.file_name().to_string_lossy())
            {
                continue;
            }
            let file = File::open(entry.path())?;
            match crate::platform::try_lock_exclusive(&file) {
                Ok(()) => {}
                Err(StoreError::Busy) => return Ok(true),
                Err(error) => return Err(error),
            }
        }
        Ok(false)
    }

    pub fn create_thread(
        &self,
        thread_id: &str,
        genesis: Event,
    ) -> Result<CreateOutcome, StoreError> {
        self.create_thread_with_assets(thread_id, genesis, &[])
    }

    /// Publishes immutable content-addressed carriers before the genesis
    /// barrier and active-folder rename. Replays verify that every supplied
    /// carrier is present under the already-created identity before returning
    /// a deduplicated result.
    pub fn create_thread_with_assets(
        &self,
        thread_id: &str,
        genesis: Event,
        assets: &[Vec<u8>],
    ) -> Result<CreateOutcome, StoreError> {
        if !matches!(genesis.kind(), EventKind::Genesis)
            || genesis.seq() != 1
            || genesis.string_field("thread") != Some(thread_id)
        {
            return Err(StoreError::Corruption(
                "create requires matching seq-1 genesis".to_owned(),
            ));
        }
        let origin = genesis
            .origin_tuple()?
            .ok_or_else(|| StoreError::Corruption("genesis is missing origin tuple".to_owned()))?;
        crate::tail::check_write_versions(std::iter::once(&genesis), 1)?;
        // Membership is observed by the thread-search authority under the
        // shared form of this lock. Hold it across every active-folder
        // publication so a catalog snapshot cannot miss or duplicate a
        // concurrently created identity.
        let _catalog = NamedLock::exclusive(self.root.join(".thread-catalog.lock"))?;
        let lock_path = self.root.join(".create.lock");
        let _lock_file = open_exclusive_create(&lock_path, false)?;

        if let Some(existing) = self.find_by_origin(&origin)? {
            let active = self.root.join("threads").join(&existing);
            let archived = self.root.join("archive").join(&existing);
            let folder = if active.is_dir() {
                active
            } else if archived.is_dir() {
                archived
            } else {
                return Err(StoreError::Corruption(
                    "origin index names a missing active/archive folder".to_owned(),
                ));
            };
            crate::tail::check_ledger_write_versions(&fs::read(folder.join("main.jsonl"))?, 1)?;
            publish_create_assets(&folder.join("assets"), assets)?;
            return Ok(CreateOutcome {
                thread_id: existing,
                created: false,
            });
        }
        if self.destination_has_live_rewrite(thread_id)? {
            return Err(StoreError::ThreadCollision);
        }
        if let Some((existing, stage)) = self.find_staged_create_by_origin(&origin)? {
            let destination = self.root.join("threads").join(&existing);
            if destination.exists() || self.root.join("archive").join(&existing).exists() {
                return Err(StoreError::ThreadCollision);
            }
            crate::tail::check_ledger_write_versions(&fs::read(stage.join("main.jsonl"))?, 1)?;
            publish_create_assets(&stage.join("assets"), assets)?;
            fs::rename(&stage, &destination)?;
            File::open(self.root.join(".create-staging"))?.sync_all()?;
            File::open(self.root.join("threads"))?.sync_all()?;
            crate::folder_events::emit(FolderEvent::Published {
                root: self.root.clone(),
                id: existing.clone(),
            });
            return Ok(CreateOutcome {
                thread_id: existing,
                created: false,
            });
        }
        let destination = self.root.join("threads").join(thread_id);
        if destination.exists() || self.root.join("archive").join(thread_id).exists() {
            return Err(StoreError::ThreadCollision);
        }
        self.remove_incomplete_create_stages(thread_id)?;
        let stage = self
            .root
            .join(".create-staging")
            .join(format!("create-{thread_id}-{}", std::process::id()));
        if stage.exists() {
            return Err(StoreError::Corruption(format!(
                "live create staging path already exists: {}",
                stage.display()
            )));
        }
        fs::create_dir(&stage)?;
        fs::create_dir(stage.join("assets"))?;
        publish_create_assets(&stage.join("assets"), assets)?;
        let line = stage.join("main.jsonl");
        File::create(&line)?;
        {
            let mut ledger = LockedLedger::open(&line, 1)?;
            ledger.append_contract(genesis, BarrierContext::default())?;
        }
        File::open(&stage)?.sync_all()?;
        fs::rename(&stage, &destination)?;
        File::open(self.root.join("threads"))?.sync_all()?;
        crate::folder_events::emit(FolderEvent::Published {
            root: self.root.clone(),
            id: thread_id.to_owned(),
        });
        Ok(CreateOutcome {
            thread_id: thread_id.to_owned(),
            created: true,
        })
    }

    pub fn append_keyed<F>(
        &self,
        thread_id: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<AppendOutcome, StoreError>
    where
        F: FnOnce(u64) -> Result<Event, StoreError>,
    {
        self.append_keyed_with_projection(thread_id, origin, |seq, _| build(seq))
    }

    pub fn append_keyed_with_projection<F>(
        &self,
        thread_id: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<AppendOutcome, StoreError>
    where
        F: FnOnce(u64, &schema::LedgerProjection) -> Result<Event, StoreError>,
    {
        match self.append_keyed_with_projection_if(thread_id, origin, |seq, projection| {
            build(seq, projection).map(Some)
        })? {
            ConditionalAppendOutcome::Appended(outcome) => Ok(outcome),
            ConditionalAppendOutcome::Skipped => {
                unreachable!("unconditional keyed append cannot be skipped")
            }
        }
    }

    /// Atomically decides whether to append while holding the line lock. This
    /// is the mutation primitive for compare-and-append metadata such as an
    /// automatic title that must not overtake a manual rename.
    pub fn append_keyed_with_projection_if<F>(
        &self,
        thread_id: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<ConditionalAppendOutcome, StoreError>
    where
        F: FnOnce(u64, &schema::LedgerProjection) -> Result<Option<Event>, StoreError>,
    {
        self.append_line_keyed_with_projection_if(thread_id, "main.jsonl", origin, build)
    }

    /// Appends to a session line under the session lifecycle lock and the
    /// line's exclusive writer lock. Child lines must trace to a durable spawn
    /// in this session; a filename alone never grants mutation authority.
    /// Read a line for routing only after validating its durable spawn ancestry.
    /// The eventual writer must revalidate under its own append lock.
    pub fn validated_line_projection(
        &self,
        thread_id: &str,
        line_file: &str,
    ) -> Result<schema::LedgerProjection, StoreError> {
        if Path::new(thread_id)
            .file_name()
            .and_then(|name| name.to_str())
            != Some(thread_id)
            || matches!(thread_id, "." | "..")
        {
            return Err(StoreError::Corruption("invalid session directory".into()));
        }
        validate_line_name(line_file)?;
        if self.root.join("archive").join(thread_id).exists() {
            return Err(StoreError::Archived);
        }
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let folder = self.root.join("threads").join(thread_id);
        if !folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        let _lifecycle = DirectoryLock::shared(&folder)?;
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let path = folder.join(line_file);
        if !fs::symlink_metadata(&path)?.file_type().is_file() {
            return Err(StoreError::Corruption(
                "ledger is not a regular file".into(),
            ));
        }
        // Boot no longer repairs every tail: repair this line's partial tail if nobody holds it.
        let (_, scan) = crate::read_ledger_repaired(&path)?;
        if scan.needs_repair() {
            return Err(StoreError::Corruption(
                "ledger requires tail recovery".into(),
            ));
        }
        let projection = scan
            .projection
            .ok_or_else(|| StoreError::Corruption("empty ledger".into()))?;
        validate_line_ancestry(&folder, thread_id, line_file, Some(&projection))?;
        Ok(projection)
    }

    pub fn append_line_keyed_with_projection_if<F>(
        &self,
        thread_id: &str,
        line_file: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<ConditionalAppendOutcome, StoreError>
    where
        F: FnOnce(u64, &schema::LedgerProjection) -> Result<Option<Event>, StoreError>,
    {
        if Path::new(thread_id)
            .file_name()
            .and_then(|name| name.to_str())
            != Some(thread_id)
            || thread_id == "."
            || thread_id == ".."
        {
            return Err(StoreError::Corruption(
                "invalid session directory".to_owned(),
            ));
        }
        validate_line_name(line_file)?;
        if self.root.join("archive").join(thread_id).exists() {
            return Err(StoreError::Archived);
        }
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let folder = self.root.join("threads").join(thread_id);
        if !folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        let _lifecycle = DirectoryLock::shared(&folder)?;
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let mut ledger = LockedLedger::open(folder.join(line_file), 1)?;
        if line_file != "main.jsonl" {
            validate_line_ancestry(&folder, thread_id, line_file, ledger.projection())?;
        }
        if let Some(seq) = ledger
            .projection()
            .ok_or_else(|| StoreError::Corruption("empty thread ledger".to_owned()))?
            .origin_tuples
            .get(origin)
        {
            return Ok(ConditionalAppendOutcome::Appended(AppendOutcome {
                seq: *seq,
                deduplicated: true,
            }));
        }
        let Some(event) = build(
            ledger.next_seq(),
            ledger
                .projection()
                .ok_or_else(|| StoreError::Corruption("empty thread ledger".to_owned()))?,
        )?
        else {
            return Ok(ConditionalAppendOutcome::Skipped);
        };
        if event.seq() != ledger.next_seq()
            || event.origin_key() != Some(origin.key.as_str())
            || event.origin_tuple()?.as_ref() != Some(origin)
        {
            return Err(StoreError::Corruption(
                "keyed append builder changed seq or origin".to_owned(),
            ));
        }
        let seq = event.seq();
        ledger.append_contract(event, BarrierContext::default())?;
        Ok(ConditionalAppendOutcome::Appended(AppendOutcome {
            seq,
            deduplicated: false,
        }))
    }

    /// Keyed conditional append on the main line whose builder owns the locked
    /// ledger for the duration of the write: it may create a checkpoint and
    /// publish assets before returning the event. Same lifecycle lock, rewrite
    /// gate, origin dedup and seq/origin verification as
    /// `append_keyed_with_projection_if`. Used for supervisor-authored manual
    /// compaction on a line with no live worker (D-61 whitelist).
    pub fn append_keyed_with_ledger_if<F>(
        &self,
        thread_id: &str,
        origin: &OriginTuple,
        build: F,
    ) -> Result<ConditionalAppendOutcome, StoreError>
    where
        F: FnOnce(&mut LockedLedger) -> Result<Option<Event>, StoreError>,
    {
        if Path::new(thread_id)
            .file_name()
            .and_then(|name| name.to_str())
            != Some(thread_id)
            || thread_id == "."
            || thread_id == ".."
        {
            return Err(StoreError::Corruption(
                "invalid session directory".to_owned(),
            ));
        }
        if self.root.join("archive").join(thread_id).exists() {
            return Err(StoreError::Archived);
        }
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let folder = self.root.join("threads").join(thread_id);
        if !folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        let _lifecycle = DirectoryLock::shared(&folder)?;
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let mut ledger = LockedLedger::open(folder.join("main.jsonl"), 1)?;
        if let Some(seq) = ledger
            .projection()
            .ok_or_else(|| StoreError::Corruption("empty thread ledger".to_owned()))?
            .origin_tuples
            .get(origin)
        {
            return Ok(ConditionalAppendOutcome::Appended(AppendOutcome {
                seq: *seq,
                deduplicated: true,
            }));
        }
        let Some(event) = build(&mut ledger)? else {
            return Ok(ConditionalAppendOutcome::Skipped);
        };
        if event.seq() != ledger.next_seq()
            || event.origin_key() != Some(origin.key.as_str())
            || event.origin_tuple()?.as_ref() != Some(origin)
        {
            return Err(StoreError::Corruption(
                "keyed append builder changed seq or origin".to_owned(),
            ));
        }
        let seq = event.seq();
        ledger.append_contract(event, BarrierContext::default())?;
        Ok(ConditionalAppendOutcome::Appended(AppendOutcome {
            seq,
            deduplicated: false,
        }))
    }

    pub fn archive(&self, thread_id: &str) -> Result<(), StoreError> {
        self.move_folder(thread_id, "threads", "archive", false)
    }

    pub fn archive_nonblocking(&self, thread_id: &str) -> Result<(), StoreError> {
        self.move_folder(thread_id, "threads", "archive", true)
    }

    fn move_folder(
        &self,
        thread_id: &str,
        source_area: &str,
        destination_area: &str,
        nonblocking: bool,
    ) -> Result<(), StoreError> {
        // Search takes the shared form while enumerating both areas. This
        // makes an active/archive move one catalog transition instead of two
        // independently observable directory changes.
        let _catalog = NamedLock::exclusive(self.root.join(".thread-catalog.lock"))?;
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        let source = self.root.join(source_area).join(thread_id);
        if !source.is_dir() {
            return Err(StoreError::NotFound);
        }
        let _lock = if nonblocking {
            DirectoryLock::try_exclusive(&source)?
        } else {
            DirectoryLock::exclusive(&source)?
        };
        if self.source_has_live_rewrite(thread_id)? {
            return Err(StoreError::RewriteInProgress);
        }
        move_thread_folder(&source, &self.root.join(destination_area).join(thread_id))
    }

    pub fn unarchive(&self, thread_id: &str) -> Result<(), StoreError> {
        self.move_folder(thread_id, "archive", "threads", false)
    }

    /// True when the active folder's genesis carries `ephemeral: true`.
    /// `NotFound` when the folder is absent (archived folders are never
    /// ephemeral: archive refuses them).
    pub fn session_is_ephemeral(&self, thread_id: &str) -> Result<bool, StoreError> {
        let folder = self.root.join("threads").join(thread_id);
        if !folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        folder_is_ephemeral(&folder)
    }

    /// Removes an ephemeral session folder. The folder must be idle: a live
    /// line holder is `Busy`, and a durable session is `NotEphemeral`. The
    /// removal is one catalog transition (rename into `.rewrite-trash`, then
    /// delete) so a crash never leaves a half-deleted active folder; startup
    /// GC finishes an orphaned tombstone.
    pub fn discard_ephemeral(&self, thread_id: &str) -> Result<(), StoreError> {
        // The catalog lock covers the rename only; the deletion of the (possibly large)
        // tombstone runs after it is released, so create, archive and search never wait on it.
        let tombstone = {
            let _catalog = NamedLock::exclusive(self.root.join(".thread-catalog.lock"))?;
            if self.source_has_live_rewrite(thread_id)? {
                return Err(StoreError::RewriteInProgress);
            }
            let source = self.root.join("threads").join(thread_id);
            if !source.is_dir() {
                return Err(StoreError::NotFound);
            }
            let lock = DirectoryLock::try_exclusive(&source)?;
            if !folder_is_ephemeral(&source)? {
                return Err(StoreError::NotEphemeral);
            }
            if self.session_has_live_line_holder(thread_id)? {
                return Err(StoreError::Busy);
            }
            let tombstone = self.retire_ephemeral_folder(thread_id, &source)?;
            // The lifecycle lock is an open descriptor on the folder; release it
            // before the tree is deleted.
            drop(lock);
            tombstone
        };
        Self::remove_tombstone(self.root(), &tombstone)
    }

    /// Removes every ephemeral folder (no exclusions). Offline tools and tests only: the Kernel
    /// uses `cleanup_stale_ephemeral` after readiness. Returns the removed ids.
    pub fn sweep_ephemeral(&self) -> Result<Vec<String>, StoreError> {
        Ok(self.cleanup_stale_ephemeral(&|_| false)?.removed)
    }

    fn retire_ephemeral_folder(
        &self,
        thread_id: &str,
        source: &Path,
    ) -> Result<PathBuf, StoreError> {
        let tombstone = self
            .root
            .join(".rewrite-trash")
            .join(format!("{DISCARD_TOMBSTONE_PREFIX}{thread_id}"));
        park_stale_tombstone(&self.root, thread_id)?;
        self.retire_to(thread_id, source, &tombstone)?;
        Ok(tombstone)
    }

    /// Renames `threads/<id>` to `tombstone` (inside `.rewrite-trash`), makes it durable and
    /// announces the removal. The caller holds `.thread-catalog.lock`.
    fn retire_to(
        &self,
        thread_id: &str,
        source: &Path,
        tombstone: &Path,
    ) -> Result<(), StoreError> {
        fs::rename(source, tombstone)?;
        File::open(self.root.join("threads"))?.sync_all()?;
        File::open(self.root.join(".rewrite-trash"))?.sync_all()?;
        crate::folder_events::emit(FolderEvent::Removed {
            root: self.root.clone(),
            id: thread_id.to_owned(),
            reason: RemoveReason::Discarded,
        });
        Ok(())
    }

    fn remove_tombstone(root: &Path, tombstone: &Path) -> Result<(), StoreError> {
        remove_tree_if_present(tombstone)?;
        File::open(root.join(".rewrite-trash"))?.sync_all()?;
        Ok(())
    }

    /// Post-ready cleanup of ephemeral scratch threads left by a previous process
    /// (ADDENDUM-02 S4, ADDENDUM-03 C7, E831 E3). `excluded` names threads this process owns
    /// (admitted) and is consulted again under the catalog lock. The scan is unlocked; the
    /// catalog lock is held only around the eligibility re-check and the rename of one
    /// candidate to a private tombstone name, and the deletion runs outside every lock, so
    /// create, archive and search are never blocked for the duration of a delete.
    pub fn cleanup_stale_ephemeral(
        &self,
        excluded: &dyn Fn(&str) -> bool,
    ) -> Result<CleanupReport, StoreError> {
        let mut report = CleanupReport::default();
        let swept = self.cleanup_ephemeral_threads(excluded, &mut report);
        // Leftover tombstones are cleaned whatever happened to the ephemeral threads.
        let tombstones = self.cleanup_discard_tombstones();
        swept?;
        report.tombstones = tombstones?;
        Ok(report)
    }

    fn cleanup_ephemeral_threads(
        &self,
        excluded: &dyn Fn(&str) -> bool,
        report: &mut CleanupReport,
    ) -> Result<(), StoreError> {
        let mut entries =
            fs::read_dir(self.root.join("threads"))?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let thread_id = entry.file_name().to_string_lossy().into_owned();
            if excluded(&thread_id) {
                continue;
            }
            // Unlocked pre-check: only an ephemeral genesis is a candidate.
            if !folder_is_ephemeral(&entry.path()).unwrap_or(false) {
                continue;
            }
            let private = {
                let _catalog = NamedLock::exclusive(self.root.join(".thread-catalog.lock"))?;
                let source = self.root.join("threads").join(&thread_id);
                if excluded(&thread_id)
                    || !source.is_dir()
                    || self.source_has_live_rewrite(&thread_id)?
                    || self.destination_has_live_rewrite(&thread_id)?
                {
                    continue;
                }
                let Ok(lifecycle) = DirectoryLock::try_exclusive(&source) else {
                    report.busy.push(thread_id);
                    continue;
                };
                if self
                    .session_has_live_line_holder(&thread_id)
                    .unwrap_or(true)
                {
                    report.busy.push(thread_id);
                    continue;
                }
                let private = self.root.join(".rewrite-trash").join(format!(
                    "{DISCARD_TOMBSTONE_PREFIX}{thread_id}.deleting-{}-{}",
                    std::process::id(),
                    TOMBSTONE_ORDINAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ));
                self.retire_to(&thread_id, &source, &private)?;
                drop(lifecycle);
                private
            };
            Self::remove_tombstone(self.root(), &private)?;
            crate::folder_events::emit(FolderEvent::Removed {
                root: self.root.clone(),
                id: thread_id.clone(),
                reason: RemoveReason::TombstoneDeleted,
            });
            report.removed.push(thread_id);
        }
        Ok(())
    }

    /// Deletes leftover `discard-*` tombstones (a crash between retire and delete, or a
    /// previous cleanup's private name). A tombstone that disappears underneath is success.
    pub fn cleanup_discard_tombstones(&self) -> Result<u32, StoreError> {
        let mut removed = 0;
        for entry in fs::read_dir(self.root.join(".rewrite-trash"))? {
            let entry = entry?;
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with(DISCARD_TOMBSTONE_PREFIX)
            {
                remove_tree_if_present(&entry.path())?;
                removed += 1;
            }
        }
        if removed > 0 {
            File::open(self.root.join(".rewrite-trash"))?.sync_all()?;
        }
        Ok(removed)
    }

    fn find_by_origin(&self, origin: &OriginTuple) -> Result<Option<String>, StoreError> {
        for area in ["threads", "archive"] {
            for entry in fs::read_dir(self.root.join(area))? {
                let entry = entry?;
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                // Only the first line (the genesis) is needed. Reading the whole ledger of every
                // thread on every create made session creation O(total ledger bytes).
                let bytes = read_first_line(&entry.path().join("main.jsonl"))?;
                let line = bytes.as_slice();
                if line.is_empty() {
                    continue;
                }
                let genesis = Event::decode_canonical(line)?;
                if genesis.origin_tuple()?.as_ref() == Some(origin) {
                    return Ok(Some(entry.file_name().to_string_lossy().into_owned()));
                }
            }
        }
        Ok(None)
    }

    fn find_staged_create_by_origin(
        &self,
        origin: &OriginTuple,
    ) -> Result<Option<(String, PathBuf)>, StoreError> {
        let mut candidates = Vec::new();
        for entry in fs::read_dir(self.root.join(".create-staging"))? {
            let entry = entry?;
            if !entry.file_type()?.is_dir()
                || !entry.file_name().to_string_lossy().starts_with("create-")
            {
                continue;
            }
            let path = entry.path();
            let Ok(bytes) = fs::read(path.join("main.jsonl")) else {
                continue;
            };
            let Some(line) = bytes.split(|byte| *byte == b'\n').next() else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            let genesis = Event::decode_canonical(line)?;
            if !matches!(genesis.kind(), EventKind::Genesis) || genesis.seq() != 1 {
                continue;
            }
            if genesis.origin_tuple()?.as_ref() == Some(origin) {
                let thread_id = genesis
                    .string_field("thread")
                    .ok_or_else(|| {
                        StoreError::Corruption("staged genesis lacks thread id".to_owned())
                    })?
                    .to_owned();
                candidates.push((thread_id, path));
            }
        }
        match candidates.len() {
            0 => Ok(None),
            1 => Ok(candidates.pop()),
            _ => Err(StoreError::Corruption(
                "multiple durable staged creates share one origin tuple".to_owned(),
            )),
        }
    }

    fn remove_incomplete_create_stages(&self, thread_id: &str) -> Result<(), StoreError> {
        let staging = self.root.join(".create-staging");
        let prefix = format!("create-{thread_id}-");
        let mut removed = false;
        for entry in fs::read_dir(&staging)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir()
                || !entry.file_name().to_string_lossy().starts_with(&prefix)
            {
                continue;
            }
            let main = entry.path().join("main.jsonl");
            let valid_genesis = fs::read(&main).ok().is_some_and(|bytes| {
                let line = bytes
                    .split(|byte| *byte == b'\n')
                    .next()
                    .unwrap_or_default();
                !line.is_empty()
                    && Event::decode_canonical(line).is_ok_and(|event| {
                        matches!(event.kind(), EventKind::Genesis)
                            && event.seq() == 1
                            && event.string_field("thread") == Some(thread_id)
                    })
            });
            if valid_genesis {
                continue;
            }
            fs::remove_dir_all(entry.path())?;
            removed = true;
        }
        if removed {
            File::open(staging)?.sync_all()?;
        }
        Ok(())
    }
}

fn publish_create_assets(root: &Path, assets: &[Vec<u8>]) -> Result<(), StoreError> {
    let store = AssetStore::new(root)?;
    for bytes in assets {
        store.publish(bytes)?;
    }
    File::open(root)?.sync_all()?;
    Ok(())
}

fn validate_line_name(name: &str) -> Result<(), StoreError> {
    if name == "main.jsonl" {
        return Ok(());
    }
    if name
        .strip_suffix(".jsonl")
        .is_some_and(|id| uuid::Uuid::parse_str(id).is_ok_and(|parsed| parsed.to_string() == id))
    {
        return Ok(());
    }
    Err(StoreError::Corruption(
        "invalid child ledger name".to_owned(),
    ))
}

fn validate_line_ancestry(
    folder: &Path,
    session: &str,
    line_file: &str,
    projection: Option<&schema::LedgerProjection>,
) -> Result<(), StoreError> {
    let corrupt = |message: &str| StoreError::Corruption(message.to_owned());
    let mut current = projection
        .ok_or_else(|| corrupt("empty child ledger"))?
        .clone();
    let mut file = line_file.to_owned();
    let mut visited = std::collections::HashSet::new();
    loop {
        if !visited.insert(file.clone()) {
            return Err(corrupt("cyclic child ancestry"));
        }
        let genesis = current
            .events
            .first()
            .ok_or_else(|| corrupt("missing line genesis"))?;
        let expected = if file == "main.jsonl" {
            session
        } else {
            file.strip_suffix(".jsonl")
                .ok_or_else(|| corrupt("invalid child filename"))?
        };
        if genesis.kind() != &EventKind::Genesis || genesis.string_field("thread") != Some(expected)
        {
            return Err(corrupt("line identity disagrees with genesis"));
        }
        if file == "main.jsonl" {
            if current.lifecycle.stop_active {
                return Err(corrupt("session stop is active"));
            }
            return Ok(());
        }
        let value: serde_json::Value = serde_json::from_slice(&genesis.canonical_bytes()?)
            .map_err(|error| corrupt(&error.to_string()))?;
        let parent = value
            .get("parent")
            .ok_or_else(|| corrupt("child lacks parent binding"))?;
        let parent_file = parent
            .get("file")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| corrupt("child lacks parent filename"))?;
        validate_line_name(parent_file)?;
        let parent_seq = parent
            .get("seq")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| corrupt("child lacks parent sequence"))?;
        let spawn_id = parent
            .get("spawn_id")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| corrupt("child lacks spawn identity"))?;
        let path = folder.join(parent_file);
        if !fs::symlink_metadata(&path)?.file_type().is_file() {
            return Err(corrupt("parent ledger is not a regular file"));
        }
        let mut parent_bytes = Vec::new();
        fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(path)?
            .read_to_end(&mut parent_bytes)?;
        let scan = crate::scan_valid_prefix(&parent_bytes, 1);
        let parent_projection = scan
            .projection
            .ok_or_else(|| corrupt("empty parent ledger"))?;
        let spawn = parent_projection
            .events
            .iter()
            .find(|event| event.seq() == parent_seq)
            .ok_or_else(|| corrupt("parent spawn is absent"))?;
        if spawn.kind() != &EventKind::Spawn
            || !matches!(spawn.string_field("child"), Some(child) if child == expected || child == file)
            || spawn.string_field("spawn_id") != Some(spawn_id)
        {
            return Err(corrupt("child parent binding disagrees with spawn"));
        }
        file = parent_file.to_owned();
        current = parent_projection;
    }
}

const DISCARD_TOMBSTONE_PREFIX: &str = "discard-";
static TOMBSTONE_ORDINAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// What `cleanup_stale_ephemeral` did.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct CleanupReport {
    pub removed: Vec<String>,
    /// Ephemeral folders skipped because a lock or worker still holds them (retried next tick).
    pub busy: Vec<String>,
    /// Leftover `discard-*` tombstones deleted.
    pub tombstones: u32,
}

/// How long a deleter waits for another deleter that is still removing the same tree.
const TREE_REMOVAL_WAIT: std::time::Duration = std::time::Duration::from_secs(10);

/// `remove_dir_all` where a path that vanishes underneath (another deleter, E831 E3) is success.
/// A tree another deleter is still removing (so `ENOTEMPTY` races) is waited for, bounded.
pub(crate) fn remove_tree_if_present(path: &Path) -> Result<(), StoreError> {
    let deadline = std::time::Instant::now() + TREE_REMOVAL_WAIT;
    loop {
        match fs::remove_dir_all(path) {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound || !path.exists() => {
                return Ok(());
            }
            // Only the races of a concurrent deleter are waited for: the directory refills or
            // empties underneath us (`ENOTEMPTY`, `ENOENT`). Anything else is a real failure.
            Err(error)
                if error.raw_os_error() == Some(libc::ENOTEMPTY)
                    && std::time::Instant::now() < deadline =>
            {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            Err(error) => return Err(error.into()),
        }
    }
}

/// A stale `discard-<id>` tombstone must not make the id unusable, and deleting it must not
/// happen under the catalog and rewrite locks (a large tombstone would block create, archive
/// and search for the whole delete, E832 E3). Under the caller's locks it is only renamed to a
/// private name that no longer ends with the id; the deletion is handed to a background thread
/// (and, if the process dies first, to the post-ready tombstone cleanup, which removes every
/// `discard-*`).
pub(crate) fn park_stale_tombstone(root: &Path, thread_id: &str) -> Result<(), StoreError> {
    let trash = root.join(".rewrite-trash");
    let stale = trash.join(format!("{DISCARD_TOMBSTONE_PREFIX}{thread_id}"));
    if fs::symlink_metadata(&stale).is_err() {
        return Ok(());
    }
    let private = trash.join(format!(
        "{DISCARD_TOMBSTONE_PREFIX}{thread_id}.deleting-{}-{}",
        std::process::id(),
        TOMBSTONE_ORDINAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    match fs::rename(&stale, &private) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    }
    let trash_dir = trash.clone();
    let spawned = std::thread::Builder::new()
        .name("tombstone-delete".to_owned())
        .spawn(move || {
            if remove_tree_if_present(&private).is_ok() {
                if let Ok(directory) = File::open(&trash_dir) {
                    let _ = directory.sync_all();
                }
            }
        });
    // If no thread could be started the private tombstone stays for the cleanup.
    drop(spawned);
    Ok(())
}

/// The only function that moves a thread folder between `threads/` and `archive/`
/// (ADDENDUM-03 C1): rename, make both parents durable, announce `Moved`. Callers hold the
/// locks their protocol requires (`.thread-catalog.lock`, the folder lifecycle lock).
pub fn move_thread_folder(source: &Path, destination: &Path) -> Result<(), StoreError> {
    fn area(directory: &Path) -> Result<Area, StoreError> {
        match directory.file_name().and_then(|name| name.to_str()) {
            Some("threads") => Ok(Area::Threads),
            Some("archive") => Ok(Area::Archive),
            _ => Err(StoreError::Corruption(format!(
                "thread folder move names an unknown area: {}",
                directory.display()
            ))),
        }
    }
    let corrupt = |what: &str| StoreError::Corruption(format!("thread folder move: {what}"));
    let from_parent = source
        .parent()
        .ok_or_else(|| corrupt("source has no parent"))?;
    let to_parent = destination
        .parent()
        .ok_or_else(|| corrupt("destination has no parent"))?;
    let root = from_parent
        .parent()
        .ok_or_else(|| corrupt("source has no storage root"))?;
    if to_parent.parent() != Some(root) {
        return Err(corrupt("source and destination are under different roots"));
    }
    let id = source
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| corrupt("source has no name"))?;
    if destination.file_name().and_then(|name| name.to_str()) != Some(id) {
        return Err(corrupt("destination renames the thread"));
    }
    let (from, to) = (area(from_parent)?, area(to_parent)?);
    fs::rename(source, destination)?;
    File::open(from_parent)?.sync_all()?;
    File::open(to_parent)?.sync_all()?;
    crate::folder_events::emit(FolderEvent::Moved {
        root: root.to_path_buf(),
        id: id.to_owned(),
        from,
        to,
    });
    Ok(())
}

/// The bytes before the first LF (or the whole file when it has none), read incrementally. A
/// file without any LF is bounded by the same cap as `read_genesis` so a corrupt ledger cannot
/// be pulled into memory by an origin lookup.
fn read_first_line(path: &Path) -> Result<Vec<u8>, StoreError> {
    use std::io::{BufRead, Read};
    let file = File::open(path)?;
    let mut reader = std::io::BufReader::new(file.take(16 * 1024 * 1024));
    let mut line = Vec::new();
    reader.read_until(b'\n', &mut line)?;
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    Ok(line)
}

/// Reads the genesis line only; a folder without a decodable genesis is not
/// ephemeral (the semantic sweep reports corruption separately).
fn folder_is_ephemeral(folder: &Path) -> Result<bool, StoreError> {
    Ok(crate::read_genesis(folder.join("main.jsonl"))?
        .as_ref()
        .is_some_and(Event::is_ephemeral_genesis))
}

#[cfg(test)]
mod child_line_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn child_append_requires_matching_durable_spawn() {
        let temp = tempfile::tempdir().unwrap();
        let store = ThreadStore::open(temp.path()).unwrap();
        let root = "018f0000-0000-7000-8000-000000000010";
        let child = "018f0000-0000-7000-8000-000000000011";
        let folder = temp.path().join("threads").join(root);
        fs::create_dir(&folder).unwrap();
        let genesis = |id: &str| {
            json!({
                "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T09:00:00.000Z",
                "format":1,"min_reader":1,"min_writer":1,"thread":id,
                "workspace":"ws","resume":"never","config":{"digest":"cfg"},
                "origin_key":"create","origin_tuple":{
                    "principal":"p","client":"c","target":id,"op":"create","key":"create"
                }
            })
        };
        let write = |file: &str, events: &[serde_json::Value]| {
            let mut bytes = Vec::new();
            for event in events {
                bytes.extend(serde_json_canonicalizer::to_vec(event).unwrap());
                bytes.push(b'\n');
            }
            fs::write(folder.join(file), bytes).unwrap();
        };
        write(
            "main.jsonl",
            &[
                genesis(root),
                json!({"v":1,"seq":2,"kind":"input","ts":"2026-08-28T09:00:00.000Z",
                "origin_key":"input","origin_tuple":{"principal":"p","client":"c","target":root,"op":"submit","key":"input"},
                "content":[{"type":"text","text":"spawn"}]}),
                json!({"v":1,"seq":3,"kind":"turn_open","ts":"2026-08-28T09:00:00.000Z","turn":1,"trigger":{"inputs":[2]}}),
                json!({"v":1,"seq":4,"kind":"spawn","ts":"2026-08-28T09:00:00.000Z","turn":1,"call":"child-call","child":format!("{child}.jsonl"),"spawn_id":"spawn-1","resume":"never","seed":{"kinds":[]}}),
            ],
        );
        let mut child_genesis = genesis(child);
        child_genesis["parent"] = json!({"file":"main.jsonl","seq":4,"spawn_id":"spawn-1"});
        let file = format!("{child}.jsonl");
        write(&file, &[child_genesis.clone()]);
        let origin = OriginTuple {
            principal: "p".into(),
            client: "c".into(),
            target: root.into(),
            op: "respond".into(),
            key: "reply".into(),
        };
        let result = store
            .append_line_keyed_with_projection_if(root, &file, &origin, |seq, projection| {
                assert_eq!(seq, 2);
                assert_eq!(projection.events[0].string_field("thread"), Some(child));
                Ok(None)
            })
            .unwrap();
        assert_eq!(result, ConditionalAppendOutcome::Skipped);
        let main_before = fs::read(folder.join("main.jsonl")).unwrap();
        let appended = store
            .append_line_keyed_with_projection_if(root, &file, &origin, |seq, _| {
                let value = json!({"v":1,"seq":seq,"kind":"input","ts":"2026-08-28T09:00:00.000Z",
                "origin_key":origin.key,"origin_tuple":origin,
                "content":[{"type":"text","text":"child-only input"}]});
                Ok(Some(Event::decode_canonical(
                    &serde_json_canonicalizer::to_vec(&value).unwrap(),
                )?))
            })
            .unwrap();
        assert_eq!(
            appended,
            ConditionalAppendOutcome::Appended(AppendOutcome {
                seq: 2,
                deduplicated: false
            })
        );
        let child_after = fs::read(folder.join(&file)).unwrap();
        let duplicate = store
            .append_line_keyed_with_projection_if(root, &file, &origin, |_, _| {
                panic!("deduplicated child append rebuilt the event")
            })
            .unwrap();
        assert_eq!(
            duplicate,
            ConditionalAppendOutcome::Appended(AppendOutcome {
                seq: 2,
                deduplicated: true
            })
        );
        assert_eq!(fs::read(folder.join(&file)).unwrap(), child_after);
        assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), main_before);
        for binding in [
            json!({"file":"main.jsonl","seq":3,"spawn_id":"spawn-1"}),
            json!({"file":"main.jsonl","seq":4,"spawn_id":"forged"}),
            json!({"file":"../main.jsonl","seq":4,"spawn_id":"spawn-1"}),
        ] {
            child_genesis["parent"] = binding;
            write(&file, &[child_genesis.clone()]);
            assert!(
                store
                    .append_line_keyed_with_projection_if(root, &file, &origin, |_, _| panic!(
                        "invalid ancestry reached writer"
                    ))
                    .is_err()
            );
        }
        assert!(
            store
                .append_line_keyed_with_projection_if(
                    root,
                    "../main.jsonl",
                    &origin,
                    |_, _| panic!("path escape reached writer")
                )
                .is_err()
        );
    }
}

#[cfg(test)]
mod ephemeral_tests {
    use super::*;
    use serde_json::json;

    fn write_genesis(root: &Path, id: &str, ephemeral: bool) {
        let folder = root.join("threads").join(id);
        fs::create_dir(&folder).unwrap();
        let mut genesis = json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T09:00:00.000Z",
            "format":1,"min_reader":1,"min_writer":1,"thread":id,
            "workspace":"ws","resume":"never","config":{"digest":"cfg"},
            "origin_key":"create","origin_tuple":{
                "principal":"p","client":"c","target":id,"op":"create","key":"create"
            }
        });
        if ephemeral {
            genesis["ephemeral"] = json!(true);
        }
        let mut bytes = serde_json_canonicalizer::to_vec(&genesis).unwrap();
        bytes.push(b'\n');
        fs::write(folder.join("main.jsonl"), bytes).unwrap();
    }

    #[test]
    fn discard_removes_only_idle_ephemeral_folders_and_sweep_leaves_durable_ones() {
        let temp = tempfile::tempdir().unwrap();
        let store = ThreadStore::open(temp.path()).unwrap();
        let durable = "018f0000-0000-7000-8000-000000000020";
        let scratch = "018f0000-0000-7000-8000-000000000021";
        let held = "018f0000-0000-7000-8000-000000000022";
        write_genesis(temp.path(), durable, false);
        write_genesis(temp.path(), scratch, true);
        write_genesis(temp.path(), held, true);
        assert!(!store.session_is_ephemeral(durable).unwrap());
        assert!(store.session_is_ephemeral(scratch).unwrap());
        assert!(matches!(
            store.discard_ephemeral(durable),
            Err(StoreError::NotEphemeral)
        ));
        assert!(matches!(
            store.discard_ephemeral("018f0000-0000-7000-8000-0000000000ff"),
            Err(StoreError::NotFound)
        ));

        let ledger = File::open(temp.path().join("threads").join(held).join("main.jsonl")).unwrap();
        crate::platform::try_lock_exclusive(&ledger).unwrap();
        assert!(matches!(
            store.discard_ephemeral(held),
            Err(StoreError::Busy)
        ));
        assert!(temp.path().join("threads").join(held).is_dir());
        drop(ledger);

        store.discard_ephemeral(scratch).unwrap();
        assert!(!temp.path().join("threads").join(scratch).exists());
        assert_eq!(
            fs::read_dir(temp.path().join(".rewrite-trash"))
                .unwrap()
                .count(),
            0
        );

        // An orphaned tombstone from a crash mid-discard is finished by the sweep.
        fs::create_dir(temp.path().join(".rewrite-trash").join("discard-orphan")).unwrap();
        let swept = store.sweep_ephemeral().unwrap();
        assert_eq!(swept, vec![held.to_owned()]);
        assert!(temp.path().join("threads").join(durable).is_dir());
        assert!(!temp.path().join("threads").join(held).exists());
        assert_eq!(
            fs::read_dir(temp.path().join(".rewrite-trash"))
                .unwrap()
                .count(),
            0
        );
    }
}
