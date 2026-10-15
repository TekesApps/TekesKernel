//! The one place a session's configuration scope is resolved from durable
//! state: the session-owned seed and folder edits in its ledger, with a
//! legacy workspace adapter for records lacking their original config asset.

use std::path::{Path, PathBuf};

use crate::{ConfigRepository, ConfigSnapshot, InstructionResolver, ProfileError, ScopeSelection};

/// Where a session's configuration comes from. Built only from the session's
/// own ledger, never from a client-supplied workspace or root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionScope {
    session_folder: PathBuf,
    workspace_id: String,
    folder_binding: Option<String>,
    environment: Option<String>,
    folders: Option<Vec<String>>,
    session_config: Option<crate::SessionConfig>,
    legacy_seed: Option<crate::WorkspaceConfig>,
}

/// One of a session's folders: its stable id and its current path as the
/// Folder catalog records it. The id is what the session records; the path
/// follows relocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderRecord {
    pub id: String,
    pub path: String,
}

impl SessionScope {
    /// The scope of the active or archived session `session_id` under
    /// `storage_root`. The caller validates the session id.
    pub fn locate(storage_root: &Path, session_id: &str) -> Result<Self, ProfileError> {
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| storage_root.join(area).join(session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| ProfileError::InvalidPath {
                path: storage_root.join("threads").join(session_id),
                reason: "session folder does not exist".to_owned(),
            })?;
        Self::for_session_folder(folder)
    }

    /// The scope recorded in the genesis of the session at `session_folder`.
    /// Launch reads the validated prefix to apply session-owned folder edits.
    pub fn for_session_folder(session_folder: impl Into<PathBuf>) -> Result<Self, ProfileError> {
        Self::for_ledger(session_folder.into().join("main.jsonl"))
    }

    /// The scope recorded in the genesis of `ledger`; its folder is the
    /// session folder.
    pub fn for_ledger(ledger: impl AsRef<Path>) -> Result<Self, ProfileError> {
        let ledger = ledger.as_ref().to_path_buf();
        let session_folder = ledger
            .parent()
            .ok_or_else(|| ProfileError::InvalidPath {
                path: ledger.clone(),
                reason: "session ledger has no folder".to_owned(),
            })?
            .to_path_buf();
        let bytes = std::fs::read(&ledger)?;
        let scan = store::scan_valid_prefix(&bytes, schema::READER_VERSION);
        let projection = scan
            .projection
            .ok_or_else(|| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: "session ledger has no valid genesis".to_owned(),
            })?;
        if projection.read_only {
            return Err(ProfileError::InvalidReference {
                path: ledger,
                reason: "session requires a newer reader".to_owned(),
            });
        }
        let raw = schema::effective_session_genesis(&projection.events)
            .map_err(|error| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: error.to_string(),
            })?
            .expect("validated genesis");
        Self::from_genesis_value(session_folder, &raw)
    }

    /// Resolves a scope already folded under the caller's ledger lock.
    pub fn from_genesis_value(
        session_folder: impl Into<PathBuf>,
        raw: &serde_json::Value,
    ) -> Result<Self, ProfileError> {
        let session_folder = session_folder.into();
        let ledger = session_folder.join("main.jsonl");
        let workspace_id = raw
            .get("workspace")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: "session genesis has no compatibility address".to_owned(),
            })?
            .to_owned();
        let folder_binding = raw
            .get("folder_binding")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);
        let environment = raw
            .get("environment")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);
        let folders = raw
            .get("folders")
            .and_then(serde_json::Value::as_array)
            .map(|folders| {
                folders
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_owned)
                    .collect()
            });
        let session_config: Option<crate::SessionConfig> = raw
            .get("session_scope")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(|error| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: format!("invalid session scope: {error}"),
            })?;
        // Read an old session's checksum-verified creation asset lazily. Do
        // not rewrite its ledger or enumerate other sessions at startup.
        let legacy_seed = if session_config.is_none() {
            legacy_seed(&session_folder, &workspace_id, raw)?
        } else {
            None
        };
        if let Some(scope) = &session_config
            && scope.seed.id != workspace_id
        {
            return Err(ProfileError::InvalidReference {
                path: ledger,
                reason: "session seed compatibility address differs".to_owned(),
            });
        }
        Ok(Self {
            session_folder,
            workspace_id,
            folder_binding,
            environment,
            folders,
            session_config,
            legacy_seed,
        })
    }

    /// A scope from parts the caller already read from the session genesis.
    #[must_use]
    pub fn from_parts(
        session_folder: impl Into<PathBuf>,
        workspace_id: impl Into<String>,
        folder_binding: Option<String>,
    ) -> Self {
        Self {
            session_folder: session_folder.into(),
            workspace_id: workspace_id.into(),
            folder_binding,
            environment: None,
            folders: None,
            session_config: None,
            legacy_seed: None,
        }
    }

    #[must_use]
    pub fn session_folder(&self) -> &Path {
        &self.session_folder
    }

    #[must_use]
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    #[must_use]
    pub fn folder_binding(&self) -> Option<&str> {
        self.folder_binding.as_deref()
    }

    /// The environment the session runs in: its genesis `environment`, or
    /// [`schema::DEFAULT_ENVIRONMENT`] for a session that predates the field.
    #[must_use]
    pub fn environment(&self) -> &str {
        self.environment
            .as_deref()
            .unwrap_or(schema::DEFAULT_ENVIRONMENT)
    }

    /// The session's folders with their current paths: the ids its genesis
    /// records, or every workspace folder for a session that predates them.
    /// A recorded id the workspace no longer has is an error, never skipped.
    pub fn folders(
        &self,
        repository: &ConfigRepository,
    ) -> Result<Vec<FolderRecord>, ProfileError> {
        let workspace = match &self.session_config {
            Some(scope) => crate::FolderCatalog::new(repository.root()).resolve(scope)?,
            None => match &self.legacy_seed {
                Some(seed) => crate::FolderCatalog::new(repository.root()).resolve_legacy(seed)?,
                None => repository.workspace(&self.workspace_id)?,
            },
        };
        let ids = self
            .folders
            .clone()
            .unwrap_or_else(|| workspace.folder_ids());
        ids.into_iter()
            .map(|id| {
                let path = workspace.path_for_binding(&id).ok_or_else(|| {
                    ProfileError::InvalidReference {
                        path: self.session_folder.join("main.jsonl"),
                        reason: format!("session folder {id} is not in its workspace"),
                    }
                })?;
                Ok(FolderRecord {
                    id,
                    path: path.to_owned(),
                })
            })
            .collect()
    }

    /// The configuration the session's next worker launch captures,
    /// including its session-local settings. A session created before stable
    /// folder bindings resolves only while its workspace has one folder.
    pub fn config(&self, repository: &ConfigRepository) -> Result<ConfigSnapshot, ProfileError> {
        if let Some(scope) = &self.session_config {
            let seed = crate::FolderCatalog::new(repository.root()).resolve(scope)?;
            return repository.resolve_session_seed(
                &seed,
                Some(&self.session_folder),
                self.folder_binding.as_deref(),
                self.selection(),
            );
        }
        if let Some(seed) = &self.legacy_seed {
            let seed = crate::FolderCatalog::new(repository.root()).resolve_legacy(seed)?;
            return repository.resolve_session_seed(
                &seed,
                Some(&self.session_folder),
                self.folder_binding.as_deref(),
                self.selection(),
            );
        }
        repository.resolve_scoped(
            &self.workspace_id,
            Some(&self.session_folder),
            self.folder_binding.as_deref(),
            self.selection(),
        )
    }

    /// The workspace-level configuration in the session's scope (its
    /// environment, binding and folders) without its session-local settings:
    /// what a new session in the same place would start from.
    pub fn workspace_config(
        &self,
        repository: &ConfigRepository,
    ) -> Result<ConfigSnapshot, ProfileError> {
        if let Some(scope) = &self.session_config {
            let seed = crate::FolderCatalog::new(repository.root()).resolve(scope)?;
            return repository.resolve_session_seed(
                &seed,
                None,
                self.folder_binding.as_deref(),
                self.selection(),
            );
        }
        if let Some(seed) = &self.legacy_seed {
            let seed = crate::FolderCatalog::new(repository.root()).resolve_legacy(seed)?;
            return repository.resolve_session_seed(
                &seed,
                None,
                self.folder_binding.as_deref(),
                self.selection(),
            );
        }
        repository.resolve_scoped(
            &self.workspace_id,
            None,
            self.folder_binding.as_deref(),
            self.selection(),
        )
    }

    #[must_use]
    pub fn session_config(&self) -> Option<&crate::SessionConfig> {
        self.session_config.as_ref()
    }

    /// Whether the ledger owns its configuration rather than a live workspace.
    #[must_use]
    pub fn is_detached(&self) -> bool {
        self.session_config.is_some() || self.legacy_seed.is_some()
    }

    /// The environment and recorded folder ids this scope resolves in.
    #[must_use]
    pub fn selection(&self) -> ScopeSelection<'_> {
        ScopeSelection {
            environment: self.environment(),
            folders: self.folders.as_deref(),
        }
    }

    /// The instruction resolver for `config`, resolved from this scope: the
    /// user-level `.agents` directory, the workspace's data directory in the
    /// repository, and the configured folders.
    pub fn instructions(
        &self,
        repository: &ConfigRepository,
        user_agent_dir: &Path,
        config: &ConfigSnapshot,
    ) -> Result<InstructionResolver, ProfileError> {
        if self
            .session_config
            .as_ref()
            .is_some_and(|scope| !scope.compatibility_instructions)
        {
            return Ok(InstructionResolver::new_scoped(
                user_agent_dir,
                repository
                    .root()
                    .join("session-instructions")
                    .join(self.session_folder.file_name().expect("session folder")),
                config.workspace.cwd.iter().map(Path::new),
            )
            .with_folder_authority(repository.root()));
        }
        workspace_instructions(repository, &self.workspace_id, user_agent_dir, config)
    }
}

fn legacy_seed(
    folder: &Path,
    workspace_id: &str,
    raw: &serde_json::Value,
) -> Result<Option<crate::WorkspaceConfig>, ProfileError> {
    let Some(expected) = raw
        .get("config")
        .and_then(|config| config.get("digest"))
        .and_then(serde_json::Value::as_str)
        .filter(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
    else {
        return Ok(None);
    };
    let path = folder.join("assets").join(format!("sha256-{expected}"));
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let actual = crate::digest(&bytes);
    if actual != expected {
        return Err(ProfileError::DigestMismatch {
            expected: expected.to_owned(),
            actual,
        });
    }
    let snapshot: ConfigSnapshot = crate::parse_canonical(&path, &bytes)?;
    let workspace = snapshot.workspace;
    let ids = workspace.folders.unwrap_or_else(|| {
        (1..=workspace.cwd.len())
            .map(|index| format!("folder-{index:04}"))
            .collect()
    });
    if ids.len() != workspace.cwd.len() || workspace.id != workspace_id {
        return Err(ProfileError::InvalidReference {
            path,
            reason: "legacy config asset scope differs from genesis".to_owned(),
        });
    }
    Ok(Some(crate::WorkspaceConfig {
        format: 1,
        revision: workspace.revision,
        id: workspace.id,
        name: workspace.name,
        cwd: Vec::new(),
        folders: ids
            .into_iter()
            .zip(workspace.cwd)
            .map(|(id, path)| crate::WorkspaceFolder { id, path })
            .collect(),
        policy: Some(workspace.policy),
    }))
}

/// The instruction resolver for `workspace_id` with `config`: the one source
/// of the user and workspace instruction directories, used for sessions and
/// for a session that does not exist yet.
pub fn workspace_instructions(
    repository: &ConfigRepository,
    workspace_id: &str,
    user_agent_dir: &Path,
    config: &ConfigSnapshot,
) -> Result<InstructionResolver, ProfileError> {
    Ok(InstructionResolver::new_scoped(
        user_agent_dir,
        repository.workspace_data_dir(workspace_id)?,
        config.workspace.cwd.iter().map(Path::new),
    )
    .with_folder_authority(repository.root()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorkspaceConfig, WorkspaceFolder};

    fn write_genesis(folder: &Path, binding: Option<&str>) {
        write_genesis_with(folder, binding, serde_json::json!({}));
    }

    fn write_genesis_with(folder: &Path, binding: Option<&str>, extra: serde_json::Value) {
        std::fs::create_dir_all(folder).unwrap();
        let mut genesis = serde_json::json!({
            "config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,
            "min_writer":1,"origin_key":"create",
            "origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":"018f0000-0000-7000-8000-000000000001"},
            "resume":"never","seq":1,"thread":"018f0000-0000-7000-8000-000000000001","ts":"2026-10-15T00:00:00.000Z",
            "v":1,"workspace":"workspace-1",
        });
        if let Some(binding) = binding {
            genesis["folder_binding"] = binding.into();
        }
        for (field, value) in extra.as_object().unwrap() {
            genesis[field] = value.clone();
        }
        std::fs::write(
            folder.join("main.jsonl"),
            format!(
                "{}\n",
                serde_json_canonicalizer::to_string(&genesis).unwrap()
            ),
        )
        .unwrap();
    }

    #[test]
    fn scope_reads_workspace_and_binding_from_the_genesis_only() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("threads").join("a");
        let archived = root.path().join("archive").join("b");
        write_genesis(&active, Some("folder-0002"));
        write_genesis(&archived, None);

        let scope = SessionScope::locate(root.path(), "a").unwrap();
        assert_eq!(scope.workspace_id(), "workspace-1");
        assert_eq!(scope.folder_binding(), Some("folder-0002"));
        assert_eq!(scope.session_folder(), active);
        let scope = SessionScope::locate(root.path(), "b").unwrap();
        assert_eq!(scope.folder_binding(), None);
        assert_eq!(scope.session_folder(), archived);
        assert!(SessionScope::locate(root.path(), "missing").is_err());

        std::fs::write(active.join("main.jsonl"), b"").unwrap();
        assert!(SessionScope::for_session_folder(&active).is_err());
    }

    #[test]
    fn session_and_workspace_config_differ_only_in_session_settings() {
        let root = tempfile::tempdir().unwrap();
        let first = root.path().join("first");
        let second = root.path().join("second");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        let repository = ConfigRepository::open(root.path()).unwrap();
        let canonical = |path: &Path| path.canonicalize().unwrap().to_string_lossy().into_owned();
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-1".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: Vec::new(),
                    folders: vec![
                        WorkspaceFolder {
                            id: "folder-0001".to_owned(),
                            path: canonical(&first),
                        },
                        WorkspaceFolder {
                            id: "folder-0002".to_owned(),
                            path: canonical(&second),
                        },
                    ],
                    policy: None,
                },
            )
            .unwrap();
        let folder = root.path().join("threads").join("a");
        write_genesis(&folder, Some("folder-0002"));
        let scope = SessionScope::for_session_folder(&folder).unwrap();
        let session = scope.config(&repository).unwrap();
        let workspace = scope.workspace_config(&repository).unwrap();
        assert_eq!(session.workspace, workspace.workspace);
        assert_eq!(session.workspace.selected_cwd, Some(canonical(&second)));

        let user_agent_dir = root.path().join("custom-user");
        let assets = store::AssetStore::new(folder.join("assets")).unwrap();
        let launch = crate::LaunchProfile::resolve_and_publish_for_scope(
            &repository,
            &scope,
            &user_agent_dir,
            &assets,
        )
        .unwrap();
        assert_eq!(
            launch.config.user_agent_dir.as_deref(),
            user_agent_dir.to_str()
        );
        let decoded = ConfigSnapshot::decode(&launch.config.canonical_bytes().unwrap()).unwrap();
        assert_eq!(decoded.user_agent_dir, launch.config.user_agent_dir);
        assert!(
            decoded
                .protected_roots(repository.root())
                .iter()
                .any(|path| { Path::new(path).ends_with("custom-user") })
        );

        // Without a stable binding a multi-folder session has no defined root.
        write_genesis(&folder, None);
        let legacy = SessionScope::for_session_folder(&folder).unwrap();
        assert!(legacy.config(&repository).is_err());

        let instructions = scope
            .instructions(&repository, &root.path().join("agents"), &session)
            .unwrap();
        assert_eq!(
            instructions,
            InstructionResolver::new_scoped(
                root.path().join("agents"),
                root.path().join("workspaces").join("workspace-1"),
                session.workspace.cwd.iter().map(Path::new),
            )
            .with_folder_authority(repository.root())
        );
    }

    #[test]
    fn environment_and_folders_come_from_the_genesis_or_the_workspace() {
        let root = tempfile::tempdir().unwrap();
        let first = root.path().join("first");
        let second = root.path().join("second");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        let repository = ConfigRepository::open(root.path()).unwrap();
        let path = |dir: &Path| dir.canonicalize().unwrap().to_string_lossy().into_owned();
        repository
            .publish_workspace(
                0,
                &WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-1".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: Vec::new(),
                    folders: vec![
                        WorkspaceFolder {
                            id: "folder-0001".to_owned(),
                            path: path(&first),
                        },
                        WorkspaceFolder {
                            id: "folder-0002".to_owned(),
                            path: path(&second),
                        },
                    ],
                    policy: None,
                },
            )
            .unwrap();
        let record = |id: &str, dir: &Path| FolderRecord {
            id: id.to_owned(),
            path: path(dir),
        };
        let folder = root.path().join("threads").join("a");

        // A session that predates the fields: `local`, every workspace folder.
        write_genesis(&folder, Some("folder-0002"));
        let legacy = SessionScope::for_session_folder(&folder).unwrap();
        assert_eq!(legacy.environment(), "local");
        assert_eq!(
            legacy.folders(&repository).unwrap(),
            vec![
                record("folder-0001", &first),
                record("folder-0002", &second)
            ]
        );

        // Recorded fields win, in recorded order.
        write_genesis_with(
            &folder,
            Some("folder-0002"),
            serde_json::json!({"environment":"sandboxed","folders":["folder-0002"]}),
        );
        let recorded = SessionScope::for_session_folder(&folder).unwrap();
        assert_eq!(recorded.environment(), "sandboxed");
        assert_eq!(
            recorded.folders(&repository).unwrap(),
            vec![record("folder-0002", &second)]
        );

        // A recorded folder the workspace no longer has is an error.
        write_genesis_with(
            &folder,
            None,
            serde_json::json!({"folders":["folder-0009"]}),
        );
        let missing = SessionScope::for_session_folder(&folder).unwrap();
        assert!(missing.folders(&repository).is_err());
    }
    #[test]
    fn phase7_legacy_asset_is_lazy_frozen_and_follows_logical_relocation_without_workspace() {
        let root = tempfile::tempdir().unwrap();
        let project = root.path().join("project");
        std::fs::create_dir(&project).unwrap();
        let path = project
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let repository = ConfigRepository::open(root.path()).unwrap();
        let seed = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "workspace-1".into(),
            name: "Legacy".into(),
            cwd: Vec::new(),
            folders: vec![WorkspaceFolder {
                id: "folder-0001".into(),
                path: path.clone(),
            }],
            policy: Some(crate::WorkspacePolicy {
                writable_roots: vec![path.clone()],
                network: true,
                ..Default::default()
            }),
        };
        repository.publish_workspace(0, &seed).unwrap();
        let snapshot = repository
            .resolve_scoped(
                "workspace-1",
                None,
                Some("folder-0001"),
                ScopeSelection {
                    environment: "local",
                    folders: Some(&["folder-0001".into()]),
                },
            )
            .unwrap();
        let bytes = snapshot.canonical_bytes().unwrap();
        let digest = crate::digest(&bytes);
        let session = root.path().join("threads/legacy");
        write_genesis_with(
            &session,
            Some("folder-0001"),
            serde_json::json!({"config":{"digest":digest},"environment":"local","folders":["folder-0001"]}),
        );
        std::fs::create_dir(session.join("assets")).unwrap();
        let asset = session.join("assets").join(format!("sha256-{digest}"));
        std::fs::write(&asset, &bytes).unwrap();
        let ledger_before = std::fs::read(session.join("main.jsonl")).unwrap();
        let catalog = crate::FolderCatalog::new(root.path());
        let native = catalog.capture(&seed).unwrap();
        std::fs::remove_dir_all(root.path().join("workspaces/workspace-1")).unwrap();
        let legacy = SessionScope::for_session_folder(&session).unwrap();
        assert!(legacy.is_detached());
        assert_eq!(
            legacy.config(&repository).unwrap().workspace.policy,
            snapshot.workspace.policy
        );
        let moved = root.path().join("moved");
        std::fs::rename(&project, &moved).unwrap();
        let moved = moved.canonicalize().unwrap().to_string_lossy().into_owned();
        catalog
            .relocate_alias("workspace-1", "folder-0001", &path, &moved)
            .unwrap();
        let restarted = SessionScope::for_session_folder(&session).unwrap();
        assert_eq!(
            restarted.config(&repository).unwrap().execution_cwd(),
            Some(moved.as_str())
        );
        assert_eq!(catalog.resolve(&native).unwrap().folders[0].path, moved);
        assert_eq!(
            std::fs::read(session.join("main.jsonl")).unwrap(),
            ledger_before
        );
        std::fs::write(asset, b"tampered\n").unwrap();
        assert!(matches!(
            SessionScope::for_session_folder(&session),
            Err(ProfileError::DigestMismatch { .. })
        ));
    }

    #[test]
    fn phase7_environment_is_live_while_workspace_policy_is_a_creation_seed() {
        let root = tempfile::tempdir().unwrap();
        let project = root.path().join("project");
        std::fs::create_dir(&project).unwrap();
        let path = project
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let repository = ConfigRepository::open(root.path()).unwrap();
        let seed = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "workspace-1".into(),
            name: "Seed".into(),
            cwd: vec![path.clone()],
            folders: Vec::new(),
            policy: Some(crate::WorkspacePolicy {
                writable_roots: vec![path],
                network: true,
                ..Default::default()
            }),
        };
        let scope = crate::FolderCatalog::new(root.path())
            .capture(&seed)
            .unwrap();
        let session = root.path().join("threads/native");
        write_genesis_with(
            &session,
            Some("folder-0001"),
            serde_json::json!({"min_reader":4,"session_scope":scope,"environment":"local","folders":["folder-0001"]}),
        );
        for (revision, network) in [(1, false), (2, true)] {
            let mut environment = crate::Environment::local();
            environment.network = network;
            repository
                .publish_environments(
                    revision - 1,
                    &crate::EnvironmentsConfig {
                        format: 1,
                        revision,
                        environments: vec![environment],
                    },
                )
                .unwrap();
            let config = SessionScope::for_session_folder(&session)
                .unwrap()
                .config(&repository)
                .unwrap();
            assert_eq!(config.workspace.policy.network, network);
            assert_eq!(config.revisions.environments, Some(revision));
        }
    }
}
