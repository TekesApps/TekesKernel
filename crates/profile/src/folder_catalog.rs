//! Logical folder identities and session-owned creation seeds. Trust deliberately
//! remains bound to the physical directory in FolderAuthorityStore.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use store::{AtomicPublisher, NamedLock};

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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Relocation {
    hash: String,
    id: String,
    previous: String,
    path: String,
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

    pub fn folders(&self) -> Result<Vec<CatalogFolder>, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.folders.lock"))?;
        Ok(self.read()?.folders.into_values().collect())
    }

    /// Registers paths only; this does not request or grant project trust.
    /// Existing aliases retain their identity across a relocation.
    pub fn capture(&self, seed: &WorkspaceConfig) -> Result<SessionConfig, ProfileError> {
        let _lock = NamedLock::exclusive(self.root.join("config/.folders.lock"))?;
        let mut catalog = self.read()?;
        let before = canonical_line("<catalog>", &catalog)?;
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
        if before != canonical_line("<catalog>", &catalog)? {
            catalog.revision += 1;
            self.publish(&catalog)?;
        }
        Ok(SessionConfig {
            seed: seed.clone(),
            compatibility_instructions: true,
            folder_refs,
        })
    }

    pub fn resolve(&self, scope: &SessionConfig) -> Result<WorkspaceConfig, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.folders.lock"))?;
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
        let _lock = NamedLock::exclusive(self.root.join("config/.folders.lock"))?;
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
        let _lock = NamedLock::shared(self.root.join("config/.folders.lock"))?;
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
        let _lock = NamedLock::exclusive(self.root.join("config/.folders.lock"))?;
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

    fn invalid(&self, reason: &str) -> ProfileError {
        ProfileError::InvalidReference {
            path: self.root.join("config/folders.json"),
            reason: reason.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
