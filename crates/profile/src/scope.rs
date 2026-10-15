//! The one place a session's configuration scope is resolved from durable
//! state: the workspace and stable folder binding frozen in its genesis, and
//! the session folder that holds its settings.

use std::path::{Path, PathBuf};

use crate::{ConfigRepository, ConfigSnapshot, InstructionResolver, ProfileError};

/// Where a session's configuration comes from. Built only from the session's
/// own genesis, never from a client-supplied workspace or root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionScope {
    session_folder: PathBuf,
    workspace_id: String,
    folder_binding: Option<String>,
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
    /// Only the genesis line is read, never the whole ledger.
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
        let genesis =
            store::read_genesis(&ledger)?.ok_or_else(|| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: "session ledger has no valid genesis".to_owned(),
            })?;
        let workspace_id = genesis
            .string_field("workspace")
            .ok_or_else(|| ProfileError::InvalidReference {
                path: ledger.clone(),
                reason: "session genesis has no workspace".to_owned(),
            })?
            .to_owned();
        let folder_binding = genesis.string_field("folder_binding").map(str::to_owned);
        Ok(Self {
            session_folder,
            workspace_id,
            folder_binding,
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

    /// The configuration the session's next worker launch captures,
    /// including its session-local settings. A session created before stable
    /// folder bindings resolves only while its workspace has one folder.
    pub fn config(&self, repository: &ConfigRepository) -> Result<ConfigSnapshot, ProfileError> {
        match &self.folder_binding {
            Some(binding) => repository.resolve_for_session_binding(
                &self.workspace_id,
                &self.session_folder,
                binding,
            ),
            None => repository.resolve_for_session(&self.workspace_id, &self.session_folder),
        }
    }

    /// The workspace-level configuration at the session's folder binding,
    /// without its session-local settings: what a new session in the same
    /// place would start from.
    pub fn workspace_config(
        &self,
        repository: &ConfigRepository,
    ) -> Result<ConfigSnapshot, ProfileError> {
        match &self.folder_binding {
            Some(binding) => repository.resolve_for_binding(&self.workspace_id, binding),
            None => repository.resolve(&self.workspace_id),
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
        workspace_instructions(repository, &self.workspace_id, user_agent_dir, config)
    }
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
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorkspaceConfig, WorkspaceFolder};

    fn write_genesis(folder: &Path, binding: Option<&str>) {
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
        );
    }
}
