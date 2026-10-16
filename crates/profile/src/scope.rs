//! Session-owned execution resolved from its validated ledger.
use crate::{ConfigRepository, ConfigSnapshot, InstructionResolver, ProfileError};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionScope {
    session_folder: PathBuf,
    session_id: String,
    execution: crate::ExecutionConfig,
}
impl SessionScope {
    pub fn locate(root: &Path, id: &str) -> Result<Self, ProfileError> {
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| root.join(area).join(id))
            .find(|folder| folder.is_dir())
            .ok_or_else(|| ProfileError::InvalidPath {
                path: root.join("threads").join(id),
                reason: "session folder does not exist".into(),
            })?;
        Self::for_session_folder(folder)
    }
    pub fn for_session_folder(folder: impl Into<PathBuf>) -> Result<Self, ProfileError> {
        Self::for_ledger(folder.into().join("main.jsonl"))
    }
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

    pub fn from_genesis_value(
        folder: impl Into<PathBuf>,
        raw: &serde_json::Value,
    ) -> Result<Self, ProfileError> {
        let session_folder = folder.into();
        let invalid = |reason: String| ProfileError::InvalidReference {
            path: session_folder.join("main.jsonl"),
            reason,
        };
        if [
            "workspace",
            "folder_binding",
            "environment",
            "folders",
            "session_scope",
        ]
        .iter()
        .any(|field| raw.get(field).is_some())
        {
            return Err(invalid("retired management scope is not executable".into()));
        }
        let session_id = raw
            .get("thread")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| invalid("session id is missing".into()))?
            .to_owned();
        let execution: crate::ExecutionConfig = serde_json::from_value(
            raw.get("execution")
                .cloned()
                .ok_or_else(|| invalid("execution is required".into()))?,
        )
        .map_err(|e| invalid(e.to_string()))?;
        execution
            .validate()
            .map_err(|reason| invalid(reason.into()))?;
        Ok(Self {
            session_folder,
            session_id,
            execution,
        })
    }
    pub fn session_folder(&self) -> &Path {
        &self.session_folder
    }
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
    pub fn execution(&self) -> &crate::ExecutionConfig {
        &self.execution
    }
    pub fn config(&self, repository: &ConfigRepository) -> Result<ConfigSnapshot, ProfileError> {
        repository.resolve_execution(
            &self.session_id,
            &self.execution,
            Some(&self.session_folder),
        )
    }
    pub fn base_config(
        &self,
        repository: &ConfigRepository,
    ) -> Result<ConfigSnapshot, ProfileError> {
        repository.resolve_execution(&self.session_id, &self.execution, None)
    }
    pub fn instructions(
        &self,
        repository: &ConfigRepository,
        user_agent_dir: &Path,
        config: &ConfigSnapshot,
    ) -> Result<InstructionResolver, ProfileError> {
        Ok(InstructionResolver::new_scoped(
            user_agent_dir,
            repository
                .root()
                .join("session-instructions")
                .join(self.session_folder.file_name().expect("Session folder")),
            config.execution.cwd.iter().map(Path::new),
        ))
    }
}
