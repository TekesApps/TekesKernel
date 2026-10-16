//! Logical folder identities and session-owned creation seeds. Trust deliberately
//! remains bound to the physical directory in FolderAuthorityStore.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use store::{AtomicPublisher, FullSync, NamedLock};

use crate::{
    ProfileError, WorkspaceConfig, WorkspaceFolder, canonical_line, digest, parse_canonical,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionConfig {
    /// Frozen compatibility input, never reloaded from workspaces/<id>.
    pub seed: WorkspaceConfig,
    pub compatibility_instructions: bool,
    /// Compatibility binding -> logical folder id.
    pub folder_refs: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogFolder {
    pub id: String,
    pub path: String,
    pub aliases: Vec<(String, String)>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    format: u64,
    revision: u64,
    folders: BTreeMap<String, CatalogFolder>,
    #[serde(default)]
    relocations: BTreeMap<String, Relocation>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            format: 1,
            revision: 0,
            folders: BTreeMap::new(),
            relocations: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Relocation {
    hash: String,
    id: String,
    previous: String,
    path: String,
}

/// A catalog publication is conditional on the exact scope event reaching the
/// session ledger. The private intent survives a crash at either barrier.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CaptureIntent {
    ledger: PathBuf,
    seq: u64,
    event_sha256: String,
    catalog: Catalog,
}

pub struct FolderCapture {
    root: PathBuf,
    _lock: NamedLock,
    catalog: Catalog,
    scope: SessionConfig,
}

impl FolderCapture {
    #[must_use]
    pub fn scope(&self) -> &SessionConfig {
        &self.scope
    }

    /// Must precede the ledger append, while both catalog and ledger locks
    /// remain held. A failed append is resolved by the same recovery path.
    pub fn prepare(&self, ledger: &Path, event: &schema::Event) -> Result<(), ProfileError> {
        let ledger = ledger
            .strip_prefix(&self.root)
            .map_err(|_| {
                FolderCatalog::new(&self.root).invalid("capture ledger is outside storage")
            })?
            .to_path_buf();
        validate_capture_ledger(&ledger)?;
        let intent = CaptureIntent {
            ledger,
            seq: event.seq(),
            event_sha256: digest(&event.canonical_bytes().map_err(|error| {
                ProfileError::InvalidBytes {
                    path: PathBuf::from("<scope-event>"),
                    reason: error.to_string(),
                }
            })?),
            catalog: self.catalog.clone(),
        };
        let path = self.root.join("config/.folder-capture.json");
        AtomicPublisher::replace(&path, &canonical_line(&path, &intent)?)?;
        Ok(())
    }

    pub fn finish(self) -> Result<(), ProfileError> {
        FolderCatalog::new(&self.root).recover_capture()
    }
}

fn validate_capture_ledger(ledger: &Path) -> Result<(), ProfileError> {
    let parts = ledger.components().collect::<Vec<_>>();
    if parts.len() != 3
        || parts[0].as_os_str() != "threads"
        || !matches!(parts[1], std::path::Component::Normal(_))
        || parts[2].as_os_str() != "main.jsonl"
    {
        return Err(ProfileError::InvalidReference {
            path: ledger.to_path_buf(),
            reason: "invalid capture ledger".into(),
        });
    }
    Ok(())
}

pub struct FolderCatalog {
    root: PathBuf,
}

impl FolderCatalog {
    #[must_use]
    pub fn new(storage_root: impl Into<PathBuf>) -> Self {
        Self {
            root: storage_root.into(),
        }
    }

    fn lock(&self) -> Result<NamedLock, ProfileError> {
        let lock = NamedLock::exclusive(self.root.join("config/.folders.lock"))?;
        self.recover_capture()?;
        Ok(lock)
    }

    fn recover_capture(&self) -> Result<(), ProfileError> {
        let path = self.root.join("config/.folder-capture.json");
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let intent: CaptureIntent = parse_canonical(&path, &bytes)?;
        validate_capture_ledger(&intent.ledger)?;
        let active = self.root.join(&intent.ledger);
        let ledger = if active.exists() {
            active
        } else {
            self.root
                .join("archive")
                .join(
                    intent
                        .ledger
                        .components()
                        .nth(1)
                        .expect("validated")
                        .as_os_str(),
                )
                .join("main.jsonl")
        };
        // No ledger lock is acquired here: the catalog lock excludes the sole
        // staged writer. Recovery must not invert the ledger -> catalog order.
        let bytes = fs::read(&ledger)?;
        let scan = store::scan_valid_prefix(&bytes, schema::READER_VERSION);
        let projection = scan
            .projection
            .ok_or_else(|| self.invalid("capture ledger has no valid genesis"))?;
        let committed = projection
            .events
            .iter()
            .find(|event| event.seq() == intent.seq)
            .map(|event| {
                event
                    .canonical_bytes()
                    .map(|bytes| digest(&bytes) == intent.event_sha256)
                    .map_err(|error| self.invalid(error.to_string()))
            })
            .transpose()?
            .unwrap_or(false);
        if committed {
            // An append may have returned an ambiguous sync error. Make the
            // matching prefix durable before publishing its catalog state.
            FullSync::full_sync(
                &fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&ledger)?,
            )?;
            let current = self.read()?;
            if current.revision < intent.catalog.revision {
                self.publish(&intent.catalog)?;
            } else if current.revision == intent.catalog.revision
                && canonical_line("<catalog>", &current)?
                    != canonical_line("<catalog>", &intent.catalog)?
            {
                return Err(self.invalid("capture catalog revision has conflicting contents"));
            }
            // A failed directory sync after intent removal can resurrect an
            // old intent after restart. Never roll back a newer publication.
        }
        fs::remove_file(&path)?;
        fs::File::open(self.root.join("config"))?.sync_all()?;
        Ok(())
    }

    fn read(&self) -> Result<Catalog, ProfileError> {
        let path = self.root.join("config/folders.json");
        match fs::read(&path) {
            Ok(bytes) => {
                let catalog: Catalog = parse_canonical(&path, &bytes)?;
                if catalog.format != 1 {
                    return Err(ProfileError::UnsupportedFormat {
                        path,
                        format: catalog.format,
                    });
                }
                Ok(catalog)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Catalog::default()),
            Err(error) => Err(error.into()),
        }
    }

    fn publish(&self, catalog: &Catalog) -> Result<(), ProfileError> {
        let path = self.root.join("config/folders.json");
        AtomicPublisher::replace(&path, &canonical_line(&path, catalog)?)?;
        Ok(())
    }

    pub fn revision(&self) -> Result<u64, ProfileError> {
        let _lock = self.lock()?;
        Ok(self.read()?.revision)
    }

    pub fn folders(&self) -> Result<Vec<CatalogFolder>, ProfileError> {
        let _lock = self.lock()?;
        Ok(self.read()?.folders.into_values().collect())
    }

    /// Registers paths only; this does not request or grant project trust.
    /// Existing aliases retain their identity across a relocation.
    pub fn capture(&self, seed: &WorkspaceConfig) -> Result<SessionConfig, ProfileError> {
        let _lock = self.lock()?;
        let mut catalog = self.read()?;
        let before = canonical_line("<catalog>", &catalog)?;
        let scope = Self::capture_into(&mut catalog, seed);
        if before != canonical_line("<catalog>", &catalog)? {
            catalog.revision += 1;
            self.publish(&catalog)?;
        }
        Ok(scope)
    }

    /// Computes identities without publishing them. Readers cannot see the
    /// staged state until finish/recovery observes the durable scope event.
    pub fn begin_capture(
        &self,
        seed: &WorkspaceConfig,
        expected_revision: Option<u64>,
    ) -> Result<FolderCapture, ProfileError> {
        let lock = self.lock()?;
        let mut catalog = self.read()?;
        if let Some(expected) = expected_revision
            && catalog.revision != expected
        {
            return Err(ProfileError::StaleRevision {
                expected,
                actual: catalog.revision,
            });
        }
        let before = canonical_line("<catalog>", &catalog)?;
        let scope = Self::capture_into(&mut catalog, seed);
        if before != canonical_line("<catalog>", &catalog)? {
            catalog.revision += 1;
        }
        Ok(FolderCapture {
            root: self.root.clone(),
            _lock: lock,
            catalog,
            scope,
        })
    }

    fn capture_into(catalog: &mut Catalog, seed: &WorkspaceConfig) -> SessionConfig {
        let mut folder_refs = BTreeMap::new();
        for (binding, path) in seed.folder_ids().into_iter().zip(seed.folder_paths()) {
            let alias = (seed.id.clone(), binding.clone());
            let existing = catalog.folders.values().find(|folder| folder.path == path);
            let id = existing.map(|folder| folder.id.clone()).unwrap_or_else(|| {
                // Revision prevents reusing a relocated id when its old path is registered anew.
                format!(
                    "folder-{}",
                    digest(format!("{}:{path}", catalog.revision).as_bytes())
                )
            });
            for other in catalog.folders.values_mut() {
                if other.id != id {
                    other.aliases.retain(|existing| existing != &alias);
                }
            }
            let folder = catalog
                .folders
                .entry(id.clone())
                .or_insert_with(|| CatalogFolder {
                    id: id.clone(),
                    path: path.to_owned(),
                    aliases: Vec::new(),
                });
            // An alias is an old protocol address, not the folder's identity.
            if !folder.aliases.contains(&alias) {
                folder.aliases.push(alias);
                folder.aliases.sort();
            }
            folder_refs.insert(binding, id);
        }
        SessionConfig {
            seed: seed.clone(),
            compatibility_instructions: true,
            folder_refs,
        }
    }

    pub fn resolve(&self, scope: &SessionConfig) -> Result<WorkspaceConfig, ProfileError> {
        let _lock = self.lock()?;
        let catalog = self.read()?;
        let mut seed = scope.seed.clone();
        let mut folders = Vec::new();
        let old_paths = seed
            .folder_paths()
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        for binding in seed.folder_ids() {
            let id = scope
                .folder_refs
                .get(&binding)
                .ok_or_else(|| self.invalid("session folder has no logical identity"))?;
            let folder = catalog
                .folders
                .get(id)
                .ok_or_else(|| self.invalid("logical session folder is absent"))?;
            folders.push(WorkspaceFolder {
                id: binding,
                path: folder.path.clone(),
            });
        }
        if let Some(policy) = &mut seed.policy {
            for root in &mut policy.writable_roots {
                if let Some((old, new)) = old_paths
                    .iter()
                    .zip(&folders)
                    .filter(|(old, _)| Path::new(root).starts_with(old))
                    .max_by_key(|(old, _)| old.len())
                {
                    *root = Path::new(&new.path)
                        .join(Path::new(root).strip_prefix(old).expect("checked"))
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }
        seed.cwd.clear();
        seed.folders = folders;
        Ok(seed)
    }

    /// An old seed can use catalog aliases without registering anything on a
    /// read path. Missing aliases retain the checksum-verified original path.
    pub fn resolve_legacy(
        &self,
        original: &WorkspaceConfig,
    ) -> Result<WorkspaceConfig, ProfileError> {
        let folders = self.folders()?;
        let mut seed = original.clone();
        seed.folders = original
            .folder_ids()
            .into_iter()
            .zip(original.folder_paths())
            .map(|(id, path)| {
                let current = folders
                    .iter()
                    .find(|folder| folder.aliases.contains(&(original.id.clone(), id.clone())))
                    .map_or(path, |folder| folder.path.as_str());
                WorkspaceFolder {
                    id,
                    path: current.to_owned(),
                }
            })
            .collect();
        seed.cwd.clear();
        if let Some(policy) = &mut seed.policy {
            for root in &mut policy.writable_roots {
                if let Some((old, new)) = original
                    .folder_paths()
                    .into_iter()
                    .zip(&seed.folders)
                    .filter(|(old, _)| Path::new(root).starts_with(old))
                    .max_by_key(|(old, _)| old.len())
                {
                    *root = Path::new(&new.path)
                        .join(Path::new(root).strip_prefix(old).expect("checked"))
                        .to_string_lossy()
                        .into_owned();
                }
            }
        }
        Ok(seed)
    }

    /// Called inside the existing durable relocation operation, before its
    /// semantic receipt. Retrying the same journal phase is idempotent.
    pub fn relocate_alias(
        &self,
        workspace: &str,
        binding: &str,
        previous: &str,
        path: &str,
    ) -> Result<(), ProfileError> {
        let _lock = self.lock()?;
        let mut catalog = self.read()?;
        let before = canonical_line("<catalog>", &catalog)?;
        let alias = (workspace.to_owned(), binding.to_owned());
        let id = match catalog
            .folders
            .values()
            .find(|folder| folder.aliases.contains(&alias))
        {
            Some(folder) if folder.path == previous || folder.path == path => folder.id.clone(),
            Some(_) => return Err(self.invalid("folder relocation is stale")),
            None => {
                // Recovery may encounter an intent prepared before the global
                // catalog existed. Import its old address without requiring
                // the old directory to be mounted or rewriting any ledger.
                let id = catalog
                    .folders
                    .values()
                    .find(|folder| folder.path == previous)
                    .map(|folder| folder.id.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "folder-{}",
                            digest(format!("{}:{previous}", catalog.revision).as_bytes())
                        )
                    });
                let folder = catalog
                    .folders
                    .entry(id.clone())
                    .or_insert_with(|| CatalogFolder {
                        id: id.clone(),
                        path: previous.to_owned(),
                        aliases: Vec::new(),
                    });
                folder.aliases.push(alias);
                folder.aliases.sort();
                id
            }
        };
        if catalog
            .folders
            .values()
            .any(|folder| folder.id != id && folder.path == path)
        {
            return Err(self.invalid("relocation destination belongs to another logical folder"));
        }
        catalog.folders.get_mut(&id).expect("found above").path = path.to_owned();
        if before != canonical_line("<catalog>", &catalog)? {
            catalog.revision += 1;
            self.publish(&catalog)?;
        }
        Ok(())
    }

    /// Resolve an exact retry before validating paths that may no longer exist.
    pub fn relocation(
        &self,
        rpc: &str,
        hash: &str,
        previous: &str,
    ) -> Result<Option<(String, bool)>, ProfileError> {
        let _lock = self.lock()?;
        let catalog = self.read()?;
        if let Some(receipt) = catalog.relocations.get(rpc) {
            if receipt.hash != hash {
                return Err(self.invalid("folder relocation idempotency conflict"));
            }
            return Ok(Some((receipt.id.clone(), true)));
        }
        Ok(catalog
            .folders
            .values()
            .find(|folder| folder.path == previous)
            .map(|folder| (folder.id.clone(), false)))
    }

    pub fn relocate(
        &self,
        rpc: &str,
        hash: &str,
        id: &str,
        previous: &str,
        path: &str,
    ) -> Result<(), ProfileError> {
        let _lock = self.lock()?;
        let mut catalog = self.read()?;
        if let Some(receipt) = catalog.relocations.get(rpc) {
            if receipt.hash != hash {
                return Err(self.invalid("folder relocation idempotency conflict"));
            }
            return Ok(());
        }
        if catalog
            .folders
            .values()
            .any(|folder| folder.id != id && folder.path == path)
        {
            return Err(self.invalid("relocation destination belongs to another logical folder"));
        }
        let folder = catalog
            .folders
            .get_mut(id)
            .ok_or_else(|| self.invalid("folder missing"))?;
        if folder.path != previous {
            return Err(self.invalid("folder relocation is stale"));
        }
        folder.path = path.to_owned();
        catalog.relocations.insert(
            rpc.to_owned(),
            Relocation {
                hash: hash.to_owned(),
                id: id.to_owned(),
                previous: previous.to_owned(),
                path: path.to_owned(),
            },
        );
        catalog.revision += 1;
        self.publish(&catalog)
    }

    fn invalid(&self, reason: impl Into<String>) -> ProfileError {
        ProfileError::InvalidReference {
            path: self.root.join("config/folders.json"),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folder_capture_refuses_a_relocation_between_resolution_and_preparation() {
        let root = tempfile::tempdir().unwrap();
        crate::ConfigRepository::open(root.path()).unwrap();
        let catalog = FolderCatalog::new(root.path());
        let seed = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "ws".into(),
            name: "test".into(),
            cwd: vec!["/project/a".into()],
            folders: Vec::new(),
            policy: None,
        };
        catalog.capture(&seed).unwrap();
        let revision = catalog.revision().unwrap();
        catalog
            .relocate_alias("ws", "folder-0001", "/project/a", "/project/moved")
            .unwrap();
        let before = fs::read(root.path().join("config/folders.json")).unwrap();
        assert!(matches!(
            catalog.begin_capture(&seed, Some(revision)),
            Err(ProfileError::StaleRevision { .. })
        ));
        assert_eq!(
            fs::read(root.path().join("config/folders.json")).unwrap(),
            before
        );
        assert!(!root.path().join("config/.folder-capture.json").exists());
    }

    #[test]
    fn folder_capture_recovers_only_the_exact_committed_event_across_failure_boundaries() {
        for boundary in [
            "prepared",
            "partial",
            "different-event",
            "committed",
            "published",
            "publish-error",
            "archived",
            "newer-publication",
        ] {
            let root = tempfile::tempdir().unwrap();
            crate::ConfigRepository::open(root.path()).unwrap();
            let catalog = FolderCatalog::new(root.path());
            let seed = WorkspaceConfig {
                format: 1,
                revision: 1,
                id: "ws".into(),
                name: "test".into(),
                cwd: Vec::new(),
                folders: vec![WorkspaceFolder {
                    id: "a".into(),
                    path: "/project/a".into(),
                }],
                policy: None,
            };
            catalog.capture(&seed).unwrap();
            let before = fs::read(root.path().join("config/folders.json")).unwrap();
            let session = "018f0000-0000-7000-8000-000000000001";
            let genesis = serde_json::json!({"v":1,"format":1,"seq":1,"kind":"genesis","ts":"2026-10-16T00:00:00.000Z","thread":session,"workspace":"ws","config":{"digest":"cfg"},"min_reader":1,"min_writer":1,"resume":"never","origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":session,"op":"create","key":"create"}});
            let ledger = root.path().join("threads").join(session).join("main.jsonl");
            fs::create_dir_all(ledger.parent().unwrap()).unwrap();
            fs::write(&ledger, canonical_line("<genesis>", &genesis).unwrap()).unwrap();
            let mut changed = seed.clone();
            changed.folders.push(WorkspaceFolder {
                id: "b".into(),
                path: "/project/b".into(),
            });
            let staged = catalog.begin_capture(&changed, None).unwrap();
            let value = serde_json::json!({"v":1,"seq":2,"kind":"meta","ts":"2026-10-16T00:00:01.000Z","min_reader":4,"scope_change":{"session_scope":staged.scope(),"folders":["a","b"],"folder_binding":"a","cwd":"/project/a","environment":"local"}});
            let event = schema::Event::from_value(
                schema::IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap(),
            )
            .unwrap();
            staged.prepare(&ledger, &event).unwrap();
            assert_eq!(
                fs::read(root.path().join("config/folders.json")).unwrap(),
                before,
                "prepared state must be private"
            );
            let mut bytes = fs::read(&ledger).unwrap();
            if boundary == "partial" {
                bytes.extend_from_slice(b"{\"kind\":");
            } else if boundary == "different-event" {
                bytes.extend_from_slice(&canonical_line("<other>",&serde_json::json!({"v":1,"seq":2,"kind":"meta","ts":"2026-10-16T00:00:01.000Z","title":"other"})).unwrap());
            } else if !matches!(boundary, "prepared") {
                bytes.extend_from_slice(&event.canonical_bytes().unwrap());
                bytes.push(b'\n');
            }
            fs::write(&ledger, &bytes).unwrap();
            if boundary == "published" {
                catalog.publish(&staged.catalog).unwrap();
            }
            if boundary == "newer-publication" {
                let mut later = staged.catalog.clone();
                later.revision += 1;
                later.folders.insert(
                    "later".into(),
                    CatalogFolder {
                        id: "later".into(),
                        path: "/project/later".into(),
                        aliases: Vec::new(),
                    },
                );
                catalog.publish(&later).unwrap();
            }
            if boundary == "publish-error" {
                fs::remove_file(root.path().join("config/folders.json")).unwrap();
                fs::create_dir(root.path().join("config/folders.json")).unwrap();
                assert!(staged.finish().is_err());
                assert!(root.path().join("config/.folder-capture.json").exists());
                fs::remove_dir(root.path().join("config/folders.json")).unwrap();
            } else {
                drop(staged);
            }
            if boundary == "archived" {
                fs::create_dir_all(root.path().join("archive")).unwrap();
                fs::rename(
                    ledger.parent().unwrap(),
                    root.path().join("archive").join(session),
                )
                .unwrap();
            }
            // A fresh catalog instance simulates restart without any in-memory receipt.
            let restarted = FolderCatalog::new(root.path());
            let folders = restarted.folders().unwrap();
            let committed = matches!(
                boundary,
                "committed" | "published" | "publish-error" | "archived" | "newer-publication"
            );
            assert_eq!(
                folders.len(),
                if boundary == "newer-publication" {
                    3
                } else if committed {
                    2
                } else {
                    1
                },
                "{boundary}"
            );
            if !committed {
                assert_eq!(
                    fs::read(root.path().join("config/folders.json")).unwrap(),
                    before,
                    "{boundary}"
                );
            }
            assert!(
                !root.path().join("config/.folder-capture.json").exists(),
                "{boundary}"
            );
            assert_eq!(
                restarted.folders().unwrap(),
                folders,
                "recovery must be idempotent"
            );
        }
    }

    #[test]
    fn phase7_pre_catalog_relocation_recovery_imports_an_offline_legacy_address_once() {
        let root = tempfile::tempdir().unwrap();
        crate::ConfigRepository::open(root.path()).unwrap();
        let catalog = FolderCatalog::new(root.path());
        catalog
            .relocate_alias(
                "legacy",
                "folder-0001",
                "/offline/original",
                "/mounted/restored",
            )
            .unwrap();
        let before = std::fs::read(root.path().join("config/folders.json")).unwrap();
        catalog
            .relocate_alias(
                "legacy",
                "folder-0001",
                "/offline/original",
                "/mounted/restored",
            )
            .unwrap();
        assert_eq!(
            std::fs::read(root.path().join("config/folders.json")).unwrap(),
            before
        );
        let folders = catalog.folders().unwrap();
        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].path, "/mounted/restored");
        assert_eq!(
            folders[0].aliases,
            vec![("legacy".to_owned(), "folder-0001".to_owned())]
        );
    }
}
